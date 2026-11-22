#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1406574d0, "unknown_1406574d0", fn_1406574d0 as *const ()), // sub_1406574d0
    Replacement::draft(0x1406574f0, "fmt_1406574f0", fn_1406574f0 as *const ()), // fmt::v9::detail::iterator_buffer<fmt::v9::appender,char,fmt::v9::detail::buffer_traits>::fmt::v9::detail::iterator_buffe
    Replacement::draft(0x140657500, "unknown_140657500", fn_140657500 as *const ()), // sub_140657500
    Replacement::draft_throws(0x1406575a0, "unknown_1406575a0", fn_1406575a0 as *const (), Throws::Caught), // sub_1406575a0
    Replacement::draft(0x1406576f0, "bonsai_1406576f0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143222b78, 0x15, 0x17, 0x8> as *const ()), // sub_1406576f0
    Replacement::draft(0x140657790, "bonsai_140657790", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143222b78, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140657790
    Replacement::draft_throws(0x1406579e0, "bonsai_1406579e0", fn_1406579e0 as *const (), Throws::Caught), // sub_1406579e0
    Replacement::draft_throws(0x140657b70, "bonsai_140657b70", tpl_1905::<0x14387c410, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140657b70
    Replacement::draft(0x140657c30, "bonsai_140657c30", tpl_5987::<0x1431f8560, 0x60, 0x1430f6b98, 0x38, 0x1432316d0, 0x141834d30> as *const ()), // bonsai::LevitateProgram::bonsai::LevitateProgram::vfunc_0
    Replacement::draft(0x140657c80, "bonsai_140657c80", tpl_6280::<0x141b726a0, 0x141834d30> as *const ()), // bonsai::KnowledgeListener::bonsai::KnowledgeListener::vfunc_0
    Replacement::draft(0x140657cc0, "bonsai_140657cc0", fn_140657cc0 as *const ()), // sub_140657cc0
    Replacement::draft_throws(0x140657ee0, "bonsai_140657ee0", fn_140657ee0 as *const (), Throws::Caught), // sub_140657ee0
    Replacement::draft_throws(0x140658070, "bonsai_140658070", tpl_1905::<0x14387c420, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140658070
    Replacement::draft_throws(0x140658120, "bonsai_140658120", tpl_4804::<0x143cc0eb8, 0x143cc0ec0, 0x140658560, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432230f6, 0x142646960, 0x143cc0ee0, 0x143cc0ee8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::knowledge::updateKnowledgeTimeStamp>
    Replacement::draft_throws(0x140658330, "bonsai_140658330", tpl_6282::<0x143c8f4d0, 0x140658710, 0x14020e8f0> as *const (), Throws::Discard), // sub_140658330
    Replacement::draft_throws(0x140658560, "bonsai_140658560", tpl_3697::<0x1411e9f50, 0x140658950, 0x143cc0f0c, 0x1420af120, 0x140215750, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x1406588e0, 0x1401de410> as *const (), Throws::Discard), // sub_140658560
    Replacement::draft_throws(0x140658710, "bonsai_140658710", fn_140658710 as *const (), Throws::Caught), // sub_140658710
    Replacement::draft(0x140658890, "bonsai_140658890", fn_140658890 as *const ()), // sub_140658890
    Replacement::draft(0x1406588e0, "bonsai_1406588e0", tpl_3028::<0x143cc0f0c, 0x20, 0x8, 0x143c8d910, 0x580e0, 0x141b734b0> as *const ()), // sub_1406588e0
    Replacement::draft(0x140658b00, "bonsai_140658b00", tpl_5163::<0x100, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140658b90> as *const ()), // sub_140658b00
    Replacement::draft(0x140658b90, "bonsai_140658b90", tpl_5164::<0x143223298, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x143222d60, 0x38, 0xb8, 0x40, 0x48> as *const ()), // sub_140658b90
    Replacement::draft(0x140658c40, "bonsai_140658c40", tpl_64337::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140658b90> as *const ()), // sub_140658c40
    Replacement::draft_throws(0x140658e90, "bonsai_140658e90", fn_140658e90 as *const (), Throws::Caught), // sub_140658e90
    Replacement::draft_throws(0x140659020, "bonsai_140659020", tpl_1905::<0x14387c430, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140659020
    Replacement::draft(0x1406590e0, "bonsai_1406590e0", fn_1406590e0 as *const ()), // bonsai::IdleBodyProgram::bonsai::IdleBodyProgram::vfunc_0
    Replacement::draft(0x140659130, "bonsai_140659130", tpl_64921::<0x10, 0x18, 0x143c8d87c, 0x1806c, 0x18058, 0x142489ee0, 0x18068, 0x143c95c74, 0x18050, 0x20, 0x38> as *const ()), // sub_140659130
    Replacement::draft_throws(0x140659210, "bonsai_140659210", fn_140659210 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::hidden_npc::system>
    Replacement::draft_throws(0x140659570, "bonsai_140659570", fn_140659570 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::unhidden_npc::system>
    Replacement::draft(0x1406598d0, "unknown_1406598d0", tpl_1095::<0x143c8f4d4, 0x14020e8f0> as *const ()), // sub_1406598d0
    Replacement::draft_throws(0x140659ab0, "unknown_140659ab0", tpl_6288::<0x140659c40, 0x1420b09c0, 0x143cc10a8, 0x1420c66d0, 0x140399380, 0x1420b0520, 0x1401de830, 0x14241c190, 0x141b75070, 0x14065a1a0> as *const (), Throws::Discard), // sub_140659ab0
    Replacement::draft_throws(0x140659c40, "unknown_140659c40", tpl_6289::<0x143cc10ac, 0x1420b09c0, 0x143cc10a8, 0x14039c9d0, 0x1420c66d0, 0x140399380, 0x1420b0520, 0x1401de830, 0x14241c190> as *const (), Throws::Discard), // sub_140659c40
    Replacement::draft_throws(0x140659e20, "unknown_140659e20", tpl_6288::<0x140659fb0, 0x1420b09c0, 0x143cc10a0, 0x1420c66d0, 0x140399380, 0x1420b0520, 0x1401de830, 0x14241c190, 0x141b74e00, 0x14065a1a0> as *const (), Throws::Discard), // sub_140659e20
    Replacement::draft_throws(0x140659fb0, "unknown_140659fb0", tpl_6289::<0x143cc10a4, 0x1420b09c0, 0x143cc10a0, 0x14039c9d0, 0x1420c66d0, 0x140399380, 0x1420b0520, 0x1401de830, 0x14241c190> as *const (), Throws::Discard), // sub_140659fb0
    Replacement::draft_throws(0x14065a1a0, "unknown_14065a1a0", fn_14065a1a0 as *const (), Throws::Discard), // sub_14065a1a0
    Replacement::draft(0x14065ab50, "unknown_14065ab50", fn_14065ab50 as *const ()), // sub_14065ab50
    Replacement::draft_throws(0x14065abc0, "hazardsystem_14065abc0", fn_14065abc0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<hazardsystem::update>
    Replacement::draft(0x14065adc0, "unknown_14065adc0", fn_14065adc0 as *const ()), // sub_14065adc0
    Replacement::draft(0x14065ae90, "unknown_14065ae90", tpl_1094::<0x143cc11d4, 0x14065b3f0, 0x14020e8f0> as *const ()), // sub_14065ae90
    Replacement::draft_throws(0x14065b090, "unknown_14065b090", fn_14065b090 as *const (), Throws::Discard), // sub_14065b090
    Replacement::draft(0x14065b330, "unknown_14065b330", fn_14065b330 as *const ()), // sub_14065b330
    Replacement::draft(0x14065b3f0, "unknown_14065b3f0", tpl_4470::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143224e78> as *const ()), // sub_14065b3f0
    Replacement::draft(0x14065b580, "unknown_14065b580", tpl_3752::<0x143cc12b4, 0x20, 0x8, 0x141b761d0> as *const ()), // sub_14065b580
    Replacement::draft(0x14065b910, "unknown_14065b910", fn_14065b910 as *const ()), // sub_14065b910
    Replacement::draft(0x14065bbd0, "unknown_14065bbd0", fn_14065bbd0 as *const ()), // sub_14065bbd0
    Replacement::draft(0x14065bf20, "bonsai_14065bf20", fn_14065bf20 as *const ()), // sub_14065bf20
    Replacement::draft(0x14065bfc0, "bonsai_14065bfc0", fn_14065bfc0 as *const ()), // sub_14065bfc0
    Replacement::draft_throws(0x14065c220, "bonsai_14065c220", fn_14065c220 as *const (), Throws::Caught), // sub_14065c220
    Replacement::draft_throws(0x14065c3b0, "bonsai_14065c3b0", tpl_1905::<0x14387c460, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14065c3b0
    Replacement::draft(0x14065c460, "bonsai_14065c460", fn_14065c460 as *const ()), // bonsai::HazardServiceInstance::bonsai::HazardServiceInstance::vfunc_0
    Replacement::draft(0x14065c4d0, "bonsai_14065c4d0", tpl_6300::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x28, 0x30, 0x143225478> as *const ()), // sub_14065c4d0
    Replacement::draft(0x14065c590, "bonsai_14065c590", tpl_6301::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143225478, 0xffffffffffffffd0, 0xfffffffffffffff0, 0x18, 0x68> as *const ()), // sub_14065c590
    Replacement::draft_throws(0x14065c6b0, "bonsai_14065c6b0", fn_14065c6b0 as *const (), Throws::Caught), // sub_14065c6b0
    Replacement::draft_throws(0x14065c740, "bonsai_14065c740", tpl_5859::<0x14387c470, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c1e0, 0x14387c9e8> as *const (), Throws::Discard), // sub_14065c740
    Replacement::draft_throws(0x14065c820, "bonsai_14065c820", fn_14065c820 as *const (), Throws::Caught), // sub_14065c820
    Replacement::draft(0x14065cc70, "bonsai_14065cc70", tpl_6300::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x28, 0x30, 0x1432255b8> as *const ()), // sub_14065cc70
    Replacement::draft(0x14065cd30, "bonsai_14065cd30", tpl_6301::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432255b8, 0xffffffffffffffd0, 0xfffffffffffffff0, 0x18, 0x68> as *const ()), // sub_14065cd30
    Replacement::draft_throws(0x14065ce50, "bonsai_14065ce50", fn_14065ce50 as *const (), Throws::Caught), // sub_14065ce50
    Replacement::draft_throws(0x14065cee0, "bonsai_14065cee0", tpl_5859::<0x14387c480, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c1e0, 0x14387c9e8> as *const (), Throws::Discard), // sub_14065cee0
    Replacement::draft(0x14065d020, "bonsai_14065d020", fn_14065d020 as *const ()), // bonsai::FloatDifferenceConditionDecoratorInstance::bonsai::FloatDifferenceConditionDecoratorInstance::vfunc_0
    Replacement::draft(0x14065d070, "bonsai_14065d070", fn_14065d070 as *const ()), // sub_14065d070
    Replacement::draft(0x14065d1d0, "bonsai_14065d1d0", fn_14065d1d0 as *const ()), // bonsai::KnowledgeConditionDecoratorInstance::bonsai::KnowledgeConditionDecoratorInstance::vfunc_0
    Replacement::draft(0x14065d220, "bonsai_14065d220", fn_14065d220 as *const ()), // sub_14065d220
    Replacement::draft(0x14065d2b0, "bonsai_14065d2b0", fn_14065d2b0 as *const ()), // sub_14065d2b0
    Replacement::draft_throws(0x14065d3b0, "bonsai_14065d3b0", fn_14065d3b0 as *const (), Throws::Caught), // sub_14065d3b0
    Replacement::draft_throws(0x14065d440, "bonsai_14065d440", tpl_5859::<0x14387c490, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_14065d440
    Replacement::draft(0x14065d520, "bonsai_14065d520", fn_14065d520 as *const ()), // sub_14065d520
    Replacement::draft_throws(0x14065d590, "bonsai_14065d590", fn_14065d590 as *const (), Throws::Caught), // sub_14065d590
    Replacement::draft(0x14065da10, "bonsai_14065da10", fn_14065da10 as *const ()), // sub_14065da10
    Replacement::draft(0x14065da70, "bonsai_14065da70", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x143225fe8, 0x28, 0x30> as *const ()), // sub_14065da70
    Replacement::draft(0x14065db30, "bonsai_14065db30", tpl_6309::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143225fe8, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14065db30
    Replacement::draft_throws(0x14065dc70, "bonsai_14065dc70", fn_14065dc70 as *const (), Throws::Caught), // sub_14065dc70
    Replacement::draft_throws(0x14065dd00, "bonsai_14065dd00", tpl_5859::<0x14387c4a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c4c0, 0x14387c7a8> as *const (), Throws::Discard), // sub_14065dd00
    Replacement::draft_throws(0x14065de10, "bonsai_14065de10", fn_14065de10 as *const (), Throws::Caught), // sub_14065de10
    Replacement::draft_throws(0x14065e3d0, "bonsai_14065e3d0", fn_14065e3d0 as *const (), Throws::Caught), // sub_14065e3d0
    Replacement::draft_throws(0x14065e560, "bonsai_14065e560", tpl_1905::<0x14387c4c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c7a8> as *const (), Throws::Discard), // sub_14065e560
    Replacement::draft(0x14065e610, "bonsai_14065e610", tpl_6310::<0x8, 0x10> as *const ()), // sub_14065e610
    Replacement::draft(0x14065e660, "bonsai_14065e660", fn_14065e660 as *const ()), // sub_14065e660
    Replacement::draft(0x14065e780, "bonsai_14065e780", tpl_865::<0x8, 0x1401de830> as *const ()), // sub_14065e780
    Replacement::draft(0x14065e7f0, "bonsai_14065e7f0", tpl_6120::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x28, 0x30, 0x38, 0x143226308> as *const ()), // sub_14065e7f0
    Replacement::draft(0x14065e8b0, "bonsai_14065e8b0", fn_14065e8b0 as *const ()), // sub_14065e8b0
    Replacement::draft_throws(0x14065e9f0, "bonsai_14065e9f0", fn_14065e9f0 as *const (), Throws::Caught), // sub_14065e9f0
    Replacement::draft_throws(0x14065ea80, "bonsai_14065ea80", tpl_5859::<0x14387c4b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c4c0, 0x14387c7a8> as *const (), Throws::Discard), // sub_14065ea80
    Replacement::draft(0x14065eb60, "bonsai_14065eb60", fn_14065eb60 as *const ()), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_0
    Replacement::draft_throws(0x14065eba0, "bonsai_14065eba0", fn_14065eba0 as *const (), Throws::Discard), // sub_14065eba0
    Replacement::draft_throws(0x14065ec00, "bonsai_14065ec00", fn_14065ec00 as *const (), Throws::Discard), // sub_14065ec00
    Replacement::draft(0x14065efc0, "bonsai_14065efc0", fn_14065efc0 as *const ()), // sub_14065efc0
    Replacement::draft(0x14065f150, "bonsai_14065f150", fn_14065f150 as *const ()), // sub_14065f150
    Replacement::draft(0x14065f220, "bonsai_14065f220", fn_14065f220 as *const ()), // sub_14065f220
    Replacement::draft(0x14065f4a0, "bonsai_14065f4a0", fn_14065f4a0 as *const ()), // sub_14065f4a0
    Replacement::draft(0x14065f5b0, "bonsai_14065f5b0", fn_14065f5b0 as *const ()), // sub_14065f5b0
    Replacement::draft(0x14065f6d0, "bonsai_14065f6d0", tpl_6319::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x42, 0x143225eb8, 0x46, 0x8, 0x20> as *const ()), // sub_14065f6d0
    Replacement::draft(0x14065f780, "bonsai_14065f780", tpl_6320::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143225eb8, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14065f780
    Replacement::draft(0x14065f870, "bonsai_14065f870", tpl_6319::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x42, 0x143226650, 0x46, 0x8, 0x20> as *const ()), // sub_14065f870
    Replacement::draft(0x14065f920, "bonsai_14065f920", tpl_6320::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143226650, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14065f920
    Replacement::draft_throws(0x14065fa50, "bonsai_14065fa50", fn_14065fa50 as *const (), Throws::Caught), // sub_14065fa50
    Replacement::draft_throws(0x14065fae0, "bonsai_14065fae0", tpl_5859::<0x14387c4d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c4c0, 0x14387c7a8> as *const (), Throws::Discard), // sub_14065fae0
    Replacement::draft(0x14065fbf0, "bonsai_14065fbf0", fn_14065fbf0 as *const ()), // sub_14065fbf0
    Replacement::draft(0x14065fcb0, "bonsai_14065fcb0", fn_14065fcb0 as *const ()), // sub_14065fcb0
    Replacement::draft(0x14065fd70, "unknown_14065fd70", fn_14065fd70 as *const ()), // sub_14065fd70
    Replacement::draft_throws(0x14065fde0, "unknown_14065fde0", fn_14065fde0 as *const (), Throws::Caught), // sub_14065fde0
    Replacement::draft_throws(0x14065fe70, "unknown_14065fe70", tpl_5859::<0x14387c4e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c4c0, 0x14387c7a8> as *const (), Throws::Discard), // sub_14065fe70
    Replacement::draft(0x14065ff50, "unknown_14065ff50", fn_14065ff50 as *const ()), // sub_14065ff50
    Replacement::draft_throws(0x14065ff90, "unknown_14065ff90", tpl_838::<0x14065ffa0> as *const (), Throws::Discard), // sub_14065ff90
    Replacement::draft_throws(0x14065ffa0, "unknown_14065ffa0", fn_14065ffa0 as *const (), Throws::Discard), // sub_14065ffa0
    Replacement::draft_throws(0x140660010, "unknown_140660010", fn_140660010 as *const (), Throws::Discard), // sub_140660010
    Replacement::draft_throws(0x140660040, "unknown_140660040", fn_140660040 as *const (), Throws::Discard), // sub_140660040
    Replacement::draft_throws(0x140660090, "unknown_140660090", fn_140660090 as *const (), Throws::Discard), // sub_140660090
    Replacement::draft(0x140660160, "unknown_140660160", fn_140660160 as *const ()), // sub_140660160
    Replacement::draft(0x140660350, "unknown_140660350", fn_140660350 as *const ()), // sub_140660350
    Replacement::draft_throws(0x140660540, "std_140660540", fn_140660540 as *const (), Throws::Discard), // sub_140660540
    Replacement::draft_throws(0x140660880, "std_140660880", tpl_3740::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_140660880
    Replacement::draft_throws(0x1406608a0, "std_1406608a0", tpl_65423::<0x14307e970, 0x142645870, 0x140660090, 0x1406608d4> as *const (), Throws::Discard), // sub_1406608a0
    Replacement::draft(0x140660a90, "unknown_140660a90", fn_140660a90 as *const ()), // sub_140660a90
    Replacement::draft(0x140660c50, "bonsai_140660c50", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x143226c90, 0x1f, 0x8, 0x1c> as *const ()), // sub_140660c50
    Replacement::draft(0x140660ce0, "bonsai_140660ce0", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143226c90, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_140660ce0
    Replacement::draft_throws(0x140660f30, "bonsai_140660f30", fn_140660f30 as *const (), Throws::Caught), // sub_140660f30
    Replacement::draft_throws(0x1406610c0, "bonsai_1406610c0", tpl_1905::<0x14387c4f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_1406610c0
    Replacement::draft(0x140661170, "bonsai_140661170", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14317fc70, 0x8> as *const ()), // sub_140661170
    Replacement::draft(0x1406611f0, "bonsai_1406611f0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14317fc70> as *const ()), // sub_1406611f0
    Replacement::draft(0x1406612c0, "bonsai_1406612c0", fn_1406612c0 as *const ()), // bonsai::EQSNodeInstance::bonsai::EQSNodeInstance::vfunc_0
    Replacement::draft(0x140661320, "bonsai_140661320", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432272f0, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_140661320
    Replacement::draft(0x1406613c0, "bonsai_1406613c0", tpl_6334::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x28, 0x1432272f0, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1406613c0
    Replacement::draft_throws(0x1406614e0, "bonsai_1406614e0", fn_1406614e0 as *const (), Throws::Caught), // sub_1406614e0
    Replacement::draft_throws(0x140661570, "bonsai_140661570", tpl_5859::<0x14387c510, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c5d8, 0x14387c578> as *const (), Throws::Discard), // sub_140661570
    Replacement::draft_throws(0x140661650, "bonsai_140661650", fn_140661650 as *const (), Throws::Caught), // sub_140661650
    Replacement::draft_throws(0x140661c10, "bonsai_140661c10", fn_140661c10 as *const (), Throws::Caught), // sub_140661c10
    Replacement::draft_throws(0x140661da0, "bonsai_140661da0", tpl_1905::<0x14387c5d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c578> as *const (), Throws::Discard), // sub_140661da0
    Replacement::draft_throws(0x140661e50, "bonsai_140661e50", fn_140661e50 as *const (), Throws::Caught), // sub_140661e50
    Replacement::draft_throws(0x140662260, "bonsai_140662260", fn_140662260 as *const (), Throws::Caught), // sub_140662260
    Replacement::draft_throws(0x1406622f0, "bonsai_1406622f0", tpl_1908::<0x14387c578, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1406622f0
    Replacement::draft(0x1406623b0, "bonsai_1406623b0", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143227588, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_1406623b0
    Replacement::draft(0x140662450, "bonsai_140662450", tpl_6334::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x28, 0x143227588, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140662450
    Replacement::draft_throws(0x1406626a0, "bonsai_1406626a0", fn_1406626a0 as *const (), Throws::Caught), // sub_1406626a0
    Replacement::draft_throws(0x140662830, "bonsai_140662830", tpl_1905::<0x14387c520, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c578> as *const (), Throws::Discard), // sub_140662830
    Replacement::draft(0x140662910, "bonsai_140662910", fn_140662910 as *const ()), // sub_140662910
    Replacement::draft(0x1406629b0, "bonsai_1406629b0", fn_1406629b0 as *const ()), // sub_1406629b0
    Replacement::draft_throws(0x140662c00, "bonsai_140662c00", fn_140662c00 as *const (), Throws::Caught), // sub_140662c00
    Replacement::draft_throws(0x140662d90, "bonsai_140662d90", tpl_1905::<0x14387c530, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c578> as *const (), Throws::Discard), // sub_140662d90
    Replacement::draft(0x140662e70, "bonsai_140662e70", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143227b08, 0x8> as *const ()), // sub_140662e70
    Replacement::draft(0x140662ef0, "bonsai_140662ef0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143227b08> as *const ()), // sub_140662ef0
    Replacement::draft_throws(0x140663120, "bonsai_140663120", fn_140663120 as *const (), Throws::Caught), // sub_140663120
    Replacement::draft_throws(0x1406632b0, "bonsai_1406632b0", tpl_1905::<0x14387c540, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1406632b0
    Replacement::draft(0x140663390, "bonsai_140663390", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143227dd8, 0x8> as *const ()), // sub_140663390
    Replacement::draft(0x140663410, "bonsai_140663410", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143227dd8> as *const ()), // sub_140663410
    Replacement::draft_throws(0x140663640, "bonsai_140663640", fn_140663640 as *const (), Throws::Caught), // sub_140663640
    Replacement::draft_throws(0x1406637d0, "bonsai_1406637d0", tpl_1905::<0x14387c550, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1406637d0
    Replacement::draft_throws(0x140663880, "bonsai_140663880", fn_140663880 as *const (), Throws::Discard), // sub_140663880
    Replacement::draft(0x140663930, "bonsai_140663930", fn_140663930 as *const ()), // sub_140663930
    Replacement::draft(0x140663980, "bonsai_140663980", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x143228098, 0x1f, 0x8, 0x1c> as *const ()), // sub_140663980
    Replacement::draft(0x140663a10, "bonsai_140663a10", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143228098, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_140663a10
    Replacement::draft_throws(0x140663c60, "bonsai_140663c60", fn_140663c60 as *const (), Throws::Caught), // sub_140663c60
    Replacement::draft_throws(0x140663df0, "bonsai_140663df0", tpl_1905::<0x14387c560, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_140663df0
    Replacement::draft_throws(0x140663ea0, "bonsai_140663ea0", fn_140663ea0 as *const (), Throws::Discard), // sub_140663ea0
    Replacement::draft(0x140663fd0, "bonsai_140663fd0", fn_140663fd0 as *const ()), // sub_140663fd0
    Replacement::draft(0x140663ff0, "bonsai_140663ff0", fn_140663ff0 as *const ()), // sub_140663ff0
    Replacement::draft(0x140664010, "bonsai_140664010", tpl_6342::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143427178, 0x1432281b8, 0x20, 0x28, 0x30, 0x143159a38, 0x143159a40> as *const ()), // sub_140664010
    Replacement::draft(0x1406640e0, "bonsai_1406640e0", fn_1406640e0 as *const ()), // bonsai::EQSQueryInstance::bonsai::EQSQueryInstance::vfunc_0
    Replacement::draft(0x140664130, "bonsai_140664130", tpl_6342::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143427178, 0x1432281b8, 0x20, 0x28, 0x30, 0x143159a38, 0x143159a40> as *const ()), // sub_140664130
    Replacement::draft(0x140664200, "bonsai_140664200", fn_140664200 as *const ()), // sub_140664200
    Replacement::draft_throws(0x1406642e0, "bonsai_1406642e0", fn_1406642e0 as *const (), Throws::Caught), // sub_1406642e0
    Replacement::draft_throws(0x140664370, "bonsai_140664370", tpl_1908::<0x14387c568, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140664370
    Replacement::draft(0x1406643f0, "bonsai_1406643f0", fn_1406643f0 as *const ()), // sub_1406643f0
    Replacement::draft(0x140664460, "bonsai_140664460", fn_140664460 as *const ()), // sub_140664460
    Replacement::draft_throws(0x1406645b0, "bonsai_1406645b0", fn_1406645b0 as *const (), Throws::Discard), // sub_1406645b0
    Replacement::draft_throws(0x1406648a0, "bonsai_1406648a0", fn_1406648a0 as *const (), Throws::Discard), // sub_1406648a0
    Replacement::draft(0x1406649f0, "bonsai_1406649f0", fn_1406649f0 as *const ()), // sub_1406649f0
    Replacement::draft(0x140664ce0, "bonsai_140664ce0", fn_140664ce0 as *const ()), // sub_140664ce0
    Replacement::draft_throws(0x140664ed0, "bonsai_140664ed0", fn_140664ed0 as *const (), Throws::Discard), // sub_140664ed0
    Replacement::draft_throws(0x1406650d0, "bonsai_1406650d0", fn_1406650d0 as *const (), Throws::Discard), // sub_1406650d0
    Replacement::draft(0x1406658e0, "bonsai_1406658e0", fn_1406658e0 as *const ()), // sub_1406658e0
    Replacement::draft(0x140665af0, "bonsai_140665af0", fn_140665af0 as *const ()), // sub_140665af0
    Replacement::draft(0x1406662b0, "bonsai_1406662b0", fn_1406662b0 as *const ()), // sub_1406662b0
    Replacement::draft(0x140666360, "bonsai_140666360", fn_140666360 as *const ()), // sub_140666360
    Replacement::draft(0x1406663e0, "bonsai_1406663e0", fn_1406663e0 as *const ()), // sub_1406663e0
    Replacement::draft_throws(0x140666610, "bonsai_140666610", fn_140666610 as *const (), Throws::Discard), // sub_140666610
    Replacement::draft(0x140666840, "bonsai_140666840", fn_140666840 as *const ()), // sub_140666840
    Replacement::draft_throws(0x140666a40, "bonsai_140666a40", fn_140666a40 as *const (), Throws::Discard), // sub_140666a40
    Replacement::draft(0x140666af0, "bonsai_140666af0", fn_140666af0 as *const ()), // sub_140666af0
    Replacement::draft(0x140666b90, "unknown_140666b90", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432283f0, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_140666b90
    Replacement::draft(0x140666c30, "unknown_140666c30", fn_140666c30 as *const ()), // sub_140666c30
    Replacement::draft(0x140666ce0, "std_140666ce0", tpl_1::<0x14388e670> as *const ()), // std::_Func_impl_no_alloc<bool (*)(const bonsai::EQSQueryPointData&,const bonsai::EQSQueryPointData&),bool,const bonsai::
    Replacement::draft(0x140666cf0, "std_140666cf0", fn_140666cf0 as *const ()), // std::_Func_impl_no_alloc<bool (*)(const bonsai::EQSQueryPointData&,const bonsai::EQSQueryPointData&),bool,const bonsai::
    Replacement::draft(0x140666d00, "std_140666d00", tpl_550::<0x143228388, 0x8> as *const ()), // std::_Func_impl_no_alloc<bool (*)(const bonsai::EQSQueryPointData&,const bonsai::EQSQueryPointData&),bool,const bonsai::
    Replacement::draft(0x140666d20, "unknown_140666d20", fn_140666d20 as *const ()), // sub_140666d20
    Replacement::draft(0x140666f10, "bonsai_140666f10", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143228878, 0x8> as *const ()), // sub_140666f10
    Replacement::draft(0x140666f90, "bonsai_140666f90", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143228878> as *const ()), // sub_140666f90
    Replacement::draft_throws(0x1406671c0, "bonsai_1406671c0", fn_1406671c0 as *const (), Throws::Caught), // sub_1406671c0
    Replacement::draft_throws(0x140667350, "bonsai_140667350", tpl_1905::<0x14387c588, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_140667350
    Replacement::draft(0x140667430, "bonsai_140667430", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143228b58, 0x8> as *const ()), // sub_140667430
    Replacement::draft(0x1406674b0, "bonsai_1406674b0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143228b58> as *const ()), // sub_1406674b0
    Replacement::draft_throws(0x1406676e0, "bonsai_1406676e0", fn_1406676e0 as *const (), Throws::Caught), // sub_1406676e0
    Replacement::draft_throws(0x140667870, "bonsai_140667870", tpl_1905::<0x14387c598, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_140667870
    Replacement::draft(0x140667950, "bonsai_140667950", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143228e20, 0x8> as *const ()), // sub_140667950
    Replacement::draft(0x1406679d0, "bonsai_1406679d0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143228e20> as *const ()), // sub_1406679d0
    Replacement::draft_throws(0x140667c00, "bonsai_140667c00", fn_140667c00 as *const (), Throws::Caught), // sub_140667c00
    Replacement::draft_throws(0x140667d90, "bonsai_140667d90", tpl_1905::<0x14387c5a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_140667d90
    Replacement::draft(0x140667e70, "bonsai_140667e70", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143228f20, 0x8> as *const ()), // sub_140667e70
    Replacement::draft(0x140667ef0, "bonsai_140667ef0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143228f20> as *const ()), // sub_140667ef0
    Replacement::draft_throws(0x140668120, "bonsai_140668120", fn_140668120 as *const (), Throws::Caught), // sub_140668120
    Replacement::draft_throws(0x1406682b0, "bonsai_1406682b0", tpl_1905::<0x14387c5b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1406682b0
    Replacement::draft(0x140668390, "bonsai_140668390", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432291e8, 0x8> as *const ()), // sub_140668390
    Replacement::draft(0x140668410, "bonsai_140668410", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432291e8> as *const ()), // sub_140668410
    Replacement::draft_throws(0x140668640, "bonsai_140668640", fn_140668640 as *const (), Throws::Caught), // sub_140668640
    Replacement::draft_throws(0x1406687d0, "bonsai_1406687d0", tpl_1905::<0x14387c5c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1406687d0
    Replacement::draft(0x1406688a0, "bonsai_1406688a0", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432294b0, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_1406688a0
    Replacement::draft(0x140668940, "bonsai_140668940", tpl_6334::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x28, 0x1432294b0, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140668940
    Replacement::draft(0x140668a20, "bonsai_140668a20", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143229938, 0x8> as *const ()), // sub_140668a20
    Replacement::draft(0x140668aa0, "bonsai_140668aa0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143229938> as *const ()), // sub_140668aa0
    Replacement::draft_throws(0x140668cd0, "bonsai_140668cd0", fn_140668cd0 as *const (), Throws::Caught), // sub_140668cd0
    Replacement::draft_throws(0x140668e60, "bonsai_140668e60", tpl_1905::<0x14387c5e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_140668e60
    Replacement::draft(0x140668f40, "bonsai_140668f40", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143229c00, 0x8> as *const ()), // sub_140668f40
    Replacement::draft(0x140668fc0, "bonsai_140668fc0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143229c00> as *const ()), // sub_140668fc0
    Replacement::draft_throws(0x1406691f0, "bonsai_1406691f0", fn_1406691f0 as *const (), Throws::Caught), // sub_1406691f0
    Replacement::draft_throws(0x140669380, "bonsai_140669380", tpl_1905::<0x14387c5f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_140669380
    Replacement::draft(0x140669480, "bonsai_140669480", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143229ea0, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_140669480
    Replacement::draft(0x140669520, "bonsai_140669520", tpl_6334::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x28, 0x143229ea0, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140669520
    Replacement::draft_throws(0x140669640, "bonsai_140669640", fn_140669640 as *const (), Throws::Caught), // sub_140669640
    Replacement::draft_throws(0x1406696d0, "bonsai_1406696d0", tpl_5859::<0x14387c608, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c5d8, 0x14387c578> as *const (), Throws::Discard), // sub_1406696d0
    Replacement::draft(0x1406697e0, "bonsai_1406697e0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14322a190, 0x8> as *const ()), // sub_1406697e0
    Replacement::draft(0x140669860, "bonsai_140669860", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322a190> as *const ()), // sub_140669860
    Replacement::draft_throws(0x140669a90, "bonsai_140669a90", fn_140669a90 as *const (), Throws::Caught), // sub_140669a90
    Replacement::draft_throws(0x140669c20, "bonsai_140669c20", tpl_1905::<0x14387c618, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_140669c20
    Replacement::draft(0x140669d00, "bonsai_140669d00", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14322a298, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_140669d00
    Replacement::draft(0x140669da0, "bonsai_140669da0", tpl_6334::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x28, 0x14322a298, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140669da0
    Replacement::draft_throws(0x140669ec0, "bonsai_140669ec0", fn_140669ec0 as *const (), Throws::Caught), // sub_140669ec0
    Replacement::draft_throws(0x140669f50, "bonsai_140669f50", tpl_5859::<0x14387c628, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c5d8, 0x14387c578> as *const (), Throws::Discard), // sub_140669f50
    Replacement::draft(0x14066a060, "bonsai_14066a060", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14322a718, 0x8> as *const ()), // sub_14066a060
    Replacement::draft(0x14066a0e0, "bonsai_14066a0e0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322a718> as *const ()), // sub_14066a0e0
    Replacement::draft_throws(0x14066a310, "bonsai_14066a310", fn_14066a310 as *const (), Throws::Caught), // sub_14066a310
    Replacement::draft_throws(0x14066a4a0, "bonsai_14066a4a0", tpl_1905::<0x14387c638, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_14066a4a0
    Replacement::draft(0x14066a580, "bonsai_14066a580", tpl_6333::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14322a828, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_14066a580
    Replacement::draft(0x14066a620, "bonsai_14066a620", tpl_6334::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x28, 0x14322a828, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_14066a620
    Replacement::draft_throws(0x14066a740, "bonsai_14066a740", fn_14066a740 as *const (), Throws::Caught), // sub_14066a740
    Replacement::draft_throws(0x14066a7d0, "bonsai_14066a7d0", tpl_5859::<0x14387c648, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c5d8, 0x14387c578> as *const (), Throws::Discard), // sub_14066a7d0
    Replacement::draft(0x14066a8b0, "bonsai_14066a8b0", fn_14066a8b0 as *const ()), // sub_14066a8b0
    Replacement::draft(0x14066ae10, "bonsai_14066ae10", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14322aca8, 0x8> as *const ()), // sub_14066ae10
    Replacement::draft(0x14066ae90, "bonsai_14066ae90", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322aca8> as *const ()), // sub_14066ae90
    Replacement::draft_throws(0x14066b0c0, "bonsai_14066b0c0", fn_14066b0c0 as *const (), Throws::Caught), // sub_14066b0c0
    Replacement::draft_throws(0x14066b250, "bonsai_14066b250", tpl_1905::<0x14387c658, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_14066b250
    Replacement::draft(0x14066b330, "bonsai_14066b330", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14322af70, 0x8> as *const ()), // sub_14066b330
    Replacement::draft(0x14066b3b0, "bonsai_14066b3b0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322af70> as *const ()), // sub_14066b3b0
    Replacement::draft_throws(0x14066b5e0, "bonsai_14066b5e0", fn_14066b5e0 as *const (), Throws::Caught), // sub_14066b5e0
    Replacement::draft_throws(0x14066b770, "bonsai_14066b770", tpl_1905::<0x14387c668, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_14066b770
    Replacement::draft(0x14066b820, "bonsai_14066b820", fn_14066b820 as *const ()), // sub_14066b820
    Replacement::draft(0x14066b8a0, "bonsai_14066b8a0", fn_14066b8a0 as *const ()), // sub_14066b8a0
    Replacement::draft(0x14066b930, "bonsai_14066b930", fn_14066b930 as *const ()), // sub_14066b930
    Replacement::draft_throws(0x14066b9c0, "bonsai_14066b9c0", fn_14066b9c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::eqs::update::system>
    Replacement::draft_throws(0x14066bc10, "std_14066bc10", fn_14066bc10 as *const (), Throws::Discard), // sub_14066bc10
    Replacement::draft(0x14066be30, "unknown_14066be30", tpl_1094::<0x143ca761c, 0x14066c6f0, 0x14020e8f0> as *const ()), // sub_14066be30
    Replacement::draft(0x14066c030, "unknown_14066c030", tpl_1436::<0x58, 0x60, 0x158, 0x143ca761c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14066c030
    Replacement::draft_throws(0x14066c110, "bonsai_14066c110", fn_14066c110 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::eqs::stream_in::system>
    Replacement::draft_throws(0x14066c430, "bonsai_14066c430", tpl_4388::<0x143cc149c, 0x143cc1430, 0x1411e9f50, 0x14066d4d0, 0x143cc1488, 0x1420af120, 0x1420a28e0, 0x14066d000, 0x14066cde0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14322b5e6, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::eqs::stream_out::system>
    Replacement::draft(0x14066c6f0, "unknown_14066c6f0", tpl_5728::<0x1401e6d50, 0x14066ccb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1404bdc00> as *const ()), // sub_14066c6f0
    Replacement::draft_throws(0x14066c870, "unknown_14066c870", fn_14066c870 as *const (), Throws::Discard), // sub_14066c870
    Replacement::draft(0x14066ccb0, "unknown_14066ccb0", fn_14066ccb0 as *const ()), // sub_14066ccb0
    Replacement::draft(0x14066cd10, "unknown_14066cd10", tpl_6086::<0x142407c80, 0x143cada18, 0x580e0, 0x580f0, 0x141b91250> as *const ()), // sub_14066cd10
    Replacement::draft(0x14066cde0, "unknown_14066cde0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14066c030> as *const ()), // sub_14066cde0
    Replacement::draft_throws(0x14066ce80, "unknown_14066ce80", tpl_6367::<0x143cc15c4, 0x20, 0x8, 0x141b91510> as *const (), Throws::Discard), // sub_14066ce80
    Replacement::draft(0x14066cee0, "unknown_14066cee0", tpl_6088::<0x142407c80, 0x143cada18, 0x141b91250> as *const ()), // sub_14066cee0
    Replacement::draft(0x14066d000, "unknown_14066d000", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14066c030> as *const ()), // sub_14066d000
    Replacement::draft(0x14066d0e0, "unknown_14066d0e0", tpl_4539::<0x143cc1428, 0x143cc1378, 0x8, 0x10, 0x18, 0x141834a78, 0x143c8f4d4, 0x143ca761c, 0x143cc1388, 0x143cc1390, 0x141834a0c> as *const ()), // sub_14066d0e0
    Replacement::draft_throws(0x14066d1d0, "unknown_14066d1d0", tpl_6368::<0x14066d760, 0x140206290, 0x1404bce80, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14025f9d0, 0x1401e5730, 0x14066dad0, 0x1401e5380, 0x1401e66f0, 0x142442de0> as *const (), Throws::Discard), // sub_14066d1d0
    Replacement::draft(0x14066d390, "unknown_14066d390", tpl_5928::<0x142537280, 0x1426456c8, 0x1426453a0, 0x14025f9d0, 0x1401e5730> as *const ()), // sub_14066d390
    Replacement::draft(0x14066d4d0, "unknown_14066d4d0", fn_14066d4d0 as *const ()), // sub_14066d4d0
    Replacement::draft(0x14066d5d0, "unknown_14066d5d0", tpl_5305::<0x143cc15c0, 0x143cc1590, 0x8, 0x10, 0x18, 0x141834a78, 0x143c8d694, 0x143c8f4f0, 0x143ca761c, 0x143cc15b0, 0x141834a0c> as *const ()), // sub_14066d5d0
    Replacement::draft_throws(0x14066d6d0, "unknown_14066d6d0", tpl_4541::<0x143cc15c4, 0x14066d0e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14066d6d0
    Replacement::draft(0x14066d760, "unknown_14066d760", fn_14066d760 as *const ()), // sub_14066d760
    Replacement::draft_throws(0x14066d9b0, "unknown_14066d9b0", tpl_4541::<0x143cc1488, 0x14066d4d0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14066d9b0
    Replacement::draft_throws(0x14066da40, "unknown_14066da40", tpl_4541::<0x143cc148c, 0x14066d5d0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14066da40
    Replacement::draft_throws(0x14066dad0, "std_14066dad0", fn_14066dad0 as *const (), Throws::Discard), // sub_14066dad0
    Replacement::draft(0x14066dd20, "unknown_14066dd20", fn_14066dd20 as *const ()), // sub_14066dd20
    Replacement::draft(0x14066de90, "unknown_14066de90", tpl_1526::<0x142793470, 0x1410d6ca0, 0x14066e130, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14066de90
    Replacement::draft(0x14066e130, "navigation_14066e130", tpl_6373::<0x143cc15d4, 0x143cc15d8, 0x1411e9f50, 0x143c8fd28, 0x140215610, 0x143c904a0, 0x1420a28e0, 0x14066e5f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14322c096, 0x142646960, 0x143cc15f8, 0x143cc1600, 0x143cc1604, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<navigation::navigationsystem::update::system>
    Replacement::draft(0x14066e400, "navigation_14066e400", fn_14066e400 as *const ()), // sub_14066e400
    Replacement::draft_throws(0x14066e620, "navigation_14066e620", tpl_6368::<0x14066ed60, 0x140206290, 0x1402128d0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401e5930, 0x1401e5730, 0x14066ef80, 0x1401e5380, 0x1401e66f0, 0x142442de0> as *const (), Throws::Discard), // sub_14066e620
    Replacement::draft(0x14066e7e0, "navigation_14066e7e0", tpl_5928::<0x142537280, 0x1426456c8, 0x1426453a0, 0x1401e5930, 0x1401e5730> as *const ()), // sub_14066e7e0
    Replacement::draft(0x14066e920, "navigation_14066e920", fn_14066e920 as *const ()), // sub_14066e920
    Replacement::draft(0x14066ecf0, "navigation_14066ecf0", fn_14066ecf0 as *const ()), // sub_14066ecf0
    Replacement::draft(0x14066ed60, "unknown_14066ed60", fn_14066ed60 as *const ()), // sub_14066ed60
    Replacement::draft_throws(0x14066ef80, "std_14066ef80", fn_14066ef80 as *const (), Throws::Discard), // sub_14066ef80
    Replacement::draft(0x14066f230, "bonsai_14066f230", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322c328> as *const ()), // sub_14066f230
    Replacement::draft(0x14066f2d0, "bonsai_14066f2d0", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322c328, 0x14183acc2, 0x14065d070> as *const ()), // sub_14066f2d0
    Replacement::draft_throws(0x14066f400, "bonsai_14066f400", fn_14066f400 as *const (), Throws::Caught), // sub_14066f400
    Replacement::draft_throws(0x14066f490, "bonsai_14066f490", tpl_5859::<0x14387c678, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_14066f490
    Replacement::draft(0x14066f5a0, "bonsai_14066f5a0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322c630, 0x15, 0x17, 0x8> as *const ()), // sub_14066f5a0
    Replacement::draft(0x14066f640, "bonsai_14066f640", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322c630, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14066f640
    Replacement::draft_throws(0x14066f890, "bonsai_14066f890", fn_14066f890 as *const (), Throws::Caught), // sub_14066f890
    Replacement::draft_throws(0x14066fa20, "bonsai_14066fa20", tpl_1905::<0x14387c680, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14066fa20
    Replacement::draft(0x14066fb00, "bonsai_14066fb00", tpl_6379::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14322c900, 0x8, 0x1c> as *const ()), // sub_14066fb00
    Replacement::draft(0x14066fb90, "bonsai_14066fb90", fn_14066fb90 as *const ()), // sub_14066fb90
    Replacement::draft_throws(0x14066fdd0, "bonsai_14066fdd0", fn_14066fdd0 as *const (), Throws::Caught), // sub_14066fdd0
    Replacement::draft_throws(0x14066ff60, "bonsai_14066ff60", tpl_1905::<0x14387c690, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14066ff60
    Replacement::draft(0x140670040, "bonsai_140670040", fn_140670040 as *const ()), // sub_140670040
    Replacement::draft(0x1406700f0, "bonsai_1406700f0", fn_1406700f0 as *const ()), // sub_1406700f0
    Replacement::draft(0x1406701b0, "bonsai_1406701b0", tpl_6383::<0x14387c6a0, 0x14243b0b0, 0x14387c9e8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1406701b0
    Replacement::draft_throws(0x140670250, "bonsai_140670250", tpl_6384::<0x14387c6a8, 0x14387c6a0, 0x1401e9fa0, 0x14387c9e4, 0x14387c9e8, 0x1401e9e70> as *const (), Throws::Discard), // sub_140670250
    Replacement::draft_throws(0x1406702d0, "bonsai_1406702d0", tpl_6384::<0x14387c6a8, 0x14387c6a0, 0x1401e9b60, 0x14387c9e4, 0x14387c9e8, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406702d0
    Replacement::draft_throws(0x140670350, "bonsai_140670350", fn_140670350 as *const (), Throws::Caught), // sub_140670350
    Replacement::draft(0x1406703e0, "bonsai_1406703e0", tpl_6386::<0x14387c6a8, 0x14387c6a0, 0x1401e9970, 0x14387c9e4, 0x14387c9e8, 0x1401e9860> as *const ()), // sub_1406703e0
    Replacement::draft_throws(0x140670460, "bonsai_140670460", tpl_6384::<0x14387c6a8, 0x14387c6a0, 0x1401e9770, 0x14387c9e4, 0x14387c9e8, 0x1401e9590> as *const (), Throws::Discard), // sub_140670460
    Replacement::draft_throws(0x1406704e0, "bonsai_1406704e0", tpl_6387::<0x14387c6a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406704e0
    Replacement::draft(0x1406705c0, "bonsai_1406705c0", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322cef0> as *const ()), // sub_1406705c0
    Replacement::draft(0x140670660, "bonsai_140670660", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322cef0, 0x14183acc2, 0x14065d070> as *const ()), // sub_140670660
    Replacement::draft_throws(0x140670790, "bonsai_140670790", fn_140670790 as *const (), Throws::Caught), // sub_140670790
    Replacement::draft_throws(0x140670820, "bonsai_140670820", tpl_5859::<0x14387c6b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_140670820
    Replacement::draft(0x140670900, "bonsai_140670900", fn_140670900 as *const ()), // sub_140670900
    Replacement::draft(0x1406709b0, "bonsai_1406709b0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322d200, 0x15, 0x17, 0x8> as *const ()), // sub_1406709b0
    Replacement::draft(0x140670a50, "bonsai_140670a50", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322d200, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140670a50
    Replacement::draft_throws(0x140670ca0, "bonsai_140670ca0", fn_140670ca0 as *const (), Throws::Caught), // sub_140670ca0
    Replacement::draft_throws(0x140670e30, "bonsai_140670e30", tpl_1905::<0x14387c6c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140670e30
    Replacement::draft(0x140670f10, "bonsai_140670f10", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322d4f8> as *const ()), // sub_140670f10
    Replacement::draft(0x140670fb0, "bonsai_140670fb0", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322d4f8, 0x14183acc2, 0x14065d070> as *const ()), // sub_140670fb0
    Replacement::draft_throws(0x1406710e0, "bonsai_1406710e0", fn_1406710e0 as *const (), Throws::Caught), // sub_1406710e0
    Replacement::draft_throws(0x140671170, "bonsai_140671170", tpl_5859::<0x14387c6c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_140671170
    Replacement::draft(0x140671280, "bonsai_140671280", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322d618> as *const ()), // sub_140671280
    Replacement::draft(0x140671320, "bonsai_140671320", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322d618, 0x14183acc2, 0x14065d070> as *const ()), // sub_140671320
    Replacement::draft_throws(0x140671450, "bonsai_140671450", fn_140671450 as *const (), Throws::Caught), // sub_140671450
    Replacement::draft_throws(0x1406714e0, "bonsai_1406714e0", tpl_5859::<0x14387c6d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406714e0
    Replacement::draft(0x1406715f0, "bonsai_1406715f0", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322d950> as *const ()), // sub_1406715f0
    Replacement::draft(0x140671690, "bonsai_140671690", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322d950, 0x14183acc2, 0x14065d070> as *const ()), // sub_140671690
    Replacement::draft_throws(0x1406717c0, "bonsai_1406717c0", fn_1406717c0 as *const (), Throws::Caught), // sub_1406717c0
    Replacement::draft_throws(0x140671850, "bonsai_140671850", tpl_5859::<0x14387c6e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_140671850
    Replacement::draft(0x140671960, "bonsai_140671960", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322ddf0> as *const ()), // sub_140671960
    Replacement::draft(0x140671a00, "bonsai_140671a00", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322ddf0, 0x14183acc2, 0x14065d070> as *const ()), // sub_140671a00
    Replacement::draft_throws(0x140671b30, "bonsai_140671b30", fn_140671b30 as *const (), Throws::Caught), // sub_140671b30
    Replacement::draft_throws(0x140671bc0, "bonsai_140671bc0", tpl_5859::<0x14387c6f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_140671bc0
    Replacement::draft(0x140671cd0, "bonsai_140671cd0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322e128, 0x15, 0x17, 0x8> as *const ()), // sub_140671cd0
    Replacement::draft(0x140671d70, "bonsai_140671d70", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322e128, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140671d70
    Replacement::draft(0x140671e20, "bonsai_140671e20", tpl_6383::<0x14387c700, 0x14243b0b0, 0x14387c9e8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140671e20
    Replacement::draft_throws(0x140671ec0, "bonsai_140671ec0", tpl_6384::<0x14387c708, 0x14387c700, 0x1401e9fa0, 0x14387c9e4, 0x14387c9e8, 0x1401e9e70> as *const (), Throws::Discard), // sub_140671ec0
    Replacement::draft_throws(0x140671f40, "bonsai_140671f40", tpl_6384::<0x14387c708, 0x14387c700, 0x1401e9b60, 0x14387c9e4, 0x14387c9e8, 0x1401e9a20> as *const (), Throws::Discard), // sub_140671f40
    Replacement::draft_throws(0x140671fc0, "bonsai_140671fc0", fn_140671fc0 as *const (), Throws::Caught), // sub_140671fc0
    Replacement::draft(0x140672050, "bonsai_140672050", tpl_6386::<0x14387c708, 0x14387c700, 0x1401e9970, 0x14387c9e4, 0x14387c9e8, 0x1401e9860> as *const ()), // sub_140672050
    Replacement::draft_throws(0x1406720d0, "bonsai_1406720d0", tpl_6384::<0x14387c708, 0x14387c700, 0x1401e9770, 0x14387c9e4, 0x14387c9e8, 0x1401e9590> as *const (), Throws::Discard), // sub_1406720d0
    Replacement::draft_throws(0x140672150, "bonsai_140672150", tpl_6387::<0x14387c700, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140672150
    Replacement::draft(0x140672230, "bonsai_140672230", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322e420, 0x15, 0x17, 0x8> as *const ()), // sub_140672230
    Replacement::draft(0x1406722d0, "bonsai_1406722d0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322e420, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_1406722d0
    Replacement::draft_throws(0x140672520, "bonsai_140672520", fn_140672520 as *const (), Throws::Caught), // sub_140672520
    Replacement::draft_throws(0x1406726b0, "bonsai_1406726b0", tpl_1905::<0x14387c710, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406726b0
    Replacement::draft(0x140672790, "bonsai_140672790", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x14322e6f0> as *const ()), // sub_140672790
    Replacement::draft(0x140672830, "bonsai_140672830", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322e6f0, 0x14183acc2, 0x14065d070> as *const ()), // sub_140672830
    Replacement::draft_throws(0x140672960, "bonsai_140672960", fn_140672960 as *const (), Throws::Caught), // sub_140672960
    Replacement::draft_throws(0x1406729f0, "bonsai_1406729f0", tpl_5859::<0x14387c720, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406729f0
    Replacement::draft(0x140672b00, "bonsai_140672b00", fn_140672b00 as *const ()), // sub_140672b00
    Replacement::draft(0x140672ba0, "bonsai_140672ba0", fn_140672ba0 as *const ()), // sub_140672ba0
    Replacement::draft_throws(0x140672e00, "bonsai_140672e00", fn_140672e00 as *const (), Throws::Caught), // sub_140672e00
    Replacement::draft_throws(0x140672f90, "bonsai_140672f90", tpl_1905::<0x14387c730, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_140672f90
    Replacement::draft_throws(0x140673040, "bonsai_140673040", fn_140673040 as *const (), Throws::Discard), // sub_140673040
    Replacement::draft(0x140673250, "bonsai_140673250", fn_140673250 as *const ()), // bonsai::AimServiceInstance::bonsai::AimServiceInstance::vfunc_0
    Replacement::draft_throws(0x140673290, "bonsai_140673290", fn_140673290 as *const (), Throws::Discard), // sub_140673290
    Replacement::draft(0x140673330, "bonsai_140673330", tpl_6393::<0xb9> as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_10
    Replacement::draft(0x140673360, "bonsai_140673360", fn_140673360 as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_0
    Replacement::draft(0x1406733d0, "bonsai_1406733d0", tpl_5987::<0x1431f8560, 0x90, 0x1430f6b98, 0x40, 0x1432316d0, 0x141834d30> as *const ()), // bonsai::CoverEnterProgram::bonsai::CoverEnterProgram::vfunc_0
    Replacement::draft(0x140673450, "bonsai_140673450", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x14322ee98, 0x1f, 0x8, 0x1c> as *const ()), // sub_140673450
    Replacement::draft(0x1406734e0, "bonsai_1406734e0", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14322ee98, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_1406734e0
    Replacement::draft_throws(0x140673730, "bonsai_140673730", fn_140673730 as *const (), Throws::Caught), // sub_140673730
    Replacement::draft_throws(0x1406738c0, "bonsai_1406738c0", tpl_1905::<0x14387c740, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_1406738c0
    Replacement::draft(0x140673970, "bonsai_140673970", tpl_2074::<0x14387c750, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140673970
    Replacement::draft_throws(0x140673a10, "bonsai_140673a10", tpl_2075::<0x14387c748, 0x14387c750, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140673a10
    Replacement::draft(0x140673a90, "bonsai_140673a90", tpl_2076::<0x14387c750, 0x14243b0b0, 0x338, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140673a90
    Replacement::draft_throws(0x140673b00, "bonsai_140673b00", tpl_2077::<0x14387c748, 0x14387c750, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140673b00
    Replacement::draft_throws(0x140673b90, "bonsai_140673b90", fn_140673b90 as *const (), Throws::Caught), // sub_140673b90
    Replacement::draft_throws(0x140673c50, "bonsai_140673c50", fn_140673c50 as *const (), Throws::Caught), // sub_140673c50
    Replacement::draft_throws(0x140673d10, "bonsai_140673d10", fn_140673d10 as *const (), Throws::Caught), // sub_140673d10
    Replacement::draft(0x140673dd0, "bonsai_140673dd0", tpl_2079::<0x14387c750, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140673dd0
    Replacement::draft(0x140673e40, "bonsai_140673e40", tpl_2080::<0x14387c748, 0x14387c750, 0x1401e9970, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140673e40
    Replacement::draft(0x140673eb0, "bonsai_140673eb0", tpl_2076::<0x14387c750, 0x14243b0b0, 0x338, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140673eb0
    Replacement::draft_throws(0x140673f20, "bonsai_140673f20", tpl_2081::<0x14387c748, 0x14387c750, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140673f20
    Replacement::draft_throws(0x140673f90, "bonsai_140673f90", tpl_2082::<0x14387c750, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14322ef70, 0x1, 0x1402c4e50> as *const (), Throws::Discard), // sub_140673f90
    Replacement::draft(0x140674080, "bonsai_140674080", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322f168, 0x15, 0x17, 0x8> as *const ()), // sub_140674080
    Replacement::draft(0x140674120, "bonsai_140674120", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322f168, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140674120
    Replacement::draft_throws(0x140674370, "bonsai_140674370", fn_140674370 as *const (), Throws::Caught), // sub_140674370
    Replacement::draft_throws(0x140674500, "bonsai_140674500", tpl_1905::<0x14387c760, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140674500
    Replacement::draft_throws(0x1406745b0, "bonsai_1406745b0", fn_1406745b0 as *const (), Throws::Discard), // sub_1406745b0
    Replacement::draft(0x140674870, "bonsai_140674870", fn_140674870 as *const ()), // sub_140674870
    Replacement::draft_throws(0x1406749f0, "bonsai_1406749f0", fn_1406749f0 as *const (), Throws::Discard), // sub_1406749f0
    Replacement::draft_throws(0x140674a80, "bonsai_140674a80", fn_140674a80 as *const (), Throws::Discard), // sub_140674a80
    Replacement::draft(0x140674b60, "bonsai_140674b60", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322f480, 0x15, 0x17, 0x8> as *const ()), // sub_140674b60
    Replacement::draft(0x140674c00, "bonsai_140674c00", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322f480, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140674c00
    Replacement::draft_throws(0x140674e50, "bonsai_140674e50", fn_140674e50 as *const (), Throws::Caught), // sub_140674e50
    Replacement::draft_throws(0x140674fe0, "bonsai_140674fe0", tpl_1905::<0x14387c770, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140674fe0
    Replacement::draft(0x1406750c0, "bonsai_1406750c0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14322f910, 0x15, 0x17, 0x8> as *const ()), // sub_1406750c0
    Replacement::draft(0x140675160, "bonsai_140675160", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14322f910, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140675160
    Replacement::draft_throws(0x1406753b0, "bonsai_1406753b0", fn_1406753b0 as *const (), Throws::Caught), // sub_1406753b0
    Replacement::draft_throws(0x140675540, "bonsai_140675540", tpl_1905::<0x14387c778, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140675540
    Replacement::draft(0x1406755f0, "bonsai_1406755f0", tpl_1094::<0x143ca7608, 0x140675820, 0x14020e8f0> as *const ()), // sub_1406755f0
    Replacement::draft(0x140675820, "bonsai_140675820", fn_140675820 as *const ()), // sub_140675820
    Replacement::draft(0x1406759c0, "bonsai_1406759c0", fn_1406759c0 as *const ()), // bonsai::BonsaiLinePath::bonsai::BonsaiLinePath::vfunc_0
    Replacement::draft(0x140675a90, "unknown_140675a90", fn_140675a90 as *const ()), // sub_140675a90
    Replacement::draft(0x140675c00, "unknown_140675c00", tpl_6078::<0x30, 0x143cc180c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140675c86> as *const ()), // sub_140675c00
    Replacement::draft(0x140675c90, "unknown_140675c90", fn_140675c90 as *const ()), // sub_140675c90
    Replacement::draft(0x140675ce0, "unknown_140675ce0", fn_140675ce0 as *const ()), // sub_140675ce0
    Replacement::draft_throws(0x140675d80, "bonsai_140675d80", tpl_4361::<0x143cc16c8, 0x143cc1720, 0x1411e9f50, 0x140679b20, 0x143cc1800, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x1406786e0, 0x140677b80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14322fa96, 0x142646960, 0x143cc1740, 0x143cc1744, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bonsaiscript::update::system>
    Replacement::draft_throws(0x140676090, "bonsai_140676090", tpl_3059::<0x143cc16c0, 0x143cc1790, 0x1411e9f50, 0x140677c80, 0x1406788a0, 0x140677e70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14322fc36, 0x142646960, 0x143cc17b0, 0x143cc17c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bonsaiscript::update_post_physics::system>
    Replacement::draft_throws(0x1406762e0, "bonsai_1406762e0", tpl_6402::<0x143cc1a9c, 0x143cc19e8, 0x1406774e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14322ff46, 0x142646960, 0x143cc1a08, 0x143cc1a18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bonsaiscript::check_path_completion::system>
    Replacement::draft(0x1406764f0, "bonsai_1406764f0", fn_1406764f0 as *const ()), // ecs::system::ecs::system::wrapper<bonsai::bonsaiscript::query_removed::system>
    Replacement::draft(0x1406767e0, "bonsai_1406767e0", tpl_1094::<0x143cc180c, 0x1406776a0, 0x14020e8f0> as *const ()), // sub_1406767e0
    Replacement::draft(0x1406769e0, "bonsai_1406769e0", tpl_1403::<0x143cc1804, 0x140677880, 0x14020e8f0> as *const ()), // sub_1406769e0
    Replacement::draft(0x140676bc0, "bonsai_140676bc0", tpl_1094::<0x143c9fca8, 0x140677a00, 0x14020e8f0> as *const ()), // sub_140676bc0
    Replacement::draft(0x140676dc0, "bonsai_140676dc0", tpl_1436::<0x58, 0x60, 0x158, 0x143c9fca8, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140676dc0
    Replacement::draft_throws(0x140676ed0, "bonsai_140676ed0", tpl_3086::<0x143cc17c4, 0x143cc1748, 0x1411e9f50, 0x140679d00, 0x143cc1a58, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x1406795d0, 0x140678580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143230216, 0x142646960, 0x143cc1768, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bonsaiscript::stream_in::system>
    Replacement::draft_throws(0x1406771d0, "bonsai_1406771d0", fn_1406771d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bonsaiscript::stream_out::system>
    Replacement::draft_throws(0x1406774e0, "unknown_1406774e0", tpl_5552::<0x1411e9f50, 0x140679870, 0x143cc1970, 0x1420af120, 0x143c8ceec, 0x140215610, 0x143c8fd28, 0x1420a28e0, 0x140678b80, 0x140678070> as *const (), Throws::Discard), // sub_1406774e0
    Replacement::draft(0x1406776a0, "unknown_1406776a0", fn_1406776a0 as *const ()), // sub_1406776a0
    Replacement::draft(0x140677880, "unknown_140677880", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143231058> as *const ()), // sub_140677880
    Replacement::draft(0x140677a00, "unknown_140677a00", fn_140677a00 as *const ()), // sub_140677a00
    Replacement::draft(0x140677b80, "unknown_140677b80", tpl_64057::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x1406783c0, 0x141ba0930> as *const ()), // sub_140677b80
    Replacement::draft_throws(0x140677c80, "unknown_140677c80", tpl_3110::<0x140679a10, 0x143cc1808, 0x1420af120, 0x140679960, 0x143cc1974, 0x140215750, 0x143c8fd28, 0x140215610, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_140677c80
    Replacement::draft(0x140677e70, "unknown_140677e70", tpl_1408::<0x143cc1974, 0x20, 0x8, 0x143c8d08c, 0x143c8fd28, 0x140678290, 0x140677f90> as *const ()), // sub_140677e70
    Replacement::draft(0x140677f90, "unknown_140677f90", tpl_5107::<0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x141ba0560> as *const ()), // sub_140677f90
    Replacement::draft(0x140678070, "unknown_140678070", tpl_63708::<0x88, 0x10088, 0x580e0, 0x143c8fd28, 0x143c8ceec, 0x140678190, 0x141ba03e0> as *const ()), // sub_140678070
    Replacement::draft(0x1406784c0, "unknown_1406784c0", fn_1406784c0 as *const ()), // sub_1406784c0
    Replacement::draft(0x140678530, "unknown_140678530", fn_140678530 as *const ()), // sub_140678530
    Replacement::draft(0x140678580, "unknown_140678580", tpl_6086::<0x142407c80, 0x143cada18, 0x580e0, 0x580f0, 0x141ba0d30> as *const ()), // sub_140678580
    Replacement::draft(0x140678650, "unknown_140678650", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x140676dc0> as *const ()), // sub_140678650
    Replacement::draft(0x1406786e0, "unknown_1406786e0", tpl_64074::<0x143c8ceec, 0x143c9fca8, 0x142489ee0, 0x143c8d160, 0x141ba0930> as *const ()), // sub_1406786e0
    Replacement::draft(0x1406788a0, "unknown_1406788a0", tpl_61144::<0x143cc1974, 0x143c8fd28, 0x143c8d08c, 0x143cc180c, 0x142489ee0, 0x143c8d120, 0x143c8ff58, 0x141ba0560> as *const ()), // sub_1406788a0
    Replacement::draft(0x140678b80, "unknown_140678b80", tpl_63711::<0x143c8fd28, 0x143c8ceec, 0x143cc180c, 0x142489ee0, 0x143c8d160, 0x141ba03e0> as *const ()), // sub_140678b80
    Replacement::draft(0x140678d80, "unknown_140678d80", tpl_3172::<0x143cc180c, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141ba0ba0, 0x140679290> as *const ()), // sub_140678d80
    Replacement::draft(0x140678e30, "unknown_140678e30", tpl_3188::<0x1406797d0, 0x143231020, 0x140679290, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140678e30
    Replacement::draft(0x1406790d0, "unknown_1406790d0", fn_1406790d0 as *const ()), // sub_1406790d0
    Replacement::draft(0x140679290, "unknown_140679290", fn_140679290 as *const ()), // sub_140679290
    Replacement::draft(0x1406795d0, "unknown_1406795d0", tpl_6088::<0x142407c80, 0x143cada18, 0x141ba0d30> as *const ()), // sub_1406795d0
    Replacement::draft(0x1406796f0, "unknown_1406796f0", tpl_6412::<0x8, 0x142407c80, 0x88, 0x10088, 0x10, 0x580f0, 0x140676dc0> as *const ()), // sub_1406796f0
    Replacement::draft(0x1406797d0, "unknown_1406797d0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143231020, 0x8, 0x20> as *const ()), // sub_1406797d0
    Replacement::draft(0x14067a160, "unknown_14067a160", tpl_1094::<0x143ca78c8, 0x14067a390, 0x14020e8f0> as *const ()), // sub_14067a160
    Replacement::draft(0x14067a390, "unknown_14067a390", fn_14067a390 as *const ()), // sub_14067a390
    Replacement::draft(0x14067a590, "unknown_14067a590", fn_14067a590 as *const ()), // sub_14067a590
    Replacement::draft(0x14067a670, "unknown_14067a670", fn_14067a670 as *const ()), // sub_14067a670
    Replacement::draft(0x14067a830, "unknown_14067a830", fn_14067a830 as *const ()), // sub_14067a830
    Replacement::draft(0x14067a970, "unknown_14067a970", fn_14067a970 as *const ()), // sub_14067a970
    Replacement::draft(0x14067aa90, "unknown_14067aa90", fn_14067aa90 as *const ()), // sub_14067aa90
    Replacement::draft(0x14067abd0, "unknown_14067abd0", fn_14067abd0 as *const ()), // sub_14067abd0
    Replacement::draft(0x14067ad00, "unknown_14067ad00", fn_14067ad00 as *const ()), // sub_14067ad00
    Replacement::draft(0x14067ae80, "unknown_14067ae80", fn_14067ae80 as *const ()), // sub_14067ae80
    Replacement::draft(0x14067afc0, "unknown_14067afc0", tpl_6422::<0x142535070, 0x1426456c8, 0x1426453a0, 0x141ba4aa0, 0x1432405f0, 0x1430f6b98, 0x143231900, 0x1431f8560> as *const ()), // sub_14067afc0
    Replacement::draft(0x14067b120, "unknown_14067b120", fn_14067b120 as *const ()), // sub_14067b120
    Replacement::draft(0x14067b260, "unknown_14067b260", tpl_6422::<0x142535070, 0x1426456c8, 0x1426453a0, 0x141ba4aa0, 0x143240540, 0x1430f6b98, 0x14323f990, 0x1431f8560> as *const ()), // sub_14067b260
    Replacement::draft(0x14067b3c0, "unknown_14067b3c0", fn_14067b3c0 as *const ()), // sub_14067b3c0
    Replacement::draft(0x14067b4e0, "unknown_14067b4e0", tpl_5852::<0x142535070, 0x1426456c8, 0x1426453a0, 0x141ba4aa0, 0x143242728, 0x1430f6b98, 0x1431ea988, 0x1431f8560> as *const ()), // sub_14067b4e0
    Replacement::draft(0x14067b660, "ak_14067b660", fn_14067b660 as *const ()), // AK::Monitor::AK::Monitor::AddTranslator
    Replacement::draft(0x14067b6f0, "unknown_14067b6f0", fn_14067b6f0 as *const ()), // sub_14067b6f0
    Replacement::draft(0x14067b7c0, "unknown_14067b7c0", fn_14067b7c0 as *const ()), // sub_14067b7c0
    Replacement::draft(0x14067b850, "unknown_14067b850", fn_14067b850 as *const ()), // sub_14067b850
    Replacement::draft(0x14067b8e0, "unknown_14067b8e0", tpl_4888::<0x30, 0x143c8de68, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14067b963> as *const ()), // sub_14067b8e0
    Replacement::draft(0x14067bac0, "unknown_14067bac0", tpl_6428::<0x8, 0x142442de0> as *const ()), // sub_14067bac0
    Replacement::draft(0x14067bb40, "unknown_14067bb40", tpl_6429::<0xc, 0x8, 0x140680330> as *const ()), // sub_14067bb40
    Replacement::draft(0x14067bbc0, "unknown_14067bbc0", fn_14067bbc0 as *const ()), // sub_14067bbc0
    Replacement::draft(0x14067bc60, "unknown_14067bc60", tpl_1171::<0x58, 0x60, 0x143cc239c, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14067bc60
    Replacement::draft(0x14067bd40, "unknown_14067bd40", tpl_3942::<0x30, 0x143cbe480, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14067bd90> as *const ()), // sub_14067bd40
    Replacement::draft(0x14067bda0, "unknown_14067bda0", tpl_3942::<0x30, 0x143c8de68, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14067bdf0> as *const ()), // sub_14067bda0
    Replacement::draft(0x14067be00, "bonsai_14067be00", tpl_6431::<0x143cc1d84, 0x143cc1fd8, 0x1411e9f50, 0x140684700, 0x143c8ff58, 0x1401fc320, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x1406835e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432319c6, 0x142646960, 0x143cc1ff8, 0x143cc2000, 0x143cc2004, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::bodyControllerRemoved>
    Replacement::draft_throws(0x14067c0f0, "bonsai_14067c0f0", fn_14067c0f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::bodyControllerEnableNavigationAgent>
    Replacement::draft_throws(0x14067c420, "bonsai_14067c420", fn_14067c420 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::bodyControllerDisableNavigationAgent>
    Replacement::draft_throws(0x14067c750, "bonsai_14067c750", fn_14067c750 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::bodyControllerUpdate>
    Replacement::draft_throws(0x14067cb40, "bonsai_14067cb40", tpl_3202::<0x143cc200c, 0x143cc2018, 0x1411e9f50, 0x140686e50, 0x143cc2010, 0x1420af120, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x140683d80, 0x140681220, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143231d46, 0x142646960, 0x143cc2038, 0x143cc2048, 0x143cc204c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::updatePredictPositionOnSpline>
    Replacement::draft_throws(0x14067ce70, "bonsai_14067ce70", tpl_4194::<0x143cc1ce4, 0x143cc1c08, 0x1411e9f50, 0x140681340, 0x140683f00, 0x140681500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143232076, 0x142646960, 0x143cc1c28, 0x143cc1c38, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::bodyControllerUpdatePostPhysics>
    Replacement::draft_throws(0x14067d0c0, "bonsai_14067d0c0", tpl_3621::<0x143cc1c00, 0x143cc2518, 0x1411e9f50, 0x1406817c0, 0x140684210, 0x140681980, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143233676, 0x142646960, 0x143cc2538, 0x143cc2548, 0x143cc2550, 0x143cc2554, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::update_quadruped_post_physics::system>
    Replacement::draft_throws(0x14067d320, "bonsai_14067d320", fn_14067d320 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::bodyControllerUpdateAirNavigation>
    Replacement::draft(0x14067d640, "bonsai_14067d640", fn_14067d640 as *const ()), // sub_14067d640
    Replacement::draft(0x14067d6c0, "bonsai_14067d6c0", tpl_1094::<0x143cc0b4c, 0x1406804a0, 0x14020e8f0> as *const ()), // sub_14067d6c0
    Replacement::draft(0x14067d8c0, "bonsai_14067d8c0", tpl_1094::<0x143cc25cc, 0x140680680, 0x14020e8f0> as *const ()), // sub_14067d8c0
    Replacement::draft(0x14067dac0, "bonsai_14067dac0", tpl_1095::<0x143c8f4f0, 0x14020e8f0> as *const ()), // sub_14067dac0
    Replacement::draft(0x14067dca0, "bonsai_14067dca0", tpl_1466::<0x143cc1fc8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067dca0
    Replacement::draft(0x14067df70, "bonsai_14067df70", tpl_1094::<0x143c9a7f4, 0x140680800, 0x14020e8f0> as *const ()), // sub_14067df70
    Replacement::draft(0x14067e170, "bonsai_14067e170", tpl_3073::<0x143c9a7f8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067e170
    Replacement::draft(0x14067e4d0, "bonsai_14067e4d0", tpl_1402::<0x143cc2090, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067e4d0
    Replacement::draft(0x14067e820, "bonsai_14067e820", tpl_1466::<0x143cc1d88, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067e820
    Replacement::draft(0x14067eaf0, "bonsai_14067eaf0", tpl_1466::<0x143cc1fc4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067eaf0
    Replacement::draft(0x14067edc0, "bonsai_14067edc0", tpl_1466::<0x143cc2244, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067edc0
    Replacement::draft(0x14067f090, "bonsai_14067f090", tpl_1466::<0x143ca7878, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067f090
    Replacement::draft(0x14067f360, "bonsai_14067f360", tpl_3278::<0x143ca7600, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14067f360
    Replacement::draft(0x14067f6a0, "bonsai_14067f6a0", tpl_1094::<0x143c8ff58, 0x140680980, 0x14020e8f0> as *const ()), // sub_14067f6a0
    Replacement::draft(0x14067f8a0, "bonsai_14067f8a0", tpl_1095::<0x143cc239c, 0x14020e8f0> as *const ()), // sub_14067f8a0
    Replacement::draft_throws(0x14067fa80, "bonsai_14067fa80", tpl_6282::<0x143cc23e8, 0x140680b00, 0x14020e8f0> as *const (), Throws::Discard), // sub_14067fa80
    Replacement::draft_throws(0x14067fce0, "bonsai_14067fce0", fn_14067fce0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::streamIn>
    Replacement::draft_throws(0x140680020, "bonsai_140680020", fn_140680020 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::bodycontroller::streamOut>
    Replacement::draft(0x140680330, "unknown_140680330", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1403d0db0> as *const ()), // sub_140680330
    Replacement::draft(0x1406804a0, "unknown_1406804a0", fn_1406804a0 as *const ()), // sub_1406804a0
    Replacement::draft(0x140680680, "unknown_140680680", fn_140680680 as *const ()), // sub_140680680
    Replacement::draft(0x140680800, "unknown_140680800", tpl_6441::<0x1401e6d50, 0x1406831d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1403d0b70> as *const ()), // sub_140680800
    Replacement::draft(0x140680980, "unknown_140680980", fn_140680980 as *const ()), // sub_140680980
    Replacement::draft_throws(0x140680b00, "unknown_140680b00", fn_140680b00 as *const (), Throws::Discard), // sub_140680b00
    Replacement::draft(0x140680cf0, "unknown_140680cf0", tpl_1377::<0x142407c80, 0x143c8fd28, 0x143c8ff58, 0x142489ee0, 0x141ba80c0> as *const ()), // sub_140680cf0
    Replacement::draft(0x140680e30, "unknown_140680e30", tpl_1377::<0x142407c80, 0x143c8fd28, 0x143c8ff58, 0x142489ee0, 0x141ba8040> as *const ()), // sub_140680e30
    Replacement::draft_throws(0x140680f70, "unknown_140680f70", fn_140680f70 as *const (), Throws::Discard), // sub_140680f70
    Replacement::draft_throws(0x140681050, "unknown_140681050", fn_140681050 as *const (), Throws::Discard), // sub_140681050
    Replacement::draft(0x140681220, "unknown_140681220", fn_140681220 as *const ()), // sub_140681220
    Replacement::draft_throws(0x140681340, "unknown_140681340", tpl_6446::<0x1406861e0, 0x143cc2094, 0x1420af120, 0x140682630, 0x143c8fd28, 0x140215610, 0x143c8ceec, 0x143c8d08c, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_140681340
    Replacement::draft_throws(0x140681500, "unknown_140681500", tpl_63004::<0x143cc212c, 0x143c8d910, 0x143c8fd28, 0x143c8ceec, 0x143c8d08c, 0x140682330, 0x140681660> as *const (), Throws::Discard), // sub_140681500
    Replacement::draft_throws(0x140681660, "unknown_140681660", tpl_3687::<0x141ba67c0> as *const (), Throws::Discard), // sub_140681660
    Replacement::draft_throws(0x1406817c0, "unknown_1406817c0", tpl_6446::<0x140685e30, 0x143cc2234, 0x1420af120, 0x140682630, 0x143c8fd28, 0x140215610, 0x143c8ceec, 0x143c8d08c, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_1406817c0
    Replacement::draft_throws(0x140681980, "unknown_140681980", fn_140681980 as *const (), Throws::Discard), // sub_140681980
    Replacement::draft_throws(0x140681ae0, "unknown_140681ae0", fn_140681ae0 as *const (), Throws::Discard), // sub_140681ae0
    Replacement::draft(0x140681c30, "unknown_140681c30", tpl_60899::<0x143c8d910, 0x140681e80, 0x141ba6610> as *const ()), // sub_140681c30
    Replacement::draft(0x140681d80, "unknown_140681d80", fn_140681d80 as *const ()), // sub_140681d80
    Replacement::draft(0x140681e00, "content_140681e00", fn_140681e00 as *const ()), // content::NavigationAvoidanceConfigurationComponent::content::NavigationAvoidanceConfigurationComponent::vfunc_0
    Replacement::draft_throws(0x140682630, "unknown_140682630", tpl_1330::<0x8, 0x140686090, 0x143cc212c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140682630
    Replacement::draft(0x140682710, "unknown_140682710", fn_140682710 as *const ()), // sub_140682710
    Replacement::draft(0x140682840, "unknown_140682840", fn_140682840 as *const ()), // sub_140682840
    Replacement::draft(0x140682d70, "unknown_140682d70", fn_140682d70 as *const ()), // sub_140682d70
    Replacement::draft(0x140683010, "unknown_140683010", tpl_4382::<0x143cc25cc, 0x142489ee0, 0x143c8ff58, 0x143c97ad0, 0x143c91f1c, 0x143ca7610, 0x143c8f4d4> as *const ()), // sub_140683010
    Replacement::draft(0x1406831a0, "unknown_1406831a0", tpl_3324::<0x210, 0x1e0> as *const ()), // sub_1406831a0
    Replacement::draft(0x1406831d0, "unknown_1406831d0", fn_1406831d0 as *const ()), // sub_1406831d0
    Replacement::draft(0x140683240, "unknown_140683240", fn_140683240 as *const ()), // sub_140683240
    Replacement::draft_throws(0x1406832a0, "unknown_1406832a0", fn_1406832a0 as *const (), Throws::Caught), // sub_1406832a0
    Replacement::draft_throws(0x1406833b0, "unknown_1406833b0", fn_1406833b0 as *const (), Throws::Discard), // sub_1406833b0
    Replacement::draft(0x1406834a0, "unknown_1406834a0", fn_1406834a0 as *const ()), // sub_1406834a0
    Replacement::draft(0x1406835e0, "unknown_1406835e0", tpl_3172::<0x143c8ff58, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141ba84b0, 0x140684ef0> as *const ()), // sub_1406835e0
    Replacement::draft(0x140683690, "unknown_140683690", tpl_6262::<0x142407c80, 0x143c8fd28, 0x143c8ff58, 0x142489ee0, 0x141ba80c0> as *const ()), // sub_140683690
    Replacement::draft(0x140683810, "unknown_140683810", tpl_6262::<0x142407c80, 0x143c8fd28, 0x143c8ff58, 0x142489ee0, 0x141ba8040> as *const ()), // sub_140683810
    Replacement::draft_throws(0x140683990, "unknown_140683990", fn_140683990 as *const (), Throws::Discard), // sub_140683990
    Replacement::draft(0x140683c90, "unknown_140683c90", fn_140683c90 as *const ()), // sub_140683c90
    Replacement::draft(0x140683d80, "unknown_140683d80", fn_140683d80 as *const ()), // sub_140683d80
    Replacement::draft_throws(0x140683f00, "unknown_140683f00", tpl_63005::<0x143cc212c, 0x143c8d910, 0x143c8fd28, 0x143c8ceec, 0x143c8d08c, 0x140682430, 0x141ba67c0> as *const (), Throws::Discard), // sub_140683f00
    Replacement::draft_throws(0x140684210, "unknown_140684210", fn_140684210 as *const (), Throws::Discard), // sub_140684210
    Replacement::draft(0x140684530, "unknown_140684530", tpl_60914::<0x143c8d910, 0x140681e80, 0x141ba6610> as *const ()), // sub_140684530
    Replacement::draft(0x140684700, "unknown_140684700", tpl_3188::<0x140686b50, 0x143231940, 0x140684ef0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140684700
    Replacement::draft(0x140684da0, "unknown_140684da0", fn_140684da0 as *const ()), // sub_140684da0
    Replacement::draft(0x140684ef0, "unknown_140684ef0", fn_140684ef0 as *const ()), // sub_140684ef0
    Replacement::draft(0x140685020, "unknown_140685020", tpl_6462::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140685020
    Replacement::draft_throws(0x140685840, "unknown_140685840", fn_140685840 as *const (), Throws::Caught), // sub_140685840
    Replacement::draft(0x140685a70, "unknown_140685a70", fn_140685a70 as *const ()), // sub_140685a70
    Replacement::draft(0x140685e30, "unknown_140685e30", fn_140685e30 as *const ()), // sub_140685e30
    Replacement::draft(0x1406861e0, "unknown_1406861e0", fn_1406861e0 as *const ()), // sub_1406861e0
    Replacement::draft(0x140686440, "unknown_140686440", tpl_3326::<0x143cc1e60, 0x143cc1f84, 0x10, 0x18, 0x8, 0x80, 0xffffffffffffff80, 0xffffffffffffff90, 0xffffffffffffffa0, 0xffffffffffffffb0, 0xffffffffffffffc0, 0xffffffffffffffd0, 0x141834a78, 0x140686560, 0x141834a0c> as *const ()), // sub_140686440
    Replacement::draft(0x140686560, "unknown_140686560", fn_140686560 as *const ()), // sub_140686560
    Replacement::draft(0x140686b50, "unknown_140686b50", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143231940, 0x8, 0x20> as *const ()), // sub_140686b50
    Replacement::draft_throws(0x140686bf0, "std_140686bf0", fn_140686bf0 as *const (), Throws::Discard), // sub_140686bf0
    Replacement::draft(0x140686e50, "unknown_140686e50", fn_140686e50 as *const ()), // sub_140686e50
    Replacement::draft(0x140686fe0, "unknown_140686fe0", fn_140686fe0 as *const ()), // sub_140686fe0
    Replacement::draft(0x140687120, "unknown_140687120", fn_140687120 as *const ()), // sub_140687120
    Replacement::draft_throws(0x140687330, "unknown_140687330", tpl_4541::<0x143cc212c, 0x140686090, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687330
    Replacement::draft_throws(0x1406873c0, "unknown_1406873c0", tpl_4541::<0x143cc1dd0, 0x140685cb0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1406873c0
    Replacement::draft_throws(0x140687450, "unknown_140687450", tpl_4541::<0x143cc2234, 0x140685e30, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687450
    Replacement::draft_throws(0x1406874e0, "unknown_1406874e0", tpl_4541::<0x143cc2094, 0x1406861e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1406874e0
    Replacement::draft_throws(0x140687570, "unknown_140687570", tpl_3328::<0x143cc2128, 0x140686440, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687570
    Replacement::draft_throws(0x140687600, "unknown_140687600", tpl_4541::<0x143cc2390, 0x1406868e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687600
    Replacement::draft_throws(0x140687690, "unknown_140687690", tpl_4541::<0x143cc2590, 0x140686a10, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687690
    Replacement::draft_throws(0x140687720, "unknown_140687720", tpl_4541::<0x143cc2010, 0x140686e50, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687720
    Replacement::draft_throws(0x1406877b0, "unknown_1406877b0", tpl_4541::<0x143cc2130, 0x140686fe0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1406877b0
    Replacement::draft_throws(0x140687840, "unknown_140687840", tpl_4541::<0x143cc2270, 0x140687120, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140687840
    Replacement::draft(0x1406878d0, "unknown_1406878d0", fn_1406878d0 as *const ()), // sub_1406878d0
    Replacement::draft(0x140687a00, "bonsai_140687a00", tpl_6473::<0x78, 0x10, 0x18, 0xffffffffffffffa0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x143237348, 0x28> as *const ()), // sub_140687a00
    Replacement::draft(0x140687ad0, "bonsai_140687ad0", tpl_6474::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143237348, 0x22, 0x26, 0x49, 0x4d> as *const ()), // sub_140687ad0
    Replacement::draft_throws(0x140687d40, "bonsai_140687d40", fn_140687d40 as *const (), Throws::Caught), // sub_140687d40
    Replacement::draft_throws(0x140687ef0, "bonsai_140687ef0", tpl_5858::<0x14387c838, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140687ef0
    Replacement::draft(0x140687fb0, "bonsai_140687fb0", fn_140687fb0 as *const ()), // sub_140687fb0
    Replacement::draft(0x1406880a0, "bonsai_1406880a0", fn_1406880a0 as *const ()), // sub_1406880a0
    Replacement::draft_throws(0x140688330, "bonsai_140688330", fn_140688330 as *const (), Throws::Caught), // sub_140688330
    Replacement::draft_throws(0x1406884e0, "bonsai_1406884e0", tpl_5858::<0x14387c848, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1406884e0
    Replacement::draft(0x1406885a0, "bonsai_1406885a0", tpl_6120::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x28, 0x30, 0x38, 0x1431ea018> as *const ()), // sub_1406885a0
    Replacement::draft(0x140688660, "bonsai_140688660", tpl_6121::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x1431ea018, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140688660
    Replacement::draft(0x140688760, "bonsai_140688760", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x143237b70, 0x28, 0x30> as *const ()), // sub_140688760
    Replacement::draft(0x140688820, "bonsai_140688820", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143237b70, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140688820
    Replacement::draft_throws(0x140688a80, "bonsai_140688a80", fn_140688a80 as *const (), Throws::Caught), // sub_140688a80
    Replacement::draft_throws(0x140688c30, "bonsai_140688c30", tpl_5858::<0x14387c860, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140688c30
    Replacement::draft(0x140688cf0, "bonsai_140688cf0", fn_140688cf0 as *const ()), // sub_140688cf0
    Replacement::draft(0x140688db0, "bonsai_140688db0", fn_140688db0 as *const ()), // sub_140688db0
    Replacement::draft_throws(0x140689010, "bonsai_140689010", fn_140689010 as *const (), Throws::Caught), // sub_140689010
    Replacement::draft_throws(0x1406891c0, "bonsai_1406891c0", tpl_5858::<0x14387c870, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1406891c0
    Replacement::draft(0x140689250, "bonsai_140689250", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x1431ea3b8, 0x28, 0x30> as *const ()), // sub_140689250
    Replacement::draft(0x140689310, "bonsai_140689310", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x1431ea3b8, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140689310
    Replacement::draft(0x140689410, "bonsai_140689410", fn_140689410 as *const ()), // sub_140689410
    Replacement::draft(0x1406894f0, "bonsai_1406894f0", fn_1406894f0 as *const ()), // sub_1406894f0
    Replacement::draft_throws(0x140689760, "bonsai_140689760", fn_140689760 as *const (), Throws::Caught), // sub_140689760
    Replacement::draft_throws(0x140689910, "bonsai_140689910", tpl_5858::<0x14387c890, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140689910
    Replacement::draft(0x1406899a0, "bonsai_1406899a0", tpl_1171::<0x58, 0x60, 0x143ca7868, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1406899a0
    Replacement::draft_throws(0x140689a80, "bonsai_140689a80", fn_140689a80 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree::removeBehaviorTree>
    Replacement::draft_throws(0x140689d40, "bonsai_140689d40", tpl_6141::<0x143cc2708, 0x143cc2880, 0x1411e9f50, 0x14068bc60, 0x143ca7868, 0x1401fc320, 0x14068b7f0, 0x1420a28e0, 0x14068bbc0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143238966, 0x142646960, 0x143cc28a0, 0x143cc28b0, 0x143cc28b2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree::removeBehaviorTreeBTResource>
    Replacement::draft(0x14068a020, "bonsai_14068a020", tpl_1094::<0x143ca7620, 0x14068b1b0, 0x14020e8f0> as *const ()), // sub_14068a020
    Replacement::draft(0x14068a220, "bonsai_14068a220", tpl_1094::<0x143ca7868, 0x14068b330, 0x14020e8f0> as *const ()), // sub_14068a220
    Replacement::draft(0x14068a420, "bonsai_14068a420", tpl_1094::<0x143c9a664, 0x14068b4b0, 0x14020e8f0> as *const ()), // sub_14068a420
    Replacement::draft_throws(0x14068a650, "bonsai_14068a650", fn_14068a650 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree::streamIn>
    Replacement::draft_throws(0x14068a970, "bonsai_14068a970", tpl_6484::<0x143cc2770, 0x143cc2840, 0x1411e9f50, 0x14068cbf0, 0x143cc2838, 0x1420af120, 0x140215750, 0x14068b7f0, 0x1420a28e0, 0x14068c8b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432394f6, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree::streamOut>
    Replacement::draft(0x14068ac70, "unknown_14068ac70", fn_14068ac70 as *const ()), // sub_14068ac70
    Replacement::draft_throws(0x14068ae80, "coregame_14068ae80", tpl_6486::<0x143cc283c, 0x143cc2710, 0x14068b630, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143239836, 0x142646960, 0x143cc2730, 0x143cc2750, 0x143cc2760, 0x143cc2768, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::BehaviorTreeCompone
    Replacement::draft(0x14068b1b0, "unknown_14068b1b0", fn_14068b1b0 as *const ()), // sub_14068b1b0
    Replacement::draft(0x14068b330, "unknown_14068b330", tpl_2731::<0x1401e6d50, 0x14068b960, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_14068b330
    Replacement::draft(0x14068b4b0, "unknown_14068b4b0", tpl_6488::<0x1401e6d50, 0x14068b9b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1403d0f40> as *const ()), // sub_14068b4b0
    Replacement::draft_throws(0x14068b630, "unknown_14068b630", tpl_3011::<0x1411e9f50, 0x14068cb00, 0x143cc287c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14068c910, 0x1401de410> as *const (), Throws::Discard), // sub_14068b630
    Replacement::draft_throws(0x14068b7f0, "unknown_14068b7f0", fn_14068b7f0 as *const (), Throws::Discard), // sub_14068b7f0
    Replacement::draft(0x14068b960, "unknown_14068b960", tpl_6490::<0x8, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14068b960
    Replacement::draft(0x14068b9b0, "unknown_14068b9b0", tpl_1955::<0x70, 0x1403d1040> as *const ()), // sub_14068b9b0
    Replacement::draft_throws(0x14068b9e0, "unknown_14068b9e0", tpl_65568::<0x88, 0x10088, 0x143cada18, 0x580e0, 0x14068bfd0, 0x141bb2680> as *const (), Throws::Discard), // sub_14068b9e0
    Replacement::draft_throws(0x14068bb20, "unknown_14068bb20", tpl_5499::<0x143c9a664, 0x5e308, 0x10, 0x8, 0x18, 0x141bb2070, 0x14068c410> as *const (), Throws::Discard), // sub_14068bb20
    Replacement::draft(0x14068bbc0, "unknown_14068bbc0", tpl_6144::<0x143ca7868, 0x5e308, 0x10, 0x8, 0x18, 0x141bb2060, 0x14068c530> as *const ()), // sub_14068bbc0
    Replacement::draft(0x14068bc60, "unknown_14068bc60", tpl_3188::<0x14068c980, 0x143239a80, 0x14068c530, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14068bc60
    Replacement::draft(0x14068bce0, "unknown_14068bce0", tpl_3188::<0x14068ca20, 0x143239ab8, 0x14068c410, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14068bce0
    Replacement::draft(0x14068c2a0, "unknown_14068c2a0", fn_14068c2a0 as *const ()), // sub_14068c2a0
    Replacement::draft(0x14068c410, "unknown_14068c410", fn_14068c410 as *const ()), // sub_14068c410
    Replacement::draft(0x14068c530, "unknown_14068c530", fn_14068c530 as *const ()), // sub_14068c530
    Replacement::draft_throws(0x14068c650, "unknown_14068c650", tpl_65573::<0x143cada18, 0x143ca7620, 0x142489ee0, 0x143c8fa04, 0x143c8d160, 0x141bb2680> as *const (), Throws::Discard), // sub_14068c650
    Replacement::draft_throws(0x14068c8b0, "unknown_14068c8b0", tpl_5128::<0x143cc2838, 0x20, 0x8, 0x141bb21f0> as *const (), Throws::Discard), // sub_14068c8b0
    Replacement::draft(0x14068c910, "unknown_14068c910", tpl_3028::<0x143cc287c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14068ac70> as *const ()), // sub_14068c910
    Replacement::draft(0x14068c980, "unknown_14068c980", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143239a80, 0x8, 0x20> as *const ()), // sub_14068c980
    Replacement::draft(0x14068ca20, "unknown_14068ca20", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143239ab8, 0x8, 0x20> as *const ()), // sub_14068ca20
    Replacement::draft(0x14068cac0, "unknown_14068cac0", tpl_6495::<0x8, 0x1403d1040> as *const ()), // sub_14068cac0
    Replacement::draft(0x14068d0f0, "bonsai_14068d0f0", tpl_6120::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x28, 0x30, 0x38, 0x143239cc8> as *const ()), // sub_14068d0f0
    Replacement::draft(0x14068d1b0, "bonsai_14068d1b0", tpl_6121::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143239cc8, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14068d1b0
    Replacement::draft_throws(0x14068d410, "bonsai_14068d410", fn_14068d410 as *const (), Throws::Caught), // sub_14068d410
    Replacement::draft_throws(0x14068d5c0, "bonsai_14068d5c0", tpl_5858::<0x14387c8b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_14068d5c0
    Replacement::draft(0x14068d6b0, "bonsai_14068d6b0", fn_14068d6b0 as *const ()), // sub_14068d6b0
    Replacement::draft(0x14068d7b0, "bonsai_14068d7b0", fn_14068d7b0 as *const ()), // sub_14068d7b0
    Replacement::draft_throws(0x14068da50, "bonsai_14068da50", fn_14068da50 as *const (), Throws::Caught), // sub_14068da50
    Replacement::draft_throws(0x14068dc00, "bonsai_14068dc00", tpl_5858::<0x14387c8c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_14068dc00
    Replacement::draft(0x14068dcf0, "bonsai_14068dcf0", fn_14068dcf0 as *const ()), // sub_14068dcf0
    Replacement::draft(0x14068ddc0, "bonsai_14068ddc0", fn_14068ddc0 as *const ()), // sub_14068ddc0
    Replacement::draft_throws(0x14068e020, "bonsai_14068e020", fn_14068e020 as *const (), Throws::Caught), // sub_14068e020
    Replacement::draft_throws(0x14068e1d0, "bonsai_14068e1d0", tpl_5858::<0x14387c8d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_14068e1d0
    Replacement::draft(0x14068e280, "bonsai_14068e280", tpl_6379::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14323a4a0, 0x8, 0x1c> as *const ()), // sub_14068e280
    Replacement::draft(0x14068e310, "bonsai_14068e310", fn_14068e310 as *const ()), // sub_14068e310
    Replacement::draft(0x14068e410, "bonsai_14068e410", tpl_6501::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x28, 0x30, 0x14323a798> as *const ()), // sub_14068e410
    Replacement::draft(0x14068e4d0, "bonsai_14068e4d0", tpl_6502::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323a798, 0x22, 0x26, 0xffffffffffffffe8, 0x40> as *const ()), // sub_14068e4d0
    Replacement::draft_throws(0x14068e730, "bonsai_14068e730", fn_14068e730 as *const (), Throws::Caught), // sub_14068e730
    Replacement::draft_throws(0x14068e8e0, "bonsai_14068e8e0", tpl_5858::<0x14387c8f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14068ee90> as *const (), Throws::Discard), // sub_14068e8e0
    Replacement::draft_throws(0x14068e970, "bonsai_14068e970", fn_14068e970 as *const (), Throws::Caught), // sub_14068e970
    Replacement::draft_throws(0x14068ee00, "bonsai_14068ee00", fn_14068ee00 as *const (), Throws::Caught), // sub_14068ee00
    Replacement::draft_throws(0x14068ee90, "bonsai_14068ee90", tpl_5859::<0x14387c9f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c948, 0x14387c7a8> as *const (), Throws::Discard), // sub_14068ee90
    Replacement::draft(0x14068efa0, "bonsai_14068efa0", fn_14068efa0 as *const ()), // sub_14068efa0
    Replacement::draft(0x14068f070, "bonsai_14068f070", fn_14068f070 as *const ()), // sub_14068f070
    Replacement::draft_throws(0x14068f2d0, "bonsai_14068f2d0", fn_14068f2d0 as *const (), Throws::Caught), // sub_14068f2d0
    Replacement::draft_throws(0x14068f480, "bonsai_14068f480", tpl_5858::<0x14387c900, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14068ee90> as *const (), Throws::Discard), // sub_14068f480
    Replacement::draft(0x14068f510, "bonsai_14068f510", fn_14068f510 as *const ()), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_0
    Replacement::draft(0x14068f580, "bonsai_14068f580", tpl_6473::<0x78, 0x10, 0x18, 0xffffffffffffffa0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x14323af00, 0x28> as *const ()), // sub_14068f580
    Replacement::draft(0x14068f650, "bonsai_14068f650", tpl_6474::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323af00, 0x22, 0x26, 0x49, 0x4d> as *const ()), // sub_14068f650
    Replacement::draft_throws(0x14068f8c0, "bonsai_14068f8c0", fn_14068f8c0 as *const (), Throws::Caught), // sub_14068f8c0
    Replacement::draft_throws(0x14068fa70, "bonsai_14068fa70", tpl_5858::<0x14387c908, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_14068fa70
    Replacement::draft(0x14068fb30, "bonsai_14068fb30", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x14323b3d8, 0x28, 0x30> as *const ()), // sub_14068fb30
    Replacement::draft(0x14068fbf0, "bonsai_14068fbf0", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323b3d8, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14068fbf0
    Replacement::draft_throws(0x14068fd30, "bonsai_14068fd30", fn_14068fd30 as *const (), Throws::Caught), // sub_14068fd30
    Replacement::draft_throws(0x14068fdc0, "bonsai_14068fdc0", tpl_5859::<0x14387c918, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c948, 0x14387c7a8> as *const (), Throws::Discard), // sub_14068fdc0
    Replacement::draft(0x14068fed0, "bonsai_14068fed0", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x14323b538, 0x28, 0x30> as *const ()), // sub_14068fed0
    Replacement::draft(0x14068ff90, "bonsai_14068ff90", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323b538, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14068ff90
    Replacement::draft_throws(0x1406900d0, "bonsai_1406900d0", fn_1406900d0 as *const (), Throws::Caught), // sub_1406900d0
    Replacement::draft_throws(0x140690160, "bonsai_140690160", tpl_5859::<0x14387c920, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c948, 0x14387c7a8> as *const (), Throws::Discard), // sub_140690160
    Replacement::draft(0x140690270, "bonsai_140690270", fn_140690270 as *const ()), // sub_140690270
    Replacement::draft(0x140690340, "bonsai_140690340", fn_140690340 as *const ()), // sub_140690340
    Replacement::draft_throws(0x1406905a0, "bonsai_1406905a0", fn_1406905a0 as *const (), Throws::Caught), // sub_1406905a0
    Replacement::draft_throws(0x140690750, "bonsai_140690750", tpl_5858::<0x14387c930, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14068ee90> as *const (), Throws::Discard), // sub_140690750
    Replacement::draft(0x140690810, "bonsai_140690810", tpl_6501::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x28, 0x30, 0x14323bb78> as *const ()), // sub_140690810
    Replacement::draft(0x1406908d0, "bonsai_1406908d0", tpl_6502::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323bb78, 0x22, 0x26, 0xffffffffffffffe8, 0x40> as *const ()), // sub_1406908d0
    Replacement::draft_throws(0x140690b30, "bonsai_140690b30", fn_140690b30 as *const (), Throws::Caught), // sub_140690b30
    Replacement::draft_throws(0x140690ce0, "bonsai_140690ce0", tpl_5858::<0x14387c940, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14068ee90> as *const (), Throws::Discard), // sub_140690ce0
    Replacement::draft(0x140690d70, "bonsai_140690d70", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x1431ea288, 0x28, 0x30> as *const ()), // sub_140690d70
    Replacement::draft(0x140690e30, "bonsai_140690e30", tpl_6309::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x1431ea288, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140690e30
    Replacement::draft(0x140690f30, "bonsai_140690f30", fn_140690f30 as *const ()), // sub_140690f30
    Replacement::draft(0x140691030, "bonsai_140691030", fn_140691030 as *const ()), // sub_140691030
    Replacement::draft_throws(0x1406912c0, "bonsai_1406912c0", fn_1406912c0 as *const (), Throws::Caught), // sub_1406912c0
    Replacement::draft_throws(0x140691470, "bonsai_140691470", tpl_5858::<0x14387c958, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140691470
    Replacement::draft(0x140691530, "bonsai_140691530", fn_140691530 as *const ()), // sub_140691530
    Replacement::draft(0x140691640, "bonsai_140691640", fn_140691640 as *const ()), // sub_140691640
    Replacement::draft_throws(0x140691970, "bonsai_140691970", fn_140691970 as *const (), Throws::Caught), // sub_140691970
    Replacement::draft_throws(0x140691b20, "bonsai_140691b20", tpl_5858::<0x14387c968, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140691b20
    Replacement::draft(0x140691c10, "bonsai_140691c10", fn_140691c10 as *const ()), // sub_140691c10
    Replacement::draft(0x140691d00, "bonsai_140691d00", fn_140691d00 as *const ()), // sub_140691d00
    Replacement::draft_throws(0x140691f90, "bonsai_140691f90", fn_140691f90 as *const (), Throws::Caught), // sub_140691f90
    Replacement::draft_throws(0x140692140, "bonsai_140692140", tpl_5858::<0x14387c970, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140692140
    Replacement::draft(0x140692200, "bonsai_140692200", tpl_3351::<0x30, 0x143ca78c8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140692282> as *const ()), // sub_140692200
    Replacement::draft(0x140692290, "bonsai_140692290", tpl_6513::<0xe8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140692800> as *const ()), // sub_140692290
    Replacement::draft(0x140692320, "bonsai_140692320", fn_140692320 as *const ()), // sub_140692320
    Replacement::draft_throws(0x140692560, "bonsai_140692560", fn_140692560 as *const (), Throws::Caught), // sub_140692560
    Replacement::draft_throws(0x140692710, "bonsai_140692710", tpl_5858::<0x14387c980, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140692710
    Replacement::draft(0x1406927a0, "bonsai_1406927a0", tpl_3942::<0x30, 0x143ca78c8, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1406927f0> as *const ()), // sub_1406927a0
    Replacement::draft(0x140692800, "bonsai_140692800", fn_140692800 as *const ()), // sub_140692800
    Replacement::draft(0x140692920, "bonsai_140692920", tpl_6501::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x28, 0x30, 0x14323cee0> as *const ()), // sub_140692920
    Replacement::draft(0x1406929e0, "bonsai_1406929e0", tpl_6502::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323cee0, 0x22, 0x26, 0xffffffffffffffe8, 0x40> as *const ()), // sub_1406929e0
    Replacement::draft_throws(0x140692c40, "bonsai_140692c40", fn_140692c40 as *const (), Throws::Caught), // sub_140692c40
    Replacement::draft_throws(0x140692df0, "bonsai_140692df0", tpl_5858::<0x14387c990, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14068ee90> as *const (), Throws::Discard), // sub_140692df0
    Replacement::draft(0x140692eb0, "bonsai_140692eb0", tpl_6120::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x28, 0x30, 0x38, 0x14323d040> as *const ()), // sub_140692eb0
    Replacement::draft(0x140692f70, "bonsai_140692f70", tpl_6121::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323d040, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140692f70
    Replacement::draft_throws(0x1406931d0, "bonsai_1406931d0", fn_1406931d0 as *const (), Throws::Caught), // sub_1406931d0
    Replacement::draft_throws(0x140693380, "bonsai_140693380", tpl_5858::<0x14387c998, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140693380
    Replacement::draft(0x140693440, "bonsai_140693440", fn_140693440 as *const ()), // sub_140693440
    Replacement::draft(0x140693520, "bonsai_140693520", fn_140693520 as *const ()), // sub_140693520
    Replacement::draft_throws(0x1406937a0, "bonsai_1406937a0", fn_1406937a0 as *const (), Throws::Caught), // sub_1406937a0
    Replacement::draft_throws(0x140693950, "bonsai_140693950", tpl_5858::<0x14387c9a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140693950
    Replacement::draft_throws(0x1406939e0, "bonsai_1406939e0", fn_1406939e0 as *const (), Throws::Discard), // sub_1406939e0
    Replacement::draft(0x140693b50, "bonsai_140693b50", fn_140693b50 as *const ()), // sub_140693b50
    Replacement::draft(0x140693c20, "bonsai_140693c20", fn_140693c20 as *const ()), // sub_140693c20
    Replacement::draft_throws(0x140693e90, "bonsai_140693e90", fn_140693e90 as *const (), Throws::Caught), // sub_140693e90
    Replacement::draft_throws(0x140694040, "bonsai_140694040", tpl_5858::<0x14387c9b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140694040
    Replacement::draft(0x140694100, "bonsai_140694100", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x14323d998, 0x28, 0x30> as *const ()), // sub_140694100
    Replacement::draft(0x1406941c0, "bonsai_1406941c0", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323d998, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_1406941c0
    Replacement::draft_throws(0x140694420, "bonsai_140694420", fn_140694420 as *const (), Throws::Caught), // sub_140694420
    Replacement::draft_throws(0x1406945d0, "bonsai_1406945d0", tpl_5858::<0x14387c9c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1406945d0
    Replacement::draft(0x140694690, "bonsai_140694690", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x14323de78, 0x28, 0x30> as *const ()), // sub_140694690
    Replacement::draft(0x140694750, "bonsai_140694750", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323de78, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140694750
    Replacement::draft_throws(0x1406949b0, "bonsai_1406949b0", fn_1406949b0 as *const (), Throws::Caught), // sub_1406949b0
    Replacement::draft_throws(0x140694b60, "bonsai_140694b60", tpl_5858::<0x14387c9d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140694b60
    Replacement::draft(0x140694bf0, "bonsai_140694bf0", fn_140694bf0 as *const ()), // sub_140694bf0
    Replacement::draft(0x140694c90, "bonsai_140694c90", fn_140694c90 as *const ()), // sub_140694c90
    Replacement::draft(0x140694d40, "bonsai_140694d40", tpl_6308::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x14323aa88, 0x28, 0x30> as *const ()), // sub_140694d40
    Replacement::draft(0x140694e00, "bonsai_140694e00", tpl_6477::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323aa88, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140694e00
    Replacement::draft(0x140694f00, "bonsai_140694f00", fn_140694f00 as *const ()), // sub_140694f00
    Replacement::draft(0x140694fd0, "bonsai_140694fd0", fn_140694fd0 as *const ()), // sub_140694fd0
    Replacement::draft_throws(0x140695240, "bonsai_140695240", fn_140695240 as *const (), Throws::Caught), // sub_140695240
    Replacement::draft_throws(0x1406953f0, "bonsai_1406953f0", tpl_5858::<0x14387ca00, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14068ee90> as *const (), Throws::Discard), // sub_1406953f0
    Replacement::draft_throws(0x1406955f0, "bonsai_1406955f0", fn_1406955f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree_attachment::process_attachment::system>
    Replacement::draft_throws(0x140695810, "std_140695810", tpl_6525::<0x14307e950, 0x142645870, 0x1401e5930, 0x140206290, 0x140695aa0, 0x14069593e> as *const (), Throws::Discard), // sub_140695810
    Replacement::draft_throws(0x140695aa0, "std_140695aa0", fn_140695aa0 as *const (), Throws::Discard), // sub_140695aa0
    Replacement::draft(0x140695c90, "unknown_140695c90", tpl_62805::<0x10, 0x38, 0x8, 0x140695d40, 0x1401e66f0, 0x20, 0x18, 0x1401e6780> as *const ()), // sub_140695c90
    Replacement::draft(0x140695d40, "std_140695d40", fn_140695d40 as *const ()), // sub_140695d40
    Replacement::draft(0x140695f90, "unknown_140695f90", tpl_1094::<0x143ca7888, 0x140492c90, 0x14020e8f0> as *const ()), // sub_140695f90
    Replacement::draft(0x140696190, "unknown_140696190", tpl_1436::<0x58, 0x60, 0x158, 0x143ca7888, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140696190
    Replacement::draft_throws(0x140696270, "bonsai_140696270", tpl_3211::<0x143cc2ad0, 0x143cc29a0, 0x1411e9f50, 0x140697370, 0x143cc2b24, 0x1420af120, 0x1420a28e0, 0x140696d50, 0x140696b70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14323ecf6, 0x142646960, 0x143cc29c0, 0x143cc29d0, 0x143cc29d2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree_attachment::stream_in::system>
    Replacement::draft_throws(0x140696560, "bonsai_140696560", tpl_3691::<0x143cc2ad8, 0x143cc29d8, 0x1411e9f50, 0x140697280, 0x143cc2944, 0x1420af120, 0x1420a28e0, 0x140696e60, 0x140696c30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14323f1c6, 0x142646960, 0x143cc29f8, 0x143cc2a08, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree_attachment::stream_out::system>
    Replacement::draft_throws(0x140696840, "unknown_140696840", fn_140696840 as *const (), Throws::Discard), // sub_140696840
    Replacement::draft(0x140696b70, "unknown_140696b70", tpl_1126::<0x142407c80, 0x580f0, 0x141bca200> as *const ()), // sub_140696b70
    Replacement::draft(0x140696c30, "unknown_140696c30", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140696190> as *const ()), // sub_140696c30
    Replacement::draft_throws(0x140696cd0, "unknown_140696cd0", fn_140696cd0 as *const (), Throws::Discard), // sub_140696cd0
    Replacement::draft(0x140696d50, "unknown_140696d50", tpl_1130::<0x142407c80, 0x141bca200> as *const ()), // sub_140696d50
    Replacement::draft(0x140696e60, "unknown_140696e60", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140696190> as *const ()), // sub_140696e60
    Replacement::draft(0x140696f40, "unknown_140696f40", fn_140696f40 as *const ()), // sub_140696f40
    Replacement::draft_throws(0x1406970c0, "unknown_1406970c0", tpl_6368::<0x140697500, 0x140206290, 0x140695aa0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401e5930, 0x1401e5730, 0x140697840, 0x1401e5380, 0x1401e66f0, 0x142442de0> as *const (), Throws::Discard), // sub_1406970c0
    Replacement::draft(0x140697280, "unknown_140697280", fn_140697280 as *const ()), // sub_140697280
    Replacement::draft(0x140697370, "unknown_140697370", tpl_5305::<0x143cc293c, 0x143cc2a10, 0x8, 0x10, 0x18, 0x141834a78, 0x143c8d694, 0x143c9a664, 0x143ca7888, 0x143cc2a30, 0x141834a0c> as *const ()), // sub_140697370
    Replacement::draft_throws(0x140697470, "unknown_140697470", tpl_4541::<0x143cc2b2c, 0x140696f40, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140697470
    Replacement::draft(0x140697500, "unknown_140697500", fn_140697500 as *const ()), // sub_140697500
    Replacement::draft_throws(0x140697720, "unknown_140697720", tpl_4541::<0x143cc2944, 0x140697280, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140697720
    Replacement::draft_throws(0x1406977b0, "unknown_1406977b0", tpl_4541::<0x143cc2b24, 0x140697370, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1406977b0
    Replacement::draft_throws(0x140697840, "std_140697840", tpl_6531::<0x1401e5930, 0x140206290, 0x140695aa0, 0x14307e950, 0x142645870, 0x140697ae4> as *const (), Throws::Discard), // sub_140697840
    Replacement::draft(0x140697af0, "kaim_140697af0", fn_140697af0 as *const ()), // Kaim::BaseNearestBorderHalfEdgeFromPosQuery::Kaim::BaseNearestBorderHalfEdgeFromPosQuery::vfunc_1
    Replacement::draft(0x140697b30, "bonsai_140697b30", tpl_6120::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x28, 0x30, 0x38, 0x14323f6a0> as *const ()), // sub_140697b30
    Replacement::draft(0x140697bf0, "bonsai_140697bf0", tpl_6121::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14323f6a0, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140697bf0
    Replacement::draft_throws(0x140697e50, "bonsai_140697e50", fn_140697e50 as *const (), Throws::Caught), // sub_140697e50
    Replacement::draft_throws(0x140698000, "bonsai_140698000", tpl_5858::<0x14387ca10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_140698000
    Replacement::draft(0x1406980c0, "bonsai_1406980c0", fn_1406980c0 as *const ()), // sub_1406980c0
    Replacement::draft(0x1406981a0, "bonsai_1406981a0", fn_1406981a0 as *const ()), // sub_1406981a0
    Replacement::draft(0x140698200, "bonsai_140698200", fn_140698200 as *const ()), // sub_140698200
    Replacement::draft_throws(0x1406982b0, "bonsai_1406982b0", fn_1406982b0 as *const (), Throws::Discard), // sub_1406982b0
    Replacement::draft(0x140698380, "bonsai_140698380", fn_140698380 as *const ()), // sub_140698380
    Replacement::draft(0x1406984c0, "bonsai_1406984c0", tpl_6537::<0x140698600, 0x14183acc8, 0x1401dec00, 0x1401de830> as *const ()), // sub_1406984c0
    Replacement::draft(0x140698600, "bonsai_140698600", fn_140698600 as *const ()), // sub_140698600
    Replacement::draft(0x1406986c0, "bonsai_1406986c0", fn_1406986c0 as *const ()), // sub_1406986c0
    Replacement::draft(0x1406989d0, "bonsai_1406989d0", fn_1406989d0 as *const ()), // sub_1406989d0
    Replacement::draft(0x140698ae0, "bonsai_140698ae0", fn_140698ae0 as *const ()), // sub_140698ae0
    Replacement::draft(0x140698ea0, "bonsai_140698ea0", fn_140698ea0 as *const ()), // sub_140698ea0
    Replacement::draft(0x140699000, "bonsai_140699000", fn_140699000 as *const ()), // sub_140699000
    Replacement::draft(0x140699100, "bonsai_140699100", fn_140699100 as *const ()), // sub_140699100
    Replacement::draft(0x1406991f0, "bonsai_1406991f0", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x14323fba0, 0x1f, 0x8, 0x1c> as *const ()), // sub_1406991f0
    Replacement::draft(0x140699280, "bonsai_140699280", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14323fba0, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_140699280
    Replacement::draft_throws(0x1406994d0, "bonsai_1406994d0", fn_1406994d0 as *const (), Throws::Caught), // sub_1406994d0
    Replacement::draft_throws(0x140699660, "bonsai_140699660", tpl_1905::<0x14387ca18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_140699660
    Replacement::draft(0x140699740, "bonsai_140699740", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x14323fe70, 0x15, 0x17, 0x8> as *const ()), // sub_140699740
    Replacement::draft(0x1406997e0, "bonsai_1406997e0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x14323fe70, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_1406997e0
    Replacement::draft(0x140699890, "bonsai_140699890", tpl_6383::<0x14387ca28, 0x14243b0b0, 0x14387c9e8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140699890
    Replacement::draft_throws(0x140699930, "bonsai_140699930", tpl_6384::<0x14387ca24, 0x14387ca28, 0x1401e9fa0, 0x14387c9e4, 0x14387c9e8, 0x1401e9e70> as *const (), Throws::Discard), // sub_140699930
    Replacement::draft_throws(0x1406999b0, "bonsai_1406999b0", tpl_6384::<0x14387ca24, 0x14387ca28, 0x1401e9b60, 0x14387c9e4, 0x14387c9e8, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406999b0
    Replacement::draft_throws(0x140699a30, "bonsai_140699a30", fn_140699a30 as *const (), Throws::Caught), // sub_140699a30
    Replacement::draft(0x140699ac0, "bonsai_140699ac0", tpl_6386::<0x14387ca24, 0x14387ca28, 0x1401e9970, 0x14387c9e4, 0x14387c9e8, 0x1401e9860> as *const ()), // sub_140699ac0
    Replacement::draft_throws(0x140699b40, "bonsai_140699b40", tpl_6384::<0x14387ca24, 0x14387ca28, 0x1401e9770, 0x14387c9e4, 0x14387c9e8, 0x1401e9590> as *const (), Throws::Discard), // sub_140699b40
    Replacement::draft_throws(0x140699bc0, "bonsai_140699bc0", tpl_6387::<0x14387ca28, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140699bc0
    Replacement::draft(0x140699ca0, "bonsai_140699ca0", fn_140699ca0 as *const ()), // sub_140699ca0
    Replacement::draft(0x140699d40, "bonsai_140699d40", fn_140699d40 as *const ()), // sub_140699d40
    Replacement::draft_throws(0x140699fa0, "bonsai_140699fa0", fn_140699fa0 as *const (), Throws::Caught), // sub_140699fa0
    Replacement::draft_throws(0x14069a130, "bonsai_14069a130", tpl_1905::<0x14387ca30, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14069a130
    Replacement::draft(0x14069a240, "bonsai_14069a240", tpl_1902::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143240270, 0x20, 0x28, 0x30> as *const ()), // sub_14069a240
    Replacement::draft(0x14069a2e0, "bonsai_14069a2e0", tpl_1903::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143240270, 0xffffffffffffffd8, 0xffffffffffffffe0, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14069a2e0
    Replacement::draft_throws(0x14069a530, "bonsai_14069a530", fn_14069a530 as *const (), Throws::Caught), // sub_14069a530
    Replacement::draft_throws(0x14069a6c0, "bonsai_14069a6c0", tpl_1905::<0x14387ca40, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14069a6c0
    Replacement::draft(0x14069a7c0, "bonsai_14069a7c0", tpl_6378::<0x1b8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14065d070, 0x143240878> as *const ()), // sub_14069a7c0
    Replacement::draft(0x14069a860, "bonsai_14069a860", tpl_65472::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143240878, 0x14183acc2, 0x14065d070> as *const ()), // sub_14069a860
    Replacement::draft_throws(0x14069a990, "bonsai_14069a990", fn_14069a990 as *const (), Throws::Caught), // sub_14069a990
    Replacement::draft_throws(0x14069aa20, "bonsai_14069aa20", tpl_5859::<0x14387ca50, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c420, 0x14387c9e8> as *const (), Throws::Discard), // sub_14069aa20
    Replacement::draft(0x14069ab00, "bonsai_14069ab00", tpl_1094::<0x143ca78cc, 0x14069b3b0, 0x14020e8f0> as *const ()), // sub_14069ab00
    Replacement::draft(0x14069ad00, "bonsai_14069ad00", tpl_1436::<0x58, 0x60, 0x158, 0x143ca78cc, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14069ad00
    Replacement::draft_throws(0x14069ade0, "bonsai_14069ade0", tpl_1886::<0x143cc2b88, 0x143cc2b38, 0x1411e9f50, 0x14069b9a0, 0x143cc2b8c, 0x1420af120, 0x1420a28e0, 0x14069b6c0, 0x14069b560, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432409c6, 0x142646960, 0x143cc2b58, 0x143cc2b5c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::ai_tickets::stream_in::system>
    Replacement::draft_throws(0x14069b0c0, "bonsai_14069b0c0", tpl_4129::<0x143cc2b94, 0x143cc2b60, 0x1411e9f50, 0x14069b8b0, 0x143cc2b34, 0x1420af120, 0x1420a28e0, 0x14069b7d0, 0x14069b620, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143240bb6, 0x142646960, 0x143cc2b80, 0x143cc2b84, 0x143cc2b86, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::ai_tickets::stream_out::system>
    Replacement::draft(0x14069b3b0, "bonsai_14069b3b0", tpl_6546::<0x1401e6d50, 0x14069b530, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1404bdd40> as *const ()), // sub_14069b3b0
    Replacement::draft(0x14069b530, "bonsai_14069b530", tpl_1955::<0x40, 0x1404bdab0> as *const ()), // sub_14069b530
    Replacement::draft(0x14069b560, "bonsai_14069b560", tpl_1126::<0x142407c80, 0x580f0, 0x141bd17a0> as *const ()), // sub_14069b560
    Replacement::draft(0x14069b620, "bonsai_14069b620", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14069ad00> as *const ()), // sub_14069b620
    Replacement::draft(0x14069b6c0, "bonsai_14069b6c0", tpl_1130::<0x142407c80, 0x141bd17a0> as *const ()), // sub_14069b6c0
    Replacement::draft(0x14069b7d0, "bonsai_14069b7d0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14069ad00> as *const ()), // sub_14069b7d0
    Replacement::draft_throws(0x14069bd30, "bonsai_14069bd30", fn_14069bd30 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::aiteleport::update::system>
    Replacement::draft_throws(0x14069c050, "bonsai_14069c050", tpl_6548::<0x143cc2d3c, 0x143cc2d08, 0x1411e9f50, 0x14069d020, 0x143cc2ccc, 0x1420af120, 0x140215750, 0x1420a28e0, 0x14069cfc0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143241cc6, 0x142646960, 0x143cc2d28, 0x143cc2d38, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::aiteleport::update_has_teleported::system>
    Replacement::draft(0x14069c360, "bonsai_14069c360", tpl_1435::<0x143ca7870, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14069c360
    Replacement::draft(0x14069c6b0, "bonsai_14069c6b0", tpl_6549::<0x143426abc, 0x8, 0x14069c360> as *const ()), // sub_14069c6b0
    Replacement::draft_throws(0x14069c710, "bonsai_14069c710", fn_14069c710 as *const (), Throws::Discard), // sub_14069c710
    Replacement::draft_throws(0x14069c850, "bonsai_14069c850", fn_14069c850 as *const (), Throws::Discard), // sub_14069c850
    Replacement::draft_throws(0x14069ccb0, "bonsai_14069ccb0", fn_14069ccb0 as *const (), Throws::Discard), // sub_14069ccb0
    Replacement::draft_throws(0x14069cfc0, "bonsai_14069cfc0", tpl_5214::<0x143cc2ccc, 0x20, 0x8, 0x141bd2150> as *const (), Throws::Discard), // sub_14069cfc0
    Replacement::draft(0x14069d160, "bonsai_14069d160", fn_14069d160 as *const ()), // sub_14069d160
    Replacement::draft(0x14069d490, "bonsai_14069d490", fn_14069d490 as *const ()), // sub_14069d490
    Replacement::draft(0x14069d5c0, "bonsai_14069d5c0", fn_14069d5c0 as *const ()), // bonsai::AirMoveProgram::bonsai::AirMoveProgram::vfunc_0
    Replacement::draft(0x14069d640, "bonsai_14069d640", fn_14069d640 as *const ()), // sub_14069d640
    Replacement::draft(0x14069d6d0, "bonsai_14069d6d0", fn_14069d6d0 as *const ()), // sub_14069d6d0
    Replacement::draft_throws(0x14069d920, "bonsai_14069d920", fn_14069d920 as *const (), Throws::Caught), // sub_14069d920
    Replacement::draft_throws(0x14069dab0, "bonsai_14069dab0", tpl_1905::<0x14387ca58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14069dab0
    Replacement::draft(0x14069db70, "bonsai_14069db70", fn_14069db70 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_0
    Replacement::draft(0x14069dbc0, "unknown_14069dbc0", tpl_6558::<0x14069e740> as *const ()), // sub_14069dbc0
    Replacement::draft(0x14069dbe0, "unknown_14069dbe0", tpl_6559::<0x14069e740> as *const ()), // sub_14069dbe0
    Replacement::draft(0x14069dc00, "unknown_14069dc00", tpl_5915::<0x8, 0x20, 0x14069e740, 0x1401e66f0> as *const ()), // sub_14069dc00
    Replacement::draft(0x14069dc70, "unknown_14069dc70", tpl_1526::<0x1427cc610, 0x1410d6ca0, 0x14069e820, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14069dc70
    Replacement::draft_throws(0x14069df10, "std_14069df10", fn_14069df10 as *const (), Throws::Discard), // sub_14069df10
    Replacement::draft(0x14069e1a0, "std_14069e1a0", tpl_6561::<0x8, 0x14069e740, 0x1401fb4d0> as *const ()), // sub_14069e1a0
    Replacement::draft_throws(0x14069e1d0, "std_14069e1d0", fn_14069e1d0 as *const (), Throws::Discard), // sub_14069e1d0
    Replacement::draft(0x14069e3a0, "std_14069e3a0", fn_14069e3a0 as *const ()), // sub_14069e3a0
    Replacement::draft(0x14069e470, "std_14069e470", fn_14069e470 as *const ()), // sub_14069e470
    Replacement::draft(0x14069e670, "bonsai_14069e670", fn_14069e670 as *const ()), // sub_14069e670
    Replacement::draft(0x14069e740, "bonsai_14069e740", fn_14069e740 as *const ()), // sub_14069e740
    Replacement::draft(0x14069e820, "bonsai_14069e820", tpl_6567::<0x143cc2e04, 0x143cc2e48, 0x1411e9f50, 0x143ca77b8, 0x140215610, 0x143c8fd28, 0x1420a28e0, 0x14069eb50, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143242396, 0x142646960, 0x143cc2e68, 0x143cc2e70, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bonsai::ai_cover_manager::update::system>
    Replacement::draft(0x14069eaf0, "navigation_14069eaf0", fn_14069eaf0 as *const ()), // navigation::IntegratedCoverPoint::navigation::IntegratedCoverPoint::vfunc_0
    Replacement::draft_throws(0x14069eb80, "unknown_14069eb80", fn_14069eb80 as *const (), Throws::Discard), // sub_14069eb80
    Replacement::draft(0x14069ed10, "unknown_14069ed10", fn_14069ed10 as *const ()), // sub_14069ed10
    Replacement::draft_throws(0x14069ee60, "unknown_14069ee60", fn_14069ee60 as *const (), Throws::Discard), // sub_14069ee60
    Replacement::draft(0x14069efb0, "unknown_14069efb0", tpl_6572::<0x8, 0x18, 0x10, 0x20, 0x14069e740, 0x1401e66f0> as *const ()), // sub_14069efb0
    Replacement::draft(0x14069f010, "unknown_14069f010", fn_14069f010 as *const ()), // sub_14069f010
    Replacement::draft(0x14069f1d0, "unknown_14069f1d0", fn_14069f1d0 as *const ()), // sub_14069f1d0
    Replacement::draft_throws(0x14069f2c0, "std_14069f2c0", fn_14069f2c0 as *const (), Throws::Discard), // sub_14069f2c0
    Replacement::draft(0x14069f560, "unknown_14069f560", fn_14069f560 as *const ()), // sub_14069f560
    Replacement::draft(0x14069f830, "bonsai_14069f830", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143242618, 0x15, 0x17, 0x8> as *const ()), // sub_14069f830
    Replacement::draft(0x14069f8d0, "bonsai_14069f8d0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143242618, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14069f8d0
    Replacement::draft(0x14069f980, "unknown_14069f980", tpl_6135::<0x14387ca68, 0x14243b0b0, 0x14387c9e8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14069f980
    Replacement::draft_throws(0x14069fa20, "unknown_14069fa20", tpl_6136::<0x14387ca64, 0x14387ca68, 0x1401e9fa0, 0x14387c9e4, 0x14387c9e8, 0x1401e9e70> as *const (), Throws::Discard), // sub_14069fa20
    Replacement::draft_throws(0x14069faa0, "unknown_14069faa0", tpl_6136::<0x14387ca64, 0x14387ca68, 0x1401e9b60, 0x14387c9e4, 0x14387c9e8, 0x1401e9a20> as *const (), Throws::Discard), // sub_14069faa0
    Replacement::draft_throws(0x14069fb20, "unknown_14069fb20", fn_14069fb20 as *const (), Throws::Caught), // sub_14069fb20
    Replacement::draft(0x14069fbb0, "unknown_14069fbb0", tpl_6137::<0x14387ca64, 0x14387ca68, 0x1401e9970, 0x14387c9e4, 0x14387c9e8, 0x1401e9860> as *const ()), // sub_14069fbb0
    Replacement::draft_throws(0x14069fc30, "unknown_14069fc30", tpl_6136::<0x14387ca64, 0x14387ca68, 0x1401e9770, 0x14387c9e4, 0x14387c9e8, 0x1401e9590> as *const (), Throws::Discard), // sub_14069fc30
    Replacement::draft_throws(0x14069fcb0, "unknown_14069fcb0", tpl_1905::<0x14387ca68, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14069fcb0
    Replacement::draft_throws(0x14069fd60, "std_14069fd60", fn_14069fd60 as *const (), Throws::Discard), // sub_14069fd60
    Replacement::draft(0x1406a0030, "unknown_1406a0030", tpl_1094::<0x143ca7874, 0x1406a0940, 0x14020e8f0> as *const ()), // sub_1406a0030
    Replacement::draft(0x1406a0230, "unknown_1406a0230", tpl_1436::<0x58, 0x60, 0x158, 0x143ca7874, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1406a0230
    Replacement::draft_throws(0x1406a0310, "bonsai_1406a0310", tpl_5818::<0x143cc2f78, 0x143cc2f48, 0x1411e9f50, 0x1406a0f60, 0x143cc2fa0, 0x1420af120, 0x143cbb750, 0x140215610, 0x143cada18, 0x1420a28e0, 0x1406a0c60, 0x1406a0ac0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143242b06, 0x142646960, 0x143cc2f68, 0x143cc2f6c, 0x143cc2f6e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::abilityinfo::stream_in::system>
    Replacement::draft_throws(0x1406a0660, "bonsai_1406a0660", tpl_3682::<0x143cc2f7c, 0x143cc2ea0, 0x1411e9f50, 0x1406a0e70, 0x143cc2f9c, 0x1420af120, 0x1420a28e0, 0x1406a0d90, 0x1406a0bc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143242d36, 0x142646960, 0x143cc2ec0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::abilityinfo::stream_out::system>
    Replacement::draft(0x1406a0940, "unknown_1406a0940", tpl_6546::<0x1401e6d50, 0x14069b530, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1404bdb10> as *const ()), // sub_1406a0940
    Replacement::draft_throws(0x1406a0ac0, "unknown_1406a0ac0", fn_1406a0ac0 as *const (), Throws::Caught), // sub_1406a0ac0
    Replacement::draft(0x1406a0bc0, "unknown_1406a0bc0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1406a0230> as *const ()), // sub_1406a0bc0
    Replacement::draft_throws(0x1406a0c60, "unknown_1406a0c60", fn_1406a0c60 as *const (), Throws::Caught), // sub_1406a0c60
    Replacement::draft(0x1406a0d90, "unknown_1406a0d90", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1406a0230> as *const ()), // sub_1406a0d90
    Replacement::draft(0x1406a11a0, "unknown_1406a11a0", fn_1406a11a0 as *const ()), // sub_1406a11a0
    Replacement::draft(0x1406a1220, "unknown_1406a1220", tpl_6580::<0x14340ed48, 0x8, 0x143243328, 0x143242fe0, 0x142498f00> as *const ()), // sub_1406a1220
    Replacement::draft(0x1406a1260, "unknown_1406a1260", fn_1406a1260 as *const ()), // sub_1406a1260
    Replacement::draft(0x1406a12f0, "unknown_1406a12f0", fn_1406a12f0 as *const ()), // sub_1406a12f0
    Replacement::draft(0x1406a13e0, "unknown_1406a13e0", fn_1406a13e0 as *const ()), // sub_1406a13e0
    Replacement::draft_throws(0x1406a14f0, "unknown_1406a14f0", fn_1406a14f0 as *const (), Throws::Discard), // sub_1406a14f0
    Replacement::draft_throws(0x1406a1660, "unknown_1406a1660", fn_1406a1660 as *const (), Throws::Discard), // sub_1406a1660
    Replacement::draft_throws(0x1406a17d0, "unknown_1406a17d0", fn_1406a17d0 as *const (), Throws::Discard), // sub_1406a17d0
    Replacement::draft_throws(0x1406a1980, "unknown_1406a1980", fn_1406a1980 as *const (), Throws::Discard), // sub_1406a1980
    Replacement::draft_throws(0x1406a1b20, "unknown_1406a1b20", fn_1406a1b20 as *const (), Throws::Caught), // sub_1406a1b20
    Replacement::draft_throws(0x1406a1c80, "unknown_1406a1c80", fn_1406a1c80 as *const (), Throws::Caught), // sub_1406a1c80
    Replacement::draft_throws(0x1406a1de0, "unknown_1406a1de0", fn_1406a1de0 as *const (), Throws::Caught), // sub_1406a1de0
    Replacement::draft(0x1406a1f40, "unknown_1406a1f40", fn_1406a1f40 as *const ()), // sub_1406a1f40
    Replacement::draft(0x1406a20b0, "unknown_1406a20b0", fn_1406a20b0 as *const ()), // sub_1406a20b0
    Replacement::draft(0x1406a2250, "unknown_1406a2250", fn_1406a2250 as *const ()), // sub_1406a2250
    Replacement::draft_throws(0x1406a2410, "unknown_1406a2410", fn_1406a2410 as *const (), Throws::Discard), // sub_1406a2410
    Replacement::draft_throws(0x1406a25c0, "unknown_1406a25c0", fn_1406a25c0 as *const (), Throws::Discard), // sub_1406a25c0
    Replacement::draft(0x1406a2700, "unknown_1406a2700", fn_1406a2700 as *const ()), // sub_1406a2700
    Replacement::draft(0x1406a27a0, "unknown_1406a27a0", fn_1406a27a0 as *const ()), // sub_1406a27a0
    Replacement::draft(0x1406a2830, "unknown_1406a2830", fn_1406a2830 as *const ()), // sub_1406a2830
    Replacement::draft_throws(0x1406a28d0, "unknown_1406a28d0", tpl_65670::<0x142442de0, 0x143a381b8, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1406a28d0
    Replacement::draft(0x1406a29b0, "unknown_1406a29b0", fn_1406a29b0 as *const ()), // sub_1406a29b0
    Replacement::draft_throws(0x1406a2a90, "unknown_1406a2a90", fn_1406a2a90 as *const (), Throws::Caught), // sub_1406a2a90
    Replacement::draft_throws(0x1406a2d90, "unknown_1406a2d90", tpl_65672::<0x142442de0, 0x143a381b8, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1406a2d90
    Replacement::draft_throws(0x1406a2fb0, "unknown_1406a2fb0", fn_1406a2fb0 as *const (), Throws::Caught), // sub_1406a2fb0
    Replacement::draft_throws(0x1406a32a0, "unknown_1406a32a0", fn_1406a32a0 as *const (), Throws::Caught), // sub_1406a32a0
    Replacement::draft_throws(0x1406a3d70, "unknown_1406a3d70", fn_1406a3d70 as *const (), Throws::Caught), // sub_1406a3d70
    Replacement::draft_throws(0x1406a4810, "unknown_1406a4810", fn_1406a4810 as *const (), Throws::Caught), // sub_1406a4810
    Replacement::draft_throws(0x1406a52a0, "unknown_1406a52a0", fn_1406a52a0 as *const (), Throws::Caught), // sub_1406a52a0
    Replacement::draft(0x1406a54f0, "unknown_1406a54f0", fn_1406a54f0 as *const ()), // sub_1406a54f0
    Replacement::draft_throws(0x1406a5540, "unknown_1406a5540", fn_1406a5540 as *const (), Throws::Caught), // sub_1406a5540
    Replacement::draft(0x1406a55b0, "unknown_1406a55b0", tpl_65675::<0x8, 0xc, 0x1401deba0, 0x14183acc2> as *const ()), // sub_1406a55b0
    Replacement::draft_throws(0x1406a5660, "unknown_1406a5660", fn_1406a5660 as *const (), Throws::Caught), // sub_1406a5660
    Replacement::draft(0x1406a56d0, "unknown_1406a56d0", fn_1406a56d0 as *const ()), // sub_1406a56d0
    Replacement::draft(0x1406a5790, "unknown_1406a5790", fn_1406a5790 as *const ()), // sub_1406a5790
    Replacement::draft(0x1406a5840, "unknown_1406a5840", fn_1406a5840 as *const ()), // sub_1406a5840
    Replacement::draft_throws(0x1406a59f0, "unknown_1406a59f0", fn_1406a59f0 as *const (), Throws::Caught), // sub_1406a59f0
    Replacement::draft_throws(0x1406a5b30, "unknown_1406a5b30", fn_1406a5b30 as *const (), Throws::Caught), // sub_1406a5b30
    Replacement::draft_throws(0x1406a5eb0, "unknown_1406a5eb0", fn_1406a5eb0 as *const (), Throws::Caught), // sub_1406a5eb0
    Replacement::draft_throws(0x1406a5ff0, "unknown_1406a5ff0", fn_1406a5ff0 as *const (), Throws::Caught), // sub_1406a5ff0
    Replacement::draft_throws(0x1406a6340, "unknown_1406a6340", fn_1406a6340 as *const (), Throws::Caught), // sub_1406a6340
    Replacement::draft_throws(0x1406a6480, "unknown_1406a6480", fn_1406a6480 as *const (), Throws::Caught), // sub_1406a6480
    Replacement::draft(0x1406a67c0, "unknown_1406a67c0", fn_1406a67c0 as *const ()), // sub_1406a67c0
    Replacement::draft(0x1406a6910, "unknown_1406a6910", fn_1406a6910 as *const ()), // sub_1406a6910
    Replacement::draft(0x1406a69a0, "unknown_1406a69a0", fn_1406a69a0 as *const ()), // sub_1406a69a0
    Replacement::draft_throws(0x1406a6a10, "unknown_1406a6a10", fn_1406a6a10 as *const (), Throws::Caught), // sub_1406a6a10
    Replacement::draft(0x1406a6ae0, "unknown_1406a6ae0", fn_1406a6ae0 as *const ()), // sub_1406a6ae0
    Replacement::draft(0x1406a6cb0, "unknown_1406a6cb0", fn_1406a6cb0 as *const ()), // sub_1406a6cb0
    Replacement::draft_throws(0x1406a6d80, "unknown_1406a6d80", fn_1406a6d80 as *const (), Throws::Caught), // sub_1406a6d80
    Replacement::draft_throws(0x1406a7330, "unknown_1406a7330", fn_1406a7330 as *const (), Throws::Caught), // sub_1406a7330
    Replacement::draft_throws(0x1406a78a0, "unknown_1406a78a0", fn_1406a78a0 as *const (), Throws::Caught), // sub_1406a78a0
    Replacement::draft_throws(0x1406a7e20, "unknown_1406a7e20", fn_1406a7e20 as *const (), Throws::Caught), // sub_1406a7e20
    Replacement::draft_throws(0x1406a8300, "unknown_1406a8300", fn_1406a8300 as *const (), Throws::Caught), // sub_1406a8300
    Replacement::draft_throws(0x1406a8750, "unknown_1406a8750", fn_1406a8750 as *const (), Throws::Caught), // sub_1406a8750
    Replacement::draft(0x1406a8c20, "unknown_1406a8c20", fn_1406a8c20 as *const ()), // sub_1406a8c20
    Replacement::draft(0x1406a8d40, "unknown_1406a8d40", fn_1406a8d40 as *const ()), // sub_1406a8d40
    Replacement::draft_throws(0x1406a8e90, "unknown_1406a8e90", fn_1406a8e90 as *const (), Throws::Discard), // sub_1406a8e90
    Replacement::draft_throws(0x1406a9040, "unknown_1406a9040", fn_1406a9040 as *const (), Throws::Discard), // sub_1406a9040
    Replacement::draft_throws(0x1406a91d0, "unknown_1406a91d0", fn_1406a91d0 as *const (), Throws::Discard), // sub_1406a91d0
    Replacement::draft_throws(0x1406a93b0, "unknown_1406a93b0", fn_1406a93b0 as *const (), Throws::Discard), // sub_1406a93b0
    Replacement::draft_throws(0x1406a9550, "unknown_1406a9550", fn_1406a9550 as *const (), Throws::Caught), // sub_1406a9550
    Replacement::draft_throws(0x1406a96b0, "unknown_1406a96b0", fn_1406a96b0 as *const (), Throws::Caught), // sub_1406a96b0
    Replacement::draft_throws(0x1406a9810, "unknown_1406a9810", fn_1406a9810 as *const (), Throws::Caught), // sub_1406a9810
    Replacement::draft(0x1406a9970, "unknown_1406a9970", fn_1406a9970 as *const ()), // sub_1406a9970
    Replacement::draft(0x1406a9b10, "unknown_1406a9b10", fn_1406a9b10 as *const ()), // sub_1406a9b10
    Replacement::draft(0x1406a9cf0, "unknown_1406a9cf0", fn_1406a9cf0 as *const ()), // sub_1406a9cf0
    Replacement::draft_throws(0x1406a9ee0, "unknown_1406a9ee0", fn_1406a9ee0 as *const (), Throws::Discard), // sub_1406a9ee0
    Replacement::draft_throws(0x1406aa0c0, "unknown_1406aa0c0", fn_1406aa0c0 as *const (), Throws::Discard), // sub_1406aa0c0
    Replacement::draft(0x1406aa200, "unknown_1406aa200", fn_1406aa200 as *const ()), // sub_1406aa200
    Replacement::draft(0x1406aa2a0, "unknown_1406aa2a0", fn_1406aa2a0 as *const ()), // sub_1406aa2a0
    Replacement::draft(0x1406aa330, "unknown_1406aa330", fn_1406aa330 as *const ()), // sub_1406aa330
    Replacement::draft_throws(0x1406aa3d0, "unknown_1406aa3d0", tpl_65670::<0x142442de0, 0x143a38748, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1406aa3d0
    Replacement::draft_throws(0x1406aa4b0, "unknown_1406aa4b0", fn_1406aa4b0 as *const (), Throws::Caught), // sub_1406aa4b0
    Replacement::draft_throws(0x1406aa7b0, "unknown_1406aa7b0", tpl_65672::<0x142442de0, 0x143a38748, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1406aa7b0
    Replacement::draft_throws(0x1406aa9d0, "unknown_1406aa9d0", fn_1406aa9d0 as *const (), Throws::Caught), // sub_1406aa9d0
    Replacement::draft_throws(0x1406aacc0, "unknown_1406aacc0", fn_1406aacc0 as *const (), Throws::Caught), // sub_1406aacc0
    Replacement::draft_throws(0x1406ab780, "unknown_1406ab780", fn_1406ab780 as *const (), Throws::Caught), // sub_1406ab780
    Replacement::draft_throws(0x1406ac210, "unknown_1406ac210", fn_1406ac210 as *const (), Throws::Caught), // sub_1406ac210
    Replacement::draft_throws(0x1406acc90, "unknown_1406acc90", fn_1406acc90 as *const (), Throws::Caught), // sub_1406acc90
    Replacement::draft_throws(0x1406ace90, "unknown_1406ace90", fn_1406ace90 as *const (), Throws::Caught), // sub_1406ace90
    Replacement::draft_throws(0x1406acfd0, "unknown_1406acfd0", fn_1406acfd0 as *const (), Throws::Caught), // sub_1406acfd0
    Replacement::draft(0x1406ad650, "content_1406ad650", tpl_6638::<0x3, 0x4, 0x8, 0x143243d70, 0x142409b50> as *const ()), // content::BehaviorTreeDecorator::content::BehaviorTreeDecorator::vfunc_1
    Replacement::draft(0x1406ad6e0, "content_1406ad6e0", tpl_6638::<0x3, 0x4, 0x8, 0x143216f98, 0x142409b50> as *const ()), // content::OverrideSpeedDecorator::content::OverrideSpeedDecorator::vfunc_1
    Replacement::draft_throws(0x1406ad740, "content_1406ad740", tpl_6639::<0x143cc2fa4, 0x143cc2fa8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::OverrideSpeedDecorator::content::OverrideSpeedDecorator::vfunc_2
    Replacement::draft(0x1406ad960, "contentbonsai_1406ad960", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14315c2c8, 0x8> as *const ()), // sub_1406ad960
    Replacement::draft(0x1406ad9e0, "contentbonsai_1406ad9e0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14315c2c8> as *const ()), // sub_1406ad9e0
    Replacement::draft(0x1406ada70, "contentbonsai_1406ada70", fn_1406ada70 as *const ()), // sub_1406ada70
    Replacement::draft(0x1406adb10, "contentbonsai_1406adb10", tpl_6641::<0x14387cbe0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406f29a0, 0x318, 0x328, 0x320, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1406adb10
    Replacement::draft_throws(0x1406adbc0, "contentbonsai_1406adbc0", tpl_6642::<0x14387cb90, 0x14387cbe0, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1406d7200, 0x318, 0x328, 0x320, 0x1, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406adbc0
    Replacement::draft(0x1406adc60, "contentbonsai_1406adc60", tpl_6641::<0x14387cbe0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406f2ea0, 0x318, 0x328, 0x320, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1406adc60
    Replacement::draft_throws(0x1406add10, "contentbonsai_1406add10", tpl_6642::<0x14387cb90, 0x14387cbe0, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1406d7330, 0x318, 0x328, 0x320, 0x1, 0x38, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406add10
    Replacement::draft_throws(0x1406addb0, "contentbonsai_1406addb0", fn_1406addb0 as *const (), Throws::Caught), // sub_1406addb0
    Replacement::draft_throws(0x1406ade80, "contentbonsai_1406ade80", fn_1406ade80 as *const (), Throws::Caught), // sub_1406ade80
    Replacement::draft_throws(0x1406adf50, "contentbonsai_1406adf50", fn_1406adf50 as *const (), Throws::Caught), // sub_1406adf50
    Replacement::draft(0x1406ae020, "contentbonsai_1406ae020", tpl_6641::<0x14387cbe0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406f2d10, 0x318, 0x328, 0x320, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1406ae020
    Replacement::draft(0x1406ae0d0, "contentbonsai_1406ae0d0", fn_1406ae0d0 as *const ()), // sub_1406ae0d0
    Replacement::draft(0x1406ae170, "contentbonsai_1406ae170", tpl_6641::<0x14387cbe0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406f3300, 0x318, 0x328, 0x320, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1406ae170
    Replacement::draft_throws(0x1406ae220, "contentbonsai_1406ae220", fn_1406ae220 as *const (), Throws::Discard), // sub_1406ae220
    Replacement::draft_throws(0x1406ae2c0, "contentbonsai_1406ae2c0", fn_1406ae2c0 as *const (), Throws::Discard), // sub_1406ae2c0
    Replacement::draft(0x1406ae390, "contentbonsai_1406ae390", fn_1406ae390 as *const ()), // sub_1406ae390
    Replacement::draft(0x1406ae440, "contentbonsai_1406ae440", fn_1406ae440 as *const ()), // sub_1406ae440
    Replacement::draft(0x1406ae510, "contentbonsai_1406ae510", fn_1406ae510 as *const ()), // sub_1406ae510
    Replacement::draft(0x1406ae650, "contentbonsai_1406ae650", fn_1406ae650 as *const ()), // sub_1406ae650
    Replacement::draft_throws(0x1406ae9d0, "contentbonsai_1406ae9d0", fn_1406ae9d0 as *const (), Throws::Discard), // sub_1406ae9d0
    Replacement::draft(0x1406aed00, "contentbonsai_1406aed00", fn_1406aed00 as *const ()), // sub_1406aed00
    Replacement::draft_throws(0x1406aef30, "contentbonsai_1406aef30", fn_1406aef30 as *const (), Throws::Discard), // sub_1406aef30
    Replacement::draft(0x1406af2d0, "contentbonsai_1406af2d0", fn_1406af2d0 as *const ()), // sub_1406af2d0
    Replacement::draft_throws(0x1406af3b0, "contentbonsai_1406af3b0", fn_1406af3b0 as *const (), Throws::Caught), // sub_1406af3b0
    Replacement::draft_throws(0x1406af900, "contentbonsai_1406af900", fn_1406af900 as *const (), Throws::Caught), // sub_1406af900
    Replacement::draft_throws(0x1406afdb0, "contentbonsai_1406afdb0", fn_1406afdb0 as *const (), Throws::Caught), // sub_1406afdb0
    Replacement::draft(0x1406b0270, "contentbonsai_1406b0270", fn_1406b0270 as *const ()), // sub_1406b0270
    Replacement::draft(0x1406b0440, "contentbonsai_1406b0440", fn_1406b0440 as *const ()), // sub_1406b0440
    Replacement::draft(0x1406b06d0, "contentbonsai_1406b06d0", fn_1406b06d0 as *const ()), // sub_1406b06d0
    Replacement::draft_throws(0x1406b0970, "contentbonsai_1406b0970", fn_1406b0970 as *const (), Throws::Discard), // sub_1406b0970
    Replacement::draft_throws(0x1406b0be0, "contentbonsai_1406b0be0", fn_1406b0be0 as *const (), Throws::Caught), // sub_1406b0be0
    Replacement::draft(0x1406b0f30, "contentbonsai_1406b0f30", fn_1406b0f30 as *const ()), // sub_1406b0f30
    Replacement::draft(0x1406b1090, "contentbonsai_1406b1090", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143096c60, 0x8> as *const ()), // sub_1406b1090
    Replacement::draft(0x1406b1110, "contentbonsai_1406b1110", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143096c60> as *const ()), // sub_1406b1110
    Replacement::draft(0x1406b11a0, "contentbonsai_1406b11a0", fn_1406b11a0 as *const ()), // sub_1406b11a0
    Replacement::draft(0x1406b14c0, "contentbonsai_1406b14c0", fn_1406b14c0 as *const ()), // sub_1406b14c0
    Replacement::draft_throws(0x1406b1830, "contentbonsai_1406b1830", fn_1406b1830 as *const (), Throws::Caught), // sub_1406b1830
    Replacement::draft(0x1406b1c20, "contentbonsai_1406b1c20", fn_1406b1c20 as *const ()), // sub_1406b1c20
    Replacement::draft_throws(0x1406b1fb0, "contentbonsai_1406b1fb0", fn_1406b1fb0 as *const (), Throws::Discard), // sub_1406b1fb0
    Replacement::draft_throws(0x1406b2320, "contentbonsai_1406b2320", fn_1406b2320 as *const (), Throws::Caught), // sub_1406b2320
    Replacement::draft_throws(0x1406b2580, "contentbonsai_1406b2580", fn_1406b2580 as *const (), Throws::Caught), // sub_1406b2580
    Replacement::draft_throws(0x1406b27b0, "contentbonsai_1406b27b0", fn_1406b27b0 as *const (), Throws::Caught), // sub_1406b27b0
    Replacement::draft(0x1406b29e0, "contentbonsai_1406b29e0", fn_1406b29e0 as *const ()), // sub_1406b29e0
    Replacement::draft(0x1406b2dd0, "contentbonsai_1406b2dd0", fn_1406b2dd0 as *const ()), // sub_1406b2dd0
    Replacement::draft(0x1406b3140, "contentbonsai_1406b3140", fn_1406b3140 as *const ()), // sub_1406b3140
    Replacement::draft_throws(0x1406b34c0, "contentbonsai_1406b34c0", fn_1406b34c0 as *const (), Throws::Discard), // sub_1406b34c0
    Replacement::draft_throws(0x1406b3830, "contentbonsai_1406b3830", fn_1406b3830 as *const (), Throws::Discard), // sub_1406b3830
    Replacement::draft(0x1406b3960, "contentbonsai_1406b3960", tpl_6676::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14315c170, 0x8> as *const ()), // sub_1406b3960
    Replacement::draft(0x1406b39f0, "contentbonsai_1406b39f0", tpl_6677::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14315c170, 0xfffffffffffffff8, 0x20> as *const ()), // sub_1406b39f0
    Replacement::draft(0x1406b3a90, "contentbonsai_1406b3a90", fn_1406b3a90 as *const ()), // sub_1406b3a90
    Replacement::draft(0x1406b3b90, "contentbonsai_1406b3b90", fn_1406b3b90 as *const ()), // sub_1406b3b90
    Replacement::draft_throws(0x1406b3ce0, "contentbonsai_1406b3ce0", fn_1406b3ce0 as *const (), Throws::Discard), // sub_1406b3ce0
    Replacement::draft_throws(0x1406b3e20, "contentbonsai_1406b3e20", fn_1406b3e20 as *const (), Throws::Discard), // sub_1406b3e20
    Replacement::draft_throws(0x1406b3f90, "contentbonsai_1406b3f90", fn_1406b3f90 as *const (), Throws::Discard), // sub_1406b3f90
    Replacement::draft_throws(0x1406b4110, "contentbonsai_1406b4110", fn_1406b4110 as *const (), Throws::Caught), // sub_1406b4110
    Replacement::draft_throws(0x1406b4370, "contentbonsai_1406b4370", fn_1406b4370 as *const (), Throws::Caught), // sub_1406b4370
    Replacement::draft_throws(0x1406b45a0, "contentbonsai_1406b45a0", fn_1406b45a0 as *const (), Throws::Caught), // sub_1406b45a0
    Replacement::draft(0x1406b47d0, "contentbonsai_1406b47d0", fn_1406b47d0 as *const ()), // sub_1406b47d0
    Replacement::draft(0x1406b48c0, "contentbonsai_1406b48c0", fn_1406b48c0 as *const ()), // sub_1406b48c0
    Replacement::draft(0x1406b49c0, "contentbonsai_1406b49c0", fn_1406b49c0 as *const ()), // sub_1406b49c0
    Replacement::draft_throws(0x1406b4ad0, "contentbonsai_1406b4ad0", fn_1406b4ad0 as *const (), Throws::Discard), // sub_1406b4ad0
    Replacement::draft_throws(0x1406b4bc0, "contentbonsai_1406b4bc0", fn_1406b4bc0 as *const (), Throws::Discard), // sub_1406b4bc0
    Replacement::draft(0x1406b4cc0, "contentbonsai_1406b4cc0", fn_1406b4cc0 as *const ()), // sub_1406b4cc0
    Replacement::draft(0x1406b4d80, "contentbonsai_1406b4d80", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143231058> as *const ()), // sub_1406b4d80
    Replacement::draft(0x1406b4de0, "contentbonsai_1406b4de0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143231058> as *const ()), // sub_1406b4de0
    Replacement::draft(0x1406b4e70, "contentbonsai_1406b4e70", fn_1406b4e70 as *const ()), // sub_1406b4e70
    Replacement::draft_throws(0x1406b4fe0, "contentbonsai_1406b4fe0", tpl_6689::<0x14387cbcc, 0x14387cba0, 0x1401e9fa0, 0x1406e02b0, 0x1406d7a10, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406b4fe0
    Replacement::draft_throws(0x1406b5140, "contentbonsai_1406b5140", tpl_6689::<0x14387cbcc, 0x14387cba0, 0x1401e9b60, 0x1406e03a0, 0x1406d7a10, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406b5140
    Replacement::draft_throws(0x1406b52a0, "contentbonsai_1406b52a0", fn_1406b52a0 as *const (), Throws::Caught), // sub_1406b52a0
    Replacement::draft(0x1406b5330, "contentbonsai_1406b5330", fn_1406b5330 as *const ()), // sub_1406b5330
    Replacement::draft_throws(0x1406b5490, "contentbonsai_1406b5490", tpl_6689::<0x14387cbcc, 0x14387cba0, 0x1401e9770, 0x1406e1570, 0x1406d7a10, 0x1401e9590> as *const (), Throws::Discard), // sub_1406b5490
    Replacement::draft_throws(0x1406b55f0, "contentbonsai_1406b55f0", fn_1406b55f0 as *const (), Throws::Discard), // sub_1406b55f0
    Replacement::draft(0x1406b56a0, "contentbonsai_1406b56a0", fn_1406b56a0 as *const ()), // sub_1406b56a0
    Replacement::draft(0x1406b5740, "contentbonsai_1406b5740", fn_1406b5740 as *const ()), // sub_1406b5740
    Replacement::draft(0x1406b57e0, "contentbonsai_1406b57e0", tpl_6695::<0x14387ca98, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406b57e0
    Replacement::draft(0x1406b5850, "contentbonsai_1406b5850", fn_1406b5850 as *const ()), // sub_1406b5850
    Replacement::draft_throws(0x1406b5a10, "contentbonsai_1406b5a10", fn_1406b5a10 as *const (), Throws::Caught), // sub_1406b5a10
    Replacement::draft(0x1406b5cd0, "contentbonsai_1406b5cd0", fn_1406b5cd0 as *const ()), // sub_1406b5cd0
    Replacement::draft_throws(0x1406b5e70, "contentbonsai_1406b5e70", fn_1406b5e70 as *const (), Throws::Discard), // sub_1406b5e70
    Replacement::draft_throws(0x1406b5f80, "contentbonsai_1406b5f80", fn_1406b5f80 as *const (), Throws::Caught), // sub_1406b5f80
    Replacement::draft_throws(0x1406b65c0, "contentbonsai_1406b65c0", fn_1406b65c0 as *const (), Throws::Caught), // sub_1406b65c0
    Replacement::draft_throws(0x1406b6b10, "contentbonsai_1406b6b10", fn_1406b6b10 as *const (), Throws::Caught), // sub_1406b6b10
    Replacement::draft(0x1406b7050, "contentbonsai_1406b7050", fn_1406b7050 as *const ()), // sub_1406b7050
    Replacement::draft(0x1406b7400, "contentbonsai_1406b7400", fn_1406b7400 as *const ()), // sub_1406b7400
    Replacement::draft(0x1406b7560, "contentbonsai_1406b7560", fn_1406b7560 as *const ()), // sub_1406b7560
    Replacement::draft_throws(0x1406b76d0, "contentbonsai_1406b76d0", fn_1406b76d0 as *const (), Throws::Discard), // sub_1406b76d0
    Replacement::draft_throws(0x1406b7820, "contentbonsai_1406b7820", fn_1406b7820 as *const (), Throws::Discard), // sub_1406b7820
    Replacement::draft(0x1406b79f0, "contentbonsai_1406b79f0", tpl_6708::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14315c430, 0x28, 0x20> as *const ()), // sub_1406b79f0
    Replacement::draft(0x1406b7a90, "contentbonsai_1406b7a90", tpl_6709::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x14315c430, 0xffffffffffffffe8, 0xfffffffffffffff8, 0x9, 0xd> as *const ()), // sub_1406b7a90
    Replacement::draft(0x1406b7b40, "contentbonsai_1406b7b40", fn_1406b7b40 as *const ()), // sub_1406b7b40
    Replacement::draft(0x1406b7c80, "contentbonsai_1406b7c80", fn_1406b7c80 as *const ()), // sub_1406b7c80
    Replacement::draft_throws(0x1406b7e60, "contentbonsai_1406b7e60", fn_1406b7e60 as *const (), Throws::Discard), // sub_1406b7e60
    Replacement::draft(0x1406b8000, "contentbonsai_1406b8000", fn_1406b8000 as *const ()), // sub_1406b8000
    Replacement::draft_throws(0x1406b8160, "contentbonsai_1406b8160", fn_1406b8160 as *const (), Throws::Discard), // sub_1406b8160
    Replacement::draft_throws(0x1406b8310, "contentbonsai_1406b8310", fn_1406b8310 as *const (), Throws::Caught), // sub_1406b8310
    Replacement::draft_throws(0x1406b8400, "contentbonsai_1406b8400", fn_1406b8400 as *const (), Throws::Caught), // sub_1406b8400
    Replacement::draft_throws(0x1406b84f0, "contentbonsai_1406b84f0", fn_1406b84f0 as *const (), Throws::Caught), // sub_1406b84f0
    Replacement::draft(0x1406b85e0, "contentbonsai_1406b85e0", fn_1406b85e0 as *const ()), // sub_1406b85e0
    Replacement::draft(0x1406b8740, "contentbonsai_1406b8740", fn_1406b8740 as *const ()), // sub_1406b8740
    Replacement::draft(0x1406b8890, "contentbonsai_1406b8890", fn_1406b8890 as *const ()), // sub_1406b8890
    Replacement::draft_throws(0x1406b89f0, "contentbonsai_1406b89f0", fn_1406b89f0 as *const (), Throws::Discard), // sub_1406b89f0
    Replacement::draft_throws(0x1406b8b50, "contentbonsai_1406b8b50", fn_1406b8b50 as *const (), Throws::Caught), // sub_1406b8b50
    Replacement::draft(0x1406b8cf0, "contentbonsai_1406b8cf0", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143224e78> as *const ()), // sub_1406b8cf0
    Replacement::draft(0x1406b8d80, "contentbonsai_1406b8d80", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143224e78, 0x18> as *const ()), // sub_1406b8d80
    Replacement::draft(0x1406b8e30, "contentbonsai_1406b8e30", fn_1406b8e30 as *const ()), // sub_1406b8e30
    Replacement::draft(0x1406b8eb0, "contentbonsai_1406b8eb0", fn_1406b8eb0 as *const ()), // sub_1406b8eb0
    Replacement::draft_throws(0x1406b8fc0, "contentbonsai_1406b8fc0", fn_1406b8fc0 as *const (), Throws::Discard), // sub_1406b8fc0
    Replacement::draft(0x1406b90c0, "contentbonsai_1406b90c0", tpl_6725::<0x14387cb28, 0x14243b0b0, 0x1406e01b0, 0x2, 0x1406d7c90, 0x338, 0x8, 0x80, 0xc, 0x38, 0x10, 0xfffffffffffffff8> as *const ()), // sub_1406b90c0
    Replacement::draft_throws(0x1406b9170, "contentbonsai_1406b9170", fn_1406b9170 as *const (), Throws::Discard), // sub_1406b9170
    Replacement::draft_throws(0x1406b9280, "contentbonsai_1406b9280", fn_1406b9280 as *const (), Throws::Caught), // sub_1406b9280
    Replacement::draft_throws(0x1406b93f0, "contentbonsai_1406b93f0", fn_1406b93f0 as *const (), Throws::Caught), // sub_1406b93f0
    Replacement::draft_throws(0x1406b9560, "contentbonsai_1406b9560", fn_1406b9560 as *const (), Throws::Caught), // sub_1406b9560
    Replacement::draft(0x1406b96d0, "contentbonsai_1406b96d0", fn_1406b96d0 as *const ()), // sub_1406b96d0
    Replacement::draft(0x1406b9760, "contentbonsai_1406b9760", fn_1406b9760 as *const ()), // sub_1406b9760
    Replacement::draft(0x1406b9810, "contentbonsai_1406b9810", tpl_6725::<0x14387cb28, 0x14243b0b0, 0x1406e01b0, 0x2, 0x1406d7c90, 0x338, 0x8, 0x120, 0xc, 0xd8, 0x10, 0xfffffffffffffff8> as *const ()), // sub_1406b9810
    Replacement::draft_throws(0x1406b98d0, "contentbonsai_1406b98d0", fn_1406b98d0 as *const (), Throws::Discard), // sub_1406b98d0
    Replacement::draft_throws(0x1406b99a0, "contentbonsai_1406b99a0", fn_1406b99a0 as *const (), Throws::Discard), // sub_1406b99a0
    Replacement::draft(0x1406b9b30, "contentbonsai_1406b9b30", tpl_65793::<0x143426ac0, 0x143244db0, 0xc, 0x143244d60, 0x143244d88, 0x14020f270, 0x143426ae8, 0x143244930, 0x10, 0x143244968, 0x143244908> as *const ()), // sub_1406b9b30
    Replacement::draft(0x1406b9bd0, "contentbonsai_1406b9bd0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143248a60> as *const ()), // sub_1406b9bd0
    Replacement::draft(0x1406b9c30, "contentbonsai_1406b9c30", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143248a60> as *const ()), // sub_1406b9c30
    Replacement::draft(0x1406b9cc0, "contentbonsai_1406b9cc0", fn_1406b9cc0 as *const ()), // sub_1406b9cc0
    Replacement::draft_throws(0x1406b9d60, "contentbonsai_1406b9d60", tpl_6734::<0x14387cb68, 0x14387cb18, 0x1401e9fa0, 0x1406e02b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406b9d60
    Replacement::draft_throws(0x1406b9df0, "contentbonsai_1406b9df0", tpl_6734::<0x14387cb68, 0x14387cb18, 0x1401e9b60, 0x1406e03a0, 0x318, 0x328, 0x320, 0x1406e3600, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406b9df0
    Replacement::draft_throws(0x1406b9e80, "contentbonsai_1406b9e80", fn_1406b9e80 as *const (), Throws::Caught), // sub_1406b9e80
    Replacement::draft(0x1406b9f10, "contentbonsai_1406b9f10", fn_1406b9f10 as *const ()), // sub_1406b9f10
    Replacement::draft_throws(0x1406b9fa0, "contentbonsai_1406b9fa0", tpl_6734::<0x14387cb68, 0x14387cb18, 0x1401e9770, 0x1406e1570, 0x318, 0x328, 0x320, 0x1406e3600, 0x1401e9590> as *const (), Throws::Discard), // sub_1406b9fa0
    Replacement::draft_throws(0x1406ba030, "contentbonsai_1406ba030", tpl_6737::<0x14387cb18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406ba030
    Replacement::draft(0x1406ba0e0, "contentbonsai_1406ba0e0", fn_1406ba0e0 as *const ()), // sub_1406ba0e0
    Replacement::draft(0x1406ba1b0, "contentbonsai_1406ba1b0", fn_1406ba1b0 as *const ()), // sub_1406ba1b0
    Replacement::draft(0x1406ba290, "contentbonsai_1406ba290", fn_1406ba290 as *const ()), // sub_1406ba290
    Replacement::draft(0x1406ba370, "contentbonsai_1406ba370", fn_1406ba370 as *const ()), // sub_1406ba370
    Replacement::draft_throws(0x1406ba460, "contentbonsai_1406ba460", fn_1406ba460 as *const (), Throws::Discard), // sub_1406ba460
    Replacement::draft_throws(0x1406ba510, "contentbonsai_1406ba510", fn_1406ba510 as *const (), Throws::Discard), // sub_1406ba510
    Replacement::draft_throws(0x1406ba5d0, "contentbonsai_1406ba5d0", fn_1406ba5d0 as *const (), Throws::Discard), // sub_1406ba5d0
    Replacement::draft_throws(0x1406ba670, "contentbonsai_1406ba670", fn_1406ba670 as *const (), Throws::Discard), // sub_1406ba670
    Replacement::draft_throws(0x1406ba7e0, "contentbonsai_1406ba7e0", fn_1406ba7e0 as *const (), Throws::Caught), // sub_1406ba7e0
    Replacement::draft_throws(0x1406ba890, "contentbonsai_1406ba890", fn_1406ba890 as *const (), Throws::Caught), // sub_1406ba890
    Replacement::draft_throws(0x1406ba940, "contentbonsai_1406ba940", fn_1406ba940 as *const (), Throws::Caught), // sub_1406ba940
    Replacement::draft(0x1406ba9f0, "contentbonsai_1406ba9f0", fn_1406ba9f0 as *const ()), // sub_1406ba9f0
    Replacement::draft(0x1406baaf0, "contentbonsai_1406baaf0", fn_1406baaf0 as *const ()), // sub_1406baaf0
    Replacement::draft(0x1406babe0, "contentbonsai_1406babe0", fn_1406babe0 as *const ()), // sub_1406babe0
    Replacement::draft_throws(0x1406bacd0, "contentbonsai_1406bacd0", fn_1406bacd0 as *const (), Throws::Discard), // sub_1406bacd0
    Replacement::draft_throws(0x1406badc0, "contentbonsai_1406badc0", tpl_6745::<0x14387cc00, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x143244920, 0x8, 0x1432448c8, 0x1406d7ec0> as *const (), Throws::Discard), // sub_1406badc0
    Replacement::draft(0x1406baea0, "contentbonsai_1406baea0", fn_1406baea0 as *const ()), // sub_1406baea0
    Replacement::draft(0x1406baf80, "contentbonsai_1406baf80", fn_1406baf80 as *const ()), // sub_1406baf80
    Replacement::draft(0x1406bb040, "contentbonsai_1406bb040", tpl_6748::<0x14387cc08, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406bb040
    Replacement::draft(0x1406bb0b0, "contentbonsai_1406bb0b0", fn_1406bb0b0 as *const ()), // sub_1406bb0b0
    Replacement::draft_throws(0x1406bb450, "contentbonsai_1406bb450", fn_1406bb450 as *const (), Throws::Caught), // sub_1406bb450
    Replacement::draft(0x1406bb900, "contentbonsai_1406bb900", fn_1406bb900 as *const ()), // sub_1406bb900
    Replacement::draft_throws(0x1406bbb90, "contentbonsai_1406bbb90", fn_1406bbb90 as *const (), Throws::Discard), // sub_1406bbb90
    Replacement::draft(0x1406bbdf0, "contentbonsai_1406bbdf0", fn_1406bbdf0 as *const ()), // sub_1406bbdf0
    Replacement::draft_throws(0x1406bbee0, "contentbonsai_1406bbee0", fn_1406bbee0 as *const (), Throws::Caught), // sub_1406bbee0
    Replacement::draft_throws(0x1406bc670, "contentbonsai_1406bc670", fn_1406bc670 as *const (), Throws::Caught), // sub_1406bc670
    Replacement::draft_throws(0x1406bcce0, "contentbonsai_1406bcce0", fn_1406bcce0 as *const (), Throws::Caught), // sub_1406bcce0
    Replacement::draft(0x1406bd3a0, "contentbonsai_1406bd3a0", fn_1406bd3a0 as *const ()), // sub_1406bd3a0
    Replacement::draft(0x1406bd930, "contentbonsai_1406bd930", fn_1406bd930 as *const ()), // sub_1406bd930
    Replacement::draft(0x1406bdb40, "contentbonsai_1406bdb40", fn_1406bdb40 as *const ()), // sub_1406bdb40
    Replacement::draft_throws(0x1406bdd60, "contentbonsai_1406bdd60", fn_1406bdd60 as *const (), Throws::Discard), // sub_1406bdd60
    Replacement::draft_throws(0x1406bdf80, "contentbonsai_1406bdf80", fn_1406bdf80 as *const (), Throws::Caught), // sub_1406bdf80
    Replacement::draft(0x1406be3b0, "contentbonsai_1406be3b0", fn_1406be3b0 as *const ()), // sub_1406be3b0
    Replacement::draft(0x1406be450, "contentbonsai_1406be450", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143245fe8> as *const ()), // sub_1406be450
    Replacement::draft(0x1406be4b0, "contentbonsai_1406be4b0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143245fe8> as *const ()), // sub_1406be4b0
    Replacement::draft_throws(0x1406be5b0, "contentbonsai_1406be5b0", fn_1406be5b0 as *const (), Throws::Caught), // sub_1406be5b0
    Replacement::draft_throws(0x1406be640, "contentbonsai_1406be640", tpl_6762::<0x14387cbb8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406be640
    Replacement::draft(0x1406be6f0, "contentbonsai_1406be6f0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143220278> as *const ()), // sub_1406be6f0
    Replacement::draft(0x1406be750, "contentbonsai_1406be750", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143220278> as *const ()), // sub_1406be750
    Replacement::draft_throws(0x1406be850, "contentbonsai_1406be850", fn_1406be850 as *const (), Throws::Caught), // sub_1406be850
    Replacement::draft_throws(0x1406be8e0, "contentbonsai_1406be8e0", tpl_6763::<0x14387cac0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406be8e0
    Replacement::draft(0x1406be990, "contentbonsai_1406be990", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14321d330> as *const ()), // sub_1406be990
    Replacement::draft(0x1406be9f0, "contentbonsai_1406be9f0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14321d330> as *const ()), // sub_1406be9f0
    Replacement::draft_throws(0x1406beaf0, "contentbonsai_1406beaf0", fn_1406beaf0 as *const (), Throws::Caught), // sub_1406beaf0
    Replacement::draft_throws(0x1406beb80, "contentbonsai_1406beb80", tpl_6763::<0x14387cb00, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406beb80
    Replacement::draft(0x1406bec30, "contentbonsai_1406bec30", fn_1406bec30 as *const ()), // sub_1406bec30
    Replacement::draft(0x1406becd0, "contentbonsai_1406becd0", fn_1406becd0 as *const ()), // sub_1406becd0
    Replacement::draft(0x1406bed80, "contentbonsai_1406bed80", tpl_6766::<0x14387cb88, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406bed80
    Replacement::draft(0x1406bedf0, "contentbonsai_1406bedf0", fn_1406bedf0 as *const ()), // sub_1406bedf0
    Replacement::draft_throws(0x1406bef70, "contentbonsai_1406bef70", fn_1406bef70 as *const (), Throws::Discard), // sub_1406bef70
    Replacement::draft(0x1406bf110, "contentbonsai_1406bf110", fn_1406bf110 as *const ()), // sub_1406bf110
    Replacement::draft_throws(0x1406bf240, "contentbonsai_1406bf240", fn_1406bf240 as *const (), Throws::Discard), // sub_1406bf240
    Replacement::draft(0x1406bf3a0, "contentbonsai_1406bf3a0", fn_1406bf3a0 as *const ()), // sub_1406bf3a0
    Replacement::draft_throws(0x1406bf450, "contentbonsai_1406bf450", fn_1406bf450 as *const (), Throws::Caught), // sub_1406bf450
    Replacement::draft_throws(0x1406bf850, "contentbonsai_1406bf850", fn_1406bf850 as *const (), Throws::Caught), // sub_1406bf850
    Replacement::draft_throws(0x1406bfbe0, "contentbonsai_1406bfbe0", fn_1406bfbe0 as *const (), Throws::Caught), // sub_1406bfbe0
    Replacement::draft(0x1406bff60, "contentbonsai_1406bff60", fn_1406bff60 as *const ()), // sub_1406bff60
    Replacement::draft(0x1406c0070, "contentbonsai_1406c0070", fn_1406c0070 as *const ()), // sub_1406c0070
    Replacement::draft(0x1406c01a0, "contentbonsai_1406c01a0", fn_1406c01a0 as *const ()), // sub_1406c01a0
    Replacement::draft_throws(0x1406c02e0, "contentbonsai_1406c02e0", fn_1406c02e0 as *const (), Throws::Discard), // sub_1406c02e0
    Replacement::draft_throws(0x1406c0420, "contentbonsai_1406c0420", fn_1406c0420 as *const (), Throws::Discard), // sub_1406c0420
    Replacement::draft_throws(0x1406c05d0, "contentbonsai_1406c05d0", fn_1406c05d0 as *const (), Throws::Discard), // sub_1406c05d0
    Replacement::draft(0x1406c0660, "contentbonsai_1406c0660", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14321abf0> as *const ()), // sub_1406c0660
    Replacement::draft(0x1406c06c0, "contentbonsai_1406c06c0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14321abf0> as *const ()), // sub_1406c06c0
    Replacement::draft_throws(0x1406c07c0, "contentbonsai_1406c07c0", fn_1406c07c0 as *const (), Throws::Caught), // sub_1406c07c0
    Replacement::draft_throws(0x1406c0850, "contentbonsai_1406c0850", tpl_6762::<0x14387cb60, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406c0850
    Replacement::draft(0x1406c0900, "contentbonsai_1406c0900", fn_1406c0900 as *const ()), // sub_1406c0900
    Replacement::draft(0x1406c09f0, "contentbonsai_1406c09f0", fn_1406c09f0 as *const ()), // sub_1406c09f0
    Replacement::draft(0x1406c0b20, "contentbonsai_1406c0b20", fn_1406c0b20 as *const ()), // sub_1406c0b20
    Replacement::draft(0x1406c0c10, "contentbonsai_1406c0c10", fn_1406c0c10 as *const ()), // sub_1406c0c10
    Replacement::draft_throws(0x1406c0e10, "contentbonsai_1406c0e10", fn_1406c0e10 as *const (), Throws::Caught), // sub_1406c0e10
    Replacement::draft(0x1406c13f0, "contentbonsai_1406c13f0", fn_1406c13f0 as *const ()), // sub_1406c13f0
    Replacement::draft_throws(0x1406c1590, "contentbonsai_1406c1590", fn_1406c1590 as *const (), Throws::Caught), // sub_1406c1590
    Replacement::draft(0x1406c1b30, "contentbonsai_1406c1b30", fn_1406c1b30 as *const ()), // sub_1406c1b30
    Replacement::draft_throws(0x1406c1c90, "contentbonsai_1406c1c90", fn_1406c1c90 as *const (), Throws::Caught), // sub_1406c1c90
    Replacement::draft_throws(0x1406c2120, "contentbonsai_1406c2120", fn_1406c2120 as *const (), Throws::Caught), // sub_1406c2120
    Replacement::draft_throws(0x1406c2550, "contentbonsai_1406c2550", fn_1406c2550 as *const (), Throws::Caught), // sub_1406c2550
    Replacement::draft(0x1406c2960, "contentbonsai_1406c2960", fn_1406c2960 as *const ()), // sub_1406c2960
    Replacement::draft(0x1406c2bc0, "contentbonsai_1406c2bc0", fn_1406c2bc0 as *const ()), // sub_1406c2bc0
    Replacement::draft(0x1406c2d30, "contentbonsai_1406c2d30", fn_1406c2d30 as *const ()), // sub_1406c2d30
    Replacement::draft_throws(0x1406c2eb0, "contentbonsai_1406c2eb0", fn_1406c2eb0 as *const (), Throws::Discard), // sub_1406c2eb0
    Replacement::draft_throws(0x1406c3020, "contentbonsai_1406c3020", fn_1406c3020 as *const (), Throws::Caught), // sub_1406c3020
    Replacement::draft(0x1406c33b0, "contentbonsai_1406c33b0", fn_1406c33b0 as *const ()), // sub_1406c33b0
    Replacement::draft(0x1406c35c0, "contentbonsai_1406c35c0", fn_1406c35c0 as *const ()), // sub_1406c35c0
    Replacement::draft(0x1406c3670, "contentbonsai_1406c3670", fn_1406c3670 as *const ()), // sub_1406c3670
    Replacement::draft(0x1406c3770, "contentbonsai_1406c3770", tpl_6796::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x70, 0x143246b58, 0xffffffffffffff90, 0xffffffffffffffb0, 0xffffffffffffffd0, 0xfffffffffffffff0> as *const ()), // sub_1406c3770
    Replacement::draft(0x1406c3890, "contentbonsai_1406c3890", tpl_6797::<0x14387cb78, 0x14243b0b0, 0x1406ea3a0, 0x14387d230, 0x20, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1406c3890
    Replacement::draft(0x1406c3930, "contentbonsai_1406c3930", tpl_6798::<0x14387cb78, 0x14243b0b0, 0x1406ea470, 0x20, 0x1406fc6d0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406c3930
    Replacement::draft_throws(0x1406c39b0, "contentbonsai_1406c39b0", tpl_6799::<0x14387caa8, 0x14387cb78, 0x1401e9fa0, 0x1406ea600, 0x20, 0x1406d96a0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406c39b0
    Replacement::draft_throws(0x1406c3a20, "contentbonsai_1406c3a20", fn_1406c3a20 as *const (), Throws::Discard), // sub_1406c3a20
    Replacement::draft_throws(0x1406c3aa0, "contentbonsai_1406c3aa0", tpl_6799::<0x14387caa8, 0x14387cb78, 0x1401e9b60, 0x1406ea8d0, 0x20, 0x1406d97d0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406c3aa0
    Replacement::draft_throws(0x1406c3b10, "contentbonsai_1406c3b10", fn_1406c3b10 as *const (), Throws::Discard), // sub_1406c3b10
    Replacement::draft_throws(0x1406c3be0, "contentbonsai_1406c3be0", fn_1406c3be0 as *const (), Throws::Caught), // sub_1406c3be0
    Replacement::draft_throws(0x1406c3ca0, "contentbonsai_1406c3ca0", fn_1406c3ca0 as *const (), Throws::Caught), // sub_1406c3ca0
    Replacement::draft_throws(0x1406c3d60, "contentbonsai_1406c3d60", fn_1406c3d60 as *const (), Throws::Caught), // sub_1406c3d60
    Replacement::draft(0x1406c3e20, "contentbonsai_1406c3e20", tpl_6803::<0x14387cb78, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x1406fc920, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406c3e20
    Replacement::draft(0x1406c3ea0, "contentbonsai_1406c3ea0", tpl_6804::<0x14387caa8, 0x14387cb78, 0x1401e9970, 0x1406ec510, 0x20, 0x1406fce10, 0x1401e9860> as *const ()), // sub_1406c3ea0
    Replacement::draft(0x1406c3f10, "contentbonsai_1406c3f10", tpl_6798::<0x14387cb78, 0x14243b0b0, 0x1406ec600, 0x20, 0x1406fcfb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406c3f10
    Replacement::draft_throws(0x1406c3f90, "contentbonsai_1406c3f90", tpl_6805::<0x14387caa8, 0x14387cb78, 0x1401e9770, 0x1406ec700, 0x20, 0x1406fd150, 0x1401e9590> as *const (), Throws::Discard), // sub_1406c3f90
    Replacement::draft_throws(0x1406c4000, "contentbonsai_1406c4000", fn_1406c4000 as *const (), Throws::Discard), // sub_1406c4000
    Replacement::draft(0x1406c40b0, "contentbonsai_1406c40b0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143216b80> as *const ()), // sub_1406c40b0
    Replacement::draft(0x1406c4110, "contentbonsai_1406c4110", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143216b80> as *const ()), // sub_1406c4110
    Replacement::draft_throws(0x1406c4210, "contentbonsai_1406c4210", fn_1406c4210 as *const (), Throws::Caught), // sub_1406c4210
    Replacement::draft_throws(0x1406c42a0, "contentbonsai_1406c42a0", tpl_6763::<0x14387cb48, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406c42a0
    Replacement::draft(0x1406c4350, "contentbonsai_1406c4350", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14317d818, 0x8> as *const ()), // sub_1406c4350
    Replacement::draft(0x1406c43d0, "contentbonsai_1406c43d0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14317d818> as *const ()), // sub_1406c43d0
    Replacement::draft(0x1406c44d0, "contentbonsai_1406c44d0", fn_1406c44d0 as *const ()), // sub_1406c44d0
    Replacement::draft_throws(0x1406c45c0, "contentbonsai_1406c45c0", fn_1406c45c0 as *const (), Throws::Caught), // sub_1406c45c0
    Replacement::draft(0x1406c4700, "contentbonsai_1406c4700", fn_1406c4700 as *const ()), // sub_1406c4700
    Replacement::draft_throws(0x1406c4810, "contentbonsai_1406c4810", fn_1406c4810 as *const (), Throws::Discard), // sub_1406c4810
    Replacement::draft_throws(0x1406c48b0, "contentbonsai_1406c48b0", fn_1406c48b0 as *const (), Throws::Caught), // sub_1406c48b0
    Replacement::draft_throws(0x1406c4bf0, "contentbonsai_1406c4bf0", fn_1406c4bf0 as *const (), Throws::Caught), // sub_1406c4bf0
    Replacement::draft_throws(0x1406c4ed0, "contentbonsai_1406c4ed0", fn_1406c4ed0 as *const (), Throws::Caught), // sub_1406c4ed0
    Replacement::draft(0x1406c51a0, "contentbonsai_1406c51a0", fn_1406c51a0 as *const ()), // sub_1406c51a0
    Replacement::draft(0x1406c5390, "contentbonsai_1406c5390", fn_1406c5390 as *const ()), // sub_1406c5390
    Replacement::draft(0x1406c5460, "contentbonsai_1406c5460", fn_1406c5460 as *const ()), // sub_1406c5460
    Replacement::draft_throws(0x1406c5540, "contentbonsai_1406c5540", fn_1406c5540 as *const (), Throws::Discard), // sub_1406c5540
    Replacement::draft_throws(0x1406c5610, "contentbonsai_1406c5610", fn_1406c5610 as *const (), Throws::Discard), // sub_1406c5610
    Replacement::draft(0x1406c5740, "contentbonsai_1406c5740", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143247948> as *const ()), // sub_1406c5740
    Replacement::draft(0x1406c57a0, "contentbonsai_1406c57a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143247948> as *const ()), // sub_1406c57a0
    Replacement::draft(0x1406c5830, "contentbonsai_1406c5830", tpl_6819::<0x14387cab0, 0x14243b0b0, 0x1406e01b0, 0x1, 0x1406d7a10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406c5830
    Replacement::draft_throws(0x1406c58c0, "contentbonsai_1406c58c0", tpl_65875::<0x14387cae4, 0x14387cab0, 0x1401e9fa0, 0x1406e02b0, 0x1, 0x1406d7a10, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406c58c0
    Replacement::draft_throws(0x1406c5940, "contentbonsai_1406c5940", tpl_65875::<0x14387cae4, 0x14387cab0, 0x1401e9b60, 0x1406e03a0, 0x1, 0x1406d7a10, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406c5940
    Replacement::draft_throws(0x1406c59c0, "contentbonsai_1406c59c0", fn_1406c59c0 as *const (), Throws::Caught), // sub_1406c59c0
    Replacement::draft(0x1406c5a50, "contentbonsai_1406c5a50", fn_1406c5a50 as *const ()), // sub_1406c5a50
    Replacement::draft_throws(0x1406c5ad0, "contentbonsai_1406c5ad0", tpl_65875::<0x14387cae4, 0x14387cab0, 0x1401e9770, 0x1406e1570, 0x1, 0x1406d7a10, 0x1401e9590> as *const (), Throws::Discard), // sub_1406c5ad0
    Replacement::draft_throws(0x1406c5b50, "contentbonsai_1406c5b50", tpl_6823::<0x14387cab0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1406c5b50
    Replacement::draft(0x1406c5c00, "contentbonsai_1406c5c00", fn_1406c5c00 as *const ()), // sub_1406c5c00
    Replacement::draft(0x1406c5c80, "contentbonsai_1406c5c80", fn_1406c5c80 as *const ()), // sub_1406c5c80
    Replacement::draft(0x1406c5d90, "contentbonsai_1406c5d90", fn_1406c5d90 as *const ()), // sub_1406c5d90
    Replacement::draft_throws(0x1406c5e70, "contentbonsai_1406c5e70", fn_1406c5e70 as *const (), Throws::Discard), // sub_1406c5e70
    Replacement::draft(0x1406c5f60, "contentbonsai_1406c5f60", tpl_6827::<0x14387cbd0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0x9, 0xc, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406c5f60
    Replacement::draft_throws(0x1406c6000, "contentbonsai_1406c6000", fn_1406c6000 as *const (), Throws::Discard), // sub_1406c6000
    Replacement::draft_throws(0x1406c6100, "contentbonsai_1406c6100", fn_1406c6100 as *const (), Throws::Caught), // sub_1406c6100
    Replacement::draft_throws(0x1406c6260, "contentbonsai_1406c6260", fn_1406c6260 as *const (), Throws::Caught), // sub_1406c6260
    Replacement::draft_throws(0x1406c63c0, "contentbonsai_1406c63c0", fn_1406c63c0 as *const (), Throws::Caught), // sub_1406c63c0
    Replacement::draft(0x1406c6520, "contentbonsai_1406c6520", fn_1406c6520 as *const ()), // sub_1406c6520
    Replacement::draft(0x1406c65a0, "contentbonsai_1406c65a0", tpl_6831::<0x14387cbd8, 0x14387cbd0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x9, 0xc, 0xe8, 0x1401e9860> as *const ()), // sub_1406c65a0
    Replacement::draft(0x1406c6640, "contentbonsai_1406c6640", tpl_6827::<0x14387cbd0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0x9, 0xc, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406c6640
    Replacement::draft_throws(0x1406c66f0, "contentbonsai_1406c66f0", tpl_6832::<0x14387cbd8, 0x14387cbd0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x9, 0xc, 0x1401e9590> as *const (), Throws::Discard), // sub_1406c66f0
    Replacement::draft_throws(0x1406c67a0, "contentbonsai_1406c67a0", fn_1406c67a0 as *const (), Throws::Discard), // sub_1406c67a0
    Replacement::draft(0x1406c6920, "contentbonsai_1406c6920", fn_1406c6920 as *const ()), // sub_1406c6920
    Replacement::draft(0x1406c6a10, "contentbonsai_1406c6a10", fn_1406c6a10 as *const ()), // sub_1406c6a10
    Replacement::draft(0x1406c6b00, "contentbonsai_1406c6b00", fn_1406c6b00 as *const ()), // sub_1406c6b00
    Replacement::draft(0x1406c6bc0, "contentbonsai_1406c6bc0", fn_1406c6bc0 as *const ()), // sub_1406c6bc0
    Replacement::draft_throws(0x1406c6e40, "contentbonsai_1406c6e40", fn_1406c6e40 as *const (), Throws::Discard), // sub_1406c6e40
    Replacement::draft(0x1406c7170, "contentbonsai_1406c7170", fn_1406c7170 as *const ()), // sub_1406c7170
    Replacement::draft_throws(0x1406c7390, "contentbonsai_1406c7390", fn_1406c7390 as *const (), Throws::Discard), // sub_1406c7390
    Replacement::draft(0x1406c7600, "contentbonsai_1406c7600", fn_1406c7600 as *const ()), // sub_1406c7600
    Replacement::draft_throws(0x1406c77c0, "contentbonsai_1406c77c0", fn_1406c77c0 as *const (), Throws::Caught), // sub_1406c77c0
    Replacement::draft_throws(0x1406c7d60, "contentbonsai_1406c7d60", fn_1406c7d60 as *const (), Throws::Caught), // sub_1406c7d60
    Replacement::draft_throws(0x1406c8240, "contentbonsai_1406c8240", fn_1406c8240 as *const (), Throws::Caught), // sub_1406c8240
    Replacement::draft(0x1406c8720, "contentbonsai_1406c8720", fn_1406c8720 as *const ()), // sub_1406c8720
    Replacement::draft(0x1406c89b0, "contentbonsai_1406c89b0", fn_1406c89b0 as *const ()), // sub_1406c89b0
    Replacement::draft(0x1406c8bc0, "contentbonsai_1406c8bc0", fn_1406c8bc0 as *const ()), // sub_1406c8bc0
    Replacement::draft_throws(0x1406c8de0, "contentbonsai_1406c8de0", fn_1406c8de0 as *const (), Throws::Discard), // sub_1406c8de0
    Replacement::draft_throws(0x1406c9080, "contentbonsai_1406c9080", fn_1406c9080 as *const (), Throws::Discard), // sub_1406c9080
    Replacement::draft_throws(0x1406c92e0, "contentbonsai_1406c92e0", fn_1406c92e0 as *const (), Throws::Discard), // sub_1406c92e0
    Replacement::draft(0x1406c9370, "contentbonsai_1406c9370", fn_1406c9370 as *const ()), // sub_1406c9370
    Replacement::draft(0x1406c93e0, "contentbonsai_1406c93e0", fn_1406c93e0 as *const ()), // sub_1406c93e0
    Replacement::draft(0x1406c9490, "contentbonsai_1406c9490", fn_1406c9490 as *const ()), // sub_1406c9490
    Replacement::draft(0x1406c9550, "contentbonsai_1406c9550", fn_1406c9550 as *const ()), // sub_1406c9550
    Replacement::draft(0x1406c9630, "contentbonsai_1406c9630", fn_1406c9630 as *const ()), // sub_1406c9630
    Replacement::draft_throws(0x1406c9880, "contentbonsai_1406c9880", fn_1406c9880 as *const (), Throws::Caught), // sub_1406c9880
    Replacement::draft(0x1406c9b10, "contentbonsai_1406c9b10", fn_1406c9b10 as *const ()), // sub_1406c9b10
    Replacement::draft_throws(0x1406c9d40, "contentbonsai_1406c9d40", fn_1406c9d40 as *const (), Throws::Discard), // sub_1406c9d40
    Replacement::draft(0x1406c9f80, "contentbonsai_1406c9f80", fn_1406c9f80 as *const ()), // sub_1406c9f80
    Replacement::draft_throws(0x1406ca060, "contentbonsai_1406ca060", fn_1406ca060 as *const (), Throws::Caught), // sub_1406ca060
    Replacement::draft_throws(0x1406ca4a0, "contentbonsai_1406ca4a0", fn_1406ca4a0 as *const (), Throws::Caught), // sub_1406ca4a0
    Replacement::draft_throws(0x1406ca860, "contentbonsai_1406ca860", fn_1406ca860 as *const (), Throws::Caught), // sub_1406ca860
    Replacement::draft(0x1406cac20, "contentbonsai_1406cac20", fn_1406cac20 as *const ()), // sub_1406cac20
    Replacement::draft(0x1406caf30, "contentbonsai_1406caf30", fn_1406caf30 as *const ()), // sub_1406caf30
    Replacement::draft(0x1406cb0f0, "contentbonsai_1406cb0f0", fn_1406cb0f0 as *const ()), // sub_1406cb0f0
    Replacement::draft_throws(0x1406cb2c0, "contentbonsai_1406cb2c0", fn_1406cb2c0 as *const (), Throws::Discard), // sub_1406cb2c0
    Replacement::draft_throws(0x1406cb4a0, "contentbonsai_1406cb4a0", fn_1406cb4a0 as *const (), Throws::Discard), // sub_1406cb4a0
    Replacement::draft(0x1406cb6b0, "contentbonsai_1406cb6b0", fn_1406cb6b0 as *const ()), // sub_1406cb6b0
    Replacement::draft(0x1406cb770, "contentbonsai_1406cb770", tpl_5937::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1406db3c0> as *const ()), // sub_1406cb770
    Replacement::draft(0x1406cb870, "contentbonsai_1406cb870", tpl_6868::<0x14387cac8, 0x14243b0b0, 0x1438820c0, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1406cb870
    Replacement::draft(0x1406cb900, "contentbonsai_1406cb900", tpl_6869::<0x14387cac8, 0x14243b0b0, 0x1406fe630, 0x338, 0x18, 0x1426d4580, 0x1, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406cb900
    Replacement::draft_throws(0x1406cb9e0, "contentbonsai_1406cb9e0", tpl_65919::<0x14387cb6c, 0x14387cac8, 0x1401e9fa0, 0x1406db430, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406cb9e0
    Replacement::draft(0x1406cbb00, "contentbonsai_1406cbb00", tpl_6870::<0x14387cac8, 0x14243b0b0, 0x1406fe890, 0x338, 0x18, 0x38, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x8> as *const ()), // sub_1406cbb00
    Replacement::draft_throws(0x1406cbbe0, "contentbonsai_1406cbbe0", tpl_6871::<0x14387cb6c, 0x14387cac8, 0x1401e9b60, 0x1406db670, 0x328, 0x338, 0x120, 0x18, 0x8, 0x20, 0x369, 0x24, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406cbbe0
    Replacement::draft_throws(0x1406cbcd0, "contentbonsai_1406cbcd0", fn_1406cbcd0 as *const (), Throws::Caught), // sub_1406cbcd0
    Replacement::draft_throws(0x1406cc030, "contentbonsai_1406cc030", fn_1406cc030 as *const (), Throws::Caught), // sub_1406cc030
    Replacement::draft_throws(0x1406cc340, "contentbonsai_1406cc340", fn_1406cc340 as *const (), Throws::Caught), // sub_1406cc340
    Replacement::draft(0x1406cc630, "contentbonsai_1406cc630", tpl_6875::<0x14387cac8, 0x14243b0b0, 0x1406fe800, 0x18, 0x20, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406cc630
    Replacement::draft(0x1406cc6b0, "contentbonsai_1406cc6b0", tpl_6876::<0x14387cb6c, 0x14387cac8, 0x1401e9970, 0x1406fea30, 0x18, 0xd8, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x1401e9860> as *const ()), // sub_1406cc6b0
    Replacement::draft(0x1406cc790, "contentbonsai_1406cc790", tpl_6870::<0x14387cac8, 0x14243b0b0, 0x1406feb00, 0x338, 0x18, 0xd8, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x8> as *const ()), // sub_1406cc790
    Replacement::draft_throws(0x1406cc880, "contentbonsai_1406cc880", tpl_6877::<0x14387cb6c, 0x14387cac8, 0x1401e9770, 0x1406febe0, 0x338, 0x18, 0x120, 0x20, 0x8, 0x24, 0x2c, 0x1401e9590> as *const (), Throws::Discard), // sub_1406cc880
    Replacement::draft_throws(0x1406cc920, "contentbonsai_1406cc920", fn_1406cc920 as *const (), Throws::Caught), // sub_1406cc920
    Replacement::draft_throws(0x1406cca90, "contentbonsai_1406cca90", tpl_6879::<0x18, 0x1432442a0, 0x14307eab0, 0x1402c4e80, 0x1432442b0, 0x24, 0x143244280> as *const (), Throws::Discard), // sub_1406cca90
    Replacement::draft(0x1406ccb10, "contentbonsai_1406ccb10", fn_1406ccb10 as *const ()), // sub_1406ccb10
    Replacement::draft(0x1406ccbb0, "contentbonsai_1406ccbb0", fn_1406ccbb0 as *const ()), // sub_1406ccbb0
    Replacement::draft(0x1406ccc60, "contentbonsai_1406ccc60", fn_1406ccc60 as *const ()), // sub_1406ccc60
    Replacement::draft(0x1406ccd00, "contentbonsai_1406ccd00", fn_1406ccd00 as *const ()), // sub_1406ccd00
    Replacement::draft_throws(0x1406ccee0, "contentbonsai_1406ccee0", fn_1406ccee0 as *const (), Throws::Caught), // sub_1406ccee0
    Replacement::draft(0x1406cd0b0, "contentbonsai_1406cd0b0", fn_1406cd0b0 as *const ()), // sub_1406cd0b0
    Replacement::draft_throws(0x1406cd250, "contentbonsai_1406cd250", fn_1406cd250 as *const (), Throws::Discard), // sub_1406cd250
    Replacement::draft(0x1406cd410, "contentbonsai_1406cd410", fn_1406cd410 as *const ()), // sub_1406cd410
    Replacement::draft_throws(0x1406cd520, "contentbonsai_1406cd520", fn_1406cd520 as *const (), Throws::Caught), // sub_1406cd520
    Replacement::draft_throws(0x1406cd7a0, "contentbonsai_1406cd7a0", fn_1406cd7a0 as *const (), Throws::Caught), // sub_1406cd7a0
    Replacement::draft_throws(0x1406cd9f0, "contentbonsai_1406cd9f0", fn_1406cd9f0 as *const (), Throws::Caught), // sub_1406cd9f0
    Replacement::draft(0x1406cdc30, "contentbonsai_1406cdc30", fn_1406cdc30 as *const ()), // sub_1406cdc30
    Replacement::draft(0x1406cde00, "contentbonsai_1406cde00", fn_1406cde00 as *const ()), // sub_1406cde00
    Replacement::draft(0x1406cdf60, "contentbonsai_1406cdf60", fn_1406cdf60 as *const ()), // sub_1406cdf60
    Replacement::draft_throws(0x1406ce0d0, "contentbonsai_1406ce0d0", fn_1406ce0d0 as *const (), Throws::Discard), // sub_1406ce0d0
    Replacement::draft_throws(0x1406ce210, "contentbonsai_1406ce210", fn_1406ce210 as *const (), Throws::Discard), // sub_1406ce210
    Replacement::draft(0x1406ce370, "contentbonsai_1406ce370", fn_1406ce370 as *const ()), // sub_1406ce370
    Replacement::draft(0x1406ce430, "contentbonsai_1406ce430", fn_1406ce430 as *const ()), // sub_1406ce430
    Replacement::draft(0x1406ce4e0, "contentbonsai_1406ce4e0", fn_1406ce4e0 as *const ()), // sub_1406ce4e0
    Replacement::draft(0x1406ce580, "contentbonsai_1406ce580", tpl_6896::<0x14387cbf0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406fe630, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406ce580
    Replacement::draft_throws(0x1406ce5f0, "contentbonsai_1406ce5f0", tpl_6897::<0x14387cb38, 0x14387cbf0, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1406db430, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406ce5f0
    Replacement::draft(0x1406ce660, "contentbonsai_1406ce660", tpl_6896::<0x14387cbf0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406fe890, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406ce660
    Replacement::draft_throws(0x1406ce6d0, "contentbonsai_1406ce6d0", tpl_6897::<0x14387cb38, 0x14387cbf0, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1406db670, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406ce6d0
    Replacement::draft_throws(0x1406ce740, "contentbonsai_1406ce740", fn_1406ce740 as *const (), Throws::Caught), // sub_1406ce740
    Replacement::draft_throws(0x1406ce800, "contentbonsai_1406ce800", fn_1406ce800 as *const (), Throws::Caught), // sub_1406ce800
    Replacement::draft_throws(0x1406ce8c0, "contentbonsai_1406ce8c0", fn_1406ce8c0 as *const (), Throws::Caught), // sub_1406ce8c0
    Replacement::draft(0x1406ce980, "contentbonsai_1406ce980", tpl_6896::<0x14387cbf0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406fe800, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406ce980
    Replacement::draft(0x1406ce9f0, "contentbonsai_1406ce9f0", tpl_6899::<0x14387cb38, 0x14387cbf0, 0x1401e9970, 0x1406e1480, 0x8, 0x1406fea30, 0x1401e9860> as *const ()), // sub_1406ce9f0
    Replacement::draft(0x1406cea50, "contentbonsai_1406cea50", tpl_6896::<0x14387cbf0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406feb00, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406cea50
    Replacement::draft_throws(0x1406ceac0, "contentbonsai_1406ceac0", tpl_6900::<0x14387cb38, 0x14387cbf0, 0x1401e9770, 0x1406e1570, 0x8, 0x1406febe0, 0x1401e9590> as *const (), Throws::Discard), // sub_1406ceac0
    Replacement::draft_throws(0x1406ceb20, "contentbonsai_1406ceb20", fn_1406ceb20 as *const (), Throws::Caught), // sub_1406ceb20
    Replacement::draft(0x1406cec30, "contentbonsai_1406cec30", fn_1406cec30 as *const ()), // sub_1406cec30
    Replacement::draft(0x1406cecd0, "contentbonsai_1406cecd0", fn_1406cecd0 as *const ()), // sub_1406cecd0
    Replacement::draft(0x1406cee00, "contentbonsai_1406cee00", fn_1406cee00 as *const ()), // sub_1406cee00
    Replacement::draft_throws(0x1406cf1e0, "contentbonsai_1406cf1e0", fn_1406cf1e0 as *const (), Throws::Caught), // sub_1406cf1e0
    Replacement::draft(0x1406cf6d0, "contentbonsai_1406cf6d0", fn_1406cf6d0 as *const ()), // sub_1406cf6d0
    Replacement::draft_throws(0x1406cf9b0, "contentbonsai_1406cf9b0", fn_1406cf9b0 as *const (), Throws::Discard), // sub_1406cf9b0
    Replacement::draft(0x1406cfb70, "contentbonsai_1406cfb70", fn_1406cfb70 as *const ()), // sub_1406cfb70
    Replacement::draft_throws(0x1406cfc60, "contentbonsai_1406cfc60", fn_1406cfc60 as *const (), Throws::Caught), // sub_1406cfc60
    Replacement::draft_throws(0x1406d0300, "contentbonsai_1406d0300", fn_1406d0300 as *const (), Throws::Caught), // sub_1406d0300
    Replacement::draft_throws(0x1406d08b0, "contentbonsai_1406d08b0", fn_1406d08b0 as *const (), Throws::Caught), // sub_1406d08b0
    Replacement::draft(0x1406d0ec0, "contentbonsai_1406d0ec0", fn_1406d0ec0 as *const ()), // sub_1406d0ec0
    Replacement::draft(0x1406d1500, "contentbonsai_1406d1500", fn_1406d1500 as *const ()), // sub_1406d1500
    Replacement::draft(0x1406d1720, "contentbonsai_1406d1720", fn_1406d1720 as *const ()), // sub_1406d1720
    Replacement::draft_throws(0x1406d1950, "contentbonsai_1406d1950", fn_1406d1950 as *const (), Throws::Discard), // sub_1406d1950
    Replacement::draft_throws(0x1406d1ba0, "contentbonsai_1406d1ba0", fn_1406d1ba0 as *const (), Throws::Discard), // sub_1406d1ba0
    Replacement::draft(0x1406d1ea0, "contentbonsai_1406d1ea0", fn_1406d1ea0 as *const ()), // sub_1406d1ea0
    Replacement::draft(0x1406d1f20, "contentbonsai_1406d1f20", fn_1406d1f20 as *const ()), // sub_1406d1f20
    Replacement::draft(0x1406d2010, "contentbonsai_1406d2010", fn_1406d2010 as *const ()), // sub_1406d2010
    Replacement::draft(0x1406d2110, "contentbonsai_1406d2110", fn_1406d2110 as *const ()), // sub_1406d2110
    Replacement::draft(0x1406d2210, "contentbonsai_1406d2210", fn_1406d2210 as *const ()), // sub_1406d2210
    Replacement::draft_throws(0x1406d2670, "contentbonsai_1406d2670", fn_1406d2670 as *const (), Throws::Caught), // sub_1406d2670
    Replacement::draft(0x1406d2a60, "contentbonsai_1406d2a60", fn_1406d2a60 as *const ()), // sub_1406d2a60
    Replacement::draft_throws(0x1406d2ca0, "contentbonsai_1406d2ca0", fn_1406d2ca0 as *const (), Throws::Discard), // sub_1406d2ca0
    Replacement::draft(0x1406d3130, "contentbonsai_1406d3130", fn_1406d3130 as *const ()), // sub_1406d3130
    Replacement::draft_throws(0x1406d32e0, "contentbonsai_1406d32e0", fn_1406d32e0 as *const (), Throws::Caught), // sub_1406d32e0
    Replacement::draft_throws(0x1406d3a90, "contentbonsai_1406d3a90", fn_1406d3a90 as *const (), Throws::Caught), // sub_1406d3a90
    Replacement::draft_throws(0x1406d4180, "contentbonsai_1406d4180", fn_1406d4180 as *const (), Throws::Caught), // sub_1406d4180
    Replacement::draft(0x1406d4870, "contentbonsai_1406d4870", fn_1406d4870 as *const ()), // sub_1406d4870
    Replacement::draft(0x1406d49d0, "contentbonsai_1406d49d0", fn_1406d49d0 as *const ()), // sub_1406d49d0
    Replacement::draft(0x1406d4c10, "contentbonsai_1406d4c10", fn_1406d4c10 as *const ()), // sub_1406d4c10
    Replacement::draft_throws(0x1406d4e60, "contentbonsai_1406d4e60", fn_1406d4e60 as *const (), Throws::Discard), // sub_1406d4e60
    Replacement::draft_throws(0x1406d5170, "contentbonsai_1406d5170", fn_1406d5170 as *const (), Throws::Discard), // sub_1406d5170
    Replacement::draft(0x1406d5750, "contentbonsai_1406d5750", fn_1406d5750 as *const ()), // sub_1406d5750
    Replacement::draft(0x1406d5a00, "contentbonsai_1406d5a00", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143247158, 0x8> as *const ()), // sub_1406d5a00
    Replacement::draft(0x1406d5a90, "contentbonsai_1406d5a90", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143247158, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1406d5a90
    Replacement::draft(0x1406d5b40, "contentbonsai_1406d5b40", fn_1406d5b40 as *const ()), // sub_1406d5b40
    Replacement::draft(0x1406d5c20, "contentbonsai_1406d5c20", fn_1406d5c20 as *const ()), // sub_1406d5c20
    Replacement::draft_throws(0x1406d5d10, "contentbonsai_1406d5d10", tpl_6937::<0x14387cbb0, 0x14387cba8, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1406dc540, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406d5d10
    Replacement::draft(0x1406d5d70, "contentbonsai_1406d5d70", fn_1406d5d70 as *const ()), // sub_1406d5d70
    Replacement::draft_throws(0x1406d5e30, "contentbonsai_1406d5e30", tpl_6937::<0x14387cbb0, 0x14387cba8, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1406dc6f0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406d5e30
    Replacement::draft(0x1406d5e90, "contentbonsai_1406d5e90", tpl_6939::<0x8, 0x1406dd200, 0x1406d3130> as *const ()), // sub_1406d5e90
    Replacement::draft_throws(0x1406d5f10, "contentbonsai_1406d5f10", fn_1406d5f10 as *const (), Throws::Caught), // sub_1406d5f10
    Replacement::draft_throws(0x1406d5fd0, "contentbonsai_1406d5fd0", fn_1406d5fd0 as *const (), Throws::Caught), // sub_1406d5fd0
    Replacement::draft_throws(0x1406d6090, "contentbonsai_1406d6090", fn_1406d6090 as *const (), Throws::Caught), // sub_1406d6090
    Replacement::draft(0x1406d6150, "contentbonsai_1406d6150", fn_1406d6150 as *const ()), // sub_1406d6150
    Replacement::draft(0x1406d6200, "contentbonsai_1406d6200", fn_1406d6200 as *const ()), // sub_1406d6200
    Replacement::draft(0x1406d62b0, "contentbonsai_1406d62b0", fn_1406d62b0 as *const ()), // sub_1406d62b0
    Replacement::draft_throws(0x1406d6370, "contentbonsai_1406d6370", fn_1406d6370 as *const (), Throws::Discard), // sub_1406d6370
    Replacement::draft_throws(0x1406d6410, "contentbonsai_1406d6410", tpl_6941::<0x14387cba8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x143245388, 0x8, 0x143245358, 0x1406dc8e0> as *const (), Throws::Discard), // sub_1406d6410
    Replacement::draft(0x1406d64f0, "contentbonsai_1406d64f0", tpl_6942::<0x1401de830, 0x18, 0x143084e28, 0x141834d30> as *const ()), // contentbonsai::NavmeshVolumeComponent::contentbonsai::NavmeshVolumeComponent::vfunc_0
    Replacement::draft(0x1406d6540, "unknown_1406d6540", tpl_1707::<0x8, 0xc, 0x140701ce0, 0x14183acc8> as *const ()), // sub_1406d6540
    Replacement::draft(0x1406d6620, "unknown_1406d6620", tpl_6943::<0x140701e10, 0x14183acc8> as *const ()), // sub_1406d6620
    Replacement::draft(0x1406d6720, "unknown_1406d6720", fn_1406d6720 as *const ()), // sub_1406d6720
    Replacement::draft(0x1406d6850, "content_1406d6850", tpl_1953::<0x1401de830, 0x143084e28, 0x141834d30> as *const ()), // content::TagComponent::content::TagComponent::vfunc_0
    Replacement::draft(0x1406d68a0, "content_1406d68a0", fn_1406d68a0 as *const ()), // sub_1406d68a0
    Replacement::draft(0x1406d6990, "content_1406d6990", fn_1406d6990 as *const ()), // content::OverrideSpeedDecorator::content::OverrideSpeedDecorator::vfunc_0
    Replacement::draft(0x1406d69f0, "content_1406d69f0", tpl_6947::<0x1401ffd50, 0x141834d30> as *const ()), // content::BehaviorTreeDecorator::content::BehaviorTreeDecorator::vfunc_0
    Replacement::draft_throws(0x1406d6a30, "unknown_1406d6a30", fn_1406d6a30 as *const (), Throws::Caught), // sub_1406d6a30
    Replacement::draft_throws(0x1406d6ba0, "unknown_1406d6ba0", fn_1406d6ba0 as *const (), Throws::Caught), // sub_1406d6ba0
    Replacement::draft_throws(0x1406d6d30, "unknown_1406d6d30", fn_1406d6d30 as *const (), Throws::Discard), // sub_1406d6d30
    Replacement::draft(0x1406d6d80, "unknown_1406d6d80", tpl_4781::<0x8, 0x10, 0x140685020, 0x18, 0x30, 0x38, 0x1401e5730> as *const ()), // sub_1406d6d80
    Replacement::draft_throws(0x1406d6df0, "unknown_1406d6df0", fn_1406d6df0 as *const (), Throws::Caught), // sub_1406d6df0
    Replacement::draft_throws(0x1406d6fb0, "unknown_1406d6fb0", fn_1406d6fb0 as *const (), Throws::Caught), // sub_1406d6fb0
    Replacement::draft(0x1406d70a0, "unknown_1406d70a0", fn_1406d70a0 as *const ()), // sub_1406d70a0
    Replacement::draft_throws(0x1406d7200, "unknown_1406d7200", fn_1406d7200 as *const (), Throws::Caught), // sub_1406d7200
    Replacement::draft_throws(0x1406d7330, "unknown_1406d7330", fn_1406d7330 as *const (), Throws::Caught), // sub_1406d7330
    Replacement::draft_throws(0x1406d7460, "unknown_1406d7460", fn_1406d7460 as *const (), Throws::Caught), // sub_1406d7460
    Replacement::draft_throws(0x1406d7870, "unknown_1406d7870", fn_1406d7870 as *const (), Throws::Caught), // sub_1406d7870
    Replacement::draft(0x1406d7950, "unknown_1406d7950", tpl_365::<0x1406f3580> as *const ()), // sub_1406d7950
    Replacement::draft(0x1406d7960, "unknown_1406d7960", fn_1406d7960 as *const ()), // sub_1406d7960
    Replacement::draft(0x1406d7a10, "unknown_1406d7a10", fn_1406d7a10 as *const ()), // sub_1406d7a10
    Replacement::draft(0x1406d7a90, "unknown_1406d7a90", fn_1406d7a90 as *const ()), // sub_1406d7a90
    Replacement::draft(0x1406d7b60, "unknown_1406d7b60", fn_1406d7b60 as *const ()), // sub_1406d7b60
    Replacement::draft_throws(0x1406d7c10, "unknown_1406d7c10", tpl_838::<0x1406e16e0> as *const (), Throws::Discard), // sub_1406d7c10
    Replacement::draft_throws(0x1406d7c20, "unknown_1406d7c20", tpl_838::<0x1406e19a0> as *const (), Throws::Discard), // sub_1406d7c20
    Replacement::draft_throws(0x1406d7c30, "unknown_1406d7c30", tpl_838::<0x1406e1c30> as *const (), Throws::Discard), // sub_1406d7c30
    Replacement::draft_throws(0x1406d7c40, "unknown_1406d7c40", tpl_2502::<0x20, 0x8, 0x1402da920> as *const (), Throws::Discard), // sub_1406d7c40
    Replacement::draft(0x1406d7c90, "unknown_1406d7c90", fn_1406d7c90 as *const ()), // sub_1406d7c90
    Replacement::draft(0x1406d7d00, "unknown_1406d7d00", fn_1406d7d00 as *const ()), // sub_1406d7d00
    Replacement::draft(0x1406d7da0, "unknown_1406d7da0", fn_1406d7da0 as *const ()), // sub_1406d7da0
    Replacement::draft_throws(0x1406d7e40, "unknown_1406d7e40", tpl_838::<0x1406e2780> as *const (), Throws::Discard), // sub_1406d7e40
    Replacement::draft_throws(0x1406d7e50, "unknown_1406d7e50", tpl_838::<0x1406e2c30> as *const (), Throws::Discard), // sub_1406d7e50
    Replacement::draft_throws(0x1406d7e60, "unknown_1406d7e60", tpl_838::<0x1406e30b0> as *const (), Throws::Discard), // sub_1406d7e60
    Replacement::draft_throws(0x1406d7e70, "unknown_1406d7e70", tpl_6961::<0x20, 0x8, 0x1406e3520> as *const (), Throws::Discard), // sub_1406d7e70
    Replacement::draft_throws(0x1406d7ec0, "unknown_1406d7ec0", tpl_838::<0x1406e4960> as *const (), Throws::Discard), // sub_1406d7ec0
    Replacement::draft_throws(0x1406d7ed0, "unknown_1406d7ed0", fn_1406d7ed0 as *const (), Throws::Caught), // sub_1406d7ed0
    Replacement::draft_throws(0x1406d86b0, "unknown_1406d86b0", fn_1406d86b0 as *const (), Throws::Caught), // sub_1406d86b0
    Replacement::draft_throws(0x1406d8d90, "unknown_1406d8d90", fn_1406d8d90 as *const (), Throws::Caught), // sub_1406d8d90
    Replacement::draft(0x1406d93e0, "unknown_1406d93e0", fn_1406d93e0 as *const ()), // sub_1406d93e0
    Replacement::draft(0x1406d94b0, "unknown_1406d94b0", tpl_6966::<0x1406dce20, 0x14183acbc> as *const ()), // sub_1406d94b0
    Replacement::draft_throws(0x1406d9560, "unknown_1406d9560", tpl_838::<0x1406e4ad0> as *const (), Throws::Discard), // sub_1406d9560
    Replacement::draft_throws(0x1406d9570, "unknown_1406d9570", tpl_838::<0x1406e4dc0> as *const (), Throws::Discard), // sub_1406d9570
    Replacement::draft_throws(0x1406d9580, "unknown_1406d9580", tpl_838::<0x1406e5080> as *const (), Throws::Discard), // sub_1406d9580
    Replacement::draft_throws(0x1406d95e0, "unknown_1406d95e0", tpl_838::<0x1406e5ef0> as *const (), Throws::Discard), // sub_1406d95e0
    Replacement::draft_throws(0x1406d95f0, "unknown_1406d95f0", tpl_838::<0x1406e7b00> as *const (), Throws::Discard), // sub_1406d95f0
    Replacement::draft_throws(0x1406d9600, "unknown_1406d9600", tpl_838::<0x1406e9440> as *const (), Throws::Discard), // sub_1406d9600
    Replacement::draft_throws(0x1406d9610, "unknown_1406d9610", tpl_6967::<0x20, 0x8, 0x1406ea2d0> as *const (), Throws::Discard), // sub_1406d9610
    Replacement::draft(0x1406d9660, "unknown_1406d9660", tpl_365::<0x1406fc3a0> as *const ()), // sub_1406d9660
    Replacement::draft(0x1406d9670, "unknown_1406d9670", tpl_365::<0x1406fc440> as *const ()), // sub_1406d9670
    Replacement::draft(0x1406d9680, "unknown_1406d9680", tpl_365::<0x1406fc4e0> as *const ()), // sub_1406d9680
    Replacement::draft(0x1406d9690, "unknown_1406d9690", tpl_365::<0x1406fc580> as *const ()), // sub_1406d9690
    Replacement::draft_throws(0x1406d96a0, "unknown_1406d96a0", fn_1406d96a0 as *const (), Throws::Caught), // sub_1406d96a0
    Replacement::draft_throws(0x1406d97d0, "unknown_1406d97d0", fn_1406d97d0 as *const (), Throws::Caught), // sub_1406d97d0
    Replacement::draft_throws(0x1406d9900, "unknown_1406d9900", fn_1406d9900 as *const (), Throws::Caught), // sub_1406d9900
    Replacement::draft_throws(0x1406d9c00, "unknown_1406d9c00", fn_1406d9c00 as *const (), Throws::Caught), // sub_1406d9c00
    Replacement::draft_throws(0x1406d9d40, "unknown_1406d9d40", fn_1406d9d40 as *const (), Throws::Caught), // sub_1406d9d40
    Replacement::draft_throws(0x1406da040, "unknown_1406da040", fn_1406da040 as *const (), Throws::Caught), // sub_1406da040
    Replacement::draft_throws(0x1406da180, "unknown_1406da180", fn_1406da180 as *const (), Throws::Caught), // sub_1406da180
    Replacement::draft_throws(0x1406da480, "unknown_1406da480", fn_1406da480 as *const (), Throws::Caught), // sub_1406da480
    Replacement::draft_throws(0x1406da5c0, "unknown_1406da5c0", fn_1406da5c0 as *const (), Throws::Caught), // sub_1406da5c0
    Replacement::draft(0x1406da6c0, "unknown_1406da6c0", fn_1406da6c0 as *const ()), // sub_1406da6c0
    Replacement::draft(0x1406da760, "unknown_1406da760", fn_1406da760 as *const ()), // sub_1406da760
    Replacement::draft(0x1406da7e0, "unknown_1406da7e0", fn_1406da7e0 as *const ()), // sub_1406da7e0
    Replacement::draft(0x1406daa00, "unknown_1406daa00", tpl_66008::<0x338, 0x48, 0x1406dcfb0, 0x1404b9a10> as *const ()), // sub_1406daa00
    Replacement::draft(0x1406daaa0, "unknown_1406daaa0", fn_1406daaa0 as *const ()), // sub_1406daaa0
    Replacement::draft(0x1406dab80, "unknown_1406dab80", fn_1406dab80 as *const ()), // sub_1406dab80
    Replacement::draft_throws(0x1406dac10, "unknown_1406dac10", fn_1406dac10 as *const (), Throws::Caught), // sub_1406dac10
    Replacement::draft_throws(0x1406dad10, "unknown_1406dad10", tpl_838::<0x1406ecd10> as *const (), Throws::Discard), // sub_1406dad10
    Replacement::draft_throws(0x1406dad20, "unknown_1406dad20", fn_1406dad20 as *const (), Throws::Caught), // sub_1406dad20
    Replacement::draft_throws(0x1406dae20, "unknown_1406dae20", tpl_838::<0x1406ed080> as *const (), Throws::Discard), // sub_1406dae20
    Replacement::draft_throws(0x1406dae30, "unknown_1406dae30", fn_1406dae30 as *const (), Throws::Caught), // sub_1406dae30
    Replacement::draft_throws(0x1406db010, "unknown_1406db010", tpl_838::<0x1406ed3a0> as *const (), Throws::Discard), // sub_1406db010
    Replacement::draft(0x1406db020, "unknown_1406db020", fn_1406db020 as *const ()), // sub_1406db020
    Replacement::draft_throws(0x1406db0a0, "unknown_1406db0a0", fn_1406db0a0 as *const (), Throws::Caught), // sub_1406db0a0
    Replacement::draft_throws(0x1406db1d0, "unknown_1406db1d0", tpl_6980::<0x20, 0x8, 0x1406ed7a0> as *const (), Throws::Discard), // sub_1406db1d0
    Replacement::draft(0x1406db220, "unknown_1406db220", fn_1406db220 as *const ()), // sub_1406db220
    Replacement::draft(0x1406db330, "unknown_1406db330", fn_1406db330 as *const ()), // sub_1406db330
    Replacement::draft(0x1406db3c0, "unknown_1406db3c0", fn_1406db3c0 as *const ()), // sub_1406db3c0
    Replacement::draft_throws(0x1406db430, "unknown_1406db430", fn_1406db430 as *const (), Throws::Caught), // sub_1406db430
    Replacement::draft_throws(0x1406db670, "unknown_1406db670", fn_1406db670 as *const (), Throws::Caught), // sub_1406db670
    Replacement::draft_throws(0x1406db8b0, "unknown_1406db8b0", fn_1406db8b0 as *const (), Throws::Caught), // sub_1406db8b0
    Replacement::draft_throws(0x1406dbb00, "unknown_1406dbb00", fn_1406dbb00 as *const (), Throws::Caught), // sub_1406dbb00
    Replacement::draft_throws(0x1406dbd50, "unknown_1406dbd50", fn_1406dbd50 as *const (), Throws::Caught), // sub_1406dbd50
    Replacement::draft_throws(0x1406dbfa0, "unknown_1406dbfa0", tpl_838::<0x1406ef230> as *const (), Throws::Discard), // sub_1406dbfa0
    Replacement::draft_throws(0x1406dbfb0, "unknown_1406dbfb0", tpl_838::<0x1406ef4f0> as *const (), Throws::Discard), // sub_1406dbfb0
    Replacement::draft_throws(0x1406dbfc0, "unknown_1406dbfc0", tpl_838::<0x1406efaa0> as *const (), Throws::Discard), // sub_1406dbfc0
    Replacement::draft_throws(0x1406dbfd0, "unknown_1406dbfd0", tpl_838::<0x1406efd30> as *const (), Throws::Discard), // sub_1406dbfd0
    Replacement::draft_throws(0x1406dbfe0, "unknown_1406dbfe0", tpl_838::<0x1406f02b0> as *const (), Throws::Discard), // sub_1406dbfe0
    Replacement::draft_throws(0x1406dbff0, "unknown_1406dbff0", tpl_838::<0x1406f0540> as *const (), Throws::Discard), // sub_1406dbff0
    Replacement::draft_throws(0x1406dc000, "unknown_1406dc000", tpl_2502::<0x20, 0x8, 0x1406f0ab0> as *const (), Throws::Discard), // sub_1406dc000
    Replacement::draft_throws(0x1406dc050, "unknown_1406dc050", tpl_6985::<0x20, 0x8, 0x1406f0c00> as *const (), Throws::Discard), // sub_1406dc050
    Replacement::draft_throws(0x1406dc0a0, "unknown_1406dc0a0", fn_1406dc0a0 as *const (), Throws::Caught), // sub_1406dc0a0
    Replacement::draft_throws(0x1406dc1a0, "unknown_1406dc1a0", fn_1406dc1a0 as *const (), Throws::Caught), // sub_1406dc1a0
    Replacement::draft_throws(0x1406dc2a0, "unknown_1406dc2a0", fn_1406dc2a0 as *const (), Throws::Caught), // sub_1406dc2a0
    Replacement::draft(0x1406dc480, "unknown_1406dc480", fn_1406dc480 as *const ()), // sub_1406dc480
    Replacement::draft_throws(0x1406dc540, "unknown_1406dc540", fn_1406dc540 as *const (), Throws::Caught), // sub_1406dc540
    Replacement::draft_throws(0x1406dc6f0, "unknown_1406dc6f0", fn_1406dc6f0 as *const (), Throws::Caught), // sub_1406dc6f0
    Replacement::draft_throws(0x1406dc8b0, "unknown_1406dc8b0", tpl_838::<0x1406f0cd0> as *const (), Throws::Discard), // sub_1406dc8b0
    Replacement::draft_throws(0x1406dc8c0, "unknown_1406dc8c0", tpl_838::<0x1406f0fb0> as *const (), Throws::Discard), // sub_1406dc8c0
    Replacement::draft_throws(0x1406dc8d0, "unknown_1406dc8d0", tpl_838::<0x1406f1260> as *const (), Throws::Discard), // sub_1406dc8d0
    Replacement::draft_throws(0x1406dc8e0, "unknown_1406dc8e0", fn_1406dc8e0 as *const (), Throws::Discard), // sub_1406dc8e0
    Replacement::draft(0x1406dc930, "unknown_1406dc930", fn_1406dc930 as *const ()), // sub_1406dc930
    Replacement::draft(0x1406dcbc0, "unknown_1406dcbc0", fn_1406dcbc0 as *const ()), // sub_1406dcbc0
    Replacement::draft(0x1406dcc50, "unknown_1406dcc50", fn_1406dcc50 as *const ()), // sub_1406dcc50
    Replacement::draft(0x1406dcd10, "unknown_1406dcd10", fn_1406dcd10 as *const ()), // sub_1406dcd10
    Replacement::draft(0x1406dce20, "unknown_1406dce20", tpl_66024::<0x8, 0xc, 0x1401deba0> as *const ()), // sub_1406dce20
    Replacement::draft(0x1406dcee0, "unknown_1406dcee0", fn_1406dcee0 as *const ()), // sub_1406dcee0
    Replacement::draft(0x1406dcfb0, "unknown_1406dcfb0", tpl_66026::<0x140701c30> as *const ()), // sub_1406dcfb0
    Replacement::draft(0x1406dd080, "unknown_1406dd080", tpl_62153::<0x8, 0xc, 0x140701ce0, 0x14183acc2> as *const ()), // sub_1406dd080
    Replacement::draft(0x1406dd130, "unknown_1406dd130", tpl_66027::<0x8, 0xc, 0x140701e10, 0x1406db3c0> as *const ()), // sub_1406dd130
    Replacement::draft(0x1406dd200, "unknown_1406dd200", fn_1406dd200 as *const ()), // sub_1406dd200
    Replacement::draft(0x1406dd2c0, "unknown_1406dd2c0", tpl_6995::<0x1426d4580, 0x1406f1d80, 0x1406f1ce0, 0x1401ee7c0> as *const ()), // sub_1406dd2c0
    Replacement::draft_throws(0x1406dd430, "unknown_1406dd430", fn_1406dd430 as *const (), Throws::Caught), // sub_1406dd430
    Replacement::draft_throws(0x1406ddcf0, "unknown_1406ddcf0", fn_1406ddcf0 as *const (), Throws::Caught), // sub_1406ddcf0
    Replacement::draft_throws(0x1406de490, "unknown_1406de490", fn_1406de490 as *const (), Throws::Caught), // sub_1406de490
    Replacement::draft(0x1406deba0, "unknown_1406deba0", fn_1406deba0 as *const ()), // sub_1406deba0
    Replacement::draft_throws(0x1406decd0, "unknown_1406decd0", fn_1406decd0 as *const (), Throws::Caught), // sub_1406decd0
    Replacement::draft_throws(0x1406df4a0, "unknown_1406df4a0", fn_1406df4a0 as *const (), Throws::Caught), // sub_1406df4a0
    Replacement::draft_throws(0x1406dfb70, "unknown_1406dfb70", fn_1406dfb70 as *const (), Throws::Caught), // sub_1406dfb70
    Replacement::draft_throws(0x1406e02b0, "unknown_1406e02b0", tpl_7004::<0x14387d9d0, 0x14387d9c8, 0x1401e9fa0, 0x318, 0x328, 0x320, 0x8, 0x38, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_1406e02b0
    Replacement::draft_throws(0x1406e03a0, "unknown_1406e03a0", tpl_7004::<0x14387d9d0, 0x14387d9c8, 0x1401e9b60, 0x318, 0x328, 0x320, 0x8, 0x38, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406e03a0
    Replacement::draft_throws(0x1406e0490, "unknown_1406e0490", fn_1406e0490 as *const (), Throws::Caught), // sub_1406e0490
    Replacement::draft_throws(0x1406e0520, "unknown_1406e0520", fn_1406e0520 as *const (), Throws::Caught), // sub_1406e0520
    Replacement::draft_throws(0x1406e0ad0, "unknown_1406e0ad0", fn_1406e0ad0 as *const (), Throws::Caught), // sub_1406e0ad0
    Replacement::draft_throws(0x1406e1040, "unknown_1406e1040", fn_1406e1040 as *const (), Throws::Caught), // sub_1406e1040
    Replacement::draft(0x1406e1480, "unknown_1406e1480", fn_1406e1480 as *const ()), // sub_1406e1480
    Replacement::draft_throws(0x1406e1570, "unknown_1406e1570", tpl_7004::<0x14387d9d0, 0x14387d9c8, 0x1401e9770, 0x318, 0x328, 0x320, 0x8, 0x38, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_1406e1570
    Replacement::draft_throws(0x1406e1660, "unknown_1406e1660", fn_1406e1660 as *const (), Throws::Discard), // sub_1406e1660
    Replacement::draft_throws(0x1406e16e0, "unknown_1406e16e0", fn_1406e16e0 as *const (), Throws::Caught), // sub_1406e16e0
    Replacement::draft_throws(0x1406e19a0, "unknown_1406e19a0", fn_1406e19a0 as *const (), Throws::Caught), // sub_1406e19a0
    Replacement::draft_throws(0x1406e1c30, "unknown_1406e1c30", fn_1406e1c30 as *const (), Throws::Caught), // sub_1406e1c30
    Replacement::draft_throws(0x1406e1ec0, "unknown_1406e1ec0", fn_1406e1ec0 as *const (), Throws::Caught), // sub_1406e1ec0
    Replacement::draft_throws(0x1406e21b0, "unknown_1406e21b0", fn_1406e21b0 as *const (), Throws::Caught), // sub_1406e21b0
    Replacement::draft(0x1406e24a0, "unknown_1406e24a0", fn_1406e24a0 as *const ()), // sub_1406e24a0
    Replacement::draft_throws(0x1406e2780, "unknown_1406e2780", fn_1406e2780 as *const (), Throws::Caught), // sub_1406e2780
    Replacement::draft_throws(0x1406e2c30, "unknown_1406e2c30", fn_1406e2c30 as *const (), Throws::Caught), // sub_1406e2c30
    Replacement::draft_throws(0x1406e30b0, "unknown_1406e30b0", fn_1406e30b0 as *const (), Throws::Caught), // sub_1406e30b0
    Replacement::draft_throws(0x1406e3520, "unknown_1406e3520", fn_1406e3520 as *const (), Throws::Caught), // sub_1406e3520
    Replacement::draft(0x1406e35f0, "unknown_1406e35f0", tpl_7013::<0x1407030c0> as *const ()), // sub_1406e35f0
    Replacement::draft(0x1406e3600, "unknown_1406e3600", fn_1406e3600 as *const ()), // sub_1406e3600
    Replacement::draft_throws(0x1406e3680, "unknown_1406e3680", fn_1406e3680 as *const (), Throws::Caught), // sub_1406e3680
    Replacement::draft_throws(0x1406e3d00, "unknown_1406e3d00", fn_1406e3d00 as *const (), Throws::Caught), // sub_1406e3d00
    Replacement::draft_throws(0x1406e4330, "unknown_1406e4330", fn_1406e4330 as *const (), Throws::Caught), // sub_1406e4330
    Replacement::draft_throws(0x1406e4960, "unknown_1406e4960", fn_1406e4960 as *const (), Throws::Caught), // sub_1406e4960
    Replacement::draft_throws(0x1406e4ad0, "unknown_1406e4ad0", fn_1406e4ad0 as *const (), Throws::Caught), // sub_1406e4ad0
    Replacement::draft_throws(0x1406e4dc0, "unknown_1406e4dc0", fn_1406e4dc0 as *const (), Throws::Caught), // sub_1406e4dc0
    Replacement::draft_throws(0x1406e5080, "unknown_1406e5080", fn_1406e5080 as *const (), Throws::Caught), // sub_1406e5080
    Replacement::draft_throws(0x1406e5330, "unknown_1406e5330", fn_1406e5330 as *const (), Throws::Caught), // sub_1406e5330
    Replacement::draft_throws(0x1406e53f0, "unknown_1406e53f0", fn_1406e53f0 as *const (), Throws::Caught), // sub_1406e53f0
    Replacement::draft_throws(0x1406e5970, "unknown_1406e5970", fn_1406e5970 as *const (), Throws::Caught), // sub_1406e5970
    Replacement::draft_throws(0x1406e5ef0, "unknown_1406e5ef0", fn_1406e5ef0 as *const (), Throws::Caught), // sub_1406e5ef0
    Replacement::draft_throws(0x1406e6560, "unknown_1406e6560", fn_1406e6560 as *const (), Throws::Caught), // sub_1406e6560
    Replacement::draft_throws(0x1406e6ae0, "unknown_1406e6ae0", fn_1406e6ae0 as *const (), Throws::Caught), // sub_1406e6ae0
    Replacement::draft_throws(0x1406e7060, "unknown_1406e7060", fn_1406e7060 as *const (), Throws::Caught), // sub_1406e7060
    Replacement::draft_throws(0x1406e75b0, "unknown_1406e75b0", fn_1406e75b0 as *const (), Throws::Caught), // sub_1406e75b0
    Replacement::draft_throws(0x1406e7b00, "unknown_1406e7b00", fn_1406e7b00 as *const (), Throws::Caught), // sub_1406e7b00
    Replacement::draft_throws(0x1406e8120, "unknown_1406e8120", fn_1406e8120 as *const (), Throws::Caught), // sub_1406e8120
    Replacement::draft_throws(0x1406e8670, "unknown_1406e8670", fn_1406e8670 as *const (), Throws::Caught), // sub_1406e8670
    Replacement::draft_throws(0x1406e8bc0, "unknown_1406e8bc0", fn_1406e8bc0 as *const (), Throws::Caught), // sub_1406e8bc0
    Replacement::draft_throws(0x1406e9000, "unknown_1406e9000", fn_1406e9000 as *const (), Throws::Caught), // sub_1406e9000
    Replacement::draft_throws(0x1406e9440, "unknown_1406e9440", fn_1406e9440 as *const (), Throws::Caught), // sub_1406e9440
    Replacement::draft_throws(0x1406e9a50, "unknown_1406e9a50", fn_1406e9a50 as *const (), Throws::Caught), // sub_1406e9a50
    Replacement::draft_throws(0x1406e9e90, "unknown_1406e9e90", fn_1406e9e90 as *const (), Throws::Caught), // sub_1406e9e90
    Replacement::draft_throws(0x1406ea2d0, "unknown_1406ea2d0", fn_1406ea2d0 as *const (), Throws::Caught), // sub_1406ea2d0
    Replacement::draft(0x1406ea3a0, "unknown_1406ea3a0", fn_1406ea3a0 as *const ()), // sub_1406ea3a0
    Replacement::draft(0x1406ea470, "unknown_1406ea470", fn_1406ea470 as *const ()), // sub_1406ea470
    Replacement::draft_throws(0x1406ea600, "unknown_1406ea600", fn_1406ea600 as *const (), Throws::Caught), // sub_1406ea600
    Replacement::draft_throws(0x1406ea780, "unknown_1406ea780", fn_1406ea780 as *const (), Throws::Discard), // sub_1406ea780
    Replacement::draft_throws(0x1406ea8d0, "unknown_1406ea8d0", fn_1406ea8d0 as *const (), Throws::Discard), // sub_1406ea8d0
    Replacement::draft_throws(0x1406ea9c0, "unknown_1406ea9c0", fn_1406ea9c0 as *const (), Throws::Discard), // sub_1406ea9c0
    Replacement::draft_throws(0x1406eaa60, "unknown_1406eaa60", fn_1406eaa60 as *const (), Throws::Caught), // sub_1406eaa60
    Replacement::draft_throws(0x1406eadc0, "unknown_1406eadc0", fn_1406eadc0 as *const (), Throws::Caught), // sub_1406eadc0
    Replacement::draft_throws(0x1406eb370, "unknown_1406eb370", fn_1406eb370 as *const (), Throws::Caught), // sub_1406eb370
    Replacement::draft_throws(0x1406eb670, "unknown_1406eb670", fn_1406eb670 as *const (), Throws::Caught), // sub_1406eb670
    Replacement::draft_throws(0x1406ebbe0, "unknown_1406ebbe0", fn_1406ebbe0 as *const (), Throws::Caught), // sub_1406ebbe0
    Replacement::draft_throws(0x1406ebed0, "unknown_1406ebed0", fn_1406ebed0 as *const (), Throws::Caught), // sub_1406ebed0
    Replacement::draft(0x1406ec310, "unknown_1406ec310", fn_1406ec310 as *const ()), // sub_1406ec310
    Replacement::draft(0x1406ec510, "unknown_1406ec510", fn_1406ec510 as *const ()), // sub_1406ec510
    Replacement::draft(0x1406ec600, "unknown_1406ec600", fn_1406ec600 as *const ()), // sub_1406ec600
    Replacement::draft_throws(0x1406ec700, "unknown_1406ec700", fn_1406ec700 as *const (), Throws::Discard), // sub_1406ec700
    Replacement::draft_throws(0x1406ec7e0, "unknown_1406ec7e0", fn_1406ec7e0 as *const (), Throws::Discard), // sub_1406ec7e0
    Replacement::draft(0x1406ec8f0, "unknown_1406ec8f0", fn_1406ec8f0 as *const ()), // sub_1406ec8f0
    Replacement::draft(0x1406ec9b0, "unknown_1406ec9b0", fn_1406ec9b0 as *const ()), // sub_1406ec9b0
    Replacement::draft(0x1406eca20, "unknown_1406eca20", fn_1406eca20 as *const ()), // sub_1406eca20
    Replacement::draft(0x1406ecbf0, "unknown_1406ecbf0", fn_1406ecbf0 as *const ()), // sub_1406ecbf0
    Replacement::draft_throws(0x1406ecd10, "unknown_1406ecd10", fn_1406ecd10 as *const (), Throws::Caught), // sub_1406ecd10
    Replacement::draft_throws(0x1406ed080, "unknown_1406ed080", fn_1406ed080 as *const (), Throws::Caught), // sub_1406ed080
    Replacement::draft_throws(0x1406ed3a0, "unknown_1406ed3a0", fn_1406ed3a0 as *const (), Throws::Caught), // sub_1406ed3a0
    Replacement::draft(0x1406ed6a0, "unknown_1406ed6a0", fn_1406ed6a0 as *const ()), // sub_1406ed6a0
    Replacement::draft_throws(0x1406ed7a0, "unknown_1406ed7a0", fn_1406ed7a0 as *const (), Throws::Caught), // sub_1406ed7a0
    Replacement::draft_throws(0x1406ed890, "unknown_1406ed890", fn_1406ed890 as *const (), Throws::Caught), // sub_1406ed890
    Replacement::draft_throws(0x1406edf40, "unknown_1406edf40", fn_1406edf40 as *const (), Throws::Caught), // sub_1406edf40
    Replacement::draft_throws(0x1406ee5b0, "unknown_1406ee5b0", fn_1406ee5b0 as *const (), Throws::Caught), // sub_1406ee5b0
    Replacement::draft_throws(0x1406ee9f0, "unknown_1406ee9f0", fn_1406ee9f0 as *const (), Throws::Caught), // sub_1406ee9f0
    Replacement::draft_throws(0x1406eee10, "unknown_1406eee10", fn_1406eee10 as *const (), Throws::Caught), // sub_1406eee10
    Replacement::draft_throws(0x1406ef230, "unknown_1406ef230", fn_1406ef230 as *const (), Throws::Caught), // sub_1406ef230
    Replacement::draft_throws(0x1406ef4f0, "unknown_1406ef4f0", fn_1406ef4f0 as *const (), Throws::Caught), // sub_1406ef4f0
    Replacement::draft_throws(0x1406efaa0, "unknown_1406efaa0", fn_1406efaa0 as *const (), Throws::Caught), // sub_1406efaa0
    Replacement::draft_throws(0x1406efd30, "unknown_1406efd30", fn_1406efd30 as *const (), Throws::Caught), // sub_1406efd30
    Replacement::draft_throws(0x1406f02b0, "unknown_1406f02b0", fn_1406f02b0 as *const (), Throws::Caught), // sub_1406f02b0
    Replacement::draft_throws(0x1406f0540, "unknown_1406f0540", fn_1406f0540 as *const (), Throws::Caught), // sub_1406f0540
    Replacement::draft_throws(0x1406f0ab0, "unknown_1406f0ab0", fn_1406f0ab0 as *const (), Throws::Caught), // sub_1406f0ab0
    Replacement::draft_throws(0x1406f0c00, "unknown_1406f0c00", fn_1406f0c00 as *const (), Throws::Caught), // sub_1406f0c00
    Replacement::draft_throws(0x1406f0cd0, "unknown_1406f0cd0", fn_1406f0cd0 as *const (), Throws::Caught), // sub_1406f0cd0
    Replacement::draft_throws(0x1406f0fb0, "unknown_1406f0fb0", fn_1406f0fb0 as *const (), Throws::Caught), // sub_1406f0fb0
    Replacement::draft_throws(0x1406f1260, "unknown_1406f1260", fn_1406f1260 as *const (), Throws::Caught), // sub_1406f1260
    Replacement::draft_throws(0x1406f1500, "unknown_1406f1500", fn_1406f1500 as *const (), Throws::Caught), // sub_1406f1500
    Replacement::draft(0x1406f15d0, "unknown_1406f15d0", fn_1406f15d0 as *const ()), // sub_1406f15d0
    Replacement::draft(0x1406f17f0, "unknown_1406f17f0", fn_1406f17f0 as *const ()), // sub_1406f17f0
    Replacement::draft(0x1406f1a80, "unknown_1406f1a80", fn_1406f1a80 as *const ()), // sub_1406f1a80
    Replacement::draft(0x1406f1c60, "unknown_1406f1c60", fn_1406f1c60 as *const ()), // sub_1406f1c60
    Replacement::draft(0x1406f1ca0, "unknown_1406f1ca0", fn_1406f1ca0 as *const ()), // sub_1406f1ca0
    Replacement::draft(0x1406f1ce0, "unknown_1406f1ce0", tpl_7074::<0x143c89ed8, 0x143c89f18, 0x141834a78, 0x1425faf40, 0x143c89f1c, 0x143c89f20, 0x143c89f1b, 0x141834d18, 0x141834a0c> as *const ()), // sub_1406f1ce0
    Replacement::draft(0x1406f1d80, "unknown_1406f1d80", tpl_7075::<0x143c89f10, 0x143c89ee0, 0x141834a78, 0x143c89ee3, 0x143c89ee4, 0x143c89ee8, 0x143243df8, 0x142409b50, 0x1425faf30, 0x141834d18, 0x141834a0c> as *const ()), // sub_1406f1d80
    Replacement::draft(0x1406f1e50, "unknown_1406f1e50", fn_1406f1e50 as *const ()), // sub_1406f1e50
    Replacement::draft(0x1406f1f30, "unknown_1406f1f30", fn_1406f1f30 as *const ()), // sub_1406f1f30
    Replacement::draft(0x1406f2130, "unknown_1406f2130", fn_1406f2130 as *const ()), // sub_1406f2130
    Replacement::draft(0x1406f2210, "unknown_1406f2210", fn_1406f2210 as *const ()), // sub_1406f2210
    Replacement::draft_throws(0x1406f2520, "unknown_1406f2520", tpl_7080::<0x1401ffe00, 0x140708f60, 0x140708e80, 0x142442de0, 0x143a399b8, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_1406f2520
    Replacement::draft(0x1406f27c0, "unknown_1406f27c0", tpl_7081::<0x140701c30, 0x1404b9990, 0x1404b9a10, 0x142417470> as *const ()), // sub_1406f27c0
    Replacement::draft(0x1406f29a0, "unknown_1406f29a0", fn_1406f29a0 as *const ()), // sub_1406f29a0
    Replacement::draft_throws(0x1406f2b60, "unknown_1406f2b60", fn_1406f2b60 as *const (), Throws::Caught), // sub_1406f2b60
    Replacement::draft(0x1406f2d10, "unknown_1406f2d10", fn_1406f2d10 as *const ()), // sub_1406f2d10
    Replacement::draft(0x1406f2ea0, "unknown_1406f2ea0", fn_1406f2ea0 as *const ()), // sub_1406f2ea0
    Replacement::draft_throws(0x1406f2fe0, "unknown_1406f2fe0", fn_1406f2fe0 as *const (), Throws::Caught), // sub_1406f2fe0
    Replacement::draft(0x1406f3190, "unknown_1406f3190", fn_1406f3190 as *const ()), // sub_1406f3190
    Replacement::draft(0x1406f3300, "unknown_1406f3300", fn_1406f3300 as *const ()), // sub_1406f3300
    Replacement::draft_throws(0x1406f3450, "unknown_1406f3450", fn_1406f3450 as *const (), Throws::Caught), // sub_1406f3450
    Replacement::draft(0x1406f3580, "unknown_1406f3580", fn_1406f3580 as *const ()), // sub_1406f3580
    Replacement::draft_throws(0x1406f3620, "unknown_1406f3620", fn_1406f3620 as *const (), Throws::Caught), // sub_1406f3620
    Replacement::draft_throws(0x1406f3af0, "unknown_1406f3af0", fn_1406f3af0 as *const (), Throws::Caught), // sub_1406f3af0
    Replacement::draft_throws(0x1406f3f80, "unknown_1406f3f80", fn_1406f3f80 as *const (), Throws::Caught), // sub_1406f3f80
    Replacement::draft_throws(0x1406f4410, "unknown_1406f4410", fn_1406f4410 as *const (), Throws::Caught), // sub_1406f4410
    Replacement::draft_throws(0x1406f4c20, "unknown_1406f4c20", fn_1406f4c20 as *const (), Throws::Discard), // sub_1406f4c20
    Replacement::draft_throws(0x1406f5250, "unknown_1406f5250", fn_1406f5250 as *const (), Throws::Caught), // sub_1406f5250
    Replacement::draft_throws(0x1406f5af0, "unknown_1406f5af0", fn_1406f5af0 as *const (), Throws::Caught), // sub_1406f5af0
    Replacement::draft_throws(0x1406f6190, "unknown_1406f6190", fn_1406f6190 as *const (), Throws::Caught), // sub_1406f6190
    Replacement::draft_throws(0x1406f6830, "unknown_1406f6830", fn_1406f6830 as *const (), Throws::Caught), // sub_1406f6830
    Replacement::draft_throws(0x1406f6ed0, "content_1406f6ed0", fn_1406f6ed0 as *const (), Throws::Caught), // sub_1406f6ed0
    Replacement::draft_throws(0x1406f7470, "content_1406f7470", fn_1406f7470 as *const (), Throws::Caught), // sub_1406f7470
    Replacement::draft_throws(0x1406f7960, "content_1406f7960", fn_1406f7960 as *const (), Throws::Caught), // sub_1406f7960
    Replacement::draft(0x1406f7e50, "unknown_1406f7e50", fn_1406f7e50 as *const ()), // sub_1406f7e50
    Replacement::draft(0x1406f8010, "unknown_1406f8010", fn_1406f8010 as *const ()), // sub_1406f8010
    Replacement::draft(0x1406f81d0, "unknown_1406f81d0", fn_1406f81d0 as *const ()), // sub_1406f81d0
    Replacement::draft(0x1406f8390, "unknown_1406f8390", fn_1406f8390 as *const ()), // sub_1406f8390
    Replacement::draft_throws(0x1406f8550, "unknown_1406f8550", fn_1406f8550 as *const (), Throws::Caught), // sub_1406f8550
    Replacement::draft(0x1406f8700, "unknown_1406f8700", fn_1406f8700 as *const ()), // sub_1406f8700
    Replacement::draft_throws(0x1406f8890, "unknown_1406f8890", fn_1406f8890 as *const (), Throws::Caught), // sub_1406f8890
    Replacement::draft(0x1406f8a40, "unknown_1406f8a40", fn_1406f8a40 as *const ()), // sub_1406f8a40
    Replacement::draft_throws(0x1406f8bd0, "unknown_1406f8bd0", fn_1406f8bd0 as *const (), Throws::Caught), // sub_1406f8bd0
    Replacement::draft_throws(0x1406f8fc0, "unknown_1406f8fc0", fn_1406f8fc0 as *const (), Throws::Caught), // sub_1406f8fc0
    Replacement::draft(0x1406f9170, "unknown_1406f9170", fn_1406f9170 as *const ()), // sub_1406f9170
    Replacement::draft_throws(0x1406f9300, "unknown_1406f9300", fn_1406f9300 as *const (), Throws::Caught), // sub_1406f9300
    Replacement::draft(0x1406f94b0, "unknown_1406f94b0", fn_1406f94b0 as *const ()), // sub_1406f94b0
    Replacement::draft(0x1406f9640, "unknown_1406f9640", fn_1406f9640 as *const ()), // sub_1406f9640
    Replacement::draft(0x1406f9780, "unknown_1406f9780", fn_1406f9780 as *const ()), // sub_1406f9780
    Replacement::draft(0x1406f98c0, "unknown_1406f98c0", fn_1406f98c0 as *const ()), // sub_1406f98c0
    Replacement::draft(0x1406f9a00, "unknown_1406f9a00", fn_1406f9a00 as *const ()), // sub_1406f9a00
    Replacement::draft_throws(0x1406f9b40, "unknown_1406f9b40", fn_1406f9b40 as *const (), Throws::Caught), // sub_1406f9b40
    Replacement::draft_throws(0x1406f9cf0, "unknown_1406f9cf0", fn_1406f9cf0 as *const (), Throws::Caught), // sub_1406f9cf0
    Replacement::draft_throws(0x1406f9ea0, "unknown_1406f9ea0", fn_1406f9ea0 as *const (), Throws::Caught), // sub_1406f9ea0
    Replacement::draft_throws(0x1406fa290, "unknown_1406fa290", fn_1406fa290 as *const (), Throws::Caught), // sub_1406fa290
    Replacement::draft_throws(0x1406fa440, "unknown_1406fa440", fn_1406fa440 as *const (), Throws::Caught), // sub_1406fa440
    Replacement::draft_throws(0x1406fa5f0, "unknown_1406fa5f0", fn_1406fa5f0 as *const (), Throws::Caught), // sub_1406fa5f0
    Replacement::draft_throws(0x1406faac0, "unknown_1406faac0", fn_1406faac0 as *const (), Throws::Caught), // sub_1406faac0
    Replacement::draft_throws(0x1406faf50, "unknown_1406faf50", fn_1406faf50 as *const (), Throws::Caught), // sub_1406faf50
    Replacement::draft(0x1406fb3e0, "unknown_1406fb3e0", fn_1406fb3e0 as *const ()), // sub_1406fb3e0
    Replacement::draft(0x1406fb550, "unknown_1406fb550", fn_1406fb550 as *const ()), // sub_1406fb550
    Replacement::draft(0x1406fb6c0, "unknown_1406fb6c0", fn_1406fb6c0 as *const ()), // sub_1406fb6c0
    Replacement::draft(0x1406fb830, "unknown_1406fb830", fn_1406fb830 as *const ()), // sub_1406fb830
    Replacement::draft(0x1406fb9a0, "unknown_1406fb9a0", fn_1406fb9a0 as *const ()), // sub_1406fb9a0
    Replacement::draft(0x1406fbaf0, "unknown_1406fbaf0", fn_1406fbaf0 as *const ()), // sub_1406fbaf0
    Replacement::draft(0x1406fbc40, "unknown_1406fbc40", fn_1406fbc40 as *const ()), // sub_1406fbc40
    Replacement::draft(0x1406fbd90, "unknown_1406fbd90", fn_1406fbd90 as *const ()), // sub_1406fbd90
    Replacement::draft_throws(0x1406fbee0, "unknown_1406fbee0", fn_1406fbee0 as *const (), Throws::Caught), // sub_1406fbee0
    Replacement::draft_throws(0x1406fc010, "unknown_1406fc010", fn_1406fc010 as *const (), Throws::Caught), // sub_1406fc010
    Replacement::draft_throws(0x1406fc140, "unknown_1406fc140", fn_1406fc140 as *const (), Throws::Caught), // sub_1406fc140
    Replacement::draft_throws(0x1406fc270, "unknown_1406fc270", fn_1406fc270 as *const (), Throws::Caught), // sub_1406fc270
    Replacement::draft(0x1406fc3a0, "unknown_1406fc3a0", fn_1406fc3a0 as *const ()), // sub_1406fc3a0
    Replacement::draft(0x1406fc440, "unknown_1406fc440", fn_1406fc440 as *const ()), // sub_1406fc440
    Replacement::draft(0x1406fc4e0, "unknown_1406fc4e0", fn_1406fc4e0 as *const ()), // sub_1406fc4e0
    Replacement::draft(0x1406fc580, "unknown_1406fc580", fn_1406fc580 as *const ()), // sub_1406fc580
    Replacement::draft(0x1406fc620, "unknown_1406fc620", fn_1406fc620 as *const ()), // sub_1406fc620
    Replacement::draft(0x1406fc6d0, "unknown_1406fc6d0", fn_1406fc6d0 as *const ()), // sub_1406fc6d0
    Replacement::draft_throws(0x1406fc820, "unknown_1406fc820", fn_1406fc820 as *const (), Throws::Discard), // sub_1406fc820
    Replacement::draft(0x1406fc920, "unknown_1406fc920", fn_1406fc920 as *const ()), // sub_1406fc920
    Replacement::draft(0x1406fca00, "unknown_1406fca00", fn_1406fca00 as *const ()), // sub_1406fca00
    Replacement::draft_throws(0x1406fcac0, "unknown_1406fcac0", fn_1406fcac0 as *const (), Throws::Discard), // sub_1406fcac0
    Replacement::draft_throws(0x1406fcb60, "unknown_1406fcb60", fn_1406fcb60 as *const (), Throws::Discard), // sub_1406fcb60
    Replacement::draft(0x1406fcc90, "unknown_1406fcc90", fn_1406fcc90 as *const ()), // sub_1406fcc90
    Replacement::draft_throws(0x1406fccf0, "unknown_1406fccf0", tpl_838::<0x1407065f0> as *const (), Throws::Discard), // sub_1406fccf0
    Replacement::draft_throws(0x1406fcd00, "unknown_1406fcd00", tpl_838::<0x1407069a0> as *const (), Throws::Discard), // sub_1406fcd00
    Replacement::draft_throws(0x1406fcd10, "unknown_1406fcd10", tpl_838::<0x140706d20> as *const (), Throws::Discard), // sub_1406fcd10
    Replacement::draft(0x1406fcd20, "unknown_1406fcd20", tpl_66146::<0x8, 0x338, 0xe8, 0x120, 0x143a399b8, 0x28, 0xe0, 0x48> as *const ()), // sub_1406fcd20
    Replacement::draft(0x1406fce10, "unknown_1406fce10", fn_1406fce10 as *const ()), // sub_1406fce10
    Replacement::draft(0x1406fcec0, "unknown_1406fcec0", tpl_66146::<0x8, 0x338, 0xe8, 0x120, 0x143a399b8, 0x28, 0xe0, 0x40> as *const ()), // sub_1406fcec0
    Replacement::draft(0x1406fcfb0, "unknown_1406fcfb0", fn_1406fcfb0 as *const ()), // sub_1406fcfb0
    Replacement::draft(0x1406fd060, "unknown_1406fd060", fn_1406fd060 as *const ()), // sub_1406fd060
    Replacement::draft_throws(0x1406fd150, "unknown_1406fd150", fn_1406fd150 as *const (), Throws::Discard), // sub_1406fd150
    Replacement::draft_throws(0x1406fd260, "content_1406fd260", fn_1406fd260 as *const (), Throws::Caught), // sub_1406fd260
    Replacement::draft_throws(0x1406fd920, "content_1406fd920", fn_1406fd920 as *const (), Throws::Caught), // sub_1406fd920
    Replacement::draft_throws(0x1406fdf60, "content_1406fdf60", fn_1406fdf60 as *const (), Throws::Caught), // sub_1406fdf60
    Replacement::draft_throws(0x1406fe520, "unknown_1406fe520", fn_1406fe520 as *const (), Throws::Caught), // sub_1406fe520
    Replacement::draft(0x1406fe630, "unknown_1406fe630", fn_1406fe630 as *const ()), // sub_1406fe630
    Replacement::draft_throws(0x1406fe700, "unknown_1406fe700", fn_1406fe700 as *const (), Throws::Discard), // sub_1406fe700
    Replacement::draft(0x1406fe800, "unknown_1406fe800", fn_1406fe800 as *const ()), // sub_1406fe800
    Replacement::draft(0x1406fe890, "unknown_1406fe890", tpl_7154::<0x1438820c0, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406fe890
    Replacement::draft_throws(0x1406fe960, "unknown_1406fe960", fn_1406fe960 as *const (), Throws::Discard), // sub_1406fe960
    Replacement::draft(0x1406fea30, "unknown_1406fea30", fn_1406fea30 as *const ()), // sub_1406fea30
    Replacement::draft(0x1406feb00, "unknown_1406feb00", tpl_7154::<0x1438820c0, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1406feb00
    Replacement::draft_throws(0x1406febe0, "unknown_1406febe0", fn_1406febe0 as *const (), Throws::Discard), // sub_1406febe0
    Replacement::draft_throws(0x1406fec80, "content_1406fec80", fn_1406fec80 as *const (), Throws::Caught), // sub_1406fec80
    Replacement::draft_throws(0x1406ff1f0, "unknown_1406ff1f0", fn_1406ff1f0 as *const (), Throws::Caught), // sub_1406ff1f0
    Replacement::draft_throws(0x1406ff6c0, "content_1406ff6c0", fn_1406ff6c0 as *const (), Throws::Caught), // sub_1406ff6c0
    Replacement::draft_throws(0x1406ffba0, "unknown_1406ffba0", fn_1406ffba0 as *const (), Throws::Caught), // sub_1406ffba0
    Replacement::draft_throws(0x140700030, "content_140700030", fn_140700030 as *const (), Throws::Caught), // sub_140700030
    Replacement::draft_throws(0x140700500, "unknown_140700500", fn_140700500 as *const (), Throws::Caught), // sub_140700500
    Replacement::draft_throws(0x140700990, "unknown_140700990", fn_140700990 as *const (), Throws::Caught), // sub_140700990
    Replacement::draft_throws(0x140700e60, "unknown_140700e60", fn_140700e60 as *const (), Throws::Caught), // sub_140700e60
    Replacement::draft_throws(0x1407012f0, "unknown_1407012f0", fn_1407012f0 as *const (), Throws::Caught), // sub_1407012f0
    Replacement::draft_throws(0x140701780, "std_140701780", fn_140701780 as *const (), Throws::Discard), // sub_140701780
    Replacement::draft(0x140701a30, "unknown_140701a30", fn_140701a30 as *const ()), // sub_140701a30
    Replacement::draft(0x140701a90, "unknown_140701a90", fn_140701a90 as *const ()), // sub_140701a90
    Replacement::draft(0x140701b70, "unknown_140701b70", tpl_66164::<0x8, 0xc, 0x140708f60, 0x14183acc2, 0x1401ffe00> as *const ()), // sub_140701b70
    Replacement::draft(0x140701c30, "unknown_140701c30", fn_140701c30 as *const ()), // sub_140701c30
    Replacement::draft_throws(0x140702080, "unknown_140702080", fn_140702080 as *const (), Throws::Caught), // sub_140702080
    Replacement::draft_throws(0x140702210, "unknown_140702210", fn_140702210 as *const (), Throws::Caught), // sub_140702210
    Replacement::draft_throws(0x1407023a0, "unknown_1407023a0", fn_1407023a0 as *const (), Throws::Caught), // sub_1407023a0
    Replacement::draft_throws(0x140702530, "unknown_140702530", fn_140702530 as *const (), Throws::Caught), // sub_140702530
    Replacement::draft(0x1407026c0, "unknown_1407026c0", fn_1407026c0 as *const ()), // sub_1407026c0
    Replacement::draft_throws(0x140702780, "unknown_140702780", fn_140702780 as *const (), Throws::Caught), // sub_140702780
    Replacement::draft_throws(0x1407029f0, "unknown_1407029f0", fn_1407029f0 as *const (), Throws::Caught), // sub_1407029f0
    Replacement::draft_throws(0x140702c60, "unknown_140702c60", fn_140702c60 as *const (), Throws::Caught), // sub_140702c60
    Replacement::draft_throws(0x140702ed0, "unknown_140702ed0", fn_140702ed0 as *const (), Throws::Caught), // sub_140702ed0
    Replacement::draft(0x1407030c0, "unknown_1407030c0", tpl_7173::<0x143244f68, 0x20, 0x1, 0x14028a830, 0x28, 0x143245068> as *const ()), // sub_1407030c0
    Replacement::draft_throws(0x140703130, "unknown_140703130", fn_140703130 as *const (), Throws::Caught), // sub_140703130
    Replacement::draft_throws(0x140703270, "unknown_140703270", fn_140703270 as *const (), Throws::Discard), // sub_140703270
    Replacement::draft(0x140703410, "unknown_140703410", fn_140703410 as *const ()), // sub_140703410
    Replacement::draft(0x1407034e0, "unknown_1407034e0", fn_1407034e0 as *const ()), // sub_1407034e0
    Replacement::draft_throws(0x1407035c0, "unknown_1407035c0", fn_1407035c0 as *const (), Throws::Caught), // sub_1407035c0
    Replacement::draft_throws(0x140703750, "unknown_140703750", fn_140703750 as *const (), Throws::Caught), // sub_140703750
    Replacement::draft_throws(0x1407038e0, "unknown_1407038e0", fn_1407038e0 as *const (), Throws::Caught), // sub_1407038e0
    Replacement::draft_throws(0x140703cd0, "unknown_140703cd0", fn_140703cd0 as *const (), Throws::Caught), // sub_140703cd0
    Replacement::draft_throws(0x140703e60, "unknown_140703e60", fn_140703e60 as *const (), Throws::Caught), // sub_140703e60
    Replacement::draft_throws(0x140703ff0, "unknown_140703ff0", fn_140703ff0 as *const (), Throws::Caught), // sub_140703ff0
    Replacement::draft_throws(0x140704180, "unknown_140704180", fn_140704180 as *const (), Throws::Caught), // sub_140704180
    Replacement::draft_throws(0x140704310, "unknown_140704310", fn_140704310 as *const (), Throws::Caught), // sub_140704310
    Replacement::draft_throws(0x140704700, "unknown_140704700", fn_140704700 as *const (), Throws::Caught), // sub_140704700
    Replacement::draft_throws(0x140704890, "unknown_140704890", fn_140704890 as *const (), Throws::Caught), // sub_140704890
    Replacement::draft_throws(0x140704a20, "unknown_140704a20", fn_140704a20 as *const (), Throws::Caught), // sub_140704a20
    Replacement::draft_throws(0x140704bb0, "unknown_140704bb0", fn_140704bb0 as *const (), Throws::Caught), // sub_140704bb0
    Replacement::draft_throws(0x140704d40, "unknown_140704d40", fn_140704d40 as *const (), Throws::Caught), // sub_140704d40
    Replacement::draft_throws(0x140705130, "unknown_140705130", fn_140705130 as *const (), Throws::Caught), // sub_140705130
    Replacement::draft_throws(0x1407052c0, "unknown_1407052c0", fn_1407052c0 as *const (), Throws::Caught), // sub_1407052c0
    Replacement::draft_throws(0x140705450, "unknown_140705450", fn_140705450 as *const (), Throws::Caught), // sub_140705450
    Replacement::draft_throws(0x1407055e0, "unknown_1407055e0", fn_1407055e0 as *const (), Throws::Caught), // sub_1407055e0
    Replacement::draft_throws(0x140705770, "unknown_140705770", fn_140705770 as *const (), Throws::Caught), // sub_140705770
    Replacement::draft_throws(0x140705960, "unknown_140705960", fn_140705960 as *const (), Throws::Caught), // sub_140705960
    Replacement::draft_throws(0x140705af0, "unknown_140705af0", fn_140705af0 as *const (), Throws::Caught), // sub_140705af0
    Replacement::draft_throws(0x140705c80, "unknown_140705c80", fn_140705c80 as *const (), Throws::Caught), // sub_140705c80
    Replacement::draft_throws(0x140705fb0, "unknown_140705fb0", tpl_66193::<0x142442de0, 0x143a399b8, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140705fb0
    Replacement::draft_throws(0x140706210, "unknown_140706210", fn_140706210 as *const (), Throws::Caught), // sub_140706210
    Replacement::draft_throws(0x140706530, "unknown_140706530", fn_140706530 as *const (), Throws::Discard), // sub_140706530
    Replacement::draft_throws(0x1407065f0, "unknown_1407065f0", fn_1407065f0 as *const (), Throws::Caught), // sub_1407065f0
    Replacement::draft_throws(0x1407068e0, "unknown_1407068e0", fn_1407068e0 as *const (), Throws::Caught), // sub_1407068e0
    Replacement::draft_throws(0x1407069a0, "unknown_1407069a0", fn_1407069a0 as *const (), Throws::Caught), // sub_1407069a0
    Replacement::draft_throws(0x140706c60, "unknown_140706c60", fn_140706c60 as *const (), Throws::Caught), // sub_140706c60
    Replacement::draft_throws(0x140706d20, "unknown_140706d20", fn_140706d20 as *const (), Throws::Caught), // sub_140706d20
    Replacement::draft_throws(0x140706fe0, "unknown_140706fe0", fn_140706fe0 as *const (), Throws::Caught), // sub_140706fe0
    Replacement::draft_throws(0x1407070a0, "unknown_1407070a0", fn_1407070a0 as *const (), Throws::Caught), // sub_1407070a0
    Replacement::draft_throws(0x140707170, "unknown_140707170", tpl_7203::<0x14387d230, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143244fb0, 0x1402c4730, 0x48, 0x143244f78, 0x14070db80> as *const (), Throws::Discard), // sub_140707170
    Replacement::draft_throws(0x140707240, "unknown_140707240", fn_140707240 as *const (), Throws::Caught), // sub_140707240
    Replacement::draft_throws(0x140707580, "unknown_140707580", fn_140707580 as *const (), Throws::Caught), // sub_140707580
    Replacement::draft_throws(0x140707860, "unknown_140707860", fn_140707860 as *const (), Throws::Caught), // sub_140707860
    Replacement::draft_throws(0x140707b30, "unknown_140707b30", fn_140707b30 as *const (), Throws::Discard), // sub_140707b30
    Replacement::draft_throws(0x140707c50, "unknown_140707c50", fn_140707c50 as *const (), Throws::Caught), // sub_140707c50
    Replacement::draft_throws(0x140707f90, "unknown_140707f90", fn_140707f90 as *const (), Throws::Caught), // sub_140707f90
    Replacement::draft_throws(0x1407082c0, "unknown_1407082c0", fn_1407082c0 as *const (), Throws::Caught), // sub_1407082c0
    Replacement::draft_throws(0x1407085f0, "unknown_1407085f0", fn_1407085f0 as *const (), Throws::Caught), // sub_1407085f0
    Replacement::draft_throws(0x140708680, "unknown_140708680", fn_140708680 as *const (), Throws::Caught), // sub_140708680
    Replacement::draft_throws(0x140708870, "unknown_140708870", fn_140708870 as *const (), Throws::Caught), // sub_140708870
    Replacement::draft_throws(0x1407089b0, "unknown_1407089b0", fn_1407089b0 as *const (), Throws::Caught), // sub_1407089b0
    Replacement::draft_throws(0x140708af0, "unknown_140708af0", fn_140708af0 as *const (), Throws::Caught), // sub_140708af0
    Replacement::draft_throws(0x140708c30, "unknown_140708c30", fn_140708c30 as *const (), Throws::Caught), // sub_140708c30
    Replacement::draft(0x140708e20, "unknown_140708e20", fn_140708e20 as *const ()), // sub_140708e20
    Replacement::draft_throws(0x140708e80, "unknown_140708e80", tpl_65670::<0x142442de0, 0x143a399b8, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x70> as *const (), Throws::Discard), // sub_140708e80
    Replacement::draft(0x140708f60, "unknown_140708f60", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1401ffd50> as *const ()), // sub_140708f60
    Replacement::draft(0x1407090d0, "unknown_1407090d0", tpl_66202::<0x8, 0xc, 0x14052f910, 0x14183acc2> as *const ()), // sub_1407090d0
    Replacement::draft_throws(0x140709190, "unknown_140709190", fn_140709190 as *const (), Throws::Discard), // sub_140709190
    Replacement::draft(0x140709290, "unknown_140709290", fn_140709290 as *const ()), // sub_140709290
    Replacement::draft_throws(0x140709350, "unknown_140709350", fn_140709350 as *const (), Throws::Discard), // sub_140709350
    Replacement::draft_throws(0x1407093f0, "unknown_1407093f0", fn_1407093f0 as *const (), Throws::Discard), // sub_1407093f0
    Replacement::draft_throws(0x140709500, "unknown_140709500", fn_140709500 as *const (), Throws::Caught), // sub_140709500
    Replacement::draft_throws(0x140709ab0, "unknown_140709ab0", fn_140709ab0 as *const (), Throws::Caught), // sub_140709ab0
    Replacement::draft_throws(0x14070a050, "unknown_14070a050", fn_14070a050 as *const (), Throws::Caught), // sub_14070a050
    Replacement::draft(0x14070a3a0, "unknown_14070a3a0", fn_14070a3a0 as *const ()), // sub_14070a3a0
    Replacement::draft(0x14070a470, "unknown_14070a470", fn_14070a470 as *const ()), // sub_14070a470
    Replacement::draft_throws(0x14070a540, "unknown_14070a540", fn_14070a540 as *const (), Throws::Discard), // sub_14070a540
    Replacement::draft(0x14070a630, "unknown_14070a630", fn_14070a630 as *const ()), // sub_14070a630
    Replacement::draft(0x14070a740, "unknown_14070a740", fn_14070a740 as *const ()), // sub_14070a740
    Replacement::draft(0x14070a960, "unknown_14070a960", fn_14070a960 as *const ()), // sub_14070a960
    Replacement::draft_throws(0x14070aa90, "unknown_14070aa90", fn_14070aa90 as *const (), Throws::Caught), // sub_14070aa90
    Replacement::draft_throws(0x14070abd0, "unknown_14070abd0", fn_14070abd0 as *const (), Throws::Caught), // sub_14070abd0
    Replacement::draft_throws(0x14070ad10, "unknown_14070ad10", fn_14070ad10 as *const (), Throws::Caught), // sub_14070ad10
    Replacement::draft(0x14070ae50, "unknown_14070ae50", fn_14070ae50 as *const ()), // sub_14070ae50
    Replacement::draft(0x14070af90, "unknown_14070af90", fn_14070af90 as *const ()), // sub_14070af90
    Replacement::draft(0x14070b0b0, "unknown_14070b0b0", fn_14070b0b0 as *const ()), // sub_14070b0b0
    Replacement::draft_throws(0x14070b1e0, "unknown_14070b1e0", fn_14070b1e0 as *const (), Throws::Discard), // sub_14070b1e0
    Replacement::draft_throws(0x14070b350, "unknown_14070b350", fn_14070b350 as *const (), Throws::Discard), // sub_14070b350
    Replacement::draft(0x14070b470, "unknown_14070b470", fn_14070b470 as *const ()), // sub_14070b470
    Replacement::draft(0x14070b590, "unknown_14070b590", tpl_7230::<0x14387d2f8, 0x14243b0b0, 0x338, 0x140715850, 0x318, 0x328, 0x320, 0x8, 0xc, 0x38, 0x30, 0x1406fe890> as *const ()), // sub_14070b590
    Replacement::draft_throws(0x14070b640, "unknown_14070b640", fn_14070b640 as *const (), Throws::Discard), // sub_14070b640
    Replacement::draft_throws(0x14070b7a0, "unknown_14070b7a0", fn_14070b7a0 as *const (), Throws::Caught), // sub_14070b7a0
    Replacement::draft_throws(0x14070bee0, "unknown_14070bee0", fn_14070bee0 as *const (), Throws::Caught), // sub_14070bee0
    Replacement::draft_throws(0x14070c5f0, "unknown_14070c5f0", fn_14070c5f0 as *const (), Throws::Caught), // sub_14070c5f0
    Replacement::draft(0x14070c940, "unknown_14070c940", fn_14070c940 as *const ()), // sub_14070c940
    Replacement::draft(0x14070c9e0, "unknown_14070c9e0", tpl_7230::<0x14387d2f8, 0x14243b0b0, 0x338, 0x140715760, 0x318, 0x328, 0x320, 0x8, 0xc, 0x38, 0x30, 0x1406feb00> as *const ()), // sub_14070c9e0
    Replacement::draft_throws(0x14070ca90, "unknown_14070ca90", fn_14070ca90 as *const (), Throws::Discard), // sub_14070ca90
    Replacement::draft_throws(0x14070cb80, "unknown_14070cb80", fn_14070cb80 as *const (), Throws::Caught), // sub_14070cb80
    Replacement::draft_throws(0x14070d050, "unknown_14070d050", tpl_838::<0x1407118d0> as *const (), Throws::Discard), // sub_14070d050
    Replacement::draft_throws(0x14070d060, "unknown_14070d060", fn_14070d060 as *const (), Throws::Caught), // sub_14070d060
    Replacement::draft_throws(0x14070d4e0, "unknown_14070d4e0", tpl_838::<0x140712630> as *const (), Throws::Discard), // sub_14070d4e0
    Replacement::draft_throws(0x14070d4f0, "unknown_14070d4f0", fn_14070d4f0 as *const (), Throws::Caught), // sub_14070d4f0
    Replacement::draft_throws(0x14070d970, "unknown_14070d970", tpl_838::<0x140713360> as *const (), Throws::Discard), // sub_14070d970
    Replacement::draft_throws(0x14070d980, "unknown_14070d980", fn_14070d980 as *const (), Throws::Caught), // sub_14070d980
    Replacement::draft_throws(0x14070dbd0, "content_14070dbd0", fn_14070dbd0 as *const (), Throws::Discard), // sub_14070dbd0
    Replacement::draft_throws(0x14070dc90, "unknown_14070dc90", fn_14070dc90 as *const (), Throws::Caught), // sub_14070dc90
    Replacement::draft_throws(0x14070e300, "content_14070e300", fn_14070e300 as *const (), Throws::Discard), // sub_14070e300
    Replacement::draft_throws(0x14070e370, "unknown_14070e370", fn_14070e370 as *const (), Throws::Caught), // sub_14070e370
    Replacement::draft_throws(0x14070e9b0, "unknown_14070e9b0", fn_14070e9b0 as *const (), Throws::Caught), // sub_14070e9b0
    Replacement::draft_throws(0x14070ed00, "unknown_14070ed00", fn_14070ed00 as *const (), Throws::Caught), // sub_14070ed00
    Replacement::draft_throws(0x14070f170, "unknown_14070f170", fn_14070f170 as *const (), Throws::Caught), // sub_14070f170
    Replacement::draft_throws(0x14070f5a0, "unknown_14070f5a0", fn_14070f5a0 as *const (), Throws::Caught), // sub_14070f5a0
    Replacement::draft_throws(0x14070f8f0, "unknown_14070f8f0", fn_14070f8f0 as *const (), Throws::Discard), // sub_14070f8f0
    Replacement::draft_throws(0x14070fa00, "unknown_14070fa00", fn_14070fa00 as *const (), Throws::Caught), // sub_14070fa00
    Replacement::draft_throws(0x14070ffb0, "unknown_14070ffb0", fn_14070ffb0 as *const (), Throws::Caught), // sub_14070ffb0
    Replacement::draft_throws(0x140710520, "unknown_140710520", fn_140710520 as *const (), Throws::Caught), // sub_140710520
    Replacement::draft_throws(0x140710960, "unknown_140710960", fn_140710960 as *const (), Throws::Caught), // sub_140710960
    Replacement::draft_throws(0x140710cb0, "unknown_140710cb0", fn_140710cb0 as *const (), Throws::Caught), // sub_140710cb0
    Replacement::draft_throws(0x140710e10, "unknown_140710e10", fn_140710e10 as *const (), Throws::Caught), // sub_140710e10
    Replacement::draft_throws(0x1407118d0, "unknown_1407118d0", fn_1407118d0 as *const (), Throws::Caught), // sub_1407118d0
    Replacement::draft_throws(0x140711ba0, "unknown_140711ba0", fn_140711ba0 as *const (), Throws::Caught), // sub_140711ba0
    Replacement::draft_throws(0x140712630, "unknown_140712630", fn_140712630 as *const (), Throws::Caught), // sub_140712630
    Replacement::draft_throws(0x1407128e0, "unknown_1407128e0", fn_1407128e0 as *const (), Throws::Caught), // sub_1407128e0
    Replacement::draft_throws(0x140713360, "unknown_140713360", fn_140713360 as *const (), Throws::Caught), // sub_140713360
    Replacement::draft_throws(0x140713600, "unknown_140713600", fn_140713600 as *const (), Throws::Caught), // sub_140713600
    Replacement::draft_throws(0x1407136e0, "content_1407136e0", fn_1407136e0 as *const (), Throws::Discard), // sub_1407136e0
    Replacement::draft_throws(0x140713780, "unknown_140713780", fn_140713780 as *const (), Throws::Caught), // sub_140713780
    Replacement::draft_throws(0x1407138b0, "unknown_1407138b0", fn_1407138b0 as *const (), Throws::Caught), // sub_1407138b0
    Replacement::draft_throws(0x1407139e0, "unknown_1407139e0", fn_1407139e0 as *const (), Throws::Caught), // sub_1407139e0
    Replacement::draft_throws(0x140713b10, "unknown_140713b10", fn_140713b10 as *const (), Throws::Discard), // sub_140713b10
    Replacement::draft(0x140713d00, "unknown_140713d00", fn_140713d00 as *const ()), // sub_140713d00
    Replacement::draft_throws(0x140713e20, "unknown_140713e20", fn_140713e20 as *const (), Throws::Discard), // sub_140713e20
    Replacement::draft_throws(0x140714070, "unknown_140714070", fn_140714070 as *const (), Throws::Caught), // sub_140714070
    Replacement::draft_throws(0x140714160, "unknown_140714160", fn_140714160 as *const (), Throws::Caught), // sub_140714160
    Replacement::draft_throws(0x140714250, "unknown_140714250", fn_140714250 as *const (), Throws::Caught), // sub_140714250
    Replacement::draft_throws(0x140714480, "unknown_140714480", fn_140714480 as *const (), Throws::Caught), // sub_140714480
    Replacement::draft_throws(0x1407145c0, "unknown_1407145c0", fn_1407145c0 as *const (), Throws::Caught), // sub_1407145c0
    Replacement::draft_throws(0x140714ab0, "unknown_140714ab0", fn_140714ab0 as *const (), Throws::Caught), // sub_140714ab0
    Replacement::draft_throws(0x140714bf0, "unknown_140714bf0", fn_140714bf0 as *const (), Throws::Caught), // sub_140714bf0
    Replacement::draft_throws(0x1407150a0, "unknown_1407150a0", fn_1407150a0 as *const (), Throws::Caught), // sub_1407150a0
    Replacement::draft_throws(0x1407151e0, "unknown_1407151e0", fn_1407151e0 as *const (), Throws::Caught), // sub_1407151e0
    Replacement::draft_throws(0x140715690, "unknown_140715690", fn_140715690 as *const (), Throws::Caught), // sub_140715690
    Replacement::draft(0x140715760, "unknown_140715760", fn_140715760 as *const ()), // sub_140715760
    Replacement::draft(0x140715850, "unknown_140715850", fn_140715850 as *const ()), // sub_140715850
    Replacement::draft_throws(0x140715910, "unknown_140715910", fn_140715910 as *const (), Throws::Caught), // sub_140715910
    Replacement::draft_throws(0x140715ab0, "unknown_140715ab0", fn_140715ab0 as *const (), Throws::Caught), // sub_140715ab0
    Replacement::draft(0x140715c90, "unknown_140715c90", fn_140715c90 as *const ()), // sub_140715c90
    Replacement::draft(0x140715df0, "unknown_140715df0", tpl_7264::<0x14387d268, 0x14243b0b0, 0x1426d4580> as *const ()), // sub_140715df0
    Replacement::draft(0x140715f00, "unknown_140715f00", tpl_7264::<0x14387d220, 0x14243b0b0, 0x1426d4580> as *const ()), // sub_140715f00
    Replacement::draft_throws(0x140716010, "unknown_140716010", fn_140716010 as *const (), Throws::Caught), // sub_140716010
    Replacement::draft_throws(0x140716280, "unknown_140716280", fn_140716280 as *const (), Throws::Caught), // sub_140716280
    Replacement::draft_throws(0x1407164e0, "unknown_1407164e0", fn_1407164e0 as *const (), Throws::Caught), // sub_1407164e0
    Replacement::draft_throws(0x140716740, "unknown_140716740", fn_140716740 as *const (), Throws::Caught), // sub_140716740
    Replacement::draft_throws(0x1407169b0, "unknown_1407169b0", fn_1407169b0 as *const (), Throws::Caught), // sub_1407169b0
    Replacement::draft_throws(0x140716c10, "unknown_140716c10", fn_140716c10 as *const (), Throws::Caught), // sub_140716c10
    Replacement::draft_throws(0x140716e70, "unknown_140716e70", fn_140716e70 as *const (), Throws::Caught), // sub_140716e70
    Replacement::draft_throws(0x140717170, "unknown_140717170", fn_140717170 as *const (), Throws::Caught), // sub_140717170
    Replacement::draft_throws(0x140717470, "unknown_140717470", fn_140717470 as *const (), Throws::Caught), // sub_140717470
    Replacement::draft(0x140717770, "unknown_140717770", fn_140717770 as *const ()), // sub_140717770
    Replacement::draft(0x140717830, "unknown_140717830", fn_140717830 as *const ()), // sub_140717830
    Replacement::draft(0x1407178d0, "unknown_1407178d0", fn_1407178d0 as *const ()), // sub_1407178d0
    Replacement::draft(0x140717970, "unknown_140717970", fn_140717970 as *const ()), // sub_140717970
    Replacement::draft(0x140717a20, "unknown_140717a20", tpl_7273::<0x14387d29c, 0x14387d268, 0x1401e9970, 0x338, 0x120, 0x1, 0x4, 0xd8, 0x8, 0x1401e9860> as *const ()), // sub_140717a20
    Replacement::draft(0x140717ac0, "unknown_140717ac0", tpl_7273::<0x14387d2a0, 0x14387d220, 0x1401e9970, 0x338, 0x120, 0x1, 0x4, 0xd8, 0x8, 0x1401e9860> as *const ()), // sub_140717ac0
    Replacement::draft(0x140717b60, "unknown_140717b60", tpl_7274::<0x14387d2a8, 0x14243b0b0, 0x338, 0xd8, 0x4, 0xe8, 0x8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140717b60
    Replacement::draft(0x140717c20, "unknown_140717c20", tpl_7275::<0x14387d268, 0x14243b0b0, 0x338, 0x120, 0x1, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140717c20
    Replacement::draft(0x140717cd0, "unknown_140717cd0", tpl_7275::<0x14387d220, 0x14243b0b0, 0x338, 0x120, 0x1, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140717cd0
    Replacement::draft_throws(0x140717d80, "unknown_140717d80", fn_140717d80 as *const (), Throws::Discard), // sub_140717d80
    Replacement::draft_throws(0x140717e60, "unknown_140717e60", tpl_7277::<0x14387d29c, 0x14387d268, 0x1401e9770, 0x338, 0x120, 0x1, 0x4, 0x8, 0x1401e9590> as *const (), Throws::Discard), // sub_140717e60
    Replacement::draft_throws(0x140717f20, "unknown_140717f20", tpl_7277::<0x14387d2a0, 0x14387d220, 0x1401e9770, 0x338, 0x120, 0x1, 0x4, 0x8, 0x1401e9590> as *const (), Throws::Discard), // sub_140717f20
    Replacement::draft_throws(0x140717fe0, "unknown_140717fe0", fn_140717fe0 as *const (), Throws::Caught), // sub_140717fe0
    Replacement::draft_throws(0x140718590, "content_140718590", fn_140718590 as *const (), Throws::Discard), // sub_140718590
    Replacement::draft_throws(0x140718650, "unknown_140718650", fn_140718650 as *const (), Throws::Caught), // sub_140718650
    Replacement::draft_throws(0x140718bd0, "content_140718bd0", fn_140718bd0 as *const (), Throws::Discard), // sub_140718bd0
    Replacement::draft_throws(0x140718c30, "unknown_140718c30", fn_140718c30 as *const (), Throws::Caught), // sub_140718c30
    Replacement::draft_throws(0x140719080, "content_140719080", fn_140719080 as *const (), Throws::Discard), // sub_140719080
    Replacement::draft_throws(0x1407190d0, "unknown_1407190d0", fn_1407190d0 as *const (), Throws::Caught), // sub_1407190d0
    Replacement::draft_throws(0x140719340, "unknown_140719340", fn_140719340 as *const (), Throws::Caught), // sub_140719340
    Replacement::draft_throws(0x1407195a0, "unknown_1407195a0", fn_1407195a0 as *const (), Throws::Caught), // sub_1407195a0
    Replacement::draft_throws(0x140719800, "unknown_140719800", fn_140719800 as *const (), Throws::Caught), // sub_140719800
    Replacement::draft_throws(0x140719a70, "unknown_140719a70", fn_140719a70 as *const (), Throws::Caught), // sub_140719a70
    Replacement::draft_throws(0x140719cd0, "unknown_140719cd0", fn_140719cd0 as *const (), Throws::Caught), // sub_140719cd0
    Replacement::draft_throws(0x140719f30, "unknown_140719f30", fn_140719f30 as *const (), Throws::Caught), // sub_140719f30
    Replacement::draft_throws(0x14071a1a0, "unknown_14071a1a0", fn_14071a1a0 as *const (), Throws::Caught), // sub_14071a1a0
    Replacement::draft_throws(0x14071a400, "unknown_14071a400", fn_14071a400 as *const (), Throws::Caught), // sub_14071a400
    Replacement::draft_throws(0x14071a660, "unknown_14071a660", fn_14071a660 as *const (), Throws::Discard), // sub_14071a660
    Replacement::draft_throws(0x14071a790, "unknown_14071a790", fn_14071a790 as *const (), Throws::Discard), // sub_14071a790
    Replacement::draft_throws(0x14071a890, "unknown_14071a890", fn_14071a890 as *const (), Throws::Discard), // sub_14071a890
    Replacement::draft(0x14071a990, "unknown_14071a990", tpl_7274::<0x14387d2a8, 0x14243b0b0, 0x338, 0x38, 0x4, 0x48, 0x8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14071a990
    Replacement::draft(0x14071aa40, "unknown_14071aa40", tpl_7275::<0x14387d268, 0x14243b0b0, 0x338, 0x80, 0x1, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14071aa40
    Replacement::draft(0x14071aae0, "unknown_14071aae0", tpl_7275::<0x14387d220, 0x14243b0b0, 0x338, 0x80, 0x1, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14071aae0
    Replacement::draft_throws(0x14071ab80, "unknown_14071ab80", fn_14071ab80 as *const (), Throws::Discard), // sub_14071ab80
    Replacement::draft_throws(0x14071ace0, "unknown_14071ace0", fn_14071ace0 as *const (), Throws::Discard), // sub_14071ace0
    Replacement::draft_throws(0x14071ae00, "unknown_14071ae00", fn_14071ae00 as *const (), Throws::Discard), // sub_14071ae00
    Replacement::draft_throws(0x14071af20, "unknown_14071af20", fn_14071af20 as *const (), Throws::Caught), // sub_14071af20
    Replacement::draft_throws(0x14071b5d0, "unknown_14071b5d0", fn_14071b5d0 as *const (), Throws::Caught), // sub_14071b5d0
    Replacement::draft_throws(0x14071bc70, "unknown_14071bc70", fn_14071bc70 as *const (), Throws::Caught), // sub_14071bc70
    Replacement::draft_throws(0x14071c310, "unknown_14071c310", fn_14071c310 as *const (), Throws::Caught), // sub_14071c310
    Replacement::draft_throws(0x14071c990, "unknown_14071c990", fn_14071c990 as *const (), Throws::Caught), // sub_14071c990
    Replacement::draft_throws(0x14071d000, "unknown_14071d000", fn_14071d000 as *const (), Throws::Caught), // sub_14071d000
    Replacement::draft_throws(0x14071d670, "unknown_14071d670", fn_14071d670 as *const (), Throws::Caught), // sub_14071d670
    Replacement::draft_throws(0x14071dab0, "unknown_14071dab0", fn_14071dab0 as *const (), Throws::Caught), // sub_14071dab0
    Replacement::draft_throws(0x14071def0, "unknown_14071def0", fn_14071def0 as *const (), Throws::Caught), // sub_14071def0
    Replacement::draft_throws(0x14071e330, "unknown_14071e330", fn_14071e330 as *const (), Throws::Discard), // sub_14071e330
    Replacement::draft_throws(0x14071e4d0, "unknown_14071e4d0", fn_14071e4d0 as *const (), Throws::Discard), // sub_14071e4d0
    Replacement::draft_throws(0x14071e630, "unknown_14071e630", fn_14071e630 as *const (), Throws::Discard), // sub_14071e630
    Replacement::draft_throws(0x14071e790, "unknown_14071e790", fn_14071e790 as *const (), Throws::Caught), // sub_14071e790
    Replacement::draft_throws(0x14071e940, "unknown_14071e940", fn_14071e940 as *const (), Throws::Caught), // sub_14071e940
    Replacement::draft_throws(0x14071eab0, "unknown_14071eab0", fn_14071eab0 as *const (), Throws::Caught), // sub_14071eab0
    Replacement::draft_throws(0x14071ec20, "unknown_14071ec20", fn_14071ec20 as *const (), Throws::Caught), // sub_14071ec20
    Replacement::draft_throws(0x14071edd0, "unknown_14071edd0", fn_14071edd0 as *const (), Throws::Caught), // sub_14071edd0
    Replacement::draft_throws(0x14071ef40, "unknown_14071ef40", fn_14071ef40 as *const (), Throws::Caught), // sub_14071ef40
    Replacement::draft_throws(0x14071f0b0, "unknown_14071f0b0", fn_14071f0b0 as *const (), Throws::Caught), // sub_14071f0b0
    Replacement::draft_throws(0x14071f260, "unknown_14071f260", fn_14071f260 as *const (), Throws::Caught), // sub_14071f260
    Replacement::draft_throws(0x14071f3d0, "unknown_14071f3d0", fn_14071f3d0 as *const (), Throws::Caught), // sub_14071f3d0
    Replacement::draft(0x14071f5a0, "content_14071f5a0", tpl_7305::<0x3, 0x4, 0x8, 0x143226430, 0x142409b50> as *const ()), // content::FiniteStateMachineNode::content::FiniteStateMachineNode::vfunc_22
    Replacement::draft(0x14071f630, "content_14071f630", tpl_7306::<0x3, 0x4, 0x8, 0x143226938, 0x142409b50> as *const ()), // content::FiniteStateMachineConnectionNode::content::FiniteStateMachineConnectionNode::vfunc_22
    Replacement::draft_throws(0x14071f690, "content_14071f690", tpl_7307::<0x143c89f48, 0x143c89f50, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::FiniteStateMachineConnectionNode::content::FiniteStateMachineConnectionNode::vfunc_23
    Replacement::draft(0x14071f7b0, "content_14071f7b0", tpl_7308::<0x3, 0x4, 0x8, 0x143226618, 0x142409b50> as *const ()), // content::FiniteStateMachineExitNode::content::FiniteStateMachineExitNode::vfunc_22
    Replacement::draft_throws(0x14071f810, "content_14071f810", tpl_7307::<0x143c89f58, 0x143c89f30, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::FiniteStateMachineExitNode::content::FiniteStateMachineExitNode::vfunc_23
    Replacement::draft(0x14071f930, "content_14071f930", tpl_7309::<0x3, 0x4, 0x8, 0x143225cc0, 0x142409b50> as *const ()), // content::FiniteStateMachineStateNode::content::FiniteStateMachineStateNode::vfunc_22
    Replacement::draft_throws(0x14071f990, "content_14071f990", tpl_7307::<0x143c89f5c, 0x143c89f38, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::FiniteStateMachineStateNode::content::FiniteStateMachineStateNode::vfunc_23
    Replacement::draft(0x14071fab0, "content_14071fab0", tpl_7308::<0x3, 0x4, 0x8, 0x143226110, 0x142409b50> as *const ()), // content::FiniteStateMachineRootNode::content::FiniteStateMachineRootNode::vfunc_22
    Replacement::draft_throws(0x14071fb10, "content_14071fb10", tpl_7307::<0x143c89f28, 0x143c89f40, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::FiniteStateMachineRootNode::content::FiniteStateMachineRootNode::vfunc_23
    Replacement::draft(0x14071fc30, "content_14071fc30", fn_14071fc30 as *const ()), // sub_14071fc30
    Replacement::draft(0x14071fcd0, "content_14071fcd0", fn_14071fcd0 as *const ()), // sub_14071fcd0
    Replacement::draft(0x14071fd90, "content_14071fd90", tpl_7312::<0x14387cc48, 0x14243b0b0, 0x140721130, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14071fd90
    Replacement::draft(0x14071fe00, "content_14071fe00", fn_14071fe00 as *const ()), // sub_14071fe00
    Replacement::draft_throws(0x14071ff00, "content_14071ff00", fn_14071ff00 as *const (), Throws::Discard), // sub_14071ff00
    Replacement::draft_throws(0x14071fff0, "content_14071fff0", fn_14071fff0 as *const (), Throws::Discard), // sub_14071fff0
    Replacement::draft_throws(0x140720090, "content_140720090", fn_140720090 as *const (), Throws::Discard), // sub_140720090
    Replacement::draft_throws(0x140720190, "content_140720190", fn_140720190 as *const (), Throws::Discard), // sub_140720190
    Replacement::draft_throws(0x140720210, "content_140720210", fn_140720210 as *const (), Throws::Caught), // sub_140720210
    Replacement::draft_throws(0x140720370, "content_140720370", fn_140720370 as *const (), Throws::Caught), // sub_140720370
    Replacement::draft_throws(0x1407204d0, "content_1407204d0", fn_1407204d0 as *const (), Throws::Caught), // sub_1407204d0
    Replacement::draft(0x140720630, "content_140720630", fn_140720630 as *const ()), // sub_140720630
    Replacement::draft(0x1407206b0, "content_1407206b0", fn_1407206b0 as *const ()), // sub_1407206b0
    Replacement::draft(0x140720750, "content_140720750", tpl_7320::<0x14387cc48, 0x14243b0b0, 0x1407219e0, 0x338, 0x18, 0xe8, 0x1c, 0x20, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140720750
    Replacement::draft_throws(0x140720800, "content_140720800", fn_140720800 as *const (), Throws::Discard), // sub_140720800
    Replacement::draft_throws(0x1407208b0, "content_1407208b0", fn_1407208b0 as *const (), Throws::Discard), // sub_1407208b0
    Replacement::draft(0x140720a00, "content_140720a00", fn_140720a00 as *const ()), // sub_140720a00
    Replacement::draft(0x140720a90, "content_140720a90", fn_140720a90 as *const ()), // sub_140720a90
    Replacement::draft_throws(0x140720c90, "content_140720c90", fn_140720c90 as *const (), Throws::Caught), // sub_140720c90
    Replacement::draft_throws(0x140720d30, "content_140720d30", fn_140720d30 as *const (), Throws::Caught), // sub_140720d30
    Replacement::draft_throws(0x140720dd0, "content_140720dd0", fn_140720dd0 as *const (), Throws::Caught), // sub_140720dd0
    Replacement::draft_throws(0x140720f50, "content_140720f50", tpl_7326::<0x14387cc38, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140721b20> as *const (), Throws::Discard), // sub_140720f50
    Replacement::draft(0x140720ff0, "content_140720ff0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14324a638, 0x8> as *const ()), // sub_140720ff0
    Replacement::draft(0x140721080, "content_140721080", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324a638, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140721080
    Replacement::draft(0x140721130, "content_140721130", fn_140721130 as *const ()), // sub_140721130
    Replacement::draft(0x140721210, "content_140721210", fn_140721210 as *const ()), // sub_140721210
    Replacement::draft_throws(0x140721340, "content_140721340", fn_140721340 as *const (), Throws::Discard), // sub_140721340
    Replacement::draft_throws(0x140721420, "content_140721420", fn_140721420 as *const (), Throws::Discard), // sub_140721420
    Replacement::draft_throws(0x140721500, "content_140721500", fn_140721500 as *const (), Throws::Discard), // sub_140721500
    Replacement::draft_throws(0x140721600, "content_140721600", fn_140721600 as *const (), Throws::Caught), // sub_140721600
    Replacement::draft_throws(0x1407216c0, "content_1407216c0", fn_1407216c0 as *const (), Throws::Caught), // sub_1407216c0
    Replacement::draft_throws(0x140721780, "content_140721780", fn_140721780 as *const (), Throws::Caught), // sub_140721780
    Replacement::draft(0x140721840, "content_140721840", fn_140721840 as *const ()), // sub_140721840
    Replacement::draft(0x140721950, "content_140721950", fn_140721950 as *const ()), // sub_140721950
    Replacement::draft(0x1407219e0, "content_1407219e0", fn_1407219e0 as *const ()), // sub_1407219e0
    Replacement::draft_throws(0x140721a90, "content_140721a90", fn_140721a90 as *const (), Throws::Discard), // sub_140721a90
    Replacement::draft_throws(0x140721b20, "content_140721b20", fn_140721b20 as *const (), Throws::Discard), // sub_140721b20
    Replacement::draft(0x140721c00, "content_140721c00", fn_140721c00 as *const ()), // sub_140721c00
    Replacement::draft(0x140721ca0, "content_140721ca0", fn_140721ca0 as *const ()), // sub_140721ca0
    Replacement::draft(0x140721d50, "content_140721d50", fn_140721d50 as *const ()), // sub_140721d50
    Replacement::draft(0x140721e20, "content_140721e20", fn_140721e20 as *const ()), // sub_140721e20
    Replacement::draft_throws(0x140721f30, "content_140721f30", fn_140721f30 as *const (), Throws::Discard), // sub_140721f30
    Replacement::draft_throws(0x140721ff0, "content_140721ff0", fn_140721ff0 as *const (), Throws::Discard), // sub_140721ff0
    Replacement::draft_throws(0x1407220b0, "content_1407220b0", fn_1407220b0 as *const (), Throws::Discard), // sub_1407220b0
    Replacement::draft_throws(0x140722190, "content_140722190", fn_140722190 as *const (), Throws::Discard), // sub_140722190
    Replacement::draft_throws(0x140722300, "content_140722300", fn_140722300 as *const (), Throws::Caught), // sub_140722300
    Replacement::draft_throws(0x1407223d0, "content_1407223d0", fn_1407223d0 as *const (), Throws::Caught), // sub_1407223d0
    Replacement::draft_throws(0x1407224a0, "content_1407224a0", fn_1407224a0 as *const (), Throws::Caught), // sub_1407224a0
    Replacement::draft(0x140722570, "content_140722570", fn_140722570 as *const ()), // sub_140722570
    Replacement::draft(0x140722680, "content_140722680", fn_140722680 as *const ()), // sub_140722680
    Replacement::draft(0x140722840, "content_140722840", fn_140722840 as *const ()), // sub_140722840
    Replacement::draft_throws(0x140722a10, "content_140722a10", fn_140722a10 as *const (), Throws::Discard), // sub_140722a10
    Replacement::draft_throws(0x140722ba0, "content_140722ba0", fn_140722ba0 as *const (), Throws::Discard), // sub_140722ba0
    Replacement::draft(0x140722c70, "content_140722c70", fn_140722c70 as *const ()), // sub_140722c70
    Replacement::draft(0x140722d20, "content_140722d20", fn_140722d20 as *const ()), // sub_140722d20
    Replacement::draft(0x140722dd0, "content_140722dd0", fn_140722dd0 as *const ()), // sub_140722dd0
    Replacement::draft(0x140722e70, "content_140722e70", fn_140722e70 as *const ()), // sub_140722e70
    Replacement::draft(0x140722f30, "content_140722f30", fn_140722f30 as *const ()), // sub_140722f30
    Replacement::draft(0x140722fe0, "content_140722fe0", fn_140722fe0 as *const ()), // sub_140722fe0
    Replacement::draft_throws(0x140723110, "content_140723110", fn_140723110 as *const (), Throws::Discard), // sub_140723110
    Replacement::draft_throws(0x140723220, "content_140723220", fn_140723220 as *const (), Throws::Discard), // sub_140723220
    Replacement::draft_throws(0x140723300, "content_140723300", fn_140723300 as *const (), Throws::Discard), // sub_140723300
    Replacement::draft_throws(0x140723430, "content_140723430", fn_140723430 as *const (), Throws::Discard), // sub_140723430
    Replacement::draft_throws(0x140723570, "content_140723570", fn_140723570 as *const (), Throws::Caught), // sub_140723570
    Replacement::draft_throws(0x1407236e0, "content_1407236e0", fn_1407236e0 as *const (), Throws::Caught), // sub_1407236e0
    Replacement::draft_throws(0x140723850, "content_140723850", fn_140723850 as *const (), Throws::Caught), // sub_140723850
    Replacement::draft(0x1407239c0, "content_1407239c0", fn_1407239c0 as *const ()), // sub_1407239c0
    Replacement::draft(0x140723ad0, "content_140723ad0", fn_140723ad0 as *const ()), // sub_140723ad0
    Replacement::draft(0x140723c50, "content_140723c50", fn_140723c50 as *const ()), // sub_140723c50
    Replacement::draft_throws(0x140723de0, "content_140723de0", fn_140723de0 as *const (), Throws::Discard), // sub_140723de0
    Replacement::draft_throws(0x140723f40, "content_140723f40", fn_140723f40 as *const (), Throws::Discard), // sub_140723f40
    Replacement::draft(0x140724050, "content_140724050", fn_140724050 as *const ()), // sub_140724050
    Replacement::draft(0x1407241d0, "content_1407241d0", fn_1407241d0 as *const ()), // sub_1407241d0
    Replacement::draft(0x1407242c0, "content_1407242c0", fn_1407242c0 as *const ()), // content::FiniteStateMachineConnectionNode::content::FiniteStateMachineConnectionNode::vfunc_0
    Replacement::draft(0x140724310, "content_140724310", tpl_1953::<0x1401ffd50, 0x14324ab18, 0x141834d30> as *const ()), // content::FiniteStateMachineNode::content::FiniteStateMachineNode::vfunc_0
    Replacement::draft(0x140724360, "content_140724360", fn_140724360 as *const ()), // content::FiniteStateMachineRootNode::content::FiniteStateMachineRootNode::vfunc_0
    Replacement::draft(0x1407243e0, "content_1407243e0", fn_1407243e0 as *const ()), // content::FiniteStateMachineStateNode::content::FiniteStateMachineStateNode::vfunc_0
    Replacement::draft_throws(0x140724430, "unknown_140724430", tpl_65670::<0x142442de0, 0x143a38110, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_140724430
    Replacement::draft_throws(0x140724510, "unknown_140724510", fn_140724510 as *const (), Throws::Caught), // sub_140724510
    Replacement::draft_throws(0x140724810, "unknown_140724810", fn_140724810 as *const (), Throws::Caught), // sub_140724810
    Replacement::draft_throws(0x140724b10, "unknown_140724b10", fn_140724b10 as *const (), Throws::Caught), // sub_140724b10
    Replacement::draft_throws(0x140724e10, "unknown_140724e10", fn_140724e10 as *const (), Throws::Caught), // sub_140724e10
    Replacement::draft_throws(0x140725220, "unknown_140725220", fn_140725220 as *const (), Throws::Caught), // sub_140725220
    Replacement::draft_throws(0x140725520, "unknown_140725520", tpl_65672::<0x142442de0, 0x143a38110, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140725520
    Replacement::draft_throws(0x140725740, "unknown_140725740", fn_140725740 as *const (), Throws::Caught), // sub_140725740
    Replacement::draft_throws(0x140725a40, "unknown_140725a40", fn_140725a40 as *const (), Throws::Caught), // sub_140725a40
    Replacement::draft_throws(0x140726510, "unknown_140726510", fn_140726510 as *const (), Throws::Caught), // sub_140726510
    Replacement::draft_throws(0x140726fb0, "unknown_140726fb0", fn_140726fb0 as *const (), Throws::Caught), // sub_140726fb0
    Replacement::draft_throws(0x140727a80, "unknown_140727a80", fn_140727a80 as *const (), Throws::Caught), // sub_140727a80
    Replacement::draft_throws(0x140727c80, "unknown_140727c80", fn_140727c80 as *const (), Throws::Caught), // sub_140727c80
    Replacement::draft_throws(0x140727db0, "unknown_140727db0", fn_140727db0 as *const (), Throws::Caught), // sub_140727db0
    Replacement::draft_throws(0x140727f60, "unknown_140727f60", fn_140727f60 as *const (), Throws::Caught), // sub_140727f60
    Replacement::draft(0x140728030, "unknown_140728030", fn_140728030 as *const ()), // sub_140728030
    Replacement::draft_throws(0x1407280e0, "unknown_1407280e0", fn_1407280e0 as *const (), Throws::Caught), // sub_1407280e0
    Replacement::draft_throws(0x140728170, "unknown_140728170", tpl_1908::<0x14387e740, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140728170
    Replacement::draft_throws(0x1407281f0, "unknown_1407281f0", fn_1407281f0 as *const (), Throws::Caught), // sub_1407281f0
    Replacement::draft_throws(0x140728510, "unknown_140728510", tpl_66331::<0x142442de0, 0x143a38180, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140728510
    Replacement::draft_throws(0x140728770, "unknown_140728770", fn_140728770 as *const (), Throws::Caught), // sub_140728770
    Replacement::draft_throws(0x140728a90, "unknown_140728a90", fn_140728a90 as *const (), Throws::Caught), // sub_140728a90
    Replacement::draft_throws(0x140728d80, "unknown_140728d80", fn_140728d80 as *const (), Throws::Caught), // sub_140728d80
    Replacement::draft_throws(0x140728ec0, "unknown_140728ec0", fn_140728ec0 as *const (), Throws::Caught), // sub_140728ec0
    Replacement::draft_throws(0x140729180, "unknown_140729180", fn_140729180 as *const (), Throws::Caught), // sub_140729180
    Replacement::draft_throws(0x140729440, "unknown_140729440", fn_140729440 as *const (), Throws::Caught), // sub_140729440
    Replacement::draft_throws(0x140729510, "unknown_140729510", fn_140729510 as *const (), Throws::Caught), // sub_140729510
    Replacement::draft_throws(0x140729830, "unknown_140729830", tpl_66331::<0x142442de0, 0x143a381f0, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140729830
    Replacement::draft_throws(0x140729a90, "unknown_140729a90", fn_140729a90 as *const (), Throws::Caught), // sub_140729a90
    Replacement::draft_throws(0x140729db0, "unknown_140729db0", fn_140729db0 as *const (), Throws::Caught), // sub_140729db0
    Replacement::draft_throws(0x14072a0a0, "unknown_14072a0a0", fn_14072a0a0 as *const (), Throws::Caught), // sub_14072a0a0
    Replacement::draft_throws(0x14072a620, "unknown_14072a620", fn_14072a620 as *const (), Throws::Caught), // sub_14072a620
    Replacement::draft_throws(0x14072a8e0, "unknown_14072a8e0", fn_14072a8e0 as *const (), Throws::Caught), // sub_14072a8e0
    Replacement::draft_throws(0x14072ae30, "unknown_14072ae30", fn_14072ae30 as *const (), Throws::Caught), // sub_14072ae30
    Replacement::draft_throws(0x14072b0f0, "unknown_14072b0f0", fn_14072b0f0 as *const (), Throws::Caught), // sub_14072b0f0
    Replacement::draft_throws(0x14072b530, "unknown_14072b530", fn_14072b530 as *const (), Throws::Caught), // sub_14072b530
    Replacement::draft_throws(0x14072b600, "unknown_14072b600", tpl_7370::<0x1401ffe00, 0x140708f60, 0x1407333d0, 0x142442de0, 0x143a381f0, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_14072b600
    Replacement::draft_throws(0x14072b8a0, "unknown_14072b8a0", tpl_7370::<0x1401ffe00, 0x140708f60, 0x1407332f0, 0x142442de0, 0x143a38180, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_14072b8a0
    Replacement::draft_throws(0x14072bb40, "unknown_14072bb40", fn_14072bb40 as *const (), Throws::Caught), // sub_14072bb40
    Replacement::draft_throws(0x14072c010, "unknown_14072c010", fn_14072c010 as *const (), Throws::Caught), // sub_14072c010
    Replacement::draft_throws(0x14072c5c0, "unknown_14072c5c0", fn_14072c5c0 as *const (), Throws::Caught), // sub_14072c5c0
    Replacement::draft_throws(0x14072ca40, "unknown_14072ca40", fn_14072ca40 as *const (), Throws::Caught), // sub_14072ca40
    Replacement::draft_throws(0x14072cec0, "unknown_14072cec0", fn_14072cec0 as *const (), Throws::Caught), // sub_14072cec0
    Replacement::draft(0x14072d0c0, "unknown_14072d0c0", fn_14072d0c0 as *const ()), // sub_14072d0c0
    Replacement::draft_throws(0x14072d280, "unknown_14072d280", fn_14072d280 as *const (), Throws::Caught), // sub_14072d280
    Replacement::draft(0x14072d430, "unknown_14072d430", fn_14072d430 as *const ()), // sub_14072d430
    Replacement::draft(0x14072d5c0, "unknown_14072d5c0", fn_14072d5c0 as *const ()), // sub_14072d5c0
    Replacement::draft_throws(0x14072d700, "unknown_14072d700", fn_14072d700 as *const (), Throws::Caught), // sub_14072d700
    Replacement::draft_throws(0x14072d8b0, "unknown_14072d8b0", fn_14072d8b0 as *const (), Throws::Caught), // sub_14072d8b0
    Replacement::draft_throws(0x14072dd80, "unknown_14072dd80", fn_14072dd80 as *const (), Throws::Caught), // sub_14072dd80
    Replacement::draft_throws(0x14072e200, "unknown_14072e200", fn_14072e200 as *const (), Throws::Caught), // sub_14072e200
    Replacement::draft(0x14072e680, "unknown_14072e680", fn_14072e680 as *const ()), // sub_14072e680
    Replacement::draft(0x14072e7f0, "unknown_14072e7f0", fn_14072e7f0 as *const ()), // sub_14072e7f0
    Replacement::draft_throws(0x14072e940, "unknown_14072e940", fn_14072e940 as *const (), Throws::Caught), // sub_14072e940
    Replacement::draft_throws(0x14072ea70, "unknown_14072ea70", fn_14072ea70 as *const (), Throws::Caught), // sub_14072ea70
    Replacement::draft(0x14072ec70, "unknown_14072ec70", fn_14072ec70 as *const ()), // sub_14072ec70
    Replacement::draft_throws(0x14072ed10, "unknown_14072ed10", fn_14072ed10 as *const (), Throws::Caught), // sub_14072ed10
    Replacement::draft_throws(0x14072f7d0, "unknown_14072f7d0", fn_14072f7d0 as *const (), Throws::Caught), // sub_14072f7d0
    Replacement::draft_throws(0x140730260, "unknown_140730260", fn_140730260 as *const (), Throws::Caught), // sub_140730260
    Replacement::draft_throws(0x140730ce0, "unknown_140730ce0", fn_140730ce0 as *const (), Throws::Caught), // sub_140730ce0
    Replacement::draft_throws(0x1407317a0, "unknown_1407317a0", fn_1407317a0 as *const (), Throws::Caught), // sub_1407317a0
    Replacement::draft_throws(0x140731930, "unknown_140731930", fn_140731930 as *const (), Throws::Caught), // sub_140731930
    Replacement::draft_throws(0x1407323c0, "unknown_1407323c0", fn_1407323c0 as *const (), Throws::Caught), // sub_1407323c0
    Replacement::draft_throws(0x140732550, "unknown_140732550", fn_140732550 as *const (), Throws::Caught), // sub_140732550
    Replacement::draft_throws(0x140732fd0, "unknown_140732fd0", fn_140732fd0 as *const (), Throws::Caught), // sub_140732fd0
    Replacement::draft_throws(0x140733160, "unknown_140733160", fn_140733160 as *const (), Throws::Caught), // sub_140733160
    Replacement::draft_throws(0x1407332f0, "unknown_1407332f0", tpl_65670::<0x142442de0, 0x143a38180, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1407332f0
    Replacement::draft_throws(0x1407333d0, "unknown_1407333d0", tpl_65670::<0x142442de0, 0x143a381f0, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1407333d0
    Replacement::draft(0x140733880, "content_140733880", fn_140733880 as *const ()), // content::EQSTask::content::EQSTask::vfunc_1
    Replacement::draft(0x140733900, "content_140733900", tpl_7386::<0x3, 0x4, 0x8, 0x14322ad80, 0x142409b50> as *const ()), // content::EQSAllyInLoSTask::content::EQSAllyInLoSTask::vfunc_1
    Replacement::draft_throws(0x140733960, "content_140733960", tpl_6639::<0x143cc2fb8, 0x143cc2fd0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSAllyInLoSTask::content::EQSAllyInLoSTask::vfunc_2
    Replacement::draft(0x140733ab0, "content_140733ab0", tpl_7387::<0x3, 0x4, 0x8, 0x14322aab8, 0x142409b50> as *const ()), // content::EQSAngleCompareTask::content::EQSAngleCompareTask::vfunc_1
    Replacement::draft_throws(0x140733b10, "content_140733b10", tpl_6639::<0x143cc30d8, 0x143cc3000, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSAngleCompareTask::content::EQSAngleCompareTask::vfunc_2
    Replacement::draft(0x140733c60, "content_140733c60", tpl_7388::<0x3, 0x4, 0x8, 0x143229480, 0x142409b50> as *const ()), // content::EQSGeneratorBaseNode::content::EQSGeneratorBaseNode::vfunc_22
    Replacement::draft_throws(0x140733cc0, "content_140733cc0", tpl_7307::<0x143cc3034, 0x143cc3050, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSGeneratorBaseNode::content::EQSGeneratorBaseNode::vfunc_23
    Replacement::draft(0x140733de0, "content_140733de0", tpl_7389::<0x3, 0x4, 0x8, 0x14322a7f0, 0x142409b50> as *const ()), // content::EQSCoverPositionGenerator::content::EQSCoverPositionGenerator::vfunc_22
    Replacement::draft_throws(0x140733e40, "content_140733e40", tpl_7307::<0x143cc3058, 0x143cc2fb0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSCoverPositionGenerator::content::EQSCoverPositionGenerator::vfunc_23
    Replacement::draft(0x140733f60, "content_140733f60", tpl_7387::<0x3, 0x4, 0x8, 0x14322a528, 0x142409b50> as *const ()), // content::EQSCoverScoringTask::content::EQSCoverScoringTask::vfunc_1
    Replacement::draft_throws(0x140733fc0, "content_140733fc0", tpl_6639::<0x143cc2fe0, 0x143cc30d0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSCoverScoringTask::content::EQSCoverScoringTask::vfunc_2
    Replacement::draft(0x1407340e0, "content_1407340e0", tpl_7388::<0x3, 0x4, 0x8, 0x14322a268, 0x142409b50> as *const ()), // content::EQSDiscGeneratorNode::content::EQSDiscGeneratorNode::vfunc_22
    Replacement::draft_throws(0x140734140, "content_140734140", tpl_7307::<0x143cc304c, 0x143cc3040, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSDiscGeneratorNode::content::EQSDiscGeneratorNode::vfunc_23
    Replacement::draft(0x1407342c0, "content_1407342c0", tpl_7390::<0x3, 0x4, 0x8, 0x143229fa0, 0x142409b50> as *const ()), // content::EQSDotCompareTask::content::EQSDotCompareTask::vfunc_1
    Replacement::draft_throws(0x140734320, "content_140734320", tpl_6639::<0x143cc3068, 0x143cc2fe8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSDotCompareTask::content::EQSDotCompareTask::vfunc_2
    Replacement::draft(0x140734440, "content_140734440", tpl_7309::<0x3, 0x4, 0x8, 0x143229f70, 0x142409b50> as *const ()), // content::EQSDynamicGridGeneratorNode::content::EQSDynamicGridGeneratorNode::vfunc_22
    Replacement::draft_throws(0x1407344a0, "content_1407344a0", tpl_7307::<0x143cc30cc, 0x143cc3060, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSDynamicGridGeneratorNode::content::EQSDynamicGridGeneratorNode::vfunc_23
    Replacement::draft(0x1407345c0, "content_1407345c0", tpl_7386::<0x3, 0x4, 0x8, 0x143229a10, 0x142409b50> as *const ()), // content::EQSElevationTask::content::EQSElevationTask::vfunc_1
    Replacement::draft_throws(0x140734620, "content_140734620", tpl_6639::<0x143cc3048, 0x143cc2ff0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSElevationTask::content::EQSElevationTask::vfunc_2
    Replacement::draft(0x140734740, "content_140734740", tpl_7391::<0x3, 0x4, 0x8, 0x143229740, 0x142409b50> as *const ()), // content::EQSEuclideanDistanceTask::content::EQSEuclideanDistanceTask::vfunc_1
    Replacement::draft_throws(0x1407347a0, "content_1407347a0", tpl_6639::<0x143cc3038, 0x143cc30a0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSEuclideanDistanceTask::content::EQSEuclideanDistanceTask::vfunc_2
    Replacement::draft(0x140734890, "content_140734890", tpl_7392 as *const ()), // content::EQSEuclideanDistanceTask::content::EQSEuclideanDistanceTask::vfunc_23
    Replacement::draft_throws(0x1407348d0, "content_1407348d0", tpl_6639::<0x143cc3030, 0x143cc3078, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSEuclideanLinearDistanceTask::content::EQSEuclideanLinearDistanceTask::vfunc_2
    Replacement::draft(0x1407349c0, "content_1407349c0", tpl_586::<0x90> as *const ()), // content::EQSEuclideanLinearDistanceTask::content::EQSEuclideanLinearDistanceTask::vfunc_23
    Replacement::draft(0x1407349d0, "content_1407349d0", tpl_586::<0x94> as *const ()), // content::EQSEuclideanLinearDistanceTask::content::EQSEuclideanLinearDistanceTask::vfunc_24
    Replacement::draft(0x1407349e0, "content_1407349e0", tpl_586::<0x98> as *const ()), // content::EQSEuclideanLinearDistanceTask::content::EQSEuclideanLinearDistanceTask::vfunc_25
    Replacement::draft(0x1407349f0, "content_1407349f0", tpl_586::<0x9c> as *const ()), // content::EQSEuclideanPreferredDistanceTask::content::EQSEuclideanPreferredDistanceTask::vfunc_25
    Replacement::draft_throws(0x140734a30, "content_140734a30", tpl_6639::<0x143cc30a8, 0x143cc2fd8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSEuclideanPreferredDistanceTask::content::EQSEuclideanPreferredDistanceTask::vfunc_2
    Replacement::draft(0x140734b20, "content_140734b20", fn_140734b20 as *const ()), // content::EQSEuclideanPreferredDistanceTask::content::EQSEuclideanPreferredDistanceTask::vfunc_23
    Replacement::draft(0x140734b40, "content_140734b40", fn_140734b40 as *const ()), // content::EQSEuclideanPreferredDistanceTask::content::EQSEuclideanPreferredDistanceTask::vfunc_24
    Replacement::draft(0x140734bc0, "content_140734bc0", tpl_7395::<0x3, 0x4, 0x8, 0x1432291b8, 0x142409b50> as *const ()), // content::EQSHazardFieldTask::content::EQSHazardFieldTask::vfunc_1
    Replacement::draft_throws(0x140734c20, "content_140734c20", tpl_6639::<0x143cc302c, 0x143cc2fc8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSHazardFieldTask::content::EQSHazardFieldTask::vfunc_2
    Replacement::draft(0x140734d40, "content_140734d40", tpl_7396::<0x3, 0x4, 0x8, 0x143228ef8, 0x142409b50> as *const ()), // content::EQSHeightTask::content::EQSHeightTask::vfunc_1
    Replacement::draft_throws(0x140734da0, "content_140734da0", tpl_6639::<0x143cc2ffc, 0x143cc3010, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSHeightTask::content::EQSHeightTask::vfunc_2
    Replacement::draft(0x140734ec0, "content_140734ec0", tpl_7305::<0x3, 0x4, 0x8, 0x143228c30, 0x142409b50> as *const ()), // content::EQSInvalidateCoverTask::content::EQSInvalidateCoverTask::vfunc_1
    Replacement::draft_throws(0x140734f20, "content_140734f20", tpl_6639::<0x143cc30c8, 0x143cc3090, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSInvalidateCoverTask::content::EQSInvalidateCoverTask::vfunc_2
    Replacement::draft(0x140735040, "content_140735040", tpl_7397::<0x3, 0x4, 0x8, 0x143228950, 0x142409b50> as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::content::EQSKeepDistanceFromReservedPositionsTask::vfunc_1
    Replacement::draft_throws(0x1407350a0, "content_1407350a0", tpl_6639::<0x143cc3028, 0x143cc30c0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSKeepDistanceFromReservedPositionsTask::content::EQSKeepDistanceFromReservedPositionsTask::vfunc_2
    Replacement::draft(0x1407351c0, "content_1407351c0", tpl_7395::<0x3, 0x4, 0x8, 0x143228688, 0x142409b50> as *const ()), // content::EQSLineOfSightTask::content::EQSLineOfSightTask::vfunc_1
    Replacement::draft_throws(0x140735220, "content_140735220", tpl_6639::<0x143cc2ff8, 0x143cc3070, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSLineOfSightTask::content::EQSLineOfSightTask::vfunc_2
    Replacement::draft(0x140735370, "content_140735370", tpl_7398::<0x3, 0x4, 0x8, 0x143227eb0, 0x142409b50> as *const ()), // content::EQSQueryService::content::EQSQueryService::vfunc_1
    Replacement::draft_throws(0x1407353d0, "content_1407353d0", tpl_6639::<0x143cc3018, 0x143cc3008, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSQueryService::content::EQSQueryService::vfunc_2
    Replacement::draft(0x1407354f0, "content_1407354f0", tpl_7399::<0x3, 0x4, 0x8, 0x143227be0, 0x142409b50> as *const ()), // content::EQSRandomEuclideanDistanceTask::content::EQSRandomEuclideanDistanceTask::vfunc_1
    Replacement::draft_throws(0x140735550, "content_140735550", tpl_6639::<0x143cc30b8, 0x143cc30b0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSRandomEuclideanDistanceTask::content::EQSRandomEuclideanDistanceTask::vfunc_2
    Replacement::draft(0x140735670, "content_140735670", tpl_7400::<0x3, 0x4, 0x8, 0x143227910, 0x142409b50> as *const ()), // content::EQSRaycastToTargetOnNavmeshTask::content::EQSRaycastToTargetOnNavmeshTask::vfunc_1
    Replacement::draft_throws(0x1407356d0, "content_1407356d0", tpl_6639::<0x143cc3098, 0x143cc2fc0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSRaycastToTargetOnNavmeshTask::content::EQSRaycastToTargetOnNavmeshTask::vfunc_2
    Replacement::draft(0x1407357f0, "content_1407357f0", tpl_7401::<0x3, 0x4, 0x8, 0x143227668, 0x142409b50> as *const ()), // content::EQSRootNode::content::EQSRootNode::vfunc_22
    Replacement::draft_throws(0x140735850, "content_140735850", tpl_7307::<0x143c89f68, 0x143c89f60, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSRootNode::content::EQSRootNode::vfunc_23
    Replacement::draft(0x140735970, "content_140735970", tpl_7402::<0x3, 0x4, 0x8, 0x143227648, 0x142409b50> as *const ()), // content::EQSScoreNode::content::EQSScoreNode::vfunc_22
    Replacement::draft_throws(0x1407359d0, "content_1407359d0", tpl_7307::<0x143cc3084, 0x143cc3020, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSScoreNode::content::EQSScoreNode::vfunc_23
    Replacement::draft(0x140735af0, "content_140735af0", tpl_7305::<0x3, 0x4, 0x8, 0x143227100, 0x142409b50> as *const ()), // content::EQSSphereGeneratorNode::content::EQSSphereGeneratorNode::vfunc_22
    Replacement::draft_throws(0x140735b50, "content_140735b50", tpl_7307::<0x143cc3080, 0x143cc3088, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSSphereGeneratorNode::content::EQSSphereGeneratorNode::vfunc_23
    Replacement::draft(0x140735c70, "unknown_140735c70", fn_140735c70 as *const ()), // sub_140735c70
    Replacement::draft(0x140735d00, "unknown_140735d00", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324cb68, 0x18> as *const ()), // sub_140735d00
    Replacement::draft(0x140735db0, "unknown_140735db0", tpl_7312::<0x14387cd00, 0x14243b0b0, 0x140758e00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140735db0
    Replacement::draft(0x140735e20, "unknown_140735e20", tpl_7404::<0x14387cd00, 0x14243b0b0, 0x140758e80, 0x338, 0x10, 0x1426d4580, 0x1, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140735e20
    Replacement::draft_throws(0x140735f00, "unknown_140735f00", fn_140735f00 as *const (), Throws::Discard), // sub_140735f00
    Replacement::draft(0x140735fc0, "unknown_140735fc0", fn_140735fc0 as *const ()), // sub_140735fc0
    Replacement::draft_throws(0x140736050, "unknown_140736050", fn_140736050 as *const (), Throws::Discard), // sub_140736050
    Replacement::draft(0x140736120, "unknown_140736120", fn_140736120 as *const ()), // sub_140736120
    Replacement::draft_throws(0x1407361b0, "unknown_1407361b0", fn_1407361b0 as *const (), Throws::Caught), // sub_1407361b0
    Replacement::draft_throws(0x1407363f0, "unknown_1407363f0", fn_1407363f0 as *const (), Throws::Caught), // sub_1407363f0
    Replacement::draft_throws(0x140736600, "unknown_140736600", fn_140736600 as *const (), Throws::Caught), // sub_140736600
    Replacement::draft(0x140736810, "unknown_140736810", fn_140736810 as *const ()), // sub_140736810
    Replacement::draft(0x1407368a0, "unknown_1407368a0", tpl_7413::<0x14387cd38, 0x14387cd00, 0x1401e9970, 0x140759f40, 0x338, 0x10, 0xd8, 0x14, 0xe8, 0x1401e9860> as *const ()), // sub_1407368a0
    Replacement::draft(0x140736930, "unknown_140736930", tpl_7414::<0x14387cd00, 0x14243b0b0, 0x14075a030, 0x338, 0x10, 0xd8, 0x14, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140736930
    Replacement::draft_throws(0x1407369d0, "unknown_1407369d0", tpl_7415::<0x14387cd38, 0x14387cd00, 0x1401e9770, 0x14075a130, 0x338, 0x10, 0x120, 0x4, 0x14, 0x1401e9590> as *const (), Throws::Discard), // sub_1407369d0
    Replacement::draft_throws(0x140736a70, "unknown_140736a70", fn_140736a70 as *const (), Throws::Discard), // sub_140736a70
    Replacement::draft(0x140736b70, "unknown_140736b70", fn_140736b70 as *const ()), // sub_140736b70
    Replacement::draft(0x140736c10, "unknown_140736c10", fn_140736c10 as *const ()), // sub_140736c10
    Replacement::draft(0x140736ca0, "unknown_140736ca0", fn_140736ca0 as *const ()), // sub_140736ca0
    Replacement::draft(0x140736d50, "unknown_140736d50", fn_140736d50 as *const ()), // sub_140736d50
    Replacement::draft(0x140736ec0, "unknown_140736ec0", fn_140736ec0 as *const ()), // sub_140736ec0
    Replacement::draft_throws(0x1407370a0, "unknown_1407370a0", fn_1407370a0 as *const (), Throws::Discard), // sub_1407370a0
    Replacement::draft(0x140737280, "unknown_140737280", fn_140737280 as *const ()), // sub_140737280
    Replacement::draft_throws(0x1407373a0, "unknown_1407373a0", fn_1407373a0 as *const (), Throws::Discard), // sub_1407373a0
    Replacement::draft(0x1407375c0, "unknown_1407375c0", fn_1407375c0 as *const ()), // sub_1407375c0
    Replacement::draft_throws(0x1407377a0, "unknown_1407377a0", fn_1407377a0 as *const (), Throws::Caught), // sub_1407377a0
    Replacement::draft_throws(0x140737b80, "unknown_140737b80", fn_140737b80 as *const (), Throws::Caught), // sub_140737b80
    Replacement::draft_throws(0x140737f20, "unknown_140737f20", fn_140737f20 as *const (), Throws::Caught), // sub_140737f20
    Replacement::draft(0x1407382c0, "unknown_1407382c0", fn_1407382c0 as *const ()), // sub_1407382c0
    Replacement::draft(0x140738410, "unknown_140738410", fn_140738410 as *const ()), // sub_140738410
    Replacement::draft(0x140738540, "unknown_140738540", fn_140738540 as *const ()), // sub_140738540
    Replacement::draft_throws(0x140738680, "unknown_140738680", fn_140738680 as *const (), Throws::Discard), // sub_140738680
    Replacement::draft_throws(0x1407387e0, "unknown_1407387e0", fn_1407387e0 as *const (), Throws::Discard), // sub_1407387e0
    Replacement::draft(0x140738a40, "unknown_140738a40", fn_140738a40 as *const ()), // sub_140738a40
    Replacement::draft(0x140738b40, "unknown_140738b40", tpl_7434::<0x130, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14075a6e0> as *const ()), // sub_140738b40
    Replacement::draft(0x140738bd0, "unknown_140738bd0", tpl_66380::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x14075a6e0> as *const ()), // sub_140738bd0
    Replacement::draft(0x140738c80, "unknown_140738c80", fn_140738c80 as *const ()), // sub_140738c80
    Replacement::draft(0x140738d80, "unknown_140738d80", fn_140738d80 as *const ()), // sub_140738d80
    Replacement::draft_throws(0x140738ec0, "unknown_140738ec0", fn_140738ec0 as *const (), Throws::Discard), // sub_140738ec0
    Replacement::draft_throws(0x140739010, "unknown_140739010", fn_140739010 as *const (), Throws::Discard), // sub_140739010
    Replacement::draft_throws(0x1407390e0, "unknown_1407390e0", fn_1407390e0 as *const (), Throws::Discard), // sub_1407390e0
    Replacement::draft_throws(0x140739250, "unknown_140739250", fn_140739250 as *const (), Throws::Discard), // sub_140739250
    Replacement::draft_throws(0x140739360, "unknown_140739360", fn_140739360 as *const (), Throws::Caught), // sub_140739360
    Replacement::draft_throws(0x140739620, "unknown_140739620", fn_140739620 as *const (), Throws::Caught), // sub_140739620
    Replacement::draft_throws(0x1407398b0, "unknown_1407398b0", fn_1407398b0 as *const (), Throws::Caught), // sub_1407398b0
    Replacement::draft(0x140739b30, "unknown_140739b30", fn_140739b30 as *const ()), // sub_140739b30
    Replacement::draft(0x140739c20, "unknown_140739c20", fn_140739c20 as *const ()), // sub_140739c20
    Replacement::draft(0x140739d20, "unknown_140739d20", fn_140739d20 as *const ()), // sub_140739d20
    Replacement::draft_throws(0x140739e00, "unknown_140739e00", fn_140739e00 as *const (), Throws::Discard), // sub_140739e00
    Replacement::draft_throws(0x140739f10, "unknown_140739f10", fn_140739f10 as *const (), Throws::Discard), // sub_140739f10
    Replacement::draft(0x14073a0a0, "unknown_14073a0a0", tpl_7449::<0x140753b60, 0x14384e350, 0x30, 0x1426cdcb0, 0x14324fd00, 0x1401de410> as *const ()), // sub_14073a0a0
    Replacement::draft(0x14073a100, "unknown_14073a100", fn_14073a100 as *const ()), // sub_14073a100
    Replacement::draft(0x14073a1d0, "unknown_14073a1d0", tpl_7449::<0x140753c00, 0x14384e350, 0x30, 0x1426cdcb0, 0x14324fd00, 0x1401de410> as *const ()), // sub_14073a1d0
    Replacement::draft(0x14073a230, "unknown_14073a230", fn_14073a230 as *const ()), // sub_14073a230
    Replacement::draft(0x14073a320, "unknown_14073a320", tpl_66397::<0x142535360, 0x1426456c8, 0x1426453a0, 0x14324b6d0, 0x14183acc2> as *const ()), // sub_14073a320
    Replacement::draft(0x14073a450, "unknown_14073a450", tpl_7452::<0x14387cd08, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x28, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14073a450
    Replacement::draft(0x14073a4f0, "unknown_14073a4f0", fn_14073a4f0 as *const ()), // sub_14073a4f0
    Replacement::draft_throws(0x14073a680, "unknown_14073a680", fn_14073a680 as *const (), Throws::Discard), // sub_14073a680
    Replacement::draft(0x14073a7d0, "unknown_14073a7d0", fn_14073a7d0 as *const ()), // sub_14073a7d0
    Replacement::draft_throws(0x14073a8a0, "unknown_14073a8a0", fn_14073a8a0 as *const (), Throws::Discard), // sub_14073a8a0
    Replacement::draft(0x14073aa20, "unknown_14073aa20", fn_14073aa20 as *const ()), // sub_14073aa20
    Replacement::draft_throws(0x14073ab30, "unknown_14073ab30", fn_14073ab30 as *const (), Throws::Caught), // sub_14073ab30
    Replacement::draft_throws(0x14073ae50, "unknown_14073ae50", fn_14073ae50 as *const (), Throws::Caught), // sub_14073ae50
    Replacement::draft_throws(0x14073b140, "unknown_14073b140", fn_14073b140 as *const (), Throws::Caught), // sub_14073b140
    Replacement::draft(0x14073b420, "unknown_14073b420", fn_14073b420 as *const ()), // sub_14073b420
    Replacement::draft(0x14073b500, "unknown_14073b500", fn_14073b500 as *const ()), // sub_14073b500
    Replacement::draft(0x14073b5e0, "unknown_14073b5e0", fn_14073b5e0 as *const ()), // sub_14073b5e0
    Replacement::draft_throws(0x14073b6d0, "unknown_14073b6d0", fn_14073b6d0 as *const (), Throws::Discard), // sub_14073b6d0
    Replacement::draft_throws(0x14073b7d0, "unknown_14073b7d0", fn_14073b7d0 as *const (), Throws::Discard), // sub_14073b7d0
    Replacement::draft(0x14073b990, "unknown_14073b990", fn_14073b990 as *const ()), // sub_14073b990
    Replacement::draft(0x14073ba90, "unknown_14073ba90", fn_14073ba90 as *const ()), // sub_14073ba90
    Replacement::draft(0x14073bbb0, "unknown_14073bbb0", fn_14073bbb0 as *const ()), // sub_14073bbb0
    Replacement::draft(0x14073bcf0, "unknown_14073bcf0", tpl_7468::<0x14387cdb8, 0x14243b0b0, 0x14074a3e0, 0x14387d230, 0x28, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14073bcf0
    Replacement::draft(0x14073bd90, "unknown_14073bd90", fn_14073bd90 as *const ()), // sub_14073bd90
    Replacement::draft_throws(0x14073bec0, "unknown_14073bec0", fn_14073bec0 as *const (), Throws::Discard), // sub_14073bec0
    Replacement::draft_throws(0x14073c000, "unknown_14073c000", fn_14073c000 as *const (), Throws::Discard), // sub_14073c000
    Replacement::draft_throws(0x14073c0b0, "unknown_14073c0b0", fn_14073c0b0 as *const (), Throws::Discard), // sub_14073c0b0
    Replacement::draft_throws(0x14073c200, "unknown_14073c200", fn_14073c200 as *const (), Throws::Discard), // sub_14073c200
    Replacement::draft_throws(0x14073c2b0, "unknown_14073c2b0", fn_14073c2b0 as *const (), Throws::Caught), // sub_14073c2b0
    Replacement::draft_throws(0x14073c560, "unknown_14073c560", fn_14073c560 as *const (), Throws::Caught), // sub_14073c560
    Replacement::draft_throws(0x14073c7e0, "unknown_14073c7e0", fn_14073c7e0 as *const (), Throws::Caught), // sub_14073c7e0
    Replacement::draft(0x14073ca50, "unknown_14073ca50", fn_14073ca50 as *const ()), // sub_14073ca50
    Replacement::draft(0x14073cb10, "unknown_14073cb10", fn_14073cb10 as *const ()), // sub_14073cb10
    Replacement::draft(0x14073cc00, "unknown_14073cc00", fn_14073cc00 as *const ()), // sub_14073cc00
    Replacement::draft_throws(0x14073ccd0, "unknown_14073ccd0", fn_14073ccd0 as *const (), Throws::Discard), // sub_14073ccd0
    Replacement::draft_throws(0x14073cdd0, "unknown_14073cdd0", fn_14073cdd0 as *const (), Throws::Discard), // sub_14073cdd0
    Replacement::draft(0x14073cf50, "unknown_14073cf50", tpl_7173::<0x14324fc90, 0xa4, 0x1, 0x14020f270, 0xa8, 0x143250720> as *const ()), // sub_14073cf50
    Replacement::draft(0x14073cfd0, "unknown_14073cfd0", fn_14073cfd0 as *const ()), // sub_14073cfd0
    Replacement::draft(0x14073d0d0, "unknown_14073d0d0", fn_14073d0d0 as *const ()), // sub_14073d0d0
    Replacement::draft(0x14073d210, "unknown_14073d210", tpl_7452::<0x14387cc60, 0x14243b0b0, 0x14073e860, 0x14387d230, 0x50, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14073d210
    Replacement::draft(0x14073d2b0, "unknown_14073d2b0", fn_14073d2b0 as *const ()), // sub_14073d2b0
    Replacement::draft_throws(0x14073d400, "unknown_14073d400", fn_14073d400 as *const (), Throws::Discard), // sub_14073d400
    Replacement::draft(0x14073d530, "unknown_14073d530", fn_14073d530 as *const ()), // sub_14073d530
    Replacement::draft_throws(0x14073d5f0, "unknown_14073d5f0", fn_14073d5f0 as *const (), Throws::Discard), // sub_14073d5f0
    Replacement::draft(0x14073d740, "unknown_14073d740", fn_14073d740 as *const ()), // sub_14073d740
    Replacement::draft_throws(0x14073d810, "unknown_14073d810", fn_14073d810 as *const (), Throws::Caught), // sub_14073d810
    Replacement::draft_throws(0x14073daf0, "unknown_14073daf0", fn_14073daf0 as *const (), Throws::Caught), // sub_14073daf0
    Replacement::draft_throws(0x14073dda0, "unknown_14073dda0", fn_14073dda0 as *const (), Throws::Caught), // sub_14073dda0
    Replacement::draft(0x14073e050, "unknown_14073e050", fn_14073e050 as *const ()), // sub_14073e050
    Replacement::draft(0x14073e120, "unknown_14073e120", fn_14073e120 as *const ()), // sub_14073e120
    Replacement::draft(0x14073e1f0, "unknown_14073e1f0", fn_14073e1f0 as *const ()), // sub_14073e1f0
    Replacement::draft_throws(0x14073e2d0, "unknown_14073e2d0", fn_14073e2d0 as *const (), Throws::Discard), // sub_14073e2d0
    Replacement::draft_throws(0x14073e3b0, "unknown_14073e3b0", fn_14073e3b0 as *const (), Throws::Discard), // sub_14073e3b0
    Replacement::draft(0x14073e530, "unknown_14073e530", tpl_7496::<0x14074a030, 0x14384e320, 0x40, 0x1426cdcb0, 0x1432506f0, 0x1401de410> as *const ()), // sub_14073e530
    Replacement::draft(0x14073e5a0, "unknown_14073e5a0", fn_14073e5a0 as *const ()), // sub_14073e5a0
    Replacement::draft(0x14073e6a0, "unknown_14073e6a0", tpl_7496::<0x14074a210, 0x14384e320, 0x40, 0x1426cdcb0, 0x1432506f0, 0x1401de410> as *const ()), // sub_14073e6a0
    Replacement::draft(0x14073e710, "unknown_14073e710", fn_14073e710 as *const ()), // sub_14073e710
    Replacement::draft(0x14073e7b0, "unknown_14073e7b0", fn_14073e7b0 as *const ()), // sub_14073e7b0
    Replacement::draft(0x14073e8d0, "unknown_14073e8d0", fn_14073e8d0 as *const ()), // sub_14073e8d0
    Replacement::draft_throws(0x14073e9b0, "unknown_14073e9b0", fn_14073e9b0 as *const (), Throws::Caught), // sub_14073e9b0
    Replacement::draft(0x14073eaf0, "unknown_14073eaf0", fn_14073eaf0 as *const ()), // sub_14073eaf0
    Replacement::draft_throws(0x14073ebc0, "unknown_14073ebc0", fn_14073ebc0 as *const (), Throws::Discard), // sub_14073ebc0
    Replacement::draft_throws(0x14073ec70, "unknown_14073ec70", fn_14073ec70 as *const (), Throws::Caught), // sub_14073ec70
    Replacement::draft_throws(0x14073eef0, "unknown_14073eef0", fn_14073eef0 as *const (), Throws::Caught), // sub_14073eef0
    Replacement::draft_throws(0x14073f140, "unknown_14073f140", fn_14073f140 as *const (), Throws::Caught), // sub_14073f140
    Replacement::draft(0x14073f380, "unknown_14073f380", fn_14073f380 as *const ()), // sub_14073f380
    Replacement::draft(0x14073f4e0, "unknown_14073f4e0", fn_14073f4e0 as *const ()), // sub_14073f4e0
    Replacement::draft(0x14073f590, "unknown_14073f590", fn_14073f590 as *const ()), // sub_14073f590
    Replacement::draft_throws(0x14073f650, "unknown_14073f650", fn_14073f650 as *const (), Throws::Discard), // sub_14073f650
    Replacement::draft_throws(0x14073f700, "unknown_14073f700", fn_14073f700 as *const (), Throws::Discard), // sub_14073f700
    Replacement::draft(0x14073f810, "unknown_14073f810", tpl_7434::<0x130, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14075a7f0> as *const ()), // sub_14073f810
    Replacement::draft(0x14073f8a0, "unknown_14073f8a0", tpl_66380::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x14075a7f0> as *const ()), // sub_14073f8a0
    Replacement::draft(0x14073f950, "unknown_14073f950", tpl_7512::<0x14387cca0, 0x14243b0b0, 0x140748620, 0x14387d230, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xa8> as *const ()), // sub_14073f950
    Replacement::draft(0x14073fa40, "unknown_14073fa40", fn_14073fa40 as *const ()), // sub_14073fa40
    Replacement::draft_throws(0x14073fb40, "unknown_14073fb40", fn_14073fb40 as *const (), Throws::Discard), // sub_14073fb40
    Replacement::draft(0x14073fc30, "unknown_14073fc30", fn_14073fc30 as *const ()), // sub_14073fc30
    Replacement::draft_throws(0x14073fce0, "unknown_14073fce0", fn_14073fce0 as *const (), Throws::Discard), // sub_14073fce0
    Replacement::draft(0x14073fdf0, "unknown_14073fdf0", fn_14073fdf0 as *const ()), // sub_14073fdf0
    Replacement::draft_throws(0x14073fee0, "unknown_14073fee0", fn_14073fee0 as *const (), Throws::Caught), // sub_14073fee0
    Replacement::draft_throws(0x140740160, "unknown_140740160", fn_140740160 as *const (), Throws::Caught), // sub_140740160
    Replacement::draft_throws(0x1407403b0, "unknown_1407403b0", fn_1407403b0 as *const (), Throws::Caught), // sub_1407403b0
    Replacement::draft(0x140740600, "unknown_140740600", fn_140740600 as *const ()), // sub_140740600
    Replacement::draft(0x1407406d0, "unknown_1407406d0", fn_1407406d0 as *const ()), // sub_1407406d0
    Replacement::draft(0x140740780, "unknown_140740780", fn_140740780 as *const ()), // sub_140740780
    Replacement::draft_throws(0x140740840, "unknown_140740840", fn_140740840 as *const (), Throws::Discard), // sub_140740840
    Replacement::draft_throws(0x140740900, "unknown_140740900", fn_140740900 as *const (), Throws::Discard), // sub_140740900
    Replacement::draft(0x140740a30, "unknown_140740a30", tpl_7526::<0x14074a030, 0x14384e280, 0x124, 0x1426cdcb0, 0x1432505e8, 0x1401de410> as *const ()), // sub_140740a30
    Replacement::draft(0x140740aa0, "unknown_140740aa0", fn_140740aa0 as *const ()), // sub_140740aa0
    Replacement::draft(0x140740b50, "unknown_140740b50", tpl_7526::<0x14074a210, 0x14384e280, 0x124, 0x1426cdcb0, 0x1432505e8, 0x1401de410> as *const ()), // sub_140740b50
    Replacement::draft(0x140740bc0, "unknown_140740bc0", fn_140740bc0 as *const ()), // sub_140740bc0
    Replacement::draft(0x140740c50, "unknown_140740c50", fn_140740c50 as *const ()), // sub_140740c50
    Replacement::draft(0x140740d00, "unknown_140740d00", fn_140740d00 as *const ()), // sub_140740d00
    Replacement::draft(0x140740e70, "unknown_140740e70", fn_140740e70 as *const ()), // sub_140740e70
    Replacement::draft_throws(0x140741030, "unknown_140741030", fn_140741030 as *const (), Throws::Discard), // sub_140741030
    Replacement::draft_throws(0x140741220, "unknown_140741220", fn_140741220 as *const (), Throws::Discard), // sub_140741220
    Replacement::draft_throws(0x140741350, "unknown_140741350", fn_140741350 as *const (), Throws::Discard), // sub_140741350
    Replacement::draft_throws(0x140741570, "unknown_140741570", fn_140741570 as *const (), Throws::Discard), // sub_140741570
    Replacement::draft_throws(0x1407416d0, "unknown_1407416d0", fn_1407416d0 as *const (), Throws::Caught), // sub_1407416d0
    Replacement::draft_throws(0x140741a40, "unknown_140741a40", fn_140741a40 as *const (), Throws::Caught), // sub_140741a40
    Replacement::draft_throws(0x140741d90, "unknown_140741d90", fn_140741d90 as *const (), Throws::Caught), // sub_140741d90
    Replacement::draft(0x1407420d0, "unknown_1407420d0", fn_1407420d0 as *const ()), // sub_1407420d0
    Replacement::draft(0x140742220, "unknown_140742220", fn_140742220 as *const ()), // sub_140742220
    Replacement::draft(0x140742390, "unknown_140742390", fn_140742390 as *const ()), // sub_140742390
    Replacement::draft_throws(0x1407424d0, "unknown_1407424d0", fn_1407424d0 as *const (), Throws::Discard), // sub_1407424d0
    Replacement::draft_throws(0x140742650, "unknown_140742650", fn_140742650 as *const (), Throws::Discard), // sub_140742650
    Replacement::draft(0x140742890, "unknown_140742890", tpl_7543::<0x140753b60, 0x14384e338, 0x19c, 0x1426cdcb0, 0x143250818, 0x1401de410> as *const ()), // sub_140742890
    Replacement::draft(0x140742900, "unknown_140742900", fn_140742900 as *const ()), // sub_140742900
    Replacement::draft(0x1407429f0, "unknown_1407429f0", tpl_7543::<0x140753c00, 0x14384e338, 0x19c, 0x1426cdcb0, 0x143250818, 0x1401de410> as *const ()), // sub_1407429f0
    Replacement::draft(0x140742a60, "unknown_140742a60", tpl_7545::<0xa0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x80, 0x78, 0x90, 0x14324e2a0> as *const ()), // sub_140742a60
    Replacement::draft(0x140742b60, "unknown_140742b60", tpl_6796::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x70, 0x14324e2a0, 0xffffffffffffff90, 0xffffffffffffffb0, 0xffffffffffffffd0, 0xfffffffffffffff0> as *const ()), // sub_140742b60
    Replacement::draft(0x140742c80, "unknown_140742c80", fn_140742c80 as *const ()), // sub_140742c80
    Replacement::draft(0x140742d90, "unknown_140742d90", fn_140742d90 as *const ()), // sub_140742d90
    Replacement::draft_throws(0x140742f70, "unknown_140742f70", fn_140742f70 as *const (), Throws::Discard), // sub_140742f70
    Replacement::draft(0x140743100, "unknown_140743100", fn_140743100 as *const ()), // sub_140743100
    Replacement::draft_throws(0x140743230, "unknown_140743230", fn_140743230 as *const (), Throws::Discard), // sub_140743230
    Replacement::draft(0x1407433f0, "unknown_1407433f0", fn_1407433f0 as *const ()), // sub_1407433f0
    Replacement::draft_throws(0x1407434f0, "unknown_1407434f0", fn_1407434f0 as *const (), Throws::Caught), // sub_1407434f0
    Replacement::draft_throws(0x1407436c0, "unknown_1407436c0", fn_1407436c0 as *const (), Throws::Caught), // sub_1407436c0
    Replacement::draft_throws(0x140743890, "unknown_140743890", fn_140743890 as *const (), Throws::Caught), // sub_140743890
    Replacement::draft(0x140743a60, "unknown_140743a60", fn_140743a60 as *const ()), // sub_140743a60
    Replacement::draft(0x140743ba0, "unknown_140743ba0", fn_140743ba0 as *const ()), // sub_140743ba0
    Replacement::draft(0x140743cd0, "unknown_140743cd0", fn_140743cd0 as *const ()), // sub_140743cd0
    Replacement::draft_throws(0x140743e10, "unknown_140743e10", fn_140743e10 as *const (), Throws::Discard), // sub_140743e10
    Replacement::draft_throws(0x140743f60, "unknown_140743f60", fn_140743f60 as *const (), Throws::Discard), // sub_140743f60
    Replacement::draft(0x140744100, "unknown_140744100", fn_140744100 as *const ()), // sub_140744100
    Replacement::draft(0x140744200, "unknown_140744200", tpl_7545::<0xa0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x80, 0x78, 0x90, 0x14324b910> as *const ()), // sub_140744200
    Replacement::draft(0x140744300, "unknown_140744300", tpl_6796::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x70, 0x14324b910, 0xffffffffffffff90, 0xffffffffffffffb0, 0xffffffffffffffd0, 0xfffffffffffffff0> as *const ()), // sub_140744300
    Replacement::draft(0x140744420, "unknown_140744420", tpl_6797::<0x14387cd60, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x20, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140744420
    Replacement::draft(0x1407444c0, "unknown_1407444c0", fn_1407444c0 as *const ()), // sub_1407444c0
    Replacement::draft_throws(0x140744600, "unknown_140744600", fn_140744600 as *const (), Throws::Discard), // sub_140744600
    Replacement::draft(0x140744720, "unknown_140744720", tpl_7561::<0x14387cd60, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x38, 0x14, 0x18, 0x1c, 0x80, 0x20, 0x1406fca00> as *const ()), // sub_140744720
    Replacement::draft_throws(0x1407447e0, "unknown_1407447e0", fn_1407447e0 as *const (), Throws::Discard), // sub_1407447e0
    Replacement::draft(0x140744930, "unknown_140744930", fn_140744930 as *const ()), // sub_140744930
    Replacement::draft_throws(0x140744a20, "unknown_140744a20", fn_140744a20 as *const (), Throws::Caught), // sub_140744a20
    Replacement::draft_throws(0x140744bd0, "unknown_140744bd0", fn_140744bd0 as *const (), Throws::Caught), // sub_140744bd0
    Replacement::draft_throws(0x140744d80, "unknown_140744d80", fn_140744d80 as *const (), Throws::Caught), // sub_140744d80
    Replacement::draft(0x140744f30, "unknown_140744f30", fn_140744f30 as *const ()), // sub_140744f30
    Replacement::draft(0x140745000, "unknown_140745000", tpl_7566::<0x14387cd48, 0x14387cd60, 0x1401e9970, 0x140759f40, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x1c, 0x120, 0x20, 0x1406fce10, 0x1401e9860> as *const ()), // sub_140745000
    Replacement::draft(0x1407450c0, "unknown_1407450c0", tpl_7561::<0x14387cd60, 0x14243b0b0, 0x14075a030, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x1c, 0x120, 0x20, 0x1406fcfb0> as *const ()), // sub_1407450c0
    Replacement::draft_throws(0x140745190, "unknown_140745190", fn_140745190 as *const (), Throws::Discard), // sub_140745190
    Replacement::draft_throws(0x140745270, "unknown_140745270", fn_140745270 as *const (), Throws::Discard), // sub_140745270
    Replacement::draft(0x140745400, "unknown_140745400", fn_140745400 as *const ()), // sub_140745400
    Replacement::draft(0x1407454d0, "unknown_1407454d0", fn_1407454d0 as *const ()), // sub_1407454d0
    Replacement::draft(0x1407455c0, "unknown_1407455c0", tpl_7571::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x68, 0x14324e6d0, 0xffffffffffffff98, 0xffffffffffffffb8, 0xffffffffffffffd8, 0xfffffffffffffff8> as *const ()), // sub_1407455c0
    Replacement::draft(0x1407456d0, "unknown_1407456d0", tpl_7572::<0x14387ccc8, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x18, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1407456d0
    Replacement::draft(0x140745770, "unknown_140745770", fn_140745770 as *const ()), // sub_140745770
    Replacement::draft_throws(0x140745820, "unknown_140745820", fn_140745820 as *const (), Throws::Discard), // sub_140745820
    Replacement::draft(0x1407458c0, "unknown_1407458c0", tpl_7575::<0x14387ccc8, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x80, 0x18, 0x1406fca00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407458c0
    Replacement::draft_throws(0x140745950, "unknown_140745950", fn_140745950 as *const (), Throws::Discard), // sub_140745950
    Replacement::draft(0x1407459f0, "unknown_1407459f0", fn_1407459f0 as *const ()), // sub_1407459f0
    Replacement::draft_throws(0x140745ab0, "unknown_140745ab0", fn_140745ab0 as *const (), Throws::Caught), // sub_140745ab0
    Replacement::draft_throws(0x140745ba0, "unknown_140745ba0", fn_140745ba0 as *const (), Throws::Caught), // sub_140745ba0
    Replacement::draft_throws(0x140745c90, "unknown_140745c90", fn_140745c90 as *const (), Throws::Caught), // sub_140745c90
    Replacement::draft(0x140745d80, "unknown_140745d80", fn_140745d80 as *const ()), // sub_140745d80
    Replacement::draft(0x140745e10, "unknown_140745e10", fn_140745e10 as *const ()), // sub_140745e10
    Replacement::draft(0x140745e90, "unknown_140745e90", tpl_7575::<0x14387ccc8, 0x14243b0b0, 0x14075a030, 0x338, 0x10, 0x120, 0x18, 0x1406fcfb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140745e90
    Replacement::draft_throws(0x140745f20, "unknown_140745f20", fn_140745f20 as *const (), Throws::Discard), // sub_140745f20
    Replacement::draft_throws(0x140745fa0, "unknown_140745fa0", fn_140745fa0 as *const (), Throws::Discard), // sub_140745fa0
    Replacement::draft(0x140746080, "unknown_140746080", tpl_7583::<0xa8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14324e428, 0x70, 0x1b, 0x1c, 0x20> as *const ()), // sub_140746080
    Replacement::draft(0x140746160, "unknown_140746160", tpl_66517::<0x142535360, 0x1426456c8, 0x1426453a0, 0x14324e428, 0x14183acc2> as *const ()), // sub_140746160
    Replacement::draft(0x140746300, "unknown_140746300", tpl_7584::<0x14387cca8, 0x14243b0b0, 0x140745770, 0x1426d4580> as *const ()), // sub_140746300
    Replacement::draft_throws(0x140746440, "unknown_140746440", fn_140746440 as *const (), Throws::Discard), // sub_140746440
    Replacement::draft(0x140746560, "unknown_140746560", tpl_7586::<0x14387cca8, 0x14243b0b0, 0x1407458c0, 0x338, 0x90, 0x38, 0x94, 0x98, 0x9c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140746560
    Replacement::draft_throws(0x140746620, "unknown_140746620", fn_140746620 as *const (), Throws::Discard), // sub_140746620
    Replacement::draft(0x140746770, "unknown_140746770", fn_140746770 as *const ()), // sub_140746770
    Replacement::draft_throws(0x140746810, "unknown_140746810", fn_140746810 as *const (), Throws::Caught), // sub_140746810
    Replacement::draft_throws(0x1407469d0, "unknown_1407469d0", fn_1407469d0 as *const (), Throws::Caught), // sub_1407469d0
    Replacement::draft_throws(0x140746b90, "unknown_140746b90", fn_140746b90 as *const (), Throws::Caught), // sub_140746b90
    Replacement::draft(0x140746d50, "unknown_140746d50", fn_140746d50 as *const ()), // sub_140746d50
    Replacement::draft(0x140746e10, "unknown_140746e10", tpl_7591::<0x14387cce0, 0x14387cca8, 0x1401e9970, 0x140745e10, 0x338, 0x90, 0xd8, 0x94, 0x98, 0x9c, 0x1401e9860> as *const ()), // sub_140746e10
    Replacement::draft(0x140746ed0, "unknown_140746ed0", tpl_7586::<0x14387cca8, 0x14243b0b0, 0x140745e90, 0x338, 0x90, 0xd8, 0x94, 0x98, 0x9c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140746ed0
    Replacement::draft_throws(0x140746fa0, "unknown_140746fa0", tpl_7592::<0x14387cce0, 0x14387cca8, 0x1401e9770, 0x140745f20, 0x338, 0x90, 0x120, 0x4, 0x94, 0x98, 0x9c, 0x1401e9590> as *const (), Throws::Discard), // sub_140746fa0
    Replacement::draft_throws(0x140747080, "unknown_140747080", fn_140747080 as *const (), Throws::Discard), // sub_140747080
    Replacement::draft(0x140747220, "unknown_140747220", fn_140747220 as *const ()), // sub_140747220
    Replacement::draft(0x140747330, "unknown_140747330", tpl_7583::<0xa8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14324d168, 0x70, 0x1b, 0x1c, 0x20> as *const ()), // sub_140747330
    Replacement::draft(0x140747410, "unknown_140747410", tpl_66517::<0x142535360, 0x1426456c8, 0x1426453a0, 0x14324d168, 0x14183acc2> as *const ()), // sub_140747410
    Replacement::draft(0x1407475b0, "unknown_1407475b0", tpl_7584::<0x14387cc90, 0x14243b0b0, 0x140745770, 0x1426d4580> as *const ()), // sub_1407475b0
    Replacement::draft_throws(0x1407476f0, "unknown_1407476f0", fn_1407476f0 as *const (), Throws::Discard), // sub_1407476f0
    Replacement::draft(0x140747810, "unknown_140747810", tpl_7586::<0x14387cc90, 0x14243b0b0, 0x1407458c0, 0x338, 0x90, 0x38, 0x94, 0x98, 0x9c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140747810
    Replacement::draft_throws(0x1407478d0, "unknown_1407478d0", fn_1407478d0 as *const (), Throws::Discard), // sub_1407478d0
    Replacement::draft_throws(0x140747a20, "unknown_140747a20", fn_140747a20 as *const (), Throws::Caught), // sub_140747a20
    Replacement::draft_throws(0x140747bc0, "unknown_140747bc0", fn_140747bc0 as *const (), Throws::Caught), // sub_140747bc0
    Replacement::draft_throws(0x140747d60, "unknown_140747d60", fn_140747d60 as *const (), Throws::Caught), // sub_140747d60
    Replacement::draft(0x140747f00, "unknown_140747f00", fn_140747f00 as *const ()), // sub_140747f00
    Replacement::draft(0x140747fc0, "unknown_140747fc0", tpl_7591::<0x14387cc6c, 0x14387cc90, 0x1401e9970, 0x140745e10, 0x338, 0x90, 0xd8, 0x94, 0x98, 0x9c, 0x1401e9860> as *const ()), // sub_140747fc0
    Replacement::draft(0x140748080, "unknown_140748080", tpl_7586::<0x14387cc90, 0x14243b0b0, 0x140745e90, 0x338, 0x90, 0xd8, 0x94, 0x98, 0x9c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140748080
    Replacement::draft_throws(0x140748150, "unknown_140748150", tpl_7592::<0x14387cc6c, 0x14387cc90, 0x1401e9770, 0x140745f20, 0x338, 0x90, 0x120, 0x4, 0x94, 0x98, 0x9c, 0x1401e9590> as *const (), Throws::Discard), // sub_140748150
    Replacement::draft_throws(0x140748230, "unknown_140748230", fn_140748230 as *const (), Throws::Discard), // sub_140748230
    Replacement::draft(0x1407483c0, "unknown_1407483c0", fn_1407483c0 as *const ()), // sub_1407483c0
    Replacement::draft(0x1407484c0, "unknown_1407484c0", fn_1407484c0 as *const ()), // sub_1407484c0
    Replacement::draft(0x140748560, "unknown_140748560", fn_140748560 as *const ()), // sub_140748560
    Replacement::draft(0x140748690, "unknown_140748690", fn_140748690 as *const ()), // sub_140748690
    Replacement::draft_throws(0x140748850, "unknown_140748850", fn_140748850 as *const (), Throws::Discard), // sub_140748850
    Replacement::draft(0x1407489d0, "unknown_1407489d0", fn_1407489d0 as *const ()), // sub_1407489d0
    Replacement::draft_throws(0x140748ac0, "unknown_140748ac0", fn_140748ac0 as *const (), Throws::Discard), // sub_140748ac0
    Replacement::draft(0x140748c80, "unknown_140748c80", fn_140748c80 as *const ()), // sub_140748c80
    Replacement::draft_throws(0x140748d60, "unknown_140748d60", fn_140748d60 as *const (), Throws::Caught), // sub_140748d60
    Replacement::draft_throws(0x140749200, "unknown_140749200", fn_140749200 as *const (), Throws::Caught), // sub_140749200
    Replacement::draft_throws(0x140749600, "unknown_140749600", fn_140749600 as *const (), Throws::Caught), // sub_140749600
    Replacement::draft(0x140749a10, "unknown_140749a10", fn_140749a10 as *const ()), // sub_140749a10
    Replacement::draft(0x140749af0, "unknown_140749af0", fn_140749af0 as *const ()), // sub_140749af0
    Replacement::draft(0x140749bf0, "unknown_140749bf0", fn_140749bf0 as *const ()), // sub_140749bf0
    Replacement::draft_throws(0x140749d00, "unknown_140749d00", fn_140749d00 as *const (), Throws::Discard), // sub_140749d00
    Replacement::draft_throws(0x140749e30, "unknown_140749e30", fn_140749e30 as *const (), Throws::Discard), // sub_140749e30
    Replacement::draft(0x14074a030, "unknown_14074a030", tpl_66544::<0x14075a380, 0x1426cdcb0, 0x14384e248, 0x143250440, 0x1, 0x1401de410, 0x14384e2a0, 0x1432503e8> as *const ()), // sub_14074a030
    Replacement::draft(0x14074a0c0, "unknown_14074a0c0", fn_14074a0c0 as *const ()), // sub_14074a0c0
    Replacement::draft(0x14074a210, "unknown_14074a210", tpl_66544::<0x14075a4d0, 0x1426cdcb0, 0x14384e248, 0x143250440, 0x1, 0x1401de410, 0x14384e2a0, 0x1432503e8> as *const ()), // sub_14074a210
    Replacement::draft(0x14074a2a0, "unknown_14074a2a0", tpl_7618::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14324f5c8, 0x20> as *const ()), // sub_14074a2a0
    Replacement::draft(0x14074a330, "unknown_14074a330", tpl_7619::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14324f5c8, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0xffffffffffffffec> as *const ()), // sub_14074a330
    Replacement::draft_throws(0x14074a530, "unknown_14074a530", fn_14074a530 as *const (), Throws::Caught), // sub_14074a530
    Replacement::draft_throws(0x14074a5d0, "unknown_14074a5d0", fn_14074a5d0 as *const (), Throws::Caught), // sub_14074a5d0
    Replacement::draft_throws(0x14074a670, "unknown_14074a670", fn_14074a670 as *const (), Throws::Caught), // sub_14074a670
    Replacement::draft_throws(0x14074a7f0, "unknown_14074a7f0", tpl_7326::<0x14387cd50, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140753a70> as *const (), Throws::Discard), // sub_14074a7f0
    Replacement::draft(0x14074a890, "unknown_14074a890", fn_14074a890 as *const ()), // sub_14074a890
    Replacement::draft(0x14074a9b0, "unknown_14074a9b0", tpl_66547::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324bee0, 0xffffffffffffff70, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_14074a9b0
    Replacement::draft(0x14074aaf0, "unknown_14074aaf0", tpl_7572::<0x14387cd20, 0x14243b0b0, 0x14074a3e0, 0x14387d230, 0x40, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14074aaf0
    Replacement::draft(0x14074ab90, "unknown_14074ab90", fn_14074ab90 as *const ()), // sub_14074ab90
    Replacement::draft_throws(0x14074acf0, "unknown_14074acf0", fn_14074acf0 as *const (), Throws::Caught), // sub_14074acf0
    Replacement::draft_throws(0x14074af30, "unknown_14074af30", fn_14074af30 as *const (), Throws::Discard), // sub_14074af30
    Replacement::draft_throws(0x14074b070, "unknown_14074b070", fn_14074b070 as *const (), Throws::Discard), // sub_14074b070
    Replacement::draft_throws(0x14074b1c0, "unknown_14074b1c0", fn_14074b1c0 as *const (), Throws::Discard), // sub_14074b1c0
    Replacement::draft_throws(0x14074b270, "unknown_14074b270", fn_14074b270 as *const (), Throws::Caught), // sub_14074b270
    Replacement::draft_throws(0x14074b6c0, "unknown_14074b6c0", fn_14074b6c0 as *const (), Throws::Caught), // sub_14074b6c0
    Replacement::draft_throws(0x14074baa0, "unknown_14074baa0", fn_14074baa0 as *const (), Throws::Caught), // sub_14074baa0
    Replacement::draft(0x14074be70, "unknown_14074be70", fn_14074be70 as *const ()), // sub_14074be70
    Replacement::draft(0x14074c0e0, "unknown_14074c0e0", fn_14074c0e0 as *const ()), // sub_14074c0e0
    Replacement::draft(0x14074c220, "unknown_14074c220", fn_14074c220 as *const ()), // sub_14074c220
    Replacement::draft_throws(0x14074c330, "unknown_14074c330", fn_14074c330 as *const (), Throws::Discard), // sub_14074c330
    Replacement::draft_throws(0x14074c470, "unknown_14074c470", fn_14074c470 as *const (), Throws::Discard), // sub_14074c470
    Replacement::draft(0x14074c620, "unknown_14074c620", fn_14074c620 as *const ()), // sub_14074c620
    Replacement::draft(0x14074c6a0, "unknown_14074c6a0", tpl_7635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324c568, 0x18, 0x20> as *const ()), // sub_14074c6a0
    Replacement::draft(0x14074c7b0, "unknown_14074c7b0", fn_14074c7b0 as *const ()), // sub_14074c7b0
    Replacement::draft_throws(0x14074c8b0, "unknown_14074c8b0", fn_14074c8b0 as *const (), Throws::Caught), // sub_14074c8b0
    Replacement::draft(0x14074ca20, "unknown_14074ca20", fn_14074ca20 as *const ()), // sub_14074ca20
    Replacement::draft_throws(0x14074cb00, "unknown_14074cb00", fn_14074cb00 as *const (), Throws::Discard), // sub_14074cb00
    Replacement::draft(0x14074cbf0, "unknown_14074cbf0", fn_14074cbf0 as *const ()), // sub_14074cbf0
    Replacement::draft_throws(0x14074cc90, "unknown_14074cc90", fn_14074cc90 as *const (), Throws::Caught), // sub_14074cc90
    Replacement::draft_throws(0x14074cf50, "unknown_14074cf50", fn_14074cf50 as *const (), Throws::Caught), // sub_14074cf50
    Replacement::draft_throws(0x14074d1e0, "unknown_14074d1e0", fn_14074d1e0 as *const (), Throws::Caught), // sub_14074d1e0
    Replacement::draft(0x14074d450, "unknown_14074d450", fn_14074d450 as *const ()), // sub_14074d450
    Replacement::draft(0x14074d5c0, "unknown_14074d5c0", fn_14074d5c0 as *const ()), // sub_14074d5c0
    Replacement::draft(0x14074d680, "unknown_14074d680", fn_14074d680 as *const ()), // sub_14074d680
    Replacement::draft_throws(0x14074d750, "unknown_14074d750", fn_14074d750 as *const (), Throws::Discard), // sub_14074d750
    Replacement::draft_throws(0x14074d820, "unknown_14074d820", fn_14074d820 as *const (), Throws::Discard), // sub_14074d820
    Replacement::draft(0x14074d960, "unknown_14074d960", fn_14074d960 as *const ()), // sub_14074d960
    Replacement::draft(0x14074da50, "unknown_14074da50", fn_14074da50 as *const ()), // sub_14074da50
    Replacement::draft(0x14074db80, "unknown_14074db80", tpl_6797::<0x14387ccf0, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x20, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14074db80
    Replacement::draft(0x14074dc20, "unknown_14074dc20", fn_14074dc20 as *const ()), // sub_14074dc20
    Replacement::draft_throws(0x14074ddb0, "unknown_14074ddb0", fn_14074ddb0 as *const (), Throws::Discard), // sub_14074ddb0
    Replacement::draft(0x14074df00, "unknown_14074df00", fn_14074df00 as *const ()), // sub_14074df00
    Replacement::draft_throws(0x14074dfd0, "unknown_14074dfd0", fn_14074dfd0 as *const (), Throws::Discard), // sub_14074dfd0
    Replacement::draft(0x14074e160, "unknown_14074e160", fn_14074e160 as *const ()), // sub_14074e160
    Replacement::draft_throws(0x14074e270, "unknown_14074e270", fn_14074e270 as *const (), Throws::Caught), // sub_14074e270
    Replacement::draft_throws(0x14074e5a0, "unknown_14074e5a0", fn_14074e5a0 as *const (), Throws::Caught), // sub_14074e5a0
    Replacement::draft_throws(0x14074e890, "unknown_14074e890", fn_14074e890 as *const (), Throws::Caught), // sub_14074e890
    Replacement::draft(0x14074eb80, "unknown_14074eb80", fn_14074eb80 as *const ()), // sub_14074eb80
    Replacement::draft(0x14074ec70, "unknown_14074ec70", fn_14074ec70 as *const ()), // sub_14074ec70
    Replacement::draft(0x14074ed50, "unknown_14074ed50", fn_14074ed50 as *const ()), // sub_14074ed50
    Replacement::draft_throws(0x14074ee40, "unknown_14074ee40", fn_14074ee40 as *const (), Throws::Discard), // sub_14074ee40
    Replacement::draft_throws(0x14074ef40, "unknown_14074ef40", fn_14074ef40 as *const (), Throws::Discard), // sub_14074ef40
    Replacement::draft(0x14074f110, "unknown_14074f110", fn_14074f110 as *const ()), // sub_14074f110
    Replacement::draft(0x14074f250, "unknown_14074f250", tpl_7545::<0xa0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x80, 0x78, 0x90, 0x14324e948> as *const ()), // sub_14074f250
    Replacement::draft(0x14074f350, "unknown_14074f350", tpl_6796::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x70, 0x14324e948, 0xffffffffffffff90, 0xffffffffffffffb0, 0xffffffffffffffd0, 0xfffffffffffffff0> as *const ()), // sub_14074f350
    Replacement::draft(0x14074f470, "unknown_14074f470", tpl_6797::<0x14387cdc8, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x20, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14074f470
    Replacement::draft(0x14074f510, "unknown_14074f510", fn_14074f510 as *const ()), // sub_14074f510
    Replacement::draft_throws(0x14074f620, "unknown_14074f620", fn_14074f620 as *const (), Throws::Discard), // sub_14074f620
    Replacement::draft(0x14074f720, "unknown_14074f720", tpl_7666::<0x14387cdc8, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x38, 0x14, 0x18, 0x80, 0x20, 0x1406fca00, 0x8> as *const ()), // sub_14074f720
    Replacement::draft_throws(0x14074f7d0, "unknown_14074f7d0", fn_14074f7d0 as *const (), Throws::Discard), // sub_14074f7d0
    Replacement::draft(0x14074f8e0, "unknown_14074f8e0", fn_14074f8e0 as *const ()), // sub_14074f8e0
    Replacement::draft_throws(0x14074f9c0, "unknown_14074f9c0", fn_14074f9c0 as *const (), Throws::Caught), // sub_14074f9c0
    Replacement::draft_throws(0x14074fb30, "unknown_14074fb30", fn_14074fb30 as *const (), Throws::Caught), // sub_14074fb30
    Replacement::draft_throws(0x14074fca0, "unknown_14074fca0", fn_14074fca0 as *const (), Throws::Caught), // sub_14074fca0
    Replacement::draft(0x14074fe10, "unknown_14074fe10", fn_14074fe10 as *const ()), // sub_14074fe10
    Replacement::draft(0x14074fed0, "unknown_14074fed0", fn_14074fed0 as *const ()), // sub_14074fed0
    Replacement::draft(0x14074ff80, "unknown_14074ff80", tpl_7666::<0x14387cdc8, 0x14243b0b0, 0x14075a030, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x120, 0x20, 0x1406fcfb0, 0x8> as *const ()), // sub_14074ff80
    Replacement::draft_throws(0x140750030, "unknown_140750030", fn_140750030 as *const (), Throws::Discard), // sub_140750030
    Replacement::draft_throws(0x1407500f0, "unknown_1407500f0", fn_1407500f0 as *const (), Throws::Discard), // sub_1407500f0
    Replacement::draft(0x140750250, "unknown_140750250", fn_140750250 as *const ()), // sub_140750250
    Replacement::draft(0x1407502f0, "unknown_1407502f0", tpl_7675::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14324eac8, 0x88, 0x30, 0x33, 0x34> as *const ()), // sub_1407502f0
    Replacement::draft(0x1407503e0, "unknown_1407503e0", tpl_66600::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324eac8, 0xffffffffffffff80, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_1407503e0
    Replacement::draft(0x140750510, "unknown_140750510", fn_140750510 as *const ()), // sub_140750510
    Replacement::draft(0x1407505b0, "unknown_1407505b0", fn_1407505b0 as *const ()), // sub_1407505b0
    Replacement::draft_throws(0x1407507b0, "unknown_1407507b0", fn_1407507b0 as *const (), Throws::Discard), // sub_1407507b0
    Replacement::draft(0x140750950, "unknown_140750950", fn_140750950 as *const ()), // sub_140750950
    Replacement::draft_throws(0x140750a40, "unknown_140750a40", fn_140750a40 as *const (), Throws::Discard), // sub_140750a40
    Replacement::draft(0x140750c20, "unknown_140750c20", fn_140750c20 as *const ()), // sub_140750c20
    Replacement::draft_throws(0x140750d60, "unknown_140750d60", fn_140750d60 as *const (), Throws::Caught), // sub_140750d60
    Replacement::draft_throws(0x140751320, "unknown_140751320", fn_140751320 as *const (), Throws::Caught), // sub_140751320
    Replacement::draft_throws(0x140751820, "unknown_140751820", fn_140751820 as *const (), Throws::Caught), // sub_140751820
    Replacement::draft(0x140751d20, "unknown_140751d20", fn_140751d20 as *const ()), // sub_140751d20
    Replacement::draft(0x140751e20, "unknown_140751e20", fn_140751e20 as *const ()), // sub_140751e20
    Replacement::draft(0x140751f30, "unknown_140751f30", fn_140751f30 as *const ()), // sub_140751f30
    Replacement::draft_throws(0x140752050, "unknown_140752050", fn_140752050 as *const (), Throws::Discard), // sub_140752050
    Replacement::draft_throws(0x140752180, "unknown_140752180", fn_140752180 as *const (), Throws::Discard), // sub_140752180
    Replacement::draft(0x140752390, "unknown_140752390", tpl_7690::<0x14075a380, 0x1426cdcb0, 0x14384e2e0, 0x143250550, 0x1, 0x1401de410, 0x14384e208, 0x143250520, 0x14384e228, 0x1432505e8, 0x3> as *const ()), // sub_140752390
    Replacement::draft(0x140752450, "unknown_140752450", fn_140752450 as *const ()), // sub_140752450
    Replacement::draft(0x1407525e0, "unknown_1407525e0", tpl_7690::<0x14075a4d0, 0x1426cdcb0, 0x14384e2e0, 0x143250550, 0x1, 0x1401de410, 0x14384e208, 0x143250520, 0x14384e228, 0x1432505e8, 0x3> as *const ()), // sub_1407525e0
    Replacement::draft(0x1407526a0, "unknown_1407526a0", fn_1407526a0 as *const ()), // sub_1407526a0
    Replacement::draft(0x140752730, "unknown_140752730", fn_140752730 as *const ()), // sub_140752730
    Replacement::draft(0x1407527e0, "unknown_1407527e0", fn_1407527e0 as *const ()), // sub_1407527e0
    Replacement::draft(0x140752900, "unknown_140752900", fn_140752900 as *const ()), // sub_140752900
    Replacement::draft_throws(0x140752ab0, "unknown_140752ab0", fn_140752ab0 as *const (), Throws::Discard), // sub_140752ab0
    Replacement::draft_throws(0x140752be0, "unknown_140752be0", fn_140752be0 as *const (), Throws::Discard), // sub_140752be0
    Replacement::draft_throws(0x140752d10, "unknown_140752d10", fn_140752d10 as *const (), Throws::Discard), // sub_140752d10
    Replacement::draft_throws(0x140752e70, "unknown_140752e70", fn_140752e70 as *const (), Throws::Discard), // sub_140752e70
    Replacement::draft_throws(0x140752ff0, "unknown_140752ff0", fn_140752ff0 as *const (), Throws::Caught), // sub_140752ff0
    Replacement::draft_throws(0x1407530c0, "unknown_1407530c0", fn_1407530c0 as *const (), Throws::Caught), // sub_1407530c0
    Replacement::draft_throws(0x140753190, "unknown_140753190", fn_140753190 as *const (), Throws::Caught), // sub_140753190
    Replacement::draft(0x140753260, "unknown_140753260", fn_140753260 as *const ()), // sub_140753260
    Replacement::draft(0x1407533e0, "unknown_1407533e0", fn_1407533e0 as *const ()), // sub_1407533e0
    Replacement::draft(0x140753610, "unknown_140753610", fn_140753610 as *const ()), // sub_140753610
    Replacement::draft_throws(0x140753870, "unknown_140753870", fn_140753870 as *const (), Throws::Discard), // sub_140753870
    Replacement::draft_throws(0x140753a70, "unknown_140753a70", fn_140753a70 as *const (), Throws::Discard), // sub_140753a70
    Replacement::draft(0x140753b60, "unknown_140753b60", fn_140753b60 as *const ()), // sub_140753b60
    Replacement::draft(0x140753c00, "unknown_140753c00", fn_140753c00 as *const ()), // sub_140753c00
    Replacement::draft(0x140753ca0, "unknown_140753ca0", fn_140753ca0 as *const ()), // sub_140753ca0
    Replacement::draft(0x140753d30, "unknown_140753d30", fn_140753d30 as *const ()), // sub_140753d30
    Replacement::draft(0x140753de0, "unknown_140753de0", fn_140753de0 as *const ()), // sub_140753de0
    Replacement::draft(0x140753f10, "unknown_140753f10", tpl_7706::<0x14387cda8, 0x14243b0b0, 0x140761e30, 0x10, 0x1407662e0, 0x18, 0x1406fc6d0, 0x90, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140753f10
    Replacement::draft_throws(0x140753fb0, "unknown_140753fb0", tpl_7707::<0x14387cd10, 0x14387cda8, 0x1401e9fa0, 0x140761f40, 0x10, 0x14075edb0, 0x18, 0x1406d96a0, 0x90, 0x108, 0x1401e9e70> as *const (), Throws::Discard), // sub_140753fb0
    Replacement::draft(0x140754060, "unknown_140754060", tpl_7706::<0x14387cda8, 0x14243b0b0, 0x140762060, 0x10, 0x1407667e0, 0x18, 0x1406fca00, 0x90, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140754060
    Replacement::draft_throws(0x140754100, "unknown_140754100", tpl_7707::<0x14387cd10, 0x14387cda8, 0x1401e9b60, 0x140762160, 0x10, 0x14075eee0, 0x18, 0x1406d97d0, 0x90, 0x108, 0x1401e9a20> as *const (), Throws::Discard), // sub_140754100
    Replacement::draft(0x1407541b0, "unknown_1407541b0", fn_1407541b0 as *const ()), // sub_1407541b0
    Replacement::draft_throws(0x140754310, "unknown_140754310", fn_140754310 as *const (), Throws::Caught), // sub_140754310
    Replacement::draft_throws(0x140754440, "unknown_140754440", fn_140754440 as *const (), Throws::Caught), // sub_140754440
    Replacement::draft_throws(0x140754570, "unknown_140754570", fn_140754570 as *const (), Throws::Caught), // sub_140754570
    Replacement::draft(0x1407546a0, "unknown_1407546a0", fn_1407546a0 as *const ()), // sub_1407546a0
    Replacement::draft(0x140754770, "unknown_140754770", fn_140754770 as *const ()), // sub_140754770
    Replacement::draft(0x140754800, "unknown_140754800", tpl_7706::<0x14387cda8, 0x14243b0b0, 0x140763ac0, 0x10, 0x140766c40, 0x18, 0x1406fcfb0, 0x90, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140754800
    Replacement::draft_throws(0x1407548a0, "unknown_1407548a0", fn_1407548a0 as *const (), Throws::Discard), // sub_1407548a0
    Replacement::draft_throws(0x140754930, "unknown_140754930", fn_140754930 as *const (), Throws::Discard), // sub_140754930
    Replacement::draft(0x140754a10, "unknown_140754a10", fn_140754a10 as *const ()), // sub_140754a10
    Replacement::draft(0x140754af0, "unknown_140754af0", fn_140754af0 as *const ()), // sub_140754af0
    Replacement::draft(0x140754c20, "unknown_140754c20", tpl_7452::<0x14387cd98, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140754c20
    Replacement::draft(0x140754cc0, "unknown_140754cc0", fn_140754cc0 as *const ()), // sub_140754cc0
    Replacement::draft_throws(0x140754e30, "unknown_140754e30", fn_140754e30 as *const (), Throws::Discard), // sub_140754e30
    Replacement::draft(0x140754f90, "unknown_140754f90", fn_140754f90 as *const ()), // sub_140754f90
    Replacement::draft_throws(0x140755070, "unknown_140755070", fn_140755070 as *const (), Throws::Discard), // sub_140755070
    Replacement::draft(0x140755210, "unknown_140755210", fn_140755210 as *const ()), // sub_140755210
    Replacement::draft_throws(0x140755340, "unknown_140755340", fn_140755340 as *const (), Throws::Caught), // sub_140755340
    Replacement::draft_throws(0x140755530, "unknown_140755530", fn_140755530 as *const (), Throws::Caught), // sub_140755530
    Replacement::draft_throws(0x140755720, "unknown_140755720", fn_140755720 as *const (), Throws::Caught), // sub_140755720
    Replacement::draft(0x140755910, "unknown_140755910", fn_140755910 as *const ()), // sub_140755910
    Replacement::draft(0x140755a00, "unknown_140755a00", fn_140755a00 as *const ()), // sub_140755a00
    Replacement::draft(0x140755ae0, "unknown_140755ae0", fn_140755ae0 as *const ()), // sub_140755ae0
    Replacement::draft_throws(0x140755bd0, "unknown_140755bd0", fn_140755bd0 as *const (), Throws::Discard), // sub_140755bd0
    Replacement::draft_throws(0x140755ce0, "unknown_140755ce0", fn_140755ce0 as *const (), Throws::Discard), // sub_140755ce0
    Replacement::draft(0x140755ec0, "unknown_140755ec0", fn_140755ec0 as *const ()), // sub_140755ec0
    Replacement::draft(0x140755fc0, "unknown_140755fc0", tpl_7545::<0xa0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x80, 0x78, 0x90, 0x14324b610> as *const ()), // sub_140755fc0
    Replacement::draft(0x1407560c0, "unknown_1407560c0", tpl_6796::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x70, 0x14324b610, 0xffffffffffffff90, 0xffffffffffffffb0, 0xffffffffffffffd0, 0xfffffffffffffff0> as *const ()), // sub_1407560c0
    Replacement::draft(0x1407561e0, "unknown_1407561e0", tpl_6797::<0x14387cd70, 0x14243b0b0, 0x140758e00, 0x14387d230, 0x20, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1407561e0
    Replacement::draft(0x140756280, "unknown_140756280", fn_140756280 as *const ()), // sub_140756280
    Replacement::draft_throws(0x1407563a0, "unknown_1407563a0", fn_1407563a0 as *const (), Throws::Discard), // sub_1407563a0
    Replacement::draft(0x1407564c0, "unknown_1407564c0", tpl_7729::<0x14387cd70, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x38, 0x14, 0x18, 0x80, 0x19, 0x20, 0x1406fca00> as *const ()), // sub_1407564c0
    Replacement::draft_throws(0x140756580, "unknown_140756580", fn_140756580 as *const (), Throws::Discard), // sub_140756580
    Replacement::draft(0x1407566c0, "unknown_1407566c0", fn_1407566c0 as *const ()), // sub_1407566c0
    Replacement::draft_throws(0x1407567b0, "unknown_1407567b0", fn_1407567b0 as *const (), Throws::Caught), // sub_1407567b0
    Replacement::draft_throws(0x140756950, "unknown_140756950", fn_140756950 as *const (), Throws::Caught), // sub_140756950
    Replacement::draft_throws(0x140756af0, "unknown_140756af0", fn_140756af0 as *const (), Throws::Caught), // sub_140756af0
    Replacement::draft(0x140756c90, "unknown_140756c90", fn_140756c90 as *const ()), // sub_140756c90
    Replacement::draft(0x140756d60, "unknown_140756d60", fn_140756d60 as *const ()), // sub_140756d60
    Replacement::draft(0x140756e20, "unknown_140756e20", tpl_7729::<0x14387cd70, 0x14243b0b0, 0x14075a030, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x120, 0x19, 0x20, 0x1406fcfb0> as *const ()), // sub_140756e20
    Replacement::draft_throws(0x140756ef0, "unknown_140756ef0", fn_140756ef0 as *const (), Throws::Discard), // sub_140756ef0
    Replacement::draft_throws(0x140756fd0, "unknown_140756fd0", fn_140756fd0 as *const (), Throws::Discard), // sub_140756fd0
    Replacement::draft(0x140757160, "unknown_140757160", fn_140757160 as *const ()), // sub_140757160
    Replacement::draft(0x140757200, "unknown_140757200", tpl_7618::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14324bb90, 0x20> as *const ()), // sub_140757200
    Replacement::draft(0x140757290, "unknown_140757290", tpl_7619::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14324bb90, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0xffffffffffffffec> as *const ()), // sub_140757290
    Replacement::draft_throws(0x140757490, "unknown_140757490", fn_140757490 as *const (), Throws::Caught), // sub_140757490
    Replacement::draft_throws(0x140757530, "unknown_140757530", fn_140757530 as *const (), Throws::Caught), // sub_140757530
    Replacement::draft_throws(0x1407575d0, "unknown_1407575d0", fn_1407575d0 as *const (), Throws::Caught), // sub_1407575d0
    Replacement::draft_throws(0x140757750, "unknown_140757750", tpl_7326::<0x14387cdd0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140753a70> as *const (), Throws::Discard), // sub_140757750
    Replacement::draft(0x1407577f0, "unknown_1407577f0", tpl_7618::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14324e7b8, 0x20> as *const ()), // sub_1407577f0
    Replacement::draft(0x140757880, "unknown_140757880", tpl_7619::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14324e7b8, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0xffffffffffffffec> as *const ()), // sub_140757880
    Replacement::draft_throws(0x140757a80, "unknown_140757a80", fn_140757a80 as *const (), Throws::Caught), // sub_140757a80
    Replacement::draft_throws(0x140757b20, "unknown_140757b20", fn_140757b20 as *const (), Throws::Caught), // sub_140757b20
    Replacement::draft_throws(0x140757bc0, "unknown_140757bc0", fn_140757bc0 as *const (), Throws::Caught), // sub_140757bc0
    Replacement::draft_throws(0x140757d40, "unknown_140757d40", tpl_7326::<0x14387cd68, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140753a70> as *const (), Throws::Discard), // sub_140757d40
    Replacement::draft(0x140757de0, "unknown_140757de0", fn_140757de0 as *const ()), // sub_140757de0
    Replacement::draft(0x140757ef0, "unknown_140757ef0", tpl_66600::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324d790, 0xffffffffffffff80, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_140757ef0
    Replacement::draft(0x140758020, "unknown_140758020", tpl_7739::<0x14387cd90, 0x14243b0b0, 0x14074a3e0, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x14387d230, 0x30, 0xfffffffffffffff8> as *const ()), // sub_140758020
    Replacement::draft(0x140758100, "unknown_140758100", fn_140758100 as *const ()), // sub_140758100
    Replacement::draft_throws(0x140758200, "unknown_140758200", fn_140758200 as *const (), Throws::Discard), // sub_140758200
    Replacement::draft_throws(0x140758300, "unknown_140758300", fn_140758300 as *const (), Throws::Discard), // sub_140758300
    Replacement::draft_throws(0x1407583c0, "unknown_1407583c0", fn_1407583c0 as *const (), Throws::Discard), // sub_1407583c0
    Replacement::draft_throws(0x1407584d0, "unknown_1407584d0", fn_1407584d0 as *const (), Throws::Discard), // sub_1407584d0
    Replacement::draft_throws(0x140758560, "unknown_140758560", fn_140758560 as *const (), Throws::Caught), // sub_140758560
    Replacement::draft_throws(0x140758660, "unknown_140758660", fn_140758660 as *const (), Throws::Caught), // sub_140758660
    Replacement::draft_throws(0x140758760, "unknown_140758760", fn_140758760 as *const (), Throws::Caught), // sub_140758760
    Replacement::draft(0x140758860, "unknown_140758860", fn_140758860 as *const ()), // sub_140758860
    Replacement::draft(0x140758930, "unknown_140758930", fn_140758930 as *const ()), // sub_140758930
    Replacement::draft(0x140758a20, "unknown_140758a20", fn_140758a20 as *const ()), // sub_140758a20
    Replacement::draft_throws(0x140758ae0, "unknown_140758ae0", fn_140758ae0 as *const (), Throws::Discard), // sub_140758ae0
    Replacement::draft_throws(0x140758bd0, "unknown_140758bd0", fn_140758bd0 as *const (), Throws::Discard), // sub_140758bd0
    Replacement::draft(0x140758cf0, "unknown_140758cf0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14324b850, 0x8> as *const ()), // sub_140758cf0
    Replacement::draft(0x140758d70, "unknown_140758d70", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324b850> as *const ()), // sub_140758d70
    Replacement::draft(0x140758e00, "unknown_140758e00", fn_140758e00 as *const ()), // sub_140758e00
    Replacement::draft(0x140758e80, "unknown_140758e80", fn_140758e80 as *const ()), // sub_140758e80
    Replacement::draft_throws(0x140758fc0, "unknown_140758fc0", fn_140758fc0 as *const (), Throws::Caught), // sub_140758fc0
    Replacement::draft(0x140759110, "unknown_140759110", fn_140759110 as *const ()), // sub_140759110
    Replacement::draft_throws(0x140759220, "unknown_140759220", fn_140759220 as *const (), Throws::Discard), // sub_140759220
    Replacement::draft(0x140759330, "unknown_140759330", fn_140759330 as *const ()), // sub_140759330
    Replacement::draft_throws(0x1407593b0, "unknown_1407593b0", fn_1407593b0 as *const (), Throws::Caught), // sub_1407593b0
    Replacement::draft_throws(0x140759750, "unknown_140759750", fn_140759750 as *const (), Throws::Caught), // sub_140759750
    Replacement::draft_throws(0x140759a80, "unknown_140759a80", fn_140759a80 as *const (), Throws::Caught), // sub_140759a80
    Replacement::draft(0x140759da0, "unknown_140759da0", fn_140759da0 as *const ()), // sub_140759da0
    Replacement::draft(0x140759f40, "unknown_140759f40", fn_140759f40 as *const ()), // sub_140759f40
    Replacement::draft(0x14075a030, "unknown_14075a030", fn_14075a030 as *const ()), // sub_14075a030
    Replacement::draft_throws(0x14075a130, "unknown_14075a130", fn_14075a130 as *const (), Throws::Discard), // sub_14075a130
    Replacement::draft_throws(0x14075a230, "unknown_14075a230", fn_14075a230 as *const (), Throws::Discard), // sub_14075a230
    Replacement::draft(0x14075a380, "unknown_14075a380", fn_14075a380 as *const ()), // sub_14075a380
    Replacement::draft(0x14075a400, "unknown_14075a400", fn_14075a400 as *const ()), // sub_14075a400
    Replacement::draft(0x14075a4d0, "unknown_14075a4d0", fn_14075a4d0 as *const ()), // sub_14075a4d0
    Replacement::draft(0x14075a550, "unknown_14075a550", fn_14075a550 as *const ()), // sub_14075a550
    Replacement::draft(0x14075a6e0, "unknown_14075a6e0", fn_14075a6e0 as *const ()), // sub_14075a6e0
    Replacement::draft(0x14075a7f0, "unknown_14075a7f0", fn_14075a7f0 as *const ()), // sub_14075a7f0
    Replacement::draft(0x14075a8d0, "unknown_14075a8d0", fn_14075a8d0 as *const ()), // sub_14075a8d0
    Replacement::draft(0x14075aa20, "content_14075aa20", fn_14075aa20 as *const ()), // content::EQSNode::content::EQSNode::vfunc_0
    Replacement::draft(0x14075aa70, "content_14075aa70", fn_14075aa70 as *const ()), // sub_14075aa70
    Replacement::draft(0x14075ab90, "content_14075ab90", fn_14075ab90 as *const ()), // content::EQSAngleCompareTask::content::EQSAngleCompareTask::vfunc_0
    Replacement::draft(0x14075ac30, "content_14075ac30", fn_14075ac30 as *const ()), // content::EQSCoverPositionGenerator::content::EQSCoverPositionGenerator::vfunc_0
    Replacement::draft(0x14075acc0, "content_14075acc0", fn_14075acc0 as *const ()), // content::EQSCoverScoringTask::content::EQSCoverScoringTask::vfunc_0
    Replacement::draft(0x14075ad10, "content_14075ad10", fn_14075ad10 as *const ()), // content::EQSDiscGeneratorNode::content::EQSDiscGeneratorNode::vfunc_0
    Replacement::draft(0x14075ad80, "content_14075ad80", fn_14075ad80 as *const ()), // content::EQSDistanceFieldGenerationTask::content::EQSDistanceFieldGenerationTask::vfunc_0
    Replacement::draft(0x14075add0, "content_14075add0", fn_14075add0 as *const ()), // content::EQSDotCompareTask::content::EQSDotCompareTask::vfunc_0
    Replacement::draft(0x14075ae40, "content_14075ae40", fn_14075ae40 as *const ()), // content::EQSDynamicGridGeneratorNode::content::EQSDynamicGridGeneratorNode::vfunc_0
    Replacement::draft(0x14075aee0, "content_14075aee0", fn_14075aee0 as *const ()), // content::EQSElevationTask::content::EQSElevationTask::vfunc_0
    Replacement::draft(0x14075af30, "content_14075af30", fn_14075af30 as *const ()), // content::SmartObjectService::content::SmartObjectService::vfunc_0
    Replacement::draft(0x14075af80, "content_14075af80", fn_14075af80 as *const ()), // content::EQSEuclideanLinearDistanceTask::content::EQSEuclideanLinearDistanceTask::vfunc_0
    Replacement::draft(0x14075afd0, "content_14075afd0", fn_14075afd0 as *const ()), // content::EQSGridPropagationGeneratorNode::content::EQSGridPropagationGeneratorNode::vfunc_0
    Replacement::draft(0x14075b040, "content_14075b040", fn_14075b040 as *const ()), // content::EQSHeightTask::content::EQSHeightTask::vfunc_0
    Replacement::draft(0x14075b090, "content_14075b090", fn_14075b090 as *const ()), // content::HazardService::content::HazardService::vfunc_0
    Replacement::draft(0x14075b0e0, "content_14075b0e0", fn_14075b0e0 as *const ()), // content::EQSQueryService::content::EQSQueryService::vfunc_0
    Replacement::draft(0x14075b170, "content_14075b170", fn_14075b170 as *const ()), // content::EQSRandomEuclideanDistanceTask::content::EQSRandomEuclideanDistanceTask::vfunc_0
    Replacement::draft(0x14075b1c0, "content_14075b1c0", fn_14075b1c0 as *const ()), // content::EQSSphereGeneratorNode::content::EQSSphereGeneratorNode::vfunc_0
    Replacement::draft_throws(0x14075b230, "content_14075b230", fn_14075b230 as *const (), Throws::Caught), // sub_14075b230
    Replacement::draft_throws(0x14075b530, "content_14075b530", fn_14075b530 as *const (), Throws::Caught), // sub_14075b530
    Replacement::draft_throws(0x14075b830, "content_14075b830", fn_14075b830 as *const (), Throws::Caught), // sub_14075b830
    Replacement::draft(0x14075bb30, "content_14075bb30", fn_14075bb30 as *const ()), // sub_14075bb30
    Replacement::draft_throws(0x14075bc50, "content_14075bc50", fn_14075bc50 as *const (), Throws::Caught), // sub_14075bc50
    Replacement::draft_throws(0x14075c070, "content_14075c070", fn_14075c070 as *const (), Throws::Caught), // sub_14075c070
    Replacement::draft_throws(0x14075c490, "content_14075c490", fn_14075c490 as *const (), Throws::Caught), // sub_14075c490
    Replacement::draft_throws(0x14075c8b0, "content_14075c8b0", fn_14075c8b0 as *const (), Throws::Caught), // sub_14075c8b0
    Replacement::draft_throws(0x14075cbb0, "content_14075cbb0", fn_14075cbb0 as *const (), Throws::Caught), // sub_14075cbb0
    Replacement::draft_throws(0x14075ceb0, "content_14075ceb0", fn_14075ceb0 as *const (), Throws::Caught), // sub_14075ceb0
    Replacement::draft_throws(0x14075d1b0, "content_14075d1b0", fn_14075d1b0 as *const (), Throws::Caught), // sub_14075d1b0
    Replacement::draft_throws(0x14075d4b0, "content_14075d4b0", fn_14075d4b0 as *const (), Throws::Caught), // sub_14075d4b0
    Replacement::draft_throws(0x14075d7b0, "content_14075d7b0", fn_14075d7b0 as *const (), Throws::Caught), // sub_14075d7b0
    Replacement::draft_throws(0x14075dab0, "content_14075dab0", fn_14075dab0 as *const (), Throws::Caught), // sub_14075dab0
    Replacement::draft_throws(0x14075ddb0, "content_14075ddb0", fn_14075ddb0 as *const (), Throws::Caught), // sub_14075ddb0
    Replacement::draft_throws(0x14075e0b0, "content_14075e0b0", fn_14075e0b0 as *const (), Throws::Caught), // sub_14075e0b0
    Replacement::draft_throws(0x14075e3b0, "content_14075e3b0", fn_14075e3b0 as *const (), Throws::Caught), // sub_14075e3b0
    Replacement::draft_throws(0x14075e6b0, "content_14075e6b0", fn_14075e6b0 as *const (), Throws::Caught), // sub_14075e6b0
    Replacement::draft_throws(0x14075e9b0, "content_14075e9b0", fn_14075e9b0 as *const (), Throws::Caught), // sub_14075e9b0
    Replacement::draft_throws(0x14075ecb0, "content_14075ecb0", tpl_838::<0x140760b30> as *const (), Throws::Discard), // sub_14075ecb0
    Replacement::draft_throws(0x14075edb0, "content_14075edb0", fn_14075edb0 as *const (), Throws::Caught), // sub_14075edb0
    Replacement::draft_throws(0x14075eee0, "content_14075eee0", fn_14075eee0 as *const (), Throws::Caught), // sub_14075eee0
    Replacement::draft_throws(0x14075f010, "content_14075f010", fn_14075f010 as *const (), Throws::Caught), // sub_14075f010
    Replacement::draft_throws(0x14075f310, "content_14075f310", fn_14075f310 as *const (), Throws::Caught), // sub_14075f310
    Replacement::draft_throws(0x14075f610, "content_14075f610", fn_14075f610 as *const (), Throws::Caught), // sub_14075f610
    Replacement::draft_throws(0x14075f910, "content_14075f910", fn_14075f910 as *const (), Throws::Caught), // sub_14075f910
    Replacement::draft_throws(0x14075f9f0, "content_14075f9f0", fn_14075f9f0 as *const (), Throws::Caught), // sub_14075f9f0
    Replacement::draft_throws(0x14075fd10, "content_14075fd10", fn_14075fd10 as *const (), Throws::Caught), // sub_14075fd10
    Replacement::draft_throws(0x140760030, "content_140760030", tpl_66193::<0x142442de0, 0x143a38228, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140760030
    Replacement::draft_throws(0x140760290, "content_140760290", tpl_66331::<0x142442de0, 0x143a38298, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140760290
    Replacement::draft_throws(0x1407604f0, "content_1407604f0", fn_1407604f0 as *const (), Throws::Caught), // sub_1407604f0
    Replacement::draft_throws(0x140760810, "content_140760810", fn_140760810 as *const (), Throws::Caught), // sub_140760810
    Replacement::draft_throws(0x140760b30, "content_140760b30", fn_140760b30 as *const (), Throws::Caught), // sub_140760b30
    Replacement::draft_throws(0x140760e20, "content_140760e20", fn_140760e20 as *const (), Throws::Caught), // sub_140760e20
    Replacement::draft_throws(0x140761110, "content_140761110", fn_140761110 as *const (), Throws::Caught), // sub_140761110
    Replacement::draft_throws(0x1407613d0, "content_1407613d0", fn_1407613d0 as *const (), Throws::Caught), // sub_1407613d0
    Replacement::draft_throws(0x140761690, "content_140761690", fn_140761690 as *const (), Throws::Caught), // sub_140761690
    Replacement::draft_throws(0x140761950, "content_140761950", fn_140761950 as *const (), Throws::Caught), // sub_140761950
    Replacement::draft_throws(0x140761c10, "content_140761c10", fn_140761c10 as *const (), Throws::Caught), // sub_140761c10
    Replacement::draft_throws(0x140761ce0, "content_140761ce0", fn_140761ce0 as *const (), Throws::Caught), // sub_140761ce0
    Replacement::draft(0x140761db0, "content_140761db0", fn_140761db0 as *const ()), // sub_140761db0
    Replacement::draft(0x140761e30, "content_140761e30", fn_140761e30 as *const ()), // sub_140761e30
    Replacement::draft_throws(0x140761f40, "content_140761f40", fn_140761f40 as *const (), Throws::Caught), // sub_140761f40
    Replacement::draft(0x140762060, "content_140762060", fn_140762060 as *const ()), // sub_140762060
    Replacement::draft_throws(0x140762160, "content_140762160", fn_140762160 as *const (), Throws::Discard), // sub_140762160
    Replacement::draft(0x140762240, "content_140762240", fn_140762240 as *const ()), // sub_140762240
    Replacement::draft_throws(0x1407622a0, "content_1407622a0", fn_1407622a0 as *const (), Throws::Caught), // sub_1407622a0
    Replacement::draft_throws(0x140762500, "content_140762500", fn_140762500 as *const (), Throws::Caught), // sub_140762500
    Replacement::draft_throws(0x140762a80, "content_140762a80", fn_140762a80 as *const (), Throws::Caught), // sub_140762a80
    Replacement::draft_throws(0x140762cb0, "content_140762cb0", fn_140762cb0 as *const (), Throws::Caught), // sub_140762cb0
    Replacement::draft_throws(0x140763200, "content_140763200", fn_140763200 as *const (), Throws::Caught), // sub_140763200
    Replacement::draft_throws(0x140763420, "content_140763420", fn_140763420 as *const (), Throws::Caught), // sub_140763420
    Replacement::draft(0x140763860, "content_140763860", fn_140763860 as *const ()), // sub_140763860
    Replacement::draft(0x1407639f0, "content_1407639f0", fn_1407639f0 as *const ()), // sub_1407639f0
    Replacement::draft(0x140763ac0, "content_140763ac0", fn_140763ac0 as *const ()), // sub_140763ac0
    Replacement::draft_throws(0x140763ba0, "content_140763ba0", fn_140763ba0 as *const (), Throws::Discard), // sub_140763ba0
    Replacement::draft_throws(0x140763c80, "content_140763c80", fn_140763c80 as *const (), Throws::Discard), // sub_140763c80
    Replacement::draft(0x140763d90, "content_140763d90", fn_140763d90 as *const ()), // sub_140763d90
    Replacement::draft_throws(0x140763e00, "content_140763e00", tpl_7080::<0x1401ffe00, 0x140708f60, 0x14076b620, 0x142442de0, 0x143a38228, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_140763e00
    Replacement::draft_throws(0x1407640a0, "content_1407640a0", tpl_7370::<0x1401ffe00, 0x140708f60, 0x14076b540, 0x142442de0, 0x143a38298, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_1407640a0
    Replacement::draft_throws(0x140764340, "content_140764340", fn_140764340 as *const (), Throws::Caught), // sub_140764340
    Replacement::draft_throws(0x140764810, "content_140764810", fn_140764810 as *const (), Throws::Caught), // sub_140764810
    Replacement::draft_throws(0x140764ce0, "content_140764ce0", fn_140764ce0 as *const (), Throws::Caught), // sub_140764ce0
    Replacement::draft_throws(0x140765160, "content_140765160", fn_140765160 as *const (), Throws::Caught), // sub_140765160
    Replacement::draft_throws(0x1407655e0, "content_1407655e0", fn_1407655e0 as *const (), Throws::Caught), // sub_1407655e0
    Replacement::draft_throws(0x140765a60, "content_140765a60", fn_140765a60 as *const (), Throws::Caught), // sub_140765a60
    Replacement::draft_throws(0x140765ee0, "content_140765ee0", fn_140765ee0 as *const (), Throws::Caught), // sub_140765ee0
    Replacement::draft_throws(0x1407660e0, "content_1407660e0", fn_1407660e0 as *const (), Throws::Caught), // sub_1407660e0
    Replacement::draft(0x1407662e0, "content_1407662e0", fn_1407662e0 as *const ()), // sub_1407662e0
    Replacement::draft_throws(0x1407664a0, "content_1407664a0", fn_1407664a0 as *const (), Throws::Caught), // sub_1407664a0
    Replacement::draft(0x140766650, "content_140766650", fn_140766650 as *const ()), // sub_140766650
    Replacement::draft(0x1407667e0, "content_1407667e0", fn_1407667e0 as *const ()), // sub_1407667e0
    Replacement::draft_throws(0x140766920, "content_140766920", fn_140766920 as *const (), Throws::Caught), // sub_140766920
    Replacement::draft(0x140766ad0, "content_140766ad0", fn_140766ad0 as *const ()), // sub_140766ad0
    Replacement::draft(0x140766c40, "content_140766c40", fn_140766c40 as *const ()), // sub_140766c40
    Replacement::draft_throws(0x140766d90, "content_140766d90", fn_140766d90 as *const (), Throws::Caught), // sub_140766d90
    Replacement::draft(0x140766ec0, "content_140766ec0", fn_140766ec0 as *const ()), // sub_140766ec0
    Replacement::draft_throws(0x140766f60, "content_140766f60", fn_140766f60 as *const (), Throws::Caught), // sub_140766f60
    Replacement::draft_throws(0x140767a20, "content_140767a20", fn_140767a20 as *const (), Throws::Caught), // sub_140767a20
    Replacement::draft_throws(0x1407684e0, "content_1407684e0", fn_1407684e0 as *const (), Throws::Caught), // sub_1407684e0
    Replacement::draft_throws(0x140768f70, "content_140768f70", fn_140768f70 as *const (), Throws::Caught), // sub_140768f70
    Replacement::draft_throws(0x140769a00, "content_140769a00", fn_140769a00 as *const (), Throws::Caught), // sub_140769a00
    Replacement::draft_throws(0x14076a480, "content_14076a480", fn_14076a480 as *const (), Throws::Caught), // sub_14076a480
    Replacement::draft_throws(0x14076af00, "content_14076af00", fn_14076af00 as *const (), Throws::Caught), // sub_14076af00
    Replacement::draft_throws(0x14076b090, "content_14076b090", fn_14076b090 as *const (), Throws::Caught), // sub_14076b090
    Replacement::draft_throws(0x14076b220, "content_14076b220", fn_14076b220 as *const (), Throws::Caught), // sub_14076b220
    Replacement::draft_throws(0x14076b3b0, "content_14076b3b0", fn_14076b3b0 as *const (), Throws::Caught), // sub_14076b3b0
    Replacement::draft_throws(0x14076b540, "content_14076b540", tpl_65670::<0x142442de0, 0x143a38298, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_14076b540
    Replacement::draft_throws(0x14076b620, "content_14076b620", tpl_65670::<0x142442de0, 0x143a38228, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x70> as *const (), Throws::Discard), // sub_14076b620
    Replacement::draft_throws(0x14076b700, "content_14076b700", fn_14076b700 as *const (), Throws::Caught), // sub_14076b700
    Replacement::draft_throws(0x14076b840, "content_14076b840", fn_14076b840 as *const (), Throws::Caught), // sub_14076b840
    Replacement::draft_throws(0x14076b980, "content_14076b980", fn_14076b980 as *const (), Throws::Caught), // sub_14076b980
    Replacement::draft_throws(0x14076bf30, "content_14076bf30", fn_14076bf30 as *const (), Throws::Caught), // sub_14076bf30
    Replacement::draft(0x14076c4e0, "content_14076c4e0", tpl_7825::<0x3, 0x4, 0x8, 0x143a391d0, 0x143a391d4, 0x143a391d3, 0x143a391d8, 0x142409b50> as *const ()), // content::AcquireTicketDecorator::content::AcquireTicketDecorator::vfunc_1
    Replacement::draft_throws(0x14076c590, "content_14076c590", tpl_6639::<0x143cc31d0, 0x143cc3198, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AcquireTicketDecorator::content::AcquireTicketDecorator::vfunc_2
    Replacement::draft(0x14076c6e0, "content_14076c6e0", tpl_7399::<0x3, 0x4, 0x8, 0x143240680, 0x142409b50> as *const ()), // content::AngleCompareConditionDecorator::content::AngleCompareConditionDecorator::vfunc_1
    Replacement::draft_throws(0x14076c740, "content_14076c740", tpl_6639::<0x143cc3264, 0x143cc3280, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AngleCompareConditionDecorator::content::AngleCompareConditionDecorator::vfunc_2
    Replacement::draft(0x14076c860, "content_14076c860", tpl_7305::<0x3, 0x4, 0x8, 0x143240240, 0x142409b50> as *const ()), // content::AnimationControlParamDecorator::content::AnimationControlParamDecorator::vfunc_1
    Replacement::draft_throws(0x14076c8c0, "content_14076c8c0", tpl_6639::<0x143cc31d4, 0x143cc3230, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AnimationControlParamDecorator::content::AnimationControlParamDecorator::vfunc_2
    Replacement::draft(0x14076c9e0, "content_14076c9e0", tpl_7825::<0x3, 0x4, 0x8, 0x143a39260, 0x143a39264, 0x143a39263, 0x143a39268, 0x142409b50> as *const ()), // content::AvoidanceConfigScopeDecorator::content::AvoidanceConfigScopeDecorator::vfunc_1
    Replacement::draft_throws(0x14076ca90, "content_14076ca90", tpl_6639::<0x143cc31e0, 0x143cc3108, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AvoidanceConfigScopeDecorator::content::AvoidanceConfigScopeDecorator::vfunc_2
    Replacement::draft(0x14076cbb0, "content_14076cbb0", tpl_7309::<0x3, 0x4, 0x8, 0x14322fa18, 0x142409b50> as *const ()), // content::CalculateDirectionDecorator::content::CalculateDirectionDecorator::vfunc_1
    Replacement::draft_throws(0x14076cc10, "content_14076cc10", tpl_6639::<0x143cc3138, 0x143cc3148, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::CalculateDirectionDecorator::content::CalculateDirectionDecorator::vfunc_2
    Replacement::draft(0x14076cd30, "content_14076cd30", tpl_7401::<0x3, 0x4, 0x8, 0x14322f458, 0x142409b50> as *const ()), // content::CallScriptDecorator::content::CallScriptDecorator::vfunc_1
    Replacement::draft_throws(0x14076cd90, "content_14076cd90", tpl_6639::<0x143cc3178, 0x143cc3170, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::CallScriptDecorator::content::CallScriptDecorator::vfunc_2
    Replacement::draft(0x14076ceb0, "content_14076ceb0", tpl_7826::<0x3, 0x4, 0x8, 0x14322f430, 0x142409b50> as *const ()), // content::CopyPropertyDecorator::content::CopyPropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076cf10, "content_14076cf10", tpl_6639::<0x143cc31e4, 0x143cc3208, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::CopyPropertyDecorator::content::CopyPropertyDecorator::vfunc_2
    Replacement::draft(0x14076d030, "content_14076d030", tpl_7827::<0x3, 0x4, 0x8, 0x14322e808, 0x142409b50> as *const ()), // content::DamageTrackingDecorator::content::DamageTrackingDecorator::vfunc_1
    Replacement::draft_throws(0x14076d090, "content_14076d090", tpl_6639::<0x143cc31b0, 0x143cc31e8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DamageTrackingDecorator::content::DamageTrackingDecorator::vfunc_2
    Replacement::draft(0x14076d1b0, "content_14076d1b0", tpl_7305::<0x3, 0x4, 0x8, 0x14322e230, 0x142409b50> as *const ()), // content::DecoratorAlwaysSucceed::content::DecoratorAlwaysSucceed::vfunc_1
    Replacement::draft_throws(0x14076d210, "content_14076d210", tpl_6639::<0x143cc32bc, 0x143cc32d0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorAlwaysSucceed::content::DecoratorAlwaysSucceed::vfunc_2
    Replacement::draft(0x14076d330, "content_14076d330", tpl_7825::<0x3, 0x4, 0x8, 0x143a394d0, 0x143a394d4, 0x143a394d3, 0x143a394d8, 0x142409b50> as *const ()), // content::DecoratorBlockTicketStealing::content::DecoratorBlockTicketStealing::vfunc_1
    Replacement::draft_throws(0x14076d3e0, "content_14076d3e0", tpl_6639::<0x143cc313c, 0x143cc31a8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorBlockTicketStealing::content::DecoratorBlockTicketStealing::vfunc_2
    Replacement::draft(0x14076d500, "content_14076d500", tpl_7305::<0x3, 0x4, 0x8, 0x14322df08, 0x142409b50> as *const ()), // content::DecoratorBoolCondition::content::DecoratorBoolCondition::vfunc_1
    Replacement::draft_throws(0x14076d560, "content_14076d560", tpl_6639::<0x143cc321c, 0x143cc3258, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorBoolCondition::content::DecoratorBoolCondition::vfunc_2
    Replacement::draft(0x14076d680, "content_14076d680", tpl_7309::<0x3, 0x4, 0x8, 0x14322d918, 0x142409b50> as *const ()), // content::DecoratorCanChargeCondition::content::DecoratorCanChargeCondition::vfunc_1
    Replacement::draft_throws(0x14076d6e0, "content_14076d6e0", tpl_6639::<0x143cc32f8, 0x143cc30f0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorCanChargeCondition::content::DecoratorCanChargeCondition::vfunc_2
    Replacement::draft(0x14076d830, "content_14076d830", tpl_7827::<0x3, 0x4, 0x8, 0x14322d8f0, 0x142409b50> as *const ()), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_1
    Replacement::draft_throws(0x14076d890, "content_14076d890", tpl_6639::<0x143cc3100, 0x143cc3190, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_2
    Replacement::draft(0x14076d9b0, "content_14076d9b0", tpl_7826::<0x3, 0x4, 0x8, 0x14322d308, 0x142409b50> as *const ()), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_1
    Replacement::draft_throws(0x14076da10, "content_14076da10", tpl_6639::<0x143cc3168, 0x143cc3238, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_2
    Replacement::draft(0x14076db30, "content_14076db30", fn_14076db30 as *const ()), // content::DecoratorKnowledgeBulletinCondition::content::DecoratorKnowledgeBulletinCondition::vfunc_1
    Replacement::draft_throws(0x14076db90, "content_14076db90", tpl_6639::<0x143cc3298, 0x143cc3130, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorKnowledgeBulletinCondition::content::DecoratorKnowledgeBulletinCondition::vfunc_2
    Replacement::draft(0x14076dcb0, "content_14076dcb0", tpl_7825::<0x3, 0x4, 0x8, 0x143a392f0, 0x143a392f4, 0x143a392f3, 0x143a392f8, 0x142409b50> as *const ()), // content::DecoratorLostTicketCondition::content::DecoratorLostTicketCondition::vfunc_1
    Replacement::draft_throws(0x14076dd60, "content_14076dd60", tpl_6639::<0x143cc3154, 0x143cc3158, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorLostTicketCondition::content::DecoratorLostTicketCondition::vfunc_2
    Replacement::draft(0x14076de80, "content_14076de80", fn_14076de80 as *const ()), // content::DecoratorRestrictedCondition::content::DecoratorRestrictedCondition::vfunc_1
    Replacement::draft_throws(0x14076dee0, "content_14076dee0", tpl_6639::<0x143cc32c0, 0x143cc30e0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorRestrictedCondition::content::DecoratorRestrictedCondition::vfunc_2
    Replacement::draft(0x14076e000, "content_14076e000", tpl_7825::<0x3, 0x4, 0x8, 0x143a39380, 0x143a39384, 0x143a39383, 0x143a39388, 0x142409b50> as *const ()), // content::DecoratorTicketCondition::content::DecoratorTicketCondition::vfunc_1
    Replacement::draft_throws(0x14076e0b0, "content_14076e0b0", tpl_6639::<0x143cc3110, 0x143cc32e0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorTicketCondition::content::DecoratorTicketCondition::vfunc_2
    Replacement::draft(0x14076e1d0, "content_14076e1d0", tpl_7390::<0x3, 0x4, 0x8, 0x14322c440, 0x142409b50> as *const ()), // content::DisabledDecorator::content::DisabledDecorator::vfunc_1
    Replacement::draft_throws(0x14076e230, "content_14076e230", tpl_6639::<0x143cc3228, 0x143cc30f8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DisabledDecorator::content::DisabledDecorator::vfunc_2
    Replacement::draft(0x14076e350, "content_14076e350", tpl_7308::<0x3, 0x4, 0x8, 0x14322c130, 0x142409b50> as *const ()), // content::DistanceConditionDecorator::content::DistanceConditionDecorator::vfunc_1
    Replacement::draft_throws(0x14076e3b0, "content_14076e3b0", tpl_6639::<0x143cc32e8, 0x143cc32a8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DistanceConditionDecorator::content::DistanceConditionDecorator::vfunc_2
    Replacement::draft(0x14076e4d0, "content_14076e4d0", tpl_7830::<0x3, 0x4, 0x8, 0x143225888, 0x142409b50> as *const ()), // content::FloatDifferenceConditionDecorator::content::FloatDifferenceConditionDecorator::vfunc_1
    Replacement::draft_throws(0x14076e530, "content_14076e530", tpl_6639::<0x143cc3118, 0x143cc3220, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::FloatDifferenceConditionDecorator::content::FloatDifferenceConditionDecorator::vfunc_2
    Replacement::draft(0x14076e650, "content_14076e650", tpl_7305::<0x3, 0x4, 0x8, 0x143212d38, 0x142409b50> as *const ()), // content::SpeedMatchingDecorator::content::SpeedMatchingDecorator::vfunc_1
    Replacement::draft_throws(0x14076e6b0, "content_14076e6b0", tpl_6639::<0x143c89f78, 0x143c89f70, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SpeedMatchingDecorator::content::SpeedMatchingDecorator::vfunc_2
    Replacement::draft(0x14076e7d0, "content_14076e7d0", tpl_7388::<0x3, 0x4, 0x8, 0x143225588, 0x142409b50> as *const ()), // content::FollowSpeedDecorator::content::FollowSpeedDecorator::vfunc_1
    Replacement::draft_throws(0x14076e830, "content_14076e830", tpl_6639::<0x143cc3288, 0x143cc30e8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::FollowSpeedDecorator::content::FollowSpeedDecorator::vfunc_2
    Replacement::draft(0x14076e950, "content_14076e950", tpl_7387::<0x3, 0x4, 0x8, 0x143225288, 0x142409b50> as *const ()), // content::GuideSpeedDecorator::content::GuideSpeedDecorator::vfunc_1
    Replacement::draft_throws(0x14076e9b0, "content_14076e9b0", tpl_6639::<0x143cc3188, 0x143cc31c0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::GuideSpeedDecorator::content::GuideSpeedDecorator::vfunc_2
    Replacement::draft(0x14076ead0, "content_14076ead0", tpl_7400::<0x3, 0x4, 0x8, 0x143223260, 0x142409b50> as *const ()), // content::IsPropertySetConditionDecorator::content::IsPropertySetConditionDecorator::vfunc_1
    Replacement::draft_throws(0x14076eb30, "content_14076eb30", tpl_6639::<0x143cc3140, 0x143cc31f0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::IsPropertySetConditionDecorator::content::IsPropertySetConditionDecorator::vfunc_2
    Replacement::draft(0x14076ec50, "content_14076ec50", tpl_7387::<0x3, 0x4, 0x8, 0x143222988, 0x142409b50> as *const ()), // content::LogicBlockDecorator::content::LogicBlockDecorator::vfunc_1
    Replacement::draft_throws(0x14076ecb0, "content_14076ecb0", tpl_6639::<0x143cc329c, 0x143cc3248, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::LogicBlockDecorator::content::LogicBlockDecorator::vfunc_2
    Replacement::draft(0x14076edd0, "content_14076edd0", tpl_7308::<0x3, 0x4, 0x8, 0x143222320, 0x142409b50> as *const ()), // content::ModifyIntPropertyDecorator::content::ModifyIntPropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076ee30, "content_14076ee30", tpl_6639::<0x143cc3104, 0x143cc3120, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::ModifyIntPropertyDecorator::content::ModifyIntPropertyDecorator::vfunc_2
    Replacement::draft(0x14076ef50, "content_14076ef50", tpl_7389::<0x3, 0x4, 0x8, 0x143217288, 0x142409b50> as *const ()), // content::OnStateCompletedDecorator::content::OnStateCompletedDecorator::vfunc_1
    Replacement::draft_throws(0x14076efb0, "content_14076efb0", tpl_6639::<0x143cc32cc, 0x143cc3210, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::OnStateCompletedDecorator::content::OnStateCompletedDecorator::vfunc_2
    Replacement::draft(0x14076f0d0, "content_14076f0d0", tpl_7825::<0x3, 0x4, 0x8, 0x143a39410, 0x143a39414, 0x143a39413, 0x143a39418, 0x142409b50> as *const ()), // content::ReleaseTicketDecorator::content::ReleaseTicketDecorator::vfunc_1
    Replacement::draft_throws(0x14076f180, "content_14076f180", tpl_6639::<0x143cc3150, 0x143cc32f0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::ReleaseTicketDecorator::content::ReleaseTicketDecorator::vfunc_2
    Replacement::draft(0x14076f2a0, "content_14076f2a0", tpl_7402::<0x3, 0x4, 0x8, 0x143214398, 0x142409b50> as *const ()), // content::SayDecorator::content::SayDecorator::vfunc_1
    Replacement::draft_throws(0x14076f300, "content_14076f300", tpl_6639::<0x143cc3260, 0x143cc32b0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SayDecorator::content::SayDecorator::vfunc_2
    Replacement::draft(0x14076f420, "content_14076f420", tpl_7831::<0x3, 0x4, 0x8, 0x143250ea8, 0x142409b50> as *const ()), // content::SetKnowledgeBulletinDecorator::content::SetKnowledgeBulletinDecorator::vfunc_1
    Replacement::draft_throws(0x14076f480, "content_14076f480", tpl_6639::<0x143cc3218, 0x143cc32a0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetKnowledgeBulletinDecorator::content::SetKnowledgeBulletinDecorator::vfunc_2
    Replacement::draft(0x14076f5a0, "content_14076f5a0", tpl_7831::<0x3, 0x4, 0x8, 0x143213138, 0x142409b50> as *const ()), // content::SetKnowledgePropertyDecorator::content::SetKnowledgePropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076f600, "content_14076f600", tpl_6639::<0x143cc3114, 0x143cc32d8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetKnowledgePropertyDecorator::content::SetKnowledgePropertyDecorator::vfunc_2
    Replacement::draft(0x14076f720, "content_14076f720", tpl_7306::<0x3, 0x4, 0x8, 0x1432131d8, 0x142409b50> as *const ()), // content::SetIntKnowledgePropertyDecorator::content::SetIntKnowledgePropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076f780, "content_14076f780", tpl_6639::<0x143cc3128, 0x143cc31b8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetIntKnowledgePropertyDecorator::content::SetIntKnowledgePropertyDecorator::vfunc_2
    Replacement::draft(0x14076f8a0, "content_14076f8a0", tpl_7832::<0x3, 0x4, 0x8, 0x1432131a0, 0x142409b50> as *const ()), // content::SetRandomIntKnowledgePropertyDecorator::content::SetRandomIntKnowledgePropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076f900, "content_14076f900", tpl_6639::<0x143cc32c8, 0x143cc31d8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetRandomIntKnowledgePropertyDecorator::content::SetRandomIntKnowledgePropertyDecorator::vfunc_2
    Replacement::draft(0x14076fa20, "content_14076fa20", fn_14076fa20 as *const ()), // content::SetFloatKnowledgePropertyDecorator::content::SetFloatKnowledgePropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076fa80, "content_14076fa80", tpl_6639::<0x143cc3250, 0x143cc31c8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetFloatKnowledgePropertyDecorator::content::SetFloatKnowledgePropertyDecorator::vfunc_2
    Replacement::draft(0x14076fba0, "content_14076fba0", tpl_7397::<0x3, 0x4, 0x8, 0x143213210, 0x142409b50> as *const ()), // content::SetRandomFloatKnowledgePropertyDecorator::content::SetRandomFloatKnowledgePropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076fc00, "content_14076fc00", tpl_6639::<0x143cc3240, 0x143cc3180, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetRandomFloatKnowledgePropertyDecorator::content::SetRandomFloatKnowledgePropertyDecorator::vfunc_2
    Replacement::draft(0x14076fd20, "content_14076fd20", tpl_7830::<0x3, 0x4, 0x8, 0x143213168, 0x142409b50> as *const ()), // content::SetBoolKnowledgePropertyDecorator::content::SetBoolKnowledgePropertyDecorator::vfunc_1
    Replacement::draft_throws(0x14076fd80, "content_14076fd80", tpl_6639::<0x143cc32b8, 0x143cc3290, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetBoolKnowledgePropertyDecorator::content::SetBoolKnowledgePropertyDecorator::vfunc_2
    Replacement::draft(0x14076fea0, "content_14076fea0", tpl_7400::<0x3, 0x4, 0x8, 0x143212e38, 0x142409b50> as *const ()), // content::SetPositionReservationDecorator::content::SetPositionReservationDecorator::vfunc_1
    Replacement::draft_throws(0x14076ff00, "content_14076ff00", tpl_6639::<0x143cc31a0, 0x143cc3270, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetPositionReservationDecorator::content::SetPositionReservationDecorator::vfunc_2
    Replacement::draft(0x140770020, "content_140770020", tpl_7832::<0x3, 0x4, 0x8, 0x143211d58, 0x142409b50> as *const ()), // content::TimeSincePropertySetConditionDecorator::content::TimeSincePropertySetConditionDecorator::vfunc_1
    Replacement::draft_throws(0x140770080, "content_140770080", tpl_6639::<0x143cc3278, 0x143cc3160, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::TimeSincePropertySetConditionDecorator::content::TimeSincePropertySetConditionDecorator::vfunc_2
    Replacement::draft(0x1407701a0, "content_1407701a0", tpl_7305::<0x3, 0x4, 0x8, 0x143211d30, 0x142409b50> as *const ()), // content::UnsetPropertyDecorator::content::UnsetPropertyDecorator::vfunc_1
    Replacement::draft_throws(0x140770200, "content_140770200", tpl_6639::<0x143cc32c4, 0x143cc3200, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::UnsetPropertyDecorator::content::UnsetPropertyDecorator::vfunc_2
    Replacement::draft(0x140770320, "content_140770320", tpl_7388::<0x3, 0x4, 0x8, 0x143211490, 0x142409b50> as *const ()), // content::WarningShotDecorator::content::WarningShotDecorator::vfunc_1
    Replacement::draft_throws(0x140770380, "content_140770380", tpl_6639::<0x143cc31f8, 0x143cc3268, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::WarningShotDecorator::content::WarningShotDecorator::vfunc_2
    Replacement::draft(0x1407704a0, "content_1407704a0", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432577d0, 0x20> as *const ()), // sub_1407704a0
    Replacement::draft(0x140770530, "content_140770530", tpl_7835::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432577d0, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140770530
    Replacement::draft(0x140770650, "content_140770650", tpl_7836::<0x14387d020, 0x14243b0b0, 0x1406ea470, 0x20, 0x1426d4580, 0x338, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140770650
    Replacement::draft_throws(0x140770700, "content_140770700", fn_140770700 as *const (), Throws::Caught), // sub_140770700
    Replacement::draft_throws(0x1407707d0, "content_1407707d0", tpl_7838::<0x14387d020, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x14240ab10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_1407707d0
    Replacement::draft_throws(0x140770890, "content_140770890", tpl_7839::<0x14387cf48, 0x14387d020, 0x1401e9b60, 0x1406ea8d0, 0x1426cdcb0, 0x20, 0x1406a6ae0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140770890
    Replacement::draft_throws(0x140770910, "content_140770910", fn_140770910 as *const (), Throws::Discard), // sub_140770910
    Replacement::draft_throws(0x1407709b0, "content_1407709b0", fn_1407709b0 as *const (), Throws::Caught), // sub_1407709b0
    Replacement::draft_throws(0x140770bb0, "content_140770bb0", fn_140770bb0 as *const (), Throws::Caught), // sub_140770bb0
    Replacement::draft_throws(0x140770d80, "content_140770d80", fn_140770d80 as *const (), Throws::Caught), // sub_140770d80
    Replacement::draft(0x140770f40, "content_140770f40", tpl_7844::<0x14387d020, 0x14243b0b0, 0x1406ec310, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_140770f40
    Replacement::draft(0x140771090, "content_140771090", tpl_7845::<0x14387cf48, 0x14387d020, 0x1401e9970, 0x1406ec510, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x1401e9860> as *const ()), // sub_140771090
    Replacement::draft(0x140771120, "content_140771120", tpl_7846::<0x14387d020, 0x14243b0b0, 0x1406ec600, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140771120
    Replacement::draft_throws(0x1407711c0, "content_1407711c0", tpl_7847::<0x14387cf48, 0x14387d020, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1407711c0
    Replacement::draft_throws(0x140771260, "content_140771260", tpl_7848::<0x14387d020, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x1432515c8, 0x1406d6fb0> as *const (), Throws::Discard), // sub_140771260
    Replacement::draft(0x140771310, "content_140771310", tpl_7849::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0, 0x143255890> as *const ()), // sub_140771310
    Replacement::draft(0x1407713b0, "content_1407713b0", tpl_66744::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143255890, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_1407713b0
    Replacement::draft(0x1407714e0, "content_1407714e0", tpl_7850::<0x14387d008, 0x14243b0b0, 0x140785e90, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407714e0
    Replacement::draft_throws(0x140771590, "content_140771590", tpl_7851::<0x14387cea8, 0x14387d008, 0x1401e9fa0, 0x140785f80, 0x328, 0x338, 0x118, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140771590
    Replacement::draft_throws(0x140771620, "content_140771620", tpl_7852::<0x14387d008, 0x14243b0b0, 0x140786060, 0x338, 0x118, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140771620
    Replacement::draft_throws(0x1407716a0, "content_1407716a0", tpl_7853::<0x14387cea8, 0x14387d008, 0x1401e9b60, 0x140786110, 0x328, 0x338, 0x120, 0x118, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407716a0
    Replacement::draft_throws(0x140771740, "content_140771740", fn_140771740 as *const (), Throws::Caught), // sub_140771740
    Replacement::draft_throws(0x140771820, "content_140771820", fn_140771820 as *const (), Throws::Caught), // sub_140771820
    Replacement::draft_throws(0x140771900, "content_140771900", fn_140771900 as *const (), Throws::Caught), // sub_140771900
    Replacement::draft(0x1407719e0, "content_1407719e0", tpl_7855::<0x14387d008, 0x14243b0b0, 0x140786a20, 0x338, 0x118, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407719e0
    Replacement::draft(0x140771a60, "content_140771a60", tpl_7856::<0x14387cea8, 0x14387d008, 0x1401e9970, 0x140786ae0, 0x338, 0x118, 0xd8, 0x1401e9860> as *const ()), // sub_140771a60
    Replacement::draft(0x140771ad0, "content_140771ad0", tpl_7857::<0x14387d008, 0x14243b0b0, 0x140786b90, 0x338, 0x118, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140771ad0
    Replacement::draft_throws(0x140771b50, "content_140771b50", tpl_7858::<0x14387cea8, 0x14387d008, 0x1401e9770, 0x140786c50, 0x338, 0x118, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140771b50
    Replacement::draft_throws(0x140771bd0, "content_140771bd0", tpl_7859::<0x14387d008, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140786d00, 0x118, 0x1432515b0, 0x1, 0x14020f240> as *const (), Throws::Discard), // sub_140771bd0
    Replacement::draft(0x140771ca0, "content_140771ca0", tpl_7860::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410, 0x1432515b0, 0x1, 0x14020f270> as *const ()), // sub_140771ca0
    Replacement::draft(0x140771d20, "content_140771d20", tpl_7861::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143255390, 0x20> as *const ()), // sub_140771d20
    Replacement::draft(0x140771db0, "content_140771db0", tpl_7862::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143255390, 0xffffffffffffffec, 0x4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140771db0
    Replacement::draft(0x140771ed0, "content_140771ed0", fn_140771ed0 as *const ()), // sub_140771ed0
    Replacement::draft_throws(0x140771f90, "content_140771f90", fn_140771f90 as *const (), Throws::Caught), // sub_140771f90
    Replacement::draft_throws(0x140772060, "content_140772060", fn_140772060 as *const (), Throws::Discard), // sub_140772060
    Replacement::draft_throws(0x140772120, "content_140772120", tpl_7839::<0x14387cf20, 0x14387ce98, 0x1401e9b60, 0x1406ea8d0, 0x1426cdcb0, 0x20, 0x1402c1540, 0x1401e9a20> as *const (), Throws::Discard), // sub_140772120
    Replacement::draft_throws(0x1407721a0, "content_1407721a0", fn_1407721a0 as *const (), Throws::Discard), // sub_1407721a0
    Replacement::draft_throws(0x140772240, "content_140772240", fn_140772240 as *const (), Throws::Caught), // sub_140772240
    Replacement::draft_throws(0x140772440, "content_140772440", fn_140772440 as *const (), Throws::Caught), // sub_140772440
    Replacement::draft_throws(0x140772600, "content_140772600", fn_140772600 as *const (), Throws::Caught), // sub_140772600
    Replacement::draft(0x1407727c0, "content_1407727c0", fn_1407727c0 as *const ()), // sub_1407727c0
    Replacement::draft(0x140772900, "content_140772900", tpl_7871::<0x14387cf20, 0x14387ce98, 0x1401e9970, 0x1406ec510, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0xe0, 0x1401e9860> as *const ()), // sub_140772900
    Replacement::draft(0x1407729a0, "content_1407729a0", fn_1407729a0 as *const ()), // sub_1407729a0
    Replacement::draft_throws(0x140772a40, "content_140772a40", tpl_7873::<0x14387cf20, 0x14387ce98, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140772a40
    Replacement::draft_throws(0x140772ad0, "content_140772ad0", fn_140772ad0 as *const (), Throws::Discard), // sub_140772ad0
    Replacement::draft(0x140772ba0, "content_140772ba0", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143253310, 0x20> as *const ()), // sub_140772ba0
    Replacement::draft(0x140772c30, "content_140772c30", tpl_7835::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143253310, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140772c30
    Replacement::draft(0x140772d50, "content_140772d50", tpl_7836::<0x14387cef8, 0x14243b0b0, 0x1406ea470, 0x20, 0x1426d4580, 0x338, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140772d50
    Replacement::draft_throws(0x140772e00, "content_140772e00", fn_140772e00 as *const (), Throws::Caught), // sub_140772e00
    Replacement::draft_throws(0x140772ed0, "content_140772ed0", tpl_7838::<0x14387cef8, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x14240ab10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_140772ed0
    Replacement::draft_throws(0x140772f90, "content_140772f90", tpl_7839::<0x14387ceec, 0x14387cef8, 0x1401e9b60, 0x1406ea8d0, 0x1426cdcb0, 0x20, 0x1406a6ae0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140772f90
    Replacement::draft_throws(0x140773010, "content_140773010", fn_140773010 as *const (), Throws::Caught), // sub_140773010
    Replacement::draft_throws(0x140773210, "content_140773210", fn_140773210 as *const (), Throws::Caught), // sub_140773210
    Replacement::draft_throws(0x1407733e0, "content_1407733e0", fn_1407733e0 as *const (), Throws::Caught), // sub_1407733e0
    Replacement::draft(0x1407735a0, "content_1407735a0", tpl_7844::<0x14387cef8, 0x14243b0b0, 0x1406ec310, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_1407735a0
    Replacement::draft(0x1407736f0, "content_1407736f0", tpl_7845::<0x14387ceec, 0x14387cef8, 0x1401e9970, 0x1406ec510, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x1401e9860> as *const ()), // sub_1407736f0
    Replacement::draft(0x140773780, "content_140773780", tpl_7846::<0x14387cef8, 0x14243b0b0, 0x1406ec600, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140773780
    Replacement::draft_throws(0x140773820, "content_140773820", tpl_7847::<0x14387ceec, 0x14387cef8, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140773820
    Replacement::draft_throws(0x1407738c0, "content_1407738c0", tpl_7848::<0x14387cef8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x1432517b8, 0x1406d6fb0> as *const (), Throws::Discard), // sub_1407738c0
    Replacement::draft(0x140773970, "content_140773970", fn_140773970 as *const ()), // sub_140773970
    Replacement::draft(0x140773a00, "content_140773a00", fn_140773a00 as *const ()), // sub_140773a00
    Replacement::draft(0x140773ab0, "content_140773ab0", fn_140773ab0 as *const ()), // sub_140773ab0
    Replacement::draft(0x140773be0, "content_140773be0", fn_140773be0 as *const ()), // sub_140773be0
    Replacement::draft_throws(0x140773cc0, "content_140773cc0", fn_140773cc0 as *const (), Throws::Discard), // sub_140773cc0
    Replacement::draft_throws(0x140773d80, "content_140773d80", fn_140773d80 as *const (), Throws::Discard), // sub_140773d80
    Replacement::draft_throws(0x140773e30, "content_140773e30", fn_140773e30 as *const (), Throws::Discard), // sub_140773e30
    Replacement::draft_throws(0x140773f00, "content_140773f00", fn_140773f00 as *const (), Throws::Discard), // sub_140773f00
    Replacement::draft_throws(0x140774060, "content_140774060", fn_140774060 as *const (), Throws::Caught), // sub_140774060
    Replacement::draft_throws(0x1407742c0, "content_1407742c0", fn_1407742c0 as *const (), Throws::Caught), // sub_1407742c0
    Replacement::draft_throws(0x1407744f0, "content_1407744f0", fn_1407744f0 as *const (), Throws::Caught), // sub_1407744f0
    Replacement::draft(0x140774710, "content_140774710", fn_140774710 as *const ()), // sub_140774710
    Replacement::draft(0x1407747e0, "content_1407747e0", fn_1407747e0 as *const ()), // sub_1407747e0
    Replacement::draft(0x140774890, "content_140774890", fn_140774890 as *const ()), // sub_140774890
    Replacement::draft_throws(0x140774950, "content_140774950", fn_140774950 as *const (), Throws::Discard), // sub_140774950
    Replacement::draft_throws(0x1407749f0, "content_1407749f0", fn_1407749f0 as *const (), Throws::Discard), // sub_1407749f0
    Replacement::draft(0x140774af0, "content_140774af0", tpl_7893::<0x1406ec8f0, 0x14384e410, 0x20, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140774af0
    Replacement::draft(0x140774b50, "content_140774b50", tpl_7893::<0x1406eca20, 0x14384e410, 0x20, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140774b50
    Replacement::draft(0x140774bb0, "content_140774bb0", fn_140774bb0 as *const ()), // sub_140774bb0
    Replacement::draft(0x140774c70, "content_140774c70", fn_140774c70 as *const ()), // sub_140774c70
    Replacement::draft(0x140774db0, "content_140774db0", tpl_7896::<0x14387cf90, 0x14243b0b0, 0x1406ea470, 0x338, 0x20, 0x1426d4580, 0x1, 0x2c, 0x27, 0x28, 0x24, 0x1401ee7c0> as *const ()), // sub_140774db0
    Replacement::draft_throws(0x140774eb0, "content_140774eb0", tpl_7897::<0x14387cf10, 0x14387cf90, 0x1401e9fa0, 0x1406ea600, 0x328, 0x20, 0x338, 0x48, 0x369, 0x24, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140774eb0
    Replacement::draft_throws(0x140774f90, "content_140774f90", tpl_7898::<0x14387cf90, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140774f90
    Replacement::draft_throws(0x140775020, "content_140775020", tpl_7899::<0x14387cf10, 0x14387cf90, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x369, 0x20, 0x24, 0x1424b4da0, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140775020
    Replacement::draft_throws(0x140775100, "content_140775100", fn_140775100 as *const (), Throws::Discard), // sub_140775100
    Replacement::draft_throws(0x1407751c0, "content_1407751c0", fn_1407751c0 as *const (), Throws::Caught), // sub_1407751c0
    Replacement::draft_throws(0x1407753f0, "content_1407753f0", fn_1407753f0 as *const (), Throws::Caught), // sub_1407753f0
    Replacement::draft_throws(0x1407755e0, "content_1407755e0", fn_1407755e0 as *const (), Throws::Caught), // sub_1407755e0
    Replacement::draft(0x140775910, "content_140775910", tpl_7904::<0x14387cf90, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x24, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140775910
    Replacement::draft(0x1407759c0, "content_1407759c0", tpl_7905::<0x14387cf10, 0x14387cf90, 0x1401e9970, 0x1406ec510, 0x20, 0x338, 0xe8, 0x24, 0xc0, 0x1401e9860> as *const ()), // sub_1407759c0
    Replacement::draft(0x140775a50, "content_140775a50", tpl_7906::<0x14387cf90, 0x14243b0b0, 0x1406ec600, 0x20, 0x338, 0xe8, 0x24, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140775a50
    Replacement::draft_throws(0x140775af0, "content_140775af0", tpl_66780::<0x14387cf10, 0x14387cf90, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x24, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_140775af0
    Replacement::draft_throws(0x140775b80, "content_140775b80", tpl_7908::<0x14387cf90, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x143251798, 0x1426cdcb0, 0x24, 0x143251740, 0x1402c4730> as *const (), Throws::Discard), // sub_140775b80
    Replacement::draft(0x140775c70, "content_140775c70", tpl_7496::<0x1406ec8f0, 0x14384e410, 0x20, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140775c70
    Replacement::draft(0x140775ce0, "content_140775ce0", tpl_7496::<0x1406eca20, 0x14384e410, 0x20, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140775ce0
    Replacement::draft(0x140775d50, "content_140775d50", tpl_7909::<0x120, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b5d0> as *const ()), // sub_140775d50
    Replacement::draft(0x140775de0, "content_140775de0", tpl_66781::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x14079b5d0> as *const ()), // sub_140775de0
    Replacement::draft(0x140775e90, "content_140775e90", fn_140775e90 as *const ()), // sub_140775e90
    Replacement::draft(0x140775f80, "content_140775f80", fn_140775f80 as *const ()), // sub_140775f80
    Replacement::draft_throws(0x140776050, "content_140776050", fn_140776050 as *const (), Throws::Discard), // sub_140776050
    Replacement::draft_throws(0x140776100, "content_140776100", fn_140776100 as *const (), Throws::Discard), // sub_140776100
    Replacement::draft_throws(0x1407761a0, "content_1407761a0", fn_1407761a0 as *const (), Throws::Discard), // sub_1407761a0
    Replacement::draft_throws(0x140776260, "content_140776260", fn_140776260 as *const (), Throws::Discard), // sub_140776260
    Replacement::draft_throws(0x140776380, "content_140776380", fn_140776380 as *const (), Throws::Caught), // sub_140776380
    Replacement::draft_throws(0x1407765c0, "content_1407765c0", fn_1407765c0 as *const (), Throws::Caught), // sub_1407765c0
    Replacement::draft_throws(0x1407767d0, "content_1407767d0", fn_1407767d0 as *const (), Throws::Caught), // sub_1407767d0
    Replacement::draft(0x1407769e0, "content_1407769e0", fn_1407769e0 as *const ()), // sub_1407769e0
    Replacement::draft(0x140776a90, "content_140776a90", fn_140776a90 as *const ()), // sub_140776a90
    Replacement::draft(0x140776b30, "content_140776b30", fn_140776b30 as *const ()), // sub_140776b30
    Replacement::draft_throws(0x140776be0, "content_140776be0", fn_140776be0 as *const (), Throws::Discard), // sub_140776be0
    Replacement::draft_throws(0x140776c70, "content_140776c70", fn_140776c70 as *const (), Throws::Discard), // sub_140776c70
    Replacement::draft(0x140776d60, "content_140776d60", tpl_7526::<0x1406ec8f0, 0x14384e410, 0x20, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140776d60
    Replacement::draft(0x140776dd0, "content_140776dd0", tpl_7526::<0x1406eca20, 0x14384e410, 0x20, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140776dd0
    Replacement::draft(0x140776e40, "content_140776e40", fn_140776e40 as *const ()), // sub_140776e40
    Replacement::draft(0x140776ee0, "content_140776ee0", fn_140776ee0 as *const ()), // sub_140776ee0
    Replacement::draft(0x140776fa0, "content_140776fa0", tpl_7925::<0x14387cf58, 0x14243b0b0, 0x140785da0, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140776fa0
    Replacement::draft(0x140777040, "content_140777040", fn_140777040 as *const ()), // sub_140777040
    Replacement::draft_throws(0x140777190, "content_140777190", fn_140777190 as *const (), Throws::Discard), // sub_140777190
    Replacement::draft_throws(0x1407772e0, "content_1407772e0", fn_1407772e0 as *const (), Throws::Discard), // sub_1407772e0
    Replacement::draft_throws(0x1407773d0, "content_1407773d0", fn_1407773d0 as *const (), Throws::Discard), // sub_1407773d0
    Replacement::draft_throws(0x140777540, "content_140777540", fn_140777540 as *const (), Throws::Discard), // sub_140777540
    Replacement::draft_throws(0x1407775d0, "content_1407775d0", fn_1407775d0 as *const (), Throws::Caught), // sub_1407775d0
    Replacement::draft_throws(0x140777780, "content_140777780", fn_140777780 as *const (), Throws::Caught), // sub_140777780
    Replacement::draft_throws(0x140777930, "content_140777930", fn_140777930 as *const (), Throws::Caught), // sub_140777930
    Replacement::draft(0x140777ae0, "content_140777ae0", fn_140777ae0 as *const ()), // sub_140777ae0
    Replacement::draft(0x140777bd0, "content_140777bd0", fn_140777bd0 as *const ()), // sub_140777bd0
    Replacement::draft(0x140777cc0, "content_140777cc0", fn_140777cc0 as *const ()), // sub_140777cc0
    Replacement::draft_throws(0x140777dc0, "content_140777dc0", fn_140777dc0 as *const (), Throws::Discard), // sub_140777dc0
    Replacement::draft_throws(0x140777ed0, "content_140777ed0", fn_140777ed0 as *const (), Throws::Discard), // sub_140777ed0
    Replacement::draft(0x140778050, "content_140778050", fn_140778050 as *const ()), // sub_140778050
    Replacement::draft(0x140778100, "content_140778100", tpl_7938::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x143254450> as *const ()), // sub_140778100
    Replacement::draft(0x140778190, "content_140778190", tpl_7939::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143254450, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140778190
    Replacement::draft_throws(0x140778380, "content_140778380", fn_140778380 as *const (), Throws::Discard), // sub_140778380
    Replacement::draft_throws(0x140778410, "content_140778410", fn_140778410 as *const (), Throws::Caught), // sub_140778410
    Replacement::draft_throws(0x1407784b0, "content_1407784b0", fn_1407784b0 as *const (), Throws::Caught), // sub_1407784b0
    Replacement::draft_throws(0x140778550, "content_140778550", fn_140778550 as *const (), Throws::Caught), // sub_140778550
    Replacement::draft_throws(0x1407786d0, "content_1407786d0", tpl_7326::<0x14387ced8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0> as *const (), Throws::Discard), // sub_1407786d0
    Replacement::draft(0x140778770, "content_140778770", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143251d50, 0x20> as *const ()), // sub_140778770
    Replacement::draft(0x140778800, "content_140778800", tpl_7835::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143251d50, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140778800
    Replacement::draft(0x140778920, "content_140778920", tpl_7836::<0x14387cde0, 0x14243b0b0, 0x1406ea470, 0x20, 0x1426d4580, 0x338, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140778920
    Replacement::draft_throws(0x1407789d0, "content_1407789d0", fn_1407789d0 as *const (), Throws::Caught), // sub_1407789d0
    Replacement::draft_throws(0x140778aa0, "content_140778aa0", tpl_7838::<0x14387cde0, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x14240ab10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_140778aa0
    Replacement::draft_throws(0x140778b60, "content_140778b60", tpl_7839::<0x14387ce38, 0x14387cde0, 0x1401e9b60, 0x1406ea8d0, 0x1426cdcb0, 0x20, 0x1406a6ae0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140778b60
    Replacement::draft_throws(0x140778be0, "content_140778be0", fn_140778be0 as *const (), Throws::Caught), // sub_140778be0
    Replacement::draft_throws(0x140778de0, "content_140778de0", fn_140778de0 as *const (), Throws::Caught), // sub_140778de0
    Replacement::draft_throws(0x140778fb0, "content_140778fb0", fn_140778fb0 as *const (), Throws::Caught), // sub_140778fb0
    Replacement::draft(0x140779170, "content_140779170", tpl_7844::<0x14387cde0, 0x14243b0b0, 0x1406ec310, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_140779170
    Replacement::draft(0x1407792c0, "content_1407792c0", tpl_7845::<0x14387ce38, 0x14387cde0, 0x1401e9970, 0x1406ec510, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x1401e9860> as *const ()), // sub_1407792c0
    Replacement::draft(0x140779350, "content_140779350", tpl_7846::<0x14387cde0, 0x14243b0b0, 0x1406ec600, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140779350
    Replacement::draft_throws(0x1407793f0, "content_1407793f0", tpl_7847::<0x14387ce38, 0x14387cde0, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1407793f0
    Replacement::draft_throws(0x140779490, "content_140779490", tpl_7848::<0x14387cde0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x1432515c8, 0x1406d6fb0> as *const (), Throws::Discard), // sub_140779490
    Replacement::draft(0x140779540, "content_140779540", tpl_7849::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0, 0x1432557d0> as *const ()), // sub_140779540
    Replacement::draft(0x1407795e0, "content_1407795e0", tpl_66744::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432557d0, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_1407795e0
    Replacement::draft(0x1407796a0, "content_1407796a0", tpl_7312::<0x14387ce20, 0x14243b0b0, 0x140785da0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407796a0
    Replacement::draft(0x140779710, "content_140779710", fn_140779710 as *const ()), // sub_140779710
    Replacement::draft_throws(0x1407797b0, "content_1407797b0", fn_1407797b0 as *const (), Throws::Discard), // sub_1407797b0
    Replacement::draft_throws(0x140779840, "content_140779840", fn_140779840 as *const (), Throws::Discard), // sub_140779840
    Replacement::draft_throws(0x1407798c0, "content_1407798c0", fn_1407798c0 as *const (), Throws::Discard), // sub_1407798c0
    Replacement::draft_throws(0x140779960, "content_140779960", fn_140779960 as *const (), Throws::Caught), // sub_140779960
    Replacement::draft_throws(0x140779a30, "content_140779a30", fn_140779a30 as *const (), Throws::Caught), // sub_140779a30
    Replacement::draft_throws(0x140779b00, "content_140779b00", fn_140779b00 as *const (), Throws::Caught), // sub_140779b00
    Replacement::draft(0x140779bd0, "content_140779bd0", fn_140779bd0 as *const ()), // sub_140779bd0
    Replacement::draft(0x140779c50, "content_140779c50", tpl_7947::<0x14387ce3c, 0x14387ce20, 0x1401e9970, 0x140786ae0, 0x338, 0x118, 0x120, 0x1401e9860> as *const ()), // sub_140779c50
    Replacement::draft(0x140779cc0, "content_140779cc0", fn_140779cc0 as *const ()), // sub_140779cc0
    Replacement::draft_throws(0x140779d40, "content_140779d40", tpl_7949::<0x14387ce3c, 0x14387ce20, 0x1401e9770, 0x140786c50, 0x338, 0x118, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140779d40
    Replacement::draft_throws(0x140779dc0, "content_140779dc0", fn_140779dc0 as *const (), Throws::Discard), // sub_140779dc0
    Replacement::draft(0x140779e90, "content_140779e90", tpl_7849::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0, 0x1432541d0> as *const ()), // sub_140779e90
    Replacement::draft(0x140779f30, "content_140779f30", tpl_66744::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432541d0, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_140779f30
    Replacement::draft(0x14077a060, "content_14077a060", tpl_7850::<0x14387ce60, 0x14243b0b0, 0x140785e90, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077a060
    Replacement::draft_throws(0x14077a110, "content_14077a110", tpl_7951::<0x14387ce80, 0x14387ce60, 0x1401e9fa0, 0x140785f80, 0x328, 0x338, 0x118, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14077a110
    Replacement::draft_throws(0x14077a1a0, "content_14077a1a0", tpl_7852::<0x14387ce60, 0x14243b0b0, 0x140786060, 0x338, 0x118, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14077a1a0
    Replacement::draft_throws(0x14077a220, "content_14077a220", tpl_7952::<0x14387ce80, 0x14387ce60, 0x1401e9b60, 0x140786110, 0x328, 0x338, 0x120, 0x118, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14077a220
    Replacement::draft_throws(0x14077a2c0, "content_14077a2c0", fn_14077a2c0 as *const (), Throws::Caught), // sub_14077a2c0
    Replacement::draft_throws(0x14077a3b0, "content_14077a3b0", fn_14077a3b0 as *const (), Throws::Caught), // sub_14077a3b0
    Replacement::draft_throws(0x14077a4a0, "content_14077a4a0", fn_14077a4a0 as *const (), Throws::Caught), // sub_14077a4a0
    Replacement::draft(0x14077a590, "content_14077a590", fn_14077a590 as *const ()), // sub_14077a590
    Replacement::draft(0x14077a610, "content_14077a610", tpl_7856::<0x14387ce80, 0x14387ce60, 0x1401e9970, 0x140786ae0, 0x338, 0x118, 0xd8, 0x1401e9860> as *const ()), // sub_14077a610
    Replacement::draft(0x14077a680, "content_14077a680", tpl_7857::<0x14387ce60, 0x14243b0b0, 0x140786b90, 0x338, 0x118, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077a680
    Replacement::draft_throws(0x14077a700, "content_14077a700", tpl_7858::<0x14387ce80, 0x14387ce60, 0x1401e9770, 0x140786c50, 0x338, 0x118, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_14077a700
    Replacement::draft_throws(0x14077a780, "content_14077a780", fn_14077a780 as *const (), Throws::Discard), // sub_14077a780
    Replacement::draft(0x14077a870, "content_14077a870", fn_14077a870 as *const ()), // sub_14077a870
    Replacement::draft(0x14077a910, "content_14077a910", fn_14077a910 as *const ()), // sub_14077a910
    Replacement::draft(0x14077a9a0, "content_14077a9a0", fn_14077a9a0 as *const ()), // sub_14077a9a0
    Replacement::draft(0x14077aa50, "content_14077aa50", fn_14077aa50 as *const ()), // sub_14077aa50
    Replacement::draft(0x14077ac00, "content_14077ac00", fn_14077ac00 as *const ()), // sub_14077ac00
    Replacement::draft_throws(0x14077ad60, "content_14077ad60", fn_14077ad60 as *const (), Throws::Discard), // sub_14077ad60
    Replacement::draft_throws(0x14077aeb0, "content_14077aeb0", fn_14077aeb0 as *const (), Throws::Discard), // sub_14077aeb0
    Replacement::draft_throws(0x14077afe0, "content_14077afe0", fn_14077afe0 as *const (), Throws::Discard), // sub_14077afe0
    Replacement::draft_throws(0x14077b140, "content_14077b140", fn_14077b140 as *const (), Throws::Discard), // sub_14077b140
    Replacement::draft_throws(0x14077b260, "content_14077b260", fn_14077b260 as *const (), Throws::Caught), // sub_14077b260
    Replacement::draft_throws(0x14077b3a0, "content_14077b3a0", fn_14077b3a0 as *const (), Throws::Caught), // sub_14077b3a0
    Replacement::draft_throws(0x14077b4e0, "content_14077b4e0", fn_14077b4e0 as *const (), Throws::Caught), // sub_14077b4e0
    Replacement::draft(0x14077b620, "content_14077b620", fn_14077b620 as *const ()), // sub_14077b620
    Replacement::draft(0x14077b770, "content_14077b770", fn_14077b770 as *const ()), // sub_14077b770
    Replacement::draft(0x14077b890, "content_14077b890", fn_14077b890 as *const ()), // sub_14077b890
    Replacement::draft_throws(0x14077b9c0, "content_14077b9c0", fn_14077b9c0 as *const (), Throws::Discard), // sub_14077b9c0
    Replacement::draft_throws(0x14077baf0, "content_14077baf0", fn_14077baf0 as *const (), Throws::Discard), // sub_14077baf0
    Replacement::draft(0x14077bc10, "content_14077bc10", fn_14077bc10 as *const ()), // sub_14077bc10
    Replacement::draft(0x14077bca0, "content_14077bca0", tpl_7849::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0, 0x143252310> as *const ()), // sub_14077bca0
    Replacement::draft(0x14077bd40, "content_14077bd40", tpl_66744::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143252310, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_14077bd40
    Replacement::draft(0x14077be70, "content_14077be70", tpl_7850::<0x14387ceb8, 0x14243b0b0, 0x140785e90, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077be70
    Replacement::draft_throws(0x14077bf20, "content_14077bf20", tpl_7851::<0x14387d010, 0x14387ceb8, 0x1401e9fa0, 0x140785f80, 0x328, 0x338, 0x118, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14077bf20
    Replacement::draft_throws(0x14077bfb0, "content_14077bfb0", tpl_7852::<0x14387ceb8, 0x14243b0b0, 0x140786060, 0x338, 0x118, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14077bfb0
    Replacement::draft_throws(0x14077c030, "content_14077c030", tpl_7853::<0x14387d010, 0x14387ceb8, 0x1401e9b60, 0x140786110, 0x328, 0x338, 0x120, 0x118, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14077c030
    Replacement::draft_throws(0x14077c0d0, "content_14077c0d0", fn_14077c0d0 as *const (), Throws::Caught), // sub_14077c0d0
    Replacement::draft_throws(0x14077c1b0, "content_14077c1b0", fn_14077c1b0 as *const (), Throws::Caught), // sub_14077c1b0
    Replacement::draft_throws(0x14077c290, "content_14077c290", fn_14077c290 as *const (), Throws::Caught), // sub_14077c290
    Replacement::draft(0x14077c370, "content_14077c370", tpl_7855::<0x14387ceb8, 0x14243b0b0, 0x140786a20, 0x338, 0x118, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077c370
    Replacement::draft(0x14077c3f0, "content_14077c3f0", tpl_7856::<0x14387d010, 0x14387ceb8, 0x1401e9970, 0x140786ae0, 0x338, 0x118, 0xd8, 0x1401e9860> as *const ()), // sub_14077c3f0
    Replacement::draft(0x14077c460, "content_14077c460", tpl_7857::<0x14387ceb8, 0x14243b0b0, 0x140786b90, 0x338, 0x118, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077c460
    Replacement::draft_throws(0x14077c4e0, "content_14077c4e0", tpl_7858::<0x14387d010, 0x14387ceb8, 0x1401e9770, 0x140786c50, 0x338, 0x118, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_14077c4e0
    Replacement::draft_throws(0x14077c560, "content_14077c560", tpl_7859::<0x14387ceb8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140786d00, 0x118, 0x1432513c0, 0x1, 0x14020f240> as *const (), Throws::Discard), // sub_14077c560
    Replacement::draft(0x14077c630, "content_14077c630", tpl_7860::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410, 0x1432513c0, 0x1, 0x14020f270> as *const ()), // sub_14077c630
    Replacement::draft(0x14077c6b0, "content_14077c6b0", tpl_7849::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0, 0x143253f50> as *const ()), // sub_14077c6b0
    Replacement::draft(0x14077c750, "content_14077c750", tpl_66744::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143253f50, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_14077c750
    Replacement::draft(0x14077c880, "content_14077c880", fn_14077c880 as *const ()), // sub_14077c880
    Replacement::draft_throws(0x14077c930, "content_14077c930", tpl_7851::<0x14387cf44, 0x14387ce30, 0x1401e9fa0, 0x140785f80, 0x328, 0x338, 0x118, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14077c930
    Replacement::draft_throws(0x14077c9c0, "content_14077c9c0", tpl_7852::<0x14387ce30, 0x14243b0b0, 0x140786060, 0x338, 0x118, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14077c9c0
    Replacement::draft_throws(0x14077ca40, "content_14077ca40", tpl_7972::<0x14387cf44, 0x14387ce30, 0x1401e9b60, 0x140786110, 0x328, 0x338, 0x120, 0x118, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14077ca40
    Replacement::draft_throws(0x14077cae0, "content_14077cae0", fn_14077cae0 as *const (), Throws::Caught), // sub_14077cae0
    Replacement::draft_throws(0x14077cbb0, "content_14077cbb0", fn_14077cbb0 as *const (), Throws::Caught), // sub_14077cbb0
    Replacement::draft_throws(0x14077cc80, "content_14077cc80", fn_14077cc80 as *const (), Throws::Caught), // sub_14077cc80
    Replacement::draft(0x14077cd50, "content_14077cd50", tpl_7855::<0x14387ce30, 0x14243b0b0, 0x140786a20, 0x338, 0x118, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077cd50
    Replacement::draft(0x14077cdd0, "content_14077cdd0", tpl_7856::<0x14387cf44, 0x14387ce30, 0x1401e9970, 0x140786ae0, 0x338, 0x118, 0xe8, 0x1401e9860> as *const ()), // sub_14077cdd0
    Replacement::draft(0x14077ce40, "content_14077ce40", tpl_7857::<0x14387ce30, 0x14243b0b0, 0x140786b90, 0x338, 0x118, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077ce40
    Replacement::draft_throws(0x14077cec0, "content_14077cec0", tpl_7974::<0x14387cf44, 0x14387ce30, 0x1401e9770, 0x140786c50, 0x338, 0x118, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14077cec0
    Replacement::draft_throws(0x14077cf40, "content_14077cf40", tpl_7975::<0x14387ce30, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140786d00, 0x118, 0x1432513a8, 0x1, 0x1402c4e50> as *const (), Throws::Discard), // sub_14077cf40
    Replacement::draft(0x14077d010, "content_14077d010", tpl_7861::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143257550, 0x20> as *const ()), // sub_14077d010
    Replacement::draft(0x14077d0a0, "content_14077d0a0", tpl_7862::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143257550, 0xffffffffffffffec, 0x4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_14077d0a0
    Replacement::draft(0x14077d150, "content_14077d150", tpl_7312::<0x14387cfd0, 0x14243b0b0, 0x1406ea3a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077d150
    Replacement::draft(0x14077d1c0, "content_14077d1c0", fn_14077d1c0 as *const ()), // sub_14077d1c0
    Replacement::draft_throws(0x14077d2b0, "content_14077d2b0", fn_14077d2b0 as *const (), Throws::Discard), // sub_14077d2b0
    Replacement::draft_throws(0x14077d3a0, "content_14077d3a0", fn_14077d3a0 as *const (), Throws::Discard), // sub_14077d3a0
    Replacement::draft_throws(0x14077d440, "content_14077d440", fn_14077d440 as *const (), Throws::Discard), // sub_14077d440
    Replacement::draft_throws(0x14077d540, "content_14077d540", fn_14077d540 as *const (), Throws::Discard), // sub_14077d540
    Replacement::draft_throws(0x14077d600, "content_14077d600", fn_14077d600 as *const (), Throws::Caught), // sub_14077d600
    Replacement::draft_throws(0x14077d870, "content_14077d870", fn_14077d870 as *const (), Throws::Caught), // sub_14077d870
    Replacement::draft_throws(0x14077dab0, "content_14077dab0", fn_14077dab0 as *const (), Throws::Caught), // sub_14077dab0
    Replacement::draft(0x14077dcf0, "content_14077dcf0", fn_14077dcf0 as *const ()), // sub_14077dcf0
    Replacement::draft(0x14077dd90, "content_14077dd90", fn_14077dd90 as *const ()), // sub_14077dd90
    Replacement::draft(0x14077de40, "content_14077de40", fn_14077de40 as *const ()), // sub_14077de40
    Replacement::draft_throws(0x14077def0, "content_14077def0", fn_14077def0 as *const (), Throws::Discard), // sub_14077def0
    Replacement::draft_throws(0x14077dfa0, "content_14077dfa0", fn_14077dfa0 as *const (), Throws::Discard), // sub_14077dfa0
    Replacement::draft(0x14077e0c0, "content_14077e0c0", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143251a50, 0x20> as *const ()), // sub_14077e0c0
    Replacement::draft(0x14077e150, "content_14077e150", tpl_7835::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143251a50, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_14077e150
    Replacement::draft(0x14077e270, "content_14077e270", tpl_7836::<0x14387cf38, 0x14243b0b0, 0x1406ea470, 0x20, 0x1426d4580, 0x338, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077e270
    Replacement::draft_throws(0x14077e320, "content_14077e320", fn_14077e320 as *const (), Throws::Caught), // sub_14077e320
    Replacement::draft_throws(0x14077e3f0, "content_14077e3f0", tpl_7838::<0x14387cf38, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x14240ab10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_14077e3f0
    Replacement::draft_throws(0x14077e4b0, "content_14077e4b0", tpl_7839::<0x14387cf78, 0x14387cf38, 0x1401e9b60, 0x1406ea8d0, 0x1426cdcb0, 0x20, 0x1406a6ae0, 0x1401e9a20> as *const (), Throws::Discard), // sub_14077e4b0
    Replacement::draft_throws(0x14077e530, "content_14077e530", fn_14077e530 as *const (), Throws::Caught), // sub_14077e530
    Replacement::draft_throws(0x14077e730, "content_14077e730", fn_14077e730 as *const (), Throws::Caught), // sub_14077e730
    Replacement::draft_throws(0x14077e900, "content_14077e900", fn_14077e900 as *const (), Throws::Caught), // sub_14077e900
    Replacement::draft(0x14077eac0, "content_14077eac0", tpl_7844::<0x14387cf38, 0x14243b0b0, 0x1406ec310, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_14077eac0
    Replacement::draft(0x14077ec10, "content_14077ec10", tpl_7845::<0x14387cf78, 0x14387cf38, 0x1401e9970, 0x1406ec510, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x1401e9860> as *const ()), // sub_14077ec10
    Replacement::draft(0x14077eca0, "content_14077eca0", tpl_7846::<0x14387cf38, 0x14243b0b0, 0x1406ec600, 0x338, 0x1426cdcb0, 0x20, 0xa0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14077eca0
    Replacement::draft_throws(0x14077ed40, "content_14077ed40", tpl_7847::<0x14387cf78, 0x14387cf38, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14077ed40
    Replacement::draft_throws(0x14077ede0, "content_14077ede0", tpl_7848::<0x14387cf38, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x1432515c8, 0x1406d6fb0> as *const (), Throws::Discard), // sub_14077ede0
    Replacement::draft(0x14077ee90, "content_14077ee90", fn_14077ee90 as *const ()), // sub_14077ee90
    Replacement::draft(0x14077ef30, "content_14077ef30", fn_14077ef30 as *const ()), // sub_14077ef30
    Replacement::draft_throws(0x14077f140, "content_14077f140", fn_14077f140 as *const (), Throws::Caught), // sub_14077f140
    Replacement::draft_throws(0x14077f1e0, "content_14077f1e0", fn_14077f1e0 as *const (), Throws::Caught), // sub_14077f1e0
    Replacement::draft_throws(0x14077f280, "content_14077f280", fn_14077f280 as *const (), Throws::Caught), // sub_14077f280
    Replacement::draft_throws(0x14077f400, "content_14077f400", tpl_7326::<0x14387ce58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140786d00> as *const (), Throws::Discard), // sub_14077f400
    Replacement::draft(0x14077f4a0, "content_14077f4a0", fn_14077f4a0 as *const ()), // sub_14077f4a0
    Replacement::draft(0x14077f5b0, "content_14077f5b0", tpl_66397::<0x142535360, 0x1426456c8, 0x1426453a0, 0x1432538d0, 0x14183acc2> as *const ()), // sub_14077f5b0
    Replacement::draft(0x14077f6e0, "content_14077f6e0", tpl_7468::<0x14387cfb0, 0x14243b0b0, 0x1406ea3a0, 0x14387d230, 0x28, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14077f6e0
    Replacement::draft(0x14077f780, "content_14077f780", fn_14077f780 as *const ()), // sub_14077f780
    Replacement::draft_throws(0x14077f840, "content_14077f840", fn_14077f840 as *const (), Throws::Caught), // sub_14077f840
    Replacement::draft_throws(0x14077f920, "content_14077f920", fn_14077f920 as *const (), Throws::Discard), // sub_14077f920
    Replacement::draft_throws(0x14077f9f0, "content_14077f9f0", fn_14077f9f0 as *const (), Throws::Discard), // sub_14077f9f0
    Replacement::draft_throws(0x14077fa80, "content_14077fa80", fn_14077fa80 as *const (), Throws::Discard), // sub_14077fa80
    Replacement::draft_throws(0x14077fb60, "content_14077fb60", fn_14077fb60 as *const (), Throws::Caught), // sub_14077fb60
    Replacement::draft_throws(0x14077fd90, "content_14077fd90", fn_14077fd90 as *const (), Throws::Caught), // sub_14077fd90
    Replacement::draft_throws(0x14077ff80, "content_14077ff80", fn_14077ff80 as *const (), Throws::Caught), // sub_14077ff80
    Replacement::draft(0x140780170, "content_140780170", fn_140780170 as *const ()), // sub_140780170
    Replacement::draft(0x1407802e0, "content_1407802e0", fn_1407802e0 as *const ()), // sub_1407802e0
    Replacement::draft(0x140780390, "content_140780390", fn_140780390 as *const ()), // sub_140780390
    Replacement::draft_throws(0x140780450, "content_140780450", fn_140780450 as *const (), Throws::Discard), // sub_140780450
    Replacement::draft_throws(0x1407804f0, "content_1407804f0", fn_1407804f0 as *const (), Throws::Discard), // sub_1407804f0
    Replacement::draft(0x1407805b0, "content_1407805b0", tpl_7938::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x143256110> as *const ()), // sub_1407805b0
    Replacement::draft(0x140780640, "content_140780640", tpl_7939::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143256110, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140780640
    Replacement::draft_throws(0x140780830, "content_140780830", fn_140780830 as *const (), Throws::Caught), // sub_140780830
    Replacement::draft_throws(0x1407808d0, "content_1407808d0", fn_1407808d0 as *const (), Throws::Caught), // sub_1407808d0
    Replacement::draft_throws(0x140780970, "content_140780970", fn_140780970 as *const (), Throws::Caught), // sub_140780970
    Replacement::draft_throws(0x140780af0, "content_140780af0", tpl_7326::<0x14387ce50, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0> as *const (), Throws::Discard), // sub_140780af0
    Replacement::draft(0x140780b90, "content_140780b90", tpl_7849::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0, 0x143253650> as *const ()), // sub_140780b90
    Replacement::draft(0x140780c30, "content_140780c30", tpl_66744::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143253650, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_140780c30
    Replacement::draft(0x140780d60, "content_140780d60", tpl_7850::<0x14387ce08, 0x14243b0b0, 0x140785e90, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140780d60
    Replacement::draft_throws(0x140780e10, "content_140780e10", tpl_7851::<0x14387cfc0, 0x14387ce08, 0x1401e9fa0, 0x140785f80, 0x328, 0x338, 0x118, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140780e10
    Replacement::draft_throws(0x140780ea0, "content_140780ea0", tpl_7852::<0x14387ce08, 0x14243b0b0, 0x140786060, 0x338, 0x118, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140780ea0
    Replacement::draft_throws(0x140780f20, "content_140780f20", tpl_7853::<0x14387cfc0, 0x14387ce08, 0x1401e9b60, 0x140786110, 0x328, 0x338, 0x120, 0x118, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140780f20
    Replacement::draft_throws(0x140780fc0, "content_140780fc0", fn_140780fc0 as *const (), Throws::Caught), // sub_140780fc0
    Replacement::draft_throws(0x1407810a0, "content_1407810a0", fn_1407810a0 as *const (), Throws::Caught), // sub_1407810a0
    Replacement::draft_throws(0x140781180, "content_140781180", fn_140781180 as *const (), Throws::Caught), // sub_140781180
    Replacement::draft(0x140781260, "content_140781260", tpl_7855::<0x14387ce08, 0x14243b0b0, 0x140786a20, 0x338, 0x118, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140781260
    Replacement::draft(0x1407812e0, "content_1407812e0", tpl_7856::<0x14387cfc0, 0x14387ce08, 0x1401e9970, 0x140786ae0, 0x338, 0x118, 0xd8, 0x1401e9860> as *const ()), // sub_1407812e0
    Replacement::draft(0x140781350, "content_140781350", tpl_7857::<0x14387ce08, 0x14243b0b0, 0x140786b90, 0x338, 0x118, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140781350
    Replacement::draft_throws(0x1407813d0, "content_1407813d0", tpl_7858::<0x14387cfc0, 0x14387ce08, 0x1401e9770, 0x140786c50, 0x338, 0x118, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_1407813d0
    Replacement::draft_throws(0x140781450, "content_140781450", fn_140781450 as *const (), Throws::Discard), // sub_140781450
    Replacement::draft(0x140781530, "content_140781530", fn_140781530 as *const ()), // sub_140781530
    Replacement::draft(0x1407815c0, "content_1407815c0", fn_1407815c0 as *const ()), // sub_1407815c0
    Replacement::draft(0x140781660, "content_140781660", fn_140781660 as *const ()), // sub_140781660
    Replacement::draft(0x140781730, "content_140781730", tpl_7312::<0x14387ce78, 0x14243b0b0, 0x140785da0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140781730
    Replacement::draft(0x1407817a0, "content_1407817a0", tpl_7404::<0x14387ce78, 0x14243b0b0, 0x140785e90, 0x338, 0x11c, 0x1426d4580, 0x1, 0x118, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407817a0
    Replacement::draft_throws(0x140781880, "content_140781880", fn_140781880 as *const (), Throws::Discard), // sub_140781880
    Replacement::draft_throws(0x140781940, "content_140781940", fn_140781940 as *const (), Throws::Discard), // sub_140781940
    Replacement::draft_throws(0x1407819d0, "content_1407819d0", fn_1407819d0 as *const (), Throws::Discard), // sub_1407819d0
    Replacement::draft_throws(0x140781ab0, "content_140781ab0", tpl_8010::<0x8, 0x140786200, 0x11c, 0x118> as *const (), Throws::Discard), // sub_140781ab0
    Replacement::draft_throws(0x140781b10, "content_140781b10", fn_140781b10 as *const (), Throws::Caught), // sub_140781b10
    Replacement::draft_throws(0x140781d60, "content_140781d60", fn_140781d60 as *const (), Throws::Caught), // sub_140781d60
    Replacement::draft_throws(0x140781f80, "content_140781f80", fn_140781f80 as *const (), Throws::Caught), // sub_140781f80
    Replacement::draft(0x140782190, "content_140782190", fn_140782190 as *const ()), // sub_140782190
    Replacement::draft(0x140782230, "content_140782230", tpl_7413::<0x14387cee4, 0x14387ce78, 0x1401e9970, 0x140786ae0, 0x338, 0x11c, 0xd8, 0x118, 0xe8, 0x1401e9860> as *const ()), // sub_140782230
    Replacement::draft(0x1407822d0, "content_1407822d0", tpl_7414::<0x14387ce78, 0x14243b0b0, 0x140786b90, 0x338, 0x11c, 0xd8, 0x118, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407822d0
    Replacement::draft_throws(0x140782380, "content_140782380", tpl_7415::<0x14387cee4, 0x14387ce78, 0x1401e9770, 0x140786c50, 0x338, 0x11c, 0x120, 0x4, 0x118, 0x1401e9590> as *const (), Throws::Discard), // sub_140782380
    Replacement::draft_throws(0x140782420, "content_140782420", fn_140782420 as *const (), Throws::Discard), // sub_140782420
    Replacement::draft(0x140782520, "content_140782520", tpl_66876::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410, 0x14384e3b0, 0x118, 0x1426cdcb0, 0x143251310> as *const ()), // sub_140782520
    Replacement::draft(0x1407825c0, "content_1407825c0", fn_1407825c0 as *const ()), // sub_1407825c0
    Replacement::draft(0x140782690, "content_140782690", tpl_66876::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410, 0x14384e3b0, 0x118, 0x1426cdcb0, 0x143251310> as *const ()), // sub_140782690
    Replacement::draft(0x140782730, "content_140782730", fn_140782730 as *const ()), // sub_140782730
    Replacement::draft(0x1407827d0, "content_1407827d0", fn_1407827d0 as *const ()), // sub_1407827d0
    Replacement::draft(0x140782900, "content_140782900", tpl_8019::<0x14387d018, 0x14243b0b0, 0x140794b40, 0x338, 0x128, 0x1426d4580, 0x1, 0x12c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140782900
    Replacement::draft_throws(0x1407829e0, "content_1407829e0", fn_1407829e0 as *const (), Throws::Discard), // sub_1407829e0
    Replacement::draft_throws(0x140782aa0, "content_140782aa0", tpl_8021::<0x14387d018, 0x14243b0b0, 0x140795000, 0x338, 0x128, 0x38, 0x12c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140782aa0
    Replacement::draft_throws(0x140782b30, "content_140782b30", fn_140782b30 as *const (), Throws::Discard), // sub_140782b30
    Replacement::draft_throws(0x140782c10, "content_140782c10", fn_140782c10 as *const (), Throws::Discard), // sub_140782c10
    Replacement::draft_throws(0x140782c80, "content_140782c80", fn_140782c80 as *const (), Throws::Caught), // sub_140782c80
    Replacement::draft_throws(0x140782dc0, "content_140782dc0", fn_140782dc0 as *const (), Throws::Caught), // sub_140782dc0
    Replacement::draft_throws(0x140782f00, "content_140782f00", fn_140782f00 as *const (), Throws::Caught), // sub_140782f00
    Replacement::draft(0x140783040, "content_140783040", fn_140783040 as *const ()), // sub_140783040
    Replacement::draft(0x1407830c0, "content_1407830c0", tpl_8026::<0x14387cf98, 0x14387d018, 0x1401e9970, 0x1407960d0, 0x338, 0x128, 0xd8, 0x12c, 0x1401e9860> as *const ()), // sub_1407830c0
    Replacement::draft(0x140783150, "content_140783150", tpl_8027::<0x14387d018, 0x14243b0b0, 0x140796270, 0x338, 0x128, 0xd8, 0x12c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140783150
    Replacement::draft_throws(0x1407831f0, "content_1407831f0", tpl_8028::<0x14387cf98, 0x14387d018, 0x1401e9770, 0x140796420, 0x338, 0x128, 0x120, 0x4, 0x12c, 0x1401e9590> as *const (), Throws::Discard), // sub_1407831f0
    Replacement::draft_throws(0x140783290, "content_140783290", fn_140783290 as *const (), Throws::Discard), // sub_140783290
    Replacement::draft(0x1407833c0, "content_1407833c0", fn_1407833c0 as *const ()), // sub_1407833c0
    Replacement::draft(0x140783470, "content_140783470", fn_140783470 as *const ()), // sub_140783470
    Replacement::draft(0x140783510, "content_140783510", fn_140783510 as *const ()), // sub_140783510
    Replacement::draft(0x1407835d0, "content_1407835d0", fn_1407835d0 as *const ()), // sub_1407835d0
    Replacement::draft(0x140783720, "content_140783720", fn_140783720 as *const ()), // sub_140783720
    Replacement::draft_throws(0x140783940, "content_140783940", fn_140783940 as *const (), Throws::Discard), // sub_140783940
    Replacement::draft_throws(0x140783b40, "content_140783b40", fn_140783b40 as *const (), Throws::Discard), // sub_140783b40
    Replacement::draft_throws(0x140783ce0, "content_140783ce0", fn_140783ce0 as *const (), Throws::Discard), // sub_140783ce0
    Replacement::draft_throws(0x140783f10, "content_140783f10", fn_140783f10 as *const (), Throws::Discard), // sub_140783f10
    Replacement::draft_throws(0x140783fb0, "content_140783fb0", fn_140783fb0 as *const (), Throws::Caught), // sub_140783fb0
    Replacement::draft_throws(0x140784160, "content_140784160", fn_140784160 as *const (), Throws::Caught), // sub_140784160
    Replacement::draft_throws(0x140784310, "content_140784310", fn_140784310 as *const (), Throws::Caught), // sub_140784310
    Replacement::draft(0x1407844c0, "content_1407844c0", fn_1407844c0 as *const ()), // sub_1407844c0
    Replacement::draft(0x140784650, "content_140784650", fn_140784650 as *const ()), // sub_140784650
    Replacement::draft(0x1407847f0, "content_1407847f0", fn_1407847f0 as *const ()), // sub_1407847f0
    Replacement::draft_throws(0x140784990, "content_140784990", fn_140784990 as *const (), Throws::Discard), // sub_140784990
    Replacement::draft_throws(0x140784b50, "content_140784b50", fn_140784b50 as *const (), Throws::Discard), // sub_140784b50
    Replacement::draft(0x140784cf0, "content_140784cf0", fn_140784cf0 as *const ()), // sub_140784cf0
    Replacement::draft(0x140784dc0, "content_140784dc0", fn_140784dc0 as *const ()), // sub_140784dc0
    Replacement::draft(0x140784ec0, "content_140784ec0", fn_140784ec0 as *const ()), // sub_140784ec0
    Replacement::draft(0x140784ff0, "content_140784ff0", tpl_7452::<0x14387ce48, 0x14243b0b0, 0x1406ea3a0, 0x14387d230, 0x20, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140784ff0
    Replacement::draft(0x140785090, "content_140785090", fn_140785090 as *const ()), // sub_140785090
    Replacement::draft_throws(0x140785170, "content_140785170", fn_140785170 as *const (), Throws::Discard), // sub_140785170
    Replacement::draft_throws(0x140785250, "content_140785250", fn_140785250 as *const (), Throws::Discard), // sub_140785250
    Replacement::draft_throws(0x140785300, "content_140785300", fn_140785300 as *const (), Throws::Discard), // sub_140785300
    Replacement::draft_throws(0x1407853f0, "content_1407853f0", fn_1407853f0 as *const (), Throws::Discard), // sub_1407853f0
    Replacement::draft_throws(0x140785500, "content_140785500", fn_140785500 as *const (), Throws::Caught), // sub_140785500
    Replacement::draft_throws(0x140785630, "content_140785630", fn_140785630 as *const (), Throws::Caught), // sub_140785630
    Replacement::draft_throws(0x140785760, "content_140785760", fn_140785760 as *const (), Throws::Caught), // sub_140785760
    Replacement::draft(0x140785890, "content_140785890", fn_140785890 as *const ()), // sub_140785890
    Replacement::draft(0x140785940, "content_140785940", fn_140785940 as *const ()), // sub_140785940
    Replacement::draft(0x1407859e0, "content_1407859e0", fn_1407859e0 as *const ()), // sub_1407859e0
    Replacement::draft_throws(0x140785a90, "content_140785a90", fn_140785a90 as *const (), Throws::Discard), // sub_140785a90
    Replacement::draft_throws(0x140785b40, "content_140785b40", fn_140785b40 as *const (), Throws::Discard), // sub_140785b40
    Replacement::draft(0x140785c60, "content_140785c60", tpl_7909::<0x120, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_140785c60
    Replacement::draft(0x140785cf0, "content_140785cf0", tpl_66781::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x14079b2c0> as *const ()), // sub_140785cf0
    Replacement::draft(0x140785da0, "content_140785da0", fn_140785da0 as *const ()), // sub_140785da0
    Replacement::draft(0x140785e90, "content_140785e90", fn_140785e90 as *const ()), // sub_140785e90
    Replacement::draft_throws(0x140785f80, "content_140785f80", fn_140785f80 as *const (), Throws::Discard), // sub_140785f80
    Replacement::draft_throws(0x140786060, "content_140786060", fn_140786060 as *const (), Throws::Discard), // sub_140786060
    Replacement::draft_throws(0x140786110, "content_140786110", fn_140786110 as *const (), Throws::Discard), // sub_140786110
    Replacement::draft_throws(0x140786200, "content_140786200", fn_140786200 as *const (), Throws::Discard), // sub_140786200
    Replacement::draft_throws(0x140786330, "content_140786330", fn_140786330 as *const (), Throws::Caught), // sub_140786330
    Replacement::draft_throws(0x1407865a0, "content_1407865a0", fn_1407865a0 as *const (), Throws::Caught), // sub_1407865a0
    Replacement::draft_throws(0x1407867e0, "content_1407867e0", fn_1407867e0 as *const (), Throws::Caught), // sub_1407867e0
    Replacement::draft(0x140786a20, "content_140786a20", fn_140786a20 as *const ()), // sub_140786a20
    Replacement::draft(0x140786ae0, "content_140786ae0", fn_140786ae0 as *const ()), // sub_140786ae0
    Replacement::draft(0x140786b90, "content_140786b90", fn_140786b90 as *const ()), // sub_140786b90
    Replacement::draft_throws(0x140786c50, "content_140786c50", fn_140786c50 as *const (), Throws::Discard), // sub_140786c50
    Replacement::draft_throws(0x140786d00, "content_140786d00", fn_140786d00 as *const (), Throws::Discard), // sub_140786d00
    Replacement::draft(0x140786e20, "content_140786e20", tpl_7861::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143258010, 0x20> as *const ()), // sub_140786e20
    Replacement::draft(0x140786eb0, "content_140786eb0", tpl_7862::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143258010, 0xffffffffffffffec, 0x4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140786eb0
    Replacement::draft(0x140786fd0, "content_140786fd0", tpl_8069::<0x14387cfa0, 0x14243b0b0, 0x1406ea470, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140786fd0
    Replacement::draft_throws(0x140787080, "content_140787080", tpl_7851::<0x14387cee0, 0x14387cfa0, 0x1401e9fa0, 0x1406ea600, 0x328, 0x338, 0x20, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140787080
    Replacement::draft_throws(0x140787110, "content_140787110", tpl_7852::<0x14387cfa0, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140787110
    Replacement::draft_throws(0x140787190, "content_140787190", tpl_7972::<0x14387cee0, 0x14387cfa0, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x20, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140787190
    Replacement::draft_throws(0x140787220, "content_140787220", fn_140787220 as *const (), Throws::Discard), // sub_140787220
    Replacement::draft_throws(0x1407872c0, "content_1407872c0", fn_1407872c0 as *const (), Throws::Caught), // sub_1407872c0
    Replacement::draft_throws(0x1407874c0, "content_1407874c0", fn_1407874c0 as *const (), Throws::Caught), // sub_1407874c0
    Replacement::draft_throws(0x140787680, "content_140787680", fn_140787680 as *const (), Throws::Caught), // sub_140787680
    Replacement::draft(0x140787840, "content_140787840", tpl_7855::<0x14387cfa0, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140787840
    Replacement::draft(0x1407878c0, "content_1407878c0", tpl_8074::<0x14387cee0, 0x14387cfa0, 0x1401e9970, 0x1406ec510, 0x20, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_1407878c0
    Replacement::draft(0x140787940, "content_140787940", tpl_8075::<0x14387cfa0, 0x14243b0b0, 0x1406ec600, 0x20, 0x338, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140787940
    Replacement::draft_throws(0x1407879d0, "content_1407879d0", tpl_7974::<0x14387cee0, 0x14387cfa0, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1407879d0
    Replacement::draft_throws(0x140787a50, "content_140787a50", tpl_8076::<0x14387cfa0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x1432514f8> as *const (), Throws::Discard), // sub_140787a50
    Replacement::draft(0x140787b10, "content_140787b10", tpl_7496::<0x1406ec8f0, 0x14384e490, 0x20, 0x1426cdcb0, 0x1432514f8, 0x1401de410> as *const ()), // sub_140787b10
    Replacement::draft(0x140787b80, "content_140787b80", tpl_7496::<0x1406eca20, 0x14384e490, 0x20, 0x1426cdcb0, 0x1432514f8, 0x1401de410> as *const ()), // sub_140787b80
    Replacement::draft(0x140787bf0, "content_140787bf0", fn_140787bf0 as *const ()), // sub_140787bf0
    Replacement::draft(0x140787c80, "content_140787c80", fn_140787c80 as *const ()), // sub_140787c80
    Replacement::draft(0x140787d30, "content_140787d30", fn_140787d30 as *const ()), // sub_140787d30
    Replacement::draft(0x140787e60, "content_140787e60", fn_140787e60 as *const ()), // sub_140787e60
    Replacement::draft_throws(0x140787fb0, "content_140787fb0", fn_140787fb0 as *const (), Throws::Discard), // sub_140787fb0
    Replacement::draft_throws(0x1407880e0, "content_1407880e0", fn_1407880e0 as *const (), Throws::Discard), // sub_1407880e0
    Replacement::draft_throws(0x1407881b0, "content_1407881b0", fn_1407881b0 as *const (), Throws::Discard), // sub_1407881b0
    Replacement::draft_throws(0x140788300, "content_140788300", fn_140788300 as *const (), Throws::Discard), // sub_140788300
    Replacement::draft_throws(0x140788490, "content_140788490", fn_140788490 as *const (), Throws::Caught), // sub_140788490
    Replacement::draft_throws(0x140788890, "content_140788890", fn_140788890 as *const (), Throws::Caught), // sub_140788890
    Replacement::draft_throws(0x140788c00, "content_140788c00", fn_140788c00 as *const (), Throws::Caught), // sub_140788c00
    Replacement::draft(0x140788f70, "content_140788f70", fn_140788f70 as *const ()), // sub_140788f70
    Replacement::draft(0x140789070, "content_140789070", fn_140789070 as *const ()), // sub_140789070
    Replacement::draft(0x140789150, "content_140789150", fn_140789150 as *const ()), // sub_140789150
    Replacement::draft_throws(0x140789240, "content_140789240", fn_140789240 as *const (), Throws::Discard), // sub_140789240
    Replacement::draft_throws(0x140789320, "content_140789320", fn_140789320 as *const (), Throws::Discard), // sub_140789320
    Replacement::draft(0x140789480, "content_140789480", tpl_8092::<0x1406ec8f0, 0x1426cdcb0, 0x14384e410, 0x20, 0x143251798, 0x1, 0x1401de410, 0x14384e438, 0x118, 0x1432514e8> as *const ()), // sub_140789480
    Replacement::draft(0x140789510, "content_140789510", fn_140789510 as *const ()), // sub_140789510
    Replacement::draft(0x1407895a0, "content_1407895a0", tpl_8092::<0x1406eca20, 0x1426cdcb0, 0x14384e410, 0x20, 0x143251798, 0x1, 0x1401de410, 0x14384e438, 0x118, 0x1432514e8> as *const ()), // sub_1407895a0
    Replacement::draft(0x140789630, "content_140789630", fn_140789630 as *const ()), // sub_140789630
    Replacement::draft(0x1407896d0, "content_1407896d0", fn_1407896d0 as *const ()), // sub_1407896d0
    Replacement::draft(0x140789800, "content_140789800", tpl_8096::<0x14387cf00, 0x14243b0b0, 0x1406ea470, 0x20, 0x1426d4580, 0x338, 0x21, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140789800
    Replacement::draft_throws(0x1407898b0, "content_1407898b0", fn_1407898b0 as *const (), Throws::Discard), // sub_1407898b0
    Replacement::draft_throws(0x140789970, "content_140789970", tpl_8021::<0x14387cf00, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x80, 0x21, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140789970
    Replacement::draft_throws(0x140789a00, "content_140789a00", fn_140789a00 as *const (), Throws::Discard), // sub_140789a00
    Replacement::draft_throws(0x140789ad0, "content_140789ad0", fn_140789ad0 as *const (), Throws::Discard), // sub_140789ad0
    Replacement::draft_throws(0x140789b90, "content_140789b90", fn_140789b90 as *const (), Throws::Caught), // sub_140789b90
    Replacement::draft_throws(0x140789cb0, "content_140789cb0", fn_140789cb0 as *const (), Throws::Caught), // sub_140789cb0
    Replacement::draft_throws(0x140789dd0, "content_140789dd0", fn_140789dd0 as *const (), Throws::Caught), // sub_140789dd0
    Replacement::draft(0x140789ef0, "content_140789ef0", fn_140789ef0 as *const ()), // sub_140789ef0
    Replacement::draft(0x140789f60, "content_140789f60", tpl_8026::<0x14387ce68, 0x14387cf00, 0x1401e9970, 0x1406ec510, 0x338, 0x20, 0x120, 0x21, 0x1401e9860> as *const ()), // sub_140789f60
    Replacement::draft(0x140789ff0, "content_140789ff0", tpl_8027::<0x14387cf00, 0x14243b0b0, 0x1406ec600, 0x338, 0x20, 0x120, 0x21, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140789ff0
    Replacement::draft_throws(0x14078a080, "content_14078a080", fn_14078a080 as *const (), Throws::Discard), // sub_14078a080
    Replacement::draft_throws(0x14078a110, "content_14078a110", fn_14078a110 as *const (), Throws::Discard), // sub_14078a110
    Replacement::draft(0x14078a220, "content_14078a220", tpl_8104::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143254a50, 0x28, 0x20> as *const ()), // sub_14078a220
    Replacement::draft(0x14078a2c0, "content_14078a2c0", tpl_8105::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143254a50, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_14078a2c0
    Replacement::draft(0x14078a3e0, "content_14078a3e0", fn_14078a3e0 as *const ()), // sub_14078a3e0
    Replacement::draft_throws(0x14078a4d0, "content_14078a4d0", fn_14078a4d0 as *const (), Throws::Caught), // sub_14078a4d0
    Replacement::draft_throws(0x14078a5d0, "content_14078a5d0", fn_14078a5d0 as *const (), Throws::Discard), // sub_14078a5d0
    Replacement::draft_throws(0x14078a6a0, "content_14078a6a0", fn_14078a6a0 as *const (), Throws::Discard), // sub_14078a6a0
    Replacement::draft_throws(0x14078a750, "content_14078a750", fn_14078a750 as *const (), Throws::Discard), // sub_14078a750
    Replacement::draft_throws(0x14078a800, "content_14078a800", fn_14078a800 as *const (), Throws::Caught), // sub_14078a800
    Replacement::draft_throws(0x14078ab40, "content_14078ab40", fn_14078ab40 as *const (), Throws::Caught), // sub_14078ab40
    Replacement::draft_throws(0x14078ae20, "content_14078ae20", fn_14078ae20 as *const (), Throws::Caught), // sub_14078ae20
    Replacement::draft(0x14078b100, "content_14078b100", fn_14078b100 as *const ()), // sub_14078b100
    Replacement::draft(0x14078b260, "content_14078b260", fn_14078b260 as *const ()), // sub_14078b260
    Replacement::draft(0x14078b310, "content_14078b310", fn_14078b310 as *const ()), // sub_14078b310
    Replacement::draft_throws(0x14078b3d0, "content_14078b3d0", fn_14078b3d0 as *const (), Throws::Discard), // sub_14078b3d0
    Replacement::draft_throws(0x14078b480, "content_14078b480", fn_14078b480 as *const (), Throws::Discard), // sub_14078b480
    Replacement::draft(0x14078b560, "content_14078b560", tpl_7496::<0x1406ec8f0, 0x14384e410, 0x28, 0x1426cdcb0, 0x143251450, 0x1401de410> as *const ()), // sub_14078b560
    Replacement::draft(0x14078b5d0, "content_14078b5d0", tpl_7496::<0x1406eca20, 0x14384e410, 0x28, 0x1426cdcb0, 0x143251450, 0x1401de410> as *const ()), // sub_14078b5d0
    Replacement::draft(0x14078b640, "content_14078b640", fn_14078b640 as *const ()), // sub_14078b640
    Replacement::draft(0x14078b700, "content_14078b700", fn_14078b700 as *const ()), // sub_14078b700
    Replacement::draft(0x14078b840, "content_14078b840", tpl_7896::<0x14387cf80, 0x14243b0b0, 0x1406ea470, 0x338, 0x20, 0x1426d4580, 0x1, 0x2c, 0x27, 0x28, 0x24, 0x1401ee7c0> as *const ()), // sub_14078b840
    Replacement::draft_throws(0x14078b940, "content_14078b940", tpl_7897::<0x14387cfc8, 0x14387cf80, 0x1401e9fa0, 0x1406ea600, 0x328, 0x20, 0x338, 0x48, 0x369, 0x24, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14078b940
    Replacement::draft_throws(0x14078ba20, "content_14078ba20", tpl_7898::<0x14387cf80, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14078ba20
    Replacement::draft_throws(0x14078bab0, "content_14078bab0", tpl_7899::<0x14387cfc8, 0x14387cf80, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x369, 0x20, 0x24, 0x1424b4da0, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14078bab0
    Replacement::draft_throws(0x14078bb90, "content_14078bb90", fn_14078bb90 as *const (), Throws::Caught), // sub_14078bb90
    Replacement::draft_throws(0x14078bf30, "content_14078bf30", fn_14078bf30 as *const (), Throws::Caught), // sub_14078bf30
    Replacement::draft_throws(0x14078c270, "content_14078c270", fn_14078c270 as *const (), Throws::Caught), // sub_14078c270
    Replacement::draft(0x14078c5a0, "content_14078c5a0", tpl_7904::<0x14387cf80, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x24, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078c5a0
    Replacement::draft(0x14078c650, "content_14078c650", tpl_7905::<0x14387cfc8, 0x14387cf80, 0x1401e9970, 0x1406ec510, 0x20, 0x338, 0xe8, 0x24, 0xc0, 0x1401e9860> as *const ()), // sub_14078c650
    Replacement::draft(0x14078c6e0, "content_14078c6e0", tpl_7906::<0x14387cf80, 0x14243b0b0, 0x1406ec600, 0x20, 0x338, 0xe8, 0x24, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078c6e0
    Replacement::draft_throws(0x14078c780, "content_14078c780", tpl_66780::<0x14387cfc8, 0x14387cf80, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x24, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_14078c780
    Replacement::draft_throws(0x14078c810, "content_14078c810", tpl_7908::<0x14387cf80, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x143251798, 0x1426cdcb0, 0x24, 0x143251438, 0x1402c4730> as *const (), Throws::Discard), // sub_14078c810
    Replacement::draft(0x14078c900, "content_14078c900", tpl_8124::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x143254d90> as *const ()), // sub_14078c900
    Replacement::draft(0x14078ca10, "content_14078ca10", tpl_66973::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143254d90, 0xffffffffffffff88, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_14078ca10
    Replacement::draft(0x14078cbb0, "content_14078cbb0", tpl_8125::<0x14387cf88, 0x14243b0b0, 0x140790080, 0xa0, 0x338, 0x1426d4580, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078cbb0
    Replacement::draft_throws(0x14078cc50, "content_14078cc50", tpl_8126::<0x14387cff8, 0x14387cf88, 0x1401e9fa0, 0x140790140, 0x328, 0x338, 0xa0, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14078cc50
    Replacement::draft_throws(0x14078cce0, "content_14078cce0", tpl_7852::<0x14387cf88, 0x14243b0b0, 0x1407901e0, 0x338, 0xa0, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14078cce0
    Replacement::draft_throws(0x14078cd60, "content_14078cd60", tpl_8127::<0x14387cff8, 0x14387cf88, 0x1401e9b60, 0x140790270, 0x328, 0x338, 0x120, 0xa0, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14078cd60
    Replacement::draft_throws(0x14078ce00, "content_14078ce00", fn_14078ce00 as *const (), Throws::Caught), // sub_14078ce00
    Replacement::draft_throws(0x14078ced0, "content_14078ced0", fn_14078ced0 as *const (), Throws::Caught), // sub_14078ced0
    Replacement::draft_throws(0x14078cfa0, "content_14078cfa0", fn_14078cfa0 as *const (), Throws::Caught), // sub_14078cfa0
    Replacement::draft(0x14078d070, "content_14078d070", tpl_8129::<0x14387cf88, 0x14243b0b0, 0x140790a10, 0x338, 0xa0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078d070
    Replacement::draft(0x14078d0f0, "content_14078d0f0", tpl_7856::<0x14387cff8, 0x14387cf88, 0x1401e9970, 0x140790ab0, 0x338, 0xa0, 0x120, 0x1401e9860> as *const ()), // sub_14078d0f0
    Replacement::draft(0x14078d160, "content_14078d160", tpl_7857::<0x14387cf88, 0x14243b0b0, 0x140790b40, 0x338, 0xa0, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078d160
    Replacement::draft_throws(0x14078d1e0, "content_14078d1e0", tpl_8130::<0x14387cff8, 0x14387cf88, 0x1401e9770, 0x140790be0, 0x338, 0xa0, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14078d1e0
    Replacement::draft_throws(0x14078d260, "content_14078d260", tpl_8131::<0x14387cf88, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140790c60, 0xa0, 0x143251138, 0x1, 0x14020f0f0> as *const (), Throws::Discard), // sub_14078d260
    Replacement::draft(0x14078d330, "content_14078d330", tpl_8124::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x1432529d0> as *const ()), // sub_14078d330
    Replacement::draft(0x14078d440, "content_14078d440", tpl_66973::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432529d0, 0xffffffffffffff88, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_14078d440
    Replacement::draft(0x14078d5e0, "content_14078d5e0", fn_14078d5e0 as *const ()), // sub_14078d5e0
    Replacement::draft_throws(0x14078d690, "content_14078d690", tpl_7851::<0x14387cf70, 0x14387cdf0, 0x1401e9fa0, 0x140790140, 0x328, 0x338, 0xa0, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14078d690
    Replacement::draft_throws(0x14078d720, "content_14078d720", tpl_7852::<0x14387cdf0, 0x14243b0b0, 0x1407901e0, 0x338, 0xa0, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14078d720
    Replacement::draft_throws(0x14078d7a0, "content_14078d7a0", tpl_7853::<0x14387cf70, 0x14387cdf0, 0x1401e9b60, 0x140790270, 0x328, 0x338, 0x120, 0xa0, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14078d7a0
    Replacement::draft_throws(0x14078d840, "content_14078d840", fn_14078d840 as *const (), Throws::Caught), // sub_14078d840
    Replacement::draft_throws(0x14078d920, "content_14078d920", fn_14078d920 as *const (), Throws::Caught), // sub_14078d920
    Replacement::draft_throws(0x14078da00, "content_14078da00", fn_14078da00 as *const (), Throws::Caught), // sub_14078da00
    Replacement::draft(0x14078dae0, "content_14078dae0", tpl_7855::<0x14387cdf0, 0x14243b0b0, 0x140790a10, 0x338, 0xa0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078dae0
    Replacement::draft(0x14078db60, "content_14078db60", tpl_7856::<0x14387cf70, 0x14387cdf0, 0x1401e9970, 0x140790ab0, 0x338, 0xa0, 0xd8, 0x1401e9860> as *const ()), // sub_14078db60
    Replacement::draft(0x14078dbd0, "content_14078dbd0", tpl_7857::<0x14387cdf0, 0x14243b0b0, 0x140790b40, 0x338, 0xa0, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078dbd0
    Replacement::draft_throws(0x14078dc50, "content_14078dc50", tpl_7858::<0x14387cf70, 0x14387cdf0, 0x1401e9770, 0x140790be0, 0x338, 0xa0, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_14078dc50
    Replacement::draft_throws(0x14078dcd0, "content_14078dcd0", tpl_7859::<0x14387cdf0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140790c60, 0xa0, 0x143251128, 0x1, 0x14020f240> as *const (), Throws::Discard), // sub_14078dcd0
    Replacement::draft(0x14078dda0, "content_14078dda0", fn_14078dda0 as *const ()), // sub_14078dda0
    Replacement::draft(0x14078de20, "content_14078de20", tpl_8124::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x143257d90> as *const ()), // sub_14078de20
    Replacement::draft(0x14078df30, "content_14078df30", tpl_66973::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143257d90, 0xffffffffffffff88, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_14078df30
    Replacement::draft(0x14078e0d0, "content_14078e0d0", fn_14078e0d0 as *const ()), // sub_14078e0d0
    Replacement::draft_throws(0x14078e180, "content_14078e180", tpl_7851::<0x14387cfa8, 0x14387cdf8, 0x1401e9fa0, 0x140790140, 0x328, 0x338, 0xa0, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14078e180
    Replacement::draft_throws(0x14078e210, "content_14078e210", tpl_7852::<0x14387cdf8, 0x14243b0b0, 0x1407901e0, 0x338, 0xa0, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_14078e210
    Replacement::draft_throws(0x14078e290, "content_14078e290", tpl_7972::<0x14387cfa8, 0x14387cdf8, 0x1401e9b60, 0x140790270, 0x328, 0x338, 0x120, 0xa0, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14078e290
    Replacement::draft_throws(0x14078e330, "content_14078e330", fn_14078e330 as *const (), Throws::Caught), // sub_14078e330
    Replacement::draft_throws(0x14078e400, "content_14078e400", fn_14078e400 as *const (), Throws::Caught), // sub_14078e400
    Replacement::draft_throws(0x14078e4d0, "content_14078e4d0", fn_14078e4d0 as *const (), Throws::Caught), // sub_14078e4d0
    Replacement::draft(0x14078e5a0, "content_14078e5a0", tpl_7855::<0x14387cdf8, 0x14243b0b0, 0x140790a10, 0x338, 0xa0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078e5a0
    Replacement::draft(0x14078e620, "content_14078e620", tpl_7856::<0x14387cfa8, 0x14387cdf8, 0x1401e9970, 0x140790ab0, 0x338, 0xa0, 0xe8, 0x1401e9860> as *const ()), // sub_14078e620
    Replacement::draft(0x14078e690, "content_14078e690", tpl_7857::<0x14387cdf8, 0x14243b0b0, 0x140790b40, 0x338, 0xa0, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14078e690
    Replacement::draft_throws(0x14078e710, "content_14078e710", tpl_7974::<0x14387cfa8, 0x14387cdf8, 0x1401e9770, 0x140790be0, 0x338, 0xa0, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14078e710
    Replacement::draft_throws(0x14078e790, "content_14078e790", tpl_7975::<0x14387cdf8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140790c60, 0xa0, 0x143251120, 0x1, 0x1402c4e50> as *const (), Throws::Discard), // sub_14078e790
    Replacement::draft(0x14078e860, "content_14078e860", tpl_8104::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143251c90, 0x28, 0x20> as *const ()), // sub_14078e860
    Replacement::draft(0x14078e900, "content_14078e900", tpl_8105::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143251c90, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_14078e900
    Replacement::draft(0x14078ea20, "content_14078ea20", fn_14078ea20 as *const ()), // sub_14078ea20
    Replacement::draft_throws(0x14078eb20, "content_14078eb20", fn_14078eb20 as *const (), Throws::Discard), // sub_14078eb20
    Replacement::draft_throws(0x14078ec10, "content_14078ec10", tpl_8136::<0x14387cf18, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x24, 0x28, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_14078ec10
    Replacement::draft_throws(0x14078ecb0, "content_14078ecb0", fn_14078ecb0 as *const (), Throws::Discard), // sub_14078ecb0
    Replacement::draft_throws(0x14078eda0, "content_14078eda0", fn_14078eda0 as *const (), Throws::Discard), // sub_14078eda0
    Replacement::draft_throws(0x14078ee60, "content_14078ee60", fn_14078ee60 as *const (), Throws::Caught), // sub_14078ee60
    Replacement::draft_throws(0x14078f200, "content_14078f200", fn_14078f200 as *const (), Throws::Caught), // sub_14078f200
    Replacement::draft_throws(0x14078f520, "content_14078f520", fn_14078f520 as *const (), Throws::Caught), // sub_14078f520
    Replacement::draft(0x14078f830, "content_14078f830", fn_14078f830 as *const ()), // sub_14078f830
    Replacement::draft(0x14078f8d0, "content_14078f8d0", fn_14078f8d0 as *const ()), // sub_14078f8d0
    Replacement::draft(0x14078f980, "content_14078f980", fn_14078f980 as *const ()), // sub_14078f980
    Replacement::draft_throws(0x14078fa40, "content_14078fa40", fn_14078fa40 as *const (), Throws::Discard), // sub_14078fa40
    Replacement::draft_throws(0x14078faf0, "content_14078faf0", fn_14078faf0 as *const (), Throws::Discard), // sub_14078faf0
    Replacement::draft(0x14078fc00, "content_14078fc00", tpl_66989::<0x1406ec8f0, 0x14384e410, 0x20, 0x143251798, 0x1, 0x1401de410, 0x14384e390, 0x24, 0x143251388> as *const ()), // sub_14078fc00
    Replacement::draft(0x14078fc90, "content_14078fc90", fn_14078fc90 as *const ()), // sub_14078fc90
    Replacement::draft(0x14078fd10, "content_14078fd10", tpl_66989::<0x1406eca20, 0x14384e410, 0x20, 0x143251798, 0x1, 0x1401de410, 0x14384e390, 0x24, 0x143251388> as *const ()), // sub_14078fd10
    Replacement::draft(0x14078fda0, "content_14078fda0", tpl_8149::<0xa8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x143256f90> as *const ()), // sub_14078fda0
    Replacement::draft(0x14078feb0, "content_14078feb0", tpl_66397::<0x142535360, 0x1426456c8, 0x1426453a0, 0x143256f90, 0x14183acc2> as *const ()), // sub_14078feb0
    Replacement::draft(0x14078ffe0, "content_14078ffe0", tpl_7452::<0x14387cea0, 0x14243b0b0, 0x1406ea3a0, 0x14387d230, 0x28, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14078ffe0
    Replacement::draft(0x140790080, "content_140790080", tpl_8150::<0x14387cea0, 0x14243b0b0, 0x1406ea470, 0x338, 0x20, 0x1426d4580, 0x1, 0x28, 0x1406fc6d0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140790080
    Replacement::draft_throws(0x140790140, "content_140790140", tpl_8151::<0x14387ce70, 0x14387cea0, 0x1401e9fa0, 0x1406ea600, 0x328, 0x20, 0x338, 0x48, 0x369, 0x28, 0x1406d96a0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140790140
    Replacement::draft_throws(0x1407901e0, "content_1407901e0", tpl_8152::<0x14387cea0, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x28, 0x1406fca00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1407901e0
    Replacement::draft_throws(0x140790270, "content_140790270", tpl_8153::<0x14387ce70, 0x14387cea0, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x20, 0x369, 0x28, 0x1406d97d0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140790270
    Replacement::draft_throws(0x140790320, "content_140790320", fn_140790320 as *const (), Throws::Discard), // sub_140790320
    Replacement::draft_throws(0x140790400, "content_140790400", fn_140790400 as *const (), Throws::Caught), // sub_140790400
    Replacement::draft_throws(0x140790630, "content_140790630", fn_140790630 as *const (), Throws::Caught), // sub_140790630
    Replacement::draft_throws(0x140790820, "content_140790820", fn_140790820 as *const (), Throws::Caught), // sub_140790820
    Replacement::draft(0x140790a10, "content_140790a10", tpl_8158::<0x14387cea0, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x28, 0x1406fc920, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140790a10
    Replacement::draft(0x140790ab0, "content_140790ab0", tpl_66992::<0x14387ce70, 0x14387cea0, 0x1401e9970, 0x1406ec510, 0x20, 0x338, 0xe8, 0x28, 0x1406fce10, 0x1401e9860> as *const ()), // sub_140790ab0
    Replacement::draft(0x140790b40, "content_140790b40", tpl_8160::<0x14387cea0, 0x14243b0b0, 0x1406ec600, 0x20, 0x338, 0xe8, 0x28, 0x1406fcfb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140790b40
    Replacement::draft_throws(0x140790be0, "content_140790be0", tpl_66993::<0x14387ce70, 0x14387cea0, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x28, 0x1406fd150, 0x1401e9590> as *const (), Throws::Discard), // sub_140790be0
    Replacement::draft_throws(0x140790c60, "content_140790c60", tpl_8162::<0x14387cea0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x143251798, 0x28, 0x1432447a8, 0x1406da5c0> as *const (), Throws::Discard), // sub_140790c60
    Replacement::draft(0x140790d30, "content_140790d30", fn_140790d30 as *const ()), // sub_140790d30
    Replacement::draft(0x140790e40, "content_140790e40", fn_140790e40 as *const ()), // sub_140790e40
    Replacement::draft(0x140790f80, "content_140790f80", tpl_7452::<0x14387cf50, 0x14243b0b0, 0x1406ea3a0, 0x14387d230, 0x38, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140790f80
    Replacement::draft(0x140791020, "content_140791020", fn_140791020 as *const ()), // sub_140791020
    Replacement::draft_throws(0x1407911e0, "content_1407911e0", fn_1407911e0 as *const (), Throws::Discard), // sub_1407911e0
    Replacement::draft_throws(0x140791360, "content_140791360", fn_140791360 as *const (), Throws::Discard), // sub_140791360
    Replacement::draft_throws(0x140791450, "content_140791450", fn_140791450 as *const (), Throws::Discard), // sub_140791450
    Replacement::draft_throws(0x140791600, "content_140791600", fn_140791600 as *const (), Throws::Discard), // sub_140791600
    Replacement::draft_throws(0x140791740, "content_140791740", fn_140791740 as *const (), Throws::Caught), // sub_140791740
    Replacement::draft_throws(0x140791ca0, "content_140791ca0", fn_140791ca0 as *const (), Throws::Caught), // sub_140791ca0
    Replacement::draft_throws(0x140792130, "content_140792130", fn_140792130 as *const (), Throws::Caught), // sub_140792130
    Replacement::draft(0x1407925d0, "content_1407925d0", fn_1407925d0 as *const ()), // sub_1407925d0
    Replacement::draft(0x1407926c0, "content_1407926c0", fn_1407926c0 as *const ()), // sub_1407926c0
    Replacement::draft(0x1407927c0, "content_1407927c0", fn_1407927c0 as *const ()), // sub_1407927c0
    Replacement::draft_throws(0x1407928d0, "content_1407928d0", fn_1407928d0 as *const (), Throws::Discard), // sub_1407928d0
    Replacement::draft_throws(0x1407929e0, "content_1407929e0", fn_1407929e0 as *const (), Throws::Discard), // sub_1407929e0
    Replacement::draft(0x140792ba0, "content_140792ba0", tpl_8177::<0x1406ec8f0, 0x1426cdcb0, 0x14384e3d0, 0x20, 0x1432510f0, 0x1, 0x1401de410, 0x14384e410, 0x2c, 0x143251798, 0x14384e468, 0xb0, 0x143251098> as *const ()), // sub_140792ba0
    Replacement::draft(0x140792c60, "content_140792c60", fn_140792c60 as *const ()), // sub_140792c60
    Replacement::draft(0x140792d60, "content_140792d60", tpl_8177::<0x1406eca20, 0x1426cdcb0, 0x14384e3d0, 0x20, 0x1432510f0, 0x1, 0x1401de410, 0x14384e410, 0x2c, 0x143251798, 0x14384e468, 0xb0, 0x143251098> as *const ()), // sub_140792d60
    Replacement::draft(0x140792e20, "content_140792e20", tpl_8124::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x143256050> as *const ()), // sub_140792e20
    Replacement::draft(0x140792f30, "content_140792f30", tpl_66973::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143256050, 0xffffffffffffff88, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_140792f30
    Replacement::draft(0x1407930d0, "content_1407930d0", tpl_8019::<0x14387cf30, 0x14243b0b0, 0x140790080, 0x338, 0xa0, 0x1426d4580, 0x1, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407930d0
    Replacement::draft_throws(0x1407931b0, "content_1407931b0", tpl_8179::<0x14387cf40, 0x14387cf30, 0x1401e9fa0, 0x140790140, 0x328, 0xa0, 0x338, 0x38, 0x369, 0xa4, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407931b0
    Replacement::draft_throws(0x140793270, "content_140793270", tpl_8021::<0x14387cf30, 0x14243b0b0, 0x1407901e0, 0x338, 0xa0, 0x38, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140793270
    Replacement::draft_throws(0x140793300, "content_140793300", tpl_8180::<0x14387cf40, 0x14387cf30, 0x1401e9b60, 0x140790270, 0x328, 0x338, 0x120, 0xa0, 0x4, 0x369, 0xa4, 0x1401e9a20> as *const (), Throws::Discard), // sub_140793300
    Replacement::draft_throws(0x1407933e0, "content_1407933e0", tpl_8010::<0x8, 0x140790320, 0xa0, 0xa4> as *const (), Throws::Discard), // sub_1407933e0
    Replacement::draft_throws(0x140793440, "content_140793440", fn_140793440 as *const (), Throws::Caught), // sub_140793440
    Replacement::draft_throws(0x140793570, "content_140793570", fn_140793570 as *const (), Throws::Caught), // sub_140793570
    Replacement::draft_throws(0x1407936a0, "content_1407936a0", fn_1407936a0 as *const (), Throws::Caught), // sub_1407936a0
    Replacement::draft(0x1407937d0, "content_1407937d0", tpl_8182::<0x14387cf30, 0x14243b0b0, 0x140790a10, 0x338, 0xa0, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407937d0
    Replacement::draft(0x140793860, "content_140793860", tpl_8026::<0x14387cf40, 0x14387cf30, 0x1401e9970, 0x140790ab0, 0x338, 0xa0, 0xd8, 0xa4, 0x1401e9860> as *const ()), // sub_140793860
    Replacement::draft(0x1407938f0, "content_1407938f0", tpl_8027::<0x14387cf30, 0x14243b0b0, 0x140790b40, 0x338, 0xa0, 0xd8, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407938f0
    Replacement::draft_throws(0x140793990, "content_140793990", tpl_8028::<0x14387cf40, 0x14387cf30, 0x1401e9770, 0x140790be0, 0x338, 0xa0, 0x120, 0x4, 0xa4, 0x1401e9590> as *const (), Throws::Discard), // sub_140793990
    Replacement::draft_throws(0x140793a30, "content_140793a30", fn_140793a30 as *const (), Throws::Discard), // sub_140793a30
    Replacement::draft(0x140793b40, "content_140793b40", fn_140793b40 as *const ()), // sub_140793b40
    Replacement::draft(0x140793bf0, "content_140793bf0", tpl_8124::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x1432573d0> as *const ()), // sub_140793bf0
    Replacement::draft(0x140793d00, "content_140793d00", tpl_66973::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432573d0, 0xffffffffffffff88, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_140793d00
    Replacement::draft(0x140793ea0, "content_140793ea0", tpl_8185::<0x14387cf68, 0x14243b0b0, 0x140790080, 0x338, 0xa0, 0x1426d4580, 0x1, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140793ea0
    Replacement::draft_throws(0x140793f80, "content_140793f80", tpl_8179::<0x14387cfe8, 0x14387cf68, 0x1401e9fa0, 0x140790140, 0x328, 0xa0, 0x338, 0x48, 0x369, 0xa4, 0x1401e9e70> as *const (), Throws::Discard), // sub_140793f80
    Replacement::draft_throws(0x140794040, "content_140794040", tpl_8021::<0x14387cf68, 0x14243b0b0, 0x1407901e0, 0x338, 0xa0, 0x48, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140794040
    Replacement::draft_throws(0x1407940d0, "content_1407940d0", tpl_8186::<0x14387cfe8, 0x14387cf68, 0x1401e9b60, 0x140790270, 0x328, 0x338, 0x120, 0x369, 0xa0, 0xa4, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407940d0
    Replacement::draft_throws(0x1407941b0, "content_1407941b0", fn_1407941b0 as *const (), Throws::Discard), // sub_1407941b0
    Replacement::draft_throws(0x140794210, "content_140794210", fn_140794210 as *const (), Throws::Caught), // sub_140794210
    Replacement::draft_throws(0x140794320, "content_140794320", fn_140794320 as *const (), Throws::Caught), // sub_140794320
    Replacement::draft_throws(0x140794430, "content_140794430", fn_140794430 as *const (), Throws::Caught), // sub_140794430
    Replacement::draft(0x140794540, "content_140794540", tpl_8182::<0x14387cf68, 0x14243b0b0, 0x140790a10, 0x338, 0xa0, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140794540
    Replacement::draft(0x1407945d0, "content_1407945d0", tpl_8026::<0x14387cfe8, 0x14387cf68, 0x1401e9970, 0x140790ab0, 0x338, 0xa0, 0xe8, 0xa4, 0x1401e9860> as *const ()), // sub_1407945d0
    Replacement::draft(0x140794660, "content_140794660", tpl_8027::<0x14387cf68, 0x14243b0b0, 0x140790b40, 0x338, 0xa0, 0xe8, 0xa4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140794660
    Replacement::draft_throws(0x140794700, "content_140794700", tpl_67013::<0x14387cfe8, 0x14387cf68, 0x1401e9770, 0x140790be0, 0x338, 0xa0, 0x120, 0xa4, 0x1401e9590> as *const (), Throws::Discard), // sub_140794700
    Replacement::draft_throws(0x1407947a0, "content_1407947a0", fn_1407947a0 as *const (), Throws::Discard), // sub_1407947a0
    Replacement::draft(0x1407948a0, "content_1407948a0", tpl_7434::<0x130, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079b7f0> as *const ()), // sub_1407948a0
    Replacement::draft(0x140794930, "content_140794930", tpl_66380::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x14079b7f0> as *const ()), // sub_140794930
    Replacement::draft(0x1407949e0, "content_1407949e0", fn_1407949e0 as *const ()), // sub_1407949e0
    Replacement::draft(0x140794b40, "content_140794b40", fn_140794b40 as *const ()), // sub_140794b40
    Replacement::draft_throws(0x140794da0, "content_140794da0", fn_140794da0 as *const (), Throws::Discard), // sub_140794da0
    Replacement::draft_throws(0x140795000, "content_140795000", fn_140795000 as *const (), Throws::Discard), // sub_140795000
    Replacement::draft_throws(0x1407951a0, "content_1407951a0", fn_1407951a0 as *const (), Throws::Discard), // sub_1407951a0
    Replacement::draft_throws(0x140795460, "content_140795460", fn_140795460 as *const (), Throws::Discard), // sub_140795460
    Replacement::draft_throws(0x140795620, "content_140795620", fn_140795620 as *const (), Throws::Caught), // sub_140795620
    Replacement::draft_throws(0x140795940, "content_140795940", fn_140795940 as *const (), Throws::Caught), // sub_140795940
    Replacement::draft_throws(0x140795c60, "content_140795c60", fn_140795c60 as *const (), Throws::Caught), // sub_140795c60
    Replacement::draft(0x140795f80, "content_140795f80", fn_140795f80 as *const ()), // sub_140795f80
    Replacement::draft(0x1407960d0, "content_1407960d0", fn_1407960d0 as *const ()), // sub_1407960d0
    Replacement::draft(0x140796270, "content_140796270", fn_140796270 as *const ()), // sub_140796270
    Replacement::draft_throws(0x140796420, "content_140796420", fn_140796420 as *const (), Throws::Discard), // sub_140796420
    Replacement::draft_throws(0x140796600, "content_140796600", fn_140796600 as *const (), Throws::Discard), // sub_140796600
    Replacement::draft(0x1407968d0, "content_1407968d0", fn_1407968d0 as *const ()), // sub_1407968d0
    Replacement::draft(0x140796a00, "content_140796a00", tpl_8204::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14079bc00> as *const ()), // sub_140796a00
    Replacement::draft(0x140796a90, "content_140796a90", fn_140796a90 as *const ()), // sub_140796a90
    Replacement::draft(0x140796b40, "content_140796b40", fn_140796b40 as *const ()), // sub_140796b40
    Replacement::draft(0x140796c30, "content_140796c30", fn_140796c30 as *const ()), // sub_140796c30
    Replacement::draft_throws(0x140796d80, "content_140796d80", fn_140796d80 as *const (), Throws::Discard), // sub_140796d80
    Replacement::draft_throws(0x140796ec0, "content_140796ec0", fn_140796ec0 as *const (), Throws::Discard), // sub_140796ec0
    Replacement::draft_throws(0x140796f90, "content_140796f90", fn_140796f90 as *const (), Throws::Discard), // sub_140796f90
    Replacement::draft_throws(0x1407970f0, "content_1407970f0", fn_1407970f0 as *const (), Throws::Discard), // sub_1407970f0
    Replacement::draft_throws(0x140797240, "content_140797240", fn_140797240 as *const (), Throws::Caught), // sub_140797240
    Replacement::draft_throws(0x140797530, "content_140797530", fn_140797530 as *const (), Throws::Caught), // sub_140797530
    Replacement::draft_throws(0x1407977f0, "content_1407977f0", fn_1407977f0 as *const (), Throws::Caught), // sub_1407977f0
    Replacement::draft(0x140797ab0, "content_140797ab0", fn_140797ab0 as *const ()), // sub_140797ab0
    Replacement::draft(0x140797b90, "content_140797b90", fn_140797b90 as *const ()), // sub_140797b90
    Replacement::draft(0x140797c70, "content_140797c70", fn_140797c70 as *const ()), // sub_140797c70
    Replacement::draft_throws(0x140797d50, "content_140797d50", fn_140797d50 as *const (), Throws::Discard), // sub_140797d50
    Replacement::draft_throws(0x140797e40, "content_140797e40", fn_140797e40 as *const (), Throws::Discard), // sub_140797e40
    Replacement::draft(0x140797fd0, "content_140797fd0", tpl_8219::<0x1406ec8f0, 0x14384e3f8, 0x28, 0x1426cdcb0, 0x143250ff0, 0x1401de410> as *const ()), // sub_140797fd0
    Replacement::draft(0x140798030, "content_140798030", fn_140798030 as *const ()), // sub_140798030
    Replacement::draft(0x1407980d0, "content_1407980d0", tpl_8219::<0x1406eca20, 0x14384e3f8, 0x28, 0x1426cdcb0, 0x143250ff0, 0x1401de410> as *const ()), // sub_1407980d0
    Replacement::draft(0x140798130, "content_140798130", tpl_8149::<0xa8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x80, 0x8, 0x24, 0x44, 0x64, 0x84, 0x143256990> as *const ()), // sub_140798130
    Replacement::draft(0x140798240, "content_140798240", tpl_66397::<0x142535360, 0x1426456c8, 0x1426453a0, 0x143256990, 0x14183acc2> as *const ()), // sub_140798240
    Replacement::draft(0x140798370, "content_140798370", tpl_7452::<0x14387d000, 0x14243b0b0, 0x1406ea3a0, 0x14387d230, 0x28, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140798370
    Replacement::draft(0x140798410, "content_140798410", tpl_8150::<0x14387d000, 0x14243b0b0, 0x1406ea470, 0x338, 0x20, 0x1426d4580, 0x1, 0x28, 0x1406fc6d0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140798410
    Replacement::draft_throws(0x1407984d0, "content_1407984d0", tpl_8151::<0x14387ce40, 0x14387d000, 0x1401e9fa0, 0x1406ea600, 0x328, 0x20, 0x338, 0x48, 0x369, 0x28, 0x1406d96a0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407984d0
    Replacement::draft_throws(0x140798570, "content_140798570", tpl_8152::<0x14387d000, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x28, 0x1406fca00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140798570
    Replacement::draft_throws(0x140798600, "content_140798600", tpl_8153::<0x14387ce40, 0x14387d000, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x20, 0x369, 0x28, 0x1406d97d0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140798600
    Replacement::draft_throws(0x1407986b0, "content_1407986b0", fn_1407986b0 as *const (), Throws::Caught), // sub_1407986b0
    Replacement::draft_throws(0x1407988e0, "content_1407988e0", fn_1407988e0 as *const (), Throws::Caught), // sub_1407988e0
    Replacement::draft_throws(0x140798ad0, "content_140798ad0", fn_140798ad0 as *const (), Throws::Caught), // sub_140798ad0
    Replacement::draft(0x140798cc0, "content_140798cc0", tpl_8158::<0x14387d000, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x28, 0x1406fc920, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140798cc0
    Replacement::draft(0x140798d60, "content_140798d60", tpl_66992::<0x14387ce40, 0x14387d000, 0x1401e9970, 0x1406ec510, 0x20, 0x338, 0xe8, 0x28, 0x1406fce10, 0x1401e9860> as *const ()), // sub_140798d60
    Replacement::draft(0x140798df0, "content_140798df0", tpl_8160::<0x14387d000, 0x14243b0b0, 0x1406ec600, 0x20, 0x338, 0xe8, 0x28, 0x1406fcfb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140798df0
    Replacement::draft_throws(0x140798e90, "content_140798e90", tpl_66993::<0x14387ce40, 0x14387d000, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x28, 0x1406fd150, 0x1401e9590> as *const (), Throws::Discard), // sub_140798e90
    Replacement::draft_throws(0x140798f10, "content_140798f10", tpl_8162::<0x14387d000, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x143251798, 0x28, 0x1432447a8, 0x1406da5c0> as *const (), Throws::Discard), // sub_140798f10
    Replacement::draft(0x140798fe0, "content_140798fe0", fn_140798fe0 as *const ()), // sub_140798fe0
    Replacement::draft(0x140799070, "content_140799070", fn_140799070 as *const ()), // sub_140799070
    Replacement::draft(0x140799120, "content_140799120", fn_140799120 as *const ()), // sub_140799120
    Replacement::draft(0x140799250, "content_140799250", fn_140799250 as *const ()), // sub_140799250
    Replacement::draft_throws(0x1407993c0, "content_1407993c0", fn_1407993c0 as *const (), Throws::Discard), // sub_1407993c0
    Replacement::draft_throws(0x1407995b0, "content_1407995b0", fn_1407995b0 as *const (), Throws::Discard), // sub_1407995b0
    Replacement::draft_throws(0x140799710, "content_140799710", fn_140799710 as *const (), Throws::Discard), // sub_140799710
    Replacement::draft_throws(0x1407998f0, "content_1407998f0", fn_1407998f0 as *const (), Throws::Discard), // sub_1407998f0
    Replacement::draft_throws(0x140799ac0, "content_140799ac0", fn_140799ac0 as *const (), Throws::Caught), // sub_140799ac0
    Replacement::draft_throws(0x14079a060, "content_14079a060", fn_14079a060 as *const (), Throws::Caught), // sub_14079a060
    Replacement::draft_throws(0x14079a540, "content_14079a540", fn_14079a540 as *const (), Throws::Caught), // sub_14079a540
    Replacement::draft(0x14079aa10, "content_14079aa10", fn_14079aa10 as *const ()), // sub_14079aa10
    Replacement::draft(0x14079ab40, "content_14079ab40", fn_14079ab40 as *const ()), // sub_14079ab40
    Replacement::draft(0x14079aca0, "content_14079aca0", fn_14079aca0 as *const ()), // sub_14079aca0
    Replacement::draft_throws(0x14079ae10, "content_14079ae10", fn_14079ae10 as *const (), Throws::Discard), // sub_14079ae10
    Replacement::draft_throws(0x14079af30, "content_14079af30", fn_14079af30 as *const (), Throws::Discard), // sub_14079af30
    Replacement::draft_throws(0x14079b120, "content_14079b120", fn_14079b120 as *const (), Throws::Discard), // sub_14079b120
    Replacement::draft(0x14079b280, "content_14079b280", fn_14079b280 as *const ()), // content::LogicBlockDecorator::content::LogicBlockDecorator::vfunc_0
    Replacement::draft(0x14079b2c0, "content_14079b2c0", tpl_8237::<0x143251bd0, 0x8, 0x10, 0x18, 0x80, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88> as *const ()), // sub_14079b2c0
    Replacement::draft_throws(0x14079b390, "content_14079b390", fn_14079b390 as *const (), Throws::Discard), // sub_14079b390
    Replacement::draft(0x14079b440, "content_14079b440", fn_14079b440 as *const ()), // sub_14079b440
    Replacement::draft(0x14079b580, "content_14079b580", fn_14079b580 as *const ()), // content::CallScriptDecorator::content::CallScriptDecorator::vfunc_0
    Replacement::draft(0x14079b5d0, "content_14079b5d0", tpl_8237::<0x143252e10, 0x8, 0x10, 0x18, 0x80, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88> as *const ()), // sub_14079b5d0
    Replacement::draft(0x14079b6a0, "content_14079b6a0", fn_14079b6a0 as *const ()), // sub_14079b6a0
    Replacement::draft(0x14079b7f0, "content_14079b7f0", tpl_8242::<0x143257c10, 0x8, 0x10, 0x18, 0x78, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88> as *const ()), // sub_14079b7f0
    Replacement::draft_throws(0x14079b8c0, "content_14079b8c0", fn_14079b8c0 as *const (), Throws::Discard), // sub_14079b8c0
    Replacement::draft(0x14079b9c0, "content_14079b9c0", fn_14079b9c0 as *const ()), // sub_14079b9c0
    Replacement::draft(0x14079bb00, "content_14079bb00", tpl_8245::<0x1401ffd50, 0x141834d30> as *const ()), // content::ReleaseTicketDecorator::content::ReleaseTicketDecorator::vfunc_0
    Replacement::draft(0x14079bb40, "content_14079bb40", fn_14079bb40 as *const ()), // content::SayDecorator::content::SayDecorator::vfunc_0
    Replacement::draft_throws(0x14079bb90, "content_14079bb90", fn_14079bb90 as *const (), Throws::Discard), // sub_14079bb90
    Replacement::draft(0x14079bc00, "content_14079bc00", fn_14079bc00 as *const ()), // sub_14079bc00
    Replacement::draft(0x14079bce0, "content_14079bce0", fn_14079bce0 as *const ()), // sub_14079bce0
    Replacement::draft(0x14079be30, "content_14079be30", fn_14079be30 as *const ()), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_0
    Replacement::draft(0x14079beb0, "content_14079beb0", fn_14079beb0 as *const ()), // content::CopyPropertyDecorator::content::CopyPropertyDecorator::vfunc_0
    Replacement::draft(0x14079bf30, "content_14079bf30", fn_14079bf30 as *const ()), // content::CalculateDirectionDecorator::content::CalculateDirectionDecorator::vfunc_0
    Replacement::draft(0x14079bfc0, "content_14079bfc0", fn_14079bfc0 as *const ()), // content::DamageTrackingDecorator::content::DamageTrackingDecorator::vfunc_0
    Replacement::draft(0x14079c040, "content_14079c040", fn_14079c040 as *const ()), // content::DecoratorCanThrowCondition::content::DecoratorCanThrowCondition::vfunc_0
    Replacement::draft(0x14079c100, "content_14079c100", fn_14079c100 as *const ()), // content::UnsetPropertyDecorator::content::UnsetPropertyDecorator::vfunc_0
    Replacement::draft(0x14079c160, "content_14079c160", fn_14079c160 as *const ()), // content::FollowSpeedDecorator::content::FollowSpeedDecorator::vfunc_0
    Replacement::draft(0x14079c1e0, "content_14079c1e0", fn_14079c1e0 as *const ()), // content::SpeedMatchingDecorator::content::SpeedMatchingDecorator::vfunc_0
    Replacement::draft(0x14079c260, "content_14079c260", fn_14079c260 as *const ()), // content::LeadDecorator::content::LeadDecorator::vfunc_0
    Replacement::draft(0x14079c2e0, "content_14079c2e0", fn_14079c2e0 as *const ()), // content::IsPropertySetConditionDecorator::content::IsPropertySetConditionDecorator::vfunc_0
    Replacement::draft(0x14079c340, "content_14079c340", fn_14079c340 as *const ()), // content::ModifyIntPropertyDecorator::content::ModifyIntPropertyDecorator::vfunc_0
    Replacement::draft(0x14079c3d0, "content_14079c3d0", fn_14079c3d0 as *const ()), // content::SetIntKnowledgePropertyDecorator::content::SetIntKnowledgePropertyDecorator::vfunc_0
    Replacement::draft(0x14079c430, "content_14079c430", fn_14079c430 as *const ()), // content::SetPositionReservationDecorator::content::SetPositionReservationDecorator::vfunc_0
    Replacement::draft(0x14079c490, "content_14079c490", fn_14079c490 as *const ()), // content::TimeSincePropertySetConditionDecorator::content::TimeSincePropertySetConditionDecorator::vfunc_0
    Replacement::draft(0x14079c510, "content_14079c510", fn_14079c510 as *const ()), // content::WarningShotDecorator::content::WarningShotDecorator::vfunc_0
    Replacement::draft_throws(0x14079c5a0, "content_14079c5a0", fn_14079c5a0 as *const (), Throws::Caught), // sub_14079c5a0
    Replacement::draft_throws(0x14079c8a0, "content_14079c8a0", fn_14079c8a0 as *const (), Throws::Caught), // sub_14079c8a0
    Replacement::draft_throws(0x14079cba0, "content_14079cba0", fn_14079cba0 as *const (), Throws::Caught), // sub_14079cba0
    Replacement::draft_throws(0x14079cea0, "content_14079cea0", fn_14079cea0 as *const (), Throws::Caught), // sub_14079cea0
    Replacement::draft_throws(0x14079d1a0, "content_14079d1a0", fn_14079d1a0 as *const (), Throws::Caught), // sub_14079d1a0
    Replacement::draft_throws(0x14079d4a0, "content_14079d4a0", fn_14079d4a0 as *const (), Throws::Caught), // sub_14079d4a0
    Replacement::draft_throws(0x14079d7a0, "content_14079d7a0", fn_14079d7a0 as *const (), Throws::Caught), // sub_14079d7a0
    Replacement::draft_throws(0x14079d980, "content_14079d980", fn_14079d980 as *const (), Throws::Caught), // sub_14079d980
    Replacement::draft_throws(0x14079db30, "content_14079db30", fn_14079db30 as *const (), Throws::Caught), // sub_14079db30
    Replacement::draft_throws(0x14079e0e0, "content_14079e0e0", fn_14079e0e0 as *const (), Throws::Caught), // sub_14079e0e0
    Replacement::draft_throws(0x14079e650, "content_14079e650", fn_14079e650 as *const (), Throws::Caught), // sub_14079e650
    Replacement::draft(0x14079ec20, "content_14079ec20", tpl_7399::<0x3, 0x4, 0x8, 0x14323f668, 0x142409b50> as *const ()), // content::BehaviorTreeAnimatedMoveToNode::content::BehaviorTreeAnimatedMoveToNode::vfunc_22
    Replacement::draft_throws(0x14079ec80, "content_14079ec80", tpl_7307::<0x143cc33f8, 0x143cc3360, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeAnimatedMoveToNode::content::BehaviorTreeAnimatedMoveToNode::vfunc_23
    Replacement::draft(0x14079edd0, "content_14079edd0", tpl_7389::<0x3, 0x4, 0x8, 0x14323e378, 0x142409b50> as *const ()), // content::BehaviorTreeChildTreeNode::content::BehaviorTreeChildTreeNode::vfunc_22
    Replacement::draft_throws(0x14079ee30, "content_14079ee30", tpl_7307::<0x143cc33d8, 0x143cc3338, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeChildTreeNode::content::BehaviorTreeChildTreeNode::vfunc_23
    Replacement::draft(0x14079ef50, "content_14079ef50", tpl_7400::<0x3, 0x4, 0x8, 0x14323dc80, 0x142409b50> as *const ()), // content::BehaviorTreeDetachChildTreeNode::content::BehaviorTreeDetachChildTreeNode::vfunc_22
    Replacement::draft_throws(0x14079efb0, "content_14079efb0", tpl_7307::<0x143cc33f4, 0x143cc33e0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeDetachChildTreeNode::content::BehaviorTreeDetachChildTreeNode::vfunc_23
    Replacement::draft(0x14079f0d0, "content_14079f0d0", tpl_7391::<0x3, 0x4, 0x8, 0x14323d960, 0x142409b50> as *const ()), // content::BehaviorTreeEQSQueryNode::content::BehaviorTreeEQSQueryNode::vfunc_22
    Replacement::draft_throws(0x14079f130, "content_14079f130", tpl_7307::<0x143cc33d4, 0x143cc3448, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeEQSQueryNode::content::BehaviorTreeEQSQueryNode::vfunc_23
    Replacement::draft(0x14079f250, "content_14079f250", tpl_7400::<0x3, 0x4, 0x8, 0x14323d640, 0x142409b50> as *const ()), // content::BehaviorTreeEquipNextWeaponNode::content::BehaviorTreeEquipNextWeaponNode::vfunc_22
    Replacement::draft_throws(0x14079f2b0, "content_14079f2b0", tpl_7307::<0x143cc33f0, 0x143cc3450, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeEquipNextWeaponNode::content::BehaviorTreeEquipNextWeaponNode::vfunc_23
    Replacement::draft(0x14079f3d0, "content_14079f3d0", tpl_7826::<0x3, 0x4, 0x8, 0x14323d618, 0x142409b50> as *const ()), // content::BehaviorTreeFlyToNode::content::BehaviorTreeFlyToNode::vfunc_22
    Replacement::draft_throws(0x14079f430, "content_14079f430", tpl_7307::<0x143cc33bc, 0x143cc3380, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeFlyToNode::content::BehaviorTreeFlyToNode::vfunc_23
    Replacement::draft(0x14079f550, "content_14079f550", tpl_7391::<0x3, 0x4, 0x8, 0x14323d008, 0x142409b50> as *const ()), // content::BehaviorTreeLevitateNode::content::BehaviorTreeLevitateNode::vfunc_22
    Replacement::draft_throws(0x14079f5b0, "content_14079f5b0", tpl_7307::<0x143cc3378, 0x143cc3430, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeLevitateNode::content::BehaviorTreeLevitateNode::vfunc_23
    Replacement::draft(0x14079f6d0, "content_14079f6d0", tpl_7388::<0x3, 0x4, 0x8, 0x14323ccf0, 0x142409b50> as *const ()), // content::BehaviorTreeLoopNode::content::BehaviorTreeLoopNode::vfunc_22
    Replacement::draft_throws(0x14079f730, "content_14079f730", tpl_7307::<0x143cc3428, 0x143cc33e8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeLoopNode::content::BehaviorTreeLoopNode::vfunc_23
    Replacement::draft(0x14079f850, "content_14079f850", tpl_7309::<0x3, 0x4, 0x8, 0x14323c9a0, 0x142409b50> as *const ()), // content::BehaviorTreeMoveOnRouteNode::content::BehaviorTreeMoveOnRouteNode::vfunc_22
    Replacement::draft_throws(0x14079f8b0, "content_14079f8b0", tpl_7307::<0x143cc3328, 0x143cc33c0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeMoveOnRouteNode::content::BehaviorTreeMoveOnRouteNode::vfunc_23
    Replacement::draft(0x14079f9d0, "content_14079f9d0", tpl_7309::<0x3, 0x4, 0x8, 0x14323c970, 0x142409b50> as *const ()), // content::BehaviorTreeMoveToCoverNode::content::BehaviorTreeMoveToCoverNode::vfunc_22
    Replacement::draft_throws(0x14079fa30, "content_14079fa30", tpl_7307::<0x143cc3318, 0x143cc33a0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeMoveToCoverNode::content::BehaviorTreeMoveToCoverNode::vfunc_23
    Replacement::draft(0x14079fb50, "content_14079fb50", tpl_7391::<0x3, 0x4, 0x8, 0x14323c360, 0x142409b50> as *const ()), // content::BehaviorTreeMoveToFollow::content::BehaviorTreeMoveToFollow::vfunc_22
    Replacement::draft_throws(0x14079fbb0, "content_14079fbb0", tpl_7307::<0x143cc32fc, 0x143cc3310, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeMoveToFollow::content::BehaviorTreeMoveToFollow::vfunc_23
    Replacement::draft(0x14079fcd0, "content_14079fcd0", tpl_7305::<0x3, 0x4, 0x8, 0x14323c338, 0x142409b50> as *const ()), // content::BehaviorTreeMoveToNode::content::BehaviorTreeMoveToNode::vfunc_22
    Replacement::draft_throws(0x14079fd30, "content_14079fd30", tpl_7307::<0x143cc3410, 0x143cc3308, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeMoveToNode::content::BehaviorTreeMoveToNode::vfunc_23
    Replacement::draft(0x14079fe50, "content_14079fe50", tpl_7391::<0x3, 0x4, 0x8, 0x14323bb40, 0x142409b50> as *const ()), // content::BehaviorTreeParallelNode::content::BehaviorTreeParallelNode::vfunc_22
    Replacement::draft_throws(0x14079feb0, "content_14079feb0", tpl_7307::<0x143cc3300, 0x143cc3438, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeParallelNode::content::BehaviorTreeParallelNode::vfunc_23
    Replacement::draft(0x14079ffd0, "content_14079ffd0", tpl_7399::<0x3, 0x4, 0x8, 0x14323bb10, 0x142409b50> as *const ()), // content::BehaviorTreeRandomSelectorNode::content::BehaviorTreeRandomSelectorNode::vfunc_22
    Replacement::draft_throws(0x1407a0030, "content_1407a0030", tpl_7307::<0x143cc3388, 0x143cc3370, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeRandomSelectorNode::content::BehaviorTreeRandomSelectorNode::vfunc_23
    Replacement::draft(0x1407a0150, "content_1407a0150", tpl_7389::<0x3, 0x4, 0x8, 0x14323b500, 0x142409b50> as *const ()), // content::BehaviorTreeReferenceNode::content::BehaviorTreeReferenceNode::vfunc_22
    Replacement::draft_throws(0x1407a01b0, "content_1407a01b0", tpl_7307::<0x143cc3358, 0x143cc3320, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeReferenceNode::content::BehaviorTreeReferenceNode::vfunc_23
    Replacement::draft(0x1407a02d0, "content_1407a02d0", tpl_7388::<0x3, 0x4, 0x8, 0x14323b1e8, 0x142409b50> as *const ()), // content::BehaviorTreeRootNode::content::BehaviorTreeRootNode::vfunc_22
    Replacement::draft_throws(0x1407a0330, "content_1407a0330", tpl_7307::<0x143c89f7c, 0x143c89f80, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeRootNode::content::BehaviorTreeRootNode::vfunc_23
    Replacement::draft(0x1407a04e0, "content_1407a04e0", tpl_7387::<0x3, 0x4, 0x8, 0x14323aed0, 0x142409b50> as *const ()), // content::BehaviorTreeSayNode::content::BehaviorTreeSayNode::vfunc_22
    Replacement::draft_throws(0x1407a0540, "content_1407a0540", tpl_7307::<0x143cc3424, 0x143cc3400, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeSayNode::content::BehaviorTreeSayNode::vfunc_23
    Replacement::draft(0x1407a0660, "content_1407a0660", tpl_7391::<0x3, 0x4, 0x8, 0x14323abb0, 0x142409b50> as *const ()), // content::BehaviorTreeSelectorNode::content::BehaviorTreeSelectorNode::vfunc_22
    Replacement::draft_throws(0x1407a06c0, "content_1407a06c0", tpl_7307::<0x143cc337c, 0x143cc33c8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeSelectorNode::content::BehaviorTreeSelectorNode::vfunc_23
    Replacement::draft(0x1407a07e0, "content_1407a07e0", tpl_7391::<0x3, 0x4, 0x8, 0x14323a760, 0x142409b50> as *const ()), // content::BehaviorTreeSequenceNode::content::BehaviorTreeSequenceNode::vfunc_22
    Replacement::draft_throws(0x1407a0840, "content_1407a0840", tpl_7307::<0x143cc335c, 0x143cc33b0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeSequenceNode::content::BehaviorTreeSequenceNode::vfunc_23
    Replacement::draft(0x1407a0960, "content_1407a0960", tpl_7827::<0x3, 0x4, 0x8, 0x14323a160, 0x142409b50> as *const ()), // content::BehaviorTreeShootAtNode::content::BehaviorTreeShootAtNode::vfunc_22
    Replacement::draft_throws(0x1407a09c0, "content_1407a09c0", tpl_7307::<0x143cc3420, 0x143cc3348, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeShootAtNode::content::BehaviorTreeShootAtNode::vfunc_23
    Replacement::draft(0x1407a0ae0, "content_1407a0ae0", tpl_7399::<0x3, 0x4, 0x8, 0x14323a130, 0x142409b50> as *const ()), // content::BehaviorTreeShootFromCoverNode::content::BehaviorTreeShootFromCoverNode::vfunc_22
    Replacement::draft_throws(0x1407a0b40, "content_1407a0b40", tpl_7307::<0x143cc33d0, 0x143cc3418, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeShootFromCoverNode::content::BehaviorTreeShootFromCoverNode::vfunc_23
    Replacement::draft(0x1407a0c60, "content_1407a0c60", tpl_7827::<0x3, 0x4, 0x8, 0x143239ad8, 0x142409b50> as *const ()), // content::BehaviorTreeStunnedNode::content::BehaviorTreeStunnedNode::vfunc_22
    Replacement::draft_throws(0x1407a0cc0, "content_1407a0cc0", tpl_7307::<0x143cc332c, 0x143cc3398, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeStunnedNode::content::BehaviorTreeStunnedNode::vfunc_23
    Replacement::draft(0x1407a0de0, "content_1407a0de0", tpl_7389::<0x3, 0x4, 0x8, 0x143238658, 0x142409b50> as *const ()), // content::BehaviorTreeTakeCoverNode::content::BehaviorTreeTakeCoverNode::vfunc_22
    Replacement::draft_throws(0x1407a0e40, "content_1407a0e40", tpl_7307::<0x143cc338c, 0x143cc3330, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeTakeCoverNode::content::BehaviorTreeTakeCoverNode::vfunc_23
    Replacement::draft(0x1407a0f60, "content_1407a0f60", tpl_7391::<0x3, 0x4, 0x8, 0x143237e58, 0x142409b50> as *const ()), // content::BehaviorTreeTeleportNode::content::BehaviorTreeTeleportNode::vfunc_22
    Replacement::draft_throws(0x1407a0fc0, "content_1407a0fc0", tpl_7307::<0x143cc33b8, 0x143cc3390, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeTeleportNode::content::BehaviorTreeTeleportNode::vfunc_23
    Replacement::draft(0x1407a10e0, "content_1407a10e0", tpl_7388::<0x3, 0x4, 0x8, 0x143237b40, 0x142409b50> as *const ()), // content::BehaviorTreeTestNode::content::BehaviorTreeTestNode::vfunc_22
    Replacement::draft_throws(0x1407a1140, "content_1407a1140", tpl_7307::<0x143cc3440, 0x143cc33a8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeTestNode::content::BehaviorTreeTestNode::vfunc_23
    Replacement::draft(0x1407a12c0, "content_1407a12c0", tpl_7308::<0x3, 0x4, 0x8, 0x143237950, 0x142409b50> as *const ()), // content::BehaviorTreeUseAbilityNode::content::BehaviorTreeUseAbilityNode::vfunc_22
    Replacement::draft_throws(0x1407a1320, "content_1407a1320", tpl_7307::<0x143c89f90, 0x143c89f88, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeUseAbilityNode::content::BehaviorTreeUseAbilityNode::vfunc_23
    Replacement::draft(0x1407a1440, "content_1407a1440", tpl_7309::<0x3, 0x4, 0x8, 0x143237920, 0x142409b50> as *const ()), // content::BehaviorTreeUseWorkspotNode::content::BehaviorTreeUseWorkspotNode::vfunc_22
    Replacement::draft_throws(0x1407a14a0, "content_1407a14a0", tpl_7307::<0x143cc3368, 0x143cc3408, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeUseWorkspotNode::content::BehaviorTreeUseWorkspotNode::vfunc_23
    Replacement::draft(0x1407a15c0, "content_1407a15c0", tpl_7388::<0x3, 0x4, 0x8, 0x143237318, 0x142409b50> as *const ()), // content::BehaviorTreeWaitNode::content::BehaviorTreeWaitNode::vfunc_22
    Replacement::draft_throws(0x1407a1620, "content_1407a1620", tpl_7307::<0x143cc3340, 0x143cc3350, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeWaitNode::content::BehaviorTreeWaitNode::vfunc_23
    Replacement::draft(0x1407a1740, "content_1407a1740", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1407caae0> as *const ()), // sub_1407a1740
    Replacement::draft(0x1407a17d0, "content_1407a17d0", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x1407caae0> as *const ()), // sub_1407a17d0
    Replacement::draft(0x1407a1880, "content_1407a1880", tpl_7512::<0x14387d0b8, 0x14243b0b0, 0x1407c0230, 0x14387d230, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8> as *const ()), // sub_1407a1880
    Replacement::draft(0x1407a1970, "content_1407a1970", fn_1407a1970 as *const ()), // sub_1407a1970
    Replacement::draft_throws(0x1407a1ab0, "content_1407a1ab0", fn_1407a1ab0 as *const (), Throws::Caught), // sub_1407a1ab0
    Replacement::draft_throws(0x1407a1c40, "content_1407a1c40", fn_1407a1c40 as *const (), Throws::Discard), // sub_1407a1c40
    Replacement::draft_throws(0x1407a1d50, "content_1407a1d50", fn_1407a1d50 as *const (), Throws::Discard), // sub_1407a1d50
    Replacement::draft_throws(0x1407a1ea0, "content_1407a1ea0", fn_1407a1ea0 as *const (), Throws::Discard), // sub_1407a1ea0
    Replacement::draft_throws(0x1407a1fa0, "content_1407a1fa0", fn_1407a1fa0 as *const (), Throws::Caught), // sub_1407a1fa0
    Replacement::draft_throws(0x1407a2260, "content_1407a2260", fn_1407a2260 as *const (), Throws::Caught), // sub_1407a2260
    Replacement::draft_throws(0x1407a24f0, "content_1407a24f0", fn_1407a24f0 as *const (), Throws::Caught), // sub_1407a24f0
    Replacement::draft(0x1407a2770, "content_1407a2770", fn_1407a2770 as *const ()), // sub_1407a2770
    Replacement::draft(0x1407a2910, "content_1407a2910", fn_1407a2910 as *const ()), // sub_1407a2910
    Replacement::draft(0x1407a2a30, "content_1407a2a30", fn_1407a2a30 as *const ()), // sub_1407a2a30
    Replacement::draft_throws(0x1407a2b30, "content_1407a2b30", fn_1407a2b30 as *const (), Throws::Discard), // sub_1407a2b30
    Replacement::draft_throws(0x1407a2c50, "content_1407a2c50", fn_1407a2c50 as *const (), Throws::Discard), // sub_1407a2c50
    Replacement::draft(0x1407a2dd0, "content_1407a2dd0", tpl_8284::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x143259908, 0x20, 0x28, 0x30> as *const ()), // sub_1407a2dd0
    Replacement::draft(0x1407a2e70, "content_1407a2e70", tpl_8285::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143259908, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_1407a2e70
    Replacement::draft(0x1407a2f90, "content_1407a2f90", tpl_7836::<0x14387d128, 0x14243b0b0, 0x1407a3e60, 0x38, 0x1426d4580, 0x338, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407a2f90
    Replacement::draft_throws(0x1407a3040, "content_1407a3040", fn_1407a3040 as *const (), Throws::Caught), // sub_1407a3040
    Replacement::draft_throws(0x1407a3140, "content_1407a3140", tpl_8287::<0x14387d128, 0x14243b0b0, 0x1407a3ed0, 0x338, 0x20, 0x38, 0x14240ab10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_1407a3140
    Replacement::draft_throws(0x1407a3200, "content_1407a3200", tpl_8288::<0x14387d124, 0x14387d128, 0x1401e9b60, 0x14387d0c0, 0x14387d1c8, 0x1407b44c0, 0x1401e9a20, 0x1426cdcb0, 0x38, 0x1406a6ae0> as *const (), Throws::Discard), // sub_1407a3200
    Replacement::draft_throws(0x1407a32b0, "content_1407a32b0", fn_1407a32b0 as *const (), Throws::Caught), // sub_1407a32b0
    Replacement::draft_throws(0x1407a34b0, "content_1407a34b0", fn_1407a34b0 as *const (), Throws::Caught), // sub_1407a34b0
    Replacement::draft_throws(0x1407a3680, "content_1407a3680", fn_1407a3680 as *const (), Throws::Caught), // sub_1407a3680
    Replacement::draft(0x1407a3840, "content_1407a3840", tpl_8292::<0x14387d128, 0x14243b0b0, 0x1407a4120, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_1407a3840
    Replacement::draft(0x1407a3990, "content_1407a3990", tpl_8293::<0x14387d124, 0x14387d128, 0x1401e9970, 0x14387d0c0, 0x14387d1c8, 0x1407b4c60, 0x1401e9860, 0x338, 0x1426cdcb0, 0x38, 0xa0, 0x88> as *const ()), // sub_1407a3990
    Replacement::draft(0x1407a3a50, "content_1407a3a50", tpl_7846::<0x14387d128, 0x14243b0b0, 0x1407a4190, 0x338, 0x1426cdcb0, 0x38, 0xa0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407a3a50
    Replacement::draft_throws(0x1407a3af0, "content_1407a3af0", tpl_8294::<0x14387d124, 0x14387d128, 0x1401e9770, 0x14387d0c0, 0x14387d1c8, 0x1407b5100, 0x1401e9590, 0x338, 0x38, 0x120> as *const (), Throws::Discard), // sub_1407a3af0
    Replacement::draft_throws(0x1407a3bc0, "content_1407a3bc0", tpl_8295::<0x14387d128, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d1c8, 0x1407b5310, 0x38, 0x14325e938, 0x1406d6fb0> as *const (), Throws::Discard), // sub_1407a3bc0
    Replacement::draft(0x1407a3ca0, "content_1407a3ca0", tpl_8296::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14325cc70, 0x20, 0x28, 0x30> as *const ()), // sub_1407a3ca0
    Replacement::draft(0x1407a3d40, "content_1407a3d40", tpl_8297::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325cc70, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_1407a3d40
    Replacement::draft_throws(0x1407a3f40, "content_1407a3f40", fn_1407a3f40 as *const (), Throws::Caught), // sub_1407a3f40
    Replacement::draft_throws(0x1407a3fe0, "content_1407a3fe0", fn_1407a3fe0 as *const (), Throws::Caught), // sub_1407a3fe0
    Replacement::draft_throws(0x1407a4080, "content_1407a4080", fn_1407a4080 as *const (), Throws::Caught), // sub_1407a4080
    Replacement::draft_throws(0x1407a4200, "content_1407a4200", tpl_7326::<0x14387d1c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1407b5310> as *const (), Throws::Discard), // sub_1407a4200
    Replacement::draft(0x1407a42a0, "content_1407a42a0", tpl_8298::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143249e28, 0x8> as *const ()), // sub_1407a42a0
    Replacement::draft(0x1407a4330, "content_1407a4330", tpl_8299::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x143249e28, 0xffffffffffffffec, 0xffffffffffffffe4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_1407a4330
    Replacement::draft(0x1407a43d0, "content_1407a43d0", tpl_8284::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x14325a7f0, 0x20, 0x28, 0x30> as *const ()), // sub_1407a43d0
    Replacement::draft(0x1407a4470, "content_1407a4470", tpl_8285::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325a7f0, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_1407a4470
    Replacement::draft(0x1407a4590, "content_1407a4590", tpl_7836::<0x14387d0f8, 0x14243b0b0, 0x1407a3e60, 0x38, 0x1426d4580, 0x338, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407a4590
    Replacement::draft_throws(0x1407a4640, "content_1407a4640", fn_1407a4640 as *const (), Throws::Caught), // sub_1407a4640
    Replacement::draft_throws(0x1407a4740, "content_1407a4740", tpl_8287::<0x14387d0f8, 0x14243b0b0, 0x1407a3ed0, 0x338, 0x20, 0x38, 0x14240ab10, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_1407a4740
    Replacement::draft_throws(0x1407a4800, "content_1407a4800", tpl_8288::<0x14387d090, 0x14387d0f8, 0x1401e9b60, 0x14387d0c0, 0x14387d1c8, 0x1407b44c0, 0x1401e9a20, 0x1426cdcb0, 0x38, 0x1406a6ae0> as *const (), Throws::Discard), // sub_1407a4800
    Replacement::draft_throws(0x1407a48b0, "content_1407a48b0", fn_1407a48b0 as *const (), Throws::Caught), // sub_1407a48b0
    Replacement::draft_throws(0x1407a4ab0, "content_1407a4ab0", fn_1407a4ab0 as *const (), Throws::Caught), // sub_1407a4ab0
    Replacement::draft_throws(0x1407a4c80, "content_1407a4c80", fn_1407a4c80 as *const (), Throws::Caught), // sub_1407a4c80
    Replacement::draft(0x1407a4e40, "content_1407a4e40", tpl_8292::<0x14387d0f8, 0x14243b0b0, 0x1407a4120, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_1407a4e40
    Replacement::draft(0x1407a4f90, "content_1407a4f90", tpl_8293::<0x14387d090, 0x14387d0f8, 0x1401e9970, 0x14387d0c0, 0x14387d1c8, 0x1407b4c60, 0x1401e9860, 0x338, 0x1426cdcb0, 0x38, 0xa0, 0x88> as *const ()), // sub_1407a4f90
    Replacement::draft(0x1407a5050, "content_1407a5050", tpl_7846::<0x14387d0f8, 0x14243b0b0, 0x1407a4190, 0x338, 0x1426cdcb0, 0x38, 0xa0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407a5050
    Replacement::draft_throws(0x1407a50f0, "content_1407a50f0", tpl_8294::<0x14387d090, 0x14387d0f8, 0x1401e9770, 0x14387d0c0, 0x14387d1c8, 0x1407b5100, 0x1401e9590, 0x338, 0x38, 0x120> as *const (), Throws::Discard), // sub_1407a50f0
    Replacement::draft_throws(0x1407a51c0, "content_1407a51c0", tpl_8295::<0x14387d0f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d1c8, 0x1407b5310, 0x38, 0x14325e938, 0x1406d6fb0> as *const (), Throws::Discard), // sub_1407a51c0
    Replacement::draft(0x1407a52a0, "content_1407a52a0", fn_1407a52a0 as *const ()), // sub_1407a52a0
    Replacement::draft(0x1407a5330, "content_1407a5330", fn_1407a5330 as *const ()), // sub_1407a5330
    Replacement::draft(0x1407a53e0, "content_1407a53e0", fn_1407a53e0 as *const ()), // sub_1407a53e0
    Replacement::draft(0x1407a5590, "content_1407a5590", fn_1407a5590 as *const ()), // sub_1407a5590
    Replacement::draft_throws(0x1407a5700, "content_1407a5700", fn_1407a5700 as *const (), Throws::Caught), // sub_1407a5700
    Replacement::draft_throws(0x1407a59d0, "content_1407a59d0", fn_1407a59d0 as *const (), Throws::Caught), // sub_1407a59d0
    Replacement::draft_throws(0x1407a5c10, "content_1407a5c10", fn_1407a5c10 as *const (), Throws::Caught), // sub_1407a5c10
    Replacement::draft_throws(0x1407a5f00, "content_1407a5f00", fn_1407a5f00 as *const (), Throws::Discard), // sub_1407a5f00
    Replacement::draft_throws(0x1407a60d0, "content_1407a60d0", fn_1407a60d0 as *const (), Throws::Caught), // sub_1407a60d0
    Replacement::draft_throws(0x1407a6640, "content_1407a6640", fn_1407a6640 as *const (), Throws::Caught), // sub_1407a6640
    Replacement::draft_throws(0x1407a6b40, "content_1407a6b40", fn_1407a6b40 as *const (), Throws::Caught), // sub_1407a6b40
    Replacement::draft(0x1407a7040, "content_1407a7040", fn_1407a7040 as *const ()), // sub_1407a7040
    Replacement::draft(0x1407a7180, "content_1407a7180", fn_1407a7180 as *const ()), // sub_1407a7180
    Replacement::draft(0x1407a72b0, "content_1407a72b0", fn_1407a72b0 as *const ()), // sub_1407a72b0
    Replacement::draft_throws(0x1407a73c0, "content_1407a73c0", fn_1407a73c0 as *const (), Throws::Discard), // sub_1407a73c0
    Replacement::draft_throws(0x1407a7500, "content_1407a7500", fn_1407a7500 as *const (), Throws::Discard), // sub_1407a7500
    Replacement::draft(0x1407a76d0, "content_1407a76d0", fn_1407a76d0 as *const ()), // sub_1407a76d0
    Replacement::draft(0x1407a7780, "content_1407a7780", fn_1407a7780 as *const ()), // sub_1407a7780
    Replacement::draft(0x1407a7850, "content_1407a7850", fn_1407a7850 as *const ()), // sub_1407a7850
    Replacement::draft(0x1407a7910, "content_1407a7910", tpl_8318::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x38, 0x14325c220, 0x8, 0x20, 0x28> as *const ()), // sub_1407a7910
    Replacement::draft(0x1407a79b0, "content_1407a79b0", tpl_8319::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325c220, 0x24, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407a79b0
    Replacement::draft(0x1407a7a60, "content_1407a7a60", tpl_6766::<0x14387d178, 0x14243b0b0, 0x1407c0230, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407a7a60
    Replacement::draft(0x1407a7ad0, "content_1407a7ad0", fn_1407a7ad0 as *const ()), // sub_1407a7ad0
    Replacement::draft_throws(0x1407a7bb0, "content_1407a7bb0", fn_1407a7bb0 as *const (), Throws::Discard), // sub_1407a7bb0
    Replacement::draft_throws(0x1407a7ca0, "content_1407a7ca0", fn_1407a7ca0 as *const (), Throws::Discard), // sub_1407a7ca0
    Replacement::draft_throws(0x1407a7d30, "content_1407a7d30", fn_1407a7d30 as *const (), Throws::Discard), // sub_1407a7d30
    Replacement::draft_throws(0x1407a7e30, "content_1407a7e30", fn_1407a7e30 as *const (), Throws::Caught), // sub_1407a7e30
    Replacement::draft_throws(0x1407a8080, "content_1407a8080", fn_1407a8080 as *const (), Throws::Caught), // sub_1407a8080
    Replacement::draft_throws(0x1407a82a0, "content_1407a82a0", fn_1407a82a0 as *const (), Throws::Caught), // sub_1407a82a0
    Replacement::draft(0x1407a84b0, "content_1407a84b0", fn_1407a84b0 as *const ()), // sub_1407a84b0
    Replacement::draft(0x1407a8540, "content_1407a8540", fn_1407a8540 as *const ()), // sub_1407a8540
    Replacement::draft(0x1407a8600, "content_1407a8600", fn_1407a8600 as *const ()), // sub_1407a8600
    Replacement::draft_throws(0x1407a86a0, "content_1407a86a0", fn_1407a86a0 as *const (), Throws::Discard), // sub_1407a86a0
    Replacement::draft_throws(0x1407a8770, "content_1407a8770", fn_1407a8770 as *const (), Throws::Discard), // sub_1407a8770
    Replacement::draft(0x1407a88a0, "content_1407a88a0", tpl_7526::<0x1407b5410, 0x14384e5a0, 0x38, 0x1426cdcb0, 0x14325e838, 0x1401de410> as *const ()), // sub_1407a88a0
    Replacement::draft(0x1407a8910, "content_1407a8910", fn_1407a8910 as *const ()), // sub_1407a8910
    Replacement::draft(0x1407a8990, "content_1407a8990", tpl_7526::<0x1407b54e0, 0x14384e5a0, 0x38, 0x1426cdcb0, 0x14325e838, 0x1401de410> as *const ()), // sub_1407a8990
    Replacement::draft(0x1407a8a00, "content_1407a8a00", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1407caf30> as *const ()), // sub_1407a8a00
    Replacement::draft(0x1407a8a90, "content_1407a8a90", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x1407caf30> as *const ()), // sub_1407a8a90
    Replacement::draft(0x1407a8b40, "content_1407a8b40", fn_1407a8b40 as *const ()), // sub_1407a8b40
    Replacement::draft(0x1407a8c40, "content_1407a8c40", fn_1407a8c40 as *const ()), // sub_1407a8c40
    Replacement::draft_throws(0x1407a8db0, "content_1407a8db0", fn_1407a8db0 as *const (), Throws::Discard), // sub_1407a8db0
    Replacement::draft_throws(0x1407a8f30, "content_1407a8f30", fn_1407a8f30 as *const (), Throws::Discard), // sub_1407a8f30
    Replacement::draft_throws(0x1407a9010, "content_1407a9010", fn_1407a9010 as *const (), Throws::Discard), // sub_1407a9010
    Replacement::draft_throws(0x1407a91c0, "content_1407a91c0", fn_1407a91c0 as *const (), Throws::Discard), // sub_1407a91c0
    Replacement::draft_throws(0x1407a92d0, "content_1407a92d0", fn_1407a92d0 as *const (), Throws::Caught), // sub_1407a92d0
    Replacement::draft_throws(0x1407a95d0, "content_1407a95d0", fn_1407a95d0 as *const (), Throws::Caught), // sub_1407a95d0
    Replacement::draft_throws(0x1407a98a0, "content_1407a98a0", fn_1407a98a0 as *const (), Throws::Caught), // sub_1407a98a0
    Replacement::draft(0x1407a9b60, "content_1407a9b60", fn_1407a9b60 as *const ()), // sub_1407a9b60
    Replacement::draft(0x1407a9c60, "content_1407a9c60", fn_1407a9c60 as *const ()), // sub_1407a9c60
    Replacement::draft(0x1407a9d80, "content_1407a9d80", fn_1407a9d80 as *const ()), // sub_1407a9d80
    Replacement::draft_throws(0x1407a9e80, "content_1407a9e80", fn_1407a9e80 as *const (), Throws::Discard), // sub_1407a9e80
    Replacement::draft_throws(0x1407a9fb0, "content_1407a9fb0", fn_1407a9fb0 as *const (), Throws::Discard), // sub_1407a9fb0
    Replacement::draft(0x1407aa180, "content_1407aa180", tpl_8347::<0x1407b5410, 0x14384e500, 0xbc, 0x1426cdcb0, 0x1432510f0, 0x1401de410> as *const ()), // sub_1407aa180
    Replacement::draft(0x1407aa1f0, "content_1407aa1f0", fn_1407aa1f0 as *const ()), // sub_1407aa1f0
    Replacement::draft(0x1407aa2c0, "content_1407aa2c0", tpl_8347::<0x1407b54e0, 0x14384e500, 0xbc, 0x1426cdcb0, 0x1432510f0, 0x1401de410> as *const ()), // sub_1407aa2c0
    Replacement::draft(0x1407aa330, "content_1407aa330", tpl_8318::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x38, 0x14325a328, 0x8, 0x20, 0x28> as *const ()), // sub_1407aa330
    Replacement::draft(0x1407aa3d0, "content_1407aa3d0", tpl_8319::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325a328, 0x24, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407aa3d0
    Replacement::draft(0x1407aa480, "content_1407aa480", fn_1407aa480 as *const ()), // sub_1407aa480
    Replacement::draft(0x1407aa500, "content_1407aa500", fn_1407aa500 as *const ()), // sub_1407aa500
    Replacement::draft_throws(0x1407aa5c0, "content_1407aa5c0", fn_1407aa5c0 as *const (), Throws::Discard), // sub_1407aa5c0
    Replacement::draft_throws(0x1407aa690, "content_1407aa690", fn_1407aa690 as *const (), Throws::Discard), // sub_1407aa690
    Replacement::draft_throws(0x1407aa720, "content_1407aa720", fn_1407aa720 as *const (), Throws::Discard), // sub_1407aa720
    Replacement::draft_throws(0x1407aa800, "content_1407aa800", fn_1407aa800 as *const (), Throws::Caught), // sub_1407aa800
    Replacement::draft_throws(0x1407aa8e0, "content_1407aa8e0", fn_1407aa8e0 as *const (), Throws::Caught), // sub_1407aa8e0
    Replacement::draft_throws(0x1407aa9c0, "content_1407aa9c0", fn_1407aa9c0 as *const (), Throws::Caught), // sub_1407aa9c0
    Replacement::draft(0x1407aaaa0, "content_1407aaaa0", fn_1407aaaa0 as *const ()), // sub_1407aaaa0
    Replacement::draft(0x1407aab30, "content_1407aab30", fn_1407aab30 as *const ()), // sub_1407aab30
    Replacement::draft(0x1407aabe0, "content_1407aabe0", fn_1407aabe0 as *const ()), // sub_1407aabe0
    Replacement::draft_throws(0x1407aac70, "content_1407aac70", fn_1407aac70 as *const (), Throws::Discard), // sub_1407aac70
    Replacement::draft_throws(0x1407aad30, "content_1407aad30", fn_1407aad30 as *const (), Throws::Discard), // sub_1407aad30
    Replacement::draft(0x1407aae40, "content_1407aae40", tpl_8318::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x38, 0x143258e68, 0x8, 0x20, 0x28> as *const ()), // sub_1407aae40
    Replacement::draft(0x1407aaee0, "content_1407aaee0", tpl_8319::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143258e68, 0x24, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407aaee0
    Replacement::draft(0x1407ab000, "content_1407ab000", fn_1407ab000 as *const ()), // sub_1407ab000
    Replacement::draft_throws(0x1407ab0b0, "content_1407ab0b0", fn_1407ab0b0 as *const (), Throws::Discard), // sub_1407ab0b0
    Replacement::draft_throws(0x1407ab170, "content_1407ab170", tpl_7852::<0x14387d080, 0x14243b0b0, 0x1407a3ed0, 0x338, 0x38, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1407ab170
    Replacement::draft_throws(0x1407ab1f0, "content_1407ab1f0", fn_1407ab1f0 as *const (), Throws::Discard), // sub_1407ab1f0
    Replacement::draft_throws(0x1407ab2b0, "content_1407ab2b0", fn_1407ab2b0 as *const (), Throws::Caught), // sub_1407ab2b0
    Replacement::draft_throws(0x1407ab380, "content_1407ab380", fn_1407ab380 as *const (), Throws::Caught), // sub_1407ab380
    Replacement::draft_throws(0x1407ab450, "content_1407ab450", fn_1407ab450 as *const (), Throws::Caught), // sub_1407ab450
    Replacement::draft(0x1407ab520, "content_1407ab520", tpl_7855::<0x14387d080, 0x14243b0b0, 0x1407a4120, 0x338, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ab520
    Replacement::draft(0x1407ab5a0, "content_1407ab5a0", tpl_8363::<0x14387d078, 0x14387d080, 0x1401e9970, 0x14387d0c0, 0x14387d1c8, 0x1407b4c60, 0x1401e9860, 0x338, 0x38, 0xe8> as *const ()), // sub_1407ab5a0
    Replacement::draft(0x1407ab640, "content_1407ab640", tpl_7857::<0x14387d080, 0x14243b0b0, 0x1407a4190, 0x338, 0x38, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ab640
    Replacement::draft_throws(0x1407ab6c0, "content_1407ab6c0", fn_1407ab6c0 as *const (), Throws::Discard), // sub_1407ab6c0
    Replacement::draft_throws(0x1407ab770, "content_1407ab770", fn_1407ab770 as *const (), Throws::Discard), // sub_1407ab770
    Replacement::draft(0x1407ab870, "content_1407ab870", fn_1407ab870 as *const ()), // sub_1407ab870
    Replacement::draft(0x1407ab900, "content_1407ab900", fn_1407ab900 as *const ()), // sub_1407ab900
    Replacement::draft(0x1407ab9b0, "content_1407ab9b0", fn_1407ab9b0 as *const ()), // sub_1407ab9b0
    Replacement::draft(0x1407abb30, "content_1407abb30", fn_1407abb30 as *const ()), // sub_1407abb30
    Replacement::draft_throws(0x1407abc80, "content_1407abc80", fn_1407abc80 as *const (), Throws::Discard), // sub_1407abc80
    Replacement::draft_throws(0x1407abe00, "content_1407abe00", fn_1407abe00 as *const (), Throws::Discard), // sub_1407abe00
    Replacement::draft_throws(0x1407abef0, "content_1407abef0", fn_1407abef0 as *const (), Throws::Discard), // sub_1407abef0
    Replacement::draft_throws(0x1407ac080, "content_1407ac080", fn_1407ac080 as *const (), Throws::Discard), // sub_1407ac080
    Replacement::draft_throws(0x1407ac210, "content_1407ac210", fn_1407ac210 as *const (), Throws::Caught), // sub_1407ac210
    Replacement::draft_throws(0x1407ac620, "content_1407ac620", fn_1407ac620 as *const (), Throws::Caught), // sub_1407ac620
    Replacement::draft_throws(0x1407ac9c0, "content_1407ac9c0", fn_1407ac9c0 as *const (), Throws::Caught), // sub_1407ac9c0
    Replacement::draft(0x1407acd50, "content_1407acd50", fn_1407acd50 as *const ()), // sub_1407acd50
    Replacement::draft(0x1407ace70, "content_1407ace70", fn_1407ace70 as *const ()), // sub_1407ace70
    Replacement::draft(0x1407acfa0, "content_1407acfa0", fn_1407acfa0 as *const ()), // sub_1407acfa0
    Replacement::draft_throws(0x1407ad0b0, "content_1407ad0b0", fn_1407ad0b0 as *const (), Throws::Discard), // sub_1407ad0b0
    Replacement::draft_throws(0x1407ad1e0, "content_1407ad1e0", fn_1407ad1e0 as *const (), Throws::Discard), // sub_1407ad1e0
    Replacement::draft(0x1407ad380, "content_1407ad380", tpl_67183::<0x1407b5410, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410, 0x14384e500, 0x40, 0x1426cdcb0, 0x1432510f0> as *const ()), // sub_1407ad380
    Replacement::draft(0x1407ad420, "content_1407ad420", tpl_67183::<0x1407b54e0, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410, 0x14384e500, 0x40, 0x1426cdcb0, 0x1432510f0> as *const ()), // sub_1407ad420
    Replacement::draft(0x1407ad4c0, "content_1407ad4c0", fn_1407ad4c0 as *const ()), // sub_1407ad4c0
    Replacement::draft(0x1407ad550, "content_1407ad550", fn_1407ad550 as *const ()), // sub_1407ad550
    Replacement::draft(0x1407ad600, "content_1407ad600", fn_1407ad600 as *const ()), // sub_1407ad600
    Replacement::draft(0x1407ad740, "content_1407ad740", fn_1407ad740 as *const ()), // sub_1407ad740
    Replacement::draft_throws(0x1407ad990, "content_1407ad990", fn_1407ad990 as *const (), Throws::Discard), // sub_1407ad990
    Replacement::draft_throws(0x1407adc30, "content_1407adc30", fn_1407adc30 as *const (), Throws::Discard), // sub_1407adc30
    Replacement::draft_throws(0x1407addb0, "content_1407addb0", fn_1407addb0 as *const (), Throws::Discard), // sub_1407addb0
    Replacement::draft_throws(0x1407ae0d0, "content_1407ae0d0", fn_1407ae0d0 as *const (), Throws::Discard), // sub_1407ae0d0
    Replacement::draft_throws(0x1407ae2a0, "content_1407ae2a0", fn_1407ae2a0 as *const (), Throws::Caught), // sub_1407ae2a0
    Replacement::draft_throws(0x1407ae860, "content_1407ae860", fn_1407ae860 as *const (), Throws::Caught), // sub_1407ae860
    Replacement::draft_throws(0x1407aed70, "content_1407aed70", fn_1407aed70 as *const (), Throws::Caught), // sub_1407aed70
    Replacement::draft(0x1407af270, "content_1407af270", fn_1407af270 as *const ()), // sub_1407af270
    Replacement::draft(0x1407af410, "content_1407af410", fn_1407af410 as *const ()), // sub_1407af410
    Replacement::draft(0x1407af5d0, "content_1407af5d0", fn_1407af5d0 as *const ()), // sub_1407af5d0
    Replacement::draft_throws(0x1407af770, "content_1407af770", fn_1407af770 as *const (), Throws::Discard), // sub_1407af770
    Replacement::draft_throws(0x1407af970, "content_1407af970", fn_1407af970 as *const (), Throws::Discard), // sub_1407af970
    Replacement::draft(0x1407afc70, "content_1407afc70", tpl_8397::<0x1407b5410, 0x14384e500, 0x1b0, 0x1426cdcb0, 0x1432510f0, 0x1401de410, 0x14384e4e0, 0x1b4, 0x14325e4f0> as *const ()), // sub_1407afc70
    Replacement::draft(0x1407afd10, "content_1407afd10", fn_1407afd10 as *const ()), // sub_1407afd10
    Replacement::draft(0x1407afe40, "content_1407afe40", tpl_8397::<0x1407b54e0, 0x14384e500, 0x1b0, 0x1426cdcb0, 0x1432510f0, 0x1401de410, 0x14384e4e0, 0x1b4, 0x14325e4f0> as *const ()), // sub_1407afe40
    Replacement::draft(0x1407afee0, "content_1407afee0", fn_1407afee0 as *const ()), // sub_1407afee0
    Replacement::draft(0x1407aff70, "content_1407aff70", fn_1407aff70 as *const ()), // sub_1407aff70
    Replacement::draft(0x1407b0020, "content_1407b0020", fn_1407b0020 as *const ()), // sub_1407b0020
    Replacement::draft(0x1407b0110, "content_1407b0110", fn_1407b0110 as *const ()), // sub_1407b0110
    Replacement::draft_throws(0x1407b0290, "content_1407b0290", fn_1407b0290 as *const (), Throws::Discard), // sub_1407b0290
    Replacement::draft_throws(0x1407b0420, "content_1407b0420", fn_1407b0420 as *const (), Throws::Discard), // sub_1407b0420
    Replacement::draft_throws(0x1407b0500, "content_1407b0500", fn_1407b0500 as *const (), Throws::Discard), // sub_1407b0500
    Replacement::draft_throws(0x1407b06b0, "content_1407b06b0", fn_1407b06b0 as *const (), Throws::Discard), // sub_1407b06b0
    Replacement::draft_throws(0x1407b07e0, "content_1407b07e0", fn_1407b07e0 as *const (), Throws::Caught), // sub_1407b07e0
    Replacement::draft_throws(0x1407b0d40, "content_1407b0d40", fn_1407b0d40 as *const (), Throws::Caught), // sub_1407b0d40
    Replacement::draft_throws(0x1407b11d0, "content_1407b11d0", fn_1407b11d0 as *const (), Throws::Caught), // sub_1407b11d0
    Replacement::draft(0x1407b1660, "content_1407b1660", fn_1407b1660 as *const ()), // sub_1407b1660
    Replacement::draft(0x1407b1750, "content_1407b1750", fn_1407b1750 as *const ()), // sub_1407b1750
    Replacement::draft(0x1407b1880, "content_1407b1880", fn_1407b1880 as *const ()), // sub_1407b1880
    Replacement::draft_throws(0x1407b1990, "content_1407b1990", fn_1407b1990 as *const (), Throws::Discard), // sub_1407b1990
    Replacement::draft_throws(0x1407b1ac0, "content_1407b1ac0", fn_1407b1ac0 as *const (), Throws::Discard), // sub_1407b1ac0
    Replacement::draft(0x1407b1c90, "content_1407b1c90", tpl_8414::<0x1407b5410, 0x14384e558, 0x38, 0x14325e6f0, 0x2, 0x1401de410, 0x14384e4e0, 0x3c, 0x14325e4f0, 0x3, 0x14384e500, 0x48, 0x1426cdcb0, 0x1432510f0> as *const ()), // sub_1407b1c90
    Replacement::draft(0x1407b1d60, "content_1407b1d60", fn_1407b1d60 as *const ()), // sub_1407b1d60
    Replacement::draft(0x1407b1e50, "content_1407b1e50", tpl_8414::<0x1407b54e0, 0x14384e558, 0x38, 0x14325e6f0, 0x2, 0x1401de410, 0x14384e4e0, 0x3c, 0x14325e4f0, 0x3, 0x14384e500, 0x48, 0x1426cdcb0, 0x1432510f0> as *const ()), // sub_1407b1e50
    Replacement::draft(0x1407b1f20, "content_1407b1f20", fn_1407b1f20 as *const ()), // sub_1407b1f20
    Replacement::draft(0x1407b2040, "content_1407b2040", fn_1407b2040 as *const ()), // sub_1407b2040
    Replacement::draft(0x1407b2180, "content_1407b2180", fn_1407b2180 as *const ()), // sub_1407b2180
    Replacement::draft(0x1407b2250, "content_1407b2250", fn_1407b2250 as *const ()), // sub_1407b2250
    Replacement::draft_throws(0x1407b2410, "content_1407b2410", fn_1407b2410 as *const (), Throws::Discard), // sub_1407b2410
    Replacement::draft_throws(0x1407b25b0, "content_1407b25b0", fn_1407b25b0 as *const (), Throws::Discard), // sub_1407b25b0
    Replacement::draft_throws(0x1407b26a0, "content_1407b26a0", fn_1407b26a0 as *const (), Throws::Discard), // sub_1407b26a0
    Replacement::draft_throws(0x1407b2880, "content_1407b2880", fn_1407b2880 as *const (), Throws::Discard), // sub_1407b2880
    Replacement::draft_throws(0x1407b2960, "content_1407b2960", fn_1407b2960 as *const (), Throws::Caught), // sub_1407b2960
    Replacement::draft_throws(0x1407b2da0, "content_1407b2da0", fn_1407b2da0 as *const (), Throws::Caught), // sub_1407b2da0
    Replacement::draft_throws(0x1407b3150, "content_1407b3150", fn_1407b3150 as *const (), Throws::Caught), // sub_1407b3150
    Replacement::draft(0x1407b34f0, "content_1407b34f0", fn_1407b34f0 as *const ()), // sub_1407b34f0
    Replacement::draft(0x1407b35f0, "content_1407b35f0", fn_1407b35f0 as *const ()), // sub_1407b35f0
    Replacement::draft(0x1407b3730, "content_1407b3730", fn_1407b3730 as *const ()), // sub_1407b3730
    Replacement::draft_throws(0x1407b3850, "content_1407b3850", fn_1407b3850 as *const (), Throws::Discard), // sub_1407b3850
    Replacement::draft_throws(0x1407b39a0, "content_1407b39a0", fn_1407b39a0 as *const (), Throws::Discard), // sub_1407b39a0
    Replacement::draft(0x1407b3b70, "content_1407b3b70", tpl_67234::<0x1407b5410, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410, 0x14384e500, 0x44, 0x1426cdcb0, 0x1432510f0> as *const ()), // sub_1407b3b70
    Replacement::draft(0x1407b3c00, "content_1407b3c00", fn_1407b3c00 as *const ()), // sub_1407b3c00
    Replacement::draft(0x1407b3cf0, "content_1407b3cf0", tpl_67234::<0x1407b54e0, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410, 0x14384e500, 0x44, 0x1426cdcb0, 0x1432510f0> as *const ()), // sub_1407b3cf0
    Replacement::draft(0x1407b3d80, "content_1407b3d80", tpl_8433::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143259680, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_1407b3d80
    Replacement::draft(0x1407b3e20, "content_1407b3e20", tpl_8434::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143259680, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407b3e20
    Replacement::draft(0x1407b3ed0, "content_1407b3ed0", fn_1407b3ed0 as *const ()), // sub_1407b3ed0
    Replacement::draft(0x1407b4020, "content_1407b4020", fn_1407b4020 as *const ()), // sub_1407b4020
    Replacement::draft_throws(0x1407b41d0, "content_1407b41d0", fn_1407b41d0 as *const (), Throws::Discard), // sub_1407b41d0
    Replacement::draft_throws(0x1407b4350, "content_1407b4350", fn_1407b4350 as *const (), Throws::Discard), // sub_1407b4350
    Replacement::draft_throws(0x1407b44c0, "content_1407b44c0", fn_1407b44c0 as *const (), Throws::Discard), // sub_1407b44c0
    Replacement::draft_throws(0x1407b4640, "content_1407b4640", fn_1407b4640 as *const (), Throws::Discard), // sub_1407b4640
    Replacement::draft_throws(0x1407b47e0, "content_1407b47e0", fn_1407b47e0 as *const (), Throws::Caught), // sub_1407b47e0
    Replacement::draft_throws(0x1407b48c0, "content_1407b48c0", fn_1407b48c0 as *const (), Throws::Caught), // sub_1407b48c0
    Replacement::draft_throws(0x1407b49a0, "content_1407b49a0", fn_1407b49a0 as *const (), Throws::Caught), // sub_1407b49a0
    Replacement::draft(0x1407b4a80, "content_1407b4a80", fn_1407b4a80 as *const ()), // sub_1407b4a80
    Replacement::draft(0x1407b4c60, "content_1407b4c60", fn_1407b4c60 as *const ()), // sub_1407b4c60
    Replacement::draft(0x1407b4ea0, "content_1407b4ea0", fn_1407b4ea0 as *const ()), // sub_1407b4ea0
    Replacement::draft_throws(0x1407b5100, "content_1407b5100", fn_1407b5100 as *const (), Throws::Discard), // sub_1407b5100
    Replacement::draft_throws(0x1407b5310, "content_1407b5310", fn_1407b5310 as *const (), Throws::Discard), // sub_1407b5310
    Replacement::draft(0x1407b5410, "content_1407b5410", fn_1407b5410 as *const ()), // sub_1407b5410
    Replacement::draft(0x1407b54e0, "content_1407b54e0", fn_1407b54e0 as *const ()), // sub_1407b54e0
    Replacement::draft(0x1407b55b0, "content_1407b55b0", tpl_8318::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x38, 0x1432595b8, 0x8, 0x20, 0x28> as *const ()), // sub_1407b55b0
    Replacement::draft(0x1407b5650, "content_1407b5650", tpl_8319::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432595b8, 0x24, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407b5650
    Replacement::draft(0x1407b5770, "content_1407b5770", tpl_8096::<0x14387d108, 0x14243b0b0, 0x1407a3e60, 0x38, 0x1426d4580, 0x338, 0x39, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b5770
    Replacement::draft_throws(0x1407b5820, "content_1407b5820", fn_1407b5820 as *const (), Throws::Discard), // sub_1407b5820
    Replacement::draft_throws(0x1407b5910, "content_1407b5910", tpl_8021::<0x14387d108, 0x14243b0b0, 0x1407a3ed0, 0x338, 0x38, 0x80, 0x39, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1407b5910
    Replacement::draft_throws(0x1407b59a0, "content_1407b59a0", fn_1407b59a0 as *const (), Throws::Discard), // sub_1407b59a0
    Replacement::draft_throws(0x1407b5aa0, "content_1407b5aa0", fn_1407b5aa0 as *const (), Throws::Caught), // sub_1407b5aa0
    Replacement::draft_throws(0x1407b5bb0, "content_1407b5bb0", fn_1407b5bb0 as *const (), Throws::Caught), // sub_1407b5bb0
    Replacement::draft_throws(0x1407b5cc0, "content_1407b5cc0", fn_1407b5cc0 as *const (), Throws::Caught), // sub_1407b5cc0
    Replacement::draft(0x1407b5dd0, "content_1407b5dd0", fn_1407b5dd0 as *const ()), // sub_1407b5dd0
    Replacement::draft(0x1407b5e60, "content_1407b5e60", tpl_8449::<0x14387d048, 0x14387d108, 0x1401e9970, 0x14387d0c0, 0x14387d1c8, 0x1407b4c60, 0x1401e9860, 0x338, 0x38, 0x120, 0x39> as *const ()), // sub_1407b5e60
    Replacement::draft(0x1407b5f20, "content_1407b5f20", tpl_8027::<0x14387d108, 0x14243b0b0, 0x1407a4190, 0x338, 0x38, 0x120, 0x39, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b5f20
    Replacement::draft_throws(0x1407b5fb0, "content_1407b5fb0", fn_1407b5fb0 as *const (), Throws::Discard), // sub_1407b5fb0
    Replacement::draft_throws(0x1407b6070, "content_1407b6070", fn_1407b6070 as *const (), Throws::Discard), // sub_1407b6070
    Replacement::draft(0x1407b61a0, "content_1407b61a0", fn_1407b61a0 as *const ()), // sub_1407b61a0
    Replacement::draft(0x1407b6250, "content_1407b6250", fn_1407b6250 as *const ()), // sub_1407b6250
    Replacement::draft(0x1407b6310, "content_1407b6310", tpl_7312::<0x14387d060, 0x14243b0b0, 0x1407a3df0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b6310
    Replacement::draft(0x1407b6380, "content_1407b6380", fn_1407b6380 as *const ()), // sub_1407b6380
    Replacement::draft_throws(0x1407b64c0, "content_1407b64c0", fn_1407b64c0 as *const (), Throws::Discard), // sub_1407b64c0
    Replacement::draft_throws(0x1407b65d0, "content_1407b65d0", fn_1407b65d0 as *const (), Throws::Discard), // sub_1407b65d0
    Replacement::draft_throws(0x1407b66a0, "content_1407b66a0", fn_1407b66a0 as *const (), Throws::Discard), // sub_1407b66a0
    Replacement::draft_throws(0x1407b6770, "content_1407b6770", fn_1407b6770 as *const (), Throws::Discard), // sub_1407b6770
    Replacement::draft_throws(0x1407b67f0, "content_1407b67f0", fn_1407b67f0 as *const (), Throws::Caught), // sub_1407b67f0
    Replacement::draft_throws(0x1407b68e0, "content_1407b68e0", fn_1407b68e0 as *const (), Throws::Caught), // sub_1407b68e0
    Replacement::draft_throws(0x1407b69d0, "content_1407b69d0", fn_1407b69d0 as *const (), Throws::Caught), // sub_1407b69d0
    Replacement::draft(0x1407b6ac0, "content_1407b6ac0", fn_1407b6ac0 as *const ()), // sub_1407b6ac0
    Replacement::draft(0x1407b6b80, "content_1407b6b80", fn_1407b6b80 as *const ()), // sub_1407b6b80
    Replacement::draft(0x1407b6c60, "content_1407b6c60", fn_1407b6c60 as *const ()), // sub_1407b6c60
    Replacement::draft_throws(0x1407b6d20, "content_1407b6d20", fn_1407b6d20 as *const (), Throws::Discard), // sub_1407b6d20
    Replacement::draft_throws(0x1407b6e30, "content_1407b6e30", fn_1407b6e30 as *const (), Throws::Discard), // sub_1407b6e30
    Replacement::draft(0x1407b6f50, "content_1407b6f50", fn_1407b6f50 as *const ()), // sub_1407b6f50
    Replacement::draft(0x1407b6ff0, "content_1407b6ff0", fn_1407b6ff0 as *const ()), // sub_1407b6ff0
    Replacement::draft(0x1407b70b0, "content_1407b70b0", tpl_7312::<0x14387d0b0, 0x14243b0b0, 0x1407b3ed0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b70b0
    Replacement::draft(0x1407b7120, "content_1407b7120", tpl_8466::<0x14387d0b0, 0x14243b0b0, 0x1407b4020, 0x38, 0x1406f29a0, 0x40, 0x14072d0c0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b7120
    Replacement::draft_throws(0x1407b71a0, "content_1407b71a0", tpl_67268::<0x14387d04c, 0x14387d0b0, 0x1401e9fa0, 0x1407b41d0, 0x38, 0x1406d7200, 0x40, 0x140727c80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407b71a0
    Replacement::draft_throws(0x1407b7220, "content_1407b7220", fn_1407b7220 as *const (), Throws::Discard), // sub_1407b7220
    Replacement::draft_throws(0x1407b72a0, "content_1407b72a0", tpl_67268::<0x14387d04c, 0x14387d0b0, 0x1401e9b60, 0x1407b44c0, 0x38, 0x1406d7330, 0x40, 0x140727db0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407b72a0
    Replacement::draft_throws(0x1407b7320, "content_1407b7320", fn_1407b7320 as *const (), Throws::Caught), // sub_1407b7320
    Replacement::draft_throws(0x1407b7440, "content_1407b7440", fn_1407b7440 as *const (), Throws::Caught), // sub_1407b7440
    Replacement::draft_throws(0x1407b7560, "content_1407b7560", fn_1407b7560 as *const (), Throws::Caught), // sub_1407b7560
    Replacement::draft(0x1407b7680, "content_1407b7680", fn_1407b7680 as *const ()), // sub_1407b7680
    Replacement::draft(0x1407b7720, "content_1407b7720", fn_1407b7720 as *const ()), // sub_1407b7720
    Replacement::draft(0x1407b77a0, "content_1407b77a0", tpl_8466::<0x14387d0b0, 0x14243b0b0, 0x1407b4ea0, 0x38, 0x1406f3300, 0x40, 0x14072e7f0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b77a0
    Replacement::draft_throws(0x1407b7820, "content_1407b7820", fn_1407b7820 as *const (), Throws::Discard), // sub_1407b7820
    Replacement::draft_throws(0x1407b78a0, "content_1407b78a0", fn_1407b78a0 as *const (), Throws::Discard), // sub_1407b78a0
    Replacement::draft(0x1407b7960, "content_1407b7960", fn_1407b7960 as *const ()), // sub_1407b7960
    Replacement::draft(0x1407b7a20, "content_1407b7a20", fn_1407b7a20 as *const ()), // sub_1407b7a20
    Replacement::draft(0x1407b7a90, "content_1407b7a90", tpl_8296::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14325a260, 0x20, 0x28, 0x30> as *const ()), // sub_1407b7a90
    Replacement::draft(0x1407b7b30, "content_1407b7b30", tpl_8297::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325a260, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_1407b7b30
    Replacement::draft_throws(0x1407b7d30, "content_1407b7d30", fn_1407b7d30 as *const (), Throws::Caught), // sub_1407b7d30
    Replacement::draft_throws(0x1407b7dd0, "content_1407b7dd0", fn_1407b7dd0 as *const (), Throws::Caught), // sub_1407b7dd0
    Replacement::draft_throws(0x1407b7e70, "content_1407b7e70", fn_1407b7e70 as *const (), Throws::Caught), // sub_1407b7e70
    Replacement::draft_throws(0x1407b7ff0, "content_1407b7ff0", tpl_7326::<0x14387d0a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1407b5310> as *const (), Throws::Discard), // sub_1407b7ff0
    Replacement::draft(0x1407b8090, "content_1407b8090", fn_1407b8090 as *const ()), // sub_1407b8090
    Replacement::draft(0x1407b8170, "content_1407b8170", fn_1407b8170 as *const ()), // sub_1407b8170
    Replacement::draft(0x1407b8310, "content_1407b8310", tpl_8478::<0x14387d158, 0x14243b0b0, 0x1407c02a0, 0x3b, 0x40, 0x3c, 0x38, 0x338, 0x1401ee7c0, 0xc0, 0x1426d4580, 0x1> as *const ()), // sub_1407b8310
    Replacement::draft_throws(0x1407b8400, "content_1407b8400", fn_1407b8400 as *const (), Throws::Discard), // sub_1407b8400
    Replacement::draft_throws(0x1407b8520, "content_1407b8520", tpl_7898::<0x14387d158, 0x14243b0b0, 0x1407c0310, 0x338, 0x38, 0x20, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1407b8520
    Replacement::draft_throws(0x1407b85b0, "content_1407b85b0", fn_1407b85b0 as *const (), Throws::Discard), // sub_1407b85b0
    Replacement::draft_throws(0x1407b86e0, "content_1407b86e0", fn_1407b86e0 as *const (), Throws::Discard), // sub_1407b86e0
    Replacement::draft_throws(0x1407b8740, "content_1407b8740", fn_1407b8740 as *const (), Throws::Caught), // sub_1407b8740
    Replacement::draft_throws(0x1407b89e0, "content_1407b89e0", fn_1407b89e0 as *const (), Throws::Caught), // sub_1407b89e0
    Replacement::draft_throws(0x1407b8c60, "content_1407b8c60", fn_1407b8c60 as *const (), Throws::Caught), // sub_1407b8c60
    Replacement::draft(0x1407b8ed0, "content_1407b8ed0", fn_1407b8ed0 as *const ()), // sub_1407b8ed0
    Replacement::draft(0x1407b8f90, "content_1407b8f90", fn_1407b8f90 as *const ()), // sub_1407b8f90
    Replacement::draft(0x1407b9050, "content_1407b9050", fn_1407b9050 as *const ()), // sub_1407b9050
    Replacement::draft_throws(0x1407b90f0, "content_1407b90f0", fn_1407b90f0 as *const (), Throws::Discard), // sub_1407b90f0
    Replacement::draft_throws(0x1407b91b0, "content_1407b91b0", fn_1407b91b0 as *const (), Throws::Discard), // sub_1407b91b0
    Replacement::draft(0x1407b92e0, "content_1407b92e0", tpl_8318::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x38, 0x143259428, 0x8, 0x20, 0x28> as *const ()), // sub_1407b92e0
    Replacement::draft(0x1407b9380, "content_1407b9380", tpl_8319::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143259428, 0x24, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407b9380
    Replacement::draft(0x1407b94a0, "content_1407b94a0", tpl_8125::<0x14387d0e8, 0x14243b0b0, 0x1407a3e60, 0x38, 0x338, 0x1426d4580, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b94a0
    Replacement::draft_throws(0x1407b9540, "content_1407b9540", fn_1407b9540 as *const (), Throws::Discard), // sub_1407b9540
    Replacement::draft_throws(0x1407b9600, "content_1407b9600", tpl_7852::<0x14387d0e8, 0x14243b0b0, 0x1407a3ed0, 0x338, 0x38, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1407b9600
    Replacement::draft_throws(0x1407b9680, "content_1407b9680", fn_1407b9680 as *const (), Throws::Discard), // sub_1407b9680
    Replacement::draft_throws(0x1407b9740, "content_1407b9740", fn_1407b9740 as *const (), Throws::Caught), // sub_1407b9740
    Replacement::draft_throws(0x1407b9810, "content_1407b9810", fn_1407b9810 as *const (), Throws::Caught), // sub_1407b9810
    Replacement::draft_throws(0x1407b98e0, "content_1407b98e0", fn_1407b98e0 as *const (), Throws::Caught), // sub_1407b98e0
    Replacement::draft(0x1407b99b0, "content_1407b99b0", tpl_8129::<0x14387d0e8, 0x14243b0b0, 0x1407a4120, 0x338, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b99b0
    Replacement::draft(0x1407b9a30, "content_1407b9a30", tpl_8363::<0x14387d1f8, 0x14387d0e8, 0x1401e9970, 0x14387d0c0, 0x14387d1c8, 0x1407b4c60, 0x1401e9860, 0x338, 0x38, 0x120> as *const ()), // sub_1407b9a30
    Replacement::draft(0x1407b9ad0, "content_1407b9ad0", tpl_7857::<0x14387d0e8, 0x14243b0b0, 0x1407a4190, 0x338, 0x38, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407b9ad0
    Replacement::draft_throws(0x1407b9b50, "content_1407b9b50", fn_1407b9b50 as *const (), Throws::Discard), // sub_1407b9b50
    Replacement::draft_throws(0x1407b9c00, "content_1407b9c00", fn_1407b9c00 as *const (), Throws::Discard), // sub_1407b9c00
    Replacement::draft(0x1407b9d00, "content_1407b9d00", tpl_8296::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14325aec0, 0x20, 0x28, 0x30> as *const ()), // sub_1407b9d00
    Replacement::draft(0x1407b9da0, "content_1407b9da0", tpl_8297::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325aec0, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_1407b9da0
    Replacement::draft_throws(0x1407ba0c0, "content_1407ba0c0", fn_1407ba0c0 as *const (), Throws::Caught), // sub_1407ba0c0
    Replacement::draft_throws(0x1407ba160, "content_1407ba160", fn_1407ba160 as *const (), Throws::Caught), // sub_1407ba160
    Replacement::draft_throws(0x1407ba200, "content_1407ba200", fn_1407ba200 as *const (), Throws::Caught), // sub_1407ba200
    Replacement::draft_throws(0x1407ba4a0, "content_1407ba4a0", tpl_8493::<0x14387d030, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d1c8, 0x1407b5310> as *const (), Throws::Discard), // sub_1407ba4a0
    Replacement::draft(0x1407ba570, "content_1407ba570", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14324f508, 0x8> as *const ()), // sub_1407ba570
    Replacement::draft(0x1407ba5f0, "content_1407ba5f0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324f508> as *const ()), // sub_1407ba5f0
    Replacement::draft(0x1407ba680, "content_1407ba680", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1407cb560> as *const ()), // sub_1407ba680
    Replacement::draft(0x1407ba710, "content_1407ba710", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x1407cb560> as *const ()), // sub_1407ba710
    Replacement::draft(0x1407ba7c0, "content_1407ba7c0", fn_1407ba7c0 as *const ()), // sub_1407ba7c0
    Replacement::draft(0x1407ba8e0, "content_1407ba8e0", fn_1407ba8e0 as *const ()), // sub_1407ba8e0
    Replacement::draft_throws(0x1407baa90, "content_1407baa90", fn_1407baa90 as *const (), Throws::Discard), // sub_1407baa90
    Replacement::draft_throws(0x1407bac40, "content_1407bac40", fn_1407bac40 as *const (), Throws::Discard), // sub_1407bac40
    Replacement::draft_throws(0x1407bad50, "content_1407bad50", fn_1407bad50 as *const (), Throws::Discard), // sub_1407bad50
    Replacement::draft_throws(0x1407baf40, "content_1407baf40", fn_1407baf40 as *const (), Throws::Discard), // sub_1407baf40
    Replacement::draft_throws(0x1407bb060, "content_1407bb060", fn_1407bb060 as *const (), Throws::Caught), // sub_1407bb060
    Replacement::draft_throws(0x1407bb260, "content_1407bb260", fn_1407bb260 as *const (), Throws::Caught), // sub_1407bb260
    Replacement::draft_throws(0x1407bb460, "content_1407bb460", fn_1407bb460 as *const (), Throws::Caught), // sub_1407bb460
    Replacement::draft(0x1407bb660, "content_1407bb660", fn_1407bb660 as *const ()), // sub_1407bb660
    Replacement::draft(0x1407bb790, "content_1407bb790", fn_1407bb790 as *const ()), // sub_1407bb790
    Replacement::draft(0x1407bb8d0, "content_1407bb8d0", fn_1407bb8d0 as *const ()), // sub_1407bb8d0
    Replacement::draft_throws(0x1407bb9f0, "content_1407bb9f0", fn_1407bb9f0 as *const (), Throws::Discard), // sub_1407bb9f0
    Replacement::draft_throws(0x1407bbb60, "content_1407bbb60", fn_1407bbb60 as *const (), Throws::Discard), // sub_1407bbb60
    Replacement::draft(0x1407bbd50, "content_1407bbd50", fn_1407bbd50 as *const ()), // sub_1407bbd50
    Replacement::draft(0x1407bbe60, "content_1407bbe60", fn_1407bbe60 as *const ()), // sub_1407bbe60
    Replacement::draft(0x1407bbef0, "content_1407bbef0", fn_1407bbef0 as *const ()), // sub_1407bbef0
    Replacement::draft(0x1407bbfa0, "content_1407bbfa0", fn_1407bbfa0 as *const ()), // sub_1407bbfa0
    Replacement::draft(0x1407bc1a0, "content_1407bc1a0", fn_1407bc1a0 as *const ()), // sub_1407bc1a0
    Replacement::draft_throws(0x1407bc450, "content_1407bc450", fn_1407bc450 as *const (), Throws::Discard), // sub_1407bc450
    Replacement::draft_throws(0x1407bc6f0, "content_1407bc6f0", fn_1407bc6f0 as *const (), Throws::Discard), // sub_1407bc6f0
    Replacement::draft_throws(0x1407bc8a0, "content_1407bc8a0", fn_1407bc8a0 as *const (), Throws::Discard), // sub_1407bc8a0
    Replacement::draft_throws(0x1407bcb90, "content_1407bcb90", fn_1407bcb90 as *const (), Throws::Discard), // sub_1407bcb90
    Replacement::draft_throws(0x1407bcd40, "content_1407bcd40", fn_1407bcd40 as *const (), Throws::Caught), // sub_1407bcd40
    Replacement::draft_throws(0x1407bd020, "content_1407bd020", fn_1407bd020 as *const (), Throws::Caught), // sub_1407bd020
    Replacement::draft_throws(0x1407bd300, "content_1407bd300", fn_1407bd300 as *const (), Throws::Caught), // sub_1407bd300
    Replacement::draft(0x1407bd5e0, "content_1407bd5e0", fn_1407bd5e0 as *const ()), // sub_1407bd5e0
    Replacement::draft(0x1407bd7d0, "content_1407bd7d0", fn_1407bd7d0 as *const ()), // sub_1407bd7d0
    Replacement::draft(0x1407bd9b0, "content_1407bd9b0", fn_1407bd9b0 as *const ()), // sub_1407bd9b0
    Replacement::draft_throws(0x1407bdb70, "content_1407bdb70", fn_1407bdb70 as *const (), Throws::Discard), // sub_1407bdb70
    Replacement::draft_throws(0x1407bdda0, "content_1407bdda0", fn_1407bdda0 as *const (), Throws::Discard), // sub_1407bdda0
    Replacement::draft(0x1407be060, "content_1407be060", fn_1407be060 as *const ()), // sub_1407be060
    Replacement::draft(0x1407be1b0, "content_1407be1b0", tpl_8318::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x38, 0x14325d180, 0x8, 0x20, 0x28> as *const ()), // sub_1407be1b0
    Replacement::draft(0x1407be250, "content_1407be250", tpl_8319::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325d180, 0x24, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1407be250
    Replacement::draft(0x1407be370, "content_1407be370", tpl_8019::<0x14387d1f0, 0x14243b0b0, 0x1407c02a0, 0x338, 0x38, 0x1426d4580, 0x1, 0x3c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407be370
    Replacement::draft_throws(0x1407be450, "content_1407be450", fn_1407be450 as *const (), Throws::Discard), // sub_1407be450
    Replacement::draft_throws(0x1407be540, "content_1407be540", fn_1407be540 as *const (), Throws::Discard), // sub_1407be540
    Replacement::draft_throws(0x1407be5d0, "content_1407be5d0", fn_1407be5d0 as *const (), Throws::Discard), // sub_1407be5d0
    Replacement::draft_throws(0x1407be6e0, "content_1407be6e0", fn_1407be6e0 as *const (), Throws::Caught), // sub_1407be6e0
    Replacement::draft_throws(0x1407be810, "content_1407be810", fn_1407be810 as *const (), Throws::Caught), // sub_1407be810
    Replacement::draft_throws(0x1407be940, "content_1407be940", fn_1407be940 as *const (), Throws::Caught), // sub_1407be940
    Replacement::draft(0x1407bea70, "content_1407bea70", fn_1407bea70 as *const ()), // sub_1407bea70
    Replacement::draft(0x1407beb00, "content_1407beb00", tpl_8449::<0x14387d1d0, 0x14387d1f0, 0x1401e9970, 0x14387d050, 0x14387d160, 0x1407b4c60, 0x1401e9860, 0x338, 0x38, 0xd8, 0x3c> as *const ()), // sub_1407beb00
    Replacement::draft(0x1407bebc0, "content_1407bebc0", tpl_8027::<0x14387d1f0, 0x14243b0b0, 0x1407c05d0, 0x338, 0x38, 0xd8, 0x3c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407bebc0
    Replacement::draft_throws(0x1407bec50, "content_1407bec50", fn_1407bec50 as *const (), Throws::Discard), // sub_1407bec50
    Replacement::draft_throws(0x1407bed20, "content_1407bed20", fn_1407bed20 as *const (), Throws::Discard), // sub_1407bed20
    Replacement::draft(0x1407bee60, "content_1407bee60", fn_1407bee60 as *const ()), // sub_1407bee60
    Replacement::draft(0x1407beef0, "content_1407beef0", fn_1407beef0 as *const ()), // sub_1407beef0
    Replacement::draft(0x1407bef80, "content_1407bef80", fn_1407bef80 as *const ()), // sub_1407bef80
    Replacement::draft(0x1407bf030, "content_1407bf030", fn_1407bf030 as *const ()), // sub_1407bf030
    Replacement::draft(0x1407bf160, "content_1407bf160", fn_1407bf160 as *const ()), // sub_1407bf160
    Replacement::draft_throws(0x1407bf290, "content_1407bf290", fn_1407bf290 as *const (), Throws::Discard), // sub_1407bf290
    Replacement::draft_throws(0x1407bf3f0, "content_1407bf3f0", fn_1407bf3f0 as *const (), Throws::Discard), // sub_1407bf3f0
    Replacement::draft_throws(0x1407bf4e0, "content_1407bf4e0", fn_1407bf4e0 as *const (), Throws::Discard), // sub_1407bf4e0
    Replacement::draft_throws(0x1407bf650, "content_1407bf650", fn_1407bf650 as *const (), Throws::Discard), // sub_1407bf650
    Replacement::draft_throws(0x1407bf740, "content_1407bf740", fn_1407bf740 as *const (), Throws::Caught), // sub_1407bf740
    Replacement::draft_throws(0x1407bf890, "content_1407bf890", fn_1407bf890 as *const (), Throws::Caught), // sub_1407bf890
    Replacement::draft_throws(0x1407bf9e0, "content_1407bf9e0", fn_1407bf9e0 as *const (), Throws::Caught), // sub_1407bf9e0
    Replacement::draft(0x1407bfb30, "content_1407bfb30", fn_1407bfb30 as *const ()), // sub_1407bfb30
    Replacement::draft(0x1407bfc40, "content_1407bfc40", fn_1407bfc40 as *const ()), // sub_1407bfc40
    Replacement::draft(0x1407bfd60, "content_1407bfd60", fn_1407bfd60 as *const ()), // sub_1407bfd60
    Replacement::draft_throws(0x1407bfe50, "content_1407bfe50", fn_1407bfe50 as *const (), Throws::Discard), // sub_1407bfe50
    Replacement::draft_throws(0x1407bff80, "content_1407bff80", fn_1407bff80 as *const (), Throws::Discard), // sub_1407bff80
    Replacement::draft(0x1407c00e0, "content_1407c00e0", tpl_8296::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14325cba8, 0x20, 0x28, 0x30> as *const ()), // sub_1407c00e0
    Replacement::draft(0x1407c0180, "content_1407c0180", tpl_8297::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14325cba8, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_1407c0180
    Replacement::draft_throws(0x1407c0380, "content_1407c0380", fn_1407c0380 as *const (), Throws::Caught), // sub_1407c0380
    Replacement::draft_throws(0x1407c0420, "content_1407c0420", fn_1407c0420 as *const (), Throws::Caught), // sub_1407c0420
    Replacement::draft_throws(0x1407c04c0, "content_1407c04c0", fn_1407c04c0 as *const (), Throws::Caught), // sub_1407c04c0
    Replacement::draft_throws(0x1407c0640, "content_1407c0640", tpl_7326::<0x14387d160, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1407b5310> as *const (), Throws::Discard), // sub_1407c0640
    Replacement::draft(0x1407c06e0, "content_1407c06e0", fn_1407c06e0 as *const ()), // sub_1407c06e0
    Replacement::draft(0x1407c0770, "content_1407c0770", fn_1407c0770 as *const ()), // sub_1407c0770
    Replacement::draft(0x1407c0820, "content_1407c0820", fn_1407c0820 as *const ()), // sub_1407c0820
    Replacement::draft(0x1407c0950, "content_1407c0950", fn_1407c0950 as *const ()), // sub_1407c0950
    Replacement::draft_throws(0x1407c0a50, "content_1407c0a50", fn_1407c0a50 as *const (), Throws::Discard), // sub_1407c0a50
    Replacement::draft_throws(0x1407c0b80, "content_1407c0b80", fn_1407c0b80 as *const (), Throws::Discard), // sub_1407c0b80
    Replacement::draft_throws(0x1407c0c40, "content_1407c0c40", fn_1407c0c40 as *const (), Throws::Discard), // sub_1407c0c40
    Replacement::draft_throws(0x1407c0d90, "content_1407c0d90", fn_1407c0d90 as *const (), Throws::Discard), // sub_1407c0d90
    Replacement::draft_throws(0x1407c0ee0, "content_1407c0ee0", fn_1407c0ee0 as *const (), Throws::Caught), // sub_1407c0ee0
    Replacement::draft_throws(0x1407c1050, "content_1407c1050", fn_1407c1050 as *const (), Throws::Caught), // sub_1407c1050
    Replacement::draft_throws(0x1407c11c0, "content_1407c11c0", fn_1407c11c0 as *const (), Throws::Caught), // sub_1407c11c0
    Replacement::draft(0x1407c1330, "content_1407c1330", fn_1407c1330 as *const ()), // sub_1407c1330
    Replacement::draft(0x1407c1410, "content_1407c1410", fn_1407c1410 as *const ()), // sub_1407c1410
    Replacement::draft(0x1407c1500, "content_1407c1500", fn_1407c1500 as *const ()), // sub_1407c1500
    Replacement::draft_throws(0x1407c15c0, "content_1407c15c0", fn_1407c15c0 as *const (), Throws::Discard), // sub_1407c15c0
    Replacement::draft_throws(0x1407c16c0, "content_1407c16c0", fn_1407c16c0 as *const (), Throws::Discard), // sub_1407c16c0
    Replacement::draft(0x1407c1840, "content_1407c1840", fn_1407c1840 as *const ()), // sub_1407c1840
    Replacement::draft(0x1407c1920, "content_1407c1920", fn_1407c1920 as *const ()), // sub_1407c1920
    Replacement::draft(0x1407c1a10, "content_1407c1a10", fn_1407c1a10 as *const ()), // sub_1407c1a10
    Replacement::draft(0x1407c1aa0, "content_1407c1aa0", fn_1407c1aa0 as *const ()), // sub_1407c1aa0
    Replacement::draft_throws(0x1407c1b90, "content_1407c1b90", fn_1407c1b90 as *const (), Throws::Caught), // sub_1407c1b90
    Replacement::draft_throws(0x1407c1eb0, "content_1407c1eb0", fn_1407c1eb0 as *const (), Throws::Discard), // sub_1407c1eb0
    Replacement::draft_throws(0x1407c1f60, "content_1407c1f60", fn_1407c1f60 as *const (), Throws::Caught), // sub_1407c1f60
    Replacement::draft_throws(0x1407c2290, "content_1407c2290", fn_1407c2290 as *const (), Throws::Discard), // sub_1407c2290
    Replacement::draft_throws(0x1407c2330, "content_1407c2330", fn_1407c2330 as *const (), Throws::Caught), // sub_1407c2330
    Replacement::draft_throws(0x1407c2470, "content_1407c2470", fn_1407c2470 as *const (), Throws::Caught), // sub_1407c2470
    Replacement::draft_throws(0x1407c25b0, "content_1407c25b0", fn_1407c25b0 as *const (), Throws::Caught), // sub_1407c25b0
    Replacement::draft(0x1407c26f0, "content_1407c26f0", fn_1407c26f0 as *const ()), // sub_1407c26f0
    Replacement::draft(0x1407c27c0, "content_1407c27c0", fn_1407c27c0 as *const ()), // sub_1407c27c0
    Replacement::draft(0x1407c28a0, "content_1407c28a0", fn_1407c28a0 as *const ()), // sub_1407c28a0
    Replacement::draft_throws(0x1407c2960, "content_1407c2960", fn_1407c2960 as *const (), Throws::Discard), // sub_1407c2960
    Replacement::draft_throws(0x1407c2a50, "content_1407c2a50", fn_1407c2a50 as *const (), Throws::Caught), // sub_1407c2a50
    Replacement::draft(0x1407c2d70, "content_1407c2d70", tpl_8569::<0x1407b5410, 0x40, 0x14325e390, 0x1, 0x1407ce8a0, 0x58, 0x14325e378> as *const ()), // sub_1407c2d70
    Replacement::draft(0x1407c2de0, "content_1407c2de0", tpl_8569::<0x1407b54e0, 0x40, 0x14325e390, 0x1, 0x1407ce8c0, 0x58, 0x14325e378> as *const ()), // sub_1407c2de0
    Replacement::draft(0x1407c2e50, "content_1407c2e50", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1407cbaa0> as *const ()), // sub_1407c2e50
    Replacement::draft(0x1407c2ee0, "content_1407c2ee0", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x1407cbaa0> as *const ()), // sub_1407c2ee0
    Replacement::draft(0x1407c2f90, "content_1407c2f90", fn_1407c2f90 as *const ()), // sub_1407c2f90
    Replacement::draft(0x1407c3150, "content_1407c3150", fn_1407c3150 as *const ()), // sub_1407c3150
    Replacement::draft_throws(0x1407c3370, "content_1407c3370", fn_1407c3370 as *const (), Throws::Discard), // sub_1407c3370
    Replacement::draft_throws(0x1407c3590, "content_1407c3590", fn_1407c3590 as *const (), Throws::Discard), // sub_1407c3590
    Replacement::draft_throws(0x1407c3720, "content_1407c3720", fn_1407c3720 as *const (), Throws::Discard), // sub_1407c3720
    Replacement::draft_throws(0x1407c3970, "content_1407c3970", fn_1407c3970 as *const (), Throws::Discard), // sub_1407c3970
    Replacement::draft_throws(0x1407c3a90, "content_1407c3a90", fn_1407c3a90 as *const (), Throws::Caught), // sub_1407c3a90
    Replacement::draft_throws(0x1407c3c30, "content_1407c3c30", fn_1407c3c30 as *const (), Throws::Caught), // sub_1407c3c30
    Replacement::draft_throws(0x1407c3dd0, "content_1407c3dd0", fn_1407c3dd0 as *const (), Throws::Caught), // sub_1407c3dd0
    Replacement::draft(0x1407c3f70, "content_1407c3f70", fn_1407c3f70 as *const ()), // sub_1407c3f70
    Replacement::draft(0x1407c4120, "content_1407c4120", fn_1407c4120 as *const ()), // sub_1407c4120
    Replacement::draft(0x1407c42e0, "content_1407c42e0", fn_1407c42e0 as *const ()), // sub_1407c42e0
    Replacement::draft_throws(0x1407c4480, "content_1407c4480", fn_1407c4480 as *const (), Throws::Discard), // sub_1407c4480
    Replacement::draft_throws(0x1407c4660, "content_1407c4660", fn_1407c4660 as *const (), Throws::Discard), // sub_1407c4660
    Replacement::draft(0x1407c4810, "content_1407c4810", fn_1407c4810 as *const ()), // sub_1407c4810
    Replacement::draft(0x1407c48d0, "content_1407c48d0", fn_1407c48d0 as *const ()), // sub_1407c48d0
    Replacement::draft(0x1407c49f0, "content_1407c49f0", fn_1407c49f0 as *const ()), // sub_1407c49f0
    Replacement::draft(0x1407c4b30, "content_1407c4b30", fn_1407c4b30 as *const ()), // sub_1407c4b30
    Replacement::draft(0x1407c4db0, "content_1407c4db0", tpl_8583::<0x14387d1a8, 0x14243b0b0, 0x1407c02a0, 0x14075bb30, 0x1406d7a10, 0x1406fc6d0> as *const ()), // sub_1407c4db0
    Replacement::draft_throws(0x1407c5000, "content_1407c5000", tpl_8584::<0x14387d094, 0x14387d1a8, 0x1401e9fa0, 0x14387d050, 0x14387d160, 0x1407b41d0, 0x1401e9e70, 0x14075bb30, 0x1406d7a10, 0x1406d96a0> as *const (), Throws::Discard), // sub_1407c5000
    Replacement::draft_throws(0x1407c5280, "content_1407c5280", fn_1407c5280 as *const (), Throws::Discard), // sub_1407c5280
    Replacement::draft_throws(0x1407c54d0, "content_1407c54d0", tpl_8584::<0x14387d094, 0x14387d1a8, 0x1401e9b60, 0x14387d050, 0x14387d160, 0x1407b44c0, 0x1401e9a20, 0x14075bb30, 0x1406d7a10, 0x1406d97d0> as *const (), Throws::Discard), // sub_1407c54d0
    Replacement::draft_throws(0x1407c5750, "content_1407c5750", fn_1407c5750 as *const (), Throws::Discard), // sub_1407c5750
    Replacement::draft_throws(0x1407c57d0, "content_1407c57d0", fn_1407c57d0 as *const (), Throws::Caught), // sub_1407c57d0
    Replacement::draft_throws(0x1407c5890, "content_1407c5890", fn_1407c5890 as *const (), Throws::Caught), // sub_1407c5890
    Replacement::draft_throws(0x1407c5950, "content_1407c5950", fn_1407c5950 as *const (), Throws::Caught), // sub_1407c5950
    Replacement::draft(0x1407c5a10, "content_1407c5a10", fn_1407c5a10 as *const ()), // sub_1407c5a10
    Replacement::draft(0x1407c5c70, "content_1407c5c70", fn_1407c5c70 as *const ()), // sub_1407c5c70
    Replacement::draft(0x1407c5ee0, "content_1407c5ee0", tpl_8583::<0x14387d1a8, 0x14243b0b0, 0x1407c05d0, 0x14075bb30, 0x1406d7a10, 0x1406fcfb0> as *const ()), // sub_1407c5ee0
    Replacement::draft_throws(0x1407c6130, "content_1407c6130", fn_1407c6130 as *const (), Throws::Discard), // sub_1407c6130
    Replacement::draft_throws(0x1407c63a0, "content_1407c63a0", fn_1407c63a0 as *const (), Throws::Discard), // sub_1407c63a0
    Replacement::draft(0x1407c6480, "content_1407c6480", fn_1407c6480 as *const ()), // sub_1407c6480
    Replacement::draft(0x1407c6530, "content_1407c6530", fn_1407c6530 as *const ()), // sub_1407c6530
    Replacement::draft(0x1407c65f0, "content_1407c65f0", fn_1407c65f0 as *const ()), // sub_1407c65f0
    Replacement::draft(0x1407c66d0, "content_1407c66d0", fn_1407c66d0 as *const ()), // sub_1407c66d0
    Replacement::draft_throws(0x1407c68a0, "content_1407c68a0", fn_1407c68a0 as *const (), Throws::Caught), // sub_1407c68a0
    Replacement::draft_throws(0x1407c6ac0, "content_1407c6ac0", fn_1407c6ac0 as *const (), Throws::Discard), // sub_1407c6ac0
    Replacement::draft_throws(0x1407c6c30, "content_1407c6c30", fn_1407c6c30 as *const (), Throws::Discard), // sub_1407c6c30
    Replacement::draft_throws(0x1407c6e20, "content_1407c6e20", fn_1407c6e20 as *const (), Throws::Discard), // sub_1407c6e20
    Replacement::draft_throws(0x1407c6ea0, "content_1407c6ea0", fn_1407c6ea0 as *const (), Throws::Caught), // sub_1407c6ea0
    Replacement::draft_throws(0x1407c72b0, "content_1407c72b0", fn_1407c72b0 as *const (), Throws::Caught), // sub_1407c72b0
    Replacement::draft_throws(0x1407c7630, "content_1407c7630", fn_1407c7630 as *const (), Throws::Caught), // sub_1407c7630
    Replacement::draft(0x1407c79c0, "content_1407c79c0", fn_1407c79c0 as *const ()), // sub_1407c79c0
    Replacement::draft(0x1407c7bc0, "content_1407c7bc0", fn_1407c7bc0 as *const ()), // sub_1407c7bc0
    Replacement::draft(0x1407c7d50, "content_1407c7d50", fn_1407c7d50 as *const ()), // sub_1407c7d50
    Replacement::draft_throws(0x1407c7ec0, "content_1407c7ec0", fn_1407c7ec0 as *const (), Throws::Discard), // sub_1407c7ec0
    Replacement::draft_throws(0x1407c8060, "content_1407c8060", fn_1407c8060 as *const (), Throws::Discard), // sub_1407c8060
    Replacement::draft(0x1407c8220, "content_1407c8220", tpl_7496::<0x1407b5410, 0x14384e578, 0x40, 0x1426cdcb0, 0x14325e2f8, 0x1401de410> as *const ()), // sub_1407c8220
    Replacement::draft(0x1407c8290, "content_1407c8290", fn_1407c8290 as *const ()), // sub_1407c8290
    Replacement::draft(0x1407c8310, "content_1407c8310", tpl_7496::<0x1407b54e0, 0x14384e578, 0x40, 0x1426cdcb0, 0x14325e2f8, 0x1401de410> as *const ()), // sub_1407c8310
    Replacement::draft(0x1407c8380, "content_1407c8380", fn_1407c8380 as *const ()), // sub_1407c8380
    Replacement::draft(0x1407c8420, "content_1407c8420", fn_1407c8420 as *const ()), // sub_1407c8420
    Replacement::draft(0x1407c84e0, "content_1407c84e0", fn_1407c84e0 as *const ()), // sub_1407c84e0
    Replacement::draft(0x1407c8580, "content_1407c8580", fn_1407c8580 as *const ()), // sub_1407c8580
    Replacement::draft_throws(0x1407c86d0, "content_1407c86d0", fn_1407c86d0 as *const (), Throws::Discard), // sub_1407c86d0
    Replacement::draft_throws(0x1407c8820, "content_1407c8820", fn_1407c8820 as *const (), Throws::Discard), // sub_1407c8820
    Replacement::draft_throws(0x1407c88f0, "content_1407c88f0", fn_1407c88f0 as *const (), Throws::Discard), // sub_1407c88f0
    Replacement::draft_throws(0x1407c8a50, "content_1407c8a50", fn_1407c8a50 as *const (), Throws::Discard), // sub_1407c8a50
    Replacement::draft_throws(0x1407c8ab0, "content_1407c8ab0", fn_1407c8ab0 as *const (), Throws::Caught), // sub_1407c8ab0
    Replacement::draft_throws(0x1407c8d30, "content_1407c8d30", fn_1407c8d30 as *const (), Throws::Caught), // sub_1407c8d30
    Replacement::draft_throws(0x1407c8f80, "content_1407c8f80", fn_1407c8f80 as *const (), Throws::Caught), // sub_1407c8f80
    Replacement::draft(0x1407c91d0, "content_1407c91d0", fn_1407c91d0 as *const ()), // sub_1407c91d0
    Replacement::draft(0x1407c92a0, "content_1407c92a0", fn_1407c92a0 as *const ()), // sub_1407c92a0
    Replacement::draft(0x1407c93b0, "content_1407c93b0", fn_1407c93b0 as *const ()), // sub_1407c93b0
    Replacement::draft_throws(0x1407c94a0, "content_1407c94a0", fn_1407c94a0 as *const (), Throws::Discard), // sub_1407c94a0
    Replacement::draft_throws(0x1407c95c0, "content_1407c95c0", fn_1407c95c0 as *const (), Throws::Discard), // sub_1407c95c0
    Replacement::draft(0x1407c9720, "content_1407c9720", fn_1407c9720 as *const ()), // sub_1407c9720
    Replacement::draft(0x1407c97c0, "content_1407c97c0", fn_1407c97c0 as *const ()), // sub_1407c97c0
    Replacement::draft(0x1407c9860, "content_1407c9860", fn_1407c9860 as *const ()), // sub_1407c9860
    Replacement::draft(0x1407c9920, "content_1407c9920", fn_1407c9920 as *const ()), // sub_1407c9920
    Replacement::draft(0x1407c99f0, "content_1407c99f0", fn_1407c99f0 as *const ()), // sub_1407c99f0
    Replacement::draft_throws(0x1407c9b70, "content_1407c9b70", fn_1407c9b70 as *const (), Throws::Discard), // sub_1407c9b70
    Replacement::draft_throws(0x1407c9cf0, "content_1407c9cf0", fn_1407c9cf0 as *const (), Throws::Discard), // sub_1407c9cf0
    Replacement::draft_throws(0x1407c9e00, "content_1407c9e00", fn_1407c9e00 as *const (), Throws::Discard), // sub_1407c9e00
    Replacement::draft_throws(0x1407c9fa0, "content_1407c9fa0", fn_1407c9fa0 as *const (), Throws::Discard), // sub_1407c9fa0
    Replacement::draft_throws(0x1407ca010, "content_1407ca010", fn_1407ca010 as *const (), Throws::Caught), // sub_1407ca010
    Replacement::draft_throws(0x1407ca170, "content_1407ca170", fn_1407ca170 as *const (), Throws::Caught), // sub_1407ca170
    Replacement::draft_throws(0x1407ca2d0, "content_1407ca2d0", fn_1407ca2d0 as *const (), Throws::Caught), // sub_1407ca2d0
    Replacement::draft(0x1407ca430, "content_1407ca430", fn_1407ca430 as *const ()), // sub_1407ca430
    Replacement::draft(0x1407ca540, "content_1407ca540", fn_1407ca540 as *const ()), // sub_1407ca540
    Replacement::draft(0x1407ca670, "content_1407ca670", fn_1407ca670 as *const ()), // sub_1407ca670
    Replacement::draft_throws(0x1407ca780, "content_1407ca780", fn_1407ca780 as *const (), Throws::Discard), // sub_1407ca780
    Replacement::draft_throws(0x1407ca8d0, "content_1407ca8d0", fn_1407ca8d0 as *const (), Throws::Discard), // sub_1407ca8d0
    Replacement::draft(0x1407caa50, "content_1407caa50", fn_1407caa50 as *const ()), // sub_1407caa50
    Replacement::draft(0x1407caae0, "content_1407caae0", fn_1407caae0 as *const ()), // sub_1407caae0
    Replacement::draft_throws(0x1407cabd0, "content_1407cabd0", fn_1407cabd0 as *const (), Throws::Discard), // sub_1407cabd0
    Replacement::draft(0x1407cac50, "content_1407cac50", fn_1407cac50 as *const ()), // content::BehaviorTreeLoopNode::content::BehaviorTreeLoopNode::vfunc_0
    Replacement::draft(0x1407cacb0, "content_1407cacb0", tpl_8643::<0x1401ffd50, 0x18, 0x8, 0x14324ab18, 0x141834d30> as *const ()), // content::BehaviorTreeNode::content::BehaviorTreeNode::vfunc_0
    Replacement::draft(0x1407cad10, "content_1407cad10", fn_1407cad10 as *const ()), // sub_1407cad10
    Replacement::draft(0x1407caf30, "content_1407caf30", fn_1407caf30 as *const ()), // sub_1407caf30
    Replacement::draft(0x1407cb020, "content_1407cb020", fn_1407cb020 as *const ()), // sub_1407cb020
    Replacement::draft(0x1407cb1e0, "content_1407cb1e0", fn_1407cb1e0 as *const ()), // sub_1407cb1e0
    Replacement::draft(0x1407cb330, "content_1407cb330", fn_1407cb330 as *const ()), // sub_1407cb330
    Replacement::draft(0x1407cb420, "content_1407cb420", fn_1407cb420 as *const ()), // content::BehaviorTreeRandomSelectorNode::content::BehaviorTreeRandomSelectorNode::vfunc_0
    Replacement::draft(0x1407cb490, "content_1407cb490", fn_1407cb490 as *const ()), // content::BehaviorTreeWaitNode::content::BehaviorTreeWaitNode::vfunc_0
    Replacement::draft(0x1407cb4f0, "content_1407cb4f0", fn_1407cb4f0 as *const ()), // content::BehaviorTreeSayNode::content::BehaviorTreeSayNode::vfunc_0
    Replacement::draft(0x1407cb560, "content_1407cb560", fn_1407cb560 as *const ()), // sub_1407cb560
    Replacement::draft(0x1407cb650, "content_1407cb650", fn_1407cb650 as *const ()), // sub_1407cb650
    Replacement::draft(0x1407cb800, "content_1407cb800", fn_1407cb800 as *const ()), // sub_1407cb800
    Replacement::draft(0x1407cb8f0, "content_1407cb8f0", fn_1407cb8f0 as *const ()), // sub_1407cb8f0
    Replacement::draft(0x1407cba40, "content_1407cba40", fn_1407cba40 as *const ()), // content::BehaviorTreeTestNode::content::BehaviorTreeTestNode::vfunc_0
    Replacement::draft(0x1407cbaa0, "content_1407cbaa0", fn_1407cbaa0 as *const ()), // sub_1407cbaa0
    Replacement::draft(0x1407cbb90, "content_1407cbb90", fn_1407cbb90 as *const ()), // content::BehaviorTreeUseAbilityNode::content::BehaviorTreeUseAbilityNode::vfunc_0
    Replacement::draft(0x1407cbbf0, "content_1407cbbf0", tpl_8659::<0x1403b8050, 0xb8, 0x14240ab10, 0x88, 0x40, 0x28, 0x1401ffd50, 0x18, 0x8, 0x14324ab18, 0x141834d30> as *const ()), // content::BehaviorTreeAnimatedMoveToNode::content::BehaviorTreeAnimatedMoveToNode::vfunc_0
    Replacement::draft(0x1407cbc90, "content_1407cbc90", fn_1407cbc90 as *const ()), // content::BehaviorTreeEQSQueryNode::content::BehaviorTreeEQSQueryNode::vfunc_0
    Replacement::draft(0x1407cbd80, "content_1407cbd80", fn_1407cbd80 as *const ()), // content::BehaviorTreeFlyToNode::content::BehaviorTreeFlyToNode::vfunc_0
    Replacement::draft(0x1407cbe20, "content_1407cbe20", fn_1407cbe20 as *const ()), // content::BehaviorTreeMoveToCoverNode::content::BehaviorTreeMoveToCoverNode::vfunc_0
    Replacement::draft(0x1407cbef0, "content_1407cbef0", fn_1407cbef0 as *const ()), // content::BehaviorTreeMoveToFollow::content::BehaviorTreeMoveToFollow::vfunc_0
    Replacement::draft(0x1407cbfa0, "content_1407cbfa0", fn_1407cbfa0 as *const ()), // content::BehaviorTreeMoveToNode::content::BehaviorTreeMoveToNode::vfunc_0
    Replacement::draft(0x1407cc040, "content_1407cc040", fn_1407cc040 as *const ()), // content::BehaviorTreeMoveOnRouteNode::content::BehaviorTreeMoveOnRouteNode::vfunc_0
    Replacement::draft(0x1407cc0c0, "content_1407cc0c0", fn_1407cc0c0 as *const ()), // content::BehaviorTreeShootAtNode::content::BehaviorTreeShootAtNode::vfunc_0
    Replacement::draft(0x1407cc160, "content_1407cc160", fn_1407cc160 as *const ()), // content::BehaviorTreeShootFromCoverNode::content::BehaviorTreeShootFromCoverNode::vfunc_0
    Replacement::draft(0x1407cc230, "content_1407cc230", fn_1407cc230 as *const ()), // content::BehaviorTreeTakeCoverNode::content::BehaviorTreeTakeCoverNode::vfunc_0
    Replacement::draft(0x1407cc2d0, "content_1407cc2d0", fn_1407cc2d0 as *const ()), // content::BehaviorTreeTeleportNode::content::BehaviorTreeTeleportNode::vfunc_0
    Replacement::draft(0x1407cc380, "content_1407cc380", tpl_8659::<0x1403b8050, 0xb8, 0x14240ab10, 0x80, 0x38, 0x28, 0x1401ffd50, 0x18, 0x8, 0x14324ab18, 0x141834d30> as *const ()), // content::BehaviorTreeThrowGrenadeFromCoverNode::content::BehaviorTreeThrowGrenadeFromCoverNode::vfunc_0
    Replacement::draft(0x1407cc420, "content_1407cc420", fn_1407cc420 as *const ()), // content::BehaviorTreeThrowGrenadeNode::content::BehaviorTreeThrowGrenadeNode::vfunc_0
    Replacement::draft_throws(0x1407cc4a0, "content_1407cc4a0", fn_1407cc4a0 as *const (), Throws::Caught), // sub_1407cc4a0
    Replacement::draft_throws(0x1407cc8c0, "content_1407cc8c0", fn_1407cc8c0 as *const (), Throws::Caught), // sub_1407cc8c0
    Replacement::draft_throws(0x1407ccce0, "content_1407ccce0", fn_1407ccce0 as *const (), Throws::Caught), // sub_1407ccce0
    Replacement::draft_throws(0x1407cd100, "content_1407cd100", fn_1407cd100 as *const (), Throws::Caught), // sub_1407cd100
    Replacement::draft_throws(0x1407cd520, "content_1407cd520", fn_1407cd520 as *const (), Throws::Caught), // sub_1407cd520
    Replacement::draft_throws(0x1407cd940, "content_1407cd940", fn_1407cd940 as *const (), Throws::Caught), // sub_1407cd940
    Replacement::draft_throws(0x1407cdd60, "content_1407cdd60", fn_1407cdd60 as *const (), Throws::Caught), // sub_1407cdd60
    Replacement::draft_throws(0x1407ce060, "content_1407ce060", fn_1407ce060 as *const (), Throws::Caught), // sub_1407ce060
    Replacement::draft_throws(0x1407ce360, "content_1407ce360", fn_1407ce360 as *const (), Throws::Caught), // sub_1407ce360
    Replacement::draft_throws(0x1407ce760, "content_1407ce760", fn_1407ce760 as *const (), Throws::Caught), // sub_1407ce760
    Replacement::draft(0x1407ce8a0, "content_1407ce8a0", tpl_8671::<0x18> as *const ()), // sub_1407ce8a0
    Replacement::draft(0x1407ce8c0, "content_1407ce8c0", tpl_8671::<0x8> as *const ()), // sub_1407ce8c0
    Replacement::draft(0x1407ce8e0, "content_1407ce8e0", fn_1407ce8e0 as *const ()), // sub_1407ce8e0
    Replacement::draft_throws(0x1407ce960, "content_1407ce960", fn_1407ce960 as *const (), Throws::Caught), // sub_1407ce960
    Replacement::draft_throws(0x1407cec80, "content_1407cec80", fn_1407cec80 as *const (), Throws::Caught), // sub_1407cec80
    Replacement::draft_throws(0x1407cefa0, "content_1407cefa0", tpl_66193::<0x142442de0, 0x143a39a28, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1407cefa0
    Replacement::draft_throws(0x1407cf200, "content_1407cf200", tpl_66331::<0x142442de0, 0x143a398d8, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1407cf200
    Replacement::draft_throws(0x1407cf460, "content_1407cf460", fn_1407cf460 as *const (), Throws::Caught), // sub_1407cf460
    Replacement::draft_throws(0x1407cf780, "content_1407cf780", fn_1407cf780 as *const (), Throws::Caught), // sub_1407cf780
    Replacement::draft_throws(0x1407cfaa0, "content_1407cfaa0", fn_1407cfaa0 as *const (), Throws::Caught), // sub_1407cfaa0
    Replacement::draft_throws(0x1407cfd90, "content_1407cfd90", fn_1407cfd90 as *const (), Throws::Caught), // sub_1407cfd90
    Replacement::draft_throws(0x1407d0080, "content_1407d0080", fn_1407d0080 as *const (), Throws::Caught), // sub_1407d0080
    Replacement::draft_throws(0x1407d0340, "content_1407d0340", fn_1407d0340 as *const (), Throws::Caught), // sub_1407d0340
    Replacement::draft_throws(0x1407d0600, "content_1407d0600", fn_1407d0600 as *const (), Throws::Caught), // sub_1407d0600
    Replacement::draft_throws(0x1407d08c0, "content_1407d08c0", fn_1407d08c0 as *const (), Throws::Caught), // sub_1407d08c0
    Replacement::draft_throws(0x1407d0b80, "content_1407d0b80", fn_1407d0b80 as *const (), Throws::Caught), // sub_1407d0b80
    Replacement::draft_throws(0x1407d0c50, "content_1407d0c50", fn_1407d0c50 as *const (), Throws::Caught), // sub_1407d0c50
    Replacement::draft_throws(0x1407d0d20, "content_1407d0d20", fn_1407d0d20 as *const (), Throws::Caught), // sub_1407d0d20
    Replacement::draft_throws(0x1407d12d0, "content_1407d12d0", tpl_7080::<0x1401ffe00, 0x140708f60, 0x1407d7830, 0x142442de0, 0x143a39a28, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_1407d12d0
    Replacement::draft_throws(0x1407d1570, "content_1407d1570", tpl_7370::<0x1401ffe00, 0x140708f60, 0x1407d7750, 0x142442de0, 0x143a398d8, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_1407d1570
    Replacement::draft_throws(0x1407d1810, "content_1407d1810", fn_1407d1810 as *const (), Throws::Caught), // sub_1407d1810
    Replacement::draft_throws(0x1407d1ce0, "content_1407d1ce0", fn_1407d1ce0 as *const (), Throws::Caught), // sub_1407d1ce0
    Replacement::draft_throws(0x1407d21b0, "content_1407d21b0", fn_1407d21b0 as *const (), Throws::Caught), // sub_1407d21b0
    Replacement::draft_throws(0x1407d2630, "content_1407d2630", fn_1407d2630 as *const (), Throws::Caught), // sub_1407d2630
    Replacement::draft_throws(0x1407d2ab0, "content_1407d2ab0", fn_1407d2ab0 as *const (), Throws::Caught), // sub_1407d2ab0
    Replacement::draft_throws(0x1407d2f30, "content_1407d2f30", fn_1407d2f30 as *const (), Throws::Caught), // sub_1407d2f30
    Replacement::draft_throws(0x1407d33b0, "content_1407d33b0", fn_1407d33b0 as *const (), Throws::Caught), // sub_1407d33b0
    Replacement::draft_throws(0x1407d35b0, "content_1407d35b0", fn_1407d35b0 as *const (), Throws::Caught), // sub_1407d35b0
    Replacement::draft_throws(0x1407d37b0, "content_1407d37b0", fn_1407d37b0 as *const (), Throws::Caught), // sub_1407d37b0
    Replacement::draft_throws(0x1407d4270, "content_1407d4270", fn_1407d4270 as *const (), Throws::Caught), // sub_1407d4270
    Replacement::draft_throws(0x1407d4d30, "content_1407d4d30", fn_1407d4d30 as *const (), Throws::Caught), // sub_1407d4d30
    Replacement::draft_throws(0x1407d57c0, "content_1407d57c0", fn_1407d57c0 as *const (), Throws::Caught), // sub_1407d57c0
    Replacement::draft_throws(0x1407d6250, "content_1407d6250", fn_1407d6250 as *const (), Throws::Caught), // sub_1407d6250
    Replacement::draft_throws(0x1407d6cd0, "content_1407d6cd0", fn_1407d6cd0 as *const (), Throws::Caught), // sub_1407d6cd0
    Replacement::draft_throws(0x1407d7750, "content_1407d7750", tpl_65670::<0x142442de0, 0x143a398d8, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1407d7750
    Replacement::draft_throws(0x1407d7830, "content_1407d7830", tpl_65670::<0x142442de0, 0x143a39a28, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x70> as *const (), Throws::Discard), // sub_1407d7830
    Replacement::draft_throws(0x1407d7910, "content_1407d7910", fn_1407d7910 as *const (), Throws::Caught), // sub_1407d7910
    Replacement::draft_throws(0x1407d7a50, "content_1407d7a50", fn_1407d7a50 as *const (), Throws::Caught), // sub_1407d7a50
    Replacement::draft_throws(0x1407d8000, "content_1407d8000", fn_1407d8000 as *const (), Throws::Discard), // sub_1407d8000
    Replacement::draft(0x1407d8470, "content_1407d8470", tpl_8673::<0x3, 0x4, 0x8, 0x143242298, 0x142409b50> as *const ()), // content::AimService::content::AimService::vfunc_1
    Replacement::draft_throws(0x1407d84d0, "content_1407d84d0", tpl_6639::<0x143cc3498, 0x143cc34b8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AimService::content::AimService::vfunc_2
    Replacement::draft(0x1407d85f0, "content_1407d85f0", tpl_7388::<0x3, 0x4, 0x8, 0x143240218, 0x142409b50> as *const ()), // content::AnimationModeService::content::AnimationModeService::vfunc_1
    Replacement::draft_throws(0x1407d8650, "content_1407d8650", tpl_6639::<0x143cc348c, 0x143cc34a0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AnimationModeService::content::AnimationModeService::vfunc_2
    Replacement::draft(0x1407d8770, "content_1407d8770", tpl_7402::<0x3, 0x4, 0x8, 0x14323f9b8, 0x142409b50> as *const ()), // content::AvoidanceTypeService::content::AvoidanceTypeService::vfunc_1
    Replacement::draft_throws(0x1407d87d0, "content_1407d87d0", tpl_6639::<0x143cc3490, 0x143cc34c8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::AvoidanceTypeService::content::AvoidanceTypeService::vfunc_2
    Replacement::draft(0x1407d88f0, "content_1407d88f0", tpl_7386::<0x3, 0x4, 0x8, 0x14322eca8, 0x142409b50> as *const ()), // content::CountdownService::content::CountdownService::vfunc_1
    Replacement::draft_throws(0x1407d8950, "content_1407d8950", tpl_6639::<0x143cc34dc, 0x143cc3500, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::CountdownService::content::CountdownService::vfunc_2
    Replacement::draft(0x1407d8a70, "content_1407d8a70", tpl_7402::<0x3, 0x4, 0x8, 0x14322e830, 0x142409b50> as *const ()), // content::CoverService::content::CoverService::vfunc_1
    Replacement::draft_throws(0x1407d8ad0, "content_1407d8ad0", tpl_6639::<0x143cc34d8, 0x143cc3470, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::CoverService::content::CoverService::vfunc_2
    Replacement::draft(0x1407d8bf0, "content_1407d8bf0", tpl_7398::<0x3, 0x4, 0x8, 0x14322c9d8, 0x142409b50> as *const ()), // content::DisableAvoidanceService::content::DisableAvoidanceService::vfunc_1
    Replacement::draft_throws(0x1407d8c50, "content_1407d8c50", tpl_6639::<0x143cc34f0, 0x143cc3480, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DisableAvoidanceService::content::DisableAvoidanceService::vfunc_2
    Replacement::draft(0x1407d8d70, "content_1407d8d70", tpl_7387::<0x3, 0x4, 0x8, 0x143226c60, 0x142409b50> as *const ()), // content::FindWorkspotService::content::FindWorkspotService::vfunc_1
    Replacement::draft_throws(0x1407d8dd0, "content_1407d8dd0", tpl_6639::<0x143cc34a8, 0x143cc34e0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::FindWorkspotService::content::FindWorkspotService::vfunc_2
    Replacement::draft(0x1407d8ef0, "content_1407d8ef0", tpl_7396::<0x3, 0x4, 0x8, 0x143224fc8, 0x142409b50> as *const ()), // content::HazardService::content::HazardService::vfunc_1
    Replacement::draft_throws(0x1407d8f50, "content_1407d8f50", tpl_6639::<0x143cc3494, 0x143cc3478, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::HazardService::content::HazardService::vfunc_2
    Replacement::draft(0x1407d9070, "content_1407d9070", tpl_7388::<0x3, 0x4, 0x8, 0x143216cd0, 0x142409b50> as *const ()), // content::PathFollowingService::content::PathFollowingService::vfunc_1
    Replacement::draft_throws(0x1407d90d0, "content_1407d90d0", tpl_6639::<0x143cc34e8, 0x143cc34f8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::PathFollowingService::content::PathFollowingService::vfunc_2
    Replacement::draft(0x1407d91f0, "content_1407d91f0", tpl_7395::<0x3, 0x4, 0x8, 0x14325f428, 0x142409b50> as *const ()), // content::SmartObjectService::content::SmartObjectService::vfunc_1
    Replacement::draft_throws(0x1407d9250, "content_1407d9250", tpl_6639::<0x143cc34c0, 0x143cc34b0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SmartObjectService::content::SmartObjectService::vfunc_2
    Replacement::draft(0x1407d9370, "content_1407d9370", tpl_7388::<0x3, 0x4, 0x8, 0x143212780, 0x142409b50> as *const ()), // content::SpeedOverrideService::content::SpeedOverrideService::vfunc_1
    Replacement::draft_throws(0x1407d93d0, "content_1407d93d0", tpl_6639::<0x143cc3468, 0x143cc3510, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SpeedOverrideService::content::SpeedOverrideService::vfunc_2
    Replacement::draft(0x1407d94f0, "content_1407d94f0", tpl_7826::<0x3, 0x4, 0x8, 0x143212450, 0x142409b50> as *const ()), // content::StuckDetectionService::content::StuckDetectionService::vfunc_1
    Replacement::draft_throws(0x1407d9550, "content_1407d9550", tpl_6639::<0x143cc34ec, 0x143cc3458, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::StuckDetectionService::content::StuckDetectionService::vfunc_2
    Replacement::draft(0x1407d9670, "content_1407d9670", tpl_7827::<0x3, 0x4, 0x8, 0x143212060, 0x142409b50> as *const ()), // content::SwitchTraversalNodeMode::content::SwitchTraversalNodeMode::vfunc_22
    Replacement::draft_throws(0x1407d96d0, "content_1407d96d0", tpl_7307::<0x143cc3508, 0x143cc34d0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::SwitchTraversalNodeMode::content::SwitchTraversalNodeMode::vfunc_23
    Replacement::draft(0x1407d97f0, "content_1407d97f0", tpl_7390::<0x3, 0x4, 0x8, 0x143211798, 0x142409b50> as *const ()), // content::UseAbilityService::content::UseAbilityService::vfunc_1
    Replacement::draft_throws(0x1407d9850, "content_1407d9850", tpl_6639::<0x143cc3488, 0x143cc3460, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::UseAbilityService::content::UseAbilityService::vfunc_2
    Replacement::draft(0x1407d9970, "content_1407d9970", fn_1407d9970 as *const ()), // sub_1407d9970
    Replacement::draft(0x1407d9a30, "content_1407d9a30", fn_1407d9a30 as *const ()), // sub_1407d9a30
    Replacement::draft(0x1407d9c20, "content_1407d9c20", fn_1407d9c20 as *const ()), // sub_1407d9c20
    Replacement::draft_throws(0x1407d9db0, "content_1407d9db0", fn_1407d9db0 as *const (), Throws::Discard), // sub_1407d9db0
    Replacement::draft_throws(0x1407d9fd0, "content_1407d9fd0", fn_1407d9fd0 as *const (), Throws::Caught), // sub_1407d9fd0
    Replacement::draft_throws(0x1407da6c0, "content_1407da6c0", fn_1407da6c0 as *const (), Throws::Caught), // sub_1407da6c0
    Replacement::draft_throws(0x1407dacd0, "content_1407dacd0", fn_1407dacd0 as *const (), Throws::Caught), // sub_1407dacd0
    Replacement::draft(0x1407db2c0, "content_1407db2c0", fn_1407db2c0 as *const ()), // sub_1407db2c0
    Replacement::draft(0x1407db4e0, "content_1407db4e0", fn_1407db4e0 as *const ()), // sub_1407db4e0
    Replacement::draft(0x1407db650, "content_1407db650", fn_1407db650 as *const ()), // sub_1407db650
    Replacement::draft_throws(0x1407db7e0, "content_1407db7e0", fn_1407db7e0 as *const (), Throws::Discard), // sub_1407db7e0
    Replacement::draft_throws(0x1407db930, "content_1407db930", fn_1407db930 as *const (), Throws::Discard), // sub_1407db930
    Replacement::draft_throws(0x1407dbb40, "content_1407dbb40", fn_1407dbb40 as *const (), Throws::Discard), // sub_1407dbb40
    Replacement::draft(0x1407dbbf0, "content_1407dbbf0", fn_1407dbbf0 as *const ()), // sub_1407dbbf0
    Replacement::draft(0x1407dbcc0, "content_1407dbcc0", tpl_8688::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1407f51a0> as *const ()), // sub_1407dbcc0
    Replacement::draft(0x1407dbd30, "content_1407dbd30", fn_1407dbd30 as *const ()), // sub_1407dbd30
    Replacement::draft(0x1407dbed0, "content_1407dbed0", fn_1407dbed0 as *const ()), // sub_1407dbed0
    Replacement::draft_throws(0x1407dc0f0, "content_1407dc0f0", fn_1407dc0f0 as *const (), Throws::Discard), // sub_1407dc0f0
    Replacement::draft(0x1407dc2a0, "content_1407dc2a0", fn_1407dc2a0 as *const ()), // sub_1407dc2a0
    Replacement::draft_throws(0x1407dc3a0, "content_1407dc3a0", fn_1407dc3a0 as *const (), Throws::Discard), // sub_1407dc3a0
    Replacement::draft_throws(0x1407dc5c0, "content_1407dc5c0", fn_1407dc5c0 as *const (), Throws::Caught), // sub_1407dc5c0
    Replacement::draft_throws(0x1407dc8a0, "content_1407dc8a0", fn_1407dc8a0 as *const (), Throws::Caught), // sub_1407dc8a0
    Replacement::draft_throws(0x1407dcb80, "content_1407dcb80", fn_1407dcb80 as *const (), Throws::Caught), // sub_1407dcb80
    Replacement::draft(0x1407dce60, "content_1407dce60", fn_1407dce60 as *const ()), // sub_1407dce60
    Replacement::draft(0x1407dced0, "content_1407dced0", fn_1407dced0 as *const ()), // sub_1407dced0
    Replacement::draft(0x1407dcfe0, "content_1407dcfe0", fn_1407dcfe0 as *const ()), // sub_1407dcfe0
    Replacement::draft_throws(0x1407dd0f0, "content_1407dd0f0", fn_1407dd0f0 as *const (), Throws::Discard), // sub_1407dd0f0
    Replacement::draft_throws(0x1407dd250, "content_1407dd250", fn_1407dd250 as *const (), Throws::Discard), // sub_1407dd250
    Replacement::draft(0x1407dd4e0, "content_1407dd4e0", fn_1407dd4e0 as *const ()), // sub_1407dd4e0
    Replacement::draft(0x1407dd620, "content_1407dd620", fn_1407dd620 as *const ()), // sub_1407dd620
    Replacement::draft(0x1407dd6f0, "content_1407dd6f0", fn_1407dd6f0 as *const ()), // sub_1407dd6f0
    Replacement::draft_throws(0x1407dd7e0, "content_1407dd7e0", tpl_8703::<0x14387d338, 0x14387d2e0, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x1407f6680, 0x1407f68e0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407dd7e0
    Replacement::draft(0x1407dd880, "content_1407dd880", fn_1407dd880 as *const ()), // sub_1407dd880
    Replacement::draft_throws(0x1407dd940, "content_1407dd940", tpl_8704::<0x14387d338, 0x14387d2e0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x1407f6680, 0x1407f6bb0, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_1407dd940
    Replacement::draft_throws(0x1407dd9f0, "content_1407dd9f0", fn_1407dd9f0 as *const (), Throws::Caught), // sub_1407dd9f0
    Replacement::draft_throws(0x1407ddab0, "content_1407ddab0", fn_1407ddab0 as *const (), Throws::Caught), // sub_1407ddab0
    Replacement::draft_throws(0x1407ddb70, "content_1407ddb70", fn_1407ddb70 as *const (), Throws::Caught), // sub_1407ddb70
    Replacement::draft(0x1407ddc30, "content_1407ddc30", fn_1407ddc30 as *const ()), // sub_1407ddc30
    Replacement::draft(0x1407ddce0, "content_1407ddce0", fn_1407ddce0 as *const ()), // sub_1407ddce0
    Replacement::draft(0x1407ddd90, "content_1407ddd90", fn_1407ddd90 as *const ()), // sub_1407ddd90
    Replacement::draft_throws(0x1407dde50, "content_1407dde50", fn_1407dde50 as *const (), Throws::Discard), // sub_1407dde50
    Replacement::draft_throws(0x1407ddef0, "content_1407ddef0", fn_1407ddef0 as *const (), Throws::Discard), // sub_1407ddef0
    Replacement::draft(0x1407ddfa0, "content_1407ddfa0", fn_1407ddfa0 as *const ()), // sub_1407ddfa0
    Replacement::draft(0x1407de040, "content_1407de040", fn_1407de040 as *const ()), // sub_1407de040
    Replacement::draft(0x1407de100, "content_1407de100", fn_1407de100 as *const ()), // sub_1407de100
    Replacement::draft(0x1407de180, "content_1407de180", fn_1407de180 as *const ()), // sub_1407de180
    Replacement::draft_throws(0x1407de2f0, "content_1407de2f0", fn_1407de2f0 as *const (), Throws::Caught), // sub_1407de2f0
    Replacement::draft(0x1407de580, "content_1407de580", fn_1407de580 as *const ()), // sub_1407de580
    Replacement::draft_throws(0x1407de700, "content_1407de700", fn_1407de700 as *const (), Throws::Discard), // sub_1407de700
    Replacement::draft(0x1407de900, "content_1407de900", fn_1407de900 as *const ()), // sub_1407de900
    Replacement::draft_throws(0x1407de990, "content_1407de990", fn_1407de990 as *const (), Throws::Caught), // sub_1407de990
    Replacement::draft_throws(0x1407dece0, "content_1407dece0", fn_1407dece0 as *const (), Throws::Caught), // sub_1407dece0
    Replacement::draft_throws(0x1407df0d0, "content_1407df0d0", fn_1407df0d0 as *const (), Throws::Caught), // sub_1407df0d0
    Replacement::draft(0x1407df4b0, "content_1407df4b0", fn_1407df4b0 as *const ()), // sub_1407df4b0
    Replacement::draft(0x1407df670, "content_1407df670", fn_1407df670 as *const ()), // sub_1407df670
    Replacement::draft(0x1407df7d0, "content_1407df7d0", fn_1407df7d0 as *const ()), // sub_1407df7d0
    Replacement::draft_throws(0x1407df940, "content_1407df940", fn_1407df940 as *const (), Throws::Discard), // sub_1407df940
    Replacement::draft_throws(0x1407dfa40, "content_1407dfa40", fn_1407dfa40 as *const (), Throws::Caught), // sub_1407dfa40
    Replacement::draft(0x1407dfc40, "content_1407dfc40", fn_1407dfc40 as *const ()), // sub_1407dfc40
    Replacement::draft(0x1407dfcc0, "content_1407dfcc0", fn_1407dfcc0 as *const ()), // sub_1407dfcc0
    Replacement::draft(0x1407dfd60, "content_1407dfd60", fn_1407dfd60 as *const ()), // sub_1407dfd60
    Replacement::draft(0x1407dfe90, "content_1407dfe90", fn_1407dfe90 as *const ()), // sub_1407dfe90
    Replacement::draft(0x1407dffa0, "content_1407dffa0", fn_1407dffa0 as *const ()), // sub_1407dffa0
    Replacement::draft(0x1407e00a0, "content_1407e00a0", fn_1407e00a0 as *const ()), // sub_1407e00a0
    Replacement::draft(0x1407e01b0, "content_1407e01b0", fn_1407e01b0 as *const ()), // sub_1407e01b0
    Replacement::draft(0x1407e0340, "content_1407e0340", fn_1407e0340 as *const ()), // sub_1407e0340
    Replacement::draft(0x1407e0490, "content_1407e0490", fn_1407e0490 as *const ()), // sub_1407e0490
    Replacement::draft(0x1407e0520, "content_1407e0520", fn_1407e0520 as *const ()), // sub_1407e0520
    Replacement::draft(0x1407e0650, "content_1407e0650", fn_1407e0650 as *const ()), // sub_1407e0650
    Replacement::draft(0x1407e0740, "content_1407e0740", fn_1407e0740 as *const ()), // sub_1407e0740
    Replacement::draft(0x1407e07b0, "content_1407e07b0", fn_1407e07b0 as *const ()), // sub_1407e07b0
    Replacement::draft(0x1407e08d0, "content_1407e08d0", tpl_67527::<0x143426cf0, 0x1426cdcb0, 0x4, 0x143245158, 0x14020f270, 0x143426b78, 0x8, 0x143245088> as *const ()), // sub_1407e08d0
    Replacement::draft(0x1407e0960, "content_1407e0960", fn_1407e0960 as *const ()), // sub_1407e0960
    Replacement::draft(0x1407e09c0, "content_1407e09c0", fn_1407e09c0 as *const ()), // sub_1407e09c0
    Replacement::draft(0x1407e0ae0, "content_1407e0ae0", tpl_67527::<0x143426a48, 0x1426cdcb0, 0x4, 0x143245420, 0x14020f270, 0x143426a38, 0x8, 0x143245370> as *const ()), // sub_1407e0ae0
    Replacement::draft(0x1407e0b70, "content_1407e0b70", fn_1407e0b70 as *const ()), // sub_1407e0b70
    Replacement::draft(0x1407e0c50, "content_1407e0c50", fn_1407e0c50 as *const ()), // sub_1407e0c50
    Replacement::draft(0x1407e0d60, "content_1407e0d60", fn_1407e0d60 as *const ()), // sub_1407e0d60
    Replacement::draft(0x1407e0e50, "content_1407e0e50", tpl_8742::<0x14387d2b8, 0x14243b0b0, 0x140761e30, 0x3, 0x14075bb30, 0x10, 0x1406fc6d0, 0x2, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e0e50
    Replacement::draft_throws(0x1407e0f10, "content_1407e0f10", tpl_8743::<0x14387d24c, 0x14387d2b8, 0x1401e9fa0, 0x140761f40, 0x3, 0x14075bb30, 0x10, 0x1406d96a0, 0x2, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407e0f10
    Replacement::draft(0x1407e0fc0, "content_1407e0fc0", tpl_8742::<0x14387d2b8, 0x14243b0b0, 0x140762060, 0x3, 0x14075bb30, 0x10, 0x1406fca00, 0x2, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e0fc0
    Replacement::draft_throws(0x1407e1080, "content_1407e1080", tpl_8743::<0x14387d24c, 0x14387d2b8, 0x1401e9b60, 0x140762160, 0x3, 0x14075bb30, 0x10, 0x1406d97d0, 0x2, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407e1080
    Replacement::draft(0x1407e1130, "content_1407e1130", fn_1407e1130 as *const ()), // sub_1407e1130
    Replacement::draft_throws(0x1407e11e0, "content_1407e11e0", fn_1407e11e0 as *const (), Throws::Caught), // sub_1407e11e0
    Replacement::draft_throws(0x1407e12a0, "content_1407e12a0", fn_1407e12a0 as *const (), Throws::Caught), // sub_1407e12a0
    Replacement::draft_throws(0x1407e1360, "content_1407e1360", fn_1407e1360 as *const (), Throws::Caught), // sub_1407e1360
    Replacement::draft(0x1407e1420, "content_1407e1420", fn_1407e1420 as *const ()), // sub_1407e1420
    Replacement::draft(0x1407e14f0, "content_1407e14f0", fn_1407e14f0 as *const ()), // sub_1407e14f0
    Replacement::draft(0x1407e15a0, "content_1407e15a0", tpl_8742::<0x14387d2b8, 0x14243b0b0, 0x140763ac0, 0x3, 0x14075bb30, 0x10, 0x1406fcfb0, 0x2, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e15a0
    Replacement::draft_throws(0x1407e1660, "content_1407e1660", fn_1407e1660 as *const (), Throws::Discard), // sub_1407e1660
    Replacement::draft_throws(0x1407e1710, "content_1407e1710", fn_1407e1710 as *const (), Throws::Discard), // sub_1407e1710
    Replacement::draft(0x1407e17c0, "content_1407e17c0", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432601a0, 0x8> as *const ()), // sub_1407e17c0
    Replacement::draft(0x1407e1850, "content_1407e1850", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432601a0, 0x18> as *const ()), // sub_1407e1850
    Replacement::draft(0x1407e1970, "content_1407e1970", fn_1407e1970 as *const ()), // sub_1407e1970
    Replacement::draft_throws(0x1407e1a20, "content_1407e1a20", tpl_7851::<0x14387d250, 0x14387d320, 0x1401e9fa0, 0x140761f40, 0x328, 0x338, 0x10, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407e1a20
    Replacement::draft(0x1407e1ab0, "content_1407e1ab0", tpl_7857::<0x14387d320, 0x14243b0b0, 0x140762060, 0x338, 0x10, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e1ab0
    Replacement::draft_throws(0x1407e1b30, "content_1407e1b30", tpl_7972::<0x14387d250, 0x14387d320, 0x1401e9b60, 0x140762160, 0x328, 0x338, 0x120, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407e1b30
    Replacement::draft(0x1407e1bc0, "content_1407e1bc0", fn_1407e1bc0 as *const ()), // sub_1407e1bc0
    Replacement::draft_throws(0x1407e1c30, "content_1407e1c30", fn_1407e1c30 as *const (), Throws::Caught), // sub_1407e1c30
    Replacement::draft_throws(0x1407e1e30, "content_1407e1e30", fn_1407e1e30 as *const (), Throws::Caught), // sub_1407e1e30
    Replacement::draft_throws(0x1407e1ff0, "content_1407e1ff0", fn_1407e1ff0 as *const (), Throws::Caught), // sub_1407e1ff0
    Replacement::draft(0x1407e21b0, "content_1407e21b0", tpl_7855::<0x14387d320, 0x14243b0b0, 0x140763860, 0x338, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e21b0
    Replacement::draft(0x1407e2230, "content_1407e2230", tpl_8074::<0x14387d250, 0x14387d320, 0x1401e9970, 0x1407639f0, 0x10, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_1407e2230
    Replacement::draft(0x1407e22b0, "content_1407e22b0", tpl_8075::<0x14387d320, 0x14243b0b0, 0x140763ac0, 0x10, 0x338, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e22b0
    Replacement::draft_throws(0x1407e2340, "content_1407e2340", tpl_7974::<0x14387d250, 0x14387d320, 0x1401e9770, 0x140763ba0, 0x338, 0x10, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1407e2340
    Replacement::draft_throws(0x1407e23c0, "content_1407e23c0", fn_1407e23c0 as *const (), Throws::Discard), // sub_1407e23c0
    Replacement::draft(0x1407e2480, "content_1407e2480", tpl_8756::<0x14384e610, 0x10, 0x1426cdcb0, 0x14325ed38, 0x1401de410> as *const ()), // sub_1407e2480
    Replacement::draft(0x1407e2500, "content_1407e2500", tpl_7496::<0x140763d90, 0x14384e610, 0x10, 0x1426cdcb0, 0x14325ed38, 0x1401de410> as *const ()), // sub_1407e2500
    Replacement::draft(0x1407e2570, "content_1407e2570", tpl_8757::<0x14384e610, 0x10, 0x1426cdcb0, 0x14325ed38, 0x1401de410> as *const ()), // sub_1407e2570
    Replacement::draft(0x1407e25f0, "content_1407e25f0", fn_1407e25f0 as *const ()), // sub_1407e25f0
    Replacement::draft(0x1407e2680, "content_1407e2680", fn_1407e2680 as *const ()), // sub_1407e2680
    Replacement::draft(0x1407e2730, "content_1407e2730", tpl_8759::<0x14387d240, 0x14243b0b0, 0x140761db0, 0x14387d230, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x88> as *const ()), // sub_1407e2730
    Replacement::draft(0x1407e2820, "content_1407e2820", tpl_8760::<0x14387d240, 0x14243b0b0, 0x140761e30, 0x10, 0x1406fc6d0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e2820
    Replacement::draft_throws(0x1407e28b0, "content_1407e28b0", tpl_67546::<0x14387d254, 0x14387d240, 0x1401e9fa0, 0x140761f40, 0x10, 0x1406d96a0, 0x88, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407e28b0
    Replacement::draft(0x1407e2930, "content_1407e2930", tpl_8760::<0x14387d240, 0x14243b0b0, 0x140762060, 0x10, 0x1406fca00, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e2930
    Replacement::draft_throws(0x1407e29c0, "content_1407e29c0", tpl_67546::<0x14387d254, 0x14387d240, 0x1401e9b60, 0x140762160, 0x10, 0x1406d97d0, 0x88, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407e29c0
    Replacement::draft(0x1407e2a40, "content_1407e2a40", fn_1407e2a40 as *const ()), // sub_1407e2a40
    Replacement::draft_throws(0x1407e2b40, "content_1407e2b40", fn_1407e2b40 as *const (), Throws::Caught), // sub_1407e2b40
    Replacement::draft_throws(0x1407e2c10, "content_1407e2c10", fn_1407e2c10 as *const (), Throws::Caught), // sub_1407e2c10
    Replacement::draft_throws(0x1407e2ce0, "content_1407e2ce0", fn_1407e2ce0 as *const (), Throws::Caught), // sub_1407e2ce0
    Replacement::draft(0x1407e2db0, "content_1407e2db0", fn_1407e2db0 as *const ()), // sub_1407e2db0
    Replacement::draft(0x1407e2e50, "content_1407e2e50", fn_1407e2e50 as *const ()), // sub_1407e2e50
    Replacement::draft(0x1407e2ed0, "content_1407e2ed0", tpl_8760::<0x14387d240, 0x14243b0b0, 0x140763ac0, 0x10, 0x1406fcfb0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407e2ed0
    Replacement::draft_throws(0x1407e2f60, "content_1407e2f60", fn_1407e2f60 as *const (), Throws::Discard), // sub_1407e2f60
    Replacement::draft_throws(0x1407e2fe0, "content_1407e2fe0", fn_1407e2fe0 as *const (), Throws::Discard), // sub_1407e2fe0
    Replacement::draft(0x1407e30a0, "content_1407e30a0", tpl_8768::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432615e0> as *const ()), // sub_1407e30a0
    Replacement::draft(0x1407e3130, "content_1407e3130", fn_1407e3130 as *const ()), // sub_1407e3130
    Replacement::draft(0x1407e31d0, "content_1407e31d0", fn_1407e31d0 as *const ()), // sub_1407e31d0
    Replacement::draft(0x1407e32a0, "content_1407e32a0", fn_1407e32a0 as *const ()), // sub_1407e32a0
    Replacement::draft_throws(0x1407e33a0, "content_1407e33a0", tpl_8772::<0x14387d2c4, 0x14387d2d0, 0x1401e9fa0, 0x140761f40, 0x10, 0x1407f52c0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407e33a0
    Replacement::draft(0x1407e3410, "content_1407e3410", fn_1407e3410 as *const ()), // sub_1407e3410
    Replacement::draft_throws(0x1407e34e0, "content_1407e34e0", tpl_8772::<0x14387d2c4, 0x14387d2d0, 0x1401e9b60, 0x140762160, 0x10, 0x1407f5470, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407e34e0
    Replacement::draft(0x1407e3550, "content_1407e3550", fn_1407e3550 as *const ()), // sub_1407e3550
    Replacement::draft_throws(0x1407e3640, "content_1407e3640", fn_1407e3640 as *const (), Throws::Caught), // sub_1407e3640
    Replacement::draft_throws(0x1407e3700, "content_1407e3700", fn_1407e3700 as *const (), Throws::Caught), // sub_1407e3700
    Replacement::draft_throws(0x1407e37c0, "content_1407e37c0", fn_1407e37c0 as *const (), Throws::Caught), // sub_1407e37c0
    Replacement::draft(0x1407e3880, "content_1407e3880", fn_1407e3880 as *const ()), // sub_1407e3880
    Replacement::draft(0x1407e3940, "content_1407e3940", fn_1407e3940 as *const ()), // sub_1407e3940
    Replacement::draft(0x1407e39f0, "content_1407e39f0", fn_1407e39f0 as *const ()), // sub_1407e39f0
    Replacement::draft_throws(0x1407e3ac0, "content_1407e3ac0", fn_1407e3ac0 as *const (), Throws::Discard), // sub_1407e3ac0
    Replacement::draft_throws(0x1407e3b70, "content_1407e3b70", fn_1407e3b70 as *const (), Throws::Discard), // sub_1407e3b70
    Replacement::draft(0x1407e3c20, "content_1407e3c20", fn_1407e3c20 as *const ()), // sub_1407e3c20
    Replacement::draft(0x1407e3cb0, "content_1407e3cb0", fn_1407e3cb0 as *const ()), // sub_1407e3cb0
    Replacement::draft(0x1407e3d60, "content_1407e3d60", fn_1407e3d60 as *const ()), // sub_1407e3d60
    Replacement::draft(0x1407e3f10, "content_1407e3f10", fn_1407e3f10 as *const ()), // sub_1407e3f10
    Replacement::draft_throws(0x1407e40d0, "content_1407e40d0", fn_1407e40d0 as *const (), Throws::Caught), // sub_1407e40d0
    Replacement::draft(0x1407e42f0, "content_1407e42f0", fn_1407e42f0 as *const ()), // sub_1407e42f0
    Replacement::draft_throws(0x1407e4440, "content_1407e4440", fn_1407e4440 as *const (), Throws::Discard), // sub_1407e4440
    Replacement::draft(0x1407e4600, "content_1407e4600", fn_1407e4600 as *const ()), // sub_1407e4600
    Replacement::draft_throws(0x1407e4810, "content_1407e4810", fn_1407e4810 as *const (), Throws::Caught), // sub_1407e4810
    Replacement::draft_throws(0x1407e4bd0, "content_1407e4bd0", fn_1407e4bd0 as *const (), Throws::Caught), // sub_1407e4bd0
    Replacement::draft_throws(0x1407e4f60, "content_1407e4f60", fn_1407e4f60 as *const (), Throws::Caught), // sub_1407e4f60
    Replacement::draft(0x1407e5300, "content_1407e5300", fn_1407e5300 as *const ()), // sub_1407e5300
    Replacement::draft(0x1407e5520, "content_1407e5520", fn_1407e5520 as *const ()), // sub_1407e5520
    Replacement::draft(0x1407e5650, "content_1407e5650", fn_1407e5650 as *const ()), // sub_1407e5650
    Replacement::draft_throws(0x1407e5790, "content_1407e5790", fn_1407e5790 as *const (), Throws::Discard), // sub_1407e5790
    Replacement::draft_throws(0x1407e58f0, "content_1407e58f0", fn_1407e58f0 as *const (), Throws::Discard), // sub_1407e58f0
    Replacement::draft(0x1407e5b20, "content_1407e5b20", fn_1407e5b20 as *const ()), // sub_1407e5b20
    Replacement::draft(0x1407e5c20, "content_1407e5c20", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143261be0, 0x8> as *const ()), // sub_1407e5c20
    Replacement::draft(0x1407e5ca0, "content_1407e5ca0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143261be0> as *const ()), // sub_1407e5ca0
    Replacement::draft_throws(0x1407e5e80, "content_1407e5e80", fn_1407e5e80 as *const (), Throws::Caught), // sub_1407e5e80
    Replacement::draft_throws(0x1407e5f20, "content_1407e5f20", fn_1407e5f20 as *const (), Throws::Caught), // sub_1407e5f20
    Replacement::draft_throws(0x1407e5fc0, "content_1407e5fc0", fn_1407e5fc0 as *const (), Throws::Caught), // sub_1407e5fc0
    Replacement::draft_throws(0x1407e6140, "content_1407e6140", tpl_7326::<0x14387d348, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140763c80> as *const (), Throws::Discard), // sub_1407e6140
    Replacement::draft(0x1407e61e0, "content_1407e61e0", tpl_8792::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143261520, 0x20, 0x28> as *const ()), // sub_1407e61e0
    Replacement::draft(0x1407e6280, "content_1407e6280", fn_1407e6280 as *const ()), // sub_1407e6280
    Replacement::draft(0x1407e63b0, "content_1407e63b0", fn_1407e63b0 as *const ()), // sub_1407e63b0
    Replacement::draft_throws(0x1407e64d0, "content_1407e64d0", fn_1407e64d0 as *const (), Throws::Discard), // sub_1407e64d0
    Replacement::draft(0x1407e6610, "content_1407e6610", fn_1407e6610 as *const ()), // sub_1407e6610
    Replacement::draft_throws(0x1407e6700, "content_1407e6700", fn_1407e6700 as *const (), Throws::Discard), // sub_1407e6700
    Replacement::draft(0x1407e6880, "content_1407e6880", fn_1407e6880 as *const ()), // sub_1407e6880
    Replacement::draft_throws(0x1407e6970, "content_1407e6970", fn_1407e6970 as *const (), Throws::Caught), // sub_1407e6970
    Replacement::draft_throws(0x1407e6d20, "content_1407e6d20", fn_1407e6d20 as *const (), Throws::Caught), // sub_1407e6d20
    Replacement::draft_throws(0x1407e7070, "content_1407e7070", fn_1407e7070 as *const (), Throws::Caught), // sub_1407e7070
    Replacement::draft(0x1407e73b0, "content_1407e73b0", fn_1407e73b0 as *const ()), // sub_1407e73b0
    Replacement::draft(0x1407e7490, "content_1407e7490", fn_1407e7490 as *const ()), // sub_1407e7490
    Replacement::draft(0x1407e7570, "content_1407e7570", fn_1407e7570 as *const ()), // sub_1407e7570
    Replacement::draft_throws(0x1407e7660, "content_1407e7660", fn_1407e7660 as *const (), Throws::Discard), // sub_1407e7660
    Replacement::draft_throws(0x1407e7740, "content_1407e7740", fn_1407e7740 as *const (), Throws::Discard), // sub_1407e7740
    Replacement::draft(0x1407e7880, "content_1407e7880", fn_1407e7880 as *const ()), // sub_1407e7880
    Replacement::draft(0x1407e7930, "content_1407e7930", fn_1407e7930 as *const ()), // sub_1407e7930
    Replacement::draft(0x1407e7a00, "content_1407e7a00", fn_1407e7a00 as *const ()), // sub_1407e7a00
    Replacement::draft(0x1407e7ab0, "content_1407e7ab0", tpl_7675::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1432610e0, 0x88, 0x30, 0x33, 0x34> as *const ()), // sub_1407e7ab0
    Replacement::draft(0x1407e7ba0, "content_1407e7ba0", tpl_66600::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432610e0, 0xffffffffffffff80, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_1407e7ba0
    Replacement::draft(0x1407e7cd0, "content_1407e7cd0", tpl_7739::<0x14387d270, 0x14243b0b0, 0x140761db0, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x14387d230, 0x30, 0xfffffffffffffff8> as *const ()), // sub_1407e7cd0
    Replacement::draft(0x1407e7db0, "content_1407e7db0", fn_1407e7db0 as *const ()), // sub_1407e7db0
    Replacement::draft_throws(0x1407e7f90, "content_1407e7f90", fn_1407e7f90 as *const (), Throws::Caught), // sub_1407e7f90
    Replacement::draft(0x1407e8190, "content_1407e8190", fn_1407e8190 as *const ()), // sub_1407e8190
    Replacement::draft_throws(0x1407e82e0, "content_1407e82e0", fn_1407e82e0 as *const (), Throws::Discard), // sub_1407e82e0
    Replacement::draft(0x1407e8490, "content_1407e8490", fn_1407e8490 as *const ()), // sub_1407e8490
    Replacement::draft_throws(0x1407e85b0, "content_1407e85b0", fn_1407e85b0 as *const (), Throws::Caught), // sub_1407e85b0
    Replacement::draft_throws(0x1407e8910, "content_1407e8910", fn_1407e8910 as *const (), Throws::Caught), // sub_1407e8910
    Replacement::draft_throws(0x1407e8c40, "content_1407e8c40", fn_1407e8c40 as *const (), Throws::Caught), // sub_1407e8c40
    Replacement::draft(0x1407e8f70, "content_1407e8f70", fn_1407e8f70 as *const ()), // sub_1407e8f70
    Replacement::draft(0x1407e9160, "content_1407e9160", fn_1407e9160 as *const ()), // sub_1407e9160
    Replacement::draft(0x1407e9290, "content_1407e9290", fn_1407e9290 as *const ()), // sub_1407e9290
    Replacement::draft_throws(0x1407e93d0, "content_1407e93d0", fn_1407e93d0 as *const (), Throws::Discard), // sub_1407e93d0
    Replacement::draft_throws(0x1407e9520, "content_1407e9520", fn_1407e9520 as *const (), Throws::Discard), // sub_1407e9520
    Replacement::draft(0x1407e96f0, "content_1407e96f0", fn_1407e96f0 as *const ()), // sub_1407e96f0
    Replacement::draft(0x1407e97f0, "content_1407e97f0", fn_1407e97f0 as *const ()), // sub_1407e97f0
    Replacement::draft(0x1407e9880, "content_1407e9880", fn_1407e9880 as *const ()), // sub_1407e9880
    Replacement::draft(0x1407e9930, "content_1407e9930", fn_1407e9930 as *const ()), // sub_1407e9930
    Replacement::draft(0x1407e9a60, "content_1407e9a60", fn_1407e9a60 as *const ()), // sub_1407e9a60
    Replacement::draft_throws(0x1407e9b70, "content_1407e9b70", fn_1407e9b70 as *const (), Throws::Caught), // sub_1407e9b70
    Replacement::draft(0x1407e9cf0, "content_1407e9cf0", fn_1407e9cf0 as *const ()), // sub_1407e9cf0
    Replacement::draft_throws(0x1407e9df0, "content_1407e9df0", fn_1407e9df0 as *const (), Throws::Discard), // sub_1407e9df0
    Replacement::draft(0x1407e9ee0, "content_1407e9ee0", fn_1407e9ee0 as *const ()), // sub_1407e9ee0
    Replacement::draft_throws(0x1407ea040, "content_1407ea040", fn_1407ea040 as *const (), Throws::Caught), // sub_1407ea040
    Replacement::draft_throws(0x1407ea300, "content_1407ea300", fn_1407ea300 as *const (), Throws::Caught), // sub_1407ea300
    Replacement::draft_throws(0x1407ea590, "content_1407ea590", fn_1407ea590 as *const (), Throws::Caught), // sub_1407ea590
    Replacement::draft(0x1407ea830, "content_1407ea830", fn_1407ea830 as *const ()), // sub_1407ea830
    Replacement::draft(0x1407ea9e0, "content_1407ea9e0", fn_1407ea9e0 as *const ()), // sub_1407ea9e0
    Replacement::draft(0x1407eaab0, "content_1407eaab0", fn_1407eaab0 as *const ()), // sub_1407eaab0
    Replacement::draft_throws(0x1407eab90, "content_1407eab90", fn_1407eab90 as *const (), Throws::Discard), // sub_1407eab90
    Replacement::draft_throws(0x1407eac70, "content_1407eac70", fn_1407eac70 as *const (), Throws::Discard), // sub_1407eac70
    Replacement::draft(0x1407eadc0, "content_1407eadc0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14325b590, 0x8> as *const ()), // sub_1407eadc0
    Replacement::draft(0x1407eae40, "content_1407eae40", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14325b590> as *const ()), // sub_1407eae40
    Replacement::draft(0x1407eaed0, "content_1407eaed0", tpl_8838::<0x14387d358, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407eaed0
    Replacement::draft_throws(0x1407eaf70, "content_1407eaf70", tpl_8839::<0x14387d278, 0x14387d358, 0x1401e9fa0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407eaf70
    Replacement::draft(0x1407eaff0, "content_1407eaff0", tpl_8840::<0x14387d358, 0x14243b0b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407eaff0
    Replacement::draft_throws(0x1407eb060, "content_1407eb060", tpl_8841::<0x14387d278, 0x14387d358, 0x1401e9b60, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407eb060
    Replacement::draft_throws(0x1407eb0f0, "content_1407eb0f0", fn_1407eb0f0 as *const (), Throws::Caught), // sub_1407eb0f0
    Replacement::draft_throws(0x1407eb2e0, "content_1407eb2e0", fn_1407eb2e0 as *const (), Throws::Caught), // sub_1407eb2e0
    Replacement::draft_throws(0x1407eb4a0, "content_1407eb4a0", fn_1407eb4a0 as *const (), Throws::Caught), // sub_1407eb4a0
    Replacement::draft(0x1407eb660, "content_1407eb660", fn_1407eb660 as *const ()), // sub_1407eb660
    Replacement::draft(0x1407eb6d0, "content_1407eb6d0", fn_1407eb6d0 as *const ()), // sub_1407eb6d0
    Replacement::draft(0x1407eb740, "content_1407eb740", fn_1407eb740 as *const ()), // sub_1407eb740
    Replacement::draft_throws(0x1407eb7c0, "content_1407eb7c0", tpl_8848::<0x14387d278, 0x14387d358, 0x1401e9770, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1407eb7c0
    Replacement::draft_throws(0x1407eb830, "content_1407eb830", fn_1407eb830 as *const (), Throws::Discard), // sub_1407eb830
    Replacement::draft(0x1407eb8e0, "content_1407eb8e0", tpl_8850::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x1432590f0, 0x14, 0x8> as *const ()), // sub_1407eb8e0
    Replacement::draft(0x1407eb970, "content_1407eb970", tpl_8851::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432590f0, 0x14, 0x18> as *const ()), // sub_1407eb970
    Replacement::draft(0x1407ebac0, "content_1407ebac0", tpl_8019::<0x14387d308, 0x14243b0b0, 0x1407eaed0, 0x338, 0x10, 0x1426d4580, 0x1, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ebac0
    Replacement::draft_throws(0x1407ebba0, "content_1407ebba0", tpl_8179::<0x14387d214, 0x14387d308, 0x1401e9fa0, 0x1407eaf70, 0x328, 0x10, 0x338, 0x38, 0x369, 0x14, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407ebba0
    Replacement::draft(0x1407ebc60, "content_1407ebc60", tpl_8027::<0x14387d308, 0x14243b0b0, 0x1407eaff0, 0x338, 0x10, 0x38, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ebc60
    Replacement::draft_throws(0x1407ebcf0, "content_1407ebcf0", tpl_8180::<0x14387d214, 0x14387d308, 0x1401e9b60, 0x1407eb060, 0x328, 0x338, 0x120, 0x10, 0x4, 0x369, 0x14, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407ebcf0
    Replacement::draft_throws(0x1407ebdc0, "content_1407ebdc0", fn_1407ebdc0 as *const (), Throws::Caught), // sub_1407ebdc0
    Replacement::draft_throws(0x1407ebef0, "content_1407ebef0", fn_1407ebef0 as *const (), Throws::Caught), // sub_1407ebef0
    Replacement::draft_throws(0x1407ec020, "content_1407ec020", fn_1407ec020 as *const (), Throws::Caught), // sub_1407ec020
    Replacement::draft(0x1407ec150, "content_1407ec150", tpl_8182::<0x14387d308, 0x14243b0b0, 0x1407eb660, 0x338, 0x10, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ec150
    Replacement::draft(0x1407ec1e0, "content_1407ec1e0", tpl_8026::<0x14387d214, 0x14387d308, 0x1401e9970, 0x1407eb6d0, 0x338, 0x10, 0xd8, 0x14, 0x1401e9860> as *const ()), // sub_1407ec1e0
    Replacement::draft(0x1407ec270, "content_1407ec270", tpl_8027::<0x14387d308, 0x14243b0b0, 0x1407eb740, 0x338, 0x10, 0xd8, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ec270
    Replacement::draft_throws(0x1407ec300, "content_1407ec300", tpl_8028::<0x14387d214, 0x14387d308, 0x1401e9770, 0x1407eb7c0, 0x338, 0x10, 0x120, 0x4, 0x14, 0x1401e9590> as *const (), Throws::Discard), // sub_1407ec300
    Replacement::draft_throws(0x1407ec3a0, "content_1407ec3a0", fn_1407ec3a0 as *const (), Throws::Discard), // sub_1407ec3a0
    Replacement::draft(0x1407ec4f0, "content_1407ec4f0", tpl_7173::<0x143251088, 0x10, 0x1, 0x14020f270, 0x14, 0x143251078> as *const ()), // sub_1407ec4f0
    Replacement::draft(0x1407ec560, "content_1407ec560", tpl_8850::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x14325a740, 0x14, 0x8> as *const ()), // sub_1407ec560
    Replacement::draft(0x1407ec5f0, "content_1407ec5f0", tpl_8851::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14325a740, 0x14, 0x18> as *const ()), // sub_1407ec5f0
    Replacement::draft(0x1407ec740, "content_1407ec740", tpl_8185::<0x14387d238, 0x14243b0b0, 0x1407eaed0, 0x338, 0x10, 0x1426d4580, 0x1, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ec740
    Replacement::draft_throws(0x1407ec820, "content_1407ec820", tpl_8179::<0x14387d354, 0x14387d238, 0x1401e9fa0, 0x1407eaf70, 0x328, 0x10, 0x338, 0x48, 0x369, 0x14, 0x1401e9e70> as *const (), Throws::Discard), // sub_1407ec820
    Replacement::draft(0x1407ec8e0, "content_1407ec8e0", tpl_8027::<0x14387d238, 0x14243b0b0, 0x1407eaff0, 0x338, 0x10, 0x48, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ec8e0
    Replacement::draft_throws(0x1407ec970, "content_1407ec970", tpl_8186::<0x14387d354, 0x14387d238, 0x1401e9b60, 0x1407eb060, 0x328, 0x338, 0x120, 0x369, 0x10, 0x14, 0x1401e9a20> as *const (), Throws::Discard), // sub_1407ec970
    Replacement::draft_throws(0x1407eca40, "content_1407eca40", fn_1407eca40 as *const (), Throws::Caught), // sub_1407eca40
    Replacement::draft_throws(0x1407ecb50, "content_1407ecb50", fn_1407ecb50 as *const (), Throws::Caught), // sub_1407ecb50
    Replacement::draft_throws(0x1407ecc60, "content_1407ecc60", fn_1407ecc60 as *const (), Throws::Caught), // sub_1407ecc60
    Replacement::draft(0x1407ecd70, "content_1407ecd70", tpl_8182::<0x14387d238, 0x14243b0b0, 0x1407eb660, 0x338, 0x10, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ecd70
    Replacement::draft(0x1407ece00, "content_1407ece00", tpl_8026::<0x14387d354, 0x14387d238, 0x1401e9970, 0x1407eb6d0, 0x338, 0x10, 0xe8, 0x14, 0x1401e9860> as *const ()), // sub_1407ece00
    Replacement::draft(0x1407ece90, "content_1407ece90", tpl_8027::<0x14387d238, 0x14243b0b0, 0x1407eb740, 0x338, 0x10, 0xe8, 0x14, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ece90
    Replacement::draft_throws(0x1407ecf20, "content_1407ecf20", tpl_67013::<0x14387d354, 0x14387d238, 0x1401e9770, 0x1407eb7c0, 0x338, 0x10, 0x120, 0x14, 0x1401e9590> as *const (), Throws::Discard), // sub_1407ecf20
    Replacement::draft_throws(0x1407ecfb0, "content_1407ecfb0", fn_1407ecfb0 as *const (), Throws::Discard), // sub_1407ecfb0
    Replacement::draft(0x1407ed0f0, "content_1407ed0f0", fn_1407ed0f0 as *const ()), // sub_1407ed0f0
    Replacement::draft(0x1407ed1e0, "content_1407ed1e0", tpl_7571::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x68, 0x143262460, 0xffffffffffffff98, 0xffffffffffffffb8, 0xffffffffffffffd8, 0xfffffffffffffff8> as *const ()), // sub_1407ed1e0
    Replacement::draft(0x1407ed2f0, "content_1407ed2f0", tpl_7452::<0x14387d300, 0x14243b0b0, 0x140761db0, 0x14387d230, 0x18, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1407ed2f0
    Replacement::draft(0x1407ed390, "content_1407ed390", fn_1407ed390 as *const ()), // sub_1407ed390
    Replacement::draft_throws(0x1407ed450, "content_1407ed450", fn_1407ed450 as *const (), Throws::Discard), // sub_1407ed450
    Replacement::draft(0x1407ed4f0, "content_1407ed4f0", tpl_8857::<0x14387d300, 0x14243b0b0, 0x140762060, 0x338, 0x10, 0x38, 0x18, 0x1406fca00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407ed4f0
    Replacement::draft_throws(0x1407ed580, "content_1407ed580", fn_1407ed580 as *const (), Throws::Discard), // sub_1407ed580
    Replacement::draft(0x1407ed630, "content_1407ed630", fn_1407ed630 as *const ()), // sub_1407ed630
    Replacement::draft_throws(0x1407ed6f0, "content_1407ed6f0", fn_1407ed6f0 as *const (), Throws::Caught), // sub_1407ed6f0
    Replacement::draft_throws(0x1407ed7f0, "content_1407ed7f0", fn_1407ed7f0 as *const (), Throws::Caught), // sub_1407ed7f0
    Replacement::draft_throws(0x1407ed8f0, "content_1407ed8f0", fn_1407ed8f0 as *const (), Throws::Caught), // sub_1407ed8f0
    Replacement::draft(0x1407ed9f0, "content_1407ed9f0", fn_1407ed9f0 as *const ()), // sub_1407ed9f0
    Replacement::draft(0x1407eda90, "content_1407eda90", fn_1407eda90 as *const ()), // sub_1407eda90
    Replacement::draft(0x1407edb10, "content_1407edb10", tpl_8857::<0x14387d300, 0x14243b0b0, 0x140763ac0, 0x338, 0x10, 0xd8, 0x18, 0x1406fcfb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407edb10
    Replacement::draft_throws(0x1407edba0, "content_1407edba0", fn_1407edba0 as *const (), Throws::Discard), // sub_1407edba0
    Replacement::draft_throws(0x1407edc30, "content_1407edc30", fn_1407edc30 as *const (), Throws::Discard), // sub_1407edc30
    Replacement::draft(0x1407edd20, "content_1407edd20", tpl_7909::<0x120, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1407f4bb0> as *const ()), // sub_1407edd20
    Replacement::draft(0x1407eddb0, "content_1407eddb0", tpl_66781::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x1407f4bb0> as *const ()), // sub_1407eddb0
    Replacement::draft(0x1407ede60, "content_1407ede60", tpl_8759::<0x14387d260, 0x14243b0b0, 0x140761db0, 0x14387d230, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x88> as *const ()), // sub_1407ede60
    Replacement::draft(0x1407edf50, "content_1407edf50", fn_1407edf50 as *const ()), // sub_1407edf50
    Replacement::draft_throws(0x1407ee100, "content_1407ee100", fn_1407ee100 as *const (), Throws::Discard), // sub_1407ee100
    Replacement::draft(0x1407ee2a0, "content_1407ee2a0", fn_1407ee2a0 as *const ()), // sub_1407ee2a0
    Replacement::draft_throws(0x1407ee3a0, "content_1407ee3a0", fn_1407ee3a0 as *const (), Throws::Discard), // sub_1407ee3a0
    Replacement::draft(0x1407ee580, "content_1407ee580", fn_1407ee580 as *const ()), // sub_1407ee580
    Replacement::draft_throws(0x1407ee6f0, "content_1407ee6f0", fn_1407ee6f0 as *const (), Throws::Caught), // sub_1407ee6f0
    Replacement::draft_throws(0x1407eeda0, "content_1407eeda0", fn_1407eeda0 as *const (), Throws::Caught), // sub_1407eeda0
    Replacement::draft_throws(0x1407ef350, "content_1407ef350", fn_1407ef350 as *const (), Throws::Caught), // sub_1407ef350
    Replacement::draft(0x1407ef8f0, "content_1407ef8f0", fn_1407ef8f0 as *const ()), // sub_1407ef8f0
    Replacement::draft(0x1407efa00, "content_1407efa00", fn_1407efa00 as *const ()), // sub_1407efa00
    Replacement::draft(0x1407efb30, "content_1407efb30", fn_1407efb30 as *const ()), // sub_1407efb30
    Replacement::draft_throws(0x1407efc70, "content_1407efc70", fn_1407efc70 as *const (), Throws::Discard), // sub_1407efc70
    Replacement::draft_throws(0x1407efda0, "content_1407efda0", fn_1407efda0 as *const (), Throws::Discard), // sub_1407efda0
    Replacement::draft(0x1407eff70, "content_1407eff70", fn_1407eff70 as *const ()), // sub_1407eff70
    Replacement::draft(0x1407f0070, "content_1407f0070", fn_1407f0070 as *const ()), // sub_1407f0070
    Replacement::draft(0x1407f0180, "content_1407f0180", fn_1407f0180 as *const ()), // sub_1407f0180
    Replacement::draft(0x1407f0280, "content_1407f0280", fn_1407f0280 as *const ()), // sub_1407f0280
    Replacement::draft(0x1407f0310, "content_1407f0310", fn_1407f0310 as *const ()), // sub_1407f0310
    Replacement::draft(0x1407f03c0, "content_1407f03c0", fn_1407f03c0 as *const ()), // sub_1407f03c0
    Replacement::draft(0x1407f0530, "content_1407f0530", fn_1407f0530 as *const ()), // sub_1407f0530
    Replacement::draft_throws(0x1407f0750, "content_1407f0750", fn_1407f0750 as *const (), Throws::Discard), // sub_1407f0750
    Replacement::draft(0x1407f0940, "content_1407f0940", fn_1407f0940 as *const ()), // sub_1407f0940
    Replacement::draft_throws(0x1407f0a70, "content_1407f0a70", fn_1407f0a70 as *const (), Throws::Discard), // sub_1407f0a70
    Replacement::draft(0x1407f0cd0, "content_1407f0cd0", fn_1407f0cd0 as *const ()), // sub_1407f0cd0
    Replacement::draft_throws(0x1407f0ee0, "content_1407f0ee0", fn_1407f0ee0 as *const (), Throws::Caught), // sub_1407f0ee0
    Replacement::draft_throws(0x1407f11e0, "content_1407f11e0", fn_1407f11e0 as *const (), Throws::Caught), // sub_1407f11e0
    Replacement::draft_throws(0x1407f14e0, "content_1407f14e0", fn_1407f14e0 as *const (), Throws::Caught), // sub_1407f14e0
    Replacement::draft(0x1407f17e0, "content_1407f17e0", fn_1407f17e0 as *const ()), // sub_1407f17e0
    Replacement::draft(0x1407f1950, "content_1407f1950", fn_1407f1950 as *const ()), // sub_1407f1950
    Replacement::draft(0x1407f1a90, "content_1407f1a90", fn_1407f1a90 as *const ()), // sub_1407f1a90
    Replacement::draft_throws(0x1407f1be0, "content_1407f1be0", fn_1407f1be0 as *const (), Throws::Discard), // sub_1407f1be0
    Replacement::draft_throws(0x1407f1d60, "content_1407f1d60", fn_1407f1d60 as *const (), Throws::Discard), // sub_1407f1d60
    Replacement::draft(0x1407f2000, "content_1407f2000", fn_1407f2000 as *const ()), // sub_1407f2000
    Replacement::draft(0x1407f2170, "content_1407f2170", fn_1407f2170 as *const ()), // sub_1407f2170
    Replacement::draft(0x1407f2280, "content_1407f2280", tpl_66547::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143262b20, 0xffffffffffffff70, 0x14183acc2, 0xffffffffffffffb0, 0xffffffffffffffb4, 0xffffffffffffffb8> as *const ()), // sub_1407f2280
    Replacement::draft(0x1407f23c0, "content_1407f23c0", tpl_7452::<0x14387d2c8, 0x14243b0b0, 0x1407c0230, 0x14387d230, 0x40, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1407f23c0
    Replacement::draft(0x1407f2460, "content_1407f2460", fn_1407f2460 as *const ()), // sub_1407f2460
    Replacement::draft_throws(0x1407f2510, "content_1407f2510", fn_1407f2510 as *const (), Throws::Discard), // sub_1407f2510
    Replacement::draft_throws(0x1407f25e0, "content_1407f25e0", tpl_8152::<0x14387d2c8, 0x14243b0b0, 0x1407c0310, 0x338, 0x38, 0x80, 0x40, 0x1406fca00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1407f25e0
    Replacement::draft_throws(0x1407f2670, "content_1407f2670", fn_1407f2670 as *const (), Throws::Discard), // sub_1407f2670
    Replacement::draft_throws(0x1407f2740, "content_1407f2740", fn_1407f2740 as *const (), Throws::Discard), // sub_1407f2740
    Replacement::draft_throws(0x1407f27d0, "content_1407f27d0", fn_1407f27d0 as *const (), Throws::Caught), // sub_1407f27d0
    Replacement::draft_throws(0x1407f28c0, "content_1407f28c0", fn_1407f28c0 as *const (), Throws::Caught), // sub_1407f28c0
    Replacement::draft_throws(0x1407f29b0, "content_1407f29b0", fn_1407f29b0 as *const (), Throws::Caught), // sub_1407f29b0
    Replacement::draft(0x1407f2aa0, "content_1407f2aa0", fn_1407f2aa0 as *const ()), // sub_1407f2aa0
    Replacement::draft(0x1407f2b30, "content_1407f2b30", fn_1407f2b30 as *const ()), // sub_1407f2b30
    Replacement::draft(0x1407f2be0, "content_1407f2be0", tpl_8857::<0x14387d2c8, 0x14243b0b0, 0x1407c05d0, 0x338, 0x38, 0x120, 0x40, 0x1406fcfb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407f2be0
    Replacement::draft_throws(0x1407f2c70, "content_1407f2c70", fn_1407f2c70 as *const (), Throws::Discard), // sub_1407f2c70
    Replacement::draft_throws(0x1407f2d30, "content_1407f2d30", fn_1407f2d30 as *const (), Throws::Discard), // sub_1407f2d30
    Replacement::draft(0x1407f2e40, "content_1407f2e40", fn_1407f2e40 as *const ()), // sub_1407f2e40
    Replacement::draft(0x1407f2ec0, "content_1407f2ec0", tpl_7635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143260ca0, 0x18, 0x20> as *const ()), // sub_1407f2ec0
    Replacement::draft(0x1407f2f60, "content_1407f2f60", tpl_6819::<0x14387d330, 0x14243b0b0, 0x140761db0, 0x1, 0x14075bb30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1407f2f60
    Replacement::draft(0x1407f2ff0, "content_1407f2ff0", fn_1407f2ff0 as *const ()), // sub_1407f2ff0
    Replacement::draft_throws(0x1407f3120, "content_1407f3120", fn_1407f3120 as *const (), Throws::Caught), // sub_1407f3120
    Replacement::draft(0x1407f3270, "content_1407f3270", fn_1407f3270 as *const ()), // sub_1407f3270
    Replacement::draft_throws(0x1407f3370, "content_1407f3370", fn_1407f3370 as *const (), Throws::Discard), // sub_1407f3370
    Replacement::draft(0x1407f3480, "content_1407f3480", fn_1407f3480 as *const ()), // sub_1407f3480
    Replacement::draft_throws(0x1407f3510, "content_1407f3510", fn_1407f3510 as *const (), Throws::Caught), // sub_1407f3510
    Replacement::draft_throws(0x1407f38c0, "content_1407f38c0", fn_1407f38c0 as *const (), Throws::Caught), // sub_1407f38c0
    Replacement::draft_throws(0x1407f3bf0, "content_1407f3bf0", fn_1407f3bf0 as *const (), Throws::Caught), // sub_1407f3bf0
    Replacement::draft(0x1407f3f10, "content_1407f3f10", fn_1407f3f10 as *const ()), // sub_1407f3f10
    Replacement::draft(0x1407f40a0, "content_1407f40a0", fn_1407f40a0 as *const ()), // sub_1407f40a0
    Replacement::draft(0x1407f4190, "content_1407f4190", fn_1407f4190 as *const ()), // sub_1407f4190
    Replacement::draft_throws(0x1407f4290, "content_1407f4290", fn_1407f4290 as *const (), Throws::Discard), // sub_1407f4290
    Replacement::draft_throws(0x1407f4380, "content_1407f4380", fn_1407f4380 as *const (), Throws::Discard), // sub_1407f4380
    Replacement::draft(0x1407f44b0, "content_1407f44b0", tpl_8756::<0x14384e650, 0x18, 0x1426cdcb0, 0x14325e2f8, 0x1401de410> as *const ()), // sub_1407f44b0
    Replacement::draft(0x1407f4530, "content_1407f4530", fn_1407f4530 as *const ()), // sub_1407f4530
    Replacement::draft(0x1407f45c0, "content_1407f45c0", tpl_8757::<0x14384e650, 0x18, 0x1426cdcb0, 0x14325e2f8, 0x1401de410> as *const ()), // sub_1407f45c0
    Replacement::draft(0x1407f4640, "content_1407f4640", fn_1407f4640 as *const ()), // sub_1407f4640
    Replacement::draft(0x1407f4740, "content_1407f4740", fn_1407f4740 as *const ()), // sub_1407f4740
    Replacement::draft(0x1407f4800, "content_1407f4800", tpl_6947::<0x1407f6810, 0x141834d30> as *const ()), // content::CountdownService::content::CountdownService::vfunc_0
    Replacement::draft(0x1407f4850, "content_1407f4850", fn_1407f4850 as *const ()), // sub_1407f4850
    Replacement::draft(0x1407f4a40, "content_1407f4a40", tpl_8245::<0x1401de830, 0x141834d30> as *const ()), // content::FindWorkspotService::content::FindWorkspotService::vfunc_0
    Replacement::draft(0x1407f4a80, "content_1407f4a80", fn_1407f4a80 as *const ()), // sub_1407f4a80
    Replacement::draft(0x1407f4bb0, "content_1407f4bb0", fn_1407f4bb0 as *const ()), // sub_1407f4bb0
    Replacement::draft(0x1407f4c80, "content_1407f4c80", fn_1407f4c80 as *const ()), // sub_1407f4c80
    Replacement::draft(0x1407f4e00, "content_1407f4e00", fn_1407f4e00 as *const ()), // content::AimService::content::AimService::vfunc_0
    Replacement::draft(0x1407f4e50, "content_1407f4e50", fn_1407f4e50 as *const ()), // content::AvoidanceTypeService::content::AvoidanceTypeService::vfunc_0
    Replacement::draft(0x1407f4ec0, "content_1407f4ec0", fn_1407f4ec0 as *const ()), // content::CoverService::content::CoverService::vfunc_0
    Replacement::draft(0x1407f4f80, "content_1407f4f80", fn_1407f4f80 as *const ()), // content::PathFollowingService::content::PathFollowingService::vfunc_0
    Replacement::draft(0x1407f5010, "content_1407f5010", fn_1407f5010 as *const ()), // content::SpeedOverrideService::content::SpeedOverrideService::vfunc_0
    Replacement::draft(0x1407f5080, "content_1407f5080", fn_1407f5080 as *const ()), // content::StuckDetectionService::content::StuckDetectionService::vfunc_0
    Replacement::draft(0x1407f5120, "content_1407f5120", fn_1407f5120 as *const ()), // content::SwitchTraversalNodeMode::content::SwitchTraversalNodeMode::vfunc_0
    Replacement::draft(0x1407f51a0, "unknown_1407f51a0", fn_1407f51a0 as *const ()), // sub_1407f51a0
    Replacement::draft_throws(0x1407f5260, "unknown_1407f5260", fn_1407f5260 as *const (), Throws::Discard), // sub_1407f5260
    Replacement::draft_throws(0x1407f52c0, "unknown_1407f52c0", fn_1407f52c0 as *const (), Throws::Caught), // sub_1407f52c0
    Replacement::draft_throws(0x1407f5470, "unknown_1407f5470", fn_1407f5470 as *const (), Throws::Caught), // sub_1407f5470
    Replacement::draft_throws(0x1407f5660, "unknown_1407f5660", tpl_8937::<0x20, 0x8, 0x1407f8990> as *const (), Throws::Discard), // sub_1407f5660
    Replacement::draft_throws(0x1407f56b0, "unknown_1407f56b0", fn_1407f56b0 as *const (), Throws::Caught), // sub_1407f56b0
    Replacement::draft_throws(0x1407f5c20, "unknown_1407f5c20", fn_1407f5c20 as *const (), Throws::Caught), // sub_1407f5c20
    Replacement::draft_throws(0x1407f6150, "unknown_1407f6150", fn_1407f6150 as *const (), Throws::Caught), // sub_1407f6150
    Replacement::draft(0x1407f6680, "unknown_1407f6680", fn_1407f6680 as *const ()), // sub_1407f6680
    Replacement::draft(0x1407f6740, "unknown_1407f6740", tpl_67717::<0x8, 0xc, 0x1407faa10, 0x1407f51a0, 0x1407f8aa0> as *const ()), // sub_1407f6740
    Replacement::draft(0x1407f6810, "unknown_1407f6810", fn_1407f6810 as *const ()), // sub_1407f6810
    Replacement::draft_throws(0x1407f68e0, "unknown_1407f68e0", fn_1407f68e0 as *const (), Throws::Caught), // sub_1407f68e0
    Replacement::draft_throws(0x1407f6bb0, "unknown_1407f6bb0", fn_1407f6bb0 as *const (), Throws::Caught), // sub_1407f6bb0
    Replacement::draft(0x1407f6e80, "unknown_1407f6e80", fn_1407f6e80 as *const ()), // sub_1407f6e80
    Replacement::draft_throws(0x1407f7410, "unknown_1407f7410", fn_1407f7410 as *const (), Throws::Caught), // sub_1407f7410
    Replacement::draft_throws(0x1407f7850, "unknown_1407f7850", fn_1407f7850 as *const (), Throws::Caught), // sub_1407f7850
    Replacement::draft_throws(0x1407f7c60, "unknown_1407f7c60", fn_1407f7c60 as *const (), Throws::Caught), // sub_1407f7c60
    Replacement::draft_throws(0x1407f8070, "unknown_1407f8070", fn_1407f8070 as *const (), Throws::Caught), // sub_1407f8070
    Replacement::draft_throws(0x1407f8140, "unknown_1407f8140", fn_1407f8140 as *const (), Throws::Caught), // sub_1407f8140
    Replacement::draft_throws(0x1407f8430, "unknown_1407f8430", fn_1407f8430 as *const (), Throws::Caught), // sub_1407f8430
    Replacement::draft_throws(0x1407f86f0, "unknown_1407f86f0", fn_1407f86f0 as *const (), Throws::Caught), // sub_1407f86f0
    Replacement::draft_throws(0x1407f8990, "unknown_1407f8990", fn_1407f8990 as *const (), Throws::Caught), // sub_1407f8990
    Replacement::draft(0x1407f8a60, "unknown_1407f8a60", fn_1407f8a60 as *const ()), // sub_1407f8a60
    Replacement::draft(0x1407f8aa0, "unknown_1407f8aa0", fn_1407f8aa0 as *const ()), // sub_1407f8aa0
    Replacement::draft(0x1407f8ae0, "unknown_1407f8ae0", fn_1407f8ae0 as *const ()), // sub_1407f8ae0
    Replacement::draft_throws(0x1407f8cf0, "unknown_1407f8cf0", fn_1407f8cf0 as *const (), Throws::Caught), // sub_1407f8cf0
    Replacement::draft_throws(0x1407f91c0, "unknown_1407f91c0", fn_1407f91c0 as *const (), Throws::Caught), // sub_1407f91c0
    Replacement::draft_throws(0x1407f9650, "unknown_1407f9650", fn_1407f9650 as *const (), Throws::Caught), // sub_1407f9650
    Replacement::draft_throws(0x1407f9ae0, "unknown_1407f9ae0", fn_1407f9ae0 as *const (), Throws::Caught), // sub_1407f9ae0
    Replacement::draft_throws(0x1407f9fb0, "unknown_1407f9fb0", fn_1407f9fb0 as *const (), Throws::Caught), // sub_1407f9fb0
    Replacement::draft_throws(0x1407fa440, "unknown_1407fa440", fn_1407fa440 as *const (), Throws::Caught), // sub_1407fa440
    Replacement::draft(0x1407faa10, "unknown_1407faa10", fn_1407faa10 as *const ()), // sub_1407faa10
    Replacement::draft_throws(0x1407fabc0, "unknown_1407fabc0", fn_1407fabc0 as *const (), Throws::Caught), // sub_1407fabc0
    Replacement::draft_throws(0x1407fae20, "unknown_1407fae20", fn_1407fae20 as *const (), Throws::Caught), // sub_1407fae20
    Replacement::draft_throws(0x1407fb080, "unknown_1407fb080", fn_1407fb080 as *const (), Throws::Caught), // sub_1407fb080
    Replacement::draft_throws(0x1407fb2e0, "unknown_1407fb2e0", fn_1407fb2e0 as *const (), Throws::Caught), // sub_1407fb2e0
    Replacement::draft_throws(0x1407fb4d0, "unknown_1407fb4d0", fn_1407fb4d0 as *const (), Throws::Caught), // sub_1407fb4d0
    Replacement::draft_throws(0x1407fb610, "unknown_1407fb610", fn_1407fb610 as *const (), Throws::Caught), // sub_1407fb610
    Replacement::draft_throws(0x1407fb750, "unknown_1407fb750", fn_1407fb750 as *const (), Throws::Caught), // sub_1407fb750
    Replacement::draft_throws(0x1407fb890, "unknown_1407fb890", fn_1407fb890 as *const (), Throws::Caught), // sub_1407fb890
    Replacement::draft_throws(0x1407fbb00, "unknown_1407fbb00", fn_1407fbb00 as *const (), Throws::Caught), // sub_1407fbb00
    Replacement::draft_throws(0x1407fc080, "unknown_1407fc080", fn_1407fc080 as *const (), Throws::Caught), // sub_1407fc080
    Replacement::draft_throws(0x1407fc5c0, "unknown_1407fc5c0", fn_1407fc5c0 as *const (), Throws::Caught), // sub_1407fc5c0
    Replacement::draft_throws(0x1407fc910, "unknown_1407fc910", fn_1407fc910 as *const (), Throws::Caught), // sub_1407fc910
    Replacement::draft_throws(0x1407fcd80, "unknown_1407fcd80", fn_1407fcd80 as *const (), Throws::Caught), // sub_1407fcd80
    Replacement::draft_throws(0x1407fd1b0, "unknown_1407fd1b0", fn_1407fd1b0 as *const (), Throws::Caught), // sub_1407fd1b0
    Replacement::draft(0x1407fd500, "unknown_1407fd500", fn_1407fd500 as *const ()), // sub_1407fd500
    Replacement::draft(0x1407fd5f0, "unknown_1407fd5f0", fn_1407fd5f0 as *const ()), // sub_1407fd5f0
    Replacement::draft_throws(0x1407fd700, "unknown_1407fd700", fn_1407fd700 as *const (), Throws::Discard), // sub_1407fd700
    Replacement::draft_throws(0x1407fd870, "unknown_1407fd870", fn_1407fd870 as *const (), Throws::Discard), // sub_1407fd870
    Replacement::draft_throws(0x1407fd9e0, "unknown_1407fd9e0", fn_1407fd9e0 as *const (), Throws::Discard), // sub_1407fd9e0
    Replacement::draft_throws(0x1407fdb90, "unknown_1407fdb90", fn_1407fdb90 as *const (), Throws::Discard), // sub_1407fdb90
    Replacement::draft_throws(0x1407fdd30, "unknown_1407fdd30", fn_1407fdd30 as *const (), Throws::Caught), // sub_1407fdd30
    Replacement::draft_throws(0x1407fde90, "unknown_1407fde90", fn_1407fde90 as *const (), Throws::Caught), // sub_1407fde90
    Replacement::draft_throws(0x1407fdff0, "unknown_1407fdff0", fn_1407fdff0 as *const (), Throws::Caught), // sub_1407fdff0
    Replacement::draft(0x1407fe150, "unknown_1407fe150", fn_1407fe150 as *const ()), // sub_1407fe150
    Replacement::draft(0x1407fe2c0, "unknown_1407fe2c0", fn_1407fe2c0 as *const ()), // sub_1407fe2c0
    Replacement::draft(0x1407fe460, "unknown_1407fe460", fn_1407fe460 as *const ()), // sub_1407fe460
    Replacement::draft_throws(0x1407fe620, "unknown_1407fe620", fn_1407fe620 as *const (), Throws::Discard), // sub_1407fe620
    Replacement::draft_throws(0x1407fe7d0, "unknown_1407fe7d0", fn_1407fe7d0 as *const (), Throws::Discard), // sub_1407fe7d0
    Replacement::draft(0x1407fe910, "unknown_1407fe910", fn_1407fe910 as *const ()), // sub_1407fe910
    Replacement::draft(0x1407fe9b0, "unknown_1407fe9b0", fn_1407fe9b0 as *const ()), // sub_1407fe9b0
    Replacement::draft(0x1407fea40, "unknown_1407fea40", fn_1407fea40 as *const ()), // sub_1407fea40
    Replacement::draft_throws(0x1407feae0, "unknown_1407feae0", tpl_65670::<0x142442de0, 0x143a39638, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1407feae0
    Replacement::draft_throws(0x1407febc0, "unknown_1407febc0", fn_1407febc0 as *const (), Throws::Caught), // sub_1407febc0
    Replacement::draft_throws(0x1407feec0, "unknown_1407feec0", tpl_65672::<0x142442de0, 0x143a39638, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1407feec0
    Replacement::draft_throws(0x1407ff0e0, "unknown_1407ff0e0", fn_1407ff0e0 as *const (), Throws::Caught), // sub_1407ff0e0
    Replacement::draft_throws(0x1407ff3d0, "unknown_1407ff3d0", fn_1407ff3d0 as *const (), Throws::Caught), // sub_1407ff3d0
    Replacement::draft_throws(0x1407ffe90, "unknown_1407ffe90", fn_1407ffe90 as *const (), Throws::Caught), // sub_1407ffe90
    Replacement::draft_throws(0x140800920, "unknown_140800920", fn_140800920 as *const (), Throws::Caught), // sub_140800920
    Replacement::draft_throws(0x1408013a0, "unknown_1408013a0", fn_1408013a0 as *const (), Throws::Caught), // sub_1408013a0
    Replacement::draft_throws(0x1408015a0, "unknown_1408015a0", fn_1408015a0 as *const (), Throws::Caught), // sub_1408015a0
    Replacement::draft_throws(0x1408016e0, "unknown_1408016e0", fn_1408016e0 as *const (), Throws::Caught), // sub_1408016e0
    Replacement::draft(0x140801d00, "coregame_140801d00", fn_140801d00 as *const ()), // coregame::TimelineResource::coregame::TimelineResource::vfunc_0
    Replacement::draft(0x140801d80, "unknown_140801d80", fn_140801d80 as *const ()), // sub_140801d80
    Replacement::draft(0x140801e00, "unknown_140801e00", fn_140801e00 as *const ()), // sub_140801e00
    Replacement::draft(0x140801ea0, "unknown_140801ea0", tpl_8984::<0x14387d380, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140804770> as *const ()), // sub_140801ea0
    Replacement::draft(0x140801f50, "unknown_140801f50", tpl_8984::<0x14387d380, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140804890> as *const ()), // sub_140801f50
    Replacement::draft_throws(0x140802000, "unknown_140802000", fn_140802000 as *const (), Throws::Caught), // sub_140802000
    Replacement::draft_throws(0x1408021a0, "unknown_1408021a0", fn_1408021a0 as *const (), Throws::Discard), // sub_1408021a0
    Replacement::draft_throws(0x140802250, "unknown_140802250", fn_140802250 as *const (), Throws::Caught), // sub_140802250
    Replacement::draft_throws(0x1408023f0, "unknown_1408023f0", fn_1408023f0 as *const (), Throws::Caught), // sub_1408023f0
    Replacement::draft_throws(0x1408024a0, "unknown_1408024a0", fn_1408024a0 as *const (), Throws::Caught), // sub_1408024a0
    Replacement::draft_throws(0x140802550, "unknown_140802550", fn_140802550 as *const (), Throws::Caught), // sub_140802550
    Replacement::draft(0x140802600, "unknown_140802600", fn_140802600 as *const ()), // sub_140802600
    Replacement::draft(0x1408026b0, "unknown_1408026b0", fn_1408026b0 as *const ()), // sub_1408026b0
    Replacement::draft(0x140802740, "unknown_140802740", tpl_8984::<0x14387d380, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140805590> as *const ()), // sub_140802740
    Replacement::draft_throws(0x1408027f0, "unknown_1408027f0", fn_1408027f0 as *const (), Throws::Discard), // sub_1408027f0
    Replacement::draft_throws(0x140802880, "unknown_140802880", fn_140802880 as *const (), Throws::Caught), // sub_140802880
    Replacement::draft(0x140802990, "unknown_140802990", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432637a8, 0x8> as *const ()), // sub_140802990
    Replacement::draft(0x140802a10, "unknown_140802a10", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432637a8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140802a10
    Replacement::draft_throws(0x140802ab0, "unknown_140802ab0", fn_140802ab0 as *const (), Throws::Caught), // sub_140802ab0
    Replacement::draft_throws(0x140802b40, "unknown_140802b40", tpl_649::<0x14387d378, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140802b40
    Replacement::draft_throws(0x140802bc0, "unknown_140802bc0", fn_140802bc0 as *const (), Throws::Caught), // sub_140802bc0
    Replacement::draft_throws(0x140802fd0, "unknown_140802fd0", fn_140802fd0 as *const (), Throws::Caught), // sub_140802fd0
    Replacement::draft_throws(0x140803110, "unknown_140803110", fn_140803110 as *const (), Throws::Caught), // sub_140803110
    Replacement::draft_throws(0x140803250, "unknown_140803250", fn_140803250 as *const (), Throws::Caught), // sub_140803250
    Replacement::draft(0x140803390, "unknown_140803390", tpl_8994::<0x30, 0x38, 0x18> as *const ()), // sub_140803390
    Replacement::draft(0x1408033e0, "unknown_1408033e0", tpl_8994::<0x30, 0x38, 0x8> as *const ()), // sub_1408033e0
    Replacement::draft(0x140803430, "unknown_140803430", fn_140803430 as *const ()), // sub_140803430
    Replacement::draft_throws(0x140803700, "unknown_140803700", fn_140803700 as *const (), Throws::Caught), // sub_140803700
    Replacement::draft_throws(0x140803790, "unknown_140803790", fn_140803790 as *const (), Throws::Caught), // sub_140803790
    Replacement::draft_throws(0x140803d40, "unknown_140803d40", fn_140803d40 as *const (), Throws::Caught), // sub_140803d40
    Replacement::draft_throws(0x1408042b0, "unknown_1408042b0", fn_1408042b0 as *const (), Throws::Caught), // sub_1408042b0
    Replacement::draft_throws(0x1408046f0, "unknown_1408046f0", tpl_1908::<0x143881f80, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1408046f0
    Replacement::draft(0x140804770, "unknown_140804770", fn_140804770 as *const ()), // sub_140804770
    Replacement::draft(0x140804890, "unknown_140804890", fn_140804890 as *const ()), // sub_140804890
    Replacement::draft_throws(0x140804aa0, "unknown_140804aa0", fn_140804aa0 as *const (), Throws::Discard), // sub_140804aa0
    Replacement::draft(0x140804cb0, "unknown_140804cb0", fn_140804cb0 as *const ()), // sub_140804cb0
    Replacement::draft_throws(0x140804ea0, "unknown_140804ea0", fn_140804ea0 as *const (), Throws::Discard), // sub_140804ea0
    Replacement::draft_throws(0x140805040, "unknown_140805040", fn_140805040 as *const (), Throws::Discard), // sub_140805040
    Replacement::draft_throws(0x140805230, "unknown_140805230", fn_140805230 as *const (), Throws::Discard), // sub_140805230
    Replacement::draft(0x140805360, "unknown_140805360", fn_140805360 as *const ()), // sub_140805360
    Replacement::draft(0x140805590, "unknown_140805590", fn_140805590 as *const ()), // sub_140805590
    Replacement::draft_throws(0x1408057e0, "unknown_1408057e0", fn_1408057e0 as *const (), Throws::Discard), // sub_1408057e0
    Replacement::draft(0x140805a00, "unknown_140805a00", fn_140805a00 as *const ()), // sub_140805a00
    Replacement::draft(0x140805af0, "unknown_140805af0", fn_140805af0 as *const ()), // sub_140805af0
    Replacement::draft_throws(0x140805bc0, "unknown_140805bc0", fn_140805bc0 as *const (), Throws::Caught), // sub_140805bc0
    Replacement::draft_throws(0x140805e70, "unknown_140805e70", fn_140805e70 as *const (), Throws::Caught), // sub_140805e70
    Replacement::draft_throws(0x1408060f0, "unknown_1408060f0", fn_1408060f0 as *const (), Throws::Caught), // sub_1408060f0
    Replacement::draft_throws(0x140806360, "unknown_140806360", fn_140806360 as *const (), Throws::Discard), // sub_140806360
    Replacement::draft(0x140806470, "unknown_140806470", tpl_66164::<0x8, 0xc, 0x140808110, 0x14183acc2, 0x1401ffe00> as *const ()), // sub_140806470
    Replacement::draft_throws(0x140806530, "unknown_140806530", fn_140806530 as *const (), Throws::Caught), // sub_140806530
    Replacement::draft_throws(0x140806850, "unknown_140806850", tpl_66331::<0x142442de0, 0x143a3a758, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140806850
    Replacement::draft_throws(0x140806ab0, "unknown_140806ab0", fn_140806ab0 as *const (), Throws::Caught), // sub_140806ab0
    Replacement::draft_throws(0x140806e80, "unknown_140806e80", tpl_2502::<0x20, 0x8, 0x140808030> as *const (), Throws::Discard), // sub_140806e80
    Replacement::draft(0x140806ed0, "unknown_140806ed0", fn_140806ed0 as *const ()), // sub_140806ed0
    Replacement::draft_throws(0x140806f10, "unknown_140806f10", fn_140806f10 as *const (), Throws::Caught), // sub_140806f10
    Replacement::draft_throws(0x140807200, "unknown_140807200", fn_140807200 as *const (), Throws::Caught), // sub_140807200
    Replacement::draft_throws(0x1408074c0, "unknown_1408074c0", fn_1408074c0 as *const (), Throws::Caught), // sub_1408074c0
    Replacement::draft_throws(0x140807780, "unknown_140807780", fn_140807780 as *const (), Throws::Caught), // sub_140807780
    Replacement::draft_throws(0x140807a10, "unknown_140807a10", fn_140807a10 as *const (), Throws::Caught), // sub_140807a10
    Replacement::draft_throws(0x140807cd0, "unknown_140807cd0", fn_140807cd0 as *const (), Throws::Caught), // sub_140807cd0
    Replacement::draft_throws(0x140807f60, "unknown_140807f60", fn_140807f60 as *const (), Throws::Caught), // sub_140807f60
    Replacement::draft_throws(0x140808030, "unknown_140808030", fn_140808030 as *const (), Throws::Caught), // sub_140808030
    Replacement::draft(0x140808110, "unknown_140808110", tpl_9010::<0x8, 0x1401dec00, 0x1401ffd50, 0xc> as *const ()), // sub_140808110
    Replacement::draft_throws(0x140808190, "unknown_140808190", fn_140808190 as *const (), Throws::Caught), // sub_140808190
    Replacement::draft_throws(0x140808660, "unknown_140808660", fn_140808660 as *const (), Throws::Caught), // sub_140808660
    Replacement::draft_throws(0x140808b60, "unknown_140808b60", fn_140808b60 as *const (), Throws::Caught), // sub_140808b60
    Replacement::draft_throws(0x140808fe0, "unknown_140808fe0", fn_140808fe0 as *const (), Throws::Caught), // sub_140808fe0
    Replacement::draft_throws(0x140809490, "unknown_140809490", fn_140809490 as *const (), Throws::Caught), // sub_140809490
    Replacement::draft_throws(0x140809910, "unknown_140809910", fn_140809910 as *const (), Throws::Caught), // sub_140809910
    Replacement::draft_throws(0x140809dc0, "unknown_140809dc0", fn_140809dc0 as *const (), Throws::Caught), // sub_140809dc0
    Replacement::draft_throws(0x140809fc0, "unknown_140809fc0", fn_140809fc0 as *const (), Throws::Discard), // sub_140809fc0
    Replacement::draft_throws(0x140809fe0, "unknown_140809fe0", fn_140809fe0 as *const (), Throws::Caught), // sub_140809fe0
    Replacement::draft_throws(0x14080aad0, "unknown_14080aad0", fn_14080aad0 as *const (), Throws::Caught), // sub_14080aad0
    Replacement::draft_throws(0x14080b560, "unknown_14080b560", fn_14080b560 as *const (), Throws::Discard), // sub_14080b560
    Replacement::draft_throws(0x14080b580, "unknown_14080b580", fn_14080b580 as *const (), Throws::Caught), // sub_14080b580
    Replacement::draft_throws(0x14080c000, "unknown_14080c000", tpl_2834::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2cc0> as *const (), Throws::Discard), // sub_14080c000
    Replacement::draft_throws(0x14080c050, "unknown_14080c050", fn_14080c050 as *const (), Throws::Caught), // sub_14080c050
    Replacement::draft_throws(0x14080c190, "unknown_14080c190", tpl_65670::<0x142442de0, 0x143a3a758, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_14080c190
    Replacement::draft_throws(0x14080c270, "unknown_14080c270", tpl_7370::<0x1401ffe00, 0x140808110, 0x14080c190, 0x142442de0, 0x143a3a758, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_14080c270
    Replacement::draft_throws(0x14080c510, "unknown_14080c510", fn_14080c510 as *const (), Throws::Caught), // sub_14080c510
    Replacement::draft(0x14080cac0, "unknown_14080cac0", fn_14080cac0 as *const ()), // sub_14080cac0
    Replacement::draft(0x14080cb20, "content_14080cb20", fn_14080cb20 as *const ()), // content::TimelineEvent::content::TimelineEvent::vfunc_0
    Replacement::draft(0x14080cbc0, "content_14080cbc0", tpl_500::<0x2c> as *const ()), // content::SpotLightComponent::content::SpotLightComponent::vfunc_4
    Replacement::draft(0x14080cbd0, "content_14080cbd0", tpl_586::<0x44> as *const ()), // content::SpotLightComponent::content::SpotLightComponent::vfunc_8
    Replacement::draft(0x14080cbe0, "content_14080cbe0", fn_14080cbe0 as *const ()), // content::SpotLightComponent::content::SpotLightComponent::vfunc_18
    Replacement::draft(0x14080cd60, "content_14080cd60", tpl_9016::<0x90, 0x8f> as *const ()), // content::SpotLightComponent::content::SpotLightComponent::vfunc_21
    Replacement::draft(0x14080cff0, "content_14080cff0", fn_14080cff0 as *const ()), // content::TimelineTrack::content::TimelineTrack::vfunc_0
    Replacement::draft(0x14080d160, "content_14080d160", fn_14080d160 as *const ()), // sub_14080d160
    Replacement::draft(0x14080d210, "content_14080d210", fn_14080d210 as *const ()), // sub_14080d210
    Replacement::draft(0x14080d2d0, "content_14080d2d0", fn_14080d2d0 as *const ()), // sub_14080d2d0
    Replacement::draft(0x14080d390, "content_14080d390", tpl_9021::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x143264978, 0xffffffffffffffe8, 0x28, 0x38, 0x58> as *const ()), // sub_14080d390
    Replacement::draft(0x14080d440, "content_14080d440", fn_14080d440 as *const ()), // sub_14080d440
    Replacement::draft(0x14080d560, "content_14080d560", fn_14080d560 as *const ()), // sub_14080d560
    Replacement::draft_throws(0x14080d7a0, "content_14080d7a0", fn_14080d7a0 as *const (), Throws::Discard), // sub_14080d7a0
    Replacement::draft_throws(0x14080d9d0, "content_14080d9d0", fn_14080d9d0 as *const (), Throws::Discard), // sub_14080d9d0
    Replacement::draft_throws(0x14080db90, "content_14080db90", fn_14080db90 as *const (), Throws::Discard), // sub_14080db90
    Replacement::draft_throws(0x14080ddf0, "content_14080ddf0", fn_14080ddf0 as *const (), Throws::Discard), // sub_14080ddf0
    Replacement::draft_throws(0x14080df40, "content_14080df40", fn_14080df40 as *const (), Throws::Caught), // sub_14080df40
    Replacement::draft_throws(0x14080e0f0, "content_14080e0f0", fn_14080e0f0 as *const (), Throws::Caught), // sub_14080e0f0
    Replacement::draft_throws(0x14080e2a0, "content_14080e2a0", fn_14080e2a0 as *const (), Throws::Caught), // sub_14080e2a0
    Replacement::draft(0x14080e450, "content_14080e450", fn_14080e450 as *const ()), // sub_14080e450
    Replacement::draft(0x14080e690, "content_14080e690", fn_14080e690 as *const ()), // sub_14080e690
    Replacement::draft(0x14080e8c0, "content_14080e8c0", fn_14080e8c0 as *const ()), // sub_14080e8c0
    Replacement::draft_throws(0x14080eb00, "content_14080eb00", fn_14080eb00 as *const (), Throws::Discard), // sub_14080eb00
    Replacement::draft_throws(0x14080ed60, "content_14080ed60", fn_14080ed60 as *const (), Throws::Discard), // sub_14080ed60
    Replacement::draft(0x14080ef00, "content_14080ef00", fn_14080ef00 as *const ()), // sub_14080ef00
    Replacement::draft(0x14080ef80, "content_14080ef80", fn_14080ef80 as *const ()), // sub_14080ef80
    Replacement::draft(0x14080f070, "content_14080f070", fn_14080f070 as *const ()), // sub_14080f070
    Replacement::draft(0x14080f1d0, "content_14080f1d0", tpl_9031::<0x14387d470, 0x14243b0b0, 0x14081bf10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14080f1d0
    Replacement::draft(0x14080f240, "content_14080f240", fn_14080f240 as *const ()), // sub_14080f240
    Replacement::draft_throws(0x14080f390, "content_14080f390", fn_14080f390 as *const (), Throws::Caught), // sub_14080f390
    Replacement::draft(0x14080f5d0, "content_14080f5d0", tpl_9034::<0x14387d470, 0x14243b0b0, 0x14081c750, 0x60, 0x140835a70, 0x338, 0x68, 0x20, 0xf0, 0x38, 0xf4, 0x8> as *const ()), // sub_14080f5d0
    Replacement::draft_throws(0x14080f680, "content_14080f680", fn_14080f680 as *const (), Throws::Caught), // sub_14080f680
    Replacement::draft(0x14080f8e0, "content_14080f8e0", fn_14080f8e0 as *const ()), // sub_14080f8e0
    Replacement::draft_throws(0x14080f990, "content_14080f990", fn_14080f990 as *const (), Throws::Caught), // sub_14080f990
    Replacement::draft_throws(0x14080fca0, "content_14080fca0", fn_14080fca0 as *const (), Throws::Caught), // sub_14080fca0
    Replacement::draft_throws(0x14080ff80, "content_14080ff80", fn_14080ff80 as *const (), Throws::Caught), // sub_14080ff80
    Replacement::draft(0x140810250, "content_140810250", fn_140810250 as *const ()), // sub_140810250
    Replacement::draft(0x140810330, "content_140810330", fn_140810330 as *const ()), // sub_140810330
    Replacement::draft(0x1408103e0, "content_1408103e0", tpl_9034::<0x14387d470, 0x14243b0b0, 0x14081dec0, 0x60, 0x140835ed0, 0x338, 0x68, 0xc0, 0xf0, 0xd8, 0xf4, 0x8> as *const ()), // sub_1408103e0
    Replacement::draft_throws(0x1408104a0, "content_1408104a0", fn_1408104a0 as *const (), Throws::Discard), // sub_1408104a0
    Replacement::draft_throws(0x140810560, "content_140810560", fn_140810560 as *const (), Throws::Caught), // sub_140810560
    Replacement::draft(0x140810760, "content_140810760", tpl_9044::<0x60, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc> as *const ()), // sub_140810760
    Replacement::draft(0x140810800, "content_140810800", tpl_7173::<0x143263d10, 0xf0, 0x1, 0x14020f270, 0xf4, 0x143263d28> as *const ()), // sub_140810800
    Replacement::draft(0x140810880, "content_140810880", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140810910> as *const ()), // sub_140810880
    Replacement::draft(0x140810910, "content_140810910", fn_140810910 as *const ()), // sub_140810910
    Replacement::draft(0x1408109b0, "content_1408109b0", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140810910> as *const ()), // sub_1408109b0
    Replacement::draft(0x140810a60, "content_140810a60", fn_140810a60 as *const ()), // sub_140810a60
    Replacement::draft(0x140810ae0, "content_140810ae0", fn_140810ae0 as *const ()), // sub_140810ae0
    Replacement::draft_throws(0x140810c20, "content_140810c20", fn_140810c20 as *const (), Throws::Discard), // sub_140810c20
    Replacement::draft(0x140810d50, "content_140810d50", fn_140810d50 as *const ()), // sub_140810d50
    Replacement::draft_throws(0x140810e20, "content_140810e20", fn_140810e20 as *const (), Throws::Discard), // sub_140810e20
    Replacement::draft(0x140810f80, "content_140810f80", fn_140810f80 as *const ()), // sub_140810f80
    Replacement::draft_throws(0x140811070, "content_140811070", fn_140811070 as *const (), Throws::Caught), // sub_140811070
    Replacement::draft_throws(0x140811340, "content_140811340", fn_140811340 as *const (), Throws::Caught), // sub_140811340
    Replacement::draft_throws(0x1408115e0, "content_1408115e0", fn_1408115e0 as *const (), Throws::Caught), // sub_1408115e0
    Replacement::draft(0x140811880, "content_140811880", fn_140811880 as *const ()), // sub_140811880
    Replacement::draft(0x140811960, "content_140811960", fn_140811960 as *const ()), // sub_140811960
    Replacement::draft(0x140811a40, "content_140811a40", fn_140811a40 as *const ()), // sub_140811a40
    Replacement::draft_throws(0x140811b30, "content_140811b30", fn_140811b30 as *const (), Throws::Discard), // sub_140811b30
    Replacement::draft_throws(0x140811c20, "content_140811c20", fn_140811c20 as *const (), Throws::Caught), // sub_140811c20
    Replacement::draft(0x140811e00, "content_140811e00", tpl_7173::<0x143263d50, 0x128, 0x2, 0x14020f270, 0x12c, 0x143263d60> as *const ()), // sub_140811e00
    Replacement::draft(0x140811e80, "content_140811e80", fn_140811e80 as *const ()), // sub_140811e80
    Replacement::draft(0x140811f50, "content_140811f50", fn_140811f50 as *const ()), // sub_140811f50
    Replacement::draft(0x1408120a0, "content_1408120a0", fn_1408120a0 as *const ()), // sub_1408120a0
    Replacement::draft_throws(0x140812140, "content_140812140", tpl_9063::<0x14387d440, 0x14387d390, 0x1401e9fa0, 0x14081c440, 0x328, 0x338, 0x60, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140812140
    Replacement::draft(0x1408121d0, "content_1408121d0", tpl_9064::<0x14387d390, 0x14243b0b0, 0x14081c750, 0x338, 0x60, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408121d0
    Replacement::draft_throws(0x140812250, "content_140812250", tpl_9065::<0x14387d440, 0x14387d390, 0x1401e9b60, 0x14081c990, 0x328, 0x338, 0x120, 0x60, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140812250
    Replacement::draft(0x1408122e0, "content_1408122e0", fn_1408122e0 as *const ()), // sub_1408122e0
    Replacement::draft_throws(0x140812360, "content_140812360", fn_140812360 as *const (), Throws::Caught), // sub_140812360
    Replacement::draft_throws(0x140812560, "content_140812560", fn_140812560 as *const (), Throws::Caught), // sub_140812560
    Replacement::draft_throws(0x140812730, "content_140812730", fn_140812730 as *const (), Throws::Caught), // sub_140812730
    Replacement::draft(0x140812900, "content_140812900", tpl_9070::<0x14387d390, 0x14243b0b0, 0x14081d9f0, 0x338, 0x60, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140812900
    Replacement::draft(0x140812980, "content_140812980", tpl_67834::<0x14387d440, 0x14387d390, 0x1401e9970, 0x14081dc70, 0x60, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140812980
    Replacement::draft(0x140812a00, "content_140812a00", fn_140812a00 as *const ()), // sub_140812a00
    Replacement::draft_throws(0x140812a90, "content_140812a90", tpl_9073::<0x14387d440, 0x14387d390, 0x1401e9770, 0x14081e120, 0x338, 0x60, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140812a90
    Replacement::draft_throws(0x140812b00, "content_140812b00", fn_140812b00 as *const (), Throws::Discard), // sub_140812b00
    Replacement::draft(0x140812bc0, "content_140812bc0", tpl_9075::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x8, 0x1432648c8, 0xb> as *const ()), // sub_140812bc0
    Replacement::draft(0x140812c90, "content_140812c90", tpl_9076::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432648c8, 0xfffffffffffffff4, 0x14, 0x34, 0xfffffffffffffffc, 0x4> as *const ()), // sub_140812c90
    Replacement::draft(0x140812d70, "content_140812d70", tpl_9031::<0x14387d3b0, 0x14243b0b0, 0x14081bf10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140812d70
    Replacement::draft(0x140812de0, "content_140812de0", tpl_9031::<0x14387d3b0, 0x14243b0b0, 0x14081c0d0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140812de0
    Replacement::draft(0x140812e50, "content_140812e50", tpl_9031::<0x14387d3b0, 0x14243b0b0, 0x14081c750, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140812e50
    Replacement::draft_throws(0x140812ec0, "content_140812ec0", fn_140812ec0 as *const (), Throws::Caught), // sub_140812ec0
    Replacement::draft_throws(0x140812f60, "content_140812f60", fn_140812f60 as *const (), Throws::Caught), // sub_140812f60
    Replacement::draft_throws(0x140813000, "content_140813000", fn_140813000 as *const (), Throws::Caught), // sub_140813000
    Replacement::draft(0x1408130a0, "content_1408130a0", tpl_9031::<0x14387d3b0, 0x14243b0b0, 0x14081d9f0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408130a0
    Replacement::draft(0x140813110, "content_140813110", tpl_9031::<0x14387d3b0, 0x14243b0b0, 0x14081dec0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140813110
    Replacement::draft_throws(0x140813180, "content_140813180", tpl_7326::<0x14387d3b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14081e380> as *const (), Throws::Discard), // sub_140813180
    Replacement::draft(0x140813220, "content_140813220", fn_140813220 as *const ()), // sub_140813220
    Replacement::draft(0x140813320, "content_140813320", fn_140813320 as *const ()), // sub_140813320
    Replacement::draft(0x140813480, "content_140813480", fn_140813480 as *const ()), // sub_140813480
    Replacement::draft(0x140813500, "content_140813500", fn_140813500 as *const ()), // sub_140813500
    Replacement::draft_throws(0x140813610, "content_140813610", fn_140813610 as *const (), Throws::Discard), // sub_140813610
    Replacement::draft(0x1408136f0, "content_1408136f0", fn_1408136f0 as *const ()), // sub_1408136f0
    Replacement::draft_throws(0x140813790, "content_140813790", fn_140813790 as *const (), Throws::Discard), // sub_140813790
    Replacement::draft(0x140813880, "content_140813880", fn_140813880 as *const ()), // sub_140813880
    Replacement::draft_throws(0x140813990, "content_140813990", fn_140813990 as *const (), Throws::Caught), // sub_140813990
    Replacement::draft_throws(0x140813be0, "content_140813be0", fn_140813be0 as *const (), Throws::Caught), // sub_140813be0
    Replacement::draft_throws(0x140813e00, "content_140813e00", fn_140813e00 as *const (), Throws::Caught), // sub_140813e00
    Replacement::draft(0x140814010, "content_140814010", fn_140814010 as *const ()), // sub_140814010
    Replacement::draft(0x1408140c0, "content_1408140c0", fn_1408140c0 as *const ()), // sub_1408140c0
    Replacement::draft(0x140814160, "content_140814160", fn_140814160 as *const ()), // sub_140814160
    Replacement::draft_throws(0x140814210, "content_140814210", fn_140814210 as *const (), Throws::Discard), // sub_140814210
    Replacement::draft_throws(0x1408142b0, "content_1408142b0", fn_1408142b0 as *const (), Throws::Discard), // sub_1408142b0
    Replacement::draft(0x1408143c0, "content_1408143c0", fn_1408143c0 as *const ()), // sub_1408143c0
    Replacement::draft(0x140814480, "content_140814480", fn_140814480 as *const ()), // sub_140814480
    Replacement::draft(0x1408145d0, "content_1408145d0", tpl_8125::<0x14387d3b8, 0x14243b0b0, 0x14081c0d0, 0x60, 0x338, 0x1426d4580, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408145d0
    Replacement::draft_throws(0x140814670, "content_140814670", tpl_8126::<0x14387d424, 0x14387d3b8, 0x1401e9fa0, 0x14081c440, 0x328, 0x338, 0x60, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140814670
    Replacement::draft(0x140814700, "content_140814700", tpl_7857::<0x14387d3b8, 0x14243b0b0, 0x14081c750, 0x338, 0x60, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140814700
    Replacement::draft_throws(0x140814780, "content_140814780", tpl_8127::<0x14387d424, 0x14387d3b8, 0x1401e9b60, 0x14081c990, 0x328, 0x338, 0x120, 0x60, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140814780
    Replacement::draft(0x140814810, "content_140814810", fn_140814810 as *const ()), // sub_140814810
    Replacement::draft_throws(0x140814890, "content_140814890", fn_140814890 as *const (), Throws::Caught), // sub_140814890
    Replacement::draft_throws(0x140814960, "content_140814960", fn_140814960 as *const (), Throws::Caught), // sub_140814960
    Replacement::draft_throws(0x140814a30, "content_140814a30", fn_140814a30 as *const (), Throws::Caught), // sub_140814a30
    Replacement::draft(0x140814b00, "content_140814b00", tpl_8129::<0x14387d3b8, 0x14243b0b0, 0x14081d9f0, 0x338, 0x60, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140814b00
    Replacement::draft(0x140814b80, "content_140814b80", tpl_7856::<0x14387d424, 0x14387d3b8, 0x1401e9970, 0x14081dc70, 0x338, 0x60, 0x120, 0x1401e9860> as *const ()), // sub_140814b80
    Replacement::draft(0x140814bf0, "content_140814bf0", tpl_7857::<0x14387d3b8, 0x14243b0b0, 0x14081dec0, 0x338, 0x60, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140814bf0
    Replacement::draft_throws(0x140814c70, "content_140814c70", tpl_8130::<0x14387d424, 0x14387d3b8, 0x1401e9770, 0x14081e120, 0x338, 0x60, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140814c70
    Replacement::draft_throws(0x140814cf0, "content_140814cf0", tpl_8131::<0x14387d3b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14081e380, 0x60, 0x143263b88, 0x1, 0x14020f0f0> as *const (), Throws::Discard), // sub_140814cf0
    Replacement::draft(0x140814dc0, "content_140814dc0", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140814e50> as *const ()), // sub_140814dc0
    Replacement::draft(0x140814e50, "content_140814e50", fn_140814e50 as *const ()), // sub_140814e50
    Replacement::draft(0x140814f00, "content_140814f00", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140814e50> as *const ()), // sub_140814f00
    Replacement::draft(0x140814fb0, "content_140814fb0", tpl_9096::<0x14387d478, 0x14243b0b0, 0x14081bf10, 0x100, 0x1408375a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140814fb0
    Replacement::draft(0x140815030, "content_140815030", fn_140815030 as *const ()), // sub_140815030
    Replacement::draft_throws(0x140815250, "content_140815250", fn_140815250 as *const (), Throws::Caught), // sub_140815250
    Replacement::draft(0x140815530, "content_140815530", fn_140815530 as *const ()), // sub_140815530
    Replacement::draft_throws(0x140815630, "content_140815630", fn_140815630 as *const (), Throws::Caught), // sub_140815630
    Replacement::draft(0x140815940, "content_140815940", fn_140815940 as *const ()), // sub_140815940
    Replacement::draft_throws(0x140815ac0, "content_140815ac0", fn_140815ac0 as *const (), Throws::Caught), // sub_140815ac0
    Replacement::draft_throws(0x140815ce0, "content_140815ce0", fn_140815ce0 as *const (), Throws::Caught), // sub_140815ce0
    Replacement::draft_throws(0x140815f00, "content_140815f00", fn_140815f00 as *const (), Throws::Caught), // sub_140815f00
    Replacement::draft(0x140816120, "content_140816120", fn_140816120 as *const ()), // sub_140816120
    Replacement::draft(0x140816270, "content_140816270", fn_140816270 as *const ()), // sub_140816270
    Replacement::draft(0x140816370, "content_140816370", fn_140816370 as *const ()), // sub_140816370
    Replacement::draft_throws(0x140816480, "content_140816480", fn_140816480 as *const (), Throws::Discard), // sub_140816480
    Replacement::draft_throws(0x1408165a0, "content_1408165a0", fn_1408165a0 as *const (), Throws::Caught), // sub_1408165a0
    Replacement::draft(0x140816840, "content_140816840", tpl_9044::<0xf8, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc> as *const ()), // sub_140816840
    Replacement::draft(0x1408168e0, "content_1408168e0", tpl_7173::<0x143263bf8, 0x130, 0x1, 0x14020f270, 0x134, 0x143263c08> as *const ()), // sub_1408168e0
    Replacement::draft(0x140816960, "content_140816960", fn_140816960 as *const ()), // sub_140816960
    Replacement::draft(0x140816a30, "content_140816a30", fn_140816a30 as *const ()), // sub_140816a30
    Replacement::draft(0x140816b20, "content_140816b20", fn_140816b20 as *const ()), // sub_140816b20
    Replacement::draft(0x140816bc0, "content_140816bc0", tpl_9111::<0x14387d3f0, 0x14243b0b0, 0x14081c0d0, 0x60, 0x140838ee0, 0x318, 0x328, 0x320, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140816bc0
    Replacement::draft_throws(0x140816c70, "content_140816c70", fn_140816c70 as *const (), Throws::Discard), // sub_140816c70
    Replacement::draft(0x140816d40, "content_140816d40", fn_140816d40 as *const ()), // sub_140816d40
    Replacement::draft_throws(0x140816e10, "content_140816e10", fn_140816e10 as *const (), Throws::Discard), // sub_140816e10
    Replacement::draft(0x140816ed0, "content_140816ed0", fn_140816ed0 as *const ()), // sub_140816ed0
    Replacement::draft_throws(0x140816f50, "content_140816f50", fn_140816f50 as *const (), Throws::Caught), // sub_140816f50
    Replacement::draft_throws(0x140817170, "content_140817170", fn_140817170 as *const (), Throws::Caught), // sub_140817170
    Replacement::draft_throws(0x140817360, "content_140817360", fn_140817360 as *const (), Throws::Caught), // sub_140817360
    Replacement::draft(0x140817550, "content_140817550", fn_140817550 as *const ()), // sub_140817550
    Replacement::draft(0x140817610, "content_140817610", fn_140817610 as *const ()), // sub_140817610
    Replacement::draft(0x1408176b0, "content_1408176b0", tpl_9111::<0x14387d3f0, 0x14243b0b0, 0x14081dec0, 0x60, 0x140839840, 0x318, 0x328, 0x320, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1408176b0
    Replacement::draft_throws(0x140817760, "content_140817760", fn_140817760 as *const (), Throws::Discard), // sub_140817760
    Replacement::draft_throws(0x140817800, "content_140817800", fn_140817800 as *const (), Throws::Discard), // sub_140817800
    Replacement::draft(0x1408178b0, "content_1408178b0", tpl_9122::<0x78, 0x10, 0x18, 0xffffffffffffffa0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x8, 0x143266a38> as *const ()), // sub_1408178b0
    Replacement::draft(0x140817980, "content_140817980", tpl_9123::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143266a38, 0xfffffffffffffff4, 0x14, 0x34, 0x54, 0xfffffffffffffffc> as *const ()), // sub_140817980
    Replacement::draft(0x140817a70, "content_140817a70", tpl_9031::<0x14387d480, 0x14243b0b0, 0x14081bf10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140817a70
    Replacement::draft(0x140817ae0, "content_140817ae0", fn_140817ae0 as *const ()), // sub_140817ae0
    Replacement::draft_throws(0x140817ba0, "content_140817ba0", fn_140817ba0 as *const (), Throws::Discard), // sub_140817ba0
    Replacement::draft(0x140817c40, "content_140817c40", fn_140817c40 as *const ()), // sub_140817c40
    Replacement::draft_throws(0x140817cd0, "content_140817cd0", fn_140817cd0 as *const (), Throws::Discard), // sub_140817cd0
    Replacement::draft(0x140817d80, "content_140817d80", fn_140817d80 as *const ()), // sub_140817d80
    Replacement::draft_throws(0x140817e10, "content_140817e10", fn_140817e10 as *const (), Throws::Caught), // sub_140817e10
    Replacement::draft_throws(0x140818040, "content_140818040", fn_140818040 as *const (), Throws::Caught), // sub_140818040
    Replacement::draft_throws(0x140818240, "content_140818240", fn_140818240 as *const (), Throws::Caught), // sub_140818240
    Replacement::draft(0x140818430, "content_140818430", fn_140818430 as *const ()), // sub_140818430
    Replacement::draft(0x1408184d0, "content_1408184d0", fn_1408184d0 as *const ()), // sub_1408184d0
    Replacement::draft(0x140818560, "content_140818560", fn_140818560 as *const ()), // sub_140818560
    Replacement::draft_throws(0x140818600, "content_140818600", fn_140818600 as *const (), Throws::Discard), // sub_140818600
    Replacement::draft_throws(0x140818680, "content_140818680", fn_140818680 as *const (), Throws::Discard), // sub_140818680
    Replacement::draft(0x140818750, "content_140818750", fn_140818750 as *const ()), // sub_140818750
    Replacement::draft(0x140818860, "content_140818860", fn_140818860 as *const ()), // sub_140818860
    Replacement::draft(0x140818a50, "content_140818a50", fn_140818a50 as *const ()), // sub_140818a50
    Replacement::draft_throws(0x140818b60, "content_140818b60", fn_140818b60 as *const (), Throws::Discard), // sub_140818b60
    Replacement::draft(0x140818c70, "content_140818c70", tpl_9139::<0x14387d3e8, 0x14243b0b0, 0x14081c750, 0x338, 0x60, 0x20, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140818c70
    Replacement::draft_throws(0x140818d00, "content_140818d00", tpl_67896::<0x14387d3a0, 0x14387d3e8, 0x1401e9b60, 0x14081c990, 0x60, 0x1426cdcb0, 0x328, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0xe8, 0x1401e9a20> as *const (), Throws::Discard), // sub_140818d00
    Replacement::draft(0x140818e10, "content_140818e10", fn_140818e10 as *const ()), // sub_140818e10
    Replacement::draft_throws(0x140818ea0, "content_140818ea0", fn_140818ea0 as *const (), Throws::Caught), // sub_140818ea0
    Replacement::draft_throws(0x140819290, "content_140819290", fn_140819290 as *const (), Throws::Caught), // sub_140819290
    Replacement::draft_throws(0x140819620, "content_140819620", fn_140819620 as *const (), Throws::Caught), // sub_140819620
    Replacement::draft(0x140819990, "content_140819990", fn_140819990 as *const ()), // sub_140819990
    Replacement::draft(0x140819a70, "content_140819a70", tpl_67902::<0x14387d3a0, 0x14387d3e8, 0x1401e9970, 0x14081dc70, 0x338, 0x60, 0xc0, 0xe8, 0x1401e9860> as *const ()), // sub_140819a70
    Replacement::draft(0x140819af0, "content_140819af0", tpl_9139::<0x14387d3e8, 0x14243b0b0, 0x14081dec0, 0x338, 0x60, 0xc0, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140819af0
    Replacement::draft_throws(0x140819b80, "content_140819b80", tpl_67903::<0x14387d3a0, 0x14387d3e8, 0x1401e9770, 0x14081e120, 0x338, 0x60, 0x1424b4cb0, 0xe8, 0x1401e9590> as *const (), Throws::Discard), // sub_140819b80
    Replacement::draft_throws(0x140819c00, "content_140819c00", fn_140819c00 as *const (), Throws::Discard), // sub_140819c00
    Replacement::draft(0x140819cf0, "content_140819cf0", tpl_9122::<0x78, 0x10, 0x18, 0xffffffffffffffa0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x60, 0x8, 0x143266f18> as *const ()), // sub_140819cf0
    Replacement::draft(0x140819dc0, "content_140819dc0", tpl_9123::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143266f18, 0xfffffffffffffff4, 0x14, 0x34, 0x54, 0xfffffffffffffffc> as *const ()), // sub_140819dc0
    Replacement::draft(0x140819eb0, "content_140819eb0", tpl_9031::<0x14387d3c0, 0x14243b0b0, 0x14081bf10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140819eb0
    Replacement::draft(0x140819f20, "content_140819f20", fn_140819f20 as *const ()), // sub_140819f20
    Replacement::draft_throws(0x14081a030, "content_14081a030", fn_14081a030 as *const (), Throws::Discard), // sub_14081a030
    Replacement::draft(0x14081a120, "content_14081a120", tpl_9149::<0x14387d3c0, 0x14243b0b0, 0x14081c750, 0x338, 0x60, 0x40, 0x64, 0x68, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081a120
    Replacement::draft_throws(0x14081a1c0, "content_14081a1c0", fn_14081a1c0 as *const (), Throws::Discard), // sub_14081a1c0
];
