#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft_throws(0x141eec190, "unknown_141eec190", tpl_41541::<0x142119080> as *const (), Throws::Discard), // sub_141eec190
    Replacement::draft_throws(0x141eec1b0, "unknown_141eec1b0", tpl_41541::<0x142119080> as *const (), Throws::Discard), // sub_141eec1b0
    Replacement::draft(0x141eec1d0, "unknown_141eec1d0", tpl_34318 as *const ()), // sub_141eec1d0
    Replacement::draft(0x141eec1f0, "unknown_141eec1f0", tpl_34318 as *const ()), // sub_141eec1f0
    Replacement::draft(0x141eec210, "unknown_141eec210", tpl_34319::<0x143cd5d90, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141eec210
    Replacement::draft(0x141eec270, "unknown_141eec270", tpl_34319::<0x143cd6100, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141eec270
    Replacement::draft(0x141eec2d0, "unknown_141eec2d0", tpl_361 as *const ()), // sub_141eec2d0
    Replacement::draft_throws(0x141eec890, "unknown_141eec890", fn_141eec890 as *const (), Throws::Discard), // sub_141eec890
    Replacement::draft(0x141eec940, "unknown_141eec940", fn_141eec940 as *const ()), // sub_141eec940
    Replacement::draft_throws(0x141eeca80, "unknown_141eeca80", fn_141eeca80 as *const (), Throws::Discard), // sub_141eeca80
    Replacement::draft_throws(0x141eecc10, "unknown_141eecc10", fn_141eecc10 as *const (), Throws::Discard), // sub_141eecc10
    Replacement::draft(0x141eece70, "unknown_141eece70", tpl_361 as *const ()), // sub_141eece70
    Replacement::draft(0x141eece80, "unknown_141eece80", tpl_361 as *const ()), // sub_141eece80
    Replacement::draft(0x141eece90, "unknown_141eece90", tpl_361 as *const ()), // sub_141eece90
    Replacement::draft(0x141eecea0, "unknown_141eecea0", tpl_361 as *const ()), // sub_141eecea0
    Replacement::draft(0x141eeceb0, "unknown_141eeceb0", tpl_361 as *const ()), // sub_141eeceb0
    Replacement::draft(0x141eecec0, "unknown_141eecec0", tpl_361 as *const ()), // sub_141eecec0
    Replacement::draft(0x141eeced0, "unknown_141eeced0", tpl_361 as *const ()), // sub_141eeced0
    Replacement::draft_throws(0x141eecee0, "unknown_141eecee0", fn_141eecee0 as *const (), Throws::Caught), // sub_141eecee0
    Replacement::draft_throws(0x141eed220, "unknown_141eed220", fn_141eed220 as *const (), Throws::Caught), // sub_141eed220
    Replacement::draft_throws(0x141ef09c0, "unknown_141ef09c0", fn_141ef09c0 as *const (), Throws::Caught), // sub_141ef09c0
    Replacement::draft(0x141ef1310, "unknown_141ef1310", fn_141ef1310 as *const ()), // sub_141ef1310
    Replacement::draft_throws(0x141ef1340, "unknown_141ef1340", fn_141ef1340 as *const (), Throws::Caught), // sub_141ef1340
    Replacement::draft(0x141ef1640, "unknown_141ef1640", tpl_41549::<0x8> as *const ()), // sub_141ef1640
    Replacement::draft_throws(0x141ef1660, "unknown_141ef1660", fn_141ef1660 as *const (), Throws::Discard), // sub_141ef1660
    Replacement::draft_throws(0x141ef1c10, "unknown_141ef1c10", fn_141ef1c10 as *const (), Throws::Caught), // sub_141ef1c10
    Replacement::draft(0x141ef1d80, "unknown_141ef1d80", fn_141ef1d80 as *const ()), // sub_141ef1d80
    Replacement::draft_throws(0x141ef1e10, "unknown_141ef1e10", fn_141ef1e10 as *const (), Throws::Discard), // sub_141ef1e10
    Replacement::draft(0x141ef20e0, "unknown_141ef20e0", fn_141ef20e0 as *const ()), // sub_141ef20e0
    Replacement::draft(0x141ef2220, "unknown_141ef2220", fn_141ef2220 as *const ()), // sub_141ef2220
    Replacement::draft_throws(0x141ef2380, "unknown_141ef2380", fn_141ef2380 as *const (), Throws::Discard), // sub_141ef2380
    Replacement::draft(0x141ef2a20, "unknown_141ef2a20", fn_141ef2a20 as *const ()), // sub_141ef2a20
    Replacement::draft_throws(0x141ef2a50, "unknown_141ef2a50", fn_141ef2a50 as *const (), Throws::Caught), // sub_141ef2a50
    Replacement::draft(0x141ef2f80, "unknown_141ef2f80", fn_141ef2f80 as *const ()), // sub_141ef2f80
    Replacement::draft_throws(0x141ef2fe0, "unknown_141ef2fe0", fn_141ef2fe0 as *const (), Throws::Discard), // sub_141ef2fe0
    Replacement::draft_throws(0x141ef3390, "unknown_141ef3390", fn_141ef3390 as *const (), Throws::Discard), // sub_141ef3390
    Replacement::draft(0x141ef36a0, "unknown_141ef36a0", fn_141ef36a0 as *const ()), // sub_141ef36a0
    Replacement::draft_throws(0x141ef3810, "unknown_141ef3810", fn_141ef3810 as *const (), Throws::Discard), // sub_141ef3810
    Replacement::draft_throws(0x141ef38e0, "unknown_141ef38e0", fn_141ef38e0 as *const (), Throws::Caught), // sub_141ef38e0
    Replacement::draft_throws(0x141ef40e0, "unknown_141ef40e0", fn_141ef40e0 as *const (), Throws::Discard), // sub_141ef40e0
    Replacement::draft_throws(0x141ef4210, "unknown_141ef4210", fn_141ef4210 as *const (), Throws::Discard), // sub_141ef4210
    Replacement::draft(0x141ef4360, "unknown_141ef4360", fn_141ef4360 as *const ()), // sub_141ef4360
    Replacement::draft(0x141ef4380, "unknown_141ef4380", fn_141ef4380 as *const ()), // sub_141ef4380
    Replacement::draft(0x141ef4550, "unknown_141ef4550", fn_141ef4550 as *const ()), // sub_141ef4550
    Replacement::draft_throws(0x141ef4620, "unknown_141ef4620", fn_141ef4620 as *const (), Throws::Caught), // sub_141ef4620
    Replacement::draft_throws(0x141ef4820, "unknown_141ef4820", fn_141ef4820 as *const (), Throws::Caught), // sub_141ef4820
    Replacement::draft(0x141ef4900, "unknown_141ef4900", tpl_34994::<0x143cd6c68, 0x143880bf8, 0x143880bf0, 0x141834a78, 0x1426119d0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ef4900
    Replacement::draft(0x141ef4970, "unknown_141ef4970", fn_141ef4970 as *const ()), // sub_141ef4970
    Replacement::draft(0x141ef4ab0, "unknown_141ef4ab0", tpl_40847::<0x141075a40> as *const ()), // sub_141ef4ab0
    Replacement::draft(0x141ef4ad0, "unknown_141ef4ad0", fn_141ef4ad0 as *const ()), // sub_141ef4ad0
    Replacement::draft(0x141ef4b10, "unknown_141ef4b10", tpl_34994::<0x143cd6c78, 0x143880bd8, 0x143880bd0, 0x141834a78, 0x1426119b0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ef4b10
    Replacement::draft(0x141ef4b80, "unknown_141ef4b80", tpl_94875::<0x8, 0xc> as *const ()), // sub_141ef4b80
    Replacement::draft(0x141ef4c40, "unknown_141ef4c40", tpl_40465::<0x8, 0xc> as *const ()), // sub_141ef4c40
    Replacement::draft(0x141ef4c80, "unknown_141ef4c80", tpl_34684::<0x1401de830> as *const ()), // sub_141ef4c80
    Replacement::draft(0x141ef4cc0, "unknown_141ef4cc0", tpl_34994::<0x143cd6ccc, 0x143880be8, 0x143880be0, 0x141834a78, 0x1426119c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ef4cc0
    Replacement::draft(0x141ef4d30, "unknown_141ef4d30", tpl_35252 as *const ()), // sub_141ef4d30
    Replacement::draft(0x141ef4d50, "unknown_141ef4d50", tpl_34994::<0x143cd6c60, 0x143880bc8, 0x143880bc0, 0x141834a78, 0x1426119a0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ef4d50
    Replacement::draft(0x141ef4dc0, "unknown_141ef4dc0", tpl_35332 as *const ()), // sub_141ef4dc0
    Replacement::draft(0x141ef4dd0, "unknown_141ef4dd0", tpl_36277::<0x8> as *const ()), // sub_141ef4dd0
    Replacement::draft(0x141ef4df0, "unknown_141ef4df0", tpl_369 as *const ()), // sub_141ef4df0
    Replacement::draft(0x141ef4e00, "unknown_141ef4e00", tpl_36278::<0x8> as *const ()), // sub_141ef4e00
    Replacement::draft(0x141ef4e10, "unknown_141ef4e10", tpl_35253 as *const ()), // sub_141ef4e10
    Replacement::draft(0x141ef4e20, "unknown_141ef4e20", tpl_34318 as *const ()), // sub_141ef4e20
    Replacement::draft(0x141ef4e40, "unknown_141ef4e40", tpl_34319::<0x143cd6c5c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ef4e40
    Replacement::draft(0x141ef4ea0, "unknown_141ef4ea0", tpl_37729 as *const ()), // sub_141ef4ea0
    Replacement::draft_throws(0x141ef4ec0, "unknown_141ef4ec0", tpl_37728::<0x14241c0a0> as *const (), Throws::Discard), // sub_141ef4ec0
    Replacement::draft(0x141ef4ee0, "unknown_141ef4ee0", tpl_37731 as *const ()), // sub_141ef4ee0
    Replacement::draft(0x141ef4ef0, "unknown_141ef4ef0", tpl_361 as *const ()), // sub_141ef4ef0
    Replacement::draft(0x141ef4f00, "unknown_141ef4f00", tpl_496 as *const ()), // sub_141ef4f00
    Replacement::draft(0x141ef4f10, "unknown_141ef4f10", tpl_361 as *const ()), // sub_141ef4f10
    Replacement::draft(0x141ef4f20, "unknown_141ef4f20", tpl_548::<0x8> as *const ()), // sub_141ef4f20
    Replacement::draft(0x141ef4f30, "unknown_141ef4f30", tpl_548::<0xc> as *const ()), // sub_141ef4f30
    Replacement::draft(0x141ef4f40, "unknown_141ef4f40", tpl_473 as *const ()), // sub_141ef4f40
    Replacement::draft(0x141ef4f50, "unknown_141ef4f50", tpl_37730::<0xc> as *const ()), // sub_141ef4f50
    Replacement::draft_throws(0x141ef4f70, "unknown_141ef4f70", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_141ef4f70
    Replacement::draft(0x141ef4f90, "unknown_141ef4f90", tpl_895::<0x8> as *const ()), // sub_141ef4f90
    Replacement::draft(0x141ef4fb0, "unknown_141ef4fb0", tpl_361 as *const ()), // sub_141ef4fb0
    Replacement::draft(0x141ef4fc0, "unknown_141ef4fc0", tpl_361 as *const ()), // sub_141ef4fc0
    Replacement::draft(0x141ef4fd0, "unknown_141ef4fd0", tpl_895::<0x8> as *const ()), // sub_141ef4fd0
    Replacement::draft(0x141ef4ff0, "unknown_141ef4ff0", tpl_361 as *const ()), // sub_141ef4ff0
    Replacement::draft(0x141ef5000, "unknown_141ef5000", tpl_37730::<0x8> as *const ()), // sub_141ef5000
    Replacement::draft(0x141ef5020, "unknown_141ef5020", tpl_34443 as *const ()), // sub_141ef5020
    Replacement::draft(0x141ef5030, "unknown_141ef5030", tpl_34318 as *const ()), // sub_141ef5030
    Replacement::draft(0x141ef5050, "unknown_141ef5050", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ef5050
    Replacement::draft(0x141ef50b0, "unknown_141ef50b0", tpl_35253 as *const ()), // sub_141ef50b0
    Replacement::draft(0x141ef50c0, "unknown_141ef50c0", tpl_473 as *const ()), // sub_141ef50c0
    Replacement::draft(0x141ef50d0, "unknown_141ef50d0", tpl_37733 as *const ()), // sub_141ef50d0
    Replacement::draft(0x141ef50f0, "unknown_141ef50f0", tpl_23999 as *const ()), // sub_141ef50f0
    Replacement::draft(0x141ef5130, "unknown_141ef5130", tpl_895::<0x8> as *const ()), // sub_141ef5130
    Replacement::draft_throws(0x141ef5150, "unknown_141ef5150", fn_141ef5150 as *const (), Throws::Discard), // sub_141ef5150
    Replacement::draft_throws(0x141ef5170, "unknown_141ef5170", tpl_35998::<0x141073580, 0x12> as *const (), Throws::Discard), // sub_141ef5170
    Replacement::draft_throws(0x141ef5190, "unknown_141ef5190", fn_141ef5190 as *const (), Throws::Discard), // sub_141ef5190
    Replacement::draft_throws(0x141ef52a0, "unknown_141ef52a0", tpl_35874::<0x141073280, 0x8, 0xc, 0x10> as *const (), Throws::Discard), // sub_141ef52a0
    Replacement::draft_throws(0x141ef52e0, "unknown_141ef52e0", tpl_35874::<0x141073280, 0x8, 0xb, 0x10> as *const (), Throws::Discard), // sub_141ef52e0
    Replacement::draft_throws(0x141ef5320, "unknown_141ef5320", tpl_41573::<0x141073280, 0x141073580, 0x10, 0x8, 0x1429868d0, 0x11, 0x14, 0xffffffffffffffff, 0x1421158a0, 0x141ef539c> as *const (), Throws::Discard), // sub_141ef5320
    Replacement::draft_throws(0x141ef53b0, "unknown_141ef53b0", tpl_41573::<0x141073280, 0x141073580, 0x10, 0x8, 0x1429868d0, 0x17, 0x13, 0xffffffffffffffff, 0x1421158a0, 0x141ef542c> as *const (), Throws::Discard), // sub_141ef53b0
    Replacement::draft_throws(0x141ef5440, "unknown_141ef5440", fn_141ef5440 as *const (), Throws::Discard), // sub_141ef5440
    Replacement::draft_throws(0x141ef5470, "unknown_141ef5470", fn_141ef5470 as *const (), Throws::Discard), // sub_141ef5470
    Replacement::draft_throws(0x141ef5500, "unknown_141ef5500", tpl_35998::<0x141073580, 0x2> as *const (), Throws::Discard), // sub_141ef5500
    Replacement::draft_throws(0x141ef5520, "unknown_141ef5520", fn_141ef5520 as *const (), Throws::Discard), // sub_141ef5520
    Replacement::draft_throws(0x141ef55b0, "unknown_141ef55b0", fn_141ef55b0 as *const (), Throws::Discard), // sub_141ef55b0
    Replacement::draft_throws(0x141ef55e0, "unknown_141ef55e0", fn_141ef55e0 as *const (), Throws::Discard), // sub_141ef55e0
    Replacement::draft_throws(0x141ef5660, "unknown_141ef5660", fn_141ef5660 as *const (), Throws::Discard), // sub_141ef5660
    Replacement::draft_throws(0x141ef5700, "unknown_141ef5700", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143c8d0a0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141ef5839, 0x141ef584b, 0x141ef5839, 0x141ef584b> as *const (), Throws::Discard), // sub_141ef5700
    Replacement::draft_throws(0x141ef5860, "unknown_141ef5860", fn_141ef5860 as *const (), Throws::Discard), // sub_141ef5860
    Replacement::draft_throws(0x141ef5910, "unknown_141ef5910", fn_141ef5910 as *const (), Throws::Discard), // sub_141ef5910
    Replacement::draft(0x141ef59f0, "unknown_141ef59f0", tpl_41581::<0x143a342e8, 0x268, 0x48, 0x143a2acfc, 0x142416650> as *const ()), // sub_141ef59f0
    Replacement::draft_throws(0x141ef5a40, "unknown_141ef5a40", tpl_35890::<0x78, 0x143c8d014, 0x580e0, 0x1403eaa40, 0x143110698, 0x142116b30, 0x141ef5a83> as *const (), Throws::Discard), // sub_141ef5a40
    Replacement::draft_throws(0x141ef5a90, "unknown_141ef5a90", fn_141ef5a90 as *const (), Throws::Discard), // sub_141ef5a90
    Replacement::draft_throws(0x141ef5c30, "unknown_141ef5c30", fn_141ef5c30 as *const (), Throws::Discard), // sub_141ef5c30
    Replacement::draft_throws(0x141ef5d10, "unknown_141ef5d10", fn_141ef5d10 as *const (), Throws::Discard), // sub_141ef5d10
    Replacement::draft_throws(0x141ef5d40, "unknown_141ef5d40", fn_141ef5d40 as *const (), Throws::Caught), // sub_141ef5d40
    Replacement::draft(0x141ef6020, "unknown_141ef6020", tpl_41584::<0x8, 0x14397d8e3, 0x10> as *const ()), // sub_141ef6020
    Replacement::draft(0x141ef6050, "unknown_141ef6050", fn_141ef6050 as *const ()), // sub_141ef6050
    Replacement::draft_throws(0x141ef6070, "unknown_141ef6070", tpl_35926::<0x78, 0x143c8cec4, 0x580e0, 0x8, 0xd9, 0x10, 0x143110698, 0x142116b30, 0x141ef60cc> as *const (), Throws::Discard), // sub_141ef6070
    Replacement::draft_throws(0x141ef60e0, "unknown_141ef60e0", fn_141ef60e0 as *const (), Throws::Discard), // sub_141ef60e0
    Replacement::draft_throws(0x141ef6170, "unknown_141ef6170", fn_141ef6170 as *const (), Throws::Discard), // sub_141ef6170
    Replacement::draft_throws(0x141ef6220, "unknown_141ef6220", fn_141ef6220 as *const (), Throws::Discard), // sub_141ef6220
    Replacement::draft(0x141ef62d0, "unknown_141ef62d0", fn_141ef62d0 as *const ()), // sub_141ef62d0
    Replacement::draft_throws(0x141ef6310, "unknown_141ef6310", fn_141ef6310 as *const (), Throws::Discard), // sub_141ef6310
    Replacement::draft(0x141ef6380, "unknown_141ef6380", tpl_41581::<0x143a342e8, 0x268, 0x48, 0x143a2aebc, 0x142416650> as *const ()), // sub_141ef6380
    Replacement::draft_throws(0x141ef63d0, "unknown_141ef63d0", fn_141ef63d0 as *const (), Throws::Caught), // sub_141ef63d0
    Replacement::draft_throws(0x141ef66e0, "unknown_141ef66e0", fn_141ef66e0 as *const (), Throws::Caught), // sub_141ef66e0
    Replacement::draft_throws(0x141ef6b70, "unknown_141ef6b70", fn_141ef6b70 as *const (), Throws::Caught), // sub_141ef6b70
    Replacement::draft(0x141ef6dd0, "unknown_141ef6dd0", tpl_41584::<0x8, 0x143a28c0e, 0x10> as *const ()), // sub_141ef6dd0
    Replacement::draft_throws(0x141ef6e00, "unknown_141ef6e00", fn_141ef6e00 as *const (), Throws::Discard), // sub_141ef6e00
    Replacement::draft_throws(0x141ef6f80, "unknown_141ef6f80", fn_141ef6f80 as *const (), Throws::Caught), // sub_141ef6f80
    Replacement::draft_throws(0x141ef71f0, "unknown_141ef71f0", fn_141ef71f0 as *const (), Throws::Discard), // sub_141ef71f0
    Replacement::draft_throws(0x141ef74d0, "unknown_141ef74d0", fn_141ef74d0 as *const (), Throws::Discard), // sub_141ef74d0
    Replacement::draft(0x141ef7510, "unknown_141ef7510", fn_141ef7510 as *const ()), // sub_141ef7510
    Replacement::draft_throws(0x141ef7560, "unknown_141ef7560", fn_141ef7560 as *const (), Throws::Discard), // sub_141ef7560
    Replacement::draft(0x141ef75e0, "unknown_141ef75e0", fn_141ef75e0 as *const ()), // sub_141ef75e0
    Replacement::draft(0x141ef76b0, "unknown_141ef76b0", tpl_41599::<0x8, 0x10> as *const ()), // sub_141ef76b0
    Replacement::draft(0x141ef76e0, "unknown_141ef76e0", tpl_41599::<0x8, 0x10> as *const ()), // sub_141ef76e0
    Replacement::draft(0x141ef7710, "unknown_141ef7710", tpl_559 as *const ()), // sub_141ef7710
    Replacement::draft(0x141ef7720, "unknown_141ef7720", fn_141ef7720 as *const ()), // sub_141ef7720
    Replacement::draft(0x141ef7750, "unknown_141ef7750", tpl_559 as *const ()), // sub_141ef7750
    Replacement::draft(0x141ef7760, "unknown_141ef7760", tpl_559 as *const ()), // sub_141ef7760
    Replacement::draft_throws(0x141ef7770, "unknown_141ef7770", fn_141ef7770 as *const (), Throws::Discard), // sub_141ef7770
    Replacement::draft(0x141ef7860, "unknown_141ef7860", tpl_37935::<0x8, 0x10> as *const ()), // sub_141ef7860
    Replacement::draft(0x141ef7880, "unknown_141ef7880", tpl_559 as *const ()), // sub_141ef7880
    Replacement::draft(0x141ef7890, "unknown_141ef7890", tpl_559 as *const ()), // sub_141ef7890
    Replacement::draft(0x141ef78a0, "unknown_141ef78a0", fn_141ef78a0 as *const ()), // sub_141ef78a0
    Replacement::draft_throws(0x141ef78f0, "luau_141ef78f0", fn_141ef78f0 as *const (), Throws::Caught), // sub_141ef78f0
    Replacement::draft(0x141ef7b00, "luau_141ef7b00", tpl_559 as *const ()), // sub_141ef7b00
    Replacement::draft_throws(0x141ef7b10, "luau_141ef7b10", fn_141ef7b10 as *const (), Throws::Discard), // sub_141ef7b10
    Replacement::draft_throws(0x141ef7c80, "luau_141ef7c80", fn_141ef7c80 as *const (), Throws::Discard), // sub_141ef7c80
    Replacement::draft_throws(0x141ef7e20, "luau_141ef7e20", fn_141ef7e20 as *const (), Throws::Discard), // sub_141ef7e20
    Replacement::draft_throws(0x141ef7eb0, "luau_141ef7eb0", fn_141ef7eb0 as *const (), Throws::Discard), // sub_141ef7eb0
    Replacement::draft_throws(0x141ef7f00, "luau_141ef7f00", fn_141ef7f00 as *const (), Throws::Discard), // sub_141ef7f00
    Replacement::draft_throws(0x141ef7f30, "luau_141ef7f30", fn_141ef7f30 as *const (), Throws::Discard), // sub_141ef7f30
    Replacement::draft_throws(0x141ef7f60, "luau_141ef7f60", fn_141ef7f60 as *const (), Throws::Caught), // sub_141ef7f60
    Replacement::draft_throws(0x141ef82c0, "luau_141ef82c0", fn_141ef82c0 as *const (), Throws::Caught), // sub_141ef82c0
    Replacement::draft_throws(0x141ef88d0, "luau_141ef88d0", fn_141ef88d0 as *const (), Throws::Caught), // sub_141ef88d0
    Replacement::draft_throws(0x141ef8d90, "luau_141ef8d90", fn_141ef8d90 as *const (), Throws::Caught), // sub_141ef8d90
    Replacement::draft_throws(0x141ef93a0, "luau_141ef93a0", fn_141ef93a0 as *const (), Throws::Caught), // sub_141ef93a0
    Replacement::draft_throws(0x141ef95e0, "luau_141ef95e0", fn_141ef95e0 as *const (), Throws::Caught), // sub_141ef95e0
    Replacement::draft_throws(0x141ef9ca0, "luau_141ef9ca0", fn_141ef9ca0 as *const (), Throws::Discard), // sub_141ef9ca0
    Replacement::draft_throws(0x141ef9d60, "luau_141ef9d60", fn_141ef9d60 as *const (), Throws::Caught), // sub_141ef9d60
    Replacement::draft_throws(0x141ef9f80, "luau_141ef9f80", fn_141ef9f80 as *const (), Throws::Caught), // sub_141ef9f80
    Replacement::draft_throws(0x141efa1a0, "luau_141efa1a0", fn_141efa1a0 as *const (), Throws::Discard), // sub_141efa1a0
    Replacement::draft_throws(0x141efa320, "luau_141efa320", fn_141efa320 as *const (), Throws::Discard), // sub_141efa320
    Replacement::draft_throws(0x141efa350, "luau_141efa350", fn_141efa350 as *const (), Throws::Discard), // sub_141efa350
    Replacement::draft_throws(0x141efa3a0, "luau_141efa3a0", fn_141efa3a0 as *const (), Throws::Discard), // sub_141efa3a0
    Replacement::draft_throws(0x141efa940, "luau_141efa940", fn_141efa940 as *const (), Throws::Discard), // sub_141efa940
    Replacement::draft_throws(0x141efab40, "luau_141efab40", fn_141efab40 as *const (), Throws::Discard), // sub_141efab40
    Replacement::draft_throws(0x141efad10, "luau_141efad10", fn_141efad10 as *const (), Throws::Discard), // sub_141efad10
    Replacement::draft_throws(0x141efb000, "luau_141efb000", fn_141efb000 as *const (), Throws::Discard), // sub_141efb000
    Replacement::draft_throws(0x141efb220, "luau_141efb220", fn_141efb220 as *const (), Throws::Caught), // sub_141efb220
    Replacement::draft_throws(0x141efb650, "luau_141efb650", fn_141efb650 as *const (), Throws::Caught), // sub_141efb650
    Replacement::draft_throws(0x141efb9e0, "luau_141efb9e0", fn_141efb9e0 as *const (), Throws::Discard), // sub_141efb9e0
    Replacement::draft_throws(0x141efbcc0, "luau_141efbcc0", fn_141efbcc0 as *const (), Throws::Discard), // sub_141efbcc0
    Replacement::draft_throws(0x141efbec0, "luau_141efbec0", fn_141efbec0 as *const (), Throws::Discard), // sub_141efbec0
    Replacement::draft_throws(0x141efc090, "luau_141efc090", fn_141efc090 as *const (), Throws::Discard), // sub_141efc090
    Replacement::draft_throws(0x141efc2c0, "luau_141efc2c0", fn_141efc2c0 as *const (), Throws::Discard), // sub_141efc2c0
    Replacement::draft_throws(0x141efc4e0, "luau_141efc4e0", fn_141efc4e0 as *const (), Throws::Discard), // sub_141efc4e0
    Replacement::draft_throws(0x141efc580, "luau_141efc580", fn_141efc580 as *const (), Throws::Discard), // sub_141efc580
    Replacement::draft_throws(0x141efc610, "luau_141efc610", fn_141efc610 as *const (), Throws::Discard), // sub_141efc610
    Replacement::draft_throws(0x141efc950, "luau_141efc950", fn_141efc950 as *const (), Throws::Discard), // sub_141efc950
    Replacement::draft_throws(0x141efca20, "luau_141efca20", fn_141efca20 as *const (), Throws::Discard), // sub_141efca20
    Replacement::draft_throws(0x141efcbb0, "luau_141efcbb0", fn_141efcbb0 as *const (), Throws::Discard), // sub_141efcbb0
    Replacement::draft_throws(0x141efcd50, "luau_141efcd50", fn_141efcd50 as *const (), Throws::Discard), // sub_141efcd50
    Replacement::draft_throws(0x141efcf00, "luau_141efcf00", fn_141efcf00 as *const (), Throws::Discard), // sub_141efcf00
    Replacement::draft_throws(0x141efd2a0, "luau_141efd2a0", fn_141efd2a0 as *const (), Throws::Discard), // sub_141efd2a0
    Replacement::draft_throws(0x141efd600, "luau_141efd600", fn_141efd600 as *const (), Throws::Caught), // sub_141efd600
    Replacement::draft_throws(0x141efd800, "luau_141efd800", fn_141efd800 as *const (), Throws::Caught), // sub_141efd800
    Replacement::draft_throws(0x141efd910, "luau_141efd910", fn_141efd910 as *const (), Throws::Discard), // sub_141efd910
    Replacement::draft_throws(0x141efdc00, "luau_141efdc00", fn_141efdc00 as *const (), Throws::Discard), // sub_141efdc00
    Replacement::draft_throws(0x141efdd20, "luau_141efdd20", fn_141efdd20 as *const (), Throws::Caught), // sub_141efdd20
    Replacement::draft_throws(0x141efdde0, "luau_141efdde0", fn_141efdde0 as *const (), Throws::Discard), // sub_141efdde0
    Replacement::draft_throws(0x141efdf50, "luau_141efdf50", fn_141efdf50 as *const (), Throws::Discard), // sub_141efdf50
    Replacement::draft_throws(0x141efe160, "luau_141efe160", fn_141efe160 as *const (), Throws::Discard), // sub_141efe160
    Replacement::draft_throws(0x141efe210, "luau_141efe210", fn_141efe210 as *const (), Throws::Discard), // sub_141efe210
    Replacement::draft_throws(0x141efe360, "luau_141efe360", tpl_41310::<0x10, 0x8, 0x1429868d0, 0x78, 0x143c8ceec, 0x580e0, 0xc8, 0xc0, 0x14104e810, 0x1433710e0, 0x142116b30, 0x2, 0x1421158a0, 0x141efe3e6, 0x141efe3f8> as *const (), Throws::Discard), // sub_141efe360
    Replacement::draft_throws(0x141efe400, "luau_141efe400", fn_141efe400 as *const (), Throws::Discard), // sub_141efe400
    Replacement::draft_throws(0x141efe550, "luau_141efe550", fn_141efe550 as *const (), Throws::Caught), // sub_141efe550
    Replacement::draft_throws(0x141efe940, "luau_141efe940", fn_141efe940 as *const (), Throws::Discard), // sub_141efe940
    Replacement::draft_throws(0x141efe9e0, "luau_141efe9e0", fn_141efe9e0 as *const (), Throws::Discard), // sub_141efe9e0
    Replacement::draft_throws(0x141efeba0, "luau_141efeba0", tpl_838::<0x141efed90> as *const (), Throws::Discard), // sub_141efeba0
    Replacement::draft_throws(0x141efebb0, "luau_141efebb0", fn_141efebb0 as *const (), Throws::Discard), // sub_141efebb0
    Replacement::draft_throws(0x141efed90, "luau_141efed90", fn_141efed90 as *const (), Throws::Discard), // sub_141efed90
    Replacement::draft_throws(0x141efee30, "luau_141efee30", fn_141efee30 as *const (), Throws::Caught), // sub_141efee30
    Replacement::draft_throws(0x141efefe0, "luau_141efefe0", fn_141efefe0 as *const (), Throws::Discard), // sub_141efefe0
    Replacement::draft_throws(0x141eff090, "luau_141eff090", fn_141eff090 as *const (), Throws::Discard), // sub_141eff090
    Replacement::draft_throws(0x141eff1b0, "luau_141eff1b0", tpl_41636::<0x1410729e0, 0x8, 0x10, 0x1429868d0, 0x1420ecd00, 0x1424b90b0, 0x1, 0x1421158a0, 0x14336f9b0, 0x142116b30, 0x141eff26c, 0x141eff27c> as *const (), Throws::Discard), // sub_141eff1b0
    Replacement::draft_throws(0x141eff290, "luau_141eff290", fn_141eff290 as *const (), Throws::Discard), // sub_141eff290
    Replacement::draft_throws(0x141eff430, "luau_141eff430", tpl_41636::<0x1410729e0, 0x8, 0x10, 0x1429868d0, 0x1420ecd00, 0x1424b9270, 0x1, 0x1421158a0, 0x14336f9b0, 0x142116b30, 0x141eff4ec, 0x141eff4fc> as *const (), Throws::Discard), // sub_141eff430
    Replacement::draft_throws(0x141eff510, "luau_141eff510", fn_141eff510 as *const (), Throws::Discard), // sub_141eff510
    Replacement::draft_throws(0x141eff6b0, "luau_141eff6b0", fn_141eff6b0 as *const (), Throws::Discard), // sub_141eff6b0
    Replacement::draft_throws(0x141eff700, "luau_141eff700", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143ccbf30, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141eff839, 0x141eff84b, 0x141eff839, 0x141eff84b> as *const (), Throws::Discard), // sub_141eff700
    Replacement::draft_throws(0x141eff860, "luau_141eff860", fn_141eff860 as *const (), Throws::Discard), // sub_141eff860
    Replacement::draft_throws(0x141eff8c0, "luau_141eff8c0", fn_141eff8c0 as *const (), Throws::Discard), // sub_141eff8c0
    Replacement::draft_throws(0x141effd50, "luau_141effd50", fn_141effd50 as *const (), Throws::Discard), // sub_141effd50
    Replacement::draft_throws(0x141effe60, "luau_141effe60", fn_141effe60 as *const (), Throws::Discard), // sub_141effe60
    Replacement::draft_throws(0x141efff60, "luau_141efff60", tpl_41643::<0x78, 0x8, 0x10, 0x143cc56c8, 0x580e0, 0x1429868d0, 0xd, 0x1421158a0, 0x141f00022> as *const (), Throws::Discard), // sub_141efff60
    Replacement::draft_throws(0x141f00030, "luau_141f00030", tpl_41643::<0x78, 0x8, 0x10, 0x143cc56c8, 0x580e0, 0x1429868d0, 0xc, 0x1421158a0, 0x141f000f2> as *const (), Throws::Discard), // sub_141f00030
    Replacement::draft(0x141f00100, "luau_141f00100", tpl_34443 as *const ()), // sub_141f00100
    Replacement::draft_throws(0x141f00110, "luau_141f00110", tpl_41644::<0x1429868d0, 0x143cd6cc0, 0x140271b50, 0x1421158a0, 0x141f00215> as *const (), Throws::Discard), // sub_141f00110
    Replacement::draft_throws(0x141f00220, "luau_141f00220", tpl_101353::<0x14310c350, 0x1406569f0, 0x141072f30, 0x1429868d0, 0x143cd6cc0, 0x140271b50, 0x1401fa730, 0x1403ff090, 0x141e1cfc0, 0x1421158a0, 0x141f00424> as *const (), Throws::Discard), // sub_141f00220
    Replacement::draft(0x141f00430, "luau_141f00430", tpl_34443 as *const ()), // sub_141f00430
    Replacement::draft_throws(0x141f00440, "luau_141f00440", tpl_41644::<0x1429868d0, 0x143cd6cc0, 0x140271b50, 0x1421158a0, 0x141f00545> as *const (), Throws::Discard), // sub_141f00440
    Replacement::draft(0x141f00550, "luau_141f00550", tpl_361 as *const ()), // sub_141f00550
    Replacement::draft_throws(0x141f00560, "luau_141f00560", tpl_101353::<0x14310c350, 0x1406569f0, 0x141072f30, 0x1429868d0, 0x143cd6cc0, 0x140271b50, 0x1401fa730, 0x1403ff090, 0x141e1ce50, 0x1421158a0, 0x141f00764> as *const (), Throws::Discard), // sub_141f00560
    Replacement::draft(0x141f00770, "luau_141f00770", tpl_559 as *const ()), // sub_141f00770
    Replacement::draft_throws(0x141f00780, "luau_141f00780", fn_141f00780 as *const (), Throws::Discard), // sub_141f00780
    Replacement::draft_throws(0x141f00970, "luau_141f00970", fn_141f00970 as *const (), Throws::Discard), // sub_141f00970
    Replacement::draft_throws(0x141f00bc0, "luau_141f00bc0", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cc6d44, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f00cf9, 0x141f00d0b, 0x141f00cf9, 0x141f00d0b> as *const (), Throws::Discard), // sub_141f00bc0
    Replacement::draft_throws(0x141f00d20, "luau_141f00d20", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cc7394, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f00e59, 0x141f00e6b, 0x141f00e59, 0x141f00e6b> as *const (), Throws::Discard), // sub_141f00d20
    Replacement::draft_throws(0x141f00e80, "luau_141f00e80", fn_141f00e80 as *const (), Throws::Discard), // sub_141f00e80
    Replacement::draft_throws(0x141f01090, "luau_141f01090", fn_141f01090 as *const (), Throws::Discard), // sub_141f01090
    Replacement::draft_throws(0x141f01150, "luau_141f01150", fn_141f01150 as *const (), Throws::Discard), // sub_141f01150
    Replacement::draft_throws(0x141f011f0, "luau_141f011f0", fn_141f011f0 as *const (), Throws::Discard), // sub_141f011f0
    Replacement::draft_throws(0x141f012b0, "luau_141f012b0", fn_141f012b0 as *const (), Throws::Discard), // sub_141f012b0
    Replacement::draft_throws(0x141f01370, "luau_141f01370", fn_141f01370 as *const (), Throws::Discard), // sub_141f01370
    Replacement::draft_throws(0x141f01430, "luau_141f01430", fn_141f01430 as *const (), Throws::Discard), // sub_141f01430
    Replacement::draft_throws(0x141f01900, "luau_141f01900", fn_141f01900 as *const (), Throws::Discard), // sub_141f01900
    Replacement::draft_throws(0x141f01a20, "luau_141f01a20", fn_141f01a20 as *const (), Throws::Discard), // sub_141f01a20
    Replacement::draft_throws(0x141f01b40, "luau_141f01b40", fn_141f01b40 as *const (), Throws::Discard), // sub_141f01b40
    Replacement::draft_throws(0x141f01c50, "luau_141f01c50", fn_141f01c50 as *const (), Throws::Discard), // sub_141f01c50
    Replacement::draft_throws(0x141f01d60, "luau_141f01d60", fn_141f01d60 as *const (), Throws::Discard), // sub_141f01d60
    Replacement::draft_throws(0x141f01f10, "luau_141f01f10", fn_141f01f10 as *const (), Throws::Discard), // sub_141f01f10
    Replacement::draft_throws(0x141f02030, "luau_141f02030", fn_141f02030 as *const (), Throws::Discard), // sub_141f02030
    Replacement::draft_throws(0x141f021d0, "luau_141f021d0", fn_141f021d0 as *const (), Throws::Discard), // sub_141f021d0
    Replacement::draft_throws(0x141f022e0, "luau_141f022e0", fn_141f022e0 as *const (), Throws::Discard), // sub_141f022e0
    Replacement::draft_throws(0x141f02660, "luau_141f02660", fn_141f02660 as *const (), Throws::Discard), // sub_141f02660
    Replacement::draft_throws(0x141f02a30, "luau_141f02a30", fn_141f02a30 as *const (), Throws::Discard), // sub_141f02a30
    Replacement::draft_throws(0x141f02c20, "luau_141f02c20", fn_141f02c20 as *const (), Throws::Discard), // sub_141f02c20
    Replacement::draft_throws(0x141f02cd0, "luau_141f02cd0", fn_141f02cd0 as *const (), Throws::Discard), // sub_141f02cd0
    Replacement::draft_throws(0x141f02d80, "luau_141f02d80", fn_141f02d80 as *const (), Throws::Discard), // sub_141f02d80
    Replacement::draft_throws(0x141f02e30, "luau_141f02e30", fn_141f02e30 as *const (), Throws::Discard), // sub_141f02e30
    Replacement::draft_throws(0x141f02ed0, "luau_141f02ed0", fn_141f02ed0 as *const (), Throws::Discard), // sub_141f02ed0
    Replacement::draft_throws(0x141f02f70, "luau_141f02f70", fn_141f02f70 as *const (), Throws::Discard), // sub_141f02f70
    Replacement::draft_throws(0x141f03020, "luau_141f03020", tpl_101380::<0x1403f5260, 0x141074490, 0x141074520, 0x1429868d0, 0x1420ecd00, 0x143427b00, 0x143427a60, 0x143427ab0, 0x143428120, 0x1426fd060, 0x1426fcaa0, 0x1426fce00, 0x1426fcc90, 0x1426fcfc0, 0x1426fd020, 0x1426fccc0, 0x1426fcf60, 0x1426fcb90, 0x1426fccd0, 0x1426fcb00, 0x1426fcd70, 0x1426fd070, 0x1426fcd60, 0x1426fcce0, 0x1426fcbf0, 0x1426fcb70, 0x143428460, 0x1426fd090, 0x141e23990, 0x1421158a0, 0x141f033ff> as *const (), Throws::Discard), // sub_141f03020
    Replacement::draft_throws(0x141f03410, "luau_141f03410", tpl_101380::<0x1403f5260, 0x141074490, 0x141074520, 0x1429868d0, 0x1420ecd00, 0x143427b00, 0x143427730, 0x143427ab0, 0x143428120, 0x1426fd060, 0x1426fcaa0, 0x1426fce00, 0x1426fcc90, 0x1426fcfc0, 0x1426fd020, 0x1426fccc0, 0x1426fcf60, 0x1426fcb90, 0x1426fccd0, 0x1426fcb00, 0x1426fcd70, 0x1426fd070, 0x1426fcd60, 0x1426fcce0, 0x1426fcbf0, 0x1426fcb70, 0x143428460, 0x1426fd090, 0x141e23990, 0x1421158a0, 0x141f037ef> as *const (), Throws::Discard), // sub_141f03410
    Replacement::draft_throws(0x141f03800, "luau_141f03800", tpl_101380::<0x1403f5260, 0x141074490, 0x141074520, 0x1429868d0, 0x1420ecd00, 0x143427b00, 0x143427660, 0x143427ab0, 0x143428120, 0x1426fd060, 0x1426fcaa0, 0x1426fce00, 0x1426fcc90, 0x1426fcfc0, 0x1426fd020, 0x1426fccc0, 0x1426fcf60, 0x1426fcb90, 0x1426fccd0, 0x1426fcb00, 0x1426fcd70, 0x1426fd070, 0x1426fcd60, 0x1426fcce0, 0x1426fcbf0, 0x1426fcb70, 0x143428460, 0x1426fd090, 0x141e23990, 0x1421158a0, 0x141f03bdf> as *const (), Throws::Discard), // sub_141f03800
    Replacement::draft_throws(0x141f03bf0, "luau_141f03bf0", fn_141f03bf0 as *const (), Throws::Discard), // sub_141f03bf0
    Replacement::draft_throws(0x141f03ef0, "luau_141f03ef0", fn_141f03ef0 as *const (), Throws::Discard), // sub_141f03ef0
    Replacement::draft_throws(0x141f041f0, "luau_141f041f0", fn_141f041f0 as *const (), Throws::Discard), // sub_141f041f0
    Replacement::draft_throws(0x141f044f0, "luau_141f044f0", fn_141f044f0 as *const (), Throws::Discard), // sub_141f044f0
    Replacement::draft_throws(0x141f04800, "luau_141f04800", fn_141f04800 as *const (), Throws::Discard), // sub_141f04800
    Replacement::draft_throws(0x141f04b10, "luau_141f04b10", fn_141f04b10 as *const (), Throws::Discard), // sub_141f04b10
    Replacement::draft_throws(0x141f04e20, "luau_141f04e20", fn_141f04e20 as *const (), Throws::Discard), // sub_141f04e20
    Replacement::draft_throws(0x141f051c0, "luau_141f051c0", fn_141f051c0 as *const (), Throws::Discard), // sub_141f051c0
    Replacement::draft_throws(0x141f05330, "luau_141f05330", fn_141f05330 as *const (), Throws::Discard), // sub_141f05330
    Replacement::draft_throws(0x141f05400, "luau_141f05400", fn_141f05400 as *const (), Throws::Discard), // sub_141f05400
    Replacement::draft_throws(0x141f054c0, "luau_141f054c0", fn_141f054c0 as *const (), Throws::Discard), // sub_141f054c0
    Replacement::draft_throws(0x141f05610, "luau_141f05610", fn_141f05610 as *const (), Throws::Discard), // sub_141f05610
    Replacement::draft_throws(0x141f05690, "luau_141f05690", fn_141f05690 as *const (), Throws::Discard), // sub_141f05690
    Replacement::draft_throws(0x141f05820, "luau_141f05820", fn_141f05820 as *const (), Throws::Discard), // sub_141f05820
    Replacement::draft_throws(0x141f05950, "luau_141f05950", fn_141f05950 as *const (), Throws::Discard), // sub_141f05950
    Replacement::draft_throws(0x141f05a90, "luau_141f05a90", fn_141f05a90 as *const (), Throws::Discard), // sub_141f05a90
    Replacement::draft_throws(0x141f05f90, "luau_141f05f90", fn_141f05f90 as *const (), Throws::Discard), // sub_141f05f90
    Replacement::draft_throws(0x141f06120, "luau_141f06120", fn_141f06120 as *const (), Throws::Discard), // sub_141f06120
    Replacement::draft(0x141f06250, "luau_141f06250", tpl_369 as *const ()), // sub_141f06250
    Replacement::draft_throws(0x141f06260, "luau_141f06260", fn_141f06260 as *const (), Throws::Discard), // sub_141f06260
    Replacement::draft_throws(0x141f06480, "luau_141f06480", fn_141f06480 as *const (), Throws::Discard), // sub_141f06480
    Replacement::draft_throws(0x141f06700, "luau_141f06700", fn_141f06700 as *const (), Throws::Discard), // sub_141f06700
    Replacement::draft_throws(0x141f067f0, "luau_141f067f0", fn_141f067f0 as *const (), Throws::Discard), // sub_141f067f0
    Replacement::draft_throws(0x141f06910, "luau_141f06910", fn_141f06910 as *const (), Throws::Discard), // sub_141f06910
    Replacement::draft_throws(0x141f06b00, "luau_141f06b00", fn_141f06b00 as *const (), Throws::Discard), // sub_141f06b00
    Replacement::draft_throws(0x141f06ca0, "luau_141f06ca0", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143c8d694, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f06dd9, 0x141f06deb, 0x141f06dd9, 0x141f06deb> as *const (), Throws::Discard), // sub_141f06ca0
    Replacement::draft_throws(0x141f06e00, "luau_141f06e00", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cd5ff8, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f06f39, 0x141f06f4b, 0x141f06f39, 0x141f06f4b> as *const (), Throws::Discard), // sub_141f06e00
    Replacement::draft_throws(0x141f06f60, "luau_141f06f60", fn_141f06f60 as *const (), Throws::Discard), // sub_141f06f60
    Replacement::draft_throws(0x141f06fa0, "luau_141f06fa0", fn_141f06fa0 as *const (), Throws::Discard), // sub_141f06fa0
    Replacement::draft_throws(0x141f070b0, "luau_141f070b0", fn_141f070b0 as *const (), Throws::Discard), // sub_141f070b0
    Replacement::draft_throws(0x141f071c0, "luau_141f071c0", fn_141f071c0 as *const (), Throws::Discard), // sub_141f071c0
    Replacement::draft_throws(0x141f07260, "luau_141f07260", fn_141f07260 as *const (), Throws::Discard), // sub_141f07260
    Replacement::draft_throws(0x141f07490, "luau_141f07490", fn_141f07490 as *const (), Throws::Discard), // sub_141f07490
    Replacement::draft_throws(0x141f076a0, "luau_141f076a0", fn_141f076a0 as *const (), Throws::Discard), // sub_141f076a0
    Replacement::draft_throws(0x141f077c0, "luau_141f077c0", fn_141f077c0 as *const (), Throws::Discard), // sub_141f077c0
    Replacement::draft_throws(0x141f078b0, "luau_141f078b0", fn_141f078b0 as *const (), Throws::Discard), // sub_141f078b0
    Replacement::draft_throws(0x141f07920, "luau_141f07920", fn_141f07920 as *const (), Throws::Caught), // sub_141f07920
    Replacement::draft_throws(0x141f07de0, "luau_141f07de0", fn_141f07de0 as *const (), Throws::Caught), // sub_141f07de0
    Replacement::draft_throws(0x141f08180, "luau_141f08180", fn_141f08180 as *const (), Throws::Discard), // sub_141f08180
    Replacement::draft_throws(0x141f08460, "luau_141f08460", fn_141f08460 as *const (), Throws::Discard), // sub_141f08460
    Replacement::draft_throws(0x141f08660, "luau_141f08660", fn_141f08660 as *const (), Throws::Discard), // sub_141f08660
    Replacement::draft_throws(0x141f08880, "luau_141f08880", fn_141f08880 as *const (), Throws::Discard), // sub_141f08880
    Replacement::draft_throws(0x141f08a30, "luau_141f08a30", fn_141f08a30 as *const (), Throws::Discard), // sub_141f08a30
    Replacement::draft_throws(0x141f08be0, "luau_141f08be0", fn_141f08be0 as *const (), Throws::Discard), // sub_141f08be0
    Replacement::draft(0x141f08ce0, "luau_141f08ce0", fn_141f08ce0 as *const ()), // sub_141f08ce0
    Replacement::draft_throws(0x141f08e20, "luau_141f08e20", tpl_101422::<0x1429868d0, 0x1401fa730, 0x141f08fd0, 0x1421158a0, 0x14336fa38, 0x142116bc0, 0x141f08f07, 0x141f08f1c> as *const (), Throws::Discard), // sub_141f08e20
    Replacement::draft_throws(0x141f08f30, "luau_141f08f30", tpl_41680::<0x78, 0x143c9c018, 0x580e0, 0x20, 0x18, 0x141f08fd0> as *const (), Throws::Discard), // sub_141f08f30
    Replacement::draft_throws(0x141f08fd0, "luau_141f08fd0", fn_141f08fd0 as *const (), Throws::Caught), // sub_141f08fd0
    Replacement::draft_throws(0x141f09220, "luau_141f09220", fn_141f09220 as *const (), Throws::Discard), // sub_141f09220
    Replacement::draft_throws(0x141f09330, "luau_141f09330", fn_141f09330 as *const (), Throws::Caught), // sub_141f09330
    Replacement::draft_throws(0x141f09430, "luau_141f09430", tpl_101422::<0x1429868d0, 0x1401fa730, 0x141f095e0, 0x1421158a0, 0x14336fa38, 0x142116bc0, 0x141f09517, 0x141f0952c> as *const (), Throws::Discard), // sub_141f09430
    Replacement::draft_throws(0x141f09540, "luau_141f09540", tpl_41680::<0x78, 0x143c9c018, 0x580e0, 0x20, 0x18, 0x141f095e0> as *const (), Throws::Discard), // sub_141f09540
    Replacement::draft_throws(0x141f095e0, "luau_141f095e0", fn_141f095e0 as *const (), Throws::Caught), // sub_141f095e0
    Replacement::draft_throws(0x141f098d0, "luau_141f098d0", fn_141f098d0 as *const (), Throws::Caught), // sub_141f098d0
    Replacement::draft_throws(0x141f09ee0, "luau_141f09ee0", fn_141f09ee0 as *const (), Throws::Caught), // sub_141f09ee0
    Replacement::draft_throws(0x141f0a120, "luau_141f0a120", fn_141f0a120 as *const (), Throws::Discard), // sub_141f0a120
    Replacement::draft_throws(0x141f0a210, "luau_141f0a210", fn_141f0a210 as *const (), Throws::Caught), // sub_141f0a210
    Replacement::draft_throws(0x141f0a490, "luau_141f0a490", fn_141f0a490 as *const (), Throws::Discard), // sub_141f0a490
    Replacement::draft(0x141f0a610, "luau_141f0a610", fn_141f0a610 as *const ()), // sub_141f0a610
    Replacement::draft(0x141f0a7b0, "luau_141f0a7b0", tpl_361 as *const ()), // sub_141f0a7b0
    Replacement::draft_throws(0x141f0a7c0, "luau_141f0a7c0", fn_141f0a7c0 as *const (), Throws::Discard), // sub_141f0a7c0
    Replacement::draft_throws(0x141f0a860, "luau_141f0a860", fn_141f0a860 as *const (), Throws::Discard), // sub_141f0a860
    Replacement::draft(0x141f0a8b0, "luau_141f0a8b0", fn_141f0a8b0 as *const ()), // sub_141f0a8b0
    Replacement::draft_throws(0x141f0a8e0, "luau_141f0a8e0", tpl_41692::<0x14336f838, 0x141073b20> as *const (), Throws::Discard), // sub_141f0a8e0
    Replacement::draft_throws(0x141f0a900, "luau_141f0a900", fn_141f0a900 as *const (), Throws::Discard), // sub_141f0a900
    Replacement::draft_throws(0x141f0aa30, "luau_141f0aa30", tpl_838::<0x141073910> as *const (), Throws::Discard), // sub_141f0aa30
    Replacement::draft_throws(0x141f0aa40, "luau_141f0aa40", tpl_838::<0x141073750> as *const (), Throws::Discard), // sub_141f0aa40
    Replacement::draft_throws(0x141f0aa50, "luau_141f0aa50", tpl_838::<0x141073580> as *const (), Throws::Discard), // sub_141f0aa50
    Replacement::draft_throws(0x141f0aa60, "luau_141f0aa60", tpl_838::<0x141073400> as *const (), Throws::Discard), // sub_141f0aa60
    Replacement::draft_throws(0x141f0aa70, "luau_141f0aa70", tpl_838::<0x141073280> as *const (), Throws::Discard), // sub_141f0aa70
    Replacement::draft_throws(0x141f0aa80, "luau_141f0aa80", tpl_838::<0x141073100> as *const (), Throws::Discard), // sub_141f0aa80
    Replacement::draft_throws(0x141f0aa90, "luau_141f0aa90", tpl_838::<0x141072f30> as *const (), Throws::Discard), // sub_141f0aa90
    Replacement::draft_throws(0x141f0aaa0, "luau_141f0aaa0", tpl_838::<0x141072d60> as *const (), Throws::Discard), // sub_141f0aaa0
    Replacement::draft_throws(0x141f0aab0, "luau_141f0aab0", tpl_838::<0x141072ba0> as *const (), Throws::Discard), // sub_141f0aab0
    Replacement::draft_throws(0x141f0aac0, "luau_141f0aac0", tpl_37953::<0x142119010, 0x142118e10, 0x1401fa730, 0x140400730, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310c350, 0x141ef4820, 0x141f0abb4, 0x141f0abc6, 0x141f0abdb, 0x141f0abb4, 0x141f0abc6, 0x141f0abdb> as *const (), Throws::Discard), // sub_141f0aac0
    Replacement::draft_throws(0x141f0abf0, "luau_141f0abf0", tpl_41692::<0x14310c350, 0x1406569f0> as *const (), Throws::Discard), // sub_141f0abf0
    Replacement::draft_throws(0x141f0ac10, "luau_141f0ac10", tpl_37953::<0x142119010, 0x142118e10, 0x1401fa730, 0x14040c7a0, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310aa40, 0x141ef4820, 0x141f0ad04, 0x141f0ad16, 0x141f0ad2b, 0x141f0ad04, 0x141f0ad16, 0x141f0ad2b> as *const (), Throws::Discard), // sub_141f0ac10
    Replacement::draft_throws(0x141f0ad40, "luau_141f0ad40", tpl_41694::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c95490, 0x142489ee0, 0x14047c5e0, 0x1426467e8, 0x1421158a0, 0x141f0ae44, 0x141f0ae6c, 0x141f0ae44, 0x141f0ae6c> as *const (), Throws::Discard), // sub_141f0ad40
    Replacement::draft_throws(0x141f0ae80, "luau_141f0ae80", tpl_838::<0x1410729e0> as *const (), Throws::Discard), // sub_141f0ae80
    Replacement::draft_throws(0x141f0ae90, "luau_141f0ae90", fn_141f0ae90 as *const (), Throws::Caught), // sub_141f0ae90
    Replacement::draft_throws(0x141f0afe0, "luau_141f0afe0", fn_141f0afe0 as *const (), Throws::Caught), // sub_141f0afe0
    Replacement::draft_throws(0x141f0b180, "luau_141f0b180", fn_141f0b180 as *const (), Throws::Discard), // sub_141f0b180
    Replacement::draft_throws(0x141f0b3b0, "luau_141f0b3b0", fn_141f0b3b0 as *const (), Throws::Discard), // sub_141f0b3b0
    Replacement::draft_throws(0x141f0b440, "luau_141f0b440", fn_141f0b440 as *const (), Throws::Discard), // sub_141f0b440
    Replacement::draft_throws(0x141f0b4c0, "luau_141f0b4c0", fn_141f0b4c0 as *const (), Throws::Discard), // sub_141f0b4c0
    Replacement::draft_throws(0x141f0b540, "luau_141f0b540", tpl_41461::<0x142119010, 0x142118e10, 0x142116bc0, 0x1421158a0, 0x141f0b5b4, 0x141f0b5c6> as *const (), Throws::Discard), // sub_141f0b540
    Replacement::draft_throws(0x141f0b5d0, "luau_141f0b5d0", fn_141f0b5d0 as *const (), Throws::Discard), // sub_141f0b5d0
    Replacement::draft_throws(0x141f0cd50, "luau_141f0cd50", fn_141f0cd50 as *const (), Throws::Discard), // sub_141f0cd50
    Replacement::draft_throws(0x141f0cd80, "luau_141f0cd80", fn_141f0cd80 as *const (), Throws::Caught), // sub_141f0cd80
    Replacement::draft(0x141f0d430, "luau_141f0d430", tpl_41703::<0x143a21840, 0x143a47030, 0x141072900> as *const ()), // sub_141f0d430
    Replacement::draft(0x141f0d450, "luau_141f0d450", tpl_41703::<0x143a22350, 0x143a47130, 0x141072900> as *const ()), // sub_141f0d450
    Replacement::draft(0x141f0d470, "luau_141f0d470", tpl_41703::<0x143a21c90, 0x143a46fb0, 0x141072900> as *const ()), // sub_141f0d470
    Replacement::draft(0x141f0d490, "luau_141f0d490", tpl_1::<0x143a47130> as *const ()), // sub_141f0d490
    Replacement::draft(0x141f0d4a0, "luau_141f0d4a0", tpl_1::<0x143a46fb0> as *const ()), // sub_141f0d4a0
    Replacement::draft_throws(0x141f0d4b0, "luau_141f0d4b0", tpl_37501::<0x1428e3590, 0x1410d6ca0, 0x141073fc0, 0x143823768, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141f0d4b0
    Replacement::draft_throws(0x141f0d640, "luau_141f0d640", fn_141f0d640 as *const (), Throws::Discard), // sub_141f0d640
    Replacement::draft_throws(0x141f0d910, "luau_141f0d910", fn_141f0d910 as *const (), Throws::Discard), // sub_141f0d910
    Replacement::draft_throws(0x141f0dbc0, "luau_141f0dbc0", fn_141f0dbc0 as *const (), Throws::Discard), // sub_141f0dbc0
    Replacement::draft_throws(0x141f0dc50, "luau_141f0dc50", fn_141f0dc50 as *const (), Throws::Discard), // sub_141f0dc50
    Replacement::draft_throws(0x141f0dca0, "luau_141f0dca0", fn_141f0dca0 as *const (), Throws::Discard), // sub_141f0dca0
    Replacement::draft_throws(0x141f0e270, "luau_141f0e270", fn_141f0e270 as *const (), Throws::Discard), // sub_141f0e270
    Replacement::draft_throws(0x141f0e710, "luau_141f0e710", fn_141f0e710 as *const (), Throws::Discard), // sub_141f0e710
    Replacement::draft_throws(0x141f0e8a0, "luau_141f0e8a0", fn_141f0e8a0 as *const (), Throws::Discard), // sub_141f0e8a0
    Replacement::draft_throws(0x141f0ea20, "luau_141f0ea20", fn_141f0ea20 as *const (), Throws::Discard), // sub_141f0ea20
    Replacement::draft_throws(0x141f0eb50, "luau_141f0eb50", fn_141f0eb50 as *const (), Throws::Discard), // sub_141f0eb50
    Replacement::draft_throws(0x141f0ebc0, "luau_141f0ebc0", fn_141f0ebc0 as *const (), Throws::Discard), // sub_141f0ebc0
    Replacement::draft_throws(0x141f0ec20, "luau_141f0ec20", fn_141f0ec20 as *const (), Throws::Discard), // sub_141f0ec20
    Replacement::draft(0x141f0ec80, "luau_141f0ec80", tpl_41711::<0x1424069a0, 0x8, 0x10> as *const ()), // sub_141f0ec80
    Replacement::draft(0x141f0ecc0, "luau_141f0ecc0", tpl_559 as *const ()), // sub_141f0ecc0
    Replacement::draft(0x141f0ecd0, "luau_141f0ecd0", tpl_559 as *const ()), // sub_141f0ecd0
    Replacement::draft_throws(0x141f0ece0, "luau_141f0ece0", fn_141f0ece0 as *const (), Throws::Discard), // sub_141f0ece0
    Replacement::draft_throws(0x141f0ef50, "luau_141f0ef50", fn_141f0ef50 as *const (), Throws::Caught), // sub_141f0ef50
    Replacement::draft_throws(0x141f0f070, "luau_141f0f070", fn_141f0f070 as *const (), Throws::Discard), // sub_141f0f070
    Replacement::draft(0x141f0f2b0, "luau_141f0f2b0", tpl_34994::<0x143cd6de4, 0x143880c08, 0x143880c00, 0x141834a78, 0x1426119f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f0f2b0
    Replacement::draft(0x141f0f320, "luau_141f0f320", fn_141f0f320 as *const ()), // sub_141f0f320
    Replacement::draft(0x141f0f400, "luau_141f0f400", fn_141f0f400 as *const ()), // sub_141f0f400
    Replacement::draft(0x141f0f460, "luau_141f0f460", tpl_34607::<0x1410767d0> as *const ()), // sub_141f0f460
    Replacement::draft(0x141f0f4a0, "luau_141f0f4a0", tpl_34994::<0x143cd6e48, 0x143880c18, 0x143880c10, 0x141834a78, 0x1426119e0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f0f4a0
    Replacement::draft(0x141f0f510, "luau_141f0f510", fn_141f0f510 as *const ()), // sub_141f0f510
    Replacement::draft(0x141f0f660, "luau_141f0f660", tpl_34992::<0x141076700> as *const ()), // sub_141f0f660
    Replacement::draft(0x141f0f680, "luau_141f0f680", fn_141f0f680 as *const ()), // sub_141f0f680
    Replacement::draft(0x141f0f6d0, "luau_141f0f6d0", fn_141f0f6d0 as *const ()), // sub_141f0f6d0
    Replacement::draft(0x141f0f6f0, "luau_141f0f6f0", fn_141f0f6f0 as *const ()), // sub_141f0f6f0
    Replacement::draft_throws(0x141f0f730, "luau_141f0f730", fn_141f0f730 as *const (), Throws::Caught), // sub_141f0f730
    Replacement::draft_throws(0x141f0f820, "luau_141f0f820", fn_141f0f820 as *const (), Throws::Caught), // sub_141f0f820
    Replacement::draft_throws(0x141f0fa00, "luau_141f0fa00", fn_141f0fa00 as *const (), Throws::Caught), // sub_141f0fa00
    Replacement::draft(0x141f0fad0, "luau_141f0fad0", tpl_730 as *const ()), // sub_141f0fad0
    Replacement::draft(0x141f0fae0, "luau_141f0fae0", fn_141f0fae0 as *const ()), // sub_141f0fae0
    Replacement::draft_throws(0x141f0fb00, "luau_141f0fb00", fn_141f0fb00 as *const (), Throws::Caught), // sub_141f0fb00
    Replacement::draft_throws(0x141f0fc40, "luau_141f0fc40", fn_141f0fc40 as *const (), Throws::Caught), // sub_141f0fc40
    Replacement::draft_throws(0x141f0ff40, "luau_141f0ff40", fn_141f0ff40 as *const (), Throws::Caught), // sub_141f0ff40
    Replacement::draft_throws(0x141f10120, "luau_141f10120", fn_141f10120 as *const (), Throws::Caught), // sub_141f10120
    Replacement::draft_throws(0x141f10250, "luau_141f10250", fn_141f10250 as *const (), Throws::Caught), // sub_141f10250
    Replacement::draft_throws(0x141f103f0, "luau_141f103f0", fn_141f103f0 as *const (), Throws::Caught), // sub_141f103f0
    Replacement::draft_throws(0x141f104f0, "luau_141f104f0", fn_141f104f0 as *const (), Throws::Caught), // sub_141f104f0
    Replacement::draft_throws(0x141f105f0, "luau_141f105f0", fn_141f105f0 as *const (), Throws::Caught), // sub_141f105f0
    Replacement::draft_throws(0x141f106f0, "luau_141f106f0", fn_141f106f0 as *const (), Throws::Caught), // sub_141f106f0
    Replacement::draft_throws(0x141f109c0, "luau_141f109c0", fn_141f109c0 as *const (), Throws::Caught), // sub_141f109c0
    Replacement::draft_throws(0x141f10bc0, "luau_141f10bc0", fn_141f10bc0 as *const (), Throws::Caught), // sub_141f10bc0
    Replacement::draft_throws(0x141f10c80, "luau_141f10c80", fn_141f10c80 as *const (), Throws::Discard), // sub_141f10c80
    Replacement::draft_throws(0x141f10e10, "luau_141f10e10", fn_141f10e10 as *const (), Throws::Discard), // sub_141f10e10
    Replacement::draft_throws(0x141f10e90, "luau_141f10e90", fn_141f10e90 as *const (), Throws::Caught), // sub_141f10e90
    Replacement::draft_throws(0x141f11150, "luau_141f11150", fn_141f11150 as *const (), Throws::Discard), // sub_141f11150
    Replacement::draft_throws(0x141f113b0, "luau_141f113b0", fn_141f113b0 as *const (), Throws::Discard), // sub_141f113b0
    Replacement::draft_throws(0x141f11490, "luau_141f11490", fn_141f11490 as *const (), Throws::Discard), // sub_141f11490
    Replacement::draft_throws(0x141f11550, "luau_141f11550", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cd6e28, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f11689, 0x141f1169b, 0x141f11689, 0x141f1169b> as *const (), Throws::Discard), // sub_141f11550
    Replacement::draft_throws(0x141f116b0, "luau_141f116b0", fn_141f116b0 as *const (), Throws::Caught), // sub_141f116b0
    Replacement::draft_throws(0x141f11830, "luau_141f11830", fn_141f11830 as *const (), Throws::Discard), // sub_141f11830
    Replacement::draft_throws(0x141f118f0, "luau_141f118f0", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cc69b0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f11a29, 0x141f11a3b, 0x141f11a29, 0x141f11a3b> as *const (), Throws::Discard), // sub_141f118f0
    Replacement::draft_throws(0x141f11a50, "luau_141f11a50", fn_141f11a50 as *const (), Throws::Discard), // sub_141f11a50
    Replacement::draft_throws(0x141f11bf0, "luau_141f11bf0", fn_141f11bf0 as *const (), Throws::Discard), // sub_141f11bf0
    Replacement::draft_throws(0x141f11d00, "luau_141f11d00", fn_141f11d00 as *const (), Throws::Discard), // sub_141f11d00
    Replacement::draft_throws(0x141f11d70, "luau_141f11d70", fn_141f11d70 as *const (), Throws::Discard), // sub_141f11d70
    Replacement::draft_throws(0x141f11e40, "luau_141f11e40", fn_141f11e40 as *const (), Throws::Discard), // sub_141f11e40
    Replacement::draft_throws(0x141f11f10, "luau_141f11f10", fn_141f11f10 as *const (), Throws::Discard), // sub_141f11f10
    Replacement::draft_throws(0x141f120d0, "luau_141f120d0", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143ccc40c, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f12209, 0x141f1221b, 0x141f12209, 0x141f1221b> as *const (), Throws::Discard), // sub_141f120d0
    Replacement::draft_throws(0x141f12230, "luau_141f12230", fn_141f12230 as *const (), Throws::Caught), // sub_141f12230
    Replacement::draft_throws(0x141f12540, "luau_141f12540", fn_141f12540 as *const (), Throws::Caught), // sub_141f12540
    Replacement::draft_throws(0x141f12970, "luau_141f12970", fn_141f12970 as *const (), Throws::Caught), // sub_141f12970
    Replacement::draft_throws(0x141f12c80, "luau_141f12c80", fn_141f12c80 as *const (), Throws::Discard), // sub_141f12c80
    Replacement::draft_throws(0x141f12d00, "luau_141f12d00", fn_141f12d00 as *const (), Throws::Caught), // sub_141f12d00
    Replacement::draft_throws(0x141f12de0, "luau_141f12de0", fn_141f12de0 as *const (), Throws::Caught), // sub_141f12de0
    Replacement::draft_throws(0x141f13170, "luau_141f13170", fn_141f13170 as *const (), Throws::Caught), // sub_141f13170
    Replacement::draft_throws(0x141f133c0, "luau_141f133c0", fn_141f133c0 as *const (), Throws::Caught), // sub_141f133c0
    Replacement::draft_throws(0x141f13590, "luau_141f13590", fn_141f13590 as *const (), Throws::Caught), // sub_141f13590
    Replacement::draft_throws(0x141f13760, "luau_141f13760", fn_141f13760 as *const (), Throws::Caught), // sub_141f13760
    Replacement::draft_throws(0x141f13ad0, "luau_141f13ad0", fn_141f13ad0 as *const (), Throws::Caught), // sub_141f13ad0
    Replacement::draft_throws(0x141f13d50, "luau_141f13d50", fn_141f13d50 as *const (), Throws::Caught), // sub_141f13d50
    Replacement::draft_throws(0x141f13f60, "luau_141f13f60", fn_141f13f60 as *const (), Throws::Caught), // sub_141f13f60
    Replacement::draft_throws(0x141f14010, "luau_141f14010", fn_141f14010 as *const (), Throws::Caught), // sub_141f14010
    Replacement::draft_throws(0x141f14630, "luau_141f14630", fn_141f14630 as *const (), Throws::Caught), // sub_141f14630
    Replacement::draft_throws(0x141f14b50, "luau_141f14b50", fn_141f14b50 as *const (), Throws::Caught), // sub_141f14b50
    Replacement::draft_throws(0x141f14c00, "luau_141f14c00", fn_141f14c00 as *const (), Throws::Caught), // sub_141f14c00
    Replacement::draft_throws(0x141f14ca0, "luau_141f14ca0", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cba178, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f14dd9, 0x141f14deb, 0x141f14dd9, 0x141f14deb> as *const (), Throws::Discard), // sub_141f14ca0
    Replacement::draft_throws(0x141f14e00, "luau_141f14e00", fn_141f14e00 as *const (), Throws::Discard), // sub_141f14e00
    Replacement::draft(0x141f15000, "luau_141f15000", fn_141f15000 as *const ()), // sub_141f15000
    Replacement::draft_throws(0x141f151d0, "luau_141f151d0", fn_141f151d0 as *const (), Throws::Discard), // sub_141f151d0
    Replacement::draft_throws(0x141f15240, "luau_141f15240", fn_141f15240 as *const (), Throws::Discard), // sub_141f15240
    Replacement::draft(0x141f15540, "luau_141f15540", tpl_35786::<0x8, 0x143a08428, 0x1194, 0x10> as *const ()), // sub_141f15540
    Replacement::draft_throws(0x141f15580, "luau_141f15580", fn_141f15580 as *const (), Throws::Discard), // sub_141f15580
    Replacement::draft(0x141f15880, "luau_141f15880", tpl_35786::<0x8, 0x143a08428, 0x116c, 0x10> as *const ()), // sub_141f15880
    Replacement::draft_throws(0x141f158c0, "luau_141f158c0", fn_141f158c0 as *const (), Throws::Discard), // sub_141f158c0
    Replacement::draft(0x141f15bc0, "luau_141f15bc0", tpl_35786::<0x8, 0x143a08428, 0x1180, 0x10> as *const ()), // sub_141f15bc0
    Replacement::draft_throws(0x141f15c00, "luau_141f15c00", fn_141f15c00 as *const (), Throws::Discard), // sub_141f15c00
    Replacement::draft_throws(0x141f15cb0, "luau_141f15cb0", fn_141f15cb0 as *const (), Throws::Discard), // sub_141f15cb0
    Replacement::draft_throws(0x141f15da0, "luau_141f15da0", fn_141f15da0 as *const (), Throws::Discard), // sub_141f15da0
    Replacement::draft_throws(0x141f15dc0, "luau_141f15dc0", fn_141f15dc0 as *const (), Throws::Discard), // sub_141f15dc0
    Replacement::draft_throws(0x141f15e60, "luau_141f15e60", fn_141f15e60 as *const (), Throws::Discard), // sub_141f15e60
    Replacement::draft_throws(0x141f15f00, "luau_141f15f00", fn_141f15f00 as *const (), Throws::Discard), // sub_141f15f00
    Replacement::draft_throws(0x141f15f80, "unknown_141f15f80", fn_141f15f80 as *const (), Throws::Discard), // sub_141f15f80
    Replacement::draft_throws(0x141f16080, "unknown_141f16080", fn_141f16080 as *const (), Throws::Discard), // sub_141f16080
    Replacement::draft_throws(0x141f16310, "unknown_141f16310", fn_141f16310 as *const (), Throws::Caught), // sub_141f16310
    Replacement::draft_throws(0x141f16480, "unknown_141f16480", fn_141f16480 as *const (), Throws::Discard), // sub_141f16480
    Replacement::draft_throws(0x141f16570, "unknown_141f16570", fn_141f16570 as *const (), Throws::Discard), // sub_141f16570
    Replacement::draft_throws(0x141f166c0, "unknown_141f166c0", fn_141f166c0 as *const (), Throws::Discard), // sub_141f166c0
    Replacement::draft_throws(0x141f167f0, "unknown_141f167f0", fn_141f167f0 as *const (), Throws::Discard), // sub_141f167f0
    Replacement::draft_throws(0x141f168c0, "unknown_141f168c0", fn_141f168c0 as *const (), Throws::Discard), // sub_141f168c0
    Replacement::draft(0x141f16b30, "unknown_141f16b30", fn_141f16b30 as *const ()), // sub_141f16b30
    Replacement::draft_throws(0x141f16bf0, "unknown_141f16bf0", fn_141f16bf0 as *const (), Throws::Caught), // sub_141f16bf0
    Replacement::draft_throws(0x141f16de0, "unknown_141f16de0", tpl_41770::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cccac8, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f16f22, 0x141f16f4a, 0x141f16f22, 0x141f16f4a> as *const (), Throws::Discard), // sub_141f16de0
    Replacement::draft_throws(0x141f16f60, "unknown_141f16f60", fn_141f16f60 as *const (), Throws::Caught), // sub_141f16f60
    Replacement::draft_throws(0x141f170b0, "unknown_141f170b0", tpl_41467::<0x14310c350, 0x1406569f0> as *const (), Throws::Discard), // sub_141f170b0
    Replacement::draft_throws(0x141f170d0, "unknown_141f170d0", fn_141f170d0 as *const (), Throws::Caught), // sub_141f170d0
    Replacement::draft_throws(0x141f17280, "unknown_141f17280", tpl_41770::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd6de0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f173c2, 0x141f173ea, 0x141f173c2, 0x141f173ea> as *const (), Throws::Discard), // sub_141f17280
    Replacement::draft_throws(0x141f17400, "unknown_141f17400", fn_141f17400 as *const (), Throws::Caught), // sub_141f17400
    Replacement::draft_throws(0x141f17610, "unknown_141f17610", tpl_41694::<0x142119010, 0x142118e10, 0x1401fa730, 0x143ca83d8, 0x142489ee0, 0x1404c99f0, 0x1426467e8, 0x1421158a0, 0x141f17714, 0x141f1773c, 0x141f17714, 0x141f1773c> as *const (), Throws::Discard), // sub_141f17610
    Replacement::draft_throws(0x141f17750, "unknown_141f17750", fn_141f17750 as *const (), Throws::Discard), // sub_141f17750
    Replacement::draft(0x141f17fc0, "unknown_141f17fc0", tpl_34994::<0x143cd6e60, 0x143880c38, 0x143880c30, 0x141834a78, 0x142611a60, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f17fc0
    Replacement::draft(0x141f18030, "unknown_141f18030", fn_141f18030 as *const ()), // sub_141f18030
    Replacement::draft(0x141f181c0, "unknown_141f181c0", tpl_34722 as *const ()), // sub_141f181c0
    Replacement::draft(0x141f18270, "unknown_141f18270", fn_141f18270 as *const ()), // sub_141f18270
    Replacement::draft(0x141f18360, "unknown_141f18360", fn_141f18360 as *const ()), // sub_141f18360
    Replacement::draft(0x141f183c0, "unknown_141f183c0", tpl_40933::<0x1401f2bc0> as *const ()), // sub_141f183c0
    Replacement::draft_throws(0x141f18520, "unknown_141f18520", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143c8ec20, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f18659, 0x141f1866b, 0x141f18659, 0x141f1866b> as *const (), Throws::Discard), // sub_141f18520
    Replacement::draft_throws(0x141f18680, "unknown_141f18680", tpl_35998::<0x1410771d0, 0x1c> as *const (), Throws::Discard), // sub_141f18680
    Replacement::draft_throws(0x141f186a0, "unknown_141f186a0", tpl_41778::<0x141f18d30, 0x10, 0x8, 0x1429868d0, 0x143427080, 0x14, 0x1420ecd00, 0x1421158a0, 0x141f1873b> as *const (), Throws::Discard), // sub_141f186a0
    Replacement::draft_throws(0x141f18750, "unknown_141f18750", fn_141f18750 as *const (), Throws::Discard), // sub_141f18750
    Replacement::draft_throws(0x141f187f0, "unknown_141f187f0", fn_141f187f0 as *const (), Throws::Discard), // sub_141f187f0
    Replacement::draft_throws(0x141f188a0, "unknown_141f188a0", fn_141f188a0 as *const (), Throws::Discard), // sub_141f188a0
    Replacement::draft_throws(0x141f189b0, "unknown_141f189b0", tpl_41778::<0x141f18d30, 0x10, 0x8, 0x1429868d0, 0x143427080, 0x4, 0x1420ecd00, 0x1421158a0, 0x141f18a4b> as *const (), Throws::Discard), // sub_141f189b0
    Replacement::draft_throws(0x141f18a60, "unknown_141f18a60", fn_141f18a60 as *const (), Throws::Discard), // sub_141f18a60
    Replacement::draft_throws(0x141f18ae0, "unknown_141f18ae0", fn_141f18ae0 as *const (), Throws::Discard), // sub_141f18ae0
    Replacement::draft_throws(0x141f18c30, "unknown_141f18c30", fn_141f18c30 as *const (), Throws::Discard), // sub_141f18c30
    Replacement::draft_throws(0x141f18d30, "unknown_141f18d30", fn_141f18d30 as *const (), Throws::Discard), // sub_141f18d30
    Replacement::draft_throws(0x141f18e30, "unknown_141f18e30", fn_141f18e30 as *const (), Throws::Discard), // sub_141f18e30
    Replacement::draft_throws(0x141f18ef0, "unknown_141f18ef0", fn_141f18ef0 as *const (), Throws::Discard), // sub_141f18ef0
    Replacement::draft_throws(0x141f19050, "unknown_141f19050", fn_141f19050 as *const (), Throws::Discard), // sub_141f19050
    Replacement::draft_throws(0x141f19120, "unknown_141f19120", fn_141f19120 as *const (), Throws::Discard), // sub_141f19120
    Replacement::draft_throws(0x141f191a0, "unknown_141f191a0", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cbe480, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f192d9, 0x141f192eb, 0x141f192d9, 0x141f192eb> as *const (), Throws::Discard), // sub_141f191a0
    Replacement::draft_throws(0x141f19300, "unknown_141f19300", tpl_41789::<0x141076bc0> as *const (), Throws::Discard), // sub_141f19300
    Replacement::draft_throws(0x141f19330, "unknown_141f19330", fn_141f19330 as *const (), Throws::Discard), // sub_141f19330
    Replacement::draft_throws(0x141f19440, "unknown_141f19440", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143cccd90, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f19579, 0x141f1958b, 0x141f19579, 0x141f1958b> as *const (), Throws::Discard), // sub_141f19440
    Replacement::draft_throws(0x141f195a0, "unknown_141f195a0", tpl_41789::<0x1410769f0> as *const (), Throws::Discard), // sub_141f195a0
    Replacement::draft_throws(0x141f195d0, "unknown_141f195d0", fn_141f195d0 as *const (), Throws::Discard), // sub_141f195d0
    Replacement::draft_throws(0x141f19620, "unknown_141f19620", tpl_41396::<0x1429868d0, 0x1401fa730, 0x143c8dd14, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x141f19759, 0x141f1976b, 0x141f19759, 0x141f1976b> as *const (), Throws::Discard), // sub_141f19620
    Replacement::draft_throws(0x141f19780, "unknown_141f19780", fn_141f19780 as *const (), Throws::Discard), // sub_141f19780
    Replacement::draft_throws(0x141f199b0, "unknown_141f199b0", fn_141f199b0 as *const (), Throws::Discard), // sub_141f199b0
    Replacement::draft_throws(0x141f19aa0, "unknown_141f19aa0", fn_141f19aa0 as *const (), Throws::Discard), // sub_141f19aa0
    Replacement::draft_throws(0x141f19b30, "unknown_141f19b30", fn_141f19b30 as *const (), Throws::Discard), // sub_141f19b30
    Replacement::draft_throws(0x141f19c00, "unknown_141f19c00", fn_141f19c00 as *const (), Throws::Discard), // sub_141f19c00
    Replacement::draft_throws(0x141f19dc0, "unknown_141f19dc0", fn_141f19dc0 as *const (), Throws::Discard), // sub_141f19dc0
    Replacement::draft_throws(0x141f19e50, "unknown_141f19e50", fn_141f19e50 as *const (), Throws::Discard), // sub_141f19e50
    Replacement::draft_throws(0x141f19ef0, "unknown_141f19ef0", fn_141f19ef0 as *const (), Throws::Discard), // sub_141f19ef0
    Replacement::draft_throws(0x141f19f10, "unknown_141f19f10", fn_141f19f10 as *const (), Throws::Caught), // sub_141f19f10
    Replacement::draft_throws(0x141f1a020, "unknown_141f1a020", tpl_41800::<0x10, 0x1429868d0, 0x8, 0x141076910, 0x1411780c0, 0x4, 0x1433735d0, 0x141ef4820, 0x1, 0x1421158a0, 0x141f1a0d9, 0x141f1a0eb> as *const (), Throws::Discard), // sub_141f1a020
    Replacement::draft_throws(0x141f1a100, "unknown_141f1a100", fn_141f1a100 as *const (), Throws::Discard), // sub_141f1a100
    Replacement::draft_throws(0x141f1a200, "unknown_141f1a200", fn_141f1a200 as *const (), Throws::Discard), // sub_141f1a200
    Replacement::draft_throws(0x141f1a2e0, "unknown_141f1a2e0", fn_141f1a2e0 as *const (), Throws::Discard), // sub_141f1a2e0
    Replacement::draft_throws(0x141f1a460, "unknown_141f1a460", tpl_41800::<0x10, 0x1429868d0, 0x8, 0x141076910, 0x141177f80, 0x4, 0x1433735d0, 0x141ef4820, 0x1, 0x1421158a0, 0x141f1a519, 0x141f1a52b> as *const (), Throws::Discard), // sub_141f1a460
    Replacement::draft_throws(0x141f1a540, "unknown_141f1a540", tpl_41800::<0x10, 0x1429868d0, 0x8, 0x141076910, 0x141177f80, 0x4, 0x1433735d0, 0x141ef4820, 0x1, 0x1421158a0, 0x141f1a5f9, 0x141f1a60b> as *const (), Throws::Discard), // sub_141f1a540
    Replacement::draft_throws(0x141f1a620, "unknown_141f1a620", fn_141f1a620 as *const (), Throws::Discard), // sub_141f1a620
    Replacement::draft_throws(0x141f1a720, "unknown_141f1a720", fn_141f1a720 as *const (), Throws::Discard), // sub_141f1a720
    Replacement::draft_throws(0x141f1a7f0, "unknown_141f1a7f0", fn_141f1a7f0 as *const (), Throws::Discard), // sub_141f1a7f0
    Replacement::draft_throws(0x141f1a920, "unknown_141f1a920", fn_141f1a920 as *const (), Throws::Discard), // sub_141f1a920
    Replacement::draft_throws(0x141f1a9c0, "unknown_141f1a9c0", fn_141f1a9c0 as *const (), Throws::Discard), // sub_141f1a9c0
    Replacement::draft_throws(0x141f1ac00, "unknown_141f1ac00", fn_141f1ac00 as *const (), Throws::Discard), // sub_141f1ac00
    Replacement::draft_throws(0x141f1af80, "unknown_141f1af80", fn_141f1af80 as *const (), Throws::Discard), // sub_141f1af80
    Replacement::draft_throws(0x141f1b250, "unknown_141f1b250", fn_141f1b250 as *const (), Throws::Discard), // sub_141f1b250
    Replacement::draft_throws(0x141f1b310, "unknown_141f1b310", fn_141f1b310 as *const (), Throws::Discard), // sub_141f1b310
    Replacement::draft_throws(0x141f1b380, "unknown_141f1b380", fn_141f1b380 as *const (), Throws::Discard), // sub_141f1b380
    Replacement::draft_throws(0x141f1b490, "unknown_141f1b490", fn_141f1b490 as *const (), Throws::Discard), // sub_141f1b490
    Replacement::draft_throws(0x141f1b5a0, "unknown_141f1b5a0", fn_141f1b5a0 as *const (), Throws::Discard), // sub_141f1b5a0
    Replacement::draft_throws(0x141f1b630, "unknown_141f1b630", fn_141f1b630 as *const (), Throws::Discard), // sub_141f1b630
    Replacement::draft(0x141f1bbb0, "unknown_141f1bbb0", fn_141f1bbb0 as *const ()), // sub_141f1bbb0
    Replacement::draft(0x141f1bc80, "unknown_141f1bc80", fn_141f1bc80 as *const ()), // sub_141f1bc80
    Replacement::draft(0x141f1bd80, "unknown_141f1bd80", fn_141f1bd80 as *const ()), // sub_141f1bd80
    Replacement::draft(0x141f1bf50, "unknown_141f1bf50", fn_141f1bf50 as *const ()), // sub_141f1bf50
    Replacement::draft(0x141f1c030, "unknown_141f1c030", fn_141f1c030 as *const ()), // sub_141f1c030
    Replacement::draft(0x141f1c200, "unknown_141f1c200", fn_141f1c200 as *const ()), // sub_141f1c200
    Replacement::draft(0x141f1c2d0, "unknown_141f1c2d0", tpl_94790 as *const ()), // sub_141f1c2d0
    Replacement::draft(0x141f1c380, "unknown_141f1c380", tpl_36742::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141f1c380
    Replacement::draft(0x141f1cd50, "unknown_141f1cd50", fn_141f1cd50 as *const ()), // sub_141f1cd50
    Replacement::draft(0x141f1ce40, "unknown_141f1ce40", fn_141f1ce40 as *const ()), // sub_141f1ce40
    Replacement::draft(0x141f1cfc0, "unknown_141f1cfc0", fn_141f1cfc0 as *const ()), // sub_141f1cfc0
    Replacement::draft(0x141f1d110, "unknown_141f1d110", fn_141f1d110 as *const ()), // sub_141f1d110
    Replacement::draft(0x141f1d560, "unknown_141f1d560", fn_141f1d560 as *const ()), // sub_141f1d560
    Replacement::draft(0x141f1d5b0, "unknown_141f1d5b0", fn_141f1d5b0 as *const ()), // sub_141f1d5b0
    Replacement::draft(0x141f1d840, "unknown_141f1d840", fn_141f1d840 as *const ()), // sub_141f1d840
    Replacement::draft_throws(0x141f1da10, "unknown_141f1da10", fn_141f1da10 as *const (), Throws::Discard), // sub_141f1da10
    Replacement::draft(0x141f1eb30, "unknown_141f1eb30", fn_141f1eb30 as *const ()), // sub_141f1eb30
    Replacement::draft(0x141f1f480, "unknown_141f1f480", fn_141f1f480 as *const ()), // sub_141f1f480
    Replacement::draft(0x141f1f890, "unknown_141f1f890", fn_141f1f890 as *const ()), // sub_141f1f890
    Replacement::draft(0x141f1fc10, "unknown_141f1fc10", fn_141f1fc10 as *const ()), // sub_141f1fc10
    Replacement::draft_throws(0x141f1ffc0, "unknown_141f1ffc0", fn_141f1ffc0 as *const (), Throws::Caught), // sub_141f1ffc0
    Replacement::draft(0x141f20870, "unknown_141f20870", fn_141f20870 as *const ()), // sub_141f20870
    Replacement::draft(0x141f20b20, "unknown_141f20b20", fn_141f20b20 as *const ()), // sub_141f20b20
    Replacement::draft(0x141f20c20, "unknown_141f20c20", fn_141f20c20 as *const ()), // sub_141f20c20
    Replacement::draft(0x141f20cb0, "unknown_141f20cb0", fn_141f20cb0 as *const ()), // sub_141f20cb0
    Replacement::draft(0x141f20f40, "unknown_141f20f40", fn_141f20f40 as *const ()), // sub_141f20f40
    Replacement::draft(0x141f21020, "unknown_141f21020", fn_141f21020 as *const ()), // sub_141f21020
    Replacement::draft(0x141f21060, "unknown_141f21060", fn_141f21060 as *const ()), // sub_141f21060
    Replacement::draft(0x141f21320, "unknown_141f21320", fn_141f21320 as *const ()), // sub_141f21320
    Replacement::draft(0x141f21430, "unknown_141f21430", fn_141f21430 as *const ()), // sub_141f21430
    Replacement::draft_throws(0x141f217b0, "unknown_141f217b0", fn_141f217b0 as *const (), Throws::Discard), // sub_141f217b0
    Replacement::draft(0x141f22090, "unknown_141f22090", tpl_37480::<0x20> as *const ()), // sub_141f22090
    Replacement::draft(0x141f22440, "unknown_141f22440", tpl_3280::<0x8, 0x1410810c0> as *const ()), // sub_141f22440
    Replacement::draft(0x141f22480, "unknown_141f22480", fn_141f22480 as *const ()), // sub_141f22480
    Replacement::draft(0x141f22900, "unknown_141f22900", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cd7878, 0x158, 0x14020e8f0> as *const ()), // sub_141f22900
    Replacement::draft(0x141f22a00, "unknown_141f22a00", fn_141f22a00 as *const ()), // sub_141f22a00
    Replacement::draft_throws(0x141f22a80, "unknown_141f22a80", fn_141f22a80 as *const (), Throws::Discard), // sub_141f22a80
    Replacement::draft(0x141f22fd0, "unknown_141f22fd0", tpl_34612 as *const ()), // sub_141f22fd0
    Replacement::draft(0x141f23060, "unknown_141f23060", tpl_34612 as *const ()), // sub_141f23060
    Replacement::draft(0x141f230f0, "unknown_141f230f0", tpl_94428::<0x143288528, 0x8> as *const ()), // sub_141f230f0
    Replacement::draft(0x141f232f0, "unknown_141f232f0", fn_141f232f0 as *const ()), // sub_141f232f0
    Replacement::draft(0x141f233b0, "unknown_141f233b0", fn_141f233b0 as *const ()), // sub_141f233b0
    Replacement::draft(0x141f23420, "unknown_141f23420", fn_141f23420 as *const ()), // sub_141f23420
    Replacement::draft(0x141f234a0, "unknown_141f234a0", fn_141f234a0 as *const ()), // sub_141f234a0
    Replacement::draft_throws(0x141f23500, "unknown_141f23500", tpl_36820::<0x14387e1d4, 0x8, 0x1410835e0, 0x1402116e0, 0x1428e6800, 0x18, 0x1410d6ca0, 0x10, 0x141082800, 0x1428e67f0, 0x141082aa0> as *const (), Throws::Discard), // sub_141f23500
    Replacement::draft(0x141f23660, "unknown_141f23660", fn_141f23660 as *const ()), // sub_141f23660
    Replacement::draft(0x141f23740, "unknown_141f23740", fn_141f23740 as *const ()), // sub_141f23740
    Replacement::draft(0x141f23860, "unknown_141f23860", tpl_94790 as *const ()), // sub_141f23860
    Replacement::draft(0x141f23910, "unknown_141f23910", tpl_36742::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141f23910
    Replacement::draft(0x141f239f0, "unknown_141f239f0", fn_141f239f0 as *const ()), // sub_141f239f0
    Replacement::draft(0x141f23c80, "unknown_141f23c80", tpl_94790 as *const ()), // sub_141f23c80
    Replacement::draft(0x141f23d30, "unknown_141f23d30", tpl_36742::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141f23d30
    Replacement::draft(0x141f23e10, "unknown_141f23e10", tpl_34613 as *const ()), // sub_141f23e10
    Replacement::draft(0x141f23ea0, "unknown_141f23ea0", tpl_34394::<0x10> as *const ()), // sub_141f23ea0
    Replacement::draft(0x141f23f40, "unknown_141f23f40", fn_141f23f40 as *const ()), // sub_141f23f40
    Replacement::draft(0x141f24010, "unknown_141f24010", fn_141f24010 as *const ()), // sub_141f24010
    Replacement::draft_throws(0x141f24070, "unknown_141f24070", fn_141f24070 as *const (), Throws::Discard), // sub_141f24070
    Replacement::draft(0x141f24120, "unknown_141f24120", tpl_34610::<0x20> as *const ()), // sub_141f24120
    Replacement::draft(0x141f241c0, "unknown_141f241c0", fn_141f241c0 as *const ()), // sub_141f241c0
    Replacement::draft(0x141f24400, "unknown_141f24400", tpl_36768 as *const ()), // sub_141f24400
    Replacement::draft(0x141f244b0, "unknown_141f244b0", tpl_34613 as *const ()), // sub_141f244b0
    Replacement::draft(0x141f24540, "unknown_141f24540", tpl_34393 as *const ()), // sub_141f24540
    Replacement::draft(0x141f245e0, "unknown_141f245e0", fn_141f245e0 as *const ()), // sub_141f245e0
    Replacement::draft(0x141f24670, "unknown_141f24670", fn_141f24670 as *const ()), // sub_141f24670
    Replacement::draft_throws(0x141f24af0, "unknown_141f24af0", fn_141f24af0 as *const (), Throws::Discard), // sub_141f24af0
    Replacement::draft_throws(0x141f24b10, "unknown_141f24b10", fn_141f24b10 as *const (), Throws::Discard), // sub_141f24b10
    Replacement::draft(0x141f24ce0, "unknown_141f24ce0", tpl_41844 as *const ()), // sub_141f24ce0
    Replacement::draft(0x141f24d60, "unknown_141f24d60", tpl_41845::<0xc> as *const ()), // sub_141f24d60
    Replacement::draft(0x141f24e10, "unknown_141f24e10", fn_141f24e10 as *const ()), // sub_141f24e10
    Replacement::draft(0x141f24f30, "unknown_141f24f30", tpl_41845::<0x8> as *const ()), // sub_141f24f30
    Replacement::draft(0x141f257b0, "unknown_141f257b0", tpl_41847 as *const ()), // sub_141f257b0
    Replacement::draft(0x141f25830, "unknown_141f25830", fn_141f25830 as *const ()), // sub_141f25830
    Replacement::draft_throws(0x141f25a90, "unknown_141f25a90", fn_141f25a90 as *const (), Throws::Discard), // sub_141f25a90
    Replacement::draft(0x141f25ce0, "unknown_141f25ce0", fn_141f25ce0 as *const ()), // sub_141f25ce0
    Replacement::draft(0x141f263f0, "unknown_141f263f0", fn_141f263f0 as *const ()), // sub_141f263f0
    Replacement::draft(0x141f26470, "unknown_141f26470", fn_141f26470 as *const ()), // sub_141f26470
    Replacement::draft(0x141f26740, "unknown_141f26740", fn_141f26740 as *const ()), // sub_141f26740
    Replacement::draft(0x141f267a0, "unknown_141f267a0", tpl_41088::<0x142407c80, 0x143cd82e0, 0x14020e8f0, 0x143cd7fa0> as *const ()), // sub_141f267a0
    Replacement::draft_throws(0x141f269a0, "unknown_141f269a0", fn_141f269a0 as *const (), Throws::Caught), // sub_141f269a0
    Replacement::draft(0x141f26de0, "unknown_141f26de0", fn_141f26de0 as *const ()), // sub_141f26de0
    Replacement::draft(0x141f283a0, "unknown_141f283a0", fn_141f283a0 as *const ()), // sub_141f283a0
    Replacement::draft(0x141f28870, "unknown_141f28870", fn_141f28870 as *const ()), // sub_141f28870
    Replacement::draft_throws(0x141f28940, "unknown_141f28940", fn_141f28940 as *const (), Throws::Discard), // sub_141f28940
    Replacement::draft(0x141f289d0, "unknown_141f289d0", fn_141f289d0 as *const ()), // sub_141f289d0
    Replacement::draft(0x141f28cc0, "unknown_141f28cc0", tpl_41088::<0x142407c80, 0x143cd8a20, 0x14020e8f0, 0x143cd8cc8> as *const ()), // sub_141f28cc0
    Replacement::draft_throws(0x141f28ec0, "unknown_141f28ec0", fn_141f28ec0 as *const (), Throws::Caught), // sub_141f28ec0
    Replacement::draft(0x141f297f0, "unknown_141f297f0", fn_141f297f0 as *const ()), // sub_141f297f0
    Replacement::draft(0x141f29840, "unknown_141f29840", fn_141f29840 as *const ()), // sub_141f29840
    Replacement::draft_throws(0x141f29b20, "unknown_141f29b20", fn_141f29b20 as *const (), Throws::Discard), // sub_141f29b20
    Replacement::draft(0x141f2a880, "unknown_141f2a880", fn_141f2a880 as *const ()), // sub_141f2a880
    Replacement::draft(0x141f2ad50, "unknown_141f2ad50", fn_141f2ad50 as *const ()), // sub_141f2ad50
    Replacement::draft(0x141f2b040, "unknown_141f2b040", tpl_41088::<0x142407c80, 0x143cd852c, 0x14020e8f0, 0x143cd8490> as *const ()), // sub_141f2b040
    Replacement::draft_throws(0x141f2b240, "unknown_141f2b240", fn_141f2b240 as *const (), Throws::Caught), // sub_141f2b240
    Replacement::draft(0x141f2b8f0, "unknown_141f2b8f0", tpl_20414::<0x8, 0x14108b060, 0x14108b260> as *const ()), // sub_141f2b8f0
    Replacement::draft(0x141f2bba0, "unknown_141f2bba0", fn_141f2bba0 as *const ()), // sub_141f2bba0
    Replacement::draft(0x141f2c190, "unknown_141f2c190", tpl_41861::<0x10, 0x14> as *const ()), // sub_141f2c190
    Replacement::draft(0x141f2c200, "unknown_141f2c200", fn_141f2c200 as *const ()), // sub_141f2c200
    Replacement::draft(0x141f2c3c0, "unknown_141f2c3c0", fn_141f2c3c0 as *const ()), // sub_141f2c3c0
    Replacement::draft(0x141f2c430, "unknown_141f2c430", fn_141f2c430 as *const ()), // sub_141f2c430
    Replacement::draft(0x141f2c470, "unknown_141f2c470", fn_141f2c470 as *const ()), // sub_141f2c470
    Replacement::draft(0x141f2c4c0, "unknown_141f2c4c0", tpl_41864 as *const ()), // sub_141f2c4c0
    Replacement::draft_throws(0x141f2c4f0, "unknown_141f2c4f0", fn_141f2c4f0 as *const (), Throws::Discard), // sub_141f2c4f0
    Replacement::draft_throws(0x141f2d430, "unknown_141f2d430", fn_141f2d430 as *const (), Throws::Discard), // sub_141f2d430
    Replacement::draft(0x141f2d4f0, "unknown_141f2d4f0", fn_141f2d4f0 as *const ()), // sub_141f2d4f0
    Replacement::draft(0x141f2d510, "unknown_141f2d510", fn_141f2d510 as *const ()), // sub_141f2d510
    Replacement::draft(0x141f2d540, "unknown_141f2d540", fn_141f2d540 as *const ()), // sub_141f2d540
    Replacement::draft(0x141f2dbd0, "unknown_141f2dbd0", fn_141f2dbd0 as *const ()), // sub_141f2dbd0
    Replacement::draft(0x141f2df70, "unknown_141f2df70", fn_141f2df70 as *const ()), // sub_141f2df70
    Replacement::draft(0x141f2e090, "unknown_141f2e090", fn_141f2e090 as *const ()), // sub_141f2e090
    Replacement::draft(0x141f2e100, "unknown_141f2e100", fn_141f2e100 as *const ()), // sub_141f2e100
    Replacement::draft(0x141f2e1f0, "unknown_141f2e1f0", fn_141f2e1f0 as *const ()), // sub_141f2e1f0
    Replacement::draft(0x141f2e360, "unknown_141f2e360", fn_141f2e360 as *const ()), // sub_141f2e360
    Replacement::draft(0x141f2e5a0, "unknown_141f2e5a0", fn_141f2e5a0 as *const ()), // sub_141f2e5a0
    Replacement::draft(0x141f2e5d0, "unknown_141f2e5d0", fn_141f2e5d0 as *const ()), // sub_141f2e5d0
    Replacement::draft(0x141f2e610, "unknown_141f2e610", fn_141f2e610 as *const ()), // sub_141f2e610
    Replacement::draft(0x141f2e630, "unknown_141f2e630", fn_141f2e630 as *const ()), // sub_141f2e630
    Replacement::draft(0x141f2e760, "unknown_141f2e760", fn_141f2e760 as *const ()), // sub_141f2e760
    Replacement::draft(0x141f2eb70, "unknown_141f2eb70", fn_141f2eb70 as *const ()), // sub_141f2eb70
    Replacement::draft(0x141f2ec50, "unknown_141f2ec50", fn_141f2ec50 as *const ()), // sub_141f2ec50
    Replacement::draft(0x141f2f0f0, "unknown_141f2f0f0", tpl_1439::<0x8, 0x14109d3d0> as *const ()), // sub_141f2f0f0
    Replacement::draft(0x141f2f130, "unknown_141f2f130", tpl_5094::<0x8, 0x14109cf00, 0x14109d100> as *const ()), // sub_141f2f130
    Replacement::draft(0x141f2f1a0, "unknown_141f2f1a0", fn_141f2f1a0 as *const ()), // sub_141f2f1a0
    Replacement::draft(0x141f2f210, "unknown_141f2f210", fn_141f2f210 as *const ()), // sub_141f2f210
    Replacement::draft(0x141f2f2a0, "unknown_141f2f2a0", fn_141f2f2a0 as *const ()), // sub_141f2f2a0
    Replacement::draft(0x141f2f580, "unknown_141f2f580", fn_141f2f580 as *const ()), // sub_141f2f580
    Replacement::draft(0x141f2f870, "unknown_141f2f870", fn_141f2f870 as *const ()), // sub_141f2f870
    Replacement::draft(0x141f2fb80, "unknown_141f2fb80", fn_141f2fb80 as *const ()), // sub_141f2fb80
    Replacement::draft(0x141f2fe80, "unknown_141f2fe80", fn_141f2fe80 as *const ()), // sub_141f2fe80
    Replacement::draft(0x141f301b0, "unknown_141f301b0", fn_141f301b0 as *const ()), // sub_141f301b0
    Replacement::draft(0x141f30500, "unknown_141f30500", fn_141f30500 as *const ()), // sub_141f30500
    Replacement::draft_throws(0x141f30540, "unknown_141f30540", fn_141f30540 as *const (), Throws::Discard), // sub_141f30540
    Replacement::draft(0x141f30860, "unknown_141f30860", fn_141f30860 as *const ()), // sub_141f30860
    Replacement::draft_throws(0x141f30880, "unknown_141f30880", fn_141f30880 as *const (), Throws::Discard), // sub_141f30880
    Replacement::draft(0x141f30900, "unknown_141f30900", fn_141f30900 as *const ()), // sub_141f30900
    Replacement::draft(0x141f30920, "unknown_141f30920", fn_141f30920 as *const ()), // sub_141f30920
    Replacement::draft(0x141f30940, "unknown_141f30940", fn_141f30940 as *const ()), // sub_141f30940
    Replacement::draft(0x141f31140, "unknown_141f31140", fn_141f31140 as *const ()), // sub_141f31140
    Replacement::draft(0x141f31280, "unknown_141f31280", fn_141f31280 as *const ()), // sub_141f31280
    Replacement::draft(0x141f313d0, "unknown_141f313d0", fn_141f313d0 as *const ()), // sub_141f313d0
    Replacement::draft(0x141f31520, "unknown_141f31520", tpl_35056::<0x1410a0350> as *const ()), // sub_141f31520
    Replacement::draft(0x141f31540, "unknown_141f31540", fn_141f31540 as *const ()), // sub_141f31540
    Replacement::draft(0x141f315f0, "unknown_141f315f0", fn_141f315f0 as *const ()), // sub_141f315f0
    Replacement::draft(0x141f31700, "unknown_141f31700", fn_141f31700 as *const ()), // sub_141f31700
    Replacement::draft(0x141f317e0, "unknown_141f317e0", tpl_94441::<0x8> as *const ()), // sub_141f317e0
    Replacement::draft(0x141f318b0, "unknown_141f318b0", tpl_34684::<0x1410a0480> as *const ()), // sub_141f318b0
    Replacement::draft(0x141f31f00, "unknown_141f31f00", fn_141f31f00 as *const ()), // sub_141f31f00
    Replacement::draft(0x141f32030, "unknown_141f32030", fn_141f32030 as *const ()), // sub_141f32030
    Replacement::draft(0x141f32540, "unknown_141f32540", fn_141f32540 as *const ()), // sub_141f32540
    Replacement::draft(0x141f32600, "unknown_141f32600", fn_141f32600 as *const ()), // sub_141f32600
    Replacement::draft(0x141f328f0, "unknown_141f328f0", fn_141f328f0 as *const ()), // sub_141f328f0
    Replacement::draft_throws(0x141f32be0, "unknown_141f32be0", fn_141f32be0 as *const (), Throws::Caught), // sub_141f32be0
    Replacement::draft(0x141f33510, "unknown_141f33510", fn_141f33510 as *const ()), // sub_141f33510
    Replacement::draft_throws(0x141f33570, "unknown_141f33570", fn_141f33570 as *const (), Throws::Discard), // sub_141f33570
    Replacement::draft(0x141f33b90, "unknown_141f33b90", fn_141f33b90 as *const ()), // sub_141f33b90
    Replacement::draft(0x141f33bf0, "unknown_141f33bf0", fn_141f33bf0 as *const ()), // sub_141f33bf0
    Replacement::draft(0x141f33f60, "unknown_141f33f60", fn_141f33f60 as *const ()), // sub_141f33f60
    Replacement::draft(0x141f34160, "unknown_141f34160", fn_141f34160 as *const ()), // sub_141f34160
    Replacement::draft(0x141f34250, "unknown_141f34250", fn_141f34250 as *const ()), // sub_141f34250
    Replacement::draft(0x141f34300, "unknown_141f34300", fn_141f34300 as *const ()), // sub_141f34300
    Replacement::draft(0x141f343b0, "unknown_141f343b0", tpl_30667::<0x2ec> as *const ()), // sub_141f343b0
    Replacement::draft(0x141f343c0, "unknown_141f343c0", fn_141f343c0 as *const ()), // sub_141f343c0
    Replacement::draft(0x141f346a0, "unknown_141f346a0", tpl_35758::<0x8, 0x1410a7590> as *const ()), // sub_141f346a0
    Replacement::draft(0x141f34750, "unknown_141f34750", tpl_35759::<0x8, 0x1410a7590> as *const ()), // sub_141f34750
    Replacement::draft(0x141f34780, "unknown_141f34780", tpl_34684::<0x1410a74e0> as *const ()), // sub_141f34780
    Replacement::draft(0x141f347c0, "unknown_141f347c0", tpl_41907::<0x14238d770, 0x1434268d0, 0x143426f34, 0x14242ac10, 0x143c8c44c, 0x143c8c444, 0x143c8c440, 0x143384d10, 0x1423fe030, 0x142409b50> as *const ()), // sub_141f347c0
    Replacement::draft(0x141f34990, "unknown_141f34990", fn_141f34990 as *const ()), // sub_141f34990
    Replacement::draft(0x141f35480, "unknown_141f35480", fn_141f35480 as *const ()), // sub_141f35480
    Replacement::draft(0x141f35780, "unknown_141f35780", fn_141f35780 as *const ()), // sub_141f35780
    Replacement::draft(0x141f36bf0, "unknown_141f36bf0", tpl_41910::<0x143a341c8> as *const ()), // sub_141f36bf0
    Replacement::draft(0x141f36c20, "unknown_141f36c20", tpl_366::<0x143a341c8> as *const ()), // sub_141f36c20
    Replacement::draft(0x141f36c30, "unknown_141f36c30", tpl_23860::<0x2100, 0x20e0> as *const ()), // sub_141f36c30
    Replacement::draft(0x141f36c50, "unknown_141f36c50", fn_141f36c50 as *const ()), // sub_141f36c50
    Replacement::draft(0x141f370d0, "unknown_141f370d0", tpl_41907::<0x14238d770, 0x1434268d0, 0x143426f34, 0x14242ac10, 0x143c8c44c, 0x143c8c444, 0x143c8c440, 0x143384d10, 0x1423fe030, 0x142409b50> as *const ()), // sub_141f370d0
    Replacement::draft(0x141f37350, "unknown_141f37350", tpl_41912::<0x8> as *const ()), // sub_141f37350
    Replacement::draft(0x141f37370, "unknown_141f37370", fn_141f37370 as *const ()), // sub_141f37370
    Replacement::draft(0x141f373c0, "unknown_141f373c0", fn_141f373c0 as *const ()), // sub_141f373c0
    Replacement::draft(0x141f37430, "unknown_141f37430", fn_141f37430 as *const ()), // sub_141f37430
    Replacement::draft_throws(0x141f37480, "unknown_141f37480", fn_141f37480 as *const (), Throws::Caught), // sub_141f37480
    Replacement::draft_throws(0x141f37860, "unknown_141f37860", fn_141f37860 as *const (), Throws::Discard), // sub_141f37860
    Replacement::draft_throws(0x141f37bb0, "unknown_141f37bb0", fn_141f37bb0 as *const (), Throws::Discard), // sub_141f37bb0
    Replacement::draft_throws(0x141f37f10, "unknown_141f37f10", fn_141f37f10 as *const (), Throws::Discard), // sub_141f37f10
    Replacement::draft(0x141f38160, "unknown_141f38160", fn_141f38160 as *const ()), // sub_141f38160
    Replacement::draft(0x141f38190, "unknown_141f38190", fn_141f38190 as *const ()), // sub_141f38190
    Replacement::draft_throws(0x141f381f0, "unknown_141f381f0", fn_141f381f0 as *const (), Throws::Caught), // sub_141f381f0
    Replacement::draft_throws(0x141f38840, "unknown_141f38840", fn_141f38840 as *const (), Throws::Caught), // sub_141f38840
    Replacement::draft(0x141f388f0, "unknown_141f388f0", fn_141f388f0 as *const ()), // sub_141f388f0
    Replacement::draft(0x141f38910, "unknown_141f38910", fn_141f38910 as *const ()), // sub_141f38910
    Replacement::draft_throws(0x141f38930, "unknown_141f38930", fn_141f38930 as *const (), Throws::Discard), // sub_141f38930
    Replacement::draft(0x141f392b0, "unknown_141f392b0", fn_141f392b0 as *const ()), // sub_141f392b0
    Replacement::draft(0x141f39490, "unknown_141f39490", tpl_36768 as *const ()), // sub_141f39490
    Replacement::draft(0x141f39540, "unknown_141f39540", fn_141f39540 as *const ()), // sub_141f39540
    Replacement::draft(0x141f396b0, "unknown_141f396b0", fn_141f396b0 as *const ()), // sub_141f396b0
    Replacement::draft(0x141f39750, "unknown_141f39750", fn_141f39750 as *const ()), // sub_141f39750
    Replacement::draft(0x141f39890, "unknown_141f39890", tpl_41928::<0x10, 0x18, 0x38> as *const ()), // sub_141f39890
    Replacement::draft(0x141f39930, "unknown_141f39930", fn_141f39930 as *const ()), // sub_141f39930
    Replacement::draft(0x141f39a80, "unknown_141f39a80", tpl_41928::<0x10, 0x18, 0x40> as *const ()), // sub_141f39a80
    Replacement::draft(0x141f39b20, "unknown_141f39b20", tpl_35099::<0x20, 0x40> as *const ()), // sub_141f39b20
    Replacement::draft(0x141f3a0f0, "unknown_141f3a0f0", fn_141f3a0f0 as *const ()), // sub_141f3a0f0
    Replacement::draft(0x141f3a3c0, "unknown_141f3a3c0", fn_141f3a3c0 as *const ()), // sub_141f3a3c0
    Replacement::draft(0x141f3a6f0, "unknown_141f3a6f0", fn_141f3a6f0 as *const ()), // sub_141f3a6f0
    Replacement::draft(0x141f3a7e0, "unknown_141f3a7e0", fn_141f3a7e0 as *const ()), // sub_141f3a7e0
    Replacement::draft(0x141f3ae20, "unknown_141f3ae20", fn_141f3ae20 as *const ()), // sub_141f3ae20
    Replacement::draft(0x141f3afe0, "unknown_141f3afe0", fn_141f3afe0 as *const ()), // sub_141f3afe0
    Replacement::draft(0x141f3b2f0, "unknown_141f3b2f0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cda2d4, 0x158, 0x14020e8f0> as *const ()), // sub_141f3b2f0
    Replacement::draft(0x141f3b3f0, "unknown_141f3b3f0", fn_141f3b3f0 as *const ()), // sub_141f3b3f0
    Replacement::draft(0x141f3b610, "unknown_141f3b610", fn_141f3b610 as *const ()), // sub_141f3b610
    Replacement::draft(0x141f3ba90, "unknown_141f3ba90", fn_141f3ba90 as *const ()), // sub_141f3ba90
    Replacement::draft(0x141f3bff0, "unknown_141f3bff0", fn_141f3bff0 as *const ()), // sub_141f3bff0
    Replacement::draft(0x141f3c2e0, "unknown_141f3c2e0", fn_141f3c2e0 as *const ()), // sub_141f3c2e0
    Replacement::draft(0x141f3c2f0, "unknown_141f3c2f0", fn_141f3c2f0 as *const ()), // sub_141f3c2f0
    Replacement::draft(0x141f3c300, "unknown_141f3c300", fn_141f3c300 as *const ()), // sub_141f3c300
    Replacement::draft_throws(0x141f3c530, "unknown_141f3c530", fn_141f3c530 as *const (), Throws::Discard), // sub_141f3c530
    Replacement::draft(0x141f3ce20, "unknown_141f3ce20", fn_141f3ce20 as *const ()), // sub_141f3ce20
    Replacement::draft(0x141f3cf60, "unknown_141f3cf60", fn_141f3cf60 as *const ()), // sub_141f3cf60
    Replacement::draft_throws(0x141f3d110, "unknown_141f3d110", fn_141f3d110 as *const (), Throws::Discard), // sub_141f3d110
    Replacement::draft(0x141f3d6f0, "unknown_141f3d6f0", fn_141f3d6f0 as *const ()), // sub_141f3d6f0
    Replacement::draft(0x141f3d900, "unknown_141f3d900", fn_141f3d900 as *const ()), // sub_141f3d900
    Replacement::draft(0x141f3d9e0, "unknown_141f3d9e0", fn_141f3d9e0 as *const ()), // sub_141f3d9e0
    Replacement::draft(0x141f3da40, "unknown_141f3da40", tpl_37447 as *const ()), // sub_141f3da40
    Replacement::draft(0x141f3da80, "unknown_141f3da80", tpl_34994::<0x143cdae54, 0x143880e68, 0x143880e60, 0x141834a78, 0x142612f50, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f3da80
    Replacement::draft(0x141f3daf0, "unknown_141f3daf0", fn_141f3daf0 as *const ()), // sub_141f3daf0
    Replacement::draft(0x141f3dba0, "unknown_141f3dba0", fn_141f3dba0 as *const ()), // sub_141f3dba0
    Replacement::draft(0x141f3dbd0, "unknown_141f3dbd0", fn_141f3dbd0 as *const ()), // sub_141f3dbd0
    Replacement::draft(0x141f3dc40, "unknown_141f3dc40", tpl_34994::<0x143cdac54, 0x143880e78, 0x143880e70, 0x141834a78, 0x142612f60, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f3dc40
    Replacement::draft(0x141f3dcb0, "unknown_141f3dcb0", tpl_35323 as *const ()), // sub_141f3dcb0
    Replacement::draft(0x141f3dcd0, "unknown_141f3dcd0", tpl_34318 as *const ()), // sub_141f3dcd0
    Replacement::draft(0x141f3dcf0, "unknown_141f3dcf0", tpl_34319::<0x143cdac50, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3dcf0
    Replacement::draft(0x141f3dd50, "unknown_141f3dd50", tpl_34318 as *const ()), // sub_141f3dd50
    Replacement::draft(0x141f3dd70, "unknown_141f3dd70", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3dd70
    Replacement::draft(0x141f3ddd0, "unknown_141f3ddd0", tpl_34318 as *const ()), // sub_141f3ddd0
    Replacement::draft(0x141f3ddf0, "unknown_141f3ddf0", tpl_34319::<0x143cdad10, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3ddf0
    Replacement::draft(0x141f3de50, "unknown_141f3de50", tpl_34318 as *const ()), // sub_141f3de50
    Replacement::draft(0x141f3de70, "unknown_141f3de70", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3de70
    Replacement::draft(0x141f3ded0, "unknown_141f3ded0", tpl_34318 as *const ()), // sub_141f3ded0
    Replacement::draft(0x141f3def0, "unknown_141f3def0", tpl_34319::<0x143cdac58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3def0
    Replacement::draft(0x141f3df50, "unknown_141f3df50", tpl_34318 as *const ()), // sub_141f3df50
    Replacement::draft(0x141f3df70, "unknown_141f3df70", tpl_34318 as *const ()), // sub_141f3df70
    Replacement::draft(0x141f3df90, "unknown_141f3df90", tpl_34319::<0x143cdb0a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3df90
    Replacement::draft(0x141f3dff0, "unknown_141f3dff0", tpl_34319::<0x143c9a620, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f3dff0
    Replacement::draft(0x141f3e050, "unknown_141f3e050", tpl_361 as *const ()), // sub_141f3e050
    Replacement::draft(0x141f3e060, "unknown_141f3e060", tpl_20527::<0x58, 0x8, 0x1410b5cb0> as *const ()), // sub_141f3e060
    Replacement::draft(0x141f3e0b0, "unknown_141f3e0b0", tpl_1439::<0x8, 0x1410b5ab0> as *const ()), // sub_141f3e0b0
    Replacement::draft_throws(0x141f3e0f0, "unknown_141f3e0f0", fn_141f3e0f0 as *const (), Throws::Discard), // sub_141f3e0f0
    Replacement::draft_throws(0x141f3e170, "unknown_141f3e170", fn_141f3e170 as *const (), Throws::Discard), // sub_141f3e170
    Replacement::draft_throws(0x141f3e1f0, "unknown_141f3e1f0", fn_141f3e1f0 as *const (), Throws::Discard), // sub_141f3e1f0
    Replacement::draft(0x141f3e3b0, "unknown_141f3e3b0", tpl_361 as *const ()), // sub_141f3e3b0
    Replacement::draft(0x141f3e3c0, "unknown_141f3e3c0", fn_141f3e3c0 as *const ()), // sub_141f3e3c0
    Replacement::draft(0x141f3e420, "unknown_141f3e420", tpl_361 as *const ()), // sub_141f3e420
    Replacement::draft(0x141f3e430, "unknown_141f3e430", tpl_35031::<0x142407c80, 0x143cdac50, 0x14020e8f0> as *const ()), // sub_141f3e430
    Replacement::draft(0x141f3e550, "unknown_141f3e550", tpl_496 as *const ()), // sub_141f3e550
    Replacement::draft_throws(0x141f3e560, "unknown_141f3e560", fn_141f3e560 as *const (), Throws::Caught), // sub_141f3e560
    Replacement::draft(0x141f3e960, "unknown_141f3e960", tpl_361 as *const ()), // sub_141f3e960
    Replacement::draft_throws(0x141f3e970, "unknown_141f3e970", fn_141f3e970 as *const (), Throws::Discard), // sub_141f3e970
    Replacement::draft(0x141f3ec60, "unknown_141f3ec60", tpl_361 as *const ()), // sub_141f3ec60
    Replacement::draft(0x141f3ec70, "unknown_141f3ec70", fn_141f3ec70 as *const ()), // sub_141f3ec70
    Replacement::draft(0x141f3ecf0, "unknown_141f3ecf0", tpl_361 as *const ()), // sub_141f3ecf0
    Replacement::draft(0x141f3ed00, "unknown_141f3ed00", fn_141f3ed00 as *const ()), // sub_141f3ed00
    Replacement::draft(0x141f3ee70, "unknown_141f3ee70", tpl_361 as *const ()), // sub_141f3ee70
    Replacement::draft_throws(0x141f3ee80, "unknown_141f3ee80", fn_141f3ee80 as *const (), Throws::Discard), // sub_141f3ee80
    Replacement::draft_throws(0x141f3f540, "unknown_141f3f540", fn_141f3f540 as *const (), Throws::Discard), // sub_141f3f540
    Replacement::draft_throws(0x141f3f750, "unknown_141f3f750", fn_141f3f750 as *const (), Throws::Discard), // sub_141f3f750
    Replacement::draft_throws(0x141f3f880, "unknown_141f3f880", fn_141f3f880 as *const (), Throws::Discard), // sub_141f3f880
    Replacement::draft(0x141f3f980, "unknown_141f3f980", fn_141f3f980 as *const ()), // sub_141f3f980
    Replacement::draft(0x141f3fa00, "unknown_141f3fa00", fn_141f3fa00 as *const ()), // sub_141f3fa00
    Replacement::draft_throws(0x141f3fa90, "unknown_141f3fa90", fn_141f3fa90 as *const (), Throws::Discard), // sub_141f3fa90
    Replacement::draft(0x141f3fca0, "unknown_141f3fca0", fn_141f3fca0 as *const ()), // sub_141f3fca0
    Replacement::draft(0x141f3fd40, "unknown_141f3fd40", tpl_34443 as *const ()), // sub_141f3fd40
    Replacement::draft(0x141f3fd50, "unknown_141f3fd50", fn_141f3fd50 as *const ()), // sub_141f3fd50
    Replacement::draft(0x141f40010, "unknown_141f40010", fn_141f40010 as *const ()), // sub_141f40010
    Replacement::draft(0x141f40050, "unknown_141f40050", fn_141f40050 as *const ()), // sub_141f40050
    Replacement::draft(0x141f400e0, "unknown_141f400e0", fn_141f400e0 as *const ()), // sub_141f400e0
    Replacement::draft(0x141f401a0, "unknown_141f401a0", fn_141f401a0 as *const ()), // sub_141f401a0
    Replacement::draft(0x141f40220, "unknown_141f40220", tpl_41965::<0x8> as *const ()), // sub_141f40220
    Replacement::draft(0x141f40250, "unknown_141f40250", tpl_41965::<0x4> as *const ()), // sub_141f40250
    Replacement::draft(0x141f40280, "unknown_141f40280", fn_141f40280 as *const ()), // sub_141f40280
    Replacement::draft(0x141f402b0, "unknown_141f402b0", fn_141f402b0 as *const ()), // sub_141f402b0
    Replacement::draft(0x141f402f0, "unknown_141f402f0", fn_141f402f0 as *const ()), // sub_141f402f0
    Replacement::draft(0x141f40310, "unknown_141f40310", tpl_22904::<0x60> as *const ()), // sub_141f40310
    Replacement::draft(0x141f40330, "unknown_141f40330", fn_141f40330 as *const ()), // sub_141f40330
    Replacement::draft(0x141f405a0, "unknown_141f405a0", fn_141f405a0 as *const ()), // sub_141f405a0
    Replacement::draft(0x141f405e0, "unknown_141f405e0", fn_141f405e0 as *const ()), // sub_141f405e0
    Replacement::draft(0x141f40890, "unknown_141f40890", fn_141f40890 as *const ()), // sub_141f40890
    Replacement::draft(0x141f40a80, "unknown_141f40a80", tpl_40837::<0x80, 0x1410bbc80, 0x1000, 0x8> as *const ()), // sub_141f40a80
    Replacement::draft(0x141f40ad0, "unknown_141f40ad0", fn_141f40ad0 as *const ()), // sub_141f40ad0
    Replacement::draft(0x141f40db0, "unknown_141f40db0", fn_141f40db0 as *const ()), // sub_141f40db0
    Replacement::draft(0x141f40f20, "unknown_141f40f20", tpl_34318 as *const ()), // sub_141f40f20
    Replacement::draft(0x141f40f40, "unknown_141f40f40", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f40f40
    Replacement::draft(0x141f40fa0, "unknown_141f40fa0", fn_141f40fa0 as *const ()), // sub_141f40fa0
    Replacement::draft_throws(0x141f410b0, "unknown_141f410b0", fn_141f410b0 as *const (), Throws::Discard), // sub_141f410b0
    Replacement::draft(0x141f41130, "unknown_141f41130", fn_141f41130 as *const ()), // sub_141f41130
    Replacement::draft_throws(0x141f41200, "sparsehash_141f41200", fn_141f41200 as *const (), Throws::Discard), // sub_141f41200
    Replacement::draft(0x141f412d0, "sparsehash_141f412d0", tpl_361 as *const ()), // sub_141f412d0
    Replacement::draft_throws(0x141f412e0, "sparsehash_141f412e0", fn_141f412e0 as *const (), Throws::Caught), // sub_141f412e0
    Replacement::draft_throws(0x141f415e0, "unknown_141f415e0", fn_141f415e0 as *const (), Throws::Caught), // sub_141f415e0
    Replacement::draft_throws(0x141f41800, "unknown_141f41800", fn_141f41800 as *const (), Throws::Caught), // sub_141f41800
    Replacement::draft(0x141f41ca0, "unknown_141f41ca0", tpl_34994::<0x143cdb828, 0x143880ec8, 0x143880ec0, 0x141834a78, 0x142613080, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f41ca0
    Replacement::draft(0x141f41d10, "unknown_141f41d10", tpl_95295::<0x143285118, 0x8, 0x10, 0x14> as *const ()), // sub_141f41d10
    Replacement::draft(0x141f41ef0, "unknown_141f41ef0", fn_141f41ef0 as *const ()), // sub_141f41ef0
    Replacement::draft(0x141f41fe0, "unknown_141f41fe0", fn_141f41fe0 as *const ()), // sub_141f41fe0
    Replacement::draft(0x141f420f0, "unknown_141f420f0", tpl_94441::<0x8> as *const ()), // sub_141f420f0
    Replacement::draft(0x141f421c0, "unknown_141f421c0", tpl_34684::<0x1410bdec0> as *const ()), // sub_141f421c0
    Replacement::draft(0x141f42270, "unknown_141f42270", fn_141f42270 as *const ()), // sub_141f42270
    Replacement::draft(0x141f42440, "unknown_141f42440", fn_141f42440 as *const ()), // sub_141f42440
    Replacement::draft(0x141f42cd0, "unknown_141f42cd0", tpl_20568::<0x10, 0x8, 0x1410c15a0> as *const ()), // sub_141f42cd0
    Replacement::draft(0x141f42d20, "unknown_141f42d20", fn_141f42d20 as *const ()), // sub_141f42d20
    Replacement::draft(0x141f42e80, "unknown_141f42e80", fn_141f42e80 as *const ()), // sub_141f42e80
    Replacement::draft_throws(0x141f42f30, "unknown_141f42f30", fn_141f42f30 as *const (), Throws::Caught), // sub_141f42f30
    Replacement::draft_throws(0x141f43280, "unknown_141f43280", fn_141f43280 as *const (), Throws::Discard), // sub_141f43280
    Replacement::draft(0x141f433d0, "unknown_141f433d0", fn_141f433d0 as *const ()), // sub_141f433d0
    Replacement::draft(0x141f43800, "unknown_141f43800", fn_141f43800 as *const ()), // sub_141f43800
    Replacement::draft(0x141f43f40, "unknown_141f43f40", fn_141f43f40 as *const ()), // sub_141f43f40
    Replacement::draft(0x141f443a0, "unknown_141f443a0", fn_141f443a0 as *const ()), // sub_141f443a0
    Replacement::draft(0x141f44740, "unknown_141f44740", fn_141f44740 as *const ()), // sub_141f44740
    Replacement::draft(0x141f44770, "unknown_141f44770", fn_141f44770 as *const ()), // sub_141f44770
    Replacement::draft_throws(0x141f447f0, "unknown_141f447f0", fn_141f447f0 as *const (), Throws::Discard), // sub_141f447f0
    Replacement::draft(0x141f45430, "unknown_141f45430", fn_141f45430 as *const ()), // sub_141f45430
    Replacement::draft(0x141f454a0, "unknown_141f454a0", fn_141f454a0 as *const ()), // sub_141f454a0
    Replacement::draft_throws(0x141f456d0, "unknown_141f456d0", fn_141f456d0 as *const (), Throws::Caught), // sub_141f456d0
    Replacement::draft_throws(0x141f46090, "unknown_141f46090", fn_141f46090 as *const (), Throws::Discard), // sub_141f46090
    Replacement::draft_throws(0x141f46190, "unknown_141f46190", fn_141f46190 as *const (), Throws::Caught), // sub_141f46190
    Replacement::draft_throws(0x141f46410, "unknown_141f46410", fn_141f46410 as *const (), Throws::Caught), // sub_141f46410
    Replacement::draft(0x141f46b50, "unknown_141f46b50", fn_141f46b50 as *const ()), // sub_141f46b50
    Replacement::draft(0x141f46d20, "unknown_141f46d20", fn_141f46d20 as *const ()), // sub_141f46d20
    Replacement::draft(0x141f47010, "unknown_141f47010", fn_141f47010 as *const ()), // sub_141f47010
    Replacement::draft(0x141f470f0, "unknown_141f470f0", tpl_34252 as *const ()), // sub_141f470f0
    Replacement::draft_throws(0x141f47100, "unknown_141f47100", fn_141f47100 as *const (), Throws::Caught), // sub_141f47100
    Replacement::draft_throws(0x141f47120, "unknown_141f47120", fn_141f47120 as *const (), Throws::Discard), // sub_141f47120
    Replacement::draft_throws(0x141f47690, "unknown_141f47690", fn_141f47690 as *const (), Throws::Discard), // sub_141f47690
    Replacement::draft_throws(0x141f47850, "unknown_141f47850", fn_141f47850 as *const (), Throws::Discard), // sub_141f47850
    Replacement::draft_throws(0x141f47c00, "unknown_141f47c00", fn_141f47c00 as *const (), Throws::Discard), // sub_141f47c00
    Replacement::draft_throws(0x141f482c0, "unknown_141f482c0", fn_141f482c0 as *const (), Throws::Discard), // sub_141f482c0
    Replacement::draft_throws(0x141f48570, "unknown_141f48570", fn_141f48570 as *const (), Throws::Caught), // sub_141f48570
    Replacement::draft_throws(0x141f48870, "unknown_141f48870", fn_141f48870 as *const (), Throws::Caught), // sub_141f48870
    Replacement::draft_throws(0x141f48a70, "unknown_141f48a70", fn_141f48a70 as *const (), Throws::Discard), // sub_141f48a70
    Replacement::draft_throws(0x141f48c40, "unknown_141f48c40", fn_141f48c40 as *const (), Throws::Caught), // sub_141f48c40
    Replacement::draft_throws(0x141f492b0, "unknown_141f492b0", fn_141f492b0 as *const (), Throws::Discard), // sub_141f492b0
    Replacement::draft_throws(0x141f49970, "unknown_141f49970", fn_141f49970 as *const (), Throws::Caught), // sub_141f49970
    Replacement::draft_throws(0x141f49c50, "unknown_141f49c50", fn_141f49c50 as *const (), Throws::Discard), // sub_141f49c50
    Replacement::draft_throws(0x141f49ef0, "unknown_141f49ef0", fn_141f49ef0 as *const (), Throws::Discard), // sub_141f49ef0
    Replacement::draft_throws(0x141f4a0e0, "unknown_141f4a0e0", fn_141f4a0e0 as *const (), Throws::Caught), // sub_141f4a0e0
    Replacement::draft_throws(0x141f4a360, "unknown_141f4a360", fn_141f4a360 as *const (), Throws::Discard), // sub_141f4a360
    Replacement::draft_throws(0x141f4a4b0, "unknown_141f4a4b0", fn_141f4a4b0 as *const (), Throws::Discard), // sub_141f4a4b0
    Replacement::draft_throws(0x141f4a6e0, "unknown_141f4a6e0", fn_141f4a6e0 as *const (), Throws::Discard), // sub_141f4a6e0
    Replacement::draft_throws(0x141f4a8a0, "physx_141f4a8a0", fn_141f4a8a0 as *const (), Throws::Caught), // sub_141f4a8a0
    Replacement::draft_throws(0x141f4aa00, "physx_141f4aa00", fn_141f4aa00 as *const (), Throws::Discard), // sub_141f4aa00
    Replacement::draft_throws(0x141f4aaf0, "physx_141f4aaf0", fn_141f4aaf0 as *const (), Throws::Caught), // sub_141f4aaf0
    Replacement::draft_throws(0x141f4ad10, "physx_141f4ad10", fn_141f4ad10 as *const (), Throws::Discard), // sub_141f4ad10
    Replacement::draft_throws(0x141f4ae50, "physx_141f4ae50", fn_141f4ae50 as *const (), Throws::Caught), // sub_141f4ae50
    Replacement::draft_throws(0x141f4b450, "physx_141f4b450", fn_141f4b450 as *const (), Throws::Discard), // sub_141f4b450
    Replacement::draft_throws(0x141f4ba10, "physx_141f4ba10", fn_141f4ba10 as *const (), Throws::Caught), // sub_141f4ba10
    Replacement::draft_throws(0x141f4bc30, "physx_141f4bc30", fn_141f4bc30 as *const (), Throws::Discard), // sub_141f4bc30
    Replacement::draft_throws(0x141f4bd40, "physx_141f4bd40", fn_141f4bd40 as *const (), Throws::Discard), // sub_141f4bd40
    Replacement::draft_throws(0x141f4be50, "physx_141f4be50", fn_141f4be50 as *const (), Throws::Caught), // sub_141f4be50
    Replacement::draft_throws(0x141f4c270, "physx_141f4c270", fn_141f4c270 as *const (), Throws::Discard), // sub_141f4c270
    Replacement::draft_throws(0x141f4c5c0, "physx_141f4c5c0", fn_141f4c5c0 as *const (), Throws::Caught), // sub_141f4c5c0
    Replacement::draft_throws(0x141f4c7b0, "unknown_141f4c7b0", fn_141f4c7b0 as *const (), Throws::Caught), // sub_141f4c7b0
    Replacement::draft_throws(0x141f4c9b0, "unknown_141f4c9b0", fn_141f4c9b0 as *const (), Throws::Discard), // sub_141f4c9b0
    Replacement::draft_throws(0x141f4cb60, "unknown_141f4cb60", fn_141f4cb60 as *const (), Throws::Discard), // sub_141f4cb60
    Replacement::draft(0x141f4cbd0, "unknown_141f4cbd0", fn_141f4cbd0 as *const ()), // sub_141f4cbd0
    Replacement::draft(0x141f4ccd0, "unknown_141f4ccd0", tpl_34443 as *const ()), // sub_141f4ccd0
    Replacement::draft(0x141f4cce0, "unknown_141f4cce0", tpl_361 as *const ()), // sub_141f4cce0
    Replacement::draft(0x141f4ccf0, "unknown_141f4ccf0", fn_141f4ccf0 as *const ()), // sub_141f4ccf0
    Replacement::draft(0x141f4d920, "coregame_141f4d920", fn_141f4d920 as *const ()), // coregame::GameWindowMouse::coregame::GameWindowMouse::vfunc_14
    Replacement::draft(0x141f4d950, "coregame_141f4d950", fn_141f4d950 as *const ()), // coregame::GameWindowMouse::coregame::GameWindowMouse::vfunc_15
    Replacement::draft_throws(0x141f4d9a0, "unknown_141f4d9a0", fn_141f4d9a0 as *const (), Throws::Discard), // sub_141f4d9a0
    Replacement::draft(0x141f4da60, "unknown_141f4da60", fn_141f4da60 as *const ()), // sub_141f4da60
    Replacement::draft(0x141f4dba0, "unknown_141f4dba0", tpl_34443 as *const ()), // sub_141f4dba0
    Replacement::draft(0x141f4dbb0, "unknown_141f4dbb0", fn_141f4dbb0 as *const ()), // sub_141f4dbb0
    Replacement::draft(0x141f4dbe0, "unknown_141f4dbe0", tpl_34443 as *const ()), // sub_141f4dbe0
    Replacement::draft(0x141f4dbf0, "unknown_141f4dbf0", fn_141f4dbf0 as *const ()), // sub_141f4dbf0
    Replacement::draft(0x141f4dc20, "unknown_141f4dc20", fn_141f4dc20 as *const ()), // sub_141f4dc20
    Replacement::draft(0x141f4dce0, "unknown_141f4dce0", fn_141f4dce0 as *const ()), // sub_141f4dce0
    Replacement::draft(0x141f4dea0, "unknown_141f4dea0", fn_141f4dea0 as *const ()), // sub_141f4dea0
    Replacement::draft(0x141f4df00, "luau_141f4df00", fn_141f4df00 as *const ()), // sub_141f4df00
    Replacement::draft(0x141f4e270, "unknown_141f4e270", tpl_369 as *const ()), // sub_141f4e270
    Replacement::draft(0x141f4e280, "unknown_141f4e280", tpl_369 as *const ()), // sub_141f4e280
    Replacement::draft_throws(0x141f4e290, "unknown_141f4e290", fn_141f4e290 as *const (), Throws::Caught), // sub_141f4e290
    Replacement::draft_throws(0x141f4e520, "unknown_141f4e520", fn_141f4e520 as *const (), Throws::Caught), // sub_141f4e520
    Replacement::draft(0x141f4e630, "unknown_141f4e630", fn_141f4e630 as *const ()), // sub_141f4e630
    Replacement::draft(0x141f4e6b0, "unknown_141f4e6b0", fn_141f4e6b0 as *const ()), // sub_141f4e6b0
    Replacement::draft_throws(0x141f4e6d0, "coregame_141f4e6d0", fn_141f4e6d0 as *const (), Throws::Discard), // coregame::GameWindow::coregame::GameWindow::vfunc_3
    Replacement::draft_throws(0x141f4e7e0, "coregame_141f4e7e0", fn_141f4e7e0 as *const (), Throws::Discard), // coregame::GameWindow::coregame::GameWindow::vfunc_6
    Replacement::draft_throws(0x141f4e8b0, "coregame_141f4e8b0", fn_141f4e8b0 as *const (), Throws::Caught), // coregame::GameWindow::coregame::GameWindow::vfunc_7
    Replacement::draft(0x141f4e970, "coregame_141f4e970", fn_141f4e970 as *const ()), // coregame::GameWindow::coregame::GameWindow::vfunc_19
    Replacement::draft(0x141f4e980, "coregame_141f4e980", fn_141f4e980 as *const ()), // coregame::GameWindow::coregame::GameWindow::vfunc_20
    Replacement::draft(0x141f4ea70, "coregame_141f4ea70", fn_141f4ea70 as *const ()), // coregame::GameWindow::coregame::GameWindow::vfunc_21
    Replacement::draft(0x141f4ea80, "coregame_141f4ea80", fn_141f4ea80 as *const ()), // coregame::GameWindow::coregame::GameWindow::vfunc_1
    Replacement::draft_throws(0x141f4ec30, "unknown_141f4ec30", fn_141f4ec30 as *const (), Throws::Discard), // sub_141f4ec30
    Replacement::draft(0x141f4f440, "unknown_141f4f440", fn_141f4f440 as *const ()), // sub_141f4f440
    Replacement::draft(0x141f4f4b0, "unknown_141f4f4b0", fn_141f4f4b0 as *const ()), // sub_141f4f4b0
    Replacement::draft_throws(0x141f4f5a0, "unknown_141f4f5a0", fn_141f4f5a0 as *const (), Throws::Caught), // sub_141f4f5a0
    Replacement::draft_throws(0x141f4f6a0, "unknown_141f4f6a0", fn_141f4f6a0 as *const (), Throws::Discard), // sub_141f4f6a0
    Replacement::draft(0x141f4f810, "unknown_141f4f810", fn_141f4f810 as *const ()), // sub_141f4f810
    Replacement::draft(0x141f4fb80, "unknown_141f4fb80", fn_141f4fb80 as *const ()), // sub_141f4fb80
    Replacement::draft(0x141f4fd50, "unknown_141f4fd50", fn_141f4fd50 as *const ()), // sub_141f4fd50
    Replacement::draft(0x141f4fd80, "unknown_141f4fd80", fn_141f4fd80 as *const ()), // sub_141f4fd80
    Replacement::draft(0x141f4ff40, "unknown_141f4ff40", fn_141f4ff40 as *const ()), // sub_141f4ff40
    Replacement::draft(0x141f4ff80, "unknown_141f4ff80", fn_141f4ff80 as *const ()), // sub_141f4ff80
    Replacement::draft(0x141f4ffe0, "unknown_141f4ffe0", fn_141f4ffe0 as *const ()), // sub_141f4ffe0
    Replacement::draft(0x141f50140, "unknown_141f50140", fn_141f50140 as *const ()), // sub_141f50140
    Replacement::draft(0x141f501c0, "unknown_141f501c0", fn_141f501c0 as *const ()), // sub_141f501c0
    Replacement::draft(0x141f50200, "rapidjson_141f50200", fn_141f50200 as *const ()), // sub_141f50200
    Replacement::draft(0x141f50340, "luau_141f50340", fn_141f50340 as *const ()), // sub_141f50340
    Replacement::draft(0x141f503c0, "unknown_141f503c0", fn_141f503c0 as *const ()), // sub_141f503c0
    Replacement::draft(0x141f503e0, "unknown_141f503e0", tpl_42053::<0x48, 0x4c, 0x4> as *const ()), // sub_141f503e0
    Replacement::draft(0x141f50400, "unknown_141f50400", fn_141f50400 as *const ()), // sub_141f50400
    Replacement::draft(0x141f50420, "unknown_141f50420", tpl_30459::<0xb8> as *const ()), // sub_141f50420
    Replacement::draft(0x141f50440, "unknown_141f50440", tpl_30459::<0xec> as *const ()), // sub_141f50440
    Replacement::draft_throws(0x141f50460, "unknown_141f50460", fn_141f50460 as *const (), Throws::Discard), // sub_141f50460
    Replacement::draft_throws(0x141f50560, "unknown_141f50560", fn_141f50560 as *const (), Throws::Discard), // sub_141f50560
    Replacement::draft(0x141f505c0, "unknown_141f505c0", fn_141f505c0 as *const ()), // sub_141f505c0
    Replacement::draft(0x141f506e0, "unknown_141f506e0", fn_141f506e0 as *const ()), // sub_141f506e0
    Replacement::draft(0x141f50750, "unknown_141f50750", fn_141f50750 as *const ()), // sub_141f50750
    Replacement::draft(0x141f50770, "unknown_141f50770", tpl_4544::<0x64> as *const ()), // sub_141f50770
    Replacement::draft(0x141f50780, "unknown_141f50780", tpl_30363::<0x64> as *const ()), // sub_141f50780
    Replacement::draft(0x141f50790, "unknown_141f50790", tpl_42053::<0x40, 0x44, 0x4> as *const ()), // sub_141f50790
    Replacement::draft(0x141f507b0, "unknown_141f507b0", fn_141f507b0 as *const ()), // sub_141f507b0
    Replacement::draft(0x141f50860, "unknown_141f50860", fn_141f50860 as *const ()), // sub_141f50860
    Replacement::draft_throws(0x141f50e90, "unknown_141f50e90", fn_141f50e90 as *const (), Throws::Discard), // sub_141f50e90
    Replacement::draft_throws(0x141f51390, "unknown_141f51390", fn_141f51390 as *const (), Throws::Caught), // sub_141f51390
    Replacement::draft_throws(0x141f51ef0, "unknown_141f51ef0", fn_141f51ef0 as *const (), Throws::Caught), // sub_141f51ef0
    Replacement::draft(0x141f52150, "unknown_141f52150", fn_141f52150 as *const ()), // sub_141f52150
    Replacement::draft(0x141f521e0, "unknown_141f521e0", fn_141f521e0 as *const ()), // sub_141f521e0
    Replacement::draft(0x141f52220, "unknown_141f52220", tpl_33902::<0x30, 0xa0> as *const ()), // sub_141f52220
    Replacement::draft(0x141f52250, "unknown_141f52250", tpl_4544::<0x63> as *const ()), // sub_141f52250
    Replacement::draft(0x141f52260, "unknown_141f52260", tpl_4544::<0x62> as *const ()), // sub_141f52260
    Replacement::draft(0x141f52270, "unknown_141f52270", tpl_4544::<0x60> as *const ()), // sub_141f52270
    Replacement::draft(0x141f52280, "unknown_141f52280", fn_141f52280 as *const ()), // sub_141f52280
    Replacement::draft(0x141f52370, "unknown_141f52370", tpl_4544::<0x61> as *const ()), // sub_141f52370
    Replacement::draft(0x141f52380, "unknown_141f52380", tpl_1738::<0x38> as *const ()), // sub_141f52380
    Replacement::draft(0x141f52390, "unknown_141f52390", tpl_366::<0x14397da50> as *const ()), // sub_141f52390
    Replacement::draft(0x141f523a0, "unknown_141f523a0", tpl_27067::<0x30, 0xa8> as *const ()), // sub_141f523a0
    Replacement::draft(0x141f523c0, "unknown_141f523c0", fn_141f523c0 as *const ()), // sub_141f523c0
    Replacement::draft_throws(0x141f52600, "unknown_141f52600", fn_141f52600 as *const (), Throws::Caught), // sub_141f52600
    Replacement::draft(0x141f52dc0, "unknown_141f52dc0", fn_141f52dc0 as *const ()), // sub_141f52dc0
    Replacement::draft(0x141f52e60, "unknown_141f52e60", fn_141f52e60 as *const ()), // sub_141f52e60
    Replacement::draft(0x141f52e80, "unknown_141f52e80", fn_141f52e80 as *const ()), // sub_141f52e80
    Replacement::draft(0x141f52f40, "unknown_141f52f40", fn_141f52f40 as *const ()), // sub_141f52f40
    Replacement::draft(0x141f52f90, "unknown_141f52f90", fn_141f52f90 as *const ()), // sub_141f52f90
    Replacement::draft(0x141f53000, "unknown_141f53000", fn_141f53000 as *const ()), // sub_141f53000
    Replacement::draft_throws(0x141f53020, "unknown_141f53020", fn_141f53020 as *const (), Throws::Discard), // sub_141f53020
    Replacement::draft(0x141f53080, "unknown_141f53080", fn_141f53080 as *const ()), // sub_141f53080
    Replacement::draft(0x141f530c0, "unknown_141f530c0", fn_141f530c0 as *const ()), // sub_141f530c0
    Replacement::draft(0x141f53110, "unknown_141f53110", fn_141f53110 as *const ()), // sub_141f53110
    Replacement::draft(0x141f53120, "unknown_141f53120", fn_141f53120 as *const ()), // sub_141f53120
    Replacement::draft_throws(0x141f53190, "unknown_141f53190", fn_141f53190 as *const (), Throws::Discard), // sub_141f53190
    Replacement::draft(0x141f535d0, "unknown_141f535d0", fn_141f535d0 as *const ()), // sub_141f535d0
    Replacement::draft(0x141f53740, "unknown_141f53740", fn_141f53740 as *const ()), // sub_141f53740
    Replacement::draft(0x141f53960, "unknown_141f53960", fn_141f53960 as *const ()), // sub_141f53960
    Replacement::draft(0x141f53a70, "unknown_141f53a70", tpl_1610::<0x14240ab10, 0x48> as *const ()), // sub_141f53a70
    Replacement::draft(0x141f53ab0, "unknown_141f53ab0", fn_141f53ab0 as *const ()), // sub_141f53ab0
    Replacement::draft(0x141f54160, "unknown_141f54160", fn_141f54160 as *const ()), // sub_141f54160
    Replacement::draft(0x141f541b0, "unknown_141f541b0", tpl_41210::<0x143390ff8, 0x1401f3d90, 0x10, 0x18, 0x143390e10> as *const ()), // sub_141f541b0
    Replacement::draft(0x141f541f0, "unknown_141f541f0", tpl_41210::<0x143390e40, 0x1401f3d90, 0x10, 0x18, 0x1433910a0> as *const ()), // sub_141f541f0
    Replacement::draft(0x141f54230, "unknown_141f54230", tpl_41210::<0x143390df8, 0x1401f3d90, 0x10, 0x18, 0x143391028> as *const ()), // sub_141f54230
    Replacement::draft(0x141f54270, "unknown_141f54270", tpl_35429::<0x143391088, 0x8, 0x1401f3d90, 0x20, 0x28, 0x143391010> as *const ()), // sub_141f54270
    Replacement::draft(0x141f542c0, "std_141f542c0", tpl_550::<0x143390fc0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_1>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f542e0, "std_141f542e0", tpl_550::<0x143390fc0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_1>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54300, "std_141f54300", tpl_42081::<0x8, 0x143a2addc, 0x1410cf3b0> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_1>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54360, "std_141f54360", tpl_1::<0x143890320> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_1>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54370, "std_141f54370", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_1>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54380, "std_141f54380", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_1>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f543a0, "std_141f543a0", tpl_550::<0x143390e70, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_2>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f543c0, "std_141f543c0", tpl_550::<0x143390e70, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_2>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f543e0, "std_141f543e0", tpl_42081::<0x8, 0x143a2a12c, 0x1410cf3b0> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_2>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54440, "std_141f54440", tpl_1::<0x143890260> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_2>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54450, "std_141f54450", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_2>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54460, "std_141f54460", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::GameServer::onDeferredWriteAchievementsFailed::<lambda_2>,void>::std::_Func_impl_no_a
    Replacement::draft(0x141f54480, "unknown_141f54480", tpl_361 as *const ()), // sub_141f54480
    Replacement::draft(0x141f54490, "unknown_141f54490", tpl_361 as *const ()), // sub_141f54490
    Replacement::draft(0x141f544a0, "unknown_141f544a0", tpl_361 as *const ()), // sub_141f544a0
    Replacement::draft(0x141f544b0, "unknown_141f544b0", tpl_361 as *const ()), // sub_141f544b0
    Replacement::draft(0x141f544c0, "unknown_141f544c0", tpl_361 as *const ()), // sub_141f544c0
    Replacement::draft(0x141f54500, "unknown_141f54500", tpl_473 as *const ()), // sub_141f54500
    Replacement::draft(0x141f54510, "unknown_141f54510", tpl_473 as *const ()), // sub_141f54510
    Replacement::draft(0x141f54520, "unknown_141f54520", tpl_473 as *const ()), // sub_141f54520
    Replacement::draft(0x141f54530, "unknown_141f54530", tpl_34259 as *const ()), // sub_141f54530
    Replacement::draft(0x141f54550, "unknown_141f54550", tpl_34374::<0x10, 0x18, 0x8> as *const ()), // sub_141f54550
    Replacement::draft_throws(0x141f54980, "unknown_141f54980", fn_141f54980 as *const (), Throws::Caught), // sub_141f54980
    Replacement::draft(0x141f54aa0, "unknown_141f54aa0", tpl_40910::<0x143a2a12c, 0x1410cf3b0> as *const ()), // sub_141f54aa0
    Replacement::draft(0x141f54b10, "unknown_141f54b10", tpl_40910::<0x143a2addc, 0x1410cf3b0> as *const ()), // sub_141f54b10
    Replacement::draft_throws(0x141f54b70, "unknown_141f54b70", fn_141f54b70 as *const (), Throws::Discard), // sub_141f54b70
    Replacement::draft(0x141f54ba0, "unknown_141f54ba0", fn_141f54ba0 as *const ()), // sub_141f54ba0
    Replacement::draft_throws(0x141f54bb0, "unknown_141f54bb0", fn_141f54bb0 as *const (), Throws::Caught), // sub_141f54bb0
    Replacement::draft(0x141f55580, "unknown_141f55580", fn_141f55580 as *const ()), // sub_141f55580
    Replacement::draft_throws(0x141f555a0, "unknown_141f555a0", fn_141f555a0 as *const (), Throws::Discard), // sub_141f555a0
    Replacement::draft(0x141f557d0, "unknown_141f557d0", fn_141f557d0 as *const ()), // sub_141f557d0
    Replacement::draft(0x141f55810, "unknown_141f55810", fn_141f55810 as *const ()), // sub_141f55810
    Replacement::draft(0x141f55840, "unknown_141f55840", fn_141f55840 as *const ()), // sub_141f55840
    Replacement::draft(0x141f55890, "unknown_141f55890", fn_141f55890 as *const ()), // sub_141f55890
    Replacement::draft(0x141f558c0, "unknown_141f558c0", fn_141f558c0 as *const ()), // sub_141f558c0
    Replacement::draft(0x141f558e0, "unknown_141f558e0", tpl_29552::<0x8> as *const ()), // sub_141f558e0
    Replacement::draft(0x141f55900, "unknown_141f55900", fn_141f55900 as *const ()), // sub_141f55900
    Replacement::draft(0x141f55a30, "unknown_141f55a30", fn_141f55a30 as *const ()), // sub_141f55a30
    Replacement::draft_throws(0x141f55a90, "unknown_141f55a90", fn_141f55a90 as *const (), Throws::Caught), // sub_141f55a90
    Replacement::draft(0x141f55c70, "unknown_141f55c70", fn_141f55c70 as *const ()), // sub_141f55c70
    Replacement::draft(0x141f55e10, "unknown_141f55e10", fn_141f55e10 as *const ()), // sub_141f55e10
    Replacement::draft_throws(0x141f55f30, "unknown_141f55f30", fn_141f55f30 as *const (), Throws::Caught), // sub_141f55f30
    Replacement::draft_throws(0x141f55fa0, "unknown_141f55fa0", fn_141f55fa0 as *const (), Throws::Caught), // sub_141f55fa0
    Replacement::draft(0x141f560a0, "coregame_141f560a0", fn_141f560a0 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_30
    Replacement::draft_throws(0x141f561b0, "coregame_141f561b0", fn_141f561b0 as *const (), Throws::Caught), // coregame::GameServer::coregame::GameServer::vfunc_28
    Replacement::draft(0x141f56410, "coregame_141f56410", fn_141f56410 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_20
    Replacement::draft(0x141f56490, "coregame_141f56490", fn_141f56490 as *const ()), // sub_141f56490
    Replacement::draft(0x141f564b0, "coregame_141f564b0", fn_141f564b0 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_11
    Replacement::draft(0x141f565d0, "coregame_141f565d0", fn_141f565d0 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_8
    Replacement::draft(0x141f565f0, "coregame_141f565f0", fn_141f565f0 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_7
    Replacement::draft(0x141f56620, "coregame_141f56620", fn_141f56620 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_5
    Replacement::draft(0x141f566a0, "coregame_141f566a0", fn_141f566a0 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_4
    Replacement::draft(0x141f56700, "coregame_141f56700", fn_141f56700 as *const ()), // coregame::GameServer::coregame::GameServer::vfunc_2
    Replacement::draft_throws(0x141f56760, "coregame_141f56760", fn_141f56760 as *const (), Throws::Discard), // coregame::GameServer::coregame::GameServer::vfunc_1
    Replacement::draft(0x141f567a0, "unknown_141f567a0", fn_141f567a0 as *const ()), // sub_141f567a0
    Replacement::draft(0x141f567c0, "unknown_141f567c0", fn_141f567c0 as *const ()), // sub_141f567c0
    Replacement::draft(0x141f568a0, "unknown_141f568a0", fn_141f568a0 as *const ()), // sub_141f568a0
    Replacement::draft_throws(0x141f569b0, "unknown_141f569b0", fn_141f569b0 as *const (), Throws::Caught), // sub_141f569b0
    Replacement::draft(0x141f56e40, "unknown_141f56e40", fn_141f56e40 as *const ()), // sub_141f56e40
    Replacement::draft(0x141f56ea0, "unknown_141f56ea0", fn_141f56ea0 as *const ()), // sub_141f56ea0
    Replacement::draft(0x141f56ec0, "unknown_141f56ec0", tpl_4544::<0xea> as *const ()), // sub_141f56ec0
    Replacement::draft(0x141f56ee0, "unknown_141f56ee0", fn_141f56ee0 as *const ()), // sub_141f56ee0
    Replacement::draft(0x141f572e0, "unknown_141f572e0", fn_141f572e0 as *const ()), // sub_141f572e0
    Replacement::draft(0x141f57300, "unknown_141f57300", tpl_42117::<0x14248a640, 0x50, 0x38, 0x40, 0x28, 0x24, 0x8, 0x30> as *const ()), // sub_141f57300
    Replacement::draft(0x141f57480, "unknown_141f57480", fn_141f57480 as *const ()), // sub_141f57480
    Replacement::draft(0x141f57510, "unknown_141f57510", fn_141f57510 as *const ()), // sub_141f57510
    Replacement::draft(0x141f575d0, "unknown_141f575d0", fn_141f575d0 as *const ()), // sub_141f575d0
    Replacement::draft(0x141f57600, "unknown_141f57600", fn_141f57600 as *const ()), // sub_141f57600
    Replacement::draft(0x141f57790, "unknown_141f57790", fn_141f57790 as *const ()), // sub_141f57790
    Replacement::draft(0x141f57860, "unknown_141f57860", fn_141f57860 as *const ()), // sub_141f57860
    Replacement::draft_throws(0x141f578b0, "unknown_141f578b0", fn_141f578b0 as *const (), Throws::Discard), // sub_141f578b0
    Replacement::draft(0x141f57bb0, "unknown_141f57bb0", fn_141f57bb0 as *const ()), // sub_141f57bb0
    Replacement::draft(0x141f581c0, "unknown_141f581c0", fn_141f581c0 as *const ()), // sub_141f581c0
    Replacement::draft(0x141f58250, "unknown_141f58250", fn_141f58250 as *const ()), // sub_141f58250
    Replacement::draft_throws(0x141f582f0, "unknown_141f582f0", fn_141f582f0 as *const (), Throws::Discard), // sub_141f582f0
    Replacement::draft_throws(0x141f58350, "unknown_141f58350", fn_141f58350 as *const (), Throws::Discard), // sub_141f58350
    Replacement::draft(0x141f583b0, "unknown_141f583b0", fn_141f583b0 as *const ()), // sub_141f583b0
    Replacement::draft(0x141f58440, "unknown_141f58440", fn_141f58440 as *const ()), // sub_141f58440
    Replacement::draft(0x141f58480, "unknown_141f58480", fn_141f58480 as *const ()), // sub_141f58480
    Replacement::draft_throws(0x141f58620, "unknown_141f58620", fn_141f58620 as *const (), Throws::Discard), // sub_141f58620
    Replacement::draft_throws(0x141f58670, "unknown_141f58670", fn_141f58670 as *const (), Throws::Discard), // sub_141f58670
    Replacement::draft_throws(0x141f58710, "unknown_141f58710", fn_141f58710 as *const (), Throws::Discard), // sub_141f58710
    Replacement::draft(0x141f587b0, "unknown_141f587b0", fn_141f587b0 as *const ()), // sub_141f587b0
    Replacement::draft(0x141f58920, "unknown_141f58920", fn_141f58920 as *const ()), // sub_141f58920
    Replacement::draft_throws(0x141f58f10, "unknown_141f58f10", tpl_42137::<0x141f5a270> as *const (), Throws::Discard), // sub_141f58f10
    Replacement::draft_throws(0x141f58f30, "unknown_141f58f30", fn_141f58f30 as *const (), Throws::Discard), // sub_141f58f30
    Replacement::draft(0x141f59090, "unknown_141f59090", fn_141f59090 as *const ()), // sub_141f59090
    Replacement::draft(0x141f590c0, "unknown_141f590c0", fn_141f590c0 as *const ()), // sub_141f590c0
    Replacement::draft_throws(0x141f59490, "unknown_141f59490", fn_141f59490 as *const (), Throws::Discard), // sub_141f59490
    Replacement::draft_throws(0x141f5a970, "unknown_141f5a970", fn_141f5a970 as *const (), Throws::Discard), // sub_141f5a970
    Replacement::draft_throws(0x141f5af40, "unknown_141f5af40", fn_141f5af40 as *const (), Throws::Discard), // sub_141f5af40
    Replacement::draft(0x141f5b280, "unknown_141f5b280", fn_141f5b280 as *const ()), // sub_141f5b280
    Replacement::draft(0x141f5b470, "unknown_141f5b470", fn_141f5b470 as *const ()), // sub_141f5b470
    Replacement::draft_throws(0x141f5b500, "unknown_141f5b500", fn_141f5b500 as *const (), Throws::Discard), // sub_141f5b500
    Replacement::draft(0x141f5b5d0, "unknown_141f5b5d0", fn_141f5b5d0 as *const ()), // sub_141f5b5d0
    Replacement::draft_throws(0x141f5b9a0, "unknown_141f5b9a0", fn_141f5b9a0 as *const (), Throws::Caught), // sub_141f5b9a0
    Replacement::draft(0x141f5bc30, "unknown_141f5bc30", tpl_42155::<0x1410d3420, 0x70, 0x140c345c0, 0x8, 0x1410d3a00> as *const ()), // sub_141f5bc30
    Replacement::draft(0x141f5bc60, "unknown_141f5bc60", fn_141f5bc60 as *const ()), // sub_141f5bc60
    Replacement::draft(0x141f5bc80, "unknown_141f5bc80", fn_141f5bc80 as *const ()), // sub_141f5bc80
    Replacement::draft_throws(0x141f5bd50, "unknown_141f5bd50", fn_141f5bd50 as *const (), Throws::Discard), // sub_141f5bd50
    Replacement::draft(0x141f5c120, "unknown_141f5c120", tpl_369 as *const ()), // sub_141f5c120
    Replacement::draft(0x141f5c130, "unknown_141f5c130", tpl_38159::<0x143428120, 0x143428460, 0x140275010> as *const ()), // sub_141f5c130
    Replacement::draft(0x141f5c290, "unknown_141f5c290", fn_141f5c290 as *const ()), // sub_141f5c290
    Replacement::draft(0x141f5c510, "unknown_141f5c510", fn_141f5c510 as *const ()), // sub_141f5c510
    Replacement::draft(0x141f5c760, "unknown_141f5c760", fn_141f5c760 as *const ()), // sub_141f5c760
    Replacement::draft(0x141f5c9f0, "unknown_141f5c9f0", fn_141f5c9f0 as *const ()), // sub_141f5c9f0
    Replacement::draft(0x141f5cce0, "unknown_141f5cce0", fn_141f5cce0 as *const ()), // sub_141f5cce0
    Replacement::draft(0x141f5ce20, "unknown_141f5ce20", fn_141f5ce20 as *const ()), // sub_141f5ce20
    Replacement::draft(0x141f5d030, "unknown_141f5d030", fn_141f5d030 as *const ()), // sub_141f5d030
    Replacement::draft(0x141f5d1f0, "unknown_141f5d1f0", tpl_34252 as *const ()), // sub_141f5d1f0
    Replacement::draft(0x141f5d200, "unknown_141f5d200", fn_141f5d200 as *const ()), // sub_141f5d200
    Replacement::draft(0x141f5d280, "unknown_141f5d280", fn_141f5d280 as *const ()), // sub_141f5d280
    Replacement::draft(0x141f5d3f0, "unknown_141f5d3f0", fn_141f5d3f0 as *const ()), // sub_141f5d3f0
    Replacement::draft(0x141f5d440, "std_141f5d440", tpl_579::<0x143391880, 0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::GameLoopSelector,ecs::gameloop::Id,Storage::Storage&,coregame::UpdateMode>
    Replacement::draft(0x141f5d460, "std_141f5d460", tpl_579::<0x143391880, 0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::GameLoopSelector,ecs::gameloop::Id,Storage::Storage&,coregame::UpdateMode>
    Replacement::draft(0x141f5d480, "std_141f5d480", fn_141f5d480 as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::GameLoopSelector,ecs::gameloop::Id,Storage::Storage&,coregame::UpdateMode>
    Replacement::draft(0x141f5d4e0, "std_141f5d4e0", tpl_1::<0x1438903e0> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::GameLoopSelector,ecs::gameloop::Id,Storage::Storage&,coregame::UpdateMode>
    Replacement::draft(0x141f5d4f0, "std_141f5d4f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::GameLoopSelector,ecs::gameloop::Id,Storage::Storage&,coregame::UpdateMode>
    Replacement::draft(0x141f5d500, "std_141f5d500", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::GameLoopSelector,ecs::gameloop::Id,Storage::Storage&,coregame::UpdateMode>
    Replacement::draft(0x141f5d520, "std_141f5d520", fn_141f5d520 as *const ()), // sub_141f5d520
    Replacement::draft_throws(0x141f5d540, "std_141f5d540", fn_141f5d540 as *const (), Throws::Discard), // sub_141f5d540
    Replacement::draft_throws(0x141f5d5c0, "std_141f5d5c0", fn_141f5d5c0 as *const (), Throws::Caught), // sub_141f5d5c0
    Replacement::draft(0x141f5db90, "std_141f5db90", fn_141f5db90 as *const ()), // sub_141f5db90
    Replacement::draft(0x141f5dbc0, "std_141f5dbc0", fn_141f5dbc0 as *const ()), // sub_141f5dbc0
    Replacement::draft(0x141f5dc20, "std_141f5dc20", tpl_101994::<0x143c8ce94, 0x143879674, 0x143cc5814, 0x1434268d0, 0x143879660, 0x1438548d8, 0x143426574, 0x143426a40> as *const ()), // sub_141f5dc20
    Replacement::draft_throws(0x141f5dd70, "std_141f5dd70", fn_141f5dd70 as *const (), Throws::Discard), // sub_141f5dd70
    Replacement::draft(0x141f5de80, "std_141f5de80", fn_141f5de80 as *const ()), // sub_141f5de80
    Replacement::draft(0x141f5e150, "std_141f5e150", fn_141f5e150 as *const ()), // sub_141f5e150
    Replacement::draft(0x141f5e240, "std_141f5e240", tpl_365::<0x141f5e260> as *const ()), // sub_141f5e240
    Replacement::draft(0x141f5e250, "std_141f5e250", tpl_361 as *const ()), // sub_141f5e250
    Replacement::draft(0x141f5e260, "std_141f5e260", fn_141f5e260 as *const ()), // sub_141f5e260
    Replacement::draft(0x141f5e2e0, "std_141f5e2e0", fn_141f5e2e0 as *const ()), // sub_141f5e2e0
    Replacement::draft_throws(0x141f5e330, "std_141f5e330", fn_141f5e330 as *const (), Throws::Caught), // sub_141f5e330
    Replacement::draft_throws(0x141f5f130, "std_141f5f130", fn_141f5f130 as *const (), Throws::Caught), // sub_141f5f130
    Replacement::draft(0x141f5f2a0, "std_141f5f2a0", tpl_369 as *const ()), // sub_141f5f2a0
    Replacement::draft_throws(0x141f5f2b0, "std_141f5f2b0", fn_141f5f2b0 as *const (), Throws::Discard), // sub_141f5f2b0
    Replacement::draft_throws(0x141f5f4d0, "std_141f5f4d0", fn_141f5f4d0 as *const (), Throws::Discard), // sub_141f5f4d0
    Replacement::draft(0x141f5f520, "std_141f5f520", fn_141f5f520 as *const ()), // sub_141f5f520
    Replacement::draft_throws(0x141f5f550, "std_141f5f550", fn_141f5f550 as *const (), Throws::Discard), // sub_141f5f550
    Replacement::draft(0x141f5f570, "std_141f5f570", fn_141f5f570 as *const ()), // sub_141f5f570
    Replacement::draft(0x141f5f6c0, "std_141f5f6c0", tpl_550::<0x143391c40, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameHelper::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const ?bref4::PhysicsSw
    Replacement::draft(0x141f5f6e0, "std_141f5f6e0", tpl_550::<0x143391c40, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameHelper::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const ?bref4::PhysicsSw
    Replacement::draft(0x141f5f700, "std_141f5f700", fn_141f5f700 as *const ()), // std::_Func_impl_no_alloc<coregame::GameHelper::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const ?bref4::PhysicsSw
    Replacement::draft(0x141f5f7a0, "std_141f5f7a0", tpl_1::<0x143890450> as *const ()), // std::_Func_impl_no_alloc<coregame::GameHelper::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const ?bref4::PhysicsSw
    Replacement::draft(0x141f5f7b0, "std_141f5f7b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::GameHelper::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const ?bref4::PhysicsSw
    Replacement::draft(0x141f5f7c0, "std_141f5f7c0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::GameHelper::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const ?bref4::PhysicsSw
    Replacement::draft(0x141f5f7e0, "unknown_141f5f7e0", fn_141f5f7e0 as *const ()), // sub_141f5f7e0
    Replacement::draft(0x141f5f830, "unknown_141f5f830", fn_141f5f830 as *const ()), // sub_141f5f830
    Replacement::draft(0x141f5fa80, "unknown_141f5fa80", fn_141f5fa80 as *const ()), // sub_141f5fa80
    Replacement::draft(0x141f5fb20, "unknown_141f5fb20", fn_141f5fb20 as *const ()), // sub_141f5fb20
    Replacement::draft(0x141f5fbc0, "unknown_141f5fbc0", fn_141f5fbc0 as *const ()), // sub_141f5fbc0
    Replacement::draft(0x141f5fd00, "unknown_141f5fd00", fn_141f5fd00 as *const ()), // sub_141f5fd00
    Replacement::draft(0x141f5ff00, "unknown_141f5ff00", fn_141f5ff00 as *const ()), // sub_141f5ff00
    Replacement::draft(0x141f60010, "unknown_141f60010", fn_141f60010 as *const ()), // sub_141f60010
    Replacement::draft(0x141f600f0, "unknown_141f600f0", fn_141f600f0 as *const ()), // sub_141f600f0
    Replacement::draft_throws(0x141f601a0, "unknown_141f601a0", fn_141f601a0 as *const (), Throws::Discard), // sub_141f601a0
    Replacement::draft(0x141f601f0, "unknown_141f601f0", fn_141f601f0 as *const ()), // sub_141f601f0
    Replacement::draft(0x141f603c0, "unknown_141f603c0", fn_141f603c0 as *const ()), // sub_141f603c0
    Replacement::draft(0x141f60550, "unknown_141f60550", fn_141f60550 as *const ()), // sub_141f60550
    Replacement::draft(0x141f605e0, "unknown_141f605e0", fn_141f605e0 as *const ()), // sub_141f605e0
    Replacement::draft(0x141f606d0, "unknown_141f606d0", fn_141f606d0 as *const ()), // sub_141f606d0
    Replacement::draft(0x141f607a0, "unknown_141f607a0", fn_141f607a0 as *const ()), // sub_141f607a0
    Replacement::draft(0x141f60900, "unknown_141f60900", fn_141f60900 as *const ()), // sub_141f60900
    Replacement::draft(0x141f60a60, "unknown_141f60a60", fn_141f60a60 as *const ()), // sub_141f60a60
    Replacement::draft(0x141f60ac0, "unknown_141f60ac0", fn_141f60ac0 as *const ()), // sub_141f60ac0
    Replacement::draft(0x141f60b70, "unknown_141f60b70", fn_141f60b70 as *const ()), // sub_141f60b70
    Replacement::draft_throws(0x141f60bb0, "unknown_141f60bb0", fn_141f60bb0 as *const (), Throws::Discard), // sub_141f60bb0
    Replacement::draft_throws(0x141f60c00, "unknown_141f60c00", fn_141f60c00 as *const (), Throws::Discard), // sub_141f60c00
    Replacement::draft_throws(0x141f60c90, "unknown_141f60c90", fn_141f60c90 as *const (), Throws::Discard), // sub_141f60c90
    Replacement::draft(0x141f60d40, "unknown_141f60d40", fn_141f60d40 as *const ()), // sub_141f60d40
    Replacement::draft_throws(0x141f60d60, "unknown_141f60d60", tpl_42201::<0xffffffffffffffff, 0x141f60c90, 0x30, 0x38, 0x74> as *const (), Throws::Discard), // sub_141f60d60
    Replacement::draft(0x141f60db0, "unknown_141f60db0", fn_141f60db0 as *const ()), // sub_141f60db0
    Replacement::draft_throws(0x141f60eb0, "unknown_141f60eb0", fn_141f60eb0 as *const (), Throws::Discard), // sub_141f60eb0
    Replacement::draft(0x141f60fb0, "unknown_141f60fb0", tpl_42203::<0x30, 0x14397ff58, 0x6d, 0x1412d0a20> as *const ()), // sub_141f60fb0
    Replacement::draft(0x141f60fe0, "unknown_141f60fe0", tpl_42204::<0x30, 0x14397ff58, 0x1412d0b70> as *const ()), // sub_141f60fe0
    Replacement::draft_throws(0x141f61010, "unknown_141f61010", fn_141f61010 as *const (), Throws::Discard), // sub_141f61010
    Replacement::draft(0x141f61140, "unknown_141f61140", tpl_42203::<0x30, 0x14397ff58, 0x6d, 0x1412d0a20> as *const ()), // sub_141f61140
    Replacement::draft(0x141f61170, "unknown_141f61170", tpl_42204::<0x30, 0x14397ff58, 0x1412d0b70> as *const ()), // sub_141f61170
    Replacement::draft(0x141f611a0, "unknown_141f611a0", fn_141f611a0 as *const ()), // sub_141f611a0
    Replacement::draft_throws(0x141f611d0, "unknown_141f611d0", fn_141f611d0 as *const (), Throws::Caught), // sub_141f611d0
    Replacement::draft(0x141f61380, "unknown_141f61380", fn_141f61380 as *const ()), // sub_141f61380
    Replacement::draft_throws(0x141f613b0, "unknown_141f613b0", tpl_42201::<0xffffffffffffffff, 0x141f60c90, 0x30, 0x38, 0x74> as *const (), Throws::Discard), // sub_141f613b0
    Replacement::draft_throws(0x141f61400, "unknown_141f61400", fn_141f61400 as *const (), Throws::Discard), // sub_141f61400
    Replacement::draft(0x141f61540, "unknown_141f61540", fn_141f61540 as *const ()), // sub_141f61540
    Replacement::draft(0x141f61560, "unknown_141f61560", fn_141f61560 as *const ()), // sub_141f61560
    Replacement::draft_throws(0x141f61aa0, "unknown_141f61aa0", fn_141f61aa0 as *const (), Throws::Discard), // sub_141f61aa0
    Replacement::draft_throws(0x141f61b30, "unknown_141f61b30", fn_141f61b30 as *const (), Throws::Discard), // sub_141f61b30
    Replacement::draft(0x141f61b80, "unknown_141f61b80", fn_141f61b80 as *const ()), // sub_141f61b80
    Replacement::draft(0x141f61d00, "unknown_141f61d00", fn_141f61d00 as *const ()), // sub_141f61d00
    Replacement::draft(0x141f61e90, "unknown_141f61e90", tpl_37677::<0x20, 0x40, 0x50, 0x58> as *const ()), // sub_141f61e90
    Replacement::draft(0x141f61ed0, "unknown_141f61ed0", tpl_34693::<0x10, 0x18, 0x20> as *const ()), // sub_141f61ed0
    Replacement::draft(0x141f62380, "unknown_141f62380", tpl_37681::<0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x8, 0x10> as *const ()), // sub_141f62380
    Replacement::draft_throws(0x141f623e0, "unknown_141f623e0", tpl_1331::<0x8, 0x1410dba10, 0x14384eabc, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141f623e0
    Replacement::draft(0x141f625d0, "unknown_141f625d0", tpl_34696::<0x18, 0x8, 0x10, 0x20, 0x30, 0x28> as *const ()), // sub_141f625d0
    Replacement::draft_throws(0x141f62610, "unknown_141f62610", tpl_1331::<0x8, 0x1410db720, 0x14384eab8, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141f62610
    Replacement::draft(0x141f627e0, "unknown_141f627e0", tpl_37682::<0x18, 0x8, 0x10, 0x20, 0x40, 0x38, 0x60, 0x58> as *const ()), // sub_141f627e0
    Replacement::draft(0x141f62830, "unknown_141f62830", tpl_37683::<0x20, 0x18, 0x40, 0x60, 0x38, 0x8, 0x58, 0x10> as *const ()), // sub_141f62830
    Replacement::draft(0x141f62880, "unknown_141f62880", tpl_34697::<0x18, 0x20, 0x30, 0x8, 0x28, 0x10> as *const ()), // sub_141f62880
    Replacement::draft(0x141f62b20, "unknown_141f62b20", fn_141f62b20 as *const ()), // sub_141f62b20
    Replacement::draft(0x141f62b80, "unknown_141f62b80", fn_141f62b80 as *const ()), // sub_141f62b80
    Replacement::draft(0x141f62bb0, "unknown_141f62bb0", fn_141f62bb0 as *const ()), // sub_141f62bb0
    Replacement::draft(0x141f62d60, "unknown_141f62d60", fn_141f62d60 as *const ()), // sub_141f62d60
    Replacement::draft(0x141f62f50, "unknown_141f62f50", fn_141f62f50 as *const ()), // sub_141f62f50
    Replacement::draft_throws(0x141f631c0, "unknown_141f631c0", fn_141f631c0 as *const (), Throws::Caught), // sub_141f631c0
    Replacement::draft_throws(0x141f63f00, "unknown_141f63f00", fn_141f63f00 as *const (), Throws::Caught), // sub_141f63f00
    Replacement::draft(0x141f642e0, "unknown_141f642e0", fn_141f642e0 as *const ()), // sub_141f642e0
    Replacement::draft(0x141f64720, "unknown_141f64720", fn_141f64720 as *const ()), // sub_141f64720
    Replacement::draft_throws(0x141f648f0, "unknown_141f648f0", fn_141f648f0 as *const (), Throws::Caught), // sub_141f648f0
    Replacement::draft(0x141f64af0, "unknown_141f64af0", fn_141f64af0 as *const ()), // sub_141f64af0
    Replacement::draft_throws(0x141f64ba0, "unknown_141f64ba0", fn_141f64ba0 as *const (), Throws::Caught), // sub_141f64ba0
    Replacement::draft_throws(0x141f64d40, "unknown_141f64d40", fn_141f64d40 as *const (), Throws::Caught), // sub_141f64d40
    Replacement::draft(0x141f64db0, "unknown_141f64db0", fn_141f64db0 as *const ()), // sub_141f64db0
    Replacement::draft_throws(0x141f64e90, "unknown_141f64e90", fn_141f64e90 as *const (), Throws::Caught), // sub_141f64e90
    Replacement::draft_throws(0x141f65cc0, "unknown_141f65cc0", fn_141f65cc0 as *const (), Throws::Discard), // sub_141f65cc0
    Replacement::draft_throws(0x141f65e50, "unknown_141f65e50", fn_141f65e50 as *const (), Throws::Caught), // sub_141f65e50
    Replacement::draft_throws(0x141f65fe0, "unknown_141f65fe0", fn_141f65fe0 as *const (), Throws::Caught), // sub_141f65fe0
    Replacement::draft_throws(0x141f66200, "unknown_141f66200", fn_141f66200 as *const (), Throws::Caught), // sub_141f66200
    Replacement::draft(0x141f663d0, "unknown_141f663d0", fn_141f663d0 as *const ()), // sub_141f663d0
    Replacement::draft_throws(0x141f66470, "unknown_141f66470", fn_141f66470 as *const (), Throws::Caught), // sub_141f66470
    Replacement::draft_throws(0x141f66670, "unknown_141f66670", fn_141f66670 as *const (), Throws::Caught), // sub_141f66670
    Replacement::draft_throws(0x141f66b60, "unknown_141f66b60", fn_141f66b60 as *const (), Throws::Caught), // sub_141f66b60
    Replacement::draft_throws(0x141f66cb0, "unknown_141f66cb0", fn_141f66cb0 as *const (), Throws::Caught), // sub_141f66cb0
    Replacement::draft_throws(0x141f66e00, "unknown_141f66e00", fn_141f66e00 as *const (), Throws::Caught), // sub_141f66e00
    Replacement::draft_throws(0x141f67180, "unknown_141f67180", fn_141f67180 as *const (), Throws::Caught), // sub_141f67180
    Replacement::draft_throws(0x141f67620, "unknown_141f67620", fn_141f67620 as *const (), Throws::Caught), // sub_141f67620
    Replacement::draft_throws(0x141f67880, "unknown_141f67880", fn_141f67880 as *const (), Throws::Discard), // sub_141f67880
    Replacement::draft_throws(0x141f678b0, "unknown_141f678b0", fn_141f678b0 as *const (), Throws::Discard), // sub_141f678b0
    Replacement::draft(0x141f678d0, "unknown_141f678d0", tpl_42238::<0x143c48630, 0x143c48580, 0x143c484d0> as *const ()), // sub_141f678d0
    Replacement::draft(0x141f67950, "unknown_141f67950", fn_141f67950 as *const ()), // sub_141f67950
    Replacement::draft(0x141f679c0, "unknown_141f679c0", fn_141f679c0 as *const ()), // sub_141f679c0
    Replacement::draft(0x141f67a80, "unknown_141f67a80", fn_141f67a80 as *const ()), // sub_141f67a80
    Replacement::draft(0x141f67b10, "unknown_141f67b10", fn_141f67b10 as *const ()), // sub_141f67b10
    Replacement::draft(0x141f67ba0, "unknown_141f67ba0", fn_141f67ba0 as *const ()), // sub_141f67ba0
    Replacement::draft_throws(0x141f67c50, "unknown_141f67c50", fn_141f67c50 as *const (), Throws::Discard), // sub_141f67c50
    Replacement::draft(0x141f683a0, "unknown_141f683a0", tpl_34605::<0x143298af8> as *const ()), // sub_141f683a0
    Replacement::draft(0x141f686c0, "unknown_141f686c0", fn_141f686c0 as *const ()), // sub_141f686c0
    Replacement::draft_throws(0x141f68a10, "unknown_141f68a10", tpl_37972::<0x14387e060, 0x1410dcc70, 0x1402116e0, 0x142912b70, 0x1410d6ca0, 0x1410dc840, 0x143823ad8, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141f68a10
    Replacement::draft_throws(0x141f68bd0, "unknown_141f68bd0", tpl_42244::<0x143a34378, 0x1410ddaa0, 0x142442de0> as *const (), Throws::Discard), // sub_141f68bd0
    Replacement::draft_throws(0x141f68c50, "unknown_141f68c50", tpl_42244::<0x143a34378, 0x1410ddaa0, 0x142442de0> as *const (), Throws::Discard), // sub_141f68c50
    Replacement::draft_throws(0x141f68c90, "unknown_141f68c90", fn_141f68c90 as *const (), Throws::Discard), // sub_141f68c90
    Replacement::draft_throws(0x141f68d10, "unknown_141f68d10", fn_141f68d10 as *const (), Throws::Discard), // sub_141f68d10
    Replacement::draft_throws(0x141f68d80, "unknown_141f68d80", fn_141f68d80 as *const (), Throws::Discard), // sub_141f68d80
    Replacement::draft_throws(0x141f68e30, "unknown_141f68e30", fn_141f68e30 as *const (), Throws::Discard), // sub_141f68e30
    Replacement::draft(0x141f68ec0, "unknown_141f68ec0", tpl_97860::<0x143297570, 0x8, 0x18> as *const ()), // sub_141f68ec0
    Replacement::draft(0x141f68f70, "unknown_141f68f70", tpl_37868::<0x143297570, 0x8, 0x18> as *const ()), // sub_141f68f70
    Replacement::draft(0x141f69050, "unknown_141f69050", fn_141f69050 as *const ()), // sub_141f69050
    Replacement::draft(0x141f69160, "unknown_141f69160", fn_141f69160 as *const ()), // sub_141f69160
    Replacement::draft(0x141f69290, "unknown_141f69290", fn_141f69290 as *const ()), // sub_141f69290
    Replacement::draft(0x141f694a0, "unknown_141f694a0", fn_141f694a0 as *const ()), // sub_141f694a0
    Replacement::draft(0x141f695d0, "unknown_141f695d0", fn_141f695d0 as *const ()), // sub_141f695d0
    Replacement::draft(0x141f69b10, "unknown_141f69b10", fn_141f69b10 as *const ()), // sub_141f69b10
    Replacement::draft(0x141f6a370, "unknown_141f6a370", fn_141f6a370 as *const ()), // sub_141f6a370
    Replacement::draft(0x141f6abf0, "unknown_141f6abf0", fn_141f6abf0 as *const ()), // sub_141f6abf0
    Replacement::draft(0x141f6ac90, "unknown_141f6ac90", fn_141f6ac90 as *const ()), // sub_141f6ac90
    Replacement::draft(0x141f6ad30, "unknown_141f6ad30", fn_141f6ad30 as *const ()), // sub_141f6ad30
    Replacement::draft(0x141f6b020, "unknown_141f6b020", fn_141f6b020 as *const ()), // sub_141f6b020
    Replacement::draft(0x141f6b320, "unknown_141f6b320", tpl_35031::<0x142407c80, 0x143cdc620, 0x14020e8f0> as *const ()), // sub_141f6b320
    Replacement::draft(0x141f6b450, "unknown_141f6b450", tpl_35031::<0x142407c80, 0x143cdc650, 0x14020e8f0> as *const ()), // sub_141f6b450
    Replacement::draft(0x141f6b580, "unknown_141f6b580", fn_141f6b580 as *const ()), // sub_141f6b580
    Replacement::draft(0x141f6b7e0, "unknown_141f6b7e0", fn_141f6b7e0 as *const ()), // sub_141f6b7e0
    Replacement::draft(0x141f6bac0, "unknown_141f6bac0", fn_141f6bac0 as *const ()), // sub_141f6bac0
    Replacement::draft(0x141f6bbb0, "unknown_141f6bbb0", fn_141f6bbb0 as *const ()), // sub_141f6bbb0
    Replacement::draft_throws(0x141f6bc70, "unknown_141f6bc70", fn_141f6bc70 as *const (), Throws::Discard), // sub_141f6bc70
    Replacement::draft_throws(0x141f6c330, "unknown_141f6c330", tpl_42260::<0x8, 0x580f0, 0x1420b47d0, 0x40, 0x1420b3cb0> as *const (), Throws::Discard), // sub_141f6c330
    Replacement::draft_throws(0x141f6c380, "unknown_141f6c380", tpl_42260::<0x8, 0x580f0, 0x1420b47d0, 0x40, 0x1420b3cb0> as *const (), Throws::Discard), // sub_141f6c380
    Replacement::draft_throws(0x141f6c3d0, "unknown_141f6c3d0", tpl_42260::<0x8, 0x580f0, 0x1420b47d0, 0x40, 0x1420b3cb0> as *const (), Throws::Discard), // sub_141f6c3d0
    Replacement::draft_throws(0x141f6c420, "unknown_141f6c420", tpl_42260::<0x8, 0x580f0, 0x1420b47d0, 0x40, 0x1420b3cb0> as *const (), Throws::Discard), // sub_141f6c420
    Replacement::draft(0x141f6c470, "unknown_141f6c470", fn_141f6c470 as *const ()), // sub_141f6c470
    Replacement::draft(0x141f6cc70, "unknown_141f6cc70", fn_141f6cc70 as *const ()), // sub_141f6cc70
    Replacement::draft(0x141f6cd50, "unknown_141f6cd50", fn_141f6cd50 as *const ()), // sub_141f6cd50
    Replacement::draft(0x141f6cdb0, "unknown_141f6cdb0", tpl_41000 as *const ()), // sub_141f6cdb0
    Replacement::draft(0x141f6cdf0, "unknown_141f6cdf0", tpl_34994::<0x143cdd134, 0x143880fc0, 0x143880fb8, 0x141834a78, 0x142614b20, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f6cdf0
    Replacement::draft(0x141f6ce60, "unknown_141f6ce60", fn_141f6ce60 as *const ()), // sub_141f6ce60
    Replacement::draft(0x141f6cf10, "unknown_141f6cf10", fn_141f6cf10 as *const ()), // sub_141f6cf10
    Replacement::draft(0x141f6cf40, "unknown_141f6cf40", fn_141f6cf40 as *const ()), // sub_141f6cf40
    Replacement::draft(0x141f6cfd0, "unknown_141f6cfd0", tpl_34318 as *const ()), // sub_141f6cfd0
    Replacement::draft(0x141f6cff0, "unknown_141f6cff0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6cff0
    Replacement::draft(0x141f6d050, "unknown_141f6d050", tpl_34318 as *const ()), // sub_141f6d050
    Replacement::draft(0x141f6d070, "unknown_141f6d070", tpl_34318 as *const ()), // sub_141f6d070
    Replacement::draft(0x141f6d090, "unknown_141f6d090", tpl_34318 as *const ()), // sub_141f6d090
    Replacement::draft(0x141f6d0b0, "unknown_141f6d0b0", tpl_34318 as *const ()), // sub_141f6d0b0
    Replacement::draft(0x141f6d0d0, "unknown_141f6d0d0", tpl_34318 as *const ()), // sub_141f6d0d0
    Replacement::draft(0x141f6d0f0, "unknown_141f6d0f0", tpl_34318 as *const ()), // sub_141f6d0f0
    Replacement::draft(0x141f6d110, "unknown_141f6d110", tpl_34319::<0x143cdcdd8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d110
    Replacement::draft(0x141f6d170, "unknown_141f6d170", tpl_34319::<0x143cd65c4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d170
    Replacement::draft(0x141f6d1d0, "unknown_141f6d1d0", tpl_34319::<0x143cd848c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d1d0
    Replacement::draft(0x141f6d230, "unknown_141f6d230", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d230
    Replacement::draft(0x141f6d290, "unknown_141f6d290", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d290
    Replacement::draft(0x141f6d2f0, "unknown_141f6d2f0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d2f0
    Replacement::draft(0x141f6d350, "unknown_141f6d350", tpl_34318 as *const ()), // sub_141f6d350
    Replacement::draft(0x141f6d370, "unknown_141f6d370", tpl_34318 as *const ()), // sub_141f6d370
    Replacement::draft(0x141f6d390, "unknown_141f6d390", tpl_34318 as *const ()), // sub_141f6d390
    Replacement::draft(0x141f6d3b0, "unknown_141f6d3b0", tpl_34319::<0x143cd65d4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d3b0
    Replacement::draft(0x141f6d410, "unknown_141f6d410", tpl_34319::<0x143cd65b4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d410
    Replacement::draft(0x141f6d470, "unknown_141f6d470", tpl_34319::<0x143c9d1e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d470
    Replacement::draft(0x141f6d4d0, "unknown_141f6d4d0", tpl_34318 as *const ()), // sub_141f6d4d0
    Replacement::draft(0x141f6d4f0, "unknown_141f6d4f0", tpl_34318 as *const ()), // sub_141f6d4f0
    Replacement::draft(0x141f6d510, "unknown_141f6d510", tpl_34319::<0x143cdcf30, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d510
    Replacement::draft(0x141f6d570, "unknown_141f6d570", tpl_34319::<0x143c9a620, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f6d570
    Replacement::draft(0x141f6d5d0, "unknown_141f6d5d0", tpl_361 as *const ()), // sub_141f6d5d0
    Replacement::draft(0x141f6d5e0, "unknown_141f6d5e0", fn_141f6d5e0 as *const ()), // sub_141f6d5e0
    Replacement::draft(0x141f6d770, "unknown_141f6d770", fn_141f6d770 as *const ()), // sub_141f6d770
    Replacement::draft(0x141f6de00, "unknown_141f6de00", tpl_361 as *const ()), // sub_141f6de00
    Replacement::draft(0x141f6de10, "unknown_141f6de10", fn_141f6de10 as *const ()), // sub_141f6de10
    Replacement::draft(0x141f6dee0, "unknown_141f6dee0", tpl_361 as *const ()), // sub_141f6dee0
    Replacement::draft(0x141f6def0, "unknown_141f6def0", fn_141f6def0 as *const ()), // sub_141f6def0
    Replacement::draft(0x141f6dfb0, "unknown_141f6dfb0", tpl_35031::<0x142407c80, 0x143cdcdd8, 0x14020e8f0> as *const ()), // sub_141f6dfb0
    Replacement::draft(0x141f6e0d0, "unknown_141f6e0d0", tpl_496 as *const ()), // sub_141f6e0d0
    Replacement::draft_throws(0x141f6e0e0, "unknown_141f6e0e0", fn_141f6e0e0 as *const (), Throws::Caught), // sub_141f6e0e0
    Replacement::draft(0x141f6e350, "unknown_141f6e350", tpl_361 as *const ()), // sub_141f6e350
    Replacement::draft(0x141f6e360, "unknown_141f6e360", tpl_41861::<0x10, 0x14> as *const ()), // sub_141f6e360
    Replacement::draft(0x141f6e3c0, "unknown_141f6e3c0", tpl_361 as *const ()), // sub_141f6e3c0
    Replacement::draft(0x141f6e3d0, "unknown_141f6e3d0", fn_141f6e3d0 as *const ()), // sub_141f6e3d0
    Replacement::draft_throws(0x141f6e450, "unknown_141f6e450", fn_141f6e450 as *const (), Throws::Caught), // sub_141f6e450
    Replacement::draft(0x141f6e870, "unknown_141f6e870", tpl_41864 as *const ()), // sub_141f6e870
    Replacement::draft_throws(0x141f6e8a0, "unknown_141f6e8a0", fn_141f6e8a0 as *const (), Throws::Discard), // sub_141f6e8a0
    Replacement::draft(0x141f6edb0, "unknown_141f6edb0", tpl_369 as *const ()), // sub_141f6edb0
    Replacement::draft(0x141f6edc0, "unknown_141f6edc0", fn_141f6edc0 as *const ()), // sub_141f6edc0
    Replacement::draft(0x141f6ede0, "unknown_141f6ede0", tpl_101994::<0x143c8ce94, 0x143879674, 0x143cc5814, 0x1434268d0, 0x143879660, 0x1438548d8, 0x143426574, 0x143426a40> as *const ()), // sub_141f6ede0
    Replacement::draft(0x141f6ef30, "unknown_141f6ef30", fn_141f6ef30 as *const ()), // sub_141f6ef30
    Replacement::draft(0x141f6f060, "unknown_141f6f060", fn_141f6f060 as *const ()), // sub_141f6f060
    Replacement::draft(0x141f6f9d0, "unknown_141f6f9d0", tpl_42275::<0x3, 0x4, 0x8, 0x14240aa00, 0x108> as *const ()), // sub_141f6f9d0
    Replacement::draft(0x141f6fa30, "unknown_141f6fa30", tpl_42275::<0x3, 0x4, 0x8, 0x14240a730, 0x108> as *const ()), // sub_141f6fa30
    Replacement::draft(0x141f6fa90, "unknown_141f6fa90", tpl_42276::<0x141834d30> as *const ()), // sub_141f6fa90
    Replacement::draft(0x141f6fab0, "unknown_141f6fab0", tpl_42276::<0x141834d30> as *const ()), // sub_141f6fab0
    Replacement::draft(0x141f6fad0, "unknown_141f6fad0", tpl_1742::<0x14240ab10> as *const ()), // sub_141f6fad0
    Replacement::draft(0x141f6fae0, "unknown_141f6fae0", tpl_40051::<0x14240ab10> as *const ()), // sub_141f6fae0
    Replacement::draft_throws(0x141f6fb10, "std_141f6fb10", fn_141f6fb10 as *const (), Throws::Caught), // std::_Func_impl_no_alloc<coregame::FilePrecache::precacheFile::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_allo
    Replacement::draft(0x141f6fba0, "std_141f6fba0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<coregame::FilePrecache::precacheFile::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_allo
    Replacement::draft(0x141f6fbb0, "std_141f6fbb0", fn_141f6fbb0 as *const ()), // std::_Func_impl_no_alloc<coregame::FilePrecache::precacheFile::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_allo
    Replacement::draft(0x141f6fca0, "std_141f6fca0", tpl_1::<0x143890540> as *const ()), // std::_Func_impl_no_alloc<coregame::FilePrecache::precacheFile::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_allo
    Replacement::draft(0x141f6fcb0, "std_141f6fcb0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::FilePrecache::precacheFile::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_allo
    Replacement::draft(0x141f6fcc0, "std_141f6fcc0", fn_141f6fcc0 as *const ()), // std::_Func_impl_no_alloc<coregame::FilePrecache::precacheFile::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_allo
    Replacement::draft(0x141f6fd00, "unknown_141f6fd00", tpl_361 as *const ()), // sub_141f6fd00
    Replacement::draft(0x141f6fd10, "unknown_141f6fd10", tpl_361 as *const ()), // sub_141f6fd10
    Replacement::draft_throws(0x141f6fd20, "unknown_141f6fd20", fn_141f6fd20 as *const (), Throws::Caught), // sub_141f6fd20
    Replacement::draft(0x141f6feb0, "unknown_141f6feb0", tpl_1742::<0x14240ab10> as *const ()), // sub_141f6feb0
    Replacement::draft(0x141f6fec0, "coregame_141f6fec0", tpl_27243::<0x14240ab10, 0x141834d30> as *const ()), // coregame::FilePrecache::precacheAllShaders::ShaderFinder::coregame::FilePrecache::precacheAllShaders::ShaderFinder::vfun
    Replacement::draft_throws(0x141f6ff00, "coregame_141f6ff00", fn_141f6ff00 as *const (), Throws::Discard), // coregame::FilePrecache::precacheAllShaders::ShaderFinder::coregame::FilePrecache::precacheAllShaders::ShaderFinder::vfun
    Replacement::draft(0x141f70070, "coregame_141f70070", fn_141f70070 as *const ()), // sub_141f70070
    Replacement::draft(0x141f700f0, "coregame_141f700f0", fn_141f700f0 as *const ()), // sub_141f700f0
    Replacement::draft(0x141f70150, "coregame_141f70150", fn_141f70150 as *const ()), // sub_141f70150
    Replacement::draft_throws(0x141f70230, "coregame_141f70230", fn_141f70230 as *const (), Throws::Caught), // coregame::FilePrecache::coregame::FilePrecache::vfunc_2
    Replacement::draft_throws(0x141f70380, "coregame_141f70380", fn_141f70380 as *const (), Throws::Caught), // coregame::FilePrecache::coregame::FilePrecache::vfunc_0
    Replacement::draft(0x141f70970, "coregame_141f70970", fn_141f70970 as *const ()), // sub_141f70970
    Replacement::draft_throws(0x141f70a30, "coregame_141f70a30", fn_141f70a30 as *const (), Throws::Caught), // sub_141f70a30
    Replacement::draft_throws(0x141f70db0, "coregame_141f70db0", fn_141f70db0 as *const (), Throws::Caught), // sub_141f70db0
    Replacement::draft_throws(0x141f712b0, "coregame_141f712b0", fn_141f712b0 as *const (), Throws::Caught), // sub_141f712b0
    Replacement::draft(0x141f713e0, "coregame_141f713e0", fn_141f713e0 as *const ()), // sub_141f713e0
    Replacement::draft(0x141f71430, "coregame_141f71430", fn_141f71430 as *const ()), // sub_141f71430
    Replacement::draft(0x141f71470, "coregame_141f71470", fn_141f71470 as *const ()), // sub_141f71470
    Replacement::draft_throws(0x141f717f0, "coregame_141f717f0", fn_141f717f0 as *const (), Throws::Caught), // sub_141f717f0
    Replacement::draft_throws(0x141f71ae0, "coregame_141f71ae0", fn_141f71ae0 as *const (), Throws::Caught), // sub_141f71ae0
    Replacement::draft(0x141f71e20, "coregame_141f71e20", tpl_20883::<0x8, 0x1410eee00> as *const ()), // sub_141f71e20
    Replacement::draft_throws(0x141f71e70, "coregame_141f71e70", tpl_35009::<0x14387e2f0, 0x1410ef130, 0x1402116e0> as *const (), Throws::Discard), // sub_141f71e70
    Replacement::draft_throws(0x141f72510, "coregame_141f72510", fn_141f72510 as *const (), Throws::Caught), // sub_141f72510
    Replacement::draft_throws(0x141f72a00, "coregame_141f72a00", fn_141f72a00 as *const (), Throws::Caught), // sub_141f72a00
    Replacement::draft_throws(0x141f72bd0, "coregame_141f72bd0", fn_141f72bd0 as *const (), Throws::Discard), // sub_141f72bd0
    Replacement::draft_throws(0x141f73090, "coregame_141f73090", fn_141f73090 as *const (), Throws::Caught), // sub_141f73090
    Replacement::draft_throws(0x141f73350, "coregame_141f73350", fn_141f73350 as *const (), Throws::Discard), // sub_141f73350
    Replacement::draft_throws(0x141f73400, "coregame_141f73400", fn_141f73400 as *const (), Throws::Caught), // sub_141f73400
    Replacement::draft(0x141f73670, "coregame_141f73670", fn_141f73670 as *const ()), // sub_141f73670
    Replacement::draft(0x141f73730, "coregame_141f73730", fn_141f73730 as *const ()), // sub_141f73730
    Replacement::draft(0x141f738b0, "coregame_141f738b0", fn_141f738b0 as *const ()), // sub_141f738b0
    Replacement::draft(0x141f73930, "coregame_141f73930", fn_141f73930 as *const ()), // sub_141f73930
    Replacement::draft(0x141f73960, "coregame_141f73960", fn_141f73960 as *const ()), // sub_141f73960
    Replacement::draft_throws(0x141f739f0, "coregame_141f739f0", fn_141f739f0 as *const (), Throws::Discard), // sub_141f739f0
    Replacement::draft(0x141f73aa0, "coregame_141f73aa0", fn_141f73aa0 as *const ()), // sub_141f73aa0
    Replacement::draft(0x141f73ad0, "coregame_141f73ad0", fn_141f73ad0 as *const ()), // sub_141f73ad0
    Replacement::draft(0x141f741d0, "coregame_141f741d0", fn_141f741d0 as *const ()), // sub_141f741d0
    Replacement::draft(0x141f74200, "coregame_141f74200", fn_141f74200 as *const ()), // sub_141f74200
    Replacement::draft(0x141f74290, "coregame_141f74290", fn_141f74290 as *const ()), // sub_141f74290
    Replacement::draft(0x141f74320, "coregame_141f74320", fn_141f74320 as *const ()), // sub_141f74320
    Replacement::draft(0x141f74360, "coregame_141f74360", fn_141f74360 as *const ()), // sub_141f74360
    Replacement::draft(0x141f74440, "coregame_141f74440", tpl_42316::<0x10b, 0x110, 0x108, 0x10c, 0x8, 0x1426d01e0> as *const ()), // sub_141f74440
    Replacement::draft(0x141f744e0, "coregame_141f744e0", fn_141f744e0 as *const ()), // sub_141f744e0
    Replacement::draft(0x141f74600, "coregame_141f74600", tpl_42316::<0x10b, 0x110, 0x108, 0x10c, 0x8, 0x1426d01e0> as *const ()), // sub_141f74600
    Replacement::draft(0x141f746a0, "coregame_141f746a0", fn_141f746a0 as *const ()), // sub_141f746a0
    Replacement::draft_throws(0x141f746c0, "coregame_141f746c0", fn_141f746c0 as *const (), Throws::Discard), // sub_141f746c0
    Replacement::draft(0x141f74950, "coregame_141f74950", fn_141f74950 as *const ()), // sub_141f74950
    Replacement::draft_throws(0x141f74ac0, "coregame_141f74ac0", fn_141f74ac0 as *const (), Throws::Caught), // sub_141f74ac0
    Replacement::draft(0x141f74d50, "coregame_141f74d50", fn_141f74d50 as *const ()), // sub_141f74d50
    Replacement::draft(0x141f74d80, "coregame_141f74d80", fn_141f74d80 as *const ()), // sub_141f74d80
    Replacement::draft(0x141f74e00, "coregame_141f74e00", fn_141f74e00 as *const ()), // sub_141f74e00
    Replacement::draft(0x141f74ed0, "coregame_141f74ed0", fn_141f74ed0 as *const ()), // sub_141f74ed0
    Replacement::draft(0x141f74f20, "coregame_141f74f20", fn_141f74f20 as *const ()), // sub_141f74f20
    Replacement::draft_throws(0x141f75070, "coregame_141f75070", fn_141f75070 as *const (), Throws::Discard), // sub_141f75070
    Replacement::draft(0x141f75180, "coregame_141f75180", fn_141f75180 as *const ()), // sub_141f75180
    Replacement::draft_throws(0x141f75220, "coregame_141f75220", fn_141f75220 as *const (), Throws::Caught), // sub_141f75220
    Replacement::draft_throws(0x141f75500, "coregame_141f75500", fn_141f75500 as *const (), Throws::Discard), // sub_141f75500
    Replacement::draft(0x141f75740, "coregame_141f75740", tpl_30459::<0x2a0> as *const ()), // sub_141f75740
    Replacement::draft(0x141f75760, "coregame_141f75760", fn_141f75760 as *const ()), // sub_141f75760
    Replacement::draft(0x141f75780, "coregame_141f75780", fn_141f75780 as *const ()), // sub_141f75780
    Replacement::draft(0x141f757b0, "coregame_141f757b0", fn_141f757b0 as *const ()), // sub_141f757b0
    Replacement::draft(0x141f757f0, "coregame_141f757f0", fn_141f757f0 as *const ()), // sub_141f757f0
    Replacement::draft(0x141f75840, "coregame_141f75840", fn_141f75840 as *const ()), // sub_141f75840
    Replacement::draft(0x141f75890, "coregame_141f75890", fn_141f75890 as *const ()), // sub_141f75890
    Replacement::draft(0x141f75920, "coregame_141f75920", fn_141f75920 as *const ()), // sub_141f75920
    Replacement::draft(0x141f759f0, "coregame_141f759f0", fn_141f759f0 as *const ()), // sub_141f759f0
    Replacement::draft(0x141f75a50, "coregame_141f75a50", fn_141f75a50 as *const ()), // sub_141f75a50
    Replacement::draft(0x141f75ae0, "coregame_141f75ae0", fn_141f75ae0 as *const ()), // sub_141f75ae0
    Replacement::draft_throws(0x141f75be0, "coregame_141f75be0", fn_141f75be0 as *const (), Throws::Caught), // sub_141f75be0
    Replacement::draft_throws(0x141f76000, "coregame_141f76000", fn_141f76000 as *const (), Throws::Caught), // sub_141f76000
    Replacement::draft(0x141f763b0, "coregame_141f763b0", fn_141f763b0 as *const ()), // sub_141f763b0
    Replacement::draft(0x141f764c0, "coregame_141f764c0", fn_141f764c0 as *const ()), // sub_141f764c0
    Replacement::draft_throws(0x141f76520, "coregame_141f76520", fn_141f76520 as *const (), Throws::Caught), // sub_141f76520
    Replacement::draft(0x141f76a20, "coregame_141f76a20", fn_141f76a20 as *const ()), // sub_141f76a20
    Replacement::draft(0x141f76a90, "coregame_141f76a90", fn_141f76a90 as *const ()), // sub_141f76a90
    Replacement::draft(0x141f76ad0, "coregame_141f76ad0", fn_141f76ad0 as *const ()), // sub_141f76ad0
    Replacement::draft(0x141f76cc0, "coregame_141f76cc0", fn_141f76cc0 as *const ()), // sub_141f76cc0
    Replacement::draft_throws(0x141f76d40, "coregame_141f76d40", fn_141f76d40 as *const (), Throws::Caught), // sub_141f76d40
    Replacement::draft(0x141f76de0, "coregame_141f76de0", fn_141f76de0 as *const ()), // sub_141f76de0
    Replacement::draft(0x141f76f00, "coregame_141f76f00", fn_141f76f00 as *const ()), // sub_141f76f00
    Replacement::draft(0x141f77030, "coregame_141f77030", fn_141f77030 as *const ()), // coregame::Episode::updateState::StreamJob_UnloadEpisode::coregame::Episode::updateState::StreamJob_UnloadEpisode::vfunc_
    Replacement::draft(0x141f77190, "coregame_141f77190", fn_141f77190 as *const ()), // coregame::Episode::updateState::StreamJob_UnloadEpisode::coregame::Episode::updateState::StreamJob_UnloadEpisode::vfunc_
    Replacement::draft(0x141f77240, "coregame_141f77240", fn_141f77240 as *const ()), // sub_141f77240
    Replacement::draft(0x141f77280, "coregame_141f77280", fn_141f77280 as *const ()), // sub_141f77280
    Replacement::draft(0x141f772a0, "coregame_141f772a0", fn_141f772a0 as *const ()), // sub_141f772a0
    Replacement::draft(0x141f77320, "coregame_141f77320", fn_141f77320 as *const ()), // sub_141f77320
    Replacement::draft(0x141f773a0, "coregame_141f773a0", fn_141f773a0 as *const ()), // sub_141f773a0
    Replacement::draft(0x141f77420, "coregame_141f77420", fn_141f77420 as *const ()), // sub_141f77420
    Replacement::draft(0x141f774a0, "coregame_141f774a0", fn_141f774a0 as *const ()), // sub_141f774a0
    Replacement::draft(0x141f77520, "coregame_141f77520", fn_141f77520 as *const ()), // sub_141f77520
    Replacement::draft(0x141f775b0, "coregame_141f775b0", fn_141f775b0 as *const ()), // sub_141f775b0
    Replacement::draft(0x141f77ab0, "coregame_141f77ab0", fn_141f77ab0 as *const ()), // sub_141f77ab0
    Replacement::draft(0x141f77bb0, "coregame_141f77bb0", fn_141f77bb0 as *const ()), // sub_141f77bb0
    Replacement::draft(0x141f77c00, "coregame_141f77c00", fn_141f77c00 as *const ()), // sub_141f77c00
    Replacement::draft(0x141f77d40, "coregame_141f77d40", fn_141f77d40 as *const ()), // sub_141f77d40
    Replacement::draft(0x141f77e10, "coregame_141f77e10", fn_141f77e10 as *const ()), // sub_141f77e10
    Replacement::draft(0x141f77eb0, "coregame_141f77eb0", fn_141f77eb0 as *const ()), // sub_141f77eb0
    Replacement::draft_throws(0x141f77fb0, "coregame_141f77fb0", fn_141f77fb0 as *const (), Throws::Caught), // sub_141f77fb0
    Replacement::draft(0x141f78170, "coregame_141f78170", fn_141f78170 as *const ()), // sub_141f78170
    Replacement::draft(0x141f781f0, "coregame_141f781f0", fn_141f781f0 as *const ()), // sub_141f781f0
    Replacement::draft(0x141f78220, "coregame_141f78220", fn_141f78220 as *const ()), // sub_141f78220
    Replacement::draft(0x141f78280, "coregame_141f78280", fn_141f78280 as *const ()), // sub_141f78280
    Replacement::draft(0x141f782e0, "coregame_141f782e0", fn_141f782e0 as *const ()), // sub_141f782e0
    Replacement::draft(0x141f78320, "coregame_141f78320", tpl_957::<0x198, 0x58> as *const ()), // sub_141f78320
    Replacement::draft(0x141f78340, "coregame_141f78340", fn_141f78340 as *const ()), // sub_141f78340
    Replacement::draft_throws(0x141f78470, "coregame_141f78470", fn_141f78470 as *const (), Throws::Caught), // sub_141f78470
    Replacement::draft(0x141f78b60, "coregame_141f78b60", tpl_38026::<0x198, 0xe8> as *const ()), // sub_141f78b60
    Replacement::draft_throws(0x141f78b90, "coregame_141f78b90", fn_141f78b90 as *const (), Throws::Caught), // sub_141f78b90
    Replacement::draft_throws(0x141f78c40, "coregame_141f78c40", fn_141f78c40 as *const (), Throws::Discard), // sub_141f78c40
    Replacement::draft_throws(0x141f78cf0, "coregame_141f78cf0", fn_141f78cf0 as *const (), Throws::Discard), // sub_141f78cf0
    Replacement::draft_throws(0x141f78d50, "unknown_141f78d50", fn_141f78d50 as *const (), Throws::Caught), // sub_141f78d50
    Replacement::draft_throws(0x141f79190, "unknown_141f79190", fn_141f79190 as *const (), Throws::Discard), // sub_141f79190
    Replacement::draft_throws(0x141f791e0, "unknown_141f791e0", fn_141f791e0 as *const (), Throws::Caught), // sub_141f791e0
    Replacement::draft(0x141f797d0, "unknown_141f797d0", fn_141f797d0 as *const ()), // sub_141f797d0
    Replacement::draft_throws(0x141f79930, "unknown_141f79930", tpl_42379::<0x141f78d50> as *const (), Throws::Discard), // sub_141f79930
    Replacement::draft_throws(0x141f79950, "unknown_141f79950", fn_141f79950 as *const (), Throws::Discard), // sub_141f79950
    Replacement::draft(0x141f79990, "unknown_141f79990", fn_141f79990 as *const ()), // sub_141f79990
    Replacement::draft(0x141f79a50, "unknown_141f79a50", tpl_34318 as *const ()), // sub_141f79a50
    Replacement::draft(0x141f79a70, "unknown_141f79a70", tpl_34318 as *const ()), // sub_141f79a70
    Replacement::draft(0x141f79a90, "unknown_141f79a90", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f79a90
    Replacement::draft(0x141f79af0, "unknown_141f79af0", tpl_34319::<0x143c8d108, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141f79af0
    Replacement::draft(0x141f79b50, "unknown_141f79b50", tpl_361 as *const ()), // sub_141f79b50
    Replacement::draft(0x141f79b60, "luau_141f79b60", fn_141f79b60 as *const ()), // sub_141f79b60
    Replacement::draft(0x141f79f70, "unknown_141f79f70", fn_141f79f70 as *const ()), // sub_141f79f70
    Replacement::draft_throws(0x141f7a3b0, "coregame_141f7a3b0", fn_141f7a3b0 as *const (), Throws::Caught), // coregame::EntityContentLoader::coregame::EntityContentLoader::vfunc_1
    Replacement::draft_throws(0x141f7a8d0, "coregame_141f7a8d0", fn_141f7a8d0 as *const (), Throws::Discard), // coregame::EntityContentLoader::coregame::EntityContentLoader::vfunc_2
    Replacement::draft_throws(0x141f7a940, "unknown_141f7a940", tpl_42387::<0x141f7b400> as *const (), Throws::Discard), // sub_141f7a940
    Replacement::draft_throws(0x141f7a970, "unknown_141f7a970", fn_141f7a970 as *const (), Throws::Caught), // sub_141f7a970
    Replacement::draft_throws(0x141f7af70, "unknown_141f7af70", tpl_42387::<0x141f7b400> as *const (), Throws::Discard), // sub_141f7af70
    Replacement::draft_throws(0x141f7afa0, "unknown_141f7afa0", fn_141f7afa0 as *const (), Throws::Discard), // sub_141f7afa0
    Replacement::draft(0x141f7b100, "unknown_141f7b100", fn_141f7b100 as *const ()), // sub_141f7b100
    Replacement::draft_throws(0x141f7b400, "unknown_141f7b400", fn_141f7b400 as *const (), Throws::Caught), // sub_141f7b400
    Replacement::draft(0x141f7b6e0, "unknown_141f7b6e0", fn_141f7b6e0 as *const ()), // sub_141f7b6e0
    Replacement::draft_throws(0x141f7b980, "unknown_141f7b980", fn_141f7b980 as *const (), Throws::Caught), // sub_141f7b980
    Replacement::draft_throws(0x141f7bb20, "unknown_141f7bb20", fn_141f7bb20 as *const (), Throws::Caught), // sub_141f7bb20
    Replacement::draft_throws(0x141f7bcc0, "unknown_141f7bcc0", fn_141f7bcc0 as *const (), Throws::Caught), // sub_141f7bcc0
    Replacement::draft_throws(0x141f7be10, "unknown_141f7be10", fn_141f7be10 as *const (), Throws::Discard), // sub_141f7be10
    Replacement::draft_throws(0x141f7bf70, "unknown_141f7bf70", fn_141f7bf70 as *const (), Throws::Caught), // sub_141f7bf70
    Replacement::draft_throws(0x141f7c130, "unknown_141f7c130", fn_141f7c130 as *const (), Throws::Caught), // sub_141f7c130
    Replacement::draft(0x141f7c390, "unknown_141f7c390", fn_141f7c390 as *const ()), // sub_141f7c390
    Replacement::draft(0x141f7cf00, "unknown_141f7cf00", fn_141f7cf00 as *const ()), // sub_141f7cf00
    Replacement::draft(0x141f7cf30, "unknown_141f7cf30", tpl_19620::<0x8, 0x141f7ca20> as *const ()), // sub_141f7cf30
    Replacement::draft(0x141f7cf90, "unknown_141f7cf90", tpl_3188::<0x1410f53d0, 0x14339a868, 0x141f7ca20, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141f7cf90
    Replacement::draft(0x141f7d010, "unknown_141f7d010", fn_141f7d010 as *const ()), // sub_141f7d010
    Replacement::draft_throws(0x141f7d080, "unknown_141f7d080", tpl_1331::<0x8, 0x1410f5470, 0x14384ee58, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141f7d080
    Replacement::draft(0x141f7d270, "unknown_141f7d270", tpl_34696::<0x18, 0x8, 0x10, 0x20, 0x30, 0x28> as *const ()), // sub_141f7d270
    Replacement::draft(0x141f7d2b0, "unknown_141f7d2b0", fn_141f7d2b0 as *const ()), // sub_141f7d2b0
    Replacement::draft_throws(0x141f7d320, "unknown_141f7d320", tpl_1330::<0x8, 0x1410f5230, 0x14384ee5c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_141f7d320
    Replacement::draft(0x141f7d410, "unknown_141f7d410", tpl_34697::<0x18, 0x20, 0x30, 0x8, 0x28, 0x10> as *const ()), // sub_141f7d410
    Replacement::draft(0x141f7d7e0, "unknown_141f7d7e0", tpl_42410::<0x18, 0x20, 0x28, 0x30, 0x8, 0x10> as *const ()), // sub_141f7d7e0
    Replacement::draft(0x141f7d8f0, "unknown_141f7d8f0", fn_141f7d8f0 as *const ()), // sub_141f7d8f0
    Replacement::draft(0x141f7da00, "unknown_141f7da00", fn_141f7da00 as *const ()), // sub_141f7da00
    Replacement::draft(0x141f7daa0, "unknown_141f7daa0", tpl_3694::<0x1401e7c10, 0x1410f3220> as *const ()), // sub_141f7daa0
    Replacement::draft(0x141f7dbf0, "unknown_141f7dbf0", fn_141f7dbf0 as *const ()), // sub_141f7dbf0
    Replacement::draft(0x141f7dc50, "unknown_141f7dc50", fn_141f7dc50 as *const ()), // sub_141f7dc50
    Replacement::draft(0x141f7dc70, "unknown_141f7dc70", fn_141f7dc70 as *const ()), // sub_141f7dc70
    Replacement::draft(0x141f7dca0, "unknown_141f7dca0", fn_141f7dca0 as *const ()), // sub_141f7dca0
    Replacement::draft(0x141f7dcc0, "unknown_141f7dcc0", fn_141f7dcc0 as *const ()), // sub_141f7dcc0
    Replacement::draft(0x141f7dcf0, "unknown_141f7dcf0", tpl_42418::<0x8, 0x580e0, 0x143c8cef8, 0x2c> as *const ()), // sub_141f7dcf0
    Replacement::draft(0x141f7dd40, "unknown_141f7dd40", fn_141f7dd40 as *const ()), // sub_141f7dd40
    Replacement::draft(0x141f7dd90, "unknown_141f7dd90", fn_141f7dd90 as *const ()), // sub_141f7dd90
    Replacement::draft(0x141f7dee0, "unknown_141f7dee0", fn_141f7dee0 as *const ()), // sub_141f7dee0
    Replacement::draft(0x141f7e0c0, "unknown_141f7e0c0", fn_141f7e0c0 as *const ()), // sub_141f7e0c0
    Replacement::draft_throws(0x141f7e3c0, "unknown_141f7e3c0", fn_141f7e3c0 as *const (), Throws::Caught), // sub_141f7e3c0
    Replacement::draft(0x141f7ea00, "unknown_141f7ea00", fn_141f7ea00 as *const ()), // sub_141f7ea00
    Replacement::draft_throws(0x141f7f010, "unknown_141f7f010", fn_141f7f010 as *const (), Throws::Caught), // sub_141f7f010
    Replacement::draft_throws(0x141f7f1c0, "unknown_141f7f1c0", fn_141f7f1c0 as *const (), Throws::Caught), // sub_141f7f1c0
    Replacement::draft_throws(0x141f7f380, "unknown_141f7f380", fn_141f7f380 as *const (), Throws::Caught), // sub_141f7f380
    Replacement::draft_throws(0x141f7f9b0, "unknown_141f7f9b0", fn_141f7f9b0 as *const (), Throws::Caught), // sub_141f7f9b0
    Replacement::draft_throws(0x141f7fb30, "unknown_141f7fb30", fn_141f7fb30 as *const (), Throws::Caught), // sub_141f7fb30
    Replacement::draft(0x141f7fd30, "unknown_141f7fd30", tpl_42238::<0x143c48760, 0x143c486d0, 0x143c48640> as *const ()), // sub_141f7fd30
    Replacement::draft(0x141f7fd60, "unknown_141f7fd60", fn_141f7fd60 as *const ()), // sub_141f7fd60
    Replacement::draft_throws(0x141f7fdb0, "unknown_141f7fdb0", fn_141f7fdb0 as *const (), Throws::Discard), // sub_141f7fdb0
    Replacement::draft(0x141f800b0, "unknown_141f800b0", tpl_29744::<0x8> as *const ()), // sub_141f800b0
    Replacement::draft(0x141f800c0, "unknown_141f800c0", tpl_42432::<0x8> as *const ()), // sub_141f800c0
    Replacement::draft(0x141f800d0, "unknown_141f800d0", tpl_29744::<0x10> as *const ()), // sub_141f800d0
    Replacement::draft(0x141f80110, "unknown_141f80110", fn_141f80110 as *const ()), // sub_141f80110
    Replacement::draft(0x141f80160, "unknown_141f80160", tpl_42432::<0x10> as *const ()), // sub_141f80160
    Replacement::draft_throws(0x141f80170, "unknown_141f80170", tpl_42434::<0x18, 0x10, 0x140200070, 0x141f80194> as *const (), Throws::Discard), // sub_141f80170
    Replacement::draft(0x141f801a0, "unknown_141f801a0", fn_141f801a0 as *const ()), // sub_141f801a0
    Replacement::draft_throws(0x141f801f0, "unknown_141f801f0", fn_141f801f0 as *const (), Throws::Discard), // sub_141f801f0
    Replacement::draft_throws(0x141f80220, "unknown_141f80220", tpl_42437::<0x18, 0x10, 0x140200070, 0x141f80244> as *const (), Throws::Discard), // sub_141f80220
    Replacement::draft(0x141f80270, "unknown_141f80270", tpl_40083::<0x18> as *const ()), // sub_141f80270
    Replacement::draft_throws(0x141f80280, "unknown_141f80280", fn_141f80280 as *const (), Throws::Discard), // sub_141f80280
    Replacement::draft(0x141f802f0, "unknown_141f802f0", tpl_42439::<0x18> as *const ()), // sub_141f802f0
    Replacement::draft(0x141f80320, "unknown_141f80320", tpl_42440::<0x10> as *const ()), // sub_141f80320
    Replacement::draft(0x141f80340, "unknown_141f80340", fn_141f80340 as *const ()), // sub_141f80340
    Replacement::draft(0x141f80360, "unknown_141f80360", fn_141f80360 as *const ()), // sub_141f80360
    Replacement::draft(0x141f80410, "unknown_141f80410", tpl_42443::<0x8, 0x10, 0x18> as *const ()), // sub_141f80410
    Replacement::draft(0x141f80480, "unknown_141f80480", fn_141f80480 as *const ()), // sub_141f80480
    Replacement::draft(0x141f804c0, "unknown_141f804c0", tpl_21554::<0x28> as *const ()), // sub_141f804c0
    Replacement::draft(0x141f804e0, "unknown_141f804e0", fn_141f804e0 as *const ()), // sub_141f804e0
    Replacement::draft(0x141f80510, "unknown_141f80510", fn_141f80510 as *const ()), // sub_141f80510
    Replacement::draft(0x141f80540, "unknown_141f80540", fn_141f80540 as *const ()), // sub_141f80540
    Replacement::draft(0x141f80560, "unknown_141f80560", fn_141f80560 as *const ()), // sub_141f80560
    Replacement::draft(0x141f805f0, "unknown_141f805f0", tpl_42449 as *const ()), // sub_141f805f0
    Replacement::draft(0x141f80610, "unknown_141f80610", tpl_42450::<0x8, 0x10> as *const ()), // sub_141f80610
    Replacement::draft(0x141f80630, "unknown_141f80630", fn_141f80630 as *const ()), // sub_141f80630
    Replacement::draft(0x141f80710, "unknown_141f80710", tpl_41912::<0x8> as *const ()), // sub_141f80710
    Replacement::draft(0x141f80750, "unknown_141f80750", tpl_42452::<0x8> as *const ()), // sub_141f80750
    Replacement::draft(0x141f807a0, "unknown_141f807a0", fn_141f807a0 as *const ()), // sub_141f807a0
    Replacement::draft(0x141f807d0, "unknown_141f807d0", tpl_21554::<0x18> as *const ()), // sub_141f807d0
    Replacement::draft(0x141f807f0, "unknown_141f807f0", fn_141f807f0 as *const ()), // sub_141f807f0
    Replacement::draft(0x141f80800, "unknown_141f80800", tpl_42455::<0x8, 0x10> as *const ()), // sub_141f80800
    Replacement::draft(0x141f80850, "unknown_141f80850", fn_141f80850 as *const ()), // sub_141f80850
    Replacement::draft_throws(0x141f808a0, "unknown_141f808a0", fn_141f808a0 as *const (), Throws::Discard), // sub_141f808a0
    Replacement::draft(0x141f80b60, "unknown_141f80b60", fn_141f80b60 as *const ()), // sub_141f80b60
    Replacement::draft(0x141f80be0, "unknown_141f80be0", fn_141f80be0 as *const ()), // sub_141f80be0
    Replacement::draft(0x141f80c30, "unknown_141f80c30", fn_141f80c30 as *const ()), // sub_141f80c30
    Replacement::draft(0x141f80c60, "unknown_141f80c60", fn_141f80c60 as *const ()), // sub_141f80c60
    Replacement::draft(0x141f80ca0, "unknown_141f80ca0", fn_141f80ca0 as *const ()), // sub_141f80ca0
    Replacement::draft(0x141f80cd0, "unknown_141f80cd0", fn_141f80cd0 as *const ()), // sub_141f80cd0
    Replacement::draft(0x141f80cf0, "unknown_141f80cf0", fn_141f80cf0 as *const ()), // sub_141f80cf0
    Replacement::draft_throws(0x141f80d00, "unknown_141f80d00", fn_141f80d00 as *const (), Throws::Caught), // sub_141f80d00
    Replacement::draft_throws(0x141f80fe0, "unknown_141f80fe0", fn_141f80fe0 as *const (), Throws::Caught), // sub_141f80fe0
    Replacement::draft(0x141f811e0, "unknown_141f811e0", fn_141f811e0 as *const ()), // sub_141f811e0
    Replacement::draft(0x141f81660, "unknown_141f81660", fn_141f81660 as *const ()), // sub_141f81660
    Replacement::draft_throws(0x141f816b0, "unknown_141f816b0", fn_141f816b0 as *const (), Throws::Discard), // sub_141f816b0
    Replacement::draft_throws(0x141f81950, "unknown_141f81950", fn_141f81950 as *const (), Throws::Discard), // sub_141f81950
    Replacement::draft_throws(0x141f81970, "unknown_141f81970", fn_141f81970 as *const (), Throws::Caught), // sub_141f81970
    Replacement::draft(0x141f81a50, "unknown_141f81a50", fn_141f81a50 as *const ()), // sub_141f81a50
    Replacement::draft(0x141f81ae0, "unknown_141f81ae0", fn_141f81ae0 as *const ()), // sub_141f81ae0
    Replacement::draft(0x141f81ba0, "unknown_141f81ba0", fn_141f81ba0 as *const ()), // sub_141f81ba0
    Replacement::draft(0x141f81c60, "unknown_141f81c60", fn_141f81c60 as *const ()), // sub_141f81c60
    Replacement::draft_throws(0x141f82110, "coregame_141f82110", fn_141f82110 as *const (), Throws::Discard), // coregame::DialogueVariableContainer::ClassTypeInfoImplementation::coregame::DialogueVariableContainer::ClassTypeInfoImpl
    Replacement::draft(0x141f82170, "coregame_141f82170", fn_141f82170 as *const ()), // coregame::DialogueVariableContainer::ClassTypeInfoImplementation::coregame::DialogueVariableContainer::ClassTypeInfoImpl
    Replacement::draft(0x141f821b0, "coregame_141f821b0", fn_141f821b0 as *const ()), // coregame::DialogueVariableContainer::ClassTypeInfoImplementation::coregame::DialogueVariableContainer::ClassTypeInfoImpl
    Replacement::draft(0x141f82220, "coregame_141f82220", fn_141f82220 as *const ()), // coregame::DialogueVariableContainer::ClassTypeInfoImplementation::coregame::DialogueVariableContainer::ClassTypeInfoImpl
    Replacement::draft_throws(0x141f82ae0, "unknown_141f82ae0", fn_141f82ae0 as *const (), Throws::Caught), // sub_141f82ae0
    Replacement::draft_throws(0x141f82c30, "unknown_141f82c30", fn_141f82c30 as *const (), Throws::Discard), // sub_141f82c30
    Replacement::draft(0x141f82f10, "unknown_141f82f10", tpl_650::<0x8> as *const ()), // sub_141f82f10
    Replacement::draft(0x141f82f20, "unknown_141f82f20", tpl_42480::<0x8, 0x10, 0x14, 0x1401e84a0, 0x18> as *const ()), // sub_141f82f20
    Replacement::draft(0x141f82f70, "unknown_141f82f70", tpl_42481::<0x8, 0x18> as *const ()), // sub_141f82f70
    Replacement::draft_throws(0x141f82fa0, "unknown_141f82fa0", fn_141f82fa0 as *const (), Throws::Caught), // sub_141f82fa0
    Replacement::draft_throws(0x141f84330, "unknown_141f84330", fn_141f84330 as *const (), Throws::Caught), // sub_141f84330
    Replacement::draft_throws(0x141f84870, "unknown_141f84870", fn_141f84870 as *const (), Throws::Caught), // sub_141f84870
    Replacement::draft(0x141f858a0, "unknown_141f858a0", tpl_650::<0x8> as *const ()), // sub_141f858a0
    Replacement::draft(0x141f858c0, "unknown_141f858c0", tpl_42480::<0x8, 0x10, 0x14, 0x1401e84a0, 0x18> as *const ()), // sub_141f858c0
    Replacement::draft(0x141f85910, "unknown_141f85910", tpl_42481::<0x8, 0x18> as *const ()), // sub_141f85910
    Replacement::draft(0x141f85940, "unknown_141f85940", fn_141f85940 as *const ()), // sub_141f85940
    Replacement::draft(0x141f85950, "unknown_141f85950", tpl_36980::<0x30> as *const ()), // sub_141f85950
    Replacement::draft(0x141f85990, "unknown_141f85990", tpl_40767::<0x8> as *const ()), // sub_141f85990
    Replacement::draft(0x141f85a20, "unknown_141f85a20", fn_141f85a20 as *const ()), // sub_141f85a20
    Replacement::draft(0x141f85a60, "unknown_141f85a60", fn_141f85a60 as *const ()), // sub_141f85a60
    Replacement::draft_throws(0x141f85a80, "unknown_141f85a80", fn_141f85a80 as *const (), Throws::Caught), // sub_141f85a80
    Replacement::draft(0x141f881e0, "unknown_141f881e0", fn_141f881e0 as *const ()), // sub_141f881e0
    Replacement::draft_throws(0x141f88460, "unknown_141f88460", fn_141f88460 as *const (), Throws::Discard), // sub_141f88460
    Replacement::draft_throws(0x141f88710, "unknown_141f88710", fn_141f88710 as *const (), Throws::Caught), // sub_141f88710
    Replacement::draft_throws(0x141f88f80, "unknown_141f88f80", fn_141f88f80 as *const (), Throws::Caught), // sub_141f88f80
    Replacement::draft_throws(0x141f89f70, "unknown_141f89f70", fn_141f89f70 as *const (), Throws::Caught), // sub_141f89f70
    Replacement::draft_throws(0x141f8a0c0, "unknown_141f8a0c0", fn_141f8a0c0 as *const (), Throws::Caught), // sub_141f8a0c0
    Replacement::draft_throws(0x141f8a210, "unknown_141f8a210", fn_141f8a210 as *const (), Throws::Caught), // sub_141f8a210
    Replacement::draft_throws(0x141f8a300, "unknown_141f8a300", fn_141f8a300 as *const (), Throws::Caught), // sub_141f8a300
    Replacement::draft(0x141f8ac20, "unknown_141f8ac20", fn_141f8ac20 as *const ()), // sub_141f8ac20
    Replacement::draft(0x141f8adc0, "unknown_141f8adc0", fn_141f8adc0 as *const ()), // sub_141f8adc0
    Replacement::draft(0x141f8aff0, "unknown_141f8aff0", fn_141f8aff0 as *const ()), // sub_141f8aff0
    Replacement::draft(0x141f8b060, "unknown_141f8b060", fn_141f8b060 as *const ()), // sub_141f8b060
    Replacement::draft(0x141f8b0b0, "unknown_141f8b0b0", fn_141f8b0b0 as *const ()), // sub_141f8b0b0
    Replacement::draft(0x141f8b100, "unknown_141f8b100", fn_141f8b100 as *const ()), // sub_141f8b100
    Replacement::draft_throws(0x141f8b190, "unknown_141f8b190", fn_141f8b190 as *const (), Throws::Caught), // sub_141f8b190
    Replacement::draft_throws(0x141f8bce0, "unknown_141f8bce0", fn_141f8bce0 as *const (), Throws::Discard), // sub_141f8bce0
    Replacement::draft_throws(0x141f8beb0, "std_141f8beb0", fn_141f8beb0 as *const (), Throws::Discard), // sub_141f8beb0
    Replacement::draft(0x141f8c050, "unknown_141f8c050", fn_141f8c050 as *const ()), // sub_141f8c050
    Replacement::draft(0x141f8c230, "unknown_141f8c230", fn_141f8c230 as *const ()), // sub_141f8c230
    Replacement::draft(0x141f8c2f0, "unknown_141f8c2f0", tpl_650::<0x8> as *const ()), // sub_141f8c2f0
    Replacement::draft(0x141f8c300, "unknown_141f8c300", tpl_1269::<0x8, 0x10, 0x14, 0x1401e84a0> as *const ()), // sub_141f8c300
    Replacement::draft(0x141f8c330, "unknown_141f8c330", fn_141f8c330 as *const ()), // sub_141f8c330
    Replacement::draft(0x141f8c3a0, "unknown_141f8c3a0", fn_141f8c3a0 as *const ()), // sub_141f8c3a0
    Replacement::draft_throws(0x141f8c4e0, "unknown_141f8c4e0", fn_141f8c4e0 as *const (), Throws::Discard), // sub_141f8c4e0
    Replacement::draft_throws(0x141f8c5d0, "unknown_141f8c5d0", fn_141f8c5d0 as *const (), Throws::Caught), // sub_141f8c5d0
    Replacement::draft(0x141f8c8b0, "unknown_141f8c8b0", fn_141f8c8b0 as *const ()), // sub_141f8c8b0
    Replacement::draft_throws(0x141f8c900, "unknown_141f8c900", fn_141f8c900 as *const (), Throws::Discard), // sub_141f8c900
    Replacement::draft_throws(0x141f8cb40, "unknown_141f8cb40", fn_141f8cb40 as *const (), Throws::Discard), // sub_141f8cb40
    Replacement::draft(0x141f8cd40, "unknown_141f8cd40", fn_141f8cd40 as *const ()), // sub_141f8cd40
    Replacement::draft_throws(0x141f8ce30, "unknown_141f8ce30", fn_141f8ce30 as *const (), Throws::Discard), // sub_141f8ce30
    Replacement::draft(0x141f8d9a0, "unknown_141f8d9a0", fn_141f8d9a0 as *const ()), // sub_141f8d9a0
    Replacement::draft(0x141f8dbe0, "unknown_141f8dbe0", fn_141f8dbe0 as *const ()), // sub_141f8dbe0
    Replacement::draft(0x141f8dfa0, "unknown_141f8dfa0", fn_141f8dfa0 as *const ()), // sub_141f8dfa0
    Replacement::draft_throws(0x141f8e0d0, "unknown_141f8e0d0", fn_141f8e0d0 as *const (), Throws::Caught), // sub_141f8e0d0
    Replacement::draft_throws(0x141f8eb10, "unknown_141f8eb10", fn_141f8eb10 as *const (), Throws::Caught), // sub_141f8eb10
    Replacement::draft_throws(0x141f8f210, "unknown_141f8f210", fn_141f8f210 as *const (), Throws::Caught), // sub_141f8f210
    Replacement::draft(0x141f8f470, "unknown_141f8f470", fn_141f8f470 as *const ()), // sub_141f8f470
    Replacement::draft(0x141f8f5b0, "unknown_141f8f5b0", fn_141f8f5b0 as *const ()), // sub_141f8f5b0
    Replacement::draft(0x141f8f600, "unknown_141f8f600", fn_141f8f600 as *const ()), // sub_141f8f600
    Replacement::draft_throws(0x141f8f630, "unknown_141f8f630", fn_141f8f630 as *const (), Throws::Discard), // sub_141f8f630
    Replacement::draft(0x141f8f9f0, "unknown_141f8f9f0", fn_141f8f9f0 as *const ()), // sub_141f8f9f0
    Replacement::draft(0x141f8fa10, "unknown_141f8fa10", fn_141f8fa10 as *const ()), // sub_141f8fa10
    Replacement::draft(0x141f8fc40, "unknown_141f8fc40", fn_141f8fc40 as *const ()), // sub_141f8fc40
    Replacement::draft(0x141f8fcc0, "unknown_141f8fcc0", fn_141f8fcc0 as *const ()), // sub_141f8fcc0
    Replacement::draft(0x141f8fea0, "unknown_141f8fea0", fn_141f8fea0 as *const ()), // sub_141f8fea0
    Replacement::draft(0x141f8ff40, "unknown_141f8ff40", fn_141f8ff40 as *const ()), // sub_141f8ff40
    Replacement::draft(0x141f8ff90, "unknown_141f8ff90", fn_141f8ff90 as *const ()), // sub_141f8ff90
    Replacement::draft(0x141f8ffb0, "unknown_141f8ffb0", fn_141f8ffb0 as *const ()), // sub_141f8ffb0
    Replacement::draft(0x141f90110, "unknown_141f90110", fn_141f90110 as *const ()), // sub_141f90110
    Replacement::draft(0x141f901e0, "unknown_141f901e0", tpl_1044::<0x143855908> as *const ()), // sub_141f901e0
    Replacement::draft(0x141f901f0, "unknown_141f901f0", tpl_42518::<0x14385583c> as *const ()), // sub_141f901f0
    Replacement::draft(0x141f90200, "unknown_141f90200", tpl_42518::<0x14385590c> as *const ()), // sub_141f90200
    Replacement::draft(0x141f90210, "unknown_141f90210", tpl_42518::<0x14385578c> as *const ()), // sub_141f90210
    Replacement::draft(0x141f90220, "unknown_141f90220", tpl_1044::<0x143855788> as *const ()), // sub_141f90220
    Replacement::draft(0x141f90230, "unknown_141f90230", tpl_1044::<0x143855838> as *const ()), // sub_141f90230
    Replacement::draft(0x141f902b0, "unknown_141f902b0", fn_141f902b0 as *const ()), // sub_141f902b0
    Replacement::draft(0x141f903f0, "unknown_141f903f0", fn_141f903f0 as *const ()), // sub_141f903f0
    Replacement::draft(0x141f904a0, "unknown_141f904a0", tpl_34667::<0x20> as *const ()), // sub_141f904a0
    Replacement::draft(0x141f90660, "unknown_141f90660", tpl_94790 as *const ()), // sub_141f90660
    Replacement::draft(0x141f90710, "unknown_141f90710", tpl_36742::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141f90710
    Replacement::draft(0x141f90870, "unknown_141f90870", fn_141f90870 as *const ()), // sub_141f90870
    Replacement::draft(0x141f90950, "unknown_141f90950", fn_141f90950 as *const ()), // sub_141f90950
    Replacement::draft(0x141f909b0, "unknown_141f909b0", fn_141f909b0 as *const ()), // sub_141f909b0
    Replacement::draft(0x141f91a60, "unknown_141f91a60", fn_141f91a60 as *const ()), // sub_141f91a60
    Replacement::draft(0x141f91ad0, "unknown_141f91ad0", fn_141f91ad0 as *const ()), // sub_141f91ad0
    Replacement::draft(0x141f92070, "unknown_141f92070", fn_141f92070 as *const ()), // sub_141f92070
    Replacement::draft(0x141f92100, "unknown_141f92100", fn_141f92100 as *const ()), // sub_141f92100
    Replacement::draft(0x141f92160, "unknown_141f92160", fn_141f92160 as *const ()), // sub_141f92160
    Replacement::draft(0x141f924d0, "unknown_141f924d0", fn_141f924d0 as *const ()), // sub_141f924d0
    Replacement::draft_throws(0x141f92830, "unknown_141f92830", fn_141f92830 as *const (), Throws::Caught), // sub_141f92830
    Replacement::draft(0x141f93380, "unknown_141f93380", tpl_21064::<0x8, 0x14110e520, 0x98, 0x140217df0, 0x14110e720> as *const ()), // sub_141f93380
    Replacement::draft(0x141f93400, "unknown_141f93400", fn_141f93400 as *const ()), // sub_141f93400
    Replacement::draft(0x141f934f0, "unknown_141f934f0", fn_141f934f0 as *const ()), // sub_141f934f0
    Replacement::draft(0x141f935b0, "unknown_141f935b0", fn_141f935b0 as *const ()), // sub_141f935b0
    Replacement::draft(0x141f936e0, "unknown_141f936e0", fn_141f936e0 as *const ()), // sub_141f936e0
    Replacement::draft_throws(0x141f93850, "unknown_141f93850", fn_141f93850 as *const (), Throws::Caught), // sub_141f93850
    Replacement::draft_throws(0x141f93ac0, "unknown_141f93ac0", fn_141f93ac0 as *const (), Throws::Caught), // sub_141f93ac0
    Replacement::draft_throws(0x141f93c40, "unknown_141f93c40", fn_141f93c40 as *const (), Throws::Caught), // sub_141f93c40
    Replacement::draft(0x141f942a0, "unknown_141f942a0", fn_141f942a0 as *const ()), // sub_141f942a0
    Replacement::draft(0x141f94540, "unknown_141f94540", fn_141f94540 as *const ()), // sub_141f94540
    Replacement::draft_throws(0x141f947f0, "unknown_141f947f0", fn_141f947f0 as *const (), Throws::Discard), // sub_141f947f0
    Replacement::draft(0x141f94e40, "unknown_141f94e40", fn_141f94e40 as *const ()), // sub_141f94e40
    Replacement::draft(0x141f95080, "unknown_141f95080", fn_141f95080 as *const ()), // sub_141f95080
    Replacement::draft(0x141f95200, "unknown_141f95200", fn_141f95200 as *const ()), // sub_141f95200
    Replacement::draft_throws(0x141f95280, "unknown_141f95280", tpl_36792::<0x14387e0e8, 0x1401de410, 0x1402116e0, 0x14292d530, 0x18, 0x1410d6ca0, 0x10, 0x141115ee0> as *const (), Throws::Discard), // sub_141f95280
    Replacement::draft(0x141f95310, "unknown_141f95310", tpl_42543 as *const ()), // sub_141f95310
    Replacement::draft(0x141f95320, "unknown_141f95320", fn_141f95320 as *const ()), // sub_141f95320
    Replacement::draft(0x141f95350, "unknown_141f95350", fn_141f95350 as *const ()), // sub_141f95350
    Replacement::draft(0x141f956b0, "unknown_141f956b0", fn_141f956b0 as *const ()), // sub_141f956b0
    Replacement::draft(0x141f95980, "unknown_141f95980", fn_141f95980 as *const ()), // sub_141f95980
    Replacement::draft(0x141f95ca0, "unknown_141f95ca0", tpl_34605::<0x1432876a0> as *const ()), // sub_141f95ca0
    Replacement::draft(0x141f95d30, "unknown_141f95d30", tpl_36123 as *const ()), // sub_141f95d30
    Replacement::draft(0x141f95d70, "unknown_141f95d70", tpl_34994::<0x143cdef0c, 0x1438810e0, 0x1438810d8, 0x141834a78, 0x142615850, 0x141834d18, 0x141834a0c> as *const ()), // sub_141f95d70
    Replacement::draft(0x141f95de0, "unknown_141f95de0", tpl_369 as *const ()), // sub_141f95de0
    Replacement::draft(0x141f95df0, "unknown_141f95df0", tpl_42418::<0x8, 0x580e0, 0x143c8cef8, 0x1c> as *const ()), // sub_141f95df0
    Replacement::draft(0x141f95e40, "unknown_141f95e40", tpl_21089::<0x8, 0x1411172c0, 0x1411174a0> as *const ()), // sub_141f95e40
    Replacement::draft(0x141f95ea0, "unknown_141f95ea0", fn_141f95ea0 as *const ()), // sub_141f95ea0
    Replacement::draft(0x141f95fe0, "unknown_141f95fe0", tpl_496 as *const ()), // sub_141f95fe0
    Replacement::draft_throws(0x141f95ff0, "unknown_141f95ff0", fn_141f95ff0 as *const (), Throws::Discard), // sub_141f95ff0
    Replacement::draft(0x141f961e0, "unknown_141f961e0", tpl_361 as *const ()), // sub_141f961e0
    Replacement::draft(0x141f961f0, "unknown_141f961f0", tpl_361 as *const ()), // sub_141f961f0
    Replacement::draft(0x141f96200, "unknown_141f96200", fn_141f96200 as *const ()), // sub_141f96200
    Replacement::draft_throws(0x141f96290, "unknown_141f96290", fn_141f96290 as *const (), Throws::Caught), // sub_141f96290
    Replacement::draft(0x141f96490, "unknown_141f96490", tpl_28185::<0x143a28c0d> as *const ()), // sub_141f96490
    Replacement::draft(0x141f964a0, "unknown_141f964a0", fn_141f964a0 as *const ()), // sub_141f964a0
    Replacement::draft_throws(0x141f96690, "unknown_141f96690", fn_141f96690 as *const (), Throws::Discard), // sub_141f96690
    Replacement::draft(0x141f966d0, "unknown_141f966d0", tpl_28185::<0x143a28c0e> as *const ()), // sub_141f966d0
    Replacement::draft(0x141f966e0, "unknown_141f966e0", tpl_42554::<0x143a28c0e> as *const ()), // sub_141f966e0
    Replacement::draft(0x141f966f0, "unknown_141f966f0", tpl_42554::<0x143a28c0d> as *const ()), // sub_141f966f0
    Replacement::draft(0x141f96700, "std_141f96700", tpl_23342::<0x1433a4208, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::ContentBlockLoader::load::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x141f96720, "std_141f96720", tpl_23342::<0x1433a4208, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::ContentBlockLoader::load::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_alloc<
    Replacement::draft_throws(0x141f96740, "std_141f96740", fn_141f96740 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::ContentBlockLoader::load::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x141f96890, "std_141f96890", tpl_1::<0x143890ba0> as *const ()), // std::_Func_impl_no_alloc<coregame::ContentBlockLoader::load::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x141f968a0, "std_141f968a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::ContentBlockLoader::load::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x141f968b0, "std_141f968b0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::ContentBlockLoader::load::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_no_alloc<
    Replacement::draft_throws(0x141f96960, "unknown_141f96960", fn_141f96960 as *const (), Throws::Discard), // sub_141f96960
    Replacement::draft_throws(0x141f96bd0, "unknown_141f96bd0", fn_141f96bd0 as *const (), Throws::Discard), // sub_141f96bd0
    Replacement::draft_throws(0x141f96ef0, "coregame_141f96ef0", fn_141f96ef0 as *const (), Throws::Discard), // coregame::ContentBlockLoader::unload::StreamJob_Unload::coregame::ContentBlockLoader::unload::StreamJob_Unload::vfunc_2
    Replacement::draft_throws(0x141f96fb0, "anonymous_namespace_141f96fb0", fn_141f96fb0 as *const (), Throws::Discard), // sub_141f96fb0
    Replacement::draft(0x141f970f0, "anonymous_namespace_141f970f0", fn_141f970f0 as *const ()), // anonymous_namespace::StreamJob_BeginUnload::anonymous_namespace::StreamJob_BeginUnload::vfunc_2
    Replacement::draft(0x141f97110, "anonymous_namespace_141f97110", fn_141f97110 as *const ()), // anonymous_namespace::StreamJob_LoadCompleted::anonymous_namespace::StreamJob_LoadCompleted::vfunc_2
    Replacement::draft_throws(0x141f97130, "unknown_141f97130", fn_141f97130 as *const (), Throws::Discard), // sub_141f97130
    Replacement::draft_throws(0x141f97440, "unknown_141f97440", fn_141f97440 as *const (), Throws::Discard), // sub_141f97440
    Replacement::draft(0x141f976d0, "unknown_141f976d0", fn_141f976d0 as *const ()), // sub_141f976d0
    Replacement::draft(0x141f97750, "unknown_141f97750", fn_141f97750 as *const ()), // sub_141f97750
    Replacement::draft(0x141f97800, "unknown_141f97800", fn_141f97800 as *const ()), // sub_141f97800
    Replacement::draft(0x141f97860, "unknown_141f97860", fn_141f97860 as *const ()), // sub_141f97860
    Replacement::draft(0x141f978a0, "unknown_141f978a0", fn_141f978a0 as *const ()), // sub_141f978a0
    Replacement::draft(0x141f97970, "unknown_141f97970", fn_141f97970 as *const ()), // sub_141f97970
    Replacement::draft(0x141f979a0, "unknown_141f979a0", fn_141f979a0 as *const ()), // sub_141f979a0
    Replacement::draft_throws(0x141f97a90, "unknown_141f97a90", fn_141f97a90 as *const (), Throws::Discard), // sub_141f97a90
    Replacement::draft(0x141f97bb0, "unknown_141f97bb0", fn_141f97bb0 as *const ()), // sub_141f97bb0
    Replacement::draft(0x141f97db0, "unknown_141f97db0", fn_141f97db0 as *const ()), // sub_141f97db0
    Replacement::draft_throws(0x141f98320, "unknown_141f98320", fn_141f98320 as *const (), Throws::Discard), // sub_141f98320
    Replacement::draft(0x141f98410, "unknown_141f98410", fn_141f98410 as *const ()), // sub_141f98410
    Replacement::draft_throws(0x141f98af0, "unknown_141f98af0", fn_141f98af0 as *const (), Throws::Discard), // sub_141f98af0
    Replacement::draft_throws(0x141f98ba0, "unknown_141f98ba0", fn_141f98ba0 as *const (), Throws::Discard), // sub_141f98ba0
    Replacement::draft(0x141f98db0, "unknown_141f98db0", tpl_3542::<0x8, 0x141f97cc0, 0xc> as *const ()), // sub_141f98db0
    Replacement::draft(0x141f98e60, "unknown_141f98e60", fn_141f98e60 as *const ()), // sub_141f98e60
    Replacement::draft(0x141f98ef0, "unknown_141f98ef0", fn_141f98ef0 as *const ()), // sub_141f98ef0
    Replacement::draft(0x141f995b0, "unknown_141f995b0", fn_141f995b0 as *const ()), // sub_141f995b0
    Replacement::draft(0x141f995c0, "unknown_141f995c0", fn_141f995c0 as *const ()), // sub_141f995c0
    Replacement::draft(0x141f99700, "unknown_141f99700", fn_141f99700 as *const ()), // sub_141f99700
    Replacement::draft(0x141f99840, "unknown_141f99840", fn_141f99840 as *const ()), // sub_141f99840
    Replacement::draft(0x141f99f80, "unknown_141f99f80", fn_141f99f80 as *const ()), // sub_141f99f80
    Replacement::draft(0x141f9a100, "unknown_141f9a100", fn_141f9a100 as *const ()), // sub_141f9a100
    Replacement::draft_throws(0x141f9a210, "unknown_141f9a210", fn_141f9a210 as *const (), Throws::Discard), // sub_141f9a210
    Replacement::draft_throws(0x141f9a270, "unknown_141f9a270", fn_141f9a270 as *const (), Throws::Discard), // sub_141f9a270
    Replacement::draft_throws(0x141f9a290, "unknown_141f9a290", fn_141f9a290 as *const (), Throws::Discard), // sub_141f9a290
    Replacement::draft_throws(0x141f9a3c0, "unknown_141f9a3c0", fn_141f9a3c0 as *const (), Throws::Discard), // sub_141f9a3c0
    Replacement::draft_throws(0x141f9a420, "unknown_141f9a420", fn_141f9a420 as *const (), Throws::Discard), // sub_141f9a420
    Replacement::draft_throws(0x141f9a540, "unknown_141f9a540", fn_141f9a540 as *const (), Throws::Caught), // sub_141f9a540
    Replacement::draft(0x141f9a6d0, "unknown_141f9a6d0", fn_141f9a6d0 as *const ()), // sub_141f9a6d0
    Replacement::draft_throws(0x141f9a770, "unknown_141f9a770", fn_141f9a770 as *const (), Throws::Discard), // sub_141f9a770
    Replacement::draft_throws(0x141f9a7d0, "unknown_141f9a7d0", fn_141f9a7d0 as *const (), Throws::Discard), // sub_141f9a7d0
    Replacement::draft(0x141f9a880, "unknown_141f9a880", fn_141f9a880 as *const ()), // sub_141f9a880
    Replacement::draft_throws(0x141f9a9f0, "unknown_141f9a9f0", fn_141f9a9f0 as *const (), Throws::Discard), // sub_141f9a9f0
    Replacement::draft(0x141f9ac10, "unknown_141f9ac10", fn_141f9ac10 as *const ()), // sub_141f9ac10
    Replacement::draft(0x141f9ac40, "unknown_141f9ac40", tpl_42593 as *const ()), // sub_141f9ac40
    Replacement::draft_throws(0x141f9ac50, "unknown_141f9ac50", fn_141f9ac50 as *const (), Throws::Discard), // sub_141f9ac50
    Replacement::draft_throws(0x141f9af30, "unknown_141f9af30", fn_141f9af30 as *const (), Throws::Discard), // sub_141f9af30
    Replacement::draft_throws(0x141f9b1d0, "unknown_141f9b1d0", fn_141f9b1d0 as *const (), Throws::Discard), // sub_141f9b1d0
    Replacement::draft_throws(0x141f9b210, "unknown_141f9b210", fn_141f9b210 as *const (), Throws::Discard), // sub_141f9b210
    Replacement::draft_throws(0x141f9b270, "unknown_141f9b270", fn_141f9b270 as *const (), Throws::Discard), // sub_141f9b270
    Replacement::draft_throws(0x141f9b2d0, "unknown_141f9b2d0", fn_141f9b2d0 as *const (), Throws::Discard), // sub_141f9b2d0
    Replacement::draft_throws(0x141f9b320, "unknown_141f9b320", fn_141f9b320 as *const (), Throws::Discard), // sub_141f9b320
    Replacement::draft_throws(0x141f9b380, "unknown_141f9b380", fn_141f9b380 as *const (), Throws::Discard), // sub_141f9b380
    Replacement::draft_throws(0x141f9b3d0, "unknown_141f9b3d0", fn_141f9b3d0 as *const (), Throws::Discard), // sub_141f9b3d0
    Replacement::draft_throws(0x141f9b420, "unknown_141f9b420", fn_141f9b420 as *const (), Throws::Discard), // sub_141f9b420
    Replacement::draft_throws(0x141f9b460, "unknown_141f9b460", fn_141f9b460 as *const (), Throws::Discard), // sub_141f9b460
    Replacement::draft(0x141f9b7a0, "unknown_141f9b7a0", fn_141f9b7a0 as *const ()), // sub_141f9b7a0
    Replacement::draft_throws(0x141f9b8a0, "unknown_141f9b8a0", fn_141f9b8a0 as *const (), Throws::Discard), // sub_141f9b8a0
    Replacement::draft_throws(0x141f9b950, "unknown_141f9b950", fn_141f9b950 as *const (), Throws::Discard), // sub_141f9b950
    Replacement::draft_throws(0x141f9bae0, "unknown_141f9bae0", fn_141f9bae0 as *const (), Throws::Caught), // sub_141f9bae0
    Replacement::draft(0x141f9bef0, "unknown_141f9bef0", fn_141f9bef0 as *const ()), // sub_141f9bef0
    Replacement::draft(0x141f9cbc0, "unknown_141f9cbc0", fn_141f9cbc0 as *const ()), // sub_141f9cbc0
    Replacement::draft(0x141fa11e0, "unknown_141fa11e0", tpl_94835::<0x1432e9cf0, 0x8, 0x10, 0x14, 0x18> as *const ()), // sub_141fa11e0
    Replacement::draft(0x141fa12c0, "unknown_141fa12c0", tpl_35059::<0x1432e9cf0, 0x8, 0x10, 0x14, 0x18> as *const ()), // sub_141fa12c0
    Replacement::draft(0x141fa1450, "unknown_141fa1450", tpl_100070::<0x8, 0xc, 0x10> as *const ()), // sub_141fa1450
    Replacement::draft(0x141fa1510, "unknown_141fa1510", tpl_40439::<0x8, 0xc, 0x10> as *const ()), // sub_141fa1510
    Replacement::draft(0x141fa1560, "unknown_141fa1560", tpl_34607::<0x141123610> as *const ()), // sub_141fa1560
    Replacement::draft(0x141fa1610, "unknown_141fa1610", tpl_94422::<0x8, 0xc, 0x10> as *const ()), // sub_141fa1610
    Replacement::draft(0x141fa1710, "unknown_141fa1710", tpl_34607::<0x140d4aa80> as *const ()), // sub_141fa1710
    Replacement::draft(0x141fa1c50, "unknown_141fa1c50", tpl_1439::<0x8, 0x1411240f0> as *const ()), // sub_141fa1c50
    Replacement::draft(0x141fa1c90, "unknown_141fa1c90", fn_141fa1c90 as *const ()), // sub_141fa1c90
    Replacement::draft(0x141fa1fd0, "unknown_141fa1fd0", fn_141fa1fd0 as *const ()), // sub_141fa1fd0
    Replacement::draft(0x141fa23c0, "unknown_141fa23c0", fn_141fa23c0 as *const ()), // sub_141fa23c0
    Replacement::draft(0x141fa2800, "unknown_141fa2800", fn_141fa2800 as *const ()), // sub_141fa2800
    Replacement::draft(0x141fa2bb0, "unknown_141fa2bb0", fn_141fa2bb0 as *const ()), // sub_141fa2bb0
    Replacement::draft(0x141fa2f10, "unknown_141fa2f10", tpl_34723::<0xc> as *const ()), // sub_141fa2f10
    Replacement::draft(0x141fa2f20, "unknown_141fa2f20", fn_141fa2f20 as *const ()), // sub_141fa2f20
    Replacement::draft_throws(0x141fa3380, "unknown_141fa3380", fn_141fa3380 as *const (), Throws::Discard), // sub_141fa3380
    Replacement::draft_throws(0x141fa3550, "unknown_141fa3550", fn_141fa3550 as *const (), Throws::Discard), // sub_141fa3550
    Replacement::draft(0x141fa3630, "unknown_141fa3630", fn_141fa3630 as *const ()), // sub_141fa3630
    Replacement::draft_throws(0x141fa3800, "unknown_141fa3800", fn_141fa3800 as *const (), Throws::Discard), // sub_141fa3800
    Replacement::draft(0x141fa4070, "unknown_141fa4070", tpl_94771::<0x140fdea60> as *const ()), // sub_141fa4070
    Replacement::draft(0x141fa4130, "unknown_141fa4130", tpl_34684::<0x140fd8d10> as *const ()), // sub_141fa4130
    Replacement::draft(0x141fa41e0, "unknown_141fa41e0", tpl_40998::<0x8, 0x14328f7a8, 0xb, 0xc, 0x10, 0x14240a730> as *const ()), // sub_141fa41e0
    Replacement::draft(0x141fa42d0, "unknown_141fa42d0", tpl_40999::<0x14328f7a8, 0x8, 0x3, 0x4, 0x14240a730> as *const ()), // sub_141fa42d0
    Replacement::draft(0x141fa43e0, "unknown_141fa43e0", fn_141fa43e0 as *const ()), // sub_141fa43e0
    Replacement::draft(0x141fa4570, "unknown_141fa4570", tpl_34615 as *const ()), // sub_141fa4570
    Replacement::draft(0x141fa4620, "unknown_141fa4620", tpl_35099::<0x20, 0x40> as *const ()), // sub_141fa4620
    Replacement::draft(0x141fa46d0, "unknown_141fa46d0", tpl_34394::<0x10> as *const ()), // sub_141fa46d0
    Replacement::draft(0x141fa4770, "unknown_141fa4770", tpl_94771::<0x14112fbe0> as *const ()), // sub_141fa4770
    Replacement::draft(0x141fa4830, "unknown_141fa4830", tpl_34684::<0x141128780> as *const ()), // sub_141fa4830
    Replacement::draft(0x141fa4a80, "unknown_141fa4a80", tpl_41177::<0x20, 0x8, 0x40, 0x1433acef0, 0x28, 0x48> as *const ()), // sub_141fa4a80
    Replacement::draft(0x141fa4ac0, "unknown_141fa4ac0", tpl_37791::<0x20, 0x8, 0x18, 0x50, 0x30, 0x14112ef70> as *const ()), // sub_141fa4ac0
    Replacement::draft(0x141fa4b10, "unknown_141fa4b10", tpl_37790::<0x20, 0x8, 0x30, 0x1433ace80, 0x28, 0x38> as *const ()), // sub_141fa4b10
    Replacement::draft_throws(0x141fa4b50, "unknown_141fa4b50", tpl_42617::<0x20, 0x8, 0x18, 0x38, 0x30, 0x14112f130> as *const (), Throws::Discard), // sub_141fa4b50
    Replacement::draft(0x141fa4dd0, "unknown_141fa4dd0", fn_141fa4dd0 as *const ()), // sub_141fa4dd0
    Replacement::draft(0x141fa4f10, "unknown_141fa4f10", fn_141fa4f10 as *const ()), // sub_141fa4f10
    Replacement::draft(0x141fa4f30, "unknown_141fa4f30", fn_141fa4f30 as *const ()), // sub_141fa4f30
    Replacement::draft_throws(0x141fa4ff0, "unknown_141fa4ff0", tpl_42621::<0x38, 0x30, 0x18, 0x28, 0x10, 0x14112f130> as *const (), Throws::Discard), // sub_141fa4ff0
    Replacement::draft(0x141fa5090, "unknown_141fa5090", tpl_37792::<0x50, 0x48, 0x18, 0x28, 0x10, 0x14112ef70> as *const ()), // sub_141fa5090
    Replacement::draft(0x141fa50e0, "unknown_141fa50e0", tpl_473 as *const ()), // sub_141fa50e0
    Replacement::draft(0x141fa50f0, "unknown_141fa50f0", tpl_361 as *const ()), // sub_141fa50f0
    Replacement::draft(0x141fa5100, "unknown_141fa5100", tpl_496 as *const ()), // sub_141fa5100
    Replacement::draft(0x141fa5110, "unknown_141fa5110", tpl_361 as *const ()), // sub_141fa5110
    Replacement::draft(0x141fa5120, "unknown_141fa5120", tpl_548::<0xc> as *const ()), // sub_141fa5120
    Replacement::draft(0x141fa5130, "unknown_141fa5130", tpl_37730::<0xc> as *const ()), // sub_141fa5130
    Replacement::draft_throws(0x141fa5150, "unknown_141fa5150", tpl_37728::<0x14241c0a0> as *const (), Throws::Discard), // sub_141fa5150
    Replacement::draft_throws(0x141fa5170, "unknown_141fa5170", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_141fa5170
    Replacement::draft(0x141fa5190, "std_141fa5190", tpl_770::<0x1433acea8> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::<lambda_2>::operator()::<lambda_1>,m::Vector3Template<float>,c
    Replacement::draft(0x141fa51b0, "std_141fa51b0", tpl_770::<0x1433acea8> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::<lambda_2>::operator()::<lambda_1>,m::Vector3Template<float>,c
    Replacement::draft(0x141fa51d0, "std_141fa51d0", fn_141fa51d0 as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::<lambda_2>::operator()::<lambda_1>,m::Vector3Template<float>,c
    Replacement::draft(0x141fa5210, "std_141fa5210", tpl_1::<0x143890e30> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::<lambda_2>::operator()::<lambda_1>,m::Vector3Template<float>,c
    Replacement::draft(0x141fa5220, "std_141fa5220", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::<lambda_2>::operator()::<lambda_1>,m::Vector3Template<float>,c
    Replacement::draft(0x141fa5230, "std_141fa5230", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::<lambda_2>::operator()::<lambda_1>,m::Vector3Template<float>,c
    Replacement::draft(0x141fa5250, "std_141fa5250", tpl_550::<0x1433acf30, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::?bref1::operator()::<lambda_2>,m::Vector3Template<float>,const
    Replacement::draft(0x141fa5270, "std_141fa5270", tpl_550::<0x1433acf30, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::?bref1::operator()::<lambda_2>,m::Vector3Template<float>,const
    Replacement::draft(0x141fa5290, "std_141fa5290", fn_141fa5290 as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::?bref1::operator()::<lambda_2>,m::Vector3Template<float>,const
    Replacement::draft(0x141fa52f0, "std_141fa52f0", tpl_1::<0x1438912c0> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::?bref1::operator()::<lambda_2>,m::Vector3Template<float>,const
    Replacement::draft(0x141fa5300, "std_141fa5300", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::?bref1::operator()::<lambda_2>,m::Vector3Template<float>,const
    Replacement::draft(0x141fa5310, "std_141fa5310", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::cloth::simulateEntity::?bref1::operator()::<lambda_2>,m::Vector3Template<float>,const
    Replacement::draft(0x141fa5330, "unknown_141fa5330", tpl_895::<0x8> as *const ()), // sub_141fa5330
    Replacement::draft_throws(0x141fa5390, "unknown_141fa5390", tpl_42624::<0x18, 0x10, 0x30, 0x28, 0x14112f130> as *const (), Throws::Discard), // sub_141fa5390
    Replacement::draft(0x141fa5430, "unknown_141fa5430", tpl_37793::<0x18, 0x10, 0x48, 0x28, 0x14112ef70> as *const ()), // sub_141fa5430
    Replacement::draft(0x141fa5c90, "unknown_141fa5c90", tpl_1439::<0x8, 0x14112bfe0> as *const ()), // sub_141fa5c90
    Replacement::draft(0x141fa5cd0, "unknown_141fa5cd0", tpl_1439::<0x8, 0x14112bde0> as *const ()), // sub_141fa5cd0
    Replacement::draft(0x141fa5d10, "unknown_141fa5d10", fn_141fa5d10 as *const ()), // sub_141fa5d10
    Replacement::draft(0x141fa5de0, "unknown_141fa5de0", fn_141fa5de0 as *const ()), // sub_141fa5de0
    Replacement::draft(0x141fa5f80, "unknown_141fa5f80", fn_141fa5f80 as *const ()), // sub_141fa5f80
    Replacement::draft(0x141fa61e0, "unknown_141fa61e0", tpl_35031::<0x142407c80, 0x143cdfc70, 0x14020e8f0> as *const ()), // sub_141fa61e0
    Replacement::draft_throws(0x141fa6310, "unknown_141fa6310", fn_141fa6310 as *const (), Throws::Caught), // sub_141fa6310
    Replacement::draft(0x141fa6500, "unknown_141fa6500", fn_141fa6500 as *const ()), // sub_141fa6500
    Replacement::draft(0x141fa6950, "unknown_141fa6950", fn_141fa6950 as *const ()), // sub_141fa6950
    Replacement::draft(0x141fa6ad0, "unknown_141fa6ad0", fn_141fa6ad0 as *const ()), // sub_141fa6ad0
    Replacement::draft_throws(0x141fa6cf0, "unknown_141fa6cf0", fn_141fa6cf0 as *const (), Throws::Discard), // sub_141fa6cf0
    Replacement::draft_throws(0x141fa6d40, "unknown_141fa6d40", fn_141fa6d40 as *const (), Throws::Discard), // sub_141fa6d40
    Replacement::draft_throws(0x141fa6ff0, "unknown_141fa6ff0", fn_141fa6ff0 as *const (), Throws::Discard), // sub_141fa6ff0
    Replacement::draft(0x141fa7090, "unknown_141fa7090", fn_141fa7090 as *const ()), // sub_141fa7090
    Replacement::draft(0x141fa70f0, "unknown_141fa70f0", fn_141fa70f0 as *const ()), // sub_141fa70f0
    Replacement::draft(0x141fa7140, "unknown_141fa7140", fn_141fa7140 as *const ()), // sub_141fa7140
    Replacement::draft_throws(0x141fa7490, "unknown_141fa7490", fn_141fa7490 as *const (), Throws::Discard), // sub_141fa7490
    Replacement::draft_throws(0x141fa77a0, "unknown_141fa77a0", fn_141fa77a0 as *const (), Throws::Discard), // sub_141fa77a0
    Replacement::draft(0x141fa7b90, "unknown_141fa7b90", fn_141fa7b90 as *const ()), // sub_141fa7b90
    Replacement::draft_throws(0x141fa7c00, "unknown_141fa7c00", fn_141fa7c00 as *const (), Throws::Caught), // sub_141fa7c00
    Replacement::draft_throws(0x141fa7dc0, "unknown_141fa7dc0", fn_141fa7dc0 as *const (), Throws::Discard), // sub_141fa7dc0
    Replacement::draft(0x141fa85b0, "unknown_141fa85b0", fn_141fa85b0 as *const ()), // sub_141fa85b0
    Replacement::draft(0x141fa85f0, "unknown_141fa85f0", tpl_42643 as *const ()), // sub_141fa85f0
    Replacement::draft(0x141fa8640, "unknown_141fa8640", fn_141fa8640 as *const ()), // sub_141fa8640
    Replacement::draft(0x141fa86b0, "unknown_141fa86b0", fn_141fa86b0 as *const ()), // sub_141fa86b0
    Replacement::draft(0x141fa8720, "unknown_141fa8720", fn_141fa8720 as *const ()), // sub_141fa8720
    Replacement::draft(0x141fa8760, "unknown_141fa8760", fn_141fa8760 as *const ()), // sub_141fa8760
    Replacement::draft(0x141fa8a30, "unknown_141fa8a30", fn_141fa8a30 as *const ()), // sub_141fa8a30
    Replacement::draft(0x141fa8a60, "unknown_141fa8a60", fn_141fa8a60 as *const ()), // sub_141fa8a60
    Replacement::draft(0x141fa8b70, "unknown_141fa8b70", fn_141fa8b70 as *const ()), // sub_141fa8b70
    Replacement::draft(0x141fa8b90, "unknown_141fa8b90", tpl_42651::<0x8> as *const ()), // sub_141fa8b90
    Replacement::draft(0x141fa8bd0, "unknown_141fa8bd0", fn_141fa8bd0 as *const ()), // sub_141fa8bd0
    Replacement::draft(0x141fa8c50, "unknown_141fa8c50", tpl_19579::<0x143084e28, 0x30> as *const ()), // sub_141fa8c50
    Replacement::draft(0x141fa8c70, "unknown_141fa8c70", tpl_42653::<0x143084e28, 0x30> as *const ()), // sub_141fa8c70
    Replacement::draft(0x141fa8d10, "unknown_141fa8d10", fn_141fa8d10 as *const ()), // sub_141fa8d10
    Replacement::draft(0x141fa8e20, "unknown_141fa8e20", fn_141fa8e20 as *const ()), // sub_141fa8e20
    Replacement::draft(0x141fa8e60, "unknown_141fa8e60", fn_141fa8e60 as *const ()), // sub_141fa8e60
    Replacement::draft(0x141fa8f70, "unknown_141fa8f70", fn_141fa8f70 as *const ()), // sub_141fa8f70
    Replacement::draft(0x141fa8f90, "unknown_141fa8f90", tpl_42657::<0x14241d6f0, 0x1425371a0, 0x142533200> as *const ()), // sub_141fa8f90
    Replacement::draft(0x141fa90a0, "unknown_141fa90a0", fn_141fa90a0 as *const ()), // sub_141fa90a0
    Replacement::draft(0x141fa9190, "unknown_141fa9190", fn_141fa9190 as *const ()), // sub_141fa9190
    Replacement::draft(0x141fa92e0, "unknown_141fa92e0", fn_141fa92e0 as *const ()), // sub_141fa92e0
    Replacement::draft(0x141fa9330, "unknown_141fa9330", tpl_42657::<0x14241d6f0, 0x1425371a0, 0x142533200> as *const ()), // sub_141fa9330
    Replacement::draft(0x141fa94b0, "unknown_141fa94b0", fn_141fa94b0 as *const ()), // sub_141fa94b0
    Replacement::draft(0x141fa95a0, "unknown_141fa95a0", fn_141fa95a0 as *const ()), // sub_141fa95a0
    Replacement::draft(0x141fa96d0, "unknown_141fa96d0", fn_141fa96d0 as *const ()), // sub_141fa96d0
    Replacement::draft(0x141fa9710, "unknown_141fa9710", tpl_42651::<0x8> as *const ()), // sub_141fa9710
    Replacement::draft(0x141fa97a0, "unknown_141fa97a0", tpl_42643 as *const ()), // sub_141fa97a0
    Replacement::draft(0x141fa97d0, "unknown_141fa97d0", fn_141fa97d0 as *const ()), // sub_141fa97d0
    Replacement::draft_throws(0x141fa97f0, "unknown_141fa97f0", fn_141fa97f0 as *const (), Throws::Discard), // sub_141fa97f0
    Replacement::draft_throws(0x141fa9bd0, "unknown_141fa9bd0", fn_141fa9bd0 as *const (), Throws::Caught), // sub_141fa9bd0
    Replacement::draft_throws(0x141faa420, "unknown_141faa420", fn_141faa420 as *const (), Throws::Caught), // sub_141faa420
    Replacement::draft(0x141faade0, "unknown_141faade0", fn_141faade0 as *const ()), // sub_141faade0
    Replacement::draft(0x141fab260, "unknown_141fab260", fn_141fab260 as *const ()), // sub_141fab260
    Replacement::draft(0x141fab700, "unknown_141fab700", fn_141fab700 as *const ()), // sub_141fab700
    Replacement::draft(0x141fab740, "unknown_141fab740", fn_141fab740 as *const ()), // sub_141fab740
    Replacement::draft_throws(0x141fab760, "unknown_141fab760", fn_141fab760 as *const (), Throws::Discard), // sub_141fab760
    Replacement::draft(0x141fabf60, "unknown_141fabf60", fn_141fabf60 as *const ()), // sub_141fabf60
    Replacement::draft(0x141fac070, "unknown_141fac070", fn_141fac070 as *const ()), // sub_141fac070
    Replacement::draft_throws(0x141fac360, "unknown_141fac360", fn_141fac360 as *const (), Throws::Discard), // sub_141fac360
    Replacement::draft(0x141fac520, "unknown_141fac520", fn_141fac520 as *const ()), // sub_141fac520
    Replacement::draft_throws(0x141fac630, "unknown_141fac630", tpl_35009::<0x14387e510, 0x141136c90, 0x1402116e0> as *const (), Throws::Discard), // sub_141fac630
    Replacement::draft(0x141fac840, "unknown_141fac840", tpl_34393 as *const ()), // sub_141fac840
    Replacement::draft(0x141fac8d0, "unknown_141fac8d0", tpl_35403::<0x4> as *const ()), // sub_141fac8d0
    Replacement::draft(0x141fac960, "unknown_141fac960", tpl_35751::<0x20, 0x40, 0x60> as *const ()), // sub_141fac960
    Replacement::draft(0x141fac9a0, "unknown_141fac9a0", tpl_19401::<0x8, 0x141facc80> as *const ()), // sub_141fac9a0
    Replacement::draft(0x141fac9e0, "unknown_141fac9e0", tpl_19401::<0x8, 0x141facd00> as *const ()), // sub_141fac9e0
    Replacement::draft(0x141faca90, "unknown_141faca90", fn_141faca90 as *const ()), // sub_141faca90
    Replacement::draft(0x141facb60, "unknown_141facb60", fn_141facb60 as *const ()), // sub_141facb60
    Replacement::draft(0x141facc80, "unknown_141facc80", tpl_2432::<0x8, 0x141fad000, 0xc, 0x14183acc8> as *const ()), // sub_141facc80
    Replacement::draft(0x141facd00, "unknown_141facd00", tpl_18312::<0x8, 0x141fad0f0, 0xc, 0x18, 0x30, 0xffffffffffffffd0, 0xffffffffffffffd8, 0xffffffffffffffc4, 0xffffffffffffffcc, 0xffffffffffffffdc, 0xffffffffffffffe4> as *const ()), // sub_141facd00
    Replacement::draft(0x141facde0, "unknown_141facde0", tpl_37617::<0x141fadd40, 0x8, 0x14183acc8, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141facde0
    Replacement::draft(0x141facec0, "unknown_141facec0", fn_141facec0 as *const ()), // sub_141facec0
    Replacement::draft(0x141fad000, "unknown_141fad000", tpl_61669::<0xc, 0x141fadd40, 0x8, 0x14183acc8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141fad000
    Replacement::draft(0x141fad0f0, "unknown_141fad0f0", tpl_76689::<0x141fade70, 0x1425371a0, 0x142533200> as *const ()), // sub_141fad0f0
    Replacement::draft(0x141fad240, "unknown_141fad240", tpl_2880::<0x141fae560, 0xc, 0x8, 0x141fad000, 0x14183acc8> as *const ()), // sub_141fad240
    Replacement::draft(0x141fad300, "unknown_141fad300", tpl_19908::<0x141fae620, 0x141fad0f0> as *const ()), // sub_141fad300
    Replacement::draft(0x141fad420, "unknown_141fad420", tpl_21339::<0x141fadd40, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_141fad420
    Replacement::draft(0x141fad5c0, "unknown_141fad5c0", fn_141fad5c0 as *const ()), // sub_141fad5c0
    Replacement::draft(0x141fae0d0, "unknown_141fae0d0", fn_141fae0d0 as *const ()), // sub_141fae0d0
    Replacement::draft(0x141fae140, "unknown_141fae140", tpl_37107::<0x10> as *const ()), // sub_141fae140
    Replacement::draft(0x141fae6e0, "unknown_141fae6e0", tpl_40792 as *const ()), // sub_141fae6e0
    Replacement::draft(0x141fae700, "unknown_141fae700", fn_141fae700 as *const ()), // sub_141fae700
    Replacement::draft(0x141fae740, "unknown_141fae740", fn_141fae740 as *const ()), // sub_141fae740
    Replacement::draft(0x141fae760, "unknown_141fae760", fn_141fae760 as *const ()), // sub_141fae760
    Replacement::draft(0x141fae790, "unknown_141fae790", fn_141fae790 as *const ()), // sub_141fae790
    Replacement::draft(0x141fae7a0, "unknown_141fae7a0", fn_141fae7a0 as *const ()), // sub_141fae7a0
    Replacement::draft(0x141fae820, "unknown_141fae820", fn_141fae820 as *const ()), // sub_141fae820
    Replacement::draft(0x141fae8a0, "unknown_141fae8a0", fn_141fae8a0 as *const ()), // sub_141fae8a0
    Replacement::draft(0x141fae8c0, "unknown_141fae8c0", tpl_4160::<0x8, 0x1411393d0, 0x1411395d0> as *const ()), // sub_141fae8c0
    Replacement::draft(0x141fae930, "unknown_141fae930", fn_141fae930 as *const ()), // sub_141fae930
    Replacement::draft(0x141faea50, "unknown_141faea50", fn_141faea50 as *const ()), // sub_141faea50
    Replacement::draft(0x141faeb80, "unknown_141faeb80", fn_141faeb80 as *const ()), // sub_141faeb80
    Replacement::draft(0x141faf300, "unknown_141faf300", fn_141faf300 as *const ()), // sub_141faf300
    Replacement::draft(0x141faf5e0, "anonymous_namespace_141faf5e0", fn_141faf5e0 as *const ()), // sub_141faf5e0
    Replacement::draft(0x141fb08a0, "anonymous_namespace_141fb08a0", fn_141fb08a0 as *const ()), // sub_141fb08a0
    Replacement::draft(0x141fb1210, "anonymous_namespace_141fb1210", fn_141fb1210 as *const ()), // sub_141fb1210
    Replacement::draft(0x141fb1540, "anonymous_namespace_141fb1540", fn_141fb1540 as *const ()), // sub_141fb1540
    Replacement::draft(0x141fb1940, "anonymous_namespace_141fb1940", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ce09f0, 0x158, 0x14020e8f0> as *const ()), // sub_141fb1940
    Replacement::draft(0x141fb1a40, "anonymous_namespace_141fb1a40", fn_141fb1a40 as *const ()), // sub_141fb1a40
    Replacement::draft(0x141fb1be0, "anonymous_namespace_141fb1be0", fn_141fb1be0 as *const ()), // sub_141fb1be0
    Replacement::draft(0x141fb1c50, "anonymous_namespace_141fb1c50", fn_141fb1c50 as *const ()), // sub_141fb1c50
    Replacement::draft(0x141fb1cc0, "anonymous_namespace_141fb1cc0", fn_141fb1cc0 as *const ()), // sub_141fb1cc0
    Replacement::draft(0x141fb1f90, "anonymous_namespace_141fb1f90", fn_141fb1f90 as *const ()), // sub_141fb1f90
    Replacement::draft(0x141fb2050, "anonymous_namespace_141fb2050", fn_141fb2050 as *const ()), // sub_141fb2050
    Replacement::draft(0x141fb2230, "anonymous_namespace_141fb2230", fn_141fb2230 as *const ()), // sub_141fb2230
    Replacement::draft(0x141fb2280, "anonymous_namespace_141fb2280", fn_141fb2280 as *const ()), // sub_141fb2280
    Replacement::draft(0x141fb22f0, "anonymous_namespace_141fb22f0", fn_141fb22f0 as *const ()), // sub_141fb22f0
    Replacement::draft(0x141fb2390, "anonymous_namespace_141fb2390", tpl_41481::<0x14397fe70, 0x34, 0x18, 0x10, 0x8, 0xac> as *const ()), // anonymous_namespace::LookAtIgnoreFunc::anonymous_namespace::LookAtIgnoreFunc::vfunc_1
    Replacement::draft_throws(0x141fb23e0, "unknown_141fb23e0", fn_141fb23e0 as *const (), Throws::Discard), // sub_141fb23e0
    Replacement::draft(0x141fb2de0, "unknown_141fb2de0", fn_141fb2de0 as *const ()), // sub_141fb2de0
    Replacement::draft(0x141fb2e80, "unknown_141fb2e80", fn_141fb2e80 as *const ()), // sub_141fb2e80
    Replacement::draft(0x141fb3300, "unknown_141fb3300", fn_141fb3300 as *const ()), // sub_141fb3300
    Replacement::draft(0x141fb33e0, "unknown_141fb33e0", fn_141fb33e0 as *const ()), // sub_141fb33e0
    Replacement::draft(0x141fb34f0, "unknown_141fb34f0", fn_141fb34f0 as *const ()), // sub_141fb34f0
    Replacement::draft(0x141fb3690, "unknown_141fb3690", fn_141fb3690 as *const ()), // sub_141fb3690
    Replacement::draft(0x141fb3d60, "unknown_141fb3d60", tpl_21227::<0x18, 0x8, 0x14113f8d0> as *const ()), // sub_141fb3d60
    Replacement::draft(0x141fb3da0, "unknown_141fb3da0", fn_141fb3da0 as *const ()), // sub_141fb3da0
    Replacement::draft(0x141fb3e50, "unknown_141fb3e50", fn_141fb3e50 as *const ()), // sub_141fb3e50
    Replacement::draft(0x141fb49a0, "unknown_141fb49a0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ce1318, 0x158, 0x14020e8f0> as *const ()), // sub_141fb49a0
    Replacement::draft_throws(0x141fb4aa0, "unknown_141fb4aa0", fn_141fb4aa0 as *const (), Throws::Caught), // sub_141fb4aa0
    Replacement::draft_throws(0x141fb4f60, "unknown_141fb4f60", fn_141fb4f60 as *const (), Throws::Discard), // sub_141fb4f60
    Replacement::draft(0x141fb5ee0, "unknown_141fb5ee0", fn_141fb5ee0 as *const ()), // sub_141fb5ee0
    Replacement::draft(0x141fb5fd0, "unknown_141fb5fd0", fn_141fb5fd0 as *const ()), // sub_141fb5fd0
    Replacement::draft(0x141fb6310, "unknown_141fb6310", fn_141fb6310 as *const ()), // sub_141fb6310
    Replacement::draft(0x141fb63b0, "unknown_141fb63b0", fn_141fb63b0 as *const ()), // sub_141fb63b0
    Replacement::draft(0x141fb64b0, "unknown_141fb64b0", fn_141fb64b0 as *const ()), // sub_141fb64b0
    Replacement::draft(0x141fb6960, "unknown_141fb6960", fn_141fb6960 as *const ()), // sub_141fb6960
    Replacement::draft(0x141fb69d0, "unknown_141fb69d0", fn_141fb69d0 as *const ()), // sub_141fb69d0
    Replacement::draft(0x141fb6a90, "unknown_141fb6a90", fn_141fb6a90 as *const ()), // sub_141fb6a90
    Replacement::draft(0x141fb6c10, "unknown_141fb6c10", fn_141fb6c10 as *const ()), // sub_141fb6c10
    Replacement::draft_throws(0x141fb6e10, "unknown_141fb6e10", fn_141fb6e10 as *const (), Throws::Caught), // sub_141fb6e10
    Replacement::draft(0x141fb7000, "unknown_141fb7000", fn_141fb7000 as *const ()), // sub_141fb7000
    Replacement::draft_throws(0x141fb72f0, "unknown_141fb72f0", fn_141fb72f0 as *const (), Throws::Discard), // sub_141fb72f0
    Replacement::draft(0x141fb7cd0, "unknown_141fb7cd0", fn_141fb7cd0 as *const ()), // sub_141fb7cd0
    Replacement::draft(0x141fb7d60, "unknown_141fb7d60", fn_141fb7d60 as *const ()), // sub_141fb7d60
    Replacement::draft(0x141fb8060, "unknown_141fb8060", fn_141fb8060 as *const ()), // sub_141fb8060
    Replacement::draft(0x141fb80f0, "unknown_141fb80f0", fn_141fb80f0 as *const ()), // sub_141fb80f0
    Replacement::draft(0x141fb8340, "unknown_141fb8340", fn_141fb8340 as *const ()), // sub_141fb8340
    Replacement::draft(0x141fb8ba0, "unknown_141fb8ba0", fn_141fb8ba0 as *const ()), // sub_141fb8ba0
    Replacement::draft(0x141fb8c00, "unknown_141fb8c00", fn_141fb8c00 as *const ()), // sub_141fb8c00
    Replacement::draft(0x141fb8d40, "unknown_141fb8d40", fn_141fb8d40 as *const ()), // sub_141fb8d40
    Replacement::draft_throws(0x141fb8dc0, "unknown_141fb8dc0", tpl_37596::<0x142942510, 0x1410d6ca0, 0x14114a130, 0x143823ae8, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141fb8dc0
    Replacement::draft(0x141fb9160, "unknown_141fb9160", fn_141fb9160 as *const ()), // sub_141fb9160
    Replacement::draft(0x141fb9310, "unknown_141fb9310", fn_141fb9310 as *const ()), // sub_141fb9310
    Replacement::draft(0x141fb9650, "unknown_141fb9650", tpl_42711::<0x14310a8c8, 0x14248a940, 0x1434268d0> as *const ()), // sub_141fb9650
    Replacement::draft(0x141fb9690, "unknown_141fb9690", tpl_42711::<0x14310a8f0, 0x14248a940, 0x143427080> as *const ()), // sub_141fb9690
    Replacement::draft(0x141fb96d0, "unknown_141fb96d0", fn_141fb96d0 as *const ()), // sub_141fb96d0
    Replacement::draft(0x141fb9ac0, "unknown_141fb9ac0", tpl_42712::<0x143a2af9c, 0x1420e8920, 0x5aa0> as *const ()), // sub_141fb9ac0
    Replacement::draft(0x141fb9b00, "unknown_141fb9b00", fn_141fb9b00 as *const ()), // sub_141fb9b00
    Replacement::draft(0x141fb9be0, "unknown_141fb9be0", fn_141fb9be0 as *const ()), // sub_141fb9be0
    Replacement::draft(0x141fb9f10, "unknown_141fb9f10", fn_141fb9f10 as *const ()), // sub_141fb9f10
    Replacement::draft(0x141fba070, "unknown_141fba070", fn_141fba070 as *const ()), // sub_141fba070
    Replacement::draft(0x141fba1e0, "unknown_141fba1e0", fn_141fba1e0 as *const ()), // sub_141fba1e0
    Replacement::draft(0x141fba560, "unknown_141fba560", fn_141fba560 as *const ()), // sub_141fba560
    Replacement::draft(0x141fba5f0, "unknown_141fba5f0", tpl_42712::<0x143a2af9c, 0x1420e8920, 0x5aa0> as *const ()), // sub_141fba5f0
    Replacement::draft(0x141fba630, "unknown_141fba630", fn_141fba630 as *const ()), // sub_141fba630
    Replacement::draft(0x141fba720, "unknown_141fba720", fn_141fba720 as *const ()), // sub_141fba720
    Replacement::draft(0x141fba790, "unknown_141fba790", fn_141fba790 as *const ()), // sub_141fba790
    Replacement::draft(0x141fba850, "unknown_141fba850", fn_141fba850 as *const ()), // sub_141fba850
    Replacement::draft(0x141fba9f0, "unknown_141fba9f0", tpl_42721::<0x1438546b8> as *const ()), // sub_141fba9f0
    Replacement::draft(0x141fbaa00, "unknown_141fbaa00", fn_141fbaa00 as *const ()), // sub_141fbaa00
    Replacement::draft(0x141fbad10, "unknown_141fbad10", fn_141fbad10 as *const ()), // sub_141fbad10
    Replacement::draft(0x141fbad90, "unknown_141fbad90", fn_141fbad90 as *const ()), // sub_141fbad90
    Replacement::draft(0x141fbafa0, "unknown_141fbafa0", tpl_42721::<0x1438546bc> as *const ()), // sub_141fbafa0
    Replacement::draft(0x141fbafb0, "unknown_141fbafb0", fn_141fbafb0 as *const ()), // sub_141fbafb0
    Replacement::draft(0x141fbafe0, "unknown_141fbafe0", fn_141fbafe0 as *const ()), // sub_141fbafe0
    Replacement::draft(0x141fbb040, "unknown_141fbb040", fn_141fbb040 as *const ()), // sub_141fbb040
    Replacement::draft(0x141fbb1c0, "unknown_141fbb1c0", fn_141fbb1c0 as *const ()), // sub_141fbb1c0
    Replacement::draft(0x141fbb2b0, "unknown_141fbb2b0", fn_141fbb2b0 as *const ()), // sub_141fbb2b0
    Replacement::draft(0x141fbb490, "unknown_141fbb490", fn_141fbb490 as *const ()), // sub_141fbb490
    Replacement::draft(0x141fbb4d0, "unknown_141fbb4d0", fn_141fbb4d0 as *const ()), // sub_141fbb4d0
    Replacement::draft_throws(0x141fbb4f0, "unknown_141fbb4f0", fn_141fbb4f0 as *const (), Throws::Discard), // sub_141fbb4f0
    Replacement::draft(0x141fbbc20, "unknown_141fbbc20", fn_141fbbc20 as *const ()), // sub_141fbbc20
    Replacement::draft_throws(0x141fbbc70, "unknown_141fbbc70", fn_141fbbc70 as *const (), Throws::Caught), // sub_141fbbc70
    Replacement::draft_throws(0x141fbc3b0, "unknown_141fbc3b0", fn_141fbc3b0 as *const (), Throws::Caught), // sub_141fbc3b0
    Replacement::draft(0x141fbc4c0, "unknown_141fbc4c0", fn_141fbc4c0 as *const ()), // sub_141fbc4c0
    Replacement::draft_throws(0x141fbc500, "unknown_141fbc500", fn_141fbc500 as *const (), Throws::Caught), // sub_141fbc500
    Replacement::draft(0x141fbc650, "unknown_141fbc650", fn_141fbc650 as *const ()), // sub_141fbc650
    Replacement::draft(0x141fbc6f0, "unknown_141fbc6f0", fn_141fbc6f0 as *const ()), // sub_141fbc6f0
    Replacement::draft(0x141fbc790, "unknown_141fbc790", fn_141fbc790 as *const ()), // sub_141fbc790
    Replacement::draft(0x141fbc830, "unknown_141fbc830", fn_141fbc830 as *const ()), // sub_141fbc830
    Replacement::draft(0x141fbc8d0, "unknown_141fbc8d0", fn_141fbc8d0 as *const ()), // sub_141fbc8d0
    Replacement::draft_throws(0x141fbc970, "unknown_141fbc970", fn_141fbc970 as *const (), Throws::Caught), // sub_141fbc970
    Replacement::draft_throws(0x141fbca30, "unknown_141fbca30", fn_141fbca30 as *const (), Throws::Caught), // sub_141fbca30
    Replacement::draft_throws(0x141fbcad0, "unknown_141fbcad0", fn_141fbcad0 as *const (), Throws::Caught), // sub_141fbcad0
    Replacement::draft(0x141fbcb60, "unknown_141fbcb60", fn_141fbcb60 as *const ()), // sub_141fbcb60
    Replacement::draft(0x141fbcc10, "unknown_141fbcc10", tpl_102598::<0x1430782a0, 0x8, 0x10, 0x18, 0x28> as *const ()), // sub_141fbcc10
    Replacement::draft(0x141fbcd00, "unknown_141fbcd00", tpl_42742::<0x1430782a0, 0x8, 0x10, 0x18, 0x20, 0x28> as *const ()), // sub_141fbcd00
    Replacement::draft(0x141fbcd70, "unknown_141fbcd70", tpl_35320::<0x1401de4e0> as *const ()), // sub_141fbcd70
    Replacement::draft(0x141fbce20, "unknown_141fbce20", tpl_102598::<0x1430782a0, 0x8, 0x10, 0x18, 0x28> as *const ()), // sub_141fbce20
    Replacement::draft(0x141fbcf10, "unknown_141fbcf10", tpl_42742::<0x1430782a0, 0x8, 0x10, 0x18, 0x20, 0x28> as *const ()), // sub_141fbcf10
    Replacement::draft(0x141fbcf80, "unknown_141fbcf80", tpl_35320::<0x1401de4e0> as *const ()), // sub_141fbcf80
    Replacement::draft_throws(0x141fbd2e0, "unknown_141fbd2e0", fn_141fbd2e0 as *const (), Throws::Caught), // sub_141fbd2e0
    Replacement::draft_throws(0x141fbd7a0, "unknown_141fbd7a0", fn_141fbd7a0 as *const (), Throws::Caught), // sub_141fbd7a0
    Replacement::draft_throws(0x141fbda20, "unknown_141fbda20", fn_141fbda20 as *const (), Throws::Discard), // sub_141fbda20
    Replacement::draft(0x141fbdd70, "unknown_141fbdd70", fn_141fbdd70 as *const ()), // sub_141fbdd70
    Replacement::draft(0x141fbded0, "unknown_141fbded0", fn_141fbded0 as *const ()), // sub_141fbded0
    Replacement::draft(0x141fbe140, "unknown_141fbe140", fn_141fbe140 as *const ()), // sub_141fbe140
    Replacement::draft(0x141fbe280, "unknown_141fbe280", fn_141fbe280 as *const ()), // sub_141fbe280
    Replacement::draft_throws(0x141fbe6b0, "unknown_141fbe6b0", fn_141fbe6b0 as *const (), Throws::Caught), // sub_141fbe6b0
    Replacement::draft_throws(0x141fbe820, "unknown_141fbe820", fn_141fbe820 as *const (), Throws::Discard), // sub_141fbe820
    Replacement::draft_throws(0x141fbedb0, "unknown_141fbedb0", fn_141fbedb0 as *const (), Throws::Discard), // sub_141fbedb0
    Replacement::draft(0x141fbee20, "unknown_141fbee20", fn_141fbee20 as *const ()), // sub_141fbee20
    Replacement::draft_throws(0x141fbef60, "unknown_141fbef60", fn_141fbef60 as *const (), Throws::Caught), // sub_141fbef60
    Replacement::draft_throws(0x141fbf210, "unknown_141fbf210", fn_141fbf210 as *const (), Throws::Discard), // sub_141fbf210
    Replacement::draft_throws(0x141fbf460, "unknown_141fbf460", fn_141fbf460 as *const (), Throws::Discard), // sub_141fbf460
    Replacement::draft(0x141fbf580, "unknown_141fbf580", fn_141fbf580 as *const ()), // sub_141fbf580
    Replacement::draft(0x141fbf5f0, "unknown_141fbf5f0", fn_141fbf5f0 as *const ()), // sub_141fbf5f0
    Replacement::draft_throws(0x141fbf670, "unknown_141fbf670", fn_141fbf670 as *const (), Throws::Discard), // sub_141fbf670
    Replacement::draft_throws(0x141fbf6a0, "unknown_141fbf6a0", fn_141fbf6a0 as *const (), Throws::Discard), // sub_141fbf6a0
    Replacement::draft_throws(0x141fbf760, "coregame_141fbf760", fn_141fbf760 as *const (), Throws::Discard), // coregame::BundleLoadingLogic::coregame::BundleLoadingLogic::vfunc_6
    Replacement::draft_throws(0x141fbf800, "coregame_141fbf800", fn_141fbf800 as *const (), Throws::Discard), // coregame::BundleLoadingLogic::coregame::BundleLoadingLogic::vfunc_5
    Replacement::draft_throws(0x141fbf8b0, "coregame_141fbf8b0", fn_141fbf8b0 as *const (), Throws::Caught), // sub_141fbf8b0
    Replacement::draft_throws(0x141fbfb50, "coregame_141fbfb50", fn_141fbfb50 as *const (), Throws::Caught), // coregame::BundleLoadingLogic::coregame::BundleLoadingLogic::vfunc_2
    Replacement::draft(0x141fc0180, "coregame_141fc0180", fn_141fc0180 as *const ()), // sub_141fc0180
    Replacement::draft(0x141fc01d0, "coregame_141fc01d0", fn_141fc01d0 as *const ()), // sub_141fc01d0
    Replacement::draft_throws(0x141fc0340, "coregame_141fc0340", fn_141fc0340 as *const (), Throws::Caught), // coregame::BundleLoadingLogic::coregame::BundleLoadingLogic::vfunc_4
    Replacement::draft_throws(0x141fc04c0, "coregame_141fc04c0", fn_141fc04c0 as *const (), Throws::Discard), // coregame::BundleLoadingLogic::coregame::BundleLoadingLogic::vfunc_3
    Replacement::draft(0x141fc06a0, "std_141fc06a0", tpl_770::<0x1433b84a8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleLoader::createEntities::<lambda_1>,void,ecs::CommandBuffer,?bref6::FutureEntity
    Replacement::draft(0x141fc06c0, "std_141fc06c0", tpl_770::<0x1433b84a8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleLoader::createEntities::<lambda_1>,void,ecs::CommandBuffer,?bref6::FutureEntity
    Replacement::draft(0x141fc06e0, "std_141fc06e0", fn_141fc06e0 as *const ()), // std::_Func_impl_no_alloc<coregame::BundleLoader::createEntities::<lambda_1>,void,ecs::CommandBuffer,?bref6::FutureEntity
    Replacement::draft(0x141fc0720, "std_141fc0720", tpl_1::<0x143891740> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleLoader::createEntities::<lambda_1>,void,ecs::CommandBuffer,?bref6::FutureEntity
    Replacement::draft(0x141fc0730, "std_141fc0730", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleLoader::createEntities::<lambda_1>,void,ecs::CommandBuffer,?bref6::FutureEntity
    Replacement::draft(0x141fc0740, "std_141fc0740", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleLoader::createEntities::<lambda_1>,void,ecs::CommandBuffer,?bref6::FutureEntity
    Replacement::draft(0x141fc0990, "unknown_141fc0990", fn_141fc0990 as *const ()), // sub_141fc0990
    Replacement::draft(0x141fc09c0, "unknown_141fc09c0", fn_141fc09c0 as *const ()), // sub_141fc09c0
    Replacement::draft(0x141fc0a10, "unknown_141fc0a10", fn_141fc0a10 as *const ()), // sub_141fc0a10
    Replacement::draft_throws(0x141fc0a60, "unknown_141fc0a60", fn_141fc0a60 as *const (), Throws::Discard), // sub_141fc0a60
    Replacement::draft(0x141fc0af0, "coregame_141fc0af0", fn_141fc0af0 as *const ()), // coregame::BundleLoader::coregame::BundleLoader::vfunc_3
    Replacement::draft(0x141fc0b20, "coregame_141fc0b20", fn_141fc0b20 as *const ()), // coregame::BundleLoader::coregame::BundleLoader::vfunc_0
    Replacement::draft_throws(0x141fc0b30, "unknown_141fc0b30", fn_141fc0b30 as *const (), Throws::Discard), // sub_141fc0b30
    Replacement::draft(0x141fc0b80, "unknown_141fc0b80", fn_141fc0b80 as *const ()), // sub_141fc0b80
    Replacement::draft(0x141fc0b90, "unknown_141fc0b90", tpl_42777::<0x143a08298, 0x1433b8520, 0x1401de410, 0x299, 0x142417470, 0x260, 0x14246ff60, 0x8, 0xfffffffffffffff8, 0x280, 0x288, 0x250, 0x1426455a8, 0x142645598, 0x142645590> as *const ()), // sub_141fc0b90
    Replacement::draft(0x141fc0c80, "unknown_141fc0c80", tpl_41168::<0x14397da68, 0x10, 0x8, 0x142407c80, 0x580f0, 0x143c8d0cc, 0x20, 0x141f79f70> as *const ()), // sub_141fc0c80
    Replacement::draft_throws(0x141fc0d30, "unknown_141fc0d30", fn_141fc0d30 as *const (), Throws::Caught), // sub_141fc0d30
    Replacement::draft(0x141fc0dd0, "unknown_141fc0dd0", tpl_42777::<0x143a08298, 0x1433b8500, 0x1401de410, 0x299, 0x142417470, 0x260, 0x14246ff60, 0x8, 0xfffffffffffffff8, 0x280, 0x288, 0x250, 0x1426455a8, 0x142645598, 0x142645590> as *const ()), // sub_141fc0dd0
    Replacement::draft_throws(0x141fc0ec0, "unknown_141fc0ec0", fn_141fc0ec0 as *const (), Throws::Discard), // sub_141fc0ec0
    Replacement::draft_throws(0x141fc0f40, "unknown_141fc0f40", fn_141fc0f40 as *const (), Throws::Discard), // sub_141fc0f40
    Replacement::draft_throws(0x141fc0f60, "unknown_141fc0f60", fn_141fc0f60 as *const (), Throws::Caught), // sub_141fc0f60
    Replacement::draft(0x141fc0ff0, "unknown_141fc0ff0", fn_141fc0ff0 as *const ()), // sub_141fc0ff0
    Replacement::draft_throws(0x141fc1020, "unknown_141fc1020", fn_141fc1020 as *const (), Throws::Discard), // sub_141fc1020
    Replacement::draft_throws(0x141fc1060, "unknown_141fc1060", fn_141fc1060 as *const (), Throws::Caught), // sub_141fc1060
    Replacement::draft_throws(0x141fc1430, "unknown_141fc1430", fn_141fc1430 as *const (), Throws::Caught), // sub_141fc1430
    Replacement::draft(0x141fc18d0, "unknown_141fc18d0", fn_141fc18d0 as *const ()), // sub_141fc18d0
    Replacement::draft(0x141fc1900, "unknown_141fc1900", tpl_36673 as *const ()), // sub_141fc1900
    Replacement::draft(0x141fc1910, "unknown_141fc1910", tpl_40240 as *const ()), // sub_141fc1910
    Replacement::draft_throws(0x141fc1920, "unknown_141fc1920", fn_141fc1920 as *const (), Throws::Discard), // sub_141fc1920
    Replacement::draft(0x141fc2230, "unknown_141fc2230", tpl_369 as *const ()), // sub_141fc2230
    Replacement::draft(0x141fc2240, "unknown_141fc2240", tpl_369 as *const ()), // sub_141fc2240
    Replacement::draft(0x141fc2250, "unknown_141fc2250", fn_141fc2250 as *const ()), // sub_141fc2250
    Replacement::draft(0x141fc2320, "unknown_141fc2320", tpl_369 as *const ()), // sub_141fc2320
    Replacement::draft(0x141fc2330, "unknown_141fc2330", tpl_369 as *const ()), // sub_141fc2330
    Replacement::draft(0x141fc2340, "unknown_141fc2340", fn_141fc2340 as *const ()), // sub_141fc2340
    Replacement::draft(0x141fc2360, "unknown_141fc2360", fn_141fc2360 as *const ()), // sub_141fc2360
    Replacement::draft(0x141fc2400, "unknown_141fc2400", fn_141fc2400 as *const ()), // sub_141fc2400
    Replacement::draft(0x141fc2460, "unknown_141fc2460", fn_141fc2460 as *const ()), // sub_141fc2460
    Replacement::draft(0x141fc2690, "unknown_141fc2690", fn_141fc2690 as *const ()), // sub_141fc2690
    Replacement::draft(0x141fc28b0, "std_141fc28b0", tpl_550::<0x1433b8790, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_1>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc28d0, "std_141fc28d0", tpl_550::<0x1433b8790, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_1>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2900, "std_141fc2900", tpl_1::<0x143891820> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_1>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2910, "std_141fc2910", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_1>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2920, "std_141fc2920", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_1>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2940, "std_141fc2940", tpl_550::<0x1433b86e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_2>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2960, "std_141fc2960", tpl_550::<0x1433b86e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_2>,void,?bref6::FileStream&>::std::_
    Replacement::draft_throws(0x141fc2980, "std_141fc2980", tpl_42793::<0x8, 0x143399400, 0x148, 0x40, 0x1410f0ca0, 0xa0, 0x100, 0x1433993e0, 0x1410f1080, 0x108> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_2>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc29e0, "std_141fc29e0", tpl_1::<0x1438918a0> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_2>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc29f0, "std_141fc29f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_2>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2a00, "std_141fc2a00", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_2>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2a20, "std_141fc2a20", tpl_550::<0x1433b8720, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_3>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2a40, "std_141fc2a40", tpl_550::<0x1433b8720, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_3>,void,?bref6::FileStream&>::std::_
    Replacement::draft_throws(0x141fc2a60, "std_141fc2a60", tpl_42793::<0x8, 0x143399400, 0x260, 0x158, 0x1410f0ca0, 0x1b8, 0x218, 0x1433993e0, 0x1410f1080, 0x220> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_3>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2ad0, "std_141fc2ad0", tpl_1::<0x143891a40> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_3>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2ae0, "std_141fc2ae0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_3>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2af0, "std_141fc2af0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_3>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2b10, "std_141fc2b10", tpl_550::<0x1433b87c8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>::operator()::<lambda_1>,void,?bre
    Replacement::draft(0x141fc2b30, "std_141fc2b30", tpl_550::<0x1433b87c8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>::operator()::<lambda_1>,void,?bre
    Replacement::draft_throws(0x141fc2b50, "std_141fc2b50", tpl_20905::<0x141fc2d90> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>::operator()::<lambda_1>,void,?bre
    Replacement::draft(0x141fc2b80, "std_141fc2b80", tpl_1::<0x143891920> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>::operator()::<lambda_1>,void,?bre
    Replacement::draft(0x141fc2b90, "std_141fc2b90", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>::operator()::<lambda_1>,void,?bre
    Replacement::draft(0x141fc2ba0, "std_141fc2ba0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>::operator()::<lambda_1>,void,?bre
    Replacement::draft(0x141fc2bc0, "std_141fc2bc0", tpl_550::<0x1433b8808, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2be0, "std_141fc2be0", tpl_550::<0x1433b8808, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>,void,?bref6::FileStream&>::std::_
    Replacement::draft_throws(0x141fc2c00, "std_141fc2c00", fn_141fc2c00 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2c60, "std_141fc2c60", tpl_1::<0x1438917a0> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2c70, "std_141fc2c70", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2c80, "std_141fc2c80", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_4>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2ca0, "std_141fc2ca0", tpl_550::<0x1433b8758, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_5>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2cc0, "std_141fc2cc0", tpl_550::<0x1433b8758, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_5>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2ce0, "std_141fc2ce0", fn_141fc2ce0 as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_5>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2d00, "std_141fc2d00", tpl_1::<0x1438919c0> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_5>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2d10, "std_141fc2d10", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_5>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2d20, "std_141fc2d20", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::BundleContentLoader::streamerFileLoaded::<lambda_5>,void,?bref6::FileStream&>::std::_
    Replacement::draft(0x141fc2d40, "unknown_141fc2d40", tpl_34443 as *const ()), // sub_141fc2d40
    Replacement::draft(0x141fc2d50, "unknown_141fc2d50", fn_141fc2d50 as *const ()), // sub_141fc2d50
    Replacement::draft(0x141fc2d70, "unknown_141fc2d70", tpl_34443 as *const ()), // sub_141fc2d70
    Replacement::draft(0x141fc2d80, "unknown_141fc2d80", tpl_34443 as *const ()), // sub_141fc2d80
    Replacement::draft_throws(0x141fc2d90, "unknown_141fc2d90", fn_141fc2d90 as *const (), Throws::Caught), // sub_141fc2d90
    Replacement::draft_throws(0x141fc31b0, "unknown_141fc31b0", fn_141fc31b0 as *const (), Throws::Discard), // sub_141fc31b0
    Replacement::draft(0x141fc3200, "unknown_141fc3200", tpl_34443 as *const ()), // sub_141fc3200
    Replacement::draft_throws(0x141fc3210, "unknown_141fc3210", tpl_42799::<0x143399400, 0x260, 0x158, 0x1410f0ca0, 0x1b8, 0x218, 0x1433993e0, 0x1410f1080, 0x220> as *const (), Throws::Discard), // sub_141fc3210
    Replacement::draft_throws(0x141fc3290, "unknown_141fc3290", tpl_42799::<0x143399400, 0x148, 0x40, 0x1410f0ca0, 0xa0, 0x100, 0x1433993e0, 0x1410f1080, 0x108> as *const (), Throws::Discard), // sub_141fc3290
    Replacement::draft_throws(0x141fc3300, "unknown_141fc3300", fn_141fc3300 as *const (), Throws::Caught), // sub_141fc3300
    Replacement::draft_throws(0x141fc34e0, "unknown_141fc34e0", fn_141fc34e0 as *const (), Throws::Discard), // sub_141fc34e0
    Replacement::draft_throws(0x141fc3560, "coregame_141fc3560", fn_141fc3560 as *const (), Throws::Discard), // coregame::BundleContentLoader::coregame::BundleContentLoader::vfunc_2
    Replacement::draft_throws(0x141fc3630, "coregame_141fc3630", fn_141fc3630 as *const (), Throws::Discard), // coregame::BundleContentLoader::coregame::BundleContentLoader::vfunc_1
    Replacement::draft(0x141fc39c0, "coregame_141fc39c0", fn_141fc39c0 as *const ()), // sub_141fc39c0
    Replacement::draft(0x141fc3b50, "coregame_141fc3b50", fn_141fc3b50 as *const ()), // sub_141fc3b50
    Replacement::draft(0x141fc3c60, "coregame_141fc3c60", tpl_95295::<0x1432923c8, 0x8, 0x10, 0x14> as *const ()), // sub_141fc3c60
    Replacement::draft(0x141fc3e40, "coregame_141fc3e40", fn_141fc3e40 as *const ()), // sub_141fc3e40
    Replacement::draft(0x141fc3fd0, "unknown_141fc3fd0", tpl_40270::<0x14115b460> as *const ()), // sub_141fc3fd0
    Replacement::draft(0x141fc3ff0, "unknown_141fc3ff0", fn_141fc3ff0 as *const ()), // sub_141fc3ff0
    Replacement::draft(0x141fc4030, "unknown_141fc4030", tpl_34994::<0x143ce2484, 0x1438812a8, 0x1438812a0, 0x141834a78, 0x142616c40, 0x141834d18, 0x141834a0c> as *const ()), // sub_141fc4030
    Replacement::draft_throws(0x141fc40a0, "unknown_141fc40a0", fn_141fc40a0 as *const (), Throws::Discard), // sub_141fc40a0
    Replacement::draft(0x141fc41c0, "unknown_141fc41c0", fn_141fc41c0 as *const ()), // sub_141fc41c0
    Replacement::draft(0x141fc4360, "unknown_141fc4360", fn_141fc4360 as *const ()), // sub_141fc4360
    Replacement::draft(0x141fc4380, "unknown_141fc4380", fn_141fc4380 as *const ()), // sub_141fc4380
    Replacement::draft(0x141fc4400, "unknown_141fc4400", tpl_34994::<0x143ce2438, 0x143881298, 0x143881290, 0x141834a78, 0x142616c50, 0x141834d18, 0x141834a0c> as *const ()), // sub_141fc4400
    Replacement::draft(0x141fc4470, "unknown_141fc4470", fn_141fc4470 as *const ()), // sub_141fc4470
    Replacement::draft(0x141fc4590, "unknown_141fc4590", tpl_35325::<0x141158890> as *const ()), // sub_141fc4590
    Replacement::draft(0x141fc45b0, "unknown_141fc45b0", fn_141fc45b0 as *const ()), // sub_141fc45b0
    Replacement::draft(0x141fc4620, "unknown_141fc4620", tpl_34318 as *const ()), // sub_141fc4620
    Replacement::draft(0x141fc4640, "unknown_141fc4640", tpl_34319::<0x143c9a318, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4640
    Replacement::draft(0x141fc46a0, "unknown_141fc46a0", tpl_34318 as *const ()), // sub_141fc46a0
    Replacement::draft(0x141fc46c0, "unknown_141fc46c0", tpl_34318 as *const ()), // sub_141fc46c0
    Replacement::draft(0x141fc46e0, "unknown_141fc46e0", tpl_34319::<0x143ce2434, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc46e0
    Replacement::draft(0x141fc4740, "unknown_141fc4740", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4740
    Replacement::draft(0x141fc47a0, "unknown_141fc47a0", tpl_34318 as *const ()), // sub_141fc47a0
    Replacement::draft(0x141fc47c0, "unknown_141fc47c0", tpl_34318 as *const ()), // sub_141fc47c0
    Replacement::draft(0x141fc47e0, "unknown_141fc47e0", tpl_34318 as *const ()), // sub_141fc47e0
    Replacement::draft(0x141fc4800, "unknown_141fc4800", tpl_34318 as *const ()), // sub_141fc4800
    Replacement::draft(0x141fc4820, "unknown_141fc4820", tpl_34319::<0x143c8dbc8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4820
    Replacement::draft(0x141fc4880, "unknown_141fc4880", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4880
    Replacement::draft(0x141fc48e0, "unknown_141fc48e0", tpl_34319::<0x143ce2638, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc48e0
    Replacement::draft(0x141fc4940, "unknown_141fc4940", tpl_34319::<0x143cd6de0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4940
    Replacement::draft(0x141fc49a0, "unknown_141fc49a0", tpl_34318 as *const ()), // sub_141fc49a0
    Replacement::draft(0x141fc49c0, "unknown_141fc49c0", tpl_34318 as *const ()), // sub_141fc49c0
    Replacement::draft(0x141fc49e0, "unknown_141fc49e0", tpl_34318 as *const ()), // sub_141fc49e0
    Replacement::draft(0x141fc4a00, "unknown_141fc4a00", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4a00
    Replacement::draft(0x141fc4a60, "unknown_141fc4a60", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4a60
    Replacement::draft(0x141fc4ac0, "unknown_141fc4ac0", tpl_34319::<0x143ce22c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4ac0
    Replacement::draft(0x141fc4b20, "unknown_141fc4b20", tpl_34318 as *const ()), // sub_141fc4b20
    Replacement::draft(0x141fc4b40, "unknown_141fc4b40", tpl_34318 as *const ()), // sub_141fc4b40
    Replacement::draft(0x141fc4b60, "unknown_141fc4b60", tpl_34319::<0x143ce22c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4b60
    Replacement::draft(0x141fc4bc0, "unknown_141fc4bc0", tpl_34319::<0x143c9a620, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141fc4bc0
    Replacement::draft(0x141fc4c20, "unknown_141fc4c20", tpl_361 as *const ()), // sub_141fc4c20
    Replacement::draft(0x141fc4c70, "unknown_141fc4c70", fn_141fc4c70 as *const ()), // sub_141fc4c70
    Replacement::draft(0x141fc5120, "unknown_141fc5120", tpl_361 as *const ()), // sub_141fc5120
    Replacement::draft(0x141fc5130, "unknown_141fc5130", fn_141fc5130 as *const ()), // sub_141fc5130
    Replacement::draft(0x141fc51a0, "unknown_141fc51a0", fn_141fc51a0 as *const ()), // sub_141fc51a0
    Replacement::draft(0x141fc59d0, "unknown_141fc59d0", tpl_361 as *const ()), // sub_141fc59d0
    Replacement::draft_throws(0x141fc59e0, "unknown_141fc59e0", fn_141fc59e0 as *const (), Throws::Discard), // sub_141fc59e0
    Replacement::draft(0x141fc5eb0, "unknown_141fc5eb0", tpl_361 as *const ()), // sub_141fc5eb0
    Replacement::draft(0x141fc5ec0, "unknown_141fc5ec0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ce2638, 0x158, 0x14020e8f0> as *const ()), // sub_141fc5ec0
    Replacement::draft(0x141fc5fb0, "unknown_141fc5fb0", tpl_496 as *const ()), // sub_141fc5fb0
    Replacement::draft_throws(0x141fc5fc0, "unknown_141fc5fc0", fn_141fc5fc0 as *const (), Throws::Caught), // sub_141fc5fc0
    Replacement::draft(0x141fc6e50, "unknown_141fc6e50", tpl_361 as *const ()), // sub_141fc6e50
    Replacement::draft(0x141fc6e60, "unknown_141fc6e60", fn_141fc6e60 as *const ()), // sub_141fc6e60
    Replacement::draft(0x141fc6ea0, "unknown_141fc6ea0", tpl_3232::<0x18, 0x10, 0x142489d30> as *const ()), // sub_141fc6ea0
    Replacement::draft_throws(0x141fc6ef0, "unknown_141fc6ef0", fn_141fc6ef0 as *const (), Throws::Discard), // sub_141fc6ef0
    Replacement::draft_throws(0x141fc6fd0, "unknown_141fc6fd0", fn_141fc6fd0 as *const (), Throws::Discard), // sub_141fc6fd0
    Replacement::draft_throws(0x141fc76c0, "unknown_141fc76c0", fn_141fc76c0 as *const (), Throws::Discard), // sub_141fc76c0
    Replacement::draft(0x141fc7e60, "unknown_141fc7e60", fn_141fc7e60 as *const ()), // sub_141fc7e60
    Replacement::draft_throws(0x141fc7e80, "unknown_141fc7e80", fn_141fc7e80 as *const (), Throws::Discard), // sub_141fc7e80
    Replacement::draft_throws(0x141fc8000, "unknown_141fc8000", fn_141fc8000 as *const (), Throws::Discard), // sub_141fc8000
    Replacement::draft(0x141fc8120, "unknown_141fc8120", fn_141fc8120 as *const ()), // sub_141fc8120
    Replacement::draft(0x141fc8190, "cohtml_141fc8190", fn_141fc8190 as *const ()), // sub_141fc8190
    Replacement::draft(0x141fc8a70, "coregame_141fc8a70", fn_141fc8a70 as *const ()), // coregame::BaseUIPage::coregame::BaseUIPage::vfunc_28
    Replacement::draft_throws(0x141fc8aa0, "coregame_141fc8aa0", fn_141fc8aa0 as *const (), Throws::Discard), // coregame::BaseUIPage::coregame::BaseUIPage::vfunc_49
    Replacement::draft_throws(0x141fc8b40, "coregame_141fc8b40", fn_141fc8b40 as *const (), Throws::Caught), // coregame::BaseUIPage::coregame::BaseUIPage::vfunc_26
    Replacement::draft(0x141fc92f0, "unknown_141fc92f0", fn_141fc92f0 as *const ()), // sub_141fc92f0
    Replacement::draft_throws(0x141fc9370, "unknown_141fc9370", fn_141fc9370 as *const (), Throws::Caught), // sub_141fc9370
    Replacement::draft(0x141fc9900, "unknown_141fc9900", tpl_95219::<0x143291380, 0x8, 0x10> as *const ()), // sub_141fc9900
    Replacement::draft(0x141fc99b0, "unknown_141fc99b0", tpl_35484::<0x143291380, 0x8, 0x10> as *const ()), // sub_141fc99b0
    Replacement::draft(0x141fc9a90, "unknown_141fc9a90", fn_141fc9a90 as *const ()), // sub_141fc9a90
    Replacement::draft(0x141fc9ba0, "unknown_141fc9ba0", fn_141fc9ba0 as *const ()), // sub_141fc9ba0
    Replacement::draft(0x141fc9c50, "unknown_141fc9c50", fn_141fc9c50 as *const ()), // sub_141fc9c50
    Replacement::draft(0x141fc9dd0, "unknown_141fc9dd0", tpl_100464::<0x8> as *const ()), // sub_141fc9dd0
    Replacement::draft(0x141fc9e80, "unknown_141fc9e80", tpl_40817::<0x8> as *const ()), // sub_141fc9e80
    Replacement::draft(0x141fc9eb0, "unknown_141fc9eb0", tpl_100465::<0x8, 0xc> as *const ()), // sub_141fc9eb0
    Replacement::draft(0x141fca7b0, "unknown_141fca7b0", fn_141fca7b0 as *const ()), // sub_141fca7b0
    Replacement::draft_throws(0x141fca830, "unknown_141fca830", fn_141fca830 as *const (), Throws::Discard), // sub_141fca830
    Replacement::draft_throws(0x141fca8b0, "unknown_141fca8b0", fn_141fca8b0 as *const (), Throws::Discard), // sub_141fca8b0
    Replacement::draft(0x141fca950, "unknown_141fca950", fn_141fca950 as *const ()), // sub_141fca950
    Replacement::draft(0x141fcacc0, "unknown_141fcacc0", fn_141fcacc0 as *const ()), // sub_141fcacc0
    Replacement::draft(0x141fcae10, "unknown_141fcae10", fn_141fcae10 as *const ()), // sub_141fcae10
    Replacement::draft(0x141fcafb0, "unknown_141fcafb0", tpl_36980::<0x4> as *const ()), // sub_141fcafb0
    Replacement::draft_throws(0x141fcafe0, "unknown_141fcafe0", fn_141fcafe0 as *const (), Throws::Discard), // sub_141fcafe0
    Replacement::draft(0x141fcb360, "unknown_141fcb360", fn_141fcb360 as *const ()), // sub_141fcb360
    Replacement::draft(0x141fcb870, "unknown_141fcb870", fn_141fcb870 as *const ()), // sub_141fcb870
    Replacement::draft_throws(0x141fcb910, "unknown_141fcb910", fn_141fcb910 as *const (), Throws::Caught), // sub_141fcb910
    Replacement::draft_throws(0x141fcbf10, "unknown_141fcbf10", fn_141fcbf10 as *const (), Throws::Caught), // sub_141fcbf10
    Replacement::draft_throws(0x141fcd590, "unknown_141fcd590", fn_141fcd590 as *const (), Throws::Discard), // sub_141fcd590
    Replacement::draft(0x141fcdee0, "unknown_141fcdee0", fn_141fcdee0 as *const ()), // sub_141fcdee0
    Replacement::draft(0x141fce200, "unknown_141fce200", fn_141fce200 as *const ()), // sub_141fce200
    Replacement::draft(0x141fce480, "unknown_141fce480", fn_141fce480 as *const ()), // sub_141fce480
    Replacement::draft(0x141fced80, "unknown_141fced80", fn_141fced80 as *const ()), // sub_141fced80
    Replacement::draft(0x141fcee90, "unknown_141fcee90", fn_141fcee90 as *const ()), // sub_141fcee90
    Replacement::draft_throws(0x141fceee0, "unknown_141fceee0", fn_141fceee0 as *const (), Throws::Discard), // sub_141fceee0
    Replacement::draft_throws(0x141fcf1f0, "unknown_141fcf1f0", fn_141fcf1f0 as *const (), Throws::Caught), // sub_141fcf1f0
    Replacement::draft(0x141fcf6c0, "unknown_141fcf6c0", fn_141fcf6c0 as *const ()), // sub_141fcf6c0
    Replacement::draft(0x141fcf8b0, "unknown_141fcf8b0", fn_141fcf8b0 as *const ()), // sub_141fcf8b0
    Replacement::draft(0x141fcfac0, "unknown_141fcfac0", fn_141fcfac0 as *const ()), // sub_141fcfac0
    Replacement::draft(0x141fcfbc0, "unknown_141fcfbc0", fn_141fcfbc0 as *const ()), // sub_141fcfbc0
    Replacement::draft_throws(0x141fcfc40, "unknown_141fcfc40", fn_141fcfc40 as *const (), Throws::Discard), // sub_141fcfc40
    Replacement::draft(0x141fcff30, "unknown_141fcff30", fn_141fcff30 as *const ()), // sub_141fcff30
    Replacement::draft(0x141fcff60, "unknown_141fcff60", fn_141fcff60 as *const ()), // sub_141fcff60
    Replacement::draft_throws(0x141fcff90, "unknown_141fcff90", fn_141fcff90 as *const (), Throws::Discard), // sub_141fcff90
    Replacement::draft_throws(0x141fd03a0, "unknown_141fd03a0", fn_141fd03a0 as *const (), Throws::Discard), // sub_141fd03a0
    Replacement::draft(0x141fd04a0, "unknown_141fd04a0", fn_141fd04a0 as *const ()), // sub_141fd04a0
    Replacement::draft(0x141fd04f0, "unknown_141fd04f0", fn_141fd04f0 as *const ()), // sub_141fd04f0
    Replacement::draft(0x141fd0530, "unknown_141fd0530", fn_141fd0530 as *const ()), // sub_141fd0530
    Replacement::draft(0x141fd05b0, "unknown_141fd05b0", fn_141fd05b0 as *const ()), // sub_141fd05b0
    Replacement::draft_throws(0x141fd05e0, "unknown_141fd05e0", fn_141fd05e0 as *const (), Throws::Discard), // sub_141fd05e0
    Replacement::draft_throws(0x141fd06a0, "unknown_141fd06a0", fn_141fd06a0 as *const (), Throws::Discard), // sub_141fd06a0
    Replacement::draft(0x141fd0ed0, "unknown_141fd0ed0", fn_141fd0ed0 as *const ()), // sub_141fd0ed0
    Replacement::draft_throws(0x141fd1020, "unknown_141fd1020", fn_141fd1020 as *const (), Throws::Discard), // sub_141fd1020
    Replacement::draft_throws(0x141fd15c0, "unknown_141fd15c0", fn_141fd15c0 as *const (), Throws::Caught), // sub_141fd15c0
    Replacement::draft(0x141fd1840, "unknown_141fd1840", fn_141fd1840 as *const ()), // sub_141fd1840
    Replacement::draft_throws(0x141fd1870, "unknown_141fd1870", tpl_42863::<0x143c4cdb4, 0x143a08418, 0x1424cb9c0> as *const (), Throws::Discard), // sub_141fd1870
    Replacement::draft_throws(0x141fd18a0, "unknown_141fd18a0", tpl_42863::<0x143c4cdb0, 0x143a08418, 0x1424cb9c0> as *const (), Throws::Discard), // sub_141fd18a0
    Replacement::draft_throws(0x141fd18d0, "unknown_141fd18d0", fn_141fd18d0 as *const (), Throws::Discard), // sub_141fd18d0
    Replacement::draft_throws(0x141fd18f0, "unknown_141fd18f0", fn_141fd18f0 as *const (), Throws::Discard), // sub_141fd18f0
    Replacement::draft(0x141fd1910, "unknown_141fd1910", fn_141fd1910 as *const ()), // sub_141fd1910
    Replacement::draft_throws(0x141fd1940, "unknown_141fd1940", fn_141fd1940 as *const (), Throws::Discard), // sub_141fd1940
    Replacement::draft_throws(0x141fd1980, "unknown_141fd1980", fn_141fd1980 as *const (), Throws::Discard), // sub_141fd1980
    Replacement::draft_throws(0x141fd19d0, "unknown_141fd19d0", fn_141fd19d0 as *const (), Throws::Discard), // sub_141fd19d0
    Replacement::draft_throws(0x141fd19f0, "unknown_141fd19f0", fn_141fd19f0 as *const (), Throws::Discard), // sub_141fd19f0
    Replacement::draft(0x141fd1a10, "unknown_141fd1a10", fn_141fd1a10 as *const ()), // sub_141fd1a10
    Replacement::draft_throws(0x141fd1a40, "unknown_141fd1a40", fn_141fd1a40 as *const (), Throws::Discard), // sub_141fd1a40
    Replacement::draft_throws(0x141fd1a70, "unknown_141fd1a70", fn_141fd1a70 as *const (), Throws::Caught), // sub_141fd1a70
    Replacement::draft_throws(0x141fd1d80, "unknown_141fd1d80", fn_141fd1d80 as *const (), Throws::Caught), // sub_141fd1d80
    Replacement::draft(0x141fd1dd0, "unknown_141fd1dd0", tpl_42875::<0x143a41c58> as *const ()), // sub_141fd1dd0
    Replacement::draft(0x141fd1df0, "unknown_141fd1df0", fn_141fd1df0 as *const ()), // sub_141fd1df0
    Replacement::draft(0x141fd1e20, "unknown_141fd1e20", fn_141fd1e20 as *const ()), // sub_141fd1e20
    Replacement::draft(0x141fd1f00, "unknown_141fd1f00", fn_141fd1f00 as *const ()), // sub_141fd1f00
    Replacement::draft(0x141fd1fa0, "unknown_141fd1fa0", fn_141fd1fa0 as *const ()), // sub_141fd1fa0
    Replacement::draft(0x141fd2080, "unknown_141fd2080", fn_141fd2080 as *const ()), // sub_141fd2080
    Replacement::draft(0x141fd20b0, "unknown_141fd20b0", fn_141fd20b0 as *const ()), // sub_141fd20b0
    Replacement::draft(0x141fd20e0, "unknown_141fd20e0", fn_141fd20e0 as *const ()), // sub_141fd20e0
    Replacement::draft(0x141fd2230, "unknown_141fd2230", fn_141fd2230 as *const ()), // sub_141fd2230
    Replacement::draft(0x141fd2260, "unknown_141fd2260", fn_141fd2260 as *const ()), // sub_141fd2260
    Replacement::draft(0x141fd2290, "unknown_141fd2290", fn_141fd2290 as *const ()), // sub_141fd2290
    Replacement::draft(0x141fd2360, "unknown_141fd2360", fn_141fd2360 as *const ()), // sub_141fd2360
    Replacement::draft(0x141fd2390, "unknown_141fd2390", fn_141fd2390 as *const ()), // sub_141fd2390
    Replacement::draft(0x141fd24a0, "unknown_141fd24a0", fn_141fd24a0 as *const ()), // sub_141fd24a0
    Replacement::draft(0x141fd2720, "unknown_141fd2720", tpl_95295::<0x143280288, 0x8, 0x10, 0x14> as *const ()), // sub_141fd2720
    Replacement::draft(0x141fd2a40, "unknown_141fd2a40", fn_141fd2a40 as *const ()), // sub_141fd2a40
    Replacement::draft(0x141fd2b60, "unknown_141fd2b60", fn_141fd2b60 as *const ()), // sub_141fd2b60
    Replacement::draft(0x141fd2ca0, "unknown_141fd2ca0", fn_141fd2ca0 as *const ()), // sub_141fd2ca0
    Replacement::draft(0x141fd2ce0, "unknown_141fd2ce0", fn_141fd2ce0 as *const ()), // sub_141fd2ce0
    Replacement::draft_throws(0x141fd2df0, "unknown_141fd2df0", fn_141fd2df0 as *const (), Throws::Caught), // sub_141fd2df0
    Replacement::draft_throws(0x141fd33f0, "unknown_141fd33f0", fn_141fd33f0 as *const (), Throws::Discard), // sub_141fd33f0
    Replacement::draft(0x141fd3510, "unknown_141fd3510", tpl_38168::<0x10, 0x8, 0x58, 0x60, 0x143ce3808, 0x158, 0x14020e8f0> as *const ()), // sub_141fd3510
    Replacement::draft_throws(0x141fd3610, "unknown_141fd3610", fn_141fd3610 as *const (), Throws::Caught), // sub_141fd3610
    Replacement::draft_throws(0x141fd3ad0, "unknown_141fd3ad0", fn_141fd3ad0 as *const (), Throws::Discard), // sub_141fd3ad0
    Replacement::draft(0x141fd3bf0, "unknown_141fd3bf0", fn_141fd3bf0 as *const ()), // sub_141fd3bf0
    Replacement::draft(0x141fd3c00, "unknown_141fd3c00", fn_141fd3c00 as *const ()), // sub_141fd3c00
    Replacement::draft(0x141fd3c20, "unknown_141fd3c20", fn_141fd3c20 as *const ()), // sub_141fd3c20
    Replacement::draft(0x141fd3c40, "unknown_141fd3c40", tpl_34036::<0x4> as *const ()), // sub_141fd3c40
    Replacement::draft(0x141fd3c60, "unknown_141fd3c60", fn_141fd3c60 as *const ()), // sub_141fd3c60
    Replacement::draft_throws(0x141fd3ca0, "unknown_141fd3ca0", fn_141fd3ca0 as *const (), Throws::Discard), // sub_141fd3ca0
    Replacement::draft(0x141fd4070, "unknown_141fd4070", fn_141fd4070 as *const ()), // sub_141fd4070
    Replacement::draft(0x141fd41c0, "unknown_141fd41c0", fn_141fd41c0 as *const ()), // sub_141fd41c0
    Replacement::draft(0x141fd41f0, "unknown_141fd41f0", fn_141fd41f0 as *const ()), // sub_141fd41f0
    Replacement::draft_throws(0x141fd4220, "unknown_141fd4220", fn_141fd4220 as *const (), Throws::Discard), // sub_141fd4220
    Replacement::draft(0x141fd4a70, "unknown_141fd4a70", fn_141fd4a70 as *const ()), // sub_141fd4a70
    Replacement::draft(0x141fd4c00, "unknown_141fd4c00", fn_141fd4c00 as *const ()), // sub_141fd4c00
    Replacement::draft(0x141fd4c80, "unknown_141fd4c80", fn_141fd4c80 as *const ()), // sub_141fd4c80
    Replacement::draft(0x141fd4ca0, "unknown_141fd4ca0", fn_141fd4ca0 as *const ()), // sub_141fd4ca0
    Replacement::draft(0x141fd4e80, "unknown_141fd4e80", fn_141fd4e80 as *const ()), // sub_141fd4e80
    Replacement::draft(0x141fd4f10, "unknown_141fd4f10", fn_141fd4f10 as *const ()), // sub_141fd4f10
    Replacement::draft(0x141fd5270, "unknown_141fd5270", fn_141fd5270 as *const ()), // sub_141fd5270
    Replacement::draft(0x141fd52d0, "unknown_141fd52d0", fn_141fd52d0 as *const ()), // sub_141fd52d0
    Replacement::draft_throws(0x141fd5350, "unknown_141fd5350", fn_141fd5350 as *const (), Throws::Discard), // sub_141fd5350
    Replacement::draft(0x141fd5410, "unknown_141fd5410", fn_141fd5410 as *const ()), // sub_141fd5410
    Replacement::draft(0x141fd5480, "unknown_141fd5480", fn_141fd5480 as *const ()), // sub_141fd5480
    Replacement::draft_throws(0x141fd5520, "unknown_141fd5520", fn_141fd5520 as *const (), Throws::Discard), // sub_141fd5520
    Replacement::draft(0x141fd5f30, "unknown_141fd5f30", fn_141fd5f30 as *const ()), // sub_141fd5f30
    Replacement::draft_throws(0x141fd6e40, "unknown_141fd6e40", fn_141fd6e40 as *const (), Throws::Discard), // sub_141fd6e40
    Replacement::draft_throws(0x141fd7110, "unknown_141fd7110", fn_141fd7110 as *const (), Throws::Discard), // sub_141fd7110
    Replacement::draft(0x141fd7990, "unknown_141fd7990", fn_141fd7990 as *const ()), // sub_141fd7990
    Replacement::draft(0x141fd7b10, "unknown_141fd7b10", tpl_42913::<0x8> as *const ()), // sub_141fd7b10
    Replacement::draft_throws(0x141fd7b30, "unknown_141fd7b30", tpl_42137::<0x141fd82d0> as *const (), Throws::Discard), // sub_141fd7b30
    Replacement::draft(0x141fd7b50, "unknown_141fd7b50", fn_141fd7b50 as *const ()), // sub_141fd7b50
    Replacement::draft_throws(0x141fd7c70, "unknown_141fd7c70", fn_141fd7c70 as *const (), Throws::Discard), // sub_141fd7c70
    Replacement::draft_throws(0x141fd7d40, "unknown_141fd7d40", fn_141fd7d40 as *const (), Throws::Discard), // sub_141fd7d40
    Replacement::draft_throws(0x141fd7f30, "unknown_141fd7f30", fn_141fd7f30 as *const (), Throws::Discard), // sub_141fd7f30
    Replacement::draft(0x141fd81b0, "unknown_141fd81b0", fn_141fd81b0 as *const ()), // sub_141fd81b0
    Replacement::draft_throws(0x141fd82d0, "unknown_141fd82d0", fn_141fd82d0 as *const (), Throws::Discard), // sub_141fd82d0
    Replacement::draft_throws(0x141fd8cb0, "unknown_141fd8cb0", fn_141fd8cb0 as *const (), Throws::Discard), // sub_141fd8cb0
    Replacement::draft_throws(0x141fd8f70, "unknown_141fd8f70", fn_141fd8f70 as *const (), Throws::Discard), // sub_141fd8f70
    Replacement::draft_throws(0x141fd9180, "unknown_141fd9180", fn_141fd9180 as *const (), Throws::Caught), // sub_141fd9180
    Replacement::draft(0x141fd9680, "unknown_141fd9680", fn_141fd9680 as *const ()), // sub_141fd9680
    Replacement::draft(0x141fd9d80, "unknown_141fd9d80", fn_141fd9d80 as *const ()), // sub_141fd9d80
    Replacement::draft(0x141fd9e00, "unknown_141fd9e00", fn_141fd9e00 as *const ()), // sub_141fd9e00
    Replacement::draft(0x141fd9e40, "unknown_141fd9e40", fn_141fd9e40 as *const ()), // sub_141fd9e40
    Replacement::draft(0x141fd9e60, "unknown_141fd9e60", fn_141fd9e60 as *const ()), // sub_141fd9e60
    Replacement::draft(0x141fd9ef0, "unknown_141fd9ef0", fn_141fd9ef0 as *const ()), // sub_141fd9ef0
    Replacement::draft(0x141fd9fa0, "unknown_141fd9fa0", fn_141fd9fa0 as *const ()), // sub_141fd9fa0
    Replacement::draft(0x141fda020, "unknown_141fda020", fn_141fda020 as *const ()), // sub_141fda020
    Replacement::draft(0x141fda0d0, "unknown_141fda0d0", fn_141fda0d0 as *const ()), // sub_141fda0d0
    Replacement::draft(0x141fda170, "unknown_141fda170", fn_141fda170 as *const ()), // sub_141fda170
    Replacement::draft(0x141fda290, "unknown_141fda290", fn_141fda290 as *const ()), // sub_141fda290
    Replacement::draft(0x141fda390, "unknown_141fda390", fn_141fda390 as *const ()), // sub_141fda390
    Replacement::draft_throws(0x141fda4b0, "unknown_141fda4b0", fn_141fda4b0 as *const (), Throws::Discard), // sub_141fda4b0
    Replacement::draft(0x141fda570, "unknown_141fda570", fn_141fda570 as *const ()), // sub_141fda570
    Replacement::draft(0x141fda600, "unknown_141fda600", fn_141fda600 as *const ()), // sub_141fda600
    Replacement::draft(0x141fda6b0, "unknown_141fda6b0", fn_141fda6b0 as *const ()), // sub_141fda6b0
    Replacement::draft(0x141fda760, "unknown_141fda760", fn_141fda760 as *const ()), // sub_141fda760
    Replacement::draft(0x141fda810, "unknown_141fda810", fn_141fda810 as *const ()), // sub_141fda810
    Replacement::draft(0x141fda8d0, "unknown_141fda8d0", fn_141fda8d0 as *const ()), // sub_141fda8d0
    Replacement::draft_throws(0x141fda950, "unknown_141fda950", fn_141fda950 as *const (), Throws::Discard), // sub_141fda950
    Replacement::draft_throws(0x141fdac10, "unknown_141fdac10", fn_141fdac10 as *const (), Throws::Discard), // sub_141fdac10
    Replacement::draft_throws(0x141fdad40, "unknown_141fdad40", fn_141fdad40 as *const (), Throws::Discard), // sub_141fdad40
    Replacement::draft_throws(0x141fdae30, "unknown_141fdae30", fn_141fdae30 as *const (), Throws::Discard), // sub_141fdae30
    Replacement::draft(0x141fdaec0, "unknown_141fdaec0", fn_141fdaec0 as *const ()), // sub_141fdaec0
    Replacement::draft_throws(0x141fdaf60, "unknown_141fdaf60", fn_141fdaf60 as *const (), Throws::Discard), // sub_141fdaf60
    Replacement::draft(0x141fdb010, "unknown_141fdb010", fn_141fdb010 as *const ()), // sub_141fdb010
    Replacement::draft(0x141fdb050, "unknown_141fdb050", fn_141fdb050 as *const ()), // sub_141fdb050
    Replacement::draft(0x141fdb980, "unknown_141fdb980", fn_141fdb980 as *const ()), // sub_141fdb980
    Replacement::draft(0x141fdbab0, "unknown_141fdbab0", fn_141fdbab0 as *const ()), // sub_141fdbab0
    Replacement::draft_throws(0x141fdbb90, "unknown_141fdbb90", fn_141fdbb90 as *const (), Throws::Discard), // sub_141fdbb90
    Replacement::draft_throws(0x141fdbcc0, "unknown_141fdbcc0", fn_141fdbcc0 as *const (), Throws::Discard), // sub_141fdbcc0
    Replacement::draft_throws(0x141fdccc0, "unknown_141fdccc0", fn_141fdccc0 as *const (), Throws::Caught), // sub_141fdccc0
    Replacement::draft(0x141fdda10, "unknown_141fdda10", fn_141fdda10 as *const ()), // sub_141fdda10
    Replacement::draft(0x141fddae0, "unknown_141fddae0", fn_141fddae0 as *const ()), // sub_141fddae0
    Replacement::draft_throws(0x141fddb10, "unknown_141fddb10", fn_141fddb10 as *const (), Throws::Caught), // sub_141fddb10
    Replacement::draft(0x141fddbf0, "unknown_141fddbf0", fn_141fddbf0 as *const ()), // sub_141fddbf0
    Replacement::draft(0x141fddd70, "unknown_141fddd70", fn_141fddd70 as *const ()), // sub_141fddd70
    Replacement::draft(0x141fddea0, "unknown_141fddea0", tpl_34667::<0x20> as *const ()), // sub_141fddea0
    Replacement::draft(0x141fde180, "unknown_141fde180", tpl_94771::<0x140330a20> as *const ()), // sub_141fde180
    Replacement::draft(0x141fde240, "unknown_141fde240", tpl_34684::<0x1401f2bc0> as *const ()), // sub_141fde240
    Replacement::draft(0x141fde2f0, "unknown_141fde2f0", tpl_34671::<0x20> as *const ()), // sub_141fde2f0
    Replacement::draft(0x141fde390, "unknown_141fde390", tpl_94771::<0x140fdea60> as *const ()), // sub_141fde390
    Replacement::draft(0x141fde450, "unknown_141fde450", tpl_34684::<0x140fd8d10> as *const ()), // sub_141fde450
    Replacement::draft(0x141fdef90, "unknown_141fdef90", fn_141fdef90 as *const ()), // sub_141fdef90
    Replacement::draft_throws(0x141fdefb0, "unknown_141fdefb0", fn_141fdefb0 as *const (), Throws::Caught), // sub_141fdefb0
    Replacement::draft_throws(0x141fdf2b0, "unknown_141fdf2b0", fn_141fdf2b0 as *const (), Throws::Caught), // sub_141fdf2b0
    Replacement::draft(0x141fdf5d0, "unknown_141fdf5d0", tpl_42952::<0x10> as *const ()), // sub_141fdf5d0
    Replacement::draft(0x141fdf610, "unknown_141fdf610", tpl_42952::<0x10> as *const ()), // sub_141fdf610
    Replacement::draft(0x141fdf650, "unknown_141fdf650", fn_141fdf650 as *const ()), // sub_141fdf650
    Replacement::draft(0x141fdfb80, "unknown_141fdfb80", fn_141fdfb80 as *const ()), // sub_141fdfb80
    Replacement::draft(0x141fdfe10, "unknown_141fdfe10", fn_141fdfe10 as *const ()), // sub_141fdfe10
    Replacement::draft(0x141fe0320, "unknown_141fe0320", fn_141fe0320 as *const ()), // sub_141fe0320
    Replacement::draft(0x141fe0760, "unknown_141fe0760", fn_141fe0760 as *const ()), // sub_141fe0760
    Replacement::draft(0x141fe07f0, "unknown_141fe07f0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ce4608, 0x158, 0x14020e8f0> as *const ()), // sub_141fe07f0
    Replacement::draft_throws(0x141fe08f0, "unknown_141fe08f0", fn_141fe08f0 as *const (), Throws::Discard), // sub_141fe08f0
    Replacement::draft(0x141fe0a70, "unknown_141fe0a70", fn_141fe0a70 as *const ()), // sub_141fe0a70
    Replacement::draft_throws(0x141fe0d20, "unknown_141fe0d20", fn_141fe0d20 as *const (), Throws::Caught), // sub_141fe0d20
    Replacement::draft(0x141fe0f70, "unknown_141fe0f70", fn_141fe0f70 as *const ()), // sub_141fe0f70
    Replacement::draft(0x141fe1190, "unknown_141fe1190", fn_141fe1190 as *const ()), // sub_141fe1190
    Replacement::draft(0x141fe11b0, "unknown_141fe11b0", fn_141fe11b0 as *const ()), // sub_141fe11b0
    Replacement::draft(0x141fe1270, "unknown_141fe1270", fn_141fe1270 as *const ()), // sub_141fe1270
    Replacement::draft(0x141fe1300, "unknown_141fe1300", fn_141fe1300 as *const ()), // sub_141fe1300
    Replacement::draft(0x141fe1330, "unknown_141fe1330", fn_141fe1330 as *const ()), // sub_141fe1330
    Replacement::draft(0x141fe1350, "unknown_141fe1350", fn_141fe1350 as *const ()), // sub_141fe1350
    Replacement::draft(0x141fe13b0, "unknown_141fe13b0", fn_141fe13b0 as *const ()), // sub_141fe13b0
    Replacement::draft(0x141fe13e0, "unknown_141fe13e0", fn_141fe13e0 as *const ()), // sub_141fe13e0
    Replacement::draft(0x141fe1450, "unknown_141fe1450", fn_141fe1450 as *const ()), // sub_141fe1450
    Replacement::draft(0x141fe14c0, "unknown_141fe14c0", fn_141fe14c0 as *const ()), // sub_141fe14c0
    Replacement::draft_throws(0x141fe1500, "unknown_141fe1500", fn_141fe1500 as *const (), Throws::Discard), // sub_141fe1500
    Replacement::draft(0x141fe1610, "unknown_141fe1610", fn_141fe1610 as *const ()), // sub_141fe1610
    Replacement::draft(0x141fe1650, "unknown_141fe1650", fn_141fe1650 as *const ()), // sub_141fe1650
    Replacement::draft(0x141fe1700, "unknown_141fe1700", fn_141fe1700 as *const ()), // sub_141fe1700
    Replacement::draft(0x141fe1740, "unknown_141fe1740", tpl_42972::<0x1411780c0, 0x4> as *const ()), // sub_141fe1740
    Replacement::draft(0x141fe1780, "unknown_141fe1780", tpl_42972::<0x141177f80, 0x4> as *const ()), // sub_141fe1780
    Replacement::draft(0x141fe17c0, "unknown_141fe17c0", fn_141fe17c0 as *const ()), // sub_141fe17c0
    Replacement::draft(0x141fe1870, "unknown_141fe1870", fn_141fe1870 as *const ()), // sub_141fe1870
    Replacement::draft(0x141fe18b0, "unknown_141fe18b0", fn_141fe18b0 as *const ()), // sub_141fe18b0
    Replacement::draft(0x141fe18f0, "unknown_141fe18f0", fn_141fe18f0 as *const ()), // sub_141fe18f0
    Replacement::draft(0x141fe1930, "unknown_141fe1930", fn_141fe1930 as *const ()), // sub_141fe1930
    Replacement::draft(0x141fe1bf0, "unknown_141fe1bf0", fn_141fe1bf0 as *const ()), // sub_141fe1bf0
    Replacement::draft(0x141fe1ca0, "unknown_141fe1ca0", fn_141fe1ca0 as *const ()), // sub_141fe1ca0
    Replacement::draft(0x141fe1d30, "unknown_141fe1d30", tpl_42972::<0x141177f80, 0x4> as *const ()), // sub_141fe1d30
    Replacement::draft(0x141fe1d70, "unknown_141fe1d70", fn_141fe1d70 as *const ()), // sub_141fe1d70
    Replacement::draft(0x141fe1db0, "unknown_141fe1db0", fn_141fe1db0 as *const ()), // sub_141fe1db0
    Replacement::draft(0x141fe1f10, "unknown_141fe1f10", fn_141fe1f10 as *const ()), // sub_141fe1f10
    Replacement::draft_throws(0x141fe1ff0, "unknown_141fe1ff0", fn_141fe1ff0 as *const (), Throws::Discard), // sub_141fe1ff0
    Replacement::draft(0x141fe2b70, "unknown_141fe2b70", tpl_94790 as *const ()), // sub_141fe2b70
    Replacement::draft(0x141fe2c20, "unknown_141fe2c20", tpl_36742::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141fe2c20
    Replacement::draft(0x141fe39f0, "unknown_141fe39f0", tpl_21493::<0x8, 0x141183290, 0x10, 0xd, 0x14, 0xe, 0x15, 0xc, 0x1411835d0> as *const ()), // sub_141fe39f0
    Replacement::draft(0x141fe3a90, "unknown_141fe3a90", fn_141fe3a90 as *const ()), // sub_141fe3a90
    Replacement::draft(0x141fe3c60, "unknown_141fe3c60", fn_141fe3c60 as *const ()), // sub_141fe3c60
    Replacement::draft_throws(0x141fe3ef0, "unknown_141fe3ef0", fn_141fe3ef0 as *const (), Throws::Discard), // sub_141fe3ef0
    Replacement::draft(0x141fe48c0, "unknown_141fe48c0", fn_141fe48c0 as *const ()), // sub_141fe48c0
    Replacement::draft(0x141fe5160, "unknown_141fe5160", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ce0de0, 0x158, 0x14020e8f0> as *const ()), // sub_141fe5160
    Replacement::draft(0x141fe5260, "unknown_141fe5260", fn_141fe5260 as *const ()), // sub_141fe5260
    Replacement::draft_throws(0x141fe55f0, "unknown_141fe55f0", fn_141fe55f0 as *const (), Throws::Caught), // sub_141fe55f0
    Replacement::draft(0x141fe5dc0, "anonymous_namespace_141fe5dc0", fn_141fe5dc0 as *const ()), // anonymous_namespace::Raycaster::anonymous_namespace::Raycaster::vfunc_2
    Replacement::draft(0x141fe5ed0, "anonymous_namespace_141fe5ed0", fn_141fe5ed0 as *const ()), // anonymous_namespace::Raycaster::anonymous_namespace::Raycaster::vfunc_1
    Replacement::draft(0x141fe5ff0, "unknown_141fe5ff0", tpl_29831::<0x8, 0x10> as *const ()), // sub_141fe5ff0
    Replacement::draft_throws(0x141fe6000, "unknown_141fe6000", fn_141fe6000 as *const (), Throws::Discard), // sub_141fe6000
    Replacement::draft(0x141fe6950, "unknown_141fe6950", fn_141fe6950 as *const ()), // sub_141fe6950
    Replacement::draft(0x141fe6e90, "unknown_141fe6e90", tpl_21505::<0x8, 0x1411885f0> as *const ()), // sub_141fe6e90
    Replacement::draft(0x141fe6f00, "unknown_141fe6f00", fn_141fe6f00 as *const ()), // sub_141fe6f00
    Replacement::draft(0x141fe7710, "unknown_141fe7710", fn_141fe7710 as *const ()), // sub_141fe7710
    Replacement::draft_throws(0x141fe7d60, "unknown_141fe7d60", fn_141fe7d60 as *const (), Throws::Discard), // sub_141fe7d60
    Replacement::draft(0x141fe80a0, "unknown_141fe80a0", tpl_34605::<0x14327f7e8> as *const ()), // sub_141fe80a0
    Replacement::draft(0x141fe81e0, "unknown_141fe81e0", fn_141fe81e0 as *const ()), // sub_141fe81e0
    Replacement::draft(0x141fe82a0, "unknown_141fe82a0", fn_141fe82a0 as *const ()), // sub_141fe82a0
    Replacement::draft(0x141fe82e0, "unknown_141fe82e0", fn_141fe82e0 as *const ()), // sub_141fe82e0
    Replacement::draft(0x141fe83d0, "unknown_141fe83d0", tpl_94441::<0x8> as *const ()), // sub_141fe83d0
    Replacement::draft(0x141fe84a0, "unknown_141fe84a0", fn_141fe84a0 as *const ()), // sub_141fe84a0
    Replacement::draft(0x141fe8ed0, "unknown_141fe8ed0", fn_141fe8ed0 as *const ()), // sub_141fe8ed0
    Replacement::draft(0x141fe8ff0, "unknown_141fe8ff0", tpl_40903::<0x1401e7c10, 0x1411894e0, 0x1403d8aa0, 0x1401e6d50> as *const ()), // sub_141fe8ff0
    Replacement::draft_throws(0x141fe9220, "unknown_141fe9220", fn_141fe9220 as *const (), Throws::Discard), // sub_141fe9220
    Replacement::draft(0x141fe9b00, "unknown_141fe9b00", fn_141fe9b00 as *const ()), // sub_141fe9b00
    Replacement::draft(0x141fe9f60, "unknown_141fe9f60", fn_141fe9f60 as *const ()), // sub_141fe9f60
    Replacement::draft_throws(0x141fea390, "unknown_141fea390", fn_141fea390 as *const (), Throws::Caught), // sub_141fea390
    Replacement::draft_throws(0x141feaa30, "unknown_141feaa30", fn_141feaa30 as *const (), Throws::Discard), // sub_141feaa30
    Replacement::draft(0x141feb1f0, "unknown_141feb1f0", fn_141feb1f0 as *const ()), // sub_141feb1f0
    Replacement::draft(0x141feb390, "unknown_141feb390", fn_141feb390 as *const ()), // sub_141feb390
    Replacement::draft(0x141feb460, "unknown_141feb460", tpl_94422::<0x8, 0xc, 0x10> as *const ()), // sub_141feb460
    Replacement::draft(0x141feb560, "unknown_141feb560", tpl_34607::<0x14118ea40> as *const ()), // sub_141feb560
    Replacement::draft(0x141feb610, "unknown_141feb610", fn_141feb610 as *const ()), // sub_141feb610
    Replacement::draft(0x141feb760, "unknown_141feb760", fn_141feb760 as *const ()), // sub_141feb760
    Replacement::draft(0x141feb830, "unknown_141feb830", fn_141feb830 as *const ()), // sub_141feb830
    Replacement::draft(0x141feb900, "unknown_141feb900", fn_141feb900 as *const ()), // sub_141feb900
    Replacement::draft(0x141feba50, "unknown_141feba50", fn_141feba50 as *const ()), // sub_141feba50
    Replacement::draft(0x141fec430, "unknown_141fec430", fn_141fec430 as *const ()), // sub_141fec430
    Replacement::draft_throws(0x141fecb70, "unknown_141fecb70", fn_141fecb70 as *const (), Throws::Discard), // sub_141fecb70
    Replacement::draft_throws(0x141fece60, "unknown_141fece60", fn_141fece60 as *const (), Throws::Discard), // sub_141fece60
    Replacement::draft(0x141fecf40, "unknown_141fecf40", fn_141fecf40 as *const ()), // sub_141fecf40
    Replacement::draft_throws(0x141fed140, "unknown_141fed140", fn_141fed140 as *const (), Throws::Discard), // sub_141fed140
    Replacement::draft_throws(0x141fed270, "unknown_141fed270", fn_141fed270 as *const (), Throws::Caught), // sub_141fed270
    Replacement::draft(0x141fed450, "unknown_141fed450", fn_141fed450 as *const ()), // sub_141fed450
    Replacement::draft_throws(0x141fed770, "unknown_141fed770", fn_141fed770 as *const (), Throws::Caught), // sub_141fed770
    Replacement::draft(0x141fedc60, "unknown_141fedc60", fn_141fedc60 as *const ()), // sub_141fedc60
    Replacement::draft(0x141fedc90, "unknown_141fedc90", fn_141fedc90 as *const ()), // sub_141fedc90
    Replacement::draft(0x141fee050, "unknown_141fee050", fn_141fee050 as *const ()), // sub_141fee050
    Replacement::draft_throws(0x141fee310, "unknown_141fee310", fn_141fee310 as *const (), Throws::Discard), // sub_141fee310
    Replacement::draft_throws(0x141fee3a0, "unknown_141fee3a0", fn_141fee3a0 as *const (), Throws::Caught), // sub_141fee3a0
    Replacement::draft_throws(0x141fee6d0, "unknown_141fee6d0", fn_141fee6d0 as *const (), Throws::Caught), // sub_141fee6d0
    Replacement::draft(0x141fee900, "unknown_141fee900", fn_141fee900 as *const ()), // sub_141fee900
    Replacement::draft(0x141feeb60, "unknown_141feeb60", fn_141feeb60 as *const ()), // sub_141feeb60
    Replacement::draft(0x141feeb80, "unknown_141feeb80", fn_141feeb80 as *const ()), // sub_141feeb80
    Replacement::draft_throws(0x141feebe0, "unknown_141feebe0", fn_141feebe0 as *const (), Throws::Caught), // sub_141feebe0
    Replacement::draft_throws(0x141feecd0, "unknown_141feecd0", fn_141feecd0 as *const (), Throws::Caught), // sub_141feecd0
    Replacement::draft_throws(0x141feeda0, "unknown_141feeda0", fn_141feeda0 as *const (), Throws::Discard), // sub_141feeda0
    Replacement::draft_throws(0x141feee20, "unknown_141feee20", fn_141feee20 as *const (), Throws::Caught), // sub_141feee20
    Replacement::draft(0x141feefc0, "unknown_141feefc0", fn_141feefc0 as *const ()), // sub_141feefc0
    Replacement::draft_throws(0x141feefe0, "unknown_141feefe0", fn_141feefe0 as *const (), Throws::Discard), // sub_141feefe0
    Replacement::draft(0x141fef0f0, "unknown_141fef0f0", fn_141fef0f0 as *const ()), // sub_141fef0f0
    Replacement::draft(0x141fef2e0, "unknown_141fef2e0", fn_141fef2e0 as *const ()), // sub_141fef2e0
    Replacement::draft_throws(0x141fef410, "unknown_141fef410", fn_141fef410 as *const (), Throws::Discard), // sub_141fef410
    Replacement::draft(0x141fefd30, "unknown_141fefd30", fn_141fefd30 as *const ()), // sub_141fefd30
    Replacement::draft_throws(0x141ff00c0, "unknown_141ff00c0", fn_141ff00c0 as *const (), Throws::Discard), // sub_141ff00c0
    Replacement::draft(0x141ff00f0, "unknown_141ff00f0", fn_141ff00f0 as *const ()), // sub_141ff00f0
    Replacement::draft_throws(0x141ff0190, "unknown_141ff0190", fn_141ff0190 as *const (), Throws::Discard), // sub_141ff0190
    Replacement::draft_throws(0x141ff0a90, "unknown_141ff0a90", tpl_43032::<0x1426455a8, 0x8, 0x141196ad0, 0x141861800, 0x142645598> as *const (), Throws::Discard), // sub_141ff0a90
    Replacement::draft_throws(0x141ff0ad0, "unknown_141ff0ad0", fn_141ff0ad0 as *const (), Throws::Discard), // sub_141ff0ad0
    Replacement::draft_throws(0x141ff0d00, "unknown_141ff0d00", fn_141ff0d00 as *const (), Throws::Caught), // sub_141ff0d00
    Replacement::draft(0x141ff1640, "unknown_141ff1640", fn_141ff1640 as *const ()), // sub_141ff1640
    Replacement::draft_throws(0x141ff1720, "unknown_141ff1720", fn_141ff1720 as *const (), Throws::Discard), // sub_141ff1720
    Replacement::draft_throws(0x141ff1780, "unknown_141ff1780", tpl_43032::<0x1426455a8, 0x8, 0x141196ad0, 0x141861800, 0x142645598> as *const (), Throws::Discard), // sub_141ff1780
    Replacement::draft_throws(0x141ff17c0, "unknown_141ff17c0", fn_141ff17c0 as *const (), Throws::Caught), // sub_141ff17c0
    Replacement::draft_throws(0x141ff1d00, "unknown_141ff1d00", fn_141ff1d00 as *const (), Throws::Caught), // sub_141ff1d00
    Replacement::draft_throws(0x141ff22a0, "unknown_141ff22a0", tpl_42621::<0x48, 0x40, 0x18, 0x28, 0x10, 0x1411975d0> as *const (), Throws::Discard), // sub_141ff22a0
    Replacement::draft_throws(0x141ff2340, "unknown_141ff2340", tpl_42624::<0x18, 0x10, 0x40, 0x28, 0x1411975d0> as *const (), Throws::Discard), // sub_141ff2340
    Replacement::draft_throws(0x141ff2400, "unknown_141ff2400", fn_141ff2400 as *const (), Throws::Discard), // sub_141ff2400
    Replacement::draft(0x141ff2a90, "unknown_141ff2a90", fn_141ff2a90 as *const ()), // sub_141ff2a90
    Replacement::draft(0x141ff2b30, "unknown_141ff2b30", fn_141ff2b30 as *const ()), // sub_141ff2b30
    Replacement::draft(0x141ff2b70, "unknown_141ff2b70", fn_141ff2b70 as *const ()), // sub_141ff2b70
    Replacement::draft(0x141ff2cc0, "unknown_141ff2cc0", fn_141ff2cc0 as *const ()), // sub_141ff2cc0
    Replacement::draft(0x141ff2cf0, "unknown_141ff2cf0", tpl_40344::<0x143c8ce84, 0x14397da68, 0x8, 0x580e0, 0x1, 0x10> as *const ()), // sub_141ff2cf0
    Replacement::draft_throws(0x141ff2d20, "unknown_141ff2d20", fn_141ff2d20 as *const (), Throws::Caught), // sub_141ff2d20
    Replacement::draft(0x141ff2ff0, "unknown_141ff2ff0", fn_141ff2ff0 as *const ()), // sub_141ff2ff0
    Replacement::draft_throws(0x141ff3180, "unknown_141ff3180", fn_141ff3180 as *const (), Throws::Caught), // sub_141ff3180
    Replacement::draft(0x141ff33d0, "unknown_141ff33d0", fn_141ff33d0 as *const ()), // sub_141ff33d0
    Replacement::draft(0x141ff3640, "input_141ff3640", fn_141ff3640 as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_23
    Replacement::draft(0x141ff36b0, "input_141ff36b0", fn_141ff36b0 as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_15
    Replacement::draft_throws(0x141ff37e0, "input_141ff37e0", fn_141ff37e0 as *const (), Throws::Caught), // sub_141ff37e0
    Replacement::draft_throws(0x141ff3b10, "input_141ff3b10", fn_141ff3b10 as *const (), Throws::Caught), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_10
    Replacement::draft_throws(0x141ff3cd0, "input_141ff3cd0", fn_141ff3cd0 as *const (), Throws::Caught), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_9
    Replacement::draft(0x141ff3fc0, "input_141ff3fc0", fn_141ff3fc0 as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_8
    Replacement::draft(0x141ff41b0, "input_141ff41b0", fn_141ff41b0 as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_7
    Replacement::draft(0x141ff43c0, "input_141ff43c0", fn_141ff43c0 as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_6
    Replacement::draft_throws(0x141ff45d0, "input_141ff45d0", fn_141ff45d0 as *const (), Throws::Caught), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_2
    Replacement::draft_throws(0x141ff60f0, "input_141ff60f0", fn_141ff60f0 as *const (), Throws::Discard), // sub_141ff60f0
    Replacement::draft(0x141ff63e0, "input_141ff63e0", tpl_43052::<0x58, 0xd8> as *const ()), // input::coregame::FunctionAction::input::coregame::FunctionAction::vfunc_4
    Replacement::draft(0x141ff6400, "input_141ff6400", tpl_43053::<0x80, 0x6b, 0x70, 0x68, 0x6c, 0x140266cd0, 0x140266ff0, 0x78> as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_15
    Replacement::draft_throws(0x141ff64f0, "input_141ff64f0", fn_141ff64f0 as *const (), Throws::Caught), // sub_141ff64f0
    Replacement::draft_throws(0x141ff6720, "input_141ff6720", tpl_43055::<0x58, 0x1433d4020, 0x14247f340, 0x1e, 0x10, 0x68, 0x1413c1770, 0x90> as *const (), Throws::Discard), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_10
    Replacement::draft_throws(0x141ff6790, "input_141ff6790", fn_141ff6790 as *const (), Throws::Caught), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_9
    Replacement::draft(0x141ff69e0, "input_141ff69e0", fn_141ff69e0 as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_7
    Replacement::draft(0x141ff6b00, "input_141ff6b00", fn_141ff6b00 as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_6
    Replacement::draft_throws(0x141ff6c00, "input_141ff6c00", fn_141ff6c00 as *const (), Throws::Caught), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_2
    Replacement::draft(0x141ff70e0, "input_141ff70e0", tpl_43052::<0x58, 0x108> as *const ()), // input::coregame::SingleAction::input::coregame::SingleAction::vfunc_4
    Replacement::draft(0x141ff7100, "input_141ff7100", tpl_43053::<0x80, 0x6b, 0x70, 0x68, 0x6c, 0x140266cd0, 0x140266ff0, 0x78> as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_15
    Replacement::draft_throws(0x141ff71f0, "input_141ff71f0", fn_141ff71f0 as *const (), Throws::Caught), // sub_141ff71f0
    Replacement::draft_throws(0x141ff73e0, "input_141ff73e0", tpl_43055::<0x58, 0x1433d4020, 0x14247f340, 0x1e, 0x10, 0x68, 0x1413c1770, 0x90> as *const (), Throws::Discard), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_10
    Replacement::draft_throws(0x141ff7450, "input_141ff7450", fn_141ff7450 as *const (), Throws::Caught), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_9
    Replacement::draft(0x141ff76b0, "input_141ff76b0", fn_141ff76b0 as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_7
    Replacement::draft(0x141ff7710, "input_141ff7710", fn_141ff7710 as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_6
    Replacement::draft_throws(0x141ff78d0, "input_141ff78d0", fn_141ff78d0 as *const (), Throws::Discard), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_2
    Replacement::draft(0x141ff7bb0, "input_141ff7bb0", tpl_43052::<0x58, 0xe2> as *const ()), // input::coregame::SingleInputAction::input::coregame::SingleInputAction::vfunc_4
    Replacement::draft_throws(0x141ff7bd0, "input_141ff7bd0", fn_141ff7bd0 as *const (), Throws::Caught), // sub_141ff7bd0
    Replacement::draft_throws(0x141ff7dc0, "input_141ff7dc0", fn_141ff7dc0 as *const (), Throws::Caught), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_10
    Replacement::draft_throws(0x141ff7eb0, "input_141ff7eb0", fn_141ff7eb0 as *const (), Throws::Caught), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_9
    Replacement::draft_throws(0x141ff8060, "input_141ff8060", fn_141ff8060 as *const (), Throws::Discard), // input::coregame::DebugInputAction::input::coregame::DebugInputAction::vfunc_2
    Replacement::draft(0x141ff8150, "input_141ff8150", tpl_21560::<0x68, 0x10, 0x18, 0x70> as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_2
    Replacement::draft(0x141ff8190, "input_141ff8190", fn_141ff8190 as *const ()), // input::coregame::ActionIdReference::input::coregame::ActionIdReference::vfunc_4
    Replacement::draft_throws(0x141ff81a0, "input_141ff81a0", fn_141ff81a0 as *const (), Throws::Caught), // sub_141ff81a0
    Replacement::draft_throws(0x141ff8480, "input_141ff8480", fn_141ff8480 as *const (), Throws::Discard), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_10
    Replacement::draft_throws(0x141ff8570, "input_141ff8570", fn_141ff8570 as *const (), Throws::Caught), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_9
    Replacement::draft(0x141ff87f0, "input_141ff87f0", fn_141ff87f0 as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_7
    Replacement::draft(0x141ff88b0, "input_141ff88b0", fn_141ff88b0 as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_6
    Replacement::draft_throws(0x141ff8b40, "input_141ff8b40", fn_141ff8b40 as *const (), Throws::Discard), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_2
    Replacement::draft(0x141ff91f0, "input_141ff91f0", tpl_43052::<0x58, 0x1a8> as *const ()), // input::coregame::UserInputAction::input::coregame::UserInputAction::vfunc_4
    Replacement::draft(0x141ff9210, "unknown_141ff9210", fn_141ff9210 as *const ()), // sub_141ff9210
    Replacement::draft_throws(0x141ff92d0, "unknown_141ff92d0", fn_141ff92d0 as *const (), Throws::Caught), // sub_141ff92d0
    Replacement::draft_throws(0x141ff9700, "unknown_141ff9700", fn_141ff9700 as *const (), Throws::Caught), // sub_141ff9700
    Replacement::draft_throws(0x141ff9b00, "unknown_141ff9b00", fn_141ff9b00 as *const (), Throws::Caught), // sub_141ff9b00
    Replacement::draft_throws(0x141ff9be0, "unknown_141ff9be0", fn_141ff9be0 as *const (), Throws::Caught), // sub_141ff9be0
    Replacement::draft_throws(0x141ff9e70, "unknown_141ff9e70", fn_141ff9e70 as *const (), Throws::Caught), // sub_141ff9e70
    Replacement::draft(0x141ffa6c0, "unknown_141ffa6c0", fn_141ffa6c0 as *const ()), // sub_141ffa6c0
    Replacement::draft(0x141ffa990, "unknown_141ffa990", fn_141ffa990 as *const ()), // sub_141ffa990
    Replacement::draft_throws(0x141ffac40, "unknown_141ffac40", fn_141ffac40 as *const (), Throws::Discard), // sub_141ffac40
    Replacement::draft_throws(0x141ffaca0, "unknown_141ffaca0", fn_141ffaca0 as *const (), Throws::Discard), // sub_141ffaca0
    Replacement::draft(0x141ffad00, "unknown_141ffad00", fn_141ffad00 as *const ()), // sub_141ffad00
    Replacement::draft(0x141ffad30, "unknown_141ffad30", fn_141ffad30 as *const ()), // sub_141ffad30
    Replacement::draft(0x141ffad60, "unknown_141ffad60", tpl_43083::<0x1420e8920, 0x5aa0> as *const ()), // sub_141ffad60
    Replacement::draft(0x141ffad80, "unknown_141ffad80", tpl_43083::<0x1420e8920, 0x5aa0> as *const ()), // sub_141ffad80
    Replacement::draft(0x141ffada0, "unknown_141ffada0", fn_141ffada0 as *const ()), // sub_141ffada0
    Replacement::draft(0x141ffadf0, "unknown_141ffadf0", fn_141ffadf0 as *const ()), // sub_141ffadf0
    Replacement::draft(0x141ffae50, "unknown_141ffae50", fn_141ffae50 as *const ()), // sub_141ffae50
    Replacement::draft(0x141ffaea0, "unknown_141ffaea0", fn_141ffaea0 as *const ()), // sub_141ffaea0
    Replacement::draft_throws(0x141ffaee0, "unknown_141ffaee0", fn_141ffaee0 as *const (), Throws::Discard), // sub_141ffaee0
    Replacement::draft(0x141ffba80, "unknown_141ffba80", fn_141ffba80 as *const ()), // sub_141ffba80
    Replacement::draft_throws(0x141ffbaa0, "unknown_141ffbaa0", fn_141ffbaa0 as *const (), Throws::Discard), // sub_141ffbaa0
    Replacement::draft(0x141ffbe70, "unknown_141ffbe70", tpl_36651::<0x1433d3b40, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db18, 0x14240ab10> as *const ()), // sub_141ffbe70
    Replacement::draft(0x141ffc4e0, "unknown_141ffc4e0", tpl_36631::<0x1433d3b28, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daa0, 0x14240ab10> as *const ()), // sub_141ffc4e0
    Replacement::draft(0x141ffcb50, "unknown_141ffcb50", tpl_36642::<0x1433d3bb0, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dae8, 0x14240ab10> as *const ()), // sub_141ffcb50
    Replacement::draft(0x141ffd1c0, "unknown_141ffd1c0", tpl_43090::<0x1431f54f8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db08, 0x14240ab10> as *const ()), // sub_141ffd1c0
    Replacement::draft(0x141ffd830, "unknown_141ffd830", tpl_43090::<0x1431f54b8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dafc, 0x14240ab10> as *const ()), // sub_141ffd830
    Replacement::draft(0x141ffdea0, "unknown_141ffdea0", tpl_43090::<0x1431f5490, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db00, 0x14240ab10> as *const ()), // sub_141ffdea0
    Replacement::draft(0x141ffe510, "unknown_141ffe510", tpl_43090::<0x1431f5468, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db10, 0x14240ab10> as *const ()), // sub_141ffe510
    Replacement::draft(0x141ffeb80, "unknown_141ffeb80", tpl_36631::<0x1433d3b98, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dadc, 0x14240ab10> as *const ()), // sub_141ffeb80
    Replacement::draft(0x141fff1f0, "unknown_141fff1f0", tpl_43091::<0x1433d3b70, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daac, 0x14240ab10> as *const ()), // sub_141fff1f0
    Replacement::draft(0x141fff860, "unknown_141fff860", tpl_43091::<0x1433d3b48, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daa8, 0x14240ab10> as *const ()), // sub_141fff860
    Replacement::draft(0x141fffed0, "unknown_141fffed0", tpl_43092::<0x1433d3c20, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dae0, 0x14240ab10> as *const ()), // sub_141fffed0
    Replacement::draft(0x142000540, "unknown_142000540", tpl_36645::<0x1433d3c08, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dacc, 0x14240ab10> as *const ()), // sub_142000540
    Replacement::draft(0x142000bb0, "unknown_142000bb0", tpl_43090::<0x1433d3be0, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dad0, 0x14240ab10> as *const ()), // sub_142000bb0
    Replacement::draft(0x142001220, "unknown_142001220", tpl_36633::<0x1433d3bc0, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dab0, 0x14240ab10> as *const ()), // sub_142001220
    Replacement::draft(0x142001890, "unknown_142001890", tpl_36644::<0x1433d3ca8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dac4, 0x14240ab10> as *const ()), // sub_142001890
    Replacement::draft(0x142001f00, "unknown_142001f00", tpl_36644::<0x1433d3c88, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dac0, 0x14240ab10> as *const ()), // sub_142001f00
    Replacement::draft(0x142002570, "unknown_142002570", tpl_43093::<0x1433d3c68, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daa4, 0x14240ab10> as *const ()), // sub_142002570
    Replacement::draft(0x142002be0, "unknown_142002be0", tpl_43093::<0x1433d3c48, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dab8, 0x14240ab10> as *const ()), // sub_142002be0
    Replacement::draft(0x142003250, "unknown_142003250", tpl_36636::<0x1433d3950, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dad8, 0x14240ab10> as *const ()), // sub_142003250
    Replacement::draft(0x1420038c0, "unknown_1420038c0", tpl_36634::<0x1433d3930, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dae4, 0x14240ab10> as *const ()), // sub_1420038c0
    Replacement::draft(0x142003f30, "unknown_142003f30", tpl_36639::<0x1433d3918, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dac8, 0x14240ab10> as *const ()), // sub_142003f30
    Replacement::draft(0x1420045a0, "unknown_1420045a0", tpl_36639::<0x1433d3900, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dab4, 0x14240ab10> as *const ()), // sub_1420045a0
    Replacement::draft(0x142004c10, "unknown_142004c10", tpl_36638::<0x1433d39d8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dad4, 0x14240ab10> as *const ()), // sub_142004c10
    Replacement::draft(0x142005280, "unknown_142005280", tpl_36640::<0x1433d39c8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397dabc, 0x14240ab10> as *const ()), // sub_142005280
    Replacement::draft(0x1420058f0, "unknown_1420058f0", tpl_43094::<0x1433d3998, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db04, 0x14240ab10> as *const ()), // sub_1420058f0
    Replacement::draft(0x142005f60, "unknown_142005f60", tpl_43094::<0x1433d3968, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db1c, 0x14240ab10> as *const ()), // sub_142005f60
    Replacement::draft(0x1420065d0, "unknown_1420065d0", fn_1420065d0 as *const ()), // sub_1420065d0
    Replacement::draft(0x142006c40, "unknown_142006c40", tpl_36644::<0x1433d3a38, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db28, 0x14240ab10> as *const ()), // sub_142006c40
    Replacement::draft(0x1420072b0, "unknown_1420072b0", fn_1420072b0 as *const ()), // sub_1420072b0
    Replacement::draft(0x142007920, "unknown_142007920", tpl_43090::<0x1433d39e8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daec, 0x14240ab10> as *const ()), // sub_142007920
    Replacement::draft(0x142007f90, "unknown_142007f90", tpl_43092::<0x1433d3b00, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db44, 0x14240ab10> as *const ()), // sub_142007f90
    Replacement::draft(0x142008600, "unknown_142008600", tpl_43092::<0x1433d3ad8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db2c, 0x14240ab10> as *const ()), // sub_142008600
    Replacement::draft(0x142008c70, "unknown_142008c70", tpl_43097::<0x1433d3ab0, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db0c, 0x14240ab10> as *const ()), // sub_142008c70
    Replacement::draft(0x1420092e0, "unknown_1420092e0", tpl_43097::<0x1433d3a88, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db30, 0x14240ab10> as *const ()), // sub_1420092e0
    Replacement::draft(0x142009950, "unknown_142009950", fn_142009950 as *const ()), // sub_142009950
    Replacement::draft(0x142009fc0, "unknown_142009fc0", tpl_43093::<0x1433d3d08, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db34, 0x14240ab10> as *const ()), // sub_142009fc0
    Replacement::draft(0x14200a630, "unknown_14200a630", tpl_36634::<0x1433d3ce8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db24, 0x14240ab10> as *const ()), // sub_14200a630
    Replacement::draft(0x14200aca0, "unknown_14200aca0", tpl_36634::<0x1433d3cc8, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db14, 0x14240ab10> as *const ()), // sub_14200aca0
    Replacement::draft(0x14200b310, "unknown_14200b310", tpl_36631::<0x1433d3d98, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db38, 0x14240ab10> as *const ()), // sub_14200b310
    Replacement::draft(0x14200b980, "unknown_14200b980", tpl_36636::<0x1433d3d80, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397db40, 0x14240ab10> as *const ()), // sub_14200b980
    Replacement::draft(0x14200bff0, "unknown_14200bff0", tpl_36628::<0x1433d3d60, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daf4, 0x14240ab10> as *const ()), // sub_14200bff0
    Replacement::draft(0x14200c660, "unknown_14200c660", tpl_36631::<0x1433d3d48, 0x142409b50, 0x1438572c0, 0x1404a78e0, 0x1438572c8, 0x14240a730, 0x14397daf8, 0x14240ab10> as *const ()), // sub_14200c660
    Replacement::draft_throws(0x14200cce0, "unknown_14200cce0", fn_14200cce0 as *const (), Throws::Caught), // sub_14200cce0
    Replacement::draft(0x14200cd00, "unknown_14200cd00", fn_14200cd00 as *const ()), // sub_14200cd00
    Replacement::draft_throws(0x14200cd20, "unknown_14200cd20", fn_14200cd20 as *const (), Throws::Caught), // sub_14200cd20
    Replacement::draft(0x14200ce90, "unknown_14200ce90", fn_14200ce90 as *const ()), // sub_14200ce90
    Replacement::draft_throws(0x14200cf30, "unknown_14200cf30", fn_14200cf30 as *const (), Throws::Caught), // sub_14200cf30
    Replacement::draft(0x14200d040, "unknown_14200d040", fn_14200d040 as *const ()), // sub_14200d040
    Replacement::draft_throws(0x14200d140, "unknown_14200d140", fn_14200d140 as *const (), Throws::Caught), // sub_14200d140
    Replacement::draft_throws(0x14200d870, "unknown_14200d870", fn_14200d870 as *const (), Throws::Caught), // sub_14200d870
    Replacement::draft(0x14200da20, "unknown_14200da20", fn_14200da20 as *const ()), // sub_14200da20
    Replacement::draft(0x14200da60, "unknown_14200da60", fn_14200da60 as *const ()), // sub_14200da60
    Replacement::draft(0x14200daa0, "unknown_14200daa0", fn_14200daa0 as *const ()), // sub_14200daa0
    Replacement::draft(0x14200dd10, "unknown_14200dd10", fn_14200dd10 as *const ()), // sub_14200dd10
    Replacement::draft(0x14200dd30, "unknown_14200dd30", fn_14200dd30 as *const ()), // sub_14200dd30
    Replacement::draft(0x14200ddb0, "unknown_14200ddb0", fn_14200ddb0 as *const ()), // sub_14200ddb0
    Replacement::draft(0x14200de80, "unknown_14200de80", fn_14200de80 as *const ()), // sub_14200de80
    Replacement::draft(0x14200dfb0, "unknown_14200dfb0", fn_14200dfb0 as *const ()), // sub_14200dfb0
    Replacement::draft(0x14200e220, "unknown_14200e220", fn_14200e220 as *const ()), // sub_14200e220
    Replacement::draft(0x14200e2f0, "unknown_14200e2f0", fn_14200e2f0 as *const ()), // sub_14200e2f0
    Replacement::draft(0x14200e3c0, "unknown_14200e3c0", fn_14200e3c0 as *const ()), // sub_14200e3c0
    Replacement::draft(0x14200e410, "unknown_14200e410", tpl_36515::<0x8, 0x1401de830> as *const ()), // sub_14200e410
    Replacement::draft(0x14200e4c0, "unknown_14200e4c0", tpl_96964::<0x143286a40, 0x8> as *const ()), // sub_14200e4c0
    Replacement::draft(0x14200e910, "unknown_14200e910", fn_14200e910 as *const ()), // sub_14200e910
    Replacement::draft(0x14200ebc0, "unknown_14200ebc0", fn_14200ebc0 as *const ()), // sub_14200ebc0
    Replacement::draft(0x14200ed50, "unknown_14200ed50", fn_14200ed50 as *const ()), // sub_14200ed50
    Replacement::draft(0x14200efe0, "unknown_14200efe0", fn_14200efe0 as *const ()), // sub_14200efe0
    Replacement::draft(0x14200f0e0, "unknown_14200f0e0", fn_14200f0e0 as *const ()), // sub_14200f0e0
    Replacement::draft(0x14200f230, "unknown_14200f230", fn_14200f230 as *const ()), // sub_14200f230
    Replacement::draft_throws(0x14200f2b0, "unknown_14200f2b0", fn_14200f2b0 as *const (), Throws::Discard), // sub_14200f2b0
    Replacement::draft(0x14200fa40, "unknown_14200fa40", fn_14200fa40 as *const ()), // sub_14200fa40
    Replacement::draft(0x14200fb00, "unknown_14200fb00", fn_14200fb00 as *const ()), // sub_14200fb00
    Replacement::draft(0x14200fb50, "unknown_14200fb50", tpl_35060 as *const ()), // sub_14200fb50
    Replacement::draft(0x14200fb90, "unknown_14200fb90", tpl_34994::<0x143ce650c, 0x1438814e8, 0x1438814e0, 0x141834a78, 0x1426195f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_14200fb90
    Replacement::draft(0x14200fc00, "unknown_14200fc00", tpl_369 as *const ()), // sub_14200fc00
    Replacement::draft(0x14200fc10, "unknown_14200fc10", tpl_34318 as *const ()), // sub_14200fc10
    Replacement::draft(0x14200fc30, "unknown_14200fc30", tpl_34318 as *const ()), // sub_14200fc30
    Replacement::draft(0x14200fc50, "unknown_14200fc50", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14200fc50
    Replacement::draft(0x14200fcb0, "unknown_14200fcb0", tpl_34319::<0x143ce64a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14200fcb0
    Replacement::draft(0x14200fd10, "unknown_14200fd10", tpl_361 as *const ()), // sub_14200fd10
    Replacement::draft(0x14200fd20, "unknown_14200fd20", tpl_35063::<0x143ce681c, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_14200fd20
    Replacement::draft(0x14200fd70, "unknown_14200fd70", tpl_1962::<0x8, 0x1411a1b60, 0x1411a1d60> as *const ()), // sub_14200fd70
    Replacement::draft(0x14200fde0, "unknown_14200fde0", tpl_35031::<0x142407c80, 0x143ce681c, 0x14020e8f0> as *const ()), // sub_14200fde0
    Replacement::draft(0x14200ff00, "unknown_14200ff00", tpl_361 as *const ()), // sub_14200ff00
    Replacement::draft(0x14200ff10, "unknown_14200ff10", fn_14200ff10 as *const ()), // sub_14200ff10
    Replacement::draft(0x1420104e0, "unknown_1420104e0", tpl_361 as *const ()), // sub_1420104e0
    Replacement::draft_throws(0x1420109a0, "unknown_1420109a0", fn_1420109a0 as *const (), Throws::Discard), // sub_1420109a0
    Replacement::draft(0x142010f10, "unknown_142010f10", fn_142010f10 as *const ()), // sub_142010f10
    Replacement::draft_throws(0x142011020, "unknown_142011020", fn_142011020 as *const (), Throws::Discard), // sub_142011020
    Replacement::draft_throws(0x142011090, "unknown_142011090", fn_142011090 as *const (), Throws::Discard), // sub_142011090
    Replacement::draft_throws(0x1420112c0, "unknown_1420112c0", fn_1420112c0 as *const (), Throws::Discard), // sub_1420112c0
    Replacement::draft(0x142011560, "unknown_142011560", fn_142011560 as *const ()), // sub_142011560
    Replacement::draft(0x1420115d0, "unknown_1420115d0", tpl_43127::<0x7c, 0x80, 0x70, 0x88, 0x78> as *const ()), // sub_1420115d0
    Replacement::draft(0x142011600, "unknown_142011600", fn_142011600 as *const ()), // sub_142011600
    Replacement::draft_throws(0x142011770, "unknown_142011770", fn_142011770 as *const (), Throws::Discard), // sub_142011770
    Replacement::draft(0x142011ad0, "unknown_142011ad0", fn_142011ad0 as *const ()), // sub_142011ad0
    Replacement::draft(0x142011dd0, "unknown_142011dd0", fn_142011dd0 as *const ()), // sub_142011dd0
    Replacement::draft_throws(0x142011df0, "unknown_142011df0", fn_142011df0 as *const (), Throws::Discard), // sub_142011df0
    Replacement::draft_throws(0x142011f50, "unknown_142011f50", fn_142011f50 as *const (), Throws::Discard), // sub_142011f50
    Replacement::draft(0x1420120a0, "unknown_1420120a0", fn_1420120a0 as *const ()), // sub_1420120a0
    Replacement::draft(0x1420124d0, "unknown_1420124d0", fn_1420124d0 as *const ()), // sub_1420124d0
    Replacement::draft(0x142012630, "unknown_142012630", tpl_43130::<0x80> as *const ()), // sub_142012630
    Replacement::draft(0x142012650, "unknown_142012650", fn_142012650 as *const ()), // sub_142012650
    Replacement::draft(0x1420126e0, "unknown_1420126e0", fn_1420126e0 as *const ()), // sub_1420126e0
    Replacement::draft(0x142012890, "unknown_142012890", fn_142012890 as *const ()), // sub_142012890
    Replacement::draft_throws(0x1420128f0, "unknown_1420128f0", fn_1420128f0 as *const (), Throws::Discard), // sub_1420128f0
    Replacement::draft(0x142012a30, "unknown_142012a30", fn_142012a30 as *const ()), // sub_142012a30
    Replacement::draft_throws(0x142012c00, "unknown_142012c00", fn_142012c00 as *const (), Throws::Discard), // sub_142012c00
    Replacement::draft_throws(0x142012dd0, "unknown_142012dd0", fn_142012dd0 as *const (), Throws::Discard), // sub_142012dd0
    Replacement::draft(0x142013430, "unknown_142013430", fn_142013430 as *const ()), // sub_142013430
    Replacement::draft(0x1420134f0, "unknown_1420134f0", fn_1420134f0 as *const ()), // sub_1420134f0
    Replacement::draft(0x1420138b0, "unknown_1420138b0", fn_1420138b0 as *const ()), // sub_1420138b0
    Replacement::draft(0x1420139e0, "unknown_1420139e0", fn_1420139e0 as *const ()), // sub_1420139e0
    Replacement::draft(0x142013a50, "unknown_142013a50", fn_142013a50 as *const ()), // sub_142013a50
    Replacement::draft(0x142013c70, "unknown_142013c70", fn_142013c70 as *const ()), // sub_142013c70
    Replacement::draft(0x142013dd0, "unknown_142013dd0", fn_142013dd0 as *const ()), // sub_142013dd0
    Replacement::draft(0x142014100, "unknown_142014100", fn_142014100 as *const ()), // sub_142014100
    Replacement::draft(0x1420141e0, "unknown_1420141e0", fn_1420141e0 as *const ()), // sub_1420141e0
    Replacement::draft(0x142014210, "unknown_142014210", fn_142014210 as *const ()), // sub_142014210
    Replacement::draft_throws(0x1420142e0, "unknown_1420142e0", fn_1420142e0 as *const (), Throws::Discard), // sub_1420142e0
    Replacement::draft_throws(0x142014b50, "unknown_142014b50", fn_142014b50 as *const (), Throws::Discard), // sub_142014b50
    Replacement::draft(0x142014f70, "std_142014f70", tpl_770::<0x1433d6db8> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSampler::<lambda_1>,m::Vector3Template<float>,const m::?bref9&>::std::_Fun
    Replacement::draft(0x142014f90, "std_142014f90", tpl_770::<0x1433d6db8> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSampler::<lambda_1>,m::Vector3Template<float>,const m::?bref9&>::std::_Fun
    Replacement::draft(0x142014fd0, "std_142014fd0", tpl_1::<0x143891c00> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSampler::<lambda_1>,m::Vector3Template<float>,const m::?bref9&>::std::_Fun
    Replacement::draft(0x142014fe0, "std_142014fe0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSampler::<lambda_1>,m::Vector3Template<float>,const m::?bref9&>::std::_Fun
    Replacement::draft(0x142014ff0, "std_142014ff0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSampler::<lambda_1>,m::Vector3Template<float>,const m::?bref9&>::std::_Fun
    Replacement::draft(0x142015010, "std_142015010", tpl_770::<0x1433d6df0> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSamplerNonEcs::<lambda_1>,m::Vector3Template<float>,const ?bref7::?bref6&>
    Replacement::draft(0x142015030, "std_142015030", tpl_770::<0x1433d6df0> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSamplerNonEcs::<lambda_1>,m::Vector3Template<float>,const ?bref7::?bref6&>
    Replacement::draft(0x142015070, "std_142015070", tpl_1::<0x143891ba0> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSamplerNonEcs::<lambda_1>,m::Vector3Template<float>,const ?bref7::?bref6&>
    Replacement::draft(0x142015080, "std_142015080", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSamplerNonEcs::<lambda_1>,m::Vector3Template<float>,const ?bref7::?bref6&>
    Replacement::draft(0x142015090, "std_142015090", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::wind::getSamplerNonEcs::<lambda_1>,m::Vector3Template<float>,const ?bref7::?bref6&>
    Replacement::draft(0x1420150b0, "unknown_1420150b0", tpl_43145::<0x8> as *const ()), // sub_1420150b0
    Replacement::draft(0x1420150d0, "unknown_1420150d0", tpl_43145::<0x8> as *const ()), // sub_1420150d0
    Replacement::draft(0x1420150f0, "unknown_1420150f0", fn_1420150f0 as *const ()), // sub_1420150f0
    Replacement::draft(0x142015170, "unknown_142015170", tpl_361 as *const ()), // sub_142015170
    Replacement::draft(0x142015180, "unknown_142015180", fn_142015180 as *const ()), // sub_142015180
    Replacement::draft_throws(0x142015390, "unknown_142015390", fn_142015390 as *const (), Throws::Discard), // sub_142015390
    Replacement::draft_throws(0x142015710, "unknown_142015710", fn_142015710 as *const (), Throws::Discard), // sub_142015710
    Replacement::draft_throws(0x142015990, "unknown_142015990", fn_142015990 as *const (), Throws::Caught), // sub_142015990
    Replacement::draft(0x142015a80, "unknown_142015a80", fn_142015a80 as *const ()), // sub_142015a80
    Replacement::draft(0x142015b20, "unknown_142015b20", fn_142015b20 as *const ()), // sub_142015b20
    Replacement::draft(0x142015c70, "unknown_142015c70", fn_142015c70 as *const ()), // sub_142015c70
    Replacement::draft_throws(0x142015d20, "unknown_142015d20", fn_142015d20 as *const (), Throws::Discard), // sub_142015d20
    Replacement::draft_throws(0x142015df0, "unknown_142015df0", fn_142015df0 as *const (), Throws::Discard), // sub_142015df0
    Replacement::draft_throws(0x142015e90, "unknown_142015e90", fn_142015e90 as *const (), Throws::Discard), // sub_142015e90
    Replacement::draft(0x142016160, "coreshared_142016160", fn_142016160 as *const ()), // coreshared::ParticleSystemState::coreshared::ParticleSystemState::vfunc_4
    Replacement::draft_throws(0x142016190, "coreshared_142016190", fn_142016190 as *const (), Throws::Discard), // coreshared::ParticleSystemState::coreshared::ParticleSystemState::vfunc_3
    Replacement::draft_throws(0x1420161b0, "unknown_1420161b0", fn_1420161b0 as *const (), Throws::Discard), // sub_1420161b0
    Replacement::draft(0x142016280, "unknown_142016280", fn_142016280 as *const ()), // sub_142016280
    Replacement::draft(0x1420163e0, "unknown_1420163e0", fn_1420163e0 as *const ()), // sub_1420163e0
    Replacement::draft(0x142016490, "unknown_142016490", fn_142016490 as *const ()), // sub_142016490
    Replacement::draft(0x1420164d0, "unknown_1420164d0", fn_1420164d0 as *const ()), // sub_1420164d0
    Replacement::draft(0x142016500, "unknown_142016500", fn_142016500 as *const ()), // sub_142016500
    Replacement::draft_throws(0x1420165b0, "unknown_1420165b0", fn_1420165b0 as *const (), Throws::Discard), // sub_1420165b0
    Replacement::draft(0x1420165d0, "unknown_1420165d0", fn_1420165d0 as *const ()), // sub_1420165d0
    Replacement::draft(0x142016660, "unknown_142016660", fn_142016660 as *const ()), // sub_142016660
    Replacement::draft(0x1420166a0, "unknown_1420166a0", fn_1420166a0 as *const ()), // sub_1420166a0
    Replacement::draft_throws(0x142016730, "unknown_142016730", fn_142016730 as *const (), Throws::Discard), // sub_142016730
    Replacement::draft(0x142016800, "unknown_142016800", fn_142016800 as *const ()), // sub_142016800
    Replacement::draft(0x1420168b0, "unknown_1420168b0", fn_1420168b0 as *const ()), // sub_1420168b0
    Replacement::draft(0x142016990, "unknown_142016990", fn_142016990 as *const ()), // sub_142016990
    Replacement::draft(0x1420169e0, "unknown_1420169e0", fn_1420169e0 as *const ()), // sub_1420169e0
    Replacement::draft(0x142016a20, "unknown_142016a20", fn_142016a20 as *const ()), // sub_142016a20
    Replacement::draft(0x142016ab0, "unknown_142016ab0", fn_142016ab0 as *const ()), // sub_142016ab0
    Replacement::draft_throws(0x142016b60, "unknown_142016b60", fn_142016b60 as *const (), Throws::Discard), // sub_142016b60
    Replacement::draft(0x142016d70, "unknown_142016d70", fn_142016d70 as *const ()), // sub_142016d70
    Replacement::draft_throws(0x142016e00, "unknown_142016e00", fn_142016e00 as *const (), Throws::Discard), // sub_142016e00
    Replacement::draft(0x142017590, "unknown_142017590", fn_142017590 as *const ()), // sub_142017590
    Replacement::draft(0x1420175d0, "unknown_1420175d0", fn_1420175d0 as *const ()), // sub_1420175d0
    Replacement::draft(0x142017630, "unknown_142017630", fn_142017630 as *const ()), // sub_142017630
    Replacement::draft(0x142017810, "unknown_142017810", fn_142017810 as *const ()), // sub_142017810
    Replacement::draft(0x1420178c0, "unknown_1420178c0", fn_1420178c0 as *const ()), // sub_1420178c0
    Replacement::draft(0x142017a70, "unknown_142017a70", fn_142017a70 as *const ()), // sub_142017a70
    Replacement::draft(0x142017af0, "unknown_142017af0", fn_142017af0 as *const ()), // sub_142017af0
    Replacement::draft(0x1420186a0, "unknown_1420186a0", fn_1420186a0 as *const ()), // sub_1420186a0
    Replacement::draft(0x142018a00, "unknown_142018a00", fn_142018a00 as *const ()), // sub_142018a00
    Replacement::draft(0x1420190d0, "unknown_1420190d0", fn_1420190d0 as *const ()), // sub_1420190d0
    Replacement::draft(0x142019140, "unknown_142019140", fn_142019140 as *const ()), // sub_142019140
    Replacement::draft(0x142019170, "unknown_142019170", fn_142019170 as *const ()), // sub_142019170
    Replacement::draft(0x1420191b0, "unknown_1420191b0", fn_1420191b0 as *const ()), // sub_1420191b0
    Replacement::draft(0x1420191f0, "unknown_1420191f0", fn_1420191f0 as *const ()), // sub_1420191f0
    Replacement::draft(0x1420192c0, "unknown_1420192c0", fn_1420192c0 as *const ()), // sub_1420192c0
    Replacement::draft(0x142019370, "unknown_142019370", fn_142019370 as *const ()), // sub_142019370
    Replacement::draft(0x142019420, "unknown_142019420", tpl_43181::<0x143427660, 0x8, 0xc, 0x24, 0x18, 0x2c, 0x14, 0x20, 0x50, 0x99, 0xc0, 0x1402741e0> as *const ()), // sub_142019420
    Replacement::draft(0x1420194d0, "unknown_1420194d0", tpl_43181::<0x143427660, 0x8, 0xc, 0x24, 0x18, 0x2c, 0x14, 0x20, 0x50, 0x99, 0x90, 0x1402741e0> as *const ()), // sub_1420194d0
    Replacement::draft(0x142019580, "unknown_142019580", tpl_43182::<0x50, 0x99, 0x143427660, 0x1434268d0, 0xc0, 0x1402741e0, 0x78, 0xe4, 0xec, 0x8> as *const ()), // sub_142019580
    Replacement::draft(0x1420196a0, "unknown_1420196a0", tpl_43182::<0x50, 0x99, 0x143427660, 0x1434268d0, 0x90, 0x1402741e0, 0x78, 0xb4, 0xbc, 0x8> as *const ()), // sub_1420196a0
    Replacement::draft(0x1420197c0, "unknown_1420197c0", fn_1420197c0 as *const ()), // sub_1420197c0
    Replacement::draft(0x142019820, "unknown_142019820", fn_142019820 as *const ()), // sub_142019820
    Replacement::draft(0x142019850, "unknown_142019850", fn_142019850 as *const ()), // sub_142019850
    Replacement::draft_throws(0x142019910, "unknown_142019910", fn_142019910 as *const (), Throws::Discard), // sub_142019910
    Replacement::draft(0x14201b340, "unknown_14201b340", fn_14201b340 as *const ()), // sub_14201b340
    Replacement::draft(0x14201b640, "unknown_14201b640", fn_14201b640 as *const ()), // sub_14201b640
    Replacement::draft(0x14201b9d0, "unknown_14201b9d0", fn_14201b9d0 as *const ()), // sub_14201b9d0
    Replacement::draft(0x14201ba80, "unknown_14201ba80", fn_14201ba80 as *const ()), // sub_14201ba80
    Replacement::draft(0x14201bca0, "unknown_14201bca0", fn_14201bca0 as *const ()), // sub_14201bca0
    Replacement::draft(0x14201c2f0, "unknown_14201c2f0", fn_14201c2f0 as *const ()), // sub_14201c2f0
    Replacement::draft(0x14201c320, "unknown_14201c320", fn_14201c320 as *const ()), // sub_14201c320
    Replacement::draft(0x14201c370, "unknown_14201c370", fn_14201c370 as *const ()), // sub_14201c370
    Replacement::draft(0x14201da90, "unknown_14201da90", fn_14201da90 as *const ()), // sub_14201da90
    Replacement::draft(0x14201dbb0, "unknown_14201dbb0", fn_14201dbb0 as *const ()), // sub_14201dbb0
    Replacement::draft(0x14201dc70, "unknown_14201dc70", fn_14201dc70 as *const ()), // sub_14201dc70
    Replacement::draft(0x14201e020, "unknown_14201e020", fn_14201e020 as *const ()), // sub_14201e020
    Replacement::draft(0x14201e130, "unknown_14201e130", fn_14201e130 as *const ()), // sub_14201e130
    Replacement::draft(0x14201e230, "unknown_14201e230", fn_14201e230 as *const ()), // sub_14201e230
    Replacement::draft(0x14201ef00, "unknown_14201ef00", fn_14201ef00 as *const ()), // sub_14201ef00
    Replacement::draft(0x14201ef20, "unknown_14201ef20", fn_14201ef20 as *const ()), // sub_14201ef20
    Replacement::draft(0x14201f020, "unknown_14201f020", fn_14201f020 as *const ()), // sub_14201f020
    Replacement::draft(0x14201f0d0, "unknown_14201f0d0", fn_14201f0d0 as *const ()), // sub_14201f0d0
    Replacement::draft(0x14201f450, "unknown_14201f450", fn_14201f450 as *const ()), // sub_14201f450
    Replacement::draft(0x14201f5c0, "unknown_14201f5c0", fn_14201f5c0 as *const ()), // sub_14201f5c0
    Replacement::draft(0x14201fa70, "unknown_14201fa70", fn_14201fa70 as *const ()), // sub_14201fa70
    Replacement::draft_throws(0x14201fbe0, "unknown_14201fbe0", fn_14201fbe0 as *const (), Throws::Discard), // sub_14201fbe0
    Replacement::draft_throws(0x14201fd00, "unknown_14201fd00", fn_14201fd00 as *const (), Throws::Discard), // sub_14201fd00
    Replacement::draft(0x14201fd80, "unknown_14201fd80", fn_14201fd80 as *const ()), // sub_14201fd80
    Replacement::draft(0x14201fda0, "unknown_14201fda0", fn_14201fda0 as *const ()), // sub_14201fda0
    Replacement::draft(0x14201fdb0, "unknown_14201fdb0", fn_14201fdb0 as *const ()), // sub_14201fdb0
    Replacement::draft(0x14201fdc0, "unknown_14201fdc0", fn_14201fdc0 as *const ()), // sub_14201fdc0
    Replacement::draft(0x14201fdd0, "unknown_14201fdd0", fn_14201fdd0 as *const ()), // sub_14201fdd0
    Replacement::draft(0x14201fde0, "unknown_14201fde0", fn_14201fde0 as *const ()), // sub_14201fde0
    Replacement::draft(0x14201fe50, "unknown_14201fe50", fn_14201fe50 as *const ()), // sub_14201fe50
    Replacement::draft(0x142020210, "unknown_142020210", fn_142020210 as *const ()), // sub_142020210
    Replacement::draft_throws(0x142020250, "unknown_142020250", fn_142020250 as *const (), Throws::Discard), // sub_142020250
    Replacement::draft_throws(0x1420202b0, "unknown_1420202b0", fn_1420202b0 as *const (), Throws::Discard), // sub_1420202b0
    Replacement::draft(0x142020300, "unknown_142020300", fn_142020300 as *const ()), // sub_142020300
    Replacement::draft(0x142020410, "unknown_142020410", fn_142020410 as *const ()), // sub_142020410
    Replacement::draft(0x142020450, "unknown_142020450", fn_142020450 as *const ()), // sub_142020450
    Replacement::draft(0x1420204c0, "unknown_1420204c0", fn_1420204c0 as *const ()), // sub_1420204c0
    Replacement::draft(0x142020520, "unknown_142020520", fn_142020520 as *const ()), // sub_142020520
    Replacement::draft(0x1420205b0, "unknown_1420205b0", tpl_43216::<0x168, 0x8, 0x170> as *const ()), // sub_1420205b0
    Replacement::draft(0x1420205e0, "unknown_1420205e0", fn_1420205e0 as *const ()), // sub_1420205e0
    Replacement::draft(0x142020690, "unknown_142020690", tpl_29742::<0x70, 0x8> as *const ()), // sub_142020690
    Replacement::draft(0x1420206b0, "unknown_1420206b0", fn_1420206b0 as *const ()), // sub_1420206b0
    Replacement::draft(0x1420206f0, "unknown_1420206f0", fn_1420206f0 as *const ()), // sub_1420206f0
    Replacement::draft(0x142020750, "unknown_142020750", fn_142020750 as *const ()), // sub_142020750
    Replacement::draft_throws(0x142020e60, "unknown_142020e60", fn_142020e60 as *const (), Throws::Discard), // sub_142020e60
    Replacement::draft_throws(0x142020ec0, "unknown_142020ec0", fn_142020ec0 as *const (), Throws::Discard), // sub_142020ec0
    Replacement::draft_throws(0x142020f70, "unknown_142020f70", fn_142020f70 as *const (), Throws::Caught), // sub_142020f70
    Replacement::draft_throws(0x1420211a0, "unknown_1420211a0", fn_1420211a0 as *const (), Throws::Discard), // sub_1420211a0
    Replacement::draft_throws(0x1420212a0, "unknown_1420212a0", fn_1420212a0 as *const (), Throws::Caught), // sub_1420212a0
    Replacement::draft(0x142021540, "unknown_142021540", fn_142021540 as *const ()), // sub_142021540
    Replacement::draft(0x142021790, "unknown_142021790", tpl_361 as *const ()), // sub_142021790
    Replacement::draft(0x1420217a0, "unknown_1420217a0", fn_1420217a0 as *const ()), // sub_1420217a0
    Replacement::draft(0x1420217d0, "unknown_1420217d0", fn_1420217d0 as *const ()), // sub_1420217d0
    Replacement::draft(0x1420217f0, "unknown_1420217f0", fn_1420217f0 as *const ()), // sub_1420217f0
    Replacement::draft(0x142021810, "unknown_142021810", fn_142021810 as *const ()), // sub_142021810
    Replacement::draft(0x142021890, "unknown_142021890", fn_142021890 as *const ()), // sub_142021890
    Replacement::draft(0x142021900, "unknown_142021900", fn_142021900 as *const ()), // sub_142021900
    Replacement::draft(0x142021a70, "unknown_142021a70", fn_142021a70 as *const ()), // sub_142021a70
    Replacement::draft(0x142021af0, "unknown_142021af0", fn_142021af0 as *const ()), // sub_142021af0
    Replacement::draft(0x142021d20, "unknown_142021d20", fn_142021d20 as *const ()), // sub_142021d20
    Replacement::draft(0x142021dd0, "unknown_142021dd0", fn_142021dd0 as *const ()), // sub_142021dd0
    Replacement::draft(0x142021e90, "std_142021e90", tpl_550::<0x1433d72b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x142021eb0, "std_142021eb0", tpl_550::<0x1433d72b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x142021ee0, "std_142021ee0", tpl_1::<0x143891c70> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x142021ef0, "std_142021ef0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x142021f00, "std_142021f00", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x142021f20, "std_142021f20", tpl_550::<0x1433d6ef8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x142021f40, "std_142021f40", tpl_550::<0x1433d6ef8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x142021f70, "std_142021f70", tpl_1::<0x143891cd0> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x142021f80, "std_142021f80", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x142021f90, "std_142021f90", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::NPCScheduleResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft_throws(0x142021fb0, "unknown_142021fb0", fn_142021fb0 as *const (), Throws::Caught), // sub_142021fb0
    Replacement::draft(0x1420222d0, "coreshared_1420222d0", tpl_43236::<0xe8, 0x8> as *const ()), // coreshared::NPCScheduleResource::coreshared::NPCScheduleResource::vfunc_14
    Replacement::draft(0x142022300, "coreshared_142022300", tpl_43236::<0xe8, 0x18> as *const ()), // coreshared::NPCScheduleResource::coreshared::NPCScheduleResource::vfunc_16
    Replacement::draft(0x142022400, "coreshared_142022400", tpl_38958::<0xe8, 0x8> as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x142022430, "coreshared_142022430", tpl_38958::<0xe8, 0x8> as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x142022480, "coreshared_142022480", tpl_38958::<0xe8, 0x18> as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x1420224b0, "coreshared_1420224b0", tpl_38958::<0xe8, 0x18> as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x1420228d0, "coreshared_1420228d0", fn_1420228d0 as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x142022930, "coreshared_142022930", fn_142022930 as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x142022990, "coreshared_142022990", fn_142022990 as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x142022a40, "coreshared_142022a40", fn_142022a40 as *const ()), // coreshared::NPCScheduleResource::ClassTypeInfoImplementation::coreshared::NPCScheduleResource::ClassTypeInfoImplementati
    Replacement::draft(0x142023030, "coreshared_142023030", tpl_35199::<0x143881508, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NPCScheduleMetadata::ClassTypeInfoImplementation::coreshared::NPCScheduleMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x142023120, "coreshared_142023120", tpl_35199::<0x143881508, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NPCScheduleMetadata::ClassTypeInfoImplementation::coreshared::NPCScheduleMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x142023300, "coreshared_142023300", tpl_35199::<0x143881508, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NPCScheduleMetadata::ClassTypeInfoImplementation::coreshared::NPCScheduleMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x1420233f0, "coreshared_1420233f0", tpl_35199::<0x143881508, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NPCScheduleMetadata::ClassTypeInfoImplementation::coreshared::NPCScheduleMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x142023470, "coreshared_142023470", tpl_35199::<0x143881508, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NPCScheduleMetadata::ClassTypeInfoImplementation::coreshared::NPCScheduleMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x1420236e0, "std_1420236e0", tpl_550::<0x1433d7ba8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142023700, "std_142023700", tpl_550::<0x1433d7ba8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142023730, "std_142023730", tpl_1::<0x143891d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142023740, "std_142023740", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142023750, "std_142023750", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142023770, "std_142023770", tpl_550::<0x1433d77f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x142023790, "std_142023790", tpl_550::<0x1433d77f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1420237c0, "std_1420237c0", tpl_1::<0x143891d90> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1420237d0, "std_1420237d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1420237e0, "std_1420237e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::NavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft_throws(0x142023ad0, "coreshared_142023ad0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433d77f0, 0x142428500> as *const (), Throws::Discard), // coreshared::NavmeshResource::coreshared::NavmeshResource::vfunc_2
    Replacement::draft(0x142023c50, "coreshared_142023c50", fn_142023c50 as *const ()), // coreshared::NavmeshResource::coreshared::NavmeshResource::vfunc_25
    Replacement::draft(0x142023f60, "coreshared_142023f60", tpl_43244::<0x8, 0xb8, 0x10, 0x4, 0x18, 0x1408b4ad0, 0x14183acbc> as *const ()), // coreshared::NavmeshResource::ClassTypeInfoImplementation::coreshared::NavmeshResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142023fc0, "coreshared_142023fc0", tpl_43244::<0x8, 0xb8, 0x10, 0x4, 0x18, 0x1408b4ad0, 0x14183acbc> as *const ()), // coreshared::NavmeshResource::ClassTypeInfoImplementation::coreshared::NavmeshResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1420241d0, "coreshared_1420241d0", tpl_35214::<0xd8, 0x1401de830, 0xc8, 0x142428ce0> as *const ()), // coreshared::NavmeshResource::ClassTypeInfoImplementation::coreshared::NavmeshResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142024210, "coreshared_142024210", fn_142024210 as *const ()), // coreshared::NavmeshResource::ClassTypeInfoImplementation::coreshared::NavmeshResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142024270, "coreshared_142024270", tpl_35216::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xd8, 0x1401de830, 0xc8, 0x142428ce0, 0x142442c80> as *const ()), // coreshared::NavmeshResource::ClassTypeInfoImplementation::coreshared::NavmeshResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1420242f0, "coreshared_1420242f0", tpl_35217::<0xd8, 0x1401de830, 0xc8, 0x142428ce0, 0x142442de0> as *const ()), // coreshared::NavmeshResource::ClassTypeInfoImplementation::coreshared::NavmeshResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1420248c0, "coreshared_1420248c0", tpl_35199::<0x143881530, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshMetadata::ClassTypeInfoImplementation::coreshared::NavmeshMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1420249b0, "coreshared_1420249b0", tpl_35199::<0x143881530, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshMetadata::ClassTypeInfoImplementation::coreshared::NavmeshMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142024b90, "coreshared_142024b90", tpl_35199::<0x143881530, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshMetadata::ClassTypeInfoImplementation::coreshared::NavmeshMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142024c80, "coreshared_142024c80", tpl_35199::<0x143881530, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshMetadata::ClassTypeInfoImplementation::coreshared::NavmeshMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142024d00, "coreshared_142024d00", tpl_35199::<0x143881530, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshMetadata::ClassTypeInfoImplementation::coreshared::NavmeshMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142025310, "coreshared_142025310", tpl_35199::<0x143881538, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshTemplateData::ClassTypeInfoImplementation::coreshared::NavmeshTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x142025370, "coreshared_142025370", tpl_35199::<0x143881538, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::NavmeshTemplateData::ClassTypeInfoImplementation::coreshared::NavmeshTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x142025420, "coreshared_142025420", fn_142025420 as *const ()), // coreshared::NavmeshTemplateData::ClassTypeInfoImplementation::coreshared::NavmeshTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x142025480, "coreshared_142025480", fn_142025480 as *const ()), // coreshared::NavmeshTemplateData::ClassTypeInfoImplementation::coreshared::NavmeshTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x1420254a0, "coreshared_1420254a0", tpl_39306::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // coreshared::NavmeshTemplateData::ClassTypeInfoImplementation::coreshared::NavmeshTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x142025500, "coreshared_142025500", tpl_35191::<0x10, 0x1401de830, 0x142442de0> as *const ()), // coreshared::NavmeshTemplateData::ClassTypeInfoImplementation::coreshared::NavmeshTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x1420255d0, "std_1420255d0", tpl_550::<0x1433d85d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_
    Replacement::draft(0x1420255f0, "std_1420255f0", tpl_550::<0x1433d85d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_
    Replacement::draft(0x142025610, "std_142025610", tpl_43248::<0x8, 0x80> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_
    Replacement::draft(0x142025630, "std_142025630", tpl_1::<0x143891eb0> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_
    Replacement::draft(0x142025640, "std_142025640", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_
    Replacement::draft(0x142025650, "std_142025650", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_
    Replacement::draft(0x142025670, "std_142025670", tpl_550::<0x1433d8610, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x142025690, "std_142025690", tpl_550::<0x1433d8610, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x1420256b0, "std_1420256b0", tpl_43248::<0x8, 0x80> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x1420256d0, "std_1420256d0", tpl_1::<0x143891e50> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x1420256e0, "std_1420256e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x1420256f0, "std_1420256f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmeshResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x142025710, "std_142025710", tpl_550::<0x1433d7ff0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Fun
    Replacement::draft(0x142025730, "std_142025730", tpl_550::<0x1433d7ff0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Fun
    Replacement::draft(0x142025750, "std_142025750", tpl_43248::<0x8, 0x80> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Fun
    Replacement::draft(0x142025770, "std_142025770", tpl_1::<0x143891f10> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Fun
    Replacement::draft(0x142025780, "std_142025780", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Fun
    Replacement::draft(0x142025790, "std_142025790", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Fun
    Replacement::draft(0x1420257b0, "std_1420257b0", tpl_550::<0x1433d7fb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func
    Replacement::draft(0x1420257d0, "std_1420257d0", tpl_550::<0x1433d7fb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func
    Replacement::draft(0x1420257f0, "std_1420257f0", tpl_43248::<0x8, 0x80> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func
    Replacement::draft(0x142025810, "std_142025810", tpl_1::<0x143891df0> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func
    Replacement::draft(0x142025820, "std_142025820", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func
    Replacement::draft(0x142025830, "std_142025830", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::MercunaNavmesh3DResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func
    Replacement::draft(0x142025850, "unknown_142025850", tpl_43249::<0x80> as *const ()), // sub_142025850
    Replacement::draft(0x142025870, "unknown_142025870", tpl_34443 as *const ()), // sub_142025870
    Replacement::draft(0x142025880, "unknown_142025880", tpl_43250::<0x80> as *const ()), // sub_142025880
    Replacement::draft_throws(0x1420258a0, "coreshared_1420258a0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433d7fb8, 0x142428500> as *const (), Throws::Discard), // coreshared::MercunaNavmesh3DResource::coreshared::MercunaNavmesh3DResource::vfunc_2
    Replacement::draft(0x142025920, "coreshared_142025920", tpl_43250::<0x80> as *const ()), // sub_142025920
    Replacement::draft(0x142025980, "coreshared_142025980", tpl_43249::<0x80> as *const ()), // sub_142025980
    Replacement::draft(0x1420259b0, "coreshared_1420259b0", tpl_43250::<0x80> as *const ()), // sub_1420259b0
    Replacement::draft_throws(0x1420259d0, "coreshared_1420259d0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433d8610, 0x142428500> as *const (), Throws::Discard), // coreshared::MercunaNavmeshResource::coreshared::MercunaNavmeshResource::vfunc_2
    Replacement::draft(0x142025a50, "coreshared_142025a50", tpl_43250::<0x80> as *const ()), // sub_142025a50
    Replacement::draft(0x142025b60, "coreshared_142025b60", tpl_43251::<0xb8, 0x8, 0x4> as *const ()), // coreshared::MercunaNavmesh3DResource::coreshared::MercunaNavmesh3DResource::vfunc_25
    Replacement::draft(0x142026070, "coreshared_142026070", tpl_38771::<0xc8, 0x1401de830, 0x142428ce0> as *const ()), // coreshared::MercunaNavmesh3DResource::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DResource::ClassTypeInfoIm
    Replacement::draft(0x1420260a0, "coreshared_1420260a0", tpl_43252::<0x142428d70, 0x1433d8028, 0xb8, 0xc0, 0xc8, 0xd0> as *const ()), // coreshared::MercunaNavmesh3DResource::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DResource::ClassTypeInfoIm
    Replacement::draft(0x1420260f0, "coreshared_1420260f0", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xc8, 0x1401de830, 0x142428ce0, 0x142442c80> as *const ()), // coreshared::MercunaNavmesh3DResource::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DResource::ClassTypeInfoIm
    Replacement::draft(0x142026160, "coreshared_142026160", tpl_38774::<0xc8, 0x1401de830, 0x142428ce0, 0x142442de0> as *const ()), // coreshared::MercunaNavmesh3DResource::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DResource::ClassTypeInfoIm
    Replacement::draft(0x1420262f0, "coreshared_1420262f0", tpl_43251::<0xb8, 0x8, 0x4> as *const ()), // coreshared::MercunaNavmeshResource::coreshared::MercunaNavmeshResource::vfunc_25
    Replacement::draft(0x142026800, "coreshared_142026800", tpl_38771::<0xc8, 0x1401de830, 0x142428ce0> as *const ()), // coreshared::MercunaNavmeshResource::ClassTypeInfoImplementation::coreshared::MercunaNavmeshResource::ClassTypeInfoImplem
    Replacement::draft(0x142026830, "coreshared_142026830", tpl_43252::<0x142428d70, 0x1433d84c0, 0xb8, 0xc0, 0xc8, 0xd0> as *const ()), // coreshared::MercunaNavmeshResource::ClassTypeInfoImplementation::coreshared::MercunaNavmeshResource::ClassTypeInfoImplem
    Replacement::draft(0x142026880, "coreshared_142026880", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xc8, 0x1401de830, 0x142428ce0, 0x142442c80> as *const ()), // coreshared::MercunaNavmeshResource::ClassTypeInfoImplementation::coreshared::MercunaNavmeshResource::ClassTypeInfoImplem
    Replacement::draft(0x1420268f0, "coreshared_1420268f0", tpl_38774::<0xc8, 0x1401de830, 0x142428ce0, 0x142442de0> as *const ()), // coreshared::MercunaNavmeshResource::ClassTypeInfoImplementation::coreshared::MercunaNavmeshResource::ClassTypeInfoImplem
    Replacement::draft(0x142026eb0, "coreshared_142026eb0", tpl_35199::<0x143881580, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoIm
    Replacement::draft(0x142026fa0, "coreshared_142026fa0", tpl_35199::<0x143881580, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoIm
    Replacement::draft(0x142027180, "coreshared_142027180", tpl_35199::<0x143881580, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoIm
    Replacement::draft(0x142027270, "coreshared_142027270", tpl_35199::<0x143881580, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoIm
    Replacement::draft(0x1420272f0, "coreshared_1420272f0", tpl_35199::<0x143881580, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmesh3DMetadata::ClassTypeInfoIm
    Replacement::draft(0x142027a40, "coreshared_142027a40", tpl_35199::<0x143881570, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplem
    Replacement::draft(0x142027b30, "coreshared_142027b30", tpl_35199::<0x143881570, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplem
    Replacement::draft(0x142027d10, "coreshared_142027d10", tpl_35199::<0x143881570, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplem
    Replacement::draft(0x142027e00, "coreshared_142027e00", tpl_35199::<0x143881570, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplem
    Replacement::draft(0x142027e80, "coreshared_142027e80", tpl_35199::<0x143881570, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplementation::coreshared::MercunaNavmeshMetadata::ClassTypeInfoImplem
    Replacement::draft(0x142028350, "coreshared_142028350", fn_142028350 as *const ()), // coreshared::MercunaNavmeshTemplateData::ClassTypeInfoImplementation::coreshared::MercunaNavmeshTemplateData::ClassTypeIn
    Replacement::draft(0x142028390, "coreshared_142028390", fn_142028390 as *const ()), // coreshared::MercunaNavmeshTemplateData::ClassTypeInfoImplementation::coreshared::MercunaNavmeshTemplateData::ClassTypeIn
    Replacement::draft(0x1420284d0, "coreshared_1420284d0", tpl_35199::<0x143881558, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshTemplateData::ClassTypeInfoImplementation::coreshared::MercunaNavmeshTemplateData::ClassTypeIn
    Replacement::draft(0x142028530, "coreshared_142028530", tpl_35199::<0x143881558, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::MercunaNavmeshTemplateData::ClassTypeInfoImplementation::coreshared::MercunaNavmeshTemplateData::ClassTypeIn
    Replacement::draft(0x142028720, "coreshared_142028720", fn_142028720 as *const ()), // sub_142028720
    Replacement::draft(0x1420287b0, "coreshared_1420287b0", fn_1420287b0 as *const ()), // sub_1420287b0
    Replacement::draft(0x142028840, "coreshared_142028840", fn_142028840 as *const ()), // sub_142028840
    Replacement::draft(0x142028bf0, "coreshared_142028bf0", fn_142028bf0 as *const ()), // sub_142028bf0
    Replacement::draft(0x142028c50, "coreshared_142028c50", fn_142028c50 as *const ()), // sub_142028c50
    Replacement::draft(0x142028c80, "coreshared_142028c80", tpl_43259::<0x142028e00, 0x10, 0x1401de830> as *const ()), // sub_142028c80
    Replacement::draft(0x142028cb0, "coreshared_142028cb0", tpl_43260::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_142028cb0
    Replacement::draft(0x142028d10, "coreshared_142028d10", tpl_43260::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_142028d10
    Replacement::draft(0x142028dc0, "coreshared_142028dc0", fn_142028dc0 as *const ()), // sub_142028dc0
    Replacement::draft(0x142028e00, "coreshared_142028e00", fn_142028e00 as *const ()), // sub_142028e00
    Replacement::draft_throws(0x142028ef0, "std_142028ef0", fn_142028ef0 as *const (), Throws::Caught), // sub_142028ef0
    Replacement::draft_throws(0x142029630, "std_142029630", fn_142029630 as *const (), Throws::Discard), // sub_142029630
    Replacement::draft_throws(0x142029660, "std_142029660", fn_142029660 as *const (), Throws::Discard), // sub_142029660
    Replacement::draft_throws(0x1420298b0, "unknown_1420298b0", fn_1420298b0 as *const (), Throws::Discard), // sub_1420298b0
    Replacement::draft(0x1420298e0, "unknown_1420298e0", fn_1420298e0 as *const ()), // sub_1420298e0
    Replacement::draft(0x1420299d0, "unknown_1420299d0", fn_1420299d0 as *const ()), // sub_1420299d0
    Replacement::draft_throws(0x142029b80, "unknown_142029b80", fn_142029b80 as *const (), Throws::Discard), // sub_142029b80
    Replacement::draft(0x142029bc0, "unknown_142029bc0", tpl_43270::<0xc8, 0xa0, 0xb0, 0x8, 0x10, 0x18> as *const ()), // sub_142029bc0
    Replacement::draft_throws(0x142029cb0, "unknown_142029cb0", fn_142029cb0 as *const (), Throws::Discard), // sub_142029cb0
    Replacement::draft_throws(0x14202a230, "unknown_14202a230", fn_14202a230 as *const (), Throws::Discard), // sub_14202a230
    Replacement::draft_throws(0x14202a680, "unknown_14202a680", fn_14202a680 as *const (), Throws::Discard), // sub_14202a680
    Replacement::draft_throws(0x14202a6b0, "unknown_14202a6b0", fn_14202a6b0 as *const (), Throws::Discard), // sub_14202a6b0
    Replacement::draft_throws(0x14202a910, "unknown_14202a910", fn_14202a910 as *const (), Throws::Discard), // sub_14202a910
    Replacement::draft_throws(0x14202aa20, "unknown_14202aa20", fn_14202aa20 as *const (), Throws::Discard), // sub_14202aa20
    Replacement::draft(0x14202aa40, "unknown_14202aa40", tpl_43270::<0x88, 0x60, 0x70, 0x8, 0x10, 0x18> as *const ()), // sub_14202aa40
    Replacement::draft_throws(0x14202ab20, "unknown_14202ab20", fn_14202ab20 as *const (), Throws::Discard), // sub_14202ab20
    Replacement::draft(0x14202b080, "unknown_14202b080", fn_14202b080 as *const ()), // sub_14202b080
    Replacement::draft(0x14202b320, "unknown_14202b320", fn_14202b320 as *const ()), // sub_14202b320
    Replacement::draft(0x14202b7d0, "unknown_14202b7d0", tpl_366::<0x14397db58> as *const ()), // sub_14202b7d0
    Replacement::draft(0x14202b7e0, "unknown_14202b7e0", fn_14202b7e0 as *const ()), // sub_14202b7e0
    Replacement::draft(0x14202b800, "unknown_14202b800", fn_14202b800 as *const ()), // sub_14202b800
    Replacement::draft(0x14202b830, "unknown_14202b830", fn_14202b830 as *const ()), // sub_14202b830
    Replacement::draft(0x14202b860, "unknown_14202b860", fn_14202b860 as *const ()), // sub_14202b860
    Replacement::draft(0x14202b8c0, "unknown_14202b8c0", tpl_34252 as *const ()), // sub_14202b8c0
    Replacement::draft(0x14202bcf0, "unknown_14202bcf0", fn_14202bcf0 as *const ()), // sub_14202bcf0
    Replacement::draft(0x14202bd50, "unknown_14202bd50", fn_14202bd50 as *const ()), // sub_14202bd50
    Replacement::draft(0x14202be30, "unknown_14202be30", fn_14202be30 as *const ()), // sub_14202be30
    Replacement::draft(0x14202c0e0, "unknown_14202c0e0", fn_14202c0e0 as *const ()), // sub_14202c0e0
    Replacement::draft(0x14202c150, "std_14202c150", tpl_550::<0x1433d8b68, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x14202c170, "std_14202c170", tpl_550::<0x1433d8b68, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x14202c1a0, "std_14202c1a0", tpl_1::<0x143891fd0> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x14202c1b0, "std_14202c1b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x14202c1c0, "std_14202c1c0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x14202c1e0, "std_14202c1e0", tpl_550::<0x1433d90e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_al
    Replacement::draft(0x14202c200, "std_14202c200", tpl_550::<0x1433d90e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_al
    Replacement::draft(0x14202c230, "std_14202c230", tpl_1::<0x143891f70> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_al
    Replacement::draft(0x14202c240, "std_14202c240", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_al
    Replacement::draft(0x14202c250, "std_14202c250", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::GfxGraphResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_al
    Replacement::draft(0x14202c270, "coreshared_14202c270", fn_14202c270 as *const ()), // coreshared::GfxGraphResource::coreshared::GfxGraphResource::vfunc_13
    Replacement::draft_throws(0x14202c360, "coreshared_14202c360", fn_14202c360 as *const (), Throws::Caught), // sub_14202c360
    Replacement::draft(0x14202d260, "coreshared_14202d260", tpl_35199::<0x143881588, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202d350, "coreshared_14202d350", tpl_35199::<0x143881588, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202d530, "coreshared_14202d530", tpl_35199::<0x143881588, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202d620, "coreshared_14202d620", tpl_35199::<0x143881588, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202d6a0, "coreshared_14202d6a0", tpl_35199::<0x143881588, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::coreshared::GfxGraphMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x14202d9f0, "coreshared_14202d9f0", tpl_838::<0x1411bb670> as *const (), Throws::Discard), // coreshared::GfxGraphResource::coreshared::GfxGraphResource::vfunc_28
    Replacement::draft(0x14202de50, "coreshared_14202de50", fn_14202de50 as *const ()), // coreshared::GfxGraphResource::ClassTypeInfoImplementation::coreshared::GfxGraphResource::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202dec0, "coreshared_14202dec0", tpl_43289::<0x142428d70, 0x1433d8d60, 0xb8, 0xc0> as *const ()), // coreshared::GfxGraphResource::ClassTypeInfoImplementation::coreshared::GfxGraphResource::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202df00, "coreshared_14202df00", fn_14202df00 as *const ()), // coreshared::GfxGraphResource::ClassTypeInfoImplementation::coreshared::GfxGraphResource::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202dfc0, "coreshared_14202dfc0", fn_14202dfc0 as *const ()), // coreshared::GfxGraphResource::ClassTypeInfoImplementation::coreshared::GfxGraphResource::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14202e0e0, "unknown_14202e0e0", tpl_34443 as *const ()), // sub_14202e0e0
    Replacement::draft(0x14202e0f0, "unknown_14202e0f0", fn_14202e0f0 as *const ()), // sub_14202e0f0
    Replacement::draft(0x14202e130, "unknown_14202e130", fn_14202e130 as *const ()), // sub_14202e130
    Replacement::draft(0x14202e1e0, "unknown_14202e1e0", tpl_34443 as *const ()), // sub_14202e1e0
    Replacement::draft(0x14202e1f0, "unknown_14202e1f0", fn_14202e1f0 as *const ()), // sub_14202e1f0
    Replacement::draft(0x14202e2d0, "unknown_14202e2d0", fn_14202e2d0 as *const ()), // sub_14202e2d0
    Replacement::draft(0x14202e4b0, "unknown_14202e4b0", fn_14202e4b0 as *const ()), // sub_14202e4b0
    Replacement::draft(0x14202e5e0, "unknown_14202e5e0", fn_14202e5e0 as *const ()), // sub_14202e5e0
    Replacement::draft(0x14202e620, "unknown_14202e620", fn_14202e620 as *const ()), // sub_14202e620
    Replacement::draft(0x14202e6e0, "unknown_14202e6e0", tpl_43259::<0x14202e4b0, 0x10, 0x1401de830> as *const ()), // sub_14202e6e0
    Replacement::draft(0x14202e710, "unknown_14202e710", fn_14202e710 as *const ()), // sub_14202e710
    Replacement::draft(0x14202e780, "unknown_14202e780", fn_14202e780 as *const ()), // sub_14202e780
    Replacement::draft(0x14202e810, "unknown_14202e810", fn_14202e810 as *const ()), // sub_14202e810
    Replacement::draft(0x14202ea10, "unknown_14202ea10", tpl_34252 as *const ()), // sub_14202ea10
    Replacement::draft(0x14202ea20, "unknown_14202ea20", tpl_43302::<0x1411bc940, 0x18, 0x8, 0x10> as *const ()), // sub_14202ea20
    Replacement::draft(0x14202ed70, "unknown_14202ed70", fn_14202ed70 as *const ()), // sub_14202ed70
    Replacement::draft(0x14202edd0, "unknown_14202edd0", fn_14202edd0 as *const ()), // sub_14202edd0
    Replacement::draft(0x14202eea0, "unknown_14202eea0", fn_14202eea0 as *const ()), // sub_14202eea0
    Replacement::draft(0x14202eec0, "unknown_14202eec0", fn_14202eec0 as *const ()), // sub_14202eec0
    Replacement::draft(0x14202eee0, "unknown_14202eee0", fn_14202eee0 as *const ()), // sub_14202eee0
    Replacement::draft(0x14202ef10, "unknown_14202ef10", fn_14202ef10 as *const ()), // sub_14202ef10
    Replacement::draft(0x14202ef40, "unknown_14202ef40", fn_14202ef40 as *const ()), // sub_14202ef40
    Replacement::draft(0x14202efb0, "unknown_14202efb0", fn_14202efb0 as *const ()), // sub_14202efb0
    Replacement::draft(0x14202f000, "unknown_14202f000", fn_14202f000 as *const ()), // sub_14202f000
    Replacement::draft(0x14202f0c0, "unknown_14202f0c0", tpl_34252 as *const ()), // sub_14202f0c0
    Replacement::draft(0x14202f0d0, "unknown_14202f0d0", tpl_43302::<0x1411bc940, 0x18, 0x8, 0x10> as *const ()), // sub_14202f0d0
    Replacement::draft(0x14202f3b0, "unknown_14202f3b0", fn_14202f3b0 as *const ()), // sub_14202f3b0
    Replacement::draft(0x14202f450, "unknown_14202f450", fn_14202f450 as *const ()), // sub_14202f450
    Replacement::draft(0x14202f520, "unknown_14202f520", fn_14202f520 as *const ()), // sub_14202f520
    Replacement::draft_throws(0x14202f610, "unknown_14202f610", fn_14202f610 as *const (), Throws::Discard), // sub_14202f610
    Replacement::draft_throws(0x14202f830, "unknown_14202f830", fn_14202f830 as *const (), Throws::Discard), // sub_14202f830
    Replacement::draft_throws(0x14202f9b0, "physx_14202f9b0", tpl_43316::<0x143390550, 0x14339055f, 0x1420472c0, 0x1411c0e00, 0x1433d9af1, 0x1433d9aec, 0x1410cc220> as *const (), Throws::Discard), // sub_14202f9b0
    Replacement::draft_throws(0x14202faa0, "unknown_14202faa0", tpl_43316::<0x1433905b0, 0x1433905bd, 0x1420472c0, 0x1411c0c50, 0x1433d9af1, 0x1433d9aec, 0x1410cc220> as *const (), Throws::Discard), // sub_14202faa0
    Replacement::draft_throws(0x14202fb90, "unknown_14202fb90", fn_14202fb90 as *const (), Throws::Discard), // sub_14202fb90
    Replacement::draft_throws(0x14202fce0, "unknown_14202fce0", fn_14202fce0 as *const (), Throws::Discard), // sub_14202fce0
    Replacement::draft_throws(0x14202fe40, "unknown_14202fe40", fn_14202fe40 as *const (), Throws::Discard), // sub_14202fe40
    Replacement::draft_throws(0x14202ff40, "unknown_14202ff40", fn_14202ff40 as *const (), Throws::Discard), // sub_14202ff40
    Replacement::draft(0x142030070, "unknown_142030070", tpl_43319::<0x324, 0x320, 0x1433d9b48, 0x1433d9b38, 0x1410ccd70> as *const ()), // sub_142030070
    Replacement::draft(0x1420300e0, "unknown_1420300e0", tpl_43319::<0x314, 0x310, 0x1433d9970, 0x1433d9960, 0x1410ccd70> as *const ()), // sub_1420300e0
    Replacement::draft_throws(0x142030150, "unknown_142030150", tpl_43320::<0x328, 0x1433d9988, 0x1433d9999, 0x1411c01f0> as *const (), Throws::Discard), // sub_142030150
    Replacement::draft_throws(0x1420301c0, "unknown_1420301c0", fn_1420301c0 as *const (), Throws::Discard), // sub_1420301c0
    Replacement::draft_throws(0x142030240, "unknown_142030240", tpl_43320::<0x33c, 0x1433d9950, 0x1433d995d, 0x1411c01f0> as *const (), Throws::Discard), // sub_142030240
    Replacement::draft(0x1420302b0, "unknown_1420302b0", fn_1420302b0 as *const ()), // sub_1420302b0
    Replacement::draft_throws(0x142030330, "unknown_142030330", fn_142030330 as *const (), Throws::Discard), // sub_142030330
    Replacement::draft_throws(0x1420304a0, "unknown_1420304a0", fn_1420304a0 as *const (), Throws::Discard), // sub_1420304a0
    Replacement::draft_throws(0x142030810, "unknown_142030810", fn_142030810 as *const (), Throws::Discard), // sub_142030810
    Replacement::draft_throws(0x142030880, "unknown_142030880", fn_142030880 as *const (), Throws::Discard), // sub_142030880
    Replacement::draft(0x142030af0, "coreshared_142030af0", fn_142030af0 as *const ()), // sub_142030af0
    Replacement::draft(0x142030ca0, "coreshared_142030ca0", fn_142030ca0 as *const ()), // sub_142030ca0
    Replacement::draft(0x142030e80, "coreshared_142030e80", fn_142030e80 as *const ()), // sub_142030e80
    Replacement::draft(0x142031070, "coreshared_142031070", fn_142031070 as *const ()), // sub_142031070
    Replacement::draft_throws(0x142031230, "coreshared_142031230", fn_142031230 as *const (), Throws::Discard), // sub_142031230
    Replacement::draft_throws(0x142031390, "coreshared_142031390", fn_142031390 as *const (), Throws::Discard), // sub_142031390
    Replacement::draft(0x142031490, "coreshared_142031490", fn_142031490 as *const ()), // sub_142031490
    Replacement::draft(0x1420316e0, "coreshared_1420316e0", fn_1420316e0 as *const ()), // sub_1420316e0
    Replacement::draft(0x142031890, "coreshared_142031890", fn_142031890 as *const ()), // sub_142031890
    Replacement::draft(0x142031a80, "coreshared_142031a80", fn_142031a80 as *const ()), // sub_142031a80
    Replacement::draft_throws(0x142031c40, "coreshared_142031c40", fn_142031c40 as *const (), Throws::Caught), // sub_142031c40
    Replacement::draft_throws(0x142031db0, "coreshared_142031db0", fn_142031db0 as *const (), Throws::Discard), // sub_142031db0
    Replacement::draft_throws(0x142031ee0, "coreshared_142031ee0", fn_142031ee0 as *const (), Throws::Discard), // sub_142031ee0
    Replacement::draft_throws(0x142032090, "coreshared_142032090", fn_142032090 as *const (), Throws::Caught), // sub_142032090
    Replacement::draft_throws(0x1420324e0, "coreshared_1420324e0", fn_1420324e0 as *const (), Throws::Caught), // sub_1420324e0
    Replacement::draft_throws(0x142032a60, "coreshared_142032a60", fn_142032a60 as *const (), Throws::Caught), // sub_142032a60
    Replacement::draft_throws(0x142032ca0, "coreshared_142032ca0", fn_142032ca0 as *const (), Throws::Caught), // sub_142032ca0
    Replacement::draft_throws(0x142032e80, "coreshared_142032e80", fn_142032e80 as *const (), Throws::Discard), // sub_142032e80
    Replacement::draft_throws(0x142033310, "coreshared_142033310", fn_142033310 as *const (), Throws::Discard), // sub_142033310
    Replacement::draft(0x1420337c0, "coreshared_1420337c0", fn_1420337c0 as *const ()), // sub_1420337c0
    Replacement::draft_throws(0x142033920, "coreshared_142033920", fn_142033920 as *const (), Throws::Discard), // sub_142033920
    Replacement::draft_throws(0x142033a50, "coreshared_142033a50", fn_142033a50 as *const (), Throws::Discard), // sub_142033a50
    Replacement::draft_throws(0x142033b80, "coreshared_142033b80", fn_142033b80 as *const (), Throws::Caught), // sub_142033b80
    Replacement::draft_throws(0x142033d70, "coreshared_142033d70", fn_142033d70 as *const (), Throws::Caught), // sub_142033d70
    Replacement::draft_throws(0x142034100, "coreshared_142034100", fn_142034100 as *const (), Throws::Caught), // sub_142034100
    Replacement::draft_throws(0x1420343e0, "coreshared_1420343e0", fn_1420343e0 as *const (), Throws::Discard), // sub_1420343e0
    Replacement::draft_throws(0x142034570, "coreshared_142034570", fn_142034570 as *const (), Throws::Discard), // sub_142034570
    Replacement::draft_throws(0x1420346e0, "coreshared_1420346e0", fn_1420346e0 as *const (), Throws::Discard), // sub_1420346e0
    Replacement::draft_throws(0x142034810, "coreshared_142034810", fn_142034810 as *const (), Throws::Caught), // sub_142034810
    Replacement::draft_throws(0x1420349d0, "coreshared_1420349d0", fn_1420349d0 as *const (), Throws::Caught), // sub_1420349d0
    Replacement::draft_throws(0x142034cf0, "unknown_142034cf0", fn_142034cf0 as *const (), Throws::Caught), // sub_142034cf0
    Replacement::draft_throws(0x142035090, "physx_142035090", fn_142035090 as *const (), Throws::Caught), // sub_142035090
    Replacement::draft_throws(0x1420353c0, "coreshared_1420353c0", fn_1420353c0 as *const (), Throws::Caught), // sub_1420353c0
    Replacement::draft_throws(0x1420357c0, "coreshared_1420357c0", fn_1420357c0 as *const (), Throws::Caught), // sub_1420357c0
    Replacement::draft_throws(0x142035c20, "coreshared_142035c20", fn_142035c20 as *const (), Throws::Caught), // sub_142035c20
    Replacement::draft_throws(0x142035f80, "coreshared_142035f80", fn_142035f80 as *const (), Throws::Caught), // sub_142035f80
    Replacement::draft_throws(0x142036470, "coreshared_142036470", fn_142036470 as *const (), Throws::Caught), // sub_142036470
    Replacement::draft_throws(0x142036840, "coreshared_142036840", fn_142036840 as *const (), Throws::Caught), // sub_142036840
    Replacement::draft_throws(0x142036d90, "coreshared_142036d90", fn_142036d90 as *const (), Throws::Caught), // sub_142036d90
    Replacement::draft_throws(0x142037160, "coreshared_142037160", fn_142037160 as *const (), Throws::Caught), // sub_142037160
    Replacement::draft_throws(0x142037460, "coreshared_142037460", fn_142037460 as *const (), Throws::Caught), // sub_142037460
    Replacement::draft_throws(0x142037830, "coreshared_142037830", fn_142037830 as *const (), Throws::Caught), // sub_142037830
    Replacement::draft_throws(0x142037c20, "coreshared_142037c20", fn_142037c20 as *const (), Throws::Caught), // sub_142037c20
    Replacement::draft_throws(0x142038140, "coreshared_142038140", fn_142038140 as *const (), Throws::Caught), // sub_142038140
    Replacement::draft_throws(0x142038570, "coreshared_142038570", fn_142038570 as *const (), Throws::Discard), // sub_142038570
    Replacement::draft_throws(0x142038660, "coreshared_142038660", tpl_43363::<0x14329db90, 0x14329db98, 0x1410cc030, 0x1432a90d1, 0x1432a90cc, 0x1420472c0, 0x1411c11b0> as *const (), Throws::Discard), // sub_142038660
    Replacement::draft_throws(0x142038780, "coreshared_142038780", fn_142038780 as *const (), Throws::Caught), // sub_142038780
    Replacement::draft_throws(0x1420389d0, "coreshared_1420389d0", fn_1420389d0 as *const (), Throws::Discard), // sub_1420389d0
    Replacement::draft_throws(0x142038b20, "coreshared_142038b20", tpl_43363::<0x14329db90, 0x14329db98, 0x1410cc030, 0x1432a90d1, 0x1432a90cc, 0x1420472c0, 0x1411c11b0> as *const (), Throws::Discard), // sub_142038b20
    Replacement::draft_throws(0x142038c40, "coreshared_142038c40", fn_142038c40 as *const (), Throws::Discard), // sub_142038c40
    Replacement::draft(0x142038d80, "coreshared_142038d80", fn_142038d80 as *const ()), // sub_142038d80
    Replacement::draft(0x142038dc0, "coreshared_142038dc0", fn_142038dc0 as *const ()), // sub_142038dc0
    Replacement::draft(0x142038e90, "coreshared_142038e90", fn_142038e90 as *const ()), // sub_142038e90
    Replacement::draft(0x1420396d0, "coreshared_1420396d0", fn_1420396d0 as *const ()), // sub_1420396d0
    Replacement::draft(0x1420396f0, "coreshared_1420396f0", fn_1420396f0 as *const ()), // sub_1420396f0
    Replacement::draft(0x142039810, "coreshared_142039810", tpl_20244::<0x8, 0x4, 0xc, 0x14, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30> as *const ()), // sub_142039810
    Replacement::draft(0x1420398a0, "coreshared_1420398a0", fn_1420398a0 as *const ()), // sub_1420398a0
    Replacement::draft(0x142039a00, "coreshared_142039a00", fn_142039a00 as *const ()), // sub_142039a00
    Replacement::draft(0x142039a70, "coreshared_142039a70", fn_142039a70 as *const ()), // sub_142039a70
    Replacement::draft(0x142039b40, "coreshared_142039b40", fn_142039b40 as *const ()), // sub_142039b40
    Replacement::draft(0x142039b80, "coreshared_142039b80", fn_142039b80 as *const ()), // sub_142039b80
    Replacement::draft(0x142039c20, "coreshared_142039c20", fn_142039c20 as *const ()), // sub_142039c20
    Replacement::draft_throws(0x14203b190, "coreshared_14203b190", fn_14203b190 as *const (), Throws::Discard), // sub_14203b190
    Replacement::draft_throws(0x14203b280, "coreshared_14203b280", fn_14203b280 as *const (), Throws::Discard), // sub_14203b280
    Replacement::draft_throws(0x14203b370, "coreshared_14203b370", fn_14203b370 as *const (), Throws::Discard), // sub_14203b370
    Replacement::draft_throws(0x14203b440, "coreshared_14203b440", fn_14203b440 as *const (), Throws::Discard), // sub_14203b440
    Replacement::draft_throws(0x14203b5e0, "coreshared_14203b5e0", fn_14203b5e0 as *const (), Throws::Discard), // sub_14203b5e0
    Replacement::draft_throws(0x14203b750, "coreshared_14203b750", fn_14203b750 as *const (), Throws::Discard), // sub_14203b750
    Replacement::draft_throws(0x14203b890, "coreshared_14203b890", fn_14203b890 as *const (), Throws::Discard), // sub_14203b890
    Replacement::draft_throws(0x14203b9c0, "coreshared_14203b9c0", fn_14203b9c0 as *const (), Throws::Discard), // sub_14203b9c0
    Replacement::draft_throws(0x14203bae0, "coreshared_14203bae0", fn_14203bae0 as *const (), Throws::Discard), // sub_14203bae0
    Replacement::draft_throws(0x14203bc00, "coreshared_14203bc00", fn_14203bc00 as *const (), Throws::Discard), // sub_14203bc00
    Replacement::draft_throws(0x14203bd30, "coreshared_14203bd30", fn_14203bd30 as *const (), Throws::Discard), // sub_14203bd30
    Replacement::draft_throws(0x14203be50, "coreshared_14203be50", fn_14203be50 as *const (), Throws::Discard), // sub_14203be50
    Replacement::draft_throws(0x14203bf70, "coreshared_14203bf70", fn_14203bf70 as *const (), Throws::Discard), // sub_14203bf70
    Replacement::draft_throws(0x14203c070, "coreshared_14203c070", fn_14203c070 as *const (), Throws::Discard), // sub_14203c070
    Replacement::draft(0x14203c1b0, "coreshared_14203c1b0", fn_14203c1b0 as *const ()), // sub_14203c1b0
    Replacement::draft_throws(0x14203c2e0, "coreshared_14203c2e0", fn_14203c2e0 as *const (), Throws::Discard), // sub_14203c2e0
    Replacement::draft(0x14203c430, "coreshared_14203c430", fn_14203c430 as *const ()), // sub_14203c430
    Replacement::draft_throws(0x14203c570, "coreshared_14203c570", fn_14203c570 as *const (), Throws::Discard), // sub_14203c570
    Replacement::draft(0x14203c6c0, "coreshared_14203c6c0", fn_14203c6c0 as *const ()), // sub_14203c6c0
    Replacement::draft_throws(0x14203c7f0, "coreshared_14203c7f0", fn_14203c7f0 as *const (), Throws::Discard), // sub_14203c7f0
    Replacement::draft(0x14203c8f0, "coreshared_14203c8f0", fn_14203c8f0 as *const ()), // sub_14203c8f0
    Replacement::draft_throws(0x14203c9f0, "coreshared_14203c9f0", fn_14203c9f0 as *const (), Throws::Discard), // sub_14203c9f0
    Replacement::draft_throws(0x14203cad0, "coreshared_14203cad0", fn_14203cad0 as *const (), Throws::Discard), // sub_14203cad0
    Replacement::draft_throws(0x14203cbb0, "coreshared_14203cbb0", fn_14203cbb0 as *const (), Throws::Discard), // sub_14203cbb0
    Replacement::draft_throws(0x14203cc90, "coreshared_14203cc90", fn_14203cc90 as *const (), Throws::Discard), // sub_14203cc90
    Replacement::draft_throws(0x14203cdd0, "coreshared_14203cdd0", fn_14203cdd0 as *const (), Throws::Discard), // sub_14203cdd0
    Replacement::draft(0x14203cf00, "coreshared_14203cf00", fn_14203cf00 as *const ()), // sub_14203cf00
    Replacement::draft(0x14203d000, "coreshared_14203d000", fn_14203d000 as *const ()), // sub_14203d000
    Replacement::draft_throws(0x14203d100, "coreshared_14203d100", fn_14203d100 as *const (), Throws::Discard), // sub_14203d100
    Replacement::draft_throws(0x14203d220, "coreshared_14203d220", fn_14203d220 as *const (), Throws::Discard), // sub_14203d220
    Replacement::draft(0x14203d370, "coreshared_14203d370", fn_14203d370 as *const ()), // sub_14203d370
    Replacement::draft(0x14203d480, "coreshared_14203d480", fn_14203d480 as *const ()), // sub_14203d480
    Replacement::draft_throws(0x14203d5b0, "coreshared_14203d5b0", fn_14203d5b0 as *const (), Throws::Discard), // sub_14203d5b0
    Replacement::draft_throws(0x14203d6f0, "coreshared_14203d6f0", fn_14203d6f0 as *const (), Throws::Discard), // sub_14203d6f0
    Replacement::draft(0x14203d820, "coreshared_14203d820", fn_14203d820 as *const ()), // sub_14203d820
    Replacement::draft_throws(0x14203d940, "coreshared_14203d940", fn_14203d940 as *const (), Throws::Discard), // sub_14203d940
    Replacement::draft_throws(0x14203da50, "coreshared_14203da50", fn_14203da50 as *const (), Throws::Discard), // sub_14203da50
    Replacement::draft_throws(0x14203db70, "coreshared_14203db70", fn_14203db70 as *const (), Throws::Discard), // sub_14203db70
    Replacement::draft(0x14203dcc0, "coreshared_14203dcc0", fn_14203dcc0 as *const ()), // sub_14203dcc0
    Replacement::draft(0x14203ddd0, "coreshared_14203ddd0", fn_14203ddd0 as *const ()), // sub_14203ddd0
    Replacement::draft_throws(0x14203df00, "coreshared_14203df00", fn_14203df00 as *const (), Throws::Discard), // sub_14203df00
    Replacement::draft_throws(0x14203e040, "coreshared_14203e040", fn_14203e040 as *const (), Throws::Discard), // sub_14203e040
    Replacement::draft(0x14203e170, "coreshared_14203e170", fn_14203e170 as *const ()), // sub_14203e170
    Replacement::draft_throws(0x14203e290, "coreshared_14203e290", fn_14203e290 as *const (), Throws::Discard), // sub_14203e290
    Replacement::draft_throws(0x14203e3a0, "coreshared_14203e3a0", fn_14203e3a0 as *const (), Throws::Discard), // sub_14203e3a0
    Replacement::draft_throws(0x14203e570, "coreshared_14203e570", fn_14203e570 as *const (), Throws::Discard), // sub_14203e570
    Replacement::draft_throws(0x14203e760, "coreshared_14203e760", fn_14203e760 as *const (), Throws::Discard), // sub_14203e760
    Replacement::draft_throws(0x14203e860, "coreshared_14203e860", fn_14203e860 as *const (), Throws::Discard), // sub_14203e860
    Replacement::draft_throws(0x14203e9c0, "coreshared_14203e9c0", fn_14203e9c0 as *const (), Throws::Discard), // sub_14203e9c0
    Replacement::draft_throws(0x14203eac0, "coreshared_14203eac0", fn_14203eac0 as *const (), Throws::Discard), // sub_14203eac0
    Replacement::draft_throws(0x14203ed40, "coreshared_14203ed40", fn_14203ed40 as *const (), Throws::Discard), // sub_14203ed40
    Replacement::draft_throws(0x14203ee80, "coreshared_14203ee80", fn_14203ee80 as *const (), Throws::Discard), // sub_14203ee80
    Replacement::draft_throws(0x14203efd0, "coreshared_14203efd0", fn_14203efd0 as *const (), Throws::Discard), // sub_14203efd0
    Replacement::draft(0x14203f120, "coreshared_14203f120", fn_14203f120 as *const ()), // sub_14203f120
    Replacement::draft(0x14203f260, "coreshared_14203f260", fn_14203f260 as *const ()), // sub_14203f260
    Replacement::draft_throws(0x14203f3a0, "coreshared_14203f3a0", fn_14203f3a0 as *const (), Throws::Discard), // sub_14203f3a0
    Replacement::draft_throws(0x14203f4f0, "coreshared_14203f4f0", fn_14203f4f0 as *const (), Throws::Discard), // sub_14203f4f0
    Replacement::draft_throws(0x14203f630, "coreshared_14203f630", fn_14203f630 as *const (), Throws::Discard), // sub_14203f630
    Replacement::draft_throws(0x14203f760, "coreshared_14203f760", fn_14203f760 as *const (), Throws::Discard), // sub_14203f760
    Replacement::draft_throws(0x14203f870, "coreshared_14203f870", fn_14203f870 as *const (), Throws::Discard), // sub_14203f870
    Replacement::draft_throws(0x14203f9c0, "coreshared_14203f9c0", fn_14203f9c0 as *const (), Throws::Discard), // sub_14203f9c0
    Replacement::draft_throws(0x14203fb60, "coreshared_14203fb60", fn_14203fb60 as *const (), Throws::Discard), // sub_14203fb60
    Replacement::draft_throws(0x14203fcd0, "coreshared_14203fcd0", fn_14203fcd0 as *const (), Throws::Discard), // sub_14203fcd0
    Replacement::draft_throws(0x14203fde0, "coreshared_14203fde0", fn_14203fde0 as *const (), Throws::Discard), // sub_14203fde0
    Replacement::draft(0x14203ff30, "coreshared_14203ff30", fn_14203ff30 as *const ()), // sub_14203ff30
    Replacement::draft(0x142040040, "coreshared_142040040", fn_142040040 as *const ()), // sub_142040040
    Replacement::draft_throws(0x142040160, "coreshared_142040160", fn_142040160 as *const (), Throws::Discard), // sub_142040160
    Replacement::draft_throws(0x142040290, "coreshared_142040290", fn_142040290 as *const (), Throws::Discard), // sub_142040290
    Replacement::draft(0x1420403b0, "coreshared_1420403b0", fn_1420403b0 as *const ()), // sub_1420403b0
    Replacement::draft_throws(0x1420404d0, "coreshared_1420404d0", fn_1420404d0 as *const (), Throws::Discard), // sub_1420404d0
    Replacement::draft_throws(0x1420405d0, "coreshared_1420405d0", fn_1420405d0 as *const (), Throws::Discard), // sub_1420405d0
    Replacement::draft_throws(0x142040720, "coreshared_142040720", fn_142040720 as *const (), Throws::Discard), // sub_142040720
    Replacement::draft(0x142040870, "coreshared_142040870", fn_142040870 as *const ()), // sub_142040870
    Replacement::draft(0x142040990, "coreshared_142040990", fn_142040990 as *const ()), // sub_142040990
    Replacement::draft_throws(0x142040ab0, "coreshared_142040ab0", fn_142040ab0 as *const (), Throws::Discard), // sub_142040ab0
    Replacement::draft_throws(0x142040bf0, "coreshared_142040bf0", fn_142040bf0 as *const (), Throws::Discard), // sub_142040bf0
    Replacement::draft(0x142040d20, "coreshared_142040d20", fn_142040d20 as *const ()), // sub_142040d20
    Replacement::draft_throws(0x142040e40, "coreshared_142040e40", fn_142040e40 as *const (), Throws::Discard), // sub_142040e40
    Replacement::draft_throws(0x142040f40, "coreshared_142040f40", fn_142040f40 as *const (), Throws::Discard), // sub_142040f40
    Replacement::draft_throws(0x142041090, "coreshared_142041090", fn_142041090 as *const (), Throws::Discard), // sub_142041090
    Replacement::draft(0x1420411e0, "coreshared_1420411e0", fn_1420411e0 as *const ()), // sub_1420411e0
    Replacement::draft(0x142041300, "coreshared_142041300", fn_142041300 as *const ()), // sub_142041300
    Replacement::draft_throws(0x142041420, "coreshared_142041420", fn_142041420 as *const (), Throws::Discard), // sub_142041420
    Replacement::draft_throws(0x142041560, "coreshared_142041560", fn_142041560 as *const (), Throws::Discard), // sub_142041560
    Replacement::draft(0x142041690, "coreshared_142041690", fn_142041690 as *const ()), // sub_142041690
    Replacement::draft_throws(0x1420417b0, "coreshared_1420417b0", fn_1420417b0 as *const (), Throws::Discard), // sub_1420417b0
    Replacement::draft_throws(0x1420418b0, "coreshared_1420418b0", fn_1420418b0 as *const (), Throws::Discard), // sub_1420418b0
    Replacement::draft_throws(0x1420419b0, "coreshared_1420419b0", fn_1420419b0 as *const (), Throws::Discard), // sub_1420419b0
    Replacement::draft(0x142041ab0, "coreshared_142041ab0", fn_142041ab0 as *const ()), // sub_142041ab0
    Replacement::draft_throws(0x142041b90, "coreshared_142041b90", fn_142041b90 as *const (), Throws::Discard), // sub_142041b90
    Replacement::draft_throws(0x142041c70, "coreshared_142041c70", fn_142041c70 as *const (), Throws::Discard), // sub_142041c70
    Replacement::draft_throws(0x142041d50, "coreshared_142041d50", fn_142041d50 as *const (), Throws::Discard), // sub_142041d50
    Replacement::draft(0x142041e40, "coreshared_142041e40", fn_142041e40 as *const ()), // sub_142041e40
    Replacement::draft_throws(0x142041f20, "coreshared_142041f20", fn_142041f20 as *const (), Throws::Discard), // sub_142041f20
    Replacement::draft_throws(0x142041ff0, "coreshared_142041ff0", fn_142041ff0 as *const (), Throws::Discard), // sub_142041ff0
    Replacement::draft_throws(0x142042140, "coreshared_142042140", fn_142042140 as *const (), Throws::Discard), // sub_142042140
    Replacement::draft_throws(0x1420422c0, "coreshared_1420422c0", fn_1420422c0 as *const (), Throws::Discard), // sub_1420422c0
    Replacement::draft_throws(0x142042490, "coreshared_142042490", fn_142042490 as *const (), Throws::Discard), // sub_142042490
    Replacement::draft(0x142042610, "coreshared_142042610", fn_142042610 as *const ()), // sub_142042610
    Replacement::draft_throws(0x142042740, "coreshared_142042740", fn_142042740 as *const (), Throws::Discard), // sub_142042740
    Replacement::draft_throws(0x1420428d0, "coreshared_1420428d0", fn_1420428d0 as *const (), Throws::Discard), // sub_1420428d0
    Replacement::draft_throws(0x142042a20, "coreshared_142042a20", fn_142042a20 as *const (), Throws::Discard), // sub_142042a20
    Replacement::draft_throws(0x142042b30, "coreshared_142042b30", fn_142042b30 as *const (), Throws::Discard), // sub_142042b30
    Replacement::draft_throws(0x142042cf0, "coreshared_142042cf0", fn_142042cf0 as *const (), Throws::Discard), // sub_142042cf0
    Replacement::draft(0x142042e50, "coreshared_142042e50", fn_142042e50 as *const ()), // sub_142042e50
    Replacement::draft_throws(0x142042f60, "coreshared_142042f60", fn_142042f60 as *const (), Throws::Discard), // sub_142042f60
    Replacement::draft_throws(0x1420430f0, "coreshared_1420430f0", fn_1420430f0 as *const (), Throws::Discard), // sub_1420430f0
    Replacement::draft_throws(0x142043240, "coreshared_142043240", fn_142043240 as *const (), Throws::Discard), // sub_142043240
    Replacement::draft(0x142043350, "coreshared_142043350", fn_142043350 as *const ()), // sub_142043350
    Replacement::draft_throws(0x142045570, "coreshared_142045570", fn_142045570 as *const (), Throws::Caught), // sub_142045570
    Replacement::draft_throws(0x142045630, "coreshared_142045630", fn_142045630 as *const (), Throws::Caught), // sub_142045630
    Replacement::draft_throws(0x1420456f0, "coreshared_1420456f0", fn_1420456f0 as *const (), Throws::Caught), // sub_1420456f0
    Replacement::draft(0x1420459d0, "coreshared_1420459d0", fn_1420459d0 as *const ()), // sub_1420459d0
    Replacement::draft(0x1420460a0, "coreshared_1420460a0", fn_1420460a0 as *const ()), // sub_1420460a0
    Replacement::draft(0x1420460d0, "coreshared_1420460d0", fn_1420460d0 as *const ()), // sub_1420460d0
    Replacement::draft(0x1420468a0, "coreshared_1420468a0", fn_1420468a0 as *const ()), // sub_1420468a0
    Replacement::draft_throws(0x142046900, "coreshared_142046900", fn_142046900 as *const (), Throws::Discard), // sub_142046900
    Replacement::draft(0x142046950, "coreshared_142046950", fn_142046950 as *const ()), // sub_142046950
    Replacement::draft(0x142046a00, "coreshared_142046a00", fn_142046a00 as *const ()), // sub_142046a00
    Replacement::draft_throws(0x142046d60, "coreshared_142046d60", fn_142046d60 as *const (), Throws::Caught), // sub_142046d60
    Replacement::draft(0x142046e60, "coreshared_142046e60", fn_142046e60 as *const ()), // sub_142046e60
    Replacement::draft(0x142047190, "coreshared_142047190", fn_142047190 as *const ()), // sub_142047190
    Replacement::draft(0x142047220, "coreshared_142047220", fn_142047220 as *const ()), // sub_142047220
    Replacement::draft(0x142047260, "coreshared_142047260", fn_142047260 as *const ()), // sub_142047260
    Replacement::draft(0x142047330, "coreshared_142047330", fn_142047330 as *const ()), // sub_142047330
    Replacement::draft_throws(0x1420473d0, "coreshared_1420473d0", fn_1420473d0 as *const (), Throws::Discard), // sub_1420473d0
    Replacement::draft_throws(0x142047650, "coreshared_142047650", fn_142047650 as *const (), Throws::Discard), // sub_142047650
    Replacement::draft(0x1420476f0, "coreshared_1420476f0", fn_1420476f0 as *const ()), // sub_1420476f0
    Replacement::draft_throws(0x142047790, "coreshared_142047790", fn_142047790 as *const (), Throws::Caught), // sub_142047790
    Replacement::draft(0x142047b30, "coreshared_142047b30", fn_142047b30 as *const ()), // sub_142047b30
    Replacement::draft(0x142047b90, "coreshared_142047b90", fn_142047b90 as *const ()), // sub_142047b90
    Replacement::draft_throws(0x142047d00, "coreshared_142047d00", tpl_838::<0x1411cdb70> as *const (), Throws::Discard), // coreshared::FiniteStateMachineResource::coreshared::FiniteStateMachineResource::vfunc_22
    Replacement::draft_throws(0x142047d10, "coreshared_142047d10", tpl_838::<0x1411cdb70> as *const (), Throws::Discard), // coreshared::FiniteStateMachineResource::coreshared::FiniteStateMachineResource::vfunc_23
    Replacement::draft_throws(0x142047d20, "coreshared_142047d20", tpl_838::<0x1411cdb70> as *const (), Throws::Discard), // coreshared::FiniteStateMachineResource::coreshared::FiniteStateMachineResource::vfunc_24
    Replacement::draft(0x142048820, "coreshared_142048820", tpl_35199::<0x1438815c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::FiniteStateMachineMetadata::ClassTypeInfoImplementation::coreshared::FiniteStateMachineMetadata::ClassTypeIn
    Replacement::draft(0x142048910, "coreshared_142048910", tpl_35199::<0x1438815c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::FiniteStateMachineMetadata::ClassTypeInfoImplementation::coreshared::FiniteStateMachineMetadata::ClassTypeIn
    Replacement::draft(0x142048af0, "coreshared_142048af0", tpl_35199::<0x1438815c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::FiniteStateMachineMetadata::ClassTypeInfoImplementation::coreshared::FiniteStateMachineMetadata::ClassTypeIn
    Replacement::draft(0x142048be0, "coreshared_142048be0", tpl_35199::<0x1438815c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::FiniteStateMachineMetadata::ClassTypeInfoImplementation::coreshared::FiniteStateMachineMetadata::ClassTypeIn
    Replacement::draft(0x142048c60, "coreshared_142048c60", tpl_35199::<0x1438815c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::FiniteStateMachineMetadata::ClassTypeInfoImplementation::coreshared::FiniteStateMachineMetadata::ClassTypeIn
    Replacement::draft(0x142049a70, "coreshared_142049a70", tpl_35199::<0x1438815e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x142049b60, "coreshared_142049b60", tpl_35199::<0x1438815e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x142049d40, "coreshared_142049d40", tpl_35199::<0x1438815e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x142049e30, "coreshared_142049e30", tpl_35199::<0x1438815e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x142049eb0, "coreshared_142049eb0", tpl_35199::<0x1438815e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::coreshared::EQSQueryMetadata::ClassTypeInfoImplementation::vf
    Replacement::draft(0x14204a120, "coreshared_14204a120", fn_14204a120 as *const ()), // sub_14204a120
    Replacement::draft(0x14204a140, "coreshared_14204a140", fn_14204a140 as *const ()), // sub_14204a140
    Replacement::draft(0x14204a880, "coreshared_14204a880", fn_14204a880 as *const ()), // sub_14204a880
    Replacement::draft(0x14204b520, "coreshared_14204b520", tpl_35199::<0x143881600, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementation::coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x14204b610, "coreshared_14204b610", tpl_35199::<0x143881600, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementation::coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x14204b7f0, "coreshared_14204b7f0", tpl_35199::<0x143881600, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementation::coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x14204b8e0, "coreshared_14204b8e0", tpl_35199::<0x143881600, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementation::coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x14204b960, "coreshared_14204b960", tpl_35199::<0x143881600, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementation::coreshared::BehaviorTreeMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x14204bbd0, "std_14204bbd0", tpl_550::<0x1433dbec0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x14204bbf0, "std_14204bbf0", tpl_550::<0x1433dbec0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x14204bc20, "std_14204bc20", tpl_1::<0x143892430> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x14204bc30, "std_14204bc30", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x14204bc40, "std_14204bc40", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x14204bc60, "std_14204bc60", tpl_550::<0x1433db9f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x14204bc80, "std_14204bc80", tpl_550::<0x1433db9f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x14204bcb0, "std_14204bcb0", tpl_1::<0x1438923d0> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x14204bcc0, "std_14204bcc0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x14204bcd0, "std_14204bcd0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coreshared::AICoverResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft_throws(0x14204bcf0, "unknown_14204bcf0", fn_14204bcf0 as *const (), Throws::Caught), // sub_14204bcf0
    Replacement::draft_throws(0x14204c150, "coreshared_14204c150", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433db9f0, 0x142428500> as *const (), Throws::Discard), // coreshared::AICoverResource::coreshared::AICoverResource::vfunc_2
    Replacement::draft(0x14204c2d0, "coreshared_14204c2d0", fn_14204c2d0 as *const ()), // coreshared::AICoverResource::coreshared::AICoverResource::vfunc_25
    Replacement::draft(0x14204c5b0, "coreshared_14204c5b0", tpl_43509::<0x8, 0xbc, 0xc0, 0xc4> as *const ()), // coreshared::AICoverResource::ClassTypeInfoImplementation::coreshared::AICoverResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204c5f0, "coreshared_14204c5f0", tpl_43509::<0x8, 0xbc, 0xc0, 0xc4> as *const ()), // coreshared::AICoverResource::ClassTypeInfoImplementation::coreshared::AICoverResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204c7e0, "coreshared_14204c7e0", tpl_38771::<0xc8, 0x1401de830, 0x142428ce0> as *const ()), // coreshared::AICoverResource::ClassTypeInfoImplementation::coreshared::AICoverResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204c810, "coreshared_14204c810", fn_14204c810 as *const ()), // coreshared::AICoverResource::ClassTypeInfoImplementation::coreshared::AICoverResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204c860, "coreshared_14204c860", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xc8, 0x1401de830, 0x142428ce0, 0x142442c80> as *const ()), // coreshared::AICoverResource::ClassTypeInfoImplementation::coreshared::AICoverResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204c8d0, "coreshared_14204c8d0", tpl_38774::<0xc8, 0x1401de830, 0x142428ce0, 0x142442de0> as *const ()), // coreshared::AICoverResource::ClassTypeInfoImplementation::coreshared::AICoverResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204ce90, "coreshared_14204ce90", tpl_35199::<0x143881628, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverMetadata::ClassTypeInfoImplementation::coreshared::AICoverMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204cf80, "coreshared_14204cf80", tpl_35199::<0x143881628, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverMetadata::ClassTypeInfoImplementation::coreshared::AICoverMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204d160, "coreshared_14204d160", tpl_35199::<0x143881628, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverMetadata::ClassTypeInfoImplementation::coreshared::AICoverMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204d250, "coreshared_14204d250", tpl_35199::<0x143881628, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverMetadata::ClassTypeInfoImplementation::coreshared::AICoverMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204d2d0, "coreshared_14204d2d0", tpl_35199::<0x143881628, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverMetadata::ClassTypeInfoImplementation::coreshared::AICoverMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x14204d7e0, "coreshared_14204d7e0", tpl_39533::<0x8, 0x4> as *const ()), // coreshared::AICoverTemplateData::ClassTypeInfoImplementation::coreshared::AICoverTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x14204d920, "coreshared_14204d920", tpl_35199::<0x143881620, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverTemplateData::ClassTypeInfoImplementation::coreshared::AICoverTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x14204d980, "coreshared_14204d980", tpl_35199::<0x143881620, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // coreshared::AICoverTemplateData::ClassTypeInfoImplementation::coreshared::AICoverTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x14204da30, "coreshared_14204da30", tpl_34638::<0x8> as *const ()), // coreshared::AICoverTemplateData::ClassTypeInfoImplementation::coreshared::AICoverTemplateData::ClassTypeInfoImplementati
    Replacement::draft(0x14204db70, "unknown_14204db70", tpl_25145::<0x142645750> as *const ()), // sub_14204db70
    Replacement::draft(0x14204e350, "unknown_14204e350", tpl_43520::<0x14397fb80, 0x1426455a8, 0x8, 0xa, 0x50, 0x58, 0x1, 0x14209dc80, 0x98, 0x14243b3e0, 0x142645598> as *const ()), // sub_14204e350
    Replacement::draft_throws(0x14204f0e0, "unknown_14204f0e0", fn_14204f0e0 as *const (), Throws::Caught), // sub_14204f0e0
    Replacement::draft(0x14204f840, "unknown_14204f840", tpl_42053::<0x8, 0xc, 0x4> as *const ()), // sub_14204f840
    Replacement::draft_throws(0x14204f920, "unknown_14204f920", fn_14204f920 as *const (), Throws::Caught), // sub_14204f920
    Replacement::draft_throws(0x14204fce0, "unknown_14204fce0", fn_14204fce0 as *const (), Throws::Caught), // sub_14204fce0
    Replacement::draft_throws(0x14204fed0, "unknown_14204fed0", fn_14204fed0 as *const (), Throws::Caught), // sub_14204fed0
    Replacement::draft(0x142050970, "unknown_142050970", fn_142050970 as *const ()), // sub_142050970
    Replacement::draft(0x142051450, "unknown_142051450", fn_142051450 as *const ()), // sub_142051450
    Replacement::draft(0x1420516e0, "unknown_1420516e0", tpl_43569::<0x6, 0x8> as *const ()), // sub_1420516e0
    Replacement::draft_throws(0x142051710, "unknown_142051710", fn_142051710 as *const (), Throws::Discard), // sub_142051710
    Replacement::draft(0x142051cd0, "unknown_142051cd0", tpl_369 as *const ()), // sub_142051cd0
    Replacement::draft(0x142051ce0, "unknown_142051ce0", tpl_43573::<0x14397fb90, 0xc0> as *const ()), // sub_142051ce0
    Replacement::draft(0x142051db0, "unknown_142051db0", fn_142051db0 as *const ()), // sub_142051db0
    Replacement::draft(0x1420526d0, "unknown_1420526d0", tpl_369 as *const ()), // sub_1420526d0
    Replacement::draft(0x1420526e0, "unknown_1420526e0", tpl_43573::<0x14397fb98, 0xb0> as *const ()), // sub_1420526e0
    Replacement::draft(0x142052ad0, "unknown_142052ad0", tpl_559 as *const ()), // sub_142052ad0
    Replacement::draft_throws(0x142052c00, "unknown_142052c00", fn_142052c00 as *const (), Throws::Caught), // sub_142052c00
    Replacement::draft(0x142052e40, "unknown_142052e40", tpl_34443 as *const ()), // sub_142052e40
    Replacement::draft_throws(0x142052e70, "unknown_142052e70", fn_142052e70 as *const (), Throws::Caught), // sub_142052e70
    Replacement::draft(0x1420533a0, "unknown_1420533a0", tpl_1044::<0x14397db78> as *const ()), // sub_1420533a0
    Replacement::draft(0x142053480, "unknown_142053480", fn_142053480 as *const ()), // sub_142053480
    Replacement::draft_throws(0x1420538c0, "unknown_1420538c0", fn_1420538c0 as *const (), Throws::Discard), // sub_1420538c0
    Replacement::draft(0x142054980, "unknown_142054980", tpl_424::<0x142442de0> as *const ()), // sub_142054980
    Replacement::draft(0x1420549a0, "unknown_1420549a0", tpl_369 as *const ()), // sub_1420549a0
    Replacement::draft_throws(0x1420549b0, "unknown_1420549b0", fn_1420549b0 as *const (), Throws::Caught), // sub_1420549b0
    Replacement::draft(0x142054d50, "unknown_142054d50", tpl_369 as *const ()), // sub_142054d50
    Replacement::draft(0x142054de0, "unknown_142054de0", tpl_43598::<0x142054c80, 0x8, 0x108> as *const ()), // sub_142054de0
    Replacement::draft(0x142054e60, "unknown_142054e60", tpl_43598::<0x142054c80, 0xc8, 0x108> as *const ()), // sub_142054e60
    Replacement::draft(0x142054e90, "unknown_142054e90", tpl_361 as *const ()), // sub_142054e90
    Replacement::draft(0x142054f40, "unknown_142054f40", tpl_361 as *const ()), // sub_142054f40
    Replacement::draft(0x142054f50, "unknown_142054f50", tpl_361 as *const ()), // sub_142054f50
    Replacement::draft(0x142054f60, "unknown_142054f60", tpl_361 as *const ()), // sub_142054f60
    Replacement::draft(0x142054f70, "unknown_142054f70", tpl_361 as *const ()), // sub_142054f70
    Replacement::draft(0x142054f80, "unknown_142054f80", tpl_361 as *const ()), // sub_142054f80
    Replacement::draft(0x142054f90, "unknown_142054f90", tpl_361 as *const ()), // sub_142054f90
    Replacement::draft(0x142054fa0, "unknown_142054fa0", tpl_361 as *const ()), // sub_142054fa0
    Replacement::draft(0x142054fb0, "unknown_142054fb0", tpl_361 as *const ()), // sub_142054fb0
    Replacement::draft(0x142054fc0, "unknown_142054fc0", tpl_361 as *const ()), // sub_142054fc0
    Replacement::draft(0x142054fd0, "unknown_142054fd0", tpl_361 as *const ()), // sub_142054fd0
    Replacement::draft(0x142054fe0, "unknown_142054fe0", tpl_361 as *const ()), // sub_142054fe0
    Replacement::draft(0x142054ff0, "unknown_142054ff0", tpl_361 as *const ()), // sub_142054ff0
    Replacement::draft(0x142055000, "unknown_142055000", tpl_361 as *const ()), // sub_142055000
    Replacement::draft(0x142055010, "unknown_142055010", tpl_3047::<0x8> as *const ()), // sub_142055010
    Replacement::draft(0x142055020, "unknown_142055020", fn_142055020 as *const ()), // sub_142055020
    Replacement::draft(0x142055400, "unknown_142055400", tpl_369 as *const ()), // sub_142055400
    Replacement::draft(0x142055430, "unknown_142055430", tpl_895::<0x8> as *const ()), // sub_142055430
    Replacement::draft(0x142055450, "unknown_142055450", tpl_43605::<0x14397fbf8, 0x1411d44d0, 0x142442de0, 0x14397fbb0, 0x14397fbe8, 0x14397fbc0, 0x14397fbb8, 0x14397fbc8, 0x14397fba8, 0x14397fbd8, 0x14397fbd0, 0x14397fc00, 0x14397fbe0, 0x14397fbf0> as *const ()), // sub_142055450
    Replacement::draft(0x142055a50, "unknown_142055a50", tpl_361 as *const ()), // sub_142055a50
    Replacement::draft_throws(0x142055c40, "unknown_142055c40", fn_142055c40 as *const (), Throws::Discard), // sub_142055c40
    Replacement::draft(0x142056030, "unknown_142056030", tpl_369 as *const ()), // sub_142056030
    Replacement::draft(0x142056490, "unknown_142056490", tpl_43614::<0x143a631e8, 0x1401de830> as *const ()), // sub_142056490
    Replacement::draft(0x1420564b0, "unknown_1420564b0", fn_1420564b0 as *const ()), // sub_1420564b0
    Replacement::draft(0x142056820, "d3d_142056820", tpl_43617::<0x1433dc308, 0x142645790, 0x143857a18, 0x143857a10, 0x1426455a8, 0x143a63208, 0x143a63210, 0x143857a1c, 0x14183acc8, 0x142645598> as *const ()), // sub_142056820
    Replacement::draft(0x142056940, "d3d_142056940", fn_142056940 as *const ()), // sub_142056940
    Replacement::draft(0x142056af0, "d3d_142056af0", tpl_43617::<0x1433dc320, 0x142645790, 0x143857a08, 0x143857a00, 0x1426455a8, 0x143a63218, 0x143a63220, 0x143857a0c, 0x14183acc8, 0x142645598> as *const ()), // sub_142056af0
    Replacement::draft(0x142056c10, "d3d_142056c10", tpl_43619::<0x1433dc320, 0x142645790, 0x143857a08, 0x143857a00, 0x1426455a8, 0x143857a0c, 0x143a63220, 0x143a63218, 0x143a63224, 0x1411d5540, 0x142645598> as *const ()), // sub_142056c10
    Replacement::draft(0x142056db0, "d3d_142056db0", tpl_43617::<0x1433dc2f0, 0x142645790, 0x143857a28, 0x143857a20, 0x1426455a8, 0x143a631f8, 0x143a63200, 0x143857a2c, 0x14183acc8, 0x142645598> as *const ()), // sub_142056db0
    Replacement::draft(0x142056ed0, "d3d_142056ed0", tpl_43619::<0x1433dc2f0, 0x142645790, 0x143857a28, 0x143857a20, 0x1426455a8, 0x143857a2c, 0x143a63200, 0x143a631f8, 0x143a63204, 0x1411d53f0, 0x142645598> as *const ()), // sub_142056ed0
    Replacement::draft(0x142056f90, "d3d_142056f90", tpl_43621::<0x143857a40, 0x142645790, 0x143857a48, 0x1426455a8, 0x143857a4c, 0x143a63238, 0x143a63240, 0x8, 0x1401de830, 0x142645598> as *const ()), // sub_142056f90
    Replacement::draft(0x142057060, "d3d_142057060", tpl_43622::<0x8, 0x142645790, 0x143857a48, 0x143857a40, 0x1426455a8, 0x143857a4c, 0x143a63240, 0x143a63238, 0x143a63244, 0x1411d59b0, 0x142645598> as *const ()), // sub_142057060
    Replacement::draft(0x142057270, "d3d_142057270", tpl_43621::<0x143857a30, 0x142645790, 0x143857a38, 0x1426455a8, 0x143857a3c, 0x143a63228, 0x143a63230, 0x8, 0x1401de830, 0x142645598> as *const ()), // sub_142057270
    Replacement::draft(0x142057340, "d3d_142057340", tpl_43622::<0x8, 0x142645790, 0x143857a38, 0x143857a30, 0x1426455a8, 0x143857a3c, 0x143a63230, 0x143a63228, 0x143a63234, 0x1411d5860, 0x142645598> as *const ()), // sub_142057340
    Replacement::draft(0x142057550, "unknown_142057550", tpl_28926::<0x143a35918> as *const ()), // sub_142057550
    Replacement::draft(0x142057560, "unknown_142057560", tpl_366::<0x143a35918> as *const ()), // sub_142057560
    Replacement::draft(0x142057570, "unknown_142057570", tpl_43623::<0x143857a30, 0x142645790, 0x143857a38, 0x1426455a8, 0x143857a3c, 0x143a63230, 0x143a63228, 0x14183acc8, 0x142645598> as *const ()), // sub_142057570
    Replacement::draft(0x142057690, "unknown_142057690", tpl_43623::<0x143857a40, 0x142645790, 0x143857a48, 0x1426455a8, 0x143857a4c, 0x143a63240, 0x143a63238, 0x14183acc8, 0x142645598> as *const ()), // sub_142057690
    Replacement::draft_throws(0x1420577b0, "unknown_1420577b0", fn_1420577b0 as *const (), Throws::Caught), // sub_1420577b0
    Replacement::draft(0x142058640, "unknown_142058640", fn_142058640 as *const ()), // sub_142058640
    Replacement::draft_throws(0x142058950, "unknown_142058950", fn_142058950 as *const (), Throws::Caught), // sub_142058950
    Replacement::draft(0x142059380, "unknown_142059380", tpl_32881::<0x110, 0x142645c78> as *const ()), // sub_142059380
    Replacement::draft(0x1420593b0, "unknown_1420593b0", tpl_32881::<0x110, 0x142645c90> as *const ()), // sub_1420593b0
    Replacement::draft_throws(0x1420594c0, "unknown_1420594c0", fn_1420594c0 as *const (), Throws::Caught), // sub_1420594c0
    Replacement::draft_throws(0x1420597a0, "unknown_1420597a0", fn_1420597a0 as *const (), Throws::Caught), // sub_1420597a0
    Replacement::draft_throws(0x142059ea0, "unknown_142059ea0", fn_142059ea0 as *const (), Throws::Discard), // sub_142059ea0
    Replacement::draft_throws(0x14205a6a0, "unknown_14205a6a0", fn_14205a6a0 as *const (), Throws::Caught), // sub_14205a6a0
    Replacement::draft_throws(0x14205ab00, "unknown_14205ab00", fn_14205ab00 as *const (), Throws::Discard), // sub_14205ab00
    Replacement::draft(0x14205ad90, "unknown_14205ad90", tpl_365::<0x14205ae20> as *const ()), // sub_14205ad90
    Replacement::draft(0x14205ae20, "unknown_14205ae20", fn_14205ae20 as *const ()), // sub_14205ae20
    Replacement::draft(0x14205b090, "unknown_14205b090", tpl_365::<0x14205ae20> as *const ()), // sub_14205b090
    Replacement::draft(0x14205b0a0, "unknown_14205b0a0", fn_14205b0a0 as *const ()), // sub_14205b0a0
    Replacement::draft_throws(0x14205c2d0, "unknown_14205c2d0", tpl_43654::<0x143c48fc0, 0x14205d410, 0x142645868, 0x143a35d08, 0x143857d20, 0x143857d10, 0x143857d50, 0x143857d70, 0x143857d48, 0x143857d28, 0x143857d18, 0x1426464a8, 0x143a35b28, 0x1426464a0> as *const (), Throws::Discard), // sub_14205c2d0
    Replacement::draft(0x14205ca90, "unknown_14205ca90", tpl_369 as *const ()), // sub_14205ca90
    Replacement::draft(0x14205cdc0, "unknown_14205cdc0", tpl_43661 as *const ()), // sub_14205cdc0
    Replacement::draft(0x14205cde0, "d3d_14205cde0", tpl_23827::<0x14340ed48, 0x8, 0x1433dc7d8, 0x142498f00> as *const ()), // sub_14205cde0
    Replacement::draft(0x14205ce20, "d3d_14205ce20", fn_14205ce20 as *const ()), // sub_14205ce20
    Replacement::draft(0x14205d3f0, "d3d_14205d3f0", tpl_22030::<0x14340ed48, 0x8, 0x1433dc7d8> as *const ()), // sub_14205d3f0
    Replacement::draft(0x14205d5d0, "d3d_14205d5d0", tpl_361 as *const ()), // sub_14205d5d0
    Replacement::draft(0x14205d5e0, "d3d_14205d5e0", tpl_361 as *const ()), // sub_14205d5e0
    Replacement::draft(0x14205d5f0, "d3d_14205d5f0", tpl_361 as *const ()), // sub_14205d5f0
    Replacement::draft(0x14205d600, "d3d_14205d600", tpl_43666::<0x143a35d58, 0x142442de0, 0x143a35e28, 0x1401de830, 0x14397fc10, 0x142645790, 0x1426455a8, 0x14209a1e0, 0x142645598, 0x1403496f0> as *const ()), // sub_14205d600
    Replacement::draft_throws(0x14205d790, "d3d_14205d790", tpl_43667::<0x1411d7da0, 0x143a35d58, 0x140fd31f0, 0x143a35e28, 0x1411d7e20, 0x14397fc10, 0x1411d82e0> as *const (), Throws::Discard), // sub_14205d790
    Replacement::draft(0x14205dc40, "d3d_14205dc40", tpl_361 as *const ()), // sub_14205dc40
    Replacement::draft(0x14205dc50, "d3d_14205dc50", tpl_361 as *const ()), // sub_14205dc50
    Replacement::draft(0x14205dc60, "d3d_14205dc60", tpl_361 as *const ()), // sub_14205dc60
    Replacement::draft(0x14205dc70, "d3d_14205dc70", tpl_43666::<0x143a35e78, 0x142442de0, 0x143a35ec8, 0x1401de830, 0x14397fc18, 0x142645790, 0x1426455a8, 0x14209a1e0, 0x142645598, 0x1403496f0> as *const ()), // sub_14205dc70
    Replacement::draft_throws(0x14205de00, "d3d_14205de00", tpl_43667::<0x1411d7da0, 0x143a35e78, 0x140fd31f0, 0x143a35ec8, 0x1411d7e20, 0x14397fc18, 0x1411d8b90> as *const (), Throws::Discard), // sub_14205de00
    Replacement::draft(0x14205e280, "d3d_14205e280", tpl_361 as *const ()), // sub_14205e280
    Replacement::draft(0x14205e290, "d3d_14205e290", tpl_361 as *const ()), // sub_14205e290
    Replacement::draft(0x14205e6b0, "d3d_14205e6b0", tpl_361 as *const ()), // sub_14205e6b0
    Replacement::draft(0x14205e6c0, "d3d_14205e6c0", tpl_361 as *const ()), // sub_14205e6c0
    Replacement::draft(0x14205e770, "d3d_14205e770", fn_14205e770 as *const ()), // sub_14205e770
    Replacement::draft(0x14205ea50, "d3d_14205ea50", fn_14205ea50 as *const ()), // sub_14205ea50
    Replacement::draft_throws(0x14205ec10, "d3d_14205ec10", tpl_43686::<0x1411d7e20, 0x14397fc28, 0x1411d9ab0> as *const (), Throws::Discard), // sub_14205ec10
    Replacement::draft(0x14205ec30, "d3d_14205ec30", tpl_361 as *const ()), // sub_14205ec30
    Replacement::draft(0x14205ec40, "d3d_14205ec40", tpl_361 as *const ()), // sub_14205ec40
    Replacement::draft(0x14205f830, "d3d_14205f830", tpl_361 as *const ()), // sub_14205f830
    Replacement::draft(0x14205f840, "d3d_14205f840", tpl_361 as *const ()), // sub_14205f840
    Replacement::draft(0x14205f850, "d3d_14205f850", tpl_361 as *const ()), // sub_14205f850
    Replacement::draft(0x14205f860, "d3d_14205f860", tpl_361 as *const ()), // sub_14205f860
    Replacement::draft_throws(0x14205f870, "d3d_14205f870", tpl_43695::<0x1426455a8, 0x1401deba0, 0x14020bcb0, 0x142645598, 0x143857f50, 0x142076db0> as *const (), Throws::Discard), // sub_14205f870
    Replacement::draft_throws(0x14205f9a0, "d3d_14205f9a0", tpl_43695::<0x1426455a8, 0x1401deba0, 0x14020bcb0, 0x142645598, 0x143857f50, 0x142076db0> as *const (), Throws::Discard), // sub_14205f9a0
    Replacement::draft_throws(0x14205fad0, "d3d_14205fad0", fn_14205fad0 as *const (), Throws::Discard), // sub_14205fad0
    Replacement::draft_throws(0x14205fdc0, "d3d_14205fdc0", fn_14205fdc0 as *const (), Throws::Discard), // sub_14205fdc0
    Replacement::draft(0x1420604b0, "d3d_1420604b0", tpl_361 as *const ()), // sub_1420604b0
    Replacement::draft(0x1420604c0, "d3d_1420604c0", tpl_361 as *const ()), // sub_1420604c0
    Replacement::draft(0x142060c30, "unknown_142060c30", tpl_881 as *const ()), // sub_142060c30
    Replacement::draft(0x142060d40, "unknown_142060d40", tpl_895::<0x8> as *const ()), // sub_142060d40
    Replacement::draft(0x142060e50, "unknown_142060e50", tpl_422::<0x8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_142060e50
    Replacement::draft(0x142060fe0, "unknown_142060fe0", tpl_5649::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_142060fe0
    Replacement::draft(0x1420610a0, "unknown_1420610a0", tpl_43721::<0x8, 0x10> as *const ()), // sub_1420610a0
    Replacement::draft(0x142061210, "unknown_142061210", tpl_5649::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_142061210
    Replacement::draft_throws(0x142061420, "unknown_142061420", fn_142061420 as *const (), Throws::Discard), // sub_142061420
    Replacement::draft(0x142062110, "unknown_142062110", fn_142062110 as *const ()), // sub_142062110
    Replacement::draft(0x142062350, "unknown_142062350", fn_142062350 as *const ()), // sub_142062350
    Replacement::draft(0x142062940, "unknown_142062940", fn_142062940 as *const ()), // sub_142062940
    Replacement::draft(0x142062c10, "unknown_142062c10", tpl_43755::<0x8, 0x10> as *const ()), // sub_142062c10
    Replacement::draft(0x142062c30, "unknown_142062c30", tpl_22123::<0x1411db190, 0xfffffffffffffffa, 0x141834ee8, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_142062c30
    Replacement::draft_throws(0x142062c90, "unknown_142062c90", fn_142062c90 as *const (), Throws::Discard), // sub_142062c90
    Replacement::draft(0x1420632a0, "unknown_1420632a0", tpl_369 as *const ()), // sub_1420632a0
    Replacement::draft(0x142063350, "unknown_142063350", tpl_369 as *const ()), // sub_142063350
    Replacement::draft(0x1420633d0, "unknown_1420633d0", tpl_361 as *const ()), // sub_1420633d0
    Replacement::draft(0x1420633e0, "unknown_1420633e0", tpl_361 as *const ()), // sub_1420633e0
    Replacement::draft(0x1420633f0, "unknown_1420633f0", tpl_361 as *const ()), // sub_1420633f0
    Replacement::draft(0x142063400, "unknown_142063400", tpl_361 as *const ()), // sub_142063400
    Replacement::draft(0x142063440, "d3d_142063440", fn_142063440 as *const ()), // sub_142063440
    Replacement::draft_throws(0x142063c40, "unknown_142063c40", fn_142063c40 as *const (), Throws::Discard), // sub_142063c40
    Replacement::draft(0x142064110, "unknown_142064110", tpl_369 as *const ()), // sub_142064110
    Replacement::draft(0x14206b370, "unknown_14206b370", fn_14206b370 as *const ()), // sub_14206b370
    Replacement::draft_throws(0x14206b640, "unknown_14206b640", fn_14206b640 as *const (), Throws::Discard), // sub_14206b640
    Replacement::draft_throws(0x14206bea0, "unknown_14206bea0", fn_14206bea0 as *const (), Throws::Caught), // sub_14206bea0
    Replacement::draft_throws(0x14206c8b0, "unknown_14206c8b0", fn_14206c8b0 as *const (), Throws::Discard), // sub_14206c8b0
    Replacement::draft_throws(0x14206cf90, "unknown_14206cf90", fn_14206cf90 as *const (), Throws::Discard), // sub_14206cf90
    Replacement::draft(0x14206d520, "unknown_14206d520", fn_14206d520 as *const ()), // sub_14206d520
    Replacement::draft_throws(0x14206da30, "unknown_14206da30", fn_14206da30 as *const (), Throws::Discard), // sub_14206da30
    Replacement::draft_throws(0x14206df80, "unknown_14206df80", fn_14206df80 as *const (), Throws::Discard), // sub_14206df80
    Replacement::draft_throws(0x14206e740, "unknown_14206e740", tpl_42379::<0x1420734e0> as *const (), Throws::Discard), // sub_14206e740
    Replacement::draft_throws(0x14206e7d0, "unknown_14206e7d0", tpl_42379::<0x1420734e0> as *const (), Throws::Discard), // sub_14206e7d0
    Replacement::draft_throws(0x14206e7f0, "unknown_14206e7f0", fn_14206e7f0 as *const (), Throws::Discard), // sub_14206e7f0
    Replacement::draft_throws(0x142070370, "global_142070370", fn_142070370 as *const (), Throws::Discard), // gpu_feature_validator
    Replacement::draft(0x142071320, "unknown_142071320", tpl_43892::<0x14397fc58, 0x28, 0x142071360> as *const ()), // sub_142071320
    Replacement::draft(0x142071340, "unknown_142071340", tpl_43892::<0x14397fc58, 0x20, 0x142071360> as *const ()), // sub_142071340
    Replacement::draft_throws(0x142071640, "unknown_142071640", fn_142071640 as *const (), Throws::Discard), // sub_142071640
    Replacement::draft_throws(0x1420719f0, "unknown_1420719f0", fn_1420719f0 as *const (), Throws::Discard), // sub_1420719f0
    Replacement::draft_throws(0x142071da0, "unknown_142071da0", fn_142071da0 as *const (), Throws::Discard), // sub_142071da0
    Replacement::draft(0x142072070, "unknown_142072070", fn_142072070 as *const ()), // sub_142072070
    Replacement::draft(0x142072550, "unknown_142072550", fn_142072550 as *const ()), // sub_142072550
    Replacement::draft(0x142072730, "unknown_142072730", tpl_43901::<0x10> as *const ()), // sub_142072730
    Replacement::draft_throws(0x142072760, "unknown_142072760", fn_142072760 as *const (), Throws::Discard), // sub_142072760
    Replacement::draft(0x142072a00, "unknown_142072a00", tpl_33406::<0x143a28c3d, 0x140001a60> as *const ()), // sub_142072a00
    Replacement::draft(0x142072c30, "unknown_142072c30", tpl_43910::<0x14397fc58, 0x9d> as *const ()), // sub_142072c30
    Replacement::draft(0x142072c50, "unknown_142072c50", tpl_43910::<0x14397fc58, 0xae> as *const ()), // sub_142072c50
    Replacement::draft(0x142072c70, "unknown_142072c70", tpl_43910::<0x14397fc58, 0xad> as *const ()), // sub_142072c70
    Replacement::draft(0x142072c90, "unknown_142072c90", tpl_43910::<0x14397fc58, 0xaf> as *const ()), // sub_142072c90
    Replacement::draft(0x142072cb0, "unknown_142072cb0", tpl_43910::<0x14397fc58, 0x9e> as *const ()), // sub_142072cb0
    Replacement::draft(0x142072ec0, "unknown_142072ec0", fn_142072ec0 as *const ()), // sub_142072ec0
    Replacement::draft(0x142073170, "unknown_142073170", fn_142073170 as *const ()), // sub_142073170
    Replacement::draft(0x1420736b0, "unknown_1420736b0", tpl_34265::<0x14397fc58, 0x88> as *const ()), // sub_1420736b0
    Replacement::draft(0x1420736f0, "unknown_1420736f0", tpl_43910::<0x14397fc58, 0xac> as *const ()), // sub_1420736f0
    Replacement::draft(0x142073cd0, "unknown_142073cd0", fn_142073cd0 as *const ()), // sub_142073cd0
    Replacement::draft(0x142073fe0, "unknown_142073fe0", fn_142073fe0 as *const ()), // sub_142073fe0
    Replacement::draft(0x142074570, "unknown_142074570", fn_142074570 as *const ()), // sub_142074570
    Replacement::draft_throws(0x142074d70, "unknown_142074d70", fn_142074d70 as *const (), Throws::Discard), // sub_142074d70
    Replacement::draft(0x142076410, "unknown_142076410", fn_142076410 as *const ()), // sub_142076410
    Replacement::draft(0x142076740, "unknown_142076740", fn_142076740 as *const ()), // sub_142076740
    Replacement::draft_throws(0x142076be0, "unknown_142076be0", fn_142076be0 as *const (), Throws::Discard), // sub_142076be0
    Replacement::draft_throws(0x142076e50, "unknown_142076e50", tpl_43959::<0x10, 0x78, 0x7785fffb, 0x14205d410> as *const (), Throws::Discard), // sub_142076e50
    Replacement::draft_throws(0x142076e90, "unknown_142076e90", tpl_43959::<0x10, 0x70, 0x7785fffb, 0x14205d410> as *const (), Throws::Discard), // sub_142076e90
    Replacement::draft(0x142077470, "unknown_142077470", fn_142077470 as *const ()), // sub_142077470
    Replacement::draft(0x142077880, "unknown_142077880", fn_142077880 as *const ()), // sub_142077880
    Replacement::draft_throws(0x1420782a0, "unknown_1420782a0", fn_1420782a0 as *const (), Throws::Discard), // sub_1420782a0
    Replacement::draft(0x142078b80, "unknown_142078b80", tpl_913::<0x340> as *const ()), // sub_142078b80
    Replacement::draft(0x142078c60, "unknown_142078c60", tpl_22183::<0x1411d2b20, 0xfffffffffffffffb, 0x141834ee8, 0xfffffffffffffff5> as *const ()), // sub_142078c60
    Replacement::draft_throws(0x142078cb0, "unknown_142078cb0", fn_142078cb0 as *const (), Throws::Discard), // sub_142078cb0
    Replacement::draft(0x142079ee0, "unknown_142079ee0", tpl_43987::<0x8, 0x10, 0x18, 0x1c, 0x20, 0x28> as *const ()), // sub_142079ee0
    Replacement::draft(0x142079f30, "unknown_142079f30", tpl_43987::<0x8, 0x10, 0x18, 0x1c, 0x20, 0x28> as *const ()), // sub_142079f30
    Replacement::draft(0x14207a140, "unknown_14207a140", tpl_496 as *const ()), // sub_14207a140
    Replacement::draft(0x14207a150, "unknown_14207a150", tpl_361 as *const ()), // sub_14207a150
    Replacement::draft(0x14207a160, "unknown_14207a160", tpl_27483 as *const ()), // sub_14207a160
    Replacement::draft(0x14207a170, "unknown_14207a170", tpl_43990::<0x66> as *const ()), // sub_14207a170
    Replacement::draft(0x14207a1b0, "unknown_14207a1b0", tpl_43992::<0x66, 0x143a2b11c, 0x67> as *const ()), // sub_14207a1b0
    Replacement::draft_throws(0x14207a1e0, "unknown_14207a1e0", fn_14207a1e0 as *const (), Throws::Discard), // sub_14207a1e0
    Replacement::draft(0x14207a530, "unknown_14207a530", tpl_43994::<0x41> as *const ()), // sub_14207a530
    Replacement::draft_throws(0x14207a680, "unknown_14207a680", fn_14207a680 as *const (), Throws::Caught), // sub_14207a680
    Replacement::draft(0x14207a9a0, "unknown_14207a9a0", tpl_369 as *const ()), // sub_14207a9a0
    Replacement::draft(0x14207a9b0, "unknown_14207a9b0", tpl_28365::<0x48> as *const ()), // sub_14207a9b0
    Replacement::draft_throws(0x14207a9c0, "unknown_14207a9c0", fn_14207a9c0 as *const (), Throws::Discard), // sub_14207a9c0
    Replacement::draft(0x14207ad00, "unknown_14207ad00", tpl_34998::<0x8, 0x10> as *const ()), // sub_14207ad00
    Replacement::draft(0x14207adc0, "unknown_14207adc0", tpl_36390::<0x18, 0x8, 0x10> as *const ()), // sub_14207adc0
    Replacement::draft_throws(0x14207aec0, "unknown_14207aec0", fn_14207aec0 as *const (), Throws::Caught), // sub_14207aec0
    Replacement::draft(0x14207b430, "unknown_14207b430", fn_14207b430 as *const ()), // sub_14207b430
    Replacement::draft_throws(0x14207b830, "unknown_14207b830", fn_14207b830 as *const (), Throws::Discard), // sub_14207b830
    Replacement::draft_throws(0x14207ba10, "unknown_14207ba10", fn_14207ba10 as *const (), Throws::Discard), // sub_14207ba10
    Replacement::draft_throws(0x14207c1c0, "unknown_14207c1c0", fn_14207c1c0 as *const (), Throws::Discard), // sub_14207c1c0
    Replacement::draft_throws(0x14207c4b0, "unknown_14207c4b0", fn_14207c4b0 as *const (), Throws::Discard), // sub_14207c4b0
    Replacement::draft(0x14207c6d0, "unknown_14207c6d0", tpl_6559::<0x1401de830> as *const ()), // sub_14207c6d0
    Replacement::draft(0x14207c6f0, "unknown_14207c6f0", tpl_6559::<0x1401de830> as *const ()), // sub_14207c6f0
    Replacement::draft_throws(0x14207c7a0, "unknown_14207c7a0", fn_14207c7a0 as *const (), Throws::Discard), // sub_14207c7a0
    Replacement::draft(0x14207cc20, "unknown_14207cc20", tpl_43661 as *const ()), // sub_14207cc20
    Replacement::draft(0x14207cc30, "unknown_14207cc30", tpl_369 as *const ()), // sub_14207cc30
    Replacement::draft(0x14207cc80, "unknown_14207cc80", tpl_43992::<0x66, 0x143a2b11c, 0x67> as *const ()), // sub_14207cc80
    Replacement::draft_throws(0x14207ccb0, "unknown_14207ccb0", fn_14207ccb0 as *const (), Throws::Discard), // sub_14207ccb0
    Replacement::draft_throws(0x14207ced0, "unknown_14207ced0", fn_14207ced0 as *const (), Throws::Discard), // sub_14207ced0
    Replacement::draft_throws(0x14207d110, "unknown_14207d110", tpl_838::<0x14207b710> as *const (), Throws::Discard), // sub_14207d110
    Replacement::draft(0x14207d120, "unknown_14207d120", tpl_44028::<0x8, 0x10, 0x18, 0x20> as *const ()), // sub_14207d120
    Replacement::draft(0x14207d170, "unknown_14207d170", tpl_44029::<0x8, 0x9, 0x10, 0x18> as *const ()), // sub_14207d170
    Replacement::draft(0x14207d1f0, "unknown_14207d1f0", tpl_44029::<0x8, 0x9, 0x10, 0x18> as *const ()), // sub_14207d1f0
    Replacement::draft(0x14207d230, "unknown_14207d230", tpl_44028::<0x8, 0x10, 0x18, 0x20> as *const ()), // sub_14207d230
    Replacement::draft(0x14207d280, "unknown_14207d280", tpl_44031::<0x141834d30> as *const ()), // sub_14207d280
    Replacement::draft(0x14207d2a0, "unknown_14207d2a0", tpl_44032::<0x141834d30> as *const ()), // sub_14207d2a0
    Replacement::draft(0x14207d2c0, "unknown_14207d2c0", tpl_44032::<0x141834d30> as *const ()), // sub_14207d2c0
    Replacement::draft(0x14207d2e0, "unknown_14207d2e0", tpl_44032::<0x141834d30> as *const ()), // sub_14207d2e0
    Replacement::draft(0x14207d300, "unknown_14207d300", tpl_44032::<0x141834d30> as *const ()), // sub_14207d300
    Replacement::draft(0x14207d320, "unknown_14207d320", tpl_44031::<0x141834d30> as *const ()), // sub_14207d320
    Replacement::draft(0x14207d3b0, "std_14207d3b0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::reserveAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::StreamedTex
    Replacement::draft(0x14207d400, "std_14207d400", tpl_1::<0x143892650> as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::reserveAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::StreamedTex
    Replacement::draft(0x14207d410, "std_14207d410", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::reserveAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::StreamedTex
    Replacement::draft(0x14207d420, "std_14207d420", tpl_20904::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::reserveAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::StreamedTex
    Replacement::draft(0x14207d4a0, "std_14207d4a0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::tryCommitAsync::<lambda_1>,void,?bref6::LoadedData&&>::std::_Func_impl_no
    Replacement::draft(0x14207d500, "std_14207d500", tpl_1::<0x1438924f0> as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::tryCommitAsync::<lambda_1>,void,?bref6::LoadedData&&>::std::_Func_impl_no
    Replacement::draft(0x14207d510, "std_14207d510", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::tryCommitAsync::<lambda_1>,void,?bref6::LoadedData&&>::std::_Func_impl_no
    Replacement::draft(0x14207d520, "std_14207d520", tpl_44037::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<d3d::StreamedTexture::tryCommitAsync::<lambda_1>,void,?bref6::LoadedData&&>::std::_Func_impl_no
    Replacement::draft(0x14207d5a0, "std_14207d5a0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<d3d::detail::StreamedTextureImpl::commitAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::
    Replacement::draft(0x14207d5e0, "std_14207d5e0", tpl_1::<0x1438925b0> as *const ()), // std::_Func_impl_no_alloc<d3d::detail::StreamedTextureImpl::commitAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::
    Replacement::draft(0x14207d5f0, "std_14207d5f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<d3d::detail::StreamedTextureImpl::commitAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::
    Replacement::draft(0x14207d600, "std_14207d600", tpl_44037::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<d3d::detail::StreamedTextureImpl::commitAsync::<lambda_1>,void>::std::_Func_impl_no_alloc<d3d::
    Replacement::draft_throws(0x14207da50, "unknown_14207da50", fn_14207da50 as *const (), Throws::Caught), // sub_14207da50
    Replacement::draft_throws(0x14207def0, "unknown_14207def0", fn_14207def0 as *const (), Throws::Discard), // sub_14207def0
    Replacement::draft_throws(0x14207e690, "unknown_14207e690", fn_14207e690 as *const (), Throws::Caught), // sub_14207e690
    Replacement::draft_throws(0x14207e9d0, "unknown_14207e9d0", fn_14207e9d0 as *const (), Throws::Discard), // sub_14207e9d0
    Replacement::draft_throws(0x14207ec20, "unknown_14207ec20", fn_14207ec20 as *const (), Throws::Caught), // sub_14207ec20
    Replacement::draft_throws(0x14207f3a0, "unknown_14207f3a0", fn_14207f3a0 as *const (), Throws::Discard), // sub_14207f3a0
    Replacement::draft_throws(0x14207f700, "unknown_14207f700", tpl_838::<0x14207e500> as *const (), Throws::Discard), // sub_14207f700
    Replacement::draft_throws(0x14207f790, "unknown_14207f790", fn_14207f790 as *const (), Throws::Caught), // sub_14207f790
    Replacement::draft_throws(0x14207fa10, "unknown_14207fa10", fn_14207fa10 as *const (), Throws::Caught), // sub_14207fa10
    Replacement::draft_throws(0x14207fc90, "unknown_14207fc90", fn_14207fc90 as *const (), Throws::Caught), // sub_14207fc90
    Replacement::draft_throws(0x14207ffe0, "unknown_14207ffe0", tpl_884::<0x14207f3a0> as *const (), Throws::Discard), // sub_14207ffe0
    Replacement::draft_throws(0x142080000, "unknown_142080000", tpl_838::<0x14207f310> as *const (), Throws::Discard), // sub_142080000
    Replacement::draft(0x142080010, "unknown_142080010", tpl_26861::<0x14207f660> as *const ()), // sub_142080010
    Replacement::draft(0x142080080, "unknown_142080080", tpl_44064::<0x8, 0x1c> as *const ()), // sub_142080080
    Replacement::draft(0x1420800d0, "unknown_1420800d0", tpl_44064::<0x8, 0x1c> as *const ()), // sub_1420800d0
    Replacement::draft(0x142080670, "unknown_142080670", tpl_44065::<0x8, 0x18> as *const ()), // sub_142080670
    Replacement::draft(0x1420806e0, "unknown_1420806e0", tpl_35253 as *const ()), // sub_1420806e0
    Replacement::draft(0x1420806f0, "unknown_1420806f0", tpl_35253 as *const ()), // sub_1420806f0
    Replacement::draft(0x142080700, "unknown_142080700", tpl_44066::<0x38, 0x1401fa4d0> as *const ()), // sub_142080700
    Replacement::draft(0x142080730, "unknown_142080730", tpl_369 as *const ()), // sub_142080730
    Replacement::draft(0x142080740, "unknown_142080740", tpl_361 as *const ()), // sub_142080740
    Replacement::draft(0x142080750, "unknown_142080750", tpl_369 as *const ()), // sub_142080750
    Replacement::draft(0x142080760, "unknown_142080760", tpl_369 as *const ()), // sub_142080760
    Replacement::draft(0x142080770, "unknown_142080770", tpl_369 as *const ()), // sub_142080770
    Replacement::draft(0x142080980, "unknown_142080980", tpl_40071::<0x10, 0x14183acc2> as *const ()), // sub_142080980
    Replacement::draft(0x142080b50, "unknown_142080b50", fn_142080b50 as *const ()), // sub_142080b50
    Replacement::draft(0x142080e90, "unknown_142080e90", fn_142080e90 as *const ()), // sub_142080e90
    Replacement::draft(0x142081160, "unknown_142081160", tpl_40074::<0x18, 0x8> as *const ()), // sub_142081160
    Replacement::draft(0x142081210, "unknown_142081210", tpl_381::<0x142080e90, 0x142080b50> as *const ()), // sub_142081210
    Replacement::draft(0x142081410, "unknown_142081410", tpl_43721::<0x18, 0x10> as *const ()), // sub_142081410
    Replacement::draft(0x142081440, "unknown_142081440", tpl_43721::<0x18, 0x10> as *const ()), // sub_142081440
    Replacement::draft(0x1420814e0, "unknown_1420814e0", tpl_44072::<0x18, 0x10> as *const ()), // sub_1420814e0
    Replacement::draft(0x142081510, "unknown_142081510", tpl_44072::<0x18, 0x10> as *const ()), // sub_142081510
    Replacement::draft(0x142081540, "unknown_142081540", tpl_44065::<0x8, 0x18> as *const ()), // sub_142081540
    Replacement::draft(0x142081770, "unknown_142081770", tpl_22131::<0x10> as *const ()), // sub_142081770
    Replacement::draft(0x142081840, "unknown_142081840", tpl_44078::<0x8> as *const ()), // sub_142081840
    Replacement::draft(0x142081860, "unknown_142081860", tpl_44079::<0x10> as *const ()), // sub_142081860
    Replacement::draft(0x142081890, "unknown_142081890", tpl_44079::<0x10> as *const ()), // sub_142081890
    Replacement::draft_throws(0x142081e80, "unknown_142081e80", tpl_44086::<0x14241c0a0> as *const (), Throws::Discard), // sub_142081e80
    Replacement::draft(0x142082100, "unknown_142082100", tpl_44093 as *const ()), // sub_142082100
    Replacement::draft(0x142082180, "unknown_142082180", tpl_44095 as *const ()), // sub_142082180
    Replacement::draft(0x142082510, "unknown_142082510", tpl_473 as *const ()), // sub_142082510
    Replacement::draft(0x142082520, "unknown_142082520", tpl_37733 as *const ()), // sub_142082520
    Replacement::draft(0x142082540, "unknown_142082540", tpl_496 as *const ()), // sub_142082540
    Replacement::draft(0x142082550, "unknown_142082550", tpl_361 as *const ()), // sub_142082550
    Replacement::draft(0x142082560, "unknown_142082560", tpl_548::<0x8> as *const ()), // sub_142082560
    Replacement::draft(0x142082570, "unknown_142082570", tpl_548::<0xc> as *const ()), // sub_142082570
    Replacement::draft(0x142082580, "unknown_142082580", tpl_473 as *const ()), // sub_142082580
    Replacement::draft_throws(0x142082590, "unknown_142082590", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_142082590
    Replacement::draft(0x1420825b0, "unknown_1420825b0", tpl_496 as *const ()), // sub_1420825b0
    Replacement::draft(0x1420825c0, "unknown_1420825c0", tpl_361 as *const ()), // sub_1420825c0
    Replacement::draft(0x1420825d0, "unknown_1420825d0", tpl_548::<0xc> as *const ()), // sub_1420825d0
    Replacement::draft(0x1420825e0, "unknown_1420825e0", tpl_473 as *const ()), // sub_1420825e0
    Replacement::draft(0x1420825f0, "unknown_1420825f0", tpl_44097::<0xc> as *const ()), // sub_1420825f0
    Replacement::draft_throws(0x142082610, "unknown_142082610", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_142082610
    Replacement::draft(0x142082630, "unknown_142082630", tpl_496 as *const ()), // sub_142082630
    Replacement::draft(0x142082640, "unknown_142082640", tpl_361 as *const ()), // sub_142082640
    Replacement::draft(0x142082650, "unknown_142082650", tpl_548::<0xc> as *const ()), // sub_142082650
    Replacement::draft(0x142082660, "unknown_142082660", tpl_473 as *const ()), // sub_142082660
    Replacement::draft(0x142082670, "unknown_142082670", tpl_44098::<0xc> as *const ()), // sub_142082670
    Replacement::draft_throws(0x142082690, "unknown_142082690", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_142082690
    Replacement::draft(0x1420826b0, "unknown_1420826b0", tpl_496 as *const ()), // sub_1420826b0
    Replacement::draft(0x1420826c0, "unknown_1420826c0", tpl_361 as *const ()), // sub_1420826c0
    Replacement::draft(0x1420826d0, "unknown_1420826d0", tpl_548::<0xc> as *const ()), // sub_1420826d0
    Replacement::draft(0x1420826e0, "unknown_1420826e0", tpl_473 as *const ()), // sub_1420826e0
    Replacement::draft(0x1420826f0, "unknown_1420826f0", tpl_44099::<0xc> as *const ()), // sub_1420826f0
    Replacement::draft_throws(0x142082710, "unknown_142082710", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_142082710
    Replacement::draft(0x142082730, "unknown_142082730", tpl_44100 as *const ()), // sub_142082730
    Replacement::draft(0x142082740, "unknown_142082740", tpl_37731 as *const ()), // sub_142082740
    Replacement::draft(0x142082760, "unknown_142082760", tpl_44102 as *const ()), // sub_142082760
    Replacement::draft(0x142082780, "std_142082780", tpl_550::<0x1433dce60, 0x8> as *const ()), // std::_Func_impl_no_alloc<d3d::NativeStreamedTextureHeap::~NativeStreamedTextureHeap::<lambda_1>,void>::std::_Func_impl_n
    Replacement::draft(0x1420827a0, "std_1420827a0", tpl_550::<0x1433dce60, 0x8> as *const ()), // std::_Func_impl_no_alloc<d3d::NativeStreamedTextureHeap::~NativeStreamedTextureHeap::<lambda_1>,void>::std::_Func_impl_n
    Replacement::draft(0x142082800, "std_142082800", tpl_1::<0x143892700> as *const ()), // std::_Func_impl_no_alloc<d3d::NativeStreamedTextureHeap::~NativeStreamedTextureHeap::<lambda_1>,void>::std::_Func_impl_n
    Replacement::draft(0x142082810, "std_142082810", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<d3d::NativeStreamedTextureHeap::~NativeStreamedTextureHeap::<lambda_1>,void>::std::_Func_impl_n
    Replacement::draft(0x142082820, "std_142082820", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<d3d::NativeStreamedTextureHeap::~NativeStreamedTextureHeap::<lambda_1>,void>::std::_Func_impl_n
    Replacement::draft(0x142082870, "unknown_142082870", tpl_31205::<0x142084000> as *const ()), // sub_142082870
    Replacement::draft(0x142082890, "unknown_142082890", tpl_895::<0x8> as *const ()), // sub_142082890
    Replacement::draft(0x1420828b0, "unknown_1420828b0", tpl_895::<0x8> as *const ()), // sub_1420828b0
    Replacement::draft(0x1420828d0, "unknown_1420828d0", tpl_895::<0x8> as *const ()), // sub_1420828d0
    Replacement::draft(0x1420828f0, "unknown_1420828f0", tpl_895::<0x8> as *const ()), // sub_1420828f0
    Replacement::draft(0x142082910, "unknown_142082910", tpl_895::<0x8> as *const ()), // sub_142082910
    Replacement::draft(0x142082930, "unknown_142082930", tpl_895::<0x8> as *const ()), // sub_142082930
    Replacement::draft(0x142082950, "unknown_142082950", tpl_895::<0x8> as *const ()), // sub_142082950
    Replacement::draft(0x142082970, "unknown_142082970", tpl_361 as *const ()), // sub_142082970
    Replacement::draft(0x142082980, "unknown_142082980", tpl_361 as *const ()), // sub_142082980
    Replacement::draft(0x142082990, "unknown_142082990", tpl_361 as *const ()), // sub_142082990
    Replacement::draft(0x1420829a0, "unknown_1420829a0", tpl_361 as *const ()), // sub_1420829a0
    Replacement::draft(0x1420829b0, "unknown_1420829b0", tpl_361 as *const ()), // sub_1420829b0
    Replacement::draft(0x1420829c0, "unknown_1420829c0", tpl_361 as *const ()), // sub_1420829c0
    Replacement::draft(0x1420829d0, "unknown_1420829d0", tpl_361 as *const ()), // sub_1420829d0
    Replacement::draft(0x1420829e0, "unknown_1420829e0", tpl_361 as *const ()), // sub_1420829e0
    Replacement::draft(0x1420829f0, "unknown_1420829f0", tpl_361 as *const ()), // sub_1420829f0
    Replacement::draft(0x142082a00, "unknown_142082a00", tpl_361 as *const ()), // sub_142082a00
    Replacement::draft(0x142082a10, "unknown_142082a10", tpl_361 as *const ()), // sub_142082a10
    Replacement::draft(0x142082a20, "unknown_142082a20", tpl_361 as *const ()), // sub_142082a20
    Replacement::draft(0x142082a30, "unknown_142082a30", tpl_44106::<0x50> as *const ()), // sub_142082a30
    Replacement::draft(0x142082f20, "unknown_142082f20", tpl_44106::<0x50> as *const ()), // sub_142082f20
    Replacement::draft(0x142082f30, "unknown_142082f30", tpl_44106::<0x50> as *const ()), // sub_142082f30
    Replacement::draft(0x142082fa0, "unknown_142082fa0", fn_142082fa0 as *const ()), // sub_142082fa0
    Replacement::draft(0x142083150, "unknown_142083150", fn_142083150 as *const ()), // sub_142083150
    Replacement::draft(0x142083450, "unknown_142083450", fn_142083450 as *const ()), // sub_142083450
    Replacement::draft(0x142083930, "unknown_142083930", tpl_44097::<0x8> as *const ()), // sub_142083930
    Replacement::draft(0x142083990, "unknown_142083990", tpl_44098::<0x8> as *const ()), // sub_142083990
    Replacement::draft(0x1420839f0, "unknown_1420839f0", tpl_44099::<0x8> as *const ()), // sub_1420839f0
    Replacement::draft(0x142083af0, "unknown_142083af0", fn_142083af0 as *const ()), // sub_142083af0
    Replacement::draft(0x142083ca0, "unknown_142083ca0", fn_142083ca0 as *const ()), // sub_142083ca0
    Replacement::draft(0x142083e50, "unknown_142083e50", fn_142083e50 as *const ()), // sub_142083e50
    Replacement::draft(0x142084310, "unknown_142084310", tpl_40084::<0x18, 0x1> as *const ()), // sub_142084310
    Replacement::draft(0x142084400, "unknown_142084400", tpl_40083::<0x50> as *const ()), // sub_142084400
    Replacement::draft(0x142084600, "unknown_142084600", tpl_40085::<0x11, 0x8> as *const ()), // sub_142084600
    Replacement::draft(0x142084650, "unknown_142084650", tpl_21274::<0x10, 0x18, 0x1, 0x28> as *const ()), // sub_142084650
    Replacement::draft(0x142084870, "unknown_142084870", tpl_44079::<0x10> as *const ()), // sub_142084870
    Replacement::draft(0x1420848a0, "unknown_1420848a0", tpl_44079::<0x10> as *const ()), // sub_1420848a0
    Replacement::draft(0x142084d60, "unknown_142084d60", tpl_44123::<0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_142084d60
    Replacement::draft(0x1420852b0, "unknown_1420852b0", tpl_44130::<0x10, 0x8> as *const ()), // sub_1420852b0
    Replacement::draft(0x1420853b0, "unknown_1420853b0", tpl_44133::<0x8, 0xc, 0x10, 0x14> as *const ()), // sub_1420853b0
    Replacement::draft(0x142085430, "unknown_142085430", tpl_5650::<0x142084af0, 0x8, 0x10> as *const ()), // sub_142085430
    Replacement::draft(0x142085580, "unknown_142085580", tpl_44123::<0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_142085580
    Replacement::draft(0x1420856f0, "unknown_1420856f0", tpl_40092::<0x18> as *const ()), // sub_1420856f0
    Replacement::draft(0x1420859b0, "unknown_1420859b0", tpl_44142::<0x8> as *const ()), // sub_1420859b0
    Replacement::draft(0x1420859d0, "unknown_1420859d0", tpl_44143::<0x8> as *const ()), // sub_1420859d0
    Replacement::draft_throws(0x142086180, "unknown_142086180", fn_142086180 as *const (), Throws::Discard), // sub_142086180
    Replacement::draft_throws(0x142086f60, "unknown_142086f60", fn_142086f60 as *const (), Throws::Discard), // sub_142086f60
    Replacement::draft(0x142087400, "unknown_142087400", tpl_22131::<0x10> as *const ()), // sub_142087400
    Replacement::draft_throws(0x142087430, "unknown_142087430", fn_142087430 as *const (), Throws::Discard), // sub_142087430
    Replacement::draft(0x1420878f0, "unknown_1420878f0", tpl_22131::<0x10> as *const ()), // sub_1420878f0
    Replacement::draft(0x142087920, "unknown_142087920", fn_142087920 as *const ()), // sub_142087920
    Replacement::draft_throws(0x142087d80, "unknown_142087d80", fn_142087d80 as *const (), Throws::Discard), // sub_142087d80
    Replacement::draft(0x142087fa0, "unknown_142087fa0", tpl_44159::<0x18, 0x210, 0x170> as *const ()), // sub_142087fa0
    Replacement::draft_throws(0x142087fd0, "unknown_142087fd0", tpl_44160::<0x142407c80, 0x143c4aa00, 0x18, 0x14208a430> as *const (), Throws::Discard), // sub_142087fd0
    Replacement::draft_throws(0x142088020, "unknown_142088020", tpl_44161::<0x1426455a8, 0x142087430, 0x142087d80, 0x14205d410, 0x14208ad10, 0x142645788, 0x143a08298, 0x142645598, 0x142645590, 0x142086f60> as *const (), Throws::Discard), // sub_142088020
    Replacement::draft_throws(0x142088250, "unknown_142088250", fn_142088250 as *const (), Throws::Discard), // sub_142088250
    Replacement::draft_throws(0x1420884b0, "unknown_1420884b0", tpl_44165::<0x142085dc0, 0x142085c40, 0x1411dfda0, 0x14397fc80, 0x14205d410, 0x1433dc7e8> as *const (), Throws::Discard), // sub_1420884b0
    Replacement::draft(0x142088680, "unknown_142088680", tpl_420::<0x38, 0x20> as *const ()), // sub_142088680
    Replacement::draft_throws(0x142088940, "unknown_142088940", fn_142088940 as *const (), Throws::Discard), // sub_142088940
    Replacement::draft(0x142088e90, "unknown_142088e90", fn_142088e90 as *const ()), // sub_142088e90
    Replacement::draft(0x142089030, "unknown_142089030", tpl_22131::<0x10> as *const ()), // sub_142089030
    Replacement::draft(0x1420898c0, "unknown_1420898c0", tpl_22131::<0x10> as *const ()), // sub_1420898c0
    Replacement::draft(0x142089a50, "unknown_142089a50", tpl_44182::<0x8> as *const ()), // sub_142089a50
    Replacement::draft_throws(0x142089d10, "unknown_142089d10", fn_142089d10 as *const (), Throws::Discard), // sub_142089d10
    Replacement::draft(0x14208a190, "unknown_14208a190", tpl_44182::<0x8> as *const ()), // sub_14208a190
    Replacement::draft(0x14208a9d0, "anonymous_namespace_14208a9d0", tpl_43721::<0x8, 0x10> as *const ()), // sub_14208a9d0
    Replacement::draft_throws(0x14208aa90, "unknown_14208aa90", fn_14208aa90 as *const (), Throws::Discard), // sub_14208aa90
    Replacement::draft(0x14208ac90, "unknown_14208ac90", tpl_44199::<0x8> as *const ()), // sub_14208ac90
    Replacement::draft(0x14208acf0, "unknown_14208acf0", tpl_44199::<0x8> as *const ()), // sub_14208acf0
    Replacement::draft_throws(0x14208ad10, "unknown_14208ad10", fn_14208ad10 as *const (), Throws::Discard), // sub_14208ad10
    Replacement::draft_throws(0x14208b190, "unknown_14208b190", fn_14208b190 as *const (), Throws::Discard), // sub_14208b190
    Replacement::draft(0x14208c000, "unknown_14208c000", fn_14208c000 as *const ()), // sub_14208c000
    Replacement::draft_throws(0x14208c1c0, "unknown_14208c1c0", fn_14208c1c0 as *const (), Throws::Discard), // sub_14208c1c0
    Replacement::draft_throws(0x14208c6b0, "unknown_14208c6b0", tpl_44161::<0x1426455a8, 0x142087430, 0x142087d80, 0x14205d410, 0x14208ad10, 0x142645788, 0x143a08298, 0x142645598, 0x142645590, 0x142086f60> as *const (), Throws::Discard), // sub_14208c6b0
    Replacement::draft_throws(0x14208c800, "unknown_14208c800", tpl_44165::<0x142085dc0, 0x142085c40, 0x1411dfda0, 0x14397fc80, 0x14205d410, 0x1433dc7e8> as *const (), Throws::Discard), // sub_14208c800
    Replacement::draft(0x14208c9e0, "unknown_14208c9e0", tpl_44159::<0x18, 0x210, 0x170> as *const ()), // sub_14208c9e0
    Replacement::draft_throws(0x14208ca10, "unknown_14208ca10", tpl_44160::<0x142407c80, 0x143c4aa00, 0x18, 0x14208a430> as *const (), Throws::Discard), // sub_14208ca10
    Replacement::draft(0x14208ce80, "unknown_14208ce80", fn_14208ce80 as *const ()), // sub_14208ce80
    Replacement::draft_throws(0x14208d1f0, "unknown_14208d1f0", fn_14208d1f0 as *const (), Throws::Discard), // sub_14208d1f0
    Replacement::draft(0x14208d470, "unknown_14208d470", tpl_44233::<0x14397fc80, 0x14397fc90> as *const ()), // sub_14208d470
    Replacement::draft_throws(0x14208d490, "global_14208d490", fn_14208d490 as *const (), Throws::Discard), // d3d12_init_and_check
    Replacement::draft(0x14208e540, "unknown_14208e540", fn_14208e540 as *const ()), // sub_14208e540
    Replacement::draft(0x14208e990, "unknown_14208e990", fn_14208e990 as *const ()), // sub_14208e990
    Replacement::draft_throws(0x14208ee00, "unknown_14208ee00", fn_14208ee00 as *const (), Throws::Discard), // sub_14208ee00
    Replacement::draft(0x14208f3d0, "unknown_14208f3d0", fn_14208f3d0 as *const ()), // sub_14208f3d0
    Replacement::draft(0x142090890, "unknown_142090890", fn_142090890 as *const ()), // sub_142090890
    Replacement::draft_throws(0x142090e50, "unknown_142090e50", fn_142090e50 as *const (), Throws::Caught), // sub_142090e50
    Replacement::draft(0x142091010, "unknown_142091010", fn_142091010 as *const ()), // sub_142091010
    Replacement::draft(0x142091560, "unknown_142091560", fn_142091560 as *const ()), // sub_142091560
    Replacement::draft(0x1420917c0, "unknown_1420917c0", tpl_28185::<0x1438583cd> as *const ()), // sub_1420917c0
    Replacement::draft(0x1420917d0, "unknown_1420917d0", fn_1420917d0 as *const ()), // sub_1420917d0
    Replacement::draft(0x142091af0, "unknown_142091af0", tpl_28185::<0x1438583cb> as *const ()), // sub_142091af0
    Replacement::draft(0x142091b00, "unknown_142091b00", tpl_28965::<0x1438583f4> as *const ()), // sub_142091b00
    Replacement::draft(0x142091b30, "unknown_142091b30", tpl_28185::<0x1438583c8> as *const ()), // sub_142091b30
    Replacement::draft(0x142091bc0, "unknown_142091bc0", tpl_28185::<0x1438583ce> as *const ()), // sub_142091bc0
    Replacement::draft(0x142091d50, "unknown_142091d50", tpl_906::<0x142647368> as *const ()), // sub_142091d50
    Replacement::draft(0x142091ff0, "unknown_142091ff0", tpl_361 as *const ()), // sub_142091ff0
    Replacement::draft(0x142092000, "unknown_142092000", tpl_361 as *const ()), // sub_142092000
    Replacement::draft(0x142092120, "unknown_142092120", fn_142092120 as *const ()), // sub_142092120
    Replacement::draft(0x1420922c0, "d3d_1420922c0", tpl_44276::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0xc0, 0x142093930, 0x1c8, 0x220, 0x170, 0x118, 0x142645598> as *const ()), // d3d::GlobalDescriptorManager::d3d::GlobalDescriptorManager::vfunc_1
    Replacement::draft(0x142092430, "d3d_142092430", tpl_44276::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0xc0, 0x142093930, 0x1c8, 0x220, 0x170, 0x118, 0x142645598> as *const ()), // sub_142092430
    Replacement::draft(0x1420924f0, "unknown_1420924f0", tpl_44278::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x140, 0x10, 0x14, 0x1403497a0, 0x142645598> as *const ()), // sub_1420924f0
    Replacement::draft_throws(0x1420925b0, "unknown_1420925b0", fn_1420925b0 as *const (), Throws::Discard), // sub_1420925b0
    Replacement::draft_throws(0x142092770, "unknown_142092770", fn_142092770 as *const (), Throws::Discard), // sub_142092770
    Replacement::draft_throws(0x142092930, "unknown_142092930", fn_142092930 as *const (), Throws::Discard), // sub_142092930
    Replacement::draft(0x142092af0, "unknown_142092af0", tpl_34443 as *const ()), // sub_142092af0
    Replacement::draft_throws(0x142092de0, "unknown_142092de0", tpl_44286::<0x14397fcb8, 0x142093200> as *const (), Throws::Discard), // sub_142092de0
    Replacement::draft_throws(0x142092f20, "unknown_142092f20", tpl_44286::<0x14397fcb8, 0x142093200> as *const (), Throws::Discard), // sub_142092f20
    Replacement::draft_throws(0x142093200, "unknown_142093200", fn_142093200 as *const (), Throws::Discard), // sub_142093200
    Replacement::draft(0x142093590, "unknown_142093590", tpl_44293::<0x14397fcb8, 0xb0> as *const ()), // sub_142093590
    Replacement::draft(0x1420935b0, "unknown_1420935b0", tpl_367::<0x80> as *const ()), // sub_1420935b0
    Replacement::draft(0x1420935d0, "d3d_1420935d0", fn_1420935d0 as *const ()), // sub_1420935d0
    Replacement::draft(0x142093910, "d3d_142093910", tpl_32103::<0x10> as *const ()), // sub_142093910
    Replacement::draft_throws(0x142093d00, "d3d_142093d00", fn_142093d00 as *const (), Throws::Discard), // sub_142093d00
    Replacement::draft(0x142093ff0, "d3d_142093ff0", tpl_365::<0x142096c10> as *const ()), // sub_142093ff0
    Replacement::draft_throws(0x142094030, "d3d_142094030", fn_142094030 as *const (), Throws::Discard), // sub_142094030
    Replacement::draft_throws(0x142094450, "d3d_142094450", fn_142094450 as *const (), Throws::Discard), // sub_142094450
    Replacement::draft_throws(0x142094680, "d3d_142094680", fn_142094680 as *const (), Throws::Discard), // sub_142094680
    Replacement::draft(0x142095b60, "unknown_142095b60", tpl_968::<0x58> as *const ()), // sub_142095b60
    Replacement::draft(0x142095b70, "unknown_142095b70", tpl_968::<0x50> as *const ()), // sub_142095b70
    Replacement::draft(0x142095b80, "unknown_142095b80", tpl_968::<0x28> as *const ()), // sub_142095b80
    Replacement::draft(0x142095b90, "unknown_142095b90", tpl_968::<0x20> as *const ()), // sub_142095b90
    Replacement::draft(0x142095cc0, "unknown_142095cc0", fn_142095cc0 as *const ()), // sub_142095cc0
    Replacement::draft(0x142095e90, "unknown_142095e90", fn_142095e90 as *const ()), // sub_142095e90
    Replacement::draft(0x1420960c0, "unknown_1420960c0", fn_1420960c0 as *const ()), // sub_1420960c0
    Replacement::draft_throws(0x142096630, "unknown_142096630", fn_142096630 as *const (), Throws::Discard), // sub_142096630
    Replacement::draft(0x142096a90, "unknown_142096a90", tpl_44278::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0xe8, 0x10, 0x14, 0x1403497a0, 0x142645598> as *const ()), // sub_142096a90
    Replacement::draft(0x142096b50, "unknown_142096b50", tpl_44315::<0x14397fcb8, 0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x248, 0x10, 0x14, 0x1403497a0, 0x142645598> as *const ()), // sub_142096b50
    Replacement::draft(0x142096c10, "unknown_142096c10", tpl_44278::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x1f0, 0x10, 0x14, 0x1403497a0, 0x142645598> as *const ()), // sub_142096c10
    Replacement::draft(0x142096cd0, "unknown_142096cd0", tpl_44315::<0x14397fcb8, 0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x198, 0x10, 0x14, 0x1403497a0, 0x142645598> as *const ()), // sub_142096cd0
    Replacement::draft_throws(0x142096d90, "unknown_142096d90", tpl_44316::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0xc0, 0x142093a30, 0x142645598> as *const (), Throws::Discard), // sub_142096d90
    Replacement::draft_throws(0x142096e20, "unknown_142096e20", tpl_44317::<0x14397fcb8, 0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x220, 0x142093a30, 0x142645598> as *const (), Throws::Discard), // sub_142096e20
    Replacement::draft_throws(0x142096ec0, "unknown_142096ec0", tpl_44316::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x1c8, 0x142093a30, 0x142645598> as *const (), Throws::Discard), // sub_142096ec0
    Replacement::draft_throws(0x142096f50, "unknown_142096f50", tpl_44317::<0x14397fcb8, 0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x170, 0x142093a30, 0x142645598> as *const (), Throws::Discard), // sub_142096f50
    Replacement::draft_throws(0x142096ff0, "unknown_142096ff0", tpl_44316::<0x30, 0x142645790, 0x8, 0x1426455a8, 0xc, 0x118, 0x142093a30, 0x142645598> as *const (), Throws::Discard), // sub_142096ff0
    Replacement::draft(0x142097080, "unknown_142097080", tpl_367::<0xa8> as *const ()), // sub_142097080
    Replacement::draft(0x1420970a0, "unknown_1420970a0", tpl_44293::<0x14397fcb8, 0xa0> as *const ()), // sub_1420970a0
    Replacement::draft(0x1420970c0, "unknown_1420970c0", tpl_367::<0x98> as *const ()), // sub_1420970c0
    Replacement::draft(0x1420970e0, "unknown_1420970e0", tpl_367::<0x90> as *const ()), // sub_1420970e0
    Replacement::draft(0x142097100, "unknown_142097100", tpl_367::<0x88> as *const ()), // sub_142097100
    Replacement::draft_throws(0x142097120, "unknown_142097120", fn_142097120 as *const (), Throws::Discard), // sub_142097120
    Replacement::draft(0x142097340, "unknown_142097340", tpl_361 as *const ()), // sub_142097340
    Replacement::draft(0x142097350, "unknown_142097350", tpl_361 as *const ()), // sub_142097350
    Replacement::draft(0x142097360, "unknown_142097360", tpl_361 as *const ()), // sub_142097360
    Replacement::draft(0x142097370, "unknown_142097370", tpl_361 as *const ()), // sub_142097370
    Replacement::draft(0x142097380, "unknown_142097380", tpl_361 as *const ()), // sub_142097380
    Replacement::draft(0x142097390, "unknown_142097390", tpl_361 as *const ()), // sub_142097390
    Replacement::draft(0x1420973a0, "unknown_1420973a0", tpl_361 as *const ()), // sub_1420973a0
    Replacement::draft(0x1420973b0, "unknown_1420973b0", tpl_361 as *const ()), // sub_1420973b0
    Replacement::draft(0x1420973c0, "unknown_1420973c0", tpl_361 as *const ()), // sub_1420973c0
    Replacement::draft(0x1420973d0, "unknown_1420973d0", tpl_361 as *const ()), // sub_1420973d0
    Replacement::draft(0x1420973e0, "unknown_1420973e0", tpl_361 as *const ()), // sub_1420973e0
    Replacement::draft(0x1420973f0, "unknown_1420973f0", tpl_361 as *const ()), // sub_1420973f0
    Replacement::draft(0x142097470, "unknown_142097470", tpl_43605::<0x14397fd00, 0x1411e3240, 0x142442de0, 0x14397fd18, 0x14397fcf0, 0x14397fce8, 0x14397fcc8, 0x14397fd08, 0x14397fce0, 0x14397fcd8, 0x14397fd10, 0x14397fcc0, 0x14397fcf8, 0x14397fcd0> as *const ()), // sub_142097470
    Replacement::draft(0x142097640, "unknown_142097640", fn_142097640 as *const ()), // sub_142097640
    Replacement::draft(0x142097860, "unknown_142097860", tpl_30780::<0x4> as *const ()), // sub_142097860
    Replacement::draft_throws(0x1420978a0, "unknown_1420978a0", fn_1420978a0 as *const (), Throws::Caught), // sub_1420978a0
    Replacement::draft_throws(0x142097c10, "unknown_142097c10", tpl_22028::<0x14206b020, 0x142442de0> as *const (), Throws::Discard), // sub_142097c10
    Replacement::draft(0x142097fe0, "unknown_142097fe0", tpl_44330 as *const ()), // sub_142097fe0
    Replacement::draft(0x142098210, "unknown_142098210", tpl_44133::<0x8, 0xc, 0x10, 0x14> as *const ()), // sub_142098210
    Replacement::draft(0x142098270, "unknown_142098270", tpl_80418::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1425371a0, 0x142533200> as *const ()), // sub_142098270
    Replacement::draft(0x142098490, "unknown_142098490", tpl_44332::<0x10, 0x8, 0x14> as *const ()), // sub_142098490
    Replacement::draft(0x1420984c0, "unknown_1420984c0", fn_1420984c0 as *const ()), // sub_1420984c0
    Replacement::draft(0x1420986c0, "unknown_1420986c0", tpl_44333::<0x8, 0x10, 0xc, 0x14> as *const ()), // sub_1420986c0
    Replacement::draft(0x142098720, "unknown_142098720", tpl_5650::<0x1420989b0, 0x8, 0x10> as *const ()), // sub_142098720
    Replacement::draft(0x142098800, "unknown_142098800", tpl_23860::<0x8, 0x10> as *const ()), // sub_142098800
    Replacement::draft(0x1420988b0, "unknown_1420988b0", tpl_23860::<0x8, 0x10> as *const ()), // sub_1420988b0
    Replacement::draft(0x1420989b0, "unknown_1420989b0", tpl_5651::<0x8, 0x10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x14, 0x142533200> as *const ()), // sub_1420989b0
    Replacement::draft(0x142098b60, "unknown_142098b60", tpl_44339::<0x8, 0x14> as *const ()), // sub_142098b60
    Replacement::draft(0x142098b80, "unknown_142098b80", tpl_5651::<0x8, 0x10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x14, 0x142533200> as *const ()), // sub_142098b80
    Replacement::draft(0x142098c80, "unknown_142098c80", tpl_44340::<0x14, 0x8> as *const ()), // sub_142098c80
    Replacement::draft(0x142098ca0, "unknown_142098ca0", tpl_44341::<0x10, 0x14, 0x1420984c0, 0xc> as *const ()), // sub_142098ca0
    Replacement::draft(0x142098d00, "unknown_142098d00", tpl_44342::<0x10, 0x8, 0x14, 0xffffffffffffffff> as *const ()), // sub_142098d00
    Replacement::draft(0x142098dd0, "unknown_142098dd0", tpl_365::<0x1411e3380> as *const ()), // sub_142098dd0
    Replacement::draft(0x142098de0, "unknown_142098de0", tpl_44343::<0x8, 0x10> as *const ()), // sub_142098de0
    Replacement::draft(0x142098e00, "unknown_142098e00", tpl_5650::<0x1420989b0, 0x8, 0x10> as *const ()), // sub_142098e00
    Replacement::draft(0x142098e30, "unknown_142098e30", tpl_44333::<0x8, 0x10, 0xc, 0x14> as *const ()), // sub_142098e30
    Replacement::draft(0x142098e80, "unknown_142098e80", tpl_44344::<0x8> as *const ()), // sub_142098e80
    Replacement::draft(0x142098ea0, "unknown_142098ea0", tpl_44344::<0x8> as *const ()), // sub_142098ea0
    Replacement::draft(0x142098ec0, "unknown_142098ec0", tpl_44130::<0x10, 0x8> as *const ()), // sub_142098ec0
    Replacement::draft(0x142098ee0, "unknown_142098ee0", tpl_44130::<0x10, 0x8> as *const ()), // sub_142098ee0
    Replacement::draft(0x142098f00, "unknown_142098f00", tpl_44345::<0x10> as *const ()), // sub_142098f00
    Replacement::draft(0x142098ff0, "unknown_142098ff0", tpl_24002::<0x142645790, 0x1426455a8, 0x1411e3380, 0x1420984c0, 0x142645598> as *const ()), // sub_142098ff0
    Replacement::draft(0x142099100, "d3d_142099100", tpl_44347::<0x142645790, 0x8, 0x1426455a8, 0xc, 0x20, 0x24, 0x10, 0x1420984c0, 0x1c, 0x142645598> as *const ()), // sub_142099100
    Replacement::draft(0x1420991b0, "unknown_1420991b0", tpl_44343::<0x8, 0x10> as *const ()), // sub_1420991b0
    Replacement::draft(0x1420991d0, "unknown_1420991d0", tpl_2964::<0x8, 0x10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1420991d0
    Replacement::draft(0x142099280, "unknown_142099280", tpl_44340::<0x14, 0x8> as *const ()), // sub_142099280
    Replacement::draft(0x1420992a0, "unknown_1420992a0", tpl_44345::<0x10> as *const ()), // sub_1420992a0
    Replacement::draft(0x1420992b0, "unknown_1420992b0", tpl_1691::<0x10> as *const ()), // sub_1420992b0
    Replacement::draft(0x1420992c0, "unknown_1420992c0", tpl_44341::<0x10, 0x14, 0x1420984c0, 0xc> as *const ()), // sub_1420992c0
    Replacement::draft(0x142099320, "unknown_142099320", tpl_44342::<0x10, 0x8, 0x14, 0xffffffffffffffff> as *const ()), // sub_142099320
    Replacement::draft(0x142099370, "unknown_142099370", tpl_44349::<0x8, 0x14> as *const ()), // sub_142099370
    Replacement::draft(0x1420993a0, "unknown_1420993a0", tpl_44142::<0x8> as *const ()), // sub_1420993a0
    Replacement::draft(0x1420993c0, "unknown_1420993c0", tpl_44143::<0x8> as *const ()), // sub_1420993c0
    Replacement::draft(0x1420993e0, "unknown_1420993e0", tpl_44349::<0x8, 0x14> as *const ()), // sub_1420993e0
    Replacement::draft(0x142099470, "unknown_142099470", tpl_5650::<0x1420989b0, 0x8, 0x10> as *const ()), // sub_142099470
    Replacement::draft(0x1420994a0, "unknown_1420994a0", tpl_44333::<0x8, 0x10, 0xc, 0x14> as *const ()), // sub_1420994a0
    Replacement::draft(0x142099750, "unknown_142099750", tpl_44355::<0x8, 0xc, 0x10, 0x18, 0x20, 0x28> as *const ()), // sub_142099750
    Replacement::draft_throws(0x142099860, "unknown_142099860", fn_142099860 as *const (), Throws::Discard), // sub_142099860
    Replacement::draft_throws(0x142099c20, "unknown_142099c20", fn_142099c20 as *const (), Throws::Discard), // sub_142099c20
    Replacement::draft(0x14209a4e0, "d3d_14209a4e0", tpl_44366::<0x143858590> as *const ()), // sub_14209a4e0
    Replacement::draft_throws(0x14209a520, "d3d_14209a520", fn_14209a520 as *const (), Throws::Discard), // sub_14209a520
    Replacement::draft(0x14209aa50, "d3d_14209aa50", tpl_22426::<0x1433dd578, 0x142056db0> as *const ()), // sub_14209aa50
    Replacement::draft(0x14209b330, "unknown_14209b330", tpl_369 as *const ()), // sub_14209b330
    Replacement::draft(0x14209b950, "unknown_14209b950", tpl_44389::<0x14220e9c0, 0x14397fd28, 0x14220ea10, 0x14397fd20> as *const ()), // sub_14209b950
    Replacement::draft_throws(0x14209bb20, "unknown_14209bb20", fn_14209bb20 as *const (), Throws::Caught), // sub_14209bb20
    Replacement::draft(0x14209c080, "unknown_14209c080", tpl_34256::<0x142442de0> as *const ()), // sub_14209c080
    Replacement::draft(0x14209c090, "unknown_14209c090", tpl_365::<0x142075970> as *const ()), // sub_14209c090
    Replacement::draft_throws(0x14209c160, "unknown_14209c160", fn_14209c160 as *const (), Throws::Discard), // sub_14209c160
    Replacement::draft_throws(0x14209c560, "unknown_14209c560", fn_14209c560 as *const (), Throws::Discard), // sub_14209c560
    Replacement::draft_throws(0x14209d6f0, "unknown_14209d6f0", fn_14209d6f0 as *const (), Throws::Discard), // sub_14209d6f0
    Replacement::draft(0x14209da50, "unknown_14209da50", tpl_32636::<0x14209d670, 0x58, 0x1411e7fc0> as *const ()), // sub_14209da50
    Replacement::draft(0x14209e1c0, "unknown_14209e1c0", fn_14209e1c0 as *const ()), // sub_14209e1c0
    Replacement::draft(0x14209e400, "unknown_14209e400", tpl_42543 as *const ()), // sub_14209e400
    Replacement::draft(0x14209e5c0, "unknown_14209e5c0", tpl_361 as *const ()), // sub_14209e5c0
    Replacement::draft(0x14209e5d0, "unknown_14209e5d0", tpl_361 as *const ()), // sub_14209e5d0
    Replacement::draft(0x14209e5e0, "unknown_14209e5e0", tpl_361 as *const ()), // sub_14209e5e0
    Replacement::draft(0x14209e5f0, "unknown_14209e5f0", tpl_361 as *const ()), // sub_14209e5f0
    Replacement::draft(0x14209e600, "unknown_14209e600", tpl_361 as *const ()), // sub_14209e600
    Replacement::draft(0x14209e610, "unknown_14209e610", tpl_361 as *const ()), // sub_14209e610
    Replacement::draft(0x14209e620, "unknown_14209e620", tpl_361 as *const ()), // sub_14209e620
    Replacement::draft(0x14209e630, "unknown_14209e630", tpl_361 as *const ()), // sub_14209e630
    Replacement::draft(0x14209e640, "unknown_14209e640", tpl_361 as *const ()), // sub_14209e640
    Replacement::draft(0x14209e650, "unknown_14209e650", tpl_361 as *const ()), // sub_14209e650
    Replacement::draft(0x14209e660, "unknown_14209e660", tpl_361 as *const ()), // sub_14209e660
    Replacement::draft(0x14209e670, "unknown_14209e670", tpl_361 as *const ()), // sub_14209e670
    Replacement::draft(0x14209e680, "unknown_14209e680", tpl_361 as *const ()), // sub_14209e680
    Replacement::draft(0x14209e690, "unknown_14209e690", tpl_361 as *const ()), // sub_14209e690
    Replacement::draft(0x14209e6a0, "unknown_14209e6a0", tpl_361 as *const ()), // sub_14209e6a0
    Replacement::draft(0x14209e6b0, "unknown_14209e6b0", tpl_361 as *const ()), // sub_14209e6b0
    Replacement::draft(0x14209e6c0, "unknown_14209e6c0", tpl_361 as *const ()), // sub_14209e6c0
    Replacement::draft(0x14209e6d0, "unknown_14209e6d0", tpl_361 as *const ()), // sub_14209e6d0
    Replacement::draft(0x14209e6e0, "unknown_14209e6e0", tpl_361 as *const ()), // sub_14209e6e0
    Replacement::draft(0x14209e760, "unknown_14209e760", fn_14209e760 as *const ()), // sub_14209e760
    Replacement::draft(0x14209e9e0, "unknown_14209e9e0", fn_14209e9e0 as *const ()), // sub_14209e9e0
    Replacement::draft(0x14209ecc0, "unknown_14209ecc0", fn_14209ecc0 as *const ()), // sub_14209ecc0
    Replacement::draft_throws(0x14209f200, "unknown_14209f200", fn_14209f200 as *const (), Throws::Caught), // sub_14209f200
    Replacement::draft_throws(0x14209f3f0, "unknown_14209f3f0", fn_14209f3f0 as *const (), Throws::Discard), // sub_14209f3f0
    Replacement::draft_throws(0x14209f640, "unknown_14209f640", fn_14209f640 as *const (), Throws::Discard), // sub_14209f640
    Replacement::draft_throws(0x14209fa80, "unknown_14209fa80", fn_14209fa80 as *const (), Throws::Discard), // sub_14209fa80
    Replacement::draft_throws(0x14209fd70, "unknown_14209fd70", fn_14209fd70 as *const (), Throws::Discard), // sub_14209fd70
    Replacement::draft_throws(0x14209ff70, "unknown_14209ff70", fn_14209ff70 as *const (), Throws::Caught), // sub_14209ff70
    Replacement::draft_throws(0x1420a0650, "unknown_1420a0650", fn_1420a0650 as *const (), Throws::Discard), // sub_1420a0650
    Replacement::draft(0x1420a0b40, "unknown_1420a0b40", fn_1420a0b40 as *const ()), // sub_1420a0b40
    Replacement::draft(0x1420a0d70, "unknown_1420a0d70", tpl_103600::<0x1411eaff0> as *const ()), // sub_1420a0d70
    Replacement::draft(0x1420a0f60, "unknown_1420a0f60", tpl_103600::<0x1411eaff0> as *const ()), // sub_1420a0f60
    Replacement::draft(0x1420a1250, "unknown_1420a1250", fn_1420a1250 as *const ()), // sub_1420a1250
    Replacement::draft(0x1420a1710, "unknown_1420a1710", fn_1420a1710 as *const ()), // sub_1420a1710
    Replacement::draft(0x1420a1a50, "unknown_1420a1a50", tpl_34252 as *const ()), // sub_1420a1a50
    Replacement::draft_throws(0x1420a1a60, "unknown_1420a1a60", fn_1420a1a60 as *const (), Throws::Caught), // sub_1420a1a60
    Replacement::draft(0x1420a22d0, "unknown_1420a22d0", tpl_361 as *const ()), // sub_1420a22d0
    Replacement::draft(0x1420a22e0, "unknown_1420a22e0", tpl_361 as *const ()), // sub_1420a22e0
    Replacement::draft(0x1420a22f0, "unknown_1420a22f0", tpl_44468::<0x8> as *const ()), // sub_1420a22f0
    Replacement::draft(0x1420a23b0, "unknown_1420a23b0", tpl_1::<0x143a29054> as *const ()), // sub_1420a23b0
    Replacement::draft(0x1420a23e0, "unknown_1420a23e0", tpl_44470::<0x8, 0x18, 0x1420a2320> as *const ()), // sub_1420a23e0
    Replacement::draft(0x1420a2410, "unknown_1420a2410", tpl_913::<0x10> as *const ()), // sub_1420a2410
    Replacement::draft(0x1420a2420, "unknown_1420a2420", tpl_44471::<0x8, 0x1420a2320> as *const ()), // sub_1420a2420
    Replacement::draft(0x1420a2430, "unknown_1420a2430", tpl_44472::<0x8> as *const ()), // sub_1420a2430
    Replacement::draft(0x1420a2460, "unknown_1420a2460", tpl_496 as *const ()), // sub_1420a2460
    Replacement::draft(0x1420a2480, "unknown_1420a2480", tpl_44474::<0x1411eb740, 0x1411eb600> as *const ()), // sub_1420a2480
    Replacement::draft_throws(0x1420a25b0, "unknown_1420a25b0", fn_1420a25b0 as *const (), Throws::Discard), // sub_1420a25b0
    Replacement::draft(0x1420a2a20, "unknown_1420a2a20", tpl_369 as *const ()), // sub_1420a2a20
    Replacement::draft(0x1420a2a30, "unknown_1420a2a30", tpl_369 as *const ()), // sub_1420a2a30
    Replacement::draft(0x1420a2a40, "unknown_1420a2a40", tpl_44476::<0x8, 0x10> as *const ()), // sub_1420a2a40
    Replacement::draft(0x1420a2a70, "unknown_1420a2a70", tpl_36277::<0x8> as *const ()), // sub_1420a2a70
    Replacement::draft(0x1420a2a90, "unknown_1420a2a90", tpl_369 as *const ()), // sub_1420a2a90
    Replacement::draft(0x1420a2aa0, "unknown_1420a2aa0", tpl_36278::<0x8> as *const ()), // sub_1420a2aa0
    Replacement::draft_throws(0x1420a2ab0, "unknown_1420a2ab0", tpl_44477::<0x14241c0a0> as *const (), Throws::Discard), // sub_1420a2ab0
    Replacement::draft(0x1420a2ad0, "unknown_1420a2ad0", tpl_30363::<0x20> as *const ()), // sub_1420a2ad0
    Replacement::draft(0x1420a2b10, "unknown_1420a2b10", tpl_44479::<0x8, 0x1420a3250> as *const ()), // sub_1420a2b10
    Replacement::draft(0x1420a2b60, "unknown_1420a2b60", tpl_44481::<0xc> as *const ()), // sub_1420a2b60
    Replacement::draft(0x1420a2b80, "unknown_1420a2b80", tpl_913::<0x10> as *const ()), // sub_1420a2b80
    Replacement::draft(0x1420a2b90, "unknown_1420a2b90", tpl_34443 as *const ()), // sub_1420a2b90
    Replacement::draft(0x1420a2c10, "unknown_1420a2c10", tpl_361 as *const ()), // sub_1420a2c10
    Replacement::draft(0x1420a2c20, "unknown_1420a2c20", tpl_361 as *const ()), // sub_1420a2c20
    Replacement::draft(0x1420a2c50, "unknown_1420a2c50", tpl_361 as *const ()), // sub_1420a2c50
    Replacement::draft(0x1420a2c60, "unknown_1420a2c60", tpl_361 as *const ()), // sub_1420a2c60
    Replacement::draft(0x1420a2c70, "unknown_1420a2c70", tpl_361 as *const ()), // sub_1420a2c70
    Replacement::draft(0x1420a2c80, "unknown_1420a2c80", tpl_361 as *const ()), // sub_1420a2c80
    Replacement::draft(0x1420a2c90, "unknown_1420a2c90", tpl_40083::<0x20> as *const ()), // sub_1420a2c90
    Replacement::draft(0x1420a2ca0, "unknown_1420a2ca0", tpl_361 as *const ()), // sub_1420a2ca0
    Replacement::draft(0x1420a2cb0, "unknown_1420a2cb0", tpl_44106::<0x20> as *const ()), // sub_1420a2cb0
    Replacement::draft(0x1420a2d00, "unknown_1420a2d00", tpl_496 as *const ()), // sub_1420a2d00
    Replacement::draft(0x1420a2d10, "unknown_1420a2d10", tpl_361 as *const ()), // sub_1420a2d10
    Replacement::draft(0x1420a2d20, "unknown_1420a2d20", tpl_361 as *const ()), // sub_1420a2d20
    Replacement::draft(0x1420a2d30, "unknown_1420a2d30", tpl_548::<0xc> as *const ()), // sub_1420a2d30
    Replacement::draft_throws(0x1420a2d40, "unknown_1420a2d40", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_1420a2d40
    Replacement::draft(0x1420a2d60, "unknown_1420a2d60", tpl_35253 as *const ()), // sub_1420a2d60
    Replacement::draft(0x1420a2db0, "unknown_1420a2db0", tpl_44488::<0x20, 0x8, 0x10> as *const ()), // sub_1420a2db0
    Replacement::draft(0x1420a2e00, "unknown_1420a2e00", tpl_44488::<0x20, 0x8, 0x10> as *const ()), // sub_1420a2e00
    Replacement::draft(0x1420a2e50, "unknown_1420a2e50", tpl_44488::<0x20, 0x8, 0x10> as *const ()), // sub_1420a2e50
    Replacement::draft(0x1420a2ea0, "unknown_1420a2ea0", tpl_44488::<0x20, 0x8, 0x10> as *const ()), // sub_1420a2ea0
    Replacement::draft(0x1420a2ef0, "unknown_1420a2ef0", tpl_44488::<0x20, 0x8, 0x10> as *const ()), // sub_1420a2ef0
    Replacement::draft(0x1420a2f40, "unknown_1420a2f40", tpl_44488::<0x20, 0x8, 0x10> as *const ()), // sub_1420a2f40
    Replacement::draft(0x1420a2f90, "unknown_1420a2f90", tpl_895::<0x8> as *const ()), // sub_1420a2f90
    Replacement::draft(0x1420a2fb0, "unknown_1420a2fb0", tpl_361 as *const ()), // sub_1420a2fb0
    Replacement::draft(0x1420a2fc0, "unknown_1420a2fc0", tpl_361 as *const ()), // sub_1420a2fc0
    Replacement::draft(0x1420a2fd0, "unknown_1420a2fd0", tpl_29744::<0x8> as *const ()), // sub_1420a2fd0
    Replacement::draft(0x1420a2fe0, "unknown_1420a2fe0", tpl_35253 as *const ()), // sub_1420a2fe0
    Replacement::draft(0x1420a2ff0, "unknown_1420a2ff0", tpl_473 as *const ()), // sub_1420a2ff0
    Replacement::draft(0x1420a3000, "unknown_1420a3000", tpl_44489 as *const ()), // sub_1420a3000
    Replacement::draft(0x1420a3020, "unknown_1420a3020", tpl_895::<0x8> as *const ()), // sub_1420a3020
    Replacement::draft(0x1420a3040, "unknown_1420a3040", tpl_548::<0x8> as *const ()), // sub_1420a3040
    Replacement::draft(0x1420a3050, "unknown_1420a3050", tpl_473 as *const ()), // sub_1420a3050
    Replacement::draft(0x1420a3060, "unknown_1420a3060", tpl_44481::<0x8> as *const ()), // sub_1420a3060
    Replacement::draft(0x1420a3080, "unknown_1420a3080", tpl_22031::<0x8> as *const ()), // sub_1420a3080
    Replacement::draft(0x1420a30d0, "unknown_1420a30d0", tpl_361 as *const ()), // sub_1420a30d0
    Replacement::draft(0x1420a30e0, "unknown_1420a30e0", tpl_44492 as *const ()), // sub_1420a30e0
    Replacement::draft(0x1420a30f0, "unknown_1420a30f0", tpl_496 as *const ()), // sub_1420a30f0
    Replacement::draft(0x1420a3120, "unknown_1420a3120", tpl_367::<0x8> as *const ()), // sub_1420a3120
    Replacement::draft(0x1420a3140, "unknown_1420a3140", tpl_44494::<0x8> as *const ()), // sub_1420a3140
    Replacement::draft(0x1420a3160, "unknown_1420a3160", tpl_361 as *const ()), // sub_1420a3160
    Replacement::draft(0x1420a3170, "unknown_1420a3170", tpl_361 as *const ()), // sub_1420a3170
    Replacement::draft(0x1420a3180, "unknown_1420a3180", tpl_361 as *const ()), // sub_1420a3180
    Replacement::draft(0x1420a3190, "unknown_1420a3190", tpl_361 as *const ()), // sub_1420a3190
    Replacement::draft(0x1420a31a0, "unknown_1420a31a0", tpl_361 as *const ()), // sub_1420a31a0
    Replacement::draft(0x1420a31b0, "unknown_1420a31b0", tpl_44495 as *const ()), // sub_1420a31b0
    Replacement::draft(0x1420a31c0, "unknown_1420a31c0", tpl_42450::<0x8, 0x10> as *const ()), // sub_1420a31c0
    Replacement::draft(0x1420a3230, "unknown_1420a3230", tpl_361 as *const ()), // sub_1420a3230
    Replacement::draft(0x1420a3240, "unknown_1420a3240", tpl_44495 as *const ()), // sub_1420a3240
    Replacement::draft(0x1420a32d0, "unknown_1420a32d0", tpl_42449 as *const ()), // sub_1420a32d0
    Replacement::draft(0x1420a33c0, "unknown_1420a33c0", tpl_44495 as *const ()), // sub_1420a33c0
    Replacement::draft(0x1420a3470, "unknown_1420a3470", tpl_361 as *const ()), // sub_1420a3470
    Replacement::draft(0x1420a3480, "unknown_1420a3480", tpl_44501::<0x8> as *const ()), // sub_1420a3480
    Replacement::draft(0x1420a3510, "unknown_1420a3510", tpl_44503::<0x8> as *const ()), // sub_1420a3510
    Replacement::draft(0x1420a35c0, "unknown_1420a35c0", tpl_21554::<0x18> as *const ()), // sub_1420a35c0
    Replacement::draft(0x1420a3640, "unknown_1420a3640", tpl_42455::<0x8, 0x10> as *const ()), // sub_1420a3640
    Replacement::draft(0x1420a3660, "unknown_1420a3660", tpl_361 as *const ()), // sub_1420a3660
    Replacement::draft(0x1420a3690, "unknown_1420a3690", tpl_44503::<0x8> as *const ()), // sub_1420a3690
    Replacement::draft(0x1420a3700, "unknown_1420a3700", tpl_42452::<0x8> as *const ()), // sub_1420a3700
    Replacement::draft(0x1420a37a0, "unknown_1420a37a0", tpl_44503::<0x8> as *const ()), // sub_1420a37a0
    Replacement::draft(0x1420a37c0, "unknown_1420a37c0", tpl_44511::<0x8, 0x10> as *const ()), // sub_1420a37c0
    Replacement::draft(0x1420a3810, "unknown_1420a3810", tpl_913::<0x10> as *const ()), // sub_1420a3810
    Replacement::draft(0x1420a3850, "unknown_1420a3850", tpl_361 as *const ()), // sub_1420a3850
    Replacement::draft(0x1420a3860, "unknown_1420a3860", tpl_42439::<0x9> as *const ()), // sub_1420a3860
    Replacement::draft(0x1420a38b0, "unknown_1420a38b0", tpl_38188::<0x18> as *const ()), // sub_1420a38b0
    Replacement::draft(0x1420a38f0, "unknown_1420a38f0", tpl_44517::<0x8> as *const ()), // sub_1420a38f0
    Replacement::draft_throws(0x1420a3910, "unknown_1420a3910", fn_1420a3910 as *const (), Throws::Caught), // sub_1420a3910
    Replacement::draft(0x1420a3b60, "unknown_1420a3b60", tpl_496 as *const ()), // sub_1420a3b60
    Replacement::draft_throws(0x1420a3b70, "unknown_1420a3b70", fn_1420a3b70 as *const (), Throws::Discard), // sub_1420a3b70
    Replacement::draft(0x1420a3d60, "unknown_1420a3d60", tpl_44520 as *const ()), // sub_1420a3d60
    Replacement::draft_throws(0x1420a3df0, "unknown_1420a3df0", tpl_22293::<0x18, 0x14241c190, 0x20, 0x8, 0x10> as *const (), Throws::Discard), // sub_1420a3df0
    Replacement::draft(0x1420a4440, "unknown_1420a4440", tpl_496 as *const ()), // sub_1420a4440
    Replacement::draft(0x1420a4450, "unknown_1420a4450", tpl_34443 as *const ()), // sub_1420a4450
    Replacement::draft(0x1420a4650, "unknown_1420a4650", tpl_26849::<0xa> as *const ()), // sub_1420a4650
    Replacement::draft(0x1420a4660, "unknown_1420a4660", tpl_496 as *const ()), // sub_1420a4660
    Replacement::draft(0x1420a4670, "unknown_1420a4670", tpl_44529::<0x8> as *const ()), // sub_1420a4670
    Replacement::draft(0x1420a4ad0, "unknown_1420a4ad0", tpl_34443 as *const ()), // sub_1420a4ad0
    Replacement::draft(0x1420a50a0, "unknown_1420a50a0", tpl_11355::<0x18, 0x20> as *const ()), // sub_1420a50a0
    Replacement::draft(0x1420a50c0, "unknown_1420a50c0", tpl_895::<0x8> as *const ()), // sub_1420a50c0
    Replacement::draft_throws(0x1420a5150, "unknown_1420a5150", fn_1420a5150 as *const (), Throws::Discard), // sub_1420a5150
    Replacement::draft(0x1420a5310, "unknown_1420a5310", tpl_40069::<0x8, 0xc> as *const ()), // sub_1420a5310
    Replacement::draft_throws(0x1420a5380, "unknown_1420a5380", fn_1420a5380 as *const (), Throws::Discard), // sub_1420a5380
    Replacement::draft_throws(0x1420a5ba0, "unknown_1420a5ba0", fn_1420a5ba0 as *const (), Throws::Discard), // sub_1420a5ba0
    Replacement::draft_throws(0x1420a5dc0, "unknown_1420a5dc0", fn_1420a5dc0 as *const (), Throws::Discard), // sub_1420a5dc0
    Replacement::draft(0x1420a6000, "unknown_1420a6000", tpl_496 as *const ()), // sub_1420a6000
    Replacement::draft(0x1420a6010, "unknown_1420a6010", tpl_34443 as *const ()), // sub_1420a6010
    Replacement::draft(0x1420a62f0, "unknown_1420a62f0", tpl_895::<0x8> as *const ()), // sub_1420a62f0
    Replacement::draft(0x1420a6560, "unknown_1420a6560", tpl_44556::<0x10, 0x20> as *const ()), // sub_1420a6560
    Replacement::draft(0x1420a6580, "unknown_1420a6580", tpl_44556::<0x10, 0x21> as *const ()), // sub_1420a6580
    Replacement::draft_throws(0x1420a6680, "unknown_1420a6680", fn_1420a6680 as *const (), Throws::Discard), // sub_1420a6680
    Replacement::draft(0x1420a6d90, "unknown_1420a6d90", tpl_44566::<0x8, 0x10, 0x18> as *const ()), // sub_1420a6d90
];
