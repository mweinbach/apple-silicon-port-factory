#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft_throws(0x1403e7860, "bigfish_1403e7860", fn_1403e7860 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setupMapVisualizations>
    Replacement::draft_throws(0x1403e7b40, "bigfish_1403e7b40", tpl_3744::<0x143c9be1c, 0x143c9bdf0, 0x1411e9f50, 0x1403e9750, 0x143c9bd58, 0x1401fc320, 0x1403e9010, 0x1420a28e0, 0x1403e9510, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143101526, 0x142646960, 0x143c9be10, 0x143c9be14, 0x143c9be16, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::cleanupMapVisualizations>
    Replacement::draft_throws(0x1403e7e10, "unknown_1403e7e10", tpl_3745::<0x19, 0x10, 0x1403e7e10, 0x14241c190> as *const (), Throws::Discard), // sub_1403e7e10
    Replacement::draft_throws(0x1403e7e70, "unknown_1403e7e70", fn_1403e7e70 as *const (), Throws::Discard), // sub_1403e7e70
    Replacement::draft_throws(0x1403e7f10, "unknown_1403e7f10", fn_1403e7f10 as *const (), Throws::Discard), // sub_1403e7f10
    Replacement::draft(0x1403e80f0, "unknown_1403e80f0", fn_1403e80f0 as *const ()), // sub_1403e80f0
    Replacement::draft(0x1403e82c0, "unknown_1403e82c0", fn_1403e82c0 as *const ()), // sub_1403e82c0
    Replacement::draft_throws(0x1403e8480, "unknown_1403e8480", fn_1403e8480 as *const (), Throws::Caught), // sub_1403e8480
    Replacement::draft(0x1403e8600, "unknown_1403e8600", fn_1403e8600 as *const ()), // sub_1403e8600
    Replacement::draft_throws(0x1403e8660, "unknown_1403e8660", tpl_1330::<0x8, 0x1403e96a0, 0x143c9bde4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1403e8660
    Replacement::draft(0x1403e8740, "unknown_1403e8740", fn_1403e8740 as *const ()), // sub_1403e8740
    Replacement::draft(0x1403e8890, "unknown_1403e8890", tpl_3260::<0x7, 0x48, 0x38, 0x14028d970, 0x10, 0x14240ab10, 0x1401f2bc0> as *const ()), // sub_1403e8890
    Replacement::draft(0x1403e88f0, "unknown_1403e88f0", tpl_3752::<0x143c8ceb4, 0x20, 0x8, 0x141957c00> as *const ()), // sub_1403e88f0
    Replacement::draft_throws(0x1403e8950, "unknown_1403e8950", fn_1403e8950 as *const (), Throws::Discard), // sub_1403e8950
    Replacement::draft_throws(0x1403e8a30, "unknown_1403e8a30", fn_1403e8a30 as *const (), Throws::Caught), // sub_1403e8a30
    Replacement::draft_throws(0x1403e8b60, "unknown_1403e8b60", fn_1403e8b60 as *const (), Throws::Discard), // sub_1403e8b60
    Replacement::draft_throws(0x1403e9300, "unknown_1403e9300", fn_1403e9300 as *const (), Throws::Caught), // sub_1403e9300
    Replacement::draft_throws(0x1403e9510, "unknown_1403e9510", fn_1403e9510 as *const (), Throws::Caught), // sub_1403e9510
    Replacement::draft(0x1403e9750, "unknown_1403e9750", tpl_3188::<0x1403e9e90, 0x143102850, 0x1403e9bb0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1403e9750
    Replacement::draft(0x1403e99d0, "unknown_1403e99d0", fn_1403e99d0 as *const ()), // sub_1403e99d0
    Replacement::draft(0x1403e9a40, "unknown_1403e9a40", fn_1403e9a40 as *const ()), // sub_1403e9a40
    Replacement::draft(0x1403e9bb0, "unknown_1403e9bb0", fn_1403e9bb0 as *const ()), // sub_1403e9bb0
    Replacement::draft(0x1403e9ce0, "unknown_1403e9ce0", fn_1403e9ce0 as *const ()), // sub_1403e9ce0
    Replacement::draft(0x1403e9e90, "unknown_1403e9e90", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143102850, 0x8, 0x20> as *const ()), // sub_1403e9e90
    Replacement::draft(0x1403ea070, "unknown_1403ea070", tpl_1824::<0x8, 0x38, 0x14028d970, 0x10, 0x14240ab10, 0x1401f2bc0> as *const ()), // sub_1403ea070
    Replacement::draft(0x1403ea290, "unknown_1403ea290", fn_1403ea290 as *const ()), // sub_1403ea290
    Replacement::draft(0x1403ea390, "unknown_1403ea390", fn_1403ea390 as *const ()), // sub_1403ea390
    Replacement::draft(0x1403ea490, "unknown_1403ea490", fn_1403ea490 as *const ()), // sub_1403ea490
    Replacement::draft(0x1403ea4c0, "unknown_1403ea4c0", fn_1403ea4c0 as *const ()), // sub_1403ea4c0
    Replacement::draft(0x1403ea580, "unknown_1403ea580", fn_1403ea580 as *const ()), // sub_1403ea580
    Replacement::draft(0x1403ea6c0, "unknown_1403ea6c0", fn_1403ea6c0 as *const ()), // sub_1403ea6c0
    Replacement::draft(0x1403ea7a0, "unknown_1403ea7a0", fn_1403ea7a0 as *const ()), // sub_1403ea7a0
    Replacement::draft(0x1403ea8f0, "unknown_1403ea8f0", fn_1403ea8f0 as *const ()), // sub_1403ea8f0
    Replacement::draft(0x1403ea9a0, "unknown_1403ea9a0", tpl_3774::<0x30, 0x143c8cf5c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1403eaa22> as *const ()), // sub_1403ea9a0
    Replacement::draft(0x1403eaa40, "unknown_1403eaa40", tpl_3775::<0x8, 0x19, 0x10, 0x140246bb0, 0x1401e66f0> as *const ()), // sub_1403eaa40
    Replacement::draft(0x1403eaac0, "unknown_1403eaac0", tpl_63073::<0x143c9bebc, 0x142489ee0, 0x143c9c01c, 0x143c8d120> as *const ()), // sub_1403eaac0
    Replacement::draft_throws(0x1403eaed0, "unknown_1403eaed0", tpl_1480::<0x142721680, 0x1410d6ca0, 0x1403ecea0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1403eaed0
    Replacement::draft_throws(0x1403eb170, "unknown_1403eb170", tpl_1480::<0x142721680, 0x1410d6ca0, 0x1403ed210, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1403eb170
    Replacement::draft_throws(0x1403eb410, "unknown_1403eb410", tpl_1480::<0x142721680, 0x1410d6ca0, 0x1403ed580, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1403eb410
    Replacement::draft_throws(0x1403eb6b0, "bigfish_1403eb6b0", fn_1403eb6b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setMapMarkerMapAndRooms>
    Replacement::draft_throws(0x1403eba20, "bigfish_1403eba20", fn_1403eba20 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setRoomClearState>
    Replacement::draft_throws(0x1403ebd60, "bigfish_1403ebd60", tpl_3312::<0x143c9c27c, 0x143c9c450, 0x1411e9f50, 0x1403ee5e0, 0x1403efa00, 0x1403ee7d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431028f6, 0x142646960, 0x143c9c470, 0x143c9c474, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setMapMarkerHiddenState>
    Replacement::draft(0x1403ebfb0, "bigfish_1403ebfb0", fn_1403ebfb0 as *const ()), // sub_1403ebfb0
    Replacement::draft(0x1403ec100, "bigfish_1403ec100", tpl_1094::<0x143c9c02c, 0x1403ed910, 0x14020e8f0> as *const ()), // sub_1403ec100
    Replacement::draft(0x1403ec300, "bigfish_1403ec300", tpl_1094::<0x143c9c1a8, 0x1403edad0, 0x14020e8f0> as *const ()), // sub_1403ec300
    Replacement::draft(0x1403ec500, "bigfish_1403ec500", tpl_1094::<0x143c9c280, 0x1403edcf0, 0x14020e8f0> as *const ()), // sub_1403ec500
    Replacement::draft(0x1403ec700, "bigfish_1403ec700", tpl_1094::<0x143c9bd60, 0x1403ede70, 0x14020e8f0> as *const ()), // sub_1403ec700
    Replacement::draft(0x1403ec900, "bigfish_1403ec900", tpl_1094::<0x143c9bebc, 0x1403ee070, 0x14020e8f0> as *const ()), // sub_1403ec900
    Replacement::draft(0x1403ecb00, "bigfish_1403ecb00", tpl_1369::<0x143c9c034, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1403ecb00
    Replacement::draft_throws(0x1403ecea0, "bigfish_1403ecea0", fn_1403ecea0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setupMapAreas>
    Replacement::draft_throws(0x1403ed210, "bigfish_1403ed210", fn_1403ed210 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setupMapMarkers>
    Replacement::draft_throws(0x1403ed580, "bigfish_1403ed580", fn_1403ed580 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::map::setupSavedMapDataBoundLifetime>
    Replacement::draft(0x1403ed910, "unknown_1403ed910", fn_1403ed910 as *const ()), // sub_1403ed910
    Replacement::draft(0x1403edad0, "unknown_1403edad0", fn_1403edad0 as *const ()), // sub_1403edad0
    Replacement::draft(0x1403edcf0, "unknown_1403edcf0", fn_1403edcf0 as *const ()), // sub_1403edcf0
    Replacement::draft(0x1403ede70, "unknown_1403ede70", fn_1403ede70 as *const ()), // sub_1403ede70
    Replacement::draft(0x1403ee070, "unknown_1403ee070", fn_1403ee070 as *const ()), // sub_1403ee070
    Replacement::draft_throws(0x1403ee2c0, "unknown_1403ee2c0", fn_1403ee2c0 as *const (), Throws::Discard), // sub_1403ee2c0
    Replacement::draft_throws(0x1403ee3b0, "unknown_1403ee3b0", tpl_3788::<0x18, 0x20, 0x30, 0x8, 0x28, 0x14195b1f0> as *const (), Throws::Discard), // sub_1403ee3b0
    Replacement::draft_throws(0x1403ee5e0, "unknown_1403ee5e0", tpl_3790::<0x1403f0d30, 0x143c9c4a0, 0x1420af120, 0x1403f0bf0, 0x143c9c44c, 0x140215750, 0x1403eeca0, 0x143c8cef8, 0x140215610, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1403ee5e0
    Replacement::draft(0x1403ee7d0, "unknown_1403ee7d0", fn_1403ee7d0 as *const ()), // sub_1403ee7d0
    Replacement::draft(0x1403ee930, "unknown_1403ee930", fn_1403ee930 as *const ()), // sub_1403ee930
    Replacement::draft(0x1403eea50, "unknown_1403eea50", fn_1403eea50 as *const ()), // sub_1403eea50
    Replacement::draft(0x1403eead0, "content_1403eead0", fn_1403eead0 as *const ()), // content::MapMarkerComponent::content::MapMarkerComponent::vfunc_0
    Replacement::draft(0x1403eeb30, "unknown_1403eeb30", tpl_3795::<0x143c8d694, 0x142489ee0, 0x143c9bebc, 0x143c8d120, 0x143c8d87c> as *const ()), // sub_1403eeb30
    Replacement::draft_throws(0x1403eeca0, "unknown_1403eeca0", tpl_1330::<0x8, 0x1403f0b40, 0x143c9c108, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1403eeca0
    Replacement::draft(0x1403eed80, "unknown_1403eed80", tpl_3796::<0x143c9bebc, 0x142489ee0, 0x143c8d120, 0x143c8d694> as *const ()), // sub_1403eed80
    Replacement::draft_throws(0x1403eeed0, "unknown_1403eeed0", tpl_3146::<0x143c9c028, 0x143c9c1b4, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8ce9c, 0x143c9c1b0, 0x143c9c1b8, 0x141834a0c> as *const (), Throws::Discard), // sub_1403eeed0
    Replacement::draft(0x1403eefb0, "unknown_1403eefb0", fn_1403eefb0 as *const ()), // sub_1403eefb0
    Replacement::draft(0x1403ef000, "unknown_1403ef000", tpl_1124::<0x90> as *const ()), // sub_1403ef000
    Replacement::draft(0x1403ef040, "unknown_1403ef040", tpl_3798::<0x38, 0x14240ab10> as *const ()), // sub_1403ef040
    Replacement::draft(0x1403ef080, "unknown_1403ef080", fn_1403ef080 as *const ()), // sub_1403ef080
    Replacement::draft(0x1403ef0e0, "unknown_1403ef0e0", fn_1403ef0e0 as *const ()), // sub_1403ef0e0
    Replacement::draft(0x1403ef130, "unknown_1403ef130", tpl_3801::<0x88, 0x10088, 0x143c9c408, 0x20, 0x8, 0x1403f0080, 0x1403ef220> as *const ()), // sub_1403ef130
    Replacement::draft(0x1403ef220, "unknown_1403ef220", tpl_3802::<0x18, 0x20, 0x30, 0x8, 0x28, 0x14195c1d0> as *const ()), // sub_1403ef220
    Replacement::draft_throws(0x1403ef300, "unknown_1403ef300", tpl_3803::<0x88, 0x10088, 0x143c9c030, 0x20, 0x8, 0x1403eff20, 0x1403ef3f0> as *const (), Throws::Discard), // sub_1403ef300
    Replacement::draft_throws(0x1403ef3f0, "unknown_1403ef3f0", tpl_3804::<0x18, 0x20, 0x8, 0x14195bac0> as *const (), Throws::Discard), // sub_1403ef3f0
    Replacement::draft(0x1403ef4d0, "unknown_1403ef4d0", tpl_63093::<0x143c9bebc, 0x142489ee0, 0x14195b710> as *const ()), // sub_1403ef4d0
    Replacement::draft_throws(0x1403ef5d0, "unknown_1403ef5d0", fn_1403ef5d0 as *const (), Throws::Discard), // sub_1403ef5d0
    Replacement::draft_throws(0x1403ef850, "unknown_1403ef850", fn_1403ef850 as *const (), Throws::Discard), // sub_1403ef850
    Replacement::draft(0x1403efa00, "unknown_1403efa00", fn_1403efa00 as *const ()), // sub_1403efa00
    Replacement::draft_throws(0x1403f3360, "unknown_1403f3360", fn_1403f3360 as *const (), Throws::Discard), // sub_1403f3360
    Replacement::draft_throws(0x1403f3510, "unknown_1403f3510", tpl_847::<0x143c9c658, 0x143c9c70c, 0x143c9c5a0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9bd58, 0x141834a0c> as *const (), Throws::Discard), // sub_1403f3510
    Replacement::draft_throws(0x1403f3640, "unknown_1403f3640", fn_1403f3640 as *const (), Throws::Discard), // sub_1403f3640
    Replacement::draft(0x1403f37b0, "bigfish_1403f37b0", fn_1403f37b0 as *const ()), // bigfish::game_ui_page::MainUIPage::bigfish::game_ui_page::MainUIPage::vfunc_0
    Replacement::draft(0x1403f3830, "unknown_1403f3830", fn_1403f3830 as *const ()), // sub_1403f3830
    Replacement::draft(0x1403f38e0, "unknown_1403f38e0", tpl_3818::<0x88, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143079950, 0x1418a0600> as *const ()), // sub_1403f38e0
    Replacement::draft(0x1403f3980, "unknown_1403f3980", tpl_3818::<0x88, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143106628, 0x1418a0600> as *const ()), // sub_1403f3980
    Replacement::draft(0x1403f3a20, "unknown_1403f3a20", tpl_3818::<0x88, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143106640, 0x1418a0600> as *const ()), // sub_1403f3a20
    Replacement::draft(0x1403f3ac0, "cohtml_1403f3ac0", tpl_3819::<0x1403f3b10> as *const ()), // cohtml::EventHandler<void (bigfish::game_ui_page::MainUIPage::*)(const r::DynamicArray<r::InplaceString<32>,r::Allocator
    Replacement::draft(0x1403f3ad0, "cohtml_1403f3ad0", tpl_1686::<0x1430a0020, 0x142646d08, 0x1401de410> as *const ()), // cohtml::EventHandler<bool (ui::Page::*)(unsigned int)>::cohtml::EventHandler<bool (ui::Page::*)(unsigned int)>::vfunc_0
    Replacement::draft(0x1403f3b10, "cohtml_1403f3b10", fn_1403f3b10 as *const ()), // sub_1403f3b10
    Replacement::draft(0x1403f3cb0, "cohtml_1403f3cb0", fn_1403f3cb0 as *const ()), // sub_1403f3cb0
    Replacement::draft(0x1403f3d40, "unknown_1403f3d40", fn_1403f3d40 as *const ()), // sub_1403f3d40
    Replacement::draft(0x1403f3e60, "unknown_1403f3e60", fn_1403f3e60 as *const ()), // sub_1403f3e60
    Replacement::draft(0x1403f3ed0, "unknown_1403f3ed0", fn_1403f3ed0 as *const ()), // sub_1403f3ed0
    Replacement::draft(0x1403f3f60, "unknown_1403f3f60", fn_1403f3f60 as *const ()), // sub_1403f3f60
    Replacement::draft(0x1403f4030, "content_1403f4030", fn_1403f4030 as *const ()), // sub_1403f4030
    Replacement::draft_throws(0x1403f4140, "content_1403f4140", fn_1403f4140 as *const (), Throws::Discard), // sub_1403f4140
    Replacement::draft_throws(0x1403f4270, "content_1403f4270", fn_1403f4270 as *const (), Throws::Discard), // sub_1403f4270
    Replacement::draft_throws(0x1403f4460, "content_1403f4460", fn_1403f4460 as *const (), Throws::Discard), // sub_1403f4460
    Replacement::draft_throws(0x1403f44f0, "content_1403f44f0", fn_1403f44f0 as *const (), Throws::Discard), // sub_1403f44f0
    Replacement::draft_throws(0x1403f45a0, "content_1403f45a0", fn_1403f45a0 as *const (), Throws::Discard), // sub_1403f45a0
    Replacement::draft_throws(0x1403f4620, "content_1403f4620", fn_1403f4620 as *const (), Throws::Discard), // sub_1403f4620
    Replacement::draft_throws(0x1403f46d0, "content_1403f46d0", fn_1403f46d0 as *const (), Throws::Discard), // sub_1403f46d0
    Replacement::draft_throws(0x1403f4750, "content_1403f4750", fn_1403f4750 as *const (), Throws::Discard), // sub_1403f4750
    Replacement::draft_throws(0x1403f4810, "content_1403f4810", fn_1403f4810 as *const (), Throws::Discard), // sub_1403f4810
    Replacement::draft_throws(0x1403f48d0, "content_1403f48d0", fn_1403f48d0 as *const (), Throws::Discard), // sub_1403f48d0
    Replacement::draft_throws(0x1403f4960, "content_1403f4960", fn_1403f4960 as *const (), Throws::Discard), // sub_1403f4960
    Replacement::draft_throws(0x1403f49e0, "content_1403f49e0", fn_1403f49e0 as *const (), Throws::Discard), // sub_1403f49e0
    Replacement::draft_throws(0x1403f4bc0, "content_1403f4bc0", fn_1403f4bc0 as *const (), Throws::Discard), // sub_1403f4bc0
    Replacement::draft_throws(0x1403f4d30, "content_1403f4d30", fn_1403f4d30 as *const (), Throws::Discard), // sub_1403f4d30
    Replacement::draft(0x1403f4e10, "content_1403f4e10", fn_1403f4e10 as *const ()), // content::CameraComponent::content::CameraComponent::vfunc_4
    Replacement::draft(0x1403f4e30, "content_1403f4e30", fn_1403f4e30 as *const ()), // content::CameraComponent::content::CameraComponent::vfunc_12
    Replacement::draft(0x1403f4e90, "content_1403f4e90", fn_1403f4e90 as *const ()), // sub_1403f4e90
    Replacement::draft(0x1403f4ee0, "content_1403f4ee0", fn_1403f4ee0 as *const ()), // sub_1403f4ee0
    Replacement::draft(0x1403f4f50, "content_1403f4f50", fn_1403f4f50 as *const ()), // sub_1403f4f50
    Replacement::draft(0x1403f4fe0, "content_1403f4fe0", fn_1403f4fe0 as *const ()), // sub_1403f4fe0
    Replacement::draft(0x1403f5180, "content_1403f5180", fn_1403f5180 as *const ()), // content::LadderComponent::content::LadderComponent::vfunc_4
    Replacement::draft(0x1403f5190, "content_1403f5190", fn_1403f5190 as *const ()), // content::LadderComponent::content::LadderComponent::vfunc_5
    Replacement::draft(0x1403f51a0, "content_1403f51a0", fn_1403f51a0 as *const ()), // content::LadderComponent::content::LadderComponent::vfunc_6
    Replacement::draft(0x1403f51b0, "content_1403f51b0", fn_1403f51b0 as *const ()), // content::BoxShapeComponent::content::BoxShapeComponent::vfunc_10
    Replacement::draft_throws(0x1403f5260, "unknown_1403f5260", fn_1403f5260 as *const (), Throws::Discard), // sub_1403f5260
    Replacement::draft_throws(0x1403f5340, "unknown_1403f5340", fn_1403f5340 as *const (), Throws::Discard), // sub_1403f5340
    Replacement::draft(0x1403f53b0, "unknown_1403f53b0", fn_1403f53b0 as *const ()), // sub_1403f53b0
    Replacement::draft(0x1403f5420, "unknown_1403f5420", fn_1403f5420 as *const ()), // sub_1403f5420
    Replacement::draft(0x1403f54d0, "unknown_1403f54d0", tpl_3611::<0x60, 0x14240ab10> as *const ()), // sub_1403f54d0
    Replacement::draft(0x1403f5500, "unknown_1403f5500", fn_1403f5500 as *const ()), // sub_1403f5500
    Replacement::draft(0x1403f55c0, "unknown_1403f55c0", fn_1403f55c0 as *const ()), // sub_1403f55c0
    Replacement::draft(0x1403f55e0, "unknown_1403f55e0", fn_1403f55e0 as *const ()), // sub_1403f55e0
    Replacement::draft(0x1403f55f0, "unknown_1403f55f0", fn_1403f55f0 as *const ()), // sub_1403f55f0
    Replacement::draft(0x1403f5740, "unknown_1403f5740", fn_1403f5740 as *const ()), // sub_1403f5740
    Replacement::draft(0x1403f5790, "unknown_1403f5790", tpl_1842::<0x1401e7c10, 0x143c90164, 0x142489ee0> as *const ()), // sub_1403f5790
    Replacement::draft(0x1403f58e0, "unknown_1403f58e0", tpl_3857::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x20, 0x1403f7480> as *const ()), // sub_1403f58e0
    Replacement::draft(0x1403f5970, "unknown_1403f5970", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143c8cf5c, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1403f5970
    Replacement::draft(0x1403f5a60, "unknown_1403f5a60", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143c8d478, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1403f5a60
    Replacement::draft(0x1403f5b50, "unknown_1403f5b50", fn_1403f5b50 as *const ()), // sub_1403f5b50
    Replacement::draft(0x1403f5c00, "unknown_1403f5c00", fn_1403f5c00 as *const ()), // sub_1403f5c00
    Replacement::draft(0x1403f5c90, "unknown_1403f5c90", fn_1403f5c90 as *const ()), // sub_1403f5c90
    Replacement::draft(0x1403f5cc0, "unknown_1403f5cc0", tpl_170::<0x1403f6090> as *const ()), // sub_1403f5cc0
    Replacement::draft(0x1403f5cd0, "unknown_1403f5cd0", tpl_170::<0x1403f6200> as *const ()), // sub_1403f5cd0
    Replacement::draft(0x1403f5ce0, "unknown_1403f5ce0", fn_1403f5ce0 as *const ()), // sub_1403f5ce0
    Replacement::draft(0x1403f5e20, "unknown_1403f5e20", fn_1403f5e20 as *const ()), // sub_1403f5e20
    Replacement::draft(0x1403f5fa0, "unknown_1403f5fa0", fn_1403f5fa0 as *const ()), // sub_1403f5fa0
    Replacement::draft(0x1403f5ff0, "unknown_1403f5ff0", fn_1403f5ff0 as *const ()), // sub_1403f5ff0
    Replacement::draft(0x1403f6090, "unknown_1403f6090", fn_1403f6090 as *const ()), // sub_1403f6090
    Replacement::draft(0x1403f6150, "unknown_1403f6150", tpl_808::<0x1403f7710, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1403f6150
    Replacement::draft(0x1403f6200, "unknown_1403f6200", tpl_808::<0x1403f7780, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1403f6200
    Replacement::draft(0x1403f62b0, "unknown_1403f62b0", fn_1403f62b0 as *const ()), // sub_1403f62b0
    Replacement::draft(0x1403f6410, "unknown_1403f6410", tpl_3867::<0x143c8d120, 0x142489ee0, 0x143c8d160, 0x143c8fab8, 0x143c8ec20, 0x143c8e70c> as *const ()), // sub_1403f6410
    Replacement::draft(0x1403f6560, "unknown_1403f6560", tpl_60622::<0x10, 0x18, 0x143c9d1b4, 0x142489ee0, 0xc03, 0x143c9efb0, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6560
    Replacement::draft(0x1403f6660, "unknown_1403f6660", tpl_60622::<0x10, 0x18, 0x143c91064, 0x142489ee0, 0xc03, 0x143c98790, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6660
    Replacement::draft(0x1403f6760, "unknown_1403f6760", tpl_60622::<0x10, 0x18, 0x143c8e710, 0x142489ee0, 0xc03, 0x143c9665c, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6760
    Replacement::draft(0x1403f6860, "unknown_1403f6860", tpl_60622::<0x10, 0x18, 0x143c8e710, 0x142489ee0, 0xc03, 0x143c956a0, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6860
    Replacement::draft(0x1403f6960, "unknown_1403f6960", tpl_60622::<0x10, 0x18, 0x143c8e710, 0x142489ee0, 0xc03, 0x143c8f690, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6960
    Replacement::draft(0x1403f6a60, "unknown_1403f6a60", tpl_60622::<0x10, 0x18, 0x143c8dbcc, 0x142489ee0, 0xc03, 0x143c8ec24, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6a60
    Replacement::draft(0x1403f6b60, "unknown_1403f6b60", tpl_60622::<0x10, 0x18, 0x143c8dbcc, 0x142489ee0, 0xc03, 0x143c8e664, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6b60
    Replacement::draft(0x1403f6c60, "unknown_1403f6c60", tpl_60622::<0x10, 0x18, 0x143c8dbcc, 0x142489ee0, 0xc03, 0x143c8eb98, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6c60
    Replacement::draft(0x1403f6d60, "unknown_1403f6d60", tpl_60622::<0x10, 0x18, 0x143c8dbcc, 0x142489ee0, 0xc03, 0x143c8eba0, 0x20, 0x38, 0x40> as *const ()), // sub_1403f6d60
    Replacement::draft(0x1403f6e60, "unknown_1403f6e60", tpl_63157::<0x143c9ed30, 0x142489ee0, 0x143c9d1d8, 0x143c9afe8> as *const ()), // sub_1403f6e60
    Replacement::draft(0x1403f6fa0, "unknown_1403f6fa0", tpl_63158::<0x143c8f0d0, 0x142489ee0, 0x143c8d160, 0x143c8d87c, 0x143c8e70c> as *const ()), // sub_1403f6fa0
    Replacement::draft(0x1403f7110, "unknown_1403f7110", tpl_60875::<0x143c8d160, 0x142489ee0, 0x143c9b508, 0x143c8fab0, 0x143c9eac0, 0x143c9b4c4, 0x143c8f8d4> as *const ()), // sub_1403f7110
    Replacement::draft(0x1403f7280, "unknown_1403f7280", tpl_60622::<0x10, 0x18, 0x143c95468, 0x142489ee0, 0xc03, 0x143c8d87c, 0x20, 0x38, 0x40> as *const ()), // sub_1403f7280
    Replacement::draft(0x1403f7380, "unknown_1403f7380", tpl_60622::<0x10, 0x18, 0x143c8f6d8, 0x142489ee0, 0xc03, 0x143c8d120, 0x20, 0x38, 0x40> as *const ()), // sub_1403f7380
    Replacement::draft(0x1403f7480, "unknown_1403f7480", fn_1403f7480 as *const ()), // sub_1403f7480
    Replacement::draft(0x1403f7510, "unknown_1403f7510", tpl_60622::<0x10, 0x18, 0x143c8d160, 0x142489ee0, 0xc03, 0x143c90164, 0x20, 0x38, 0x40> as *const ()), // sub_1403f7510
    Replacement::draft(0x1403f7610, "unknown_1403f7610", tpl_60622::<0x10, 0x18, 0x143c90164, 0x142489ee0, 0xc03, 0x143c8d160, 0x20, 0x38, 0x40> as *const ()), // sub_1403f7610
    Replacement::draft(0x1403f7710, "unknown_1403f7710", fn_1403f7710 as *const ()), // sub_1403f7710
    Replacement::draft(0x1403f7780, "unknown_1403f7780", fn_1403f7780 as *const ()), // sub_1403f7780
    Replacement::draft(0x1403f77f0, "unknown_1403f77f0", fn_1403f77f0 as *const ()), // sub_1403f77f0
    Replacement::draft(0x1403f7860, "unknown_1403f7860", tpl_63163::<0x1403c8190, 0x1403c81f0> as *const ()), // sub_1403f7860
    Replacement::draft(0x1403f7bf0, "unknown_1403f7bf0", tpl_3872::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x1403ff090> as *const ()), // sub_1403f7bf0
    Replacement::draft(0x1403f7c90, "unknown_1403f7c90", tpl_3872::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x1403ff100> as *const ()), // sub_1403f7c90
    Replacement::draft_throws(0x1403f7d30, "unknown_1403f7d30", fn_1403f7d30 as *const (), Throws::Discard), // sub_1403f7d30
    Replacement::draft_throws(0x1403f7ec0, "unknown_1403f7ec0", fn_1403f7ec0 as *const (), Throws::Discard), // sub_1403f7ec0
    Replacement::draft_throws(0x1403f8050, "unknown_1403f8050", tpl_3875::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8f8dc, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x143106de0, 0x142116bc0, 0x1403f81ac, 0x1403f81be, 0x1403f81d3, 0x1403f81ac, 0x1403f81be, 0x1403f81d3> as *const (), Throws::Discard), // sub_1403f8050
    Replacement::draft_throws(0x1403f81e0, "unknown_1403f81e0", fn_1403f81e0 as *const (), Throws::Discard), // sub_1403f81e0
    Replacement::draft_throws(0x1403f8370, "unknown_1403f8370", fn_1403f8370 as *const (), Throws::Discard), // sub_1403f8370
    Replacement::draft_throws(0x1403f8500, "unknown_1403f8500", fn_1403f8500 as *const (), Throws::Discard), // sub_1403f8500
    Replacement::draft(0x1403f8650, "unknown_1403f8650", tpl_3879::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143c9d198, 0x180a8, 0x18090, 0x142489ee0, 0x14040f7a0, 0x1426467e8, 0x1403f8726> as *const ()), // sub_1403f8650
    Replacement::draft_throws(0x1403f8740, "unknown_1403f8740", tpl_3880::<0x78, 0x143c9da5c, 0x580e0, 0x143110698, 0x142116b30, 0x1403f8776> as *const (), Throws::Discard), // sub_1403f8740
    Replacement::draft(0x1403f8780, "unknown_1403f8780", fn_1403f8780 as *const ()), // sub_1403f8780
    Replacement::draft(0x1403f88a0, "unknown_1403f88a0", fn_1403f88a0 as *const ()), // sub_1403f88a0
    Replacement::draft_throws(0x1403f89c0, "unknown_1403f89c0", tpl_3883::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c91a84, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x143109850, 0x142116bc0, 0x1403f8b1b, 0x1403f8b2d, 0x1403f8b42, 0x1403f8b1b, 0x1403f8b2d, 0x1403f8b42> as *const (), Throws::Discard), // sub_1403f89c0
    Replacement::draft_throws(0x1403f8b50, "unknown_1403f8b50", tpl_3884::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c91b28, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x1431099c8, 0x142116bc0, 0x1403f8caf, 0x1403f8cc1, 0x1403f8cd6, 0x1403f8caf, 0x1403f8cc1, 0x1403f8cd6> as *const (), Throws::Discard), // sub_1403f8b50
    Replacement::draft_throws(0x1403f8ce0, "unknown_1403f8ce0", fn_1403f8ce0 as *const (), Throws::Discard), // sub_1403f8ce0
    Replacement::draft(0x1403f8e70, "unknown_1403f8e70", tpl_3886::<0x14310a1e8, 0x8> as *const ()), // sub_1403f8e70
    Replacement::draft_throws(0x1403f8e90, "unknown_1403f8e90", tpl_3887::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c96de4, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310a1b8, 0x142116bc0, 0x1403f8ff2, 0x1403f9004, 0x1403f9019, 0x1403f8ff2, 0x1403f9004, 0x1403f9019> as *const (), Throws::Discard), // sub_1403f8e90
    Replacement::draft(0x1403f9020, "unknown_1403f9020", fn_1403f9020 as *const ()), // sub_1403f9020
    Replacement::draft_throws(0x1403f90c0, "unknown_1403f90c0", tpl_3889::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c98a54, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310a320, 0x142116bc0, 0x1403f921c, 0x1403f922e, 0x1403f9243, 0x1403f921c, 0x1403f922e, 0x1403f9243> as *const (), Throws::Discard), // sub_1403f90c0
    Replacement::draft(0x1403f9250, "unknown_1403f9250", fn_1403f9250 as *const ()), // sub_1403f9250
    Replacement::draft(0x1403f9360, "unknown_1403f9360", tpl_3872::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x140400730> as *const ()), // sub_1403f9360
    Replacement::draft(0x1403f9400, "unknown_1403f9400", tpl_3879::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143c8efb0, 0x180a8, 0x18090, 0x142489ee0, 0x14040f830, 0x1426467e8, 0x1403f94d6> as *const ()), // sub_1403f9400
    Replacement::draft(0x1403f94f0, "unknown_1403f94f0", fn_1403f94f0 as *const ()), // sub_1403f94f0
    Replacement::draft(0x1403f9660, "unknown_1403f9660", fn_1403f9660 as *const ()), // sub_1403f9660
    Replacement::draft(0x1403f97b0, "unknown_1403f97b0", fn_1403f97b0 as *const ()), // sub_1403f97b0
    Replacement::draft(0x1403f9900, "unknown_1403f9900", fn_1403f9900 as *const ()), // sub_1403f9900
    Replacement::draft(0x1403f9a30, "unknown_1403f9a30", tpl_3895::<0x143c8df88, 0x580e0> as *const ()), // sub_1403f9a30
    Replacement::draft(0x1403f9a50, "unknown_1403f9a50", tpl_3895::<0x143c93214, 0x580e0> as *const ()), // sub_1403f9a50
    Replacement::draft(0x1403f9a70, "unknown_1403f9a70", fn_1403f9a70 as *const ()), // sub_1403f9a70
    Replacement::draft(0x1403f9b90, "unknown_1403f9b90", tpl_3895::<0x143c90e64, 0x580e0> as *const ()), // sub_1403f9b90
    Replacement::draft_throws(0x1403f9bb0, "unknown_1403f9bb0", tpl_3884::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8eb90, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x143109db8, 0x142116bc0, 0x1403f9d0f, 0x1403f9d21, 0x1403f9d36, 0x1403f9d0f, 0x1403f9d21, 0x1403f9d36> as *const (), Throws::Discard), // sub_1403f9bb0
    Replacement::draft_throws(0x1403f9d40, "unknown_1403f9d40", tpl_3897::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c90198, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x143109d88, 0x142116bc0, 0x1403f9ea0, 0x1403f9eb2, 0x1403f9ec7, 0x1403f9ea0, 0x1403f9eb2, 0x1403f9ec7> as *const (), Throws::Discard), // sub_1403f9d40
    Replacement::draft_throws(0x1403f9ed0, "unknown_1403f9ed0", tpl_3875::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c977b4, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310a018, 0x142116bc0, 0x1403fa02c, 0x1403fa03e, 0x1403fa053, 0x1403fa02c, 0x1403fa03e, 0x1403fa053> as *const (), Throws::Discard), // sub_1403f9ed0
    Replacement::draft_throws(0x1403fa060, "unknown_1403fa060", fn_1403fa060 as *const (), Throws::Discard), // sub_1403fa060
    Replacement::draft_throws(0x1403fa1f0, "unknown_1403fa1f0", fn_1403fa1f0 as *const (), Throws::Discard), // sub_1403fa1f0
    Replacement::draft_throws(0x1403fa380, "unknown_1403fa380", tpl_3887::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c91f48, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x143109f78, 0x142116bc0, 0x1403fa4e2, 0x1403fa4f4, 0x1403fa509, 0x1403fa4e2, 0x1403fa4f4, 0x1403fa509> as *const (), Throws::Discard), // sub_1403fa380
    Replacement::draft_throws(0x1403fa510, "unknown_1403fa510", tpl_3884::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8eb90, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x143109f48, 0x142116bc0, 0x1403fa66f, 0x1403fa681, 0x1403fa696, 0x1403fa66f, 0x1403fa681, 0x1403fa696> as *const (), Throws::Discard), // sub_1403fa510
    Replacement::draft_throws(0x1403fa6a0, "unknown_1403fa6a0", tpl_3889::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c9d6c4, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310a0e8, 0x142116bc0, 0x1403fa7fc, 0x1403fa80e, 0x1403fa823, 0x1403fa7fc, 0x1403fa80e, 0x1403fa823> as *const (), Throws::Discard), // sub_1403fa6a0
    Replacement::draft_throws(0x1403fa830, "unknown_1403fa830", fn_1403fa830 as *const (), Throws::Discard), // sub_1403fa830
    Replacement::draft(0x1403fa9c0, "unknown_1403fa9c0", tpl_63182::<0x143c9d154, 0x1401e7c10, 0x143c8d120, 0x142489ee0, 0x143c8dbc4> as *const ()), // sub_1403fa9c0
    Replacement::draft_throws(0x1403fabb0, "unknown_1403fabb0", fn_1403fabb0 as *const (), Throws::Discard), // sub_1403fabb0
    Replacement::draft_throws(0x1403face0, "unknown_1403face0", fn_1403face0 as *const (), Throws::Discard), // sub_1403face0
    Replacement::draft_throws(0x1403fad50, "unknown_1403fad50", fn_1403fad50 as *const (), Throws::Discard), // sub_1403fad50
    Replacement::draft(0x1403fae40, "unknown_1403fae40", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9f3b0, 0x1426467e8, 0x1404057b0, 0x1403faf4d> as *const ()), // sub_1403fae40
    Replacement::draft_throws(0x1403fafe0, "unknown_1403fafe0", tpl_3883::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c914b0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310abd0, 0x142116bc0, 0x1403fb13b, 0x1403fb14d, 0x1403fb162, 0x1403fb13b, 0x1403fb14d, 0x1403fb162> as *const (), Throws::Discard), // sub_1403fafe0
    Replacement::draft_throws(0x1403fb170, "unknown_1403fb170", tpl_3884::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8e70c, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310ab70, 0x142116bc0, 0x1403fb2cf, 0x1403fb2e1, 0x1403fb2f6, 0x1403fb2cf, 0x1403fb2e1, 0x1403fb2f6> as *const (), Throws::Discard), // sub_1403fb170
    Replacement::draft(0x1403fb300, "unknown_1403fb300", tpl_3867::<0x143c93b20, 0x142489ee0, 0x143c8d120, 0x143c8e70c, 0x143c9db0c, 0x143c9ecdc> as *const ()), // sub_1403fb300
    Replacement::draft_throws(0x1403fb450, "unknown_1403fb450", fn_1403fb450 as *const (), Throws::Discard), // sub_1403fb450
    Replacement::draft_throws(0x1403fb5e0, "unknown_1403fb5e0", fn_1403fb5e0 as *const (), Throws::Discard), // sub_1403fb5e0
    Replacement::draft_throws(0x1403fb770, "unknown_1403fb770", fn_1403fb770 as *const (), Throws::Discard), // sub_1403fb770
    Replacement::draft(0x1403fb900, "unknown_1403fb900", fn_1403fb900 as *const ()), // sub_1403fb900
    Replacement::draft_throws(0x1403fba40, "unknown_1403fba40", fn_1403fba40 as *const (), Throws::Caught), // sub_1403fba40
    Replacement::draft(0x1403fbb20, "unknown_1403fbb20", fn_1403fbb20 as *const ()), // sub_1403fbb20
    Replacement::draft(0x1403fbd90, "unknown_1403fbd90", fn_1403fbd90 as *const ()), // sub_1403fbd90
    Replacement::draft_throws(0x1403fbe90, "unknown_1403fbe90", fn_1403fbe90 as *const (), Throws::Discard), // sub_1403fbe90
    Replacement::draft_throws(0x1403fbf10, "unknown_1403fbf10", fn_1403fbf10 as *const (), Throws::Discard), // sub_1403fbf10
    Replacement::draft_throws(0x1403fc0a0, "unknown_1403fc0a0", tpl_3889::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8eba0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310c330, 0x142116bc0, 0x1403fc1fc, 0x1403fc20e, 0x1403fc223, 0x1403fc1fc, 0x1403fc20e, 0x1403fc223> as *const (), Throws::Discard), // sub_1403fc0a0
    Replacement::draft_throws(0x1403fc230, "unknown_1403fc230", fn_1403fc230 as *const (), Throws::Discard), // sub_1403fc230
    Replacement::draft_throws(0x1403fc3c0, "unknown_1403fc3c0", fn_1403fc3c0 as *const (), Throws::Discard), // sub_1403fc3c0
    Replacement::draft_throws(0x1403fc550, "unknown_1403fc550", tpl_3897::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8dbd0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310c300, 0x142116bc0, 0x1403fc6b0, 0x1403fc6c2, 0x1403fc6d7, 0x1403fc6b0, 0x1403fc6c2, 0x1403fc6d7> as *const (), Throws::Discard), // sub_1403fc550
    Replacement::draft_throws(0x1403fc6e0, "unknown_1403fc6e0", tpl_3884::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c98fc0, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310c2d8, 0x142116bc0, 0x1403fc83f, 0x1403fc851, 0x1403fc866, 0x1403fc83f, 0x1403fc851, 0x1403fc866> as *const (), Throws::Discard), // sub_1403fc6e0
    Replacement::draft_throws(0x1403fc870, "unknown_1403fc870", fn_1403fc870 as *const (), Throws::Discard), // sub_1403fc870
    Replacement::draft_throws(0x1403fc8b0, "unknown_1403fc8b0", fn_1403fc8b0 as *const (), Throws::Discard), // sub_1403fc8b0
    Replacement::draft_throws(0x1403fcae0, "unknown_1403fcae0", fn_1403fcae0 as *const (), Throws::Discard), // sub_1403fcae0
    Replacement::draft_throws(0x1403fccd0, "unknown_1403fccd0", fn_1403fccd0 as *const (), Throws::Discard), // sub_1403fccd0
    Replacement::draft_throws(0x1403fcec0, "unknown_1403fcec0", fn_1403fcec0 as *const (), Throws::Discard), // sub_1403fcec0
    Replacement::draft_throws(0x1403fd0b0, "unknown_1403fd0b0", fn_1403fd0b0 as *const (), Throws::Discard), // sub_1403fd0b0
    Replacement::draft_throws(0x1403fd290, "unknown_1403fd290", fn_1403fd290 as *const (), Throws::Discard), // sub_1403fd290
    Replacement::draft_throws(0x1403fd460, "unknown_1403fd460", fn_1403fd460 as *const (), Throws::Discard), // sub_1403fd460
    Replacement::draft_throws(0x1403fd640, "unknown_1403fd640", fn_1403fd640 as *const (), Throws::Discard), // sub_1403fd640
    Replacement::draft_throws(0x1403fd820, "unknown_1403fd820", fn_1403fd820 as *const (), Throws::Discard), // sub_1403fd820
    Replacement::draft_throws(0x1403fda10, "unknown_1403fda10", fn_1403fda10 as *const (), Throws::Caught), // sub_1403fda10
    Replacement::draft(0x1403fdb50, "unknown_1403fdb50", fn_1403fdb50 as *const ()), // sub_1403fdb50
    Replacement::draft(0x1403fdb70, "unknown_1403fdb70", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d8c0, 0x1426467e8, 0x143c8ec68, 0x142489ee0, 0x143c90164, 0x1403fdc63> as *const ()), // sub_1403fdb70
    Replacement::draft(0x1403fdde0, "unknown_1403fdde0", fn_1403fdde0 as *const ()), // sub_1403fdde0
    Replacement::draft(0x1403fdf30, "unknown_1403fdf30", tpl_63210::<0x30, 0x143c9d2f0, 0x180a8, 0x18090, 0x142489ee0, 0x180ac, 0x18098, 0x40, 0x1426467e8, 0x1403fdfeb> as *const ()), // sub_1403fdf30
    Replacement::draft(0x1403fdff0, "unknown_1403fdff0", fn_1403fdff0 as *const ()), // sub_1403fdff0
    Replacement::draft(0x1403fe0b0, "unknown_1403fe0b0", fn_1403fe0b0 as *const ()), // sub_1403fe0b0
    Replacement::draft(0x1403fe180, "unknown_1403fe180", fn_1403fe180 as *const ()), // sub_1403fe180
    Replacement::draft_throws(0x1403fe1e0, "unknown_1403fe1e0", tpl_3883::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c9d61c, 0x142489ee0, 0x1426467e8, 0x1421158a0, 0x14310d818, 0x142116bc0, 0x1403fe33b, 0x1403fe34d, 0x1403fe362, 0x1403fe33b, 0x1403fe34d, 0x1403fe362> as *const (), Throws::Discard), // sub_1403fe1e0
    Replacement::draft(0x1403fe370, "unknown_1403fe370", fn_1403fe370 as *const ()), // sub_1403fe370
    Replacement::draft(0x1403fe440, "unknown_1403fe440", fn_1403fe440 as *const ()), // sub_1403fe440
    Replacement::draft(0x1403fe4a0, "unknown_1403fe4a0", tpl_1842::<0x1401e7c10, 0x143c8efb0, 0x142489ee0> as *const ()), // sub_1403fe4a0
    Replacement::draft(0x1403fe5f0, "unknown_1403fe5f0", tpl_62612::<0x1403fe880> as *const ()), // sub_1403fe5f0
    Replacement::draft(0x1403fe710, "unknown_1403fe710", fn_1403fe710 as *const ()), // sub_1403fe710
    Replacement::draft(0x1403fe7e0, "unknown_1403fe7e0", fn_1403fe7e0 as *const ()), // sub_1403fe7e0
    Replacement::draft(0x1403fe880, "unknown_1403fe880", tpl_3049::<0x143c8d120, 0x142489ee0, 0x143c97150, 0x143c8d478, 0x143c919a8, 0x143c8e840, 0x143c8de68, 0x143c8dbc4, 0x143c97ad0> as *const ()), // sub_1403fe880
    Replacement::draft(0x1403fea40, "unknown_1403fea40", fn_1403fea40 as *const ()), // sub_1403fea40
    Replacement::draft(0x1403feb00, "unknown_1403feb00", tpl_1791::<0xc, 0x8, 0x140266020, 0x140407390, 0x1403feb80> as *const ()), // sub_1403feb00
    Replacement::draft(0x1403feb80, "unknown_1403feb80", tpl_1708::<0x8, 0x14040a300, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1403feb80
    Replacement::draft(0x1403fecc0, "unknown_1403fecc0", tpl_60953::<0x1426467e8, 0x143c8f6e4, 0x142489ee0, 0x1403fed3c> as *const ()), // sub_1403fecc0
    Replacement::draft(0x1403fee70, "unknown_1403fee70", tpl_60953::<0x1426467e8, 0x143c8fa04, 0x142489ee0, 0x1403feeec> as *const ()), // sub_1403fee70
    Replacement::draft(0x1403ff020, "unknown_1403ff020", tpl_3936::<0x30, 0x143c8ec20, 0x180a8, 0x18090, 0x142489ee0, 0x14040f560, 0x1426467e8, 0x1403ff086> as *const ()), // sub_1403ff020
    Replacement::draft(0x1403ff090, "unknown_1403ff090", tpl_3936::<0x30, 0x143c8d120, 0x180a8, 0x18090, 0x142489ee0, 0x14040f5f0, 0x1426467e8, 0x1403ff0f6> as *const ()), // sub_1403ff090
    Replacement::draft(0x1403ff100, "unknown_1403ff100", tpl_3936::<0x30, 0x143c8f4d0, 0x180a8, 0x18090, 0x142489ee0, 0x14040f710, 0x1426467e8, 0x1403ff166> as *const ()), // sub_1403ff100
    Replacement::draft(0x1403ff170, "unknown_1403ff170", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9dba0, 0x1426467e8, 0x14040a380, 0x1403ff27d> as *const ()), // sub_1403ff170
    Replacement::draft(0x1403ff310, "unknown_1403ff310", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c8d7b8, 0x1426467e8, 0x143c8d68c, 0x142489ee0, 0x1403ff407> as *const ()), // sub_1403ff310
    Replacement::draft(0x1403ff520, "unknown_1403ff520", tpl_63219::<0x143c8d08c, 0x1401fa730, 0x143c9f308, 0x140226fa0, 0x143c8d68c, 0x142489ee0, 0x143c8d708, 0x1426467e8, 0x1403ff721> as *const ()), // sub_1403ff520
    Replacement::draft(0x1403ff740, "unknown_1403ff740", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d190, 0x1426467e8, 0x143c8d478, 0x142489ee0, 0x1403ff837> as *const ()), // sub_1403ff740
    Replacement::draft(0x1403ff950, "unknown_1403ff950", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9e624, 0x1426467e8, 0x143c8d518, 0x142489ee0, 0x1403ffa47> as *const ()), // sub_1403ff950
    Replacement::draft(0x1403ffb60, "unknown_1403ffb60", tpl_3937::<0x143c8d08c, 0x1401fa730, 0x143c9e15c, 0x140226fa0, 0x143c8ec6c, 0x142489ee0, 0x1426467e8, 0x1403ffd02> as *const ()), // sub_1403ffb60
    Replacement::draft(0x1403ffd20, "unknown_1403ffd20", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9f204, 0x1426467e8, 0x14040a4f0, 0x1403ffe2d> as *const ()), // sub_1403ffd20
    Replacement::draft(0x1403ffec0, "unknown_1403ffec0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9dc70, 0x1426467e8, 0x143c8d68c, 0x142489ee0, 0x1403fffb7> as *const ()), // sub_1403ffec0
    Replacement::draft(0x1404000d0, "unknown_1404000d0", tpl_1095::<0x143c9f0c0, 0x14020e8f0> as *const ()), // sub_1404000d0
    Replacement::draft(0x1404002b0, "unknown_1404002b0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9eb94, 0x1426467e8, 0x143c8d68c, 0x142489ee0, 0x1404003a7> as *const ()), // sub_1404002b0
    Replacement::draft(0x1404004c0, "unknown_1404004c0", fn_1404004c0 as *const ()), // sub_1404004c0
    Replacement::draft(0x140400730, "unknown_140400730", tpl_3938::<0x30, 0x143c933f0, 0x180a8, 0x18090, 0x142489ee0, 0x14040a770, 0x1426467e8, 0x14040079b> as *const ()), // sub_140400730
    Replacement::draft(0x1404007a0, "unknown_1404007a0", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d230, 0x1426467e8, 0x143c9a330, 0x142489ee0, 0x143c9a320, 0x140400893> as *const ()), // sub_1404007a0
    Replacement::draft(0x140400a10, "unknown_140400a10", fn_140400a10 as *const ()), // sub_140400a10
    Replacement::draft(0x140400c40, "unknown_140400c40", tpl_3421::<0x143c8d08c, 0x1401fa730, 0x143c9d818, 0x1426467e8, 0x14040aa90, 0x140400d4f> as *const ()), // sub_140400c40
    Replacement::draft(0x140400df0, "unknown_140400df0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9f174, 0x1426467e8, 0x143c8dbc4, 0x142489ee0, 0x140400ee7> as *const ()), // sub_140400df0
    Replacement::draft(0x140401000, "unknown_140401000", tpl_3937::<0x143c8d08c, 0x1401fa730, 0x143c8df7c, 0x140226fa0, 0x143c8dbc4, 0x142489ee0, 0x1426467e8, 0x1404011a2> as *const ()), // sub_140401000
    Replacement::draft(0x1404011c0, "unknown_1404011c0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d1a4, 0x1426467e8, 0x143c93310, 0x142489ee0, 0x1404012b7> as *const ()), // sub_1404011c0
    Replacement::draft(0x1404013d0, "unknown_1404013d0", tpl_3810::<0x143c8d08c, 0x1401fa730, 0x143c9db08, 0x1426467e8, 0x14040ada0, 0x1404014d0> as *const ()), // sub_1404013d0
    Replacement::draft(0x140401600, "unknown_140401600", fn_140401600 as *const ()), // sub_140401600
    Replacement::draft(0x140401840, "unknown_140401840", fn_140401840 as *const ()), // sub_140401840
    Replacement::draft(0x140401a70, "unknown_140401a70", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d168, 0x1426467e8, 0x143c8e9b0, 0x142489ee0, 0x140401b67> as *const ()), // sub_140401a70
    Replacement::draft(0x140401c80, "unknown_140401c80", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9f0c8, 0x1426467e8, 0x14040b220, 0x140401d8d> as *const ()), // sub_140401c80
    Replacement::draft(0x140401e20, "unknown_140401e20", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d45c, 0x1426467e8, 0x143c9ea00, 0x142489ee0, 0x140401f17> as *const ()), // sub_140401e20
    Replacement::draft(0x140402030, "unknown_140402030", tpl_3937::<0x143c8d08c, 0x1401fa730, 0x143c9d170, 0x140226fa0, 0x143c9ea00, 0x142489ee0, 0x1426467e8, 0x1404021d2> as *const ()), // sub_140402030
    Replacement::draft(0x1404021f0, "unknown_1404021f0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d1dc, 0x1426467e8, 0x143c9d6b8, 0x142489ee0, 0x1404022e7> as *const ()), // sub_1404021f0
    Replacement::draft(0x140402400, "unknown_140402400", tpl_3942::<0x30, 0x143c9d6b8, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x140402450> as *const ()), // sub_140402400
    Replacement::draft(0x140402460, "unknown_140402460", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c93788, 0x1426467e8, 0x143c8efb0, 0x142489ee0, 0x140402557> as *const ()), // sub_140402460
    Replacement::draft(0x140402670, "unknown_140402670", tpl_3943::<0x30, 0x143c97e40, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1404026f3> as *const ()), // sub_140402670
    Replacement::draft(0x140402700, "unknown_140402700", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9d1a0, 0x1426467e8, 0x14040b390, 0x14040280d> as *const ()), // sub_140402700
    Replacement::draft(0x1404028a0, "unknown_1404028a0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9e37c, 0x1426467e8, 0x143c92c98, 0x142489ee0, 0x140402997> as *const ()), // sub_1404028a0
    Replacement::draft(0x140402ab0, "unknown_140402ab0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d1d0, 0x1426467e8, 0x143c92c98, 0x142489ee0, 0x140402ba7> as *const ()), // sub_140402ab0
    Replacement::draft(0x140402cc0, "unknown_140402cc0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9e49c, 0x1426467e8, 0x143c92ee8, 0x142489ee0, 0x140402db7> as *const ()), // sub_140402cc0
    Replacement::draft(0x140402ed0, "unknown_140402ed0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9ee28, 0x1426467e8, 0x143c92ee8, 0x142489ee0, 0x140402fc7> as *const ()), // sub_140402ed0
    Replacement::draft(0x1404030e0, "unknown_1404030e0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9f1fc, 0x1426467e8, 0x143c9d19c, 0x142489ee0, 0x1404031d7> as *const ()), // sub_1404030e0
    Replacement::draft(0x1404032f0, "unknown_1404032f0", tpl_3810::<0x143c8d08c, 0x1401fa730, 0x143c9e884, 0x1426467e8, 0x14040b500, 0x1404033f0> as *const ()), // sub_1404032f0
    Replacement::draft(0x140403520, "unknown_140403520", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9e4f4, 0x1426467e8, 0x143c92c98, 0x142489ee0, 0x143c937c8, 0x140403613> as *const ()), // sub_140403520
    Replacement::draft(0x140403790, "unknown_140403790", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9e4a0, 0x1426467e8, 0x143c937c8, 0x142489ee0, 0x140403887> as *const ()), // sub_140403790
    Replacement::draft(0x1404039a0, "unknown_1404039a0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d204, 0x1426467e8, 0x143c92a7c, 0x142489ee0, 0x140403a97> as *const ()), // sub_1404039a0
    Replacement::draft(0x140403bb0, "unknown_140403bb0", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d674, 0x1426467e8, 0x143c92a7c, 0x142489ee0, 0x143c937c8, 0x140403ca3> as *const ()), // sub_140403bb0
    Replacement::draft(0x140403e20, "unknown_140403e20", tpl_3421::<0x143c8d08c, 0x1401fa730, 0x143c9f004, 0x1426467e8, 0x14040b650, 0x140403f2f> as *const ()), // sub_140403e20
    Replacement::draft(0x140403fd0, "unknown_140403fd0", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d44c, 0x1426467e8, 0x143c937c8, 0x142489ee0, 0x143c9d840, 0x1404040c3> as *const ()), // sub_140403fd0
    Replacement::draft(0x140404240, "unknown_140404240", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9e660, 0x1426467e8, 0x14040b7f0, 0x14040434d> as *const ()), // sub_140404240
    Replacement::draft(0x1404043e0, "unknown_1404043e0", fn_1404043e0 as *const ()), // sub_1404043e0
    Replacement::draft(0x1404045a0, "unknown_1404045a0", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d178, 0x1426467e8, 0x143c93228, 0x142489ee0, 0x143c8d87c, 0x140404693> as *const ()), // sub_1404045a0
    Replacement::draft(0x140404810, "unknown_140404810", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d16c, 0x1426467e8, 0x143c93228, 0x142489ee0, 0x140404907> as *const ()), // sub_140404810
    Replacement::draft(0x140404a20, "unknown_140404a20", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d8c8, 0x1426467e8, 0x143c93228, 0x142489ee0, 0x140404b17> as *const ()), // sub_140404a20
    Replacement::draft(0x140404c30, "unknown_140404c30", fn_140404c30 as *const ()), // sub_140404c30
    Replacement::draft(0x140404e00, "unknown_140404e00", fn_140404e00 as *const ()), // sub_140404e00
    Replacement::draft(0x140405060, "unknown_140405060", fn_140405060 as *const ()), // sub_140405060
    Replacement::draft(0x140405290, "unknown_140405290", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d848, 0x1426467e8, 0x143c93228, 0x142489ee0, 0x143c9e55c, 0x140405383> as *const ()), // sub_140405290
    Replacement::draft(0x140405500, "unknown_140405500", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9eef8, 0x1426467e8, 0x14040c630, 0x14040560d> as *const ()), // sub_140405500
    Replacement::draft(0x1404056a0, "unknown_1404056a0", tpl_3936::<0x30, 0x143c8f0d0, 0x180a8, 0x18090, 0x142489ee0, 0x1402372c0, 0x1426467e8, 0x140405706> as *const ()), // sub_1404056a0
    Replacement::draft(0x140405710, "unknown_140405710", tpl_3872::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x14040c7a0> as *const ()), // sub_140405710
    Replacement::draft(0x1404057b0, "unknown_1404057b0", tpl_61019::<0x143c96b14, 0x142489ee0, 0x143c96b20, 0x143c8ec68, 0x1426467e8, 0x140405913> as *const ()), // sub_1404057b0
    Replacement::draft(0x140405920, "unknown_140405920", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9e7d4, 0x1426467e8, 0x143c956a0, 0x142489ee0, 0x140405a17> as *const ()), // sub_140405920
    Replacement::draft(0x140405b30, "unknown_140405b30", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9d220, 0x1426467e8, 0x143c8ea68, 0x142489ee0, 0x143c95048, 0x140405c23> as *const ()), // sub_140405b30
    Replacement::draft(0x140405da0, "unknown_140405da0", tpl_1097::<0x143c99fb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140405da0
    Replacement::draft(0x1404060d0, "unknown_1404060d0", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9d670, 0x1426467e8, 0x14040c810, 0x1404061dd> as *const ()), // sub_1404060d0
    Replacement::draft(0x140406270, "unknown_140406270", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9e880, 0x1426467e8, 0x143c9db0c, 0x142489ee0, 0x143c9ecdc, 0x140406363> as *const ()), // sub_140406270
    Replacement::draft(0x1404064e0, "unknown_1404064e0", fn_1404064e0 as *const ()), // sub_1404064e0
    Replacement::draft(0x140406740, "unknown_140406740", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9d830, 0x1426467e8, 0x143c8e70c, 0x142489ee0, 0x140406837> as *const ()), // sub_140406740
    Replacement::draft(0x140406950, "unknown_140406950", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9dac8, 0x1426467e8, 0x143c9779c, 0x142489ee0, 0x140406a47> as *const ()), // sub_140406950
    Replacement::draft(0x140406b60, "unknown_140406b60", tpl_3810::<0x143c8d08c, 0x1401fa730, 0x143c9ea04, 0x1426467e8, 0x14040c980, 0x140406c60> as *const ()), // sub_140406b60
    Replacement::draft(0x140406d90, "unknown_140406d90", tpl_3879::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143c8f7a0, 0x180a8, 0x18090, 0x142489ee0, 0x140411bb0, 0x1426467e8, 0x140406e66> as *const ()), // sub_140406d90
    Replacement::draft(0x140406e80, "unknown_140406e80", tpl_3879::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143c8f6dc, 0x180a8, 0x18090, 0x142489ee0, 0x140411c40, 0x1426467e8, 0x140406f56> as *const ()), // sub_140406e80
    Replacement::draft(0x140406f70, "unknown_140406f70", fn_140406f70 as *const ()), // sub_140406f70
    Replacement::draft(0x140407150, "unknown_140407150", fn_140407150 as *const ()), // sub_140407150
    Replacement::draft(0x140407390, "unknown_140407390", fn_140407390 as *const ()), // sub_140407390
    Replacement::draft(0x140407400, "unknown_140407400", tpl_63219::<0x143c8d08c, 0x1401fa730, 0x143c9e4a4, 0x140226fa0, 0x143c8f7a0, 0x142489ee0, 0x143c8d120, 0x1426467e8, 0x140407601> as *const ()), // sub_140407400
    Replacement::draft(0x140407620, "unknown_140407620", fn_140407620 as *const ()), // sub_140407620
    Replacement::draft(0x140407940, "unknown_140407940", fn_140407940 as *const ()), // sub_140407940
    Replacement::draft(0x140407b70, "unknown_140407b70", fn_140407b70 as *const ()), // sub_140407b70
    Replacement::draft(0x140407db0, "unknown_140407db0", fn_140407db0 as *const ()), // sub_140407db0
    Replacement::draft(0x140407fa0, "unknown_140407fa0", fn_140407fa0 as *const ()), // sub_140407fa0
    Replacement::draft(0x1404081e0, "unknown_1404081e0", fn_1404081e0 as *const ()), // sub_1404081e0
    Replacement::draft(0x1404083d0, "unknown_1404083d0", fn_1404083d0 as *const ()), // sub_1404083d0
    Replacement::draft(0x1404085d0, "unknown_1404085d0", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143c9f0d0, 0x1426467e8, 0x14040eb60, 0x1404086dd> as *const ()), // sub_1404085d0
    Replacement::draft(0x140408770, "unknown_140408770", fn_140408770 as *const ()), // sub_140408770
    Replacement::draft(0x140408a00, "unknown_140408a00", fn_140408a00 as *const ()), // sub_140408a00
    Replacement::draft(0x140408c30, "unknown_140408c30", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9dc78, 0x1426467e8, 0x143c9dba4, 0x142489ee0, 0x140408d27> as *const ()), // sub_140408c30
    Replacement::draft(0x140408e40, "unknown_140408e40", fn_140408e40 as *const ()), // sub_140408e40
    Replacement::draft(0x140409020, "unknown_140409020", fn_140409020 as *const ()), // sub_140409020
    Replacement::draft(0x140409230, "unknown_140409230", fn_140409230 as *const ()), // sub_140409230
    Replacement::draft(0x140409430, "unknown_140409430", fn_140409430 as *const ()), // sub_140409430
    Replacement::draft(0x1404094a0, "unknown_1404094a0", tpl_3964::<0x140409f80, 0x1403d1410, 0x14040f020, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1404094a0
    Replacement::draft(0x140409660, "unknown_140409660", tpl_1189::<0x14021cf00, 0x14240a730, 0x14021e830, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_140409660
    Replacement::draft(0x140409840, "unknown_140409840", fn_140409840 as *const ()), // sub_140409840
    Replacement::draft(0x1404098e0, "unknown_1404098e0", fn_1404098e0 as *const ()), // sub_1404098e0
    Replacement::draft(0x140409920, "std_140409920", tpl_1::<0x143887cb0> as *const ()), // std::_Func_impl_no_alloc<void (*)(lua_State*,const r::StringHash<unsigned int>&,r::VectorOld<unsigned __int64*,r::Scratc
    Replacement::draft(0x140409930, "std_140409930", fn_140409930 as *const ()), // std::_Func_impl_no_alloc<void (*)(lua_State*,const r::StringHash<unsigned int>&,r::VectorOld<unsigned __int64*,r::Scratc
    Replacement::draft(0x140409950, "std_140409950", tpl_550::<0x14311f238, 0x8> as *const ()), // std::_Func_impl_no_alloc<void (*)(lua_State*,const r::StringHash<unsigned int>&,r::VectorOld<unsigned __int64*,r::Scratc
    Replacement::draft(0x140409970, "std_140409970", tpl_1::<0x143887c80> as *const ()), // std::_Func_impl_no_alloc<void (*)(lua_State*,void*),void,lua_State*,void*>::std::_Func_impl_no_alloc<void (*)(lua_State*
    Replacement::draft(0x140409980, "std_140409980", fn_140409980 as *const ()), // std::_Func_impl_no_alloc<void (*)(lua_State*,void*),void,lua_State*,void*>::std::_Func_impl_no_alloc<void (*)(lua_State*
    Replacement::draft(0x140409990, "std_140409990", tpl_550::<0x14311efe8, 0x8> as *const ()), // std::_Func_impl_no_alloc<void (*)(lua_State*,void*),void,lua_State*,void*>::std::_Func_impl_no_alloc<void (*)(lua_State*
    Replacement::draft(0x1404099b0, "unknown_1404099b0", fn_1404099b0 as *const ()), // sub_1404099b0
    Replacement::draft(0x140409c00, "unknown_140409c00", fn_140409c00 as *const ()), // sub_140409c00
    Replacement::draft(0x140409e40, "unknown_140409e40", fn_140409e40 as *const ()), // sub_140409e40
    Replacement::draft(0x14040a070, "unknown_14040a070", fn_14040a070 as *const ()), // sub_14040a070
    Replacement::draft(0x14040a1a0, "unknown_14040a1a0", fn_14040a1a0 as *const ()), // sub_14040a1a0
    Replacement::draft(0x14040a300, "unknown_14040a300", fn_14040a300 as *const ()), // sub_14040a300
    Replacement::draft(0x14040a380, "unknown_14040a380", tpl_61019::<0x143c8d68c, 0x142489ee0, 0x143c8d708, 0x143c8d5b0, 0x1426467e8, 0x14040a4e3> as *const ()), // sub_14040a380
    Replacement::draft(0x14040a4f0, "unknown_14040a4f0", tpl_61019::<0x143c8f7a0, 0x142489ee0, 0x143c8d120, 0x143c8f6dc, 0x1426467e8, 0x14040a653> as *const ()), // sub_14040a4f0
    Replacement::draft(0x14040a660, "unknown_14040a660", fn_14040a660 as *const ()), // sub_14040a660
    Replacement::draft(0x14040a770, "unknown_14040a770", tpl_3351::<0x30, 0x143c933f0, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14040a7f2> as *const ()), // sub_14040a770
    Replacement::draft(0x14040a800, "unknown_14040a800", fn_14040a800 as *const ()), // sub_14040a800
    Replacement::draft(0x14040a890, "unknown_14040a890", fn_14040a890 as *const ()), // sub_14040a890
    Replacement::draft(0x14040a8d0, "unknown_14040a8d0", fn_14040a8d0 as *const ()), // sub_14040a8d0
    Replacement::draft(0x14040aa30, "unknown_14040aa30", fn_14040aa30 as *const ()), // sub_14040aa30
    Replacement::draft(0x14040aa90, "unknown_14040aa90", tpl_61197::<0x143c8dbc4, 0x142489ee0, 0x143c8e670, 0x143c8ec28, 0x143c8dbd0, 0x1426467e8, 0x14040ac23> as *const ()), // sub_14040aa90
    Replacement::draft(0x14040ac30, "unknown_14040ac30", tpl_61019::<0x143c8ec28, 0x142489ee0, 0x143c8dbc4, 0x143c8dbd0, 0x1426467e8, 0x14040ad93> as *const ()), // sub_14040ac30
    Replacement::draft(0x14040b220, "unknown_14040b220", tpl_61019::<0x143c9ea00, 0x142489ee0, 0x143c8d68c, 0x143c964c0, 0x1426467e8, 0x14040b383> as *const ()), // sub_14040b220
    Replacement::draft(0x14040b390, "unknown_14040b390", tpl_61019::<0x143c92a7c, 0x142489ee0, 0x143c937c8, 0x143c92ee8, 0x1426467e8, 0x14040b4f3> as *const ()), // sub_14040b390
    Replacement::draft(0x14040b650, "unknown_14040b650", tpl_61197::<0x143c92a7c, 0x142489ee0, 0x143c92c98, 0x143c937c8, 0x143c9d840, 0x1426467e8, 0x14040b7e3> as *const ()), // sub_14040b650
    Replacement::draft(0x14040b7f0, "unknown_14040b7f0", tpl_61019::<0x143c92a7c, 0x142489ee0, 0x143c937c8, 0x143c9d840, 0x1426467e8, 0x14040b953> as *const ()), // sub_14040b7f0
    Replacement::draft(0x14040b960, "unknown_14040b960", fn_14040b960 as *const ()), // sub_14040b960
    Replacement::draft(0x14040bb50, "unknown_14040bb50", tpl_61196::<0x143c8d708, 0x142489ee0, 0x143c8d5b0, 0x1426467e8, 0x14040bc6d> as *const ()), // sub_14040bb50
    Replacement::draft(0x14040bc70, "unknown_14040bc70", fn_14040bc70 as *const ()), // sub_14040bc70
    Replacement::draft(0x14040be20, "unknown_14040be20", fn_14040be20 as *const ()), // sub_14040be20
    Replacement::draft(0x14040c110, "unknown_14040c110", tpl_61019::<0x143c8d68c, 0x142489ee0, 0x143c8d120, 0x143c933f0, 0x1426467e8, 0x14040c273> as *const ()), // sub_14040c110
    Replacement::draft(0x14040c280, "unknown_14040c280", fn_14040c280 as *const ()), // sub_14040c280
    Replacement::draft(0x14040c630, "unknown_14040c630", tpl_61019::<0x143c92ee8, 0x142489ee0, 0x143c937c8, 0x143c9e55c, 0x1426467e8, 0x14040c793> as *const ()), // sub_14040c630
    Replacement::draft(0x14040c7a0, "unknown_14040c7a0", tpl_3936::<0x30, 0x143c8d87c, 0x180a8, 0x18090, 0x142489ee0, 0x14021a360, 0x1426467e8, 0x14040c806> as *const ()), // sub_14040c7a0
    Replacement::draft(0x14040c810, "unknown_14040c810", tpl_61019::<0x143c8e70c, 0x142489ee0, 0x143c9db0c, 0x143c9ecdc, 0x1426467e8, 0x14040c973> as *const ()), // sub_14040c810
    Replacement::draft(0x14040cad0, "unknown_14040cad0", fn_14040cad0 as *const ()), // sub_14040cad0
    Replacement::draft(0x14040cb90, "unknown_14040cb90", fn_14040cb90 as *const ()), // sub_14040cb90
    Replacement::draft(0x14040cc20, "unknown_14040cc20", fn_14040cc20 as *const ()), // sub_14040cc20
    Replacement::draft(0x14040cca0, "unknown_14040cca0", fn_14040cca0 as *const ()), // sub_14040cca0
    Replacement::draft(0x14040ce40, "unknown_14040ce40", tpl_3936::<0x30, 0x143c8d160, 0x180a8, 0x18090, 0x142489ee0, 0x1402717e0, 0x1426467e8, 0x14040cea6> as *const ()), // sub_14040ce40
    Replacement::draft(0x14040ceb0, "unknown_14040ceb0", fn_14040ceb0 as *const ()), // sub_14040ceb0
    Replacement::draft(0x14040d1e0, "unknown_14040d1e0", fn_14040d1e0 as *const ()), // sub_14040d1e0
    Replacement::draft(0x14040d2a0, "unknown_14040d2a0", fn_14040d2a0 as *const ()), // sub_14040d2a0
    Replacement::draft(0x14040d360, "unknown_14040d360", fn_14040d360 as *const ()), // sub_14040d360
    Replacement::draft(0x14040d410, "unknown_14040d410", fn_14040d410 as *const ()), // sub_14040d410
    Replacement::draft(0x14040d570, "unknown_14040d570", fn_14040d570 as *const ()), // sub_14040d570
    Replacement::draft(0x14040d670, "unknown_14040d670", fn_14040d670 as *const ()), // sub_14040d670
    Replacement::draft(0x14040d860, "unknown_14040d860", fn_14040d860 as *const ()), // sub_14040d860
    Replacement::draft(0x14040da60, "unknown_14040da60", fn_14040da60 as *const ()), // sub_14040da60
    Replacement::draft(0x14040dcb0, "unknown_14040dcb0", fn_14040dcb0 as *const ()), // sub_14040dcb0
    Replacement::draft(0x14040ded0, "unknown_14040ded0", fn_14040ded0 as *const ()), // sub_14040ded0
    Replacement::draft(0x14040e080, "unknown_14040e080", fn_14040e080 as *const ()), // sub_14040e080
    Replacement::draft(0x14040e2c0, "unknown_14040e2c0", fn_14040e2c0 as *const ()), // sub_14040e2c0
    Replacement::draft(0x14040e4a0, "unknown_14040e4a0", fn_14040e4a0 as *const ()), // sub_14040e4a0
    Replacement::draft(0x14040e6c0, "unknown_14040e6c0", fn_14040e6c0 as *const ()), // sub_14040e6c0
    Replacement::draft(0x14040e8f0, "unknown_14040e8f0", fn_14040e8f0 as *const ()), // sub_14040e8f0
    Replacement::draft(0x14040eae0, "unknown_14040eae0", fn_14040eae0 as *const ()), // sub_14040eae0
    Replacement::draft(0x14040eb60, "unknown_14040eb60", tpl_61019::<0x143c9db0c, 0x142489ee0, 0x143c9ecdc, 0x143c9d1ac, 0x1426467e8, 0x14040ecc3> as *const ()), // sub_14040eb60
    Replacement::draft(0x14040ecd0, "unknown_14040ecd0", tpl_3491::<0x143c8d694, 0x142489ee0, 0x143c8dbc4, 0x143c8dbd0, 0x143c8ec28, 0x143c8d120, 0x143c8ec68> as *const ()), // sub_14040ecd0
    Replacement::draft(0x14040ee50, "unknown_14040ee50", fn_14040ee50 as *const ()), // sub_14040ee50
    Replacement::draft(0x14040ef10, "unknown_14040ef10", fn_14040ef10 as *const ()), // sub_14040ef10
    Replacement::draft(0x14040f020, "unknown_14040f020", fn_14040f020 as *const ()), // sub_14040f020
    Replacement::draft(0x14040f080, "unknown_14040f080", fn_14040f080 as *const ()), // sub_14040f080
    Replacement::draft(0x14040f130, "unknown_14040f130", fn_14040f130 as *const ()), // sub_14040f130
    Replacement::draft(0x14040f180, "unknown_14040f180", fn_14040f180 as *const ()), // sub_14040f180
    Replacement::draft(0x14040f390, "unknown_14040f390", fn_14040f390 as *const ()), // sub_14040f390
    Replacement::draft_throws(0x14040f460, "unknown_14040f460", fn_14040f460 as *const (), Throws::Discard), // sub_14040f460
    Replacement::draft(0x14040f4d0, "unknown_14040f4d0", fn_14040f4d0 as *const ()), // sub_14040f4d0
    Replacement::draft(0x14040f560, "unknown_14040f560", tpl_1360::<0x30, 0x143c8ec20, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14040f5e3> as *const ()), // sub_14040f560
    Replacement::draft(0x14040f5f0, "unknown_14040f5f0", tpl_4008::<0x30, 0x143c8d120, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14040f673> as *const ()), // sub_14040f5f0
    Replacement::draft(0x14040f680, "unknown_14040f680", fn_14040f680 as *const ()), // sub_14040f680
    Replacement::draft(0x14040f710, "unknown_14040f710", fn_14040f710 as *const ()), // sub_14040f710
    Replacement::draft(0x14040f7a0, "unknown_14040f7a0", tpl_4008::<0x30, 0x143c9d198, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14040f823> as *const ()), // sub_14040f7a0
    Replacement::draft(0x14040f830, "unknown_14040f830", fn_14040f830 as *const ()), // sub_14040f830
    Replacement::draft(0x14040f8c0, "unknown_14040f8c0", fn_14040f8c0 as *const ()), // sub_14040f8c0
    Replacement::draft(0x14040f930, "unknown_14040f930", tpl_3163::<0x38, 0x8, 0x140415fd0> as *const ()), // sub_14040f930
    Replacement::draft(0x14040f970, "unknown_14040f970", fn_14040f970 as *const ()), // sub_14040f970
    Replacement::draft(0x14040fa60, "unknown_14040fa60", fn_14040fa60 as *const ()), // sub_14040fa60
    Replacement::draft(0x14040faf0, "unknown_14040faf0", fn_14040faf0 as *const ()), // sub_14040faf0
    Replacement::draft_throws(0x14040fb50, "unknown_14040fb50", tpl_2918::<0x143c9d170, 0x143c9f334, 0x143c9e5d0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9ea00, 0x141834a0c> as *const (), Throws::Discard), // sub_14040fb50
    Replacement::draft_throws(0x14040fc80, "unknown_14040fc80", tpl_4016::<0x143c9f11c, 0x143c9ce60, 0x143c9c880, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c91608, 0x143c8cf5c, 0x143c9c890, 0x143c9c898, 0x141834a0c> as *const (), Throws::Discard), // sub_14040fc80
    Replacement::draft_throws(0x14040fde0, "unknown_14040fde0", tpl_4017::<0x143c9f40c, 0x143c9d458, 0x143c9d7b8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c90164, 0x143c9d17c, 0x143c9d7d8, 0x141834a0c> as *const (), Throws::Discard), // sub_14040fde0
    Replacement::draft_throws(0x14040ff50, "unknown_14040ff50", tpl_4017::<0x143c9e064, 0x143c9c940, 0x143c9deb8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c90164, 0x143c94188, 0x143c9ded8, 0x141834a0c> as *const (), Throws::Discard), // sub_14040ff50
    Replacement::draft_throws(0x1404100c0, "unknown_1404100c0", tpl_4018::<0x143c9d1cc, 0x143c9f010, 0x143c9d318, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c90164, 0x143c8d160, 0x143c9d328, 0x143c9d330, 0x141834a0c> as *const (), Throws::Discard), // sub_1404100c0
    Replacement::draft_throws(0x140410220, "unknown_140410220", tpl_847::<0x143c9d1e4, 0x143c9f06c, 0x143c9d560, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d1b8, 0x141834a0c> as *const (), Throws::Discard), // sub_140410220
    Replacement::draft_throws(0x140410350, "unknown_140410350", tpl_1845::<0x143c9e060, 0x143c9cc20, 0x143c9d8d8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d478, 0x143c8d0d0, 0x143c9d8e8, 0x143c9d8f0, 0x141834a0c> as *const (), Throws::Discard), // sub_140410350
    Replacement::draft(0x14041de20, "unknown_14041de20", fn_14041de20 as *const ()), // sub_14041de20
    Replacement::draft_throws(0x14041df60, "unknown_14041df60", fn_14041df60 as *const (), Throws::Discard), // sub_14041df60
    Replacement::draft(0x14041ede0, "unknown_14041ede0", fn_14041ede0 as *const ()), // sub_14041ede0
    Replacement::draft(0x14041efb0, "unknown_14041efb0", tpl_1158::<0x8, 0x4, 0x14240ab10> as *const ()), // sub_14041efb0
    Replacement::draft(0x14041f8b0, "unknown_14041f8b0", fn_14041f8b0 as *const ()), // sub_14041f8b0
    Replacement::draft(0x14041fa20, "unknown_14041fa20", fn_14041fa20 as *const ()), // sub_14041fa20
    Replacement::draft(0x14041faa0, "unknown_14041faa0", fn_14041faa0 as *const ()), // sub_14041faa0
    Replacement::draft(0x14041fcb0, "content_14041fcb0", fn_14041fcb0 as *const ()), // content::AbilityAIMeleeComponent::content::AbilityAIMeleeComponent::vfunc_0
    Replacement::draft(0x14041fd40, "content_14041fd40", tpl_855::<0x140420440, 0x1425371a0, 0x142533200, 0x1404206d0, 0x140420b80> as *const ()), // sub_14041fd40
    Replacement::draft(0x14041feb0, "content_14041feb0", tpl_855::<0x1404204b0, 0x1425371a0, 0x142533200, 0x1404208f0, 0x140420c20> as *const ()), // sub_14041feb0
    Replacement::draft(0x1404200c0, "content_1404200c0", fn_1404200c0 as *const ()), // sub_1404200c0
    Replacement::draft(0x1404203d0, "content_1404203d0", tpl_3583::<0x8, 0x18, 0x1404213b0> as *const ()), // sub_1404203d0
    Replacement::draft(0x140420440, "content_140420440", tpl_3583::<0x8, 0x10, 0x1404213b0> as *const ()), // sub_140420440
    Replacement::draft(0x1404204b0, "content_1404204b0", fn_1404204b0 as *const ()), // sub_1404204b0
    Replacement::draft(0x140420520, "content_140420520", fn_140420520 as *const ()), // sub_140420520
    Replacement::draft(0x1404206d0, "content_1404206d0", fn_1404206d0 as *const ()), // sub_1404206d0
    Replacement::draft(0x1404208f0, "content_1404208f0", fn_1404208f0 as *const ()), // sub_1404208f0
    Replacement::draft(0x140420ae0, "content_140420ae0", fn_140420ae0 as *const ()), // sub_140420ae0
    Replacement::draft(0x140420b80, "content_140420b80", fn_140420b80 as *const ()), // sub_140420b80
    Replacement::draft(0x140420c20, "content_140420c20", fn_140420c20 as *const ()), // sub_140420c20
    Replacement::draft(0x140420d20, "content_140420d20", tpl_1854::<0x8, 0x140421460> as *const ()), // sub_140420d20
    Replacement::draft(0x140420d60, "content_140420d60", tpl_4095::<0x8, 0x10, 0x1404213b0> as *const ()), // sub_140420d60
    Replacement::draft(0x140420dc0, "content_140420dc0", fn_140420dc0 as *const ()), // sub_140420dc0
    Replacement::draft(0x1404210a0, "content_1404210a0", fn_1404210a0 as *const ()), // sub_1404210a0
    Replacement::draft(0x140421250, "content_140421250", tpl_855::<0x140421580, 0x1425371a0, 0x142533200, 0x140421830, 0x140421a30> as *const ()), // sub_140421250
    Replacement::draft(0x140421510, "content_140421510", tpl_2059::<0x8, 0x1401f2bc0> as *const ()), // sub_140421510
    Replacement::draft(0x140421580, "content_140421580", fn_140421580 as *const ()), // sub_140421580
    Replacement::draft(0x140421600, "content_140421600", fn_140421600 as *const ()), // sub_140421600
    Replacement::draft(0x140421830, "content_140421830", fn_140421830 as *const ()), // sub_140421830
    Replacement::draft(0x140421a30, "content_140421a30", fn_140421a30 as *const ()), // sub_140421a30
    Replacement::draft(0x140421b40, "content_140421b40", fn_140421b40 as *const ()), // sub_140421b40
    Replacement::draft(0x140421b90, "content_140421b90", tpl_1037::<0x8, 0x1401f2bc0> as *const ()), // sub_140421b90
    Replacement::draft_throws(0x140421c10, "anonymous_namespace_140421c10", fn_140421c10 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::locomotionprobe::update>
    Replacement::draft_throws(0x140421ef0, "unknown_140421ef0", fn_140421ef0 as *const (), Throws::Discard), // sub_140421ef0
    Replacement::draft(0x140422180, "unknown_140422180", fn_140422180 as *const ()), // sub_140422180
    Replacement::draft(0x140422560, "unknown_140422560", tpl_1403::<0x143c9f540, 0x140422f30, 0x14020e8f0> as *const ()), // sub_140422560
    Replacement::draft(0x140422770, "unknown_140422770", fn_140422770 as *const ()), // sub_140422770
    Replacement::draft(0x1404227c0, "unknown_1404227c0", fn_1404227c0 as *const ()), // sub_1404227c0
    Replacement::draft(0x1404229d0, "unknown_1404229d0", fn_1404229d0 as *const ()), // sub_1404229d0
    Replacement::draft(0x140422f30, "unknown_140422f30", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14311fae8> as *const ()), // sub_140422f30
    Replacement::draft_throws(0x1404230b0, "unknown_1404230b0", tpl_4104::<0x88, 0x10088, 0x140423240, 0x140423150> as *const (), Throws::Discard), // sub_1404230b0
    Replacement::draft_throws(0x140423150, "unknown_140423150", tpl_4105::<0x18, 0x20, 0x40, 0x48, 0x8, 0x1419b7e10> as *const (), Throws::Discard), // sub_140423150
    Replacement::draft(0x140423460, "unknown_140423460", fn_140423460 as *const ()), // sub_140423460
    Replacement::draft_throws(0x140423610, "unknown_140423610", tpl_63343::<0x1404232f0, 0x1419b7e10> as *const (), Throws::Discard), // sub_140423610
    Replacement::draft(0x140423810, "unknown_140423810", fn_140423810 as *const ()), // sub_140423810
    Replacement::draft_throws(0x140423d60, "bigfish_140423d60", tpl_4108::<0x143c9f5c0, 0x143c9f608, 0x1411e9f50, 0x140424dd0, 0x143c9f54c, 0x1420af120, 0x14024e0b0, 0x1420a28e0, 0x140424a30, 0x140424930, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14311fc76, 0x142646960, 0x143c9f628, 0x143c9f638, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::location_intro::system::handleExpiration>
    Replacement::draft_throws(0x140424060, "bigfish_140424060", tpl_1480::<0x142724c00, 0x1410d6ca0, 0x140424630, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140424060
    Replacement::draft(0x140424300, "bigfish_140424300", tpl_1097::<0x143c9d1b8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140424300
    Replacement::draft_throws(0x140424630, "bigfish_140424630", fn_140424630 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::location_intro::system::handleEpisodeChange>
    Replacement::draft_throws(0x140424930, "bigfish_140424930", tpl_62933::<0x143c9d1b8, 0x142489ee0, 0x1419b8cf0> as *const (), Throws::Discard), // sub_140424930
    Replacement::draft_throws(0x140424a30, "bigfish_140424a30", tpl_62934::<0x143c9d1b8, 0x142489ee0, 0x1419b8cf0> as *const (), Throws::Discard), // sub_140424a30
    Replacement::draft_throws(0x140424b80, "bigfish_140424b80", tpl_62933::<0x143c9d1b8, 0x142489ee0, 0x1419b8c10> as *const (), Throws::Discard), // sub_140424b80
    Replacement::draft_throws(0x140424c80, "bigfish_140424c80", tpl_62934::<0x143c9d1b8, 0x142489ee0, 0x1419b8c10> as *const (), Throws::Discard), // sub_140424c80
    Replacement::draft(0x140425090, "bigfish_140425090", fn_140425090 as *const ()), // sub_140425090
    Replacement::draft_throws(0x140425260, "bigfish_140425260", tpl_4111::<0x143c9f7c8, 0x143c9f790, 0x1411e9f50, 0x140425530, 0x140425960, 0x1404257a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143120646, 0x142646960, 0x143c9f7b0, 0x143c9f7c0, 0x143c9f7c4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::systems::lightshifter::input_actions::update>
    Replacement::draft(0x1404254b0, "bigfish_1404254b0", tpl_2751::<0x3, 0x4, 0x8, 0x14240aa00> as *const ()), // sub_1404254b0
    Replacement::draft_throws(0x140425530, "bigfish_140425530", fn_140425530 as *const (), Throws::Discard), // sub_140425530
    Replacement::draft_throws(0x1404257a0, "bigfish_1404257a0", fn_1404257a0 as *const (), Throws::Discard), // sub_1404257a0
    Replacement::draft_throws(0x140425960, "bigfish_140425960", fn_140425960 as *const (), Throws::Discard), // sub_140425960
    Replacement::draft(0x140425da0, "bigfish_140425da0", fn_140425da0 as *const ()), // sub_140425da0
    Replacement::draft(0x140425f20, "bigfish_140425f20", fn_140425f20 as *const ()), // sub_140425f20
    Replacement::draft_throws(0x1404264c0, "bigfish_1404264c0", tpl_1480::<0x142725e80, 0x1410d6ca0, 0x140426e60, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404264c0
    Replacement::draft_throws(0x140426760, "bigfish_140426760", tpl_1480::<0x142725e80, 0x1410d6ca0, 0x1404270b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140426760
    Replacement::draft(0x140426a00, "bigfish_140426a00", fn_140426a00 as *const ()), // sub_140426a00
    Replacement::draft(0x140426aa0, "bigfish_140426aa0", tpl_3406::<0x143c9dba4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140426aa0
    Replacement::draft(0x140426df0, "bigfish_140426df0", tpl_4115::<0x8, 0x20, 0x24, 0x2c, 0x140426aa0> as *const ()), // sub_140426df0
    Replacement::draft_throws(0x140426e60, "bigfish_140426e60", tpl_3059::<0x143c9f96c, 0x143c9f850, 0x1411e9f50, 0x140427400, 0x140428390, 0x1404275c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431214e6, 0x142646960, 0x143c9f870, 0x143c9f880, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_sensitive_decal::gfx_graph::system>
    Replacement::draft_throws(0x1404270b0, "bigfish_1404270b0", tpl_3368::<0x143c9f8f0, 0x143c9f970, 0x1411e9f50, 0x1404278d0, 0x140428630, 0x140427ad0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143121ba6, 0x142646960, 0x143c9f990, 0x143c9f9a0, 0x143c9f9a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_sensitive_decal::decal_activation::system>
    Replacement::draft(0x140427310, "unknown_140427310", fn_140427310 as *const ()), // sub_140427310
    Replacement::draft_throws(0x140427400, "unknown_140427400", fn_140427400 as *const (), Throws::Discard), // sub_140427400
    Replacement::draft_throws(0x1404275c0, "unknown_1404275c0", tpl_4117::<0x88, 0x10088, 0x140427690, 0x140427ff0, 0x140427790> as *const (), Throws::Discard), // sub_1404275c0
    Replacement::draft(0x140427690, "unknown_140427690", tpl_3251::<0x8, 0x20, 0x143c9f9c4, 0x580e0, 0x143c9f84c, 0x143c9f934, 0x143c900d8, 0x143c9f9c0> as *const ()), // sub_140427690
    Replacement::draft_throws(0x140427790, "unknown_140427790", fn_140427790 as *const (), Throws::Discard), // sub_140427790
    Replacement::draft_throws(0x1404278d0, "unknown_1404278d0", fn_1404278d0 as *const (), Throws::Discard), // sub_1404278d0
    Replacement::draft(0x140427ad0, "unknown_140427ad0", fn_140427ad0 as *const ()), // sub_140427ad0
    Replacement::draft(0x140427ba0, "unknown_140427ba0", tpl_3101::<0x8, 0x20, 0x143c9f9c4, 0x143c8f63c, 0x143c9f9bc, 0x143c9f934, 0x580e0, 0x143c904a0, 0x143c8ceec> as *const ()), // sub_140427ba0
    Replacement::draft(0x140427cb0, "unknown_140427cb0", fn_140427cb0 as *const ()), // sub_140427cb0
    Replacement::draft_throws(0x140428120, "unknown_140428120", tpl_1330::<0x8, 0x1404289f0, 0x143c9f934, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140428120
    Replacement::draft_throws(0x140428200, "unknown_140428200", tpl_1330::<0x8, 0x140428bf0, 0x143c9f9c4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140428200
    Replacement::draft_throws(0x1404282e0, "unknown_1404282e0", tpl_1331::<0x8, 0x140428d00, 0x143c9f968, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_1404282e0
    Replacement::draft_throws(0x140428390, "unknown_140428390", fn_140428390 as *const (), Throws::Discard), // sub_140428390
    Replacement::draft(0x140428630, "unknown_140428630", fn_140428630 as *const ()), // sub_140428630
    Replacement::draft(0x140429080, "unknown_140429080", fn_140429080 as *const ()), // sub_140429080
    Replacement::draft(0x140429170, "unknown_140429170", fn_140429170 as *const ()), // sub_140429170
    Replacement::draft(0x140429240, "unknown_140429240", fn_140429240 as *const ()), // sub_140429240
    Replacement::draft(0x140429360, "unknown_140429360", fn_140429360 as *const ()), // sub_140429360
    Replacement::draft(0x140429470, "unknown_140429470", tpl_4126::<0x1401e5070, 0x1401de830, 0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_140429470
    Replacement::draft(0x140429590, "unknown_140429590", tpl_4126::<0x1401e5070, 0x1401de830, 0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_140429590
    Replacement::draft(0x1404296b0, "unknown_1404296b0", tpl_4127::<0x140427310> as *const ()), // sub_1404296b0
    Replacement::draft(0x140429720, "unknown_140429720", fn_140429720 as *const ()), // sub_140429720
    Replacement::draft_throws(0x140429850, "bigfish_140429850", tpl_4129::<0x143ca0018, 0x143c9fc40, 0x1411e9f50, 0x14042f050, 0x143c9fb64, 0x1420af120, 0x1420a28e0, 0x14042d360, 0x14042be40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143123c66, 0x142646960, 0x143c9fc60, 0x143c9fc64, 0x143c9fc66, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::reset::system>
    Replacement::draft_throws(0x140429b40, "bigfish_140429b40", tpl_3281::<0x143c9fca0, 0x143c9fc70, 0x1411e9f50, 0x14042ef60, 0x143c9fc2c, 0x1420af120, 0x1420a28e0, 0x14042d4b0, 0x14042bf40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143123ad6, 0x142646960, 0x143c9fc90, 0x143c9fc98, 0x143c9fc9c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::achievement::system>
    Replacement::draft_throws(0x140429e30, "bigfish_140429e30", tpl_4130::<0x143ca00ac, 0x143c9fab0, 0x1411e9f50, 0x14042ed00, 0x143c9ff48, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x14042d650, 0x14042c020, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143122a36, 0x142646960, 0x143c9fad0, 0x143c9fad8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::flashlight::system>
    Replacement::draft_throws(0x14042a140, "bigfish_14042a140", fn_14042a140 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::flare::system>
    Replacement::draft_throws(0x14042a460, "bigfish_14042a460", fn_14042a460 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::focal_point_destroyed::system>
    Replacement::draft_throws(0x14042a790, "bigfish_14042a790", tpl_3058::<0x143c9fddc, 0x143c9fa78, 0x1411e9f50, 0x14042e710, 0x143c9fea0, 0x1420af120, 0x143c8d910, 0x140215610, 0x143c8ceec, 0x1420a28e0, 0x14042dd80, 0x14042c5c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143122f26, 0x142646960, 0x143c9fa98, 0x143c9faa0, 0x143c9faa2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::flashbang::system>
    Replacement::draft(0x14042aaf0, "bigfish_14042aaf0", tpl_4133::<0x143c9fe9c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14042aaf0
    Replacement::draft(0x14042ae20, "bigfish_14042ae20", tpl_4133::<0x143c9fb58, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14042ae20
    Replacement::draft(0x14042b150, "bigfish_14042b150", tpl_1402::<0x143c9fb54, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14042b150
    Replacement::draft(0x14042b4a0, "bigfish_14042b4a0", tpl_1094::<0x143c9fca4, 0x14042bca0, 0x14020e8f0> as *const ()), // sub_14042b4a0
    Replacement::draft_throws(0x14042b6d0, "bigfish_14042b6d0", tpl_1438::<0x143c9fde4, 0x143c9fde8, 0x1411e9f50, 0x14042f240, 0x143c9fee4, 0x1420af120, 0x1420a28e0, 0x14042e4f0, 0x14042dfa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431236a6, 0x142646960, 0x143c9fe08, 0x143c9fe10, 0x143c9fe12, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::stream_in::system>
    Replacement::draft_throws(0x14042b9c0, "bigfish_14042b9c0", tpl_1467::<0x143c9fb78, 0x143c9fc00, 0x1411e9f50, 0x14042f150, 0x143c9fb68, 0x1420af120, 0x1420a28e0, 0x14042e600, 0x14042e060, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143123936, 0x142646960, 0x143c9fc20, 0x143c9fc28, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_reaction::stream_out::system>
    Replacement::draft(0x14042bca0, "unknown_14042bca0", tpl_4134::<0x1401e6d50, 0x140285d50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431223a0> as *const ()), // sub_14042bca0
    Replacement::draft(0x14042be40, "unknown_14042be40", tpl_3136::<0x143c8de68, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1419bd9d0> as *const ()), // sub_14042be40
    Replacement::draft_throws(0x14042bf40, "unknown_14042bf40", fn_14042bf40 as *const (), Throws::Discard), // sub_14042bf40
    Replacement::draft_throws(0x14042c020, "unknown_14042c020", tpl_4135::<0x88, 0x10088, 0x580e0, 0x143c8d910, 0x14042cf10, 0x14042c0e0> as *const (), Throws::Discard), // sub_14042c020
    Replacement::draft_throws(0x14042c0e0, "unknown_14042c0e0", tpl_4136::<0x18, 0x20, 0x40, 0x60, 0x68, 0x8, 0x1419bd510> as *const (), Throws::Discard), // sub_14042c0e0
    Replacement::draft_throws(0x14042c210, "unknown_14042c210", tpl_4135::<0x88, 0x10088, 0x580e0, 0x143c8d910, 0x14042cbc0, 0x14042c2d0> as *const (), Throws::Discard), // sub_14042c210
    Replacement::draft_throws(0x14042c2d0, "unknown_14042c2d0", tpl_4136::<0x18, 0x20, 0x40, 0x60, 0x68, 0x8, 0x1419bd270> as *const (), Throws::Discard), // sub_14042c2d0
    Replacement::draft_throws(0x14042c400, "unknown_14042c400", fn_14042c400 as *const (), Throws::Discard), // sub_14042c400
    Replacement::draft_throws(0x14042c4b0, "unknown_14042c4b0", tpl_4138::<0x18, 0x20, 0x40, 0x8, 0x1419bd080> as *const (), Throws::Discard), // sub_14042c4b0
    Replacement::draft(0x14042c5c0, "unknown_14042c5c0", tpl_1917::<0x143c8ceec, 0x143c8d910, 0x14042c7d0, 0x14042c6d0> as *const ()), // sub_14042c5c0
    Replacement::draft(0x14042c6d0, "unknown_14042c6d0", tpl_1918::<0x18, 0x20, 0x40, 0x8, 0x1419bcfb0> as *const ()), // sub_14042c6d0
    Replacement::draft(0x14042d360, "unknown_14042d360", tpl_62566::<0x143c8de68, 0x142489ee0, 0x1419bd9d0> as *const ()), // sub_14042d360
    Replacement::draft_throws(0x14042d4b0, "unknown_14042d4b0", fn_14042d4b0 as *const (), Throws::Discard), // sub_14042d4b0
    Replacement::draft_throws(0x14042d650, "unknown_14042d650", tpl_63371::<0x143c8d910, 0x14042d030, 0x1419bd510> as *const (), Throws::Discard), // sub_14042d650
    Replacement::draft_throws(0x14042d8d0, "unknown_14042d8d0", tpl_63371::<0x143c8d910, 0x14042cce0, 0x1419bd270> as *const (), Throws::Discard), // sub_14042d8d0
    Replacement::draft_throws(0x14042db50, "unknown_14042db50", fn_14042db50 as *const (), Throws::Discard), // sub_14042db50
    Replacement::draft(0x14042dd80, "unknown_14042dd80", tpl_61557::<0x143c8ceec, 0x143c8d910, 0x14042c7d0, 0x1419bcfb0> as *const ()), // sub_14042dd80
    Replacement::draft(0x14042dfa0, "unknown_14042dfa0", tpl_1126::<0x142407c80, 0x580f0, 0x1419bcea0> as *const ()), // sub_14042dfa0
    Replacement::draft(0x14042e060, "unknown_14042e060", tpl_1126::<0x142407c80, 0x580f0, 0x1419bcc30> as *const ()), // sub_14042e060
    Replacement::draft(0x14042f9b0, "unknown_14042f9b0", fn_14042f9b0 as *const ()), // sub_14042f9b0
    Replacement::draft(0x14042fab0, "unknown_14042fab0", fn_14042fab0 as *const ()), // sub_14042fab0
    Replacement::draft_throws(0x14042fb60, "bigfish_14042fb60", tpl_4143::<0x143ca030c, 0x143ca01b8, 0x140430c80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143125cf6, 0x142646960, 0x143ca01d8, 0x143ca01e8, 0x143ca01ec, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::lightdamagereceiver::simulated_light::system>
    Replacement::draft_throws(0x14042fd80, "bigfish_14042fd80", tpl_4144::<0x143ca0294, 0x143ca0360, 0x1411e9f50, 0x140431ab0, 0x143ca0298, 0x1420af120, 0x140215750, 0x1420a28e0, 0x140431500, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143126186, 0x142646960, 0x143ca0380, 0x143ca0388, 0x143ca038c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::lightdamagereceiver::cleanup::system>
    Replacement::draft_throws(0x1404300a0, "bigfish_1404300a0", fn_1404300a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::lightdamagereceiver::light_damage_events::system>
    Replacement::draft(0x1404303d0, "unknown_1404303d0", tpl_1403::<0x143ca0230, 0x140430e20, 0x14020e8f0> as *const ()), // sub_1404303d0
    Replacement::draft(0x1404305b0, "unknown_1404305b0", tpl_3072::<0x143c9fde0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404305b0
    Replacement::draft(0x140430900, "unknown_140430900", tpl_1435::<0x143ca02c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140430900
    Replacement::draft_throws(0x140430c80, "unknown_140430c80", fn_140430c80 as *const (), Throws::Discard), // sub_140430c80
    Replacement::draft(0x140430e20, "unknown_140430e20", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143126360> as *const ()), // sub_140430e20
    Replacement::draft(0x140430fa0, "unknown_140430fa0", tpl_62759::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x1404310f0, 0x1419bef10> as *const ()), // sub_140430fa0
    Replacement::draft_throws(0x140431220, "unknown_140431220", tpl_1330::<0x8, 0x140431b60, 0x143ca0308, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140431220
    Replacement::draft(0x140431300, "unknown_140431300", tpl_61019::<0x143c8d120, 0x142489ee0, 0x143c8d160, 0x143c9fde0, 0x1426467e8, 0x140431463> as *const ()), // sub_140431300
    Replacement::draft_throws(0x140431470, "unknown_140431470", tpl_4147::<0x143ca0308, 0x143c8ceec, 0x20, 0x143c9d1b0, 0x8, 0x580e0, 0x143c8f63c, 0x1419bf510> as *const (), Throws::Discard), // sub_140431470
    Replacement::draft(0x140431500, "unknown_140431500", tpl_4148::<0x143ca0298, 0x20, 0x8, 0x1419bec80> as *const ()), // sub_140431500
    Replacement::draft(0x140431560, "unknown_140431560", tpl_62765::<0x143c8ceec, 0x143c9fde0, 0x142489ee0, 0x143ca02c8, 0x143c8d160, 0x1419bef10> as *const ()), // sub_140431560
    Replacement::draft(0x140431c70, "unknown_140431c70", fn_140431c70 as *const ()), // sub_140431c70
    Replacement::draft(0x140432120, "unknown_140432120", tpl_1171::<0x58, 0x60, 0x143ca0550, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140432120
    Replacement::draft(0x140432200, "unknown_140432200", tpl_1171::<0x58, 0x60, 0x143ca05c0, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140432200
    Replacement::draft_throws(0x1404322e0, "bigfish_1404322e0", tpl_3683::<0x143ca06f4, 0x143ca0650, 0x1411e9f50, 0x140435370, 0x143ca07e0, 0x1420af120, 0x1420a28e0, 0x140434610, 0x140434110, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143126de6, 0x142646960, 0x143ca0670, 0x143ca0678, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::leash_zone_core::update::system>
    Replacement::draft_throws(0x1404325c0, "bigfish_1404325c0", tpl_1438::<0x143ca05c4, 0x143ca0478, 0x1411e9f50, 0x140435270, 0x143ca0708, 0x1420af120, 0x1420a28e0, 0x1404347c0, 0x140434200, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143126fe6, 0x142646960, 0x143ca0498, 0x143ca04a0, 0x143ca04a2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::leash_zone_buffer::update::system>
    Replacement::draft(0x1404328b0, "bigfish_1404328b0", tpl_1095::<0x143ca0550, 0x14020e8f0> as *const ()), // sub_1404328b0
    Replacement::draft(0x140432a90, "bigfish_140432a90", tpl_1436::<0x58, 0x60, 0x158, 0x143ca0550, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140432a90
    Replacement::draft(0x140432b70, "bigfish_140432b70", tpl_1095::<0x143ca05c0, 0x14020e8f0> as *const ()), // sub_140432b70
    Replacement::draft(0x140432d50, "bigfish_140432d50", tpl_1436::<0x58, 0x60, 0x158, 0x143ca05c0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140432d50
    Replacement::draft(0x140432e30, "bigfish_140432e30", tpl_1403::<0x143ca0700, 0x140433e10, 0x14020e8f0> as *const ()), // sub_140432e30
    Replacement::draft(0x140433010, "bigfish_140433010", tpl_1403::<0x143ca0598, 0x140433f90, 0x14020e8f0> as *const ()), // sub_140433010
    Replacement::draft_throws(0x140433250, "bigfish_140433250", tpl_1467::<0x143ca05bc, 0x143ca0408, 0x1411e9f50, 0x140435790, 0x143ca06f8, 0x1420af120, 0x1420a28e0, 0x140434d50, 0x1404342f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431271e6, 0x142646960, 0x143ca0428, 0x143ca0430, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::leash_zone_core::stream_in::system>
    Replacement::draft_throws(0x140433530, "bigfish_140433530", tpl_3281::<0x143ca07d4, 0x143ca0688, 0x1411e9f50, 0x1404356a0, 0x143ca07d0, 0x1420af120, 0x1420a28e0, 0x140434f00, 0x1404343e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143127746, 0x142646960, 0x143ca06a8, 0x143ca06b0, 0x143ca06b4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::leash_zone_core::stream_out::system>
    Replacement::draft_throws(0x140433820, "bigfish_140433820", tpl_3061::<0x143ca0470, 0x143ca0868, 0x1411e9f50, 0x140435560, 0x143ca0680, 0x1420af120, 0x1420a28e0, 0x140434fe0, 0x140434480, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143127496, 0x142646960, 0x143ca0888, 0x143ca0890, 0x143ca0894, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::leash_zone_buffer::stream_in::system>
    Replacement::draft_throws(0x140433b10, "bigfish_140433b10", tpl_1101::<0x143ca07e8, 0x143ca0738, 0x1411e9f50, 0x140435470, 0x143ca0704, 0x1420af120, 0x1420a28e0, 0x140435190, 0x140434570, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431265a6, 0x142646960, 0x143ca0758, 0x143ca0760, 0x143ca0764, 0x143ca0766, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::leash_zone_buffer::stream_out::system>
    Replacement::draft(0x140433e10, "unknown_140433e10", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143127f48> as *const ()), // sub_140433e10
    Replacement::draft(0x140433f90, "unknown_140433f90", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431280a0> as *const ()), // sub_140433f90
    Replacement::draft(0x140434110, "unknown_140434110", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140381a70, 0x580f0, 0x1419c0430> as *const ()), // sub_140434110
    Replacement::draft(0x140434200, "unknown_140434200", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140381a70, 0x580f0, 0x1419c02c0> as *const ()), // sub_140434200
    Replacement::draft(0x1404342f0, "unknown_1404342f0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140381a70, 0x580f0, 0x1419c04f0> as *const ()), // sub_1404342f0
    Replacement::draft(0x1404343e0, "unknown_1404343e0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140432a90> as *const ()), // sub_1404343e0
    Replacement::draft(0x140434480, "unknown_140434480", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140381a70, 0x580f0, 0x1419c0380> as *const ()), // sub_140434480
    Replacement::draft(0x140434570, "unknown_140434570", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140432d50> as *const ()), // sub_140434570
    Replacement::draft(0x140434610, "unknown_140434610", tpl_3213::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x1419c0430> as *const ()), // sub_140434610
    Replacement::draft(0x1404347c0, "unknown_1404347c0", tpl_3213::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x1419c02c0> as *const ()), // sub_1404347c0
    Replacement::draft(0x140434d50, "unknown_140434d50", tpl_3213::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x1419c04f0> as *const ()), // sub_140434d50
    Replacement::draft(0x140434f00, "unknown_140434f00", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140432a90> as *const ()), // sub_140434f00
    Replacement::draft(0x140434fe0, "unknown_140434fe0", tpl_3213::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x1419c0380> as *const ()), // sub_140434fe0
    Replacement::draft(0x140435190, "unknown_140435190", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140432d50> as *const ()), // sub_140435190
    Replacement::draft(0x140435ce0, "unknown_140435ce0", fn_140435ce0 as *const ()), // sub_140435ce0
    Replacement::draft(0x140435da0, "unknown_140435da0", fn_140435da0 as *const ()), // sub_140435da0
    Replacement::draft_throws(0x140436010, "unknown_140436010", fn_140436010 as *const (), Throws::Caught), // sub_140436010
    Replacement::draft_throws(0x1404361a0, "unknown_1404361a0", tpl_1905::<0x14387b200, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_1404361a0
    Replacement::draft_throws(0x140436250, "unknown_140436250", fn_140436250 as *const (), Throws::Discard), // sub_140436250
    Replacement::draft(0x1404363a0, "content_1404363a0", fn_1404363a0 as *const ()), // content::LadderComponent::content::LadderComponent::vfunc_10
    Replacement::draft(0x140436410, "unknown_140436410", fn_140436410 as *const ()), // sub_140436410
    Replacement::draft(0x140436790, "unknown_140436790", tpl_1171::<0x58, 0x60, 0x143c9ba14, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140436790
    Replacement::draft_throws(0x140436870, "bigfish_140436870", fn_140436870 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ladder::updateInteractable>
    Replacement::draft_throws(0x140436bc0, "bigfish_140436bc0", fn_140436bc0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ladder::update>
    Replacement::draft_throws(0x140436f10, "bigfish_140436f10", fn_140436f10 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ladder::align>
    Replacement::draft(0x140437220, "bigfish_140437220", tpl_3072::<0x143c9ba14, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140437220
    Replacement::draft(0x140437570, "bigfish_140437570", tpl_1436::<0x58, 0x60, 0x158, 0x143c9ba14, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140437570
    Replacement::draft(0x140437650, "bigfish_140437650", tpl_1094::<0x143ca0920, 0x140438540, 0x14020e8f0> as *const ()), // sub_140437650
    Replacement::draft(0x140437850, "bigfish_140437850", tpl_1466::<0x143c8d470, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140437850
    Replacement::draft(0x140437b20, "bigfish_140437b20", tpl_3073::<0x143ca0c74, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140437b20
    Replacement::draft_throws(0x140437e80, "bigfish_140437e80", fn_140437e80 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ladder::streamIn>
    Replacement::draft_throws(0x140438160, "bigfish_140438160", fn_140438160 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ladder::streamOut>
    Replacement::draft(0x140438440, "unknown_140438440", tpl_4160::<0x8, 0x140437650, 0x140437850> as *const ()), // sub_140438440
    Replacement::draft(0x1404384b0, "unknown_1404384b0", tpl_4161::<0x8, 0x140437b20> as *const ()), // sub_1404384b0
    Replacement::draft(0x140438540, "unknown_140438540", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143128510> as *const ()), // sub_140438540
    Replacement::draft(0x1404386c0, "unknown_1404386c0", fn_1404386c0 as *const ()), // sub_1404386c0
    Replacement::draft(0x140438840, "unknown_140438840", fn_140438840 as *const ()), // sub_140438840
    Replacement::draft(0x140438940, "unknown_140438940", fn_140438940 as *const ()), // sub_140438940
    Replacement::draft(0x140438a80, "unknown_140438a80", tpl_62759::<0x88, 0x10088, 0x143c8d910, 0x580e0, 0x140438bd0, 0x1419c29f0> as *const ()), // sub_140438a80
    Replacement::draft(0x140439150, "unknown_140439150", fn_140439150 as *const ()), // sub_140439150
    Replacement::draft(0x140439320, "unknown_140439320", fn_140439320 as *const ()), // sub_140439320
    Replacement::draft(0x1404395d0, "unknown_1404395d0", tpl_62765::<0x143c8d910, 0x143ca0c74, 0x142489ee0, 0x143c8ec20, 0x143c8d120, 0x1419c29f0> as *const ()), // sub_1404395d0
    Replacement::draft(0x140439820, "unknown_140439820", tpl_1127::<0x142407c80, 0x143ca0920, 0x142489ee0, 0x1419c27a0> as *const ()), // sub_140439820
    Replacement::draft(0x140439930, "unknown_140439930", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140437570> as *const ()), // sub_140439930
    Replacement::draft(0x140439bc0, "unknown_140439bc0", tpl_1131::<0x142407c80, 0x143ca0920, 0x142489ee0, 0x1419c27a0> as *const ()), // sub_140439bc0
    Replacement::draft(0x140439d20, "unknown_140439d20", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140437570> as *const ()), // sub_140439d20
    Replacement::draft(0x14043a020, "unknown_14043a020", fn_14043a020 as *const ()), // sub_14043a020
    Replacement::draft(0x14043aad0, "unknown_14043aad0", tpl_1368::<0x143c98790, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14043aad0
    Replacement::draft_throws(0x14043ae00, "bigfish_14043ae00", fn_14043ae00 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::item_balancing_data::healing_item_stream_in::system>
    Replacement::draft(0x14043b130, "unknown_14043b130", tpl_4168::<0x143c98648, 0x142407c80, 0x143c91064, 0x142489ee0, 0x1419c4c40> as *const ()), // sub_14043b130
    Replacement::draft(0x14043b270, "unknown_14043b270", tpl_4169::<0x143c98648, 0x142407c80, 0x143c91064, 0x142489ee0, 0x1419c4c40> as *const ()), // sub_14043b270
    Replacement::draft(0x14043b3f0, "unknown_14043b3f0", tpl_4170::<0x14043b8d0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_14043b3f0
    Replacement::draft(0x14043b640, "unknown_14043b640", fn_14043b640 as *const ()), // sub_14043b640
    Replacement::draft(0x14043b7e0, "unknown_14043b7e0", fn_14043b7e0 as *const ()), // sub_14043b7e0
    Replacement::draft(0x14043b8d0, "unknown_14043b8d0", fn_14043b8d0 as *const ()), // sub_14043b8d0
    Replacement::draft(0x14043bac0, "unknown_14043bac0", fn_14043bac0 as *const ()), // sub_14043bac0
    Replacement::draft(0x14043bd80, "unknown_14043bd80", fn_14043bd80 as *const ()), // sub_14043bd80
    Replacement::draft(0x14043be10, "unknown_14043be10", fn_14043be10 as *const ()), // sub_14043be10
    Replacement::draft(0x14043bf10, "unknown_14043bf10", fn_14043bf10 as *const ()), // sub_14043bf10
    Replacement::draft_throws(0x14043c020, "unknown_14043c020", fn_14043c020 as *const (), Throws::Discard), // sub_14043c020
    Replacement::draft(0x14043c2a0, "unknown_14043c2a0", fn_14043c2a0 as *const ()), // sub_14043c2a0
    Replacement::draft(0x14043c3b0, "unknown_14043c3b0", fn_14043c3b0 as *const ()), // sub_14043c3b0
    Replacement::draft(0x14043c500, "unknown_14043c500", fn_14043c500 as *const ()), // sub_14043c500
    Replacement::draft(0x14043c6f0, "unknown_14043c6f0", fn_14043c6f0 as *const ()), // sub_14043c6f0
    Replacement::draft_throws(0x14043c7a0, "unknown_14043c7a0", fn_14043c7a0 as *const (), Throws::Discard), // sub_14043c7a0
    Replacement::draft_throws(0x14043c890, "unknown_14043c890", fn_14043c890 as *const (), Throws::Discard), // sub_14043c890
    Replacement::draft(0x14043cb50, "unknown_14043cb50", fn_14043cb50 as *const ()), // sub_14043cb50
    Replacement::draft(0x14043cc00, "unknown_14043cc00", tpl_4182::<0xb0, 0x14020c520, 0x20, 0x18, 0x143a24623, 0x143a24620, 0x143a24628, 0x143a24624, 0x1, 0x3, 0x14020bfd0, 0x14183acbc, 0x14312b5d0, 0x140226cb0> as *const ()), // sub_14043cc00
    Replacement::draft(0x14043cd10, "unknown_14043cd10", fn_14043cd10 as *const ()), // sub_14043cd10
    Replacement::draft_throws(0x14043ce00, "unknown_14043ce00", fn_14043ce00 as *const (), Throws::Caught), // sub_14043ce00
    Replacement::draft(0x14043d000, "unknown_14043d000", tpl_4182::<0xb0, 0x14020c520, 0x20, 0x18, 0x143a246eb, 0x143a246e8, 0x143a246f0, 0x143a246ec, 0x1, 0x3, 0x14020bfd0, 0x14183acbc, 0x14312b7f8, 0x140226cb0> as *const ()), // sub_14043d000
    Replacement::draft(0x14043d110, "unknown_14043d110", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143ca2650, 0x1426467e8, 0x143c8edbc, 0x142489ee0, 0x14043d207> as *const ()), // sub_14043d110
    Replacement::draft(0x14043d320, "unknown_14043d320", fn_14043d320 as *const ()), // sub_14043d320
    Replacement::draft_throws(0x14043d510, "unknown_14043d510", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140443a40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043d510
    Replacement::draft_throws(0x14043d7b0, "unknown_14043d7b0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140443ca0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043d7b0
    Replacement::draft_throws(0x14043da50, "unknown_14043da50", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140443f80, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043da50
    Replacement::draft_throws(0x14043dcf0, "unknown_14043dcf0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x1404441c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043dcf0
    Replacement::draft(0x14043df90, "unknown_14043df90", tpl_1526::<0x14272a2f0, 0x1410d6ca0, 0x140444400, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14043df90
    Replacement::draft_throws(0x14043e230, "unknown_14043e230", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140444640, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043e230
    Replacement::draft_throws(0x14043e4d0, "unknown_14043e4d0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140444890, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043e4d0
    Replacement::draft_throws(0x14043e770, "unknown_14043e770", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140444ae0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043e770
    Replacement::draft_throws(0x14043ea10, "unknown_14043ea10", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140444d30, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043ea10
    Replacement::draft_throws(0x14043ecb0, "unknown_14043ecb0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140444f80, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043ecb0
    Replacement::draft_throws(0x14043ef50, "unknown_14043ef50", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x1404451d0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043ef50
    Replacement::draft_throws(0x14043f1f0, "unknown_14043f1f0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140445420, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043f1f0
    Replacement::draft_throws(0x14043f490, "unknown_14043f490", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140445670, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043f490
    Replacement::draft_throws(0x14043f730, "unknown_14043f730", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x1404458c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043f730
    Replacement::draft_throws(0x14043f9d0, "unknown_14043f9d0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140445b10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043f9d0
    Replacement::draft_throws(0x14043fc70, "unknown_14043fc70", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140445d50, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043fc70
    Replacement::draft_throws(0x14043ff10, "unknown_14043ff10", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140445fa0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14043ff10
    Replacement::draft_throws(0x1404401b0, "unknown_1404401b0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x1404461f0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404401b0
    Replacement::draft_throws(0x140440450, "unknown_140440450", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140446520, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140440450
    Replacement::draft_throws(0x1404406f0, "unknown_1404406f0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140446840, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404406f0
    Replacement::draft(0x140440990, "unknown_140440990", tpl_1526::<0x14272a2f0, 0x1410d6ca0, 0x140446a90, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140440990
    Replacement::draft(0x140440c30, "unknown_140440c30", tpl_1526::<0x14272a2f0, 0x1410d6ca0, 0x140446ce0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140440c30
    Replacement::draft_throws(0x140440ed0, "unknown_140440ed0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140446f20, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140440ed0
    Replacement::draft_throws(0x140441170, "unknown_140441170", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140447170, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140441170
    Replacement::draft_throws(0x140441410, "unknown_140441410", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x1404473c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140441410
    Replacement::draft_throws(0x1404416b0, "unknown_1404416b0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140447710, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404416b0
    Replacement::draft_throws(0x140441950, "unknown_140441950", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140447960, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140441950
    Replacement::draft_throws(0x140441bf0, "unknown_140441bf0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140447ba0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140441bf0
    Replacement::draft(0x140441e90, "unknown_140441e90", tpl_1526::<0x14272a2f0, 0x1410d6ca0, 0x140447e00, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140441e90
    Replacement::draft_throws(0x140442130, "unknown_140442130", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140448040, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140442130
    Replacement::draft_throws(0x1404423d0, "unknown_1404423d0", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x140448290, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404423d0
    Replacement::draft(0x140442670, "unknown_140442670", tpl_1526::<0x14272a2f0, 0x1410d6ca0, 0x1404484a0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140442670
    Replacement::draft_throws(0x140442910, "unknown_140442910", tpl_1480::<0x14272a2f0, 0x1410d6ca0, 0x1404486e0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140442910
    Replacement::draft_throws(0x140442bb0, "unknown_140442bb0", fn_140442bb0 as *const (), Throws::Discard), // sub_140442bb0
    Replacement::draft_throws(0x140442c10, "unknown_140442c10", fn_140442c10 as *const (), Throws::Discard), // sub_140442c10
    Replacement::draft(0x140442c90, "unknown_140442c90", fn_140442c90 as *const ()), // sub_140442c90
    Replacement::draft(0x140442dd0, "unknown_140442dd0", tpl_1094::<0x143ca22ec, 0x140449070, 0x14020e8f0> as *const ()), // sub_140442dd0
    Replacement::draft(0x140442fd0, "unknown_140442fd0", tpl_1094::<0x143c8dbc4, 0x140449230, 0x14020e8f0> as *const ()), // sub_140442fd0
    Replacement::draft(0x1404431d0, "unknown_1404431d0", tpl_1094::<0x143c933b0, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_1404431d0
    Replacement::draft(0x1404433d0, "unknown_1404433d0", fn_1404433d0 as *const ()), // sub_1404433d0
    Replacement::draft_throws(0x1404436e0, "unknown_1404436e0", fn_1404436e0 as *const (), Throws::Discard), // sub_1404436e0
    Replacement::draft(0x1404437e0, "unknown_1404437e0", fn_1404437e0 as *const ()), // sub_1404437e0
    Replacement::draft_throws(0x140443a40, "bigfish_140443a40", tpl_3062::<0x143ca24ac, 0x143ca1ac8, 0x1411e9f50, 0x14044b4b0, 0x140453fd0, 0x14044b6b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312dfc6, 0x142646960, 0x143ca1ae8, 0x143ca1af0, 0x143ca1af4, 0x143ca1af6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::input::canOpenInventory>
    Replacement::draft_throws(0x140443ca0, "bigfish_140443ca0", fn_140443ca0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::setup::system>
    Replacement::draft_throws(0x140443f80, "bigfish_140443f80", tpl_1532::<0x143ca1128, 0x143ca0fe0, 0x1411e9f50, 0x14044b9a0, 0x1404542c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312db36, 0x142646960, 0x143ca1000, 0x143ca1004, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::state::system>
    Replacement::draft_throws(0x1404441c0, "bigfish_1404441c0", tpl_1532::<0x143ca14b4, 0x143ca1500, 0x1411e9f50, 0x14044bb80, 0x1404543f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312eee6, 0x142646960, 0x143ca1520, 0x143ca1524, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::input::system>
    Replacement::draft(0x140444400, "bigfish_140444400", tpl_4192::<0x143ca1124, 0x143ca1b58, 0x1411e9f50, 0x14044bf90, 0x140454610, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312ed86, 0x142646960, 0x143ca1b78, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::input::updateDiscardHold>
    Replacement::draft_throws(0x140444640, "bigfish_140444640", tpl_3653::<0x143ca2540, 0x143ca1c00, 0x1411e9f50, 0x14044c100, 0x140454670, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312eaf6, 0x142646960, 0x143ca1c20, 0x143ca1c28, 0x143ca1c2c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::input::updateMoveHold>
    Replacement::draft_throws(0x140444890, "bigfish_140444890", tpl_1428::<0x143ca2284, 0x143ca1668, 0x1411e9f50, 0x14044c340, 0x140454760, 0x14044c6c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312e506, 0x142646960, 0x143ca1688, 0x143ca168c, 0x143ca168e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::picking::system>
    Replacement::draft_throws(0x140444ae0, "bigfish_140444ae0", tpl_4193::<0x143ca1ec0, 0x143ca10f0, 0x1411e9f50, 0x14044ca60, 0x1404549c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312c426, 0x142646960, 0x143ca1110, 0x143ca1120, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::picking::processEnterRequest>
    Replacement::draft_throws(0x140444d30, "bigfish_140444d30", tpl_1535::<0x143ca1eb8, 0x143ca1fe8, 0x1411e9f50, 0x14044ccf0, 0x140454b40, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312be36, 0x142646960, 0x143ca2008, 0x143ca2010, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::highlight::system>
    Replacement::draft_throws(0x140444f80, "bigfish_140444f80", tpl_3462::<0x143ca0f60, 0x143ca1990, 0x1411e9f50, 0x14044d0c0, 0x140454d40, 0x14044d2c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312bb56, 0x142646960, 0x143ca19b0, 0x143ca19c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::upgrade_highlight::system>
    Replacement::draft_throws(0x1404451d0, "bigfish_1404451d0", tpl_4194::<0x143ca2588, 0x143ca0fa8, 0x1411e9f50, 0x14044d0c0, 0x140454fa0, 0x14044d4c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312b876, 0x142646960, 0x143ca0fc8, 0x143ca0fd8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::charms_upgrade_highlight::system>
    Replacement::draft_throws(0x140445420, "bigfish_140445420", tpl_3368::<0x143ca0fa0, 0x143ca2028, 0x1411e9f50, 0x14044d6c0, 0x140455200, 0x14044d9a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312d726, 0x142646960, 0x143ca2048, 0x143ca2058, 0x143ca2060, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::charms_tutorial_highlight::system>
    Replacement::draft_throws(0x140445670, "bigfish_140445670", tpl_3064::<0x143ca26c8, 0x143ca1d08, 0x1411e9f50, 0x14044dc80, 0x140455480, 0x14044dde0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312d336, 0x142646960, 0x143ca1d28, 0x143ca1d30, 0x143ca1d34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::move::init::initCells>
    Replacement::draft_throws(0x1404458c0, "bigfish_1404458c0", tpl_4195::<0x143ca22c8, 0x143ca2338, 0x1411e9f50, 0x14044e190, 0x140455690, 0x14044e500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312cc16, 0x142646960, 0x143ca2358, 0x143ca2360, 0x143ca2362, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::move::state::system>
    Replacement::draft_throws(0x140445b10, "bigfish_140445b10", tpl_3005::<0x143ca2288, 0x143ca1598, 0x1411e9f50, 0x14044e960, 0x140455930, 0x14044ec00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14312c846, 0x142646960, 0x143ca15b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::move::system>
    Replacement::draft_throws(0x140445d50, "bigfish_140445d50", tpl_1503::<0x143ca1d70, 0x143ca16b8, 0x1411e9f50, 0x14044eeb0, 0x140455ba0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143134526, 0x142646960, 0x143ca16d8, 0x143ca16dc, 0x143ca16de, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::shoebox::system>
    Replacement::draft_throws(0x140445fa0, "bigfish_140445fa0", tpl_1578::<0x143ca265c, 0x143ca1460, 0x1411e9f50, 0x14044f090, 0x140455cd0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143134066, 0x142646960, 0x143ca1480, 0x143ca1484, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::events::system>
    Replacement::draft_throws(0x1404461f0, "bigfish_1404461f0", fn_1404461f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::storage_grid::init::initCells>
    Replacement::draft_throws(0x140446520, "bigfish_140446520", tpl_4197::<0x143ca2498, 0x143ca1a08, 0x1411e9f50, 0x140459db0, 0x143ca21fc, 0x1420af120, 0x143c93214, 0x140215610, 0x1420a28e0, 0x140455fc0, 0x14044f3e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143133b36, 0x142646960, 0x143ca1a28, 0x143ca1a38, 0x143ca1a3c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::storage_grid::processClearNew>
    Replacement::draft_throws(0x140446840, "bigfish_140446840", tpl_1185::<0x143ca2658, 0x143ca1c88, 0x1411e9f50, 0x14044f4f0, 0x140456130, 0x14044f7e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143134ea6, 0x142646960, 0x143ca1ca8, 0x143ca1cb0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::context_menu::system>
    Replacement::draft(0x140446a90, "bigfish_140446a90", fn_140446a90 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::state::sync>
    Replacement::draft(0x140446ce0, "bigfish_140446ce0", fn_140446ce0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::move::sync>
    Replacement::draft_throws(0x140446f20, "bigfish_140446f20", tpl_3465::<0x143ca19c4, 0x143ca23a0, 0x1411e9f50, 0x14044fdf0, 0x140456420, 0x14044ff90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431348a6, 0x142646960, 0x143ca23c0, 0x143ca23d0, 0x143ca23d2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::move::syncDndItemData>
    Replacement::draft_throws(0x140447170, "bigfish_140447170", tpl_3455::<0x143ca2660, 0x143ca18f0, 0x1411e9f50, 0x140450130, 0x1404566b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143132a56, 0x142646960, 0x143ca1910, 0x143ca1920, 0x143ca1924, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::move::updateCancelTimer>
    Replacement::draft_throws(0x1404473c0, "bigfish_1404473c0", fn_1404473c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::move::handleAutoCombineTimerDeletions>
    Replacement::draft_throws(0x140447710, "bigfish_140447710", tpl_4195::<0x143ca232c, 0x143ca1568, 0x1411e9f50, 0x140450400, 0x140456a20, 0x140450590, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143132536, 0x142646960, 0x143ca1588, 0x143ca1590, 0x143ca1592, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::shoebox::sync>
    Replacement::draft_throws(0x140447960, "bigfish_140447960", tpl_3060::<0x143ca1c40, 0x143ca1720, 0x1411e9f50, 0x1404506d0, 0x140456c40, 0x140450a10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143131e66, 0x142646960, 0x143ca1740, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::storage_grid::sync>
    Replacement::draft_throws(0x140447ba0, "bigfish_140447ba0", tpl_4201::<0x143ca1170, 0x143ca1c48, 0x1411e9f50, 0x140450e30, 0x140456ea0, 0x1404512e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143133466, 0x142646960, 0x143ca1c68, 0x143ca1c78, 0x143ca1c80, 0x143ca1c82, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::storage_grid::syncFocusedItem>
    Replacement::draft(0x140447e00, "bigfish_140447e00", tpl_4192::<0x143ca2018, 0x143ca2368, 0x1411e9f50, 0x1404516e0, 0x140457190, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143133266, 0x142646960, 0x143ca2388, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::context_menu::sync>
    Replacement::draft_throws(0x140448040, "bigfish_140448040", tpl_3064::<0x143ca2648, 0x143ca2740, 0x1411e9f50, 0x140451900, 0x140457230, 0x140451a50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143132fb6, 0x142646960, 0x143ca2760, 0x143ca2768, 0x143ca276c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::key_items::sync>
    Replacement::draft_throws(0x140448290, "bigfish_140448290", fn_140448290 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::data::key_items::syncFocusedItem>
    Replacement::draft(0x1404484a0, "bigfish_1404484a0", tpl_4203::<0x143ca1ac0, 0x143ca17c8, 0x1411e9f50, 0x140451ce0, 0x1404576a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143135ed6, 0x142646960, 0x143ca17e8, 0x143ca17ec, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::error::system>
    Replacement::draft_throws(0x1404486e0, "bigfish_1404486e0", tpl_1503::<0x143ca15f8, 0x143ca1890, 0x1411e9f50, 0x140451e20, 0x1404576e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143135a86, 0x142646960, 0x143ca18b0, 0x143ca18b4, 0x143ca18b6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_ui::cleanup::system>
    Replacement::draft_throws(0x140448930, "unknown_140448930", fn_140448930 as *const (), Throws::Discard), // sub_140448930
    Replacement::draft(0x140448ae0, "unknown_140448ae0", fn_140448ae0 as *const ()), // sub_140448ae0
    Replacement::draft(0x140448cc0, "unknown_140448cc0", fn_140448cc0 as *const ()), // sub_140448cc0
    Replacement::draft(0x140448e90, "unknown_140448e90", fn_140448e90 as *const ()), // sub_140448e90
    Replacement::draft(0x140449070, "unknown_140449070", fn_140449070 as *const ()), // sub_140449070
    Replacement::draft(0x140449230, "unknown_140449230", fn_140449230 as *const ()), // sub_140449230
    Replacement::draft(0x1404493b0, "unknown_1404493b0", tpl_4210::<0x1401e6d50, 0x14044af60, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_1404493b0
    Replacement::draft(0x140449540, "unknown_140449540", fn_140449540 as *const ()), // sub_140449540
    Replacement::draft(0x140449730, "unknown_140449730", fn_140449730 as *const ()), // sub_140449730
    Replacement::draft(0x140449c40, "unknown_140449c40", fn_140449c40 as *const ()), // sub_140449c40
    Replacement::draft(0x14044a150, "unknown_14044a150", fn_14044a150 as *const ()), // sub_14044a150
    Replacement::draft(0x14044a350, "unknown_14044a350", fn_14044a350 as *const ()), // sub_14044a350
    Replacement::draft_throws(0x14044a780, "unknown_14044a780", fn_14044a780 as *const (), Throws::Discard), // sub_14044a780
    Replacement::draft(0x14044a920, "unknown_14044a920", fn_14044a920 as *const ()), // sub_14044a920
    Replacement::draft(0x14044a990, "unknown_14044a990", tpl_4216::<0x14240a730> as *const ()), // sub_14044a990
    Replacement::draft(0x14044aae0, "unknown_14044aae0", fn_14044aae0 as *const ()), // sub_14044aae0
    Replacement::draft(0x14044abf0, "content_14044abf0", fn_14044abf0 as *const ()), // content::InventoryComponent::content::InventoryComponent::vfunc_0
    Replacement::draft(0x14044ac40, "unknown_14044ac40", fn_14044ac40 as *const ()), // sub_14044ac40
    Replacement::draft(0x14044aec0, "unknown_14044aec0", tpl_1124::<0x48> as *const ()), // sub_14044aec0
    Replacement::draft(0x14044aef0, "unknown_14044aef0", fn_14044aef0 as *const ()), // sub_14044aef0
    Replacement::draft(0x14044af60, "unknown_14044af60", tpl_1955::<0x10, 0x1401de830> as *const ()), // sub_14044af60
    Replacement::draft(0x14044af90, "unknown_14044af90", fn_14044af90 as *const ()), // sub_14044af90
    Replacement::draft(0x14044b130, "unknown_14044b130", fn_14044b130 as *const ()), // sub_14044b130
    Replacement::draft(0x14044b440, "unknown_14044b440", fn_14044b440 as *const ()), // sub_14044b440
    Replacement::draft_throws(0x14044b4b0, "unknown_14044b4b0", fn_14044b4b0 as *const (), Throws::Discard), // sub_14044b4b0
    Replacement::draft(0x14044b6b0, "unknown_14044b6b0", fn_14044b6b0 as *const ()), // sub_14044b6b0
    Replacement::draft(0x14044b840, "unknown_14044b840", fn_14044b840 as *const ()), // sub_14044b840
    Replacement::draft_throws(0x14044b9a0, "unknown_14044b9a0", fn_14044b9a0 as *const (), Throws::Discard), // sub_14044b9a0
    Replacement::draft_throws(0x14044bb80, "unknown_14044bb80", fn_14044bb80 as *const (), Throws::Discard), // sub_14044bb80
    Replacement::draft(0x14044bf90, "unknown_14044bf90", fn_14044bf90 as *const ()), // sub_14044bf90
    Replacement::draft_throws(0x14044c100, "unknown_14044c100", fn_14044c100 as *const (), Throws::Discard), // sub_14044c100
    Replacement::draft_throws(0x14044c340, "unknown_14044c340", fn_14044c340 as *const (), Throws::Discard), // sub_14044c340
    Replacement::draft_throws(0x14044c6c0, "unknown_14044c6c0", fn_14044c6c0 as *const (), Throws::Discard), // sub_14044c6c0
    Replacement::draft(0x14044c790, "unknown_14044c790", fn_14044c790 as *const ()), // sub_14044c790
    Replacement::draft_throws(0x14044c8c0, "unknown_14044c8c0", fn_14044c8c0 as *const (), Throws::Discard), // sub_14044c8c0
    Replacement::draft_throws(0x14044ca60, "unknown_14044ca60", fn_14044ca60 as *const (), Throws::Discard), // sub_14044ca60
    Replacement::draft_throws(0x14044ccf0, "unknown_14044ccf0", fn_14044ccf0 as *const (), Throws::Discard), // sub_14044ccf0
    Replacement::draft_throws(0x14044d0c0, "unknown_14044d0c0", fn_14044d0c0 as *const (), Throws::Discard), // sub_14044d0c0
    Replacement::draft_throws(0x14044d2c0, "unknown_14044d2c0", tpl_63459::<0x143c8ce88, 0x143c8dbc4, 0x143c8ced8, 0x143c94010, 0x143c9f408, 0x143c93214, 0x143ca14fc, 0x143c904a0, 0x142489ee0, 0x1419d65d0> as *const (), Throws::Discard), // sub_14044d2c0
    Replacement::draft_throws(0x14044d4c0, "unknown_14044d4c0", tpl_63459::<0x143c8ce88, 0x143c8dbc4, 0x143c8ced8, 0x143c94010, 0x143c9f408, 0x143c93214, 0x143ca14fc, 0x143c904a0, 0x142489ee0, 0x1419d64b0> as *const (), Throws::Discard), // sub_14044d4c0
    Replacement::draft_throws(0x14044d6c0, "unknown_14044d6c0", fn_14044d6c0 as *const (), Throws::Discard), // sub_14044d6c0
    Replacement::draft_throws(0x14044d9a0, "unknown_14044d9a0", fn_14044d9a0 as *const (), Throws::Discard), // sub_14044d9a0
    Replacement::draft(0x14044da60, "unknown_14044da60", fn_14044da60 as *const ()), // sub_14044da60
    Replacement::draft_throws(0x14044db60, "unknown_14044db60", fn_14044db60 as *const (), Throws::Discard), // sub_14044db60
    Replacement::draft_throws(0x14044dc80, "unknown_14044dc80", fn_14044dc80 as *const (), Throws::Discard), // sub_14044dc80
    Replacement::draft(0x14044dde0, "unknown_14044dde0", fn_14044dde0 as *const ()), // sub_14044dde0
    Replacement::draft(0x14044dfa0, "unknown_14044dfa0", fn_14044dfa0 as *const ()), // sub_14044dfa0
    Replacement::draft(0x14044e080, "unknown_14044e080", fn_14044e080 as *const ()), // sub_14044e080
    Replacement::draft_throws(0x14044e190, "unknown_14044e190", fn_14044e190 as *const (), Throws::Discard), // sub_14044e190
    Replacement::draft_throws(0x14044e500, "unknown_14044e500", fn_14044e500 as *const (), Throws::Discard), // sub_14044e500
    Replacement::draft(0x14044e640, "unknown_14044e640", fn_14044e640 as *const ()), // sub_14044e640
    Replacement::draft_throws(0x14044e7d0, "unknown_14044e7d0", fn_14044e7d0 as *const (), Throws::Discard), // sub_14044e7d0
    Replacement::draft_throws(0x14044e960, "unknown_14044e960", fn_14044e960 as *const (), Throws::Discard), // sub_14044e960
    Replacement::draft_throws(0x14044ec00, "unknown_14044ec00", tpl_4245::<0x88, 0x10088, 0x14044ecc0, 0x14035a180, 0x14044edb0> as *const (), Throws::Discard), // sub_14044ec00
    Replacement::draft(0x14044ecc0, "unknown_14044ecc0", fn_14044ecc0 as *const ()), // sub_14044ecc0
    Replacement::draft_throws(0x14044edb0, "unknown_14044edb0", fn_14044edb0 as *const (), Throws::Discard), // sub_14044edb0
    Replacement::draft_throws(0x14044eeb0, "unknown_14044eeb0", fn_14044eeb0 as *const (), Throws::Discard), // sub_14044eeb0
    Replacement::draft_throws(0x14044f090, "unknown_14044f090", fn_14044f090 as *const (), Throws::Discard), // sub_14044f090
    Replacement::draft(0x14044f3e0, "unknown_14044f3e0", tpl_60901::<0x143c93214, 0x580e0, 0x143c8dbc4, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1419cec50> as *const ()), // sub_14044f3e0
    Replacement::draft_throws(0x14044f4f0, "unknown_14044f4f0", fn_14044f4f0 as *const (), Throws::Discard), // sub_14044f4f0
    Replacement::draft(0x14044f7e0, "unknown_14044f7e0", tpl_4250::<0x88, 0x10088, 0x14044f8b0, 0x140452750, 0x14044f9c0> as *const ()), // sub_14044f7e0
    Replacement::draft(0x14044f8b0, "unknown_14044f8b0", fn_14044f8b0 as *const ()), // sub_14044f8b0
    Replacement::draft(0x14044f9c0, "unknown_14044f9c0", fn_14044f9c0 as *const ()), // sub_14044f9c0
    Replacement::draft(0x14044fb30, "unknown_14044fb30", fn_14044fb30 as *const ()), // sub_14044fb30
    Replacement::draft(0x14044fcb0, "unknown_14044fcb0", fn_14044fcb0 as *const ()), // sub_14044fcb0
    Replacement::draft_throws(0x14044fdf0, "unknown_14044fdf0", tpl_3018::<0x140459b10, 0x143ca2328, 0x1420af120, 0x14022e5f0, 0x143ca249c, 0x140215610, 0x143ca2280, 0x143c8ce88, 0x1420a28e0> as *const (), Throws::Discard), // sub_14044fdf0
    Replacement::draft_throws(0x14044ff90, "unknown_14044ff90", fn_14044ff90 as *const (), Throws::Discard), // sub_14044ff90
    Replacement::draft_throws(0x140450130, "unknown_140450130", fn_140450130 as *const (), Throws::Discard), // sub_140450130
    Replacement::draft(0x1404502f0, "unknown_1404502f0", fn_1404502f0 as *const ()), // sub_1404502f0
    Replacement::draft_throws(0x140450400, "unknown_140450400", fn_140450400 as *const (), Throws::Discard), // sub_140450400
    Replacement::draft_throws(0x140450590, "unknown_140450590", tpl_63486::<0x143ca1c34, 0x143c9f408, 0x143c8ce88, 0x14035a180, 0x1419c95c0> as *const (), Throws::Discard), // sub_140450590
    Replacement::draft_throws(0x1404506d0, "unknown_1404506d0", fn_1404506d0 as *const (), Throws::Discard), // sub_1404506d0
    Replacement::draft_throws(0x140450a10, "unknown_140450a10", fn_140450a10 as *const (), Throws::Discard), // sub_140450a10
    Replacement::draft(0x140450ad0, "unknown_140450ad0", fn_140450ad0 as *const ()), // sub_140450ad0
    Replacement::draft_throws(0x140450c80, "unknown_140450c80", fn_140450c80 as *const (), Throws::Discard), // sub_140450c80
    Replacement::draft_throws(0x140450e30, "unknown_140450e30", fn_140450e30 as *const (), Throws::Discard), // sub_140450e30
    Replacement::draft_throws(0x1404512e0, "unknown_1404512e0", fn_1404512e0 as *const (), Throws::Discard), // sub_1404512e0
    Replacement::draft(0x1404513b0, "unknown_1404513b0", fn_1404513b0 as *const ()), // sub_1404513b0
    Replacement::draft_throws(0x140451530, "unknown_140451530", fn_140451530 as *const (), Throws::Discard), // sub_140451530
    Replacement::draft(0x1404516e0, "unknown_1404516e0", fn_1404516e0 as *const ()), // sub_1404516e0
    Replacement::draft_throws(0x140451900, "unknown_140451900", fn_140451900 as *const (), Throws::Discard), // sub_140451900
    Replacement::draft_throws(0x140451a50, "unknown_140451a50", fn_140451a50 as *const (), Throws::Discard), // sub_140451a50
    Replacement::draft_throws(0x140451ba0, "unknown_140451ba0", tpl_63486::<0x143ca1de8, 0x143ca14ac, 0x143c8ce88, 0x1404520e0, 0x1419c9f00> as *const (), Throws::Discard), // sub_140451ba0
    Replacement::draft(0x140451ce0, "unknown_140451ce0", fn_140451ce0 as *const ()), // sub_140451ce0
    Replacement::draft_throws(0x140451e20, "unknown_140451e20", fn_140451e20 as *const (), Throws::Discard), // sub_140451e20
    Replacement::draft_throws(0x1404521e0, "unknown_1404521e0", tpl_1331::<0x8, 0x140459550, 0x143ca2464, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_1404521e0
    Replacement::draft_throws(0x140452970, "unknown_140452970", tpl_1330::<0x8, 0x140459ea0, 0x143ca2664, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140452970
    Replacement::draft_throws(0x140452b50, "unknown_140452b50", tpl_1331::<0x8, 0x14045a2f0, 0x143ca27d4, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_140452b50
    Replacement::draft(0x140453090, "unknown_140453090", fn_140453090 as *const ()), // sub_140453090
    Replacement::draft(0x140453100, "unknown_140453100", fn_140453100 as *const ()), // sub_140453100
    Replacement::draft(0x1404532d0, "unknown_1404532d0", fn_1404532d0 as *const ()), // sub_1404532d0
    Replacement::draft(0x140453550, "unknown_140453550", fn_140453550 as *const ()), // sub_140453550
    Replacement::draft(0x140453e20, "unknown_140453e20", fn_140453e20 as *const ()), // sub_140453e20
    Replacement::draft(0x140453f60, "unknown_140453f60", tpl_4270::<0x4c, 0x14044b440> as *const ()), // sub_140453f60
    Replacement::draft(0x140453fd0, "unknown_140453fd0", fn_140453fd0 as *const ()), // sub_140453fd0
    Replacement::draft(0x140454290, "unknown_140454290", fn_140454290 as *const ()), // sub_140454290
    Replacement::draft_throws(0x1404542c0, "unknown_1404542c0", tpl_4272::<0x143c8ced4, 0x143c905d4, 0x143c93cb0, 0x143c8ceec, 0x143c94010, 0x143ca249c, 0x143ca14a0, 0x143c8ced8, 0x1419d8500> as *const (), Throws::Discard), // sub_1404542c0
    Replacement::draft_throws(0x1404543f0, "unknown_1404543f0", fn_1404543f0 as *const (), Throws::Discard), // sub_1404543f0
    Replacement::draft_throws(0x140454610, "unknown_140454610", tpl_1562::<0x143c8ce88, 0x143c904a0, 0x143ca14a8, 0x580e0, 0x143c8cebc, 0x143c8ced8, 0x1419d8230> as *const (), Throws::Discard), // sub_140454610
    Replacement::draft(0x140454670, "unknown_140454670", fn_140454670 as *const ()), // sub_140454670
    Replacement::draft_throws(0x140454760, "unknown_140454760", fn_140454760 as *const (), Throws::Discard), // sub_140454760
    Replacement::draft_throws(0x1404549c0, "unknown_1404549c0", fn_1404549c0 as *const (), Throws::Discard), // sub_1404549c0
    Replacement::draft_throws(0x140454b40, "unknown_140454b40", fn_140454b40 as *const (), Throws::Discard), // sub_140454b40
    Replacement::draft_throws(0x140454d40, "unknown_140454d40", tpl_63511::<0x143c8ce88, 0x143c93214, 0x143c8ced8, 0x143ca14fc, 0x143c904a0, 0x143c9f408, 0x143c94010, 0x143c8dbc4, 0x142489ee0, 0x1419d65d0> as *const (), Throws::Discard), // sub_140454d40
    Replacement::draft_throws(0x140454fa0, "unknown_140454fa0", tpl_63511::<0x143c8ce88, 0x143c93214, 0x143c8ced8, 0x143ca14fc, 0x143c904a0, 0x143c9f408, 0x143c94010, 0x143c8dbc4, 0x142489ee0, 0x1419d64b0> as *const (), Throws::Discard), // sub_140454fa0
    Replacement::draft_throws(0x140455200, "unknown_140455200", fn_140455200 as *const (), Throws::Discard), // sub_140455200
    Replacement::draft(0x140455480, "unknown_140455480", fn_140455480 as *const ()), // sub_140455480
    Replacement::draft_throws(0x140455690, "unknown_140455690", fn_140455690 as *const (), Throws::Discard), // sub_140455690
    Replacement::draft_throws(0x140455930, "unknown_140455930", fn_140455930 as *const (), Throws::Discard), // sub_140455930
    Replacement::draft_throws(0x140455ba0, "unknown_140455ba0", tpl_4272::<0x143c8ced4, 0x143c905d4, 0x143c93cb0, 0x143c8ceec, 0x143c904a0, 0x143ca14a0, 0x143c8ced8, 0x143c9f408, 0x1419ce020> as *const (), Throws::Discard), // sub_140455ba0
    Replacement::draft(0x140455eb0, "unknown_140455eb0", fn_140455eb0 as *const ()), // sub_140455eb0
    Replacement::draft(0x140455fc0, "unknown_140455fc0", tpl_60916::<0x143c93214, 0x143c8dbc4, 0x142489ee0, 0x1419cec50> as *const ()), // sub_140455fc0
    Replacement::draft(0x140456130, "unknown_140456130", fn_140456130 as *const ()), // sub_140456130
    Replacement::draft_throws(0x140456380, "unknown_140456380", tpl_1562::<0x143c8ce88, 0x143c9f408, 0x143ca249c, 0x580e0, 0x143ca24a8, 0x143c8ced8, 0x1419cde30> as *const (), Throws::Discard), // sub_140456380
    Replacement::draft_throws(0x140456420, "unknown_140456420", fn_140456420 as *const (), Throws::Discard), // sub_140456420
    Replacement::draft_throws(0x1404566b0, "unknown_1404566b0", fn_1404566b0 as *const (), Throws::Discard), // sub_1404566b0
    Replacement::draft(0x1404567a0, "unknown_1404567a0", tpl_62838::<0x143c904a0, 0x143ca249c, 0x143c8ec28, 0x142489ee0, 0x143c8dbc4, 0x143c8dbd0, 0x1419c9680> as *const ()), // sub_1404567a0
    Replacement::draft_throws(0x140456a20, "unknown_140456a20", tpl_63520::<0x143ca1c34, 0x143c8ce88, 0x143c9f408, 0x143c8dbc4, 0x142489ee0, 0x143c8dbd0, 0x1419c95c0> as *const (), Throws::Discard), // sub_140456a20
    Replacement::draft_throws(0x140456c40, "unknown_140456c40", fn_140456c40 as *const (), Throws::Discard), // sub_140456c40
    Replacement::draft_throws(0x140456ea0, "unknown_140456ea0", fn_140456ea0 as *const (), Throws::Discard), // sub_140456ea0
    Replacement::draft_throws(0x140457190, "unknown_140457190", fn_140457190 as *const (), Throws::Discard), // sub_140457190
    Replacement::draft_throws(0x140457230, "unknown_140457230", fn_140457230 as *const (), Throws::Discard), // sub_140457230
    Replacement::draft_throws(0x140457480, "unknown_140457480", tpl_63520::<0x143ca1de8, 0x143c8ce88, 0x143ca14ac, 0x143c8dbc4, 0x142489ee0, 0x143c933b0, 0x1419c9f00> as *const (), Throws::Discard), // sub_140457480
    Replacement::draft_throws(0x1404576e0, "unknown_1404576e0", fn_1404576e0 as *const (), Throws::Caught), // sub_1404576e0
    Replacement::draft(0x1404577b0, "unknown_1404577b0", fn_1404577b0 as *const ()), // sub_1404577b0
    Replacement::draft_throws(0x140457980, "unknown_140457980", tpl_847::<0x143ca2650, 0x143ca13d4, 0x143ca22d8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8edbc, 0x141834a0c> as *const (), Throws::Discard), // sub_140457980
    Replacement::draft(0x140457ab0, "unknown_140457ab0", tpl_1507::<0x3, 0x4, 0x8, 0x14240aa00, 0x28, 0x2c, 0x30, 0x34> as *const ()), // sub_140457ab0
    Replacement::draft(0x140457b10, "unknown_140457b10", tpl_4216::<0x14240aa00> as *const ()), // sub_140457b10
    Replacement::draft(0x140457c60, "unknown_140457c60", fn_140457c60 as *const ()), // sub_140457c60
    Replacement::draft(0x140457c90, "unknown_140457c90", fn_140457c90 as *const ()), // sub_140457c90
    Replacement::draft(0x140457db0, "unknown_140457db0", fn_140457db0 as *const ()), // sub_140457db0
    Replacement::draft(0x140457fa0, "unknown_140457fa0", fn_140457fa0 as *const ()), // sub_140457fa0
    Replacement::draft(0x1404580f0, "unknown_1404580f0", fn_1404580f0 as *const ()), // sub_1404580f0
    Replacement::draft(0x140458310, "unknown_140458310", fn_140458310 as *const ()), // sub_140458310
    Replacement::draft(0x140458450, "unknown_140458450", fn_140458450 as *const ()), // sub_140458450
    Replacement::draft(0x140458660, "unknown_140458660", fn_140458660 as *const ()), // sub_140458660
    Replacement::draft(0x1404587f0, "unknown_1404587f0", fn_1404587f0 as *const ()), // sub_1404587f0
    Replacement::draft(0x1404589b0, "unknown_1404589b0", fn_1404589b0 as *const ()), // sub_1404589b0
    Replacement::draft(0x140458ae0, "unknown_140458ae0", fn_140458ae0 as *const ()), // sub_140458ae0
    Replacement::draft(0x140458c90, "unknown_140458c90", fn_140458c90 as *const ()), // sub_140458c90
    Replacement::draft(0x140458e80, "unknown_140458e80", fn_140458e80 as *const ()), // sub_140458e80
    Replacement::draft(0x140458fa0, "unknown_140458fa0", fn_140458fa0 as *const ()), // sub_140458fa0
    Replacement::draft(0x1404592c0, "unknown_1404592c0", fn_1404592c0 as *const ()), // sub_1404592c0
    Replacement::draft(0x140459470, "unknown_140459470", fn_140459470 as *const ()), // sub_140459470
    Replacement::draft(0x140459c10, "unknown_140459c10", fn_140459c10 as *const ()), // sub_140459c10
    Replacement::draft(0x14045a050, "unknown_14045a050", fn_14045a050 as *const ()), // sub_14045a050
    Replacement::draft(0x14045a4e0, "unknown_14045a4e0", fn_14045a4e0 as *const ()), // sub_14045a4e0
    Replacement::draft(0x14045a680, "unknown_14045a680", fn_14045a680 as *const ()), // sub_14045a680
    Replacement::draft(0x14045a850, "unknown_14045a850", fn_14045a850 as *const ()), // sub_14045a850
    Replacement::draft(0x14045ab80, "unknown_14045ab80", fn_14045ab80 as *const ()), // sub_14045ab80
    Replacement::draft(0x14045abd0, "unknown_14045abd0", fn_14045abd0 as *const ()), // sub_14045abd0
    Replacement::draft(0x14045ad90, "unknown_14045ad90", fn_14045ad90 as *const ()), // sub_14045ad90
    Replacement::draft(0x14045afd0, "unknown_14045afd0", fn_14045afd0 as *const ()), // sub_14045afd0
    Replacement::draft(0x14045b200, "unknown_14045b200", fn_14045b200 as *const ()), // sub_14045b200
    Replacement::draft(0x14045b3c0, "unknown_14045b3c0", tpl_4305::<0x38, 0x140459470> as *const ()), // sub_14045b3c0
    Replacement::draft(0x14045b430, "unknown_14045b430", fn_14045b430 as *const ()), // sub_14045b430
    Replacement::draft(0x14045bad0, "unknown_14045bad0", fn_14045bad0 as *const ()), // sub_14045bad0
    Replacement::draft(0x14045bb70, "unknown_14045bb70", fn_14045bb70 as *const ()), // sub_14045bb70
    Replacement::draft(0x14045bc40, "unknown_14045bc40", fn_14045bc40 as *const ()), // sub_14045bc40
    Replacement::draft(0x14045bcc0, "unknown_14045bcc0", tpl_63557::<0x8, 0x19, 0x10, 0x140246bb0, 0x1401e66f0, 0x1403f12b0> as *const ()), // sub_14045bcc0
    Replacement::draft(0x14045bd50, "unknown_14045bd50", fn_14045bd50 as *const ()), // sub_14045bd50
    Replacement::draft(0x14045bfd0, "unknown_14045bfd0", fn_14045bfd0 as *const ()), // sub_14045bfd0
    Replacement::draft(0x14045cbf0, "unknown_14045cbf0", fn_14045cbf0 as *const ()), // sub_14045cbf0
    Replacement::draft(0x14045ccb0, "unknown_14045ccb0", fn_14045ccb0 as *const ()), // sub_14045ccb0
    Replacement::draft(0x14045cd60, "unknown_14045cd60", fn_14045cd60 as *const ()), // sub_14045cd60
    Replacement::draft(0x14045cd80, "unknown_14045cd80", tpl_1520::<0x120, 0x14240ab10> as *const ()), // sub_14045cd80
    Replacement::draft(0x14045cde0, "unknown_14045cde0", fn_14045cde0 as *const ()), // sub_14045cde0
    Replacement::draft(0x14045cf90, "unknown_14045cf90", fn_14045cf90 as *const ()), // sub_14045cf90
    Replacement::draft_throws(0x14045cfa0, "unknown_14045cfa0", fn_14045cfa0 as *const (), Throws::Caught), // sub_14045cfa0
    Replacement::draft(0x14045d1b0, "unknown_14045d1b0", fn_14045d1b0 as *const ()), // sub_14045d1b0
    Replacement::draft(0x14045d2e0, "unknown_14045d2e0", fn_14045d2e0 as *const ()), // sub_14045d2e0
    Replacement::draft(0x14045d310, "unknown_14045d310", fn_14045d310 as *const ()), // sub_14045d310
    Replacement::draft(0x14045d3c0, "unknown_14045d3c0", fn_14045d3c0 as *const ()), // sub_14045d3c0
    Replacement::draft(0x14045d4c0, "unknown_14045d4c0", fn_14045d4c0 as *const ()), // sub_14045d4c0
    Replacement::draft_throws(0x14045d5b0, "bigfish_14045d5b0", tpl_3202::<0x143ca2d88, 0x143ca2978, 0x1411e9f50, 0x140462f70, 0x143ca2dd0, 0x1420af120, 0x143c8df88, 0x140215610, 0x1420a28e0, 0x140461910, 0x140460440, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143138006, 0x142646960, 0x143ca2998, 0x143ca29a8, 0x143ca29ac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_item::initialize_inventory::system>
    Replacement::draft_throws(0x14045d8e0, "bigfish_14045d8e0", tpl_1480::<0x14272b2f0, 0x1410d6ca0, 0x14045f9a0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14045d8e0
    Replacement::draft_throws(0x14045db80, "bigfish_14045db80", tpl_1185::<0x143ca2e18, 0x143ca2910, 0x1411e9f50, 0x140460860, 0x140461b20, 0x1404609e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143138a96, 0x142646960, 0x143ca2930, 0x143ca2938, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_item::item_queue::system>
    Replacement::draft_throws(0x14045ddd0, "bigfish_14045ddd0", tpl_4322::<0x143ca2cb8, 0x143ca2d58, 0x1411e9f50, 0x140462ad0, 0x143ca2974, 0x1420af120, 0x1403a3180, 0x143c98648, 0x140215610, 0x1420a28e0, 0x140461db0, 0x140460c40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143138766, 0x142646960, 0x143ca2d78, 0x143ca2d80, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_item::can_use::system>
    Replacement::draft(0x14045e0f0, "bigfish_14045e0f0", fn_14045e0f0 as *const ()), // sub_14045e0f0
    Replacement::draft(0x14045e240, "bigfish_14045e240", tpl_60953::<0x1426467e8, 0x143c91064, 0x142489ee0, 0x14045e2bc> as *const ()), // sub_14045e240
    Replacement::draft(0x14045e3f0, "bigfish_14045e3f0", tpl_1094::<0x143ca2d94, 0x14045ff00, 0x14020e8f0> as *const ()), // sub_14045e3f0
    Replacement::draft(0x14045e5f0, "bigfish_14045e5f0", tpl_1094::<0x143c8e670, 0x1404600a0, 0x14020e8f0> as *const ()), // sub_14045e5f0
    Replacement::draft(0x14045e7f0, "bigfish_14045e7f0", tpl_4324::<0x143ca2b5c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14045e7f0
    Replacement::draft(0x14045eb30, "bigfish_14045eb30", tpl_1094::<0x143c8ec28, 0x140460240, 0x14020e8f0> as *const ()), // sub_14045eb30
    Replacement::draft(0x14045ed30, "bigfish_14045ed30", fn_14045ed30 as *const ()), // sub_14045ed30
    Replacement::draft(0x14045ef40, "bigfish_14045ef40", tpl_1095::<0x143ca2e9c, 0x14020e8f0> as *const ()), // sub_14045ef40
    Replacement::draft(0x14045f120, "bigfish_14045f120", tpl_1095::<0x143ca29b4, 0x14020e8f0> as *const ()), // sub_14045f120
    Replacement::draft_throws(0x14045f300, "bigfish_14045f300", tpl_4326::<0x143ca2b64, 0x143ca27f0, 0x1411e9f50, 0x140463940, 0x143ca2d4c, 0x1420af120, 0x143c8df88, 0x140215610, 0x1420a28e0, 0x1404625d0, 0x140461660, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431383b6, 0x142646960, 0x143ca2810, 0x143ca2820, 0x143ca2828, 0x143ca282a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_item::stream_in_inventory_owner::system>
    Replacement::draft_throws(0x14045f630, "bigfish_14045f630", fn_14045f630 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_item::stream_out_inventory_owner::system>
    Replacement::draft_throws(0x14045f9a0, "bigfish_14045f9a0", tpl_3367::<0x143ca2c58, 0x143ca2940, 0x140460690, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143137796, 0x142646960, 0x143ca2960, 0x143ca2968, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inventory_item::clean_up::system>
    Replacement::draft_throws(0x14045fbe0, "unknown_14045fbe0", fn_14045fbe0 as *const (), Throws::Discard), // sub_14045fbe0
    Replacement::draft_throws(0x14045fd20, "unknown_14045fd20", tpl_3745::<0x19, 0x10, 0x14045fd20, 0x14241c190> as *const (), Throws::Discard), // sub_14045fd20
    Replacement::draft(0x14045fd80, "unknown_14045fd80", fn_14045fd80 as *const ()), // sub_14045fd80
    Replacement::draft(0x14045ff00, "unknown_14045ff00", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431391c8, 0x140461550> as *const ()), // sub_14045ff00
    Replacement::draft(0x1404600a0, "unknown_1404600a0", tpl_4329::<0x1401e6d50, 0x1404615d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_1404600a0
    Replacement::draft(0x140460240, "unknown_140460240", fn_140460240 as *const ()), // sub_140460240
    Replacement::draft_throws(0x140460440, "unknown_140460440", fn_140460440 as *const (), Throws::Caught), // sub_140460440
    Replacement::draft_throws(0x1404605a0, "unknown_1404605a0", fn_1404605a0 as *const (), Throws::Discard), // sub_1404605a0
    Replacement::draft_throws(0x140460690, "unknown_140460690", tpl_3370::<0x1411e9f50, 0x140463120, 0x143c8e670, 0x1401fc320, 0x140463610, 0x143ca2aec, 0x1420af120, 0x140215750, 0x1420a28e0, 0x140462960, 0x1401de410> as *const (), Throws::Discard), // sub_140460690
    Replacement::draft_throws(0x140460860, "unknown_140460860", tpl_4333::<0x140462c10, 0x143ca2be0, 0x1420af120, 0x143c8ef20, 0x140215610, 0x143c8ced8, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_140460860
    Replacement::draft(0x1404609e0, "unknown_1404609e0", fn_1404609e0 as *const ()), // sub_1404609e0
    Replacement::draft(0x140460ac0, "unknown_140460ac0", fn_140460ac0 as *const ()), // sub_140460ac0
    Replacement::draft(0x140460f80, "unknown_140460f80", tpl_1332::<0x18, 0x10, 0x8, 0x140461060, 0x14022ef90, 0x20, 0x40, 0x60, 0x80> as *const ()), // sub_140460f80
    Replacement::draft(0x1404613d0, "unknown_1404613d0", tpl_3491::<0x143c8d694, 0x142489ee0, 0x143c8dbc4, 0x143c8dbd0, 0x143c8ec28, 0x143ca2d94, 0x143c8d120> as *const ()), // sub_1404613d0
    Replacement::draft(0x140461550, "unknown_140461550", fn_140461550 as *const ()), // sub_140461550
    Replacement::draft(0x1404615d0, "unknown_1404615d0", tpl_1955::<0x18, 0x140236fa0> as *const ()), // sub_1404615d0
    Replacement::draft(0x140461600, "unknown_140461600", fn_140461600 as *const ()), // sub_140461600
    Replacement::draft_throws(0x140461660, "unknown_140461660", fn_140461660 as *const (), Throws::Caught), // sub_140461660
    Replacement::draft(0x1404617b0, "unknown_1404617b0", tpl_4341::<0x143ca29b0, 0x142407c80, 0x143c8dbc4, 0x142489ee0, 0x1419deaf0> as *const ()), // sub_1404617b0
    Replacement::draft_throws(0x140461910, "unknown_140461910", fn_140461910 as *const (), Throws::Caught), // sub_140461910
    Replacement::draft(0x140461b20, "unknown_140461b20", fn_140461b20 as *const ()), // sub_140461b20
    Replacement::draft(0x140461db0, "unknown_140461db0", fn_140461db0 as *const ()), // sub_140461db0
    Replacement::draft_throws(0x1404625d0, "unknown_1404625d0", fn_1404625d0 as *const (), Throws::Caught), // sub_1404625d0
    Replacement::draft(0x1404627b0, "unknown_1404627b0", tpl_4344::<0x143ca29b0, 0x142407c80, 0x143c8dbc4, 0x142489ee0, 0x1419deaf0> as *const ()), // sub_1404627b0
    Replacement::draft(0x140462960, "unknown_140462960", tpl_4345::<0x143c8e670, 0x143ca2aec, 0x142407c80, 0x1419de810, 0x1404634f0> as *const ()), // sub_140462960
    Replacement::draft(0x140462c10, "unknown_140462c10", fn_140462c10 as *const ()), // sub_140462c10
    Replacement::draft(0x140462f70, "unknown_140462f70", fn_140462f70 as *const ()), // sub_140462f70
    Replacement::draft(0x140463120, "unknown_140463120", tpl_3188::<0x1404636c0, 0x143139320, 0x1404634f0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140463120
    Replacement::draft(0x1404631a0, "unknown_1404631a0", tpl_4348::<0x20, 0x28, 0x1404634f0> as *const ()), // sub_1404631a0
    Replacement::draft(0x140463370, "unknown_140463370", fn_140463370 as *const ()), // sub_140463370
    Replacement::draft(0x1404634f0, "unknown_1404634f0", tpl_4352::<0x1401de830, 0x140236fa0, 0x1425371a0, 0x142533200> as *const ()), // sub_1404634f0
    Replacement::draft(0x1404636c0, "unknown_1404636c0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143139320, 0x8, 0x20> as *const ()), // sub_1404636c0
    Replacement::draft(0x140463940, "unknown_140463940", fn_140463940 as *const ()), // sub_140463940
    Replacement::draft(0x140463ed0, "unknown_140463ed0", fn_140463ed0 as *const ()), // sub_140463ed0
    Replacement::draft_throws(0x140463fb0, "unknown_140463fb0", fn_140463fb0 as *const (), Throws::Caught), // sub_140463fb0
    Replacement::draft(0x1404641f0, "unknown_1404641f0", fn_1404641f0 as *const ()), // sub_1404641f0
    Replacement::draft(0x140464200, "unknown_140464200", fn_140464200 as *const ()), // sub_140464200
    Replacement::draft(0x140464270, "unknown_140464270", fn_140464270 as *const ()), // sub_140464270
    Replacement::draft(0x1404643f0, "unknown_1404643f0", tpl_1171::<0x58, 0x60, 0x143c8d528, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1404643f0
    Replacement::draft(0x1404644d0, "unknown_1404644d0", tpl_61019::<0x143c8d160, 0x142489ee0, 0x143c8d708, 0x143ca3428, 0x1426467e8, 0x140464633> as *const ()), // sub_1404644d0
    Replacement::draft(0x140464640, "unknown_140464640", tpl_63595::<0x140464780, 0x1426467e8, 0x14046477e> as *const ()), // sub_140464640
    Replacement::draft_throws(0x1404649b0, "bigfish_1404649b0", tpl_3086::<0x143ca38f0, 0x143ca3368, 0x1411e9f50, 0x14046d180, 0x143ca3420, 0x1420af120, 0x143ca38e0, 0x140215610, 0x1420a28e0, 0x140469d70, 0x140468250, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313d016, 0x142646960, 0x143ca3388, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_in_combat_state::system>
    Replacement::draft_throws(0x140464cb0, "bigfish_140464cb0", tpl_4359::<0x143ca3234, 0x143ca3600, 0x140467970, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313d1d6, 0x142646960, 0x143ca3620, 0x143ca3628, 0x143ca362c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_inventory_full_state::system>
    Replacement::draft_throws(0x140464ec0, "bigfish_140464ec0", tpl_3677::<0x143ca32bc, 0x143ca3290, 0x1411e9f50, 0x14046ce80, 0x143ca323c, 0x1420af120, 0x140469af0, 0x1420a28e0, 0x14046a110, 0x1404684c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313c186, 0x142646960, 0x143ca32b0, 0x143ca32b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_locked_door_state::system>
    Replacement::draft_throws(0x1404651c0, "bigfish_1404651c0", fn_1404651c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_locked_double_door_state::system>
    Replacement::draft_throws(0x140465530, "bigfish_140465530", tpl_1396::<0x143ca3080, 0x143ca3630, 0x1411e9f50, 0x140468800, 0x14046a560, 0x140468a00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313c826, 0x142646960, 0x143ca3650, 0x143ca3658, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_input_reservation::system>
    Replacement::draft_throws(0x140465780, "bigfish_140465780", tpl_4361::<0x143ca3154, 0x143ca38b8, 0x1411e9f50, 0x14046c840, 0x143ca38e8, 0x1420af120, 0x143ca38e0, 0x140215610, 0x1420a28e0, 0x14046a810, 0x140468cc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313ce76, 0x142646960, 0x143ca38d8, 0x143ca38dc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_player_state::system>
    Replacement::draft_throws(0x140465a90, "bigfish_140465a90", fn_140465a90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_interactor_disabled::system>
    Replacement::draft_throws(0x140465de0, "bigfish_140465de0", fn_140465de0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::manual_save::updateOffsets>
    Replacement::draft_throws(0x140466100, "bigfish_140466100", tpl_1396::<0x143ca3354, 0x143ca3888, 0x1411e9f50, 0x140469080, 0x14046ae00, 0x140469260, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313b916, 0x142646960, 0x143ca38a8, 0x143ca38b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::manual_save::updateInputHandling>
    Replacement::draft_throws(0x140466350, "bigfish_140466350", fn_140466350 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::manual_save::updateTriggeredHandling>
    Replacement::draft_throws(0x1404666b0, "bigfish_1404666b0", tpl_1480::<0x14272bc60, 0x1410d6ca0, 0x140467ba0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404666b0
    Replacement::draft_throws(0x140466950, "bigfish_140466950", fn_140466950 as *const (), Throws::Discard), // sub_140466950
    Replacement::draft_throws(0x140466a90, "bigfish_140466a90", fn_140466a90 as *const (), Throws::Discard), // sub_140466a90
    Replacement::draft(0x140466b00, "bigfish_140466b00", tpl_1094::<0x143ca36f4, 0x140467f50, 0x14020e8f0> as *const ()), // sub_140466b00
    Replacement::draft(0x140466d00, "bigfish_140466d00", tpl_1403::<0x143ca37d4, 0x1404680d0, 0x14020e8f0> as *const ()), // sub_140466d00
    Replacement::draft(0x140466ee0, "bigfish_140466ee0", tpl_1099::<0x143ca3428, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140466ee0
    Replacement::draft(0x140467220, "bigfish_140467220", tpl_1095::<0x143c8d528, 0x14020e8f0> as *const ()), // sub_140467220
    Replacement::draft(0x140467400, "bigfish_140467400", tpl_1095::<0x143ca3424, 0x14020e8f0> as *const ()), // sub_140467400
    Replacement::draft(0x1404675e0, "bigfish_1404675e0", tpl_1096::<0x143ca365c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404675e0
    Replacement::draft_throws(0x140467970, "bigfish_140467970", tpl_3083::<0x1411e9f50, 0x14046d080, 0x143ca3300, 0x1420af120, 0x14046cf90, 0x143ca3170, 0x140215750, 0x143c8ce88, 0x140215610, 0x1420a28e0, 0x140469ee0, 0x140468360> as *const (), Throws::Discard), // sub_140467970
    Replacement::draft_throws(0x140467ba0, "bigfish_140467ba0", tpl_2988::<0x143ca3360, 0x143ca35c8, 0x1411e9f50, 0x14046b270, 0x14046c0b0, 0x14046b590, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313a926, 0x142646960, 0x143ca35e8, 0x143ca35f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ui_systems::updateInteractFacts>
    Replacement::draft(0x140467df0, "unknown_140467df0", fn_140467df0 as *const ()), // sub_140467df0
    Replacement::draft(0x140467f50, "unknown_140467f50", tpl_4368::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1401ff5b0> as *const ()), // sub_140467f50
    Replacement::draft(0x1404680d0, "unknown_1404680d0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14313e820> as *const ()), // sub_1404680d0
    Replacement::draft(0x140468250, "unknown_140468250", tpl_60901::<0x143ca38e0, 0x580e0, 0x143c8d708, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1419ea5e0> as *const ()), // sub_140468250
    Replacement::draft(0x140468360, "unknown_140468360", tpl_62526::<0x143ca3170, 0x143c8ce88, 0x140469bd0, 0x1419ea170> as *const ()), // sub_140468360
    Replacement::draft(0x1404684c0, "unknown_1404684c0", tpl_1781::<0x88, 0x10088, 0x143c9f308, 0x20, 0x8, 0x1404699c0, 0x1404685a0> as *const ()), // sub_1404684c0
    Replacement::draft(0x1404685a0, "unknown_1404685a0", tpl_1782::<0x18, 0x20, 0x30, 0x10, 0x28, 0x8, 0x1419ea070> as *const ()), // sub_1404685a0
    Replacement::draft(0x140468680, "unknown_140468680", tpl_4369::<0x143ca35c4, 0x143c9f308, 0x143ca38f8, 0x142489ee0, 0x1419e9db0> as *const ()), // sub_140468680
    Replacement::draft_throws(0x140468800, "unknown_140468800", fn_140468800 as *const (), Throws::Discard), // sub_140468800
    Replacement::draft_throws(0x140468a00, "unknown_140468a00", tpl_4371::<0x88, 0x10088, 0x140468ac0, 0x140469790, 0x140468ba0> as *const (), Throws::Discard), // sub_140468a00
    Replacement::draft(0x140468ac0, "unknown_140468ac0", tpl_4372::<0x20, 0x143ca3a64, 0x8, 0x143ca3168, 0x143ca38e0, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_140468ac0
    Replacement::draft_throws(0x140468ba0, "unknown_140468ba0", tpl_4373::<0x18, 0x20, 0x8, 0x10, 0x38, 0x28, 0x1419e9620> as *const (), Throws::Discard), // sub_140468ba0
    Replacement::draft(0x140468cc0, "unknown_140468cc0", fn_140468cc0 as *const ()), // sub_140468cc0
    Replacement::draft(0x140468dd0, "unknown_140468dd0", fn_140468dd0 as *const ()), // sub_140468dd0
    Replacement::draft(0x140468f20, "unknown_140468f20", fn_140468f20 as *const ()), // sub_140468f20
    Replacement::draft_throws(0x140469080, "unknown_140469080", tpl_3484::<0x140469910, 0x14046c420, 0x143ca39fc, 0x1420af120, 0x140215750, 0x143ca38e0, 0x140215610, 0x143c8ceec, 0x143c8ce88, 0x1420a28e0> as *const (), Throws::Discard), // sub_140469080
    Replacement::draft_throws(0x140469260, "unknown_140469260", tpl_4371::<0x88, 0x10088, 0x140469320, 0x140469790, 0x140469400> as *const (), Throws::Discard), // sub_140469260
    Replacement::draft(0x140469320, "unknown_140469320", tpl_3105::<0x143ca39fc, 0x20, 0x8, 0x143ca38e0, 0x580e0, 0x143c8ceec, 0x142407c80, 0x580f0, 0x143c8ce88, 0x10, 0x18, 0x30> as *const ()), // sub_140469320
    Replacement::draft_throws(0x140469400, "unknown_140469400", fn_140469400 as *const (), Throws::Discard), // sub_140469400
    Replacement::draft(0x140469500, "unknown_140469500", tpl_62820::<0x143c8d910, 0x143ca3428, 0x143c8ce88, 0x142489ee0, 0x1419e8e50> as *const ()), // sub_140469500
    Replacement::draft(0x140469620, "unknown_140469620", tpl_3795::<0x143ca3424, 0x142489ee0, 0x143c8eb90, 0x143c96de4, 0x143c97794> as *const ()), // sub_140469620
    Replacement::draft(0x140469790, "unknown_140469790", tpl_4376::<0x143c8d160, 0x142489ee0, 0x143c8d87c, 0x143c8d478, 0x143c8d528> as *const ()), // sub_140469790
    Replacement::draft_throws(0x140469910, "unknown_140469910", tpl_1331::<0x8, 0x14046cb90, 0x143ca316c, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_140469910
    Replacement::draft_throws(0x140469af0, "unknown_140469af0", tpl_1330::<0x8, 0x14041ec40, 0x143c9f308, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140469af0
    Replacement::draft_throws(0x140469cd0, "unknown_140469cd0", fn_140469cd0 as *const (), Throws::Discard), // sub_140469cd0
    Replacement::draft(0x140469d40, "unknown_140469d40", tpl_1955::<0x10, 0x1401f2bc0> as *const ()), // sub_140469d40
    Replacement::draft(0x140469d70, "unknown_140469d70", tpl_60916::<0x143ca38e0, 0x143c8d708, 0x142489ee0, 0x1419ea5e0> as *const ()), // sub_140469d70
    Replacement::draft(0x140469ee0, "unknown_140469ee0", tpl_62563::<0x143ca3170, 0x143c8ce88, 0x143c8dbc4, 0x142489ee0, 0x143c8d478, 0x1419ea170> as *const ()), // sub_140469ee0
    Replacement::draft(0x14046a110, "unknown_14046a110", tpl_62640::<0x143c9f308, 0x143c93228, 0x142489ee0, 0x143c92ee8, 0x143c9d9fc, 0x1419ea070> as *const ()), // sub_14046a110
    Replacement::draft(0x14046a380, "unknown_14046a380", tpl_63609::<0x143ca35c4, 0x143c9f308, 0x143ca38f8, 0x142489ee0, 0x1419e9db0> as *const ()), // sub_14046a380
    Replacement::draft_throws(0x14046a560, "unknown_14046a560", fn_14046a560 as *const (), Throws::Discard), // sub_14046a560
    Replacement::draft(0x14046a810, "unknown_14046a810", fn_14046a810 as *const ()), // sub_14046a810
    Replacement::draft(0x14046a980, "unknown_14046a980", fn_14046a980 as *const ()), // sub_14046a980
    Replacement::draft(0x14046ac30, "unknown_14046ac30", fn_14046ac30 as *const ()), // sub_14046ac30
    Replacement::draft_throws(0x14046ae00, "unknown_14046ae00", fn_14046ae00 as *const (), Throws::Discard), // sub_14046ae00
    Replacement::draft(0x14046b0c0, "unknown_14046b0c0", tpl_62837::<0x143c8d910, 0x143c8ce88, 0x143ca3428, 0x142489ee0, 0x1419e8e50> as *const ()), // sub_14046b0c0
    Replacement::draft_throws(0x14046b270, "unknown_14046b270", fn_14046b270 as *const (), Throws::Discard), // sub_14046b270
    Replacement::draft_throws(0x14046b590, "unknown_14046b590", fn_14046b590 as *const (), Throws::Discard), // sub_14046b590
    Replacement::draft(0x14046b6d0, "unknown_14046b6d0", fn_14046b6d0 as *const ()), // sub_14046b6d0
    Replacement::draft_throws(0x14046b800, "unknown_14046b800", fn_14046b800 as *const (), Throws::Discard), // sub_14046b800
    Replacement::draft(0x14046b960, "unknown_14046b960", tpl_4382::<0x143ca36f4, 0x142489ee0, 0x143c8d478, 0x143c8d4b4, 0x143c8d46c, 0x143c8d120, 0x143c8d528> as *const ()), // sub_14046b960
    Replacement::draft(0x14046dc30, "unknown_14046dc30", tpl_63557::<0x8, 0x19, 0x10, 0x140246bb0, 0x1401e66f0, 0x14024ed60> as *const ()), // sub_14046dc30
    Replacement::draft(0x14046e2f0, "unknown_14046e2f0", tpl_3648::<0x143c8d68c, 0x142489ee0, 0x143c8d708, 0x143c8d120, 0x143c964c0> as *const ()), // sub_14046e2f0
    Replacement::draft_throws(0x14046e420, "unknown_14046e420", tpl_1480::<0x14272c580, 0x1410d6ca0, 0x14046fdc0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14046e420
    Replacement::draft_throws(0x14046e6c0, "bigfish_14046e6c0", tpl_4388::<0x143ca3c88, 0x143ca3b50, 0x1411e9f50, 0x1404714e0, 0x143ca3b44, 0x1420af120, 0x1420a28e0, 0x140470db0, 0x140470620, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14313f566, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::systems::updateOffsets>
    Replacement::draft(0x14046e980, "bigfish_14046e980", tpl_4389::<0x143c8d68c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14046e980
    Replacement::draft(0x14046ecd0, "bigfish_14046ecd0", tpl_4390::<0x143c964c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14046ecd0
    Replacement::draft(0x14046f000, "bigfish_14046f000", tpl_4390::<0x143ca34f4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14046f000
    Replacement::draft(0x14046f330, "bigfish_14046f330", tpl_1098::<0x143c8d708, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14046f330
    Replacement::draft(0x14046f670, "bigfish_14046f670", tpl_1403::<0x143c8d4f0, 0x140470000, 0x14020e8f0> as *const ()), // sub_14046f670
    Replacement::draft(0x14046f850, "bigfish_14046f850", tpl_1094::<0x143c8d478, 0x140470180, 0x14020e8f0> as *const ()), // sub_14046f850
    Replacement::draft(0x14046fa50, "bigfish_14046fa50", fn_14046fa50 as *const ()), // sub_14046fa50
    Replacement::draft_throws(0x14046fdc0, "bigfish_14046fdc0", fn_14046fdc0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::systems::update>
    Replacement::draft(0x140470000, "bigfish_140470000", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143084cd0> as *const ()), // sub_140470000
    Replacement::draft(0x140470180, "bigfish_140470180", fn_140470180 as *const ()), // sub_140470180
    Replacement::draft(0x140470320, "bigfish_140470320", fn_140470320 as *const ()), // sub_140470320
    Replacement::draft(0x140470620, "bigfish_140470620", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140470710, 0x580f0, 0x1419eb880> as *const ()), // sub_140470620
    Replacement::draft(0x140470810, "bigfish_140470810", tpl_1955::<0x18, 0x1401de830> as *const ()), // sub_140470810
    Replacement::draft(0x140470840, "bigfish_140470840", fn_140470840 as *const ()), // sub_140470840
    Replacement::draft_throws(0x140470940, "bigfish_140470940", tpl_3790::<0x140471720, 0x143ca3be4, 0x1420af120, 0x1404715e0, 0x143ca3c2c, 0x140215750, 0x1403eeca0, 0x143c8cef8, 0x140215610, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_140470940
    Replacement::draft_throws(0x140470b30, "bigfish_140470b30", fn_140470b30 as *const (), Throws::Discard), // sub_140470b30
    Replacement::draft_throws(0x140470c90, "bigfish_140470c90", tpl_4373::<0x18, 0x20, 0x8, 0x10, 0x38, 0x28, 0x1419eb8f0> as *const (), Throws::Discard), // sub_140470c90
    Replacement::draft(0x140470db0, "bigfish_140470db0", tpl_3213::<0x142407c80, 0x143c964c0, 0x142489ee0, 0x143ca34f4, 0x1419eb880> as *const ()), // sub_140470db0
    Replacement::draft(0x140470f60, "bigfish_140470f60", tpl_4376::<0x143c8d478, 0x142489ee0, 0x143c8d120, 0x143c917cc, 0x143ca3424> as *const ()), // sub_140470f60
    Replacement::draft(0x1404710e0, "bigfish_1404710e0", fn_1404710e0 as *const ()), // sub_1404710e0
    Replacement::draft_throws(0x1404711a0, "bigfish_1404711a0", fn_1404711a0 as *const (), Throws::Discard), // sub_1404711a0
    Replacement::draft_throws(0x140471a10, "bigfish_140471a10", fn_140471a10 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::updateInteractableObjectSearch>
    Replacement::draft(0x140471e00, "unknown_140471e00", tpl_1097::<0x143ca3cfc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140471e00
    Replacement::draft_throws(0x140472130, "unknown_140472130", tpl_61549::<0x140472260, 0x1419ed590> as *const (), Throws::Discard), // sub_140472130
    Replacement::draft_throws(0x1404723c0, "unknown_1404723c0", tpl_61550::<0x140472260, 0x1419ed590> as *const (), Throws::Discard), // sub_1404723c0
    Replacement::draft(0x140472840, "unknown_140472840", fn_140472840 as *const ()), // sub_140472840
    Replacement::draft(0x1404728b0, "unknown_1404728b0", fn_1404728b0 as *const ()), // sub_1404728b0
    Replacement::draft(0x140472910, "unknown_140472910", fn_140472910 as *const ()), // sub_140472910
    Replacement::draft(0x140472950, "unknown_140472950", fn_140472950 as *const ()), // sub_140472950
    Replacement::draft(0x140472980, "unknown_140472980", fn_140472980 as *const ()), // sub_140472980
    Replacement::draft(0x140472ac0, "unknown_140472ac0", fn_140472ac0 as *const ()), // sub_140472ac0
    Replacement::draft_throws(0x140472c60, "unknown_140472c60", fn_140472c60 as *const (), Throws::Discard), // sub_140472c60
    Replacement::draft_throws(0x140472e20, "unknown_140472e20", tpl_4406::<0x1438491f8, 0x1401e8680, 0x1401eb860> as *const (), Throws::Discard), // sub_140472e20
    Replacement::draft_throws(0x140472f90, "unknown_140472f90", tpl_4406::<0x1438490d8, 0x1401e8680, 0x1401eb860> as *const (), Throws::Discard), // sub_140472f90
    Replacement::draft(0x140473100, "unknown_140473100", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143ca3e40, 0x1426467e8, 0x143c8d120, 0x142489ee0, 0x143c8f6d8, 0x1404731f3> as *const ()), // sub_140473100
    Replacement::draft_throws(0x140473370, "unknown_140473370", tpl_1480::<0x14272d740, 0x1410d6ca0, 0x1404752c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140473370
    Replacement::draft(0x140473610, "bigfish_140473610", fn_140473610 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::updateUnlockablesCount>
    Replacement::draft_throws(0x140473820, "bigfish_140473820", tpl_1396::<0x143ca3dec, 0x143ca3fb0, 0x1411e9f50, 0x140476430, 0x140476b50, 0x140476640, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431413e6, 0x142646960, 0x143ca3fd0, 0x143ca3fd8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::syncInspectPointData>
    Replacement::draft(0x140473a70, "bigfish_140473a70", fn_140473a70 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::syncUnlockData>
    Replacement::draft(0x140473c70, "bigfish_140473c70", tpl_4409::<0x143ca4084, 0x143ca4030, 0x1411e9f50, 0x1404767c0, 0x140476da0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143140fe6, 0x142646960, 0x143ca4050, 0x143ca4052, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::handleCleanup>
    Replacement::draft(0x140473eb0, "bigfish_140473eb0", fn_140473eb0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::setupShortcut>
    Replacement::draft(0x1404740b0, "bigfish_1404740b0", fn_1404740b0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::syncTooltipData>
    Replacement::draft_throws(0x1404742b0, "bigfish_1404742b0", tpl_1480::<0x14272d740, 0x1410d6ca0, 0x140475b00, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404742b0
    Replacement::draft_throws(0x140474550, "bigfish_140474550", tpl_1480::<0x14272d740, 0x1410d6ca0, 0x140475d50, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140474550
    Replacement::draft(0x1404747f0, "bigfish_1404747f0", tpl_1526::<0x14272d740, 0x1410d6ca0, 0x140475fa0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1404747f0
    Replacement::draft_throws(0x140474a90, "bigfish_140474a90", tpl_1480::<0x14272d740, 0x1410d6ca0, 0x1404761e0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140474a90
    Replacement::draft_throws(0x140474d30, "bigfish_140474d30", fn_140474d30 as *const (), Throws::Discard), // sub_140474d30
    Replacement::draft(0x140474eb0, "bigfish_140474eb0", tpl_60956::<0x19, 0x10, 0x140474eb0, 0x30, 0x1404728b0, 0x1401e66f0> as *const ()), // sub_140474eb0
    Replacement::draft(0x140474f30, "bigfish_140474f30", fn_140474f30 as *const ()), // sub_140474f30
    Replacement::draft(0x140474fb0, "bigfish_140474fb0", fn_140474fb0 as *const ()), // sub_140474fb0
    Replacement::draft(0x1404750b0, "bigfish_1404750b0", tpl_1095::<0x143ca4008, 0x14020e8f0> as *const ()), // sub_1404750b0
    Replacement::draft_throws(0x1404752c0, "bigfish_1404752c0", tpl_4415::<0x143ca4190, 0x143ca3f20, 0x1411e9f50, 0x140476940, 0x1404776b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143140786, 0x142646960, 0x143ca3f40, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::changeState>
    Replacement::draft(0x140475500, "bigfish_140475500", tpl_1765::<0x1411e9f50, 0x143c90168, 0x140215610, 0x143c9e074, 0x143c8ce88, 0x1420a28e0, 0x140476b10, 0x1401de410> as *const ()), // sub_140475500
    Replacement::draft(0x140475680, "bigfish_140475680", tpl_1765::<0x1411e9f50, 0x143c90168, 0x140215610, 0x143c9e074, 0x143c8ce88, 0x1420a28e0, 0x140476d60, 0x1401de410> as *const ()), // sub_140475680
    Replacement::draft(0x140475800, "bigfish_140475800", tpl_1765::<0x1411e9f50, 0x143c90168, 0x140215610, 0x143c9e074, 0x143c8ce88, 0x1420a28e0, 0x140476e00, 0x1401de410> as *const ()), // sub_140475800
    Replacement::draft(0x140475980, "bigfish_140475980", tpl_4416::<0x1411e9f50, 0x143c90168, 0x140215610, 0x143c8d0d4, 0x143c8ce88, 0x1420a28e0, 0x140476e40, 0x1401de410> as *const ()), // sub_140475980
    Replacement::draft_throws(0x140475b00, "bigfish_140475b00", tpl_1584::<0x143ca3da4, 0x143ca3df0, 0x1411e9f50, 0x140476e80, 0x140477800, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143140a46, 0x142646960, 0x143ca3e10, 0x143ca3e18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::syncInspectPositions>
    Replacement::draft_throws(0x140475d50, "bigfish_140475d50", tpl_1535::<0x143ca40c0, 0x143ca3f80, 0x1411e9f50, 0x140477020, 0x140477860, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143140d36, 0x142646960, 0x143ca3fa0, 0x143ca3fa8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::syncTooltipPosition>
    Replacement::draft(0x140475fa0, "bigfish_140475fa0", tpl_4409::<0x143ca4004, 0x143ca40f0, 0x1411e9f50, 0x140477170, 0x1404778b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431402c6, 0x142646960, 0x143ca4110, 0x143ca4112, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::handleRemoved>
    Replacement::draft_throws(0x1404761e0, "bigfish_1404761e0", tpl_4417::<0x143ca4188, 0x143ca3e98, 0x1411e9f50, 0x140477310, 0x1404779b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143140426, 0x142646960, 0x143ca3ea8, 0x143ca3eb0, 0x143ca3eb2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::inspect_ui::reset>
    Replacement::draft_throws(0x140476430, "unknown_140476430", fn_140476430 as *const (), Throws::Discard), // sub_140476430
    Replacement::draft_throws(0x140476640, "unknown_140476640", fn_140476640 as *const (), Throws::Discard), // sub_140476640
    Replacement::draft(0x1404767c0, "unknown_1404767c0", tpl_4419::<0x143c90168, 0x140215610, 0x143c90260, 0x143c9e074, 0x143c8ce88, 0x143ca4114, 0x1420a28e0> as *const ()), // sub_1404767c0
    Replacement::draft_throws(0x140476940, "unknown_140476940", fn_140476940 as *const (), Throws::Discard), // sub_140476940
    Replacement::draft_throws(0x140476b50, "unknown_140476b50", fn_140476b50 as *const (), Throws::Discard), // sub_140476b50
    Replacement::draft_throws(0x140476da0, "unknown_140476da0", fn_140476da0 as *const (), Throws::Discard), // sub_140476da0
    Replacement::draft_throws(0x140476e80, "unknown_140476e80", fn_140476e80 as *const (), Throws::Discard), // sub_140476e80
    Replacement::draft_throws(0x140477020, "unknown_140477020", fn_140477020 as *const (), Throws::Discard), // sub_140477020
    Replacement::draft(0x140477170, "unknown_140477170", fn_140477170 as *const ()), // sub_140477170
    Replacement::draft_throws(0x140477310, "unknown_140477310", tpl_4425::<0x140478140, 0x143ca4118, 0x1420af120, 0x140215750, 0x143c90260, 0x140215610, 0x143c90168, 0x1420a28e0> as *const (), Throws::Discard), // sub_140477310
    Replacement::draft_throws(0x140477490, "unknown_140477490", fn_140477490 as *const (), Throws::Discard), // sub_140477490
    Replacement::draft_throws(0x1404776b0, "unknown_1404776b0", fn_1404776b0 as *const (), Throws::Caught), // sub_1404776b0
    Replacement::draft_throws(0x140477800, "unknown_140477800", fn_140477800 as *const (), Throws::Discard), // sub_140477800
    Replacement::draft_throws(0x140477860, "unknown_140477860", fn_140477860 as *const (), Throws::Discard), // sub_140477860
    Replacement::draft_throws(0x1404778b0, "unknown_1404778b0", fn_1404778b0 as *const (), Throws::Caught), // sub_1404778b0
    Replacement::draft(0x1404779b0, "unknown_1404779b0", tpl_4431::<0x143ca4118, 0x20, 0x143c90260, 0x8, 0x580e0, 0x143c90168, 0x1419ee500> as *const ()), // sub_1404779b0
    Replacement::draft_throws(0x140477a30, "unknown_140477a30", tpl_4018::<0x143ca3e40, 0x143ca3e80, 0x143ca4010, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d120, 0x143c8f6d8, 0x143ca4020, 0x143ca4028, 0x141834a0c> as *const (), Throws::Discard), // sub_140477a30
    Replacement::draft(0x140477c80, "unknown_140477c80", tpl_1991::<0x1404781f0, 0x14313fe68, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140477c80
    Replacement::draft(0x140477cf0, "unknown_140477cf0", tpl_1992::<0x20, 0x28, 0x8, 0x1401de830> as *const ()), // sub_140477cf0
    Replacement::draft(0x140477d70, "unknown_140477d70", tpl_3665::<0x1403ea7a0, 0x14240aa00, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14240ab10, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_140477d70
    Replacement::draft(0x140477ee0, "unknown_140477ee0", fn_140477ee0 as *const ()), // sub_140477ee0
    Replacement::draft(0x1404781f0, "unknown_1404781f0", tpl_1997::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14313fe68, 0x8> as *const ()), // sub_1404781f0
    Replacement::draft(0x140478460, "unknown_140478460", tpl_4433::<0x19, 0x140474f30, 0x20, 0x140472980, 0x10, 0x18, 0x8, 0x140478460> as *const ()), // sub_140478460
    Replacement::draft(0x140478520, "unknown_140478520", tpl_1663::<0x8, 0x140474eb0, 0x1401e66f0> as *const ()), // sub_140478520
    Replacement::draft_throws(0x140478690, "unknown_140478690", tpl_1480::<0x14272e100, 0x1410d6ca0, 0x140479200, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_140478690
    Replacement::draft_throws(0x140478930, "unknown_140478930", tpl_1480::<0x14272e100, 0x1410d6ca0, 0x140479540, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_140478930
    Replacement::draft(0x140478bd0, "unknown_140478bd0", tpl_1094::<0x143ca4244, 0x140479d60, 0x14020e8f0> as *const ()), // sub_140478bd0
    Replacement::draft(0x140478dd0, "unknown_140478dd0", tpl_4434::<0x143ca4318, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140478dd0
    Replacement::draft(0x140479120, "unknown_140479120", tpl_1436::<0x58, 0x60, 0x158, 0x143ca4318, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140479120
    Replacement::draft_throws(0x140479200, "bigfish_140479200", fn_140479200 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::input_minigame::system::update>
    Replacement::draft_throws(0x140479540, "bigfish_140479540", tpl_4436::<0x143ca4424, 0x143ca4298, 0x140479f00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143142126, 0x142646960, 0x143ca42b8, 0x143ca42c0, 0x143ca42c4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::input_minigame::system::updatePrompt>
    Replacement::draft_throws(0x140479760, "bigfish_140479760", tpl_1438::<0x143ca424c, 0x143ca43c0, 0x1411e9f50, 0x14047b2e0, 0x143ca43ec, 0x1420af120, 0x1420a28e0, 0x14047ae70, 0x14047a500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143141f26, 0x142646960, 0x143ca43e0, 0x143ca43e8, 0x143ca43ea, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::input_minigame::stream_in::system>
    Replacement::draft_throws(0x140479a50, "bigfish_140479a50", tpl_1467::<0x143ca42d4, 0x143ca4390, 0x1411e9f50, 0x14047b1f0, 0x143ca42c8, 0x1420af120, 0x1420a28e0, 0x14047af80, 0x14047a5c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143141946, 0x142646960, 0x143ca43b0, 0x143ca43b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::input_minigame::stream_out::system>
    Replacement::draft(0x140479d30, "unknown_140479d30", tpl_1439::<0x8, 0x140478bd0> as *const ()), // sub_140479d30
    Replacement::draft(0x140479d60, "unknown_140479d60", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143141708, 0x14047a1d0> as *const ()), // sub_140479d60
    Replacement::draft_throws(0x140479f00, "unknown_140479f00", tpl_3011::<0x1411e9f50, 0x14047b3e0, 0x143ca42cc, 0x1420af120, 0x140215750, 0x143c8ce88, 0x140215610, 0x1420a28e0, 0x14047ae00, 0x1401de410> as *const (), Throws::Discard), // sub_140479f00
    Replacement::draft(0x14047a0c0, "content_14047a0c0", tpl_1953::<0x14047a120, 0x143084e28, 0x141834d30> as *const ()), // content::InputMinigameComponent::content::InputMinigameComponent::vfunc_0
    Replacement::draft(0x14047a1d0, "unknown_14047a1d0", fn_14047a1d0 as *const ()), // sub_14047a1d0
    Replacement::draft(0x14047a320, "unknown_14047a320", fn_14047a320 as *const ()), // sub_14047a320
    Replacement::draft_throws(0x14047a3a0, "unknown_14047a3a0", fn_14047a3a0 as *const (), Throws::Discard), // sub_14047a3a0
    Replacement::draft(0x14047a500, "unknown_14047a500", tpl_1126::<0x142407c80, 0x580f0, 0x1419f2fe0> as *const ()), // sub_14047a500
    Replacement::draft(0x14047a5c0, "unknown_14047a5c0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140479120> as *const ()), // sub_14047a5c0
    Replacement::draft(0x14047a980, "unknown_14047a980", fn_14047a980 as *const ()), // sub_14047a980
    Replacement::draft_throws(0x14047ab80, "unknown_14047ab80", fn_14047ab80 as *const (), Throws::Discard), // sub_14047ab80
    Replacement::draft_throws(0x14047ae00, "unknown_14047ae00", tpl_4439::<0x143ca42cc, 0x20, 0x8, 0x143c8ce88, 0x580e0, 0x1419f3210> as *const (), Throws::Discard), // sub_14047ae00
    Replacement::draft(0x14047ae70, "unknown_14047ae70", tpl_1130::<0x142407c80, 0x1419f2fe0> as *const ()), // sub_14047ae70
    Replacement::draft(0x14047af80, "unknown_14047af80", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140479120> as *const ()), // sub_14047af80
    Replacement::draft(0x14047b060, "unknown_14047b060", fn_14047b060 as *const ()), // sub_14047b060
    Replacement::draft(0x14047b1a0, "unknown_14047b1a0", fn_14047b1a0 as *const ()), // sub_14047b1a0
    Replacement::draft(0x14047b5e0, "unknown_14047b5e0", tpl_855::<0x14047a320, 0x1425371a0, 0x142533200, 0x14047a980, 0x14047b060> as *const ()), // sub_14047b5e0
    Replacement::draft(0x14047b980, "unknown_14047b980", fn_14047b980 as *const ()), // sub_14047b980
    Replacement::draft(0x14047ba10, "unknown_14047ba10", tpl_63667::<0x8, 0xc0, 0x30, 0x178, 0x140> as *const ()), // sub_14047ba10
    Replacement::draft(0x14047bab0, "unknown_14047bab0", tpl_4443::<0x24> as *const ()), // sub_14047bab0
    Replacement::draft(0x14047bac0, "unknown_14047bac0", tpl_4443::<0x8> as *const ()), // sub_14047bac0
    Replacement::draft(0x14047bad0, "unknown_14047bad0", fn_14047bad0 as *const ()), // sub_14047bad0
    Replacement::draft(0x14047bce0, "unknown_14047bce0", fn_14047bce0 as *const ()), // sub_14047bce0
    Replacement::draft_throws(0x14047bf60, "unknown_14047bf60", fn_14047bf60 as *const (), Throws::Discard), // sub_14047bf60
    Replacement::draft(0x14047c090, "unknown_14047c090", fn_14047c090 as *const ()), // sub_14047c090
    Replacement::draft(0x14047c100, "unknown_14047c100", tpl_2030::<0x14047c090, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14047c100
    Replacement::draft_throws(0x14047c1b0, "unknown_14047c1b0", fn_14047c1b0 as *const (), Throws::Discard), // sub_14047c1b0
    Replacement::draft(0x14047c2e0, "unknown_14047c2e0", fn_14047c2e0 as *const ()), // sub_14047c2e0
    Replacement::draft(0x14047c4f0, "unknown_14047c4f0", tpl_3942::<0x30, 0x143c8dc6c, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14047c540> as *const ()), // sub_14047c4f0
    Replacement::draft(0x14047c550, "unknown_14047c550", tpl_1168::<0x30, 0x143c8fab8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14047c5d0> as *const ()), // sub_14047c550
    Replacement::draft(0x14047c5e0, "unknown_14047c5e0", fn_14047c5e0 as *const ()), // sub_14047c5e0
    Replacement::draft_throws(0x14047c670, "unknown_14047c670", fn_14047c670 as *const (), Throws::Discard), // sub_14047c670
    Replacement::draft(0x14047c7b0, "unknown_14047c7b0", tpl_4452::<0x8, 0x1401f2bc0> as *const ()), // sub_14047c7b0
    Replacement::draft(0x14047c7f0, "unknown_14047c7f0", fn_14047c7f0 as *const ()), // sub_14047c7f0
    Replacement::draft(0x14047ca60, "unknown_14047ca60", fn_14047ca60 as *const ()), // sub_14047ca60
    Replacement::draft(0x14047cd70, "unknown_14047cd70", tpl_1363::<0x30, 0x143c95490, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14047cdbd> as *const ()), // sub_14047cd70
    Replacement::draft(0x14047cdc0, "unknown_14047cdc0", fn_14047cdc0 as *const ()), // sub_14047cdc0
    Replacement::draft(0x14047ce30, "unknown_14047ce30", fn_14047ce30 as *const ()), // sub_14047ce30
    Replacement::draft(0x14047cfd0, "unknown_14047cfd0", fn_14047cfd0 as *const ()), // sub_14047cfd0
    Replacement::draft(0x14047d330, "unknown_14047d330", fn_14047d330 as *const ()), // sub_14047d330
    Replacement::draft(0x14047d530, "unknown_14047d530", fn_14047d530 as *const ()), // sub_14047d530
    Replacement::draft(0x14047d920, "unknown_14047d920", fn_14047d920 as *const ()), // sub_14047d920
    Replacement::draft(0x14047da60, "unknown_14047da60", fn_14047da60 as *const ()), // sub_14047da60
    Replacement::draft(0x14047dbd0, "unknown_14047dbd0", fn_14047dbd0 as *const ()), // sub_14047dbd0
    Replacement::draft(0x14047dca0, "unknown_14047dca0", fn_14047dca0 as *const ()), // sub_14047dca0
    Replacement::draft_throws(0x14047dd90, "bigfish_14047dd90", fn_14047dd90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::impulse::update>
    Replacement::draft(0x14047e030, "bigfish_14047e030", fn_14047e030 as *const ()), // sub_14047e030
    Replacement::draft(0x14047e440, "bigfish_14047e440", fn_14047e440 as *const ()), // sub_14047e440
    Replacement::draft(0x14047e680, "bigfish_14047e680", fn_14047e680 as *const ()), // sub_14047e680
    Replacement::draft(0x14047ead0, "bigfish_14047ead0", fn_14047ead0 as *const ()), // sub_14047ead0
    Replacement::draft(0x14047ece0, "bigfish_14047ece0", fn_14047ece0 as *const ()), // sub_14047ece0
    Replacement::draft(0x14047f180, "bigfish_14047f180", tpl_4466::<0x19, 0x1401fc200, 0x1c, 0x10, 0x18, 0x8, 0x14047f180> as *const ()), // sub_14047f180
    Replacement::draft(0x14047f230, "bigfish_14047f230", tpl_4466::<0x19, 0x1401fc200, 0x20, 0x10, 0x18, 0x8, 0x14047f230> as *const ()), // sub_14047f230
    Replacement::draft(0x14047f2e0, "bigfish_14047f2e0", fn_14047f2e0 as *const ()), // sub_14047f2e0
    Replacement::draft(0x14047f430, "bigfish_14047f430", fn_14047f430 as *const ()), // sub_14047f430
    Replacement::draft_throws(0x14047f500, "bigfish_14047f500", tpl_3395::<0x143ca46a8, 0x143ca4728, 0x140480400, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431431e6, 0x142646960, 0x143ca4748, 0x143ca4750, 0x143ca4752, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::holstering::state_machine::system>
    Replacement::draft(0x14047f710, "bigfish_14047f710", tpl_1094::<0x143c970e0, 0x140480800, 0x14020e8f0> as *const ()), // sub_14047f710
    Replacement::draft(0x14047f910, "bigfish_14047f910", tpl_3354::<0x143c8e9b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14047f910
    Replacement::draft(0x14047fc60, "bigfish_14047fc60", tpl_3278::<0x143c8dbd0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14047fc60
    Replacement::draft(0x14047ffa0, "bigfish_14047ffa0", tpl_1094::<0x143c9deb0, 0x1404809a0, 0x14020e8f0> as *const ()), // sub_14047ffa0
    Replacement::draft(0x1404801a0, "bigfish_1404801a0", tpl_1094::<0x143c95468, 0x140480b30, 0x14020e8f0> as *const ()), // sub_1404801a0
    Replacement::draft_throws(0x140480400, "bigfish_140480400", tpl_3083::<0x1411e9f50, 0x140481920, 0x143ca4664, 0x1420af120, 0x140481710, 0x143ca4660, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x140481440, 0x140480cc0> as *const (), Throws::Discard), // sub_140480400
    Replacement::draft(0x140480630, "bigfish_140480630", fn_140480630 as *const ()), // sub_140480630
    Replacement::draft(0x140480800, "bigfish_140480800", fn_140480800 as *const ()), // sub_140480800
    Replacement::draft(0x1404809a0, "bigfish_1404809a0", tpl_4469::<0x1401e6d50, 0x140481410, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14041ede0> as *const ()), // sub_1404809a0
    Replacement::draft(0x140480b30, "bigfish_140480b30", tpl_4470::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1430bf490> as *const ()), // sub_140480b30
    Replacement::draft(0x140480cc0, "bigfish_140480cc0", fn_140480cc0 as *const ()), // sub_140480cc0
    Replacement::draft(0x140480dd0, "bigfish_140480dd0", fn_140480dd0 as *const ()), // sub_140480dd0
    Replacement::draft(0x140481410, "bigfish_140481410", tpl_1955::<0x10, 0x14041ed30> as *const ()), // sub_140481410
    Replacement::draft(0x140481440, "bigfish_140481440", fn_140481440 as *const ()), // sub_140481440
    Replacement::draft(0x140481710, "bigfish_140481710", fn_140481710 as *const ()), // sub_140481710
    Replacement::draft(0x140481920, "bigfish_140481920", fn_140481920 as *const ()), // sub_140481920
    Replacement::draft_throws(0x140481c80, "bigfish_140481c80", tpl_4475::<0x143ca4788, 0x143ca4758, 0x1411e9f50, 0x140482d80, 0x143ca4878, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x1404827b0, 0x140482490, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143144746, 0x142646960, 0x143ca4778, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::check_character_hidden::system>
    Replacement::draft_throws(0x140481f90, "bigfish_140481f90", fn_140481f90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::check_character_not_hidden::system>
    Replacement::draft(0x1404822b0, "unknown_1404822b0", tpl_1095::<0x143ca4784, 0x14020e8f0> as *const ()), // sub_1404822b0
    Replacement::draft(0x140482490, "unknown_140482490", tpl_4477::<0x142407c80, 0x143c8ceec, 0x1404826b0, 0x1419fc760> as *const ()), // sub_140482490
    Replacement::draft(0x1404825a0, "unknown_1404825a0", tpl_4477::<0x142407c80, 0x143c8ceec, 0x1404826b0, 0x1419fc600> as *const ()), // sub_1404825a0
    Replacement::draft(0x1404827b0, "unknown_1404827b0", tpl_4478::<0x142407c80, 0x143c8ceec, 0x143c8d87c, 0x142489ee0, 0x143c8d160, 0x1419fc760> as *const ()), // sub_1404827b0
    Replacement::draft(0x140482980, "unknown_140482980", tpl_4478::<0x142407c80, 0x143c8ceec, 0x143c8d87c, 0x142489ee0, 0x143c8d160, 0x1419fc600> as *const ()), // sub_140482980
    Replacement::draft(0x140482fe0, "unknown_140482fe0", fn_140482fe0 as *const ()), // sub_140482fe0
    Replacement::draft(0x140482ff0, "unknown_140482ff0", tpl_63595::<0x140483130, 0x1426467e8, 0x14048312e> as *const ()), // sub_140482ff0
    Replacement::draft_throws(0x140483360, "bigfish_140483360", tpl_3405::<0x143ca4a44, 0x143ca50b8, 0x1411e9f50, 0x14048b220, 0x143ca5138, 0x1420af120, 0x143c8ce88, 0x140215610, 0x1420a28e0, 0x1404884e0, 0x1404873f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143144ef6, 0x142646960, 0x143ca50d8, 0x143ca50e8, 0x143ca50ea, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::update_user_accessibility::system>
    Replacement::draft_throws(0x140483680, "bigfish_140483680", tpl_1535::<0x143ca49b4, 0x143ca5140, 0x1411e9f50, 0x140487500, 0x140488650, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143145096, 0x142646960, 0x143ca5160, 0x143ca5168, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::handle_requests::system>
    Replacement::draft_throws(0x1404838d0, "bigfish_1404838d0", tpl_3242::<0x143ca4d00, 0x143ca49b8, 0x1411e9f50, 0x14048aed0, 0x143ca4be0, 0x1420af120, 0x143c8ce88, 0x140215610, 0x1420a28e0, 0x1404886e0, 0x1404876b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431455b6, 0x142646960, 0x143ca49d8, 0x143ca49e0, 0x143ca49e4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::update_entity_facts::system>
    Replacement::draft_throws(0x140483bf0, "bigfish_140483bf0", fn_140483bf0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::heal_over_time::system>
    Replacement::draft_throws(0x140483f30, "bigfish_140483f30", tpl_4481::<0x143ca4c3c, 0x143ca5110, 0x1411e9f50, 0x14048ab80, 0x143ca4eb0, 0x1420af120, 0x143c8d910, 0x140215610, 0x143c8ceec, 0x1420a28e0, 0x140488aa0, 0x1404878d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143145926, 0x142646960, 0x143ca5130, 0x143ca5134, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::regeneration::system>
    Replacement::draft_throws(0x140484280, "bigfish_140484280", tpl_4130::<0x143ca4ba0, 0x143ca4a80, 0x1411e9f50, 0x14048aa90, 0x143ca510c, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x140488d30, 0x140487b20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143145e16, 0x142646960, 0x143ca4aa0, 0x143ca4aa8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::accumulated_damage::system>
    Replacement::draft_throws(0x140484590, "bigfish_140484590", tpl_1480::<0x142739920, 0x1410d6ca0, 0x140486a10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140484590
    Replacement::draft_throws(0x140484830, "bigfish_140484830", fn_140484830 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::health_condition_update::system>
    Replacement::draft(0x140484b80, "bigfish_140484b80", fn_140484b80 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::health::clean_up::system>
    Replacement::draft(0x140484e60, "bigfish_140484e60", tpl_1094::<0x143c9db0c, 0x140487210, 0x14020e8f0> as *const ()), // sub_140484e60
    Replacement::draft(0x140485060, "bigfish_140485060", tpl_3270::<0x143c8e70c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140485060
    Replacement::draft(0x1404853a0, "bigfish_1404853a0", tpl_3270::<0x143ca4c40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404853a0
    Replacement::draft(0x1404856e0, "bigfish_1404856e0", tpl_1099::<0x143ca51ac, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404856e0
    Replacement::draft(0x140485a20, "bigfish_140485a20", tpl_1098::<0x143ca50ec, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140485a20
    Replacement::draft(0x140485d60, "bigfish_140485d60", tpl_1369::<0x143c9779c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140485d60
    Replacement::draft(0x1404860a0, "bigfish_1404860a0", tpl_1098::<0x143ca5004, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404860a0
    Replacement::draft(0x1404863e0, "bigfish_1404863e0", tpl_3278::<0x143c97e70, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404863e0
    Replacement::draft(0x140486720, "bigfish_140486720", tpl_1095::<0x143ca4d5c, 0x14020e8f0> as *const ()), // sub_140486720
    Replacement::draft(0x140486900, "bigfish_140486900", tpl_1436::<0x58, 0x60, 0x158, 0x143ca4d5c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140486900
    Replacement::draft_throws(0x140486a10, "bigfish_140486a10", tpl_4484::<0x143ca5074, 0x143ca4c48, 0x1411e9f50, 0x140488ea0, 0x14048a330, 0x140489040, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431463c6, 0x142646960, 0x143ca4c58, 0x143ca4c60, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::ui::system>
    Replacement::draft_throws(0x140486c60, "bigfish_140486c60", tpl_1078::<0x143ca4b28, 0x143ca4b58, 0x1411e9f50, 0x14048b520, 0x143ca4a3c, 0x1420af120, 0x1420a28e0, 0x14048a5e0, 0x140489550, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143146c86, 0x142646960, 0x143ca4b78, 0x143ca4b7a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::stream_in::system>
    Replacement::draft_throws(0x140486f40, "bigfish_140486f40", tpl_4485::<0x143ca4de0, 0x143ca4968, 0x1411e9f50, 0x14048b430, 0x143ca4a38, 0x1420af120, 0x1420a28e0, 0x14048a790, 0x140489680, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143146f76, 0x142646960, 0x143ca4988, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::health::stream_out::system>
    Replacement::draft(0x140487210, "unknown_140487210", fn_140487210 as *const ()), // sub_140487210
    Replacement::draft(0x1404873f0, "unknown_1404873f0", tpl_60901::<0x143c8ce88, 0x580e0, 0x143c8e70c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1419fd900> as *const ()), // sub_1404873f0
    Replacement::draft_throws(0x140487500, "unknown_140487500", tpl_4487::<0x14048afc0, 0x143ca4b2c, 0x1420af120, 0x140215750, 0x143c8ceec, 0x140215610, 0x143c94b38, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_140487500
    Replacement::draft_throws(0x1404876b0, "unknown_1404876b0", tpl_62718::<0x88, 0x10088, 0x143c8ce88, 0x580e0, 0x1404883e0, 0x1419fd9b0> as *const (), Throws::Discard), // sub_1404876b0
    Replacement::draft(0x1404877b0, "unknown_1404877b0", tpl_63708::<0x88, 0x10088, 0x580e0, 0x143c8d910, 0x143c94b38, 0x1404882e0, 0x1419fd280> as *const ()), // sub_1404877b0
    Replacement::draft_throws(0x1404878d0, "unknown_1404878d0", fn_1404878d0 as *const (), Throws::Discard), // sub_1404878d0
    Replacement::draft_throws(0x1404879a0, "unknown_1404879a0", fn_1404879a0 as *const (), Throws::Discard), // sub_1404879a0
    Replacement::draft(0x140487b20, "unknown_140487b20", tpl_60901::<0x143c8d910, 0x580e0, 0x143ca51ac, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1419fdcb0> as *const ()), // sub_140487b20
    Replacement::draft(0x140487c30, "unknown_140487c30", tpl_1917::<0x143c8ceec, 0x143c8d910, 0x140487e40, 0x140487d40> as *const ()), // sub_140487c30
    Replacement::draft(0x140487d40, "unknown_140487d40", tpl_1918::<0x18, 0x20, 0x40, 0x8, 0x1419fd0c0> as *const ()), // sub_140487d40
    Replacement::draft(0x1404884e0, "unknown_1404884e0", tpl_60916::<0x143c8ce88, 0x143c8e70c, 0x142489ee0, 0x1419fd900> as *const ()), // sub_1404884e0
    Replacement::draft_throws(0x140488650, "unknown_140488650", tpl_4147::<0x143ca4b2c, 0x143c94b38, 0x20, 0x143c8ceec, 0x8, 0x580e0, 0x143c8d08c, 0x1419fd370> as *const (), Throws::Discard), // sub_140488650
    Replacement::draft_throws(0x1404886e0, "unknown_1404886e0", tpl_62736::<0x143c8ce88, 0x143c8d160, 0x142489ee0, 0x143c8e70c, 0x1419fd9b0> as *const (), Throws::Discard), // sub_1404886e0
    Replacement::draft(0x1404888a0, "unknown_1404888a0", tpl_63711::<0x143c8d910, 0x143c94b38, 0x143ca5004, 0x142489ee0, 0x143c8d160, 0x1419fd280> as *const ()), // sub_1404888a0
    Replacement::draft_throws(0x140488aa0, "unknown_140488aa0", fn_140488aa0 as *const (), Throws::Discard), // sub_140488aa0
    Replacement::draft(0x140488d30, "unknown_140488d30", tpl_60916::<0x143c8d910, 0x143ca51ac, 0x142489ee0, 0x1419fdcb0> as *const ()), // sub_140488d30
    Replacement::draft_throws(0x140488ea0, "unknown_140488ea0", fn_140488ea0 as *const (), Throws::Discard), // sub_140488ea0
    Replacement::draft_throws(0x140489040, "unknown_140489040", fn_140489040 as *const (), Throws::Discard), // sub_140489040
    Replacement::draft_throws(0x140489200, "unknown_140489200", fn_140489200 as *const (), Throws::Discard), // sub_140489200
    Replacement::draft(0x140489330, "unknown_140489330", tpl_61557::<0x143c8ceec, 0x143c8d910, 0x140487e40, 0x1419fd0c0> as *const ()), // sub_140489330
    Replacement::draft(0x140489550, "unknown_140489550", tpl_4492::<0x142407c80, 0x1404899a0, 0x1419fe870> as *const ()), // sub_140489550
    Replacement::draft(0x140489680, "unknown_140489680", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140486900> as *const ()), // sub_140489680
    Replacement::draft(0x140489720, "unknown_140489720", tpl_3172::<0x143c9779c, 0x5e308, 0x143c8ceec, 0x580e0, 0x10, 0x8, 0x18, 0x1419fe6b0, 0x140489dd0> as *const ()), // sub_140489720
    Replacement::draft(0x1404897d0, "unknown_1404897d0", tpl_3188::<0x14048a870, 0x143144ca0, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1404897d0
    Replacement::draft(0x140489ca0, "unknown_140489ca0", fn_140489ca0 as *const ()), // sub_140489ca0
    Replacement::draft(0x140489dd0, "unknown_140489dd0", tpl_4495::<0x1401de830, 0x18, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x1425371a0, 0x142533200> as *const ()), // sub_140489dd0
    Replacement::draft_throws(0x14048a330, "unknown_14048a330", fn_14048a330 as *const (), Throws::Discard), // sub_14048a330
    Replacement::draft(0x14048a5e0, "unknown_14048a5e0", tpl_4496::<0x142407c80, 0x1404899a0, 0x1419fe870> as *const ()), // sub_14048a5e0
    Replacement::draft(0x14048a790, "unknown_14048a790", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140486900> as *const ()), // sub_14048a790
    Replacement::draft(0x14048a870, "unknown_14048a870", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143144ca0, 0x8, 0x20> as *const ()), // sub_14048a870
    Replacement::draft(0x14048ab80, "unknown_14048ab80", fn_14048ab80 as *const ()), // sub_14048ab80
    Replacement::draft(0x14048afc0, "unknown_14048afc0", fn_14048afc0 as *const ()), // sub_14048afc0
    Replacement::draft(0x14048b310, "unknown_14048b310", tpl_4170::<0x1402c0a70, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_14048b310
    Replacement::draft(0x14048be00, "unknown_14048be00", tpl_1517::<0x60, 0x14240ab10> as *const ()), // sub_14048be00
    Replacement::draft(0x14048be30, "unknown_14048be30", fn_14048be30 as *const ()), // sub_14048be30
    Replacement::draft(0x14048bf30, "unknown_14048bf30", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143ca54fc, 0x1426467e8, 0x143c98790, 0x142489ee0, 0x143c8f4cc, 0x14048c023> as *const ()), // sub_14048bf30
    Replacement::draft_throws(0x14048c1a0, "unknown_14048c1a0", fn_14048c1a0 as *const (), Throws::Caught), // sub_14048c1a0
    Replacement::draft_throws(0x14048c3f0, "bigfish_14048c3f0", tpl_1298::<0x143ca54b8, 0x143ca56c0, 0x1411e9f50, 0x14048db20, 0x14048ea40, 0x14048ddf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143148b06, 0x142646960, 0x143ca56e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::healing_system::update::system>
    Replacement::draft_throws(0x14048c630, "bigfish_14048c630", tpl_1480::<0x14273a220, 0x1410d6ca0, 0x14048d430, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14048c630
    Replacement::draft_throws(0x14048c8d0, "bigfish_14048c8d0", tpl_1480::<0x14273a220, 0x1410d6ca0, 0x14048d670, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14048c8d0
    Replacement::draft(0x14048cb70, "bigfish_14048cb70", tpl_1403::<0x143ca56ec, 0x14048d9a0, 0x14020e8f0> as *const ()), // sub_14048cb70
    Replacement::draft(0x14048cd50, "bigfish_14048cd50", tpl_1098::<0x143c97a20, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14048cd50
    Replacement::draft(0x14048d090, "bigfish_14048d090", fn_14048d090 as *const ()), // sub_14048d090
    Replacement::draft_throws(0x14048d430, "bigfish_14048d430", tpl_3005::<0x143ca54ac, 0x143ca55b0, 0x1411e9f50, 0x14048ed80, 0x14048fd50, 0x14048f0b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143149f56, 0x142646960, 0x143ca55d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::healing_system::ui::system>
    Replacement::draft_throws(0x14048d670, "bigfish_14048d670", fn_14048d670 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::healing_system::ui::prompt::system>
    Replacement::draft(0x14048d9a0, "unknown_14048d9a0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431488f0> as *const ()), // sub_14048d9a0
    Replacement::draft_throws(0x14048db20, "unknown_14048db20", fn_14048db20 as *const (), Throws::Discard), // sub_14048db20
    Replacement::draft_throws(0x14048ddf0, "unknown_14048ddf0", fn_14048ddf0 as *const (), Throws::Discard), // sub_14048ddf0
    Replacement::draft(0x14048df00, "unknown_14048df00", tpl_3359::<0x143c8ced8, 0x20, 0x143ca56e8, 0x8, 0x143c8d910, 0x143c91068, 0x143c8ceec, 0x143c94b38, 0x143c98648, 0x143c8ef20, 0x10, 0x18> as *const ()), // sub_14048df00
    Replacement::draft_throws(0x14048e000, "unknown_14048e000", fn_14048e000 as *const (), Throws::Discard), // sub_14048e000
    Replacement::draft(0x14048e220, "unknown_14048e220", tpl_3321::<0x8, 0x18, 0x10, 0x14048e330, 0x80, 0x20, 0x40, 0x60, 0xa0, 0x140397b80, 0xc0> as *const ()), // sub_14048e220
    Replacement::draft(0x14048e330, "unknown_14048e330", tpl_62682::<0x143c96de4, 0x142489ee0, 0x143c93b20, 0x143c8e840, 0x143c8d474, 0x143c8d508, 0x143c8e70c, 0x143c9db0c, 0x143c8d160, 0x143c8d120, 0x143c97794, 0x143c8e9b0, 0x143c90164, 0x143c8ec68, 0x143c9ecdc, 0x143ca55e8, 0x143ca5314, 0x143c8dbd0, 0x143c97a20, 0x143ca5004, 0x143c8e670, 0x143c8ec28, 0x143c8dbc4, 0x143c8de68, 0x143c9f300> as *const ()), // sub_14048e330
    Replacement::draft(0x14048e8a0, "unknown_14048e8a0", tpl_61197::<0x143c98790, 0x142489ee0, 0x143c91064, 0x143c8f4cc, 0x143c8d160, 0x1426467e8, 0x14048ea33> as *const ()), // sub_14048e8a0
    Replacement::draft_throws(0x14048ea40, "unknown_14048ea40", fn_14048ea40 as *const (), Throws::Discard), // sub_14048ea40
    Replacement::draft_throws(0x14048ed80, "unknown_14048ed80", fn_14048ed80 as *const (), Throws::Discard), // sub_14048ed80
    Replacement::draft_throws(0x14048f0b0, "unknown_14048f0b0", fn_14048f0b0 as *const (), Throws::Discard), // sub_14048f0b0
    Replacement::draft_throws(0x14048f220, "unknown_14048f220", fn_14048f220 as *const (), Throws::Discard), // sub_14048f220
    Replacement::draft_throws(0x14048f390, "unknown_14048f390", fn_14048f390 as *const (), Throws::Discard), // sub_14048f390
    Replacement::draft_throws(0x14048f490, "unknown_14048f490", fn_14048f490 as *const (), Throws::Discard), // sub_14048f490
    Replacement::draft(0x1404913f0, "unknown_1404913f0", fn_1404913f0 as *const ()), // sub_1404913f0
    Replacement::draft(0x1404917f0, "unknown_1404917f0", fn_1404917f0 as *const ()), // sub_1404917f0
    Replacement::draft_throws(0x140491810, "bigfish_140491810", tpl_1428::<0x143ca5928, 0x143ca59a0, 0x1411e9f50, 0x140492e50, 0x140493b00, 0x140493110, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314c2b6, 0x142646960, 0x143ca59c0, 0x143ca59c4, 0x143ca59c6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::haptic_sensor::update::system>
    Replacement::draft(0x140491a60, "bigfish_140491a60", fn_140491a60 as *const ()), // sub_140491a60
    Replacement::draft(0x140491c70, "bigfish_140491c70", fn_140491c70 as *const ()), // sub_140491c70
    Replacement::draft(0x140491c90, "bigfish_140491c90", tpl_3080::<0x143c9eac0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140491c90
    Replacement::draft(0x140491fd0, "bigfish_140491fd0", tpl_1436::<0x58, 0x60, 0x158, 0x143c9eac0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140491fd0
    Replacement::draft(0x1404920b0, "bigfish_1404920b0", tpl_1094::<0x143ca5aa8, 0x140492ab0, 0x14020e8f0> as *const ()), // sub_1404920b0
    Replacement::draft(0x1404922b0, "bigfish_1404922b0", tpl_1094::<0x143ca5aac, 0x140492c90, 0x14020e8f0> as *const ()), // sub_1404922b0
    Replacement::draft_throws(0x1404924b0, "bigfish_1404924b0", tpl_1437::<0x143ca5914, 0x143ca5878, 0x1411e9f50, 0x140494a60, 0x143ca5910, 0x1420af120, 0x1420a28e0, 0x1404940d0, 0x140493850, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314cea6, 0x142646960, 0x143ca5898, 0x143ca58a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::haptic_sensor::stream_in::system>
    Replacement::draft_throws(0x140492790, "bigfish_140492790", tpl_1438::<0x143ca5794, 0x143ca5a78, 0x1411e9f50, 0x140494970, 0x143ca591c, 0x1420af120, 0x1420a28e0, 0x1404942a0, 0x140493a60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314cd06, 0x142646960, 0x143ca5a98, 0x143ca5aa0, 0x143ca5aa2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::haptic_sensor::stream_out::system>
    Replacement::draft(0x140492ab0, "unknown_140492ab0", fn_140492ab0 as *const ()), // sub_140492ab0
    Replacement::draft(0x140492c90, "unknown_140492c90", fn_140492c90 as *const ()), // sub_140492c90
    Replacement::draft_throws(0x140492e50, "unknown_140492e50", fn_140492e50 as *const (), Throws::Discard), // sub_140492e50
    Replacement::draft_throws(0x140493110, "unknown_140493110", fn_140493110 as *const (), Throws::Discard), // sub_140493110
    Replacement::draft(0x1404931f0, "unknown_1404931f0", tpl_4519::<0x8, 0x20, 0x143ca5958, 0x143ca5950, 0x143ca5ae8, 0x143c8d910, 0x580e0> as *const ()), // sub_1404931f0
    Replacement::draft_throws(0x1404932e0, "unknown_1404932e0", fn_1404932e0 as *const (), Throws::Discard), // sub_1404932e0
    Replacement::draft(0x1404937d0, "unknown_1404937d0", tpl_1124::<0x38> as *const ()), // sub_1404937d0
    Replacement::draft(0x140493800, "unknown_140493800", tpl_4521::<0x7, 0x20, 0x10, 0x1401de830> as *const ()), // sub_140493800
    Replacement::draft(0x140493850, "unknown_140493850", tpl_4522::<0x88, 0x10088, 0x142407c80, 0x180ac, 0x143ca5aa8, 0x18098, 0x142489ee0, 0xc03, 0x40, 0x140493960> as *const ()), // sub_140493850
    Replacement::draft(0x140493960, "unknown_140493960", fn_140493960 as *const ()), // sub_140493960
    Replacement::draft(0x140493a60, "unknown_140493a60", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140491fd0> as *const ()), // sub_140493a60
    Replacement::draft_throws(0x140493b00, "unknown_140493b00", fn_140493b00 as *const (), Throws::Discard), // sub_140493b00
    Replacement::draft(0x1404940d0, "unknown_1404940d0", fn_1404940d0 as *const ()), // sub_1404940d0
    Replacement::draft(0x1404942a0, "unknown_1404942a0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140491fd0> as *const ()), // sub_1404942a0
    Replacement::draft(0x1404946f0, "unknown_1404946f0", fn_1404946f0 as *const ()), // sub_1404946f0
    Replacement::draft(0x140494da0, "unknown_140494da0", fn_140494da0 as *const ()), // sub_140494da0
    Replacement::draft(0x140495050, "navigation_140495050", tpl_1995::<0x14314f768, 0x1401de830, 0x141834d30> as *const ()), // navigation::ExtendedTraversalLogic::navigation::ExtendedTraversalLogic::vfunc_0
    Replacement::draft(0x1404950e0, "unknown_1404950e0", fn_1404950e0 as *const ()), // sub_1404950e0
    Replacement::draft_throws(0x140495190, "bigfish_140495190", tpl_4527::<0x143ca5ec0, 0x143ca5e80, 0x1411e9f50, 0x1404965d0, 0x1404971e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314e286, 0x142646960, 0x143ca5ea0, 0x143ca5eb0, 0x143ca5eb8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::game_specific_navigation::query_removed::system>
    Replacement::draft_throws(0x1404953e0, "bigfish_1404953e0", tpl_1074::<0x143ca5afc, 0x143ca5cf8, 0x1411e9f50, 0x140498160, 0x143ca5b84, 0x1420af120, 0x1420a28e0, 0x1404972f0, 0x140496770, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314e436, 0x142646960, 0x143ca5d18, 0x143ca5d28, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::game_specific_navigation::update::system>
    Replacement::draft_throws(0x1404956c0, "bigfish_1404956c0", tpl_4528::<0x143ca5b98, 0x143ca5cb0, 0x1411e9f50, 0x1404968a0, 0x140497490, 0x140496a90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314ea86, 0x142646960, 0x143ca5cd0, 0x143ca5ce0, 0x143ca5ce8, 0x143ca5cec, 0x143ca5cee, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::game_specific_navigation::update_post_physics::system>
    Replacement::draft(0x140495930, "bigfish_140495930", fn_140495930 as *const ()), // bigfish::component::GameExtendedTraversalLogic::bigfish::component::GameExtendedTraversalLogic::vfunc_0
    Replacement::draft(0x140495980, "bigfish_140495980", tpl_1094::<0x143ca5f00, 0x140366c40, 0x14020e8f0> as *const ()), // sub_140495980
    Replacement::draft(0x140495b80, "bigfish_140495b80", tpl_1436::<0x58, 0x60, 0x158, 0x143ca5f00, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140495b80
    Replacement::draft(0x140495c60, "bigfish_140495c60", tpl_1403::<0x143ca5cf4, 0x140496450, 0x14020e8f0> as *const ()), // sub_140495c60
    Replacement::draft_throws(0x140495e70, "bigfish_140495e70", tpl_3407::<0x143ca5b3c, 0x143ca5b00, 0x1411e9f50, 0x1404984e0, 0x143ca5b80, 0x1420af120, 0x1420a28e0, 0x140497d30, 0x140497040, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314e676, 0x142646960, 0x143ca5b20, 0x143ca5b30, 0x143ca5b34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::game_specific_navigation::stream_in::system>
    Replacement::draft_throws(0x140496160, "bigfish_140496160", tpl_3068::<0x143ca5ba0, 0x143ca5d68, 0x1411e9f50, 0x1404983f0, 0x143ca5e5c, 0x1420af120, 0x1420a28e0, 0x140497e90, 0x140497150, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14314e8d6, 0x142646960, 0x143ca5d88, 0x143ca5d98, 0x143ca5d9c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::game_specific_navigation::stream_out::system>
    Replacement::draft(0x140496450, "unknown_140496450", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14314f7f0> as *const ()), // sub_140496450
    Replacement::draft_throws(0x1404965d0, "unknown_1404965d0", tpl_4530::<0x140497690, 0x143ca5f00, 0x1401fc320, 0x1404982a0, 0x143ca5b44, 0x1420af120, 0x140215750, 0x143c8fd28, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404965d0
    Replacement::draft_throws(0x140496770, "unknown_140496770", tpl_63752::<0x88, 0x10088, 0x140496df0, 0x141a06810> as *const (), Throws::Discard), // sub_140496770
    Replacement::draft_throws(0x1404968a0, "unknown_1404968a0", tpl_1199::<0x140498020, 0x143ca5b40, 0x1420af120, 0x140497f70, 0x143ca5b90, 0x140215750, 0x143c8d08c, 0x140215610, 0x143c8fd28, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404968a0
    Replacement::draft_throws(0x140496a90, "unknown_140496a90", fn_140496a90 as *const (), Throws::Discard), // sub_140496a90
    Replacement::draft_throws(0x140496ba0, "unknown_140496ba0", fn_140496ba0 as *const (), Throws::Discard), // sub_140496ba0
    Replacement::draft(0x140497040, "unknown_140497040", tpl_1127::<0x142407c80, 0x143c8ff58, 0x142489ee0, 0x141a06ba0> as *const ()), // sub_140497040
    Replacement::draft(0x140497150, "unknown_140497150", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x140495b80> as *const ()), // sub_140497150
    Replacement::draft(0x1404971e0, "unknown_1404971e0", fn_1404971e0 as *const ()), // sub_1404971e0
    Replacement::draft_throws(0x1404972f0, "unknown_1404972f0", tpl_63756::<0x140496df0, 0x141a06810> as *const (), Throws::Discard), // sub_1404972f0
    Replacement::draft_throws(0x140497490, "unknown_140497490", fn_140497490 as *const (), Throws::Discard), // sub_140497490
    Replacement::draft(0x140497690, "unknown_140497690", tpl_3188::<0x140498350, 0x14314f7c8, 0x140497a00, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140497690
    Replacement::draft(0x140497710, "unknown_140497710", tpl_4348::<0x20, 0x28, 0x140497a00> as *const ()), // sub_140497710
    Replacement::draft(0x1404978b0, "unknown_1404978b0", fn_1404978b0 as *const ()), // sub_1404978b0
    Replacement::draft(0x140497a00, "unknown_140497a00", fn_140497a00 as *const ()), // sub_140497a00
    Replacement::draft(0x140497d30, "unknown_140497d30", tpl_1131::<0x142407c80, 0x143c8ff58, 0x142489ee0, 0x141a06ba0> as *const ()), // sub_140497d30
    Replacement::draft(0x140497e90, "unknown_140497e90", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140495b80> as *const ()), // sub_140497e90
    Replacement::draft(0x140498350, "unknown_140498350", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14314f7c8, 0x8, 0x20> as *const ()), // sub_140498350
    Replacement::draft(0x1404983f0, "unknown_1404983f0", tpl_4539::<0x143ca5cf0, 0x143ca5e60, 0x8, 0x10, 0x18, 0x141834a78, 0x143c8d694, 0x143ca5f00, 0x143ca5e70, 0x143ca5e78, 0x141834a0c> as *const ()), // sub_1404983f0
    Replacement::draft(0x1404984e0, "unknown_1404984e0", fn_1404984e0 as *const ()), // sub_1404984e0
    Replacement::draft_throws(0x140498610, "unknown_140498610", tpl_4541::<0x143ca5b90, 0x140497f70, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140498610
    Replacement::draft_throws(0x1404986a0, "unknown_1404986a0", tpl_4541::<0x143ca5b40, 0x140498020, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1404986a0
    Replacement::draft_throws(0x140498730, "unknown_140498730", tpl_4541::<0x143ca5b84, 0x140498160, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140498730
    Replacement::draft_throws(0x1404987c0, "unknown_1404987c0", tpl_4541::<0x143ca5b44, 0x1404982a0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1404987c0
    Replacement::draft_throws(0x140498850, "unknown_140498850", tpl_4541::<0x143ca5e5c, 0x1404983f0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140498850
    Replacement::draft_throws(0x1404988e0, "unknown_1404988e0", tpl_4541::<0x143ca5b80, 0x1404984e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1404988e0
    Replacement::draft(0x140498970, "unknown_140498970", fn_140498970 as *const ()), // sub_140498970
    Replacement::draft_throws(0x140498990, "unknown_140498990", fn_140498990 as *const (), Throws::Discard), // sub_140498990
    Replacement::draft(0x1404989e0, "unknown_1404989e0", tpl_4544::<0x120> as *const ()), // sub_1404989e0
    Replacement::draft(0x1404989f0, "ak_1404989f0", tpl_1691::<0x14> as *const ()), // AK::WriteBytesBuffer::AK::WriteBytesBuffer::GetPos
    Replacement::draft(0x140498a00, "unknown_140498a00", tpl_4545::<0x1401de3c0> as *const ()), // sub_140498a00
    Replacement::draft(0x140498a20, "unknown_140498a20", fn_140498a20 as *const ()), // sub_140498a20
    Replacement::draft(0x140498bd0, "unknown_140498bd0", tpl_4547::<0x140499130> as *const ()), // sub_140498bd0
    Replacement::draft(0x140498bf0, "unknown_140498bf0", tpl_4547::<0x140498f90> as *const ()), // sub_140498bf0
    Replacement::draft(0x140498c10, "unknown_140498c10", fn_140498c10 as *const ()), // sub_140498c10
    Replacement::draft(0x140498cc0, "unknown_140498cc0", tpl_4547::<0x1404994f0> as *const ()), // sub_140498cc0
    Replacement::draft(0x140498ce0, "unknown_140498ce0", fn_140498ce0 as *const ()), // sub_140498ce0
    Replacement::draft(0x140498d60, "unknown_140498d60", fn_140498d60 as *const ()), // sub_140498d60
    Replacement::draft(0x140498dd0, "unknown_140498dd0", fn_140498dd0 as *const ()), // sub_140498dd0
    Replacement::draft(0x140498e30, "unknown_140498e30", fn_140498e30 as *const ()), // sub_140498e30
    Replacement::draft(0x140498e90, "unknown_140498e90", tpl_4553 as *const ()), // sub_140498e90
    Replacement::draft(0x140498ea0, "unknown_140498ea0", fn_140498ea0 as *const ()), // sub_140498ea0
    Replacement::draft(0x140498ed0, "unknown_140498ed0", tpl_4555::<0x8, 0x1404993c0> as *const ()), // sub_140498ed0
    Replacement::draft(0x140498f30, "unknown_140498f30", fn_140498f30 as *const ()), // sub_140498f30
    Replacement::draft(0x140498f90, "unknown_140498f90", fn_140498f90 as *const ()), // sub_140498f90
    Replacement::draft_throws(0x140499010, "std_140499010", fn_140499010 as *const (), Throws::Discard), // sub_140499010
    Replacement::draft(0x1404990d0, "std_1404990d0", tpl_4555::<0x8, 0x140499440> as *const ()), // sub_1404990d0
    Replacement::draft(0x140499130, "std_140499130", fn_140499130 as *const ()), // sub_140499130
    Replacement::draft(0x140499180, "std_140499180", fn_140499180 as *const ()), // sub_140499180
    Replacement::draft(0x140499240, "std_140499240", fn_140499240 as *const ()), // sub_140499240
    Replacement::draft(0x140499280, "std_140499280", fn_140499280 as *const ()), // sub_140499280
    Replacement::draft(0x1404992e0, "std_1404992e0", fn_1404992e0 as *const ()), // sub_1404992e0
    Replacement::draft(0x1404993c0, "std_1404993c0", tpl_4564::<0x1430782b0, 0x1401de2f0, 0x14049c930, 0x1401de410> as *const ()), // sub_1404993c0
    Replacement::draft(0x140499440, "std_140499440", tpl_4564::<0x1430782b0, 0x1401de2f0, 0x14049ca20, 0x1401de410> as *const ()), // sub_140499440
    Replacement::draft(0x1404994c0, "std_1404994c0", fn_1404994c0 as *const ()), // sub_1404994c0
    Replacement::draft(0x1404994f0, "std_1404994f0", tpl_4547::<0x14049d680> as *const ()), // sub_1404994f0
    Replacement::draft(0x140499510, "std_140499510", fn_140499510 as *const ()), // sub_140499510
    Replacement::draft(0x140499560, "std_140499560", fn_140499560 as *const ()), // sub_140499560
    Replacement::draft(0x1404995b0, "std_1404995b0", fn_1404995b0 as *const ()), // sub_1404995b0
    Replacement::draft_throws(0x140499670, "std_140499670", fn_140499670 as *const (), Throws::Caught), // sub_140499670
    Replacement::draft_throws(0x140499710, "std_140499710", fn_140499710 as *const (), Throws::Discard), // sub_140499710
    Replacement::draft_throws(0x140499870, "std_140499870", fn_140499870 as *const (), Throws::Caught), // sub_140499870
    Replacement::draft_throws(0x1404998e0, "std_1404998e0", fn_1404998e0 as *const (), Throws::Caught), // sub_1404998e0
    Replacement::draft(0x140499950, "std_140499950", tpl_4573::<0x1401e5530, 0x14028e350, 0x14028e000, 0x14307d640, 0x14049d8c0, 0x1401f3ce0, 0x1402c0a50, 0x1401e5510, 0x1401f3e20, 0x1401e54e0, 0x141a08670, 0x1402c0a60, 0x1401f4970, 0x14049d990> as *const ()), // sub_140499950
    Replacement::draft(0x140499c70, "std_140499c70", tpl_4573::<0x1401e5530, 0x14028e350, 0x14028e000, 0x14307d640, 0x14049da10, 0x1401f3ce0, 0x1402c0a50, 0x1401e5510, 0x1401f3e20, 0x1401e54e0, 0x141a08870, 0x1402c0a60, 0x1401f4970, 0x14049dae0> as *const ()), // sub_140499c70
    Replacement::draft(0x140499f90, "std_140499f90", fn_140499f90 as *const ()), // sub_140499f90
    Replacement::draft(0x14049a2d0, "std_14049a2d0", tpl_63787::<0x14049a360, 0x1430782b0, 0x14049dc40, 0x28, 0x8, 0x30, 0x10> as *const ()), // sub_14049a2d0
    Replacement::draft(0x14049a360, "std_14049a360", fn_14049a360 as *const ()), // sub_14049a360
    Replacement::draft(0x14049a450, "std_14049a450", tpl_63787::<0x14049a4e0, 0x1430782b0, 0x14049dc40, 0x28, 0x8, 0x30, 0x10> as *const ()), // sub_14049a450
    Replacement::draft(0x14049a4e0, "std_14049a4e0", fn_14049a4e0 as *const ()), // sub_14049a4e0
    Replacement::draft(0x14049a5e0, "std_14049a5e0", fn_14049a5e0 as *const ()), // sub_14049a5e0
    Replacement::draft(0x14049aa50, "std_14049aa50", fn_14049aa50 as *const ()), // sub_14049aa50
    Replacement::draft_throws(0x14049aac0, "std_14049aac0", tpl_4580::<0x1401e81d0, 0x1401f3a20> as *const (), Throws::Discard), // sub_14049aac0
    Replacement::draft_throws(0x14049ab30, "std_14049ab30", tpl_4580::<0x1401e81d0, 0x14025e3c0> as *const (), Throws::Discard), // sub_14049ab30
    Replacement::draft(0x14049aba0, "std_14049aba0", fn_14049aba0 as *const ()), // sub_14049aba0
    Replacement::draft(0x14049b010, "std_14049b010", fn_14049b010 as *const ()), // sub_14049b010
    Replacement::draft(0x14049b380, "std_14049b380", fn_14049b380 as *const ()), // sub_14049b380
    Replacement::draft_throws(0x14049b6b0, "std_14049b6b0", fn_14049b6b0 as *const (), Throws::Caught), // sub_14049b6b0
    Replacement::draft(0x14049b9f0, "std_14049b9f0", fn_14049b9f0 as *const ()), // sub_14049b9f0
    Replacement::draft(0x14049ba90, "std_14049ba90", tpl_4586::<0x1424b4cb0> as *const ()), // sub_14049ba90
    Replacement::draft(0x14049bac0, "std_14049bac0", fn_14049bac0 as *const ()), // sub_14049bac0
    Replacement::draft_throws(0x14049bc30, "std_14049bc30", fn_14049bc30 as *const (), Throws::Discard), // sub_14049bc30
    Replacement::draft(0x14049bcb0, "std_14049bcb0", tpl_4586::<0x1424b4da0> as *const ()), // sub_14049bcb0
    Replacement::draft_throws(0x14049bce0, "std_14049bce0", fn_14049bce0 as *const (), Throws::Discard), // sub_14049bce0
    Replacement::draft_throws(0x14049bda0, "std_14049bda0", fn_14049bda0 as *const (), Throws::Discard), // sub_14049bda0
    Replacement::draft_throws(0x14049c1c0, "std_14049c1c0", tpl_4592::<0x14049c0a0, 0x1402174f0, 0x14049d490, 0x140200130, 0x1401de2f0, 0x1430782b0, 0x14049e670, 0x14049cb10, 0x14049cda0> as *const (), Throws::Discard), // sub_14049c1c0
    Replacement::draft_throws(0x14049c4c0, "std_14049c4c0", tpl_4592::<0x14049c0a0, 0x1402174f0, 0x14049d490, 0x140200130, 0x1401de2f0, 0x1430782b0, 0x14049e860, 0x14049cb10, 0x14049cda0> as *const (), Throws::Discard), // sub_14049c4c0
    Replacement::draft(0x14049c7c0, "std_14049c7c0", fn_14049c7c0 as *const ()), // sub_14049c7c0
    Replacement::draft(0x14049c880, "unknown_14049c880", fn_14049c880 as *const ()), // sub_14049c880
    Replacement::draft(0x14049c930, "unknown_14049c930", fn_14049c930 as *const ()), // sub_14049c930
    Replacement::draft(0x14049ca20, "unknown_14049ca20", fn_14049ca20 as *const ()), // sub_14049ca20
    Replacement::draft(0x14049cb10, "unknown_14049cb10", fn_14049cb10 as *const ()), // sub_14049cb10
    Replacement::draft(0x14049cc20, "unknown_14049cc20", fn_14049cc20 as *const ()), // sub_14049cc20
    Replacement::draft(0x14049cd20, "unknown_14049cd20", fn_14049cd20 as *const ()), // sub_14049cd20
    Replacement::draft(0x14049cda0, "unknown_14049cda0", fn_14049cda0 as *const ()), // sub_14049cda0
    Replacement::draft(0x14049d090, "unknown_14049d090", fn_14049d090 as *const ()), // sub_14049d090
    Replacement::draft(0x14049d140, "unknown_14049d140", fn_14049d140 as *const ()), // sub_14049d140
    Replacement::draft(0x14049d170, "unknown_14049d170", tpl_4603 as *const ()), // sub_14049d170
    Replacement::draft(0x14049d180, "unknown_14049d180", tpl_4604::<0x8, 0x142442de0> as *const ()), // sub_14049d180
    Replacement::draft(0x14049d1c0, "unknown_14049d1c0", tpl_4604::<0x8, 0x142442de0> as *const ()), // sub_14049d1c0
    Replacement::draft(0x14049d200, "unknown_14049d200", fn_14049d200 as *const ()), // sub_14049d200
    Replacement::draft(0x14049d270, "unknown_14049d270", fn_14049d270 as *const ()), // sub_14049d270
    Replacement::draft(0x14049d350, "unknown_14049d350", fn_14049d350 as *const ()), // sub_14049d350
    Replacement::draft(0x14049d430, "unknown_14049d430", fn_14049d430 as *const ()), // sub_14049d430
    Replacement::draft(0x14049d490, "unknown_14049d490", fn_14049d490 as *const ()), // sub_14049d490
    Replacement::draft(0x14049d4f0, "unknown_14049d4f0", fn_14049d4f0 as *const ()), // sub_14049d4f0
    Replacement::draft(0x14049d510, "unknown_14049d510", fn_14049d510 as *const ()), // sub_14049d510
    Replacement::draft(0x14049d540, "unknown_14049d540", tpl_4612::<0x8> as *const ()), // sub_14049d540
    Replacement::draft(0x14049d5b0, "unknown_14049d5b0", fn_14049d5b0 as *const ()), // sub_14049d5b0
    Replacement::draft(0x14049d5f0, "unknown_14049d5f0", fn_14049d5f0 as *const ()), // sub_14049d5f0
    Replacement::draft(0x14049d680, "unknown_14049d680", fn_14049d680 as *const ()), // sub_14049d680
    Replacement::draft(0x14049d860, "unknown_14049d860", fn_14049d860 as *const ()), // sub_14049d860
    Replacement::draft(0x14049d8c0, "unknown_14049d8c0", tpl_4617::<0x142442e70, 0x1401de2e0, 0x14034e370, 0x14314f9f8, 0x8, 0x1401f3d70, 0x14314f980> as *const ()), // sub_14049d8c0
    Replacement::draft(0x14049d990, "unknown_14049d990", fn_14049d990 as *const ()), // sub_14049d990
    Replacement::draft(0x14049da10, "unknown_14049da10", tpl_4617::<0x142442e70, 0x1401de2e0, 0x14034e370, 0x14314fb30, 0x8, 0x1401f3d70, 0x14314fb00> as *const ()), // sub_14049da10
    Replacement::draft(0x14049dae0, "unknown_14049dae0", fn_14049dae0 as *const ()), // sub_14049dae0
    Replacement::draft(0x14049db70, "unknown_14049db70", tpl_4617::<0x142442e70, 0x1401de2e0, 0x14034e370, 0x14314f998, 0x8, 0x1401f3d70, 0x14314fb18> as *const ()), // sub_14049db70
    Replacement::draft(0x14049dc40, "unknown_14049dc40", fn_14049dc40 as *const ()), // sub_14049dc40
    Replacement::draft(0x14049dc80, "unknown_14049dc80", fn_14049dc80 as *const ()), // sub_14049dc80
    Replacement::draft_throws(0x14049dce0, "unknown_14049dce0", fn_14049dce0 as *const (), Throws::Caught), // sub_14049dce0
    Replacement::draft(0x14049ddd0, "unknown_14049ddd0", tpl_4623::<0x14049f260, 0x1401de2e0, 0x1401de2f0, 0x14314f948, 0x1401e5920> as *const ()), // sub_14049ddd0
    Replacement::draft(0x14049de40, "unknown_14049de40", tpl_4623::<0x14049f260, 0x1401de2e0, 0x1401de2f0, 0x14314fa10, 0x1401e5920> as *const ()), // sub_14049de40
    Replacement::draft(0x14049deb0, "unknown_14049deb0", fn_14049deb0 as *const ()), // sub_14049deb0
    Replacement::draft(0x14049dfa0, "unknown_14049dfa0", fn_14049dfa0 as *const ()), // sub_14049dfa0
    Replacement::draft(0x14049e060, "unknown_14049e060", fn_14049e060 as *const ()), // sub_14049e060
    Replacement::draft_throws(0x14049e150, "unknown_14049e150", fn_14049e150 as *const (), Throws::Discard), // sub_14049e150
    Replacement::draft(0x14049e1e0, "unknown_14049e1e0", fn_14049e1e0 as *const ()), // sub_14049e1e0
    Replacement::draft(0x14049e300, "unknown_14049e300", fn_14049e300 as *const ()), // sub_14049e300
    Replacement::draft(0x14049e340, "unknown_14049e340", fn_14049e340 as *const ()), // sub_14049e340
    Replacement::draft(0x14049e5c0, "unknown_14049e5c0", fn_14049e5c0 as *const ()), // sub_14049e5c0
    Replacement::draft(0x14049e670, "unknown_14049e670", tpl_4632::<0x142442e70, 0x14049f9a0, 0x1401de2e0> as *const ()), // sub_14049e670
    Replacement::draft(0x14049e860, "unknown_14049e860", tpl_4632::<0x142442e70, 0x14049fa90, 0x1401de2e0> as *const ()), // sub_14049e860
    Replacement::draft(0x14049ea50, "unknown_14049ea50", fn_14049ea50 as *const ()), // sub_14049ea50
    Replacement::draft(0x14049ead0, "unknown_14049ead0", fn_14049ead0 as *const ()), // sub_14049ead0
    Replacement::draft(0x14049eb20, "unknown_14049eb20", fn_14049eb20 as *const ()), // sub_14049eb20
    Replacement::draft(0x14049eb60, "unknown_14049eb60", fn_14049eb60 as *const ()), // sub_14049eb60
    Replacement::draft(0x14049ebe0, "unknown_14049ebe0", fn_14049ebe0 as *const ()), // sub_14049ebe0
    Replacement::draft(0x14049ec50, "unknown_14049ec50", fn_14049ec50 as *const ()), // sub_14049ec50
    Replacement::draft(0x14049ecb0, "unknown_14049ecb0", fn_14049ecb0 as *const ()), // sub_14049ecb0
    Replacement::draft(0x14049ed30, "unknown_14049ed30", fn_14049ed30 as *const ()), // sub_14049ed30
    Replacement::draft(0x14049ed60, "unknown_14049ed60", fn_14049ed60 as *const ()), // sub_14049ed60
    Replacement::draft(0x14049edb0, "unknown_14049edb0", fn_14049edb0 as *const ()), // sub_14049edb0
    Replacement::draft(0x14049ee30, "unknown_14049ee30", tpl_4612::<0xc> as *const ()), // sub_14049ee30
    Replacement::draft(0x14049eea0, "unknown_14049eea0", fn_14049eea0 as *const ()), // sub_14049eea0
    Replacement::draft(0x14049ef10, "unknown_14049ef10", fn_14049ef10 as *const ()), // sub_14049ef10
    Replacement::draft(0x14049f0a0, "unknown_14049f0a0", fn_14049f0a0 as *const ()), // sub_14049f0a0
    Replacement::draft(0x14049f0f0, "unknown_14049f0f0", fn_14049f0f0 as *const ()), // sub_14049f0f0
    Replacement::draft_throws(0x14049f150, "unknown_14049f150", fn_14049f150 as *const (), Throws::Discard), // sub_14049f150
    Replacement::draft(0x14049f1f0, "unknown_14049f1f0", fn_14049f1f0 as *const ()), // sub_14049f1f0
    Replacement::draft(0x14049f260, "unknown_14049f260", fn_14049f260 as *const ()), // sub_14049f260
    Replacement::draft_throws(0x14049f270, "unknown_14049f270", fn_14049f270 as *const (), Throws::Discard), // sub_14049f270
    Replacement::draft(0x14049f530, "unknown_14049f530", fn_14049f530 as *const ()), // sub_14049f530
    Replacement::draft(0x14049f5a0, "unknown_14049f5a0", fn_14049f5a0 as *const ()), // sub_14049f5a0
    Replacement::draft(0x14049f6b0, "unknown_14049f6b0", fn_14049f6b0 as *const ()), // sub_14049f6b0
    Replacement::draft(0x14049f6f0, "unknown_14049f6f0", fn_14049f6f0 as *const ()), // sub_14049f6f0
    Replacement::draft(0x14049f720, "unknown_14049f720", fn_14049f720 as *const ()), // sub_14049f720
    Replacement::draft(0x14049f7a0, "unknown_14049f7a0", fn_14049f7a0 as *const ()), // sub_14049f7a0
    Replacement::draft(0x14049f9a0, "unknown_14049f9a0", tpl_4657::<0x1401de2e0, 0x1401de2f0, 0x1404a04d0> as *const ()), // sub_14049f9a0
    Replacement::draft(0x14049fa90, "unknown_14049fa90", tpl_4657::<0x1401de2e0, 0x1401de2f0, 0x1404a0580> as *const ()), // sub_14049fa90
    Replacement::draft(0x14049fb80, "unknown_14049fb80", fn_14049fb80 as *const ()), // sub_14049fb80
    Replacement::draft(0x14049fbc0, "unknown_14049fbc0", fn_14049fbc0 as *const ()), // sub_14049fbc0
    Replacement::draft(0x14049fee0, "unknown_14049fee0", fn_14049fee0 as *const ()), // sub_14049fee0
    Replacement::draft(0x1404a0330, "unknown_1404a0330", fn_1404a0330 as *const ()), // sub_1404a0330
    Replacement::draft(0x1404a0390, "unknown_1404a0390", fn_1404a0390 as *const ()), // sub_1404a0390
    Replacement::draft(0x1404a0440, "unknown_1404a0440", fn_1404a0440 as *const ()), // sub_1404a0440
    Replacement::draft(0x1404a04d0, "unknown_1404a04d0", tpl_4672::<0x1401f3380, 0x1401eeda0, 0x28, 0x29, 0x30, 0x38> as *const ()), // sub_1404a04d0
    Replacement::draft(0x1404a0580, "unknown_1404a0580", tpl_4672::<0x1401fc770, 0x1401eeda0, 0x28, 0x29, 0x30, 0x38> as *const ()), // sub_1404a0580
    Replacement::draft(0x1404a0630, "unknown_1404a0630", fn_1404a0630 as *const ()), // sub_1404a0630
    Replacement::draft(0x1404a0790, "unknown_1404a0790", fn_1404a0790 as *const ()), // sub_1404a0790
    Replacement::draft(0x1404a07c0, "unknown_1404a07c0", fn_1404a07c0 as *const ()), // sub_1404a07c0
    Replacement::draft(0x1404a0800, "unknown_1404a0800", fn_1404a0800 as *const ()), // sub_1404a0800
    Replacement::draft(0x1404a0890, "unknown_1404a0890", fn_1404a0890 as *const ()), // sub_1404a0890
    Replacement::draft(0x1404a08c0, "unknown_1404a08c0", fn_1404a08c0 as *const ()), // sub_1404a08c0
    Replacement::draft(0x1404a0920, "unknown_1404a0920", fn_1404a0920 as *const ()), // sub_1404a0920
    Replacement::draft_throws(0x1404a0a30, "unknown_1404a0a30", fn_1404a0a30 as *const (), Throws::Caught), // sub_1404a0a30
    Replacement::draft_throws(0x1404a0cb0, "unknown_1404a0cb0", fn_1404a0cb0 as *const (), Throws::Caught), // sub_1404a0cb0
    Replacement::draft_throws(0x1404a0fd0, "bigfish_1404a0fd0", tpl_1177::<0x143ca6180, 0x143ca6070, 0x1411e9f50, 0x1404a2510, 0x1404a33a0, 0x1404a2750, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431516f6, 0x142646960, 0x143ca6090, 0x143ca60b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameprojectiledamageaction::update_projectile_location::system>
    Replacement::draft_throws(0x1404a1220, "bigfish_1404a1220", tpl_4682::<0x143ca6178, 0x143ca62f0, 0x1404a1d60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143150486, 0x142646960, 0x143ca6310, 0x143ca6330, 0x143ca6334, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameprojectiledamageaction::update_projectile_spawn::system>
    Replacement::draft_throws(0x1404a1430, "bigfish_1404a1430", fn_1404a1430 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameprojectiledamageaction::update_projectile_spawned::system>
    Replacement::draft(0x1404a1650, "bigfish_1404a1650", tpl_60953::<0x1426467e8, 0x143c954e0, 0x142489ee0, 0x1404a16cc> as *const ()), // sub_1404a1650
    Replacement::draft(0x1404a1800, "bigfish_1404a1800", tpl_1094::<0x143ca6370, 0x1404a2340, 0x14020e8f0> as *const ()), // sub_1404a1800
    Replacement::draft_throws(0x1404a1a30, "bigfish_1404a1a30", fn_1404a1a30 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameprojectiledamageaction::stream_in::system>
    Replacement::draft_throws(0x1404a1d60, "unknown_1404a1d60", fn_1404a1d60 as *const (), Throws::Discard), // sub_1404a1d60
    Replacement::draft_throws(0x1404a1f90, "unknown_1404a1f90", tpl_1187::<0x1411e9f50, 0x1404a3d80, 0x143ca62a8, 0x1420af120, 0x1404a3c90, 0x143ca63ac, 0x140215750, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x1404a3840, 0x1404a2b40> as *const (), Throws::Discard), // sub_1404a1f90
    Replacement::draft(0x1404a21b0, "unknown_1404a21b0", tpl_63880::<0x1426467e8, 0x1404a2f10, 0x1404a21d9> as *const ()), // sub_1404a21b0
    Replacement::draft(0x1404a2340, "unknown_1404a2340", fn_1404a2340 as *const ()), // sub_1404a2340
    Replacement::draft_throws(0x1404a2510, "unknown_1404a2510", fn_1404a2510 as *const (), Throws::Discard), // sub_1404a2510
    Replacement::draft(0x1404a2750, "unknown_1404a2750", fn_1404a2750 as *const ()), // sub_1404a2750
    Replacement::draft(0x1404a2820, "unknown_1404a2820", tpl_4519::<0x8, 0x20, 0x143ca60c4, 0x143ca6184, 0x143c94b44, 0x143c8d08c, 0x580e0> as *const ()), // sub_1404a2820
    Replacement::draft(0x1404a2910, "unknown_1404a2910", fn_1404a2910 as *const ()), // sub_1404a2910
    Replacement::draft_throws(0x1404a2a40, "unknown_1404a2a40", fn_1404a2a40 as *const (), Throws::Discard), // sub_1404a2a40
    Replacement::draft(0x1404a2b40, "unknown_1404a2b40", tpl_60964::<0x143ca63ac, 0x143c8e660, 0x143c8d08c, 0x142489ee0, 0x141a0ff20> as *const ()), // sub_1404a2b40
    Replacement::draft(0x1404a2ca0, "content_1404a2ca0", fn_1404a2ca0 as *const ()), // content::GameProjectileDamageActionComponent::content::GameProjectileDamageActionComponent::vfunc_0
    Replacement::draft(0x1404a30c0, "unknown_1404a30c0", tpl_61197::<0x143c92bcc, 0x142489ee0, 0x143c8ee34, 0x143c933f0, 0x143c8d120, 0x1426467e8, 0x1404a3253> as *const ()), // sub_1404a30c0
    Replacement::draft_throws(0x1404a3260, "unknown_1404a3260", fn_1404a3260 as *const (), Throws::Caught), // sub_1404a3260
    Replacement::draft(0x1404a33a0, "unknown_1404a33a0", fn_1404a33a0 as *const ()), // sub_1404a33a0
    Replacement::draft_throws(0x1404a3650, "unknown_1404a3650", fn_1404a3650 as *const (), Throws::Discard), // sub_1404a3650
    Replacement::draft(0x1404a3840, "unknown_1404a3840", tpl_60979::<0x143ca63ac, 0x143c8d08c, 0x143c8e660, 0x142489ee0, 0x141a0ff20> as *const ()), // sub_1404a3840
    Replacement::draft_throws(0x1404a3b10, "unknown_1404a3b10", fn_1404a3b10 as *const (), Throws::Caught), // sub_1404a3b10
    Replacement::draft(0x1404a4060, "unknown_1404a4060", tpl_4691::<0x143ca62a4, 0x143ca6120, 0x141834a78, 0x143ca6370, 0x143c8d160, 0x143c8d120, 0x143c8ec20, 0x143c8dbcc, 0x143c8edbc, 0x143c8e660, 0x143ca6140, 0x143ca6160, 0x143ca6170, 0x141834a0c> as *const ()), // sub_1404a4060
    Replacement::draft(0x1404a4800, "unknown_1404a4800", fn_1404a4800 as *const ()), // sub_1404a4800
    Replacement::draft(0x1404a4870, "unknown_1404a4870", fn_1404a4870 as *const ()), // sub_1404a4870
    Replacement::draft(0x1404a4980, "unknown_1404a4980", fn_1404a4980 as *const ()), // sub_1404a4980
    Replacement::draft(0x1404a4ae0, "unknown_1404a4ae0", fn_1404a4ae0 as *const ()), // sub_1404a4ae0
    Replacement::draft(0x1404a4c30, "unknown_1404a4c30", fn_1404a4c30 as *const ()), // sub_1404a4c30
    Replacement::draft(0x1404a4d50, "unknown_1404a4d50", fn_1404a4d50 as *const ()), // sub_1404a4d50
    Replacement::draft(0x1404a4e70, "unknown_1404a4e70", fn_1404a4e70 as *const ()), // sub_1404a4e70
    Replacement::draft(0x1404a4f00, "unknown_1404a4f00", fn_1404a4f00 as *const ()), // sub_1404a4f00
    Replacement::draft(0x1404a52b0, "unknown_1404a52b0", fn_1404a52b0 as *const ()), // sub_1404a52b0
    Replacement::draft(0x1404a55c0, "unknown_1404a55c0", fn_1404a55c0 as *const ()), // sub_1404a55c0
    Replacement::draft_throws(0x1404a56f0, "unknown_1404a56f0", fn_1404a56f0 as *const (), Throws::Caught), // sub_1404a56f0
    Replacement::draft(0x1404a5950, "unknown_1404a5950", fn_1404a5950 as *const ()), // sub_1404a5950
    Replacement::draft_throws(0x1404a5a80, "unknown_1404a5a80", fn_1404a5a80 as *const (), Throws::Caught), // sub_1404a5a80
    Replacement::draft_throws(0x1404a5ef0, "bigfish_1404a5ef0", tpl_4703::<0x143ca6410, 0x143ca63f0, 0x1404a61c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143152006, 0x142646960, 0x143ca6400, 0x143ca6408, 0x143ca640c, 0x143ca640e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::light_manager::system>
    Replacement::draft(0x1404a6100, "unknown_1404a6100", fn_1404a6100 as *const ()), // sub_1404a6100
    Replacement::draft_throws(0x1404a61c0, "unknown_1404a61c0", fn_1404a61c0 as *const (), Throws::Discard), // sub_1404a61c0
    Replacement::draft_throws(0x1404a6360, "unknown_1404a6360", fn_1404a6360 as *const (), Throws::Discard), // sub_1404a6360
    Replacement::draft(0x1404a6540, "unknown_1404a6540", fn_1404a6540 as *const ()), // sub_1404a6540
    Replacement::draft(0x1404a6570, "unknown_1404a6570", fn_1404a6570 as *const ()), // sub_1404a6570
    Replacement::draft(0x1404a6650, "unknown_1404a6650", fn_1404a6650 as *const ()), // sub_1404a6650
    Replacement::draft(0x1404a6730, "unknown_1404a6730", tpl_4710::<0xfffffffffffffffe, 0x30, 0x1404a4c30, 0x1404a4d50, 0x142442de0> as *const ()), // sub_1404a6730
    Replacement::draft(0x1404a67a0, "unknown_1404a67a0", tpl_381::<0x1404a6a60, 0x1404a4f00> as *const ()), // sub_1404a67a0
    Replacement::draft(0x1404a6900, "unknown_1404a6900", tpl_381::<0x1404a6d20, 0x1404a52b0> as *const ()), // sub_1404a6900
    Replacement::draft(0x1404a6a60, "unknown_1404a6a60", fn_1404a6a60 as *const ()), // sub_1404a6a60
    Replacement::draft(0x1404a6d20, "unknown_1404a6d20", fn_1404a6d20 as *const ()), // sub_1404a6d20
    Replacement::draft(0x1404a7040, "unknown_1404a7040", fn_1404a7040 as *const ()), // sub_1404a7040
    Replacement::draft(0x1404a70d0, "unknown_1404a70d0", fn_1404a70d0 as *const ()), // sub_1404a70d0
    Replacement::draft(0x1404a7180, "unknown_1404a7180", fn_1404a7180 as *const ()), // sub_1404a7180
    Replacement::draft(0x1404a7200, "unknown_1404a7200", fn_1404a7200 as *const ()), // sub_1404a7200
    Replacement::draft(0x1404a72a0, "unknown_1404a72a0", fn_1404a72a0 as *const ()), // sub_1404a72a0
    Replacement::draft(0x1404a7410, "unknown_1404a7410", fn_1404a7410 as *const ()), // sub_1404a7410
    Replacement::draft(0x1404a74b0, "unknown_1404a74b0", fn_1404a74b0 as *const ()), // sub_1404a74b0
    Replacement::draft(0x1404a7500, "unknown_1404a7500", fn_1404a7500 as *const ()), // sub_1404a7500
    Replacement::draft(0x1404a7610, "unknown_1404a7610", fn_1404a7610 as *const ()), // sub_1404a7610
    Replacement::draft_throws(0x1404a76b0, "unknown_1404a76b0", fn_1404a76b0 as *const (), Throws::Discard), // sub_1404a76b0
    Replacement::draft(0x1404a7800, "unknown_1404a7800", tpl_3605::<0x19, 0x10, 0x1404a7800, 0x28, 0x140217f00, 0x1401e66f0> as *const ()), // sub_1404a7800
    Replacement::draft(0x1404a7880, "unknown_1404a7880", fn_1404a7880 as *const ()), // sub_1404a7880
    Replacement::draft(0x1404a78e0, "unknown_1404a78e0", fn_1404a78e0 as *const ()), // sub_1404a78e0
    Replacement::draft(0x1404a7a00, "unknown_1404a7a00", tpl_381::<0x1404a7b60, 0x1404a7ed0> as *const ()), // sub_1404a7a00
    Replacement::draft(0x1404a7b60, "unknown_1404a7b60", fn_1404a7b60 as *const ()), // sub_1404a7b60
    Replacement::draft(0x1404a7ed0, "unknown_1404a7ed0", fn_1404a7ed0 as *const ()), // sub_1404a7ed0
    Replacement::draft(0x1404a8200, "unknown_1404a8200", tpl_63667::<0x8, 0xc0, 0x30, 0x178, 0x130> as *const ()), // sub_1404a8200
    Replacement::draft(0x1404a82a0, "coregame_1404a82a0", tpl_579::<0x143152f28, 0x8> as *const ()), // coregame::CoreProgramFlow::ActionRestartCheckpointParameters::coregame::CoreProgramFlow::ActionRestartCheckpointParamete
    Replacement::draft(0x1404a83c0, "unknown_1404a83c0", tpl_4008::<0x30, 0x143c914bc, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1404a8443> as *const ()), // sub_1404a83c0
    Replacement::draft(0x1404a8450, "unknown_1404a8450", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143ca64c8, 0x1426467e8, 0x143c8e710, 0x142489ee0, 0x1404a8547> as *const ()), // sub_1404a8450
    Replacement::draft(0x1404a8660, "unknown_1404a8660", tpl_1363::<0x30, 0x143c914bc, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1404a86ad> as *const ()), // sub_1404a8660
    Replacement::draft(0x1404a86b0, "unknown_1404a86b0", fn_1404a86b0 as *const ()), // sub_1404a86b0
    Replacement::draft_throws(0x1404a88f0, "unknown_1404a88f0", tpl_4037::<0x143ca6478, 0x143ca64d0, 0x143ca6458, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dc20, 0x143c8dbc8, 0x143ca6468, 0x143ca6470, 0x141834a0c> as *const (), Throws::Discard), // sub_1404a88f0
    Replacement::draft_throws(0x1404a8a50, "unknown_1404a8a50", tpl_847::<0x143ca64c8, 0x143ca64cc, 0x143ca64b8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e710, 0x141834a0c> as *const (), Throws::Discard), // sub_1404a8a50
    Replacement::draft(0x1404a8b80, "unknown_1404a8b80", fn_1404a8b80 as *const ()), // sub_1404a8b80
    Replacement::draft(0x1404a8d60, "unknown_1404a8d60", fn_1404a8d60 as *const ()), // sub_1404a8d60
    Replacement::draft(0x1404a8eb0, "unknown_1404a8eb0", fn_1404a8eb0 as *const ()), // sub_1404a8eb0
    Replacement::draft(0x1404a8f40, "unknown_1404a8f40", fn_1404a8f40 as *const ()), // sub_1404a8f40
    Replacement::draft(0x1404a9130, "unknown_1404a9130", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143c8d120, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1404a9130
    Replacement::draft(0x1404a9350, "unknown_1404a9350", fn_1404a9350 as *const ()), // sub_1404a9350
    Replacement::draft(0x1404a9390, "unknown_1404a9390", tpl_63073::<0x143c8d120, 0x142489ee0, 0x143c933f0, 0x143c9d618> as *const ()), // sub_1404a9390
    Replacement::draft(0x1404a9500, "unknown_1404a9500", fn_1404a9500 as *const ()), // sub_1404a9500
    Replacement::draft(0x1404a9580, "unknown_1404a9580", tpl_3648::<0x143c8d160, 0x142489ee0, 0x143c9d1d8, 0x143c8f6d8, 0x143c8d120> as *const ()), // sub_1404a9580
    Replacement::draft(0x1404a96b0, "unknown_1404a96b0", tpl_61019::<0x143c8f7a0, 0x142489ee0, 0x143c900a8, 0x143c8f6dc, 0x1426467e8, 0x1404a9813> as *const ()), // sub_1404a96b0
    Replacement::draft(0x1404a9820, "unknown_1404a9820", fn_1404a9820 as *const ()), // sub_1404a9820
    Replacement::draft(0x1404a9a60, "unknown_1404a9a60", tpl_61197::<0x143c8f7a0, 0x142489ee0, 0x143c9c6e4, 0x143c900a8, 0x143c9c7b0, 0x1426467e8, 0x1404a9bf3> as *const ()), // sub_1404a9a60
    Replacement::draft(0x1404a9c00, "unknown_1404a9c00", tpl_3056::<0x143c900a8, 0x40, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1404a9c00
    Replacement::draft(0x1404a9c60, "unknown_1404a9c60", tpl_63932::<0x1404a9d90, 0x1426467e8, 0x1404a9d81> as *const ()), // sub_1404a9c60
    Replacement::draft_throws(0x1404a9f90, "bigfish_1404a9f90", tpl_1503::<0x143ca6a80, 0x143ca6658, 0x1411e9f50, 0x1404aea70, 0x1404b06f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143155726, 0x142646960, 0x143ca6678, 0x143ca667c, 0x143ca667e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::updateCursorInput>
    Replacement::draft_throws(0x1404aa1e0, "bigfish_1404aa1e0", tpl_1698::<0x143ca6b84, 0x143ca6890, 0x1411e9f50, 0x1404aec50, 0x1404b0850, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143155366, 0x142646960, 0x143ca68b0, 0x143ca68b2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::updateRaycast>
    Replacement::draft(0x1404aa420, "bigfish_1404aa420", fn_1404aa420 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::sendCursorEventsToMainUIPage>
    Replacement::draft_throws(0x1404aa700, "bigfish_1404aa700", tpl_4193::<0x143ca6d60, 0x143ca6688, 0x1411e9f50, 0x1404aeda0, 0x1404b0920, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143156b86, 0x142646960, 0x143ca66a8, 0x143ca66b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::sendCursorEventsToTextureViews>
    Replacement::draft_throws(0x1404aa950, "bigfish_1404aa950", tpl_1396::<0x143ca6fc0, 0x143ca6e50, 0x1411e9f50, 0x1404aef50, 0x1404b09b0, 0x1404af140, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431561d6, 0x142646960, 0x143ca6e70, 0x143ca6e78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::updateDraggingEntity>
    Replacement::draft_throws(0x1404aaba0, "bigfish_1404aaba0", fn_1404aaba0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::resolveDragOffsetAnimationRequests>
    Replacement::draft_throws(0x1404aae90, "bigfish_1404aae90", tpl_3407::<0x143ca6680, 0x143ca66c0, 0x1411e9f50, 0x1404b25e0, 0x143ca679c, 0x1420af120, 0x1420a28e0, 0x1404b0ef0, 0x1404af630, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143155d86, 0x142646960, 0x143ca66e0, 0x143ca66f0, 0x143ca66f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::checkStretchyDragReturnFinished>
    Replacement::draft_throws(0x1404ab180, "bigfish_1404ab180", tpl_1583::<0x143ca6970, 0x143ca70b8, 0x1411e9f50, 0x1404af750, 0x1404b1110, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143157726, 0x142646960, 0x143ca70d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::handleCursorAction>
    Replacement::draft_throws(0x1404ab3c0, "bigfish_1404ab3c0", tpl_3367::<0x143ca6b18, 0x143ca6e20, 0x1404ada20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143157126, 0x142646960, 0x143ca6e40, 0x143ca6e48, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::updateFlickTargeting>
    Replacement::draft_throws(0x1404ab5d0, "bigfish_1404ab5d0", tpl_1503::<0x143ca6654, 0x143ca7020, 0x1411e9f50, 0x1404af950, 0x1404b1350, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143156e66, 0x142646960, 0x143ca7040, 0x143ca7044, 0x143ca7046, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::handleDragPanning>
    Replacement::draft_throws(0x1404ab820, "bigfish_1404ab820", tpl_1480::<0x14273f640, 0x1410d6ca0, 0x1404adc10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404ab820
    Replacement::draft_throws(0x1404abac0, "bigfish_1404abac0", tpl_1480::<0x14273f640, 0x1410d6ca0, 0x1404ade60, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404abac0
    Replacement::draft_throws(0x1404abd60, "bigfish_1404abd60", tpl_1480::<0x14273f640, 0x1410d6ca0, 0x1404ae0a0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404abd60
    Replacement::draft(0x1404ac000, "bigfish_1404ac000", tpl_1526::<0x14273f640, 0x1410d6ca0, 0x1404ae390, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1404ac000
    Replacement::draft(0x1404ac2a0, "bigfish_1404ac2a0", fn_1404ac2a0 as *const ()), // sub_1404ac2a0
    Replacement::draft(0x1404ac410, "bigfish_1404ac410", tpl_60953::<0x1426467e8, 0x143c9e938, 0x142489ee0, 0x1404ac48c> as *const ()), // sub_1404ac410
    Replacement::draft(0x1404ac5c0, "bigfish_1404ac5c0", tpl_4324::<0x143ca6798, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404ac5c0
    Replacement::draft(0x1404ac900, "bigfish_1404ac900", tpl_1097::<0x143c9e938, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404ac900
    Replacement::draft(0x1404acc30, "bigfish_1404acc30", tpl_4732::<0x143ca6a84, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404acc30
    Replacement::draft(0x1404acf80, "bigfish_1404acf80", tpl_1098::<0x143ca6c78, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404acf80
    Replacement::draft(0x1404ad2c0, "bigfish_1404ad2c0", tpl_1097::<0x143ca6cf0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404ad2c0
    Replacement::draft(0x1404ad5f0, "bigfish_1404ad5f0", tpl_1094::<0x143c9ed30, 0x1404ae760, 0x14020e8f0> as *const ()), // sub_1404ad5f0
    Replacement::draft(0x1404ad7f0, "bigfish_1404ad7f0", tpl_1094::<0x143c9d1d8, 0x1404ae8f0, 0x14020e8f0> as *const ()), // sub_1404ad7f0
    Replacement::draft_throws(0x1404ada20, "bigfish_1404ada20", fn_1404ada20 as *const (), Throws::Discard), // sub_1404ada20
    Replacement::draft_throws(0x1404adc10, "bigfish_1404adc10", tpl_3654::<0x143ca6f24, 0x143ca6a50, 0x1411e9f50, 0x1404b13d0, 0x1404b1c80, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143158836, 0x142646960, 0x143ca6a70, 0x143ca6a78, 0x143ca6a7a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::updateCursorMagnetism>
    Replacement::draft_throws(0x1404ade60, "bigfish_1404ade60", tpl_4415::<0x143ca6d6c, 0x143ca69f8, 0x1411e9f50, 0x1404b16f0, 0x1404b1e60, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143158116, 0x142646960, 0x143ca6a18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::syncUIFacts>
    Replacement::draft_throws(0x1404ae0a0, "bigfish_1404ae0a0", fn_1404ae0a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::setMagnetismCursorPos>
    Replacement::draft(0x1404ae390, "bigfish_1404ae390", fn_1404ae390 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::gamecursor::updateEventPropagationToggle>
    Replacement::draft(0x1404ae5a0, "unknown_1404ae5a0", tpl_60530::<0x1426467e8, 0x143c8d160, 0x142489ee0, 0x143c9e938, 0x1404ae5c9> as *const ()), // sub_1404ae5a0
    Replacement::draft(0x1404ae760, "unknown_1404ae760", fn_1404ae760 as *const ()), // sub_1404ae760
    Replacement::draft(0x1404ae8f0, "unknown_1404ae8f0", fn_1404ae8f0 as *const ()), // sub_1404ae8f0
    Replacement::draft_throws(0x1404aea70, "unknown_1404aea70", fn_1404aea70 as *const (), Throws::Discard), // sub_1404aea70
    Replacement::draft_throws(0x1404aec50, "unknown_1404aec50", fn_1404aec50 as *const (), Throws::Discard), // sub_1404aec50
    Replacement::draft_throws(0x1404aeda0, "unknown_1404aeda0", fn_1404aeda0 as *const (), Throws::Discard), // sub_1404aeda0
    Replacement::draft_throws(0x1404aef50, "unknown_1404aef50", fn_1404aef50 as *const (), Throws::Discard), // sub_1404aef50
    Replacement::draft(0x1404af140, "unknown_1404af140", fn_1404af140 as *const ()), // sub_1404af140
    Replacement::draft(0x1404af230, "unknown_1404af230", fn_1404af230 as *const ()), // sub_1404af230
    Replacement::draft(0x1404af370, "unknown_1404af370", fn_1404af370 as *const ()), // sub_1404af370
    Replacement::draft(0x1404af510, "unknown_1404af510", tpl_1113::<0x88, 0x10088, 0x142407c80, 0x1404aff30, 0x580f0, 0x141a43ee0> as *const ()), // sub_1404af510
    Replacement::draft(0x1404af630, "unknown_1404af630", tpl_1113::<0x88, 0x10088, 0x142407c80, 0x1404afe00, 0x580f0, 0x141a43db0> as *const ()), // sub_1404af630
    Replacement::draft_throws(0x1404af750, "unknown_1404af750", fn_1404af750 as *const (), Throws::Discard), // sub_1404af750
    Replacement::draft_throws(0x1404af950, "unknown_1404af950", tpl_4746::<0x1404b2040, 0x143ca7080, 0x1420af120, 0x140215750, 0x143c8d0d4, 0x140215610, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404af950
    Replacement::draft(0x1404afac0, "unknown_1404afac0", tpl_1765::<0x1411e9f50, 0x143c8d0d4, 0x140215610, 0x143c8ce6c, 0x143c8ce88, 0x1420a28e0, 0x1404b2000, 0x1401de410> as *const ()), // sub_1404afac0
    Replacement::draft_throws(0x1404afc40, "unknown_1404afc40", tpl_1330::<0x8, 0x14041d610, 0x143c9f2f8, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1404afc40
    Replacement::draft_throws(0x1404afd20, "unknown_1404afd20", tpl_1330::<0x8, 0x1404b2280, 0x143ca67a4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1404afd20
    Replacement::draft_throws(0x1404b0400, "unknown_1404b0400", tpl_1330::<0x8, 0x1404b2d20, 0x143ca7084, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1404b0400
    Replacement::draft_throws(0x1404b04e0, "unknown_1404b04e0", tpl_1330::<0x8, 0x1404b2f10, 0x143ca6d34, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1404b04e0
    Replacement::draft_throws(0x1404b1c80, "unknown_1404b1c80", fn_1404b1c80 as *const (), Throws::Discard), // sub_1404b1c80
    Replacement::draft_throws(0x1404b1e60, "unknown_1404b1e60", fn_1404b1e60 as *const (), Throws::Discard), // sub_1404b1e60
    Replacement::draft_throws(0x1404b1f80, "unknown_1404b1f80", tpl_3565::<0x143ca7084, 0x580e0, 0x20, 0x8, 0x143c8ce88, 0x143c8d0d4, 0x141a3fb50> as *const (), Throws::Discard), // sub_1404b1f80
    Replacement::draft(0x1404b23c0, "unknown_1404b23c0", fn_1404b23c0 as *const ()), // sub_1404b23c0
    Replacement::draft(0x1404b2980, "unknown_1404b2980", fn_1404b2980 as *const ()), // sub_1404b2980
    Replacement::draft(0x1404b3090, "unknown_1404b3090", fn_1404b3090 as *const ()), // sub_1404b3090
    Replacement::draft(0x1404b3190, "unknown_1404b3190", fn_1404b3190 as *const ()), // sub_1404b3190
    Replacement::draft(0x1404b3830, "unknown_1404b3830", tpl_4758::<0x14240a730> as *const ()), // sub_1404b3830
    Replacement::draft(0x1404b3cd0, "unknown_1404b3cd0", fn_1404b3cd0 as *const ()), // sub_1404b3cd0
    Replacement::draft(0x1404b4780, "unknown_1404b4780", tpl_4758::<0x14240aa00> as *const ()), // sub_1404b4780
    Replacement::draft(0x1404b4de0, "content_1404b4de0", fn_1404b4de0 as *const ()), // content::DirectionalLightComponent::content::DirectionalLightComponent::vfunc_4
    Replacement::draft(0x1404b4df0, "content_1404b4df0", tpl_1044::<0x143426780> as *const ()), // content::DirectionalLightComponent::content::DirectionalLightComponent::vfunc_8
    Replacement::draft(0x1404b4e00, "content_1404b4e00", tpl_4761::<0x143159c98, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::CharacterControllerComponent::content::CharacterControllerComponent::vfunc_18
    Replacement::draft_throws(0x1404b4f00, "unknown_1404b4f00", fn_1404b4f00 as *const (), Throws::Discard), // sub_1404b4f00
    Replacement::draft_throws(0x1404b5650, "game_character_controller_1404b5650", tpl_3281::<0x143ca715c, 0x143ca7280, 0x1411e9f50, 0x1404b8520, 0x143ca71c0, 0x1420af120, 0x1420a28e0, 0x1404b7b20, 0x1404b7030, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14315a926, 0x142646960, 0x143ca72a0, 0x143ca72a8, 0x143ca72ac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<game_character_controller::updatePushability>
    Replacement::draft_throws(0x1404b62d0, "coregame_1404b62d0", fn_1404b62d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::moveController<struct `anonymous-namespace'::GameCharac
    Replacement::draft_throws(0x1404b6540, "coregame_1404b6540", fn_1404b6540 as *const (), Throws::Discard), // sub_1404b6540
    Replacement::draft_throws(0x1404b6b90, "coregame_1404b6b90", fn_1404b6b90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::charactercontroller::applyToPhysics<struct `anonymous-namespace'::GameCharac
    Replacement::draft(0x1404b6f70, "unknown_1404b6f70", tpl_60443::<0x143c914d0, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x1404b702a> as *const ()), // sub_1404b6f70
    Replacement::draft(0x1404b7030, "unknown_1404b7030", tpl_62730::<0x88, 0x10088, 0x1404b7150, 0x141a4ace0> as *const ()), // sub_1404b7030
    Replacement::draft(0x1404b7150, "unknown_1404b7150", tpl_3433::<0x143c91f48, 0x142489ee0, 0x143c92bc8, 0x143ca72c8> as *const ()), // sub_1404b7150
    Replacement::draft_throws(0x1404b7280, "unknown_1404b7280", fn_1404b7280 as *const (), Throws::Discard), // sub_1404b7280
    Replacement::draft_throws(0x1404b7880, "unknown_1404b7880", fn_1404b7880 as *const (), Throws::Discard), // sub_1404b7880
    Replacement::draft(0x1404b7b20, "unknown_1404b7b20", fn_1404b7b20 as *const ()), // sub_1404b7b20
    Replacement::draft(0x1404b7d30, "unknown_1404b7d30", tpl_3289::<0x18, 0x10, 0x8, 0x1404b7e30, 0x140390990, 0x20, 0x40, 0x60, 0x80, 0x90> as *const ()), // sub_1404b7d30
    Replacement::draft(0x1404b7e30, "unknown_1404b7e30", tpl_3290::<0x143c97a60, 0x142489ee0, 0x143ca72cc, 0x143c97a5c, 0x143ca72c8, 0x143c91f1c, 0x143ca7420, 0x143c8d120, 0x143c8e68c, 0x143c97a2c, 0x143c8fa04, 0x143c8d160, 0x143c917cc, 0x143c9f438, 0x143c8d074, 0x143c8f6e4> as *const ()), // sub_1404b7e30
    Replacement::draft_throws(0x1404b8170, "unknown_1404b8170", fn_1404b8170 as *const (), Throws::Discard), // sub_1404b8170
    Replacement::draft_throws(0x1404b8470, "unknown_1404b8470", tpl_4769::<0x8, 0x20, 0x143ca72b0, 0x143ca71a0, 0x1404b6540> as *const (), Throws::Discard), // sub_1404b8470
    Replacement::draft(0x1404b9440, "unknown_1404b9440", fn_1404b9440 as *const ()), // sub_1404b9440
    Replacement::draft(0x1404b9610, "content_1404b9610", tpl_4773::<0x14315db38, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::BehaviorTreeComponent::content::BehaviorTreeComponent::vfunc_18
    Replacement::draft(0x1404b9720, "content_1404b9720", tpl_4774::<0x14315db68, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::BonsaiRouteComponent::content::BonsaiRouteComponent::vfunc_18
    Replacement::draft(0x1404b9840, "content_1404b9840", tpl_4761::<0x14315daf0, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::WorkspotComponent::content::WorkspotComponent::vfunc_18
    Replacement::draft(0x1404b9990, "unknown_1404b9990", fn_1404b9990 as *const ()), // sub_1404b9990
    Replacement::draft_throws(0x1404b9a70, "unknown_1404b9a70", fn_1404b9a70 as *const (), Throws::Discard), // sub_1404b9a70
    Replacement::draft(0x1404b9d30, "unknown_1404b9d30", tpl_63972::<0x1401e5380, 0x8, 0x1401e66f0> as *const ()), // sub_1404b9d30
    Replacement::draft_throws(0x1404b9d90, "unknown_1404b9d90", fn_1404b9d90 as *const (), Throws::Discard), // sub_1404b9d90
    Replacement::draft_throws(0x1404b9f20, "bonsai_1404b9f20", fn_1404b9f20 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::behaviortree::behaviorTreeUpdate<struct bigfish::GameBehaviorTreeContext>>
    Replacement::draft(0x1404ba180, "unknown_1404ba180", fn_1404ba180 as *const ()), // sub_1404ba180
    Replacement::draft(0x1404ba1e0, "unknown_1404ba1e0", tpl_4781::<0x8, 0x10, 0x1401fc200, 0x18, 0x30, 0x38, 0x1401e5730> as *const ()), // sub_1404ba1e0
    Replacement::draft(0x1404ba250, "unknown_1404ba250", fn_1404ba250 as *const ()), // sub_1404ba250
    Replacement::draft_throws(0x1404ba350, "unknown_1404ba350", fn_1404ba350 as *const (), Throws::Discard), // sub_1404ba350
    Replacement::draft_throws(0x1404ba760, "unknown_1404ba760", fn_1404ba760 as *const (), Throws::Discard), // sub_1404ba760
    Replacement::draft_throws(0x1404ba840, "unknown_1404ba840", fn_1404ba840 as *const (), Throws::Discard), // sub_1404ba840
    Replacement::draft(0x1404baa40, "unknown_1404baa40", fn_1404baa40 as *const ()), // sub_1404baa40
    Replacement::draft(0x1404baba0, "unknown_1404baba0", fn_1404baba0 as *const ()), // sub_1404baba0
    Replacement::draft(0x1404bb2d0, "unknown_1404bb2d0", fn_1404bb2d0 as *const ()), // sub_1404bb2d0
    Replacement::draft_throws(0x1404bb6a0, "unknown_1404bb6a0", fn_1404bb6a0 as *const (), Throws::Discard), // sub_1404bb6a0
    Replacement::draft(0x1404bb9d0, "unknown_1404bb9d0", fn_1404bb9d0 as *const ()), // sub_1404bb9d0
    Replacement::draft(0x1404bbb30, "unknown_1404bbb30", fn_1404bbb30 as *const ()), // sub_1404bbb30
    Replacement::draft_throws(0x1404bbc70, "unknown_1404bbc70", fn_1404bbc70 as *const (), Throws::Discard), // sub_1404bbc70
    Replacement::draft(0x1404bbe10, "unknown_1404bbe10", fn_1404bbe10 as *const ()), // sub_1404bbe10
    Replacement::draft(0x1404bc2c0, "unknown_1404bc2c0", fn_1404bc2c0 as *const ()), // sub_1404bc2c0
    Replacement::draft(0x1404bce70, "unknown_1404bce70", fn_1404bce70 as *const ()), // sub_1404bce70
    Replacement::draft_throws(0x1404bce80, "std_1404bce80", fn_1404bce80 as *const (), Throws::Discard), // sub_1404bce80
    Replacement::draft(0x1404bcff0, "unknown_1404bcff0", fn_1404bcff0 as *const ()), // sub_1404bcff0
    Replacement::draft_throws(0x1404bd1d0, "unknown_1404bd1d0", tpl_4541::<0x143ca7828, 0x1404bbb30, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1404bd1d0
    Replacement::draft(0x1404bd260, "unknown_1404bd260", fn_1404bd260 as *const ()), // sub_1404bd260
    Replacement::draft(0x1404bd490, "unknown_1404bd490", fn_1404bd490 as *const ()), // sub_1404bd490
    Replacement::draft_throws(0x1404bd4e0, "unknown_1404bd4e0", fn_1404bd4e0 as *const (), Throws::Discard), // sub_1404bd4e0
    Replacement::draft_throws(0x1404bd870, "std_1404bd870", fn_1404bd870 as *const (), Throws::Discard), // sub_1404bd870
    Replacement::draft(0x1404bdab0, "unknown_1404bdab0", fn_1404bdab0 as *const ()), // sub_1404bdab0
    Replacement::draft(0x1404bdb10, "unknown_1404bdb10", tpl_4797::<0x8, 0x10, 0x1404bde90, 0x18, 0x20, 0x28, 0x1401e5730, 0x30, 0x38> as *const ()), // sub_1404bdb10
    Replacement::draft(0x1404bdc00, "unknown_1404bdc00", fn_1404bdc00 as *const ()), // sub_1404bdc00
    Replacement::draft(0x1404bdd40, "unknown_1404bdd40", tpl_4797::<0x8, 0x10, 0x14025f9d0, 0x18, 0x20, 0x28, 0x1401e5730, 0x30, 0x38> as *const ()), // sub_1404bdd40
    Replacement::draft(0x1404bde30, "unknown_1404bde30", fn_1404bde30 as *const ()), // sub_1404bde30
    Replacement::draft(0x1404bde90, "unknown_1404bde90", tpl_525::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18> as *const ()), // sub_1404bde90
    Replacement::draft(0x1404bdf00, "unknown_1404bdf00", fn_1404bdf00 as *const ()), // sub_1404bdf00
    Replacement::draft(0x1404bdf90, "content_1404bdf90", tpl_1967::<0x1401de830, 0x143084e28, 0x141834d30> as *const ()), // content::AreaShapeComponent::content::AreaShapeComponent::vfunc_0
    Replacement::draft(0x1404bdfe0, "content_1404bdfe0", fn_1404bdfe0 as *const ()), // content::WorkspotComponent::content::WorkspotComponent::vfunc_0
    Replacement::draft(0x1404be0a0, "unknown_1404be0a0", tpl_60953::<0x1426467e8, 0x143c9d1c4, 0x142489ee0, 0x1404be11c> as *const ()), // sub_1404be0a0
    Replacement::draft_throws(0x1404be250, "unknown_1404be250", tpl_1480::<0x142741ab0, 0x1410d6ca0, 0x1404bfdf0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404be250
    Replacement::draft_throws(0x1404be4f0, "bigfish_1404be4f0", tpl_3062::<0x143ca7eec, 0x143ca7aa0, 0x1411e9f50, 0x1404c0c30, 0x1404c19b0, 0x1404c0e20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431602c6, 0x142646960, 0x143ca7ac0, 0x143ca7ac8, 0x143ca7acc, 0x143ca7ace, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamearmoury::setup_npc_weapon::system>
    Replacement::draft_throws(0x1404be750, "bigfish_1404be750", fn_1404be750 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamearmoury::update_detaching::system>
    Replacement::draft_throws(0x1404beac0, "bigfish_1404beac0", tpl_1480::<0x142741ab0, 0x1410d6ca0, 0x1404c0000, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404beac0
    Replacement::draft(0x1404bed60, "bigfish_1404bed60", tpl_60953::<0x1426467e8, 0x143c8e840, 0x142489ee0, 0x1404beddc> as *const ()), // sub_1404bed60
    Replacement::draft(0x1404bef10, "bigfish_1404bef10", tpl_1094::<0x143c98fc0, 0x140467f50, 0x14020e8f0> as *const ()), // sub_1404bef10
    Replacement::draft_throws(0x1404bf110, "bigfish_1404bf110", fn_1404bf110 as *const (), Throws::Caught), // sub_1404bf110
    Replacement::draft(0x1404bf310, "bigfish_1404bf310", tpl_1436::<0x58, 0x60, 0x158, 0x143c98fc0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1404bf310
    Replacement::draft(0x1404bf3f0, "bigfish_1404bf3f0", tpl_1094::<0x143c9d1c4, 0x1404c06d0, 0x14020e8f0> as *const ()), // sub_1404bf3f0
    Replacement::draft(0x1404bf5f0, "bigfish_1404bf5f0", tpl_1094::<0x143ca7dbc, 0x1404c0880, 0x14020e8f0> as *const ()), // sub_1404bf5f0
    Replacement::draft_throws(0x1404bf820, "bigfish_1404bf820", tpl_3591::<0x143ca7cc4, 0x143ca7c40, 0x1411e9f50, 0x1404c3e20, 0x143ca7bfc, 0x1420af120, 0x1404c2240, 0x1420a28e0, 0x1404c27a0, 0x1404c1820, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431607d6, 0x142646960, 0x143ca7c60, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamearmoury::stream_in::system>
    Replacement::draft_throws(0x1404bfb10, "bigfish_1404bfb10", tpl_3683::<0x143ca7bb4, 0x143ca7b88, 0x1411e9f50, 0x1404c3d30, 0x143ca7a98, 0x1420af120, 0x1420a28e0, 0x1404c2960, 0x1404c1920, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143161466, 0x142646960, 0x143ca7ba8, 0x143ca7bb0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamearmoury::stream_out::system>
    Replacement::draft_throws(0x1404bfdf0, "bigfish_1404bfdf0", tpl_4804::<0x143ca79a4, 0x143ca7f50, 0x1404c0a40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143161236, 0x142646960, 0x143ca7f70, 0x143ca7f78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamearmoury::query_removed::system>
    Replacement::draft_throws(0x1404c0000, "bigfish_1404c0000", tpl_3690::<0x143ca7cc8, 0x143ca7d08, 0x1411e9f50, 0x1404c3a40, 0x143ca7cc0, 0x1420af120, 0x1404c3900, 0x143ca7d3c, 0x140215750, 0x1420a28e0, 0x1404c2ba0, 0x1404c1d40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143161686, 0x142646960, 0x143ca7d28, 0x143ca7d38, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gamearmoury::update_hide_reason::system>
    Replacement::draft(0x1404c0350, "unknown_1404c0350", tpl_60530::<0x1426467e8, 0x143c8d160, 0x142489ee0, 0x143c98fc0, 0x1404c0379> as *const ()), // sub_1404c0350
    Replacement::draft(0x1404c0510, "unknown_1404c0510", tpl_60530::<0x1426467e8, 0x143c8d87c, 0x142489ee0, 0x143c8dbc8, 0x1404c0539> as *const ()), // sub_1404c0510
    Replacement::draft(0x1404c06d0, "unknown_1404c06d0", fn_1404c06d0 as *const ()), // sub_1404c06d0
    Replacement::draft(0x1404c0880, "unknown_1404c0880", fn_1404c0880 as *const ()), // sub_1404c0880
    Replacement::draft_throws(0x1404c0a40, "unknown_1404c0a40", tpl_4807::<0x1411e9f50, 0x1404c3330, 0x143c98fc0, 0x1401fc320, 0x1404c3be0, 0x143ca7e1c, 0x1420af120, 0x140215750, 0x1404c2240, 0x1420a28e0, 0x1404c2a40, 0x1401de410> as *const (), Throws::Discard), // sub_1404c0a40
    Replacement::draft_throws(0x1404c0c30, "unknown_1404c0c30", tpl_3110::<0x1404c31b0, 0x143ca7a90, 0x1420af120, 0x1404c3070, 0x143ca7bf8, 0x140215750, 0x143c8ceec, 0x140215610, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404c0c30
    Replacement::draft(0x1404c0e20, "unknown_1404c0e20", tpl_1374::<0x88, 0x10088, 0x1404c0ee0, 0x1404c1410, 0x1404c0fa0> as *const ()), // sub_1404c0e20
    Replacement::draft(0x1404c0ee0, "unknown_1404c0ee0", tpl_1375::<0x143ca7bf8, 0x20, 0x8, 0x142407c80, 0x580f0, 0x580e0, 0x143c8ceec, 0x143c8d08c, 0x10, 0x18, 0x28> as *const ()), // sub_1404c0ee0
    Replacement::draft(0x1404c0fa0, "unknown_1404c0fa0", tpl_1376::<0x18, 0x20, 0x10, 0x8, 0x28, 0x141a4eeb0> as *const ()), // sub_1404c0fa0
    Replacement::draft(0x1404c10a0, "unknown_1404c10a0", tpl_4808::<0x88, 0x10088, 0x143ca7c00, 0x20, 0x8, 0x1404c12b0, 0x1404c1180> as *const ()), // sub_1404c10a0
    Replacement::draft(0x1404c1180, "unknown_1404c1180", tpl_4809::<0x18, 0x20, 0x8, 0x141a4ebc0> as *const ()), // sub_1404c1180
    Replacement::draft(0x1404c1260, "content_1404c1260", fn_1404c1260 as *const ()), // content::GameArmoryComponent::content::GameArmoryComponent::vfunc_0
    Replacement::draft(0x1404c1630, "unknown_1404c1630", fn_1404c1630 as *const ()), // sub_1404c1630
    Replacement::draft(0x1404c1730, "unknown_1404c1730", fn_1404c1730 as *const ()), // sub_1404c1730
    Replacement::draft(0x1404c17b0, "unknown_1404c17b0", tpl_3798::<0x54, 0x14240ab10> as *const ()), // sub_1404c17b0
    Replacement::draft(0x1404c17f0, "unknown_1404c17f0", tpl_1124::<0x60> as *const ()), // sub_1404c17f0
    Replacement::draft_throws(0x1404c1820, "unknown_1404c1820", fn_1404c1820 as *const (), Throws::Caught), // sub_1404c1820
    Replacement::draft(0x1404c1920, "unknown_1404c1920", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x1404bf310> as *const ()), // sub_1404c1920
    Replacement::draft(0x1404c19b0, "unknown_1404c19b0", tpl_61113::<0x1404c0ee0, 0x1404c1410, 0x141a4eeb0> as *const ()), // sub_1404c19b0
    Replacement::draft(0x1404c1b70, "unknown_1404c1b70", tpl_64008::<0x143ca7c00, 0x1404c12b0, 0x141a4ebc0> as *const ()), // sub_1404c1b70
    Replacement::draft(0x1404c1d40, "unknown_1404c1d40", tpl_1956::<0x88, 0x10088, 0x143ca7d3c, 0x20, 0x8, 0x1404c1f20, 0x1404c1e20> as *const ()), // sub_1404c1d40
    Replacement::draft(0x1404c1e20, "unknown_1404c1e20", tpl_1957::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141a4ed60> as *const ()), // sub_1404c1e20
    Replacement::draft_throws(0x1404c2240, "unknown_1404c2240", tpl_1498::<0x143c88518, 0x143c8850c, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8df88, 0x143c88508, 0x143c88510, 0x141834a0c> as *const (), Throws::Discard), // sub_1404c2240
    Replacement::draft(0x1404c2320, "unknown_1404c2320", fn_1404c2320 as *const ()), // sub_1404c2320
    Replacement::draft(0x1404c2500, "unknown_1404c2500", fn_1404c2500 as *const ()), // sub_1404c2500
    Replacement::draft_throws(0x1404c27a0, "unknown_1404c27a0", fn_1404c27a0 as *const (), Throws::Caught), // sub_1404c27a0
    Replacement::draft(0x1404c2960, "unknown_1404c2960", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1404bf310> as *const ()), // sub_1404c2960
    Replacement::draft(0x1404c2a40, "unknown_1404c2a40", fn_1404c2a40 as *const ()), // sub_1404c2a40
    Replacement::draft(0x1404c2ba0, "unknown_1404c2ba0", tpl_61593::<0x143ca7d3c, 0x1404c1fd0, 0x141a4ed60> as *const ()), // sub_1404c2ba0
    Replacement::draft(0x1404c3330, "unknown_1404c3330", tpl_3188::<0x1404c3c90, 0x14315f0e8, 0x1404c37e0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1404c3330
    Replacement::draft(0x1404c33b0, "unknown_1404c33b0", fn_1404c33b0 as *const ()), // sub_1404c33b0
    Replacement::draft(0x1404c3480, "unknown_1404c3480", tpl_4348::<0x28, 0x30, 0x1404c37e0> as *const ()), // sub_1404c3480
    Replacement::draft(0x1404c3680, "unknown_1404c3680", tpl_4820::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1401ff5b0, 0x142442de0> as *const ()), // sub_1404c3680
    Replacement::draft(0x1404c37e0, "unknown_1404c37e0", tpl_4821::<0x1401de830, 0x1401f2bc0, 0x1425371a0, 0x142533200> as *const ()), // sub_1404c37e0
    Replacement::draft(0x1404c3a40, "unknown_1404c3a40", tpl_4822::<0x143ca7bbc, 0x143ca79e0, 0x141834a78, 0x143c8d694, 0x143c9d1c4, 0x143c8d160, 0x143c8ea68, 0x143c8dbd4, 0x143c8d87c, 0x143c8dbc8, 0x143ca7a00, 0x143ca7a20, 0x143ca7a30, 0x141834a0c> as *const ()), // sub_1404c3a40
    Replacement::draft(0x1404c3c90, "unknown_1404c3c90", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14315f0e8, 0x8, 0x20> as *const ()), // sub_1404c3c90
    Replacement::draft(0x1404c3f60, "unknown_1404c3f60", tpl_1094::<0x143ca7e20, 0x1404c4280, 0x14020e8f0> as *const ()), // sub_1404c3f60
    Replacement::draft(0x1404c4280, "unknown_1404c4280", fn_1404c4280 as *const ()), // sub_1404c4280
    Replacement::draft(0x1404c45d0, "unknown_1404c45d0", fn_1404c45d0 as *const ()), // sub_1404c45d0
    Replacement::draft(0x1404c4850, "unknown_1404c4850", fn_1404c4850 as *const ()), // sub_1404c4850
    Replacement::draft(0x1404c48c0, "unknown_1404c48c0", tpl_855::<0x1404c1730, 0x1425371a0, 0x142533200, 0x1404c2320, 0x1404c33b0> as *const ()), // sub_1404c48c0
    Replacement::draft(0x1404c4a20, "unknown_1404c4a20", fn_1404c4a20 as *const ()), // sub_1404c4a20
    Replacement::draft(0x1404c4b80, "unknown_1404c4b80", tpl_1094::<0x143c914dc, 0x1404c5d40, 0x14020e8f0> as *const ()), // sub_1404c4b80
    Replacement::draft(0x1404c4d80, "unknown_1404c4d80", tpl_1466::<0x143c914b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404c4d80
    Replacement::draft(0x1404c5050, "unknown_1404c5050", fn_1404c5050 as *const ()), // sub_1404c5050
    Replacement::draft(0x1404c5240, "unknown_1404c5240", tpl_1094::<0x143c914ac, 0x1404c5f20, 0x14020e8f0> as *const ()), // sub_1404c5240
    Replacement::draft_throws(0x1404c5470, "bigfish_1404c5470", tpl_4828::<0x143ca80b8, 0x143ca8090, 0x1411e9f50, 0x1404c6d60, 0x143ca80c0, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1404c66c0, 0x1404c62e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431621c6, 0x142646960, 0x143ca80b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameaimtarget::streamIn1>
    Replacement::draft_throws(0x1404c5760, "bigfish_1404c5760", tpl_4828::<0x143ca813c, 0x143ca7f80, 0x1411e9f50, 0x1404c6c20, 0x143ca801c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1404c6810, 0x1404c63e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143162626, 0x142646960, 0x143ca7fa0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameaimtarget::streamIn2>
    Replacement::draft_throws(0x1404c5a50, "bigfish_1404c5a50", tpl_4828::<0x143ca80b4, 0x143ca8068, 0x1411e9f50, 0x1404c6b10, 0x143ca8138, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1404c69c0, 0x1404c64c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431623f6, 0x142646960, 0x143ca8088, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::gameaimtarget::streamOut>
    Replacement::draft(0x1404c5d40, "unknown_1404c5d40", fn_1404c5d40 as *const ()), // sub_1404c5d40
    Replacement::draft(0x1404c5f20, "unknown_1404c5f20", tpl_4368::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1404c60a0> as *const ()), // sub_1404c5f20
    Replacement::draft(0x1404c60a0, "unknown_1404c60a0", fn_1404c60a0 as *const ()), // sub_1404c60a0
    Replacement::draft(0x1404c61f0, "unknown_1404c61f0", fn_1404c61f0 as *const ()), // sub_1404c61f0
    Replacement::draft(0x1404c62e0, "unknown_1404c62e0", tpl_63093::<0x143c914dc, 0x142489ee0, 0x141a50190> as *const ()), // sub_1404c62e0
    Replacement::draft(0x1404c63e0, "unknown_1404c63e0", tpl_64022::<0x88, 0x10088, 0x1404c65c0, 0x141a4ff10> as *const ()), // sub_1404c63e0
    Replacement::draft(0x1404c64c0, "unknown_1404c64c0", tpl_63093::<0x143c914ac, 0x142489ee0, 0x141a4fd10> as *const ()), // sub_1404c64c0
    Replacement::draft(0x1404c66c0, "unknown_1404c66c0", tpl_63099::<0x143c914dc, 0x142489ee0, 0x141a50190> as *const ()), // sub_1404c66c0
    Replacement::draft(0x1404c6810, "unknown_1404c6810", tpl_64023::<0x143c914dc, 0x142489ee0, 0x143c8ec20, 0x141a4ff10> as *const ()), // sub_1404c6810
    Replacement::draft(0x1404c69c0, "unknown_1404c69c0", tpl_63099::<0x143c914ac, 0x142489ee0, 0x141a4fd10> as *const ()), // sub_1404c69c0
    Replacement::draft(0x1404c7010, "unknown_1404c7010", fn_1404c7010 as *const ()), // sub_1404c7010
    Replacement::draft(0x1404c70e0, "unknown_1404c70e0", fn_1404c70e0 as *const ()), // sub_1404c70e0
    Replacement::draft(0x1404c71e0, "unknown_1404c71e0", tpl_3943::<0x30, 0x143c8cf5c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1404c7263> as *const ()), // sub_1404c71e0
    Replacement::draft_throws(0x1404c7270, "bigfish_1404c7270", fn_1404c7270 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::floatingtext::system>
    Replacement::draft(0x1404c7680, "bigfish_1404c7680", tpl_1094::<0x143ca8288, 0x1404c82b0, 0x14020e8f0> as *const ()), // sub_1404c7680
    Replacement::draft(0x1404c7880, "bigfish_1404c7880", tpl_1094::<0x143ca8328, 0x1404c84a0, 0x14020e8f0> as *const ()), // sub_1404c7880
    Replacement::draft(0x1404c7a80, "bigfish_1404c7a80", tpl_1094::<0x143ca8328, 0x1404c8620, 0x14020e8f0> as *const ()), // sub_1404c7a80
    Replacement::draft_throws(0x1404c7c80, "bigfish_1404c7c80", tpl_4834::<0x143ca8188, 0x143ca82b8, 0x1411e9f50, 0x1404c95a0, 0x143ca839c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1404c9160, 0x1404c8dd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431633f6, 0x142646960, 0x143ca82d8, 0x143ca82e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::floatingtext::streamInFloatingText>
    Replacement::draft_throws(0x1404c7f80, "bigfish_1404c7f80", tpl_4835::<0x143ca81b8, 0x143ca81e8, 0x1411e9f50, 0x1404c94b0, 0x143ca818c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1404c92b0, 0x1404c8ed0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143163636, 0x142646960, 0x143ca8208, 0x143ca8210, 0x143ca8214, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::floatingtext::streamOutFloatingText>
    Replacement::draft(0x1404c82b0, "unknown_1404c82b0", fn_1404c82b0 as *const ()), // sub_1404c82b0
    Replacement::draft(0x1404c84a0, "unknown_1404c84a0", fn_1404c84a0 as *const ()), // sub_1404c84a0
    Replacement::draft(0x1404c8620, "unknown_1404c8620", fn_1404c8620 as *const ()), // sub_1404c8620
    Replacement::draft(0x1404c8800, "unknown_1404c8800", tpl_64030::<0x88, 0x10088, 0x1404c8a00, 0x141a516d0> as *const ()), // sub_1404c8800
    Replacement::draft(0x1404c8920, "unknown_1404c8920", fn_1404c8920 as *const ()), // sub_1404c8920
    Replacement::draft(0x1404c8b30, "unknown_1404c8b30", tpl_1124::<0xa8> as *const ()), // sub_1404c8b30
    Replacement::draft(0x1404c8b70, "unknown_1404c8b70", fn_1404c8b70 as *const ()), // sub_1404c8b70
    Replacement::draft(0x1404c8bb0, "unknown_1404c8bb0", tpl_64033::<0x143ca8328, 0x142489ee0, 0x143ca8288, 0x143c8d120, 0x141a516d0> as *const ()), // sub_1404c8bb0
    Replacement::draft(0x1404c8dd0, "unknown_1404c8dd0", tpl_63093::<0x143ca8288, 0x142489ee0, 0x141a51440> as *const ()), // sub_1404c8dd0
    Replacement::draft(0x1404c8ed0, "unknown_1404c8ed0", tpl_1898::<0x88, 0x10088, 0x141a51310> as *const ()), // sub_1404c8ed0
    Replacement::draft(0x1404c9b70, "unknown_1404c9b70", tpl_4843::<0x10, 0xc> as *const ()), // sub_1404c9b70
    Replacement::draft(0x1404c9bc0, "unknown_1404c9bc0", fn_1404c9bc0 as *const ()), // sub_1404c9bc0
    Replacement::draft(0x1404c9d80, "unknown_1404c9d80", fn_1404c9d80 as *const ()), // sub_1404c9d80
    Replacement::draft(0x1404c9e60, "unknown_1404c9e60", tpl_4844::<0x1401e7c10, 0x1404c9fc0> as *const ()), // sub_1404c9e60
    Replacement::draft(0x1404c9fc0, "unknown_1404c9fc0", tpl_63073::<0x143c8ec68, 0x142489ee0, 0x143ca8568, 0x143c917cc> as *const ()), // sub_1404c9fc0
    Replacement::draft(0x1404ca550, "unknown_1404ca550", tpl_61019::<0x143c8e65c, 0x142489ee0, 0x143c8d120, 0x143c8d160, 0x1426467e8, 0x1404ca6b3> as *const ()), // sub_1404ca550
    Replacement::draft_throws(0x1404ca6c0, "bigfish_1404ca6c0", tpl_1100::<0x143ca8600, 0x143ca87f0, 0x1411e9f50, 0x1404cdca0, 0x143ca89c4, 0x1420af120, 0x1420a28e0, 0x1404cc520, 0x1404cb140, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431649e6, 0x142646960, 0x143ca8810, 0x143ca8820, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight::targetmodule::clean::system>
    Replacement::draft_throws(0x1404ca9a0, "bigfish_1404ca9a0", tpl_4194::<0x143ca87e4, 0x143ca8980, 0x1411e9f50, 0x1404cb240, 0x1404cc670, 0x1404cb530, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143163aa6, 0x142646960, 0x143ca89a0, 0x143ca89b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight::targetmodule::find_targets::system>
    Replacement::draft_throws(0x1404cabf0, "bigfish_1404cabf0", tpl_3238::<0x143ca8608, 0x143ca87a0, 0x1411e9f50, 0x1404cb920, 0x1404cc8b0, 0x1404cbbd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143164426, 0x142646960, 0x143ca87c0, 0x143ca87d0, 0x143ca87d8, 0x143ca87dc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight::targetmodule::update_boost_stun::system>
    Replacement::draft_throws(0x1404cae50, "bigfish_1404cae50", tpl_4528::<0x143ca888c, 0x143ca8938, 0x1411e9f50, 0x1404cbe50, 0x1404ccbc0, 0x1404cc040, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143165046, 0x142646960, 0x143ca8958, 0x143ca8968, 0x143ca8970, 0x143ca8974, 0x143ca8976, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight::targetmodule::update_boost_ambush::system>
    Replacement::draft(0x1404cb0c0, "unknown_1404cb0c0", fn_1404cb0c0 as *const ()), // sub_1404cb0c0
    Replacement::draft(0x1404cb140, "unknown_1404cb140", fn_1404cb140 as *const ()), // sub_1404cb140
    Replacement::draft_throws(0x1404cb240, "unknown_1404cb240", tpl_4847::<0x1404cdb20, 0x143ca8708, 0x1420af120, 0x1404cd970, 0x143ca88cc, 0x140215750, 0x1404cd7d0, 0x143ca85fc, 0x143c8ceec, 0x140215610, 0x143c8f63c, 0x143c8d910, 0x143c8ef20, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404cb240
    Replacement::draft_throws(0x1404cb530, "unknown_1404cb530", fn_1404cb530 as *const (), Throws::Discard), // sub_1404cb530
    Replacement::draft(0x1404cb700, "unknown_1404cb700", tpl_1444::<0x8, 0x20, 0x143ca88cc, 0x143c8ceec, 0x143ca85fc, 0x143c8ef20, 0x580e0, 0x143c8f63c, 0x143c8d910> as *const ()), // sub_1404cb700
    Replacement::draft_throws(0x1404cb7e0, "unknown_1404cb7e0", fn_1404cb7e0 as *const (), Throws::Discard), // sub_1404cb7e0
    Replacement::draft_throws(0x1404cb920, "unknown_1404cb920", fn_1404cb920 as *const (), Throws::Discard), // sub_1404cb920
    Replacement::draft(0x1404cbbd0, "unknown_1404cbbd0", fn_1404cbbd0 as *const ()), // sub_1404cbbd0
    Replacement::draft(0x1404cbd30, "unknown_1404cbd30", fn_1404cbd30 as *const ()), // sub_1404cbd30
    Replacement::draft_throws(0x1404cbe50, "unknown_1404cbe50", tpl_4853::<0x1404cd380, 0x143ca88d0, 0x1420af120, 0x1404cd2d0, 0x143ca882c, 0x140215750, 0x1404cd1e0, 0x143ca85f0, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404cbe50
    Replacement::draft(0x1404cc040, "unknown_1404cc040", tpl_64043::<0x143ca882c, 0x143ca85f0, 0x1404cc1a0, 0x141a53500> as *const ()), // sub_1404cc040
    Replacement::draft(0x1404cc520, "unknown_1404cc520", fn_1404cc520 as *const ()), // sub_1404cc520
    Replacement::draft_throws(0x1404cc670, "unknown_1404cc670", fn_1404cc670 as *const (), Throws::Discard), // sub_1404cc670
    Replacement::draft(0x1404cc8b0, "unknown_1404cc8b0", fn_1404cc8b0 as *const ()), // sub_1404cc8b0
    Replacement::draft(0x1404ccbc0, "unknown_1404ccbc0", tpl_64047::<0x143ca882c, 0x143ca85f0, 0x143ca87e8, 0x142489ee0, 0x143c9d198, 0x141a53500> as *const ()), // sub_1404ccbc0
    Replacement::draft(0x1404cd7d0, "unknown_1404cd7d0", tpl_3389::<0x143ca8648, 0x143ca84b0, 0x141834a78, 0x143ca5918, 0x143c8d160, 0x143c8d87c, 0x143c8d120, 0x143ca87e8, 0x143c98134, 0x143ca88bc, 0x143ca84d0, 0x143ca84f0, 0x143ca8500, 0x141834a0c> as *const ()), // sub_1404cd7d0
    Replacement::draft(0x1404cd970, "unknown_1404cd970", fn_1404cd970 as *const ()), // sub_1404cd970
    Replacement::draft(0x1404ce2f0, "unknown_1404ce2f0", fn_1404ce2f0 as *const ()), // sub_1404ce2f0
    Replacement::draft(0x1404ce3d0, "content_1404ce3d0", fn_1404ce3d0 as *const ()), // content::FlashlightDamageComponent::content::FlashlightDamageComponent::vfunc_0
    Replacement::draft(0x1404ce430, "content_1404ce430", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143ca89f0, 0x1426467e8, 0x143c8d160, 0x142489ee0, 0x143c98134, 0x1404ce523> as *const ()), // sub_1404ce430
    Replacement::draft(0x1404ce6a0, "content_1404ce6a0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143ca8a30, 0x1426467e8, 0x143c98134, 0x142489ee0, 0x1404ce797> as *const ()), // sub_1404ce6a0
    Replacement::draft(0x1404ce8b0, "content_1404ce8b0", fn_1404ce8b0 as *const ()), // sub_1404ce8b0
    Replacement::draft(0x1404ce9a0, "content_1404ce9a0", tpl_1403::<0x143c9f060, 0x1404cebb0, 0x14020e8f0> as *const ()), // sub_1404ce9a0
    Replacement::draft(0x1404cebb0, "content_1404cebb0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143106928> as *const ()), // sub_1404cebb0
    Replacement::draft_throws(0x1404cedc0, "content_1404cedc0", tpl_4017::<0x143ca89f0, 0x143ca8a34, 0x143ca89c8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c98134, 0x143c9f060, 0x143ca89e8, 0x141834a0c> as *const (), Throws::Discard), // sub_1404cedc0
    Replacement::draft(0x1404cf020, "unknown_1404cf020", fn_1404cf020 as *const ()), // sub_1404cf020
    Replacement::draft(0x1404cf1a0, "unknown_1404cf1a0", fn_1404cf1a0 as *const ()), // sub_1404cf1a0
    Replacement::draft(0x1404cf300, "unknown_1404cf300", tpl_63073::<0x143c9d658, 0x142489ee0, 0x143c9e4a8, 0x143c976e0> as *const ()), // sub_1404cf300
    Replacement::draft(0x1404cf880, "unknown_1404cf880", fn_1404cf880 as *const ()), // sub_1404cf880
    Replacement::draft(0x1404cfa80, "unknown_1404cfa80", fn_1404cfa80 as *const ()), // sub_1404cfa80
    Replacement::draft_throws(0x1404cfb60, "bigfish_1404cfb60", fn_1404cfb60 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::enable_feature::system>
    Replacement::draft_throws(0x1404cfec0, "bigfish_1404cfec0", tpl_4197::<0x143ca935c, 0x143ca8c80, 0x1411e9f50, 0x1404d9230, 0x143ca8d2c, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x1404d5e50, 0x1404d2800, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316d276, 0x142646960, 0x143ca8ca0, 0x143ca8cb0, 0x143ca8cb4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::boosting_input::system>
    Replacement::draft_throws(0x1404d01e0, "bigfish_1404d01e0", tpl_3465::<0x143ca8f38, 0x143ca9448, 0x1411e9f50, 0x1404d2900, 0x1404d6010, 0x1404d2b70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316ca86, 0x142646960, 0x143ca9468, 0x143ca9478, 0x143ca947a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::drain_update::system>
    Replacement::draft_throws(0x1404d0430, "bigfish_1404d0430", tpl_3464::<0x143ca9260, 0x143ca8a40, 0x1411e9f50, 0x1404d2f10, 0x1404d6250, 0x1404d3180, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316c426, 0x142646960, 0x143ca8a60, 0x143ca8a70, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::reload_update::system>
    Replacement::draft_throws(0x1404d0680, "bigfish_1404d0680", tpl_4863::<0x143ca9800, 0x143ca95c8, 0x1411e9f50, 0x1404d8b60, 0x143ca97ac, 0x1420af120, 0x14041ea80, 0x143c9e294, 0x140215750, 0x1420a28e0, 0x1404d6450, 0x1404d34f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316c0c6, 0x142646960, 0x143ca95e8, 0x143ca95f8, 0x143ca95fc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::reload_finished::system>
    Replacement::draft_throws(0x1404d09e0, "bigfish_1404d09e0", tpl_3368::<0x143ca8ef4, 0x143ca8af8, 0x1411e9f50, 0x1404d3640, 0x1404d6640, 0x1404d3860, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316bc46, 0x142646960, 0x143ca8b18, 0x143ca8b28, 0x143ca8b30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::auto_refill_update::system>
    Replacement::draft_throws(0x1404d0c30, "bigfish_1404d0c30", tpl_3368::<0x143ca925c, 0x143ca9140, 0x1411e9f50, 0x1404d3a00, 0x1404d68d0, 0x1404d3c20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316b5f6, 0x142646960, 0x143ca9160, 0x143ca9170, 0x143ca9178, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::light_damage_boost::system>
    Replacement::draft_throws(0x1404d0e80, "bigfish_1404d0e80", tpl_1296::<0x143ca9264, 0x143ca97c0, 0x1411e9f50, 0x1404d3ec0, 0x1404d6b20, 0x1404d40f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316a276, 0x142646960, 0x143ca97e0, 0x143ca97f0, 0x143ca97f8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::battery_item_usable::system>
    Replacement::draft_throws(0x1404d10e0, "bigfish_1404d10e0", tpl_3465::<0x143ca9360, 0x143ca9370, 0x1411e9f50, 0x1404d4320, 0x1404d6d60, 0x1404d4520, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143169c06, 0x142646960, 0x143ca9390, 0x143ca93a0, 0x143ca93a2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::reload_input::system>
    Replacement::draft_throws(0x1404d1330, "bigfish_1404d1330", tpl_3464::<0x143ca8fe4, 0x143ca9690, 0x1411e9f50, 0x1404d4780, 0x1404d6fc0, 0x1404d49b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143169436, 0x142646960, 0x143ca96b0, 0x143ca96c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::check_battery::system>
    Replacement::draft_throws(0x1404d1580, "bigfish_1404d1580", tpl_1775::<0x143ca97b4, 0x143ca8c40, 0x1411e9f50, 0x1404d7cf0, 0x143ca8df4, 0x1420af120, 0x1404d7c00, 0x143ca93a8, 0x140215750, 0x1420a28e0, 0x1404d71d0, 0x1404d4cd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431690b6, 0x142646960, 0x143ca8c60, 0x143ca8c70, 0x143ca8c78, 0x143ca8c7c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_resource::animation_params_update::system>
    Replacement::draft(0x1404d1900, "unknown_1404d1900", tpl_1403::<0x143ca8d38, 0x1404d2520, 0x14020e8f0> as *const ()), // sub_1404d1900
    Replacement::draft(0x1404d1ae0, "unknown_1404d1ae0", tpl_3270::<0x143c988d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404d1ae0
    Replacement::draft(0x1404d1e20, "unknown_1404d1e20", tpl_3354::<0x143c9d198, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404d1e20
    Replacement::draft(0x1404d2170, "unknown_1404d2170", tpl_4864::<0x143ca8dec, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404d2170
    Replacement::draft(0x1404d24a0, "unknown_1404d24a0", tpl_4865::<0x8, 0x1404d1900, 0x1404d1ae0> as *const ()), // sub_1404d24a0
    Replacement::draft(0x1404d2520, "unknown_1404d2520", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431667e0> as *const ()), // sub_1404d2520
    Replacement::draft(0x1404d26a0, "unknown_1404d26a0", tpl_4341::<0x143ca9640, 0x142407c80, 0x143c98134, 0x142489ee0, 0x141a59860> as *const ()), // sub_1404d26a0
    Replacement::draft(0x1404d2800, "unknown_1404d2800", tpl_64057::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x1404d5ab0, 0x141a59800> as *const ()), // sub_1404d2800
    Replacement::draft_throws(0x1404d2900, "unknown_1404d2900", tpl_4866::<0x1404d9080, 0x143ca97b8, 0x1420af120, 0x1404d8f00, 0x143ca952c, 0x140215750, 0x14035a280, 0x143c8d910, 0x140215610, 0x143c8ceec, 0x143c9da5c, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404d2900
    Replacement::draft(0x1404d2b70, "unknown_1404d2b70", fn_1404d2b70 as *const ()), // sub_1404d2b70
    Replacement::draft(0x1404d2ce0, "unknown_1404d2ce0", tpl_1444::<0x8, 0x20, 0x143ca952c, 0x143c8d910, 0x143c94108, 0x143c8d08c, 0x580e0, 0x143c8ceec, 0x143c9da5c> as *const ()), // sub_1404d2ce0
    Replacement::draft(0x1404d2dc0, "unknown_1404d2dc0", fn_1404d2dc0 as *const ()), // sub_1404d2dc0
    Replacement::draft_throws(0x1404d2f10, "unknown_1404d2f10", tpl_4866::<0x1404d8dc0, 0x143ca8e70, 0x1420af120, 0x1404d8c70, 0x143ca8f94, 0x140215750, 0x140355330, 0x143c8d910, 0x140215610, 0x143c8ceec, 0x143c9da5c, 0x143c8d00c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404d2f10
    Replacement::draft(0x1404d3180, "unknown_1404d3180", fn_1404d3180 as *const ()), // sub_1404d3180
    Replacement::draft(0x1404d32e0, "unknown_1404d32e0", tpl_1444::<0x8, 0x20, 0x143ca8f94, 0x143c8d910, 0x143c93d20, 0x143c8d00c, 0x580e0, 0x143c8ceec, 0x143c9da5c> as *const ()), // sub_1404d32e0
    Replacement::draft(0x1404d33c0, "unknown_1404d33c0", fn_1404d33c0 as *const ()), // sub_1404d33c0
    Replacement::draft(0x1404d34f0, "unknown_1404d34f0", tpl_61053::<0x88, 0x10088, 0x143c9e294, 0x20, 0x8, 0x1404d5630, 0x141a57f30> as *const ()), // sub_1404d34f0
    Replacement::draft_throws(0x1404d3640, "unknown_1404d3640", fn_1404d3640 as *const (), Throws::Discard), // sub_1404d3640
    Replacement::draft(0x1404d3860, "unknown_1404d3860", fn_1404d3860 as *const ()), // sub_1404d3860
    Replacement::draft_throws(0x1404d3a00, "unknown_1404d3a00", fn_1404d3a00 as *const (), Throws::Discard), // sub_1404d3a00
    Replacement::draft(0x1404d3c20, "unknown_1404d3c20", tpl_4873::<0x143ca9024, 0x143c8d910, 0x143c8ceec, 0x143c9da5c, 0x1404d53e0, 0x1404d3da0> as *const ()), // sub_1404d3c20
    Replacement::draft(0x1404d3da0, "unknown_1404d3da0", tpl_4874::<0x18, 0x20, 0x40, 0x28, 0x10, 0x8, 0x141a58410> as *const ()), // sub_1404d3da0
    Replacement::draft_throws(0x1404d3ec0, "unknown_1404d3ec0", fn_1404d3ec0 as *const (), Throws::Discard), // sub_1404d3ec0
    Replacement::draft(0x1404d40f0, "unknown_1404d40f0", fn_1404d40f0 as *const ()), // sub_1404d40f0
    Replacement::draft(0x1404d4210, "unknown_1404d4210", fn_1404d4210 as *const ()), // sub_1404d4210
    Replacement::draft_throws(0x1404d4320, "unknown_1404d4320", fn_1404d4320 as *const (), Throws::Discard), // sub_1404d4320
    Replacement::draft(0x1404d4520, "unknown_1404d4520", fn_1404d4520 as *const ()), // sub_1404d4520
    Replacement::draft(0x1404d4660, "unknown_1404d4660", fn_1404d4660 as *const ()), // sub_1404d4660
    Replacement::draft_throws(0x1404d4780, "unknown_1404d4780", fn_1404d4780 as *const (), Throws::Discard), // sub_1404d4780
    Replacement::draft(0x1404d49b0, "unknown_1404d49b0", fn_1404d49b0 as *const ()), // sub_1404d49b0
    Replacement::draft(0x1404d4a70, "unknown_1404d4a70", tpl_3101::<0x8, 0x20, 0x143ca9648, 0x143c8ceec, 0x143c93d20, 0x143c94108, 0x580e0, 0x143c9da5c, 0x143c8d00c> as *const ()), // sub_1404d4a70
    Replacement::draft(0x1404d4b80, "unknown_1404d4b80", fn_1404d4b80 as *const ()), // sub_1404d4b80
    Replacement::draft(0x1404d4cd0, "unknown_1404d4cd0", tpl_1781::<0x88, 0x10088, 0x143ca93a8, 0x20, 0x8, 0x1404d4e90, 0x1404d4db0> as *const ()), // sub_1404d4cd0
    Replacement::draft(0x1404d4db0, "unknown_1404d4db0", tpl_1782::<0x18, 0x20, 0x30, 0x10, 0x28, 0x8, 0x141a56e30> as *const ()), // sub_1404d4db0
    Replacement::draft_throws(0x1404da250, "bigfish_1404da250", tpl_4886::<0x143ca98b0, 0x143ca9940, 0x1411e9f50, 0x1404db1f0, 0x143ca9848, 0x1420af120, 0x1403b1e40, 0x1420a28e0, 0x1404dae50, 0x1404dac70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14316dca6, 0x142646960, 0x143ca9960, 0x143ca9970, 0x143ca9974, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlightreceiver::flashlightReceiverSystem>
    Replacement::draft(0x1404da530, "unknown_1404da530", tpl_1094::<0x143ca984c, 0x1404daaf0, 0x14020e8f0> as *const ()), // sub_1404da530
    Replacement::draft(0x1404da730, "unknown_1404da730", tpl_3080::<0x143ca9850, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404da730
    Replacement::draft(0x1404daa70, "unknown_1404daa70", tpl_4887::<0x8, 0x1404da530, 0x1404da730> as *const ()), // sub_1404daa70
    Replacement::draft(0x1404daaf0, "unknown_1404daaf0", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14316dec8> as *const ()), // sub_1404daaf0
    Replacement::draft(0x1404dac70, "unknown_1404dac70", tpl_64022::<0x88, 0x10088, 0x1404dad50, 0x141a5a400> as *const ()), // sub_1404dac70
    Replacement::draft(0x1404dae50, "unknown_1404dae50", tpl_64023::<0x143ca9850, 0x142489ee0, 0x143ca984c, 0x141a5a400> as *const ()), // sub_1404dae50
    Replacement::draft(0x1404db430, "content_1404db430", tpl_4888::<0x30, 0x143c8ea68, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1404db4b3> as *const ()), // sub_1404db430
    Replacement::draft(0x1404db4c0, "content_1404db4c0", tpl_4889::<0x14021f130, 0x143084e28> as *const ()), // sub_1404db4c0
    Replacement::draft(0x1404db4f0, "content_1404db4f0", tpl_1953::<0x14021f130, 0x143084e28, 0x141834d30> as *const ()), // content::FlashlightInventoryComponent::content::FlashlightInventoryComponent::vfunc_0
    Replacement::draft(0x1404db540, "unknown_1404db540", tpl_1842::<0x1401e7c10, 0x143c8e840, 0x142489ee0> as *const ()), // sub_1404db540
    Replacement::draft(0x1404db690, "unknown_1404db690", fn_1404db690 as *const ()), // sub_1404db690
    Replacement::draft(0x1404db9e0, "unknown_1404db9e0", fn_1404db9e0 as *const ()), // sub_1404db9e0
    Replacement::draft(0x1404dbb60, "unknown_1404dbb60", tpl_61197::<0x143c9efb0, 0x142489ee0, 0x143c8d160, 0x143c9d198, 0x143c98134, 0x1426467e8, 0x1404dbcf3> as *const ()), // sub_1404dbb60
    Replacement::draft(0x1404dbd00, "unknown_1404dbd00", fn_1404dbd00 as *const ()), // sub_1404dbd00
    Replacement::draft_throws(0x1404dbd70, "unknown_1404dbd70", fn_1404dbd70 as *const (), Throws::Caught), // sub_1404dbd70
    Replacement::draft(0x1404dbf80, "unknown_1404dbf80", tpl_61019::<0x143c8d160, 0x142489ee0, 0x143c988d0, 0x143c98134, 0x1426467e8, 0x1404dc0e3> as *const ()), // sub_1404dbf80
    Replacement::draft_throws(0x1404dc0f0, "unknown_1404dc0f0", tpl_1480::<0x142745a70, 0x1410d6ca0, 0x1404dfe90, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404dc0f0
    Replacement::draft_throws(0x1404dc390, "unknown_1404dc390", tpl_1480::<0x142745a70, 0x1410d6ca0, 0x1404e00a0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1404dc390
    Replacement::draft_throws(0x1404dc630, "bigfish_1404dc630", fn_1404dc630 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_light_spawn::system>
    Replacement::draft_throws(0x1404dc970, "bigfish_1404dc970", tpl_4895::<0x143caaa20, 0x143caaaa0, 0x1411e9f50, 0x1404e7e10, 0x143ca9df0, 0x1420af120, 0x143c8df88, 0x140215610, 0x1420a28e0, 0x1404e45e0, 0x1404e1650, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143171776, 0x142646960, 0x143caaac0, 0x143caaad0, 0x143caaad8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_spawn::system>
    Replacement::draft_throws(0x1404dcc90, "bigfish_1404dcc90", tpl_4896::<0x143caa0f4, 0x143caa100, 0x1411e9f50, 0x1404e1760, 0x1404e47b0, 0x1404e1950, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431719c6, 0x142646960, 0x143caa120, 0x143caa140, 0x143caa144, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_spawned_light_init::system>
    Replacement::draft_throws(0x1404dcef0, "bigfish_1404dcef0", tpl_1295::<0x143caa624, 0x143caa230, 0x1404e02c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143171fd6, 0x142646960, 0x143caa250, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_spawned_init::system>
    Replacement::draft_throws(0x1404dd0f0, "bigfish_1404dd0f0", tpl_4896::<0x143caa374, 0x143caa1c0, 0x1411e9f50, 0x1404e1e00, 0x1404e4c10, 0x1404e2000, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143172486, 0x142646960, 0x143caa1e0, 0x143caa200, 0x143caa204, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_boosted_activation::system>
    Replacement::draft_throws(0x1404dd350, "bigfish_1404dd350", tpl_4896::<0x143caa5bc, 0x143caa2a0, 0x1411e9f50, 0x1404e2290, 0x1404e4f50, 0x1404e2420, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143172a16, 0x142646960, 0x143caa2c0, 0x143caa2e0, 0x143caa2e4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_grapple_fake_boost::system>
    Replacement::draft_throws(0x1404dd5b0, "bigfish_1404dd5b0", tpl_1296::<0x143caa370, 0x143caa440, 0x1411e9f50, 0x1404e2640, 0x1404e5250, 0x1404e2830, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143172f36, 0x142646960, 0x143caa460, 0x143caa470, 0x143caa478, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_toggle::system>
    Replacement::draft_throws(0x1404dd810, "bigfish_1404dd810", fn_1404dd810 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_light_toggled::system>
    Replacement::draft_throws(0x1404dda60, "bigfish_1404dda60", tpl_4682::<0x143ca9980, 0x143ca9d90, 0x1404e0580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143173876, 0x142646960, 0x143ca9db0, 0x143ca9dd0, 0x143ca9dd4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_owner_hide_update::system>
    Replacement::draft_throws(0x1404ddc70, "bigfish_1404ddc70", fn_1404ddc70 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_owner_state_update::system>
    Replacement::draft_throws(0x1404dde90, "bigfish_1404dde90", tpl_4326::<0x143ca9ee0, 0x143caa7a8, 0x1411e9f50, 0x1404e6a10, 0x143caaa18, 0x1420af120, 0x143c8ce88, 0x140215610, 0x1420a28e0, 0x1404e5bc0, 0x1404e3170, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143171066, 0x142646960, 0x143caa7c8, 0x143caa7d8, 0x143caa7e0, 0x143caa7e2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::animations_control::system>
    Replacement::draft(0x1404de1c0, "bigfish_1404de1c0", tpl_60984::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1404e09c0, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1404de1c0
    Replacement::draft(0x1404de3b0, "bigfish_1404de3b0", tpl_1094::<0x143c9d1b4, 0x1404e0ab0, 0x14020e8f0> as *const ()), // sub_1404de3b0
    Replacement::draft(0x1404de5b0, "bigfish_1404de5b0", tpl_4899::<0x143c98134, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404de5b0
    Replacement::draft(0x1404de900, "bigfish_1404de900", tpl_1403::<0x143caa600, 0x1404e0c60, 0x14020e8f0> as *const ()), // sub_1404de900
    Replacement::draft(0x1404deae0, "bigfish_1404deae0", tpl_1097::<0x143c982ac, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404deae0
    Replacement::draft(0x1404dee10, "bigfish_1404dee10", tpl_1094::<0x143caa62c, 0x1404e0de0, 0x14020e8f0> as *const ()), // sub_1404dee10
    Replacement::draft(0x1404df010, "bigfish_1404df010", tpl_1094::<0x143caa3b8, 0x1404e0fd0, 0x14020e8f0> as *const ()), // sub_1404df010
    Replacement::draft(0x1404df210, "bigfish_1404df210", fn_1404df210 as *const ()), // sub_1404df210
    Replacement::draft(0x1404df570, "bigfish_1404df570", fn_1404df570 as *const ()), // sub_1404df570
    Replacement::draft(0x1404df600, "bigfish_1404df600", tpl_1095::<0x143ca9dd8, 0x14020e8f0> as *const ()), // sub_1404df600
    Replacement::draft(0x1404df7e0, "bigfish_1404df7e0", tpl_1095::<0x143caa94c, 0x14020e8f0> as *const ()), // sub_1404df7e0
    Replacement::draft(0x1404df9c0, "bigfish_1404df9c0", tpl_1097::<0x143ca9de8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404df9c0
    Replacement::draft(0x1404dfcf0, "bigfish_1404dfcf0", tpl_4902::<0x8, 0x1404de3b0, 0x1a, 0x1404de5b0> as *const ()), // sub_1404dfcf0
    Replacement::draft(0x1404dfdf0, "bigfish_1404dfdf0", tpl_4903::<0x8, 0x1404df010, 0x1404df210> as *const ()), // sub_1404dfdf0
    Replacement::draft_throws(0x1404dfe90, "bigfish_1404dfe90", fn_1404dfe90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_light_cleanup::system>
    Replacement::draft_throws(0x1404e00a0, "bigfish_1404e00a0", tpl_4905::<0x143caa950, 0x143ca9c18, 0x1404e1340, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143171426, 0x142646960, 0x143ca9c38, 0x143ca9c48, 0x143ca9c50, 0x143ca9c52, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_controller::flashlight_cleanup::system>
    Replacement::draft_throws(0x1404e02c0, "unknown_1404e02c0", fn_1404e02c0 as *const (), Throws::Discard), // sub_1404e02c0
    Replacement::draft_throws(0x1404e0580, "unknown_1404e0580", tpl_1187::<0x1411e9f50, 0x1404e6e10, 0x143ca9ddc, 0x1420af120, 0x1404e6d00, 0x143caa638, 0x140215750, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x1404e5740, 0x1404e2dd0> as *const (), Throws::Discard), // sub_1404e0580
    Replacement::draft_throws(0x1404e07a0, "unknown_1404e07a0", tpl_1187::<0x1411e9f50, 0x1404e6c10, 0x143caa640, 0x1420af120, 0x1404e6b20, 0x143caaa50, 0x140215750, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x1404e5990, 0x1404e3010> as *const (), Throws::Discard), // sub_1404e07a0
    Replacement::draft(0x1404e09c0, "unknown_1404e09c0", fn_1404e09c0 as *const ()), // sub_1404e09c0
    Replacement::draft(0x1404e0ab0, "unknown_1404e0ab0", fn_1404e0ab0 as *const ()), // sub_1404e0ab0
    Replacement::draft(0x1404e0c60, "unknown_1404e0c60", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143174260> as *const ()), // sub_1404e0c60
    Replacement::draft(0x1404e0de0, "unknown_1404e0de0", fn_1404e0de0 as *const ()), // sub_1404e0de0
    Replacement::draft(0x1404e0fd0, "unknown_1404e0fd0", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143174108, 0x1404db690> as *const ()), // sub_1404e0fd0
    Replacement::draft_throws(0x1404e1170, "unknown_1404e1170", tpl_3370::<0x1411e9f50, 0x1404e8090, 0x143caa94c, 0x1401fc320, 0x1404e8d40, 0x143caa4b8, 0x1420af120, 0x140215750, 0x1420a28e0, 0x1404e65b0, 0x1401de410> as *const (), Throws::Discard), // sub_1404e1170
    Replacement::draft_throws(0x1404e1340, "unknown_1404e1340", tpl_3370::<0x1411e9f50, 0x1404e8010, 0x143c8ec6c, 0x1401fc320, 0x1404cef30, 0x143ca8a30, 0x1420af120, 0x140215750, 0x1420a28e0, 0x1404e6730, 0x1401de410> as *const (), Throws::Discard), // sub_1404e1340
    Replacement::draft_throws(0x1404e1510, "unknown_1404e1510", fn_1404e1510 as *const (), Throws::Caught), // sub_1404e1510
    Replacement::draft_throws(0x1404e1650, "unknown_1404e1650", fn_1404e1650 as *const (), Throws::Caught), // sub_1404e1650
    Replacement::draft_throws(0x1404e1760, "unknown_1404e1760", tpl_4853::<0x1404e7c70, 0x143ca9de4, 0x1420af120, 0x1404e7b70, 0x143caa5c0, 0x140215750, 0x1404e7a80, 0x143caa09c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404e1760
    Replacement::draft(0x1404e1950, "unknown_1404e1950", fn_1404e1950 as *const ()), // sub_1404e1950
    Replacement::draft(0x1404e1a20, "unknown_1404e1a20", fn_1404e1a20 as *const ()), // sub_1404e1a20
    Replacement::draft(0x1404e1ae0, "unknown_1404e1ae0", fn_1404e1ae0 as *const ()), // sub_1404e1ae0
    Replacement::draft(0x1404e1c10, "unknown_1404e1c10", fn_1404e1c10 as *const ()), // sub_1404e1c10
    Replacement::draft(0x1404e1d10, "unknown_1404e1d10", fn_1404e1d10 as *const ()), // sub_1404e1d10
    Replacement::draft_throws(0x1404e1e00, "unknown_1404e1e00", fn_1404e1e00 as *const (), Throws::Discard), // sub_1404e1e00
    Replacement::draft(0x1404e2000, "unknown_1404e2000", fn_1404e2000 as *const ()), // sub_1404e2000
    Replacement::draft(0x1404e2180, "unknown_1404e2180", fn_1404e2180 as *const ()), // sub_1404e2180
    Replacement::draft_throws(0x1404e2290, "unknown_1404e2290", fn_1404e2290 as *const (), Throws::Discard), // sub_1404e2290
    Replacement::draft(0x1404e2420, "unknown_1404e2420", fn_1404e2420 as *const ()), // sub_1404e2420
    Replacement::draft(0x1404e2550, "unknown_1404e2550", fn_1404e2550 as *const ()), // sub_1404e2550
    Replacement::draft_throws(0x1404e2640, "unknown_1404e2640", tpl_3110::<0x1404e7440, 0x143ca9e5c, 0x1420af120, 0x1404e7300, 0x143ca9f58, 0x140215750, 0x143c8ceec, 0x140215610, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404e2640
    Replacement::draft(0x1404e2830, "unknown_1404e2830", tpl_64107::<0x143ca9f58, 0x143c8ceec, 0x143c8d08c, 0x1404e38d0, 0x141a5cf70> as *const ()), // sub_1404e2830
    Replacement::draft_throws(0x1404e29b0, "unknown_1404e29b0", tpl_4853::<0x1404e71c0, 0x143ca9e9c, 0x1420af120, 0x1404e70b0, 0x143caa08c, 0x140215750, 0x1404e6fc0, 0x143caa0a8, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404e29b0
    Replacement::draft(0x1404e2ba0, "unknown_1404e2ba0", fn_1404e2ba0 as *const ()), // sub_1404e2ba0
    Replacement::draft(0x1404e2cd0, "unknown_1404e2cd0", fn_1404e2cd0 as *const ()), // sub_1404e2cd0
    Replacement::draft(0x1404e2dd0, "unknown_1404e2dd0", tpl_3141::<0x88, 0x10088, 0x143caa638, 0x20, 0x8, 0x143c8d08c, 0x580e0, 0x1404e3540, 0x1404e2ee0> as *const ()), // sub_1404e2dd0
    Replacement::draft(0x1404e2ee0, "unknown_1404e2ee0", tpl_3142::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141a5bf70> as *const ()), // sub_1404e2ee0
    Replacement::draft(0x1404e3010, "unknown_1404e3010", tpl_62526::<0x143caaa50, 0x143c8d910, 0x1404e3440, 0x141a5c300> as *const ()), // sub_1404e3010
    Replacement::draft_throws(0x1404e3170, "unknown_1404e3170", tpl_64110::<0x88, 0x10088, 0x143c8ce88, 0x580e0, 0x1404e3310, 0x141a5bb00> as *const (), Throws::Discard), // sub_1404e3170
    Replacement::draft(0x1404e32c0, "content_1404e32c0", fn_1404e32c0 as *const ()), // content::FlashlightLightComponent::content::FlashlightLightComponent::vfunc_0
    Replacement::draft(0x1404e3540, "unknown_1404e3540", tpl_4925::<0x8, 0x10, 0x18, 0x1404e35f0, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50> as *const ()), // sub_1404e3540
    Replacement::draft(0x1404e35f0, "unknown_1404e35f0", tpl_2541::<0x143c8d694, 0x142489ee0, 0x143c8d87c, 0x143c8ec6c, 0x143caa3b8, 0x143c9a320, 0x143c9a330, 0x143c8e840> as *const ()), // sub_1404e35f0
    Replacement::draft_throws(0x1404e3c30, "unknown_1404e3c30", tpl_1330::<0x8, 0x1404e7680, 0x143caa0fc, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1404e3c30
    Replacement::draft(0x1404e4250, "unknown_1404e4250", tpl_1124::<0x88> as *const ()), // sub_1404e4250
    Replacement::draft(0x1404e4290, "unknown_1404e4290", fn_1404e4290 as *const ()), // sub_1404e4290
    Replacement::draft(0x1404e42e0, "unknown_1404e42e0", fn_1404e42e0 as *const ()), // sub_1404e42e0
    Replacement::draft(0x1404e4330, "unknown_1404e4330", tpl_1212::<0x8, 0x1404e8ac0> as *const ()), // sub_1404e4330
    Replacement::draft(0x1404e65b0, "unknown_1404e65b0", fn_1404e65b0 as *const ()), // sub_1404e65b0
    Replacement::draft(0x1404e6730, "unknown_1404e6730", tpl_4345::<0x143c8ec6c, 0x143ca8a30, 0x142407c80, 0x141a5c5a0, 0x140489dd0> as *const ()), // sub_1404e6730
    Replacement::draft_throws(0x1404e68a0, "unknown_1404e68a0", fn_1404e68a0 as *const (), Throws::Discard), // sub_1404e68a0
    Replacement::draft(0x1404e6e10, "unknown_1404e6e10", fn_1404e6e10 as *const ()), // sub_1404e6e10
    Replacement::draft(0x1404e7c70, "unknown_1404e7c70", fn_1404e7c70 as *const ()), // sub_1404e7c70
    Replacement::draft(0x1404e8010, "unknown_1404e8010", tpl_3188::<0x1404e8ca0, 0x1431743d8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1404e8010
    Replacement::draft(0x1404e8090, "unknown_1404e8090", tpl_1991::<0x1404e8df0, 0x1431740b0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1404e8090
    Replacement::draft(0x1404e8100, "unknown_1404e8100", tpl_4348::<0x20, 0x28, 0x140489dd0> as *const ()), // sub_1404e8100
    Replacement::draft(0x1404e8310, "unknown_1404e8310", fn_1404e8310 as *const ()), // sub_1404e8310
    Replacement::draft(0x1404e8440, "unknown_1404e8440", tpl_1098::<0x143caa270, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404e8440
    Replacement::draft(0x1404e8780, "unknown_1404e8780", tpl_4324::<0x143c9a074, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404e8780
    Replacement::draft(0x1404e8ac0, "unknown_1404e8ac0", tpl_1095::<0x143ca9de0, 0x14020e8f0> as *const ()), // sub_1404e8ac0
    Replacement::draft(0x1404e8ca0, "unknown_1404e8ca0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431743d8, 0x8, 0x20> as *const ()), // sub_1404e8ca0
    Replacement::draft(0x1404e8df0, "unknown_1404e8df0", tpl_1997::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431740b0, 0x8> as *const ()), // sub_1404e8df0
    Replacement::draft(0x1404e9940, "unknown_1404e9940", fn_1404e9940 as *const ()), // sub_1404e9940
    Replacement::draft(0x1404e99f0, "unknown_1404e99f0", fn_1404e99f0 as *const ()), // sub_1404e99f0
    Replacement::draft(0x1404e9ab0, "unknown_1404e9ab0", tpl_855::<0x14021f370, 0x1425371a0, 0x142533200, 0x1404de1c0, 0x1404e09c0> as *const ()), // sub_1404e9ab0
    Replacement::draft_throws(0x1404e9c10, "bigfish_1404e9c10", tpl_1074::<0x143caab18, 0x143caaba8, 0x1411e9f50, 0x1404ebb80, 0x143caac1c, 0x1420af120, 0x1420a28e0, 0x1404eb530, 0x1404eade0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143174a26, 0x142646960, 0x143caabc8, 0x143caabd8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlightdamage::flashlightDamageSystem>
    Replacement::draft_throws(0x1404e9ef0, "bigfish_1404e9ef0", tpl_1463::<0x143caaba0, 0x143caab60, 0x1411e9f50, 0x1404eba00, 0x143caab1c, 0x1420af120, 0x1404eb950, 0x143caac58, 0x140215750, 0x1420a28e0, 0x1404eb740, 0x1404eaf00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143174c66, 0x142646960, 0x143caab80, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlightdamage::init::system>
    Replacement::draft(0x1404ea240, "bigfish_1404ea240", tpl_1094::<0x143ca88bc, 0x1404ea870, 0x14020e8f0> as *const ()), // sub_1404ea240
    Replacement::draft(0x1404ea440, "bigfish_1404ea440", tpl_1094::<0x143ca87e8, 0x1404eaaa0, 0x14020e8f0> as *const ()), // sub_1404ea440
    Replacement::draft(0x1404ea640, "bigfish_1404ea640", tpl_1094::<0x143ca5918, 0x1404eac40, 0x14020e8f0> as *const ()), // sub_1404ea640
    Replacement::draft(0x1404ea870, "bigfish_1404ea870", fn_1404ea870 as *const ()), // sub_1404ea870
    Replacement::draft(0x1404eaaa0, "bigfish_1404eaaa0", fn_1404eaaa0 as *const ()), // sub_1404eaaa0
    Replacement::draft(0x1404eac40, "bigfish_1404eac40", fn_1404eac40 as *const ()), // sub_1404eac40
    Replacement::draft(0x1404eade0, "bigfish_1404eade0", tpl_62730::<0x88, 0x10088, 0x1404eb340, 0x141a5f810> as *const ()), // sub_1404eade0
    Replacement::draft(0x1404eaf00, "bigfish_1404eaf00", fn_1404eaf00 as *const ()), // sub_1404eaf00
    Replacement::draft(0x1404eb020, "bigfish_1404eb020", tpl_4943::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141a5f6a0> as *const ()), // sub_1404eb020
    Replacement::draft(0x1404eb470, "bigfish_1404eb470", tpl_1124::<0xa0> as *const ()), // sub_1404eb470
    Replacement::draft(0x1404eb4b0, "bigfish_1404eb4b0", fn_1404eb4b0 as *const ()), // sub_1404eb4b0
    Replacement::draft(0x1404eb4f0, "bigfish_1404eb4f0", fn_1404eb4f0 as *const ()), // sub_1404eb4f0
    Replacement::draft(0x1404eb530, "bigfish_1404eb530", tpl_62741::<0x143ca88bc, 0x142489ee0, 0x143c98134, 0x143ca87e8, 0x141a5f810> as *const ()), // sub_1404eb530
    Replacement::draft(0x1404eb740, "bigfish_1404eb740", fn_1404eb740 as *const ()), // sub_1404eb740
    Replacement::draft(0x1404ebe70, "bigfish_1404ebe70", tpl_1842::<0x1401e7c10, 0x143ca9708, 0x142489ee0> as *const ()), // sub_1404ebe70
    Replacement::draft(0x1404ebfc0, "bigfish_1404ebfc0", tpl_63073::<0x143ca9708, 0x142489ee0, 0x143ca8f88, 0x143c9dc58> as *const ()), // sub_1404ebfc0
    Replacement::draft(0x1404ec280, "bigfish_1404ec280", tpl_1368::<0x143c9efb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404ec280
    Replacement::draft_throws(0x1404ec5b0, "bigfish_1404ec5b0", tpl_1175::<0x143caad58, 0x143caae20, 0x1411e9f50, 0x1404ec810, 0x1404ecd80, 0x1404eca70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431755b6, 0x142646960, 0x143caae40, 0x143caae60, 0x143caae68, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_balancing_data::flashlight_resource_stream_in::system>
    Replacement::draft_throws(0x1404ec810, "unknown_1404ec810", fn_1404ec810 as *const (), Throws::Discard), // sub_1404ec810
    Replacement::draft(0x1404eca70, "unknown_1404eca70", fn_1404eca70 as *const ()), // sub_1404eca70
    Replacement::draft(0x1404ecb70, "unknown_1404ecb70", fn_1404ecb70 as *const ()), // sub_1404ecb70
    Replacement::draft(0x1404ecd80, "unknown_1404ecd80", fn_1404ecd80 as *const ()), // sub_1404ecd80
    Replacement::draft(0x1404ed050, "unknown_1404ed050", tpl_4170::<0x1404ed670, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_1404ed050
    Replacement::draft(0x1404ed3f0, "unknown_1404ed3f0", fn_1404ed3f0 as *const ()), // sub_1404ed3f0
    Replacement::draft(0x1404ed580, "unknown_1404ed580", fn_1404ed580 as *const ()), // sub_1404ed580
    Replacement::draft(0x1404ed670, "unknown_1404ed670", fn_1404ed670 as *const ()), // sub_1404ed670
    Replacement::draft_throws(0x1404ed9a0, "bigfish_1404ed9a0", tpl_3062::<0x143caaf9c, 0x143caafb8, 0x1411e9f50, 0x1404ee840, 0x1404eeeb0, 0x1404eea20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431766f6, 0x142646960, 0x143caafd8, 0x143caafe0, 0x143caafe4, 0x143caafe6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::fadeout_vision_sensor::update::system>
    Replacement::draft(0x1404edc00, "bigfish_1404edc00", tpl_1095::<0x143caafec, 0x14020e8f0> as *const ()), // sub_1404edc00
    Replacement::draft(0x1404edde0, "bigfish_1404edde0", tpl_1436::<0x58, 0x60, 0x158, 0x143caafec, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1404edde0
    Replacement::draft(0x1404edec0, "bigfish_1404edec0", tpl_1094::<0x143caaf98, 0x1404ee6c0, 0x14020e8f0> as *const ()), // sub_1404edec0
    Replacement::draft_throws(0x1404ee0c0, "bigfish_1404ee0c0", tpl_1074::<0x143caaf50, 0x143caae78, 0x1411e9f50, 0x1404ef950, 0x143caaf90, 0x1420af120, 0x1420a28e0, 0x1404ef380, 0x1404eed50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143176eb6, 0x142646960, 0x143caae98, 0x143caaea8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::fadeout_vision_sensor::stream_in::system>
    Replacement::draft_throws(0x1404ee3a0, "bigfish_1404ee3a0", tpl_3211::<0x143caafa8, 0x143cab128, 0x1411e9f50, 0x1404ef860, 0x143caafb4, 0x1420af120, 0x1420a28e0, 0x1404ef490, 0x1404eee10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143176d16, 0x142646960, 0x143cab148, 0x143cab158, 0x143cab15a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::fadeout_vision_sensor::stream_out::system>
    Replacement::draft(0x1404ee6c0, "unknown_1404ee6c0", tpl_1888::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143175b40> as *const ()), // sub_1404ee6c0
    Replacement::draft_throws(0x1404ee840, "unknown_1404ee840", tpl_1196::<0x1404ef720, 0x143caafe8, 0x1420af120, 0x1404ef570, 0x143cab08c, 0x140215750, 0x143c8f63c, 0x140215610, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404ee840
    Replacement::draft_throws(0x1404eea20, "unknown_1404eea20", fn_1404eea20 as *const (), Throws::Discard), // sub_1404eea20
    Replacement::draft_throws(0x1404eeb40, "unknown_1404eeb40", fn_1404eeb40 as *const (), Throws::Discard), // sub_1404eeb40
    Replacement::draft(0x1404eed50, "unknown_1404eed50", tpl_1126::<0x142407c80, 0x580f0, 0x141a61c00> as *const ()), // sub_1404eed50
    Replacement::draft(0x1404eee10, "unknown_1404eee10", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1404edde0> as *const ()), // sub_1404eee10
    Replacement::draft_throws(0x1404eeeb0, "unknown_1404eeeb0", fn_1404eeeb0 as *const (), Throws::Discard), // sub_1404eeeb0
    Replacement::draft(0x1404ef380, "unknown_1404ef380", tpl_1130::<0x142407c80, 0x141a61c00> as *const ()), // sub_1404ef380
    Replacement::draft(0x1404ef490, "unknown_1404ef490", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1404edde0> as *const ()), // sub_1404ef490
    Replacement::draft(0x1404ef570, "unknown_1404ef570", fn_1404ef570 as *const ()), // sub_1404ef570
    Replacement::draft(0x1404efc90, "unknown_1404efc90", tpl_1095::<0x143cab35c, 0x14020e8f0> as *const ()), // sub_1404efc90
    Replacement::draft(0x1404efe70, "unknown_1404efe70", tpl_1436::<0x58, 0x60, 0x158, 0x143cab35c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1404efe70
    Replacement::draft(0x1404eff50, "unknown_1404eff50", tpl_1094::<0x143c91600, 0x1404f0730, 0x14020e8f0> as *const ()), // sub_1404eff50
    Replacement::draft_throws(0x1404f0180, "bigfish_1404f0180", tpl_4485::<0x143cab318, 0x143cab1e8, 0x1411e9f50, 0x1404f0fb0, 0x143cab358, 0x1420af120, 0x1420a28e0, 0x1404f0c30, 0x1404f08b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143177246, 0x142646960, 0x143cab208, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::fadeout::stream_in::system>
    Replacement::draft_throws(0x1404f0450, "bigfish_1404f0450", tpl_4955::<0x143cab238, 0x143cab210, 0x1411e9f50, 0x1404f0ec0, 0x143cab23c, 0x1420af120, 0x1420a28e0, 0x1404f0de0, 0x1404f09a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431770b6, 0x142646960, 0x143cab230, 0x143cab234, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::fadeout::stream_out::system>
    Replacement::draft(0x1404f0730, "unknown_1404f0730", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1430a7230> as *const ()), // sub_1404f0730
    Replacement::draft(0x1404f08b0, "unknown_1404f08b0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x1404f0a40, 0x580f0, 0x141a62ec0> as *const ()), // sub_1404f08b0
    Replacement::draft(0x1404f09a0, "unknown_1404f09a0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1404efe70> as *const ()), // sub_1404f09a0
    Replacement::draft(0x1404f0c30, "unknown_1404f0c30", tpl_3213::<0x142407c80, 0x143c91600, 0x142489ee0, 0x143c8f6e4, 0x141a62ec0> as *const ()), // sub_1404f0c30
    Replacement::draft(0x1404f0de0, "unknown_1404f0de0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1404efe70> as *const ()), // sub_1404f0de0
    Replacement::draft(0x1404f1250, "content_1404f1250", tpl_1094::<0x143cab3a4, 0x1404f1480, 0x14020e8f0> as *const ()), // sub_1404f1250
    Replacement::draft(0x1404f1480, "content_1404f1480", fn_1404f1480 as *const ()), // sub_1404f1480
    Replacement::draft(0x1404f16b0, "content_1404f16b0", fn_1404f16b0 as *const ()), // content::FactionUtilitySingletonComponent::content::FactionUtilitySingletonComponent::vfunc_0
    Replacement::draft(0x1404f1840, "unknown_1404f1840", fn_1404f1840 as *const ()), // sub_1404f1840
    Replacement::draft(0x1404f18d0, "unknown_1404f18d0", fn_1404f18d0 as *const ()), // sub_1404f18d0
    Replacement::draft(0x1404f19c0, "content_1404f19c0", fn_1404f19c0 as *const ()), // content::QuadrupedBodyObstacleComponent::content::QuadrupedBodyObstacleComponent::vfunc_20
    Replacement::draft(0x1404f1a10, "unknown_1404f1a10", tpl_4844::<0x1401e7c10, 0x1404f1b70> as *const ()), // sub_1404f1a10
    Replacement::draft(0x1404f1b70, "unknown_1404f1b70", tpl_63073::<0x143c8ec68, 0x142489ee0, 0x143c96b14, 0x143c96b20> as *const ()), // sub_1404f1b70
    Replacement::draft(0x1404f1ce0, "unknown_1404f1ce0", fn_1404f1ce0 as *const ()), // sub_1404f1ce0
    Replacement::draft(0x1404f1da0, "unknown_1404f1da0", tpl_4961::<0x143c8d87c, 0x142489ee0, 0x143c8d120, 0x143c8d160, 0x143c8dbd4, 0x143c8fa04, 0x143c8ec20, 0x143cab498, 0x143c8f4f0> as *const ()), // sub_1404f1da0
    Replacement::draft(0x1404f1f70, "unknown_1404f1f70", fn_1404f1f70 as *const ()), // sub_1404f1f70
    Replacement::draft(0x1404f20a0, "unknown_1404f20a0", fn_1404f20a0 as *const ()), // sub_1404f20a0
    Replacement::draft_throws(0x1404f2260, "bigfish_1404f2260", tpl_3060::<0x143cab44c, 0x143cab9a0, 0x1411e9f50, 0x1404f35a0, 0x1404f68f0, 0x1404f3720, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317c6e6, 0x142646960, 0x143cab9c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::explosion_on_damage::system>
    Replacement::draft_throws(0x1404f24a0, "bigfish_1404f24a0", tpl_3464::<0x143cab998, 0x143cab418, 0x1411e9f50, 0x1404f38e0, 0x1404f6b40, 0x1404f3a60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317d356, 0x142646960, 0x143cab438, 0x143cab448, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::explosion_timers_update::system>
    Replacement::draft_throws(0x1404f26f0, "bigfish_1404f26f0", tpl_4111::<0x143cab82c, 0x143cab628, 0x1411e9f50, 0x1404f3c40, 0x1404f6da0, 0x1404f3f30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317cac6, 0x142646960, 0x143cab648, 0x143cab658, 0x143cab65c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::explosion_activate_update::system>
    Replacement::draft_throws(0x1404f2940, "bigfish_1404f2940", tpl_1291::<0x143cabac4, 0x143cab830, 0x1411e9f50, 0x1404f4350, 0x1404f7050, 0x1404f4580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317aa66, 0x142646960, 0x143cab850, 0x143cab858, 0x143cab85c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::explosion_targets::system>
    Replacement::draft_throws(0x1404f2b90, "bigfish_1404f2b90", tpl_3464::<0x143cabbec, 0x143cab570, 0x1411e9f50, 0x1404f4820, 0x1404f7300, 0x1404f4ad0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143179d66, 0x142646960, 0x143cab590, 0x143cab5a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::explosion_damage_update::system>
    Replacement::draft_throws(0x1404f2de0, "bigfish_1404f2de0", fn_1404f2de0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::add_explosion_noise_event::system>
    Replacement::draft_throws(0x1404f3100, "bigfish_1404f3100", tpl_3464::<0x143cabbf4, 0x143caba90, 0x1411e9f50, 0x1404f5070, 0x1404f7800, 0x1404f5250, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317b426, 0x142646960, 0x143cabab0, 0x143cabac0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::explosion::explosion_state_cleanup::system>
    Replacement::draft(0x1404f3350, "unknown_1404f3350", fn_1404f3350 as *const ()), // sub_1404f3350
    Replacement::draft_throws(0x1404f35a0, "unknown_1404f35a0", fn_1404f35a0 as *const (), Throws::Discard), // sub_1404f35a0
    Replacement::draft(0x1404f3720, "unknown_1404f3720", fn_1404f3720 as *const ()), // sub_1404f3720
    Replacement::draft(0x1404f37f0, "unknown_1404f37f0", fn_1404f37f0 as *const ()), // sub_1404f37f0
    Replacement::draft_throws(0x1404f38e0, "unknown_1404f38e0", fn_1404f38e0 as *const (), Throws::Discard), // sub_1404f38e0
    Replacement::draft(0x1404f3a60, "unknown_1404f3a60", fn_1404f3a60 as *const ()), // sub_1404f3a60
    Replacement::draft(0x1404f3b30, "unknown_1404f3b30", fn_1404f3b30 as *const ()), // sub_1404f3b30
    Replacement::draft_throws(0x1404f3c40, "unknown_1404f3c40", tpl_4847::<0x1404f8b10, 0x143cab860, 0x1420af120, 0x1404f8a30, 0x143cab494, 0x140215750, 0x1404f8950, 0x143cab4a0, 0x143c8ceec, 0x140215610, 0x143c8f4d8, 0x143c8d910, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1404f3c40
    Replacement::draft_throws(0x1404f3f30, "unknown_1404f3f30", fn_1404f3f30 as *const (), Throws::Discard), // sub_1404f3f30
    Replacement::draft(0x1404f40c0, "unknown_1404f40c0", tpl_1444::<0x8, 0x20, 0x143cab494, 0x143c8ceec, 0x143cab4a0, 0x143c8f63c, 0x580e0, 0x143c8f4d8, 0x143c8d910> as *const ()), // sub_1404f40c0
    Replacement::draft_throws(0x1404f41a0, "unknown_1404f41a0", fn_1404f41a0 as *const (), Throws::Discard), // sub_1404f41a0
    Replacement::draft_throws(0x1404f4350, "unknown_1404f4350", fn_1404f4350 as *const (), Throws::Discard), // sub_1404f4350
    Replacement::draft_throws(0x1404f4580, "unknown_1404f4580", fn_1404f4580 as *const (), Throws::Discard), // sub_1404f4580
    Replacement::draft_throws(0x1404f46e0, "unknown_1404f46e0", fn_1404f46e0 as *const (), Throws::Discard), // sub_1404f46e0
    Replacement::draft_throws(0x1404f4820, "unknown_1404f4820", fn_1404f4820 as *const (), Throws::Discard), // sub_1404f4820
    Replacement::draft(0x1404f4ad0, "unknown_1404f4ad0", tpl_4977::<0x88, 0x10088, 0x1404f4bb0, 0x1404f5a70, 0x1404f4ce0> as *const ()), // sub_1404f4ad0
    Replacement::draft(0x1404f4bb0, "unknown_1404f4bb0", tpl_3023::<0x8, 0x580e0, 0x20, 0x143cab5e0, 0x143c94b34, 0x143ca0308, 0x143cab958, 0x143c8ceec, 0x143c8d910, 0x143c8ef20, 0x143c8f4d8> as *const ()), // sub_1404f4bb0
    Replacement::draft(0x1404f4ce0, "unknown_1404f4ce0", tpl_4978::<0x141a64ec0> as *const ()), // sub_1404f4ce0
    Replacement::draft(0x1404f4e80, "unknown_1404f4e80", tpl_1915::<0x143c8f048, 0x1404f5920, 0x1404f4f80> as *const ()), // sub_1404f4e80
    Replacement::draft(0x1404f4f80, "unknown_1404f4f80", tpl_1916::<0x18, 0x20, 0x40, 0x8, 0x141a647a0> as *const ()), // sub_1404f4f80
    Replacement::draft_throws(0x1404f5070, "unknown_1404f5070", fn_1404f5070 as *const (), Throws::Discard), // sub_1404f5070
    Replacement::draft(0x1404f5250, "unknown_1404f5250", fn_1404f5250 as *const ()), // sub_1404f5250
    Replacement::draft(0x1404f53d0, "unknown_1404f53d0", tpl_1444::<0x8, 0x20, 0x143cab958, 0x143c8ceec, 0x143c912bc, 0x143c8ef20, 0x580e0, 0x143c8d08c, 0x143c8f4d8> as *const ()), // sub_1404f53d0
    Replacement::draft(0x1404f54b0, "unknown_1404f54b0", fn_1404f54b0 as *const ()), // sub_1404f54b0
    Replacement::draft_throws(0x1404f5f90, "unknown_1404f5f90", tpl_1330::<0x8, 0x1404f8420, 0x143cab958, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1404f5f90
    Replacement::draft(0x1404f6070, "unknown_1404f6070", tpl_3150::<0x18, 0x10, 0x8, 0x1404f6150, 0x1402716c0, 0x20, 0x40, 0x60, 0x80> as *const ()), // sub_1404f6070
    Replacement::draft(0x1404f68f0, "unknown_1404f68f0", fn_1404f68f0 as *const ()), // sub_1404f68f0
    Replacement::draft(0x1404f6b40, "unknown_1404f6b40", fn_1404f6b40 as *const ()), // sub_1404f6b40
    Replacement::draft_throws(0x1404f6da0, "unknown_1404f6da0", fn_1404f6da0 as *const (), Throws::Discard), // sub_1404f6da0
    Replacement::draft_throws(0x1404f7050, "unknown_1404f7050", fn_1404f7050 as *const (), Throws::Discard), // sub_1404f7050
    Replacement::draft(0x1404f7300, "unknown_1404f7300", tpl_64176::<0x1404f4bb0, 0x1404f5b70, 0x141a64ec0> as *const ()), // sub_1404f7300
    Replacement::draft(0x1404f75f0, "unknown_1404f75f0", tpl_61556::<0x143c8f048, 0x1404f5920, 0x141a647a0> as *const ()), // sub_1404f75f0
    Replacement::draft(0x1404f7800, "unknown_1404f7800", fn_1404f7800 as *const ()), // sub_1404f7800
    Replacement::draft(0x1404f7bb0, "unknown_1404f7bb0", fn_1404f7bb0 as *const ()), // sub_1404f7bb0
    Replacement::draft(0x1404f7f70, "unknown_1404f7f70", fn_1404f7f70 as *const ()), // sub_1404f7f70
    Replacement::draft(0x1404f81e0, "unknown_1404f81e0", fn_1404f81e0 as *const ()), // sub_1404f81e0
    Replacement::draft(0x1404f8560, "unknown_1404f8560", fn_1404f8560 as *const ()), // sub_1404f8560
    Replacement::draft(0x1404f87a0, "unknown_1404f87a0", fn_1404f87a0 as *const ()), // sub_1404f87a0
    Replacement::draft(0x1404f8b10, "unknown_1404f8b10", fn_1404f8b10 as *const ()), // sub_1404f8b10
    Replacement::draft(0x1404f8db0, "unknown_1404f8db0", tpl_3177::<0x143cabbf0, 0x143cab3b0, 0x141834a78, 0x143c8e654, 0x143c95748, 0x143c964e0, 0x143c8f4cc, 0x143c8d160, 0x143c8d120, 0x143c8fe18, 0x143c956a0, 0x143cab3d0, 0x143cab3f0, 0x143cab410, 0x141834a0c> as *const ()), // sub_1404f8db0
    Replacement::draft(0x1404f8f70, "unknown_1404f8f70", tpl_4822::<0x143cab4ac, 0x143cab670, 0x141834a78, 0x143c8d694, 0x143c8e654, 0x143c8e70c, 0x143c95844, 0x143c8d160, 0x143c8fe18, 0x143c8d708, 0x143cab690, 0x143cab6b0, 0x143cab6c0, 0x141834a0c> as *const ()), // sub_1404f8f70
    Replacement::draft(0x1404f97d0, "unknown_1404f97d0", fn_1404f97d0 as *const ()), // sub_1404f97d0
    Replacement::draft_throws(0x1404f9860, "bigfish_1404f9860", tpl_4989::<0x143cabfa8, 0x143cac140, 0x1411e9f50, 0x1404fdc40, 0x143cabe5c, 0x1420af120, 0x140239e00, 0x1420a28e0, 0x1404fcc90, 0x1404fc010, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317fa16, 0x142646960, 0x143cac160, 0x143cac170, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::equipment::systems::update_removal::system>
    Replacement::draft_throws(0x1404f9b60, "bigfish_1404f9b60", tpl_4895::<0x143cac020, 0x143cac068, 0x1411e9f50, 0x1404fdb90, 0x143cabd88, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x1404fcea0, 0x1404fc150, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317f536, 0x142646960, 0x143cac088, 0x143cac098, 0x143cac0a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::equipment::systems::deferred_lua_events::system>
    Replacement::draft_throws(0x1404f9e80, "bigfish_1404f9e80", fn_1404f9e80 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::equipment::systems::ui::sync>
    Replacement::draft(0x1404fa1b0, "bigfish_1404fa1b0", tpl_64186::<0x1426467e8, 0x140228ce0, 0x1404fa227> as *const ()), // sub_1404fa1b0
    Replacement::draft(0x1404fa330, "bigfish_1404fa330", tpl_1094::<0x143c8f4cc, 0x1404fbad0, 0x14020e8f0> as *const ()), // sub_1404fa330
    Replacement::draft(0x1404fa530, "bigfish_1404fa530", tpl_1403::<0x143c91070, 0x1404fbc50, 0x14020e8f0> as *const ()), // sub_1404fa530
    Replacement::draft(0x1404fa710, "bigfish_1404fa710", fn_1404fa710 as *const ()), // sub_1404fa710
    Replacement::draft(0x1404fa920, "bigfish_1404fa920", tpl_3270::<0x143c91064, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1404fa920
    Replacement::draft(0x1404fac60, "bigfish_1404fac60", tpl_1095::<0x143ca0e70, 0x14020e8f0> as *const ()), // sub_1404fac60
    Replacement::draft(0x1404fae40, "bigfish_1404fae40", tpl_1094::<0x143c8dbcc, 0x1404fbe60, 0x14020e8f0> as *const ()), // sub_1404fae40
    Replacement::draft(0x1404fb040, "bigfish_1404fb040", tpl_1436::<0x58, 0x60, 0x158, 0x143c8dbcc, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1404fb040
    Replacement::draft_throws(0x1404fb180, "bigfish_1404fb180", fn_1404fb180 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::equipment::systems::attach_equippable::system>
    Replacement::draft_throws(0x1404fb4f0, "bigfish_1404fb4f0", tpl_4886::<0x143cabe48, 0x143cabf38, 0x1411e9f50, 0x1404fde60, 0x143cabd90, 0x1420af120, 0x14035a280, 0x1420a28e0, 0x1404fd7c0, 0x1404fca90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317e276, 0x142646960, 0x143cabf58, 0x143cabf68, 0x143cabf6c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::equipment::systems::stream_in_weapon::system>
    Replacement::draft_throws(0x1404fb7d0, "bigfish_1404fb7d0", tpl_3408::<0x143cabd84, 0x143cabf70, 0x1411e9f50, 0x1404fdd30, 0x143cabec4, 0x1420af120, 0x1420a28e0, 0x1404fd970, 0x1404fcbf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14317e626, 0x142646960, 0x143cabf90, 0x143cabfa0, 0x143cabfa4, 0x143cabfa6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::equipment::systems::stream_out_weapon::system>
    Replacement::draft(0x1404fbad0, "unknown_1404fbad0", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143094930> as *const ()), // sub_1404fbad0
    Replacement::draft(0x1404fbc50, "unknown_1404fbc50", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1430a1868> as *const ()), // sub_1404fbc50
    Replacement::draft(0x1404fbdd0, "unknown_1404fbdd0", fn_1404fbdd0 as *const ()), // sub_1404fbdd0
    Replacement::draft(0x1404fbe60, "unknown_1404fbe60", fn_1404fbe60 as *const ()), // sub_1404fbe60
    Replacement::draft(0x1404fc010, "unknown_1404fc010", fn_1404fc010 as *const ()), // sub_1404fc010
    Replacement::draft(0x1404fc150, "unknown_1404fc150", tpl_60901::<0x143c8ceec, 0x580e0, 0x143c9deb0, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141a68a40> as *const ()), // sub_1404fc150
    Replacement::draft_throws(0x1404fc260, "unknown_1404fc260", tpl_1410::<0x88, 0x10088, 0x143c8edb4, 0x20, 0x8, 0x143c8ce88, 0x580e0, 0x1404fc440, 0x1404fc360> as *const (), Throws::Discard), // sub_1404fc260
    Replacement::draft_throws(0x1404fc360, "unknown_1404fc360", tpl_1411::<0x18, 0x20, 0x30, 0x8, 0x28, 0x141a68540> as *const (), Throws::Discard), // sub_1404fc360
    Replacement::draft(0x1404fc670, "unknown_1404fc670", tpl_61197::<0x143c8d160, 0x142489ee0, 0x143c970e0, 0x143c8dbd0, 0x143c8dbc4, 0x1426467e8, 0x1404fc803> as *const ()), // sub_1404fc670
    Replacement::draft(0x1404fc810, "unknown_1404fc810", tpl_4996::<0x54, 0x14240ab10> as *const ()), // sub_1404fc810
    Replacement::draft(0x1404fc840, "unknown_1404fc840", tpl_3167::<0x88, 0x10088, 0x143cabd8c, 0x20, 0x8, 0x142407c80, 0x1404fd2c0, 0x1404fc960> as *const ()), // sub_1404fc840
    Replacement::draft(0x1404fc960, "unknown_1404fc960", tpl_3168::<0x18, 0x20, 0x40, 0x8, 0x141a69110> as *const ()), // sub_1404fc960
    Replacement::draft(0x1404fca90, "unknown_1404fca90", tpl_4341::<0x143c94108, 0x142407c80, 0x143c91064, 0x142489ee0, 0x141a68e90> as *const ()), // sub_1404fca90
    Replacement::draft(0x1404fcbf0, "unknown_1404fcbf0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1404fb040> as *const ()), // sub_1404fcbf0
    Replacement::draft(0x1404fcc90, "unknown_1404fcc90", fn_1404fcc90 as *const ()), // sub_1404fcc90
    Replacement::draft(0x1404fcea0, "unknown_1404fcea0", tpl_60916::<0x143c8ceec, 0x143c9deb0, 0x142489ee0, 0x141a68a40> as *const ()), // sub_1404fcea0
    Replacement::draft_throws(0x1404fd010, "unknown_1404fd010", tpl_61145::<0x143c8edb4, 0x143c8ce88, 0x143c8dbc4, 0x142489ee0, 0x143c8dbd0, 0x143c96dec, 0x141a68540> as *const (), Throws::Discard), // sub_1404fd010
    Replacement::draft(0x1404fd560, "unknown_1404fd560", tpl_3170::<0x143cabd8c, 0x142407c80, 0x1404fd3a0, 0x141a69110> as *const ()), // sub_1404fd560
    Replacement::draft(0x1404fd7c0, "unknown_1404fd7c0", tpl_4344::<0x143c94108, 0x142407c80, 0x143c91064, 0x142489ee0, 0x141a68e90> as *const ()), // sub_1404fd7c0
    Replacement::draft(0x1404fd970, "unknown_1404fd970", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1404fb040> as *const ()), // sub_1404fd970
    Replacement::draft(0x1404fe0e0, "unknown_1404fe0e0", fn_1404fe0e0 as *const ()), // sub_1404fe0e0
    Replacement::draft(0x1404fe7a0, "bigfish_1404fe7a0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14317ff10, 0x8> as *const ()), // sub_1404fe7a0
    Replacement::draft(0x1404fe820, "bigfish_1404fe820", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14317ff10> as *const ()), // sub_1404fe820
    Replacement::draft_throws(0x1404fea50, "bigfish_1404fea50", fn_1404fea50 as *const (), Throws::Caught), // sub_1404fea50
    Replacement::draft_throws(0x1404febe0, "bigfish_1404febe0", tpl_1905::<0x14387b7c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1404febe0
    Replacement::draft_throws(0x1404fec90, "bigfish_1404fec90", fn_1404fec90 as *const (), Throws::Caught), // sub_1404fec90
    Replacement::draft_throws(0x1404ff0a0, "bigfish_1404ff0a0", fn_1404ff0a0 as *const (), Throws::Caught), // sub_1404ff0a0
    Replacement::draft_throws(0x1404ff130, "bigfish_1404ff130", tpl_1908::<0x14387c500, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1404ff130
    Replacement::draft(0x1404ff1e0, "bigfish_1404ff1e0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14317fff0, 0x8> as *const ()), // sub_1404ff1e0
    Replacement::draft(0x1404ff260, "bigfish_1404ff260", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14317fff0> as *const ()), // sub_1404ff260
    Replacement::draft_throws(0x1404ff490, "bigfish_1404ff490", fn_1404ff490 as *const (), Throws::Caught), // sub_1404ff490
    Replacement::draft_throws(0x1404ff620, "bigfish_1404ff620", tpl_1905::<0x14387b7c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1404ff620
    Replacement::draft_throws(0x1404ff6d0, "bigfish_1404ff6d0", tpl_5001::<0x143cac1f0, 0x143c88944, 0x143c88918, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d120, 0x143c91c28, 0x143c91b20, 0x143c88938, 0x141834a0c> as *const (), Throws::Discard), // sub_1404ff6d0
    Replacement::draft(0x1404ff880, "unknown_1404ff880", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431804b0, 0x8> as *const ()), // sub_1404ff880
    Replacement::draft(0x1404ff900, "unknown_1404ff900", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431804b0> as *const ()), // sub_1404ff900
    Replacement::draft_throws(0x1404ffb30, "unknown_1404ffb30", fn_1404ffb30 as *const (), Throws::Caught), // sub_1404ffb30
    Replacement::draft_throws(0x1404ffcc0, "unknown_1404ffcc0", tpl_1905::<0x14387b7d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c500> as *const (), Throws::Discard), // sub_1404ffcc0
    Replacement::draft(0x1404ffdb0, "unknown_1404ffdb0", tpl_1171::<0x58, 0x60, 0x143cac378, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1404ffdb0
    Replacement::draft_throws(0x1404ffe90, "bigfish_1404ffe90", tpl_3061::<0x143cac454, 0x143cac3f0, 0x1411e9f50, 0x140501640, 0x143cac458, 0x1420af120, 0x1420a28e0, 0x140501010, 0x140500d90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143181076, 0x142646960, 0x143cac410, 0x143cac418, 0x143cac41c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::enemy_advantage_zone::update::system>
    Replacement::draft(0x140500180, "bigfish_140500180", tpl_1095::<0x143cac378, 0x14020e8f0> as *const ()), // sub_140500180
    Replacement::draft(0x140500360, "bigfish_140500360", tpl_1436::<0x58, 0x60, 0x158, 0x143cac378, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140500360
    Replacement::draft(0x140500440, "bigfish_140500440", tpl_1403::<0x143cac1f8, 0x140500c10, 0x14020e8f0> as *const ()), // sub_140500440
    Replacement::draft_throws(0x140500650, "bigfish_140500650", tpl_1100::<0x143cac2f0, 0x143cac278, 0x1411e9f50, 0x140501870, 0x143cac494, 0x1420af120, 0x1420a28e0, 0x1405013b0, 0x140500e80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431814d6, 0x142646960, 0x143cac298, 0x143cac2a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::enemy_advantage_zone::stream_in::system>
    Replacement::draft_throws(0x140500930, "bigfish_140500930", tpl_1074::<0x143cac270, 0x143cac420, 0x1411e9f50, 0x140501770, 0x143cac374, 0x1420af120, 0x1420a28e0, 0x140501560, 0x140500f70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431812d6, 0x142646960, 0x143cac440, 0x143cac450, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::enemy_advantage_zone::stream_out::system>
    Replacement::draft(0x140500c10, "unknown_140500c10", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143180770> as *const ()), // sub_140500c10
    Replacement::draft(0x140500d90, "unknown_140500d90", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140381a70, 0x580f0, 0x141a6ce80> as *const ()), // sub_140500d90
    Replacement::draft(0x140500e80, "unknown_140500e80", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140381a70, 0x580f0, 0x141a6cf40> as *const ()), // sub_140500e80
    Replacement::draft(0x140500f70, "unknown_140500f70", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140500360> as *const ()), // sub_140500f70
    Replacement::draft(0x140501010, "unknown_140501010", tpl_3213::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x141a6ce80> as *const ()), // sub_140501010
    Replacement::draft(0x1405013b0, "unknown_1405013b0", tpl_3213::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x141a6cf40> as *const ()), // sub_1405013b0
    Replacement::draft(0x140501560, "unknown_140501560", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140500360> as *const ()), // sub_140501560
    Replacement::draft(0x140501b60, "unknown_140501b60", tpl_2944::<0x1430b62b0, 0x8, 0xc, 0x10, 0x18, 0x14, 0x1401e84a0, 0x38, 0x39, 0x3a, 0x3c, 0x40, 0x14240aa00> as *const ()), // sub_140501b60
    Replacement::draft(0x140501c40, "unknown_140501c40", tpl_1149::<0x8, 0x140267a30, 0x1401e66f0> as *const ()), // sub_140501c40
    Replacement::draft(0x140501d80, "unknown_140501d80", tpl_64194::<0x140501e80, 0x1426467e8, 0x140501e7d> as *const ()), // sub_140501d80
    Replacement::draft(0x140502040, "unknown_140502040", tpl_60443::<0x143ca38f8, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x1405020fa> as *const ()), // sub_140502040
    Replacement::draft(0x140502100, "unknown_140502100", tpl_61508::<0x30, 0x40, 0x48, 0x140502210, 0x140502520, 0x20, 0x68, 0x60, 0x70, 0x1426467e8, 0x140502201> as *const ()), // sub_140502100
    Replacement::draft(0x140502210, "unknown_140502210", fn_140502210 as *const ()), // sub_140502210
    Replacement::draft(0x140502520, "unknown_140502520", fn_140502520 as *const ()), // sub_140502520
    Replacement::draft(0x140502630, "unknown_140502630", fn_140502630 as *const ()), // sub_140502630
    Replacement::draft(0x1405026c0, "unknown_1405026c0", tpl_60443::<0x143c9d614, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x14050277a> as *const ()), // sub_1405026c0
    Replacement::draft(0x140502780, "unknown_140502780", tpl_64194::<0x140502880, 0x1426467e8, 0x14050287d> as *const ()), // sub_140502780
    Replacement::draft(0x140502a40, "unknown_140502a40", fn_140502a40 as *const ()), // sub_140502a40
    Replacement::draft(0x140502cb0, "unknown_140502cb0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cacda4, 0x158, 0x14020e8f0> as *const ()), // sub_140502cb0
    Replacement::draft(0x140502da0, "unknown_140502da0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c9d19c, 0x158, 0x14020e8f0> as *const ()), // sub_140502da0
    Replacement::draft(0x140502e90, "unknown_140502e90", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca5ebc, 0x158, 0x14020e8f0> as *const ()), // sub_140502e90
    Replacement::draft(0x140502f80, "unknown_140502f80", tpl_1526::<0x142751880, 0x1410d6ca0, 0x14050b070, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140502f80
    Replacement::draft_throws(0x140503220, "bigfish_140503220", fn_140503220 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_boneless_render_transform::system>
    Replacement::draft_throws(0x140503570, "bigfish_140503570", tpl_1100::<0x143cac668, 0x143cada20, 0x1411e9f50, 0x140514c00, 0x143cac91c, 0x1420af120, 0x1420a28e0, 0x14050fbf0, 0x14050baf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143189d96, 0x142646960, 0x143cada40, 0x143cada50, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_previous_transform::system>
    Replacement::draft_throws(0x140503850, "bigfish_140503850", tpl_4130::<0x143cac930, 0x143cacae0, 0x1411e9f50, 0x1405149f0, 0x143cacef0, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x14050fd90, 0x14050bbd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143189956, 0x142646960, 0x143cacb00, 0x143cacb08, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_scripted_move::system>
    Replacement::draft_throws(0x140503b60, "bigfish_140503b60", tpl_3591::<0x143cad1e8, 0x143cac8e8, 0x1411e9f50, 0x1405148b0, 0x143cac920, 0x1420af120, 0x14036fad0, 0x1420a28e0, 0x14050ffb0, 0x14050bde0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318b096, 0x142646960, 0x143cac908, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_mimicking::system>
    Replacement::draft_throws(0x140503e50, "bigfish_140503e50", tpl_5008::<0x143cad8ec, 0x143cacff0, 0x1411e9f50, 0x140514710, 0x143cac934, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x140510180, 0x14050bfa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318acf6, 0x142646960, 0x143cad010, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_rigid_body_from_current_transform::system>
    Replacement::draft_throws(0x140504160, "bigfish_140504160", tpl_3084::<0x143cad800, 0x143cac4d8, 0x1411e9f50, 0x140514620, 0x143cad6d8, 0x1420af120, 0x1405143e0, 0x143cacd10, 0x140215750, 0x1420a28e0, 0x1405103a0, 0x14050c140, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318a756, 0x142646960, 0x143cac4f8, 0x143cac500, 0x143cac504, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_latching_double::system>
    Replacement::draft_throws(0x1405044c0, "bigfish_1405044c0", tpl_3061::<0x143cad188, 0x143cad3a8, 0x1411e9f50, 0x1405141a0, 0x143cada64, 0x1420af120, 0x1420a28e0, 0x140510550, 0x14050c2a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318a2e6, 0x142646960, 0x143cad3c8, 0x143cad3d0, 0x143cad3d4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_latching_single::system>
    Replacement::draft_throws(0x1405047b0, "bigfish_1405047b0", fn_1405047b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_current_transform_from_rigid_body::system>
    Replacement::draft_throws(0x140504ab0, "bigfish_140504ab0", tpl_2988::<0x143cadab4, 0x143cac520, 0x1411e9f50, 0x14050c630, 0x140510970, 0x14050c870, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143187666, 0x142646960, 0x143cac540, 0x143cac548, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_navigation::system>
    Replacement::draft_throws(0x140504d00, "bigfish_140504d00", tpl_5010::<0x143cacbd8, 0x143cad808, 0x1411e9f50, 0x140513a70, 0x143cac92c, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x140510c30, 0x14050cc00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143187296, 0x142646960, 0x143cad828, 0x143cad830, 0x143cad834, 0x143cad836, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_transform_events::system>
    Replacement::draft_throws(0x140505030, "bigfish_140505030", tpl_3084::<0x143cad318, 0x143cad720, 0x1411e9f50, 0x140513980, 0x143cac990, 0x1420af120, 0x1405138d0, 0x143cad3dc, 0x140215750, 0x1420a28e0, 0x140510e40, 0x14050cdd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143187026, 0x142646960, 0x143cad740, 0x143cad748, 0x143cad74c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::initialize_double_door::system>
    Replacement::draft_throws(0x140505390, "bigfish_140505390", tpl_5011::<0x143cad3e8, 0x143cad600, 0x1411e9f50, 0x140513750, 0x143cad718, 0x1420af120, 0x14050e390, 0x1420a28e0, 0x140510ff0, 0x14050cf30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143189526, 0x142646960, 0x143cad620, 0x143cad630, 0x143cad638, 0x143cad63c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_single_door_interact_visibility::system>
    Replacement::draft_throws(0x140505680, "bigfish_140505680", fn_140505680 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_double_door_interact_visibility::system>
    Replacement::draft_throws(0x140505a10, "bigfish_140505a10", tpl_3211::<0x143cad090, 0x143cad4d8, 0x1411e9f50, 0x140513350, 0x143cad3e0, 0x1420af120, 0x1420a28e0, 0x1405113e0, 0x14050d300, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143188cf6, 0x142646960, 0x143cad4f8, 0x143cad508, 0x143cad50a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_previous_hide_reason::system>
    Replacement::draft_throws(0x140505d00, "bigfish_140505d00", tpl_4359::<0x143cad69c, 0x143cad7c8, 0x14050b280, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143188716, 0x142646960, 0x143cad7e8, 0x143cad7f0, 0x143cad7f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_player_contact::system>
    Replacement::draft_throws(0x140505f10, "bigfish_140505f10", tpl_1101::<0x143cacc8c, 0x143cada80, 0x1411e9f50, 0x140512e70, 0x143cad030, 0x1420af120, 0x1420a28e0, 0x1405117a0, 0x14050d620, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143186286, 0x142646960, 0x143cadaa0, 0x143cadaa8, 0x143cadaac, 0x143cadaae, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_partially_opened::system>
    Replacement::draft_throws(0x140506210, "bigfish_140506210", tpl_3281::<0x143cac514, 0x143cad2e8, 0x1411e9f50, 0x140512d80, 0x143cadab0, 0x1420af120, 0x1420a28e0, 0x140511990, 0x14050d820, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431860d6, 0x142646960, 0x143cad308, 0x143cad310, 0x143cad314, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::remove_mimic_rotation::system>
    Replacement::draft_throws(0x140506500, "bigfish_140506500", fn_140506500 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::send_events::system>
    Replacement::draft(0x140506810, "bigfish_140506810", tpl_3354::<0x143ca38f8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140506810
    Replacement::draft(0x140506b60, "bigfish_140506b60", tpl_1097::<0x143c9d614, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140506b60
    Replacement::draft(0x140506e90, "bigfish_140506e90", tpl_1098::<0x143cad48c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140506e90
    Replacement::draft(0x1405071d0, "bigfish_1405071d0", tpl_1402::<0x143c9e55c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405071d0
    Replacement::draft(0x140507520, "bigfish_140507520", tpl_5014::<0x143c8f6e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140507520
    Replacement::draft(0x140507850, "bigfish_140507850", tpl_3077::<0x143c9d19c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140507850
    Replacement::draft(0x140507b80, "bigfish_140507b80", tpl_1095::<0x143cacda4, 0x14020e8f0> as *const ()), // sub_140507b80
    Replacement::draft(0x140507d60, "bigfish_140507d60", tpl_3076::<0x143ca5ebc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140507d60
    Replacement::draft(0x1405080b0, "bigfish_1405080b0", tpl_1095::<0x143c9f008, 0x14020e8f0> as *const ()), // sub_1405080b0
    Replacement::draft(0x140508290, "bigfish_140508290", tpl_1095::<0x143cada68, 0x14020e8f0> as *const ()), // sub_140508290
    Replacement::draft(0x140508470, "bigfish_140508470", tpl_1094::<0x143c92a7c, 0x14050b4a0, 0x14020e8f0> as *const ()), // sub_140508470
    Replacement::draft(0x140508670, "bigfish_140508670", tpl_1094::<0x143c92ee8, 0x14050b620, 0x14020e8f0> as *const ()), // sub_140508670
    Replacement::draft(0x140508870, "bigfish_140508870", tpl_3073::<0x143c937c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140508870
    Replacement::draft(0x140508bd0, "bigfish_140508bd0", tpl_1095::<0x143c9e5cc, 0x14020e8f0> as *const ()), // sub_140508bd0
    Replacement::draft(0x140508db0, "bigfish_140508db0", tpl_5015::<0x143c9d840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140508db0
    Replacement::draft(0x140509120, "bigfish_140509120", tpl_3077::<0x143c92c98, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140509120
    Replacement::draft(0x140509450, "bigfish_140509450", tpl_4324::<0x143c93228, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140509450
    Replacement::draft(0x140509790, "bigfish_140509790", tpl_3077::<0x143caca68, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140509790
    Replacement::draft(0x140509ac0, "bigfish_140509ac0", tpl_3074::<0x143cad484, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140509ac0
    Replacement::draft(0x140509d90, "bigfish_140509d90", tpl_3270::<0x143c9d9fc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140509d90
    Replacement::draft_throws(0x14050a100, "bigfish_14050a100", fn_14050a100 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::stream_in_navigation::system>
    Replacement::draft_throws(0x14050a450, "bigfish_14050a450", tpl_5017::<0x143cac910, 0x143cac6c0, 0x1411e9f50, 0x140515570, 0x143cad0e0, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x140512660, 0x14050f5a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431867d6, 0x142646960, 0x143cac6e0, 0x143cac6e8, 0x143cac6ea, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::stream_in_transform::system>
    Replacement::draft_throws(0x14050a770, "bigfish_14050a770", fn_14050a770 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::stream_out_component::system<struct bigfish::door::component::Transform
    Replacement::draft_throws(0x14050aa70, "bigfish_14050aa70", fn_14050aa70 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::stream_out_component::system<struct bigfish::door::component::DynamicBo
    Replacement::draft_throws(0x14050ad70, "bigfish_14050ad70", fn_14050ad70 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::stream_out_component::system<struct bigfish::door::component::Navigatio
    Replacement::draft(0x14050b070, "bigfish_14050b070", fn_14050b070 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::door::on_navigation_removed::system>
    Replacement::draft_throws(0x14050b280, "unknown_14050b280", tpl_1187::<0x1411e9f50, 0x140513260, 0x143cad7f8, 0x1420af120, 0x140513020, 0x143cacd7c, 0x140215750, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x1405115f0, 0x14050d420> as *const (), Throws::Discard), // sub_14050b280
    Replacement::draft(0x14050b4a0, "unknown_14050b4a0", tpl_5022::<0x1401e6d50, 0x140213020, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140501b60> as *const ()), // sub_14050b4a0
    Replacement::draft(0x14050b620, "unknown_14050b620", fn_14050b620 as *const ()), // sub_14050b620
    Replacement::draft(0x14050b7e0, "unknown_14050b7e0", tpl_5024::<0x1411e9f50, 0x140514e90, 0x143ca5ebc, 0x1401fc320, 0x143c8fd28, 0x140215610, 0x143cada18, 0x1420a28e0, 0x140512bd0, 0x1401de410> as *const ()), // sub_14050b7e0
    Replacement::draft(0x14050baf0, "unknown_14050baf0", tpl_62755::<0x88, 0x10088, 0x14050f1b0, 0x141a71fc0> as *const ()), // sub_14050baf0
    Replacement::draft(0x14050bbd0, "unknown_14050bbd0", fn_14050bbd0 as *const ()), // sub_14050bbd0
    Replacement::draft(0x14050bcd0, "unknown_14050bcd0", tpl_3132::<0x18, 0x20, 0x40, 0x8, 0x141a717a0> as *const ()), // sub_14050bcd0
    Replacement::draft(0x14050bde0, "unknown_14050bde0", tpl_4808::<0x88, 0x10088, 0x143c94b44, 0x20, 0x8, 0x14050edb0, 0x14050bec0> as *const ()), // sub_14050bde0
    Replacement::draft(0x14050bec0, "unknown_14050bec0", tpl_4809::<0x18, 0x20, 0x8, 0x141a71590> as *const ()), // sub_14050bec0
    Replacement::draft(0x14050bfa0, "unknown_14050bfa0", tpl_5026::<0x88, 0x10088, 0x580e0, 0x143c8f63c, 0x14050eb90, 0x14050c050> as *const ()), // sub_14050bfa0
    Replacement::draft(0x14050c050, "unknown_14050c050", tpl_5027::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141a70d10> as *const ()), // sub_14050c050
    Replacement::draft(0x14050c140, "unknown_14050c140", tpl_4341::<0x143cacd10, 0x142407c80, 0x143ca38f8, 0x142489ee0, 0x141a70910> as *const ()), // sub_14050c140
    Replacement::draft(0x14050c2a0, "unknown_14050c2a0", fn_14050c2a0 as *const ()), // sub_14050c2a0
    Replacement::draft(0x14050c380, "unknown_14050c380", tpl_3560::<0x18, 0x20, 0x40, 0x8, 0x141a70840> as *const ()), // sub_14050c380
    Replacement::draft(0x14050c490, "unknown_14050c490", tpl_5026::<0x88, 0x10088, 0x580e0, 0x143c8f63c, 0x14050e890, 0x14050c540> as *const ()), // sub_14050c490
    Replacement::draft(0x14050c540, "unknown_14050c540", tpl_5027::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141a70760> as *const ()), // sub_14050c540
    Replacement::draft_throws(0x14050c630, "unknown_14050c630", fn_14050c630 as *const (), Throws::Discard), // sub_14050c630
    Replacement::draft(0x14050c870, "unknown_14050c870", fn_14050c870 as *const ()), // sub_14050c870
    Replacement::draft(0x14050c960, "unknown_14050c960", tpl_4519::<0x8, 0x20, 0x143cace78, 0x143cac918, 0x143ca3714, 0x143c8fd28, 0x580e0> as *const ()), // sub_14050c960
    Replacement::draft(0x14050ca50, "unknown_14050ca50", fn_14050ca50 as *const ()), // sub_14050ca50
    Replacement::draft(0x14050cc00, "unknown_14050cc00", tpl_5032::<0x88, 0x10088, 0x580e0, 0x143c8d910, 0x14050e470, 0x14050ccc0> as *const ()), // sub_14050cc00
    Replacement::draft(0x14050ccc0, "unknown_14050ccc0", tpl_5033::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141a711b0> as *const ()), // sub_14050ccc0
    Replacement::draft(0x14050cdd0, "unknown_14050cdd0", tpl_4341::<0x143cad3dc, 0x142407c80, 0x143ca38f8, 0x142489ee0, 0x141a6f990> as *const ()), // sub_14050cdd0
    Replacement::draft(0x14050cf30, "unknown_14050cf30", tpl_5034::<0x143c9f020, 0x14050e240, 0x14050d070> as *const ()), // sub_14050cf30
    Replacement::draft(0x14050d070, "unknown_14050d070", tpl_5035::<0x18, 0x20, 0x40, 0x8, 0x141a6fb80> as *const ()), // sub_14050d070
    Replacement::draft(0x14050d180, "unknown_14050d180", tpl_4369::<0x143cac924, 0x143c9f020, 0x143ca38f8, 0x142489ee0, 0x141a6f640> as *const ()), // sub_14050d180
    Replacement::draft(0x14050d300, "unknown_14050d300", tpl_62730::<0x88, 0x10088, 0x14050e110, 0x141a6f330> as *const ()), // sub_14050d300
    Replacement::draft_throws(0x14050d420, "unknown_14050d420", fn_14050d420 as *const (), Throws::Discard), // sub_14050d420
    Replacement::draft(0x14050d580, "unknown_14050d580", tpl_3709::<0x143cacd7c, 0x20, 0x8, 0x143c8f63c, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_14050d580
    Replacement::draft(0x14050d620, "unknown_14050d620", fn_14050d620 as *const ()), // sub_14050d620
    Replacement::draft(0x14050d710, "unknown_14050d710", tpl_5033::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141a71db0> as *const ()), // sub_14050d710
    Replacement::draft(0x14050d820, "unknown_14050d820", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x14050ddc0, 0x580f0, 0x141a72100> as *const ()), // sub_14050d820
    Replacement::draft(0x14050d910, "unknown_14050d910", tpl_64057::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x14050dcc0, 0x141a6f380> as *const ()), // sub_14050d910
    Replacement::draft(0x14050da10, "unknown_14050da10", fn_14050da10 as *const ()), // sub_14050da10
    Replacement::draft(0x14050dec0, "unknown_14050dec0", tpl_5037::<0x8, 0x10, 0x18, 0x14050df70, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50> as *const ()), // sub_14050dec0
    Replacement::draft(0x14050df70, "unknown_14050df70", tpl_5038::<0x143c92a7c, 0x142489ee0, 0x143c937c8, 0x143c9d840, 0x143c93228, 0x143c92c98, 0x143c9e55c, 0x143c9f008> as *const ()), // sub_14050df70
    Replacement::draft_throws(0x14050e390, "unknown_14050e390", tpl_1330::<0x8, 0x14041e990, 0x143c9f020, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14050e390
    Replacement::draft(0x14050e6e0, "unknown_14050e6e0", fn_14050e6e0 as *const ()), // sub_14050e6e0
    Replacement::draft_throws(0x14050e7b0, "unknown_14050e7b0", tpl_1330::<0x8, 0x14046cdd0, 0x143ca3714, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14050e7b0
    Replacement::draft(0x14050ef10, "unknown_14050ef10", fn_14050ef10 as *const ()), // sub_14050ef10
    Replacement::draft(0x14050efe0, "unknown_14050efe0", tpl_4961::<0x143c92a7c, 0x142489ee0, 0x143c92ee8, 0x143c92c98, 0x143cad484, 0x143c9d840, 0x143c937c8, 0x143c93228, 0x143c9f008> as *const ()), // sub_14050efe0
    Replacement::draft(0x14050f410, "unknown_14050f410", tpl_3259::<0x30, 0x14240ab10> as *const ()), // sub_14050f410
    Replacement::draft_throws(0x14050f450, "unknown_14050f450", fn_14050f450 as *const (), Throws::Discard), // sub_14050f450
    Replacement::draft(0x14050f5a0, "unknown_14050f5a0", fn_14050f5a0 as *const ()), // sub_14050f5a0
    Replacement::draft(0x14050f6b0, "unknown_14050f6b0", fn_14050f6b0 as *const ()), // sub_14050f6b0
    Replacement::draft(0x14050f7d0, "unknown_14050f7d0", tpl_1126::<0x142407c80, 0x580f0, 0x140502cb0> as *const ()), // sub_14050f7d0
    Replacement::draft(0x14050f890, "unknown_14050f890", tpl_1126::<0x142407c80, 0x580f0, 0x140502da0> as *const ()), // sub_14050f890
    Replacement::draft(0x14050f950, "unknown_14050f950", tpl_1126::<0x142407c80, 0x580f0, 0x140502e90> as *const ()), // sub_14050f950
    Replacement::draft(0x14050fa10, "unknown_14050fa10", tpl_64222::<0x143c900d8, 0x143c8f63c, 0x14050f2b0, 0x141a72040> as *const ()), // sub_14050fa10
    Replacement::draft(0x14050fbf0, "unknown_14050fbf0", tpl_62763::<0x143c937c8, 0x142489ee0, 0x143cad484, 0x141a71fc0> as *const ()), // sub_14050fbf0
    Replacement::draft(0x14050fd90, "unknown_14050fd90", fn_14050fd90 as *const ()), // sub_14050fd90
    Replacement::draft(0x14050ffb0, "unknown_14050ffb0", tpl_64008::<0x143c94b44, 0x14050edb0, 0x141a71590> as *const ()), // sub_14050ffb0
    Replacement::draft(0x140510180, "unknown_140510180", tpl_64224::<0x143c8f63c, 0x14050ec40, 0x141a70d10> as *const ()), // sub_140510180
    Replacement::draft(0x1405103a0, "unknown_1405103a0", tpl_4344::<0x143cacd10, 0x142407c80, 0x143ca38f8, 0x142489ee0, 0x141a70910> as *const ()), // sub_1405103a0
    Replacement::draft(0x140510550, "unknown_140510550", fn_140510550 as *const ()), // sub_140510550
    Replacement::draft(0x140510750, "unknown_140510750", tpl_64224::<0x143c8f63c, 0x14050e940, 0x141a70760> as *const ()), // sub_140510750
    Replacement::draft(0x140510970, "unknown_140510970", fn_140510970 as *const ()), // sub_140510970
    Replacement::draft(0x140510c30, "unknown_140510c30", tpl_64227::<0x143c8d910, 0x14050e530, 0x141a711b0> as *const ()), // sub_140510c30
    Replacement::draft(0x140510e40, "unknown_140510e40", tpl_4344::<0x143cad3dc, 0x142407c80, 0x143ca38f8, 0x142489ee0, 0x141a6f990> as *const ()), // sub_140510e40
    Replacement::draft(0x140510ff0, "unknown_140510ff0", tpl_64228::<0x143c9f020, 0x14050e240, 0x141a6fb80> as *const ()), // sub_140510ff0
    Replacement::draft(0x140511200, "unknown_140511200", tpl_63609::<0x143cac924, 0x143c9f020, 0x143ca38f8, 0x142489ee0, 0x141a6f640> as *const ()), // sub_140511200
    Replacement::draft(0x1405113e0, "unknown_1405113e0", tpl_62741::<0x143c8d87c, 0x142489ee0, 0x143c93228, 0x143caca68, 0x141a6f330> as *const ()), // sub_1405113e0
    Replacement::draft_throws(0x1405115f0, "unknown_1405115f0", fn_1405115f0 as *const (), Throws::Discard), // sub_1405115f0
    Replacement::draft(0x1405117a0, "unknown_1405117a0", fn_1405117a0 as *const ()), // sub_1405117a0
    Replacement::draft(0x140511990, "unknown_140511990", tpl_3213::<0x142407c80, 0x143c9e55c, 0x142489ee0, 0x143c93228, 0x141a72100> as *const ()), // sub_140511990
    Replacement::draft(0x140511b40, "unknown_140511b40", tpl_64074::<0x143c8ceec, 0x143c8d160, 0x142489ee0, 0x143c9d9fc, 0x141a6f380> as *const ()), // sub_140511b40
    Replacement::draft_throws(0x1405124b0, "unknown_1405124b0", fn_1405124b0 as *const (), Throws::Discard), // sub_1405124b0
    Replacement::draft(0x140512660, "unknown_140512660", fn_140512660 as *const ()), // sub_140512660
    Replacement::draft(0x1405128a0, "unknown_1405128a0", tpl_1130::<0x142407c80, 0x140502cb0> as *const ()), // sub_1405128a0
    Replacement::draft(0x1405129b0, "unknown_1405129b0", tpl_1130::<0x142407c80, 0x140502da0> as *const ()), // sub_1405129b0
    Replacement::draft(0x140512ac0, "unknown_140512ac0", tpl_1130::<0x142407c80, 0x140502e90> as *const ()), // sub_140512ac0
    Replacement::draft(0x140512bd0, "unknown_140512bd0", tpl_5049::<0x143ca5ebc, 0x5e308, 0x580e0, 0x143c8fd28, 0x143cada18, 0x10, 0x8, 0x18, 0x141a72250, 0x140489dd0> as *const ()), // sub_140512bd0
    Replacement::draft(0x140512e70, "unknown_140512e70", fn_140512e70 as *const ()), // sub_140512e70
    Replacement::draft(0x140513020, "unknown_140513020", fn_140513020 as *const ()), // sub_140513020
    Replacement::draft(0x140513490, "unknown_140513490", fn_140513490 as *const ()), // sub_140513490
    Replacement::draft(0x140513a70, "unknown_140513a70", tpl_5053::<0x143cac5b0, 0x143cacb10, 0x141834a78, 0x143c9e5cc, 0x143c8d160, 0x143c937c8, 0x143c92c98, 0x143cad484, 0x143c9d9fc, 0x143c9d840, 0x143c93228, 0x143cacb30, 0x143cacb50, 0x143cacb70, 0x141834a0c> as *const ()), // sub_140513a70
    Replacement::draft(0x140513c30, "unknown_140513c30", fn_140513c30 as *const ()), // sub_140513c30
    Replacement::draft(0x140514000, "unknown_140514000", tpl_5055::<0x143cac678, 0x143cad190, 0x141834a78, 0x143c9e5cc, 0x143c92a7c, 0x143c8d87c, 0x143c914bc, 0x143c8d120, 0x143c9d19c, 0x143c937c8, 0x143cad1b0, 0x143cad1d0, 0x143cad1e0, 0x141834a0c> as *const ()), // sub_140514000
    Replacement::draft(0x1405141a0, "unknown_1405141a0", fn_1405141a0 as *const ()), // sub_1405141a0
    Replacement::draft(0x1405143e0, "unknown_1405143e0", fn_1405143e0 as *const ()), // sub_1405143e0
    Replacement::draft(0x140514710, "unknown_140514710", tpl_5055::<0x143cad490, 0x143cad840, 0x141834a78, 0x143c9e5cc, 0x143c8d87c, 0x143c8f6d8, 0x143c8d120, 0x143c9d19c, 0x143c937c8, 0x143c914bc, 0x143cad860, 0x143cad880, 0x143cad890, 0x141834a0c> as *const ()), // sub_140514710
    Replacement::draft(0x1405149f0, "unknown_1405149f0", fn_1405149f0 as *const ()), // sub_1405149f0
    Replacement::draft(0x140514cf0, "unknown_140514cf0", fn_140514cf0 as *const ()), // sub_140514cf0
    Replacement::draft(0x140514e90, "unknown_140514e90", tpl_3188::<0x140515200, 0x14318c008, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140514e90
    Replacement::draft(0x140514f10, "unknown_140514f10", tpl_4348::<0x10, 0x18, 0x140489dd0> as *const ()), // sub_140514f10
    Replacement::draft(0x1405150d0, "unknown_1405150d0", fn_1405150d0 as *const ()), // sub_1405150d0
    Replacement::draft(0x140515200, "unknown_140515200", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14318c008, 0x8, 0x20> as *const ()), // sub_140515200
    Replacement::draft(0x140515570, "unknown_140515570", fn_140515570 as *const ()), // sub_140515570
    Replacement::draft(0x140516a30, "unknown_140516a30", fn_140516a30 as *const ()), // sub_140516a30
    Replacement::draft(0x140516b90, "unknown_140516b90", tpl_1171::<0x58, 0x60, 0x143cad48c, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140516b90
    Replacement::draft(0x140516c70, "unknown_140516c70", fn_140516c70 as *const ()), // sub_140516c70
    Replacement::draft(0x140516e10, "unknown_140516e10", fn_140516e10 as *const ()), // sub_140516e10
    Replacement::draft(0x140517100, "unknown_140517100", tpl_61197::<0x143c933f0, 0x142489ee0, 0x143c8d120, 0x143c8d708, 0x143c8d68c, 0x1426467e8, 0x140517293> as *const ()), // sub_140517100
    Replacement::draft(0x1405172a0, "unknown_1405172a0", fn_1405172a0 as *const ()), // sub_1405172a0
    Replacement::draft(0x140517360, "unknown_140517360", fn_140517360 as *const ()), // sub_140517360
    Replacement::draft_throws(0x1405173d0, "bigfish_1405173d0", tpl_1298::<0x143cade58, 0x143cadb30, 0x1411e9f50, 0x140518be0, 0x14051b230, 0x140518d90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143191016, 0x142646960, 0x143cadb50, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_animation::system>
    Replacement::draft_throws(0x140517610, "bigfish_140517610", fn_140517610 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_reset_id::system>
    Replacement::draft_throws(0x1405178a0, "bigfish_1405178a0", tpl_4129::<0x143cadd38, 0x143cadf40, 0x1411e9f50, 0x14051d080, 0x143cade48, 0x1420af120, 0x1420a28e0, 0x14051b8a0, 0x1405192d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318fd66, 0x142646960, 0x143cadf60, 0x143cadf64, 0x143cadf66, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_reselect::system>
    Replacement::draft_throws(0x140517b90, "bigfish_140517b90", tpl_3005::<0x143cae15c, 0x143cae160, 0x1411e9f50, 0x1405193d0, 0x14051b9f0, 0x140519650, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318fee6, 0x142646960, 0x143cae180, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_barge::system>
    Replacement::draft_throws(0x140517dd0, "bigfish_140517dd0", tpl_3690::<0x143cade3c, 0x143cae250, 0x1411e9f50, 0x14051c910, 0x143cae074, 0x1420af120, 0x14051c810, 0x143cae034, 0x140215750, 0x1420a28e0, 0x14051bcd0, 0x1405199b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143190c06, 0x142646960, 0x143cae270, 0x143cae280, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_animation_fallback::system>
    Replacement::draft_throws(0x140518120, "bigfish_140518120", tpl_3062::<0x143cade38, 0x143cae040, 0x1411e9f50, 0x140519c00, 0x14051bee0, 0x140519dd0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318e6b6, 0x142646960, 0x143cae060, 0x143cae068, 0x143cae06c, 0x143cae06e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::process_picking_success::system>
    Replacement::draft_throws(0x140518380, "bigfish_140518380", tpl_1102::<0x143cadf00, 0x143cadf10, 0x1411e9f50, 0x14051c420, 0x143cade30, 0x1420af120, 0x1420a28e0, 0x14051c170, 0x14051a130, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14318f206, 0x142646960, 0x143cadf30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::door::update_interaction_state::system>
    Replacement::draft(0x140518660, "unknown_140518660", tpl_1095::<0x143cadde4, 0x14020e8f0> as *const ()), // sub_140518660
    Replacement::draft(0x140518840, "unknown_140518840", fn_140518840 as *const ()), // sub_140518840
    Replacement::draft_throws(0x140518be0, "unknown_140518be0", fn_140518be0 as *const (), Throws::Discard), // sub_140518be0
    Replacement::draft(0x140518d90, "unknown_140518d90", fn_140518d90 as *const ()), // sub_140518d90
    Replacement::draft(0x140518ef0, "unknown_140518ef0", fn_140518ef0 as *const ()), // sub_140518ef0
    Replacement::draft(0x140519070, "unknown_140519070", fn_140519070 as *const ()), // sub_140519070
    Replacement::draft(0x140519170, "unknown_140519170", fn_140519170 as *const ()), // sub_140519170
    Replacement::draft(0x1405192d0, "unknown_1405192d0", tpl_3136::<0x143c8de68, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141a7aef0> as *const ()), // sub_1405192d0
    Replacement::draft_throws(0x1405193d0, "unknown_1405193d0", fn_1405193d0 as *const (), Throws::Discard), // sub_1405193d0
    Replacement::draft_throws(0x140519650, "unknown_140519650", fn_140519650 as *const (), Throws::Discard), // sub_140519650
    Replacement::draft(0x140519730, "unknown_140519730", tpl_3251::<0x8, 0x20, 0x143cae314, 0x580e0, 0x143ca3714, 0x143cadea0, 0x143c97a34, 0x143c8f63c> as *const ()), // sub_140519730
    Replacement::draft_throws(0x140519830, "unknown_140519830", fn_140519830 as *const (), Throws::Discard), // sub_140519830
    Replacement::draft(0x1405199b0, "unknown_1405199b0", tpl_5034::<0x143cae034, 0x14051a830, 0x140519af0> as *const ()), // sub_1405199b0
    Replacement::draft(0x140519af0, "unknown_140519af0", tpl_5035::<0x18, 0x20, 0x40, 0x8, 0x141a7abd0> as *const ()), // sub_140519af0
    Replacement::draft_throws(0x140519c00, "unknown_140519c00", tpl_5076::<0x14051c530, 0x143cade9c, 0x1420af120, 0x14051b0a0, 0x14041d7c0, 0x143c9d21c, 0x140215750, 0x143c8ce6c, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_140519c00
    Replacement::draft(0x140519dd0, "unknown_140519dd0", fn_140519dd0 as *const ()), // sub_140519dd0
    Replacement::draft(0x140519ec0, "unknown_140519ec0", tpl_4372::<0x20, 0x143cade98, 0x8, 0x143c9d21c, 0x143c8ce6c, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_140519ec0
    Replacement::draft(0x140519fa0, "unknown_140519fa0", fn_140519fa0 as *const ()), // sub_140519fa0
    Replacement::draft(0x14051a130, "unknown_14051a130", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x14051a220, 0x580f0, 0x141a792e0> as *const ()), // sub_14051a130
    Replacement::draft(0x14051a220, "unknown_14051a220", tpl_5079::<0x8, 0x10, 0x18, 0x40, 0x143cadde4, 0x142489ee0, 0x143c9d9d0, 0xc03, 0x20, 0x28> as *const ()), // sub_14051a220
    Replacement::draft(0x14051a300, "unknown_14051a300", fn_14051a300 as *const ()), // sub_14051a300
    Replacement::draft(0x14051a3d0, "unknown_14051a3d0", fn_14051a3d0 as *const ()), // sub_14051a3d0
    Replacement::draft(0x14051a710, "unknown_14051a710", fn_14051a710 as *const ()), // sub_14051a710
    Replacement::draft(0x14051acd0, "unknown_14051acd0", fn_14051acd0 as *const ()), // sub_14051acd0
    Replacement::draft(0x14051ae00, "unknown_14051ae00", tpl_5084::<0x143c92a7c, 0x142489ee0, 0x143c92c98, 0x143c8d120, 0x143c9d9fc, 0x143c92ee8, 0x143c9d840, 0x143c937c8, 0x143c9d9d0, 0x143c93228, 0x143c933f0, 0x143c9d19c, 0x143c9e5cc> as *const ()), // sub_14051ae00
    Replacement::draft_throws(0x14051b0a0, "unknown_14051b0a0", tpl_1330::<0x8, 0x14051d170, 0x143cade98, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14051b0a0
    Replacement::draft_throws(0x14051b180, "unknown_14051b180", tpl_1331::<0x8, 0x14051d480, 0x143cae0c4, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_14051b180
    Replacement::draft(0x14051b230, "unknown_14051b230", fn_14051b230 as *const ()), // sub_14051b230
    Replacement::draft(0x14051b5b0, "unknown_14051b5b0", fn_14051b5b0 as *const ()), // sub_14051b5b0
    Replacement::draft(0x14051b8a0, "unknown_14051b8a0", tpl_62566::<0x143c8de68, 0x142489ee0, 0x141a7aef0> as *const ()), // sub_14051b8a0
    Replacement::draft_throws(0x14051b9f0, "unknown_14051b9f0", fn_14051b9f0 as *const (), Throws::Discard), // sub_14051b9f0
    Replacement::draft(0x14051bcd0, "unknown_14051bcd0", tpl_64228::<0x143cae034, 0x14051a830, 0x141a7abd0> as *const ()), // sub_14051bcd0
    Replacement::draft(0x14051bee0, "unknown_14051bee0", fn_14051bee0 as *const ()), // sub_14051bee0
    Replacement::draft(0x14051c170, "unknown_14051c170", fn_14051c170 as *const ()), // sub_14051c170
    Replacement::draft(0x14051c530, "unknown_14051c530", fn_14051c530 as *const ()), // sub_14051c530
    Replacement::draft(0x14051cb90, "unknown_14051cb90", fn_14051cb90 as *const ()), // sub_14051cb90
    Replacement::draft(0x14051ce30, "unknown_14051ce30", fn_14051ce30 as *const ()), // sub_14051ce30
    Replacement::draft(0x14051d170, "unknown_14051d170", tpl_3265::<0x143cadde8, 0x143cae1c0, 0x143c8d0d0, 0x143c8d120, 0x143c97150, 0x143c8d478, 0x143c919a8, 0x143c8e840, 0x143c8de68, 0x143c8dbc4, 0x143c97ad0, 0x143cae1e0, 0x143cae200, 0x143cae220, 0x141834a0c, 0x141834a78> as *const ()), // sub_14051d170
    Replacement::draft(0x14051d480, "unknown_14051d480", fn_14051d480 as *const ()), // sub_14051d480
    Replacement::draft(0x14051e0f0, "unknown_14051e0f0", fn_14051e0f0 as *const ()), // sub_14051e0f0
    Replacement::draft(0x14051e2a0, "unknown_14051e2a0", tpl_60443::<0x143c8cf5c, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x14051e35a> as *const ()), // sub_14051e2a0
    Replacement::draft_throws(0x14051e360, "bigfish_14051e360", tpl_1073::<0x143cae410, 0x143cae3e0, 0x1411e9f50, 0x1405216a0, 0x140523590, 0x140521890, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431940a6, 0x142646960, 0x143cae3f0, 0x143cae3f8, 0x143cae3fc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::start::system>
    Replacement::draft_throws(0x14051e5b0, "bigfish_14051e5b0", tpl_5090::<0x143cae6c8, 0x143cae588, 0x1411e9f50, 0x140525100, 0x143caea10, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x140523870, 0x140521ba0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431949c6, 0x142646960, 0x143cae5a8, 0x143cae5b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::update_perfect_dodge_stun::system>
    Replacement::draft_throws(0x14051e8c0, "bigfish_14051e8c0", tpl_3353::<0x143cae4c8, 0x143cae530, 0x1411e9f50, 0x140521d90, 0x140523a80, 0x140521f50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143194c86, 0x142646960, 0x143cae540, 0x143cae548, 0x143cae54c, 0x143cae54e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::update::system>
    Replacement::draft_throws(0x14051eb20, "bigfish_14051eb20", tpl_5091::<0x143cae744, 0x143cae928, 0x140520cf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143195316, 0x142646960, 0x143cae948, 0x143cae958, 0x143cae95c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::perfect_dodged_target_update::system>
    Replacement::draft_throws(0x14051ed30, "bigfish_14051ed30", tpl_5092::<0x143cae454, 0x143caeb18, 0x1411e9f50, 0x140524b90, 0x143cae520, 0x1420af120, 0x143cae9bc, 0x140215610, 0x1420a28e0, 0x140523fd0, 0x1405224d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143193936, 0x142646960, 0x143caeb38, 0x143caeb40, 0x143caeb42, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::slow_motion_update::system>
    Replacement::draft(0x14051f050, "bigfish_14051f050", tpl_1097::<0x143cae7c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14051f050
    Replacement::draft(0x14051f380, "bigfish_14051f380", tpl_1094::<0x143caeb44, 0x140520fb0, 0x14020e8f0> as *const ()), // sub_14051f380
    Replacement::draft(0x14051f580, "bigfish_14051f580", tpl_1097::<0x143caea0c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14051f580
    Replacement::draft(0x14051f8b0, "bigfish_14051f8b0", fn_14051f8b0 as *const ()), // sub_14051f8b0
    Replacement::draft(0x14051fa50, "bigfish_14051fa50", tpl_1436::<0x58, 0x60, 0x158, 0x143caeb44, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14051fa50
    Replacement::draft(0x14051fb30, "bigfish_14051fb30", tpl_1094::<0x143cae688, 0x140521140, 0x14020e8f0> as *const ()), // sub_14051fb30
    Replacement::draft(0x14051fd30, "bigfish_14051fd30", tpl_1466::<0x143cae524, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14051fd30
    Replacement::draft(0x140520000, "bigfish_140520000", tpl_1094::<0x143cae5c4, 0x140521300, 0x14020e8f0> as *const ()), // sub_140520000
    Replacement::draft(0x140520200, "bigfish_140520200", tpl_1097::<0x143c9d6c4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140520200
    Replacement::draft(0x140520530, "bigfish_140520530", tpl_1094::<0x143c97794, 0x140521520, 0x14020e8f0> as *const ()), // sub_140520530
    Replacement::draft_throws(0x140520730, "bigfish_140520730", tpl_3682::<0x143caea08, 0x143cae7d0, 0x1411e9f50, 0x140525810, 0x143cae868, 0x1420af120, 0x1420a28e0, 0x140524890, 0x1405233b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143193ae6, 0x142646960, 0x143cae7f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::stream_in_enemy::system>
    Replacement::draft_throws(0x140520a10, "bigfish_140520a10", tpl_3683::<0x143cae960, 0x143cae558, 0x1411e9f50, 0x1405256d0, 0x143cae834, 0x1420af120, 0x1420a28e0, 0x140524ab0, 0x1405234d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143193df6, 0x142646960, 0x143cae578, 0x143cae580, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dodge::stream_out_enemy::system>
    Replacement::draft_throws(0x140520cf0, "unknown_140520cf0", tpl_1187::<0x1411e9f50, 0x140524d70, 0x143caeb48, 0x1420af120, 0x140524c80, 0x143cae864, 0x140215750, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x140523d70, 0x140522280> as *const (), Throws::Discard), // sub_140520cf0
    Replacement::draft(0x140520f40, "unknown_140520f40", tpl_5094::<0x8, 0x14051fb30, 0x14051fd30> as *const ()), // sub_140520f40
    Replacement::draft(0x140520fb0, "unknown_140520fb0", tpl_4469::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1405231c0> as *const ()), // sub_140520fb0
    Replacement::draft(0x140521140, "unknown_140521140", fn_140521140 as *const ()), // sub_140521140
    Replacement::draft(0x140521300, "unknown_140521300", fn_140521300 as *const ()), // sub_140521300
    Replacement::draft(0x140521520, "unknown_140521520", fn_140521520 as *const ()), // sub_140521520
    Replacement::draft_throws(0x1405216a0, "unknown_1405216a0", tpl_1199::<0x140525450, 0x143caeab4, 0x1420af120, 0x140525240, 0x143cae51c, 0x140215750, 0x143c8d910, 0x140215610, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405216a0
    Replacement::draft(0x140521890, "unknown_140521890", fn_140521890 as *const ()), // sub_140521890
    Replacement::draft(0x140521970, "unknown_140521970", tpl_3114::<0x143cae51c, 0x20, 0x8, 0x143c8d910, 0x580e0, 0x143c8ceec, 0x142407c80, 0x580f0, 0x10, 0x18, 0x28> as *const ()), // sub_140521970
    Replacement::draft(0x140521a30, "unknown_140521a30", fn_140521a30 as *const ()), // sub_140521a30
    Replacement::draft_throws(0x140521ba0, "unknown_140521ba0", fn_140521ba0 as *const (), Throws::Discard), // sub_140521ba0
    Replacement::draft_throws(0x140521ca0, "unknown_140521ca0", tpl_5101::<0x18, 0x20, 0x40, 0x8, 0x141a801b0> as *const (), Throws::Discard), // sub_140521ca0
    Replacement::draft_throws(0x140521d90, "unknown_140521d90", fn_140521d90 as *const (), Throws::Discard), // sub_140521d90
    Replacement::draft(0x140521f50, "unknown_140521f50", fn_140521f50 as *const ()), // sub_140521f50
    Replacement::draft(0x140522030, "unknown_140522030", tpl_5104::<0x143c9c108, 0x20, 0x8, 0x143c8cef8, 0x580e0, 0x143c8d910, 0x143c8f63c, 0x143c8ceec, 0x142407c80, 0x580f0, 0x10, 0x18> as *const ()), // sub_140522030
    Replacement::draft(0x140522110, "unknown_140522110", fn_140522110 as *const ()), // sub_140522110
    Replacement::draft(0x140522280, "unknown_140522280", fn_140522280 as *const ()), // sub_140522280
    Replacement::draft(0x140522350, "unknown_140522350", tpl_3709::<0x143cae864, 0x20, 0x8, 0x143c8d910, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_140522350
    Replacement::draft(0x1405223f0, "unknown_1405223f0", tpl_5107::<0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x141a7e780> as *const ()), // sub_1405223f0
    Replacement::draft(0x1405224d0, "unknown_1405224d0", tpl_64057::<0x88, 0x10088, 0x143cae9bc, 0x580e0, 0x140522660, 0x141a803e0> as *const ()), // sub_1405224d0
    Replacement::draft(0x1405225d0, "content_1405225d0", tpl_5108::<0x1401f2bc0, 0x143084e28, 0x141834d30> as *const ()), // content::PerfectDodgeTargetComponent::content::PerfectDodgeTargetComponent::vfunc_0
    Replacement::draft(0x140522620, "content_140522620", fn_140522620 as *const ()), // content::AbilityDodgeComponent::content::AbilityDodgeComponent::vfunc_0
    Replacement::draft(0x1405231c0, "unknown_1405231c0", fn_1405231c0 as *const ()), // sub_1405231c0
    Replacement::draft(0x140523350, "unknown_140523350", tpl_1124::<0x70> as *const ()), // sub_140523350
    Replacement::draft(0x140523380, "unknown_140523380", tpl_3324::<0x100, 0xe0> as *const ()), // sub_140523380
    Replacement::draft(0x1405233b0, "unknown_1405233b0", tpl_1113::<0x88, 0x10088, 0x142407c80, 0x140524570, 0x580f0, 0x141a7e560> as *const ()), // sub_1405233b0
    Replacement::draft(0x1405234d0, "unknown_1405234d0", fn_1405234d0 as *const ()), // sub_1405234d0
    Replacement::draft(0x140523590, "unknown_140523590", fn_140523590 as *const ()), // sub_140523590
    Replacement::draft_throws(0x140523870, "unknown_140523870", fn_140523870 as *const (), Throws::Discard), // sub_140523870
    Replacement::draft(0x140523a80, "unknown_140523a80", fn_140523a80 as *const ()), // sub_140523a80
    Replacement::draft(0x140523d70, "unknown_140523d70", fn_140523d70 as *const ()), // sub_140523d70
    Replacement::draft(0x140523fd0, "unknown_140523fd0", tpl_64074::<0x143cae9bc, 0x143cae5c4, 0x142489ee0, 0x143c97794, 0x141a803e0> as *const ()), // sub_140523fd0
    Replacement::draft(0x140524890, "unknown_140524890", tpl_1128::<0x142407c80, 0x143c8e840, 0x142489ee0, 0x143c8de68, 0x143cae688, 0x141a7e560> as *const ()), // sub_140524890
    Replacement::draft(0x140524ab0, "unknown_140524ab0", tpl_5111::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14051fa50> as *const ()), // sub_140524ab0
    Replacement::draft(0x140524e80, "unknown_140524e80", fn_140524e80 as *const ()), // sub_140524e80
    Replacement::draft(0x140525240, "unknown_140525240", fn_140525240 as *const ()), // sub_140525240
    Replacement::draft(0x140525450, "unknown_140525450", fn_140525450 as *const ()), // sub_140525450
    Replacement::draft(0x140525d80, "unknown_140525d80", fn_140525d80 as *const ()), // sub_140525d80
    Replacement::draft(0x140525fa0, "unknown_140525fa0", fn_140525fa0 as *const ()), // sub_140525fa0
    Replacement::draft(0x140526050, "unknown_140526050", tpl_1094::<0x143c8e83c, 0x140526280, 0x14020e8f0> as *const ()), // sub_140526050
    Replacement::draft(0x140526250, "unknown_140526250", tpl_1439::<0x8, 0x140526050> as *const ()), // sub_140526250
    Replacement::draft(0x140526280, "unknown_140526280", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143088f50> as *const ()), // sub_140526280
    Replacement::draft(0x140526400, "unknown_140526400", fn_140526400 as *const ()), // sub_140526400
    Replacement::draft(0x140526420, "unknown_140526420", fn_140526420 as *const ()), // sub_140526420
    Replacement::draft(0x140526610, "content_140526610", fn_140526610 as *const ()), // content::DialogueControllerComponent::content::DialogueControllerComponent::vfunc_18
    Replacement::draft(0x140526850, "unknown_140526850", fn_140526850 as *const ()), // sub_140526850
    Replacement::draft(0x140526900, "unknown_140526900", tpl_5038::<0x143caef00, 0x142489ee0, 0x143caef10, 0x143c8d160, 0x143c8d120, 0x143c8d87c, 0x143c8fab8, 0x143c8d0d0> as *const ()), // sub_140526900
    Replacement::draft(0x140526aa0, "unknown_140526aa0", fn_140526aa0 as *const ()), // sub_140526aa0
    Replacement::draft_throws(0x140526bd0, "bigfish_140526bd0", tpl_3395::<0x143caef70, 0x143caec88, 0x1405274f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143196096, 0x142646960, 0x143caeca8, 0x143caecb0, 0x143caecb2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dialogue::updateDialogueVariables>
    Replacement::draft(0x140526de0, "bigfish_140526de0", fn_140526de0 as *const ()), // sub_140526de0
    Replacement::draft(0x140527120, "bigfish_140527120", tpl_1436::<0x58, 0x60, 0x158, 0x143caef00, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140527120
    Replacement::draft(0x140527200, "bigfish_140527200", fn_140527200 as *const ()), // sub_140527200
    Replacement::draft_throws(0x1405274f0, "bigfish_1405274f0", fn_1405274f0 as *const (), Throws::Discard), // sub_1405274f0
    Replacement::draft_throws(0x140527760, "bigfish_140527760", fn_140527760 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dialogue::streamIn>
    Replacement::draft_throws(0x140527a70, "bigfish_140527a70", tpl_5126::<0x143caecb4, 0x143caef18, 0x1411e9f50, 0x140528c30, 0x143caec78, 0x1420af120, 0x1420a28e0, 0x1405289a0, 0x140528340, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143195b46, 0x142646960, 0x143caef28, 0x143caef30, 0x143caef34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dialogue::streamOut>
    Replacement::draft(0x140527d50, "unknown_140527d50", fn_140527d50 as *const ()), // sub_140527d50
    Replacement::draft(0x140528010, "unknown_140528010", fn_140528010 as *const ()), // sub_140528010
    Replacement::draft_throws(0x1405280f0, "unknown_1405280f0", tpl_5128::<0x143caecf4, 0x20, 0x8, 0x141a83880> as *const (), Throws::Discard), // sub_1405280f0
    Replacement::draft_throws(0x140528150, "unknown_140528150", fn_140528150 as *const (), Throws::Caught), // sub_140528150
    Replacement::draft_throws(0x140528250, "unknown_140528250", tpl_1469::<0x18, 0x20, 0x40, 0x8, 0x48, 0x141a846f0> as *const (), Throws::Discard), // sub_140528250
    Replacement::draft(0x140528340, "unknown_140528340", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140527120> as *const ()), // sub_140528340
    Replacement::draft(0x140529170, "unknown_140529170", fn_140529170 as *const ()), // sub_140529170
    Replacement::draft(0x140529270, "unknown_140529270", fn_140529270 as *const ()), // sub_140529270
    Replacement::draft(0x1405293c0, "unknown_1405293c0", fn_1405293c0 as *const ()), // sub_1405293c0
    Replacement::draft(0x1405294c0, "content_1405294c0", fn_1405294c0 as *const ()), // content::DialogueControllerComponent::content::DialogueControllerComponent::vfunc_0
    Replacement::draft(0x140529530, "unknown_140529530", fn_140529530 as *const ()), // sub_140529530
    Replacement::draft_throws(0x140529620, "bigfish_140529620", fn_140529620 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::destruction_events::launchPendingEvents>
    Replacement::draft(0x1405299b0, "unknown_1405299b0", fn_1405299b0 as *const ()), // sub_1405299b0
    Replacement::draft(0x140529be0, "unknown_140529be0", fn_140529be0 as *const ()), // sub_140529be0
    Replacement::draft(0x140529e20, "unknown_140529e20", fn_140529e20 as *const ()), // sub_140529e20
    Replacement::draft(0x140529f40, "unknown_140529f40", fn_140529f40 as *const ()), // sub_140529f40
    Replacement::draft(0x14052a0e0, "unknown_14052a0e0", fn_14052a0e0 as *const ()), // sub_14052a0e0
    Replacement::draft(0x14052a190, "unknown_14052a190", tpl_5015::<0x143caf094, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14052a190
    Replacement::draft(0x14052a500, "unknown_14052a500", tpl_1095::<0x143caf200, 0x14020e8f0> as *const ()), // sub_14052a500
    Replacement::draft(0x14052a6e0, "unknown_14052a6e0", tpl_1403::<0x143caf140, 0x14052a8c0, 0x14020e8f0> as *const ()), // sub_14052a6e0
    Replacement::draft(0x14052a8c0, "unknown_14052a8c0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143197a78> as *const ()), // sub_14052a8c0
    Replacement::draft(0x14052b080, "unknown_14052b080", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143caf55c, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_14052b080
    Replacement::draft(0x14052b170, "unknown_14052b170", fn_14052b170 as *const ()), // sub_14052b170
    Replacement::draft(0x14052b340, "unknown_14052b340", tpl_60443::<0x143caf55c, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x14052b3fa> as *const ()), // sub_14052b340
    Replacement::draft_throws(0x14052b400, "bigfish_14052b400", tpl_4527::<0x143caf558, 0x143caf440, 0x1411e9f50, 0x14052d840, 0x14052df70, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143198936, 0x142646960, 0x143caf460, 0x143caf470, 0x143caf478, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::deduction_prompt::state::updateUIAssetStreaming>
    Replacement::draft_throws(0x14052b650, "bigfish_14052b650", tpl_1480::<0x142756180, 0x1410d6ca0, 0x14052c9d0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14052b650
    Replacement::draft_throws(0x14052b8f0, "bigfish_14052b8f0", tpl_1480::<0x142756180, 0x1410d6ca0, 0x14052cce0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14052b8f0
    Replacement::draft(0x14052bb90, "bigfish_14052bb90", tpl_1526::<0x142756180, 0x1410d6ca0, 0x14052cf30, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14052bb90
    Replacement::draft_throws(0x14052be30, "bigfish_14052be30", tpl_1480::<0x142756180, 0x1410d6ca0, 0x14052d190, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14052be30
    Replacement::draft(0x14052c0d0, "bigfish_14052c0d0", tpl_1526::<0x142756180, 0x1410d6ca0, 0x14052d3e0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14052c0d0
    Replacement::draft(0x14052c370, "bigfish_14052c370", fn_14052c370 as *const ()), // sub_14052c370
    Replacement::draft_throws(0x14052c490, "bigfish_14052c490", tpl_5147::<0x8, 0x14240ab10, 0x14241c190> as *const (), Throws::Discard), // sub_14052c490
    Replacement::draft(0x14052c4f0, "bigfish_14052c4f0", fn_14052c4f0 as *const ()), // sub_14052c4f0
    Replacement::draft(0x14052c700, "bigfish_14052c700", tpl_5148::<0xc, 0x8, 0x1403f3e60, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_14052c700
    Replacement::draft(0x14052c7d0, "bigfish_14052c7d0", tpl_1094::<0x143caf55c, 0x14052d6c0, 0x14020e8f0> as *const ()), // sub_14052c7d0
    Replacement::draft_throws(0x14052c9d0, "bigfish_14052c9d0", fn_14052c9d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::deduction_prompt::state::setupNotification>
    Replacement::draft_throws(0x14052cce0, "bigfish_14052cce0", tpl_4193::<0x143caf434, 0x143caf400, 0x1411e9f50, 0x14052e020, 0x14052e760, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431983a6, 0x142646960, 0x143caf420, 0x143caf430, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::deduction_prompt::state::syncDeductionData>
    Replacement::draft(0x14052cf30, "bigfish_14052cf30", tpl_1538::<0x143caf384, 0x143caf388, 0x1411e9f50, 0x14052e1f0, 0x14052e880, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143198656, 0x142646960, 0x143caf3a8, 0x143caf3b0, 0x143caf3b4, 0x143caf3b6, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::deduction_prompt::state::updateTimers>
    Replacement::draft_throws(0x14052d190, "bigfish_14052d190", tpl_3455::<0x143caf350, 0x143caf480, 0x1411e9f50, 0x14052e330, 0x14052e8c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143198016, 0x142646960, 0x143caf4a0, 0x143caf4b0, 0x143caf4b4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::deduction_prompt::state::handleFlushRequest>
    Replacement::draft(0x14052d3e0, "bigfish_14052d3e0", fn_14052d3e0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::deduction_prompt::state::handleCleanupRequest>
    Replacement::draft_throws(0x14052d600, "unknown_14052d600", fn_14052d600 as *const (), Throws::Discard), // sub_14052d600
    Replacement::draft(0x14052d6c0, "unknown_14052d6c0", tpl_5022::<0x1401e6d50, 0x14052df10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14052dc00> as *const ()), // sub_14052d6c0
    Replacement::draft_throws(0x14052d840, "unknown_14052d840", fn_14052d840 as *const (), Throws::Discard), // sub_14052d840
    Replacement::draft(0x14052da80, "unknown_14052da80", tpl_1765::<0x1411e9f50, 0x143c9ad1c, 0x140215610, 0x143caf360, 0x143c8ce88, 0x1420a28e0, 0x14052e990, 0x1401de410> as *const ()), // sub_14052da80
    Replacement::draft(0x14052dc00, "unknown_14052dc00", fn_14052dc00 as *const ()), // sub_14052dc00
    Replacement::draft(0x14052dc80, "unknown_14052dc80", fn_14052dc80 as *const ()), // sub_14052dc80
    Replacement::draft(0x14052df10, "unknown_14052df10", fn_14052df10 as *const ()), // sub_14052df10
    Replacement::draft_throws(0x14052df70, "unknown_14052df70", tpl_5156::<0x143caf33c, 0x143c90260, 0x143c8ce6c, 0x20, 0x143c9ad1c, 0x8, 0x143c8ce88, 0x143caf360, 0x141a882f0> as *const (), Throws::Discard), // sub_14052df70
    Replacement::draft_throws(0x14052e020, "unknown_14052e020", tpl_5157::<0x14052e600, 0x143c8d110, 0x140215610, 0x143c90260, 0x143c9ad1c, 0x143c9b0d0, 0x143caf360, 0x143c8ce88, 0x1420a28e0> as *const (), Throws::Discard), // sub_14052e020
    Replacement::draft(0x14052e1f0, "unknown_14052e1f0", fn_14052e1f0 as *const ()), // sub_14052e1f0
    Replacement::draft_throws(0x14052e330, "unknown_14052e330", tpl_5157::<0x14052e600, 0x143c8d110, 0x140215610, 0x143c9b0d0, 0x143c90260, 0x143c9ad1c, 0x143caf360, 0x143c8ce88, 0x1420a28e0> as *const (), Throws::Discard), // sub_14052e330
    Replacement::draft_throws(0x14052e600, "unknown_14052e600", tpl_1330::<0x8, 0x14052eca0, 0x143caf4d8, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14052e600
    Replacement::draft(0x14052e6e0, "unknown_14052e6e0", tpl_3642::<0x143caf3b8, 0x20, 0x8, 0x142407c80, 0x580f0, 0x141a88040> as *const ()), // sub_14052e6e0
    Replacement::draft_throws(0x14052e760, "unknown_14052e760", fn_14052e760 as *const (), Throws::Caught), // sub_14052e760
    Replacement::draft_throws(0x14052e8c0, "unknown_14052e8c0", fn_14052e8c0 as *const (), Throws::Discard), // sub_14052e8c0
    Replacement::draft_throws(0x14052e990, "unknown_14052e990", fn_14052e990 as *const (), Throws::Discard), // sub_14052e990
    Replacement::draft(0x14052eac0, "unknown_14052eac0", fn_14052eac0 as *const ()), // sub_14052eac0
    Replacement::draft_throws(0x14052f030, "unknown_14052f030", fn_14052f030 as *const (), Throws::Caught), // sub_14052f030
    Replacement::draft(0x14052f300, "bigfish_14052f300", tpl_5163::<0x100, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14052f390> as *const ()), // sub_14052f300
    Replacement::draft(0x14052f390, "bigfish_14052f390", tpl_5164::<0x143199008, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x143222d60, 0x38, 0xb8, 0x40, 0x48> as *const ()), // sub_14052f390
    Replacement::draft(0x14052f440, "bigfish_14052f440", tpl_64337::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x14052f390> as *const ()), // sub_14052f440
    Replacement::draft_throws(0x14052f690, "bigfish_14052f690", fn_14052f690 as *const (), Throws::Caught), // sub_14052f690
    Replacement::draft_throws(0x14052f820, "bigfish_14052f820", tpl_1905::<0x14387b970, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14052f820
    Replacement::draft(0x14052f8d0, "bigfish_14052f8d0", fn_14052f8d0 as *const ()), // bigfish::DecoratorCheckAttackCategoriesInstance::bigfish::DecoratorCheckAttackCategoriesInstance::vfunc_0
    Replacement::draft(0x14052f910, "unknown_14052f910", fn_14052f910 as *const ()), // sub_14052f910
    Replacement::draft(0x14052f984, "unknown_14052f984", tpl_5167::<0x108> as *const ()), // sub_14052f984
    Replacement::draft(0x14052f9d0, "unknown_14052f9d0", fn_14052f9d0 as *const ()), // sub_14052f9d0
    Replacement::draft(0x14052f9e0, "unknown_14052f9e0", tpl_1403::<0x143caf610, 0x14052fed0, 0x14020e8f0> as *const ()), // sub_14052f9e0
    Replacement::draft_throws(0x14052fbf0, "bigfish_14052fbf0", tpl_1437::<0x143caf64c, 0x143caf698, 0x1411e9f50, 0x1405303b0, 0x143caf650, 0x1420af120, 0x1420a28e0, 0x140530250, 0x140530050, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431993d6, 0x142646960, 0x143caf6b8, 0x143caf6c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::dark_presence::stream_in::system>
    Replacement::draft(0x14052fed0, "unknown_14052fed0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143199578> as *const ()), // sub_14052fed0
    Replacement::draft(0x140530050, "unknown_140530050", fn_140530050 as *const ()), // sub_140530050
    Replacement::draft(0x140530250, "unknown_140530250", fn_140530250 as *const ()), // sub_140530250
    Replacement::draft(0x140530550, "content_140530550", fn_140530550 as *const ()), // content::DarknessShieldComponent::content::DarknessShieldComponent::vfunc_20
    Replacement::draft(0x140530710, "unknown_140530710", tpl_61197::<0x143c914b0, 0x142489ee0, 0x143c954e4, 0x143c8d160, 0x143c9fde0, 0x1426467e8, 0x1405308a3> as *const ()), // sub_140530710
    Replacement::draft(0x1405308b0, "unknown_1405308b0", fn_1405308b0 as *const ()), // sub_1405308b0
    Replacement::draft(0x140530970, "unknown_140530970", fn_140530970 as *const ()), // sub_140530970
    Replacement::draft(0x140530ab0, "unknown_140530ab0", tpl_61019::<0x143c914b0, 0x142489ee0, 0x143c954e4, 0x143c8d160, 0x1426467e8, 0x140530c13> as *const ()), // sub_140530ab0
    Replacement::draft(0x140530c20, "unknown_140530c20", tpl_1171::<0x58, 0x60, 0x143c9626c, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140530c20
    Replacement::draft(0x140530e50, "unknown_140530e50", tpl_61019::<0x143c8d160, 0x142489ee0, 0x143c9626c, 0x143caf9bc, 0x1426467e8, 0x140530fb3> as *const ()), // sub_140530e50
    Replacement::draft_throws(0x140530fc0, "bigfish_140530fc0", tpl_5172::<0x143caff1c, 0x143caf840, 0x140533770, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a0986, 0x142646960, 0x143caf860, 0x143caf870, 0x143caf878, 0x143caf87c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::attachment_entity_spawning::system>
    Replacement::draft_throws(0x1405311e0, "bigfish_1405311e0", tpl_4201::<0x143caf8c8, 0x143caf9c8, 0x1411e9f50, 0x140534ec0, 0x1405391a0, 0x140535270, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a0296, 0x142646960, 0x143caf9e8, 0x143caf9f8, 0x143cafa00, 0x143cafa02, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::bullet_raycast_filtering::system>
    Replacement::draft_throws(0x140531440, "bigfish_140531440", tpl_4111::<0x143cafa40, 0x143cb00f0, 0x1411e9f50, 0x1405355f0, 0x1405393c0, 0x1405357e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319fe96, 0x142646960, 0x143cb0110, 0x143cb0120, 0x143cb0124, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::source_point_damage::system>
    Replacement::draft_throws(0x140531690, "bigfish_140531690", tpl_3002::<0x143cb06b8, 0x143cb08f0, 0x1411e9f50, 0x140535a60, 0x140539600, 0x140535d70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319f586, 0x142646960, 0x143cb0910, 0x143cb0920, 0x143cb0924, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::focal_point_damage::system>
    Replacement::draft_throws(0x1405318e0, "bigfish_1405318e0", tpl_5173::<0x143cb092c, 0x143cb0420, 0x1405339a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319ef16, 0x142646960, 0x143cb0440, 0x143cb0450, 0x143cb0458, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::focal_point_regenerate::system>
    Replacement::draft_throws(0x140531af0, "bigfish_140531af0", tpl_1291::<0x143caf998, 0x143cb0530, 0x1411e9f50, 0x1405363b0, 0x140539b20, 0x140536580, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319e8c6, 0x142646960, 0x143cb0550, 0x143cb0558, 0x143cb055c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::enemy_death::system>
    Replacement::draft_throws(0x140531d40, "bigfish_140531d40", fn_140531d40 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::attachment_transform_update::system>
    Replacement::draft_throws(0x140531f60, "bigfish_140531f60", tpl_5175::<0x143cafc40, 0x143cb0728, 0x140533d90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319def6, 0x142646960, 0x143cb0748, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::flashlight_healing::system>
    Replacement::draft_throws(0x140532160, "bigfish_140532160", fn_140532160 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::chroma_flashing_burning::system>
    Replacement::draft_throws(0x1405323d0, "bigfish_1405323d0", fn_1405323d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::attachment_visibility_update::system>
    Replacement::draft_throws(0x140532600, "bigfish_140532600", tpl_3066::<0x143caf748, 0x143cafa10, 0x1411e9f50, 0x14053bdb0, 0x143cb0520, 0x1420af120, 0x143c8ceec, 0x140215610, 0x14036e0c0, 0x1420a28e0, 0x14053a510, 0x140536f70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319d396, 0x142646960, 0x143cafa30, 0x143cafa38, 0x143cafa3c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::stream_out::system>
    Replacement::draft_throws(0x140532940, "bigfish_140532940", fn_140532940 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::stream_out::darknessShieldRemoved>
    Replacement::draft_throws(0x140532ba0, "bigfish_140532ba0", fn_140532ba0 as *const (), Throws::Caught), // sub_140532ba0
    Replacement::draft(0x140532ee0, "bigfish_140532ee0", tpl_1094::<0x143c9626c, 0x140534730, 0x14020e8f0> as *const ()), // sub_140532ee0
    Replacement::draft(0x1405330e0, "bigfish_1405330e0", tpl_1094::<0x143caf9bc, 0x1405348b0, 0x14020e8f0> as *const ()), // sub_1405330e0
    Replacement::draft_throws(0x1405332e0, "bigfish_1405332e0", fn_1405332e0 as *const (), Throws::Caught), // sub_1405332e0
    Replacement::draft(0x140533570, "bigfish_140533570", tpl_1094::<0x143cafe00, 0x140534a30, 0x14020e8f0> as *const ()), // sub_140533570
    Replacement::draft_throws(0x140533770, "bigfish_140533770", tpl_3083::<0x1411e9f50, 0x14053d7f0, 0x143cb02f8, 0x1420af120, 0x14053d6e0, 0x143cb0790, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x140538f70, 0x140534c20> as *const (), Throws::Discard), // sub_140533770
    Replacement::draft_throws(0x1405339a0, "bigfish_1405339a0", fn_1405339a0 as *const (), Throws::Discard), // sub_1405339a0
    Replacement::draft_throws(0x140533b90, "bigfish_140533b90", tpl_5182::<0x1411e9f50, 0x14053c510, 0x143cb0934, 0x1420af120, 0x14053c420, 0x143cb078c, 0x140215750, 0x14036fad0, 0x14036e720, 0x1420a28e0, 0x140539dd0, 0x140536840> as *const (), Throws::Discard), // sub_140533b90
    Replacement::draft_throws(0x140533d90, "bigfish_140533d90", tpl_1405::<0x1411e9f50, 0x14053c320, 0x143cb093c, 0x1420af120, 0x140537980, 0x143c8d910, 0x140215610, 0x143c94b38, 0x1420a28e0, 0x14053a010, 0x140536bb0> as *const (), Throws::Discard), // sub_140533d90
    Replacement::draft_throws(0x140533f60, "bigfish_140533f60", tpl_3083::<0x1411e9f50, 0x14053c030, 0x143cb03dc, 0x1420af120, 0x14053bef0, 0x143cb0128, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14053a2c0, 0x140536d30> as *const (), Throws::Discard), // sub_140533f60
    Replacement::draft_throws(0x140534190, "bigfish_140534190", tpl_4804::<0x143cb012c, 0x143cb08c0, 0x140537150, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14319cd46, 0x142646960, 0x143cb08e0, 0x143cb08e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_shield::stream_in::system>
    Replacement::draft(0x1405343d0, "unknown_1405343d0", tpl_60530::<0x1426467e8, 0x143c8ec20, 0x142489ee0, 0x143c8d120, 0x1405343f9> as *const ()), // sub_1405343d0
    Replacement::draft(0x140534590, "unknown_140534590", tpl_61197::<0x143c914b0, 0x142489ee0, 0x143c954e4, 0x143c8d160, 0x143c95ed0, 0x1426467e8, 0x140534723> as *const ()), // sub_140534590
    Replacement::draft(0x140534730, "unknown_140534730", fn_140534730 as *const ()), // sub_140534730
    Replacement::draft(0x1405348b0, "unknown_1405348b0", fn_1405348b0 as *const ()), // sub_1405348b0
    Replacement::draft(0x140534a30, "unknown_140534a30", fn_140534a30 as *const ()), // sub_140534a30
    Replacement::draft(0x140534c20, "unknown_140534c20", fn_140534c20 as *const ()), // sub_140534c20
    Replacement::draft(0x140534cf0, "unknown_140534cf0", tpl_3709::<0x143cb0790, 0x20, 0x8, 0x143c8ceec, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_140534cf0
    Replacement::draft(0x140534d90, "unknown_140534d90", tpl_1780::<0x18, 0x20, 0x40, 0x8, 0x10, 0x141a8e070> as *const ()), // sub_140534d90
    Replacement::draft_throws(0x140534ec0, "unknown_140534ec0", fn_140534ec0 as *const (), Throws::Discard), // sub_140534ec0
    Replacement::draft_throws(0x140535270, "unknown_140535270", tpl_3628::<0x88, 0x10088, 0x140535380, 0x143caf9ac, 0x180ac, 0x18098, 0x142489ee0, 0xc03, 0x40, 0x1405354b0> as *const (), Throws::Discard), // sub_140535270
    Replacement::draft(0x140535380, "unknown_140535380", tpl_3023::<0x8, 0x580e0, 0x20, 0x143caf9b0, 0x143c8eb9c, 0x143cb01ec, 0x143cafd5c, 0x143c8f63c, 0x143c8d00c, 0x143c8ceec, 0x143c8de64> as *const ()), // sub_140535380
    Replacement::draft_throws(0x1405354b0, "unknown_1405354b0", fn_1405354b0 as *const (), Throws::Discard), // sub_1405354b0
    Replacement::draft_throws(0x1405355f0, "unknown_1405355f0", tpl_5189::<0x14053d190, 0x143cafd64, 0x1420af120, 0x14053d0a0, 0x143caf99c, 0x140215750, 0x143c94b34, 0x140215610, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405355f0
    Replacement::draft(0x1405357e0, "unknown_1405357e0", fn_1405357e0 as *const ()), // sub_1405357e0
    Replacement::draft(0x140535940, "unknown_140535940", fn_140535940 as *const ()), // sub_140535940
    Replacement::draft_throws(0x140535a60, "unknown_140535a60", fn_140535a60 as *const (), Throws::Discard), // sub_140535a60
    Replacement::draft(0x140535d70, "unknown_140535d70", fn_140535d70 as *const ()), // sub_140535d70
    Replacement::draft(0x140535e50, "unknown_140535e50", fn_140535e50 as *const ()), // sub_140535e50
    Replacement::draft(0x140535f70, "unknown_140535f70", fn_140535f70 as *const ()), // sub_140535f70
    Replacement::draft_throws(0x1405360e0, "unknown_1405360e0", fn_1405360e0 as *const (), Throws::Discard), // sub_1405360e0
    Replacement::draft_throws(0x140536290, "unknown_140536290", fn_140536290 as *const (), Throws::Discard), // sub_140536290
    Replacement::draft_throws(0x1405363b0, "unknown_1405363b0", tpl_5076::<0x14053c7d0, 0x143cb097c, 0x1420af120, 0x140537f00, 0x14053c690, 0x143cafa08, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405363b0
    Replacement::draft(0x140536580, "unknown_140536580", fn_140536580 as *const ()), // sub_140536580
    Replacement::draft(0x140536650, "unknown_140536650", tpl_4372::<0x20, 0x143caf9a4, 0x8, 0x143cafa08, 0x143c8ceec, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_140536650
    Replacement::draft(0x140536730, "unknown_140536730", tpl_5199::<0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x38, 0x141a8ed60> as *const ()), // sub_140536730
    Replacement::draft(0x140536840, "unknown_140536840", tpl_5200::<0x143cb078c, 0x143c94b44, 0x143c94b30, 0x140537a60, 0x140536a80> as *const ()), // sub_140536840
    Replacement::draft(0x1405369a0, "unknown_1405369a0", tpl_5201::<0x8, 0x20, 0x143cb078c, 0x143c94b44, 0x143c94b30> as *const ()), // sub_1405369a0
    Replacement::draft(0x140536a80, "unknown_140536a80", tpl_5202::<0x18, 0x20, 0x40, 0x10, 0x28, 0x48, 0x30, 0x8, 0x141a8e860> as *const ()), // sub_140536a80
    Replacement::draft(0x140536bb0, "unknown_140536bb0", tpl_64107::<0x143cb0928, 0x143c8d910, 0x143c94b38, 0x140537880, 0x141a8c810> as *const ()), // sub_140536bb0
    Replacement::draft_throws(0x140536d30, "unknown_140536d30", fn_140536d30 as *const (), Throws::Discard), // sub_140536d30
    Replacement::draft_throws(0x140536e40, "unknown_140536e40", fn_140536e40 as *const (), Throws::Discard), // sub_140536e40
    Replacement::draft(0x140536f70, "unknown_140536f70", fn_140536f70 as *const ()), // sub_140536f70
    Replacement::draft(0x1405370b0, "unknown_1405370b0", fn_1405370b0 as *const ()), // sub_1405370b0
    Replacement::draft_throws(0x140537150, "unknown_140537150", tpl_1778::<0x1411e9f50, 0x14053db90, 0x143cafb08, 0x1420af120, 0x1404c2240, 0x143c8ceec, 0x140215610, 0x143c8d08c, 0x1420a28e0, 0x14053b9a0, 0x14053a700> as *const (), Throws::Discard), // sub_140537150
    Replacement::draft_throws(0x140537320, "unknown_140537320", tpl_4530::<0x14053aac0, 0x143caf9bc, 0x1401fc320, 0x14053bc20, 0x143cb01f0, 0x1420af120, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_140537320
    Replacement::draft(0x1405374c0, "content_1405374c0", fn_1405374c0 as *const ()), // content::DarknessShieldComponent::content::DarknessShieldComponent::vfunc_0
    Replacement::draft(0x140537620, "unknown_140537620", tpl_4925::<0x8, 0x10, 0x18, 0x1405376d0, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50> as *const ()), // sub_140537620
    Replacement::draft(0x1405376d0, "unknown_1405376d0", tpl_2541::<0x143cb0938, 0x142489ee0, 0x143c95ed0, 0x143c914b8, 0x143c8d160, 0x143c914b0, 0x143c954e4, 0x143c8d87c> as *const ()), // sub_1405376d0
    Replacement::draft_throws(0x140537980, "unknown_140537980", tpl_1330::<0x8, 0x14053c1e0, 0x143cb0928, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140537980
    Replacement::draft_throws(0x140537f00, "unknown_140537f00", tpl_1330::<0x8, 0x14053c910, 0x143caf9a4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140537f00
    Replacement::draft_throws(0x140538250, "unknown_140538250", tpl_1330::<0x8, 0x14053cc80, 0x143caf9b8, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140538250
    Replacement::draft(0x140538480, "unknown_140538480", fn_140538480 as *const ()), // sub_140538480
    Replacement::draft(0x140538560, "unknown_140538560", fn_140538560 as *const ()), // sub_140538560
    Replacement::draft(0x140538800, "unknown_140538800", fn_140538800 as *const ()), // sub_140538800
    Replacement::draft(0x140538940, "unknown_140538940", fn_140538940 as *const ()), // sub_140538940
    Replacement::draft(0x1405389c0, "unknown_1405389c0", fn_1405389c0 as *const ()), // sub_1405389c0
    Replacement::draft(0x140538a40, "unknown_140538a40", tpl_61197::<0x143c9626c, 0x142489ee0, 0x143c8d160, 0x143c8d120, 0x143c8f4d0, 0x1426467e8, 0x140538bd3> as *const ()), // sub_140538a40
    Replacement::draft(0x140538be0, "unknown_140538be0", tpl_61019::<0x143c8dbcc, 0x142489ee0, 0x143c8de7c, 0x143c8eb98, 0x1426467e8, 0x140538d43> as *const ()), // sub_140538be0
    Replacement::draft(0x140538d50, "unknown_140538d50", tpl_61197::<0x143c8d160, 0x142489ee0, 0x143c95ed0, 0x143c914b0, 0x143c954e4, 0x1426467e8, 0x140538ee3> as *const ()), // sub_140538d50
    Replacement::draft(0x140538ef0, "unknown_140538ef0", fn_140538ef0 as *const ()), // sub_140538ef0
    Replacement::draft(0x140538f30, "unknown_140538f30", tpl_5213::<0x18, 0x1401f2bc0> as *const ()), // sub_140538f30
    Replacement::draft(0x140538f70, "unknown_140538f70", fn_140538f70 as *const ()), // sub_140538f70
    Replacement::draft_throws(0x1405391a0, "unknown_1405391a0", fn_1405391a0 as *const (), Throws::Discard), // sub_1405391a0
    Replacement::draft(0x1405393c0, "unknown_1405393c0", fn_1405393c0 as *const ()), // sub_1405393c0
    Replacement::draft(0x140539600, "unknown_140539600", fn_140539600 as *const ()), // sub_140539600
    Replacement::draft_throws(0x140539890, "unknown_140539890", fn_140539890 as *const (), Throws::Discard), // sub_140539890
    Replacement::draft(0x140539b20, "unknown_140539b20", fn_140539b20 as *const ()), // sub_140539b20
    Replacement::draft(0x140539dd0, "unknown_140539dd0", fn_140539dd0 as *const ()), // sub_140539dd0
    Replacement::draft(0x14053a010, "unknown_14053a010", tpl_64118::<0x143cb0928, 0x143c8d910, 0x143c94b38, 0x143caf9c0, 0x142489ee0, 0x143c8d160, 0x141a8c810> as *const ()), // sub_14053a010
    Replacement::draft_throws(0x14053a260, "unknown_14053a260", tpl_5214::<0x143cb0928, 0x20, 0x8, 0x141a8c220> as *const (), Throws::Discard), // sub_14053a260
    Replacement::draft_throws(0x14053a2c0, "unknown_14053a2c0", fn_14053a2c0 as *const (), Throws::Discard), // sub_14053a2c0
    Replacement::draft(0x14053a510, "unknown_14053a510", fn_14053a510 as *const ()), // sub_14053a510
    Replacement::draft_throws(0x14053a700, "unknown_14053a700", fn_14053a700 as *const (), Throws::Caught), // sub_14053a700
    Replacement::draft(0x14053a860, "unknown_14053a860", fn_14053a860 as *const ()), // sub_14053a860
    Replacement::draft(0x14053aac0, "unknown_14053aac0", tpl_3188::<0x14053bd10, 0x143199740, 0x14053afb0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14053aac0
    Replacement::draft(0x14053ab40, "unknown_14053ab40", tpl_4348::<0x28, 0x30, 0x14053afb0> as *const ()), // sub_14053ab40
    Replacement::draft(0x14053ae40, "unknown_14053ae40", fn_14053ae40 as *const ()), // sub_14053ae40
    Replacement::draft(0x14053afb0, "unknown_14053afb0", fn_14053afb0 as *const ()), // sub_14053afb0
    Replacement::draft(0x14053b0d0, "unknown_14053b0d0", fn_14053b0d0 as *const ()), // sub_14053b0d0
    Replacement::draft(0x14053b2d0, "unknown_14053b2d0", fn_14053b2d0 as *const ()), // sub_14053b2d0
    Replacement::draft(0x14053f520, "unknown_14053f520", fn_14053f520 as *const ()), // sub_14053f520
    Replacement::draft(0x14053f5f0, "unknown_14053f5f0", fn_14053f5f0 as *const ()), // sub_14053f5f0
    Replacement::draft(0x14053f670, "unknown_14053f670", fn_14053f670 as *const ()), // sub_14053f670
    Replacement::draft(0x14053fee0, "unknown_14053fee0", fn_14053fee0 as *const ()), // sub_14053fee0
    Replacement::draft(0x14053ff70, "unknown_14053ff70", tpl_855::<0x1405389c0, 0x1425371a0, 0x142533200, 0x14053b0d0, 0x14053d9c0> as *const ()), // sub_14053ff70
    Replacement::draft(0x1405400d0, "unknown_1405400d0", fn_1405400d0 as *const ()), // sub_1405400d0
    Replacement::draft(0x140540100, "unknown_140540100", fn_140540100 as *const ()), // sub_140540100
    Replacement::draft_throws(0x1405402d0, "bigfish_1405402d0", fn_1405402d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::darkness_burner::trigger_check_update::system>
    Replacement::draft(0x1405406c0, "unknown_1405406c0", tpl_1094::<0x143cb0b5c, 0x140540af0, 0x14020e8f0> as *const ()), // sub_1405406c0
    Replacement::draft(0x1405408c0, "unknown_1405408c0", tpl_1094::<0x143cb0b3c, 0x140467f50, 0x14020e8f0> as *const ()), // sub_1405408c0
    Replacement::draft(0x140540af0, "unknown_140540af0", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431a1b00> as *const ()), // sub_140540af0
    Replacement::draft(0x140540c70, "unknown_140540c70", tpl_64022::<0x88, 0x10088, 0x140540d50, 0x141a91f40> as *const ()), // sub_140540c70
    Replacement::draft(0x140540e50, "unknown_140540e50", tpl_64023::<0x143cb0b3c, 0x142489ee0, 0x143cb0ae8, 0x141a91f40> as *const ()), // sub_140540e50
    Replacement::draft(0x140541480, "unknown_140541480", fn_140541480 as *const ()), // sub_140541480
    Replacement::draft(0x140541650, "unknown_140541650", fn_140541650 as *const ()), // sub_140541650
    Replacement::draft(0x140541730, "unknown_140541730", tpl_3583::<0x8, 0x18, 0x1401f2bc0> as *const ()), // sub_140541730
    Replacement::draft(0x1405417a0, "unknown_1405417a0", fn_1405417a0 as *const ()), // sub_1405417a0
    Replacement::draft_throws(0x1405417f0, "bigfish_1405417f0", tpl_4475::<0x143cb0d10, 0x143cb0e98, 0x1411e9f50, 0x1405458b0, 0x143cb0f28, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x1405443c0, 0x140543620, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a37a6, 0x142646960, 0x143cb0eb8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::damage_reaction::death::system>
    Replacement::draft_throws(0x140541b00, "bigfish_140541b00", fn_140541b00 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::damage_reaction::damage::system>
    Replacement::draft(0x140541e50, "bigfish_140541e50", fn_140541e50 as *const ()), // sub_140541e50
    Replacement::draft(0x140542050, "bigfish_140542050", tpl_1095::<0x143c95c74, 0x14020e8f0> as *const ()), // sub_140542050
    Replacement::draft(0x140542230, "bigfish_140542230", fn_140542230 as *const ()), // sub_140542230
    Replacement::draft(0x140542250, "bigfish_140542250", tpl_1094::<0x143cb0f2c, 0x140543250, 0x14020e8f0> as *const ()), // sub_140542250
    Replacement::draft(0x140542450, "bigfish_140542450", tpl_1466::<0x143cb0f1c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140542450
    Replacement::draft(0x140542720, "bigfish_140542720", fn_140542720 as *const ()), // sub_140542720
    Replacement::draft(0x140542960, "bigfish_140542960", tpl_1094::<0x143cb0c4c, 0x140543480, 0x14020e8f0> as *const ()), // sub_140542960
    Replacement::draft_throws(0x140542b60, "bigfish_140542b60", tpl_4130::<0x143cb0b60, 0x143cb0b68, 0x1411e9f50, 0x140545bb0, 0x143cb0c48, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x1405452b0, 0x1405448b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a2ec6, 0x142646960, 0x143cb0b88, 0x143cb0b90, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::damage_reaction::stream_in::system>
    Replacement::draft_throws(0x140542e70, "bigfish_140542e70", tpl_3281::<0x143cb0b94, 0x143cb0d80, 0x1411e9f50, 0x140545ab0, 0x143cb0d00, 0x1420af120, 0x1420a28e0, 0x140545500, 0x1405449f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a2cc6, 0x142646960, 0x143cb0da0, 0x143cb0da8, 0x143cb0dac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::damage_reaction::stream_out::system>
    Replacement::draft(0x140543190, "unknown_140543190", fn_140543190 as *const ()), // sub_140543190
    Replacement::draft(0x140543210, "unknown_140543210", tpl_4095::<0x8, 0x18, 0x1401f2bc0> as *const ()), // sub_140543210
    Replacement::draft(0x140543250, "unknown_140543250", tpl_5241::<0x1401e6d50, 0x140544370, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140543a20> as *const ()), // sub_140543250
    Replacement::draft(0x1405433d0, "unknown_1405433d0", fn_1405433d0 as *const ()), // sub_1405433d0
    Replacement::draft(0x140543480, "unknown_140543480", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431a3b50, 0x140544220> as *const ()), // sub_140543480
    Replacement::draft(0x140543620, "unknown_140543620", fn_140543620 as *const ()), // sub_140543620
    Replacement::draft(0x140543720, "unknown_140543720", fn_140543720 as *const ()), // sub_140543720
    Replacement::draft_throws(0x140543810, "unknown_140543810", fn_140543810 as *const (), Throws::Discard), // sub_140543810
    Replacement::draft_throws(0x1405438e0, "unknown_1405438e0", fn_1405438e0 as *const (), Throws::Discard), // sub_1405438e0
    Replacement::draft(0x140543a20, "unknown_140543a20", fn_140543a20 as *const ()), // sub_140543a20
    Replacement::draft(0x140543ad0, "content_140543ad0", tpl_1953::<0x140544170, 0x143084e28, 0x141834d30> as *const ()), // content::EnemyDamageReactionComponent::content::EnemyDamageReactionComponent::vfunc_0
    Replacement::draft(0x140543b20, "unknown_140543b20", tpl_4889::<0x140544170, 0x143084e28> as *const ()), // sub_140543b20
    Replacement::draft(0x140544220, "unknown_140544220", fn_140544220 as *const ()), // sub_140544220
    Replacement::draft(0x140544300, "unknown_140544300", tpl_865::<0x8, 0x1405415a0> as *const ()), // sub_140544300
    Replacement::draft(0x140544370, "unknown_140544370", fn_140544370 as *const ()), // sub_140544370
    Replacement::draft(0x1405443c0, "unknown_1405443c0", fn_1405443c0 as *const ()), // sub_1405443c0
    Replacement::draft_throws(0x1405445d0, "unknown_1405445d0", fn_1405445d0 as *const (), Throws::Discard), // sub_1405445d0
    Replacement::draft_throws(0x1405448b0, "unknown_1405448b0", tpl_5251::<0x142407c80, 0x143cada18, 0x140544c80, 0x141a93a90> as *const (), Throws::Discard), // sub_1405448b0
    Replacement::draft(0x1405449f0, "unknown_1405449f0", tpl_1126::<0x142407c80, 0x580f0, 0x141a938b0> as *const ()), // sub_1405449f0
    Replacement::draft(0x140544ab0, "unknown_140544ab0", fn_140544ab0 as *const ()), // sub_140544ab0
    Replacement::draft(0x140544b90, "unknown_140544b90", fn_140544b90 as *const ()), // sub_140544b90
    Replacement::draft(0x140544db0, "unknown_140544db0", fn_140544db0 as *const ()), // sub_140544db0
    Replacement::draft_throws(0x1405452b0, "unknown_1405452b0", tpl_5255::<0x142407c80, 0x143cada18, 0x143cb0c4c, 0x142489ee0, 0x143c8e70c, 0x143c8d87c, 0x141a93a90> as *const (), Throws::Discard), // sub_1405452b0
    Replacement::draft(0x140545500, "unknown_140545500", tpl_1130::<0x142407c80, 0x141a938b0> as *const ()), // sub_140545500
    Replacement::draft(0x140545610, "unknown_140545610", fn_140545610 as *const ()), // sub_140545610
    Replacement::draft(0x1405458b0, "unknown_1405458b0", fn_1405458b0 as *const ()), // sub_1405458b0
    Replacement::draft(0x140545a70, "unknown_140545a70", tpl_866::<0x8, 0x1405415a0> as *const ()), // sub_140545a70
    Replacement::draft(0x140545d30, "unknown_140545d30", fn_140545d30 as *const ()), // sub_140545d30
    Replacement::draft(0x140546110, "unknown_140546110", fn_140546110 as *const ()), // sub_140546110
    Replacement::draft(0x1405462b0, "unknown_1405462b0", fn_1405462b0 as *const ()), // sub_1405462b0
    Replacement::draft_throws(0x140546520, "bigfish_140546520", fn_140546520 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_difficulty_damage_modification::system>
    Replacement::draft_throws(0x140546780, "bigfish_140546780", tpl_3465::<0x143cb0f70, 0x143cb10b0, 0x1411e9f50, 0x1405470c0, 0x140547b80, 0x140547420, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a3ee6, 0x142646960, 0x143cb10d0, 0x143cb10e0, 0x143cb10e2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_dealer_damage_modification::system>
    Replacement::draft_throws(0x1405469d0, "bigfish_1405469d0", tpl_3455::<0x143cb12b8, 0x143cb1000, 0x1411e9f50, 0x140547750, 0x140547dd0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a4726, 0x142646960, 0x143cb1020, 0x143cb1030, 0x143cb1034, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::update_receiver_damage_modification::system>
    Replacement::draft(0x140546c20, "bigfish_140546c20", tpl_5262::<0x143cb12b0, 0x143cb1228, 0x1411e9f50, 0x143c94b34, 0x140215610, 0x1420a28e0, 0x140547e50, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a4a26, 0x142646960, 0x143cb1248, 0x143cb1250, 0x143cb1252, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::apply_damage_modification::system>
    Replacement::draft_throws(0x140546ec0, "unknown_140546ec0", fn_140546ec0 as *const (), Throws::Discard), // sub_140546ec0
    Replacement::draft_throws(0x1405470c0, "unknown_1405470c0", tpl_5264::<0x140548430, 0x143cb10e4, 0x1420af120, 0x1405482f0, 0x143cb12a4, 0x140215750, 0x1405481b0, 0x143cb1254, 0x143c8ef20, 0x140215610, 0x143c8d08c, 0x143c8d00c, 0x143c8de64, 0x143c94b34, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405470c0
    Replacement::draft(0x140547420, "unknown_140547420", tpl_4250::<0x88, 0x10088, 0x1405474f0, 0x1405478c0, 0x1405475f0> as *const ()), // sub_140547420
    Replacement::draft(0x1405474f0, "unknown_1405474f0", tpl_3359::<0x143c8ceec, 0x20, 0x143cb12a4, 0x8, 0x143c8d00c, 0x143cb1254, 0x143c8d08c, 0x143c8ef20, 0x143c8de64, 0x143c94b34, 0x10, 0x18> as *const ()), // sub_1405474f0
    Replacement::draft(0x1405475f0, "unknown_1405475f0", fn_1405475f0 as *const ()), // sub_1405475f0
    Replacement::draft_throws(0x140547750, "unknown_140547750", tpl_5266::<0x143c94b34, 0x140215610, 0x143c8d08c, 0x140548060, 0x143cb1220, 0x1420af120, 0x140215750, 0x1420a28e0> as *const (), Throws::Discard), // sub_140547750
    Replacement::draft(0x140547ae0, "unknown_140547ae0", fn_140547ae0 as *const ()), // sub_140547ae0
    Replacement::draft(0x140547b80, "unknown_140547b80", fn_140547b80 as *const ()), // sub_140547b80
    Replacement::draft_throws(0x140547dd0, "unknown_140547dd0", tpl_1804::<0x143cb1220, 0x580e0, 0x20, 0x8, 0x143c8d08c, 0x143c94b34, 0x141a96d80> as *const (), Throws::Discard), // sub_140547dd0
    Replacement::draft(0x140548a40, "unknown_140548a40", fn_140548a40 as *const ()), // sub_140548a40
    Replacement::draft(0x140548b90, "d_140548b90", tpl_509::<0x24, 0x20> as *const ()), // d::Tweakable<d::TweakableList,0>::d::Tweakable<d::TweakableList,0>::vfunc_2
    Replacement::draft(0x140548ba0, "d_140548ba0", tpl_508::<0x24, 0x20> as *const ()), // d::Tweakable<d::TweakableList,0>::d::Tweakable<d::TweakableList,0>::vfunc_1
    Replacement::draft(0x140548bb0, "d_140548bb0", fn_140548bb0 as *const ()), // d::Tweakable<d::TweakableList,0>::d::Tweakable<d::TweakableList,0>::vfunc_0
    Replacement::draft(0x140548c00, "unknown_140548c00", fn_140548c00 as *const ()), // sub_140548c00
    Replacement::draft(0x140548cc0, "unknown_140548cc0", fn_140548cc0 as *const ()), // sub_140548cc0
    Replacement::draft_throws(0x140548f10, "anonymous_namespace_140548f10", tpl_5273::<0x143cb13a8, 0x143cb12f8, 0x1411e9f50, 0x14054aa60, 0x14054b170, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a57c6, 0x142646960, 0x143cb1318, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::handleDamage>
    Replacement::draft_throws(0x140549150, "anonymous_namespace_140549150", fn_140549150 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::damage_status_cleanup::system>
    Replacement::draft_throws(0x140549470, "anonymous_namespace_140549470", tpl_5275::<0x143cb1530, 0x143cb1338, 0x1411e9f50, 0x14054b440, 0x143cb1708, 0x1420af120, 0x14054b060, 0x1420a28e0, 0x14054b210, 0x14054ae10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a6796, 0x142646960, 0x143cb1358, 0x143cb135c, 0x143cb135e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::contact::system>
    Replacement::draft(0x140549770, "unknown_140549770", tpl_3278::<0x143cb12b4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140549770
    Replacement::draft(0x140549ab0, "unknown_140549ab0", tpl_1094::<0x143cb10e8, 0x14054a740, 0x14020e8f0> as *const ()), // sub_140549ab0
    Replacement::draft(0x140549cb0, "unknown_140549cb0", tpl_1094::<0x143c93b20, 0x14054a8d0, 0x14020e8f0> as *const ()), // sub_140549cb0
    Replacement::draft(0x140549eb0, "unknown_140549eb0", fn_140549eb0 as *const ()), // sub_140549eb0
    Replacement::draft(0x14054a200, "unknown_14054a200", fn_14054a200 as *const ()), // sub_14054a200
    Replacement::draft_throws(0x14054a360, "unknown_14054a360", fn_14054a360 as *const (), Throws::Discard), // sub_14054a360
    Replacement::draft(0x14054a510, "unknown_14054a510", fn_14054a510 as *const ()), // sub_14054a510
    Replacement::draft(0x14054a740, "unknown_14054a740", tpl_4210::<0x1401e6d50, 0x14054b140, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_14054a740
    Replacement::draft(0x14054a8d0, "unknown_14054a8d0", fn_14054a8d0 as *const ()), // sub_14054a8d0
    Replacement::draft_throws(0x14054aa60, "unknown_14054aa60", fn_14054aa60 as *const (), Throws::Discard), // sub_14054aa60
    Replacement::draft(0x14054ae10, "unknown_14054ae10", tpl_64030::<0x88, 0x10088, 0x14054af30, 0x141a9a450> as *const ()), // sub_14054ae10
    Replacement::draft_throws(0x14054b060, "unknown_14054b060", tpl_1498::<0x143c88dd0, 0x143c88ddc, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8ceec, 0x143c88dd8, 0x143c88de0, 0x141834a0c> as *const (), Throws::Discard), // sub_14054b060
    Replacement::draft(0x14054b140, "unknown_14054b140", tpl_1955::<0x10, 0x1401e59a0> as *const ()), // sub_14054b140
    Replacement::draft_throws(0x14054b170, "unknown_14054b170", fn_14054b170 as *const (), Throws::Discard), // sub_14054b170
    Replacement::draft(0x14054b1b0, "unknown_14054b1b0", tpl_4148::<0x143cb1360, 0x20, 0x8, 0x141a9a4e0> as *const ()), // sub_14054b1b0
    Replacement::draft(0x14054b600, "unknown_14054b600", fn_14054b600 as *const ()), // sub_14054b600
    Replacement::draft(0x14054ba00, "unknown_14054ba00", tpl_5284::<0x14054c6c0, 0x14054c850, 0x142537280, 0x1426456c8, 0x1426453a0, 0x140548c00, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_14054ba00
    Replacement::draft(0x14054bb40, "unknown_14054bb40", fn_14054bb40 as *const ()), // sub_14054bb40
    Replacement::draft(0x14054be20, "unknown_14054be20", fn_14054be20 as *const ()), // sub_14054be20
    Replacement::draft(0x14054bfc0, "unknown_14054bfc0", fn_14054bfc0 as *const ()), // sub_14054bfc0
    Replacement::draft(0x14054c0c0, "unknown_14054c0c0", fn_14054c0c0 as *const ()), // sub_14054c0c0
    Replacement::draft_throws(0x14054c1b0, "unknown_14054c1b0", fn_14054c1b0 as *const (), Throws::Discard), // sub_14054c1b0
    Replacement::draft(0x14054c570, "unknown_14054c570", fn_14054c570 as *const ()), // sub_14054c570
    Replacement::draft(0x14054c6c0, "unknown_14054c6c0", fn_14054c6c0 as *const ()), // sub_14054c6c0
    Replacement::draft(0x14054c850, "unknown_14054c850", fn_14054c850 as *const ()), // sub_14054c850
    Replacement::draft(0x14054c9c0, "unknown_14054c9c0", tpl_3774::<0x30, 0x143cb1824, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x14054ca42> as *const ()), // sub_14054c9c0
    Replacement::draft(0x14054ca50, "unknown_14054ca50", tpl_64455::<0x8, 0x1403d0e60, 0x10, 0x14054ce20> as *const ()), // sub_14054ca50
    Replacement::draft(0x14054cab0, "unknown_14054cab0", tpl_3942::<0x30, 0x143c9a624, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14054cb00> as *const ()), // sub_14054cab0
    Replacement::draft(0x14054cb10, "unknown_14054cb10", fn_14054cb10 as *const ()), // sub_14054cb10
    Replacement::draft(0x14054cb80, "unknown_14054cb80", fn_14054cb80 as *const ()), // sub_14054cb80
    Replacement::draft_throws(0x14054cbe0, "bigfish_14054cbe0", tpl_5293::<0x143cb1a90, 0x143cb1a98, 0x1411e9f50, 0x14054e040, 0x14054ee20, 0x14054e3a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a74b6, 0x142646960, 0x143cb1ab8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdnpc::update::system>
    Replacement::draft(0x14054ced0, "bigfish_14054ced0", tpl_1094::<0x143cb1778, 0x14054de50, 0x14020e8f0> as *const ()), // sub_14054ced0
    Replacement::draft(0x14054d0d0, "bigfish_14054d0d0", tpl_1436::<0x58, 0x60, 0x158, 0x143cb1778, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14054d0d0
    Replacement::draft_throws(0x14054d1b0, "bigfish_14054d1b0", tpl_4955::<0x143cb1740, 0x143cb1918, 0x1411e9f50, 0x14054fdc0, 0x143cb1a8c, 0x1420af120, 0x1420a28e0, 0x14054f4a0, 0x14054f150, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a7d46, 0x142646960, 0x143cb1938, 0x143cb193c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdnpc::stream_in::system>
    Replacement::draft_throws(0x14054d490, "bigfish_14054d490", tpl_1886::<0x143cb1768, 0x143cb1850, 0x1411e9f50, 0x14054fcc0, 0x143cb176c, 0x1420af120, 0x1420a28e0, 0x14054f600, 0x14054f260, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a7f36, 0x142646960, 0x143cb1870, 0x143cb1874, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdnpc::stream_out::system>
    Replacement::draft(0x14054d770, "unknown_14054d770", fn_14054d770 as *const ()), // sub_14054d770
    Replacement::draft_throws(0x14054da10, "coregame_14054da10", tpl_3594::<0x143cb1970, 0x143cb1980, 0x14054e720, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a8126, 0x142646960, 0x143cb19a0, 0x143cb19c0, 0x143cb19d0, 0x143cb19d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<struct bigfish::component::CrowdNP
    Replacement::draft(0x14054dc40, "unknown_14054dc40", fn_14054dc40 as *const ()), // sub_14054dc40
    Replacement::draft(0x14054de50, "unknown_14054de50", fn_14054de50 as *const ()), // sub_14054de50
    Replacement::draft_throws(0x14054e040, "unknown_14054e040", tpl_5264::<0x14054f950, 0x143cb1820, 0x1420af120, 0x14054f840, 0x143cb182c, 0x140215750, 0x14054f750, 0x143cb1968, 0x143c8d910, 0x140215610, 0x143c8d08c, 0x143c90ecc, 0x143c9a4ec, 0x143c9a628, 0x143c8fd28, 0x1420a28e0> as *const (), Throws::Discard), // sub_14054e040
    Replacement::draft_throws(0x14054e3a0, "unknown_14054e3a0", fn_14054e3a0 as *const (), Throws::Discard), // sub_14054e3a0
    Replacement::draft(0x14054e480, "unknown_14054e480", tpl_3359::<0x143c8fd28, 0x20, 0x143cb182c, 0x8, 0x143c90ecc, 0x143cb1968, 0x143c8d08c, 0x143c8d910, 0x143c9a4ec, 0x143c9a628, 0x10, 0x18> as *const ()), // sub_14054e480
    Replacement::draft_throws(0x14054e580, "unknown_14054e580", fn_14054e580 as *const (), Throws::Discard), // sub_14054e580
    Replacement::draft_throws(0x14054e720, "unknown_14054e720", tpl_3011::<0x1411e9f50, 0x14054fbd0, 0x143cb1910, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14054f6e0, 0x1401de410> as *const (), Throws::Discard), // sub_14054e720
    Replacement::draft(0x14054ecd0, "unknown_14054ecd0", tpl_3600::<0x70, 0x68, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14054ecd0
    Replacement::draft_throws(0x14054f4a0, "unknown_14054f4a0", fn_14054f4a0 as *const (), Throws::Caught), // sub_14054f4a0
    Replacement::draft(0x14054f600, "unknown_14054f600", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14054d0d0> as *const ()), // sub_14054f600
    Replacement::draft(0x14054f6e0, "unknown_14054f6e0", tpl_3028::<0x143cb1910, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14054d770> as *const ()), // sub_14054f6e0
    Replacement::draft(0x14054f750, "unknown_14054f750", tpl_5302::<0x143cb1770, 0x143cb1748, 0x8, 0x10, 0x18, 0x141834a78, 0x143cb1824, 0x143c8fa04, 0x143cb1758, 0x143cb1760, 0x141834a0c> as *const ()), // sub_14054f750
    Replacement::draft(0x14054f840, "unknown_14054f840", fn_14054f840 as *const ()), // sub_14054f840
    Replacement::draft(0x14054f950, "unknown_14054f950", fn_14054f950 as *const ()), // sub_14054f950
    Replacement::draft(0x14054fbd0, "unknown_14054fbd0", tpl_5302::<0x143cb196c, 0x143cb1830, 0x8, 0x10, 0x18, 0x141834a78, 0x143c9a620, 0x143c9e158, 0x143cb1840, 0x143cb1848, 0x141834a0c> as *const ()), // sub_14054fbd0
    Replacement::draft(0x14054fcc0, "unknown_14054fcc0", tpl_5305::<0x143cb170c, 0x143cb18e8, 0x8, 0x10, 0x18, 0x141834a78, 0x143cb1778, 0x143c9e158, 0x143c8d694, 0x143cb1908, 0x141834a0c> as *const ()), // sub_14054fcc0
    Replacement::draft(0x14054fdc0, "unknown_14054fdc0", fn_14054fdc0 as *const ()), // sub_14054fdc0
    Replacement::draft_throws(0x14054fec0, "unknown_14054fec0", tpl_4541::<0x143cb1968, 0x14054f750, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14054fec0
    Replacement::draft_throws(0x14054ff50, "unknown_14054ff50", tpl_4541::<0x143cb182c, 0x14054f840, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14054ff50
    Replacement::draft_throws(0x14054ffe0, "unknown_14054ffe0", tpl_4541::<0x143cb1820, 0x14054f950, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14054ffe0
    Replacement::draft(0x140550070, "unknown_140550070", fn_140550070 as *const ()), // sub_140550070
    Replacement::draft(0x140550150, "unknown_140550150", fn_140550150 as *const ()), // sub_140550150
    Replacement::draft(0x1405501b0, "unknown_1405501b0", fn_1405501b0 as *const ()), // sub_1405501b0
    Replacement::draft_throws(0x140550240, "unknown_140550240", tpl_4541::<0x143cb1910, 0x14054fbd0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140550240
    Replacement::draft_throws(0x1405502d0, "unknown_1405502d0", tpl_4541::<0x143cb176c, 0x14054fcc0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1405502d0
    Replacement::draft_throws(0x140550360, "unknown_140550360", tpl_4541::<0x143cb1a8c, 0x14054fdc0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140550360
    Replacement::draft(0x1405503f0, "unknown_1405503f0", tpl_1::<0x143c88e68> as *const ()), // sub_1405503f0
    Replacement::draft(0x140550400, "crt_140550400", fn_140550400 as *const ()), // sub_140550400
    Replacement::draft(0x140550470, "content_140550470", fn_140550470 as *const ()), // content::CrowdAreaComponent::content::CrowdAreaComponent::vfunc_18
    Replacement::draft(0x140550620, "unknown_140550620", fn_140550620 as *const ()), // sub_140550620
    Replacement::draft(0x1405506c0, "unknown_1405506c0", fn_1405506c0 as *const ()), // sub_1405506c0
    Replacement::draft(0x140550770, "unknown_140550770", tpl_2365::<0x14387bac8, 0x14243b0b0, 0x8, 0x1405557c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140550770
    Replacement::draft(0x1405507e0, "unknown_1405507e0", tpl_2365::<0x14387bac8, 0x14243b0b0, 0x8, 0x140555cc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1405507e0
    Replacement::draft_throws(0x140550850, "unknown_140550850", fn_140550850 as *const (), Throws::Caught), // sub_140550850
    Replacement::draft_throws(0x140550920, "unknown_140550920", fn_140550920 as *const (), Throws::Caught), // sub_140550920
    Replacement::draft_throws(0x1405509f0, "unknown_1405509f0", fn_1405509f0 as *const (), Throws::Caught), // sub_1405509f0
    Replacement::draft(0x140550ac0, "unknown_140550ac0", tpl_2365::<0x14387bac8, 0x14243b0b0, 0x8, 0x140555b30, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140550ac0
    Replacement::draft(0x140550b30, "unknown_140550b30", tpl_2365::<0x14387bac8, 0x14243b0b0, 0x8, 0x140556120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140550b30
    Replacement::draft_throws(0x140550ba0, "unknown_140550ba0", fn_140550ba0 as *const (), Throws::Discard), // sub_140550ba0
    Replacement::draft_throws(0x140550c40, "unknown_140550c40", tpl_1480::<0x142760810, 0x1410d6ca0, 0x1405530b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140550c40
    Replacement::draft_throws(0x140550ee0, "bigfish_140550ee0", tpl_3369::<0x143cb1b00, 0x143cb1b08, 0x1411e9f50, 0x1405589b0, 0x143cb1e70, 0x1420af120, 0x1420a28e0, 0x140556900, 0x140554aa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a9526, 0x142646960, 0x143cb1b28, 0x143cb1b38, 0x143cb1b40, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::add_components_after_streamin::system>
    Replacement::draft_throws(0x1405511d0, "bigfish_1405511d0", tpl_1102::<0x143cb1e78, 0x143cb1bf0, 0x1411e9f50, 0x140558810, 0x143cb1e80, 0x1420af120, 0x1420a28e0, 0x140556a60, 0x140554bb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a91b6, 0x142646960, 0x143cb1c10, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::wait_for_teleport::system>
    Replacement::draft_throws(0x1405514b0, "bigfish_1405514b0", fn_1405514b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::despawn_npcs::system>
    Replacement::draft_throws(0x140551810, "bigfish_140551810", tpl_3681::<0x143cb1b4c, 0x143cb20e0, 0x1411e9f50, 0x140554fd0, 0x140556ec0, 0x140555160, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a9736, 0x142646960, 0x143cb2100, 0x143cb2104, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::update::system>
    Replacement::draft(0x140551a50, "bigfish_140551a50", fn_140551a50 as *const ()), // sub_140551a50
    Replacement::draft_throws(0x140551b50, "bigfish_140551b50", fn_140551b50 as *const (), Throws::Caught), // sub_140551b50
    Replacement::draft_throws(0x140551c80, "bigfish_140551c80", fn_140551c80 as *const (), Throws::Caught), // sub_140551c80
    Replacement::draft_throws(0x140551db0, "bigfish_140551db0", fn_140551db0 as *const (), Throws::Caught), // sub_140551db0
    Replacement::draft(0x140551ea0, "bigfish_140551ea0", tpl_1094::<0x143cb218c, 0x140554420, 0x14020e8f0> as *const ()), // sub_140551ea0
    Replacement::draft(0x1405520a0, "bigfish_1405520a0", tpl_1436::<0x58, 0x60, 0x158, 0x143cb218c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405520a0
    Replacement::draft(0x140552180, "bigfish_140552180", tpl_3072::<0x143c9a86c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140552180
    Replacement::draft(0x1405524d0, "bigfish_1405524d0", tpl_1094::<0x143c9a660, 0x1405545b0, 0x14020e8f0> as *const ()), // sub_1405524d0
    Replacement::draft(0x1405526d0, "bigfish_1405526d0", tpl_1095::<0x143cb2060, 0x14020e8f0> as *const ()), // sub_1405526d0
    Replacement::draft(0x1405528b0, "bigfish_1405528b0", tpl_1094::<0x143cb1f90, 0x140554730, 0x14020e8f0> as *const ()), // sub_1405528b0
    Replacement::draft_throws(0x140552ae0, "bigfish_140552ae0", tpl_3591::<0x143cb1f24, 0x143cb2108, 0x1411e9f50, 0x140559c20, 0x143cb1f3c, 0x1420af120, 0x1404c2240, 0x1420a28e0, 0x140558250, 0x140556780, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a9dd6, 0x142646960, 0x143cb2128, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::stream_in::system>
    Replacement::draft_throws(0x140552dd0, "bigfish_140552dd0", tpl_3683::<0x143cb2018, 0x143cb1d08, 0x1411e9f50, 0x140559b30, 0x143cb1be8, 0x1420af120, 0x1420a28e0, 0x140558400, 0x140556860, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431a9c46, 0x142646960, 0x143cb1d28, 0x143cb1d30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::stream_out::system>
    Replacement::draft_throws(0x1405530b0, "bigfish_1405530b0", tpl_4804::<0x143cb1e8c, 0x143cb1b50, 0x1405548d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431aa0d6, 0x142646960, 0x143cb1b70, 0x143cb1b78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crowdmodule::query_removed::system>
    Replacement::draft(0x1405532c0, "unknown_1405532c0", fn_1405532c0 as *const ()), // sub_1405532c0
    Replacement::draft_throws(0x1405534c0, "unknown_1405534c0", fn_1405534c0 as *const (), Throws::Caught), // sub_1405534c0
    Replacement::draft_throws(0x140553a70, "unknown_140553a70", fn_140553a70 as *const (), Throws::Caught), // sub_140553a70
    Replacement::draft_throws(0x140553fe0, "unknown_140553fe0", fn_140553fe0 as *const (), Throws::Caught), // sub_140553fe0
    Replacement::draft(0x140554420, "unknown_140554420", tpl_4210::<0x1401e6d50, 0x140556440, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_140554420
    Replacement::draft(0x1405545b0, "unknown_1405545b0", tpl_5321::<0x1401e6d50, 0x140556610, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1403d0c40> as *const ()), // sub_1405545b0
    Replacement::draft(0x140554730, "unknown_140554730", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431ab0e0, 0x140555640> as *const ()), // sub_140554730
    Replacement::draft_throws(0x1405548d0, "unknown_1405548d0", tpl_3370::<0x1411e9f50, 0x140558ab0, 0x143cb218c, 0x1401fc320, 0x1405599e0, 0x143cb1f38, 0x1420af120, 0x140215750, 0x1420a28e0, 0x1405584e0, 0x1401de410> as *const (), Throws::Discard), // sub_1405548d0
    Replacement::draft(0x140554aa0, "unknown_140554aa0", tpl_1127::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x141a9fd30> as *const ()), // sub_140554aa0
    Replacement::draft(0x140554bb0, "unknown_140554bb0", fn_140554bb0 as *const ()), // sub_140554bb0
    Replacement::draft(0x140554ce0, "unknown_140554ce0", tpl_1916::<0x18, 0x20, 0x40, 0x8, 0x141a9fb30> as *const ()), // sub_140554ce0
    Replacement::draft_throws(0x140554dd0, "unknown_140554dd0", fn_140554dd0 as *const (), Throws::Caught), // sub_140554dd0
    Replacement::draft_throws(0x140554ef0, "unknown_140554ef0", tpl_3788::<0x18, 0x20, 0x30, 0x8, 0x28, 0x141a9f7b0> as *const (), Throws::Discard), // sub_140554ef0
    Replacement::draft_throws(0x140554fd0, "unknown_140554fd0", tpl_5324::<0x140558650, 0x143cb1f8c, 0x1420af120, 0x1404c2240, 0x143c8d910, 0x140215610, 0x143c90ecc, 0x143c9a4ec, 0x1420a28e0> as *const (), Throws::Discard), // sub_140554fd0
    Replacement::draft_throws(0x140555160, "unknown_140555160", fn_140555160 as *const (), Throws::Discard), // sub_140555160
    Replacement::draft(0x1405552b0, "content_1405552b0", tpl_1953::<0x140555590, 0x143084e28, 0x141834d30> as *const ()), // content::CrowdAreaComponent::content::CrowdAreaComponent::vfunc_0
    Replacement::draft(0x140555640, "unknown_140555640", fn_140555640 as *const ()), // sub_140555640
    Replacement::draft(0x140555740, "unknown_140555740", fn_140555740 as *const ()), // sub_140555740
    Replacement::draft(0x1405557c0, "unknown_1405557c0", fn_1405557c0 as *const ()), // sub_1405557c0
    Replacement::draft_throws(0x140555980, "unknown_140555980", fn_140555980 as *const (), Throws::Caught), // sub_140555980
    Replacement::draft(0x140555b30, "unknown_140555b30", fn_140555b30 as *const ()), // sub_140555b30
    Replacement::draft(0x140555cc0, "unknown_140555cc0", fn_140555cc0 as *const ()), // sub_140555cc0
    Replacement::draft_throws(0x140555e00, "unknown_140555e00", fn_140555e00 as *const (), Throws::Caught), // sub_140555e00
    Replacement::draft(0x140555fb0, "unknown_140555fb0", fn_140555fb0 as *const ()), // sub_140555fb0
    Replacement::draft(0x140556120, "unknown_140556120", fn_140556120 as *const ()), // sub_140556120
    Replacement::draft_throws(0x140556270, "unknown_140556270", fn_140556270 as *const (), Throws::Caught), // sub_140556270
    Replacement::draft(0x1405563a0, "unknown_1405563a0", fn_1405563a0 as *const ()), // sub_1405563a0
    Replacement::draft(0x140556440, "unknown_140556440", tpl_1955::<0x10, 0x140551a50> as *const ()), // sub_140556440
    Replacement::draft(0x140556470, "unknown_140556470", fn_140556470 as *const ()), // sub_140556470
    Replacement::draft(0x140556610, "unknown_140556610", fn_140556610 as *const ()), // sub_140556610
    Replacement::draft_throws(0x140558250, "unknown_140558250", tpl_64499::<0x143cb1f90, 0x142489ee0, 0x143c8d120, 0x141aa0560> as *const (), Throws::Discard), // sub_140558250
    Replacement::draft(0x140558400, "unknown_140558400", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405520a0> as *const ()), // sub_140558400
    Replacement::draft(0x1405584e0, "unknown_1405584e0", tpl_4345::<0x143cb218c, 0x143cb1f38, 0x142407c80, 0x141aa0190, 0x1405593a0> as *const ()), // sub_1405584e0
    Replacement::draft(0x140558810, "unknown_140558810", fn_140558810 as *const ()), // sub_140558810
    Replacement::draft(0x140558ab0, "unknown_140558ab0", tpl_3188::<0x140559a90, 0x1431ab238, 0x1405593a0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140558ab0
    Replacement::draft(0x140558b30, "unknown_140558b30", fn_140558b30 as *const ()), // sub_140558b30
    Replacement::draft(0x140558c00, "unknown_140558c00", tpl_4348::<0x20, 0x28, 0x1405593a0> as *const ()), // sub_140558c00
    Replacement::draft(0x140559220, "unknown_140559220", fn_140559220 as *const ()), // sub_140559220
    Replacement::draft(0x1405593a0, "unknown_1405593a0", tpl_4821::<0x1401de830, 0x140551a50, 0x1425371a0, 0x142533200> as *const ()), // sub_1405593a0
    Replacement::draft_throws(0x140559510, "unknown_140559510", fn_140559510 as *const (), Throws::Caught), // sub_140559510
    Replacement::draft_throws(0x1405596d0, "unknown_1405596d0", fn_1405596d0 as *const (), Throws::Caught), // sub_1405596d0
    Replacement::draft_throws(0x140559860, "unknown_140559860", fn_140559860 as *const (), Throws::Caught), // sub_140559860
    Replacement::draft(0x140559a90, "unknown_140559a90", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431ab238, 0x8, 0x20> as *const ()), // sub_140559a90
    Replacement::draft(0x140559d70, "unknown_140559d70", fn_140559d70 as *const ()), // sub_140559d70
    Replacement::draft(0x140559e50, "unknown_140559e50", fn_140559e50 as *const ()), // sub_140559e50
    Replacement::draft(0x140559ea0, "unknown_140559ea0", tpl_5361::<0x143881f70, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140559ea0
    Replacement::draft(0x140559fc0, "unknown_140559fc0", tpl_2089::<0x143881f70, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140559fc0
    Replacement::draft(0x14055a030, "unknown_14055a030", tpl_2086::<0x143881f70, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14055a030
    Replacement::draft(0x14055a130, "unknown_14055a130", tpl_2080::<0x143881f78, 0x143881f70, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // sub_14055a130
    Replacement::draft(0x14055a1a0, "unknown_14055a1a0", tpl_2076::<0x143881f70, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14055a1a0
    Replacement::draft_throws(0x14055a210, "unknown_14055a210", tpl_2090::<0x143881f78, 0x143881f70, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14055a210
    Replacement::draft(0x14055a280, "unknown_14055a280", tpl_1095::<0x143cb1be0, 0x14020e8f0> as *const ()), // sub_14055a280
    Replacement::draft(0x14055a460, "unknown_14055a460", tpl_1094::<0x143c9e158, 0x14055ab00, 0x14020e8f0> as *const ()), // sub_14055a460
    Replacement::draft(0x14055a6f0, "unknown_14055a6f0", fn_14055a6f0 as *const ()), // sub_14055a6f0
    Replacement::draft(0x14055a910, "unknown_14055a910", fn_14055a910 as *const ()), // sub_14055a910
    Replacement::draft(0x14055a9f0, "unknown_14055a9f0", fn_14055a9f0 as *const ()), // sub_14055a9f0
    Replacement::draft(0x14055ab00, "unknown_14055ab00", fn_14055ab00 as *const ()), // sub_14055ab00
    Replacement::draft(0x14055ae50, "unknown_14055ae50", fn_14055ae50 as *const ()), // sub_14055ae50
    Replacement::draft(0x14055af30, "unknown_14055af30", tpl_5367::<0x7, 0x30, 0x18> as *const ()), // sub_14055af30
    Replacement::draft(0x14055b100, "unknown_14055b100", tpl_1265::<0x28, 0x10> as *const ()), // sub_14055b100
    Replacement::draft(0x14055b110, "unknown_14055b110", tpl_855::<0x140555740, 0x1425371a0, 0x142533200, 0x1405571a0, 0x140558b30> as *const ()), // sub_14055b110
    Replacement::draft(0x14055b270, "unknown_14055b270", fn_14055b270 as *const ()), // sub_14055b270
    Replacement::draft(0x14055b3d0, "unknown_14055b3d0", fn_14055b3d0 as *const ()), // sub_14055b3d0
    Replacement::draft(0x14055b440, "unknown_14055b440", tpl_3695::<0x1401e7c10, 0x14055b560> as *const ()), // sub_14055b440
    Replacement::draft_throws(0x14055b6d0, "unknown_14055b6d0", tpl_1480::<0x142761980, 0x1410d6ca0, 0x14055bb40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14055b6d0
    Replacement::draft(0x14055b970, "unknown_14055b970", tpl_64515::<0x1426467e8, 0x14055bd90, 0x14055b9e9> as *const ()), // sub_14055b970
    Replacement::draft_throws(0x14055bb40, "bigfish_14055bb40", tpl_4417::<0x143cb2190, 0x143cb22c0, 0x1411e9f50, 0x14055bee0, 0x14055c140, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ab476, 0x142646960, 0x143cb22d0, 0x143cb22d8, 0x143cb22da, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::crosshair::system>
    Replacement::draft_throws(0x14055bee0, "unknown_14055bee0", fn_14055bee0 as *const (), Throws::Discard), // sub_14055bee0
    Replacement::draft_throws(0x14055c140, "unknown_14055c140", fn_14055c140 as *const (), Throws::Discard), // sub_14055c140
    Replacement::draft(0x14055c390, "unknown_14055c390", fn_14055c390 as *const ()), // sub_14055c390
    Replacement::draft_throws(0x14055c730, "bigfish_14055c730", tpl_3353::<0x143cb2430, 0x143cb22e0, 0x1411e9f50, 0x14055d370, 0x14055da80, 0x14055d5e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ac1c6, 0x142646960, 0x143cb22f0, 0x143cb22f8, 0x143cb22fc, 0x143cb22fe, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::corpse_hiding::update>
    Replacement::draft(0x14055c990, "bigfish_14055c990", tpl_1096::<0x143cb24e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14055c990
    Replacement::draft(0x14055ccc0, "bigfish_14055ccc0", tpl_1094::<0x143cb241c, 0x14055d1f0, 0x14020e8f0> as *const ()), // sub_14055ccc0
    Replacement::draft(0x14055cec0, "bigfish_14055cec0", tpl_1439::<0x8, 0x14055ccc0> as *const ()), // sub_14055cec0
    Replacement::draft_throws(0x14055cef0, "bigfish_14055cef0", tpl_5374::<0x143cb2538, 0x143cb2448, 0x1411e9f50, 0x14055e3f0, 0x143cb2424, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14055dfb0, 0x14055d980, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ac6f6, 0x142646960, 0x143cb2468, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::corpse_hiding::streamIn>
    Replacement::draft(0x14055d1f0, "unknown_14055d1f0", tpl_1888::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431ad160> as *const ()), // sub_14055d1f0
    Replacement::draft_throws(0x14055d370, "unknown_14055d370", fn_14055d370 as *const (), Throws::Discard), // sub_14055d370
    Replacement::draft(0x14055d5e0, "unknown_14055d5e0", fn_14055d5e0 as *const ()), // sub_14055d5e0
    Replacement::draft(0x14055d720, "unknown_14055d720", tpl_3483::<0x18, 0x20, 0x8, 0x10, 0x30, 0x141aa28c0> as *const ()), // sub_14055d720
    Replacement::draft(0x14055d980, "unknown_14055d980", tpl_5377::<0x143c8ceec, 0x142407c80, 0x14055dcc0, 0x580f0, 0x141aa27f0> as *const ()), // sub_14055d980
    Replacement::draft(0x14055da80, "unknown_14055da80", fn_14055da80 as *const ()), // sub_14055da80
    Replacement::draft(0x14055dfb0, "unknown_14055dfb0", tpl_5378::<0x143c8ceec, 0x142407c80, 0x143c8e70c, 0x142489ee0, 0x143c8d160, 0x141aa27f0> as *const ()), // sub_14055dfb0
    Replacement::draft(0x14055e6e0, "unknown_14055e6e0", fn_14055e6e0 as *const ()), // sub_14055e6e0
    Replacement::draft(0x14055e730, "unknown_14055e730", fn_14055e730 as *const ()), // sub_14055e730
    Replacement::draft(0x14055e760, "unknown_14055e760", fn_14055e760 as *const ()), // sub_14055e760
    Replacement::draft_throws(0x14055e7f0, "unknown_14055e7f0", fn_14055e7f0 as *const (), Throws::Discard), // sub_14055e7f0
    Replacement::draft_throws(0x14055e830, "unknown_14055e830", fn_14055e830 as *const (), Throws::Discard), // sub_14055e830
    Replacement::draft(0x14055e990, "unknown_14055e990", tpl_63932::<0x14055eac0, 0x1426467e8, 0x14055eab1> as *const ()), // sub_14055e990
    Replacement::draft(0x14055ecc0, "unknown_14055ecc0", tpl_61197::<0x143c9ea00, 0x142489ee0, 0x143c8d68c, 0x143c964c0, 0x143c8d160, 0x1426467e8, 0x14055ee53> as *const ()), // sub_14055ecc0
    Replacement::draft_throws(0x14055ee60, "unknown_14055ee60", tpl_1480::<0x142762c30, 0x1410d6ca0, 0x14055fb80, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14055ee60
    Replacement::draft_throws(0x14055f100, "bigfish_14055f100", tpl_1298::<0x143cb260c, 0x143cb2630, 0x1411e9f50, 0x1405600d0, 0x140560ea0, 0x140560380, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ad3b6, 0x142646960, 0x143cb2650, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::systems::input_actions::update>
    Replacement::draft_throws(0x14055f340, "bigfish_14055f340", fn_14055f340 as *const (), Throws::Discard), // sub_14055f340
    Replacement::draft(0x14055f420, "bigfish_14055f420", tpl_1094::<0x143c9ea00, 0x14055ff50, 0x14020e8f0> as *const ()), // sub_14055f420
    Replacement::draft(0x14055f620, "bigfish_14055f620", tpl_1094::<0x143cb2908, 0x140467f50, 0x14020e8f0> as *const ()), // sub_14055f620
    Replacement::draft(0x14055f820, "bigfish_14055f820", tpl_1096::<0x143cb28c4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14055f820
    Replacement::draft_throws(0x14055fb80, "bigfish_14055fb80", tpl_1298::<0x143cb2690, 0x143cb27e8, 0x1411e9f50, 0x1405606f0, 0x1405614b0, 0x140560ab0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ad986, 0x142646960, 0x143cb2808, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::systems::indicators::syncFacts>
    Replacement::draft_throws(0x14055fdc0, "unknown_14055fdc0", tpl_5384::<0x14241c0a0, 0x14240a730, 0x14021e830, 0x14240ab10, 0x14241c190> as *const (), Throws::Discard), // sub_14055fdc0
    Replacement::draft(0x14055ff50, "unknown_14055ff50", tpl_3091::<0x1401e6d50, 0x1405606c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1403f4f50> as *const ()), // sub_14055ff50
    Replacement::draft_throws(0x1405600d0, "unknown_1405600d0", fn_1405600d0 as *const (), Throws::Discard), // sub_1405600d0
    Replacement::draft_throws(0x140560380, "unknown_140560380", fn_140560380 as *const (), Throws::Discard), // sub_140560380
    Replacement::draft_throws(0x1405604e0, "unknown_1405604e0", fn_1405604e0 as *const (), Throws::Discard), // sub_1405604e0
    Replacement::draft(0x1405606c0, "unknown_1405606c0", tpl_3160::<0x130, 0x1403f4ee0> as *const ()), // sub_1405606c0
    Replacement::draft_throws(0x1405606f0, "unknown_1405606f0", fn_1405606f0 as *const (), Throws::Discard), // sub_1405606f0
    Replacement::draft_throws(0x140560ab0, "unknown_140560ab0", fn_140560ab0 as *const (), Throws::Discard), // sub_140560ab0
    Replacement::draft(0x140560b70, "unknown_140560b70", fn_140560b70 as *const ()), // sub_140560b70
    Replacement::draft_throws(0x140560cf0, "unknown_140560cf0", fn_140560cf0 as *const (), Throws::Discard), // sub_140560cf0
    Replacement::draft_throws(0x140560ea0, "unknown_140560ea0", fn_140560ea0 as *const (), Throws::Discard), // sub_140560ea0
    Replacement::draft_throws(0x1405614b0, "unknown_1405614b0", fn_1405614b0 as *const (), Throws::Discard), // sub_1405614b0
    Replacement::draft(0x140561920, "unknown_140561920", fn_140561920 as *const ()), // sub_140561920
    Replacement::draft(0x140561a50, "unknown_140561a50", tpl_1769 as *const ()), // sub_140561a50
    Replacement::draft(0x140561a80, "unknown_140561a80", fn_140561a80 as *const ()), // sub_140561a80
    Replacement::draft(0x140561ba0, "unknown_140561ba0", fn_140561ba0 as *const ()), // sub_140561ba0
    Replacement::draft(0x140561d30, "unknown_140561d30", fn_140561d30 as *const ()), // sub_140561d30
    Replacement::draft(0x140562080, "unknown_140562080", fn_140562080 as *const ()), // sub_140562080
    Replacement::draft(0x1405620f0, "unknown_1405620f0", tpl_3715::<0x3, 0x4, 0x8, 0x14240a730, 0x28, 0x30, 0x14025f9d0, 0x10, 0x18> as *const ()), // sub_1405620f0
    Replacement::draft(0x140562210, "unknown_140562210", fn_140562210 as *const ()), // sub_140562210
    Replacement::draft(0x1405622b0, "unknown_1405622b0", fn_1405622b0 as *const ()), // sub_1405622b0
    Replacement::draft(0x140562600, "unknown_140562600", tpl_3717::<0x3, 0x4, 0x8, 0x14240aa00, 0x28, 0x14025f9d0, 0x10, 0x18, 0x140562550> as *const ()), // sub_140562600
    Replacement::draft(0x140562730, "unknown_140562730", fn_140562730 as *const ()), // sub_140562730
    Replacement::draft_throws(0x140562bb0, "bigfish_140562bb0", tpl_3002::<0x143cb2b30, 0x143cb2a30, 0x1411e9f50, 0x140563c60, 0x1405645f0, 0x140563e80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431af076, 0x142646960, 0x143cb2a50, 0x143cb2a60, 0x143cb2a64, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_status::update_combat_status::system>
    Replacement::draft_throws(0x140562e00, "bigfish_140562e00", fn_140562e00 as *const (), Throws::Discard), // sub_140562e00
    Replacement::draft(0x140562f50, "bigfish_140562f50", tpl_1094::<0x143cb2c50, 0x140563ac0, 0x14020e8f0> as *const ()), // sub_140562f50
    Replacement::draft(0x140563150, "bigfish_140563150", tpl_1098::<0x143c976e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140563150
    Replacement::draft_throws(0x1405634c0, "bigfish_1405634c0", tpl_1437::<0x143cb2b28, 0x143cb2af8, 0x1411e9f50, 0x140565180, 0x143cb2af0, 0x1420af120, 0x1420a28e0, 0x1405649b0, 0x1405643a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431afad6, 0x142646960, 0x143cb2b18, 0x143cb2b20, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_status::stream_in::system>
    Replacement::draft_throws(0x1405637a0, "bigfish_1405637a0", tpl_5017::<0x143cb2b34, 0x143cb2c20, 0x1411e9f50, 0x140565090, 0x143cb2d70, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x140564b10, 0x1405644b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431af886, 0x142646960, 0x143cb2c40, 0x143cb2c48, 0x143cb2c4a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_status::stream_out::system>
    Replacement::draft(0x140563ac0, "unknown_140563ac0", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431aeee0, 0x140244c00> as *const ()), // sub_140563ac0
    Replacement::draft_throws(0x140563c60, "unknown_140563c60", fn_140563c60 as *const (), Throws::Discard), // sub_140563c60
    Replacement::draft_throws(0x140563e80, "unknown_140563e80", fn_140563e80 as *const (), Throws::Discard), // sub_140563e80
    Replacement::draft_throws(0x140563fc0, "unknown_140563fc0", fn_140563fc0 as *const (), Throws::Discard), // sub_140563fc0
    Replacement::draft(0x1405643a0, "unknown_1405643a0", tpl_1127::<0x142407c80, 0x143c8de68, 0x142489ee0, 0x141aa8370> as *const ()), // sub_1405643a0
    Replacement::draft(0x1405644b0, "unknown_1405644b0", tpl_1377::<0x142407c80, 0x143c8ceec, 0x143c976e0, 0x142489ee0, 0x141aa8240> as *const ()), // sub_1405644b0
    Replacement::draft_throws(0x1405645f0, "unknown_1405645f0", fn_1405645f0 as *const (), Throws::Discard), // sub_1405645f0
    Replacement::draft(0x1405649b0, "unknown_1405649b0", tpl_1131::<0x142407c80, 0x143c8de68, 0x142489ee0, 0x141aa8370> as *const ()), // sub_1405649b0
    Replacement::draft(0x140564b10, "unknown_140564b10", tpl_1378::<0x142407c80, 0x143c8ceec, 0x143c976e0, 0x142489ee0, 0x141aa8240> as *const ()), // sub_140564b10
    Replacement::draft(0x140564c90, "unknown_140564c90", fn_140564c90 as *const ()), // sub_140564c90
    Replacement::draft(0x140564ed0, "unknown_140564ed0", tpl_3292::<0x143cb2b70, 0x143cb2c60, 0x141834a78, 0x143c976e0, 0x143c8de68, 0x143cb2c50, 0x143c9779c, 0x143c8d160, 0x143c8fab8, 0x143c8d120, 0x143c8d508, 0x143cb2c80, 0x143cb2ca0, 0x143cb2cc0, 0x141834a0c> as *const ()), // sub_140564ed0
    Replacement::draft(0x1405657d0, "unknown_1405657d0", fn_1405657d0 as *const ()), // sub_1405657d0
    Replacement::draft_throws(0x1405658f0, "bigfish_1405658f0", tpl_1185::<0x143cb3158, 0x143cb2d98, 0x1411e9f50, 0x140566ee0, 0x140567ad0, 0x140567110, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b08b6, 0x142646960, 0x143cb2db8, 0x143cb2dc0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_state_sensor_update::system>
    Replacement::draft_throws(0x140565b40, "bigfish_140565b40", tpl_4527::<0x143cb3164, 0x143cb2ec8, 0x1411e9f50, 0x1405673c0, 0x140567e40, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b14b6, 0x142646960, 0x143cb2ee8, 0x143cb2ef8, 0x143cb2f00, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combate_state_sensor_update_global_data::system>
    Replacement::draft(0x140565d90, "bigfish_140565d90", tpl_1402::<0x143cb301c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140565d90
    Replacement::draft(0x1405660e0, "bigfish_1405660e0", tpl_1436::<0x58, 0x60, 0x158, 0x143cb301c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405660e0
    Replacement::draft(0x1405661c0, "bigfish_1405661c0", tpl_1094::<0x143c9b508, 0x140566d50, 0x14020e8f0> as *const ()), // sub_1405661c0
    Replacement::draft(0x1405663c0, "bigfish_1405663c0", tpl_1097::<0x143c8fab0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405663c0
    Replacement::draft_throws(0x1405666f0, "bigfish_1405666f0", tpl_1101::<0x143cb3020, 0x143cb2f78, 0x1411e9f50, 0x1405688e0, 0x143cb2d88, 0x1420af120, 0x1420a28e0, 0x140567fc0, 0x140567970, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b1116, 0x142646960, 0x143cb2f98, 0x143cb2fa0, 0x143cb2fa4, 0x143cb2fa6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_state_sensor_stream_in::system>
    Replacement::draft_throws(0x1405669f0, "bigfish_1405669f0", tpl_1102::<0x143cb3060, 0x143cb2f08, 0x1411e9f50, 0x1405687f0, 0x143cb2d84, 0x1420af120, 0x1420a28e0, 0x1405680d0, 0x140567a30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b1316, 0x142646960, 0x143cb2f28, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_state_sensor_stream_out::system>
    Replacement::draft(0x140566cd0, "unknown_140566cd0", tpl_5401::<0x8, 0x1405661c0, 0x1405663c0> as *const ()), // sub_140566cd0
    Replacement::draft(0x140566d50, "unknown_140566d50", tpl_1844::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1430fddd0> as *const ()), // sub_140566d50
    Replacement::draft_throws(0x140566ee0, "unknown_140566ee0", fn_140566ee0 as *const (), Throws::Discard), // sub_140566ee0
    Replacement::draft_throws(0x140567110, "unknown_140567110", fn_140567110 as *const (), Throws::Discard), // sub_140567110
    Replacement::draft_throws(0x140567250, "unknown_140567250", fn_140567250 as *const (), Throws::Discard), // sub_140567250
    Replacement::draft_throws(0x1405673c0, "unknown_1405673c0", fn_1405673c0 as *const (), Throws::Discard), // sub_1405673c0
    Replacement::draft(0x140567970, "unknown_140567970", tpl_1126::<0x142407c80, 0x580f0, 0x141aaa230> as *const ()), // sub_140567970
    Replacement::draft(0x140567a30, "unknown_140567a30", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1405660e0> as *const ()), // sub_140567a30
    Replacement::draft_throws(0x140567ad0, "unknown_140567ad0", fn_140567ad0 as *const (), Throws::Discard), // sub_140567ad0
    Replacement::draft_throws(0x140567e40, "unknown_140567e40", tpl_4147::<0x143cb3018, 0x143c8ceec, 0x20, 0x143c8fd28, 0x8, 0x580e0, 0x143c8ce88, 0x141aaa2d0> as *const (), Throws::Discard), // sub_140567e40
    Replacement::draft(0x140567fc0, "unknown_140567fc0", tpl_1130::<0x142407c80, 0x141aaa230> as *const ()), // sub_140567fc0
    Replacement::draft(0x1405680d0, "unknown_1405680d0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405660e0> as *const ()), // sub_1405680d0
    Replacement::draft(0x1405681b0, "unknown_1405681b0", tpl_1472::<0x143cb2ec0, 0x143cb3100, 0x141834a78, 0x143c8f4d4, 0x143c8ff58, 0x143c8fab0, 0x143c8e70c, 0x143c8d160, 0x143cb301c, 0x143c8f4d0, 0x143cb3120, 0x143cb3140, 0x143cb3150, 0x141834a0c> as *const ()), // sub_1405681b0
    Replacement::draft(0x140568540, "unknown_140568540", fn_140568540 as *const ()), // sub_140568540
    Replacement::draft(0x140568d90, "content_140568d90", fn_140568d90 as *const ()), // content::CombatEncounterComponent::content::CombatEncounterComponent::vfunc_20
    Replacement::draft(0x140568de0, "unknown_140568de0", fn_140568de0 as *const ()), // sub_140568de0
    Replacement::draft(0x140568f00, "unknown_140568f00", fn_140568f00 as *const ()), // sub_140568f00
    Replacement::draft(0x140569130, "unknown_140569130", fn_140569130 as *const ()), // sub_140569130
    Replacement::draft_throws(0x140569380, "bigfish_140569380", tpl_1396::<0x143cb36ac, 0x143cb3890, 0x1411e9f50, 0x14056b9c0, 0x14056c770, 0x14056bc20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b3436, 0x142646960, 0x143cb38b0, 0x143cb38b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::update::system>
    Replacement::draft_throws(0x1405695d0, "bigfish_1405695d0", tpl_1533::<0x143cb3494, 0x143cb3640, 0x1411e9f50, 0x14056bfa0, 0x14056ca10, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b2b86, 0x142646960, 0x143cb3660, 0x143cb3668, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::reviving::system>
    Replacement::draft_throws(0x140569820, "bigfish_140569820", tpl_1291::<0x143cb337c, 0x143cb3858, 0x1411e9f50, 0x14056c120, 0x14056cad0, 0x14056c310, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b3046, 0x142646960, 0x143cb3878, 0x143cb3880, 0x143cb3884, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::activation::system>
    Replacement::draft(0x140569a70, "bigfish_140569a70", fn_140569a70 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::query_removed_encounter::system>
    Replacement::draft(0x140569c90, "bigfish_140569c90", tpl_5412::<0x1402250d0, 0x14240a730, 0x14056b620, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_140569c90
    Replacement::draft(0x140569e80, "bigfish_140569e80", tpl_1095::<0x143cb3888, 0x14020e8f0> as *const ()), // sub_140569e80
    Replacement::draft(0x14056a060, "bigfish_14056a060", fn_14056a060 as *const ()), // sub_14056a060
    Replacement::draft(0x14056a290, "bigfish_14056a290", tpl_1369::<0x143cb3734, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14056a290
    Replacement::draft(0x14056a5d0, "bigfish_14056a5d0", tpl_1436::<0x58, 0x60, 0x158, 0x143cb3734, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14056a5d0
    Replacement::draft(0x14056a6b0, "bigfish_14056a6b0", tpl_1436::<0x58, 0x60, 0x158, 0x143cb3888, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14056a6b0
    Replacement::draft(0x14056a790, "bigfish_14056a790", tpl_1403::<0x143cb3490, 0x14056b6b0, 0x14020e8f0> as *const ()), // sub_14056a790
    Replacement::draft(0x14056a970, "bigfish_14056a970", tpl_1094::<0x143cb3548, 0x14056b830, 0x14020e8f0> as *const ()), // sub_14056a970
    Replacement::draft_throws(0x14056ab70, "bigfish_14056ab70", tpl_3691::<0x143cb31ac, 0x143cb34a0, 0x1411e9f50, 0x14056e4a0, 0x143cb32b4, 0x1420af120, 0x1420a28e0, 0x14056d650, 0x14056cdb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b39a6, 0x142646960, 0x143cb34c0, 0x143cb34d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::stream_in_avatar::system>
    Replacement::draft_throws(0x14056ae50, "bigfish_14056ae50", tpl_3407::<0x143cb354c, 0x143cb35d0, 0x1411e9f50, 0x14056e3b0, 0x143cb349c, 0x1420af120, 0x1420a28e0, 0x14056d760, 0x14056ce70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b3de6, 0x142646960, 0x143cb35f0, 0x143cb3600, 0x143cb3604, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::stream_out_avatar::system>
    Replacement::draft_throws(0x14056b140, "bigfish_14056b140", tpl_3691::<0x143cb33f4, 0x143cb36f8, 0x1411e9f50, 0x14056e2c0, 0x143cb3450, 0x1420af120, 0x1420a28e0, 0x14056d840, 0x14056cf10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b3f96, 0x142646960, 0x143cb3718, 0x143cb3728, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::combat_encounter::stream_out_enemy::system>
    Replacement::draft(0x14056b420, "unknown_14056b420", tpl_1988::<0x1411e9f50, 0x14056d070, 0x143cb3734, 0x1401fc320, 0x143c8ceec, 0x140215610, 0x143cb3334, 0x1420a28e0, 0x14056cfb0, 0x1401de410> as *const ()), // sub_14056b420
    Replacement::draft(0x14056b620, "unknown_14056b620", tpl_5414::<0x8, 0x3, 0x4, 0x14240a730> as *const ()), // sub_14056b620
    Replacement::draft(0x14056b6b0, "unknown_14056b6b0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431b2810> as *const ()), // sub_14056b6b0
    Replacement::draft(0x14056b830, "unknown_14056b830", tpl_1844::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431b29c0> as *const ()), // sub_14056b830
    Replacement::draft_throws(0x14056b9c0, "unknown_14056b9c0", fn_14056b9c0 as *const (), Throws::Discard), // sub_14056b9c0
    Replacement::draft(0x14056bc20, "unknown_14056bc20", fn_14056bc20 as *const ()), // sub_14056bc20
    Replacement::draft(0x14056bd90, "unknown_14056bd90", tpl_1444::<0x8, 0x20, 0x143cb372c, 0x143cb3334, 0x143c9c108, 0x143c8cef8, 0x580e0, 0x143c8ceec, 0x143c8d910> as *const ()), // sub_14056bd90
    Replacement::draft(0x14056be70, "unknown_14056be70", fn_14056be70 as *const ()), // sub_14056be70
    Replacement::draft_throws(0x14056bfa0, "unknown_14056bfa0", tpl_4425::<0x14056dc00, 0x143cb32b8, 0x1420af120, 0x140215750, 0x143cb3334, 0x140215610, 0x143c94b38, 0x1420a28e0> as *const (), Throws::Discard), // sub_14056bfa0
    Replacement::draft_throws(0x14056c120, "unknown_14056c120", tpl_5189::<0x14056dac0, 0x143cb3338, 0x1420af120, 0x14056d9c0, 0x143cb3384, 0x140215750, 0x143cb3334, 0x140215610, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_14056c120
    Replacement::draft(0x14056c310, "unknown_14056c310", tpl_1408::<0x143cb3384, 0x20, 0x8, 0x143c8ceec, 0x143cb3334, 0x14056c510, 0x14056c430> as *const ()), // sub_14056c310
    Replacement::draft(0x14056c430, "unknown_14056c430", tpl_5107::<0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x141aadc90> as *const ()), // sub_14056c430
    Replacement::draft(0x14056c770, "unknown_14056c770", fn_14056c770 as *const ()), // sub_14056c770
    Replacement::draft_throws(0x14056ca10, "unknown_14056ca10", fn_14056ca10 as *const (), Throws::Caught), // sub_14056ca10
    Replacement::draft(0x14056cad0, "unknown_14056cad0", tpl_61144::<0x143cb3384, 0x143cb3334, 0x143c8ceec, 0x143cb3734, 0x142489ee0, 0x143cb3548, 0x143c8d160, 0x141aadc90> as *const ()), // sub_14056cad0
    Replacement::draft(0x14056cdb0, "unknown_14056cdb0", tpl_1126::<0x142407c80, 0x580f0, 0x141aad0a0> as *const ()), // sub_14056cdb0
    Replacement::draft(0x14056ce70, "unknown_14056ce70", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14056a5d0> as *const ()), // sub_14056ce70
    Replacement::draft(0x14056cf10, "unknown_14056cf10", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14056a6b0> as *const ()), // sub_14056cf10
    Replacement::draft(0x14056cfb0, "unknown_14056cfb0", tpl_5049::<0x143cb3734, 0x5e308, 0x580e0, 0x143c8ceec, 0x143cb3334, 0x10, 0x8, 0x18, 0x141aace10, 0x140489dd0> as *const ()), // sub_14056cfb0
    Replacement::draft(0x14056d070, "unknown_14056d070", tpl_3188::<0x14056d920, 0x1431b2978, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14056d070
    Replacement::draft(0x14056d0f0, "unknown_14056d0f0", fn_14056d0f0 as *const ()), // sub_14056d0f0
    Replacement::draft(0x14056d650, "unknown_14056d650", tpl_1130::<0x142407c80, 0x141aad0a0> as *const ()), // sub_14056d650
    Replacement::draft(0x14056d760, "unknown_14056d760", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14056a5d0> as *const ()), // sub_14056d760
    Replacement::draft(0x14056d840, "unknown_14056d840", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14056a6b0> as *const ()), // sub_14056d840
    Replacement::draft(0x14056d920, "unknown_14056d920", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431b2978, 0x8, 0x20> as *const ()), // sub_14056d920
    Replacement::draft(0x14056dc00, "unknown_14056dc00", fn_14056dc00 as *const ()), // sub_14056dc00
    Replacement::draft(0x14056e0c0, "unknown_14056e0c0", fn_14056e0c0 as *const ()), // sub_14056e0c0
    Replacement::draft(0x14056e5a0, "unknown_14056e5a0", fn_14056e5a0 as *const ()), // sub_14056e5a0
    Replacement::draft(0x14056e830, "unknown_14056e830", fn_14056e830 as *const ()), // sub_14056e830
    Replacement::draft(0x14056ea80, "unknown_14056ea80", fn_14056ea80 as *const ()), // sub_14056ea80
    Replacement::draft(0x14056eb30, "unknown_14056eb30", fn_14056eb30 as *const ()), // sub_14056eb30
    Replacement::draft(0x14056ef60, "unknown_14056ef60", fn_14056ef60 as *const ()), // sub_14056ef60
    Replacement::draft(0x14056f240, "unknown_14056f240", tpl_5426::<0x8, 0x14056ef60> as *const ()), // sub_14056f240
    Replacement::draft(0x14056f280, "content_14056f280", fn_14056f280 as *const ()), // content::ClimbableLedgeComponent::content::ClimbableLedgeComponent::vfunc_10
    Replacement::draft_throws(0x14056f5f0, "bigfish_14056f5f0", tpl_5428::<0x143cb3bfc, 0x143cb3bd8, 0x14056fd30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b6606, 0x142646960, 0x143cb3bf8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::climbable_ledge::update>
    Replacement::draft(0x14056f7f0, "bigfish_14056f7f0", tpl_1094::<0x143cb3a34, 0x140570530, 0x14020e8f0> as *const ()), // sub_14056f7f0
    Replacement::draft(0x14056f9f0, "bigfish_14056f9f0", tpl_1098::<0x143cb3a24, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14056f9f0
    Replacement::draft_throws(0x14056fd30, "bigfish_14056fd30", tpl_1187::<0x1411e9f50, 0x140571600, 0x143cb3a1c, 0x1420af120, 0x140571460, 0x143cb3a20, 0x140215750, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x140570d90, 0x1405706c0> as *const (), Throws::Discard), // sub_14056fd30
    Replacement::draft_throws(0x14056ff50, "bigfish_14056ff50", tpl_1078::<0x143cb3c40, 0x143cb3b20, 0x1411e9f50, 0x1405719d0, 0x143cb3a30, 0x1420af120, 0x1420a28e0, 0x140571300, 0x140571000, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b6416, 0x142646960, 0x143cb3b40, 0x143cb3b42, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::climbable_ledge::streamIn>
    Replacement::draft_throws(0x140570230, "bigfish_140570230", tpl_4485::<0x143cb3a28, 0x143cb3bb0, 0x1411e9f50, 0x1405718d0, 0x143cb3a18, 0x1420af120, 0x1420a28e0, 0x140439d20, 0x140439930, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b58d6, 0x142646960, 0x143cb3bd0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::climbable_ledge::streamOut>
    Replacement::draft(0x140570530, "unknown_140570530", fn_140570530 as *const ()), // sub_140570530
    Replacement::draft(0x1405706c0, "unknown_1405706c0", fn_1405706c0 as *const ()), // sub_1405706c0
    Replacement::draft(0x1405707b0, "unknown_1405707b0", tpl_3709::<0x143cb3a20, 0x20, 0x8, 0x143c8f63c, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_1405707b0
    Replacement::draft(0x140570850, "unknown_140570850", fn_140570850 as *const ()), // sub_140570850
    Replacement::draft(0x1405709d0, "unknown_1405709d0", tpl_3150::<0x18, 0x10, 0x8, 0x140570ab0, 0x1402716c0, 0x20, 0x40, 0x60, 0x80> as *const ()), // sub_1405709d0
    Replacement::draft(0x140570d90, "unknown_140570d90", fn_140570d90 as *const ()), // sub_140570d90
    Replacement::draft(0x140571000, "unknown_140571000", tpl_1127::<0x142407c80, 0x143cb3a34, 0x142489ee0, 0x141aaf6f0> as *const ()), // sub_140571000
    Replacement::draft(0x140571300, "unknown_140571300", tpl_1131::<0x142407c80, 0x143cb3a34, 0x142489ee0, 0x141aaf6f0> as *const ()), // sub_140571300
    Replacement::draft(0x140571460, "unknown_140571460", fn_140571460 as *const ()), // sub_140571460
    Replacement::draft(0x140571600, "unknown_140571600", fn_140571600 as *const ()), // sub_140571600
    Replacement::draft(0x140571d10, "unknown_140571d10", fn_140571d10 as *const ()), // sub_140571d10
    Replacement::draft(0x140571d50, "unknown_140571d50", fn_140571d50 as *const ()), // sub_140571d50
    Replacement::draft(0x140571ee0, "unknown_140571ee0", fn_140571ee0 as *const ()), // sub_140571ee0
    Replacement::draft_throws(0x140571f70, "unknown_140571f70", fn_140571f70 as *const (), Throws::Caught), // sub_140571f70
    Replacement::draft(0x1405720a0, "unknown_1405720a0", tpl_1459::<0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_1405720a0
    Replacement::draft(0x1405720f0, "unknown_1405720f0", fn_1405720f0 as *const ()), // sub_1405720f0
    Replacement::draft_throws(0x1405722d0, "bigfish_1405722d0", fn_1405722d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::charm::update_charms::system>
    Replacement::draft_throws(0x140572600, "bigfish_140572600", tpl_1467::<0x143cb3c48, 0x143cb3e30, 0x1411e9f50, 0x1405740d0, 0x143cb3c8c, 0x1420af120, 0x1420a28e0, 0x140573d70, 0x140573160, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b8166, 0x142646960, 0x143cb3e50, 0x143cb3e58, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::charm::update_revive_charm::system>
    Replacement::draft(0x1405728e0, "bigfish_1405728e0", tpl_1097::<0x143c9dc58, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405728e0
    Replacement::draft_throws(0x140572c10, "bigfish_140572c10", tpl_1404::<0x143cb3d98, 0x143cb3e08, 0x1411e9f50, 0x140574520, 0x143cb3d08, 0x1420af120, 0x1420a28e0, 0x140573f10, 0x140573870, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b7176, 0x142646960, 0x143cb3e28, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::charm::stream_in::system>
    Replacement::draft(0x140572ee0, "unknown_140572ee0", fn_140572ee0 as *const ()), // sub_140572ee0
    Replacement::draft(0x140573000, "unknown_140573000", fn_140573000 as *const ()), // sub_140573000
    Replacement::draft(0x140573160, "unknown_140573160", tpl_62531::<0x88, 0x10088, 0x140573290, 0x141ab5820> as *const ()), // sub_140573160
    Replacement::draft(0x1405733f0, "unknown_1405733f0", fn_1405733f0 as *const ()), // sub_1405733f0
    Replacement::draft(0x140573520, "unknown_140573520", fn_140573520 as *const ()), // sub_140573520
    Replacement::draft_throws(0x140573790, "unknown_140573790", tpl_1330::<0x8, 0x140574210, 0x143cb3c88, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140573790
    Replacement::draft(0x140573870, "unknown_140573870", tpl_4522::<0x88, 0x10088, 0x142407c80, 0x180ac, 0x143c8ec68, 0x18098, 0x142489ee0, 0xc03, 0x40, 0x140573980> as *const ()), // sub_140573870
    Replacement::draft(0x140573980, "unknown_140573980", fn_140573980 as *const ()), // sub_140573980
    Replacement::draft(0x140573a70, "unknown_140573a70", fn_140573a70 as *const ()), // sub_140573a70
    Replacement::draft(0x140573d70, "unknown_140573d70", tpl_62567::<0x140573290, 0x141ab5820> as *const ()), // sub_140573d70
    Replacement::draft(0x140573f10, "unknown_140573f10", fn_140573f10 as *const ()), // sub_140573f10
    Replacement::draft(0x1405742c0, "unknown_1405742c0", fn_1405742c0 as *const ()), // sub_1405742c0
    Replacement::draft(0x140574860, "unknown_140574860", fn_140574860 as *const ()), // sub_140574860
    Replacement::draft(0x1405749a0, "unknown_1405749a0", fn_1405749a0 as *const ()), // sub_1405749a0
    Replacement::draft(0x140574a90, "unknown_140574a90", tpl_1094::<0x143c8ec68, 0x140574cc0, 0x14020e8f0> as *const ()), // sub_140574a90
    Replacement::draft(0x140574cc0, "unknown_140574cc0", fn_140574cc0 as *const ()), // sub_140574cc0
    Replacement::draft(0x140574e90, "unknown_140574e90", fn_140574e90 as *const ()), // sub_140574e90
    Replacement::draft(0x140574eb0, "unknown_140574eb0", tpl_4170::<0x140575160, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_140574eb0
    Replacement::draft(0x140574fd0, "unknown_140574fd0", fn_140574fd0 as *const ()), // sub_140574fd0
    Replacement::draft(0x140575160, "unknown_140575160", fn_140575160 as *const ()), // sub_140575160
    Replacement::draft_throws(0x140575210, "bigfish_140575210", fn_140575210 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::charge_attack_sensor::update::system>
    Replacement::draft(0x140575570, "bigfish_140575570", tpl_1094::<0x143cb4008, 0x140576040, 0x14020e8f0> as *const ()), // sub_140575570
    Replacement::draft(0x140575770, "bigfish_140575770", tpl_1436::<0x58, 0x60, 0x158, 0x143cb4008, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140575770
    Replacement::draft(0x140575850, "bigfish_140575850", tpl_1094::<0x143cb4198, 0x1405761c0, 0x14020e8f0> as *const ()), // sub_140575850
    Replacement::draft(0x140575a50, "bigfish_140575a50", tpl_1439::<0x8, 0x140575850> as *const ()), // sub_140575a50
    Replacement::draft_throws(0x140575a80, "bigfish_140575a80", tpl_1100::<0x143cb4044, 0x143cb4010, 0x1411e9f50, 0x140577200, 0x143cb4048, 0x1420af120, 0x1420a28e0, 0x140576d50, 0x1405767c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b8f36, 0x142646960, 0x143cb4030, 0x143cb4040, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::charge_attack_sensor::stream_in::system>
    Replacement::draft_throws(0x140575d60, "bigfish_140575d60", tpl_1074::<0x143cb4190, 0x143cb4058, 0x1411e9f50, 0x140577100, 0x143cb4050, 0x1420af120, 0x1420a28e0, 0x140576e60, 0x140576880, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b8776, 0x142646960, 0x143cb4078, 0x143cb4088, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::charge_attack_sensor::stream_out::system>
    Replacement::draft(0x140576040, "unknown_140576040", fn_140576040 as *const ()), // sub_140576040
    Replacement::draft(0x1405761c0, "unknown_1405761c0", fn_1405761c0 as *const ()), // sub_1405761c0
    Replacement::draft_throws(0x140576350, "unknown_140576350", tpl_5455::<0x88, 0x10088, 0x580e0, 0x143c8fd28, 0x143c8d910, 0x140576530, 0x140576420> as *const (), Throws::Discard), // sub_140576350
    Replacement::draft_throws(0x140576420, "unknown_140576420", tpl_5456::<0x18, 0x20, 0x50, 0x40, 0x8, 0x48, 0x141ab78c0> as *const (), Throws::Discard), // sub_140576420
    Replacement::draft(0x1405767a0, "unknown_1405767a0", fn_1405767a0 as *const ()), // sub_1405767a0
    Replacement::draft(0x1405767c0, "unknown_1405767c0", tpl_1126::<0x142407c80, 0x580f0, 0x141ab7c40> as *const ()), // sub_1405767c0
    Replacement::draft(0x140576880, "unknown_140576880", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140575770> as *const ()), // sub_140576880
    Replacement::draft_throws(0x140576920, "unknown_140576920", tpl_64609::<0x143c8d910, 0x143c8fd28, 0x1405765f0, 0x141ab78c0> as *const (), Throws::Discard), // sub_140576920
    Replacement::draft(0x140577790, "unknown_140577790", fn_140577790 as *const ()), // sub_140577790
    Replacement::draft(0x140577dd0, "unknown_140577dd0", fn_140577dd0 as *const ()), // sub_140577dd0
    Replacement::draft(0x140578100, "unknown_140578100", fn_140578100 as *const ()), // sub_140578100
    Replacement::draft(0x140578190, "unknown_140578190", fn_140578190 as *const ()), // sub_140578190
    Replacement::draft(0x140578300, "unknown_140578300", fn_140578300 as *const ()), // sub_140578300
    Replacement::draft(0x140578480, "unknown_140578480", fn_140578480 as *const ()), // sub_140578480
    Replacement::draft(0x140578600, "unknown_140578600", fn_140578600 as *const ()), // sub_140578600
    Replacement::draft(0x140578780, "unknown_140578780", fn_140578780 as *const ()), // sub_140578780
    Replacement::draft(0x140578900, "unknown_140578900", fn_140578900 as *const ()), // sub_140578900
    Replacement::draft(0x140578a80, "unknown_140578a80", fn_140578a80 as *const ()), // sub_140578a80
    Replacement::draft(0x140578c00, "unknown_140578c00", fn_140578c00 as *const ()), // sub_140578c00
    Replacement::draft(0x140578d80, "unknown_140578d80", fn_140578d80 as *const ()), // sub_140578d80
    Replacement::draft(0x140578f00, "unknown_140578f00", fn_140578f00 as *const ()), // sub_140578f00
    Replacement::draft(0x140579080, "unknown_140579080", fn_140579080 as *const ()), // sub_140579080
    Replacement::draft(0x140579200, "unknown_140579200", fn_140579200 as *const ()), // sub_140579200
    Replacement::draft(0x140579380, "unknown_140579380", fn_140579380 as *const ()), // sub_140579380
    Replacement::draft(0x140579500, "unknown_140579500", fn_140579500 as *const ()), // sub_140579500
    Replacement::draft(0x140579680, "unknown_140579680", fn_140579680 as *const ()), // sub_140579680
    Replacement::draft(0x140579800, "unknown_140579800", fn_140579800 as *const ()), // sub_140579800
    Replacement::draft(0x140579980, "unknown_140579980", fn_140579980 as *const ()), // sub_140579980
    Replacement::draft(0x140579b00, "unknown_140579b00", fn_140579b00 as *const ()), // sub_140579b00
    Replacement::draft(0x140579c80, "unknown_140579c80", fn_140579c80 as *const ()), // sub_140579c80
    Replacement::draft(0x140579e00, "unknown_140579e00", fn_140579e00 as *const ()), // sub_140579e00
    Replacement::draft(0x140579f80, "unknown_140579f80", fn_140579f80 as *const ()), // sub_140579f80
    Replacement::draft(0x14057a100, "unknown_14057a100", fn_14057a100 as *const ()), // sub_14057a100
    Replacement::draft(0x14057a280, "unknown_14057a280", fn_14057a280 as *const ()), // sub_14057a280
    Replacement::draft(0x14057a400, "unknown_14057a400", fn_14057a400 as *const ()), // sub_14057a400
    Replacement::draft(0x14057a580, "unknown_14057a580", fn_14057a580 as *const ()), // sub_14057a580
    Replacement::draft(0x14057a700, "unknown_14057a700", fn_14057a700 as *const ()), // sub_14057a700
    Replacement::draft(0x14057a880, "unknown_14057a880", fn_14057a880 as *const ()), // sub_14057a880
    Replacement::draft(0x14057aa00, "unknown_14057aa00", fn_14057aa00 as *const ()), // sub_14057aa00
    Replacement::draft(0x14057ab80, "unknown_14057ab80", fn_14057ab80 as *const ()), // sub_14057ab80
    Replacement::draft(0x14057ad00, "unknown_14057ad00", fn_14057ad00 as *const ()), // sub_14057ad00
    Replacement::draft(0x14057ae80, "unknown_14057ae80", fn_14057ae80 as *const ()), // sub_14057ae80
    Replacement::draft(0x14057b000, "unknown_14057b000", fn_14057b000 as *const ()), // sub_14057b000
    Replacement::draft(0x14057b180, "unknown_14057b180", fn_14057b180 as *const ()), // sub_14057b180
    Replacement::draft(0x14057b300, "unknown_14057b300", fn_14057b300 as *const ()), // sub_14057b300
    Replacement::draft(0x14057b490, "unknown_14057b490", fn_14057b490 as *const ()), // sub_14057b490
    Replacement::draft(0x14057b610, "unknown_14057b610", fn_14057b610 as *const ()), // sub_14057b610
    Replacement::draft_throws(0x14057b790, "unknown_14057b790", fn_14057b790 as *const (), Throws::Discard), // sub_14057b790
    Replacement::draft(0x14057b7f0, "unknown_14057b7f0", fn_14057b7f0 as *const ()), // sub_14057b7f0
    Replacement::draft_throws(0x14057b900, "bigfish_14057b900", tpl_4201::<0x143cb426c, 0x143cb4568, 0x1411e9f50, 0x140581100, 0x140581a90, 0x1405813b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b9ab6, 0x142646960, 0x143cb4588, 0x143cb4598, 0x143cb45a0, 0x143cb45a2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::character_upgrade::handle_on_kill_upgrade::system>
    Replacement::draft_throws(0x14057bb60, "bigfish_14057bb60", tpl_5172::<0x143cb4854, 0x143cb4618, 0x140580f40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431b9fc6, 0x142646960, 0x143cb4638, 0x143cb4648, 0x143cb4650, 0x143cb4654, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::character_upgrade::handle_definition_change::system>
    Replacement::draft_throws(0x14057bd80, "bigfish_14057bd80", tpl_4193::<0x143cb4690, 0x143cb45a8, 0x1411e9f50, 0x140581660, 0x140581d10, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431bb066, 0x142646960, 0x143cb45c8, 0x143cb45d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::character_upgrade::update_upgrades::system>
    Replacement::draft(0x14057bfd0, "bigfish_14057bfd0", fn_14057bfd0 as *const ()), // sub_14057bfd0
    Replacement::draft(0x14057c030, "bigfish_14057c030", tpl_2403::<0x3, 0x4, 0x8, 0x14240aa00, 0x4c, 0xd8, 0x14057c0d0, 0xe8> as *const ()), // sub_14057c030
    Replacement::draft(0x14057c0d0, "bigfish_14057c0d0", fn_14057c0d0 as *const ()), // sub_14057c0d0
    Replacement::draft(0x14057c220, "bigfish_14057c220", tpl_1097::<0x143c9d1ac, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057c220
    Replacement::draft(0x14057c550, "bigfish_14057c550", tpl_1097::<0x143ca9708, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057c550
    Replacement::draft(0x14057c880, "bigfish_14057c880", tpl_1099::<0x143ca55e8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057c880
    Replacement::draft(0x14057cbc0, "bigfish_14057cbc0", tpl_1097::<0x143ca5314, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057cbc0
    Replacement::draft(0x14057cef0, "bigfish_14057cef0", tpl_1097::<0x143c9ecdc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057cef0
    Replacement::draft(0x14057d220, "bigfish_14057d220", tpl_3074::<0x143c9c01c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057d220
    Replacement::draft(0x14057d4f0, "bigfish_14057d4f0", tpl_3077::<0x143c94e58, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057d4f0
    Replacement::draft(0x14057d820, "bigfish_14057d820", tpl_1099::<0x143c96b20, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057d820
    Replacement::draft(0x14057db60, "bigfish_14057db60", tpl_1096::<0x143c96b14, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057db60
    Replacement::draft(0x14057de90, "bigfish_14057de90", tpl_1096::<0x143ca88c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057de90
    Replacement::draft(0x14057e1c0, "bigfish_14057e1c0", tpl_1096::<0x143ca8d3c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057e1c0
    Replacement::draft(0x14057e4f0, "bigfish_14057e4f0", tpl_3077::<0x143caf9c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057e4f0
    Replacement::draft(0x14057e820, "bigfish_14057e820", tpl_1097::<0x143ca8f88, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057e820
    Replacement::draft(0x14057eb50, "bigfish_14057eb50", tpl_3077::<0x143c9d658, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057eb50
    Replacement::draft(0x14057ee80, "bigfish_14057ee80", tpl_1096::<0x143c9e4a8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057ee80
    Replacement::draft(0x14057f1b0, "bigfish_14057f1b0", tpl_1096::<0x143ca9678, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057f1b0
    Replacement::draft(0x14057f4e0, "bigfish_14057f4e0", tpl_1099::<0x143ca8568, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057f4e0
    Replacement::draft(0x14057f820, "bigfish_14057f820", tpl_3278::<0x143cb11b8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057f820
    Replacement::draft(0x14057fb60, "bigfish_14057fb60", tpl_1096::<0x143c8ed30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057fb60
    Replacement::draft(0x14057fe90, "bigfish_14057fe90", tpl_1096::<0x143cb0930, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14057fe90
    Replacement::draft(0x1405801c0, "bigfish_1405801c0", tpl_3077::<0x143c9eeec, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405801c0
    Replacement::draft(0x1405804f0, "bigfish_1405804f0", tpl_1095::<0x14384bfdc, 0x14020e8f0> as *const ()), // sub_1405804f0
    Replacement::draft(0x1405806d0, "bigfish_1405806d0", tpl_1095::<0x143cb4748, 0x14020e8f0> as *const ()), // sub_1405806d0
    Replacement::draft_throws(0x1405808b0, "bigfish_1405808b0", fn_1405808b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::character_upgrade::stream_in_character_upgrade_only::system>
    Replacement::draft_throws(0x140580bd0, "bigfish_140580bd0", fn_140580bd0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::character_upgrade::stream_in_shared_upgrades::system>
    Replacement::draft_throws(0x140580f40, "unknown_140580f40", tpl_3011::<0x1411e9f50, 0x1405825e0, 0x14384bfd8, 0x1420af120, 0x140215750, 0x143c8d0c8, 0x140215610, 0x1420a28e0, 0x140581c70, 0x1401de410> as *const (), Throws::Discard), // sub_140580f40
    Replacement::draft_throws(0x140581100, "unknown_140581100", fn_140581100 as *const (), Throws::Discard), // sub_140581100
    Replacement::draft(0x1405813b0, "unknown_1405813b0", fn_1405813b0 as *const ()), // sub_1405813b0
    Replacement::draft(0x140581460, "unknown_140581460", fn_140581460 as *const ()), // sub_140581460
    Replacement::draft(0x140581570, "unknown_140581570", fn_140581570 as *const ()), // sub_140581570
    Replacement::draft_throws(0x140581660, "unknown_140581660", fn_140581660 as *const (), Throws::Discard), // sub_140581660
    Replacement::draft(0x140581830, "unknown_140581830", tpl_4168::<0x143c8d0c8, 0x142407c80, 0x143c90164, 0x142489ee0, 0x141abbdc0> as *const ()), // sub_140581830
    Replacement::draft(0x140581970, "unknown_140581970", fn_140581970 as *const ()), // sub_140581970
    Replacement::draft(0x140581a90, "unknown_140581a90", fn_140581a90 as *const ()), // sub_140581a90
    Replacement::draft_throws(0x140581c70, "unknown_140581c70", fn_140581c70 as *const (), Throws::Discard), // sub_140581c70
    Replacement::draft(0x140581d10, "unknown_140581d10", fn_140581d10 as *const ()), // sub_140581d10
    Replacement::draft(0x140582110, "unknown_140582110", tpl_4169::<0x143c8d0c8, 0x142407c80, 0x143c90164, 0x142489ee0, 0x141abbdc0> as *const ()), // sub_140582110
    Replacement::draft(0x140582290, "unknown_140582290", fn_140582290 as *const ()), // sub_140582290
    Replacement::draft(0x140582490, "unknown_140582490", fn_140582490 as *const ()), // sub_140582490
    Replacement::draft(0x1405828d0, "unknown_1405828d0", fn_1405828d0 as *const ()), // sub_1405828d0
    Replacement::draft(0x140582a00, "unknown_140582a00", fn_140582a00 as *const ()), // sub_140582a00
    Replacement::draft(0x140582e30, "unknown_140582e30", fn_140582e30 as *const ()), // sub_140582e30
    Replacement::draft(0x140583040, "unknown_140583040", fn_140583040 as *const ()), // sub_140583040
    Replacement::draft(0x1405833f0, "unknown_1405833f0", fn_1405833f0 as *const ()), // sub_1405833f0
    Replacement::draft(0x1405836b0, "unknown_1405836b0", fn_1405836b0 as *const ()), // sub_1405836b0
    Replacement::draft(0x140583880, "unknown_140583880", tpl_2421::<0x140578100, 0x1405836b0, 0x8, 0xc, 0x14057c030> as *const ()), // sub_140583880
    Replacement::draft(0x140584290, "unknown_140584290", fn_140584290 as *const ()), // sub_140584290
    Replacement::draft(0x140584400, "unknown_140584400", fn_140584400 as *const ()), // sub_140584400
    Replacement::draft(0x1405845e0, "unknown_1405845e0", fn_1405845e0 as *const ()), // sub_1405845e0
    Replacement::draft(0x140584750, "unknown_140584750", fn_140584750 as *const ()), // sub_140584750
    Replacement::draft(0x1405848d0, "unknown_1405848d0", fn_1405848d0 as *const ()), // sub_1405848d0
    Replacement::draft(0x140584a40, "unknown_140584a40", fn_140584a40 as *const ()), // sub_140584a40
    Replacement::draft(0x140584bc0, "unknown_140584bc0", fn_140584bc0 as *const ()), // sub_140584bc0
    Replacement::draft_throws(0x140584d30, "unknown_140584d30", tpl_1480::<0x14276a530, 0x1410d6ca0, 0x1405862f0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140584d30
    Replacement::draft_throws(0x140584fd0, "unknown_140584fd0", tpl_1480::<0x14276a530, 0x1410d6ca0, 0x140586540, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140584fd0
    Replacement::draft_throws(0x140585270, "character_light_rig_140585270", tpl_5490::<0x143cb4a5c, 0x143cb4d70, 0x1411e9f50, 0x140587530, 0x143cb4dcc, 0x1401fc320, 0x140586c40, 0x1420a28e0, 0x1405871b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431be566, 0x142646960, 0x143cb4d90, 0x143cb4d94, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<character_light_rig::lightRigRemoved>
    Replacement::draft(0x140585530, "character_light_rig_140585530", tpl_1456::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_140585530
    Replacement::draft(0x1405855a0, "character_light_rig_1405855a0", tpl_1094::<0x143cb4dbc, 0x140586740, 0x14020e8f0> as *const ()), // sub_1405855a0
    Replacement::draft(0x1405857a0, "character_light_rig_1405857a0", tpl_1095::<0x143cb4c64, 0x14020e8f0> as *const ()), // sub_1405857a0
    Replacement::draft(0x140585980, "character_light_rig_140585980", tpl_3076::<0x143cb4dcc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140585980
    Replacement::draft(0x140585cd0, "character_light_rig_140585cd0", tpl_5491::<0x8, 0x1405855a0, 0x1405857a0> as *const ()), // sub_140585cd0
    Replacement::draft_throws(0x140585d30, "character_light_rig_140585d30", tpl_3552::<0x143cb4ce4, 0x143cb48d8, 0x1411e9f50, 0x140588470, 0x143cb49bc, 0x1420af120, 0x140586c40, 0x1420a28e0, 0x1405877a0, 0x140586d70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431bdad6, 0x142646960, 0x143cb48e8, 0x143cb48f0, 0x143cb48f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<character_light_rig::streamIn>
    Replacement::draft_throws(0x140586010, "character_light_rig_140586010", tpl_5492::<0x143cb4938, 0x143cb4d28, 0x1411e9f50, 0x140588380, 0x143cb4ae8, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1405878f0, 0x140586e70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431bd906, 0x142646960, 0x143cb4d38, 0x143cb4d40, 0x143cb4d44, 0x143cb4d46, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<character_light_rig::streamOut>
    Replacement::draft_throws(0x1405862f0, "character_light_rig_1405862f0", fn_1405862f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<character_light_rig::bind>
    Replacement::draft_throws(0x140586540, "character_light_rig_140586540", fn_140586540 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<character_light_rig::update>
    Replacement::draft(0x140586740, "unknown_140586740", fn_140586740 as *const ()), // sub_140586740
    Replacement::draft_throws(0x1405868c0, "unknown_1405868c0", tpl_3697::<0x1411e9f50, 0x140587e10, 0x143cb4adc, 0x1420af120, 0x140215750, 0x143c904a0, 0x140215610, 0x1420a28e0, 0x140587b00, 0x1401de410> as *const (), Throws::Discard), // sub_1405868c0
    Replacement::draft(0x140586a70, "unknown_140586a70", fn_140586a70 as *const ()), // sub_140586a70
    Replacement::draft(0x140586be0, "content_140586be0", fn_140586be0 as *const ()), // content::CharacterLightRigComponent::content::CharacterLightRigComponent::vfunc_0
    Replacement::draft_throws(0x140586c40, "unknown_140586c40", tpl_1498::<0x143cb4ae4, 0x143cb4c54, 0x10, 0x1420a25b0, 0x141834a78, 0x143cb4a58, 0x143cb4c50, 0x143cb4c58, 0x141834a0c> as *const (), Throws::Discard), // sub_140586c40
    Replacement::draft(0x140586d30, "unknown_140586d30", tpl_1124::<0xf8> as *const ()), // sub_140586d30
    Replacement::draft_throws(0x140586d70, "unknown_140586d70", tpl_62933::<0x143cb4dbc, 0x142489ee0, 0x141ac24e0> as *const (), Throws::Discard), // sub_140586d70
    Replacement::draft(0x140586e70, "unknown_140586e70", tpl_1898::<0x88, 0x10088, 0x141ac23b0> as *const ()), // sub_140586e70
    Replacement::draft_throws(0x140586f00, "unknown_140586f00", fn_140586f00 as *const (), Throws::Discard), // sub_140586f00
    Replacement::draft_throws(0x1405871b0, "unknown_1405871b0", tpl_5499::<0x143cb4dcc, 0x5e308, 0x10, 0x8, 0x18, 0x141ac2300, 0x140489dd0> as *const (), Throws::Discard), // sub_1405871b0
    Replacement::draft(0x140587530, "unknown_140587530", tpl_3188::<0x140587b70, 0x1431bf538, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140587530
    Replacement::draft_throws(0x1405877a0, "unknown_1405877a0", tpl_62934::<0x143cb4dbc, 0x142489ee0, 0x141ac24e0> as *const (), Throws::Discard), // sub_1405877a0
    Replacement::draft(0x1405878f0, "unknown_1405878f0", tpl_61552::<0x8, 0x88, 0x10088, 0x141ac23b0> as *const ()), // sub_1405878f0
    Replacement::draft(0x1405879e0, "unknown_1405879e0", fn_1405879e0 as *const ()), // sub_1405879e0
    Replacement::draft(0x140587b00, "unknown_140587b00", tpl_3028::<0x143cb4adc, 0x20, 0x8, 0x143c904a0, 0x580e0, 0x141ac1850> as *const ()), // sub_140587b00
    Replacement::draft(0x140587b70, "unknown_140587b70", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431bf538, 0x8, 0x20> as *const ()), // sub_140587b70
    Replacement::draft(0x140587c10, "unknown_140587c10", fn_140587c10 as *const ()), // sub_140587c10
    Replacement::draft(0x140587e10, "unknown_140587e10", fn_140587e10 as *const ()), // sub_140587e10
    Replacement::draft(0x140588620, "unknown_140588620", fn_140588620 as *const ()), // sub_140588620
    Replacement::draft(0x140588b20, "unknown_140588b20", tpl_1500::<0x19, 0x141ac0df0, 0x20, 0x10, 0x18, 0x8, 0x140588b20> as *const ()), // sub_140588b20
    Replacement::draft(0x140588c40, "unknown_140588c40", fn_140588c40 as *const ()), // sub_140588c40
    Replacement::draft(0x140588e10, "content_140588e10", fn_140588e10 as *const ()), // content::BoardComponent::content::BoardComponent::vfunc_18
    Replacement::draft(0x140588f30, "unknown_140588f30", tpl_4888::<0x30, 0x143cb5d00, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140588fb3> as *const ()), // sub_140588f30
    Replacement::draft(0x140588fc0, "unknown_140588fc0", fn_140588fc0 as *const ()), // sub_140588fc0
    Replacement::draft(0x140588ff0, "unknown_140588ff0", tpl_3046::<0x58> as *const ()), // sub_140588ff0
    Replacement::draft(0x140589040, "unknown_140589040", tpl_1149::<0x8, 0x14058f060, 0x1401e66f0> as *const ()), // sub_140589040
    Replacement::draft(0x140589070, "unknown_140589070", tpl_1149::<0x8, 0x14058efd0, 0x1401e66f0> as *const ()), // sub_140589070
    Replacement::draft(0x1405890a0, "unknown_1405890a0", tpl_1149::<0x8, 0x14058ef40, 0x1401e66f0> as *const ()), // sub_1405890a0
    Replacement::draft(0x1405890d0, "unknown_1405890d0", fn_1405890d0 as *const ()), // sub_1405890d0
    Replacement::draft(0x1405891f0, "unknown_1405891f0", fn_1405891f0 as *const ()), // sub_1405891f0
    Replacement::draft(0x1405893d0, "unknown_1405893d0", tpl_5510::<0x143c9afe8, 0x142489ee0, 0x143c92bcc, 0x143c9bad4> as *const ()), // sub_1405893d0
    Replacement::draft(0x140589800, "unknown_140589800", fn_140589800 as *const ()), // sub_140589800
    Replacement::draft(0x140589b20, "unknown_140589b20", fn_140589b20 as *const ()), // sub_140589b20
    Replacement::draft(0x140589b90, "unknown_140589b90", fn_140589b90 as *const ()), // sub_140589b90
    Replacement::draft(0x140589d00, "unknown_140589d00", tpl_1171::<0x58, 0x60, 0x143cb52a8, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140589d00
    Replacement::draft(0x140589de0, "unknown_140589de0", fn_140589de0 as *const ()), // sub_140589de0
    Replacement::draft(0x140589ec0, "unknown_140589ec0", fn_140589ec0 as *const ()), // sub_140589ec0
    Replacement::draft(0x140589fd0, "unknown_140589fd0", fn_140589fd0 as *const ()), // sub_140589fd0
    Replacement::draft(0x14058a160, "unknown_14058a160", tpl_63595::<0x14058a2a0, 0x1426467e8, 0x14058a29e> as *const ()), // sub_14058a160
    Replacement::draft(0x14058a4d0, "unknown_14058a4d0", fn_14058a4d0 as *const ()), // sub_14058a4d0
    Replacement::draft(0x14058a550, "unknown_14058a550", fn_14058a550 as *const ()), // sub_14058a550
    Replacement::draft(0x14058a5c0, "unknown_14058a5c0", fn_14058a5c0 as *const ()), // sub_14058a5c0
    Replacement::draft(0x14058a6f0, "unknown_14058a6f0", tpl_63595::<0x1405895d0, 0x1426467e8, 0x14058a82e> as *const ()), // sub_14058a6f0
    Replacement::draft_throws(0x14058a830, "unknown_14058a830", fn_14058a830 as *const (), Throws::Discard), // sub_14058a830
    Replacement::draft(0x14058a9a0, "unknown_14058a9a0", fn_14058a9a0 as *const ()), // sub_14058a9a0
    Replacement::draft(0x14058ab00, "unknown_14058ab00", tpl_61021::<0x30, 0x40, 0x48, 0x14058abc0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x1426467e8, 0x14058abba> as *const ()), // sub_14058ab00
    Replacement::draft(0x14058ad10, "unknown_14058ad10", fn_14058ad10 as *const ()), // sub_14058ad10
    Replacement::draft(0x14058add0, "unknown_14058add0", tpl_1171::<0x58, 0x60, 0x143cb6030, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14058add0
    Replacement::draft(0x14058aeb0, "unknown_14058aeb0", tpl_61019::<0x143c9afe8, 0x142489ee0, 0x143c8d160, 0x143c9b0d4, 0x1426467e8, 0x14058b013> as *const ()), // sub_14058aeb0
    Replacement::draft_throws(0x14058b020, "unknown_14058b020", fn_14058b020 as *const (), Throws::Caught), // sub_14058b020
    Replacement::draft_throws(0x14058b210, "unknown_14058b210", fn_14058b210 as *const (), Throws::Caught), // sub_14058b210
    Replacement::draft(0x14058b410, "unknown_14058b410", fn_14058b410 as *const ()), // sub_14058b410
    Replacement::draft_throws(0x14058b4a0, "bigfish_14058b4a0", tpl_5293::<0x143cb50fc, 0x143cb6068, 0x1411e9f50, 0x1405935c0, 0x14059a320, 0x140593840, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c14e6, 0x142646960, 0x143cb6088, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::initCaseBoard>
    Replacement::draft_throws(0x14058b6e0, "bigfish_14058b6e0", tpl_3242::<0x143cb6250, 0x143cb5320, 0x1411e9f50, 0x1405a0070, 0x143cb5274, 0x1420af120, 0x143c8d110, 0x140215610, 0x1420a28e0, 0x14059a5c0, 0x140593b70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c0ff6, 0x142646960, 0x143cb5340, 0x143cb5348, 0x143cb534c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateStretchyReturnItem>
    Replacement::draft_throws(0x14058ba00, "bigfish_14058ba00", tpl_3085::<0x143cb4ed0, 0x143cb4e10, 0x1411e9f50, 0x14059feb0, 0x143cb620c, 0x1420af120, 0x143c8d110, 0x140215610, 0x1420a28e0, 0x14059a820, 0x140593e10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431bfa46, 0x142646960, 0x143cb4e30, 0x143cb4e34, 0x143cb4e36, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateDraggingItem>
    Replacement::draft_throws(0x14058bd20, "bigfish_14058bd20", tpl_5526::<0x143cb4fe0, 0x143cb55f8, 0x140592340, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431bf7f6, 0x142646960, 0x143cb5618, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateFailedToPlace>
    Replacement::draft_throws(0x14058bf20, "bigfish_14058bf20", tpl_3681::<0x143cb4e44, 0x143cb6258, 0x1411e9f50, 0x140594170, 0x14059abd0, 0x1405943b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c0c76, 0x142646960, 0x143cb6278, 0x143cb627c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::checkCursorInput>
    Replacement::draft_throws(0x14058c160, "bigfish_14058c160", tpl_1291::<0x143cb5d04, 0x143cb56c8, 0x1411e9f50, 0x1405946d0, 0x14059ae10, 0x140594970, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431bfe56, 0x142646960, 0x143cb56e8, 0x143cb56f0, 0x143cb56f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateCaseBoardItemStates>
    Replacement::draft_throws(0x14058c3b0, "bigfish_14058c3b0", tpl_1396::<0x143cb5f04, 0x143cb5800, 0x1411e9f50, 0x140594de0, 0x14059b150, 0x140595050, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c37a6, 0x142646960, 0x143cb5820, 0x143cb5828, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updatePlacingFromHand>
    Replacement::draft_throws(0x14058c600, "bigfish_14058c600", tpl_3005::<0x143cb5c84, 0x143cb55d0, 0x1411e9f50, 0x140595380, 0x14059b440, 0x1405955b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c3326, 0x142646960, 0x143cb55f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateCaseBoard>
    Replacement::draft(0x14058c840, "bigfish_14058c840", fn_14058c840 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::caseboard::accumulateDraggingDelta>
    Replacement::draft_throws(0x14058cb10, "bigfish_14058cb10", tpl_4111::<0x143cb527c, 0x143cb60f8, 0x1411e9f50, 0x140595900, 0x14059b710, 0x140595a90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c4026, 0x142646960, 0x143cb6118, 0x143cb6128, 0x143cb612c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateCaseBoardItemMagnetismZones>
    Replacement::draft(0x14058cd60, "bigfish_14058cd60", tpl_5528::<0x143cb5408, 0x143cb5d50, 0x1411e9f50, 0x143c8d110, 0x140215610, 0x1420a28e0, 0x14059b960, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c2176, 0x142646960, 0x143cb5d70, 0x143cb5d78, 0x143cb5d7c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::caseboard::clearMagnetismUpdateFlag>
    Replacement::draft_throws(0x14058d000, "bigfish_14058d000", tpl_3006::<0x143cb5830, 0x143cb6090, 0x140592500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c1926, 0x142646960, 0x143cb60b0, 0x143cb60b8, 0x143cb60bc, 0x143cb60be, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updatePlacedItemsTransform>
    Replacement::draft_throws(0x14058d220, "bigfish_14058d220", tpl_1535::<0x143cb57dc, 0x143cb54c8, 0x1411e9f50, 0x140596090, 0x14059bca0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c25c6, 0x142646960, 0x143cb54e8, 0x143cb54f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateAnimationQueue>
    Replacement::draft_throws(0x14058d470, "bigfish_14058d470", tpl_5529::<0x143cb5ee0, 0x143cb5180, 0x1411e9f50, 0x14059ea60, 0x143cb526c, 0x1420af120, 0x1405985c0, 0x143c8d110, 0x140215610, 0x1420a28e0, 0x14059bfa0, 0x140596350, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c2206, 0x142646960, 0x143cb51a0, 0x143cb51a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateParentsItemBounds>
    Replacement::draft_throws(0x14058d7a0, "bigfish_14058d7a0", tpl_1428::<0x143cb5530, 0x143cb52b0, 0x1411e9f50, 0x1405964b0, 0x14059c1d0, 0x1405966a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c5a76, 0x142646960, 0x143cb52d0, 0x143cb52d4, 0x143cb52d6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateItemsOverlap>
    Replacement::draft_throws(0x14058d9f0, "bigfish_14058d9f0", tpl_2988::<0x143cb5f9c, 0x143cb61d0, 0x1411e9f50, 0x140596960, 0x14059c420, 0x140596b20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c5546, 0x142646960, 0x143cb61f0, 0x143cb61f8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::adjustTransformDepth>
    Replacement::draft_throws(0x14058dc40, "bigfish_14058dc40", tpl_3059::<0x143cb5758, 0x143cb4fe8, 0x1411e9f50, 0x140596e30, 0x14059c600, 0x140596ff0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c6116, 0x142646960, 0x143cb5008, 0x143cb5018, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::adjustAttachedItemsTransforms>
    Replacement::draft(0x14058de90, "bigfish_14058de90", tpl_5530::<0x143cb52d8, 0x143cb5538, 0x1411e9f50, 0x143c8d110, 0x140215610, 0x1420a28e0, 0x14059c8e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c6076, 0x142646960, 0x143cb5558, 0x143cb5568, 0x143cb556c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::caseboard::clearHasConfigurationChangedState>
    Replacement::draft_throws(0x14058e140, "bigfish_14058e140", tpl_3062::<0x143cb5cfc, 0x143cb5e20, 0x1411e9f50, 0x140597370, 0x14059c900, 0x140597590, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c46e6, 0x142646960, 0x143cb5e40, 0x143cb5e48, 0x143cb5e4c, 0x143cb5e4e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateCaseBoardItemThreads>
    Replacement::draft_throws(0x14058e3a0, "bigfish_14058e3a0", tpl_3068::<0x143cb5398, 0x143cb62c0, 0x1411e9f50, 0x14059dd80, 0x143cb5bc8, 0x1420af120, 0x1420a28e0, 0x14059cb60, 0x1405978c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c44d6, 0x142646960, 0x143cb62e0, 0x143cb62f0, 0x143cb62f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updatePostMotionPinClipVisibility>
    Replacement::draft_throws(0x14058e690, "bigfish_14058e690", tpl_1503::<0x143cb575c, 0x143cb5280, 0x1411e9f50, 0x1405979b0, 0x14059cd10, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431c5316, 0x142646960, 0x143cb52a0, 0x143cb52a4, 0x143cb52a6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updatePendingBlips>
    Replacement::draft(0x14058e8e0, "bigfish_14058e8e0", fn_14058e8e0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::caseboard::updateDebugHints>
    Replacement::draft(0x14058eb60, "bigfish_14058eb60", fn_14058eb60 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::caseboard::syncUIData>
    Replacement::draft_throws(0x14058edc0, "unknown_14058edc0", fn_14058edc0 as *const (), Throws::Discard), // sub_14058edc0
    Replacement::draft(0x14058ef40, "unknown_14058ef40", tpl_5534::<0x19, 0x10, 0x14058ef40, 0x68, 0x14058f2c0, 0x20, 0x14240ab10, 0x1401e66f0> as *const ()), // sub_14058ef40
    Replacement::draft(0x14058efd0, "unknown_14058efd0", tpl_5534::<0x19, 0x10, 0x14058efd0, 0x68, 0x14022a610, 0x20, 0x14240ab10, 0x1401e66f0> as *const ()), // sub_14058efd0
    Replacement::draft(0x14058f060, "unknown_14058f060", tpl_64713::<0x19, 0x10, 0x14058f060, 0x64, 0x14240ab10, 0x1c, 0x1401e66f0> as *const ()), // sub_14058f060
    Replacement::draft(0x14058f0e0, "unknown_14058f0e0", fn_14058f0e0 as *const ()), // sub_14058f0e0
    Replacement::draft(0x14058f160, "unknown_14058f160", fn_14058f160 as *const ()), // sub_14058f160
    Replacement::draft(0x14058f1e0, "unknown_14058f1e0", fn_14058f1e0 as *const ()), // sub_14058f1e0
    Replacement::draft_throws(0x14058f260, "unknown_14058f260", fn_14058f260 as *const (), Throws::Discard), // sub_14058f260
    Replacement::draft(0x14058f2c0, "unknown_14058f2c0", tpl_5539::<0x8, 0x14022a610, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14058f2c0
    Replacement::draft(0x14058f390, "unknown_14058f390", tpl_60953::<0x1426467e8, 0x143c9afe8, 0x142489ee0, 0x14058f40c> as *const ()), // sub_14058f390
    Replacement::draft(0x14058f540, "unknown_14058f540", tpl_1095::<0x143cb557c, 0x14020e8f0> as *const ()), // sub_14058f540
    Replacement::draft(0x14058f720, "unknown_14058f720", fn_14058f720 as *const ()), // sub_14058f720
    Replacement::draft(0x14058fa40, "unknown_14058fa40", fn_14058fa40 as *const ()), // sub_14058fa40
    Replacement::draft(0x14058fc40, "unknown_14058fc40", fn_14058fc40 as *const ()), // sub_14058fc40
    Replacement::draft(0x14058fde0, "unknown_14058fde0", tpl_3073::<0x143cb52a8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14058fde0
    Replacement::draft_throws(0x140590140, "unknown_140590140", fn_140590140 as *const (), Throws::Discard), // sub_140590140
    Replacement::draft_throws(0x1405902c0, "unknown_1405902c0", fn_1405902c0 as *const (), Throws::Discard), // sub_1405902c0
    Replacement::draft_throws(0x140590440, "unknown_140590440", fn_140590440 as *const (), Throws::Discard), // sub_140590440
    Replacement::draft(0x1405905d0, "unknown_1405905d0", fn_1405905d0 as *const ()), // sub_1405905d0
    Replacement::draft(0x140590710, "unknown_140590710", fn_140590710 as *const ()), // sub_140590710
    Replacement::draft(0x140590920, "unknown_140590920", tpl_1466::<0x143cb5f08, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140590920
    Replacement::draft_throws(0x140590bf0, "unknown_140590bf0", fn_140590bf0 as *const (), Throws::Discard), // sub_140590bf0
    Replacement::draft(0x140590d70, "unknown_140590d70", fn_140590d70 as *const ()), // sub_140590d70
    Replacement::draft(0x140590ec0, "unknown_140590ec0", fn_140590ec0 as *const ()), // sub_140590ec0
    Replacement::draft(0x140591250, "unknown_140591250", fn_140591250 as *const ()), // sub_140591250
    Replacement::draft(0x140591580, "unknown_140591580", fn_140591580 as *const ()), // sub_140591580
    Replacement::draft(0x1405916e0, "unknown_1405916e0", fn_1405916e0 as *const ()), // sub_1405916e0
    Replacement::draft(0x140591850, "unknown_140591850", tpl_1095::<0x143cb5a2c, 0x14020e8f0> as *const ()), // sub_140591850
    Replacement::draft(0x140591a30, "unknown_140591a30", fn_140591a30 as *const ()), // sub_140591a30
    Replacement::draft(0x140591c30, "unknown_140591c30", fn_140591c30 as *const ()), // sub_140591c30
    Replacement::draft(0x140591c90, "unknown_140591c90", tpl_1094::<0x143cb6248, 0x140593440, 0x14020e8f0> as *const ()), // sub_140591c90
    Replacement::draft(0x140591e90, "unknown_140591e90", tpl_1466::<0x143cb58bc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140591e90
    Replacement::draft(0x140592160, "unknown_140592160", tpl_1095::<0x143cb54c0, 0x14020e8f0> as *const ()), // sub_140592160
    Replacement::draft_throws(0x140592340, "unknown_140592340", tpl_5552::<0x1411e9f50, 0x14059fdb0, 0x143cb5a98, 0x1420af120, 0x143c8d110, 0x140215610, 0x143c8d0d4, 0x1420a28e0, 0x14059aa20, 0x140594030> as *const (), Throws::Discard), // sub_140592340
    Replacement::draft_throws(0x140592500, "unknown_140592500", tpl_5553::<0x1411e9f50, 0x14059ede0, 0x143cb6130, 0x1420af120, 0x14036fad0, 0x14036e720, 0x143c8d0d4, 0x140215610, 0x143c8d110, 0x1420a28e0, 0x14059b990, 0x140595d30> as *const (), Throws::Discard), // sub_140592500
    Replacement::draft(0x1405926e0, "unknown_1405926e0", tpl_5554::<0x8, 0x140591c90, 0x140591e90, 0x140592160> as *const ()), // sub_1405926e0
    Replacement::draft(0x140592780, "unknown_140592780", fn_140592780 as *const ()), // sub_140592780
    Replacement::draft(0x140592820, "unknown_140592820", tpl_3964::<0x140409f80, 0x1403d4e60, 0x14040f020, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_140592820
    Replacement::draft(0x1405929e0, "unknown_1405929e0", tpl_60530::<0x1426467e8, 0x143c8d120, 0x142489ee0, 0x143cb5358, 0x140592a09> as *const ()), // sub_1405929e0
    Replacement::draft(0x140592ba0, "unknown_140592ba0", fn_140592ba0 as *const ()), // sub_140592ba0
    Replacement::draft(0x140592e70, "unknown_140592e70", fn_140592e70 as *const ()), // sub_140592e70
    Replacement::draft(0x140593170, "unknown_140593170", fn_140593170 as *const ()), // sub_140593170
    Replacement::draft(0x140593440, "unknown_140593440", tpl_5241::<0x1401e6d50, 0x14059a2e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140597c80> as *const ()), // sub_140593440
    Replacement::draft_throws(0x1405935c0, "unknown_1405935c0", fn_1405935c0 as *const (), Throws::Discard), // sub_1405935c0
    Replacement::draft_throws(0x140593840, "unknown_140593840", tpl_4117::<0x88, 0x10088, 0x140593910, 0x140599bf0, 0x140593a30> as *const (), Throws::Discard), // sub_140593840
    Replacement::draft(0x140593910, "unknown_140593910", fn_140593910 as *const ()), // sub_140593910
    Replacement::draft_throws(0x140593a30, "unknown_140593a30", fn_140593a30 as *const (), Throws::Discard), // sub_140593a30
    Replacement::draft(0x140593b70, "unknown_140593b70", fn_140593b70 as *const ()), // sub_140593b70
    Replacement::draft(0x140593c80, "unknown_140593c80", fn_140593c80 as *const ()), // sub_140593c80
    Replacement::draft(0x140593e10, "unknown_140593e10", fn_140593e10 as *const ()), // sub_140593e10
    Replacement::draft(0x140593f20, "unknown_140593f20", fn_140593f20 as *const ()), // sub_140593f20
    Replacement::draft(0x140594030, "unknown_140594030", tpl_5561::<0x143c8d110, 0x143c8d0d4, 0x142407c80, 0x143c8d160, 0x142489ee0, 0x141ac9410> as *const ()), // sub_140594030
    Replacement::draft_throws(0x140594170, "unknown_140594170", fn_140594170 as *const (), Throws::Discard), // sub_140594170
    Replacement::draft(0x1405943b0, "unknown_1405943b0", tpl_5563::<0x88, 0x10088, 0x1405944c0, 0x143c9d618, 0x180ac, 0x18098, 0x142489ee0, 0xc03, 0x40, 0x1405945d0> as *const ()), // sub_1405943b0
    Replacement::draft(0x1405944c0, "unknown_1405944c0", fn_1405944c0 as *const ()), // sub_1405944c0
    Replacement::draft(0x1405945d0, "unknown_1405945d0", fn_1405945d0 as *const ()), // sub_1405945d0
    Replacement::draft_throws(0x1405946d0, "unknown_1405946d0", fn_1405946d0 as *const (), Throws::Discard), // sub_1405946d0
    Replacement::draft_throws(0x140594970, "unknown_140594970", fn_140594970 as *const (), Throws::Discard), // sub_140594970
    Replacement::draft(0x140594a50, "unknown_140594a50", fn_140594a50 as *const ()), // sub_140594a50
    Replacement::draft_throws(0x140594c20, "unknown_140594c20", fn_140594c20 as *const (), Throws::Discard), // sub_140594c20
    Replacement::draft_throws(0x140594de0, "unknown_140594de0", fn_140594de0 as *const (), Throws::Discard), // sub_140594de0
    Replacement::draft_throws(0x140595050, "unknown_140595050", fn_140595050 as *const (), Throws::Discard), // sub_140595050
    Replacement::draft(0x140595130, "unknown_140595130", tpl_5104::<0x143cb5278, 0x20, 0x8, 0x143c8d110, 0x580e0, 0x143c8d118, 0x143c8ceec, 0x143c8d0d4, 0x142407c80, 0x580f0, 0x10, 0x18> as *const ()), // sub_140595130
    Replacement::draft_throws(0x140595210, "unknown_140595210", fn_140595210 as *const (), Throws::Discard), // sub_140595210
    Replacement::draft_throws(0x140595380, "unknown_140595380", fn_140595380 as *const (), Throws::Discard), // sub_140595380
    Replacement::draft_throws(0x1405955b0, "unknown_1405955b0", tpl_4117::<0x88, 0x10088, 0x140595680, 0x140598be0, 0x1405957c0> as *const (), Throws::Discard), // sub_1405955b0
    Replacement::draft(0x140595680, "unknown_140595680", tpl_5574::<0x143c96518, 0x143c982b4, 0x143c8d110, 0x143c8d118, 0x143c8d0d4, 0x143c8cef8, 0x143c8ceec, 0x142407c80> as *const ()), // sub_140595680
    Replacement::draft_throws(0x1405957c0, "unknown_1405957c0", fn_1405957c0 as *const (), Throws::Discard), // sub_1405957c0
    Replacement::draft_throws(0x140595900, "unknown_140595900", fn_140595900 as *const (), Throws::Discard), // sub_140595900
    Replacement::draft(0x140595a90, "unknown_140595a90", tpl_4873::<0x143c8ced4, 0x143c8d110, 0x143c8d118, 0x143c8d0d4, 0x140598a90, 0x140595c10> as *const ()), // sub_140595a90
    Replacement::draft(0x140595c10, "unknown_140595c10", tpl_4874::<0x18, 0x20, 0x40, 0x28, 0x10, 0x8, 0x141acdfd0> as *const ()), // sub_140595c10
    Replacement::draft(0x140595d30, "unknown_140595d30", fn_140595d30 as *const ()), // sub_140595d30
    Replacement::draft(0x140595e10, "unknown_140595e10", tpl_5578::<0x20, 0x143c94b44, 0x8, 0x143c94b30, 0x143c8d0d4, 0x580e0, 0x143c8d110, 0x142407c80, 0x580f0, 0x10, 0x18, 0x28> as *const ()), // sub_140595e10
    Replacement::draft(0x140595f10, "unknown_140595f10", fn_140595f10 as *const ()), // sub_140595f10
    Replacement::draft_throws(0x140596090, "unknown_140596090", fn_140596090 as *const (), Throws::Discard), // sub_140596090
    Replacement::draft(0x140596350, "unknown_140596350", tpl_62526::<0x143cb5d10, 0x143c8d110, 0x1405984c0, 0x141acb750> as *const ()), // sub_140596350
    Replacement::draft_throws(0x1405964b0, "unknown_1405964b0", tpl_3790::<0x14059e830, 0x143cb540c, 0x1420af120, 0x14059e720, 0x143cb5e90, 0x140215750, 0x1405985c0, 0x143c8d0d4, 0x140215610, 0x143c8d110, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405964b0
    Replacement::draft(0x1405966a0, "unknown_1405966a0", tpl_1316::<0x88, 0x10088, 0x140596760, 0x1405984c0, 0x140596860> as *const ()), // sub_1405966a0
    Replacement::draft(0x140596760, "unknown_140596760", tpl_5578::<0x20, 0x143cb5e90, 0x8, 0x143cb5d10, 0x143c8d0d4, 0x580e0, 0x143c8d110, 0x142407c80, 0x580f0, 0x10, 0x18, 0x28> as *const ()), // sub_140596760
    Replacement::draft(0x140596860, "unknown_140596860", fn_140596860 as *const ()), // sub_140596860
    Replacement::draft_throws(0x140596960, "unknown_140596960", tpl_5581::<0x14059e670, 0x143cb5f90, 0x1420af120, 0x14059e520, 0x143cb5258, 0x140215750, 0x14036fad0, 0x14036e720, 0x143c8d110, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_140596960
    Replacement::draft(0x140596b20, "unknown_140596b20", tpl_5563::<0x88, 0x10088, 0x140596c30, 0x143cb5940, 0x180ac, 0x18098, 0x142489ee0, 0xc03, 0x40, 0x140596d20> as *const ()), // sub_140596b20
    Replacement::draft(0x140596c30, "unknown_140596c30", tpl_4519::<0x8, 0x20, 0x143cb5258, 0x143c94b44, 0x143c94b30, 0x143c8d110, 0x580e0> as *const ()), // sub_140596c30
    Replacement::draft(0x140596d20, "unknown_140596d20", fn_140596d20 as *const ()), // sub_140596d20
    Replacement::draft_throws(0x140596e30, "unknown_140596e30", tpl_5581::<0x14059e2c0, 0x143cb5260, 0x1420af120, 0x14059e180, 0x143cb5d0c, 0x140215750, 0x14036fad0, 0x14036e720, 0x143c8d110, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_140596e30
    Replacement::draft(0x140596ff0, "unknown_140596ff0", tpl_5583::<0x88, 0x10088, 0x1405970d0, 0x140598170, 0x1405971f0> as *const ()), // sub_140596ff0
    Replacement::draft(0x1405970d0, "unknown_1405970d0", tpl_5584::<0x20, 0x143cb5d0c, 0x8, 0x143c94b44, 0x143c94b30, 0x143c8d110, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_1405970d0
    Replacement::draft(0x1405971f0, "unknown_1405971f0", fn_1405971f0 as *const ()), // sub_1405971f0
    Replacement::draft_throws(0x140597370, "unknown_140597370", tpl_3099::<0x14059dfe0, 0x143cb5268, 0x1420af120, 0x14059de90, 0x143cb56c4, 0x140215750, 0x140599450, 0x14036fad0, 0x143c8d110, 0x140215610, 0x143c8d08c, 0x143c904a0, 0x1420a28e0> as *const (), Throws::Discard), // sub_140597370
    Replacement::draft(0x140597590, "unknown_140597590", tpl_4250::<0x88, 0x10088, 0x140597660, 0x140597f50, 0x140597770> as *const ()), // sub_140597590
    Replacement::draft(0x140597660, "unknown_140597660", tpl_3101::<0x8, 0x20, 0x143cb56c4, 0x143c904a0, 0x143cb57ec, 0x143c94b44, 0x580e0, 0x143c8d110, 0x143c8d08c> as *const ()), // sub_140597660
    Replacement::draft(0x140597770, "unknown_140597770", fn_140597770 as *const ()), // sub_140597770
    Replacement::draft(0x1405978c0, "unknown_1405978c0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140597e50, 0x580f0, 0x141ac9230> as *const ()), // sub_1405978c0
    Replacement::draft_throws(0x1405979b0, "unknown_1405979b0", tpl_3660::<0x143c8d110, 0x140215610, 0x14059dc70, 0x143cb5270, 0x1420af120, 0x140215750, 0x143c9b0d0, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405979b0
    Replacement::draft(0x140597b30, "unknown_140597b30", fn_140597b30 as *const ()), // sub_140597b30
    Replacement::draft(0x140597c80, "unknown_140597c80", fn_140597c80 as *const ()), // sub_140597c80
    Replacement::draft(0x140597de0, "content_140597de0", fn_140597de0 as *const ()), // content::BoardComponent::content::BoardComponent::vfunc_0
    Replacement::draft_throws(0x1405985c0, "unknown_1405985c0", tpl_1330::<0x8, 0x14059e970, 0x143cb5d10, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405985c0
    Replacement::draft_throws(0x140599450, "unknown_140599450", tpl_1330::<0x8, 0x14059f730, 0x143cb57ec, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140599450
    Replacement::draft_throws(0x140599530, "unknown_140599530", tpl_1330::<0x8, 0x14059fc10, 0x143cb54b8, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140599530
    Replacement::draft(0x140599880, "unknown_140599880", fn_140599880 as *const ()), // sub_140599880
    Replacement::draft(0x140599990, "unknown_140599990", fn_140599990 as *const ()), // sub_140599990
    Replacement::draft(0x140599d30, "unknown_140599d30", fn_140599d30 as *const ()), // sub_140599d30
    Replacement::draft(0x140599e00, "unknown_140599e00", fn_140599e00 as *const ()), // sub_140599e00
    Replacement::draft(0x140599f10, "unknown_140599f10", fn_140599f10 as *const ()), // sub_140599f10
    Replacement::draft(0x140599ff0, "unknown_140599ff0", fn_140599ff0 as *const ()), // sub_140599ff0
    Replacement::draft(0x1405a4bf0, "unknown_1405a4bf0", fn_1405a4bf0 as *const ()), // sub_1405a4bf0
    Replacement::draft(0x1405a4d20, "unknown_1405a4d20", fn_1405a4d20 as *const ()), // sub_1405a4d20
    Replacement::draft(0x1405a4e10, "unknown_1405a4e10", fn_1405a4e10 as *const ()), // sub_1405a4e10
    Replacement::draft(0x1405a5010, "unknown_1405a5010", fn_1405a5010 as *const ()), // sub_1405a5010
    Replacement::draft(0x1405a5220, "unknown_1405a5220", fn_1405a5220 as *const ()), // sub_1405a5220
    Replacement::draft(0x1405a5570, "unknown_1405a5570", fn_1405a5570 as *const ()), // sub_1405a5570
    Replacement::draft(0x1405a5890, "unknown_1405a5890", fn_1405a5890 as *const ()), // sub_1405a5890
    Replacement::draft(0x1405a5ab0, "unknown_1405a5ab0", fn_1405a5ab0 as *const ()), // sub_1405a5ab0
    Replacement::draft(0x1405a5c50, "unknown_1405a5c50", fn_1405a5c50 as *const ()), // sub_1405a5c50
    Replacement::draft(0x1405a5d20, "unknown_1405a5d20", fn_1405a5d20 as *const ()), // sub_1405a5d20
    Replacement::draft(0x1405a5d90, "unknown_1405a5d90", fn_1405a5d90 as *const ()), // sub_1405a5d90
    Replacement::draft(0x1405a5fc0, "unknown_1405a5fc0", tpl_3671::<0x14240aa00, 0x1405a62d0> as *const ()), // sub_1405a5fc0
    Replacement::draft(0x1405a61b0, "unknown_1405a61b0", fn_1405a61b0 as *const ()), // sub_1405a61b0
    Replacement::draft(0x1405a62d0, "unknown_1405a62d0", fn_1405a62d0 as *const ()), // sub_1405a62d0
    Replacement::draft(0x1405a63f0, "unknown_1405a63f0", fn_1405a63f0 as *const ()), // sub_1405a63f0
    Replacement::draft(0x1405a6520, "unknown_1405a6520", tpl_5649::<0x8, 0x19, 0x10, 0x140246bb0, 0x1401e66f0> as *const ()), // sub_1405a6520
    Replacement::draft(0x1405a6580, "unknown_1405a6580", tpl_5650::<0x1405a65b0, 0x8, 0x10> as *const ()), // sub_1405a6580
    Replacement::draft(0x1405a65b0, "unknown_1405a65b0", tpl_5651::<0x8, 0x10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x14, 0x142533200> as *const ()), // sub_1405a65b0
    Replacement::draft_throws(0x1405a6950, "unknown_1405a6950", fn_1405a6950 as *const (), Throws::Caught), // sub_1405a6950
    Replacement::draft_throws(0x1405a6bd0, "unknown_1405a6bd0", fn_1405a6bd0 as *const (), Throws::Caught), // sub_1405a6bd0
    Replacement::draft_throws(0x1405a6e60, "unknown_1405a6e60", fn_1405a6e60 as *const (), Throws::Caught), // sub_1405a6e60
    Replacement::draft_throws(0x1405a70d0, "bigfish_1405a70d0", tpl_1396::<0x143cb6dcc, 0x143cb6bd8, 0x1411e9f50, 0x1405a9070, 0x1405ab030, 0x1405a91f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431cd6d6, 0x142646960, 0x143cb6bf8, 0x143cb6c00, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::init::system>
    Replacement::draft(0x1405a7320, "bigfish_1405a7320", fn_1405a7320 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::process_queue::system>
    Replacement::draft_throws(0x1405a7530, "bigfish_1405a7530", tpl_5008::<0x143cb6b90, 0x143cb65d0, 0x1411e9f50, 0x1405ac730, 0x143cb6948, 0x1420af120, 0x143c8d110, 0x140215610, 0x1420a28e0, 0x1405ab2d0, 0x1405a9330, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431cd4f6, 0x142646960, 0x143cb65f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::finalize::save_case_handle::system>
    Replacement::draft_throws(0x1405a7840, "bigfish_1405a7840", tpl_4193::<0x143cb6bd0, 0x143cb6590, 0x1411e9f50, 0x1405a9440, 0x1405ab440, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431cc686, 0x142646960, 0x143cb65b0, 0x143cb65c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::finalize::item::system>
    Replacement::draft_throws(0x1405a7a90, "bigfish_1405a7a90", tpl_3465::<0x143cb6b88, 0x143cb6370, 0x1411e9f50, 0x1405a95f0, 0x1405ab590, 0x1405a97b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ccd36, 0x142646960, 0x143cb6390, 0x143cb63a0, 0x143cb63a2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::finalize::pin::system>
    Replacement::draft_throws(0x1405a7ce0, "bigfish_1405a7ce0", tpl_4143::<0x143cb64c8, 0x143cb6520, 0x1405a8910, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431cde46, 0x142646960, 0x143cb6540, 0x143cb6550, 0x143cb6554, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::finalize::thread::system>
    Replacement::draft_throws(0x1405a7f00, "bigfish_1405a7f00", tpl_1770::<0x143cb6d10, 0x143cb6a60, 0x1405a8b30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ce216, 0x142646960, 0x143cb6a80, 0x143cb6a90, 0x143cb6a94, 0x143cb6a96, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::spawner::finalize::sticker::system>
    Replacement::draft(0x1405a8120, "unknown_1405a8120", tpl_3077::<0x143cb5fe4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405a8120
    Replacement::draft(0x1405a8450, "unknown_1405a8450", tpl_1098::<0x143cb5350, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405a8450
    Replacement::draft(0x1405a8790, "unknown_1405a8790", tpl_4416::<0x1411e9f50, 0x143c9f26c, 0x140215610, 0x143c8d110, 0x143c8df88, 0x1420a28e0, 0x1405ab250, 0x1401de410> as *const ()), // sub_1405a8790
    Replacement::draft_throws(0x1405a8910, "unknown_1405a8910", tpl_1187::<0x1411e9f50, 0x1405ac0b0, 0x143cb6c40, 0x1420af120, 0x1405abfc0, 0x143cb64cc, 0x140215750, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x1405ab7f0, 0x1405a9ab0> as *const (), Throws::Discard), // sub_1405a8910
    Replacement::draft_throws(0x1405a8b30, "unknown_1405a8b30", tpl_5553::<0x1411e9f50, 0x1405abe80, 0x143cb68d4, 0x1420af120, 0x14036fad0, 0x140599530, 0x143c8d110, 0x140215610, 0x143c9f1c4, 0x1420a28e0, 0x1405ab9b0, 0x1405a9d00> as *const (), Throws::Discard), // sub_1405a8b30
    Replacement::draft(0x1405a8d10, "unknown_1405a8d10", fn_1405a8d10 as *const ()), // sub_1405a8d10
    Replacement::draft(0x1405a8eb0, "unknown_1405a8eb0", tpl_60530::<0x1426467e8, 0x143c8f7a0, 0x142489ee0, 0x143c9afe8, 0x1405a8ed9> as *const ()), // sub_1405a8eb0
    Replacement::draft_throws(0x1405a9070, "unknown_1405a9070", tpl_5657::<0x1405ac820, 0x143cb6b48, 0x1420af120, 0x143c9f26c, 0x140215610, 0x143c8df88, 0x143c8d110, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405a9070
    Replacement::draft_throws(0x1405a91f0, "unknown_1405a91f0", tpl_63486::<0x143c8d110, 0x143c9f26c, 0x143c8df88, 0x1405aa7f0, 0x141ae3870> as *const (), Throws::Discard), // sub_1405a91f0
    Replacement::draft(0x1405a9330, "unknown_1405a9330", tpl_60901::<0x143c8d110, 0x580e0, 0x143cb5940, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141ae23f0> as *const ()), // sub_1405a9330
    Replacement::draft_throws(0x1405a9440, "unknown_1405a9440", fn_1405a9440 as *const (), Throws::Discard), // sub_1405a9440
    Replacement::draft_throws(0x1405a95f0, "unknown_1405a95f0", fn_1405a95f0 as *const (), Throws::Discard), // sub_1405a95f0
    Replacement::draft(0x1405a97b0, "unknown_1405a97b0", fn_1405a97b0 as *const ()), // sub_1405a97b0
    Replacement::draft(0x1405a9890, "unknown_1405a9890", tpl_4372::<0x20, 0x143cb68d0, 0x8, 0x143c94b44, 0x143c8d110, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_1405a9890
    Replacement::draft(0x1405a9970, "unknown_1405a9970", fn_1405a9970 as *const ()), // sub_1405a9970
    Replacement::draft(0x1405a9ab0, "unknown_1405a9ab0", tpl_3708::<0x88, 0x10088, 0x1405a9b70, 0x1405aa420, 0x1405a9c10> as *const ()), // sub_1405a9ab0
    Replacement::draft(0x1405a9b70, "unknown_1405a9b70", tpl_3709::<0x143cb64cc, 0x20, 0x8, 0x143c8d08c, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_1405a9b70
    Replacement::draft(0x1405a9c10, "unknown_1405a9c10", tpl_3710::<0x18, 0x20, 0x8, 0x10, 0x141ae0980> as *const ()), // sub_1405a9c10
    Replacement::draft(0x1405a9d00, "unknown_1405a9d00", fn_1405a9d00 as *const ()), // sub_1405a9d00
    Replacement::draft(0x1405a9dc0, "unknown_1405a9dc0", tpl_5578::<0x20, 0x143c94b44, 0x8, 0x143cb54b8, 0x143c8d110, 0x580e0, 0x143c9f1c4, 0x142407c80, 0x580f0, 0x10, 0x18, 0x28> as *const ()), // sub_1405a9dc0
    Replacement::draft(0x1405a9ec0, "unknown_1405a9ec0", fn_1405a9ec0 as *const ()), // sub_1405a9ec0
    Replacement::draft(0x1405a9fe0, "unknown_1405a9fe0", fn_1405a9fe0 as *const ()), // sub_1405a9fe0
    Replacement::draft(0x1405aa8f0, "unknown_1405aa8f0", tpl_61197::<0x143c8d160, 0x142489ee0, 0x143c8f7a0, 0x143c8f6dc, 0x143c9afe8, 0x1426467e8, 0x1405aaa83> as *const ()), // sub_1405aa8f0
    Replacement::draft(0x1405aaa90, "unknown_1405aaa90", fn_1405aaa90 as *const ()), // sub_1405aaa90
    Replacement::draft(0x1405aab70, "unknown_1405aab70", fn_1405aab70 as *const ()), // sub_1405aab70
    Replacement::draft(0x1405aac60, "unknown_1405aac60", fn_1405aac60 as *const ()), // sub_1405aac60
    Replacement::draft(0x1405aadc0, "unknown_1405aadc0", fn_1405aadc0 as *const ()), // sub_1405aadc0
    Replacement::draft(0x1405aadf0, "unknown_1405aadf0", fn_1405aadf0 as *const ()), // sub_1405aadf0
    Replacement::draft(0x1405aae70, "unknown_1405aae70", fn_1405aae70 as *const ()), // sub_1405aae70
    Replacement::draft(0x1405aaed0, "unknown_1405aaed0", tpl_1212::<0x8, 0x1405ad570> as *const ()), // sub_1405aaed0
    Replacement::draft(0x1405aaf00, "unknown_1405aaf00", tpl_3163::<0x38, 0x8, 0x1405ad750> as *const ()), // sub_1405aaf00
    Replacement::draft(0x1405aaf40, "unknown_1405aaf40", tpl_1212::<0x8, 0x1405ada80> as *const ()), // sub_1405aaf40
    Replacement::draft(0x1405aaf70, "unknown_1405aaf70", fn_1405aaf70 as *const ()), // sub_1405aaf70
    Replacement::draft(0x1405aafc0, "unknown_1405aafc0", tpl_3163::<0x38, 0x8, 0x1405adfa0> as *const ()), // sub_1405aafc0
    Replacement::draft(0x1405ab000, "unknown_1405ab000", tpl_1212::<0x8, 0x1405ae2d0> as *const ()), // sub_1405ab000
    Replacement::draft_throws(0x1405ab030, "unknown_1405ab030", tpl_63520::<0x143c8d110, 0x143c8df88, 0x143c9f26c, 0x143cb6248, 0x142489ee0, 0x143c8d120, 0x141ae3870> as *const (), Throws::Discard), // sub_1405ab030
    Replacement::draft_throws(0x1405ab250, "unknown_1405ab250", fn_1405ab250 as *const (), Throws::Caught), // sub_1405ab250
    Replacement::draft(0x1405ab2d0, "unknown_1405ab2d0", tpl_60916::<0x143c8d110, 0x143cb5940, 0x142489ee0, 0x141ae23f0> as *const ()), // sub_1405ab2d0
    Replacement::draft(0x1405ab440, "unknown_1405ab440", fn_1405ab440 as *const ()), // sub_1405ab440
    Replacement::draft(0x1405ab590, "unknown_1405ab590", fn_1405ab590 as *const ()), // sub_1405ab590
    Replacement::draft(0x1405ab7f0, "unknown_1405ab7f0", tpl_63019::<0x1405a9b70, 0x1405aa420, 0x141ae0980> as *const ()), // sub_1405ab7f0
    Replacement::draft(0x1405ab9b0, "unknown_1405ab9b0", fn_1405ab9b0 as *const ()), // sub_1405ab9b0
    Replacement::draft(0x1405ac330, "unknown_1405ac330", fn_1405ac330 as *const ()), // sub_1405ac330
    Replacement::draft(0x1405ac4f0, "unknown_1405ac4f0", fn_1405ac4f0 as *const ()), // sub_1405ac4f0
    Replacement::draft(0x1405ac910, "unknown_1405ac910", fn_1405ac910 as *const ()), // sub_1405ac910
    Replacement::draft(0x1405acb40, "unknown_1405acb40", fn_1405acb40 as *const ()), // sub_1405acb40
    Replacement::draft(0x1405acbb0, "unknown_1405acbb0", fn_1405acbb0 as *const ()), // sub_1405acbb0
    Replacement::draft(0x1405ace20, "unknown_1405ace20", tpl_1094::<0x143c9afe8, 0x1405ae670, 0x14020e8f0> as *const ()), // sub_1405ace20
    Replacement::draft(0x1405ad020, "unknown_1405ad020", tpl_1094::<0x143cb5940, 0x1405ae7f0, 0x14020e8f0> as *const ()), // sub_1405ad020
    Replacement::draft(0x1405ad220, "unknown_1405ad220", tpl_3406::<0x143cb5358, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405ad220
    Replacement::draft(0x1405ad570, "unknown_1405ad570", tpl_1095::<0x143cb66b8, 0x14020e8f0> as *const ()), // sub_1405ad570
    Replacement::draft(0x1405ad750, "unknown_1405ad750", tpl_3077::<0x143cb69f4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405ad750
    Replacement::draft(0x1405ada80, "unknown_1405ada80", tpl_1095::<0x143cb5ed0, 0x14020e8f0> as *const ()), // sub_1405ada80
    Replacement::draft(0x1405adc60, "unknown_1405adc60", tpl_3080::<0x143cb5d00, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405adc60
    Replacement::draft(0x1405adfa0, "unknown_1405adfa0", tpl_3077::<0x143cb56c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405adfa0
    Replacement::draft(0x1405ae2d0, "unknown_1405ae2d0", tpl_1095::<0x143cb68c4, 0x14020e8f0> as *const ()), // sub_1405ae2d0
    Replacement::draft(0x1405ae4b0, "unknown_1405ae4b0", fn_1405ae4b0 as *const ()), // sub_1405ae4b0
    Replacement::draft(0x1405ae670, "unknown_1405ae670", fn_1405ae670 as *const ()), // sub_1405ae670
    Replacement::draft(0x1405ae7f0, "unknown_1405ae7f0", fn_1405ae7f0 as *const ()), // sub_1405ae7f0
    Replacement::draft(0x1405aeac0, "unknown_1405aeac0", tpl_64835::<0x8, 0x10, 0x14, 0x1403497a0, 0xc> as *const ()), // sub_1405aeac0
    Replacement::draft(0x1405aeb50, "unknown_1405aeb50", tpl_3160::<0x630, 0x1403db100> as *const ()), // sub_1405aeb50
    Replacement::draft(0x1405aeb80, "unknown_1405aeb80", tpl_5680::<0x7, 0x60, 0x48, 0x1401f2bc0, 0x14240ab10> as *const ()), // sub_1405aeb80
    Replacement::draft(0x1405aef30, "unknown_1405aef30", fn_1405aef30 as *const ()), // sub_1405aef30
    Replacement::draft(0x1405aefa0, "unknown_1405aefa0", fn_1405aefa0 as *const ()), // sub_1405aefa0
    Replacement::draft(0x1405af220, "unknown_1405af220", fn_1405af220 as *const ()), // sub_1405af220
    Replacement::draft(0x1405af360, "unknown_1405af360", fn_1405af360 as *const ()), // sub_1405af360
    Replacement::draft(0x1405af3a0, "unknown_1405af3a0", fn_1405af3a0 as *const ()), // sub_1405af3a0
    Replacement::draft_throws(0x1405af3f0, "unknown_1405af3f0", tpl_1480::<0x14276cb00, 0x1410d6ca0, 0x1405af690, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1405af3f0
    Replacement::draft_throws(0x1405af690, "bigfish_1405af690", tpl_3696::<0x143cb6eb8, 0x143cb6ec0, 0x1411e9f50, 0x1405af950, 0x1405afca0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431cf7b6, 0x142646960, 0x143cb6ee0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::case_board_hints::system>
    Replacement::draft(0x1405af8d0, "unknown_1405af8d0", fn_1405af8d0 as *const ()), // sub_1405af8d0
    Replacement::draft_throws(0x1405af950, "unknown_1405af950", fn_1405af950 as *const (), Throws::Discard), // sub_1405af950
    Replacement::draft(0x1405afbf0, "unknown_1405afbf0", fn_1405afbf0 as *const ()), // sub_1405afbf0
    Replacement::draft_throws(0x1405afca0, "unknown_1405afca0", fn_1405afca0 as *const (), Throws::Discard), // sub_1405afca0
    Replacement::draft(0x1405afd80, "unknown_1405afd80", fn_1405afd80 as *const ()), // sub_1405afd80
    Replacement::draft(0x1405afeb0, "unknown_1405afeb0", fn_1405afeb0 as *const ()), // sub_1405afeb0
    Replacement::draft(0x1405b0060, "unknown_1405b0060", tpl_3351::<0x30, 0x143c8f6d8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1405b00e2> as *const ()), // sub_1405b0060
    Replacement::draft(0x1405b00f0, "unknown_1405b00f0", fn_1405b00f0 as *const ()), // sub_1405b00f0
    Replacement::draft(0x1405b01b0, "unknown_1405b01b0", fn_1405b01b0 as *const ()), // sub_1405b01b0
    Replacement::draft(0x1405b0220, "unknown_1405b0220", fn_1405b0220 as *const ()), // sub_1405b0220
    Replacement::draft(0x1405b0390, "unknown_1405b0390", tpl_64850::<0x1426467e8, 0x1405b2500, 0x14036f9a0, 0x1405b0448> as *const ()), // sub_1405b0390
    Replacement::draft_throws(0x1405b05c0, "bigfish_1405b05c0", tpl_5694::<0x143cb6ef0, 0x143cb7028, 0x1411e9f50, 0x1405b1720, 0x1405b2980, 0x1405b1b00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d03f6, 0x142646960, 0x143cb7048, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::hand::system>
    Replacement::draft_throws(0x1405b0800, "bigfish_1405b0800", tpl_1480::<0x14276d470, 0x1410d6ca0, 0x1405b1270, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1405b0800
    Replacement::draft(0x1405b0b50, "bigfish_1405b0b50", fn_1405b0b50 as *const ()), // sub_1405b0b50
    Replacement::draft(0x1405b0c90, "bigfish_1405b0c90", fn_1405b0c90 as *const ()), // sub_1405b0c90
    Replacement::draft(0x1405b0da0, "bigfish_1405b0da0", fn_1405b0da0 as *const ()), // sub_1405b0da0
    Replacement::draft(0x1405b0e90, "bigfish_1405b0e90", tpl_1095::<0x143cb6030, 0x14020e8f0> as *const ()), // sub_1405b0e90
    Replacement::draft(0x1405b1070, "bigfish_1405b1070", tpl_1094::<0x143cb6fac, 0x1405b1580, 0x14020e8f0> as *const ()), // sub_1405b1070
    Replacement::draft_throws(0x1405b1270, "bigfish_1405b1270", tpl_1584::<0x143cb6ee8, 0x143cb7050, 0x1411e9f50, 0x1405b2c90, 0x1405b3300, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d0106, 0x142646960, 0x143cb7070, 0x143cb7078, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::hand::sync_ui::system>
    Replacement::draft(0x1405b14f0, "unknown_1405b14f0", fn_1405b14f0 as *const ()), // sub_1405b14f0
    Replacement::draft(0x1405b1580, "unknown_1405b1580", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431d1720, 0x1405b2ec0> as *const ()), // sub_1405b1580
    Replacement::draft_throws(0x1405b1720, "unknown_1405b1720", fn_1405b1720 as *const (), Throws::Discard), // sub_1405b1720
    Replacement::draft_throws(0x1405b1b00, "unknown_1405b1b00", fn_1405b1b00 as *const (), Throws::Discard), // sub_1405b1b00
    Replacement::draft(0x1405b1bc0, "unknown_1405b1bc0", fn_1405b1bc0 as *const ()), // sub_1405b1bc0
    Replacement::draft_throws(0x1405b1e10, "unknown_1405b1e10", fn_1405b1e10 as *const (), Throws::Discard), // sub_1405b1e10
    Replacement::draft(0x1405b1fc0, "content_1405b1fc0", tpl_1953::<0x1405b2200, 0x143084e28, 0x141834d30> as *const ()), // content::BoardHandComponent::content::BoardHandComponent::vfunc_0
    Replacement::draft(0x1405b2200, "unknown_1405b2200", fn_1405b2200 as *const ()), // sub_1405b2200
    Replacement::draft(0x1405b22e0, "unknown_1405b22e0", fn_1405b22e0 as *const ()), // sub_1405b22e0
    Replacement::draft(0x1405b2330, "unknown_1405b2330", fn_1405b2330 as *const ()), // sub_1405b2330
    Replacement::draft(0x1405b2810, "unknown_1405b2810", tpl_61019::<0x143c8d120, 0x142489ee0, 0x143c8f6d8, 0x143c9afe8, 0x1426467e8, 0x1405b2973> as *const ()), // sub_1405b2810
    Replacement::draft_throws(0x1405b2980, "unknown_1405b2980", fn_1405b2980 as *const (), Throws::Discard), // sub_1405b2980
    Replacement::draft_throws(0x1405b2c90, "unknown_1405b2c90", fn_1405b2c90 as *const (), Throws::Discard), // sub_1405b2c90
    Replacement::draft(0x1405b2ec0, "unknown_1405b2ec0", fn_1405b2ec0 as *const ()), // sub_1405b2ec0
    Replacement::draft(0x1405b3030, "unknown_1405b3030", fn_1405b3030 as *const ()), // sub_1405b3030
    Replacement::draft_throws(0x1405b3300, "unknown_1405b3300", tpl_5156::<0x143cb712c, 0x143c8d118, 0x143c8d0d4, 0x20, 0x143c8d110, 0x8, 0x143c8ce88, 0x143c900d8, 0x141ae67a0> as *const (), Throws::Discard), // sub_1405b3300
    Replacement::draft(0x1405b3500, "unknown_1405b3500", fn_1405b3500 as *const ()), // sub_1405b3500
    Replacement::draft(0x1405b38d0, "unknown_1405b38d0", fn_1405b38d0 as *const ()), // sub_1405b38d0
    Replacement::draft(0x1405b39f0, "unknown_1405b39f0", fn_1405b39f0 as *const ()), // sub_1405b39f0
    Replacement::draft(0x1405b3a30, "unknown_1405b3a30", fn_1405b3a30 as *const ()), // sub_1405b3a30
    Replacement::draft(0x1405b3b40, "unknown_1405b3b40", fn_1405b3b40 as *const ()), // sub_1405b3b40
    Replacement::draft(0x1405b3d80, "unknown_1405b3d80", fn_1405b3d80 as *const ()), // sub_1405b3d80
    Replacement::draft(0x1405b4140, "unknown_1405b4140", fn_1405b4140 as *const ()), // sub_1405b4140
    Replacement::draft(0x1405b43b0, "unknown_1405b43b0", fn_1405b43b0 as *const ()), // sub_1405b43b0
    Replacement::draft(0x1405b4600, "unknown_1405b4600", fn_1405b4600 as *const ()), // sub_1405b4600
    Replacement::draft(0x1405b48b0, "unknown_1405b48b0", fn_1405b48b0 as *const ()), // sub_1405b48b0
    Replacement::draft(0x1405b4a20, "unknown_1405b4a20", fn_1405b4a20 as *const ()), // sub_1405b4a20
    Replacement::draft(0x1405b4b00, "unknown_1405b4b00", fn_1405b4b00 as *const ()), // sub_1405b4b00
    Replacement::draft(0x1405b4d40, "unknown_1405b4d40", fn_1405b4d40 as *const ()), // sub_1405b4d40
    Replacement::draft(0x1405b4dd0, "unknown_1405b4dd0", fn_1405b4dd0 as *const ()), // sub_1405b4dd0
    Replacement::draft(0x1405b51c0, "content_1405b51c0", tpl_5719::<0x1431d3820, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::CaseBoardCabinetComponent::content::CaseBoardCabinetComponent::vfunc_18
    Replacement::draft(0x1405b5300, "unknown_1405b5300", tpl_3648::<0x143c9afe8, 0x142489ee0, 0x143cb5940, 0x143c8d160, 0x143c9b0d4> as *const ()), // sub_1405b5300
    Replacement::draft_throws(0x1405b5980, "unknown_1405b5980", fn_1405b5980 as *const (), Throws::Discard), // sub_1405b5980
    Replacement::draft_throws(0x1405b5c70, "bigfish_1405b5c70", tpl_1776::<0x143cb7b40, 0x143cb73b8, 0x1411e9f50, 0x1405b95d0, 0x1405bb120, 0x1405b9a00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d41e6, 0x142646960, 0x143cb73d8, 0x143cb73da, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::drawer::system>
    Replacement::draft_throws(0x1405b5eb0, "bigfish_1405b5eb0", tpl_3005::<0x143cb75ec, 0x143cb7ad8, 0x1411e9f50, 0x1405b9ed0, 0x1405bb420, 0x1405ba1d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d3b26, 0x142646960, 0x143cb7af8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::cabinet::system>
    Replacement::draft_throws(0x1405b60f0, "bigfish_1405b60f0", tpl_1480::<0x14276ddc0, 0x1410d6ca0, 0x1405b8340, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1405b60f0
    Replacement::draft_throws(0x1405b6390, "bigfish_1405b6390", tpl_1480::<0x14276ddc0, 0x1410d6ca0, 0x1405b8590, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1405b6390
    Replacement::draft(0x1405b6630, "bigfish_1405b6630", fn_1405b6630 as *const ()), // sub_1405b6630
    Replacement::draft(0x1405b66c0, "bigfish_1405b66c0", tpl_64186::<0x1426467e8, 0x1405ba770, 0x1405b6737> as *const ()), // sub_1405b66c0
    Replacement::draft(0x1405b6840, "bigfish_1405b6840", tpl_1094::<0x143cb760c, 0x1405b8aa0, 0x14020e8f0> as *const ()), // sub_1405b6840
    Replacement::draft(0x1405b6a40, "bigfish_1405b6a40", tpl_1436::<0x58, 0x60, 0x158, 0x143cb760c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405b6a40
    Replacement::draft(0x1405b6b20, "bigfish_1405b6b20", tpl_1094::<0x143cb7b38, 0x1405b8c20, 0x14020e8f0> as *const ()), // sub_1405b6b20
    Replacement::draft(0x1405b6d20, "bigfish_1405b6d20", fn_1405b6d20 as *const ()), // sub_1405b6d20
    Replacement::draft(0x1405b7220, "bigfish_1405b7220", tpl_1094::<0x143cb7a1c, 0x1405b9280, 0x14020e8f0> as *const ()), // sub_1405b7220
    Replacement::draft(0x1405b7420, "bigfish_1405b7420", tpl_1436::<0x58, 0x60, 0x158, 0x143cb7a1c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405b7420
    Replacement::draft(0x1405b7500, "bigfish_1405b7500", tpl_1094::<0x143cb73e0, 0x1405b9400, 0x14020e8f0> as *const ()), // sub_1405b7500
    Replacement::draft_throws(0x1405b7700, "bigfish_1405b7700", fn_1405b7700 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::drawer::stream_in::system>
    Replacement::draft_throws(0x1405b7a30, "bigfish_1405b7a30", tpl_1101::<0x143cb7268, 0x143cb7aa8, 0x1411e9f50, 0x1405be3f0, 0x143cb7830, 0x1420af120, 0x1420a28e0, 0x1405bcba0, 0x1405bae60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d5a76, 0x142646960, 0x143cb7ac8, 0x143cb7ad0, 0x143cb7ad4, 0x143cb7ad6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::drawer::stream_out::system>
    Replacement::draft_throws(0x1405b7d30, "bigfish_1405b7d30", tpl_5010::<0x143cb7a18, 0x143cb7700, 0x1411e9f50, 0x1405be2f0, 0x143cb7908, 0x1420af120, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x1405bcc80, 0x1405baf20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d5836, 0x142646960, 0x143cb7720, 0x143cb7728, 0x143cb772c, 0x143cb772e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::cabinet::stream_in::system>
    Replacement::draft_throws(0x1405b8060, "bigfish_1405b8060", tpl_1102::<0x143cb7360, 0x143cb7580, 0x1411e9f50, 0x1405be1e0, 0x143cb73dc, 0x1420af120, 0x1420a28e0, 0x1405bce00, 0x1405bb060, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d5626, 0x142646960, 0x143cb75a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::cabinet::stream_out::system>
    Replacement::draft_throws(0x1405b8340, "bigfish_1405b8340", tpl_4195::<0x143cb73a0, 0x143cb7330, 0x1411e9f50, 0x1405bb620, 0x1405bcee0, 0x1405bb9e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d5106, 0x142646960, 0x143cb7350, 0x143cb7358, 0x143cb735a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::drawer_overlay::syncUI>
    Replacement::draft_throws(0x1405b8590, "bigfish_1405b8590", tpl_5724::<0x143cb764c, 0x143cb7910, 0x1411e9f50, 0x1405bde90, 0x143cb77d0, 0x1420af120, 0x143c8ce6c, 0x140215610, 0x1420a28e0, 0x1405bd0f0, 0x1405bbd40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d4fa6, 0x142646960, 0x143cb7930, 0x143cb7934, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::caseboard::ui_events::system>
    Replacement::draft(0x1405b8910, "unknown_1405b8910", fn_1405b8910 as *const ()), // sub_1405b8910
    Replacement::draft(0x1405b8aa0, "unknown_1405b8aa0", fn_1405b8aa0 as *const ()), // sub_1405b8aa0
    Replacement::draft(0x1405b8c20, "unknown_1405b8c20", fn_1405b8c20 as *const ()), // sub_1405b8c20
    Replacement::draft(0x1405b8dc0, "unknown_1405b8dc0", fn_1405b8dc0 as *const ()), // sub_1405b8dc0
    Replacement::draft(0x1405b9280, "unknown_1405b9280", tpl_5728::<0x1401e6d50, 0x1405baaf0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1405ba640> as *const ()), // sub_1405b9280
    Replacement::draft(0x1405b9400, "unknown_1405b9400", fn_1405b9400 as *const ()), // sub_1405b9400
    Replacement::draft_throws(0x1405b95d0, "unknown_1405b95d0", fn_1405b95d0 as *const (), Throws::Discard), // sub_1405b95d0
    Replacement::draft_throws(0x1405b9a00, "unknown_1405b9a00", fn_1405b9a00 as *const (), Throws::Discard), // sub_1405b9a00
    Replacement::draft(0x1405b9ad0, "unknown_1405b9ad0", fn_1405b9ad0 as *const ()), // sub_1405b9ad0
    Replacement::draft_throws(0x1405b9d10, "unknown_1405b9d10", fn_1405b9d10 as *const (), Throws::Discard), // sub_1405b9d10
    Replacement::draft_throws(0x1405b9ed0, "unknown_1405b9ed0", fn_1405b9ed0 as *const (), Throws::Discard), // sub_1405b9ed0
    Replacement::draft_throws(0x1405ba1d0, "unknown_1405ba1d0", fn_1405ba1d0 as *const (), Throws::Discard), // sub_1405ba1d0
    Replacement::draft(0x1405ba2d0, "unknown_1405ba2d0", fn_1405ba2d0 as *const ()), // sub_1405ba2d0
    Replacement::draft_throws(0x1405ba460, "unknown_1405ba460", fn_1405ba460 as *const (), Throws::Discard), // sub_1405ba460
    Replacement::draft(0x1405ba5d0, "unknown_1405ba5d0", fn_1405ba5d0 as *const ()), // sub_1405ba5d0
    Replacement::draft(0x1405ba640, "unknown_1405ba640", fn_1405ba640 as *const ()), // sub_1405ba640
    Replacement::draft(0x1405ba720, "content_1405ba720", tpl_5739::<0x140217f00, 0x143084e28, 0x141834d30> as *const ()), // content::CaseBoardCabinetDrawerComponent::content::CaseBoardCabinetDrawerComponent::vfunc_0
    Replacement::draft(0x1405ba770, "unknown_1405ba770", tpl_1289::<0x143cb7b38, 0x18058, 0x1806c, 0x142489ee0, 0xc03, 0x143cb760c, 0x8> as *const ()), // sub_1405ba770
    Replacement::draft(0x1405ba8f0, "unknown_1405ba8f0", tpl_3798::<0x58, 0x14240ab10> as *const ()), // sub_1405ba8f0
    Replacement::draft(0x1405ba930, "unknown_1405ba930", tpl_1124::<0x28> as *const ()), // sub_1405ba930
    Replacement::draft(0x1405ba960, "unknown_1405ba960", fn_1405ba960 as *const ()), // sub_1405ba960
    Replacement::draft(0x1405baaf0, "unknown_1405baaf0", tpl_3156::<0xf, 0xc0, 0x6c, 0x14240ab10, 0x24> as *const ()), // sub_1405baaf0
    Replacement::draft(0x1405bab40, "unknown_1405bab40", fn_1405bab40 as *const ()), // sub_1405bab40
    Replacement::draft(0x1405bac20, "unknown_1405bac20", tpl_4372::<0x20, 0x143c94b44, 0x8, 0x143c94b30, 0x143c8d08c, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_1405bac20
    Replacement::draft(0x1405bad00, "unknown_1405bad00", fn_1405bad00 as *const ()), // sub_1405bad00
    Replacement::draft(0x1405bae60, "unknown_1405bae60", tpl_5743::<0x88, 0x10088, 0x142407c80, 0x1405bc150, 0x10, 0x1405b7420> as *const ()), // sub_1405bae60
    Replacement::draft(0x1405baf20, "unknown_1405baf20", tpl_4168::<0x143c8d08c, 0x142407c80, 0x143cb7b38, 0x142489ee0, 0x141af0fa0> as *const ()), // sub_1405baf20
    Replacement::draft(0x1405bb060, "unknown_1405bb060", tpl_5743::<0x88, 0x10088, 0x142407c80, 0x1405bc050, 0x10, 0x1405b6a40> as *const ()), // sub_1405bb060
    Replacement::draft_throws(0x1405bb120, "unknown_1405bb120", fn_1405bb120 as *const (), Throws::Discard), // sub_1405bb120
    Replacement::draft_throws(0x1405bb420, "unknown_1405bb420", fn_1405bb420 as *const (), Throws::Discard), // sub_1405bb420
    Replacement::draft_throws(0x1405bb620, "unknown_1405bb620", fn_1405bb620 as *const (), Throws::Discard), // sub_1405bb620
    Replacement::draft_throws(0x1405bb9e0, "unknown_1405bb9e0", tpl_3628::<0x88, 0x10088, 0x1405bbaf0, 0x143cb760c, 0x180ac, 0x18098, 0x142489ee0, 0xc03, 0x40, 0x1405bbc10> as *const (), Throws::Discard), // sub_1405bb9e0
    Replacement::draft(0x1405bbaf0, "unknown_1405bbaf0", tpl_5745::<0x143c9b0d0, 0x143cb75e8, 0x8, 0x580e0, 0x20, 0x143cb7ba8, 0x143c904a0, 0x143cb745c, 0x143c8d08c, 0x143c8d110, 0x143c8ce88, 0x143c8ceec> as *const ()), // sub_1405bbaf0
    Replacement::draft_throws(0x1405bbc10, "unknown_1405bbc10", fn_1405bbc10 as *const (), Throws::Discard), // sub_1405bbc10
    Replacement::draft(0x1405bbd40, "unknown_1405bbd40", fn_1405bbd40 as *const ()), // sub_1405bbd40
    Replacement::draft(0x1405bc7a0, "unknown_1405bc7a0", fn_1405bc7a0 as *const ()), // sub_1405bc7a0
    Replacement::draft(0x1405bc8e0, "unknown_1405bc8e0", fn_1405bc8e0 as *const ()), // sub_1405bc8e0
    Replacement::draft(0x1405bcba0, "unknown_1405bcba0", tpl_5111::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405b7420> as *const ()), // sub_1405bcba0
    Replacement::draft(0x1405bcc80, "unknown_1405bcc80", tpl_4169::<0x143c8d08c, 0x142407c80, 0x143cb7b38, 0x142489ee0, 0x141af0fa0> as *const ()), // sub_1405bcc80
    Replacement::draft(0x1405bce00, "unknown_1405bce00", tpl_5111::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405b6a40> as *const ()), // sub_1405bce00
    Replacement::draft_throws(0x1405bcee0, "unknown_1405bcee0", fn_1405bcee0 as *const (), Throws::Discard), // sub_1405bcee0
    Replacement::draft(0x1405bd0f0, "unknown_1405bd0f0", tpl_60916::<0x143c8ce6c, 0x143cb760c, 0x142489ee0, 0x141aee840> as *const ()), // sub_1405bd0f0
    Replacement::draft(0x1405bd6f0, "unknown_1405bd6f0", fn_1405bd6f0 as *const ()), // sub_1405bd6f0
    Replacement::draft(0x1405bd930, "unknown_1405bd930", fn_1405bd930 as *const ()), // sub_1405bd930
    Replacement::draft(0x1405bdba0, "unknown_1405bdba0", fn_1405bdba0 as *const ()), // sub_1405bdba0
    Replacement::draft(0x1405be500, "unknown_1405be500", fn_1405be500 as *const ()), // sub_1405be500
    Replacement::draft(0x1405beb70, "unknown_1405beb70", fn_1405beb70 as *const ()), // sub_1405beb70
    Replacement::draft(0x1405bef40, "unknown_1405bef40", fn_1405bef40 as *const ()), // sub_1405bef40
    Replacement::draft(0x1405bf290, "unknown_1405bf290", fn_1405bf290 as *const ()), // sub_1405bf290
    Replacement::draft(0x1405bf350, "unknown_1405bf350", fn_1405bf350 as *const ()), // sub_1405bf350
    Replacement::draft(0x1405bf5c0, "unknown_1405bf5c0", fn_1405bf5c0 as *const ()), // sub_1405bf5c0
    Replacement::draft(0x1405bf680, "content_1405bf680", tpl_1892::<0x1431d6de8, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::CameraPanningVolumeComponent::content::CameraPanningVolumeComponent::vfunc_18
    Replacement::draft(0x1405bf780, "unknown_1405bf780", fn_1405bf780 as *const ()), // sub_1405bf780
    Replacement::draft(0x1405bf8d0, "unknown_1405bf8d0", tpl_64920::<0x1401e7c10, 0x143c92bcc, 0x142489ee0, 0x143c9d618> as *const ()), // sub_1405bf8d0
    Replacement::draft(0x1405bfa40, "unknown_1405bfa40", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143c9d618, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1405bfa40
    Replacement::draft(0x1405bfb30, "unknown_1405bfb30", tpl_61520::<0x8, 0x1401e7c10, 0x18, 0x28, 0x30, 0x10, 0x48, 0x20, 0x10048, 0x143c8d160, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1405bfb30
    Replacement::draft(0x1405bfc20, "unknown_1405bfc20", tpl_64921::<0x10, 0x18, 0x143c9d618, 0x1806c, 0x18058, 0x142489ee0, 0x18068, 0x143cb80c8, 0x18050, 0x20, 0x38> as *const ()), // sub_1405bfc20
    Replacement::draft(0x1405bfd00, "unknown_1405bfd00", tpl_1171::<0x58, 0x60, 0x143cb80c8, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405bfd00
    Replacement::draft(0x1405bfde0, "unknown_1405bfde0", fn_1405bfde0 as *const ()), // sub_1405bfde0
    Replacement::draft_throws(0x1405bfe90, "bigfish_1405bfe90", tpl_3367::<0x143cb7e18, 0x143cb7de8, 0x1405c0ee0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d79a6, 0x142646960, 0x143cb7e08, 0x143cb7e10, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::camerapanning::initCameraPanning>
    Replacement::draft_throws(0x1405c00a0, "bigfish_1405c00a0", tpl_1534::<0x143cb7fd4, 0x143cb7c70, 0x1411e9f50, 0x1405c1d40, 0x1405c2920, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d7c56, 0x142646960, 0x143cb7c90, 0x143cb7ca0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::camerapanning::checkActiveCameraPanning>
    Replacement::draft_throws(0x1405c02f0, "bigfish_1405c02f0", tpl_1185::<0x143cb7cf0, 0x143cb8030, 0x1411e9f50, 0x1405c1f30, 0x1405c2a50, 0x1405c21c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d8036, 0x142646960, 0x143cb8050, 0x143cb8058, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::camerapanning::updateCameraPanning>
    Replacement::draft(0x1405c0540, "unknown_1405c0540", tpl_1094::<0x143c9d618, 0x1405c17b0, 0x14020e8f0> as *const ()), // sub_1405c0540
    Replacement::draft(0x1405c0740, "unknown_1405c0740", tpl_1095::<0x143c9dc68, 0x14020e8f0> as *const ()), // sub_1405c0740
    Replacement::draft(0x1405c0920, "unknown_1405c0920", tpl_1095::<0x143cb80c8, 0x14020e8f0> as *const ()), // sub_1405c0920
    Replacement::draft(0x1405c0b00, "unknown_1405c0b00", tpl_1094::<0x143cb7e20, 0x1405c1930, 0x14020e8f0> as *const ()), // sub_1405c0b00
    Replacement::draft(0x1405c0d00, "unknown_1405c0d00", tpl_1095::<0x143cb7ddc, 0x14020e8f0> as *const ()), // sub_1405c0d00
    Replacement::draft_throws(0x1405c0ee0, "unknown_1405c0ee0", tpl_1187::<0x1411e9f50, 0x1405c3990, 0x143cb7f38, 0x1420af120, 0x1405c38e0, 0x143cb80b0, 0x140215750, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x1405c2770, 0x1405c1b40> as *const (), Throws::Discard), // sub_1405c0ee0
    Replacement::draft(0x1405c1100, "unknown_1405c1100", tpl_1962::<0x8, 0x1405c0b00, 0x1405c0d00> as *const ()), // sub_1405c1100
    Replacement::draft(0x1405c1160, "unknown_1405c1160", tpl_60530::<0x1426467e8, 0x143cb7e20, 0x142489ee0, 0x143c8d120, 0x1405c1189> as *const ()), // sub_1405c1160
    Replacement::draft(0x1405c1320, "unknown_1405c1320", fn_1405c1320 as *const ()), // sub_1405c1320
    Replacement::draft(0x1405c17b0, "unknown_1405c17b0", fn_1405c17b0 as *const ()), // sub_1405c17b0
    Replacement::draft(0x1405c1930, "unknown_1405c1930", fn_1405c1930 as *const ()), // sub_1405c1930
    Replacement::draft(0x1405c1b40, "unknown_1405c1b40", fn_1405c1b40 as *const ()), // sub_1405c1b40
    Replacement::draft(0x1405c1ca0, "unknown_1405c1ca0", tpl_3709::<0x143cb80b0, 0x20, 0x8, 0x143c8d08c, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_1405c1ca0
    Replacement::draft_throws(0x1405c1d40, "unknown_1405c1d40", fn_1405c1d40 as *const (), Throws::Discard), // sub_1405c1d40
    Replacement::draft_throws(0x1405c1f30, "unknown_1405c1f30", fn_1405c1f30 as *const (), Throws::Discard), // sub_1405c1f30
    Replacement::draft_throws(0x1405c21c0, "unknown_1405c21c0", fn_1405c21c0 as *const (), Throws::Discard), // sub_1405c21c0
    Replacement::draft(0x1405c2290, "unknown_1405c2290", tpl_5574::<0x143cb8084, 0x143c94b44, 0x143c8d0d4, 0x143c9f0b8, 0x143c8d904, 0x143c8d910, 0x143c8ceec, 0x142407c80> as *const ()), // sub_1405c2290
    Replacement::draft_throws(0x1405c23d0, "unknown_1405c23d0", fn_1405c23d0 as *const (), Throws::Discard), // sub_1405c23d0
    Replacement::draft(0x1405c2520, "content_1405c2520", fn_1405c2520 as *const ()), // content::CameraPanningVolumeComponent::content::CameraPanningVolumeComponent::vfunc_0
    Replacement::draft(0x1405c2570, "unknown_1405c2570", fn_1405c2570 as *const ()), // sub_1405c2570
    Replacement::draft(0x1405c2700, "unknown_1405c2700", fn_1405c2700 as *const ()), // sub_1405c2700
    Replacement::draft(0x1405c2740, "unknown_1405c2740", tpl_1124::<0x78> as *const ()), // sub_1405c2740
    Replacement::draft(0x1405c2770, "unknown_1405c2770", fn_1405c2770 as *const ()), // sub_1405c2770
    Replacement::draft(0x1405c2920, "unknown_1405c2920", fn_1405c2920 as *const ()), // sub_1405c2920
    Replacement::draft_throws(0x1405c2a50, "unknown_1405c2a50", fn_1405c2a50 as *const (), Throws::Discard), // sub_1405c2a50
    Replacement::draft_throws(0x1405c2c80, "unknown_1405c2c80", tpl_1845::<0x143cb7f34, 0x143cb8028, 0x143cb8090, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d618, 0x143c9dc68, 0x143cb80a0, 0x143cb80a8, 0x141834a0c> as *const (), Throws::Discard), // sub_1405c2c80
    Replacement::draft_throws(0x1405c2de0, "unknown_1405c2de0", tpl_1845::<0x143cb8020, 0x143cb8064, 0x143cb7f40, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c9dc68, 0x143cb7f50, 0x143cb7f58, 0x141834a0c> as *const (), Throws::Discard), // sub_1405c2de0
    Replacement::draft(0x1405c3de0, "unknown_1405c3de0", fn_1405c3de0 as *const ()), // sub_1405c3de0
    Replacement::draft(0x1405c3f70, "content_1405c3f70", fn_1405c3f70 as *const ()), // sub_1405c3f70
    Replacement::draft(0x1405c3f80, "content_1405c3f80", fn_1405c3f80 as *const ()), // sub_1405c3f80
    Replacement::draft(0x1405c4160, "content_1405c4160", fn_1405c4160 as *const ()), // content::PlayerCameraComponent::content::PlayerCameraComponent::vfunc_0
    Replacement::draft(0x1405c41a0, "unknown_1405c41a0", fn_1405c41a0 as *const ()), // sub_1405c41a0
    Replacement::draft(0x1405c4300, "unknown_1405c4300", fn_1405c4300 as *const ()), // sub_1405c4300
    Replacement::draft(0x1405c43d0, "unknown_1405c43d0", fn_1405c43d0 as *const ()), // sub_1405c43d0
    Replacement::draft(0x1405c4dc0, "unknown_1405c4dc0", fn_1405c4dc0 as *const ()), // sub_1405c4dc0
    Replacement::draft(0x1405c4e60, "unknown_1405c4e60", tpl_4842::<0x14397da68, 0x143c8d08c, 0x8, 0x580e0, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143c8d120, 0x180a8, 0x18090, 0x142489ee0, 0x14040f5f0, 0x1426467e8, 0x1405c4f45> as *const ()), // sub_1405c4e60
    Replacement::draft(0x1405c4f60, "unknown_1405c4f60", fn_1405c4f60 as *const ()), // sub_1405c4f60
    Replacement::draft(0x1405c50b0, "unknown_1405c50b0", fn_1405c50b0 as *const ()), // sub_1405c50b0
    Replacement::draft(0x1405c5130, "unknown_1405c5130", fn_1405c5130 as *const ()), // sub_1405c5130
    Replacement::draft(0x1405c52a0, "unknown_1405c52a0", fn_1405c52a0 as *const ()), // sub_1405c52a0
    Replacement::draft(0x1405c5330, "unknown_1405c5330", tpl_63182::<0x143cb82c0, 0x1401e7c10, 0x143c8d120, 0x142489ee0, 0x143c8efb0> as *const ()), // sub_1405c5330
    Replacement::draft_throws(0x1405c5520, "bigfish_1405c5520", fn_1405c5520 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::applyTimelineCameras>
    Replacement::draft_throws(0x1405c5970, "bigfish_1405c5970", tpl_1437::<0x143cb8b4c, 0x143cb8718, 0x1411e9f50, 0x1405ceb80, 0x143cb8464, 0x1420af120, 0x1420a28e0, 0x1405cb010, 0x1405c8d30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dfa96, 0x142646960, 0x143cb8738, 0x143cb8740, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::updateAlignToPlayer>
    Replacement::draft_throws(0x1405c5c50, "bigfish_1405c5c50", tpl_5724::<0x143cb8a20, 0x143cb8670, 0x1411e9f50, 0x1405cea90, 0x143cb8588, 0x1420af120, 0x143c8cef8, 0x140215610, 0x1420a28e0, 0x1405cb220, 0x1405c8e50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dfc86, 0x142646960, 0x143cb8690, 0x143cb8694, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::initCameraMixer>
    Replacement::draft(0x1405c5f70, "bigfish_1405c5f70", tpl_5783::<0x143cb8748, 0x143cb8108, 0x1411e9f50, 0x1405ccbd0, 0x143c8cefc, 0x1401fc320, 0x143c8cef8, 0x140215610, 0x1420a28e0, 0x1405cb3a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d8cf6, 0x142646960, 0x143cb8128, 0x143cb8130, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::cameraMixerRemoved>
    Replacement::draft_throws(0x1405c6260, "bigfish_1405c6260", tpl_3005::<0x143cb8c04, 0x143cb8b78, 0x1411e9f50, 0x1405c8f90, 0x1405cb450, 0x1405c9120, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d8dc6, 0x142646960, 0x143cb8b98, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::initialUpdate>
    Replacement::draft_throws(0x1405c64a0, "bigfish_1405c64a0", tpl_1698::<0x143cb8c54, 0x143cb8ba0, 0x1411e9f50, 0x1405c9400, 0x1405cb660, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431d9aa6, 0x142646960, 0x143cb8bc0, 0x143cb8bc2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::finishUpdate>
    Replacement::draft_throws(0x1405c66e0, "bigfish_1405c66e0", tpl_1437::<0x143cb8b00, 0x143cb8758, 0x1411e9f50, 0x1405cda50, 0x143cb8c08, 0x1420af120, 0x1420a28e0, 0x1405cba90, 0x1405c9690, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dbe76, 0x142646960, 0x143cb8778, 0x143cb8780, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::updateAimAnimations>
    Replacement::draft_throws(0x1405c69c0, "bigfish_1405c69c0", tpl_3064::<0x143cb885c, 0x143cb87c8, 0x1411e9f50, 0x1405c97b0, 0x1405cbca0, 0x1405c9960, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dc066, 0x142646960, 0x143cb87e8, 0x143cb87f0, 0x143cb87f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::preTimelineInterpolate>
    Replacement::draft_throws(0x1405c6c10, "bigfish_1405c6c10", tpl_3084::<0x143cb8b48, 0x143cb8138, 0x1411e9f50, 0x1405cd800, 0x143cb8bc4, 0x1420af120, 0x1405cd750, 0x143cb8458, 0x140215750, 0x1420a28e0, 0x1405cbf10, 0x1405c9ba0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dc3d6, 0x142646960, 0x143cb8158, 0x143cb8160, 0x143cb8164, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::postTimelineInterpolate>
    Replacement::draft_throws(0x1405c6f70, "bigfish_1405c6f70", tpl_4195::<0x143cb8460, 0x143cb8428, 0x1411e9f50, 0x1405c8f90, 0x1405cc180, 0x1405c9d60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dc676, 0x142646960, 0x143cb8448, 0x143cb8450, 0x143cb8452, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::loadingInitialUpdate>
    Replacement::draft_throws(0x1405c71c0, "bigfish_1405c71c0", tpl_1584::<0x143cb8424, 0x143cb8590, 0x1411e9f50, 0x1405c9400, 0x1405cc390, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431dd356, 0x142646960, 0x143cb85b0, 0x143cb85b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::cameramixer::loadingFinishUpdate>
    Replacement::draft(0x1405c7410, "unknown_1405c7410", fn_1405c7410 as *const ()), // sub_1405c7410
    Replacement::draft(0x1405c74a0, "unknown_1405c74a0", tpl_1094::<0x143cb8630, 0x1405c8740, 0x14020e8f0> as *const ()), // sub_1405c74a0
    Replacement::draft(0x1405c76a0, "unknown_1405c76a0", tpl_1095::<0x143cb8c58, 0x14020e8f0> as *const ()), // sub_1405c76a0
    Replacement::draft(0x1405c7880, "unknown_1405c7880", tpl_1094::<0x143c8cefc, 0x1405c88c0, 0x14020e8f0> as *const ()), // sub_1405c7880
    Replacement::draft(0x1405c7a80, "unknown_1405c7a80", tpl_4390::<0x143cb87f8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405c7a80
    Replacement::draft(0x1405c7db0, "unknown_1405c7db0", fn_1405c7db0 as *const ()), // sub_1405c7db0
    Replacement::draft(0x1405c8120, "unknown_1405c8120", tpl_1094::<0x143cb8d20, 0x1405c8a40, 0x14020e8f0> as *const ()), // sub_1405c8120
    Replacement::draft(0x1405c8350, "unknown_1405c8350", fn_1405c8350 as *const ()), // sub_1405c8350
    Replacement::draft(0x1405c8530, "unknown_1405c8530", fn_1405c8530 as *const ()), // sub_1405c8530
    Replacement::draft(0x1405c8740, "unknown_1405c8740", tpl_5788::<0x1401e6d50, 0x1404937d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1405c9fd0> as *const ()), // sub_1405c8740
    Replacement::draft(0x1405c88c0, "unknown_1405c88c0", fn_1405c88c0 as *const ()), // sub_1405c88c0
    Replacement::draft(0x1405c8a40, "unknown_1405c8a40", fn_1405c8a40 as *const ()), // sub_1405c8a40
    Replacement::draft(0x1405c8c30, "unknown_1405c8c30", tpl_63093::<0x143cb82c8, 0x142489ee0, 0x141afcce0> as *const ()), // sub_1405c8c30
    Replacement::draft(0x1405c8d30, "unknown_1405c8d30", tpl_62730::<0x88, 0x10088, 0x1405cac20, 0x141afd3e0> as *const ()), // sub_1405c8d30
    Replacement::draft(0x1405c8e50, "unknown_1405c8e50", tpl_4168::<0x143c8cef8, 0x142407c80, 0x143c8cefc, 0x142489ee0, 0x141b025b0> as *const ()), // sub_1405c8e50
    Replacement::draft_throws(0x1405c8f90, "unknown_1405c8f90", fn_1405c8f90 as *const (), Throws::Discard), // sub_1405c8f90
    Replacement::draft_throws(0x1405c9120, "unknown_1405c9120", tpl_5792::<0x88, 0x10088, 0x1405c91e0, 0x1405ca850, 0x1405c92c0> as *const (), Throws::Discard), // sub_1405c9120
    Replacement::draft(0x1405c91e0, "unknown_1405c91e0", fn_1405c91e0 as *const ()), // sub_1405c91e0
    Replacement::draft_throws(0x1405c92c0, "unknown_1405c92c0", fn_1405c92c0 as *const (), Throws::Discard), // sub_1405c92c0
    Replacement::draft_throws(0x1405c9400, "unknown_1405c9400", fn_1405c9400 as *const (), Throws::Discard), // sub_1405c9400
    Replacement::draft(0x1405c9690, "unknown_1405c9690", tpl_62730::<0x88, 0x10088, 0x1405ca2c0, 0x141afdc70> as *const ()), // sub_1405c9690
    Replacement::draft_throws(0x1405c97b0, "unknown_1405c97b0", fn_1405c97b0 as *const (), Throws::Discard), // sub_1405c97b0
    Replacement::draft(0x1405c9960, "unknown_1405c9960", fn_1405c9960 as *const ()), // sub_1405c9960
    Replacement::draft(0x1405c9aa0, "unknown_1405c9aa0", fn_1405c9aa0 as *const ()), // sub_1405c9aa0
    Replacement::draft_throws(0x1405c9ba0, "unknown_1405c9ba0", tpl_1470::<0x88, 0x10088, 0x143cb8458, 0x20, 0x8, 0x1405ca040, 0x1405c9c80> as *const (), Throws::Discard), // sub_1405c9ba0
    Replacement::draft_throws(0x1405c9c80, "unknown_1405c9c80", tpl_1471::<0x18, 0x20, 0x30, 0x10, 0x28, 0x8, 0x141afd580> as *const (), Throws::Discard), // sub_1405c9c80
    Replacement::draft_throws(0x1405c9d60, "unknown_1405c9d60", tpl_5792::<0x88, 0x10088, 0x1405c91e0, 0x1405ca850, 0x1405c9e20> as *const (), Throws::Discard), // sub_1405c9d60
    Replacement::draft_throws(0x1405c9e20, "unknown_1405c9e20", fn_1405c9e20 as *const (), Throws::Discard), // sub_1405c9e20
    Replacement::draft(0x1405c9f70, "unknown_1405c9f70", fn_1405c9f70 as *const ()), // sub_1405c9f70
    Replacement::draft(0x1405c9fd0, "unknown_1405c9fd0", fn_1405c9fd0 as *const ()), // sub_1405c9fd0
    Replacement::draft_throws(0x1405ca3f0, "unknown_1405ca3f0", tpl_1330::<0x8, 0x1405cdb60, 0x143cb82d0, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ca3f0
    Replacement::draft_throws(0x1405ca4d0, "unknown_1405ca4d0", tpl_1330::<0x8, 0x1405cdc70, 0x143cb8744, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ca4d0
    Replacement::draft_throws(0x1405ca5b0, "unknown_1405ca5b0", tpl_1330::<0x8, 0x1405cdfe0, 0x143cb874c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ca5b0
    Replacement::draft_throws(0x1405ca690, "unknown_1405ca690", tpl_1330::<0x8, 0x1405ce0d0, 0x143cb866c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ca690
    Replacement::draft_throws(0x1405ca770, "unknown_1405ca770", tpl_1330::<0x8, 0x1405ce390, 0x143cb8898, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ca770
    Replacement::draft_throws(0x1405ca9b0, "unknown_1405ca9b0", tpl_1330::<0x8, 0x14041d980, 0x143c9d194, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ca9b0
    Replacement::draft_throws(0x1405caa90, "unknown_1405caa90", tpl_1330::<0x8, 0x1405ce5d0, 0x143cb861c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405caa90
    Replacement::draft_throws(0x1405cab70, "unknown_1405cab70", tpl_1331::<0x8, 0x1405ce8b0, 0x143cb8558, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_1405cab70
    Replacement::draft(0x1405cc830, "unknown_1405cc830", fn_1405cc830 as *const ()), // sub_1405cc830
    Replacement::draft(0x1405ccbd0, "unknown_1405ccbd0", tpl_3188::<0x1405ce9f0, 0x1431d8a08, 0x1405cd120, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1405ccbd0
    Replacement::draft(0x1405ccfb0, "unknown_1405ccfb0", fn_1405ccfb0 as *const ()), // sub_1405ccfb0
    Replacement::draft(0x1405cd120, "unknown_1405cd120", fn_1405cd120 as *const ()), // sub_1405cd120
    Replacement::draft(0x1405cf8e0, "unknown_1405cf8e0", fn_1405cf8e0 as *const ()), // sub_1405cf8e0
    Replacement::draft_throws(0x1405cf9d0, "bigfish_1405cf9d0", fn_1405cf9d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::camera_attach::updateEvents>
    Replacement::draft_throws(0x1405cfd10, "bigfish_1405cfd10", tpl_5818::<0x143cb8e30, 0x143cb8d80, 0x1411e9f50, 0x1405d27a0, 0x143cb8fcc, 0x1420af120, 0x143c8d910, 0x140215610, 0x143c8d904, 0x1420a28e0, 0x1405d1f10, 0x1405d1600, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e3526, 0x142646960, 0x143cb8da0, 0x143cb8da4, 0x143cb8da6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::camera_attach::updateOverride>
    Replacement::draft_throws(0x1405d0060, "bigfish_1405d0060", tpl_4955::<0x143cb8ec0, 0x143cb8f68, 0x1411e9f50, 0x1405d2690, 0x143cb8e34, 0x1420af120, 0x1420a28e0, 0x1405d2190, 0x1405d1760, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e3346, 0x142646960, 0x143cb8f88, 0x143cb8f8c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::camera_attach::updateOutput>
    Replacement::draft(0x1405d0340, "unknown_1405d0340", tpl_1094::<0x143cb8ec8, 0x1405d1140, 0x14020e8f0> as *const ()), // sub_1405d0340
    Replacement::draft(0x1405d0540, "unknown_1405d0540", tpl_1094::<0x143cb8eb4, 0x1405d12f0, 0x14020e8f0> as *const ()), // sub_1405d0540
    Replacement::draft(0x1405d0740, "unknown_1405d0740", tpl_5819::<0x143cb8fc8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405d0740
    Replacement::draft(0x1405d0a90, "unknown_1405d0a90", tpl_1098::<0x143cb8420, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405d0a90
    Replacement::draft(0x1405d0dd0, "unknown_1405d0dd0", tpl_1098::<0x143c98afc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405d0dd0
    Replacement::draft(0x1405d1140, "unknown_1405d1140", fn_1405d1140 as *const ()), // sub_1405d1140
    Replacement::draft(0x1405d12f0, "unknown_1405d12f0", fn_1405d12f0 as *const ()), // sub_1405d12f0
    Replacement::draft(0x1405d14a0, "unknown_1405d14a0", tpl_62821::<0x143c8d904, 0x143c8d910, 0x1405d1b30, 0x141b05e40> as *const ()), // sub_1405d14a0
    Replacement::draft(0x1405d1600, "unknown_1405d1600", tpl_62821::<0x143c8d904, 0x143c8d910, 0x1405d1a00, 0x141b05c80> as *const ()), // sub_1405d1600
    Replacement::draft(0x1405d1760, "unknown_1405d1760", tpl_62730::<0x88, 0x10088, 0x1405d18d0, 0x141b05bc0> as *const ()), // sub_1405d1760
    Replacement::draft(0x1405d1880, "content_1405d1880", tpl_1967::<0x14028d970, 0x143084e28, 0x141834d30> as *const ()), // content::CameraAttachComponent::content::CameraAttachComponent::vfunc_0
    Replacement::draft(0x1405d1c60, "unknown_1405d1c60", tpl_1955::<0x40, 0x1401f2bc0> as *const ()), // sub_1405d1c60
    Replacement::draft(0x1405d1c90, "unknown_1405d1c90", tpl_62838::<0x143c8d904, 0x143c8d910, 0x143cb8eb4, 0x142489ee0, 0x143cb8ec8, 0x143c8e840, 0x141b05e40> as *const ()), // sub_1405d1c90
    Replacement::draft(0x1405d1f10, "unknown_1405d1f10", tpl_62838::<0x143c8d904, 0x143c8d910, 0x143cb8eb4, 0x142489ee0, 0x143cb8fc8, 0x143c98afc, 0x141b05c80> as *const ()), // sub_1405d1f10
    Replacement::draft(0x1405d2190, "unknown_1405d2190", tpl_62741::<0x143cb8eb4, 0x142489ee0, 0x143cb8fc8, 0x143cb8420, 0x141b05bc0> as *const ()), // sub_1405d2190
    Replacement::draft(0x1405d2b70, "unknown_1405d2b70", fn_1405d2b70 as *const ()), // sub_1405d2b70
    Replacement::draft_throws(0x1405d2c50, "bigfish_1405d2c50", tpl_1396::<0x143cb9290, 0x143cb9078, 0x1411e9f50, 0x1405d5770, 0x1405d74b0, 0x1405d5a50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e3cd6, 0x142646960, 0x143cb9098, 0x143cb90a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bullet_ray_cast::casting::system>
    Replacement::draft_throws(0x1405d2ea0, "bigfish_1405d2ea0", tpl_1396::<0x143cb8fd4, 0x143cb9380, 0x1411e9f50, 0x1405d5db0, 0x1405d7790, 0x1405d60e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e4696, 0x142646960, 0x143cb93a0, 0x143cb93a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bullet_ray_cast::results::system>
    Replacement::draft_throws(0x1405d30f0, "bigfish_1405d30f0", fn_1405d30f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::wounds::system>
    Replacement::draft(0x1405d3450, "unknown_1405d3450", tpl_1094::<0x143c8f3b8, 0x1405d3fd0, 0x14020e8f0> as *const ()), // sub_1405d3450
    Replacement::draft(0x1405d3650, "unknown_1405d3650", tpl_1094::<0x143c8e81c, 0x1405d41a0, 0x14020e8f0> as *const ()), // sub_1405d3650
    Replacement::draft(0x1405d3850, "unknown_1405d3850", tpl_1094::<0x143caf9ac, 0x1405d4320, 0x14020e8f0> as *const ()), // sub_1405d3850
    Replacement::draft(0x1405d3a50, "unknown_1405d3a50", fn_1405d3a50 as *const ()), // sub_1405d3a50
    Replacement::draft(0x1405d3c40, "unknown_1405d3c40", fn_1405d3c40 as *const ()), // sub_1405d3c40
    Replacement::draft(0x1405d3fd0, "unknown_1405d3fd0", fn_1405d3fd0 as *const ()), // sub_1405d3fd0
    Replacement::draft(0x1405d41a0, "unknown_1405d41a0", tpl_1191::<0x1401e6d50, 0x1405d6e60, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1402357d0> as *const ()), // sub_1405d41a0
    Replacement::draft(0x1405d4320, "unknown_1405d4320", tpl_4469::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140540100> as *const ()), // sub_1405d4320
    Replacement::draft(0x1405d44b0, "unknown_1405d44b0", fn_1405d44b0 as *const ()), // sub_1405d44b0
    Replacement::draft(0x1405d4bd0, "unknown_1405d4bd0", fn_1405d4bd0 as *const ()), // sub_1405d4bd0
    Replacement::draft_throws(0x1405d5770, "unknown_1405d5770", fn_1405d5770 as *const (), Throws::Discard), // sub_1405d5770
    Replacement::draft(0x1405d5a50, "unknown_1405d5a50", tpl_5583::<0x88, 0x10088, 0x1405d5b30, 0x1405d69d0, 0x1405d5c30> as *const ()), // sub_1405d5a50
    Replacement::draft(0x1405d5b30, "unknown_1405d5b30", tpl_3359::<0x143c8f63c, 0x20, 0x143cb90a8, 0x8, 0x143c8de64, 0x143c95654, 0x143c8d00c, 0x143c8d08c, 0x143c8eb9c, 0x143c8f048, 0x10, 0x18> as *const ()), // sub_1405d5b30
    Replacement::draft(0x1405d5c30, "unknown_1405d5c30", fn_1405d5c30 as *const ()), // sub_1405d5c30
    Replacement::draft_throws(0x1405d5db0, "unknown_1405d5db0", fn_1405d5db0 as *const (), Throws::Discard), // sub_1405d5db0
    Replacement::draft(0x1405d60e0, "unknown_1405d60e0", tpl_4977::<0x88, 0x10088, 0x1405d61c0, 0x1405d66d0, 0x1405d62f0> as *const ()), // sub_1405d60e0
    Replacement::draft(0x1405d61c0, "unknown_1405d61c0", tpl_3023::<0x8, 0x580e0, 0x20, 0x143cb92f8, 0x143cb9030, 0x143cb9288, 0x143c95654, 0x143c94b34, 0x143c8f63c, 0x143c8de64, 0x143c8d08c> as *const ()), // sub_1405d61c0
    Replacement::draft(0x1405d62f0, "unknown_1405d62f0", tpl_4978::<0x141b06e60> as *const ()), // sub_1405d62f0
    Replacement::draft(0x1405d6490, "unknown_1405d6490", tpl_63093::<0x143c8e81c, 0x142489ee0, 0x141b06d60> as *const ()), // sub_1405d6490
    Replacement::draft(0x1405d6590, "unknown_1405d6590", fn_1405d6590 as *const ()), // sub_1405d6590
    Replacement::draft(0x1405d6e20, "unknown_1405d6e20", tpl_1124::<0xb0> as *const ()), // sub_1405d6e20
    Replacement::draft(0x1405d6e60, "unknown_1405d6e60", fn_1405d6e60 as *const ()), // sub_1405d6e60
    Replacement::draft(0x1405d6ea0, "unknown_1405d6ea0", fn_1405d6ea0 as *const ()), // sub_1405d6ea0
    Replacement::draft(0x1405d7410, "unknown_1405d7410", fn_1405d7410 as *const ()), // sub_1405d7410
    Replacement::draft(0x1405d74b0, "unknown_1405d74b0", fn_1405d74b0 as *const ()), // sub_1405d74b0
    Replacement::draft(0x1405d7790, "unknown_1405d7790", tpl_64176::<0x1405d61c0, 0x1405d67d0, 0x141b06e60> as *const ()), // sub_1405d7790
    Replacement::draft(0x1405d7a80, "unknown_1405d7a80", tpl_63099::<0x143c8e81c, 0x142489ee0, 0x141b06d60> as *const ()), // sub_1405d7a80
    Replacement::draft(0x1405d7bd0, "unknown_1405d7bd0", fn_1405d7bd0 as *const ()), // sub_1405d7bd0
    Replacement::draft(0x1405d7fb0, "unknown_1405d7fb0", tpl_5834::<0x143cb8fd0, 0x143cb9140, 0x143c8ea68, 0x143c8d160, 0x143c8fa04, 0x143c8d120, 0x143c8eb98, 0x143c8dbcc, 0x143c8edbc, 0x143c8e81c, 0x143caf9ac, 0x143cb9160, 0x143cb9180, 0x143cb91a0, 0x141834a0c, 0x141834a78> as *const ()), // sub_1405d7fb0
    Replacement::draft(0x1405d82d0, "unknown_1405d82d0", tpl_5835::<0x143cb8fd8, 0x143cb90c0, 0x143c8f3b8, 0x143c8d160, 0x143c8e66c, 0x143c8f4cc, 0x143c8eb98, 0x143c8de7c, 0x143c8dbcc, 0x143c8edbc, 0x143c8e81c, 0x143caf9ac, 0x143cb90e0, 0x143cb9100, 0x143cb9120, 0x143cb9130, 0x143cb9138, 0x141834a0c, 0x141834a78> as *const ()), // sub_1405d82d0
    Replacement::draft(0x1405d8870, "unknown_1405d8870", tpl_1094::<0x143c9f01c, 0x1405d8aa0, 0x14020e8f0> as *const ()), // sub_1405d8870
    Replacement::draft(0x1405d8aa0, "unknown_1405d8aa0", tpl_3411::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14311f060> as *const ()), // sub_1405d8aa0
    Replacement::draft_throws(0x1405d8da0, "unknown_1405d8da0", fn_1405d8da0 as *const (), Throws::Discard), // sub_1405d8da0
    Replacement::draft_throws(0x1405d8e10, "unknown_1405d8e10", fn_1405d8e10 as *const (), Throws::Discard), // sub_1405d8e10
    Replacement::draft_throws(0x1405d8e80, "unknown_1405d8e80", fn_1405d8e80 as *const (), Throws::Discard), // sub_1405d8e80
    Replacement::draft_throws(0x1405d8ef0, "unknown_1405d8ef0", fn_1405d8ef0 as *const (), Throws::Discard), // sub_1405d8ef0
    Replacement::draft_throws(0x1405d8f60, "unknown_1405d8f60", fn_1405d8f60 as *const (), Throws::Discard), // sub_1405d8f60
    Replacement::draft_throws(0x1405d8fd0, "bigfish_1405d8fd0", tpl_1185::<0x143cb93b0, 0x143cb9580, 0x1411e9f50, 0x1405d9d70, 0x1405dac50, 0x1405d9f30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e6156, 0x142646960, 0x143cb95a0, 0x143cb95a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bonsai_player_info::update::system>
    Replacement::draft_throws(0x1405d9220, "bigfish_1405d9220", tpl_3369::<0x143cb941c, 0x143cb94d0, 0x1411e9f50, 0x1405db410, 0x143cb95cc, 0x1420af120, 0x1420a28e0, 0x1405daf80, 0x1405da270, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e7466, 0x142646960, 0x143cb94f0, 0x143cb9500, 0x143cb9508, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bonsai_player_info::update_character_space::system>
    Replacement::draft(0x1405d9510, "bigfish_1405d9510", tpl_60953::<0x1426467e8, 0x143c8dbcc, 0x142489ee0, 0x1405d958c> as *const ()), // sub_1405d9510
    Replacement::draft(0x1405d96c0, "bigfish_1405d96c0", tpl_1094::<0x143c8fabc, 0x1405d9bc0, 0x14020e8f0> as *const ()), // sub_1405d96c0
    Replacement::draft_throws(0x1405d98c0, "bigfish_1405d98c0", tpl_1101::<0x143cb95b8, 0x143cb95d0, 0x1411e9f50, 0x1405dba10, 0x143cb95bc, 0x1420af120, 0x1420a28e0, 0x1405db300, 0x1405dab90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e7656, 0x142646960, 0x143cb95f0, 0x143cb95f8, 0x143cb95fc, 0x143cb95fe, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bonsai_player_info::stream_in::system>
    Replacement::draft(0x1405d9bc0, "unknown_1405d9bc0", fn_1405d9bc0 as *const ()), // sub_1405d9bc0
    Replacement::draft_throws(0x1405d9d70, "unknown_1405d9d70", fn_1405d9d70 as *const (), Throws::Discard), // sub_1405d9d70
    Replacement::draft_throws(0x1405d9f30, "unknown_1405d9f30", fn_1405d9f30 as *const (), Throws::Discard), // sub_1405d9f30
    Replacement::draft_throws(0x1405da0a0, "unknown_1405da0a0", fn_1405da0a0 as *const (), Throws::Discard), // sub_1405da0a0
    Replacement::draft_throws(0x1405da270, "unknown_1405da270", fn_1405da270 as *const (), Throws::Discard), // sub_1405da270
    Replacement::draft(0x1405da350, "unknown_1405da350", fn_1405da350 as *const ()), // sub_1405da350
    Replacement::draft(0x1405da410, "unknown_1405da410", tpl_5079::<0x8, 0x10, 0x18, 0x40, 0x143c9d17c, 0x142489ee0, 0x143c8fabc, 0xc03, 0x20, 0x28> as *const ()), // sub_1405da410
    Replacement::draft(0x1405da4f0, "unknown_1405da4f0", fn_1405da4f0 as *const ()), // sub_1405da4f0
    Replacement::draft(0x1405da5f0, "unknown_1405da5f0", tpl_65008::<0x143c96de8, 0x142489ee0, 0x143c8dbd0, 0x143c91168, 0x143c8eb90, 0x143c8d474, 0x143c8d508, 0x143c8dbc4, 0x143c97a20, 0x143c91f48, 0x143c8e658, 0x143c8d160, 0x143c8e70c, 0x143c8d87c, 0x143c91f1c, 0x143c8d120, 0x143c8fabc, 0x143c97794> as *const ()), // sub_1405da5f0
    Replacement::draft(0x1405da9b0, "unknown_1405da9b0", fn_1405da9b0 as *const ()), // sub_1405da9b0
    Replacement::draft(0x1405dab60, "unknown_1405dab60", fn_1405dab60 as *const ()), // sub_1405dab60
    Replacement::draft(0x1405dab90, "unknown_1405dab90", tpl_1126::<0x142407c80, 0x580f0, 0x141b0b360> as *const ()), // sub_1405dab90
    Replacement::draft_throws(0x1405dac50, "unknown_1405dac50", fn_1405dac50 as *const (), Throws::Discard), // sub_1405dac50
    Replacement::draft_throws(0x1405daf80, "unknown_1405daf80", fn_1405daf80 as *const (), Throws::Discard), // sub_1405daf80
    Replacement::draft(0x1405db300, "unknown_1405db300", tpl_1130::<0x142407c80, 0x141b0b360> as *const ()), // sub_1405db300
    Replacement::draft(0x1405db660, "unknown_1405db660", fn_1405db660 as *const ()), // sub_1405db660
    Replacement::draft_throws(0x1405dbd50, "bigfish_1405dbd50", tpl_4129::<0x143cb9700, 0x143cb9748, 0x1411e9f50, 0x1405dc3c0, 0x143cb9708, 0x1420af120, 0x1420a28e0, 0x1405dc220, 0x1405dc040, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e7de6, 0x142646960, 0x143cb9768, 0x143cb976c, 0x143cb976e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bonsaiflowpassthrough::update>
    Replacement::draft(0x1405dc040, "bigfish_1405dc040", tpl_62755::<0x88, 0x10088, 0x1405dc120, 0x141b0ca70> as *const ()), // sub_1405dc040
    Replacement::draft(0x1405dc220, "bigfish_1405dc220", tpl_62763::<0x143c8ff58, 0x142489ee0, 0x143c8de68, 0x141b0ca70> as *const ()), // sub_1405dc220
    Replacement::draft_throws(0x1405dc540, "bigfish_1405dc540", tpl_3061::<0x143cb9ba8, 0x143cb9ac8, 0x1411e9f50, 0x1405dfc20, 0x143cb9b6c, 0x1420af120, 0x1420a28e0, 0x1405decc0, 0x1405ddf20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e8856, 0x142646960, 0x143cb9ae8, 0x143cb9af0, 0x143cb9af4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bodysensor::update_component::system>
    Replacement::draft_throws(0x1405dc830, "bigfish_1405dc830", tpl_5850::<0x143cb9ac4, 0x143cb9bb0, 0x1411e9f50, 0x1405df9c0, 0x143cb9854, 0x1420af120, 0x143c8fb50, 0x140215610, 0x1420a28e0, 0x1405deea0, 0x1405de100, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e80d6, 0x142646960, 0x143cb9bd0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bodysensor::update::system>
    Replacement::draft_throws(0x1405dcb30, "bigfish_1405dcb30", tpl_4895::<0x143cb9b68, 0x143cb9998, 0x1411e9f50, 0x1405df880, 0x143cb9850, 0x1420af120, 0x143c9a628, 0x140215610, 0x1420a28e0, 0x1405df120, 0x1405de2f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e85a6, 0x142646960, 0x143cb99b8, 0x143cb99c8, 0x143cb99d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bodysensor::update_position_reservation::system>
    Replacement::draft_throws(0x1405dce50, "bigfish_1405dce50", tpl_3274::<0x143cb9b9c, 0x143cb9a90, 0x1411e9f50, 0x1405df770, 0x143cb9ba4, 0x1420af120, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x1405df370, 0x1405de440, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e8cc6, 0x142646960, 0x143cb9ab0, 0x143cb9ac0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bodysensor::update_position_wlz::system>
    Replacement::draft(0x1405dd160, "bigfish_1405dd160", tpl_1435::<0x143c8faa0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405dd160
    Replacement::draft(0x1405dd4b0, "bigfish_1405dd4b0", tpl_1436::<0x58, 0x60, 0x158, 0x143c8faa0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405dd4b0
    Replacement::draft(0x1405dd590, "bigfish_1405dd590", tpl_1094::<0x143c90060, 0x1405ddd90, 0x14020e8f0> as *const ()), // sub_1405dd590
    Replacement::draft_throws(0x1405dd790, "bigfish_1405dd790", tpl_4129::<0x143cb9b60, 0x143cb97f0, 0x1411e9f50, 0x1405dfe50, 0x143cb9890, 0x1420af120, 0x1420a28e0, 0x1405df530, 0x1405deb10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e8ae6, 0x142646960, 0x143cb9810, 0x143cb9814, 0x143cb9816, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bodysensor::stream_in::system>
    Replacement::draft_throws(0x1405dda80, "bigfish_1405dda80", tpl_3682::<0x143cb9ba0, 0x143cb9af8, 0x1411e9f50, 0x1405dfd60, 0x143cb9b74, 0x1420af120, 0x1420a28e0, 0x1405df690, 0x1405dec20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431e95a6, 0x142646960, 0x143cb9b18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::bodysensor::stream_out::system>
    Replacement::draft(0x1405ddd60, "unknown_1405ddd60", tpl_1439::<0x8, 0x1405dd590> as *const ()), // sub_1405ddd60
    Replacement::draft(0x1405ddd90, "unknown_1405ddd90", tpl_3397::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143096c60> as *const ()), // sub_1405ddd90
    Replacement::draft(0x1405ddf20, "unknown_1405ddf20", tpl_1889::<0x40, 0x48, 0x10048, 0x1405de9c0, 0x1405de020> as *const ()), // sub_1405ddf20
    Replacement::draft(0x1405de020, "unknown_1405de020", tpl_1890::<0x18, 0x20, 0x40, 0x8, 0x141b0d5f0> as *const ()), // sub_1405de020
    Replacement::draft_throws(0x1405de100, "unknown_1405de100", tpl_4135::<0x88, 0x10088, 0x580e0, 0x143c8fb50, 0x1405de670, 0x1405de1c0> as *const (), Throws::Discard), // sub_1405de100
    Replacement::draft_throws(0x1405de1c0, "unknown_1405de1c0", tpl_4136::<0x18, 0x20, 0x40, 0x60, 0x68, 0x8, 0x141b0dac0> as *const (), Throws::Discard), // sub_1405de1c0
    Replacement::draft_throws(0x1405de2f0, "unknown_1405de2f0", tpl_64110::<0x88, 0x10088, 0x143c9a628, 0x580e0, 0x1405de540, 0x141b0d500> as *const (), Throws::Discard), // sub_1405de2f0
    Replacement::draft_throws(0x1405de440, "unknown_1405de440", tpl_62718::<0x88, 0x10088, 0x143c8fd28, 0x580e0, 0x140248e30, 0x141b0d390> as *const (), Throws::Discard), // sub_1405de440
    Replacement::draft(0x1405deb10, "unknown_1405deb10", tpl_1127::<0x142407c80, 0x143c90060, 0x142489ee0, 0x141b0dfc0> as *const ()), // sub_1405deb10
    Replacement::draft(0x1405dec20, "unknown_1405dec20", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1405dd4b0> as *const ()), // sub_1405dec20
    Replacement::draft(0x1405decc0, "unknown_1405decc0", tpl_61544::<0x1405de9c0, 0x141b0d5f0> as *const ()), // sub_1405decc0
    Replacement::draft_throws(0x1405deea0, "unknown_1405deea0", tpl_63371::<0x143c8fb50, 0x1405de790, 0x141b0dac0> as *const (), Throws::Discard), // sub_1405deea0
    Replacement::draft_throws(0x1405df120, "unknown_1405df120", tpl_64121::<0x143c9a628, 0x143c8d160, 0x142489ee0, 0x143c8e70c, 0x143c8f4d0, 0x141b0d500> as *const (), Throws::Discard), // sub_1405df120
    Replacement::draft_throws(0x1405df370, "unknown_1405df370", tpl_62736::<0x143c8fd28, 0x143c8d160, 0x142489ee0, 0x143c8f4d0, 0x141b0d390> as *const (), Throws::Discard), // sub_1405df370
    Replacement::draft(0x1405df530, "unknown_1405df530", tpl_1131::<0x142407c80, 0x143c90060, 0x142489ee0, 0x141b0dfc0> as *const ()), // sub_1405df530
    Replacement::draft(0x1405df690, "unknown_1405df690", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405dd4b0> as *const ()), // sub_1405df690
    Replacement::draft(0x1405df9c0, "unknown_1405df9c0", fn_1405df9c0 as *const ()), // sub_1405df9c0
    Replacement::draft(0x1405e02b0, "unknown_1405e02b0", tpl_5852::<0x142535070, 0x1426456c8, 0x1426453a0, 0x141ba4aa0, 0x1431f84c8, 0x1430f6b98, 0x1431eaeb8, 0x1431f8560> as *const ()), // sub_1405e02b0
    Replacement::draft(0x1405e0420, "unknown_1405e0420", tpl_3074::<0x143c8f8dc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405e0420
    Replacement::draft(0x1405e06f0, "unknown_1405e06f0", tpl_3278::<0x143c8fb0c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405e06f0
    Replacement::draft(0x1405e0a30, "unknown_1405e0a30", tpl_5853::<0x8, 0x1405e0420, 0x143428240, 0x1434268d0, 0x1405e06f0> as *const ()), // sub_1405e0a30
    Replacement::draft(0x1405e0c10, "unknown_1405e0c10", fn_1405e0c10 as *const ()), // sub_1405e0c10
    Replacement::draft(0x1405e0cd0, "unknown_1405e0cd0", fn_1405e0cd0 as *const ()), // sub_1405e0cd0
    Replacement::draft_throws(0x1405e0e10, "unknown_1405e0e10", fn_1405e0e10 as *const (), Throws::Caught), // sub_1405e0e10
    Replacement::draft_throws(0x1405e0ea0, "unknown_1405e0ea0", fn_1405e0ea0 as *const (), Throws::Discard), // sub_1405e0ea0
    Replacement::draft_throws(0x1405e0f60, "unknown_1405e0f60", fn_1405e0f60 as *const (), Throws::Caught), // sub_1405e0f60
    Replacement::draft_throws(0x1405e1510, "unknown_1405e1510", fn_1405e1510 as *const (), Throws::Caught), // sub_1405e1510
    Replacement::draft_throws(0x1405e16c0, "unknown_1405e16c0", tpl_5858::<0x14387c858, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1405e16c0
    Replacement::draft_throws(0x1405e1750, "unknown_1405e1750", fn_1405e1750 as *const (), Throws::Caught), // sub_1405e1750
    Replacement::draft_throws(0x1405e1be0, "unknown_1405e1be0", fn_1405e1be0 as *const (), Throws::Caught), // sub_1405e1be0
    Replacement::draft_throws(0x1405e1c70, "unknown_1405e1c70", tpl_5859::<0x14387c880, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c948, 0x14387c7a8> as *const (), Throws::Discard), // sub_1405e1c70
    Replacement::draft_throws(0x1405e1d50, "unknown_1405e1d50", fn_1405e1d50 as *const (), Throws::Caught), // sub_1405e1d50
    Replacement::draft_throws(0x1405e2310, "unknown_1405e2310", fn_1405e2310 as *const (), Throws::Caught), // sub_1405e2310
    Replacement::draft_throws(0x1405e24a0, "unknown_1405e24a0", tpl_1905::<0x14387c948, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c7a8> as *const (), Throws::Discard), // sub_1405e24a0
    Replacement::draft_throws(0x1405e2550, "unknown_1405e2550", fn_1405e2550 as *const (), Throws::Caught), // sub_1405e2550
    Replacement::draft_throws(0x1405e2960, "unknown_1405e2960", fn_1405e2960 as *const (), Throws::Caught), // sub_1405e2960
    Replacement::draft_throws(0x1405e29f0, "unknown_1405e29f0", tpl_1908::<0x14387c7a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1405e29f0
    Replacement::draft(0x1405e2ad0, "unknown_1405e2ad0", fn_1405e2ad0 as *const ()), // sub_1405e2ad0
    Replacement::draft(0x1405e2bb0, "unknown_1405e2bb0", fn_1405e2bb0 as *const ()), // sub_1405e2bb0
    Replacement::draft_throws(0x1405e2e30, "unknown_1405e2e30", fn_1405e2e30 as *const (), Throws::Caught), // sub_1405e2e30
    Replacement::draft_throws(0x1405e2fe0, "unknown_1405e2fe0", tpl_5858::<0x14387bf10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1405e2fe0
    Replacement::draft_throws(0x1405e3070, "unknown_1405e3070", fn_1405e3070 as *const (), Throws::Discard), // sub_1405e3070
    Replacement::draft(0x1405e3230, "bigfish_1405e3230", fn_1405e3230 as *const ()), // sub_1405e3230
    Replacement::draft(0x1405e3320, "bigfish_1405e3320", fn_1405e3320 as *const ()), // sub_1405e3320
    Replacement::draft_throws(0x1405e35b0, "bigfish_1405e35b0", fn_1405e35b0 as *const (), Throws::Caught), // sub_1405e35b0
    Replacement::draft_throws(0x1405e3760, "bigfish_1405e3760", tpl_5858::<0x14387bf18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1405e3760
    Replacement::draft_throws(0x1405e37f0, "bigfish_1405e37f0", fn_1405e37f0 as *const (), Throws::Discard), // sub_1405e37f0
    Replacement::draft(0x1405e3960, "bigfish_1405e3960", fn_1405e3960 as *const ()), // sub_1405e3960
    Replacement::draft(0x1405e3a10, "bigfish_1405e3a10", fn_1405e3a10 as *const ()), // sub_1405e3a10
    Replacement::draft_throws(0x1405e3c60, "bigfish_1405e3c60", fn_1405e3c60 as *const (), Throws::Caught), // sub_1405e3c60
    Replacement::draft_throws(0x1405e3e10, "bigfish_1405e3e10", tpl_5858::<0x14387bf28, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1405e3e10
    Replacement::draft(0x1405e3ed0, "bigfish_1405e3ed0", fn_1405e3ed0 as *const ()), // sub_1405e3ed0
    Replacement::draft(0x1405e3fa0, "bigfish_1405e3fa0", fn_1405e3fa0 as *const ()), // sub_1405e3fa0
    Replacement::draft_throws(0x1405e4210, "bigfish_1405e4210", fn_1405e4210 as *const (), Throws::Caught), // sub_1405e4210
    Replacement::draft_throws(0x1405e43c0, "bigfish_1405e43c0", tpl_5858::<0x14387bf30, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_1405e43c0
    Replacement::draft(0x1405e4480, "bigfish_1405e4480", fn_1405e4480 as *const ()), // sub_1405e4480
    Replacement::draft_throws(0x1405e47e0, "bigfish_1405e47e0", fn_1405e47e0 as *const (), Throws::Discard), // sub_1405e47e0
    Replacement::draft_throws(0x1405e4910, "bigfish_1405e4910", fn_1405e4910 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::lamp_indicator::system>
    Replacement::draft_throws(0x1405e4ba0, "bigfish_1405e4ba0", tpl_1776::<0x143cb9df8, 0x143cb9d98, 0x1411e9f50, 0x1405e52c0, 0x1405e5d30, 0x1405e5530, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431eba96, 0x142646960, 0x143cb9db8, 0x143cb9dba, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::battery_indicator::system>
    Replacement::draft_throws(0x1405e4de0, "bigfish_1405e4de0", fn_1405e4de0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::battery_indicator::syncUIWidgetVisibility>
    Replacement::draft(0x1405e50a0, "unknown_1405e50a0", fn_1405e50a0 as *const ()), // sub_1405e50a0
    Replacement::draft(0x1405e5100, "unknown_1405e5100", fn_1405e5100 as *const ()), // sub_1405e5100
    Replacement::draft_throws(0x1405e51b0, "unknown_1405e51b0", tpl_62823::<0x143c8ce88, 0x580e0, 0x143c8ec6c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141b14f60> as *const (), Throws::Discard), // sub_1405e51b0
    Replacement::draft_throws(0x1405e52c0, "unknown_1405e52c0", fn_1405e52c0 as *const (), Throws::Discard), // sub_1405e52c0
    Replacement::draft_throws(0x1405e5530, "unknown_1405e5530", fn_1405e5530 as *const (), Throws::Discard), // sub_1405e5530
    Replacement::draft(0x1405e5690, "unknown_1405e5690", tpl_1444::<0x8, 0x20, 0x143cb9c60, 0x143c8ce88, 0x143c94108, 0x143c8d910, 0x580e0, 0x143c9da5c, 0x143cb9d50> as *const ()), // sub_1405e5690
    Replacement::draft_throws(0x1405e5770, "unknown_1405e5770", fn_1405e5770 as *const (), Throws::Discard), // sub_1405e5770
    Replacement::draft_throws(0x1405e58a0, "unknown_1405e58a0", tpl_62823::<0x143c8ce88, 0x580e0, 0x143c8ec6c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x141b139d0> as *const (), Throws::Discard), // sub_1405e58a0
    Replacement::draft_throws(0x1405e5b10, "unknown_1405e5b10", tpl_1331::<0x8, 0x1404289f0, 0x143c9f934, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_1405e5b10
    Replacement::draft_throws(0x1405e5bc0, "unknown_1405e5bc0", tpl_62839::<0x143c8ce88, 0x143c8ec6c, 0x142489ee0, 0x141b14f60> as *const (), Throws::Discard), // sub_1405e5bc0
    Replacement::draft_throws(0x1405e5d30, "unknown_1405e5d30", fn_1405e5d30 as *const (), Throws::Discard), // sub_1405e5d30
    Replacement::draft_throws(0x1405e5f30, "unknown_1405e5f30", tpl_62839::<0x143c8ce88, 0x143c8ec6c, 0x142489ee0, 0x141b139d0> as *const (), Throws::Discard), // sub_1405e5f30
    Replacement::draft(0x1405e60a0, "unknown_1405e60a0", fn_1405e60a0 as *const ()), // sub_1405e60a0
    Replacement::draft(0x1405e63c0, "unknown_1405e63c0", fn_1405e63c0 as *const ()), // sub_1405e63c0
    Replacement::draft(0x1405e6670, "unknown_1405e6670", tpl_1526::<0x142778bd0, 0x1410d6ca0, 0x1405e72c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1405e6670
    Replacement::draft(0x1405e6910, "unknown_1405e6910", tpl_1094::<0x143c90164, 0x1405e75b0, 0x14020e8f0> as *const ()), // sub_1405e6910
    Replacement::draft(0x1405e6b10, "unknown_1405e6b10", tpl_1095::<0x143c92a98, 0x14020e8f0> as *const ()), // sub_1405e6b10
    Replacement::draft(0x1405e6cf0, "unknown_1405e6cf0", tpl_1095::<0x143c97d20, 0x14020e8f0> as *const ()), // sub_1405e6cf0
    Replacement::draft(0x1405e6ed0, "unknown_1405e6ed0", tpl_1095::<0x143c94188, 0x14020e8f0> as *const ()), // sub_1405e6ed0
    Replacement::draft(0x1405e70b0, "unknown_1405e70b0", tpl_1095::<0x143c9d17c, 0x14020e8f0> as *const ()), // sub_1405e70b0
    Replacement::draft(0x1405e72c0, "anonymous_namespace_1405e72c0", tpl_5879::<0x143cb9ed8, 0x143cb9eb0, 0x1411e9f50, 0x1405e78a0, 0x143c90164, 0x1401fc320, 0x143c8d0b4, 0x140215610, 0x1420a28e0, 0x1405e77f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ec446, 0x142646960, 0x143cb9ed0, 0x143cb9ed4, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::avatar_removed>
    Replacement::draft(0x1405e75b0, "unknown_1405e75b0", fn_1405e75b0 as *const ()), // sub_1405e75b0
    Replacement::draft(0x1405e77b0, "unknown_1405e77b0", tpl_1124::<0x120> as *const ()), // sub_1405e77b0
    Replacement::draft(0x1405e77f0, "unknown_1405e77f0", tpl_3172::<0x143c90164, 0x5e308, 0x143c8d0b4, 0x580e0, 0x10, 0x8, 0x18, 0x141b151e0, 0x1405e7ce0> as *const ()), // sub_1405e77f0
    Replacement::draft(0x1405e78a0, "unknown_1405e78a0", tpl_3188::<0x1405e8100, 0x1431ec4e8, 0x1405e7ce0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1405e78a0
    Replacement::draft(0x1405e7b70, "unknown_1405e7b70", fn_1405e7b70 as *const ()), // sub_1405e7b70
    Replacement::draft(0x1405e7ce0, "unknown_1405e7ce0", fn_1405e7ce0 as *const ()), // sub_1405e7ce0
    Replacement::draft(0x1405e7e00, "unknown_1405e7e00", fn_1405e7e00 as *const ()), // sub_1405e7e00
    Replacement::draft(0x1405e7f80, "unknown_1405e7f80", fn_1405e7f80 as *const ()), // sub_1405e7f80
    Replacement::draft(0x1405e8100, "unknown_1405e8100", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431ec4e8, 0x8, 0x20> as *const ()), // sub_1405e8100
    Replacement::draft(0x1405e81a0, "unknown_1405e81a0", fn_1405e81a0 as *const ()), // sub_1405e81a0
    Replacement::draft(0x1405e8220, "content_1405e8220", tpl_3904::<0x143c8d08c, 0x1401fa730, 0x143cb9f4c, 0x1426467e8, 0x1405e8970, 0x1405e832d> as *const ()), // sub_1405e8220
    Replacement::draft(0x1405e83c0, "content_1405e83c0", tpl_1094::<0x143cb9f8c, 0x1405e8ae0, 0x14020e8f0> as *const ()), // sub_1405e83c0
    Replacement::draft(0x1405e85c0, "content_1405e85c0", fn_1405e85c0 as *const ()), // sub_1405e85c0
    Replacement::draft(0x1405e8970, "content_1405e8970", tpl_61019::<0x143c914b0, 0x142489ee0, 0x143c914dc, 0x143c8d160, 0x1426467e8, 0x1405e8ad3> as *const ()), // sub_1405e8970
    Replacement::draft(0x1405e8ae0, "content_1405e8ae0", fn_1405e8ae0 as *const ()), // sub_1405e8ae0
    Replacement::draft(0x1405e8cf0, "content_1405e8cf0", fn_1405e8cf0 as *const ()), // content::AutoAimComponent::content::AutoAimComponent::vfunc_0
    Replacement::draft_throws(0x1405e8f30, "unknown_1405e8f30", tpl_5001::<0x143cb9f4c, 0x143cb9ee0, 0x143cb9fd8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c914b0, 0x143c914dc, 0x143c8d160, 0x143cb9ff8, 0x141834a0c> as *const (), Throws::Discard), // sub_1405e8f30
    Replacement::draft(0x1405e9250, "unknown_1405e9250", fn_1405e9250 as *const ()), // sub_1405e9250
    Replacement::draft(0x1405e9470, "unknown_1405e9470", fn_1405e9470 as *const ()), // sub_1405e9470
    Replacement::draft_throws(0x1405e94e0, "bigfish_1405e94e0", tpl_5893::<0x143cba1b4, 0x143cba048, 0x1411e9f50, 0x1405ea120, 0x1405ea290, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ed5e6, 0x142646960, 0x143cba068, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::audio_surface_material::update::system>
    Replacement::draft(0x1405e9720, "bigfish_1405e9720", fn_1405e9720 as *const ()), // sub_1405e9720
    Replacement::draft(0x1405e97a0, "bigfish_1405e97a0", tpl_1466::<0x143c9d61c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405e97a0
    Replacement::draft(0x1405e9a70, "bigfish_1405e9a70", tpl_1436::<0x58, 0x60, 0x158, 0x143c9d61c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405e9a70
    Replacement::draft_throws(0x1405e9b50, "bigfish_1405e9b50", tpl_3211::<0x143cba24c, 0x143cba140, 0x1411e9f50, 0x1405ea9a0, 0x143cba0b8, 0x1420af120, 0x1420a28e0, 0x1405ea570, 0x1405ea310, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ed2c6, 0x142646960, 0x143cba160, 0x143cba170, 0x143cba172, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::audio_surface_material::stream_in::system>
    Replacement::draft_throws(0x1405e9e40, "bigfish_1405e9e40", tpl_3691::<0x143cba0bc, 0x143cba180, 0x1411e9f50, 0x1405ea8a0, 0x143cba078, 0x1420af120, 0x1420a28e0, 0x1405ea680, 0x1405ea3d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ed0c6, 0x142646960, 0x143cba1a0, 0x143cba1b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::audio_surface_material::stream_out::system>
    Replacement::draft_throws(0x1405ea120, "unknown_1405ea120", tpl_4746::<0x1405ea760, 0x143cba100, 0x1420af120, 0x140215750, 0x143cba0c0, 0x140215610, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405ea120
    Replacement::draft(0x1405ea290, "unknown_1405ea290", tpl_4431::<0x143cba100, 0x20, 0x143cba0c0, 0x8, 0x580e0, 0x143c8f63c, 0x141b17140> as *const ()), // sub_1405ea290
    Replacement::draft(0x1405ea310, "unknown_1405ea310", tpl_1126::<0x142407c80, 0x580f0, 0x141b170c0> as *const ()), // sub_1405ea310
    Replacement::draft(0x1405ea3d0, "unknown_1405ea3d0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1405e9a70> as *const ()), // sub_1405ea3d0
    Replacement::draft(0x1405ea570, "unknown_1405ea570", tpl_1130::<0x142407c80, 0x141b170c0> as *const ()), // sub_1405ea570
    Replacement::draft(0x1405ea680, "unknown_1405ea680", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405e9a70> as *const ()), // sub_1405ea680
    Replacement::draft(0x1405eac50, "unknown_1405eac50", tpl_1526::<0x14277a7b0, 0x1410d6ca0, 0x1405eaef0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1405eac50
    Replacement::draft(0x1405eaef0, "bigfish_1405eaef0", fn_1405eaef0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::aspect_ratio_override::transition_aspect_ratio_override::system>
    Replacement::draft_throws(0x1405eb200, "bigfish_1405eb200", tpl_1467::<0x143cba2e0, 0x143cba310, 0x1411e9f50, 0x1405eb860, 0x143cba2ec, 0x1420af120, 0x1420a28e0, 0x1405eb6c0, 0x1405eb4e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ed986, 0x142646960, 0x143cba330, 0x143cba338, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::animation::update_teleport::system>
    Replacement::draft(0x1405eb4e0, "unknown_1405eb4e0", tpl_62755::<0x88, 0x10088, 0x1405eb5c0, 0x141b17c90> as *const ()), // sub_1405eb4e0
    Replacement::draft(0x1405eb5c0, "unknown_1405eb5c0", tpl_3492::<0x143c8e840, 0x142489ee0, 0x143c91f1c> as *const ()), // sub_1405eb5c0
    Replacement::draft(0x1405eb6c0, "unknown_1405eb6c0", tpl_62763::<0x143c8e840, 0x142489ee0, 0x143c91f1c, 0x141b17c90> as *const ()), // sub_1405eb6c0
    Replacement::draft(0x1405eb860, "unknown_1405eb860", tpl_5896::<0x143cba2e4, 0x143cba2f0, 0x8, 0x10, 0x18, 0x141834a78, 0x143c8e840, 0x143c91f1c, 0x143cba300, 0x143cba308, 0x141834a0c> as *const ()), // sub_1405eb860
    Replacement::draft_throws(0x1405eb950, "unknown_1405eb950", tpl_4541::<0x143cba2ec, 0x1405eb860, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_1405eb950
    Replacement::draft(0x1405eb9e0, "unknown_1405eb9e0", tpl_1::<0x143c896f0> as *const ()), // sub_1405eb9e0
    Replacement::draft(0x1405eb9f0, "global_1405eb9f0", fn_1405eb9f0 as *const ()), // vsnprintf
    Replacement::draft(0x1405eba60, "unknown_1405eba60", fn_1405eba60 as *const ()), // sub_1405eba60
    Replacement::draft_throws(0x1405eba80, "unknown_1405eba80", fn_1405eba80 as *const (), Throws::Caught), // sub_1405eba80
    Replacement::draft(0x1405ebb80, "unknown_1405ebb80", fn_1405ebb80 as *const ()), // sub_1405ebb80
    Replacement::draft(0x1405ebc30, "unknown_1405ebc30", tpl_64920::<0x1401e7c10, 0x143c8d120, 0x142489ee0, 0x143c8efb0> as *const ()), // sub_1405ebc30
    Replacement::draft(0x1405ebda0, "unknown_1405ebda0", fn_1405ebda0 as *const ()), // sub_1405ebda0
    Replacement::draft_throws(0x1405ebe00, "bigfish_1405ebe00", tpl_1396::<0x143cba5a0, 0x143cba3e0, 0x1411e9f50, 0x1405ecbc0, 0x1405edc00, 0x1405ecd60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431eee76, 0x142646960, 0x143cba400, 0x143cba408, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::analytics::player_status::system>
    Replacement::draft_throws(0x1405ec050, "bigfish_1405ec050", tpl_2988::<0x143cba4a4, 0x143cba410, 0x1411e9f50, 0x1405ed010, 0x1405edf10, 0x1405ed170, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ef5b6, 0x142646960, 0x143cba430, 0x143cba438, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::analytics::player_death::system>
    Replacement::draft_throws(0x1405ec2a0, "bigfish_1405ec2a0", fn_1405ec2a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::analytics::enemy::system>
    Replacement::draft_throws(0x1405ec5e0, "bigfish_1405ec5e0", tpl_3004::<0x143cba4f8, 0x143cba478, 0x1405eca30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431eecd6, 0x142646960, 0x143cba498, 0x143cba49c, 0x143cba49e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::analytics::lua_events::system>
    Replacement::draft_throws(0x1405ec7f0, "bigfish_1405ec7f0", tpl_1583::<0x143cba6b8, 0x143cba380, 0x1411e9f50, 0x1405ed540, 0x1405ee480, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ee6e6, 0x142646960, 0x143cba3a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::analytics::performance::system>
    Replacement::draft_throws(0x1405eca30, "unknown_1405eca30", fn_1405eca30 as *const (), Throws::Discard), // sub_1405eca30
    Replacement::draft_throws(0x1405ecbc0, "unknown_1405ecbc0", fn_1405ecbc0 as *const (), Throws::Discard), // sub_1405ecbc0
    Replacement::draft(0x1405ecd60, "unknown_1405ecd60", fn_1405ecd60 as *const ()), // sub_1405ecd60
    Replacement::draft(0x1405ecec0, "unknown_1405ecec0", fn_1405ecec0 as *const ()), // sub_1405ecec0
    Replacement::draft_throws(0x1405ed010, "unknown_1405ed010", fn_1405ed010 as *const (), Throws::Discard), // sub_1405ed010
    Replacement::draft(0x1405ed170, "unknown_1405ed170", fn_1405ed170 as *const ()), // sub_1405ed170
    Replacement::draft_throws(0x1405ed330, "unknown_1405ed330", fn_1405ed330 as *const (), Throws::Discard), // sub_1405ed330
    Replacement::draft_throws(0x1405ed450, "unknown_1405ed450", tpl_1469::<0x18, 0x20, 0x40, 0x8, 0x48, 0x141b18ee0> as *const (), Throws::Discard), // sub_1405ed450
    Replacement::draft_throws(0x1405ed540, "unknown_1405ed540", tpl_5909::<0x1405ee510, 0x143cba4b0, 0x1420af120, 0x140215750, 0x143cba55c, 0x140215610, 0x143c904a0, 0x143c8cec4, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405ed540
    Replacement::draft(0x1405ed6f0, "unknown_1405ed6f0", tpl_4382::<0x143c8f0d0, 0x142489ee0, 0x143c8d87c, 0x143c8e70c, 0x143c8fab0, 0x143c8fa04, 0x143c8d694> as *const ()), // sub_1405ed6f0
    Replacement::draft_throws(0x1405ed880, "unknown_1405ed880", tpl_1330::<0x8, 0x14041d710, 0x143c9ecd4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1405ed880
    Replacement::draft(0x1405edc00, "unknown_1405edc00", fn_1405edc00 as *const ()), // sub_1405edc00
    Replacement::draft(0x1405edf10, "unknown_1405edf10", fn_1405edf10 as *const ()), // sub_1405edf10
    Replacement::draft_throws(0x1405ee1b0, "unknown_1405ee1b0", fn_1405ee1b0 as *const (), Throws::Discard), // sub_1405ee1b0
    Replacement::draft_throws(0x1405ee3f0, "unknown_1405ee3f0", tpl_4147::<0x143c9ecd4, 0x143c8d08c, 0x20, 0x143c8ceec, 0x8, 0x580e0, 0x143cba55c, 0x141b186a0> as *const (), Throws::Discard), // sub_1405ee3f0
    Replacement::draft(0x1405ee480, "unknown_1405ee480", tpl_5910::<0x143cba4b0, 0x143c904a0, 0x20, 0x143cba55c, 0x8, 0x580e0, 0x143c8cec4, 0x141b18a40> as *const ()), // sub_1405ee480
    Replacement::draft(0x1405ee7d0, "unknown_1405ee7d0", fn_1405ee7d0 as *const ()), // sub_1405ee7d0
    Replacement::draft(0x1405ee9e0, "unknown_1405ee9e0", tpl_5912::<0xfffffffffffffffe, 0x1405ebb80, 0x142442de0> as *const ()), // sub_1405ee9e0
    Replacement::draft(0x1405eea30, "unknown_1405eea30", fn_1405eea30 as *const ()), // sub_1405eea30
    Replacement::draft_throws(0x1405eed80, "unknown_1405eed80", tpl_3734::<0x8, 0x1405efbf0, 0x14241c190> as *const (), Throws::Discard), // sub_1405eed80
    Replacement::draft(0x1405eedb0, "unknown_1405eedb0", tpl_5914::<0x10, 0x38, 0x8, 0x1405eeef0, 0x18, 0x1401de830, 0x1401e66f0, 0x20, 0x1401e6780> as *const ()), // sub_1405eedb0
    Replacement::draft(0x1405eee80, "unknown_1405eee80", tpl_5915::<0x8, 0x18, 0x1401de830, 0x1401e66f0> as *const ()), // sub_1405eee80
    Replacement::draft(0x1405eeef0, "unknown_1405eeef0", fn_1405eeef0 as *const ()), // sub_1405eeef0
    Replacement::draft_throws(0x1405ef220, "bigfish_1405ef220", tpl_3654::<0x143cba810, 0x143cba710, 0x1411e9f50, 0x1405efe90, 0x1405f0220, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431efd56, 0x142646960, 0x143cba730, 0x143cba738, 0x143cba73a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_ticket_scoring::update::system>
    Replacement::draft_throws(0x1405ef470, "std_1405ef470", tpl_5917::<0x14307e950, 0x142645870, 0x1401fc200, 0x140206290, 0x1405ef6b0, 0x1405ef529> as *const (), Throws::Discard), // sub_1405ef470
    Replacement::draft(0x1405ef680, "std_1405ef680", tpl_5918::<0x8, 0x1401de830, 0x1401fb4d0> as *const ()), // sub_1405ef680
    Replacement::draft_throws(0x1405ef6b0, "std_1405ef6b0", fn_1405ef6b0 as *const (), Throws::Discard), // sub_1405ef6b0
    Replacement::draft(0x1405ef820, "unknown_1405ef820", fn_1405ef820 as *const ()), // sub_1405ef820
    Replacement::draft(0x1405ef830, "unknown_1405ef830", tpl_1094::<0x143cba740, 0x1405efcf0, 0x14020e8f0> as *const ()), // sub_1405ef830
    Replacement::draft(0x1405efa30, "unknown_1405efa30", fn_1405efa30 as *const ()), // sub_1405efa30
    Replacement::draft(0x1405efbc0, "unknown_1405efbc0", tpl_1439::<0x8, 0x1405ef830> as *const ()), // sub_1405efbc0
    Replacement::draft_throws(0x1405efbf0, "unknown_1405efbf0", fn_1405efbf0 as *const (), Throws::Discard), // sub_1405efbf0
    Replacement::draft_throws(0x1405efc80, "unknown_1405efc80", fn_1405efc80 as *const (), Throws::Discard), // sub_1405efc80
    Replacement::draft(0x1405efcf0, "unknown_1405efcf0", tpl_1951::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431f01a8, 0x1405f0020> as *const ()), // sub_1405efcf0
    Replacement::draft_throws(0x1405efe90, "unknown_1405efe90", tpl_5922::<0x1405f04a0, 0x143cba708, 0x1420af120, 0x140215750, 0x1403bc900, 0x143cba6fc, 0x140215610, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405efe90
    Replacement::draft(0x1405f0020, "unknown_1405f0020", fn_1405f0020 as *const ()), // sub_1405f0020
    Replacement::draft(0x1405f00a0, "unknown_1405f00a0", fn_1405f00a0 as *const ()), // sub_1405f00a0
    Replacement::draft_throws(0x1405f0220, "unknown_1405f0220", tpl_5925::<0x8, 0x20, 0x143cba708, 0x143c8d07c, 0x580e0, 0x143cba6fc, 0x143c8d910, 0x141b1aa40> as *const (), Throws::Discard), // sub_1405f0220
    Replacement::draft(0x1405f02f0, "unknown_1405f02f0", fn_1405f02f0 as *const ()), // sub_1405f02f0
    Replacement::draft_throws(0x1405f0620, "unknown_1405f0620", fn_1405f0620 as *const (), Throws::Discard), // sub_1405f0620
    Replacement::draft(0x1405f07c0, "unknown_1405f07c0", tpl_5928::<0x142537280, 0x1426456c8, 0x1426453a0, 0x1401fc200, 0x1401e5730> as *const ()), // sub_1405f07c0
    Replacement::draft(0x1405f0990, "unknown_1405f0990", fn_1405f0990 as *const ()), // sub_1405f0990
    Replacement::draft(0x1405f0b90, "unknown_1405f0b90", fn_1405f0b90 as *const ()), // sub_1405f0b90
    Replacement::draft(0x1405f0c70, "unknown_1405f0c70", fn_1405f0c70 as *const ()), // sub_1405f0c70
    Replacement::draft(0x1405f0cd0, "unknown_1405f0cd0", fn_1405f0cd0 as *const ()), // sub_1405f0cd0
    Replacement::draft(0x1405f0e60, "unknown_1405f0e60", fn_1405f0e60 as *const ()), // sub_1405f0e60
    Replacement::draft_throws(0x1405f0f20, "std_1405f0f20", fn_1405f0f20 as *const (), Throws::Discard), // sub_1405f0f20
    Replacement::draft(0x1405f1160, "unknown_1405f1160", tpl_1149::<0x8, 0x1405f2200, 0x1401e66f0> as *const ()), // sub_1405f1160
    Replacement::draft(0x1405f1190, "unknown_1405f1190", fn_1405f1190 as *const ()), // sub_1405f1190
    Replacement::draft(0x1405f11d0, "unknown_1405f11d0", tpl_64921::<0x10, 0x18, 0x143ca78cc, 0x1806c, 0x18058, 0x142489ee0, 0x18068, 0x143c8f4d4, 0x18050, 0x20, 0x38> as *const ()), // sub_1405f11d0
    Replacement::draft_throws(0x1405f12b0, "unknown_1405f12b0", fn_1405f12b0 as *const (), Throws::Caught), // sub_1405f12b0
    Replacement::draft(0x1405f1a70, "unknown_1405f1a70", tpl_5937::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1405f2280> as *const ()), // sub_1405f1a70
    Replacement::draft(0x1405f1b70, "bigfish_1405f1b70", tpl_5938::<0x143cba878, 0x143cba840, 0x1405f22f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f0916, 0x142646960, 0x143cba860, 0x143cba868, 0x143cba86a, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bigfish::ai_tickets::query_removed::system>
    Replacement::draft_throws(0x1405f1d80, "bigfish_1405f1d80", tpl_1532::<0x143cba86c, 0x143cba8a0, 0x1411e9f50, 0x1405f2730, 0x1405f2d50, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f0a16, 0x142646960, 0x143cba8c0, 0x143cba8c4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_tickets::prepare::system>
    Replacement::draft_throws(0x1405f1fc0, "bigfish_1405f1fc0", tpl_5273::<0x143cba81c, 0x143cba8c8, 0x1411e9f50, 0x1405f28a0, 0x1405f2dd0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f0716, 0x142646960, 0x143cba8e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_tickets::update::system>
    Replacement::draft(0x1405f2200, "unknown_1405f2200", tpl_60956::<0x19, 0x10, 0x1405f2200, 0x28, 0x140260f30, 0x1401e66f0> as *const ()), // sub_1405f2200
    Replacement::draft(0x1405f2280, "unknown_1405f2280", fn_1405f2280 as *const ()), // sub_1405f2280
    Replacement::draft(0x1405f22f0, "unknown_1405f22f0", tpl_5024::<0x1411e9f50, 0x1405f2e60, 0x143ca78cc, 0x1401fc320, 0x143c8d004, 0x140215610, 0x143c8d910, 0x1420a28e0, 0x1405f2c90, 0x1401de410> as *const ()), // sub_1405f22f0
    Replacement::draft_throws(0x1405f2490, "unknown_1405f2490", fn_1405f2490 as *const (), Throws::Caught), // sub_1405f2490
    Replacement::draft_throws(0x1405f2730, "unknown_1405f2730", tpl_4746::<0x1405f34f0, 0x143cba874, 0x1420af120, 0x140215750, 0x143c8d004, 0x140215610, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405f2730
    Replacement::draft_throws(0x1405f28a0, "unknown_1405f28a0", tpl_4487::<0x1405f3400, 0x143cba928, 0x1420af120, 0x140215750, 0x143c8d004, 0x140215610, 0x143cba6fc, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_1405f28a0
    Replacement::draft(0x1405f2a50, "unknown_1405f2a50", fn_1405f2a50 as *const ()), // sub_1405f2a50
    Replacement::draft(0x1405f2b50, "unknown_1405f2b50", fn_1405f2b50 as *const ()), // sub_1405f2b50
    Replacement::draft_throws(0x1405f2c90, "unknown_1405f2c90", fn_1405f2c90 as *const (), Throws::Caught), // sub_1405f2c90
    Replacement::draft_throws(0x1405f2d50, "unknown_1405f2d50", tpl_5944::<0x143cba874, 0x20, 0x143c8d004, 0x8, 0x580e0, 0x143c8d910, 0x141b1c730> as *const (), Throws::Discard), // sub_1405f2d50
    Replacement::draft_throws(0x1405f2dd0, "unknown_1405f2dd0", tpl_4147::<0x143cba928, 0x143cba6fc, 0x20, 0x143c8d004, 0x8, 0x580e0, 0x143c8d910, 0x141b1bb30> as *const (), Throws::Discard), // sub_1405f2dd0
    Replacement::draft(0x1405f2e60, "unknown_1405f2e60", tpl_3188::<0x1405f35e0, 0x1431f0300, 0x1405f32c0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1405f2e60
    Replacement::draft(0x1405f2ee0, "unknown_1405f2ee0", tpl_4348::<0x10, 0x18, 0x1405f32c0> as *const ()), // sub_1405f2ee0
    Replacement::draft(0x1405f3150, "unknown_1405f3150", fn_1405f3150 as *const ()), // sub_1405f3150
    Replacement::draft(0x1405f32c0, "unknown_1405f32c0", fn_1405f32c0 as *const ()), // sub_1405f32c0
    Replacement::draft(0x1405f35e0, "unknown_1405f35e0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431f0300, 0x8, 0x20> as *const ()), // sub_1405f35e0
    Replacement::draft(0x1405f3680, "unknown_1405f3680", tpl_5949::<0x8, 0x1404bdab0> as *const ()), // sub_1405f3680
    Replacement::draft(0x1405f36c0, "unknown_1405f36c0", fn_1405f36c0 as *const ()), // sub_1405f36c0
    Replacement::draft(0x1405f3b60, "unknown_1405f3b60", fn_1405f3b60 as *const ()), // sub_1405f3b60
    Replacement::draft(0x1405f3c90, "unknown_1405f3c90", tpl_1663::<0x8, 0x1405f2200, 0x1401e66f0> as *const ()), // sub_1405f3c90
    Replacement::draft(0x1405f3d00, "unknown_1405f3d00", fn_1405f3d00 as *const ()), // sub_1405f3d00
    Replacement::draft_throws(0x1405f3ed0, "bigfish_1405f3ed0", fn_1405f3ed0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_targeting::update::system>
    Replacement::draft(0x1405f4220, "bigfish_1405f4220", tpl_1095::<0x143cbaa18, 0x14020e8f0> as *const ()), // sub_1405f4220
    Replacement::draft(0x1405f4400, "bigfish_1405f4400", tpl_1436::<0x58, 0x60, 0x158, 0x143cbaa18, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405f4400
    Replacement::draft(0x1405f44e0, "bigfish_1405f44e0", tpl_1403::<0x143cbacc4, 0x1405f4cb0, 0x14020e8f0> as *const ()), // sub_1405f44e0
    Replacement::draft_throws(0x1405f46f0, "bigfish_1405f46f0", tpl_3683::<0x143cbac88, 0x143cbab70, 0x1411e9f50, 0x1405f5d20, 0x143cbab68, 0x1420af120, 0x1420a28e0, 0x1405f5730, 0x1405f5120, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f0d76, 0x142646960, 0x143cbab90, 0x143cbab98, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_targeting::stream_in::system>
    Replacement::draft_throws(0x1405f49d0, "bigfish_1405f49d0", tpl_1437::<0x143cbaa98, 0x143cbac90, 0x1411e9f50, 0x1405f5c30, 0x143cbac80, 0x1420af120, 0x1420a28e0, 0x1405f5890, 0x1405f5230, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f0be6, 0x142646960, 0x143cbacb0, 0x143cbacb8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_targeting::stream_out::system>
    Replacement::draft(0x1405f4cb0, "unknown_1405f4cb0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431f1d70> as *const ()), // sub_1405f4cb0
    Replacement::draft_throws(0x1405f4e30, "unknown_1405f4e30", tpl_1470::<0x88, 0x10088, 0x143cbaa9c, 0x20, 0x8, 0x1405f4ff0, 0x1405f4f10> as *const (), Throws::Discard), // sub_1405f4e30
    Replacement::draft_throws(0x1405f4f10, "unknown_1405f4f10", tpl_1471::<0x18, 0x20, 0x30, 0x10, 0x28, 0x8, 0x141b1e020> as *const (), Throws::Discard), // sub_1405f4f10
    Replacement::draft(0x1405f5120, "unknown_1405f5120", tpl_1127::<0x142407c80, 0x143cbacc4, 0x142489ee0, 0x141b1e4e0> as *const ()), // sub_1405f5120
    Replacement::draft(0x1405f5230, "unknown_1405f5230", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1405f4400> as *const ()), // sub_1405f5230
    Replacement::draft_throws(0x1405f52d0, "unknown_1405f52d0", tpl_61190::<0x143cbaa9c, 0x143c8f0d0, 0x142489ee0, 0x143c8d160, 0x143c8f4d0, 0x141b1e020> as *const (), Throws::Discard), // sub_1405f52d0
    Replacement::draft(0x1405f5730, "unknown_1405f5730", tpl_1131::<0x142407c80, 0x143cbacc4, 0x142489ee0, 0x141b1e4e0> as *const ()), // sub_1405f5730
    Replacement::draft(0x1405f5890, "unknown_1405f5890", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405f4400> as *const ()), // sub_1405f5890
    Replacement::draft_throws(0x1405f60a0, "bigfish_1405f60a0", tpl_4129::<0x143cbad60, 0x143cbacd0, 0x1411e9f50, 0x1405f88e0, 0x143cbade4, 0x1420af120, 0x1420a28e0, 0x1405f7d60, 0x1405f7520, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f2bd6, 0x142646960, 0x143cbacf0, 0x143cbacf4, 0x143cbacf6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_locomotion::update::system>
    Replacement::draft_throws(0x1405f6390, "bigfish_1405f6390", tpl_5090::<0x143cbafd4, 0x143cbaf70, 0x1411e9f50, 0x1405f8720, 0x143cbaddc, 0x1420af120, 0x143c97a34, 0x140215610, 0x1420a28e0, 0x1405f7f70, 0x1405f7640, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f27e6, 0x142646960, 0x143cbaf90, 0x143cbafa0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_locomotion::update_locomotion::system>
    Replacement::draft(0x1405f66a0, "bigfish_1405f66a0", fn_1405f66a0 as *const ()), // sub_1405f66a0
    Replacement::draft(0x1405f6a00, "bigfish_1405f6a00", tpl_1436::<0x58, 0x60, 0x158, 0x143cbade0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1405f6a00
    Replacement::draft(0x1405f6ae0, "bigfish_1405f6ae0", tpl_1094::<0x143cbb020, 0x1405f7320, 0x14020e8f0> as *const ()), // sub_1405f6ae0
    Replacement::draft_throws(0x1405f6d10, "bigfish_1405f6d10", tpl_3003::<0x143cbae40, 0x143cbad30, 0x1411e9f50, 0x1405f8b10, 0x143cbb01c, 0x1420af120, 0x143c97a34, 0x140215610, 0x1420a28e0, 0x1405f8470, 0x1405f7bb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f2566, 0x142646960, 0x143cbad50, 0x143cbad58, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_locomotion::stream_in::system>
    Replacement::draft_throws(0x1405f7030, "bigfish_1405f7030", tpl_1438::<0x143cbae3c, 0x143cbafa8, 0x1411e9f50, 0x1405f8a20, 0x143cbafa4, 0x1420af120, 0x1420a28e0, 0x1405f8640, 0x1405f7cc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f23c6, 0x142646960, 0x143cbafc8, 0x143cbafd0, 0x143cbafd2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_locomotion::stream_out::system>
    Replacement::draft(0x1405f7320, "unknown_1405f7320", fn_1405f7320 as *const ()), // sub_1405f7320
    Replacement::draft(0x1405f7520, "unknown_1405f7520", tpl_62730::<0x88, 0x10088, 0x1405f7a80, 0x141b1f3c0> as *const ()), // sub_1405f7520
    Replacement::draft(0x1405f7640, "unknown_1405f7640", tpl_5032::<0x88, 0x10088, 0x580e0, 0x143c97a34, 0x1405f7810, 0x1405f7700> as *const ()), // sub_1405f7640
    Replacement::draft(0x1405f7700, "unknown_1405f7700", tpl_5033::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141b1f180> as *const ()), // sub_1405f7700
    Replacement::draft(0x1405f7bb0, "unknown_1405f7bb0", tpl_4477::<0x142407c80, 0x143c97a34, 0x1405f8180, 0x141b1f560> as *const ()), // sub_1405f7bb0
    Replacement::draft(0x1405f7cc0, "unknown_1405f7cc0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1405f6a00> as *const ()), // sub_1405f7cc0
    Replacement::draft(0x1405f7d60, "unknown_1405f7d60", tpl_62741::<0x143c8ff58, 0x142489ee0, 0x143cbade0, 0x143ca7600, 0x141b1f3c0> as *const ()), // sub_1405f7d60
    Replacement::draft(0x1405f7f70, "unknown_1405f7f70", tpl_64227::<0x143c97a34, 0x1405f78d0, 0x141b1f180> as *const ()), // sub_1405f7f70
    Replacement::draft(0x1405f8470, "unknown_1405f8470", tpl_4478::<0x142407c80, 0x143c97a34, 0x143cbb020, 0x142489ee0, 0x143ca7600, 0x141b1f560> as *const ()), // sub_1405f8470
    Replacement::draft(0x1405f8640, "unknown_1405f8640", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1405f6a00> as *const ()), // sub_1405f8640
    Replacement::draft(0x1405f8720, "unknown_1405f8720", fn_1405f8720 as *const ()), // sub_1405f8720
    Replacement::draft_throws(0x1405f8ed0, "unknown_1405f8ed0", fn_1405f8ed0 as *const (), Throws::Discard), // sub_1405f8ed0
    Replacement::draft_throws(0x1405f8fd0, "bigfish_1405f8fd0", fn_1405f8fd0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::interactableObjectRemoved>
    Replacement::draft(0x1405f9290, "bigfish_1405f9290", tpl_1095::<0x143cbb1e0, 0x14020e8f0> as *const ()), // sub_1405f9290
    Replacement::draft(0x1405f9470, "bigfish_1405f9470", tpl_1094::<0x143cbb228, 0x1405f9cf0, 0x14020e8f0> as *const ()), // sub_1405f9470
    Replacement::draft_throws(0x1405f96a0, "bigfish_1405f96a0", fn_1405f96a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::streamIn>
    Replacement::draft_throws(0x1405f9980, "bigfish_1405f9980", fn_1405f9980 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::streamOut>
    Replacement::draft(0x1405f9c70, "unknown_1405f9c70", tpl_3605::<0x19, 0x10, 0x1405f9c70, 0x28, 0x1401f2bc0, 0x1401e66f0> as *const ()), // sub_1405f9c70
    Replacement::draft(0x1405f9cf0, "unknown_1405f9cf0", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431f4068> as *const ()), // sub_1405f9cf0
    Replacement::draft_throws(0x1405f9e70, "unknown_1405f9e70", tpl_1498::<0x143cbb0c0, 0x143cbb234, 0x10, 0x1420a25b0, 0x141834a78, 0x143ca3d00, 0x143cbb230, 0x143cbb238, 0x141834a0c> as *const (), Throws::Discard), // sub_1405f9e70
    Replacement::draft_throws(0x1405f9f50, "unknown_1405f9f50", tpl_62933::<0x143cbb228, 0x142489ee0, 0x141b201f0> as *const (), Throws::Discard), // sub_1405f9f50
    Replacement::draft_throws(0x1405fa050, "unknown_1405fa050", tpl_62933::<0x143cbb228, 0x142489ee0, 0x141b20090> as *const (), Throws::Discard), // sub_1405fa050
    Replacement::draft_throws(0x1405fa150, "unknown_1405fa150", tpl_5499::<0x143cbb228, 0x5e308, 0x10, 0x8, 0x18, 0x141b20300, 0x1405fa560> as *const (), Throws::Discard), // sub_1405fa150
    Replacement::draft(0x1405fa1f0, "unknown_1405fa1f0", tpl_3188::<0x1405fab10, 0x1431f4040, 0x1405fa560, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1405fa1f0
    Replacement::draft(0x1405fa410, "unknown_1405fa410", fn_1405fa410 as *const ()), // sub_1405fa410
    Replacement::draft(0x1405fa560, "unknown_1405fa560", fn_1405fa560 as *const ()), // sub_1405fa560
    Replacement::draft_throws(0x1405fa870, "unknown_1405fa870", tpl_62934::<0x143cbb228, 0x142489ee0, 0x141b201f0> as *const (), Throws::Discard), // sub_1405fa870
    Replacement::draft_throws(0x1405fa9c0, "unknown_1405fa9c0", tpl_62934::<0x143cbb228, 0x142489ee0, 0x141b20090> as *const (), Throws::Discard), // sub_1405fa9c0
    Replacement::draft(0x1405fab10, "unknown_1405fab10", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431f4040, 0x8, 0x20> as *const ()), // sub_1405fab10
    Replacement::draft(0x1405fabb0, "unknown_1405fabb0", tpl_5962::<0x1405f9c70, 0x1405faf40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14025f9d0, 0x1401e66f0, 0x142442de0> as *const ()), // sub_1405fabb0
    Replacement::draft(0x1405fb110, "unknown_1405fb110", fn_1405fb110 as *const ()), // sub_1405fb110
    Replacement::draft(0x1405fb1d0, "unknown_1405fb1d0", tpl_1663::<0x8, 0x1405f9c70, 0x1401e66f0> as *const ()), // sub_1405fb1d0
    Replacement::draft_throws(0x1405fb210, "bigfish_1405fb210", tpl_3407::<0x143cbb2c8, 0x143cbb2d0, 0x1411e9f50, 0x1405fb930, 0x143cbb24c, 0x1420af120, 0x1420a28e0, 0x1405fb790, 0x1405fb500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f4246, 0x142646960, 0x143cbb2f0, 0x143cbb300, 0x143cbb304, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ai_aim::update_animation_parameters::system>
    Replacement::draft(0x1405fb500, "unknown_1405fb500", tpl_62531::<0x88, 0x10088, 0x1405fb630, 0x141b20a30> as *const ()), // sub_1405fb500
    Replacement::draft(0x1405fb790, "unknown_1405fb790", tpl_62567::<0x1405fb630, 0x141b20a30> as *const ()), // sub_1405fb790
    Replacement::draft(0x1405fbb00, "unknown_1405fbb00", tpl_1094::<0x143c992cc, 0x1405fbd30, 0x14020e8f0> as *const ()), // sub_1405fbb00
    Replacement::draft(0x1405fbd30, "unknown_1405fbd30", fn_1405fbd30 as *const ()), // sub_1405fbd30
    Replacement::draft(0x1405fbef0, "unknown_1405fbef0", fn_1405fbef0 as *const ()), // sub_1405fbef0
    Replacement::draft(0x1405fbf90, "unknown_1405fbf90", fn_1405fbf90 as *const ()), // sub_1405fbf90
    Replacement::draft(0x1405fc120, "unknown_1405fc120", fn_1405fc120 as *const ()), // sub_1405fc120
    Replacement::draft(0x1405fc330, "unknown_1405fc330", fn_1405fc330 as *const ()), // sub_1405fc330
    Replacement::draft(0x1405fc410, "unknown_1405fc410", fn_1405fc410 as *const ()), // sub_1405fc410
    Replacement::draft(0x1405fc4c0, "unknown_1405fc4c0", fn_1405fc4c0 as *const ()), // sub_1405fc4c0
    Replacement::draft(0x1405fc650, "unknown_1405fc650", fn_1405fc650 as *const ()), // sub_1405fc650
    Replacement::draft_throws(0x1405fc9e0, "bigfish_1405fc9e0", tpl_1776::<0x143cbb660, 0x143cbb398, 0x1411e9f50, 0x1405fdc20, 0x1405ff1e0, 0x1405fe020, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f6726, 0x142646960, 0x143cbb3b8, 0x143cbb3ba, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityinputlogic::update>
    Replacement::draft(0x1405fcc20, "unknown_1405fcc20", tpl_1094::<0x143cbb6cc, 0x1405fdaa0, 0x14020e8f0> as *const ()), // sub_1405fcc20
    Replacement::draft(0x1405fce20, "unknown_1405fce20", tpl_1369::<0x143cbb354, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405fce20
    Replacement::draft(0x1405fd160, "unknown_1405fd160", tpl_3406::<0x143c91f94, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405fd160
    Replacement::draft(0x1405fd4b0, "unknown_1405fd4b0", tpl_5972::<0x143cbb3c4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405fd4b0
    Replacement::draft(0x1405fd790, "unknown_1405fd790", tpl_5972::<0x143c9e9d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1405fd790
    Replacement::draft(0x1405fdaa0, "unknown_1405fdaa0", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431f5318> as *const ()), // sub_1405fdaa0
    Replacement::draft_throws(0x1405fdc20, "unknown_1405fdc20", fn_1405fdc20 as *const (), Throws::Discard), // sub_1405fdc20
    Replacement::draft_throws(0x1405fe020, "unknown_1405fe020", fn_1405fe020 as *const (), Throws::Discard), // sub_1405fe020
    Replacement::draft(0x1405fe140, "unknown_1405fe140", fn_1405fe140 as *const ()), // sub_1405fe140
    Replacement::draft_throws(0x1405fe2c0, "unknown_1405fe2c0", fn_1405fe2c0 as *const (), Throws::Discard), // sub_1405fe2c0
    Replacement::draft(0x1405fe740, "unknown_1405fe740", fn_1405fe740 as *const ()), // sub_1405fe740
    Replacement::draft(0x1405fe890, "unknown_1405fe890", fn_1405fe890 as *const ()), // sub_1405fe890
    Replacement::draft(0x1405feec0, "unknown_1405feec0", fn_1405feec0 as *const ()), // sub_1405feec0
    Replacement::draft_throws(0x1405ff1e0, "unknown_1405ff1e0", fn_1405ff1e0 as *const (), Throws::Discard), // sub_1405ff1e0
    Replacement::draft(0x1405ff460, "unknown_1405ff460", fn_1405ff460 as *const ()), // sub_1405ff460
    Replacement::draft(0x1405ffba0, "unknown_1405ffba0", fn_1405ffba0 as *const ()), // sub_1405ffba0
    Replacement::draft(0x1405ffd00, "unknown_1405ffd00", fn_1405ffd00 as *const ()), // sub_1405ffd00
    Replacement::draft(0x140600350, "unknown_140600350", fn_140600350 as *const ()), // sub_140600350
    Replacement::draft(0x1406003b0, "unknown_1406003b0", fn_1406003b0 as *const ()), // sub_1406003b0
    Replacement::draft(0x140600590, "unknown_140600590", fn_140600590 as *const ()), // sub_140600590
    Replacement::draft_throws(0x140600670, "bigfish_140600670", tpl_4703::<0x143cbb75c, 0x143cbb810, 0x140600de0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f7c16, 0x142646960, 0x143cbb820, 0x143cbb828, 0x143cbb82c, 0x143cbb82e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilitycrouch::update>
    Replacement::draft(0x140600880, "unknown_140600880", tpl_1094::<0x143cbb808, 0x140600fa0, 0x14020e8f0> as *const ()), // sub_140600880
    Replacement::draft(0x140600a80, "unknown_140600a80", tpl_1912::<0x143c96de4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140600a80
    Replacement::draft(0x140600d60, "unknown_140600d60", tpl_5985::<0x8, 0x140600880, 0x140600a80> as *const ()), // sub_140600d60
    Replacement::draft_throws(0x140600de0, "unknown_140600de0", tpl_5552::<0x1411e9f50, 0x1406018a0, 0x143cbb830, 0x1420af120, 0x143c8ceec, 0x140215610, 0x143c8f63c, 0x1420a28e0, 0x140601570, 0x140601120> as *const (), Throws::Discard), // sub_140600de0
    Replacement::draft(0x140600fa0, "unknown_140600fa0", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431f8350> as *const ()), // sub_140600fa0
    Replacement::draft_throws(0x140601120, "unknown_140601120", tpl_5455::<0x88, 0x10088, 0x580e0, 0x143c8f63c, 0x143c8ceec, 0x140601300, 0x1406011f0> as *const (), Throws::Discard), // sub_140601120
    Replacement::draft_throws(0x1406011f0, "unknown_1406011f0", tpl_5456::<0x18, 0x20, 0x50, 0x40, 0x8, 0x48, 0x141b29190> as *const (), Throws::Discard), // sub_1406011f0
    Replacement::draft_throws(0x140601570, "unknown_140601570", tpl_64609::<0x143c8ceec, 0x143c8f63c, 0x1406013c0, 0x141b29190> as *const (), Throws::Discard), // sub_140601570
    Replacement::draft(0x1406018a0, "unknown_1406018a0", tpl_5986::<0x143cbb804, 0x143cbb7a0, 0x141834a78, 0x143c96de4, 0x143c917cc, 0x143c8fab8, 0x143cbb808, 0x143ca72c4, 0x143c91f94, 0x143c8d508, 0x143cbb7c0, 0x143cbb7e0, 0x143cbb7f0, 0x141834a0c> as *const ()), // sub_1406018a0
    Replacement::draft(0x140601b40, "bonsai_140601b40", tpl_5987::<0x1431f8560, 0x98, 0x1430f6b98, 0x58, 0x1432316d0, 0x141834d30> as *const ()), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_0
    Replacement::draft(0x140601c10, "unknown_140601c10", fn_140601c10 as *const ()), // sub_140601c10
    Replacement::draft(0x140601db0, "unknown_140601db0", tpl_61197::<0x143c8d160, 0x142489ee0, 0x143ca4244, 0x143c8f4d0, 0x143ca4318, 0x1426467e8, 0x140601f43> as *const ()), // sub_140601db0
    Replacement::draft_throws(0x140601f50, "bigfish_140601f50", fn_140601f50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::updateActivation>
    Replacement::draft_throws(0x140602300, "bigfish_140602300", tpl_1535::<0x143cbbb64, 0x143cbbb38, 0x1411e9f50, 0x140604750, 0x1406051f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f8606, 0x142646960, 0x143cbbb58, 0x143cbbb60, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::updateWaves>
    Replacement::draft_throws(0x140602550, "bigfish_140602550", fn_140602550 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::updateReaction>
    Replacement::draft(0x1406028c0, "bigfish_1406028c0", fn_1406028c0 as *const ()), // sub_1406028c0
    Replacement::draft(0x140602ac0, "bigfish_140602ac0", fn_140602ac0 as *const ()), // sub_140602ac0
    Replacement::draft(0x140602c80, "bigfish_140602c80", tpl_1094::<0x143cbbecc, 0x140604240, 0x14020e8f0> as *const ()), // sub_140602c80
    Replacement::draft(0x140602e80, "bigfish_140602e80", tpl_1094::<0x143cbbc68, 0x140604400, 0x14020e8f0> as *const ()), // sub_140602e80
    Replacement::draft(0x140603080, "bigfish_140603080", tpl_1095::<0x143cbbc0c, 0x14020e8f0> as *const ()), // sub_140603080
    Replacement::draft(0x140603260, "bigfish_140603260", tpl_1098::<0x143c977a4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140603260
    Replacement::draft_throws(0x1406035a0, "bigfish_1406035a0", tpl_5993::<0x143cbbea8, 0x143cbb8f8, 0x1411e9f50, 0x140606890, 0x143cbbdd4, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1406059e0, 0x1406054b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f9646, 0x142646960, 0x143cbb918, 0x143cbb920, 0x143cbb922, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::streamInEnemy>
    Replacement::draft_throws(0x1406038a0, "bigfish_1406038a0", tpl_4834::<0x143cbbb68, 0x143cbb8c0, 0x1411e9f50, 0x1406067a0, 0x143cbbc70, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140605ad0, 0x140605540, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f9896, 0x142646960, 0x143cbb8e0, 0x143cbb8e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::streamOutEnemy>
    Replacement::draft_throws(0x140603ba0, "bigfish_140603ba0", tpl_4834::<0x143cbb878, 0x143cbbcf0, 0x1411e9f50, 0x1406066a0, 0x143cbba60, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140605bc0, 0x1406055d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f9ba6, 0x142646960, 0x143cbbd10, 0x143cbbd18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::streamInTarget>
    Replacement::draft_throws(0x140603ea0, "bigfish_140603ea0", tpl_4835::<0x143cbbd28, 0x143cbbda0, 0x1411e9f50, 0x1406065b0, 0x143cbbd20, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140605cb0, 0x140605660, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431f9df6, 0x142646960, 0x143cbbdc0, 0x143cbbdc8, 0x143cbbdcc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaitidalwave::streamOutTarget>
    Replacement::draft(0x1406041a0, "unknown_1406041a0", tpl_5994::<0x8, 0x140602c80, 0x10, 0x14384d970, 0x14384d96c, 0x14, 0x14384d8f0, 0x14384d8ec, 0x18, 0x14384d950, 0x14384d94c> as *const ()), // sub_1406041a0
    Replacement::draft(0x140604240, "unknown_140604240", fn_140604240 as *const ()), // sub_140604240
    Replacement::draft(0x140604400, "unknown_140604400", tpl_1914::<0x1401e6d50, 0x14027f460, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1431fb5a0> as *const ()), // sub_140604400
    Replacement::draft_throws(0x140604590, "unknown_140604590", tpl_5996::<0x88, 0x10088, 0x140604d80, 0x140604640> as *const (), Throws::Discard), // sub_140604590
    Replacement::draft_throws(0x140604640, "unknown_140604640", tpl_3284::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141b2d3d0> as *const (), Throws::Discard), // sub_140604640
    Replacement::draft_throws(0x140604750, "unknown_140604750", fn_140604750 as *const (), Throws::Discard), // sub_140604750
    Replacement::draft_throws(0x1406049e0, "unknown_1406049e0", fn_1406049e0 as *const (), Throws::Discard), // sub_1406049e0
    Replacement::draft_throws(0x140604b20, "unknown_140604b20", tpl_5999::<0x18, 0x20, 0x40, 0x8, 0x141b2bcd0> as *const (), Throws::Discard), // sub_140604b20
    Replacement::draft_throws(0x140604ff0, "unknown_140604ff0", tpl_65139::<0x140604e40, 0x141b2d3d0> as *const (), Throws::Discard), // sub_140604ff0
    Replacement::draft(0x1406051f0, "unknown_1406051f0", fn_1406051f0 as *const ()), // sub_1406051f0
    Replacement::draft_throws(0x1406052a0, "unknown_1406052a0", fn_1406052a0 as *const (), Throws::Discard), // sub_1406052a0
    Replacement::draft(0x1406054b0, "unknown_1406054b0", tpl_1898::<0x88, 0x10088, 0x141b2bb80> as *const ()), // sub_1406054b0
    Replacement::draft(0x140605540, "unknown_140605540", tpl_1898::<0x88, 0x10088, 0x141b2b9c0> as *const ()), // sub_140605540
    Replacement::draft(0x1406055d0, "unknown_1406055d0", tpl_1898::<0x88, 0x10088, 0x141b2baf0> as *const ()), // sub_1406055d0
    Replacement::draft(0x140605660, "unknown_140605660", tpl_1898::<0x88, 0x10088, 0x141b2b890> as *const ()), // sub_140605660
    Replacement::draft(0x1406059e0, "unknown_1406059e0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b2bb80> as *const ()), // sub_1406059e0
    Replacement::draft(0x140605ad0, "unknown_140605ad0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b2b9c0> as *const ()), // sub_140605ad0
    Replacement::draft(0x140605bc0, "unknown_140605bc0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b2baf0> as *const ()), // sub_140605bc0
    Replacement::draft(0x140605cb0, "unknown_140605cb0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b2b890> as *const ()), // sub_140605cb0
    Replacement::draft(0x140606260, "unknown_140606260", fn_140606260 as *const ()), // sub_140606260
    Replacement::draft(0x140606470, "unknown_140606470", tpl_5284::<0x141b29ef0, 0x141b29f70, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141b2b450, 0x141b2b300, 0x142442de0> as *const ()), // sub_140606470
    Replacement::draft(0x140606ee0, "unknown_140606ee0", tpl_63073::<0x143c9b928, 0x142489ee0, 0x143c8d120, 0x143c8e658> as *const ()), // sub_140606ee0
    Replacement::draft_throws(0x1406071c0, "bigfish_1406071c0", tpl_4201::<0x143cbbfc0, 0x143cbc430, 0x1411e9f50, 0x1406086f0, 0x140609260, 0x1406088d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fb8c6, 0x142646960, 0x143cbc450, 0x143cbc460, 0x143cbc468, 0x143cbc46a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_throw_weapon::update_targeting::system>
    Replacement::draft_throws(0x140607420, "bigfish_140607420", tpl_3067::<0x143cbc3f0, 0x143cbc158, 0x140607c60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fbe06, 0x142646960, 0x143cbc178, 0x143cbc188, 0x143cbc190, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_throw_weapon::update_throwing::system>
    Replacement::draft(0x140607640, "bigfish_140607640", tpl_1094::<0x143cbc354, 0x140608480, 0x14020e8f0> as *const ()), // sub_140607640
    Replacement::draft(0x140607840, "bigfish_140607840", tpl_1098::<0x143cbc044, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140607840
    Replacement::draft(0x140607b80, "bigfish_140607b80", tpl_1436::<0x58, 0x60, 0x158, 0x143cbc044, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140607b80
    Replacement::draft_throws(0x140607c60, "bigfish_140607c60", tpl_1187::<0x1411e9f50, 0x14060a020, 0x143cbc03c, 0x1420af120, 0x140609ee0, 0x143cbc040, 0x140215750, 0x143c8f4d8, 0x140215610, 0x1420a28e0, 0x1406094b0, 0x140608a50> as *const (), Throws::Discard), // sub_140607c60
    Replacement::draft_throws(0x140607e80, "bigfish_140607e80", tpl_3691::<0x143cbc1d8, 0x143cbc3f8, 0x1411e9f50, 0x14060a650, 0x143cbc3f4, 0x1420af120, 0x1420a28e0, 0x140609c50, 0x1406097d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fc5b6, 0x142646960, 0x143cbc418, 0x143cbc428, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_throw_weapon::stream_in::system>
    Replacement::draft_throws(0x140608160, "bigfish_140608160", tpl_3407::<0x143cbbff8, 0x143cbc2e0, 0x1411e9f50, 0x14060a560, 0x143cbc230, 0x1420af120, 0x1420a28e0, 0x140609e00, 0x1406098c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fc886, 0x142646960, 0x143cbc300, 0x143cbc310, 0x143cbc314, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_throw_weapon::stream_out::system>
    Replacement::draft(0x140608480, "unknown_140608480", fn_140608480 as *const ()), // sub_140608480
    Replacement::draft_throws(0x1406086f0, "unknown_1406086f0", tpl_1196::<0x14060a460, 0x143cbc4ac, 0x1420af120, 0x14060a2c0, 0x143cbc048, 0x140215750, 0x143c8d910, 0x140215610, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1406086f0
    Replacement::draft_throws(0x1406088d0, "unknown_1406088d0", fn_1406088d0 as *const (), Throws::Discard), // sub_1406088d0
    Replacement::draft_throws(0x140608a50, "unknown_140608a50", fn_140608a50 as *const (), Throws::Discard), // sub_140608a50
    Replacement::draft_throws(0x140608b60, "unknown_140608b60", fn_140608b60 as *const (), Throws::Discard), // sub_140608b60
    Replacement::draft(0x140608cc0, "content_140608cc0", fn_140608cc0 as *const ()), // content::AbilityAIThrowWeaponComponent::content::AbilityAIThrowWeaponComponent::vfunc_0
    Replacement::draft(0x140609220, "unknown_140609220", tpl_1124::<0x138> as *const ()), // sub_140609220
    Replacement::draft_throws(0x140609260, "unknown_140609260", fn_140609260 as *const (), Throws::Discard), // sub_140609260
    Replacement::draft_throws(0x1406094b0, "unknown_1406094b0", fn_1406094b0 as *const (), Throws::Discard), // sub_1406094b0
    Replacement::draft_throws(0x1406097d0, "unknown_1406097d0", fn_1406097d0 as *const (), Throws::Caught), // sub_1406097d0
    Replacement::draft(0x1406098c0, "unknown_1406098c0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140607b80> as *const ()), // sub_1406098c0
    Replacement::draft_throws(0x140609c50, "unknown_140609c50", fn_140609c50 as *const (), Throws::Caught), // sub_140609c50
    Replacement::draft(0x140609e00, "unknown_140609e00", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140607b80> as *const ()), // sub_140609e00
    Replacement::draft(0x14060a020, "unknown_14060a020", fn_14060a020 as *const ()), // sub_14060a020
    Replacement::draft(0x14060a2c0, "unknown_14060a2c0", tpl_1472::<0x143cbc1e0, 0x143cbc390, 0x141834a78, 0x143c8f4d4, 0x143c8d160, 0x143c8d120, 0x143c8ff58, 0x143cbc354, 0x143c8f4d0, 0x143cbc044, 0x143cbc3b0, 0x143cbc3d0, 0x143cbc3e0, 0x141834a0c> as *const ()), // sub_14060a2c0
    Replacement::draft(0x14060a9d0, "unknown_14060a9d0", fn_14060a9d0 as *const ()), // sub_14060a9d0
    Replacement::draft(0x14060ac10, "unknown_14060ac10", fn_14060ac10 as *const ()), // sub_14060ac10
    Replacement::draft(0x14060acb0, "unknown_14060acb0", tpl_2049::<0x8, 0xa0, 0x14240ab10> as *const ()), // sub_14060acb0
    Replacement::draft_throws(0x14060ad30, "bigfish_14060ad30", tpl_1492::<0x143cbc514, 0x143cbc588, 0x1411e9f50, 0x14060d1a0, 0x14060ead0, 0x14060d520, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431ffd96, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaimelee::system>
    Replacement::draft_throws(0x14060af60, "bigfish_14060af60", tpl_5893::<0x143cbcab4, 0x143cbc950, 0x1411e9f50, 0x14060d8c0, 0x14060eda0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fdc56, 0x142646960, 0x143cbc970, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaimelee::hit_processing::system>
    Replacement::draft_throws(0x14060b1a0, "bigfish_14060b1a0", tpl_3060::<0x143cbc89c, 0x143cbc6b8, 0x1411e9f50, 0x14060dbb0, 0x14060eef0, 0x14060dd30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fe1d6, 0x142646960, 0x143cbc6d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaimelee::weapon_impulse::system>
    Replacement::draft(0x14060b3e0, "bigfish_14060b3e0", fn_14060b3e0 as *const ()), // sub_14060b3e0
    Replacement::draft(0x14060b5e0, "bigfish_14060b5e0", tpl_6012::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0xc, 0x143200890> as *const ()), // sub_14060b5e0
    Replacement::draft_throws(0x14060b6b0, "bigfish_14060b6b0", fn_14060b6b0 as *const (), Throws::Discard), // sub_14060b6b0
    Replacement::draft(0x14060b800, "bigfish_14060b800", fn_14060b800 as *const ()), // sub_14060b800
    Replacement::draft(0x14060bb10, "bigfish_14060bb10", tpl_60530::<0x1426467e8, 0x143c98fc0, 0x142489ee0, 0x143ca7dbc, 0x14060bb39> as *const ()), // sub_14060bb10
    Replacement::draft(0x14060bcd0, "bigfish_14060bcd0", fn_14060bcd0 as *const ()), // sub_14060bcd0
    Replacement::draft(0x14060bea0, "bigfish_14060bea0", tpl_1094::<0x143c9d8d0, 0x14060ce40, 0x14020e8f0> as *const ()), // sub_14060bea0
    Replacement::draft(0x14060c0a0, "bigfish_14060c0a0", tpl_1436::<0x58, 0x60, 0x158, 0x143c9d8d0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14060c0a0
    Replacement::draft(0x14060c180, "bigfish_14060c180", tpl_1094::<0x143c9dc74, 0x14060cfc0, 0x14020e8f0> as *const ()), // sub_14060c180
    Replacement::draft_throws(0x14060c380, "bigfish_14060c380", tpl_1438::<0x143cbc7d8, 0x143cbca68, 0x1411e9f50, 0x1406100b0, 0x143cbc6e8, 0x1420af120, 0x1420a28e0, 0x14060f320, 0x14060e940, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fe6f6, 0x142646960, 0x143cbca88, 0x143cbca90, 0x143cbca92, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaimelee::stream_in::system>
    Replacement::draft_throws(0x14060c670, "bigfish_14060c670", tpl_1467::<0x143cbc7cc, 0x143cbc680, 0x1411e9f50, 0x14060ffc0, 0x143cbcab8, 0x1420af120, 0x1420a28e0, 0x14060f4d0, 0x14060ea30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1431fe946, 0x142646960, 0x143cbc6a0, 0x143cbc6a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaimelee::stream_out::system>
    Replacement::draft(0x14060c950, "unknown_14060c950", tpl_1439::<0x8, 0x14060c180> as *const ()), // sub_14060c950
    Replacement::draft(0x14060c980, "unknown_14060c980", fn_14060c980 as *const ()), // sub_14060c980
    Replacement::draft(0x14060cb70, "std_14060cb70", tpl_1::<0x14388e000> as *const ()), // std::_Func_impl_no_alloc<void (*)(ecs::Entity<ecs::access::Definition<r::TypeList<ecs::access::DefinitionComponentEntry<
    Replacement::draft(0x14060cb80, "std_14060cb80", fn_14060cb80 as *const ()), // std::_Func_impl_no_alloc<void (*)(ecs::Entity<ecs::access::Definition<r::TypeList<ecs::access::DefinitionComponentEntry<
    Replacement::draft(0x14060cbd0, "std_14060cbd0", tpl_550::<0x143200820, 0x8> as *const ()), // std::_Func_impl_no_alloc<void (*)(ecs::Entity<ecs::access::Definition<r::TypeList<ecs::access::DefinitionComponentEntry<
    Replacement::draft(0x14060cbf0, "std_14060cbf0", tpl_6018::<0x10, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // std::_Ref_count_obj_alloc3<rend::DistanceFieldData,r::Allocator<rend::DistanceFieldData,0>>::std::_Ref_count_obj_alloc3<
    Replacement::draft(0x14060cc90, "std_14060cc90", tpl_901::<0x14060f270> as *const ()), // std::_Ref_count_obj_alloc3<bigfish::global::AIMeleeEventRequestStorage::AIMeleeEventRequest,r::Allocator<bigfish::global
    Replacement::draft(0x14060cca0, "std_14060cca0", tpl_6019::<0x143200890, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<bigfish::global::AIMeleeEventRequestStorage::AIMeleeEventRequest,r::Allocator<bigfish::global
    Replacement::draft(0x14060ccd0, "unknown_14060ccd0", fn_14060ccd0 as *const ()), // sub_14060ccd0
    Replacement::draft(0x14060cdc0, "unknown_14060cdc0", fn_14060cdc0 as *const ()), // sub_14060cdc0
    Replacement::draft(0x14060ce40, "unknown_14060ce40", fn_14060ce40 as *const ()), // sub_14060ce40
    Replacement::draft(0x14060cfc0, "unknown_14060cfc0", fn_14060cfc0 as *const ()), // sub_14060cfc0
    Replacement::draft_throws(0x14060d1a0, "unknown_14060d1a0", fn_14060d1a0 as *const (), Throws::Discard), // sub_14060d1a0
    Replacement::draft_throws(0x14060d520, "unknown_14060d520", tpl_3250::<0x88, 0x10088, 0x14060d610, 0x14060e120, 0x14060d710> as *const (), Throws::Discard), // sub_14060d520
    Replacement::draft(0x14060d610, "unknown_14060d610", tpl_3359::<0x143c8fd28, 0x20, 0x143cbc580, 0x8, 0x143c8f63c, 0x143cbc540, 0x143cbc6f0, 0x143c94b34, 0x143c8d910, 0x143c8d08c, 0x10, 0x18> as *const ()), // sub_14060d610
    Replacement::draft_throws(0x14060d710, "unknown_14060d710", fn_14060d710 as *const (), Throws::Discard), // sub_14060d710
    Replacement::draft_throws(0x14060d8c0, "unknown_14060d8c0", fn_14060d8c0 as *const (), Throws::Discard), // sub_14060d8c0
    Replacement::draft_throws(0x14060dbb0, "unknown_14060dbb0", tpl_4333::<0x14060f5b0, 0x143cbc910, 0x1420af120, 0x143c8f63c, 0x140215610, 0x143cbc6f0, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_14060dbb0
    Replacement::draft(0x14060dd30, "unknown_14060dd30", fn_14060dd30 as *const ()), // sub_14060dd30
    Replacement::draft(0x14060de60, "unknown_14060de60", fn_14060de60 as *const ()), // sub_14060de60
    Replacement::draft(0x14060df60, "unknown_14060df60", fn_14060df60 as *const ()), // sub_14060df60
    Replacement::draft(0x14060e120, "unknown_14060e120", tpl_3151::<0x18, 0x10, 0x8, 0x14060e200, 0x14036f9a0, 0x20, 0x40, 0x60, 0x80> as *const ()), // sub_14060e120
    Replacement::draft(0x14060e510, "unknown_14060e510", fn_14060e510 as *const ()), // sub_14060e510
    Replacement::draft(0x14060e5f0, "unknown_14060e5f0", fn_14060e5f0 as *const ()), // sub_14060e5f0
    Replacement::draft(0x14060e730, "unknown_14060e730", tpl_61197::<0x143c8d160, 0x142489ee0, 0x143c8e840, 0x143c8f0d0, 0x143c8fa04, 0x1426467e8, 0x14060e8c3> as *const ()), // sub_14060e730
    Replacement::draft(0x14060e8d0, "unknown_14060e8d0", fn_14060e8d0 as *const ()), // sub_14060e8d0
    Replacement::draft_throws(0x14060e940, "unknown_14060e940", tpl_6033::<0x88, 0x10088, 0x142407c80, 0x14060f120, 0x580f0, 0x141b326b0> as *const (), Throws::Discard), // sub_14060e940
    Replacement::draft(0x14060ea30, "unknown_14060ea30", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14060c0a0> as *const ()), // sub_14060ea30
    Replacement::draft_throws(0x14060ead0, "unknown_14060ead0", fn_14060ead0 as *const (), Throws::Discard), // sub_14060ead0
    Replacement::draft(0x14060eda0, "unknown_14060eda0", fn_14060eda0 as *const ()), // sub_14060eda0
    Replacement::draft(0x14060eef0, "unknown_14060eef0", fn_14060eef0 as *const ()), // sub_14060eef0
    Replacement::draft(0x14060f220, "unknown_14060f220", tpl_2746::<0x8, 0xa0, 0x14240ab10> as *const ()), // sub_14060f220
    Replacement::draft(0x14060f270, "unknown_14060f270", tpl_2030::<0x14060acb0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14060f270
    Replacement::draft_throws(0x14060f320, "unknown_14060f320", tpl_6035::<0x142407c80, 0x143c9dc74, 0x142489ee0, 0x143ca7874, 0x141b326b0> as *const (), Throws::Discard), // sub_14060f320
    Replacement::draft(0x14060f4d0, "unknown_14060f4d0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14060c0a0> as *const ()), // sub_14060f4d0
    Replacement::draft(0x14060fc00, "unknown_14060fc00", fn_14060fc00 as *const ()), // sub_14060fc00
    Replacement::draft(0x14060ff00, "unknown_14060ff00", fn_14060ff00 as *const ()), // sub_14060ff00
    Replacement::draft(0x140610310, "unknown_140610310", fn_140610310 as *const ()), // sub_140610310
    Replacement::draft(0x1406103f0, "unknown_1406103f0", fn_1406103f0 as *const ()), // sub_1406103f0
    Replacement::draft_throws(0x140610950, "anonymous_namespace_140610950", tpl_5993::<0x143cbccac, 0x143cbcb58, 0x1411e9f50, 0x1406127f0, 0x14384c564, 0x1420af120, 0x140611e50, 0x1420a28e0, 0x140612160, 0x140611a40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143201756, 0x142646960, 0x143cbcb78, 0x143cbcb80, 0x143cbcb82, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::updateAbilityAILeap>
    Replacement::draft(0x140610c50, "anonymous_namespace_140610c50", tpl_3072::<0x14384cf68, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140610c50
    Replacement::draft(0x140610fa0, "anonymous_namespace_140610fa0", tpl_1436::<0x58, 0x60, 0x158, 0x14384cf68, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140610fa0
    Replacement::draft(0x140611080, "anonymous_namespace_140611080", tpl_1094::<0x143cbcbe0, 0x1406118a0, 0x14020e8f0> as *const ()), // sub_140611080
    Replacement::draft_throws(0x1406112b0, "anonymous_namespace_1406112b0", tpl_6039::<0x143cbcbb0, 0x143cbcb88, 0x1411e9f50, 0x140612b40, 0x14384cf6c, 0x1420af120, 0x140611e50, 0x1420a28e0, 0x1406125c0, 0x140611fc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432013a6, 0x142646960, 0x143cbcba8, 0x143cbcbac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::streamInEntity>
    Replacement::draft_throws(0x1406115b0, "anonymous_namespace_1406115b0", tpl_4129::<0x143cbcb1c, 0x143cbcbb8, 0x1411e9f50, 0x140612a50, 0x14384dc70, 0x1420af120, 0x1420a28e0, 0x140612710, 0x1406120c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143201206, 0x142646960, 0x143cbcbd8, 0x143cbcbdc, 0x143cbcbde, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::streamOutEntity>
    Replacement::draft(0x1406118a0, "unknown_1406118a0", fn_1406118a0 as *const ()), // sub_1406118a0
    Replacement::draft_throws(0x140611a40, "unknown_140611a40", tpl_6041::<0x88, 0x10088, 0x141b396b0, 0x140611af0> as *const (), Throws::Discard), // sub_140611a40
    Replacement::draft_throws(0x140611af0, "unknown_140611af0", tpl_4136::<0x18, 0x20, 0x40, 0x60, 0x68, 0x8, 0x141b39ae0> as *const (), Throws::Discard), // sub_140611af0
    Replacement::draft_throws(0x140611e50, "unknown_140611e50", fn_140611e50 as *const (), Throws::Discard), // sub_140611e50
    Replacement::draft(0x140611fc0, "unknown_140611fc0", tpl_63093::<0x143cbcbe0, 0x142489ee0, 0x141b39a00> as *const ()), // sub_140611fc0
    Replacement::draft(0x1406120c0, "unknown_1406120c0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140610fa0> as *const ()), // sub_1406120c0
    Replacement::draft_throws(0x140612160, "unknown_140612160", tpl_65181::<0x140611c20, 0x141b39ae0> as *const (), Throws::Discard), // sub_140612160
    Replacement::draft(0x1406125c0, "unknown_1406125c0", tpl_63099::<0x143cbcbe0, 0x142489ee0, 0x141b39a00> as *const ()), // sub_1406125c0
    Replacement::draft(0x140612710, "unknown_140612710", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140610fa0> as *const ()), // sub_140612710
    Replacement::draft(0x1406127f0, "unknown_1406127f0", fn_1406127f0 as *const ()), // sub_1406127f0
    Replacement::draft(0x140612e00, "content_140612e00", fn_140612e00 as *const ()), // content::AbilityAIGrappleComponent::content::AbilityAIGrappleComponent::vfunc_20
    Replacement::draft(0x140612e90, "unknown_140612e90", tpl_6045::<0x30, 0x143c954e4, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140612f0e> as *const ()), // sub_140612e90
    Replacement::draft(0x140613440, "unknown_140613440", fn_140613440 as *const ()), // sub_140613440
    Replacement::draft(0x140613690, "unknown_140613690", tpl_64850::<0x1426467e8, 0x140618bb0, 0x14036f9a0, 0x140613748> as *const ()), // sub_140613690
    Replacement::draft_throws(0x1406138c0, "bigfish_1406138c0", tpl_5091::<0x143cbd50c, 0x143cbced8, 0x140615700, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143206d26, 0x142646960, 0x143cbcef8, 0x143cbcf08, 0x143cbcf0c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::target_validation::system>
    Replacement::draft_throws(0x140613ad0, "bigfish_140613ad0", tpl_3456::<0x143cbcdf0, 0x143cbd828, 0x1411e9f50, 0x140617170, 0x1406193e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143206936, 0x142646960, 0x143cbd848, 0x143cbd850, 0x143cbd854, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::activation::system>
    Replacement::draft_throws(0x140613d20, "bigfish_140613d20", tpl_1584::<0x143cbd990, 0x143cbd438, 0x1411e9f50, 0x1406172e0, 0x140619460, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432059c6, 0x142646960, 0x143cbd458, 0x143cbd460, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::update::system>
    Replacement::draft_throws(0x140613f70, "bigfish_140613f70", tpl_6046::<0x143cbd7ac, 0x143cbd520, 0x1411e9f50, 0x1406177e0, 0x140619660, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143204e36, 0x142646960, 0x143cbd540, 0x143cbd548, 0x143cbd54c, 0x143cbd54e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::escape_item::system>
    Replacement::draft_throws(0x1406141d0, "bigfish_1406141d0", fn_1406141d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::update_detached_escape_item::system>
    Replacement::draft_throws(0x140614550, "bigfish_140614550", tpl_3368::<0x143cbd518, 0x143cbd730, 0x1411e9f50, 0x140617c20, 0x140619a50, 0x140617de0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143207386, 0x142646960, 0x143cbd750, 0x143cbd760, 0x143cbd768, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::update_flare_on_death::system>
    Replacement::draft(0x1406147a0, "bigfish_1406147a0", fn_1406147a0 as *const ()), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::grappleRemoved>
    Replacement::draft(0x1406149b0, "bigfish_1406149b0", tpl_1096::<0x143cbd6c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1406149b0
    Replacement::draft(0x140614ce0, "bigfish_140614ce0", tpl_1094::<0x143cbd004, 0x140616c00, 0x14020e8f0> as *const ()), // sub_140614ce0
    Replacement::draft(0x140614ee0, "bigfish_140614ee0", tpl_1094::<0x143cbd464, 0x140616e90, 0x14020e8f0> as *const ()), // sub_140614ee0
    Replacement::draft(0x1406150e0, "bigfish_1406150e0", tpl_3072::<0x143cbd4c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1406150e0
    Replacement::draft(0x140615430, "bigfish_140615430", tpl_1466::<0x143c9de20, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140615430
    Replacement::draft_throws(0x140615700, "bigfish_140615700", tpl_3083::<0x1411e9f50, 0x14061bc10, 0x143cbd7a8, 0x1420af120, 0x14061ba50, 0x143cbd514, 0x140215750, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x1406191b0, 0x140617010> as *const (), Throws::Discard), // sub_140615700
    Replacement::draft_throws(0x140615930, "bigfish_140615930", tpl_3592::<0x143cbd7bc, 0x143cbcd88, 0x1411e9f50, 0x14061c520, 0x143cbcff8, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x14061a4e0, 0x140619c90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432080a6, 0x142646960, 0x143cbcda8, 0x143cbcdb0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::streamInEnemy>
    Replacement::draft_throws(0x140615c30, "bigfish_140615c30", tpl_3677::<0x143cbd98c, 0x143cbd010, 0x1411e9f50, 0x14061c430, 0x143cbd7b4, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x14061a5d0, 0x140619d20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143207ed6, 0x142646960, 0x143cbd030, 0x143cbd038, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::streamOutEnemy>
    Replacement::draft_throws(0x140615f30, "bigfish_140615f30", tpl_3677::<0x143cbd42c, 0x143cbd3f0, 0x1411e9f50, 0x14061c330, 0x143cbd860, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x14061a6c0, 0x140619db0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143207ca6, 0x142646960, 0x143cbd410, 0x143cbd418, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::streamInTarget>
    Replacement::draft_throws(0x140616230, "bigfish_140616230", tpl_5993::<0x143cbd3e8, 0x143cbdad0, 0x1411e9f50, 0x14061c240, 0x143cbd09c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x14061a7b0, 0x140619e40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143208476, 0x142646960, 0x143cbdaf0, 0x143cbdaf8, 0x143cbdafa, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::abilityaigrapple::streamOutTarget>
    Replacement::draft(0x140616530, "unknown_140616530", tpl_5024::<0x1411e9f50, 0x14061a1f0, 0x143cbd4c8, 0x1401fc320, 0x143c8ceec, 0x140215610, 0x143c8d08c, 0x1420a28e0, 0x140619ed0, 0x1401de410> as *const ()), // sub_140616530
    Replacement::draft(0x140616730, "unknown_140616730", tpl_63880::<0x1426467e8, 0x140618480, 0x140616759> as *const ()), // sub_140616730
    Replacement::draft(0x1406168c0, "unknown_1406168c0", fn_1406168c0 as *const ()), // sub_1406168c0
    Replacement::draft(0x140616a70, "unknown_140616a70", tpl_63880::<0x1426467e8, 0x140618ec0, 0x140616a99> as *const ()), // sub_140616a70
    Replacement::draft(0x140616c00, "unknown_140616c00", fn_140616c00 as *const ()), // sub_140616c00
    Replacement::draft(0x140616e90, "unknown_140616e90", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143201f88> as *const ()), // sub_140616e90
    Replacement::draft_throws(0x140617010, "unknown_140617010", tpl_62463::<0x143cbd514, 0x143c8fd28, 0x140618380, 0x141b40bb0> as *const (), Throws::Discard), // sub_140617010
    Replacement::draft_throws(0x140617170, "unknown_140617170", tpl_4746::<0x14061b890, 0x143cbd7b8, 0x1420af120, 0x140215750, 0x143c8d068, 0x140215610, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_140617170
    Replacement::draft_throws(0x1406172e0, "unknown_1406172e0", fn_1406172e0 as *const (), Throws::Discard), // sub_1406172e0
    Replacement::draft_throws(0x1406177e0, "unknown_1406177e0", fn_1406177e0 as *const (), Throws::Discard), // sub_1406177e0
    Replacement::draft(0x140617ae0, "unknown_140617ae0", fn_140617ae0 as *const ()), // sub_140617ae0
    Replacement::draft_throws(0x140617c20, "unknown_140617c20", tpl_5581::<0x14061ab00, 0x143cbda58, 0x1420af120, 0x14061a940, 0x143cbd000, 0x140215750, 0x14036fad0, 0x14036e720, 0x143c8d08c, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_140617c20
    Replacement::draft(0x140617de0, "unknown_140617de0", fn_140617de0 as *const ()), // sub_140617de0
    Replacement::draft(0x140617ea0, "unknown_140617ea0", tpl_5584::<0x20, 0x143cbd000, 0x8, 0x143c94b44, 0x143c94b30, 0x143c8d08c, 0x580e0, 0x142407c80, 0x580f0, 0x10, 0x28, 0x38> as *const ()), // sub_140617ea0
    Replacement::draft(0x140617fc0, "unknown_140617fc0", fn_140617fc0 as *const ()), // sub_140617fc0
    Replacement::draft(0x1406180f0, "content_1406180f0", fn_1406180f0 as *const ()), // content::AbilityAIGrappleComponent::content::AbilityAIGrappleComponent::vfunc_0
    Replacement::draft(0x1406187f0, "unknown_1406187f0", tpl_65008::<0x143c8e65c, 0x142489ee0, 0x143c8d120, 0x143c8e840, 0x143c8d160, 0x143c8ec20, 0x143cbd004, 0x143ca4244, 0x143c9a4e8, 0x143c8e83c, 0x143c8f4d0, 0x143ca7874, 0x143c919a8, 0x143c8de68, 0x143cbd4c8, 0x143c94194, 0x143cb0f1c, 0x143ca4318> as *const ()), // sub_1406187f0
    Replacement::draft(0x140619070, "unknown_140619070", tpl_1212::<0x8, 0x14061bd10> as *const ()), // sub_140619070
    Replacement::draft(0x1406190a0, "unknown_1406190a0", tpl_6055::<0x143cbd41c, 0x1405400d0, 0x20, 0x140619120, 0x143cbd08c, 0x1402215b0> as *const ()), // sub_1406190a0
    Replacement::draft(0x140619120, "unknown_140619120", tpl_5222::<0x38, 0x20, 0x8, 0x14061bef0> as *const ()), // sub_140619120
    Replacement::draft(0x140619170, "unknown_140619170", tpl_1124::<0xc8> as *const ()), // sub_140619170
    Replacement::draft_throws(0x1406191b0, "unknown_1406191b0", tpl_62466::<0x143cbd514, 0x143c8fd28, 0x143c8d160, 0x142489ee0, 0x143c8d120, 0x141b40bb0> as *const (), Throws::Discard), // sub_1406191b0
    Replacement::draft_throws(0x1406193e0, "unknown_1406193e0", tpl_5944::<0x143cbd7b8, 0x20, 0x143c8d068, 0x8, 0x580e0, 0x143c8d08c, 0x141b40660> as *const (), Throws::Discard), // sub_1406193e0
    Replacement::draft_throws(0x140619460, "unknown_140619460", fn_140619460 as *const (), Throws::Caught), // sub_140619460
    Replacement::draft(0x140619660, "unknown_140619660", fn_140619660 as *const ()), // sub_140619660
    Replacement::draft(0x1406197f0, "unknown_1406197f0", fn_1406197f0 as *const ()), // sub_1406197f0
    Replacement::draft(0x140619a50, "unknown_140619a50", fn_140619a50 as *const ()), // sub_140619a50
    Replacement::draft(0x140619c90, "unknown_140619c90", tpl_1898::<0x88, 0x10088, 0x141b3c950> as *const ()), // sub_140619c90
    Replacement::draft(0x140619d20, "unknown_140619d20", tpl_1898::<0x88, 0x10088, 0x141b3c790> as *const ()), // sub_140619d20
    Replacement::draft(0x140619db0, "unknown_140619db0", tpl_1898::<0x88, 0x10088, 0x141b3c8c0> as *const ()), // sub_140619db0
    Replacement::draft(0x140619e40, "unknown_140619e40", tpl_1898::<0x88, 0x10088, 0x141b3c530> as *const ()), // sub_140619e40
    Replacement::draft(0x140619ed0, "unknown_140619ed0", fn_140619ed0 as *const ()), // sub_140619ed0
    Replacement::draft(0x14061a1f0, "unknown_14061a1f0", tpl_3188::<0x14061a8a0, 0x143201f40, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14061a1f0
    Replacement::draft(0x14061a270, "unknown_14061a270", tpl_4348::<0x18, 0x20, 0x140489dd0> as *const ()), // sub_14061a270
    Replacement::draft(0x14061d0c0, "content_14061d0c0", fn_14061d0c0 as *const ()), // content::AbilityAIGrabAndThrowComponent::content::AbilityAIGrabAndThrowComponent::vfunc_20
    Replacement::draft_throws(0x14061d840, "bigfish_14061d840", tpl_4863::<0x143cbded0, 0x143cbdbc0, 0x1411e9f50, 0x140621060, 0x143cbe110, 0x1420af120, 0x140620ee0, 0x143cbe088, 0x140215750, 0x1420a28e0, 0x14061fb80, 0x14061f100, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432095e6, 0x142646960, 0x143cbdbe0, 0x143cbdbf0, 0x143cbdbf4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_grab_and_throw::targeting::system>
    Replacement::draft_throws(0x14061dba0, "bigfish_14061dba0", tpl_1770::<0x143cbe08c, 0x143cbde00, 0x14061e510, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143208ed6, 0x142646960, 0x143cbde20, 0x143cbde30, 0x143cbde34, 0x143cbde36, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_grab_and_throw::activation::system>
    Replacement::draft_throws(0x14061ddc0, "bigfish_14061ddc0", tpl_5091::<0x143cbddf8, 0x143cbe198, 0x14061e740, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143208826, 0x142646960, 0x143cbe1b8, 0x143cbe1c8, 0x143cbe1cc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_grab_and_throw::throwing::system>
    Replacement::draft(0x14061dfd0, "bigfish_14061dfd0", tpl_1098::<0x143cbe098, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14061dfd0
    Replacement::draft(0x14061e310, "bigfish_14061e310", tpl_1094::<0x143cbdc80, 0x14061ef60, 0x14020e8f0> as *const ()), // sub_14061e310
    Replacement::draft_throws(0x14061e510, "bigfish_14061e510", tpl_3083::<0x1411e9f50, 0x140620d60, 0x143cbdd1c, 0x1420af120, 0x140620b20, 0x143cbe150, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14061fd50, 0x14061f2c0> as *const (), Throws::Discard), // sub_14061e510
    Replacement::draft_throws(0x14061e740, "bigfish_14061e740", tpl_3083::<0x1411e9f50, 0x1406209a0, 0x143cbdc84, 0x1420af120, 0x140620790, 0x143cbdf60, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14061ff70, 0x14061f520> as *const (), Throws::Discard), // sub_14061e740
    Replacement::draft_throws(0x14061e970, "bigfish_14061e970", tpl_4886::<0x143cbe078, 0x143cbe118, 0x1411e9f50, 0x140621290, 0x143cbe1d8, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1406204f0, 0x140620190, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143209d86, 0x142646960, 0x143cbe138, 0x143cbe148, 0x143cbe14c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_grab_and_throw::stream_in::system>
    Replacement::draft_throws(0x14061ec50, "bigfish_14061ec50", tpl_6068::<0x143cbe080, 0x143cbe158, 0x1411e9f50, 0x1406211a0, 0x143cbded4, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1406206a0, 0x140620270, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143209b96, 0x142646960, 0x143cbe178, 0x143cbe188, 0x143cbe18c, 0x143cbe18e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bigfish::ability_ai_grab_and_throw::stream_out::system>
    Replacement::draft(0x14061ef60, "unknown_14061ef60", fn_14061ef60 as *const ()), // sub_14061ef60
    Replacement::draft_throws(0x14061f100, "unknown_14061f100", tpl_3692::<0x88, 0x10088, 0x143cbe088, 0x20, 0x8, 0x14061fa20, 0x14061f1e0> as *const (), Throws::Discard), // sub_14061f100
    Replacement::draft_throws(0x14061f1e0, "unknown_14061f1e0", tpl_3693::<0x18, 0x20, 0x8, 0x141b43520> as *const (), Throws::Discard), // sub_14061f1e0
    Replacement::draft_throws(0x14061f2c0, "unknown_14061f2c0", tpl_6070::<0x143cbe150, 0x143c8ceec, 0x14061f8d0, 0x14061f410> as *const (), Throws::Discard), // sub_14061f2c0
    Replacement::draft_throws(0x14061f410, "unknown_14061f410", tpl_5999::<0x18, 0x20, 0x40, 0x8, 0x141b42c20> as *const (), Throws::Discard), // sub_14061f410
    Replacement::draft_throws(0x14061f520, "unknown_14061f520", tpl_6070::<0x143cbdf60, 0x143c8ceec, 0x14061f780, 0x14061f670> as *const (), Throws::Discard), // sub_14061f520
    Replacement::draft_throws(0x14061f670, "unknown_14061f670", tpl_5999::<0x18, 0x20, 0x40, 0x8, 0x141b426b0> as *const (), Throws::Discard), // sub_14061f670
    Replacement::draft_throws(0x14061fb80, "unknown_14061fb80", tpl_63006::<0x143cbe088, 0x14061fa20, 0x141b43520> as *const (), Throws::Discard), // sub_14061fb80
    Replacement::draft_throws(0x14061fd50, "unknown_14061fd50", tpl_65202::<0x143cbe150, 0x143c8ceec, 0x14061f8d0, 0x141b42c20> as *const (), Throws::Discard), // sub_14061fd50
    Replacement::draft_throws(0x14061ff70, "unknown_14061ff70", tpl_65202::<0x143cbdf60, 0x143c8ceec, 0x14061f780, 0x141b426b0> as *const (), Throws::Discard), // sub_14061ff70
    Replacement::draft_throws(0x140620190, "unknown_140620190", tpl_64494::<0x88, 0x10088, 0x140248e30, 0x141b425c0> as *const (), Throws::Discard), // sub_140620190
    Replacement::draft(0x140620270, "unknown_140620270", tpl_1898::<0x88, 0x10088, 0x141b42490> as *const ()), // sub_140620270
    Replacement::draft_throws(0x1406204f0, "unknown_1406204f0", tpl_64499::<0x143c8d160, 0x142489ee0, 0x143c8f4d0, 0x141b425c0> as *const (), Throws::Discard), // sub_1406204f0
    Replacement::draft(0x1406206a0, "unknown_1406206a0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b42490> as *const ()), // sub_1406206a0
    Replacement::draft(0x140620790, "unknown_140620790", tpl_6071::<0x143cbdd20, 0x143cbdb50, 0x143c8f4d4, 0x143c8e840, 0x143c8d160, 0x143c8d120, 0x143cbdc80, 0x143c9a4e8, 0x143ca7874, 0x143c919a8, 0x143cbe098, 0x143cbdb70, 0x143cbdb90, 0x143cbdbb0, 0x141834a0c, 0x141834a78> as *const ()), // sub_140620790
    Replacement::draft(0x140620b20, "unknown_140620b20", fn_140620b20 as *const ()), // sub_140620b20
    Replacement::draft_throws(0x140621890, "anonymous_namespace_140621890", fn_140621890 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::update>
    Replacement::draft(0x140621c20, "anonymous_namespace_140621c20", tpl_1094::<0x14384dee8, 0x140622650, 0x14020e8f0> as *const ()), // sub_140621c20
    Replacement::draft(0x140621e20, "anonymous_namespace_140621e20", tpl_1094::<0x143cbe1fc, 0x1406227d0, 0x14020e8f0> as *const ()), // sub_140621e20
    Replacement::draft_throws(0x140622050, "anonymous_namespace_140622050", tpl_6039::<0x143cbe368, 0x143cbe208, 0x1411e9f50, 0x140623910, 0x14384deec, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1406233a0, 0x140623010, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14320b986, 0x142646960, 0x143cbe228, 0x143cbe22c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::streamInEntity>
    Replacement::draft_throws(0x140622350, "anonymous_namespace_140622350", tpl_5275::<0x143cbe1e0, 0x143cbe238, 0x1411e9f50, 0x140623820, 0x14384dc74, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x1406234f0, 0x140623110, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14320c6b6, 0x142646960, 0x143cbe258, 0x143cbe25c, 0x143cbe25e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::streamOutEntity>
    Replacement::draft(0x140622650, "unknown_140622650", fn_140622650 as *const ()), // sub_140622650
    Replacement::draft(0x1406227d0, "unknown_1406227d0", tpl_4134::<0x1401e6d50, 0x140285d50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14320cbb8> as *const ()), // sub_1406227d0
    Replacement::draft_throws(0x140622970, "unknown_140622970", tpl_6041::<0x88, 0x10088, 0x141b452e0, 0x140622a20> as *const (), Throws::Discard), // sub_140622970
    Replacement::draft_throws(0x140622a20, "unknown_140622a20", tpl_4136::<0x18, 0x20, 0x40, 0x60, 0x68, 0x8, 0x141b45790> as *const (), Throws::Discard), // sub_140622a20
    Replacement::draft(0x140622d80, "unknown_140622d80", fn_140622d80 as *const ()), // sub_140622d80
    Replacement::draft_throws(0x140622da0, "unknown_140622da0", tpl_65181::<0x140622b50, 0x141b45790> as *const (), Throws::Discard), // sub_140622da0
    Replacement::draft(0x140623010, "unknown_140623010", tpl_63093::<0x143cbe1fc, 0x142489ee0, 0x141b45670> as *const ()), // sub_140623010
    Replacement::draft(0x140623110, "unknown_140623110", tpl_1898::<0x88, 0x10088, 0x141b45510> as *const ()), // sub_140623110
    Replacement::draft(0x1406233a0, "unknown_1406233a0", tpl_63099::<0x143cbe1fc, 0x142489ee0, 0x141b45670> as *const ()), // sub_1406233a0
    Replacement::draft(0x1406234f0, "unknown_1406234f0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b45510> as *const ()), // sub_1406234f0
    Replacement::draft(0x1406235e0, "unknown_1406235e0", fn_1406235e0 as *const ()), // sub_1406235e0
    Replacement::draft(0x140623bd0, "content_140623bd0", tpl_6077::<0x14320dd40, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::WorkspotAreaComponent::content::WorkspotAreaComponent::vfunc_18
    Replacement::draft(0x140623cd0, "unknown_140623cd0", tpl_6078::<0x30, 0x143c9a624, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140623d56> as *const ()), // sub_140623cd0
    Replacement::draft_throws(0x1406241e0, "unknown_1406241e0", fn_1406241e0 as *const (), Throws::Discard), // sub_1406241e0
    Replacement::draft(0x1406242a0, "unknown_1406242a0", fn_1406242a0 as *const ()), // sub_1406242a0
    Replacement::draft(0x140624310, "unknown_140624310", fn_140624310 as *const ()), // sub_140624310
    Replacement::draft(0x1406243c0, "unknown_1406243c0", tpl_6082::<0x143c9a86c, 0x40, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_1406243c0
    Replacement::draft_throws(0x140624430, "bonsai_140624430", tpl_1583::<0x143cbe484, 0x143cbe488, 0x1411e9f50, 0x140624e50, 0x1406255a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14320e2a6, 0x142646960, 0x143cbe4a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspotmanager::update::system>
    Replacement::draft_throws(0x140624670, "bonsai_140624670", fn_140624670 as *const (), Throws::Discard), // sub_140624670
    Replacement::draft(0x1406246f0, "bonsai_1406246f0", fn_1406246f0 as *const ()), // sub_1406246f0
    Replacement::draft_throws(0x140624850, "bonsai_140624850", tpl_5092::<0x143cbe4b0, 0x143cbe4b8, 0x1411e9f50, 0x1406261d0, 0x143cbe7ec, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x140625700, 0x140625410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14320ddb6, 0x142646960, 0x143cbe4d8, 0x143cbe4e0, 0x143cbe4e2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot_search::stream_in::system>
    Replacement::draft_throws(0x140624b70, "bonsai_140624b70", tpl_1467::<0x143cbe4e4, 0x143cbe4e8, 0x1411e9f50, 0x1406260e0, 0x143cbe7e8, 0x1420af120, 0x1420a28e0, 0x140625820, 0x1406254e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14320dfd6, 0x142646960, 0x143cbe508, 0x143cbe510, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot_search::stream_out::system>
    Replacement::draft_throws(0x140624e50, "unknown_140624e50", fn_140624e50 as *const (), Throws::Discard), // sub_140624e50
    Replacement::draft(0x140625410, "unknown_140625410", tpl_6086::<0x142407c80, 0x143cada18, 0x580e0, 0x580f0, 0x141b47fa0> as *const ()), // sub_140625410
    Replacement::draft(0x1406254e0, "unknown_1406254e0", tpl_1126::<0x142407c80, 0x580f0, 0x141b47de0> as *const ()), // sub_1406254e0
    Replacement::draft_throws(0x1406255a0, "unknown_1406255a0", fn_1406255a0 as *const (), Throws::Discard), // sub_1406255a0
    Replacement::draft(0x140625700, "unknown_140625700", tpl_6088::<0x142407c80, 0x143cada18, 0x141b47fa0> as *const ()), // sub_140625700
    Replacement::draft(0x140625820, "unknown_140625820", tpl_1130::<0x142407c80, 0x141b47de0> as *const ()), // sub_140625820
    Replacement::draft(0x140625930, "unknown_140625930", fn_140625930 as *const ()), // sub_140625930
    Replacement::draft(0x140625ad0, "unknown_140625ad0", tpl_6090::<0x143cbe578, 0x143cbe580, 0x141834a78, 0x143c9a624, 0x143cbe480, 0x143ca7604, 0x143c8fa04, 0x143c8d160, 0x143c8d120, 0x143c8d694, 0x143cbe5a0, 0x143cbe5c0, 0x143cbe5d0, 0x141834a0c> as *const ()), // sub_140625ad0
    Replacement::draft(0x140625db0, "unknown_140625db0", tpl_1223::<0x143cbe620, 0x143cbe630, 0x141834a78, 0x143c8f4d0, 0x143c9a86c, 0x143c9a660, 0x143c8de68, 0x143c8fa04, 0x143c8d160, 0x143c8d120, 0x143cbe650, 0x143cbe670, 0x143cbe680, 0x141834a0c> as *const ()), // sub_140625db0
    Replacement::draft(0x140625f50, "unknown_140625f50", tpl_5962::<0x1403d0e60, 0x14054ce20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14025f9d0, 0x1401e66f0, 0x142442de0> as *const ()), // sub_140625f50
    Replacement::draft(0x140626510, "unknown_140626510", fn_140626510 as *const ()), // sub_140626510
    Replacement::draft(0x140626610, "content_140626610", fn_140626610 as *const ()), // content::WorkspotAreaComponent::content::WorkspotAreaComponent::vfunc_0
    Replacement::draft(0x1406267a0, "unknown_1406267a0", tpl_1663::<0x8, 0x1403d0e60, 0x1401e66f0> as *const ()), // sub_1406267a0
    Replacement::draft(0x1406268d0, "unknown_1406268d0", fn_1406268d0 as *const ()), // sub_1406268d0
    Replacement::draft(0x140626960, "unknown_140626960", tpl_61019::<0x143c8f4d0, 0x142489ee0, 0x143cbe8e4, 0x143c8d160, 0x1426467e8, 0x140626ac3> as *const ()), // sub_140626960
    Replacement::draft(0x140626ad0, "unknown_140626ad0", fn_140626ad0 as *const ()), // sub_140626ad0
    Replacement::draft(0x140626b80, "unknown_140626b80", tpl_1171::<0x58, 0x60, 0x143cbe838, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140626b80
    Replacement::draft_throws(0x140626c60, "bonsai_140626c60", tpl_5694::<0x143cbe8bc, 0x143cbe8c0, 0x1411e9f50, 0x140628be0, 0x140629c00, 0x140628e00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143210546, 0x142646960, 0x143cbe8e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot::update::system>
    Replacement::draft(0x140626ea0, "bonsai_140626ea0", tpl_1095::<0x143cbe838, 0x14020e8f0> as *const ()), // sub_140626ea0
    Replacement::draft(0x140627080, "bonsai_140627080", tpl_1094::<0x143c9a624, 0x1406284a0, 0x14020e8f0> as *const ()), // sub_140627080
    Replacement::draft(0x140627280, "bonsai_140627280", tpl_1094::<0x143cb1824, 0x140628680, 0x14020e8f0> as *const ()), // sub_140627280
    Replacement::draft(0x140627480, "bonsai_140627480", tpl_1094::<0x143ca7604, 0x140628800, 0x14020e8f0> as *const ()), // sub_140627480
    Replacement::draft(0x140627680, "bonsai_140627680", tpl_1094::<0x143cbe47c, 0x1406289d0, 0x14020e8f0> as *const ()), // sub_140627680
    Replacement::draft_throws(0x1406278e0, "bonsai_1406278e0", fn_1406278e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot::stream_in::system>
    Replacement::draft_throws(0x140627c20, "bonsai_140627c20", tpl_2988::<0x143cbe914, 0x143cbe918, 0x1411e9f50, 0x1406294a0, 0x14062a5d0, 0x140629700, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143210d86, 0x142646960, 0x143cbe938, 0x143cbe940, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot_area::stream_in::system>
    Replacement::draft_throws(0x140627e70, "bonsai_140627e70", tpl_4361::<0x143cbe944, 0x143cbe948, 0x1411e9f50, 0x14062b000, 0x143cbec34, 0x1420af120, 0x143cbe514, 0x140215610, 0x1420a28e0, 0x14062a820, 0x1406299b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432111e6, 0x142646960, 0x143cbe968, 0x143cbe96c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot::stream_out::system>
    Replacement::draft_throws(0x140628180, "bonsai_140628180", tpl_3003::<0x143cbe970, 0x143cbe978, 0x1411e9f50, 0x14062ae90, 0x143cbec30, 0x1420af120, 0x143cbe514, 0x140215610, 0x1420a28e0, 0x14062a9f0, 0x140629ac0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14320f546, 0x142646960, 0x143cbe998, 0x143cbe9a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::workspot_area::stream_out::system>
    Replacement::draft(0x1406284a0, "unknown_1406284a0", fn_1406284a0 as *const ()), // sub_1406284a0
    Replacement::draft(0x140628680, "unknown_140628680", fn_140628680 as *const ()), // sub_140628680
    Replacement::draft(0x140628800, "unknown_140628800", fn_140628800 as *const ()), // sub_140628800
    Replacement::draft(0x1406289d0, "unknown_1406289d0", fn_1406289d0 as *const ()), // sub_1406289d0
    Replacement::draft_throws(0x140628be0, "unknown_140628be0", tpl_1108::<0x14062ad50, 0x143cbea24, 0x1420af120, 0x14062ac40, 0x143cbea20, 0x140215750, 0x143c8ceec, 0x140215610, 0x143c8d910, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_140628be0
    Replacement::draft(0x140628e00, "unknown_140628e00", fn_140628e00 as *const ()), // sub_140628e00
    Replacement::draft(0x140628f80, "unknown_140628f80", fn_140628f80 as *const ()), // sub_140628f80
    Replacement::draft(0x1406290a0, "unknown_1406290a0", fn_1406290a0 as *const ()), // sub_1406290a0
    Replacement::draft(0x1406292a0, "unknown_1406292a0", tpl_3688::<0x50, 0x1401de830> as *const ()), // sub_1406292a0
    Replacement::draft(0x1406292e0, "unknown_1406292e0", tpl_3160::<0x40, 0x140550150> as *const ()), // sub_1406292e0
    Replacement::draft(0x140629310, "unknown_140629310", tpl_1124::<0x50> as *const ()), // sub_140629310
    Replacement::draft_throws(0x140629340, "unknown_140629340", fn_140629340 as *const (), Throws::Discard), // sub_140629340
    Replacement::draft_throws(0x1406294a0, "unknown_1406294a0", fn_1406294a0 as *const (), Throws::Discard), // sub_1406294a0
    Replacement::draft_throws(0x140629700, "unknown_140629700", tpl_4245::<0x88, 0x10088, 0x1406297c0, 0x14062a090, 0x1406298b0> as *const (), Throws::Discard), // sub_140629700
    Replacement::draft(0x1406297c0, "unknown_1406297c0", fn_1406297c0 as *const ()), // sub_1406297c0
    Replacement::draft_throws(0x1406298b0, "unknown_1406298b0", fn_1406298b0 as *const (), Throws::Discard), // sub_1406298b0
    Replacement::draft_throws(0x1406299b0, "unknown_1406299b0", fn_1406299b0 as *const (), Throws::Discard), // sub_1406299b0
    Replacement::draft_throws(0x140629ac0, "unknown_140629ac0", tpl_5251::<0x142407c80, 0x143cbe514, 0x140629e60, 0x141b4b4b0> as *const (), Throws::Discard), // sub_140629ac0
    Replacement::draft(0x140629c00, "unknown_140629c00", fn_140629c00 as *const ()), // sub_140629c00
    Replacement::draft(0x14062bb80, "bonsai_14062bb80", fn_14062bb80 as *const ()), // sub_14062bb80
    Replacement::draft(0x14062bbf0, "bonsai_14062bbf0", fn_14062bbf0 as *const ()), // sub_14062bbf0
    Replacement::draft_throws(0x14062be30, "bonsai_14062be30", fn_14062be30 as *const (), Throws::Caught), // sub_14062be30
    Replacement::draft_throws(0x14062bfc0, "bonsai_14062bfc0", tpl_1905::<0x14387c170, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14062bfc0
    Replacement::draft_throws(0x14062c070, "bonsai_14062c070", fn_14062c070 as *const (), Throws::Discard), // sub_14062c070
    Replacement::draft(0x14062c1e0, "bonsai_14062c1e0", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x1432117c8, 0x1f, 0x8, 0x1c> as *const ()), // sub_14062c1e0
    Replacement::draft(0x14062c270, "bonsai_14062c270", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432117c8, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_14062c270
    Replacement::draft_throws(0x14062c4c0, "bonsai_14062c4c0", fn_14062c4c0 as *const (), Throws::Caught), // sub_14062c4c0
    Replacement::draft_throws(0x14062c650, "bonsai_14062c650", tpl_1905::<0x14387c180, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14062c650
    Replacement::draft(0x14062c700, "bonsai_14062c700", fn_14062c700 as *const ()), // bonsai::BehaviorTreeServiceInstance::bonsai::BehaviorTreeServiceInstance::vfunc_0
    Replacement::draft_throws(0x14062c740, "bonsai_14062c740", fn_14062c740 as *const (), Throws::Caught), // sub_14062c740
    Replacement::draft_throws(0x14062cb50, "bonsai_14062cb50", fn_14062cb50 as *const (), Throws::Caught), // sub_14062cb50
    Replacement::draft_throws(0x14062cbe0, "bonsai_14062cbe0", tpl_1908::<0x14387c8e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_14062cbe0
    Replacement::draft(0x14062cc90, "bonsai_14062cc90", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143211a68, 0x15, 0x17, 0x8> as *const ()), // sub_14062cc90
    Replacement::draft(0x14062cd30, "bonsai_14062cd30", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143211a68, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14062cd30
    Replacement::draft_throws(0x14062cf80, "bonsai_14062cf80", fn_14062cf80 as *const (), Throws::Caught), // sub_14062cf80
    Replacement::draft_throws(0x14062d110, "bonsai_14062d110", tpl_1905::<0x14387c190, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14062d110
    Replacement::draft_throws(0x14062d1c0, "bonsai_14062d1c0", fn_14062d1c0 as *const (), Throws::Discard), // sub_14062d1c0
    Replacement::draft(0x14062d330, "bonsai_14062d330", tpl_6116::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x11, 0x143211d98, 0x15, 0x17, 0x38> as *const ()), // sub_14062d330
    Replacement::draft(0x14062d3d0, "bonsai_14062d3d0", tpl_6117::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143211d98, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14062d3d0
    Replacement::draft_throws(0x14062d630, "bonsai_14062d630", fn_14062d630 as *const (), Throws::Caught), // sub_14062d630
    Replacement::draft_throws(0x14062d7c0, "bonsai_14062d7c0", tpl_1905::<0x14387c1a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14062d7c0
    Replacement::draft_throws(0x14062d870, "bonsai_14062d870", fn_14062d870 as *const (), Throws::Discard), // sub_14062d870
    Replacement::draft_throws(0x14062d980, "bonsai_14062d980", fn_14062d980 as *const (), Throws::Discard), // sub_14062d980
    Replacement::draft(0x14062dae0, "unknown_14062dae0", tpl_6120::<0x70, 0x10, 0x18, 0xffffffffffffffa8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x28, 0x30, 0x38, 0x143212090> as *const ()), // sub_14062dae0
    Replacement::draft(0x14062dba0, "unknown_14062dba0", tpl_6121::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143212090, 0x22, 0x26, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14062dba0
    Replacement::draft_throws(0x14062de00, "unknown_14062de00", fn_14062de00 as *const (), Throws::Caught), // sub_14062de00
    Replacement::draft_throws(0x14062dfb0, "unknown_14062dfb0", tpl_5858::<0x14387c1b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1405e1c70> as *const (), Throws::Discard), // sub_14062dfb0
    Replacement::draft(0x14062e090, "bonsai_14062e090", fn_14062e090 as *const ()), // bonsai::StunnedProgram::bonsai::StunnedProgram::vfunc_0
    Replacement::draft(0x14062e110, "bonsai_14062e110", fn_14062e110 as *const ()), // sub_14062e110
    Replacement::draft(0x14062e1e0, "bonsai_14062e1e0", fn_14062e1e0 as *const ()), // sub_14062e1e0
    Replacement::draft_throws(0x14062e490, "bonsai_14062e490", fn_14062e490 as *const (), Throws::Caught), // sub_14062e490
    Replacement::draft_throws(0x14062e620, "bonsai_14062e620", tpl_1905::<0x14387c1c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14062e620
    Replacement::draft_throws(0x14062e6d0, "bonsai_14062e6d0", fn_14062e6d0 as *const (), Throws::Discard), // sub_14062e6d0
    Replacement::draft_throws(0x14062e8e0, "bonsai_14062e8e0", fn_14062e8e0 as *const (), Throws::Discard), // sub_14062e8e0
    Replacement::draft_throws(0x14062eaf0, "bonsai_14062eaf0", fn_14062eaf0 as *const (), Throws::Discard), // sub_14062eaf0
    Replacement::draft(0x14062ed00, "bonsai_14062ed00", fn_14062ed00 as *const ()), // bonsai::StuckDetectionServiceInstance::bonsai::StuckDetectionServiceInstance::vfunc_0
    Replacement::draft(0x14062ed40, "bonsai_14062ed40", tpl_65251::<0x8, 0xc, 0x14062ee30, 0x14183acc2> as *const ()), // sub_14062ed40
    Replacement::draft(0x14062edf0, "bonsai_14062edf0", fn_14062edf0 as *const ()), // sub_14062edf0
    Replacement::draft(0x14062ee30, "bonsai_14062ee30", fn_14062ee30 as *const ()), // sub_14062ee30
    Replacement::draft(0x14062ee90, "bonsai_14062ee90", fn_14062ee90 as *const ()), // sub_14062ee90
    Replacement::draft(0x14062f020, "bonsai_14062f020", fn_14062f020 as *const ()), // sub_14062f020
    Replacement::draft(0x14062f1c0, "bonsai_14062f1c0", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x1432127c8, 0x1f, 0x8, 0x1c> as *const ()), // sub_14062f1c0
    Replacement::draft(0x14062f250, "bonsai_14062f250", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432127c8, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_14062f250
    Replacement::draft_throws(0x14062f4a0, "bonsai_14062f4a0", fn_14062f4a0 as *const (), Throws::Caught), // sub_14062f4a0
    Replacement::draft_throws(0x14062f630, "bonsai_14062f630", tpl_1905::<0x14387c1d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14062f630
    Replacement::draft_throws(0x14062f6e0, "bonsai_14062f6e0", fn_14062f6e0 as *const (), Throws::Discard), // sub_14062f6e0
    Replacement::draft(0x14062f840, "bonsai_14062f840", fn_14062f840 as *const ()), // sub_14062f840
    Replacement::draft(0x14062f8f0, "bonsai_14062f8f0", fn_14062f8f0 as *const ()), // sub_14062f8f0
    Replacement::draft_throws(0x14062fb50, "bonsai_14062fb50", fn_14062fb50 as *const (), Throws::Caught), // sub_14062fb50
    Replacement::draft_throws(0x14062fce0, "bonsai_14062fce0", tpl_1905::<0x14387c1e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14062fce0
    Replacement::draft(0x14062fda0, "bonsai_14062fda0", fn_14062fda0 as *const ()), // bonsai::ShootProgram::bonsai::ShootProgram::vfunc_0
    Replacement::draft(0x14062fe20, "bonsai_14062fe20", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143212e70, 0x15, 0x17, 0x8> as *const ()), // sub_14062fe20
    Replacement::draft(0x14062fec0, "bonsai_14062fec0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143212e70, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14062fec0
    Replacement::draft_throws(0x140630110, "bonsai_140630110", fn_140630110 as *const (), Throws::Caught), // sub_140630110
    Replacement::draft_throws(0x1406302a0, "bonsai_1406302a0", tpl_1905::<0x14387c1f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406302a0
    Replacement::draft(0x140630470, "bonsai_140630470", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x1432136f0, 0x15, 0x17, 0x8> as *const ()), // sub_140630470
    Replacement::draft(0x140630510, "bonsai_140630510", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432136f0, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140630510
    Replacement::draft_throws(0x140630760, "bonsai_140630760", fn_140630760 as *const (), Throws::Caught), // sub_140630760
    Replacement::draft_throws(0x1406308f0, "bonsai_1406308f0", tpl_1905::<0x14387c200, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406308f0
    Replacement::draft(0x1406309a0, "bonsai_1406309a0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143213808, 0x15, 0x17, 0x8> as *const ()), // sub_1406309a0
    Replacement::draft(0x140630a40, "bonsai_140630a40", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143213808, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140630a40
    Replacement::draft_throws(0x140630b60, "bonsai_140630b60", fn_140630b60 as *const (), Throws::Caught), // sub_140630b60
    Replacement::draft_throws(0x140630bf0, "bonsai_140630bf0", tpl_5859::<0x14387c210, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c200, 0x14387c9e8> as *const (), Throws::Discard), // sub_140630bf0
    Replacement::draft(0x140630cd0, "bonsai_140630cd0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x1432134c0, 0x15, 0x17, 0x8> as *const ()), // sub_140630cd0
    Replacement::draft(0x140630d70, "bonsai_140630d70", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432134c0, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140630d70
    Replacement::draft_throws(0x140630e90, "bonsai_140630e90", fn_140630e90 as *const (), Throws::Caught), // sub_140630e90
    Replacement::draft_throws(0x140630f20, "bonsai_140630f20", tpl_5859::<0x14387c220, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c200, 0x14387c9e8> as *const (), Throws::Discard), // sub_140630f20
    Replacement::draft(0x140631000, "bonsai_140631000", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x1432135d8, 0x15, 0x17, 0x8> as *const ()), // sub_140631000
    Replacement::draft(0x1406310a0, "bonsai_1406310a0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432135d8, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_1406310a0
    Replacement::draft_throws(0x1406311c0, "bonsai_1406311c0", fn_1406311c0 as *const (), Throws::Caught), // sub_1406311c0
    Replacement::draft_throws(0x140631250, "bonsai_140631250", tpl_5859::<0x14387c230, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c200, 0x14387c9e8> as *const (), Throws::Discard), // sub_140631250
    Replacement::draft(0x140631330, "bonsai_140631330", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143213290, 0x15, 0x17, 0x8> as *const ()), // sub_140631330
    Replacement::draft(0x1406313d0, "bonsai_1406313d0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143213290, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_1406313d0
    Replacement::draft_throws(0x1406314f0, "bonsai_1406314f0", fn_1406314f0 as *const (), Throws::Caught), // sub_1406314f0
    Replacement::draft_throws(0x140631580, "bonsai_140631580", tpl_5859::<0x14387c240, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c200, 0x14387c9e8> as *const (), Throws::Discard), // sub_140631580
    Replacement::draft(0x140631660, "bonsai_140631660", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x1432133a8, 0x15, 0x17, 0x8> as *const ()), // sub_140631660
    Replacement::draft(0x140631700, "bonsai_140631700", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432133a8, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140631700
    Replacement::draft_throws(0x140631820, "bonsai_140631820", fn_140631820 as *const (), Throws::Caught), // sub_140631820
    Replacement::draft_throws(0x1406318b0, "bonsai_1406318b0", tpl_5859::<0x14387c250, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c200, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406318b0
    Replacement::draft_throws(0x140631990, "bonsai_140631990", fn_140631990 as *const (), Throws::Discard), // sub_140631990
    Replacement::draft_throws(0x140631ba0, "bonsai_140631ba0", fn_140631ba0 as *const (), Throws::Caught), // sub_140631ba0
    Replacement::draft(0x140632020, "bonsai_140632020", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x1432143c0, 0x15, 0x17, 0x8> as *const ()), // sub_140632020
    Replacement::draft(0x1406320c0, "bonsai_1406320c0", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432143c0, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_1406320c0
    Replacement::draft_throws(0x140632310, "bonsai_140632310", fn_140632310 as *const (), Throws::Caught), // sub_140632310
    Replacement::draft_throws(0x1406324a0, "bonsai_1406324a0", tpl_1905::<0x14387c260, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_1406324a0
    Replacement::draft(0x140632580, "bonsai_140632580", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x1432146b8, 0x15, 0x17, 0x8> as *const ()), // sub_140632580
    Replacement::draft(0x140632620, "bonsai_140632620", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432146b8, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140632620
    Replacement::draft(0x1406326d0, "unknown_1406326d0", tpl_6135::<0x14387c270, 0x14243b0b0, 0x14387c9e8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1406326d0
    Replacement::draft_throws(0x140632770, "unknown_140632770", tpl_6136::<0x14387c278, 0x14387c270, 0x1401e9fa0, 0x14387c9e4, 0x14387c9e8, 0x1401e9e70> as *const (), Throws::Discard), // sub_140632770
    Replacement::draft_throws(0x1406327f0, "unknown_1406327f0", tpl_6136::<0x14387c278, 0x14387c270, 0x1401e9b60, 0x14387c9e4, 0x14387c9e8, 0x1401e9a20> as *const (), Throws::Discard), // sub_1406327f0
    Replacement::draft_throws(0x140632870, "unknown_140632870", fn_140632870 as *const (), Throws::Caught), // sub_140632870
    Replacement::draft(0x140632900, "unknown_140632900", tpl_6137::<0x14387c278, 0x14387c270, 0x1401e9970, 0x14387c9e4, 0x14387c9e8, 0x1401e9860> as *const ()), // sub_140632900
    Replacement::draft_throws(0x140632980, "unknown_140632980", tpl_6136::<0x14387c278, 0x14387c270, 0x1401e9770, 0x14387c9e4, 0x14387c9e8, 0x1401e9590> as *const (), Throws::Discard), // sub_140632980
    Replacement::draft_throws(0x140632a00, "unknown_140632a00", tpl_1905::<0x14387c270, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140632a00
    Replacement::draft_throws(0x140632ab0, "std_140632ab0", fn_140632ab0 as *const (), Throws::Discard), // sub_140632ab0
    Replacement::draft(0x140632d80, "unknown_140632d80", tpl_4771::<0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140632d80
    Replacement::draft(0x140632db0, "unknown_140632db0", fn_140632db0 as *const ()), // sub_140632db0
    Replacement::draft_throws(0x140632e50, "unknown_140632e50", tpl_1480::<0x1427899d0, 0x1410d6ca0, 0x140633e30, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140632e50
    Replacement::draft_throws(0x1406330f0, "bonsai_1406330f0", tpl_4828::<0x143cbedcc, 0x143cbedd0, 0x1411e9f50, 0x140635180, 0x143cbeef0, 0x1420af120, 0x1406346d0, 0x1420a28e0, 0x1406349f0, 0x140634440, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143214dc6, 0x142646960, 0x143cbedf0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::quadruped::update::system>
    Replacement::draft(0x1406333e0, "bonsai_1406333e0", tpl_1094::<0x143cbedf4, 0x140634110, 0x14020e8f0> as *const ()), // sub_1406333e0
    Replacement::draft(0x1406335e0, "bonsai_1406335e0", tpl_1094::<0x143cab498, 0x1406342b0, 0x14020e8f0> as *const ()), // sub_1406335e0
    Replacement::draft_throws(0x140633810, "bonsai_140633810", tpl_6140::<0x143cbedf8, 0x143cbee00, 0x1411e9f50, 0x140635950, 0x143cbefac, 0x1420af120, 0x1406346d0, 0x143cada18, 0x140215610, 0x1420a28e0, 0x140634e10, 0x140634810, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432149f6, 0x142646960, 0x143cbee20, 0x143cbee24, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::quadruped::stream_in::system>
    Replacement::draft_throws(0x140633b30, "bonsai_140633b30", tpl_6039::<0x143cbee28, 0x143cbee30, 0x1411e9f50, 0x140635860, 0x143cbefa8, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140634ff0, 0x140634960, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143215296, 0x142646960, 0x143cbee50, 0x143cbee54, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::quadruped::stream_out::system>
    Replacement::draft_throws(0x140633e30, "bonsai_140633e30", tpl_6141::<0x143cbee58, 0x143cbee60, 0x1411e9f50, 0x1406352c0, 0x143cbedf4, 0x1401fc320, 0x1406346d0, 0x1420a28e0, 0x1406350e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143215116, 0x142646960, 0x143cbee80, 0x143cbee90, 0x143cbee92, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::quadruped::query_quadruped_removed::system>
    Replacement::draft(0x140634110, "unknown_140634110", fn_140634110 as *const ()), // sub_140634110
    Replacement::draft(0x1406342b0, "unknown_1406342b0", tpl_1844::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14317d818> as *const ()), // sub_1406342b0
    Replacement::draft(0x140634440, "unknown_140634440", tpl_62998::<0x140634570, 0x141b5ad10> as *const ()), // sub_140634440
    Replacement::draft_throws(0x1406346d0, "unknown_1406346d0", tpl_1498::<0x143cbee94, 0x143cbee9c, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8fd28, 0x143cbee98, 0x143cbeea0, 0x141834a0c> as *const (), Throws::Discard), // sub_1406346d0
    Replacement::draft(0x1406347b0, "unknown_1406347b0", fn_1406347b0 as *const ()), // sub_1406347b0
    Replacement::draft(0x140634810, "unknown_140634810", tpl_65265::<0x143cada18, 0x140634bb0, 0x141b5b1b0> as *const ()), // sub_140634810
    Replacement::draft(0x140634960, "unknown_140634960", tpl_1898::<0x88, 0x10088, 0x141b5b080> as *const ()), // sub_140634960
    Replacement::draft(0x1406349f0, "unknown_1406349f0", tpl_62999::<0x140634570, 0x141b5ad10> as *const ()), // sub_1406349f0
    Replacement::draft(0x140634e10, "unknown_140634e10", fn_140634e10 as *const ()), // sub_140634e10
    Replacement::draft(0x140634ff0, "unknown_140634ff0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b5b080> as *const ()), // sub_140634ff0
    Replacement::draft(0x1406350e0, "unknown_1406350e0", tpl_6144::<0x143cbedf4, 0x5e308, 0x10, 0x8, 0x18, 0x141b5afe0, 0x140635690> as *const ()), // sub_1406350e0
    Replacement::draft(0x1406352c0, "unknown_1406352c0", tpl_3188::<0x1406357c0, 0x143214988, 0x140635690, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1406352c0
    Replacement::draft(0x140635510, "unknown_140635510", fn_140635510 as *const ()), // sub_140635510
    Replacement::draft(0x140635690, "unknown_140635690", fn_140635690 as *const ()), // sub_140635690
    Replacement::draft(0x1406357c0, "unknown_1406357c0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143214988, 0x8, 0x20> as *const ()), // sub_1406357c0
    Replacement::draft(0x140635c80, "unknown_140635c80", fn_140635c80 as *const ()), // sub_140635c80
    Replacement::draft(0x140635cb0, "bonsai_140635cb0", fn_140635cb0 as *const ()), // bonsai::PositionReservationEntry::bonsai::PositionReservationEntry::vfunc_0
    Replacement::draft(0x140635d10, "bonsai_140635d10", fn_140635d10 as *const ()), // sub_140635d10
    Replacement::draft(0x140635df0, "bonsai_140635df0", fn_140635df0 as *const ()), // sub_140635df0
    Replacement::draft(0x140635e10, "bonsai_140635e10", fn_140635e10 as *const ()), // bonsai::EntityPositionReservation::bonsai::EntityPositionReservation::vfunc_0
    Replacement::draft(0x140635e60, "bonsai_140635e60", fn_140635e60 as *const ()), // sub_140635e60
    Replacement::draft_throws(0x140635f80, "bonsai_140635f80", tpl_5273::<0x143cbf034, 0x143cbf038, 0x1411e9f50, 0x1406368b0, 0x140636c90, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143215bb6, 0x142646960, 0x143cbf058, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::positionreservation::update>
    Replacement::draft(0x1406361c0, "unknown_1406361c0", tpl_6154::<0x8, 0xc, 0x1, 0x14062ee30> as *const ()), // sub_1406361c0
    Replacement::draft_throws(0x140636230, "unknown_140636230", fn_140636230 as *const (), Throws::Discard), // sub_140636230
    Replacement::draft(0x140636390, "unknown_140636390", fn_140636390 as *const ()), // sub_140636390
    Replacement::draft(0x140636410, "unknown_140636410", fn_140636410 as *const ()), // sub_140636410
    Replacement::draft(0x140636490, "unknown_140636490", fn_140636490 as *const ()), // sub_140636490
    Replacement::draft(0x140636570, "unknown_140636570", fn_140636570 as *const ()), // sub_140636570
    Replacement::draft_throws(0x1406368b0, "unknown_1406368b0", tpl_4746::<0x140348040, 0x143c92a80, 0x1420af120, 0x140215750, 0x143c9a628, 0x140215610, 0x143c8d910, 0x1420a28e0> as *const (), Throws::Discard), // sub_1406368b0
    Replacement::draft(0x140636a20, "unknown_140636a20", tpl_3646::<0x8, 0xc, 0x14052f910, 0x14183acc8> as *const ()), // sub_140636a20
    Replacement::draft(0x140636b00, "unknown_140636b00", fn_140636b00 as *const ()), // sub_140636b00
    Replacement::draft_throws(0x140636c90, "unknown_140636c90", tpl_5944::<0x143c92a80, 0x20, 0x143c9a628, 0x8, 0x580e0, 0x143c8d910, 0x141b5c310> as *const (), Throws::Discard), // sub_140636c90
    Replacement::draft(0x140636d10, "unknown_140636d10", fn_140636d10 as *const ()), // sub_140636d10
    Replacement::draft(0x140636d50, "unknown_140636d50", fn_140636d50 as *const ()), // sub_140636d50
    Replacement::draft(0x140636dd0, "unknown_140636dd0", fn_140636dd0 as *const ()), // sub_140636dd0
    Replacement::draft(0x140637030, "unknown_140637030", fn_140637030 as *const ()), // sub_140637030
    Replacement::draft(0x140637130, "unknown_140637130", tpl_1663::<0x8, 0x140636410, 0x1401e66f0> as *const ()), // sub_140637130
    Replacement::draft(0x1406371b0, "unknown_1406371b0", tpl_1526::<0x14278a330, 0x1410d6ca0, 0x140638290, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_1406371b0
    Replacement::draft_throws(0x140637450, "bonsai_140637450", tpl_4475::<0x143cbf0a0, 0x143cbf0a8, 0x1411e9f50, 0x1406394e0, 0x143cbf220, 0x1420af120, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x140638d50, 0x140638890, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143215da6, 0x142646960, 0x143cbf0c8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::player_obstacle::update::system>
    Replacement::draft(0x140637760, "bonsai_140637760", tpl_1094::<0x143cbf0d0, 0x140638590, 0x14020e8f0> as *const ()), // sub_140637760
    Replacement::draft(0x140637960, "bonsai_140637960", tpl_1436::<0x58, 0x60, 0x158, 0x143cbf0d0, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140637960
    Replacement::draft(0x140637a40, "bonsai_140637a40", tpl_1403::<0x143cbf0d4, 0x140638710, 0x14020e8f0> as *const ()), // sub_140637a40
    Replacement::draft_throws(0x140637c50, "bonsai_140637c50", fn_140637c50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::player_obstacle::stream_in::system>
    Replacement::draft_throws(0x140637fb0, "bonsai_140637fb0", tpl_1467::<0x143cbf10c, 0x143cbf110, 0x1411e9f50, 0x140639b50, 0x143cbf2b8, 0x1420af120, 0x1420a28e0, 0x140639350, 0x140638cc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143216596, 0x142646960, 0x143cbf130, 0x143cbf138, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::player_obstacle::stream_out::system>
    Replacement::draft(0x140638290, "bonsai_140638290", fn_140638290 as *const ()), // ecs::system::ecs::system::wrapper<bonsai::player_obstacle::query_removed::system>
    Replacement::draft(0x140638590, "unknown_140638590", fn_140638590 as *const ()), // sub_140638590
    Replacement::draft(0x140638710, "unknown_140638710", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143216b80> as *const ()), // sub_140638710
    Replacement::draft(0x140638890, "unknown_140638890", tpl_62759::<0x88, 0x10088, 0x143c8fd28, 0x580e0, 0x1406389e0, 0x141b5d8a0> as *const ()), // sub_140638890
    Replacement::draft(0x140638b10, "unknown_140638b10", tpl_6168::<0x7, 0x10, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140638b10
    Replacement::draft(0x140638b70, "unknown_140638b70", tpl_6169::<0x143c8fd28, 0x142407c80, 0x143cada18, 0x143c8d160, 0x142489ee0, 0x141b5d6f0> as *const ()), // sub_140638b70
    Replacement::draft(0x140638cc0, "unknown_140638cc0", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x140637960> as *const ()), // sub_140638cc0
    Replacement::draft(0x140638d50, "unknown_140638d50", tpl_62765::<0x143c8fd28, 0x143cbf0d0, 0x142489ee0, 0x143c917cc, 0x143c8d120, 0x141b5d8a0> as *const ()), // sub_140638d50
    Replacement::draft(0x1406391a0, "unknown_1406391a0", tpl_6170::<0x143c8fd28, 0x142407c80, 0x143cada18, 0x143c8d160, 0x142489ee0, 0x141b5d6f0> as *const ()), // sub_1406391a0
    Replacement::draft(0x140639350, "unknown_140639350", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140637960> as *const ()), // sub_140639350
    Replacement::draft(0x140639430, "unknown_140639430", tpl_3172::<0x143cbf0d0, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141b5d560, 0x140639980> as *const ()), // sub_140639430
    Replacement::draft(0x1406395f0, "unknown_1406395f0", tpl_3188::<0x140639ab0, 0x143216b38, 0x140639980, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1406395f0
    Replacement::draft(0x140639820, "unknown_140639820", fn_140639820 as *const ()), // sub_140639820
    Replacement::draft(0x140639980, "unknown_140639980", fn_140639980 as *const ()), // sub_140639980
    Replacement::draft(0x140639ab0, "unknown_140639ab0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143216b38, 0x8, 0x20> as *const ()), // sub_140639ab0
    Replacement::draft(0x140639f50, "bonsai_140639f50", tpl_6113::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1d, 0x143216d00, 0x1f, 0x8, 0x1c> as *const ()), // sub_140639f50
    Replacement::draft(0x140639fe0, "bonsai_140639fe0", tpl_6114::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143216d00, 0xd, 0xf, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_140639fe0
    Replacement::draft_throws(0x14063a230, "bonsai_14063a230", fn_14063a230 as *const (), Throws::Caught), // sub_14063a230
    Replacement::draft_throws(0x14063a3c0, "bonsai_14063a3c0", tpl_1905::<0x14387c2b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c8e0> as *const (), Throws::Discard), // sub_14063a3c0
    Replacement::draft_throws(0x14063a470, "bonsai_14063a470", fn_14063a470 as *const (), Throws::Discard), // sub_14063a470
    Replacement::draft(0x14063a5c0, "bonsai_14063a5c0", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143216fc0, 0x15, 0x17, 0x8> as *const ()), // sub_14063a5c0
    Replacement::draft(0x14063a660, "bonsai_14063a660", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143216fc0, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14063a660
    Replacement::draft_throws(0x14063a8b0, "bonsai_14063a8b0", fn_14063a8b0 as *const (), Throws::Caught), // sub_14063a8b0
    Replacement::draft_throws(0x14063aa40, "bonsai_14063aa40", tpl_1905::<0x14387c2c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14063aa40
    Replacement::draft_throws(0x14063aaf0, "bonsai_14063aaf0", fn_14063aaf0 as *const (), Throws::Discard), // sub_14063aaf0
    Replacement::draft(0x14063ac40, "bonsai_14063ac40", tpl_6116::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x11, 0x1432172c0, 0x15, 0x17, 0x38> as *const ()), // sub_14063ac40
    Replacement::draft(0x14063ace0, "bonsai_14063ace0", tpl_6117::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x1432172c0, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_14063ace0
    Replacement::draft_throws(0x14063af40, "bonsai_14063af40", fn_14063af40 as *const (), Throws::Caught), // sub_14063af40
    Replacement::draft_throws(0x14063b0d0, "bonsai_14063b0d0", tpl_1905::<0x14387c2d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_14063b0d0
    Replacement::draft_throws(0x14063b350, "bonsai_14063b350", tpl_1100::<0x143cbf348, 0x143cbf350, 0x1411e9f50, 0x14063d980, 0x143cbf5a0, 0x1420af120, 0x1420a28e0, 0x14063d070, 0x14063c770, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432177d6, 0x142646960, 0x143cbf370, 0x143cbf380, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::npcdatacollection::update_entity::system>
    Replacement::draft_throws(0x14063b630, "bonsai_14063b630", tpl_6176::<0x143cbf384, 0x143cbf388, 0x1411e9f50, 0x14063c8e0, 0x14063d270, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143217e86, 0x142646960, 0x143cbf3a8, 0x143cbf3b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::npc_data_collection::update_global_data::system>
    Replacement::draft_throws(0x14063b880, "bonsai_14063b880", tpl_6176::<0x143cbf3c0, 0x143cbf3c8, 0x1411e9f50, 0x14063cac0, 0x14063d330, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143217b66, 0x142646960, 0x143cbf3e8, 0x143cbf3f8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::npc_data_collection::perform_los_checks::system>
    Replacement::draft_throws(0x14063bad0, "bonsai_14063bad0", fn_14063bad0 as *const (), Throws::Discard), // sub_14063bad0
    Replacement::draft_throws(0x14063bc20, "bonsai_14063bc20", fn_14063bc20 as *const (), Throws::Discard), // sub_14063bc20
    Replacement::draft(0x14063bda0, "bonsai_14063bda0", tpl_1098::<0x143ca760c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14063bda0
    Replacement::draft_throws(0x14063c0e0, "bonsai_14063c0e0", tpl_5010::<0x143cbf400, 0x143cbf408, 0x1411e9f50, 0x14063de70, 0x143cbf70c, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x14063d3c0, 0x14063ce90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432175c6, 0x142646960, 0x143cbf428, 0x143cbf430, 0x143cbf434, 0x143cbf436, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::npc_data_collection::stream_in::system>
    Replacement::draft_throws(0x14063c410, "bonsai_14063c410", tpl_1102::<0x143cbf438, 0x143cbf440, 0x1411e9f50, 0x14063dd30, 0x143cbf708, 0x1420af120, 0x1420a28e0, 0x14063d4e0, 0x14063cf60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143218b66, 0x142646960, 0x143cbf460, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::npc_data_collection::stream_out::system>
    Replacement::draft(0x14063c6f0, "bonsai_14063c6f0", fn_14063c6f0 as *const ()), // sub_14063c6f0
    Replacement::draft_throws(0x14063c770, "bonsai_14063c770", fn_14063c770 as *const (), Throws::Discard), // sub_14063c770
    Replacement::draft_throws(0x14063c800, "bonsai_14063c800", fn_14063c800 as *const (), Throws::Discard), // sub_14063c800
    Replacement::draft_throws(0x14063c8e0, "bonsai_14063c8e0", fn_14063c8e0 as *const (), Throws::Discard), // sub_14063c8e0
    Replacement::draft_throws(0x14063cac0, "bonsai_14063cac0", tpl_5909::<0x14063d640, 0x143cbf594, 0x1420af120, 0x140215750, 0x143c8fb50, 0x140215610, 0x143c8d08c, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_14063cac0
    Replacement::draft(0x14063ce90, "bonsai_14063ce90", tpl_6086::<0x142407c80, 0x143cada18, 0x580e0, 0x580f0, 0x141b5fca0> as *const ()), // sub_14063ce90
    Replacement::draft(0x14063cf60, "bonsai_14063cf60", tpl_1127::<0x142407c80, 0x143ca760c, 0x142489ee0, 0x141b5fab0> as *const ()), // sub_14063cf60
    Replacement::draft_throws(0x14063d070, "bonsai_14063d070", fn_14063d070 as *const (), Throws::Discard), // sub_14063d070
    Replacement::draft_throws(0x14063d270, "bonsai_14063d270", fn_14063d270 as *const (), Throws::Discard), // sub_14063d270
    Replacement::draft_throws(0x14063d330, "bonsai_14063d330", tpl_4147::<0x143cbf594, 0x143c8d08c, 0x20, 0x143c8fb50, 0x8, 0x580e0, 0x143c8f63c, 0x141b5fe50> as *const (), Throws::Discard), // sub_14063d330
    Replacement::draft(0x14063d3c0, "bonsai_14063d3c0", tpl_6088::<0x142407c80, 0x143cada18, 0x141b5fca0> as *const ()), // sub_14063d3c0
    Replacement::draft(0x14063d4e0, "bonsai_14063d4e0", tpl_1131::<0x142407c80, 0x143ca760c, 0x142489ee0, 0x141b5fab0> as *const ()), // sub_14063d4e0
    Replacement::draft(0x14063d980, "bonsai_14063d980", fn_14063d980 as *const ()), // sub_14063d980
    Replacement::draft(0x14063db40, "bonsai_14063db40", fn_14063db40 as *const ()), // sub_14063db40
    Replacement::draft(0x14063e380, "bonsai_14063e380", fn_14063e380 as *const ()), // sub_14063e380
    Replacement::draft(0x14063e4f0, "bonsai_14063e4f0", tpl_1663::<0x8, 0x14063c6f0, 0x1401e66f0> as *const ()), // sub_14063e4f0
    Replacement::draft_throws(0x14063e530, "bonsai_14063e530", fn_14063e530 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::noiseeventstorage::update>
    Replacement::draft(0x14063e7a0, "unknown_14063e7a0", fn_14063e7a0 as *const ()), // sub_14063e7a0
    Replacement::draft(0x14063e890, "std_14063e890", fn_14063e890 as *const ()), // std::_Ref_count_obj_alloc3<bonsai::global::NoiseEventStorage::NoiseEventStoragePtr,r::Allocator<bonsai::global::NoiseEve
    Replacement::draft(0x14063e8c0, "std_14063e8c0", fn_14063e8c0 as *const ()), // std::_Ref_count_obj_alloc3<bonsai::global::NoiseEventStorage::NoiseEventStoragePtr,r::Allocator<bonsai::global::NoiseEve
    Replacement::draft(0x14063e8f0, "unknown_14063e8f0", tpl_6190::<0x141b623c0> as *const ()), // sub_14063e8f0
    Replacement::draft(0x14063e900, "unknown_14063e900", fn_14063e900 as *const ()), // sub_14063e900
    Replacement::draft(0x14063e930, "unknown_14063e930", fn_14063e930 as *const ()), // sub_14063e930
    Replacement::draft_throws(0x14063ea20, "unknown_14063ea20", tpl_1480::<0x14278ae50, 0x1410d6ca0, 0x140640560, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14063ea20
    Replacement::draft(0x14063ecc0, "unknown_14063ecc0", tpl_1526::<0x14278ae50, 0x1410d6ca0, 0x140640770, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14063ecc0
    Replacement::draft(0x14063ef60, "navigation_14063ef60", fn_14063ef60 as *const ()), // ecs::system::ecs::system::wrapper<navigation::navmesh::query_removed::query>
    Replacement::draft(0x14063f250, "navigation_14063f250", tpl_1094::<0x143cbf874, 0x140640a80, 0x14020e8f0> as *const ()), // sub_14063f250
    Replacement::draft(0x14063f450, "navigation_14063f450", tpl_1094::<0x143cbf878, 0x140640d50, 0x14020e8f0> as *const ()), // sub_14063f450
    Replacement::draft(0x14063f650, "navigation_14063f650", tpl_1403::<0x143cbf87c, 0x140640ee0, 0x14020e8f0> as *const ()), // sub_14063f650
    Replacement::draft(0x14063f830, "navigation_14063f830", tpl_1094::<0x143cbf880, 0x140641060, 0x14020e8f0> as *const ()), // sub_14063f830
    Replacement::draft(0x14063fa30, "navigation_14063fa30", tpl_1094::<0x143cbf884, 0x1406411e0, 0x14020e8f0> as *const ()), // sub_14063fa30
    Replacement::draft_throws(0x14063fcc0, "navigation_14063fcc0", tpl_1428::<0x143cbf888, 0x143cbf890, 0x1411e9f50, 0x140641c60, 0x1406430c0, 0x140641de0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143219326, 0x142646960, 0x143cbf8b0, 0x143cbf8b4, 0x143cbf8b6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<navigation::navmesh::stream_in::system>
    Replacement::draft_throws(0x14063ff10, "navigation_14063ff10", tpl_5526::<0x143cbf8b8, 0x143cbf8c0, 0x140641360, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143219c36, 0x142646960, 0x143cbf8e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<navigation::navmesh::stream_out::system>
    Replacement::draft(0x140640110, "unknown_140640110", fn_140640110 as *const ()), // sub_140640110
    Replacement::draft_throws(0x140640320, "coregame_140640320", tpl_6195::<0x143cbf8e8, 0x143cbf8f0, 0x140641520, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143219ed6, 0x142646960, 0x143cbf910, 0x143cbf930, 0x143cbf940, 0x143cbf948, 0x143cbf94a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::NavmeshVolumeCompon
    Replacement::draft_throws(0x140640560, "navigation_140640560", tpl_1793::<0x143cbf94c, 0x143cbf950, 0x1406416e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143219886, 0x142646960, 0x143cbf970, 0x143cbf978, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<navigation::navmesh::hide_update::system>
    Replacement::draft(0x140640770, "navigation_140640770", tpl_5938::<0x143cbf97c, 0x143cbf980, 0x1406418a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143219b26, 0x142646960, 0x143cbf9a0, 0x143cbf9a8, 0x143cbf9aa, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<navigation::ai_cover::query_removed::query>
    Replacement::draft(0x140640a80, "unknown_140640a80", fn_140640a80 as *const ()), // sub_140640a80
    Replacement::draft(0x140640d50, "unknown_140640d50", tpl_6197::<0x1401e6d50, 0x140285d50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14321a930> as *const ()), // sub_140640d50
    Replacement::draft(0x140640ee0, "unknown_140640ee0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14321abf0> as *const ()), // sub_140640ee0
    Replacement::draft(0x140641060, "unknown_140641060", fn_140641060 as *const ()), // sub_140641060
    Replacement::draft(0x1406411e0, "unknown_1406411e0", fn_1406411e0 as *const ()), // sub_1406411e0
    Replacement::draft_throws(0x140641360, "unknown_140641360", tpl_5552::<0x1411e9f50, 0x1406440d0, 0x143cbfbc8, 0x1420af120, 0x143c8fd28, 0x140215610, 0x143ca77b8, 0x1420a28e0, 0x140643310, 0x140641ff0> as *const (), Throws::Discard), // sub_140641360
    Replacement::draft_throws(0x140641520, "unknown_140641520", tpl_3011::<0x1411e9f50, 0x140643fe0, 0x143cbfbc4, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140643510, 0x1401de410> as *const (), Throws::Discard), // sub_140641520
    Replacement::draft_throws(0x1406416e0, "unknown_1406416e0", tpl_5552::<0x1411e9f50, 0x140643ea0, 0x143cbfbc0, 0x1420af120, 0x143ca77b8, 0x140215610, 0x143c8fd28, 0x1420a28e0, 0x140643580, 0x140642100> as *const (), Throws::Discard), // sub_1406416e0
    Replacement::draft(0x1406418a0, "unknown_1406418a0", tpl_1988::<0x1411e9f50, 0x140643940, 0x143cbf880, 0x1401fc320, 0x143ca77b8, 0x140215610, 0x143c8fd28, 0x1420a28e0, 0x1406437e0, 0x1401de410> as *const ()), // sub_1406418a0
    Replacement::draft(0x140641a40, "unknown_140641a40", fn_140641a40 as *const ()), // sub_140641a40
    Replacement::draft(0x140641af0, "content_140641af0", fn_140641af0 as *const ()), // content::NavmeshVolumeComponent::content::NavmeshVolumeComponent::vfunc_0
    Replacement::draft(0x140641b40, "navigation_140641b40", tpl_1995::<0x14321abe0, 0x1401de830, 0x141834d30> as *const ()), // navigation::NavmeshData::navigation::NavmeshData::vfunc_0
    Replacement::draft(0x140641b90, "unknown_140641b90", tpl_3600::<0x10, 0x8, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_140641b90
    Replacement::draft(0x140641be0, "unknown_140641be0", fn_140641be0 as *const ()), // sub_140641be0
    Replacement::draft_throws(0x140641c60, "unknown_140641c60", tpl_5657::<0x140644210, 0x143cbfbcc, 0x1420af120, 0x143c8fd28, 0x140215610, 0x143ca77b8, 0x143cada18, 0x1420a28e0> as *const (), Throws::Discard), // sub_140641c60
    Replacement::draft_throws(0x140641de0, "unknown_140641de0", fn_140641de0 as *const (), Throws::Caught), // sub_140641de0
    Replacement::draft(0x140641f50, "unknown_140641f50", fn_140641f50 as *const ()), // sub_140641f50
    Replacement::draft(0x140641ff0, "unknown_140641ff0", tpl_6205::<0x143c8fd28, 0x143ca77b8, 0x142407c80, 0x1406426a0, 0x141b63650> as *const ()), // sub_140641ff0
    Replacement::draft_throws(0x140642100, "unknown_140642100", fn_140642100 as *const (), Throws::Discard), // sub_140642100
    Replacement::draft(0x140642240, "unknown_140642240", tpl_3172::<0x143cbf884, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141b633d0, 0x140642a80> as *const ()), // sub_140642240
    Replacement::draft(0x1406422f0, "unknown_1406422f0", tpl_3188::<0x1406438a0, 0x14321ad98, 0x140642a80, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1406422f0
    Replacement::draft(0x140642920, "unknown_140642920", fn_140642920 as *const ()), // sub_140642920
    Replacement::draft(0x140642a80, "unknown_140642a80", fn_140642a80 as *const ()), // sub_140642a80
    Replacement::draft_throws(0x1406430c0, "unknown_1406430c0", fn_1406430c0 as *const (), Throws::Discard), // sub_1406430c0
    Replacement::draft(0x140643310, "unknown_140643310", tpl_6211::<0x143c8fd28, 0x143ca77b8, 0x142407c80, 0x143cbf884, 0x142489ee0, 0x143cbf880, 0x141b63650> as *const ()), // sub_140643310
    Replacement::draft(0x140643510, "unknown_140643510", tpl_3028::<0x143cbfbc4, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140640110> as *const ()), // sub_140643510
    Replacement::draft_throws(0x140643580, "unknown_140643580", fn_140643580 as *const (), Throws::Discard), // sub_140643580
    Replacement::draft(0x1406437e0, "unknown_1406437e0", tpl_5049::<0x143cbf880, 0x5e308, 0x580e0, 0x143ca77b8, 0x143c8fd28, 0x10, 0x8, 0x18, 0x141b634b0, 0x140643ce0> as *const ()), // sub_1406437e0
    Replacement::draft(0x1406438a0, "unknown_1406438a0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14321ad98, 0x8, 0x20> as *const ()), // sub_1406438a0
    Replacement::draft(0x140643940, "unknown_140643940", tpl_3188::<0x140643e00, 0x14321adc0, 0x140643ce0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140643940
    Replacement::draft(0x1406439c0, "unknown_1406439c0", tpl_4348::<0x10, 0x18, 0x140643ce0> as *const ()), // sub_1406439c0
    Replacement::draft(0x140643b70, "unknown_140643b70", fn_140643b70 as *const ()), // sub_140643b70
    Replacement::draft(0x140643ce0, "unknown_140643ce0", fn_140643ce0 as *const ()), // sub_140643ce0
    Replacement::draft(0x140643e00, "unknown_140643e00", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14321adc0, 0x8, 0x20> as *const ()), // sub_140643e00
    Replacement::draft(0x140644590, "unknown_140644590", fn_140644590 as *const ()), // sub_140644590
    Replacement::draft(0x140644670, "unknown_140644670", tpl_1526::<0x14278afb0, 0x1410d6ca0, 0x140645cc0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140644670
    Replacement::draft_throws(0x140644910, "bonsai_140644910", tpl_4197::<0x143cbfce4, 0x143cbfce8, 0x1411e9f50, 0x140647420, 0x143cbfed4, 0x1420af120, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x140646aa0, 0x1406462f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321b1e6, 0x142646960, 0x143cbfd08, 0x143cbfd18, 0x143cbfd1c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navmesh_modifier_zone::create_object::system>
    Replacement::draft_throws(0x140644c30, "bonsai_140644c30", tpl_3202::<0x143cbfd20, 0x143cbfd28, 0x1411e9f50, 0x140647310, 0x143cbfed0, 0x1420af120, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x140646cb0, 0x1406464e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321b4d6, 0x142646960, 0x143cbfd48, 0x143cbfd58, 0x143cbfd5c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navmesh_modifier_zone::destroy_object::system>
    Replacement::draft(0x140644f60, "bonsai_140644f60", tpl_1098::<0x143c91ad4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140644f60
    Replacement::draft(0x1406452a0, "bonsai_1406452a0", tpl_1094::<0x143c91b20, 0x140645fc0, 0x14020e8f0> as *const ()), // sub_1406452a0
    Replacement::draft(0x1406454a0, "bonsai_1406454a0", tpl_1094::<0x143c91c28, 0x140646140, 0x14020e8f0> as *const ()), // sub_1406454a0
    Replacement::draft_throws(0x1406456d0, "bonsai_1406456d0", tpl_3463::<0x143cbfd60, 0x143cbfd68, 0x1411e9f50, 0x140647ce0, 0x143cbffac, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x140646f00, 0x140646910, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321ae16, 0x142646960, 0x143cbfd88, 0x143cbfd98, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navmesh_modifier_zone::stream_in::system>
    Replacement::draft_throws(0x1406459e0, "bonsai_1406459e0", tpl_1074::<0x143cbfd9c, 0x143cbfda0, 0x1411e9f50, 0x140647bf0, 0x143cbffa8, 0x1420af120, 0x1420a28e0, 0x140647020, 0x1406469e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321b046, 0x142646960, 0x143cbfdc0, 0x143cbfdd0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navmesh_modifier_zone::stream_out::system>
    Replacement::draft(0x140645cc0, "bonsai_140645cc0", tpl_6217::<0x143cbfdd4, 0x143cbfdd8, 0x1411e9f50, 0x140647560, 0x143c91b20, 0x1401fc320, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x140647130, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321be86, 0x142646960, 0x143cbfdf8, 0x143cbfe08, 0x143cbfe0c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<bonsai::navmesh_modifier_zone::query_removed::system>
    Replacement::draft(0x140645fc0, "unknown_140645fc0", tpl_3315::<0x1401e6d50, 0x1406468b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14027a900> as *const ()), // sub_140645fc0
    Replacement::draft(0x140646140, "unknown_140646140", fn_140646140 as *const ()), // sub_140646140
    Replacement::draft(0x1406462f0, "unknown_1406462f0", tpl_1915::<0x143c8fd28, 0x140646760, 0x1406463f0> as *const ()), // sub_1406462f0
    Replacement::draft(0x1406463f0, "unknown_1406463f0", tpl_1916::<0x18, 0x20, 0x40, 0x8, 0x141b650a0> as *const ()), // sub_1406463f0
    Replacement::draft(0x1406464e0, "unknown_1406464e0", tpl_62759::<0x88, 0x10088, 0x143c8fd28, 0x580e0, 0x140646630, 0x141b64ec0> as *const ()), // sub_1406464e0
    Replacement::draft(0x1406468b0, "unknown_1406468b0", fn_1406468b0 as *const ()), // sub_1406468b0
    Replacement::draft(0x140646910, "unknown_140646910", tpl_6086::<0x142407c80, 0x143cada18, 0x580e0, 0x580f0, 0x141b64c60> as *const ()), // sub_140646910
    Replacement::draft(0x1406469e0, "unknown_1406469e0", tpl_1126::<0x142407c80, 0x580f0, 0x141b64aa0> as *const ()), // sub_1406469e0
    Replacement::draft(0x140646aa0, "unknown_140646aa0", tpl_61556::<0x143c8fd28, 0x140646760, 0x141b650a0> as *const ()), // sub_140646aa0
    Replacement::draft(0x140646cb0, "unknown_140646cb0", tpl_62765::<0x143c8fd28, 0x143c91c28, 0x142489ee0, 0x143c91b20, 0x143c91ad4, 0x141b64ec0> as *const ()), // sub_140646cb0
    Replacement::draft(0x140646f00, "unknown_140646f00", tpl_6088::<0x142407c80, 0x143cada18, 0x141b64c60> as *const ()), // sub_140646f00
    Replacement::draft(0x140647020, "unknown_140647020", tpl_1130::<0x142407c80, 0x141b64aa0> as *const ()), // sub_140647020
    Replacement::draft(0x140647130, "unknown_140647130", tpl_3172::<0x143c91b20, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141b64a30, 0x140647a10> as *const ()), // sub_140647130
    Replacement::draft_throws(0x1406471e0, "unknown_1406471e0", tpl_847::<0x143cbfe10, 0x143cbffb0, 0x143cbffb8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c91b20, 0x141834a0c> as *const (), Throws::Discard), // sub_1406471e0
    Replacement::draft(0x140647560, "unknown_140647560", tpl_3188::<0x140647b50, 0x14321c050, 0x140647a10, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140647560
    Replacement::draft(0x1406478a0, "unknown_1406478a0", fn_1406478a0 as *const ()), // sub_1406478a0
    Replacement::draft(0x140647a10, "unknown_140647a10", fn_140647a10 as *const ()), // sub_140647a10
    Replacement::draft(0x140647b50, "unknown_140647b50", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14321c050, 0x8, 0x20> as *const ()), // sub_140647b50
    Replacement::draft(0x140647de0, "unknown_140647de0", fn_140647de0 as *const ()), // sub_140647de0
    Replacement::draft(0x1406480c0, "unknown_1406480c0", tpl_1171::<0x58, 0x60, 0x143cc007c, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1406480c0
    Replacement::draft(0x1406481a0, "unknown_1406481a0", tpl_1171::<0x58, 0x60, 0x143cc0078, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_1406481a0
    Replacement::draft_throws(0x140648280, "bonsai_140648280", tpl_4955::<0x143cc0048, 0x143cc0050, 0x1411e9f50, 0x14064a0c0, 0x143cc01b0, 0x1420af120, 0x1420a28e0, 0x140649740, 0x140649290, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321c486, 0x142646960, 0x143cc0070, 0x143cc0074, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::blocker_zone::update::system>
    Replacement::draft(0x140648560, "bonsai_140648560", tpl_1095::<0x143cc0078, 0x14020e8f0> as *const ()), // sub_140648560
    Replacement::draft(0x140648740, "bonsai_140648740", tpl_1095::<0x143cc007c, 0x14020e8f0> as *const ()), // sub_140648740
    Replacement::draft(0x140648920, "bonsai_140648920", tpl_1403::<0x143cc0080, 0x140649110, 0x14020e8f0> as *const ()), // sub_140648920
    Replacement::draft_throws(0x140648b30, "bonsai_140648b30", tpl_3086::<0x143cc0084, 0x143cc0088, 0x1411e9f50, 0x14064a310, 0x143cc0294, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x140649d90, 0x140649500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321c746, 0x142646960, 0x143cc00a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::blocker_zone::stream_in::system>
    Replacement::draft_throws(0x140648e30, "bonsai_140648e30", tpl_3683::<0x143cc00b0, 0x143cc00b8, 0x1411e9f50, 0x14064a200, 0x143cc0290, 0x1420af120, 0x1420a28e0, 0x140649f70, 0x140649650, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321c1e6, 0x142646960, 0x143cc00d8, 0x143cc00e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::blocker_zone::stream_out::system>
    Replacement::draft(0x140649110, "unknown_140649110", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14321d330> as *const ()), // sub_140649110
    Replacement::draft(0x140649290, "unknown_140649290", tpl_1113::<0x88, 0x10088, 0x142407c80, 0x1406493b0, 0x580f0, 0x141b66470> as *const ()), // sub_140649290
    Replacement::draft(0x1406493b0, "unknown_1406493b0", tpl_3796::<0x143c8d87c, 0x142489ee0, 0x143c91ad4, 0x143cc0078> as *const ()), // sub_1406493b0
    Replacement::draft(0x140649500, "unknown_140649500", fn_140649500 as *const ()), // sub_140649500
    Replacement::draft(0x140649650, "unknown_140649650", tpl_6226::<0x88, 0x10088, 0x142407c80, 0x180a8, 0x143cc0078, 0x18090, 0x142489ee0, 0x580f0, 0x141b66510> as *const ()), // sub_140649650
    Replacement::draft(0x140649740, "unknown_140649740", fn_140649740 as *const ()), // sub_140649740
    Replacement::draft(0x140649d90, "unknown_140649d90", fn_140649d90 as *const ()), // sub_140649d90
    Replacement::draft(0x140649f70, "unknown_140649f70", tpl_6229::<0x142407c80, 0x143cc0078, 0x142489ee0, 0x141b66510> as *const ()), // sub_140649f70
    Replacement::draft(0x14064a310, "unknown_14064a310", fn_14064a310 as *const ()), // sub_14064a310
    Replacement::draft(0x14064a810, "unknown_14064a810", tpl_1171::<0x58, 0x60, 0x143cc03ac, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14064a810
    Replacement::draft(0x14064a8f0, "unknown_14064a8f0", tpl_60443::<0x143c914bc, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x14064a9aa> as *const ()), // sub_14064a8f0
    Replacement::draft_throws(0x14064a9b0, "unknown_14064a9b0", tpl_1480::<0x14278b200, 0x1410d6ca0, 0x14064cd70, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14064a9b0
    Replacement::draft_throws(0x14064ac50, "unknown_14064ac50", tpl_1480::<0x14278b200, 0x1410d6ca0, 0x14064cfe0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14064ac50
    Replacement::draft(0x14064aef0, "unknown_14064aef0", tpl_1526::<0x14278b200, 0x1410d6ca0, 0x14064d2b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14064aef0
    Replacement::draft_throws(0x14064b190, "unknown_14064b190", tpl_1480::<0x14278b200, 0x1410d6ca0, 0x14064d5b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14064b190
    Replacement::draft_throws(0x14064b430, "bonsai_14064b430", tpl_3465::<0x143cc0324, 0x143cc0328, 0x1411e9f50, 0x14064dc00, 0x14064eba0, 0x14064dd90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321fb66, 0x142646960, 0x143cc0348, 0x143cc0358, 0x143cc035a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::update_active::system>
    Replacement::draft_throws(0x14064b680, "bonsai_14064b680", fn_14064b680 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::update_inactive::system>
    Replacement::draft(0x14064b9b0, "bonsai_14064b9b0", tpl_1403::<0x143cc03a0, 0x14064d8e0, 0x14020e8f0> as *const ()), // sub_14064b9b0
    Replacement::draft(0x14064bb90, "bonsai_14064bb90", tpl_3278::<0x143cc03a4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14064bb90
    Replacement::draft(0x14064bed0, "bonsai_14064bed0", tpl_1094::<0x143cc039c, 0x14064da60, 0x14020e8f0> as *const ()), // sub_14064bed0
    Replacement::draft(0x14064c0d0, "bonsai_14064c0d0", tpl_1095::<0x143cc03a8, 0x14020e8f0> as *const ()), // sub_14064c0d0
    Replacement::draft(0x14064c2b0, "bonsai_14064c2b0", tpl_3073::<0x143cc0398, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14064c2b0
    Replacement::draft(0x14064c610, "bonsai_14064c610", tpl_1095::<0x143cc03ac, 0x14020e8f0> as *const ()), // sub_14064c610
    Replacement::draft_throws(0x14064c820, "bonsai_14064c820", tpl_3062::<0x143cc03b0, 0x143cc03b8, 0x1411e9f50, 0x14064e490, 0x14064f660, 0x14064e6b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321f746, 0x142646960, 0x143cc03d8, 0x143cc03e0, 0x143cc03e4, 0x143cc03e6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::stream_in::system>
    Replacement::draft_throws(0x14064ca80, "bonsai_14064ca80", tpl_3068::<0x143cc03e8, 0x143cc03f0, 0x1411e9f50, 0x140650ee0, 0x143cc0994, 0x1420af120, 0x1420a28e0, 0x14064f850, 0x14064e920, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14321ed16, 0x142646960, 0x143cc0410, 0x143cc0420, 0x143cc0424, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::disable_obstacle::system>
    Replacement::draft_throws(0x14064cd70, "bonsai_14064cd70", fn_14064cd70 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::physics_obstacle_creation::system>
    Replacement::draft_throws(0x14064cfe0, "bonsai_14064cfe0", fn_14064cfe0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::physics_obstacle_destruction::system>
    Replacement::draft(0x14064d2b0, "bonsai_14064d2b0", fn_14064d2b0 as *const ()), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::obstacle_removed::system>
    Replacement::draft_throws(0x14064d5b0, "bonsai_14064d5b0", fn_14064d5b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navigation_obstacle::physics_obstacle_removed::system>
    Replacement::draft(0x14064d8e0, "unknown_14064d8e0", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143220278> as *const ()), // sub_14064d8e0
    Replacement::draft(0x14064da60, "unknown_14064da60", fn_14064da60 as *const ()), // sub_14064da60
    Replacement::draft_throws(0x14064dc00, "unknown_14064dc00", tpl_3103::<0x1406500d0, 0x143cc079c, 0x1420af120, 0x14064e3b0, 0x143c8fd28, 0x140215610, 0x143c8d910, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_14064dc00
    Replacement::draft(0x14064dd90, "unknown_14064dd90", fn_14064dd90 as *const ()), // sub_14064dd90
    Replacement::draft(0x14064def0, "unknown_14064def0", tpl_1110::<0x143cc0794, 0x20, 0x8, 0x143c8fd28, 0x580e0, 0x143c8d910, 0x143c8f63c, 0x142407c80, 0x580f0, 0x10, 0x18, 0x30> as *const ()), // sub_14064def0
    Replacement::draft(0x14064dfd0, "unknown_14064dfd0", fn_14064dfd0 as *const ()), // sub_14064dfd0
    Replacement::draft(0x14064e110, "unknown_14064e110", tpl_3709::<0x143cc0794, 0x20, 0x8, 0x143c8f63c, 0x580e0, 0x142407c80, 0x580f0, 0x10> as *const ()), // sub_14064e110
    Replacement::draft_throws(0x14064e3b0, "unknown_14064e3b0", tpl_1330::<0x8, 0x14064ffe0, 0x143cc0794, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14064e3b0
    Replacement::draft_throws(0x14064e490, "unknown_14064e490", tpl_1108::<0x1406510d0, 0x143cc0998, 0x1420af120, 0x140650fe0, 0x143cc0904, 0x140215750, 0x143c8fd28, 0x140215610, 0x143c8f63c, 0x143cada18, 0x1420a28e0> as *const (), Throws::Discard), // sub_14064e490
    Replacement::draft(0x14064e6b0, "unknown_14064e6b0", fn_14064e6b0 as *const ()), // sub_14064e6b0
    Replacement::draft(0x14064e840, "unknown_14064e840", tpl_3105::<0x143cc0904, 0x20, 0x8, 0x143c8fd28, 0x580e0, 0x143c8f63c, 0x142407c80, 0x580f0, 0x143cada18, 0x10, 0x18, 0x30> as *const ()), // sub_14064e840
    Replacement::draft(0x14064e920, "unknown_14064e920", tpl_6226::<0x88, 0x10088, 0x142407c80, 0x180a8, 0x143cc03ac, 0x18090, 0x142489ee0, 0x580f0, 0x141b67b10> as *const ()), // sub_14064e920
    Replacement::draft_throws(0x14064ea10, "unknown_14064ea10", tpl_5922::<0x140650d30, 0x143cc0990, 0x1420af120, 0x140215750, 0x14064f1b0, 0x143c8fd28, 0x140215610, 0x143c8f63c, 0x1420a28e0> as *const (), Throws::Discard), // sub_14064ea10
    Replacement::draft(0x14064eba0, "unknown_14064eba0", fn_14064eba0 as *const ()), // sub_14064eba0
    Replacement::draft(0x14064edd0, "unknown_14064edd0", fn_14064edd0 as *const ()), // sub_14064edd0
    Replacement::draft(0x14064f660, "unknown_14064f660", fn_14064f660 as *const ()), // sub_14064f660
    Replacement::draft(0x14064f850, "unknown_14064f850", tpl_6229::<0x142407c80, 0x143cc03ac, 0x142489ee0, 0x141b67b10> as *const ()), // sub_14064f850
    Replacement::draft(0x14064f9a0, "unknown_14064f9a0", tpl_3344::<0x20, 0x143cc0990, 0x8, 0x143cc098c, 0x143c8fd28, 0x580e0, 0x143c8f63c, 0x142407c80, 0x141b68540> as *const ()), // sub_14064f9a0
    Replacement::draft(0x14064fab0, "unknown_14064fab0", tpl_4345::<0x143cc039c, 0x143cc098c, 0x142407c80, 0x141b68450, 0x140650800> as *const ()), // sub_14064fab0
    Replacement::draft(0x14064fc20, "unknown_14064fc20", tpl_3172::<0x143cc0398, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141b67a80, 0x140489dd0> as *const ()), // sub_14064fc20
    Replacement::draft(0x14064fcd0, "unknown_14064fcd0", tpl_6237::<0x20, 0x143cc0988, 0x8, 0x143cc098c, 0x142407c80, 0x580f0, 0x141b68170> as *const ()), // sub_14064fcd0
    Replacement::draft_throws(0x14064fda0, "unknown_14064fda0", tpl_847::<0x143cc05c8, 0x143cc09d8, 0x143cc09e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cc0398, 0x141834a0c> as *const (), Throws::Discard), // sub_14064fda0
    Replacement::draft(0x140650210, "unknown_140650210", tpl_3188::<0x140650b40, 0x143220250, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140650210
    Replacement::draft(0x140650290, "unknown_140650290", tpl_3188::<0x140650be0, 0x143220228, 0x140650800, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140650290
    Replacement::draft(0x140650310, "unknown_140650310", tpl_4348::<0x20, 0x28, 0x140650800> as *const ()), // sub_140650310
    Replacement::draft(0x140650670, "unknown_140650670", fn_140650670 as *const ()), // sub_140650670
    Replacement::draft(0x140650800, "unknown_140650800", fn_140650800 as *const ()), // sub_140650800
    Replacement::draft(0x140650920, "unknown_140650920", tpl_65350::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc8, 0x142442de0> as *const ()), // sub_140650920
    Replacement::draft(0x140650b40, "unknown_140650b40", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143220250, 0x8, 0x20> as *const ()), // sub_140650b40
    Replacement::draft(0x140650be0, "unknown_140650be0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143220228, 0x8, 0x20> as *const ()), // sub_140650be0
    Replacement::draft(0x140650d30, "unknown_140650d30", fn_140650d30 as *const ()), // sub_140650d30
    Replacement::draft(0x140651760, "unknown_140651760", tpl_4864::<0x143cc0b48, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140651760
    Replacement::draft(0x140651e50, "unknown_140651e50", fn_140651e50 as *const ()), // sub_140651e50
    Replacement::draft(0x140651f00, "content_140651f00", fn_140651f00 as *const ()), // content::BodyControlComponent::content::BodyControlComponent::vfunc_0
    Replacement::draft(0x140651f70, "content_140651f70", tpl_6246::<0x143222098, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::NavigationLinkComponent::content::NavigationLinkComponent::vfunc_18
    Replacement::draft(0x140652070, "unknown_140652070", tpl_6082::<0x143c919a8, 0x40, 0x1806c, 0x18058, 0x142489ee0> as *const ()), // sub_140652070
    Replacement::draft(0x1406520e0, "unknown_1406520e0", fn_1406520e0 as *const ()), // sub_1406520e0
    Replacement::draft(0x140652230, "bonsai_140652230", fn_140652230 as *const ()), // ecs::system::ecs::system::wrapper<bonsai::navlink::query_removed::system>
    Replacement::draft_throws(0x140652520, "bonsai_140652520", tpl_3353::<0x143cc0c08, 0x143cc0c10, 0x1411e9f50, 0x140653690, 0x140654320, 0x140653900, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143221ae6, 0x142646960, 0x143cc0c20, 0x143cc0c28, 0x143cc0c2c, 0x143cc0c2e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navlink::updateNavLink>
    Replacement::draft(0x140652780, "bonsai_140652780", tpl_1363::<0x30, 0x143c919a8, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1406527cd> as *const ()), // sub_140652780
    Replacement::draft(0x1406527d0, "bonsai_1406527d0", tpl_1094::<0x143cc0c30, 0x140653110, 0x14020e8f0> as *const ()), // sub_1406527d0
    Replacement::draft(0x1406529d0, "bonsai_1406529d0", tpl_1094::<0x143cc0c34, 0x140653290, 0x14020e8f0> as *const ()), // sub_1406529d0
    Replacement::draft_throws(0x140652c00, "bonsai_140652c00", tpl_6249::<0x143cc0c38, 0x143cc0c40, 0x1406534c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432213c6, 0x142646960, 0x143cc0c60, 0x143cc0c62, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navlink::stream_in::system>
    Replacement::draft_throws(0x140652e00, "bonsai_140652e00", tpl_6250::<0x143cc0c64, 0x143cc0c68, 0x1411e9f50, 0x140655590, 0x143cc0e28, 0x1420af120, 0x143c8fd28, 0x140215610, 0x1420a28e0, 0x140655090, 0x140654130, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1432217e6, 0x142646960, 0x143cc0c88, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<bonsai::navlink::stream_out::system>
    Replacement::draft(0x140653110, "unknown_140653110", fn_140653110 as *const ()), // sub_140653110
    Replacement::draft(0x140653290, "unknown_140653290", fn_140653290 as *const ()), // sub_140653290
    Replacement::draft_throws(0x1406534c0, "unknown_1406534c0", tpl_1405::<0x1411e9f50, 0x140655690, 0x143cc0e2c, 0x1420af120, 0x14036fad0, 0x143c8d08c, 0x140215610, 0x143c8fd28, 0x1420a28e0, 0x140654ed0, 0x140653eb0> as *const (), Throws::Discard), // sub_1406534c0
    Replacement::draft_throws(0x140653690, "unknown_140653690", fn_140653690 as *const (), Throws::Discard), // sub_140653690
    Replacement::draft_throws(0x140653900, "unknown_140653900", tpl_3400::<0x143cc0d20, 0x143c8d910, 0x143c8fd28, 0x143c8d08c, 0x143c8ceec, 0x140653d00, 0x140653a90> as *const (), Throws::Discard), // sub_140653900
    Replacement::draft_throws(0x140653a90, "unknown_140653a90", tpl_3401::<0x18, 0x20, 0x40, 0x8, 0x10, 0x30, 0x141b6acd0> as *const (), Throws::Discard), // sub_140653a90
    Replacement::draft(0x140653bc0, "unknown_140653bc0", fn_140653bc0 as *const ()), // sub_140653bc0
    Replacement::draft(0x140653e50, "unknown_140653e50", fn_140653e50 as *const ()), // sub_140653e50
    Replacement::draft_throws(0x140653eb0, "unknown_140653eb0", fn_140653eb0 as *const (), Throws::Discard), // sub_140653eb0
    Replacement::draft(0x140653f70, "unknown_140653f70", tpl_1375::<0x143c94b44, 0x20, 0x8, 0x142407c80, 0x580f0, 0x580e0, 0x143c8d08c, 0x143c8fd28, 0x10, 0x18, 0x28> as *const ()), // sub_140653f70
    Replacement::draft_throws(0x140654030, "unknown_140654030", fn_140654030 as *const (), Throws::Discard), // sub_140654030
    Replacement::draft(0x140654130, "unknown_140654130", tpl_1377::<0x142407c80, 0x143c8fd28, 0x143cc0c30, 0x142489ee0, 0x141b6a4a0> as *const ()), // sub_140654130
    Replacement::draft(0x140654270, "unknown_140654270", tpl_3172::<0x143cc0c30, 0x5e308, 0x143c8fd28, 0x580e0, 0x10, 0x8, 0x18, 0x141b6a3e0, 0x140654ba0> as *const ()), // sub_140654270
    Replacement::draft_throws(0x140654320, "unknown_140654320", tpl_62749::<0x143cc0d20, 0x143c8fd28, 0x143c8d910, 0x143c8d08c, 0x143c8ceec, 0x140653d00, 0x141b6acd0> as *const (), Throws::Discard), // sub_140654320
    Replacement::draft(0x1406545a0, "unknown_1406545a0", tpl_3188::<0x1406554f0, 0x143222258, 0x140654ba0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1406545a0
    Replacement::draft(0x140654a50, "unknown_140654a50", fn_140654a50 as *const ()), // sub_140654a50
    Replacement::draft(0x140654ba0, "unknown_140654ba0", fn_140654ba0 as *const ()), // sub_140654ba0
    Replacement::draft_throws(0x140654ed0, "unknown_140654ed0", fn_140654ed0 as *const (), Throws::Discard), // sub_140654ed0
    Replacement::draft(0x140655090, "unknown_140655090", tpl_6262::<0x142407c80, 0x143c8fd28, 0x143cc0c30, 0x142489ee0, 0x141b6a4a0> as *const ()), // sub_140655090
    Replacement::draft(0x1406554f0, "unknown_1406554f0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143222258, 0x8, 0x20> as *const ()), // sub_1406554f0
    Replacement::draft(0x140655590, "unknown_140655590", fn_140655590 as *const ()), // sub_140655590
    Replacement::draft(0x140655a50, "unknown_140655a50", fn_140655a50 as *const ()), // sub_140655a50
    Replacement::draft(0x140655b50, "bonsai_140655b50", fn_140655b50 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_0
    Replacement::draft(0x140655bd0, "bonsai_140655bd0", fn_140655bd0 as *const ()), // sub_140655bd0
    Replacement::draft(0x140655c70, "bonsai_140655c70", tpl_6267::<0x8, 0xc, 0x14062ee30, 0x14183acc8> as *const ()), // sub_140655c70
    Replacement::draft(0x140655d80, "unknown_140655d80", tpl_1983::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x11, 0x143222358, 0x15, 0x17, 0x8> as *const ()), // sub_140655d80
    Replacement::draft(0x140655e20, "unknown_140655e20", tpl_1984::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x30, 0x143222358, 0xffffffffffffffe1, 0xffffffffffffffe5, 0xffffffffffffffe7, 0xffffffffffffffd8> as *const ()), // sub_140655e20
    Replacement::draft(0x140655ed0, "unknown_140655ed0", tpl_6135::<0x14387c400, 0x14243b0b0, 0x14387c9e8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140655ed0
    Replacement::draft_throws(0x140655f70, "unknown_140655f70", tpl_6136::<0x14387c408, 0x14387c400, 0x1401e9fa0, 0x14387c9e4, 0x14387c9e8, 0x1401e9e70> as *const (), Throws::Discard), // sub_140655f70
    Replacement::draft_throws(0x140655ff0, "unknown_140655ff0", tpl_6136::<0x14387c408, 0x14387c400, 0x1401e9b60, 0x14387c9e4, 0x14387c9e8, 0x1401e9a20> as *const (), Throws::Discard), // sub_140655ff0
    Replacement::draft_throws(0x140656070, "unknown_140656070", fn_140656070 as *const (), Throws::Caught), // sub_140656070
    Replacement::draft(0x140656100, "unknown_140656100", tpl_6137::<0x14387c408, 0x14387c400, 0x1401e9970, 0x14387c9e4, 0x14387c9e8, 0x1401e9860> as *const ()), // sub_140656100
    Replacement::draft_throws(0x140656180, "unknown_140656180", tpl_6136::<0x14387c408, 0x14387c400, 0x1401e9770, 0x14387c9e4, 0x14387c9e8, 0x1401e9590> as *const (), Throws::Discard), // sub_140656180
    Replacement::draft_throws(0x140656200, "unknown_140656200", tpl_1905::<0x14387c400, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387c9e8> as *const (), Throws::Discard), // sub_140656200
    Replacement::draft_throws(0x1406562b0, "unknown_1406562b0", fn_1406562b0 as *const (), Throws::Discard), // sub_1406562b0
    Replacement::draft(0x1406563d0, "unknown_1406563d0", fn_1406563d0 as *const ()), // sub_1406563d0
    Replacement::draft_throws(0x140656520, "unknown_140656520", fn_140656520 as *const (), Throws::Caught), // sub_140656520
    Replacement::draft_throws(0x1406565e0, "unknown_1406565e0", fn_1406565e0 as *const (), Throws::Discard), // sub_1406565e0
    Replacement::draft_throws(0x140656690, "unknown_140656690", fn_140656690 as *const (), Throws::Caught), // sub_140656690
    Replacement::draft_throws(0x140656750, "std_140656750", tpl_5917::<0x14307e950, 0x142645870, 0x1401fc200, 0x140206290, 0x1404bce80, 0x140656809> as *const (), Throws::Discard), // sub_140656750
    Replacement::draft(0x140656960, "unknown_140656960", tpl_6271::<0xc, 0x8, 0x1401dec00, 0x1403d1260> as *const ()), // sub_140656960
    Replacement::draft_throws(0x1406569f0, "unknown_1406569f0", fn_1406569f0 as *const (), Throws::Discard), // sub_1406569f0
    Replacement::draft_throws(0x140656b10, "unknown_140656b10", fn_140656b10 as *const (), Throws::Discard), // sub_140656b10
    Replacement::draft_throws(0x140656ce0, "unknown_140656ce0", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c9a624, 0x142489ee0, 0x1403cd370, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14315dab8, 0x141ef4820, 0x140656df6, 0x140656e2a, 0x140656e3c, 0x140656e51, 0x140656df6, 0x140656e2a, 0x140656e3c, 0x140656e51> as *const (), Throws::Discard), // sub_140656ce0
    Replacement::draft_throws(0x140656e60, "unknown_140656e60", fn_140656e60 as *const (), Throws::Discard), // sub_140656e60
    Replacement::draft_throws(0x140657030, "unknown_140657030", tpl_6276::<0x142119010, 0x142118e10, 0x1401fa730, 0x143ca7870, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1432227d0, 0x141ef4820, 0x140657191, 0x1406571c5, 0x1406571d7, 0x1406571ec, 0x140657191, 0x1406571c5, 0x1406571d7, 0x1406571ec> as *const (), Throws::Discard), // sub_140657030
    Replacement::draft_throws(0x1406571f0, "unknown_1406571f0", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143ca7888, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1432227e8, 0x141ef4820, 0x140657351, 0x140657385, 0x140657397, 0x1406573ac, 0x140657351, 0x140657385, 0x140657397, 0x1406573ac> as *const (), Throws::Discard), // sub_1406571f0
    Replacement::draft_throws(0x1406573b0, "unknown_1406573b0", fn_1406573b0 as *const (), Throws::Caught), // sub_1406573b0
    Replacement::draft(0x1406574b0, "unknown_1406574b0", tpl_3886::<0x1432228a8, 0x8> as *const ()), // sub_1406574b0
];
