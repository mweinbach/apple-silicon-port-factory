#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x141091c00, "unknown_141091c00", fn_141091c00 as *const ()), // sub_141091c00
    Replacement::draft(0x141091db0, "unknown_141091db0", fn_141091db0 as *const ()), // sub_141091db0
    Replacement::draft(0x141092080, "unknown_141092080", fn_141092080 as *const ()), // sub_141092080
    Replacement::draft(0x141092320, "unknown_141092320", tpl_62755::<0x88, 0x10088, 0x141093950, 0x141f297f0> as *const ()), // sub_141092320
    Replacement::draft(0x141092400, "unknown_141092400", tpl_1917::<0x143c8d904, 0x143c904a0, 0x141093800, 0x141092510> as *const ()), // sub_141092400
    Replacement::draft(0x141092510, "unknown_141092510", tpl_1918::<0x18, 0x20, 0x40, 0x8, 0x141f2c080> as *const ()), // sub_141092510
    Replacement::draft(0x1410928d0, "unknown_1410928d0", tpl_62755::<0x88, 0x10088, 0x1410934a0, 0x141f263f0> as *const ()), // sub_1410928d0
    Replacement::draft(0x1410929b0, "unknown_1410929b0", tpl_19667::<0x88, 0x10088, 0x1410930c0, 0x141092a60> as *const ()), // sub_1410929b0
    Replacement::draft(0x141092a60, "unknown_141092a60", tpl_19668::<0x18, 0x20, 0x40, 0x60, 0x8, 0x141f25ce0> as *const ()), // sub_141092a60
    Replacement::draft_throws(0x141092b80, "unknown_141092b80", tpl_3011::<0x1411e9f50, 0x141099960, 0x143cd84e0, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141098210, 0x1401de410> as *const (), Throws::Discard), // sub_141092b80
    Replacement::draft_throws(0x141092d40, "unknown_141092d40", tpl_3011::<0x1411e9f50, 0x141099870, 0x143cd7f90, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141098280, 0x1401de410> as *const (), Throws::Discard), // sub_141092d40
    Replacement::draft_throws(0x141092f00, "unknown_141092f00", tpl_3011::<0x1411e9f50, 0x141099780, 0x143cd83ac, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410982f0, 0x1401de410> as *const (), Throws::Discard), // sub_141092f00
    Replacement::draft_throws(0x1410933c0, "unknown_1410933c0", tpl_3146::<0x143c8c3fc, 0x143c8c3f4, 0x10, 0x1420a25b0, 0x141834a78, 0x143c900d8, 0x143c8c3f0, 0x143c8c3f8, 0x141834a0c> as *const (), Throws::Discard), // sub_1410933c0
    Replacement::draft(0x141093b50, "unknown_141093b50", tpl_60530::<0x1426467e8, 0x143c9f8e8, 0x142489ee0, 0x143cd65c0, 0x141093b79> as *const ()), // sub_141093b50
    Replacement::draft_throws(0x141093d10, "unknown_141093d10", fn_141093d10 as *const (), Throws::Discard), // sub_141093d10
    Replacement::draft(0x141093d40, "unknown_141093d40", tpl_62763::<0x143c9d2f0, 0x142489ee0, 0x143cd65b8, 0x141f2c190> as *const ()), // sub_141093d40
    Replacement::draft(0x141093ee0, "unknown_141093ee0", tpl_62763::<0x143c9f8e8, 0x142489ee0, 0x143cd65c0, 0x141f297f0> as *const ()), // sub_141093ee0
    Replacement::draft(0x141094080, "unknown_141094080", tpl_3028::<0x143cd89c4, 0x20, 0x8, 0x143c8ce80, 0x580e0, 0x141f2bba0> as *const ()), // sub_141094080
    Replacement::draft(0x1410940f0, "unknown_1410940f0", tpl_61557::<0x143c8d904, 0x143c904a0, 0x141093800, 0x141f2c080> as *const ()), // sub_1410940f0
    Replacement::draft(0x141094310, "unknown_141094310", tpl_62838::<0x143c8d904, 0x143c904a0, 0x143cd65b0, 0x142489ee0, 0x143cd8830, 0x143cd848c, 0x141f2b970> as *const ()), // sub_141094310
    Replacement::draft(0x141094590, "unknown_141094590", fn_141094590 as *const ()), // sub_141094590
    Replacement::draft(0x141094610, "unknown_141094610", tpl_62838::<0x143c8d904, 0x143c904a0, 0x143cd65d0, 0x142489ee0, 0x143c9f8e8, 0x143cd65c0, 0x141f288b0> as *const ()), // sub_141094610
    Replacement::draft(0x141094890, "unknown_141094890", tpl_3028::<0x143cd7fa8, 0x20, 0x8, 0x143c8ce80, 0x580e0, 0x141f26470> as *const ()), // sub_141094890
    Replacement::draft(0x141094900, "unknown_141094900", tpl_4148::<0x143cd8ccc, 0x20, 0x8, 0x141f2a880> as *const ()), // sub_141094900
    Replacement::draft(0x141094960, "unknown_141094960", tpl_4148::<0x143cd8160, 0x20, 0x8, 0x141f283a0> as *const ()), // sub_141094960
    Replacement::draft(0x1410949c0, "unknown_1410949c0", tpl_62763::<0x143c8d87c, 0x142489ee0, 0x143cd82e0, 0x141f263f0> as *const ()), // sub_1410949c0
    Replacement::draft_throws(0x141094b60, "unknown_141094b60", tpl_5128::<0x143cd85c4, 0x20, 0x8, 0x141f29b20> as *const (), Throws::Discard), // sub_141094b60
    Replacement::draft_throws(0x141094bc0, "unknown_141094bc0", tpl_5128::<0x143cd8950, 0x20, 0x8, 0x141f27160> as *const (), Throws::Discard), // sub_141094bc0
    Replacement::draft(0x141094c20, "unknown_141094c20", tpl_78777::<0x1410931c0, 0x141f25ce0> as *const ()), // sub_141094c20
    Replacement::draft(0x141094e70, "unknown_141094e70", tpl_1898::<0x88, 0x10088, 0x141f2b040> as *const ()), // sub_141094e70
    Replacement::draft(0x141094f00, "unknown_141094f00", tpl_1898::<0x88, 0x10088, 0x141f28cc0> as *const ()), // sub_141094f00
    Replacement::draft(0x141094f90, "unknown_141094f90", tpl_1898::<0x88, 0x10088, 0x141f267a0> as *const ()), // sub_141094f90
    Replacement::draft_throws(0x141095020, "unknown_141095020", tpl_5996::<0x88, 0x10088, 0x141096900, 0x1410950d0> as *const (), Throws::Discard), // sub_141095020
    Replacement::draft_throws(0x1410950d0, "unknown_1410950d0", tpl_3284::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141f2b240> as *const (), Throws::Discard), // sub_1410950d0
    Replacement::draft_throws(0x1410951e0, "unknown_1410951e0", fn_1410951e0 as *const (), Throws::Caught), // sub_1410951e0
    Replacement::draft_throws(0x1410952d0, "unknown_1410952d0", tpl_3284::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141f28ec0> as *const (), Throws::Discard), // sub_1410952d0
    Replacement::draft_throws(0x1410953e0, "unknown_1410953e0", tpl_20423::<0x40, 0x48, 0x10048, 0x141096540, 0x1410954d0> as *const (), Throws::Discard), // sub_1410953e0
    Replacement::draft_throws(0x1410954d0, "unknown_1410954d0", tpl_5101::<0x18, 0x20, 0x40, 0x8, 0x141f269a0> as *const (), Throws::Discard), // sub_1410954d0
    Replacement::draft(0x1410955c0, "unknown_1410955c0", tpl_6144::<0x143cd852c, 0x5e308, 0x10, 0x8, 0x18, 0x141f2ad50, 0x140489dd0> as *const ()), // sub_1410955c0
    Replacement::draft(0x141095660, "unknown_141095660", tpl_6144::<0x143cd8a20, 0x5e308, 0x10, 0x8, 0x18, 0x141f289d0, 0x140489dd0> as *const ()), // sub_141095660
    Replacement::draft_throws(0x141095700, "unknown_141095700", fn_141095700 as *const (), Throws::Caught), // sub_141095700
    Replacement::draft(0x141095790, "unknown_141095790", tpl_6144::<0x143cd82e0, 0x5e308, 0x10, 0x8, 0x18, 0x141f26740, 0x140489dd0> as *const ()), // sub_141095790
    Replacement::draft_throws(0x141095830, "unknown_141095830", tpl_5214::<0x143cd867c, 0x20, 0x8, 0x141f25a90> as *const (), Throws::Discard), // sub_141095830
    Replacement::draft(0x141095e50, "unknown_141095e50", tpl_3188::<0x141098450, 0x1433809c8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141095e50
    Replacement::draft_throws(0x141095ed0, "unknown_141095ed0", fn_141095ed0 as *const (), Throws::Caught), // sub_141095ed0
    Replacement::draft(0x141095f50, "unknown_141095f50", tpl_3188::<0x141098590, 0x143380968, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141095f50
    Replacement::draft(0x141095fd0, "unknown_141095fd0", tpl_3188::<0x141098630, 0x143380a40, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141095fd0
    Replacement::draft_throws(0x141096050, "unknown_141096050", fn_141096050 as *const (), Throws::Discard), // sub_141096050
    Replacement::draft(0x141096cb0, "unknown_141096cb0", fn_141096cb0 as *const ()), // sub_141096cb0
    Replacement::draft(0x141096de0, "unknown_141096de0", fn_141096de0 as *const ()), // sub_141096de0
    Replacement::draft(0x141096f10, "unknown_141096f10", fn_141096f10 as *const ()), // sub_141096f10
    Replacement::draft_throws(0x1410970a0, "unknown_1410970a0", fn_1410970a0 as *const (), Throws::Caught), // sub_1410970a0
    Replacement::draft(0x1410971c0, "unknown_1410971c0", fn_1410971c0 as *const ()), // sub_1410971c0
    Replacement::draft(0x1410972f0, "unknown_1410972f0", tpl_61019::<0x143c9d2f0, 0x142489ee0, 0x143cd65b8, 0x143cd848c, 0x1426467e8, 0x141097453> as *const ()), // sub_1410972f0
    Replacement::draft(0x141097950, "unknown_141097950", tpl_61552::<0x8, 0x88, 0x10088, 0x141f2b040> as *const ()), // sub_141097950
    Replacement::draft(0x141097a40, "unknown_141097a40", tpl_61552::<0x8, 0x88, 0x10088, 0x141f28cc0> as *const ()), // sub_141097a40
    Replacement::draft(0x141097b30, "unknown_141097b30", tpl_61552::<0x8, 0x88, 0x10088, 0x141f267a0> as *const ()), // sub_141097b30
    Replacement::draft_throws(0x141097c20, "unknown_141097c20", tpl_65139::<0x1410969c0, 0x141f2b240> as *const (), Throws::Discard), // sub_141097c20
    Replacement::draft_throws(0x141097e20, "unknown_141097e20", fn_141097e20 as *const (), Throws::Caught), // sub_141097e20
    Replacement::draft_throws(0x141098020, "unknown_141098020", tpl_78787::<0x141096540, 0x141f269a0> as *const (), Throws::Discard), // sub_141098020
    Replacement::draft(0x141098210, "unknown_141098210", tpl_3028::<0x143cd84e0, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410905a0> as *const ()), // sub_141098210
    Replacement::draft(0x141098280, "unknown_141098280", tpl_3028::<0x143cd7f90, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410909d0> as *const ()), // sub_141098280
    Replacement::draft(0x1410982f0, "unknown_1410982f0", tpl_3028::<0x143cd83ac, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141090e20> as *const ()), // sub_1410982f0
    Replacement::draft(0x141098450, "unknown_141098450", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433809c8, 0x8, 0x20> as *const ()), // sub_141098450
    Replacement::draft(0x1410984f0, "unknown_1410984f0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143380990, 0x8, 0x20> as *const ()), // sub_1410984f0
    Replacement::draft(0x141098590, "unknown_141098590", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143380968, 0x8, 0x20> as *const ()), // sub_141098590
    Replacement::draft(0x141098630, "unknown_141098630", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143380a40, 0x8, 0x20> as *const ()), // sub_141098630
    Replacement::draft(0x1410986d0, "unknown_1410986d0", fn_1410986d0 as *const ()), // sub_1410986d0
    Replacement::draft(0x1410988e0, "unknown_1410988e0", tpl_5835::<0x143cd8d38, 0x143cd7fb0, 0x143c9d2f0, 0x143cd65b8, 0x143cd848c, 0x143c9f8e8, 0x143cd65c0, 0x143cd7f88, 0x143c8d120, 0x143c8f6d8, 0x143cd8a20, 0x143cd8cc8, 0x143cd7fd0, 0x143cd7ff0, 0x143cd8010, 0x143cd8020, 0x143cd8028, 0x141834a0c, 0x141834a78> as *const ()), // sub_1410988e0
    Replacement::draft(0x141098b20, "unknown_141098b20", tpl_4691::<0x143cd8ba8, 0x143cd8ce0, 0x141834a78, 0x143c9d2f0, 0x143cd65b8, 0x143cd848c, 0x143cd65bc, 0x143c8d120, 0x143c8f6d8, 0x143cd852c, 0x143cd8d00, 0x143cd8d20, 0x143cd8d30, 0x141834a0c> as *const ()), // sub_141098b20
    Replacement::draft(0x141099a50, "unknown_141099a50", fn_141099a50 as *const ()), // sub_141099a50
    Replacement::draft(0x141099c10, "unknown_141099c10", tpl_20435::<0x143cd84b0, 0x143cd7e70, 0x143c8d694, 0x143cd8a20, 0x143cd8cc8, 0x143c8d120, 0x143c9d2f0, 0x143cd848c, 0x143c9f8e8, 0x143cd7f88, 0x143c8fa04, 0x143c8d87c, 0x143cd7e90, 0x143cd7eb0, 0x143cd7ed0, 0x143cd7ee0, 0x143cd7ee8, 0x141834a0c, 0x141834a78> as *const ()), // sub_141099c10
    Replacement::draft(0x141099e50, "unknown_141099e50", tpl_20435::<0x143cd88d8, 0x143cd8b20, 0x143c8d694, 0x143cd852c, 0x143cd8490, 0x143c8d120, 0x143c9d2f0, 0x143cd848c, 0x143cd65bc, 0x143cd7f8c, 0x143c8fa04, 0x143c8d87c, 0x143cd8b40, 0x143cd8b60, 0x143cd8b80, 0x143cd8b90, 0x143cd8b98, 0x141834a0c, 0x141834a78> as *const ()), // sub_141099e50
    Replacement::draft(0x14109ac00, "unknown_14109ac00", fn_14109ac00 as *const ()), // sub_14109ac00
    Replacement::draft(0x14109b250, "unknown_14109b250", fn_14109b250 as *const ()), // sub_14109b250
    Replacement::draft_throws(0x14109b310, "coregame_14109b310", tpl_19901::<0x143cd8fa0, 0x143cd8fe8, 0x1411e9f50, 0x14109b850, 0x143cd8fac, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x14109b700, 0x14109b600, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143380ac6, 0x142646960, 0x143cd9008, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lifetime::manageLifetimes>
    Replacement::draft(0x14109b700, "coregame_14109b700", tpl_63099::<0x143c8de6c, 0x142489ee0, 0x141f2d380> as *const ()), // sub_14109b700
    Replacement::draft(0x14109b990, "coregame_14109b990", tpl_4192::<0x143cd90c0, 0x143cd90c8, 0x1411e9f50, 0x14109bea0, 0x14109bfe0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143380eb6, 0x142646960, 0x143cd90e8, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::legacy_time::copyVariableTimeToLegacy>
    Replacement::draft(0x14109bbd0, "coregame_14109bbd0", tpl_6373::<0x143cd9054, 0x143cd9058, 0x1411e9f50, 0x143cd9048, 0x140215610, 0x143c8d910, 0x1420a28e0, 0x14109c020, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143380de6, 0x142646960, 0x143cd9078, 0x143cd9080, 0x143cd9084, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::legacy_time::copyFixedTimeToLegacy>
    Replacement::draft(0x14109bea0, "coregame_14109bea0", tpl_19318::<0x143cd9048, 0x140215610, 0x143c8d910, 0x143c904a0, 0x143c900d8, 0x1420a28e0> as *const ()), // sub_14109bea0
    Replacement::draft(0x14109c050, "coregame_14109c050", fn_14109c050 as *const ()), // sub_14109c050
    Replacement::draft_throws(0x14109c1f0, "coregame_14109c1f0", fn_14109c1f0 as *const (), Throws::Discard), // sub_14109c1f0
    Replacement::draft_throws(0x14109c360, "coregame_14109c360", fn_14109c360 as *const (), Throws::Discard), // sub_14109c360
    Replacement::draft(0x14109c500, "coregame_14109c500", tpl_20440::<0x143381388, 0x8, 0x3, 0x4, 0x14240aa00, 0x50, 0x98, 0xe0> as *const ()), // coregame::CoreProgramFlow::ActionRequestLoadWorldParameters::coregame::CoreProgramFlow::ActionRequestLoadWorldParameters
    Replacement::draft_throws(0x14109c920, "coregame_14109c920", tpl_3683::<0x143cd9250, 0x143cd91e0, 0x1411e9f50, 0x14109f410, 0x143cd92f0, 0x1420af120, 0x1420a28e0, 0x14109e740, 0x14109dcf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143381476, 0x142646960, 0x143cd9200, 0x143cd9208, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::jumppoint::applyTeleportedJump>
    Replacement::draft_throws(0x14109cc00, "coregame_14109cc00", tpl_5275::<0x143cd94c4, 0x143cd94d0, 0x1411e9f50, 0x14109f310, 0x143cd92f4, 0x1420af120, 0x14054b060, 0x1420a28e0, 0x14109e950, 0x14109de10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143381e26, 0x142646960, 0x143cd94f0, 0x143cd94f4, 0x143cd94f6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::jumppoint::applyJumpComplete>
    Replacement::draft(0x14109cf00, "unknown_14109cf00", tpl_1094::<0x143cd6cc8, 0x14109d980, 0x14020e8f0> as *const ()), // sub_14109cf00
    Replacement::draft(0x14109d100, "unknown_14109d100", tpl_1466::<0x143ccaebc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14109d100
    Replacement::draft(0x14109d3d0, "unknown_14109d3d0", tpl_1094::<0x143cd9300, 0x14109db50, 0x14020e8f0> as *const ()), // sub_14109d3d0
    Replacement::draft(0x14109d5d0, "unknown_14109d5d0", tpl_5094::<0x8, 0x14109cf00, 0x14109d100> as *const ()), // sub_14109d5d0
    Replacement::draft(0x14109d640, "unknown_14109d640", tpl_1439::<0x8, 0x14109d3d0> as *const ()), // sub_14109d640
    Replacement::draft(0x14109d670, "unknown_14109d670", fn_14109d670 as *const ()), // sub_14109d670
    Replacement::draft(0x14109d980, "unknown_14109d980", fn_14109d980 as *const ()), // sub_14109d980
    Replacement::draft(0x14109db50, "unknown_14109db50", fn_14109db50 as *const ()), // sub_14109db50
    Replacement::draft(0x14109dcf0, "unknown_14109dcf0", tpl_62730::<0x88, 0x10088, 0x14109dff0, 0x141f2f1a0> as *const ()), // sub_14109dcf0
    Replacement::draft(0x14109de10, "unknown_14109de10", tpl_64022::<0x88, 0x10088, 0x14109def0, 0x141f2f210> as *const ()), // sub_14109de10
    Replacement::draft(0x14109e120, "unknown_14109e120", fn_14109e120 as *const ()), // sub_14109e120
    Replacement::draft(0x14109e2e0, "unknown_14109e2e0", fn_14109e2e0 as *const ()), // sub_14109e2e0
    Replacement::draft(0x14109e740, "unknown_14109e740", tpl_62741::<0x143ccfc30, 0x142489ee0, 0x143c8d074, 0x143cb8c4c, 0x141f2f1a0> as *const ()), // sub_14109e740
    Replacement::draft(0x14109e950, "unknown_14109e950", tpl_64023::<0x143ccaebc, 0x142489ee0, 0x143c8d160, 0x141f2f210> as *const ()), // sub_14109e950
    Replacement::draft(0x14109eb00, "unknown_14109eb00", fn_14109eb00 as *const ()), // sub_14109eb00
    Replacement::draft(0x14109ec40, "unknown_14109ec40", fn_14109ec40 as *const ()), // sub_14109ec40
    Replacement::draft_throws(0x14109eda0, "unknown_14109eda0", tpl_5001::<0x143cd941c, 0x143cd92fc, 0x143cd9290, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143cd6cc8, 0x143c8d120, 0x143cd92b0, 0x141834a0c> as *const (), Throws::Discard), // sub_14109eda0
    Replacement::draft_throws(0x14109ef20, "unknown_14109ef20", tpl_4018::<0x143cd9420, 0x143cd92f8, 0x143cd9428, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cd6cc8, 0x143c8d120, 0x143cd9438, 0x143cd9440, 0x141834a0c> as *const (), Throws::Discard), // sub_14109ef20
    Replacement::draft_throws(0x14109f080, "unknown_14109f080", tpl_4018::<0x143cd94b8, 0x143cd94c0, 0x143cd9400, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cd6cc8, 0x143c8fa04, 0x143cd9410, 0x143cd9418, 0x141834a0c> as *const (), Throws::Discard), // sub_14109f080
    Replacement::draft_throws(0x14109f1e0, "unknown_14109f1e0", tpl_847::<0x143cd93f8, 0x143cd94f8, 0x143cd93b0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cd6cc8, 0x141834a0c> as *const (), Throws::Discard), // sub_14109f1e0
    Replacement::draft(0x14109f550, "unknown_14109f550", tpl_20448::<0x1402174f0, 0x14109ec40> as *const ()), // sub_14109f550
    Replacement::draft(0x14109f710, "anonymous_namespace_14109f710", tpl_5262::<0x143cd9558, 0x143cd9528, 0x1411e9f50, 0x143cd9590, 0x140215610, 0x1420a28e0, 0x1410a01b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143382586, 0x142646960, 0x143cd9548, 0x143cd9550, 0x143cd9552, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::inputVariableUpdate>
    Replacement::draft(0x14109f9b0, "anonymous_namespace_14109f9b0", tpl_5528::<0x143cd9640, 0x143cd9560, 0x1411e9f50, 0x143cd9590, 0x140215610, 0x1420a28e0, 0x1410a01e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143382436, 0x142646960, 0x143cd9580, 0x143cd9588, 0x143cd958c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::inputFixedUpdateStart>
    Replacement::draft(0x14109fc50, "anonymous_namespace_14109fc50", fn_14109fc50 as *const ()), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::inputFixedUpdate>
    Replacement::draft(0x14109ff10, "anonymous_namespace_14109ff10", tpl_1537::<0x143cd9554, 0x143cd9610, 0x1411e9f50, 0x143cd9590, 0x140215610, 0x1420a28e0, 0x1410a0230, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433823a6, 0x142646960, 0x143cd9630, 0x143cd9638, 0x143cd963c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::inputFixedUpdateFinish>
    Replacement::draft(0x1410a01b0, "unknown_1410a01b0", fn_1410a01b0 as *const ()), // sub_1410a01b0
    Replacement::draft(0x1410a0250, "unknown_1410a0250", fn_1410a0250 as *const ()), // sub_1410a0250
    Replacement::draft(0x1410a02f0, "unknown_1410a02f0", fn_1410a02f0 as *const ()), // sub_1410a02f0
    Replacement::draft(0x1410a0350, "unknown_1410a0350", fn_1410a0350 as *const ()), // sub_1410a0350
    Replacement::draft(0x1410a03e0, "coregame_1410a03e0", fn_1410a03e0 as *const ()), // coregame::component::DeferredIlluminationVolume::coregame::component::DeferredIlluminationVolume::vfunc_0
    Replacement::draft(0x1410a0440, "coregame_1410a0440", tpl_495::<0x1410a02f0, 0x141834d30> as *const ()), // coregame::component::DeferredIlluminationVolume::coregame::component::DeferredIlluminationVolume::vfunc_1
    Replacement::draft(0x1410a0480, "coregame_1410a0480", fn_1410a0480 as *const ()), // sub_1410a0480
    Replacement::draft(0x1410a0500, "coregame_1410a0500", fn_1410a0500 as *const ()), // sub_1410a0500
    Replacement::draft(0x1410a06b0, "coregame_1410a06b0", fn_1410a06b0 as *const ()), // sub_1410a06b0
    Replacement::draft(0x1410a0820, "coregame_1410a0820", fn_1410a0820 as *const ()), // sub_1410a0820
    Replacement::draft(0x1410a0990, "coregame_1410a0990", fn_1410a0990 as *const ()), // sub_1410a0990
    Replacement::draft(0x1410a0b40, "coregame_1410a0b40", fn_1410a0b40 as *const ()), // sub_1410a0b40
    Replacement::draft(0x1410a0cb0, "coregame_1410a0cb0", fn_1410a0cb0 as *const ()), // sub_1410a0cb0
    Replacement::draft(0x1410a0e20, "coregame_1410a0e20", fn_1410a0e20 as *const ()), // sub_1410a0e20
    Replacement::draft(0x1410a0f90, "coregame_1410a0f90", fn_1410a0f90 as *const ()), // sub_1410a0f90
    Replacement::draft(0x1410a1100, "coregame_1410a1100", fn_1410a1100 as *const ()), // sub_1410a1100
    Replacement::draft(0x1410a12c0, "coregame_1410a12c0", fn_1410a12c0 as *const ()), // sub_1410a12c0
    Replacement::draft(0x1410a1470, "coregame_1410a1470", fn_1410a1470 as *const ()), // sub_1410a1470
    Replacement::draft(0x1410a1620, "coregame_1410a1620", fn_1410a1620 as *const ()), // sub_1410a1620
    Replacement::draft(0x1410a1790, "coregame_1410a1790", fn_1410a1790 as *const ()), // sub_1410a1790
    Replacement::draft_throws(0x1410a1900, "coregame_1410a1900", tpl_1467::<0x143cd978c, 0x143cd9758, 0x1411e9f50, 0x1410a6970, 0x143cd980c, 0x1420af120, 0x1420a28e0, 0x1410a5040, 0x1410a4580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143383476, 0x142646960, 0x143cd9778, 0x143cd9780, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::illumination_volume::copyPrevious>
    Replacement::draft_throws(0x1410a1be0, "coregame_1410a1be0", fn_1410a1be0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::illumination_volume::updateRenderer>
    Replacement::draft_throws(0x1410a1ec0, "coregame_1410a1ec0", fn_1410a1ec0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::illumination_volume::updateParameterBlends>
    Replacement::draft_throws(0x1410a2210, "coregame_1410a2210", fn_1410a2210 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::illumination_volume::removeIlluminationVolumeLight>
    Replacement::draft(0x1410a2500, "coregame_1410a2500", tpl_1094::<0x143cd9a10, 0x1410a4040, 0x14020e8f0> as *const ()), // sub_1410a2500
    Replacement::draft(0x1410a2700, "coregame_1410a2700", fn_1410a2700 as *const ()), // sub_1410a2700
    Replacement::draft(0x1410a2a80, "coregame_1410a2a80", tpl_1094::<0x143cd9be0, 0x1410a41d0, 0x14020e8f0> as *const ()), // sub_1410a2a80
    Replacement::draft(0x1410a2c80, "coregame_1410a2c80", tpl_1099::<0x143c9e9c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410a2c80
    Replacement::draft(0x1410a2fc0, "coregame_1410a2fc0", tpl_1099::<0x143cd65d8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410a2fc0
    Replacement::draft(0x1410a3300, "coregame_1410a3300", tpl_1094::<0x143cd9a84, 0x1410a4390, 0x14020e8f0> as *const ()), // sub_1410a3300
    Replacement::draft_throws(0x1410a3500, "coregame_1410a3500", tpl_3591::<0x143cd9b58, 0x143cd9af8, 0x1411e9f50, 0x1410a6c90, 0x143cd9bdc, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410a6260, 0x1410a51e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433838b6, 0x142646960, 0x143cd9b18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::illumination_volume::streamIn>
    Replacement::draft_throws(0x1410a37f0, "coregame_1410a37f0", tpl_3592::<0x143cd99b0, 0x143cd97e0, 0x1411e9f50, 0x1410a6b50, 0x143cd9a74, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410a6450, 0x1410a53c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143383606, 0x142646960, 0x143cd9800, 0x143cd9808, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::illumination_volume::streamOut>
    Replacement::draft(0x1410a3af0, "coregame_1410a3af0", tpl_20410::<0x1401e7c10, 0x1410a3f40, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1410a3af0
    Replacement::draft_throws(0x1410a3d00, "coregame_1410a3d00", tpl_20473::<0x143cd9a0c, 0x143cd9a90, 0x1410a4960, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143384046, 0x142646960, 0x143cd9ab0, 0x143cd9ad0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::IlluminationVolumeC
    Replacement::draft(0x1410a4040, "unknown_1410a4040", tpl_19680::<0x1401e6d50, 0x1410a4fa0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_1410a4040
    Replacement::draft(0x1410a41d0, "unknown_1410a41d0", fn_1410a41d0 as *const ()), // sub_1410a41d0
    Replacement::draft(0x1410a4390, "unknown_1410a4390", fn_1410a4390 as *const ()), // sub_1410a4390
    Replacement::draft(0x1410a4580, "unknown_1410a4580", tpl_62755::<0x88, 0x10088, 0x1410a4ea0, 0x141f33510> as *const ()), // sub_1410a4580
    Replacement::draft(0x1410a4660, "unknown_1410a4660", fn_1410a4660 as *const ()), // sub_1410a4660
    Replacement::draft(0x1410a4700, "unknown_1410a4700", tpl_5027::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141f32030> as *const ()), // sub_1410a4700
    Replacement::draft(0x1410a47f0, "unknown_1410a47f0", fn_1410a47f0 as *const ()), // sub_1410a47f0
    Replacement::draft_throws(0x1410a4960, "unknown_1410a4960", tpl_3011::<0x1411e9f50, 0x1410a6a60, 0x143cd9a7c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410a65a0, 0x1401de410> as *const (), Throws::Discard), // sub_1410a4960
    Replacement::draft(0x1410a4fa0, "unknown_1410a4fa0", tpl_1955::<0x10, 0x1410a0480> as *const ()), // sub_1410a4fa0
    Replacement::draft(0x1410a4fd0, "unknown_1410a4fd0", fn_1410a4fd0 as *const ()), // sub_1410a4fd0
    Replacement::draft(0x1410a5010, "unknown_1410a5010", tpl_5596::<0x60> as *const ()), // sub_1410a5010
    Replacement::draft(0x1410a5040, "unknown_1410a5040", tpl_62763::<0x143c9e9c8, 0x142489ee0, 0x143cd65d8, 0x141f33510> as *const ()), // sub_1410a5040
    Replacement::draft_throws(0x1410a51e0, "unknown_1410a51e0", tpl_20423::<0x40, 0x48, 0x10048, 0x1410a5b40, 0x1410a52d0> as *const (), Throws::Discard), // sub_1410a51e0
    Replacement::draft_throws(0x1410a52d0, "unknown_1410a52d0", tpl_5101::<0x18, 0x20, 0x40, 0x8, 0x141f32be0> as *const (), Throws::Discard), // sub_1410a52d0
    Replacement::draft(0x1410a53c0, "unknown_1410a53c0", tpl_63093::<0x143cd9be0, 0x142489ee0, 0x141f328f0> as *const ()), // sub_1410a53c0
    Replacement::draft(0x1410a54c0, "unknown_1410a54c0", fn_1410a54c0 as *const ()), // sub_1410a54c0
    Replacement::draft(0x1410a56c0, "unknown_1410a56c0", tpl_64222::<0x143c8d904, 0x143c904a0, 0x1410a4b20, 0x141f32540> as *const ()), // sub_1410a56c0
    Replacement::draft(0x1410a58a0, "unknown_1410a58a0", tpl_6144::<0x143cd9810, 0x5e308, 0x10, 0x8, 0x18, 0x141f32600, 0x140489dd0> as *const ()), // sub_1410a58a0
    Replacement::draft(0x1410a5940, "unknown_1410a5940", tpl_3188::<0x1410a6610, 0x143384930, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410a5940
    Replacement::draft(0x1410a5ce0, "unknown_1410a5ce0", fn_1410a5ce0 as *const ()), // sub_1410a5ce0
    Replacement::draft_throws(0x1410a73a0, "unknown_1410a73a0", tpl_2918::<0x143cd9ccc, 0x143cd9c28, 0x143cd9c98, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cd9c68, 0x141834a0c> as *const (), Throws::Discard), // sub_1410a73a0
    Replacement::draft(0x1410a7590, "unknown_1410a7590", fn_1410a7590 as *const ()), // sub_1410a7590
    Replacement::draft(0x1410a7760, "unknown_1410a7760", tpl_6428::<0x8, 0x142442de0> as *const ()), // sub_1410a7760
    Replacement::draft(0x1410a77e0, "unknown_1410a77e0", fn_1410a77e0 as *const ()), // sub_1410a77e0
    Replacement::draft(0x1410a7840, "unknown_1410a7840", fn_1410a7840 as *const ()), // sub_1410a7840
    Replacement::draft_throws(0x1410a78e0, "coregame_1410a78e0", fn_1410a78e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::startGatheringEntitiesWithResourcesToHotload>
    Replacement::draft_throws(0x1410a7be0, "coregame_1410a7be0", fn_1410a7be0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::startHotloading>
    Replacement::draft_throws(0x1410a7de0, "coregame_1410a7de0", fn_1410a7de0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::stopHotloading>
    Replacement::draft(0x1410a80f0, "coregame_1410a80f0", fn_1410a80f0 as *const ()), // sub_1410a80f0
    Replacement::draft(0x1410a8200, "coregame_1410a8200", fn_1410a8200 as *const ()), // sub_1410a8200
    Replacement::draft_throws(0x1410a8570, "coregame_1410a8570", fn_1410a8570 as *const (), Throws::Discard), // sub_1410a8570
    Replacement::draft(0x1410a8740, "coregame_1410a8740", tpl_1095::<0x143cc2784, 0x14020e8f0> as *const ()), // sub_1410a8740
    Replacement::draft(0x1410a8920, "coregame_1410a8920", fn_1410a8920 as *const ()), // sub_1410a8920
    Replacement::draft_throws(0x1410a8c20, "coregame_1410a8c20", tpl_1330::<0x8, 0x1410a9ab0, 0x143cd9de0, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1410a8c20
    Replacement::draft(0x1410a8d00, "coregame_1410a8d00", fn_1410a8d00 as *const ()), // sub_1410a8d00
    Replacement::draft(0x1410a8e00, "coregame_1410a8e00", fn_1410a8e00 as *const ()), // sub_1410a8e00
    Replacement::draft_throws(0x1410a8f40, "coregame_1410a8f40", tpl_1804::<0x143cd9de0, 0x580e0, 0x20, 0x8, 0x143c9a718, 0x143c8ce8c, 0x141f37bb0> as *const (), Throws::Discard), // sub_1410a8f40
    Replacement::draft_throws(0x1410a8fc0, "coregame_1410a8fc0", fn_1410a8fc0 as *const (), Throws::Discard), // sub_1410a8fc0
    Replacement::draft_throws(0x1410a9090, "coregame_1410a9090", fn_1410a9090 as *const (), Throws::Discard), // sub_1410a9090
    Replacement::draft_throws(0x1410a92d0, "coregame_1410a92d0", fn_1410a92d0 as *const (), Throws::Caught), // sub_1410a92d0
    Replacement::draft_throws(0x1410a9450, "coregame_1410a9450", fn_1410a9450 as *const (), Throws::Discard), // sub_1410a9450
    Replacement::draft(0x1410a96a0, "coregame_1410a96a0", fn_1410a96a0 as *const ()), // sub_1410a96a0
    Replacement::draft(0x1410a9730, "coregame_1410a9730", fn_1410a9730 as *const ()), // sub_1410a9730
    Replacement::draft(0x1410a9800, "coregame_1410a9800", fn_1410a9800 as *const ()), // sub_1410a9800
    Replacement::draft(0x1410a9ba0, "coregame_1410a9ba0", fn_1410a9ba0 as *const ()), // sub_1410a9ba0
    Replacement::draft(0x1410a9ca0, "coregame_1410a9ca0", fn_1410a9ca0 as *const ()), // sub_1410a9ca0
    Replacement::draft(0x1410a9df0, "coregame_1410a9df0", fn_1410a9df0 as *const ()), // sub_1410a9df0
    Replacement::draft(0x1410a9ee0, "coregame_1410a9ee0", fn_1410a9ee0 as *const ()), // sub_1410a9ee0
    Replacement::draft(0x1410aa240, "coregame_1410aa240", tpl_1171::<0x58, 0x60, 0x143cda2dc, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1410aa240
    Replacement::draft_throws(0x1410aa320, "coregame_1410aa320", fn_1410aa320 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::calculateLodVisibility>
    Replacement::draft_throws(0x1410aa620, "coregame_1410aa620", tpl_19446::<0x143cda858, 0x143cda678, 0x1411e9f50, 0x1410b2340, 0x143cda2cc, 0x1420af120, 0x143c8d0c0, 0x140215610, 0x1420a28e0, 0x1410afd70, 0x1410ae8a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143387e86, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::update>
    Replacement::draft_throws(0x1410aa910, "coregame_1410aa910", tpl_1428::<0x143cd9fe0, 0x143cda320, 0x1411e9f50, 0x1410ae9a0, 0x1410aff40, 0x1410aece0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433880a6, 0x142646960, 0x143cda340, 0x143cda344, 0x143cda346, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::waitAndSwitch>
    Replacement::draft_throws(0x1410aab60, "coregame_1410aab60", tpl_1480::<0x142901ce8, 0x1410d6ca0, 0x1410ace40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410aab60
    Replacement::draft_throws(0x1410aae00, "coregame_1410aae00", tpl_1480::<0x142901ce8, 0x1410d6ca0, 0x1410ad150, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410aae00
    Replacement::draft_throws(0x1410ab0a0, "coregame_1410ab0a0", fn_1410ab0a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::applyHide>
    Replacement::draft(0x1410ab3c0, "coregame_1410ab3c0", tpl_1526::<0x142901ce8, 0x1410d6ca0, 0x1410ad9f0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1410ab3c0
    Replacement::draft(0x1410ab660, "coregame_1410ab660", tpl_1526::<0x142901ce8, 0x1410d6ca0, 0x1410adce0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1410ab660
    Replacement::draft_throws(0x1410ab900, "coregame_1410ab900", fn_1410ab900 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::ignoreStreamingBroadphase>
    Replacement::draft(0x1410abc30, "coregame_1410abc30", tpl_1094::<0x143cda2dc, 0x1410adfd0, 0x14020e8f0> as *const ()), // sub_1410abc30
    Replacement::draft(0x1410abe30, "coregame_1410abe30", tpl_1094::<0x143cda2d4, 0x1410ae150, 0x14020e8f0> as *const ()), // sub_1410abe30
    Replacement::draft(0x1410ac030, "coregame_1410ac030", tpl_1436::<0x58, 0x60, 0x158, 0x143cda2d4, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1410ac030
    Replacement::draft(0x1410ac110, "coregame_1410ac110", tpl_1094::<0x143cda0f8, 0x1410ae2d0, 0x14020e8f0> as *const ()), // sub_1410ac110
    Replacement::draft(0x1410ac310, "coregame_1410ac310", tpl_1466::<0x143cd65dc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410ac310
    Replacement::draft(0x1410ac5e0, "coregame_1410ac5e0", tpl_1094::<0x143cda0f0, 0x1410ae530, 0x14020e8f0> as *const ()), // sub_1410ac5e0
    Replacement::draft(0x1410ac7e0, "coregame_1410ac7e0", tpl_3073::<0x143cda714, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410ac7e0
    Replacement::draft(0x1410acb40, "coregame_1410acb40", tpl_1466::<0x143cda3cc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410acb40
    Replacement::draft_throws(0x1410ace40, "coregame_1410ace40", fn_1410ace40 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::freezePhysicsInCollapsedBatches>
    Replacement::draft_throws(0x1410ad150, "coregame_1410ad150", fn_1410ad150 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::freezeCharactersInCollapsedBatches>
    Replacement::draft_throws(0x1410ad440, "coregame_1410ad440", tpl_1404::<0x143cda408, 0x143cda270, 0x1411e9f50, 0x1410b3230, 0x143cda850, 0x1420af120, 0x1420a28e0, 0x1410b1840, 0x1410b0750, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433891a6, 0x142646960, 0x143cda290, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::streamIn>
    Replacement::draft_throws(0x1410ad710, "coregame_1410ad710", tpl_1078::<0x143cda3c0, 0x143cda2a0, 0x1411e9f50, 0x1410b3130, 0x143cda66c, 0x1420af120, 0x1420a28e0, 0x1410b19f0, 0x1410b0840, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433893f6, 0x142646960, 0x143cda2c0, 0x143cda2c2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::streamOut>
    Replacement::draft(0x1410ad9f0, "coregame_1410ad9f0", tpl_19357::<0x143cda13c, 0x143cda5b0, 0x1411e9f50, 0x1410b2650, 0x143cda2dc, 0x1401fc320, 0x143c8d0c0, 0x140215610, 0x1420a28e0, 0x1410b1ad0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143388706, 0x142646960, 0x143cda5d0, 0x143cda5e0, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::streamInAndSwitchRemoved>
    Replacement::draft(0x1410adce0, "coregame_1410adce0", fn_1410adce0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::hierarchy_lod::hierarchyLODRemoved>
    Replacement::draft(0x1410adfd0, "unknown_1410adfd0", fn_1410adfd0 as *const ()), // sub_1410adfd0
    Replacement::draft(0x1410ae150, "unknown_1410ae150", tpl_6546::<0x1401e6d50, 0x1410afac0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1410af4f0> as *const ()), // sub_1410ae150
    Replacement::draft(0x1410ae2d0, "unknown_1410ae2d0", fn_1410ae2d0 as *const ()), // sub_1410ae2d0
    Replacement::draft(0x1410ae530, "unknown_1410ae530", tpl_5788::<0x1401e6d50, 0x1410afb30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1410af570> as *const ()), // sub_1410ae530
    Replacement::draft(0x1410ae6b0, "unknown_1410ae6b0", tpl_1915::<0x143c8d014, 0x1410af940, 0x1410ae7b0> as *const ()), // sub_1410ae6b0
    Replacement::draft(0x1410ae7b0, "unknown_1410ae7b0", tpl_1916::<0x18, 0x20, 0x40, 0x8, 0x141f3bff0> as *const ()), // sub_1410ae7b0
    Replacement::draft(0x1410ae8a0, "unknown_1410ae8a0", tpl_5377::<0x143c8d0c0, 0x142407c80, 0x1410af840, 0x580f0, 0x141f3ba90> as *const ()), // sub_1410ae8a0
    Replacement::draft_throws(0x1410ae9a0, "unknown_1410ae9a0", fn_1410ae9a0 as *const (), Throws::Discard), // sub_1410ae9a0
    Replacement::draft(0x1410aece0, "unknown_1410aece0", fn_1410aece0 as *const ()), // sub_1410aece0
    Replacement::draft(0x1410aedb0, "unknown_1410aedb0", fn_1410aedb0 as *const ()), // sub_1410aedb0
    Replacement::draft(0x1410aef00, "unknown_1410aef00", fn_1410aef00 as *const ()), // sub_1410aef00
    Replacement::draft(0x1410af050, "unknown_1410af050", fn_1410af050 as *const ()), // sub_1410af050
    Replacement::draft(0x1410af150, "unknown_1410af150", tpl_3802::<0x18, 0x20, 0x30, 0x8, 0x28, 0x141f3a6f0> as *const ()), // sub_1410af150
    Replacement::draft_throws(0x1410af230, "unknown_1410af230", tpl_19474::<0x140fa8ed0, 0x143cda0f0, 0x142489ee0, 0xc03, 0x1410af350, 0x14065a1a0> as *const (), Throws::Discard), // sub_1410af230
    Replacement::draft(0x1410af350, "unknown_1410af350", tpl_77866::<0x18, 0x20, 0x28, 0x1c8, 0x1e0, 0x1d0, 0x141f3a3c0> as *const ()), // sub_1410af350
    Replacement::draft(0x1410af450, "unknown_1410af450", fn_1410af450 as *const ()), // sub_1410af450
    Replacement::draft(0x1410af4f0, "unknown_1410af4f0", fn_1410af4f0 as *const ()), // sub_1410af4f0
    Replacement::draft(0x1410af570, "unknown_1410af570", fn_1410af570 as *const ()), // sub_1410af570
    Replacement::draft(0x1410afa90, "unknown_1410afa90", tpl_20518::<0x7, 0x78, 0x30, 0x18> as *const ()), // sub_1410afa90
    Replacement::draft(0x1410afac0, "unknown_1410afac0", tpl_20518::<0x7, 0x40, 0x28, 0x10> as *const ()), // sub_1410afac0
    Replacement::draft(0x1410afaf0, "unknown_1410afaf0", tpl_5596::<0xa0> as *const ()), // sub_1410afaf0
    Replacement::draft(0x1410afb30, "unknown_1410afb30", tpl_20518::<0x7, 0x38, 0x28, 0x10> as *const ()), // sub_1410afb30
    Replacement::draft(0x1410afb60, "unknown_1410afb60", tpl_61556::<0x143c8d014, 0x1410af940, 0x141f3bff0> as *const ()), // sub_1410afb60
    Replacement::draft(0x1410afd70, "unknown_1410afd70", tpl_5378::<0x143c8d0c0, 0x142407c80, 0x143c8d108, 0x142489ee0, 0x143cda2d4, 0x141f3ba90> as *const ()), // sub_1410afd70
    Replacement::draft(0x1410aff40, "unknown_1410aff40", fn_1410aff40 as *const ()), // sub_1410aff40
    Replacement::draft(0x1410b04a0, "unknown_1410b04a0", fn_1410b04a0 as *const ()), // sub_1410b04a0
    Replacement::draft(0x1410b0750, "unknown_1410b0750", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x1410b0d90, 0x580f0, 0x141f3b3f0> as *const ()), // sub_1410b0750
    Replacement::draft(0x1410b0840, "unknown_1410b0840", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x1410ac030> as *const ()), // sub_1410b0840
    Replacement::draft_throws(0x1410b08d0, "unknown_1410b08d0", tpl_19476::<0x140fa8ed0, 0x143cda0f0, 0x142489ee0, 0x141f3a3c0, 0x14065a1a0> as *const (), Throws::Discard), // sub_1410b08d0
    Replacement::draft(0x1410b1460, "unknown_1410b1460", tpl_61071::<0x143cda0f4, 0x143c8d120, 0x142489ee0, 0x143ccf820, 0x141f3aa30> as *const ()), // sub_1410b1460
    Replacement::draft(0x1410b1650, "unknown_1410b1650", tpl_61071::<0x143cda0f4, 0x143c8d120, 0x142489ee0, 0x143ca72cc, 0x141f3a990> as *const ()), // sub_1410b1650
    Replacement::draft(0x1410b1840, "unknown_1410b1840", tpl_3213::<0x142407c80, 0x143c8d108, 0x142489ee0, 0x143cda0f0, 0x141f3b3f0> as *const ()), // sub_1410b1840
    Replacement::draft(0x1410b19f0, "unknown_1410b19f0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1410ac030> as *const ()), // sub_1410b19f0
    Replacement::draft(0x1410b1ad0, "unknown_1410b1ad0", tpl_3172::<0x143cda2dc, 0x5e308, 0x143c8d0c0, 0x580e0, 0x10, 0x8, 0x18, 0x141f3ae20, 0x1410b2c90> as *const ()), // sub_1410b1ad0
    Replacement::draft(0x1410b1b80, "unknown_1410b1b80", tpl_3172::<0x143cda2d4, 0x5e308, 0x143c8d0c0, 0x580e0, 0x10, 0x8, 0x18, 0x141f3afe0, 0x1410b2ef0> as *const ()), // sub_1410b1b80
    Replacement::draft_throws(0x1410b1c30, "unknown_1410b1c30", tpl_4016::<0x143cda660, 0x143cda4b8, 0x143cda748, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cda2dc, 0x143c8d87c, 0x143cda758, 0x143cda760, 0x141834a0c> as *const (), Throws::Discard), // sub_1410b1c30
    Replacement::draft(0x1410b25d0, "unknown_1410b25d0", tpl_3188::<0x1410b2ff0, 0x143389950, 0x1410b2ef0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410b25d0
    Replacement::draft(0x1410b2650, "unknown_1410b2650", tpl_3188::<0x1410b3090, 0x1433899d8, 0x1410b2c90, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410b2650
    Replacement::draft(0x1410b2b30, "unknown_1410b2b30", fn_1410b2b30 as *const ()), // sub_1410b2b30
    Replacement::draft(0x1410b2c90, "unknown_1410b2c90", fn_1410b2c90 as *const ()), // sub_1410b2c90
    Replacement::draft(0x1410b2d90, "unknown_1410b2d90", fn_1410b2d90 as *const ()), // sub_1410b2d90
    Replacement::draft(0x1410b2ef0, "unknown_1410b2ef0", fn_1410b2ef0 as *const ()), // sub_1410b2ef0
    Replacement::draft(0x1410b2ff0, "unknown_1410b2ff0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143389950, 0x8, 0x20> as *const ()), // sub_1410b2ff0
    Replacement::draft(0x1410b3090, "unknown_1410b3090", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433899d8, 0x8, 0x20> as *const ()), // sub_1410b3090
    Replacement::draft(0x1410b3e30, "unknown_1410b3e30", fn_1410b3e30 as *const ()), // sub_1410b3e30
    Replacement::draft_throws(0x1410b3f30, "coregame_1410b3f30", fn_1410b3f30 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hide::applyTimelineHide>
    Replacement::draft_throws(0x1410b41b0, "coregame_1410b41b0", tpl_1467::<0x143cda970, 0x143cda908, 0x1411e9f50, 0x1410b4890, 0x143cda8b4, 0x1420af120, 0x1420a28e0, 0x1410b46e0, 0x1410b4490, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143389a66, 0x142646960, 0x143cda928, 0x143cda930, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::hide::overrideSpawningHideReasons>
    Replacement::draft(0x1410b4490, "coregame_1410b4490", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x1410b4580, 0x580f0, 0x141f3ce20> as *const ()), // sub_1410b4490
    Replacement::draft(0x1410b4680, "coregame_1410b4680", tpl_1217::<0x143ccaec8, 0x20, 0x8, 0x141f3cf60> as *const ()), // sub_1410b4680
    Replacement::draft(0x1410b46e0, "coregame_1410b46e0", tpl_3213::<0x142407c80, 0x143c957c4, 0x142489ee0, 0x143c8d87c, 0x141f3ce20> as *const ()), // sub_1410b46e0
    Replacement::draft(0x1410b4a10, "coregame_1410b4a10", tpl_1171::<0x58, 0x60, 0x143cdac58, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1410b4a10
    Replacement::draft_throws(0x1410b4af0, "coregame_1410b4af0", tpl_1480::<0x142902fa0, 0x1410d6ca0, 0x1410b6300, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410b4af0
    Replacement::draft_throws(0x1410b4d90, "coregame_1410b4d90", tpl_1480::<0x142902fa0, 0x1410d6ca0, 0x1410b6600, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410b4d90
    Replacement::draft_throws(0x1410b5030, "coregame_1410b5030", tpl_1480::<0x142902fa0, 0x1410d6ca0, 0x1410b7870, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410b5030
    Replacement::draft_throws(0x1410b52d0, "coregame_1410b52d0", tpl_1480::<0x142902fa0, 0x1410d6ca0, 0x1410b7b30, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410b52d0
    Replacement::draft(0x1410b5570, "coregame_1410b5570", tpl_1369::<0x143cdac50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410b5570
    Replacement::draft(0x1410b58b0, "coregame_1410b58b0", tpl_1094::<0x143cdac58, 0x1410b7f60, 0x14020e8f0> as *const ()), // sub_1410b58b0
    Replacement::draft(0x1410b5ab0, "coregame_1410b5ab0", tpl_1094::<0x143cdad10, 0x1410b80f0, 0x14020e8f0> as *const ()), // sub_1410b5ab0
    Replacement::draft(0x1410b5cb0, "coregame_1410b5cb0", tpl_1094::<0x143cdb0a8, 0x1410b8290, 0x14020e8f0> as *const ()), // sub_1410b5cb0
    Replacement::draft(0x1410b5eb0, "coregame_1410b5eb0", fn_1410b5eb0 as *const ()), // sub_1410b5eb0
    Replacement::draft_throws(0x1410b60c0, "coregame_1410b60c0", tpl_20413::<0x143cdb044, 0x143cdaf30, 0x1410b8480, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338be86, 0x142646960, 0x143cdaf50, 0x143cdaf70, 0x143cdaf80, 0x143cdaf88, 0x143cdaf8c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::HeightfieldLayerCom
    Replacement::draft_throws(0x1410b6300, "coregame_1410b6300", tpl_6039::<0x143cdae50, 0x143cdaaf0, 0x1411e9f50, 0x1410baef0, 0x143cdaae8, 0x1420af120, 0x1410b90c0, 0x1420a28e0, 0x1410b9600, 0x1410b86b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338bc46, 0x142646960, 0x143cdab10, 0x143cdab14, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::applyHideStack>
    Replacement::draft_throws(0x1410b6600, "coregame_1410b6600", tpl_6039::<0x143cdafe8, 0x143cdab20, 0x1411e9f50, 0x1410bae00, 0x143cdaf10, 0x1420af120, 0x1410b90c0, 0x1420a28e0, 0x1410b97b0, 0x1410b8790, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338c366, 0x142646960, 0x143cdab40, 0x143cdab44, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::applyHideLayer>
    Replacement::draft_throws(0x1410b6900, "coregame_1410b6900", tpl_19806::<0x143cdaac4, 0x143cdae28, 0x1411e9f50, 0x1410bacc0, 0x143cdaecc, 0x1420af120, 0x1410b90c0, 0x1420a28e0, 0x1410b9960, 0x1410b8870, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338c076, 0x142646960, 0x143cdae48, 0x143cdae4c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::streamInStack>
    Replacement::draft_throws(0x1410b6bf0, "coregame_1410b6bf0", tpl_19806::<0x143cda974, 0x143cdac28, 0x1411e9f50, 0x1410bab80, 0x143cdafec, 0x1420af120, 0x1410b90c0, 0x1420a28e0, 0x1410b9b10, 0x1410b8950, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338b436, 0x142646960, 0x143cdac48, 0x143cdac4c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::streamInLayer>
    Replacement::draft_throws(0x1410b6ee0, "coregame_1410b6ee0", tpl_6039::<0x143cda9ac, 0x143cdb048, 0x1411e9f50, 0x1410baa90, 0x143cdae58, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410b9cc0, 0x1410b8a30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338b256, 0x142646960, 0x143cdb068, 0x143cdb06c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::streamOutStack>
    Replacement::draft_throws(0x1410b71e0, "coregame_1410b71e0", tpl_6039::<0x143cdaa24, 0x143cda978, 0x1411e9f50, 0x1410ba980, 0x143cdafe4, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410b9db0, 0x1410b8ac0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338ba16, 0x142646960, 0x143cda998, 0x143cda99c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::streamOutLayer>
    Replacement::draft_throws(0x1410b74e0, "coregame_1410b74e0", fn_1410b74e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::streamOutLayerWhenHotloading>
    Replacement::draft_throws(0x1410b7870, "coregame_1410b7870", tpl_20526::<0x143cdac98, 0x143cdab58, 0x1411e9f50, 0x1410ba210, 0x143cdac50, 0x1401fc320, 0x1410b90c0, 0x1420a28e0, 0x1410ba050, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338a436, 0x142646960, 0x143cdab78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::stackRemoved>
    Replacement::draft_throws(0x1410b7b30, "coregame_1410b7b30", tpl_20526::<0x143cdaa20, 0x143cdb078, 0x1411e9f50, 0x1410ba190, 0x143cdac58, 0x1401fc320, 0x1410b90c0, 0x1420a28e0, 0x1410ba0f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338a2c6, 0x142646960, 0x143cdb098, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::heightfield::layerRemoved>
    Replacement::draft(0x1410b7df0, "unknown_1410b7df0", tpl_1439::<0x8, 0x1410b5ab0> as *const ()), // sub_1410b7df0
    Replacement::draft(0x1410b7e20, "unknown_1410b7e20", tpl_20527::<0x58, 0x8, 0x1410b5cb0> as *const ()), // sub_1410b7e20
    Replacement::draft(0x1410b7f60, "unknown_1410b7f60", fn_1410b7f60 as *const ()), // sub_1410b7f60
    Replacement::draft(0x1410b80f0, "unknown_1410b80f0", fn_1410b80f0 as *const ()), // sub_1410b80f0
    Replacement::draft(0x1410b8290, "unknown_1410b8290", fn_1410b8290 as *const ()), // sub_1410b8290
    Replacement::draft_throws(0x1410b8480, "unknown_1410b8480", tpl_3011::<0x1411e9f50, 0x1410bafe0, 0x143cdaae4, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410b9590, 0x1401de410> as *const (), Throws::Discard), // sub_1410b8480
    Replacement::draft(0x1410b8640, "unknown_1410b8640", tpl_20531::<0x7, 0x18, 0x8, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_1410b8640
    Replacement::draft(0x1410b86b0, "unknown_1410b86b0", tpl_64022::<0x88, 0x10088, 0x1410b8fc0, 0x141f3ed00> as *const ()), // sub_1410b86b0
    Replacement::draft(0x1410b8790, "unknown_1410b8790", tpl_64022::<0x88, 0x10088, 0x1410b8ec0, 0x141f3ec70> as *const ()), // sub_1410b8790
    Replacement::draft_throws(0x1410b8870, "unknown_1410b8870", tpl_64494::<0x88, 0x10088, 0x1410b8dc0, 0x141f3e970> as *const (), Throws::Discard), // sub_1410b8870
    Replacement::draft_throws(0x1410b8950, "unknown_1410b8950", tpl_64494::<0x88, 0x10088, 0x1410b8cc0, 0x141f3e560> as *const (), Throws::Discard), // sub_1410b8950
    Replacement::draft(0x1410b8a30, "unknown_1410b8a30", tpl_1898::<0x88, 0x10088, 0x141f3e430> as *const ()), // sub_1410b8a30
    Replacement::draft(0x1410b8ac0, "unknown_1410b8ac0", tpl_63093::<0x143cdac58, 0x142489ee0, 0x141f3e3c0> as *const ()), // sub_1410b8ac0
    Replacement::draft_throws(0x1410b8bc0, "unknown_1410b8bc0", tpl_62933::<0x143cdac58, 0x142489ee0, 0x141f3e1f0> as *const (), Throws::Discard), // sub_1410b8bc0
    Replacement::draft_throws(0x1410b90c0, "unknown_1410b90c0", tpl_1498::<0x143cdad50, 0x143cdafdc, 0x10, 0x1420a25b0, 0x141834a78, 0x143cdabcc, 0x143cdafd8, 0x143cdafe0, 0x141834a0c> as *const (), Throws::Discard), // sub_1410b90c0
    Replacement::draft(0x1410b9590, "unknown_1410b9590", tpl_3028::<0x143cdaae4, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410b5eb0> as *const ()), // sub_1410b9590
    Replacement::draft(0x1410b9600, "unknown_1410b9600", tpl_64023::<0x143cdac50, 0x142489ee0, 0x143c8d87c, 0x141f3ed00> as *const ()), // sub_1410b9600
    Replacement::draft(0x1410b97b0, "unknown_1410b97b0", tpl_64023::<0x143cdac58, 0x142489ee0, 0x143c8d87c, 0x141f3ec70> as *const ()), // sub_1410b97b0
    Replacement::draft_throws(0x1410b9960, "unknown_1410b9960", tpl_64499::<0x143cdad10, 0x142489ee0, 0x143c8d120, 0x141f3e970> as *const (), Throws::Discard), // sub_1410b9960
    Replacement::draft_throws(0x1410b9b10, "unknown_1410b9b10", tpl_64499::<0x143cdb0a8, 0x142489ee0, 0x143c8d120, 0x141f3e560> as *const (), Throws::Discard), // sub_1410b9b10
    Replacement::draft(0x1410b9cc0, "unknown_1410b9cc0", tpl_61552::<0x8, 0x88, 0x10088, 0x141f3e430> as *const ()), // sub_1410b9cc0
    Replacement::draft(0x1410b9db0, "unknown_1410b9db0", tpl_63099::<0x143cdac58, 0x142489ee0, 0x141f3e3c0> as *const ()), // sub_1410b9db0
    Replacement::draft_throws(0x1410b9f00, "unknown_1410b9f00", tpl_62934::<0x143cdac58, 0x142489ee0, 0x141f3e1f0> as *const (), Throws::Discard), // sub_1410b9f00
    Replacement::draft_throws(0x1410ba050, "unknown_1410ba050", tpl_5499::<0x143cdac50, 0x5e308, 0x10, 0x8, 0x18, 0x141f3e170, 0x140489dd0> as *const (), Throws::Discard), // sub_1410ba050
    Replacement::draft_throws(0x1410ba0f0, "unknown_1410ba0f0", tpl_5499::<0x143cdac58, 0x5e308, 0x10, 0x8, 0x18, 0x141f3e0f0, 0x1410ba610> as *const (), Throws::Discard), // sub_1410ba0f0
    Replacement::draft(0x1410ba190, "unknown_1410ba190", tpl_3188::<0x1410ba740, 0x14338c570, 0x1410ba610, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410ba190
    Replacement::draft(0x1410ba210, "unknown_1410ba210", tpl_3188::<0x1410ba7e0, 0x14338c5d8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410ba210
    Replacement::draft(0x1410ba4a0, "unknown_1410ba4a0", fn_1410ba4a0 as *const ()), // sub_1410ba4a0
    Replacement::draft(0x1410ba610, "unknown_1410ba610", fn_1410ba610 as *const ()), // sub_1410ba610
    Replacement::draft(0x1410ba740, "unknown_1410ba740", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14338c570, 0x8, 0x20> as *const ()), // sub_1410ba740
    Replacement::draft(0x1410ba7e0, "unknown_1410ba7e0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14338c5d8, 0x8, 0x20> as *const ()), // sub_1410ba7e0
    Replacement::draft(0x1410bb550, "unknown_1410bb550", fn_1410bb550 as *const ()), // sub_1410bb550
    Replacement::draft(0x1410bb5f0, "coregame_1410bb5f0", tpl_6373::<0x143cdb0e0, 0x143cdb0b0, 0x1411e9f50, 0x143c98798, 0x140215610, 0x143c904a0, 0x1420a28e0, 0x1410bb8c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338c636, 0x142646960, 0x143cdb0d0, 0x143cdb0d8, 0x143cdb0dc, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::gyro_input::gravity_vector::system>
    Replacement::draft_throws(0x1410bb8c0, "unknown_1410bb8c0", tpl_1704::<0x143c904a0, 0x143c98798, 0x580e0, 0x141f3f540> as *const (), Throws::Discard), // sub_1410bb8c0
    Replacement::draft(0x1410bb8f0, "unknown_1410bb8f0", fn_1410bb8f0 as *const ()), // sub_1410bb8f0
    Replacement::draft(0x1410bba30, "unknown_1410bba30", fn_1410bba30 as *const ()), // sub_1410bba30
    Replacement::draft(0x1410bbb70, "unknown_1410bbb70", fn_1410bbb70 as *const ()), // sub_1410bbb70
    Replacement::draft(0x1410bbc80, "unknown_1410bbc80", fn_1410bbc80 as *const ()), // sub_1410bbc80
    Replacement::draft(0x1410bbfc0, "unknown_1410bbfc0", fn_1410bbfc0 as *const ()), // sub_1410bbfc0
    Replacement::draft(0x1410bc110, "unknown_1410bc110", fn_1410bc110 as *const ()), // sub_1410bc110
    Replacement::draft(0x1410bc2d0, "unknown_1410bc2d0", tpl_381::<0x1410bc590, 0x140fc5740> as *const ()), // sub_1410bc2d0
    Replacement::draft(0x1410bc430, "unknown_1410bc430", tpl_381::<0x1410bc800, 0x140fc5a40> as *const ()), // sub_1410bc430
    Replacement::draft(0x1410bc590, "unknown_1410bc590", fn_1410bc590 as *const ()), // sub_1410bc590
    Replacement::draft(0x1410bc800, "unknown_1410bc800", fn_1410bc800 as *const ()), // sub_1410bc800
    Replacement::draft_throws(0x1410bca70, "sparsehash_1410bca70", tpl_77984::<0x140227220, 0x140fbf1e0, 0x140227190, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1410bcca2> as *const (), Throws::Discard), // sub_1410bca70
    Replacement::draft_throws(0x1410bccb0, "coregame_1410bccb0", fn_1410bccb0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::global_id_to_entity_map::update>
    Replacement::draft(0x1410bcfe0, "coregame_1410bcfe0", tpl_1095::<0x143c97e7c, 0x14020e8f0> as *const ()), // sub_1410bcfe0
    Replacement::draft_throws(0x1410bd1c0, "coregame_1410bd1c0", tpl_3395::<0x143cdb160, 0x143cdb198, 0x1410bd3d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338c866, 0x142646960, 0x143cdb1b8, 0x143cdb1c0, 0x143cdb1c2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::global_id_to_entity_map::cleanup>
    Replacement::draft_throws(0x1410bd3d0, "unknown_1410bd3d0", fn_1410bd3d0 as *const (), Throws::Discard), // sub_1410bd3d0
    Replacement::draft_throws(0x1410bd5c0, "unknown_1410bd5c0", tpl_1498::<0x143cdb244, 0x143cdb18c, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8d08c, 0x143cdb188, 0x143cdb190, 0x141834a0c> as *const (), Throws::Discard), // sub_1410bd5c0
    Replacement::draft_throws(0x1410bd6a0, "unknown_1410bd6a0", tpl_6367::<0x143cdb240, 0x20, 0x8, 0x141f410b0> as *const (), Throws::Discard), // sub_1410bd6a0
    Replacement::draft(0x1410bd700, "unknown_1410bd700", fn_1410bd700 as *const ()), // sub_1410bd700
    Replacement::draft(0x1410bd8f0, "unknown_1410bd8f0", tpl_3188::<0x1410bdc30, 0x14338cdd0, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410bd8f0
    Replacement::draft_throws(0x1410bd9c0, "unknown_1410bd9c0", fn_1410bd9c0 as *const (), Throws::Caught), // sub_1410bd9c0
    Replacement::draft(0x1410bdc30, "unknown_1410bdc30", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14338cdd0, 0x8, 0x20> as *const ()), // sub_1410bdc30
    Replacement::draft(0x1410bddf0, "unknown_1410bddf0", tpl_19579::<0x14330bca8, 0x20> as *const ()), // sub_1410bddf0
    Replacement::draft(0x1410bde00, "unknown_1410bde00", fn_1410bde00 as *const ()), // sub_1410bde00
    Replacement::draft(0x1410bdec0, "unknown_1410bdec0", fn_1410bdec0 as *const ()), // sub_1410bdec0
    Replacement::draft(0x1410bdf20, "unknown_1410bdf20", fn_1410bdf20 as *const ()), // sub_1410bdf20
    Replacement::draft(0x1410bdf50, "unknown_1410bdf50", fn_1410bdf50 as *const ()), // sub_1410bdf50
    Replacement::draft(0x1410bdfc0, "unknown_1410bdfc0", fn_1410bdfc0 as *const ()), // sub_1410bdfc0
    Replacement::draft(0x1410be050, "unknown_1410be050", fn_1410be050 as *const ()), // sub_1410be050
    Replacement::draft(0x1410be090, "unknown_1410be090", fn_1410be090 as *const ()), // sub_1410be090
    Replacement::draft(0x1410be1a0, "unknown_1410be1a0", tpl_19541::<0x1401de830, 0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_1410be1a0
    Replacement::draft(0x1410be2c0, "unknown_1410be2c0", tpl_20552::<0x10, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_1410be2c0
    Replacement::draft(0x1410be310, "unknown_1410be310", tpl_20553::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_1410be310
    Replacement::draft(0x1410be4b0, "unknown_1410be4b0", fn_1410be4b0 as *const ()), // sub_1410be4b0
    Replacement::draft(0x1410be550, "unknown_1410be550", tpl_64515::<0x1426467e8, 0x1410c43d0, 0x1410be5c9> as *const ()), // sub_1410be550
    Replacement::draft(0x1410be720, "unknown_1410be720", fn_1410be720 as *const ()), // sub_1410be720
    Replacement::draft(0x1410be890, "unknown_1410be890", fn_1410be890 as *const ()), // sub_1410be890
    Replacement::draft(0x1410bea00, "unknown_1410bea00", fn_1410bea00 as *const ()), // sub_1410bea00
    Replacement::draft(0x1410beb70, "unknown_1410beb70", fn_1410beb70 as *const ()), // sub_1410beb70
    Replacement::draft(0x1410bec40, "unknown_1410bec40", fn_1410bec40 as *const ()), // sub_1410bec40
    Replacement::draft(0x1410bece0, "unknown_1410bece0", fn_1410bece0 as *const ()), // sub_1410bece0
    Replacement::draft(0x1410bed90, "unknown_1410bed90", fn_1410bed90 as *const ()), // sub_1410bed90
    Replacement::draft_throws(0x1410beec0, "unknown_1410beec0", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c24f0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410beec0
    Replacement::draft_throws(0x1410bf160, "unknown_1410bf160", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c27c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410bf160
    Replacement::draft_throws(0x1410bf400, "unknown_1410bf400", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c29f0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410bf400
    Replacement::draft_throws(0x1410bf6a0, "unknown_1410bf6a0", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c2c40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410bf6a0
    Replacement::draft_throws(0x1410bf940, "unknown_1410bf940", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c2e90, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410bf940
    Replacement::draft_throws(0x1410bfbe0, "unknown_1410bfbe0", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c30d0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410bfbe0
    Replacement::draft_throws(0x1410bfe80, "unknown_1410bfe80", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c3420, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410bfe80
    Replacement::draft_throws(0x1410c0120, "unknown_1410c0120", tpl_1480::<0x1429062e0, 0x1410d6ca0, 0x1410c3620, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410c0120
    Replacement::draft(0x1410c03c0, "unknown_1410c03c0", fn_1410c03c0 as *const ()), // sub_1410c03c0
    Replacement::draft(0x1410c05d0, "unknown_1410c05d0", tpl_867::<0x8, 0x1410c3eb0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410c05d0
    Replacement::draft(0x1410c0680, "unknown_1410c0680", tpl_20563::<0x8, 0x1410c3f20, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410c0680
    Replacement::draft(0x1410c0740, "unknown_1410c0740", tpl_20563::<0x8, 0x1410c3f90, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410c0740
    Replacement::draft(0x1410c0950, "unknown_1410c0950", fn_1410c0950 as *const ()), // sub_1410c0950
    Replacement::draft(0x1410c0a40, "unknown_1410c0a40", tpl_10126::<0x8, 0x1410c4050, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410c0a40
    Replacement::draft(0x1410c0b00, "unknown_1410c0b00", fn_1410c0b00 as *const ()), // sub_1410c0b00
    Replacement::draft(0x1410c0ba0, "unknown_1410c0ba0", tpl_1094::<0x143cdb3fc, 0x1410c40c0, 0x14020e8f0> as *const ()), // sub_1410c0ba0
    Replacement::draft(0x1410c0da0, "unknown_1410c0da0", tpl_1094::<0x143cdb414, 0x1410c4250, 0x14020e8f0> as *const ()), // sub_1410c0da0
    Replacement::draft_throws(0x1410c0fa0, "unknown_1410c0fa0", tpl_6282::<0x143cdb3f8, 0x1410c4520, 0x14020e8f0> as *const (), Throws::Discard), // sub_1410c0fa0
    Replacement::draft(0x1410c11a0, "unknown_1410c11a0", tpl_1094::<0x143c9f930, 0x1410c4740, 0x14020e8f0> as *const ()), // sub_1410c11a0
    Replacement::draft(0x1410c13a0, "unknown_1410c13a0", tpl_1094::<0x143cd65e0, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_1410c13a0
    Replacement::draft(0x1410c15a0, "unknown_1410c15a0", tpl_1094::<0x143cdb4d8, 0x1410c48e0, 0x14020e8f0> as *const ()), // sub_1410c15a0
    Replacement::draft(0x1410c17a0, "unknown_1410c17a0", fn_1410c17a0 as *const ()), // sub_1410c17a0
    Replacement::draft_throws(0x1410c19b0, "coregame_1410c19b0", tpl_19367::<0x143cdb630, 0x143cdb830, 0x1410c4ae0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338e3d6, 0x142646960, 0x143cdb850, 0x143cdb870, 0x143cdb880, 0x143cdb884, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::GfxGraphComponent>>
    Replacement::draft_throws(0x1410c1be0, "coregame_1410c1be0", tpl_19901::<0x143cdbc90, 0x143cdbbd0, 0x1411e9f50, 0x1410cabb0, 0x143cdb93c, 0x1420af120, 0x1404c2240, 0x1420a28e0, 0x1410c8060, 0x1410c5b70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338e5b6, 0x142646960, 0x143cdbbf0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::streamInSpawner>
    Replacement::draft_throws(0x1410c1ed0, "coregame_1410c1ed0", tpl_6140::<0x143cdbc6c, 0x143cdb430, 0x1411e9f50, 0x1410caa00, 0x143cdbbf4, 0x1420af120, 0x14027c6a0, 0x143cada18, 0x140215610, 0x1420a28e0, 0x1410c8280, 0x1410c5c90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338ff56, 0x142646960, 0x143cdb450, 0x143cdb454, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::streamInInstance>
    Replacement::draft_throws(0x1410c21f0, "coregame_1410c21f0", tpl_6039::<0x143cdb62c, 0x143cdb560, 0x1411e9f50, 0x1410ca910, 0x143cdb544, 0x1420af120, 0x1410c7960, 0x1420a28e0, 0x1410c8460, 0x1410c5de0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143390316, 0x142646960, 0x143cdb580, 0x143cdb584, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::streamOutInstance>
    Replacement::draft_throws(0x1410c24f0, "coregame_1410c24f0", fn_1410c24f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::updateGlobal>
    Replacement::draft_throws(0x1410c27c0, "coregame_1410c27c0", tpl_1492::<0x143cdbb00, 0x143cdb4e0, 0x1411e9f50, 0x1410c5ee0, 0x1410c85e0, 0x1410c60e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338fcc6, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::updateState>
    Replacement::draft_throws(0x1410c29f0, "coregame_1410c29f0", tpl_3654::<0x143cdbb90, 0x143cdbc00, 0x1411e9f50, 0x1410c6220, 0x1410c8790, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338ec96, 0x142646960, 0x143cdbc20, 0x143cdbc28, 0x143cdbc2a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::collectActiveInstances>
    Replacement::draft_throws(0x1410c2c40, "coregame_1410c2c40", tpl_1584::<0x143cdb8a4, 0x143cdb320, 0x1411e9f50, 0x1410c6420, 0x1410c8830, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338ef26, 0x142646960, 0x143cdb340, 0x143cdb348, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::updateActiveInstances>
    Replacement::draft_throws(0x1410c2e90, "coregame_1410c2e90", tpl_3005::<0x143cdbcd0, 0x143cdbaa0, 0x1411e9f50, 0x1410c66c0, 0x1410c88b0, 0x1410c68d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338e8f6, 0x142646960, 0x143cdbac0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::updateDrawSetup>
    Replacement::draft_throws(0x1410c30d0, "coregame_1410c30d0", fn_1410c30d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::destructSpawner>
    Replacement::draft_throws(0x1410c3420, "coregame_1410c3420", tpl_3589::<0x143cdb74c, 0x143cdb388, 0x1410c4ca0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338d236, 0x142646960, 0x143cdb3a8, 0x143cdb3ac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::destructInstance>
    Replacement::draft_throws(0x1410c3620, "coregame_1410c3620", tpl_1578::<0x143cdba74, 0x143cdb9b8, 0x1411e9f50, 0x1410c6a10, 0x1410c8bd0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14338d396, 0x142646960, 0x143cdb9d8, 0x143cdb9dc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::gfxgraph::destructDrawSetup>
    Replacement::draft(0x1410c38a0, "unknown_1410c38a0", tpl_20568::<0x10, 0x8, 0x1410c15a0> as *const ()), // sub_1410c38a0
    Replacement::draft(0x1410c38f0, "unknown_1410c38f0", fn_1410c38f0 as *const ()), // sub_1410c38f0
    Replacement::draft(0x1410c3eb0, "unknown_1410c3eb0", fn_1410c3eb0 as *const ()), // sub_1410c3eb0
    Replacement::draft(0x1410c3f20, "unknown_1410c3f20", fn_1410c3f20 as *const ()), // sub_1410c3f20
    Replacement::draft(0x1410c3f90, "unknown_1410c3f90", fn_1410c3f90 as *const ()), // sub_1410c3f90
    Replacement::draft(0x1410c4000, "unknown_1410c4000", fn_1410c4000 as *const ()), // sub_1410c4000
    Replacement::draft(0x1410c4050, "unknown_1410c4050", fn_1410c4050 as *const ()), // sub_1410c4050
    Replacement::draft(0x1410c40c0, "unknown_1410c40c0", tpl_19680::<0x1401e6d50, 0x1410c59a0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_1410c40c0
    Replacement::draft(0x1410c4250, "unknown_1410c4250", tpl_6488::<0x1401e6d50, 0x1410c59d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1410c57b0> as *const ()), // sub_1410c4250
    Replacement::draft_throws(0x1410c4520, "unknown_1410c4520", fn_1410c4520 as *const (), Throws::Discard), // sub_1410c4520
    Replacement::draft(0x1410c4740, "unknown_1410c4740", fn_1410c4740 as *const ()), // sub_1410c4740
    Replacement::draft(0x1410c48e0, "unknown_1410c48e0", fn_1410c48e0 as *const ()), // sub_1410c48e0
    Replacement::draft_throws(0x1410c4ae0, "unknown_1410c4ae0", tpl_3011::<0x1411e9f50, 0x1410cacf0, 0x143cdb42c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410c7ff0, 0x1401de410> as *const (), Throws::Discard), // sub_1410c4ae0
    Replacement::draft_throws(0x1410c4ca0, "unknown_1410c4ca0", fn_1410c4ca0 as *const (), Throws::Discard), // sub_1410c4ca0
    Replacement::draft(0x1410c4e50, "unknown_1410c4e50", fn_1410c4e50 as *const ()), // sub_1410c4e50
    Replacement::draft(0x1410c57b0, "unknown_1410c57b0", fn_1410c57b0 as *const ()), // sub_1410c57b0
    Replacement::draft(0x1410c5890, "unknown_1410c5890", fn_1410c5890 as *const ()), // sub_1410c5890
    Replacement::draft(0x1410c58b0, "unknown_1410c58b0", fn_1410c58b0 as *const ()), // sub_1410c58b0
    Replacement::draft(0x1410c58f0, "unknown_1410c58f0", fn_1410c58f0 as *const ()), // sub_1410c58f0
    Replacement::draft(0x1410c5910, "unknown_1410c5910", fn_1410c5910 as *const ()), // sub_1410c5910
    Replacement::draft(0x1410c5950, "unknown_1410c5950", fn_1410c5950 as *const ()), // sub_1410c5950
    Replacement::draft(0x1410c59a0, "unknown_1410c59a0", tpl_1955::<0x10, 0x1410bdec0> as *const ()), // sub_1410c59a0
    Replacement::draft(0x1410c59d0, "unknown_1410c59d0", fn_1410c59d0 as *const ()), // sub_1410c59d0
    Replacement::draft(0x1410c5a40, "unknown_1410c5a40", tpl_1955::<0x28, 0x140429080> as *const ()), // sub_1410c5a40
    Replacement::draft(0x1410c7ff0, "unknown_1410c7ff0", tpl_3028::<0x143cdb42c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410c17a0> as *const ()), // sub_1410c7ff0
    Replacement::draft_throws(0x1410c8060, "unknown_1410c8060", tpl_78195::<0x143c8d160, 0x142489ee0, 0x143c8f7a0, 0x143cdb4d8, 0x141f42f30> as *const (), Throws::Discard), // sub_1410c8060
    Replacement::draft_throws(0x1410c8280, "unknown_1410c8280", tpl_78056::<0x143cada18, 0x1410c7a40, 0x141f456d0> as *const (), Throws::Discard), // sub_1410c8280
    Replacement::draft(0x1410c8460, "unknown_1410c8460", tpl_63099::<0x143cdb3fc, 0x142489ee0, 0x141f454a0> as *const ()), // sub_1410c8460
    Replacement::draft_throws(0x1410c85b0, "unknown_1410c85b0", fn_1410c85b0 as *const (), Throws::Discard), // sub_1410c85b0
    Replacement::draft_throws(0x1410c85e0, "unknown_1410c85e0", fn_1410c85e0 as *const (), Throws::Discard), // sub_1410c85e0
    Replacement::draft_throws(0x1410c8790, "unknown_1410c8790", tpl_19259::<0x143cdb988, 0x143cdb400, 0x143cdb540, 0x20, 0x143c8cea0, 0x8, 0x580e0, 0x143c8ceec, 0x141f447f0> as *const (), Throws::Discard), // sub_1410c8790
    Replacement::draft_throws(0x1410c8830, "unknown_1410c8830", tpl_19963::<0x143cdb40c, 0x20, 0x143c8cea0, 0x8, 0x580e0, 0x143cdb400, 0x141f444f0> as *const (), Throws::Discard), // sub_1410c8830
    Replacement::draft(0x1410c88b0, "unknown_1410c88b0", fn_1410c88b0 as *const ()), // sub_1410c88b0
    Replacement::draft(0x1410c8a60, "unknown_1410c8a60", tpl_6144::<0x143cd65e0, 0x5e308, 0x10, 0x8, 0x18, 0x141f42d20, 0x140f82730> as *const ()), // sub_1410c8a60
    Replacement::draft(0x1410c8b00, "unknown_1410c8b00", tpl_20653::<0x143cdb3fc, 0x5e308, 0x580e0, 0x143cdb628, 0x143cada18, 0x10, 0x8, 0x18, 0x141f45430, 0x1410c9900> as *const ()), // sub_1410c8b00
    Replacement::draft(0x1410c8bd0, "unknown_1410c8bd0", tpl_20653::<0x143cdb414, 0x5e308, 0x580e0, 0x143ccef30, 0x143c9f9c0, 0x10, 0x8, 0x18, 0x141f43800, 0x1410c9b70> as *const ()), // sub_1410c8bd0
    Replacement::draft(0x1410c8ca0, "unknown_1410c8ca0", tpl_3188::<0x1410ca420, 0x14338ce88, 0x1410c9b70, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410c8ca0
    Replacement::draft(0x1410c8d20, "unknown_1410c8d20", tpl_3188::<0x1410ca4c0, 0x14338ce60, 0x1410c9900, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410c8d20
    Replacement::draft(0x1410c8da0, "unknown_1410c8da0", tpl_3188::<0x1410ca560, 0x14338cdf8, 0x140f82730, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410c8da0
    Replacement::draft(0x1410c8e20, "unknown_1410c8e20", fn_1410c8e20 as *const ()), // sub_1410c8e20
    Replacement::draft(0x1410c91c0, "unknown_1410c91c0", tpl_4348::<0x18, 0x20, 0x1410c9900> as *const ()), // sub_1410c91c0
    Replacement::draft(0x1410c91f0, "unknown_1410c91f0", tpl_4348::<0x18, 0x20, 0x1410c9b70> as *const ()), // sub_1410c91f0
    Replacement::draft(0x1410c97a0, "unknown_1410c97a0", fn_1410c97a0 as *const ()), // sub_1410c97a0
    Replacement::draft(0x1410c9900, "unknown_1410c9900", tpl_4821::<0x1401de830, 0x1410bdec0, 0x1425371a0, 0x142533200> as *const ()), // sub_1410c9900
    Replacement::draft(0x1410c9a20, "unknown_1410c9a20", fn_1410c9a20 as *const ()), // sub_1410c9a20
    Replacement::draft(0x1410c9b70, "unknown_1410c9b70", fn_1410c9b70 as *const ()), // sub_1410c9b70
    Replacement::draft(0x1410c9cb0, "unknown_1410c9cb0", fn_1410c9cb0 as *const ()), // sub_1410c9cb0
    Replacement::draft(0x1410c9fc0, "unknown_1410c9fc0", fn_1410c9fc0 as *const ()), // sub_1410c9fc0
    Replacement::draft(0x1410ca170, "unknown_1410ca170", fn_1410ca170 as *const ()), // sub_1410ca170
    Replacement::draft(0x1410ca270, "unknown_1410ca270", tpl_19567::<0x1410cb120, 0x1410be1a0, 0x1403d79a0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1403c7e70, 0x1401de830, 0x142442de0> as *const ()), // sub_1410ca270
    Replacement::draft(0x1410ca420, "unknown_1410ca420", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14338ce88, 0x8, 0x20> as *const ()), // sub_1410ca420
    Replacement::draft(0x1410ca4c0, "unknown_1410ca4c0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14338ce60, 0x8, 0x20> as *const ()), // sub_1410ca4c0
    Replacement::draft(0x1410ca560, "unknown_1410ca560", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14338cdf8, 0x8, 0x20> as *const ()), // sub_1410ca560
    Replacement::draft(0x1410caa00, "unknown_1410caa00", fn_1410caa00 as *const ()), // sub_1410caa00
    Replacement::draft(0x1410cade0, "unknown_1410cade0", fn_1410cade0 as *const ()), // sub_1410cade0
    Replacement::draft(0x1410cae60, "unknown_1410cae60", tpl_1094::<0x143cd65e4, 0x1410cb2f0, 0x14020e8f0> as *const ()), // sub_1410cae60
    Replacement::draft(0x1410cb060, "unknown_1410cb060", fn_1410cb060 as *const ()), // sub_1410cb060
    Replacement::draft(0x1410cb120, "unknown_1410cb120", fn_1410cb120 as *const ()), // sub_1410cb120
    Replacement::draft(0x1410cb2f0, "unknown_1410cb2f0", tpl_20666::<0x1401e6d50, 0x1410cb470, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14106a9c0> as *const ()), // sub_1410cb2f0
    Replacement::draft(0x1410cb470, "unknown_1410cb470", fn_1410cb470 as *const ()), // sub_1410cb470
    Replacement::draft(0x1410cb8d0, "unknown_1410cb8d0", fn_1410cb8d0 as *const ()), // sub_1410cb8d0
    Replacement::draft(0x1410cba50, "unknown_1410cba50", fn_1410cba50 as *const ()), // sub_1410cba50
    Replacement::draft(0x1410cba70, "unknown_1410cba70", fn_1410cba70 as *const ()), // sub_1410cba70
    Replacement::draft(0x1410cbac0, "unknown_1410cbac0", fn_1410cbac0 as *const ()), // sub_1410cbac0
    Replacement::draft(0x1410cbb70, "unknown_1410cbb70", tpl_6045::<0x30, 0x143cd4dcc, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1410cbbee> as *const ()), // sub_1410cbb70
    Replacement::draft(0x1410cbbf0, "unknown_1410cbbf0", tpl_3936::<0x30, 0x143c8f6dc, 0x180a8, 0x18090, 0x142489ee0, 0x140411c40, 0x1426467e8, 0x1410cbc56> as *const ()), // sub_1410cbbf0
    Replacement::draft(0x1410cbc60, "unknown_1410cbc60", tpl_3936::<0x30, 0x143c8f7a0, 0x180a8, 0x18090, 0x142489ee0, 0x140411bb0, 0x1426467e8, 0x1410cbcc6> as *const ()), // sub_1410cbc60
    Replacement::draft(0x1410cbcd0, "unknown_1410cbcd0", fn_1410cbcd0 as *const ()), // sub_1410cbcd0
    Replacement::draft(0x1410cbd60, "unknown_1410cbd60", fn_1410cbd60 as *const ()), // sub_1410cbd60
    Replacement::draft(0x1410cbdf0, "unknown_1410cbdf0", fn_1410cbdf0 as *const ()), // sub_1410cbdf0
    Replacement::draft(0x1410cbe80, "unknown_1410cbe80", fn_1410cbe80 as *const ()), // sub_1410cbe80
    Replacement::draft(0x1410cbef0, "unknown_1410cbef0", tpl_6045::<0x30, 0x143c9c6e4, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1410cbf6e> as *const ()), // sub_1410cbef0
    Replacement::draft_throws(0x1410cbf70, "unknown_1410cbf70", fn_1410cbf70 as *const (), Throws::Discard), // sub_1410cbf70
    Replacement::draft_throws(0x1410cc030, "unknown_1410cc030", fn_1410cc030 as *const (), Throws::Discard), // sub_1410cc030
    Replacement::draft_throws(0x1410cc0b0, "unknown_1410cc0b0", fn_1410cc0b0 as *const (), Throws::Discard), // sub_1410cc0b0
    Replacement::draft(0x1410cc130, "unknown_1410cc130", fn_1410cc130 as *const ()), // sub_1410cc130
    Replacement::draft_throws(0x1410cc1a0, "unknown_1410cc1a0", fn_1410cc1a0 as *const (), Throws::Discard), // sub_1410cc1a0
    Replacement::draft_throws(0x1410cc220, "unknown_1410cc220", fn_1410cc220 as *const (), Throws::Discard), // sub_1410cc220
    Replacement::draft(0x1410cc2e0, "unknown_1410cc2e0", tpl_3942::<0x30, 0x143c8f7a0, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1410cc330> as *const ()), // sub_1410cc2e0
    Replacement::draft(0x1410cc340, "unknown_1410cc340", fn_1410cc340 as *const ()), // sub_1410cc340
    Replacement::draft_throws(0x1410cc3c0, "unknown_1410cc3c0", fn_1410cc3c0 as *const (), Throws::Discard), // sub_1410cc3c0
    Replacement::draft_throws(0x1410cc480, "unknown_1410cc480", fn_1410cc480 as *const (), Throws::Caught), // sub_1410cc480
    Replacement::draft(0x1410cc570, "unknown_1410cc570", tpl_3942::<0x30, 0x143c8d120, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1410cc5c0> as *const ()), // sub_1410cc570
    Replacement::draft(0x1410cc5d0, "unknown_1410cc5d0", tpl_4008::<0x30, 0x143c8f6d8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1410cc653> as *const ()), // sub_1410cc5d0
    Replacement::draft(0x1410cc660, "unknown_1410cc660", fn_1410cc660 as *const ()), // sub_1410cc660
    Replacement::draft_throws(0x1410cc750, "unknown_1410cc750", fn_1410cc750 as *const (), Throws::Discard), // sub_1410cc750
    Replacement::draft_throws(0x1410cc830, "unknown_1410cc830", fn_1410cc830 as *const (), Throws::Discard), // sub_1410cc830
    Replacement::draft(0x1410cc8f0, "unknown_1410cc8f0", fn_1410cc8f0 as *const ()), // sub_1410cc8f0
    Replacement::draft_throws(0x1410cc9f0, "unknown_1410cc9f0", fn_1410cc9f0 as *const (), Throws::Discard), // sub_1410cc9f0
    Replacement::draft(0x1410ccab0, "unknown_1410ccab0", tpl_3942::<0x30, 0x143cc5938, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1410ccb00> as *const ()), // sub_1410ccab0
    Replacement::draft(0x1410ccb10, "unknown_1410ccb10", fn_1410ccb10 as *const ()), // sub_1410ccb10
    Replacement::draft_throws(0x1410ccb90, "unknown_1410ccb90", fn_1410ccb90 as *const (), Throws::Discard), // sub_1410ccb90
    Replacement::draft_throws(0x1410ccc60, "unknown_1410ccc60", fn_1410ccc60 as *const (), Throws::Caught), // sub_1410ccc60
    Replacement::draft_throws(0x1410ccd00, "unknown_1410ccd00", fn_1410ccd00 as *const (), Throws::Discard), // sub_1410ccd00
    Replacement::draft(0x1410ccd70, "unknown_1410ccd70", fn_1410ccd70 as *const ()), // sub_1410ccd70
    Replacement::draft_throws(0x1410cce60, "unknown_1410cce60", fn_1410cce60 as *const (), Throws::Caught), // sub_1410cce60
    Replacement::draft_throws(0x1410ccff0, "unknown_1410ccff0", fn_1410ccff0 as *const (), Throws::Caught), // sub_1410ccff0
    Replacement::draft_throws(0x1410cd180, "unknown_1410cd180", fn_1410cd180 as *const (), Throws::Caught), // sub_1410cd180
    Replacement::draft_throws(0x1410cd330, "unknown_1410cd330", fn_1410cd330 as *const (), Throws::Caught), // sub_1410cd330
    Replacement::draft_throws(0x1410cd4c0, "unknown_1410cd4c0", fn_1410cd4c0 as *const (), Throws::Caught), // sub_1410cd4c0
    Replacement::draft_throws(0x1410cd650, "unknown_1410cd650", fn_1410cd650 as *const (), Throws::Caught), // sub_1410cd650
    Replacement::draft_throws(0x1410cd7e0, "unknown_1410cd7e0", fn_1410cd7e0 as *const (), Throws::Caught), // sub_1410cd7e0
    Replacement::draft(0x1410cd970, "unknown_1410cd970", fn_1410cd970 as *const ()), // sub_1410cd970
    Replacement::draft(0x1410cda40, "unknown_1410cda40", fn_1410cda40 as *const ()), // sub_1410cda40
    Replacement::draft(0x1410cda70, "unknown_1410cda70", fn_1410cda70 as *const ()), // sub_1410cda70
    Replacement::draft(0x1410cdb10, "unknown_1410cdb10", fn_1410cdb10 as *const ()), // sub_1410cdb10
    Replacement::draft(0x1410cdbb0, "unknown_1410cdbb0", fn_1410cdbb0 as *const ()), // sub_1410cdbb0
    Replacement::draft(0x1410cdbd0, "unknown_1410cdbd0", fn_1410cdbd0 as *const ()), // sub_1410cdbd0
    Replacement::draft(0x1410cdc70, "unknown_1410cdc70", fn_1410cdc70 as *const ()), // sub_1410cdc70
    Replacement::draft(0x1410cdd10, "unknown_1410cdd10", fn_1410cdd10 as *const ()), // sub_1410cdd10
    Replacement::draft(0x1410cddb0, "unknown_1410cddb0", fn_1410cddb0 as *const ()), // sub_1410cddb0
    Replacement::draft(0x1410cde50, "unknown_1410cde50", fn_1410cde50 as *const ()), // sub_1410cde50
    Replacement::draft(0x1410cded0, "coregame_1410cded0", fn_1410cded0 as *const ()), // coregame::GameWindow::coregame::GameWindow::vfunc_0
    Replacement::draft(0x1410ce010, "unknown_1410ce010", fn_1410ce010 as *const ()), // sub_1410ce010
    Replacement::draft(0x1410ce180, "unknown_1410ce180", fn_1410ce180 as *const ()), // sub_1410ce180
    Replacement::draft(0x1410ce390, "unknown_1410ce390", fn_1410ce390 as *const ()), // sub_1410ce390
    Replacement::draft(0x1410ce450, "unknown_1410ce450", fn_1410ce450 as *const ()), // sub_1410ce450
    Replacement::draft(0x1410ce740, "unknown_1410ce740", fn_1410ce740 as *const ()), // sub_1410ce740
    Replacement::draft(0x1410ce910, "unknown_1410ce910", fn_1410ce910 as *const ()), // sub_1410ce910
    Replacement::draft(0x1410cecb0, "unknown_1410cecb0", fn_1410cecb0 as *const ()), // sub_1410cecb0
    Replacement::draft(0x1410cee00, "unknown_1410cee00", fn_1410cee00 as *const ()), // sub_1410cee00
    Replacement::draft(0x1410ceee0, "unknown_1410ceee0", fn_1410ceee0 as *const ()), // sub_1410ceee0
    Replacement::draft_throws(0x1410ceef0, "unknown_1410ceef0", fn_1410ceef0 as *const (), Throws::Caught), // sub_1410ceef0
    Replacement::draft(0x1410cf050, "unknown_1410cf050", tpl_2733::<0x8, 0x9, 0x1410cf4c0, 0x1401e5470, 0x142442de0, 0x1401f3d00> as *const ()), // sub_1410cf050
    Replacement::draft(0x1410cf120, "unknown_1410cf120", fn_1410cf120 as *const ()), // sub_1410cf120
    Replacement::draft_throws(0x1410cf2e0, "unknown_1410cf2e0", fn_1410cf2e0 as *const (), Throws::Discard), // sub_1410cf2e0
    Replacement::draft(0x1410cf320, "coregame_1410cf320", fn_1410cf320 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_0
    Replacement::draft(0x1410cf360, "unknown_1410cf360", fn_1410cf360 as *const ()), // sub_1410cf360
    Replacement::draft(0x1410cf3b0, "unknown_1410cf3b0", fn_1410cf3b0 as *const ()), // sub_1410cf3b0
    Replacement::draft(0x1410cf460, "unknown_1410cf460", fn_1410cf460 as *const ()), // sub_1410cf460
    Replacement::draft(0x1410cf4c0, "unknown_1410cf4c0", tpl_20724::<0x1410d0a40, 0x1425371a0, 0x142533200, 0x142442de0, 0x1401f3ee0, 0x1401f3c00> as *const ()), // sub_1410cf4c0
    Replacement::draft(0x1410cf790, "unknown_1410cf790", fn_1410cf790 as *const ()), // sub_1410cf790
    Replacement::draft(0x1410cf8e0, "unknown_1410cf8e0", fn_1410cf8e0 as *const ()), // sub_1410cf8e0
    Replacement::draft(0x1410cfa10, "unknown_1410cfa10", fn_1410cfa10 as *const ()), // sub_1410cfa10
    Replacement::draft_throws(0x1410cfa90, "unknown_1410cfa90", fn_1410cfa90 as *const (), Throws::Caught), // sub_1410cfa90
    Replacement::draft(0x1410cfd60, "unknown_1410cfd60", fn_1410cfd60 as *const ()), // sub_1410cfd60
    Replacement::draft_throws(0x1410d0030, "unknown_1410d0030", fn_1410d0030 as *const (), Throws::Caught), // sub_1410d0030
    Replacement::draft(0x1410d02f0, "unknown_1410d02f0", fn_1410d02f0 as *const ()), // sub_1410d02f0
    Replacement::draft(0x1410d03a0, "unknown_1410d03a0", fn_1410d03a0 as *const ()), // sub_1410d03a0
    Replacement::draft_throws(0x1410d0670, "unknown_1410d0670", fn_1410d0670 as *const (), Throws::Caught), // sub_1410d0670
    Replacement::draft(0x1410d0f50, "unknown_1410d0f50", fn_1410d0f50 as *const ()), // sub_1410d0f50
    Replacement::draft_throws(0x1410d0fa0, "unknown_1410d0fa0", fn_1410d0fa0 as *const (), Throws::Caught), // sub_1410d0fa0
    Replacement::draft(0x1410d19e0, "unknown_1410d19e0", fn_1410d19e0 as *const ()), // sub_1410d19e0
    Replacement::draft(0x1410d1ba0, "unknown_1410d1ba0", fn_1410d1ba0 as *const ()), // sub_1410d1ba0
    Replacement::draft(0x1410d1bf0, "coregame_1410d1bf0", fn_1410d1bf0 as *const ()), // coregame::GameParameterManager::Modifier::coregame::GameParameterManager::Modifier::vfunc_4
    Replacement::draft(0x1410d1c20, "coregame_1410d1c20", fn_1410d1c20 as *const ()), // coregame::GameParameterManager::Modifier::coregame::GameParameterManager::Modifier::vfunc_5
    Replacement::draft(0x1410d1c40, "coregame_1410d1c40", fn_1410d1c40 as *const ()), // sub_1410d1c40
    Replacement::draft(0x1410d1ca0, "coregame_1410d1ca0", fn_1410d1ca0 as *const ()), // coregame::GameParameterManager::ConstantModifier::coregame::GameParameterManager::ConstantModifier::vfunc_1
    Replacement::draft(0x1410d1cd0, "coregame_1410d1cd0", fn_1410d1cd0 as *const ()), // coregame::GameParameterManager::TimelineModifier::coregame::GameParameterManager::TimelineModifier::vfunc_2
    Replacement::draft(0x1410d1ce0, "coregame_1410d1ce0", tpl_586::<0x30> as *const ()), // coregame::GameParameterManager::TimelineModifier::coregame::GameParameterManager::TimelineModifier::vfunc_3
    Replacement::draft(0x1410d1cf0, "coregame_1410d1cf0", fn_1410d1cf0 as *const ()), // coregame::GameParameterManager::ConstantModifier::coregame::GameParameterManager::ConstantModifier::vfunc_5
    Replacement::draft(0x1410d1e10, "coregame_1410d1e10", fn_1410d1e10 as *const ()), // sub_1410d1e10
    Replacement::draft(0x1410d1fe0, "coregame_1410d1fe0", fn_1410d1fe0 as *const ()), // coregame::GameParameterManager::CurveModifier::coregame::GameParameterManager::CurveModifier::vfunc_4
    Replacement::draft(0x1410d2040, "coregame_1410d2040", fn_1410d2040 as *const ()), // coregame::GameParameterManager::CurveModifier::coregame::GameParameterManager::CurveModifier::vfunc_1
    Replacement::draft(0x1410d2070, "coregame_1410d2070", fn_1410d2070 as *const ()), // coregame::GameParameterManager::CurveModifier::coregame::GameParameterManager::CurveModifier::vfunc_2
    Replacement::draft(0x1410d2180, "coregame_1410d2180", fn_1410d2180 as *const ()), // coregame::GameParameterManager::CurveModifier::coregame::GameParameterManager::CurveModifier::vfunc_3
    Replacement::draft(0x1410d2260, "coregame_1410d2260", tpl_9016::<0x38, 0x39> as *const ()), // coregame::GameParameterManager::TimelineModifier::coregame::GameParameterManager::TimelineModifier::vfunc_4
    Replacement::draft(0x1410d22f0, "coregame_1410d22f0", fn_1410d22f0 as *const ()), // coregame::GameParameterManager::CurveModifier::coregame::GameParameterManager::CurveModifier::vfunc_0
    Replacement::draft(0x1410d2360, "coregame_1410d2360", fn_1410d2360 as *const ()), // sub_1410d2360
    Replacement::draft(0x1410d2430, "coregame_1410d2430", fn_1410d2430 as *const ()), // sub_1410d2430
    Replacement::draft(0x1410d2550, "coregame_1410d2550", fn_1410d2550 as *const ()), // sub_1410d2550
    Replacement::draft(0x1410d25f0, "coregame_1410d25f0", fn_1410d25f0 as *const ()), // sub_1410d25f0
    Replacement::draft(0x1410d26c0, "coregame_1410d26c0", fn_1410d26c0 as *const ()), // sub_1410d26c0
    Replacement::draft(0x1410d2790, "coregame_1410d2790", fn_1410d2790 as *const ()), // sub_1410d2790
    Replacement::draft(0x1410d27f0, "coregame_1410d27f0", tpl_20752::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x8, 0x14025f9d0> as *const ()), // sub_1410d27f0
    Replacement::draft(0x1410d28a0, "coregame_1410d28a0", fn_1410d28a0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::game_parameter_manager::update>
    Replacement::draft(0x1410d2b30, "coregame_1410d2b30", tpl_1537::<0x143cdbd88, 0x143cdbd58, 0x1411e9f50, 0x143c8ce80, 0x140215610, 0x1420a28e0, 0x1410d3020, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143391286, 0x142646960, 0x143cdbd78, 0x143cdbd80, 0x143cdbd84, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::game_parameter_manager::fixedUpdate>
    Replacement::draft(0x1410d2dd0, "unknown_1410d2dd0", tpl_19784::<0x142407c80, 0x143c4aa00, 0x141f57300, 0x1426455a8, 0x142645598, 0x141f57280, 0x142645580> as *const ()), // sub_1410d2dd0
    Replacement::draft(0x1410d2f20, "unknown_1410d2f20", fn_1410d2f20 as *const ()), // sub_1410d2f20
    Replacement::draft(0x1410d3000, "unknown_1410d3000", tpl_18946::<0x143c8ce80, 0x580e0, 0x141f573a0> as *const ()), // sub_1410d3000
    Replacement::draft(0x1410d3020, "unknown_1410d3020", fn_1410d3020 as *const ()), // sub_1410d3020
    Replacement::draft(0x1410d3050, "unknown_1410d3050", fn_1410d3050 as *const ()), // sub_1410d3050
    Replacement::draft(0x1410d3110, "unknown_1410d3110", tpl_365::<0x1410d3a00> as *const ()), // sub_1410d3110
    Replacement::draft(0x1410d3120, "unknown_1410d3120", tpl_365::<0x1410d3420> as *const ()), // sub_1410d3120
    Replacement::draft_throws(0x1410d3130, "unknown_1410d3130", fn_1410d3130 as *const (), Throws::Discard), // sub_1410d3130
    Replacement::draft(0x1410d3190, "unknown_1410d3190", fn_1410d3190 as *const ()), // sub_1410d3190
    Replacement::draft_throws(0x1410d32e0, "unknown_1410d32e0", fn_1410d32e0 as *const (), Throws::Caught), // sub_1410d32e0
    Replacement::draft(0x1410d3420, "unknown_1410d3420", tpl_20759::<0x60, 0x40, 0x140235d60, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410d3420
    Replacement::draft(0x1410d3680, "unknown_1410d3680", fn_1410d3680 as *const ()), // sub_1410d3680
    Replacement::draft_throws(0x1410d38c0, "unknown_1410d38c0", fn_1410d38c0 as *const (), Throws::Caught), // sub_1410d38c0
    Replacement::draft(0x1410d3a00, "unknown_1410d3a00", tpl_20759::<0x60, 0x40, 0x140236010, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410d3a00
    Replacement::draft(0x1410d3af0, "unknown_1410d3af0", tpl_6558::<0x140235d60> as *const ()), // sub_1410d3af0
    Replacement::draft(0x1410d3b10, "unknown_1410d3b10", tpl_6558::<0x140236010> as *const ()), // sub_1410d3b10
    Replacement::draft(0x1410d3b30, "unknown_1410d3b30", tpl_170::<0x140235d60> as *const ()), // sub_1410d3b30
    Replacement::draft(0x1410d3b40, "unknown_1410d3b40", fn_1410d3b40 as *const ()), // sub_1410d3b40
    Replacement::draft(0x1410d3d10, "unknown_1410d3d10", tpl_170::<0x140236010> as *const ()), // sub_1410d3d10
    Replacement::draft(0x1410d3e70, "unknown_1410d3e70", fn_1410d3e70 as *const ()), // sub_1410d3e70
    Replacement::draft(0x1410d3ee0, "unknown_1410d3ee0", fn_1410d3ee0 as *const ()), // sub_1410d3ee0
    Replacement::draft(0x1410d4040, "unknown_1410d4040", fn_1410d4040 as *const ()), // sub_1410d4040
    Replacement::draft(0x1410d4130, "unknown_1410d4130", fn_1410d4130 as *const ()), // sub_1410d4130
    Replacement::draft(0x1410d4310, "unknown_1410d4310", fn_1410d4310 as *const ()), // sub_1410d4310
    Replacement::draft_throws(0x1410d45c0, "unknown_1410d45c0", fn_1410d45c0 as *const (), Throws::Caught), // sub_1410d45c0
    Replacement::draft(0x1410d4780, "unknown_1410d4780", fn_1410d4780 as *const ()), // sub_1410d4780
    Replacement::draft_throws(0x1410d4a10, "unknown_1410d4a10", fn_1410d4a10 as *const (), Throws::Caught), // sub_1410d4a10
    Replacement::draft_throws(0x1410d4bd0, "sparsehash_1410d4bd0", fn_1410d4bd0 as *const (), Throws::Caught), // sub_1410d4bd0
    Replacement::draft_throws(0x1410d4d80, "sparsehash_1410d4d80", fn_1410d4d80 as *const (), Throws::Caught), // sub_1410d4d80
    Replacement::draft(0x1410d4f30, "unknown_1410d4f30", fn_1410d4f30 as *const ()), // sub_1410d4f30
    Replacement::draft(0x1410d5100, "unknown_1410d5100", fn_1410d5100 as *const ()), // sub_1410d5100
    Replacement::draft(0x1410d52e0, "unknown_1410d52e0", fn_1410d52e0 as *const ()), // sub_1410d52e0
    Replacement::draft_throws(0x1410d53d0, "unknown_1410d53d0", fn_1410d53d0 as *const (), Throws::Discard), // sub_1410d53d0
    Replacement::draft_throws(0x1410d5470, "coregame_1410d5470", tpl_1173::<0x143cdbe34, 0x143cdbdf8, 0x1411e9f50, 0x1410d68d0, 0x1410d6c60, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143391396, 0x142646960, 0x143cdbe18, 0x143cdbe28, 0x143cdbe2c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::updateGameObjectStreamingManager>
    Replacement::draft(0x1410d56c0, "unknown_1410d56c0", fn_1410d56c0 as *const ()), // sub_1410d56c0
    Replacement::draft(0x1410d5790, "unknown_1410d5790", tpl_4452::<0x8, 0x140236010> as *const ()), // sub_1410d5790
    Replacement::draft(0x1410d57d0, "unknown_1410d57d0", tpl_4452::<0x8, 0x140235d60> as *const ()), // sub_1410d57d0
    Replacement::draft(0x1410d5a00, "unknown_1410d5a00", fn_1410d5a00 as *const ()), // sub_1410d5a00
    Replacement::draft(0x1410d5b20, "unknown_1410d5b20", fn_1410d5b20 as *const ()), // sub_1410d5b20
    Replacement::draft_throws(0x1410d5d40, "sparsehash_1410d5d40", fn_1410d5d40 as *const (), Throws::Caught), // sub_1410d5d40
    Replacement::draft_throws(0x1410d5e50, "sparsehash_1410d5e50", fn_1410d5e50 as *const (), Throws::Caught), // sub_1410d5e50
    Replacement::draft(0x1410d6090, "sparsehash_1410d6090", fn_1410d6090 as *const ()), // sub_1410d6090
    Replacement::draft_throws(0x1410d6210, "sparsehash_1410d6210", fn_1410d6210 as *const (), Throws::Caught), // sub_1410d6210
    Replacement::draft_throws(0x1410d6300, "sparsehash_1410d6300", fn_1410d6300 as *const (), Throws::Caught), // sub_1410d6300
    Replacement::draft(0x1410d6830, "unknown_1410d6830", fn_1410d6830 as *const ()), // sub_1410d6830
    Replacement::draft_throws(0x1410d68d0, "unknown_1410d68d0", fn_1410d68d0 as *const (), Throws::Discard), // sub_1410d68d0
    Replacement::draft(0x1410d6bf0, "unknown_1410d6bf0", fn_1410d6bf0 as *const ()), // sub_1410d6bf0
    Replacement::draft(0x1410d6ca0, "unknown_1410d6ca0", fn_1410d6ca0 as *const ()), // sub_1410d6ca0
    Replacement::draft(0x1410d6e30, "unknown_1410d6e30", fn_1410d6e30 as *const ()), // sub_1410d6e30
    Replacement::draft(0x1410d6fe0, "unknown_1410d6fe0", tpl_381::<0x1410d7140, 0x1410d7380> as *const ()), // sub_1410d6fe0
    Replacement::draft(0x1410d7140, "unknown_1410d7140", fn_1410d7140 as *const ()), // sub_1410d7140
    Replacement::draft(0x1410d7380, "unknown_1410d7380", fn_1410d7380 as *const ()), // sub_1410d7380
    Replacement::draft(0x1410d7620, "std_1410d7620", tpl_1::<0x143890418> as *const ()), // std::_Func_impl_no_alloc<void (*)(ecs::Storage&),void,ecs::Storage&>::std::_Func_impl_no_alloc<void (*)(ecs::Storage&),v
    Replacement::draft(0x1410d7630, "std_1410d7630", fn_1410d7630 as *const ()), // std::_Func_impl_no_alloc<void (*)(ecs::Storage&),void,ecs::Storage&>::std::_Func_impl_no_alloc<void (*)(ecs::Storage&),v
    Replacement::draft(0x1410d7640, "std_1410d7640", tpl_550::<0x143391848, 0x8> as *const ()), // std::_Func_impl_no_alloc<void (*)(ecs::Storage&),void,ecs::Storage&>::std::_Func_impl_no_alloc<void (*)(ecs::Storage&),v
    Replacement::draft(0x1410d7660, "unknown_1410d7660", fn_1410d7660 as *const ()), // sub_1410d7660
    Replacement::draft(0x1410d7830, "unknown_1410d7830", tpl_20791 as *const ()), // sub_1410d7830
    Replacement::draft_throws(0x1410d7840, "unknown_1410d7840", fn_1410d7840 as *const (), Throws::Caught), // sub_1410d7840
    Replacement::draft(0x1410d79c0, "unknown_1410d79c0", fn_1410d79c0 as *const ()), // sub_1410d79c0
    Replacement::draft(0x1410d7c20, "unknown_1410d7c20", tpl_19784::<0x142407c80, 0x143c4aa00, 0x141f5e260, 0x1426455a8, 0x142645598, 0x141f5e240, 0x142645580> as *const ()), // sub_1410d7c20
    Replacement::draft(0x1410d7d70, "unknown_1410d7d70", fn_1410d7d70 as *const ()), // sub_1410d7d70
    Replacement::draft(0x1410d7de0, "unknown_1410d7de0", tpl_20795::<0x8, 0x1410d7d70> as *const ()), // sub_1410d7de0
    Replacement::draft(0x1410d7df0, "unknown_1410d7df0", fn_1410d7df0 as *const ()), // sub_1410d7df0
    Replacement::draft(0x1410d7e10, "unknown_1410d7e10", fn_1410d7e10 as *const ()), // sub_1410d7e10
    Replacement::draft(0x1410d7e80, "unknown_1410d7e80", tpl_2030::<0x1410d7e10, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410d7e80
    Replacement::draft(0x1410d7f30, "coregame_1410d7f30", fn_1410d7f30 as *const ()), // coregame::GameConsole::coregame::GameConsole::vfunc_1
    Replacement::draft(0x1410d8000, "unknown_1410d8000", fn_1410d8000 as *const ()), // sub_1410d8000
    Replacement::draft(0x1410d8080, "unknown_1410d8080", fn_1410d8080 as *const ()), // sub_1410d8080
    Replacement::draft(0x1410d80d0, "unknown_1410d80d0", fn_1410d80d0 as *const ()), // sub_1410d80d0
    Replacement::draft(0x1410d8130, "unknown_1410d8130", fn_1410d8130 as *const ()), // sub_1410d8130
    Replacement::draft_throws(0x1410d8220, "unknown_1410d8220", fn_1410d8220 as *const (), Throws::Caught), // sub_1410d8220
    Replacement::draft_throws(0x1410d84b0, "anonymous_namespace_1410d84b0", tpl_6039::<0x143cdc010, 0x143cdbe78, 0x1411e9f50, 0x1410dba10, 0x14384eabc, 0x1420af120, 0x1410da920, 0x1420a28e0, 0x1410daa20, 0x1410d9e60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433932e6, 0x142646960, 0x143cdbe98, 0x143cdbe9c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::initFreeCamera>
    Replacement::draft_throws(0x1410d87b0, "coregame_1410d87b0", fn_1410d87b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::freecamera::freeCameraRemoved>
    Replacement::draft_throws(0x1410d8a70, "coregame_1410d8a70", fn_1410d8a70 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::freecamera::update>
    Replacement::draft_throws(0x1410d8cc0, "std_1410d8cc0", fn_1410d8cc0 as *const (), Throws::Discard), // sub_1410d8cc0
    Replacement::draft(0x1410d8f60, "unknown_1410d8f60", tpl_1095::<0x14384e98c, 0x14020e8f0> as *const ()), // sub_1410d8f60
    Replacement::draft(0x1410d9140, "unknown_1410d9140", tpl_3472::<0x143c998ac, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410d9140
    Replacement::draft(0x1410d94a0, "unknown_1410d94a0", tpl_1466::<0x14384e988, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410d94a0
    Replacement::draft(0x1410d9770, "unknown_1410d9770", fn_1410d9770 as *const ()), // sub_1410d9770
    Replacement::draft(0x1410d9ae0, "unknown_1410d9ae0", tpl_1435::<0x143c97e40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410d9ae0
    Replacement::draft(0x1410d9e60, "unknown_1410d9e60", tpl_64030::<0x88, 0x10088, 0x141f62490, 0x141f62d60> as *const ()), // sub_1410d9e60
    Replacement::draft_throws(0x1410d9f80, "unknown_1410d9f80", fn_1410d9f80 as *const (), Throws::Discard), // sub_1410d9f80
    Replacement::draft_throws(0x1410da290, "unknown_1410da290", fn_1410da290 as *const (), Throws::Discard), // sub_1410da290
    Replacement::draft(0x1410da370, "unknown_1410da370", tpl_5745::<0x143c997a0, 0x143c8cef8, 0x8, 0x580e0, 0x20, 0x143c94b44, 0x143c8d08c, 0x143cdc088, 0x143c904a0, 0x143c8d910, 0x143c900d8, 0x143c8f63c> as *const ()), // sub_1410da370
    Replacement::draft_throws(0x1410da490, "unknown_1410da490", fn_1410da490 as *const (), Throws::Discard), // sub_1410da490
    Replacement::draft(0x1410da610, "unknown_1410da610", fn_1410da610 as *const ()), // sub_1410da610
    Replacement::draft_throws(0x1410da840, "unknown_1410da840", tpl_1498::<0x143c8c464, 0x143c8c46c, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8cef8, 0x143c8c468, 0x143c8c470, 0x141834a0c> as *const (), Throws::Discard), // sub_1410da840
    Replacement::draft_throws(0x1410da920, "unknown_1410da920", fn_1410da920 as *const (), Throws::Discard), // sub_1410da920
    Replacement::draft(0x1410daa20, "unknown_1410daa20", tpl_64033::<0x143c8d120, 0x142489ee0, 0x143c97e40, 0x143c8cf5c, 0x141f62d60> as *const ()), // sub_1410daa20
    Replacement::draft(0x1410dac40, "unknown_1410dac40", tpl_6144::<0x143c998ac, 0x5e308, 0x10, 0x8, 0x18, 0x141f62b20, 0x140489dd0> as *const ()), // sub_1410dac40
    Replacement::draft_throws(0x1410dace0, "unknown_1410dace0", fn_1410dace0 as *const (), Throws::Discard), // sub_1410dace0
    Replacement::draft(0x1410dafd0, "unknown_1410dafd0", tpl_3188::<0x1410db970, 0x143391ce0, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410dafd0
    Replacement::draft(0x1410db1e0, "unknown_1410db1e0", fn_1410db1e0 as *const ()), // sub_1410db1e0
    Replacement::draft(0x1410db720, "unknown_1410db720", fn_1410db720 as *const ()), // sub_1410db720
    Replacement::draft(0x1410db970, "unknown_1410db970", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143391ce0, 0x8, 0x20> as *const ()), // sub_1410db970
    Replacement::draft(0x1410dba10, "unknown_1410dba10", tpl_20815::<0x143cdbf38, 0x143cdc018, 0x8, 0x10, 0x18, 0x141834a78, 0x14384e98c, 0x143c8d120, 0x143c97e40, 0x143c8cf5c, 0x143cdc038, 0x143cdc048, 0x141834a0c> as *const ()), // sub_1410dba10
    Replacement::draft_throws(0x1410dbb50, "unknown_1410dbb50", fn_1410dbb50 as *const (), Throws::Discard), // sub_1410dbb50
    Replacement::draft(0x1410dbd30, "unknown_1410dbd30", fn_1410dbd30 as *const ()), // sub_1410dbd30
    Replacement::draft_throws(0x1410dbe90, "unknown_1410dbe90", tpl_4541::<0x143cdc088, 0x1410db5e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1410dbe90
    Replacement::draft_throws(0x1410dbf20, "unknown_1410dbf20", tpl_4541::<0x14384eab8, 0x1410db720, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1410dbf20
    Replacement::draft_throws(0x1410dbfb0, "unknown_1410dbfb0", tpl_4541::<0x14384eabc, 0x1410dba10, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1410dbfb0
    Replacement::draft(0x1410dc040, "unknown_1410dc040", fn_1410dc040 as *const ()), // sub_1410dc040
    Replacement::draft_throws(0x1410dc2c0, "std_1410dc2c0", fn_1410dc2c0 as *const (), Throws::Discard), // sub_1410dc2c0
    Replacement::draft(0x1410dc560, "unknown_1410dc560", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143cb87f8, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1410dc560
    Replacement::draft(0x1410dc650, "unknown_1410dc650", tpl_63182::<0x143cdc2cc, 0x1401e7c10, 0x143c8d120, 0x142489ee0, 0x143c8f6d8> as *const ()), // sub_1410dc650
    Replacement::draft_throws(0x1410dc840, "coregame_1410dc840", tpl_4484::<0x143cdc250, 0x143cdc1e0, 0x1411e9f50, 0x1410dce20, 0x1410dd210, 0x1410dd0c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143393986, 0x142646960, 0x143cdc1f0, 0x143cdc1f8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fpscamera::update>
    Replacement::draft(0x1410dca90, "unknown_1410dca90", tpl_1403::<0x143cdc2c8, 0x1410dcca0, 0x14020e8f0> as *const ()), // sub_1410dca90
    Replacement::draft(0x1410dcca0, "unknown_1410dcca0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143298af8> as *const ()), // sub_1410dcca0
    Replacement::draft_throws(0x1410dce20, "unknown_1410dce20", fn_1410dce20 as *const (), Throws::Discard), // sub_1410dce20
    Replacement::draft(0x1410dd0c0, "unknown_1410dd0c0", fn_1410dd0c0 as *const ()), // sub_1410dd0c0
    Replacement::draft(0x1410dd210, "unknown_1410dd210", fn_1410dd210 as *const ()), // sub_1410dd210
    Replacement::draft_throws(0x1410dd4d0, "unknown_1410dd4d0", tpl_4017::<0x143cdc2cc, 0x143cdc2f8, 0x143cdc258, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d120, 0x143c8f6d8, 0x143c8d0d0, 0x143cdc278, 0x141834a0c> as *const (), Throws::Discard), // sub_1410dd4d0
    Replacement::draft_throws(0x1410dd900, "unknown_1410dd900", fn_1410dd900 as *const (), Throws::Caught), // sub_1410dd900
    Replacement::draft_throws(0x1410ddaa0, "unknown_1410ddaa0", fn_1410ddaa0 as *const (), Throws::Discard), // sub_1410ddaa0
    Replacement::draft_throws(0x1410ddb10, "unknown_1410ddb10", fn_1410ddb10 as *const (), Throws::Discard), // sub_1410ddb10
    Replacement::draft_throws(0x1410ddc60, "unknown_1410ddc60", fn_1410ddc60 as *const (), Throws::Discard), // sub_1410ddc60
    Replacement::draft(0x1410dddc0, "unknown_1410dddc0", fn_1410dddc0 as *const ()), // sub_1410dddc0
    Replacement::draft(0x1410de0f0, "unknown_1410de0f0", fn_1410de0f0 as *const ()), // sub_1410de0f0
    Replacement::draft(0x1410de300, "unknown_1410de300", fn_1410de300 as *const ()), // sub_1410de300
    Replacement::draft(0x1410de4d0, "unknown_1410de4d0", fn_1410de4d0 as *const ()), // sub_1410de4d0
    Replacement::draft(0x1410de640, "unknown_1410de640", fn_1410de640 as *const ()), // sub_1410de640
    Replacement::draft_throws(0x1410de7b0, "unknown_1410de7b0", tpl_1480::<0x142913e00, 0x1410d6ca0, 0x1410e0c30, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410de7b0
    Replacement::draft_throws(0x1410dea50, "unknown_1410dea50", tpl_1480::<0x142913e00, 0x1410d6ca0, 0x1410e0f10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410dea50
    Replacement::draft(0x1410decf0, "unknown_1410decf0", tpl_1526::<0x142913e00, 0x1410d6ca0, 0x1410e1e10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1410decf0
    Replacement::draft(0x1410def90, "unknown_1410def90", tpl_1526::<0x142913e00, 0x1410d6ca0, 0x1410e20b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1410def90
    Replacement::draft_throws(0x1410df230, "coregame_1410df230", tpl_6484::<0x143cdc470, 0x143cdc6f8, 0x1411e9f50, 0x1410e47a0, 0x143cdc6b8, 0x1420af120, 0x140215750, 0x1410933c0, 0x1420a28e0, 0x1410e31f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143395596, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::updateFogVolumes>
    Replacement::draft_throws(0x1410df530, "coregame_1410df530", fn_1410df530 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::updateFogBlockers>
    Replacement::draft(0x1410df840, "coregame_1410df840", fn_1410df840 as *const ()), // sub_1410df840
    Replacement::draft(0x1410dfc20, "coregame_1410dfc20", fn_1410dfc20 as *const ()), // sub_1410dfc20
    Replacement::draft(0x1410dffb0, "coregame_1410dffb0", tpl_1094::<0x143cdca98, 0x1410e25b0, 0x14020e8f0> as *const ()), // sub_1410dffb0
    Replacement::draft(0x1410e01b0, "coregame_1410e01b0", tpl_1094::<0x143cdc71c, 0x1410e2780, 0x14020e8f0> as *const ()), // sub_1410e01b0
    Replacement::draft(0x1410e03b0, "coregame_1410e03b0", fn_1410e03b0 as *const ()), // sub_1410e03b0
    Replacement::draft_throws(0x1410e05c0, "coregame_1410e05c0", tpl_19691::<0x143cdc474, 0x143cdc370, 0x1410e2910, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143395846, 0x142646960, 0x143cdc390, 0x143cdc3b0, 0x143cdc3c0, 0x143cdc3c4, 0x143cdc3c6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::FogVolumeComponent>
    Replacement::draft(0x1410e0800, "coregame_1410e0800", tpl_20349::<0x1401e7c10, 0x1410e23b0, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1410e0800
    Replacement::draft_throws(0x1410e0a10, "coregame_1410e0a10", tpl_20411::<0x143cdc3c8, 0x143cdc660, 0x1410e2ad0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143395a26, 0x142646960, 0x143cdc680, 0x143cdc6a0, 0x143cdc6b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::FogBlockerComponent
    Replacement::draft_throws(0x1410e0c30, "coregame_1410e0c30", tpl_5126::<0x143cdc9d8, 0x143cdc818, 0x1411e9f50, 0x1410e5820, 0x143cdc6c4, 0x1420af120, 0x1420a28e0, 0x1410e3bd0, 0x1410e2c90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143394d16, 0x142646960, 0x143cdc828, 0x143cdc830, 0x143cdc834, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::hideFogVolume>
    Replacement::draft_throws(0x1410e0f10, "coregame_1410e0f10", tpl_1963::<0x143cdc518, 0x143cdc7e8, 0x1411e9f50, 0x1410e5710, 0x143cdc810, 0x1420af120, 0x1420a28e0, 0x1410e3de0, 0x1410e2db0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143394ee6, 0x142646960, 0x143cdc7f8, 0x143cdc800, 0x143cdc804, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::hideFogBlocker>
    Replacement::draft_throws(0x1410e11f0, "coregame_1410e11f0", fn_1410e11f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::streamInFogVolume>
    Replacement::draft_throws(0x1410e1510, "coregame_1410e1510", fn_1410e1510 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::streamInFogBlocker>
    Replacement::draft_throws(0x1410e1830, "coregame_1410e1830", tpl_4828::<0x143cdcac4, 0x143cdc5d0, 0x1411e9f50, 0x1410e53a0, 0x143cdca9c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410e4390, 0x1410e30d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433951f6, 0x142646960, 0x143cdc5f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::streamOutFogVolume>
    Replacement::draft_throws(0x1410e1b20, "coregame_1410e1b20", tpl_19902::<0x143cdc864, 0x143cdc6d0, 0x1411e9f50, 0x1410e52b0, 0x143cdc51c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410e4480, 0x1410e3160, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433953c6, 0x142646960, 0x143cdc6f0, 0x143cdc6f2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::fog::streamOutFogBlocker>
    Replacement::draft(0x1410e1e10, "coregame_1410e1e10", fn_1410e1e10 as *const ()), // ecs::system::ecs::system::wrapper<coregame::fog::removeFogVolumes>
    Replacement::draft(0x1410e20b0, "coregame_1410e20b0", tpl_20837::<0x143cdc6c8, 0x143cdc840, 0x1411e9f50, 0x1410e48b0, 0x143cdc620, 0x1401fc320, 0x1420a28e0, 0x1410e4600, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143395156, 0x142646960, 0x143cdc860, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::fog::removeFogBlockers>
    Replacement::draft(0x1410e25b0, "unknown_1410e25b0", fn_1410e25b0 as *const ()), // sub_1410e25b0
    Replacement::draft(0x1410e2780, "unknown_1410e2780", tpl_6197::<0x1401e6d50, 0x140285d50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143297570> as *const ()), // sub_1410e2780
    Replacement::draft_throws(0x1410e2910, "unknown_1410e2910", tpl_3011::<0x1411e9f50, 0x1410e5a20, 0x143cdcb78, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410e3af0, 0x1401de410> as *const (), Throws::Discard), // sub_1410e2910
    Replacement::draft_throws(0x1410e2ad0, "unknown_1410e2ad0", tpl_3011::<0x1411e9f50, 0x1410e5930, 0x143cdc520, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410e3b60, 0x1401de410> as *const (), Throws::Discard), // sub_1410e2ad0
    Replacement::draft(0x1410e2c90, "unknown_1410e2c90", tpl_62730::<0x88, 0x10088, 0x1410e37c0, 0x141f6ac90> as *const ()), // sub_1410e2c90
    Replacement::draft(0x1410e2db0, "unknown_1410e2db0", tpl_62730::<0x88, 0x10088, 0x1410e3690, 0x141f6abf0> as *const ()), // sub_1410e2db0
    Replacement::draft(0x1410e2ed0, "unknown_1410e2ed0", tpl_79166::<0x88, 0x10088, 0x143cdc8e8, 0x580e0, 0x1410e3590, 0x141f6b7e0> as *const ()), // sub_1410e2ed0
    Replacement::draft(0x1410e2fd0, "unknown_1410e2fd0", tpl_79166::<0x88, 0x10088, 0x143cdc92c, 0x580e0, 0x1410e3490, 0x141f6b580> as *const ()), // sub_1410e2fd0
    Replacement::draft(0x1410e30d0, "unknown_1410e30d0", tpl_1898::<0x88, 0x10088, 0x141f6b450> as *const ()), // sub_1410e30d0
    Replacement::draft(0x1410e3160, "unknown_1410e3160", tpl_1898::<0x88, 0x10088, 0x141f6b320> as *const ()), // sub_1410e3160
    Replacement::draft(0x1410e31f0, "unknown_1410e31f0", tpl_1217::<0x143cdc6b8, 0x20, 0x8, 0x141f6a370> as *const ()), // sub_1410e31f0
    Replacement::draft(0x1410e3250, "unknown_1410e3250", tpl_1217::<0x143cdc95c, 0x20, 0x8, 0x141f69b10> as *const ()), // sub_1410e3250
    Replacement::draft(0x1410e3af0, "unknown_1410e3af0", tpl_3028::<0x143cdcb78, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410e03b0> as *const ()), // sub_1410e3af0
    Replacement::draft(0x1410e3b60, "unknown_1410e3b60", tpl_3028::<0x143cdc520, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410e0800> as *const ()), // sub_1410e3b60
    Replacement::draft(0x1410e3bd0, "unknown_1410e3bd0", tpl_62741::<0x143c8d87c, 0x142489ee0, 0x143c8d120, 0x143cdc650, 0x141f6ac90> as *const ()), // sub_1410e3bd0
    Replacement::draft(0x1410e3de0, "unknown_1410e3de0", tpl_62741::<0x143c8d87c, 0x142489ee0, 0x143c8d120, 0x143cdc620, 0x141f6abf0> as *const ()), // sub_1410e3de0
    Replacement::draft(0x1410e3ff0, "unknown_1410e3ff0", tpl_79167::<0x143cdc8e8, 0x143cdca98, 0x142489ee0, 0x143c8d120, 0x141f6b7e0> as *const ()), // sub_1410e3ff0
    Replacement::draft(0x1410e41c0, "unknown_1410e41c0", tpl_79167::<0x143cdc92c, 0x143cdc71c, 0x142489ee0, 0x143c8d120, 0x141f6b580> as *const ()), // sub_1410e41c0
    Replacement::draft(0x1410e4390, "unknown_1410e4390", tpl_61552::<0x8, 0x88, 0x10088, 0x141f6b450> as *const ()), // sub_1410e4390
    Replacement::draft(0x1410e4480, "unknown_1410e4480", tpl_61552::<0x8, 0x88, 0x10088, 0x141f6b320> as *const ()), // sub_1410e4480
    Replacement::draft(0x1410e4570, "unknown_1410e4570", tpl_19695::<0x143cdc650, 0x5e308, 0x10, 0x8, 0x18, 0x141f6b020, 0x140489dd0> as *const ()), // sub_1410e4570
    Replacement::draft(0x1410e4600, "unknown_1410e4600", tpl_19695::<0x143cdc620, 0x5e308, 0x10, 0x8, 0x18, 0x141f6ad30, 0x140489dd0> as *const ()), // sub_1410e4600
    Replacement::draft(0x1410e48b0, "unknown_1410e48b0", tpl_3188::<0x1410e5170, 0x143394038, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410e48b0
    Replacement::draft(0x1410e4930, "unknown_1410e4930", tpl_3188::<0x1410e5210, 0x143394060, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410e4930
    Replacement::draft(0x1410e4e80, "unknown_1410e4e80", fn_1410e4e80 as *const ()), // sub_1410e4e80
    Replacement::draft(0x1410e4fc0, "unknown_1410e4fc0", fn_1410e4fc0 as *const ()), // sub_1410e4fc0
    Replacement::draft(0x1410e5170, "unknown_1410e5170", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143394038, 0x8, 0x20> as *const ()), // sub_1410e5170
    Replacement::draft(0x1410e5210, "unknown_1410e5210", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143394060, 0x8, 0x20> as *const ()), // sub_1410e5210
    Replacement::draft_throws(0x1410e60b0, "unknown_1410e60b0", fn_1410e60b0 as *const (), Throws::Discard), // sub_1410e60b0
    Replacement::draft(0x1410e60f0, "coregame_1410e60f0", fn_1410e60f0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::flushFactoryOutputs>
    Replacement::draft(0x1410e6360, "coregame_1410e6360", tpl_20844::<0x143cdcc40, 0x143cdcbf8, 0x1411e9f50, 0x1420a28e0, 0x1410e6a70, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143396ce6, 0x142646960, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::flushEndOfFixedUpdate>
    Replacement::draft(0x1410e65b0, "coregame_1410e65b0", tpl_20844::<0x143cdcc3c, 0x143cdcc48, 0x1411e9f50, 0x1420a28e0, 0x1410e6a70, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143396c66, 0x142646960, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::flushStreamOutOutputs>
    Replacement::draft(0x1410e6800, "coregame_1410e6800", fn_1410e6800 as *const ()), // ecs::system::ecs::system::wrapper<coregame::flushStreamInOutputs>
    Replacement::draft_throws(0x1410e6a70, "coregame_1410e6a70", fn_1410e6a70 as *const (), Throws::Discard), // sub_1410e6a70
    Replacement::draft(0x1410e6aa0, "coregame_1410e6aa0", fn_1410e6aa0 as *const ()), // sub_1410e6aa0
    Replacement::draft(0x1410e6af0, "coregame_1410e6af0", fn_1410e6af0 as *const ()), // sub_1410e6af0
    Replacement::draft(0x1410e6c60, "coregame_1410e6c60", fn_1410e6c60 as *const ()), // sub_1410e6c60
    Replacement::draft(0x1410e6dd0, "coregame_1410e6dd0", fn_1410e6dd0 as *const ()), // sub_1410e6dd0
    Replacement::draft(0x1410e6e70, "coregame_1410e6e70", fn_1410e6e70 as *const ()), // sub_1410e6e70
    Replacement::draft(0x1410e6fe0, "coregame_1410e6fe0", fn_1410e6fe0 as *const ()), // sub_1410e6fe0
    Replacement::draft(0x1410e7150, "coregame_1410e7150", fn_1410e7150 as *const ()), // sub_1410e7150
    Replacement::draft(0x1410e72d0, "coregame_1410e72d0", fn_1410e72d0 as *const ()), // sub_1410e72d0
    Replacement::draft(0x1410e7440, "coregame_1410e7440", fn_1410e7440 as *const ()), // sub_1410e7440
    Replacement::draft(0x1410e75c0, "coregame_1410e75c0", fn_1410e75c0 as *const ()), // sub_1410e75c0
    Replacement::draft_throws(0x1410e7750, "coregame_1410e7750", tpl_1963::<0x143cdceac, 0x143cdceb0, 0x1411e9f50, 0x1410ec150, 0x143cdcd9c, 0x1420af120, 0x1420a28e0, 0x1410eae00, 0x1410ea2c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143397b76, 0x142646960, 0x143cdcec0, 0x143cdcec8, 0x143cdcecc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::flare::copyPrevious>
    Replacement::draft_throws(0x1410e7a30, "coregame_1410e7a30", tpl_1075::<0x143cdcea8, 0x143cdcdb0, 0x1411e9f50, 0x1410ebf40, 0x143cdcfac, 0x1420af120, 0x143c900d8, 0x140215610, 0x1420a28e0, 0x1410eafa0, 0x1410ea3a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433980e6, 0x142646960, 0x143cdcdd0, 0x143cdcdd2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::flare::updateToRendering>
    Replacement::draft_throws(0x1410e7d40, "coregame_1410e7d40", tpl_5818::<0x143cdd0f4, 0x143cdd070, 0x1411e9f50, 0x1410ebe30, 0x143cdcfa8, 0x1420af120, 0x143c904a0, 0x140215610, 0x143c8d904, 0x1420a28e0, 0x1410eb200, 0x1410ea580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143397eb6, 0x142646960, 0x143cdd090, 0x143cdd094, 0x143cdd096, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::flare::updateParameterBlends>
    Replacement::draft(0x1410e8090, "coregame_1410e8090", tpl_1526::<0x142915060, 0x1410d6ca0, 0x1410e9ba0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1410e8090
    Replacement::draft(0x1410e8330, "coregame_1410e8330", tpl_1094::<0x143cdcdd8, 0x1410e9f80, 0x14020e8f0> as *const ()), // sub_1410e8330
    Replacement::draft(0x1410e8530, "coregame_1410e8530", tpl_1402::<0x143cd65b4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410e8530
    Replacement::draft(0x1410e8880, "coregame_1410e8880", tpl_1402::<0x143cd65d4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410e8880
    Replacement::draft(0x1410e8bd0, "coregame_1410e8bd0", tpl_1094::<0x143cdcf30, 0x1410ea110, 0x14020e8f0> as *const ()), // sub_1410e8bd0
    Replacement::draft(0x1410e8dd0, "coregame_1410e8dd0", tpl_5015::<0x143c9d1e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410e8dd0
    Replacement::draft_throws(0x1410e9140, "coregame_1410e9140", tpl_19638::<0x143cdcfb0, 0x143cdd0f8, 0x1411e9f50, 0x1410ec9f0, 0x143cdcff0, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1410eb9e0, 0x1410eb480, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143397ce6, 0x142646960, 0x143cdd108, 0x143cdd110, 0x143cdd112, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::flare::streamOut>
    Replacement::draft_throws(0x1410e9440, "coregame_1410e9440", tpl_19663::<0x143cdcd3c, 0x143cdcd80, 0x1411e9f50, 0x1410ec8b0, 0x143cdcda0, 0x1420af120, 0x14027c6a0, 0x143cada18, 0x140215610, 0x1420a28e0, 0x1410ebad0, 0x1410eb510, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433970f6, 0x142646960, 0x143cdcd90, 0x143cdcd98, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::flare::streamIn>
    Replacement::draft(0x1410e9760, "coregame_1410e9760", tpl_19366::<0x1401e7c10, 0x1410e9e80, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1410e9760
    Replacement::draft_throws(0x1410e9970, "coregame_1410e9970", tpl_19662::<0x143cdd114, 0x143cdd0a0, 0x1410ea6e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143396f16, 0x142646960, 0x143cdd0c0, 0x143cdd0e0, 0x143cdd0f0, 0x143cdd0f2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::FlareComponent>>
    Replacement::draft(0x1410e9ba0, "coregame_1410e9ba0", fn_1410e9ba0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::flare::removed>
    Replacement::draft(0x1410e9f80, "unknown_1410e9f80", fn_1410e9f80 as *const ()), // sub_1410e9f80
    Replacement::draft(0x1410ea110, "unknown_1410ea110", fn_1410ea110 as *const ()), // sub_1410ea110
    Replacement::draft(0x1410ea2c0, "unknown_1410ea2c0", tpl_62755::<0x88, 0x10088, 0x1410eacd0, 0x141f6e360> as *const ()), // sub_1410ea2c0
    Replacement::draft(0x1410ea3a0, "unknown_1410ea3a0", fn_1410ea3a0 as *const ()), // sub_1410ea3a0
    Replacement::draft(0x1410ea460, "unknown_1410ea460", tpl_19668::<0x18, 0x20, 0x40, 0x60, 0x8, 0x141f6d770> as *const ()), // sub_1410ea460
    Replacement::draft(0x1410ea580, "unknown_1410ea580", tpl_62821::<0x143c8d904, 0x143c904a0, 0x1410ea8a0, 0x141f6de10> as *const ()), // sub_1410ea580
    Replacement::draft_throws(0x1410ea6e0, "unknown_1410ea6e0", tpl_3011::<0x1411e9f50, 0x1410ec7c0, 0x143cdce1c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1410ebd30, 0x1401de410> as *const (), Throws::Discard), // sub_1410ea6e0
    Replacement::draft(0x1410eadd0, "unknown_1410eadd0", tpl_1955::<0x18, 0x1410eb650> as *const ()), // sub_1410eadd0
    Replacement::draft(0x1410eae00, "unknown_1410eae00", tpl_62763::<0x143cd65b4, 0x142489ee0, 0x143cd65d4, 0x141f6e360> as *const ()), // sub_1410eae00
    Replacement::draft(0x1410eafa0, "unknown_1410eafa0", fn_1410eafa0 as *const ()), // sub_1410eafa0
    Replacement::draft(0x1410eb200, "unknown_1410eb200", tpl_62838::<0x143c8d904, 0x143c904a0, 0x143c9d1e0, 0x142489ee0, 0x143cd65b4, 0x143cd65d4, 0x141f6de10> as *const ()), // sub_1410eb200
    Replacement::draft(0x1410eb480, "unknown_1410eb480", tpl_1898::<0x88, 0x10088, 0x141f6dfb0> as *const ()), // sub_1410eb480
    Replacement::draft_throws(0x1410eb510, "unknown_1410eb510", tpl_65568::<0x88, 0x10088, 0x143cada18, 0x580e0, 0x1410eb6b0, 0x141f6e0e0> as *const (), Throws::Discard), // sub_1410eb510
    Replacement::draft(0x1410eb650, "unknown_1410eb650", fn_1410eb650 as *const ()), // sub_1410eb650
    Replacement::draft(0x1410eb9e0, "unknown_1410eb9e0", tpl_61552::<0x8, 0x88, 0x10088, 0x141f6dfb0> as *const ()), // sub_1410eb9e0
    Replacement::draft_throws(0x1410ebad0, "unknown_1410ebad0", tpl_65573::<0x143cada18, 0x143cdcf30, 0x142489ee0, 0x143c8d160, 0x143cd65c4, 0x141f6e0e0> as *const (), Throws::Discard), // sub_1410ebad0
    Replacement::draft(0x1410ebd30, "unknown_1410ebd30", tpl_3028::<0x143cdce1c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410e9760> as *const ()), // sub_1410ebd30
    Replacement::draft(0x1410ebda0, "unknown_1410ebda0", tpl_19695::<0x143cdcdd8, 0x5e308, 0x10, 0x8, 0x18, 0x141f6def0, 0x1410ec5d0> as *const ()), // sub_1410ebda0
    Replacement::draft(0x1410ebf40, "unknown_1410ebf40", fn_1410ebf40 as *const ()), // sub_1410ebf40
    Replacement::draft(0x1410ec240, "unknown_1410ec240", tpl_3188::<0x1410ec720, 0x143398a48, 0x1410ec5d0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410ec240
    Replacement::draft(0x1410ec2c0, "unknown_1410ec2c0", tpl_19620::<0x8, 0x1410ec5d0> as *const ()), // sub_1410ec2c0
    Replacement::draft(0x1410ec470, "unknown_1410ec470", fn_1410ec470 as *const ()), // sub_1410ec470
    Replacement::draft(0x1410ec5d0, "unknown_1410ec5d0", fn_1410ec5d0 as *const ()), // sub_1410ec5d0
    Replacement::draft(0x1410ec720, "unknown_1410ec720", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143398a48, 0x8, 0x20> as *const ()), // sub_1410ec720
    Replacement::draft(0x1410ece40, "coregame_1410ece40", fn_1410ece40 as *const ()), // coregame::FilePrecache::coregame::FilePrecache::vfunc_3
    Replacement::draft(0x1410eceb0, "coregame_1410eceb0", fn_1410eceb0 as *const ()), // sub_1410eceb0
    Replacement::draft(0x1410ecf60, "coregame_1410ecf60", fn_1410ecf60 as *const ()), // sub_1410ecf60
    Replacement::draft(0x1410ecfe0, "coregame_1410ecfe0", fn_1410ecfe0 as *const ()), // sub_1410ecfe0
    Replacement::draft(0x1410ed230, "coregame_1410ed230", fn_1410ed230 as *const ()), // sub_1410ed230
    Replacement::draft(0x1410ed2b0, "coregame_1410ed2b0", fn_1410ed2b0 as *const ()), // sub_1410ed2b0
    Replacement::draft(0x1410ed390, "coregame_1410ed390", fn_1410ed390 as *const ()), // sub_1410ed390
    Replacement::draft(0x1410ed660, "coregame_1410ed660", fn_1410ed660 as *const ()), // sub_1410ed660
    Replacement::draft(0x1410ed710, "coregame_1410ed710", fn_1410ed710 as *const ()), // sub_1410ed710
    Replacement::draft(0x1410ed960, "coregame_1410ed960", fn_1410ed960 as *const ()), // sub_1410ed960
    Replacement::draft(0x1410eda00, "coregame_1410eda00", fn_1410eda00 as *const ()), // sub_1410eda00
    Replacement::draft(0x1410edd10, "coregame_1410edd10", fn_1410edd10 as *const ()), // sub_1410edd10
    Replacement::draft(0x1410edec0, "coregame_1410edec0", fn_1410edec0 as *const ()), // sub_1410edec0
    Replacement::draft(0x1410ee4a0, "coregame_1410ee4a0", tpl_20001::<0x120, 0x138, 0x130, 0x128, 0x10, 0x14240ab10> as *const ()), // sub_1410ee4a0
    Replacement::draft(0x1410ee4e0, "coregame_1410ee4e0", fn_1410ee4e0 as *const ()), // sub_1410ee4e0
    Replacement::draft(0x1410ee750, "coregame_1410ee750", fn_1410ee750 as *const ()), // sub_1410ee750
    Replacement::draft(0x1410ee920, "coregame_1410ee920", fn_1410ee920 as *const ()), // sub_1410ee920
    Replacement::draft(0x1410eeb30, "coregame_1410eeb30", fn_1410eeb30 as *const ()), // sub_1410eeb30
    Replacement::draft(0x1410eecd0, "coregame_1410eecd0", fn_1410eecd0 as *const ()), // sub_1410eecd0
    Replacement::draft_throws(0x1410eedec, "coregame_1410eedec", fn_1410eedec as *const (), Throws::Discard), // coregame::FilePrecache::coregame::FilePrecache::vfunc_0
    Replacement::draft(0x1410eee00, "unknown_1410eee00", tpl_3077::<0x143c8f0d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410eee00
    Replacement::draft(0x1410ef130, "unknown_1410ef130", tpl_20883::<0x8, 0x1410eee00> as *const ()), // sub_1410ef130
    Replacement::draft_throws(0x1410ef190, "unknown_1410ef190", fn_1410ef190 as *const (), Throws::Discard), // sub_1410ef190
    Replacement::draft_throws(0x1410ef200, "unknown_1410ef200", fn_1410ef200 as *const (), Throws::Discard), // sub_1410ef200
    Replacement::draft_throws(0x1410ef2c0, "anonymous_namespace_1410ef2c0", tpl_4193::<0x143cdd254, 0x143cdd180, 0x1411e9f50, 0x1410ef510, 0x1410ef680, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143398e76, 0x142646960, 0x143cdd1a0, 0x143cdd1b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::garbageCollectFactDictionary>
    Replacement::draft_throws(0x1410ef510, "unknown_1410ef510", tpl_5266::<0x143c8ce88, 0x140215610, 0x143cd6bac, 0x1410ef9b0, 0x143cdd1b8, 0x1420af120, 0x140215750, 0x1420a28e0> as *const (), Throws::Discard), // sub_1410ef510
    Replacement::draft_throws(0x1410ef680, "unknown_1410ef680", tpl_1804::<0x143cdd1b8, 0x580e0, 0x20, 0x8, 0x143cd6bac, 0x143c8ce88, 0x141f71fc0> as *const (), Throws::Discard), // sub_1410ef680
    Replacement::draft_throws(0x1410ef700, "unknown_1410ef700", fn_1410ef700 as *const (), Throws::Caught), // sub_1410ef700
    Replacement::draft_throws(0x1410efb30, "coregame_1410efb30", tpl_19806::<0x143cdd2e0, 0x143cdd2b8, 0x1411e9f50, 0x1410f0140, 0x143cdd260, 0x1420af120, 0x140f2d980, 0x1420a28e0, 0x1410f0070, 0x1410f0000, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143399076, 0x142646960, 0x143cdd2d8, 0x143cdd2dc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::episodes::setActiveEpisode>
    Replacement::draft(0x1410efe20, "unknown_1410efe20", tpl_1095::<0x143cc5598, 0x14020e8f0> as *const ()), // sub_1410efe20
    Replacement::draft(0x1410f0000, "unknown_1410f0000", fn_1410f0000 as *const ()), // sub_1410f0000
    Replacement::draft(0x1410f0070, "unknown_1410f0070", fn_1410f0070 as *const ()), // sub_1410f0070
    Replacement::draft_throws(0x1410f0280, "unknown_1410f0280", fn_1410f0280 as *const (), Throws::Caught), // sub_1410f0280
    Replacement::draft(0x1410f0390, "unknown_1410f0390", fn_1410f0390 as *const ()), // sub_1410f0390
    Replacement::draft(0x1410f0400, "unknown_1410f0400", fn_1410f0400 as *const ()), // sub_1410f0400
    Replacement::draft_throws(0x1410f05a0, "unknown_1410f05a0", fn_1410f05a0 as *const (), Throws::Caught), // sub_1410f05a0
    Replacement::draft_throws(0x1410f0750, "unknown_1410f0750", fn_1410f0750 as *const (), Throws::Discard), // sub_1410f0750
    Replacement::draft_throws(0x1410f0810, "unknown_1410f0810", fn_1410f0810 as *const (), Throws::Discard), // sub_1410f0810
    Replacement::draft(0x1410f08c0, "unknown_1410f08c0", fn_1410f08c0 as *const ()), // sub_1410f08c0
    Replacement::draft(0x1410f0b70, "unknown_1410f0b70", fn_1410f0b70 as *const ()), // sub_1410f0b70
    Replacement::draft_throws(0x1410f0ca0, "unknown_1410f0ca0", fn_1410f0ca0 as *const (), Throws::Discard), // sub_1410f0ca0
    Replacement::draft_throws(0x1410f1080, "unknown_1410f1080", fn_1410f1080 as *const (), Throws::Discard), // sub_1410f1080
    Replacement::draft_throws(0x1410f1160, "unknown_1410f1160", fn_1410f1160 as *const (), Throws::Caught), // sub_1410f1160
    Replacement::draft_throws(0x1410f1580, "unknown_1410f1580", fn_1410f1580 as *const (), Throws::Caught), // sub_1410f1580
    Replacement::draft_throws(0x1410f19a0, "unknown_1410f19a0", fn_1410f19a0 as *const (), Throws::Discard), // sub_1410f19a0
    Replacement::draft(0x1410f1a70, "std_1410f1a70", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_1>,void,
    Replacement::draft(0x1410f1a80, "std_1410f1a80", tpl_1::<0x143890720> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<227,content::GenericComponent>::<lambda_1>,vo
    Replacement::draft_throws(0x1410f1a90, "std_1410f1a90", fn_1410f1a90 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_1>,void,
    Replacement::draft(0x1410f1af0, "std_1410f1af0", tpl_20903::<0x143399428, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<227,content::GenericComponent>::<lambda_1>,vo
    Replacement::draft(0x1410f1b20, "std_1410f1b20", tpl_20904::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_2>,void,
    Replacement::draft(0x1410f1b30, "std_1410f1b30", tpl_1::<0x143890a20> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<227,content::GenericComponent>::<lambda_2>,vo
    Replacement::draft_throws(0x1410f1b40, "std_1410f1b40", tpl_20905::<0x1410f0d90> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_2>,void,
    Replacement::draft(0x1410f1b70, "std_1410f1b70", tpl_20906::<0x143399498, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<227,content::GenericComponent>::<lambda_2>,vo
    Replacement::draft(0x1410f1ba0, "std_1410f1ba0", tpl_1::<0x1438908a0> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_1>,void,
    Replacement::draft(0x1410f1bb0, "std_1410f1bb0", tpl_20903::<0x143399460, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_1>,void,
    Replacement::draft(0x1410f1be0, "std_1410f1be0", tpl_1::<0x1438905a0> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_2>,void,
    Replacement::draft(0x1410f1bf0, "std_1410f1bf0", tpl_20906::<0x1433994d0, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<coregame::GenericEntityUtils::createContentBlocks<156,content::GenericEntity>::<lambda_2>,void,
    Replacement::draft(0x1410f1c20, "unknown_1410f1c20", fn_1410f1c20 as *const ()), // sub_1410f1c20
    Replacement::draft(0x1410f1d70, "unknown_1410f1d70", fn_1410f1d70 as *const ()), // sub_1410f1d70
    Replacement::draft(0x1410f1de0, "unknown_1410f1de0", tpl_60622::<0x10, 0x18, 0x143c8d108, 0x142489ee0, 0xc03, 0x143c8d160, 0x20, 0x38, 0x40> as *const ()), // sub_1410f1de0
    Replacement::draft(0x1410f1ee0, "luau_1410f1ee0", tpl_1368::<0x143c8d108, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f1ee0
    Replacement::draft(0x1410f2210, "luau_1410f2210", tpl_1368::<0x143cd6c54, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f2210
    Replacement::draft(0x1410f2540, "luau_1410f2540", tpl_1094::<0x143cd6d0c, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_1410f2540
    Replacement::draft(0x1410f2740, "luau_1410f2740", tpl_1466::<0x143c9a4e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f2740
    Replacement::draft(0x1410f2a10, "luau_1410f2a10", tpl_1368::<0x143ccb120, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f2a10
    Replacement::draft(0x1410f2d40, "unknown_1410f2d40", fn_1410f2d40 as *const ()), // sub_1410f2d40
    Replacement::draft(0x1410f2d70, "unknown_1410f2d70", fn_1410f2d70 as *const ()), // sub_1410f2d70
    Replacement::draft_throws(0x1410f2e10, "unknown_1410f2e10", fn_1410f2e10 as *const (), Throws::Caught), // sub_1410f2e10
    Replacement::draft(0x1410f3190, "unknown_1410f3190", tpl_6271::<0xc, 0x8, 0x1401dec00, 0x1401e3e80> as *const ()), // sub_1410f3190
    Replacement::draft(0x1410f3220, "unknown_1410f3220", fn_1410f3220 as *const ()), // sub_1410f3220
    Replacement::draft_throws(0x1410f33d0, "anonymous_namespace_1410f33d0", tpl_3591::<0x143cdd418, 0x143cdd420, 0x1411e9f50, 0x1410f5470, 0x14384ee58, 0x1420af120, 0x1410da920, 0x1420a28e0, 0x1410f4b00, 0x1410f49e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339a506, 0x142646960, 0x143cdd440, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::initEditorCamera>
    Replacement::draft_throws(0x1410f36c0, "coregame_1410f36c0", fn_1410f36c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::editorcamera::editorCameraRemoved>
    Replacement::draft_throws(0x1410f3990, "coregame_1410f3990", tpl_4703::<0x143cdd600, 0x143cdd540, 0x1410f4730, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143399cc6, 0x142646960, 0x143cdd550, 0x143cdd558, 0x143cdd55c, 0x143cdd55e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::editorcamera::update>
    Replacement::draft(0x1410f3ba0, "unknown_1410f3ba0", tpl_1095::<0x14384ed68, 0x14020e8f0> as *const ()), // sub_1410f3ba0
    Replacement::draft(0x1410f3d80, "unknown_1410f3d80", tpl_3073::<0x1438542ec, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f3d80
    Replacement::draft(0x1410f40e0, "unknown_1410f40e0", tpl_1466::<0x14384ed6c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f40e0
    Replacement::draft(0x1410f43b0, "unknown_1410f43b0", tpl_3072::<0x143cdd39c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410f43b0
    Replacement::draft_throws(0x1410f4730, "unknown_1410f4730", fn_1410f4730 as *const (), Throws::Discard), // sub_1410f4730
    Replacement::draft(0x1410f49e0, "unknown_1410f49e0", tpl_64030::<0x88, 0x10088, 0x141f7d130, 0x141f7dee0> as *const ()), // sub_1410f49e0
    Replacement::draft(0x1410f4b00, "unknown_1410f4b00", tpl_64033::<0x143c8d120, 0x142489ee0, 0x143cdd39c, 0x143c8cf5c, 0x141f7dee0> as *const ()), // sub_1410f4b00
    Replacement::draft(0x1410f4d20, "unknown_1410f4d20", tpl_6144::<0x1438542ec, 0x5e308, 0x10, 0x8, 0x18, 0x141f7dcf0, 0x141f7ca20> as *const ()), // sub_1410f4d20
    Replacement::draft_throws(0x1410f4dc0, "unknown_1410f4dc0", tpl_4769::<0x8, 0x20, 0x14384ee5c, 0x143c94b44, 0x141f7e3c0> as *const (), Throws::Discard), // sub_1410f4dc0
    Replacement::draft(0x1410f5230, "unknown_1410f5230", fn_1410f5230 as *const ()), // sub_1410f5230
    Replacement::draft(0x1410f53d0, "unknown_1410f53d0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14339a868, 0x8, 0x20> as *const ()), // sub_1410f53d0
    Replacement::draft(0x1410f5470, "unknown_1410f5470", tpl_20815::<0x143cdd394, 0x143cdd508, 0x8, 0x10, 0x18, 0x141834a78, 0x14384ed68, 0x143c8d120, 0x143cdd39c, 0x143c8cf5c, 0x143cdd528, 0x143cdd538, 0x141834a0c> as *const ()), // sub_1410f5470
    Replacement::draft_throws(0x1410f55b0, "unknown_1410f55b0", tpl_4541::<0x14384ee5c, 0x1410f5230, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1410f55b0
    Replacement::draft_throws(0x1410f5640, "unknown_1410f5640", tpl_4541::<0x14384ee58, 0x1410f5470, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1410f5640
    Replacement::draft(0x1410f56d0, "unknown_1410f56d0", fn_1410f56d0 as *const ()), // sub_1410f56d0
    Replacement::draft(0x1410f5860, "unknown_1410f5860", tpl_170::<0x1410f5db0> as *const ()), // sub_1410f5860
    Replacement::draft(0x1410f5870, "unknown_1410f5870", tpl_427::<0x1410f5cd0, 0x142442de0> as *const ()), // sub_1410f5870
    Replacement::draft(0x1410f5920, "unknown_1410f5920", tpl_5539::<0x8, 0x1401de830, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410f5920
    Replacement::draft_throws(0x1410f59f0, "unknown_1410f59f0", fn_1410f59f0 as *const (), Throws::Caught), // sub_1410f59f0
    Replacement::draft(0x1410f5a90, "unknown_1410f5a90", fn_1410f5a90 as *const ()), // sub_1410f5a90
    Replacement::draft(0x1410f5c20, "unknown_1410f5c20", fn_1410f5c20 as *const ()), // sub_1410f5c20
    Replacement::draft(0x1410f5cd0, "unknown_1410f5cd0", fn_1410f5cd0 as *const ()), // sub_1410f5cd0
    Replacement::draft(0x1410f5d60, "unknown_1410f5d60", tpl_365::<0x1410f5920> as *const ()), // sub_1410f5d60
    Replacement::draft(0x1410f5d70, "unknown_1410f5d70", fn_1410f5d70 as *const ()), // sub_1410f5d70
    Replacement::draft(0x1410f5da0, "unknown_1410f5da0", tpl_170::<0x1410f5f00> as *const ()), // sub_1410f5da0
    Replacement::draft(0x1410f5db0, "unknown_1410f5db0", fn_1410f5db0 as *const ()), // sub_1410f5db0
    Replacement::draft_throws(0x1410f5e80, "unknown_1410f5e80", fn_1410f5e80 as *const (), Throws::Discard), // sub_1410f5e80
    Replacement::draft(0x1410f5f00, "unknown_1410f5f00", tpl_10126::<0x8, 0x1410f5fc0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410f5f00
    Replacement::draft(0x1410f5fc0, "unknown_1410f5fc0", fn_1410f5fc0 as *const ()), // sub_1410f5fc0
    Replacement::draft(0x1410f6020, "unknown_1410f6020", fn_1410f6020 as *const ()), // sub_1410f6020
    Replacement::draft(0x1410f61a0, "unknown_1410f61a0", fn_1410f61a0 as *const ()), // sub_1410f61a0
    Replacement::draft(0x1410f6220, "unknown_1410f6220", fn_1410f6220 as *const ()), // sub_1410f6220
    Replacement::draft(0x1410f6320, "unknown_1410f6320", tpl_170::<0x1410f6370> as *const ()), // sub_1410f6320
    Replacement::draft(0x1410f6330, "unknown_1410f6330", tpl_20928::<0x8, 0x18, 0x1401e59a0, 0x14240ab10> as *const ()), // sub_1410f6330
    Replacement::draft(0x1410f6370, "unknown_1410f6370", fn_1410f6370 as *const ()), // sub_1410f6370
    Replacement::draft(0x1410f6460, "unknown_1410f6460", tpl_5539::<0x8, 0x1401de830, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410f6460
    Replacement::draft_throws(0x1410f6530, "unknown_1410f6530", fn_1410f6530 as *const (), Throws::Caught), // sub_1410f6530
    Replacement::draft(0x1410f6770, "unknown_1410f6770", fn_1410f6770 as *const ()), // sub_1410f6770
    Replacement::draft(0x1410f6980, "unknown_1410f6980", tpl_170::<0x1410f6a40> as *const ()), // sub_1410f6980
    Replacement::draft(0x1410f6990, "unknown_1410f6990", fn_1410f6990 as *const ()), // sub_1410f6990
    Replacement::draft(0x1410f69c0, "unknown_1410f69c0", fn_1410f69c0 as *const ()), // sub_1410f69c0
    Replacement::draft(0x1410f6a40, "unknown_1410f6a40", fn_1410f6a40 as *const ()), // sub_1410f6a40
    Replacement::draft(0x1410f6b30, "unknown_1410f6b30", tpl_61657::<0x8, 0x14, 0x14240ab10, 0x1401e66f0> as *const ()), // sub_1410f6b30
    Replacement::draft(0x1410f6ba0, "unknown_1410f6ba0", fn_1410f6ba0 as *const ()), // sub_1410f6ba0
    Replacement::draft(0x1410f6c20, "unknown_1410f6c20", fn_1410f6c20 as *const ()), // sub_1410f6c20
    Replacement::draft(0x1410f6cd0, "unknown_1410f6cd0", fn_1410f6cd0 as *const ()), // sub_1410f6cd0
    Replacement::draft(0x1410f6e50, "unknown_1410f6e50", fn_1410f6e50 as *const ()), // sub_1410f6e50
    Replacement::draft_throws(0x1410f6f20, "unknown_1410f6f20", tpl_17350::<0x143881018, 0x143881020, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x1410f9760, 0x1401e9e70> as *const (), Throws::Discard), // sub_1410f6f20
    Replacement::draft_throws(0x1410f6fc0, "unknown_1410f6fc0", tpl_17351::<0x143881020, 0x14243b0b0, 0x338, 0x48, 0x1410f9da0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_1410f6fc0
    Replacement::draft_throws(0x1410f7070, "unknown_1410f7070", tpl_17352::<0x143881018, 0x143881020, 0x1401e9b60, 0x328, 0x338, 0x120, 0x1410fa370, 0x1401e9a20> as *const (), Throws::Discard), // sub_1410f7070
    Replacement::draft_throws(0x1410f7110, "unknown_1410f7110", fn_1410f7110 as *const (), Throws::Discard), // sub_1410f7110
    Replacement::draft_throws(0x1410f7280, "unknown_1410f7280", fn_1410f7280 as *const (), Throws::Caught), // sub_1410f7280
    Replacement::draft_throws(0x1410f7320, "unknown_1410f7320", fn_1410f7320 as *const (), Throws::Caught), // sub_1410f7320
    Replacement::draft_throws(0x1410f73c0, "unknown_1410f73c0", fn_1410f73c0 as *const (), Throws::Caught), // sub_1410f73c0
    Replacement::draft(0x1410f7460, "unknown_1410f7460", fn_1410f7460 as *const ()), // sub_1410f7460
    Replacement::draft(0x1410f7590, "unknown_1410f7590", fn_1410f7590 as *const ()), // sub_1410f7590
    Replacement::draft(0x1410f7650, "unknown_1410f7650", fn_1410f7650 as *const ()), // sub_1410f7650
    Replacement::draft_throws(0x1410f7710, "unknown_1410f7710", fn_1410f7710 as *const (), Throws::Discard), // sub_1410f7710
    Replacement::draft_throws(0x1410f77c0, "unknown_1410f77c0", tpl_17357::<0x143881020, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14339aac8, 0x1410f7fb0> as *const (), Throws::Discard), // sub_1410f77c0
    Replacement::draft_throws(0x1410f7860, "std_1410f7860", fn_1410f7860 as *const (), Throws::Discard), // sub_1410f7860
    Replacement::draft(0x1410f7ac0, "std_1410f7ac0", fn_1410f7ac0 as *const ()), // sub_1410f7ac0
    Replacement::draft_throws(0x1410f7af0, "std_1410f7af0", fn_1410f7af0 as *const (), Throws::Discard), // sub_1410f7af0
    Replacement::draft(0x1410f7cc0, "std_1410f7cc0", fn_1410f7cc0 as *const ()), // sub_1410f7cc0
    Replacement::draft(0x1410f7cd0, "std_1410f7cd0", fn_1410f7cd0 as *const ()), // sub_1410f7cd0
    Replacement::draft(0x1410f7d90, "std_1410f7d90", fn_1410f7d90 as *const ()), // sub_1410f7d90
    Replacement::draft_throws(0x1410f7fb0, "unknown_1410f7fb0", fn_1410f7fb0 as *const (), Throws::Caught), // sub_1410f7fb0
    Replacement::draft(0x1410f80f0, "unknown_1410f80f0", fn_1410f80f0 as *const ()), // sub_1410f80f0
    Replacement::draft(0x1410f8360, "unknown_1410f8360", fn_1410f8360 as *const ()), // sub_1410f8360
    Replacement::draft_throws(0x1410f83c0, "unknown_1410f83c0", fn_1410f83c0 as *const (), Throws::Caught), // sub_1410f83c0
    Replacement::draft_throws(0x1410f8a40, "unknown_1410f8a40", fn_1410f8a40 as *const (), Throws::Caught), // sub_1410f8a40
    Replacement::draft_throws(0x1410f9070, "unknown_1410f9070", fn_1410f9070 as *const (), Throws::Caught), // sub_1410f9070
    Replacement::draft(0x1410f96a0, "unknown_1410f96a0", fn_1410f96a0 as *const ()), // sub_1410f96a0
    Replacement::draft(0x1410f9720, "unknown_1410f9720", fn_1410f9720 as *const ()), // sub_1410f9720
    Replacement::draft_throws(0x1410f9760, "unknown_1410f9760", fn_1410f9760 as *const (), Throws::Caught), // sub_1410f9760
    Replacement::draft_throws(0x1410f9da0, "unknown_1410f9da0", fn_1410f9da0 as *const (), Throws::Discard), // sub_1410f9da0
    Replacement::draft_throws(0x1410fa370, "unknown_1410fa370", fn_1410fa370 as *const (), Throws::Caught), // sub_1410fa370
    Replacement::draft_throws(0x1410fa9d0, "unknown_1410fa9d0", fn_1410fa9d0 as *const (), Throws::Caught), // sub_1410fa9d0
    Replacement::draft_throws(0x1410fb080, "unknown_1410fb080", fn_1410fb080 as *const (), Throws::Caught), // sub_1410fb080
    Replacement::draft_throws(0x1410fb730, "unknown_1410fb730", fn_1410fb730 as *const (), Throws::Caught), // sub_1410fb730
    Replacement::draft_throws(0x1410fbde0, "unknown_1410fbde0", tpl_13418::<0x1410fbf60> as *const (), Throws::Discard), // sub_1410fbde0
    Replacement::draft_throws(0x1410fbdf0, "unknown_1410fbdf0", fn_1410fbdf0 as *const (), Throws::Discard), // sub_1410fbdf0
    Replacement::draft_throws(0x1410fbf60, "unknown_1410fbf60", fn_1410fbf60 as *const (), Throws::Discard), // sub_1410fbf60
    Replacement::draft(0x1410fbfc0, "unknown_1410fbfc0", fn_1410fbfc0 as *const ()), // sub_1410fbfc0
    Replacement::draft(0x1410fc010, "unknown_1410fc010", fn_1410fc010 as *const ()), // sub_1410fc010
    Replacement::draft(0x1410fc030, "unknown_1410fc030", fn_1410fc030 as *const ()), // sub_1410fc030
    Replacement::draft(0x1410fc080, "unknown_1410fc080", tpl_20959::<0x8, 0x10, 0x30, 0x3, 0x4, 0x14240aa00, 0x48, 0x18, 0x4c, 0x1c, 0x50> as *const ()), // sub_1410fc080
    Replacement::draft(0x1410fc120, "unknown_1410fc120", fn_1410fc120 as *const ()), // sub_1410fc120
    Replacement::draft(0x1410fc180, "unknown_1410fc180", tpl_20961::<0x14240ab10> as *const ()), // sub_1410fc180
    Replacement::draft(0x1410fc190, "unknown_1410fc190", fn_1410fc190 as *const ()), // sub_1410fc190
    Replacement::draft(0x1410fc1c0, "unknown_1410fc1c0", tpl_1265::<0x20, 0x8> as *const ()), // sub_1410fc1c0
    Replacement::draft(0x1410fc1d0, "unknown_1410fc1d0", fn_1410fc1d0 as *const ()), // sub_1410fc1d0
    Replacement::draft(0x1410fc1f0, "unknown_1410fc1f0", fn_1410fc1f0 as *const ()), // sub_1410fc1f0
    Replacement::draft(0x1410fc210, "unknown_1410fc210", tpl_3857::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x20, 0x1410fd1a0> as *const ()), // sub_1410fc210
    Replacement::draft(0x1410fc2a0, "unknown_1410fc2a0", fn_1410fc2a0 as *const ()), // sub_1410fc2a0
    Replacement::draft(0x1410fc370, "unknown_1410fc370", tpl_5915::<0x8, 0x20, 0x140236fa0, 0x1401e66f0> as *const ()), // sub_1410fc370
    Replacement::draft_throws(0x1410fc3e0, "unknown_1410fc3e0", fn_1410fc3e0 as *const (), Throws::Discard), // sub_1410fc3e0
    Replacement::draft(0x1410fc400, "unknown_1410fc400", tpl_1606::<0x8, 0x140267600, 0x10> as *const ()), // sub_1410fc400
    Replacement::draft(0x1410fc440, "unknown_1410fc440", fn_1410fc440 as *const ()), // sub_1410fc440
    Replacement::draft(0x1410fc460, "unknown_1410fc460", fn_1410fc460 as *const ()), // sub_1410fc460
    Replacement::draft(0x1410fc4e0, "unknown_1410fc4e0", fn_1410fc4e0 as *const ()), // sub_1410fc4e0
    Replacement::draft(0x1410fc550, "unknown_1410fc550", fn_1410fc550 as *const ()), // sub_1410fc550
    Replacement::draft(0x1410fc5c0, "std_1410fc5c0", fn_1410fc5c0 as *const ()), // sub_1410fc5c0
    Replacement::draft(0x1410fc820, "unknown_1410fc820", tpl_808::<0x1410fc460, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410fc820
    Replacement::draft(0x1410fc8d0, "unknown_1410fc8d0", tpl_808::<0x1410fc4e0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410fc8d0
    Replacement::draft(0x1410fc980, "unknown_1410fc980", tpl_808::<0x1410fc550, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410fc980
    Replacement::draft(0x1410fca30, "unknown_1410fca30", tpl_63158::<0x143caef10, 0x142489ee0, 0x143cba178, 0x143cdd924, 0x143cd6de0> as *const ()), // sub_1410fca30
    Replacement::draft(0x1410fcba0, "unknown_1410fcba0", tpl_60875::<0x143caef10, 0x142489ee0, 0x143c8d160, 0x143c8d120, 0x143cba178, 0x143cdd924, 0x143cd6de0> as *const ()), // sub_1410fcba0
    Replacement::draft(0x1410fcd10, "unknown_1410fcd10", tpl_60622::<0x10, 0x18, 0x143c8d160, 0x142489ee0, 0xc03, 0x143c8d120, 0x20, 0x38, 0x40> as *const ()), // sub_1410fcd10
    Replacement::draft(0x1410fcee0, "unknown_1410fcee0", tpl_3049::<0x143caef10, 0x142489ee0, 0x143ca83d8, 0x143cba178, 0x143c8d160, 0x143c8d120, 0x143cdd924, 0x143cd6de0, 0x143cc8944> as *const ()), // sub_1410fcee0
    Replacement::draft(0x1410fd0a0, "unknown_1410fd0a0", tpl_60622::<0x10, 0x18, 0x143caef10, 0x142489ee0, 0xc03, 0x143c8d160, 0x20, 0x38, 0x40> as *const ()), // sub_1410fd0a0
    Replacement::draft(0x1410fd240, "unknown_1410fd240", tpl_60875::<0x143cd6e28, 0x142489ee0, 0x143cba178, 0x143c8d160, 0x143c8d120, 0x143c8d87c, 0x143caef10> as *const ()), // sub_1410fd240
    Replacement::draft(0x1410fd3b0, "unknown_1410fd3b0", fn_1410fd3b0 as *const ()), // sub_1410fd3b0
    Replacement::draft(0x1410fd410, "unknown_1410fd410", fn_1410fd410 as *const ()), // sub_1410fd410
    Replacement::draft(0x1410fd550, "unknown_1410fd550", fn_1410fd550 as *const ()), // sub_1410fd550
    Replacement::draft(0x1410fd600, "unknown_1410fd600", fn_1410fd600 as *const ()), // sub_1410fd600
    Replacement::draft(0x1410fd670, "unknown_1410fd670", tpl_170::<0x1410ff700> as *const ()), // sub_1410fd670
    Replacement::draft(0x1410fd680, "unknown_1410fd680", fn_1410fd680 as *const ()), // sub_1410fd680
    Replacement::draft(0x1410fd750, "unknown_1410fd750", tpl_20977::<0xb0, 0x14020c520, 0x20, 0x18, 0x140226cb0, 0x14020bfd0, 0x14339af38, 0x1, 0x4, 0x8> as *const ()), // sub_1410fd750
    Replacement::draft(0x1410fd800, "unknown_1410fd800", tpl_20977::<0xb0, 0x14020c520, 0x20, 0x18, 0x140226cb0, 0x14020bfd0, 0x14339ae40, 0x1, 0x4, 0x8> as *const ()), // sub_1410fd800
    Replacement::draft(0x1410fd8b0, "unknown_1410fd8b0", fn_1410fd8b0 as *const ()), // sub_1410fd8b0
    Replacement::draft(0x1410fd910, "unknown_1410fd910", tpl_20979::<0x8, 0xc, 0x141100500, 0x1, 0x1410ff320> as *const ()), // sub_1410fd910
    Replacement::draft_throws(0x1410fd980, "unknown_1410fd980", fn_1410fd980 as *const (), Throws::Caught), // sub_1410fd980
    Replacement::draft(0x1410fdac0, "unknown_1410fdac0", fn_1410fdac0 as *const ()), // sub_1410fdac0
    Replacement::draft_throws(0x1410fdb00, "unknown_1410fdb00", fn_1410fdb00 as *const (), Throws::Caught), // sub_1410fdb00
    Replacement::draft(0x1410fdc30, "unknown_1410fdc30", tpl_20979::<0x8, 0xc, 0x1411006e0, 0x1, 0x1410ff3e0> as *const ()), // sub_1410fdc30
    Replacement::draft(0x1410fdca0, "unknown_1410fdca0", fn_1410fdca0 as *const ()), // sub_1410fdca0
    Replacement::draft_throws(0x1410fdd10, "unknown_1410fdd10", fn_1410fdd10 as *const (), Throws::Caught), // sub_1410fdd10
    Replacement::draft_throws(0x1410fde70, "coregame_1410fde70", tpl_3696::<0x143cddcb0, 0x143cddb10, 0x1411e9f50, 0x141103e20, 0x1411052b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339c886, 0x142646960, 0x143cddb30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::processEvents>
    Replacement::draft_throws(0x1410fe0b0, "coregame_1410fe0b0", tpl_5273::<0x143cdd77c, 0x143cdda18, 0x1411e9f50, 0x1411040b0, 0x141105310, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339bf26, 0x142646960, 0x143cdda38, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::processCommands>
    Replacement::draft_throws(0x1410fe2f0, "coregame_1410fe2f0", tpl_3004::<0x143cdd7cc, 0x143cddf70, 0x141100c90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339bbd6, 0x142646960, 0x143cddf90, 0x143cddf94, 0x143cddf96, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::processChainRemove>
    Replacement::draft_throws(0x1410fe500, "coregame_1410fe500", tpl_6249::<0x143cdd774, 0x143cdd900, 0x141100ed0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339b836, 0x142646960, 0x143cdd920, 0x143cdd922, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::processActions>
    Replacement::draft_throws(0x1410fe700, "coregame_1410fe700", tpl_5175::<0x143cddf98, 0x143cdde40, 0x141101150, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339b386, 0x142646960, 0x143cdde60, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::processPlayback>
    Replacement::draft_throws(0x1410fe900, "coregame_1410fe900", tpl_1467::<0x143cddd88, 0x143cddcc0, 0x1411e9f50, 0x1411065f0, 0x143cdd92c, 0x1420af120, 0x1420a28e0, 0x1411054e0, 0x141104410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339b086, 0x142646960, 0x143cddce0, 0x143cddce8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::updateDialogueSubSource>
    Replacement::draft(0x1410febe0, "unknown_1410febe0", tpl_1526::<0x142925eb0, 0x1410d6ca0, 0x1411017f0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1410febe0
    Replacement::draft_throws(0x1410fee80, "std_1410fee80", fn_1410fee80 as *const (), Throws::Discard), // sub_1410fee80
    Replacement::draft_throws(0x1410ff110, "unknown_1410ff110", fn_1410ff110 as *const (), Throws::Discard), // sub_1410ff110
    Replacement::draft(0x1410ff280, "unknown_1410ff280", fn_1410ff280 as *const ()), // sub_1410ff280
    Replacement::draft(0x1410ff320, "unknown_1410ff320", tpl_20988::<0x8, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x14, 0x1401e84a0, 0x40, 0x48, 0x3, 0x4, 0x14240a730> as *const ()), // sub_1410ff320
    Replacement::draft(0x1410ff3e0, "unknown_1410ff3e0", tpl_20988::<0x8, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x14, 0x1401e84a0, 0x40, 0x48, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1410ff3e0
    Replacement::draft(0x1410ff4a0, "unknown_1410ff4a0", tpl_20959::<0x8, 0x10, 0x30, 0x3, 0x4, 0x14240a730, 0x48, 0x18, 0x4c, 0x1c, 0x50> as *const ()), // sub_1410ff4a0
    Replacement::draft(0x1410ff540, "unknown_1410ff540", tpl_6561::<0x8, 0x140236fa0, 0x1401fb4d0> as *const ()), // sub_1410ff540
    Replacement::draft(0x1410ff570, "unknown_1410ff570", fn_1410ff570 as *const ()), // sub_1410ff570
    Replacement::draft(0x1410ff580, "unknown_1410ff580", fn_1410ff580 as *const ()), // sub_1410ff580
    Replacement::draft(0x1410ff700, "unknown_1410ff700", fn_1410ff700 as *const ()), // sub_1410ff700
    Replacement::draft(0x1410ff7d0, "unknown_1410ff7d0", fn_1410ff7d0 as *const ()), // sub_1410ff7d0
    Replacement::draft(0x1410ffa40, "unknown_1410ffa40", tpl_1369::<0x143cdd924, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410ffa40
    Replacement::draft(0x1410ffd80, "unknown_1410ffd80", tpl_1094::<0x143cd6e28, 0x1411026c0, 0x14020e8f0> as *const ()), // sub_1410ffd80
    Replacement::draft(0x1410fff80, "unknown_1410fff80", tpl_1094::<0x143caef10, 0x141102840, 0x14020e8f0> as *const ()), // sub_1410fff80
    Replacement::draft(0x141100180, "unknown_141100180", fn_141100180 as *const ()), // sub_141100180
    Replacement::draft(0x141100370, "unknown_141100370", fn_141100370 as *const ()), // sub_141100370
    Replacement::draft(0x141100500, "unknown_141100500", tpl_20993::<0x1403d9d80, 0x1410ff320, 0x141103be0, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_141100500
    Replacement::draft(0x1411006e0, "unknown_1411006e0", tpl_20993::<0x1403d9d80, 0x1410ff3e0, 0x141103be0, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1411006e0
    Replacement::draft(0x1411008c0, "unknown_1411008c0", fn_1411008c0 as *const ()), // sub_1411008c0
    Replacement::draft(0x141100a90, "unknown_141100a90", tpl_1094::<0x143caef04, 0x141103ca0, 0x14020e8f0> as *const ()), // sub_141100a90
    Replacement::draft_throws(0x141100c90, "unknown_141100c90", fn_141100c90 as *const (), Throws::Discard), // sub_141100c90
    Replacement::draft_throws(0x141100ed0, "unknown_141100ed0", tpl_20996::<0x1411e9f50, 0x141106900, 0x143cdda84, 0x1420af120, 0x140215750, 0x143cdda10, 0x143cde0dc, 0x1420a25b0, 0x1420a28e0, 0x141105420, 0x1401de410, 0x141834a78, 0x143cd6e20, 0x143cddd58, 0x143c8ce88, 0x143c8d910, 0x143cde0d8, 0x143cde0f8, 0x141834a0c> as *const (), Throws::Discard), // sub_141100ed0
    Replacement::draft_throws(0x141101150, "unknown_141101150", tpl_20996::<0x1411e9f50, 0x141106770, 0x143cde0b4, 0x1420af120, 0x140215750, 0x143cddac0, 0x143cddfbc, 0x1420a25b0, 0x1420a28e0, 0x141105480, 0x1401de410, 0x141834a78, 0x143cddd58, 0x143cd6e2c, 0x143c8ceec, 0x143c8d910, 0x143cddfb8, 0x143cddfd8, 0x141834a0c> as *const (), Throws::Discard), // sub_141101150
    Replacement::draft(0x1411013d0, "unknown_1411013d0", fn_1411013d0 as *const ()), // sub_1411013d0
    Replacement::draft_throws(0x1411015e0, "coregame_1411015e0", tpl_20473::<0x143cddb0c, 0x143cdd710, 0x1411045f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339f0e6, 0x142646960, 0x143cdd730, 0x143cdd750, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::DialogueControllerC
    Replacement::draft(0x1411017f0, "coregame_1411017f0", fn_1411017f0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::dialogue::removedDialogueSource>
    Replacement::draft_throws(0x141101ad0, "coregame_141101ad0", fn_141101ad0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::streamIn>
    Replacement::draft_throws(0x141101de0, "coregame_141101de0", tpl_1963::<0x143cde100, 0x143cddc18, 0x1411e9f50, 0x141107e20, 0x143cddfdc, 0x1420af120, 0x1420a28e0, 0x1411063d0, 0x141105800, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339ea86, 0x142646960, 0x143cddc28, 0x143cddc30, 0x143cddc34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogue::streamOut>
    Replacement::draft(0x1411020f0, "unknown_1411020f0", fn_1411020f0 as *const ()), // sub_1411020f0
    Replacement::draft(0x141102130, "unknown_141102130", fn_141102130 as *const ()), // sub_141102130
    Replacement::draft_throws(0x141102190, "unknown_141102190", fn_141102190 as *const (), Throws::Discard), // sub_141102190
    Replacement::draft_throws(0x141102390, "unknown_141102390", fn_141102390 as *const (), Throws::Discard), // sub_141102390
    Replacement::draft(0x1411026c0, "unknown_1411026c0", tpl_6546::<0x1401e6d50, 0x141104f10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x141076700> as *const ()), // sub_1411026c0
    Replacement::draft(0x141102840, "unknown_141102840", fn_141102840 as *const ()), // sub_141102840
    Replacement::draft(0x1411029c0, "unknown_1411029c0", fn_1411029c0 as *const ()), // sub_1411029c0
    Replacement::draft(0x141102a20, "unknown_141102a20", fn_141102a20 as *const ()), // sub_141102a20
    Replacement::draft(0x141102c70, "unknown_141102c70", fn_141102c70 as *const ()), // sub_141102c70
    Replacement::draft(0x141102f10, "unknown_141102f10", fn_141102f10 as *const ()), // sub_141102f10
    Replacement::draft(0x1411030d0, "unknown_1411030d0", fn_1411030d0 as *const ()), // sub_1411030d0
    Replacement::draft(0x141103be0, "unknown_141103be0", tpl_20046::<0x1410ff320> as *const ()), // sub_141103be0
    Replacement::draft(0x141103c40, "unknown_141103c40", tpl_19165::<0x1410ff4a0> as *const ()), // sub_141103c40
    Replacement::draft(0x141103ca0, "unknown_141103ca0", fn_141103ca0 as *const ()), // sub_141103ca0
    Replacement::draft_throws(0x141103e20, "unknown_141103e20", fn_141103e20 as *const (), Throws::Discard), // sub_141103e20
    Replacement::draft_throws(0x1411040b0, "unknown_1411040b0", fn_1411040b0 as *const (), Throws::Discard), // sub_1411040b0
    Replacement::draft(0x141104410, "unknown_141104410", tpl_1889::<0x40, 0x48, 0x10048, 0x141104c40, 0x141104510> as *const ()), // sub_141104410
    Replacement::draft(0x141104510, "unknown_141104510", tpl_1890::<0x18, 0x20, 0x40, 0x8, 0x141f881e0> as *const ()), // sub_141104510
    Replacement::draft_throws(0x1411045f0, "unknown_1411045f0", tpl_3011::<0x1411e9f50, 0x141108160, 0x143cdd934, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141106060, 0x1401de410> as *const (), Throws::Discard), // sub_1411045f0
    Replacement::draft(0x1411047b0, "unknown_1411047b0", fn_1411047b0 as *const ()), // sub_1411047b0
    Replacement::draft(0x141104a30, "unknown_141104a30", fn_141104a30 as *const ()), // sub_141104a30
    Replacement::draft(0x141104ab0, "unknown_141104ab0", fn_141104ab0 as *const ()), // sub_141104ab0
    Replacement::draft(0x141104d90, "unknown_141104d90", fn_141104d90 as *const ()), // sub_141104d90
    Replacement::draft(0x141104f10, "unknown_141104f10", fn_141104f10 as *const ()), // sub_141104f10
    Replacement::draft(0x141104f60, "unknown_141104f60", fn_141104f60 as *const ()), // sub_141104f60
    Replacement::draft(0x141104fb0, "unknown_141104fb0", fn_141104fb0 as *const ()), // sub_141104fb0
    Replacement::draft(0x141105180, "unknown_141105180", fn_141105180 as *const ()), // sub_141105180
    Replacement::draft_throws(0x1411052b0, "unknown_1411052b0", tpl_5128::<0x143cdd93c, 0x20, 0x8, 0x141f85a80> as *const (), Throws::Discard), // sub_1411052b0
    Replacement::draft_throws(0x141105310, "unknown_141105310", tpl_4769::<0x8, 0x20, 0x143cddbd4, 0x143cdda78, 0x141f84870> as *const (), Throws::Discard), // sub_141105310
    Replacement::draft_throws(0x1411053c0, "unknown_1411053c0", tpl_5128::<0x143cddb00, 0x20, 0x8, 0x141f82c30> as *const (), Throws::Discard), // sub_1411053c0
    Replacement::draft_throws(0x141105420, "unknown_141105420", tpl_5128::<0x143cdda84, 0x20, 0x8, 0x141f84330> as *const (), Throws::Discard), // sub_141105420
    Replacement::draft_throws(0x141105480, "unknown_141105480", tpl_5128::<0x143cde0b4, 0x20, 0x8, 0x141f82fa0> as *const (), Throws::Discard), // sub_141105480
    Replacement::draft(0x1411054e0, "unknown_1411054e0", tpl_61544::<0x141104c40, 0x141f881e0> as *const ()), // sub_1411054e0
    Replacement::draft_throws(0x1411056c0, "unknown_1411056c0", fn_1411056c0 as *const (), Throws::Caught), // sub_1411056c0
    Replacement::draft_throws(0x141105800, "unknown_141105800", tpl_21017::<0x88, 0x10088, 0x142407c80, 0x141105cd0, 0x580f0, 0x141f88460> as *const (), Throws::Discard), // sub_141105800
    Replacement::draft_throws(0x141105a10, "std_141105a10", fn_141105a10 as *const (), Throws::Discard), // sub_141105a10
    Replacement::draft(0x141105cd0, "unknown_141105cd0", tpl_1117::<0x143caef04, 0x142489ee0, 0x143cdd924, 0x143cd6de0> as *const ()), // sub_141105cd0
    Replacement::draft(0x141105e00, "unknown_141105e00", tpl_1117::<0x143caef04, 0x142489ee0, 0x143cd6de0, 0x143c8fa04> as *const ()), // sub_141105e00
    Replacement::draft(0x141105f30, "unknown_141105f30", tpl_12946::<0x8, 0x30, 0x14240ab10> as *const ()), // sub_141105f30
    Replacement::draft(0x141105f70, "unknown_141105f70", fn_141105f70 as *const ()), // sub_141105f70
    Replacement::draft(0x141105fb0, "unknown_141105fb0", fn_141105fb0 as *const ()), // sub_141105fb0
    Replacement::draft(0x141105ff0, "unknown_141105ff0", tpl_4270::<0x30, 0x141105180> as *const ()), // sub_141105ff0
    Replacement::draft(0x141106060, "unknown_141106060", tpl_3028::<0x143cdd934, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1411013d0> as *const ()), // sub_141106060
    Replacement::draft_throws(0x1411060d0, "unknown_1411060d0", fn_1411060d0 as *const (), Throws::Caught), // sub_1411060d0
    Replacement::draft_throws(0x141106180, "unknown_141106180", fn_141106180 as *const (), Throws::Caught), // sub_141106180
    Replacement::draft_throws(0x1411063d0, "unknown_1411063d0", tpl_21022::<0x142407c80, 0x143caef04, 0x142489ee0, 0x143cdd924, 0x143cd6de0, 0x141f88460> as *const (), Throws::Discard), // sub_1411063d0
    Replacement::draft(0x1411065f0, "unknown_1411065f0", fn_1411065f0 as *const ()), // sub_1411065f0
    Replacement::draft(0x141106770, "unknown_141106770", fn_141106770 as *const ()), // sub_141106770
    Replacement::draft(0x141106900, "unknown_141106900", fn_141106900 as *const ()), // sub_141106900
    Replacement::draft(0x141106a00, "unknown_141106a00", fn_141106a00 as *const ()), // sub_141106a00
    Replacement::draft(0x141106b40, "unknown_141106b40", tpl_5302::<0x143cdd778, 0x143cdd940, 0x8, 0x10, 0x18, 0x141834a78, 0x143caef10, 0x143c8d160, 0x143cdd950, 0x143cdd958, 0x141834a0c> as *const ()), // sub_141106b40
    Replacement::draft(0x141106c30, "unknown_141106c30", fn_141106c30 as *const ()), // sub_141106c30
    Replacement::draft(0x141106e40, "unknown_141106e40", fn_141106e40 as *const ()), // sub_141106e40
    Replacement::draft(0x141106fc0, "unknown_141106fc0", tpl_3188::<0x1411080c0, 0x14339f7d0, 0x1411073e0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141106fc0
    Replacement::draft(0x141107280, "unknown_141107280", fn_141107280 as *const ()), // sub_141107280
    Replacement::draft(0x1411073e0, "unknown_1411073e0", fn_1411073e0 as *const ()), // sub_1411073e0
    Replacement::draft(0x141107510, "unknown_141107510", tpl_4170::<0x141108fa0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1410fc980, 0x142442de0> as *const ()), // sub_141107510
    Replacement::draft(0x141107630, "unknown_141107630", fn_141107630 as *const ()), // sub_141107630
    Replacement::draft(0x141107770, "unknown_141107770", tpl_4170::<0x141108f00, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1410fc8d0, 0x142442de0> as *const ()), // sub_141107770
    Replacement::draft(0x141107890, "unknown_141107890", tpl_4170::<0x141108df0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1410fc820, 0x142442de0> as *const ()), // sub_141107890
    Replacement::draft(0x1411079b0, "unknown_1411079b0", fn_1411079b0 as *const ()), // sub_1411079b0
    Replacement::draft_throws(0x141107b80, "unknown_141107b80", fn_141107b80 as *const (), Throws::Discard), // sub_141107b80
    Replacement::draft(0x141107ce0, "unknown_141107ce0", tpl_5928::<0x142537280, 0x1426456c8, 0x1426453a0, 0x14025f9d0, 0x1401e5730> as *const ()), // sub_141107ce0
    Replacement::draft(0x1411080c0, "unknown_1411080c0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14339f7d0, 0x8, 0x20> as *const ()), // sub_1411080c0
    Replacement::draft(0x141108160, "unknown_141108160", tpl_5302::<0x143cddf30, 0x143cddea0, 0x8, 0x10, 0x18, 0x141834a78, 0x143c9a620, 0x143caef04, 0x143cddeb0, 0x143cddeb8, 0x141834a0c> as *const ()), // sub_141108160
    Replacement::draft(0x141108250, "unknown_141108250", fn_141108250 as *const ()), // sub_141108250
    Replacement::draft(0x1411084e0, "unknown_1411084e0", fn_1411084e0 as *const ()), // sub_1411084e0
    Replacement::draft(0x141108770, "unknown_141108770", fn_141108770 as *const ()), // sub_141108770
    Replacement::draft(0x1411089f0, "unknown_1411089f0", fn_1411089f0 as *const ()), // sub_1411089f0
    Replacement::draft(0x141108b50, "unknown_141108b50", fn_141108b50 as *const ()), // sub_141108b50
    Replacement::draft(0x141108ca0, "unknown_141108ca0", fn_141108ca0 as *const ()), // sub_141108ca0
    Replacement::draft(0x141108df0, "unknown_141108df0", fn_141108df0 as *const ()), // sub_141108df0
    Replacement::draft(0x141108f00, "unknown_141108f00", fn_141108f00 as *const ()), // sub_141108f00
    Replacement::draft(0x141108fa0, "unknown_141108fa0", fn_141108fa0 as *const ()), // sub_141108fa0
    Replacement::draft_throws(0x141109040, "unknown_141109040", fn_141109040 as *const (), Throws::Discard), // sub_141109040
    Replacement::draft_throws(0x1411091a0, "unknown_1411091a0", tpl_4541::<0x143cdd92c, 0x1411065f0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411091a0
    Replacement::draft_throws(0x141109230, "unknown_141109230", tpl_4541::<0x143cde0b4, 0x141106770, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141109230
    Replacement::draft_throws(0x1411092c0, "unknown_1411092c0", tpl_4541::<0x143cdda84, 0x141106900, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411092c0
    Replacement::draft_throws(0x141109350, "unknown_141109350", tpl_4541::<0x143cddb00, 0x141106a00, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141109350
    Replacement::draft_throws(0x1411093e0, "unknown_1411093e0", tpl_4541::<0x143cdda78, 0x141106b40, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411093e0
    Replacement::draft_throws(0x141109470, "unknown_141109470", tpl_4541::<0x143cddbd4, 0x141106c30, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141109470
    Replacement::draft_throws(0x141109500, "unknown_141109500", tpl_4541::<0x143cdd93c, 0x141106e40, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141109500
    Replacement::draft(0x141109640, "unknown_141109640", tpl_6572::<0x8, 0x18, 0x10, 0x20, 0x140236fa0, 0x1401e66f0> as *const ()), // sub_141109640
    Replacement::draft_throws(0x1411096a0, "unknown_1411096a0", tpl_4541::<0x143cddfdc, 0x141107e20, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411096a0
    Replacement::draft_throws(0x141109730, "unknown_141109730", tpl_4541::<0x143cdd930, 0x141107f70, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141109730
    Replacement::draft_throws(0x1411097c0, "unknown_1411097c0", tpl_4541::<0x143cdd934, 0x141108160, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411097c0
    Replacement::draft(0x141109850, "unknown_141109850", fn_141109850 as *const ()), // sub_141109850
    Replacement::draft(0x1411099f0, "unknown_1411099f0", fn_1411099f0 as *const ()), // sub_1411099f0
    Replacement::draft(0x141109af0, "unknown_141109af0", tpl_1663::<0x8, 0x140267600, 0x1401e66f0> as *const ()), // sub_141109af0
    Replacement::draft(0x141109b30, "unknown_141109b30", fn_141109b30 as *const ()), // sub_141109b30
    Replacement::draft_throws(0x141109c30, "std_141109c30", fn_141109c30 as *const (), Throws::Discard), // sub_141109c30
    Replacement::draft(0x141109ee0, "unknown_141109ee0", fn_141109ee0 as *const ()), // sub_141109ee0
    Replacement::draft(0x141109f60, "unknown_141109f60", fn_141109f60 as *const ()), // sub_141109f60
    Replacement::draft_throws(0x14110a080, "coregame_14110a080", tpl_5850::<0x143cde1d4, 0x143cde240, 0x1411e9f50, 0x14110af40, 0x143cde238, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x14110ac90, 0x14110a810, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14339fbf6, 0x142646960, 0x143cde260, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dialogueanimation::update>
    Replacement::draft(0x14110a380, "unknown_14110a380", fn_14110a380 as *const ()), // sub_14110a380
    Replacement::draft(0x14110a3f0, "unknown_14110a3f0", tpl_1094::<0x143cde18c, 0x14110a690, 0x14020e8f0> as *const ()), // sub_14110a3f0
    Replacement::draft(0x14110a620, "unknown_14110a620", fn_14110a620 as *const ()), // sub_14110a620
    Replacement::draft(0x14110a690, "unknown_14110a690", fn_14110a690 as *const ()), // sub_14110a690
    Replacement::draft_throws(0x14110a810, "unknown_14110a810", fn_14110a810 as *const (), Throws::Discard), // sub_14110a810
    Replacement::draft(0x14110a8d0, "unknown_14110a8d0", fn_14110a8d0 as *const ()), // sub_14110a8d0
    Replacement::draft(0x14110ac60, "unknown_14110ac60", tpl_1955::<0x160, 0x141109ee0> as *const ()), // sub_14110ac60
    Replacement::draft_throws(0x14110ac90, "unknown_14110ac90", fn_14110ac90 as *const (), Throws::Discard), // sub_14110ac90
    Replacement::draft(0x14110af40, "unknown_14110af40", fn_14110af40 as *const ()), // sub_14110af40
    Replacement::draft(0x14110b170, "coregame_14110b170", tpl_20440::<0x14339ff50, 0x8, 0x3, 0x4, 0x14240aa00, 0x50, 0x98, 0xe0> as *const ()), // coregame::CoreProgramFlow::ActionDebugPageLoadWorldParameters::coregame::CoreProgramFlow::ActionDebugPageLoadWorldParame
    Replacement::draft(0x14110b240, "coregame_14110b240", fn_14110b240 as *const ()), // coregame::DevCommandServer::coregame::DevCommandServer::vfunc_0
    Replacement::draft(0x14110b280, "unknown_14110b280", fn_14110b280 as *const ()), // sub_14110b280
    Replacement::draft(0x14110b430, "unknown_14110b430", tpl_20552::<0x60, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14110b430
    Replacement::draft(0x14110b480, "unknown_14110b480", tpl_20552::<0x48, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14110b480
    Replacement::draft(0x14110b610, "unknown_14110b610", tpl_16373::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1411118d0, 0x140f0c350, 0x1425371a0, 0x142533200> as *const ()), // sub_14110b610
    Replacement::draft(0x14110b810, "unknown_14110b810", fn_14110b810 as *const ()), // sub_14110b810
    Replacement::draft(0x14110bb10, "unknown_14110bb10", fn_14110bb10 as *const ()), // sub_14110bb10
    Replacement::draft_throws(0x14110bbd0, "unknown_14110bbd0", fn_14110bbd0 as *const (), Throws::Caught), // sub_14110bbd0
    Replacement::draft_throws(0x14110c580, "unknown_14110c580", fn_14110c580 as *const (), Throws::Caught), // sub_14110c580
    Replacement::draft_throws(0x14110cf50, "coregame_14110cf50", fn_14110cf50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::updateDecalHiddenStatusToRenderer>
    Replacement::draft_throws(0x14110d260, "coregame_14110d260", fn_14110d260 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::updateDecalHiddenStatusToPhysics>
    Replacement::draft_throws(0x14110d590, "coregame_14110d590", tpl_19279::<0x143cde308, 0x143cde290, 0x1411e9f50, 0x141114970, 0x143cdec90, 0x1420af120, 0x141111560, 0x1420a28e0, 0x141112360, 0x141111120, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a22b6, 0x142646960, 0x143cde2a0, 0x143cde2a8, 0x143cde2ac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::updateFade>
    Replacement::draft_throws(0x14110d880, "coregame_14110d880", tpl_19806::<0x143cde354, 0x143cde3e0, 0x1411e9f50, 0x141114880, 0x143cde368, 0x1420af120, 0x141111560, 0x1420a28e0, 0x141112510, 0x141111200, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a2086, 0x142646960, 0x143cde400, 0x143cde404, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::updateEmissionFade>
    Replacement::draft_throws(0x14110db70, "coregame_14110db70", tpl_19880::<0x143cde268, 0x143cde408, 0x1411e9f50, 0x141114740, 0x143cde524, 0x1420af120, 0x140215750, 0x1410933c0, 0x1420a28e0, 0x1411126c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a1d76, 0x142646960, 0x143cde428, 0x143cde42c, 0x143cde42e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::refreshDynamicDecals>
    Replacement::draft(0x14110dea0, "coregame_14110dea0", tpl_20837::<0x143cdeb8c, 0x143cde530, 0x1411e9f50, 0x141112b30, 0x143cde790, 0x1401fc320, 0x1420a28e0, 0x141112720, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a1cd6, 0x142646960, 0x143cde550, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::decals::destructDecals>
    Replacement::draft_throws(0x14110e140, "coregame_14110e140", tpl_3744::<0x143cde364, 0x143cde810, 0x1411e9f50, 0x141112ab0, 0x143cdeb24, 0x1401fc320, 0x140ff0a40, 0x1420a28e0, 0x1411127b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a1b76, 0x142646960, 0x143cde830, 0x143cde834, 0x143cde836, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::destructDecalPhysics>
    Replacement::draft(0x14110e410, "coregame_14110e410", fn_14110e410 as *const ()), // sub_14110e410
    Replacement::draft(0x14110e4e0, "coregame_14110e4e0", tpl_16383::<0x8, 0x140f0c350> as *const ()), // sub_14110e4e0
    Replacement::draft(0x14110e520, "coregame_14110e520", tpl_1094::<0x143ccbf28, 0x141110810, 0x14020e8f0> as *const ()), // sub_14110e520
    Replacement::draft(0x14110e720, "coregame_14110e720", tpl_1095::<0x143cde9c4, 0x14020e8f0> as *const ()), // sub_14110e720
    Replacement::draft(0x14110e900, "coregame_14110e900", tpl_1094::<0x143cdea38, 0x14068b330, 0x14020e8f0> as *const ()), // sub_14110e900
    Replacement::draft(0x14110eb00, "coregame_14110eb00", tpl_1094::<0x143cde790, 0x1411109a0, 0x14020e8f0> as *const ()), // sub_14110eb00
    Replacement::draft(0x14110ed00, "coregame_14110ed00", tpl_1095::<0x143c9da04, 0x14020e8f0> as *const ()), // sub_14110ed00
    Replacement::draft(0x14110eee0, "coregame_14110eee0", tpl_3270::<0x143cdebcc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14110eee0
    Replacement::draft(0x14110f220, "coregame_14110f220", tpl_3270::<0x143cde8c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14110f220
    Replacement::draft(0x14110f560, "coregame_14110f560", tpl_1094::<0x143cdeb24, 0x141110b60, 0x14020e8f0> as *const ()), // sub_14110f560
    Replacement::draft(0x14110f760, "coregame_14110f760", tpl_21064::<0x8, 0x14110e520, 0x98, 0x140217df0, 0x14110e720> as *const ()), // sub_14110f760
    Replacement::draft(0x14110f7e0, "coregame_14110f7e0", fn_14110f7e0 as *const ()), // sub_14110f7e0
    Replacement::draft_throws(0x14110f9f0, "coregame_14110f9f0", tpl_19662::<0x143cde680, 0x143cde700, 0x141110ce0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a1996, 0x142646960, 0x143cde720, 0x143cde740, 0x143cde750, 0x143cde752, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::DecalComponent>>
    Replacement::draft_throws(0x14110fc20, "coregame_14110fc20", fn_14110fc20 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::streamInDecal>
    Replacement::draft_throws(0x14110ff30, "coregame_14110ff30", tpl_5275::<0x143cde9b8, 0x143cdea10, 0x1411e9f50, 0x141114ea0, 0x143cde378, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x141114250, 0x141111d80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a11c6, 0x142646960, 0x143cdea30, 0x143cdea34, 0x143cdea36, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::streamInDecalPhysics>
    Replacement::draft_throws(0x141110230, "coregame_141110230", tpl_18900::<0x143cde9bc, 0x143cde270, 0x1411e9f50, 0x141115290, 0x143cde9cc, 0x1420af120, 0x1420a28e0, 0x141114400, 0x141111e60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a0f96, 0x142646960, 0x143cde280, 0x143cde288, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::decals::streamOut>
    Replacement::draft(0x141110500, "unknown_141110500", fn_141110500 as *const ()), // sub_141110500
    Replacement::draft(0x141110810, "unknown_141110810", fn_141110810 as *const ()), // sub_141110810
    Replacement::draft(0x1411109a0, "unknown_1411109a0", fn_1411109a0 as *const ()), // sub_1411109a0
    Replacement::draft(0x141110b60, "unknown_141110b60", fn_141110b60 as *const ()), // sub_141110b60
    Replacement::draft_throws(0x141110ce0, "unknown_141110ce0", tpl_3011::<0x1411e9f50, 0x141114fe0, 0x143cde87c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141114030, 0x1401de410> as *const (), Throws::Discard), // sub_141110ce0
    Replacement::draft(0x141110ea0, "unknown_141110ea0", tpl_3203::<0x143c900d8, 0x142407c80, 0x1411117a0, 0x141f91e90> as *const ()), // sub_141110ea0
    Replacement::draft(0x141110fd0, "unknown_141110fd0", tpl_65265::<0x143c900d8, 0x141111640, 0x141f91bd0> as *const ()), // sub_141110fd0
    Replacement::draft(0x141111120, "unknown_141111120", tpl_64022::<0x88, 0x10088, 0x141111460, 0x141f91ad0> as *const ()), // sub_141111120
    Replacement::draft(0x141111200, "unknown_141111200", tpl_64022::<0x88, 0x10088, 0x141111360, 0x141f91a60> as *const ()), // sub_141111200
    Replacement::draft(0x1411112e0, "unknown_1411112e0", fn_1411112e0 as *const ()), // sub_1411112e0
    Replacement::draft_throws(0x141111560, "unknown_141111560", tpl_3146::<0x143cde264, 0x143cdec4c, 0x10, 0x1420a25b0, 0x141834a78, 0x143c904a0, 0x143cdec48, 0x143cdec50, 0x141834a0c> as *const (), Throws::Discard), // sub_141111560
    Replacement::draft(0x1411118d0, "unknown_1411118d0", fn_1411118d0 as *const ()), // sub_1411118d0
    Replacement::draft(0x141111af0, "unknown_141111af0", tpl_5596::<0xc0> as *const ()), // sub_141111af0
    Replacement::draft(0x141111b30, "unknown_141111b30", tpl_20531::<0x7, 0x80, 0x60, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141111b30
    Replacement::draft(0x141111ba0, "unknown_141111ba0", fn_141111ba0 as *const ()), // sub_141111ba0
    Replacement::draft_throws(0x141111be0, "unknown_141111be0", fn_141111be0 as *const (), Throws::Discard), // sub_141111be0
    Replacement::draft(0x141111ce0, "unknown_141111ce0", tpl_3709::<0x143c94b44, 0x20, 0x8, 0x143cde758, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_141111ce0
    Replacement::draft(0x141111d80, "unknown_141111d80", tpl_64022::<0x88, 0x10088, 0x141112f00, 0x141f924d0> as *const ()), // sub_141111d80
    Replacement::draft(0x141111e60, "unknown_141111e60", fn_141111e60 as *const ()), // sub_141111e60
    Replacement::draft(0x141111f30, "unknown_141111f30", tpl_3204::<0x143c900d8, 0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c8d120, 0x143cde790, 0x141f91e90> as *const ()), // sub_141111f30
    Replacement::draft(0x141112360, "unknown_141112360", tpl_64023::<0x143cdebcc, 0x142489ee0, 0x143cde790, 0x141f91ad0> as *const ()), // sub_141112360
    Replacement::draft(0x141112510, "unknown_141112510", tpl_64023::<0x143cde8c0, 0x142489ee0, 0x143cde790, 0x141f91a60> as *const ()), // sub_141112510
    Replacement::draft(0x1411126c0, "unknown_1411126c0", tpl_1217::<0x143cde524, 0x20, 0x8, 0x141f91130> as *const ()), // sub_1411126c0
    Replacement::draft(0x141112720, "unknown_141112720", tpl_19695::<0x143cde790, 0x5e308, 0x10, 0x8, 0x18, 0x141f92100, 0x1411135b0> as *const ()), // sub_141112720
    Replacement::draft(0x1411127b0, "unknown_1411127b0", tpl_6144::<0x143cdeb24, 0x5e308, 0x10, 0x8, 0x18, 0x141f92070, 0x141113820> as *const ()), // sub_1411127b0
    Replacement::draft_throws(0x141112850, "unknown_141112850", tpl_4016::<0x143cde4ac, 0x143cdeb00, 0x143cde2e8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9da04, 0x143c8dbd4, 0x143cde2f8, 0x143cde300, 0x141834a0c> as *const (), Throws::Discard), // sub_141112850
    Replacement::draft(0x141112ab0, "unknown_141112ab0", tpl_3188::<0x141114600, 0x1433a33b8, 0x141113820, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141112ab0
    Replacement::draft(0x141112b30, "unknown_141112b30", tpl_3188::<0x1411146a0, 0x1433a3400, 0x1411135b0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141112b30
    Replacement::draft(0x141112bb0, "unknown_141112bb0", tpl_19620::<0x8, 0x1411135b0> as *const ()), // sub_141112bb0
    Replacement::draft(0x141113410, "unknown_141113410", fn_141113410 as *const ()), // sub_141113410
    Replacement::draft(0x1411135b0, "unknown_1411135b0", fn_1411135b0 as *const ()), // sub_1411135b0
    Replacement::draft(0x1411136e0, "unknown_1411136e0", fn_1411136e0 as *const ()), // sub_1411136e0
    Replacement::draft(0x141113820, "unknown_141113820", fn_141113820 as *const ()), // sub_141113820
    Replacement::draft(0x141113f00, "unknown_141113f00", fn_141113f00 as *const ()), // sub_141113f00
    Replacement::draft(0x141114030, "unknown_141114030", tpl_3028::<0x143cde87c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14110f7e0> as *const ()), // sub_141114030
    Replacement::draft_throws(0x1411140a0, "unknown_1411140a0", fn_1411140a0 as *const (), Throws::Discard), // sub_1411140a0
    Replacement::draft(0x141114250, "unknown_141114250", tpl_64023::<0x143ccbf28, 0x142489ee0, 0x143c8d120, 0x141f924d0> as *const ()), // sub_141114250
    Replacement::draft(0x141114400, "unknown_141114400", fn_141114400 as *const ()), // sub_141114400
    Replacement::draft(0x141114600, "unknown_141114600", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433a33b8, 0x8, 0x20> as *const ()), // sub_141114600
    Replacement::draft(0x1411146a0, "unknown_1411146a0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433a3400, 0x8, 0x20> as *const ()), // sub_1411146a0
    Replacement::draft(0x141114cb0, "unknown_141114cb0", tpl_2030::<0x141114d60, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141114cb0
    Replacement::draft(0x141114d60, "unknown_141114d60", tpl_16124::<0x8, 0x140f0c350> as *const ()), // sub_141114d60
    Replacement::draft(0x141114dd0, "unknown_141114dd0", tpl_6055::<0x143cde6f8, 0x1405400d0, 0x20, 0x141114e50, 0x143cde9d0, 0x1402215b0> as *const ()), // sub_141114dd0
    Replacement::draft(0x141114e50, "unknown_141114e50", tpl_5222::<0x38, 0x20, 0x8, 0x141115480> as *const ()), // sub_141114e50
    Replacement::draft(0x1411150d0, "unknown_1411150d0", fn_1411150d0 as *const ()), // sub_1411150d0
    Replacement::draft(0x1411153d0, "unknown_1411153d0", fn_1411153d0 as *const ()), // sub_1411153d0
    Replacement::draft(0x141115480, "unknown_141115480", tpl_4732::<0x143cde9c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141115480
    Replacement::draft(0x141115ce0, "unknown_141115ce0", fn_141115ce0 as *const ()), // sub_141115ce0
    Replacement::draft(0x141115ea0, "coregame_141115ea0", fn_141115ea0 as *const ()), // coregame::DebugPageLoadAwareState::coregame::DebugPageLoadAwareState::vfunc_0
    Replacement::draft_throws(0x141115ee0, "coregame_141115ee0", tpl_1897::<0x143cded70, 0x143cded80, 0x1411e9f50, 0x141116640, 0x143cded10, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1411163a0, 0x1411161c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a3496, 0x142646960, 0x143cdeda0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::debug::assignDebugName>
    Replacement::draft(0x1411161c0, "unknown_1411161c0", tpl_64022::<0x88, 0x10088, 0x1411162a0, 0x141f95080> as *const ()), // sub_1411161c0
    Replacement::draft(0x1411163a0, "unknown_1411163a0", tpl_64023::<0x143cded78, 0x142489ee0, 0x143c8fa04, 0x141f95080> as *const ()), // sub_1411163a0
    Replacement::draft(0x1411167c0, "coregame_1411167c0", fn_1411167c0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::debug_draw::beginFixedUpdate>
    Replacement::draft(0x141116a30, "coregame_141116a30", fn_141116a30 as *const ()), // ecs::system::ecs::system::wrapper<coregame::debug_draw::endFixedUpdate>
    Replacement::draft_throws(0x141116cf0, "coregame_141116cf0", tpl_5993::<0x143cdee1c, 0x143cdee20, 0x1411e9f50, 0x141118150, 0x143cdee50, 0x1420af120, 0x1410da840, 0x1420a28e0, 0x1411178f0, 0x141117860, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a3ae6, 0x142646960, 0x143cdee40, 0x143cdee48, 0x143cdee4a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dcclivecamera::initDCCLiveCamera>
    Replacement::draft_throws(0x141116ff0, "coregame_141116ff0", fn_141116ff0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dcclivecamera::dccLiveCameraRemoved>
    Replacement::draft(0x1411172c0, "unknown_1411172c0", tpl_1403::<0x143cdef14, 0x1411176e0, 0x14020e8f0> as *const ()), // sub_1411172c0
    Replacement::draft(0x1411174a0, "unknown_1411174a0", tpl_1095::<0x143cdef08, 0x14020e8f0> as *const ()), // sub_1411174a0
    Replacement::draft(0x141117680, "unknown_141117680", tpl_21089::<0x8, 0x1411172c0, 0x1411174a0> as *const ()), // sub_141117680
    Replacement::draft(0x1411176e0, "unknown_1411176e0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1432876a0> as *const ()), // sub_1411176e0
    Replacement::draft(0x141117860, "unknown_141117860", tpl_1898::<0x88, 0x10088, 0x141f95ea0> as *const ()), // sub_141117860
    Replacement::draft(0x1411178f0, "unknown_1411178f0", tpl_61552::<0x8, 0x88, 0x10088, 0x141f95ea0> as *const ()), // sub_1411178f0
    Replacement::draft(0x1411179e0, "unknown_1411179e0", tpl_6144::<0x143cdef14, 0x5e308, 0x10, 0x8, 0x18, 0x141f95df0, 0x141117dc0> as *const ()), // sub_1411179e0
    Replacement::draft(0x141117a80, "unknown_141117a80", tpl_3188::<0x1411180b0, 0x1433a3ef0, 0x141117dc0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141117a80
    Replacement::draft(0x141117c90, "unknown_141117c90", fn_141117c90 as *const ()), // sub_141117c90
    Replacement::draft(0x141117dc0, "unknown_141117dc0", fn_141117dc0 as *const ()), // sub_141117dc0
    Replacement::draft(0x1411180b0, "unknown_1411180b0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433a3ef0, 0x8, 0x20> as *const ()), // sub_1411180b0
    Replacement::draft(0x141118290, "unknown_141118290", fn_141118290 as *const ()), // sub_141118290
    Replacement::draft(0x141118340, "unknown_141118340", fn_141118340 as *const ()), // sub_141118340
    Replacement::draft_throws(0x141118450, "unknown_141118450", fn_141118450 as *const (), Throws::Caught), // sub_141118450
    Replacement::draft(0x141118c00, "unknown_141118c00", fn_141118c00 as *const ()), // sub_141118c00
    Replacement::draft_throws(0x141118cf0, "unknown_141118cf0", fn_141118cf0 as *const (), Throws::Caught), // sub_141118cf0
    Replacement::draft(0x1411194a0, "unknown_1411194a0", fn_1411194a0 as *const ()), // sub_1411194a0
    Replacement::draft_throws(0x141119880, "unknown_141119880", fn_141119880 as *const (), Throws::Caught), // sub_141119880
    Replacement::draft(0x14111a030, "unknown_14111a030", fn_14111a030 as *const ()), // sub_14111a030
    Replacement::draft(0x14111a0d0, "unknown_14111a0d0", fn_14111a0d0 as *const ()), // sub_14111a0d0
    Replacement::draft(0x14111a1b0, "unknown_14111a1b0", tpl_19999::<0x14, 0x20, 0x143a08300, 0x14248b0c0, 0x18, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x10, 0x1401ea4e0, 0x142442c80, 0x8, 0x142442de0> as *const ()), // sub_14111a1b0
    Replacement::draft(0x14111a280, "unknown_14111a280", fn_14111a280 as *const ()), // sub_14111a280
    Replacement::draft(0x14111a350, "coregame_14111a350", tpl_790::<0x141f97800, 0x141834d30> as *const ()), // coregame::ContentBlockLoader::coregame::ContentBlockLoader::vfunc_0
    Replacement::draft_throws(0x14111a390, "unknown_14111a390", fn_14111a390 as *const (), Throws::Discard), // sub_14111a390
    Replacement::draft(0x14111a4e0, "unknown_14111a4e0", fn_14111a4e0 as *const ()), // sub_14111a4e0
    Replacement::draft(0x14111a680, "unknown_14111a680", fn_14111a680 as *const ()), // sub_14111a680
    Replacement::draft(0x14111a790, "unknown_14111a790", tpl_60956::<0x19, 0x10, 0x14111a790, 0x40, 0x140260f30, 0x1401e66f0> as *const ()), // sub_14111a790
    Replacement::draft(0x14111a850, "unknown_14111a850", tpl_3943::<0x30, 0x143c97be4, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14111a8d3> as *const ()), // sub_14111a850
    Replacement::draft(0x14111a8e0, "unknown_14111a8e0", fn_14111a8e0 as *const ()), // sub_14111a8e0
    Replacement::draft_throws(0x14111a910, "unknown_14111a910", fn_14111a910 as *const (), Throws::Caught), // sub_14111a910
    Replacement::draft_throws(0x14111a9a0, "unknown_14111a9a0", fn_14111a9a0 as *const (), Throws::Caught), // sub_14111a9a0
    Replacement::draft_throws(0x14111abb0, "unknown_14111abb0", fn_14111abb0 as *const (), Throws::Discard), // sub_14111abb0
    Replacement::draft(0x14111ae60, "unknown_14111ae60", fn_14111ae60 as *const ()), // sub_14111ae60
    Replacement::draft(0x14111afd0, "unknown_14111afd0", fn_14111afd0 as *const ()), // sub_14111afd0
    Replacement::draft_throws(0x14111b040, "std_14111b040", fn_14111b040 as *const (), Throws::Discard), // sub_14111b040
    Replacement::draft(0x14111b200, "unknown_14111b200", fn_14111b200 as *const ()), // sub_14111b200
    Replacement::draft(0x14111b230, "unknown_14111b230", tpl_365::<0x14111b420> as *const ()), // sub_14111b230
    Replacement::draft(0x14111b240, "unknown_14111b240", fn_14111b240 as *const ()), // sub_14111b240
    Replacement::draft(0x14111b270, "unknown_14111b270", tpl_170::<0x14111d5b0> as *const ()), // sub_14111b270
    Replacement::draft(0x14111b280, "unknown_14111b280", tpl_61657::<0x8, 0x10, 0x14111b420, 0x1401e66f0> as *const ()), // sub_14111b280
    Replacement::draft(0x14111b2f0, "unknown_14111b2f0", fn_14111b2f0 as *const ()), // sub_14111b2f0
    Replacement::draft(0x14111b420, "unknown_14111b420", fn_14111b420 as *const ()), // sub_14111b420
    Replacement::draft(0x14111b4b0, "unknown_14111b4b0", tpl_63157::<0x143c8f7a0, 0x142489ee0, 0x143c9c6e4, 0x143c9c7b0> as *const ()), // sub_14111b4b0
    Replacement::draft(0x14111b5f0, "unknown_14111b5f0", fn_14111b5f0 as *const ()), // sub_14111b5f0
    Replacement::draft(0x14111b610, "unknown_14111b610", fn_14111b610 as *const ()), // sub_14111b610
    Replacement::draft(0x14111b690, "unknown_14111b690", tpl_1171::<0x58, 0x60, 0x143c9c7b0, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14111b690
    Replacement::draft(0x14111b770, "unknown_14111b770", fn_14111b770 as *const ()), // sub_14111b770
    Replacement::draft_throws(0x14111b8b0, "unknown_14111b8b0", fn_14111b8b0 as *const (), Throws::Discard), // sub_14111b8b0
    Replacement::draft(0x14111b980, "unknown_14111b980", fn_14111b980 as *const ()), // sub_14111b980
    Replacement::draft(0x14111ba10, "unknown_14111ba10", tpl_61197::<0x143c8f7a0, 0x142489ee0, 0x143c900a8, 0x143c9c7b0, 0x143c9c6e4, 0x1426467e8, 0x14111bba3> as *const ()), // sub_14111ba10
    Replacement::draft(0x14111bbb0, "coregame_14111bbb0", fn_14111bbb0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::advanceCoherentTextureViewModule>
    Replacement::draft(0x14111be60, "coregame_14111be60", fn_14111be60 as *const ()), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::cleanupCoherentTextureViews>
    Replacement::draft_throws(0x14111c120, "coregame_14111c120", tpl_1480::<0x1429371b0, 0x1410d6ca0, 0x14111dea0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14111c120
    Replacement::draft_throws(0x14111c3c0, "coregame_14111c3c0", tpl_1480::<0x1429371b0, 0x1410d6ca0, 0x14111e0c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14111c3c0
    Replacement::draft(0x14111c660, "coregame_14111c660", fn_14111c660 as *const ()), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::handleMouseEvents>
    Replacement::draft_throws(0x14111c900, "coregame_14111c900", tpl_1480::<0x1429371c0, 0x1410d6ca0, 0x14111e3b0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14111c900
    Replacement::draft_throws(0x14111cba0, "coregame_14111cba0", tpl_1480::<0x1429371c0, 0x1410d6ca0, 0x14111e5c0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14111cba0
    Replacement::draft_throws(0x14111ce40, "coregame_14111ce40", tpl_1480::<0x1429371c0, 0x1410d6ca0, 0x14111e7e0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14111ce40
    Replacement::draft(0x14111d0e0, "coregame_14111d0e0", fn_14111d0e0 as *const ()), // sub_14111d0e0
    Replacement::draft(0x14111d240, "coregame_14111d240", fn_14111d240 as *const ()), // sub_14111d240
    Replacement::draft(0x14111d340, "coregame_14111d340", fn_14111d340 as *const ()), // sub_14111d340
    Replacement::draft(0x14111d440, "coregame_14111d440", fn_14111d440 as *const ()), // sub_14111d440
    Replacement::draft(0x14111d5b0, "coregame_14111d5b0", fn_14111d5b0 as *const ()), // sub_14111d5b0
    Replacement::draft(0x14111d690, "coregame_14111d690", fn_14111d690 as *const ()), // sub_14111d690
    Replacement::draft(0x14111d890, "coregame_14111d890", tpl_1094::<0x143c9c7b0, 0x140492c90, 0x14020e8f0> as *const ()), // sub_14111d890
    Replacement::draft_throws(0x14111da90, "coregame_14111da90", fn_14111da90 as *const (), Throws::Caught), // sub_14111da90
    Replacement::draft_throws(0x14111dea0, "coregame_14111dea0", fn_14111dea0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::checkCoherentTextureViewLifetime>
    Replacement::draft_throws(0x14111e0c0, "coregame_14111e0c0", fn_14111e0c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::destructMeshPrimitiveTextureOverrides>
    Replacement::draft_throws(0x14111e3b0, "coregame_14111e3b0", tpl_6402::<0x143cdf0c0, 0x143cdf258, 0x14111ef10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a4e16, 0x142646960, 0x143cdf278, 0x143cdf288, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::handleLuaRequests>
    Replacement::draft_throws(0x14111e5c0, "coregame_14111e5c0", tpl_4143::<0x143cdf0b4, 0x143cdf070, 0x14111f0c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a4be6, 0x142646960, 0x143cdf090, 0x143cdf0a0, 0x143cdf0a4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::setTextureOverrides>
    Replacement::draft_throws(0x14111e7e0, "coregame_14111e7e0", tpl_3369::<0x143cdf06c, 0x143cdf1d0, 0x1411e9f50, 0x14111f920, 0x143cdf068, 0x1420af120, 0x1420a28e0, 0x14111f770, 0x14111f2d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a4a26, 0x142646960, 0x143cdf1f0, 0x143cdf200, 0x143cdf208, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::coherent_texture_views::cleanBindingsAndOverrides>
    Replacement::draft(0x14111ead0, "unknown_14111ead0", fn_14111ead0 as *const ()), // sub_14111ead0
    Replacement::draft(0x14111ec20, "unknown_14111ec20", fn_14111ec20 as *const ()), // sub_14111ec20
    Replacement::draft(0x14111ed40, "std_14111ed40", tpl_1::<0x143890c10> as *const ()), // std::_Func_impl_no_alloc<bool (*)(const coregame::coherent_texture_views::CoherentTextureView&,const unsigned int&),bool
    Replacement::draft(0x14111ed50, "std_14111ed50", tpl_550::<0x1433a51e0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bool (*)(const coregame::coherent_texture_views::CoherentTextureView&,const unsigned int&),bool
    Replacement::draft_throws(0x14111ed70, "unknown_14111ed70", fn_14111ed70 as *const (), Throws::Discard), // sub_14111ed70
    Replacement::draft_throws(0x14111ef10, "unknown_14111ef10", fn_14111ef10 as *const (), Throws::Discard), // sub_14111ef10
    Replacement::draft_throws(0x14111f0c0, "unknown_14111f0c0", tpl_3697::<0x1411e9f50, 0x141120060, 0x143cdf0b0, 0x1420af120, 0x140215750, 0x143c9c824, 0x140215610, 0x1420a28e0, 0x14111f700, 0x1401de410> as *const (), Throws::Discard), // sub_14111f0c0
    Replacement::draft(0x14111f270, "unknown_14111f270", fn_14111f270 as *const ()), // sub_14111f270
    Replacement::draft(0x14111f290, "unknown_14111f290", fn_14111f290 as *const ()), // sub_14111f290
    Replacement::draft_throws(0x14111f2b0, "unknown_14111f2b0", tpl_2973::<0x143c9c824, 0x580e0, 0x141f98ba0> as *const (), Throws::Discard), // sub_14111f2b0
    Replacement::draft(0x14111f2d0, "unknown_14111f2d0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x14111f490, 0x580f0, 0x141f99700> as *const ()), // sub_14111f2d0
    Replacement::draft(0x14111f3c0, "unknown_14111f3c0", fn_14111f3c0 as *const ()), // sub_14111f3c0
    Replacement::draft(0x14111f490, "unknown_14111f490", tpl_3492::<0x143c8f7a0, 0x142489ee0, 0x143c9c7b0> as *const ()), // sub_14111f490
    Replacement::draft(0x14111f590, "unknown_14111f590", fn_14111f590 as *const ()), // sub_14111f590
    Replacement::draft(0x14111f5c0, "unknown_14111f5c0", tpl_6144::<0x143c9c7b0, 0x5e308, 0x10, 0x8, 0x18, 0x141f98ef0, 0x14111fe50> as *const ()), // sub_14111f5c0
    Replacement::draft_throws(0x14111f660, "unknown_14111f660", fn_14111f660 as *const (), Throws::Caught), // sub_14111f660
    Replacement::draft(0x14111f700, "unknown_14111f700", tpl_3028::<0x143cdf0b0, 0x20, 0x8, 0x143c9c824, 0x580e0, 0x141f99840> as *const ()), // sub_14111f700
    Replacement::draft(0x14111f770, "unknown_14111f770", tpl_3213::<0x142407c80, 0x143c8f7a0, 0x142489ee0, 0x143c9c7b0, 0x141f99700> as *const ()), // sub_14111f770
    Replacement::draft(0x14111f920, "unknown_14111f920", tpl_5896::<0x143cdf248, 0x143cdf148, 0x8, 0x10, 0x18, 0x141834a78, 0x143c8f7a0, 0x143c9c7b0, 0x143cdf158, 0x143cdf160, 0x141834a0c> as *const ()), // sub_14111f920
    Replacement::draft(0x14111fa10, "unknown_14111fa10", tpl_3188::<0x141120170, 0x1433a5218, 0x14111fe50, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14111fa10
    Replacement::draft(0x14111fcb0, "unknown_14111fcb0", fn_14111fcb0 as *const ()), // sub_14111fcb0
    Replacement::draft(0x14111fe50, "unknown_14111fe50", fn_14111fe50 as *const ()), // sub_14111fe50
    Replacement::draft_throws(0x14111ff70, "unknown_14111ff70", fn_14111ff70 as *const (), Throws::Discard), // sub_14111ff70
    Replacement::draft(0x141120170, "unknown_141120170", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433a5218, 0x8, 0x20> as *const ()), // sub_141120170
    Replacement::draft(0x141120210, "unknown_141120210", fn_141120210 as *const ()), // sub_141120210
    Replacement::draft(0x141120400, "unknown_141120400", fn_141120400 as *const ()), // sub_141120400
    Replacement::draft(0x141120630, "unknown_141120630", fn_141120630 as *const ()), // sub_141120630
    Replacement::draft(0x141120690, "unknown_141120690", fn_141120690 as *const ()), // sub_141120690
    Replacement::draft(0x141120760, "unknown_141120760", fn_141120760 as *const ()), // sub_141120760
    Replacement::draft(0x141120a70, "unknown_141120a70", fn_141120a70 as *const ()), // sub_141120a70
    Replacement::draft(0x141120ba0, "unknown_141120ba0", fn_141120ba0 as *const ()), // sub_141120ba0
    Replacement::draft(0x141120f40, "unknown_141120f40", fn_141120f40 as *const ()), // sub_141120f40
    Replacement::draft(0x1411213e0, "unknown_1411213e0", fn_1411213e0 as *const ()), // sub_1411213e0
    Replacement::draft(0x1411214a0, "unknown_1411214a0", fn_1411214a0 as *const ()), // sub_1411214a0
    Replacement::draft(0x1411217d0, "unknown_1411217d0", fn_1411217d0 as *const ()), // sub_1411217d0
    Replacement::draft(0x141121ad0, "unknown_141121ad0", fn_141121ad0 as *const ()), // sub_141121ad0
    Replacement::draft_throws(0x141121bd0, "unknown_141121bd0", fn_141121bd0 as *const (), Throws::Caught), // sub_141121bd0
    Replacement::draft_throws(0x141121db0, "unknown_141121db0", fn_141121db0 as *const (), Throws::Discard), // sub_141121db0
    Replacement::draft_throws(0x141121fc0, "unknown_141121fc0", fn_141121fc0 as *const (), Throws::Discard), // sub_141121fc0
    Replacement::draft(0x1411221a0, "coregame_1411221a0", fn_1411221a0 as *const ()), // coregame::CmdLineParser::coregame::CmdLineParser::vfunc_0
    Replacement::draft(0x1411221e0, "unknown_1411221e0", fn_1411221e0 as *const ()), // sub_1411221e0
    Replacement::draft(0x141122220, "unknown_141122220", fn_141122220 as *const ()), // sub_141122220
    Replacement::draft(0x141122270, "unknown_141122270", fn_141122270 as *const ()), // sub_141122270
    Replacement::draft(0x1411222d0, "unknown_1411222d0", fn_1411222d0 as *const ()), // sub_1411222d0
    Replacement::draft(0x141122300, "std_141122300", tpl_1::<0x143890ca8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_25_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122340, "std_141122340", tpl_550::<0x1433a6770, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_25_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122360, "std_141122360", tpl_1::<0x143890c68> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_24_>,bool,std::basic_string_view<char,r::CharTraits<char>>,boo
    Replacement::draft(0x141122370, "std_141122370", fn_141122370 as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_24_>,bool,std::basic_string_view<char,r::CharTraits<char>>,boo
    Replacement::draft(0x141122390, "std_141122390", tpl_550::<0x1433a6700, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_24_>,bool,std::basic_string_view<char,r::CharTraits<char>>,boo
    Replacement::draft(0x1411223b0, "std_1411223b0", tpl_1::<0x143890da8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_23_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x1411223f0, "std_1411223f0", tpl_550::<0x1433a67a8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_23_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122410, "std_141122410", tpl_1::<0x143890de8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_22_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122450, "std_141122450", tpl_550::<0x1433a6738, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_22_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122470, "std_141122470", tpl_1::<0x143890d28> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_21_>,bool,std::basic_string_view<char,r::CharTraits<char>>,boo
    Replacement::draft_throws(0x141122480, "std_141122480", fn_141122480 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_21_>,bool,std::basic_string_view<char,r::CharTraits<char>>,boo
    Replacement::draft(0x1411225a0, "std_1411225a0", tpl_550::<0x1433a6818, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_21_>,bool,std::basic_string_view<char,r::CharTraits<char>>,boo
    Replacement::draft(0x1411225c0, "std_1411225c0", tpl_1::<0x143890ce8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_20_>,bool,std::basic_string_view<char,r::CharTraits<char>>,int
    Replacement::draft_throws(0x1411225d0, "std_1411225d0", fn_1411225d0 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_20_>,bool,std::basic_string_view<char,r::CharTraits<char>>,int
    Replacement::draft(0x141122650, "std_141122650", tpl_550::<0x1433a67e0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_20_>,bool,std::basic_string_view<char,r::CharTraits<char>>,int
    Replacement::draft(0x141122670, "std_141122670", tpl_1::<0x143890d68> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_19_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122680, "std_141122680", fn_141122680 as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_19_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122780, "std_141122780", tpl_770::<0x1433a6850> as *const ()), // std::_Func_impl_no_alloc<coregame::CmdLineParser::<lambda_19_>,bool,std::basic_string_view<char,r::CharTraits<char>>,std
    Replacement::draft(0x141122790, "unknown_141122790", fn_141122790 as *const ()), // sub_141122790
    Replacement::draft(0x1411227d0, "unknown_1411227d0", fn_1411227d0 as *const ()), // sub_1411227d0
    Replacement::draft(0x1411229b0, "unknown_1411229b0", fn_1411229b0 as *const ()), // sub_1411229b0
    Replacement::draft_throws(0x141122b30, "unknown_141122b30", tpl_1480::<0x14293c280, 0x1410d6ca0, 0x141124320, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141122b30
    Replacement::draft_throws(0x141122dd0, "unknown_141122dd0", tpl_1480::<0x14293c280, 0x1410d6ca0, 0x141124600, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141122dd0
    Replacement::draft_throws(0x141123070, "unknown_141123070", tpl_1480::<0x14293c280, 0x1410d6ca0, 0x1411248f0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141123070
    Replacement::draft_throws(0x141123310, "coregame_141123310", fn_141123310 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth_render::overrideSkinningMatrices>
    Replacement::draft(0x141123610, "coregame_141123610", tpl_16556::<0x8, 0x10, 0x1401f2bc0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141123610
    Replacement::draft(0x1411236e0, "coregame_1411236e0", tpl_1094::<0x143cdf8e8, 0x141125470, 0x14020e8f0> as *const ()), // sub_1411236e0
    Replacement::draft(0x1411238e0, "coregame_1411238e0", tpl_1369::<0x143cdf8e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411238e0
    Replacement::draft(0x141123c20, "coregame_141123c20", tpl_1094::<0x143cdf830, 0x141125610, 0x14020e8f0> as *const ()), // sub_141123c20
    Replacement::draft(0x141123e20, "coregame_141123e20", tpl_1094::<0x143cd444c, 0x141045c20, 0x14020e8f0> as *const ()), // sub_141123e20
    Replacement::draft(0x141124020, "coregame_141124020", tpl_6012::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0xc, 0x1433a69d8> as *const ()), // sub_141124020
    Replacement::draft(0x1411240f0, "coregame_1411240f0", tpl_1094::<0x143cdf4d8, 0x1411257b0, 0x14020e8f0> as *const ()), // sub_1411240f0
    Replacement::draft(0x1411242f0, "coregame_1411242f0", tpl_1439::<0x8, 0x1411240f0> as *const ()), // sub_1411242f0
    Replacement::draft_throws(0x141124320, "coregame_141124320", tpl_1078::<0x143cdf340, 0x143cdf838, 0x1411e9f50, 0x141127ff0, 0x143cdf4e0, 0x1420af120, 0x1420a28e0, 0x141126ea0, 0x141125e40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a6e76, 0x142646960, 0x143cdf858, 0x143cdf85a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth_render::remapBones>
    Replacement::draft_throws(0x141124600, "coregame_141124600", tpl_3407::<0x143cdf8ec, 0x143cdf6c8, 0x1411e9f50, 0x141127eb0, 0x143cdf4d0, 0x1420af120, 0x1420a28e0, 0x141127080, 0x141126020, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a7386, 0x142646960, 0x143cdf6e8, 0x143cdf6f8, 0x143cdf6fc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth_render::calculateFixedUpdateMatrices>
    Replacement::draft_throws(0x1411248f0, "coregame_1411248f0", tpl_3393::<0x143cdf36c, 0x143cdf7f0, 0x1411e9f50, 0x141127d30, 0x143cdf4dc, 0x1420af120, 0x1420a28e0, 0x141127220, 0x141126150, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a7636, 0x142646960, 0x143cdf810, 0x143cdf820, 0x143cdf828, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth_render::registerBonesToGeometryTransform>
    Replacement::draft_throws(0x141124be0, "coregame_141124be0", tpl_1897::<0x143cdf76c, 0x143cdf348, 0x1411e9f50, 0x141127bb0, 0x143cdf508, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141127420, 0x1411262c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a7956, 0x142646960, 0x143cdf368, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth_render::streamIn>
    Replacement::draft_throws(0x141124ec0, "coregame_141124ec0", tpl_4828::<0x143cdf500, 0x143cdf6a0, 0x1411e9f50, 0x141127a80, 0x143cdf7a8, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141127510, 0x141126350, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433a7cc6, 0x142646960, 0x143cdf6c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth_render::streamOut>
    Replacement::draft(0x1411251b0, "std_1411251b0", tpl_6019::<0x1433a69d8, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<r::DynamicArray<rend::BoneMatrixOverride,r::Allocator<rend::BoneMatrixOverride,0>>,r::Allocat
    Replacement::draft(0x1411251e0, "unknown_1411251e0", fn_1411251e0 as *const ()), // sub_1411251e0
    Replacement::draft(0x141125370, "unknown_141125370", tpl_16563::<0x10, 0x1401f2bc0> as *const ()), // sub_141125370
    Replacement::draft(0x1411253b0, "unknown_1411253b0", fn_1411253b0 as *const ()), // sub_1411253b0
    Replacement::draft(0x141125470, "unknown_141125470", tpl_4329::<0x1401e6d50, 0x141125de0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_141125470
    Replacement::draft(0x141125610, "unknown_141125610", tpl_19665::<0x1401e6d50, 0x141125e10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_141125610
    Replacement::draft(0x1411257b0, "unknown_1411257b0", fn_1411257b0 as *const ()), // sub_1411257b0
    Replacement::draft(0x1411259b0, "unknown_1411259b0", fn_1411259b0 as *const ()), // sub_1411259b0
    Replacement::draft(0x141125a60, "unknown_141125a60", tpl_5033::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141fa23c0> as *const ()), // sub_141125a60
    Replacement::draft(0x141125de0, "unknown_141125de0", tpl_1955::<0x18, 0x140d4aa80> as *const ()), // sub_141125de0
    Replacement::draft(0x141125e10, "unknown_141125e10", tpl_1955::<0x18, 0x141123610> as *const ()), // sub_141125e10
    Replacement::draft_throws(0x141125e40, "unknown_141125e40", tpl_21168::<0x40, 0x48, 0x10048, 0x141126b60, 0x141125f40> as *const (), Throws::Discard), // sub_141125e40
    Replacement::draft_throws(0x141125f40, "unknown_141125f40", fn_141125f40 as *const (), Throws::Discard), // sub_141125f40
    Replacement::draft(0x141126020, "unknown_141126020", tpl_62531::<0x88, 0x10088, 0x141126a00, 0x141fa2800> as *const ()), // sub_141126020
    Replacement::draft(0x141126150, "unknown_141126150", fn_141126150 as *const ()), // sub_141126150
    Replacement::draft(0x1411261e0, "unknown_1411261e0", fn_1411261e0 as *const ()), // sub_1411261e0
    Replacement::draft(0x1411262c0, "unknown_1411262c0", tpl_1898::<0x88, 0x10088, 0x141fa1fd0> as *const ()), // sub_1411262c0
    Replacement::draft(0x141126350, "unknown_141126350", tpl_1898::<0x88, 0x10088, 0x141fa1c90> as *const ()), // sub_141126350
    Replacement::draft(0x1411263e0, "unknown_1411263e0", fn_1411263e0 as *const ()), // sub_1411263e0
    Replacement::draft_throws(0x141126ea0, "unknown_141126ea0", tpl_79502::<0x141126b60, 0x141fa3380> as *const (), Throws::Discard), // sub_141126ea0
    Replacement::draft(0x141127080, "unknown_141127080", tpl_62567::<0x141126a00, 0x141fa2800> as *const ()), // sub_141127080
    Replacement::draft(0x141127220, "unknown_141127220", fn_141127220 as *const ()), // sub_141127220
    Replacement::draft(0x141127420, "unknown_141127420", tpl_61552::<0x8, 0x88, 0x10088, 0x141fa1fd0> as *const ()), // sub_141127420
    Replacement::draft(0x141127510, "unknown_141127510", tpl_61552::<0x8, 0x88, 0x10088, 0x141fa1c90> as *const ()), // sub_141127510
    Replacement::draft_throws(0x141127600, "unknown_141127600", fn_141127600 as *const (), Throws::Discard), // sub_141127600
    Replacement::draft_throws(0x1411277a0, "unknown_1411277a0", tpl_2918::<0x143cdf85c, 0x143cdf8e0, 0x143cdf4e8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cdf830, 0x141834a0c> as *const (), Throws::Discard), // sub_1411277a0
    Replacement::draft(0x1411278d0, "unknown_1411278d0", fn_1411278d0 as *const ()), // sub_1411278d0
    Replacement::draft(0x141128490, "unknown_141128490", fn_141128490 as *const ()), // sub_141128490
    Replacement::draft_throws(0x1411286c0, "unknown_1411286c0", fn_1411286c0 as *const (), Throws::Discard), // sub_1411286c0
    Replacement::draft(0x141128b50, "unknown_141128b50", tpl_5510::<0x143cca0fc, 0x142489ee0, 0x143c8ec20, 0x143ce0238> as *const ()), // sub_141128b50
    Replacement::draft(0x141128e50, "unknown_141128e50", fn_141128e50 as *const ()), // sub_141128e50
    Replacement::draft(0x141129140, "unknown_141129140", fn_141129140 as *const ()), // sub_141129140
    Replacement::draft_throws(0x141129420, "unknown_141129420", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112c2a0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141129420
    Replacement::draft_throws(0x1411296c0, "unknown_1411296c0", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112c690, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_1411296c0
    Replacement::draft_throws(0x141129960, "unknown_141129960", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112c960, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141129960
    Replacement::draft_throws(0x141129c00, "unknown_141129c00", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112cc30, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141129c00
    Replacement::draft_throws(0x141129ea0, "unknown_141129ea0", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112cf10, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_141129ea0
    Replacement::draft_throws(0x14112a140, "unknown_14112a140", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112d210, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14112a140
    Replacement::draft_throws(0x14112a3e0, "unknown_14112a3e0", tpl_1480::<0x14293cd00, 0x1410d6ca0, 0x14112d4f0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14112a3e0
    Replacement::draft_throws(0x14112a680, "coregame_14112a680", tpl_5428::<0x143cdfcb8, 0x143ce0160, 0x14112d6f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aaeb6, 0x142646960, 0x143ce0180, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::gatherResources>
    Replacement::draft(0x14112a880, "coregame_14112a880", fn_14112a880 as *const ()), // sub_14112a880
    Replacement::draft(0x14112ab50, "coregame_14112ab50", fn_14112ab50 as *const ()), // sub_14112ab50
    Replacement::draft(0x14112ad50, "coregame_14112ad50", fn_14112ad50 as *const ()), // sub_14112ad50
    Replacement::draft(0x14112aef0, "coregame_14112aef0", fn_14112aef0 as *const ()), // sub_14112aef0
    Replacement::draft(0x14112b350, "coregame_14112b350", tpl_1094::<0x143cdfc70, 0x140fdd500, 0x14020e8f0> as *const ()), // sub_14112b350
    Replacement::draft(0x14112b550, "coregame_14112b550", tpl_1368::<0x143ce0188, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14112b550
    Replacement::draft(0x14112b880, "coregame_14112b880", tpl_1094::<0x143c9a324, 0x140467f50, 0x14020e8f0> as *const ()), // sub_14112b880
    Replacement::draft(0x14112ba80, "coregame_14112ba80", tpl_3073::<0x143cdfcd4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14112ba80
    Replacement::draft(0x14112bde0, "coregame_14112bde0", tpl_1094::<0x143ce0238, 0x14112f300, 0x14020e8f0> as *const ()), // sub_14112bde0
    Replacement::draft(0x14112bfe0, "coregame_14112bfe0", tpl_1094::<0x143ce0070, 0x14112f4a0, 0x14020e8f0> as *const ()), // sub_14112bfe0
    Replacement::draft(0x14112c240, "coregame_14112c240", tpl_1439::<0x8, 0x14112bde0> as *const ()), // sub_14112c240
    Replacement::draft(0x14112c270, "coregame_14112c270", tpl_1439::<0x8, 0x14112bfe0> as *const ()), // sub_14112c270
    Replacement::draft_throws(0x14112c2a0, "coregame_14112c2a0", fn_14112c2a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::simulateGroup>
    Replacement::draft_throws(0x14112c690, "coregame_14112c690", tpl_21182::<0x143ce0030, 0x143ce01c8, 0x1411e9f50, 0x1411334a0, 0x143cdff38, 0x1420af120, 0x1420a28e0, 0x141131cb0, 0x141130080, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ab5c6, 0x142646960, 0x143ce01e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::deleteInstances>
    Replacement::draft_throws(0x14112c960, "coregame_14112c960", tpl_21182::<0x143ce00e8, 0x143cdfdf0, 0x1411e9f50, 0x1411332e0, 0x143cdfcbc, 0x1420af120, 0x1420a28e0, 0x141131f70, 0x141130410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aa1c6, 0x142646960, 0x143cdfe10, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::createInstances>
    Replacement::draft_throws(0x14112cc30, "coregame_14112cc30", tpl_1467::<0x143cdfc28, 0x143cdfa08, 0x1411e9f50, 0x1411331d0, 0x143cdfeec, 0x1420af120, 0x1420a28e0, 0x141132190, 0x141130660, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aa686, 0x142646960, 0x143cdfa28, 0x143cdfa30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::updateClothParamsFromBones>
    Replacement::draft_throws(0x14112cf10, "coregame_14112cf10", tpl_4834::<0x143cdfd48, 0x143cdff90, 0x1411e9f50, 0x141133050, 0x143ce01fc, 0x1420af120, 0x14036fad0, 0x1420a28e0, 0x141132300, 0x141130760, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aa866, 0x142646960, 0x143cdffb0, 0x143cdffb8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::updateBoneAttachedWindArea>
    Replacement::draft_throws(0x14112d210, "coregame_14112d210", tpl_1078::<0x143cdfc78, 0x143cdff40, 0x1411e9f50, 0x141132f10, 0x143cdfbcc, 0x1420af120, 0x1420a28e0, 0x141132510, 0x141130940, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aac26, 0x142646960, 0x143cdff60, 0x143cdff62, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::applyClothProfile>
    Replacement::draft_throws(0x14112d4f0, "coregame_14112d4f0", tpl_5428::<0x143cdfbdc, 0x143ce0278, 0x14112f6a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433abc16, 0x142646960, 0x143ce0298, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::gatherWindAreas>
    Replacement::draft_throws(0x14112d6f0, "coregame_14112d6f0", tpl_3011::<0x1411e9f50, 0x141132b40, 0x143cdfbc0, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141130b20, 0x1401de410> as *const (), Throws::Discard), // sub_14112d6f0
    Replacement::draft_throws(0x14112d8b0, "coregame_14112d8b0", tpl_19637::<0x143cdfb8c, 0x143cdff70, 0x1411e9f50, 0x141132d20, 0x143cdfcc8, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141132750, 0x141130b90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433abe46, 0x142646960, 0x143cdff80, 0x143cdff88, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::streamIn>
    Replacement::draft_throws(0x14112dba0, "coregame_14112dba0", tpl_19638::<0x143cdfe98, 0x143cdfe18, 0x1411e9f50, 0x141132c30, 0x143cdfbbc, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1411328a0, 0x141130c90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ac076, 0x142646960, 0x143cdfe28, 0x143cdfe30, 0x143cdfe32, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cloth::streamOut>
    Replacement::draft(0x14112dea0, "unknown_14112dea0", tpl_1094::<0x143cd9c68, 0x14112f8d0, 0x14020e8f0> as *const ()), // sub_14112dea0
    Replacement::draft(0x14112e0a0, "unknown_14112e0a0", tpl_1094::<0x143ce00ec, 0x14112fa60, 0x14020e8f0> as *const ()), // sub_14112e0a0
    Replacement::draft(0x14112e2a0, "unknown_14112e2a0", tpl_3080::<0x143cdff34, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14112e2a0
    Replacement::draft(0x14112e5e0, "unknown_14112e5e0", tpl_3270::<0x143cca0fc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14112e5e0
    Replacement::draft(0x14112e920, "unknown_14112e920", fn_14112e920 as *const ()), // sub_14112e920
    Replacement::draft(0x14112eb50, "unknown_14112eb50", fn_14112eb50 as *const ()), // sub_14112eb50
    Replacement::draft(0x14112ebe0, "unknown_14112ebe0", fn_14112ebe0 as *const ()), // sub_14112ebe0
    Replacement::draft(0x14112ef70, "unknown_14112ef70", fn_14112ef70 as *const ()), // sub_14112ef70
    Replacement::draft_throws(0x14112f130, "unknown_14112f130", fn_14112f130 as *const (), Throws::Discard), // sub_14112f130
    Replacement::draft(0x14112f300, "unknown_14112f300", tpl_19812::<0x1401e6d50, 0x140244ce0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14328f7a8, 0x14240aa00> as *const ()), // sub_14112f300
    Replacement::draft(0x14112f4a0, "unknown_14112f4a0", fn_14112f4a0 as *const ()), // sub_14112f4a0
    Replacement::draft_throws(0x14112f6a0, "unknown_14112f6a0", tpl_1771::<0x1411e9f50, 0x141132e20, 0x143ce018c, 0x1420af120, 0x140215750, 0x143cdfb90, 0x143cdfbe4, 0x1420a25b0, 0x1420a28e0, 0x1411326f0, 0x1401de410, 0x141834a78, 0x143cdfbc8, 0x143cdfbe0, 0x143cdfbe8, 0x141834a0c> as *const (), Throws::Discard), // sub_14112f6a0
    Replacement::draft(0x14112f8d0, "unknown_14112f8d0", tpl_4469::<0x1401e6d50, 0x141130db0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1410a7590> as *const ()), // sub_14112f8d0
    Replacement::draft(0x14112fa60, "unknown_14112fa60", tpl_4368::<0x1401e6d50, 0x141130de0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14112fbe0> as *const ()), // sub_14112fa60
    Replacement::draft(0x14112fbe0, "unknown_14112fbe0", fn_14112fbe0 as *const ()), // sub_14112fbe0
    Replacement::draft(0x14112fe90, "unknown_14112fe90", fn_14112fe90 as *const ()), // sub_14112fe90
    Replacement::draft_throws(0x14112ffe0, "unknown_14112ffe0", fn_14112ffe0 as *const (), Throws::Discard), // sub_14112ffe0
    Replacement::draft_throws(0x141130080, "unknown_141130080", tpl_21189::<0x1411301b0, 0x141131650, 0x14039c2d0, 0x141130300, 0x14039b440> as *const (), Throws::Discard), // sub_141130080
    Replacement::draft_throws(0x1411301b0, "unknown_1411301b0", fn_1411301b0 as *const (), Throws::Discard), // sub_1411301b0
    Replacement::draft(0x141130300, "unknown_141130300", tpl_79525::<0x10, 0x18, 0x20, 0x8, 0x28, 0x30, 0x141fa7b90> as *const ()), // sub_141130300
    Replacement::draft_throws(0x141130410, "unknown_141130410", fn_141130410 as *const (), Throws::Discard), // sub_141130410
    Replacement::draft_throws(0x141130550, "unknown_141130550", fn_141130550 as *const (), Throws::Discard), // sub_141130550
    Replacement::draft(0x141130660, "unknown_141130660", fn_141130660 as *const ()), // sub_141130660
    Replacement::draft(0x141130760, "unknown_141130760", tpl_1956::<0x88, 0x10088, 0x143c94b44, 0x20, 0x8, 0x141131440, 0x141130840> as *const ()), // sub_141130760
    Replacement::draft(0x141130840, "unknown_141130840", tpl_1957::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141fa6500> as *const ()), // sub_141130840
    Replacement::draft_throws(0x141130940, "unknown_141130940", tpl_21168::<0x40, 0x48, 0x10048, 0x1411312f0, 0x141130a40> as *const (), Throws::Discard), // sub_141130940
    Replacement::draft_throws(0x141130a40, "unknown_141130a40", fn_141130a40 as *const (), Throws::Discard), // sub_141130a40
    Replacement::draft(0x141130b20, "unknown_141130b20", tpl_3028::<0x143cdfbc0, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141fa5f80> as *const ()), // sub_141130b20
    Replacement::draft_throws(0x141130b90, "unknown_141130b90", tpl_62933::<0x143c9a324, 0x142489ee0, 0x141fa6310> as *const (), Throws::Discard), // sub_141130b90
    Replacement::draft(0x141130c90, "unknown_141130c90", tpl_1898::<0x88, 0x10088, 0x141fa61e0> as *const ()), // sub_141130c90
    Replacement::draft(0x141130db0, "unknown_141130db0", tpl_1955::<0x10, 0x1410a74e0> as *const ()), // sub_141130db0
    Replacement::draft(0x141130de0, "unknown_141130de0", tpl_1955::<0x10, 0x141128780> as *const ()), // sub_141130de0
    Replacement::draft_throws(0x141131b30, "unknown_141131b30", fn_141131b30 as *const (), Throws::Discard), // sub_141131b30
    Replacement::draft_throws(0x141131cb0, "unknown_141131cb0", fn_141131cb0 as *const (), Throws::Discard), // sub_141131cb0
    Replacement::draft_throws(0x141131f70, "unknown_141131f70", fn_141131f70 as *const (), Throws::Discard), // sub_141131f70
    Replacement::draft(0x141132190, "unknown_141132190", fn_141132190 as *const ()), // sub_141132190
    Replacement::draft(0x141132300, "unknown_141132300", tpl_61593::<0x143c94b44, 0x1411289e0, 0x141fa6500> as *const ()), // sub_141132300
    Replacement::draft_throws(0x141132510, "unknown_141132510", tpl_79502::<0x1411312f0, 0x141fa7c00> as *const (), Throws::Discard), // sub_141132510
    Replacement::draft(0x1411326f0, "unknown_1411326f0", tpl_1217::<0x143ce018c, 0x20, 0x8, 0x141fa6ad0> as *const ()), // sub_1411326f0
    Replacement::draft_throws(0x141132750, "unknown_141132750", tpl_62934::<0x143c9a324, 0x142489ee0, 0x141fa6310> as *const (), Throws::Discard), // sub_141132750
    Replacement::draft(0x1411328a0, "unknown_1411328a0", tpl_61552::<0x8, 0x88, 0x10088, 0x141fa61e0> as *const ()), // sub_1411328a0
    Replacement::draft(0x1411332e0, "unknown_1411332e0", fn_1411332e0 as *const ()), // sub_1411332e0
    Replacement::draft(0x141133620, "unknown_141133620", tpl_3389::<0x143ce0184, 0x143cdfd50, 0x141834a78, 0x143cd9c68, 0x143c8ec20, 0x143ce00ec, 0x143c8d120, 0x143cca0fc, 0x143cdff34, 0x143c8d074, 0x143cdfd70, 0x143cdfd90, 0x143cdfda0, 0x141834a0c> as *const ()), // sub_141133620
    Replacement::draft(0x141133850, "unknown_141133850", fn_141133850 as *const ()), // sub_141133850
    Replacement::draft(0x141133bd0, "unknown_141133bd0", fn_141133bd0 as *const ()), // sub_141133bd0
    Replacement::draft(0x141133cb0, "unknown_141133cb0", fn_141133cb0 as *const ()), // sub_141133cb0
    Replacement::draft_throws(0x141133d10, "coregame_141133d10", fn_141133d10 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cinematic_light::handleEntersAndExits>
    Replacement::draft_throws(0x141133ff0, "coregame_141133ff0", fn_141133ff0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cinematic_light::applyToRendering>
    Replacement::draft(0x141134310, "coregame_141134310", fn_141134310 as *const ()), // sub_141134310
    Replacement::draft(0x141134640, "coregame_141134640", fn_141134640 as *const ()), // sub_141134640
    Replacement::draft_throws(0x141134960, "coregame_141134960", fn_141134960 as *const (), Throws::Discard), // sub_141134960
    Replacement::draft_throws(0x141134a60, "coregame_141134a60", tpl_1330::<0x8, 0x141134ed0, 0x143ce03c0, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_141134a60
    Replacement::draft_throws(0x141134b40, "coregame_141134b40", tpl_1330::<0x8, 0x141134fc0, 0x143ce040c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_141134b40
    Replacement::draft_throws(0x141134c20, "coregame_141134c20", fn_141134c20 as *const (), Throws::Discard), // sub_141134c20
    Replacement::draft_throws(0x141134d20, "coregame_141134d20", fn_141134d20 as *const (), Throws::Discard), // sub_141134d20
    Replacement::draft(0x141134e30, "coregame_141134e30", fn_141134e30 as *const ()), // sub_141134e30
    Replacement::draft_throws(0x141135220, "coregame_141135220", tpl_21207::<0x143ce055c, 0x143ce0530, 0x1411e9f50, 0x141136680, 0x143ce04e8, 0x1420af120, 0x14036fad0, 0x14036e720, 0x1420a28e0, 0x1411363e0, 0x141135f10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ade96, 0x142646960, 0x143ce0550, 0x143ce0558, 0x143ce055a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactermovement::applyMovement>
    Replacement::draft(0x141135510, "coregame_141135510", tpl_1435::<0x143c91f1c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141135510
    Replacement::draft(0x141135860, "coregame_141135860", tpl_3072::<0x143c9f438, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141135860
    Replacement::draft(0x141135bb0, "coregame_141135bb0", tpl_4390::<0x143ca7420, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141135bb0
    Replacement::draft(0x141135f10, "coregame_141135f10", fn_141135f10 as *const ()), // sub_141135f10
    Replacement::draft(0x141136040, "coregame_141136040", fn_141136040 as *const ()), // sub_141136040
    Replacement::draft(0x1411363e0, "coregame_1411363e0", fn_1411363e0 as *const ()), // sub_1411363e0
    Replacement::draft(0x141136680, "coregame_141136680", tpl_5053::<0x143ce0528, 0x143ce0570, 0x141834a78, 0x143ca72c4, 0x143c92bcc, 0x143c91f1c, 0x143c933f0, 0x143c8d120, 0x143c93358, 0x143c8f6d8, 0x143c8d074, 0x143ce0590, 0x143ce05b0, 0x143ce05d0, 0x141834a0c> as *const ()), // sub_141136680
    Replacement::draft(0x1411368d0, "coregame_1411368d0", tpl_1095::<0x143c8dc6c, 0x14020e8f0> as *const ()), // sub_1411368d0
    Replacement::draft(0x141136ab0, "coregame_141136ab0", tpl_1095::<0x143caed64, 0x14020e8f0> as *const ()), // sub_141136ab0
    Replacement::draft(0x141136e30, "coregame_141136e30", fn_141136e30 as *const ()), // sub_141136e30
    Replacement::draft_throws(0x141136ea0, "coregame_141136ea0", tpl_19935::<0x143ce0b44, 0x143ce0948, 0x1411398a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aea46, 0x142646960, 0x143ce0968, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::gather>
    Replacement::draft_throws(0x1411370a0, "coregame_1411370a0", tpl_1101::<0x143ce07ac, 0x143ce0e58, 0x1411e9f50, 0x14113da70, 0x143ce086c, 0x1420af120, 0x1420a28e0, 0x14113c240, 0x14113b3e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ae7f6, 0x142646960, 0x143ce0e78, 0x143ce0e80, 0x143ce0e84, 0x143ce0e86, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::updateEventActivity>
    Replacement::draft_throws(0x1411373a0, "coregame_1411373a0", tpl_5293::<0x143ce0da0, 0x143ce0e88, 0x1411e9f50, 0x14113b510, 0x14113c3e0, 0x14113b6a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433af736, 0x142646960, 0x143ce0ea8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::update>
    Replacement::draft_throws(0x1411375e0, "coregame_1411375e0", tpl_3211::<0x143ce0828, 0x143ce0aa0, 0x1411e9f50, 0x14113df00, 0x143ce09ac, 0x1420af120, 0x1420a28e0, 0x14113c5d0, 0x14113b7a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433af4c6, 0x142646960, 0x143ce0ac0, 0x143ce0ad0, 0x143ce0ad2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::updateControlParameters>
    Replacement::draft_throws(0x1411378d0, "coregame_1411378d0", tpl_1394::<0x143ce06f4, 0x143ce0eb0, 0x141139a60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433afb76, 0x142646960, 0x143ce0ed0, 0x143ce0ed4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::postUpdate>
    Replacement::draft(0x141137ae0, "coregame_141137ae0", tpl_3077::<0x143ce09f0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141137ae0
    Replacement::draft(0x141137e10, "coregame_141137e10", tpl_1436::<0x58, 0x60, 0x158, 0x143ce09f0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141137e10
    Replacement::draft(0x141137ef0, "coregame_141137ef0", fn_141137ef0 as *const ()), // sub_141137ef0
    Replacement::draft(0x141138260, "coregame_141138260", fn_141138260 as *const ()), // sub_141138260
    Replacement::draft(0x1411385a0, "coregame_1411385a0", tpl_1402::<0x143ce07e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411385a0
    Replacement::draft(0x1411388f0, "coregame_1411388f0", tpl_1097::<0x143ce0940, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411388f0
    Replacement::draft(0x141138c20, "coregame_141138c20", fn_141138c20 as *const ()), // sub_141138c20
    Replacement::draft(0x141138dd0, "coregame_141138dd0", fn_141138dd0 as *const ()), // sub_141138dd0
    Replacement::draft(0x141138fd0, "coregame_141138fd0", tpl_1094::<0x143cd6e58, 0x14113ae10, 0x14020e8f0> as *const ()), // sub_141138fd0
    Replacement::draft(0x1411391d0, "coregame_1411391d0", tpl_1094::<0x143cd6e6c, 0x14113b080, 0x14020e8f0> as *const ()), // sub_1411391d0
    Replacement::draft(0x1411393d0, "coregame_1411393d0", tpl_1094::<0x143ce0cec, 0x14113b230, 0x14020e8f0> as *const ()), // sub_1411393d0
    Replacement::draft(0x1411395d0, "coregame_1411395d0", tpl_1466::<0x143cd6e5c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411395d0
    Replacement::draft_throws(0x1411398a0, "coregame_1411398a0", tpl_3011::<0x1411e9f50, 0x14113dbb0, 0x143ce0eac, 0x1420af120, 0x140215750, 0x143ce0c20, 0x140215610, 0x1420a28e0, 0x14113c1d0, 0x1401de410> as *const (), Throws::Discard), // sub_1411398a0
    Replacement::draft_throws(0x141139a60, "coregame_141139a60", tpl_1187::<0x1411e9f50, 0x14113e040, 0x143ce0c18, 0x1420af120, 0x14113d930, 0x143ce0ed8, 0x140215750, 0x143ce0c20, 0x140215610, 0x1420a28e0, 0x14113c6a0, 0x14113b820> as *const (), Throws::Discard), // sub_141139a60
    Replacement::draft_throws(0x141139c80, "coregame_141139c80", tpl_4129::<0x143ce076c, 0x143ce0cf0, 0x1411e9f50, 0x14113e750, 0x143ce0ad4, 0x1420af120, 0x1420a28e0, 0x14113d390, 0x14113c8a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433aedc6, 0x142646960, 0x143ce0d10, 0x143ce0d14, 0x143ce0d16, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::streamInPOI>
    Replacement::draft_throws(0x141139f70, "coregame_141139f70", tpl_3682::<0x143ce0d1c, 0x143ce07b8, 0x1411e9f50, 0x14113e660, 0x143ce0a94, 0x1420af120, 0x1420a28e0, 0x14113d540, 0x14113c990, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433af326, 0x142646960, 0x143ce07d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::streamOutPOI>
    Replacement::draft_throws(0x14113a250, "coregame_14113a250", tpl_1437::<0x143ce081c, 0x143ce0b18, 0x1411e9f50, 0x14113e4e0, 0x143ce0c14, 0x1420af120, 0x1420a28e0, 0x14113d620, 0x14113ca30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433af016, 0x142646960, 0x143ce0b38, 0x143ce0b40, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::streamInLookAt>
    Replacement::draft_throws(0x14113a530, "coregame_14113a530", tpl_1438::<0x143ce0a50, 0x143ce0700, 0x1411e9f50, 0x14113e3d0, 0x143ce09b0, 0x1420af120, 0x1420a28e0, 0x14113d7d0, 0x14113cb60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b0446, 0x142646960, 0x143ce0720, 0x143ce0728, 0x143ce072a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterlookat::streamOutLookAt>
    Replacement::draft(0x14113a820, "unknown_14113a820", tpl_4160::<0x8, 0x1411393d0, 0x1411395d0> as *const ()), // sub_14113a820
    Replacement::draft(0x14113a8c0, "unknown_14113a8c0", fn_14113a8c0 as *const ()), // sub_14113a8c0
    Replacement::draft(0x14113aa60, "unknown_14113aa60", fn_14113aa60 as *const ()), // sub_14113aa60
    Replacement::draft(0x14113abf0, "unknown_14113abf0", fn_14113abf0 as *const ()), // sub_14113abf0
    Replacement::draft(0x14113ad90, "unknown_14113ad90", fn_14113ad90 as *const ()), // sub_14113ad90
    Replacement::draft(0x14113ae10, "unknown_14113ae10", fn_14113ae10 as *const ()), // sub_14113ae10
    Replacement::draft(0x14113b080, "unknown_14113b080", fn_14113b080 as *const ()), // sub_14113b080
    Replacement::draft(0x14113b230, "unknown_14113b230", fn_14113b230 as *const ()), // sub_14113b230
    Replacement::draft(0x14113b3e0, "unknown_14113b3e0", tpl_62531::<0x88, 0x10088, 0x14113b940, 0x141faea50> as *const ()), // sub_14113b3e0
    Replacement::draft_throws(0x14113b510, "unknown_14113b510", tpl_5324::<0x14113dd60, 0x143ce0edc, 0x1420af120, 0x14027c6a0, 0x143ce0c20, 0x140215610, 0x143c8f63c, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_14113b510
    Replacement::draft(0x14113b6a0, "unknown_14113b6a0", fn_14113b6a0 as *const ()), // sub_14113b6a0
    Replacement::draft(0x14113b7a0, "unknown_14113b7a0", tpl_19285::<0x88, 0x10088, 0x14113bd10, 0x141faf300> as *const ()), // sub_14113b7a0
    Replacement::draft(0x14113b820, "unknown_14113b820", fn_14113b820 as *const ()), // sub_14113b820
    Replacement::draft_throws(0x14113ef20, "coregame_14113ef20", tpl_3005::<0x143ce118c, 0x143ce0f20, 0x1411e9f50, 0x141140d20, 0x141141610, 0x141140ed0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b3016, 0x142646960, 0x143ce0f40, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterfootstep::update>
    Replacement::draft(0x14113f160, "coregame_14113f160", tpl_1526::<0x1429408c8, 0x1410d6ca0, 0x1411400a0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14113f160
    Replacement::draft(0x14113f400, "coregame_14113f400", tpl_1094::<0x143ce1318, 0x141140980, 0x14020e8f0> as *const ()), // sub_14113f400
    Replacement::draft(0x14113f600, "coregame_14113f600", tpl_1436::<0x58, 0x60, 0x158, 0x143ce1318, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14113f600
    Replacement::draft(0x14113f6e0, "coregame_14113f6e0", fn_14113f6e0 as *const ()), // sub_14113f6e0
    Replacement::draft(0x14113f8d0, "coregame_14113f8d0", tpl_1094::<0x143ce1050, 0x141140b00, 0x14020e8f0> as *const ()), // sub_14113f8d0
    Replacement::draft_throws(0x14113fad0, "coregame_14113fad0", tpl_1886::<0x143ce120c, 0x143ce1028, 0x1411e9f50, 0x141142c00, 0x143ce1164, 0x1420af120, 0x1420a28e0, 0x141141d00, 0x141141870, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b3536, 0x142646960, 0x143ce1048, 0x143ce104c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterfootstep::streamIn>
    Replacement::draft_throws(0x14113fdb0, "coregame_14113fdb0", tpl_4129::<0x143ce0f88, 0x143ce1138, 0x1411e9f50, 0x141142b10, 0x143ce12e0, 0x1420af120, 0x1420a28e0, 0x141141eb0, 0x141141960, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b29c6, 0x142646960, 0x143ce1158, 0x143ce115c, 0x143ce115e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::characterfootstep::streamOut>
    Replacement::draft(0x1411400a0, "coregame_1411400a0", fn_1411400a0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::characterfootstep::removedFootstep>
    Replacement::draft(0x141140360, "coregame_141140360", tpl_21227::<0x18, 0x8, 0x14113f8d0> as *const ()), // sub_141140360
    Replacement::draft(0x1411403a0, "coregame_1411403a0", tpl_20349::<0x1401e7c10, 0x141140800, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1411403a0
    Replacement::draft_throws(0x1411405b0, "coregame_1411405b0", tpl_19879::<0x143ce131c, 0x143ce10a0, 0x141140ff0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b2bf6, 0x142646960, 0x143ce10c0, 0x143ce10e0, 0x143ce10f0, 0x143ce10f8, 0x143ce10fc, 0x143ce10fe, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::CharacterFootstepCo
    Replacement::draft(0x141140900, "unknown_141140900", fn_141140900 as *const ()), // sub_141140900
    Replacement::draft(0x141140980, "unknown_141140980", fn_141140980 as *const ()), // sub_141140980
    Replacement::draft(0x141140b00, "unknown_141140b00", fn_141140b00 as *const ()), // sub_141140b00
    Replacement::draft_throws(0x141140d20, "unknown_141140d20", fn_141140d20 as *const (), Throws::Discard), // sub_141140d20
    Replacement::draft(0x141140ed0, "unknown_141140ed0", fn_141140ed0 as *const ()), // sub_141140ed0
    Replacement::draft_throws(0x141140ff0, "unknown_141140ff0", tpl_3011::<0x1411e9f50, 0x141142980, 0x143ce1324, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141142020, 0x1401de410> as *const (), Throws::Discard), // sub_141140ff0
    Replacement::draft(0x1411411b0, "unknown_1411411b0", fn_1411411b0 as *const ()), // sub_1411411b0
    Replacement::draft(0x1411415d0, "unknown_1411415d0", tpl_3160::<0x1f0, 0x1403b8050> as *const ()), // sub_1411415d0
    Replacement::draft(0x141141610, "unknown_141141610", fn_141141610 as *const ()), // sub_141141610
    Replacement::draft_throws(0x141141870, "unknown_141141870", fn_141141870 as *const (), Throws::Caught), // sub_141141870
    Replacement::draft(0x141141960, "unknown_141141960", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14113f600> as *const ()), // sub_141141960
    Replacement::draft_throws(0x141141d00, "unknown_141141d00", fn_141141d00 as *const (), Throws::Caught), // sub_141141d00
    Replacement::draft(0x141141eb0, "unknown_141141eb0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14113f600> as *const ()), // sub_141141eb0
    Replacement::draft(0x141141f90, "unknown_141141f90", tpl_19695::<0x143ce1318, 0x5e308, 0x10, 0x8, 0x18, 0x141fb3da0, 0x141142740> as *const ()), // sub_141141f90
    Replacement::draft(0x141142020, "unknown_141142020", tpl_3028::<0x143ce1324, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1411403a0> as *const ()), // sub_141142020
    Replacement::draft(0x141142090, "unknown_141142090", fn_141142090 as *const ()), // sub_141142090
    Replacement::draft(0x1411422e0, "unknown_1411422e0", tpl_3188::<0x141142a70, 0x1433b3760, 0x141142740, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1411422e0
    Replacement::draft(0x141142360, "unknown_141142360", tpl_19620::<0x8, 0x141142740> as *const ()), // sub_141142360
    Replacement::draft(0x1411425d0, "unknown_1411425d0", fn_1411425d0 as *const ()), // sub_1411425d0
    Replacement::draft(0x141142740, "unknown_141142740", fn_141142740 as *const ()), // sub_141142740
    Replacement::draft(0x141142860, "unknown_141142860", tpl_4170::<0x14023ba50, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_141142860
    Replacement::draft(0x141142a70, "unknown_141142a70", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433b3760, 0x8, 0x20> as *const ()), // sub_141142a70
    Replacement::draft(0x141142f80, "unknown_141142f80", tpl_21238::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1434281d0, 0x20, 0x28, 0x30> as *const ()), // sub_141142f80
    Replacement::draft(0x141143010, "unknown_141143010", fn_141143010 as *const ()), // sub_141143010
    Replacement::draft(0x1411431b0, "unknown_1411431b0", fn_1411431b0 as *const ()), // sub_1411431b0
    Replacement::draft(0x141143250, "coregame_141143250", fn_141143250 as *const ()), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::removedController>
    Replacement::draft(0x141143540, "coregame_141143540", tpl_6217::<0x143ce17a0, 0x143ce1438, 0x1411e9f50, 0x1411489c0, 0x143ca71a4, 0x1401fc320, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x141147cd0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b4506, 0x142646960, 0x143ce1458, 0x143ce1468, 0x143ce146c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::removedControllerBody>
    Replacement::draft_throws(0x141143840, "coregame_141143840", fn_141143840 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::deleteController>
    Replacement::draft_throws(0x141143b20, "coregame_141143b20", fn_141143b20 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::createController>
    Replacement::draft_throws(0x141143e30, "coregame_141143e30", tpl_3003::<0x143ce16d0, 0x143ce15c0, 0x1411e9f50, 0x1411495a0, 0x143ce16dc, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x1411482c0, 0x141146ba0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b45f6, 0x142646960, 0x143ce15e0, 0x143ce15e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::prePhysics>
    Replacement::draft_throws(0x141144150, "coregame_141144150", fn_141144150 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::applyControllerToTransform>
    Replacement::draft_throws(0x141144450, "coregame_141144450", tpl_5173::<0x143ce1760, 0x143ce1370, 0x1411454f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b38f6, 0x142646960, 0x143ce1390, 0x143ce13a0, 0x143ce13a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::invalidateActiveSimulated>
    Replacement::draft(0x141144660, "coregame_141144660", tpl_1436::<0x58, 0x60, 0x158, 0x143c917cc, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141144660
    Replacement::draft(0x141144740, "coregame_141144740", tpl_1436::<0x58, 0x60, 0x158, 0x143ca71a4, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141144740
    Replacement::draft(0x141144820, "coregame_141144820", tpl_1436::<0x58, 0x60, 0x158, 0x143ca72c8, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141144820
    Replacement::draft(0x141144900, "coregame_141144900", tpl_1436::<0x58, 0x60, 0x158, 0x143c97a60, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141144900
    Replacement::draft(0x1411449e0, "coregame_1411449e0", tpl_1094::<0x143ca72c4, 0x1411463f0, 0x14020e8f0> as *const ()), // sub_1411449e0
    Replacement::draft(0x141144be0, "coregame_141144be0", tpl_1466::<0x143cb3854, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141144be0
    Replacement::draft(0x141144eb0, "coregame_141144eb0", tpl_1097::<0x143ca72cc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141144eb0
    Replacement::draft(0x1411451e0, "coregame_1411451e0", tpl_5972::<0x143c97a5c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411451e0
    Replacement::draft_throws(0x1411454f0, "coregame_1411454f0", tpl_3697::<0x1411e9f50, 0x1411492e0, 0x143ce158c, 0x1420af120, 0x140215750, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x1411487c0, 0x1401de410> as *const (), Throws::Discard), // sub_1411454f0
    Replacement::draft_throws(0x1411456a0, "coregame_1411456a0", tpl_3683::<0x143ce16d4, 0x143ce1590, 0x1411e9f50, 0x141149ca0, 0x143ce189c, 0x1420af120, 0x1420a28e0, 0x1411491d0, 0x141148830, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b5c46, 0x142646960, 0x143ce15b0, 0x143ce15b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::streamOut>
    Replacement::draft(0x141145980, "unknown_141145980", tpl_1094::<0x143c917cc, 0x141147000, 0x14020e8f0> as *const ()), // sub_141145980
    Replacement::draft(0x141145b80, "unknown_141145b80", tpl_1094::<0x143ca71a4, 0x1411471a0, 0x14020e8f0> as *const ()), // sub_141145b80
    Replacement::draft(0x141145d80, "unknown_141145d80", tpl_1097::<0x143ca72c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141145d80
    Replacement::draft(0x1411460b0, "unknown_1411460b0", tpl_1098::<0x143c97a60, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411460b0
    Replacement::draft(0x1411463f0, "unknown_1411463f0", fn_1411463f0 as *const ()), // sub_1411463f0
    Replacement::draft_throws(0x141146590, "unknown_141146590", tpl_21189::<0x1411301b0, 0x141147af0, 0x14039c2d0, 0x1411466c0, 0x14039b440> as *const (), Throws::Discard), // sub_141146590
    Replacement::draft(0x1411466c0, "unknown_1411466c0", tpl_79525::<0x10, 0x18, 0x20, 0x8, 0x28, 0x30, 0x141fb69d0> as *const ()), // sub_1411466c0
    Replacement::draft_throws(0x1411467d0, "unknown_1411467d0", fn_1411467d0 as *const (), Throws::Discard), // sub_1411467d0
    Replacement::draft_throws(0x141146900, "unknown_141146900", fn_141146900 as *const (), Throws::Discard), // sub_141146900
    Replacement::draft(0x141146a80, "unknown_141146a80", fn_141146a80 as *const ()), // sub_141146a80
    Replacement::draft(0x141146ba0, "unknown_141146ba0", tpl_5032::<0x88, 0x10088, 0x580e0, 0x143c8f63c, 0x141147630, 0x141146c60> as *const ()), // sub_141146ba0
    Replacement::draft(0x141146c60, "unknown_141146c60", tpl_5033::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141fb6c10> as *const ()), // sub_141146c60
    Replacement::draft(0x141146d70, "unknown_141146d70", tpl_21248::<0x88, 0x10088, 0x8, 0x20, 0x143c94b44, 0x143c94b30, 0x141147330, 0x141146ea0> as *const ()), // sub_141146d70
    Replacement::draft(0x141146ea0, "unknown_141146ea0", tpl_21249::<0x141fb6a90> as *const ()), // sub_141146ea0
    Replacement::draft(0x141147000, "unknown_141147000", fn_141147000 as *const ()), // sub_141147000
    Replacement::draft(0x1411471a0, "unknown_1411471a0", tpl_2731::<0x1401e6d50, 0x141148960, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_1411471a0
    Replacement::draft(0x1411478a0, "unknown_1411478a0", tpl_5037::<0x8, 0x10, 0x18, 0x141147950, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50> as *const ()), // sub_1411478a0
    Replacement::draft(0x141147950, "unknown_141147950", tpl_5038::<0x143ca72cc, 0x142489ee0, 0x143c8d87c, 0x143ca72c4, 0x143c8fab8, 0x143c8d120, 0x143cb3854, 0x143c92bc8> as *const ()), // sub_141147950
    Replacement::draft(0x141147af0, "unknown_141147af0", tpl_1786::<0x143c8d694, 0x142489ee0, 0x143c8d87c, 0x143cb3854> as *const ()), // sub_141147af0
    Replacement::draft(0x141147c20, "unknown_141147c20", tpl_3172::<0x143c917cc, 0x5e308, 0x143c8f63c, 0x580e0, 0x10, 0x8, 0x18, 0x141fb63b0, 0x141148f60> as *const ()), // sub_141147c20
    Replacement::draft(0x141147cd0, "unknown_141147cd0", tpl_3172::<0x143ca71a4, 0x5e308, 0x143c8f63c, 0x580e0, 0x10, 0x8, 0x18, 0x141fb6310, 0x1411490a0> as *const ()), // sub_141147cd0
    Replacement::draft_throws(0x141147d80, "unknown_141147d80", fn_141147d80 as *const (), Throws::Discard), // sub_141147d80
    Replacement::draft_throws(0x141148040, "unknown_141148040", fn_141148040 as *const (), Throws::Discard), // sub_141148040
    Replacement::draft(0x1411482c0, "unknown_1411482c0", tpl_64227::<0x143c8f63c, 0x1411476f0, 0x141fb6c10> as *const ()), // sub_1411482c0
    Replacement::draft(0x1411484d0, "unknown_1411484d0", tpl_79591::<0x143c94b44, 0x143c94b30, 0x141147430, 0x141fb6a90> as *const ()), // sub_1411484d0
    Replacement::draft(0x1411487c0, "unknown_1411487c0", tpl_3028::<0x143ce158c, 0x20, 0x8, 0x143c8f63c, 0x580e0, 0x141fb5fd0> as *const ()), // sub_1411487c0
    Replacement::draft(0x141148830, "unknown_141148830", tpl_1126::<0x142407c80, 0x580f0, 0x141fb6960> as *const ()), // sub_141148830
    Replacement::draft(0x1411488f0, "unknown_1411488f0", fn_1411488f0 as *const ()), // sub_1411488f0
    Replacement::draft(0x141148960, "unknown_141148960", tpl_6168::<0x7, 0x8, 0x10, 0x1401de830, 0x142442de0> as *const ()), // sub_141148960
    Replacement::draft(0x1411489c0, "unknown_1411489c0", tpl_3188::<0x141149b60, 0x1433b6610, 0x1411490a0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1411489c0
    Replacement::draft(0x141148a40, "unknown_141148a40", tpl_3188::<0x141149c00, 0x1433b6638, 0x141148f60, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141148a40
    Replacement::draft(0x141148d00, "unknown_141148d00", fn_141148d00 as *const ()), // sub_141148d00
    Replacement::draft(0x141148f60, "unknown_141148f60", fn_141148f60 as *const ()), // sub_141148f60
    Replacement::draft(0x1411490a0, "unknown_1411490a0", fn_1411490a0 as *const ()), // sub_1411490a0
    Replacement::draft(0x1411491d0, "unknown_1411491d0", tpl_1130::<0x142407c80, 0x141fb6960> as *const ()), // sub_1411491d0
    Replacement::draft(0x141149390, "unknown_141149390", tpl_21258::<0x143ce16bc, 0x143ce16f0, 0x143ca72cc, 0x143c917cc, 0x143c91f1c, 0x143c92bcc, 0x143c933f0, 0x143c8d120, 0x143c93358, 0x143c8f6d8, 0x143c8d074, 0x143ce1710, 0x143ce1730, 0x143ce1750, 0x141834a0c, 0x141834a78> as *const ()), // sub_141149390
    Replacement::draft(0x1411495a0, "unknown_1411495a0", tpl_3173::<0x143ce1470, 0x143ce1630, 0x141834a78, 0x143c8f6d8, 0x143c8d120, 0x143c97a5c, 0x143c917cc, 0x143ca71a4, 0x143c8d074, 0x143ca7420, 0x143ce1650, 0x143ce1670, 0x143ce1680, 0x141834a0c> as *const ()), // sub_1411495a0
    Replacement::draft(0x141149740, "unknown_141149740", fn_141149740 as *const ()), // sub_141149740
    Replacement::draft(0x1411499c0, "unknown_1411499c0", fn_1411499c0 as *const ()), // sub_1411499c0
    Replacement::draft(0x141149b60, "unknown_141149b60", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433b6610, 0x8, 0x20> as *const ()), // sub_141149b60
    Replacement::draft(0x141149c00, "unknown_141149c00", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433b6638, 0x8, 0x20> as *const ()), // sub_141149c00
    Replacement::draft_throws(0x14114a130, "coregame_14114a130", fn_14114a130 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::cameraman::update>
    Replacement::draft(0x14114a4b0, "coregame_14114a4b0", tpl_62998::<0x14114a5e0, 0x141fb8340> as *const ()), // sub_14114a4b0
    Replacement::draft(0x14114a740, "coregame_14114a740", tpl_62999::<0x14114a5e0, 0x141fb8340> as *const ()), // sub_14114a740
    Replacement::draft_throws(0x14114abd0, "coregame_14114abd0", tpl_1480::<0x142942ea0, 0x1410d6ca0, 0x14114bc10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14114abd0
    Replacement::draft_throws(0x14114ae70, "coregame_14114ae70", fn_14114ae70 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::camera::update>
    Replacement::draft(0x14114b320, "coregame_14114b320", tpl_1099::<0x143cb82cc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14114b320
    Replacement::draft(0x14114b660, "coregame_14114b660", fn_14114b660 as *const ()), // sub_14114b660
    Replacement::draft(0x14114b9e0, "coregame_14114b9e0", tpl_1094::<0x143c9d6b8, 0x14114be20, 0x14020e8f0> as *const ()), // sub_14114b9e0
    Replacement::draft_throws(0x14114bc10, "coregame_14114bc10", tpl_6402::<0x143ce1ab0, 0x143ce19c0, 0x14114bfc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b7236, 0x142646960, 0x143ce19e0, 0x143ce19f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::camera::updateCameraDofFromGameParameters>
    Replacement::draft(0x14114be20, "coregame_14114be20", fn_14114be20 as *const ()), // sub_14114be20
    Replacement::draft_throws(0x14114bfc0, "coregame_14114bfc0", tpl_3697::<0x1411e9f50, 0x14114c510, 0x143ce1aac, 0x1420af120, 0x140215750, 0x143c8ce80, 0x140215610, 0x1420a28e0, 0x14114c4a0, 0x1401de410> as *const (), Throws::Discard), // sub_14114bfc0
    Replacement::draft(0x14114c170, "coregame_14114c170", tpl_60530::<0x1426467e8, 0x143c9d6b8, 0x142489ee0, 0x143cb82cc, 0x14114c199> as *const ()), // sub_14114c170
    Replacement::draft_throws(0x14114c340, "coregame_14114c340", tpl_4018::<0x143ce1a34, 0x143ce1a38, 0x143ce1a18, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8cf5c, 0x143c8d120, 0x143ce1a28, 0x143ce1a30, 0x141834a0c> as *const (), Throws::Discard), // sub_14114c340
    Replacement::draft(0x14114c4a0, "coregame_14114c4a0", tpl_3028::<0x143ce1aac, 0x20, 0x8, 0x143c8ce80, 0x580e0, 0x141fb9310> as *const ()), // sub_14114c4a0
    Replacement::draft(0x14114c690, "coregame_14114c690", tpl_1008::<0x20, 0x142442de0> as *const ()), // sub_14114c690
    Replacement::draft_throws(0x14114c6c0, "coregame_14114c6c0", fn_14114c6c0 as *const (), Throws::Discard), // sub_14114c6c0
    Replacement::draft_throws(0x14114c830, "coregame_14114c830", fn_14114c830 as *const (), Throws::Caught), // sub_14114c830
    Replacement::draft(0x14114c910, "coregame_14114c910", fn_14114c910 as *const ()), // sub_14114c910
    Replacement::draft(0x14114ca30, "coregame_14114ca30", fn_14114ca30 as *const ()), // sub_14114ca30
    Replacement::draft(0x14114ccf0, "coregame_14114ccf0", fn_14114ccf0 as *const ()), // sub_14114ccf0
    Replacement::draft_throws(0x14114cea0, "coregame_14114cea0", fn_14114cea0 as *const (), Throws::Discard), // sub_14114cea0
    Replacement::draft(0x14114cff0, "coregame_14114cff0", tpl_21270::<0x142417470, 0x143841b24, 0x143a08300, 0x14248b000, 0x14387e438> as *const ()), // sub_14114cff0
    Replacement::draft(0x14114d220, "coregame_14114d220", fn_14114d220 as *const ()), // sub_14114d220
    Replacement::draft(0x14114d290, "coregame_14114d290", tpl_21270::<0x142417470, 0x143841b24, 0x143a08300, 0x14248b000, 0x14387e2ec> as *const ()), // sub_14114d290
    Replacement::draft_throws(0x14114d4c0, "coregame_14114d4c0", fn_14114d4c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spawning::despawnBundleChildren::system>
    Replacement::draft_throws(0x14114d790, "coregame_14114d790", fn_14114d790 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spawning::disconnectBundleChildrenFromRoot>
    Replacement::draft_throws(0x14114da70, "coregame_14114da70", tpl_1074::<0x143ce1d84, 0x143ce1b70, 0x1411e9f50, 0x141150390, 0x143ce1f9c, 0x1420af120, 0x1420a28e0, 0x14114ef30, 0x14114e900, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b7ca6, 0x142646960, 0x143ce1b90, 0x143ce1ba0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spawning::reconnectBundleChildrenToRoot>
    Replacement::draft_throws(0x14114dd50, "coregame_14114dd50", tpl_1370::<0x143ce1ec0, 0x143ce1da0, 0x14114dff0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b7856, 0x142646960, 0x143ce1dc0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spawning::connectBundleChildrenToRoot>
    Replacement::draft(0x14114df50, "unknown_14114df50", tpl_21274::<0x10, 0x18, 0x1, 0x28> as *const ()), // sub_14114df50
    Replacement::draft_throws(0x14114dff0, "unknown_14114dff0", fn_14114dff0 as *const (), Throws::Discard), // sub_14114dff0
    Replacement::draft(0x14114e1c0, "unknown_14114e1c0", fn_14114e1c0 as *const ()), // sub_14114e1c0
    Replacement::draft(0x14114e390, "unknown_14114e390", fn_14114e390 as *const ()), // sub_14114e390
    Replacement::draft(0x14114e680, "unknown_14114e680", tpl_1094::<0x143ce1fa4, 0x14114ebd0, 0x14020e8f0> as *const ()), // sub_14114e680
    Replacement::draft(0x14114e880, "unknown_14114e880", tpl_1214::<0x38, 0x8, 0x14114fbe0> as *const ()), // sub_14114e880
    Replacement::draft(0x14114e8c0, "unknown_14114e8c0", tpl_3165::<0x38, 0x8, 0x14114ff10> as *const ()), // sub_14114e8c0
    Replacement::draft(0x14114e900, "unknown_14114e900", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x14114e9f0, 0x580f0, 0x141fbe140> as *const ()), // sub_14114e900
    Replacement::draft_throws(0x14114eaf0, "unknown_14114eaf0", tpl_1330::<0x8, 0x141150480, 0x143ce1f24, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14114eaf0
    Replacement::draft(0x14114ebd0, "unknown_14114ebd0", fn_14114ebd0 as *const ()), // sub_14114ebd0
    Replacement::draft(0x14114ed90, "unknown_14114ed90", tpl_6144::<0x143ce1fa4, 0x5e308, 0x10, 0x8, 0x18, 0x141fbdd70, 0x14114f890> as *const ()), // sub_14114ed90
    Replacement::draft(0x14114ee30, "unknown_14114ee30", tpl_19300::<0x143ce200c, 0x5e308, 0x143ce1f24, 0x20, 0x8, 0x10, 0x18, 0x141fbded0, 0x140489dd0> as *const ()), // sub_14114ee30
    Replacement::draft(0x14114ef30, "unknown_14114ef30", tpl_3213::<0x142407c80, 0x143ce1fa4, 0x142489ee0, 0x143ce1dd4, 0x141fbe140> as *const ()), // sub_14114ef30
    Replacement::draft(0x14114f0e0, "unknown_14114f0e0", fn_14114f0e0 as *const ()), // sub_14114f0e0
    Replacement::draft(0x14114f3b0, "unknown_14114f3b0", tpl_3188::<0x141150530, 0x1433b83d8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14114f3b0
    Replacement::draft(0x14114f430, "unknown_14114f430", tpl_3188::<0x1411505d0, 0x1433b8410, 0x14114f890, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14114f430
    Replacement::draft(0x14114f6f0, "unknown_14114f6f0", fn_14114f6f0 as *const ()), // sub_14114f6f0
    Replacement::draft(0x14114f890, "unknown_14114f890", tpl_19450::<0x1401de830, 0x1401de4e0, 0x1425371a0, 0x142533200> as *const ()), // sub_14114f890
    Replacement::draft(0x14114fbb0, "unknown_14114fbb0", tpl_1955::<0x30, 0x1401de4e0> as *const ()), // sub_14114fbb0
    Replacement::draft(0x14114fbe0, "unknown_14114fbe0", tpl_1368::<0x143ce1dd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14114fbe0
    Replacement::draft(0x14114ff10, "unknown_14114ff10", tpl_1369::<0x143ce200c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14114ff10
    Replacement::draft(0x141150250, "unknown_141150250", tpl_5912::<0xfffffffffffffffe, 0x141150670, 0x142442de0> as *const ()), // sub_141150250
    Replacement::draft(0x141150530, "unknown_141150530", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433b83d8, 0x8, 0x20> as *const ()), // sub_141150530
    Replacement::draft(0x1411505d0, "unknown_1411505d0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433b8410, 0x8, 0x20> as *const ()), // sub_1411505d0
    Replacement::draft(0x141150670, "unknown_141150670", fn_141150670 as *const ()), // sub_141150670
    Replacement::draft(0x141150770, "unknown_141150770", fn_141150770 as *const ()), // sub_141150770
    Replacement::draft(0x141150970, "unknown_141150970", fn_141150970 as *const ()), // sub_141150970
    Replacement::draft(0x141150a70, "unknown_141150a70", tpl_381::<0x141150bd0, 0x14114e390> as *const ()), // sub_141150a70
    Replacement::draft(0x141150bd0, "unknown_141150bd0", fn_141150bd0 as *const ()), // sub_141150bd0
    Replacement::draft(0x141150de0, "unknown_141150de0", fn_141150de0 as *const ()), // sub_141150de0
    Replacement::draft_throws(0x141150e20, "unknown_141150e20", tpl_4541::<0x143ce1cb8, 0x1411502a0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141150e20
    Replacement::draft_throws(0x141150eb0, "unknown_141150eb0", tpl_4541::<0x143ce1f9c, 0x141150390, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141150eb0
    Replacement::draft_throws(0x141150f40, "unknown_141150f40", tpl_4541::<0x143ce1f24, 0x141150480, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141150f40
    Replacement::draft(0x141150fd0, "unknown_141150fd0", tpl_170::<0x141151310> as *const ()), // sub_141150fd0
    Replacement::draft(0x141150fe0, "unknown_141150fe0", fn_141150fe0 as *const ()), // sub_141150fe0
    Replacement::draft_throws(0x141151050, "unknown_141151050", fn_141151050 as *const (), Throws::Discard), // sub_141151050
    Replacement::draft_throws(0x1411510e0, "unknown_1411510e0", fn_1411510e0 as *const (), Throws::Discard), // sub_1411510e0
    Replacement::draft(0x1411511c0, "unknown_1411511c0", fn_1411511c0 as *const ()), // sub_1411511c0
    Replacement::draft(0x141151240, "unknown_141151240", fn_141151240 as *const ()), // sub_141151240
    Replacement::draft(0x141151310, "unknown_141151310", tpl_21289::<0x8, 0x141150fe0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141151310
    Replacement::draft_throws(0x1411513e0, "std_1411513e0", fn_1411513e0 as *const (), Throws::Discard), // sub_1411513e0
    Replacement::draft_throws(0x141151620, "std_141151620", fn_141151620 as *const (), Throws::Discard), // sub_141151620
    Replacement::draft(0x1411518e0, "std_1411518e0", tpl_1810::<0x48, 0x14240a730, 0x3, 0x4, 0x8, 0x14240ab10> as *const ()), // sub_1411518e0
    Replacement::draft_throws(0x141151950, "std_141151950", fn_141151950 as *const (), Throws::Discard), // sub_141151950
    Replacement::draft(0x141151a40, "std_141151a40", tpl_21293::<0x8, 0x141150fe0> as *const ()), // sub_141151a40
    Replacement::draft_throws(0x141151a80, "std_141151a80", fn_141151a80 as *const (), Throws::Discard), // sub_141151a80
    Replacement::draft(0x141151bf0, "std_141151bf0", fn_141151bf0 as *const ()), // sub_141151bf0
    Replacement::draft_throws(0x141151da0, "std_141151da0", fn_141151da0 as *const (), Throws::Discard), // sub_141151da0
    Replacement::draft(0x141152080, "std_141152080", fn_141152080 as *const ()), // sub_141152080
    Replacement::draft_throws(0x141152290, "std_141152290", fn_141152290 as *const (), Throws::Discard), // sub_141152290
    Replacement::draft_throws(0x1411524d0, "unknown_1411524d0", fn_1411524d0 as *const (), Throws::Discard), // sub_1411524d0
    Replacement::draft(0x141152590, "unknown_141152590", fn_141152590 as *const ()), // sub_141152590
    Replacement::draft_throws(0x1411526d0, "coregame_1411526d0", fn_1411526d0 as *const (), Throws::Discard), // coregame::BundleLoader::coregame::BundleLoader::vfunc_4
    Replacement::draft_throws(0x141152740, "coregame_141152740", fn_141152740 as *const (), Throws::Caught), // sub_141152740
    Replacement::draft_throws(0x1411527d0, "coregame_1411527d0", fn_1411527d0 as *const (), Throws::Caught), // sub_1411527d0
    Replacement::draft_throws(0x1411528f0, "coregame_1411528f0", tpl_19843::<0x143881268, 0x142417470, 0x141152b50, 0x142645790, 0x1426455a8, 0x142645598, 0x142645868, 0x142442de0, 0x143ce20d0, 0x141152c60> as *const (), Throws::Discard), // sub_1411528f0
    Replacement::draft_throws(0x141152a90, "coregame_141152a90", fn_141152a90 as *const (), Throws::Discard), // coregame::BundleContentLoader::coregame::BundleContentLoader::vfunc_0
    Replacement::draft_throws(0x141152b50, "coregame_141152b50", tpl_19848::<0x143ce20d0, 0x8, 0x19, 0x20, 0x10, 0x143ce20d8, 0x14025f9d0, 0x28, 0x30, 0x18, 0x140208500, 0x140200130, 0x141152c55> as *const (), Throws::Discard), // sub_141152b50
    Replacement::draft(0x141152c60, "coregame_141152c60", tpl_19849::<0x143ce20d0, 0x140246bb0, 0x1401e66f0, 0x143ce20d8, 0x1403f7860> as *const ()), // sub_141152c60
    Replacement::draft(0x141152e30, "coregame_141152e30", fn_141152e30 as *const ()), // sub_141152e30
    Replacement::draft(0x141152f60, "coregame_141152f60", fn_141152f60 as *const ()), // sub_141152f60
    Replacement::draft(0x141152f90, "coregame_141152f90", tpl_61657::<0x8, 0x18, 0x1401e59a0, 0x1401e66f0> as *const ()), // sub_141152f90
    Replacement::draft(0x141153000, "coregame_141153000", fn_141153000 as *const ()), // sub_141153000
    Replacement::draft(0x141153080, "coregame_141153080", fn_141153080 as *const ()), // sub_141153080
    Replacement::draft(0x1411530e0, "coregame_1411530e0", fn_1411530e0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::bundle_flow::handle_requests::system>
    Replacement::draft_throws(0x141153300, "std_141153300", fn_141153300 as *const (), Throws::Discard), // sub_141153300
    Replacement::draft_throws(0x141153630, "std_141153630", fn_141153630 as *const (), Throws::Discard), // sub_141153630
    Replacement::draft(0x1411539a0, "std_1411539a0", fn_1411539a0 as *const ()), // sub_1411539a0
    Replacement::draft(0x141153af0, "std_141153af0", tpl_4416::<0x1411e9f50, 0x143c8ce90, 0x140215610, 0x143c8cea4, 0x143cd6cc4, 0x1420a28e0, 0x141154350, 0x1401de410> as *const ()), // sub_141153af0
    Replacement::draft(0x141153c70, "std_141153c70", tpl_5918::<0x8, 0x1401e59a0, 0x1401fb4d0> as *const ()), // sub_141153c70
    Replacement::draft_throws(0x141153ca0, "std_141153ca0", fn_141153ca0 as *const (), Throws::Discard), // sub_141153ca0
    Replacement::draft_throws(0x141153e70, "std_141153e70", fn_141153e70 as *const (), Throws::Discard), // sub_141153e70
    Replacement::draft(0x141154030, "unknown_141154030", fn_141154030 as *const ()), // sub_141154030
    Replacement::draft(0x141154170, "unknown_141154170", fn_141154170 as *const ()), // sub_141154170
    Replacement::draft(0x141154270, "unknown_141154270", fn_141154270 as *const ()), // sub_141154270
    Replacement::draft_throws(0x141154350, "unknown_141154350", fn_141154350 as *const (), Throws::Discard), // sub_141154350
    Replacement::draft(0x141154390, "unknown_141154390", fn_141154390 as *const ()), // sub_141154390
    Replacement::draft(0x1411546b0, "unknown_1411546b0", fn_1411546b0 as *const ()), // sub_1411546b0
    Replacement::draft(0x141154720, "unknown_141154720", fn_141154720 as *const ()), // sub_141154720
    Replacement::draft(0x141154770, "unknown_141154770", fn_141154770 as *const ()), // sub_141154770
    Replacement::draft(0x1411548b0, "unknown_1411548b0", fn_1411548b0 as *const ()), // sub_1411548b0
    Replacement::draft(0x141154b10, "unknown_141154b10", fn_141154b10 as *const ()), // sub_141154b10
    Replacement::draft(0x141154d50, "unknown_141154d50", fn_141154d50 as *const ()), // sub_141154d50
    Replacement::draft(0x141154dd0, "unknown_141154dd0", fn_141154dd0 as *const ()), // sub_141154dd0
    Replacement::draft(0x141154e80, "unknown_141154e80", fn_141154e80 as *const ()), // sub_141154e80
    Replacement::draft(0x1411550f0, "unknown_1411550f0", fn_1411550f0 as *const ()), // sub_1411550f0
    Replacement::draft(0x141155150, "coregame_141155150", fn_141155150 as *const ()), // ecs::system::ecs::system::wrapper<coregame::brightness::system>
    Replacement::draft(0x141155420, "unknown_141155420", fn_141155420 as *const ()), // sub_141155420
    Replacement::draft(0x141155450, "unknown_141155450", fn_141155450 as *const ()), // sub_141155450
    Replacement::draft(0x141155550, "unknown_141155550", tpl_79667::<0x10, 0x38, 0x8, 0x1403b7340, 0x1401e66f0, 0x20, 0x18, 0x1401e6780> as *const ()), // sub_141155550
    Replacement::draft_throws(0x141155600, "std_141155600", fn_141155600 as *const (), Throws::Discard), // sub_141155600
    Replacement::draft_throws(0x141155910, "coregame_141155910", tpl_18900::<0x143ce2344, 0x143ce22d0, 0x1411e9f50, 0x14115a3f0, 0x143ce2698, 0x1420af120, 0x1420a28e0, 0x141159500, 0x1411582f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b9516, 0x142646960, 0x143ce22e0, 0x143ce22e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::bonefoley::update>
    Replacement::draft_throws(0x141155be0, "coregame_141155be0", tpl_1404::<0x143ce26e8, 0x143ce2590, 0x1411e9f50, 0x14115a590, 0x143ce26a0, 0x1420af120, 0x1420a28e0, 0x141159680, 0x141158390, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b9846, 0x142646960, 0x143ce25b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::bonefoley::updateHidden>
    Replacement::draft_throws(0x141155eb0, "coregame_141155eb0", tpl_3366::<0x143ce26ec, 0x143ce2448, 0x1411e9f50, 0x14115a2e0, 0x143ce22c4, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x1411597e0, 0x141158420, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b9b16, 0x142646960, 0x143ce2468, 0x143ce2478, 0x143ce247a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::bonefoley::updateBoneTrackedSoundObjects>
    Replacement::draft(0x1411561d0, "unknown_1411561d0", tpl_1526::<0x142948b20, 0x1410d6ca0, 0x141157340, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1411561d0
    Replacement::draft_throws(0x141156470, "std_141156470", tpl_822::<0x10, 0x38, 0x143426fc8, 0x1410f7af0> as *const (), Throws::Discard), // sub_141156470
    Replacement::draft(0x141156520, "unknown_141156520", tpl_1094::<0x143ce2638, 0x141157ff0, 0x14020e8f0> as *const ()), // sub_141156520
    Replacement::draft(0x141156720, "unknown_141156720", fn_141156720 as *const ()), // sub_141156720
    Replacement::draft(0x1411568c0, "unknown_1411568c0", fn_1411568c0 as *const ()), // sub_1411568c0
    Replacement::draft(0x141156a60, "unknown_141156a60", tpl_1436::<0x58, 0x60, 0x158, 0x143ce2638, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141156a60
    Replacement::draft(0x141156b40, "unknown_141156b40", tpl_1094::<0x143ce22c8, 0x141158170, 0x14020e8f0> as *const ()), // sub_141156b40
    Replacement::draft_throws(0x141156d40, "coregame_141156d40", tpl_1880::<0x143ce25f0, 0x143ce2220, 0x1411e9f50, 0x14115aec0, 0x143ce2634, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x141159e90, 0x141159320, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433b9d56, 0x142646960, 0x143ce2230, 0x143ce2238, 0x143ce223c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::bonefoley::streamIn>
    Replacement::draft_throws(0x141157050, "coregame_141157050", tpl_19790::<0x143ce26e4, 0x143ce22f0, 0x1411e9f50, 0x14115add0, 0x143ce269c, 0x1420af120, 0x1420a28e0, 0x14115a0e0, 0x141159460, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ba026, 0x142646960, 0x143ce2300, 0x143ce2308, 0x143ce230c, 0x143ce230e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::bonefoley::streamOut>
    Replacement::draft(0x141157340, "coregame_141157340", fn_141157340 as *const ()), // ecs::system::ecs::system::wrapper<coregame::bonefoley::removedBoneFoley>
    Replacement::draft(0x141157650, "coregame_141157650", fn_141157650 as *const ()), // sub_141157650
    Replacement::draft_throws(0x141157860, "coregame_141157860", tpl_19691::<0x143ce25f4, 0x143ce23a0, 0x141158550, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ba286, 0x142646960, 0x143ce23c0, 0x143ce23e0, 0x143ce23f0, 0x143ce23f4, 0x143ce23f6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::BoneFoleyComponent>
    Replacement::draft(0x141157aa0, "unknown_141157aa0", tpl_1094::<0x143ce22c0, 0x141158710, 0x14020e8f0> as *const ()), // sub_141157aa0
    Replacement::draft_throws(0x141157ca0, "std_141157ca0", fn_141157ca0 as *const (), Throws::Discard), // sub_141157ca0
    Replacement::draft(0x141157ef0, "unknown_141157ef0", tpl_60622::<0x10, 0x18, 0x143c9a620, 0x142489ee0, 0xc03, 0x143ce22c8, 0x20, 0x38, 0x40> as *const ()), // sub_141157ef0
    Replacement::draft(0x141157ff0, "unknown_141157ff0", tpl_6441::<0x1401e6d50, 0x1411592b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x141158890> as *const ()), // sub_141157ff0
    Replacement::draft(0x141158170, "unknown_141158170", tpl_20666::<0x1401e6d50, 0x1403ad7b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x141158900> as *const ()), // sub_141158170
    Replacement::draft(0x1411582f0, "unknown_1411582f0", tpl_21334::<0x88, 0x10088, 0x141158c80, 0x141fc51a0> as *const ()), // sub_1411582f0
    Replacement::draft_throws(0x141158390, "unknown_141158390", fn_141158390 as *const (), Throws::Discard), // sub_141158390
    Replacement::draft(0x141158420, "unknown_141158420", tpl_3203::<0x143c8d910, 0x142407c80, 0x141158b50, 0x141fc4c70> as *const ()), // sub_141158420
    Replacement::draft_throws(0x141158550, "unknown_141158550", tpl_3011::<0x1411e9f50, 0x14115ac40, 0x143ce247c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14115a270, 0x1401de410> as *const (), Throws::Discard), // sub_141158550
    Replacement::draft(0x141158710, "unknown_141158710", tpl_5321::<0x1401e6d50, 0x141159a30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x141155450> as *const ()), // sub_141158710
    Replacement::draft(0x141158890, "unknown_141158890", fn_141158890 as *const ()), // sub_141158890
    Replacement::draft(0x141158900, "unknown_141158900", fn_141158900 as *const ()), // sub_141158900
    Replacement::draft(0x141158a90, "unknown_141158a90", fn_141158a90 as *const ()), // sub_141158a90
    Replacement::draft(0x141159110, "unknown_141159110", tpl_21339::<0x1401ffc30, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_141159110
    Replacement::draft(0x1411592b0, "unknown_1411592b0", fn_1411592b0 as *const ()), // sub_1411592b0
    Replacement::draft_throws(0x141159320, "unknown_141159320", fn_141159320 as *const (), Throws::Caught), // sub_141159320
    Replacement::draft(0x141159460, "unknown_141159460", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x141156a60> as *const ()), // sub_141159460
    Replacement::draft(0x141159500, "unknown_141159500", tpl_79679::<0x141158d40, 0x141fc51a0> as *const ()), // sub_141159500
    Replacement::draft_throws(0x141159680, "unknown_141159680", fn_141159680 as *const (), Throws::Discard), // sub_141159680
    Replacement::draft(0x1411597e0, "unknown_1411597e0", tpl_3204::<0x143c8d910, 0x142407c80, 0x143ce22c0, 0x142489ee0, 0x143c8d120, 0x143c8ec20, 0x141fc4c70> as *const ()), // sub_1411597e0
    Replacement::draft(0x141159a30, "unknown_141159a30", tpl_1955::<0x48, 0x1404bdab0> as *const ()), // sub_141159a30
    Replacement::draft_throws(0x141159e90, "unknown_141159e90", fn_141159e90 as *const (), Throws::Caught), // sub_141159e90
    Replacement::draft(0x14115a0e0, "unknown_14115a0e0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141156a60> as *const ()), // sub_14115a0e0
    Replacement::draft(0x14115a1c0, "unknown_14115a1c0", tpl_3172::<0x143ce2638, 0x5e308, 0x143cada18, 0x580e0, 0x10, 0x8, 0x18, 0x141fc5130, 0x14115ab00> as *const ()), // sub_14115a1c0
    Replacement::draft(0x14115a270, "unknown_14115a270", tpl_3028::<0x143ce247c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141157650> as *const ()), // sub_14115a270
    Replacement::draft(0x14115a3f0, "unknown_14115a3f0", tpl_5986::<0x143ce2630, 0x143ce2640, 0x141834a78, 0x143cd6de0, 0x143ce2638, 0x143c8ec20, 0x143c8d87c, 0x143c8dbc8, 0x143c8d120, 0x143ce22c8, 0x143ce2660, 0x143ce2680, 0x143ce2690, 0x141834a0c> as *const ()), // sub_14115a3f0
    Replacement::draft(0x14115a720, "unknown_14115a720", tpl_3188::<0x14115ad30, 0x1433ba840, 0x14115ab00, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14115a720
    Replacement::draft(0x14115a9a0, "unknown_14115a9a0", fn_14115a9a0 as *const ()), // sub_14115a9a0
    Replacement::draft(0x14115ab00, "unknown_14115ab00", fn_14115ab00 as *const ()), // sub_14115ab00
    Replacement::draft(0x14115ad30, "unknown_14115ad30", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433ba840, 0x8, 0x20> as *const ()), // sub_14115ad30
    Replacement::draft(0x14115b460, "unknown_14115b460", fn_14115b460 as *const ()), // sub_14115b460
    Replacement::draft_throws(0x14115b510, "unknown_14115b510", fn_14115b510 as *const (), Throws::Discard), // sub_14115b510
    Replacement::draft(0x14115b630, "coregame_14115b630", fn_14115b630 as *const ()), // coregame::UIDataTransform::coregame::UIDataTransform::vfunc_0
    Replacement::draft(0x14115b680, "coregame_14115b680", fn_14115b680 as *const ()), // coregame::ui_data_transform::interpolate_localized_string::Transform::coregame::ui_data_transform::interpolate_localized
    Replacement::draft(0x14115b6f0, "coregame_14115b6f0", fn_14115b6f0 as *const ()), // sub_14115b6f0
    Replacement::draft_throws(0x14115b730, "coregame_14115b730", fn_14115b730 as *const (), Throws::Caught), // sub_14115b730
    Replacement::draft(0x14115b990, "coregame_14115b990", fn_14115b990 as *const ()), // coregame::ui_data_transform::tween::Transform::coregame::ui_data_transform::tween::Transform::vfunc_0
    Replacement::draft(0x14115ba00, "coregame_14115ba00", fn_14115ba00 as *const ()), // sub_14115ba00
    Replacement::draft(0x14115ba60, "coregame_14115ba60", tpl_21351::<0x8, 0x14115d120, 0x141834d30> as *const ()), // sub_14115ba60
    Replacement::draft(0x14115ba90, "coregame_14115ba90", fn_14115ba90 as *const ()), // coregame::BaseUIPage::coregame::BaseUIPage::vfunc_0
    Replacement::draft(0x14115bad0, "unknown_14115bad0", fn_14115bad0 as *const ()), // sub_14115bad0
    Replacement::draft(0x14115bb60, "unknown_14115bb60", fn_14115bb60 as *const ()), // sub_14115bb60
    Replacement::draft(0x14115bbc0, "unknown_14115bbc0", fn_14115bbc0 as *const ()), // sub_14115bbc0
    Replacement::draft(0x14115bc20, "unknown_14115bc20", tpl_2030::<0x14115bcd0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14115bc20
    Replacement::draft(0x14115bcd0, "unknown_14115bcd0", tpl_19159::<0x8, 0x28, 0x14115d8c0, 0x14240ab10> as *const ()), // sub_14115bcd0
    Replacement::draft_throws(0x14115bd40, "unknown_14115bd40", fn_14115bd40 as *const (), Throws::Caught), // sub_14115bd40
    Replacement::draft_throws(0x14115bdd0, "unknown_14115bdd0", fn_14115bdd0 as *const (), Throws::Caught), // sub_14115bdd0
    Replacement::draft(0x14115be90, "unknown_14115be90", tpl_21357::<0x1438812b8, 0x14243b0b0, 0x143882290, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14115be90
    Replacement::draft_throws(0x14115bf30, "unknown_14115bf30", tpl_21358::<0x1438812b0, 0x1438812b8, 0x1401e9fa0, 0x143882288, 0x143882290, 0x1401e9e70> as *const (), Throws::Discard), // sub_14115bf30
    Replacement::draft_throws(0x14115bfb0, "unknown_14115bfb0", tpl_21358::<0x1438812b0, 0x1438812b8, 0x1401e9b60, 0x143882288, 0x143882290, 0x1401e9a20> as *const (), Throws::Discard), // sub_14115bfb0
    Replacement::draft_throws(0x14115c030, "unknown_14115c030", fn_14115c030 as *const (), Throws::Caught), // sub_14115c030
    Replacement::draft(0x14115c0c0, "unknown_14115c0c0", tpl_21359::<0x1438812b0, 0x1438812b8, 0x1401e9970, 0x143882288, 0x143882290, 0x1401e9860> as *const ()), // sub_14115c0c0
    Replacement::draft_throws(0x14115c140, "unknown_14115c140", tpl_21358::<0x1438812b0, 0x1438812b8, 0x1401e9770, 0x143882288, 0x143882290, 0x1401e9590> as *const (), Throws::Discard), // sub_14115c140
    Replacement::draft_throws(0x14115c1c0, "unknown_14115c1c0", tpl_1905::<0x1438812b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143882290> as *const (), Throws::Discard), // sub_14115c1c0
    Replacement::draft(0x14115c270, "unknown_14115c270", tpl_21360::<0x40, 0x10, 0x18, 0x14115cca0, 0x1433baa48, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_14115c270
    Replacement::draft(0x14115c330, "unknown_14115c330", tpl_21360::<0x40, 0x10, 0x18, 0x14115cca0, 0x1433baa90, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_14115c330
    Replacement::draft_throws(0x14115c3f0, "unknown_14115c3f0", fn_14115c3f0 as *const (), Throws::Caught), // sub_14115c3f0
    Replacement::draft(0x14115c530, "unknown_14115c530", fn_14115c530 as *const ()), // sub_14115c530
    Replacement::draft(0x14115c630, "unknown_14115c630", fn_14115c630 as *const ()), // sub_14115c630
    Replacement::draft(0x14115cca0, "unknown_14115cca0", fn_14115cca0 as *const ()), // sub_14115cca0
    Replacement::draft_throws(0x14115cd10, "unknown_14115cd10", fn_14115cd10 as *const (), Throws::Caught), // sub_14115cd10
    Replacement::draft(0x14115d120, "unknown_14115d120", fn_14115d120 as *const ()), // sub_14115d120
    Replacement::draft(0x14115d1b0, "unknown_14115d1b0", fn_14115d1b0 as *const ()), // sub_14115d1b0
    Replacement::draft(0x14115d210, "unknown_14115d210", fn_14115d210 as *const ()), // sub_14115d210
    Replacement::draft(0x14115d2c0, "cohtml_14115d2c0", fn_14115d2c0 as *const ()), // cohtml::EventHandler<const char* (coregame::BaseUIPage::*)(unsigned int)>::cohtml::EventHandler<const char* (coregame::B
    Replacement::draft(0x14115d350, "cohtml_14115d350", fn_14115d350 as *const ()), // cohtml::EventHandler<unsigned int (ui::Page::*)(const r::InplaceString<128>&)>::cohtml::EventHandler<unsigned int (ui::P
    Replacement::draft_throws(0x14115d3f0, "unknown_14115d3f0", fn_14115d3f0 as *const (), Throws::Caught), // sub_14115d3f0
    Replacement::draft_throws(0x14115d480, "unknown_14115d480", tpl_1908::<0x143882290, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_14115d480
    Replacement::draft(0x14115d500, "unknown_14115d500", fn_14115d500 as *const ()), // sub_14115d500
    Replacement::draft(0x14115d7e0, "unknown_14115d7e0", fn_14115d7e0 as *const ()), // sub_14115d7e0
    Replacement::draft(0x14115d820, "std_14115d820", tpl_1::<0x143891b10> as *const ()), // std::_Func_impl_no_alloc<coregame::ui_data_transform::tween::Transform::<lambda_140_>,double,double,double,double,double
    Replacement::draft(0x14115d830, "std_14115d830", fn_14115d830 as *const ()), // std::_Func_impl_no_alloc<coregame::ui_data_transform::tween::Transform::<lambda_140_>,double,double,double,double,double
    Replacement::draft(0x14115d860, "std_14115d860", tpl_770::<0x1433baaa8> as *const ()), // std::_Func_impl_no_alloc<coregame::ui_data_transform::tween::Transform::<lambda_140_>,double,double,double,double,double
    Replacement::draft(0x14115d870, "std_14115d870", tpl_1::<0x143891ac0> as *const ()), // std::_Func_impl_no_alloc<coregame::ui_data_transform::tween::Transform::<lambda_139_>,double,double,double,double,double
    Replacement::draft(0x14115d880, "std_14115d880", fn_14115d880 as *const ()), // std::_Func_impl_no_alloc<coregame::ui_data_transform::tween::Transform::<lambda_139_>,double,double,double,double,double
    Replacement::draft(0x14115d8a0, "std_14115d8a0", tpl_770::<0x1433baae0> as *const ()), // std::_Func_impl_no_alloc<coregame::ui_data_transform::tween::Transform::<lambda_139_>,double,double,double,double,double
    Replacement::draft(0x14115d8b0, "unknown_14115d8b0", tpl_170::<0x14115d8c0> as *const ()), // sub_14115d8b0
    Replacement::draft(0x14115d8c0, "unknown_14115d8c0", tpl_808::<0x14115d970, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14115d8c0
    Replacement::draft(0x14115d970, "unknown_14115d970", fn_14115d970 as *const ()), // sub_14115d970
    Replacement::draft(0x14115d9e0, "coregame_14115d9e0", fn_14115d9e0 as *const ()), // sub_14115d9e0
    Replacement::draft(0x14115da00, "coregame_14115da00", fn_14115da00 as *const ()), // sub_14115da00
    Replacement::draft(0x14115da90, "coregame_14115da90", fn_14115da90 as *const ()), // sub_14115da90
    Replacement::draft(0x14115dfe0, "coregame_14115dfe0", tpl_1171::<0x58, 0x60, 0x143cc6a30, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14115dfe0
    Replacement::draft(0x14115e0c0, "coregame_14115e0c0", tpl_1171::<0x58, 0x60, 0x143cccac8, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14115e0c0
    Replacement::draft(0x14115e1a0, "coregame_14115e1a0", tpl_1171::<0x58, 0x60, 0x143ce2aa8, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14115e1a0
    Replacement::draft_throws(0x14115e280, "coregame_14115e280", tpl_1480::<0x142949f50, 0x1410d6ca0, 0x141161fb0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14115e280
    Replacement::draft_throws(0x14115e520, "coregame_14115e520", tpl_1886::<0x143ce3038, 0x143ce2948, 0x1411e9f50, 0x141167210, 0x143ce3154, 0x1420af120, 0x1420a28e0, 0x1411651a0, 0x141163de0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bb926, 0x142646960, 0x143ce2968, 0x143ce296c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateAudioTransform>
    Replacement::draft_throws(0x14115e800, "coregame_14115e800", tpl_1102::<0x143ce2970, 0x143ce2f08, 0x1411e9f50, 0x1411670d0, 0x143ce2abc, 0x1420af120, 0x1420a28e0, 0x1411653b0, 0x141163f00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bbbc6, 0x142646960, 0x143ce2f28, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateVolumetricAudioTransform>
    Replacement::draft_throws(0x14115eae0, "coregame_14115eae0", tpl_1185::<0x143ce31a8, 0x143ce2bd8, 0x1411e9f50, 0x141164020, 0x1411655c0, 0x1411641a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bbe66, 0x142646960, 0x143ce2bf8, 0x143ce2c00, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updatePositionAndAcoustics>
    Replacement::draft_throws(0x14115ed30, "coregame_14115ed30", tpl_3006::<0x143ce2e6c, 0x143ce2a20, 0x1411622d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bc4f6, 0x142646960, 0x143ce2a40, 0x143ce2a48, 0x143ce2a4c, 0x143ce2a4e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::processInactiveSourceCommands>
    Replacement::draft_throws(0x14115ef50, "coregame_14115ef50", tpl_4835::<0x143ce31b0, 0x143ce2830, 0x1411e9f50, 0x1411668b0, 0x143ce2a98, 0x1420af120, 0x1411649f0, 0x1420a28e0, 0x1411658f0, 0x141164460, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bcc96, 0x142646960, 0x143ce2850, 0x143ce2858, 0x143ce285c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::processActiveSourceCommands>
    Replacement::draft_throws(0x14115f250, "coregame_14115f250", fn_14115f250 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::processGlobalCommands>
    Replacement::draft_throws(0x14115f560, "coregame_14115f560", tpl_4144::<0x143ce2b00, 0x143ce2c08, 0x1411e9f50, 0x1411667c0, 0x143ce2dcc, 0x1420af120, 0x140215750, 0x1420a28e0, 0x141165b10, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bd2e6, 0x142646960, 0x143ce2c28, 0x143ce2c30, 0x143ce2c34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::assignAcousticsUpdateIndices>
    Replacement::draft_throws(0x14115f880, "coregame_14115f880", fn_14115f880 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateObstructionAndOcclusionOverrides>
    Replacement::draft(0x14115fba0, "coregame_14115fba0", tpl_1526::<0x142949f50, 0x1410d6ca0, 0x141162ac0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14115fba0
    Replacement::draft(0x14115fe40, "coregame_14115fe40", tpl_1526::<0x142949f50, 0x1410d6ca0, 0x141162d60, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14115fe40
    Replacement::draft(0x1411600e0, "coregame_1411600e0", tpl_1368::<0x143ca83d8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1411600e0
    Replacement::draft(0x141160410, "coregame_141160410", tpl_1435::<0x143cccac4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141160410
    Replacement::draft(0x141160760, "coregame_141160760", tpl_1094::<0x143cc8944, 0x141163400, 0x14020e8f0> as *const ()), // sub_141160760
    Replacement::draft(0x141160960, "coregame_141160960", tpl_1094::<0x143ce2aa4, 0x140fc09c0, 0x14020e8f0> as *const ()), // sub_141160960
    Replacement::draft(0x141160b60, "coregame_141160b60", tpl_4390::<0x143ce2ce8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141160b60
    Replacement::draft(0x141160e90, "coregame_141160e90", tpl_1095::<0x143cc6a30, 0x14020e8f0> as *const ()), // sub_141160e90
    Replacement::draft(0x141161070, "coregame_141161070", tpl_1094::<0x143ce2b9c, 0x141163580, 0x14020e8f0> as *const ()), // sub_141161070
    Replacement::draft(0x141161270, "coregame_141161270", tpl_1094::<0x143cba178, 0x140492c90, 0x14020e8f0> as *const ()), // sub_141161270
    Replacement::draft(0x141161470, "coregame_141161470", tpl_1094::<0x143ce3158, 0x141163710, 0x14020e8f0> as *const ()), // sub_141161470
    Replacement::draft(0x141161670, "coregame_141161670", tpl_1094::<0x143cd6de0, 0x1411638a0, 0x14020e8f0> as *const ()), // sub_141161670
    Replacement::draft(0x141161870, "coregame_141161870", tpl_1094::<0x143ce2434, 0x141163a50, 0x14020e8f0> as *const ()), // sub_141161870
    Replacement::draft(0x141161a70, "coregame_141161a70", tpl_1094::<0x143cccac8, 0x141163c50, 0x14020e8f0> as *const ()), // sub_141161a70
    Replacement::draft(0x141161c70, "coregame_141161c70", tpl_3270::<0x143ce2aa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141161c70
    Replacement::draft_throws(0x141161fb0, "coregame_141161fb0", fn_141161fb0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::initMaterials>
    Replacement::draft_throws(0x1411622d0, "coregame_1411622d0", fn_1411622d0 as *const (), Throws::Discard), // sub_1411622d0
    Replacement::draft_throws(0x1411624a0, "coregame_1411624a0", fn_1411624a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamIn>
    Replacement::draft_throws(0x1411627c0, "coregame_1411627c0", tpl_19638::<0x143ce2dc0, 0x143ce2d88, 0x1411e9f50, 0x1411680b0, 0x143ce2aa0, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1411663f0, 0x141165cf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bb4b6, 0x142646960, 0x143ce2d98, 0x143ce2da0, 0x143ce2da2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamOut>
    Replacement::draft(0x141162ac0, "coregame_141162ac0", fn_141162ac0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::audio::removedAudioObject>
    Replacement::draft(0x141162d60, "coregame_141162d60", fn_141162d60 as *const ()), // ecs::system::ecs::system::wrapper<coregame::audio::removedAudioSubSources>
    Replacement::draft(0x1411630f0, "unknown_1411630f0", fn_1411630f0 as *const ()), // sub_1411630f0
    Replacement::draft(0x141163240, "unknown_141163240", fn_141163240 as *const ()), // sub_141163240
    Replacement::draft(0x141163400, "unknown_141163400", fn_141163400 as *const ()), // sub_141163400
    Replacement::draft(0x141163580, "unknown_141163580", tpl_3411::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143291380> as *const ()), // sub_141163580
    Replacement::draft(0x141163710, "unknown_141163710", fn_141163710 as *const ()), // sub_141163710
    Replacement::draft(0x1411638a0, "unknown_1411638a0", fn_1411638a0 as *const ()), // sub_1411638a0
    Replacement::draft(0x141163a50, "unknown_141163a50", fn_141163a50 as *const ()), // sub_141163a50
    Replacement::draft(0x141163c50, "unknown_141163c50", fn_141163c50 as *const ()), // sub_141163c50
    Replacement::draft(0x141163de0, "unknown_141163de0", tpl_62730::<0x88, 0x10088, 0x141164fe0, 0x141fcb870> as *const ()), // sub_141163de0
    Replacement::draft(0x141163f00, "unknown_141163f00", tpl_62730::<0x88, 0x10088, 0x141164eb0, 0x141fcb360> as *const ()), // sub_141163f00
    Replacement::draft_throws(0x141164020, "unknown_141164020", fn_141164020 as *const (), Throws::Discard), // sub_141164020
    Replacement::draft_throws(0x141164460, "unknown_141164460", tpl_19641::<0x88, 0x10088, 0x141164750, 0x141164500> as *const (), Throws::Discard), // sub_141164460
    Replacement::draft_throws(0x141164500, "unknown_141164500", tpl_4138::<0x18, 0x20, 0x40, 0x8, 0x141fcbf10> as *const (), Throws::Discard), // sub_141164500
    Replacement::draft(0x141164610, "unknown_141164610", tpl_4168::<0x143c8d910, 0x142407c80, 0x143ce2aa8, 0x142489ee0, 0x141fca7b0> as *const ()), // sub_141164610
    Replacement::draft_throws(0x1411649f0, "unknown_1411649f0", fn_1411649f0 as *const (), Throws::Discard), // sub_1411649f0
    Replacement::draft(0x141164af0, "unknown_141164af0", tpl_3150::<0x18, 0x10, 0x8, 0x141164bd0, 0x1402716c0, 0x20, 0x40, 0x60, 0x80> as *const ()), // sub_141164af0
    Replacement::draft(0x141165110, "unknown_141165110", fn_141165110 as *const ()), // sub_141165110
    Replacement::draft(0x141165170, "unknown_141165170", tpl_1955::<0x18, 0x1410767d0> as *const ()), // sub_141165170
    Replacement::draft(0x1411651a0, "unknown_1411651a0", tpl_62741::<0x143c8d120, 0x142489ee0, 0x143cccac4, 0x143ce2b9c, 0x141fcb870> as *const ()), // sub_1411651a0
    Replacement::draft(0x1411653b0, "unknown_1411653b0", tpl_62741::<0x143c8d120, 0x142489ee0, 0x143cc6a20, 0x143cccac4, 0x141fcb360> as *const ()), // sub_1411653b0
    Replacement::draft_throws(0x1411655c0, "unknown_1411655c0", fn_1411655c0 as *const (), Throws::Discard), // sub_1411655c0
    Replacement::draft_throws(0x1411658f0, "unknown_1411658f0", tpl_78029::<0x141164830, 0x141fcbf10> as *const (), Throws::Discard), // sub_1411658f0
    Replacement::draft_throws(0x141165b10, "unknown_141165b10", tpl_5214::<0x143ce2dcc, 0x20, 0x8, 0x141fcafe0> as *const (), Throws::Discard), // sub_141165b10
    Replacement::draft(0x141165b70, "unknown_141165b70", tpl_4169::<0x143c8d910, 0x142407c80, 0x143ce2aa8, 0x142489ee0, 0x141fca7b0> as *const ()), // sub_141165b70
    Replacement::draft_throws(0x141165cf0, "unknown_141165cf0", tpl_19954::<0x88, 0x10088, 0x141166250, 0x141fcd590> as *const (), Throws::Discard), // sub_141165cf0
    Replacement::draft(0x141166390, "unknown_141166390", tpl_1217::<0x143ce2aac, 0x20, 0x8, 0x141fcdee0> as *const ()), // sub_141166390
    Replacement::draft_throws(0x1411663f0, "unknown_1411663f0", tpl_78312::<0x143ca83d8, 0x142489ee0, 0x143ce2aa4, 0x143cd6de0, 0x141fcd590> as *const (), Throws::Discard), // sub_1411663f0
    Replacement::draft_throws(0x1411665f0, "unknown_1411665f0", tpl_19619::<0x143ca83d8, 0x5e308, 0x10, 0x8, 0x18, 0x141fca8b0, 0x140489dd0> as *const (), Throws::Discard), // sub_1411665f0
    Replacement::draft_throws(0x141166680, "unknown_141166680", tpl_19619::<0x143cd6de0, 0x5e308, 0x10, 0x8, 0x18, 0x141fca830, 0x141167800> as *const (), Throws::Discard), // sub_141166680
    Replacement::draft(0x1411668b0, "unknown_1411668b0", fn_1411668b0 as *const ()), // sub_1411668b0
    Replacement::draft(0x141166ac0, "unknown_141166ac0", fn_141166ac0 as *const ()), // sub_141166ac0
    Replacement::draft(0x141166de0, "unknown_141166de0", fn_141166de0 as *const ()), // sub_141166de0
    Replacement::draft(0x141167350, "unknown_141167350", tpl_3188::<0x141167df0, 0x1433bf320, 0x141167800, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141167350
    Replacement::draft(0x1411673d0, "unknown_1411673d0", tpl_3188::<0x141167e90, 0x1433bf2e8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1411673d0
    Replacement::draft(0x141167450, "unknown_141167450", tpl_19620::<0x8, 0x141167800> as *const ()), // sub_141167450
    Replacement::draft(0x141167670, "unknown_141167670", fn_141167670 as *const ()), // sub_141167670
    Replacement::draft(0x141167800, "unknown_141167800", tpl_4352::<0x1401de830, 0x1410767d0, 0x1425371a0, 0x142533200> as *const ()), // sub_141167800
    Replacement::draft(0x141167920, "unknown_141167920", fn_141167920 as *const ()), // sub_141167920
    Replacement::draft(0x141167b60, "unknown_141167b60", fn_141167b60 as *const ()), // sub_141167b60
    Replacement::draft(0x141167df0, "unknown_141167df0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433bf320, 0x8, 0x20> as *const ()), // sub_141167df0
    Replacement::draft(0x141167e90, "unknown_141167e90", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433bf2e8, 0x8, 0x20> as *const ()), // sub_141167e90
    Replacement::draft(0x141168700, "unknown_141168700", fn_141168700 as *const ()), // sub_141168700
    Replacement::draft_throws(0x141168790, "coregame_141168790", fn_141168790 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateActiveListener>
    Replacement::draft(0x141168ae0, "unknown_141168ae0", tpl_6190::<0x141fd0ed0> as *const ()), // sub_141168ae0
    Replacement::draft(0x141168af0, "unknown_141168af0", fn_141168af0 as *const ()), // sub_141168af0
    Replacement::draft_throws(0x141168b90, "std_141168b90", fn_141168b90 as *const (), Throws::Discard), // sub_141168b90
    Replacement::draft_throws(0x141168d60, "std_141168d60", fn_141168d60 as *const (), Throws::Discard), // sub_141168d60
    Replacement::draft(0x141169060, "coregame_141169060", fn_141169060 as *const ()), // coregame::audio_collision_listener::DummyContactListener::coregame::audio_collision_listener::DummyContactListener::vfun
    Replacement::draft(0x141169080, "coregame_141169080", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433c13e0> as *const ()), // sub_141169080
    Replacement::draft(0x1411690e0, "coregame_1411690e0", tpl_5915::<0x8, 0x18, 0x1401f2bc0, 0x1401e66f0> as *const ()), // sub_1411690e0
    Replacement::draft(0x141169150, "coregame_141169150", fn_141169150 as *const ()), // sub_141169150
    Replacement::draft(0x141169220, "coregame_141169220", tpl_79667::<0x10, 0x38, 0x8, 0x1411692d0, 0x1401e66f0, 0x20, 0x18, 0x1401e6780> as *const ()), // sub_141169220
    Replacement::draft(0x1411692d0, "coregame_1411692d0", fn_1411692d0 as *const ()), // sub_1411692d0
    Replacement::draft(0x141169470, "coregame_141169470", fn_141169470 as *const ()), // sub_141169470
    Replacement::draft(0x141169610, "coregame_141169610", fn_141169610 as *const ()), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::overrides::removed>
    Replacement::draft(0x141169920, "coregame_141169920", fn_141169920 as *const ()), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::readContactReports>
    Replacement::draft(0x141169c00, "coregame_141169c00", fn_141169c00 as *const ()), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::updateWithListener>
    Replacement::draft_throws(0x141169eb0, "std_141169eb0", tpl_21411::<0x143c488b0, 0x143c48898, 0x143c48888, 0x143c48890, 0x14307e950, 0x142645870, 0x1401fc200, 0x143c488b8, 0x143c48880, 0x143426fc8, 0x14116a870, 0x141169fc8> as *const (), Throws::Discard), // sub_141169eb0
    Replacement::draft_throws(0x14116a1b0, "std_14116a1b0", tpl_21411::<0x143c489c0, 0x143c489a8, 0x143c48998, 0x143c489a0, 0x14307e950, 0x142645870, 0x1401fc200, 0x143c489c8, 0x143c48990, 0x143426fc8, 0x14116aa60, 0x14116a2c8> as *const (), Throws::Discard), // sub_14116a1b0
    Replacement::draft_throws(0x14116a4b0, "std_14116a4b0", fn_14116a4b0 as *const (), Throws::Discard), // sub_14116a4b0
    Replacement::draft(0x14116a6e0, "std_14116a6e0", fn_14116a6e0 as *const ()), // sub_14116a6e0
    Replacement::draft(0x14116a750, "std_14116a750", fn_14116a750 as *const ()), // sub_14116a750
    Replacement::draft(0x14116a840, "std_14116a840", tpl_5918::<0x8, 0x1401f2bc0, 0x1401fb4d0> as *const ()), // sub_14116a840
    Replacement::draft_throws(0x14116a870, "std_14116a870", tpl_21415::<0x14307e970, 0x142645870, 0x143c48888, 0x143c48898, 0x1401e5730, 0x143c488b0, 0x143c488b8, 0x14116a8aa> as *const (), Throws::Discard), // sub_14116a870
    Replacement::draft_throws(0x14116aa60, "std_14116aa60", tpl_21415::<0x14307e970, 0x142645870, 0x143c48998, 0x143c489a8, 0x1401e5730, 0x143c489c0, 0x143c489c8, 0x14116aa9a> as *const (), Throws::Discard), // sub_14116aa60
    Replacement::draft_throws(0x14116ac50, "std_14116ac50", fn_14116ac50 as *const (), Throws::Discard), // sub_14116ac50
    Replacement::draft(0x14116adf0, "std_14116adf0", fn_14116adf0 as *const ()), // sub_14116adf0
    Replacement::draft(0x14116ae00, "std_14116ae00", fn_14116ae00 as *const ()), // sub_14116ae00
    Replacement::draft(0x14116afe0, "std_14116afe0", tpl_1094::<0x143ce33c0, 0x14116dc50, 0x14020e8f0> as *const ()), // sub_14116afe0
    Replacement::draft_throws(0x14116b1e0, "std_14116b1e0", fn_14116b1e0 as *const (), Throws::Discard), // sub_14116b1e0
    Replacement::draft_throws(0x14116b4a0, "std_14116b4a0", fn_14116b4a0 as *const (), Throws::Discard), // sub_14116b4a0
    Replacement::draft(0x14116b790, "std_14116b790", tpl_1095::<0x143ce3808, 0x14020e8f0> as *const ()), // sub_14116b790
    Replacement::draft(0x14116b970, "std_14116b970", tpl_1436::<0x58, 0x60, 0x158, 0x143ce3808, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14116b970
    Replacement::draft(0x14116ba50, "std_14116ba50", tpl_1094::<0x143ce34d4, 0x14116de60, 0x14020e8f0> as *const ()), // sub_14116ba50
    Replacement::draft_throws(0x14116bc50, "std_14116bc50", fn_14116bc50 as *const (), Throws::Discard), // sub_14116bc50
    Replacement::draft(0x14116bf30, "unknown_14116bf30", tpl_1095::<0x143ce3694, 0x14020e8f0> as *const ()), // sub_14116bf30
    Replacement::draft(0x14116c110, "unknown_14116c110", tpl_3593::<0x1401e7c10, 0x14116db50, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_14116c110
    Replacement::draft_throws(0x14116c320, "coregame_14116c320", fn_14116c320 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class contentcore::ContactSoundMat
    Replacement::draft(0x14116c560, "coregame_14116c560", tpl_20412::<0x1401e7c10, 0x14116da50, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_14116c560
    Replacement::draft_throws(0x14116c770, "coregame_14116c770", fn_14116c770 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class contentcore::ContactSoundOve
    Replacement::draft_throws(0x14116c9a0, "coregame_14116c9a0", tpl_3242::<0x143ce3650, 0x143ce3600, 0x1411e9f50, 0x1411700e0, 0x143ce371c, 0x1420af120, 0x143ce35f4, 0x140215610, 0x1420a28e0, 0x14116f750, 0x14116e420, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c1026, 0x142646960, 0x143ce3620, 0x143ce3628, 0x143ce362c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::streamIn>
    Replacement::draft_throws(0x14116ccc0, "coregame_14116ccc0", tpl_1076::<0x143ce3278, 0x143ce3540, 0x1411e9f50, 0x14116fff0, 0x143ce3438, 0x1420af120, 0x143ce35f4, 0x140215610, 0x1420a28e0, 0x14116f8d0, 0x14116e560, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c0e16, 0x142646960, 0x143ce3560, 0x143ce3568, 0x143ce356c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::streamOut>
    Replacement::draft_throws(0x14116cff0, "coregame_14116cff0", tpl_3277::<0x143ce38f0, 0x143ce36a0, 0x1411e9f50, 0x14116fef0, 0x143ce38bc, 0x1420af120, 0x143ce35f4, 0x140215610, 0x1420a28e0, 0x14116f9b0, 0x14116e5f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c0066, 0x142646960, 0x143ce36c0, 0x143ce36d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::overrides::streamIn>
    Replacement::draft_throws(0x14116d310, "coregame_14116d310", tpl_4895::<0x143ce380c, 0x143ce3280, 0x1411e9f50, 0x14116fdf0, 0x143ce35f8, 0x1420af120, 0x143ce35f4, 0x140215610, 0x1420a28e0, 0x14116fb30, 0x14116e730, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433bfde6, 0x142646960, 0x143ce32a0, 0x143ce32b0, 0x143ce32b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_collision_listener::overrides::streamOut>
    Replacement::draft(0x14116d690, "unknown_14116d690", tpl_5918::<0x8, 0x14116e9b0, 0x1401fb4d0> as *const ()), // sub_14116d690
    Replacement::draft(0x14116d6c0, "unknown_14116d6c0", fn_14116d6c0 as *const ()), // sub_14116d6c0
    Replacement::draft_throws(0x14116d710, "std_14116d710", fn_14116d710 as *const (), Throws::Discard), // sub_14116d710
    Replacement::draft_throws(0x14116d8c0, "std_14116d8c0", fn_14116d8c0 as *const (), Throws::Discard), // sub_14116d8c0
    Replacement::draft(0x14116da50, "unknown_14116da50", tpl_60622::<0x10, 0x18, 0x143c9a620, 0x142489ee0, 0xc03, 0x143ce34d4, 0x20, 0x38, 0x40> as *const ()), // sub_14116da50
    Replacement::draft(0x14116db50, "unknown_14116db50", tpl_60622::<0x10, 0x18, 0x143c9a620, 0x142489ee0, 0xc03, 0x143ce33c0, 0x20, 0x38, 0x40> as *const ()), // sub_14116db50
    Replacement::draft(0x14116dc50, "unknown_14116dc50", fn_14116dc50 as *const ()), // sub_14116dc50
    Replacement::draft(0x14116de60, "unknown_14116de60", fn_14116de60 as *const ()), // sub_14116de60
    Replacement::draft_throws(0x14116e0a0, "unknown_14116e0a0", tpl_3011::<0x1411e9f50, 0x1411702d0, 0x143ce36d8, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14116f670, 0x1401de410> as *const (), Throws::Discard), // sub_14116e0a0
    Replacement::draft_throws(0x14116e260, "unknown_14116e260", tpl_3011::<0x1411e9f50, 0x1411701e0, 0x143ce3448, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14116f6e0, 0x1401de410> as *const (), Throws::Discard), // sub_14116e260
    Replacement::draft_throws(0x14116e420, "unknown_14116e420", fn_14116e420 as *const (), Throws::Caught), // sub_14116e420
    Replacement::draft(0x14116e560, "unknown_14116e560", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x14116b970> as *const ()), // sub_14116e560
    Replacement::draft_throws(0x14116e5f0, "unknown_14116e5f0", fn_14116e5f0 as *const (), Throws::Caught), // sub_14116e5f0
    Replacement::draft(0x14116e730, "unknown_14116e730", fn_14116e730 as *const ()), // sub_14116e730
    Replacement::draft(0x14116e800, "unknown_14116e800", tpl_3172::<0x143ce34d4, 0x5e308, 0x143ce35f4, 0x580e0, 0x10, 0x8, 0x18, 0x141fd2a40, 0x14116ee30> as *const ()), // sub_14116e800
    Replacement::draft(0x14116e900, "unknown_14116e900", tpl_3188::<0x14116fc50, 0x1433c13a8, 0x14116ee30, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14116e900
    Replacement::draft(0x14116e9b0, "unknown_14116e9b0", fn_14116e9b0 as *const ()), // sub_14116e9b0
    Replacement::draft(0x14116ec20, "unknown_14116ec20", fn_14116ec20 as *const ()), // sub_14116ec20
    Replacement::draft(0x14116ecc0, "unknown_14116ecc0", fn_14116ecc0 as *const ()), // sub_14116ecc0
    Replacement::draft(0x14116ee30, "unknown_14116ee30", fn_14116ee30 as *const ()), // sub_14116ee30
    Replacement::draft(0x14116f330, "unknown_14116f330", fn_14116f330 as *const ()), // sub_14116f330
    Replacement::draft(0x14116f420, "unknown_14116f420", fn_14116f420 as *const ()), // sub_14116f420
    Replacement::draft(0x14116f550, "unknown_14116f550", fn_14116f550 as *const ()), // sub_14116f550
    Replacement::draft(0x14116f670, "unknown_14116f670", tpl_3028::<0x143ce36d8, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14116c110> as *const ()), // sub_14116f670
    Replacement::draft(0x14116f6e0, "unknown_14116f6e0", tpl_3028::<0x143ce3448, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14116c560> as *const ()), // sub_14116f6e0
    Replacement::draft_throws(0x14116f750, "unknown_14116f750", fn_14116f750 as *const (), Throws::Caught), // sub_14116f750
    Replacement::draft(0x14116f8d0, "unknown_14116f8d0", tpl_6412::<0x8, 0x142407c80, 0x88, 0x10088, 0x10, 0x580f0, 0x14116b970> as *const ()), // sub_14116f8d0
    Replacement::draft_throws(0x14116f9b0, "unknown_14116f9b0", fn_14116f9b0 as *const (), Throws::Caught), // sub_14116f9b0
    Replacement::draft(0x14116fb30, "unknown_14116fb30", fn_14116fb30 as *const ()), // sub_14116fb30
    Replacement::draft(0x14116fc50, "unknown_14116fc50", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433c13a8, 0x8, 0x20> as *const ()), // sub_14116fc50
    Replacement::draft(0x14116fd10, "unknown_14116fd10", tpl_61657::<0x8, 0x18, 0x14116e9b0, 0x1401e66f0> as *const ()), // sub_14116fd10
    Replacement::draft(0x14116fd80, "unknown_14116fd80", tpl_4781::<0x8, 0x10, 0x1411511c0, 0x18, 0x30, 0x38, 0x1401e5730> as *const ()), // sub_14116fd80
    Replacement::draft_throws(0x141170890, "coregame_141170890", fn_141170890 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::attachment_socket::localTransformsFromSkeleton>
    Replacement::draft_throws(0x141170b50, "coregame_141170b50", fn_141170b50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::attachment_socket::localTransformsFromPhysics>
    Replacement::draft(0x141170ed0, "unknown_141170ed0", tpl_5200::<0x143cd4460, 0x143c94b44, 0x143c94b30, 0x1411715c0, 0x141171110> as *const ()), // sub_141170ed0
    Replacement::draft(0x141171030, "unknown_141171030", tpl_5201::<0x8, 0x20, 0x143cd4460, 0x143c94b44, 0x143c94b30> as *const ()), // sub_141171030
    Replacement::draft(0x141171110, "unknown_141171110", tpl_5202::<0x18, 0x20, 0x40, 0x10, 0x28, 0x48, 0x30, 0x8, 0x141fd6a00> as *const ()), // sub_141171110
    Replacement::draft(0x141171240, "unknown_141171240", fn_141171240 as *const ()), // sub_141171240
    Replacement::draft(0x1411713a0, "unknown_1411713a0", tpl_19090::<0x8, 0x20, 0x143ce3960, 0x143c94b44, 0x143c94b30> as *const ()), // sub_1411713a0
    Replacement::draft(0x141171480, "unknown_141171480", fn_141171480 as *const ()), // sub_141171480
    Replacement::draft(0x141171670, "unknown_141171670", tpl_60875::<0x143c95048, 0x142489ee0, 0x143c92bcc, 0x143c933f0, 0x143c8d120, 0x143c93358, 0x143c8f6d8> as *const ()), // sub_141171670
    Replacement::draft_throws(0x1411717e0, "unknown_1411717e0", tpl_1331::<0x8, 0x141171e20, 0x143ce3a40, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_1411717e0
    Replacement::draft(0x141171d10, "unknown_141171d10", fn_141171d10 as *const ()), // sub_141171d10
    Replacement::draft(0x141171e20, "unknown_141171e20", fn_141171e20 as *const ()), // sub_141171e20
    Replacement::draft_throws(0x141171fa0, "unknown_141171fa0", tpl_4541::<0x143ce3960, 0x141171d10, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141171fa0
    Replacement::draft_throws(0x141172030, "unknown_141172030", tpl_4541::<0x143ce3a40, 0x141171e20, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_141172030
    Replacement::draft(0x1411720c0, "unknown_1411720c0", tpl_365::<0x141172650> as *const ()), // sub_1411720c0
    Replacement::draft(0x1411720d0, "unknown_1411720d0", tpl_365::<0x1411724d0> as *const ()), // sub_1411720d0
    Replacement::draft(0x1411720e0, "unknown_1411720e0", fn_1411720e0 as *const ()), // sub_1411720e0
    Replacement::draft(0x141172170, "unknown_141172170", fn_141172170 as *const ()), // sub_141172170
    Replacement::draft(0x141172320, "unknown_141172320", fn_141172320 as *const ()), // sub_141172320
    Replacement::draft(0x1411723a0, "unknown_1411723a0", fn_1411723a0 as *const ()), // sub_1411723a0
    Replacement::draft(0x1411724d0, "unknown_1411724d0", tpl_20759::<0x60, 0x40, 0x1401de830, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1411724d0
    Replacement::draft(0x1411725c0, "unknown_1411725c0", fn_1411725c0 as *const ()), // sub_1411725c0
    Replacement::draft(0x141172650, "unknown_141172650", tpl_422::<0x50, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141172650
    Replacement::draft(0x1411726e0, "unknown_1411726e0", fn_1411726e0 as *const ()), // sub_1411726e0
    Replacement::draft(0x141172830, "unknown_141172830", fn_141172830 as *const ()), // sub_141172830
    Replacement::draft(0x141172a00, "unknown_141172a00", fn_141172a00 as *const ()), // sub_141172a00
    Replacement::draft(0x141172b20, "unknown_141172b20", fn_141172b20 as *const ()), // sub_141172b20
    Replacement::draft(0x141172ca0, "unknown_141172ca0", fn_141172ca0 as *const ()), // sub_141172ca0
    Replacement::draft(0x141172d40, "unknown_141172d40", fn_141172d40 as *const ()), // sub_141172d40
    Replacement::draft_throws(0x141173000, "unknown_141173000", fn_141173000 as *const (), Throws::Caught), // sub_141173000
    Replacement::draft(0x1411731c0, "unknown_1411731c0", fn_1411731c0 as *const ()), // sub_1411731c0
    Replacement::draft_throws(0x141173320, "unknown_141173320", tpl_21456::<0x1401e2020, 0x143426fc8, 0x141173630> as *const (), Throws::Discard), // sub_141173320
    Replacement::draft_throws(0x141173480, "sparsehash_141173480", fn_141173480 as *const (), Throws::Caught), // sub_141173480
    Replacement::draft_throws(0x141173630, "unknown_141173630", fn_141173630 as *const (), Throws::Discard), // sub_141173630
    Replacement::draft(0x141173780, "unknown_141173780", fn_141173780 as *const ()), // sub_141173780
    Replacement::draft(0x141173830, "unknown_141173830", fn_141173830 as *const ()), // sub_141173830
    Replacement::draft(0x1411739a0, "unknown_1411739a0", fn_1411739a0 as *const ()), // sub_1411739a0
    Replacement::draft(0x141173b20, "unknown_141173b20", fn_141173b20 as *const ()), // sub_141173b20
    Replacement::draft_throws(0x141173c90, "unknown_141173c90", fn_141173c90 as *const (), Throws::Caught), // sub_141173c90
    Replacement::draft(0x141173d30, "unknown_141173d30", fn_141173d30 as *const ()), // sub_141173d30
    Replacement::draft(0x141173de0, "coregame_141173de0", tpl_5528::<0x143ce3b88, 0x143ce3b90, 0x1411e9f50, 0x143ce3b08, 0x140215610, 0x1420a28e0, 0x1411750d0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c27a6, 0x142646960, 0x143ce3bb0, 0x143ce3bb8, 0x143ce3bbc, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::active_particle_system_list::clear>
    Replacement::draft_throws(0x141174080, "coregame_141174080", fn_141174080 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::attachment_factory::update>
    Replacement::draft_throws(0x141174320, "coregame_141174320", tpl_1173::<0x143ce3acc, 0x143ce3a88, 0x1411e9f50, 0x141174f80, 0x141175110, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c2846, 0x142646960, 0x143ce3aa8, 0x143ce3ab8, 0x143ce3abc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::active_particle_system_list::variableUpdate>
    Replacement::draft_throws(0x141174570, "unknown_141174570", fn_141174570 as *const (), Throws::Caught), // sub_141174570
    Replacement::draft_throws(0x141174790, "sparsehash_141174790", tpl_79801::<0x141172a00, 0x141174d40, 0x14307aa70, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1411749e6, 0x141174a09> as *const (), Throws::Discard), // sub_141174790
    Replacement::draft_throws(0x141174a10, "sparsehash_141174a10", fn_141174a10 as *const (), Throws::Caught), // sub_141174a10
    Replacement::draft_throws(0x141174b00, "sparsehash_141174b00", fn_141174b00 as *const (), Throws::Discard), // sub_141174b00
    Replacement::draft_throws(0x141174d40, "sparsehash_141174d40", tpl_79803::<0x141173320, 0x1411731c0, 0x141172650, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x141174f72> as *const (), Throws::Discard), // sub_141174d40
    Replacement::draft_throws(0x141174f80, "unknown_141174f80", fn_141174f80 as *const (), Throws::Discard), // sub_141174f80
    Replacement::draft(0x1411750d0, "unknown_1411750d0", fn_1411750d0 as *const ()), // sub_1411750d0
    Replacement::draft_throws(0x1411750f0, "unknown_1411750f0", fn_1411750f0 as *const (), Throws::Discard), // sub_1411750f0
    Replacement::draft(0x141175110, "unknown_141175110", fn_141175110 as *const ()), // sub_141175110
    Replacement::draft_throws(0x141175200, "unknown_141175200", fn_141175200 as *const (), Throws::Caught), // sub_141175200
    Replacement::draft(0x141175280, "unknown_141175280", fn_141175280 as *const ()), // sub_141175280
    Replacement::draft(0x141175310, "unknown_141175310", fn_141175310 as *const ()), // sub_141175310
    Replacement::draft_throws(0x1411753c0, "coregame_1411753c0", fn_1411753c0 as *const (), Throws::Caught), // sub_1411753c0
    Replacement::draft(0x141175b70, "coregame_141175b70", fn_141175b70 as *const ()), // sub_141175b70
    Replacement::draft(0x141175ca0, "coregame_141175ca0", fn_141175ca0 as *const ()), // sub_141175ca0
    Replacement::draft_throws(0x141175db0, "coregame_141175db0", fn_141175db0 as *const (), Throws::Discard), // sub_141175db0
    Replacement::draft(0x141175e60, "coregame_141175e60", tpl_4888::<0x30, 0x143cca1d4, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141175ee3> as *const ()), // sub_141175e60
    Replacement::draft(0x141175ef0, "coregame_141175ef0", tpl_21476::<0x30, 0x143cca1dc, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141175f6f> as *const ()), // sub_141175ef0
    Replacement::draft(0x141175f70, "coregame_141175f70", tpl_21476::<0x30, 0x143cca1e0, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141175fef> as *const ()), // sub_141175f70
    Replacement::draft(0x141175ff0, "coregame_141175ff0", tpl_3044::<0x30, 0x143cca1d8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14117606f> as *const ()), // sub_141175ff0
    Replacement::draft_throws(0x141176070, "coregame_141176070", tpl_1404::<0x143ce3d50, 0x143ce4140, 0x1411e9f50, 0x14117f750, 0x143ce40f4, 0x1420af120, 0x1420a28e0, 0x14117cdc0, 0x14117b500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c6736, 0x142646960, 0x143ce4160, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::wakeUp>
    Replacement::draft_throws(0x141176340, "coregame_141176340", fn_141176340 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::advanceTime>
    Replacement::draft_throws(0x1411766f0, "coregame_1411766f0", tpl_1404::<0x143ce45d4, 0x143ce43b0, 0x1411e9f50, 0x14117f890, 0x143ce3f50, 0x1420af120, 0x1420a28e0, 0x14117d1b0, 0x14117b800, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c4e76, 0x142646960, 0x143ce43d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::sample>
    Replacement::draft_throws(0x1411769c0, "coregame_1411769c0", tpl_1102::<0x143ce4548, 0x143ce3e70, 0x1411e9f50, 0x14117faa0, 0x143ce4704, 0x1420af120, 0x1420a28e0, 0x14117d380, 0x14117b8a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c4a86, 0x142646960, 0x143ce3e90, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::sampleTrajectoryOnly>
    Replacement::draft_throws(0x141176ca0, "coregame_141176ca0", tpl_21207::<0x143ce4370, 0x143ce3d90, 0x1411e9f50, 0x14117f330, 0x143ce3f28, 0x1420af120, 0x14036fad0, 0x14036e720, 0x1420a28e0, 0x14117d500, 0x14117b940, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c5446, 0x142646960, 0x143ce3db0, 0x143ce3db8, 0x143ce3dba, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::applyTrajectory>
    Replacement::draft_throws(0x141176f90, "coregame_141176f90", tpl_3061::<0x143ce46e0, 0x143ce45d8, 0x1411e9f50, 0x14117f240, 0x143ce3f3c, 0x1420af120, 0x1420a28e0, 0x14117d7f0, 0x14117bbd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c52b6, 0x142646960, 0x143ce45f8, 0x143ce4600, 0x143ce4604, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::consumeAdvanceTime>
    Replacement::draft_throws(0x141177280, "coregame_141177280", tpl_3281::<0x143ce446c, 0x143ce44e0, 0x1411e9f50, 0x14117f150, 0x143ce40f8, 0x1420af120, 0x1420a28e0, 0x14117d940, 0x14117bcd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c5bf6, 0x142646960, 0x143ce4500, 0x143ce4508, 0x143ce450c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::consumeSampleClip>
    Replacement::draft_throws(0x141177570, "coregame_141177570", tpl_3395::<0x143ce4554, 0x143ce43f8, 0x14117aa10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c5a36, 0x142646960, 0x143ce4418, 0x143ce4420, 0x143ce4422, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::gatherResources>
    Replacement::draft_throws(0x141177780, "coregame_141177780", tpl_3683::<0x143ce3f24, 0x143ce4620, 0x1411e9f50, 0x14117ef20, 0x143ce4468, 0x1420af120, 0x1420a28e0, 0x14117de10, 0x14117bdd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c5f36, 0x142646960, 0x143ce4640, 0x143ce4648, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::setupSkeleton>
    Replacement::draft_throws(0x141177a60, "coregame_141177a60", tpl_1467::<0x143ce45d0, 0x143ce40c8, 0x1411e9f50, 0x14117ee30, 0x143ce4474, 0x1420af120, 0x1420a28e0, 0x14117e030, 0x14117bef0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c5d86, 0x142646960, 0x143ce40e8, 0x143ce40f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::shutdownSkeleton>
    Replacement::draft(0x141177d40, "coregame_141177d40", tpl_4842::<0x14397da68, 0x143c8d08c, 0x8, 0x580e0, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143c8d074, 0x180a8, 0x18090, 0x142489ee0, 0x140411cd0, 0x1426467e8, 0x141177e25> as *const ()), // sub_141177d40
    Replacement::draft(0x141177e40, "coregame_141177e40", tpl_19657::<0x14397da68, 0x143c8d08c, 0x1401fa730, 0x143cca1d4, 0x142489ee0, 0x1426467e8, 0x141177f63> as *const ()), // sub_141177e40
    Replacement::draft(0x141177f80, "coregame_141177f80", fn_141177f80 as *const ()), // sub_141177f80
    Replacement::draft(0x1411780c0, "coregame_1411780c0", fn_1411780c0 as *const ()), // sub_1411780c0
    Replacement::draft(0x141178200, "coregame_141178200", tpl_1094::<0x143ce44dc, 0x140fdd500, 0x14020e8f0> as *const ()), // sub_141178200
    Replacement::draft(0x141178400, "coregame_141178400", tpl_1435::<0x143ce4544, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141178400
    Replacement::draft(0x141178750, "coregame_141178750", tpl_1094::<0x143ce4608, 0x14117b1e0, 0x14020e8f0> as *const ()), // sub_141178750
    Replacement::draft(0x141178950, "coregame_141178950", tpl_1436::<0x58, 0x60, 0x158, 0x143ce4608, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141178950
    Replacement::draft(0x141178a30, "coregame_141178a30", tpl_1094::<0x143cca1e8, 0x14117b360, 0x14020e8f0> as *const ()), // sub_141178a30
    Replacement::draft(0x141178c30, "coregame_141178c30", tpl_1094::<0x143cca1e4, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_141178c30
    Replacement::draft(0x141178e30, "coregame_141178e30", tpl_5014::<0x143cca1dc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141178e30
    Replacement::draft(0x141179160, "coregame_141179160", tpl_1097::<0x143ce40b8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141179160
    Replacement::draft(0x141179490, "coregame_141179490", tpl_3074::<0x143cca1e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141179490
    Replacement::draft(0x141179760, "coregame_141179760", tpl_1099::<0x143cca1d8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141179760
    Replacement::draft(0x141179aa0, "coregame_141179aa0", tpl_4324::<0x143ce464c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141179aa0
    Replacement::draft(0x141179de0, "coregame_141179de0", tpl_1466::<0x143ce3f34, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141179de0
    Replacement::draft(0x14117a0b0, "coregame_14117a0b0", tpl_1466::<0x143ce3f30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14117a0b0
    Replacement::draft(0x14117a380, "coregame_14117a380", tpl_3072::<0x143ce42e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14117a380
    Replacement::draft(0x14117a6d0, "coregame_14117a6d0", tpl_3278::<0x143cca1d4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14117a6d0
    Replacement::draft_throws(0x14117aa10, "coregame_14117aa10", tpl_3011::<0x1411e9f50, 0x14117f060, 0x143ce4328, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14117da90, 0x1401de410> as *const (), Throws::Discard), // sub_14117aa10
    Replacement::draft_throws(0x14117abd0, "coregame_14117abd0", tpl_5850::<0x143ce3d10, 0x143ce4520, 0x1411e9f50, 0x14117fe20, 0x143ce3f2c, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x14117ead0, 0x14117db00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c7666, 0x142646960, 0x143ce4540, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::streamIn>
    Replacement::draft_throws(0x14117aed0, "coregame_14117aed0", tpl_4955::<0x143ce3f44, 0x143ce3f58, 0x1411e9f50, 0x14117fc60, 0x143ce3f38, 0x1420af120, 0x1420a28e0, 0x14117ec50, 0x14117dc40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c7276, 0x142646960, 0x143ce3f78, 0x143ce3f7c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationsource::streamOut>
    Replacement::draft(0x14117b1e0, "unknown_14117b1e0", tpl_4368::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140330a20> as *const ()), // sub_14117b1e0
    Replacement::draft(0x14117b360, "unknown_14117b360", fn_14117b360 as *const ()), // sub_14117b360
    Replacement::draft(0x14117b500, "unknown_14117b500", tpl_62531::<0x88, 0x10088, 0x14117c6c0, 0x141fe0760> as *const ()), // sub_14117b500
    Replacement::draft(0x14117b630, "unknown_14117b630", tpl_19667::<0x88, 0x10088, 0x14117c3c0, 0x14117b6e0> as *const ()), // sub_14117b630
    Replacement::draft(0x14117b6e0, "unknown_14117b6e0", tpl_19668::<0x18, 0x20, 0x40, 0x60, 0x8, 0x141fe0320> as *const ()), // sub_14117b6e0
    Replacement::draft(0x14117b800, "unknown_14117b800", fn_14117b800 as *const ()), // sub_14117b800
    Replacement::draft(0x14117b8a0, "unknown_14117b8a0", tpl_21334::<0x88, 0x10088, 0x14117c820, 0x141fdfb80> as *const ()), // sub_14117b8a0
    Replacement::draft(0x14117b940, "unknown_14117b940", tpl_21248::<0x88, 0x10088, 0x8, 0x20, 0x143c94b44, 0x143c94b30, 0x14117c0c0, 0x14117ba70> as *const ()), // sub_14117b940
    Replacement::draft(0x14117ba70, "unknown_14117ba70", tpl_21249::<0x141fdf650> as *const ()), // sub_14117ba70
    Replacement::draft(0x14117bbd0, "unknown_14117bbd0", tpl_21482::<0x143ce3f34, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141175ca0> as *const ()), // sub_14117bbd0
    Replacement::draft(0x14117bcd0, "unknown_14117bcd0", tpl_21482::<0x143ce3f30, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141175ca0> as *const ()), // sub_14117bcd0
    Replacement::draft_throws(0x14117bdd0, "unknown_14117bdd0", tpl_21017::<0x88, 0x10088, 0x142407c80, 0x14117bf90, 0x580f0, 0x141fe08f0> as *const (), Throws::Discard), // sub_14117bdd0
    Replacement::draft(0x14117bef0, "unknown_14117bef0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x141178950> as *const ()), // sub_14117bef0
    Replacement::draft(0x14117cd90, "unknown_14117cd90", tpl_5596::<0x50> as *const ()), // sub_14117cd90
    Replacement::draft(0x14117cdc0, "unknown_14117cdc0", tpl_62567::<0x14117c6c0, 0x141fe0760> as *const ()), // sub_14117cdc0
    Replacement::draft(0x14117cf60, "unknown_14117cf60", tpl_78777::<0x14117c4c0, 0x141fe0320> as *const ()), // sub_14117cf60
    Replacement::draft(0x14117d1b0, "unknown_14117d1b0", fn_14117d1b0 as *const ()), // sub_14117d1b0
    Replacement::draft(0x14117d380, "unknown_14117d380", tpl_79679::<0x14117c8e0, 0x141fdfb80> as *const ()), // sub_14117d380
    Replacement::draft(0x14117d500, "unknown_14117d500", tpl_79591::<0x143c94b44, 0x143c94b30, 0x14117c1c0, 0x141fdf650> as *const ()), // sub_14117d500
    Replacement::draft(0x14117d7f0, "unknown_14117d7f0", tpl_79821::<0x143ce3f34, 0x142489ee0, 0x141175ca0> as *const ()), // sub_14117d7f0
    Replacement::draft(0x14117d940, "unknown_14117d940", tpl_79821::<0x143ce3f30, 0x142489ee0, 0x141175ca0> as *const ()), // sub_14117d940
    Replacement::draft_throws(0x14117da90, "unknown_14117da90", tpl_4439::<0x143ce4328, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141fdf2b0> as *const (), Throws::Discard), // sub_14117da90
    Replacement::draft_throws(0x14117db00, "unknown_14117db00", fn_14117db00 as *const (), Throws::Caught), // sub_14117db00
    Replacement::draft(0x14117dc40, "unknown_14117dc40", fn_14117dc40 as *const ()), // sub_14117dc40
    Replacement::draft(0x14117dd20, "unknown_14117dd20", tpl_5027::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141fe0a70> as *const ()), // sub_14117dd20
    Replacement::draft_throws(0x14117de10, "unknown_14117de10", tpl_21022::<0x142407c80, 0x143cca1e8, 0x142489ee0, 0x143ce44dc, 0x143c8ec20, 0x141fe08f0> as *const (), Throws::Discard), // sub_14117de10
    Replacement::draft(0x14117e030, "unknown_14117e030", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141178950> as *const ()), // sub_14117e030
    Replacement::draft_throws(0x14117ead0, "unknown_14117ead0", fn_14117ead0 as *const (), Throws::Caught), // sub_14117ead0
    Replacement::draft(0x14117ec50, "unknown_14117ec50", fn_14117ec50 as *const ()), // sub_14117ec50
    Replacement::draft(0x14117f330, "unknown_14117f330", tpl_21258::<0x143ce3dbc, 0x143ce4670, 0x143ce3f30, 0x143ce4544, 0x143c92bcc, 0x143c8dbd4, 0x143ce42e4, 0x143c933f0, 0x143c8d120, 0x143c93358, 0x143c8f6d8, 0x143ce4690, 0x143ce46b0, 0x143ce46d0, 0x141834a0c, 0x141834a78> as *const ()), // sub_14117f330
    Replacement::draft(0x14117f540, "unknown_14117f540", fn_14117f540 as *const ()), // sub_14117f540
    Replacement::draft(0x14117f890, "unknown_14117f890", fn_14117f890 as *const ()), // sub_14117f890
    Replacement::draft(0x14117faa0, "unknown_14117faa0", fn_14117faa0 as *const ()), // sub_14117faa0
    Replacement::draft(0x14117fc60, "unknown_14117fc60", fn_14117fc60 as *const ()), // sub_14117fc60
    Replacement::draft(0x141180610, "unknown_141180610", fn_141180610 as *const ()), // sub_141180610
    Replacement::draft(0x141180900, "unknown_141180900", fn_141180900 as *const ()), // sub_141180900
    Replacement::draft(0x141180960, "unknown_141180960", tpl_1171::<0x58, 0x60, 0x143c8e840, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141180960
    Replacement::draft_throws(0x141180bf0, "coregame_141180bf0", fn_141180bf0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::preUpdate>
    Replacement::draft_throws(0x141180f30, "coregame_141180f30", tpl_1793::<0x143ce4ecc, 0x143ce4cf8, 0x1411837b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c9966, 0x142646960, 0x143ce4d18, 0x143ce4d20, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::slavePreUpdate>
    Replacement::draft_throws(0x141181140, "coregame_141181140", fn_141181140 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::postUpdate>
    Replacement::draft_throws(0x141181450, "coregame_141181450", tpl_3367::<0x143ce4cc0, 0x143ce4e50, 0x1411842b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c9556, 0x142646960, 0x143ce4e70, 0x143ce4e78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::gatherResources>
    Replacement::draft_throws(0x141181660, "coregame_141181660", tpl_1077::<0x143ce4eb8, 0x143ce4870, 0x1411e9f50, 0x141187950, 0x143ce4b3c, 0x1420af120, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x141186440, 0x141184b20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433c9726, 0x142646960, 0x143ce4890, 0x143ce4898, 0x143ce489c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::attachSlaveToParent>
    Replacement::draft(0x141181980, "coregame_141181980", tpl_1094::<0x143ce4cc4, 0x14068b330, 0x14020e8f0> as *const ()), // sub_141181980
    Replacement::draft(0x141181b80, "coregame_141181b80", tpl_1094::<0x143cca0f8, 0x141184540, 0x14020e8f0> as *const ()), // sub_141181b80
    Replacement::draft(0x141181d80, "coregame_141181d80", tpl_1094::<0x143c8e840, 0x1411846d0, 0x14020e8f0> as *const ()), // sub_141181d80
    Replacement::draft(0x141181f80, "coregame_141181f80", tpl_1095::<0x143ce0de0, 0x14020e8f0> as *const ()), // sub_141181f80
    Replacement::draft(0x141182160, "coregame_141182160", tpl_1095::<0x143ce4ed0, 0x14020e8f0> as *const ()), // sub_141182160
    Replacement::draft(0x141182340, "coregame_141182340", tpl_1095::<0x143ce4928, 0x14020e8f0> as *const ()), // sub_141182340
    Replacement::draft(0x141182520, "coregame_141182520", tpl_1436::<0x58, 0x60, 0x158, 0x143ce0de0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141182520
    Replacement::draft(0x141182600, "coregame_141182600", tpl_1368::<0x143cd6e68, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141182600
    Replacement::draft(0x141182930, "coregame_141182930", tpl_3072::<0x143c919a8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141182930
    Replacement::draft(0x141182c80, "coregame_141182c80", tpl_1097::<0x143cbe8e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141182c80
    Replacement::draft(0x141182fb0, "coregame_141182fb0", tpl_5972::<0x143c973f4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141182fb0
    Replacement::draft(0x141183290, "coregame_141183290", tpl_3278::<0x143c8dd14, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141183290
    Replacement::draft(0x1411835d0, "coregame_1411835d0", tpl_1095::<0x143c8e110, 0x14020e8f0> as *const ()), // sub_1411835d0
    Replacement::draft_throws(0x1411837b0, "coregame_1411837b0", tpl_1187::<0x1411e9f50, 0x141187520, 0x143ce4ed8, 0x1420af120, 0x141186f60, 0x143ce4b0c, 0x140215750, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x141185cb0, 0x141184950> as *const (), Throws::Discard), // sub_1411837b0
    Replacement::draft_throws(0x1411839d0, "coregame_1411839d0", tpl_1075::<0x143ce4c60, 0x143ce4b18, 0x1411e9f50, 0x141187c80, 0x143ce4924, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x141186a40, 0x1411860d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ca566, 0x142646960, 0x143ce4b38, 0x143ce4b3a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::streamIn>
    Replacement::draft_throws(0x141183ce0, "coregame_141183ce0", tpl_3683::<0x143ce4b10, 0x143ce4cc8, 0x1411e9f50, 0x141187b40, 0x143ce492c, 0x1420af120, 0x1420a28e0, 0x141186c20, 0x141186220, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ca8a6, 0x142646960, 0x143ce4ce8, 0x143ce4cf0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::streamOutMixer>
    Replacement::draft_throws(0x141183fc0, "coregame_141183fc0", tpl_3061::<0x143ce4ebc, 0x143ce4998, 0x1411e9f50, 0x141187a50, 0x143ce4bf0, 0x1420af120, 0x1420a28e0, 0x141186d90, 0x141186330, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433cbfc6, 0x142646960, 0x143ce49b8, 0x143ce49c0, 0x143ce49c4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationmixer::streamOutPostLayers>
    Replacement::draft_throws(0x1411842b0, "unknown_1411842b0", tpl_3011::<0x1411e9f50, 0x141186e70, 0x143ce4ae8, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x1411863d0, 0x1401de410> as *const (), Throws::Discard), // sub_1411842b0
    Replacement::draft(0x1411844a0, "unknown_1411844a0", tpl_21493::<0x8, 0x141183290, 0x10, 0xd, 0x14, 0xe, 0x15, 0xc, 0x1411835d0> as *const ()), // sub_1411844a0
    Replacement::draft(0x141184540, "unknown_141184540", tpl_19680::<0x1401e6d50, 0x1411859e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_141184540
    Replacement::draft(0x1411846d0, "unknown_1411846d0", tpl_5321::<0x1401e6d50, 0x141185a10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140235690> as *const ()), // sub_1411846d0
    Replacement::draft_throws(0x141184950, "unknown_141184950", fn_141184950 as *const (), Throws::Discard), // sub_141184950
    Replacement::draft(0x141184b20, "unknown_141184b20", fn_141184b20 as *const ()), // sub_141184b20
    Replacement::draft(0x141185300, "unknown_141185300", fn_141185300 as *const ()), // sub_141185300
    Replacement::draft(0x141185410, "unknown_141185410", tpl_61017::<0x143c91f1c, 0x142489ee0, 0x143cca0f8, 0x143c8e840, 0x143c8ec20, 0x143ce0188, 0x143ccf580, 0x143ccf00c, 0x143c9b928, 0x143c8fab8, 0x143c8e68c, 0x143ca7420, 0x143c919a8, 0x143c8de68, 0x143c8d120, 0x143c973f4, 0x143c8d87c, 0x143c8d074, 0x143cd7aac> as *const ()), // sub_141185410
    Replacement::draft(0x141185810, "unknown_141185810", tpl_1275 as *const ()), // sub_141185810
    Replacement::draft(0x1411859e0, "unknown_1411859e0", tpl_1955::<0x10, 0x140fa0930> as *const ()), // sub_1411859e0
    Replacement::draft(0x141185a10, "unknown_141185a10", fn_141185a10 as *const ()), // sub_141185a10
    Replacement::draft(0x141185a80, "unknown_141185a80", fn_141185a80 as *const ()), // sub_141185a80
    Replacement::draft_throws(0x141185cb0, "unknown_141185cb0", fn_141185cb0 as *const (), Throws::Discard), // sub_141185cb0
    Replacement::draft(0x141185f40, "unknown_141185f40", fn_141185f40 as *const ()), // sub_141185f40
    Replacement::draft_throws(0x1411860d0, "unknown_1411860d0", fn_1411860d0 as *const (), Throws::Caught), // sub_1411860d0
    Replacement::draft(0x141186220, "unknown_141186220", fn_141186220 as *const ()), // sub_141186220
    Replacement::draft(0x141186330, "unknown_141186330", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x141182520> as *const ()), // sub_141186330
    Replacement::draft(0x1411863d0, "unknown_1411863d0", tpl_3028::<0x143ce4ae8, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141fe3c60> as *const ()), // sub_1411863d0
    Replacement::draft(0x141186440, "unknown_141186440", fn_141186440 as *const ()), // sub_141186440
    Replacement::draft_throws(0x141186a40, "unknown_141186a40", fn_141186a40 as *const (), Throws::Caught), // sub_141186a40
    Replacement::draft(0x141186c20, "unknown_141186c20", fn_141186c20 as *const ()), // sub_141186c20
    Replacement::draft(0x141186d90, "unknown_141186d90", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141182520> as *const ()), // sub_141186d90
    Replacement::draft(0x141186f60, "unknown_141186f60", fn_141186f60 as *const ()), // sub_141186f60
    Replacement::draft(0x141187130, "unknown_141187130", fn_141187130 as *const ()), // sub_141187130
    Replacement::draft(0x141187520, "unknown_141187520", fn_141187520 as *const ()), // sub_141187520
    Replacement::draft(0x1411877b0, "unknown_1411877b0", fn_1411877b0 as *const ()), // sub_1411877b0
    Replacement::draft_throws(0x141188310, "coregame_141188310", tpl_3683::<0x143ce4f20, 0x143ce4f30, 0x1411e9f50, 0x141189200, 0x143ce501c, 0x1420af120, 0x1420a28e0, 0x141188ed0, 0x1411889c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433cd406, 0x142646960, 0x143ce4f50, 0x143ce4f58, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationinput::updateMovement>
    Replacement::draft(0x1411885f0, "unknown_1411885f0", fn_1411885f0 as *const ()), // sub_1411885f0
    Replacement::draft(0x141188950, "unknown_141188950", tpl_21505::<0x8, 0x1411885f0> as *const ()), // sub_141188950
    Replacement::draft(0x1411889c0, "unknown_1411889c0", fn_1411889c0 as *const ()), // sub_1411889c0
    Replacement::draft(0x141188a60, "unknown_141188a60", fn_141188a60 as *const ()), // sub_141188a60
    Replacement::draft(0x141188ed0, "unknown_141188ed0", fn_141188ed0 as *const ()), // sub_141188ed0
    Replacement::draft(0x141189200, "unknown_141189200", fn_141189200 as *const ()), // sub_141189200
    Replacement::draft_throws(0x141189730, "coregame_141189730", tpl_19370::<0x143ce51d4, 0x143ce5330, 0x1411e9f50, 0x14118cc40, 0x143ce54bc, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x14118bb00, 0x14118b2a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ce7a6, 0x142646960, 0x143ce5350, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationgraph::update>
    Replacement::draft_throws(0x141189a30, "coregame_141189a30", tpl_1394::<0x143ce5408, 0x143ce5110, 0x14118a580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433ce086, 0x142646960, 0x143ce5130, 0x143ce5134, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationgraph::slaveUpdate>
    Replacement::draft_throws(0x141189c40, "coregame_141189c40", tpl_3367::<0x143ce5450, 0x143ce5300, 0x14118ad80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433cdeb6, 0x142646960, 0x143ce5320, 0x143ce5328, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationgraph::gatherResources>
    Replacement::draft(0x141189e50, "coregame_141189e50", tpl_1094::<0x143ce5590, 0x14118af70, 0x14020e8f0> as *const ()), // sub_141189e50
    Replacement::draft(0x14118a050, "coregame_14118a050", tpl_1094::<0x143ce54ec, 0x14118b100, 0x14020e8f0> as *const ()), // sub_14118a050
    Replacement::draft(0x14118a250, "coregame_14118a250", tpl_1368::<0x143ce53c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14118a250
    Replacement::draft_throws(0x14118a580, "coregame_14118a580", tpl_1187::<0x1411e9f50, 0x14118ce60, 0x143ce554c, 0x1420af120, 0x14118cac0, 0x143ce54ac, 0x140215750, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x14118bcc0, 0x14118b350> as *const (), Throws::Discard), // sub_14118a580
    Replacement::draft_throws(0x14118a7a0, "coregame_14118a7a0", tpl_1075::<0x143ce51cc, 0x143ce5528, 0x1411e9f50, 0x14118d1c0, 0x143ce54a4, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x14118c690, 0x14118bf10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433cfca6, 0x142646960, 0x143ce5548, 0x143ce554a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationgraph::streamIn>
    Replacement::draft_throws(0x14118aab0, "coregame_14118aab0", tpl_4485::<0x143ce54a8, 0x143ce54c8, 0x1411e9f50, 0x14118d0b0, 0x143ce52dc, 0x1420af120, 0x1420a28e0, 0x14118c870, 0x14118c060, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433cfab6, 0x142646960, 0x143ce54e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationgraph::streamOut>
    Replacement::draft_throws(0x14118ad80, "unknown_14118ad80", tpl_3011::<0x1411e9f50, 0x14118c9d0, 0x143ce54b8, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14118c170, 0x1401de410> as *const (), Throws::Discard), // sub_14118ad80
    Replacement::draft(0x14118af70, "unknown_14118af70", tpl_19680::<0x1401e6d50, 0x14118ba60, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_14118af70
    Replacement::draft(0x14118b100, "unknown_14118b100", fn_14118b100 as *const ()), // sub_14118b100
    Replacement::draft(0x14118b2a0, "unknown_14118b2a0", fn_14118b2a0 as *const ()), // sub_14118b2a0
    Replacement::draft_throws(0x14118b350, "unknown_14118b350", fn_14118b350 as *const (), Throws::Discard), // sub_14118b350
    Replacement::draft(0x14118ba60, "unknown_14118ba60", fn_14118ba60 as *const ()), // sub_14118ba60
    Replacement::draft(0x14118bad0, "unknown_14118bad0", tpl_1955::<0x20, 0x14118c2d0> as *const ()), // sub_14118bad0
    Replacement::draft(0x14118bb00, "unknown_14118bb00", fn_14118bb00 as *const ()), // sub_14118bb00
    Replacement::draft_throws(0x14118bcc0, "unknown_14118bcc0", fn_14118bcc0 as *const (), Throws::Discard), // sub_14118bcc0
    Replacement::draft_throws(0x14118bf10, "unknown_14118bf10", fn_14118bf10 as *const (), Throws::Caught), // sub_14118bf10
    Replacement::draft(0x14118c060, "unknown_14118c060", tpl_1127::<0x142407c80, 0x143c8e840, 0x142489ee0, 0x141fe9f60> as *const ()), // sub_14118c060
    Replacement::draft(0x14118c170, "unknown_14118c170", tpl_3028::<0x143ce54b8, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141fe8ff0> as *const ()), // sub_14118c170
    Replacement::draft(0x14118d7a0, "unknown_14118d7a0", fn_14118d7a0 as *const ()), // sub_14118d7a0
    Replacement::draft_throws(0x14118d820, "unknown_14118d820", fn_14118d820 as *const (), Throws::Caught), // sub_14118d820
    Replacement::draft_throws(0x14118dab0, "coregame_14118dab0", tpl_3061::<0x143ce567c, 0x143ce5890, 0x1411e9f50, 0x141195020, 0x143ce5ab8, 0x1420af120, 0x1420a28e0, 0x141192fe0, 0x141191bb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d1646, 0x142646960, 0x143ce58b0, 0x143ce58b8, 0x143ce58bc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::updatePhysicsEvents>
    Replacement::draft_throws(0x14118dda0, "coregame_14118dda0", tpl_1467::<0x143ce5afc, 0x143ce5d18, 0x1411e9f50, 0x141194ee0, 0x143ce584c, 0x1420af120, 0x1420a28e0, 0x1411931c0, 0x141191d50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d1406, 0x142646960, 0x143ce5d38, 0x143ce5d40, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::updateAudioEvents>
    Replacement::draft_throws(0x14118e080, "coregame_14118e080", tpl_1102::<0x143ce590c, 0x143ce57a0, 0x1411e9f50, 0x141194d40, 0x143ce5b4c, 0x1420af120, 0x1420a28e0, 0x141193360, 0x141191e80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d10a6, 0x142646960, 0x143ce57c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::updateBoneAudioEvents>
    Replacement::draft_throws(0x14118e360, "coregame_14118e360", tpl_3003::<0x143ce58d4, 0x143ce5ad0, 0x1411e9f50, 0x1411951d0, 0x143ce5b9c, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x141193560, 0x141192020, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d0e46, 0x142646960, 0x143ce5af0, 0x143ce5af8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::updateLuaEvents>
    Replacement::draft_throws(0x14118e680, "coregame_14118e680", tpl_1438::<0x143ce5ba4, 0x143ce5770, 0x1411e9f50, 0x141194c00, 0x143ce5c08, 0x1420af120, 0x1420a28e0, 0x141193660, 0x1411920b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d1c06, 0x142646960, 0x143ce5790, 0x143ce5798, 0x143ce579a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::updateBoneEvents>
    Replacement::draft(0x14118e970, "coregame_14118e970", fn_14118e970 as *const ()), // sub_14118e970
    Replacement::draft(0x14118ea40, "coregame_14118ea40", fn_14118ea40 as *const ()), // sub_14118ea40
    Replacement::draft(0x14118eb10, "coregame_14118eb10", fn_14118eb10 as *const ()), // sub_14118eb10
    Replacement::draft(0x14118ed40, "coregame_14118ed40", tpl_4324::<0x143ce5808, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14118ed40
    Replacement::draft(0x14118f080, "coregame_14118f080", tpl_1094::<0x143ce5688, 0x141191380, 0x14020e8f0> as *const ()), // sub_14118f080
    Replacement::draft(0x14118f280, "coregame_14118f280", tpl_1094::<0x143ce5684, 0x141191500, 0x14020e8f0> as *const ()), // sub_14118f280
    Replacement::draft(0x14118f480, "coregame_14118f480", tpl_1094::<0x143ce5944, 0x141191710, 0x14020e8f0> as *const ()), // sub_14118f480
    Replacement::draft(0x14118f680, "coregame_14118f680", tpl_1094::<0x143ce5b48, 0x1411918b0, 0x14020e8f0> as *const ()), // sub_14118f680
    Replacement::draft(0x14118f880, "coregame_14118f880", tpl_1403::<0x143ce5454, 0x141191a30, 0x14020e8f0> as *const ()), // sub_14118f880
    Replacement::draft_throws(0x14118fa60, "coregame_14118fa60", tpl_1078::<0x143ce58c4, 0x143ce59c0, 0x1411e9f50, 0x141195650, 0x143ce5d10, 0x1420af120, 0x1420a28e0, 0x141194570, 0x141193800, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d19b6, 0x142646960, 0x143ce59e0, 0x143ce59e2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::streamIn>
    Replacement::draft_throws(0x14118fd40, "coregame_14118fd40", tpl_1074::<0x143ce5bdc, 0x143ce58d8, 0x1411e9f50, 0x141195510, 0x143ce58c8, 0x1420af120, 0x1420a28e0, 0x141194790, 0x141193920, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d0926, 0x142646960, 0x143ce58f8, 0x143ce5908, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::streamInAudioBoneEvents>
    Replacement::draft_throws(0x141190020, "coregame_141190020", tpl_4485::<0x143ce5690, 0x143ce5950, 0x1411e9f50, 0x141195420, 0x143ce5b90, 0x1420af120, 0x1420a28e0, 0x141194940, 0x141193a50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d0786, 0x142646960, 0x143ce5970, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::streamOut>
    Replacement::draft_throws(0x1411902f0, "coregame_1411902f0", tpl_3211::<0x143ce59e4, 0x143ce5598, 0x1411e9f50, 0x141195310, 0x143ce5848, 0x1420af120, 0x1420a28e0, 0x141194a50, 0x141193b10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d0576, 0x142646960, 0x143ce55b8, 0x143ce55c8, 0x143ce55ca, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animationevent::streamOutAudioBoneEvents>
    Replacement::draft(0x141190610, "unknown_141190610", fn_141190610 as *const ()), // sub_141190610
    Replacement::draft(0x141190780, "unknown_141190780", fn_141190780 as *const ()), // sub_141190780
    Replacement::draft(0x141190970, "unknown_141190970", fn_141190970 as *const ()), // sub_141190970
    Replacement::draft(0x141190c20, "unknown_141190c20", fn_141190c20 as *const ()), // sub_141190c20
    Replacement::draft(0x141191380, "unknown_141191380", tpl_3089::<0x1401e6d50, 0x141192ec0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1411921e0> as *const ()), // sub_141191380
    Replacement::draft(0x141191500, "unknown_141191500", fn_141191500 as *const ()), // sub_141191500
    Replacement::draft(0x141191710, "unknown_141191710", tpl_4329::<0x1401e6d50, 0x141192f60, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_141191710
    Replacement::draft(0x1411918b0, "unknown_1411918b0", fn_1411918b0 as *const ()), // sub_1411918b0
    Replacement::draft(0x141191a30, "unknown_141191a30", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14327f7e8> as *const ()), // sub_141191a30
    Replacement::draft(0x141191bb0, "unknown_141191bb0", fn_141191bb0 as *const ()), // sub_141191bb0
    Replacement::draft(0x141191c50, "unknown_141191c50", fn_141191c50 as *const ()), // sub_141191c50
    Replacement::draft_throws(0x141191d50, "unknown_141191d50", tpl_63752::<0x88, 0x10088, 0x141192730, 0x141fece60> as *const (), Throws::Discard), // sub_141191d50
    Replacement::draft_throws(0x141191e80, "unknown_141191e80", fn_141191e80 as *const (), Throws::Discard), // sub_141191e80
    Replacement::draft_throws(0x141191f20, "unknown_141191f20", fn_141191f20 as *const (), Throws::Discard), // sub_141191f20
    Replacement::draft(0x141192020, "unknown_141192020", fn_141192020 as *const ()), // sub_141192020
    Replacement::draft(0x1411920b0, "unknown_1411920b0", tpl_62531::<0x88, 0x10088, 0x141192360, 0x141fec430> as *const ()), // sub_1411920b0
    Replacement::draft(0x1411921e0, "unknown_1411921e0", fn_1411921e0 as *const ()), // sub_1411921e0
    Replacement::draft(0x141192280, "unknown_141192280", fn_141192280 as *const ()), // sub_141192280
    Replacement::draft(0x141192890, "unknown_141192890", tpl_5037::<0x8, 0x10, 0x18, 0x141192940, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50> as *const ()), // sub_141192890
    Replacement::draft(0x141192940, "unknown_141192940", tpl_5038::<0x143c8e840, 0x142489ee0, 0x143c973f4, 0x143c8d87c, 0x143ce5808, 0x143c97a5c, 0x143cb3854, 0x143c92bc8> as *const ()), // sub_141192940
    Replacement::draft(0x141192c40, "unknown_141192c40", fn_141192c40 as *const ()), // sub_141192c40
    Replacement::draft(0x141192e30, "unknown_141192e30", fn_141192e30 as *const ()), // sub_141192e30
    Replacement::draft(0x141192ec0, "unknown_141192ec0", tpl_1955::<0x118, 0x1401ee280> as *const ()), // sub_141192ec0
    Replacement::draft(0x141192f00, "unknown_141192f00", tpl_1210::<0x7, 0x48, 0x30, 0x1401de830, 0x20, 0x10> as *const ()), // sub_141192f00
    Replacement::draft(0x141192f60, "unknown_141192f60", tpl_1955::<0x18, 0x14118ea40> as *const ()), // sub_141192f60
    Replacement::draft(0x141192f90, "unknown_141192f90", tpl_5680::<0x7, 0x1a8, 0x190, 0x1401de830, 0x1401ee280> as *const ()), // sub_141192f90
    Replacement::draft(0x141192fe0, "unknown_141192fe0", fn_141192fe0 as *const ()), // sub_141192fe0
    Replacement::draft_throws(0x1411931c0, "unknown_1411931c0", tpl_63756::<0x141192730, 0x141fece60> as *const (), Throws::Discard), // sub_1411931c0
    Replacement::draft_throws(0x141193360, "unknown_141193360", fn_141193360 as *const (), Throws::Discard), // sub_141193360
    Replacement::draft(0x141193560, "unknown_141193560", fn_141193560 as *const ()), // sub_141193560
    Replacement::draft(0x141193660, "unknown_141193660", tpl_62567::<0x141192360, 0x141fec430> as *const ()), // sub_141193660
    Replacement::draft_throws(0x141193800, "unknown_141193800", fn_141193800 as *const (), Throws::Caught), // sub_141193800
    Replacement::draft_throws(0x141193920, "unknown_141193920", fn_141193920 as *const (), Throws::Caught), // sub_141193920
    Replacement::draft(0x141193a50, "unknown_141193a50", tpl_1126::<0x142407c80, 0x580f0, 0x141fed450> as *const ()), // sub_141193a50
    Replacement::draft_throws(0x141193b10, "unknown_141193b10", tpl_6033::<0x88, 0x10088, 0x142407c80, 0x141193c00, 0x580f0, 0x141fed140> as *const (), Throws::Discard), // sub_141193b10
    Replacement::draft(0x141193f90, "unknown_141193f90", fn_141193f90 as *const ()), // sub_141193f90
    Replacement::draft_throws(0x141194570, "unknown_141194570", fn_141194570 as *const (), Throws::Caught), // sub_141194570
    Replacement::draft_throws(0x141194790, "unknown_141194790", fn_141194790 as *const (), Throws::Caught), // sub_141194790
    Replacement::draft(0x141194940, "unknown_141194940", tpl_1130::<0x142407c80, 0x141fed450> as *const ()), // sub_141194940
    Replacement::draft_throws(0x141194a50, "unknown_141194a50", tpl_6035::<0x142407c80, 0x143ce5b48, 0x142489ee0, 0x143cd6de0, 0x141fed140> as *const (), Throws::Discard), // sub_141194a50
    Replacement::draft(0x141194d40, "unknown_141194d40", tpl_3176::<0x143ce5b50, 0x143ce5a20, 0x141834a78, 0x143c8e840, 0x143c8d87c, 0x143c8ec20, 0x143c9a318, 0x143c8d120, 0x143ce5b48, 0x143cd6de0, 0x143ce5a40, 0x143ce5a60, 0x143ce5a70, 0x141834a0c> as *const ()), // sub_141194d40
    Replacement::draft(0x141195020, "unknown_141195020", fn_141195020 as *const ()), // sub_141195020
    Replacement::draft_throws(0x141195ca0, "coregame_141195ca0", tpl_1437::<0x143ce5e58, 0x143ce5e28, 0x1411e9f50, 0x141196300, 0x143ce5e54, 0x1420af120, 0x1420a28e0, 0x141196160, 0x141195f80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d34b6, 0x142646960, 0x143ce5e48, 0x143ce5e50, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::animation_input_requests::apply>
    Replacement::draft(0x141196160, "coregame_141196160", tpl_62763::<0x143cca1f8, 0x142489ee0, 0x143c8de68, 0x141fefe90> as *const ()), // sub_141196160
    Replacement::draft(0x141196480, "coregame_141196480", fn_141196480 as *const ()), // sub_141196480
    Replacement::draft_throws(0x141196540, "coregame_141196540", fn_141196540 as *const (), Throws::Discard), // coregame::env::Analytics::ResetSessionThread::coregame::env::Analytics::ResetSessionThread::vfunc_1
    Replacement::draft_throws(0x1411966f0, "coregame_1411966f0", fn_1411966f0 as *const (), Throws::Discard), // coregame::env::Analytics::ResetSessionThread::coregame::env::Analytics::ResetSessionThread::vfunc_0
    Replacement::draft(0x1411967d0, "unknown_1411967d0", fn_1411967d0 as *const ()), // sub_1411967d0
    Replacement::draft_throws(0x141196860, "unknown_141196860", fn_141196860 as *const (), Throws::Caught), // sub_141196860
    Replacement::draft(0x141196920, "unknown_141196920", fn_141196920 as *const ()), // sub_141196920
    Replacement::draft(0x141196930, "unknown_141196930", fn_141196930 as *const ()), // sub_141196930
    Replacement::draft_throws(0x141196ad0, "unknown_141196ad0", fn_141196ad0 as *const (), Throws::Discard), // sub_141196ad0
    Replacement::draft(0x141196bc0, "unknown_141196bc0", fn_141196bc0 as *const ()), // sub_141196bc0
    Replacement::draft(0x141196e10, "unknown_141196e10", fn_141196e10 as *const ()), // sub_141196e10
    Replacement::draft_throws(0x141196e80, "unknown_141196e80", tpl_18971::<0xfffffffffffffffe, 0x141ff0ad0, 0x142442de0> as *const (), Throws::Discard), // sub_141196e80
    Replacement::draft(0x141196ed0, "unknown_141196ed0", fn_141196ed0 as *const ()), // sub_141196ed0
    Replacement::draft(0x141197060, "unknown_141197060", fn_141197060 as *const ()), // sub_141197060
    Replacement::draft(0x141197380, "unknown_141197380", fn_141197380 as *const ()), // sub_141197380
    Replacement::draft_throws(0x1411975d0, "unknown_1411975d0", fn_1411975d0 as *const (), Throws::Discard), // sub_1411975d0
    Replacement::draft(0x1411977d0, "input_1411977d0", tpl_21549::<0x5c> as *const ()), // input::coregame::InputAction::input::coregame::InputAction::vfunc_19
    Replacement::draft(0x1411977e0, "input_1411977e0", tpl_21550::<0x5c> as *const ()), // input::coregame::InputAction::input::coregame::InputAction::vfunc_20
    Replacement::draft_throws(0x141197800, "input_141197800", fn_141197800 as *const (), Throws::Discard), // input::coregame::InputAction::input::coregame::InputAction::vfunc_22
    Replacement::draft(0x141197830, "input_141197830", fn_141197830 as *const ()), // input::coregame::InputAction::input::coregame::InputAction::vfunc_0
    Replacement::draft(0x141197880, "input_141197880", fn_141197880 as *const ()), // sub_141197880
    Replacement::draft(0x141197b40, "input_141197b40", tpl_21554::<0x1a8> as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_3
    Replacement::draft(0x141197b50, "input_141197b50", tpl_21555::<0xa8, 0x1a8> as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_5
    Replacement::draft(0x141197b90, "input_141197b90", fn_141197b90 as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_8
    Replacement::draft(0x141197bf0, "input_141197bf0", fn_141197bf0 as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_0
    Replacement::draft(0x141197c90, "input_141197c90", fn_141197c90 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_3
    Replacement::draft(0x141197cb0, "input_141197cb0", fn_141197cb0 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_18
    Replacement::draft(0x141197cc0, "input_141197cc0", tpl_21560::<0x68, 0x30, 0x18, 0x70> as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_6
    Replacement::draft(0x141197d20, "input_141197d20", fn_141197d20 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_5
    Replacement::draft(0x141197d40, "input_141197d40", fn_141197d40 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_19
    Replacement::draft(0x141197d50, "input_141197d50", fn_141197d50 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_20
    Replacement::draft(0x141197d60, "input_141197d60", tpl_21564::<0x68, 0x5c> as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_21
    Replacement::draft(0x141197d90, "input_141197d90", fn_141197d90 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_0
    Replacement::draft(0x141197e00, "input_141197e00", fn_141197e00 as *const ()), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_3
    Replacement::draft_throws(0x141197e10, "input_141197e10", fn_141197e10 as *const (), Throws::Discard), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_6
    Replacement::draft(0x141197f00, "input_141197f00", fn_141197f00 as *const ()), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_7
    Replacement::draft(0x141197f30, "input_141197f30", fn_141197f30 as *const ()), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_8
    Replacement::draft(0x141197f40, "input_141197f40", fn_141197f40 as *const ()), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_0
    Replacement::draft(0x141197f90, "input_141197f90", tpl_21554::<0xe2> as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_3
    Replacement::draft(0x141197fa0, "input_141197fa0", tpl_21555::<0xa8, 0xe2> as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_5
    Replacement::draft(0x141197fe0, "input_141197fe0", fn_141197fe0 as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_8
    Replacement::draft(0x141198000, "input_141198000", fn_141198000 as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_0
    Replacement::draft(0x141198070, "input_141198070", tpl_21554::<0x108> as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_3
    Replacement::draft(0x141198080, "input_141198080", tpl_21555::<0xa8, 0x108> as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_5
    Replacement::draft(0x1411980c0, "input_1411980c0", fn_1411980c0 as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_0
    Replacement::draft(0x141198130, "input_141198130", tpl_21554::<0xd8> as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_3
    Replacement::draft(0x141198140, "input_141198140", tpl_21555::<0xa8, 0xd8> as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_5
    Replacement::draft(0x141198180, "input_141198180", fn_141198180 as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_0
    Replacement::draft(0x1411981f0, "unknown_1411981f0", fn_1411981f0 as *const ()), // sub_1411981f0
    Replacement::draft(0x141198380, "unknown_141198380", fn_141198380 as *const ()), // sub_141198380
    Replacement::draft(0x141198440, "unknown_141198440", fn_141198440 as *const ()), // sub_141198440
    Replacement::draft(0x141198560, "unknown_141198560", fn_141198560 as *const ()), // sub_141198560
    Replacement::draft_throws(0x1411986e0, "unknown_1411986e0", fn_1411986e0 as *const (), Throws::Caught), // sub_1411986e0
    Replacement::draft(0x141198810, "unknown_141198810", fn_141198810 as *const ()), // sub_141198810
    Replacement::draft(0x141198b30, "unknown_141198b30", fn_141198b30 as *const ()), // sub_141198b30
    Replacement::draft(0x141198d80, "unknown_141198d80", fn_141198d80 as *const ()), // sub_141198d80
    Replacement::draft(0x141199080, "unknown_141199080", fn_141199080 as *const ()), // sub_141199080
    Replacement::draft_throws(0x141199420, "unknown_141199420", fn_141199420 as *const (), Throws::Discard), // sub_141199420
    Replacement::draft(0x141199500, "unknown_141199500", fn_141199500 as *const ()), // sub_141199500
    Replacement::draft(0x141199810, "unknown_141199810", fn_141199810 as *const ()), // sub_141199810
    Replacement::draft(0x141199930, "unknown_141199930", fn_141199930 as *const ()), // sub_141199930
    Replacement::draft(0x141199ad0, "unknown_141199ad0", tpl_21587::<0x1424b16a0, 0x143857278, 0x1426d01e0, 0x143857268, 0x143857260> as *const ()), // sub_141199ad0
    Replacement::draft(0x141199c60, "unknown_141199c60", tpl_21587::<0x1424b16a0, 0x143857308, 0x1426d01e0, 0x1438572f8, 0x1438572f0> as *const ()), // sub_141199c60
    Replacement::draft(0x141199df0, "unknown_141199df0", tpl_21587::<0x1424b16a0, 0x1438572a8, 0x1426d01e0, 0x143857298, 0x143857290> as *const ()), // sub_141199df0
    Replacement::draft(0x141199f80, "unknown_141199f80", fn_141199f80 as *const ()), // sub_141199f80
    Replacement::draft(0x14119a0c0, "unknown_14119a0c0", tpl_21589::<0x143857338, 0x143857320, 0x143857348, 0x143857330, 0x14119a240, 0x14119a4c0> as *const ()), // sub_14119a0c0
    Replacement::draft(0x14119a240, "unknown_14119a240", fn_14119a240 as *const ()), // sub_14119a240
    Replacement::draft(0x14119a4c0, "unknown_14119a4c0", fn_14119a4c0 as *const ()), // sub_14119a4c0
    Replacement::draft(0x14119a7b0, "unknown_14119a7b0", tpl_21589::<0x143857368, 0x143857350, 0x143857378, 0x143857360, 0x14119a930, 0x14119ac00> as *const ()), // sub_14119a7b0
    Replacement::draft(0x14119a930, "unknown_14119a930", fn_14119a930 as *const ()), // sub_14119a930
    Replacement::draft(0x14119ac00, "unknown_14119ac00", fn_14119ac00 as *const ()), // sub_14119ac00
    Replacement::draft(0x14119afa0, "unknown_14119afa0", fn_14119afa0 as *const ()), // sub_14119afa0
    Replacement::draft(0x14119b060, "unknown_14119b060", fn_14119b060 as *const ()), // sub_14119b060
    Replacement::draft(0x14119b210, "unknown_14119b210", fn_14119b210 as *const ()), // sub_14119b210
    Replacement::draft_throws(0x14119b3c0, "coregame_14119b3c0", tpl_4130::<0x143ce62e8, 0x143ce6068, 0x1411e9f50, 0x14119ee30, 0x143ce6060, 0x1420af120, 0x143ccc414, 0x140215610, 0x1420a28e0, 0x14119db50, 0x14119d540, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d5cf6, 0x142646960, 0x143ce6088, 0x143ce6090, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::updateActivation>
    Replacement::draft_throws(0x14119b6d0, "coregame_14119b6d0", tpl_3006::<0x143ce601c, 0x143ce6340, 0x14119c230, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d5f76, 0x142646960, 0x143ce6360, 0x143ce6368, 0x143ce636c, 0x143ce636e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::probe_group::update>
    Replacement::draft(0x14119b8f0, "coregame_14119b8f0", tpl_1369::<0x143ce5fe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14119b8f0
    Replacement::draft(0x14119bc30, "coregame_14119bc30", tpl_1094::<0x143ce62f0, 0x14119d080, 0x14020e8f0> as *const ()), // sub_14119bc30
    Replacement::draft(0x14119be30, "coregame_14119be30", tpl_1094::<0x143ce5f98, 0x14119d200, 0x14020e8f0> as *const ()), // sub_14119be30
    Replacement::draft(0x14119c030, "coregame_14119c030", tpl_1094::<0x143ce6064, 0x14119d3a0, 0x14020e8f0> as *const ()), // sub_14119c030
    Replacement::draft_throws(0x14119c230, "coregame_14119c230", tpl_1187::<0x1411e9f50, 0x14119ed30, 0x143ce5f9c, 0x1420af120, 0x14119ec80, 0x143ce6020, 0x140215750, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x14119dda0, 0x14119d690> as *const (), Throws::Discard), // sub_14119c230
    Replacement::draft_throws(0x14119c450, "coregame_14119c450", tpl_4485::<0x143ce62ec, 0x143ce63b0, 0x1411e9f50, 0x14119f250, 0x143ce5f94, 0x1420af120, 0x1420a28e0, 0x14119e700, 0x14119dfd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d57a6, 0x142646960, 0x143ce63d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::streamIn>
    Replacement::draft_throws(0x14119c720, "coregame_14119c720", tpl_4955::<0x143ce63a8, 0x143ce5ff0, 0x1411e9f50, 0x14119f160, 0x143ce5fa8, 0x1420af120, 0x1420a28e0, 0x14119e810, 0x14119e090, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d5996, 0x142646960, 0x143ce6010, 0x143ce6014, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::streamOut>
    Replacement::draft_throws(0x14119ca00, "coregame_14119ca00", fn_14119ca00 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::probe_group::streamIn>
    Replacement::draft_throws(0x14119cd40, "coregame_14119cd40", tpl_1074::<0x143ce5fec, 0x143ce5ec8, 0x1411e9f50, 0x14119ef70, 0x143ce627c, 0x1420af120, 0x1420a28e0, 0x14119eb20, 0x14119e2f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d54c6, 0x142646960, 0x143ce5ee8, 0x143ce5ef8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::probe_group::streamOut>
    Replacement::draft(0x14119d080, "unknown_14119d080", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143286a40> as *const ()), // sub_14119d080
    Replacement::draft(0x14119d200, "unknown_14119d200", fn_14119d200 as *const ()), // sub_14119d200
    Replacement::draft(0x14119d3a0, "unknown_14119d3a0", fn_14119d3a0 as *const ()), // sub_14119d3a0
    Replacement::draft(0x14119d540, "unknown_14119d540", tpl_62759::<0x88, 0x10088, 0x143ccc414, 0x580e0, 0x14119d8f0, 0x14200efe0> as *const ()), // sub_14119d540
    Replacement::draft(0x14119d690, "unknown_14119d690", tpl_62526::<0x143ce6020, 0x143c8d08c, 0x14119d7f0, 0x14200e910> as *const ()), // sub_14119d690
    Replacement::draft(0x14119da20, "unknown_14119da20", tpl_5213::<0x20, 0x1401de830> as *const ()), // sub_14119da20
    Replacement::draft(0x14119e700, "unknown_14119e700", tpl_1130::<0x142407c80, 0x14200f230> as *const ()), // sub_14119e700
    Replacement::draft(0x14119e810, "unknown_14119e810", tpl_1131::<0x142407c80, 0x143ce5fe8, 0x142489ee0, 0x14200f0e0> as *const ()), // sub_14119e810
    Replacement::draft(0x14119e970, "unknown_14119e970", tpl_6170::<0x143c8d08c, 0x142407c80, 0x143cada18, 0x143ce6064, 0x142489ee0, 0x14200ed50> as *const ()), // sub_14119e970
    Replacement::draft(0x14119eb20, "unknown_14119eb20", tpl_1131::<0x142407c80, 0x143ce5f98, 0x142489ee0, 0x14200ebc0> as *const ()), // sub_14119eb20
    Replacement::draft(0x14119f740, "unknown_14119f740", fn_14119f740 as *const ()), // sub_14119f740
    Replacement::draft(0x14119f870, "unknown_14119f870", fn_14119f870 as *const ()), // sub_14119f870
    Replacement::draft(0x14119f8e0, "unknown_14119f8e0", fn_14119f8e0 as *const ()), // sub_14119f8e0
    Replacement::draft(0x14119f9f0, "std_14119f9f0", tpl_21600::<0x1401e66f0> as *const ()), // sub_14119f9f0
    Replacement::draft(0x14119fb10, "unknown_14119fb10", fn_14119fb10 as *const ()), // sub_14119fb10
    Replacement::draft(0x14119fb80, "unknown_14119fb80", fn_14119fb80 as *const ()), // sub_14119fb80
    Replacement::draft(0x14119fcc0, "unknown_14119fcc0", fn_14119fcc0 as *const ()), // sub_14119fcc0
    Replacement::draft_throws(0x14119fe50, "coregame_14119fe50", tpl_3003::<0x143ce6460, 0x143ce6510, 0x1411e9f50, 0x1411a3530, 0x143ce6458, 0x1420af120, 0x143cc8948, 0x140215610, 0x1420a28e0, 0x1411a2c60, 0x1411a2520, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433d6256, 0x142646960, 0x143ce6530, 0x143ce6538, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::updateListener>
    Replacement::draft_throws(0x1411a0170, "coregame_1411a0170", fn_1411a0170 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::updateAcousticsWorld>
    Replacement::draft_throws(0x1411a0500, "coregame_1411a0500", fn_1411a0500 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio_acoustics::cleanupEntities>
    Replacement::draft_throws(0x1411a0870, "std_1411a0870", fn_1411a0870 as *const (), Throws::Discard), // sub_1411a0870
    Replacement::draft_throws(0x1411a0ac0, "std_1411a0ac0", fn_1411a0ac0 as *const (), Throws::Discard), // sub_1411a0ac0
    Replacement::draft_throws(0x1411a0d50, "std_1411a0d50", tpl_6525::<0x14307e950, 0x142645870, 0x1401e5930, 0x140206290, 0x1411a13e0, 0x1411a0e7e> as *const (), Throws::Discard), // sub_1411a0d50
    Replacement::draft(0x1411a0fe0, "std_1411a0fe0", fn_1411a0fe0 as *const ()), // sub_1411a0fe0
    Replacement::draft_throws(0x1411a11f0, "std_1411a11f0", fn_1411a11f0 as *const (), Throws::Discard), // sub_1411a11f0
    Replacement::draft_throws(0x1411a13e0, "std_1411a13e0", tpl_65423::<0x14307e970, 0x142645870, 0x1401e5730, 0x1411a1414> as *const (), Throws::Discard), // sub_1411a13e0
    Replacement::draft(0x1411a15d0, "unknown_1411a15d0", fn_1411a15d0 as *const ()), // sub_1411a15d0
    Replacement::draft(0x1411a15e0, "unknown_1411a15e0", tpl_3477::<0xc, 0x8, 0x140262a30, 0x1411a2700, 0x1411a2ab0> as *const ()), // sub_1411a15e0
    Replacement::draft(0x1411a16b0, "unknown_1411a16b0", fn_1411a16b0 as *const ()), // sub_1411a16b0
    Replacement::draft(0x1411a1860, "unknown_1411a1860", fn_1411a1860 as *const ()), // sub_1411a1860
    Replacement::draft(0x1411a1b60, "unknown_1411a1b60", tpl_1094::<0x143ce64a0, 0x1411a2380, 0x14020e8f0> as *const ()), // sub_1411a1b60
    Replacement::draft(0x1411a1d60, "unknown_1411a1d60", tpl_1095::<0x143ce681c, 0x14020e8f0> as *const ()), // sub_1411a1d60
    Replacement::draft(0x1411a1f40, "unknown_1411a1f40", tpl_1962::<0x8, 0x1411a1b60, 0x1411a1d60> as *const ()), // sub_1411a1f40
    Replacement::draft(0x1411a1fa0, "unknown_1411a1fa0", fn_1411a1fa0 as *const ()), // sub_1411a1fa0
    Replacement::draft(0x1411a2010, "unknown_1411a2010", fn_1411a2010 as *const ()), // sub_1411a2010
    Replacement::draft(0x1411a2090, "unknown_1411a2090", fn_1411a2090 as *const ()), // sub_1411a2090
    Replacement::draft(0x1411a2380, "unknown_1411a2380", fn_1411a2380 as *const ()), // sub_1411a2380
    Replacement::draft(0x1411a2520, "unknown_1411a2520", tpl_64057::<0x88, 0x10088, 0x143cc8948, 0x580e0, 0x1411a29b0, 0x14200ff10> as *const ()), // sub_1411a2520
    Replacement::draft(0x1411a2620, "unknown_1411a2620", tpl_78030::<0x88, 0x10088, 0x143ce681c, 0x180a8, 0x18090, 0x142489ee0, 0x14200fde0> as *const ()), // sub_1411a2620
    Replacement::draft(0x1411a2700, "unknown_1411a2700", fn_1411a2700 as *const ()), // sub_1411a2700
    Replacement::draft(0x1411a2870, "unknown_1411a2870", fn_1411a2870 as *const ()), // sub_1411a2870
    Replacement::draft(0x1411a2ab0, "unknown_1411a2ab0", fn_1411a2ab0 as *const ()), // sub_1411a2ab0
    Replacement::draft(0x1411a2b80, "unknown_1411a2b80", fn_1411a2b80 as *const ()), // sub_1411a2b80
    Replacement::draft(0x1411a2c60, "unknown_1411a2c60", tpl_64074::<0x143cc8948, 0x143ce64a0, 0x142489ee0, 0x143c8d120, 0x14200ff10> as *const ()), // sub_1411a2c60
    Replacement::draft_throws(0x1411a2e20, "unknown_1411a2e20", tpl_21616::<0x1420104f0> as *const (), Throws::Discard), // sub_1411a2e20
    Replacement::draft(0x1411a2e30, "unknown_1411a2e30", tpl_78031::<0x143ce681c, 0x142489ee0, 0x14200fde0> as *const ()), // sub_1411a2e30
    Replacement::draft(0x1411a2f90, "unknown_1411a2f90", fn_1411a2f90 as *const ()), // sub_1411a2f90
    Replacement::draft_throws(0x1411a31f0, "unknown_1411a31f0", fn_1411a31f0 as *const (), Throws::Discard), // sub_1411a31f0
    Replacement::draft(0x1411a3440, "unknown_1411a3440", fn_1411a3440 as *const ()), // sub_1411a3440
    Replacement::draft(0x1411a3530, "unknown_1411a3530", fn_1411a3530 as *const ()), // sub_1411a3530
    Replacement::draft(0x1411a3630, "unknown_1411a3630", fn_1411a3630 as *const ()), // sub_1411a3630
    Replacement::draft(0x1411a3780, "unknown_1411a3780", tpl_170::<0x1411a2ab0> as *const ()), // sub_1411a3780
    Replacement::draft(0x1411a3790, "unknown_1411a3790", fn_1411a3790 as *const ()), // sub_1411a3790
    Replacement::draft(0x1411a3860, "unknown_1411a3860", tpl_20039::<0x8, 0x14119f8e0> as *const ()), // sub_1411a3860
    Replacement::draft_throws(0x1411a38a0, "unknown_1411a38a0", fn_1411a38a0 as *const (), Throws::Discard), // sub_1411a38a0
    Replacement::draft_throws(0x1411a3b20, "unknown_1411a3b20", fn_1411a3b20 as *const (), Throws::Discard), // sub_1411a3b20
    Replacement::draft_throws(0x1411a3cb0, "unknown_1411a3cb0", fn_1411a3cb0 as *const (), Throws::Discard), // sub_1411a3cb0
    Replacement::draft_throws(0x1411a3fe0, "unknown_1411a3fe0", fn_1411a3fe0 as *const (), Throws::Discard), // sub_1411a3fe0
    Replacement::draft_throws(0x1411a4170, "unknown_1411a4170", tpl_4541::<0x143ce6500, 0x1411a3440, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411a4170
    Replacement::draft_throws(0x1411a4200, "unknown_1411a4200", tpl_4541::<0x143ce6458, 0x1411a3530, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1411a4200
    Replacement::draft_throws(0x1411a4290, "unknown_1411a4290", fn_1411a4290 as *const (), Throws::Discard), // sub_1411a4290
    Replacement::draft_throws(0x1411a4440, "std_1411a4440", fn_1411a4440 as *const (), Throws::Discard), // sub_1411a4440
    Replacement::draft_throws(0x1411a46f0, "std_1411a46f0", tpl_6531::<0x1401e5930, 0x140206290, 0x1411a13e0, 0x14307e950, 0x142645870, 0x1411a4994> as *const (), Throws::Discard), // sub_1411a46f0
    Replacement::draft_throws(0x1411a49a0, "std_1411a49a0", fn_1411a49a0 as *const (), Throws::Discard), // sub_1411a49a0
    Replacement::draft_throws(0x1411a4bf0, "unknown_1411a4bf0", fn_1411a4bf0 as *const (), Throws::Discard), // sub_1411a4bf0
    Replacement::draft_throws(0x1411a4db0, "unknown_1411a4db0", fn_1411a4db0 as *const (), Throws::Discard), // sub_1411a4db0
    Replacement::draft(0x1411a4fb0, "unknown_1411a4fb0", fn_1411a4fb0 as *const ()), // sub_1411a4fb0
    Replacement::draft(0x1411a50d0, "unknown_1411a50d0", fn_1411a50d0 as *const ()), // sub_1411a50d0
    Replacement::draft_throws(0x1411a5180, "coreshared_1411a5180", fn_1411a5180 as *const (), Throws::Discard), // coreshared::ParticleSystemState::coreshared::ParticleSystemState::vfunc_0
    Replacement::draft(0x1411a5250, "coreshared_1411a5250", fn_1411a5250 as *const ()), // sub_1411a5250
    Replacement::draft(0x1411a5300, "coreshared_1411a5300", fn_1411a5300 as *const ()), // sub_1411a5300
    Replacement::draft(0x1411a54b0, "coreshared_1411a54b0", fn_1411a54b0 as *const ()), // sub_1411a54b0
    Replacement::draft(0x1411a5650, "coreshared_1411a5650", fn_1411a5650 as *const ()), // sub_1411a5650
    Replacement::draft_throws(0x1411a5a50, "coreshared_1411a5a50", fn_1411a5a50 as *const (), Throws::Discard), // coreshared::ParticleEmitterState::coreshared::ParticleEmitterState::vfunc_0
    Replacement::draft_throws(0x1411a5af0, "coreshared_1411a5af0", fn_1411a5af0 as *const (), Throws::Discard), // sub_1411a5af0
    Replacement::draft(0x1411a5b90, "coreshared_1411a5b90", fn_1411a5b90 as *const ()), // sub_1411a5b90
    Replacement::draft(0x1411a5eb0, "coreshared_1411a5eb0", fn_1411a5eb0 as *const ()), // sub_1411a5eb0
    Replacement::draft(0x1411a5f60, "coreshared_1411a5f60", fn_1411a5f60 as *const ()), // sub_1411a5f60
    Replacement::draft(0x1411a5fd0, "coreshared_1411a5fd0", fn_1411a5fd0 as *const ()), // sub_1411a5fd0
    Replacement::draft(0x1411a6220, "coreshared_1411a6220", fn_1411a6220 as *const ()), // sub_1411a6220
    Replacement::draft(0x1411a6270, "coreshared_1411a6270", fn_1411a6270 as *const ()), // sub_1411a6270
    Replacement::draft(0x1411a6810, "coreshared_1411a6810", fn_1411a6810 as *const ()), // sub_1411a6810
    Replacement::draft(0x1411a7040, "coreshared_1411a7040", fn_1411a7040 as *const ()), // sub_1411a7040
    Replacement::draft(0x1411a77c0, "coreshared_1411a77c0", fn_1411a77c0 as *const ()), // sub_1411a77c0
    Replacement::draft(0x1411a7e10, "coreshared_1411a7e10", fn_1411a7e10 as *const ()), // sub_1411a7e10
    Replacement::draft(0x1411a86f0, "coreshared_1411a86f0", fn_1411a86f0 as *const ()), // sub_1411a86f0
    Replacement::draft(0x1411a8f50, "coreshared_1411a8f50", fn_1411a8f50 as *const ()), // sub_1411a8f50
    Replacement::draft(0x1411a9740, "coreshared_1411a9740", fn_1411a9740 as *const ()), // sub_1411a9740
    Replacement::draft(0x1411aa190, "coreshared_1411aa190", fn_1411aa190 as *const ()), // sub_1411aa190
    Replacement::draft(0x1411aab50, "coreshared_1411aab50", fn_1411aab50 as *const ()), // sub_1411aab50
    Replacement::draft(0x1411aac60, "coreshared_1411aac60", fn_1411aac60 as *const ()), // coreshared::NPCScheduleResource::coreshared::NPCScheduleResource::vfunc_0
    Replacement::draft(0x1411aace0, "coreshared_1411aace0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433d7208, 0x8> as *const ()), // sub_1411aace0
    Replacement::draft(0x1411aad60, "coreshared_1411aad60", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d7208, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411aad60
    Replacement::draft_throws(0x1411aae00, "coreshared_1411aae00", fn_1411aae00 as *const (), Throws::Caught), // sub_1411aae00
    Replacement::draft_throws(0x1411aae90, "coreshared_1411aae90", tpl_1908::<0x143881508, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411aae90
    Replacement::draft(0x1411aaf10, "coreshared_1411aaf10", fn_1411aaf10 as *const ()), // sub_1411aaf10
    Replacement::draft(0x1411aafe0, "coreshared_1411aafe0", fn_1411aafe0 as *const ()), // sub_1411aafe0
    Replacement::draft(0x1411ab0e0, "coreshared_1411ab0e0", fn_1411ab0e0 as *const ()), // sub_1411ab0e0
    Replacement::draft(0x1411ab1a0, "coreshared_1411ab1a0", fn_1411ab1a0 as *const ()), // sub_1411ab1a0
    Replacement::draft_throws(0x1411ab2b0, "coreshared_1411ab2b0", fn_1411ab2b0 as *const (), Throws::Discard), // sub_1411ab2b0
    Replacement::draft_throws(0x1411ab3a0, "coreshared_1411ab3a0", fn_1411ab3a0 as *const (), Throws::Discard), // sub_1411ab3a0
    Replacement::draft_throws(0x1411ab460, "coreshared_1411ab460", fn_1411ab460 as *const (), Throws::Discard), // sub_1411ab460
    Replacement::draft_throws(0x1411ab540, "coreshared_1411ab540", fn_1411ab540 as *const (), Throws::Discard), // sub_1411ab540
    Replacement::draft_throws(0x1411ab600, "coreshared_1411ab600", fn_1411ab600 as *const (), Throws::Caught), // sub_1411ab600
    Replacement::draft_throws(0x1411ab890, "coreshared_1411ab890", fn_1411ab890 as *const (), Throws::Caught), // sub_1411ab890
    Replacement::draft_throws(0x1411abae0, "coreshared_1411abae0", fn_1411abae0 as *const (), Throws::Caught), // sub_1411abae0
    Replacement::draft(0x1411abd20, "coreshared_1411abd20", fn_1411abd20 as *const ()), // sub_1411abd20
    Replacement::draft(0x1411abe40, "coreshared_1411abe40", fn_1411abe40 as *const ()), // sub_1411abe40
    Replacement::draft(0x1411abf60, "coreshared_1411abf60", fn_1411abf60 as *const ()), // sub_1411abf60
    Replacement::draft_throws(0x1411ac0a0, "coreshared_1411ac0a0", fn_1411ac0a0 as *const (), Throws::Discard), // sub_1411ac0a0
    Replacement::draft_throws(0x1411ac1b0, "coreshared_1411ac1b0", fn_1411ac1b0 as *const (), Throws::Discard), // sub_1411ac1b0
    Replacement::draft_throws(0x1411ac2c0, "coreshared_1411ac2c0", fn_1411ac2c0 as *const (), Throws::Caught), // sub_1411ac2c0
    Replacement::draft_throws(0x1411ac5c0, "coreshared_1411ac5c0", tpl_65672::<0x142442de0, 0x143a3a950, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1411ac5c0
    Replacement::draft_throws(0x1411ac7e0, "coreshared_1411ac7e0", fn_1411ac7e0 as *const (), Throws::Caught), // sub_1411ac7e0
    Replacement::draft_throws(0x1411acad0, "coreshared_1411acad0", fn_1411acad0 as *const (), Throws::Caught), // sub_1411acad0
    Replacement::draft_throws(0x1411ad590, "coreshared_1411ad590", fn_1411ad590 as *const (), Throws::Caught), // sub_1411ad590
    Replacement::draft_throws(0x1411ae020, "coreshared_1411ae020", fn_1411ae020 as *const (), Throws::Caught), // sub_1411ae020
    Replacement::draft_throws(0x1411aeaa0, "coreshared_1411aeaa0", fn_1411aeaa0 as *const (), Throws::Caught), // sub_1411aeaa0
    Replacement::draft_throws(0x1411aeca0, "coreshared_1411aeca0", fn_1411aeca0 as *const (), Throws::Caught), // sub_1411aeca0
    Replacement::draft_throws(0x1411aede0, "coreshared_1411aede0", tpl_65670::<0x142442de0, 0x143a3a950, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1411aede0
    Replacement::draft_throws(0x1411aeec0, "coreshared_1411aeec0", fn_1411aeec0 as *const (), Throws::Caught), // sub_1411aeec0
    Replacement::draft(0x1411af4d0, "coreshared_1411af4d0", fn_1411af4d0 as *const ()), // coreshared::NavmeshResource::coreshared::NavmeshResource::vfunc_0
    Replacement::draft(0x1411af530, "coreshared_1411af530", tpl_10085::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x2c, 0xffffffffffffffec, 0xffffffffffffffe4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_1411af530
    Replacement::draft(0x1411af5d0, "coreshared_1411af5d0", fn_1411af5d0 as *const ()), // sub_1411af5d0
    Replacement::draft_throws(0x1411af6e0, "coreshared_1411af6e0", fn_1411af6e0 as *const (), Throws::Discard), // sub_1411af6e0
    Replacement::draft(0x1411af810, "coreshared_1411af810", fn_1411af810 as *const ()), // sub_1411af810
    Replacement::draft_throws(0x1411af8f0, "coreshared_1411af8f0", fn_1411af8f0 as *const (), Throws::Discard), // sub_1411af8f0
    Replacement::draft(0x1411af9f0, "coreshared_1411af9f0", fn_1411af9f0 as *const ()), // sub_1411af9f0
    Replacement::draft_throws(0x1411afa70, "coreshared_1411afa70", fn_1411afa70 as *const (), Throws::Caught), // sub_1411afa70
    Replacement::draft_throws(0x1411afbb0, "coreshared_1411afbb0", fn_1411afbb0 as *const (), Throws::Caught), // sub_1411afbb0
    Replacement::draft_throws(0x1411afcf0, "coreshared_1411afcf0", fn_1411afcf0 as *const (), Throws::Caught), // sub_1411afcf0
    Replacement::draft(0x1411afe30, "coreshared_1411afe30", fn_1411afe30 as *const ()), // sub_1411afe30
    Replacement::draft(0x1411aff00, "coreshared_1411aff00", fn_1411aff00 as *const ()), // sub_1411aff00
    Replacement::draft(0x1411affd0, "coreshared_1411affd0", fn_1411affd0 as *const ()), // sub_1411affd0
    Replacement::draft_throws(0x1411b00b0, "coreshared_1411b00b0", fn_1411b00b0 as *const (), Throws::Discard), // sub_1411b00b0
    Replacement::draft_throws(0x1411b01b0, "coreshared_1411b01b0", fn_1411b01b0 as *const (), Throws::Discard), // sub_1411b01b0
    Replacement::draft(0x1411b02e0, "coreshared_1411b02e0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433d7580, 0x8> as *const ()), // sub_1411b02e0
    Replacement::draft(0x1411b0360, "coreshared_1411b0360", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d7580, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411b0360
    Replacement::draft_throws(0x1411b0400, "coreshared_1411b0400", fn_1411b0400 as *const (), Throws::Caught), // sub_1411b0400
    Replacement::draft_throws(0x1411b0490, "coreshared_1411b0490", tpl_1908::<0x143881530, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411b0490
    Replacement::draft(0x1411b0510, "coreshared_1411b0510", fn_1411b0510 as *const ()), // sub_1411b0510
    Replacement::draft(0x1411b05e0, "coreshared_1411b05e0", fn_1411b05e0 as *const ()), // sub_1411b05e0
    Replacement::draft(0x1411b06e0, "coreshared_1411b06e0", tpl_9615::<0x143881518, 0x14243b0b0, 0x143881f80, 0x8, 0x143881538, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8> as *const ()), // sub_1411b06e0
    Replacement::draft(0x1411b07c0, "coreshared_1411b07c0", tpl_13342::<0x143881518, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411af5d0> as *const ()), // sub_1411b07c0
    Replacement::draft_throws(0x1411b0870, "coreshared_1411b0870", tpl_9846::<0x143881528, 0x143881518, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70, 0xb8, 0x1411b1030> as *const (), Throws::Discard), // sub_1411b0870
    Replacement::draft(0x1411b0900, "coreshared_1411b0900", tpl_13342::<0x143881518, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411af810> as *const ()), // sub_1411b0900
    Replacement::draft_throws(0x1411b09b0, "coreshared_1411b09b0", tpl_9846::<0x143881528, 0x143881518, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20, 0xb8, 0x1411b12a0> as *const (), Throws::Discard), // sub_1411b09b0
    Replacement::draft_throws(0x1411b0a40, "coreshared_1411b0a40", fn_1411b0a40 as *const (), Throws::Caught), // sub_1411b0a40
    Replacement::draft_throws(0x1411b0b00, "coreshared_1411b0b00", fn_1411b0b00 as *const (), Throws::Caught), // sub_1411b0b00
    Replacement::draft_throws(0x1411b0bc0, "coreshared_1411b0bc0", fn_1411b0bc0 as *const (), Throws::Caught), // sub_1411b0bc0
    Replacement::draft(0x1411b0c80, "coreshared_1411b0c80", tpl_13344::<0x143881518, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xb8, 0x1411afe30> as *const ()), // sub_1411b0c80
    Replacement::draft(0x1411b0d40, "coreshared_1411b0d40", tpl_9842::<0x143881528, 0x143881518, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860, 0xb8, 0x1411aff00> as *const ()), // sub_1411b0d40
    Replacement::draft(0x1411b0dd0, "coreshared_1411b0dd0", tpl_13342::<0x143881518, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411affd0> as *const ()), // sub_1411b0dd0
    Replacement::draft_throws(0x1411b0e80, "coreshared_1411b0e80", tpl_9843::<0x143881528, 0x143881518, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590, 0xb8, 0x1411b00b0> as *const (), Throws::Discard), // sub_1411b0e80
    Replacement::draft_throws(0x1411b0f10, "coreshared_1411b0f10", fn_1411b0f10 as *const (), Throws::Caught), // sub_1411b0f10
    Replacement::draft_throws(0x1411b1030, "coreshared_1411b1030", fn_1411b1030 as *const (), Throws::Caught), // sub_1411b1030
    Replacement::draft_throws(0x1411b12a0, "coreshared_1411b12a0", fn_1411b12a0 as *const (), Throws::Caught), // sub_1411b12a0
    Replacement::draft_throws(0x1411b1510, "coreshared_1411b1510", fn_1411b1510 as *const (), Throws::Caught), // sub_1411b1510
    Replacement::draft_throws(0x1411b1790, "coreshared_1411b1790", fn_1411b1790 as *const (), Throws::Caught), // sub_1411b1790
    Replacement::draft_throws(0x1411b1a10, "coreshared_1411b1a10", fn_1411b1a10 as *const (), Throws::Caught), // sub_1411b1a10
    Replacement::draft_throws(0x1411b1c90, "coreshared_1411b1c90", fn_1411b1c90 as *const (), Throws::Caught), // sub_1411b1c90
    Replacement::draft_throws(0x1411b2370, "coreshared_1411b2370", fn_1411b2370 as *const (), Throws::Caught), // sub_1411b2370
    Replacement::draft_throws(0x1411b2a10, "coreshared_1411b2a10", fn_1411b2a10 as *const (), Throws::Caught), // sub_1411b2a10
    Replacement::draft(0x1411b2f10, "coreshared_1411b2f10", fn_1411b2f10 as *const ()), // coreshared::AICoverResource::coreshared::AICoverResource::vfunc_0
    Replacement::draft(0x1411b2f60, "coreshared_1411b2f60", fn_1411b2f60 as *const ()), // sub_1411b2f60
    Replacement::draft_throws(0x1411b3060, "coreshared_1411b3060", fn_1411b3060 as *const (), Throws::Discard), // sub_1411b3060
    Replacement::draft(0x1411b3140, "coreshared_1411b3140", fn_1411b3140 as *const ()), // sub_1411b3140
    Replacement::draft_throws(0x1411b31d0, "coreshared_1411b31d0", fn_1411b31d0 as *const (), Throws::Discard), // sub_1411b31d0
    Replacement::draft_throws(0x1411b32c0, "coreshared_1411b32c0", fn_1411b32c0 as *const (), Throws::Caught), // sub_1411b32c0
    Replacement::draft_throws(0x1411b33f0, "coreshared_1411b33f0", fn_1411b33f0 as *const (), Throws::Caught), // sub_1411b33f0
    Replacement::draft_throws(0x1411b3520, "coreshared_1411b3520", fn_1411b3520 as *const (), Throws::Caught), // sub_1411b3520
    Replacement::draft(0x1411b3650, "coreshared_1411b3650", fn_1411b3650 as *const ()), // sub_1411b3650
    Replacement::draft(0x1411b36e0, "coreshared_1411b36e0", tpl_74766::<0x143881578, 0x143881558, 0x1401e9970, 0x338, 0xe8, 0x4, 0x8, 0x88, 0x1401e9860> as *const ()), // sub_1411b36e0
    Replacement::draft(0x1411b3770, "coreshared_1411b3770", fn_1411b3770 as *const ()), // sub_1411b3770
    Replacement::draft_throws(0x1411b3810, "coreshared_1411b3810", fn_1411b3810 as *const (), Throws::Discard), // sub_1411b3810
    Replacement::draft_throws(0x1411b38b0, "coreshared_1411b38b0", fn_1411b38b0 as *const (), Throws::Discard), // sub_1411b38b0
    Replacement::draft(0x1411b39d0, "coreshared_1411b39d0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433d8648, 0x8> as *const ()), // sub_1411b39d0
    Replacement::draft(0x1411b3a50, "coreshared_1411b3a50", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d8648, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411b3a50
    Replacement::draft_throws(0x1411b3af0, "coreshared_1411b3af0", fn_1411b3af0 as *const (), Throws::Caught), // sub_1411b3af0
    Replacement::draft_throws(0x1411b3b80, "coreshared_1411b3b80", tpl_1908::<0x143881570, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411b3b80
    Replacement::draft(0x1411b3c00, "coreshared_1411b3c00", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433d8a78, 0x8> as *const ()), // sub_1411b3c00
    Replacement::draft(0x1411b3c80, "coreshared_1411b3c80", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d8a78, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411b3c80
    Replacement::draft_throws(0x1411b3d20, "coreshared_1411b3d20", fn_1411b3d20 as *const (), Throws::Caught), // sub_1411b3d20
    Replacement::draft_throws(0x1411b3db0, "coreshared_1411b3db0", tpl_1908::<0x143881580, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411b3db0
    Replacement::draft(0x1411b3e30, "coreshared_1411b3e30", tpl_21685::<0xe0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x1433d84c0, 0xb8, 0xc0, 0xc8> as *const ()), // sub_1411b3e30
    Replacement::draft(0x1411b3ef0, "coreshared_1411b3ef0", tpl_21686::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d84c0, 0xffffffffffffff40, 0x14183acc2, 0x142428d70, 0xfffffffffffffff8> as *const ()), // sub_1411b3ef0
    Replacement::draft(0x1411b3fe0, "coreshared_1411b3fe0", tpl_9615::<0x143881548, 0x14243b0b0, 0x143881f80, 0x8, 0x143881558, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8> as *const ()), // sub_1411b3fe0
    Replacement::draft(0x1411b40c0, "coreshared_1411b40c0", tpl_13342::<0x143881548, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411b2f60> as *const ()), // sub_1411b40c0
    Replacement::draft_throws(0x1411b4170, "coreshared_1411b4170", tpl_9846::<0x143881550, 0x143881548, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70, 0xb8, 0x1411b5390> as *const (), Throws::Discard), // sub_1411b4170
    Replacement::draft(0x1411b4200, "coreshared_1411b4200", tpl_13342::<0x143881548, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411b3140> as *const ()), // sub_1411b4200
    Replacement::draft_throws(0x1411b42b0, "coreshared_1411b42b0", tpl_9846::<0x143881550, 0x143881548, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20, 0xb8, 0x1411b55f0> as *const (), Throws::Discard), // sub_1411b42b0
    Replacement::draft_throws(0x1411b4340, "coreshared_1411b4340", fn_1411b4340 as *const (), Throws::Caught), // sub_1411b4340
    Replacement::draft_throws(0x1411b4400, "coreshared_1411b4400", fn_1411b4400 as *const (), Throws::Caught), // sub_1411b4400
    Replacement::draft_throws(0x1411b44c0, "coreshared_1411b44c0", fn_1411b44c0 as *const (), Throws::Caught), // sub_1411b44c0
    Replacement::draft(0x1411b4580, "coreshared_1411b4580", tpl_13344::<0x143881548, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xb8, 0x1411b3650> as *const ()), // sub_1411b4580
    Replacement::draft(0x1411b4640, "coreshared_1411b4640", tpl_9842::<0x143881550, 0x143881548, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860, 0xb8, 0x1411b36e0> as *const ()), // sub_1411b4640
    Replacement::draft(0x1411b46d0, "coreshared_1411b46d0", tpl_13342::<0x143881548, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411b3770> as *const ()), // sub_1411b46d0
    Replacement::draft_throws(0x1411b4780, "coreshared_1411b4780", tpl_9843::<0x143881550, 0x143881548, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590, 0xb8, 0x1411b3810> as *const (), Throws::Discard), // sub_1411b4780
    Replacement::draft_throws(0x1411b4810, "coreshared_1411b4810", tpl_21687::<0x143881548, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80, 0xb8, 0x1411b5f70> as *const (), Throws::Discard), // sub_1411b4810
    Replacement::draft(0x1411b48e0, "coreshared_1411b48e0", tpl_21685::<0xe0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x1433d8028, 0xb8, 0xc0, 0xc8> as *const ()), // sub_1411b48e0
    Replacement::draft(0x1411b49a0, "coreshared_1411b49a0", tpl_21686::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d8028, 0xffffffffffffff40, 0x14183acc2, 0x142428d70, 0xfffffffffffffff8> as *const ()), // sub_1411b49a0
    Replacement::draft(0x1411b4a90, "coreshared_1411b4a90", tpl_9615::<0x143881540, 0x14243b0b0, 0x143881f80, 0x8, 0x143881558, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8> as *const ()), // sub_1411b4a90
    Replacement::draft(0x1411b4b70, "coreshared_1411b4b70", tpl_13342::<0x143881540, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411b2f60> as *const ()), // sub_1411b4b70
    Replacement::draft_throws(0x1411b4c20, "coreshared_1411b4c20", tpl_9846::<0x143881564, 0x143881540, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70, 0xb8, 0x1411b5390> as *const (), Throws::Discard), // sub_1411b4c20
    Replacement::draft(0x1411b4cb0, "coreshared_1411b4cb0", tpl_13342::<0x143881540, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411b3140> as *const ()), // sub_1411b4cb0
    Replacement::draft_throws(0x1411b4d60, "coreshared_1411b4d60", tpl_9846::<0x143881564, 0x143881540, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20, 0xb8, 0x1411b55f0> as *const (), Throws::Discard), // sub_1411b4d60
    Replacement::draft_throws(0x1411b4df0, "coreshared_1411b4df0", fn_1411b4df0 as *const (), Throws::Caught), // sub_1411b4df0
    Replacement::draft_throws(0x1411b4eb0, "coreshared_1411b4eb0", fn_1411b4eb0 as *const (), Throws::Caught), // sub_1411b4eb0
    Replacement::draft_throws(0x1411b4f70, "coreshared_1411b4f70", fn_1411b4f70 as *const (), Throws::Caught), // sub_1411b4f70
    Replacement::draft(0x1411b5030, "coreshared_1411b5030", tpl_13344::<0x143881540, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xb8, 0x1411b3650> as *const ()), // sub_1411b5030
    Replacement::draft(0x1411b50f0, "coreshared_1411b50f0", tpl_9842::<0x143881564, 0x143881540, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860, 0xb8, 0x1411b36e0> as *const ()), // sub_1411b50f0
    Replacement::draft(0x1411b5180, "coreshared_1411b5180", tpl_13342::<0x143881540, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x1411b3770> as *const ()), // sub_1411b5180
    Replacement::draft_throws(0x1411b5230, "coreshared_1411b5230", tpl_9843::<0x143881564, 0x143881540, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590, 0xb8, 0x1411b3810> as *const (), Throws::Discard), // sub_1411b5230
    Replacement::draft_throws(0x1411b52c0, "coreshared_1411b52c0", tpl_21687::<0x143881540, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80, 0xb8, 0x1411b5f70> as *const (), Throws::Discard), // sub_1411b52c0
    Replacement::draft_throws(0x1411b5390, "coreshared_1411b5390", fn_1411b5390 as *const (), Throws::Caught), // sub_1411b5390
    Replacement::draft_throws(0x1411b55f0, "coreshared_1411b55f0", fn_1411b55f0 as *const (), Throws::Caught), // sub_1411b55f0
    Replacement::draft_throws(0x1411b5850, "coreshared_1411b5850", fn_1411b5850 as *const (), Throws::Caught), // sub_1411b5850
    Replacement::draft_throws(0x1411b5ab0, "coreshared_1411b5ab0", fn_1411b5ab0 as *const (), Throws::Caught), // sub_1411b5ab0
    Replacement::draft_throws(0x1411b5d10, "coreshared_1411b5d10", fn_1411b5d10 as *const (), Throws::Caught), // sub_1411b5d10
    Replacement::draft_throws(0x1411b5f70, "coreshared_1411b5f70", fn_1411b5f70 as *const (), Throws::Caught), // sub_1411b5f70
    Replacement::draft_throws(0x1411b5fe0, "coreshared_1411b5fe0", fn_1411b5fe0 as *const (), Throws::Caught), // sub_1411b5fe0
    Replacement::draft_throws(0x1411b6680, "coreshared_1411b6680", fn_1411b6680 as *const (), Throws::Caught), // sub_1411b6680
    Replacement::draft_throws(0x1411b6ce0, "coreshared_1411b6ce0", fn_1411b6ce0 as *const (), Throws::Caught), // sub_1411b6ce0
    Replacement::draft(0x1411b7120, "coreshared_1411b7120", tpl_21689::<0x1420299d0, 0x141834d30> as *const ()), // coreshared::HeightfieldManager::coreshared::HeightfieldManager::vfunc_0
    Replacement::draft(0x1411b7160, "unknown_1411b7160", tpl_21690::<0x1401e5380, 0x8, 0x1411b8940, 0x1401e66f0> as *const ()), // sub_1411b7160
    Replacement::draft(0x1411b7190, "unknown_1411b7190", fn_1411b7190 as *const ()), // sub_1411b7190
    Replacement::draft(0x1411b7230, "unknown_1411b7230", fn_1411b7230 as *const ()), // sub_1411b7230
    Replacement::draft(0x1411b7400, "unknown_1411b7400", tpl_21430::<0x1411b8940, 0x1401e66f0> as *const ()), // sub_1411b7400
    Replacement::draft(0x1411b7430, "unknown_1411b7430", fn_1411b7430 as *const ()), // sub_1411b7430
    Replacement::draft(0x1411b7510, "unknown_1411b7510", fn_1411b7510 as *const ()), // sub_1411b7510
    Replacement::draft(0x1411b7650, "unknown_1411b7650", fn_1411b7650 as *const ()), // sub_1411b7650
    Replacement::draft(0x1411b7850, "unknown_1411b7850", fn_1411b7850 as *const ()), // sub_1411b7850
    Replacement::draft(0x1411b79c0, "unknown_1411b79c0", fn_1411b79c0 as *const ()), // sub_1411b79c0
    Replacement::draft(0x1411b7b30, "unknown_1411b7b30", fn_1411b7b30 as *const ()), // sub_1411b7b30
    Replacement::draft_throws(0x1411b7cc0, "std_1411b7cc0", fn_1411b7cc0 as *const (), Throws::Discard), // sub_1411b7cc0
    Replacement::draft_throws(0x1411b7f70, "std_1411b7f70", fn_1411b7f70 as *const (), Throws::Discard), // sub_1411b7f70
    Replacement::draft_throws(0x1411b8290, "std_1411b8290", fn_1411b8290 as *const (), Throws::Discard), // sub_1411b8290
    Replacement::draft_throws(0x1411b8620, "std_1411b8620", fn_1411b8620 as *const (), Throws::Discard), // sub_1411b8620
    Replacement::draft(0x1411b8940, "std_1411b8940", fn_1411b8940 as *const ()), // sub_1411b8940
    Replacement::draft(0x1411b89e0, "std_1411b89e0", fn_1411b89e0 as *const ()), // sub_1411b89e0
    Replacement::draft_throws(0x1411b8a50, "std_1411b8a50", fn_1411b8a50 as *const (), Throws::Discard), // sub_1411b8a50
    Replacement::draft(0x1411b8d00, "unknown_1411b8d00", fn_1411b8d00 as *const ()), // sub_1411b8d00
    Replacement::draft(0x1411b8fa0, "unknown_1411b8fa0", fn_1411b8fa0 as *const ()), // sub_1411b8fa0
    Replacement::draft(0x1411b9010, "unknown_1411b9010", fn_1411b9010 as *const ()), // sub_1411b9010
    Replacement::draft(0x1411b9190, "unknown_1411b9190", fn_1411b9190 as *const ()), // sub_1411b9190
    Replacement::draft(0x1411b91b0, "unknown_1411b91b0", fn_1411b91b0 as *const ()), // sub_1411b91b0
    Replacement::draft(0x1411b91d0, "unknown_1411b91d0", fn_1411b91d0 as *const ()), // sub_1411b91d0
    Replacement::draft(0x1411b9330, "unknown_1411b9330", fn_1411b9330 as *const ()), // sub_1411b9330
    Replacement::draft(0x1411b9aa0, "unknown_1411b9aa0", fn_1411b9aa0 as *const ()), // sub_1411b9aa0
    Replacement::draft(0x1411b9d20, "unknown_1411b9d20", fn_1411b9d20 as *const ()), // sub_1411b9d20
    Replacement::draft(0x1411b9ed0, "unknown_1411b9ed0", tpl_381::<0x1411ba190, 0x1411ba510> as *const ()), // sub_1411b9ed0
    Replacement::draft(0x1411ba030, "unknown_1411ba030", tpl_381::<0x1411ba850, 0x1411bac20> as *const ()), // sub_1411ba030
    Replacement::draft(0x1411ba190, "unknown_1411ba190", fn_1411ba190 as *const ()), // sub_1411ba190
    Replacement::draft(0x1411ba510, "unknown_1411ba510", fn_1411ba510 as *const ()), // sub_1411ba510
    Replacement::draft(0x1411ba850, "unknown_1411ba850", fn_1411ba850 as *const ()), // sub_1411ba850
    Replacement::draft(0x1411bac20, "unknown_1411bac20", fn_1411bac20 as *const ()), // sub_1411bac20
    Replacement::draft(0x1411baf80, "unknown_1411baf80", fn_1411baf80 as *const ()), // sub_1411baf80
    Replacement::draft(0x1411bafd0, "unknown_1411bafd0", fn_1411bafd0 as *const ()), // sub_1411bafd0
    Replacement::draft(0x1411bb050, "unknown_1411bb050", fn_1411bb050 as *const ()), // sub_1411bb050
    Replacement::draft(0x1411bb3b0, "coreshared_1411bb3b0", fn_1411bb3b0 as *const ()), // coreshared::GfxGraphResource::coreshared::GfxGraphResource::vfunc_0
    Replacement::draft(0x1411bb440, "unknown_1411bb440", fn_1411bb440 as *const ()), // sub_1411bb440
    Replacement::draft(0x1411bb4f0, "unknown_1411bb4f0", fn_1411bb4f0 as *const ()), // sub_1411bb4f0
    Replacement::draft(0x1411bb5d0, "unknown_1411bb5d0", fn_1411bb5d0 as *const ()), // sub_1411bb5d0
    Replacement::draft_throws(0x1411bb670, "unknown_1411bb670", tpl_21719::<0x143881590, 0x1438815a8, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1411bb670
    Replacement::draft_throws(0x1411bb6f0, "unknown_1411bb6f0", tpl_21719::<0x143881590, 0x1438815a8, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1411bb6f0
    Replacement::draft_throws(0x1411bb770, "unknown_1411bb770", fn_1411bb770 as *const (), Throws::Caught), // sub_1411bb770
    Replacement::draft(0x1411bb800, "unknown_1411bb800", fn_1411bb800 as *const ()), // sub_1411bb800
    Replacement::draft_throws(0x1411bb880, "unknown_1411bb880", tpl_21719::<0x143881590, 0x1438815a8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1411bb880
    Replacement::draft_throws(0x1411bb900, "unknown_1411bb900", fn_1411bb900 as *const (), Throws::Discard), // sub_1411bb900
    Replacement::draft(0x1411bb9b0, "unknown_1411bb9b0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433d9120, 0x8> as *const ()), // sub_1411bb9b0
    Replacement::draft(0x1411bba30, "unknown_1411bba30", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d9120, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411bba30
    Replacement::draft_throws(0x1411bbad0, "unknown_1411bbad0", fn_1411bbad0 as *const (), Throws::Caught), // sub_1411bbad0
    Replacement::draft_throws(0x1411bbb60, "unknown_1411bbb60", tpl_1908::<0x143881588, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411bbb60
    Replacement::draft(0x1411bbbe0, "unknown_1411bbbe0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433d8e78, 0x8> as *const ()), // sub_1411bbbe0
    Replacement::draft(0x1411bbc60, "unknown_1411bbc60", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433d8e78, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411bbc60
    Replacement::draft(0x1411bbd00, "unknown_1411bbd00", fn_1411bbd00 as *const ()), // sub_1411bbd00
    Replacement::draft_throws(0x1411bbda0, "unknown_1411bbda0", tpl_21724::<0x1438815a0, 0x143881598, 0x1401e9fa0, 0x318, 0x328, 0x320, 0x8, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_1411bbda0
    Replacement::draft_throws(0x1411bbe30, "unknown_1411bbe30", tpl_21724::<0x1438815a0, 0x143881598, 0x1401e9b60, 0x318, 0x328, 0x320, 0x8, 0x28, 0x1401e9a20> as *const (), Throws::Discard), // sub_1411bbe30
    Replacement::draft_throws(0x1411bbec0, "unknown_1411bbec0", fn_1411bbec0 as *const (), Throws::Caught), // sub_1411bbec0
    Replacement::draft(0x1411bbf50, "unknown_1411bbf50", fn_1411bbf50 as *const ()), // sub_1411bbf50
    Replacement::draft_throws(0x1411bbfe0, "unknown_1411bbfe0", tpl_21724::<0x1438815a0, 0x143881598, 0x1401e9770, 0x318, 0x328, 0x320, 0x8, 0x28, 0x1401e9590> as *const (), Throws::Discard), // sub_1411bbfe0
    Replacement::draft_throws(0x1411bc070, "unknown_1411bc070", fn_1411bc070 as *const (), Throws::Discard), // sub_1411bc070
    Replacement::draft(0x1411bc0f0, "unknown_1411bc0f0", fn_1411bc0f0 as *const ()), // sub_1411bc0f0
    Replacement::draft(0x1411bc1b0, "unknown_1411bc1b0", fn_1411bc1b0 as *const ()), // sub_1411bc1b0
    Replacement::draft(0x1411bc240, "unknown_1411bc240", fn_1411bc240 as *const ()), // sub_1411bc240
    Replacement::draft(0x1411bc330, "unknown_1411bc330", fn_1411bc330 as *const ()), // sub_1411bc330
    Replacement::draft(0x1411bc4c0, "unknown_1411bc4c0", fn_1411bc4c0 as *const ()), // sub_1411bc4c0
    Replacement::draft(0x1411bc630, "unknown_1411bc630", fn_1411bc630 as *const ()), // sub_1411bc630
    Replacement::draft(0x1411bc720, "std_1411bc720", tpl_21733::<0x1433d93f8, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<rend::GfxGraphResourceInfo,r::Allocator<rend::GfxGraphResourceInfo,0>>::std::_Ref_count_obj_a
    Replacement::draft(0x1411bc800, "unknown_1411bc800", fn_1411bc800 as *const ()), // sub_1411bc800
    Replacement::draft(0x1411bc940, "unknown_1411bc940", fn_1411bc940 as *const ()), // sub_1411bc940
    Replacement::draft(0x1411bcab0, "unknown_1411bcab0", fn_1411bcab0 as *const ()), // sub_1411bcab0
    Replacement::draft(0x1411bcc60, "unknown_1411bcc60", tpl_381::<0x1411bcf20, 0x1411bd200> as *const ()), // sub_1411bcc60
    Replacement::draft(0x1411bcdc0, "unknown_1411bcdc0", tpl_381::<0x1411bd520, 0x1411bd780> as *const ()), // sub_1411bcdc0
    Replacement::draft(0x1411bcf20, "unknown_1411bcf20", fn_1411bcf20 as *const ()), // sub_1411bcf20
    Replacement::draft(0x1411bd200, "unknown_1411bd200", fn_1411bd200 as *const ()), // sub_1411bd200
    Replacement::draft(0x1411bd520, "unknown_1411bd520", fn_1411bd520 as *const ()), // sub_1411bd520
    Replacement::draft(0x1411bd780, "unknown_1411bd780", fn_1411bd780 as *const ()), // sub_1411bd780
    Replacement::draft(0x1411bda50, "unknown_1411bda50", fn_1411bda50 as *const ()), // sub_1411bda50
    Replacement::draft(0x1411bdc20, "unknown_1411bdc20", fn_1411bdc20 as *const ()), // sub_1411bdc20
    Replacement::draft(0x1411bdca0, "std_1411bdca0", fn_1411bdca0 as *const ()), // sub_1411bdca0
    Replacement::draft(0x1411bde30, "unknown_1411bde30", fn_1411bde30 as *const ()), // sub_1411bde30
    Replacement::draft(0x1411bdfc0, "unknown_1411bdfc0", fn_1411bdfc0 as *const ()), // sub_1411bdfc0
    Replacement::draft(0x1411be000, "unknown_1411be000", fn_1411be000 as *const ()), // sub_1411be000
    Replacement::draft(0x1411be040, "unknown_1411be040", fn_1411be040 as *const ()), // sub_1411be040
    Replacement::draft(0x1411be070, "unknown_1411be070", fn_1411be070 as *const ()), // sub_1411be070
    Replacement::draft(0x1411be090, "unknown_1411be090", fn_1411be090 as *const ()), // sub_1411be090
    Replacement::draft(0x1411be0b0, "unknown_1411be0b0", fn_1411be0b0 as *const ()), // sub_1411be0b0
    Replacement::draft(0x1411be0d0, "unknown_1411be0d0", fn_1411be0d0 as *const ()), // sub_1411be0d0
    Replacement::draft_throws(0x1411be0f0, "unknown_1411be0f0", fn_1411be0f0 as *const (), Throws::Discard), // sub_1411be0f0
    Replacement::draft_throws(0x1411be160, "unknown_1411be160", fn_1411be160 as *const (), Throws::Caught), // sub_1411be160
    Replacement::draft(0x1411be310, "unknown_1411be310", fn_1411be310 as *const ()), // sub_1411be310
    Replacement::draft_throws(0x1411be3b0, "unknown_1411be3b0", fn_1411be3b0 as *const (), Throws::Caught), // sub_1411be3b0
    Replacement::draft(0x1411be560, "unknown_1411be560", fn_1411be560 as *const ()), // sub_1411be560
    Replacement::draft(0x1411be600, "unknown_1411be600", fn_1411be600 as *const ()), // sub_1411be600
    Replacement::draft(0x1411be860, "unknown_1411be860", fn_1411be860 as *const ()), // sub_1411be860
    Replacement::draft(0x1411be9d0, "unknown_1411be9d0", fn_1411be9d0 as *const ()), // sub_1411be9d0
    Replacement::draft(0x1411beba0, "unknown_1411beba0", fn_1411beba0 as *const ()), // sub_1411beba0
    Replacement::draft(0x1411bebd0, "unknown_1411bebd0", fn_1411bebd0 as *const ()), // sub_1411bebd0
    Replacement::draft_throws(0x1411bed40, "unknown_1411bed40", fn_1411bed40 as *const (), Throws::Discard), // sub_1411bed40
    Replacement::draft(0x1411bee00, "unknown_1411bee00", fn_1411bee00 as *const ()), // sub_1411bee00
    Replacement::draft_throws(0x1411befb0, "unknown_1411befb0", fn_1411befb0 as *const (), Throws::Caught), // sub_1411befb0
    Replacement::draft(0x1411bf050, "unknown_1411bf050", fn_1411bf050 as *const ()), // sub_1411bf050
    Replacement::draft(0x1411bf0d0, "unknown_1411bf0d0", fn_1411bf0d0 as *const ()), // sub_1411bf0d0
    Replacement::draft(0x1411bf150, "unknown_1411bf150", fn_1411bf150 as *const ()), // sub_1411bf150
    Replacement::draft(0x1411bf190, "unknown_1411bf190", fn_1411bf190 as *const ()), // sub_1411bf190
    Replacement::draft_throws(0x1411bf3f0, "unknown_1411bf3f0", fn_1411bf3f0 as *const (), Throws::Discard), // sub_1411bf3f0
    Replacement::draft(0x1411bf470, "unknown_1411bf470", fn_1411bf470 as *const ()), // sub_1411bf470
    Replacement::draft(0x1411bf660, "physx_1411bf660", fn_1411bf660 as *const ()), // sub_1411bf660
    Replacement::draft(0x1411bf7d0, "unknown_1411bf7d0", fn_1411bf7d0 as *const ()), // sub_1411bf7d0
    Replacement::draft(0x1411bf850, "unknown_1411bf850", fn_1411bf850 as *const ()), // sub_1411bf850
    Replacement::draft(0x1411bf8d0, "unknown_1411bf8d0", fn_1411bf8d0 as *const ()), // sub_1411bf8d0
    Replacement::draft(0x1411bfa40, "unknown_1411bfa40", fn_1411bfa40 as *const ()), // sub_1411bfa40
    Replacement::draft(0x1411bfbb0, "unknown_1411bfbb0", fn_1411bfbb0 as *const ()), // sub_1411bfbb0
    Replacement::draft_throws(0x1411bfd80, "unknown_1411bfd80", fn_1411bfd80 as *const (), Throws::Discard), // sub_1411bfd80
    Replacement::draft_throws(0x1411bfe60, "unknown_1411bfe60", fn_1411bfe60 as *const (), Throws::Discard), // sub_1411bfe60
    Replacement::draft_throws(0x1411bfee0, "unknown_1411bfee0", fn_1411bfee0 as *const (), Throws::Discard), // sub_1411bfee0
    Replacement::draft(0x1411bffa0, "unknown_1411bffa0", fn_1411bffa0 as *const ()), // sub_1411bffa0
    Replacement::draft_throws(0x1411c00a0, "unknown_1411c00a0", fn_1411c00a0 as *const (), Throws::Discard), // sub_1411c00a0
    Replacement::draft(0x1411c0120, "unknown_1411c0120", fn_1411c0120 as *const ()), // sub_1411c0120
    Replacement::draft_throws(0x1411c0160, "unknown_1411c0160", fn_1411c0160 as *const (), Throws::Discard), // sub_1411c0160
    Replacement::draft_throws(0x1411c01f0, "unknown_1411c01f0", fn_1411c01f0 as *const (), Throws::Discard), // sub_1411c01f0
    Replacement::draft(0x1411c03f0, "unknown_1411c03f0", fn_1411c03f0 as *const ()), // sub_1411c03f0
    Replacement::draft(0x1411c0410, "unknown_1411c0410", fn_1411c0410 as *const ()), // sub_1411c0410
    Replacement::draft(0x1411c0430, "unknown_1411c0430", tpl_20553::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_1411c0430
    Replacement::draft_throws(0x1411c0480, "unknown_1411c0480", fn_1411c0480 as *const (), Throws::Caught), // sub_1411c0480
    Replacement::draft_throws(0x1411c0610, "unknown_1411c0610", fn_1411c0610 as *const (), Throws::Caught), // sub_1411c0610
    Replacement::draft_throws(0x1411c07a0, "unknown_1411c07a0", fn_1411c07a0 as *const (), Throws::Discard), // sub_1411c07a0
    Replacement::draft_throws(0x1411c0910, "unknown_1411c0910", fn_1411c0910 as *const (), Throws::Caught), // sub_1411c0910
    Replacement::draft(0x1411c0ac0, "unknown_1411c0ac0", tpl_21787::<0x18> as *const ()), // sub_1411c0ac0
    Replacement::draft(0x1411c0ad0, "unknown_1411c0ad0", fn_1411c0ad0 as *const ()), // sub_1411c0ad0
    Replacement::draft_throws(0x1411c0ae0, "unknown_1411c0ae0", fn_1411c0ae0 as *const (), Throws::Discard), // sub_1411c0ae0
    Replacement::draft_throws(0x1411c0c50, "unknown_1411c0c50", fn_1411c0c50 as *const (), Throws::Caught), // sub_1411c0c50
    Replacement::draft_throws(0x1411c0e00, "physx_1411c0e00", fn_1411c0e00 as *const (), Throws::Caught), // sub_1411c0e00
    Replacement::draft_throws(0x1411c0fb0, "unknown_1411c0fb0", fn_1411c0fb0 as *const (), Throws::Discard), // sub_1411c0fb0
    Replacement::draft_throws(0x1411c1130, "unknown_1411c1130", fn_1411c1130 as *const (), Throws::Caught), // sub_1411c1130
    Replacement::draft(0x1411c11b0, "unknown_1411c11b0", fn_1411c11b0 as *const ()), // sub_1411c11b0
    Replacement::draft(0x1411c1cd0, "unknown_1411c1cd0", fn_1411c1cd0 as *const ()), // sub_1411c1cd0
    Replacement::draft(0x1411c1e20, "unknown_1411c1e20", fn_1411c1e20 as *const ()), // sub_1411c1e20
    Replacement::draft(0x1411c1f10, "unknown_1411c1f10", fn_1411c1f10 as *const ()), // sub_1411c1f10
    Replacement::draft(0x1411c1f90, "unknown_1411c1f90", fn_1411c1f90 as *const ()), // sub_1411c1f90
    Replacement::draft(0x1411c2010, "unknown_1411c2010", fn_1411c2010 as *const ()), // sub_1411c2010
    Replacement::draft(0x1411c20e0, "unknown_1411c20e0", fn_1411c20e0 as *const ()), // sub_1411c20e0
    Replacement::draft_throws(0x1411c2100, "unknown_1411c2100", fn_1411c2100 as *const (), Throws::Caught), // sub_1411c2100
    Replacement::draft_throws(0x1411c2290, "unknown_1411c2290", fn_1411c2290 as *const (), Throws::Caught), // sub_1411c2290
    Replacement::draft(0x1411c2420, "unknown_1411c2420", fn_1411c2420 as *const ()), // sub_1411c2420
    Replacement::draft(0x1411c2490, "unknown_1411c2490", fn_1411c2490 as *const ()), // sub_1411c2490
    Replacement::draft(0x1411c2510, "unknown_1411c2510", fn_1411c2510 as *const ()), // sub_1411c2510
    Replacement::draft(0x1411c2530, "unknown_1411c2530", fn_1411c2530 as *const ()), // sub_1411c2530
    Replacement::draft_throws(0x1411c25d0, "unknown_1411c25d0", fn_1411c25d0 as *const (), Throws::Caught), // sub_1411c25d0
    Replacement::draft(0x1411c28c0, "unknown_1411c28c0", fn_1411c28c0 as *const ()), // sub_1411c28c0
    Replacement::draft(0x1411c2960, "unknown_1411c2960", fn_1411c2960 as *const ()), // sub_1411c2960
    Replacement::draft(0x1411c2a00, "unknown_1411c2a00", fn_1411c2a00 as *const ()), // sub_1411c2a00
    Replacement::draft(0x1411c2a20, "unknown_1411c2a20", fn_1411c2a20 as *const ()), // sub_1411c2a20
    Replacement::draft(0x1411c2aa0, "unknown_1411c2aa0", fn_1411c2aa0 as *const ()), // sub_1411c2aa0
    Replacement::draft(0x1411c2bd0, "unknown_1411c2bd0", fn_1411c2bd0 as *const ()), // sub_1411c2bd0
    Replacement::draft(0x1411c2cc0, "unknown_1411c2cc0", fn_1411c2cc0 as *const ()), // sub_1411c2cc0
    Replacement::draft(0x1411c2d50, "unknown_1411c2d50", fn_1411c2d50 as *const ()), // sub_1411c2d50
    Replacement::draft(0x1411c2e10, "unknown_1411c2e10", fn_1411c2e10 as *const ()), // sub_1411c2e10
    Replacement::draft(0x1411c2f20, "unknown_1411c2f20", fn_1411c2f20 as *const ()), // sub_1411c2f20
    Replacement::draft(0x1411c2fa0, "unknown_1411c2fa0", fn_1411c2fa0 as *const ()), // sub_1411c2fa0
    Replacement::draft(0x1411c30f0, "unknown_1411c30f0", fn_1411c30f0 as *const ()), // sub_1411c30f0
    Replacement::draft(0x1411c3170, "unknown_1411c3170", fn_1411c3170 as *const ()), // sub_1411c3170
    Replacement::draft(0x1411c3240, "unknown_1411c3240", fn_1411c3240 as *const ()), // sub_1411c3240
    Replacement::draft(0x1411c32c0, "unknown_1411c32c0", fn_1411c32c0 as *const ()), // sub_1411c32c0
    Replacement::draft(0x1411c3390, "unknown_1411c3390", fn_1411c3390 as *const ()), // sub_1411c3390
    Replacement::draft(0x1411c3400, "unknown_1411c3400", fn_1411c3400 as *const ()), // sub_1411c3400
    Replacement::draft(0x1411c3550, "unknown_1411c3550", fn_1411c3550 as *const ()), // sub_1411c3550
    Replacement::draft(0x1411c3600, "unknown_1411c3600", fn_1411c3600 as *const ()), // sub_1411c3600
    Replacement::draft(0x1411c3740, "unknown_1411c3740", fn_1411c3740 as *const ()), // sub_1411c3740
    Replacement::draft(0x1411c3800, "unknown_1411c3800", fn_1411c3800 as *const ()), // sub_1411c3800
    Replacement::draft(0x1411c3940, "unknown_1411c3940", fn_1411c3940 as *const ()), // sub_1411c3940
    Replacement::draft(0x1411c39c0, "unknown_1411c39c0", fn_1411c39c0 as *const ()), // sub_1411c39c0
    Replacement::draft(0x1411c3a90, "unknown_1411c3a90", fn_1411c3a90 as *const ()), // sub_1411c3a90
    Replacement::draft(0x1411c3b10, "unknown_1411c3b10", fn_1411c3b10 as *const ()), // sub_1411c3b10
    Replacement::draft(0x1411c3be0, "unknown_1411c3be0", fn_1411c3be0 as *const ()), // sub_1411c3be0
    Replacement::draft(0x1411c3c60, "unknown_1411c3c60", fn_1411c3c60 as *const ()), // sub_1411c3c60
    Replacement::draft(0x1411c3d30, "unknown_1411c3d30", fn_1411c3d30 as *const ()), // sub_1411c3d30
    Replacement::draft(0x1411c3db0, "unknown_1411c3db0", fn_1411c3db0 as *const ()), // sub_1411c3db0
    Replacement::draft(0x1411c3f00, "unknown_1411c3f00", fn_1411c3f00 as *const ()), // sub_1411c3f00
    Replacement::draft(0x1411c3f80, "unknown_1411c3f80", fn_1411c3f80 as *const ()), // sub_1411c3f80
    Replacement::draft(0x1411c4000, "unknown_1411c4000", fn_1411c4000 as *const ()), // sub_1411c4000
    Replacement::draft(0x1411c4080, "unknown_1411c4080", fn_1411c4080 as *const ()), // sub_1411c4080
    Replacement::draft(0x1411c4100, "unknown_1411c4100", fn_1411c4100 as *const ()), // sub_1411c4100
    Replacement::draft(0x1411c4190, "unknown_1411c4190", fn_1411c4190 as *const ()), // sub_1411c4190
    Replacement::draft(0x1411c42b0, "unknown_1411c42b0", fn_1411c42b0 as *const ()), // sub_1411c42b0
    Replacement::draft(0x1411c4360, "unknown_1411c4360", fn_1411c4360 as *const ()), // sub_1411c4360
    Replacement::draft(0x1411c4460, "unknown_1411c4460", fn_1411c4460 as *const ()), // sub_1411c4460
    Replacement::draft(0x1411c4510, "unknown_1411c4510", fn_1411c4510 as *const ()), // sub_1411c4510
    Replacement::draft(0x1411c45a0, "unknown_1411c45a0", fn_1411c45a0 as *const ()), // sub_1411c45a0
    Replacement::draft(0x1411c4650, "unknown_1411c4650", fn_1411c4650 as *const ()), // sub_1411c4650
    Replacement::draft(0x1411c46e0, "unknown_1411c46e0", fn_1411c46e0 as *const ()), // sub_1411c46e0
    Replacement::draft(0x1411c4790, "unknown_1411c4790", fn_1411c4790 as *const ()), // sub_1411c4790
    Replacement::draft(0x1411c4890, "unknown_1411c4890", fn_1411c4890 as *const ()), // sub_1411c4890
    Replacement::draft(0x1411c4900, "unknown_1411c4900", fn_1411c4900 as *const ()), // sub_1411c4900
    Replacement::draft(0x1411c4a50, "unknown_1411c4a50", fn_1411c4a50 as *const ()), // sub_1411c4a50
    Replacement::draft(0x1411c4ac0, "unknown_1411c4ac0", fn_1411c4ac0 as *const ()), // sub_1411c4ac0
    Replacement::draft(0x1411c4c10, "unknown_1411c4c10", fn_1411c4c10 as *const ()), // sub_1411c4c10
    Replacement::draft(0x1411c4c80, "unknown_1411c4c80", fn_1411c4c80 as *const ()), // sub_1411c4c80
    Replacement::draft(0x1411c4d50, "unknown_1411c4d50", fn_1411c4d50 as *const ()), // sub_1411c4d50
    Replacement::draft(0x1411c4dc0, "unknown_1411c4dc0", fn_1411c4dc0 as *const ()), // sub_1411c4dc0
    Replacement::draft(0x1411c4e90, "unknown_1411c4e90", fn_1411c4e90 as *const ()), // sub_1411c4e90
    Replacement::draft(0x1411c4f10, "unknown_1411c4f10", fn_1411c4f10 as *const ()), // sub_1411c4f10
    Replacement::draft(0x1411c4f90, "unknown_1411c4f90", fn_1411c4f90 as *const ()), // sub_1411c4f90
    Replacement::draft(0x1411c5010, "unknown_1411c5010", fn_1411c5010 as *const ()), // sub_1411c5010
    Replacement::draft(0x1411c5090, "unknown_1411c5090", fn_1411c5090 as *const ()), // sub_1411c5090
    Replacement::draft(0x1411c5110, "unknown_1411c5110", fn_1411c5110 as *const ()), // sub_1411c5110
    Replacement::draft(0x1411c51e0, "unknown_1411c51e0", fn_1411c51e0 as *const ()), // sub_1411c51e0
    Replacement::draft(0x1411c5250, "unknown_1411c5250", fn_1411c5250 as *const ()), // sub_1411c5250
    Replacement::draft(0x1411c53a0, "unknown_1411c53a0", fn_1411c53a0 as *const ()), // sub_1411c53a0
    Replacement::draft(0x1411c5410, "unknown_1411c5410", fn_1411c5410 as *const ()), // sub_1411c5410
    Replacement::draft(0x1411c5560, "unknown_1411c5560", fn_1411c5560 as *const ()), // sub_1411c5560
    Replacement::draft(0x1411c55d0, "unknown_1411c55d0", fn_1411c55d0 as *const ()), // sub_1411c55d0
    Replacement::draft(0x1411c56a0, "unknown_1411c56a0", fn_1411c56a0 as *const ()), // sub_1411c56a0
    Replacement::draft(0x1411c5710, "unknown_1411c5710", fn_1411c5710 as *const ()), // sub_1411c5710
    Replacement::draft(0x1411c5860, "unknown_1411c5860", fn_1411c5860 as *const ()), // sub_1411c5860
    Replacement::draft(0x1411c6270, "unknown_1411c6270", fn_1411c6270 as *const ()), // sub_1411c6270
    Replacement::draft(0x1411c68b0, "unknown_1411c68b0", fn_1411c68b0 as *const ()), // sub_1411c68b0
    Replacement::draft(0x1411c68e0, "unknown_1411c68e0", fn_1411c68e0 as *const ()), // sub_1411c68e0
    Replacement::draft(0x1411c6910, "unknown_1411c6910", fn_1411c6910 as *const ()), // sub_1411c6910
    Replacement::draft(0x1411c6940, "unknown_1411c6940", fn_1411c6940 as *const ()), // sub_1411c6940
    Replacement::draft(0x1411c6980, "unknown_1411c6980", fn_1411c6980 as *const ()), // sub_1411c6980
    Replacement::draft(0x1411c69b0, "unknown_1411c69b0", fn_1411c69b0 as *const ()), // sub_1411c69b0
    Replacement::draft(0x1411c6a00, "unknown_1411c6a00", fn_1411c6a00 as *const ()), // sub_1411c6a00
    Replacement::draft(0x1411c6a30, "unknown_1411c6a30", fn_1411c6a30 as *const ()), // sub_1411c6a30
    Replacement::draft(0x1411c6a60, "unknown_1411c6a60", fn_1411c6a60 as *const ()), // sub_1411c6a60
    Replacement::draft(0x1411c6a90, "unknown_1411c6a90", fn_1411c6a90 as *const ()), // sub_1411c6a90
    Replacement::draft(0x1411c6ac0, "unknown_1411c6ac0", fn_1411c6ac0 as *const ()), // sub_1411c6ac0
    Replacement::draft(0x1411c6af0, "unknown_1411c6af0", fn_1411c6af0 as *const ()), // sub_1411c6af0
    Replacement::draft(0x1411c6b20, "unknown_1411c6b20", fn_1411c6b20 as *const ()), // sub_1411c6b20
    Replacement::draft(0x1411c6b50, "unknown_1411c6b50", fn_1411c6b50 as *const ()), // sub_1411c6b50
    Replacement::draft(0x1411c6b80, "unknown_1411c6b80", fn_1411c6b80 as *const ()), // sub_1411c6b80
    Replacement::draft(0x1411c6bb0, "unknown_1411c6bb0", fn_1411c6bb0 as *const ()), // sub_1411c6bb0
    Replacement::draft(0x1411c6be0, "unknown_1411c6be0", fn_1411c6be0 as *const ()), // sub_1411c6be0
    Replacement::draft(0x1411c6c10, "unknown_1411c6c10", fn_1411c6c10 as *const ()), // sub_1411c6c10
    Replacement::draft(0x1411c6c40, "unknown_1411c6c40", fn_1411c6c40 as *const ()), // sub_1411c6c40
    Replacement::draft(0x1411c6c70, "unknown_1411c6c70", fn_1411c6c70 as *const ()), // sub_1411c6c70
    Replacement::draft(0x1411c6ca0, "unknown_1411c6ca0", fn_1411c6ca0 as *const ()), // sub_1411c6ca0
    Replacement::draft(0x1411c6cd0, "unknown_1411c6cd0", fn_1411c6cd0 as *const ()), // sub_1411c6cd0
    Replacement::draft(0x1411c6d00, "unknown_1411c6d00", fn_1411c6d00 as *const ()), // sub_1411c6d00
    Replacement::draft(0x1411c6d30, "unknown_1411c6d30", fn_1411c6d30 as *const ()), // sub_1411c6d30
    Replacement::draft(0x1411c6d60, "unknown_1411c6d60", fn_1411c6d60 as *const ()), // sub_1411c6d60
    Replacement::draft(0x1411c6d90, "unknown_1411c6d90", fn_1411c6d90 as *const ()), // sub_1411c6d90
    Replacement::draft(0x1411c6db0, "unknown_1411c6db0", fn_1411c6db0 as *const ()), // sub_1411c6db0
    Replacement::draft(0x1411c6de0, "unknown_1411c6de0", fn_1411c6de0 as *const ()), // sub_1411c6de0
    Replacement::draft(0x1411c6e00, "unknown_1411c6e00", fn_1411c6e00 as *const ()), // sub_1411c6e00
    Replacement::draft(0x1411c6e30, "unknown_1411c6e30", fn_1411c6e30 as *const ()), // sub_1411c6e30
    Replacement::draft(0x1411c6e50, "unknown_1411c6e50", fn_1411c6e50 as *const ()), // sub_1411c6e50
    Replacement::draft(0x1411c6e80, "unknown_1411c6e80", fn_1411c6e80 as *const ()), // sub_1411c6e80
    Replacement::draft(0x1411c6ea0, "unknown_1411c6ea0", fn_1411c6ea0 as *const ()), // sub_1411c6ea0
    Replacement::draft(0x1411c6ed0, "unknown_1411c6ed0", fn_1411c6ed0 as *const ()), // sub_1411c6ed0
    Replacement::draft(0x1411c6f00, "unknown_1411c6f00", fn_1411c6f00 as *const ()), // sub_1411c6f00
    Replacement::draft(0x1411c6f30, "unknown_1411c6f30", fn_1411c6f30 as *const ()), // sub_1411c6f30
    Replacement::draft(0x1411c6f60, "unknown_1411c6f60", fn_1411c6f60 as *const ()), // sub_1411c6f60
    Replacement::draft(0x1411c6f90, "unknown_1411c6f90", fn_1411c6f90 as *const ()), // sub_1411c6f90
    Replacement::draft(0x1411c6fc0, "unknown_1411c6fc0", fn_1411c6fc0 as *const ()), // sub_1411c6fc0
    Replacement::draft(0x1411c6ff0, "unknown_1411c6ff0", fn_1411c6ff0 as *const ()), // sub_1411c6ff0
    Replacement::draft(0x1411c7020, "unknown_1411c7020", fn_1411c7020 as *const ()), // sub_1411c7020
    Replacement::draft(0x1411c7050, "unknown_1411c7050", fn_1411c7050 as *const ()), // sub_1411c7050
    Replacement::draft(0x1411c7080, "unknown_1411c7080", fn_1411c7080 as *const ()), // sub_1411c7080
    Replacement::draft(0x1411c70b0, "unknown_1411c70b0", fn_1411c70b0 as *const ()), // sub_1411c70b0
    Replacement::draft(0x1411c70e0, "unknown_1411c70e0", fn_1411c70e0 as *const ()), // sub_1411c70e0
    Replacement::draft(0x1411c7150, "unknown_1411c7150", fn_1411c7150 as *const ()), // sub_1411c7150
    Replacement::draft(0x1411c7250, "unknown_1411c7250", fn_1411c7250 as *const ()), // sub_1411c7250
    Replacement::draft(0x1411c7350, "unknown_1411c7350", fn_1411c7350 as *const ()), // sub_1411c7350
    Replacement::draft(0x1411c7450, "unknown_1411c7450", fn_1411c7450 as *const ()), // sub_1411c7450
    Replacement::draft(0x1411c7650, "unknown_1411c7650", fn_1411c7650 as *const ()), // sub_1411c7650
    Replacement::draft(0x1411c7850, "unknown_1411c7850", fn_1411c7850 as *const ()), // sub_1411c7850
    Replacement::draft(0x1411c7a50, "unknown_1411c7a50", tpl_21924::<0x1410c3eb0, 0x1425371a0, 0x142533200, 0x14241d6f0> as *const ()), // sub_1411c7a50
    Replacement::draft(0x1411c7c50, "unknown_1411c7c50", tpl_21924::<0x1410c3eb0, 0x1425371a0, 0x142533200, 0x14241d6f0> as *const ()), // sub_1411c7c50
    Replacement::draft(0x1411c7e50, "unknown_1411c7e50", fn_1411c7e50 as *const ()), // sub_1411c7e50
    Replacement::draft(0x1411c7ef0, "unknown_1411c7ef0", fn_1411c7ef0 as *const ()), // sub_1411c7ef0
    Replacement::draft(0x1411c7f90, "unknown_1411c7f90", fn_1411c7f90 as *const ()), // sub_1411c7f90
    Replacement::draft(0x1411c8030, "unknown_1411c8030", tpl_9010::<0x8, 0x1401dec00, 0x1410c05d0, 0xc> as *const ()), // sub_1411c8030
    Replacement::draft(0x1411c80b0, "unknown_1411c80b0", fn_1411c80b0 as *const ()), // sub_1411c80b0
    Replacement::draft(0x1411c8230, "unknown_1411c8230", fn_1411c8230 as *const ()), // sub_1411c8230
    Replacement::draft(0x1411c8480, "unknown_1411c8480", fn_1411c8480 as *const ()), // sub_1411c8480
    Replacement::draft(0x1411c86f0, "unknown_1411c86f0", fn_1411c86f0 as *const ()), // sub_1411c86f0
    Replacement::draft(0x1411c89c0, "unknown_1411c89c0", fn_1411c89c0 as *const ()), // sub_1411c89c0
    Replacement::draft(0x1411c8bd0, "unknown_1411c8bd0", fn_1411c8bd0 as *const ()), // sub_1411c8bd0
    Replacement::draft(0x1411c8ef0, "unknown_1411c8ef0", fn_1411c8ef0 as *const ()), // sub_1411c8ef0
    Replacement::draft(0x1411c9200, "unknown_1411c9200", fn_1411c9200 as *const ()), // sub_1411c9200
    Replacement::draft(0x1411c9480, "unknown_1411c9480", fn_1411c9480 as *const ()), // sub_1411c9480
    Replacement::draft(0x1411c9710, "unknown_1411c9710", fn_1411c9710 as *const ()), // sub_1411c9710
    Replacement::draft(0x1411c9780, "unknown_1411c9780", fn_1411c9780 as *const ()), // sub_1411c9780
    Replacement::draft(0x1411c97f0, "unknown_1411c97f0", fn_1411c97f0 as *const ()), // sub_1411c97f0
    Replacement::draft(0x1411c9860, "unknown_1411c9860", tpl_381::<0x1411c9ab0, 0x1411c9d40> as *const ()), // sub_1411c9860
    Replacement::draft(0x1411c9ab0, "unknown_1411c9ab0", fn_1411c9ab0 as *const ()), // sub_1411c9ab0
    Replacement::draft(0x1411c9d40, "unknown_1411c9d40", fn_1411c9d40 as *const ()), // sub_1411c9d40
    Replacement::draft(0x1411ca040, "unknown_1411ca040", fn_1411ca040 as *const ()), // sub_1411ca040
    Replacement::draft(0x1411ca0d0, "unknown_1411ca0d0", fn_1411ca0d0 as *const ()), // sub_1411ca0d0
    Replacement::draft(0x1411ca170, "unknown_1411ca170", fn_1411ca170 as *const ()), // sub_1411ca170
    Replacement::draft(0x1411ca220, "unknown_1411ca220", fn_1411ca220 as *const ()), // sub_1411ca220
    Replacement::draft_throws(0x1411ca240, "unknown_1411ca240", fn_1411ca240 as *const (), Throws::Discard), // sub_1411ca240
    Replacement::draft_throws(0x1411ca320, "unknown_1411ca320", fn_1411ca320 as *const (), Throws::Discard), // sub_1411ca320
    Replacement::draft_throws(0x1411ca400, "unknown_1411ca400", fn_1411ca400 as *const (), Throws::Discard), // sub_1411ca400
    Replacement::draft(0x1411ca480, "unknown_1411ca480", fn_1411ca480 as *const ()), // sub_1411ca480
    Replacement::draft_throws(0x1411ca510, "unknown_1411ca510", fn_1411ca510 as *const (), Throws::Discard), // sub_1411ca510
    Replacement::draft_throws(0x1411ca590, "unknown_1411ca590", fn_1411ca590 as *const (), Throws::Discard), // sub_1411ca590
    Replacement::draft_throws(0x1411ca640, "unknown_1411ca640", fn_1411ca640 as *const (), Throws::Discard), // sub_1411ca640
    Replacement::draft_throws(0x1411ca710, "unknown_1411ca710", fn_1411ca710 as *const (), Throws::Discard), // sub_1411ca710
    Replacement::draft_throws(0x1411ca7d0, "unknown_1411ca7d0", fn_1411ca7d0 as *const (), Throws::Discard), // sub_1411ca7d0
    Replacement::draft_throws(0x1411ca860, "unknown_1411ca860", fn_1411ca860 as *const (), Throws::Caught), // sub_1411ca860
    Replacement::draft(0x1411ca9f0, "unknown_1411ca9f0", fn_1411ca9f0 as *const ()), // sub_1411ca9f0
    Replacement::draft_throws(0x1411caaa0, "unknown_1411caaa0", fn_1411caaa0 as *const (), Throws::Caught), // sub_1411caaa0
    Replacement::draft(0x1411cac30, "unknown_1411cac30", fn_1411cac30 as *const ()), // sub_1411cac30
    Replacement::draft(0x1411cacd0, "unknown_1411cacd0", fn_1411cacd0 as *const ()), // sub_1411cacd0
    Replacement::draft(0x1411cad70, "unknown_1411cad70", fn_1411cad70 as *const ()), // sub_1411cad70
    Replacement::draft(0x1411cad90, "unknown_1411cad90", fn_1411cad90 as *const ()), // sub_1411cad90
    Replacement::draft(0x1411caea0, "unknown_1411caea0", fn_1411caea0 as *const ()), // sub_1411caea0
    Replacement::draft(0x1411caf00, "unknown_1411caf00", fn_1411caf00 as *const ()), // sub_1411caf00
    Replacement::draft(0x1411caf60, "unknown_1411caf60", fn_1411caf60 as *const ()), // sub_1411caf60
    Replacement::draft(0x1411cafc0, "unknown_1411cafc0", fn_1411cafc0 as *const ()), // sub_1411cafc0
    Replacement::draft(0x1411cb020, "unknown_1411cb020", fn_1411cb020 as *const ()), // sub_1411cb020
    Replacement::draft(0x1411cb080, "unknown_1411cb080", fn_1411cb080 as *const ()), // sub_1411cb080
    Replacement::draft(0x1411cb0f0, "unknown_1411cb0f0", fn_1411cb0f0 as *const ()), // sub_1411cb0f0
    Replacement::draft(0x1411cb160, "unknown_1411cb160", fn_1411cb160 as *const ()), // sub_1411cb160
    Replacement::draft(0x1411cb1d0, "unknown_1411cb1d0", fn_1411cb1d0 as *const ()), // sub_1411cb1d0
    Replacement::draft(0x1411cb240, "unknown_1411cb240", fn_1411cb240 as *const ()), // sub_1411cb240
    Replacement::draft(0x1411cb2a0, "unknown_1411cb2a0", fn_1411cb2a0 as *const ()), // sub_1411cb2a0
    Replacement::draft(0x1411cb300, "unknown_1411cb300", fn_1411cb300 as *const ()), // sub_1411cb300
    Replacement::draft(0x1411cb4a0, "unknown_1411cb4a0", fn_1411cb4a0 as *const ()), // sub_1411cb4a0
    Replacement::draft(0x1411cb640, "unknown_1411cb640", fn_1411cb640 as *const ()), // sub_1411cb640
    Replacement::draft(0x1411cb660, "unknown_1411cb660", fn_1411cb660 as *const ()), // sub_1411cb660
    Replacement::draft(0x1411cb6f0, "unknown_1411cb6f0", fn_1411cb6f0 as *const ()), // sub_1411cb6f0
    Replacement::draft(0x1411cb710, "unknown_1411cb710", fn_1411cb710 as *const ()), // sub_1411cb710
    Replacement::draft(0x1411cb770, "unknown_1411cb770", fn_1411cb770 as *const ()), // sub_1411cb770
    Replacement::draft(0x1411cb790, "unknown_1411cb790", fn_1411cb790 as *const ()), // sub_1411cb790
    Replacement::draft(0x1411cb7f0, "unknown_1411cb7f0", fn_1411cb7f0 as *const ()), // sub_1411cb7f0
    Replacement::draft(0x1411cb810, "unknown_1411cb810", fn_1411cb810 as *const ()), // sub_1411cb810
    Replacement::draft(0x1411cb870, "unknown_1411cb870", fn_1411cb870 as *const ()), // sub_1411cb870
    Replacement::draft(0x1411cb890, "unknown_1411cb890", fn_1411cb890 as *const ()), // sub_1411cb890
    Replacement::draft(0x1411cb8f0, "unknown_1411cb8f0", fn_1411cb8f0 as *const ()), // sub_1411cb8f0
    Replacement::draft(0x1411cb910, "unknown_1411cb910", fn_1411cb910 as *const ()), // sub_1411cb910
    Replacement::draft(0x1411cb970, "unknown_1411cb970", fn_1411cb970 as *const ()), // sub_1411cb970
    Replacement::draft(0x1411cb9e0, "unknown_1411cb9e0", fn_1411cb9e0 as *const ()), // sub_1411cb9e0
    Replacement::draft(0x1411cba50, "unknown_1411cba50", fn_1411cba50 as *const ()), // sub_1411cba50
    Replacement::draft(0x1411cbae0, "unknown_1411cbae0", fn_1411cbae0 as *const ()), // sub_1411cbae0
    Replacement::draft(0x1411cbc10, "unknown_1411cbc10", fn_1411cbc10 as *const ()), // sub_1411cbc10
    Replacement::draft(0x1411cbc50, "unknown_1411cbc50", fn_1411cbc50 as *const ()), // sub_1411cbc50
    Replacement::draft(0x1411cbc90, "unknown_1411cbc90", fn_1411cbc90 as *const ()), // sub_1411cbc90
    Replacement::draft(0x1411cbcd0, "unknown_1411cbcd0", fn_1411cbcd0 as *const ()), // sub_1411cbcd0
    Replacement::draft(0x1411cbd10, "unknown_1411cbd10", fn_1411cbd10 as *const ()), // sub_1411cbd10
    Replacement::draft(0x1411cbd50, "unknown_1411cbd50", fn_1411cbd50 as *const ()), // sub_1411cbd50
    Replacement::draft(0x1411cbdc0, "unknown_1411cbdc0", fn_1411cbdc0 as *const ()), // sub_1411cbdc0
    Replacement::draft(0x1411cbe30, "unknown_1411cbe30", fn_1411cbe30 as *const ()), // sub_1411cbe30
    Replacement::draft(0x1411cbfa0, "unknown_1411cbfa0", fn_1411cbfa0 as *const ()), // sub_1411cbfa0
    Replacement::draft(0x1411cc020, "unknown_1411cc020", fn_1411cc020 as *const ()), // sub_1411cc020
    Replacement::draft_throws(0x1411cc0b0, "unknown_1411cc0b0", fn_1411cc0b0 as *const (), Throws::Discard), // sub_1411cc0b0
    Replacement::draft_throws(0x1411cc0d0, "unknown_1411cc0d0", fn_1411cc0d0 as *const (), Throws::Discard), // sub_1411cc0d0
    Replacement::draft_throws(0x1411cc140, "unknown_1411cc140", fn_1411cc140 as *const (), Throws::Discard), // sub_1411cc140
    Replacement::draft(0x1411cc1b0, "unknown_1411cc1b0", fn_1411cc1b0 as *const ()), // sub_1411cc1b0
    Replacement::draft(0x1411cc250, "unknown_1411cc250", fn_1411cc250 as *const ()), // sub_1411cc250
    Replacement::draft(0x1411cc3d0, "unknown_1411cc3d0", fn_1411cc3d0 as *const ()), // sub_1411cc3d0
    Replacement::draft(0x1411cc540, "unknown_1411cc540", tpl_10188::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1410c0a40> as *const ()), // sub_1411cc540
    Replacement::draft(0x1411cc6b0, "unknown_1411cc6b0", fn_1411cc6b0 as *const ()), // sub_1411cc6b0
    Replacement::draft(0x1411cc860, "unknown_1411cc860", tpl_22001::<0x143857968, 0x143857980, 0x143857990, 0x143857978, 0x1411cc9e0, 0x1411cccc0> as *const ()), // sub_1411cc860
    Replacement::draft(0x1411cc9e0, "unknown_1411cc9e0", tpl_80366::<0x143857968, 0x143857980, 0x1434286b0, 0x143428770, 0x143857970, 0x143857978, 0x143857990> as *const ()), // sub_1411cc9e0
    Replacement::draft(0x1411cccc0, "unknown_1411cccc0", tpl_80367::<0x143857968, 0x143857970, 0x143857980, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x143857978, 0x143857990, 0x1425371a0, 0x142533200> as *const ()), // sub_1411cccc0
    Replacement::draft(0x1411cd1e0, "coreshared_1411cd1e0", fn_1411cd1e0 as *const ()), // coreshared::EQSQueryResource::coreshared::EQSQueryResource::vfunc_0
    Replacement::draft(0x1411cd220, "coreshared_1411cd220", fn_1411cd220 as *const ()), // sub_1411cd220
    Replacement::draft_throws(0x1411cd280, "coreshared_1411cd280", fn_1411cd280 as *const (), Throws::Caught), // coreshared::FiniteStateMachineResource::coreshared::FiniteStateMachineResource::vfunc_34
    Replacement::draft(0x1411cd4a0, "coreshared_1411cd4a0", fn_1411cd4a0 as *const ()), // coreshared::EQSQueryResource::coreshared::EQSQueryResource::vfunc_13
    Replacement::draft(0x1411cd620, "unknown_1411cd620", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433da738, 0x8> as *const ()), // sub_1411cd620
    Replacement::draft(0x1411cd6a0, "unknown_1411cd6a0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433da738, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411cd6a0
    Replacement::draft_throws(0x1411cd740, "unknown_1411cd740", fn_1411cd740 as *const (), Throws::Caught), // sub_1411cd740
    Replacement::draft_throws(0x1411cd7d0, "unknown_1411cd7d0", tpl_1908::<0x1438815c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411cd7d0
    Replacement::draft(0x1411cd850, "unknown_1411cd850", tpl_22006::<0xc8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x1433da9a8, 0xb8> as *const ()), // sub_1411cd850
    Replacement::draft(0x1411cd900, "unknown_1411cd900", tpl_80371::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433da9a8, 0x14183acc2, 0x142428d70, 0xb8> as *const ()), // sub_1411cd900
    Replacement::draft(0x1411cd9d0, "unknown_1411cd9d0", tpl_6135::<0x1438815b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1411cd9d0
    Replacement::draft_throws(0x1411cda70, "unknown_1411cda70", tpl_6136::<0x1438815b4, 0x1438815b8, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1411cda70
    Replacement::draft_throws(0x1411cdaf0, "unknown_1411cdaf0", tpl_6136::<0x1438815b4, 0x1438815b8, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1411cdaf0
    Replacement::draft_throws(0x1411cdb70, "unknown_1411cdb70", fn_1411cdb70 as *const (), Throws::Caught), // sub_1411cdb70
    Replacement::draft(0x1411cdc00, "unknown_1411cdc00", tpl_6137::<0x1438815b4, 0x1438815b8, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1411cdc00
    Replacement::draft_throws(0x1411cdc80, "unknown_1411cdc80", tpl_6136::<0x1438815b4, 0x1438815b8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1411cdc80
    Replacement::draft_throws(0x1411cdd00, "unknown_1411cdd00", tpl_1905::<0x1438815b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1411cdd00
    Replacement::draft(0x1411cddb0, "unknown_1411cddb0", fn_1411cddb0 as *const ()), // sub_1411cddb0
    Replacement::draft(0x1411cde10, "std_1411cde10", tpl_1::<0x143892030> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::FiniteStateMachineData>::blockingLoad::<lambda_1>,void
    Replacement::draft(0x1411cde30, "std_1411cde30", tpl_550::<0x1433da700, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::FiniteStateMachineData>::blockingLoad::<lambda_1>,void
    Replacement::draft(0x1411cde50, "std_1411cde50", tpl_1::<0x1438920b0> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::FiniteStateMachineData>::requestLoad::<lambda_1>,void,
    Replacement::draft(0x1411cde60, "std_1411cde60", tpl_550::<0x1433daac8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::FiniteStateMachineData>::requestLoad::<lambda_1>,void,
    Replacement::draft_throws(0x1411cdee0, "coreshared_1411cdee0", fn_1411cdee0 as *const (), Throws::Caught), // coreshared::EQSQueryResource::coreshared::EQSQueryResource::vfunc_34
    Replacement::draft(0x1411ce170, "unknown_1411ce170", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433db010, 0x8> as *const ()), // sub_1411ce170
    Replacement::draft(0x1411ce1f0, "unknown_1411ce1f0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433db010, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411ce1f0
    Replacement::draft_throws(0x1411ce290, "unknown_1411ce290", fn_1411ce290 as *const (), Throws::Caught), // sub_1411ce290
    Replacement::draft_throws(0x1411ce320, "unknown_1411ce320", tpl_1908::<0x1438815e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411ce320
    Replacement::draft(0x1411ce3a0, "unknown_1411ce3a0", tpl_22006::<0xc8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x1433dae80, 0xb8> as *const ()), // sub_1411ce3a0
    Replacement::draft(0x1411ce450, "unknown_1411ce450", tpl_80371::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433dae80, 0x14183acc2, 0x142428d70, 0xb8> as *const ()), // sub_1411ce450
    Replacement::draft(0x1411ce520, "unknown_1411ce520", tpl_6135::<0x1438815d0, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1411ce520
    Replacement::draft_throws(0x1411ce5c0, "unknown_1411ce5c0", tpl_6136::<0x1438815e8, 0x1438815d0, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1411ce5c0
    Replacement::draft_throws(0x1411ce640, "unknown_1411ce640", tpl_6136::<0x1438815e8, 0x1438815d0, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1411ce640
    Replacement::draft_throws(0x1411ce6c0, "unknown_1411ce6c0", fn_1411ce6c0 as *const (), Throws::Caught), // sub_1411ce6c0
    Replacement::draft(0x1411ce750, "unknown_1411ce750", tpl_6137::<0x1438815e8, 0x1438815d0, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1411ce750
    Replacement::draft_throws(0x1411ce7d0, "unknown_1411ce7d0", tpl_6136::<0x1438815e8, 0x1438815d0, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1411ce7d0
    Replacement::draft_throws(0x1411ce850, "unknown_1411ce850", tpl_1905::<0x1438815d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1411ce850
    Replacement::draft(0x1411ce900, "std_1411ce900", tpl_1::<0x1438921a0> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::EQSQueryData>::blockingLoad::<lambda_1>,void,r::BaseFi
    Replacement::draft(0x1411ce910, "std_1411ce910", tpl_550::<0x1433dafd8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::EQSQueryData>::blockingLoad::<lambda_1>,void,r::BaseFi
    Replacement::draft(0x1411ce930, "std_1411ce930", tpl_1::<0x143892220> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::EQSQueryData>::requestLoad::<lambda_1>,void,r::BaseFil
    Replacement::draft(0x1411ce940, "std_1411ce940", tpl_550::<0x1433dafa0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::EQSQueryData>::requestLoad::<lambda_1>,void,r::BaseFil
    Replacement::draft(0x1411ce960, "unknown_1411ce960", fn_1411ce960 as *const ()), // sub_1411ce960
    Replacement::draft_throws(0x1411ceb00, "coreshared_1411ceb00", fn_1411ceb00 as *const (), Throws::Caught), // coreshared::BehaviorTreeResource::coreshared::BehaviorTreeResource::vfunc_34
    Replacement::draft(0x1411ced90, "unknown_1411ced90", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433db1c0, 0x8> as *const ()), // sub_1411ced90
    Replacement::draft(0x1411cee10, "unknown_1411cee10", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433db1c0, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411cee10
    Replacement::draft_throws(0x1411ceeb0, "unknown_1411ceeb0", fn_1411ceeb0 as *const (), Throws::Caught), // sub_1411ceeb0
    Replacement::draft_throws(0x1411cef40, "unknown_1411cef40", tpl_1908::<0x143881600, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411cef40
    Replacement::draft(0x1411cefc0, "unknown_1411cefc0", tpl_22006::<0xc8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x1433db4a0, 0xb8> as *const ()), // sub_1411cefc0
    Replacement::draft(0x1411cf070, "unknown_1411cf070", tpl_80371::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433db4a0, 0x14183acc2, 0x142428d70, 0xb8> as *const ()), // sub_1411cf070
    Replacement::draft(0x1411cf140, "unknown_1411cf140", tpl_6135::<0x1438815f8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1411cf140
    Replacement::draft_throws(0x1411cf1e0, "unknown_1411cf1e0", tpl_6136::<0x1438815ec, 0x1438815f8, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1411cf1e0
    Replacement::draft_throws(0x1411cf260, "unknown_1411cf260", tpl_6136::<0x1438815ec, 0x1438815f8, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1411cf260
    Replacement::draft_throws(0x1411cf2e0, "unknown_1411cf2e0", fn_1411cf2e0 as *const (), Throws::Caught), // sub_1411cf2e0
    Replacement::draft(0x1411cf370, "unknown_1411cf370", tpl_6137::<0x1438815ec, 0x1438815f8, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1411cf370
    Replacement::draft_throws(0x1411cf3f0, "unknown_1411cf3f0", tpl_6136::<0x1438815ec, 0x1438815f8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1411cf3f0
    Replacement::draft_throws(0x1411cf470, "unknown_1411cf470", tpl_1905::<0x1438815f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1411cf470
    Replacement::draft(0x1411cf520, "std_1411cf520", tpl_1::<0x143892350> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::BehaviorTreeData>::blockingLoad::<lambda_1>,void,r::Ba
    Replacement::draft(0x1411cf530, "std_1411cf530", tpl_550::<0x1433db468, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::BehaviorTreeData>::blockingLoad::<lambda_1>,void,r::Ba
    Replacement::draft(0x1411cf550, "std_1411cf550", tpl_1::<0x1438922a0> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::BehaviorTreeData>::requestLoad::<lambda_1>,void,r::Bas
    Replacement::draft(0x1411cf560, "std_1411cf560", tpl_550::<0x1433db430, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::BonsaiGraphResource<content::BehaviorTreeData>::requestLoad::<lambda_1>,void,r::Bas
    Replacement::draft(0x1411cf5e0, "unknown_1411cf5e0", fn_1411cf5e0 as *const ()), // sub_1411cf5e0
    Replacement::draft_throws(0x1411cf6e0, "unknown_1411cf6e0", tpl_22010::<0x143881618, 0x143881620, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x369, 0x4, 0x8, 0x1401e9e70> as *const (), Throws::Discard), // sub_1411cf6e0
    Replacement::draft(0x1411cf7c0, "unknown_1411cf7c0", tpl_22011::<0x143881620, 0x14243b0b0, 0x338, 0x48, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1411cf7c0
    Replacement::draft_throws(0x1411cf850, "unknown_1411cf850", fn_1411cf850 as *const (), Throws::Discard), // sub_1411cf850
    Replacement::draft_throws(0x1411cf940, "unknown_1411cf940", fn_1411cf940 as *const (), Throws::Caught), // sub_1411cf940
    Replacement::draft_throws(0x1411cfa70, "unknown_1411cfa70", fn_1411cfa70 as *const (), Throws::Caught), // sub_1411cfa70
    Replacement::draft_throws(0x1411cfba0, "unknown_1411cfba0", fn_1411cfba0 as *const (), Throws::Caught), // sub_1411cfba0
    Replacement::draft(0x1411cfcd0, "unknown_1411cfcd0", tpl_22014::<0x143881620, 0x14243b0b0, 0x338, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1411cfcd0
    Replacement::draft(0x1411cfd60, "unknown_1411cfd60", tpl_71920::<0x143881618, 0x143881620, 0x1401e9970, 0x338, 0xe8, 0x4, 0x8, 0x1401e9860> as *const ()), // sub_1411cfd60
    Replacement::draft(0x1411cfdf0, "unknown_1411cfdf0", tpl_22011::<0x143881620, 0x14243b0b0, 0x338, 0xe8, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1411cfdf0
    Replacement::draft_throws(0x1411cfe90, "unknown_1411cfe90", fn_1411cfe90 as *const (), Throws::Discard), // sub_1411cfe90
    Replacement::draft_throws(0x1411cff30, "unknown_1411cff30", fn_1411cff30 as *const (), Throws::Discard), // sub_1411cff30
    Replacement::draft(0x1411d0050, "unknown_1411d0050", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433db940, 0x8> as *const ()), // sub_1411d0050
    Replacement::draft(0x1411d00d0, "unknown_1411d00d0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433db940, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1411d00d0
    Replacement::draft_throws(0x1411d0170, "unknown_1411d0170", fn_1411d0170 as *const (), Throws::Caught), // sub_1411d0170
    Replacement::draft_throws(0x1411d0200, "unknown_1411d0200", tpl_1908::<0x143881628, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1411d0200
    Replacement::draft(0x1411d0280, "unknown_1411d0280", fn_1411d0280 as *const ()), // sub_1411d0280
    Replacement::draft(0x1411d0350, "unknown_1411d0350", fn_1411d0350 as *const ()), // sub_1411d0350
    Replacement::draft(0x1411d0450, "unknown_1411d0450", tpl_9615::<0x143881610, 0x14243b0b0, 0x143881f80, 0x8, 0x143881620, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xbc> as *const ()), // sub_1411d0450
    Replacement::draft(0x1411d0530, "unknown_1411d0530", tpl_13342::<0x143881610, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xbc, 0x1411cf5e0> as *const ()), // sub_1411d0530
    Replacement::draft_throws(0x1411d05e0, "unknown_1411d05e0", tpl_9846::<0x14388161c, 0x143881610, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70, 0xbc, 0x1411d0da0> as *const (), Throws::Discard), // sub_1411d05e0
    Replacement::draft(0x1411d0670, "unknown_1411d0670", tpl_13342::<0x143881610, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xbc, 0x1411cf7c0> as *const ()), // sub_1411d0670
    Replacement::draft_throws(0x1411d0720, "unknown_1411d0720", tpl_9846::<0x14388161c, 0x143881610, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20, 0xbc, 0x1411d1000> as *const (), Throws::Discard), // sub_1411d0720
    Replacement::draft_throws(0x1411d07b0, "unknown_1411d07b0", fn_1411d07b0 as *const (), Throws::Caught), // sub_1411d07b0
    Replacement::draft_throws(0x1411d0870, "unknown_1411d0870", fn_1411d0870 as *const (), Throws::Caught), // sub_1411d0870
    Replacement::draft_throws(0x1411d0930, "unknown_1411d0930", fn_1411d0930 as *const (), Throws::Caught), // sub_1411d0930
    Replacement::draft(0x1411d09f0, "unknown_1411d09f0", tpl_13344::<0x143881610, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xbc, 0x1411cfcd0> as *const ()), // sub_1411d09f0
    Replacement::draft(0x1411d0ab0, "unknown_1411d0ab0", tpl_9842::<0x14388161c, 0x143881610, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860, 0xbc, 0x1411cfd60> as *const ()), // sub_1411d0ab0
    Replacement::draft(0x1411d0b40, "unknown_1411d0b40", tpl_13342::<0x143881610, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xbc, 0x1411cfdf0> as *const ()), // sub_1411d0b40
    Replacement::draft_throws(0x1411d0bf0, "unknown_1411d0bf0", tpl_9843::<0x14388161c, 0x143881610, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590, 0xbc, 0x1411cfe90> as *const (), Throws::Discard), // sub_1411d0bf0
    Replacement::draft_throws(0x1411d0c80, "unknown_1411d0c80", fn_1411d0c80 as *const (), Throws::Caught), // sub_1411d0c80
    Replacement::draft_throws(0x1411d0da0, "unknown_1411d0da0", fn_1411d0da0 as *const (), Throws::Caught), // sub_1411d0da0
    Replacement::draft_throws(0x1411d1000, "unknown_1411d1000", fn_1411d1000 as *const (), Throws::Caught), // sub_1411d1000
    Replacement::draft_throws(0x1411d1260, "unknown_1411d1260", fn_1411d1260 as *const (), Throws::Caught), // sub_1411d1260
    Replacement::draft_throws(0x1411d14c0, "unknown_1411d14c0", fn_1411d14c0 as *const (), Throws::Caught), // sub_1411d14c0
    Replacement::draft_throws(0x1411d1720, "unknown_1411d1720", fn_1411d1720 as *const (), Throws::Caught), // sub_1411d1720
    Replacement::draft_throws(0x1411d1980, "unknown_1411d1980", fn_1411d1980 as *const (), Throws::Caught), // sub_1411d1980
    Replacement::draft_throws(0x1411d2020, "unknown_1411d2020", fn_1411d2020 as *const (), Throws::Caught), // sub_1411d2020
    Replacement::draft_throws(0x1411d2680, "unknown_1411d2680", fn_1411d2680 as *const (), Throws::Caught), // sub_1411d2680
    Replacement::draft(0x1411d2ac0, "unknown_1411d2ac0", tpl_170::<0x1411d2b40> as *const ()), // sub_1411d2ac0
    Replacement::draft(0x1411d2ad0, "unknown_1411d2ad0", fn_1411d2ad0 as *const ()), // sub_1411d2ad0
    Replacement::draft(0x1411d2b20, "unknown_1411d2b20", fn_1411d2b20 as *const ()), // sub_1411d2b20
    Replacement::draft(0x1411d2b40, "unknown_1411d2b40", fn_1411d2b40 as *const ()), // sub_1411d2b40
    Replacement::draft(0x1411d2c10, "unknown_1411d2c10", fn_1411d2c10 as *const ()), // sub_1411d2c10
    Replacement::draft(0x1411d2d80, "unknown_1411d2d80", fn_1411d2d80 as *const ()), // sub_1411d2d80
    Replacement::draft_throws(0x1411d2db0, "unknown_1411d2db0", tpl_22028::<0x14206b020, 0x142442de0> as *const (), Throws::Discard), // sub_1411d2db0
    Replacement::draft(0x1411d2de0, "d3d_1411d2de0", fn_1411d2de0 as *const ()), // sub_1411d2de0
    Replacement::draft(0x1411d2e20, "d3d_1411d2e20", tpl_22030::<0x14340ed48, 0x8, 0x1433dc1e0> as *const ()), // sub_1411d2e20
    Replacement::draft(0x1411d2e40, "d3d_1411d2e40", tpl_1691::<0x24> as *const ()), // sub_1411d2e40
    Replacement::draft_throws(0x1411d2e50, "d3d_1411d2e50", tpl_13418::<0x1411d3020> as *const (), Throws::Discard), // sub_1411d2e50
    Replacement::draft(0x1411d2e60, "d3d_1411d2e60", tpl_22031::<0x8> as *const ()), // sub_1411d2e60
    Replacement::draft(0x1411d2e70, "d3d_1411d2e70", fn_1411d2e70 as *const ()), // sub_1411d2e70
    Replacement::draft_throws(0x1411d2ea0, "d3d_1411d2ea0", fn_1411d2ea0 as *const (), Throws::Discard), // sub_1411d2ea0
    Replacement::draft_throws(0x1411d2f60, "d3d_1411d2f60", tpl_22033::<0x14206b020, 0x142442de0> as *const (), Throws::Discard), // sub_1411d2f60
    Replacement::draft_throws(0x1411d2fb0, "d3d_1411d2fb0", fn_1411d2fb0 as *const (), Throws::Caught), // sub_1411d2fb0
    Replacement::draft_throws(0x1411d3020, "d3d_1411d3020", fn_1411d3020 as *const (), Throws::Caught), // sub_1411d3020
    Replacement::draft_throws(0x1411d3100, "d3d_1411d3100", tpl_22036::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1411d3020> as *const (), Throws::Discard), // sub_1411d3100
    Replacement::draft_throws(0x1411d3270, "d3d_1411d3270", fn_1411d3270 as *const (), Throws::Caught), // sub_1411d3270
    Replacement::draft_throws(0x1411d32d0, "d3d_1411d32d0", fn_1411d32d0 as *const (), Throws::Discard), // sub_1411d32d0
    Replacement::draft(0x1411d3470, "d3d_1411d3470", fn_1411d3470 as *const ()), // sub_1411d3470
    Replacement::draft_throws(0x1411d35d0, "d3d_1411d35d0", fn_1411d35d0 as *const (), Throws::Discard), // sub_1411d35d0
    Replacement::draft(0x1411d3820, "d3d_1411d3820", fn_1411d3820 as *const ()), // sub_1411d3820
    Replacement::draft(0x1411d3aa0, "d3d_1411d3aa0", tpl_170::<0x1411d3de0> as *const ()), // sub_1411d3aa0
    Replacement::draft(0x1411d3ab0, "d3d_1411d3ab0", tpl_365::<0x14243b540> as *const ()), // sub_1411d3ab0
    Replacement::draft(0x1411d3ac0, "d3d_1411d3ac0", fn_1411d3ac0 as *const ()), // sub_1411d3ac0
    Replacement::draft(0x1411d3c20, "d3d_1411d3c20", fn_1411d3c20 as *const ()), // sub_1411d3c20
    Replacement::draft(0x1411d3d00, "d3d_1411d3d00", fn_1411d3d00 as *const ()), // sub_1411d3d00
    Replacement::draft(0x1411d3de0, "d3d_1411d3de0", fn_1411d3de0 as *const ()), // sub_1411d3de0
    Replacement::draft(0x1411d3eb0, "d3d_1411d3eb0", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1411d3de0> as *const ()), // sub_1411d3eb0
    Replacement::draft(0x1411d4020, "d3d_1411d4020", fn_1411d4020 as *const ()), // sub_1411d4020
    Replacement::draft(0x1411d40c0, "d3d_1411d40c0", fn_1411d40c0 as *const ()), // sub_1411d40c0
    Replacement::draft(0x1411d4150, "d3d_1411d4150", fn_1411d4150 as *const ()), // sub_1411d4150
    Replacement::draft(0x1411d4190, "d3d_1411d4190", tpl_424::<0x142442de0> as *const ()), // sub_1411d4190
    Replacement::draft(0x1411d41b0, "d3d_1411d41b0", fn_1411d41b0 as *const ()), // sub_1411d41b0
    Replacement::draft(0x1411d4240, "d3d_1411d4240", fn_1411d4240 as *const ()), // sub_1411d4240
    Replacement::draft_throws(0x1411d4270, "d3d_1411d4270", fn_1411d4270 as *const (), Throws::Discard), // sub_1411d4270
    Replacement::draft_throws(0x1411d4380, "d3d_1411d4380", fn_1411d4380 as *const (), Throws::Discard), // sub_1411d4380
    Replacement::draft(0x1411d4410, "d3d_1411d4410", fn_1411d4410 as *const ()), // sub_1411d4410
    Replacement::draft(0x1411d4450, "d3d_1411d4450", fn_1411d4450 as *const ()), // sub_1411d4450
    Replacement::draft(0x1411d44d0, "d3d_1411d44d0", fn_1411d44d0 as *const ()), // sub_1411d44d0
    Replacement::draft(0x1411d4530, "d3d_1411d4530", fn_1411d4530 as *const ()), // sub_1411d4530
    Replacement::draft(0x1411d45c0, "d3d_1411d45c0", fn_1411d45c0 as *const ()), // sub_1411d45c0
    Replacement::draft(0x1411d4900, "d3d_1411d4900", fn_1411d4900 as *const ()), // sub_1411d4900
    Replacement::draft(0x1411d4be0, "d3d_1411d4be0", fn_1411d4be0 as *const ()), // sub_1411d4be0
    Replacement::draft(0x1411d4cd0, "d3d_1411d4cd0", fn_1411d4cd0 as *const ()), // sub_1411d4cd0
    Replacement::draft(0x1411d4d80, "d3d_1411d4d80", fn_1411d4d80 as *const ()), // sub_1411d4d80
    Replacement::draft_throws(0x1411d4e00, "d3d_1411d4e00", fn_1411d4e00 as *const (), Throws::Discard), // sub_1411d4e00
    Replacement::draft_throws(0x1411d4eb0, "d3d_1411d4eb0", fn_1411d4eb0 as *const (), Throws::Discard), // sub_1411d4eb0
    Replacement::draft(0x1411d5000, "d3d_1411d5000", tpl_80418::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1425371a0, 0x142533200> as *const ()), // sub_1411d5000
    Replacement::draft(0x1411d5220, "d3d_1411d5220", tpl_22063::<0x142056db0, 0x141834d30> as *const ()), // d3d::FrameBeginHandler::d3d::FrameBeginHandler::vfunc_0
    Replacement::draft(0x1411d5260, "d3d_1411d5260", tpl_22063::<0x142056af0, 0x141834d30> as *const ()), // d3d::FrameEndHandler::d3d::FrameEndHandler::vfunc_0
    Replacement::draft(0x1411d52a0, "d3d_1411d52a0", fn_1411d52a0 as *const ()), // d3d::FrameDrawnHandler::d3d::FrameDrawnHandler::vfunc_0
    Replacement::draft(0x1411d53f0, "d3d_1411d53f0", tpl_22065::<0x143a63204, 0x143a63200, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143a631f8, 0x14183acc8, 0x1401de830> as *const ()), // sub_1411d53f0
    Replacement::draft(0x1411d5540, "d3d_1411d5540", tpl_22065::<0x143a63224, 0x143a63220, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143a63218, 0x14183acc8, 0x1401de830> as *const ()), // sub_1411d5540
    Replacement::draft(0x1411d5690, "d3d_1411d5690", fn_1411d5690 as *const ()), // sub_1411d5690
    Replacement::draft(0x1411d57e0, "d3d_1411d57e0", tpl_22067::<0x142057410, 0x141834d30> as *const ()), // d3d::FrameEndReleaser::d3d::FrameEndReleaser::vfunc_0
    Replacement::draft(0x1411d5820, "d3d_1411d5820", tpl_22067::<0x142057130, 0x141834d30> as *const ()), // d3d::FrameDrawnReleaser::d3d::FrameDrawnReleaser::vfunc_0
    Replacement::draft(0x1411d5860, "d3d_1411d5860", tpl_22065::<0x143a63234, 0x143a63230, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143a63228, 0x14183acc8, 0x1401de830> as *const ()), // sub_1411d5860
    Replacement::draft(0x1411d59b0, "d3d_1411d59b0", tpl_22065::<0x143a63244, 0x143a63240, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143a63238, 0x14183acc8, 0x1401de830> as *const ()), // sub_1411d59b0
    Replacement::draft(0x1411d5bf0, "d3d_1411d5bf0", fn_1411d5bf0 as *const ()), // sub_1411d5bf0
    Replacement::draft(0x1411d5c10, "d3d_1411d5c10", fn_1411d5c10 as *const ()), // sub_1411d5c10
    Replacement::draft(0x1411d5cd0, "global_1411d5cd0", fn_1411d5cd0 as *const ()), // FID_conflict:~vector<GSISymbolEntry,std::allocator<GSISymbolEntry>_>
    Replacement::draft(0x1411d5d50, "d3d_1411d5d50", fn_1411d5d50 as *const ()), // sub_1411d5d50
    Replacement::draft(0x1411d5e10, "d3d_1411d5e10", fn_1411d5e10 as *const ()), // sub_1411d5e10
    Replacement::draft(0x1411d5e80, "d3d_1411d5e80", fn_1411d5e80 as *const ()), // sub_1411d5e80
    Replacement::draft(0x1411d5ee0, "d3d_1411d5ee0", fn_1411d5ee0 as *const ()), // sub_1411d5ee0
    Replacement::draft(0x1411d5f40, "d3d_1411d5f40", fn_1411d5f40 as *const ()), // sub_1411d5f40
    Replacement::draft_throws(0x1411d5fd0, "d3d_1411d5fd0", fn_1411d5fd0 as *const (), Throws::Caught), // sub_1411d5fd0
    Replacement::draft(0x1411d61c0, "d3d_1411d61c0", fn_1411d61c0 as *const ()), // sub_1411d61c0
    Replacement::draft_throws(0x1411d6220, "d3d_1411d6220", fn_1411d6220 as *const (), Throws::Discard), // sub_1411d6220
    Replacement::draft_throws(0x1411d6380, "d3d_1411d6380", fn_1411d6380 as *const (), Throws::Discard), // sub_1411d6380
    Replacement::draft_throws(0x1411d64d0, "d3d_1411d64d0", fn_1411d64d0 as *const (), Throws::Discard), // sub_1411d64d0
    Replacement::draft(0x1411d6660, "d3d_1411d6660", fn_1411d6660 as *const ()), // sub_1411d6660
    Replacement::draft_throws(0x1411d66a0, "d3d_1411d66a0", fn_1411d66a0 as *const (), Throws::Discard), // sub_1411d66a0
    Replacement::draft_throws(0x1411d6890, "d3d_1411d6890", fn_1411d6890 as *const (), Throws::Discard), // sub_1411d6890
    Replacement::draft(0x1411d69d0, "d3d_1411d69d0", fn_1411d69d0 as *const ()), // sub_1411d69d0
    Replacement::draft(0x1411d6a10, "d3d_1411d6a10", fn_1411d6a10 as *const ()), // sub_1411d6a10
    Replacement::draft(0x1411d6a70, "d3d_1411d6a70", fn_1411d6a70 as *const ()), // sub_1411d6a70
    Replacement::draft_throws(0x1411d6a90, "d3d_1411d6a90", fn_1411d6a90 as *const (), Throws::Caught), // sub_1411d6a90
    Replacement::draft_throws(0x1411d6b80, "d3d_1411d6b80", fn_1411d6b80 as *const (), Throws::Discard), // sub_1411d6b80
    Replacement::draft_throws(0x1411d6cb0, "d3d_1411d6cb0", fn_1411d6cb0 as *const (), Throws::Discard), // sub_1411d6cb0
    Replacement::draft_throws(0x1411d6e60, "d3d_1411d6e60", fn_1411d6e60 as *const (), Throws::Caught), // sub_1411d6e60
    Replacement::draft(0x1411d6ec0, "d3d_1411d6ec0", tpl_170::<0x1411d7950> as *const ()), // sub_1411d6ec0
    Replacement::draft(0x1411d6ed0, "d3d_1411d6ed0", fn_1411d6ed0 as *const ()), // sub_1411d6ed0
    Replacement::draft(0x1411d6f30, "d3d_1411d6f30", fn_1411d6f30 as *const ()), // sub_1411d6f30
    Replacement::draft(0x1411d6f90, "d3d_1411d6f90", fn_1411d6f90 as *const ()), // sub_1411d6f90
    Replacement::draft(0x1411d7000, "d3d_1411d7000", fn_1411d7000 as *const ()), // sub_1411d7000
    Replacement::draft(0x1411d7140, "d3d_1411d7140", fn_1411d7140 as *const ()), // sub_1411d7140
    Replacement::draft(0x1411d7440, "d3d_1411d7440", fn_1411d7440 as *const ()), // sub_1411d7440
    Replacement::draft(0x1411d7530, "d3d_1411d7530", fn_1411d7530 as *const ()), // sub_1411d7530
    Replacement::draft(0x1411d7610, "d3d_1411d7610", fn_1411d7610 as *const ()), // sub_1411d7610
    Replacement::draft(0x1411d76d0, "d3d_1411d76d0", fn_1411d76d0 as *const ()), // sub_1411d76d0
    Replacement::draft(0x1411d77a0, "d3d_1411d77a0", fn_1411d77a0 as *const ()), // sub_1411d77a0
    Replacement::draft(0x1411d7870, "d3d_1411d7870", fn_1411d7870 as *const ()), // sub_1411d7870
    Replacement::draft(0x1411d7940, "d3d_1411d7940", tpl_170::<0x1411d7a20> as *const ()), // sub_1411d7940
    Replacement::draft(0x1411d7950, "d3d_1411d7950", tpl_21289::<0x8, 0x1411d7cf0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1411d7950
    Replacement::draft(0x1411d7a20, "d3d_1411d7a20", fn_1411d7a20 as *const ()), // sub_1411d7a20
    Replacement::draft(0x1411d7af0, "d3d_1411d7af0", fn_1411d7af0 as *const ()), // sub_1411d7af0
    Replacement::draft(0x1411d7cf0, "d3d_1411d7cf0", fn_1411d7cf0 as *const ()), // sub_1411d7cf0
    Replacement::draft(0x1411d7da0, "d3d_1411d7da0", fn_1411d7da0 as *const ()), // sub_1411d7da0
    Replacement::draft(0x1411d7e20, "d3d_1411d7e20", fn_1411d7e20 as *const ()), // sub_1411d7e20
    Replacement::draft_throws(0x1411d7ec0, "d3d_1411d7ec0", tpl_22098::<0x14397fc10, 0x142645790, 0x1426455a8, 0x1411d85c0, 0x1403497a0, 0x142645598, 0x141173780> as *const (), Throws::Discard), // sub_1411d7ec0
    Replacement::draft(0x1411d8070, "d3d_1411d8070", tpl_22099::<0x14397fc10, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598> as *const ()), // d3d::PagedBufferAllocator<VertexBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<VertexBufferAllocator,d3
    Replacement::draft(0x1411d8150, "d3d_1411d8150", tpl_22100::<0x143a35e28, 0x143a35918, 0x1411d8240, 0x142057060, 0x14020bcb0> as *const ()), // sub_1411d8150
    Replacement::draft(0x1411d8240, "d3d_1411d8240", tpl_22101::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433dc800, 0x8, 0x20, 0x28> as *const ()), // sub_1411d8240
    Replacement::draft_throws(0x1411d82e0, "d3d_1411d82e0", fn_1411d82e0 as *const (), Throws::Discard), // sub_1411d82e0
    Replacement::draft(0x1411d84a0, "d3d_1411d84a0", tpl_22103::<0x1433dc800, 0x14397fc10, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598, 0x1401de830, 0x142057130, 0x141834d30> as *const ()), // d3d::PagedBufferAllocator<VertexBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<VertexBufferAllocator,d3
    Replacement::draft_throws(0x1411d85c0, "d3d_1411d85c0", fn_1411d85c0 as *const (), Throws::Discard), // sub_1411d85c0
    Replacement::draft(0x1411d86e0, "d3d_1411d86e0", fn_1411d86e0 as *const ()), // sub_1411d86e0
    Replacement::draft_throws(0x1411d8770, "d3d_1411d8770", fn_1411d8770 as *const (), Throws::Discard), // sub_1411d8770
    Replacement::draft(0x1411d8920, "d3d_1411d8920", tpl_22099::<0x14397fc18, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598> as *const ()), // d3d::PagedBufferAllocator<StagingBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<StagingBufferAllocator,
    Replacement::draft(0x1411d8a00, "d3d_1411d8a00", tpl_22107::<0x143a35ec8, 0x143a35918, 0x1411d8af0, 0x142057060, 0x14020bcb0> as *const ()), // sub_1411d8a00
    Replacement::draft(0x1411d8af0, "d3d_1411d8af0", tpl_22101::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433dc818, 0x8, 0x20, 0x28> as *const ()), // sub_1411d8af0
    Replacement::draft_throws(0x1411d8b90, "d3d_1411d8b90", fn_1411d8b90 as *const (), Throws::Discard), // sub_1411d8b90
    Replacement::draft(0x1411d8d50, "d3d_1411d8d50", tpl_22103::<0x1433dc818, 0x14397fc18, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598, 0x1401de830, 0x142057130, 0x141834d30> as *const ()), // d3d::PagedBufferAllocator<StagingBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<StagingBufferAllocator,
    Replacement::draft_throws(0x1411d8e70, "d3d_1411d8e70", fn_1411d8e70 as *const (), Throws::Discard), // sub_1411d8e70
    Replacement::draft_throws(0x1411d8f90, "d3d_1411d8f90", tpl_22098::<0x14397fc20, 0x142645790, 0x1426455a8, 0x1411d85c0, 0x1403497a0, 0x142645598, 0x141173780> as *const (), Throws::Discard), // sub_1411d8f90
    Replacement::draft(0x1411d9140, "d3d_1411d9140", tpl_22099::<0x14397fc20, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598> as *const ()), // d3d::PagedBufferAllocator<IndexBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<IndexBufferAllocator,d3d:
    Replacement::draft(0x1411d9220, "d3d_1411d9220", tpl_22107::<0x143a35f98, 0x143a35918, 0x1411d9310, 0x142057060, 0x14020bcb0> as *const ()), // sub_1411d9220
    Replacement::draft(0x1411d9310, "d3d_1411d9310", tpl_22101::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433dc830, 0x8, 0x20, 0x28> as *const ()), // sub_1411d9310
    Replacement::draft_throws(0x1411d93b0, "d3d_1411d93b0", fn_1411d93b0 as *const (), Throws::Discard), // sub_1411d93b0
    Replacement::draft(0x1411d9570, "d3d_1411d9570", tpl_22103::<0x1433dc830, 0x14397fc20, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598, 0x1401de830, 0x142057130, 0x141834d30> as *const ()), // d3d::PagedBufferAllocator<IndexBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<IndexBufferAllocator,d3d:
    Replacement::draft_throws(0x1411d9690, "d3d_1411d9690", fn_1411d9690 as *const (), Throws::Discard), // sub_1411d9690
    Replacement::draft(0x1411d9840, "d3d_1411d9840", tpl_22099::<0x14397fc28, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598> as *const ()), // d3d::PagedBufferAllocator<d3d::ConstantBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<d3d::ConstantBuff
    Replacement::draft(0x1411d9a10, "d3d_1411d9a10", tpl_22101::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433dc848, 0x8, 0x20, 0x28> as *const ()), // sub_1411d9a10
    Replacement::draft_throws(0x1411d9ab0, "d3d_1411d9ab0", fn_1411d9ab0 as *const (), Throws::Discard), // sub_1411d9ab0
    Replacement::draft(0x1411d9c70, "d3d_1411d9c70", tpl_22103::<0x1433dc848, 0x14397fc28, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598, 0x1401de830, 0x142057130, 0x141834d30> as *const ()), // d3d::PagedBufferAllocator<d3d::ConstantBufferAllocator,d3d::PageHeaderBase>::d3d::PagedBufferAllocator<d3d::ConstantBuff
    Replacement::draft_throws(0x1411d9d90, "d3d_1411d9d90", fn_1411d9d90 as *const (), Throws::Discard), // sub_1411d9d90
    Replacement::draft(0x1411d9eb0, "d3d_1411d9eb0", tpl_80471::<0x143a632e0, 0x143a632e4, 0x143a632d0, 0x1401dec00, 0x143a632d8, 0x1425371a0, 0x142533200, 0x143a632dc> as *const ()), // sub_1411d9eb0
    Replacement::draft_throws(0x1411da080, "d3d_1411da080", fn_1411da080 as *const (), Throws::Discard), // sub_1411da080
    Replacement::draft(0x1411da290, "d3d_1411da290", tpl_22099::<0x14397fc98, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598> as *const ()), // d3d::PagedBufferAllocator<d3d::DynamicBufferAllocator,d3d::DynamicBufferPageHeader>::d3d::PagedBufferAllocator<d3d::Dyna
    Replacement::draft(0x1411da370, "d3d_1411da370", tpl_22100::<0x143a35fe8, 0x143a35918, 0x1411da460, 0x142057060, 0x14020bcb0> as *const ()), // sub_1411da370
    Replacement::draft(0x1411da460, "d3d_1411da460", tpl_22101::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433dc860, 0x8, 0x20, 0x28> as *const ()), // sub_1411da460
    Replacement::draft(0x1411da500, "d3d_1411da500", tpl_22103::<0x1433dc860, 0x14397fc98, 0x142645790, 0x1426455a8, 0x1403497a0, 0x142645598, 0x1401de830, 0x142057130, 0x141834d30> as *const ()), // d3d::PagedBufferAllocator<d3d::DynamicBufferAllocator,d3d::DynamicBufferPageHeader>::d3d::PagedBufferAllocator<d3d::Dyna
    Replacement::draft(0x1411da620, "d3d_1411da620", fn_1411da620 as *const ()), // sub_1411da620
    Replacement::draft_throws(0x1411da6a0, "d3d_1411da6a0", fn_1411da6a0 as *const (), Throws::Discard), // sub_1411da6a0
    Replacement::draft(0x1411da8a0, "d3d_1411da8a0", fn_1411da8a0 as *const ()), // sub_1411da8a0
    Replacement::draft_throws(0x1411da950, "d3d_1411da950", fn_1411da950 as *const (), Throws::Discard), // sub_1411da950
    Replacement::draft_throws(0x1411daa40, "d3d_1411daa40", fn_1411daa40 as *const (), Throws::Discard), // sub_1411daa40
    Replacement::draft(0x1411dacd0, "d3d_1411dacd0", fn_1411dacd0 as *const ()), // d3d::VertexShader::d3d::VertexShader::vfunc_0
    Replacement::draft(0x1411dad70, "d3d_1411dad70", tpl_2367::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1411dad70
    Replacement::draft(0x1411dade0, "d3d_1411dade0", fn_1411dade0 as *const ()), // sub_1411dade0
    Replacement::draft(0x1411dae50, "d3d_1411dae50", fn_1411dae50 as *const ()), // sub_1411dae50
    Replacement::draft(0x1411db050, "d3d_1411db050", fn_1411db050 as *const ()), // sub_1411db050
    Replacement::draft(0x1411db140, "d3d_1411db140", tpl_22123::<0x1411db190, 0xfffffffffffffffa, 0x141834ee8, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_1411db140
    Replacement::draft(0x1411db190, "d3d_1411db190", fn_1411db190 as *const ()), // sub_1411db190
    Replacement::draft(0x1411db1c0, "d3d_1411db1c0", fn_1411db1c0 as *const ()), // sub_1411db1c0
    Replacement::draft(0x1411db210, "d3d_1411db210", fn_1411db210 as *const ()), // sub_1411db210
    Replacement::draft(0x1411db2d0, "d3d_1411db2d0", fn_1411db2d0 as *const ()), // sub_1411db2d0
    Replacement::draft(0x1411db600, "d3d_1411db600", fn_1411db600 as *const ()), // sub_1411db600
    Replacement::draft(0x1411db640, "d3d_1411db640", fn_1411db640 as *const ()), // sub_1411db640
    Replacement::draft(0x1411db810, "d3d_1411db810", tpl_6280::<0x142063440, 0x141834d30> as *const ()), // d3d::QueryHeapManager::d3d::QueryHeapManager::vfunc_0
    Replacement::draft(0x1411db850, "d3d_1411db850", tpl_22130::<0x8, 0x10> as *const ()), // sub_1411db850
    Replacement::draft(0x1411db860, "d3d_1411db860", tpl_22131::<0x10> as *const ()), // sub_1411db860
    Replacement::draft(0x1411db890, "d3d_1411db890", fn_1411db890 as *const ()), // sub_1411db890
    Replacement::draft(0x1411db9b0, "d3d_1411db9b0", fn_1411db9b0 as *const ()), // sub_1411db9b0
    Replacement::draft(0x1411dba80, "d3d_1411dba80", fn_1411dba80 as *const ()), // sub_1411dba80
    Replacement::draft(0x1411dbbc0, "d3d_1411dbbc0", fn_1411dbbc0 as *const ()), // sub_1411dbbc0
    Replacement::draft(0x1411dbd00, "d3d_1411dbd00", fn_1411dbd00 as *const ()), // sub_1411dbd00
    Replacement::draft(0x1411dbe90, "d3d_1411dbe90", tpl_381::<0x1411dbff0, 0x1411dc340> as *const ()), // sub_1411dbe90
    Replacement::draft(0x1411dbff0, "d3d_1411dbff0", fn_1411dbff0 as *const ()), // sub_1411dbff0
    Replacement::draft(0x1411dc340, "d3d_1411dc340", fn_1411dc340 as *const ()), // sub_1411dc340
    Replacement::draft(0x1411dc690, "d3d_1411dc690", tpl_365::<0x14169d070> as *const ()), // sub_1411dc690
    Replacement::draft(0x1411dc6a0, "d3d_1411dc6a0", tpl_365::<0x14169abf0> as *const ()), // sub_1411dc6a0
    Replacement::draft_throws(0x1411dc6b0, "d3d_1411dc6b0", fn_1411dc6b0 as *const (), Throws::Caught), // sub_1411dc6b0
    Replacement::draft_throws(0x1411dc7e0, "d3d_1411dc7e0", fn_1411dc7e0 as *const (), Throws::Caught), // sub_1411dc7e0
    Replacement::draft_throws(0x1411dc910, "d3d_1411dc910", fn_1411dc910 as *const (), Throws::Caught), // sub_1411dc910
    Replacement::draft_throws(0x1411dca40, "d3d_1411dca40", fn_1411dca40 as *const (), Throws::Caught), // sub_1411dca40
    Replacement::draft_throws(0x1411dcb50, "d3d_1411dcb50", fn_1411dcb50 as *const (), Throws::Caught), // sub_1411dcb50
    Replacement::draft(0x1411dce60, "d3d_1411dce60", fn_1411dce60 as *const ()), // sub_1411dce60
    Replacement::draft_throws(0x1411dcfb0, "d3d_1411dcfb0", fn_1411dcfb0 as *const (), Throws::Discard), // sub_1411dcfb0
    Replacement::draft(0x1411dd200, "d3d_1411dd200", fn_1411dd200 as *const ()), // sub_1411dd200
    Replacement::draft_throws(0x1411dd280, "d3d_1411dd280", fn_1411dd280 as *const (), Throws::Discard), // sub_1411dd280
    Replacement::draft(0x1411dd2e0, "d3d_1411dd2e0", fn_1411dd2e0 as *const ()), // sub_1411dd2e0
    Replacement::draft_throws(0x1411dd310, "d3d_1411dd310", fn_1411dd310 as *const (), Throws::Discard), // sub_1411dd310
    Replacement::draft_throws(0x1411dd370, "d3d_1411dd370", fn_1411dd370 as *const (), Throws::Discard), // sub_1411dd370
    Replacement::draft(0x1411dd4f0, "d3d_1411dd4f0", fn_1411dd4f0 as *const ()), // sub_1411dd4f0
    Replacement::draft_throws(0x1411dd5d0, "d3d_1411dd5d0", fn_1411dd5d0 as *const (), Throws::Discard), // sub_1411dd5d0
    Replacement::draft(0x1411dd660, "d3d_1411dd660", fn_1411dd660 as *const ()), // sub_1411dd660
    Replacement::draft(0x1411dd6c0, "d3d_1411dd6c0", fn_1411dd6c0 as *const ()), // sub_1411dd6c0
    Replacement::draft(0x1411dd730, "d3d_1411dd730", fn_1411dd730 as *const ()), // sub_1411dd730
    Replacement::draft_throws(0x1411dd7b0, "d3d_1411dd7b0", fn_1411dd7b0 as *const (), Throws::Discard), // sub_1411dd7b0
    Replacement::draft_throws(0x1411dd860, "d3d_1411dd860", fn_1411dd860 as *const (), Throws::Discard), // sub_1411dd860
    Replacement::draft(0x1411dd9e0, "d3d_1411dd9e0", fn_1411dd9e0 as *const ()), // sub_1411dd9e0
    Replacement::draft(0x1411dda10, "d3d_1411dda10", fn_1411dda10 as *const ()), // sub_1411dda10
    Replacement::draft(0x1411dda90, "d3d_1411dda90", fn_1411dda90 as *const ()), // sub_1411dda90
    Replacement::draft(0x1411ddb00, "d3d_1411ddb00", fn_1411ddb00 as *const ()), // sub_1411ddb00
    Replacement::draft(0x1411ddb70, "d3d_1411ddb70", tpl_427::<0x142051db0, 0x142442de0> as *const ()), // sub_1411ddb70
    Replacement::draft(0x1411ddba0, "d3d_1411ddba0", tpl_427::<0x1420527c0, 0x142442de0> as *const ()), // sub_1411ddba0
    Replacement::draft_throws(0x1411ddbd0, "d3d_1411ddbd0", tpl_22028::<0x142088250, 0x142442de0> as *const (), Throws::Discard), // sub_1411ddbd0
    Replacement::draft(0x1411ddc00, "d3d_1411ddc00", fn_1411ddc00 as *const ()), // sub_1411ddc00
    Replacement::draft(0x1411ddc10, "d3d_1411ddc10", tpl_4771::<0x8, 0x1411defd0, 0x142442de0> as *const ()), // sub_1411ddc10
    Replacement::draft(0x1411ddc40, "d3d_1411ddc40", fn_1411ddc40 as *const ()), // sub_1411ddc40
    Replacement::draft(0x1411ddca0, "d3d_1411ddca0", fn_1411ddca0 as *const ()), // sub_1411ddca0
    Replacement::draft(0x1411ddd30, "unknown_1411ddd30", tpl_22165::<0x148, 0x10, 0x18, 0x142050970, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1411ddd30
    Replacement::draft_throws(0x1411dddd0, "unknown_1411dddd0", fn_1411dddd0 as *const (), Throws::Discard), // sub_1411dddd0
    Replacement::draft_throws(0x1411de040, "unknown_1411de040", fn_1411de040 as *const (), Throws::Discard), // sub_1411de040
    Replacement::draft_throws(0x1411de130, "unknown_1411de130", fn_1411de130 as *const (), Throws::Discard), // sub_1411de130
    Replacement::draft(0x1411de210, "unknown_1411de210", fn_1411de210 as *const ()), // sub_1411de210
    Replacement::draft_throws(0x1411de360, "sparsehash_1411de360", fn_1411de360 as *const (), Throws::Discard), // sub_1411de360
    Replacement::draft_throws(0x1411de510, "sparsehash_1411de510", fn_1411de510 as *const (), Throws::Discard), // sub_1411de510
    Replacement::draft_throws(0x1411de5d0, "sparsehash_1411de5d0", tpl_77984::<0x140227220, 0x1410531f0, 0x140227190, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1411de802> as *const (), Throws::Discard), // sub_1411de5d0
    Replacement::draft_throws(0x1411de810, "unknown_1411de810", fn_1411de810 as *const (), Throws::Discard), // sub_1411de810
    Replacement::draft(0x1411dea10, "unknown_1411dea10", fn_1411dea10 as *const ()), // sub_1411dea10
    Replacement::draft(0x1411dea90, "unknown_1411dea90", fn_1411dea90 as *const ()), // sub_1411dea90
    Replacement::draft(0x1411deb40, "unknown_1411deb40", fn_1411deb40 as *const ()), // sub_1411deb40
    Replacement::draft_throws(0x1411deb90, "unknown_1411deb90", fn_1411deb90 as *const (), Throws::Discard), // sub_1411deb90
    Replacement::draft_throws(0x1411ded20, "unknown_1411ded20", fn_1411ded20 as *const (), Throws::Discard), // sub_1411ded20
    Replacement::draft(0x1411deee0, "unknown_1411deee0", fn_1411deee0 as *const ()), // sub_1411deee0
    Replacement::draft(0x1411def40, "unknown_1411def40", fn_1411def40 as *const ()), // sub_1411def40
    Replacement::draft(0x1411defd0, "unknown_1411defd0", tpl_18985::<0x8, 0x1411def40, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1411defd0
    Replacement::draft(0x1411df0a0, "unknown_1411df0a0", tpl_18985::<0x8, 0x1411deee0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1411df0a0
    Replacement::draft_throws(0x1411df170, "unknown_1411df170", fn_1411df170 as *const (), Throws::Discard), // sub_1411df170
    Replacement::draft(0x1411df270, "unknown_1411df270", fn_1411df270 as *const ()), // sub_1411df270
    Replacement::draft(0x1411df3d0, "unknown_1411df3d0", fn_1411df3d0 as *const ()), // sub_1411df3d0
    Replacement::draft(0x1411df3e0, "unknown_1411df3e0", tpl_22183::<0x1411d2b20, 0xfffffffffffffffb, 0x141834ee8, 0xfffffffffffffff5> as *const ()), // sub_1411df3e0
    Replacement::draft_throws(0x1411df430, "unknown_1411df430", fn_1411df430 as *const (), Throws::Caught), // sub_1411df430
    Replacement::draft(0x1411df590, "unknown_1411df590", fn_1411df590 as *const ()), // sub_1411df590
    Replacement::draft(0x1411df5e0, "unknown_1411df5e0", fn_1411df5e0 as *const ()), // sub_1411df5e0
    Replacement::draft_throws(0x1411df630, "unknown_1411df630", tpl_22028::<0x14204f860, 0x142442de0> as *const (), Throws::Discard), // sub_1411df630
    Replacement::draft(0x1411df660, "unknown_1411df660", fn_1411df660 as *const ()), // sub_1411df660
    Replacement::draft(0x1411df710, "unknown_1411df710", tpl_22188::<0x10, 0x28, 0x20, 0x18> as *const ()), // sub_1411df710
    Replacement::draft_throws(0x1411df740, "unknown_1411df740", fn_1411df740 as *const (), Throws::Discard), // sub_1411df740
    Replacement::draft(0x1411df800, "unknown_1411df800", fn_1411df800 as *const ()), // sub_1411df800
    Replacement::draft_throws(0x1411df8b0, "unknown_1411df8b0", fn_1411df8b0 as *const (), Throws::Caught), // sub_1411df8b0
    Replacement::draft(0x1411df930, "unknown_1411df930", fn_1411df930 as *const ()), // sub_1411df930
    Replacement::draft_throws(0x1411dfa50, "unknown_1411dfa50", fn_1411dfa50 as *const (), Throws::Discard), // sub_1411dfa50
    Replacement::draft_throws(0x1411dfa70, "unknown_1411dfa70", fn_1411dfa70 as *const (), Throws::Discard), // sub_1411dfa70
    Replacement::draft_throws(0x1411dfb70, "unknown_1411dfb70", tpl_13418::<0x1411e07e0> as *const (), Throws::Discard), // sub_1411dfb70
    Replacement::draft_throws(0x1411dfb80, "unknown_1411dfb80", fn_1411dfb80 as *const (), Throws::Discard), // sub_1411dfb80
    Replacement::draft_throws(0x1411dfc30, "unknown_1411dfc30", fn_1411dfc30 as *const (), Throws::Discard), // sub_1411dfc30
    Replacement::draft_throws(0x1411dfce0, "unknown_1411dfce0", fn_1411dfce0 as *const (), Throws::Discard), // sub_1411dfce0
    Replacement::draft_throws(0x1411dfda0, "unknown_1411dfda0", fn_1411dfda0 as *const (), Throws::Discard), // sub_1411dfda0
    Replacement::draft(0x1411dff20, "unknown_1411dff20", fn_1411dff20 as *const ()), // sub_1411dff20
    Replacement::draft_throws(0x1411e0030, "unknown_1411e0030", fn_1411e0030 as *const (), Throws::Discard), // sub_1411e0030
    Replacement::draft_throws(0x1411e0150, "unknown_1411e0150", fn_1411e0150 as *const (), Throws::Discard), // sub_1411e0150
    Replacement::draft(0x1411e0270, "unknown_1411e0270", fn_1411e0270 as *const ()), // sub_1411e0270
    Replacement::draft(0x1411e0350, "unknown_1411e0350", fn_1411e0350 as *const ()), // sub_1411e0350
    Replacement::draft(0x1411e0430, "unknown_1411e0430", fn_1411e0430 as *const ()), // sub_1411e0430
    Replacement::draft(0x1411e0480, "unknown_1411e0480", fn_1411e0480 as *const ()), // sub_1411e0480
    Replacement::draft(0x1411e05c0, "unknown_1411e05c0", fn_1411e05c0 as *const ()), // sub_1411e05c0
    Replacement::draft_throws(0x1411e07e0, "unknown_1411e07e0", fn_1411e07e0 as *const (), Throws::Discard), // sub_1411e07e0
    Replacement::draft_throws(0x1411e0860, "unknown_1411e0860", fn_1411e0860 as *const (), Throws::Discard), // sub_1411e0860
    Replacement::draft(0x1411e08e0, "unknown_1411e08e0", fn_1411e08e0 as *const ()), // sub_1411e08e0
    Replacement::draft_throws(0x1411e0be0, "unknown_1411e0be0", fn_1411e0be0 as *const (), Throws::Caught), // sub_1411e0be0
    Replacement::draft_throws(0x1411e0f40, "unknown_1411e0f40", fn_1411e0f40 as *const (), Throws::Discard), // sub_1411e0f40
    Replacement::draft(0x1411e1040, "unknown_1411e1040", fn_1411e1040 as *const ()), // sub_1411e1040
    Replacement::draft(0x1411e11c0, "unknown_1411e11c0", fn_1411e11c0 as *const ()), // sub_1411e11c0
    Replacement::draft_throws(0x1411e1210, "unknown_1411e1210", fn_1411e1210 as *const (), Throws::Discard), // sub_1411e1210
    Replacement::draft_throws(0x1411e1280, "unknown_1411e1280", fn_1411e1280 as *const (), Throws::Discard), // sub_1411e1280
    Replacement::draft(0x1411e1340, "unknown_1411e1340", fn_1411e1340 as *const ()), // sub_1411e1340
    Replacement::draft(0x1411e1390, "unknown_1411e1390", tpl_22213::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1411e0350> as *const ()), // sub_1411e1390
    Replacement::draft(0x1411e1500, "unknown_1411e1500", fn_1411e1500 as *const ()), // sub_1411e1500
    Replacement::draft(0x1411e1570, "unknown_1411e1570", fn_1411e1570 as *const ()), // sub_1411e1570
    Replacement::draft_throws(0x1411e1630, "unknown_1411e1630", fn_1411e1630 as *const (), Throws::Discard), // sub_1411e1630
    Replacement::draft(0x1411e1780, "unknown_1411e1780", fn_1411e1780 as *const ()), // sub_1411e1780
    Replacement::draft(0x1411e1910, "unknown_1411e1910", fn_1411e1910 as *const ()), // sub_1411e1910
    Replacement::draft_throws(0x1411e1980, "unknown_1411e1980", fn_1411e1980 as *const (), Throws::Discard), // sub_1411e1980
    Replacement::draft(0x1411e1a10, "unknown_1411e1a10", fn_1411e1a10 as *const ()), // sub_1411e1a10
    Replacement::draft(0x1411e1b00, "unknown_1411e1b00", fn_1411e1b00 as *const ()), // sub_1411e1b00
    Replacement::draft(0x1411e1b40, "unknown_1411e1b40", fn_1411e1b40 as *const ()), // sub_1411e1b40
    Replacement::draft(0x1411e1f60, "unknown_1411e1f60", fn_1411e1f60 as *const ()), // sub_1411e1f60
    Replacement::draft(0x1411e2330, "unknown_1411e2330", fn_1411e2330 as *const ()), // sub_1411e2330
    Replacement::draft(0x1411e2a60, "unknown_1411e2a60", fn_1411e2a60 as *const ()), // sub_1411e2a60
    Replacement::draft(0x1411e2ab0, "unknown_1411e2ab0", fn_1411e2ab0 as *const ()), // sub_1411e2ab0
    Replacement::draft_throws(0x1411e2b80, "unknown_1411e2b80", fn_1411e2b80 as *const (), Throws::Caught), // sub_1411e2b80
    Replacement::draft_throws(0x1411e2c40, "unknown_1411e2c40", tpl_22033::<0x14204f860, 0x142442de0> as *const (), Throws::Discard), // sub_1411e2c40
    Replacement::draft(0x1411e2c90, "unknown_1411e2c90", fn_1411e2c90 as *const ()), // sub_1411e2c90
    Replacement::draft(0x1411e2ca0, "d3d_1411e2ca0", fn_1411e2ca0 as *const ()), // d3d::GlobalDescriptorManager::d3d::GlobalDescriptorManager::vfunc_0
    Replacement::draft(0x1411e2ce0, "d3d_1411e2ce0", fn_1411e2ce0 as *const ()), // sub_1411e2ce0
    Replacement::draft(0x1411e2d00, "d3d_1411e2d00", fn_1411e2d00 as *const ()), // sub_1411e2d00
    Replacement::draft(0x1411e2d90, "d3d_1411e2d90", tpl_80471::<0x143a637a0, 0x143a637a4, 0x143a63790, 0x1401dec00, 0x143a63798, 0x1425371a0, 0x142533200, 0x143a6379c> as *const ()), // sub_1411e2d90
    Replacement::draft(0x1411e2f60, "d3d_1411e2f60", fn_1411e2f60 as *const ()), // sub_1411e2f60
    Replacement::draft(0x1411e3040, "d3d_1411e3040", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1411e2f60> as *const ()), // sub_1411e3040
    Replacement::draft(0x1411e31b0, "d3d_1411e31b0", fn_1411e31b0 as *const ()), // sub_1411e31b0
    Replacement::draft(0x1411e3240, "d3d_1411e3240", fn_1411e3240 as *const ()), // sub_1411e3240
    Replacement::draft(0x1411e32b0, "d3d_1411e32b0", fn_1411e32b0 as *const ()), // sub_1411e32b0
    Replacement::draft(0x1411e3350, "d3d_1411e3350", tpl_365::<0x1411e32b0> as *const ()), // sub_1411e3350
    Replacement::draft(0x1411e3360, "d3d_1411e3360", tpl_22235::<0x18, 0x10> as *const ()), // sub_1411e3360
    Replacement::draft(0x1411e3380, "d3d_1411e3380", tpl_15960::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x20> as *const ()), // sub_1411e3380
    Replacement::draft(0x1411e3410, "d3d_1411e3410", fn_1411e3410 as *const ()), // sub_1411e3410
    Replacement::draft(0x1411e3570, "d3d_1411e3570", fn_1411e3570 as *const ()), // sub_1411e3570
    Replacement::draft(0x1411e36c0, "d3d_1411e36c0", fn_1411e36c0 as *const ()), // d3d::ConstantValueProviderRegistry::d3d::ConstantValueProviderRegistry::vfunc_0
    Replacement::draft(0x1411e3700, "unknown_1411e3700", fn_1411e3700 as *const ()), // sub_1411e3700
    Replacement::draft_throws(0x1411e3720, "unknown_1411e3720", fn_1411e3720 as *const (), Throws::Caught), // sub_1411e3720
    Replacement::draft(0x1411e3790, "unknown_1411e3790", fn_1411e3790 as *const ()), // sub_1411e3790
    Replacement::draft(0x1411e38a0, "unknown_1411e38a0", fn_1411e38a0 as *const ()), // sub_1411e38a0
    Replacement::draft(0x1411e3960, "unknown_1411e3960", fn_1411e3960 as *const ()), // sub_1411e3960
    Replacement::draft(0x1411e3b70, "unknown_1411e3b70", fn_1411e3b70 as *const ()), // sub_1411e3b70
    Replacement::draft_throws(0x1411e3c20, "unknown_1411e3c20", fn_1411e3c20 as *const (), Throws::Caught), // sub_1411e3c20
    Replacement::draft(0x1411e3d70, "unknown_1411e3d70", fn_1411e3d70 as *const ()), // sub_1411e3d70
    Replacement::draft(0x1411e3d90, "unknown_1411e3d90", fn_1411e3d90 as *const ()), // sub_1411e3d90
    Replacement::draft(0x1411e3fe0, "unknown_1411e3fe0", fn_1411e3fe0 as *const ()), // sub_1411e3fe0
    Replacement::draft_throws(0x1411e4210, "unknown_1411e4210", fn_1411e4210 as *const (), Throws::Discard), // sub_1411e4210
    Replacement::draft_throws(0x1411e43c0, "unknown_1411e43c0", fn_1411e43c0 as *const (), Throws::Discard), // sub_1411e43c0
    Replacement::draft(0x1411e4f50, "unknown_1411e4f50", fn_1411e4f50 as *const ()), // sub_1411e4f50
    Replacement::draft_throws(0x1411e5200, "unknown_1411e5200", fn_1411e5200 as *const (), Throws::Discard), // sub_1411e5200
    Replacement::draft(0x1411e57c0, "unknown_1411e57c0", fn_1411e57c0 as *const ()), // sub_1411e57c0
    Replacement::draft(0x1411e5a70, "unknown_1411e5a70", fn_1411e5a70 as *const ()), // sub_1411e5a70
    Replacement::draft_throws(0x1411e5d20, "unknown_1411e5d20", fn_1411e5d20 as *const (), Throws::Discard), // sub_1411e5d20
    Replacement::draft(0x1411e62f0, "unknown_1411e62f0", fn_1411e62f0 as *const ()), // sub_1411e62f0
    Replacement::draft_throws(0x1411e65a0, "unknown_1411e65a0", fn_1411e65a0 as *const (), Throws::Discard), // sub_1411e65a0
    Replacement::draft_throws(0x1411e6b60, "sparsehash_1411e6b60", fn_1411e6b60 as *const (), Throws::Discard), // sub_1411e6b60
    Replacement::draft_throws(0x1411e6e70, "sparsehash_1411e6e70", fn_1411e6e70 as *const (), Throws::Discard), // sub_1411e6e70
    Replacement::draft_throws(0x1411e70b0, "sparsehash_1411e70b0", fn_1411e70b0 as *const (), Throws::Discard), // sub_1411e70b0
    Replacement::draft(0x1411e72e0, "unknown_1411e72e0", fn_1411e72e0 as *const ()), // sub_1411e72e0
    Replacement::draft_throws(0x1411e73b0, "unknown_1411e73b0", fn_1411e73b0 as *const (), Throws::Caught), // sub_1411e73b0
    Replacement::draft_throws(0x1411e74e0, "unknown_1411e74e0", fn_1411e74e0 as *const (), Throws::Discard), // sub_1411e74e0
    Replacement::draft(0x1411e7b70, "unknown_1411e7b70", fn_1411e7b70 as *const ()), // sub_1411e7b70
    Replacement::draft(0x1411e7ce0, "unknown_1411e7ce0", fn_1411e7ce0 as *const ()), // sub_1411e7ce0
    Replacement::draft(0x1411e7fc0, "unknown_1411e7fc0", fn_1411e7fc0 as *const ()), // sub_1411e7fc0
    Replacement::draft(0x1411e8020, "unknown_1411e8020", fn_1411e8020 as *const ()), // sub_1411e8020
    Replacement::draft(0x1411e8110, "unknown_1411e8110", fn_1411e8110 as *const ()), // sub_1411e8110
    Replacement::draft(0x1411e8190, "unknown_1411e8190", fn_1411e8190 as *const ()), // sub_1411e8190
    Replacement::draft(0x1411e8250, "unknown_1411e8250", fn_1411e8250 as *const ()), // sub_1411e8250
    Replacement::draft(0x1411e8360, "unknown_1411e8360", fn_1411e8360 as *const ()), // sub_1411e8360
    Replacement::draft(0x1411e8490, "unknown_1411e8490", fn_1411e8490 as *const ()), // sub_1411e8490
    Replacement::draft(0x1411e8760, "unknown_1411e8760", fn_1411e8760 as *const ()), // sub_1411e8760
    Replacement::draft(0x1411e87f0, "unknown_1411e87f0", fn_1411e87f0 as *const ()), // sub_1411e87f0
    Replacement::draft(0x1411e8aa0, "unknown_1411e8aa0", fn_1411e8aa0 as *const ()), // sub_1411e8aa0
    Replacement::draft(0x1411e8b10, "unknown_1411e8b10", fn_1411e8b10 as *const ()), // sub_1411e8b10
    Replacement::draft(0x1411e8b70, "unknown_1411e8b70", tpl_170::<0x142075970> as *const ()), // sub_1411e8b70
    Replacement::draft(0x1411e8b80, "unknown_1411e8b80", tpl_170::<0x1411e8ed0> as *const ()), // sub_1411e8b80
    Replacement::draft(0x1411e8b90, "unknown_1411e8b90", tpl_22260::<0x1411e8bb0, 0xfffffffffffffff3, 0x141834ee8> as *const ()), // sub_1411e8b90
    Replacement::draft_throws(0x1411e8bb0, "unknown_1411e8bb0", fn_1411e8bb0 as *const (), Throws::Discard), // sub_1411e8bb0
    Replacement::draft(0x1411e8c00, "unknown_1411e8c00", fn_1411e8c00 as *const ()), // sub_1411e8c00
    Replacement::draft(0x1411e8ed0, "unknown_1411e8ed0", fn_1411e8ed0 as *const ()), // sub_1411e8ed0
    Replacement::draft(0x1411e8fa0, "unknown_1411e8fa0", fn_1411e8fa0 as *const ()), // sub_1411e8fa0
    Replacement::draft(0x1411e9320, "unknown_1411e9320", fn_1411e9320 as *const ()), // sub_1411e9320
    Replacement::draft(0x1411e9470, "unknown_1411e9470", tpl_22265::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // sub_1411e9470
    Replacement::draft(0x1411e94a0, "unknown_1411e94a0", fn_1411e94a0 as *const ()), // sub_1411e94a0
    Replacement::draft(0x1411e9570, "unknown_1411e9570", fn_1411e9570 as *const ()), // sub_1411e9570
    Replacement::draft(0x1411e9610, "unknown_1411e9610", fn_1411e9610 as *const ()), // sub_1411e9610
    Replacement::draft(0x1411e96c0, "unknown_1411e96c0", fn_1411e96c0 as *const ()), // sub_1411e96c0
    Replacement::draft_throws(0x1411e9870, "ak_1411e9870", fn_1411e9870 as *const (), Throws::Caught), // sub_1411e9870
    Replacement::draft_throws(0x1411e98e0, "ak_1411e98e0", fn_1411e98e0 as *const (), Throws::Discard), // sub_1411e98e0
    Replacement::draft_throws(0x1411e9950, "unknown_1411e9950", tpl_838::<0x1411ea0c0> as *const (), Throws::Discard), // sub_1411e9950
    Replacement::draft_throws(0x1411e9960, "sparsehash_1411e9960", fn_1411e9960 as *const (), Throws::Caught), // sub_1411e9960
    Replacement::draft_throws(0x1411e9c00, "sparsehash_1411e9c00", fn_1411e9c00 as *const (), Throws::Caught), // sub_1411e9c00
    Replacement::draft_throws(0x1411e9ea0, "sparsehash_1411e9ea0", fn_1411e9ea0 as *const (), Throws::Discard), // sub_1411e9ea0
    Replacement::draft(0x1411e9f50, "sparsehash_1411e9f50", fn_1411e9f50 as *const ()), // sub_1411e9f50
    Replacement::draft_throws(0x1411ea0c0, "sparsehash_1411ea0c0", tpl_22276::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1411ea0c0
    Replacement::draft_throws(0x1411ea140, "sparsehash_1411ea140", fn_1411ea140 as *const (), Throws::Discard), // sub_1411ea140
    Replacement::draft(0x1411ea380, "sparsehash_1411ea380", fn_1411ea380 as *const ()), // sub_1411ea380
    Replacement::draft(0x1411ea510, "sparsehash_1411ea510", fn_1411ea510 as *const ()), // sub_1411ea510
    Replacement::draft_throws(0x1411ea690, "unknown_1411ea690", fn_1411ea690 as *const (), Throws::Discard), // sub_1411ea690
    Replacement::draft(0x1411ea800, "unknown_1411ea800", tpl_22280::<0x8, 0x1426d01e0> as *const ()), // sub_1411ea800
    Replacement::draft_throws(0x1411ea860, "unknown_1411ea860", fn_1411ea860 as *const (), Throws::Discard), // sub_1411ea860
    Replacement::draft(0x1411eab50, "unknown_1411eab50", fn_1411eab50 as *const ()), // sub_1411eab50
    Replacement::draft(0x1411eac10, "unknown_1411eac10", fn_1411eac10 as *const ()), // sub_1411eac10
    Replacement::draft(0x1411ead60, "unknown_1411ead60", fn_1411ead60 as *const ()), // sub_1411ead60
    Replacement::draft(0x1411eaff0, "unknown_1411eaff0", tpl_381::<0x1411eb150, 0x1411eb370> as *const ()), // sub_1411eaff0
    Replacement::draft(0x1411eb150, "unknown_1411eb150", fn_1411eb150 as *const ()), // sub_1411eb150
    Replacement::draft(0x1411eb370, "unknown_1411eb370", fn_1411eb370 as *const ()), // sub_1411eb370
    Replacement::draft(0x1411eb600, "unknown_1411eb600", fn_1411eb600 as *const ()), // sub_1411eb600
    Replacement::draft(0x1411eb740, "unknown_1411eb740", fn_1411eb740 as *const ()), // sub_1411eb740
    Replacement::draft(0x1411ebb80, "unknown_1411ebb80", fn_1411ebb80 as *const ()), // sub_1411ebb80
    Replacement::draft(0x1411ebc40, "unknown_1411ebc40", fn_1411ebc40 as *const ()), // sub_1411ebc40
    Replacement::draft_throws(0x1411ebdd0, "unknown_1411ebdd0", tpl_22288::<0x10, 0x14241c190, 0x18, 0x1411ec170> as *const (), Throws::Discard), // sub_1411ebdd0
    Replacement::draft(0x1411ebe00, "unknown_1411ebe00", fn_1411ebe00 as *const ()), // sub_1411ebe00
    Replacement::draft(0x1411ebe80, "unknown_1411ebe80", fn_1411ebe80 as *const ()), // sub_1411ebe80
    Replacement::draft_throws(0x1411ebfc0, "std_1411ebfc0", fn_1411ebfc0 as *const (), Throws::Discard), // sub_1411ebfc0
    Replacement::draft_throws(0x1411ec170, "unknown_1411ec170", fn_1411ec170 as *const (), Throws::Discard), // sub_1411ec170
    Replacement::draft_throws(0x1411ec390, "unknown_1411ec390", tpl_22288::<0x10, 0x14241c190, 0x18, 0x1411ec4e0> as *const (), Throws::Discard), // sub_1411ec390
    Replacement::draft_throws(0x1411ec3c0, "unknown_1411ec3c0", tpl_22293::<0x18, 0x14241c190, 0x20, 0x8, 0x10> as *const (), Throws::Discard), // sub_1411ec3c0
    Replacement::draft_throws(0x1411ec410, "unknown_1411ec410", fn_1411ec410 as *const (), Throws::Discard), // sub_1411ec410
    Replacement::draft_throws(0x1411ec4e0, "unknown_1411ec4e0", fn_1411ec4e0 as *const (), Throws::Discard), // sub_1411ec4e0
    Replacement::draft_throws(0x1411ec570, "unknown_1411ec570", fn_1411ec570 as *const (), Throws::Discard), // sub_1411ec570
    Replacement::draft_throws(0x1411ec600, "unknown_1411ec600", fn_1411ec600 as *const (), Throws::Discard), // sub_1411ec600
    Replacement::draft_throws(0x1411ec7b0, "unknown_1411ec7b0", fn_1411ec7b0 as *const (), Throws::Discard), // sub_1411ec7b0
    Replacement::draft_throws(0x1411ec9d0, "unknown_1411ec9d0", fn_1411ec9d0 as *const (), Throws::Discard), // sub_1411ec9d0
    Replacement::draft_throws(0x1411eca70, "unknown_1411eca70", fn_1411eca70 as *const (), Throws::Discard), // sub_1411eca70
    Replacement::draft_throws(0x1411eccd0, "unknown_1411eccd0", fn_1411eccd0 as *const (), Throws::Discard), // sub_1411eccd0
    Replacement::draft_throws(0x1411ed390, "unknown_1411ed390", fn_1411ed390 as *const (), Throws::Discard), // sub_1411ed390
    Replacement::draft_throws(0x1411ed540, "unknown_1411ed540", fn_1411ed540 as *const (), Throws::Discard), // sub_1411ed540
    Replacement::draft_throws(0x1411ed730, "unknown_1411ed730", fn_1411ed730 as *const (), Throws::Discard), // sub_1411ed730
    Replacement::draft(0x1411ed7d0, "unknown_1411ed7d0", tpl_170::<0x1411ed7e0> as *const ()), // sub_1411ed7d0
    Replacement::draft(0x1411ed890, "unknown_1411ed890", fn_1411ed890 as *const ()), // sub_1411ed890
    Replacement::draft(0x1411eda70, "unknown_1411eda70", tpl_365::<0x1411eda80> as *const ()), // sub_1411eda70
    Replacement::draft(0x1411eda80, "unknown_1411eda80", fn_1411eda80 as *const ()), // sub_1411eda80
    Replacement::draft(0x1411edb70, "unknown_1411edb70", fn_1411edb70 as *const ()), // sub_1411edb70
    Replacement::draft(0x1411edc30, "unknown_1411edc30", fn_1411edc30 as *const ()), // sub_1411edc30
    Replacement::draft_throws(0x1411edd90, "unknown_1411edd90", fn_1411edd90 as *const (), Throws::Discard), // sub_1411edd90
    Replacement::draft(0x1411edf00, "unknown_1411edf00", tpl_20928::<0x8, 0x18, 0x1410f6370, 0x14240ab10> as *const ()), // sub_1411edf00
    Replacement::draft(0x1411edf40, "unknown_1411edf40", fn_1411edf40 as *const ()), // sub_1411edf40
    Replacement::draft(0x1411ee040, "unknown_1411ee040", fn_1411ee040 as *const ()), // sub_1411ee040
    Replacement::draft(0x1411ee120, "unknown_1411ee120", fn_1411ee120 as *const ()), // sub_1411ee120
    Replacement::draft(0x1411ee1c0, "unknown_1411ee1c0", fn_1411ee1c0 as *const ()), // sub_1411ee1c0
    Replacement::draft_throws(0x1411ee2f0, "unknown_1411ee2f0", fn_1411ee2f0 as *const (), Throws::Discard), // sub_1411ee2f0
    Replacement::draft_throws(0x1411ee380, "unknown_1411ee380", tpl_19296::<0xc, 0x8, 0x14241c0a0, 0x14183acc8, 0x14241c190> as *const (), Throws::Discard), // sub_1411ee380
    Replacement::draft(0x1411ee400, "unknown_1411ee400", tpl_22213::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1410f5f00> as *const ()), // sub_1411ee400
    Replacement::draft(0x1411ee570, "unknown_1411ee570", fn_1411ee570 as *const ()), // sub_1411ee570
    Replacement::draft_throws(0x1411ee710, "unknown_1411ee710", fn_1411ee710 as *const (), Throws::Discard), // sub_1411ee710
    Replacement::draft_throws(0x1411ee730, "unknown_1411ee730", fn_1411ee730 as *const (), Throws::Discard), // sub_1411ee730
    Replacement::draft(0x1411ee7e0, "unknown_1411ee7e0", fn_1411ee7e0 as *const ()), // sub_1411ee7e0
    Replacement::draft_throws(0x1411ee930, "sparsehash_1411ee930", fn_1411ee930 as *const (), Throws::Caught), // sub_1411ee930
    Replacement::draft_throws(0x1411eebd0, "sparsehash_1411eebd0", fn_1411eebd0 as *const (), Throws::Caught), // sub_1411eebd0
    Replacement::draft(0x1411ef4a0, "sparsehash_1411ef4a0", fn_1411ef4a0 as *const ()), // sub_1411ef4a0
    Replacement::draft(0x1411ef5e0, "sparsehash_1411ef5e0", fn_1411ef5e0 as *const ()), // sub_1411ef5e0
    Replacement::draft(0x1411ef710, "sparsehash_1411ef710", fn_1411ef710 as *const ()), // sub_1411ef710
    Replacement::draft(0x1411ef830, "sparsehash_1411ef830", fn_1411ef830 as *const ()), // sub_1411ef830
    Replacement::draft_throws(0x1411efa40, "sparsehash_1411efa40", fn_1411efa40 as *const (), Throws::Discard), // sub_1411efa40
    Replacement::draft(0x1411f0270, "sparsehash_1411f0270", fn_1411f0270 as *const ()), // sub_1411f0270
    Replacement::draft_throws(0x1411f0500, "sparsehash_1411f0500", fn_1411f0500 as *const (), Throws::Discard), // sub_1411f0500
    Replacement::draft(0x1411f0610, "sparsehash_1411f0610", tpl_6537::<0x1403d8aa0, 0x14183acc8, 0x1401dec00, 0x1401de830> as *const ()), // sub_1411f0610
    Replacement::draft(0x1411f0750, "sparsehash_1411f0750", fn_1411f0750 as *const ()), // sub_1411f0750
    Replacement::draft(0x1411f08b0, "sparsehash_1411f08b0", fn_1411f08b0 as *const ()), // sub_1411f08b0
    Replacement::draft_throws(0x1411f0a20, "sparsehash_1411f0a20", fn_1411f0a20 as *const (), Throws::Caught), // sub_1411f0a20
    Replacement::draft(0x1411f0d70, "unknown_1411f0d70", fn_1411f0d70 as *const ()), // sub_1411f0d70
    Replacement::draft(0x1411f11c0, "unknown_1411f11c0", fn_1411f11c0 as *const ()), // sub_1411f11c0
    Replacement::draft(0x1411f11e0, "unknown_1411f11e0", tpl_22332::<0xffffffffffffffff, 0x18, 0x8, 0x1420aa290, 0x1420aa4b0, 0x1403d1980, 0x10, 0xfffffffffffffff8> as *const ()), // sub_1411f11e0
    Replacement::draft_throws(0x1411f1340, "unknown_1411f1340", fn_1411f1340 as *const (), Throws::Discard), // sub_1411f1340
    Replacement::draft(0x1411f1460, "unknown_1411f1460", fn_1411f1460 as *const ()), // sub_1411f1460
    Replacement::draft(0x1411f16c0, "unknown_1411f16c0", fn_1411f16c0 as *const ()), // sub_1411f16c0
    Replacement::draft_throws(0x1411f1ca0, "unknown_1411f1ca0", fn_1411f1ca0 as *const (), Throws::Discard), // sub_1411f1ca0
    Replacement::draft(0x1411f1ff0, "unknown_1411f1ff0", tpl_22340::<0x1420ba1e0, 0x1420ba340> as *const ()), // sub_1411f1ff0
    Replacement::draft(0x1411f2040, "unknown_1411f2040", fn_1411f2040 as *const ()), // sub_1411f2040
    Replacement::draft(0x1411f2320, "unknown_1411f2320", fn_1411f2320 as *const ()), // sub_1411f2320
    Replacement::draft(0x1411f2380, "unknown_1411f2380", fn_1411f2380 as *const ()), // sub_1411f2380
    Replacement::draft(0x1411f2650, "unknown_1411f2650", fn_1411f2650 as *const ()), // sub_1411f2650
    Replacement::draft(0x1411f2730, "unknown_1411f2730", fn_1411f2730 as *const ()), // sub_1411f2730
    Replacement::draft(0x1411f2830, "unknown_1411f2830", fn_1411f2830 as *const ()), // sub_1411f2830
    Replacement::draft(0x1411f2920, "unknown_1411f2920", fn_1411f2920 as *const ()), // sub_1411f2920
    Replacement::draft(0x1411f2990, "unknown_1411f2990", fn_1411f2990 as *const ()), // sub_1411f2990
    Replacement::draft(0x1411f2c00, "unknown_1411f2c00", fn_1411f2c00 as *const ()), // sub_1411f2c00
    Replacement::draft(0x1411f30e0, "unknown_1411f30e0", tpl_80707::<0x143a082f8, 0x142407c80, 0x142485aa0, 0x1433ddd10, 0x142486460, 0x142645580> as *const ()), // sub_1411f30e0
    Replacement::draft(0x1411f3240, "unknown_1411f3240", tpl_80707::<0x143a082f8, 0x142407c80, 0x142485aa0, 0x1433ddc68, 0x142486460, 0x142645580> as *const ()), // sub_1411f3240
    Replacement::draft(0x1411f33a0, "unknown_1411f33a0", fn_1411f33a0 as *const ()), // sub_1411f33a0
    Replacement::draft(0x1411f3850, "unknown_1411f3850", fn_1411f3850 as *const ()), // sub_1411f3850
    Replacement::draft(0x1411f39d0, "unknown_1411f39d0", fn_1411f39d0 as *const ()), // sub_1411f39d0
    Replacement::draft(0x1411f3a10, "unknown_1411f3a10", fn_1411f3a10 as *const ()), // sub_1411f3a10
    Replacement::draft(0x1411f3a90, "unknown_1411f3a90", fn_1411f3a90 as *const ()), // sub_1411f3a90
    Replacement::draft(0x1411f3b90, "unknown_1411f3b90", fn_1411f3b90 as *const ()), // sub_1411f3b90
    Replacement::draft(0x1411f3d40, "unknown_1411f3d40", fn_1411f3d40 as *const ()), // sub_1411f3d40
    Replacement::draft(0x1411f3e00, "rex_1411f3e00", fn_1411f3e00 as *const ()), // rex::Compiler::rex::Compiler::vfunc_0
    Replacement::draft(0x1411f3e70, "unknown_1411f3e70", fn_1411f3e70 as *const ()), // sub_1411f3e70
    Replacement::draft(0x1411f3ec0, "unknown_1411f3ec0", fn_1411f3ec0 as *const ()), // sub_1411f3ec0
    Replacement::draft(0x1411f3f00, "unknown_1411f3f00", fn_1411f3f00 as *const ()), // sub_1411f3f00
    Replacement::draft_throws(0x1411f4000, "unknown_1411f4000", fn_1411f4000 as *const (), Throws::Discard), // sub_1411f4000
    Replacement::draft(0x1411f4370, "unknown_1411f4370", fn_1411f4370 as *const ()), // sub_1411f4370
    Replacement::draft(0x1411f43f0, "unknown_1411f43f0", tpl_22368::<0x28, 0x8, 0x19, 0x10, 0x140246bb0, 0x1401e66f0> as *const ()), // sub_1411f43f0
    Replacement::draft(0x1411f4450, "unknown_1411f4450", tpl_22368::<0x30, 0x8, 0x19, 0x10, 0x140246bb0, 0x1401e66f0> as *const ()), // sub_1411f4450
    Replacement::draft(0x1411f44b0, "unknown_1411f44b0", fn_1411f44b0 as *const ()), // sub_1411f44b0
    Replacement::draft(0x1411f44e0, "unknown_1411f44e0", fn_1411f44e0 as *const ()), // sub_1411f44e0
    Replacement::draft(0x1411f4530, "unknown_1411f4530", fn_1411f4530 as *const ()), // sub_1411f4530
    Replacement::draft(0x1411f4590, "unknown_1411f4590", fn_1411f4590 as *const ()), // sub_1411f4590
    Replacement::draft(0x1411f4630, "unknown_1411f4630", fn_1411f4630 as *const ()), // sub_1411f4630
    Replacement::draft(0x1411f4670, "unknown_1411f4670", fn_1411f4670 as *const ()), // sub_1411f4670
    Replacement::draft(0x1411f4700, "unknown_1411f4700", tpl_22375::<0x4, 0x48, 0x10> as *const ()), // sub_1411f4700
    Replacement::draft(0x1411f4730, "unknown_1411f4730", fn_1411f4730 as *const ()), // sub_1411f4730
    Replacement::draft(0x1411f4910, "unknown_1411f4910", fn_1411f4910 as *const ()), // sub_1411f4910
    Replacement::draft(0x1411f4970, "unknown_1411f4970", fn_1411f4970 as *const ()), // sub_1411f4970
    Replacement::draft(0x1411f49e0, "unknown_1411f49e0", fn_1411f49e0 as *const ()), // sub_1411f49e0
    Replacement::draft(0x1411f4af0, "unknown_1411f4af0", fn_1411f4af0 as *const ()), // sub_1411f4af0
    Replacement::draft(0x1411f5380, "unknown_1411f5380", fn_1411f5380 as *const ()), // sub_1411f5380
    Replacement::draft(0x1411f5520, "unknown_1411f5520", fn_1411f5520 as *const ()), // sub_1411f5520
    Replacement::draft(0x1411f5690, "unknown_1411f5690", fn_1411f5690 as *const ()), // sub_1411f5690
    Replacement::draft(0x1411f5800, "unknown_1411f5800", fn_1411f5800 as *const ()), // sub_1411f5800
    Replacement::draft_throws(0x1411f58a0, "unknown_1411f58a0", fn_1411f58a0 as *const (), Throws::Discard), // sub_1411f58a0
    Replacement::draft_throws(0x1411f5a90, "unknown_1411f5a90", fn_1411f5a90 as *const (), Throws::Discard), // sub_1411f5a90
    Replacement::draft_throws(0x1411f5c20, "unknown_1411f5c20", fn_1411f5c20 as *const (), Throws::Discard), // sub_1411f5c20
    Replacement::draft(0x1411f5db0, "unknown_1411f5db0", fn_1411f5db0 as *const ()), // sub_1411f5db0
    Replacement::draft(0x1411f5ec0, "unknown_1411f5ec0", fn_1411f5ec0 as *const ()), // sub_1411f5ec0
    Replacement::draft(0x1411f5fa0, "unknown_1411f5fa0", tpl_10127::<0x8, 0x14065f150, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1411f5fa0
    Replacement::draft(0x1411f6070, "unknown_1411f6070", fn_1411f6070 as *const ()), // sub_1411f6070
    Replacement::draft(0x1411f61c0, "unknown_1411f61c0", fn_1411f61c0 as *const ()), // sub_1411f61c0
    Replacement::draft_throws(0x1411f6260, "unknown_1411f6260", fn_1411f6260 as *const (), Throws::Discard), // sub_1411f6260
    Replacement::draft_throws(0x1411f63b0, "unknown_1411f63b0", fn_1411f63b0 as *const (), Throws::Discard), // sub_1411f63b0
    Replacement::draft(0x1411f64b0, "unknown_1411f64b0", fn_1411f64b0 as *const ()), // sub_1411f64b0
    Replacement::draft(0x1411f6570, "unknown_1411f6570", fn_1411f6570 as *const ()), // sub_1411f6570
    Replacement::draft(0x1411f6c40, "unknown_1411f6c40", fn_1411f6c40 as *const ()), // sub_1411f6c40
    Replacement::draft(0x1411f6de0, "unknown_1411f6de0", fn_1411f6de0 as *const ()), // sub_1411f6de0
    Replacement::draft(0x1411f6e80, "unknown_1411f6e80", fn_1411f6e80 as *const ()), // sub_1411f6e80
    Replacement::draft(0x1411f6ec0, "unknown_1411f6ec0", fn_1411f6ec0 as *const ()), // sub_1411f6ec0
    Replacement::draft(0x1411f6fa0, "unknown_1411f6fa0", fn_1411f6fa0 as *const ()), // sub_1411f6fa0
    Replacement::draft(0x1411f7140, "unknown_1411f7140", fn_1411f7140 as *const ()), // sub_1411f7140
    Replacement::draft(0x1411f72a0, "unknown_1411f72a0", fn_1411f72a0 as *const ()), // sub_1411f72a0
    Replacement::draft(0x1411f7430, "unknown_1411f7430", tpl_1149::<0x8, 0x1411f7a70, 0x1401e66f0> as *const ()), // sub_1411f7430
    Replacement::draft_throws(0x1411f7460, "unknown_1411f7460", fn_1411f7460 as *const (), Throws::Caught), // sub_1411f7460
    Replacement::draft_throws(0x1411f7570, "unknown_1411f7570", fn_1411f7570 as *const (), Throws::Discard), // sub_1411f7570
    Replacement::draft(0x1411f76e0, "unknown_1411f76e0", fn_1411f76e0 as *const ()), // sub_1411f76e0
    Replacement::draft(0x1411f79c0, "unknown_1411f79c0", tpl_20752::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x8, 0x14024d8b0> as *const ()), // sub_1411f79c0
    Replacement::draft(0x1411f7a70, "unknown_1411f7a70", tpl_60956::<0x19, 0x10, 0x1411f7a70, 0x20, 0x1411f8c40, 0x1401e66f0> as *const ()), // sub_1411f7a70
    Replacement::draft(0x1411f7af0, "unknown_1411f7af0", fn_1411f7af0 as *const ()), // sub_1411f7af0
    Replacement::draft(0x1411f7b70, "unknown_1411f7b70", fn_1411f7b70 as *const ()), // sub_1411f7b70
    Replacement::draft(0x1411f7c30, "unknown_1411f7c30", fn_1411f7c30 as *const ()), // sub_1411f7c30
    Replacement::draft(0x1411f7ca0, "unknown_1411f7ca0", fn_1411f7ca0 as *const ()), // sub_1411f7ca0
    Replacement::draft(0x1411f7d70, "unknown_1411f7d70", fn_1411f7d70 as *const ()), // sub_1411f7d70
    Replacement::draft(0x1411f7fc0, "unknown_1411f7fc0", fn_1411f7fc0 as *const ()), // sub_1411f7fc0
    Replacement::draft(0x1411f8170, "unknown_1411f8170", tpl_19447::<0x1411f96b0> as *const ()), // sub_1411f8170
    Replacement::draft(0x1411f82d0, "unknown_1411f82d0", fn_1411f82d0 as *const ()), // sub_1411f82d0
    Replacement::draft(0x1411f8610, "unknown_1411f8610", fn_1411f8610 as *const ()), // sub_1411f8610
    Replacement::draft(0x1411f8820, "unknown_1411f8820", fn_1411f8820 as *const ()), // sub_1411f8820
    Replacement::draft(0x1411f8c40, "unknown_1411f8c40", fn_1411f8c40 as *const ()), // sub_1411f8c40
    Replacement::draft(0x1411f8ca0, "unknown_1411f8ca0", fn_1411f8ca0 as *const ()), // sub_1411f8ca0
    Replacement::draft(0x1411f8ea0, "unknown_1411f8ea0", fn_1411f8ea0 as *const ()), // sub_1411f8ea0
    Replacement::draft(0x1411f90d0, "unknown_1411f90d0", fn_1411f90d0 as *const ()), // sub_1411f90d0
    Replacement::draft(0x1411f92e0, "unknown_1411f92e0", fn_1411f92e0 as *const ()), // sub_1411f92e0
    Replacement::draft(0x1411f93c0, "unknown_1411f93c0", fn_1411f93c0 as *const ()), // sub_1411f93c0
    Replacement::draft(0x1411f9430, "unknown_1411f9430", fn_1411f9430 as *const ()), // sub_1411f9430
    Replacement::draft(0x1411f9640, "unknown_1411f9640", fn_1411f9640 as *const ()), // sub_1411f9640
    Replacement::draft(0x1411f96b0, "unknown_1411f96b0", tpl_381::<0x140fa31b0, 0x1411f9810> as *const ()), // sub_1411f96b0
    Replacement::draft(0x1411f9810, "unknown_1411f9810", fn_1411f9810 as *const ()), // sub_1411f9810
    Replacement::draft(0x1411f9af0, "luau_1411f9af0", fn_1411f9af0 as *const ()), // sub_1411f9af0
    Replacement::draft(0x1411f9b10, "luau_1411f9b10", fn_1411f9b10 as *const ()), // sub_1411f9b10
    Replacement::draft(0x1411f9b80, "luau_1411f9b80", fn_1411f9b80 as *const ()), // sub_1411f9b80
    Replacement::draft_throws(0x1411f9ba0, "luau_1411f9ba0", fn_1411f9ba0 as *const (), Throws::Discard), // lua_exception::lua_exception::vfunc_1
    Replacement::draft(0x1411f9cc0, "luau_1411f9cc0", fn_1411f9cc0 as *const ()), // lua_exception::lua_exception::vfunc_0
    Replacement::draft(0x1411f9d10, "unknown_1411f9d10", fn_1411f9d10 as *const ()), // sub_1411f9d10
    Replacement::draft(0x1411f9d70, "kaim_1411f9d70", tpl_22067::<0x14169ed80, 0x141834d30> as *const ()), // Kaim::RefCountImplCore::Kaim::RefCountImplCore::vfunc_0
    Replacement::draft(0x1411f9db0, "kaim_1411f9db0", tpl_22425::<0x8, 0x143932190, 0x60> as *const ()), // sub_1411f9db0
    Replacement::draft(0x1411f9de0, "kaim_1411f9de0", tpl_22426::<0x1433e1380, 0x14169ed80> as *const ()), // sub_1411f9de0
    Replacement::draft(0x1411f9df0, "kaim_1411f9df0", fn_1411f9df0 as *const ()), // Kaim::WorldElement::Kaim::WorldElement::vfunc_0
    Replacement::draft(0x1411f9e60, "kaim_1411f9e60", tpl_365::<0x14169ed80> as *const ()), // sub_1411f9e60
    Replacement::draft(0x1411f9e70, "kaim_1411f9e70", tpl_22428::<0x143932190, 0x60> as *const ()), // sub_1411f9e70
    Replacement::draft(0x1411f9e90, "kaim_1411f9e90", fn_1411f9e90 as *const ()), // sub_1411f9e90
    Replacement::draft(0x1411f9eb0, "kaim_1411f9eb0", tpl_1::<0x1433e1360> as *const ()), // Kaim::NavData::Kaim::NavData::vfunc_2
    Replacement::draft(0x1411f9ec0, "kaim_1411f9ec0", fn_1411f9ec0 as *const ()), // Kaim::NavData::Kaim::NavData::vfunc_0
    Replacement::draft(0x1411f9fa0, "kaim_1411f9fa0", fn_1411f9fa0 as *const ()), // sub_1411f9fa0
    Replacement::draft(0x1411fa000, "kaim_1411fa000", fn_1411fa000 as *const ()), // sub_1411fa000
    Replacement::draft(0x1411fa100, "navigation_1411fa100", tpl_1995::<0x1433e13a8, 0x1401de830, 0x141834d30> as *const ()), // navigation::WEDNavmeshData::navigation::WEDNavmeshData::vfunc_0
    Replacement::draft(0x1411fa150, "unknown_1411fa150", tpl_424::<0x14169edb0> as *const ()), // sub_1411fa150
    Replacement::draft(0x1411fa170, "unknown_1411fa170", fn_1411fa170 as *const ()), // sub_1411fa170
    Replacement::draft(0x1411fa3d0, "unknown_1411fa3d0", fn_1411fa3d0 as *const ()), // sub_1411fa3d0
    Replacement::draft(0x1411fa420, "unknown_1411fa420", tpl_424::<0x14169edd0> as *const ()), // sub_1411fa420
    Replacement::draft(0x1411fa440, "unknown_1411fa440", tpl_22435::<0x1411fa4d0> as *const ()), // sub_1411fa440
    Replacement::draft(0x1411fa450, "unknown_1411fa450", fn_1411fa450 as *const ()), // sub_1411fa450
    Replacement::draft(0x1411fa4c0, "unknown_1411fa4c0", tpl_22435::<0x1411fa550> as *const ()), // sub_1411fa4c0
    Replacement::draft(0x1411fa4d0, "unknown_1411fa4d0", tpl_22437::<0x10, 0x8, 0x1411fa5f0> as *const ()), // sub_1411fa4d0
    Replacement::draft(0x1411fa550, "unknown_1411fa550", fn_1411fa550 as *const ()), // sub_1411fa550
    Replacement::draft(0x1411fa5f0, "unknown_1411fa5f0", fn_1411fa5f0 as *const ()), // sub_1411fa5f0
    Replacement::draft(0x1411fa680, "unknown_1411fa680", fn_1411fa680 as *const ()), // sub_1411fa680
    Replacement::draft(0x1411fa710, "unknown_1411fa710", fn_1411fa710 as *const ()), // sub_1411fa710
    Replacement::draft(0x1411fabd0, "kaim_1411fabd0", tpl_22067::<0x14169ed90, 0x141834d30> as *const ()), // Kaim::RefCountNTSImplCore::Kaim::RefCountNTSImplCore::vfunc_0
    Replacement::draft(0x1411fac10, "kaim_1411fac10", tpl_22439::<0x1433e1408, 0x14169ed90, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::BlobTypeDescriptor<Kaim::NavGraphBlob>::Kaim::BlobTypeDescriptor<Kaim::NavGraphBlob>::vfunc_0
    Replacement::draft(0x1411fac80, "kaim_1411fac80", fn_1411fac80 as *const ()), // sub_1411fac80
    Replacement::draft(0x1411fad50, "navigation_1411fad50", fn_1411fad50 as *const ()), // navigation::NavigationSmartObject::navigation::NavigationSmartObject::vfunc_0
    Replacement::draft(0x1411fada0, "kaim_1411fada0", fn_1411fada0 as *const ()), // sub_1411fada0
    Replacement::draft(0x1411fae70, "kaim_1411fae70", tpl_365::<0x1416a3160> as *const ()), // sub_1411fae70
    Replacement::draft(0x1411fae80, "kaim_1411fae80", tpl_22443::<0x8, 0x143932190, 0x60> as *const ()), // sub_1411fae80
    Replacement::draft(0x1411faf00, "kaim_1411faf00", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1433e14a8> as *const ()), // Kaim::BlobHandler<Kaim::NavGraphBlob>::Kaim::BlobHandler<Kaim::NavGraphBlob>::vfunc_6
    Replacement::draft(0x1411faf60, "kaim_1411faf60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1433e1468, 0x8> as *const ()), // Kaim::BlobHandler<Kaim::NavGraphBlob>::Kaim::BlobHandler<Kaim::NavGraphBlob>::vfunc_5
    Replacement::draft(0x1411fafa0, "kaim_1411fafa0", tpl_22446::<0x1411fac80> as *const ()), // Kaim::BlobHandler<Kaim::NavGraphBlob>::Kaim::BlobHandler<Kaim::NavGraphBlob>::vfunc_4
    Replacement::draft(0x1411fafb0, "kaim_1411fafb0", tpl_1::<0x1433e13d8> as *const ()), // Kaim::BlobHandler<Kaim::NavGraphBlob>::Kaim::BlobHandler<Kaim::NavGraphBlob>::vfunc_2
    Replacement::draft(0x1411fafc0, "kaim_1411fafc0", tpl_22443::<0x8, 0x143932190, 0x60> as *const ()), // sub_1411fafc0
    Replacement::draft(0x1411fb040, "kaim_1411fb040", tpl_1674::<0x1433e13f0> as *const ()), // sub_1411fb040
    Replacement::draft(0x1411fb050, "kaim_1411fb050", tpl_22428::<0x143932190, 0x60> as *const ()), // sub_1411fb050
    Replacement::draft(0x1411fb070, "kaim_1411fb070", fn_1411fb070 as *const ()), // Kaim::BlobHandler<Kaim::NavGraphBlob>::Kaim::BlobHandler<Kaim::NavGraphBlob>::vfunc_0
    Replacement::draft(0x1411fb0d0, "kaim_1411fb0d0", tpl_22448::<0x1433e13f0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::BaseBlobBuilder<Kaim::NavGraphBlob>::Kaim::BaseBlobBuilder<Kaim::NavGraphBlob>::vfunc_0
    Replacement::draft(0x1411fb120, "kaim_1411fb120", tpl_22449::<0x18, 0x143932190, 0x60> as *const ()), // sub_1411fb120
    Replacement::draft(0x1411fb140, "kaim_1411fb140", fn_1411fb140 as *const ()), // sub_1411fb140
    Replacement::draft(0x1411fb250, "kaim_1411fb250", fn_1411fb250 as *const ()), // sub_1411fb250
    Replacement::draft(0x1411fb3e0, "kaim_1411fb3e0", fn_1411fb3e0 as *const ()), // sub_1411fb3e0
    Replacement::draft(0x1411fb490, "kaim_1411fb490", fn_1411fb490 as *const ()), // sub_1411fb490
    Replacement::draft(0x1411fb5b0, "kaim_1411fb5b0", fn_1411fb5b0 as *const ()), // sub_1411fb5b0
    Replacement::draft(0x1411fb640, "kaim_1411fb640", fn_1411fb640 as *const ()), // sub_1411fb640
    Replacement::draft(0x1411fb690, "kaim_1411fb690", fn_1411fb690 as *const ()), // sub_1411fb690
    Replacement::draft(0x1411fb6c0, "kaim_1411fb6c0", tpl_22454::<0x143932190, 0x60> as *const ()), // sub_1411fb6c0
    Replacement::draft(0x1411fb6f0, "kaim_1411fb6f0", fn_1411fb6f0 as *const ()), // sub_1411fb6f0
    Replacement::draft(0x1411fb750, "kaim_1411fb750", fn_1411fb750 as *const ()), // sub_1411fb750
    Replacement::draft(0x1411fb7b0, "kaim_1411fb7b0", fn_1411fb7b0 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x1411fb7c0, "kaim_1411fb7c0", fn_1411fb7c0 as *const ()), // Kaim::IQuery::Kaim::IQuery::vfunc_0
    Replacement::draft(0x1411fb840, "kaim_1411fb840", fn_1411fb840 as *const ()), // sub_1411fb840
    Replacement::draft(0x1411fb920, "kaim_1411fb920", fn_1411fb920 as *const ()), // sub_1411fb920
    Replacement::draft(0x1411fb950, "kaim_1411fb950", fn_1411fb950 as *const ()), // sub_1411fb950
    Replacement::draft(0x1411fb9b0, "kaim_1411fb9b0", fn_1411fb9b0 as *const ()), // sub_1411fb9b0
    Replacement::draft(0x1411fbab0, "kaim_1411fbab0", fn_1411fbab0 as *const ()), // sub_1411fbab0
    Replacement::draft(0x1411fbb40, "kaim_1411fbb40", tpl_22464::<0x1411fbab0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::TriangleFromPosAndTriangleSeedQuery::Kaim::TriangleFromPosAndTriangleSeedQuery::vfunc_0
    Replacement::draft(0x1411fbba0, "kaim_1411fbba0", tpl_22465::<0x1416a4ee0, 0x10> as *const ()), // Kaim::TriangleFromPosAndTriangleSeedQuery::Kaim::TriangleFromPosAndTriangleSeedQuery::vfunc_2
    Replacement::draft(0x1411fbbc0, "kaim_1411fbbc0", tpl_22466::<0x1416b1f20> as *const ()), // Kaim::RayCastQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::RayCastQuery<Kaim::Loc
    Replacement::draft(0x1411fbbe0, "kaim_1411fbbe0", fn_1411fbbe0 as *const ()), // sub_1411fbbe0
    Replacement::draft(0x1411fbcd0, "kaim_1411fbcd0", tpl_22468::<0x1411fbbe0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::BaseRayCastQuery::Kaim::BaseRayCastQuery::vfunc_0
    Replacement::draft(0x1411fbd30, "kaim_1411fbd30", fn_1411fbd30 as *const ()), // sub_1411fbd30
    Replacement::draft(0x1411fbda0, "kaim_1411fbda0", fn_1411fbda0 as *const ()), // sub_1411fbda0
    Replacement::draft(0x1411fbe40, "kaim_1411fbe40", tpl_22439::<0x1433e1650, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::IPathEventListObserver::Kaim::IPathEventListObserver::vfunc_0
    Replacement::draft(0x1411fbeb0, "kaim_1411fbeb0", tpl_22426::<0x1433e16c0, 0x14169ed80> as *const ()), // sub_1411fbeb0
    Replacement::draft(0x1411fbec0, "kaim_1411fbec0", tpl_22439::<0x1433e16c0, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::IAvoidanceSolver::Kaim::IAvoidanceSolver::vfunc_0
    Replacement::draft(0x1411fbf30, "kaim_1411fbf30", tpl_22439::<0x1433e1760, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::IAvoidanceFilter::Kaim::IAvoidanceFilter::vfunc_0
    Replacement::draft(0x1411fbfa0, "kaim_1411fbfa0", tpl_22466::<0x1416ac3e0> as *const ()), // Kaim::DiskCollisionQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::DiskCollisionQue
    Replacement::draft(0x1411fbfc0, "kaim_1411fbfc0", tpl_22471::<0x1433e14e8, 0x90, 0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60, 0x68, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_1411fbfc0
    Replacement::draft(0x1411fc060, "kaim_1411fc060", fn_1411fc060 as *const ()), // Kaim::DiskCollisionQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::DiskCollisionQue
    Replacement::draft(0x1411fc0c0, "kaim_1411fc0c0", tpl_20961::<0x1416adae0> as *const ()), // sub_1411fc0c0
    Replacement::draft(0x1411fc0d0, "navigation_1411fc0d0", fn_1411fc0d0 as *const ()), // navigation::NoChanelAvoidanceSolver<navigation::NavigationTraverseLogic>::navigation::NoChanelAvoidanceSolver<navigation
    Replacement::draft(0x1411fc170, "kaim_1411fc170", fn_1411fc170 as *const ()), // sub_1411fc170
    Replacement::draft(0x1411fc1c0, "kaim_1411fc1c0", fn_1411fc1c0 as *const ()), // sub_1411fc1c0
    Replacement::draft(0x1411fc240, "kaim_1411fc240", tpl_22476::<0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60> as *const ()), // sub_1411fc240
    Replacement::draft(0x1411fc280, "kaim_1411fc280", tpl_22454::<0x143932190, 0x60> as *const ()), // sub_1411fc280
    Replacement::draft(0x1411fc2b0, "kaim_1411fc2b0", fn_1411fc2b0 as *const ()), // Kaim::AvoidanceSolver<navigation::NavigationTraverseLogic>::Kaim::AvoidanceSolver<navigation::NavigationTraverseLogic>::
    Replacement::draft(0x1411fc340, "kaim_1411fc340", tpl_22478::<0x1411fc470, 0x1411fc620, 0x1416b4e30> as *const ()), // Kaim::AvoidanceSolver<navigation::NavigationTraverseLogic>::Kaim::AvoidanceSolver<navigation::NavigationTraverseLogic>::
    Replacement::draft(0x1411fc3a0, "navigation_1411fc3a0", fn_1411fc3a0 as *const ()), // navigation::NoChanelAvoidanceSolver<navigation::NavigationTraverseLogic>::navigation::NoChanelAvoidanceSolver<navigation
    Replacement::draft(0x1411fc470, "kaim_1411fc470", fn_1411fc470 as *const ()), // sub_1411fc470
    Replacement::draft(0x1411fc620, "kaim_1411fc620", fn_1411fc620 as *const ()), // sub_1411fc620
    Replacement::draft(0x1411fc810, "kaim_1411fc810", tpl_365::<0x1411fbfc0> as *const ()), // sub_1411fc810
    Replacement::draft(0x1411fc820, "kaim_1411fc820", tpl_22482::<0xd0, 0x20, 0x68, 0x10, 0xa0, 0x6c, 0x48, 0x1416b1580, 0x1411fcfd0> as *const ()), // Kaim::RayCastQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::RayCastQuery<Kaim::Loc
    Replacement::draft(0x1411fc8d0, "kaim_1411fc8d0", tpl_22426::<0x1433e1688, 0x1411fbbe0> as *const ()), // sub_1411fc8d0
    Replacement::draft(0x1411fc8e0, "kaim_1411fc8e0", tpl_22483::<0x84, 0x20, 0x68, 0x10, 0xa0, 0x58, 0x5c, 0x48, 0x1416b1580, 0x1411fca00> as *const ()), // Kaim::DiskCollisionQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::DiskCollisionQue
    Replacement::draft(0x1411fc990, "kaim_1411fc990", tpl_22484::<0x1433e1688, 0x1411fbbe0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::RayCastQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::RayCastQuery<Kaim::Loc
    Replacement::draft(0x1411fca00, "kaim_1411fca00", fn_1411fca00 as *const ()), // sub_1411fca00
    Replacement::draft(0x1411fcfd0, "kaim_1411fcfd0", fn_1411fcfd0 as *const ()), // sub_1411fcfd0
    Replacement::draft(0x1411fd880, "kaim_1411fd880", fn_1411fd880 as *const ()), // sub_1411fd880
    Replacement::draft(0x1411fd8d0, "kaim_1411fd8d0", fn_1411fd8d0 as *const ()), // sub_1411fd8d0
    Replacement::draft(0x1411fd960, "kaim_1411fd960", tpl_22488::<0x143932190, 0x1416b1b10, 0x1411fe850, 0x1416ae230, 0x1411fb750, 0x1411fe6e0> as *const ()), // sub_1411fd960
    Replacement::draft(0x1411fdde0, "kaim_1411fdde0", fn_1411fdde0 as *const ()), // sub_1411fdde0
    Replacement::draft(0x1411fe110, "kaim_1411fe110", fn_1411fe110 as *const ()), // sub_1411fe110
    Replacement::draft(0x1411fe250, "kaim_1411fe250", fn_1411fe250 as *const ()), // sub_1411fe250
    Replacement::draft(0x1411fe6e0, "kaim_1411fe6e0", fn_1411fe6e0 as *const ()), // sub_1411fe6e0
    Replacement::draft(0x1411fe7b0, "kaim_1411fe7b0", fn_1411fe7b0 as *const ()), // sub_1411fe7b0
    Replacement::draft(0x1411fe850, "kaim_1411fe850", tpl_80829::<0x1416a4590, 0x1416a4620, 0x1416a4bc0, 0x140000000, 0x143428720, 0x1416b1580, 0x1411ff040, 0x1433e16f0, 0x143932190, 0x1433e1728, 0x14169edb0, 0x14169ed80> as *const ()), // sub_1411fe850
    Replacement::draft(0x1411feed0, "unknown_1411feed0", fn_1411feed0 as *const ()), // sub_1411feed0
    Replacement::draft(0x1411fef10, "unknown_1411fef10", fn_1411fef10 as *const ()), // sub_1411fef10
    Replacement::draft(0x1411ff040, "kaim_1411ff040", fn_1411ff040 as *const ()), // sub_1411ff040
    Replacement::draft(0x1411ff250, "unknown_1411ff250", fn_1411ff250 as *const ()), // sub_1411ff250
    Replacement::draft(0x1411ff370, "unknown_1411ff370", fn_1411ff370 as *const ()), // sub_1411ff370
    Replacement::draft(0x1411ff390, "unknown_1411ff390", fn_1411ff390 as *const ()), // sub_1411ff390
    Replacement::draft(0x1411ff4c0, "unknown_1411ff4c0", fn_1411ff4c0 as *const ()), // sub_1411ff4c0
    Replacement::draft(0x1411ff550, "unknown_1411ff550", fn_1411ff550 as *const ()), // sub_1411ff550
    Replacement::draft(0x1411ff580, "unknown_1411ff580", fn_1411ff580 as *const ()), // sub_1411ff580
    Replacement::draft(0x1411ff630, "unknown_1411ff630", fn_1411ff630 as *const ()), // sub_1411ff630
    Replacement::draft(0x1411ff7e0, "kaim_1411ff7e0", fn_1411ff7e0 as *const ()), // sub_1411ff7e0
    Replacement::draft(0x1411ff870, "kaim_1411ff870", fn_1411ff870 as *const ()), // sub_1411ff870
    Replacement::draft(0x1411ff8e0, "kaim_1411ff8e0", fn_1411ff8e0 as *const ()), // Kaim::Channel::Kaim::Channel::vfunc_0
    Replacement::draft(0x1411ff9f0, "kaim_1411ff9f0", fn_1411ff9f0 as *const ()), // sub_1411ff9f0
    Replacement::draft(0x1411ffa60, "kaim_1411ffa60", fn_1411ffa60 as *const ()), // sub_1411ffa60
    Replacement::draft(0x1411ffae0, "kaim_1411ffae0", tpl_22508::<0x1416d5970, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::ChannelArray::Kaim::ChannelArray::vfunc_0
    Replacement::draft(0x1411ffb40, "kaim_1411ffb40", fn_1411ffb40 as *const ()), // sub_1411ffb40
    Replacement::draft(0x1411ffb80, "kaim_1411ffb80", fn_1411ffb80 as *const ()), // sub_1411ffb80
    Replacement::draft(0x1411ffbc0, "kaim_1411ffbc0", fn_1411ffbc0 as *const ()), // sub_1411ffbc0
    Replacement::draft(0x1411ffbf0, "kaim_1411ffbf0", fn_1411ffbf0 as *const ()), // sub_1411ffbf0
    Replacement::draft(0x1411ffc30, "kaim_1411ffc30", fn_1411ffc30 as *const ()), // sub_1411ffc30
    Replacement::draft(0x1411ffc90, "kaim_1411ffc90", tpl_22514::<0x8> as *const ()), // sub_1411ffc90
    Replacement::draft(0x1411ffcb0, "kaim_1411ffcb0", fn_1411ffcb0 as *const ()), // sub_1411ffcb0
    Replacement::draft(0x1411ffcf0, "kaim_1411ffcf0", fn_1411ffcf0 as *const ()), // sub_1411ffcf0
    Replacement::draft(0x1411ffd50, "kaim_1411ffd50", fn_1411ffd50 as *const ()), // sub_1411ffd50
    Replacement::draft(0x1411ffdb0, "kaim_1411ffdb0", fn_1411ffdb0 as *const ()), // sub_1411ffdb0
    Replacement::draft(0x1411ffe10, "kaim_1411ffe10", fn_1411ffe10 as *const ()), // sub_1411ffe10
    Replacement::draft(0x1411ffe70, "kaim_1411ffe70", fn_1411ffe70 as *const ()), // sub_1411ffe70
    Replacement::draft(0x1411ffeb0, "kaim_1411ffeb0", tpl_22521::<0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_1411ffeb0
    Replacement::draft(0x1411ffee0, "kaim_1411ffee0", fn_1411ffee0 as *const ()), // sub_1411ffee0
    Replacement::draft(0x1411fff80, "kaim_1411fff80", fn_1411fff80 as *const ()), // sub_1411fff80
    Replacement::draft(0x141200050, "kaim_141200050", tpl_22523::<0xe8, 0x1416a7360> as *const ()), // sub_141200050
    Replacement::draft(0x141200090, "kaim_141200090", tpl_22523::<0xf0, 0x1416a7360> as *const ()), // sub_141200090
    Replacement::draft(0x1412000d0, "kaim_1412000d0", tpl_22523::<0xf8, 0x1416a7360> as *const ()), // sub_1412000d0
    Replacement::draft(0x141200110, "kaim_141200110", tpl_22523::<0x108, 0x1416a7590> as *const ()), // sub_141200110
    Replacement::draft(0x141200150, "kaim_141200150", fn_141200150 as *const ()), // sub_141200150
    Replacement::draft(0x141200230, "kaim_141200230", tpl_22466::<0x1416d46f0> as *const ()), // Kaim::TriangleFromPosQuery::Kaim::TriangleFromPosQuery::vfunc_3
    Replacement::draft(0x141200250, "kaim_141200250", tpl_22524::<0x1433e1970, 0x70, 0x143932190, 0x60, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_141200250
    Replacement::draft(0x1412002b0, "kaim_1412002b0", tpl_22465::<0x1416d56f0, 0x10> as *const ()), // Kaim::TriangleFromPosQuery::Kaim::TriangleFromPosQuery::vfunc_2
    Replacement::draft(0x1412002d0, "kaim_1412002d0", tpl_22525::<0x141200250, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::TriangleFromPosQuery::Kaim::TriangleFromPosQuery::vfunc_0
    Replacement::draft(0x141200330, "kaim_141200330", fn_141200330 as *const ()), // Kaim::RayCanGoOnSegmentQuery<navigation::NavigationTraverseLogic>::Kaim::RayCanGoOnSegmentQuery<navigation::NavigationTr
    Replacement::draft(0x141200340, "kaim_141200340", tpl_22527::<0x1433e1d10, 0xd8, 0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60, 0xb8, 0xa0, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_141200340
    Replacement::draft(0x141200410, "kaim_141200410", tpl_22468::<0x141200340, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::RayCanGoOnSegmentQuery<navigation::NavigationTraverseLogic>::Kaim::RayCanGoOnSegmentQuery<navigation::NavigationTr
    Replacement::draft(0x141200470, "kaim_141200470", fn_141200470 as *const ()), // Kaim::RayCastOnSegmentQuery<navigation::NavigationTraverseLogic>::Kaim::RayCastOnSegmentQuery<navigation::NavigationTrav
    Replacement::draft(0x141200480, "kaim_141200480", fn_141200480 as *const ()), // sub_141200480
    Replacement::draft(0x141200570, "kaim_141200570", fn_141200570 as *const ()), // Kaim::BaseRayCastOnSegmentQuery::Kaim::BaseRayCastOnSegmentQuery::vfunc_0
    Replacement::draft(0x1412005d0, "kaim_1412005d0", fn_1412005d0 as *const ()), // sub_1412005d0
    Replacement::draft(0x141200610, "kaim_141200610", fn_141200610 as *const ()), // sub_141200610
    Replacement::draft(0x141200690, "kaim_141200690", fn_141200690 as *const ()), // sub_141200690
    Replacement::draft(0x141200710, "kaim_141200710", tpl_22525::<0x141200610, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::IPathFinderQuery::Kaim::IPathFinderQuery::vfunc_0
    Replacement::draft(0x141200770, "kaim_141200770", fn_141200770 as *const ()), // sub_141200770
    Replacement::draft(0x141200800, "kaim_141200800", fn_141200800 as *const ()), // sub_141200800
    Replacement::draft(0x141200870, "kaim_141200870", fn_141200870 as *const ()), // sub_141200870
    Replacement::draft(0x141200900, "kaim_141200900", fn_141200900 as *const ()), // sub_141200900
    Replacement::draft(0x141200980, "kaim_141200980", tpl_22466::<0x1416dc7b0> as *const ()), // Kaim::RayCanGoQuery<Kaim::anonymous_namespace::ExtendedPredicate<navigation::NavigationTraverseLogic>>::Kaim::RayCanGoQu
    Replacement::draft(0x1412009a0, "kaim_1412009a0", tpl_22527::<0x1433e19e0, 0xc8, 0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60, 0x88, 0x58, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_1412009a0
    Replacement::draft(0x141200a70, "kaim_141200a70", tpl_22538::<0x1412009a0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::MonodirectionalRayCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::MonodirectionalRayCanGoQuery<navigation::
    Replacement::draft(0x141200ad0, "kaim_141200ad0", fn_141200ad0 as *const ()), // sub_141200ad0
    Replacement::draft(0x141200c10, "kaim_141200c10", tpl_22466::<0x1416db890> as *const ()), // Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x141200c30, "kaim_141200c30", tpl_22527::<0x1433e1c78, 0xc0, 0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60, 0x88, 0x58, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_141200c30
    Replacement::draft(0x141200d00, "kaim_141200d00", tpl_22540::<0x141200c30, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x141200d60, "kaim_141200d60", tpl_22466::<0x1416dec70> as *const ()), // Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::vfun
    Replacement::draft(0x141200d80, "kaim_141200d80", fn_141200d80 as *const ()), // sub_141200d80
    Replacement::draft(0x141200e70, "kaim_141200e70", fn_141200e70 as *const ()), // Kaim::BaseDiskCastQuery::Kaim::BaseDiskCastQuery::vfunc_0
    Replacement::draft(0x141200ed0, "kaim_141200ed0", tpl_22466::<0x1416e5e20> as *const ()), // Kaim::NearestBorderHalfEdgeFromPosQuery<navigation::NavigationTraverseLogic>::Kaim::NearestBorderHalfEdgeFromPosQuery<na
    Replacement::draft(0x141200ef0, "kaim_141200ef0", fn_141200ef0 as *const ()), // sub_141200ef0
    Replacement::draft(0x141200fb0, "kaim_141200fb0", tpl_22540::<0x141200ef0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::NearestBorderHalfEdgeFromPosQuery<navigation::NavigationTraverseLogic>::Kaim::NearestBorderHalfEdgeFromPosQuery<na
    Replacement::draft(0x141201010, "kaim_141201010", fn_141201010 as *const ()), // Kaim::InsidePosFromOutsidePosQuery<navigation::NavigationTraverseLogic>::Kaim::InsidePosFromOutsidePosQuery<navigation::
    Replacement::draft(0x141201020, "kaim_141201020", tpl_22466::<0x1416d3bd0> as *const ()), // Kaim::InsidePosFromOutsidePosQuery<navigation::NavigationTraverseLogic>::Kaim::InsidePosFromOutsidePosQuery<navigation::
    Replacement::draft(0x141201040, "kaim_141201040", tpl_22524::<0x1433e1b50, 0xa0, 0x143932190, 0x60, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_141201040
    Replacement::draft(0x1412010b0, "kaim_1412010b0", fn_1412010b0 as *const ()), // Kaim::InsidePosFromOutsidePosQuery<navigation::NavigationTraverseLogic>::Kaim::InsidePosFromOutsidePosQuery<navigation::
    Replacement::draft(0x141201110, "kaim_141201110", tpl_22466::<0x1416d6350> as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_3
    Replacement::draft(0x141201130, "kaim_141201130", tpl_548::<0xc0> as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_7
    Replacement::draft(0x141201140, "kaim_141201140", fn_141201140 as *const ()), // sub_141201140
    Replacement::draft(0x1412012b0, "kaim_1412012b0", fn_1412012b0 as *const ()), // Kaim::BaseAStarQuery::Kaim::BaseAStarQuery::vfunc_0
    Replacement::draft(0x141201310, "kaim_141201310", tpl_22449::<0x38, 0x143932190, 0x60> as *const ()), // sub_141201310
    Replacement::draft(0x141201330, "kaim_141201330", fn_141201330 as *const ()), // sub_141201330
    Replacement::draft(0x141201450, "kaim_141201450", fn_141201450 as *const ()), // sub_141201450
    Replacement::draft(0x141201600, "kaim_141201600", fn_141201600 as *const ()), // sub_141201600
    Replacement::draft(0x1412017f0, "kaim_1412017f0", fn_1412017f0 as *const ()), // sub_1412017f0
    Replacement::draft(0x141201850, "kaim_141201850", fn_141201850 as *const ()), // sub_141201850
    Replacement::draft(0x141201940, "kaim_141201940", fn_141201940 as *const ()), // sub_141201940
    Replacement::draft(0x141201a50, "kaim_141201a50", fn_141201a50 as *const ()), // sub_141201a50
    Replacement::draft(0x141201b00, "kaim_141201b00", fn_141201b00 as *const ()), // sub_141201b00
    Replacement::draft(0x141201c30, "kaim_141201c30", fn_141201c30 as *const ()), // sub_141201c30
    Replacement::draft(0x141201cc0, "kaim_141201cc0", fn_141201cc0 as *const ()), // sub_141201cc0
    Replacement::draft(0x141201d40, "kaim_141201d40", fn_141201d40 as *const ()), // sub_141201d40
    Replacement::draft(0x141201d80, "kaim_141201d80", fn_141201d80 as *const ()), // sub_141201d80
    Replacement::draft(0x141201db0, "kaim_141201db0", fn_141201db0 as *const ()), // sub_141201db0
    Replacement::draft(0x141201f00, "kaim_141201f00", fn_141201f00 as *const ()), // sub_141201f00
    Replacement::draft(0x141201f80, "kaim_141201f80", fn_141201f80 as *const ()), // sub_141201f80
    Replacement::draft(0x141202070, "kaim_141202070", tpl_365::<0x141201040> as *const ()), // sub_141202070
    Replacement::draft(0x141202080, "kaim_141202080", tpl_365::<0x141200340> as *const ()), // sub_141202080
    Replacement::draft(0x141202090, "kaim_141202090", tpl_365::<0x141200c30> as *const ()), // sub_141202090
    Replacement::draft(0x1412020a0, "kaim_1412020a0", tpl_80893::<0xbc, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x141205050> as *const ()), // Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x141202190, "kaim_141202190", tpl_22483::<0x84, 0x20, 0x68, 0x10, 0xa0, 0x58, 0x5c, 0x48, 0x1416b1580, 0x141204a20> as *const ()), // Kaim::DiskCollisionQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCollisionQuery<navigation::NavigationTraverseLo
    Replacement::draft(0x141202240, "kaim_141202240", tpl_22465::<0x141202b10, 0x10> as *const ()), // Kaim::RayCanGoOnSegmentQuery<navigation::NavigationTraverseLogic>::Kaim::RayCanGoOnSegmentQuery<navigation::NavigationTr
    Replacement::draft(0x141202260, "kaim_141202260", tpl_22426::<0x1433e1a70, 0x141200480> as *const ()), // sub_141202260
    Replacement::draft(0x141202270, "kaim_141202270", fn_141202270 as *const ()), // Kaim::InsidePosFromOutsidePosQuery<navigation::NavigationTraverseLogic>::Kaim::InsidePosFromOutsidePosQuery<navigation::
    Replacement::draft(0x141202320, "kaim_141202320", tpl_22563::<0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60> as *const ()), // sub_141202320
    Replacement::draft(0x1412023a0, "kaim_1412023a0", tpl_22563::<0x78, 0xffffffffffffffff, 0x1416daf20, 0x143932190, 0x60> as *const ()), // sub_1412023a0
    Replacement::draft(0x141202420, "kaim_141202420", tpl_22564::<0x78, 0xffffffffffffffff, 0x1416daf20, 0x143932190, 0x60> as *const ()), // sub_141202420
    Replacement::draft(0x141202490, "kaim_141202490", tpl_22476::<0x78, 0xffffffffffffffff, 0x1416daf20, 0x143932190, 0x60> as *const ()), // sub_141202490
    Replacement::draft(0x1412024d0, "kaim_1412024d0", fn_1412024d0 as *const ()), // sub_1412024d0
    Replacement::draft(0x141202520, "kaim_141202520", fn_141202520 as *const ()), // sub_141202520
    Replacement::draft(0x141202550, "kaim_141202550", fn_141202550 as *const ()), // sub_141202550
    Replacement::draft(0x1412025a0, "kaim_1412025a0", fn_1412025a0 as *const ()), // sub_1412025a0
    Replacement::draft(0x141202610, "kaim_141202610", tpl_22569::<0x1433e1d48, 0x141201140, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_0
    Replacement::draft(0x141202680, "kaim_141202680", tpl_22570::<0x1433e1a70, 0x141200480, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::RayCastOnSegmentQuery<navigation::NavigationTraverseLogic>::Kaim::RayCastOnSegmentQuery<navigation::NavigationTrav
    Replacement::draft(0x1412026f0, "kaim_1412026f0", fn_1412026f0 as *const ()), // sub_1412026f0
    Replacement::draft(0x1412027a0, "kaim_1412027a0", fn_1412027a0 as *const ()), // sub_1412027a0
    Replacement::draft(0x141202860, "kaim_141202860", fn_141202860 as *const ()), // sub_141202860
    Replacement::draft(0x1412028f0, "kaim_1412028f0", fn_1412028f0 as *const ()), // sub_1412028f0
    Replacement::draft(0x141202980, "kaim_141202980", fn_141202980 as *const ()), // sub_141202980
    Replacement::draft(0x141202a40, "kaim_141202a40", fn_141202a40 as *const ()), // sub_141202a40
    Replacement::draft(0x141202b10, "kaim_141202b10", fn_141202b10 as *const ()), // sub_141202b10
    Replacement::draft(0x141202c10, "kaim_141202c10", fn_141202c10 as *const ()), // sub_141202c10
    Replacement::draft(0x141202cb0, "kaim_141202cb0", tpl_22574::<0x141203aa0, 0x80, 0x48, 0x10, 0x1a0> as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_2
    Replacement::draft(0x141202cf0, "kaim_141202cf0", tpl_22575::<0x20, 0xa0, 0x1a8, 0x141204490> as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_6
    Replacement::draft(0x141202d10, "kaim_141202d10", tpl_22465::<0x141204920, 0x10> as *const ()), // Kaim::RayCastOnSegmentQuery<navigation::NavigationTraverseLogic>::Kaim::RayCastOnSegmentQuery<navigation::NavigationTrav
    Replacement::draft(0x141202d30, "kaim_141202d30", fn_141202d30 as *const ()), // sub_141202d30
    Replacement::draft(0x141202e10, "kaim_141202e10", fn_141202e10 as *const ()), // sub_141202e10
    Replacement::draft(0x141202fe0, "kaim_141202fe0", fn_141202fe0 as *const ()), // sub_141202fe0
    Replacement::draft(0x141203aa0, "kaim_141203aa0", fn_141203aa0 as *const ()), // sub_141203aa0
    Replacement::draft(0x141204490, "kaim_141204490", fn_141204490 as *const ()), // sub_141204490
    Replacement::draft(0x141204520, "kaim_141204520", fn_141204520 as *const ()), // sub_141204520
    Replacement::draft(0x141204920, "kaim_141204920", fn_141204920 as *const ()), // sub_141204920
    Replacement::draft(0x141204a20, "kaim_141204a20", fn_141204a20 as *const ()), // sub_141204a20
    Replacement::draft(0x141205050, "kaim_141205050", fn_141205050 as *const ()), // sub_141205050
    Replacement::draft(0x141205a20, "kaim_141205a20", tpl_365::<0x1412009a0> as *const ()), // sub_141205a20
    Replacement::draft(0x141205a30, "kaim_141205a30", tpl_80893::<0xbc, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x14120e270> as *const ()), // Kaim::RayCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::RayCanGoQuery<navigation::NavigationTraverseLogic>::vfun
    Replacement::draft(0x141205b20, "kaim_141205b20", tpl_22482::<0xd0, 0x20, 0x68, 0x10, 0xa0, 0x6c, 0x48, 0x1416b1580, 0x141208760> as *const ()), // Kaim::RayCastQuery<navigation::NavigationTraverseLogic>::Kaim::RayCastQuery<navigation::NavigationTraverseLogic>::vfunc_
    Replacement::draft(0x141205bd0, "kaim_141205bd0", tpl_22426::<0x1433e1de0, 0x1411fbbe0> as *const ()), // sub_141205bd0
    Replacement::draft(0x141205be0, "kaim_141205be0", tpl_22484::<0x1433e1de0, 0x1411fbbe0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::RayCastQuery<navigation::NavigationTraverseLogic>::Kaim::RayCastQuery<navigation::NavigationTraverseLogic>::vfunc_
    Replacement::draft(0x141205c50, "kaim_141205c50", fn_141205c50 as *const ()), // sub_141205c50
    Replacement::draft(0x141205ea0, "kaim_141205ea0", fn_141205ea0 as *const ()), // sub_141205ea0
    Replacement::draft(0x141206160, "kaim_141206160", fn_141206160 as *const ()), // sub_141206160
    Replacement::draft(0x141206390, "kaim_141206390", fn_141206390 as *const ()), // sub_141206390
    Replacement::draft(0x141206690, "kaim_141206690", fn_141206690 as *const ()), // sub_141206690
    Replacement::draft(0x141206720, "kaim_141206720", fn_141206720 as *const ()), // sub_141206720
    Replacement::draft(0x1412067b0, "kaim_1412067b0", fn_1412067b0 as *const ()), // sub_1412067b0
    Replacement::draft(0x1412069e0, "kaim_1412069e0", fn_1412069e0 as *const ()), // sub_1412069e0
    Replacement::draft(0x141206f30, "kaim_141206f30", fn_141206f30 as *const ()), // sub_141206f30
    Replacement::draft(0x1412071f0, "kaim_1412071f0", fn_1412071f0 as *const ()), // sub_1412071f0
    Replacement::draft(0x141207f70, "kaim_141207f70", fn_141207f70 as *const ()), // sub_141207f70
    Replacement::draft(0x141208760, "kaim_141208760", fn_141208760 as *const ()), // sub_141208760
    Replacement::draft(0x1412093b0, "kaim_1412093b0", fn_1412093b0 as *const ()), // sub_1412093b0
    Replacement::draft(0x141209b60, "kaim_141209b60", fn_141209b60 as *const ()), // sub_141209b60
    Replacement::draft(0x141209e50, "kaim_141209e50", fn_141209e50 as *const ()), // sub_141209e50
    Replacement::draft(0x14120c190, "kaim_14120c190", fn_14120c190 as *const ()), // sub_14120c190
    Replacement::draft(0x14120d280, "kaim_14120d280", fn_14120d280 as *const ()), // sub_14120d280
    Replacement::draft(0x14120dc10, "kaim_14120dc10", fn_14120dc10 as *const ()), // sub_14120dc10
    Replacement::draft(0x14120df40, "kaim_14120df40", fn_14120df40 as *const ()), // sub_14120df40
    Replacement::draft(0x14120e270, "kaim_14120e270", fn_14120e270 as *const ()), // sub_14120e270
    Replacement::draft(0x14120efe0, "kaim_14120efe0", tpl_365::<0x141200ef0> as *const ()), // sub_14120efe0
    Replacement::draft(0x14120eff0, "kaim_14120eff0", tpl_22482::<0xc0, 0x20, 0x68, 0x10, 0xa0, 0x6c, 0x48, 0x1416b1580, 0x141213830> as *const ()), // Kaim::NearestBorderHalfEdgeFromPosQuery<navigation::NavigationTraverseLogic>::Kaim::NearestBorderHalfEdgeFromPosQuery<na
    Replacement::draft(0x14120f0a0, "kaim_14120f0a0", tpl_22465::<0x1412129c0, 0x10> as *const ()), // Kaim::MonodirectionalRayCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::MonodirectionalRayCanGoQuery<navigation::
    Replacement::draft(0x14120f0c0, "kaim_14120f0c0", fn_14120f0c0 as *const ()), // sub_14120f0c0
    Replacement::draft(0x14120f790, "kaim_14120f790", fn_14120f790 as *const ()), // sub_14120f790
    Replacement::draft(0x14120fe20, "kaim_14120fe20", fn_14120fe20 as *const ()), // sub_14120fe20
    Replacement::draft(0x14120ff60, "kaim_14120ff60", fn_14120ff60 as *const ()), // sub_14120ff60
    Replacement::draft(0x141210030, "kaim_141210030", fn_141210030 as *const ()), // sub_141210030
    Replacement::draft(0x1412100d0, "kaim_1412100d0", fn_1412100d0 as *const ()), // sub_1412100d0
    Replacement::draft(0x1412101b0, "kaim_1412101b0", fn_1412101b0 as *const ()), // sub_1412101b0
    Replacement::draft(0x141210370, "kaim_141210370", fn_141210370 as *const ()), // sub_141210370
    Replacement::draft(0x141210550, "kaim_141210550", fn_141210550 as *const ()), // sub_141210550
    Replacement::draft(0x141210c40, "kaim_141210c40", tpl_22488::<0x143932190, 0x1416b1b10, 0x141218e80, 0x1416ae230, 0x1411fb750, 0x1411fe6e0> as *const ()), // sub_141210c40
    Replacement::draft(0x1412110c0, "kaim_1412110c0", fn_1412110c0 as *const ()), // sub_1412110c0
    Replacement::draft(0x141211340, "kaim_141211340", fn_141211340 as *const ()), // sub_141211340
    Replacement::draft(0x1412115e0, "kaim_1412115e0", fn_1412115e0 as *const ()), // sub_1412115e0
    Replacement::draft(0x141211800, "kaim_141211800", fn_141211800 as *const ()), // sub_141211800
    Replacement::draft(0x1412119e0, "kaim_1412119e0", fn_1412119e0 as *const ()), // sub_1412119e0
    Replacement::draft(0x141211e30, "kaim_141211e30", fn_141211e30 as *const ()), // sub_141211e30
    Replacement::draft(0x1412122f0, "kaim_1412122f0", fn_1412122f0 as *const ()), // sub_1412122f0
    Replacement::draft(0x1412127e0, "kaim_1412127e0", fn_1412127e0 as *const ()), // sub_1412127e0
    Replacement::draft(0x1412129c0, "kaim_1412129c0", tpl_80942::<0xbc, 0x20, 0x68, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x141212a90> as *const ()), // sub_1412129c0
    Replacement::draft(0x141212a90, "kaim_141212a90", fn_141212a90 as *const ()), // sub_141212a90
    Replacement::draft(0x141213830, "kaim_141213830", fn_141213830 as *const ()), // sub_141213830
    Replacement::draft(0x141213b70, "kaim_141213b70", fn_141213b70 as *const ()), // sub_141213b70
    Replacement::draft(0x141214910, "kaim_141214910", fn_141214910 as *const ()), // sub_141214910
    Replacement::draft(0x141214a60, "kaim_141214a60", fn_141214a60 as *const ()), // sub_141214a60
    Replacement::draft(0x141214bb0, "kaim_141214bb0", fn_141214bb0 as *const ()), // sub_141214bb0
    Replacement::draft(0x141214ee0, "kaim_141214ee0", tpl_22604::<0x1412172f0, 0x1412182e0, 0x1416a7100, 0x14183acbc, 0x143932190, 0x141218cc0> as *const ()), // sub_141214ee0
    Replacement::draft(0x1412150b0, "kaim_1412150b0", fn_1412150b0 as *const ()), // sub_1412150b0
    Replacement::draft(0x141215450, "kaim_141215450", fn_141215450 as *const ()), // sub_141215450
    Replacement::draft(0x141215660, "kaim_141215660", fn_141215660 as *const ()), // sub_141215660
    Replacement::draft(0x141215f60, "kaim_141215f60", fn_141215f60 as *const ()), // sub_141215f60
    Replacement::draft(0x141216560, "kaim_141216560", fn_141216560 as *const ()), // sub_141216560
    Replacement::draft(0x141216730, "kaim_141216730", tpl_22608::<0x8, 0x141216ec0, 0x143932190, 0x60> as *const ()), // sub_141216730
    Replacement::draft(0x141216770, "kaim_141216770", fn_141216770 as *const ()), // sub_141216770
    Replacement::draft(0x1412167b0, "kaim_1412167b0", fn_1412167b0 as *const ()), // sub_1412167b0
    Replacement::draft(0x141216e80, "kaim_141216e80", tpl_22608::<0x8, 0x141216ec0, 0x143932190, 0x60> as *const ()), // sub_141216e80
    Replacement::draft(0x141216ec0, "kaim_141216ec0", fn_141216ec0 as *const ()), // sub_141216ec0
    Replacement::draft(0x141216f30, "kaim_141216f30", fn_141216f30 as *const ()), // sub_141216f30
    Replacement::draft(0x141216fe0, "kaim_141216fe0", fn_141216fe0 as *const ()), // sub_141216fe0
    Replacement::draft(0x141217190, "kaim_141217190", fn_141217190 as *const ()), // sub_141217190
    Replacement::draft(0x141217240, "kaim_141217240", fn_141217240 as *const ()), // sub_141217240
    Replacement::draft(0x1412172f0, "kaim_1412172f0", fn_1412172f0 as *const ()), // sub_1412172f0
    Replacement::draft(0x141217490, "kaim_141217490", fn_141217490 as *const ()), // sub_141217490
    Replacement::draft(0x141217510, "kaim_141217510", fn_141217510 as *const ()), // sub_141217510
    Replacement::draft(0x141217730, "kaim_141217730", fn_141217730 as *const ()), // sub_141217730
    Replacement::draft(0x141217900, "kaim_141217900", fn_141217900 as *const ()), // sub_141217900
    Replacement::draft(0x141217c80, "kaim_141217c80", fn_141217c80 as *const ()), // sub_141217c80
    Replacement::draft(0x141217e60, "kaim_141217e60", fn_141217e60 as *const ()), // sub_141217e60
    Replacement::draft(0x1412180b0, "kaim_1412180b0", tpl_22621::<0x60, 0x14121baf0, 0x88, 0x80, 0x18, 0xc, 0x14, 0x1c, 0x141201b00> as *const ()), // sub_1412180b0
    Replacement::draft(0x141218190, "kaim_141218190", fn_141218190 as *const ()), // sub_141218190
    Replacement::draft(0x1412182e0, "kaim_1412182e0", fn_1412182e0 as *const ()), // sub_1412182e0
    Replacement::draft(0x141218cc0, "kaim_141218cc0", fn_141218cc0 as *const ()), // sub_141218cc0
    Replacement::draft(0x141218e80, "kaim_141218e80", tpl_80829::<0x1416a4590, 0x1416a4620, 0x1416a4bc0, 0x140000000, 0x143428720, 0x1416b1580, 0x14121c470, 0x1433e16f0, 0x143932190, 0x1433e1728, 0x14169edb0, 0x14169ed80> as *const ()), // sub_141218e80
    Replacement::draft(0x141219500, "kaim_141219500", fn_141219500 as *const ()), // sub_141219500
    Replacement::draft(0x14121a260, "kaim_14121a260", fn_14121a260 as *const ()), // sub_14121a260
    Replacement::draft(0x14121a600, "kaim_14121a600", fn_14121a600 as *const ()), // sub_14121a600
    Replacement::draft(0x14121a820, "kaim_14121a820", fn_14121a820 as *const ()), // sub_14121a820
    Replacement::draft(0x14121af10, "kaim_14121af10", fn_14121af10 as *const ()), // sub_14121af10
    Replacement::draft(0x14121b0e0, "kaim_14121b0e0", fn_14121b0e0 as *const ()), // sub_14121b0e0
    Replacement::draft(0x14121b2e0, "kaim_14121b2e0", fn_14121b2e0 as *const ()), // sub_14121b2e0
    Replacement::draft(0x14121b760, "kaim_14121b760", fn_14121b760 as *const ()), // sub_14121b760
    Replacement::draft(0x14121b8c0, "kaim_14121b8c0", fn_14121b8c0 as *const ()), // sub_14121b8c0
    Replacement::draft(0x14121b950, "kaim_14121b950", fn_14121b950 as *const ()), // sub_14121b950
    Replacement::draft(0x14121b9f0, "kaim_14121b9f0", tpl_22631::<0x1416b59d0, 0x18, 0x4, 0x60, 0x1416ce4a0, 0x141218190> as *const ()), // sub_14121b9f0
    Replacement::draft(0x14121baf0, "kaim_14121baf0", fn_14121baf0 as *const ()), // sub_14121baf0
    Replacement::draft(0x14121bde0, "kaim_14121bde0", fn_14121bde0 as *const ()), // sub_14121bde0
    Replacement::draft(0x14121c0b0, "kaim_14121c0b0", fn_14121c0b0 as *const ()), // sub_14121c0b0
    Replacement::draft(0x14121c470, "kaim_14121c470", fn_14121c470 as *const ()), // sub_14121c470
    Replacement::draft(0x14121c6e0, "kaim_14121c6e0", fn_14121c6e0 as *const ()), // sub_14121c6e0
    Replacement::draft(0x14121c7f0, "kaim_14121c7f0", fn_14121c7f0 as *const ()), // sub_14121c7f0
    Replacement::draft(0x14121ca60, "kaim_14121ca60", fn_14121ca60 as *const ()), // sub_14121ca60
    Replacement::draft(0x14121cc80, "kaim_14121cc80", fn_14121cc80 as *const ()), // sub_14121cc80
    Replacement::draft(0x14121cf50, "kaim_14121cf50", fn_14121cf50 as *const ()), // sub_14121cf50
    Replacement::draft(0x14121d180, "kaim_14121d180", fn_14121d180 as *const ()), // sub_14121d180
    Replacement::draft(0x14121d4d0, "kaim_14121d4d0", fn_14121d4d0 as *const ()), // sub_14121d4d0
    Replacement::draft(0x14121d6c0, "kaim_14121d6c0", fn_14121d6c0 as *const ()), // sub_14121d6c0
    Replacement::draft(0x14121d900, "kaim_14121d900", fn_14121d900 as *const ()), // sub_14121d900
    Replacement::draft(0x14121db50, "kaim_14121db50", fn_14121db50 as *const ()), // sub_14121db50
    Replacement::draft(0x14121de90, "kaim_14121de90", fn_14121de90 as *const ()), // sub_14121de90
    Replacement::draft(0x14121e0d0, "kaim_14121e0d0", fn_14121e0d0 as *const ()), // sub_14121e0d0
    Replacement::draft(0x14121f1c0, "kaim_14121f1c0", tpl_22426::<0x1433e19a8, 0x141200d80> as *const ()), // sub_14121f1c0
    Replacement::draft(0x14121f1d0, "kaim_14121f1d0", tpl_22645::<0x1433e19a8, 0x141200d80, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::vfun
    Replacement::draft(0x14121f240, "kaim_14121f240", fn_14121f240 as *const ()), // sub_14121f240
    Replacement::draft(0x14121f6a0, "kaim_14121f6a0", fn_14121f6a0 as *const ()), // sub_14121f6a0
    Replacement::draft(0x14121f910, "kaim_14121f910", fn_14121f910 as *const ()), // sub_14121f910
    Replacement::draft(0x14121feb0, "kaim_14121feb0", tpl_22649::<0x1416bdab0, 0x1416b7e40, 0xf0, 0x14121de90, 0x1416baa10, 0x258, 0x1416b8150, 0x1416bdc60> as *const ()), // sub_14121feb0
    Replacement::draft(0x14121ff90, "kaim_14121ff90", tpl_22482::<0xe0, 0x20, 0x68, 0x10, 0xa0, 0x6c, 0x48, 0x1416b1580, 0x141220040> as *const ()), // Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::vfun
    Replacement::draft(0x141220040, "kaim_141220040", fn_141220040 as *const ()), // sub_141220040
    Replacement::draft(0x141220960, "kaim_141220960", fn_141220960 as *const ()), // sub_141220960
    Replacement::draft(0x141220b20, "kaim_141220b20", fn_141220b20 as *const ()), // sub_141220b20
    Replacement::draft(0x141221890, "kaim_141221890", fn_141221890 as *const ()), // sub_141221890
    Replacement::draft(0x141221fb0, "kaim_141221fb0", fn_141221fb0 as *const ()), // sub_141221fb0
    Replacement::draft(0x141222220, "kaim_141222220", fn_141222220 as *const ()), // sub_141222220
    Replacement::draft(0x1412228c0, "kaim_1412228c0", fn_1412228c0 as *const ()), // sub_1412228c0
    Replacement::draft(0x1412229b0, "kaim_1412229b0", fn_1412229b0 as *const ()), // sub_1412229b0
    Replacement::draft(0x141222da0, "kaim_141222da0", fn_141222da0 as *const ()), // sub_141222da0
    Replacement::draft(0x141223650, "kaim_141223650", tpl_22604::<0x1412172f0, 0x141224250, 0x1416a7100, 0x14183acbc, 0x143932190, 0x141224c80> as *const ()), // sub_141223650
    Replacement::draft(0x141223820, "kaim_141223820", fn_141223820 as *const ()), // sub_141223820
    Replacement::draft(0x141223e00, "kaim_141223e00", fn_141223e00 as *const ()), // sub_141223e00
    Replacement::draft(0x141224140, "kaim_141224140", fn_141224140 as *const ()), // sub_141224140
    Replacement::draft(0x141224250, "kaim_141224250", fn_141224250 as *const ()), // sub_141224250
    Replacement::draft(0x141224c80, "kaim_141224c80", fn_141224c80 as *const ()), // sub_141224c80
    Replacement::draft(0x141224e40, "kaim_141224e40", tpl_22604::<0x1412172f0, 0x1412251c0, 0x1416a7100, 0x14183acbc, 0x143932190, 0x141225bf0> as *const ()), // sub_141224e40
    Replacement::draft(0x141225010, "kaim_141225010", fn_141225010 as *const ()), // sub_141225010
    Replacement::draft(0x1412251c0, "kaim_1412251c0", fn_1412251c0 as *const ()), // sub_1412251c0
    Replacement::draft(0x141225db0, "kaim_141225db0", fn_141225db0 as *const ()), // Kaim::DatabaseBinding::Kaim::DatabaseBinding::vfunc_0
    Replacement::draft(0x141225e40, "kaim_141225e40", fn_141225e40 as *const ()), // sub_141225e40
    Replacement::draft(0x141225e60, "kaim_141225e60", fn_141225e60 as *const ()), // sub_141225e60
    Replacement::draft(0x141225e80, "kaim_141225e80", fn_141225e80 as *const ()), // sub_141225e80
    Replacement::draft(0x141225ea0, "kaim_141225ea0", tpl_22476::<0x78, 0xffffffffffffffff, 0x1416daf20, 0x143932190, 0x60> as *const ()), // sub_141225ea0
    Replacement::draft(0x141225ee0, "kaim_141225ee0", fn_141225ee0 as *const ()), // sub_141225ee0
    Replacement::draft(0x141225f40, "kaim_141225f40", fn_141225f40 as *const ()), // sub_141225f40
    Replacement::draft(0x141225f90, "kaim_141225f90", fn_141225f90 as *const ()), // sub_141225f90
    Replacement::draft(0x141225fe0, "kaim_141225fe0", fn_141225fe0 as *const ()), // sub_141225fe0
    Replacement::draft(0x141226310, "kaim_141226310", fn_141226310 as *const ()), // sub_141226310
    Replacement::draft(0x141226540, "kaim_141226540", tpl_22439::<0x1433e1ff0, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::IPositionOnPathValidator::Kaim::IPositionOnPathValidator::vfunc_0
    Replacement::draft(0x1412265b0, "kaim_1412265b0", fn_1412265b0 as *const ()), // sub_1412265b0
    Replacement::draft(0x1412265d0, "kaim_1412265d0", fn_1412265d0 as *const ()), // sub_1412265d0
    Replacement::draft(0x1412265f0, "kaim_1412265f0", fn_1412265f0 as *const ()), // sub_1412265f0
    Replacement::draft(0x141226610, "kaim_141226610", fn_141226610 as *const ()), // sub_141226610
    Replacement::draft(0x141226660, "kaim_141226660", fn_141226660 as *const ()), // sub_141226660
    Replacement::draft(0x1412266e0, "kaim_1412266e0", fn_1412266e0 as *const ()), // sub_1412266e0
    Replacement::draft(0x141226760, "kaim_141226760", tpl_22671::<0x30, 0x34, 0x8, 0x141715300, 0x14170e580> as *const ()), // sub_141226760
    Replacement::draft(0x1412267a0, "kaim_1412267a0", fn_1412267a0 as *const ()), // sub_1412267a0
    Replacement::draft(0x1412267e0, "kaim_1412267e0", tpl_22524::<0x1433e2040, 0x58, 0x143932190, 0x60, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_1412267e0
    Replacement::draft(0x141226840, "kaim_141226840", tpl_22464::<0x1412267e0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::BaseMoveOnNavMeshQuery::Kaim::BaseMoveOnNavMeshQuery::vfunc_0
    Replacement::draft(0x1412268a0, "kaim_1412268a0", fn_1412268a0 as *const ()), // Kaim::FindFirstVisiblePositionOnPathQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim:
    Replacement::draft(0x1412268b0, "kaim_1412268b0", tpl_22466::<0x140200580> as *const ()), // Kaim::FindFirstVisiblePositionOnPathQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim:
    Replacement::draft(0x1412268d0, "kaim_1412268d0", fn_1412268d0 as *const ()), // sub_1412268d0
    Replacement::draft(0x1412269d0, "kaim_1412269d0", fn_1412269d0 as *const ()), // Kaim::BaseFindFirstVisiblePositionOnPathQuery::Kaim::BaseFindFirstVisiblePositionOnPathQuery::vfunc_0
    Replacement::draft(0x141226a30, "navigation_141226a30", fn_141226a30 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_0
    Replacement::draft(0x141226ad0, "navigation_141226ad0", fn_141226ad0 as *const ()), // sub_141226ad0
    Replacement::draft_throws(0x141226b40, "navigation_141226b40", fn_141226b40 as *const (), Throws::Discard), // navigation::WEDNavigationManager::navigation::WEDNavigationManager::vfunc_0
    Replacement::draft_throws(0x141226bf0, "unknown_141226bf0", fn_141226bf0 as *const (), Throws::Caught), // sub_141226bf0
    Replacement::draft_throws(0x141226c80, "unknown_141226c80", fn_141226c80 as *const (), Throws::Discard), // sub_141226c80
    Replacement::draft(0x141226d70, "unknown_141226d70", fn_141226d70 as *const ()), // sub_141226d70
    Replacement::draft(0x141226df0, "unknown_141226df0", fn_141226df0 as *const ()), // sub_141226df0
    Replacement::draft(0x141226e90, "kaim_141226e90", tpl_22683::<0x1433e1ea0, 0x14170bbd0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::PathProgressComputer<navigation::NavigationTraverseLogic>::Kaim::PathProgressComputer<navigation::NavigationTraver
    Replacement::draft(0x141226f00, "unknown_141226f00", fn_141226f00 as *const ()), // sub_141226f00
    Replacement::draft(0x141226f50, "unknown_141226f50", fn_141226f50 as *const ()), // sub_141226f50
    Replacement::draft(0x141227030, "unknown_141227030", fn_141227030 as *const ()), // sub_141227030
    Replacement::draft(0x1412270c0, "unknown_1412270c0", fn_1412270c0 as *const ()), // sub_1412270c0
    Replacement::draft(0x141227150, "unknown_141227150", fn_141227150 as *const ()), // sub_141227150
    Replacement::draft(0x1412271b0, "navigation_1412271b0", fn_1412271b0 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_1
    Replacement::draft(0x141227220, "navigation_141227220", fn_141227220 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_2
    Replacement::draft(0x141227310, "navigation_141227310", fn_141227310 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_4
    Replacement::draft(0x1412273a0, "navigation_1412273a0", fn_1412273a0 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_5
    Replacement::draft(0x1412274b0, "navigation_1412274b0", fn_1412274b0 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_6
    Replacement::draft(0x1412275e0, "navigation_1412275e0", fn_1412275e0 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_9
    Replacement::draft(0x141227640, "navigation_141227640", fn_141227640 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_10
    Replacement::draft(0x141227770, "kaim_141227770", fn_141227770 as *const ()), // Kaim::PathProgressComputer<navigation::NavigationTraverseLogic>::Kaim::PathProgressComputer<navigation::NavigationTraver
    Replacement::draft(0x1412277e0, "kaim_1412277e0", fn_1412277e0 as *const ()), // Kaim::ShortcutTrajectory<navigation::NavigationTraverseLogic>::Kaim::ShortcutTrajectory<navigation::NavigationTraverseLo
    Replacement::draft(0x141227840, "kaim_141227840", fn_141227840 as *const ()), // sub_141227840
    Replacement::draft(0x1412278d0, "kaim_1412278d0", fn_1412278d0 as *const ()), // sub_1412278d0
    Replacement::draft(0x141227bf0, "kaim_141227bf0", fn_141227bf0 as *const ()), // Kaim::ShortcutTrajectory<navigation::NavigationTraverseLogic>::Kaim::ShortcutTrajectory<navigation::NavigationTraverseLo
    Replacement::draft(0x141227eb0, "kaim_141227eb0", fn_141227eb0 as *const ()), // Kaim::ShortcutTrajectory<navigation::NavigationTraverseLogic>::Kaim::ShortcutTrajectory<navigation::NavigationTraverseLo
    Replacement::draft(0x141227f30, "kaim_141227f30", fn_141227f30 as *const ()), // Kaim::ShortcutTrajectory<navigation::NavigationTraverseLogic>::Kaim::ShortcutTrajectory<navigation::NavigationTraverseLo
    Replacement::draft(0x1412281e0, "kaim_1412281e0", fn_1412281e0 as *const ()), // sub_1412281e0
    Replacement::draft(0x141228270, "kaim_141228270", fn_141228270 as *const ()), // sub_141228270
    Replacement::draft(0x1412282a0, "kaim_1412282a0", fn_1412282a0 as *const ()), // sub_1412282a0
    Replacement::draft(0x141228300, "kaim_141228300", fn_141228300 as *const ()), // sub_141228300
    Replacement::draft(0x141228940, "kaim_141228940", fn_141228940 as *const ()), // sub_141228940
    Replacement::draft(0x141228b30, "kaim_141228b30", fn_141228b30 as *const ()), // sub_141228b30
    Replacement::draft(0x141228e50, "kaim_141228e50", fn_141228e50 as *const ()), // Kaim::MoveOnNavMeshQuery<navigation::NavigationTraverseLogic>::Kaim::MoveOnNavMeshQuery<navigation::NavigationTraverseLo
    Replacement::draft(0x141228ec0, "kaim_141228ec0", tpl_22465::<0x14122a3b0, 0x10> as *const ()), // Kaim::MoveOnNavMeshQuery<navigation::NavigationTraverseLogic>::Kaim::MoveOnNavMeshQuery<navigation::NavigationTraverseLo
    Replacement::draft(0x141228ee0, "kaim_141228ee0", tpl_22426::<0x1433e1f00, 0x1412267e0> as *const ()), // sub_141228ee0
    Replacement::draft(0x141228ef0, "kaim_141228ef0", fn_141228ef0 as *const ()), // sub_141228ef0
    Replacement::draft(0x1412290b0, "kaim_1412290b0", tpl_22707::<0x14170f7b0, 0x1416dc3d0, 0x1433e1e50, 0x1416dc4b0, 0x1416bcaf0, 0x1416dd0b0, 0x143932190, 0x1412009a0, 0x1416b1580, 0x14122d1f0> as *const ()), // sub_1412290b0
    Replacement::draft(0x1412292b0, "kaim_1412292b0", fn_1412292b0 as *const ()), // sub_1412292b0
    Replacement::draft(0x1412297e0, "kaim_1412297e0", fn_1412297e0 as *const ()), // sub_1412297e0
    Replacement::draft(0x141229f80, "kaim_141229f80", fn_141229f80 as *const ()), // sub_141229f80
    Replacement::draft(0x14122a3b0, "kaim_14122a3b0", fn_14122a3b0 as *const ()), // sub_14122a3b0
    Replacement::draft(0x14122a700, "kaim_14122a700", fn_14122a700 as *const ()), // sub_14122a700
    Replacement::draft(0x14122a760, "kaim_14122a760", tpl_22426::<0x1433e2008, 0x1412268d0> as *const ()), // sub_14122a760
    Replacement::draft(0x14122a770, "kaim_14122a770", tpl_22465::<0x14122acf0, 0x10> as *const ()), // Kaim::RayCanGoQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::RayCanGoQuery<Kaim::L
    Replacement::draft(0x14122a790, "kaim_14122a790", fn_14122a790 as *const ()), // sub_14122a790
    Replacement::draft(0x14122a8d0, "kaim_14122a8d0", tpl_22714::<0x1433e2008, 0x1412268d0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::FindFirstVisiblePositionOnPathQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim:
    Replacement::draft(0x14122a940, "kaim_14122a940", fn_14122a940 as *const ()), // sub_14122a940
    Replacement::draft(0x14122acf0, "kaim_14122acf0", tpl_80942::<0xbc, 0x20, 0x68, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x14122d1f0> as *const ()), // sub_14122acf0
    Replacement::draft(0x14122adc0, "kaim_14122adc0", tpl_22465::<0x14122ade0, 0x10> as *const ()), // Kaim::FindFirstVisiblePositionOnPathQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim:
    Replacement::draft(0x14122ade0, "kaim_14122ade0", fn_14122ade0 as *const ()), // sub_14122ade0
    Replacement::draft(0x14122b910, "kaim_14122b910", fn_14122b910 as *const ()), // sub_14122b910
    Replacement::draft(0x14122bbe0, "kaim_14122bbe0", fn_14122bbe0 as *const ()), // sub_14122bbe0
    Replacement::draft(0x14122bf90, "kaim_14122bf90", fn_14122bf90 as *const ()), // sub_14122bf90
    Replacement::draft(0x14122c4c0, "kaim_14122c4c0", fn_14122c4c0 as *const ()), // sub_14122c4c0
    Replacement::draft(0x14122c780, "kaim_14122c780", fn_14122c780 as *const ()), // sub_14122c780
    Replacement::draft(0x14122cc90, "kaim_14122cc90", fn_14122cc90 as *const ()), // sub_14122cc90
    Replacement::draft(0x14122cfa0, "kaim_14122cfa0", fn_14122cfa0 as *const ()), // sub_14122cfa0
    Replacement::draft(0x14122d1f0, "kaim_14122d1f0", fn_14122d1f0 as *const ()), // sub_14122d1f0
    Replacement::draft(0x14122dbc0, "kaim_14122dbc0", fn_14122dbc0 as *const ()), // sub_14122dbc0
    Replacement::draft(0x14122e130, "kaim_14122e130", fn_14122e130 as *const ()), // sub_14122e130
    Replacement::draft(0x14122e1e0, "kaim_14122e1e0", fn_14122e1e0 as *const ()), // sub_14122e1e0
    Replacement::draft(0x14122e5d0, "kaim_14122e5d0", fn_14122e5d0 as *const ()), // sub_14122e5d0
    Replacement::draft(0x14122e920, "kaim_14122e920", fn_14122e920 as *const ()), // sub_14122e920
    Replacement::draft(0x14122ee20, "kaim_14122ee20", fn_14122ee20 as *const ()), // sub_14122ee20
    Replacement::draft(0x14122f060, "kaim_14122f060", fn_14122f060 as *const ()), // sub_14122f060
    Replacement::draft(0x14122f380, "kaim_14122f380", fn_14122f380 as *const ()), // sub_14122f380
    Replacement::draft(0x14122fb10, "kaim_14122fb10", fn_14122fb10 as *const ()), // sub_14122fb10
    Replacement::draft(0x14122fbb0, "kaim_14122fbb0", tpl_22727::<0x1416b1060, 0x1411fd8d0, 0x1411fbda0, 0x1411ff250, 0x1416b0e60, 0x141230360> as *const ()), // sub_14122fbb0
    Replacement::draft(0x14122fe40, "kaim_14122fe40", tpl_22727::<0x1416b1060, 0x1411fd8d0, 0x1411fbda0, 0x1411ff250, 0x1416b0e60, 0x141230690> as *const ()), // sub_14122fe40
    Replacement::draft(0x1412300d0, "kaim_1412300d0", tpl_22727::<0x1416b1060, 0x1411fd8d0, 0x1411fbda0, 0x1411ff250, 0x1416b0e60, 0x1412309c0> as *const ()), // sub_1412300d0
    Replacement::draft(0x141230360, "kaim_141230360", fn_141230360 as *const ()), // sub_141230360
    Replacement::draft(0x141230690, "kaim_141230690", fn_141230690 as *const ()), // sub_141230690
    Replacement::draft(0x1412309c0, "kaim_1412309c0", fn_1412309c0 as *const ()), // sub_1412309c0
    Replacement::draft(0x141230cf0, "kaim_141230cf0", fn_141230cf0 as *const ()), // sub_141230cf0
    Replacement::draft(0x141231230, "kaim_141231230", fn_141231230 as *const ()), // sub_141231230
    Replacement::draft(0x1412318f0, "kaim_1412318f0", fn_1412318f0 as *const ()), // sub_1412318f0
    Replacement::draft(0x1412320a0, "kaim_1412320a0", fn_1412320a0 as *const ()), // sub_1412320a0
    Replacement::draft(0x141232180, "kaim_141232180", tpl_22731::<0x10, 0x143932190, 0x60, 0x8, 0x14169edd0> as *const ()), // sub_141232180
    Replacement::draft(0x1412321c0, "kaim_1412321c0", fn_1412321c0 as *const ()), // sub_1412321c0
    Replacement::draft(0x1412322c0, "kaim_1412322c0", fn_1412322c0 as *const ()), // sub_1412322c0
    Replacement::draft(0x1412322f0, "kaim_1412322f0", fn_1412322f0 as *const ()), // sub_1412322f0
    Replacement::draft(0x141232380, "kaim_141232380", tpl_22735::<0x14171acf0, 0x141232ff0> as *const ()), // sub_141232380
    Replacement::draft(0x1412323a0, "kaim_1412323a0", tpl_365::<0x141232ff0> as *const ()), // sub_1412323a0
    Replacement::draft(0x1412323b0, "kaim_1412323b0", fn_1412323b0 as *const ()), // sub_1412323b0
    Replacement::draft(0x141232420, "kaim_141232420", fn_141232420 as *const ()), // sub_141232420
    Replacement::draft(0x141232460, "kaim_141232460", fn_141232460 as *const ()), // sub_141232460
    Replacement::draft(0x1412327e0, "kaim_1412327e0", fn_1412327e0 as *const ()), // Kaim::BotConfig::Kaim::BotConfig::vfunc_0
    Replacement::draft(0x141232880, "kaim_141232880", fn_141232880 as *const ()), // sub_141232880
    Replacement::draft(0x1412328e0, "kaim_1412328e0", fn_1412328e0 as *const ()), // sub_1412328e0
    Replacement::draft(0x1412329c0, "kaim_1412329c0", fn_1412329c0 as *const ()), // sub_1412329c0
    Replacement::draft(0x141232a30, "kaim_141232a30", fn_141232a30 as *const ()), // sub_141232a30
    Replacement::draft(0x141232ab0, "kaim_141232ab0", fn_141232ab0 as *const ()), // sub_141232ab0
    Replacement::draft(0x141232bd0, "kaim_141232bd0", fn_141232bd0 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x141232be0, "kaim_141232be0", fn_141232be0 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x141232c00, "kaim_141232c00", fn_141232c00 as *const ()), // sub_141232c00
    Replacement::draft(0x141232d40, "kaim_141232d40", tpl_22748::<0x141232c00, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::BaseMultiDestinationPathFinderQueryRMD::Kaim::BaseMultiDestinationPathFinderQueryRMD::vfunc_0
    Replacement::draft(0x141232da0, "unknown_141232da0", fn_141232da0 as *const ()), // sub_141232da0
    Replacement::draft(0x141232e20, "unknown_141232e20", tpl_365::<0x141232f80> as *const ()), // sub_141232e20
    Replacement::draft(0x141232e30, "navigation_141232e30", fn_141232e30 as *const ()), // navigation::NavigationAgent::navigation::NavigationAgent::vfunc_0
    Replacement::draft(0x141232f40, "unknown_141232f40", tpl_22731::<0x40, 0x143932190, 0x60, 0x10, 0x14169edd0> as *const ()), // sub_141232f40
    Replacement::draft(0x141232f80, "unknown_141232f80", fn_141232f80 as *const ()), // sub_141232f80
    Replacement::draft(0x141232ff0, "unknown_141232ff0", fn_141232ff0 as *const ()), // sub_141232ff0
    Replacement::draft(0x141233060, "unknown_141233060", fn_141233060 as *const ()), // sub_141233060
    Replacement::draft(0x1412330a0, "kaim_1412330a0", fn_1412330a0 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x141233e20, "kaim_141233e20", fn_141233e20 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x141233e40, "kaim_141233e40", fn_141233e40 as *const ()), // sub_141233e40
    Replacement::draft(0x141233f50, "kaim_141233f50", fn_141233f50 as *const ()), // sub_141233f50
    Replacement::draft(0x141234040, "kaim_141234040", fn_141234040 as *const ()), // sub_141234040
    Replacement::draft(0x1412340c0, "kaim_1412340c0", fn_1412340c0 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x141234140, "kaim_141234140", fn_141234140 as *const ()), // sub_141234140
    Replacement::draft(0x1412341b0, "kaim_1412341b0", fn_1412341b0 as *const ()), // sub_1412341b0
    Replacement::draft(0x141234220, "kaim_141234220", fn_141234220 as *const ()), // sub_141234220
    Replacement::draft(0x141234270, "kaim_141234270", fn_141234270 as *const ()), // sub_141234270
    Replacement::draft(0x141234300, "kaim_141234300", fn_141234300 as *const ()), // sub_141234300
    Replacement::draft(0x1412343e0, "kaim_1412343e0", fn_1412343e0 as *const ()), // sub_1412343e0
    Replacement::draft(0x1412346e0, "kaim_1412346e0", fn_1412346e0 as *const ()), // sub_1412346e0
    Replacement::draft(0x141234960, "kaim_141234960", fn_141234960 as *const ()), // sub_141234960
    Replacement::draft(0x141234c20, "kaim_141234c20", fn_141234c20 as *const ()), // sub_141234c20
    Replacement::draft(0x141234cc0, "kaim_141234cc0", fn_141234cc0 as *const ()), // sub_141234cc0
    Replacement::draft(0x141234d60, "kaim_141234d60", fn_141234d60 as *const ()), // sub_141234d60
    Replacement::draft(0x141234fe0, "kaim_141234fe0", fn_141234fe0 as *const ()), // sub_141234fe0
    Replacement::draft(0x141235110, "kaim_141235110", fn_141235110 as *const ()), // sub_141235110
    Replacement::draft(0x1412356e0, "kaim_1412356e0", fn_1412356e0 as *const ()), // sub_1412356e0
    Replacement::draft(0x141235730, "kaim_141235730", fn_141235730 as *const ()), // sub_141235730
    Replacement::draft(0x141235ec0, "kaim_141235ec0", fn_141235ec0 as *const ()), // sub_141235ec0
    Replacement::draft(0x141236000, "kaim_141236000", fn_141236000 as *const ()), // sub_141236000
    Replacement::draft(0x1412361d0, "kaim_1412361d0", fn_1412361d0 as *const ()), // sub_1412361d0
    Replacement::draft(0x1412363e0, "kaim_1412363e0", fn_1412363e0 as *const ()), // sub_1412363e0
    Replacement::draft(0x141236610, "kaim_141236610", fn_141236610 as *const ()), // sub_141236610
    Replacement::draft(0x1412367c0, "kaim_1412367c0", fn_1412367c0 as *const ()), // sub_1412367c0
    Replacement::draft(0x1412369a0, "kaim_1412369a0", fn_1412369a0 as *const ()), // sub_1412369a0
    Replacement::draft(0x141236df0, "kaim_141236df0", fn_141236df0 as *const ()), // sub_141236df0
    Replacement::draft(0x1412372b0, "kaim_1412372b0", fn_1412372b0 as *const ()), // sub_1412372b0
    Replacement::draft(0x1412377e0, "kaim_1412377e0", fn_1412377e0 as *const ()), // sub_1412377e0
    Replacement::draft(0x141237890, "kaim_141237890", fn_141237890 as *const ()), // sub_141237890
    Replacement::draft(0x141237ab0, "kaim_141237ab0", fn_141237ab0 as *const ()), // sub_141237ab0
    Replacement::draft(0x141237c80, "kaim_141237c80", fn_141237c80 as *const ()), // sub_141237c80
    Replacement::draft(0x141238000, "kaim_141238000", fn_141238000 as *const ()), // sub_141238000
    Replacement::draft(0x1412381e0, "kaim_1412381e0", fn_1412381e0 as *const ()), // sub_1412381e0
    Replacement::draft(0x141238430, "kaim_141238430", tpl_22621::<0x10, 0x141239140, 0x88, 0x80, 0x18, 0xc, 0x14, 0x1c, 0x141201b00> as *const ()), // sub_141238430
    Replacement::draft(0x141238510, "kaim_141238510", fn_141238510 as *const ()), // sub_141238510
    Replacement::draft(0x141238660, "kaim_141238660", fn_141238660 as *const ()), // sub_141238660
    Replacement::draft(0x141238980, "kaim_141238980", fn_141238980 as *const ()), // sub_141238980
    Replacement::draft(0x141238e40, "kaim_141238e40", fn_141238e40 as *const ()), // sub_141238e40
    Replacement::draft(0x141238fa0, "kaim_141238fa0", fn_141238fa0 as *const ()), // sub_141238fa0
    Replacement::draft(0x141239040, "kaim_141239040", tpl_22631::<0x1416b59d0, 0x18, 0x4, 0x10, 0x1416ce4a0, 0x141238510> as *const ()), // sub_141239040
    Replacement::draft(0x141239140, "kaim_141239140", fn_141239140 as *const ()), // sub_141239140
    Replacement::draft(0x1412393c0, "kaim_1412393c0", fn_1412393c0 as *const ()), // sub_1412393c0
    Replacement::draft(0x141239670, "kaim_141239670", fn_141239670 as *const ()), // sub_141239670
    Replacement::draft(0x141239a40, "kaim_141239a40", fn_141239a40 as *const ()), // sub_141239a40
    Replacement::draft(0x141239e50, "kaim_141239e50", fn_141239e50 as *const ()), // sub_141239e50
    Replacement::draft(0x14123a1a0, "kaim_14123a1a0", tpl_893::<0x1433e2298, 0x141834d30> as *const ()), // Kaim::SysAllocBase::Kaim::SysAllocBase::vfunc_0
    Replacement::draft(0x14123a1d0, "kaim_14123a1d0", tpl_1674::<0x1433e2298> as *const ()), // sub_14123a1d0
    Replacement::draft(0x14123a1e0, "navigation_14123a1e0", fn_14123a1e0 as *const ()), // navigation::KaimSysAllocWrapper::navigation::KaimSysAllocWrapper::vfunc_0
    Replacement::draft(0x14123a240, "unknown_14123a240", fn_14123a240 as *const ()), // sub_14123a240
    Replacement::draft_throws(0x14123a3d0, "unknown_14123a3d0", fn_14123a3d0 as *const (), Throws::Discard), // sub_14123a3d0
    Replacement::draft(0x14123a570, "kaim_14123a570", tpl_1::<0x1433e22b0> as *const ()), // Kaim::PointOfInterest::Kaim::PointOfInterest::vfunc_2
    Replacement::draft(0x14123a580, "kaim_14123a580", fn_14123a580 as *const ()), // Kaim::PointOfInterest::Kaim::PointOfInterest::vfunc_0
    Replacement::draft(0x14123a610, "kaim_14123a610", tpl_18960::<0x8, 0x14169edd0> as *const ()), // sub_14123a610
    Replacement::draft_throws(0x14123a630, "kaim_14123a630", fn_14123a630 as *const (), Throws::Discard), // sub_14123a630
    Replacement::draft_throws(0x14123a6e0, "kaim_14123a6e0", fn_14123a6e0 as *const (), Throws::Discard), // sub_14123a6e0
    Replacement::draft(0x14123a8f0, "kaim_14123a8f0", fn_14123a8f0 as *const ()), // sub_14123a8f0
    Replacement::draft_throws(0x14123a9c0, "kaim_14123a9c0", tpl_19083::<0x19, 0x10, 0x14123a9c0, 0x28, 0x14123abe0, 0x14241c190> as *const (), Throws::Discard), // sub_14123a9c0
    Replacement::draft(0x14123aa40, "kaim_14123aa40", fn_14123aa40 as *const ()), // sub_14123aa40
    Replacement::draft_throws(0x14123aae0, "kaim_14123aae0", fn_14123aae0 as *const (), Throws::Discard), // sub_14123aae0
    Replacement::draft_throws(0x14123ab90, "kaim_14123ab90", fn_14123ab90 as *const (), Throws::Discard), // sub_14123ab90
    Replacement::draft_throws(0x14123abe0, "kaim_14123abe0", tpl_22276::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_14123abe0
    Replacement::draft_throws(0x14123ac60, "kaim_14123ac60", fn_14123ac60 as *const (), Throws::Discard), // sub_14123ac60
    Replacement::draft(0x14123adc0, "kaim_14123adc0", tpl_22426::<0x1433e22f0, 0x141201140> as *const ()), // sub_14123adc0
    Replacement::draft(0x14123add0, "kaim_14123add0", fn_14123add0 as *const ()), // Kaim::FloodFillQuery<navigation::NavigationTraverseLogic>::Kaim::FloodFillQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x14123ae40, "kaim_14123ae40", tpl_22805::<0x8, 0x10, 0x141202860, 0xc, 0x18, 0xfffffffffffffff8, 0xfffffffffffffffc> as *const ()), // sub_14123ae40
    Replacement::draft(0x14123af00, "kaim_14123af00", fn_14123af00 as *const ()), // sub_14123af00
    Replacement::draft_throws(0x14123af70, "kaim_14123af70", tpl_838::<0x14123b460> as *const (), Throws::Discard), // Kaim::FloodFillQuery<navigation::NavigationTraverseLogic>::Kaim::FloodFillQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x14123af80, "kaim_14123af80", fn_14123af80 as *const ()), // Kaim::FloodFillQuery<navigation::NavigationTraverseLogic>::Kaim::FloodFillQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x14123afb0, "kaim_14123afb0", fn_14123afb0 as *const ()), // sub_14123afb0
    Replacement::draft_throws(0x14123b1e0, "kaim_14123b1e0", fn_14123b1e0 as *const (), Throws::Discard), // sub_14123b1e0
    Replacement::draft(0x14123b360, "kaim_14123b360", fn_14123b360 as *const ()), // sub_14123b360
    Replacement::draft_throws(0x14123b3d0, "kaim_14123b3d0", fn_14123b3d0 as *const (), Throws::Discard), // sub_14123b3d0
    Replacement::draft_throws(0x14123b460, "unknown_14123b460", fn_14123b460 as *const (), Throws::Caught), // sub_14123b460
    Replacement::draft_throws(0x14123b6c0, "unknown_14123b6c0", fn_14123b6c0 as *const (), Throws::Discard), // sub_14123b6c0
    Replacement::draft(0x14123b750, "unknown_14123b750", fn_14123b750 as *const ()), // sub_14123b750
    Replacement::draft_throws(0x14123b910, "unknown_14123b910", fn_14123b910 as *const (), Throws::Discard), // sub_14123b910
    Replacement::draft(0x14123bd30, "unknown_14123bd30", fn_14123bd30 as *const ()), // sub_14123bd30
    Replacement::draft_throws(0x14123bdc0, "unknown_14123bdc0", fn_14123bdc0 as *const (), Throws::Discard), // sub_14123bdc0
    Replacement::draft_throws(0x14123c200, "unknown_14123c200", fn_14123c200 as *const (), Throws::Discard), // sub_14123c200
    Replacement::draft(0x14123c890, "unknown_14123c890", fn_14123c890 as *const ()), // sub_14123c890
    Replacement::draft_throws(0x14123c9a0, "std_14123c9a0", fn_14123c9a0 as *const (), Throws::Discard), // sub_14123c9a0
    Replacement::draft_throws(0x14123cce0, "std_14123cce0", fn_14123cce0 as *const (), Throws::Discard), // sub_14123cce0
    Replacement::draft(0x14123cdb0, "unknown_14123cdb0", fn_14123cdb0 as *const ()), // sub_14123cdb0
    Replacement::draft_throws(0x14123cee0, "unknown_14123cee0", fn_14123cee0 as *const (), Throws::Discard), // sub_14123cee0
    Replacement::draft(0x14123d5c0, "unknown_14123d5c0", fn_14123d5c0 as *const ()), // sub_14123d5c0
    Replacement::draft(0x14123d660, "unknown_14123d660", fn_14123d660 as *const ()), // sub_14123d660
    Replacement::draft(0x14123d7a0, "unknown_14123d7a0", fn_14123d7a0 as *const ()), // sub_14123d7a0
    Replacement::draft(0x14123d850, "unknown_14123d850", fn_14123d850 as *const ()), // sub_14123d850
    Replacement::draft(0x14123d910, "unknown_14123d910", fn_14123d910 as *const ()), // sub_14123d910
    Replacement::draft(0x14123d940, "unknown_14123d940", fn_14123d940 as *const ()), // sub_14123d940
    Replacement::draft(0x14123d9d0, "unknown_14123d9d0", fn_14123d9d0 as *const ()), // sub_14123d9d0
    Replacement::draft(0x14123da60, "unknown_14123da60", fn_14123da60 as *const ()), // sub_14123da60
    Replacement::draft_throws(0x14123dbe0, "physics_14123dbe0", tpl_22293::<0x50, 0x14241c190, 0x58, 0x40, 0x48> as *const (), Throws::Discard), // sub_14123dbe0
    Replacement::draft_throws(0x14123dce0, "physics_14123dce0", tpl_22829::<0x8, 0xc, 0x14123e720> as *const (), Throws::Discard), // sub_14123dce0
    Replacement::draft_throws(0x14123dd40, "physics_14123dd40", tpl_22829::<0x8, 0xc, 0x14123e7b0> as *const (), Throws::Discard), // sub_14123dd40
    Replacement::draft(0x14123dda0, "physics_14123dda0", fn_14123dda0 as *const ()), // sub_14123dda0
    Replacement::draft(0x14123de30, "physics_14123de30", fn_14123de30 as *const ()), // sub_14123de30
    Replacement::draft_throws(0x14123e140, "physics_14123e140", fn_14123e140 as *const (), Throws::Discard), // sub_14123e140
    Replacement::draft(0x14123e490, "physics_14123e490", fn_14123e490 as *const ()), // sub_14123e490
    Replacement::draft_throws(0x14123e6a0, "physics_14123e6a0", fn_14123e6a0 as *const (), Throws::Discard), // sub_14123e6a0
    Replacement::draft_throws(0x14123e720, "physics_14123e720", fn_14123e720 as *const (), Throws::Discard), // sub_14123e720
    Replacement::draft_throws(0x14123e7b0, "physics_14123e7b0", fn_14123e7b0 as *const (), Throws::Discard), // sub_14123e7b0
    Replacement::draft_throws(0x14123e860, "physics_14123e860", fn_14123e860 as *const (), Throws::Discard), // sub_14123e860
    Replacement::draft(0x14123e900, "physics_14123e900", fn_14123e900 as *const ()), // sub_14123e900
    Replacement::draft(0x14123ec30, "physics_14123ec30", fn_14123ec30 as *const ()), // sub_14123ec30
    Replacement::draft(0x14123ecf0, "physics_14123ecf0", fn_14123ecf0 as *const ()), // sub_14123ecf0
    Replacement::draft_throws(0x14123f510, "physics_14123f510", fn_14123f510 as *const (), Throws::Discard), // sub_14123f510
    Replacement::draft(0x14123f700, "physics_14123f700", fn_14123f700 as *const ()), // sub_14123f700
    Replacement::draft(0x14123f980, "physics_14123f980", fn_14123f980 as *const ()), // sub_14123f980
    Replacement::draft(0x14123fb80, "physics_14123fb80", fn_14123fb80 as *const ()), // sub_14123fb80
    Replacement::draft(0x14123fd60, "physics_14123fd60", fn_14123fd60 as *const ()), // sub_14123fd60
    Replacement::draft(0x14123fe70, "physics_14123fe70", fn_14123fe70 as *const ()), // sub_14123fe70
    Replacement::draft(0x14123ff80, "physics_14123ff80", fn_14123ff80 as *const ()), // sub_14123ff80
    Replacement::draft_throws(0x141240020, "physics_141240020", fn_141240020 as *const (), Throws::Discard), // sub_141240020
    Replacement::draft(0x141240150, "physics_141240150", fn_141240150 as *const ()), // physics::RagdollProfileResource::physics::RagdollProfileResource::vfunc_0
    Replacement::draft(0x1412401a0, "physics_1412401a0", fn_1412401a0 as *const ()), // sub_1412401a0
    Replacement::draft(0x141240270, "physics_141240270", fn_141240270 as *const ()), // sub_141240270
    Replacement::draft(0x141240350, "physics_141240350", fn_141240350 as *const ()), // sub_141240350
    Replacement::draft_throws(0x141240420, "physics_141240420", fn_141240420 as *const (), Throws::Discard), // sub_141240420
    Replacement::draft(0x141240790, "physics_141240790", fn_141240790 as *const ()), // sub_141240790
    Replacement::draft_throws(0x141240a70, "physics_141240a70", fn_141240a70 as *const (), Throws::Discard), // sub_141240a70
    Replacement::draft(0x141240b70, "physics_141240b70", fn_141240b70 as *const ()), // sub_141240b70
    Replacement::draft_throws(0x141240d30, "physics_141240d30", fn_141240d30 as *const (), Throws::Caught), // sub_141240d30
    Replacement::draft_throws(0x141240ea0, "physics_141240ea0", fn_141240ea0 as *const (), Throws::Caught), // sub_141240ea0
    Replacement::draft_throws(0x141241010, "physics_141241010", fn_141241010 as *const (), Throws::Caught), // sub_141241010
    Replacement::draft(0x141241180, "physics_141241180", fn_141241180 as *const ()), // sub_141241180
    Replacement::draft(0x141241430, "physics_141241430", fn_141241430 as *const ()), // sub_141241430
    Replacement::draft(0x1412416c0, "physics_1412416c0", fn_1412416c0 as *const ()), // sub_1412416c0
    Replacement::draft_throws(0x141241960, "physics_141241960", fn_141241960 as *const (), Throws::Discard), // sub_141241960
    Replacement::draft_throws(0x141241c70, "physics_141241c70", fn_141241c70 as *const (), Throws::Discard), // sub_141241c70
    Replacement::draft(0x141241de0, "physics_141241de0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433e2c88, 0x8> as *const ()), // sub_141241de0
    Replacement::draft(0x141241e60, "physics_141241e60", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433e2c88, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_141241e60
    Replacement::draft_throws(0x141241f00, "physics_141241f00", fn_141241f00 as *const (), Throws::Caught), // sub_141241f00
    Replacement::draft_throws(0x141241f90, "physics_141241f90", tpl_1908::<0x143881678, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_141241f90
    Replacement::draft(0x141242010, "physics_141242010", fn_141242010 as *const ()), // sub_141242010
    Replacement::draft(0x141242090, "physics_141242090", fn_141242090 as *const ()), // sub_141242090
    Replacement::draft_throws(0x1412422d0, "physics_1412422d0", fn_1412422d0 as *const (), Throws::Caught), // sub_1412422d0
    Replacement::draft_throws(0x141242460, "physics_141242460", tpl_1905::<0x143881668, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_141242460
    Replacement::draft(0x141242510, "physics_141242510", fn_141242510 as *const ()), // sub_141242510
    Replacement::draft(0x141242880, "physics_141242880", fn_141242880 as *const ()), // sub_141242880
    Replacement::draft(0x141242a20, "physics_141242a20", fn_141242a20 as *const ()), // sub_141242a20
    Replacement::draft(0x141242b50, "physx_141242b50", tpl_893::<0x1433e2fb0, 0x141834d30> as *const ()), // physx::PxCpuDispatcher::physx::PxCpuDispatcher::vfunc_2
    Replacement::draft(0x141242b80, "unknown_141242b80", fn_141242b80 as *const ()), // sub_141242b80
    Replacement::draft(0x141242d40, "physx_141242d40", tpl_893::<0x1433e3028, 0x141834d30> as *const ()), // physx::PxSimulationEventCallback::physx::PxSimulationEventCallback::vfunc_6
    Replacement::draft(0x141242d70, "physics_141242d70", tpl_586::<0x80> as *const ()), // physics::PhysXTriggerScene::physics::PhysXTriggerScene::vfunc_6
    Replacement::draft(0x141242d90, "physics_141242d90", fn_141242d90 as *const ()), // physics::PhysXTriggerScene::physics::PhysXTriggerScene::vfunc_0
    Replacement::draft(0x141242e50, "physics_141242e50", fn_141242e50 as *const ()), // physics::PhysXSimulationEventCallback::physics::PhysXSimulationEventCallback::vfunc_6
    Replacement::draft(0x141242ec0, "unknown_141242ec0", fn_141242ec0 as *const ()), // sub_141242ec0
    Replacement::draft(0x141243000, "unknown_141243000", tpl_66202::<0x8, 0xc, 0x1408746b0, 0x14183acc2> as *const ()), // sub_141243000
    Replacement::draft(0x1412430c0, "unknown_1412430c0", fn_1412430c0 as *const ()), // sub_1412430c0
    Replacement::draft(0x141243230, "physx_141243230", tpl_22865::<0x1433e30e8, 0x141834d30> as *const ()), // physx::PxBaseTask::physx::PxBaseTask::vfunc_0
    Replacement::draft(0x141243260, "physx_141243260", tpl_22866::<0x10, 0x90> as *const ()), // physx::PxLightCpuTask::physx::PxLightCpuTask::vfunc_4
    Replacement::draft(0x141243280, "physx_141243280", tpl_22866::<0x10, 0x80> as *const ()), // physx::PxLightCpuTask::physx::PxLightCpuTask::vfunc_3
    Replacement::draft(0x1412432a0, "physx_1412432a0", tpl_22867::<0x18, 0x20> as *const ()), // physx::PxLightCpuTask::physx::PxLightCpuTask::vfunc_6
    Replacement::draft(0x1412432c0, "physx_1412432c0", fn_1412432c0 as *const ()), // physx::PxLightCpuTask::physx::PxLightCpuTask::vfunc_0
    Replacement::draft(0x141243300, "physx_141243300", tpl_893::<0x1433e31e8, 0x141834d30> as *const ()), // physx::PxContactModifyCallback::physx::PxContactModifyCallback::vfunc_1
    Replacement::draft(0x141243330, "physx_141243330", tpl_893::<0x1433e31c8, 0x141834d30> as *const ()), // physx::PxBroadPhaseCallback::physx::PxBroadPhaseCallback::vfunc_0
    Replacement::draft(0x141243360, "unknown_141243360", fn_141243360 as *const ()), // sub_141243360
    Replacement::draft(0x141243440, "physics_141243440", tpl_586::<0xec> as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_6
    Replacement::draft(0x141243460, "physics_141243460", fn_141243460 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_0
    Replacement::draft(0x1412434a0, "physics_1412434a0", fn_1412434a0 as *const ()), // physics::PhysXCompletionTask::physics::PhysXCompletionTask::vfunc_0
    Replacement::draft(0x141243510, "physics_141243510", fn_141243510 as *const ()), // sub_141243510
    Replacement::draft(0x141243590, "physics_141243590", tpl_3652::<0xc, 0x8, 0x1408746b0> as *const ()), // sub_141243590
    Replacement::draft(0x141243600, "physics_141243600", fn_141243600 as *const ()), // sub_141243600
    Replacement::draft(0x141243950, "physics_141243950", fn_141243950 as *const ()), // sub_141243950
    Replacement::draft(0x141243b10, "physics_141243b10", fn_141243b10 as *const ()), // sub_141243b10
    Replacement::draft(0x141243f60, "physics_141243f60", fn_141243f60 as *const ()), // sub_141243f60
    Replacement::draft(0x141244010, "physics_141244010", fn_141244010 as *const ()), // physics::PhysXClothResource::physics::PhysXClothResource::vfunc_0
    Replacement::draft(0x141244090, "unknown_141244090", fn_141244090 as *const ()), // sub_141244090
    Replacement::draft(0x141244170, "unknown_141244170", fn_141244170 as *const ()), // sub_141244170
    Replacement::draft(0x141244260, "unknown_141244260", fn_141244260 as *const ()), // sub_141244260
    Replacement::draft(0x1412446b0, "unknown_1412446b0", tpl_13457::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x9, 0x1433e3fe8, 0xd, 0xf, 0x8> as *const ()), // sub_1412446b0
    Replacement::draft(0x141244750, "unknown_141244750", tpl_13458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1433e3fe8, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_141244750
    Replacement::draft(0x141244800, "unknown_141244800", fn_141244800 as *const ()), // sub_141244800
    Replacement::draft_throws(0x141244880, "unknown_141244880", fn_141244880 as *const (), Throws::Discard), // sub_141244880
    Replacement::draft(0x1412449b0, "unknown_1412449b0", fn_1412449b0 as *const ()), // sub_1412449b0
    Replacement::draft_throws(0x141244ac0, "unknown_141244ac0", fn_141244ac0 as *const (), Throws::Discard), // sub_141244ac0
    Replacement::draft(0x141244b40, "unknown_141244b40", fn_141244b40 as *const ()), // sub_141244b40
    Replacement::draft_throws(0x141244bd0, "unknown_141244bd0", fn_141244bd0 as *const (), Throws::Caught), // sub_141244bd0
    Replacement::draft_throws(0x141244ca0, "unknown_141244ca0", fn_141244ca0 as *const (), Throws::Caught), // sub_141244ca0
    Replacement::draft_throws(0x141244d70, "unknown_141244d70", fn_141244d70 as *const (), Throws::Caught), // sub_141244d70
    Replacement::draft(0x141244e40, "unknown_141244e40", fn_141244e40 as *const ()), // sub_141244e40
    Replacement::draft(0x141244f50, "unknown_141244f50", fn_141244f50 as *const ()), // sub_141244f50
    Replacement::draft(0x141245030, "unknown_141245030", fn_141245030 as *const ()), // sub_141245030
    Replacement::draft_throws(0x141245120, "unknown_141245120", fn_141245120 as *const (), Throws::Discard), // sub_141245120
    Replacement::draft_throws(0x141245230, "unknown_141245230", fn_141245230 as *const (), Throws::Discard), // sub_141245230
    Replacement::draft(0x1412452f0, "unknown_1412452f0", fn_1412452f0 as *const ()), // sub_1412452f0
    Replacement::draft(0x141245370, "unknown_141245370", fn_141245370 as *const ()), // sub_141245370
    Replacement::draft(0x141245410, "unknown_141245410", tpl_6383::<0x143881688, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_141245410
    Replacement::draft_throws(0x1412454b0, "unknown_1412454b0", tpl_6384::<0x143881684, 0x143881688, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412454b0
    Replacement::draft_throws(0x141245530, "unknown_141245530", tpl_6384::<0x143881684, 0x143881688, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_141245530
    Replacement::draft_throws(0x1412455b0, "unknown_1412455b0", fn_1412455b0 as *const (), Throws::Caught), // sub_1412455b0
    Replacement::draft(0x141245640, "unknown_141245640", tpl_6386::<0x143881684, 0x143881688, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_141245640
    Replacement::draft_throws(0x1412456c0, "unknown_1412456c0", tpl_6384::<0x143881684, 0x143881688, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1412456c0
    Replacement::draft_throws(0x141245740, "unknown_141245740", tpl_6387::<0x143881688, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_141245740
    Replacement::draft_throws(0x1412457f0, "unknown_1412457f0", fn_1412457f0 as *const (), Throws::Caught), // sub_1412457f0
    Replacement::draft(0x141246120, "unknown_141246120", fn_141246120 as *const ()), // sub_141246120
    Replacement::draft(0x1412461e0, "unknown_1412461e0", tpl_72099::<0x8, 0xc, 0x140a5bd90, 0x14183acc2> as *const ()), // sub_1412461e0
    Replacement::draft(0x1412462a0, "unknown_1412462a0", fn_1412462a0 as *const ()), // sub_1412462a0
    Replacement::draft_throws(0x141246390, "unknown_141246390", fn_141246390 as *const (), Throws::Caught), // sub_141246390
    Replacement::draft_throws(0x141246660, "unknown_141246660", fn_141246660 as *const (), Throws::Caught), // sub_141246660
    Replacement::draft_throws(0x141246950, "unknown_141246950", fn_141246950 as *const (), Throws::Caught), // sub_141246950
    Replacement::draft(0x141246c20, "unknown_141246c20", fn_141246c20 as *const ()), // sub_141246c20
    Replacement::draft_throws(0x141246d20, "unknown_141246d20", tpl_22010::<0x143881718, 0x143881710, 0x1401e9fa0, 0x328, 0x338, 0x38, 0x369, 0x4, 0x8, 0x1401e9e70> as *const (), Throws::Discard), // sub_141246d20
    Replacement::draft(0x141246e00, "unknown_141246e00", tpl_22014::<0x143881710, 0x14243b0b0, 0x338, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141246e00
    Replacement::draft_throws(0x141246e90, "unknown_141246e90", fn_141246e90 as *const (), Throws::Discard), // sub_141246e90
    Replacement::draft(0x141246f90, "unknown_141246f90", fn_141246f90 as *const ()), // sub_141246f90
    Replacement::draft_throws(0x141247050, "unknown_141247050", fn_141247050 as *const (), Throws::Discard), // sub_141247050
    Replacement::draft(0x141247150, "unknown_141247150", fn_141247150 as *const ()), // sub_141247150
    Replacement::draft(0x1412471f0, "physx_1412471f0", tpl_893::<0x1433e4098, 0x141834d30> as *const ()), // physx::PxAllocatorCallback::physx::PxAllocatorCallback::vfunc_0
    Replacement::draft(0x141247220, "physx_141247220", tpl_893::<0x1433e4110, 0x141834d30> as *const ()), // physx::PxAssertHandler::physx::PxAssertHandler::vfunc_0
    Replacement::draft(0x141247250, "physx_141247250", tpl_893::<0x1433e40b8, 0x141834d30> as *const ()), // physx::PxErrorCallback::physx::PxErrorCallback::vfunc_0
    Replacement::draft(0x141247280, "unknown_141247280", fn_141247280 as *const ()), // sub_141247280
    Replacement::draft(0x141247380, "unknown_141247380", fn_141247380 as *const ()), // sub_141247380
    Replacement::draft(0x141247400, "unknown_141247400", fn_141247400 as *const ()), // sub_141247400
    Replacement::draft(0x141247500, "unknown_141247500", tpl_13191::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x20, 0x28> as *const ()), // sub_141247500
    Replacement::draft(0x141247590, "unknown_141247590", fn_141247590 as *const ()), // sub_141247590
    Replacement::draft(0x141247660, "unknown_141247660", fn_141247660 as *const ()), // sub_141247660
    Replacement::draft(0x141247830, "unknown_141247830", fn_141247830 as *const ()), // sub_141247830
    Replacement::draft(0x1412478f0, "unknown_1412478f0", tpl_2083::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412478f0
    Replacement::draft(0x141247980, "physics_141247980", fn_141247980 as *const ()), // sub_141247980
    Replacement::draft(0x141247c90, "physics_141247c90", tpl_22904::<0x60> as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_13
    Replacement::draft(0x141247cb0, "physics_141247cb0", fn_141247cb0 as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_19
    Replacement::draft(0x141247cc0, "ak_141247cc0", tpl_1691::<0x18> as *const ()), // AK::ReadBytesSkip::AK::ReadBytesSkip::Count
    Replacement::draft(0x141247cd0, "physics_141247cd0", fn_141247cd0 as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_0
    Replacement::draft(0x141247d90, "unknown_141247d90", fn_141247d90 as *const ()), // sub_141247d90
    Replacement::draft(0x141247ee0, "unknown_141247ee0", fn_141247ee0 as *const ()), // sub_141247ee0
    Replacement::draft(0x141248250, "unknown_141248250", fn_141248250 as *const ()), // sub_141248250
    Replacement::draft(0x141248550, "unknown_141248550", fn_141248550 as *const ()), // sub_141248550
    Replacement::draft(0x141248660, "unknown_141248660", fn_141248660 as *const ()), // sub_141248660
    Replacement::draft(0x141248720, "unknown_141248720", fn_141248720 as *const ()), // sub_141248720
    Replacement::draft(0x141248850, "physx_141248850", tpl_893::<0x1433e4400, 0x141834d30> as *const ()), // physx::PxQueryFilterCallback::physx::PxQueryFilterCallback::vfunc_2
    Replacement::draft(0x141248880, "physics_141248880", fn_141248880 as *const ()), // physics::PhysXFilterCallback::physics::PhysXFilterCallback::vfunc_0
    Replacement::draft(0x141248960, "physics_141248960", fn_141248960 as *const ()), // physics::PhysXFilterCallback::physics::PhysXFilterCallback::vfunc_1
    Replacement::draft(0x141248ab0, "physx_141248ab0", tpl_1674::<0x1433e4400> as *const ()), // sub_141248ab0
    Replacement::draft(0x141248ac0, "physx_141248ac0", fn_141248ac0 as *const ()), // sub_141248ac0
    Replacement::draft(0x141248da0, "physx_141248da0", tpl_1674::<0x1433e4480> as *const ()), // sub_141248da0
    Replacement::draft(0x141248db0, "physx_141248db0", tpl_1674::<0x1433e4420> as *const ()), // sub_141248db0
    Replacement::draft(0x141248dc0, "physx_141248dc0", tpl_22914::<0x1433e4480, 0x141834d30> as *const ()), // physx::PxHitCallback<physx::PxSweepHit>::physx::PxHitCallback<physx::PxSweepHit>::vfunc_2
    Replacement::draft(0x141248df0, "physx_141248df0", tpl_21733::<0x1433e44c0, 0x141834d30> as *const ()), // physx::PxHitCallback<physx::PxOverlapHit>::physx::PxHitCallback<physx::PxOverlapHit>::vfunc_2
    Replacement::draft(0x141248e20, "physx_141248e20", tpl_22914::<0x1433e4420, 0x141834d30> as *const ()), // physx::PxHitCallback<physx::PxRaycastHit>::physx::PxHitCallback<physx::PxRaycastHit>::vfunc_2
    Replacement::draft(0x141248e50, "unknown_141248e50", fn_141248e50 as *const ()), // sub_141248e50
    Replacement::draft_throws(0x141248f90, "unknown_141248f90", fn_141248f90 as *const (), Throws::Discard), // sub_141248f90
    Replacement::draft(0x141249060, "unknown_141249060", fn_141249060 as *const ()), // sub_141249060
    Replacement::draft(0x141249430, "unknown_141249430", fn_141249430 as *const ()), // sub_141249430
    Replacement::draft(0x1412494a0, "physx_1412494a0", tpl_893::<0x1433e4518, 0x141834d30> as *const ()), // physx::PxInputStream::physx::PxInputStream::vfunc_1
    Replacement::draft(0x1412494d0, "physx_1412494d0", tpl_893::<0x1433e4548, 0x141834d30> as *const ()), // physx::PxOutputStream::physx::PxOutputStream::vfunc_1
    Replacement::draft(0x141249500, "ak_141249500", tpl_913::<0x10> as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::Bytes
    Replacement::draft(0x141249510, "unknown_141249510", fn_141249510 as *const ()), // sub_141249510
    Replacement::draft(0x141249680, "unknown_141249680", fn_141249680 as *const ()), // sub_141249680
    Replacement::draft(0x141249790, "physx_141249790", tpl_1674::<0x1433e4518> as *const ()), // sub_141249790
    Replacement::draft(0x1412497a0, "unknown_1412497a0", tpl_19878::<0x168, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142153640> as *const ()), // sub_1412497a0
    Replacement::draft(0x141249830, "unknown_141249830", fn_141249830 as *const ()), // sub_141249830
    Replacement::draft(0x1412498f0, "unknown_1412498f0", fn_1412498f0 as *const ()), // sub_1412498f0
    Replacement::draft_throws(0x141249960, "unknown_141249960", fn_141249960 as *const (), Throws::Discard), // sub_141249960
    Replacement::draft_throws(0x141249aa0, "unknown_141249aa0", fn_141249aa0 as *const (), Throws::Caught), // sub_141249aa0
    Replacement::draft_throws(0x141249cd0, "unknown_141249cd0", fn_141249cd0 as *const (), Throws::Discard), // sub_141249cd0
    Replacement::draft_throws(0x141249d90, "unknown_141249d90", fn_141249d90 as *const (), Throws::Caught), // sub_141249d90
    Replacement::draft(0x141249ff0, "unknown_141249ff0", fn_141249ff0 as *const ()), // sub_141249ff0
    Replacement::draft_throws(0x14124a080, "unknown_14124a080", fn_14124a080 as *const (), Throws::Caught), // sub_14124a080
    Replacement::draft_throws(0x14124a200, "unknown_14124a200", fn_14124a200 as *const (), Throws::Caught), // sub_14124a200
    Replacement::draft_throws(0x14124a380, "unknown_14124a380", fn_14124a380 as *const (), Throws::Caught), // sub_14124a380
    Replacement::draft(0x14124a500, "unknown_14124a500", fn_14124a500 as *const ()), // sub_14124a500
    Replacement::draft(0x14124a5c0, "unknown_14124a5c0", fn_14124a5c0 as *const ()), // sub_14124a5c0
    Replacement::draft(0x14124a680, "unknown_14124a680", fn_14124a680 as *const ()), // sub_14124a680
    Replacement::draft_throws(0x14124a750, "unknown_14124a750", fn_14124a750 as *const (), Throws::Discard), // sub_14124a750
    Replacement::draft_throws(0x14124a830, "unknown_14124a830", fn_14124a830 as *const (), Throws::Caught), // sub_14124a830
    Replacement::draft(0x14124a9f0, "unknown_14124a9f0", fn_14124a9f0 as *const ()), // sub_14124a9f0
    Replacement::draft(0x14124aab0, "unknown_14124aab0", tpl_12678::<0x8, 0x14124b4c0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14124aab0
    Replacement::draft_throws(0x14124ab70, "unknown_14124ab70", fn_14124ab70 as *const (), Throws::Caught), // sub_14124ab70
    Replacement::draft_throws(0x14124acb0, "unknown_14124acb0", fn_14124acb0 as *const (), Throws::Caught), // sub_14124acb0
    Replacement::draft_throws(0x14124adf0, "unknown_14124adf0", fn_14124adf0 as *const (), Throws::Caught), // sub_14124adf0
    Replacement::draft_throws(0x14124af30, "unknown_14124af30", fn_14124af30 as *const (), Throws::Discard), // sub_14124af30
    Replacement::draft(0x14124b200, "unknown_14124b200", fn_14124b200 as *const ()), // sub_14124b200
    Replacement::draft(0x14124b460, "unknown_14124b460", fn_14124b460 as *const ()), // sub_14124b460
    Replacement::draft(0x14124b4c0, "unknown_14124b4c0", fn_14124b4c0 as *const ()), // sub_14124b4c0
    Replacement::draft(0x14124b520, "unknown_14124b520", fn_14124b520 as *const ()), // sub_14124b520
    Replacement::draft(0x14124b5a0, "unknown_14124b5a0", fn_14124b5a0 as *const ()), // sub_14124b5a0
    Replacement::draft(0x14124baa0, "unknown_14124baa0", fn_14124baa0 as *const ()), // sub_14124baa0
    Replacement::draft_throws(0x14124bfa0, "unknown_14124bfa0", fn_14124bfa0 as *const (), Throws::Caught), // sub_14124bfa0
    Replacement::draft_throws(0x14124c550, "unknown_14124c550", fn_14124c550 as *const (), Throws::Caught), // sub_14124c550
    Replacement::draft_throws(0x14124cac0, "unknown_14124cac0", fn_14124cac0 as *const (), Throws::Caught), // sub_14124cac0
    Replacement::draft_throws(0x14124cf00, "unknown_14124cf00", fn_14124cf00 as *const (), Throws::Caught), // sub_14124cf00
    Replacement::draft(0x14124d0c0, "unknown_14124d0c0", fn_14124d0c0 as *const ()), // sub_14124d0c0
    Replacement::draft_throws(0x14124d170, "unknown_14124d170", fn_14124d170 as *const (), Throws::Caught), // sub_14124d170
    Replacement::draft_throws(0x14124d330, "unknown_14124d330", fn_14124d330 as *const (), Throws::Caught), // sub_14124d330
    Replacement::draft(0x14124d4f0, "unknown_14124d4f0", tpl_10174::<0x8, 0x142154920> as *const ()), // sub_14124d4f0
    Replacement::draft(0x14124d530, "unknown_14124d530", tpl_81305::<0x8, 0xc, 0x14124b200, 0x141250270, 0x14124b460> as *const ()), // sub_14124d530
    Replacement::draft(0x14124d610, "unknown_14124d610", fn_14124d610 as *const ()), // sub_14124d610
    Replacement::draft(0x14124d6f0, "unknown_14124d6f0", fn_14124d6f0 as *const ()), // sub_14124d6f0
    Replacement::draft(0x14124d7f0, "unknown_14124d7f0", fn_14124d7f0 as *const ()), // sub_14124d7f0
    Replacement::draft(0x14124d970, "unknown_14124d970", fn_14124d970 as *const ()), // sub_14124d970
    Replacement::draft_throws(0x14124dc30, "unknown_14124dc30", fn_14124dc30 as *const (), Throws::Discard), // sub_14124dc30
    Replacement::draft_throws(0x14124dea0, "unknown_14124dea0", fn_14124dea0 as *const (), Throws::Discard), // sub_14124dea0
    Replacement::draft(0x14124e270, "unknown_14124e270", fn_14124e270 as *const ()), // sub_14124e270
    Replacement::draft_throws(0x14124e680, "unknown_14124e680", fn_14124e680 as *const (), Throws::Discard), // sub_14124e680
    Replacement::draft(0x14124eaf0, "unknown_14124eaf0", fn_14124eaf0 as *const ()), // sub_14124eaf0
    Replacement::draft(0x14124ee40, "unknown_14124ee40", fn_14124ee40 as *const ()), // sub_14124ee40
    Replacement::draft(0x14124f280, "unknown_14124f280", fn_14124f280 as *const ()), // sub_14124f280
    Replacement::draft_throws(0x14124f6c0, "unknown_14124f6c0", fn_14124f6c0 as *const (), Throws::Discard), // sub_14124f6c0
    Replacement::draft_throws(0x14124fb50, "unknown_14124fb50", fn_14124fb50 as *const (), Throws::Caught), // sub_14124fb50
    Replacement::draft(0x14124fe40, "unknown_14124fe40", fn_14124fe40 as *const ()), // sub_14124fe40
    Replacement::draft(0x14124fed0, "unknown_14124fed0", fn_14124fed0 as *const ()), // sub_14124fed0
    Replacement::draft(0x14124ff70, "unknown_14124ff70", fn_14124ff70 as *const ()), // sub_14124ff70
    Replacement::draft(0x14124ffc0, "unknown_14124ffc0", fn_14124ffc0 as *const ()), // sub_14124ffc0
    Replacement::draft(0x141250010, "unknown_141250010", fn_141250010 as *const ()), // sub_141250010
    Replacement::draft(0x141250270, "unknown_141250270", fn_141250270 as *const ()), // sub_141250270
    Replacement::draft(0x141250300, "unknown_141250300", fn_141250300 as *const ()), // sub_141250300
    Replacement::draft(0x1412503d0, "unknown_1412503d0", fn_1412503d0 as *const ()), // sub_1412503d0
    Replacement::draft_throws(0x141250480, "unknown_141250480", fn_141250480 as *const (), Throws::Caught), // sub_141250480
    Replacement::draft_throws(0x141250630, "unknown_141250630", fn_141250630 as *const (), Throws::Caught), // sub_141250630
    Replacement::draft_throws(0x1412507e0, "unknown_1412507e0", fn_1412507e0 as *const (), Throws::Caught), // sub_1412507e0
    Replacement::draft_throws(0x141250990, "unknown_141250990", fn_141250990 as *const (), Throws::Caught), // sub_141250990
    Replacement::draft_throws(0x141250b70, "unknown_141250b70", fn_141250b70 as *const (), Throws::Caught), // sub_141250b70
    Replacement::draft_throws(0x141250d50, "unknown_141250d50", fn_141250d50 as *const (), Throws::Caught), // sub_141250d50
    Replacement::draft_throws(0x141250f30, "unknown_141250f30", fn_141250f30 as *const (), Throws::Discard), // sub_141250f30
    Replacement::draft(0x141251110, "unknown_141251110", fn_141251110 as *const ()), // sub_141251110
    Replacement::draft_throws(0x1412513f0, "unknown_1412513f0", fn_1412513f0 as *const (), Throws::Caught), // sub_1412513f0
    Replacement::draft(0x1412516e0, "unknown_1412516e0", fn_1412516e0 as *const ()), // sub_1412516e0
    Replacement::draft_throws(0x141251b80, "unknown_141251b80", tpl_838::<0x141251e00> as *const (), Throws::Discard), // sub_141251b80
    Replacement::draft_throws(0x141251b90, "unknown_141251b90", tpl_838::<0x141252100> as *const (), Throws::Discard), // sub_141251b90
    Replacement::draft_throws(0x141251ba0, "unknown_141251ba0", tpl_838::<0x1412523c0> as *const (), Throws::Discard), // sub_141251ba0
    Replacement::draft_throws(0x141251bb0, "unknown_141251bb0", tpl_838::<0x1412526c0> as *const (), Throws::Discard), // sub_141251bb0
    Replacement::draft_throws(0x141251bc0, "unknown_141251bc0", tpl_838::<0x1412529a0> as *const (), Throws::Discard), // sub_141251bc0
    Replacement::draft_throws(0x141251bd0, "unknown_141251bd0", tpl_838::<0x141252e00> as *const (), Throws::Discard), // sub_141251bd0
    Replacement::draft_throws(0x141251be0, "unknown_141251be0", tpl_838::<0x1412530c0> as *const (), Throws::Discard), // sub_141251be0
    Replacement::draft_throws(0x141251bf0, "unknown_141251bf0", tpl_838::<0x141253350> as *const (), Throws::Discard), // sub_141251bf0
    Replacement::draft_throws(0x141251c00, "unknown_141251c00", tpl_838::<0x141253610> as *const (), Throws::Discard), // sub_141251c00
    Replacement::draft_throws(0x141251c10, "unknown_141251c10", tpl_838::<0x1412538c0> as *const (), Throws::Discard), // sub_141251c10
    Replacement::draft_throws(0x141251c20, "unknown_141251c20", tpl_838::<0x141253cf0> as *const (), Throws::Discard), // sub_141251c20
    Replacement::draft_throws(0x141251c30, "unknown_141251c30", tpl_838::<0x141253f90> as *const (), Throws::Discard), // sub_141251c30
    Replacement::draft_throws(0x141251c40, "unknown_141251c40", tpl_838::<0x141254220> as *const (), Throws::Discard), // sub_141251c40
    Replacement::draft_throws(0x141251c50, "unknown_141251c50", tpl_838::<0x1412544c0> as *const (), Throws::Discard), // sub_141251c50
    Replacement::draft_throws(0x141251c60, "unknown_141251c60", tpl_838::<0x141254760> as *const (), Throws::Discard), // sub_141251c60
    Replacement::draft_throws(0x141251c70, "unknown_141251c70", tpl_22963::<0x20, 0x8, 0x141254b90> as *const (), Throws::Discard), // sub_141251c70
    Replacement::draft_throws(0x141251cc0, "unknown_141251cc0", fn_141251cc0 as *const (), Throws::Discard), // sub_141251cc0
    Replacement::draft_throws(0x141251d10, "unknown_141251d10", tpl_12639::<0x20, 0x8, 0x141254c60> as *const (), Throws::Discard), // sub_141251d10
    Replacement::draft_throws(0x141251d60, "unknown_141251d60", tpl_6985::<0x20, 0x8, 0x141254d30> as *const (), Throws::Discard), // sub_141251d60
    Replacement::draft_throws(0x141251db0, "unknown_141251db0", tpl_6980::<0x20, 0x8, 0x141254e00> as *const (), Throws::Discard), // sub_141251db0
    Replacement::draft_throws(0x141251e00, "unknown_141251e00", fn_141251e00 as *const (), Throws::Caught), // sub_141251e00
    Replacement::draft_throws(0x141252100, "unknown_141252100", fn_141252100 as *const (), Throws::Caught), // sub_141252100
    Replacement::draft_throws(0x1412523c0, "unknown_1412523c0", fn_1412523c0 as *const (), Throws::Caught), // sub_1412523c0
    Replacement::draft_throws(0x1412526c0, "unknown_1412526c0", fn_1412526c0 as *const (), Throws::Caught), // sub_1412526c0
    Replacement::draft_throws(0x1412529a0, "unknown_1412529a0", fn_1412529a0 as *const (), Throws::Caught), // sub_1412529a0
    Replacement::draft_throws(0x141252e00, "unknown_141252e00", fn_141252e00 as *const (), Throws::Caught), // sub_141252e00
    Replacement::draft_throws(0x1412530c0, "unknown_1412530c0", fn_1412530c0 as *const (), Throws::Caught), // sub_1412530c0
    Replacement::draft_throws(0x141253350, "unknown_141253350", fn_141253350 as *const (), Throws::Caught), // sub_141253350
    Replacement::draft_throws(0x141253610, "unknown_141253610", fn_141253610 as *const (), Throws::Caught), // sub_141253610
    Replacement::draft_throws(0x1412538c0, "unknown_1412538c0", fn_1412538c0 as *const (), Throws::Caught), // sub_1412538c0
    Replacement::draft_throws(0x141253cf0, "unknown_141253cf0", fn_141253cf0 as *const (), Throws::Caught), // sub_141253cf0
    Replacement::draft_throws(0x141253f90, "unknown_141253f90", fn_141253f90 as *const (), Throws::Caught), // sub_141253f90
    Replacement::draft_throws(0x141254220, "unknown_141254220", fn_141254220 as *const (), Throws::Caught), // sub_141254220
    Replacement::draft_throws(0x1412544c0, "unknown_1412544c0", fn_1412544c0 as *const (), Throws::Caught), // sub_1412544c0
    Replacement::draft_throws(0x141254760, "unknown_141254760", fn_141254760 as *const (), Throws::Caught), // sub_141254760
    Replacement::draft_throws(0x141254b90, "unknown_141254b90", fn_141254b90 as *const (), Throws::Caught), // sub_141254b90
    Replacement::draft_throws(0x141254c60, "unknown_141254c60", fn_141254c60 as *const (), Throws::Caught), // sub_141254c60
    Replacement::draft_throws(0x141254d30, "unknown_141254d30", fn_141254d30 as *const (), Throws::Caught), // sub_141254d30
    Replacement::draft_throws(0x141254e00, "unknown_141254e00", fn_141254e00 as *const (), Throws::Caught), // sub_141254e00
    Replacement::draft_throws(0x141254ed0, "unknown_141254ed0", fn_141254ed0 as *const (), Throws::Discard), // sub_141254ed0
    Replacement::draft(0x141255210, "unknown_141255210", fn_141255210 as *const ()), // sub_141255210
    Replacement::draft_throws(0x1412554a0, "unknown_1412554a0", fn_1412554a0 as *const (), Throws::Discard), // sub_1412554a0
    Replacement::draft(0x1412557b0, "unknown_1412557b0", fn_1412557b0 as *const ()), // sub_1412557b0
    Replacement::draft_throws(0x141255950, "unknown_141255950", tpl_22976::<0x143881f98, 0x143881f90, 0x1401e9b60, 0x328, 0x1433e49e8, 0x142418720, 0x1401e9a20> as *const (), Throws::Discard), // sub_141255950
    Replacement::draft(0x1412559c0, "unknown_1412559c0", fn_1412559c0 as *const ()), // sub_1412559c0
    Replacement::draft(0x141255a50, "unknown_141255a50", tpl_170::<0x14124a9f0> as *const ()), // sub_141255a50
    Replacement::draft(0x141255a60, "unknown_141255a60", tpl_170::<0x14124aab0> as *const ()), // sub_141255a60
    Replacement::draft(0x141255a70, "unknown_141255a70", fn_141255a70 as *const ()), // sub_141255a70
    Replacement::draft_throws(0x141255ba0, "unknown_141255ba0", fn_141255ba0 as *const (), Throws::Discard), // sub_141255ba0
    Replacement::draft(0x141255ee0, "unknown_141255ee0", fn_141255ee0 as *const ()), // sub_141255ee0
    Replacement::draft(0x141256250, "unknown_141256250", tpl_22981::<0x143881f90, 0x14243b0b0, 0x1402c4ad0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141256250
    Replacement::draft_throws(0x1412562c0, "unknown_1412562c0", fn_1412562c0 as *const (), Throws::Caught), // sub_1412562c0
    Replacement::draft_throws(0x141256790, "unknown_141256790", fn_141256790 as *const (), Throws::Caught), // sub_141256790
    Replacement::draft_throws(0x141256c60, "unknown_141256c60", fn_141256c60 as *const (), Throws::Caught), // sub_141256c60
    Replacement::draft_throws(0x141257130, "unknown_141257130", fn_141257130 as *const (), Throws::Caught), // sub_141257130
    Replacement::draft_throws(0x141257600, "unknown_141257600", fn_141257600 as *const (), Throws::Caught), // sub_141257600
    Replacement::draft_throws(0x141257a90, "unknown_141257a90", fn_141257a90 as *const (), Throws::Caught), // sub_141257a90
    Replacement::draft_throws(0x141257f20, "unknown_141257f20", fn_141257f20 as *const (), Throws::Caught), // sub_141257f20
    Replacement::draft_throws(0x1412583b0, "unknown_1412583b0", fn_1412583b0 as *const (), Throws::Caught), // sub_1412583b0
    Replacement::draft_throws(0x141258840, "unknown_141258840", fn_141258840 as *const (), Throws::Caught), // sub_141258840
    Replacement::draft_throws(0x141258cd0, "unknown_141258cd0", fn_141258cd0 as *const (), Throws::Caught), // sub_141258cd0
    Replacement::draft_throws(0x141259160, "unknown_141259160", fn_141259160 as *const (), Throws::Caught), // sub_141259160
    Replacement::draft_throws(0x1412595f0, "unknown_1412595f0", fn_1412595f0 as *const (), Throws::Caught), // sub_1412595f0
    Replacement::draft_throws(0x141259a80, "unknown_141259a80", fn_141259a80 as *const (), Throws::Discard), // sub_141259a80
    Replacement::draft_throws(0x141259db0, "unknown_141259db0", fn_141259db0 as *const (), Throws::Discard), // sub_141259db0
    Replacement::draft_throws(0x14125a040, "unknown_14125a040", tpl_22976::<0x143881f98, 0x143881f90, 0x1401e9fa0, 0x328, 0x1433e49e8, 0x142418680, 0x1401e9e70> as *const (), Throws::Discard), // sub_14125a040
    Replacement::draft_throws(0x14125a0b0, "unknown_14125a0b0", fn_14125a0b0 as *const (), Throws::Discard), // sub_14125a0b0
    Replacement::draft(0x14125a320, "unknown_14125a320", fn_14125a320 as *const ()), // sub_14125a320
    Replacement::draft_throws(0x14125a460, "unknown_14125a460", fn_14125a460 as *const (), Throws::Discard), // sub_14125a460
    Replacement::draft(0x14125a4d0, "unknown_14125a4d0", fn_14125a4d0 as *const ()), // sub_14125a4d0
    Replacement::draft(0x14125a690, "unknown_14125a690", fn_14125a690 as *const ()), // sub_14125a690
    Replacement::draft_throws(0x14125a790, "unknown_14125a790", fn_14125a790 as *const (), Throws::Caught), // sub_14125a790
    Replacement::draft_throws(0x14125a8d0, "unknown_14125a8d0", fn_14125a8d0 as *const (), Throws::Caught), // sub_14125a8d0
    Replacement::draft_throws(0x14125aa10, "unknown_14125aa10", fn_14125aa10 as *const (), Throws::Caught), // sub_14125aa10
    Replacement::draft_throws(0x14125ab50, "unknown_14125ab50", fn_14125ab50 as *const (), Throws::Caught), // sub_14125ab50
    Replacement::draft_throws(0x14125adb0, "unknown_14125adb0", fn_14125adb0 as *const (), Throws::Caught), // sub_14125adb0
    Replacement::draft_throws(0x14125aef0, "unknown_14125aef0", fn_14125aef0 as *const (), Throws::Caught), // sub_14125aef0
    Replacement::draft_throws(0x14125b030, "unknown_14125b030", fn_14125b030 as *const (), Throws::Caught), // sub_14125b030
    Replacement::draft_throws(0x14125b170, "unknown_14125b170", fn_14125b170 as *const (), Throws::Caught), // sub_14125b170
    Replacement::draft_throws(0x14125b3d0, "unknown_14125b3d0", fn_14125b3d0 as *const (), Throws::Caught), // sub_14125b3d0
    Replacement::draft_throws(0x14125b510, "unknown_14125b510", fn_14125b510 as *const (), Throws::Caught), // sub_14125b510
    Replacement::draft_throws(0x14125b650, "unknown_14125b650", fn_14125b650 as *const (), Throws::Caught), // sub_14125b650
    Replacement::draft_throws(0x14125b790, "unknown_14125b790", fn_14125b790 as *const (), Throws::Caught), // sub_14125b790
    Replacement::draft(0x14125b9f0, "unknown_14125b9f0", fn_14125b9f0 as *const ()), // sub_14125b9f0
    Replacement::draft(0x14125bcb0, "unknown_14125bcb0", fn_14125bcb0 as *const ()), // sub_14125bcb0
    Replacement::draft(0x14125be00, "unknown_14125be00", fn_14125be00 as *const ()), // sub_14125be00
    Replacement::draft(0x14125be90, "unknown_14125be90", fn_14125be90 as *const ()), // sub_14125be90
    Replacement::draft(0x14125c160, "unknown_14125c160", fn_14125c160 as *const ()), // sub_14125c160
    Replacement::draft(0x14125c2c0, "unknown_14125c2c0", fn_14125c2c0 as *const ()), // sub_14125c2c0
    Replacement::draft_throws(0x14125c360, "unknown_14125c360", fn_14125c360 as *const (), Throws::Discard), // sub_14125c360
    Replacement::draft_throws(0x14125c650, "unknown_14125c650", fn_14125c650 as *const (), Throws::Discard), // sub_14125c650
    Replacement::draft_throws(0x14125c7f0, "unknown_14125c7f0", fn_14125c7f0 as *const (), Throws::Discard), // sub_14125c7f0
    Replacement::draft_throws(0x14125c880, "unknown_14125c880", fn_14125c880 as *const (), Throws::Caught), // sub_14125c880
    Replacement::draft_throws(0x14125ca70, "unknown_14125ca70", fn_14125ca70 as *const (), Throws::Caught), // sub_14125ca70
    Replacement::draft_throws(0x14125cc60, "unknown_14125cc60", fn_14125cc60 as *const (), Throws::Caught), // sub_14125cc60
    Replacement::draft_throws(0x14125ce50, "unknown_14125ce50", fn_14125ce50 as *const (), Throws::Caught), // sub_14125ce50
    Replacement::draft_throws(0x14125d090, "unknown_14125d090", fn_14125d090 as *const (), Throws::Caught), // sub_14125d090
    Replacement::draft_throws(0x14125d3a0, "unknown_14125d3a0", fn_14125d3a0 as *const (), Throws::Caught), // sub_14125d3a0
    Replacement::draft_throws(0x14125d810, "unknown_14125d810", fn_14125d810 as *const (), Throws::Caught), // sub_14125d810
    Replacement::draft_throws(0x14125dc80, "unknown_14125dc80", fn_14125dc80 as *const (), Throws::Caught), // sub_14125dc80
    Replacement::draft_throws(0x14125e0f0, "unknown_14125e0f0", fn_14125e0f0 as *const (), Throws::Caught), // sub_14125e0f0
    Replacement::draft_throws(0x14125e7a0, "unknown_14125e7a0", fn_14125e7a0 as *const (), Throws::Caught), // sub_14125e7a0
    Replacement::draft_throws(0x14125ebd0, "unknown_14125ebd0", fn_14125ebd0 as *const (), Throws::Caught), // sub_14125ebd0
    Replacement::draft_throws(0x14125f000, "unknown_14125f000", fn_14125f000 as *const (), Throws::Caught), // sub_14125f000
    Replacement::draft_throws(0x14125f430, "unknown_14125f430", fn_14125f430 as *const (), Throws::Caught), // sub_14125f430
    Replacement::draft_throws(0x14125fab0, "unknown_14125fab0", fn_14125fab0 as *const (), Throws::Caught), // sub_14125fab0
    Replacement::draft_throws(0x14125fe00, "unknown_14125fe00", fn_14125fe00 as *const (), Throws::Caught), // sub_14125fe00
    Replacement::draft_throws(0x141260150, "unknown_141260150", fn_141260150 as *const (), Throws::Caught), // sub_141260150
    Replacement::draft_throws(0x1412604a0, "unknown_1412604a0", fn_1412604a0 as *const (), Throws::Caught), // sub_1412604a0
    Replacement::draft_throws(0x1412608c0, "unknown_1412608c0", fn_1412608c0 as *const (), Throws::Caught), // sub_1412608c0
    Replacement::draft_throws(0x141260bd0, "unknown_141260bd0", fn_141260bd0 as *const (), Throws::Discard), // sub_141260bd0
    Replacement::draft_throws(0x141260e20, "unknown_141260e20", fn_141260e20 as *const (), Throws::Discard), // sub_141260e20
    Replacement::draft_throws(0x141261130, "unknown_141261130", tpl_2091::<0x143881f90, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1433e49e8, 0x1402c4bf0> as *const (), Throws::Discard), // sub_141261130
    Replacement::draft(0x1412611d0, "unknown_1412611d0", fn_1412611d0 as *const ()), // sub_1412611d0
    Replacement::draft(0x141261270, "unknown_141261270", fn_141261270 as *const ()), // sub_141261270
    Replacement::draft_throws(0x141261390, "unknown_141261390", fn_141261390 as *const (), Throws::Caught), // sub_141261390
    Replacement::draft_throws(0x141261790, "unknown_141261790", fn_141261790 as *const (), Throws::Caught), // sub_141261790
    Replacement::draft_throws(0x141261f00, "unknown_141261f00", fn_141261f00 as *const (), Throws::Caught), // sub_141261f00
    Replacement::draft_throws(0x141261fb0, "unknown_141261fb0", fn_141261fb0 as *const (), Throws::Caught), // sub_141261fb0
    Replacement::draft_throws(0x141262360, "unknown_141262360", fn_141262360 as *const (), Throws::Caught), // sub_141262360
    Replacement::draft_throws(0x141262a10, "unknown_141262a10", fn_141262a10 as *const (), Throws::Caught), // sub_141262a10
    Replacement::draft_throws(0x141262ac0, "unknown_141262ac0", fn_141262ac0 as *const (), Throws::Caught), // sub_141262ac0
    Replacement::draft_throws(0x141262e60, "unknown_141262e60", fn_141262e60 as *const (), Throws::Caught), // sub_141262e60
    Replacement::draft_throws(0x1412634e0, "unknown_1412634e0", fn_1412634e0 as *const (), Throws::Caught), // sub_1412634e0
    Replacement::draft_throws(0x1412635e0, "unknown_1412635e0", fn_1412635e0 as *const (), Throws::Discard), // sub_1412635e0
    Replacement::draft(0x1412636f0, "unknown_1412636f0", fn_1412636f0 as *const ()), // sub_1412636f0
    Replacement::draft_throws(0x1412637c0, "unknown_1412637c0", tpl_838::<0x141263a80> as *const (), Throws::Discard), // sub_1412637c0
    Replacement::draft_throws(0x1412637d0, "unknown_1412637d0", tpl_838::<0x141263f10> as *const (), Throws::Discard), // sub_1412637d0
    Replacement::draft_throws(0x1412637e0, "unknown_1412637e0", tpl_838::<0x141264370> as *const (), Throws::Discard), // sub_1412637e0
    Replacement::draft_throws(0x1412637f0, "unknown_1412637f0", fn_1412637f0 as *const (), Throws::Caught), // sub_1412637f0
    Replacement::draft_throws(0x1412638c0, "unknown_1412638c0", fn_1412638c0 as *const (), Throws::Discard), // sub_1412638c0
    Replacement::draft(0x1412639b0, "unknown_1412639b0", tpl_23016::<0x1438816a8, 0x14243b0b0, 0x318, 0x328, 0x320, 0x8, 0x28, 0x338, 0x40, 0x4, 0x38, 0xfffffffffffffff8> as *const ()), // sub_1412639b0
    Replacement::draft_throws(0x141263a80, "unknown_141263a80", fn_141263a80 as *const (), Throws::Caught), // sub_141263a80
    Replacement::draft_throws(0x141263f10, "unknown_141263f10", fn_141263f10 as *const (), Throws::Caught), // sub_141263f10
    Replacement::draft_throws(0x141264370, "unknown_141264370", fn_141264370 as *const (), Throws::Caught), // sub_141264370
    Replacement::draft(0x1412647c0, "unknown_1412647c0", fn_1412647c0 as *const ()), // sub_1412647c0
    Replacement::draft(0x141264880, "unknown_141264880", tpl_23016::<0x1438816a8, 0x14243b0b0, 0x318, 0x328, 0x320, 0x8, 0x28, 0x338, 0xe0, 0x4, 0xd8, 0xfffffffffffffff8> as *const ()), // sub_141264880
    Replacement::draft_throws(0x141264950, "unknown_141264950", fn_141264950 as *const (), Throws::Discard), // sub_141264950
    Replacement::draft_throws(0x141264a20, "unknown_141264a20", fn_141264a20 as *const (), Throws::Caught), // sub_141264a20
    Replacement::draft_throws(0x141264ef0, "unknown_141264ef0", fn_141264ef0 as *const (), Throws::Caught), // sub_141264ef0
    Replacement::draft_throws(0x141265370, "unknown_141265370", fn_141265370 as *const (), Throws::Caught), // sub_141265370
    Replacement::draft_throws(0x1412657f0, "unknown_1412657f0", fn_1412657f0 as *const (), Throws::Caught), // sub_1412657f0
    Replacement::draft_throws(0x141265a90, "unknown_141265a90", fn_141265a90 as *const (), Throws::Caught), // sub_141265a90
    Replacement::draft_throws(0x141265d40, "unknown_141265d40", fn_141265d40 as *const (), Throws::Caught), // sub_141265d40
    Replacement::draft_throws(0x141265fe0, "unknown_141265fe0", fn_141265fe0 as *const (), Throws::Caught), // sub_141265fe0
    Replacement::draft_throws(0x141266280, "unknown_141266280", fn_141266280 as *const (), Throws::Caught), // sub_141266280
    Replacement::draft_throws(0x141266850, "unknown_141266850", fn_141266850 as *const (), Throws::Caught), // sub_141266850
    Replacement::draft_throws(0x141266dd0, "unknown_141266dd0", fn_141266dd0 as *const (), Throws::Caught), // sub_141266dd0
    Replacement::draft_throws(0x141267120, "unknown_141267120", fn_141267120 as *const (), Throws::Discard), // sub_141267120
    Replacement::draft_throws(0x141267210, "unknown_141267210", fn_141267210 as *const (), Throws::Caught), // sub_141267210
    Replacement::draft_throws(0x141267320, "unknown_141267320", fn_141267320 as *const (), Throws::Caught), // sub_141267320
    Replacement::draft_throws(0x141267430, "unknown_141267430", fn_141267430 as *const (), Throws::Caught), // sub_141267430
    Replacement::draft(0x141267540, "unknown_141267540", tpl_23029::<0x100, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1421571a0> as *const ()), // sub_141267540
    Replacement::draft(0x1412675c0, "unknown_1412675c0", fn_1412675c0 as *const ()), // sub_1412675c0
    Replacement::draft(0x141267640, "unknown_141267640", fn_141267640 as *const ()), // sub_141267640
    Replacement::draft(0x141267720, "unknown_141267720", fn_141267720 as *const ()), // sub_141267720
    Replacement::draft(0x141267790, "unknown_141267790", tpl_16694::<0x1433e4940, 0x44, 0x1, 0x14020f270, 0xa0, 0x1432e78a0, 0xa> as *const ()), // sub_141267790
    Replacement::draft(0x141267800, "unknown_141267800", fn_141267800 as *const ()), // sub_141267800
    Replacement::draft(0x1412678e0, "unknown_1412678e0", fn_1412678e0 as *const ()), // sub_1412678e0
    Replacement::draft(0x141267960, "unknown_141267960", fn_141267960 as *const ()), // sub_141267960
    Replacement::draft(0x141267aa0, "physics_141267aa0", fn_141267aa0 as *const ()), // physics::CollisionPackageResource::physics::CollisionPackageResource::vfunc_0
    Replacement::draft(0x141267b00, "physics_141267b00", fn_141267b00 as *const ()), // sub_141267b00
    Replacement::draft(0x141267bc0, "physics_141267bc0", fn_141267bc0 as *const ()), // sub_141267bc0
    Replacement::draft(0x141267cb0, "physics_141267cb0", tpl_23039::<0x1438816e0, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xc0, 0x1412498f0> as *const ()), // sub_141267cb0
    Replacement::draft_throws(0x141267d60, "physics_141267d60", tpl_23040::<0x1438816e0, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xc0, 0x141249960> as *const (), Throws::Discard), // sub_141267d60
    Replacement::draft_throws(0x141267e10, "physics_141267e10", tpl_81395::<0x1438816e8, 0x1438816e0, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70, 0xc0, 0x141268650> as *const (), Throws::Discard), // sub_141267e10
    Replacement::draft_throws(0x141267ea0, "physics_141267ea0", tpl_23040::<0x1438816e0, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xc0, 0x141249cd0> as *const (), Throws::Discard), // sub_141267ea0
    Replacement::draft_throws(0x141267f50, "physics_141267f50", tpl_81395::<0x1438816e8, 0x1438816e0, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20, 0xc0, 0x1412688d0> as *const (), Throws::Discard), // sub_141267f50
    Replacement::draft(0x141267fe0, "physics_141267fe0", fn_141267fe0 as *const ()), // sub_141267fe0
    Replacement::draft_throws(0x141268060, "physics_141268060", fn_141268060 as *const (), Throws::Caught), // sub_141268060
    Replacement::draft_throws(0x141268120, "physics_141268120", fn_141268120 as *const (), Throws::Caught), // sub_141268120
    Replacement::draft_throws(0x1412681e0, "physics_1412681e0", fn_1412681e0 as *const (), Throws::Caught), // sub_1412681e0
    Replacement::draft(0x1412682a0, "physics_1412682a0", fn_1412682a0 as *const ()), // sub_1412682a0
    Replacement::draft(0x141268360, "physics_141268360", fn_141268360 as *const ()), // sub_141268360
    Replacement::draft(0x1412683f0, "physics_1412683f0", tpl_23039::<0x1438816e0, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xc0, 0x14124a680> as *const ()), // sub_1412683f0
    Replacement::draft_throws(0x1412684a0, "physics_1412684a0", fn_1412684a0 as *const (), Throws::Discard), // sub_1412684a0
    Replacement::draft_throws(0x141268530, "physics_141268530", fn_141268530 as *const (), Throws::Caught), // sub_141268530
    Replacement::draft_throws(0x141268650, "physics_141268650", fn_141268650 as *const (), Throws::Caught), // sub_141268650
    Replacement::draft_throws(0x1412688d0, "physics_1412688d0", fn_1412688d0 as *const (), Throws::Caught), // sub_1412688d0
    Replacement::draft_throws(0x141268b50, "physics_141268b50", fn_141268b50 as *const (), Throws::Caught), // sub_141268b50
    Replacement::draft_throws(0x141268de0, "physics_141268de0", fn_141268de0 as *const (), Throws::Caught), // sub_141268de0
    Replacement::draft_throws(0x141269070, "physics_141269070", fn_141269070 as *const (), Throws::Caught), // sub_141269070
    Replacement::draft_throws(0x141269300, "physics_141269300", fn_141269300 as *const (), Throws::Caught), // sub_141269300
    Replacement::draft_throws(0x1412699c0, "physics_1412699c0", fn_1412699c0 as *const (), Throws::Caught), // sub_1412699c0
    Replacement::draft_throws(0x14126a050, "physics_14126a050", fn_14126a050 as *const (), Throws::Caught), // sub_14126a050
    Replacement::draft(0x14126a490, "physics_14126a490", fn_14126a490 as *const ()), // sub_14126a490
    Replacement::draft(0x14126a510, "physics_14126a510", fn_14126a510 as *const ()), // sub_14126a510
    Replacement::draft(0x14126a5a0, "physics_14126a5a0", fn_14126a5a0 as *const ()), // sub_14126a5a0
    Replacement::draft(0x14126a650, "physics_14126a650", fn_14126a650 as *const ()), // sub_14126a650
    Replacement::draft(0x14126a740, "physics_14126a740", fn_14126a740 as *const ()), // sub_14126a740
    Replacement::draft_throws(0x14126a8d0, "physics_14126a8d0", fn_14126a8d0 as *const (), Throws::Caught), // sub_14126a8d0
    Replacement::draft_throws(0x14126aa80, "physics_14126aa80", fn_14126aa80 as *const (), Throws::Caught), // sub_14126aa80
    Replacement::draft_throws(0x14126abe0, "physics_14126abe0", fn_14126abe0 as *const (), Throws::Caught), // sub_14126abe0
    Replacement::draft(0x14126adb0, "physics_14126adb0", fn_14126adb0 as *const ()), // sub_14126adb0
    Replacement::draft_throws(0x14126ae80, "physics_14126ae80", fn_14126ae80 as *const (), Throws::Caught), // sub_14126ae80
    Replacement::draft_throws(0x14126aff0, "physics_14126aff0", fn_14126aff0 as *const (), Throws::Caught), // sub_14126aff0
    Replacement::draft_throws(0x14126b160, "physics_14126b160", fn_14126b160 as *const (), Throws::Caught), // sub_14126b160
    Replacement::draft(0x14126b2d0, "physics_14126b2d0", fn_14126b2d0 as *const ()), // sub_14126b2d0
    Replacement::draft(0x14126b3c0, "physics_14126b3c0", fn_14126b3c0 as *const ()), // sub_14126b3c0
    Replacement::draft(0x14126b4d0, "physics_14126b4d0", fn_14126b4d0 as *const ()), // sub_14126b4d0
    Replacement::draft_throws(0x14126b5e0, "physics_14126b5e0", fn_14126b5e0 as *const (), Throws::Discard), // sub_14126b5e0
    Replacement::draft_throws(0x14126b700, "physics_14126b700", fn_14126b700 as *const (), Throws::Discard), // sub_14126b700
    Replacement::draft(0x14126b860, "physics_14126b860", tpl_22981::<0x1438816f8, 0x14243b0b0, 0x141250300, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14126b860
    Replacement::draft_throws(0x14126b8d0, "physics_14126b8d0", fn_14126b8d0 as *const (), Throws::Discard), // sub_14126b8d0
    Replacement::draft(0x14126b980, "physics_14126b980", fn_14126b980 as *const ()), // sub_14126b980
    Replacement::draft_throws(0x14126ba30, "physics_14126ba30", tpl_17230::<0x1438816ec, 0x1438816f8, 0x1401e9b60, 0x14124d0c0, 0x1401e9a20> as *const (), Throws::Discard), // sub_14126ba30
    Replacement::draft_throws(0x14126ba90, "physics_14126ba90", fn_14126ba90 as *const (), Throws::Caught), // sub_14126ba90
    Replacement::draft_throws(0x14126bb40, "physics_14126bb40", fn_14126bb40 as *const (), Throws::Caught), // sub_14126bb40
    Replacement::draft_throws(0x14126bbf0, "physics_14126bbf0", fn_14126bbf0 as *const (), Throws::Caught), // sub_14126bbf0
    Replacement::draft(0x14126bca0, "physics_14126bca0", fn_14126bca0 as *const ()), // sub_14126bca0
    Replacement::draft(0x14126bd50, "physics_14126bd50", fn_14126bd50 as *const ()), // sub_14126bd50
    Replacement::draft(0x14126bde0, "physics_14126bde0", fn_14126bde0 as *const ()), // sub_14126bde0
    Replacement::draft_throws(0x14126be80, "physics_14126be80", fn_14126be80 as *const (), Throws::Discard), // sub_14126be80
    Replacement::draft_throws(0x14126bf30, "physics_14126bf30", tpl_2091::<0x1438816f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1433e56b0, 0x141251cc0> as *const (), Throws::Discard), // sub_14126bf30
    Replacement::draft_throws(0x14126bff0, "physics_14126bff0", fn_14126bff0 as *const (), Throws::Caught), // sub_14126bff0
    Replacement::draft_throws(0x14126c2f0, "physics_14126c2f0", fn_14126c2f0 as *const (), Throws::Caught), // sub_14126c2f0
    Replacement::draft_throws(0x14126c630, "physics_14126c630", fn_14126c630 as *const (), Throws::Caught), // sub_14126c630
    Replacement::draft_throws(0x14126cad0, "physics_14126cad0", fn_14126cad0 as *const (), Throws::Caught), // sub_14126cad0
    Replacement::draft_throws(0x14126cf40, "physics_14126cf40", fn_14126cf40 as *const (), Throws::Caught), // sub_14126cf40
    Replacement::draft_throws(0x14126d3b0, "physics_14126d3b0", fn_14126d3b0 as *const (), Throws::Caught), // sub_14126d3b0
    Replacement::draft_throws(0x14126d480, "physics_14126d480", fn_14126d480 as *const (), Throws::Caught), // sub_14126d480
    Replacement::draft_throws(0x14126d6f0, "physics_14126d6f0", fn_14126d6f0 as *const (), Throws::Caught), // sub_14126d6f0
    Replacement::draft_throws(0x14126d950, "physics_14126d950", fn_14126d950 as *const (), Throws::Caught), // sub_14126d950
    Replacement::draft_throws(0x14126dbb0, "physics_14126dbb0", fn_14126dbb0 as *const (), Throws::Caught), // sub_14126dbb0
    Replacement::draft_throws(0x14126e210, "physics_14126e210", fn_14126e210 as *const (), Throws::Caught), // sub_14126e210
    Replacement::draft_throws(0x14126e830, "physics_14126e830", fn_14126e830 as *const (), Throws::Caught), // sub_14126e830
    Replacement::draft_throws(0x14126ec50, "physics_14126ec50", fn_14126ec50 as *const (), Throws::Caught), // sub_14126ec50
    Replacement::draft(0x14126eeb0, "physics_14126eeb0", fn_14126eeb0 as *const ()), // sub_14126eeb0
    Replacement::draft_throws(0x14126efa0, "physics_14126efa0", fn_14126efa0 as *const (), Throws::Discard), // sub_14126efa0
    Replacement::draft(0x14126f070, "physics_14126f070", fn_14126f070 as *const ()), // sub_14126f070
    Replacement::draft_throws(0x14126f080, "physics_14126f080", fn_14126f080 as *const (), Throws::Discard), // sub_14126f080
    Replacement::draft_throws(0x14126f140, "physics_14126f140", fn_14126f140 as *const (), Throws::Discard), // sub_14126f140
    Replacement::draft_throws(0x14126f1b0, "physics_14126f1b0", fn_14126f1b0 as *const (), Throws::Discard), // sub_14126f1b0
    Replacement::draft_throws(0x14126f210, "physics_14126f210", fn_14126f210 as *const (), Throws::Discard), // sub_14126f210
    Replacement::draft_throws(0x14126f2d0, "physics_14126f2d0", fn_14126f2d0 as *const (), Throws::Discard), // sub_14126f2d0
    Replacement::draft(0x14126f4e0, "physics_14126f4e0", fn_14126f4e0 as *const ()), // sub_14126f4e0
    Replacement::draft_throws(0x14126f4f0, "physics_14126f4f0", fn_14126f4f0 as *const (), Throws::Discard), // sub_14126f4f0
    Replacement::draft_throws(0x14126f560, "physics_14126f560", fn_14126f560 as *const (), Throws::Discard), // sub_14126f560
    Replacement::draft(0x14126f5e0, "physics_14126f5e0", fn_14126f5e0 as *const ()), // sub_14126f5e0
    Replacement::draft(0x14126faa0, "physics_14126faa0", fn_14126faa0 as *const ()), // sub_14126faa0
    Replacement::draft(0x14126fe60, "physics_14126fe60", fn_14126fe60 as *const ()), // sub_14126fe60
    Replacement::draft(0x14126ff70, "physics_14126ff70", fn_14126ff70 as *const ()), // sub_14126ff70
    Replacement::draft(0x141270050, "physics_141270050", fn_141270050 as *const ()), // sub_141270050
    Replacement::draft(0x141270090, "physics_141270090", fn_141270090 as *const ()), // sub_141270090
    Replacement::draft_throws(0x141270110, "physics_141270110", fn_141270110 as *const (), Throws::Discard), // sub_141270110
    Replacement::draft(0x141270150, "physics_141270150", fn_141270150 as *const ()), // sub_141270150
    Replacement::draft(0x1412701d0, "physics_1412701d0", fn_1412701d0 as *const ()), // sub_1412701d0
    Replacement::draft(0x141270290, "physics_141270290", fn_141270290 as *const ()), // sub_141270290
    Replacement::draft_throws(0x1412703d0, "physics_1412703d0", fn_1412703d0 as *const (), Throws::Discard), // sub_1412703d0
    Replacement::draft(0x141270490, "physics_141270490", fn_141270490 as *const ()), // sub_141270490
    Replacement::draft(0x141270780, "physics_141270780", fn_141270780 as *const ()), // sub_141270780
    Replacement::draft(0x141270ab0, "physics_141270ab0", fn_141270ab0 as *const ()), // sub_141270ab0
    Replacement::draft(0x141270ce0, "physics_141270ce0", fn_141270ce0 as *const ()), // sub_141270ce0
    Replacement::draft_throws(0x141270e40, "physics_141270e40", fn_141270e40 as *const (), Throws::Discard), // sub_141270e40
    Replacement::draft(0x141270ee0, "physics_141270ee0", fn_141270ee0 as *const ()), // sub_141270ee0
    Replacement::draft(0x141270f60, "physics_141270f60", fn_141270f60 as *const ()), // sub_141270f60
    Replacement::draft(0x141270ff0, "physics_141270ff0", fn_141270ff0 as *const ()), // sub_141270ff0
    Replacement::draft(0x141271400, "physics_141271400", fn_141271400 as *const ()), // sub_141271400
    Replacement::draft(0x141271e10, "physics_141271e10", fn_141271e10 as *const ()), // sub_141271e10
    Replacement::draft(0x141272140, "physics_141272140", fn_141272140 as *const ()), // sub_141272140
    Replacement::draft(0x141272320, "physics_141272320", fn_141272320 as *const ()), // sub_141272320
    Replacement::draft(0x141272d50, "physics_141272d50", fn_141272d50 as *const ()), // sub_141272d50
    Replacement::draft(0x141272e00, "physics_141272e00", fn_141272e00 as *const ()), // sub_141272e00
    Replacement::draft(0x141272ee0, "physics_141272ee0", fn_141272ee0 as *const ()), // sub_141272ee0
    Replacement::draft(0x1412731c0, "physics_1412731c0", fn_1412731c0 as *const ()), // sub_1412731c0
    Replacement::draft(0x141273340, "physics_141273340", fn_141273340 as *const ()), // sub_141273340
    Replacement::draft(0x141273420, "physics_141273420", fn_141273420 as *const ()), // sub_141273420
    Replacement::draft(0x141273640, "physics_141273640", fn_141273640 as *const ()), // sub_141273640
    Replacement::draft(0x141273750, "physics_141273750", tpl_23111::<0x8, 0x1412731c0> as *const ()), // sub_141273750
    Replacement::draft(0x1412737c0, "physics_1412737c0", fn_1412737c0 as *const ()), // sub_1412737c0
    Replacement::draft(0x141273960, "physics_141273960", tpl_23111::<0x8, 0x141273640> as *const ()), // sub_141273960
    Replacement::draft(0x1412739d0, "physics_1412739d0", fn_1412739d0 as *const ()), // sub_1412739d0
    Replacement::draft_throws(0x141273af0, "physics_141273af0", fn_141273af0 as *const (), Throws::Discard), // sub_141273af0
    Replacement::draft_throws(0x141273ba0, "physics_141273ba0", fn_141273ba0 as *const (), Throws::Discard), // sub_141273ba0
    Replacement::draft_throws(0x141273cb0, "physics_141273cb0", fn_141273cb0 as *const (), Throws::Caught), // sub_141273cb0
    Replacement::draft_throws(0x141273e20, "physics_141273e20", fn_141273e20 as *const (), Throws::Caught), // sub_141273e20
    Replacement::draft_throws(0x141273f90, "physics_141273f90", fn_141273f90 as *const (), Throws::Caught), // sub_141273f90
    Replacement::draft(0x141274100, "physics_141274100", fn_141274100 as *const ()), // sub_141274100
    Replacement::draft(0x1412741d0, "physics_1412741d0", fn_1412741d0 as *const ()), // sub_1412741d0
    Replacement::draft_throws(0x1412742a0, "physics_1412742a0", fn_1412742a0 as *const (), Throws::Discard), // sub_1412742a0
    Replacement::draft_throws(0x141274390, "physics_141274390", fn_141274390 as *const (), Throws::Discard), // sub_141274390
    Replacement::draft(0x1412744d0, "physics_1412744d0", tpl_7173::<0x1433e3968, 0x4, 0x1, 0x14020f270, 0x8, 0x1433e3950> as *const ()), // sub_1412744d0
    Replacement::draft(0x141274540, "physics_141274540", fn_141274540 as *const ()), // sub_141274540
    Replacement::draft(0x141274640, "physics_141274640", tpl_22011::<0x143881710, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141274640
    Replacement::draft_throws(0x1412746d0, "physics_1412746d0", fn_1412746d0 as *const (), Throws::Discard), // sub_1412746d0
    Replacement::draft_throws(0x1412747c0, "physics_1412747c0", fn_1412747c0 as *const (), Throws::Caught), // sub_1412747c0
    Replacement::draft_throws(0x141274910, "physics_141274910", fn_141274910 as *const (), Throws::Caught), // sub_141274910
    Replacement::draft_throws(0x141274a60, "physics_141274a60", fn_141274a60 as *const (), Throws::Caught), // sub_141274a60
    Replacement::draft(0x141274bb0, "physics_141274bb0", tpl_71920::<0x143881718, 0x143881710, 0x1401e9970, 0x338, 0xd8, 0x4, 0x8, 0x1401e9860> as *const ()), // sub_141274bb0
    Replacement::draft(0x141274c40, "physics_141274c40", tpl_22011::<0x143881710, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141274c40
    Replacement::draft_throws(0x141274ce0, "physics_141274ce0", fn_141274ce0 as *const (), Throws::Discard), // sub_141274ce0
    Replacement::draft_throws(0x141274d90, "physics_141274d90", fn_141274d90 as *const (), Throws::Discard), // sub_141274d90
    Replacement::draft(0x141274ec0, "physics_141274ec0", fn_141274ec0 as *const ()), // sub_141274ec0
    Replacement::draft(0x141274f50, "physics_141274f50", fn_141274f50 as *const ()), // sub_141274f50
    Replacement::draft(0x141274fd0, "physics_141274fd0", fn_141274fd0 as *const ()), // sub_141274fd0
    Replacement::draft(0x141275100, "physics_141275100", fn_141275100 as *const ()), // sub_141275100
    Replacement::draft(0x141275230, "physics_141275230", tpl_23130::<0x143881720, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141275230
    Replacement::draft_throws(0x1412752d0, "physics_1412752d0", fn_1412752d0 as *const (), Throws::Discard), // sub_1412752d0
    Replacement::draft_throws(0x1412753f0, "physics_1412753f0", fn_1412753f0 as *const (), Throws::Caught), // sub_1412753f0
    Replacement::draft_throws(0x141275580, "physics_141275580", fn_141275580 as *const (), Throws::Caught), // sub_141275580
    Replacement::draft_throws(0x141275710, "physics_141275710", fn_141275710 as *const (), Throws::Caught), // sub_141275710
    Replacement::draft(0x1412758a0, "physics_1412758a0", fn_1412758a0 as *const ()), // sub_1412758a0
    Replacement::draft(0x141275940, "physics_141275940", tpl_23130::<0x143881720, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141275940
    Replacement::draft_throws(0x1412759f0, "physics_1412759f0", fn_1412759f0 as *const (), Throws::Discard), // sub_1412759f0
    Replacement::draft_throws(0x141275ac0, "physics_141275ac0", fn_141275ac0 as *const (), Throws::Discard), // sub_141275ac0
    Replacement::draft(0x141275c20, "physics_141275c20", tpl_13574::<0x1433e3a30, 0x1, 0x14020f270, 0x4, 0x1433e3a18, 0x8, 0x1433e3a00, 0xc, 0x1433e39f0> as *const ()), // sub_141275c20
    Replacement::draft(0x141275cd0, "physics_141275cd0", tpl_18758::<0x168, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x141244090> as *const ()), // sub_141275cd0
    Replacement::draft(0x141275d60, "physics_141275d60", fn_141275d60 as *const ()), // sub_141275d60
    Replacement::draft(0x141275e10, "physics_141275e10", fn_141275e10 as *const ()), // sub_141275e10
    Replacement::draft(0x141275fd0, "physics_141275fd0", fn_141275fd0 as *const ()), // sub_141275fd0
    Replacement::draft_throws(0x141276540, "physics_141276540", fn_141276540 as *const (), Throws::Caught), // sub_141276540
    Replacement::draft_throws(0x141276c10, "physics_141276c10", fn_141276c10 as *const (), Throws::Caught), // sub_141276c10
    Replacement::draft(0x141277250, "physics_141277250", fn_141277250 as *const ()), // sub_141277250
    Replacement::draft_throws(0x141277870, "physics_141277870", fn_141277870 as *const (), Throws::Caught), // sub_141277870
    Replacement::draft_throws(0x141277d80, "physics_141277d80", fn_141277d80 as *const (), Throws::Caught), // sub_141277d80
    Replacement::draft_throws(0x141278290, "physics_141278290", fn_141278290 as *const (), Throws::Caught), // sub_141278290
    Replacement::draft(0x1412787a0, "physics_1412787a0", fn_1412787a0 as *const ()), // sub_1412787a0
    Replacement::draft(0x141278eb0, "physics_141278eb0", fn_141278eb0 as *const ()), // sub_141278eb0
    Replacement::draft(0x141279520, "physics_141279520", fn_141279520 as *const ()), // sub_141279520
    Replacement::draft_throws(0x141279b90, "physics_141279b90", fn_141279b90 as *const (), Throws::Discard), // sub_141279b90
    Replacement::draft_throws(0x14127a340, "physics_14127a340", fn_14127a340 as *const (), Throws::Discard), // sub_14127a340
    Replacement::draft(0x14127a820, "physics_14127a820", fn_14127a820 as *const ()), // sub_14127a820
    Replacement::draft(0x14127a9f0, "physics_14127a9f0", fn_14127a9f0 as *const ()), // sub_14127a9f0
    Replacement::draft_throws(0x14127acc0, "physics_14127acc0", tpl_9694::<0x20, 0x8, 0x14127ec40> as *const (), Throws::Discard), // sub_14127acc0
    Replacement::draft_throws(0x14127ad10, "physics_14127ad10", tpl_6961::<0x20, 0x8, 0x14127ed10> as *const (), Throws::Discard), // sub_14127ad10
    Replacement::draft_throws(0x14127ad60, "physics_14127ad60", tpl_6985::<0x20, 0x8, 0x14127edd0> as *const (), Throws::Discard), // sub_14127ad60
    Replacement::draft_throws(0x14127adb0, "physics_14127adb0", tpl_9695::<0x20, 0x8, 0x14127eea0> as *const (), Throws::Discard), // sub_14127adb0
    Replacement::draft_throws(0x14127ae00, "physics_14127ae00", fn_14127ae00 as *const (), Throws::Caught), // sub_14127ae00
    Replacement::draft_throws(0x14127b0d0, "physics_14127b0d0", fn_14127b0d0 as *const (), Throws::Caught), // sub_14127b0d0
    Replacement::draft_throws(0x14127b3c0, "physics_14127b3c0", fn_14127b3c0 as *const (), Throws::Caught), // sub_14127b3c0
    Replacement::draft(0x14127b690, "physics_14127b690", fn_14127b690 as *const ()), // sub_14127b690
    Replacement::draft(0x14127b960, "physics_14127b960", fn_14127b960 as *const ()), // sub_14127b960
    Replacement::draft_throws(0x14127bc10, "physics_14127bc10", fn_14127bc10 as *const (), Throws::Caught), // sub_14127bc10
    Replacement::draft_throws(0x14127c080, "physics_14127c080", fn_14127c080 as *const (), Throws::Caught), // sub_14127c080
    Replacement::draft_throws(0x14127c390, "physics_14127c390", fn_14127c390 as *const (), Throws::Caught), // sub_14127c390
    Replacement::draft_throws(0x14127c840, "physics_14127c840", fn_14127c840 as *const (), Throws::Caught), // sub_14127c840
    Replacement::draft_throws(0x14127ccc0, "physics_14127ccc0", fn_14127ccc0 as *const (), Throws::Caught), // sub_14127ccc0
    Replacement::draft_throws(0x14127d100, "physics_14127d100", fn_14127d100 as *const (), Throws::Caught), // sub_14127d100
    Replacement::draft_throws(0x14127d3d0, "physics_14127d3d0", fn_14127d3d0 as *const (), Throws::Caught), // sub_14127d3d0
    Replacement::draft_throws(0x14127d850, "physics_14127d850", fn_14127d850 as *const (), Throws::Caught), // sub_14127d850
    Replacement::draft_throws(0x14127dca0, "physics_14127dca0", fn_14127dca0 as *const (), Throws::Caught), // sub_14127dca0
    Replacement::draft_throws(0x14127e0d0, "physics_14127e0d0", fn_14127e0d0 as *const (), Throws::Caught), // sub_14127e0d0
    Replacement::draft_throws(0x14127e390, "physics_14127e390", fn_14127e390 as *const (), Throws::Caught), // sub_14127e390
    Replacement::draft_throws(0x14127e800, "physics_14127e800", fn_14127e800 as *const (), Throws::Caught), // sub_14127e800
    Replacement::draft_throws(0x14127ec40, "physics_14127ec40", fn_14127ec40 as *const (), Throws::Caught), // sub_14127ec40
    Replacement::draft_throws(0x14127ed10, "physics_14127ed10", fn_14127ed10 as *const (), Throws::Caught), // sub_14127ed10
    Replacement::draft_throws(0x14127edd0, "physics_14127edd0", fn_14127edd0 as *const (), Throws::Caught), // sub_14127edd0
    Replacement::draft_throws(0x14127eea0, "physics_14127eea0", fn_14127eea0 as *const (), Throws::Caught), // sub_14127eea0
    Replacement::draft_throws(0x14127ef70, "physics_14127ef70", fn_14127ef70 as *const (), Throws::Caught), // sub_14127ef70
    Replacement::draft_throws(0x14127f440, "physics_14127f440", fn_14127f440 as *const (), Throws::Caught), // sub_14127f440
    Replacement::draft_throws(0x14127fa00, "physics_14127fa00", fn_14127fa00 as *const (), Throws::Caught), // sub_14127fa00
    Replacement::draft_throws(0x14127fed0, "physics_14127fed0", fn_14127fed0 as *const (), Throws::Caught), // sub_14127fed0
    Replacement::draft_throws(0x1412803a0, "physics_1412803a0", fn_1412803a0 as *const (), Throws::Caught), // sub_1412803a0
    Replacement::draft_throws(0x141280830, "physics_141280830", fn_141280830 as *const (), Throws::Caught), // sub_141280830
    Replacement::draft_throws(0x141280d70, "physics_141280d70", fn_141280d70 as *const (), Throws::Caught), // sub_141280d70
    Replacement::draft_throws(0x141281200, "physics_141281200", fn_141281200 as *const (), Throws::Caught), // sub_141281200
    Replacement::draft_throws(0x141281690, "physics_141281690", fn_141281690 as *const (), Throws::Caught), // sub_141281690
    Replacement::draft_throws(0x141281b20, "physics_141281b20", fn_141281b20 as *const (), Throws::Caught), // sub_141281b20
    Replacement::draft_throws(0x141282020, "physics_141282020", fn_141282020 as *const (), Throws::Caught), // sub_141282020
    Replacement::draft_throws(0x1412824b0, "physics_1412824b0", fn_1412824b0 as *const (), Throws::Caught), // sub_1412824b0
    Replacement::draft_throws(0x141282940, "physics_141282940", fn_141282940 as *const (), Throws::Caught), // sub_141282940
    Replacement::draft_throws(0x141282ba0, "physics_141282ba0", fn_141282ba0 as *const (), Throws::Caught), // sub_141282ba0
    Replacement::draft_throws(0x141282e10, "physics_141282e10", fn_141282e10 as *const (), Throws::Caught), // sub_141282e10
    Replacement::draft_throws(0x141283070, "physics_141283070", fn_141283070 as *const (), Throws::Caught), // sub_141283070
    Replacement::draft_throws(0x1412832d0, "physics_1412832d0", fn_1412832d0 as *const (), Throws::Caught), // sub_1412832d0
    Replacement::draft_throws(0x141283540, "physics_141283540", fn_141283540 as *const (), Throws::Caught), // sub_141283540
    Replacement::draft_throws(0x1412837a0, "physics_1412837a0", fn_1412837a0 as *const (), Throws::Caught), // sub_1412837a0
    Replacement::draft_throws(0x141283a00, "physics_141283a00", fn_141283a00 as *const (), Throws::Caught), // sub_141283a00
    Replacement::draft_throws(0x141283c70, "physics_141283c70", fn_141283c70 as *const (), Throws::Caught), // sub_141283c70
    Replacement::draft_throws(0x141283ed0, "physics_141283ed0", fn_141283ed0 as *const (), Throws::Caught), // sub_141283ed0
    Replacement::draft_throws(0x1412840c0, "physics_1412840c0", fn_1412840c0 as *const (), Throws::Caught), // sub_1412840c0
    Replacement::draft_throws(0x1412842b0, "physics_1412842b0", fn_1412842b0 as *const (), Throws::Caught), // sub_1412842b0
    Replacement::draft_throws(0x1412844a0, "physics_1412844a0", fn_1412844a0 as *const (), Throws::Caught), // sub_1412844a0
    Replacement::draft_throws(0x141284a20, "physics_141284a20", fn_141284a20 as *const (), Throws::Caught), // sub_141284a20
    Replacement::draft_throws(0x141284fd0, "physics_141284fd0", fn_141284fd0 as *const (), Throws::Caught), // sub_141284fd0
    Replacement::draft_throws(0x141285550, "physics_141285550", fn_141285550 as *const (), Throws::Caught), // sub_141285550
    Replacement::draft_throws(0x141285a90, "physics_141285a90", fn_141285a90 as *const (), Throws::Caught), // sub_141285a90
    Replacement::draft_throws(0x141286020, "physics_141286020", fn_141286020 as *const (), Throws::Caught), // sub_141286020
    Replacement::draft_throws(0x141286560, "physics_141286560", fn_141286560 as *const (), Throws::Caught), // sub_141286560
    Replacement::draft_throws(0x1412868b0, "physics_1412868b0", fn_1412868b0 as *const (), Throws::Caught), // sub_1412868b0
    Replacement::draft_throws(0x141286c00, "physics_141286c00", fn_141286c00 as *const (), Throws::Caught), // sub_141286c00
    Replacement::draft(0x141286fb0, "physics_141286fb0", fn_141286fb0 as *const ()), // physics::ClothProfileResource::physics::ClothProfileResource::vfunc_0
    Replacement::draft(0x141286ff0, "unknown_141286ff0", fn_141286ff0 as *const ()), // sub_141286ff0
    Replacement::draft(0x141287080, "unknown_141287080", fn_141287080 as *const ()), // sub_141287080
];
