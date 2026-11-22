#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1401e3800, "r_1401e3800", fn_1401e3800 as *const ()), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_0
    Replacement::draft(0x1401e38b0, "r_1401e38b0", fn_1401e38b0 as *const ()), // r::Loader_Adaptor_Pack2::r::Loader_Adaptor_Pack2::vfunc_0
    Replacement::draft(0x1401e7380, "r_1401e7380", fn_1401e7380 as *const ()), // sub_1401e7380
    Replacement::draft(0x1401e7430, "r_1401e7430", fn_1401e7430 as *const ()), // sub_1401e7430
    Replacement::draft(0x1401e74b0, "r_1401e74b0", tpl_564::<0x14340ed48, 0x8, 0x14307a4f8, 0x14307a628, 0x14307a800, 0x142498f00> as *const ()), // sub_1401e74b0
    Replacement::draft(0x1401e7510, "r_1401e7510", tpl_564::<0x14340ed48, 0x8, 0x14307a4f8, 0x14307a628, 0x14307a870, 0x142498f00> as *const ()), // sub_1401e7510
    Replacement::draft(0x1401e7590, "r_1401e7590", tpl_565::<0x14340ed48, 0x8, 0x14307a4c8, 0x14307a628, 0x14307a780, 0x142498f00> as *const ()), // sub_1401e7590
    Replacement::draft(0x1401e7610, "r_1401e7610", tpl_565::<0x14340ed48, 0x8, 0x14307a200, 0x14307a628, 0x14307a7c0, 0x142498f00> as *const ()), // sub_1401e7610
    Replacement::draft(0x1401e7690, "r_1401e7690", fn_1401e7690 as *const ()), // sub_1401e7690
    Replacement::draft(0x1401e7720, "r_1401e7720", fn_1401e7720 as *const ()), // sub_1401e7720
    Replacement::draft(0x1401e7790, "r_1401e7790", fn_1401e7790 as *const ()), // sub_1401e7790
    Replacement::draft(0x1401e77c0, "r_1401e77c0", fn_1401e77c0 as *const ()), // sub_1401e77c0
    Replacement::draft(0x1401e7a90, "r_1401e7a90", tpl_570::<0x1401ea410, 0x14340ed68, 0x141834d30> as *const ()), // r::ObjectStreamException_ErrorsOccurred::r::ObjectStreamException_ErrorsOccurred::vfunc_0
    Replacement::draft(0x1401e7ae0, "r_1401e7ae0", tpl_571::<0x1401ea410, 0x14340ed68> as *const ()), // sub_1401e7ae0
    Replacement::draft(0x1401e7cc0, "r_1401e7cc0", fn_1401e7cc0 as *const ()), // sub_1401e7cc0
    Replacement::draft(0x1401e7d00, "r_1401e7d00", fn_1401e7d00 as *const ()), // sub_1401e7d00
    Replacement::draft(0x1401ed5d0, "r_1401ed5d0", fn_1401ed5d0 as *const ()), // r::BufferedMemoryStreamWrapper::r::BufferedMemoryStreamWrapper::vfunc_1
    Replacement::draft(0x1401ed610, "r_1401ed610", fn_1401ed610 as *const ()), // r::BufferedMemoryStreamWrapper::r::BufferedMemoryStreamWrapper::vfunc_2
    Replacement::draft(0x1401ed630, "r_1401ed630", fn_1401ed630 as *const ()), // r::BufferedMemoryStreamWrapper::r::BufferedMemoryStreamWrapper::vfunc_3
    Replacement::draft(0x1401ed660, "r_1401ed660", tpl_660::<0x8, 0x108> as *const ()), // r::BufferedMemoryStreamWrapper::r::BufferedMemoryStreamWrapper::vfunc_4
    Replacement::draft(0x1401ed670, "r_1401ed670", fn_1401ed670 as *const ()), // r::BufferedMemoryStreamWrapper::r::BufferedMemoryStreamWrapper::vfunc_5
    Replacement::draft(0x1401ed6b0, "r_1401ed6b0", fn_1401ed6b0 as *const ()), // r::BufferedMemoryStreamWrapper::r::BufferedMemoryStreamWrapper::vfunc_6
    Replacement::draft(0x1401f49b0, "r_1401f49b0", fn_1401f49b0 as *const ()), // r::internal::ContinuationImpl<r::Expected<platform::User,platform::Error> (*)(const r::Expected<platform::User,platform:
    Replacement::draft(0x1401f4a10, "r_1401f4a10", tpl_743::<0x30, 0x10, 0x18, 0x14307d580, 0x8, 0x1401f3d90, 0x20, 0x14307d598, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1401f4a10
    Replacement::draft(0x1401f4ae0, "r_1401f4ae0", fn_1401f4ae0 as *const ()), // sub_1401f4ae0
    Replacement::draft_throws(0x1401f69a0, "r_1401f69a0", fn_1401f69a0 as *const (), Throws::Discard), // r::GlobalFileSystem<r::WinFileSystem>::r::GlobalFileSystem<r::WinFileSystem>::vfunc_0
    Replacement::draft(0x1401fcd50, "r_1401fcd50", fn_1401fcd50 as *const ()), // sub_1401fcd50
    Replacement::draft_throws(0x1401fce60, "r_1401fce60", fn_1401fce60 as *const (), Throws::Caught), // sub_1401fce60
    Replacement::draft(0x1401fcf20, "r_1401fcf20", fn_1401fcf20 as *const ()), // sub_1401fcf20
    Replacement::draft_throws(0x1401fd120, "r_1401fd120", fn_1401fd120 as *const (), Throws::Caught), // sub_1401fd120
    Replacement::draft_throws(0x1401fd260, "r_1401fd260", crate::shards::subsystems::fn_1401fd260 as *const (), Throws::Caught), // sub_1401fd260
    Replacement::draft_throws(0x1401fd820, "r_1401fd820", tpl_846::<0x143c8d008, 0x143c8d0ac, 0x143c8d0d8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d0b0, 0x143c8d0a0, 0x143c8d024, 0x143c8d0f8, 0x141834a0c> as *const (), Throws::Discard), // sub_1401fd820
    Replacement::draft_throws(0x1401fd990, "r_1401fd990", tpl_847::<0x143c8d06c, 0x143c8cff8, 0x143c8d090, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d000, 0x141834a0c> as *const (), Throws::Discard), // sub_1401fd990
    Replacement::draft(0x1401fdbe0, "r_1401fdbe0", fn_1401fdbe0 as *const ()), // sub_1401fdbe0
    Replacement::draft(0x1401fdc70, "r_1401fdc70", fn_1401fdc70 as *const ()), // sub_1401fdc70
    Replacement::draft(0x1401fdcd0, "r_1401fdcd0", tpl_850::<0x8, 0x14240ab10> as *const ()), // sub_1401fdcd0
    Replacement::draft(0x1401fdd10, "r_1401fdd10", fn_1401fdd10 as *const ()), // sub_1401fdd10
    Replacement::draft_throws(0x1401fde50, "r_1401fde50", crate::shards::subsystems::fn_1401fde50 as *const (), Throws::Caught), // sub_1401fde50
    Replacement::draft(0x1401fe120, "r_1401fe120", fn_1401fe120 as *const ()), // sub_1401fe120
    Replacement::draft(0x1401fe1d0, "r_1401fe1d0", fn_1401fe1d0 as *const ()), // sub_1401fe1d0
    Replacement::draft_throws(0x1401fe340, "r_1401fe340", crate::shards::subsystems::fn_1401fe340 as *const (), Throws::Caught), // sub_1401fe340
    Replacement::draft(0x1401fe7b0, "r_1401fe7b0", tpl_855::<0x1401fa640, 0x1425371a0, 0x142533200, 0x1401fcf20, 0x1401fdbe0> as *const ()), // sub_1401fe7b0
    Replacement::draft_throws(0x1401feaf0, "r_1401feaf0", fn_1401feaf0 as *const (), Throws::Discard), // sub_1401feaf0
    Replacement::draft_throws(0x1401febb0, "r_1401febb0", fn_1401febb0 as *const (), Throws::Discard), // sub_1401febb0
    Replacement::draft(0x140205f20, "r_140205f20", tpl_976::<0x141834d30> as *const ()), // r::JobCounter::r::JobCounter::vfunc_0
    Replacement::draft(0x140214920, "r_140214920", tpl_1130::<0x142407c80, 0x14186dfb0> as *const ()), // sub_140214920
    Replacement::draft(0x140214a30, "r_140214a30", tpl_1130::<0x142407c80, 0x14186dd40> as *const ()), // sub_140214a30
    Replacement::draft(0x140214b40, "r_140214b40", tpl_1131::<0x142407c80, 0x143c8d708, 0x142489ee0, 0x14186dce0> as *const ()), // sub_140214b40
    Replacement::draft(0x140214ca0, "r_140214ca0", tpl_1131::<0x142407c80, 0x143c8d708, 0x142489ee0, 0x14186dc80> as *const ()), // sub_140214ca0
    Replacement::draft(0x140215470, "r_140215470", fn_140215470 as *const ()), // sub_140215470
    Replacement::draft(0x140215890, "r_140215890", fn_140215890 as *const ()), // sub_140215890
    Replacement::draft(0x140215d20, "r_140215d20", tpl_381::<0x1401df0f0, 0x140215f60> as *const ()), // sub_140215d20
    Replacement::draft(0x140215f60, "r_140215f60", fn_140215f60 as *const ()), // sub_140215f60
    Replacement::draft(0x140216d10, "r_140216d10", fn_140216d10 as *const ()), // sub_140216d10
    Replacement::draft(0x14021f8a0, "r_14021f8a0", tpl_1210::<0x7, 0x50, 0x30, 0x1401f2bc0, 0x20, 0x10> as *const ()), // sub_14021f8a0
    Replacement::draft(0x14021f900, "r_14021f900", fn_14021f900 as *const ()), // sub_14021f900
    Replacement::draft(0x14021f9b0, "r_14021f9b0", tpl_1212::<0x8, 0x14021c030> as *const ()), // sub_14021f9b0
    Replacement::draft(0x14021f9e0, "r_14021f9e0", fn_14021f9e0 as *const ()), // sub_14021f9e0
    Replacement::draft(0x14021fa50, "r_14021fa50", tpl_1214::<0x38, 0x8, 0x140222cb0> as *const ()), // sub_14021fa50
    Replacement::draft(0x14021fa90, "r_14021fa90", fn_14021fa90 as *const ()), // sub_14021fa90
    Replacement::draft(0x14021fad0, "r_14021fad0", tpl_1124::<0x40> as *const ()), // sub_14021fad0
    Replacement::draft_throws(0x14021fb00, "r_14021fb00", tpl_1108::<0x1402230d0, 0x143c8e330, 0x1420af120, 0x140222fe0, 0x143c8e10c, 0x140215750, 0x143c8df88, 0x140215610, 0x143c8d00c, 0x143c8de64, 0x1420a28e0> as *const (), Throws::Discard), // sub_14021fb00
    Replacement::draft_throws(0x14021fd20, "r_14021fd20", tpl_60977::<0x14021fe80, 0x140220cc0, 0x141872d90> as *const (), Throws::Discard), // sub_14021fd20
    Replacement::draft(0x14021fe80, "r_14021fe80", tpl_1110::<0x143c8e10c, 0x20, 0x8, 0x143c8df88, 0x580e0, 0x143c8d00c, 0x143c8de64, 0x142407c80, 0x580f0, 0x10, 0x18, 0x30> as *const ()), // sub_14021fe80
    Replacement::draft(0x14021ff60, "r_14021ff60", fn_14021ff60 as *const ()), // sub_14021ff60
    Replacement::draft(0x140220070, "r_140220070", tpl_1217::<0x143c8dbd8, 0x20, 0x8, 0x1418745f0> as *const ()), // sub_140220070
    Replacement::draft(0x1402200d0, "r_1402200d0", fn_1402200d0 as *const ()), // sub_1402200d0
    Replacement::draft(0x140220350, "r_140220350", tpl_60979::<0x143c8df58, 0x143c8d00c, 0x143c8dc20, 0x142489ee0, 0x141874b00> as *const ()), // sub_140220350
    Replacement::draft_throws(0x140220520, "r_140220520", fn_140220520 as *const (), Throws::Discard), // sub_140220520
    Replacement::draft(0x140220c80, "r_140220c80", fn_140220c80 as *const ()), // sub_140220c80
    Replacement::draft(0x140220dc0, "r_140220dc0", fn_140220dc0 as *const ()), // sub_140220dc0
    Replacement::draft(0x140220ff0, "r_140220ff0", fn_140220ff0 as *const ()), // sub_140220ff0
    Replacement::draft(0x1402211e0, "r_1402211e0", tpl_60984::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140222880, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1402211e0
    Replacement::draft(0x1402215b0, "r_1402215b0", fn_1402215b0 as *const ()), // sub_1402215b0
    Replacement::draft(0x1402218e0, "r_1402218e0", fn_1402218e0 as *const ()), // sub_1402218e0
    Replacement::draft(0x1402219a0, "r_1402219a0", fn_1402219a0 as *const ()), // sub_1402219a0
    Replacement::draft_throws(0x140221b10, "r_140221b10", tpl_60987::<0x14021fe80, 0x143c8dbcc, 0x142489ee0, 0x143c8e080, 0x141872d90> as *const (), Throws::Discard), // sub_140221b10
    Replacement::draft(0x1402221c0, "r_1402221c0", tpl_1223::<0x143c8e2e0, 0x143c8dd80, 0x141834a78, 0x143c8de7c, 0x143c8de70, 0x143c8e070, 0x143c8dbc8, 0x143c8e2e8, 0x143c8e080, 0x143c8dbcc, 0x143c8dda0, 0x143c8ddc0, 0x143c8ddd0, 0x141834a0c> as *const ()), // sub_1402221c0
    Replacement::draft(0x1402227b0, "r_1402227b0", fn_1402227b0 as *const ()), // sub_1402227b0
    Replacement::draft(0x140222880, "r_140222880", fn_140222880 as *const ()), // sub_140222880
    Replacement::draft(0x140222970, "r_140222970", fn_140222970 as *const ()), // sub_140222970
    Replacement::draft(0x1402229b0, "r_1402229b0", tpl_1227::<0x8, 0x10, 0x14240ab10> as *const ()), // sub_1402229b0
    Replacement::draft(0x1402229f0, "r_1402229f0", tpl_1228::<0x8, 0x10, 0x1401f2bc0, 0x1402183c0> as *const ()), // sub_1402229f0
    Replacement::draft(0x140222a30, "r_140222a30", fn_140222a30 as *const ()), // sub_140222a30
    Replacement::draft(0x140222cb0, "r_140222cb0", tpl_1097::<0x143c8de6c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140222cb0
    Replacement::draft(0x140223240, "r_140223240", fn_140223240 as *const ()), // sub_140223240
    Replacement::draft(0x1402234f0, "r_1402234f0", fn_1402234f0 as *const ()), // sub_1402234f0
    Replacement::draft(0x1402237b0, "r_1402237b0", fn_1402237b0 as *const ()), // sub_1402237b0
    Replacement::draft(0x140223a70, "r_140223a70", fn_140223a70 as *const ()), // sub_140223a70
    Replacement::draft(0x140223d60, "r_140223d60", fn_140223d60 as *const ()), // sub_140223d60
    Replacement::draft(0x140223e20, "r_140223e20", fn_140223e20 as *const ()), // sub_140223e20
    Replacement::draft(0x140223f40, "r_140223f40", fn_140223f40 as *const ()), // sub_140223f40
    Replacement::draft(0x140224060, "r_140224060", fn_140224060 as *const ()), // sub_140224060
    Replacement::draft(0x140224190, "r_140224190", fn_140224190 as *const ()), // sub_140224190
    Replacement::draft(0x1402242c0, "r_1402242c0", fn_1402242c0 as *const ()), // sub_1402242c0
    Replacement::draft(0x140224b40, "r_140224b40", fn_140224b40 as *const ()), // sub_140224b40
    Replacement::draft(0x140224be0, "r_140224be0", fn_140224be0 as *const ()), // sub_140224be0
    Replacement::draft(0x140224cb0, "r_140224cb0", fn_140224cb0 as *const ()), // sub_140224cb0
    Replacement::draft(0x140224da0, "r_140224da0", fn_140224da0 as *const ()), // sub_140224da0
    Replacement::draft(0x140224e90, "r_140224e90", fn_140224e90 as *const ()), // sub_140224e90
    Replacement::draft(0x140224fc0, "r_140224fc0", fn_140224fc0 as *const ()), // sub_140224fc0
    Replacement::draft(0x140225010, "r_140225010", tpl_1241::<0x8, 0x40, 0x14240ab10> as *const ()), // sub_140225010
    Replacement::draft(0x140225050, "r_140225050", tpl_1227::<0x8, 0x4c, 0x14240ab10> as *const ()), // sub_140225050
    Replacement::draft(0x140225090, "r_140225090", fn_140225090 as *const ()), // sub_140225090
    Replacement::draft(0x1402251c0, "r_1402251c0", fn_1402251c0 as *const ()), // sub_1402251c0
    Replacement::draft(0x140225230, "r_140225230", fn_140225230 as *const ()), // sub_140225230
    Replacement::draft(0x1402252d0, "r_1402252d0", fn_1402252d0 as *const ()), // sub_1402252d0
    Replacement::draft(0x140225370, "r_140225370", fn_140225370 as *const ()), // sub_140225370
    Replacement::draft(0x140225410, "r_140225410", fn_140225410 as *const ()), // sub_140225410
    Replacement::draft(0x1402254b0, "r_1402254b0", fn_1402254b0 as *const ()), // sub_1402254b0
    Replacement::draft(0x140225740, "r_140225740", fn_140225740 as *const ()), // sub_140225740
    Replacement::draft(0x1402257b0, "r_1402257b0", fn_1402257b0 as *const ()), // sub_1402257b0
    Replacement::draft(0x140225870, "r_140225870", fn_140225870 as *const ()), // sub_140225870
    Replacement::draft(0x140225940, "r_140225940", fn_140225940 as *const ()), // sub_140225940
    Replacement::draft(0x140225ef0, "r_140225ef0", fn_140225ef0 as *const ()), // sub_140225ef0
    Replacement::draft(0x140225fa0, "r_140225fa0", fn_140225fa0 as *const ()), // sub_140225fa0
    Replacement::draft(0x140226200, "r_140226200", fn_140226200 as *const ()), // sub_140226200
    Replacement::draft(0x1402262b0, "r_1402262b0", fn_1402262b0 as *const ()), // sub_1402262b0
    Replacement::draft(0x140226330, "r_140226330", fn_140226330 as *const ()), // sub_140226330
    Replacement::draft(0x1402264d0, "r_1402264d0", tpl_855::<0x14021f300, 0x1425371a0, 0x142533200, 0x140220ff0, 0x1402227b0> as *const ()), // sub_1402264d0
    Replacement::draft(0x140226630, "r_140226630", tpl_855::<0x14021f370, 0x1425371a0, 0x142533200, 0x1402211e0, 0x140222880> as *const ()), // sub_140226630
    Replacement::draft(0x14022f9d0, "r_14022f9d0", fn_14022f9d0 as *const ()), // sub_14022f9d0
    Replacement::draft_throws(0x14022fc40, "r_14022fc40", fn_14022fc40 as *const (), Throws::Discard), // sub_14022fc40
    Replacement::draft(0x14022ff50, "r_14022ff50", tpl_61071::<0x143c8efac, 0x143c8e9b0, 0x142489ee0, 0x143c8de68, 0x141884130> as *const ()), // sub_14022ff50
    Replacement::draft(0x140230140, "r_140230140", fn_140230140 as *const ()), // sub_140230140
    Replacement::draft(0x1402303d0, "r_1402303d0", tpl_61071::<0x143c8e6c8, 0x143c8d160, 0x142489ee0, 0x143c8eb70, 0x1418837b0> as *const ()), // sub_1402303d0
    Replacement::draft(0x1402305c0, "r_1402305c0", fn_1402305c0 as *const ()), // sub_1402305c0
    Replacement::draft_throws(0x140230830, "r_140230830", tpl_61074::<0x143c8ec18, 0x143c8d08c, 0x143c8e678, 0x143c8d910, 0x143c8ceec, 0x1438440f8, 0x142489ee0, 0x143c8edbc, 0x143c8d120, 0x141882e90> as *const (), Throws::Discard), // sub_140230830
    Replacement::draft(0x140230b50, "r_140230b50", fn_140230b50 as *const ()), // sub_140230b50
    Replacement::draft_throws(0x140230d00, "r_140230d00", fn_140230d00 as *const (), Throws::Discard), // sub_140230d00
    Replacement::draft(0x140232fb0, "r_140232fb0", fn_140232fb0 as *const ()), // sub_140232fb0
    Replacement::draft(0x140233370, "r_140233370", fn_140233370 as *const ()), // sub_140233370
    Replacement::draft(0x140233990, "r_140233990", fn_140233990 as *const ()), // sub_140233990
    Replacement::draft(0x140233aa0, "r_140233aa0", fn_140233aa0 as *const ()), // sub_140233aa0
    Replacement::draft(0x140240190, "r_140240190", tpl_1131::<0x142407c80, 0x143c8f6e4, 0x142489ee0, 0x14188a780> as *const ()), // sub_140240190
    Replacement::draft(0x1402402f0, "r_1402402f0", fn_1402402f0 as *const ()), // sub_1402402f0
    Replacement::draft(0x140245280, "r_140245280", fn_140245280 as *const ()), // sub_140245280
    Replacement::draft(0x140245670, "r_140245670", tpl_1130::<0x142407c80, 0x14188bca0> as *const ()), // sub_140245670
    Replacement::draft(0x140245780, "r_140245780", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140243370> as *const ()), // sub_140245780
    Replacement::draft(0x140245aa0, "r_140245aa0", fn_140245aa0 as *const ()), // sub_140245aa0
    Replacement::draft(0x140245d80, "r_140245d80", fn_140245d80 as *const ()), // sub_140245d80
    Replacement::draft(0x140249970, "r_140249970", tpl_1131::<0x142407c80, 0x143c8f0d0, 0x142489ee0, 0x14188f9d0> as *const ()), // sub_140249970
    Replacement::draft(0x140249ad0, "r_140249ad0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140247f70> as *const ()), // sub_140249ad0
    Replacement::draft(0x140249e00, "r_140249e00", tpl_1472::<0x143c8fc24, 0x143c8ff00, 0x141834a78, 0x143c8f4d4, 0x143c8ff58, 0x143c8d160, 0x143c8fab8, 0x143c8d120, 0x143c8f4d0, 0x143c8fe90, 0x143c8ff20, 0x143c8ff40, 0x143c8ff50, 0x141834a0c> as *const ()), // sub_140249e00
    Replacement::draft(0x14024a7d0, "r_14024a7d0", fn_14024a7d0 as *const ()), // sub_14024a7d0
    Replacement::draft(0x14024a850, "r_14024a850", fn_14024a850 as *const ()), // sub_14024a850
    Replacement::draft(0x14026ca60, "r_14026ca60", fn_14026ca60 as *const ()), // sub_14026ca60
    Replacement::draft(0x14026d520, "r_14026d520", fn_14026d520 as *const ()), // sub_14026d520
    Replacement::draft(0x140273f30, "r_140273f30", tpl_1856::<0xa4, 0xac, 0x98, 0xa0> as *const ()), // r::GameGlobalParameter<m::Vector3Template<float>>::r::GameGlobalParameter<m::Vector3Template<float>>::vfunc_4
    Replacement::draft(0x140273f50, "r_140273f50", tpl_1857::<0xa4, 0xc8, 0xa8, 0xcc, 0xac, 0xd0> as *const ()), // r::GameGlobalParameter<m::Vector3Template<float>>::r::GameGlobalParameter<m::Vector3Template<float>>::vfunc_3
    Replacement::draft(0x140273fa0, "r_140273fa0", tpl_1856::<0xc8, 0xd0, 0xa4, 0xac> as *const ()), // r::GameGlobalParameter<m::Vector3Template<float>>::r::GameGlobalParameter<m::Vector3Template<float>>::vfunc_2
    Replacement::draft(0x140273fc0, "r_140273fc0", fn_140273fc0 as *const ()), // r::GameGlobalParameter<m::Vector3Template<float>>::r::GameGlobalParameter<m::Vector3Template<float>>::vfunc_1
    Replacement::draft(0x140274070, "r_140274070", fn_140274070 as *const ()), // r::GameGlobalParameter<m::Vector3Template<float>>::r::GameGlobalParameter<m::Vector3Template<float>>::vfunc_0
    Replacement::draft(0x140276570, "r_140276570", fn_140276570 as *const ()), // sub_140276570
    Replacement::draft_throws(0x1402765d0, "r_1402765d0", fn_1402765d0 as *const (), Throws::Discard), // sub_1402765d0
    Replacement::draft_throws(0x140276810, "r_140276810", fn_140276810 as *const (), Throws::Discard), // sub_140276810
    Replacement::draft(0x140276a70, "r_140276a70", fn_140276a70 as *const ()), // sub_140276a70
    Replacement::draft_throws(0x140276f30, "r_140276f30", fn_140276f30 as *const (), Throws::Discard), // sub_140276f30
    Replacement::draft(0x140278e20, "r_140278e20", tpl_1130::<0x142407c80, 0x1418c7000> as *const ()), // sub_140278e20
    Replacement::draft(0x140278f30, "r_140278f30", tpl_1885::<0x143c91a40, 0x143c918e0, 0x143c8d120, 0x143c8d160, 0x143c8e840, 0x143c8e65c, 0x143c8f4d0, 0x143c919a8, 0x143c8de68, 0x143c8eb70, 0x143c8f4d4, 0x143c91900, 0x143c91920, 0x143c91940, 0x141834a0c, 0x141834a78> as *const ()), // sub_140278f30
    Replacement::draft_throws(0x14027c6a0, "r_14027c6a0", fn_14027c6a0 as *const (), Throws::Discard), // sub_14027c6a0
    Replacement::draft(0x140321fc0, "r_140321fc0", fn_140321fc0 as *const ()), // sub_140321fc0
    Replacement::draft_throws(0x140322020, "r_140322020", fn_140322020 as *const (), Throws::Caught), // sub_140322020
    Replacement::draft_throws(0x1403220c0, "r_1403220c0", tpl_2826::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c1a50, 0x1426d0f32, 0x1430b20f0, 0x1401f2430, 0x20, 0x1401eb150, 0x14240ab10> as *const (), Throws::Discard), // sub_1403220c0
    Replacement::draft_throws(0x140322180, "r_140322180", tpl_2827::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c1a50> as *const (), Throws::Discard), // sub_140322180
    Replacement::draft_throws(0x1403221e0, "r_1403221e0", tpl_2828::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c1a50> as *const (), Throws::Discard), // sub_1403221e0
    Replacement::draft_throws(0x140322250, "r_140322250", fn_140322250 as *const (), Throws::Discard), // sub_140322250
    Replacement::draft_throws(0x140322310, "r_140322310", tpl_2830::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2280, 0x1426d0f32, 0x1430b20f0, 0x1401f2430, 0x20, 0x1401eb150, 0x14240ab10> as *const (), Throws::Discard), // sub_140322310
    Replacement::draft_throws(0x1403223e0, "r_1403223e0", fn_1403223e0 as *const (), Throws::Discard), // sub_1403223e0
    Replacement::draft_throws(0x140322440, "r_140322440", tpl_2832::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2280> as *const (), Throws::Discard), // sub_140322440
    Replacement::draft_throws(0x1403224c0, "r_1403224c0", fn_1403224c0 as *const (), Throws::Discard), // sub_1403224c0
    Replacement::draft_throws(0x140322510, "r_140322510", tpl_2834::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2280> as *const (), Throws::Discard), // sub_140322510
    Replacement::draft_throws(0x140322560, "r_140322560", crate::shards::subsystems::fn_140322560 as *const (), Throws::Caught), // sub_140322560
    Replacement::draft_throws(0x1403226a0, "r_1403226a0", crate::shards::subsystems::fn_1403226a0 as *const (), Throws::Caught), // sub_1403226a0
    Replacement::draft_throws(0x1403227e0, "r_1403227e0", crate::shards::subsystems::fn_1403227e0 as *const (), Throws::Caught), // sub_1403227e0
    Replacement::draft_throws(0x140322920, "r_140322920", crate::shards::subsystems::fn_140322920 as *const (), Throws::Caught), // sub_140322920
    Replacement::draft_throws(0x140322a60, "r_140322a60", crate::shards::subsystems::fn_140322a60 as *const (), Throws::Caught), // sub_140322a60
    Replacement::draft_throws(0x140322ba0, "r_140322ba0", crate::shards::subsystems::fn_140322ba0 as *const (), Throws::Caught), // sub_140322ba0
    Replacement::draft_throws(0x140322ce0, "r_140322ce0", crate::shards::subsystems::fn_140322ce0 as *const (), Throws::Caught), // sub_140322ce0
    Replacement::draft_throws(0x140322e20, "r_140322e20", crate::shards::subsystems::fn_140322e20 as *const (), Throws::Caught), // sub_140322e20
    Replacement::draft_throws(0x140322f60, "r_140322f60", crate::shards::subsystems::fn_140322f60 as *const (), Throws::Caught), // sub_140322f60
    Replacement::draft_throws(0x1403230a0, "r_1403230a0", crate::shards::subsystems::fn_1403230a0 as *const (), Throws::Caught), // sub_1403230a0
    Replacement::draft_throws(0x1403231e0, "r_1403231e0", crate::shards::subsystems::fn_1403231e0 as *const (), Throws::Caught), // sub_1403231e0
    Replacement::draft_throws(0x140323320, "r_140323320", crate::shards::subsystems::fn_140323320 as *const (), Throws::Caught), // sub_140323320
    Replacement::draft_throws(0x140323460, "r_140323460", crate::shards::subsystems::fn_140323460 as *const (), Throws::Caught), // sub_140323460
    Replacement::draft_throws(0x140323650, "r_140323650", crate::shards::subsystems::fn_140323650 as *const (), Throws::Caught), // sub_140323650
    Replacement::draft_throws(0x140323840, "r_140323840", crate::shards::subsystems::fn_140323840 as *const (), Throws::Caught), // sub_140323840
    Replacement::draft_throws(0x140323a30, "r_140323a30", fn_140323a30 as *const (), Throws::Caught), // sub_140323a30
    Replacement::draft_throws(0x140323ca0, "r_140323ca0", crate::shards::subsystems::fn_140323ca0 as *const (), Throws::Caught), // sub_140323ca0
    Replacement::draft_throws(0x140323de0, "r_140323de0", crate::shards::subsystems::fn_140323de0 as *const (), Throws::Caught), // sub_140323de0
    Replacement::draft_throws(0x140323f20, "r_140323f20", fn_140323f20 as *const (), Throws::Caught), // sub_140323f20
    Replacement::draft_throws(0x140324180, "r_140324180", tpl_2830::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2dc0, 0x1426d0f32, 0x1430b20f0, 0x1401f2430, 0x20, 0x1401eb150, 0x14240ab10> as *const (), Throws::Discard), // sub_140324180
    Replacement::draft_throws(0x140324230, "r_140324230", crate::shards::subsystems::fn_140324230 as *const (), Throws::Caught), // sub_140324230
    Replacement::draft_throws(0x140324370, "r_140324370", crate::shards::subsystems::fn_140324370 as *const (), Throws::Caught), // sub_140324370
    Replacement::draft_throws(0x1403244b0, "r_1403244b0", crate::shards::subsystems::fn_1403244b0 as *const (), Throws::Caught), // sub_1403244b0
    Replacement::draft_throws(0x140324700, "r_140324700", tpl_2832::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2dc0> as *const (), Throws::Discard), // sub_140324700
    Replacement::draft_throws(0x140324760, "r_140324760", crate::shards::subsystems::fn_140324760 as *const (), Throws::Caught), // sub_140324760
    Replacement::draft_throws(0x1403248a0, "r_1403248a0", crate::shards::subsystems::fn_1403248a0 as *const (), Throws::Caught), // sub_1403248a0
    Replacement::draft_throws(0x1403249e0, "r_1403249e0", crate::shards::subsystems::fn_1403249e0 as *const (), Throws::Caught), // sub_1403249e0
    Replacement::draft_throws(0x140324c30, "r_140324c30", tpl_2834::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2dc0> as *const (), Throws::Discard), // sub_140324c30
    Replacement::draft_throws(0x140324c80, "r_140324c80", crate::shards::subsystems::fn_140324c80 as *const (), Throws::Caught), // sub_140324c80
    Replacement::draft_throws(0x140324e70, "r_140324e70", crate::shards::subsystems::fn_140324e70 as *const (), Throws::Caught), // sub_140324e70
    Replacement::draft_throws(0x140325060, "r_140325060", fn_140325060 as *const (), Throws::Caught), // sub_140325060
    Replacement::draft_throws(0x1403252f0, "r_1403252f0", fn_1403252f0 as *const (), Throws::Caught), // sub_1403252f0
    Replacement::draft_throws(0x140325680, "r_140325680", fn_140325680 as *const (), Throws::Caught), // sub_140325680
    Replacement::draft_throws(0x140325a10, "r_140325a10", fn_140325a10 as *const (), Throws::Caught), // sub_140325a10
    Replacement::draft_throws(0x140325da0, "r_140325da0", fn_140325da0 as *const (), Throws::Caught), // sub_140325da0
    Replacement::draft_throws(0x140326170, "r_140326170", fn_140326170 as *const (), Throws::Caught), // sub_140326170
    Replacement::draft_throws(0x1403264e0, "r_1403264e0", crate::shards::subsystems::fn_1403264e0 as *const (), Throws::Caught), // sub_1403264e0
    Replacement::draft_throws(0x140326750, "r_140326750", fn_140326750 as *const (), Throws::Caught), // sub_140326750
    Replacement::draft_throws(0x140326ae0, "r_140326ae0", fn_140326ae0 as *const (), Throws::Caught), // sub_140326ae0
    Replacement::draft_throws(0x140326e30, "r_140326e30", fn_140326e30 as *const (), Throws::Caught), // sub_140326e30
    Replacement::draft_throws(0x1403270a0, "r_1403270a0", crate::shards::subsystems::fn_1403270a0 as *const (), Throws::Caught), // sub_1403270a0
    Replacement::draft_throws(0x140327310, "r_140327310", fn_140327310 as *const (), Throws::Caught), // sub_140327310
    Replacement::draft_throws(0x1403276a0, "r_1403276a0", fn_1403276a0 as *const (), Throws::Caught), // sub_1403276a0
    Replacement::draft_throws(0x140327a30, "r_140327a30", crate::shards::subsystems::fn_140327a30 as *const (), Throws::Caught), // sub_140327a30
    Replacement::draft_throws(0x140327dc0, "r_140327dc0", crate::shards::subsystems::fn_140327dc0 as *const (), Throws::Caught), // sub_140327dc0
    Replacement::draft_throws(0x140328140, "r_140328140", crate::shards::subsystems::fn_140328140 as *const (), Throws::Caught), // sub_140328140
    Replacement::draft_throws(0x1403284c0, "r_1403284c0", crate::shards::subsystems::fn_1403284c0 as *const (), Throws::Caught), // sub_1403284c0
    Replacement::draft_throws(0x140328890, "r_140328890", crate::shards::subsystems::fn_140328890 as *const (), Throws::Caught), // sub_140328890
    Replacement::draft_throws(0x140328c00, "r_140328c00", crate::shards::subsystems::fn_140328c00 as *const (), Throws::Caught), // sub_140328c00
    Replacement::draft_throws(0x140328e60, "r_140328e60", crate::shards::subsystems::fn_140328e60 as *const (), Throws::Caught), // sub_140328e60
    Replacement::draft_throws(0x1403291f0, "r_1403291f0", crate::shards::subsystems::fn_1403291f0 as *const (), Throws::Caught), // sub_1403291f0
    Replacement::draft_throws(0x140329540, "r_140329540", crate::shards::subsystems::fn_140329540 as *const (), Throws::Caught), // sub_140329540
    Replacement::draft_throws(0x1403297a0, "r_1403297a0", crate::shards::subsystems::fn_1403297a0 as *const (), Throws::Caught), // sub_1403297a0
    Replacement::draft_throws(0x140329a00, "r_140329a00", crate::shards::subsystems::fn_140329a00 as *const (), Throws::Caught), // sub_140329a00
    Replacement::draft_throws(0x140329d80, "r_140329d80", crate::shards::subsystems::fn_140329d80 as *const (), Throws::Caught), // sub_140329d80
    Replacement::draft_throws(0x14032a100, "r_14032a100", crate::shards::subsystems::fn_14032a100 as *const (), Throws::Caught), // sub_14032a100
    Replacement::draft_throws(0x14032a490, "r_14032a490", crate::shards::subsystems::fn_14032a490 as *const (), Throws::Caught), // sub_14032a490
    Replacement::draft_throws(0x14032a810, "r_14032a810", crate::shards::subsystems::fn_14032a810 as *const (), Throws::Caught), // sub_14032a810
    Replacement::draft_throws(0x14032ab90, "r_14032ab90", crate::shards::subsystems::fn_14032ab90 as *const (), Throws::Caught), // sub_14032ab90
    Replacement::draft_throws(0x14032af60, "r_14032af60", crate::shards::subsystems::fn_14032af60 as *const (), Throws::Caught), // sub_14032af60
    Replacement::draft_throws(0x14032b2d0, "r_14032b2d0", crate::shards::subsystems::fn_14032b2d0 as *const (), Throws::Caught), // sub_14032b2d0
    Replacement::draft_throws(0x14032b530, "r_14032b530", crate::shards::subsystems::fn_14032b530 as *const (), Throws::Caught), // sub_14032b530
    Replacement::draft_throws(0x14032b8c0, "r_14032b8c0", crate::shards::subsystems::fn_14032b8c0 as *const (), Throws::Caught), // sub_14032b8c0
    Replacement::draft_throws(0x14032bc10, "r_14032bc10", crate::shards::subsystems::fn_14032bc10 as *const (), Throws::Caught), // sub_14032bc10
    Replacement::draft_throws(0x14032be70, "r_14032be70", crate::shards::subsystems::fn_14032be70 as *const (), Throws::Caught), // sub_14032be70
    Replacement::draft_throws(0x14032c0d0, "r_14032c0d0", crate::shards::subsystems::fn_14032c0d0 as *const (), Throws::Caught), // sub_14032c0d0
    Replacement::draft_throws(0x14032c450, "r_14032c450", crate::shards::subsystems::fn_14032c450 as *const (), Throws::Caught), // sub_14032c450
    Replacement::draft_throws(0x14032c7d0, "r_14032c7d0", crate::shards::subsystems::fn_14032c7d0 as *const (), Throws::Caught), // sub_14032c7d0
    Replacement::draft_throws(0x14032c9c0, "r_14032c9c0", crate::shards::subsystems::fn_14032c9c0 as *const (), Throws::Caught), // sub_14032c9c0
    Replacement::draft_throws(0x14032cbb0, "r_14032cbb0", crate::shards::subsystems::fn_14032cbb0 as *const (), Throws::Caught), // sub_14032cbb0
    Replacement::draft_throws(0x14032cda0, "r_14032cda0", crate::shards::subsystems::fn_14032cda0 as *const (), Throws::Caught), // sub_14032cda0
    Replacement::draft_throws(0x14032cf90, "r_14032cf90", crate::shards::subsystems::fn_14032cf90 as *const (), Throws::Caught), // sub_14032cf90
    Replacement::draft_throws(0x14032d180, "r_14032d180", crate::shards::subsystems::fn_14032d180 as *const (), Throws::Caught), // sub_14032d180
    Replacement::draft_throws(0x14032d370, "r_14032d370", crate::shards::subsystems::fn_14032d370 as *const (), Throws::Caught), // sub_14032d370
    Replacement::draft_throws(0x14032d560, "r_14032d560", crate::shards::subsystems::fn_14032d560 as *const (), Throws::Caught), // sub_14032d560
    Replacement::draft_throws(0x14032d750, "r_14032d750", crate::shards::subsystems::fn_14032d750 as *const (), Throws::Caught), // sub_14032d750
    Replacement::draft_throws(0x14032d940, "r_14032d940", crate::shards::subsystems::fn_14032d940 as *const (), Throws::Caught), // sub_14032d940
    Replacement::draft_throws(0x14032db30, "r_14032db30", crate::shards::subsystems::fn_14032db30 as *const (), Throws::Caught), // sub_14032db30
    Replacement::draft_throws(0x14032dd20, "r_14032dd20", crate::shards::subsystems::fn_14032dd20 as *const (), Throws::Caught), // sub_14032dd20
    Replacement::draft_throws(0x14032df10, "r_14032df10", fn_14032df10 as *const (), Throws::Caught), // sub_14032df10
    Replacement::draft_throws(0x14032e1a0, "r_14032e1a0", crate::shards::subsystems::fn_14032e1a0 as *const (), Throws::Caught), // sub_14032e1a0
    Replacement::draft_throws(0x14032e430, "r_14032e430", crate::shards::subsystems::fn_14032e430 as *const (), Throws::Caught), // sub_14032e430
    Replacement::draft_throws(0x14032e6c0, "r_14032e6c0", fn_14032e6c0 as *const (), Throws::Caught), // sub_14032e6c0
    Replacement::draft_throws(0x14032e930, "r_14032e930", crate::shards::subsystems::fn_14032e930 as *const (), Throws::Caught), // sub_14032e930
    Replacement::draft_throws(0x14032eaf0, "r_14032eaf0", crate::shards::subsystems::fn_14032eaf0 as *const (), Throws::Caught), // sub_14032eaf0
    Replacement::draft_throws(0x14032ecb0, "r_14032ecb0", crate::shards::subsystems::fn_14032ecb0 as *const (), Throws::Caught), // sub_14032ecb0
    Replacement::draft_throws(0x14032ee70, "r_14032ee70", fn_14032ee70 as *const (), Throws::Discard), // sub_14032ee70
    Replacement::draft_throws(0x14032f020, "r_14032f020", crate::shards::subsystems::fn_14032f020 as *const (), Throws::Caught), // sub_14032f020
    Replacement::draft_throws(0x14032f120, "r_14032f120", crate::shards::subsystems::fn_14032f120 as *const (), Throws::Caught), // sub_14032f120
    Replacement::draft_throws(0x14032f220, "r_14032f220", crate::shards::subsystems::fn_14032f220 as *const (), Throws::Caught), // sub_14032f220
    Replacement::draft_throws(0x14032f320, "r_14032f320", fn_14032f320 as *const (), Throws::Discard), // sub_14032f320
    Replacement::draft(0x14032f510, "r_14032f510", fn_14032f510 as *const ()), // sub_14032f510
    Replacement::draft(0x14032f580, "r_14032f580", tpl_2871::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430b67a8, 0x8, 0x9, 0x11, 0x1401f3d90, 0x1430b8698> as *const ()), // sub_14032f580
    Replacement::draft(0x1403303f0, "r_1403303f0", tpl_2872::<0x14240aa00, 0x1402251c0> as *const ()), // sub_1403303f0
    Replacement::draft(0x140330600, "r_140330600", tpl_2872::<0x14240a730, 0x140223d60> as *const ()), // sub_140330600
    Replacement::draft(0x140330810, "r_140330810", fn_140330810 as *const ()), // sub_140330810
    Replacement::draft(0x140330860, "r_140330860", tpl_2874::<0x8, 0x30, 0x14028d900, 0x14240ab10> as *const ()), // sub_140330860
    Replacement::draft(0x1403308a0, "r_1403308a0", tpl_2874::<0x8, 0x30, 0x1401fc280, 0x14240ab10> as *const ()), // sub_1403308a0
    Replacement::draft(0x1403308e0, "r_1403308e0", fn_1403308e0 as *const ()), // sub_1403308e0
    Replacement::draft(0x140330920, "r_140330920", fn_140330920 as *const ()), // sub_140330920
    Replacement::draft(0x140330a20, "r_140330a20", fn_140330a20 as *const ()), // sub_140330a20
    Replacement::draft(0x140330bc0, "r_140330bc0", fn_140330bc0 as *const ()), // sub_140330bc0
    Replacement::draft(0x140330d50, "r_140330d50", tpl_2879::<0x1401f3d00, 0x8, 0x1402bf7c0, 0x1430b67c0, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise<r::Expected<bigfish::env::SaveGameWrite::SaveResult,platform::
    Replacement::draft(0x140330da0, "r_140330da0", tpl_2880::<0x1401f2bc0, 0xc, 0x8, 0x14028d810, 0x14183acc8> as *const ()), // sub_140330da0
    Replacement::draft(0x140330e60, "r_140330e60", fn_140330e60 as *const ()), // sub_140330e60
    Replacement::draft(0x140330f20, "r_140330f20", fn_140330f20 as *const ()), // sub_140330f20
    Replacement::draft(0x140330fb0, "r_140330fb0", fn_140330fb0 as *const ()), // sub_140330fb0
    Replacement::draft(0x140331050, "r_140331050", fn_140331050 as *const ()), // sub_140331050
    Replacement::draft(0x1403312a0, "r_1403312a0", tpl_2885::<0x14028cf90, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14028d970, 0x142442de0> as *const ()), // sub_1403312a0
    Replacement::draft_throws(0x1403313e0, "r_1403313e0", crate::shards::subsystems::fn_1403313e0 as *const (), Throws::Caught), // sub_1403313e0
    Replacement::draft_throws(0x140331850, "r_140331850", crate::shards::subsystems::fn_140331850 as *const (), Throws::Caught), // sub_140331850
    Replacement::draft_throws(0x140331cc0, "r_140331cc0", crate::shards::subsystems::fn_140331cc0 as *const (), Throws::Caught), // sub_140331cc0
    Replacement::draft_throws(0x140332130, "r_140332130", fn_140332130 as *const (), Throws::Caught), // sub_140332130
    Replacement::draft_throws(0x140332830, "r_140332830", crate::shards::subsystems::fn_140332830 as *const (), Throws::Caught), // sub_140332830
    Replacement::draft_throws(0x140332c60, "r_140332c60", crate::shards::subsystems::fn_140332c60 as *const (), Throws::Caught), // sub_140332c60
    Replacement::draft_throws(0x140333090, "r_140333090", crate::shards::subsystems::fn_140333090 as *const (), Throws::Caught), // sub_140333090
    Replacement::draft_throws(0x1403334c0, "r_1403334c0", fn_1403334c0 as *const (), Throws::Caught), // sub_1403334c0
    Replacement::draft_throws(0x140333b40, "r_140333b40", crate::shards::subsystems::fn_140333b40 as *const (), Throws::Caught), // sub_140333b40
    Replacement::draft_throws(0x140333e90, "r_140333e90", crate::shards::subsystems::fn_140333e90 as *const (), Throws::Caught), // sub_140333e90
    Replacement::draft_throws(0x1403341e0, "r_1403341e0", crate::shards::subsystems::fn_1403341e0 as *const (), Throws::Caught), // sub_1403341e0
    Replacement::draft_throws(0x140334530, "r_140334530", fn_140334530 as *const (), Throws::Caught), // sub_140334530
    Replacement::draft(0x140334a60, "r_140334a60", fn_140334a60 as *const ()), // sub_140334a60
    Replacement::draft_throws(0x140334a80, "r_140334a80", crate::shards::subsystems::fn_140334a80 as *const (), Throws::Caught), // sub_140334a80
    Replacement::draft_throws(0x140334ef0, "r_140334ef0", crate::shards::subsystems::fn_140334ef0 as *const (), Throws::Caught), // sub_140334ef0
    Replacement::draft_throws(0x140335360, "r_140335360", fn_140335360 as *const (), Throws::Caught), // sub_140335360
    Replacement::draft_throws(0x140335920, "r_140335920", crate::shards::subsystems::fn_140335920 as *const (), Throws::Caught), // sub_140335920
    Replacement::draft_throws(0x140335d50, "r_140335d50", crate::shards::subsystems::fn_140335d50 as *const (), Throws::Caught), // sub_140335d50
    Replacement::draft_throws(0x140336180, "r_140336180", fn_140336180 as *const (), Throws::Caught), // sub_140336180
    Replacement::draft_throws(0x140336700, "r_140336700", crate::shards::subsystems::fn_140336700 as *const (), Throws::Caught), // sub_140336700
    Replacement::draft_throws(0x140336a50, "r_140336a50", crate::shards::subsystems::fn_140336a50 as *const (), Throws::Caught), // sub_140336a50
    Replacement::draft_throws(0x140336da0, "r_140336da0", crate::shards::subsystems::fn_140336da0 as *const (), Throws::Caught), // sub_140336da0
    Replacement::draft_throws(0x1403370f0, "r_1403370f0", fn_1403370f0 as *const (), Throws::Caught), // sub_1403370f0
    Replacement::draft_throws(0x140337780, "r_140337780", fn_140337780 as *const (), Throws::Caught), // sub_140337780
    Replacement::draft_throws(0x140337e10, "r_140337e10", fn_140337e10 as *const (), Throws::Caught), // sub_140337e10
    Replacement::draft_throws(0x140338480, "r_140338480", fn_140338480 as *const (), Throws::Caught), // sub_140338480
    Replacement::draft_throws(0x140338b50, "r_140338b50", fn_140338b50 as *const (), Throws::Caught), // sub_140338b50
    Replacement::draft_throws(0x140339200, "r_140339200", crate::shards::subsystems::fn_140339200 as *const (), Throws::Caught), // sub_140339200
    Replacement::draft_throws(0x140339780, "r_140339780", fn_140339780 as *const (), Throws::Caught), // sub_140339780
    Replacement::draft_throws(0x140339e10, "r_140339e10", fn_140339e10 as *const (), Throws::Caught), // sub_140339e10
    Replacement::draft_throws(0x14033a4a0, "r_14033a4a0", fn_14033a4a0 as *const (), Throws::Caught), // sub_14033a4a0
    Replacement::draft_throws(0x14033aa20, "r_14033aa20", crate::shards::subsystems::fn_14033aa20 as *const (), Throws::Caught), // sub_14033aa20
    Replacement::draft_throws(0x14033afa0, "r_14033afa0", fn_14033afa0 as *const (), Throws::Caught), // sub_14033afa0
    Replacement::draft_throws(0x14033b610, "r_14033b610", fn_14033b610 as *const (), Throws::Caught), // sub_14033b610
    Replacement::draft_throws(0x14033bc80, "r_14033bc80", fn_14033bc80 as *const (), Throws::Caught), // sub_14033bc80
    Replacement::draft_throws(0x14033c2d0, "r_14033c2d0", fn_14033c2d0 as *const (), Throws::Caught), // sub_14033c2d0
    Replacement::draft_throws(0x14033c910, "r_14033c910", fn_14033c910 as *const (), Throws::Caught), // sub_14033c910
    Replacement::draft_throws(0x14033cf50, "r_14033cf50", fn_14033cf50 as *const (), Throws::Caught), // sub_14033cf50
    Replacement::draft_throws(0x14033d5e0, "r_14033d5e0", fn_14033d5e0 as *const (), Throws::Caught), // sub_14033d5e0
    Replacement::draft_throws(0x14033dc60, "r_14033dc60", crate::shards::subsystems::fn_14033dc60 as *const (), Throws::Caught), // sub_14033dc60
    Replacement::draft_throws(0x14033e1b0, "r_14033e1b0", fn_14033e1b0 as *const (), Throws::Caught), // sub_14033e1b0
    Replacement::draft_throws(0x14033e800, "r_14033e800", fn_14033e800 as *const (), Throws::Caught), // sub_14033e800
    Replacement::draft_throws(0x14033ee50, "r_14033ee50", fn_14033ee50 as *const (), Throws::Caught), // sub_14033ee50
    Replacement::draft_throws(0x14033f3a0, "r_14033f3a0", crate::shards::subsystems::fn_14033f3a0 as *const (), Throws::Caught), // sub_14033f3a0
    Replacement::draft_throws(0x14033f8f0, "r_14033f8f0", fn_14033f8f0 as *const (), Throws::Caught), // sub_14033f8f0
    Replacement::draft_throws(0x14033ff30, "r_14033ff30", fn_14033ff30 as *const (), Throws::Caught), // sub_14033ff30
    Replacement::draft_throws(0x140340570, "r_140340570", crate::shards::subsystems::fn_140340570 as *const (), Throws::Caught), // sub_140340570
    Replacement::draft_throws(0x1403408c0, "r_1403408c0", crate::shards::subsystems::fn_1403408c0 as *const (), Throws::Caught), // sub_1403408c0
    Replacement::draft_throws(0x140340c10, "r_140340c10", crate::shards::subsystems::fn_140340c10 as *const (), Throws::Caught), // sub_140340c10
    Replacement::draft_throws(0x140340f60, "r_140340f60", crate::shards::subsystems::fn_140340f60 as *const (), Throws::Caught), // sub_140340f60
    Replacement::draft_throws(0x1403412b0, "r_1403412b0", crate::shards::subsystems::fn_1403412b0 as *const (), Throws::Caught), // sub_1403412b0
    Replacement::draft_throws(0x140341600, "r_140341600", crate::shards::subsystems::fn_140341600 as *const (), Throws::Caught), // sub_140341600
    Replacement::draft_throws(0x140341950, "r_140341950", crate::shards::subsystems::fn_140341950 as *const (), Throws::Caught), // sub_140341950
    Replacement::draft_throws(0x140341ca0, "r_140341ca0", crate::shards::subsystems::fn_140341ca0 as *const (), Throws::Caught), // sub_140341ca0
    Replacement::draft_throws(0x140341ff0, "r_140341ff0", crate::shards::subsystems::fn_140341ff0 as *const (), Throws::Caught), // sub_140341ff0
    Replacement::draft_throws(0x140342340, "r_140342340", crate::shards::subsystems::fn_140342340 as *const (), Throws::Caught), // sub_140342340
    Replacement::draft_throws(0x140342690, "r_140342690", crate::shards::subsystems::fn_140342690 as *const (), Throws::Caught), // sub_140342690
    Replacement::draft_throws(0x1403429e0, "r_1403429e0", crate::shards::subsystems::fn_1403429e0 as *const (), Throws::Caught), // sub_1403429e0
    Replacement::draft_throws(0x140342d30, "r_140342d30", fn_140342d30 as *const (), Throws::Caught), // sub_140342d30
    Replacement::draft_throws(0x1403432e0, "r_1403432e0", fn_1403432e0 as *const (), Throws::Caught), // sub_1403432e0
    Replacement::draft_throws(0x140343860, "r_140343860", crate::shards::subsystems::fn_140343860 as *const (), Throws::Caught), // sub_140343860
    Replacement::draft(0x14034b170, "r_14034b170", fn_14034b170 as *const ()), // sub_14034b170
    Replacement::draft(0x14034b2a0, "r_14034b2a0", fn_14034b2a0 as *const ()), // sub_14034b2a0
    Replacement::draft(0x14034b320, "r_14034b320", fn_14034b320 as *const ()), // sub_14034b320
    Replacement::draft_throws(0x14034b3f0, "r_14034b3f0", crate::shards::subsystems::fn_14034b3f0 as *const (), Throws::Caught), // sub_14034b3f0
    Replacement::draft_throws(0x14034b550, "r_14034b550", crate::shards::subsystems::fn_14034b550 as *const (), Throws::Caught), // sub_14034b550
    Replacement::draft_throws(0x14034b6c0, "r_14034b6c0", fn_14034b6c0 as *const (), Throws::Caught), // sub_14034b6c0
    Replacement::draft(0x14034ba10, "r_14034ba10", fn_14034ba10 as *const ()), // sub_14034ba10
    Replacement::draft(0x14034baa0, "r_14034baa0", fn_14034baa0 as *const ()), // sub_14034baa0
    Replacement::draft_throws(0x14034bb30, "r_14034bb30", crate::shards::subsystems::fn_14034bb30 as *const (), Throws::Caught), // sub_14034bb30
    Replacement::draft_throws(0x14034bc90, "r_14034bc90", crate::shards::subsystems::fn_14034bc90 as *const (), Throws::Caught), // sub_14034bc90
    Replacement::draft_throws(0x14034be00, "r_14034be00", fn_14034be00 as *const (), Throws::Caught), // sub_14034be00
    Replacement::draft(0x14034c150, "r_14034c150", tpl_1842::<0x1401e7c10, 0x143c8d120, 0x142489ee0> as *const ()), // sub_14034c150
    Replacement::draft_throws(0x14035a280, "r_14035a280", tpl_1330::<0x8, 0x14035a940, 0x143c94108, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14035a280
    Replacement::draft_throws(0x14035a460, "r_14035a460", tpl_62466::<0x143c9418c, 0x143c8d08c, 0x143c94194, 0x142489ee0, 0x143c94274, 0x1418fc2b0> as *const (), Throws::Discard), // sub_14035a460
    Replacement::draft_throws(0x14035a690, "r_14035a690", fn_14035a690 as *const (), Throws::Discard), // sub_14035a690
    Replacement::draft(0x14037d830, "r_14037d830", fn_14037d830 as *const ()), // sub_14037d830
    Replacement::draft(0x14037da40, "r_14037da40", fn_14037da40 as *const ()), // sub_14037da40
    Replacement::draft(0x14037da60, "r_14037da60", fn_14037da60 as *const ()), // sub_14037da60
    Replacement::draft(0x14037dce0, "r_14037dce0", fn_14037dce0 as *const ()), // sub_14037dce0
    Replacement::draft(0x14037dd80, "r_14037dd80", fn_14037dd80 as *const ()), // sub_14037dd80
    Replacement::draft(0x14037fa50, "r_14037fa50", tpl_3204::<0x143c8f4d8, 0x142407c80, 0x143c8e710, 0x142489ee0, 0x143c96594, 0x143c9659c, 0x14190baf0> as *const ()), // sub_14037fa50
    Replacement::draft(0x14037fca0, "r_14037fca0", fn_14037fca0 as *const ()), // sub_14037fca0
    Replacement::draft(0x14037fe30, "r_14037fe30", fn_14037fe30 as *const ()), // sub_14037fe30
    Replacement::draft(0x14037ffe0, "r_14037ffe0", fn_14037ffe0 as *const ()), // sub_14037ffe0
    Replacement::draft(0x140380170, "r_140380170", fn_140380170 as *const ()), // sub_140380170
    Replacement::draft(0x140380300, "r_140380300", fn_140380300 as *const ()), // sub_140380300
    Replacement::draft(0x1403803f0, "r_1403803f0", fn_1403803f0 as *const ()), // sub_1403803f0
    Replacement::draft(0x1403806c0, "r_1403806c0", fn_1403806c0 as *const ()), // sub_1403806c0
    Replacement::draft(0x140380730, "r_140380730", fn_140380730 as *const ()), // sub_140380730
    Replacement::draft(0x140389b50, "r_140389b50", fn_140389b50 as *const ()), // sub_140389b50
    Replacement::draft_throws(0x140389d20, "r_140389d20", crate::shards::subsystems::fn_140389d20 as *const (), Throws::Caught), // sub_140389d20
    Replacement::draft(0x140389ea0, "r_140389ea0", fn_140389ea0 as *const ()), // sub_140389ea0
    Replacement::draft(0x14038a1f0, "r_14038a1f0", tpl_3265::<0x143c96d20, 0x143c969f0, 0x143c8d0d0, 0x143c96b24, 0x143c8d120, 0x143c8e660, 0x143c8dbc4, 0x143c8dbd0, 0x143c96dec, 0x143c8e9b0, 0x143c8e670, 0x143c96a10, 0x143c96a30, 0x143c96a50, 0x141834a0c, 0x141834a78> as *const ()), // sub_14038a1f0
    Replacement::draft(0x14038a400, "r_14038a400", fn_14038a400 as *const ()), // sub_14038a400
    Replacement::draft(0x14038a6e0, "r_14038a6e0", fn_14038a6e0 as *const ()), // sub_14038a6e0
    Replacement::draft(0x14038aac0, "r_14038aac0", fn_14038aac0 as *const ()), // sub_14038aac0
    Replacement::draft(0x14038aeb0, "r_14038aeb0", fn_14038aeb0 as *const ()), // sub_14038aeb0
    Replacement::draft(0x14038af90, "r_14038af90", tpl_3270::<0x143c96d60, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14038af90
    Replacement::draft(0x14038baf0, "r_14038baf0", fn_14038baf0 as *const ()), // sub_14038baf0
    Replacement::draft(0x140392130, "r_140392130", tpl_1131::<0x142407c80, 0x143c8de68, 0x142489ee0, 0x141915950> as *const ()), // sub_140392130
    Replacement::draft(0x140392290, "r_140392290", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x14038e590> as *const ()), // sub_140392290
    Replacement::draft(0x140392370, "r_140392370", tpl_3292::<0x143c972bc, 0x143c971d0, 0x141834a78, 0x143c970dc, 0x143c8de68, 0x143c977ac, 0x143c97150, 0x143c9779c, 0x143c90198, 0x143c96de4, 0x143c8eb90, 0x143c971f0, 0x143c97210, 0x143c97230, 0x141834a0c> as *const ()), // sub_140392370
    Replacement::draft(0x140392730, "r_140392730", fn_140392730 as *const ()), // sub_140392730
    Replacement::draft(0x140392990, "r_140392990", fn_140392990 as *const ()), // sub_140392990
    Replacement::draft(0x140392b40, "r_140392b40", fn_140392b40 as *const ()), // sub_140392b40
    Replacement::draft(0x140392e90, "r_140392e90", fn_140392e90 as *const ()), // sub_140392e90
    Replacement::draft(0x140398980, "r_140398980", tpl_3326::<0x143c978c0, 0x143c97a6c, 0x10, 0x18, 0x8, 0x80, 0xffffffffffffff80, 0xffffffffffffff90, 0xffffffffffffffa0, 0xffffffffffffffb0, 0xffffffffffffffc0, 0xffffffffffffffd0, 0x141834a78, 0x140398aa0, 0x141834a0c> as *const ()), // sub_140398980
    Replacement::draft(0x140398aa0, "r_140398aa0", fn_140398aa0 as *const ()), // sub_140398aa0
    Replacement::draft_throws(0x14039c2d0, "r_14039c2d0", fn_14039c2d0 as *const (), Throws::Discard), // sub_14039c2d0
    Replacement::draft(0x14039c350, "r_14039c350", fn_14039c350 as *const ()), // sub_14039c350
    Replacement::draft(0x14039c5c0, "r_14039c5c0", fn_14039c5c0 as *const ()), // sub_14039c5c0
    Replacement::draft(0x14039c9d0, "r_14039c9d0", fn_14039c9d0 as *const ()), // sub_14039c9d0
    Replacement::draft(0x14039cc80, "r_14039cc80", fn_14039cc80 as *const ()), // sub_14039cc80
    Replacement::draft(0x1403a57d0, "r_1403a57d0", tpl_3389::<0x143c9821c, 0x143c982c0, 0x141834a78, 0x143c8d508, 0x143c91f94, 0x143c97ec4, 0x143c8e840, 0x143c8de68, 0x143c8e9b0, 0x143c8dbd0, 0x143c982e0, 0x143c98300, 0x143c98310, 0x141834a0c> as *const ()), // sub_1403a57d0
    Replacement::draft(0x1403a5af0, "r_1403a5af0", fn_1403a5af0 as *const ()), // sub_1403a5af0
    Replacement::draft(0x1403a5f90, "r_1403a5f90", tpl_1997::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430e2d90, 0x8> as *const ()), // sub_1403a5f90
    Replacement::draft(0x1403a61d0, "r_1403a61d0", fn_1403a61d0 as *const ()), // sub_1403a61d0
    Replacement::draft(0x1403a6640, "r_1403a6640", fn_1403a6640 as *const ()), // sub_1403a6640
    Replacement::draft(0x1403ae7a0, "r_1403ae7a0", tpl_3213::<0x142407c80, 0x143c8de68, 0x142489ee0, 0x143c8e70c, 0x141927f60> as *const ()), // sub_1403ae7a0
    Replacement::draft(0x1403ae950, "r_1403ae950", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1403ab300> as *const ()), // sub_1403ae950
    Replacement::draft(0x1403aed80, "r_1403aed80", fn_1403aed80 as *const ()), // sub_1403aed80
    Replacement::draft_throws(0x1403b2600, "r_1403b2600", fn_1403b2600 as *const (), Throws::Discard), // sub_1403b2600
    Replacement::draft(0x1403b27a0, "r_1403b27a0", fn_1403b27a0 as *const ()), // sub_1403b27a0
    Replacement::draft(0x1403becc0, "r_1403becc0", tpl_1212::<0x8, 0x1403c0b70> as *const ()), // sub_1403becc0
    Replacement::draft(0x1403c0170, "r_1403c0170", tpl_3497::<0x3, 0x4, 0x8, 0x14240a730, 0x58, 0x68, 0x70, 0x80, 0x88> as *const ()), // sub_1403c0170
    Replacement::draft(0x1403c0210, "r_1403c0210", fn_1403c0210 as *const ()), // sub_1403c0210
    Replacement::draft(0x1403c0430, "r_1403c0430", fn_1403c0430 as *const ()), // sub_1403c0430
    Replacement::draft(0x1403c0550, "r_1403c0550", fn_1403c0550 as *const ()), // sub_1403c0550
    Replacement::draft(0x1403c0680, "r_1403c0680", fn_1403c0680 as *const ()), // sub_1403c0680
    Replacement::draft(0x1403c08a0, "r_1403c08a0", fn_1403c08a0 as *const ()), // sub_1403c08a0
    Replacement::draft(0x1403c0b30, "r_1403c0b30", tpl_3502::<0x8, 0x10, 0x14240ab10> as *const ()), // sub_1403c0b30
    Replacement::draft(0x1403c0b70, "r_1403c0b70", tpl_1095::<0x143c8d000, 0x14020e8f0> as *const ()), // sub_1403c0b70
    Replacement::draft(0x1403c10e0, "r_1403c10e0", fn_1403c10e0 as *const ()), // sub_1403c10e0
    Replacement::draft(0x1403c12b0, "r_1403c12b0", fn_1403c12b0 as *const ()), // sub_1403c12b0
    Replacement::draft(0x1403c13e0, "r_1403c13e0", fn_1403c13e0 as *const ()), // sub_1403c13e0
    Replacement::draft(0x1403c1620, "r_1403c1620", fn_1403c1620 as *const ()), // sub_1403c1620
    Replacement::draft(0x1403c1970, "r_1403c1970", fn_1403c1970 as *const ()), // sub_1403c1970
    Replacement::draft(0x1403cb030, "r_1403cb030", tpl_1885::<0x143c9a328, 0x143c9a230, 0x143c8d160, 0x143c9a330, 0x143c9a4e0, 0x143c9a320, 0x143c9a338, 0x143c9a318, 0x143c9a2d4, 0x143c9a324, 0x143c8d694, 0x143c9a250, 0x143c9a270, 0x143c9a290, 0x141834a0c, 0x141834a78> as *const ()), // sub_1403cb030
    Replacement::draft(0x1403cb240, "r_1403cb240", fn_1403cb240 as *const ()), // sub_1403cb240
    Replacement::draft(0x1403cb400, "r_1403cb400", fn_1403cb400 as *const ()), // sub_1403cb400
    Replacement::draft(0x1403cb4f0, "r_1403cb4f0", fn_1403cb4f0 as *const ()), // sub_1403cb4f0
    Replacement::draft(0x1403cb5e0, "r_1403cb5e0", fn_1403cb5e0 as *const ()), // sub_1403cb5e0
    Replacement::draft(0x1403cb780, "r_1403cb780", fn_1403cb780 as *const ()), // sub_1403cb780
    Replacement::draft_throws(0x1403cfb20, "r_1403cfb20", tpl_62934::<0x143c9a83c, 0x142489ee0, 0x1419449f0> as *const (), Throws::Discard), // sub_1403cfb20
    Replacement::draft(0x1403cfc70, "r_1403cfc70", tpl_61552::<0x8, 0x88, 0x10088, 0x1419448c0> as *const ()), // sub_1403cfc70
    Replacement::draft(0x1403cfd60, "r_1403cfd60", tpl_3028::<0x143c9a6b8, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1403ce370> as *const ()), // sub_1403cfd60
    Replacement::draft(0x1403cfdd0, "r_1403cfdd0", fn_1403cfdd0 as *const ()), // sub_1403cfdd0
    Replacement::draft_throws(0x1403d6430, "r_1403d6430", tpl_1330::<0x8, 0x1403d7710, 0x143c9ac30, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1403d6430
    Replacement::draft(0x1403de730, "r_1403de730", tpl_1131::<0x142407c80, 0x143c9b4b0, 0x142489ee0, 0x14194c590> as *const ()), // sub_1403de730
    Replacement::draft(0x1403de890, "r_1403de890", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x1403dcc20> as *const ()), // sub_1403de890
    Replacement::draft(0x1403dea60, "r_1403dea60", fn_1403dea60 as *const ()), // sub_1403dea60
    Replacement::draft(0x1403e8e80, "r_1403e8e80", fn_1403e8e80 as *const ()), // sub_1403e8e80
    Replacement::draft_throws(0x1403e9010, "r_1403e9010", tpl_3757::<0x143c9beb0, 0x143c9bdd4, 0x10, 0x1420a25b0, 0x141834a78, 0x143c9bce0, 0x143c8ce88, 0x143c9bdd0, 0x143c9bde0, 0x141834a0c> as *const (), Throws::Discard), // sub_1403e9010
    Replacement::draft(0x1403f05a0, "r_1403f05a0", tpl_63097::<0x143c9c408, 0x143c8d120, 0x142489ee0, 0x143c8d160, 0x143c9c02c, 0x14195c1d0> as *const ()), // sub_1403f05a0
    Replacement::draft_throws(0x1403f0820, "r_1403f0820", tpl_63098::<0x143c9c030, 0x1403eff20, 0x14195bac0> as *const (), Throws::Discard), // sub_1403f0820
    Replacement::draft(0x1403f09f0, "r_1403f09f0", tpl_63099::<0x143c9bebc, 0x142489ee0, 0x14195b710> as *const ()), // sub_1403f09f0
    Replacement::draft(0x1403f1360, "r_1403f1360", fn_1403f1360 as *const ()), // sub_1403f1360
    Replacement::draft(0x1403f13d0, "r_1403f13d0", fn_1403f13d0 as *const ()), // sub_1403f13d0
    Replacement::draft(0x1403f2040, "r_1403f2040", fn_1403f2040 as *const ()), // sub_1403f2040
    Replacement::draft(0x1403f20e0, "r_1403f20e0", fn_1403f20e0 as *const ()), // sub_1403f20e0
    Replacement::draft(0x1403f21c0, "r_1403f21c0", fn_1403f21c0 as *const ()), // sub_1403f21c0
    Replacement::draft(0x1403f2330, "r_1403f2330", tpl_63102::<0x143c8d08c, 0x1401fa730, 0x143c9c660, 0x1426467e8, 0x143c9c280, 0x142489ee0, 0x143c9bebc, 0x1403f2423> as *const ()), // sub_1403f2330
    Replacement::draft(0x1403f25a0, "r_1403f25a0", tpl_63103::<0x143c8d08c, 0x1401fa730, 0x143c9c658, 0x1426467e8, 0x143c9bd58, 0x142489ee0, 0x1403f2697> as *const ()), // sub_1403f25a0
    Replacement::draft(0x1403f27b0, "r_1403f27b0", tpl_3810::<0x143c8d08c, 0x1401fa730, 0x143c9c744, 0x1426467e8, 0x1403f29e0, 0x1403f28b0> as *const ()), // sub_1403f27b0
    Replacement::draft(0x1403f2b30, "r_1403f2b30", tpl_1095::<0x143c9c020, 0x14020e8f0> as *const ()), // sub_1403f2b30
    Replacement::draft_throws(0x1403f2d10, "r_1403f2d10", tpl_3811::<0x143c9c65c, 0x143c9c598, 0x143c9c808, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9bebc, 0x143c8d120, 0x143c9c818, 0x143c9c820, 0x141834a0c> as *const (), Throws::Discard), // sub_1403f2d10
    Replacement::draft_throws(0x1403f2e70, "r_1403f2e70", fn_1403f2e70 as *const (), Throws::Discard), // sub_1403f2e70
    Replacement::draft_throws(0x1403f3010, "r_1403f3010", fn_1403f3010 as *const (), Throws::Discard), // sub_1403f3010
    Replacement::draft(0x140410d50, "r_140410d50", fn_140410d50 as *const ()), // sub_140410d50
    Replacement::draft(0x140410f00, "r_140410f00", fn_140410f00 as *const ()), // sub_140410f00
    Replacement::draft(0x140411270, "r_140411270", fn_140411270 as *const ()), // sub_140411270
    Replacement::draft(0x1404112e0, "r_1404112e0", fn_1404112e0 as *const ()), // sub_1404112e0
    Replacement::draft(0x1404113d0, "r_1404113d0", fn_1404113d0 as *const ()), // sub_1404113d0
    Replacement::draft(0x140411490, "r_140411490", tpl_4024::<0x8, 0x140415e30, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140411490
    Replacement::draft(0x140411550, "r_140411550", fn_140411550 as *const ()), // sub_140411550
    Replacement::draft(0x1404115c0, "r_1404115c0", fn_1404115c0 as *const ()), // sub_1404115c0
    Replacement::draft_throws(0x140411650, "r_140411650", tpl_4027::<0x143c9f29c, 0x143c86378, 0x143c86cd8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d0d0, 0x143c9d658, 0x143c86ce8, 0x143c86cf0, 0x141834a0c> as *const (), Throws::Discard), // sub_140411650
    Replacement::draft(0x1404117b0, "r_1404117b0", tpl_1360::<0x30, 0x143c988d0, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140411833> as *const ()), // sub_1404117b0
    Replacement::draft(0x140411930, "r_140411930", tpl_4028::<0x30, 0x143c91f94, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1404119b3> as *const ()), // sub_140411930
    Replacement::draft(0x140411bb0, "r_140411bb0", fn_140411bb0 as *const ()), // sub_140411bb0
    Replacement::draft(0x140411c40, "r_140411c40", fn_140411c40 as *const ()), // sub_140411c40
    Replacement::draft(0x140411cd0, "r_140411cd0", fn_140411cd0 as *const ()), // sub_140411cd0
    Replacement::draft(0x140411d50, "r_140411d50", fn_140411d50 as *const ()), // sub_140411d50
    Replacement::draft_throws(0x140411de0, "r_140411de0", tpl_4033::<0x143c9e2d0, 0x143c9f0c4, 0x143c9d0a0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c96de8, 0x143c8d0d0, 0x143c9d0b0, 0x143c9d0b8, 0x141834a0c> as *const (), Throws::Discard), // sub_140411de0
    Replacement::draft_throws(0x140411f40, "r_140411f40", fn_140411f40 as *const (), Throws::Discard), // sub_140411f40
    Replacement::draft_throws(0x1404120a0, "r_1404120a0", fn_1404120a0 as *const (), Throws::Discard), // sub_1404120a0
    Replacement::draft_throws(0x140412290, "r_140412290", tpl_1845::<0x143c9f20c, 0x143c9d0bc, 0x143c9e7b8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c90164, 0x143c8d0d0, 0x143c9e7c8, 0x143c9e7d0, 0x141834a0c> as *const (), Throws::Discard), // sub_140412290
    Replacement::draft_throws(0x1404123f0, "r_1404123f0", tpl_4017::<0x143c9e5c8, 0x143c9f2f4, 0x143c9e3e0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9eeec, 0x143c8d160, 0x143c8d0d0, 0x143c9e400, 0x141834a0c> as *const (), Throws::Discard), // sub_1404123f0
    Replacement::draft_throws(0x140412560, "r_140412560", tpl_4036::<0x143c9eca4, 0x143c9e9cc, 0x143c9ced8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c96b14, 0x143c8d0d0, 0x143c9cee8, 0x143c9cef0, 0x141834a0c> as *const (), Throws::Discard), // sub_140412560
    Replacement::draft_throws(0x1404126c0, "r_1404126c0", tpl_4036::<0x143c9e248, 0x143c9de60, 0x143c9e540, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d1ac, 0x143c8d0d0, 0x143c9e550, 0x143c9e558, 0x141834a0c> as *const (), Throws::Discard), // sub_1404126c0
    Replacement::draft_throws(0x140412820, "r_140412820", tpl_4017::<0x143c9d450, 0x143c9e380, 0x143c9d8f8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c90164, 0x143c8d0d0, 0x143c9d918, 0x141834a0c> as *const (), Throws::Discard), // sub_140412820
    Replacement::draft_throws(0x140412990, "r_140412990", tpl_4037::<0x143c9dde4, 0x143c9dc80, 0x143c9f070, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d1b4, 0x143c9efb0, 0x143c9f080, 0x143c9f088, 0x141834a0c> as *const (), Throws::Discard), // sub_140412990
    Replacement::draft_throws(0x140412af0, "r_140412af0", tpl_4037::<0x143c9d610, 0x143c9e538, 0x143c9d588, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c91064, 0x143c98790, 0x143c9d598, 0x143c9d5a0, 0x141834a0c> as *const (), Throws::Discard), // sub_140412af0
    Replacement::draft_throws(0x140412c50, "r_140412c50", tpl_4037::<0x143c9d84c, 0x143c9d6c8, 0x143c9d9d8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e710, 0x143c9665c, 0x143c9d9e8, 0x143c9d9f0, 0x141834a0c> as *const (), Throws::Discard), // sub_140412c50
    Replacement::draft_throws(0x140412db0, "r_140412db0", tpl_4037::<0x143c9ed2c, 0x143c9cd18, 0x143c9cdd0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e710, 0x143c956a0, 0x143c9cde0, 0x143c9cde8, 0x141834a0c> as *const (), Throws::Discard), // sub_140412db0
    Replacement::draft_throws(0x140412f10, "r_140412f10", tpl_4037::<0x143c9da08, 0x143c9f2a0, 0x143c9d370, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e710, 0x143c8f690, 0x143c9d380, 0x143c9d388, 0x141834a0c> as *const (), Throws::Discard), // sub_140412f10
    Replacement::draft_throws(0x140413070, "r_140413070", tpl_4037::<0x143c9ed70, 0x143c9de74, 0x143c9cfb0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dbcc, 0x143c8ec24, 0x143c9cfc0, 0x143c9cfc8, 0x141834a0c> as *const (), Throws::Discard), // sub_140413070
    Replacement::draft_throws(0x1404131d0, "r_1404131d0", tpl_4037::<0x143c9f0bc, 0x143c9e7ac, 0x143c9c828, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dbcc, 0x143c8e664, 0x143c9c838, 0x143c9c840, 0x141834a0c> as *const (), Throws::Discard), // sub_1404131d0
    Replacement::draft_throws(0x140413330, "r_140413330", tpl_4037::<0x143c9da00, 0x143c9f1b8, 0x143c9ef70, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dbcc, 0x143c8eb98, 0x143c9ef80, 0x143c9ef88, 0x141834a0c> as *const (), Throws::Discard), // sub_140413330
    Replacement::draft_throws(0x140413490, "r_140413490", tpl_4037::<0x143c9efb4, 0x143c9dc8c, 0x143c9ef90, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dbcc, 0x143c8eba0, 0x143c9efa0, 0x143c9efa8, 0x141834a0c> as *const (), Throws::Discard), // sub_140413490
    Replacement::draft_throws(0x1404135f0, "r_1404135f0", tpl_4037::<0x143c9f124, 0x143c9cdc8, 0x143c9d5a8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dbcc, 0x143c8de7c, 0x143c9d5b8, 0x143c9d5c0, 0x141834a0c> as *const (), Throws::Discard), // sub_1404135f0
    Replacement::draft_throws(0x140413750, "r_140413750", tpl_847::<0x143c9f1c0, 0x143c9df98, 0x143c9d820, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c95bf0, 0x141834a0c> as *const (), Throws::Discard), // sub_140413750
    Replacement::draft_throws(0x140413880, "r_140413880", fn_140413880 as *const (), Throws::Discard), // sub_140413880
    Replacement::draft_throws(0x140413a40, "r_140413a40", tpl_4018::<0x143c9e8c4, 0x143c9d0e4, 0x143c9d040, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c9d1c4, 0x143c9d050, 0x143c9d058, 0x141834a0c> as *const (), Throws::Discard), // sub_140413a40
    Replacement::draft_throws(0x140413ba0, "r_140413ba0", fn_140413ba0 as *const (), Throws::Discard), // sub_140413ba0
    Replacement::draft_throws(0x140413d20, "r_140413d20", tpl_2918::<0x143c9dc88, 0x143c9db10, 0x143c9dd78, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d1d8, 0x141834a0c> as *const (), Throws::Discard), // sub_140413d20
    Replacement::draft_throws(0x140413e50, "r_140413e50", tpl_847::<0x143c9d174, 0x143c9e750, 0x143c9c9e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9e938, 0x141834a0c> as *const (), Throws::Discard), // sub_140413e50
    Replacement::draft_throws(0x140414010, "r_140414010", fn_140414010 as *const (), Throws::Discard), // sub_140414010
    Replacement::draft_throws(0x140414260, "r_140414260", tpl_4033::<0x143c9df58, 0x143c9d224, 0x143c9d0c8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8f6e4, 0x143c91600, 0x143c9d0d8, 0x143c9d0e0, 0x141834a0c> as *const (), Throws::Discard), // sub_140414260
    Replacement::draft_throws(0x1404143c0, "r_1404143c0", tpl_1845::<0x143c9f33c, 0x143c9d570, 0x143c9f310, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d120, 0x143c98fc0, 0x143c9f320, 0x143c9f328, 0x141834a0c> as *const (), Throws::Discard), // sub_1404143c0
    Replacement::draft_throws(0x140414520, "r_140414520", fn_140414520 as *const (), Throws::Discard), // sub_140414520
    Replacement::draft_throws(0x140414700, "r_140414700", fn_140414700 as *const (), Throws::Discard), // sub_140414700
    Replacement::draft_throws(0x140414a10, "r_140414a10", tpl_2918::<0x143c9d158, 0x143c9d1f0, 0x143c9e448, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9de20, 0x141834a0c> as *const (), Throws::Discard), // sub_140414a10
    Replacement::draft_throws(0x140414b50, "r_140414b50", tpl_4037::<0x143c9d18c, 0x143c9d208, 0x143c9f278, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c95468, 0x143c8d87c, 0x143c9f288, 0x143c9f290, 0x141834a0c> as *const (), Throws::Discard), // sub_140414b50
    Replacement::draft_throws(0x140414cb0, "r_140414cb0", tpl_846::<0x143c9d1c8, 0x143c9d148, 0x143c9da68, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9da04, 0x143c8f6d8, 0x143c8d120, 0x143c9da88, 0x141834a0c> as *const (), Throws::Discard), // sub_140414cb0
    Replacement::draft_throws(0x140414eb0, "r_140414eb0", tpl_846::<0x143c9d184, 0x143c9e160, 0x143c9f090, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8cefc, 0x143c8f6d8, 0x143c8d120, 0x143c9f0b0, 0x141834a0c> as *const (), Throws::Discard), // sub_140414eb0
    Replacement::draft_throws(0x140415020, "r_140415020", tpl_4016::<0x143c9f018, 0x143c9e950, 0x143c9cfd0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c93b20, 0x143c8d120, 0x143c9cfe0, 0x143c9cfe8, 0x141834a0c> as *const (), Throws::Discard), // sub_140415020
    Replacement::draft_throws(0x140415180, "r_140415180", fn_140415180 as *const (), Throws::Discard), // sub_140415180
    Replacement::draft_throws(0x140415300, "r_140415300", tpl_4018::<0x143c9e870, 0x143c9ca68, 0x143c9e408, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d160, 0x143c9a074, 0x143c9e418, 0x143c9e420, 0x141834a0c> as *const (), Throws::Discard), // sub_140415300
    Replacement::draft(0x140415460, "r_140415460", fn_140415460 as *const ()), // sub_140415460
    Replacement::draft(0x140415500, "r_140415500", tpl_4044::<0x14240aa00> as *const ()), // sub_140415500
    Replacement::draft(0x140415700, "r_140415700", tpl_4044::<0x14240a730> as *const ()), // sub_140415700
    Replacement::draft(0x140415900, "r_140415900", fn_140415900 as *const ()), // sub_140415900
    Replacement::draft(0x140415940, "r_140415940", tpl_855::<0x1403f7710, 0x1425371a0, 0x142533200, 0x14040a070, 0x14040f080> as *const ()), // sub_140415940
    Replacement::draft(0x140415aa0, "r_140415aa0", tpl_381::<0x140415c00, 0x140234b10> as *const ()), // sub_140415aa0
    Replacement::draft(0x140415c00, "r_140415c00", fn_140415c00 as *const ()), // sub_140415c00
    Replacement::draft(0x140415e30, "r_140415e30", fn_140415e30 as *const ()), // sub_140415e30
    Replacement::draft(0x140415ea0, "r_140415ea0", fn_140415ea0 as *const ()), // sub_140415ea0
    Replacement::draft(0x140415fd0, "r_140415fd0", tpl_3077::<0x143c96594, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140415fd0
    Replacement::draft(0x140416300, "r_140416300", fn_140416300 as *const ()), // sub_140416300
    Replacement::draft_throws(0x140416370, "r_140416370", tpl_4018::<0x143c9d8c0, 0x143c9e878, 0x143c9dd88, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8ec68, 0x143c90164, 0x143c9dd98, 0x143c9dda0, 0x141834a0c> as *const (), Throws::Discard), // sub_140416370
    Replacement::draft_throws(0x1404164d0, "r_1404164d0", fn_1404164d0 as *const (), Throws::Discard), // sub_1404164d0
    Replacement::draft_throws(0x140416680, "r_140416680", fn_140416680 as *const (), Throws::Discard), // sub_140416680
    Replacement::draft_throws(0x140416810, "r_140416810", fn_140416810 as *const (), Throws::Discard), // sub_140416810
    Replacement::draft_throws(0x140416ac0, "r_140416ac0", tpl_4033::<0x143c9d1a8, 0x143c86e2c, 0x143c86fc0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8efb0, 0x143c8d0d0, 0x143c86fd0, 0x143c86fd8, 0x141834a0c> as *const (), Throws::Discard), // sub_140416ac0
    Replacement::draft_throws(0x140416c20, "r_140416c20", tpl_1845::<0x143c9ec1c, 0x143c9d474, 0x143c9f1e0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8eb90, 0x143c8d0d0, 0x143c9f1f0, 0x143c9f1f8, 0x141834a0c> as *const (), Throws::Discard), // sub_140416c20
    Replacement::draft(0x140416d80, "r_140416d80", fn_140416d80 as *const ()), // sub_140416d80
    Replacement::draft(0x140417010, "r_140417010", fn_140417010 as *const ()), // sub_140417010
    Replacement::draft_throws(0x1404170b0, "r_1404170b0", fn_1404170b0 as *const (), Throws::Discard), // sub_1404170b0
    Replacement::draft_throws(0x1404182c0, "r_1404182c0", tpl_2918::<0x143c9dc78, 0x143c9e56c, 0x143c9d578, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9dba4, 0x141834a0c> as *const (), Throws::Discard), // sub_1404182c0
    Replacement::draft_throws(0x1404183f0, "r_1404183f0", fn_1404183f0 as *const (), Throws::Discard), // sub_1404183f0
    Replacement::draft_throws(0x1404185e0, "r_1404185e0", fn_1404185e0 as *const (), Throws::Discard), // sub_1404185e0
    Replacement::draft_throws(0x1404187f0, "r_1404187f0", fn_1404187f0 as *const (), Throws::Discard), // sub_1404187f0
    Replacement::draft_throws(0x1404189b0, "r_1404189b0", tpl_847::<0x143c9dac8, 0x143c9f064, 0x143c9e940, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9779c, 0x141834a0c> as *const (), Throws::Discard), // sub_1404189b0
    Replacement::draft_throws(0x140418ae0, "r_140418ae0", tpl_2918::<0x143c9d830, 0x143c9c89c, 0x143c9e760, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e70c, 0x141834a0c> as *const (), Throws::Discard), // sub_140418ae0
    Replacement::draft_throws(0x140418c10, "r_140418c10", tpl_3811::<0x143c9e880, 0x143c9d454, 0x143c9dbe8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9db0c, 0x143c9ecdc, 0x143c9dbf8, 0x143c9dc00, 0x141834a0c> as *const (), Throws::Discard), // sub_140418c10
    Replacement::draft_throws(0x140418d70, "r_140418d70", tpl_4058::<0x143c9d670, 0x143c9d8c4, 0x143c9ece8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e70c, 0x143c9db0c, 0x143c9ecdc, 0x143c9ed08, 0x141834a0c> as *const (), Throws::Discard), // sub_140418d70
    Replacement::draft_throws(0x140418ef0, "r_140418ef0", tpl_4018::<0x143c9d220, 0x143c9e7a8, 0x143c9d2f8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8ea68, 0x143c95048, 0x143c9d308, 0x143c9d310, 0x141834a0c> as *const (), Throws::Discard), // sub_140418ef0
    Replacement::draft_throws(0x140419050, "r_140419050", tpl_847::<0x143c9e7d4, 0x143c9d83c, 0x143c9eb68, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c956a0, 0x141834a0c> as *const (), Throws::Discard), // sub_140419050
    Replacement::draft_throws(0x140419180, "r_140419180", tpl_4059::<0x143c9eef8, 0x143c9e070, 0x143c9ead0, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92ee8, 0x143c937c8, 0x143c9e55c, 0x143c9eaf0, 0x141834a0c> as *const (), Throws::Discard), // sub_140419180
    Replacement::draft_throws(0x140419300, "r_140419300", fn_140419300 as *const (), Throws::Discard), // sub_140419300
    Replacement::draft_throws(0x1404194a0, "r_1404194a0", fn_1404194a0 as *const (), Throws::Discard), // sub_1404194a0
    Replacement::draft_throws(0x1404196b0, "r_1404196b0", fn_1404196b0 as *const (), Throws::Discard), // sub_1404196b0
    Replacement::draft_throws(0x140419930, "r_140419930", tpl_847::<0x143c9d8c8, 0x143c9f294, 0x143c9db18, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c93228, 0x141834a0c> as *const (), Throws::Discard), // sub_140419930
    Replacement::draft_throws(0x140419a60, "r_140419a60", tpl_2918::<0x143c9d16c, 0x143c9dcc4, 0x143c9cae0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c93228, 0x141834a0c> as *const (), Throws::Discard), // sub_140419a60
    Replacement::draft_throws(0x140419b90, "r_140419b90", fn_140419b90 as *const (), Throws::Discard), // sub_140419b90
    Replacement::draft_throws(0x140419d90, "r_140419d90", fn_140419d90 as *const (), Throws::Discard), // sub_140419d90
    Replacement::draft_throws(0x140419f50, "r_140419f50", fn_140419f50 as *const (), Throws::Discard), // sub_140419f50
    Replacement::draft_throws(0x14041a0d0, "r_14041a0d0", tpl_4018::<0x143c9d44c, 0x143c9e4b0, 0x143c9e100, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c937c8, 0x143c9d840, 0x143c9e110, 0x143c9e118, 0x141834a0c> as *const (), Throws::Discard), // sub_14041a0d0
    Replacement::draft_throws(0x14041a230, "r_14041a230", fn_14041a230 as *const (), Throws::Discard), // sub_14041a230
    Replacement::draft_throws(0x14041a3d0, "r_14041a3d0", tpl_4037::<0x143c9d674, 0x143c9ce64, 0x143c9eb78, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92a7c, 0x143c937c8, 0x143c9eb88, 0x143c9eb90, 0x141834a0c> as *const (), Throws::Discard), // sub_14041a3d0
    Replacement::draft_throws(0x14041a530, "r_14041a530", tpl_847::<0x143c9d204, 0x143c9e5e0, 0x143c9f1c8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92a7c, 0x141834a0c> as *const (), Throws::Discard), // sub_14041a530
    Replacement::draft_throws(0x14041a660, "r_14041a660", tpl_847::<0x143c9e4a0, 0x143c9eb9c, 0x143c9e460, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c937c8, 0x141834a0c> as *const (), Throws::Discard), // sub_14041a660
    Replacement::draft_throws(0x14041a790, "r_14041a790", tpl_4018::<0x143c9e4f4, 0x143c9d468, 0x143c9e428, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92c98, 0x143c937c8, 0x143c9e438, 0x143c9e440, 0x141834a0c> as *const (), Throws::Discard), // sub_14041a790
    Replacement::draft_throws(0x14041a8f0, "r_14041a8f0", tpl_2921::<0x143c9e884, 0x143c9f0d4, 0x143c9d480, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92a7c, 0x143c9d9fc, 0x143c92ee8, 0x143c9d840, 0x143c937c8, 0x143c9d4a0, 0x141834a0c> as *const (), Throws::Discard), // sub_14041a8f0
    Replacement::draft_throws(0x14041aaa0, "r_14041aaa0", tpl_847::<0x143c9f1fc, 0x143c9d8d4, 0x143c9f258, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d19c, 0x141834a0c> as *const (), Throws::Discard), // sub_14041aaa0
    Replacement::draft_throws(0x14041abd0, "r_14041abd0", tpl_847::<0x143c9ee28, 0x143c9d65c, 0x143c9d028, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92ee8, 0x141834a0c> as *const (), Throws::Discard), // sub_14041abd0
    Replacement::draft_throws(0x14041ad00, "r_14041ad00", tpl_2918::<0x143c9e49c, 0x143c9d098, 0x143c9e310, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92ee8, 0x141834a0c> as *const (), Throws::Discard), // sub_14041ad00
    Replacement::draft_throws(0x14041ae30, "r_14041ae30", tpl_847::<0x143c9d1d0, 0x143c9d9f8, 0x143c9e5b8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92c98, 0x141834a0c> as *const (), Throws::Discard), // sub_14041ae30
    Replacement::draft_throws(0x14041af60, "r_14041af60", tpl_2918::<0x143c9e37c, 0x143c9ccc8, 0x143c9d238, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c92c98, 0x141834a0c> as *const (), Throws::Discard), // sub_14041af60
    Replacement::draft_throws(0x14041b090, "r_14041b090", fn_14041b090 as *const (), Throws::Discard), // sub_14041b090
    Replacement::draft_throws(0x14041b230, "r_14041b230", tpl_847::<0x143c9d1dc, 0x143c9cc64, 0x143c9d5c8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9d6b8, 0x141834a0c> as *const (), Throws::Discard), // sub_14041b230
    Replacement::draft_throws(0x14041b360, "r_14041b360", tpl_4068::<0x143c9d45c, 0x143c9d228, 0x143c9d0f8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9ea00, 0x141834a0c> as *const (), Throws::Discard), // sub_14041b360
    Replacement::draft_throws(0x14041b490, "r_14041b490", fn_14041b490 as *const (), Throws::Discard), // sub_14041b490
    Replacement::draft_throws(0x14041b610, "r_14041b610", tpl_847::<0x143c9d168, 0x143c9e4f8, 0x143c9cd08, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8e9b0, 0x141834a0c> as *const (), Throws::Discard), // sub_14041b610
    Replacement::draft_throws(0x14041b740, "r_14041b740", fn_14041b740 as *const (), Throws::Discard), // sub_14041b740
    Replacement::draft_throws(0x14041b930, "r_14041b930", fn_14041b930 as *const (), Throws::Discard), // sub_14041b930
    Replacement::draft_throws(0x14041bb60, "r_14041bb60", fn_14041bb60 as *const (), Throws::Discard), // sub_14041bb60
    Replacement::draft_throws(0x14041bd10, "r_14041bd10", tpl_2918::<0x143c9d1a4, 0x143c9e564, 0x143c9da48, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c93310, 0x141834a0c> as *const (), Throws::Discard), // sub_14041bd10
    Replacement::draft_throws(0x14041be40, "r_14041be40", tpl_847::<0x143c9f174, 0x143c9d740, 0x143c9cc78, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8dbc4, 0x141834a0c> as *const (), Throws::Discard), // sub_14041be40
    Replacement::draft_throws(0x14041c000, "r_14041c000", fn_14041c000 as *const (), Throws::Discard), // sub_14041c000
    Replacement::draft_throws(0x14041c1a0, "r_14041c1a0", fn_14041c1a0 as *const (), Throws::Discard), // sub_14041c1a0
    Replacement::draft_throws(0x14041c360, "r_14041c360", tpl_4037::<0x143c9d230, 0x143c9da60, 0x143c9ed10, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c9a330, 0x143c9a320, 0x143c9ed20, 0x143c9ed28, 0x141834a0c> as *const (), Throws::Discard), // sub_14041c360
    Replacement::draft_throws(0x14041c4c0, "r_14041c4c0", fn_14041c4c0 as *const (), Throws::Discard), // sub_14041c4c0
    Replacement::draft_throws(0x14041c5f0, "r_14041c5f0", tpl_4068::<0x143c9dc70, 0x143c9e568, 0x143c9f2e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d68c, 0x141834a0c> as *const (), Throws::Discard), // sub_14041c5f0
    Replacement::draft_throws(0x14041c720, "r_14041c720", tpl_4058::<0x143c9f204, 0x143c9cc70, 0x143c9f410, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8f7a0, 0x143c8d120, 0x143c8f6dc, 0x143c9f430, 0x141834a0c> as *const (), Throws::Discard), // sub_14041c720
    Replacement::draft_throws(0x14041c8a0, "r_14041c8a0", fn_14041c8a0 as *const (), Throws::Discard), // sub_14041c8a0
    Replacement::draft_throws(0x14041ca30, "r_14041ca30", tpl_2918::<0x143c9e624, 0x143c9dc48, 0x143c9d660, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d518, 0x141834a0c> as *const (), Throws::Discard), // sub_14041ca30
    Replacement::draft_throws(0x14041cb60, "r_14041cb60", tpl_847::<0x143c9d190, 0x143c9db64, 0x143c9c8e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d478, 0x141834a0c> as *const (), Throws::Discard), // sub_14041cb60
    Replacement::draft_throws(0x14041cd20, "r_14041cd20", tpl_4059::<0x143c9dba0, 0x143c9eca8, 0x143c9d858, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d68c, 0x143c8d708, 0x143c8d5b0, 0x143c9d878, 0x141834a0c> as *const (), Throws::Discard), // sub_14041cd20
    Replacement::draft(0x14041cea0, "r_14041cea0", fn_14041cea0 as *const ()), // sub_14041cea0
    Replacement::draft(0x14041d160, "r_14041d160", fn_14041d160 as *const ()), // sub_14041d160
    Replacement::draft(0x14041d220, "r_14041d220", fn_14041d220 as *const ()), // sub_14041d220
    Replacement::draft(0x14041d510, "r_14041d510", fn_14041d510 as *const ()), // sub_14041d510
    Replacement::draft(0x14041f200, "r_14041f200", fn_14041f200 as *const ()), // sub_14041f200
    Replacement::draft(0x14041f2b0, "r_14041f2b0", tpl_650::<0x18> as *const ()), // sub_14041f2b0
    Replacement::draft(0x14041f2c0, "r_14041f2c0", tpl_1241::<0x8, 0x4, 0x14240ab10> as *const ()), // sub_14041f2c0
    Replacement::draft(0x14042e4f0, "r_14042e4f0", tpl_1130::<0x142407c80, 0x1419bcea0> as *const ()), // sub_14042e4f0
    Replacement::draft(0x14042e600, "r_14042e600", tpl_1130::<0x142407c80, 0x1419bcc30> as *const ()), // sub_14042e600
    Replacement::draft(0x14042e890, "r_14042e890", fn_14042e890 as *const ()), // sub_14042e890
    Replacement::draft(0x14042eaa0, "r_14042eaa0", tpl_4140::<0x143c9fbf8, 0x143c9ff50, 0x143c9fb58, 0x143c9fb54, 0x143c8f4d0, 0x143c8de68, 0x143c919a8, 0x143c9fca4, 0x143c8d120, 0x143c8d160, 0x143c8e840, 0x143c9fde0, 0x143c8f4d4, 0x143c9ff70, 0x143c9ff90, 0x143c9ffb0, 0x143c9ffd0, 0x141834a0c, 0x141834a78> as *const ()), // sub_14042eaa0
    Replacement::draft(0x14042ed00, "r_14042ed00", tpl_4140::<0x143c9fa30, 0x143c9fcf0, 0x143c9fe9c, 0x143c9fb54, 0x143c8f4d0, 0x143c8de68, 0x143c919a8, 0x143c9fca4, 0x143c8d120, 0x143c8d160, 0x143c8e840, 0x143c9fde0, 0x143c8f4d4, 0x143c9fd10, 0x143c9fd30, 0x143c9fd50, 0x143c9fd70, 0x141834a0c, 0x141834a78> as *const ()), // sub_14042ed00
    Replacement::draft_throws(0x14046c0b0, "r_14046c0b0", fn_14046c0b0 as *const (), Throws::Discard), // sub_14046c0b0
    Replacement::draft(0x14046c420, "r_14046c420", fn_14046c420 as *const ()), // sub_14046c420
    Replacement::draft(0x14046d270, "r_14046d270", fn_14046d270 as *const ()), // sub_14046d270
    Replacement::draft(0x14046d2c0, "r_14046d2c0", fn_14046d2c0 as *const ()), // sub_14046d2c0
    Replacement::draft(0x14046d480, "r_14046d480", fn_14046d480 as *const ()), // sub_14046d480
    Replacement::draft(0x14046d6d0, "r_14046d6d0", fn_14046d6d0 as *const ()), // sub_14046d6d0
    Replacement::draft_throws(0x14048fd50, "r_14048fd50", fn_14048fd50 as *const (), Throws::Discard), // sub_14048fd50
    Replacement::draft_throws(0x1404900d0, "r_1404900d0", fn_1404900d0 as *const (), Throws::Discard), // sub_1404900d0
    Replacement::draft_throws(0x140490360, "r_140490360", tpl_4018::<0x143ca54fc, 0x143ca55d4, 0x143ca5490, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c98790, 0x143c8f4cc, 0x143ca54a0, 0x143ca54a8, 0x141834a0c> as *const (), Throws::Discard), // sub_140490360
    Replacement::draft(0x140490600, "r_140490600", fn_140490600 as *const ()), // sub_140490600
    Replacement::draft(0x140490740, "r_140490740", fn_140490740 as *const ()), // sub_140490740
    Replacement::draft(0x140490ae0, "r_140490ae0", fn_140490ae0 as *const ()), // sub_140490ae0
    Replacement::draft(0x140490cf0, "r_140490cf0", fn_140490cf0 as *const ()), // sub_140490cf0
    Replacement::draft(0x14049ff20, "r_14049ff20", fn_14049ff20 as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<platform::storage::HandleErrorStatus,platform:
    Replacement::draft(0x14049ffa0, "r_14049ffa0", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_14049ffa0
    Replacement::draft(0x1404a0010, "r_1404a0010", fn_1404a0010 as *const ()), // sub_1404a0010
    Replacement::draft(0x1404a0110, "r_1404a0110", fn_1404a0110 as *const ()), // sub_1404a0110
    Replacement::draft(0x1404a01a0, "r_1404a01a0", fn_1404a01a0 as *const ()), // sub_1404a01a0
    Replacement::draft(0x1404a01d0, "r_1404a01d0", fn_1404a01d0 as *const ()), // sub_1404a01d0
    Replacement::draft(0x1404a0270, "r_1404a0270", fn_1404a0270 as *const ()), // sub_1404a0270
    Replacement::draft(0x1404a02f0, "r_1404a02f0", fn_1404a02f0 as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise<r::Expected<platform::storage::HandleErrorStatus,platform::Err
    Replacement::draft(0x1404b06b0, "r_1404b06b0", fn_1404b06b0 as *const ()), // sub_1404b06b0
    Replacement::draft_throws(0x1404b06f0, "r_1404b06f0", fn_1404b06f0 as *const (), Throws::Caught), // sub_1404b06f0
    Replacement::draft(0x1404b0850, "r_1404b0850", tpl_1558::<0x143ca6d34, 0x143c8f63c, 0x143c9c824, 0x20, 0x143c8d0d4, 0x8, 0x580e0, 0x143c8ceec, 0x141a42ab0> as *const ()), // sub_1404b0850
    Replacement::draft_throws(0x1404b0920, "r_1404b0920", tpl_4147::<0x143ca691c, 0x143c8d08c, 0x20, 0x143c8d0d4, 0x8, 0x580e0, 0x143c9c824, 0x141a41fa0> as *const (), Throws::Discard), // sub_1404b0920
    Replacement::draft(0x1404b09b0, "r_1404b09b0", fn_1404b09b0 as *const ()), // sub_1404b09b0
    Replacement::draft(0x1404b0cd0, "r_1404b0cd0", tpl_1128::<0x142407c80, 0x143ca6798, 0x142489ee0, 0x143ca6c78, 0x143ca6a84, 0x141a43ee0> as *const ()), // sub_1404b0cd0
    Replacement::draft(0x1404b0ef0, "r_1404b0ef0", tpl_1128::<0x142407c80, 0x143ca6cf0, 0x142489ee0, 0x143c9bad4, 0x143ca6c78, 0x141a43db0> as *const ()), // sub_1404b0ef0
    Replacement::draft_throws(0x1404b1110, "r_1404b1110", fn_1404b1110 as *const (), Throws::Caught), // sub_1404b1110
    Replacement::draft_throws(0x1404b1260, "r_1404b1260", fn_1404b1260 as *const (), Throws::Discard), // sub_1404b1260
    Replacement::draft(0x1404b1350, "r_1404b1350", tpl_4431::<0x143ca7080, 0x20, 0x143c8d0d4, 0x8, 0x580e0, 0x143c8f63c, 0x141a3e6d0> as *const ()), // sub_1404b1350
    Replacement::draft_throws(0x1404b13d0, "r_1404b13d0", fn_1404b13d0 as *const (), Throws::Discard), // sub_1404b13d0
    Replacement::draft_throws(0x1404b16f0, "r_1404b16f0", fn_1404b16f0 as *const (), Throws::Discard), // sub_1404b16f0
    Replacement::draft(0x1404b8a10, "r_1404b8a10", fn_1404b8a10 as *const ()), // sub_1404b8a10
    Replacement::draft(0x1404b9310, "r_1404b9310", tpl_4771::<0x10, 0x1401de830, 0x142442de0> as *const ()), // sub_1404b9310
    Replacement::draft(0x1404c9160, "r_1404c9160", tpl_63099::<0x143ca8288, 0x142489ee0, 0x141a51440> as *const ()), // sub_1404c9160
    Replacement::draft(0x1404c92b0, "r_1404c92b0", tpl_61552::<0x8, 0x88, 0x10088, 0x141a51310> as *const ()), // sub_1404c92b0
    Replacement::draft(0x1404c9850, "r_1404c9850", fn_1404c9850 as *const ()), // sub_1404c9850
    Replacement::draft(0x1404c98f0, "r_1404c98f0", tpl_4842::<0x14397da68, 0x143c8d08c, 0x8, 0x580e0, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143ca83d8, 0x180a8, 0x18090, 0x142489ee0, 0x1404c99f0, 0x1426467e8, 0x1404c99d5> as *const ()), // sub_1404c98f0
    Replacement::draft(0x1404c99f0, "r_1404c99f0", tpl_1840::<0x30, 0x143ca83d8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1404c9a70> as *const ()), // sub_1404c99f0
    Replacement::draft(0x1404d5ca0, "r_1404d5ca0", tpl_4344::<0x143ca9640, 0x142407c80, 0x143c98134, 0x142489ee0, 0x141a59860> as *const ()), // sub_1404d5ca0
    Replacement::draft(0x1404d5e50, "r_1404d5e50", tpl_64074::<0x143c8ceec, 0x143c8d160, 0x142489ee0, 0x143c9d198, 0x141a59800> as *const ()), // sub_1404d5e50
    Replacement::draft(0x1404d6010, "r_1404d6010", fn_1404d6010 as *const ()), // sub_1404d6010
    Replacement::draft(0x1404d6250, "r_1404d6250", fn_1404d6250 as *const ()), // sub_1404d6250
    Replacement::draft(0x1404d6450, "r_1404d6450", tpl_61071::<0x143c9e294, 0x143c98134, 0x142489ee0, 0x143c988d0, 0x141a57f30> as *const ()), // sub_1404d6450
    Replacement::draft(0x1404d6640, "r_1404d6640", fn_1404d6640 as *const ()), // sub_1404d6640
    Replacement::draft(0x1404d68d0, "r_1404d68d0", fn_1404d68d0 as *const ()), // sub_1404d68d0
    Replacement::draft(0x1404d6b20, "r_1404d6b20", fn_1404d6b20 as *const ()), // sub_1404d6b20
    Replacement::draft(0x1404d6d60, "r_1404d6d60", fn_1404d6d60 as *const ()), // sub_1404d6d60
    Replacement::draft(0x1404d6fc0, "r_1404d6fc0", fn_1404d6fc0 as *const ()), // sub_1404d6fc0
    Replacement::draft(0x1404d71d0, "r_1404d71d0", tpl_62640::<0x143ca93a8, 0x143c9d198, 0x142489ee0, 0x143c988d0, 0x143c98134, 0x141a56e30> as *const ()), // sub_1404d71d0
    Replacement::draft_throws(0x1404d78a0, "r_1404d78a0", fn_1404d78a0 as *const (), Throws::Discard), // sub_1404d78a0
    Replacement::draft_throws(0x1404d7aa0, "r_1404d7aa0", tpl_4027::<0x143ca8eb0, 0x143c886c4, 0x143c886a8, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d0d0, 0x143ca9678, 0x143c886b8, 0x143c886c0, 0x141834a0c> as *const (), Throws::Discard), // sub_1404d7aa0
    Replacement::draft(0x1404d9080, "r_1404d9080", fn_1404d9080 as *const ()), // sub_1404d9080
    Replacement::draft_throws(0x1404e4460, "r_1404e4460", crate::shards::subsystems::fn_1404e4460 as *const (), Throws::Caught), // sub_1404e4460
    Replacement::draft_throws(0x1404e45e0, "r_1404e45e0", crate::shards::subsystems::fn_1404e45e0 as *const (), Throws::Caught), // sub_1404e45e0
    Replacement::draft(0x1404e47b0, "r_1404e47b0", fn_1404e47b0 as *const ()), // sub_1404e47b0
    Replacement::draft(0x1404e49e0, "r_1404e49e0", fn_1404e49e0 as *const ()), // sub_1404e49e0
    Replacement::draft(0x1404e4c10, "r_1404e4c10", fn_1404e4c10 as *const ()), // sub_1404e4c10
    Replacement::draft(0x1404e4f50, "r_1404e4f50", fn_1404e4f50 as *const ()), // sub_1404e4f50
    Replacement::draft(0x1404e5250, "r_1404e5250", tpl_64118::<0x143ca9f58, 0x143c8ceec, 0x143c8d08c, 0x143c91f94, 0x142489ee0, 0x143c8ec6c, 0x141a5cf70> as *const ()), // sub_1404e5250
    Replacement::draft(0x1404e54a0, "r_1404e54a0", fn_1404e54a0 as *const ()), // sub_1404e54a0
    Replacement::draft(0x1404e5740, "r_1404e5740", fn_1404e5740 as *const ()), // sub_1404e5740
    Replacement::draft(0x1404e5990, "r_1404e5990", tpl_62563::<0x143caaa50, 0x143c8d910, 0x143c98134, 0x142489ee0, 0x143c982ac, 0x141a5c300> as *const ()), // sub_1404e5990
    Replacement::draft_throws(0x1404e5bc0, "r_1404e5bc0", tpl_64121::<0x143c8ce88, 0x143c8de68, 0x142489ee0, 0x143c8ec6c, 0x143c8e658, 0x141a5bb00> as *const (), Throws::Discard), // sub_1404e5bc0
    Replacement::draft(0x1405285d0, "r_1405285d0", fn_1405285d0 as *const ()), // sub_1405285d0
    Replacement::draft(0x140528770, "r_140528770", fn_140528770 as *const ()), // sub_140528770
    Replacement::draft_throws(0x1405287e0, "r_1405287e0", fn_1405287e0 as *const (), Throws::Caught), // sub_1405287e0
    Replacement::draft(0x1405289a0, "r_1405289a0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140527120> as *const ()), // sub_1405289a0
    Replacement::draft(0x140528a80, "r_140528a80", fn_140528a80 as *const ()), // sub_140528a80
    Replacement::draft(0x140528d20, "r_140528d20", fn_140528d20 as *const ()), // sub_140528d20
    Replacement::draft(0x14053b760, "r_14053b760", tpl_3163::<0x38, 0x8, 0x14053dd10> as *const ()), // sub_14053b760
    Replacement::draft(0x14053b7a0, "r_14053b7a0", tpl_5222::<0x38, 0x20, 0x8, 0x14053e040> as *const ()), // sub_14053b7a0
    Replacement::draft(0x14053b7f0, "r_14053b7f0", tpl_3165::<0x38, 0x8, 0x14053e390> as *const ()), // sub_14053b7f0
    Replacement::draft(0x14053b830, "r_14053b830", tpl_3164::<0x38, 0x8, 0x14053e6d0> as *const ()), // sub_14053b830
    Replacement::draft(0x14053b870, "r_14053b870", tpl_1212::<0x8, 0x14053ea00> as *const ()), // sub_14053b870
    Replacement::draft_throws(0x14053b9a0, "r_14053b9a0", fn_14053b9a0 as *const (), Throws::Caught), // sub_14053b9a0
    Replacement::draft(0x14053bd10, "r_14053bd10", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143199740, 0x8, 0x20> as *const ()), // sub_14053bd10
    Replacement::draft(0x14053c030, "r_14053c030", fn_14053c030 as *const ()), // sub_14053c030
    Replacement::draft(0x14053cf00, "r_14053cf00", tpl_3176::<0x143cb05d8, 0x143cb0660, 0x141834a78, 0x143c8d120, 0x143c8d160, 0x143caf9bc, 0x143cafe00, 0x143c8e70c, 0x143c9626c, 0x143c9fde0, 0x143cb0680, 0x143cb06a0, 0x143cb06b0, 0x141834a0c> as *const ()), // sub_14053cf00
    Replacement::draft(0x14053d7f0, "r_14053d7f0", fn_14053d7f0 as *const ()), // sub_14053d7f0
    Replacement::draft(0x14053d9c0, "r_14053d9c0", fn_14053d9c0 as *const ()), // sub_14053d9c0
    Replacement::draft(0x14053db50, "r_14053db50", tpl_5227::<0x8, 0x18, 0x1401f2bc0> as *const ()), // sub_14053db50
    Replacement::draft(0x14053dd10, "r_14053dd10", tpl_3077::<0x143cb0170, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14053dd10
    Replacement::draft(0x14053e040, "r_14053e040", tpl_3072::<0x143c914b8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14053e040
    Replacement::draft(0x14053e390, "r_14053e390", tpl_3270::<0x143c95ed0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14053e390
    Replacement::draft(0x14053e6d0, "r_14053e6d0", tpl_1096::<0x143c95ad0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14053e6d0
    Replacement::draft(0x14053ea00, "r_14053ea00", tpl_1095::<0x143cb0938, 0x14020e8f0> as *const ()), // sub_14053ea00
    Replacement::draft(0x140548430, "r_140548430", fn_140548430 as *const ()), // sub_140548430
    Replacement::draft_throws(0x14054ee20, "r_14054ee20", fn_14054ee20 as *const (), Throws::Discard), // sub_14054ee20
    Replacement::draft_throws(0x14054f150, "r_14054f150", crate::shards::subsystems::fn_14054f150 as *const (), Throws::Caught), // sub_14054f150
    Replacement::draft(0x14054f260, "r_14054f260", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x14054d0d0> as *const ()), // sub_14054f260
    Replacement::draft(0x14054f300, "r_14054f300", fn_14054f300 as *const ()), // sub_14054f300
    Replacement::draft_throws(0x140556780, "r_140556780", tpl_64494::<0x88, 0x10088, 0x1405570a0, 0x141aa0560> as *const (), Throws::Discard), // sub_140556780
    Replacement::draft(0x140556860, "r_140556860", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x1405520a0> as *const ()), // sub_140556860
    Replacement::draft(0x140556900, "r_140556900", tpl_1131::<0x142407c80, 0x143c8d87c, 0x142489ee0, 0x141a9fd30> as *const ()), // sub_140556900
    Replacement::draft(0x140556a60, "r_140556a60", fn_140556a60 as *const ()), // sub_140556a60
    Replacement::draft_throws(0x140556c50, "r_140556c50", fn_140556c50 as *const (), Throws::Caught), // sub_140556c50
    Replacement::draft_throws(0x140556ec0, "r_140556ec0", fn_140556ec0 as *const (), Throws::Discard), // sub_140556ec0
    Replacement::draft(0x1405571a0, "r_1405571a0", fn_1405571a0 as *const ()), // sub_1405571a0
    Replacement::draft_throws(0x140557390, "r_140557390", crate::shards::subsystems::fn_140557390 as *const (), Throws::Caught), // sub_140557390
    Replacement::draft_throws(0x1405575f0, "r_1405575f0", crate::shards::subsystems::fn_1405575f0 as *const (), Throws::Caught), // sub_1405575f0
    Replacement::draft_throws(0x140557850, "r_140557850", fn_140557850 as *const (), Throws::Caught), // sub_140557850
    Replacement::draft_throws(0x1405579e0, "r_1405579e0", fn_1405579e0 as *const (), Throws::Caught), // sub_1405579e0
    Replacement::draft_throws(0x140557b70, "r_140557b70", fn_140557b70 as *const (), Throws::Caught), // sub_140557b70
    Replacement::draft_throws(0x140557d00, "r_140557d00", fn_140557d00 as *const (), Throws::Caught), // sub_140557d00
    Replacement::draft(0x140557e90, "r_140557e90", tpl_1212::<0x8, 0x14055a280> as *const ()), // sub_140557e90
    Replacement::draft(0x140557ec0, "r_140557ec0", fn_140557ec0 as *const ()), // sub_140557ec0
    Replacement::draft(0x14055c690, "r_14055c690", tpl_509::<0x9c, 0x98> as *const ()), // r::GameGlobalParameter<float>::r::GameGlobalParameter<float>::vfunc_4
    Replacement::draft(0x14055c6a0, "r_14055c6a0", tpl_585::<0x9c, 0xa8> as *const ()), // r::GameGlobalParameter<float>::r::GameGlobalParameter<float>::vfunc_3
    Replacement::draft(0x14055c6c0, "r_14055c6c0", tpl_509::<0xa8, 0x9c> as *const ()), // r::GameGlobalParameter<float>::r::GameGlobalParameter<float>::vfunc_2
    Replacement::draft(0x14055c6d0, "r_14055c6d0", fn_14055c6d0 as *const ()), // r::GameGlobalParameter<float>::r::GameGlobalParameter<float>::vfunc_1
    Replacement::draft(0x14055c6e0, "r_14055c6e0", fn_14055c6e0 as *const ()), // r::GameGlobalParameter<float>::r::GameGlobalParameter<float>::vfunc_0
    Replacement::draft(0x140576d50, "r_140576d50", tpl_1130::<0x142407c80, 0x141ab7c40> as *const ()), // sub_140576d50
    Replacement::draft(0x140576e60, "r_140576e60", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140575770> as *const ()), // sub_140576e60
    Replacement::draft(0x140576f40, "r_140576f40", fn_140576f40 as *const ()), // sub_140576f40
    Replacement::draft(0x14059a200, "r_14059a200", fn_14059a200 as *const ()), // sub_14059a200
    Replacement::draft(0x14059a2e0, "r_14059a2e0", tpl_5596::<0x120> as *const ()), // sub_14059a2e0
    Replacement::draft_throws(0x14059a320, "r_14059a320", fn_14059a320 as *const (), Throws::Discard), // sub_14059a320
    Replacement::draft(0x14059a5c0, "r_14059a5c0", fn_14059a5c0 as *const ()), // sub_14059a5c0
    Replacement::draft(0x14059a820, "r_14059a820", fn_14059a820 as *const ()), // sub_14059a820
    Replacement::draft(0x14059aa20, "r_14059aa20", tpl_5599::<0x143c8d110, 0x143c8d0d4, 0x142407c80, 0x143c8d160, 0x142489ee0, 0x141ac9410> as *const ()), // sub_14059aa20
    Replacement::draft(0x14059abd0, "r_14059abd0", fn_14059abd0 as *const ()), // sub_14059abd0
    Replacement::draft_throws(0x14059ae10, "r_14059ae10", fn_14059ae10 as *const (), Throws::Discard), // sub_14059ae10
    Replacement::draft_throws(0x14059b150, "r_14059b150", fn_14059b150 as *const (), Throws::Discard), // sub_14059b150
    Replacement::draft_throws(0x14059b440, "r_14059b440", fn_14059b440 as *const (), Throws::Discard), // sub_14059b440
    Replacement::draft(0x14059b960, "r_14059b960", fn_14059b960 as *const ()), // sub_14059b960
    Replacement::draft_throws(0x14059bca0, "r_14059bca0", fn_14059bca0 as *const (), Throws::Caught), // sub_14059bca0
    Replacement::draft(0x14059bda0, "r_14059bda0", fn_14059bda0 as *const ()), // sub_14059bda0
    Replacement::draft(0x14059c420, "r_14059c420", fn_14059c420 as *const ()), // sub_14059c420
    Replacement::draft(0x14059cb60, "r_14059cb60", tpl_3213::<0x142407c80, 0x143cb5f08, 0x142489ee0, 0x143c8d87c, 0x141ac9230> as *const ()), // sub_14059cb60
    Replacement::draft(0x14059cd10, "r_14059cd10", fn_14059cd10 as *const ()), // sub_14059cd10
    Replacement::draft_throws(0x14059cfd0, "r_14059cfd0", tpl_5384::<0x14241c0a0, 0x14240aa00, 0x14021e830, 0x14240ab10, 0x14241c190> as *const (), Throws::Discard), // sub_14059cfd0
    Replacement::draft(0x14059d160, "r_14059d160", fn_14059d160 as *const ()), // sub_14059d160
    Replacement::draft(0x14059d430, "r_14059d430", fn_14059d430 as *const ()), // sub_14059d430
    Replacement::draft(0x14059d780, "r_14059d780", fn_14059d780 as *const ()), // sub_14059d780
    Replacement::draft(0x14059d820, "r_14059d820", fn_14059d820 as *const ()), // sub_14059d820
    Replacement::draft(0x14059dfe0, "r_14059dfe0", fn_14059dfe0 as *const ()), // sub_14059dfe0
    Replacement::draft(0x14059e2c0, "r_14059e2c0", fn_14059e2c0 as *const ()), // sub_14059e2c0
    Replacement::draft(0x14059eba0, "r_14059eba0", fn_14059eba0 as *const ()), // sub_14059eba0
    Replacement::draft(0x14059ede0, "r_14059ede0", fn_14059ede0 as *const ()), // sub_14059ede0
    Replacement::draft(0x14059f490, "r_14059f490", fn_14059f490 as *const ()), // sub_14059f490
    Replacement::draft(0x14059f730, "r_14059f730", fn_14059f730 as *const ()), // sub_14059f730
    Replacement::draft(0x14059f990, "r_14059f990", fn_14059f990 as *const ()), // sub_14059f990
    Replacement::draft(0x14059feb0, "r_14059feb0", fn_14059feb0 as *const ()), // sub_14059feb0
    Replacement::draft(0x1405a0070, "r_1405a0070", fn_1405a0070 as *const ()), // sub_1405a0070
    Replacement::draft(0x1405a0400, "r_1405a0400", fn_1405a0400 as *const ()), // sub_1405a0400
    Replacement::draft(0x1405a04d0, "r_1405a04d0", fn_1405a04d0 as *const ()), // sub_1405a04d0
    Replacement::draft(0x1405a05a0, "r_1405a05a0", fn_1405a05a0 as *const ()), // sub_1405a05a0
    Replacement::draft(0x1405a0690, "r_1405a0690", fn_1405a0690 as *const ()), // sub_1405a0690
    Replacement::draft(0x1405a0b30, "r_1405a0b30", tpl_5619::<0x14240a730> as *const ()), // sub_1405a0b30
    Replacement::draft(0x1405a0f10, "r_1405a0f10", fn_1405a0f10 as *const ()), // sub_1405a0f10
    Replacement::draft(0x1405a11c0, "r_1405a11c0", fn_1405a11c0 as *const ()), // sub_1405a11c0
    Replacement::draft(0x1405a1360, "r_1405a1360", fn_1405a1360 as *const ()), // sub_1405a1360
    Replacement::draft(0x1405a1ac0, "r_1405a1ac0", fn_1405a1ac0 as *const ()), // sub_1405a1ac0
    Replacement::draft(0x1405a1e10, "r_1405a1e10", fn_1405a1e10 as *const ()), // sub_1405a1e10
    Replacement::draft(0x1405a2490, "r_1405a2490", fn_1405a2490 as *const ()), // sub_1405a2490
    Replacement::draft(0x1405a2640, "r_1405a2640", fn_1405a2640 as *const ()), // sub_1405a2640
    Replacement::draft(0x1405a30d0, "r_1405a30d0", fn_1405a30d0 as *const ()), // sub_1405a30d0
    Replacement::draft(0x1405a3410, "r_1405a3410", fn_1405a3410 as *const ()), // sub_1405a3410
    Replacement::draft(0x1405a3840, "r_1405a3840", tpl_5619::<0x14240aa00> as *const ()), // sub_1405a3840
    Replacement::draft(0x1405a3c20, "r_1405a3c20", fn_1405a3c20 as *const ()), // sub_1405a3c20
    Replacement::draft(0x1405a3f00, "r_1405a3f00", fn_1405a3f00 as *const ()), // sub_1405a3f00
    Replacement::draft(0x1405a4010, "r_1405a4010", fn_1405a4010 as *const ()), // sub_1405a4010
    Replacement::draft(0x1405a4160, "r_1405a4160", fn_1405a4160 as *const ()), // sub_1405a4160
    Replacement::draft(0x1405a4250, "r_1405a4250", fn_1405a4250 as *const ()), // sub_1405a4250
    Replacement::draft(0x1405a4360, "r_1405a4360", fn_1405a4360 as *const ()), // sub_1405a4360
    Replacement::draft(0x1405a44b0, "r_1405a44b0", tpl_1663::<0x8, 0x14058f060, 0x1401e66f0> as *const ()), // sub_1405a44b0
    Replacement::draft(0x1405a44f0, "r_1405a44f0", tpl_1663::<0x8, 0x14058efd0, 0x1401e66f0> as *const ()), // sub_1405a44f0
    Replacement::draft(0x1405a4530, "r_1405a4530", tpl_1663::<0x8, 0x14058ef40, 0x1401e66f0> as *const ()), // sub_1405a4530
    Replacement::draft(0x1405a4570, "r_1405a4570", tpl_1663::<0x8, 0x140592780, 0x1401e66f0> as *const ()), // sub_1405a4570
    Replacement::draft(0x1405a45b0, "r_1405a45b0", fn_1405a45b0 as *const ()), // sub_1405a45b0
    Replacement::draft(0x1405a46b0, "r_1405a46b0", fn_1405a46b0 as *const ()), // sub_1405a46b0
    Replacement::draft(0x1405c3030, "r_1405c3030", fn_1405c3030 as *const ()), // sub_1405c3030
    Replacement::draft(0x1405c3100, "r_1405c3100", fn_1405c3100 as *const ()), // sub_1405c3100
    Replacement::draft(0x1405cae40, "r_1405cae40", fn_1405cae40 as *const ()), // sub_1405cae40
    Replacement::draft(0x1405caea0, "r_1405caea0", fn_1405caea0 as *const ()), // sub_1405caea0
    Replacement::draft(0x1405caec0, "r_1405caec0", tpl_64969::<0x143cb82c8, 0x142489ee0, 0x141afcce0> as *const ()), // sub_1405caec0
    Replacement::draft(0x1405cb010, "r_1405cb010", tpl_62741::<0x143c8d120, 0x142489ee0, 0x143c8efb0, 0x143cb8c4c, 0x141afd3e0> as *const ()), // sub_1405cb010
    Replacement::draft(0x1405cb220, "r_1405cb220", tpl_4169::<0x143c8cef8, 0x142407c80, 0x143c8cefc, 0x142489ee0, 0x141b025b0> as *const ()), // sub_1405cb220
    Replacement::draft(0x1405cb3a0, "r_1405cb3a0", tpl_3172::<0x143c8cefc, 0x5e308, 0x143c8cef8, 0x580e0, 0x10, 0x8, 0x18, 0x141b02570, 0x1405cd120> as *const ()), // sub_1405cb3a0
    Replacement::draft_throws(0x1405cb450, "r_1405cb450", fn_1405cb450 as *const (), Throws::Discard), // sub_1405cb450
    Replacement::draft_throws(0x1405cb660, "r_1405cb660", fn_1405cb660 as *const (), Throws::Discard), // sub_1405cb660
    Replacement::draft(0x1405cb800, "r_1405cb800", fn_1405cb800 as *const ()), // sub_1405cb800
    Replacement::draft(0x1405cba90, "r_1405cba90", tpl_62741::<0x143c8de68, 0x142489ee0, 0x143c8d120, 0x143c8e658, 0x141afdc70> as *const ()), // sub_1405cba90
    Replacement::draft(0x1405cbca0, "r_1405cbca0", fn_1405cbca0 as *const ()), // sub_1405cbca0
    Replacement::draft_throws(0x1405cbf10, "r_1405cbf10", tpl_61190::<0x143cb8458, 0x143c8cefc, 0x142489ee0, 0x143cb82c8, 0x143c8cf5c, 0x141afd580> as *const (), Throws::Discard), // sub_1405cbf10
    Replacement::draft_throws(0x1405cc180, "r_1405cc180", fn_1405cc180 as *const (), Throws::Discard), // sub_1405cc180
    Replacement::draft_throws(0x1405cc390, "r_1405cc390", fn_1405cc390 as *const (), Throws::Discard), // sub_1405cc390
    Replacement::draft_throws(0x1405cd5e0, "r_1405cd5e0", fn_1405cd5e0 as *const (), Throws::Discard), // sub_1405cd5e0
    Replacement::draft(0x1405cdc70, "r_1405cdc70", fn_1405cdc70 as *const ()), // sub_1405cdc70
    Replacement::draft(0x1405ce0d0, "r_1405ce0d0", fn_1405ce0d0 as *const ()), // sub_1405ce0d0
    Replacement::draft(0x1405ce390, "r_1405ce390", fn_1405ce390 as *const ()), // sub_1405ce390
    Replacement::draft(0x1405ce5d0, "r_1405ce5d0", fn_1405ce5d0 as *const ()), // sub_1405ce5d0
    Replacement::draft(0x1405ce9f0, "r_1405ce9f0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431d8a08, 0x8, 0x20> as *const ()), // sub_1405ce9f0
    Replacement::draft(0x14061a4e0, "r_14061a4e0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b3c950> as *const ()), // sub_14061a4e0
    Replacement::draft(0x14061a5d0, "r_14061a5d0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b3c790> as *const ()), // sub_14061a5d0
    Replacement::draft(0x14061a6c0, "r_14061a6c0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b3c8c0> as *const ()), // sub_14061a6c0
    Replacement::draft(0x14061a7b0, "r_14061a7b0", tpl_61552::<0x8, 0x88, 0x10088, 0x141b3c530> as *const ()), // sub_14061a7b0
    Replacement::draft(0x14061a8a0, "r_14061a8a0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143201f40, 0x8, 0x20> as *const ()), // sub_14061a8a0
    Replacement::draft(0x14061a940, "r_14061a940", fn_14061a940 as *const ()), // sub_14061a940
    Replacement::draft(0x14061ae90, "r_14061ae90", fn_14061ae90 as *const ()), // sub_14061ae90
    Replacement::draft(0x14061b0a0, "r_14061b0a0", tpl_4691::<0x143cbd174, 0x143cbd6d0, 0x141834a78, 0x143c8ec20, 0x143c8d120, 0x143c8e840, 0x143c8d160, 0x143cbd004, 0x143c9a4e8, 0x143cbd4c8, 0x143cbd6f0, 0x143cbd710, 0x143cbd720, 0x141834a0c> as *const ()), // sub_14061b0a0
    Replacement::draft(0x14061b240, "r_14061b240", fn_14061b240 as *const ()), // sub_14061b240
    Replacement::draft(0x14061b520, "r_14061b520", fn_14061b520 as *const ()), // sub_14061b520
    Replacement::draft(0x14061b890, "r_14061b890", fn_14061b890 as *const ()), // sub_14061b890
    Replacement::draft(0x14061ba50, "r_14061ba50", fn_14061ba50 as *const ()), // sub_14061ba50
    Replacement::draft(0x14061bd10, "r_14061bd10", tpl_1095::<0x143cbd85c, 0x14020e8f0> as *const ()), // sub_14061bd10
    Replacement::draft(0x14061bef0, "r_14061bef0", tpl_3072::<0x143c95048, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14061bef0
    Replacement::draft(0x14061cc50, "r_14061cc50", fn_14061cc50 as *const ()), // sub_14061cc50
    Replacement::draft_throws(0x14062a3e0, "r_14062a3e0", fn_14062a3e0 as *const (), Throws::Discard), // sub_14062a3e0
    Replacement::draft_throws(0x14062a5d0, "r_14062a5d0", fn_14062a5d0 as *const (), Throws::Discard), // sub_14062a5d0
    Replacement::draft_throws(0x14062a820, "r_14062a820", fn_14062a820 as *const (), Throws::Discard), // sub_14062a820
    Replacement::draft_throws(0x14062a9f0, "r_14062a9f0", tpl_5255::<0x142407c80, 0x143cbe514, 0x143cbe47c, 0x142489ee0, 0x143c8d160, 0x143cb1824, 0x141b4b4b0> as *const (), Throws::Discard), // sub_14062a9f0
    Replacement::draft(0x14062ae90, "r_14062ae90", fn_14062ae90 as *const ()), // sub_14062ae90
    Replacement::draft(0x14062b3e0, "r_14062b3e0", fn_14062b3e0 as *const ()), // sub_14062b3e0
    Replacement::draft_throws(0x14064f1b0, "r_14064f1b0", tpl_1330::<0x8, 0x140650c80, 0x143cc098c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14064f1b0
    Replacement::draft(0x1407334c0, "r_1407334c0", fn_1407334c0 as *const ()), // r::RedirectedClassTypeInfoImplementation::r::RedirectedClassTypeInfoImplementation::vfunc_1
    Replacement::draft(0x1408f0480, "r_1408f0480", fn_1408f0480 as *const ()), // r::ContentEntityBase::r::ContentEntityBase::vfunc_0
    Replacement::draft_throws(0x140c33aa0, "r_140c33aa0", crate::shards::subsystems::fn_140c33aa0 as *const (), Throws::Caught), // r::BalancedContent<content::ProjectileBalancedData>::r::BalancedContent<content::ProjectileBalancedData>::vfunc_1
    Replacement::draft_throws(0x140c33c60, "r_140c33c60", crate::shards::subsystems::fn_140c33c60 as *const (), Throws::Caught), // r::BalancedContent<content::GameWeaponKickbackBalancedData>::r::BalancedContent<content::GameWeaponKickbackBalancedData>
    Replacement::draft_throws(0x140c33e20, "r_140c33e20", crate::shards::subsystems::fn_140c33e20 as *const (), Throws::Caught), // r::BalancedContent<content::GameWeaponDispersionBalancedData>::r::BalancedContent<content::GameWeaponDispersionBalancedD
    Replacement::draft(0x140c33fe0, "r_140c33fe0", fn_140c33fe0 as *const ()), // sub_140c33fe0
    Replacement::draft_throws(0x140c340c0, "r_140c340c0", crate::shards::subsystems::fn_140c340c0 as *const (), Throws::Caught), // r::BalancedContent<content::GameRayCastDamageActionBalancedData>::r::BalancedContent<content::GameRayCastDamageActionBal
    Replacement::draft_throws(0x140c34280, "r_140c34280", fn_140c34280 as *const (), Throws::Caught), // r::BalancedContent<content::GameAnimationInputDamageBalancedData>::r::BalancedContent<content::GameAnimationInputDamageB
    Replacement::draft(0x140c345c0, "r_140c345c0", fn_140c345c0 as *const ()), // sub_140c345c0
    Replacement::draft_throws(0x140c34690, "r_140c34690", fn_140c34690 as *const (), Throws::Caught), // r::BalancedContent<content::AbilityDodgeBalancedData>::r::BalancedContent<content::AbilityDodgeBalancedData>::vfunc_1
    Replacement::draft(0x140c34940, "r_140c34940", fn_140c34940 as *const ()), // r::BalancedContent<content::ProjectileBalancedData>::r::BalancedContent<content::ProjectileBalancedData>::vfunc_0
    Replacement::draft(0x140c34980, "r_140c34980", fn_140c34980 as *const ()), // r::BalancedContent<content::GameWeaponKickbackBalancedData>::r::BalancedContent<content::GameWeaponKickbackBalancedData>
    Replacement::draft(0x140c349c0, "r_140c349c0", tpl_15235::<0x1432c9630, 0x10, 0x141834d30> as *const ()), // r::BalancedContent<content::GameWeaponDispersionBalancedData>::r::BalancedContent<content::GameWeaponDispersionBalancedD
    Replacement::draft(0x140c34a00, "r_140c34a00", fn_140c34a00 as *const ()), // r::BalancedContent<content::AbilityDodgeBalancedData>::r::BalancedContent<content::AbilityDodgeBalancedData>::vfunc_0
    Replacement::draft(0x140c34a40, "r_140c34a40", fn_140c34a40 as *const ()), // r::BalancedContent<content::GameAnimationInputDamageBalancedData>::r::BalancedContent<content::GameAnimationInputDamageB
    Replacement::draft(0x140c34a80, "r_140c34a80", fn_140c34a80 as *const ()), // r::BalancedContentBase::r::BalancedContentBase::vfunc_0
    Replacement::draft_throws(0x140f2b670, "r_140f2b670", fn_140f2b670 as *const (), Throws::Caught), // sub_140f2b670
    Replacement::draft(0x140f2b860, "r_140f2b860", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140f29f50> as *const ()), // sub_140f2b860
    Replacement::draft(0x140f2b940, "r_140f2b940", tpl_64224::<0x143c900d8, 0x140f2b1c0, 0x141dc1140> as *const ()), // sub_140f2b940
    Replacement::draft(0x140f2bdd0, "r_140f2bdd0", fn_140f2bdd0 as *const ()), // sub_140f2bdd0
    Replacement::draft(0x140f6db50, "r_140f6db50", fn_140f6db50 as *const ()), // sub_140f6db50
    Replacement::draft(0x140f6dd20, "r_140f6dd20", fn_140f6dd20 as *const ()), // sub_140f6dd20
    Replacement::draft(0x140f6e0d0, "r_140f6e0d0", fn_140f6e0d0 as *const ()), // sub_140f6e0d0
    Replacement::draft(0x140f742b0, "r_140f742b0", fn_140f742b0 as *const ()), // sub_140f742b0
    Replacement::draft(0x140f74b40, "r_140f74b40", tpl_6090::<0x143cc708c, 0x143cc6fd0, 0x141834a78, 0x143cc6d1c, 0x143cc6f80, 0x143cc6d00, 0x143c93078, 0x143c8d120, 0x143c8d160, 0x143cc6db8, 0x143cc6ff0, 0x143cc7010, 0x143cc7020, 0x141834a0c> as *const ()), // sub_140f74b40
    Replacement::draft(0x140f82a60, "r_140f82a60", fn_140f82a60 as *const ()), // sub_140f82a60
    Replacement::draft(0x140f82b80, "r_140f82b80", fn_140f82b80 as *const ()), // sub_140f82b80
    Replacement::draft_throws(0x140f9f320, "r_140f9f320", tpl_5925::<0x8, 0x20, 0x143cca3c8, 0x143cca3c4, 0x580e0, 0x143c90de0, 0x143cca1ec, 0x141e45590> as *const (), Throws::Discard), // sub_140f9f320
    Replacement::draft_throws(0x140f9f3f0, "r_140f9f3f0", fn_140f9f3f0 as *const (), Throws::Discard), // sub_140f9f3f0
    Replacement::draft(0x140f9f480, "r_140f9f480", tpl_4431::<0x143cca3bc, 0x20, 0x143c90de0, 0x8, 0x580e0, 0x143cca1f4, 0x141e447f0> as *const ()), // sub_140f9f480
    Replacement::draft(0x140f9f500, "r_140f9f500", tpl_4431::<0x143cca3b8, 0x20, 0x143c90de0, 0x8, 0x580e0, 0x143c90e64, 0x141e46c00> as *const ()), // sub_140f9f500
    Replacement::draft(0x140f9fb40, "r_140f9fb40", fn_140f9fb40 as *const ()), // sub_140f9fb40
    Replacement::draft(0x140fa00c0, "r_140fa00c0", tpl_5912::<0xfffffffffffffffe, 0x141e43da0, 0x142442de0> as *const ()), // sub_140fa00c0
    Replacement::draft(0x140fa0110, "r_140fa0110", tpl_4170::<0x141e41b20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e43ed0, 0x142442de0> as *const ()), // sub_140fa0110
    Replacement::draft(0x140fa0230, "r_140fa0230", tpl_4170::<0x141e41ba0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e44050, 0x142442de0> as *const ()), // sub_140fa0230
    Replacement::draft(0x140fa9ca0, "r_140fa9ca0", fn_140fa9ca0 as *const ()), // sub_140fa9ca0
    Replacement::draft(0x140fbd310, "r_140fbd310", fn_140fbd310 as *const ()), // sub_140fbd310
    Replacement::draft(0x140fbd430, "r_140fbd430", fn_140fbd430 as *const ()), // sub_140fbd430
    Replacement::draft(0x140fbd4b0, "r_140fbd4b0", fn_140fbd4b0 as *const ()), // sub_140fbd4b0
    Replacement::draft(0x140fbd530, "r_140fbd530", fn_140fbd530 as *const ()), // sub_140fbd530
    Replacement::draft(0x140fbd580, "r_140fbd580", fn_140fbd580 as *const ()), // sub_140fbd580
    Replacement::draft(0x140fbd5d0, "r_140fbd5d0", tpl_19579::<0x14330bca8, 0x10> as *const ()), // sub_140fbd5d0
    Replacement::draft(0x140fbd5e0, "r_140fbd5e0", fn_140fbd5e0 as *const ()), // sub_140fbd5e0
    Replacement::draft(0x140fbd670, "r_140fbd670", fn_140fbd670 as *const ()), // sub_140fbd670
    Replacement::draft(0x140fbd7e0, "r_140fbd7e0", fn_140fbd7e0 as *const ()), // sub_140fbd7e0
    Replacement::draft(0x140fbd950, "r_140fbd950", fn_140fbd950 as *const ()), // sub_140fbd950
    Replacement::draft_throws(0x140fbdac0, "r_140fbdac0", fn_140fbdac0 as *const (), Throws::Discard), // sub_140fbdac0
    Replacement::draft(0x140fbdc20, "r_140fbdc20", tpl_1094::<0x143ccbf30, 0x140492c90, 0x14020e8f0> as *const ()), // sub_140fbdc20
    Replacement::draft(0x140fe0c40, "r_140fe0c40", fn_140fe0c40 as *const ()), // sub_140fe0c40
    Replacement::draft(0x140ff27f0, "r_140ff27f0", fn_140ff27f0 as *const ()), // sub_140ff27f0
    Replacement::draft(0x141012880, "r_141012880", tpl_1955::<0x10, 0x141008420> as *const ()), // sub_141012880
    Replacement::draft(0x1410128c0, "r_1410128c0", fn_1410128c0 as *const ()), // sub_1410128c0
    Replacement::draft(0x141012960, "r_141012960", fn_141012960 as *const ()), // sub_141012960
    Replacement::draft_throws(0x141012a20, "r_141012a20", tpl_78312::<0x143c914bc, 0x142489ee0, 0x143c8f6e4, 0x143c8fab8, 0x141e98c80> as *const (), Throws::Discard), // sub_141012a20
    Replacement::draft(0x141012c20, "r_141012c20", fn_141012c20 as *const ()), // sub_141012c20
    Replacement::draft_throws(0x141012e20, "r_141012e20", tpl_19963::<0x143cd12b0, 0x20, 0x143c8d08c, 0x8, 0x580e0, 0x143c8f63c, 0x141e98490> as *const (), Throws::Discard), // sub_141012e20
    Replacement::draft_throws(0x141012ea0, "r_141012ea0", tpl_62736::<0x143c8f63c, 0x143c914bc, 0x142489ee0, 0x143c8f628, 0x141e96c50> as *const (), Throws::Discard), // sub_141012ea0
    Replacement::draft(0x141013060, "r_141013060", tpl_3028::<0x143cd12a8, 0x20, 0x8, 0x143c8f63c, 0x580e0, 0x141e99650> as *const ()), // sub_141013060
    Replacement::draft(0x1410130d0, "r_1410130d0", tpl_78314::<0x143c94b44, 0x143c94b30, 0x141012380, 0x141e98f10> as *const ()), // sub_1410130d0
    Replacement::draft(0x141013380, "r_141013380", tpl_78314::<0x143c94b44, 0x143c94b30, 0x141012380, 0x141e98e10> as *const ()), // sub_141013380
    Replacement::draft_throws(0x141013630, "r_141013630", fn_141013630 as *const (), Throws::Discard), // sub_141013630
    Replacement::draft(0x141013700, "r_141013700", tpl_3172::<0x143c93398, 0x5e308, 0x143cd0f1c, 0x580e0, 0x10, 0x8, 0x18, 0x141e99c10, 0x141014d20> as *const ()), // sub_141013700
    Replacement::draft(0x1410137b0, "r_1410137b0", tpl_1898::<0x88, 0x10088, 0x141e9abf0> as *const ()), // sub_1410137b0
    Replacement::draft_throws(0x141013840, "r_141013840", fn_141013840 as *const (), Throws::Discard), // sub_141013840
    Replacement::draft_throws(0x141013a40, "r_141013a40", fn_141013a40 as *const (), Throws::Discard), // sub_141013a40
    Replacement::draft_throws(0x141013b10, "r_141013b10", fn_141013b10 as *const (), Throws::Discard), // sub_141013b10
    Replacement::draft_throws(0x141013c50, "r_141013c50", tpl_5922::<0x141017620, 0x143cd0e18, 0x1420af120, 0x140215750, 0x140ff0a40, 0x143cd0f18, 0x140215610, 0x143c8ce94, 0x1420a28e0> as *const (), Throws::Discard), // sub_141013c50
    Replacement::draft(0x141013de0, "r_141013de0", tpl_1898::<0x88, 0x10088, 0x141e96f80> as *const ()), // sub_141013de0
    Replacement::draft_throws(0x141013e70, "r_141013e70", fn_141013e70 as *const (), Throws::Discard), // sub_141013e70
    Replacement::draft_throws(0x14102a4c0, "r_14102a4c0", tpl_1330::<0x8, 0x14102b240, 0x143cd1cac, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_14102a4c0
    Replacement::draft(0x141049520, "r_141049520", fn_141049520 as *const ()), // sub_141049520
    Replacement::draft(0x1410495e0, "r_1410495e0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141eadd50> as *const ()), // sub_1410495e0
    Replacement::draft(0x1410496d0, "r_1410496d0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141ead030> as *const ()), // sub_1410496d0
    Replacement::draft(0x1410497c0, "r_1410497c0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141eacf00> as *const ()), // sub_1410497c0
    Replacement::draft(0x1410498b0, "r_1410498b0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141eacdd0> as *const ()), // sub_1410498b0
    Replacement::draft(0x1410499a0, "r_1410499a0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141eacca0> as *const ()), // sub_1410499a0
    Replacement::draft_throws(0x141049a90, "r_141049a90", fn_141049a90 as *const (), Throws::Caught), // sub_141049a90
    Replacement::draft_throws(0x141049c00, "r_141049c00", fn_141049c00 as *const (), Throws::Caught), // sub_141049c00
    Replacement::draft(0x141049ce0, "r_141049ce0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141eadbf0> as *const ()), // sub_141049ce0
    Replacement::draft(0x141049dd0, "r_141049dd0", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141eada90> as *const ()), // sub_141049dd0
    Replacement::draft_throws(0x141049ec0, "r_141049ec0", fn_141049ec0 as *const (), Throws::Discard), // sub_141049ec0
    Replacement::draft_throws(0x14104a070, "r_14104a070", fn_14104a070 as *const (), Throws::Discard), // sub_14104a070
    Replacement::draft(0x14104a270, "r_14104a270", tpl_77684::<0x143c8f7a0, 0x142489ee0, 0x143cd444c, 0x141eb0580> as *const ()), // sub_14104a270
    Replacement::draft(0x14104a620, "r_14104a620", tpl_77668::<0x143c8f7a0, 0x142489ee0, 0x143c8ec20, 0x143cd4970, 0x141eac8c0> as *const ()), // sub_14104a620
    Replacement::draft(0x14104a820, "r_14104a820", fn_14104a820 as *const ()), // sub_14104a820
    Replacement::draft_throws(0x14104acc0, "r_14104acc0", tpl_5214::<0x143cd4bac, 0x20, 0x8, 0x141eaf8a0> as *const (), Throws::Discard), // sub_14104acc0
    Replacement::draft(0x14104ad20, "r_14104ad20", tpl_62741::<0x143cd445c, 0x142489ee0, 0x143c8f7a0, 0x143c8ec20, 0x141eb09b0> as *const ()), // sub_14104ad20
    Replacement::draft(0x14104af30, "r_14104af30", tpl_62640::<0x143cd4460, 0x143cd445c, 0x142489ee0, 0x143c8f7a0, 0x143cccd90, 0x141eb08e0> as *const ()), // sub_14104af30
    Replacement::draft(0x14104b1a0, "r_14104b1a0", tpl_4148::<0x143cd5084, 0x20, 0x8, 0x141eac4e0> as *const ()), // sub_14104b1a0
    Replacement::draft(0x14104b200, "r_14104b200", tpl_4148::<0x143cd446c, 0x20, 0x8, 0x141eaf160> as *const ()), // sub_14104b200
    Replacement::draft(0x14104b260, "r_14104b260", fn_14104b260 as *const ()), // sub_14104b260
    Replacement::draft(0x14104b2e0, "r_14104b2e0", tpl_5910::<0x143cd4750, 0x143c904a0, 0x20, 0x143cd53a4, 0x8, 0x580e0, 0x143c8d904, 0x141eaf620> as *const ()), // sub_14104b2e0
    Replacement::draft(0x14104b370, "r_14104b370", tpl_4148::<0x143cd4c40, 0x20, 0x8, 0x141eaf3a0> as *const ()), // sub_14104b370
    Replacement::draft(0x14104b460, "r_14104b460", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143365010, 0x8, 0x20> as *const ()), // sub_14104b460
    Replacement::draft(0x14104bf60, "r_14104bf60", fn_14104bf60 as *const ()), // sub_14104bf60
    Replacement::draft(0x14104cb80, "r_14104cb80", fn_14104cb80 as *const ()), // sub_14104cb80
    Replacement::draft(0x14104cd90, "r_14104cd90", fn_14104cd90 as *const ()), // sub_14104cd90
    Replacement::draft(0x14104d3e0, "r_14104d3e0", fn_14104d3e0 as *const ()), // sub_14104d3e0
    Replacement::draft_throws(0x141075340, "r_141075340", tpl_4016::<0x143cd6cc0, 0x143c8c0a0, 0x143c8c048, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8d0d0, 0x143c8d160, 0x143c8c058, 0x143c8c060, 0x141834a0c> as *const (), Throws::Discard), // sub_141075340
    Replacement::draft_throws(0x1410754a0, "r_1410754a0", tpl_847::<0x143cd6d08, 0x143c8c0a4, 0x143c8c038, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c95490, 0x141834a0c> as *const (), Throws::Discard), // sub_1410754a0
    Replacement::draft(0x141075a40, "r_141075a40", fn_141075a40 as *const ()), // sub_141075a40
    Replacement::draft(0x141076700, "r_141076700", fn_141076700 as *const ()), // sub_141076700
    Replacement::draft(0x1410767a0, "r_1410767a0", fn_1410767a0 as *const ()), // sub_1410767a0
    Replacement::draft(0x1410767d0, "r_1410767d0", tpl_17505::<0x8, 0x1410768a0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410767d0
    Replacement::draft(0x1410768a0, "r_1410768a0", fn_1410768a0 as *const ()), // sub_1410768a0
    Replacement::draft(0x141076910, "r_141076910", fn_141076910 as *const ()), // sub_141076910
    Replacement::draft_throws(0x1410769f0, "r_1410769f0", tpl_20312::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8dd14, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1433735c0, 0x141ef4820, 0x141076b55, 0x141076b89, 0x141076b9b, 0x141076bb0, 0x141076b55, 0x141076b89, 0x141076b9b, 0x141076bb0> as *const (), Throws::Discard), // sub_1410769f0
    Replacement::draft_throws(0x141076bc0, "r_141076bc0", tpl_20247::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cccd90, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1433735c0, 0x141ef4820, 0x141076d21, 0x141076d55, 0x141076d67, 0x141076d7c, 0x141076d21, 0x141076d55, 0x141076d67, 0x141076d7c> as *const (), Throws::Discard), // sub_141076bc0
    Replacement::draft_throws(0x141076d80, "r_141076d80", fn_141076d80 as *const (), Throws::Discard), // sub_141076d80
    Replacement::draft(0x141076f40, "r_141076f40", fn_141076f40 as *const ()), // sub_141076f40
    Replacement::draft_throws(0x141077010, "r_141077010", tpl_20250::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd6e5c, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1433735f8, 0x141ef4820, 0x14107716d, 0x1410771a1, 0x1410771b3, 0x1410771c8, 0x14107716d, 0x1410771a1, 0x1410771b3, 0x1410771c8> as *const (), Throws::Discard), // sub_141077010
    Replacement::draft_throws(0x1410771d0, "r_1410771d0", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd6e6c, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1433735e8, 0x141ef4820, 0x141077331, 0x141077365, 0x141077377, 0x14107738c, 0x141077331, 0x141077365, 0x141077377, 0x14107738c> as *const (), Throws::Discard), // sub_1410771d0
    Replacement::draft_throws(0x141077390, "r_141077390", fn_141077390 as *const (), Throws::Discard), // sub_141077390
    Replacement::draft_throws(0x141077550, "r_141077550", tpl_20312::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8de68, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x143373570, 0x141ef4820, 0x1410776b5, 0x1410776e9, 0x1410776fb, 0x141077710, 0x1410776b5, 0x1410776e9, 0x1410776fb, 0x141077710> as *const (), Throws::Discard), // sub_141077550
    Replacement::draft(0x141077720, "r_141077720", fn_141077720 as *const ()), // sub_141077720
    Replacement::draft(0x1410778f0, "r_1410778f0", fn_1410778f0 as *const ()), // sub_1410778f0
    Replacement::draft(0x141077980, "r_141077980", tpl_1840::<0x30, 0x143cbe8e4, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141077a00> as *const ()), // sub_141077980
    Replacement::draft_throws(0x1410a6260, "r_1410a6260", tpl_78787::<0x1410a5b40, 0x141f32be0> as *const (), Throws::Discard), // sub_1410a6260
    Replacement::draft(0x1410a6450, "r_1410a6450", tpl_64969::<0x143cd9be0, 0x142489ee0, 0x141f328f0> as *const ()), // sub_1410a6450
    Replacement::draft(0x1410a65a0, "r_1410a65a0", tpl_3028::<0x143cd9a7c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410a3af0> as *const ()), // sub_1410a65a0
    Replacement::draft(0x1410a6610, "r_1410a6610", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143384930, 0x8, 0x20> as *const ()), // sub_1410a6610
    Replacement::draft(0x1410a6c90, "r_1410a6c90", fn_1410a6c90 as *const ()), // sub_1410a6c90
    Replacement::draft(0x1410a7200, "r_1410a7200", fn_1410a7200 as *const ()), // sub_1410a7200
    Replacement::draft_throws(0x1410b1090, "r_1410b1090", tpl_1330::<0x8, 0x1410b3460, 0x143cda0f4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1410b1090
    Replacement::draft_throws(0x1410c5b70, "r_1410c5b70", tpl_78188::<0x88, 0x10088, 0x1410c7ba0, 0x141f42f30> as *const (), Throws::Discard), // sub_1410c5b70
    Replacement::draft_throws(0x1410c5c90, "r_1410c5c90", tpl_78054::<0x143cada18, 0x1410c7a40, 0x141f456d0> as *const (), Throws::Discard), // sub_1410c5c90
    Replacement::draft(0x1410c5de0, "r_1410c5de0", tpl_63093::<0x143cdb3fc, 0x142489ee0, 0x141f454a0> as *const ()), // sub_1410c5de0
    Replacement::draft_throws(0x1410c5ee0, "r_1410c5ee0", fn_1410c5ee0 as *const (), Throws::Discard), // sub_1410c5ee0
    Replacement::draft_throws(0x1410c6220, "r_1410c6220", fn_1410c6220 as *const (), Throws::Discard), // sub_1410c6220
    Replacement::draft_throws(0x1410c6420, "r_1410c6420", fn_1410c6420 as *const (), Throws::Discard), // sub_1410c6420
    Replacement::draft_throws(0x1410c66c0, "r_1410c66c0", fn_1410c66c0 as *const (), Throws::Discard), // sub_1410c66c0
    Replacement::draft(0x1410c68d0, "r_1410c68d0", fn_1410c68d0 as *const ()), // sub_1410c68d0
    Replacement::draft_throws(0x1410c6a10, "r_1410c6a10", fn_1410c6a10 as *const (), Throws::Discard), // sub_1410c6a10
    Replacement::draft(0x1410c6d40, "r_1410c6d40", fn_1410c6d40 as *const ()), // sub_1410c6d40
    Replacement::draft(0x1410c6d80, "r_1410c6d80", fn_1410c6d80 as *const ()), // sub_1410c6d80
    Replacement::draft(0x1410c6de0, "r_1410c6de0", fn_1410c6de0 as *const ()), // sub_1410c6de0
    Replacement::draft(0x1410c6e20, "r_1410c6e20", fn_1410c6e20 as *const ()), // sub_1410c6e20
    Replacement::draft(0x1410c6e60, "r_1410c6e60", fn_1410c6e60 as *const ()), // sub_1410c6e60
    Replacement::draft(0x1410c6ea0, "r_1410c6ea0", fn_1410c6ea0 as *const ()), // sub_1410c6ea0
    Replacement::draft(0x1410c6ec0, "r_1410c6ec0", fn_1410c6ec0 as *const ()), // sub_1410c6ec0
    Replacement::draft(0x1410c6f00, "r_1410c6f00", fn_1410c6f00 as *const ()), // sub_1410c6f00
    Replacement::draft(0x1410c6f20, "r_1410c6f20", fn_1410c6f20 as *const ()), // sub_1410c6f20
    Replacement::draft(0x1410c6f60, "r_1410c6f60", fn_1410c6f60 as *const ()), // sub_1410c6f60
    Replacement::draft(0x1410c6f80, "r_1410c6f80", fn_1410c6f80 as *const ()), // sub_1410c6f80
    Replacement::draft(0x1410c6fc0, "r_1410c6fc0", fn_1410c6fc0 as *const ()), // sub_1410c6fc0
    Replacement::draft(0x1410c6fe0, "r_1410c6fe0", fn_1410c6fe0 as *const ()), // sub_1410c6fe0
    Replacement::draft(0x1410c7020, "r_1410c7020", fn_1410c7020 as *const ()), // sub_1410c7020
    Replacement::draft(0x1410c7050, "r_1410c7050", fn_1410c7050 as *const ()), // sub_1410c7050
    Replacement::draft(0x1410c7090, "r_1410c7090", fn_1410c7090 as *const ()), // sub_1410c7090
    Replacement::draft(0x1410c70c0, "r_1410c70c0", fn_1410c70c0 as *const ()), // sub_1410c70c0
    Replacement::draft(0x1410c7100, "r_1410c7100", fn_1410c7100 as *const ()), // sub_1410c7100
    Replacement::draft(0x1410c7120, "r_1410c7120", fn_1410c7120 as *const ()), // sub_1410c7120
    Replacement::draft(0x1410c7160, "r_1410c7160", fn_1410c7160 as *const ()), // sub_1410c7160
    Replacement::draft(0x1410c7180, "r_1410c7180", fn_1410c7180 as *const ()), // sub_1410c7180
    Replacement::draft(0x1410c71c0, "r_1410c71c0", fn_1410c71c0 as *const ()), // sub_1410c71c0
    Replacement::draft(0x1410c71e0, "r_1410c71e0", fn_1410c71e0 as *const ()), // sub_1410c71e0
    Replacement::draft(0x1410c7220, "r_1410c7220", fn_1410c7220 as *const ()), // sub_1410c7220
    Replacement::draft(0x1410c7240, "r_1410c7240", fn_1410c7240 as *const ()), // sub_1410c7240
    Replacement::draft(0x1410c7280, "r_1410c7280", fn_1410c7280 as *const ()), // sub_1410c7280
    Replacement::draft(0x1410c72a0, "r_1410c72a0", fn_1410c72a0 as *const ()), // sub_1410c72a0
    Replacement::draft(0x1410c72e0, "r_1410c72e0", fn_1410c72e0 as *const ()), // sub_1410c72e0
    Replacement::draft(0x1410c7300, "r_1410c7300", fn_1410c7300 as *const ()), // sub_1410c7300
    Replacement::draft(0x1410c7340, "r_1410c7340", fn_1410c7340 as *const ()), // sub_1410c7340
    Replacement::draft(0x1410c7380, "r_1410c7380", fn_1410c7380 as *const ()), // sub_1410c7380
    Replacement::draft(0x1410c73c0, "r_1410c73c0", fn_1410c73c0 as *const ()), // sub_1410c73c0
    Replacement::draft(0x1410c73e0, "r_1410c73e0", fn_1410c73e0 as *const ()), // sub_1410c73e0
    Replacement::draft(0x1410c7420, "r_1410c7420", fn_1410c7420 as *const ()), // sub_1410c7420
    Replacement::draft(0x1410c7440, "r_1410c7440", fn_1410c7440 as *const ()), // sub_1410c7440
    Replacement::draft(0x1410c7480, "r_1410c7480", fn_1410c7480 as *const ()), // sub_1410c7480
    Replacement::draft(0x1410c74a0, "r_1410c74a0", fn_1410c74a0 as *const ()), // sub_1410c74a0
    Replacement::draft(0x1410c74e0, "r_1410c74e0", fn_1410c74e0 as *const ()), // sub_1410c74e0
    Replacement::draft(0x1410c7500, "r_1410c7500", fn_1410c7500 as *const ()), // sub_1410c7500
    Replacement::draft(0x1410c7540, "r_1410c7540", fn_1410c7540 as *const ()), // sub_1410c7540
    Replacement::draft(0x1410c7560, "r_1410c7560", fn_1410c7560 as *const ()), // sub_1410c7560
    Replacement::draft(0x1410c75a0, "r_1410c75a0", fn_1410c75a0 as *const ()), // sub_1410c75a0
    Replacement::draft(0x1410c75c0, "r_1410c75c0", fn_1410c75c0 as *const ()), // sub_1410c75c0
    Replacement::draft(0x1410c7600, "r_1410c7600", fn_1410c7600 as *const ()), // sub_1410c7600
    Replacement::draft(0x1410c7620, "r_1410c7620", fn_1410c7620 as *const ()), // sub_1410c7620
    Replacement::draft(0x1410c7660, "r_1410c7660", fn_1410c7660 as *const ()), // sub_1410c7660
    Replacement::draft(0x1410c7680, "r_1410c7680", fn_1410c7680 as *const ()), // sub_1410c7680
    Replacement::draft(0x1410c76c0, "r_1410c76c0", fn_1410c76c0 as *const ()), // sub_1410c76c0
    Replacement::draft(0x1410c76e0, "r_1410c76e0", fn_1410c76e0 as *const ()), // sub_1410c76e0
    Replacement::draft(0x1410c7720, "r_1410c7720", fn_1410c7720 as *const ()), // sub_1410c7720
    Replacement::draft(0x1410c7740, "r_1410c7740", fn_1410c7740 as *const ()), // sub_1410c7740
    Replacement::draft(0x1410c7780, "r_1410c7780", fn_1410c7780 as *const ()), // sub_1410c7780
    Replacement::draft(0x1410c77a0, "r_1410c77a0", fn_1410c77a0 as *const ()), // sub_1410c77a0
    Replacement::draft(0x1410c77e0, "r_1410c77e0", fn_1410c77e0 as *const ()), // sub_1410c77e0
    Replacement::draft(0x1410c7800, "r_1410c7800", fn_1410c7800 as *const ()), // sub_1410c7800
    Replacement::draft(0x1410c7840, "r_1410c7840", fn_1410c7840 as *const ()), // sub_1410c7840
    Replacement::draft(0x1410c7860, "r_1410c7860", fn_1410c7860 as *const ()), // sub_1410c7860
    Replacement::draft(0x1410c78a0, "r_1410c78a0", fn_1410c78a0 as *const ()), // sub_1410c78a0
    Replacement::draft(0x1410c78c0, "r_1410c78c0", fn_1410c78c0 as *const ()), // sub_1410c78c0
    Replacement::draft(0x1410c7900, "r_1410c7900", fn_1410c7900 as *const ()), // sub_1410c7900
    Replacement::draft(0x1410c7920, "r_1410c7920", fn_1410c7920 as *const ()), // sub_1410c7920
    Replacement::draft_throws(0x1410c7960, "r_1410c7960", tpl_1498::<0x143cdbb04, 0x143cdba64, 0x10, 0x1420a25b0, 0x141834a78, 0x143cdb628, 0x143cdba60, 0x143cdba68, 0x141834a0c> as *const (), Throws::Discard), // sub_1410c7960
    Replacement::draft(0x1410c7cd0, "r_1410c7cd0", fn_1410c7cd0 as *const ()), // sub_1410c7cd0
    Replacement::draft(0x1410d0960, "r_1410d0960", fn_1410d0960 as *const ()), // sub_1410d0960
    Replacement::draft(0x1410d0a40, "r_1410d0a40", tpl_2871::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143391040, 0x8, 0x9, 0x11, 0x1401f3d90, 0x143391058> as *const ()), // sub_1410d0a40
    Replacement::draft(0x1410d0b20, "r_1410d0b20", fn_1410d0b20 as *const ()), // sub_1410d0b20
    Replacement::draft(0x1410d0bf0, "r_1410d0bf0", tpl_20079::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143390ff8, 0x1401f3d90, 0x143390e10> as *const ()), // sub_1410d0bf0
    Replacement::draft(0x1410d0cb0, "r_1410d0cb0", tpl_20079::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143390e40, 0x1401f3d90, 0x1433910a0> as *const ()), // sub_1410d0cb0
    Replacement::draft(0x1410d0d70, "r_1410d0d70", tpl_20079::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143390df8, 0x1401f3d90, 0x143391028> as *const ()), // sub_1410d0d70
    Replacement::draft(0x1410d0e30, "r_1410d0e30", tpl_3310::<0x38, 0x10, 0x18, 0x143391088, 0x8, 0x1401f3d90, 0x20, 0x28, 0x143391010, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1410d0e30
    Replacement::draft(0x1410d0f00, "r_1410d0f00", fn_1410d0f00 as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise::<lambda_1>,void>::r::internal::ContinuationData<r::internal::
    Replacement::draft(0x14111a810, "r_14111a810", tpl_21104::<0x14241c980, 0x141834d30> as *const ()), // r::RingBufferMemoryResource::r::RingBufferMemoryResource::vfunc_0
    Replacement::draft(0x141131840, "r_141131840", fn_141131840 as *const ()), // sub_141131840
    Replacement::draft(0x14113c1d0, "r_14113c1d0", tpl_3028::<0x143ce0eac, 0x20, 0x8, 0x143ce0c20, 0x580e0, 0x141fb08a0> as *const ()), // sub_14113c1d0
    Replacement::draft(0x14113c240, "r_14113c240", tpl_62567::<0x14113b940, 0x141faea50> as *const ()), // sub_14113c240
    Replacement::draft(0x14113c3e0, "r_14113c3e0", fn_14113c3e0 as *const ()), // sub_14113c3e0
    Replacement::draft(0x14113c5d0, "r_14113c5d0", tpl_77683::<0x8, 0x88, 0x10088, 0x14113bd10, 0x141faf300> as *const ()), // sub_14113c5d0
    Replacement::draft(0x14113c6a0, "r_14113c6a0", fn_14113c6a0 as *const ()), // sub_14113c6a0
    Replacement::draft(0x14113c8a0, "r_14113c8a0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x14113ced0, 0x580f0, 0x141fb1a40> as *const ()), // sub_14113c8a0
    Replacement::draft(0x14113c990, "r_14113c990", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x141137e10> as *const ()), // sub_14113c990
    Replacement::draft(0x14113ca30, "r_14113ca30", tpl_4492::<0x142407c80, 0x14113cd70, 0x141fb1540> as *const ()), // sub_14113ca30
    Replacement::draft(0x14113cb60, "r_14113cb60", tpl_1127::<0x142407c80, 0x143ce0940, 0x142489ee0, 0x141fb1210> as *const ()), // sub_14113cb60
    Replacement::draft(0x14113d390, "r_14113d390", tpl_3213::<0x142407c80, 0x143ce0cec, 0x142489ee0, 0x143c8ec20, 0x141fb1a40> as *const ()), // sub_14113d390
    Replacement::draft(0x14113d540, "r_14113d540", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141137e10> as *const ()), // sub_14113d540
    Replacement::draft(0x14113d620, "r_14113d620", tpl_4496::<0x142407c80, 0x14113cd70, 0x141fb1540> as *const ()), // sub_14113d620
    Replacement::draft(0x14113d7d0, "r_14113d7d0", tpl_1131::<0x142407c80, 0x143ce0940, 0x142489ee0, 0x141fb1210> as *const ()), // sub_14113d7d0
    Replacement::draft(0x14113dbb0, "r_14113dbb0", fn_14113dbb0 as *const ()), // sub_14113dbb0
    Replacement::draft(0x14113dd60, "r_14113dd60", fn_14113dd60 as *const ()), // sub_14113dd60
    Replacement::draft(0x14113e040, "r_14113e040", fn_14113e040 as *const ()), // sub_14113e040
    Replacement::draft(0x14113e200, "r_14113e200", fn_14113e200 as *const ()), // sub_14113e200
    Replacement::draft(0x14118c2d0, "r_14118c2d0", fn_14118c2d0 as *const ()), // sub_14118c2d0
    Replacement::draft_throws(0x14118c690, "r_14118c690", crate::shards::subsystems::fn_14118c690 as *const (), Throws::Caught), // sub_14118c690
    Replacement::draft(0x14118c870, "r_14118c870", tpl_1131::<0x142407c80, 0x143c8e840, 0x142489ee0, 0x141fe9f60> as *const ()), // sub_14118c870
    Replacement::draft(0x14118cc40, "r_14118cc40", fn_14118cc40 as *const ()), // sub_14118cc40
    Replacement::draft(0x14118ce60, "r_14118ce60", fn_14118ce60 as *const ()), // sub_14118ce60
    Replacement::draft(0x14119db50, "r_14119db50", tpl_62765::<0x143ccc414, 0x143ce5fe8, 0x142489ee0, 0x143ce62f0, 0x143c8d120, 0x14200efe0> as *const ()), // sub_14119db50
    Replacement::draft(0x14119dda0, "r_14119dda0", tpl_62563::<0x143ce6020, 0x143c8d08c, 0x143ce5f98, 0x142489ee0, 0x143ce6064, 0x14200e910> as *const ()), // sub_14119dda0
    Replacement::draft(0x14119dfd0, "r_14119dfd0", tpl_1126::<0x142407c80, 0x580f0, 0x14200f230> as *const ()), // sub_14119dfd0
    Replacement::draft(0x14119e090, "r_14119e090", tpl_1127::<0x142407c80, 0x143ce5fe8, 0x142489ee0, 0x14200f0e0> as *const ()), // sub_14119e090
    Replacement::draft(0x14119e1a0, "r_14119e1a0", tpl_6169::<0x143c8d08c, 0x142407c80, 0x143cada18, 0x143ce6064, 0x142489ee0, 0x14200ed50> as *const ()), // sub_14119e1a0
    Replacement::draft(0x14119e2f0, "r_14119e2f0", tpl_1127::<0x142407c80, 0x143ce5f98, 0x142489ee0, 0x14200ebc0> as *const ()), // sub_14119e2f0
    Replacement::draft(0x1411f3cd0, "r_1411f3cd0", fn_1411f3cd0 as *const ()), // r::JobCallable<ecs::memory::paged_dynamic_array::perPageParallelForEach<const unsigned int,unsigned int*>::<lambda_1>>::
    Replacement::draft_throws(0x141299c90, "r_141299c90", tpl_23319::<0x1423f0c20, 0x141834d30> as *const (), Throws::Discard), // r::ThreadManager::Worker::r::ThreadManager::Worker::vfunc_0
    Replacement::draft(0x14129d5e0, "r_14129d5e0", fn_14129d5e0 as *const ()), // r::BufferedMemoryFile::r::BufferedMemoryFile::vfunc_0
    Replacement::draft(0x1412ab1b0, "r_1412ab1b0", fn_1412ab1b0 as *const ()), // sub_1412ab1b0
    Replacement::draft(0x1412ab2c0, "r_1412ab2c0", fn_1412ab2c0 as *const ()), // sub_1412ab2c0
    Replacement::draft(0x1412ab5d0, "r_1412ab5d0", tpl_746::<0x1412ac640, 0x1412ad320, 0x8, 0x20, 0x28, 0x142442de0> as *const ()), // sub_1412ab5d0
    Replacement::draft(0x1412ab680, "r_1412ab680", fn_1412ab680 as *const ()), // sub_1412ab680
    Replacement::draft_throws(0x1412ab950, "r_1412ab950", fn_1412ab950 as *const (), Throws::Caught), // sub_1412ab950
    Replacement::draft(0x1412abc40, "r_1412abc40", fn_1412abc40 as *const ()), // sub_1412abc40
    Replacement::draft(0x1412abf40, "r_1412abf40", fn_1412abf40 as *const ()), // sub_1412abf40
    Replacement::draft_throws(0x1412ac240, "r_1412ac240", fn_1412ac240 as *const (), Throws::Caught), // sub_1412ac240
    Replacement::draft(0x1412ac540, "r_1412ac540", fn_1412ac540 as *const ()), // sub_1412ac540
    Replacement::draft(0x1412ac5d0, "r_1412ac5d0", tpl_725::<0x28, 0x30, 0x8, 0x142442de0> as *const ()), // sub_1412ac5d0
    Replacement::draft(0x1412ac640, "r_1412ac640", tpl_23493::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x28, 0x48> as *const ()), // sub_1412ac640
    Replacement::draft(0x1412ac6d0, "r_1412ac6d0", tpl_3310::<0x38, 0x10, 0x18, 0x1433e9d28, 0x8, 0x1401f4c80, 0x20, 0x28, 0x1433e9d10, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412ac6d0
    Replacement::draft(0x1412ac7a0, "r_1412ac7a0", tpl_23494::<0x20, 0x10, 0x18, 0x1412ac640, 0x8, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412ac7a0
    Replacement::draft(0x1412ac860, "r_1412ac860", tpl_23495::<0x20, 0x10, 0x18, 0x141296150, 0x8, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412ac860
    Replacement::draft(0x1412ac900, "r_1412ac900", tpl_3310::<0x38, 0x10, 0x18, 0x1433e9e18, 0x8, 0x1412ac640, 0x20, 0x28, 0x1433e9d40, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412ac900
    Replacement::draft_throws(0x1412ac9d0, "r_1412ac9d0", fn_1412ac9d0 as *const (), Throws::Discard), // sub_1412ac9d0
    Replacement::draft(0x1412acb60, "r_1412acb60", tpl_23494::<0x20, 0x10, 0x18, 0x1412900a0, 0x8, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412acb60
    Replacement::draft(0x1412acc20, "r_1412acc20", tpl_3310::<0x38, 0x10, 0x18, 0x1433e9e48, 0x8, 0x1401f3d90, 0x20, 0x28, 0x1433e9e60, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412acc20
    Replacement::draft(0x1412accf0, "r_1412accf0", tpl_743::<0x30, 0x10, 0x18, 0x1433e9d70, 0x8, 0x1401f4c80, 0x20, 0x1433e9e00, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412accf0
    Replacement::draft(0x1412acdc0, "r_1412acdc0", tpl_743::<0x30, 0x10, 0x18, 0x1433e9db8, 0x8, 0x1412ac640, 0x20, 0x1433e9d58, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412acdc0
    Replacement::draft(0x1412ace90, "r_1412ace90", tpl_3310::<0x38, 0x10, 0x18, 0x1433e9d88, 0x8, 0x1401f4c80, 0x20, 0x28, 0x1433e9cb0, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412ace90
    Replacement::draft_throws(0x1412acf60, "r_1412acf60", fn_1412acf60 as *const (), Throws::Caught), // sub_1412acf60
    Replacement::draft(0x1412ad120, "r_1412ad120", fn_1412ad120 as *const ()), // sub_1412ad120
    Replacement::draft(0x1412ad250, "r_1412ad250", tpl_2733::<0x8, 0x9, 0x1412ad720, 0x142199330, 0x142442de0, 0x142198400> as *const ()), // sub_1412ad250
    Replacement::draft(0x1412ad320, "r_1412ad320", fn_1412ad320 as *const ()), // sub_1412ad320
    Replacement::draft(0x1412ad440, "r_1412ad440", fn_1412ad440 as *const ()), // sub_1412ad440
    Replacement::draft(0x1412ad720, "r_1412ad720", tpl_20086::<0x1412adb10, 0x1425371a0, 0x142533200, 0x142442de0, 0x1412ad320, 0x1401f3c00> as *const ()), // sub_1412ad720
    Replacement::draft(0x1412ada00, "r_1412ada00", tpl_23501::<0x11, 0x8, 0x1401f4e00, 0x20, 0x1401f3ee0> as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<platform::User,platform::Error>>::<lambda_1>,?
    Replacement::draft(0x1412ada30, "r_1412ada30", tpl_2871::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433e9ce0, 0x8, 0x9, 0x11, 0x1401f3d90, 0x1433e9dd0> as *const ()), // sub_1412ada30
    Replacement::draft(0x1412adb10, "r_1412adb10", tpl_2871::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433e9e30, 0x8, 0x9, 0x11, 0x1401f3d90, 0x1433e9de8> as *const ()), // sub_1412adb10
    Replacement::draft(0x1412adbf0, "r_1412adbf0", tpl_2879::<0x1401f3d00, 0x8, 0x1401f4c10, 0x14307d5c8, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise<r::Expected<platform::User,platform::Error>>::<lambda_1>,?bref
    Replacement::draft(0x1412adc40, "r_1412adc40", fn_1412adc40 as *const ()), // sub_1412adc40
    Replacement::draft_throws(0x1412add20, "r_1412add20", fn_1412add20 as *const (), Throws::Caught), // sub_1412add20
    Replacement::draft(0x1412adef0, "r_1412adef0", tpl_3581::<0x58, 0x14240ab10> as *const ()), // sub_1412adef0
    Replacement::draft(0x1412adf00, "r_1412adf00", fn_1412adf00 as *const ()), // sub_1412adf00
    Replacement::draft(0x1412adf60, "r_1412adf60", tpl_23505::<0x3, 0x4, 0x8, 0x14240aa00, 0x50, 0x54, 0x58, 0x68, 0x1412adfe0> as *const ()), // sub_1412adf60
    Replacement::draft(0x1412adfe0, "r_1412adfe0", fn_1412adfe0 as *const ()), // sub_1412adfe0
    Replacement::draft(0x1412ae060, "r_1412ae060", fn_1412ae060 as *const ()), // sub_1412ae060
    Replacement::draft_throws(0x1412ae090, "r_1412ae090", fn_1412ae090 as *const (), Throws::Discard), // sub_1412ae090
    Replacement::draft_throws(0x1412ae0f0, "r_1412ae0f0", fn_1412ae0f0 as *const (), Throws::Discard), // sub_1412ae0f0
    Replacement::draft(0x1412ae320, "r_1412ae320", fn_1412ae320 as *const ()), // sub_1412ae320
    Replacement::draft(0x1412ae3e0, "r_1412ae3e0", tpl_5412::<0x1402250d0, 0x14240aa00, 0x1412ae5d0, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1412ae3e0
    Replacement::draft(0x1412ae5d0, "r_1412ae5d0", tpl_5414::<0x8, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1412ae5d0
    Replacement::draft(0x1412ae660, "r_1412ae660", fn_1412ae660 as *const ()), // sub_1412ae660
    Replacement::draft(0x1412ae790, "r_1412ae790", fn_1412ae790 as *const ()), // sub_1412ae790
    Replacement::draft(0x1412ae800, "r_1412ae800", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x140941bd0> as *const ()), // sub_1412ae800
    Replacement::draft(0x1412ae8d0, "r_1412ae8d0", tpl_69810::<0x1412b1890, 0x14240ab10> as *const ()), // sub_1412ae8d0
    Replacement::draft(0x1412aea10, "r_1412aea10", fn_1412aea10 as *const ()), // sub_1412aea10
    Replacement::draft(0x1412aecf0, "r_1412aecf0", fn_1412aecf0 as *const ()), // sub_1412aecf0
    Replacement::draft(0x1412aedb0, "r_1412aedb0", fn_1412aedb0 as *const ()), // sub_1412aedb0
    Replacement::draft(0x1412aeea0, "r_1412aeea0", fn_1412aeea0 as *const ()), // sub_1412aeea0
    Replacement::draft_throws(0x1412af170, "r_1412af170", fn_1412af170 as *const (), Throws::Caught), // sub_1412af170
    Replacement::draft_throws(0x1412af460, "r_1412af460", crate::shards::subsystems::fn_1412af460 as *const (), Throws::Caught), // sub_1412af460
    Replacement::draft_throws(0x1412af750, "r_1412af750", crate::shards::subsystems::fn_1412af750 as *const (), Throws::Caught), // sub_1412af750
    Replacement::draft_throws(0x1412afa40, "r_1412afa40", fn_1412afa40 as *const (), Throws::Caught), // sub_1412afa40
    Replacement::draft(0x1412afd20, "r_1412afd20", fn_1412afd20 as *const ()), // sub_1412afd20
    Replacement::draft(0x1412b0040, "r_1412b0040", fn_1412b0040 as *const ()), // sub_1412b0040
    Replacement::draft(0x1412b0320, "r_1412b0320", fn_1412b0320 as *const ()), // sub_1412b0320
    Replacement::draft_throws(0x1412b0650, "r_1412b0650", fn_1412b0650 as *const (), Throws::Caught), // sub_1412b0650
    Replacement::draft(0x1412b0930, "r_1412b0930", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x140941bd0> as *const ()), // sub_1412b0930
    Replacement::draft(0x1412b09b0, "r_1412b09b0", tpl_23493::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x28, 0x48> as *const ()), // sub_1412b09b0
    Replacement::draft(0x1412b0a40, "r_1412b0a40", fn_1412b0a40 as *const ()), // sub_1412b0a40
    Replacement::draft(0x1412b0b40, "r_1412b0b40", fn_1412b0b40 as *const ()), // sub_1412b0b40
    Replacement::draft(0x1412b0c30, "r_1412b0c30", tpl_23526::<0x1412b1e30, 0x1412b0c30, 0x1412b1950, 0x1412b1c10, 0x1412b3260> as *const ()), // sub_1412b0c30
    Replacement::draft(0x1412b0de0, "r_1412b0de0", tpl_23526::<0x1412b2bf0, 0x1412b0de0, 0x1412b27d0, 0x1412b29a0, 0x1412b3460> as *const ()), // sub_1412b0de0
    Replacement::draft(0x1412b0f90, "r_1412b0f90", tpl_23494::<0x20, 0x10, 0x18, 0x1412b09b0, 0x8, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b0f90
    Replacement::draft(0x1412b1050, "r_1412b1050", tpl_743::<0x30, 0x10, 0x18, 0x1433ea040, 0x8, 0x1401f3d90, 0x20, 0x1433e9f38, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b1050
    Replacement::draft(0x1412b1120, "r_1412b1120", tpl_23495::<0x20, 0x10, 0x18, 0x1402bf830, 0x8, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b1120
    Replacement::draft(0x1412b11c0, "r_1412b11c0", tpl_3310::<0x38, 0x10, 0x18, 0x1433e9f20, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433e9ff8, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b11c0
    Replacement::draft(0x1412b1290, "r_1412b1290", tpl_23527::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433ea088, 0x8, 0x20, 0x1c, 0x24, 0x1402bf830, 0x1433e9fe0> as *const ()), // sub_1412b1290
    Replacement::draft(0x1412b1380, "r_1412b1380", tpl_23527::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433e9ec0, 0x8, 0x20, 0x1c, 0x24, 0x1402bf830, 0x1433ea0d0> as *const ()), // sub_1412b1380
    Replacement::draft(0x1412b1470, "r_1412b1470", tpl_743::<0x30, 0x10, 0x18, 0x1433e9f08, 0x8, 0x1402bf830, 0x20, 0x1433e9ef0, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b1470
    Replacement::draft(0x1412b1540, "r_1412b1540", tpl_743::<0x30, 0x10, 0x18, 0x1433ea028, 0x8, 0x1402bf830, 0x20, 0x1433ea0b8, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b1540
    Replacement::draft(0x1412b1610, "r_1412b1610", tpl_20079::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433e9f68, 0x1402bf830, 0x1433e9ed8> as *const ()), // sub_1412b1610
    Replacement::draft(0x1412b16d0, "r_1412b16d0", fn_1412b16d0 as *const ()), // sub_1412b16d0
    Replacement::draft(0x1412b17c0, "r_1412b17c0", tpl_3310::<0x38, 0x10, 0x18, 0x1433ea0a0, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433ea010, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412b17c0
    Replacement::draft(0x1412b1890, "r_1412b1890", fn_1412b1890 as *const ()), // sub_1412b1890
    Replacement::draft(0x1412b1950, "r_1412b1950", fn_1412b1950 as *const ()), // sub_1412b1950
    Replacement::draft(0x1412b1c10, "r_1412b1c10", fn_1412b1c10 as *const ()), // sub_1412b1c10
    Replacement::draft(0x1412b1e30, "r_1412b1e30", fn_1412b1e30 as *const ()), // sub_1412b1e30
    Replacement::draft(0x1412b27d0, "r_1412b27d0", fn_1412b27d0 as *const ()), // sub_1412b27d0
    Replacement::draft(0x1412b29a0, "r_1412b29a0", fn_1412b29a0 as *const ()), // sub_1412b29a0
    Replacement::draft(0x1412b2bf0, "r_1412b2bf0", fn_1412b2bf0 as *const ()), // sub_1412b2bf0
    Replacement::draft(0x1412b31a0, "r_1412b31a0", tpl_2733::<0x8, 0x9, 0x1412b3760, 0x1401fa0a0, 0x142442de0, 0x1402bf7c0> as *const ()), // sub_1412b31a0
    Replacement::draft(0x1412b3260, "r_1412b3260", fn_1412b3260 as *const ()), // sub_1412b3260
    Replacement::draft(0x1412b3460, "r_1412b3460", fn_1412b3460 as *const ()), // sub_1412b3460
    Replacement::draft(0x1412b3640, "r_1412b3640", tpl_23340::<0x1412b3a70, 0x1426455a8, 0x142645598, 0x1426455c0, 0x142442de0, 0x140941bd0> as *const ()), // sub_1412b3640
    Replacement::draft(0x1412b3760, "r_1412b3760", fn_1412b3760 as *const ()), // sub_1412b3760
    Replacement::draft(0x1412b3a40, "r_1412b3a40", tpl_23501::<0x11, 0x8, 0x14128fd40, 0x20, 0x1401f3ee0> as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<void,platform::Error>>::<lambda_1>,?bref3::Exp
    Replacement::draft(0x1412b3a70, "r_1412b3a70", fn_1412b3a70 as *const ()), // sub_1412b3a70
    Replacement::draft(0x1412b3c50, "r_1412b3c50", fn_1412b3c50 as *const ()), // sub_1412b3c50
    Replacement::draft(0x1412b3de0, "r_1412b3de0", tpl_23534::<0x1421a54f0> as *const ()), // sub_1412b3de0
    Replacement::draft(0x1412b3e80, "r_1412b3e80", tpl_23534::<0x1421a5440> as *const ()), // sub_1412b3e80
    Replacement::draft(0x1412b3f20, "r_1412b3f20", tpl_2871::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433ea058, 0x8, 0x9, 0x11, 0x1401f3d90, 0x1433e9fb0> as *const ()), // sub_1412b3f20
    Replacement::draft(0x1412b4000, "r_1412b4000", tpl_2879::<0x1401f3d00, 0x8, 0x1402bf7c0, 0x1430b8a68, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise<r::Expected<void,platform::Error>>::<lambda_1>,?bref3::Expecte
    Replacement::draft(0x1412cf1c0, "r_1412cf1c0", tpl_23660::<0xb1> as *const ()), // r::RendGlobalParameter<float>::r::RendGlobalParameter<float>::vfunc_6
    Replacement::draft(0x1412cf1e0, "r_1412cf1e0", fn_1412cf1e0 as *const ()), // r::RendGlobalParameter<float>::r::RendGlobalParameter<float>::vfunc_5
    Replacement::draft(0x1412cf210, "r_1412cf210", fn_1412cf210 as *const ()), // r::RendGlobalParameter<float>::r::RendGlobalParameter<float>::vfunc_0
    Replacement::draft(0x1412d0950, "r_1412d0950", fn_1412d0950 as *const ()), // r::StreamingFile::r::StreamingFile::vfunc_0
    Replacement::draft(0x1412d0a20, "r_1412d0a20", tpl_23688::<0x142407c80, 0x143c4aa00, 0x1421f3a60, 0x1426455a8, 0x142645598, 0x1421f2a80, 0x142645580> as *const ()), // sub_1412d0a20
    Replacement::draft(0x1412d0b70, "r_1412d0b70", fn_1412d0b70 as *const ()), // sub_1412d0b70
    Replacement::draft(0x1412d0d00, "r_1412d0d00", fn_1412d0d00 as *const ()), // sub_1412d0d00
    Replacement::draft(0x1412d0e80, "r_1412d0e80", fn_1412d0e80 as *const ()), // sub_1412d0e80
    Replacement::draft(0x1412d1100, "r_1412d1100", fn_1412d1100 as *const ()), // sub_1412d1100
    Replacement::draft(0x1412d1140, "r_1412d1140", fn_1412d1140 as *const ()), // sub_1412d1140
    Replacement::draft_throws(0x1412d11a0, "r_1412d11a0", fn_1412d11a0 as *const (), Throws::Caught), // sub_1412d11a0
    Replacement::draft(0x1412d1330, "r_1412d1330", fn_1412d1330 as *const ()), // sub_1412d1330
    Replacement::draft_throws(0x1412d13a0, "r_1412d13a0", fn_1412d13a0 as *const (), Throws::Discard), // sub_1412d13a0
    Replacement::draft_throws(0x1412d1450, "r_1412d1450", fn_1412d1450 as *const (), Throws::Discard), // sub_1412d1450
    Replacement::draft(0x1412d14e0, "r_1412d14e0", fn_1412d14e0 as *const ()), // sub_1412d14e0
    Replacement::draft(0x1412d1530, "r_1412d1530", fn_1412d1530 as *const ()), // sub_1412d1530
    Replacement::draft(0x1412d15c0, "r_1412d15c0", fn_1412d15c0 as *const ()), // sub_1412d15c0
    Replacement::draft(0x1412d1620, "r_1412d1620", fn_1412d1620 as *const ()), // sub_1412d1620
    Replacement::draft(0x1412d16c0, "r_1412d16c0", fn_1412d16c0 as *const ()), // sub_1412d16c0
    Replacement::draft(0x1412d1760, "r_1412d1760", fn_1412d1760 as *const ()), // sub_1412d1760
    Replacement::draft(0x1412d17f0, "r_1412d17f0", fn_1412d17f0 as *const ()), // sub_1412d17f0
    Replacement::draft(0x1412d1930, "r_1412d1930", fn_1412d1930 as *const ()), // sub_1412d1930
    Replacement::draft_throws(0x1412d1a80, "r_1412d1a80", fn_1412d1a80 as *const (), Throws::Discard), // sub_1412d1a80
    Replacement::draft(0x1412d24c0, "r_1412d24c0", fn_1412d24c0 as *const ()), // sub_1412d24c0
    Replacement::draft_throws(0x1412d2ad0, "r_1412d2ad0", fn_1412d2ad0 as *const (), Throws::Discard), // sub_1412d2ad0
    Replacement::draft(0x1412d3800, "r_1412d3800", fn_1412d3800 as *const ()), // sub_1412d3800
    Replacement::draft_throws(0x1412d3be0, "r_1412d3be0", fn_1412d3be0 as *const (), Throws::Caught), // sub_1412d3be0
    Replacement::draft_throws(0x1412d4bf0, "r_1412d4bf0", fn_1412d4bf0 as *const (), Throws::Caught), // sub_1412d4bf0
    Replacement::draft_throws(0x1412d59e0, "r_1412d59e0", fn_1412d59e0 as *const (), Throws::Caught), // sub_1412d59e0
    Replacement::draft(0x1412d67c0, "r_1412d67c0", fn_1412d67c0 as *const ()), // sub_1412d67c0
    Replacement::draft(0x1412d6b10, "r_1412d6b10", fn_1412d6b10 as *const ()), // sub_1412d6b10
    Replacement::draft(0x1412d71a0, "r_1412d71a0", fn_1412d71a0 as *const ()), // sub_1412d71a0
    Replacement::draft_throws(0x1412d7840, "r_1412d7840", fn_1412d7840 as *const (), Throws::Discard), // sub_1412d7840
    Replacement::draft_throws(0x1412d7f70, "r_1412d7f70", fn_1412d7f70 as *const (), Throws::Caught), // sub_1412d7f70
    Replacement::draft(0x1412d8960, "r_1412d8960", tpl_23711::<0x14307bcd0, 0x143878a10, 0x14307bcc0, 0x1401de410, 0x1438795d0, 0x14307bce0, 0x14307bdc8, 0x143878db0, 0x14307be80, 0x14307bda8, 0x143878f98, 0x14307be98, 0x14307beb8, 0x143878bc8, 0x14307bed8, 0x14307be08, 0x143878c60, 0x14307be28, 0x14307be48, 0x143878d38, 0x14307be68, 0x14307bf40, 0x143878f70, 0x14307bf58, 0x14307bf70, 0x143879088, 0x14307bf88, 0x14307bef0, 0x143879320, 0x14307bf20, 0x14307bf10, 0x143879578, 0x14307bf30, 0x14307c018, 0x1438793d8, 0x14307c028, 0x14307c038, 0x143879540, 0x14307c048, 0x14307bfa8, 0x143879590, 0x14307bfc0, 0x14307bfe0, 0x1438793a0, 0x14307c000, 0x14307c0c0, 0x1438792d0, 0x14307c0d8, 0x14307c0e8, 0x143879108, 0x14307c0f8, 0x14307c058, 0x14307c068, 0x143878a60, 0x14307c088, 0x143878fb8, 0x14307c0a0, 0x14307c190, 0x14307c1a8, 0x14307c1d0, 0x143878d08, 0x14307c1f0, 0x14307c108, 0x143879308, 0x14307c120, 0x14307c148, 0x1438791f0, 0x14307c170, 0x14307c258, 0x143878cd8, 0x14307c270, 0x14307c288, 0x143878b88, 0x14307c298, 0x14307c210, 0x14307c238, 0x14307c248, 0x143878d98, 0x14307c310, 0x14307c320, 0x143878b00, 0x14307c2f0, 0x14307c300, 0x143878a80, 0x14307c3c8, 0x14307c3e8, 0x1438790a8, 0x14307c360, 0x14307c370, 0x143878d20, 0x14307c390, 0x14307c448, 0x143878bf8, 0x14307c488, 0x14307c400, 0x143878a48, 0x14307c410, 0x14307c420, 0x143878c80, 0x14307c498, 0x14307c430> as *const ()), // sub_1412d8960
    Replacement::draft(0x1412d9110, "r_1412d9110", fn_1412d9110 as *const ()), // sub_1412d9110
    Replacement::draft(0x1412d9980, "r_1412d9980", tpl_23711::<0x14307bcd0, 0x143879450, 0x14307bcc0, 0x1401de410, 0x1438793f8, 0x14307bce0, 0x14307bdc8, 0x143879128, 0x14307be80, 0x14307bda8, 0x1438792e8, 0x14307be98, 0x14307beb8, 0x143879208, 0x14307bed8, 0x14307be08, 0x1438795b8, 0x14307be28, 0x14307be48, 0x143879180, 0x14307be68, 0x14307bf40, 0x143878ab0, 0x14307bf58, 0x14307bf70, 0x143879160, 0x14307bf88, 0x14307bef0, 0x143879380, 0x14307bf20, 0x14307bf10, 0x143879560, 0x14307bf30, 0x14307c018, 0x143878c20, 0x14307c028, 0x14307c038, 0x143878ef8, 0x14307c048, 0x14307bfa8, 0x143879470, 0x14307bfc0, 0x14307bfe0, 0x143878cc0, 0x14307c000, 0x14307c0c0, 0x143879250, 0x14307c0d8, 0x14307c0e8, 0x1438793b8, 0x14307c0f8, 0x14307c058, 0x14307c068, 0x143878e48, 0x14307c088, 0x143878ad8, 0x14307c0a0, 0x14307c190, 0x14307c1a8, 0x14307c1d0, 0x1438791d8, 0x14307c1f0, 0x14307c108, 0x143879498, 0x14307c120, 0x14307c148, 0x1438794e0, 0x14307c170, 0x14307c258, 0x143879008, 0x14307c270, 0x14307c288, 0x143879198, 0x14307c298, 0x14307c210, 0x14307c238, 0x14307c248, 0x143878b48, 0x14307c310, 0x14307c320, 0x143879510, 0x14307c2f0, 0x14307c300, 0x143879238, 0x14307c3c8, 0x14307c3e8, 0x143878f18, 0x14307c360, 0x14307c370, 0x143879418, 0x14307c390, 0x14307c448, 0x143878fe0, 0x14307c488, 0x14307c400, 0x143879220, 0x14307c410, 0x14307c420, 0x143879270, 0x14307c498, 0x14307c430> as *const ()), // sub_1412d9980
    Replacement::draft(0x1412da130, "r_1412da130", tpl_13639::<0x143881940, 0x14243b0b0, 0x1412dabf0, 0x10, 0x14086adb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412da130
    Replacement::draft_throws(0x1412da1b0, "r_1412da1b0", fn_1412da1b0 as *const (), Throws::Discard), // sub_1412da1b0
    Replacement::draft(0x1412da2e0, "r_1412da2e0", fn_1412da2e0 as *const ()), // sub_1412da2e0
    Replacement::draft_throws(0x1412da3e0, "r_1412da3e0", fn_1412da3e0 as *const (), Throws::Discard), // sub_1412da3e0
    Replacement::draft(0x1412da450, "r_1412da450", fn_1412da450 as *const ()), // sub_1412da450
    Replacement::draft_throws(0x1412da4e0, "r_1412da4e0", crate::shards::subsystems::fn_1412da4e0 as *const (), Throws::Caught), // sub_1412da4e0
    Replacement::draft_throws(0x1412da5a0, "r_1412da5a0", crate::shards::subsystems::fn_1412da5a0 as *const (), Throws::Caught), // sub_1412da5a0
    Replacement::draft_throws(0x1412da660, "r_1412da660", crate::shards::subsystems::fn_1412da660 as *const (), Throws::Caught), // sub_1412da660
    Replacement::draft(0x1412da720, "r_1412da720", fn_1412da720 as *const ()), // sub_1412da720
    Replacement::draft(0x1412da850, "r_1412da850", fn_1412da850 as *const ()), // sub_1412da850
    Replacement::draft(0x1412da930, "r_1412da930", fn_1412da930 as *const ()), // sub_1412da930
    Replacement::draft_throws(0x1412daa20, "r_1412daa20", fn_1412daa20 as *const (), Throws::Discard), // sub_1412daa20
    Replacement::draft_throws(0x1412dab30, "r_1412dab30", tpl_10122::<0x143881940, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1433ef5a0, 0x1412dada0, 0x10, 0x1433ef590, 0x14086af60> as *const (), Throws::Discard), // sub_1412dab30
    Replacement::draft(0x1412dabf0, "r_1412dabf0", fn_1412dabf0 as *const ()), // sub_1412dabf0
    Replacement::draft(0x1412dacc0, "r_1412dacc0", fn_1412dacc0 as *const ()), // sub_1412dacc0
    Replacement::draft_throws(0x1412dada0, "r_1412dada0", tpl_9694::<0x20, 0x8, 0x1412db620> as *const (), Throws::Discard), // sub_1412dada0
    Replacement::draft_throws(0x1412dadf0, "r_1412dadf0", fn_1412dadf0 as *const (), Throws::Caught), // sub_1412dadf0
    Replacement::draft_throws(0x1412db0d0, "r_1412db0d0", fn_1412db0d0 as *const (), Throws::Caught), // sub_1412db0d0
    Replacement::draft_throws(0x1412db380, "r_1412db380", fn_1412db380 as *const (), Throws::Caught), // sub_1412db380
    Replacement::draft_throws(0x1412db620, "r_1412db620", crate::shards::subsystems::fn_1412db620 as *const (), Throws::Caught), // sub_1412db620
    Replacement::draft_throws(0x1412db6e0, "r_1412db6e0", crate::shards::subsystems::fn_1412db6e0 as *const (), Throws::Caught), // sub_1412db6e0
    Replacement::draft_throws(0x1412dbc50, "r_1412dbc50", crate::shards::subsystems::fn_1412dbc50 as *const (), Throws::Caught), // sub_1412dbc50
    Replacement::draft_throws(0x1412dc130, "r_1412dc130", fn_1412dc130 as *const (), Throws::Caught), // sub_1412dc130
    Replacement::draft(0x1412dc600, "r_1412dc600", tpl_23726::<0x18, 0x1c, 0x4, 0x20, 0x8> as *const ()), // sub_1412dc600
    Replacement::draft(0x1412dc660, "r_1412dc660", tpl_23726::<0x24, 0x28, 0x4, 0x2c, 0x8> as *const ()), // sub_1412dc660
    Replacement::draft(0x1412dcd80, "r_1412dcd80", tpl_23660::<0xd9> as *const ()), // r::RendGlobalParameter<m::Vector3Template<float>>::r::RendGlobalParameter<m::Vector3Template<float>>::vfunc_6
    Replacement::draft(0x1412dcda0, "r_1412dcda0", fn_1412dcda0 as *const ()), // r::RendGlobalParameter<m::Vector3Template<float>>::r::RendGlobalParameter<m::Vector3Template<float>>::vfunc_5
    Replacement::draft_throws(0x1412dcde0, "r_1412dcde0", fn_1412dcde0 as *const (), Throws::Discard), // sub_1412dcde0
    Replacement::draft(0x1412dce40, "r_1412dce40", fn_1412dce40 as *const ()), // r::RendGlobalParameter<m::Vector3Template<float>>::r::RendGlobalParameter<m::Vector3Template<float>>::vfunc_0
    Replacement::draft(0x1412f44f0, "r_1412f44f0", tpl_6280::<0x142428ce0, 0x141834d30> as *const ()), // r::Resource::r::Resource::vfunc_0
    Replacement::draft(0x14136d480, "r_14136d480", tpl_23660::<0xc1> as *const ()), // r::RendGlobalParameter<m::Vector2Template<float>>::r::RendGlobalParameter<m::Vector2Template<float>>::vfunc_6
    Replacement::draft(0x14136d4a0, "r_14136d4a0", fn_14136d4a0 as *const ()), // r::RendGlobalParameter<m::Vector2Template<float>>::r::RendGlobalParameter<m::Vector2Template<float>>::vfunc_5
    Replacement::draft(0x14136d4d0, "r_14136d4d0", tpl_789::<0xa0, 0x98> as *const ()), // r::RendGlobalParameter<m::Vector2Template<float>>::r::RendGlobalParameter<m::Vector2Template<float>>::vfunc_4
    Replacement::draft(0x14136d4e0, "r_14136d4e0", tpl_788::<0xa0, 0xb8, 0xa4, 0xbc> as *const ()), // r::RendGlobalParameter<m::Vector2Template<float>>::r::RendGlobalParameter<m::Vector2Template<float>>::vfunc_3
    Replacement::draft(0x14136d510, "r_14136d510", tpl_789::<0xb8, 0xa0> as *const ()), // r::RendGlobalParameter<m::Vector2Template<float>>::r::RendGlobalParameter<m::Vector2Template<float>>::vfunc_2
    Replacement::draft(0x14136d520, "r_14136d520", fn_14136d520 as *const ()), // r::RendGlobalParameter<m::Vector2Template<float>>::r::RendGlobalParameter<m::Vector2Template<float>>::vfunc_1
    Replacement::draft_throws(0x14136ee80, "r_14136ee80", tpl_24936::<0x1423f0c20, 0x141834d30> as *const (), Throws::Discard), // r::Thread::r::Thread::vfunc_0
    Replacement::draft(0x14136f290, "r_14136f290", fn_14136f290 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_0
    Replacement::draft_throws(0x14136f330, "r_14136f330", fn_14136f330 as *const (), Throws::Caught), // sub_14136f330
    Replacement::draft_throws(0x14136f4d0, "r_14136f4d0", fn_14136f4d0 as *const (), Throws::Discard), // sub_14136f4d0
    Replacement::draft_throws(0x14136f6a0, "r_14136f6a0", fn_14136f6a0 as *const (), Throws::Caught), // sub_14136f6a0
    Replacement::draft_throws(0x14136f890, "r_14136f890", fn_14136f890 as *const (), Throws::Caught), // sub_14136f890
    Replacement::draft_throws(0x14136fab0, "r_14136fab0", fn_14136fab0 as *const (), Throws::Caught), // sub_14136fab0
    Replacement::draft_throws(0x14136fcc0, "r_14136fcc0", fn_14136fcc0 as *const (), Throws::Caught), // sub_14136fcc0
    Replacement::draft_throws(0x14136fec0, "r_14136fec0", fn_14136fec0 as *const (), Throws::Caught), // sub_14136fec0
    Replacement::draft_throws(0x1413700b0, "r_1413700b0", fn_1413700b0 as *const (), Throws::Caught), // sub_1413700b0
    Replacement::draft_throws(0x1413702c0, "r_1413702c0", fn_1413702c0 as *const (), Throws::Caught), // sub_1413702c0
    Replacement::draft_throws(0x1413704c0, "r_1413704c0", fn_1413704c0 as *const (), Throws::Discard), // sub_1413704c0
    Replacement::draft_throws(0x1413706c0, "r_1413706c0", fn_1413706c0 as *const (), Throws::Discard), // sub_1413706c0
    Replacement::draft_throws(0x1413708c0, "r_1413708c0", fn_1413708c0 as *const (), Throws::Discard), // sub_1413708c0
    Replacement::draft_throws(0x141370ab0, "r_141370ab0", fn_141370ab0 as *const (), Throws::Discard), // sub_141370ab0
    Replacement::draft_throws(0x141370ca0, "r_141370ca0", fn_141370ca0 as *const (), Throws::Discard), // sub_141370ca0
    Replacement::draft_throws(0x141370e70, "r_141370e70", fn_141370e70 as *const (), Throws::Discard), // sub_141370e70
    Replacement::draft_throws(0x141371040, "r_141371040", fn_141371040 as *const (), Throws::Discard), // sub_141371040
    Replacement::draft(0x141371260, "r_141371260", fn_141371260 as *const ()), // sub_141371260
    Replacement::draft_throws(0x1413712b0, "r_1413712b0", fn_1413712b0 as *const (), Throws::Discard), // sub_1413712b0
    Replacement::draft_throws(0x141371490, "r_141371490", fn_141371490 as *const (), Throws::Discard), // r::WinFileSystem::r::WinFileSystem::vfunc_0
    Replacement::draft(0x141371510, "r_141371510", fn_141371510 as *const ()), // sub_141371510
    Replacement::draft(0x141371610, "r_141371610", fn_141371610 as *const ()), // sub_141371610
    Replacement::draft(0x1413716e0, "r_1413716e0", fn_1413716e0 as *const ()), // sub_1413716e0
    Replacement::draft(0x1413719d0, "r_1413719d0", fn_1413719d0 as *const ()), // sub_1413719d0
    Replacement::draft(0x141371c70, "r_141371c70", tpl_24966::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143407038, 0x8, 0xc, 0x14, 0x1401f3d90, 0x143407050> as *const ()), // sub_141371c70
    Replacement::draft_throws(0x1413725b0, "r_1413725b0", fn_1413725b0 as *const (), Throws::Discard), // r::detail::win::AsyncIOManager::r::detail::win::AsyncIOManager::vfunc_0
    Replacement::draft(0x1413725f0, "r_1413725f0", fn_1413725f0 as *const ()), // sub_1413725f0
    Replacement::draft(0x1413726c0, "r_1413726c0", fn_1413726c0 as *const ()), // sub_1413726c0
    Replacement::draft(0x1413726e0, "r_1413726e0", fn_1413726e0 as *const ()), // sub_1413726e0
    Replacement::draft(0x141372bf0, "r_141372bf0", fn_141372bf0 as *const ()), // sub_141372bf0
    Replacement::draft_throws(0x141372fe0, "r_141372fe0", fn_141372fe0 as *const (), Throws::Discard), // sub_141372fe0
    Replacement::draft(0x141373100, "r_141373100", fn_141373100 as *const ()), // sub_141373100
    Replacement::draft(0x141373270, "r_141373270", fn_141373270 as *const ()), // sub_141373270
    Replacement::draft(0x141373300, "r_141373300", fn_141373300 as *const ()), // sub_141373300
    Replacement::draft(0x141373370, "r_141373370", fn_141373370 as *const ()), // sub_141373370
    Replacement::draft(0x141373500, "r_141373500", fn_141373500 as *const ()), // sub_141373500
    Replacement::draft(0x141373610, "r_141373610", fn_141373610 as *const ()), // sub_141373610
    Replacement::draft(0x141374440, "r_141374440", fn_141374440 as *const ()), // sub_141374440
    Replacement::draft(0x1413747b0, "r_1413747b0", fn_1413747b0 as *const ()), // sub_1413747b0
    Replacement::draft(0x141374950, "r_141374950", fn_141374950 as *const ()), // sub_141374950
    Replacement::draft(0x141374ae0, "r_141374ae0", fn_141374ae0 as *const ()), // sub_141374ae0
    Replacement::draft(0x141375090, "r_141375090", fn_141375090 as *const ()), // sub_141375090
    Replacement::draft(0x141375470, "r_141375470", fn_141375470 as *const ()), // sub_141375470
    Replacement::draft(0x141375600, "r_141375600", fn_141375600 as *const ()), // sub_141375600
    Replacement::draft(0x141375670, "r_141375670", fn_141375670 as *const ()), // sub_141375670
    Replacement::draft(0x141375760, "r_141375760", fn_141375760 as *const ()), // sub_141375760
    Replacement::draft(0x141376580, "r_141376580", fn_141376580 as *const ()), // sub_141376580
    Replacement::draft(0x1413768e0, "r_1413768e0", fn_1413768e0 as *const ()), // sub_1413768e0
    Replacement::draft(0x141376a70, "r_141376a70", fn_141376a70 as *const ()), // sub_141376a70
    Replacement::draft(0x141376bf0, "r_141376bf0", fn_141376bf0 as *const ()), // sub_141376bf0
    Replacement::draft(0x141376cb0, "r_141376cb0", fn_141376cb0 as *const ()), // sub_141376cb0
    Replacement::draft(0x141376d50, "r_141376d50", fn_141376d50 as *const ()), // r::reflection::detail::TypeInfoFactoryImpl::r::reflection::detail::TypeInfoFactoryImpl::vfunc_0
    Replacement::draft(0x1413794c0, "r_1413794c0", tpl_25002::<0x14240c500, 0x141834d30> as *const ()), // r::StreamingFile::ChunkResource::r::StreamingFile::ChunkResource::vfunc_0
    Replacement::draft(0x141379500, "r_141379500", fn_141379500 as *const ()), // sub_141379500
    Replacement::draft(0x141379560, "r_141379560", fn_141379560 as *const ()), // sub_141379560
    Replacement::draft(0x1413795d0, "r_1413795d0", tpl_2030::<0x141379560, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413795d0
    Replacement::draft(0x141379680, "r_141379680", fn_141379680 as *const ()), // sub_141379680
    Replacement::draft(0x1413796f0, "r_1413796f0", fn_1413796f0 as *const ()), // sub_1413796f0
    Replacement::draft(0x141379870, "r_141379870", fn_141379870 as *const ()), // sub_141379870
    Replacement::draft(0x141379b50, "r_141379b50", fn_141379b50 as *const ()), // sub_141379b50
    Replacement::draft(0x141379c00, "r_141379c00", tpl_23827::<0x14340ed48, 0x8, 0x143407948, 0x142498f00> as *const ()), // sub_141379c00
    Replacement::draft_throws(0x141379c40, "r_141379c40", fn_141379c40 as *const (), Throws::Discard), // r::Streamer::r::Streamer::vfunc_0
    Replacement::draft(0x141379da0, "r_141379da0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143407c40, 0x8> as *const ()), // sub_141379da0
    Replacement::draft(0x141379e20, "r_141379e20", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143407c40> as *const ()), // sub_141379e20
    Replacement::draft(0x141379ec0, "r_141379ec0", fn_141379ec0 as *const ()), // sub_141379ec0
    Replacement::draft(0x141379f80, "r_141379f80", fn_141379f80 as *const ()), // sub_141379f80
    Replacement::draft(0x14137a040, "r_14137a040", fn_14137a040 as *const ()), // sub_14137a040
    Replacement::draft(0x14137a140, "r_14137a140", fn_14137a140 as *const ()), // sub_14137a140
    Replacement::draft_throws(0x14137a2f0, "r_14137a2f0", fn_14137a2f0 as *const (), Throws::Discard), // sub_14137a2f0
    Replacement::draft(0x14137a3b0, "r_14137a3b0", fn_14137a3b0 as *const ()), // sub_14137a3b0
    Replacement::draft_throws(0x14137a500, "r_14137a500", fn_14137a500 as *const (), Throws::Discard), // sub_14137a500
    Replacement::draft(0x14137a5c0, "r_14137a5c0", fn_14137a5c0 as *const ()), // sub_14137a5c0
    Replacement::draft_throws(0x14137a6b0, "r_14137a6b0", crate::shards::subsystems::fn_14137a6b0 as *const (), Throws::Caught), // sub_14137a6b0
    Replacement::draft_throws(0x14137a780, "r_14137a780", crate::shards::subsystems::fn_14137a780 as *const (), Throws::Caught), // sub_14137a780
    Replacement::draft_throws(0x14137a850, "r_14137a850", crate::shards::subsystems::fn_14137a850 as *const (), Throws::Caught), // sub_14137a850
    Replacement::draft(0x14137a920, "r_14137a920", fn_14137a920 as *const ()), // sub_14137a920
    Replacement::draft(0x14137aa40, "r_14137aa40", fn_14137aa40 as *const ()), // sub_14137aa40
    Replacement::draft(0x14137ab70, "r_14137ab70", fn_14137ab70 as *const ()), // sub_14137ab70
    Replacement::draft_throws(0x14137acc0, "r_14137acc0", fn_14137acc0 as *const (), Throws::Discard), // sub_14137acc0
    Replacement::draft_throws(0x14137ae00, "r_14137ae00", fn_14137ae00 as *const (), Throws::Discard), // sub_14137ae00
    Replacement::draft(0x14137aed0, "r_14137aed0", tpl_9485::<0x143881ef0, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137aed0
    Replacement::draft_throws(0x14137af70, "r_14137af70", tpl_9486::<0x143881ef8, 0x143881ef0, 0x1401e9fa0, 0x328, 0x338, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14137af70
    Replacement::draft(0x14137aff0, "r_14137aff0", tpl_9416::<0x143881ef0, 0x14243b0b0, 0x338, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137aff0
    Replacement::draft_throws(0x14137b060, "r_14137b060", tpl_9487::<0x143881ef8, 0x143881ef0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14137b060
    Replacement::draft_throws(0x14137b0f0, "r_14137b0f0", crate::shards::subsystems::fn_14137b0f0 as *const (), Throws::Caught), // sub_14137b0f0
    Replacement::draft_throws(0x14137b1c0, "r_14137b1c0", crate::shards::subsystems::fn_14137b1c0 as *const (), Throws::Caught), // sub_14137b1c0
    Replacement::draft_throws(0x14137b290, "r_14137b290", crate::shards::subsystems::fn_14137b290 as *const (), Throws::Caught), // sub_14137b290
    Replacement::draft(0x14137b360, "r_14137b360", tpl_9491::<0x143881ef0, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137b360
    Replacement::draft(0x14137b3d0, "r_14137b3d0", tpl_9416::<0x143881ef0, 0x14243b0b0, 0x338, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137b3d0
    Replacement::draft_throws(0x14137b440, "r_14137b440", tpl_9494::<0x143881ef8, 0x143881ef0, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14137b440
    Replacement::draft_throws(0x14137b4b0, "r_14137b4b0", fn_14137b4b0 as *const (), Throws::Discard), // sub_14137b4b0
    Replacement::draft(0x14137b560, "r_14137b560", tpl_636::<0x143881f10, 0x14243b0b0, 0x143881ef0, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14137b560
    Replacement::draft_throws(0x14137b6d0, "r_14137b6d0", crate::shards::subsystems::fn_14137b6d0 as *const (), Throws::Caught), // sub_14137b6d0
    Replacement::draft_throws(0x14137b770, "r_14137b770", crate::shards::subsystems::fn_14137b770 as *const (), Throws::Caught), // sub_14137b770
    Replacement::draft_throws(0x14137b810, "r_14137b810", crate::shards::subsystems::fn_14137b810 as *const (), Throws::Caught), // sub_14137b810
    Replacement::draft(0x14137b8b0, "r_14137b8b0", fn_14137b8b0 as *const ()), // sub_14137b8b0
    Replacement::draft(0x14137b920, "r_14137b920", fn_14137b920 as *const ()), // sub_14137b920
    Replacement::draft_throws(0x14137ba20, "r_14137ba20", fn_14137ba20 as *const (), Throws::Discard), // sub_14137ba20
    Replacement::draft_throws(0x14137bac0, "r_14137bac0", fn_14137bac0 as *const (), Throws::Discard), // sub_14137bac0
    Replacement::draft(0x14137bb60, "r_14137bb60", fn_14137bb60 as *const ()), // sub_14137bb60
    Replacement::draft(0x14137bbf0, "r_14137bbf0", fn_14137bbf0 as *const ()), // sub_14137bbf0
    Replacement::draft(0x14137bca0, "r_14137bca0", fn_14137bca0 as *const ()), // sub_14137bca0
    Replacement::draft(0x14137be00, "r_14137be00", fn_14137be00 as *const ()), // sub_14137be00
    Replacement::draft_throws(0x14137bfe0, "r_14137bfe0", fn_14137bfe0 as *const (), Throws::Discard), // sub_14137bfe0
    Replacement::draft(0x14137c170, "r_14137c170", fn_14137c170 as *const ()), // sub_14137c170
    Replacement::draft_throws(0x14137c2e0, "r_14137c2e0", fn_14137c2e0 as *const (), Throws::Discard), // sub_14137c2e0
    Replacement::draft(0x14137c4d0, "r_14137c4d0", fn_14137c4d0 as *const ()), // sub_14137c4d0
    Replacement::draft_throws(0x14137c600, "r_14137c600", crate::shards::subsystems::fn_14137c600 as *const (), Throws::Caught), // sub_14137c600
    Replacement::draft_throws(0x14137c720, "r_14137c720", crate::shards::subsystems::fn_14137c720 as *const (), Throws::Caught), // sub_14137c720
    Replacement::draft_throws(0x14137c840, "r_14137c840", crate::shards::subsystems::fn_14137c840 as *const (), Throws::Caught), // sub_14137c840
    Replacement::draft(0x14137c960, "r_14137c960", fn_14137c960 as *const ()), // sub_14137c960
    Replacement::draft(0x14137cad0, "r_14137cad0", fn_14137cad0 as *const ()), // sub_14137cad0
    Replacement::draft(0x14137cc90, "r_14137cc90", fn_14137cc90 as *const ()), // sub_14137cc90
    Replacement::draft_throws(0x14137ce00, "r_14137ce00", fn_14137ce00 as *const (), Throws::Discard), // sub_14137ce00
    Replacement::draft_throws(0x14137cfd0, "r_14137cfd0", fn_14137cfd0 as *const (), Throws::Discard), // sub_14137cfd0
    Replacement::draft(0x14137d0d0, "r_14137d0d0", tpl_25038::<0x143881f30, 0x14243b0b0, 0x143881ef0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x4> as *const ()), // sub_14137d0d0
    Replacement::draft(0x14137d1a0, "r_14137d1a0", tpl_25039::<0x143881f30, 0x14243b0b0, 0x14137aed0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137d1a0
    Replacement::draft_throws(0x14137d210, "r_14137d210", tpl_25040::<0x143881f38, 0x143881f30, 0x1401e9fa0, 0x14137e5b0, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_14137d210
    Replacement::draft(0x14137d270, "r_14137d270", tpl_25039::<0x143881f30, 0x14243b0b0, 0x14137aff0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137d270
    Replacement::draft_throws(0x14137d2e0, "r_14137d2e0", tpl_25040::<0x143881f38, 0x143881f30, 0x1401e9b60, 0x14137e9c0, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_14137d2e0
    Replacement::draft_throws(0x14137d340, "r_14137d340", crate::shards::subsystems::fn_14137d340 as *const (), Throws::Caught), // sub_14137d340
    Replacement::draft_throws(0x14137d400, "r_14137d400", crate::shards::subsystems::fn_14137d400 as *const (), Throws::Caught), // sub_14137d400
    Replacement::draft_throws(0x14137d4c0, "r_14137d4c0", crate::shards::subsystems::fn_14137d4c0 as *const (), Throws::Caught), // sub_14137d4c0
    Replacement::draft(0x14137d580, "r_14137d580", tpl_25042::<0x143881f30, 0x14243b0b0, 0x338, 0x14137b360, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137d580
    Replacement::draft(0x14137d610, "r_14137d610", tpl_25043::<0x143881f38, 0x143881f30, 0x1401e9970, 0x143881ef8, 0x143881ef0, 0x338, 0xe0, 0x1401e9860, 0x4> as *const ()), // sub_14137d610
    Replacement::draft(0x14137d6e0, "r_14137d6e0", tpl_25039::<0x143881f30, 0x14243b0b0, 0x14137b3d0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137d6e0
    Replacement::draft_throws(0x14137d750, "r_14137d750", tpl_25044::<0x143881f38, 0x143881f30, 0x1401e9770, 0x143881ef8, 0x143881ef0, 0x338, 0x120, 0x1401e9590, 0x4> as *const (), Throws::Discard), // sub_14137d750
    Replacement::draft_throws(0x14137d830, "r_14137d830", tpl_25045::<0x143881f30, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1430af560, 0x14137f400, 0x4, 0x143407e48> as *const (), Throws::Discard), // sub_14137d830
    Replacement::draft(0x14137d8e0, "r_14137d8e0", tpl_25038::<0x143881f40, 0x14243b0b0, 0x143881ef0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x4> as *const ()), // sub_14137d8e0
    Replacement::draft(0x14137d9b0, "r_14137d9b0", tpl_25039::<0x143881f40, 0x14243b0b0, 0x14137aed0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137d9b0
    Replacement::draft_throws(0x14137da20, "r_14137da20", tpl_25040::<0x143881f48, 0x143881f40, 0x1401e9fa0, 0x14137e5b0, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_14137da20
    Replacement::draft(0x14137da80, "r_14137da80", tpl_25039::<0x143881f40, 0x14243b0b0, 0x14137aff0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137da80
    Replacement::draft_throws(0x14137daf0, "r_14137daf0", tpl_25040::<0x143881f48, 0x143881f40, 0x1401e9b60, 0x14137e9c0, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_14137daf0
    Replacement::draft_throws(0x14137db50, "r_14137db50", crate::shards::subsystems::fn_14137db50 as *const (), Throws::Caught), // sub_14137db50
    Replacement::draft_throws(0x14137dc10, "r_14137dc10", crate::shards::subsystems::fn_14137dc10 as *const (), Throws::Caught), // sub_14137dc10
    Replacement::draft_throws(0x14137dcd0, "r_14137dcd0", crate::shards::subsystems::fn_14137dcd0 as *const (), Throws::Caught), // sub_14137dcd0
    Replacement::draft(0x14137dd90, "r_14137dd90", tpl_25042::<0x143881f40, 0x14243b0b0, 0x338, 0x14137b360, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137dd90
    Replacement::draft(0x14137de20, "r_14137de20", tpl_25043::<0x143881f48, 0x143881f40, 0x1401e9970, 0x143881ef8, 0x143881ef0, 0x338, 0xe0, 0x1401e9860, 0x4> as *const ()), // sub_14137de20
    Replacement::draft(0x14137def0, "r_14137def0", tpl_25039::<0x143881f40, 0x14243b0b0, 0x14137b3d0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14137def0
    Replacement::draft_throws(0x14137df60, "r_14137df60", tpl_25044::<0x143881f48, 0x143881f40, 0x1401e9770, 0x143881ef8, 0x143881ef0, 0x338, 0x120, 0x1401e9590, 0x4> as *const (), Throws::Discard), // sub_14137df60
    Replacement::draft_throws(0x14137e040, "r_14137e040", tpl_25045::<0x143881f40, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143407e58, 0x14137f400, 0x4, 0x143407e38> as *const (), Throws::Discard), // sub_14137e040
    Replacement::draft(0x14137e0f0, "r_14137e0f0", tpl_25046::<0x141386e50, 0x14183acc8> as *const ()), // sub_14137e0f0
    Replacement::draft(0x14137e200, "r_14137e200", fn_14137e200 as *const ()), // sub_14137e200
    Replacement::draft(0x14137e320, "r_14137e320", fn_14137e320 as *const ()), // sub_14137e320
    Replacement::draft_throws(0x14137e400, "r_14137e400", fn_14137e400 as *const (), Throws::Caught), // sub_14137e400
    Replacement::draft_throws(0x14137e5b0, "r_14137e5b0", crate::shards::subsystems::fn_14137e5b0 as *const (), Throws::Caught), // sub_14137e5b0
    Replacement::draft_throws(0x14137e800, "r_14137e800", fn_14137e800 as *const (), Throws::Caught), // sub_14137e800
    Replacement::draft_throws(0x14137e9c0, "r_14137e9c0", crate::shards::subsystems::fn_14137e9c0 as *const (), Throws::Caught), // sub_14137e9c0
    Replacement::draft_throws(0x14137ec30, "r_14137ec30", crate::shards::subsystems::fn_14137ec30 as *const (), Throws::Caught), // sub_14137ec30
    Replacement::draft_throws(0x14137eea0, "r_14137eea0", crate::shards::subsystems::fn_14137eea0 as *const (), Throws::Caught), // sub_14137eea0
    Replacement::draft_throws(0x14137f110, "r_14137f110", crate::shards::subsystems::fn_14137f110 as *const (), Throws::Caught), // sub_14137f110
    Replacement::draft_throws(0x14137f360, "r_14137f360", tpl_2502::<0x20, 0x8, 0x141382fd0> as *const (), Throws::Discard), // sub_14137f360
    Replacement::draft_throws(0x14137f3b0, "r_14137f3b0", tpl_12645::<0x20, 0x8, 0x1413830a0> as *const (), Throws::Discard), // sub_14137f3b0
    Replacement::draft_throws(0x14137f400, "r_14137f400", fn_14137f400 as *const (), Throws::Caught), // sub_14137f400
    Replacement::draft_throws(0x14137f580, "r_14137f580", tpl_2502::<0x20, 0x8, 0x141386be0> as *const (), Throws::Discard), // sub_14137f580
    Replacement::draft(0x14137f670, "r_14137f670", tpl_62153::<0x8, 0xc, 0x141386e50, 0x14183acc2> as *const ()), // sub_14137f670
    Replacement::draft(0x14137f720, "r_14137f720", tpl_66024::<0x8, 0xc, 0x14137f7e0> as *const ()), // sub_14137f720
    Replacement::draft(0x14137f7e0, "r_14137f7e0", fn_14137f7e0 as *const ()), // sub_14137f7e0
    Replacement::draft(0x14137f870, "r_14137f870", tpl_62153::<0x8, 0xc, 0x14137f920, 0x14183acc2> as *const ()), // sub_14137f870
    Replacement::draft(0x14137f920, "r_14137f920", fn_14137f920 as *const ()), // sub_14137f920
    Replacement::draft(0x14137fa70, "r_14137fa70", fn_14137fa70 as *const ()), // sub_14137fa70
    Replacement::draft(0x14137fba0, "r_14137fba0", fn_14137fba0 as *const ()), // sub_14137fba0
    Replacement::draft(0x14137fd80, "r_14137fd80", fn_14137fd80 as *const ()), // sub_14137fd80
    Replacement::draft_throws(0x14137fdd0, "r_14137fdd0", crate::shards::subsystems::fn_14137fdd0 as *const (), Throws::Caught), // sub_14137fdd0
    Replacement::draft_throws(0x141380200, "r_141380200", crate::shards::subsystems::fn_141380200 as *const (), Throws::Caught), // sub_141380200
    Replacement::draft_throws(0x141380630, "r_141380630", fn_141380630 as *const (), Throws::Caught), // sub_141380630
    Replacement::draft_throws(0x141380bb0, "r_141380bb0", fn_141380bb0 as *const (), Throws::Caught), // sub_141380bb0
    Replacement::draft_throws(0x141380ea0, "r_141380ea0", fn_141380ea0 as *const (), Throws::Caught), // sub_141380ea0
    Replacement::draft_throws(0x141381530, "r_141381530", fn_141381530 as *const (), Throws::Caught), // sub_141381530
    Replacement::draft_throws(0x141381a90, "r_141381a90", fn_141381a90 as *const (), Throws::Caught), // sub_141381a90
    Replacement::draft_throws(0x141381d50, "r_141381d50", fn_141381d50 as *const (), Throws::Caught), // sub_141381d50
    Replacement::draft_throws(0x1413823a0, "r_1413823a0", fn_1413823a0 as *const (), Throws::Caught), // sub_1413823a0
    Replacement::draft_throws(0x1413828f0, "r_1413828f0", crate::shards::subsystems::fn_1413828f0 as *const (), Throws::Caught), // sub_1413828f0
    Replacement::draft_throws(0x141382b90, "r_141382b90", crate::shards::subsystems::fn_141382b90 as *const (), Throws::Caught), // sub_141382b90
    Replacement::draft_throws(0x141382fd0, "r_141382fd0", crate::shards::subsystems::fn_141382fd0 as *const (), Throws::Caught), // sub_141382fd0
    Replacement::draft_throws(0x1413830a0, "r_1413830a0", crate::shards::subsystems::fn_1413830a0 as *const (), Throws::Caught), // sub_1413830a0
    Replacement::draft_throws(0x141383170, "r_141383170", crate::shards::subsystems::fn_141383170 as *const (), Throws::Caught), // sub_141383170
    Replacement::draft_throws(0x141383430, "r_141383430", crate::shards::subsystems::fn_141383430 as *const (), Throws::Caught), // sub_141383430
    Replacement::draft_throws(0x1413836f0, "r_1413836f0", fn_1413836f0 as *const (), Throws::Caught), // sub_1413836f0
    Replacement::draft_throws(0x141383b10, "r_141383b10", crate::shards::subsystems::fn_141383b10 as *const (), Throws::Caught), // sub_141383b10
    Replacement::draft_throws(0x141384200, "r_141384200", crate::shards::subsystems::fn_141384200 as *const (), Throws::Caught), // sub_141384200
    Replacement::draft_throws(0x1413848f0, "r_1413848f0", fn_1413848f0 as *const (), Throws::Caught), // sub_1413848f0
    Replacement::draft_throws(0x141384cf0, "r_141384cf0", crate::shards::subsystems::fn_141384cf0 as *const (), Throws::Caught), // sub_141384cf0
    Replacement::draft_throws(0x1413853b0, "r_1413853b0", crate::shards::subsystems::fn_1413853b0 as *const (), Throws::Caught), // sub_1413853b0
    Replacement::draft_throws(0x141385a70, "r_141385a70", fn_141385a70 as *const (), Throws::Caught), // sub_141385a70
    Replacement::draft_throws(0x141385e60, "r_141385e60", crate::shards::subsystems::fn_141385e60 as *const (), Throws::Caught), // sub_141385e60
    Replacement::draft_throws(0x141386520, "r_141386520", crate::shards::subsystems::fn_141386520 as *const (), Throws::Caught), // sub_141386520
    Replacement::draft_throws(0x141386be0, "r_141386be0", crate::shards::subsystems::fn_141386be0 as *const (), Throws::Caught), // sub_141386be0
    Replacement::draft_throws(0x141386cb0, "r_141386cb0", crate::shards::subsystems::fn_141386cb0 as *const (), Throws::Caught), // sub_141386cb0
    Replacement::draft_throws(0x141386d80, "r_141386d80", crate::shards::subsystems::fn_141386d80 as *const (), Throws::Caught), // sub_141386d80
    Replacement::draft(0x141386e50, "r_141386e50", fn_141386e50 as *const ()), // sub_141386e50
    Replacement::draft_throws(0x141386ed0, "r_141386ed0", crate::shards::subsystems::fn_141386ed0 as *const (), Throws::Caught), // sub_141386ed0
    Replacement::draft_throws(0x1413873a0, "r_1413873a0", crate::shards::subsystems::fn_1413873a0 as *const (), Throws::Caught), // sub_1413873a0
    Replacement::draft_throws(0x141387870, "r_141387870", crate::shards::subsystems::fn_141387870 as *const (), Throws::Caught), // sub_141387870
    Replacement::draft_throws(0x141387cf0, "r_141387cf0", crate::shards::subsystems::fn_141387cf0 as *const (), Throws::Caught), // sub_141387cf0
    Replacement::draft_throws(0x141388180, "r_141388180", crate::shards::subsystems::fn_141388180 as *const (), Throws::Caught), // sub_141388180
    Replacement::draft_throws(0x141388600, "r_141388600", crate::shards::subsystems::fn_141388600 as *const (), Throws::Caught), // sub_141388600
    Replacement::draft_throws(0x141388a90, "r_141388a90", crate::shards::subsystems::fn_141388a90 as *const (), Throws::Caught), // sub_141388a90
    Replacement::draft_throws(0x141388f50, "r_141388f50", crate::shards::subsystems::fn_141388f50 as *const (), Throws::Caught), // sub_141388f50
    Replacement::draft_throws(0x141389410, "r_141389410", crate::shards::subsystems::fn_141389410 as *const (), Throws::Caught), // sub_141389410
    Replacement::draft_throws(0x1413898d0, "r_1413898d0", crate::shards::subsystems::fn_1413898d0 as *const (), Throws::Caught), // sub_1413898d0
    Replacement::draft_throws(0x141389d90, "r_141389d90", crate::shards::subsystems::fn_141389d90 as *const (), Throws::Caught), // sub_141389d90
    Replacement::draft_throws(0x14138a260, "r_14138a260", crate::shards::subsystems::fn_14138a260 as *const (), Throws::Caught), // sub_14138a260
    Replacement::draft_throws(0x14138a730, "r_14138a730", crate::shards::subsystems::fn_14138a730 as *const (), Throws::Caught), // sub_14138a730
    Replacement::draft_throws(0x14138ac00, "r_14138ac00", crate::shards::subsystems::fn_14138ac00 as *const (), Throws::Caught), // sub_14138ac00
    Replacement::draft_throws(0x14138b080, "r_14138b080", crate::shards::subsystems::fn_14138b080 as *const (), Throws::Caught), // sub_14138b080
    Replacement::draft_throws(0x14138b500, "r_14138b500", crate::shards::subsystems::fn_14138b500 as *const (), Throws::Caught), // sub_14138b500
    Replacement::draft_throws(0x14138b980, "r_14138b980", crate::shards::subsystems::fn_14138b980 as *const (), Throws::Caught), // sub_14138b980
    Replacement::draft_throws(0x14138be00, "r_14138be00", crate::shards::subsystems::fn_14138be00 as *const (), Throws::Caught), // sub_14138be00
    Replacement::draft_throws(0x14138c280, "r_14138c280", crate::shards::subsystems::fn_14138c280 as *const (), Throws::Caught), // sub_14138c280
    Replacement::draft(0x14138c700, "r_14138c700", tpl_25046::<0x14137f920, 0x14183acc8> as *const ()), // sub_14138c700
    Replacement::draft_throws(0x14138c810, "r_14138c810", fn_14138c810 as *const (), Throws::Caught), // sub_14138c810
    Replacement::draft_throws(0x14138cb80, "r_14138cb80", crate::shards::subsystems::fn_14138cb80 as *const (), Throws::Caught), // sub_14138cb80
    Replacement::draft_throws(0x14138ccc0, "r_14138ccc0", crate::shards::subsystems::fn_14138ccc0 as *const (), Throws::Caught), // sub_14138ccc0
    Replacement::draft_throws(0x14138d030, "r_14138d030", crate::shards::subsystems::fn_14138d030 as *const (), Throws::Caught), // sub_14138d030
    Replacement::draft_throws(0x14138d170, "r_14138d170", crate::shards::subsystems::fn_14138d170 as *const (), Throws::Caught), // sub_14138d170
    Replacement::draft_throws(0x14138d4e0, "r_14138d4e0", crate::shards::subsystems::fn_14138d4e0 as *const (), Throws::Caught), // sub_14138d4e0
    Replacement::draft_throws(0x14138d620, "r_14138d620", fn_14138d620 as *const (), Throws::Caught), // sub_14138d620
    Replacement::draft_throws(0x14138d850, "r_14138d850", crate::shards::subsystems::fn_14138d850 as *const (), Throws::Caught), // sub_14138d850
    Replacement::draft_throws(0x14138da40, "r_14138da40", fn_14138da40 as *const (), Throws::Caught), // sub_14138da40
    Replacement::draft_throws(0x14138dc90, "r_14138dc90", crate::shards::subsystems::fn_14138dc90 as *const (), Throws::Caught), // sub_14138dc90
    Replacement::draft_throws(0x14138e120, "r_14138e120", crate::shards::subsystems::fn_14138e120 as *const (), Throws::Caught), // sub_14138e120
    Replacement::draft_throws(0x14138e5b0, "r_14138e5b0", crate::shards::subsystems::fn_14138e5b0 as *const (), Throws::Caught), // sub_14138e5b0
    Replacement::draft_throws(0x14138e800, "r_14138e800", crate::shards::subsystems::fn_14138e800 as *const (), Throws::Caught), // sub_14138e800
    Replacement::draft_throws(0x14138ec90, "r_14138ec90", crate::shards::subsystems::fn_14138ec90 as *const (), Throws::Caught), // sub_14138ec90
    Replacement::draft_throws(0x14138f120, "r_14138f120", crate::shards::subsystems::fn_14138f120 as *const (), Throws::Caught), // sub_14138f120
    Replacement::draft_throws(0x14138f370, "r_14138f370", crate::shards::subsystems::fn_14138f370 as *const (), Throws::Caught), // sub_14138f370
    Replacement::draft_throws(0x14138f800, "r_14138f800", crate::shards::subsystems::fn_14138f800 as *const (), Throws::Caught), // sub_14138f800
    Replacement::draft_throws(0x14138fc90, "r_14138fc90", fn_14138fc90 as *const (), Throws::Caught), // sub_14138fc90
    Replacement::draft_throws(0x14138fee0, "r_14138fee0", crate::shards::subsystems::fn_14138fee0 as *const (), Throws::Caught), // sub_14138fee0
    Replacement::draft_throws(0x141390120, "r_141390120", crate::shards::subsystems::fn_141390120 as *const (), Throws::Caught), // sub_141390120
    Replacement::draft_throws(0x141390360, "r_141390360", fn_141390360 as *const (), Throws::Caught), // sub_141390360
    Replacement::draft_throws(0x141390a70, "r_141390a70", crate::shards::subsystems::fn_141390a70 as *const (), Throws::Caught), // sub_141390a70
    Replacement::draft_throws(0x141390ee0, "r_141390ee0", fn_141390ee0 as *const (), Throws::Caught), // sub_141390ee0
    Replacement::draft_throws(0x1413915c0, "r_1413915c0", crate::shards::subsystems::fn_1413915c0 as *const (), Throws::Caught), // sub_1413915c0
    Replacement::draft_throws(0x1413919f0, "r_1413919f0", fn_1413919f0 as *const (), Throws::Caught), // sub_1413919f0
    Replacement::draft_throws(0x141391e00, "r_141391e00", crate::shards::subsystems::fn_141391e00 as *const (), Throws::Caught), // sub_141391e00
    Replacement::draft_throws(0x141392150, "r_141392150", fn_141392150 as *const (), Throws::Caught), // sub_141392150
    Replacement::draft_throws(0x1413926b0, "r_1413926b0", crate::shards::subsystems::fn_1413926b0 as *const (), Throws::Caught), // sub_1413926b0
    Replacement::draft_throws(0x141392d60, "r_141392d60", crate::shards::subsystems::fn_141392d60 as *const (), Throws::Caught), // sub_141392d60
    Replacement::draft_throws(0x141393410, "r_141393410", fn_141393410 as *const (), Throws::Caught), // sub_141393410
    Replacement::draft_throws(0x141393940, "r_141393940", crate::shards::subsystems::fn_141393940 as *const (), Throws::Caught), // sub_141393940
    Replacement::draft_throws(0x141393fc0, "r_141393fc0", crate::shards::subsystems::fn_141393fc0 as *const (), Throws::Caught), // sub_141393fc0
    Replacement::draft_throws(0x141394640, "r_141394640", crate::shards::subsystems::fn_141394640 as *const (), Throws::Caught), // sub_141394640
    Replacement::draft_throws(0x141394990, "r_141394990", crate::shards::subsystems::fn_141394990 as *const (), Throws::Caught), // sub_141394990
    Replacement::draft_throws(0x141394ce0, "r_141394ce0", crate::shards::subsystems::fn_141394ce0 as *const (), Throws::Caught), // sub_141394ce0
    Replacement::draft(0x141395030, "r_141395030", fn_141395030 as *const ()), // sub_141395030
    Replacement::draft(0x141395280, "r_141395280", fn_141395280 as *const ()), // sub_141395280
    Replacement::draft(0x1413953d0, "r_1413953d0", fn_1413953d0 as *const ()), // sub_1413953d0
    Replacement::draft(0x141395460, "r_141395460", fn_141395460 as *const ()), // sub_141395460
    Replacement::draft(0x141395570, "r_141395570", fn_141395570 as *const ()), // sub_141395570
    Replacement::draft(0x141395680, "r_141395680", fn_141395680 as *const ()), // sub_141395680
    Replacement::draft(0x1413957b0, "r_1413957b0", fn_1413957b0 as *const ()), // sub_1413957b0
    Replacement::draft_throws(0x141395900, "r_141395900", fn_141395900 as *const (), Throws::Discard), // sub_141395900
    Replacement::draft(0x141395a30, "r_141395a30", fn_141395a30 as *const ()), // sub_141395a30
    Replacement::draft_throws(0x141395b30, "r_141395b30", fn_141395b30 as *const (), Throws::Discard), // sub_141395b30
    Replacement::draft_throws(0x141395c80, "r_141395c80", fn_141395c80 as *const (), Throws::Discard), // sub_141395c80
    Replacement::draft_throws(0x141395d80, "r_141395d80", crate::shards::subsystems::fn_141395d80 as *const (), Throws::Caught), // sub_141395d80
    Replacement::draft_throws(0x141395ea0, "r_141395ea0", crate::shards::subsystems::fn_141395ea0 as *const (), Throws::Caught), // sub_141395ea0
    Replacement::draft_throws(0x141395fc0, "r_141395fc0", crate::shards::subsystems::fn_141395fc0 as *const (), Throws::Caught), // sub_141395fc0
    Replacement::draft(0x1413960e0, "r_1413960e0", fn_1413960e0 as *const ()), // sub_1413960e0
    Replacement::draft(0x1413961f0, "r_1413961f0", fn_1413961f0 as *const ()), // sub_1413961f0
    Replacement::draft(0x141396350, "r_141396350", fn_141396350 as *const ()), // sub_141396350
    Replacement::draft_throws(0x141396450, "r_141396450", fn_141396450 as *const (), Throws::Discard), // sub_141396450
    Replacement::draft_throws(0x1413965d0, "r_1413965d0", fn_1413965d0 as *const (), Throws::Discard), // sub_1413965d0
    Replacement::draft(0x1413966e0, "r_1413966e0", fn_1413966e0 as *const ()), // sub_1413966e0
    Replacement::draft(0x141396800, "r_141396800", fn_141396800 as *const ()), // sub_141396800
    Replacement::draft(0x1413968b0, "r_1413968b0", fn_1413968b0 as *const ()), // sub_1413968b0
    Replacement::draft_throws(0x141396930, "r_141396930", fn_141396930 as *const (), Throws::Caught), // sub_141396930
    Replacement::draft(0x141396ab0, "r_141396ab0", tpl_25106::<0x143881f60, 0x14243b0b0, 0x14137aff0, 0x8, 0x90, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141396ab0
    Replacement::draft_throws(0x141396b30, "r_141396b30", fn_141396b30 as *const (), Throws::Caught), // sub_141396b30
    Replacement::draft_throws(0x141396cb0, "r_141396cb0", fn_141396cb0 as *const (), Throws::Discard), // sub_141396cb0
    Replacement::draft_throws(0x141396d30, "r_141396d30", tpl_997::<0x141396d40, 0x141396d39> as *const (), Throws::Discard), // sub_141396d30
    Replacement::draft_throws(0x141396d40, "r_141396d40", tpl_25109::<0x142415cb0, 0x142415cc0, 0x1413953c0, 0x1401e9210, 0x143408b90, 0x14137ec30, 0x1413971d0, 0x141396d9c> as *const (), Throws::Discard), // sub_141396d40
    Replacement::draft_throws(0x141396da0, "r_141396da0", tpl_997::<0x141396db0, 0x141396da9> as *const (), Throws::Discard), // sub_141396da0
    Replacement::draft_throws(0x141396db0, "r_141396db0", tpl_25109::<0x142415cb0, 0x142415cc0, 0x1413953c0, 0x1401e9210, 0x143408b90, 0x14137eea0, 0x1413971d0, 0x141396e0c> as *const (), Throws::Discard), // sub_141396db0
    Replacement::draft_throws(0x141396e10, "r_141396e10", tpl_997::<0x141396e20, 0x141396e19> as *const (), Throws::Discard), // sub_141396e10
    Replacement::draft_throws(0x141396e20, "r_141396e20", tpl_25109::<0x142415cb0, 0x142415cc0, 0x1413953c0, 0x1401e9210, 0x143408b90, 0x14137f110, 0x1413971d0, 0x141396e7c> as *const (), Throws::Discard), // sub_141396e20
    Replacement::draft(0x141396e80, "r_141396e80", fn_141396e80 as *const ()), // sub_141396e80
    Replacement::draft(0x141396f10, "r_141396f10", fn_141396f10 as *const ()), // sub_141396f10
    Replacement::draft(0x141396fb0, "r_141396fb0", tpl_25106::<0x143881f60, 0x14243b0b0, 0x14137b3d0, 0x8, 0x50, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141396fb0
    Replacement::draft_throws(0x141397030, "r_141397030", fn_141397030 as *const (), Throws::Discard), // sub_141397030
    Replacement::draft_throws(0x1413970e0, "r_1413970e0", tpl_997::<0x1413970f0, 0x1413970e9> as *const (), Throws::Discard), // sub_1413970e0
    Replacement::draft_throws(0x1413970f0, "r_1413970f0", fn_1413970f0 as *const (), Throws::Discard), // sub_1413970f0
    Replacement::draft_throws(0x141397180, "r_141397180", tpl_6985::<0x20, 0x8, 0x141398950> as *const (), Throws::Discard), // sub_141397180
    Replacement::draft_throws(0x1413971d0, "r_1413971d0", crate::shards::subsystems::fn_1413971d0 as *const (), Throws::Caught), // sub_1413971d0
    Replacement::draft_throws(0x141397200, "r_141397200", crate::shards::subsystems::fn_141397200 as *const (), Throws::Caught), // sub_141397200
    Replacement::draft(0x141397230, "r_141397230", fn_141397230 as *const ()), // sub_141397230
    Replacement::draft(0x1413973d0, "r_1413973d0", fn_1413973d0 as *const ()), // sub_1413973d0
    Replacement::draft(0x141397550, "r_141397550", fn_141397550 as *const ()), // sub_141397550
    Replacement::draft(0x141397670, "r_141397670", fn_141397670 as *const ()), // sub_141397670
    Replacement::draft_throws(0x141397810, "r_141397810", fn_141397810 as *const (), Throws::Caught), // sub_141397810
    Replacement::draft_throws(0x141397e00, "r_141397e00", fn_141397e00 as *const (), Throws::Caught), // sub_141397e00
    Replacement::draft_throws(0x1413983b0, "r_1413983b0", fn_1413983b0 as *const (), Throws::Caught), // sub_1413983b0
    Replacement::draft_throws(0x141398950, "r_141398950", fn_141398950 as *const (), Throws::Caught), // sub_141398950
    Replacement::draft_throws(0x1413989c0, "r_1413989c0", crate::shards::subsystems::fn_1413989c0 as *const (), Throws::Caught), // sub_1413989c0
    Replacement::draft_throws(0x141398d50, "r_141398d50", crate::shards::subsystems::fn_141398d50 as *const (), Throws::Caught), // sub_141398d50
    Replacement::draft_throws(0x1413990e0, "r_1413990e0", crate::shards::subsystems::fn_1413990e0 as *const (), Throws::Caught), // sub_1413990e0
    Replacement::draft_throws(0x1413995b0, "r_1413995b0", crate::shards::subsystems::fn_1413995b0 as *const (), Throws::Caught), // sub_1413995b0
    Replacement::draft_throws(0x141399a40, "r_141399a40", crate::shards::subsystems::fn_141399a40 as *const (), Throws::Caught), // sub_141399a40
    Replacement::draft_throws(0x141399ed0, "r_141399ed0", fn_141399ed0 as *const (), Throws::Discard), // sub_141399ed0
    Replacement::draft_throws(0x141399f00, "r_141399f00", fn_141399f00 as *const (), Throws::Caught), // sub_141399f00
    Replacement::draft_throws(0x14139a2a0, "r_14139a2a0", crate::shards::subsystems::fn_14139a2a0 as *const (), Throws::Caught), // sub_14139a2a0
    Replacement::draft_throws(0x14139a640, "r_14139a640", crate::shards::subsystems::fn_14139a640 as *const (), Throws::Caught), // sub_14139a640
    Replacement::draft_throws(0x14139a9e0, "r_14139a9e0", fn_14139a9e0 as *const (), Throws::Caught), // sub_14139a9e0
    Replacement::draft_throws(0x14139aa50, "r_14139aa50", fn_14139aa50 as *const (), Throws::Caught), // sub_14139aa50
    Replacement::draft_throws(0x14139ae20, "r_14139ae20", fn_14139ae20 as *const (), Throws::Caught), // sub_14139ae20
    Replacement::draft_throws(0x14139b1b0, "r_14139b1b0", fn_14139b1b0 as *const (), Throws::Caught), // sub_14139b1b0
    Replacement::draft(0x14139b230, "r_14139b230", fn_14139b230 as *const ()), // sub_14139b230
    Replacement::draft(0x14139b4a0, "r_14139b4a0", fn_14139b4a0 as *const ()), // sub_14139b4a0
    Replacement::draft(0x14139b660, "r_14139b660", fn_14139b660 as *const ()), // sub_14139b660
    Replacement::draft(0x14139b750, "r_14139b750", tpl_5539::<0x8, 0x14139b820, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14139b750
    Replacement::draft(0x14139b820, "r_14139b820", fn_14139b820 as *const ()), // sub_14139b820
    Replacement::draft(0x14139b8f0, "r_14139b8f0", fn_14139b8f0 as *const ()), // sub_14139b8f0
    Replacement::draft(0x14139b920, "r_14139b920", fn_14139b920 as *const ()), // sub_14139b920
    Replacement::draft(0x14139b950, "r_14139b950", fn_14139b950 as *const ()), // r::SystemException::r::SystemException::vfunc_1
    Replacement::draft_throws(0x14139b9e0, "r_14139b9e0", fn_14139b9e0 as *const (), Throws::Caught), // sub_14139b9e0
    Replacement::draft(0x14139ba50, "r_14139ba50", fn_14139ba50 as *const ()), // sub_14139ba50
    Replacement::draft(0x14139ba70, "r_14139ba70", fn_14139ba70 as *const ()), // r::SmallBlockMemoryResource::r::SmallBlockMemoryResource::vfunc_0
    Replacement::draft(0x14139eca0, "r_14139eca0", tpl_25168::<0x143409090, 0x18, 0x14243ef10, 0x142442de0, 0x141834d30> as *const ()), // r::MetadataProvider::r::MetadataProvider::vfunc_0
    Replacement::draft_throws(0x14139ed00, "r_14139ed00", fn_14139ed00 as *const (), Throws::Discard), // r::ResourceManager::r::ResourceManager::vfunc_0
    Replacement::draft(0x1413a3a50, "r_1413a3a50", fn_1413a3a50 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_0
    Replacement::draft_throws(0x1413a3af0, "r_1413a3af0", tpl_25197::<0x8, 0x19, 0x20, 0x10, 0x14024d8b0, 0x28, 0x18, 0x140208500, 0x140200130, 0x1413a3be1> as *const (), Throws::Discard), // sub_1413a3af0
    Replacement::draft(0x1413a3bf0, "r_1413a3bf0", fn_1413a3bf0 as *const ()), // sub_1413a3bf0
    Replacement::draft(0x1413a3cc0, "r_1413a3cc0", fn_1413a3cc0 as *const ()), // sub_1413a3cc0
    Replacement::draft(0x1413a3e80, "r_1413a3e80", fn_1413a3e80 as *const ()), // sub_1413a3e80
    Replacement::draft_throws(0x1413a3ec0, "r_1413a3ec0", fn_1413a3ec0 as *const (), Throws::Discard), // r::ProcessMonitorThread::r::ProcessMonitorThread::vfunc_0
    Replacement::draft(0x1413a3fa0, "r_1413a3fa0", fn_1413a3fa0 as *const ()), // r::ProcessBase::r::ProcessBase::vfunc_0
    Replacement::draft_throws(0x1413a3fe0, "r_1413a3fe0", fn_1413a3fe0 as *const (), Throws::Caught), // sub_1413a3fe0
    Replacement::draft(0x1413a4100, "r_1413a4100", fn_1413a4100 as *const ()), // sub_1413a4100
    Replacement::draft(0x1413a4120, "r_1413a4120", fn_1413a4120 as *const ()), // sub_1413a4120
    Replacement::draft_throws(0x1413a41b0, "r_1413a41b0", fn_1413a41b0 as *const (), Throws::Caught), // sub_1413a41b0
    Replacement::draft(0x1413a43e0, "r_1413a43e0", fn_1413a43e0 as *const ()), // sub_1413a43e0
    Replacement::draft(0x1413a4430, "r_1413a4430", tpl_20065::<0x1413a4500> as *const ()), // sub_1413a4430
    Replacement::draft(0x1413a4460, "r_1413a4460", fn_1413a4460 as *const ()), // sub_1413a4460
    Replacement::draft(0x1413a44b0, "r_1413a44b0", fn_1413a44b0 as *const ()), // sub_1413a44b0
    Replacement::draft(0x1413a4500, "r_1413a4500", fn_1413a4500 as *const ()), // sub_1413a4500
    Replacement::draft_throws(0x1413a45a0, "r_1413a45a0", fn_1413a45a0 as *const (), Throws::Caught), // sub_1413a45a0
    Replacement::draft(0x1413a46e0, "r_1413a46e0", fn_1413a46e0 as *const ()), // sub_1413a46e0
    Replacement::draft_throws(0x1413a4750, "r_1413a4750", fn_1413a4750 as *const (), Throws::Caught), // sub_1413a4750
    Replacement::draft_throws(0x1413a4810, "r_1413a4810", fn_1413a4810 as *const (), Throws::Caught), // sub_1413a4810
    Replacement::draft_throws(0x1413a48e0, "r_1413a48e0", fn_1413a48e0 as *const (), Throws::Caught), // sub_1413a48e0
    Replacement::draft(0x1413a49f0, "r_1413a49f0", fn_1413a49f0 as *const ()), // sub_1413a49f0
    Replacement::draft(0x1413a5000, "r_1413a5000", fn_1413a5000 as *const ()), // sub_1413a5000
    Replacement::draft(0x1413a5080, "r_1413a5080", fn_1413a5080 as *const ()), // sub_1413a5080
    Replacement::draft(0x1413a50e0, "r_1413a50e0", fn_1413a50e0 as *const ()), // sub_1413a50e0
    Replacement::draft_throws(0x1413a51e0, "r_1413a51e0", fn_1413a51e0 as *const (), Throws::Caught), // sub_1413a51e0
    Replacement::draft(0x1413a5360, "r_1413a5360", fn_1413a5360 as *const ()), // r::GlobalFileSystem<r::Pack2FileSystem>::r::GlobalFileSystem<r::Pack2FileSystem>::vfunc_0
    Replacement::draft(0x1413a53b0, "r_1413a53b0", fn_1413a53b0 as *const ()), // sub_1413a53b0
    Replacement::draft(0x1413a5540, "r_1413a5540", fn_1413a5540 as *const ()), // sub_1413a5540
    Replacement::draft(0x1413a55b0, "r_1413a55b0", fn_1413a55b0 as *const ()), // sub_1413a55b0
    Replacement::draft(0x1413a5710, "r_1413a5710", tpl_23484::<0x8, 0x1413a5800, 0x1401e5470> as *const ()), // sub_1413a5710
    Replacement::draft_throws(0x1413a5760, "r_1413a5760", fn_1413a5760 as *const (), Throws::Caught), // sub_1413a5760
    Replacement::draft(0x1413a5800, "r_1413a5800", fn_1413a5800 as *const ()), // sub_1413a5800
    Replacement::draft_throws(0x1413a5ac0, "r_1413a5ac0", fn_1413a5ac0 as *const (), Throws::Caught), // sub_1413a5ac0
    Replacement::draft_throws(0x1413a5d80, "r_1413a5d80", fn_1413a5d80 as *const (), Throws::Discard), // sub_1413a5d80
    Replacement::draft(0x1413a6170, "r_1413a6170", fn_1413a6170 as *const ()), // sub_1413a6170
    Replacement::draft(0x1413a6450, "r_1413a6450", fn_1413a6450 as *const ()), // sub_1413a6450
    Replacement::draft_throws(0x1413a6750, "r_1413a6750", fn_1413a6750 as *const (), Throws::Caught), // sub_1413a6750
    Replacement::draft_throws(0x1413a68c0, "r_1413a68c0", fn_1413a68c0 as *const (), Throws::Caught), // sub_1413a68c0
    Replacement::draft_throws(0x1413a6b90, "r_1413a6b90", fn_1413a6b90 as *const (), Throws::Discard), // sub_1413a6b90
    Replacement::draft(0x1413a6c60, "r_1413a6c60", fn_1413a6c60 as *const ()), // sub_1413a6c60
    Replacement::draft(0x1413a6d90, "r_1413a6d90", fn_1413a6d90 as *const ()), // sub_1413a6d90
    Replacement::draft(0x1413a7120, "r_1413a7120", fn_1413a7120 as *const ()), // sub_1413a7120
    Replacement::draft(0x1413a71b0, "r_1413a71b0", tpl_23818::<0x8, 0x1413a7120, 0x142442de0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413a71b0
    Replacement::draft(0x1413a7290, "r_1413a7290", tpl_743::<0x30, 0x10, 0x18, 0x143409ad8, 0x8, 0x1401f3d90, 0x20, 0x143409b08, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1413a7290
    Replacement::draft(0x1413a7360, "r_1413a7360", tpl_743::<0x30, 0x10, 0x18, 0x143409b20, 0x8, 0x1401f3d90, 0x20, 0x143409b38, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1413a7360
    Replacement::draft(0x1413a7430, "r_1413a7430", tpl_743::<0x30, 0x10, 0x18, 0x143409b50, 0x8, 0x140209650, 0x20, 0x143409b68, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1413a7430
    Replacement::draft(0x1413a7500, "r_1413a7500", tpl_743::<0x30, 0x10, 0x18, 0x143409b80, 0x8, 0x1401f3d90, 0x20, 0x143409b98, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1413a7500
    Replacement::draft(0x1413a75d0, "r_1413a75d0", tpl_743::<0x30, 0x10, 0x18, 0x143409bb0, 0x8, 0x1401f3d90, 0x20, 0x143409bc8, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1413a75d0
    Replacement::draft(0x1413a76a0, "r_1413a76a0", fn_1413a76a0 as *const ()), // sub_1413a76a0
    Replacement::draft(0x1413a77e0, "r_1413a77e0", fn_1413a77e0 as *const ()), // sub_1413a77e0
    Replacement::draft(0x1413a78b0, "r_1413a78b0", fn_1413a78b0 as *const ()), // sub_1413a78b0
    Replacement::draft(0x1413a7920, "r_1413a7920", tpl_23561::<0x140209650, 0x1413a7b00, 0x8, 0x10, 0x1413a4500> as *const ()), // sub_1413a7920
    Replacement::draft(0x1413a7990, "r_1413a7990", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1413a71b0> as *const ()), // sub_1413a7990
    Replacement::draft(0x1413a7b00, "r_1413a7b00", fn_1413a7b00 as *const ()), // sub_1413a7b00
    Replacement::draft(0x1413a7c80, "r_1413a7c80", tpl_25241::<0x142436a50, 0x141834d30> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_0
    Replacement::draft(0x1413aa7c0, "r_1413aa7c0", tpl_1861::<0x14243d450, 0x141834d30> as *const ()), // r::MonotonicMemoryResource::r::MonotonicMemoryResource::vfunc_0
    Replacement::draft(0x1413aa800, "r_1413aa800", fn_1413aa800 as *const ()), // sub_1413aa800
    Replacement::draft_throws(0x1413aa940, "r_1413aa940", fn_1413aa940 as *const (), Throws::Caught), // sub_1413aa940
    Replacement::draft(0x1413aad10, "r_1413aad10", fn_1413aad10 as *const ()), // sub_1413aad10
    Replacement::draft_throws(0x1413aae10, "r_1413aae10", fn_1413aae10 as *const (), Throws::Caught), // sub_1413aae10
    Replacement::draft_throws(0x1413ab1b0, "r_1413ab1b0", fn_1413ab1b0 as *const (), Throws::Discard), // sub_1413ab1b0
    Replacement::draft(0x1413ab270, "r_1413ab270", fn_1413ab270 as *const ()), // r::MetadataManager::LoadQueue::r::MetadataManager::LoadQueue::vfunc_1
    Replacement::draft(0x1413ab390, "r_1413ab390", fn_1413ab390 as *const ()), // r::MetadataManager::LoadQueue::r::MetadataManager::LoadQueue::vfunc_0
    Replacement::draft_throws(0x1413ab3e0, "r_1413ab3e0", fn_1413ab3e0 as *const (), Throws::Caught), // r::MetadataManager::LockedQueue::r::MetadataManager::LockedQueue::vfunc_1
    Replacement::draft(0x1413ab9a0, "r_1413ab9a0", fn_1413ab9a0 as *const ()), // r::MetadataManager::LoadBatch::r::MetadataManager::LoadBatch::vfunc_0
    Replacement::draft(0x1413aba10, "r_1413aba10", fn_1413aba10 as *const ()), // sub_1413aba10
    Replacement::draft(0x1413aba70, "r_1413aba70", tpl_4771::<0x30, 0x1413ac7e0, 0x142442de0> as *const ()), // sub_1413aba70
    Replacement::draft(0x1413abaa0, "r_1413abaa0", fn_1413abaa0 as *const ()), // sub_1413abaa0
    Replacement::draft(0x1413abb00, "r_1413abb00", tpl_19540::<0x80, 0x1401e3e80, 0x68, 0x1413ad4c0, 0x8, 0x140227190, 0x142442de0> as *const ()), // sub_1413abb00
    Replacement::draft(0x1413abb40, "r_1413abb40", tpl_4771::<0x10, 0x1413ac640, 0x142442de0> as *const ()), // sub_1413abb40
    Replacement::draft(0x1413abb70, "r_1413abb70", fn_1413abb70 as *const ()), // sub_1413abb70
    Replacement::draft(0x1413abd80, "r_1413abd80", fn_1413abd80 as *const ()), // sub_1413abd80
    Replacement::draft(0x1413abe30, "r_1413abe30", tpl_6429::<0xc, 0x8, 0x1401e3cd0> as *const ()), // sub_1413abe30
    Replacement::draft(0x1413abeb0, "r_1413abeb0", fn_1413abeb0 as *const ()), // sub_1413abeb0
    Replacement::draft_throws(0x1413abf50, "r_1413abf50", fn_1413abf50 as *const (), Throws::Caught), // sub_1413abf50
    Replacement::draft_throws(0x1413abff0, "r_1413abff0", fn_1413abff0 as *const (), Throws::Caught), // sub_1413abff0
    Replacement::draft_throws(0x1413ac130, "r_1413ac130", fn_1413ac130 as *const (), Throws::Discard), // sub_1413ac130
    Replacement::draft(0x1413ac2a0, "r_1413ac2a0", fn_1413ac2a0 as *const ()), // sub_1413ac2a0
    Replacement::draft(0x1413ac380, "r_1413ac380", fn_1413ac380 as *const ()), // sub_1413ac380
    Replacement::draft(0x1413ac460, "r_1413ac460", fn_1413ac460 as *const ()), // r::MetadataManager::LockedQueue::r::MetadataManager::LockedQueue::vfunc_0
    Replacement::draft(0x1413ac4a0, "r_1413ac4a0", fn_1413ac4a0 as *const ()), // sub_1413ac4a0
    Replacement::draft(0x1413ac640, "r_1413ac640", fn_1413ac640 as *const ()), // sub_1413ac640
    Replacement::draft_throws(0x1413ac760, "r_1413ac760", fn_1413ac760 as *const (), Throws::Discard), // sub_1413ac760
    Replacement::draft(0x1413ac7e0, "r_1413ac7e0", fn_1413ac7e0 as *const ()), // sub_1413ac7e0
    Replacement::draft_throws(0x1413ac8d0, "r_1413ac8d0", fn_1413ac8d0 as *const (), Throws::Discard), // sub_1413ac8d0
    Replacement::draft_throws(0x1413ac9b0, "r_1413ac9b0", fn_1413ac9b0 as *const (), Throws::Discard), // sub_1413ac9b0
    Replacement::draft(0x1413aca40, "r_1413aca40", tpl_12610::<0x8, 0x1413acb00, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413aca40
    Replacement::draft(0x1413acb00, "r_1413acb00", fn_1413acb00 as *const ()), // sub_1413acb00
    Replacement::draft_throws(0x1413acb60, "r_1413acb60", fn_1413acb60 as *const (), Throws::Discard), // sub_1413acb60
    Replacement::draft(0x1413acc00, "r_1413acc00", fn_1413acc00 as *const ()), // sub_1413acc00
    Replacement::draft_throws(0x1413acde0, "r_1413acde0", fn_1413acde0 as *const (), Throws::Discard), // sub_1413acde0
    Replacement::draft(0x1413acf60, "r_1413acf60", fn_1413acf60 as *const ()), // sub_1413acf60
    Replacement::draft(0x1413acfe0, "r_1413acfe0", fn_1413acfe0 as *const ()), // sub_1413acfe0
    Replacement::draft_throws(0x1413ad040, "r_1413ad040", fn_1413ad040 as *const (), Throws::Caught), // sub_1413ad040
    Replacement::draft_throws(0x1413ad200, "r_1413ad200", fn_1413ad200 as *const (), Throws::Discard), // r::JobCallable<r::parallelFor<unsigned __int64,r::MetadataManager::LoadBatch::push::<lambda_1>,?bref5::parallelfordetail
    Replacement::draft(0x1413ad240, "r_1413ad240", fn_1413ad240 as *const ()), // sub_1413ad240
    Replacement::draft_throws(0x1413ad290, "r_1413ad290", fn_1413ad290 as *const (), Throws::Discard), // r::JobCallable<r::parallelfordetail::parallelForJob<unsigned __int64,const r::MetadataManager::LoadBatch::push::<lambda_
    Replacement::draft(0x1413ad7e0, "r_1413ad7e0", fn_1413ad7e0 as *const ()), // sub_1413ad7e0
    Replacement::draft(0x1413ad870, "r_1413ad870", fn_1413ad870 as *const ()), // sub_1413ad870
    Replacement::draft(0x1413ad920, "r_1413ad920", fn_1413ad920 as *const ()), // sub_1413ad920
    Replacement::draft(0x1413ad9b0, "r_1413ad9b0", fn_1413ad9b0 as *const ()), // sub_1413ad9b0
    Replacement::draft_throws(0x1413adab0, "r_1413adab0", fn_1413adab0 as *const (), Throws::Discard), // sub_1413adab0
    Replacement::draft(0x1413adb90, "r_1413adb90", fn_1413adb90 as *const ()), // sub_1413adb90
    Replacement::draft_throws(0x1413adc40, "r_1413adc40", fn_1413adc40 as *const (), Throws::Discard), // sub_1413adc40
    Replacement::draft_throws(0x1413add40, "r_1413add40", crate::shards::subsystems::fn_1413add40 as *const (), Throws::Caught), // sub_1413add40
    Replacement::draft_throws(0x1413ade40, "r_1413ade40", crate::shards::subsystems::fn_1413ade40 as *const (), Throws::Caught), // sub_1413ade40
    Replacement::draft_throws(0x1413adf40, "r_1413adf40", crate::shards::subsystems::fn_1413adf40 as *const (), Throws::Caught), // sub_1413adf40
    Replacement::draft(0x1413ae040, "r_1413ae040", fn_1413ae040 as *const ()), // sub_1413ae040
    Replacement::draft(0x1413ae100, "r_1413ae100", fn_1413ae100 as *const ()), // sub_1413ae100
    Replacement::draft(0x1413ae1b0, "r_1413ae1b0", fn_1413ae1b0 as *const ()), // sub_1413ae1b0
    Replacement::draft_throws(0x1413ae270, "r_1413ae270", fn_1413ae270 as *const (), Throws::Discard), // sub_1413ae270
    Replacement::draft_throws(0x1413ae330, "r_1413ae330", fn_1413ae330 as *const (), Throws::Discard), // sub_1413ae330
    Replacement::draft(0x1413ae420, "r_1413ae420", fn_1413ae420 as *const ()), // sub_1413ae420
    Replacement::draft(0x1413ae4c0, "r_1413ae4c0", fn_1413ae4c0 as *const ()), // sub_1413ae4c0
    Replacement::draft(0x1413ae580, "r_1413ae580", fn_1413ae580 as *const ()), // sub_1413ae580
    Replacement::draft(0x1413ae690, "r_1413ae690", fn_1413ae690 as *const ()), // sub_1413ae690
    Replacement::draft_throws(0x1413ae710, "r_1413ae710", fn_1413ae710 as *const (), Throws::Discard), // sub_1413ae710
    Replacement::draft(0x1413ae7c0, "r_1413ae7c0", fn_1413ae7c0 as *const ()), // sub_1413ae7c0
    Replacement::draft_throws(0x1413ae890, "r_1413ae890", fn_1413ae890 as *const (), Throws::Discard), // sub_1413ae890
    Replacement::draft(0x1413ae960, "r_1413ae960", fn_1413ae960 as *const ()), // sub_1413ae960
    Replacement::draft_throws(0x1413ae9d0, "r_1413ae9d0", crate::shards::subsystems::fn_1413ae9d0 as *const (), Throws::Caught), // sub_1413ae9d0
    Replacement::draft_throws(0x1413aeaa0, "r_1413aeaa0", crate::shards::subsystems::fn_1413aeaa0 as *const (), Throws::Caught), // sub_1413aeaa0
    Replacement::draft_throws(0x1413aeb70, "r_1413aeb70", crate::shards::subsystems::fn_1413aeb70 as *const (), Throws::Caught), // sub_1413aeb70
    Replacement::draft(0x1413aec40, "r_1413aec40", fn_1413aec40 as *const ()), // sub_1413aec40
    Replacement::draft(0x1413aed10, "r_1413aed10", fn_1413aed10 as *const ()), // sub_1413aed10
    Replacement::draft(0x1413aedf0, "r_1413aedf0", fn_1413aedf0 as *const ()), // sub_1413aedf0
    Replacement::draft_throws(0x1413aeec0, "r_1413aeec0", fn_1413aeec0 as *const (), Throws::Discard), // sub_1413aeec0
    Replacement::draft_throws(0x1413aef70, "r_1413aef70", fn_1413aef70 as *const (), Throws::Discard), // sub_1413aef70
    Replacement::draft(0x1413af030, "r_1413af030", fn_1413af030 as *const ()), // sub_1413af030
    Replacement::draft(0x1413af0c0, "r_1413af0c0", fn_1413af0c0 as *const ()), // sub_1413af0c0
    Replacement::draft(0x1413af170, "r_1413af170", tpl_13458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x14340b238, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_1413af170
    Replacement::draft(0x1413af220, "r_1413af220", fn_1413af220 as *const ()), // sub_1413af220
    Replacement::draft_throws(0x1413af2a0, "r_1413af2a0", fn_1413af2a0 as *const (), Throws::Discard), // sub_1413af2a0
    Replacement::draft(0x1413af3b0, "r_1413af3b0", fn_1413af3b0 as *const ()), // sub_1413af3b0
    Replacement::draft_throws(0x1413af4e0, "r_1413af4e0", fn_1413af4e0 as *const (), Throws::Discard), // sub_1413af4e0
    Replacement::draft(0x1413af620, "r_1413af620", fn_1413af620 as *const ()), // sub_1413af620
    Replacement::draft_throws(0x1413af6f0, "r_1413af6f0", crate::shards::subsystems::fn_1413af6f0 as *const (), Throws::Caught), // sub_1413af6f0
    Replacement::draft_throws(0x1413af7c0, "r_1413af7c0", crate::shards::subsystems::fn_1413af7c0 as *const (), Throws::Caught), // sub_1413af7c0
    Replacement::draft_throws(0x1413af890, "r_1413af890", crate::shards::subsystems::fn_1413af890 as *const (), Throws::Caught), // sub_1413af890
    Replacement::draft(0x1413af960, "r_1413af960", fn_1413af960 as *const ()), // sub_1413af960
    Replacement::draft(0x1413afa30, "r_1413afa30", fn_1413afa30 as *const ()), // sub_1413afa30
    Replacement::draft(0x1413afb40, "r_1413afb40", fn_1413afb40 as *const ()), // sub_1413afb40
    Replacement::draft_throws(0x1413afc60, "r_1413afc60", fn_1413afc60 as *const (), Throws::Discard), // sub_1413afc60
    Replacement::draft_throws(0x1413afd70, "r_1413afd70", fn_1413afd70 as *const (), Throws::Discard), // sub_1413afd70
    Replacement::draft(0x1413afe30, "r_1413afe30", fn_1413afe30 as *const ()), // sub_1413afe30
    Replacement::draft(0x1413aff50, "r_1413aff50", fn_1413aff50 as *const ()), // sub_1413aff50
    Replacement::draft_throws(0x1413b0070, "r_1413b0070", fn_1413b0070 as *const (), Throws::Discard), // sub_1413b0070
    Replacement::draft(0x1413b0170, "r_1413b0170", fn_1413b0170 as *const ()), // sub_1413b0170
    Replacement::draft_throws(0x1413b0210, "r_1413b0210", fn_1413b0210 as *const (), Throws::Discard), // sub_1413b0210
    Replacement::draft_throws(0x1413b0320, "r_1413b0320", fn_1413b0320 as *const (), Throws::Caught), // sub_1413b0320
    Replacement::draft_throws(0x1413b05b0, "r_1413b05b0", fn_1413b05b0 as *const (), Throws::Caught), // sub_1413b05b0
    Replacement::draft_throws(0x1413b0810, "r_1413b0810", fn_1413b0810 as *const (), Throws::Caught), // sub_1413b0810
    Replacement::draft(0x1413b0a70, "r_1413b0a70", fn_1413b0a70 as *const ()), // sub_1413b0a70
    Replacement::draft(0x1413b0b10, "r_1413b0b10", fn_1413b0b10 as *const ()), // sub_1413b0b10
    Replacement::draft(0x1413b0bc0, "r_1413b0bc0", fn_1413b0bc0 as *const ()), // sub_1413b0bc0
    Replacement::draft_throws(0x1413b0c80, "r_1413b0c80", fn_1413b0c80 as *const (), Throws::Discard), // sub_1413b0c80
    Replacement::draft_throws(0x1413b0d40, "r_1413b0d40", fn_1413b0d40 as *const (), Throws::Discard), // sub_1413b0d40
    Replacement::draft(0x1413b0e70, "r_1413b0e70", tpl_23623::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14340ad58, 0x8> as *const ()), // sub_1413b0e70
    Replacement::draft(0x1413b0f10, "r_1413b0f10", tpl_13469::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x14340ad58, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_1413b0f10
    Replacement::draft(0x1413b0fc0, "r_1413b0fc0", fn_1413b0fc0 as *const ()), // sub_1413b0fc0
    Replacement::draft(0x1413b1090, "r_1413b1090", fn_1413b1090 as *const ()), // sub_1413b1090
    Replacement::draft_throws(0x1413b1180, "r_1413b1180", fn_1413b1180 as *const (), Throws::Discard), // sub_1413b1180
    Replacement::draft(0x1413b1220, "r_1413b1220", fn_1413b1220 as *const ()), // sub_1413b1220
    Replacement::draft_throws(0x1413b12e0, "r_1413b12e0", fn_1413b12e0 as *const (), Throws::Discard), // sub_1413b12e0
    Replacement::draft(0x1413b1390, "r_1413b1390", fn_1413b1390 as *const ()), // sub_1413b1390
    Replacement::draft_throws(0x1413b1410, "r_1413b1410", crate::shards::subsystems::fn_1413b1410 as *const (), Throws::Caught), // sub_1413b1410
    Replacement::draft_throws(0x1413b14c0, "r_1413b14c0", crate::shards::subsystems::fn_1413b14c0 as *const (), Throws::Caught), // sub_1413b14c0
    Replacement::draft_throws(0x1413b1570, "r_1413b1570", crate::shards::subsystems::fn_1413b1570 as *const (), Throws::Caught), // sub_1413b1570
    Replacement::draft(0x1413b1620, "r_1413b1620", fn_1413b1620 as *const ()), // sub_1413b1620
    Replacement::draft(0x1413b16d0, "r_1413b16d0", fn_1413b16d0 as *const ()), // sub_1413b16d0
    Replacement::draft(0x1413b1780, "r_1413b1780", fn_1413b1780 as *const ()), // sub_1413b1780
    Replacement::draft_throws(0x1413b1840, "r_1413b1840", fn_1413b1840 as *const (), Throws::Discard), // sub_1413b1840
    Replacement::draft_throws(0x1413b18e0, "r_1413b18e0", fn_1413b18e0 as *const (), Throws::Discard), // sub_1413b18e0
    Replacement::draft(0x1413b19b0, "r_1413b19b0", tpl_12588::<0x1401de830, 0x1430b6bd0, 0x141834d30> as *const ()), // r::ResourceMetadata::r::ResourceMetadata::vfunc_0
    Replacement::draft(0x1413b1a00, "r_1413b1a00", tpl_13661::<0x140e71010, 0x10, 0x1430b6bd0, 0x141834d30> as *const ()), // r::DevelopmentResourceMetadata::r::DevelopmentResourceMetadata::vfunc_0
    Replacement::draft(0x1413b1aa0, "r_1413b1aa0", fn_1413b1aa0 as *const ()), // sub_1413b1aa0
    Replacement::draft(0x1413b1b90, "r_1413b1b90", fn_1413b1b90 as *const ()), // sub_1413b1b90
    Replacement::draft(0x1413b1c10, "r_1413b1c10", fn_1413b1c10 as *const ()), // sub_1413b1c10
    Replacement::draft_throws(0x1413b1dc0, "r_1413b1dc0", crate::shards::subsystems::fn_1413b1dc0 as *const (), Throws::Caught), // sub_1413b1dc0
    Replacement::draft_throws(0x1413b2080, "r_1413b2080", crate::shards::subsystems::fn_1413b2080 as *const (), Throws::Caught), // sub_1413b2080
    Replacement::draft_throws(0x1413b2340, "r_1413b2340", fn_1413b2340 as *const (), Throws::Caught), // sub_1413b2340
    Replacement::draft_throws(0x1413b27b0, "r_1413b27b0", fn_1413b27b0 as *const (), Throws::Caught), // sub_1413b27b0
    Replacement::draft_throws(0x1413b2bf0, "r_1413b2bf0", fn_1413b2bf0 as *const (), Throws::Caught), // sub_1413b2bf0
    Replacement::draft_throws(0x1413b3030, "r_1413b3030", crate::shards::subsystems::fn_1413b3030 as *const (), Throws::Caught), // sub_1413b3030
    Replacement::draft_throws(0x1413b3100, "r_1413b3100", crate::shards::subsystems::fn_1413b3100 as *const (), Throws::Caught), // sub_1413b3100
    Replacement::draft_throws(0x1413b35d0, "r_1413b35d0", crate::shards::subsystems::fn_1413b35d0 as *const (), Throws::Caught), // sub_1413b35d0
    Replacement::draft_throws(0x1413b3a60, "r_1413b3a60", crate::shards::subsystems::fn_1413b3a60 as *const (), Throws::Caught), // sub_1413b3a60
    Replacement::draft_throws(0x1413b3ef0, "r_1413b3ef0", fn_1413b3ef0 as *const (), Throws::Caught), // sub_1413b3ef0
    Replacement::draft_throws(0x1413b4150, "r_1413b4150", crate::shards::subsystems::fn_1413b4150 as *const (), Throws::Caught), // sub_1413b4150
    Replacement::draft_throws(0x1413b43a0, "r_1413b43a0", crate::shards::subsystems::fn_1413b43a0 as *const (), Throws::Caught), // sub_1413b43a0
    Replacement::draft_throws(0x1413b45f0, "r_1413b45f0", crate::shards::subsystems::fn_1413b45f0 as *const (), Throws::Caught), // sub_1413b45f0
    Replacement::draft_throws(0x1413b47e0, "r_1413b47e0", fn_1413b47e0 as *const (), Throws::Caught), // sub_1413b47e0
    Replacement::draft_throws(0x1413b4d50, "r_1413b4d50", fn_1413b4d50 as *const (), Throws::Caught), // sub_1413b4d50
    Replacement::draft_throws(0x1413b5280, "r_1413b5280", crate::shards::subsystems::fn_1413b5280 as *const (), Throws::Caught), // sub_1413b5280
    Replacement::draft(0x1413b55f0, "r_1413b55f0", tpl_24940::<0x14340ed48, 0x8, 0x14340b558, 0x14340b3c0, 0x142498f00> as *const ()), // sub_1413b55f0
    Replacement::draft(0x1413b5650, "r_1413b5650", tpl_24940::<0x14340ed48, 0x8, 0x14340b540, 0x14340b378, 0x142498f00> as *const ()), // sub_1413b5650
    Replacement::draft(0x1413b56b0, "r_1413b56b0", tpl_24940::<0x14340ed48, 0x8, 0x14340b528, 0x14340b3a0, 0x142498f00> as *const ()), // sub_1413b56b0
    Replacement::draft(0x1413b56f0, "r_1413b56f0", fn_1413b56f0 as *const ()), // r::MemoryFile::r::MemoryFile::vfunc_0
    Replacement::draft(0x1413b5730, "r_1413b5730", fn_1413b5730 as *const ()), // sub_1413b5730
    Replacement::draft_throws(0x1413b5740, "r_1413b5740", fn_1413b5740 as *const (), Throws::Caught), // sub_1413b5740
    Replacement::draft_throws(0x1413b57b0, "r_1413b57b0", fn_1413b57b0 as *const (), Throws::Caught), // sub_1413b57b0
    Replacement::draft(0x1413b5820, "r_1413b5820", fn_1413b5820 as *const ()), // sub_1413b5820
    Replacement::draft_throws(0x1413b5850, "r_1413b5850", fn_1413b5850 as *const (), Throws::Caught), // sub_1413b5850
    Replacement::draft(0x1413bba30, "r_1413bba30", fn_1413bba30 as *const ()), // r::Loader_CallbackQueue::r::Loader_CallbackQueue::vfunc_0
    Replacement::draft(0x1413bbab0, "r_1413bbab0", tpl_5650::<0x1413bbae0, 0x8, 0x10> as *const ()), // sub_1413bbab0
    Replacement::draft(0x1413bbae0, "r_1413bbae0", fn_1413bbae0 as *const ()), // sub_1413bbae0
    Replacement::draft(0x1413bbc00, "r_1413bbc00", fn_1413bbc00 as *const ()), // sub_1413bbc00
    Replacement::draft(0x1413bbda0, "r_1413bbda0", fn_1413bbda0 as *const ()), // sub_1413bbda0
    Replacement::draft(0x1413bbe20, "r_1413bbe20", fn_1413bbe20 as *const ()), // sub_1413bbe20
    Replacement::draft(0x1413bbf10, "r_1413bbf10", fn_1413bbf10 as *const ()), // sub_1413bbf10
    Replacement::draft_throws(0x1413bc1e0, "r_1413bc1e0", fn_1413bc1e0 as *const (), Throws::Caught), // sub_1413bc1e0
    Replacement::draft(0x1413bc2e0, "r_1413bc2e0", fn_1413bc2e0 as *const ()), // sub_1413bc2e0
    Replacement::draft(0x1413bc3c0, "r_1413bc3c0", fn_1413bc3c0 as *const ()), // sub_1413bc3c0
    Replacement::draft(0x1413bc4c0, "r_1413bc4c0", fn_1413bc4c0 as *const ()), // sub_1413bc4c0
    Replacement::draft(0x1413bc5c0, "r_1413bc5c0", fn_1413bc5c0 as *const ()), // sub_1413bc5c0
    Replacement::draft(0x1413bc780, "r_1413bc780", fn_1413bc780 as *const ()), // sub_1413bc780
    Replacement::draft_throws(0x1413bc910, "r_1413bc910", fn_1413bc910 as *const (), Throws::Discard), // sub_1413bc910
    Replacement::draft(0x1413bcaf0, "r_1413bcaf0", tpl_381::<0x1413bccd0, 0x1413bcf50> as *const ()), // sub_1413bcaf0
    Replacement::draft(0x1413bcc50, "r_1413bcc50", fn_1413bcc50 as *const ()), // sub_1413bcc50
    Replacement::draft(0x1413bccd0, "r_1413bccd0", fn_1413bccd0 as *const ()), // sub_1413bccd0
    Replacement::draft(0x1413bcf50, "r_1413bcf50", fn_1413bcf50 as *const ()), // sub_1413bcf50
    Replacement::draft_throws(0x1413bd290, "r_1413bd290", fn_1413bd290 as *const (), Throws::Discard), // sub_1413bd290
    Replacement::draft(0x1413bd450, "r_1413bd450", fn_1413bd450 as *const ()), // sub_1413bd450
    Replacement::draft(0x1413bd4f0, "r_1413bd4f0", fn_1413bd4f0 as *const ()), // sub_1413bd4f0
    Replacement::draft(0x1413bd5a0, "r_1413bd5a0", fn_1413bd5a0 as *const ()), // sub_1413bd5a0
    Replacement::draft_throws(0x1413bd690, "r_1413bd690", fn_1413bd690 as *const (), Throws::Caught), // sub_1413bd690
    Replacement::draft_throws(0x1413bdae0, "r_1413bdae0", fn_1413bdae0 as *const (), Throws::Caught), // sub_1413bdae0
    Replacement::draft(0x1413bdd90, "r_1413bdd90", fn_1413bdd90 as *const ()), // sub_1413bdd90
    Replacement::draft_throws(0x1413bdee0, "r_1413bdee0", fn_1413bdee0 as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::ExecutorThread::r::Loader_Adaptor_FileSystem::ExecutorThread::vfunc_1
    Replacement::draft_throws(0x1413bdff0, "r_1413bdff0", tpl_25461::<0x1423f0c20, 0x141834d30> as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::ExecutorThread::r::Loader_Adaptor_FileSystem::ExecutorThread::vfunc_0
    Replacement::draft_throws(0x1413be040, "r_1413be040", fn_1413be040 as *const (), Throws::Caught), // r::Loader_Adaptor_FileSystem::SyncExecutorThread::r::Loader_Adaptor_FileSystem::SyncExecutorThread::vfunc_2
    Replacement::draft_throws(0x1413be190, "r_1413be190", tpl_25463::<0x14340cc98, 0x108, 0x142645748, 0x1423f0c20, 0x141834d30, 0x143406c90, 0x1423f0780, 0x14136eec0, 0x1413be207> as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::SyncExecutorThread::r::Loader_Adaptor_FileSystem::SyncExecutorThread::vfunc_0
    Replacement::draft_throws(0x1413be210, "r_1413be210", fn_1413be210 as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::AsyncExecutorThread::r::Loader_Adaptor_FileSystem::AsyncExecutorThread::vfunc_2
    Replacement::draft(0x1413be8e0, "r_1413be8e0", fn_1413be8e0 as *const ()), // sub_1413be8e0
    Replacement::draft(0x1413beb20, "r_1413beb20", fn_1413beb20 as *const ()), // sub_1413beb20
    Replacement::draft_throws(0x1413bec80, "r_1413bec80", tpl_25463::<0x14340cc58, 0x108, 0x142645748, 0x1423f0c20, 0x141834d30, 0x143406c90, 0x1423f0780, 0x14136eec0, 0x1413becf7> as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::AsyncExecutorThread::r::Loader_Adaptor_FileSystem::AsyncExecutorThread::vfunc_0
    Replacement::draft(0x1413bed10, "r_1413bed10", fn_1413bed10 as *const ()), // sub_1413bed10
    Replacement::draft(0x1413bed70, "r_1413bed70", fn_1413bed70 as *const ()), // sub_1413bed70
    Replacement::draft(0x1413bede0, "r_1413bede0", tpl_22340::<0x14247c4f0, 0x14247cb90> as *const ()), // sub_1413bede0
    Replacement::draft(0x1413bee30, "r_1413bee30", fn_1413bee30 as *const ()), // sub_1413bee30
    Replacement::draft(0x1413beeb0, "r_1413beeb0", fn_1413beeb0 as *const ()), // sub_1413beeb0
    Replacement::draft(0x1413bef70, "r_1413bef70", fn_1413bef70 as *const ()), // sub_1413bef70
    Replacement::draft(0x1413bf020, "r_1413bf020", tpl_20724::<0x1413bfe10, 0x1425371a0, 0x142533200, 0x142442de0, 0x1413beb20, 0x1401f3c00> as *const ()), // sub_1413bf020
    Replacement::draft(0x1413bf2f0, "r_1413bf2f0", fn_1413bf2f0 as *const ()), // sub_1413bf2f0
    Replacement::draft_throws(0x1413bf440, "r_1413bf440", crate::shards::subsystems::fn_1413bf440 as *const (), Throws::Caught), // sub_1413bf440
    Replacement::draft_throws(0x1413bf530, "r_1413bf530", crate::shards::subsystems::fn_1413bf530 as *const (), Throws::Caught), // sub_1413bf530
    Replacement::draft(0x1413bf640, "r_1413bf640", fn_1413bf640 as *const ()), // sub_1413bf640
    Replacement::draft(0x1413bf6c0, "r_1413bf6c0", fn_1413bf6c0 as *const ()), // sub_1413bf6c0
    Replacement::draft_throws(0x1413bf790, "r_1413bf790", fn_1413bf790 as *const (), Throws::Discard), // sub_1413bf790
    Replacement::draft_throws(0x1413bf850, "r_1413bf850", fn_1413bf850 as *const (), Throws::Discard), // sub_1413bf850
    Replacement::draft(0x1413bf8b0, "r_1413bf8b0", fn_1413bf8b0 as *const ()), // sub_1413bf8b0
    Replacement::draft(0x1413bf990, "r_1413bf990", fn_1413bf990 as *const ()), // sub_1413bf990
    Replacement::draft(0x1413bfe10, "r_1413bfe10", tpl_24966::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14340cd08, 0x8, 0xc, 0x14, 0x1401f3d90, 0x14340cd20> as *const ()), // sub_1413bfe10
    Replacement::draft(0x1413bff00, "r_1413bff00", tpl_25480::<0x1401f3d00, 0x8, 0x1413bf8b0, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::Loader_Adaptor_FileSystem::AsyncExecutorThread::processFileLoads::?bref1::operator()::<
    Replacement::draft(0x1413bff50, "r_1413bff50", fn_1413bff50 as *const ()), // sub_1413bff50
    Replacement::draft(0x1413c0210, "r_1413c0210", fn_1413c0210 as *const ()), // sub_1413c0210
    Replacement::draft(0x1413c0a80, "r_1413c0a80", fn_1413c0a80 as *const ()), // sub_1413c0a80
    Replacement::draft(0x1413c0c10, "r_1413c0c10", fn_1413c0c10 as *const ()), // sub_1413c0c10
    Replacement::draft(0x1413c0c70, "r_1413c0c70", fn_1413c0c70 as *const ()), // sub_1413c0c70
    Replacement::draft(0x1413c0e50, "r_1413c0e50", fn_1413c0e50 as *const ()), // sub_1413c0e50
    Replacement::draft(0x1413c0f60, "r_1413c0f60", fn_1413c0f60 as *const ()), // sub_1413c0f60
    Replacement::draft(0x1413c0fd0, "r_1413c0fd0", fn_1413c0fd0 as *const ()), // sub_1413c0fd0
    Replacement::draft(0x1413c1020, "r_1413c1020", fn_1413c1020 as *const ()), // sub_1413c1020
    Replacement::draft(0x1413c1080, "r_1413c1080", tpl_25490::<0x1413c1140, 0x38, 0x141834ee8> as *const ()), // sub_1413c1080
    Replacement::draft(0x1413c10b0, "r_1413c10b0", fn_1413c10b0 as *const ()), // sub_1413c10b0
    Replacement::draft(0x1413c10f0, "r_1413c10f0", fn_1413c10f0 as *const ()), // sub_1413c10f0
    Replacement::draft(0x1413c1140, "r_1413c1140", fn_1413c1140 as *const ()), // sub_1413c1140
    Replacement::draft(0x1413c11c0, "r_1413c11c0", tpl_477::<0x8, 0x142442de0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413c11c0
    Replacement::draft(0x1413c12a0, "r_1413c12a0", fn_1413c12a0 as *const ()), // sub_1413c12a0
    Replacement::draft(0x1413c1340, "r_1413c1340", fn_1413c1340 as *const ()), // sub_1413c1340
    Replacement::draft_throws(0x1413c1470, "r_1413c1470", fn_1413c1470 as *const (), Throws::Caught), // sub_1413c1470
    Replacement::draft(0x1413c15f0, "r_1413c15f0", fn_1413c15f0 as *const ()), // sub_1413c15f0
    Replacement::draft_throws(0x1413c1ef0, "r_1413c1ef0", crate::shards::subsystems::fn_1413c1ef0 as *const (), Throws::Caught), // sub_1413c1ef0
    Replacement::draft_throws(0x1413c2100, "r_1413c2100", crate::shards::subsystems::fn_1413c2100 as *const (), Throws::Caught), // sub_1413c2100
    Replacement::draft_throws(0x1413c29d0, "r_1413c29d0", fn_1413c29d0 as *const (), Throws::Caught), // sub_1413c29d0
    Replacement::draft_throws(0x1413c38d0, "r_1413c38d0", fn_1413c38d0 as *const (), Throws::Caught), // sub_1413c38d0
    Replacement::draft_throws(0x1413c3ae0, "r_1413c3ae0", fn_1413c3ae0 as *const (), Throws::Caught), // sub_1413c3ae0
    Replacement::draft_throws(0x1413c41a0, "r_1413c41a0", crate::shards::subsystems::fn_1413c41a0 as *const (), Throws::Caught), // sub_1413c41a0
    Replacement::draft_throws(0x1413c4360, "r_1413c4360", fn_1413c4360 as *const (), Throws::Caught), // sub_1413c4360
    Replacement::draft_throws(0x1413c4540, "r_1413c4540", crate::shards::subsystems::fn_1413c4540 as *const (), Throws::Caught), // sub_1413c4540
    Replacement::draft_throws(0x1413c4700, "r_1413c4700", fn_1413c4700 as *const (), Throws::Caught), // sub_1413c4700
    Replacement::draft_throws(0x1413c48e0, "r_1413c48e0", crate::shards::subsystems::fn_1413c48e0 as *const (), Throws::Caught), // sub_1413c48e0
    Replacement::draft_throws(0x1413c4aa0, "r_1413c4aa0", crate::shards::subsystems::fn_1413c4aa0 as *const (), Throws::Caught), // sub_1413c4aa0
    Replacement::draft_throws(0x1413c4c60, "r_1413c4c60", fn_1413c4c60 as *const (), Throws::Caught), // sub_1413c4c60
    Replacement::draft(0x1413c4e40, "r_1413c4e40", tpl_25523::<0x10, 0x142646558, 0x8, 0x141834d30> as *const ()), // sub_1413c4e40
    Replacement::draft_throws(0x1413c4e70, "r_1413c4e70", fn_1413c4e70 as *const (), Throws::Discard), // sub_1413c4e70
    Replacement::draft_throws(0x1413c4f10, "r_1413c4f10", fn_1413c4f10 as *const (), Throws::Discard), // sub_1413c4f10
    Replacement::draft(0x1413cb850, "r_1413cb850", fn_1413cb850 as *const ()), // sub_1413cb850
    Replacement::draft(0x1413cba30, "r_1413cba30", fn_1413cba30 as *const ()), // sub_1413cba30
    Replacement::draft(0x1413cbbb0, "r_1413cbbb0", fn_1413cbbb0 as *const ()), // sub_1413cbbb0
    Replacement::draft(0x1413cbc90, "r_1413cbc90", fn_1413cbc90 as *const ()), // sub_1413cbc90
    Replacement::draft(0x1413cbcb0, "r_1413cbcb0", fn_1413cbcb0 as *const ()), // sub_1413cbcb0
    Replacement::draft(0x1413cbcf0, "r_1413cbcf0", fn_1413cbcf0 as *const ()), // sub_1413cbcf0
    Replacement::draft(0x1413cbd10, "r_1413cbd10", fn_1413cbd10 as *const ()), // sub_1413cbd10
    Replacement::draft_throws(0x1413cbe60, "r_1413cbe60", fn_1413cbe60 as *const (), Throws::Caught), // sub_1413cbe60
    Replacement::draft(0x1413cbef0, "r_1413cbef0", fn_1413cbef0 as *const ()), // sub_1413cbef0
    Replacement::draft(0x1413cbfa0, "r_1413cbfa0", fn_1413cbfa0 as *const ()), // r::GlobalFileSystem<r::BasicWinFileSystem>::r::GlobalFileSystem<r::BasicWinFileSystem>::vfunc_0
    Replacement::draft(0x1413cf110, "r_1413cf110", fn_1413cf110 as *const ()), // r::BaseGlobalParameter::r::BaseGlobalParameter::vfunc_0
    Replacement::draft(0x1413d0070, "r_1413d0070", tpl_24940::<0x14340ed48, 0x8, 0x14340e8e8, 0x14340e848, 0x142498f00> as *const ()), // sub_1413d0070
    Replacement::draft(0x1413d0160, "r_1413d0160", tpl_24940::<0x14340ed48, 0x8, 0x14340e968, 0x14340e930, 0x142498f00> as *const ()), // sub_1413d0160
    Replacement::draft(0x1413d01a0, "r_1413d01a0", fn_1413d01a0 as *const ()), // sub_1413d01a0
    Replacement::draft(0x1413d0200, "r_1413d0200", fn_1413d0200 as *const ()), // sub_1413d0200
    Replacement::draft(0x1413d0220, "r_1413d0220", fn_1413d0220 as *const ()), // sub_1413d0220
    Replacement::draft_throws(0x1413d0290, "r_1413d0290", fn_1413d0290 as *const (), Throws::Caught), // sub_1413d0290
    Replacement::draft(0x1413d0300, "r_1413d0300", tpl_5167::<0x8> as *const ()), // sub_1413d0300
    Replacement::draft(0x1413d0310, "r_1413d0310", tpl_25626::<0x8, 0x20, 0x142442de0, 0x10, 0x14, 0x18> as *const ()), // sub_1413d0310
    Replacement::draft_throws(0x1413d0350, "r_1413d0350", fn_1413d0350 as *const (), Throws::Discard), // r::FolderWatcherMonitor::r::FolderWatcherMonitor::vfunc_0
    Replacement::draft_throws(0x1413d03b0, "r_1413d03b0", fn_1413d03b0 as *const (), Throws::Caught), // sub_1413d03b0
    Replacement::draft_throws(0x1413d05a0, "r_1413d05a0", fn_1413d05a0 as *const (), Throws::Discard), // r::FileSystemWatcher::r::FileSystemWatcher::vfunc_0
    Replacement::draft_throws(0x1413d0740, "r_1413d0740", fn_1413d0740 as *const (), Throws::Caught), // sub_1413d0740
    Replacement::draft(0x1413d1c70, "r_1413d1c70", fn_1413d1c70 as *const ()), // sub_1413d1c70
    Replacement::draft(0x1413d1d20, "r_1413d1d20", tpl_660::<0x8, 0x120> as *const ()), // r::FileArchive::File::r::FileArchive::File::vfunc_4
    Replacement::draft(0x1413d1d30, "r_1413d1d30", fn_1413d1d30 as *const ()), // r::FileArchive::r::FileArchive::vfunc_0
    Replacement::draft_throws(0x1413d1d80, "r_1413d1d80", fn_1413d1d80 as *const (), Throws::Caught), // sub_1413d1d80
    Replacement::draft(0x1413d1e70, "r_1413d1e70", fn_1413d1e70 as *const ()), // r::FileArchive::File::r::FileArchive::File::vfunc_0
    Replacement::draft(0x1413d1ed0, "r_1413d1ed0", fn_1413d1ed0 as *const ()), // sub_1413d1ed0
    Replacement::draft_throws(0x1413d1f40, "r_1413d1f40", fn_1413d1f40 as *const (), Throws::Caught), // sub_1413d1f40
    Replacement::draft(0x1413d1fb0, "r_1413d1fb0", fn_1413d1fb0 as *const ()), // sub_1413d1fb0
    Replacement::draft(0x1413d2080, "r_1413d2080", fn_1413d2080 as *const ()), // sub_1413d2080
    Replacement::draft(0x1413d2250, "r_1413d2250", fn_1413d2250 as *const ()), // sub_1413d2250
    Replacement::draft(0x1413d22b0, "r_1413d22b0", tpl_24940::<0x14340ed48, 0x8, 0x14340ed28, 0x14340ec98, 0x142498f00> as *const ()), // sub_1413d22b0
    Replacement::draft(0x1413d2310, "r_1413d2310", tpl_24940::<0x14340ed48, 0x8, 0x14340ecc0, 0x14340ec68, 0x142498f00> as *const ()), // sub_1413d2310
    Replacement::draft_throws(0x1413d2370, "r_1413d2370", tpl_25461::<0x142498af0, 0x141834d30> as *const (), Throws::Discard), // r::File::r::File::vfunc_0
    Replacement::draft(0x1413d2c50, "r_1413d2c50", fn_1413d2c50 as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_0
    Replacement::draft(0x1413d2d30, "r_1413d2d30", fn_1413d2d30 as *const ()), // sub_1413d2d30
    Replacement::draft(0x1413d2dd0, "r_1413d2dd0", fn_1413d2dd0 as *const ()), // sub_1413d2dd0
    Replacement::draft_throws(0x1413d2ed0, "r_1413d2ed0", fn_1413d2ed0 as *const (), Throws::Caught), // sub_1413d2ed0
    Replacement::draft(0x1413d2fc0, "r_1413d2fc0", fn_1413d2fc0 as *const ()), // sub_1413d2fc0
    Replacement::draft_throws(0x1413d3090, "r_1413d3090", fn_1413d3090 as *const (), Throws::Caught), // sub_1413d3090
    Replacement::draft(0x1413d3180, "r_1413d3180", fn_1413d3180 as *const ()), // sub_1413d3180
    Replacement::draft_throws(0x1413d3200, "r_1413d3200", fn_1413d3200 as *const (), Throws::Caught), // sub_1413d3200
    Replacement::draft_throws(0x1413d3300, "r_1413d3300", fn_1413d3300 as *const (), Throws::Caught), // sub_1413d3300
    Replacement::draft_throws(0x1413d3400, "r_1413d3400", fn_1413d3400 as *const (), Throws::Caught), // sub_1413d3400
    Replacement::draft(0x1413d3500, "r_1413d3500", fn_1413d3500 as *const ()), // sub_1413d3500
    Replacement::draft(0x1413d35e0, "r_1413d35e0", fn_1413d35e0 as *const ()), // sub_1413d35e0
    Replacement::draft(0x1413d36a0, "r_1413d36a0", fn_1413d36a0 as *const ()), // sub_1413d36a0
    Replacement::draft_throws(0x1413d3770, "r_1413d3770", fn_1413d3770 as *const (), Throws::Caught), // sub_1413d3770
    Replacement::draft_throws(0x1413d3820, "r_1413d3820", fn_1413d3820 as *const (), Throws::Caught), // sub_1413d3820
    Replacement::draft(0x1413d3930, "r_1413d3930", fn_1413d3930 as *const ()), // sub_1413d3930
    Replacement::draft(0x1413d39d0, "r_1413d39d0", fn_1413d39d0 as *const ()), // sub_1413d39d0
    Replacement::draft_throws(0x1413d3ad0, "r_1413d3ad0", fn_1413d3ad0 as *const (), Throws::Caught), // sub_1413d3ad0
    Replacement::draft(0x1413d3bc0, "r_1413d3bc0", fn_1413d3bc0 as *const ()), // sub_1413d3bc0
    Replacement::draft_throws(0x1413d3c90, "r_1413d3c90", fn_1413d3c90 as *const (), Throws::Caught), // sub_1413d3c90
    Replacement::draft(0x1413d3d80, "r_1413d3d80", fn_1413d3d80 as *const ()), // sub_1413d3d80
    Replacement::draft_throws(0x1413d3e00, "r_1413d3e00", fn_1413d3e00 as *const (), Throws::Caught), // sub_1413d3e00
    Replacement::draft_throws(0x1413d3f00, "r_1413d3f00", fn_1413d3f00 as *const (), Throws::Caught), // sub_1413d3f00
    Replacement::draft_throws(0x1413d4000, "r_1413d4000", fn_1413d4000 as *const (), Throws::Caught), // sub_1413d4000
    Replacement::draft(0x1413d4100, "r_1413d4100", fn_1413d4100 as *const ()), // sub_1413d4100
    Replacement::draft(0x1413d41e0, "r_1413d41e0", fn_1413d41e0 as *const ()), // sub_1413d41e0
    Replacement::draft(0x1413d42a0, "r_1413d42a0", fn_1413d42a0 as *const ()), // sub_1413d42a0
    Replacement::draft_throws(0x1413d4370, "r_1413d4370", fn_1413d4370 as *const (), Throws::Caught), // sub_1413d4370
    Replacement::draft_throws(0x1413d4420, "r_1413d4420", fn_1413d4420 as *const (), Throws::Caught), // sub_1413d4420
    Replacement::draft(0x1413d4530, "r_1413d4530", fn_1413d4530 as *const ()), // sub_1413d4530
    Replacement::draft(0x1413d4550, "r_1413d4550", fn_1413d4550 as *const ()), // sub_1413d4550
    Replacement::draft(0x1413d4600, "r_1413d4600", fn_1413d4600 as *const ()), // sub_1413d4600
    Replacement::draft(0x1413d4720, "r_1413d4720", fn_1413d4720 as *const ()), // sub_1413d4720
    Replacement::draft(0x1413d4870, "r_1413d4870", fn_1413d4870 as *const ()), // sub_1413d4870
    Replacement::draft(0x1413d49c0, "r_1413d49c0", fn_1413d49c0 as *const ()), // sub_1413d49c0
    Replacement::draft(0x1413d4a80, "r_1413d4a80", fn_1413d4a80 as *const ()), // sub_1413d4a80
    Replacement::draft(0x1413d4bd0, "r_1413d4bd0", fn_1413d4bd0 as *const ()), // sub_1413d4bd0
    Replacement::draft(0x1413d4ca0, "r_1413d4ca0", fn_1413d4ca0 as *const ()), // sub_1413d4ca0
    Replacement::draft_throws(0x1413d4e60, "r_1413d4e60", fn_1413d4e60 as *const (), Throws::Caught), // sub_1413d4e60
    Replacement::draft_throws(0x1413d4f70, "r_1413d4f70", fn_1413d4f70 as *const (), Throws::Caught), // sub_1413d4f70
    Replacement::draft_throws(0x1413d5060, "r_1413d5060", fn_1413d5060 as *const (), Throws::Caught), // sub_1413d5060
    Replacement::draft(0x1413d51a0, "r_1413d51a0", fn_1413d51a0 as *const ()), // sub_1413d51a0
    Replacement::draft_throws(0x1413d5250, "r_1413d5250", crate::shards::subsystems::fn_1413d5250 as *const (), Throws::Caught), // sub_1413d5250
    Replacement::draft_throws(0x1413d5320, "r_1413d5320", crate::shards::subsystems::fn_1413d5320 as *const (), Throws::Caught), // sub_1413d5320
    Replacement::draft_throws(0x1413d53f0, "r_1413d53f0", crate::shards::subsystems::fn_1413d53f0 as *const (), Throws::Caught), // sub_1413d53f0
    Replacement::draft(0x1413d54c0, "r_1413d54c0", fn_1413d54c0 as *const ()), // sub_1413d54c0
    Replacement::draft(0x1413d55f0, "r_1413d55f0", fn_1413d55f0 as *const ()), // sub_1413d55f0
    Replacement::draft(0x1413d5690, "r_1413d5690", fn_1413d5690 as *const ()), // sub_1413d5690
    Replacement::draft_throws(0x1413d5740, "r_1413d5740", fn_1413d5740 as *const (), Throws::Discard), // sub_1413d5740
    Replacement::draft_throws(0x1413d5860, "r_1413d5860", fn_1413d5860 as *const (), Throws::Discard), // sub_1413d5860
    Replacement::draft_throws(0x1413d5960, "r_1413d5960", fn_1413d5960 as *const (), Throws::Caught), // sub_1413d5960
    Replacement::draft_throws(0x1413d5ee0, "r_1413d5ee0", fn_1413d5ee0 as *const (), Throws::Caught), // sub_1413d5ee0
    Replacement::draft_throws(0x1413d63f0, "r_1413d63f0", fn_1413d63f0 as *const (), Throws::Caught), // sub_1413d63f0
    Replacement::draft_throws(0x1413d68e0, "r_1413d68e0", fn_1413d68e0 as *const (), Throws::Caught), // sub_1413d68e0
    Replacement::draft(0x1413d69f0, "r_1413d69f0", fn_1413d69f0 as *const ()), // sub_1413d69f0
    Replacement::draft(0x1413d6a90, "r_1413d6a90", fn_1413d6a90 as *const ()), // sub_1413d6a90
    Replacement::draft(0x1413d6b40, "r_1413d6b40", fn_1413d6b40 as *const ()), // r::ConsoleLogWriter::r::ConsoleLogWriter::vfunc_0
    Replacement::draft(0x1413d6f90, "r_1413d6f90", fn_1413d6f90 as *const ()), // r::CommandLine::r::CommandLine::vfunc_0
    Replacement::draft(0x1413d7000, "r_1413d7000", fn_1413d7000 as *const ()), // sub_1413d7000
    Replacement::draft(0x1413d7010, "r_1413d7010", fn_1413d7010 as *const ()), // sub_1413d7010
    Replacement::draft(0x1413d70f0, "r_1413d70f0", fn_1413d70f0 as *const ()), // sub_1413d70f0
    Replacement::draft(0x1413d7410, "r_1413d7410", fn_1413d7410 as *const ()), // sub_1413d7410
    Replacement::draft(0x1413d75e0, "r_1413d75e0", fn_1413d75e0 as *const ()), // sub_1413d75e0
    Replacement::draft(0x1413d7a90, "r_1413d7a90", fn_1413d7a90 as *const ()), // sub_1413d7a90
    Replacement::draft(0x1413d7be0, "r_1413d7be0", tpl_20448::<0x1402174f0, 0x1411f93c0> as *const ()), // sub_1413d7be0
    Replacement::draft_throws(0x1413d7c80, "r_1413d7c80", fn_1413d7c80 as *const (), Throws::Caught), // r::CmdLineParameterBase::r::CmdLineParameterBase::vfunc_0
    Replacement::draft(0x1413d7d30, "r_1413d7d30", tpl_25736::<0x3, 0x4, 0x8, 0x50, 0x14240aa00> as *const ()), // r::CmdLineParameterBase::r::CmdLineParameterBase::vfunc_1
    Replacement::draft(0x1413d7d70, "r_1413d7d70", fn_1413d7d70 as *const ()), // r::CmdLineParameterBase::r::CmdLineParameterBase::vfunc_3
    Replacement::draft(0x1413d7dd0, "r_1413d7dd0", fn_1413d7dd0 as *const ()), // r::CmdLineParser::r::CmdLineParser::vfunc_0
    Replacement::draft(0x1413d7e30, "r_1413d7e30", fn_1413d7e30 as *const ()), // r::CmdLineParameterBase::r::CmdLineParameterBase::vfunc_2
    Replacement::draft(0x1413d7e70, "r_1413d7e70", fn_1413d7e70 as *const ()), // r::CmdLineParameterString::r::CmdLineParameterString::vfunc_2
    Replacement::draft(0x1413d7eb0, "r_1413d7eb0", tpl_21689::<0x141122270, 0x141834d30> as *const ()), // r::CmdLineParameterOption::r::CmdLineParameterOption::vfunc_2
    Replacement::draft(0x1413d7ef0, "r_1413d7ef0", fn_1413d7ef0 as *const ()), // r::CmdLineParameterBoolean::r::CmdLineParameterBoolean::vfunc_2
    Replacement::draft(0x1413d7f60, "r_1413d7f60", fn_1413d7f60 as *const ()), // r::CmdLineParameterInteger::r::CmdLineParameterInteger::vfunc_2
    Replacement::draft(0x1413d7fe0, "r_1413d7fe0", fn_1413d7fe0 as *const ()), // r::CmdLineParameterMultiString::r::CmdLineParameterMultiString::vfunc_2
    Replacement::draft(0x1413d9250, "r_1413d9250", fn_1413d9250 as *const ()), // r::CachedFile::r::CachedFile::vfunc_0
    Replacement::draft_throws(0x1413d9350, "r_1413d9350", fn_1413d9350 as *const (), Throws::Caught), // sub_1413d9350
    Replacement::draft_throws(0x1413d93b0, "r_1413d93b0", tpl_18554::<0x1433fa948, 0x1402c4e80, 0xc, 0x143250720, 0x14020f270> as *const (), Throws::Discard), // sub_1413d93b0
    Replacement::draft(0x1413d9420, "r_1413d9420", fn_1413d9420 as *const ()), // sub_1413d9420
    Replacement::draft(0x1413d94c0, "r_1413d94c0", fn_1413d94c0 as *const ()), // sub_1413d94c0
    Replacement::draft_throws(0x1413d9600, "r_1413d9600", fn_1413d9600 as *const (), Throws::Discard), // sub_1413d9600
    Replacement::draft_throws(0x1413d9670, "r_1413d9670", fn_1413d9670 as *const (), Throws::Discard), // r::BackgroundJobManager::r::BackgroundJobManager::vfunc_0
    Replacement::draft(0x141853190, "r_141853190", tpl_34372::<0x14307d580, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_141853190
    Replacement::draft(0x141853280, "r_141853280", tpl_34373::<0x14307d580, 0x8, 0x1401f3d90, 0x18, 0x20, 0x14307d598> as *const ()), // sub_141853280
    Replacement::draft(0x141853360, "r_141853360", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_141853360
    Replacement::draft(0x141853380, "r_141853380", fn_141853380 as *const ()), // r::internal::VoidContinuationImpl<ProgramFlowServerStateSplashScreen::update::<lambda_1>,r::Expected<unsigned __int64,pl
    Replacement::draft_throws(0x1418597b0, "r_1418597b0", fn_1418597b0 as *const (), Throws::Discard), // r::LambdaStreamJob<WorldLoadingLogic::onEnterLoadingEpisode::<lambda_1>>::r::LambdaStreamJob<WorldLoadingLogic::onEnterL
    Replacement::draft(0x1418b0c40, "r_1418b0c40", fn_1418b0c40 as *const ()), // r::JobCallable<bigfish::UIHelper::openDeluxeStore::<lambda_1>>::r::JobCallable<bigfish::UIHelper::openDeluxeStore::<lamb
    Replacement::draft(0x1418d40d0, "r_1418d40d0", tpl_35104::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::save_game::writeWithErrorHandling::?bref1::operator()<platform::storage::WriteRes
    Replacement::draft(0x1418d4130, "r_1418d4130", tpl_34312::<0x14028b790> as *const ()), // sub_1418d4130
    Replacement::draft(0x1418d4160, "r_1418d4160", tpl_35105::<0x1402bf7c0, 0x40, 0x14240ab10, 0x18, 0x1430b57b8, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::save_game::writeWithErrorHandling::<lambda_1>,?bref8::storage::WriteResult>::r::i
    Replacement::draft(0x1418d41c0, "r_1418d41c0", tpl_35103::<0x14028b790, 0x78> as *const ()), // sub_1418d41c0
    Replacement::draft(0x1418d41f0, "r_1418d41f0", tpl_35104::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::save_game::copyCheckpointWithErrorHandling::?bref1::operator()<r::Expected<void,p
    Replacement::draft(0x1418d42a0, "r_1418d42a0", tpl_35103::<0x14028b790, 0x78> as *const ()), // sub_1418d42a0
    Replacement::draft(0x1418d42d0, "r_1418d42d0", tpl_35104::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::save_game::deleteCheckpointWithErrorHandling::?bref1::operator()<r::Expected<void
    Replacement::draft(0x1418d4380, "r_1418d4380", fn_1418d4380 as *const ()), // sub_1418d4380
    Replacement::draft(0x1418d43e0, "r_1418d43e0", tpl_35107::<0x1402d7090, 0x80, 0x14240ab10, 0x38, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::save_game::handleReadError::<lambda_1>,?bref6::Expected<platform::storage::Handle
    Replacement::draft(0x1418d4450, "r_1418d4450", tpl_35108::<0x1430b6ba8, 0x14028b790, 0x78, 0x80, 0x1402bf830, 0x88, 0x90> as *const ()), // sub_1418d4450
    Replacement::draft(0x1418d44c0, "r_1418d44c0", fn_1418d44c0 as *const ()), // sub_1418d44c0
    Replacement::draft(0x1418d4510, "r_1418d4510", tpl_35108::<0x1430b86b0, 0x14028b790, 0x78, 0x80, 0x1402bf830, 0x88, 0x90> as *const ()), // sub_1418d4510
    Replacement::draft(0x1418d4580, "r_1418d4580", tpl_34372::<0x1430b4bf0, 0x8, 0x1402bf830, 0x18, 0x20> as *const ()), // sub_1418d4580
    Replacement::draft(0x1418d45c0, "r_1418d45c0", tpl_35108::<0x1430b5990, 0x14028b790, 0x78, 0x80, 0x1402bf830, 0x88, 0x90> as *const ()), // sub_1418d45c0
    Replacement::draft(0x1418d4630, "r_1418d4630", tpl_34372::<0x1430b50f0, 0x8, 0x1402bf830, 0x18, 0x20> as *const ()), // sub_1418d4630
    Replacement::draft(0x1418d4670, "r_1418d4670", fn_1418d4670 as *const ()), // sub_1418d4670
    Replacement::draft(0x1418d46f0, "r_1418d46f0", tpl_35111::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0> as *const ()), // sub_1418d46f0
    Replacement::draft(0x1418d4730, "r_1418d4730", tpl_35111::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0> as *const ()), // sub_1418d4730
    Replacement::draft(0x1418d4770, "r_1418d4770", tpl_35104::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<bigfish::save_game::writeWithErrorHandling::?bref1::operator()<platform::storage::WriteRes
    Replacement::draft(0x1418d47d0, "r_1418d47d0", tpl_35112::<0x1402bf7c0, 0x40, 0x14240ab10, 0x18, 0x1430b57b8> as *const ()), // sub_1418d47d0
    Replacement::draft(0x1418d4810, "r_1418d4810", tpl_35112::<0x1402bf7c0, 0x40, 0x14240ab10, 0x18, 0x1430b57b8> as *const ()), // sub_1418d4810
    Replacement::draft(0x1418d4850, "r_1418d4850", tpl_35105::<0x1402bf7c0, 0x40, 0x14240ab10, 0x18, 0x1430b57b8, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<bigfish::save_game::writeWithErrorHandling::<lambda_1>,?bref8::storage::WriteResult>::r::i
    Replacement::draft(0x1418d48b0, "r_1418d48b0", tpl_35111::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0> as *const ()), // sub_1418d48b0
    Replacement::draft(0x1418d48f0, "r_1418d48f0", tpl_35111::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0> as *const ()), // sub_1418d48f0
    Replacement::draft(0x1418d4930, "r_1418d4930", tpl_35104::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<bigfish::save_game::copyCheckpointWithErrorHandling::?bref1::operator()<r::Expected<void,p
    Replacement::draft(0x1418d4a40, "r_1418d4a40", tpl_35111::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0> as *const ()), // sub_1418d4a40
    Replacement::draft(0x1418d4a80, "r_1418d4a80", tpl_35111::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0> as *const ()), // sub_1418d4a80
    Replacement::draft(0x1418d4ac0, "r_1418d4ac0", tpl_35104::<0x1402bf7c0, 0x38, 0x14240ab10, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<bigfish::save_game::deleteCheckpointWithErrorHandling::?bref1::operator()<r::Expected<void
    Replacement::draft(0x1418d4bd0, "r_1418d4bd0", tpl_35113::<0x1402d7090, 0x80, 0x14240ab10, 0x38, 0x10, 0x1430b57a0> as *const ()), // sub_1418d4bd0
    Replacement::draft(0x1418d4c20, "r_1418d4c20", tpl_35113::<0x1402d7090, 0x80, 0x14240ab10, 0x38, 0x10, 0x1430b57a0> as *const ()), // sub_1418d4c20
    Replacement::draft(0x1418d4c70, "r_1418d4c70", tpl_35107::<0x1402d7090, 0x80, 0x14240ab10, 0x38, 0x10, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<bigfish::save_game::handleReadError::<lambda_1>,?bref6::Expected<platform::storage::Handle
    Replacement::draft(0x1418d4ce0, "r_1418d4ce0", tpl_35114::<0x1430b6ba8, 0x14028b790, 0x78, 0x80, 0x1402bf830, 0x88, 0x90, 0x1430b4bd8> as *const ()), // sub_1418d4ce0
    Replacement::draft(0x1418d4d50, "r_1418d4d50", fn_1418d4d50 as *const ()), // sub_1418d4d50
    Replacement::draft(0x1418d4db0, "r_1418d4db0", tpl_35114::<0x1430b86b0, 0x14028b790, 0x78, 0x80, 0x1402bf830, 0x88, 0x90, 0x1430b8060> as *const ()), // sub_1418d4db0
    Replacement::draft(0x1418d4e20, "r_1418d4e20", tpl_34373::<0x1430b4bf0, 0x8, 0x1402bf830, 0x18, 0x20, 0x1430b5b88> as *const ()), // sub_1418d4e20
    Replacement::draft(0x1418d4e70, "r_1418d4e70", tpl_35114::<0x1430b5990, 0x14028b790, 0x78, 0x80, 0x1402bf830, 0x88, 0x90, 0x1430b7cc8> as *const ()), // sub_1418d4e70
    Replacement::draft(0x1418d4ee0, "r_1418d4ee0", tpl_34373::<0x1430b50f0, 0x8, 0x1402bf830, 0x18, 0x20, 0x1430b67e8> as *const ()), // sub_1418d4ee0
    Replacement::draft(0x1418d4f30, "r_1418d4f30", fn_1418d4f30 as *const ()), // sub_1418d4f30
    Replacement::draft(0x1418d5f90, "r_1418d5f90", tpl_35117::<0x88, 0x90, 0x8, 0xc> as *const ()), // sub_1418d5f90
    Replacement::draft_throws(0x1418d5fc0, "r_1418d5fc0", tpl_35118::<0x4, 0x80, 0x8, 0x1418d9080, 0x88, 0x1402d4af0> as *const (), Throws::Discard), // r::internal::ContinuationImpl<bigfish::save_game::writeWithErrorHandling::?bref1::operator()<platform::storage::WriteRes
    Replacement::draft(0x1418d6090, "r_1418d6090", tpl_35117::<0x90, 0x98, 0x8, 0xc> as *const ()), // sub_1418d6090
    Replacement::draft_throws(0x1418d60c0, "r_1418d60c0", fn_1418d60c0 as *const (), Throws::Discard), // r::internal::ContinuationImpl<bigfish::save_game::writeWithErrorHandling::<lambda_1>,?bref8::storage::WriteResult>::r::i
    Replacement::draft(0x1418d61d0, "r_1418d61d0", tpl_35117::<0x88, 0x90, 0x8, 0xc> as *const ()), // sub_1418d61d0
    Replacement::draft_throws(0x1418d6200, "r_1418d6200", tpl_35118::<0x4, 0x80, 0x8, 0x1418d9080, 0x88, 0x1402d4af0> as *const (), Throws::Discard), // r::internal::ContinuationImpl<bigfish::save_game::copyCheckpointWithErrorHandling::?bref1::operator()<r::Expected<void,p
    Replacement::draft(0x1418d62d0, "r_1418d62d0", tpl_35117::<0x18, 0x20, 0x8, 0xc> as *const ()), // sub_1418d62d0
    Replacement::draft_throws(0x1418d62f0, "r_1418d62f0", crate::shards::subsystems::fn_1418d62f0 as *const (), Throws::Caught), // r::internal::ContinuationImpl<bigfish::save_game::copyCheckpointWithErrorHandling::<lambda_1>,?bref6::Expected<void,plat
    Replacement::draft(0x1418d6400, "r_1418d6400", tpl_35117::<0x88, 0x90, 0x8, 0xc> as *const ()), // sub_1418d6400
    Replacement::draft_throws(0x1418d6430, "r_1418d6430", tpl_35118::<0x4, 0x80, 0x8, 0x1418d9080, 0x88, 0x1402d4af0> as *const (), Throws::Discard), // r::internal::ContinuationImpl<bigfish::save_game::deleteCheckpointWithErrorHandling::?bref1::operator()<r::Expected<void
    Replacement::draft(0x1418d6500, "r_1418d6500", tpl_35117::<0x18, 0x20, 0x8, 0xc> as *const ()), // sub_1418d6500
    Replacement::draft_throws(0x1418d6520, "r_1418d6520", crate::shards::subsystems::fn_1418d6520 as *const (), Throws::Caught), // r::internal::ContinuationImpl<bigfish::save_game::deleteCheckpointWithErrorHandling::<lambda_1>,?bref6::Expected<void,pl
    Replacement::draft(0x1418d6630, "r_1418d6630", fn_1418d6630 as *const ()), // sub_1418d6630
    Replacement::draft_throws(0x1418d6660, "r_1418d6660", fn_1418d6660 as *const (), Throws::Discard), // r::internal::ContinuationImpl<bigfish::save_game::handleReadError::<lambda_1>,?bref6::Expected<platform::storage::Handle
    Replacement::draft(0x1418f2870, "r_1418f2870", tpl_35268::<0xc, 0x8> as *const ()), // sub_1418f2870
    Replacement::draft(0x1418f2940, "r_1418f2940", tpl_35267::<0x4, 0xc, 0x8> as *const ()), // sub_1418f2940
    Replacement::draft(0x1418f29d0, "r_1418f29d0", tpl_35268::<0xc, 0x8> as *const ()), // sub_1418f29d0
    Replacement::draft(0x1418f2aa0, "r_1418f2aa0", tpl_35269::<0x1430baf60, 0x4, 0xc, 0x8, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1418f2aa0
    Replacement::draft(0x1418f2b10, "r_1418f2b10", tpl_35270::<0x1430baee8, 0x8, 0xc, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1418f2b10
    Replacement::draft(0x1418f2b70, "r_1418f2b70", tpl_34372::<0x1430b9ab0, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_1418f2b70
    Replacement::draft(0x1418f2bb0, "r_1418f2bb0", tpl_35269::<0x1430baf00, 0x4, 0xc, 0x8, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1418f2bb0
    Replacement::draft(0x1418f2c20, "r_1418f2c20", tpl_35270::<0x1430baf18, 0x8, 0xc, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1418f2c20
    Replacement::draft(0x1418f2c80, "r_1418f2c80", tpl_34372::<0x1430baf78, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_1418f2c80
    Replacement::draft(0x1418f30e0, "r_1418f30e0", tpl_35271::<0x1430baf60, 0x4, 0xc, 0x8, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28, 0x1430baf30> as *const ()), // sub_1418f30e0
    Replacement::draft(0x1418f3160, "r_1418f3160", tpl_35272::<0x1430baee8, 0x8, 0xc, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28, 0x1430baeb8> as *const ()), // sub_1418f3160
    Replacement::draft(0x1418f31c0, "r_1418f31c0", tpl_34373::<0x1430b9ab0, 0x8, 0x1401f3d90, 0x18, 0x20, 0x1430baed0> as *const ()), // sub_1418f31c0
    Replacement::draft(0x1418f3210, "r_1418f3210", tpl_35271::<0x1430baf00, 0x4, 0xc, 0x8, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28, 0x1430b9ac8> as *const ()), // sub_1418f3210
    Replacement::draft(0x1418f3290, "r_1418f3290", tpl_35272::<0x1430baf18, 0x8, 0xc, 0x14, 0x10, 0x1401f3d90, 0x20, 0x28, 0x1430baf48> as *const ()), // sub_1418f3290
    Replacement::draft(0x1418f32f0, "r_1418f32f0", tpl_34373::<0x1430baf78, 0x8, 0x1401f3d90, 0x18, 0x20, 0x1430baea0> as *const ()), // sub_1418f32f0
    Replacement::draft(0x1418f3420, "r_1418f3420", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1418f3420
    Replacement::draft(0x1418f34f0, "r_1418f34f0", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1418f34f0
    Replacement::draft(0x1418f35c0, "r_1418f35c0", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_1418f35c0
    Replacement::draft(0x1418f3690, "r_1418f3690", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1418f3690
    Replacement::draft(0x1418f3760, "r_1418f3760", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1418f3760
    Replacement::draft(0x1418f3830, "r_1418f3830", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_1418f3830
    Replacement::draft(0x1419177d0, "r_1419177d0", fn_1419177d0 as *const ()), // sub_1419177d0
    Replacement::draft(0x141917850, "r_141917850", tpl_35425::<0x1401f3d00, 0x88, 0x14240ab10, 0x40, 0x18, 0x1430dd098, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::PlayerPreparation::update::<lambda_1>,r::Expected<platform::storage::ChunkBuffer,
    Replacement::draft(0x1419178c0, "r_1419178c0", tpl_35426::<0x1430dd068, 0x8, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1419178c0
    Replacement::draft(0x141917900, "r_141917900", fn_141917900 as *const ()), // sub_141917900
    Replacement::draft(0x141917a40, "r_141917a40", tpl_35428::<0x1401f3d00, 0x88, 0x14240ab10, 0x40, 0x18, 0x1430dd098> as *const ()), // sub_141917a40
    Replacement::draft(0x141917a90, "r_141917a90", tpl_35428::<0x1401f3d00, 0x88, 0x14240ab10, 0x40, 0x18, 0x1430dd098> as *const ()), // sub_141917a90
    Replacement::draft(0x141917ae0, "r_141917ae0", tpl_35425::<0x1401f3d00, 0x88, 0x14240ab10, 0x40, 0x18, 0x1430dd098, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<bigfish::PlayerPreparation::update::<lambda_1>,r::Expected<platform::storage::ChunkBuf
    Replacement::draft(0x141917b50, "r_141917b50", tpl_35429::<0x1430dd068, 0x8, 0x1401f3d90, 0x20, 0x28, 0x1430dd0c8> as *const ()), // sub_141917b50
    Replacement::draft(0x141917ba0, "r_141917ba0", fn_141917ba0 as *const ()), // sub_141917ba0
    Replacement::draft(0x141917ce0, "r_141917ce0", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_141917ce0
    Replacement::draft(0x141917d00, "r_141917d00", fn_141917d00 as *const ()), // r::internal::VoidContinuationImpl<anonymous_namespace::onReadPreferencesCompleted::<lambda_1>,?bref5::Expected<platform:
    Replacement::draft(0x141917dd0, "r_141917dd0", tpl_34374::<0xb8, 0xc0, 0x8> as *const ()), // sub_141917dd0
    Replacement::draft(0x141a074a0, "r_141a074a0", tpl_36529::<0x1401f3d00, 0x14034c330, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::GameProperties::updateEdition::<lambda_1>,r::Expected<bool,platform::Error>>::r::
    Replacement::draft(0x141a07500, "r_141a07500", tpl_36529::<0x1401f3d00, 0x14034c330, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::GameProperties::updateEdition::<lambda_2>,r::Expected<bool,platform::Error>>::r::
    Replacement::draft(0x141a07560, "r_141a07560", tpl_36529::<0x1401f3d00, 0x14034c330, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::GameProperties::updateEdition::<lambda_3>,r::Expected<bool,platform::Error>>::r::
    Replacement::draft(0x141a075c0, "r_141a075c0", fn_141a075c0 as *const ()), // sub_141a075c0
    Replacement::draft(0x141a07640, "r_141a07640", tpl_36531::<0x141a07ce0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::GameProperties::forceSave::<lambda_1>,platform::storage::WriteResult>::r::interna
    Replacement::draft(0x141a07690, "r_141a07690", tpl_36532::<0x1401f3d00, 0x14028dfd0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::GameProperties::forceSave::<lambda_2>::operator()::<lambda_1>,platform::storage::
    Replacement::draft(0x141a076f0, "r_141a076f0", fn_141a076f0 as *const ()), // sub_141a076f0
    Replacement::draft(0x141a07780, "r_141a07780", tpl_36534::<0x1401f3d00, 0x14028c540, 0x14028dfe0, 0x141834d30> as *const ()), // r::internal::ContinuationData<bigfish::GameProperties::forceSave::<lambda_2>,r::Expected<platform::storage::HandleErrorS
    Replacement::draft(0x141a077f0, "r_141a077f0", tpl_36535::<0x14034e370, 0x14314f9f8, 0x8, 0x1401f3d70> as *const ()), // sub_141a077f0
    Replacement::draft(0x141a07850, "r_141a07850", tpl_36535::<0x14034e370, 0x14314fb30, 0x8, 0x1401f3d70> as *const ()), // sub_141a07850
    Replacement::draft(0x141a078b0, "r_141a078b0", tpl_36535::<0x14034e370, 0x14314f998, 0x8, 0x1401f3d70> as *const ()), // sub_141a078b0
    Replacement::draft(0x141a07910, "r_141a07910", fn_141a07910 as *const ()), // sub_141a07910
    Replacement::draft(0x141a07980, "r_141a07980", fn_141a07980 as *const ()), // sub_141a07980
    Replacement::draft(0x141a079d0, "r_141a079d0", fn_141a079d0 as *const ()), // sub_141a079d0
    Replacement::draft(0x141a07a40, "r_141a07a40", fn_141a07a40 as *const ()), // sub_141a07a40
    Replacement::draft(0x141a07a80, "r_141a07a80", tpl_4553 as *const ()), // sub_141a07a80
    Replacement::draft(0x141a07aa0, "r_141a07aa0", tpl_36540::<0x1401f3d00, 0x14034c330> as *const ()), // sub_141a07aa0
    Replacement::draft(0x141a07ad0, "r_141a07ad0", tpl_36540::<0x1401f3d00, 0x14034c330> as *const ()), // sub_141a07ad0
    Replacement::draft(0x141a07b00, "r_141a07b00", tpl_36529::<0x1401f3d00, 0x14034c330, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::updateEdition::<lambda_1>,r::Expected<bool,platform::Error>>:
    Replacement::draft(0x141a07b60, "r_141a07b60", tpl_36540::<0x1401f3d00, 0x14034c330> as *const ()), // sub_141a07b60
    Replacement::draft(0x141a07b90, "r_141a07b90", tpl_36540::<0x1401f3d00, 0x14034c330> as *const ()), // sub_141a07b90
    Replacement::draft(0x141a07bc0, "r_141a07bc0", tpl_36529::<0x1401f3d00, 0x14034c330, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::updateEdition::<lambda_2>,r::Expected<bool,platform::Error>>:
    Replacement::draft(0x141a07c20, "r_141a07c20", tpl_36540::<0x1401f3d00, 0x14034c330> as *const ()), // sub_141a07c20
    Replacement::draft(0x141a07c50, "r_141a07c50", tpl_36540::<0x1401f3d00, 0x14034c330> as *const ()), // sub_141a07c50
    Replacement::draft(0x141a07c80, "r_141a07c80", tpl_36529::<0x1401f3d00, 0x14034c330, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::updateEdition::<lambda_3>,r::Expected<bool,platform::Error>>:
    Replacement::draft(0x141a07ce0, "r_141a07ce0", fn_141a07ce0 as *const ()), // sub_141a07ce0
    Replacement::draft(0x141a07d50, "r_141a07d50", tpl_4547::<0x141a07ce0> as *const ()), // sub_141a07d50
    Replacement::draft(0x141a07d70, "r_141a07d70", tpl_36531::<0x141a07ce0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<bigfish::GameProperties::forceSave::<lambda_1>,platform::storage::WriteResult>::r::interna
    Replacement::draft(0x141a07dc0, "r_141a07dc0", tpl_36542::<0x1401f3d00, 0x14028dfd0> as *const ()), // sub_141a07dc0
    Replacement::draft(0x141a07df0, "r_141a07df0", tpl_36542::<0x1401f3d00, 0x14028dfd0> as *const ()), // sub_141a07df0
    Replacement::draft(0x141a07e20, "r_141a07e20", tpl_36532::<0x1401f3d00, 0x14028dfd0, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::forceSave::<lambda_2>::operator()::<lambda_1>,platform::stora
    Replacement::draft(0x141a07e80, "r_141a07e80", tpl_36543::<0x1401f3d00, 0x14028c540, 0x14028dfe0> as *const ()), // sub_141a07e80
    Replacement::draft(0x141a07ed0, "r_141a07ed0", tpl_36543::<0x1401f3d00, 0x14028c540, 0x14028dfe0> as *const ()), // sub_141a07ed0
    Replacement::draft(0x141a07f20, "r_141a07f20", tpl_36534::<0x1401f3d00, 0x14028c540, 0x14028dfe0, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::forceSave::<lambda_2>,r::Expected<platform::storage::HandleEr
    Replacement::draft(0x141a087f0, "r_141a087f0", fn_141a087f0 as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::updateEdition::<lambda_1>,r::Expected<bool,platform::Error>>:
    Replacement::draft(0x141a08870, "r_141a08870", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_141a08870
    Replacement::draft(0x141a08990, "r_141a08990", tpl_36555::<0x1401f3ce0> as *const ()), // sub_141a08990
    Replacement::draft(0x141a089f0, "r_141a089f0", fn_141a089f0 as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::updateEdition::<lambda_2>,r::Expected<bool,platform::Error>>:
    Replacement::draft(0x141a08a80, "r_141a08a80", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_141a08a80
    Replacement::draft(0x141a08ba0, "r_141a08ba0", tpl_36555::<0x1401f3ce0> as *const ()), // sub_141a08ba0
    Replacement::draft(0x141a08c00, "r_141a08c00", fn_141a08c00 as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::updateEdition::<lambda_3>,r::Expected<bool,platform::Error>>:
    Replacement::draft(0x141a08c70, "r_141a08c70", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_141a08c70
    Replacement::draft(0x141a08d90, "r_141a08d90", fn_141a08d90 as *const ()), // sub_141a08d90
    Replacement::draft(0x141a08e30, "r_141a08e30", fn_141a08e30 as *const ()), // r::internal::ContinuationImpl<bigfish::GameProperties::forceSave::<lambda_1>,platform::storage::WriteResult>::r::interna
    Replacement::draft(0x141a08ed0, "r_141a08ed0", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_141a08ed0
    Replacement::draft(0x141a08ff0, "r_141a08ff0", fn_141a08ff0 as *const ()), // sub_141a08ff0
    Replacement::draft(0x141a09050, "r_141a09050", fn_141a09050 as *const ()), // r::internal::VoidContinuationImpl<bigfish::GameProperties::forceSave::<lambda_2>::operator()::<lambda_1>,platform::stora
    Replacement::draft(0x141a090b0, "r_141a090b0", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_141a090b0
    Replacement::draft(0x141a091d0, "r_141a091d0", fn_141a091d0 as *const ()), // sub_141a091d0
    Replacement::draft_throws(0x141a09230, "r_141a09230", fn_141a09230 as *const (), Throws::Discard), // r::internal::VoidContinuationImpl<bigfish::GameProperties::forceSave::<lambda_2>,r::Expected<platform::storage::HandleEr
    Replacement::draft(0x141b528b0, "r_141b528b0", tpl_37788::<0x20, 0x143212720, 0x8, 0x28> as *const ()), // sub_141b528b0
    Replacement::draft(0x141b528e0, "r_141b528e0", tpl_37789::<0x40, 0x38, 0x20, 0x8, 0x30, 0x18, 0x14062ee90> as *const ()), // r::JobCallable<r::parallelfordetail::parallelForEachJob<const m::SIMDVector,const bonsai::sphericalpoints::getScaledPoin
    Replacement::draft(0x141b52970, "r_141b52970", tpl_37790::<0x20, 0x8, 0x30, 0x143212738, 0x28, 0x38> as *const ()), // sub_141b52970
    Replacement::draft(0x141b529b0, "r_141b529b0", tpl_37791::<0x20, 0x8, 0x18, 0x38, 0x30, 0x14062ee90> as *const ()), // r::JobCallable<r::parallelForEach<const r::StaticArray<m::SIMDVector,50>&,bonsai::sphericalpoints::getScaledPoints::<lam
    Replacement::draft(0x141b84400, "r_141b84400", tpl_38069::<0x20, 0x8, 0x40, 0x1432283d8, 0x28, 0x48> as *const ()), // sub_141b84400
    Replacement::draft(0x141b84440, "r_141b84440", tpl_37789::<0x48, 0x40, 0x20, 0x8, 0x30, 0x18, 0x140666d20> as *const ()), // r::JobCallable<r::parallelfordetail::parallelForEachJob<bonsai::EQSTaskInstance*,const ?bref2::EQSNodeInstance::executeT
    Replacement::draft(0x141b844d0, "r_141b844d0", tpl_37788::<0x20, 0x1432283c0, 0x8, 0x28> as *const ()), // sub_141b844d0
    Replacement::draft(0x141b84500, "r_141b84500", tpl_37791::<0x20, 0x8, 0x18, 0x40, 0x30, 0x140666d20> as *const ()), // r::JobCallable<r::parallelForEach<r::FixedArray<bonsai::EQSTaskInstance*>&,bonsai::EQSNodeInstance::executeTasks::<lambd
    Replacement::draft(0x141e4de10, "r_141e4de10", tpl_37790::<0x20, 0x8, 0x30, 0x14332c7f8, 0x28, 0x38> as *const ()), // sub_141e4de10
    Replacement::draft_throws(0x141e4de50, "r_141e4de50", fn_141e4de50 as *const (), Throws::Discard), // r::JobCallable<coregame::streamingVolumeChunkingSystem::<lambda_1>>::r::JobCallable<coregame::streamingVolumeChunkingSys
    Replacement::draft(0x141e554f0, "r_141e554f0", tpl_40773::<0x20, 0x143333088, 0x8, 0x28> as *const ()), // sub_141e554f0
    Replacement::draft(0x141e55520, "r_141e55520", fn_141e55520 as *const ()), // r::JobCallable<coregame::streaming::updateBroadphaseVolumeInclusions::<lambda_2>>::r::JobCallable<coregame::streaming::u
    Replacement::draft_throws(0x141e85470, "r_141e85470", fn_141e85470 as *const (), Throws::Caught), // r::JobCallable<coregame::QuestLoader::gameLoad::<lambda_1>>::r::JobCallable<coregame::QuestLoader::gameLoad::<lambda_1>>
    Replacement::draft(0x141ea05d0, "r_141ea05d0", tpl_41177::<0x20, 0x8, 0x40, 0x143356a00, 0x28, 0x48> as *const ()), // sub_141ea05d0
    Replacement::draft_throws(0x141ea0610, "r_141ea0610", fn_141ea0610 as *const (), Throws::Discard), // r::JobCallable<coregame::particle::updateAllEntities::<lambda_2>>::r::JobCallable<coregame::particle::updateAllEntities:
    Replacement::draft(0x141ea0b20, "r_141ea0b20", fn_141ea0b20 as *const ()), // sub_141ea0b20
    Replacement::draft_throws(0x141ea0e70, "r_141ea0e70", fn_141ea0e70 as *const (), Throws::Discard), // sub_141ea0e70
    Replacement::draft_throws(0x141ea0f70, "r_141ea0f70", fn_141ea0f70 as *const (), Throws::Discard), // sub_141ea0f70
    Replacement::draft_throws(0x141ea10f0, "r_141ea10f0", fn_141ea10f0 as *const (), Throws::Discard), // sub_141ea10f0
    Replacement::draft_throws(0x141ea1740, "r_141ea1740", fn_141ea1740 as *const (), Throws::Caught), // sub_141ea1740
    Replacement::draft(0x141ea2040, "r_141ea2040", tpl_41088::<0x142407c80, 0x143cd19fc, 0x14020e8f0, 0x143cd1a04> as *const ()), // sub_141ea2040
    Replacement::draft_throws(0x141ea2240, "r_141ea2240", fn_141ea2240 as *const (), Throws::Caught), // sub_141ea2240
    Replacement::draft(0x141ea2520, "r_141ea2520", tpl_20052::<0x143427080, 0x10, 0x8, 0x141027ef0, 0x1410280f0> as *const ()), // sub_141ea2520
    Replacement::draft(0x141ea25c0, "r_141ea25c0", fn_141ea25c0 as *const ()), // sub_141ea25c0
    Replacement::draft(0x141ea26b0, "r_141ea26b0", fn_141ea26b0 as *const ()), // sub_141ea26b0
    Replacement::draft_throws(0x141ea2790, "r_141ea2790", fn_141ea2790 as *const (), Throws::Discard), // sub_141ea2790
    Replacement::draft(0x141ea2e50, "r_141ea2e50", fn_141ea2e50 as *const ()), // sub_141ea2e50
    Replacement::draft(0x141ea2e80, "r_141ea2e80", fn_141ea2e80 as *const ()), // sub_141ea2e80
    Replacement::draft(0x141ea2eb0, "r_141ea2eb0", fn_141ea2eb0 as *const ()), // sub_141ea2eb0
    Replacement::draft(0x141ea2ee0, "r_141ea2ee0", fn_141ea2ee0 as *const ()), // sub_141ea2ee0
    Replacement::draft(0x141ea2f10, "r_141ea2f10", fn_141ea2f10 as *const ()), // sub_141ea2f10
    Replacement::draft(0x141ea2f40, "r_141ea2f40", fn_141ea2f40 as *const ()), // sub_141ea2f40
    Replacement::draft_throws(0x141ea2f50, "r_141ea2f50", fn_141ea2f50 as *const (), Throws::Caught), // sub_141ea2f50
    Replacement::draft(0x141ea35d0, "r_141ea35d0", fn_141ea35d0 as *const ()), // sub_141ea35d0
    Replacement::draft(0x141ea3710, "r_141ea3710", fn_141ea3710 as *const ()), // sub_141ea3710
    Replacement::draft_throws(0x141ea37f0, "r_141ea37f0", fn_141ea37f0 as *const (), Throws::Discard), // sub_141ea37f0
    Replacement::draft_throws(0x141ea3960, "r_141ea3960", fn_141ea3960 as *const (), Throws::Discard), // sub_141ea3960
    Replacement::draft_throws(0x141ea3ae0, "r_141ea3ae0", fn_141ea3ae0 as *const (), Throws::Discard), // sub_141ea3ae0
    Replacement::draft(0x141ea3cf0, "r_141ea3cf0", fn_141ea3cf0 as *const ()), // sub_141ea3cf0
    Replacement::draft(0x141ea3dd0, "r_141ea3dd0", tpl_2558::<0x8> as *const ()), // sub_141ea3dd0
    Replacement::draft(0x141ea3df0, "r_141ea3df0", tpl_20791 as *const ()), // sub_141ea3df0
    Replacement::draft(0x141ea3e00, "r_141ea3e00", tpl_41194::<0x14240a730> as *const ()), // sub_141ea3e00
    Replacement::draft(0x141ea3e30, "r_141ea3e30", fn_141ea3e30 as *const ()), // sub_141ea3e30
    Replacement::draft(0x141ea3f30, "r_141ea3f30", tpl_2879::<0x1401f3d00, 0x8, 0x141ea4e50, 0x143356ae8, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise<r::Expected<coregame::anonymous_namespace::DLCStatus,platform:
    Replacement::draft(0x141ea3f90, "r_141ea3f90", tpl_41196::<0x143356aa0, 0x8, 0x10, 0x9, 0x11, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_141ea3f90
    Replacement::draft(0x141ea3ff0, "r_141ea3ff0", tpl_41197::<0x1401f3d00, 0x8, 0x141ea4e50, 0x143356ae8> as *const ()), // sub_141ea3ff0
    Replacement::draft(0x141ea4030, "r_141ea4030", tpl_41197::<0x1401f3d00, 0x8, 0x141ea4e50, 0x143356ae8> as *const ()), // sub_141ea4030
    Replacement::draft(0x141ea4070, "r_141ea4070", tpl_2879::<0x1401f3d00, 0x8, 0x141ea4e50, 0x143356ae8, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<coregame::anonymous_namespace::DLCStatus,platf
    Replacement::draft(0x141ea40d0, "r_141ea40d0", tpl_41198::<0x143356aa0, 0x8, 0x10, 0x9, 0x11, 0x1401f3d90, 0x20, 0x28, 0x143356ad0> as *const ()), // sub_141ea40d0
    Replacement::draft(0x141ea4150, "r_141ea4150", fn_141ea4150 as *const ()), // sub_141ea4150
    Replacement::draft(0x141ea4290, "r_141ea4290", tpl_41200 as *const ()), // sub_141ea4290
    Replacement::draft(0x141ea4380, "r_141ea4380", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_141ea4380
    Replacement::draft(0x141ea43a0, "r_141ea43a0", tpl_23501::<0x11, 0x8, 0x14102d540, 0x20, 0x1401f3ee0> as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<coregame::anonymous_namespace::DLCStatus,platf
    Replacement::draft(0x141ea4440, "r_141ea4440", tpl_41201::<0x9, 0x14102d540> as *const ()), // sub_141ea4440
    Replacement::draft(0x141ea44b0, "r_141ea44b0", tpl_3579::<0x3, 0x4, 0x8, 0x14240aa00> as *const ()), // sub_141ea44b0
    Replacement::draft(0x141ea4500, "r_141ea4500", tpl_41202::<0x141ea4e50, 0x10, 0x14240ab10, 0x143356ab8, 0x141834d30> as *const ()), // r::internal::ContinuationData<coregame::OptionalContentManager::refreshAndMountDLC::?bref1::operator()::<lambda_1>&,?bre
    Replacement::draft(0x141ea4560, "r_141ea4560", fn_141ea4560 as *const ()), // sub_141ea4560
    Replacement::draft(0x141ea45f0, "r_141ea45f0", tpl_41204::<0x141ea4e50, 0x60, 0x14240ab10, 0x18, 0x143356b60, 0x141834d30> as *const ()), // r::internal::ContinuationData<coregame::OptionalContentManager::refreshAndMountDLC::<lambda_1>,r::Expected<platform::dlc
    Replacement::draft(0x141ea4650, "r_141ea4650", tpl_41205::<0x143356b18, 0x1402bf830, 0x10, 0x18> as *const ()), // sub_141ea4650
    Replacement::draft(0x141ea4690, "r_141ea4690", fn_141ea4690 as *const ()), // sub_141ea4690
    Replacement::draft(0x141ea4700, "r_141ea4700", fn_141ea4700 as *const ()), // sub_141ea4700
    Replacement::draft(0x141ea4860, "r_141ea4860", tpl_41208::<0x141ea4e50, 0x10, 0x14240ab10, 0x143356ab8> as *const ()), // sub_141ea4860
    Replacement::draft(0x141ea48a0, "r_141ea48a0", tpl_41208::<0x141ea4e50, 0x10, 0x14240ab10, 0x143356ab8> as *const ()), // sub_141ea48a0
    Replacement::draft(0x141ea48e0, "r_141ea48e0", tpl_41202::<0x141ea4e50, 0x10, 0x14240ab10, 0x143356ab8, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<coregame::OptionalContentManager::refreshAndMountDLC::?bref1::operator()::<lambda_1>&,?bre
    Replacement::draft(0x141ea4940, "r_141ea4940", tpl_41209::<0x141ea4e50, 0x60, 0x14240ab10, 0x18, 0x143356b60> as *const ()), // sub_141ea4940
    Replacement::draft(0x141ea4980, "r_141ea4980", tpl_41209::<0x141ea4e50, 0x60, 0x14240ab10, 0x18, 0x143356b60> as *const ()), // sub_141ea4980
    Replacement::draft(0x141ea49c0, "r_141ea49c0", tpl_41204::<0x141ea4e50, 0x60, 0x14240ab10, 0x18, 0x143356b60, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<coregame::OptionalContentManager::refreshAndMountDLC::<lambda_1>,r::Expected<platform::dlc
    Replacement::draft(0x141ea4a20, "r_141ea4a20", tpl_41210::<0x143356b18, 0x1402bf830, 0x10, 0x18, 0x143356b48> as *const ()), // sub_141ea4a20
    Replacement::draft(0x141ea4a60, "r_141ea4a60", fn_141ea4a60 as *const ()), // sub_141ea4a60
    Replacement::draft(0x141ea4ae0, "r_141ea4ae0", fn_141ea4ae0 as *const ()), // sub_141ea4ae0
    Replacement::draft(0x141ea4c10, "r_141ea4c10", tpl_28335::<0x24> as *const ()), // sub_141ea4c10
    Replacement::draft(0x141ea4c20, "r_141ea4c20", tpl_41213::<0x8, 0x24> as *const ()), // sub_141ea4c20
    Replacement::draft(0x141ea4c50, "r_141ea4c50", tpl_41214::<0x20, 0x24, 0x28, 0x48> as *const ()), // sub_141ea4c50
    Replacement::draft(0x141ea4c80, "r_141ea4c80", tpl_41215::<0x24, 0x28, 0x8, 0x142442de0, 0x20, 0x1c, 0x4, 0x14240ab10> as *const ()), // sub_141ea4c80
    Replacement::draft(0x141ea4dc0, "r_141ea4dc0", tpl_35117::<0x10, 0x18, 0x8, 0xc> as *const ()), // sub_141ea4dc0
    Replacement::draft(0x141ea4de0, "r_141ea4de0", fn_141ea4de0 as *const ()), // r::internal::ContinuationImpl<coregame::OptionalContentManager::refreshInstallationChunk::<lambda_1>,r::Expected<platfor
    Replacement::draft(0x141ea4e20, "r_141ea4e20", tpl_733::<0x14102ca60, 0x8> as *const ()), // sub_141ea4e20
    Replacement::draft(0x141ea4e50, "r_141ea4e50", tpl_41215::<0x24, 0x28, 0x8, 0x142442de0, 0x20, 0x1c, 0x4, 0x14240ab10> as *const ()), // sub_141ea4e50
    Replacement::draft(0x141ea4ee0, "r_141ea4ee0", tpl_41217::<0x8, 0x24> as *const ()), // sub_141ea4ee0
    Replacement::draft(0x141ea4f80, "r_141ea4f80", tpl_35117::<0x60, 0x68, 0x8, 0x24> as *const ()), // sub_141ea4f80
    Replacement::draft_throws(0x141ea4fa0, "r_141ea4fa0", fn_141ea4fa0 as *const (), Throws::Caught), // r::internal::ContinuationImpl<coregame::OptionalContentManager::refreshAndMountDLC::?bref1::operator()::<lambda_1>&,?bre
    Replacement::draft(0x141ea50e0, "r_141ea50e0", tpl_35117::<0xb0, 0xb8, 0x8, 0x24> as *const ()), // sub_141ea50e0
    Replacement::draft(0x141ea5140, "r_141ea5140", fn_141ea5140 as *const ()), // sub_141ea5140
    Replacement::draft(0x141ea51b0, "r_141ea51b0", fn_141ea51b0 as *const ()), // sub_141ea51b0
    Replacement::draft(0x141ea5200, "r_141ea5200", fn_141ea5200 as *const ()), // sub_141ea5200
    Replacement::draft(0x141f53cb0, "r_141f53cb0", fn_141f53cb0 as *const ()), // sub_141f53cb0
    Replacement::draft(0x141f53cf0, "r_141f53cf0", tpl_41205::<0x143390ff8, 0x1401f3d90, 0x10, 0x18> as *const ()), // sub_141f53cf0
    Replacement::draft(0x141f53d30, "r_141f53d30", tpl_41205::<0x143390e40, 0x1401f3d90, 0x10, 0x18> as *const ()), // sub_141f53d30
    Replacement::draft(0x141f53d70, "r_141f53d70", tpl_41205::<0x143390df8, 0x1401f3d90, 0x10, 0x18> as *const ()), // sub_141f53d70
    Replacement::draft(0x141f53db0, "r_141f53db0", tpl_35426::<0x143391088, 0x8, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_141f53db0
    Replacement::draft_throws(0x141f54570, "r_141f54570", fn_141f54570 as *const (), Throws::Caught), // r::internal::ContinuationImpl<anonymous_namespace::loginEpic::<lambda_1>,?bref5::Expected<platform::User,platform::Error
    Replacement::draft(0x141f54680, "r_141f54680", tpl_34374::<0x10, 0x18, 0x8> as *const ()), // sub_141f54680
    Replacement::draft(0x141f54730, "r_141f54730", tpl_34374::<0x10, 0x18, 0x8> as *const ()), // sub_141f54730
    Replacement::draft_throws(0x141f54750, "r_141f54750", fn_141f54750 as *const (), Throws::Caught), // r::internal::ContinuationImpl<coregame::GameServer::signInInitialUsers::<lambda_2>,void>::r::internal::ContinuationImpl<
    Replacement::draft(0x141f54850, "r_141f54850", tpl_34374::<0x10, 0x18, 0x8> as *const ()), // sub_141f54850
    Replacement::draft(0x141f54900, "r_141f54900", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_141f54900
    Replacement::draft(0x141f58e90, "r_141f58e90", tpl_42135::<0x143391818, 0x8> as *const ()), // sub_141f58e90
    Replacement::draft_throws(0x141f58eb0, "r_141f58eb0", fn_141f58eb0 as *const (), Throws::Discard), // r::JobCallable<coregame::GameObjectStreamingManager::processBatch::<lambda_2>>::r::JobCallable<coregame::GameObjectStrea
    Replacement::draft(0x141fa48f0, "r_141fa48f0", tpl_42615::<0x20, 0x8, 0x40, 0x28, 0x50, 0x1433ace48, 0x48, 0x58> as *const ()), // sub_141fa48f0
    Replacement::draft(0x141fa4930, "r_141fa4930", tpl_37789::<0x58, 0x50, 0x20, 0x8, 0x30, 0x18, 0x14112ef70> as *const ()), // r::JobCallable<r::parallelfordetail::parallelForEachJob<coregame::cloth::simulateEntity::SortedInstance,const ?bref4::?b
    Replacement::draft(0x141fa4980, "r_141fa4980", tpl_37788::<0x20, 0x1433acf08, 0x8, 0x28> as *const ()), // sub_141fa4980
    Replacement::draft_throws(0x141fa49b0, "r_141fa49b0", tpl_42616::<0x40, 0x38, 0x20, 0x8, 0x30, 0x18, 0x14112f130> as *const (), Throws::Discard), // r::JobCallable<r::parallelfordetail::parallelForEachJob<ecs::Entity<ecs::access::Definition<r::TypeList<ecs::access::Def
    Replacement::draft_throws(0x141fc0860, "r_141fc0860", fn_141fc0860 as *const (), Throws::Discard), // r::LambdaStreamJob<coregame::BundleLoader::constructContent::<lambda_1>>::r::LambdaStreamJob<coregame::BundleLoader::con
    Replacement::draft(0x141fc0940, "r_141fc0940", fn_141fc0940 as *const ()), // r::LambdaStreamJob<coregame::BundleLoader::destructContent::<lambda_1>>::r::LambdaStreamJob<coregame::BundleLoader::dest
    Replacement::draft(0x141ff20e0, "r_141ff20e0", tpl_41177::<0x20, 0x8, 0x40, 0x1433d3840, 0x28, 0x48> as *const ()), // sub_141ff20e0
    Replacement::draft_throws(0x141ff2120, "r_141ff2120", tpl_42616::<0x50, 0x48, 0x20, 0x8, 0x30, 0x18, 0x1411975d0> as *const (), Throws::Discard), // r::JobCallable<r::parallelfordetail::parallelForEachJob<coregame::ActiveParticleSystemList::ActiveParticleSystem,const ?
    Replacement::draft(0x141ff21b0, "r_141ff21b0", tpl_38069::<0x20, 0x8, 0x40, 0x1433d3858, 0x28, 0x48> as *const ()), // sub_141ff21b0
    Replacement::draft_throws(0x141ff21f0, "r_141ff21f0", tpl_42617::<0x20, 0x8, 0x18, 0x48, 0x30, 0x1411975d0> as *const (), Throws::Discard), // r::JobCallable<r::parallelForEach<r::DynamicArray<coregame::ActiveParticleSystemList::ActiveParticleSystem,r::Allocator<
    Replacement::draft_throws(0x142051390, "r_142051390", fn_142051390 as *const (), Throws::Discard), // r::JobCallable<d3d::StagingAllocManager::allocateNewPage::<lambda_2>>::r::JobCallable<d3d::StagingAllocManager::allocate
    Replacement::draft(0x142064150, "r_142064150", tpl_18960::<0x10, 0x142442de0> as *const ()), // sub_142064150
    Replacement::draft(0x142064170, "r_142064170", fn_142064170 as *const ()), // r::JobCallable<d3d::PSOManager::launchGenerationJobs::<lambda_3>>::r::JobCallable<d3d::PSOManager::launchGenerationJobs:
    Replacement::draft(0x1420641c0, "r_1420641c0", tpl_42135::<0x1433dc968, 0x8> as *const ()), // sub_1420641c0
    Replacement::draft_throws(0x1420641e0, "r_1420641e0", fn_1420641e0 as *const (), Throws::Discard), // r::JobCallable<d3d::PSOManager::launchGenerationJobs::<lambda_2>>::r::JobCallable<d3d::PSOManager::launchGenerationJobs:
    Replacement::draft(0x1420642c0, "r_1420642c0", fn_1420642c0 as *const ()), // sub_1420642c0
    Replacement::draft(0x142064390, "r_142064390", fn_142064390 as *const ()), // sub_142064390
    Replacement::draft(0x1420644f0, "r_1420644f0", fn_1420644f0 as *const ()), // sub_1420644f0
    Replacement::draft(0x142064570, "r_142064570", fn_142064570 as *const ()), // sub_142064570
    Replacement::draft_throws(0x142064600, "r_142064600", fn_142064600 as *const (), Throws::Discard), // sub_142064600
    Replacement::draft(0x142064c70, "r_142064c70", fn_142064c70 as *const ()), // sub_142064c70
    Replacement::draft(0x142064e10, "r_142064e10", fn_142064e10 as *const ()), // sub_142064e10
    Replacement::draft_throws(0x142065160, "r_142065160", fn_142065160 as *const (), Throws::Discard), // sub_142065160
    Replacement::draft(0x142065280, "r_142065280", fn_142065280 as *const ()), // sub_142065280
    Replacement::draft(0x142065420, "r_142065420", fn_142065420 as *const ()), // sub_142065420
    Replacement::draft(0x142065510, "r_142065510", fn_142065510 as *const ()), // sub_142065510
    Replacement::draft(0x142065530, "r_142065530", tpl_18960::<0x8, 0x142442de0> as *const ()), // sub_142065530
    Replacement::draft(0x142065550, "r_142065550", fn_142065550 as *const ()), // sub_142065550
    Replacement::draft_throws(0x142065570, "r_142065570", fn_142065570 as *const (), Throws::Discard), // sub_142065570
    Replacement::draft(0x1420656e0, "r_1420656e0", fn_1420656e0 as *const ()), // sub_1420656e0
    Replacement::draft_throws(0x142065710, "r_142065710", fn_142065710 as *const (), Throws::Discard), // sub_142065710
    Replacement::draft(0x1420657f0, "r_1420657f0", tpl_43569::<0x6, 0x8> as *const ()), // sub_1420657f0
    Replacement::draft(0x142065e20, "r_142065e20", fn_142065e20 as *const ()), // sub_142065e20
    Replacement::draft(0x142065fc0, "r_142065fc0", fn_142065fc0 as *const ()), // sub_142065fc0
    Replacement::draft_throws(0x142066050, "r_142066050", fn_142066050 as *const (), Throws::Discard), // sub_142066050
    Replacement::draft(0x142066180, "r_142066180", fn_142066180 as *const ()), // sub_142066180
    Replacement::draft(0x142066240, "r_142066240", fn_142066240 as *const ()), // sub_142066240
    Replacement::draft(0x142066320, "r_142066320", fn_142066320 as *const ()), // sub_142066320
    Replacement::draft(0x142066440, "r_142066440", fn_142066440 as *const ()), // sub_142066440
    Replacement::draft_throws(0x1420664a0, "r_1420664a0", fn_1420664a0 as *const (), Throws::Discard), // sub_1420664a0
    Replacement::draft_throws(0x142066500, "r_142066500", fn_142066500 as *const (), Throws::Discard), // sub_142066500
    Replacement::draft(0x1420666d0, "r_1420666d0", fn_1420666d0 as *const ()), // sub_1420666d0
    Replacement::draft(0x142066910, "r_142066910", fn_142066910 as *const ()), // sub_142066910
    Replacement::draft(0x142066b60, "r_142066b60", fn_142066b60 as *const ()), // sub_142066b60
    Replacement::draft(0x142066b80, "r_142066b80", fn_142066b80 as *const ()), // sub_142066b80
    Replacement::draft(0x142066c10, "r_142066c10", fn_142066c10 as *const ()), // sub_142066c10
    Replacement::draft(0x1420670e0, "r_1420670e0", fn_1420670e0 as *const ()), // sub_1420670e0
    Replacement::draft(0x1420673c0, "r_1420673c0", fn_1420673c0 as *const ()), // sub_1420673c0
    Replacement::draft_throws(0x1420673e0, "r_1420673e0", fn_1420673e0 as *const (), Throws::Discard), // sub_1420673e0
    Replacement::draft_throws(0x142067500, "r_142067500", fn_142067500 as *const (), Throws::Caught), // sub_142067500
    Replacement::draft_throws(0x1420676b0, "r_1420676b0", fn_1420676b0 as *const (), Throws::Discard), // sub_1420676b0
    Replacement::draft(0x142067ba0, "r_142067ba0", fn_142067ba0 as *const ()), // sub_142067ba0
    Replacement::draft(0x142067c10, "r_142067c10", fn_142067c10 as *const ()), // sub_142067c10
    Replacement::draft(0x142067c90, "r_142067c90", fn_142067c90 as *const ()), // sub_142067c90
    Replacement::draft(0x142067d50, "r_142067d50", fn_142067d50 as *const ()), // sub_142067d50
    Replacement::draft(0x142067e00, "r_142067e00", fn_142067e00 as *const ()), // sub_142067e00
    Replacement::draft(0x142067eb0, "r_142067eb0", fn_142067eb0 as *const ()), // sub_142067eb0
    Replacement::draft(0x142067f50, "r_142067f50", fn_142067f50 as *const ()), // sub_142067f50
    Replacement::draft(0x1420680c0, "r_1420680c0", fn_1420680c0 as *const ()), // sub_1420680c0
    Replacement::draft(0x142068150, "r_142068150", tpl_43520::<0x14397fb80, 0x1426455a8, 0x8, 0xa, 0x50, 0x58, 0x1, 0x14209dc80, 0x98, 0x14243b3e0, 0x142645598> as *const ()), // sub_142068150
    Replacement::draft(0x142068200, "r_142068200", fn_142068200 as *const ()), // sub_142068200
    Replacement::draft(0x1420682b0, "r_1420682b0", fn_1420682b0 as *const ()), // sub_1420682b0
    Replacement::draft(0x142068380, "r_142068380", fn_142068380 as *const ()), // sub_142068380
    Replacement::draft(0x142068440, "r_142068440", fn_142068440 as *const ()), // sub_142068440
    Replacement::draft_throws(0x142068530, "r_142068530", fn_142068530 as *const (), Throws::Caught), // sub_142068530
    Replacement::draft(0x142068780, "r_142068780", fn_142068780 as *const ()), // sub_142068780
    Replacement::draft_throws(0x142068850, "r_142068850", fn_142068850 as *const (), Throws::Discard), // sub_142068850
    Replacement::draft(0x142069060, "r_142069060", fn_142069060 as *const ()), // sub_142069060
    Replacement::draft(0x142069300, "r_142069300", tpl_43826 as *const ()), // sub_142069300
    Replacement::draft(0x142069310, "r_142069310", tpl_43826 as *const ()), // sub_142069310
    Replacement::draft(0x142069320, "r_142069320", fn_142069320 as *const ()), // sub_142069320
    Replacement::draft(0x1420693a0, "r_1420693a0", fn_1420693a0 as *const ()), // sub_1420693a0
    Replacement::draft(0x142069560, "r_142069560", fn_142069560 as *const ()), // sub_142069560
    Replacement::draft(0x142069580, "r_142069580", fn_142069580 as *const ()), // sub_142069580
    Replacement::draft_throws(0x1420695d0, "r_1420695d0", fn_1420695d0 as *const (), Throws::Caught), // sub_1420695d0
    Replacement::draft_throws(0x14206a100, "r_14206a100", fn_14206a100 as *const (), Throws::Caught), // sub_14206a100
    Replacement::draft_throws(0x14206a360, "r_14206a360", fn_14206a360 as *const (), Throws::Discard), // sub_14206a360
    Replacement::draft_throws(0x14206a5b0, "r_14206a5b0", fn_14206a5b0 as *const (), Throws::Discard), // sub_14206a5b0
    Replacement::draft_throws(0x14206a5e0, "r_14206a5e0", fn_14206a5e0 as *const (), Throws::Caught), // sub_14206a5e0
    Replacement::draft_throws(0x14206a7e0, "r_14206a7e0", fn_14206a7e0 as *const (), Throws::Caught), // sub_14206a7e0
    Replacement::draft_throws(0x14206aad0, "r_14206aad0", fn_14206aad0 as *const (), Throws::Caught), // sub_14206aad0
    Replacement::draft_throws(0x14206aec0, "r_14206aec0", fn_14206aec0 as *const (), Throws::Caught), // sub_14206aec0
    Replacement::draft_throws(0x14206b020, "r_14206b020", fn_14206b020 as *const (), Throws::Discard), // sub_14206b020
    Replacement::draft(0x14206b280, "r_14206b280", fn_14206b280 as *const ()), // sub_14206b280
    Replacement::draft_throws(0x14206b2a0, "r_14206b2a0", fn_14206b2a0 as *const (), Throws::Discard), // r::JobCallable<d3d::DeviceUtilRaytracing::raytrace::<lambda_1>>::r::JobCallable<d3d::DeviceUtilRaytracing::raytrace::<la
    Replacement::draft(0x142097dc0, "r_142097dc0", fn_142097dc0 as *const ()), // r::JobCallable<d3d::flushReleaseQueue::<lambda_2>>::r::JobCallable<d3d::flushReleaseQueue::<lambda_2>>::vfunc_0
    Replacement::draft(0x142097e20, "r_142097e20", fn_142097e20 as *const ()), // sub_142097e20
    Replacement::draft(0x142097f80, "r_142097f80", fn_142097f80 as *const ()), // r::JobCallable<d3d::flushReleaseQueue::<lambda_2>>::r::JobCallable<d3d::flushReleaseQueue::<lambda_2>>::vfunc_1
    Replacement::draft(0x1420a8e70, "r_1420a8e70", tpl_44614::<0x10, 0x8, 0x1420a9960> as *const ()), // r::JobCallable<anonymous_namespace::schedule::<lambda_1>>::r::JobCallable<anonymous_namespace::schedule::<lambda_1>>::vf
    Replacement::draft(0x1420a8eb0, "r_1420a8eb0", fn_1420a8eb0 as *const ()), // r::JobCallable<anonymous_namespace::schedule::<lambda_2>::operator()::<lambda_1>>::r::JobCallable<anonymous_namespace::s
    Replacement::draft(0x1420a8f20, "r_1420a8f20", fn_1420a8f20 as *const ()), // r::JobCallable<anonymous_namespace::schedule::<lambda_3>>::r::JobCallable<anonymous_namespace::schedule::<lambda_3>>::vf
    Replacement::draft(0x1420a8f80, "r_1420a8f80", tpl_44614::<0x10, 0x8, 0x1420a9960> as *const ()), // r::JobCallable<anonymous_namespace::startScheduling::<lambda_1>>::r::JobCallable<anonymous_namespace::startScheduling::<
    Replacement::draft(0x1420a91d0, "r_1420a91d0", fn_1420a91d0 as *const ()), // sub_1420a91d0
    Replacement::draft(0x1420a92f0, "r_1420a92f0", fn_1420a92f0 as *const ()), // sub_1420a92f0
    Replacement::draft(0x1420a9310, "r_1420a9310", tpl_44618::<0x8, 0x10, 0x1420a9960> as *const ()), // sub_1420a9310
    Replacement::draft(0x1420a9330, "r_1420a9330", fn_1420a9330 as *const ()), // sub_1420a9330
    Replacement::draft_throws(0x1420a9470, "r_1420a9470", fn_1420a9470 as *const (), Throws::Discard), // sub_1420a9470
    Replacement::draft(0x1420a9510, "r_1420a9510", tpl_44529::<0x8> as *const ()), // sub_1420a9510
    Replacement::draft(0x1420a9530, "r_1420a9530", fn_1420a9530 as *const ()), // sub_1420a9530
    Replacement::draft(0x1420a9570, "r_1420a9570", fn_1420a9570 as *const ()), // sub_1420a9570
    Replacement::draft(0x1420a95a0, "r_1420a95a0", fn_1420a95a0 as *const ()), // sub_1420a95a0
    Replacement::draft(0x1420a95c0, "r_1420a95c0", fn_1420a95c0 as *const ()), // sub_1420a95c0
    Replacement::draft(0x1420a9920, "r_1420a9920", tpl_44529::<0x8> as *const ()), // sub_1420a9920
    Replacement::draft(0x1420a9940, "r_1420a9940", tpl_44618::<0x8, 0x10, 0x1420a9960> as *const ()), // sub_1420a9940
    Replacement::draft(0x1420aa040, "r_1420aa040", fn_1420aa040 as *const ()), // sub_1420aa040
    Replacement::draft(0x1420aa060, "r_1420aa060", fn_1420aa060 as *const ()), // sub_1420aa060
    Replacement::draft(0x1420aa080, "r_1420aa080", fn_1420aa080 as *const ()), // sub_1420aa080
    Replacement::draft(0x1420aa0a0, "r_1420aa0a0", fn_1420aa0a0 as *const ()), // sub_1420aa0a0
    Replacement::draft(0x1420aa0f0, "r_1420aa0f0", fn_1420aa0f0 as *const ()), // sub_1420aa0f0
    Replacement::draft(0x1420aa130, "r_1420aa130", fn_1420aa130 as *const ()), // sub_1420aa130
    Replacement::draft(0x1420aa1a0, "r_1420aa1a0", tpl_22332::<0xffffffffffffffff, 0x18, 0x8, 0x1420aa290, 0x1420aa4b0, 0x1403d1980, 0x10, 0xfffffffffffffff8> as *const ()), // sub_1420aa1a0
    Replacement::draft(0x1420aa290, "r_1420aa290", fn_1420aa290 as *const ()), // sub_1420aa290
    Replacement::draft(0x1420aa310, "r_1420aa310", fn_1420aa310 as *const ()), // sub_1420aa310
    Replacement::draft(0x1420aa3d0, "r_1420aa3d0", fn_1420aa3d0 as *const ()), // sub_1420aa3d0
    Replacement::draft(0x1420aa590, "r_1420aa590", fn_1420aa590 as *const ()), // sub_1420aa590
    Replacement::draft(0x1420aa690, "r_1420aa690", tpl_44566::<0x8, 0x10, 0x18> as *const ()), // sub_1420aa690
    Replacement::draft(0x1420aa6c0, "r_1420aa6c0", tpl_44566::<0x8, 0x10, 0x18> as *const ()), // sub_1420aa6c0
    Replacement::draft(0x1420aa6f0, "r_1420aa6f0", fn_1420aa6f0 as *const ()), // sub_1420aa6f0
    Replacement::draft(0x1420aa710, "r_1420aa710", tpl_44479::<0x8, 0x1420aae40> as *const ()), // sub_1420aa710
    Replacement::draft(0x1420aa720, "r_1420aa720", fn_1420aa720 as *const ()), // sub_1420aa720
    Replacement::draft(0x1420aa750, "r_1420aa750", fn_1420aa750 as *const ()), // sub_1420aa750
    Replacement::draft_throws(0x1420aa790, "r_1420aa790", fn_1420aa790 as *const (), Throws::Discard), // sub_1420aa790
    Replacement::draft(0x1420aa7d0, "r_1420aa7d0", fn_1420aa7d0 as *const ()), // sub_1420aa7d0
    Replacement::draft_throws(0x1420aa800, "r_1420aa800", fn_1420aa800 as *const (), Throws::Discard), // sub_1420aa800
    Replacement::draft_throws(0x1420aa830, "r_1420aa830", fn_1420aa830 as *const (), Throws::Discard), // sub_1420aa830
    Replacement::draft_throws(0x1420aa8a0, "r_1420aa8a0", fn_1420aa8a0 as *const (), Throws::Discard), // sub_1420aa8a0
    Replacement::draft(0x1420aa910, "r_1420aa910", tpl_44575::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aa910
    Replacement::draft(0x1420aa960, "r_1420aa960", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aa960
    Replacement::draft(0x1420aa9b0, "r_1420aa9b0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aa9b0
    Replacement::draft(0x1420aaa00, "r_1420aaa00", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aaa00
    Replacement::draft(0x1420aaa50, "r_1420aaa50", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aaa50
    Replacement::draft(0x1420aaaa0, "r_1420aaaa0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aaaa0
    Replacement::draft(0x1420aaaf0, "r_1420aaaf0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aaaf0
    Replacement::draft(0x1420aab80, "r_1420aab80", tpl_40083::<0x28> as *const ()), // sub_1420aab80
    Replacement::draft(0x1420aab90, "r_1420aab90", tpl_44106::<0x28> as *const ()), // sub_1420aab90
    Replacement::draft(0x1420aaba0, "r_1420aaba0", tpl_44575::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aaba0
    Replacement::draft(0x1420aabf0, "r_1420aabf0", tpl_44106::<0x28> as *const ()), // sub_1420aabf0
    Replacement::draft(0x1420aac00, "r_1420aac00", tpl_44577::<0x8, 0x10, 0x18, 0x28> as *const ()), // sub_1420aac00
    Replacement::draft(0x1420aac30, "r_1420aac30", tpl_44578::<0x28> as *const ()), // sub_1420aac30
    Replacement::draft(0x1420aac50, "r_1420aac50", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aac50
    Replacement::draft(0x1420aaca0, "r_1420aaca0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aaca0
    Replacement::draft(0x1420aacf0, "r_1420aacf0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aacf0
    Replacement::draft(0x1420aad40, "r_1420aad40", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aad40
    Replacement::draft(0x1420aad90, "r_1420aad90", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420aad90
    Replacement::draft(0x1420aae10, "r_1420aae10", tpl_44476::<0x8, 0x10> as *const ()), // sub_1420aae10
    Replacement::draft(0x1420aae40, "r_1420aae40", fn_1420aae40 as *const ()), // sub_1420aae40
    Replacement::draft(0x1420aaf00, "r_1420aaf00", tpl_44511::<0x8, 0x10> as *const ()), // sub_1420aaf00
    Replacement::draft(0x1420aaf20, "r_1420aaf20", tpl_44470::<0x8, 0x18, 0x1420aae40> as *const ()), // sub_1420aaf20
    Replacement::draft(0x1420aaf50, "r_1420aaf50", tpl_44590::<0x8, 0x10, 0x18> as *const ()), // sub_1420aaf50
    Replacement::draft(0x1420aaf80, "r_1420aaf80", tpl_44591::<0x10, 0x18, 0x20> as *const ()), // sub_1420aaf80
    Replacement::draft(0x1420aafb0, "r_1420aafb0", tpl_44592 as *const ()), // sub_1420aafb0
    Replacement::draft(0x1420aafc0, "r_1420aafc0", fn_1420aafc0 as *const ()), // sub_1420aafc0
    Replacement::draft(0x1420ab040, "r_1420ab040", fn_1420ab040 as *const ()), // sub_1420ab040
    Replacement::draft(0x1420ab080, "r_1420ab080", fn_1420ab080 as *const ()), // sub_1420ab080
    Replacement::draft_throws(0x1420ab0b0, "r_1420ab0b0", fn_1420ab0b0 as *const (), Throws::Caught), // sub_1420ab0b0
    Replacement::draft(0x1420ab5b0, "r_1420ab5b0", fn_1420ab5b0 as *const ()), // sub_1420ab5b0
    Replacement::draft(0x1420ab660, "r_1420ab660", fn_1420ab660 as *const ()), // sub_1420ab660
    Replacement::draft(0x1420abb00, "r_1420abb00", fn_1420abb00 as *const ()), // sub_1420abb00
    Replacement::draft(0x1420abd10, "r_1420abd10", fn_1420abd10 as *const ()), // sub_1420abd10
    Replacement::draft(0x1420abd50, "r_1420abd50", fn_1420abd50 as *const ()), // sub_1420abd50
    Replacement::draft(0x1420ac410, "r_1420ac410", fn_1420ac410 as *const ()), // sub_1420ac410
    Replacement::draft_throws(0x1420ac540, "r_1420ac540", fn_1420ac540 as *const (), Throws::Caught), // sub_1420ac540
    Replacement::draft(0x1420ac710, "r_1420ac710", tpl_44654::<0x8, 0x10> as *const ()), // sub_1420ac710
    Replacement::draft(0x1420ac750, "r_1420ac750", fn_1420ac750 as *const ()), // sub_1420ac750
    Replacement::draft(0x1420ac780, "r_1420ac780", tpl_44654::<0x10, 0x18> as *const ()), // sub_1420ac780
    Replacement::draft_throws(0x1420ac7c0, "r_1420ac7c0", fn_1420ac7c0 as *const (), Throws::Caught), // sub_1420ac7c0
    Replacement::draft(0x1420aca20, "r_1420aca20", fn_1420aca20 as *const ()), // sub_1420aca20
    Replacement::draft(0x1420aca50, "r_1420aca50", fn_1420aca50 as *const ()), // sub_1420aca50
    Replacement::draft(0x1420aca80, "r_1420aca80", fn_1420aca80 as *const ()), // sub_1420aca80
    Replacement::draft(0x1420acac0, "r_1420acac0", fn_1420acac0 as *const ()), // sub_1420acac0
    Replacement::draft(0x1420acb00, "r_1420acb00", tpl_44468::<0x8> as *const ()), // sub_1420acb00
    Replacement::draft(0x1420acb50, "r_1420acb50", tpl_44471::<0x8, 0x1420acbe0> as *const ()), // sub_1420acb50
    Replacement::draft(0x1420acb60, "r_1420acb60", tpl_44472::<0x8> as *const ()), // sub_1420acb60
    Replacement::draft(0x1420acbb0, "r_1420acbb0", tpl_44476::<0x8, 0x10> as *const ()), // sub_1420acbb0
    Replacement::draft(0x1420acbe0, "r_1420acbe0", fn_1420acbe0 as *const ()), // sub_1420acbe0
    Replacement::draft(0x1420acc60, "r_1420acc60", fn_1420acc60 as *const ()), // sub_1420acc60
    Replacement::draft(0x1420acca0, "r_1420acca0", fn_1420acca0 as *const ()), // sub_1420acca0
    Replacement::draft_throws(0x1420acd60, "r_1420acd60", fn_1420acd60 as *const (), Throws::Caught), // sub_1420acd60
    Replacement::draft(0x1420acfc0, "r_1420acfc0", fn_1420acfc0 as *const ()), // sub_1420acfc0
    Replacement::draft(0x1420ad000, "r_1420ad000", fn_1420ad000 as *const ()), // sub_1420ad000
    Replacement::draft_throws(0x1420ad010, "r_1420ad010", fn_1420ad010 as *const (), Throws::Caught), // sub_1420ad010
    Replacement::draft_throws(0x1420ad2b0, "r_1420ad2b0", tpl_651::<0x10, 0x14241c190, 0x18, 0x8> as *const (), Throws::Discard), // sub_1420ad2b0
    Replacement::draft_throws(0x1420ad300, "r_1420ad300", fn_1420ad300 as *const (), Throws::Discard), // sub_1420ad300
    Replacement::draft(0x1420ad370, "r_1420ad370", fn_1420ad370 as *const ()), // sub_1420ad370
    Replacement::draft_throws(0x1420ad390, "r_1420ad390", tpl_44670::<0x14241c0a0> as *const (), Throws::Discard), // sub_1420ad390
    Replacement::draft_throws(0x1420ad3e0, "r_1420ad3e0", fn_1420ad3e0 as *const (), Throws::Discard), // sub_1420ad3e0
    Replacement::draft_throws(0x1420ad560, "r_1420ad560", tpl_44086::<0x14241c0a0> as *const (), Throws::Discard), // sub_1420ad560
    Replacement::draft_throws(0x1420ad840, "r_1420ad840", fn_1420ad840 as *const (), Throws::Discard), // sub_1420ad840
    Replacement::draft(0x1420ad8e0, "r_1420ad8e0", tpl_44520 as *const ()), // sub_1420ad8e0
    Replacement::draft(0x1420ad8f0, "r_1420ad8f0", fn_1420ad8f0 as *const ()), // sub_1420ad8f0
    Replacement::draft_throws(0x1420ad900, "r_1420ad900", fn_1420ad900 as *const (), Throws::Discard), // sub_1420ad900
    Replacement::draft_throws(0x1420ae0d0, "r_1420ae0d0", fn_1420ae0d0 as *const (), Throws::Caught), // sub_1420ae0d0
    Replacement::draft(0x1420ae1e0, "r_1420ae1e0", fn_1420ae1e0 as *const ()), // sub_1420ae1e0
    Replacement::draft(0x1420ae2d0, "r_1420ae2d0", fn_1420ae2d0 as *const ()), // sub_1420ae2d0
    Replacement::draft(0x1420ae310, "r_1420ae310", fn_1420ae310 as *const ()), // sub_1420ae310
    Replacement::draft_throws(0x1420ae320, "r_1420ae320", fn_1420ae320 as *const (), Throws::Discard), // sub_1420ae320
    Replacement::draft(0x1420aea60, "r_1420aea60", fn_1420aea60 as *const ()), // sub_1420aea60
    Replacement::draft_throws(0x1420aeb60, "r_1420aeb60", fn_1420aeb60 as *const (), Throws::Discard), // sub_1420aeb60
    Replacement::draft_throws(0x1420aedd0, "r_1420aedd0", fn_1420aedd0 as *const (), Throws::Caught), // sub_1420aedd0
    Replacement::draft_throws(0x1420aeeb0, "r_1420aeeb0", fn_1420aeeb0 as *const (), Throws::Discard), // sub_1420aeeb0
    Replacement::draft(0x1420aef30, "r_1420aef30", tpl_22265::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // sub_1420aef30
    Replacement::draft_throws(0x1420aef60, "r_1420aef60", fn_1420aef60 as *const (), Throws::Discard), // sub_1420aef60
    Replacement::draft(0x1420aeff0, "r_1420aeff0", tpl_44474::<0x1411eb740, 0x1411eb600> as *const ()), // sub_1420aeff0
    Replacement::draft_throws(0x1420af120, "r_1420af120", fn_1420af120 as *const (), Throws::Discard), // sub_1420af120
    Replacement::draft(0x1420af3c0, "r_1420af3c0", fn_1420af3c0 as *const ()), // sub_1420af3c0
    Replacement::draft(0x1420af520, "r_1420af520", tpl_37788::<0x20, 0x1433dd718, 0x8, 0x28> as *const ()), // sub_1420af520
    Replacement::draft_throws(0x1420af550, "r_1420af550", tpl_44685::<0x40, 0x8, 0x18, 0x30, 0x28, 0x20, 0x38, 0x1420b01c0> as *const (), Throws::Discard), // r::JobCallable<anonymous_namespace::sortIteration64::<lambda_1>>::r::JobCallable<anonymous_namespace::sortIteration64::<
    Replacement::draft(0x1420af5a0, "r_1420af5a0", tpl_37788::<0x20, 0x1433dd6e8, 0x8, 0x28> as *const ()), // sub_1420af5a0
    Replacement::draft_throws(0x1420af5d0, "r_1420af5d0", tpl_44685::<0x40, 0x8, 0x18, 0x30, 0x28, 0x20, 0x38, 0x1420b01c0> as *const (), Throws::Discard), // r::JobCallable<anonymous_namespace::sortIteration64::<lambda_2>>::r::JobCallable<anonymous_namespace::sortIteration64::<
    Replacement::draft(0x1420af620, "r_1420af620", tpl_37788::<0x20, 0x1433dd700, 0x8, 0x28> as *const ()), // sub_1420af620
    Replacement::draft_throws(0x1420af650, "r_1420af650", tpl_44685::<0x40, 0x8, 0x18, 0x30, 0x28, 0x20, 0x38, 0x1420afda0> as *const (), Throws::Discard), // r::JobCallable<anonymous_namespace::sortIteration32::<lambda_1>>::r::JobCallable<anonymous_namespace::sortIteration32::<
    Replacement::draft(0x1420af6a0, "r_1420af6a0", tpl_37788::<0x20, 0x1433dd6d0, 0x8, 0x28> as *const ()), // sub_1420af6a0
    Replacement::draft_throws(0x1420af6d0, "r_1420af6d0", tpl_44685::<0x40, 0x8, 0x18, 0x30, 0x28, 0x20, 0x38, 0x1420afda0> as *const (), Throws::Discard), // r::JobCallable<anonymous_namespace::sortIteration32::<lambda_2>>::r::JobCallable<anonymous_namespace::sortIteration32::<
    Replacement::draft_throws(0x1420af720, "r_1420af720", fn_1420af720 as *const (), Throws::Discard), // sub_1420af720
    Replacement::draft_throws(0x1420af9a0, "r_1420af9a0", fn_1420af9a0 as *const (), Throws::Discard), // sub_1420af9a0
    Replacement::draft(0x1420afc20, "r_1420afc20", fn_1420afc20 as *const ()), // sub_1420afc20
    Replacement::draft(0x1420afc40, "r_1420afc40", fn_1420afc40 as *const ()), // sub_1420afc40
    Replacement::draft_throws(0x1420afcb0, "r_1420afcb0", tpl_44690::<0x38, 0x10, 0x28, 0x20, 0x18, 0x30, 0x1420afda0> as *const (), Throws::Discard), // sub_1420afcb0
    Replacement::draft_throws(0x1420afd50, "r_1420afd50", tpl_44690::<0x38, 0x10, 0x28, 0x20, 0x18, 0x30, 0x1420afda0> as *const (), Throws::Discard), // sub_1420afd50
    Replacement::draft_throws(0x1420b00d0, "r_1420b00d0", tpl_44690::<0x38, 0x10, 0x28, 0x20, 0x18, 0x30, 0x1420b01c0> as *const (), Throws::Discard), // sub_1420b00d0
    Replacement::draft_throws(0x1420b0170, "r_1420b0170", tpl_44690::<0x38, 0x10, 0x28, 0x20, 0x18, 0x30, 0x1420b01c0> as *const (), Throws::Discard), // sub_1420b0170
    Replacement::draft(0x1420b04e0, "r_1420b04e0", fn_1420b04e0 as *const ()), // sub_1420b04e0
    Replacement::draft(0x1420b0730, "r_1420b0730", fn_1420b0730 as *const ()), // sub_1420b0730
    Replacement::draft(0x1420b08d0, "r_1420b08d0", fn_1420b08d0 as *const ()), // sub_1420b08d0
    Replacement::draft(0x1420b0910, "r_1420b0910", fn_1420b0910 as *const ()), // sub_1420b0910
    Replacement::draft(0x1420b0960, "r_1420b0960", fn_1420b0960 as *const ()), // sub_1420b0960
    Replacement::draft_throws(0x1420b09a0, "r_1420b09a0", fn_1420b09a0 as *const (), Throws::Discard), // sub_1420b09a0
    Replacement::draft(0x1420b09c0, "r_1420b09c0", fn_1420b09c0 as *const ()), // sub_1420b09c0
    Replacement::draft(0x1420b0a60, "r_1420b0a60", fn_1420b0a60 as *const ()), // sub_1420b0a60
    Replacement::draft(0x1420b0aa0, "r_1420b0aa0", fn_1420b0aa0 as *const ()), // sub_1420b0aa0
    Replacement::draft(0x1420b0b90, "r_1420b0b90", fn_1420b0b90 as *const ()), // sub_1420b0b90
    Replacement::draft(0x1420b0c90, "r_1420b0c90", fn_1420b0c90 as *const ()), // sub_1420b0c90
    Replacement::draft(0x1420b0e30, "r_1420b0e30", tpl_44489 as *const ()), // sub_1420b0e30
    Replacement::draft(0x1420b0f50, "r_1420b0f50", tpl_44481::<0xc> as *const ()), // sub_1420b0f50
    Replacement::draft(0x1420b0fb0, "r_1420b0fb0", tpl_44481::<0x8> as *const ()), // sub_1420b0fb0
    Replacement::draft(0x1420b0fd0, "r_1420b0fd0", tpl_44481::<0xc> as *const ()), // sub_1420b0fd0
    Replacement::draft_throws(0x1420b0ff0, "r_1420b0ff0", tpl_44477::<0x14241c0a0> as *const (), Throws::Discard), // sub_1420b0ff0
    Replacement::draft(0x1420b1030, "r_1420b1030", tpl_44492 as *const ()), // sub_1420b1030
    Replacement::draft(0x1420b1040, "r_1420b1040", tpl_37790::<0x20, 0x8, 0x30, 0x1433dd808, 0x28, 0x38> as *const ()), // sub_1420b1040
    Replacement::draft(0x1420b10d0, "r_1420b10d0", tpl_44702::<0x20, 0x1433dd7d8, 0x8, 0x28> as *const ()), // sub_1420b10d0
    Replacement::draft(0x1420b1100, "r_1420b1100", fn_1420b1100 as *const ()), // r::JobCallable<ecs::core::bindGroups::<lambda_2>>::r::JobCallable<ecs::core::bindGroups::<lambda_2>>::vfunc_1
    Replacement::draft(0x1420b1140, "r_1420b1140", tpl_42615::<0x20, 0x8, 0x40, 0x28, 0x50, 0x1433dd820, 0x48, 0x58> as *const ()), // sub_1420b1140
    Replacement::draft(0x1420b11f0, "r_1420b11f0", tpl_44705::<0x10, 0x1433dd7f0, 0x8, 0x18> as *const ()), // sub_1420b11f0
    Replacement::draft(0x1420b1220, "r_1420b1220", fn_1420b1220 as *const ()), // r::JobCallable<ecs::core::bindGroups::<lambda_4>>::r::JobCallable<ecs::core::bindGroups::<lambda_4>>::vfunc_1
    Replacement::draft(0x1420b12f0, "r_1420b12f0", tpl_44481::<0x8> as *const ()), // sub_1420b12f0
    Replacement::draft_throws(0x1420b1490, "r_1420b1490", fn_1420b1490 as *const (), Throws::Discard), // sub_1420b1490
    Replacement::draft(0x1420b1550, "r_1420b1550", tpl_44708::<0x8> as *const ()), // sub_1420b1550
    Replacement::draft(0x1420b15c0, "r_1420b15c0", tpl_44708::<0x8> as *const ()), // sub_1420b15c0
    Replacement::draft(0x1420b1670, "r_1420b1670", fn_1420b1670 as *const ()), // sub_1420b1670
    Replacement::draft(0x1420b16a0, "r_1420b16a0", fn_1420b16a0 as *const ()), // sub_1420b16a0
    Replacement::draft(0x1420b16e0, "r_1420b16e0", tpl_44711 as *const ()), // sub_1420b16e0
    Replacement::draft(0x1420b1710, "r_1420b1710", tpl_44712 as *const ()), // sub_1420b1710
    Replacement::draft(0x1420b1750, "r_1420b1750", tpl_44711 as *const ()), // sub_1420b1750
    Replacement::draft(0x1420b17b0, "r_1420b17b0", tpl_44712 as *const ()), // sub_1420b17b0
    Replacement::draft(0x1420b1890, "r_1420b1890", fn_1420b1890 as *const ()), // sub_1420b1890
    Replacement::draft(0x1420b18d0, "r_1420b18d0", fn_1420b18d0 as *const ()), // sub_1420b18d0
    Replacement::draft_throws(0x1420b1930, "r_1420b1930", fn_1420b1930 as *const (), Throws::Discard), // sub_1420b1930
    Replacement::draft(0x1420b19c0, "r_1420b19c0", fn_1420b19c0 as *const ()), // sub_1420b19c0
    Replacement::draft(0x1420b1a00, "r_1420b1a00", tpl_44717::<0x10, 0x20, 0x28> as *const ()), // sub_1420b1a00
    Replacement::draft(0x1420b1a30, "r_1420b1a30", fn_1420b1a30 as *const ()), // sub_1420b1a30
    Replacement::draft(0x1420b1aa0, "r_1420b1aa0", fn_1420b1aa0 as *const ()), // sub_1420b1aa0
    Replacement::draft(0x1420b2860, "r_1420b2860", fn_1420b2860 as *const ()), // sub_1420b2860
    Replacement::draft(0x1420b2d90, "r_1420b2d90", fn_1420b2d90 as *const ()), // sub_1420b2d90
    Replacement::draft(0x1420b2f50, "r_1420b2f50", tpl_44702::<0x20, 0x1433dd890, 0x8, 0x28> as *const ()), // sub_1420b2f50
    Replacement::draft(0x1420b2fd0, "r_1420b2fd0", tpl_42135::<0x1433dd8a8, 0x8> as *const ()), // sub_1420b2fd0
    Replacement::draft(0x1420b3260, "r_1420b3260", fn_1420b3260 as *const ()), // sub_1420b3260
    Replacement::draft(0x1420b3280, "r_1420b3280", fn_1420b3280 as *const ()), // sub_1420b3280
    Replacement::draft(0x1420b32f0, "r_1420b32f0", fn_1420b32f0 as *const ()), // sub_1420b32f0
    Replacement::draft(0x1420b3320, "r_1420b3320", fn_1420b3320 as *const ()), // sub_1420b3320
    Replacement::draft(0x1420b3370, "r_1420b3370", fn_1420b3370 as *const ()), // sub_1420b3370
    Replacement::draft(0x1420b33a0, "r_1420b33a0", fn_1420b33a0 as *const ()), // sub_1420b33a0
    Replacement::draft(0x1420b3780, "r_1420b3780", fn_1420b3780 as *const ()), // sub_1420b3780
    Replacement::draft(0x1420b38f0, "r_1420b38f0", tpl_44733::<0x18058, 0x1806c, 0x8> as *const ()), // sub_1420b38f0
    Replacement::draft_throws(0x1420b3920, "r_1420b3920", fn_1420b3920 as *const (), Throws::Discard), // sub_1420b3920
    Replacement::draft(0x1420b3a80, "r_1420b3a80", fn_1420b3a80 as *const ()), // sub_1420b3a80
    Replacement::draft(0x1420b3b10, "r_1420b3b10", fn_1420b3b10 as *const ()), // sub_1420b3b10
    Replacement::draft(0x1420b3bc0, "r_1420b3bc0", fn_1420b3bc0 as *const ()), // sub_1420b3bc0
    Replacement::draft(0x1420b4370, "r_1420b4370", tpl_44733::<0x18050, 0x18068, 0x8> as *const ()), // sub_1420b4370
    Replacement::draft(0x1420b43a0, "r_1420b43a0", fn_1420b43a0 as *const ()), // sub_1420b43a0
    Replacement::draft(0x1420b43c0, "r_1420b43c0", fn_1420b43c0 as *const ()), // sub_1420b43c0
    Replacement::draft_throws(0x1420b43f0, "r_1420b43f0", fn_1420b43f0 as *const (), Throws::Caught), // sub_1420b43f0
    Replacement::draft(0x1420b4460, "r_1420b4460", fn_1420b4460 as *const ()), // sub_1420b4460
    Replacement::draft(0x1420b44d0, "r_1420b44d0", fn_1420b44d0 as *const ()), // sub_1420b44d0
    Replacement::draft(0x1420b4590, "r_1420b4590", fn_1420b4590 as *const ()), // sub_1420b4590
    Replacement::draft(0x1420b45f0, "r_1420b45f0", fn_1420b45f0 as *const ()), // sub_1420b45f0
    Replacement::draft(0x1420b46d0, "r_1420b46d0", fn_1420b46d0 as *const ()), // sub_1420b46d0
    Replacement::draft(0x1420b4710, "r_1420b4710", fn_1420b4710 as *const ()), // sub_1420b4710
    Replacement::draft_throws(0x1420b4740, "r_1420b4740", fn_1420b4740 as *const (), Throws::Discard), // sub_1420b4740
    Replacement::draft(0x1420b47b0, "r_1420b47b0", fn_1420b47b0 as *const ()), // sub_1420b47b0
    Replacement::draft_throws(0x1420b47d0, "r_1420b47d0", fn_1420b47d0 as *const (), Throws::Discard), // sub_1420b47d0
    Replacement::draft(0x1420b4910, "r_1420b4910", tpl_37788::<0x20, 0x1433ddd28, 0x8, 0x28> as *const ()), // sub_1420b4910
    Replacement::draft(0x1420b4940, "r_1420b4940", fn_1420b4940 as *const ()), // r::JobCallable<ecs::memory::paged_dynamic_array::perPageParallelForEach<const unsigned int,anonymous_namespace::countDes
    Replacement::draft(0x1420b4ac0, "r_1420b4ac0", tpl_40773::<0x20, 0x1433ddd10, 0x8, 0x28> as *const ()), // sub_1420b4ac0
    Replacement::draft(0x1420b4af0, "r_1420b4af0", tpl_44750::<0x10, 0x8, 0x20, 0x30, 0x18> as *const ()), // r::JobCallable<ecs::memory::paged_dynamic_array::perPageParallelForEach<const unsigned int,r::Span<anonymous_namespace::
    Replacement::draft(0x1420b4b70, "r_1420b4b70", tpl_40773::<0x20, 0x1433ddc68, 0x8, 0x28> as *const ()), // sub_1420b4b70
    Replacement::draft(0x1420b4ba0, "r_1420b4ba0", tpl_44750::<0x10, 0x8, 0x20, 0x30, 0x18> as *const ()), // r::JobCallable<ecs::memory::paged_dynamic_array::perPageParallelForEach<const ecs::core::ComponentId,r::Span<anonymous_n
    Replacement::draft(0x1420b4c20, "r_1420b4c20", tpl_40773::<0x20, 0x1433ddc38, 0x8, 0x28> as *const ()), // sub_1420b4c20
    Replacement::draft(0x1420b4c50, "r_1420b4c50", tpl_44750::<0x10, 0x8, 0x20, 0x30, 0x18> as *const ()), // r::JobCallable<ecs::memory::paged_dynamic_array::perPageParallelForEach<const unsigned int,r::Span<anonymous_namespace::
    Replacement::draft(0x1420b4cd0, "r_1420b4cd0", tpl_40773::<0x20, 0x1433ddea8, 0x8, 0x28> as *const ()), // sub_1420b4cd0
    Replacement::draft(0x1420b4d00, "r_1420b4d00", tpl_44750::<0x10, 0x8, 0x20, 0x30, 0x18> as *const ()), // r::JobCallable<ecs::memory::paged_dynamic_array::perPageParallelForEach<const ecs::core::ComponentId,r::Span<anonymous_n
    Replacement::draft(0x1420b4d80, "r_1420b4d80", fn_1420b4d80 as *const ()), // sub_1420b4d80
    Replacement::draft(0x1420b4da0, "r_1420b4da0", fn_1420b4da0 as *const ()), // sub_1420b4da0
    Replacement::draft(0x1420b4df0, "r_1420b4df0", tpl_44753::<0x2> as *const ()), // sub_1420b4df0
    Replacement::draft(0x1420b4f60, "r_1420b4f60", fn_1420b4f60 as *const ()), // r::JobCallable<anonymous_namespace::handleNewGroups::<lambda_1>>::r::JobCallable<anonymous_namespace::handleNewGroups::<
    Replacement::draft(0x1420b51e0, "r_1420b51e0", tpl_44755::<0x20, 0x1420aa3d0, 0x1401de830> as *const ()), // sub_1420b51e0
    Replacement::draft(0x1420b5210, "r_1420b5210", fn_1420b5210 as *const ()), // r::JobCallable<ecs::core::execute::<lambda_4>>::r::JobCallable<ecs::core::execute::<lambda_4>>::vfunc_0
    Replacement::draft_throws(0x1420b5270, "r_1420b5270", tpl_40793::<0x14241c0a0> as *const (), Throws::Discard), // sub_1420b5270
    Replacement::draft(0x1420b5290, "r_1420b5290", tpl_41844 as *const ()), // sub_1420b5290
    Replacement::draft(0x1420b5470, "r_1420b5470", tpl_37788::<0x20, 0x1433ddc50, 0x8, 0x28> as *const ()), // sub_1420b5470
    Replacement::draft(0x1420b54b0, "r_1420b54b0", tpl_37788::<0x20, 0x1433ddde8, 0x8, 0x28> as *const ()), // sub_1420b54b0
    Replacement::draft(0x1420b54f0, "r_1420b54f0", tpl_37788::<0x20, 0x1433ddcc8, 0x8, 0x28> as *const ()), // sub_1420b54f0
    Replacement::draft(0x1420b5570, "r_1420b5570", tpl_37788::<0x20, 0x1433dddd0, 0x8, 0x28> as *const ()), // sub_1420b5570
    Replacement::draft(0x1420b5640, "r_1420b5640", tpl_37788::<0x20, 0x1433dde78, 0x8, 0x28> as *const ()), // sub_1420b5640
    Replacement::draft(0x1420b58c0, "r_1420b58c0", fn_1420b58c0 as *const ()), // sub_1420b58c0
    Replacement::draft_throws(0x1420b59c0, "r_1420b59c0", fn_1420b59c0 as *const (), Throws::Discard), // r::JobCallable<anonymous_namespace::handleNewGroups::<lambda_1>>::r::JobCallable<anonymous_namespace::handleNewGroups::<
    Replacement::draft(0x1420b5a20, "r_1420b5a20", tpl_37788::<0x20, 0x1433ddce0, 0x8, 0x28> as *const ()), // sub_1420b5a20
    Replacement::draft(0x1420b5a60, "r_1420b5a60", tpl_37788::<0x20, 0x1433ddcf8, 0x8, 0x28> as *const ()), // sub_1420b5a60
    Replacement::draft(0x1420b5aa0, "r_1420b5aa0", tpl_44702::<0x20, 0x1433ddd58, 0x8, 0x28> as *const ()), // sub_1420b5aa0
    Replacement::draft(0x1420b5ad0, "r_1420b5ad0", fn_1420b5ad0 as *const ()), // r::JobCallable<anonymous_namespace::fastParallelForEach<const anonymous_namespace::ComponentEntityPair,?bref2::handleRem
    Replacement::draft(0x1420b5b00, "r_1420b5b00", tpl_38069::<0x20, 0x8, 0x40, 0x1433ddcb0, 0x28, 0x48> as *const ()), // sub_1420b5b00
    Replacement::draft(0x1420b5b50, "r_1420b5b50", tpl_44702::<0x20, 0x1433ddec0, 0x8, 0x28> as *const ()), // sub_1420b5b50
    Replacement::draft(0x1420b5b90, "r_1420b5b90", tpl_44705::<0x10, 0x1433dde60, 0x8, 0x18> as *const ()), // sub_1420b5b90
    Replacement::draft(0x1420b5bd0, "r_1420b5bd0", tpl_40773::<0x20, 0x1433dde18, 0x8, 0x28> as *const ()), // sub_1420b5bd0
    Replacement::draft(0x1420b5c80, "r_1420b5c80", fn_1420b5c80 as *const ()), // sub_1420b5c80
    Replacement::draft(0x1420b5ce0, "r_1420b5ce0", fn_1420b5ce0 as *const ()), // r::JobCallable<ecs::core::execute::<lambda_4>::operator()::<lambda_1>>::r::JobCallable<ecs::core::execute::<lambda_4>::o
    Replacement::draft(0x1420b5dd0, "r_1420b5dd0", fn_1420b5dd0 as *const ()), // sub_1420b5dd0
    Replacement::draft(0x1420b5f50, "r_1420b5f50", fn_1420b5f50 as *const ()), // sub_1420b5f50
    Replacement::draft(0x1420b6000, "r_1420b6000", tpl_44753::<0x6> as *const ()), // sub_1420b6000
    Replacement::draft(0x1420b6780, "r_1420b6780", tpl_44705::<0x10, 0x1433ddd40, 0x8, 0x18> as *const ()), // sub_1420b6780
    Replacement::draft(0x1420b67b0, "r_1420b67b0", fn_1420b67b0 as *const ()), // r::JobCallable<anonymous_namespace::countCreates::<lambda_4>>::r::JobCallable<anonymous_namespace::countCreates::<lambda
    Replacement::draft(0x1420b6800, "r_1420b6800", tpl_42135::<0x1433ddc80, 0x8> as *const ()), // sub_1420b6800
    Replacement::draft(0x1420b6820, "r_1420b6820", fn_1420b6820 as *const ()), // r::JobCallable<anonymous_namespace::countCreates::<lambda_5>>::r::JobCallable<anonymous_namespace::countCreates::<lambda
    Replacement::draft(0x1420b6880, "r_1420b6880", tpl_37790::<0x20, 0x8, 0x30, 0x1433dde30, 0x28, 0x38> as *const ()), // sub_1420b6880
    Replacement::draft(0x1420b68c0, "r_1420b68c0", fn_1420b68c0 as *const ()), // r::JobCallable<anonymous_namespace::gatherModifications::<lambda_1>>::r::JobCallable<anonymous_namespace::gatherModifica
    Replacement::draft(0x1420b6960, "r_1420b6960", tpl_42135::<0x1433dde00, 0x8> as *const ()), // sub_1420b6960
    Replacement::draft(0x1420b6980, "r_1420b6980", tpl_44767::<0x18, 0x8, 0x20, 0x1420c1320> as *const ()), // r::JobCallable<anonymous_namespace::dispatchMoves::<lambda_1>>::r::JobCallable<anonymous_namespace::dispatchMoves::<lamb
    Replacement::draft(0x1420b69d0, "r_1420b69d0", tpl_42135::<0x1433dddb8, 0x8> as *const ()), // sub_1420b69d0
    Replacement::draft(0x1420b69f0, "r_1420b69f0", tpl_44767::<0x18, 0x8, 0x20, 0x1420c1320> as *const ()), // r::JobCallable<anonymous_namespace::dispatchMoves::<lambda_2>>::r::JobCallable<anonymous_namespace::dispatchMoves::<lamb
    Replacement::draft(0x1420b6a40, "r_1420b6a40", tpl_42135::<0x1433ddc98, 0x8> as *const ()), // sub_1420b6a40
    Replacement::draft(0x1420b6a60, "r_1420b6a60", tpl_44768::<0x8, 0x20, 0x10, 0xfffffffffffffff8, 0x18, 0x30> as *const ()), // r::JobCallable<anonymous_namespace::dispatchMoves::<lambda_3>>::r::JobCallable<anonymous_namespace::dispatchMoves::<lamb
    Replacement::draft(0x1420b6b00, "r_1420b6b00", tpl_42135::<0x1433ddc20, 0x8> as *const ()), // sub_1420b6b00
    Replacement::draft(0x1420b6b20, "r_1420b6b20", tpl_44768::<0x8, 0x20, 0x10, 0xfffffffffffffff8, 0x18, 0x30> as *const ()), // r::JobCallable<anonymous_namespace::dispatchMoves::<lambda_4>>::r::JobCallable<anonymous_namespace::dispatchMoves::<lamb
    Replacement::draft(0x1420b6cb0, "r_1420b6cb0", tpl_44705::<0x10, 0x1433dde90, 0x8, 0x18> as *const ()), // sub_1420b6cb0
    Replacement::draft(0x1420b6ce0, "r_1420b6ce0", fn_1420b6ce0 as *const ()), // r::JobCallable<anonymous_namespace::handleRemovesAndDestroys::?bref1::operator()::<lambda_1>>::r::JobCallable<anonymous_
    Replacement::draft(0x142129ed0, "r_142129ed0", tpl_41177::<0x20, 0x8, 0x40, 0x1433e2628, 0x28, 0x48> as *const ()), // sub_142129ed0
    Replacement::draft_throws(0x142129f10, "r_142129f10", tpl_42616::<0x50, 0x48, 0x20, 0x8, 0x30, 0x18, 0x14123f510> as *const (), Throws::Discard), // r::JobCallable<r::parallelfordetail::parallelForEachJob<const m::Vector3Template<int>,const anonymous_namespace::fillBlo
    Replacement::draft(0x142129fc0, "r_142129fc0", fn_142129fc0 as *const ()), // sub_142129fc0
    Replacement::draft(0x14212a300, "r_14212a300", crate::shards::subsystems::fn_14212a300 as *const ()), // sub_14212a300
    Replacement::draft(0x14212a390, "r_14212a390", tpl_381::<0x142129fc0, 0x14212b1e0> as *const ()), // sub_14212a390
    Replacement::draft(0x14212a4f0, "r_14212a4f0", tpl_38069::<0x20, 0x8, 0x40, 0x1433e2558, 0x28, 0x48> as *const ()), // sub_14212a4f0
    Replacement::draft_throws(0x14212a530, "r_14212a530", tpl_42617::<0x20, 0x8, 0x18, 0x48, 0x30, 0x14123f510> as *const (), Throws::Discard), // r::JobCallable<r::parallelForEach<r::Span<const m::Vector3Template<int>,-1>&,anonymous_namespace::fillBlocksInRange::<la
    Replacement::draft(0x1421813d0, "r_1421813d0", tpl_35426::<0x1433e7348, 0x8, 0x1401f4c80, 0x20, 0x28> as *const ()), // sub_1421813d0
    Replacement::draft(0x142181410, "r_142181410", tpl_41205::<0x1433e7330, 0x1402bf830, 0x10, 0x18> as *const ()), // sub_142181410
    Replacement::draft(0x1421815b0, "r_1421815b0", tpl_35429::<0x1433e7348, 0x8, 0x1401f4c80, 0x20, 0x28, 0x1433e7300> as *const ()), // sub_1421815b0
    Replacement::draft(0x142181600, "r_142181600", tpl_41210::<0x1433e7330, 0x1402bf830, 0x10, 0x18, 0x1433e7318> as *const ()), // sub_142181600
    Replacement::draft(0x1421816e0, "r_1421816e0", tpl_35117::<0x20, 0x28, 0x8, 0x10> as *const ()), // sub_1421816e0
    Replacement::draft(0x142181700, "r_142181700", fn_142181700 as *const ()), // r::internal::ContinuationImpl<platform::user::development::anonymous_namespace::development_signInNewUser::<lambda_1>,?b
    Replacement::draft(0x1421817f0, "r_1421817f0", tpl_35117::<0x10, 0x18, 0x8, 0xc> as *const ()), // sub_1421817f0
    Replacement::draft(0x142181810, "r_142181810", fn_142181810 as *const ()), // r::internal::ContinuationImpl<platform::user::development::addSignedInUsers::<lambda_1>,?bref7::Expected<platform::User,
    Replacement::draft(0x142181860, "r_142181860", tpl_46490::<0x8> as *const ()), // sub_142181860
    Replacement::draft_throws(0x142181880, "r_142181880", fn_142181880 as *const (), Throws::Caught), // sub_142181880
    Replacement::draft_throws(0x142181aa0, "r_142181aa0", fn_142181aa0 as *const (), Throws::Caught), // sub_142181aa0
    Replacement::draft_throws(0x142181c40, "r_142181c40", fn_142181c40 as *const (), Throws::Caught), // sub_142181c40
    Replacement::draft(0x142181dc0, "r_142181dc0", tpl_46494::<0x1412907c0> as *const ()), // sub_142181dc0
    Replacement::draft(0x142181df0, "r_142181df0", fn_142181df0 as *const ()), // sub_142181df0
    Replacement::draft(0x142181fd0, "r_142181fd0", fn_142181fd0 as *const ()), // sub_142181fd0
    Replacement::draft(0x142182020, "r_142182020", tpl_35426::<0x1433e7390, 0x8, 0x1402bf830, 0x20, 0x28> as *const ()), // sub_142182020
    Replacement::draft(0x1421821c0, "r_1421821c0", fn_1421821c0 as *const ()), // sub_1421821c0
    Replacement::draft(0x142182220, "r_142182220", tpl_35429::<0x1433e7390, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433e7360> as *const ()), // sub_142182220
    Replacement::draft(0x142182310, "r_142182310", tpl_35117::<0x28, 0x30, 0x8, 0x10> as *const ()), // sub_142182310
    Replacement::draft(0x142182330, "r_142182330", fn_142182330 as *const ()), // r::internal::ContinuationImpl<platform::user::internal::signIn::<lambda_1>,?bref7::Expected<void,platform::Error>>::r::i
    Replacement::draft(0x1421823f0, "r_1421823f0", tpl_35117::<0x20, 0x28, 0x8, 0xc> as *const ()), // sub_1421823f0
    Replacement::draft(0x142182410, "r_142182410", fn_142182410 as *const ()), // r::internal::ContinuationImpl<platform::user::internal::signOut::<lambda_1>,?bref5::storage::WriteResult>::r::internal::
    Replacement::draft(0x142182470, "r_142182470", fn_142182470 as *const ()), // sub_142182470
    Replacement::draft(0x142182490, "r_142182490", fn_142182490 as *const ()), // sub_142182490
    Replacement::draft(0x1421824c0, "r_1421824c0", fn_1421824c0 as *const ()), // sub_1421824c0
    Replacement::draft(0x142182530, "r_142182530", fn_142182530 as *const ()), // sub_142182530
    Replacement::draft(0x142182560, "r_142182560", fn_142182560 as *const ()), // sub_142182560
    Replacement::draft(0x142182580, "r_142182580", fn_142182580 as *const ()), // sub_142182580
    Replacement::draft(0x1421825a0, "r_1421825a0", fn_1421825a0 as *const ()), // sub_1421825a0
    Replacement::draft(0x142182740, "r_142182740", fn_142182740 as *const ()), // sub_142182740
    Replacement::draft(0x1421828b0, "r_1421828b0", fn_1421828b0 as *const ()), // sub_1421828b0
    Replacement::draft(0x1421829c0, "r_1421829c0", fn_1421829c0 as *const ()), // sub_1421829c0
    Replacement::draft(0x142182aa0, "r_142182aa0", fn_142182aa0 as *const ()), // sub_142182aa0
    Replacement::draft(0x142182bb0, "r_142182bb0", tpl_46512::<0x8, 0x10> as *const ()), // sub_142182bb0
    Replacement::draft(0x142182bd0, "r_142182bd0", fn_142182bd0 as *const ()), // sub_142182bd0
    Replacement::draft(0x142182dc0, "r_142182dc0", fn_142182dc0 as *const ()), // sub_142182dc0
    Replacement::draft(0x142182e20, "r_142182e20", fn_142182e20 as *const ()), // sub_142182e20
    Replacement::draft(0x142182ec0, "r_142182ec0", fn_142182ec0 as *const ()), // sub_142182ec0
    Replacement::draft(0x142182fd0, "r_142182fd0", fn_142182fd0 as *const ()), // sub_142182fd0
    Replacement::draft(0x142183000, "r_142183000", fn_142183000 as *const ()), // sub_142183000
    Replacement::draft(0x1421830d0, "r_1421830d0", fn_1421830d0 as *const ()), // sub_1421830d0
    Replacement::draft(0x1421831c0, "r_1421831c0", fn_1421831c0 as *const ()), // sub_1421831c0
    Replacement::draft(0x1421831f0, "r_1421831f0", fn_1421831f0 as *const ()), // sub_1421831f0
    Replacement::draft(0x1421833d0, "r_1421833d0", fn_1421833d0 as *const ()), // sub_1421833d0
    Replacement::draft(0x1421834b0, "r_1421834b0", fn_1421834b0 as *const ()), // sub_1421834b0
    Replacement::draft(0x1421835d0, "r_1421835d0", fn_1421835d0 as *const ()), // sub_1421835d0
    Replacement::draft(0x1421836b0, "r_1421836b0", fn_1421836b0 as *const ()), // sub_1421836b0
    Replacement::draft_throws(0x142183700, "r_142183700", fn_142183700 as *const (), Throws::Caught), // sub_142183700
    Replacement::draft_throws(0x142183a60, "r_142183a60", fn_142183a60 as *const (), Throws::Caught), // sub_142183a60
    Replacement::draft(0x142183db0, "r_142183db0", fn_142183db0 as *const ()), // sub_142183db0
    Replacement::draft(0x142183f20, "r_142183f20", fn_142183f20 as *const ()), // sub_142183f20
    Replacement::draft_throws(0x142184160, "r_142184160", fn_142184160 as *const (), Throws::Discard), // sub_142184160
    Replacement::draft_throws(0x1421841c0, "r_1421841c0", fn_1421841c0 as *const (), Throws::Caught), // sub_1421841c0
    Replacement::draft(0x142184360, "r_142184360", fn_142184360 as *const ()), // sub_142184360
    Replacement::draft(0x142184400, "r_142184400", fn_142184400 as *const ()), // sub_142184400
    Replacement::draft_throws(0x142184610, "r_142184610", fn_142184610 as *const (), Throws::Caught), // sub_142184610
    Replacement::draft(0x142184980, "r_142184980", fn_142184980 as *const ()), // sub_142184980
    Replacement::draft(0x1421849f0, "r_1421849f0", fn_1421849f0 as *const ()), // sub_1421849f0
    Replacement::draft(0x142184b70, "r_142184b70", fn_142184b70 as *const ()), // sub_142184b70
    Replacement::draft(0x142184cb0, "r_142184cb0", fn_142184cb0 as *const ()), // sub_142184cb0
    Replacement::draft(0x142184e10, "r_142184e10", fn_142184e10 as *const ()), // sub_142184e10
    Replacement::draft(0x142184f50, "r_142184f50", fn_142184f50 as *const ()), // sub_142184f50
    Replacement::draft(0x142185050, "r_142185050", fn_142185050 as *const ()), // sub_142185050
    Replacement::draft(0x142185070, "r_142185070", fn_142185070 as *const ()), // sub_142185070
    Replacement::draft(0x1421850b0, "r_1421850b0", fn_1421850b0 as *const ()), // sub_1421850b0
    Replacement::draft(0x142185200, "r_142185200", fn_142185200 as *const ()), // sub_142185200
    Replacement::draft(0x142185240, "r_142185240", fn_142185240 as *const ()), // sub_142185240
    Replacement::draft(0x142185270, "r_142185270", fn_142185270 as *const ()), // sub_142185270
    Replacement::draft(0x1421852a0, "r_1421852a0", fn_1421852a0 as *const ()), // sub_1421852a0
    Replacement::draft(0x142185340, "r_142185340", fn_142185340 as *const ()), // sub_142185340
    Replacement::draft(0x1421854a0, "r_1421854a0", fn_1421854a0 as *const ()), // sub_1421854a0
    Replacement::draft(0x1421854d0, "r_1421854d0", fn_1421854d0 as *const ()), // sub_1421854d0
    Replacement::draft(0x142185500, "r_142185500", tpl_46545::<0xb88, 0xb80, 0x8> as *const ()), // sub_142185500
    Replacement::draft(0x1421855a0, "r_1421855a0", fn_1421855a0 as *const ()), // sub_1421855a0
    Replacement::draft(0x142185650, "r_142185650", fn_142185650 as *const ()), // sub_142185650
    Replacement::draft(0x142185680, "r_142185680", fn_142185680 as *const ()), // sub_142185680
    Replacement::draft(0x142185720, "r_142185720", fn_142185720 as *const ()), // sub_142185720
    Replacement::draft(0x142185750, "r_142185750", fn_142185750 as *const ()), // sub_142185750
    Replacement::draft(0x1421857f0, "r_1421857f0", tpl_46545::<0xa88, 0xa80, 0x8> as *const ()), // sub_1421857f0
    Replacement::draft_throws(0x142185880, "r_142185880", fn_142185880 as *const (), Throws::Caught), // sub_142185880
    Replacement::draft(0x142185a00, "r_142185a00", tpl_46552::<0x10, 0x1920> as *const ()), // sub_142185a00
    Replacement::draft(0x142185a30, "r_142185a30", fn_142185a30 as *const ()), // sub_142185a30
    Replacement::draft_throws(0x142185ac0, "r_142185ac0", fn_142185ac0 as *const (), Throws::Discard), // sub_142185ac0
    Replacement::draft(0x142185af0, "r_142185af0", fn_142185af0 as *const ()), // sub_142185af0
    Replacement::draft(0x142185da0, "r_142185da0", fn_142185da0 as *const ()), // sub_142185da0
    Replacement::draft(0x142185df0, "r_142185df0", fn_142185df0 as *const ()), // sub_142185df0
    Replacement::draft(0x142185fe0, "r_142185fe0", fn_142185fe0 as *const ()), // sub_142185fe0
    Replacement::draft(0x1421860b0, "r_1421860b0", fn_1421860b0 as *const ()), // sub_1421860b0
    Replacement::draft(0x1421860d0, "r_1421860d0", fn_1421860d0 as *const ()), // sub_1421860d0
    Replacement::draft(0x1421861d0, "r_1421861d0", fn_1421861d0 as *const ()), // sub_1421861d0
    Replacement::draft(0x1421861f0, "r_1421861f0", fn_1421861f0 as *const ()), // sub_1421861f0
    Replacement::draft(0x142186290, "r_142186290", tpl_30308::<0xa80> as *const ()), // sub_142186290
    Replacement::draft(0x1421862b0, "r_1421862b0", fn_1421862b0 as *const ()), // sub_1421862b0
    Replacement::draft(0x142186360, "r_142186360", fn_142186360 as *const ()), // sub_142186360
    Replacement::draft(0x142186390, "r_142186390", fn_142186390 as *const ()), // sub_142186390
    Replacement::draft(0x1421863e0, "r_1421863e0", fn_1421863e0 as *const ()), // sub_1421863e0
    Replacement::draft(0x142186460, "r_142186460", fn_142186460 as *const ()), // sub_142186460
    Replacement::draft(0x1421864c0, "r_1421864c0", fn_1421864c0 as *const ()), // sub_1421864c0
    Replacement::draft(0x1421864e0, "r_1421864e0", fn_1421864e0 as *const ()), // sub_1421864e0
    Replacement::draft(0x142186520, "r_142186520", fn_142186520 as *const ()), // sub_142186520
    Replacement::draft(0x142186650, "r_142186650", fn_142186650 as *const ()), // sub_142186650
    Replacement::draft_throws(0x142186690, "r_142186690", fn_142186690 as *const (), Throws::Discard), // sub_142186690
    Replacement::draft(0x1421866d0, "r_1421866d0", fn_1421866d0 as *const ()), // sub_1421866d0
    Replacement::draft(0x142186740, "r_142186740", fn_142186740 as *const ()), // sub_142186740
    Replacement::draft(0x1421867d0, "r_1421867d0", fn_1421867d0 as *const ()), // sub_1421867d0
    Replacement::draft(0x142186800, "r_142186800", tpl_44095 as *const ()), // sub_142186800
    Replacement::draft(0x142186850, "r_142186850", tpl_46576::<0xc> as *const ()), // sub_142186850
    Replacement::draft(0x142186930, "r_142186930", tpl_44098::<0xc> as *const ()), // sub_142186930
    Replacement::draft(0x142186970, "r_142186970", fn_142186970 as *const ()), // sub_142186970
    Replacement::draft(0x142186980, "r_142186980", tpl_44102 as *const ()), // sub_142186980
    Replacement::draft(0x142186a60, "r_142186a60", tpl_46576::<0x8> as *const ()), // sub_142186a60
    Replacement::draft(0x142186ab0, "r_142186ab0", fn_142186ab0 as *const ()), // sub_142186ab0
    Replacement::draft(0x142186ad0, "r_142186ad0", fn_142186ad0 as *const ()), // sub_142186ad0
    Replacement::draft(0x142186b50, "r_142186b50", tpl_44098::<0x8> as *const ()), // sub_142186b50
    Replacement::draft_throws(0x142186bc0, "r_142186bc0", fn_142186bc0 as *const (), Throws::Discard), // sub_142186bc0
    Replacement::draft_throws(0x142186d60, "r_142186d60", tpl_37142::<0x8, 0x38, 0x142186bc0> as *const (), Throws::Discard), // sub_142186d60
    Replacement::draft(0x142186d90, "r_142186d90", tpl_37143::<0x10, 0x30, 0x20> as *const ()), // sub_142186d90
    Replacement::draft_throws(0x142186db0, "r_142186db0", tpl_37144::<0x8, 0x142494c40, 0x142494dc0, 0x142186bc0> as *const (), Throws::Discard), // sub_142186db0
    Replacement::draft(0x142186e40, "r_142186e40", tpl_46581::<0x10, 0x8, 0x28> as *const ()), // sub_142186e40
    Replacement::draft(0x142186ed0, "r_142186ed0", fn_142186ed0 as *const ()), // sub_142186ed0
    Replacement::draft_throws(0x142186fa0, "r_142186fa0", fn_142186fa0 as *const (), Throws::Caught), // sub_142186fa0
    Replacement::draft_throws(0x1421874f0, "r_1421874f0", fn_1421874f0 as *const (), Throws::Caught), // sub_1421874f0
    Replacement::draft_throws(0x142187560, "r_142187560", fn_142187560 as *const (), Throws::Caught), // sub_142187560
    Replacement::draft_throws(0x142187820, "r_142187820", fn_142187820 as *const (), Throws::Caught), // sub_142187820
    Replacement::draft_throws(0x1421878d0, "r_1421878d0", fn_1421878d0 as *const (), Throws::Caught), // sub_1421878d0
    Replacement::draft_throws(0x142187fa0, "r_142187fa0", fn_142187fa0 as *const (), Throws::Caught), // sub_142187fa0
    Replacement::draft_throws(0x142188070, "r_142188070", fn_142188070 as *const (), Throws::Caught), // sub_142188070
    Replacement::draft_throws(0x142188140, "r_142188140", fn_142188140 as *const (), Throws::Caught), // sub_142188140
    Replacement::draft(0x142188250, "r_142188250", fn_142188250 as *const ()), // sub_142188250
    Replacement::draft_throws(0x142188320, "r_142188320", fn_142188320 as *const (), Throws::Caught), // sub_142188320
    Replacement::draft_throws(0x142188ce0, "r_142188ce0", fn_142188ce0 as *const (), Throws::Caught), // sub_142188ce0
    Replacement::draft(0x142188d60, "r_142188d60", tpl_46581::<0x10, 0x8, 0x28> as *const ()), // sub_142188d60
    Replacement::draft(0x142188dd0, "r_142188dd0", fn_142188dd0 as *const ()), // sub_142188dd0
    Replacement::draft_throws(0x142188df0, "r_142188df0", fn_142188df0 as *const (), Throws::Caught), // sub_142188df0
    Replacement::draft_throws(0x1421890a0, "r_1421890a0", fn_1421890a0 as *const (), Throws::Caught), // sub_1421890a0
    Replacement::draft_throws(0x142189150, "r_142189150", fn_142189150 as *const (), Throws::Caught), // sub_142189150
    Replacement::draft_throws(0x1421891e0, "r_1421891e0", fn_1421891e0 as *const (), Throws::Caught), // sub_1421891e0
    Replacement::draft_throws(0x142189420, "r_142189420", fn_142189420 as *const (), Throws::Caught), // sub_142189420
    Replacement::draft_throws(0x142189690, "r_142189690", fn_142189690 as *const (), Throws::Caught), // sub_142189690
    Replacement::draft_throws(0x142189860, "r_142189860", fn_142189860 as *const (), Throws::Caught), // sub_142189860
    Replacement::draft_throws(0x142189a90, "r_142189a90", fn_142189a90 as *const (), Throws::Caught), // sub_142189a90
    Replacement::draft_throws(0x14218a2b0, "r_14218a2b0", fn_14218a2b0 as *const (), Throws::Caught), // sub_14218a2b0
    Replacement::draft_throws(0x14218a3e0, "r_14218a3e0", fn_14218a3e0 as *const (), Throws::Caught), // sub_14218a3e0
    Replacement::draft_throws(0x14218a520, "r_14218a520", fn_14218a520 as *const (), Throws::Caught), // sub_14218a520
    Replacement::draft(0x14218a680, "r_14218a680", fn_14218a680 as *const ()), // sub_14218a680
    Replacement::draft(0x14218a6c0, "r_14218a6c0", fn_14218a6c0 as *const ()), // sub_14218a6c0
    Replacement::draft_throws(0x14218a700, "r_14218a700", fn_14218a700 as *const (), Throws::Discard), // sub_14218a700
    Replacement::draft(0x14218a730, "r_14218a730", fn_14218a730 as *const ()), // sub_14218a730
    Replacement::draft_throws(0x14218a780, "r_14218a780", fn_14218a780 as *const (), Throws::Caught), // sub_14218a780
    Replacement::draft_throws(0x14218aaa0, "r_14218aaa0", fn_14218aaa0 as *const (), Throws::Caught), // sub_14218aaa0
    Replacement::draft_throws(0x14218ada0, "r_14218ada0", fn_14218ada0 as *const (), Throws::Caught), // sub_14218ada0
    Replacement::draft(0x14218b000, "r_14218b000", fn_14218b000 as *const ()), // sub_14218b000
    Replacement::draft_throws(0x14218b030, "r_14218b030", fn_14218b030 as *const (), Throws::Discard), // sub_14218b030
    Replacement::draft_throws(0x14218b220, "r_14218b220", fn_14218b220 as *const (), Throws::Discard), // sub_14218b220
    Replacement::draft_throws(0x14218b2f0, "r_14218b2f0", fn_14218b2f0 as *const (), Throws::Caught), // sub_14218b2f0
    Replacement::draft_throws(0x14218b4a0, "r_14218b4a0", fn_14218b4a0 as *const (), Throws::Caught), // sub_14218b4a0
    Replacement::draft_throws(0x14218b600, "r_14218b600", fn_14218b600 as *const (), Throws::Discard), // sub_14218b600
    Replacement::draft(0x14218b6b0, "r_14218b6b0", tpl_37328::<0x20> as *const ()), // sub_14218b6b0
    Replacement::draft(0x14218b6e0, "r_14218b6e0", fn_14218b6e0 as *const ()), // sub_14218b6e0
    Replacement::draft(0x14218b830, "r_14218b830", fn_14218b830 as *const ()), // sub_14218b830
    Replacement::draft_throws(0x14218b980, "r_14218b980", fn_14218b980 as *const (), Throws::Caught), // sub_14218b980
    Replacement::draft_throws(0x14218b9d0, "r_14218b9d0", fn_14218b9d0 as *const (), Throws::Caught), // sub_14218b9d0
    Replacement::draft_throws(0x14218ba20, "r_14218ba20", fn_14218ba20 as *const (), Throws::Caught), // sub_14218ba20
    Replacement::draft_throws(0x14218bc70, "r_14218bc70", fn_14218bc70 as *const (), Throws::Caught), // sub_14218bc70
    Replacement::draft_throws(0x14218bf90, "r_14218bf90", fn_14218bf90 as *const (), Throws::Caught), // sub_14218bf90
    Replacement::draft_throws(0x14218c160, "r_14218c160", fn_14218c160 as *const (), Throws::Caught), // sub_14218c160
    Replacement::draft_throws(0x14218c2f0, "r_14218c2f0", fn_14218c2f0 as *const (), Throws::Caught), // sub_14218c2f0
    Replacement::draft(0x14218c4f0, "r_14218c4f0", tpl_46624::<0x4> as *const ()), // sub_14218c4f0
    Replacement::draft_throws(0x14218c510, "r_14218c510", fn_14218c510 as *const (), Throws::Caught), // sub_14218c510
    Replacement::draft_throws(0x14218ca30, "r_14218ca30", fn_14218ca30 as *const (), Throws::Caught), // sub_14218ca30
    Replacement::draft_throws(0x14218cd90, "r_14218cd90", fn_14218cd90 as *const (), Throws::Caught), // sub_14218cd90
    Replacement::draft_throws(0x14218cf60, "r_14218cf60", fn_14218cf60 as *const (), Throws::Caught), // sub_14218cf60
    Replacement::draft_throws(0x14218d390, "r_14218d390", fn_14218d390 as *const (), Throws::Caught), // sub_14218d390
    Replacement::draft_throws(0x14218d520, "r_14218d520", fn_14218d520 as *const (), Throws::Caught), // sub_14218d520
    Replacement::draft(0x14218d740, "r_14218d740", tpl_44066::<0x38, 0x1401fa4d0> as *const ()), // sub_14218d740
    Replacement::draft(0x14218d770, "r_14218d770", tpl_46631::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationData<platform::storage::handleWriteError::<lambda_1>,?bref6::Expected<platform::storage::Handle
    Replacement::draft(0x14218d7f0, "r_14218d7f0", tpl_44066::<0x38, 0x1401fa4d0> as *const ()), // sub_14218d7f0
    Replacement::draft(0x14218d820, "r_14218d820", tpl_46631::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationData<platform::storage::handleReadError::<lambda_1>,?bref6::Expected<platform::storage::HandleE
    Replacement::draft(0x14218d8a0, "r_14218d8a0", tpl_46632::<0x1433e7640, 0x38, 0x1401fa4d0, 0x50, 0x14049ebe0> as *const ()), // sub_14218d8a0
    Replacement::draft(0x14218d8f0, "r_14218d8f0", tpl_46632::<0x1433e7690, 0x38, 0x1401fa4d0, 0x50, 0x14049ebe0> as *const ()), // sub_14218d8f0
    Replacement::draft(0x14218d940, "r_14218d940", tpl_46633::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0> as *const ()), // sub_14218d940
    Replacement::draft(0x14218d9a0, "r_14218d9a0", tpl_46633::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0> as *const ()), // sub_14218d9a0
    Replacement::draft(0x14218da00, "r_14218da00", tpl_46631::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<platform::storage::handleWriteError::<lambda_1>,?bref6::Expected<platform::storage::Handle
    Replacement::draft(0x14218da80, "r_14218da80", tpl_46633::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0> as *const ()), // sub_14218da80
    Replacement::draft(0x14218dae0, "r_14218dae0", tpl_46633::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0> as *const ()), // sub_14218dae0
    Replacement::draft(0x14218db40, "r_14218db40", tpl_46631::<0x14049eb20, 0x40, 0x8, 0x20, 0x38, 0x1430b57a0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<platform::storage::handleReadError::<lambda_1>,?bref6::Expected<platform::storage::HandleE
    Replacement::draft(0x14218dbc0, "r_14218dbc0", tpl_46634::<0x1433e7640, 0x38, 0x1401fa4d0, 0x50, 0x14049ebe0, 0x1433e7718> as *const ()), // sub_14218dbc0
    Replacement::draft(0x14218dc10, "r_14218dc10", tpl_46634::<0x1433e7690, 0x38, 0x1401fa4d0, 0x50, 0x14049ebe0, 0x1433e74f8> as *const ()), // sub_14218dc10
    Replacement::draft(0x14218dd40, "r_14218dd40", tpl_46635::<0x14049d200> as *const ()), // sub_14218dd40
    Replacement::draft_throws(0x14218dd70, "r_14218dd70", tpl_46636::<0x40, 0x142645878, 0x10, 0x4, 0x50, 0x14049fb80, 0x14218dd8f> as *const (), Throws::Discard), // r::internal::ContinuationImpl<platform::storage::handleWriteError::<lambda_1>,?bref6::Expected<platform::storage::Handle
    Replacement::draft(0x14218de50, "r_14218de50", tpl_46635::<0x14049d200> as *const ()), // sub_14218de50
    Replacement::draft_throws(0x14218de80, "r_14218de80", tpl_46636::<0x40, 0x142645878, 0x10, 0x4, 0x50, 0x14049fb80, 0x14218de9f> as *const (), Throws::Discard), // r::internal::ContinuationImpl<platform::storage::handleReadError::<lambda_1>,?bref6::Expected<platform::storage::HandleE
    Replacement::draft(0x14218df30, "r_14218df30", tpl_420::<0x38, 0x20> as *const ()), // sub_14218df30
    Replacement::draft(0x14218df70, "r_14218df70", tpl_46637::<0x38> as *const ()), // sub_14218df70
    Replacement::draft(0x14218dfb0, "r_14218dfb0", tpl_420::<0x38, 0x20> as *const ()), // sub_14218dfb0
    Replacement::draft(0x14218dff0, "r_14218dff0", tpl_46637::<0x38> as *const ()), // sub_14218dff0
    Replacement::draft_throws(0x14218e030, "r_14218e030", fn_14218e030 as *const (), Throws::Caught), // sub_14218e030
    Replacement::draft_throws(0x14218e0b0, "r_14218e0b0", fn_14218e0b0 as *const (), Throws::Discard), // sub_14218e0b0
    Replacement::draft(0x14218e8d0, "r_14218e8d0", tpl_46552::<0x30, 0x280> as *const ()), // sub_14218e8d0
    Replacement::draft_throws(0x14218e900, "r_14218e900", tpl_46639::<0xa0, 0x1412993b0> as *const (), Throws::Discard), // sub_14218e900
    Replacement::draft_throws(0x14218e920, "r_14218e920", fn_14218e920 as *const (), Throws::Discard), // sub_14218e920
    Replacement::draft_throws(0x14218e970, "r_14218e970", tpl_46639::<0x78, 0x1412984d0> as *const (), Throws::Discard), // sub_14218e970
    Replacement::draft_throws(0x14218e990, "r_14218e990", fn_14218e990 as *const (), Throws::Caught), // sub_14218e990
    Replacement::draft_throws(0x14218eca0, "r_14218eca0", fn_14218eca0 as *const (), Throws::Caught), // sub_14218eca0
    Replacement::draft_throws(0x14218ed50, "r_14218ed50", fn_14218ed50 as *const (), Throws::Discard), // sub_14218ed50
    Replacement::draft(0x14218edc0, "r_14218edc0", fn_14218edc0 as *const ()), // sub_14218edc0
    Replacement::draft_throws(0x14218ef70, "r_14218ef70", fn_14218ef70 as *const (), Throws::Caught), // sub_14218ef70
    Replacement::draft(0x14218f370, "r_14218f370", fn_14218f370 as *const ()), // sub_14218f370
    Replacement::draft(0x14218f3c0, "r_14218f3c0", tpl_46646::<0x1412996a0, 0x30, 0x280, 0xc, 0x14129ae10> as *const ()), // sub_14218f3c0
    Replacement::draft_throws(0x14218f4a0, "r_14218f4a0", fn_14218f4a0 as *const (), Throws::Caught), // sub_14218f4a0
    Replacement::draft(0x14218f600, "r_14218f600", tpl_46646::<0x14128fc90, 0x30, 0x280, 0xc, 0x14129bb20> as *const ()), // sub_14218f600
    Replacement::draft_throws(0x14218f6e0, "r_14218f6e0", fn_14218f6e0 as *const (), Throws::Discard), // sub_14218f6e0
    Replacement::draft(0x14218f860, "r_14218f860", fn_14218f860 as *const ()), // sub_14218f860
    Replacement::draft(0x14218f950, "r_14218f950", fn_14218f950 as *const ()), // sub_14218f950
    Replacement::draft_throws(0x14218fa10, "r_14218fa10", fn_14218fa10 as *const (), Throws::Caught), // sub_14218fa10
    Replacement::draft(0x14218fa80, "r_14218fa80", fn_14218fa80 as *const ()), // sub_14218fa80
    Replacement::draft(0x14218fdd0, "r_14218fdd0", fn_14218fdd0 as *const ()), // sub_14218fdd0
    Replacement::draft_throws(0x14218fe90, "r_14218fe90", fn_14218fe90 as *const (), Throws::Caught), // sub_14218fe90
    Replacement::draft(0x142190390, "r_142190390", fn_142190390 as *const ()), // sub_142190390
    Replacement::draft(0x142190410, "r_142190410", fn_142190410 as *const ()), // sub_142190410
    Replacement::draft(0x1421904a0, "r_1421904a0", fn_1421904a0 as *const ()), // sub_1421904a0
    Replacement::draft(0x1421905a0, "r_1421905a0", fn_1421905a0 as *const ()), // sub_1421905a0
    Replacement::draft(0x1421905d0, "r_1421905d0", fn_1421905d0 as *const ()), // sub_1421905d0
    Replacement::draft(0x142190660, "r_142190660", fn_142190660 as *const ()), // sub_142190660
    Replacement::draft_throws(0x1421906e0, "r_1421906e0", fn_1421906e0 as *const (), Throws::Caught), // sub_1421906e0
    Replacement::draft(0x142190d80, "r_142190d80", tpl_1149::<0x8, 0x1412a7f30, 0x1401e66f0> as *const ()), // sub_142190d80
    Replacement::draft(0x142190db0, "r_142190db0", fn_142190db0 as *const ()), // sub_142190db0
    Replacement::draft(0x142190e00, "r_142190e00", fn_142190e00 as *const ()), // sub_142190e00
    Replacement::draft(0x142190e60, "r_142190e60", fn_142190e60 as *const ()), // sub_142190e60
    Replacement::draft(0x142190ec0, "r_142190ec0", fn_142190ec0 as *const ()), // sub_142190ec0
    Replacement::draft(0x142190fa0, "r_142190fa0", fn_142190fa0 as *const ()), // sub_142190fa0
    Replacement::draft(0x142191030, "r_142191030", fn_142191030 as *const ()), // sub_142191030
    Replacement::draft(0x1421910b0, "r_1421910b0", fn_1421910b0 as *const ()), // sub_1421910b0
    Replacement::draft(0x142191150, "r_142191150", fn_142191150 as *const ()), // sub_142191150
    Replacement::draft(0x1421911a0, "r_1421911a0", fn_1421911a0 as *const ()), // sub_1421911a0
    Replacement::draft(0x1421911d0, "r_1421911d0", fn_1421911d0 as *const ()), // sub_1421911d0
    Replacement::draft(0x142191250, "r_142191250", fn_142191250 as *const ()), // sub_142191250
    Replacement::draft(0x142191320, "r_142191320", fn_142191320 as *const ()), // sub_142191320
    Replacement::draft(0x142191530, "r_142191530", fn_142191530 as *const ()), // sub_142191530
    Replacement::draft(0x142191570, "r_142191570", fn_142191570 as *const ()), // sub_142191570
    Replacement::draft(0x142191620, "r_142191620", fn_142191620 as *const ()), // sub_142191620
    Replacement::draft(0x1421916a0, "r_1421916a0", fn_1421916a0 as *const ()), // sub_1421916a0
    Replacement::draft(0x1421916c0, "r_1421916c0", fn_1421916c0 as *const ()), // sub_1421916c0
    Replacement::draft(0x142191c30, "r_142191c30", fn_142191c30 as *const ()), // sub_142191c30
    Replacement::draft(0x142191d00, "r_142191d00", fn_142191d00 as *const ()), // sub_142191d00
    Replacement::draft(0x142191d70, "r_142191d70", fn_142191d70 as *const ()), // sub_142191d70
    Replacement::draft(0x142191e10, "r_142191e10", fn_142191e10 as *const ()), // sub_142191e10
    Replacement::draft(0x142191e60, "r_142191e60", fn_142191e60 as *const ()), // sub_142191e60
    Replacement::draft(0x142191ee0, "r_142191ee0", fn_142191ee0 as *const ()), // sub_142191ee0
    Replacement::draft(0x142191f10, "r_142191f10", fn_142191f10 as *const ()), // sub_142191f10
    Replacement::draft(0x142191fe0, "r_142191fe0", fn_142191fe0 as *const ()), // sub_142191fe0
    Replacement::draft(0x142192040, "r_142192040", fn_142192040 as *const ()), // sub_142192040
    Replacement::draft(0x142192130, "r_142192130", fn_142192130 as *const ()), // sub_142192130
    Replacement::draft(0x142192180, "r_142192180", fn_142192180 as *const ()), // sub_142192180
    Replacement::draft(0x142192490, "r_142192490", fn_142192490 as *const ()), // sub_142192490
    Replacement::draft(0x1421924b0, "r_1421924b0", fn_1421924b0 as *const ()), // sub_1421924b0
    Replacement::draft(0x142192520, "r_142192520", fn_142192520 as *const ()), // sub_142192520
    Replacement::draft(0x1421925b0, "r_1421925b0", fn_1421925b0 as *const ()), // sub_1421925b0
    Replacement::draft(0x1421926d0, "r_1421926d0", fn_1421926d0 as *const ()), // sub_1421926d0
    Replacement::draft(0x142192700, "r_142192700", fn_142192700 as *const ()), // sub_142192700
    Replacement::draft_throws(0x142192730, "r_142192730", fn_142192730 as *const (), Throws::Discard), // sub_142192730
    Replacement::draft(0x142192780, "r_142192780", fn_142192780 as *const ()), // sub_142192780
    Replacement::draft(0x142192810, "r_142192810", fn_142192810 as *const ()), // sub_142192810
    Replacement::draft(0x142192a70, "r_142192a70", fn_142192a70 as *const ()), // sub_142192a70
    Replacement::draft(0x142193110, "r_142193110", fn_142193110 as *const ()), // sub_142193110
    Replacement::draft_throws(0x142193130, "r_142193130", fn_142193130 as *const (), Throws::Caught), // sub_142193130
    Replacement::draft(0x142193200, "r_142193200", fn_142193200 as *const ()), // sub_142193200
    Replacement::draft(0x142193250, "r_142193250", fn_142193250 as *const ()), // sub_142193250
    Replacement::draft_throws(0x1421932e0, "r_1421932e0", fn_1421932e0 as *const (), Throws::Caught), // sub_1421932e0
    Replacement::draft(0x1421934a0, "r_1421934a0", fn_1421934a0 as *const ()), // sub_1421934a0
    Replacement::draft(0x142193540, "r_142193540", tpl_650::<0x8> as *const ()), // sub_142193540
    Replacement::draft(0x142193550, "r_142193550", tpl_524::<0x8> as *const ()), // sub_142193550
    Replacement::draft(0x1421935c0, "r_1421935c0", fn_1421935c0 as *const ()), // sub_1421935c0
    Replacement::draft(0x142193690, "r_142193690", fn_142193690 as *const ()), // sub_142193690
    Replacement::draft(0x1421936e0, "r_1421936e0", tpl_46705::<0x168, 0xa78, 0x8, 0x1412a8260, 0x1412a8100, 0x9, 0x1412a8330, 0x14240ab10> as *const ()), // sub_1421936e0
    Replacement::draft(0x142193780, "r_142193780", tpl_46705::<0x160, 0xa78, 0x8, 0x1412a8260, 0x1412a8100, 0x9, 0x1412a8330, 0x14240ab10> as *const ()), // sub_142193780
    Replacement::draft(0x142193820, "r_142193820", fn_142193820 as *const ()), // sub_142193820
    Replacement::draft_throws(0x142193840, "r_142193840", fn_142193840 as *const (), Throws::Discard), // sub_142193840
    Replacement::draft(0x142193890, "r_142193890", fn_142193890 as *const ()), // sub_142193890
    Replacement::draft_throws(0x142193a20, "r_142193a20", fn_142193a20 as *const (), Throws::Discard), // sub_142193a20
    Replacement::draft_throws(0x142193c20, "r_142193c20", crate::shards::subsystems::fn_142193c20 as *const (), Throws::Caught), // sub_142193c20
    Replacement::draft(0x142193c90, "r_142193c90", fn_142193c90 as *const ()), // sub_142193c90
    Replacement::draft(0x142193d00, "r_142193d00", fn_142193d00 as *const ()), // sub_142193d00
    Replacement::draft(0x142193d50, "r_142193d50", tpl_37328::<0x954> as *const ()), // sub_142193d50
    Replacement::draft(0x142193d70, "r_142193d70", fn_142193d70 as *const ()), // sub_142193d70
    Replacement::draft(0x142193e50, "r_142193e50", fn_142193e50 as *const ()), // sub_142193e50
    Replacement::draft(0x142193eb0, "r_142193eb0", fn_142193eb0 as *const ()), // sub_142193eb0
    Replacement::draft(0x142193f00, "r_142193f00", fn_142193f00 as *const ()), // sub_142193f00
    Replacement::draft(0x142193fa0, "r_142193fa0", tpl_37328::<0x951> as *const ()), // sub_142193fa0
    Replacement::draft(0x142193fb0, "r_142193fb0", tpl_650::<0x8> as *const ()), // sub_142193fb0
    Replacement::draft(0x142193fc0, "r_142193fc0", fn_142193fc0 as *const ()), // sub_142193fc0
    Replacement::draft(0x142194000, "r_142194000", fn_142194000 as *const ()), // sub_142194000
    Replacement::draft(0x142194160, "r_142194160", fn_142194160 as *const ()), // sub_142194160
    Replacement::draft(0x142194190, "r_142194190", tpl_36233 as *const ()), // sub_142194190
    Replacement::draft(0x1421941b0, "r_1421941b0", fn_1421941b0 as *const ()), // sub_1421941b0
    Replacement::draft(0x142194250, "r_142194250", fn_142194250 as *const ()), // sub_142194250
    Replacement::draft(0x142194440, "r_142194440", fn_142194440 as *const ()), // sub_142194440
    Replacement::draft_throws(0x1421944f0, "r_1421944f0", fn_1421944f0 as *const (), Throws::Discard), // sub_1421944f0
    Replacement::draft_throws(0x142194620, "r_142194620", fn_142194620 as *const (), Throws::Caught), // sub_142194620
    Replacement::draft_throws(0x142194690, "r_142194690", fn_142194690 as *const (), Throws::Caught), // sub_142194690
    Replacement::draft(0x1421948d0, "r_1421948d0", fn_1421948d0 as *const ()), // sub_1421948d0
    Replacement::draft_throws(0x142194950, "r_142194950", fn_142194950 as *const (), Throws::Discard), // sub_142194950
    Replacement::draft(0x142194a30, "r_142194a30", fn_142194a30 as *const ()), // sub_142194a30
    Replacement::draft_throws(0x142194a50, "r_142194a50", fn_142194a50 as *const (), Throws::Discard), // sub_142194a50
    Replacement::draft(0x142194b80, "r_142194b80", fn_142194b80 as *const ()), // sub_142194b80
    Replacement::draft(0x142194c30, "r_142194c30", fn_142194c30 as *const ()), // sub_142194c30
    Replacement::draft(0x142194cf0, "r_142194cf0", fn_142194cf0 as *const ()), // sub_142194cf0
    Replacement::draft(0x142194fb0, "r_142194fb0", fn_142194fb0 as *const ()), // sub_142194fb0
    Replacement::draft(0x142194ff0, "r_142194ff0", fn_142194ff0 as *const ()), // sub_142194ff0
    Replacement::draft_throws(0x142195020, "r_142195020", fn_142195020 as *const (), Throws::Discard), // sub_142195020
    Replacement::draft_throws(0x142195190, "r_142195190", fn_142195190 as *const (), Throws::Discard), // sub_142195190
    Replacement::draft_throws(0x142195210, "r_142195210", fn_142195210 as *const (), Throws::Discard), // sub_142195210
    Replacement::draft(0x142195300, "r_142195300", tpl_37328::<0x952> as *const ()), // sub_142195300
    Replacement::draft_throws(0x142195310, "r_142195310", fn_142195310 as *const (), Throws::Discard), // sub_142195310
    Replacement::draft(0x142195370, "r_142195370", fn_142195370 as *const ()), // sub_142195370
    Replacement::draft_throws(0x142195500, "r_142195500", fn_142195500 as *const (), Throws::Discard), // sub_142195500
    Replacement::draft_throws(0x1421959c0, "r_1421959c0", crate::shards::subsystems::fn_1421959c0 as *const (), Throws::Caught), // sub_1421959c0
    Replacement::draft(0x142195a30, "r_142195a30", tpl_46735::<0x50, 0x18, 0x1412a9990> as *const ()), // sub_142195a30
    Replacement::draft(0x142195a70, "r_142195a70", tpl_46735::<0x50, 0x18, 0x1412a9bd0> as *const ()), // sub_142195a70
    Replacement::draft(0x142195ab0, "r_142195ab0", fn_142195ab0 as *const ()), // sub_142195ab0
    Replacement::draft(0x142195b00, "r_142195b00", fn_142195b00 as *const ()), // sub_142195b00
    Replacement::draft(0x142195b30, "r_142195b30", fn_142195b30 as *const ()), // sub_142195b30
    Replacement::draft(0x142195bc0, "r_142195bc0", fn_142195bc0 as *const ()), // sub_142195bc0
    Replacement::draft_throws(0x142196290, "r_142196290", fn_142196290 as *const (), Throws::Discard), // sub_142196290
    Replacement::draft(0x142196760, "r_142196760", fn_142196760 as *const ()), // sub_142196760
    Replacement::draft(0x1421967f0, "r_1421967f0", fn_1421967f0 as *const ()), // sub_1421967f0
    Replacement::draft_throws(0x142196820, "r_142196820", tpl_46743::<0x142196290> as *const (), Throws::Discard), // sub_142196820
    Replacement::draft(0x142196840, "r_142196840", tpl_46744::<0x142646e58> as *const ()), // sub_142196840
    Replacement::draft(0x142196870, "r_142196870", fn_142196870 as *const ()), // sub_142196870
    Replacement::draft_throws(0x1421968d0, "r_1421968d0", fn_1421968d0 as *const (), Throws::Discard), // sub_1421968d0
    Replacement::draft(0x142196b40, "r_142196b40", tpl_37328::<0xa63> as *const ()), // sub_142196b40
    Replacement::draft_throws(0x142196b50, "r_142196b50", fn_142196b50 as *const (), Throws::Discard), // sub_142196b50
    Replacement::draft(0x142196ba0, "r_142196ba0", fn_142196ba0 as *const ()), // sub_142196ba0
    Replacement::draft_throws(0x142196d30, "r_142196d30", fn_142196d30 as *const (), Throws::Discard), // sub_142196d30
    Replacement::draft_throws(0x142196e50, "r_142196e50", fn_142196e50 as *const (), Throws::Caught), // sub_142196e50
    Replacement::draft(0x142196ec0, "r_142196ec0", tpl_35558::<0x4> as *const ()), // sub_142196ec0
    Replacement::draft(0x142196f20, "r_142196f20", fn_142196f20 as *const ()), // sub_142196f20
    Replacement::draft_throws(0x142196f80, "r_142196f80", fn_142196f80 as *const (), Throws::Discard), // sub_142196f80
    Replacement::draft_throws(0x142197080, "r_142197080", fn_142197080 as *const (), Throws::Caught), // sub_142197080
    Replacement::draft(0x1421970f0, "r_1421970f0", fn_1421970f0 as *const ()), // sub_1421970f0
    Replacement::draft(0x142197150, "r_142197150", tpl_2879::<0x1401f3d00, 0x8, 0x142198400, 0x1433e9cc8, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::internal::fulfillPromise<r::Expected<platform::user::epic::anonymous_namespace::LoginIn
    Replacement::draft(0x1421971b0, "r_1421971b0", tpl_41196::<0x1433e9e30, 0x8, 0x10, 0x9, 0x11, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1421971b0
    Replacement::draft(0x142197210, "r_142197210", tpl_41197::<0x1401f3d00, 0x8, 0x142198400, 0x1433e9cc8> as *const ()), // sub_142197210
    Replacement::draft(0x142197250, "r_142197250", tpl_41197::<0x1401f3d00, 0x8, 0x142198400, 0x1433e9cc8> as *const ()), // sub_142197250
    Replacement::draft(0x142197290, "r_142197290", tpl_2879::<0x1401f3d00, 0x8, 0x142198400, 0x1433e9cc8, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<platform::user::epic::anonymous_namespace::Log
    Replacement::draft(0x1421972f0, "r_1421972f0", tpl_41198::<0x1433e9e30, 0x8, 0x10, 0x9, 0x11, 0x1401f3d90, 0x20, 0x28, 0x1433e9de8> as *const ()), // sub_1421972f0
    Replacement::draft(0x142197370, "r_142197370", fn_142197370 as *const ()), // sub_142197370
    Replacement::draft(0x1421973c0, "r_1421973c0", tpl_41200 as *const ()), // sub_1421973c0
    Replacement::draft(0x142197490, "r_142197490", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_142197490
    Replacement::draft(0x1421974b0, "r_1421974b0", tpl_23501::<0x11, 0x8, 0x1412ad320, 0x20, 0x1401f3ee0> as *const ()), // r::internal::VoidContinuationImpl<r::internal::fulfillPromise<r::Expected<platform::user::epic::anonymous_namespace::Log
    Replacement::draft(0x142197580, "r_142197580", tpl_41201::<0x9, 0x1412ad320> as *const ()), // sub_142197580
    Replacement::draft(0x1421977a0, "r_1421977a0", tpl_35426::<0x1433e9d28, 0x8, 0x1401f4c80, 0x20, 0x28> as *const ()), // sub_1421977a0
    Replacement::draft(0x1421977e0, "r_1421977e0", tpl_35426::<0x1433e9e18, 0x8, 0x1412ac640, 0x20, 0x28> as *const ()), // sub_1421977e0
    Replacement::draft(0x142197820, "r_142197820", tpl_35426::<0x1433e9e48, 0x8, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_142197820
    Replacement::draft(0x142197860, "r_142197860", tpl_34372::<0x1433e9d70, 0x8, 0x1401f4c80, 0x18, 0x20> as *const ()), // sub_142197860
    Replacement::draft(0x1421978a0, "r_1421978a0", tpl_34372::<0x1433e9db8, 0x8, 0x1412ac640, 0x18, 0x20> as *const ()), // sub_1421978a0
    Replacement::draft(0x1421978e0, "r_1421978e0", tpl_35426::<0x1433e9d88, 0x8, 0x1401f4c80, 0x20, 0x28> as *const ()), // sub_1421978e0
    Replacement::draft(0x142197d40, "r_142197d40", fn_142197d40 as *const ()), // sub_142197d40
    Replacement::draft(0x142197d70, "r_142197d70", tpl_35429::<0x1433e9d28, 0x8, 0x1401f4c80, 0x20, 0x28, 0x1433e9d10> as *const ()), // sub_142197d70
    Replacement::draft(0x142197dc0, "r_142197dc0", tpl_35429::<0x1433e9e18, 0x8, 0x1412ac640, 0x20, 0x28, 0x1433e9d40> as *const ()), // sub_142197dc0
    Replacement::draft(0x142197e10, "r_142197e10", tpl_35429::<0x1433e9e48, 0x8, 0x1401f3d90, 0x20, 0x28, 0x1433e9e60> as *const ()), // sub_142197e10
    Replacement::draft(0x142197e60, "r_142197e60", tpl_34373::<0x1433e9d70, 0x8, 0x1401f4c80, 0x18, 0x20, 0x1433e9e00> as *const ()), // sub_142197e60
    Replacement::draft(0x142197eb0, "r_142197eb0", tpl_34373::<0x1433e9db8, 0x8, 0x1412ac640, 0x18, 0x20, 0x1433e9d58> as *const ()), // sub_142197eb0
    Replacement::draft(0x142197f00, "r_142197f00", tpl_35429::<0x1433e9d88, 0x8, 0x1401f4c80, 0x20, 0x28, 0x1433e9cb0> as *const ()), // sub_142197f00
    Replacement::draft(0x1421980d0, "r_1421980d0", tpl_28335::<0x20> as *const ()), // sub_1421980d0
    Replacement::draft(0x1421980e0, "r_1421980e0", tpl_41213::<0x8, 0x20> as *const ()), // sub_1421980e0
    Replacement::draft(0x142198120, "r_142198120", tpl_41214::<0x18, 0x20, 0x28, 0x48> as *const ()), // sub_142198120
    Replacement::draft(0x142198150, "r_142198150", tpl_725::<0x20, 0x28, 0x8, 0x142442de0> as *const ()), // sub_142198150
    Replacement::draft(0x142198330, "r_142198330", tpl_35117::<0x20, 0x28, 0x8, 0x10> as *const ()), // sub_142198330
    Replacement::draft(0x142198350, "r_142198350", fn_142198350 as *const ()), // r::internal::ContinuationImpl<platform::user::epic::anonymous_namespace::addUser::<lambda_1>,?bref8::Expected<platform::
    Replacement::draft(0x1421983d0, "r_1421983d0", tpl_733::<0x1412ac640, 0x8> as *const ()), // sub_1421983d0
    Replacement::draft(0x142198400, "r_142198400", tpl_725::<0x20, 0x28, 0x8, 0x142442de0> as *const ()), // sub_142198400
    Replacement::draft(0x142198470, "r_142198470", tpl_41217::<0x8, 0x20> as *const ()), // sub_142198470
    Replacement::draft(0x142198490, "r_142198490", tpl_4771::<0x8, 0x142198400, 0x142442de0> as *const ()), // sub_142198490
    Replacement::draft(0x1421984f0, "r_1421984f0", tpl_4771::<0x8, 0x1412ac5d0, 0x142442de0> as *const ()), // sub_1421984f0
    Replacement::draft(0x1421985d0, "r_1421985d0", fn_1421985d0 as *const ()), // sub_1421985d0
    Replacement::draft(0x1421985f0, "r_1421985f0", fn_1421985f0 as *const ()), // r::internal::ContinuationImpl<platform::user::epic::anonymous_namespace::connectLogin::<lambda_3>,_tagEOS_Connect_LoginC
    Replacement::draft(0x142198680, "r_142198680", tpl_4771::<0x8, 0x141290030, 0x142442de0> as *const ()), // sub_142198680
    Replacement::draft(0x142198760, "r_142198760", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_142198760
    Replacement::draft_throws(0x142198780, "r_142198780", fn_142198780 as *const (), Throws::Discard), // r::internal::VoidContinuationImpl<platform::user::epic::anonymous_namespace::onAuthExpiration::<lambda_2>,r::Expected<pl
    Replacement::draft(0x142198850, "r_142198850", tpl_35117::<0x18, 0x20, 0x8, 0x10> as *const ()), // sub_142198850
    Replacement::draft(0x142198870, "r_142198870", fn_142198870 as *const ()), // r::internal::ContinuationImpl<platform::user::epic::requestSignIn::<lambda_1>,?bref7::Expected<platform::User,platform::
    Replacement::draft(0x142198960, "r_142198960", fn_142198960 as *const ()), // sub_142198960
    Replacement::draft(0x142198a10, "r_142198a10", tpl_35117::<0x20, 0x28, 0x8, 0x10> as *const ()), // sub_142198a10
    Replacement::draft(0x142198a40, "r_142198a40", tpl_4771::<0x8, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_142198a40
    Replacement::draft(0x142198ab0, "r_142198ab0", fn_142198ab0 as *const ()), // sub_142198ab0
    Replacement::draft(0x142198b20, "r_142198b20", fn_142198b20 as *const ()), // sub_142198b20
    Replacement::draft(0x142198ba0, "r_142198ba0", fn_142198ba0 as *const ()), // sub_142198ba0
    Replacement::draft(0x142198cd0, "r_142198cd0", fn_142198cd0 as *const ()), // sub_142198cd0
    Replacement::draft(0x142198e10, "r_142198e10", tpl_46766::<0x1412900a0, 0x8, 0x10> as *const ()), // sub_142198e10
    Replacement::draft(0x142198e50, "r_142198e50", fn_142198e50 as *const ()), // sub_142198e50
    Replacement::draft(0x142198f70, "r_142198f70", tpl_46768::<0x141296150, 0x8, 0x10> as *const ()), // sub_142198f70
    Replacement::draft(0x142198fa0, "r_142198fa0", fn_142198fa0 as *const ()), // sub_142198fa0
    Replacement::draft(0x142199020, "r_142199020", fn_142199020 as *const ()), // sub_142199020
    Replacement::draft(0x1421990b0, "r_1421990b0", tpl_46766::<0x1412ac640, 0x8, 0x10> as *const ()), // sub_1421990b0
    Replacement::draft(0x1421991e0, "r_1421991e0", fn_1421991e0 as *const ()), // sub_1421991e0
    Replacement::draft(0x142199210, "r_142199210", fn_142199210 as *const ()), // sub_142199210
    Replacement::draft(0x142199250, "r_142199250", tpl_2042::<0x20, 0x28, 0x8, 0x142442de0> as *const ()), // sub_142199250
    Replacement::draft(0x1421992f0, "r_1421992f0", tpl_735::<0x8> as *const ()), // sub_1421992f0
    Replacement::draft(0x142199330, "r_142199330", tpl_725::<0x20, 0x28, 0x8, 0x142442de0> as *const ()), // sub_142199330
    Replacement::draft(0x142199430, "r_142199430", tpl_46773::<0x8, 0x142645168> as *const ()), // sub_142199430
    Replacement::draft(0x142199450, "r_142199450", tpl_46773::<0x8, 0x142645168> as *const ()), // sub_142199450
    Replacement::draft(0x142199470, "r_142199470", fn_142199470 as *const ()), // sub_142199470
    Replacement::draft(0x142199490, "r_142199490", fn_142199490 as *const ()), // sub_142199490
    Replacement::draft_throws(0x1421994d0, "r_1421994d0", fn_1421994d0 as *const (), Throws::Discard), // sub_1421994d0
    Replacement::draft(0x142199540, "r_142199540", fn_142199540 as *const ()), // sub_142199540
    Replacement::draft(0x1421995a0, "r_1421995a0", fn_1421995a0 as *const ()), // sub_1421995a0
    Replacement::draft_throws(0x142199620, "r_142199620", fn_142199620 as *const (), Throws::Caught), // sub_142199620
    Replacement::draft_throws(0x142199770, "r_142199770", fn_142199770 as *const (), Throws::Discard), // sub_142199770
    Replacement::draft_throws(0x1421997a0, "r_1421997a0", fn_1421997a0 as *const (), Throws::Caught), // sub_1421997a0
    Replacement::draft_throws(0x142199970, "r_142199970", fn_142199970 as *const (), Throws::Caught), // sub_142199970
    Replacement::draft(0x142199af0, "r_142199af0", fn_142199af0 as *const ()), // sub_142199af0
    Replacement::draft(0x142199c30, "r_142199c30", fn_142199c30 as *const ()), // sub_142199c30
    Replacement::draft(0x142199ce0, "r_142199ce0", fn_142199ce0 as *const ()), // sub_142199ce0
    Replacement::draft(0x142199d50, "r_142199d50", fn_142199d50 as *const ()), // sub_142199d50
    Replacement::draft_throws(0x142199df0, "r_142199df0", fn_142199df0 as *const (), Throws::Caught), // sub_142199df0
    Replacement::draft(0x142199fd0, "r_142199fd0", fn_142199fd0 as *const ()), // sub_142199fd0
    Replacement::draft(0x14219a0d0, "r_14219a0d0", fn_14219a0d0 as *const ()), // sub_14219a0d0
    Replacement::draft(0x14219a100, "r_14219a100", tpl_46789::<0x1426451f0> as *const ()), // sub_14219a100
    Replacement::draft(0x14219a120, "r_14219a120", fn_14219a120 as *const ()), // sub_14219a120
    Replacement::draft(0x14219a2f0, "r_14219a2f0", fn_14219a2f0 as *const ()), // sub_14219a2f0
    Replacement::draft(0x14219a320, "r_14219a320", fn_14219a320 as *const ()), // sub_14219a320
    Replacement::draft_throws(0x14219a3f0, "r_14219a3f0", fn_14219a3f0 as *const (), Throws::Caught), // sub_14219a3f0
    Replacement::draft(0x14219a8a0, "r_14219a8a0", tpl_46789::<0x142645180> as *const ()), // sub_14219a8a0
    Replacement::draft_throws(0x14219a8c0, "r_14219a8c0", fn_14219a8c0 as *const (), Throws::Caught), // sub_14219a8c0
    Replacement::draft(0x14219ac60, "r_14219ac60", fn_14219ac60 as *const ()), // sub_14219ac60
    Replacement::draft(0x14219ad00, "r_14219ad00", fn_14219ad00 as *const ()), // sub_14219ad00
    Replacement::draft(0x14219adc0, "r_14219adc0", fn_14219adc0 as *const ()), // sub_14219adc0
    Replacement::draft(0x14219ae70, "r_14219ae70", fn_14219ae70 as *const ()), // sub_14219ae70
    Replacement::draft(0x14219aea0, "r_14219aea0", tpl_46494::<0x1412907c0> as *const ()), // sub_14219aea0
    Replacement::draft_throws(0x14219aed0, "r_14219aed0", fn_14219aed0 as *const (), Throws::Caught), // sub_14219aed0
    Replacement::draft(0x14219b660, "r_14219b660", fn_14219b660 as *const ()), // sub_14219b660
    Replacement::draft(0x14219b6f0, "r_14219b6f0", fn_14219b6f0 as *const ()), // sub_14219b6f0
    Replacement::draft_throws(0x14219b730, "r_14219b730", fn_14219b730 as *const (), Throws::Caught), // sub_14219b730
    Replacement::draft(0x14219ba20, "r_14219ba20", fn_14219ba20 as *const ()), // sub_14219ba20
    Replacement::draft(0x14219bb50, "r_14219bb50", fn_14219bb50 as *const ()), // sub_14219bb50
    Replacement::draft(0x14219bb70, "r_14219bb70", fn_14219bb70 as *const ()), // sub_14219bb70
    Replacement::draft_throws(0x14219bba0, "r_14219bba0", fn_14219bba0 as *const (), Throws::Caught), // sub_14219bba0
    Replacement::draft(0x14219bf00, "r_14219bf00", fn_14219bf00 as *const ()), // sub_14219bf00
    Replacement::draft(0x14219bf60, "r_14219bf60", fn_14219bf60 as *const ()), // sub_14219bf60
    Replacement::draft_throws(0x14219c010, "r_14219c010", fn_14219c010 as *const (), Throws::Caught), // sub_14219c010
    Replacement::draft_throws(0x14219c100, "r_14219c100", fn_14219c100 as *const (), Throws::Caught), // sub_14219c100
    Replacement::draft_throws(0x14219c270, "r_14219c270", fn_14219c270 as *const (), Throws::Caught), // sub_14219c270
    Replacement::draft_throws(0x14219c4a0, "r_14219c4a0", fn_14219c4a0 as *const (), Throws::Caught), // sub_14219c4a0
    Replacement::draft_throws(0x14219c600, "r_14219c600", fn_14219c600 as *const (), Throws::Caught), // sub_14219c600
    Replacement::draft_throws(0x14219c6d0, "r_14219c6d0", crate::shards::subsystems::fn_14219c6d0 as *const (), Throws::Caught), // sub_14219c6d0
    Replacement::draft(0x14219c810, "r_14219c810", tpl_46624::<0x4> as *const ()), // sub_14219c810
    Replacement::draft_throws(0x14219c830, "r_14219c830", fn_14219c830 as *const (), Throws::Caught), // sub_14219c830
    Replacement::draft_throws(0x14219c990, "r_14219c990", crate::shards::subsystems::fn_14219c990 as *const (), Throws::Caught), // sub_14219c990
    Replacement::draft_throws(0x14219ca60, "r_14219ca60", fn_14219ca60 as *const (), Throws::Caught), // sub_14219ca60
    Replacement::draft_throws(0x14219cb70, "r_14219cb70", crate::shards::subsystems::fn_14219cb70 as *const (), Throws::Caught), // sub_14219cb70
    Replacement::draft_throws(0x14219ccc0, "r_14219ccc0", crate::shards::subsystems::fn_14219ccc0 as *const (), Throws::Caught), // sub_14219ccc0
    Replacement::draft_throws(0x14219cd90, "r_14219cd90", fn_14219cd90 as *const (), Throws::Caught), // sub_14219cd90
    Replacement::draft(0x14219cf50, "r_14219cf50", tpl_46819::<0x1412a8100, 0x160> as *const ()), // sub_14219cf50
    Replacement::draft(0x14219cf80, "r_14219cf80", tpl_46819::<0x1412a8100, 0x160> as *const ()), // sub_14219cf80
    Replacement::draft(0x14219d030, "r_14219d030", fn_14219d030 as *const ()), // sub_14219d030
    Replacement::draft(0x14219d0d0, "r_14219d0d0", fn_14219d0d0 as *const ()), // sub_14219d0d0
    Replacement::draft(0x14219d150, "r_14219d150", fn_14219d150 as *const ()), // sub_14219d150
    Replacement::draft(0x14219d1a0, "r_14219d1a0", fn_14219d1a0 as *const ()), // sub_14219d1a0
    Replacement::draft_throws(0x14219d1d0, "r_14219d1d0", tpl_46743::<0x142196290> as *const (), Throws::Discard), // sub_14219d1d0
    Replacement::draft(0x14219d1f0, "r_14219d1f0", tpl_46744::<0x142646e58> as *const ()), // sub_14219d1f0
    Replacement::draft(0x14219d220, "r_14219d220", tpl_40276::<0x14020b7e0> as *const ()), // sub_14219d220
    Replacement::draft(0x14219d260, "r_14219d260", tpl_4771::<0x18, 0x14020b7e0, 0x142442de0> as *const ()), // sub_14219d260
    Replacement::draft_throws(0x14219d2c0, "r_14219d2c0", fn_14219d2c0 as *const (), Throws::Caught), // sub_14219d2c0
    Replacement::draft_throws(0x14219d320, "r_14219d320", fn_14219d320 as *const (), Throws::Caught), // sub_14219d320
    Replacement::draft(0x14219d380, "r_14219d380", fn_14219d380 as *const ()), // sub_14219d380
    Replacement::draft(0x14219d3c0, "r_14219d3c0", tpl_35558::<0x4> as *const ()), // sub_14219d3c0
    Replacement::draft(0x14219d3e0, "r_14219d3e0", fn_14219d3e0 as *const ()), // sub_14219d3e0
    Replacement::draft(0x14219d400, "r_14219d400", fn_14219d400 as *const ()), // sub_14219d400
    Replacement::draft(0x14219d420, "r_14219d420", fn_14219d420 as *const ()), // sub_14219d420
    Replacement::draft(0x14219d440, "r_14219d440", tpl_46830::<0x4> as *const ()), // sub_14219d440
    Replacement::draft(0x14219d460, "r_14219d460", fn_14219d460 as *const ()), // sub_14219d460
    Replacement::draft(0x14219d5a0, "r_14219d5a0", tpl_46830::<0x10> as *const ()), // sub_14219d5a0
    Replacement::draft(0x14219d5c0, "r_14219d5c0", fn_14219d5c0 as *const ()), // sub_14219d5c0
    Replacement::draft(0x14219d600, "r_14219d600", fn_14219d600 as *const ()), // sub_14219d600
    Replacement::draft_throws(0x14219d640, "r_14219d640", fn_14219d640 as *const (), Throws::Caught), // sub_14219d640
    Replacement::draft_throws(0x14219d800, "r_14219d800", fn_14219d800 as *const (), Throws::Caught), // sub_14219d800
    Replacement::draft_throws(0x14219d8a0, "r_14219d8a0", fn_14219d8a0 as *const (), Throws::Caught), // sub_14219d8a0
    Replacement::draft_throws(0x14219daa0, "r_14219daa0", fn_14219daa0 as *const (), Throws::Caught), // sub_14219daa0
    Replacement::draft_throws(0x14219dd10, "r_14219dd10", fn_14219dd10 as *const (), Throws::Caught), // sub_14219dd10
    Replacement::draft(0x14219dec0, "r_14219dec0", fn_14219dec0 as *const ()), // sub_14219dec0
    Replacement::draft_throws(0x14219df00, "r_14219df00", fn_14219df00 as *const (), Throws::Caught), // sub_14219df00
    Replacement::draft_throws(0x14219e3e0, "r_14219e3e0", fn_14219e3e0 as *const (), Throws::Caught), // sub_14219e3e0
    Replacement::draft_throws(0x14219e7e0, "r_14219e7e0", fn_14219e7e0 as *const (), Throws::Caught), // sub_14219e7e0
    Replacement::draft_throws(0x14219ec20, "r_14219ec20", fn_14219ec20 as *const (), Throws::Caught), // sub_14219ec20
    Replacement::draft_throws(0x14219eda0, "r_14219eda0", fn_14219eda0 as *const (), Throws::Caught), // sub_14219eda0
    Replacement::draft_throws(0x14219eec0, "r_14219eec0", fn_14219eec0 as *const (), Throws::Caught), // sub_14219eec0
    Replacement::draft_throws(0x14219f0d0, "r_14219f0d0", fn_14219f0d0 as *const (), Throws::Discard), // sub_14219f0d0
    Replacement::draft_throws(0x14219f100, "r_14219f100", fn_14219f100 as *const (), Throws::Discard), // sub_14219f100
    Replacement::draft(0x14219f1c0, "r_14219f1c0", fn_14219f1c0 as *const ()), // sub_14219f1c0
    Replacement::draft(0x14219f340, "r_14219f340", fn_14219f340 as *const ()), // sub_14219f340
    Replacement::draft(0x14219f4c0, "r_14219f4c0", tpl_790::<0x14219fcd0, 0x141834d30> as *const ()), // r::internal::ContinuationData<platform::achievements::epic::anonymous_namespace::FutureCollector::collect::<lambda_1>,?b
    Replacement::draft(0x14219f5a0, "r_14219f5a0", tpl_46850::<0x8, 0x10, 0x18, 0x1c> as *const ()), // sub_14219f5a0
    Replacement::draft(0x14219f5e0, "r_14219f5e0", tpl_15002::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68, 0x141834d30> as *const ()), // r::internal::ContinuationData<platform::achievements::epic::anonymous_namespace::unlockAchievements::<lambda_1>,?bref8::
    Replacement::draft(0x14219f640, "r_14219f640", tpl_46850::<0x8, 0x10, 0x18, 0x1c> as *const ()), // sub_14219f640
    Replacement::draft(0x14219f680, "r_14219f680", tpl_15002::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68, 0x141834d30> as *const ()), // r::internal::ContinuationData<platform::achievements::epic::anonymous_namespace::unlockAchievements::<lambda_2>,?bref8::
    Replacement::draft(0x14219f7d0, "r_14219f7d0", fn_14219f7d0 as *const ()), // sub_14219f7d0
    Replacement::draft(0x14219f8b0, "r_14219f8b0", fn_14219f8b0 as *const ()), // sub_14219f8b0
    Replacement::draft(0x14219f910, "r_14219f910", tpl_34372::<0x1433ea040, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_14219f910
    Replacement::draft(0x14219f950, "r_14219f950", tpl_35426::<0x1433e9f20, 0x8, 0x1402bf830, 0x20, 0x28> as *const ()), // sub_14219f950
    Replacement::draft(0x14219f990, "r_14219f990", tpl_46853::<0x1433ea088, 0x8, 0x10, 0x18, 0x20, 0x1c, 0x24, 0x1402bf830, 0x30, 0x38> as *const ()), // sub_14219f990
    Replacement::draft(0x14219fa00, "r_14219fa00", tpl_46853::<0x1433e9ec0, 0x8, 0x10, 0x18, 0x20, 0x1c, 0x24, 0x1402bf830, 0x30, 0x38> as *const ()), // sub_14219fa00
    Replacement::draft(0x14219fa70, "r_14219fa70", tpl_34372::<0x1433e9f08, 0x8, 0x1402bf830, 0x18, 0x20> as *const ()), // sub_14219fa70
    Replacement::draft(0x14219fab0, "r_14219fab0", tpl_34372::<0x1433ea028, 0x8, 0x1402bf830, 0x18, 0x20> as *const ()), // sub_14219fab0
    Replacement::draft(0x14219faf0, "r_14219faf0", tpl_41205::<0x1433e9f68, 0x1402bf830, 0x10, 0x18> as *const ()), // sub_14219faf0
    Replacement::draft(0x14219fb30, "r_14219fb30", fn_14219fb30 as *const ()), // sub_14219fb30
    Replacement::draft(0x14219fb80, "r_14219fb80", tpl_35426::<0x1433ea0a0, 0x8, 0x1402bf830, 0x20, 0x28> as *const ()), // sub_14219fb80
    Replacement::draft(0x14219fc40, "r_14219fc40", tpl_1494::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_14219fc40
    Replacement::draft(0x14219fcd0, "r_14219fcd0", fn_14219fcd0 as *const ()), // sub_14219fcd0
    Replacement::draft(0x14219fd70, "r_14219fd70", tpl_790::<0x14219fcd0, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<platform::achievements::epic::anonymous_namespace::FutureCollector::collect::<lambda_1
    Replacement::draft(0x14219ff10, "r_14219ff10", tpl_46855::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68> as *const ()), // sub_14219ff10
    Replacement::draft(0x14219ff50, "r_14219ff50", tpl_46855::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68> as *const ()), // sub_14219ff50
    Replacement::draft(0x14219ff90, "r_14219ff90", tpl_15002::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<platform::achievements::epic::anonymous_namespace::unlockAchievements::<lambda_1>,?bref8::
    Replacement::draft(0x14219fff0, "r_14219fff0", tpl_46855::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68> as *const ()), // sub_14219fff0
    Replacement::draft(0x1421a0030, "r_1421a0030", tpl_46855::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68> as *const ()), // sub_1421a0030
    Replacement::draft(0x1421a0070, "r_1421a0070", tpl_15002::<0x1402bf7c0, 0x18, 0x140f366d0, 0x1430b8a68, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<platform::achievements::epic::anonymous_namespace::unlockAchievements::<lambda_2>,?bref8::
    Replacement::draft(0x1421a0fa0, "r_1421a0fa0", fn_1421a0fa0 as *const ()), // r::internal::VoidContinuationImpl<platform::achievements::epic::anonymous_namespace::FutureCollector::collect::<lambda_1
    Replacement::draft(0x1421a0fe0, "r_1421a0fe0", tpl_4771::<0x8, 0x1412b0930, 0x142442de0> as *const ()), // sub_1421a0fe0
    Replacement::draft(0x1421a10c0, "r_1421a10c0", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_1421a10c0
    Replacement::draft(0x1421a1110, "r_1421a1110", tpl_4771::<0x8, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a1110
    Replacement::draft(0x1421a11f0, "r_1421a11f0", tpl_35117::<0x20, 0x28, 0x8, 0xc> as *const ()), // sub_1421a11f0
    Replacement::draft_throws(0x1421a1210, "r_1421a1210", fn_1421a1210 as *const (), Throws::Discard), // r::internal::ContinuationImpl<platform::achievements::epic::anonymous_namespace::loadPlayerAchievements::<lambda_1>,?bre
    Replacement::draft(0x1421a1260, "r_1421a1260", tpl_4771::<0x8, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a1260
    Replacement::draft(0x1421a12c0, "r_1421a12c0", tpl_4771::<0x8, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a12c0
    Replacement::draft(0x1421a13a0, "r_1421a13a0", tpl_35117::<0x30, 0x38, 0x8, 0xc> as *const ()), // sub_1421a13a0
    Replacement::draft_throws(0x1421a13c0, "r_1421a13c0", tpl_46864::<0x4, 0x1421a3340, 0x30, 0x39, 0x14128fd40> as *const (), Throws::Discard), // r::internal::ContinuationImpl<platform::achievements::epic::anonymous_namespace::unlockAchievements::<lambda_1>,?bref8::
    Replacement::draft(0x1421a1490, "r_1421a1490", tpl_35117::<0x30, 0x38, 0x8, 0xc> as *const ()), // sub_1421a1490
    Replacement::draft_throws(0x1421a14b0, "r_1421a14b0", tpl_46864::<0x4, 0x1421a30c0, 0x30, 0x39, 0x14128fd40> as *const (), Throws::Discard), // r::internal::ContinuationImpl<platform::achievements::epic::anonymous_namespace::unlockAchievements::<lambda_2>,?bref8::
    Replacement::draft(0x1421a1580, "r_1421a1580", tpl_35117::<0x18, 0x20, 0x8, 0xc> as *const ()), // sub_1421a1580
    Replacement::draft_throws(0x1421a15a0, "r_1421a15a0", crate::shards::subsystems::fn_1421a15a0 as *const (), Throws::Caught), // r::internal::ContinuationImpl<platform::achievements::epic::reportRateLimitedProgressIfNecessary::<lambda_1>,?bref7::?br
    Replacement::draft(0x1421a16c0, "r_1421a16c0", tpl_35117::<0x18, 0x20, 0x8, 0xc> as *const ()), // sub_1421a16c0
    Replacement::draft(0x1421a16e0, "r_1421a16e0", fn_1421a16e0 as *const ()), // r::internal::ContinuationImpl<platform::achievements::epic::reportRateLimitedProgressIfNecessary::<lambda_2>,?bref7::?br
    Replacement::draft(0x1421a17e0, "r_1421a17e0", tpl_35117::<0x10, 0x18, 0x8, 0xc> as *const ()), // sub_1421a17e0
    Replacement::draft(0x1421a1800, "r_1421a1800", fn_1421a1800 as *const ()), // r::internal::ContinuationImpl<platform::achievements::epic::restoreProgress::<lambda_1>,?bref7::Expected<void,platform::
    Replacement::draft(0x1421a18c0, "r_1421a18c0", tpl_35117::<0x20, 0x28, 0x8, 0xc> as *const ()), // sub_1421a18c0
    Replacement::draft(0x1421a18e0, "r_1421a18e0", fn_1421a18e0 as *const ()), // r::internal::ContinuationImpl<platform::achievements::epic::restoreProgress::<lambda_2>::operator()::<lambda_1>,?bref8::
    Replacement::draft(0x1421a19e0, "r_1421a19e0", tpl_35117::<0x20, 0x28, 0x8, 0xc> as *const ()), // sub_1421a19e0
    Replacement::draft_throws(0x1421a1a00, "r_1421a1a00", crate::shards::subsystems::fn_1421a1a00 as *const (), Throws::Caught), // r::internal::ContinuationImpl<platform::achievements::epic::restoreProgress::<lambda_2>,?bref7::Expected<void,platform::
    Replacement::draft(0x1421a68b0, "r_1421a68b0", fn_1421a68b0 as *const ()), // sub_1421a68b0
    Replacement::draft(0x1421a6930, "r_1421a6930", fn_1421a6930 as *const ()), // sub_1421a6930
    Replacement::draft(0x1421a6a00, "r_1421a6a00", fn_1421a6a00 as *const ()), // sub_1421a6a00
    Replacement::draft_throws(0x1421a6a80, "r_1421a6a80", fn_1421a6a80 as *const (), Throws::Caught), // sub_1421a6a80
    Replacement::draft(0x1421a6ea0, "r_1421a6ea0", fn_1421a6ea0 as *const ()), // sub_1421a6ea0
    Replacement::draft_throws(0x1421a6f30, "r_1421a6f30", fn_1421a6f30 as *const (), Throws::Caught), // sub_1421a6f30
    Replacement::draft_throws(0x1421a7890, "r_1421a7890", fn_1421a7890 as *const (), Throws::Caught), // sub_1421a7890
    Replacement::draft(0x1421a7e80, "r_1421a7e80", fn_1421a7e80 as *const ()), // sub_1421a7e80
    Replacement::draft_throws(0x1421a80d0, "r_1421a80d0", fn_1421a80d0 as *const (), Throws::Caught), // sub_1421a80d0
    Replacement::draft_throws(0x1421a8790, "r_1421a8790", fn_1421a8790 as *const (), Throws::Caught), // sub_1421a8790
    Replacement::draft(0x1421a8f70, "r_1421a8f70", fn_1421a8f70 as *const ()), // sub_1421a8f70
    Replacement::draft(0x1421a91c0, "r_1421a91c0", fn_1421a91c0 as *const ()), // sub_1421a91c0
    Replacement::draft_throws(0x1421a9550, "r_1421a9550", fn_1421a9550 as *const (), Throws::Caught), // sub_1421a9550
    Replacement::draft_throws(0x1421a9da0, "r_1421a9da0", fn_1421a9da0 as *const (), Throws::Caught), // sub_1421a9da0
    Replacement::draft_throws(0x1421aa190, "r_1421aa190", fn_1421aa190 as *const (), Throws::Caught), // sub_1421aa190
    Replacement::draft_throws(0x1421aa580, "r_1421aa580", fn_1421aa580 as *const (), Throws::Caught), // sub_1421aa580
    Replacement::draft_throws(0x1421aac50, "r_1421aac50", fn_1421aac50 as *const (), Throws::Caught), // sub_1421aac50
    Replacement::draft_throws(0x1421ab0e0, "r_1421ab0e0", fn_1421ab0e0 as *const (), Throws::Caught), // sub_1421ab0e0
    Replacement::draft_throws(0x1421ab440, "r_1421ab440", fn_1421ab440 as *const (), Throws::Caught), // sub_1421ab440
    Replacement::draft_throws(0x1421ab6d0, "r_1421ab6d0", fn_1421ab6d0 as *const (), Throws::Caught), // sub_1421ab6d0
    Replacement::draft_throws(0x1421abac0, "r_1421abac0", fn_1421abac0 as *const (), Throws::Caught), // sub_1421abac0
    Replacement::draft(0x1421abb70, "r_1421abb70", tpl_43990::<0xa78> as *const ()), // sub_1421abb70
    Replacement::draft(0x1421abb90, "r_1421abb90", fn_1421abb90 as *const ()), // sub_1421abb90
    Replacement::draft(0x1421abc40, "r_1421abc40", fn_1421abc40 as *const ()), // sub_1421abc40
    Replacement::draft(0x1421abc80, "r_1421abc80", fn_1421abc80 as *const ()), // sub_1421abc80
    Replacement::draft(0x1421abd40, "r_1421abd40", tpl_37328::<0x950> as *const ()), // sub_1421abd40
    Replacement::draft_throws(0x1421abd50, "r_1421abd50", fn_1421abd50 as *const (), Throws::Discard), // sub_1421abd50
    Replacement::draft(0x1421abdb0, "r_1421abdb0", fn_1421abdb0 as *const ()), // sub_1421abdb0
    Replacement::draft_throws(0x1421abf50, "r_1421abf50", fn_1421abf50 as *const (), Throws::Discard), // sub_1421abf50
    Replacement::draft_throws(0x1421ac930, "r_1421ac930", fn_1421ac930 as *const (), Throws::Caught), // sub_1421ac930
    Replacement::draft(0x1421aca40, "r_1421aca40", tpl_46954::<0x40, 0x18, 0x1412b6420, 0x8, 0x14240ab10> as *const ()), // sub_1421aca40
    Replacement::draft(0x1421aca90, "r_1421aca90", tpl_46954::<0x40, 0x18, 0x1412b66c0, 0x8, 0x14240ab10> as *const ()), // sub_1421aca90
    Replacement::draft(0x1421acae0, "r_1421acae0", tpl_46954::<0x40, 0x18, 0x1412b6150, 0x8, 0x14240ab10> as *const ()), // sub_1421acae0
    Replacement::draft_throws(0x1421acb30, "r_1421acb30", fn_1421acb30 as *const (), Throws::Caught), // sub_1421acb30
    Replacement::draft(0x1421acc50, "r_1421acc50", fn_1421acc50 as *const ()), // sub_1421acc50
    Replacement::draft(0x1421acc80, "r_1421acc80", tpl_46954::<0x40, 0x18, 0x1412b5f00, 0x8, 0x14240ab10> as *const ()), // sub_1421acc80
    Replacement::draft(0x1421accd0, "r_1421accd0", tpl_46957::<0x14128fc90> as *const ()), // sub_1421accd0
    Replacement::draft(0x1421acd00, "r_1421acd00", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acd00
    Replacement::draft(0x1421acd30, "r_1421acd30", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acd30
    Replacement::draft(0x1421acd60, "r_1421acd60", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acd60
    Replacement::draft(0x1421acd90, "r_1421acd90", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acd90
    Replacement::draft(0x1421acde0, "r_1421acde0", tpl_46958::<0x14183acc2, 0x1412a64e0, 0x300> as *const ()), // sub_1421acde0
    Replacement::draft(0x1421ace20, "r_1421ace20", fn_1421ace20 as *const ()), // sub_1421ace20
    Replacement::draft(0x1421aceb0, "r_1421aceb0", tpl_46957::<0x14128fc90> as *const ()), // sub_1421aceb0
    Replacement::draft(0x1421acee0, "r_1421acee0", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acee0
    Replacement::draft(0x1421acf10, "r_1421acf10", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acf10
    Replacement::draft(0x1421acf40, "r_1421acf40", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acf40
    Replacement::draft(0x1421acf70, "r_1421acf70", tpl_46957::<0x14128fc90> as *const ()), // sub_1421acf70
    Replacement::draft(0x1421acfd0, "r_1421acfd0", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421acfd0
    Replacement::draft(0x1421ad030, "r_1421ad030", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421ad030
    Replacement::draft(0x1421ad090, "r_1421ad090", tpl_37328::<0xa60> as *const ()), // sub_1421ad090
    Replacement::draft(0x1421ad0c0, "r_1421ad0c0", tpl_46958::<0x14183acc2, 0x1412a64e0, 0x300> as *const ()), // sub_1421ad0c0
    Replacement::draft(0x1421ad100, "r_1421ad100", fn_1421ad100 as *const ()), // sub_1421ad100
    Replacement::draft(0x1421ad190, "r_1421ad190", fn_1421ad190 as *const ()), // sub_1421ad190
    Replacement::draft(0x1421ad210, "r_1421ad210", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421ad210
    Replacement::draft(0x1421ad270, "r_1421ad270", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421ad270
    Replacement::draft(0x1421ad2d0, "r_1421ad2d0", fn_1421ad2d0 as *const ()), // sub_1421ad2d0
    Replacement::draft(0x1421ad3a0, "r_1421ad3a0", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421ad3a0
    Replacement::draft(0x1421ad400, "r_1421ad400", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421ad400
    Replacement::draft(0x1421ad460, "r_1421ad460", tpl_46960::<0xa68, 0x14128fc90> as *const ()), // sub_1421ad460
    Replacement::draft(0x1421ad5a0, "r_1421ad5a0", tpl_37625::<0xc> as *const ()), // sub_1421ad5a0
    Replacement::draft(0x1421ad5c0, "r_1421ad5c0", tpl_45911 as *const ()), // sub_1421ad5c0
    Replacement::draft(0x1421ad650, "r_1421ad650", fn_1421ad650 as *const ()), // sub_1421ad650
    Replacement::draft(0x1421ad6b0, "r_1421ad6b0", fn_1421ad6b0 as *const ()), // sub_1421ad6b0
    Replacement::draft(0x1421ad890, "r_1421ad890", tpl_40081 as *const ()), // sub_1421ad890
    Replacement::draft(0x1421ad8b0, "r_1421ad8b0", tpl_35426::<0x1433eacd0, 0x8, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1421ad8b0
    Replacement::draft(0x1421ad8f0, "r_1421ad8f0", tpl_35426::<0x1433eace8, 0x8, 0x1402bf830, 0x20, 0x28> as *const ()), // sub_1421ad8f0
    Replacement::draft(0x1421ad930, "r_1421ad930", tpl_35426::<0x1433ea8f0, 0x8, 0x1412900a0, 0x20, 0x28> as *const ()), // sub_1421ad930
    Replacement::draft(0x1421ad970, "r_1421ad970", tpl_35426::<0x1433ea908, 0x8, 0x1402bf830, 0x20, 0x28> as *const ()), // sub_1421ad970
    Replacement::draft(0x1421adda0, "r_1421adda0", tpl_35429::<0x1433eacd0, 0x8, 0x1401f3d90, 0x20, 0x28, 0x1433eaca0> as *const ()), // sub_1421adda0
    Replacement::draft(0x1421addf0, "r_1421addf0", tpl_35429::<0x1433eace8, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433eacb8> as *const ()), // sub_1421addf0
    Replacement::draft(0x1421ade40, "r_1421ade40", tpl_35429::<0x1433ea8f0, 0x8, 0x1412900a0, 0x20, 0x28, 0x1433eb080> as *const ()), // sub_1421ade40
    Replacement::draft(0x1421ade90, "r_1421ade90", tpl_35429::<0x1433ea908, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433eb098> as *const ()), // sub_1421ade90
    Replacement::draft(0x1421adee0, "r_1421adee0", tpl_40082 as *const ()), // sub_1421adee0
    Replacement::draft(0x1421adef0, "r_1421adef0", fn_1421adef0 as *const ()), // sub_1421adef0
    Replacement::draft(0x1421adf70, "r_1421adf70", tpl_46967::<0x14183acc2, 0x10, 0x8, 0xc> as *const ()), // sub_1421adf70
    Replacement::draft(0x1421ae0c0, "r_1421ae0c0", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1421ae0c0
    Replacement::draft_throws(0x1421ae0e0, "r_1421ae0e0", fn_1421ae0e0 as *const (), Throws::Discard), // r::internal::VoidContinuationImpl<platform::achievements::internal::issueDeferredProgressWritesIfNecessary::<lambda_1>,?
    Replacement::draft(0x1421ae1a0, "r_1421ae1a0", tpl_35117::<0x20, 0x28, 0x8, 0xc> as *const ()), // sub_1421ae1a0
    Replacement::draft_throws(0x1421ae1c0, "r_1421ae1c0", fn_1421ae1c0 as *const (), Throws::Discard), // r::internal::ContinuationImpl<platform::achievements::internal::readProgressFromStorage::<lambda_1>,?bref7::Expected<pla
    Replacement::draft(0x1421ae280, "r_1421ae280", tpl_35117::<0x20, 0x28, 0x8, 0x18> as *const ()), // sub_1421ae280
    Replacement::draft(0x1421ae2a0, "r_1421ae2a0", tpl_365::<0x1412bd890> as *const ()), // r::internal::ContinuationImpl<platform::achievements::storeProgress::<lambda_1>,?bref4::storage::WriteResult>::r::intern
    Replacement::draft(0x1421ae330, "r_1421ae330", tpl_35117::<0x20, 0x28, 0x8, 0xc> as *const ()), // sub_1421ae330
    Replacement::draft(0x1421ae350, "r_1421ae350", fn_1421ae350 as *const ()), // r::internal::ContinuationImpl<platform::achievements::restoreProgress::<lambda_1>,?bref6::Expected<void,platform::Error>
    Replacement::draft_throws(0x1421f16b0, "r_1421f16b0", fn_1421f16b0 as *const (), Throws::Discard), // r::JobCallable<rend::VideoStreamerFactory::updateAsync::<lambda_1>>::r::JobCallable<rend::VideoStreamerFactory::updateAs
    Replacement::draft_throws(0x142206050, "r_142206050", fn_142206050 as *const (), Throws::Discard), // r::JobCallable<rend::TextureResourceStreamManager::update::?bref1::operator()::<lambda_1>>::r::JobCallable<rend::Texture
    Replacement::draft(0x14222c460, "r_14222c460", fn_14222c460 as *const ()), // r::JobCallable<rend::SkinnedTransformManager::executeBoundsUpdate::<lambda_1>>::r::JobCallable<rend::SkinnedTransformMan
    Replacement::draft(0x14222c550, "r_14222c550", fn_14222c550 as *const ()), // r::JobCallable<rend::SkinnedTransformManager::executeBoundsUpdate::<lambda_2>>::r::JobCallable<rend::SkinnedTransformMan
    Replacement::draft(0x14222cd60, "r_14222cd60", fn_14222cd60 as *const ()), // sub_14222cd60
    Replacement::draft(0x14222ce30, "r_14222ce30", tpl_46490::<0x8> as *const ()), // sub_14222ce30
    Replacement::draft(0x14222ce50, "r_14222ce50", fn_14222ce50 as *const ()), // sub_14222ce50
    Replacement::draft_throws(0x14222cf20, "r_14222cf20", fn_14222cf20 as *const (), Throws::Discard), // sub_14222cf20
    Replacement::draft_throws(0x14222dca0, "r_14222dca0", fn_14222dca0 as *const (), Throws::Caught), // sub_14222dca0
    Replacement::draft(0x14222e280, "r_14222e280", fn_14222e280 as *const ()), // sub_14222e280
    Replacement::draft_throws(0x14222e360, "r_14222e360", fn_14222e360 as *const (), Throws::Discard), // sub_14222e360
    Replacement::draft(0x142231480, "r_142231480", tpl_38069::<0x20, 0x8, 0x40, 0x1433f1280, 0x28, 0x48> as *const ()), // sub_142231480
    Replacement::draft(0x1422314c0, "r_1422314c0", tpl_37789::<0x48, 0x40, 0x20, 0x8, 0x30, 0x18, 0x1412ed4a0> as *const ()), // r::JobCallable<r::parallelfordetail::parallelForEachJob<rend::SignedDistanceField::Cascade,const ?bref3::?bref2::update:
    Replacement::draft(0x142231550, "r_142231550", tpl_37788::<0x20, 0x1433f1268, 0x8, 0x28> as *const ()), // sub_142231550
    Replacement::draft(0x142231580, "r_142231580", tpl_37791::<0x20, 0x8, 0x18, 0x40, 0x30, 0x1412ed4a0> as *const ()), // r::JobCallable<r::parallelForEach<r::DynamicArray<rend::SignedDistanceField::Cascade,r::Allocator<rend::SignedDistanceFi
    Replacement::draft_throws(0x142250b20, "r_142250b20", fn_142250b20 as *const (), Throws::Discard), // r::JobCallable<rend::SDFDataManager::startCopyToCPU::<lambda_1>>::r::JobCallable<rend::SDFDataManager::startCopyToCPU::<
    Replacement::draft(0x142250ba0, "r_142250ba0", fn_142250ba0 as *const ()), // sub_142250ba0
    Replacement::draft(0x142250bd0, "r_142250bd0", fn_142250bd0 as *const ()), // sub_142250bd0
    Replacement::draft(0x142250c00, "r_142250c00", fn_142250c00 as *const ()), // sub_142250c00
    Replacement::draft(0x142250dc0, "r_142250dc0", tpl_47948::<0x2a8, 0x143a082f8, 0x142485780> as *const ()), // sub_142250dc0
    Replacement::draft(0x142250de0, "r_142250de0", fn_142250de0 as *const ()), // sub_142250de0
    Replacement::draft_throws(0x142250e00, "r_142250e00", fn_142250e00 as *const (), Throws::Discard), // sub_142250e00
    Replacement::draft_throws(0x142250e80, "r_142250e80", fn_142250e80 as *const (), Throws::Discard), // sub_142250e80
    Replacement::draft_throws(0x142250fa0, "r_142250fa0", fn_142250fa0 as *const (), Throws::Discard), // sub_142250fa0
    Replacement::draft_throws(0x142251980, "r_142251980", fn_142251980 as *const (), Throws::Discard), // sub_142251980
    Replacement::draft_throws(0x142251d10, "r_142251d10", fn_142251d10 as *const (), Throws::Discard), // sub_142251d10
    Replacement::draft_throws(0x142252cf0, "r_142252cf0", fn_142252cf0 as *const (), Throws::Caught), // sub_142252cf0
    Replacement::draft_throws(0x142252f10, "r_142252f10", fn_142252f10 as *const (), Throws::Caught), // sub_142252f10
    Replacement::draft(0x1422548f0, "r_1422548f0", fn_1422548f0 as *const ()), // sub_1422548f0
    Replacement::draft(0x142254940, "r_142254940", fn_142254940 as *const ()), // sub_142254940
    Replacement::draft_throws(0x142254990, "r_142254990", fn_142254990 as *const (), Throws::Discard), // sub_142254990
    Replacement::draft(0x142254de0, "r_142254de0", tpl_47957 as *const ()), // sub_142254de0
    Replacement::draft(0x142254df0, "r_142254df0", fn_142254df0 as *const ()), // sub_142254df0
    Replacement::draft(0x142254f80, "r_142254f80", tpl_47957 as *const ()), // sub_142254f80
    Replacement::draft(0x142254f90, "r_142254f90", fn_142254f90 as *const ()), // sub_142254f90
    Replacement::draft(0x142255170, "r_142255170", fn_142255170 as *const ()), // sub_142255170
    Replacement::draft_throws(0x1422554d0, "r_1422554d0", fn_1422554d0 as *const (), Throws::Discard), // sub_1422554d0
    Replacement::draft_throws(0x1422555f0, "r_1422555f0", fn_1422555f0 as *const (), Throws::Caught), // sub_1422555f0
    Replacement::draft(0x142255d10, "r_142255d10", fn_142255d10 as *const ()), // sub_142255d10
    Replacement::draft_throws(0x142255e30, "r_142255e30", fn_142255e30 as *const (), Throws::Discard), // sub_142255e30
    Replacement::draft(0x142255ec0, "r_142255ec0", fn_142255ec0 as *const ()), // sub_142255ec0
    Replacement::draft(0x142256010, "r_142256010", fn_142256010 as *const ()), // sub_142256010
    Replacement::draft_throws(0x1422561f0, "r_1422561f0", fn_1422561f0 as *const (), Throws::Discard), // sub_1422561f0
    Replacement::draft_throws(0x142256270, "r_142256270", fn_142256270 as *const (), Throws::Discard), // sub_142256270
    Replacement::draft_throws(0x1422562e0, "r_1422562e0", fn_1422562e0 as *const (), Throws::Discard), // sub_1422562e0
    Replacement::draft(0x1422568c0, "r_1422568c0", fn_1422568c0 as *const ()), // sub_1422568c0
    Replacement::draft(0x1422568e0, "r_1422568e0", fn_1422568e0 as *const ()), // sub_1422568e0
    Replacement::draft(0x142256900, "r_142256900", tpl_23993::<0x24, 0x28> as *const ()), // sub_142256900
    Replacement::draft_throws(0x142256910, "r_142256910", fn_142256910 as *const (), Throws::Caught), // sub_142256910
    Replacement::draft(0x142256c90, "r_142256c90", fn_142256c90 as *const ()), // sub_142256c90
    Replacement::draft(0x142256d10, "r_142256d10", fn_142256d10 as *const ()), // sub_142256d10
    Replacement::draft(0x142256e60, "r_142256e60", fn_142256e60 as *const ()), // sub_142256e60
    Replacement::draft(0x142256eb0, "r_142256eb0", fn_142256eb0 as *const ()), // sub_142256eb0
    Replacement::draft(0x142256f00, "r_142256f00", fn_142256f00 as *const ()), // sub_142256f00
    Replacement::draft(0x142256f60, "r_142256f60", fn_142256f60 as *const ()), // sub_142256f60
    Replacement::draft(0x142257060, "r_142257060", fn_142257060 as *const ()), // sub_142257060
    Replacement::draft(0x142257180, "r_142257180", fn_142257180 as *const ()), // sub_142257180
    Replacement::draft(0x1422571c0, "r_1422571c0", tpl_47977::<0x1438688d0, 0x142645790, 0x1438688d8, 0x1426455a8, 0x1438688dc, 0x143bbcfe0, 0x143bbcfe8, 0x10, 0xffffffffffffffff, 0x142645598> as *const ()), // sub_1422571c0
    Replacement::draft(0x142257270, "r_142257270", tpl_47977::<0x1438688d0, 0x142645790, 0x1438688d8, 0x1426455a8, 0x1438688dc, 0x143bbcfe0, 0x143bbcfe8, 0x8, 0xffffffffffffffff, 0x142645598> as *const ()), // sub_142257270
    Replacement::draft(0x142257320, "r_142257320", tpl_43617::<0x1433f2e40, 0x142645790, 0x1438688d8, 0x1438688d0, 0x1426455a8, 0x143bbcfe0, 0x143bbcfe8, 0x1438688dc, 0x14183acc8, 0x142645598> as *const ()), // sub_142257320
    Replacement::draft(0x142257440, "r_142257440", tpl_43619::<0x1433f2e40, 0x142645790, 0x1438688d8, 0x1438688d0, 0x1426455a8, 0x1438688dc, 0x143bbcfe8, 0x143bbcfe0, 0x143bbcfec, 0x1412f68b0, 0x142645598> as *const ()), // sub_142257440
    Replacement::draft_throws(0x142257560, "r_142257560", fn_142257560 as *const (), Throws::Caught), // sub_142257560
    Replacement::draft_throws(0x142257690, "r_142257690", fn_142257690 as *const (), Throws::Caught), // sub_142257690
    Replacement::draft_throws(0x1422577e0, "r_1422577e0", fn_1422577e0 as *const (), Throws::Caught), // sub_1422577e0
    Replacement::draft_throws(0x142257af0, "r_142257af0", fn_142257af0 as *const (), Throws::Caught), // sub_142257af0
    Replacement::draft_throws(0x142257c60, "r_142257c60", fn_142257c60 as *const (), Throws::Caught), // sub_142257c60
    Replacement::draft_throws(0x142257ee0, "r_142257ee0", fn_142257ee0 as *const (), Throws::Discard), // sub_142257ee0
    Replacement::draft(0x142258040, "r_142258040", fn_142258040 as *const ()), // sub_142258040
    Replacement::draft_throws(0x142258080, "r_142258080", fn_142258080 as *const (), Throws::Caught), // sub_142258080
    Replacement::draft_throws(0x1422581e0, "r_1422581e0", fn_1422581e0 as *const (), Throws::Caught), // sub_1422581e0
    Replacement::draft_throws(0x142258570, "r_142258570", fn_142258570 as *const (), Throws::Discard), // sub_142258570
    Replacement::draft(0x1422586a0, "r_1422586a0", fn_1422586a0 as *const ()), // sub_1422586a0
    Replacement::draft(0x142258760, "r_142258760", fn_142258760 as *const ()), // sub_142258760
    Replacement::draft(0x1422589f0, "r_1422589f0", fn_1422589f0 as *const ()), // sub_1422589f0
    Replacement::draft(0x142258b30, "r_142258b30", fn_142258b30 as *const ()), // sub_142258b30
    Replacement::draft(0x142258b50, "r_142258b50", fn_142258b50 as *const ()), // sub_142258b50
    Replacement::draft(0x142258bb0, "r_142258bb0", tpl_47993::<0x1438697f1, 0x140000000, 0x143426780, 0x38697f4, 0x143868a71, 0x3868a74> as *const ()), // sub_142258bb0
    Replacement::draft(0x142258c00, "r_142258c00", tpl_47993::<0x143868cb1, 0x140000000, 0x143426780, 0x3868cb4, 0x1438692b1, 0x38692b4> as *const ()), // sub_142258c00
    Replacement::draft(0x142258c50, "r_142258c50", tpl_47993::<0x143868bf1, 0x140000000, 0x143426780, 0x3868bf4, 0x1438694f1, 0x38694f4> as *const ()), // sub_142258c50
    Replacement::draft(0x142258ca0, "r_142258ca0", tpl_47993::<0x143868e31, 0x140000000, 0x143426780, 0x3868e34, 0x1438695b1, 0x38695b4> as *const ()), // sub_142258ca0
    Replacement::draft(0x142258cf0, "r_142258cf0", tpl_47993::<0x143869671, 0x140000000, 0x143426780, 0x3869674, 0x143868b31, 0x3868b34> as *const ()), // sub_142258cf0
    Replacement::draft(0x142258e80, "r_142258e80", fn_142258e80 as *const ()), // r::JobCallable<rend::RenderObjectManager::optimizeObjectIDHandlesBegin::<lambda_1>>::r::JobCallable<rend::RenderObjectMa
    Replacement::draft(0x142258f20, "r_142258f20", fn_142258f20 as *const ()), // sub_142258f20
    Replacement::draft(0x142258f50, "r_142258f50", fn_142258f50 as *const ()), // sub_142258f50
    Replacement::draft_throws(0x14225b8b0, "r_14225b8b0", fn_14225b8b0 as *const (), Throws::Discard), // sub_14225b8b0
    Replacement::draft_throws(0x14225c7a0, "r_14225c7a0", fn_14225c7a0 as *const (), Throws::Discard), // sub_14225c7a0
    Replacement::draft(0x14225c7e0, "r_14225c7e0", fn_14225c7e0 as *const ()), // sub_14225c7e0
    Replacement::draft(0x14225c880, "r_14225c880", tpl_48003::<0x10> as *const ()), // sub_14225c880
    Replacement::draft(0x14225c8c0, "r_14225c8c0", tpl_47948::<0x628, 0x143a082f8, 0x142485780> as *const ()), // sub_14225c8c0
    Replacement::draft(0x14225c8f0, "r_14225c8f0", fn_14225c8f0 as *const ()), // sub_14225c8f0
    Replacement::draft(0x14225c900, "r_14225c900", fn_14225c900 as *const ()), // sub_14225c900
    Replacement::draft(0x14225cc80, "r_14225cc80", fn_14225cc80 as *const ()), // sub_14225cc80
    Replacement::draft(0x14225ce30, "r_14225ce30", fn_14225ce30 as *const ()), // sub_14225ce30
    Replacement::draft(0x14225d3a0, "r_14225d3a0", fn_14225d3a0 as *const ()), // sub_14225d3a0
    Replacement::draft_throws(0x14225d5b0, "r_14225d5b0", fn_14225d5b0 as *const (), Throws::Discard), // sub_14225d5b0
    Replacement::draft(0x14225dd30, "r_14225dd30", fn_14225dd30 as *const ()), // sub_14225dd30
    Replacement::draft_throws(0x14225de80, "r_14225de80", fn_14225de80 as *const (), Throws::Discard), // sub_14225de80
    Replacement::draft(0x14225e6c0, "r_14225e6c0", fn_14225e6c0 as *const ()), // sub_14225e6c0
    Replacement::draft(0x14225f1f0, "r_14225f1f0", tpl_30883::<0x18> as *const ()), // sub_14225f1f0
    Replacement::draft_throws(0x14225f220, "r_14225f220", fn_14225f220 as *const (), Throws::Discard), // sub_14225f220
    Replacement::draft_throws(0x14225f310, "r_14225f310", fn_14225f310 as *const (), Throws::Discard), // sub_14225f310
    Replacement::draft_throws(0x14225f3b0, "r_14225f3b0", fn_14225f3b0 as *const (), Throws::Discard), // sub_14225f3b0
    Replacement::draft_throws(0x14225f5a0, "r_14225f5a0", fn_14225f5a0 as *const (), Throws::Discard), // sub_14225f5a0
    Replacement::draft(0x14225f5d0, "r_14225f5d0", fn_14225f5d0 as *const ()), // sub_14225f5d0
    Replacement::draft(0x142263520, "r_142263520", tpl_48030::<0x143a359f8, 0x14397fc80, 0x14397fc60, 0x10, 0x142645ca8, 0x143c48ad0, 0x1433dc3e0, 0x142645cb0, 0x143c48e20, 0x1433dc3b0, 0x143c48e48, 0x1433dc390, 0x1418349c0, 0x142072ec0> as *const ()), // r::JobCallable<rend::RendererInterface::RendererInterface::<lambda_1>>::r::JobCallable<rend::RendererInterface::Renderer
    Replacement::draft(0x142263640, "r_142263640", fn_142263640 as *const ()), // r::JobCallable<rend::RendererInterface::RendererInterface::<lambda_2>>::r::JobCallable<rend::RendererInterface::Renderer
    Replacement::draft_throws(0x142264030, "r_142264030", fn_142264030 as *const (), Throws::Caught), // r::JobCallable<rend::RendererInterface::renderWorldInternal::<lambda_3>::operator()::?bref1::operator()::<lambda_1>>::r:
    Replacement::draft(0x1422a63f0, "r_1422a63f0", tpl_42135::<0x1433f6808, 0x8> as *const ()), // sub_1422a63f0
    Replacement::draft_throws(0x1422a6440, "r_1422a6440", fn_1422a6440 as *const (), Throws::Discard), // r::JobCallable<rend::RaytracingTopLevelBuilder::recordBuildCommands::<lambda_2>>::r::JobCallable<rend::RaytracingTopLeve
    Replacement::draft(0x142309df0, "r_142309df0", tpl_1228::<0x8, 0x10, 0x1401de830, 0x140f366d0> as *const ()), // sub_142309df0
    Replacement::draft(0x1423534c0, "r_1423534c0", tpl_42653::<0x14330bca8, 0x20> as *const ()), // sub_1423534c0
    Replacement::draft_throws(0x1423534e0, "r_1423534e0", fn_1423534e0 as *const (), Throws::Discard), // sub_1423534e0
    Replacement::draft(0x1423f4040, "r_1423f4040", tpl_25480::<0x1401f3d00, 0x8, 0x1401de830, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::WinFileSystem::submitBatch::<lambda_2>,void>::r::internal::ContinuationData<r::WinFileS
    Replacement::draft(0x1423f40a0, "r_1423f40a0", tpl_49612::<0x143407038, 0x8, 0x10, 0xc, 0x14, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_1423f40a0
    Replacement::draft(0x1423f4110, "r_1423f4110", tpl_41197::<0x1401f3d00, 0x8, 0x1401de830, 0x143391070> as *const ()), // sub_1423f4110
    Replacement::draft(0x1423f4150, "r_1423f4150", tpl_41197::<0x1401f3d00, 0x8, 0x1401de830, 0x143391070> as *const ()), // sub_1423f4150
    Replacement::draft(0x1423f4190, "r_1423f4190", tpl_25480::<0x1401f3d00, 0x8, 0x1401de830, 0x143391070, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<r::WinFileSystem::submitBatch::<lambda_2>,void>::r::internal::VoidContinuationImpl<r::
    Replacement::draft(0x1423f41f0, "r_1423f41f0", tpl_49613::<0x143407038, 0x8, 0x10, 0xc, 0x14, 0x1401f3d90, 0x20, 0x28, 0x143407050> as *const ()), // sub_1423f41f0
    Replacement::draft(0x1423f42f0, "r_1423f42f0", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1423f42f0
    Replacement::draft(0x1423f4310, "r_1423f4310", fn_1423f4310 as *const ()), // r::internal::VoidContinuationImpl<r::WinFileSystem::submitBatch::<lambda_2>,void>::r::internal::VoidContinuationImpl<r::
    Replacement::draft(0x1423f43d0, "r_1423f43d0", fn_1423f43d0 as *const ()), // sub_1423f43d0
    Replacement::draft(0x1423f44c0, "r_1423f44c0", fn_1423f44c0 as *const ()), // sub_1423f44c0
    Replacement::draft(0x1423f4530, "r_1423f4530", tpl_49617::<0x8, 0x1423f6830> as *const ()), // sub_1423f4530
    Replacement::draft_throws(0x1423f4580, "r_1423f4580", fn_1423f4580 as *const (), Throws::Caught), // sub_1423f4580
    Replacement::draft_throws(0x1423f4690, "r_1423f4690", tpl_49619::<0x8, 0x1423f9190, 0x141371610, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f4690
    Replacement::draft(0x1423f4740, "r_1423f4740", fn_1423f4740 as *const ()), // sub_1423f4740
    Replacement::draft_throws(0x1423f4770, "r_1423f4770", fn_1423f4770 as *const (), Throws::Discard), // sub_1423f4770
    Replacement::draft(0x1423f48b0, "r_1423f48b0", fn_1423f48b0 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_43
    Replacement::draft(0x1423f48e0, "r_1423f48e0", fn_1423f48e0 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_41
    Replacement::draft(0x1423f4920, "r_1423f4920", fn_1423f4920 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_40
    Replacement::draft(0x1423f4990, "r_1423f4990", fn_1423f4990 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_37
    Replacement::draft_throws(0x1423f49d0, "r_1423f49d0", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x141372554, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f49d0
    Replacement::draft(0x1423f4b40, "r_1423f4b40", fn_1423f4b40 as *const ()), // sub_1423f4b40
    Replacement::draft_throws(0x1423f4c20, "r_1423f4c20", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x141372584, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f4c20
    Replacement::draft(0x1423f4d80, "r_1423f4d80", fn_1423f4d80 as *const ()), // sub_1423f4d80
    Replacement::draft_throws(0x1423f4dd0, "r_1423f4dd0", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x14137254c, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f4dd0
    Replacement::draft(0x1423f4f30, "r_1423f4f30", fn_1423f4f30 as *const ()), // sub_1423f4f30
    Replacement::draft_throws(0x1423f4fa0, "r_1423f4fa0", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x14137257c, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f4fa0
    Replacement::draft(0x1423f5110, "r_1423f5110", fn_1423f5110 as *const ()), // sub_1423f5110
    Replacement::draft_throws(0x1423f51b0, "r_1423f51b0", tpl_49619::<0x8, 0x1423f9190, 0x141371610, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f51b0
    Replacement::draft(0x1423f5290, "r_1423f5290", fn_1423f5290 as *const ()), // sub_1423f5290
    Replacement::draft_throws(0x1423f52e0, "r_1423f52e0", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x141372574, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f52e0
    Replacement::draft(0x1423f5440, "r_1423f5440", fn_1423f5440 as *const ()), // sub_1423f5440
    Replacement::draft_throws(0x1423f54c0, "r_1423f54c0", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x14137253c, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f54c0
    Replacement::draft(0x1423f5600, "r_1423f5600", fn_1423f5600 as *const ()), // sub_1423f5600
    Replacement::draft(0x1423f5630, "r_1423f5630", fn_1423f5630 as *const ()), // sub_1423f5630
    Replacement::draft(0x1423f56f0, "r_1423f56f0", tpl_49635::<0x18, 0x8, 0x141371510> as *const ()), // sub_1423f56f0
    Replacement::draft_throws(0x1423f5750, "r_1423f5750", fn_1423f5750 as *const (), Throws::Caught), // sub_1423f5750
    Replacement::draft(0x1423f58d0, "r_1423f58d0", tpl_49635::<0x18, 0x8, 0x141371510> as *const ()), // sub_1423f58d0
    Replacement::draft_throws(0x1423f5940, "r_1423f5940", fn_1423f5940 as *const (), Throws::Caught), // sub_1423f5940
    Replacement::draft(0x1423f5ac0, "r_1423f5ac0", tpl_49638::<0x8, 0x143a08318, 0x18, 0x1423f33e0, 0x3, 0x4, 0x14137256c, 0x14240ab10> as *const ()), // sub_1423f5ac0
    Replacement::draft_throws(0x1423f5bc0, "r_1423f5bc0", crate::shards::subsystems::fn_1423f5bc0 as *const (), Throws::Caught), // sub_1423f5bc0
    Replacement::draft(0x1423f5c60, "r_1423f5c60", fn_1423f5c60 as *const ()), // sub_1423f5c60
    Replacement::draft_throws(0x1423f5d70, "r_1423f5d70", fn_1423f5d70 as *const (), Throws::Caught), // sub_1423f5d70
    Replacement::draft(0x1423f5e20, "r_1423f5e20", tpl_49642::<0xfffffffffffffffe, 0x8, 0x143a08318, 0x18, 0x1423f33e0, 0x3, 0x4, 0x141372564, 0x14240ab10> as *const ()), // sub_1423f5e20
    Replacement::draft_throws(0x1423f5f40, "r_1423f5f40", crate::shards::subsystems::fn_1423f5f40 as *const (), Throws::Caught), // sub_1423f5f40
    Replacement::draft(0x1423f5fe0, "r_1423f5fe0", tpl_49638::<0x8, 0x143a08318, 0x18, 0x1423f33e0, 0x3, 0x4, 0x14137258c, 0x14240ab10> as *const ()), // sub_1423f5fe0
    Replacement::draft_throws(0x1423f60e0, "r_1423f60e0", fn_1423f60e0 as *const (), Throws::Caught), // sub_1423f60e0
    Replacement::draft(0x1423f61c0, "r_1423f61c0", tpl_49642::<0xfffffffffffffffe, 0x8, 0x143a08318, 0x18, 0x1423f33e0, 0x3, 0x4, 0x141372598, 0x14240ab10> as *const ()), // sub_1423f61c0
    Replacement::draft_throws(0x1423f62e0, "r_1423f62e0", fn_1423f62e0 as *const (), Throws::Caught), // sub_1423f62e0
    Replacement::draft_throws(0x1423f6410, "r_1423f6410", tpl_49626::<0x1423f9190, 0x143a08318, 0x1423f33e0, 0x14137255c, 0x14240ab10> as *const (), Throws::Discard), // sub_1423f6410
    Replacement::draft(0x1423f6570, "r_1423f6570", fn_1423f6570 as *const ()), // sub_1423f6570
    Replacement::draft(0x1423f6600, "r_1423f6600", tpl_49617::<0x8, 0x1423f6830> as *const ()), // sub_1423f6600
    Replacement::draft(0x1423f6630, "r_1423f6630", fn_1423f6630 as *const ()), // sub_1423f6630
    Replacement::draft_throws(0x1423f6650, "r_1423f6650", fn_1423f6650 as *const (), Throws::Caught), // sub_1423f6650
    Replacement::draft(0x1423f6780, "r_1423f6780", fn_1423f6780 as *const ()), // sub_1423f6780
    Replacement::draft(0x1423f67c0, "r_1423f67c0", fn_1423f67c0 as *const ()), // sub_1423f67c0
    Replacement::draft(0x1423f6830, "r_1423f6830", fn_1423f6830 as *const ()), // sub_1423f6830
    Replacement::draft(0x1423f69e0, "r_1423f69e0", fn_1423f69e0 as *const ()), // sub_1423f69e0
    Replacement::draft_throws(0x1423f6a00, "r_1423f6a00", fn_1423f6a00 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_48
    Replacement::draft(0x1423f6a90, "r_1423f6a90", tpl_49653::<0x10> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_35
    Replacement::draft(0x1423f6aa0, "r_1423f6aa0", tpl_49654::<0x10> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_34
    Replacement::draft(0x1423f6ab0, "r_1423f6ab0", tpl_49655::<0x28, 0x24> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_33
    Replacement::draft(0x1423f6ad0, "r_1423f6ad0", tpl_49655::<0x2c, 0x30> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_32
    Replacement::draft(0x1423f6af0, "r_1423f6af0", tpl_49656::<0x3c, 0x260, 0x1423fe420, 0x3, 0x8> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_31
    Replacement::draft(0x1423f6b80, "r_1423f6b80", tpl_49657::<0x8, 0x142645508, 0x260, 0x14240ab10, 0x142442de0> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_30
    Replacement::draft(0x1423f6bc0, "r_1423f6bc0", tpl_49658::<0x8, 0x142645510, 0x142645508, 0x260, 0x14240ab10, 0x142442de0> as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_29
    Replacement::draft_throws(0x1423f6c10, "r_1423f6c10", fn_1423f6c10 as *const (), Throws::Caught), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_28
    Replacement::draft_throws(0x1423f6dc0, "r_1423f6dc0", fn_1423f6dc0 as *const (), Throws::Caught), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_27
    Replacement::draft_throws(0x1423f6ea0, "r_1423f6ea0", fn_1423f6ea0 as *const (), Throws::Caught), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_26
    Replacement::draft_throws(0x1423f6fe0, "r_1423f6fe0", fn_1423f6fe0 as *const (), Throws::Caught), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_25
    Replacement::draft_throws(0x1423f70d0, "r_1423f70d0", fn_1423f70d0 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_24
    Replacement::draft_throws(0x1423f7120, "r_1423f7120", fn_1423f7120 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_23
    Replacement::draft_throws(0x1423f7190, "r_1423f7190", tpl_49665::<0x141370ca0, 0x4> as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_21
    Replacement::draft_throws(0x1423f71e0, "r_1423f71e0", fn_1423f71e0 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_20
    Replacement::draft_throws(0x1423f7240, "r_1423f7240", fn_1423f7240 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_19
    Replacement::draft_throws(0x1423f7290, "r_1423f7290", fn_1423f7290 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_18
    Replacement::draft_throws(0x1423f72e0, "r_1423f72e0", fn_1423f72e0 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_16
    Replacement::draft(0x1423f7350, "r_1423f7350", fn_1423f7350 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_15
    Replacement::draft_throws(0x1423f7370, "r_1423f7370", fn_1423f7370 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_14
    Replacement::draft_throws(0x1423f73e0, "r_1423f73e0", fn_1423f73e0 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_13
    Replacement::draft_throws(0x1423f7460, "r_1423f7460", tpl_49673::<0xffffffffffffffb8, 0xffffffffffffffd8, 0xffffffffffffffc0, 0x8, 0xffffffffffffffe8, 0x14136fec0, 0x4> as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_12
    Replacement::draft_throws(0x1423f74b0, "r_1423f74b0", tpl_49673::<0xffffffffffffffb8, 0xffffffffffffffd8, 0xffffffffffffffc0, 0x8, 0xffffffffffffffe8, 0x14136fcc0, 0x4> as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_11
    Replacement::draft_throws(0x1423f7500, "r_1423f7500", tpl_49673::<0xffffffffffffffb8, 0xffffffffffffffd8, 0xffffffffffffffc0, 0x8, 0xffffffffffffffe8, 0x14136fab0, 0x4> as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_10
    Replacement::draft_throws(0x1423f7550, "r_1423f7550", fn_1423f7550 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_9
    Replacement::draft_throws(0x1423f75b0, "r_1423f75b0", fn_1423f75b0 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_8
    Replacement::draft_throws(0x1423f7610, "r_1423f7610", fn_1423f7610 as *const (), Throws::Caught), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_7
    Replacement::draft_throws(0x1423f7710, "r_1423f7710", fn_1423f7710 as *const (), Throws::Caught), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_6
    Replacement::draft_throws(0x1423f7810, "r_1423f7810", tpl_49665::<0x14136f4d0, 0x4> as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_5
    Replacement::draft_throws(0x1423f7860, "r_1423f7860", fn_1423f7860 as *const (), Throws::Discard), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_4
    Replacement::draft(0x1423f78d0, "r_1423f78d0", fn_1423f78d0 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_3
    Replacement::draft(0x1423f78f0, "r_1423f78f0", fn_1423f78f0 as *const ()), // r::BasicWinFileSystem::r::BasicWinFileSystem::vfunc_2
    Replacement::draft(0x1423f7910, "r_1423f7910", tpl_49681::<0x8, 0x20, 0x1423fcae0> as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_50
    Replacement::draft(0x1423f7960, "r_1423f7960", fn_1423f7960 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_49
    Replacement::draft(0x1423f7c60, "r_1423f7c60", fn_1423f7c60 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_47
    Replacement::draft(0x1423f7d40, "r_1423f7d40", fn_1423f7d40 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_46
    Replacement::draft(0x1423f7e20, "r_1423f7e20", fn_1423f7e20 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_45
    Replacement::draft(0x1423f81a0, "r_1423f81a0", fn_1423f81a0 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_44
    Replacement::draft(0x1423f8210, "r_1423f8210", fn_1423f8210 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_43
    Replacement::draft(0x1423f82f0, "r_1423f82f0", tpl_49681::<0x8, 0x20, 0x1423fc340> as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_42
    Replacement::draft(0x1423f8340, "r_1423f8340", fn_1423f8340 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_41
    Replacement::draft(0x1423f8400, "r_1423f8400", fn_1423f8400 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_40
    Replacement::draft(0x1423f88d0, "r_1423f88d0", fn_1423f88d0 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_39
    Replacement::draft(0x1423f8ce0, "r_1423f8ce0", fn_1423f8ce0 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_22
    Replacement::draft(0x1423f8d50, "r_1423f8d50", fn_1423f8d50 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_38
    Replacement::draft_throws(0x1423f8d80, "r_1423f8d80", fn_1423f8d80 as *const (), Throws::Caught), // r::WinFileSystem::r::WinFileSystem::vfunc_37
    Replacement::draft(0x1423f8f70, "r_1423f8f70", fn_1423f8f70 as *const ()), // r::WinFileSystem::r::WinFileSystem::vfunc_17
    Replacement::draft_throws(0x1423f8fb0, "r_1423f8fb0", fn_1423f8fb0 as *const (), Throws::Discard), // sub_1423f8fb0
    Replacement::draft_throws(0x1423f9010, "r_1423f9010", fn_1423f9010 as *const (), Throws::Caught), // sub_1423f9010
    Replacement::draft_throws(0x1423f9090, "r_1423f9090", tpl_49697::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1423f9090
    Replacement::draft_throws(0x1423f90c0, "r_1423f90c0", fn_1423f90c0 as *const (), Throws::Caught), // sub_1423f90c0
    Replacement::draft_throws(0x1423f9190, "r_1423f9190", fn_1423f9190 as *const (), Throws::Discard), // sub_1423f9190
    Replacement::draft_throws(0x1423f9300, "r_1423f9300", fn_1423f9300 as *const (), Throws::Caught), // sub_1423f9300
    Replacement::draft(0x1423f9d20, "r_1423f9d20", fn_1423f9d20 as *const ()), // sub_1423f9d20
    Replacement::draft_throws(0x1423f9d50, "r_1423f9d50", tpl_49697::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1423f9d50
    Replacement::draft(0x1423f9db0, "r_1423f9db0", fn_1423f9db0 as *const ()), // sub_1423f9db0
    Replacement::draft(0x1423f9e70, "r_1423f9e70", fn_1423f9e70 as *const ()), // sub_1423f9e70
    Replacement::draft(0x1423fa630, "r_1423fa630", fn_1423fa630 as *const ()), // sub_1423fa630
    Replacement::draft(0x1423fa660, "r_1423fa660", fn_1423fa660 as *const ()), // sub_1423fa660
    Replacement::draft(0x1423fa6f0, "r_1423fa6f0", fn_1423fa6f0 as *const ()), // sub_1423fa6f0
    Replacement::draft(0x1423fa710, "r_1423fa710", fn_1423fa710 as *const ()), // sub_1423fa710
    Replacement::draft(0x1423fa720, "r_1423fa720", fn_1423fa720 as *const ()), // sub_1423fa720
    Replacement::draft(0x1423fa7b0, "r_1423fa7b0", fn_1423fa7b0 as *const ()), // sub_1423fa7b0
    Replacement::draft(0x1423fa870, "r_1423fa870", fn_1423fa870 as *const ()), // sub_1423fa870
    Replacement::draft(0x1423fa8b0, "r_1423fa8b0", fn_1423fa8b0 as *const ()), // sub_1423fa8b0
    Replacement::draft_throws(0x1423fa900, "r_1423fa900", fn_1423fa900 as *const (), Throws::Caught), // sub_1423fa900
    Replacement::draft(0x1423faa30, "r_1423faa30", fn_1423faa30 as *const ()), // sub_1423faa30
    Replacement::draft_throws(0x1423faaa0, "r_1423faaa0", fn_1423faaa0 as *const (), Throws::Discard), // sub_1423faaa0
    Replacement::draft_throws(0x1423faad0, "r_1423faad0", fn_1423faad0 as *const (), Throws::Caught), // sub_1423faad0
    Replacement::draft_throws(0x1423fabd0, "r_1423fabd0", fn_1423fabd0 as *const (), Throws::Caught), // sub_1423fabd0
    Replacement::draft_throws(0x1423face0, "r_1423face0", fn_1423face0 as *const (), Throws::Caught), // sub_1423face0
    Replacement::draft_throws(0x1423fada0, "r_1423fada0", fn_1423fada0 as *const (), Throws::Caught), // sub_1423fada0
    Replacement::draft_throws(0x1423faea0, "r_1423faea0", crate::shards::subsystems::fn_1423faea0 as *const (), Throws::Caught), // sub_1423faea0
    Replacement::draft_throws(0x1423faf20, "r_1423faf20", fn_1423faf20 as *const (), Throws::Caught), // sub_1423faf20
    Replacement::draft_throws(0x1423fafb0, "r_1423fafb0", crate::shards::subsystems::fn_1423fafb0 as *const (), Throws::Caught), // sub_1423fafb0
    Replacement::draft_throws(0x1423fb030, "r_1423fb030", fn_1423fb030 as *const (), Throws::Caught), // sub_1423fb030
    Replacement::draft_throws(0x1423fb180, "r_1423fb180", fn_1423fb180 as *const (), Throws::Caught), // sub_1423fb180
    Replacement::draft(0x1423fb2c0, "r_1423fb2c0", fn_1423fb2c0 as *const ()), // sub_1423fb2c0
    Replacement::draft(0x1423fb2d0, "r_1423fb2d0", fn_1423fb2d0 as *const ()), // sub_1423fb2d0
    Replacement::draft(0x1423fb370, "r_1423fb370", fn_1423fb370 as *const ()), // sub_1423fb370
    Replacement::draft(0x1423fb3d0, "r_1423fb3d0", tpl_49725::<0x8, 0x142645548, 0x18, 0x142645708> as *const ()), // sub_1423fb3d0
    Replacement::draft(0x1423fb450, "r_1423fb450", tpl_49725::<0x8, 0x1426457d0, 0x18, 0x142645708> as *const ()), // sub_1423fb450
    Replacement::draft(0x1423fb4d0, "r_1423fb4d0", fn_1423fb4d0 as *const ()), // sub_1423fb4d0
    Replacement::draft(0x1423fb510, "r_1423fb510", fn_1423fb510 as *const ()), // sub_1423fb510
    Replacement::draft(0x1423fb540, "r_1423fb540", fn_1423fb540 as *const ()), // sub_1423fb540
    Replacement::draft_throws(0x1423fb600, "r_1423fb600", fn_1423fb600 as *const (), Throws::Caught), // sub_1423fb600
    Replacement::draft_throws(0x1423fb6c0, "r_1423fb6c0", fn_1423fb6c0 as *const (), Throws::Caught), // sub_1423fb6c0
    Replacement::draft_throws(0x1423fb800, "r_1423fb800", fn_1423fb800 as *const (), Throws::Caught), // sub_1423fb800
    Replacement::draft_throws(0x1423fb8b0, "r_1423fb8b0", fn_1423fb8b0 as *const (), Throws::Caught), // sub_1423fb8b0
    Replacement::draft(0x1423fba20, "r_1423fba20", tpl_49658::<0x8, 0x142645510, 0x142645508, 0x260, 0x14240ab10, 0x142442de0> as *const ()), // sub_1423fba20
    Replacement::draft(0x1423fba70, "r_1423fba70", tpl_49657::<0x8, 0x142645508, 0x260, 0x14240ab10, 0x142442de0> as *const ()), // sub_1423fba70
    Replacement::draft(0x1423fbab0, "r_1423fbab0", tpl_49656::<0x3c, 0x260, 0x1423fe420, 0x3, 0x8> as *const ()), // sub_1423fbab0
    Replacement::draft(0x1423fbb40, "r_1423fbb40", tpl_49655::<0x2c, 0x30> as *const ()), // sub_1423fbb40
    Replacement::draft(0x1423fbb60, "r_1423fbb60", tpl_49655::<0x28, 0x24> as *const ()), // sub_1423fbb60
    Replacement::draft(0x1423fbb80, "r_1423fbb80", tpl_49654::<0x10> as *const ()), // sub_1423fbb80
    Replacement::draft(0x1423fbb90, "r_1423fbb90", tpl_49653::<0x10> as *const ()), // sub_1423fbb90
    Replacement::draft_throws(0x1423fbba0, "r_1423fbba0", fn_1423fbba0 as *const (), Throws::Discard), // sub_1423fbba0
    Replacement::draft_throws(0x1423fbc80, "r_1423fbc80", fn_1423fbc80 as *const (), Throws::Caught), // sub_1423fbc80
    Replacement::draft_throws(0x1423fc010, "r_1423fc010", fn_1423fc010 as *const (), Throws::Discard), // sub_1423fc010
    Replacement::draft_throws(0x1423fc0b0, "r_1423fc0b0", fn_1423fc0b0 as *const (), Throws::Caught), // sub_1423fc0b0
    Replacement::draft(0x1423fc1d0, "r_1423fc1d0", fn_1423fc1d0 as *const ()), // sub_1423fc1d0
    Replacement::draft(0x1423fc210, "r_1423fc210", fn_1423fc210 as *const ()), // sub_1423fc210
    Replacement::draft(0x1423fc270, "r_1423fc270", fn_1423fc270 as *const ()), // sub_1423fc270
    Replacement::draft(0x1423fc2e0, "r_1423fc2e0", fn_1423fc2e0 as *const ()), // sub_1423fc2e0
    Replacement::draft(0x1423fc340, "r_1423fc340", fn_1423fc340 as *const ()), // sub_1423fc340
    Replacement::draft(0x1423fc720, "r_1423fc720", fn_1423fc720 as *const ()), // sub_1423fc720
    Replacement::draft(0x1423fc920, "r_1423fc920", fn_1423fc920 as *const ()), // sub_1423fc920
    Replacement::draft(0x1423fca00, "r_1423fca00", fn_1423fca00 as *const ()), // sub_1423fca00
    Replacement::draft(0x1423fcae0, "r_1423fcae0", fn_1423fcae0 as *const ()), // sub_1423fcae0
    Replacement::draft_throws(0x1423fcba0, "r_1423fcba0", fn_1423fcba0 as *const (), Throws::Caught), // sub_1423fcba0
    Replacement::draft(0x1423fcc70, "r_1423fcc70", fn_1423fcc70 as *const ()), // sub_1423fcc70
    Replacement::draft(0x1423fce10, "r_1423fce10", fn_1423fce10 as *const ()), // sub_1423fce10
    Replacement::draft(0x1423fcfa0, "r_1423fcfa0", fn_1423fcfa0 as *const ()), // sub_1423fcfa0
    Replacement::draft(0x1423fd0b0, "r_1423fd0b0", fn_1423fd0b0 as *const ()), // sub_1423fd0b0
    Replacement::draft(0x1423fd630, "r_1423fd630", tpl_38844::<0x143881ec8, 0x143881ec0, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // r::VersionToken::ClassTypeInfoImplementation::r::VersionToken::ClassTypeInfoImplementation::vfunc_25
    Replacement::draft(0x1423fd690, "r_1423fd690", tpl_38844::<0x143881ec8, 0x143881ec0, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // r::VersionToken::ClassTypeInfoImplementation::r::VersionToken::ClassTypeInfoImplementation::vfunc_42
    Replacement::draft(0x1423fda20, "r_1423fda20", fn_1423fda20 as *const ()), // r::VersionToken::ClassTypeInfoImplementation::r::VersionToken::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1423fda50, "r_1423fda50", fn_1423fda50 as *const ()), // r::VersionToken::ClassTypeInfoImplementation::r::VersionToken::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1423fda90, "r_1423fda90", fn_1423fda90 as *const ()), // r::VersionToken::ClassTypeInfoImplementation::r::VersionToken::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1423fdb70, "r_1423fdb70", tpl_20791 as *const ()), // sub_1423fdb70
    Replacement::draft(0x1423fdbc0, "r_1423fdbc0", fn_1423fdbc0 as *const ()), // sub_1423fdbc0
    Replacement::draft(0x1423fdc40, "r_1423fdc40", fn_1423fdc40 as *const ()), // sub_1423fdc40
    Replacement::draft(0x1423fdc90, "r_1423fdc90", fn_1423fdc90 as *const ()), // sub_1423fdc90
    Replacement::draft(0x1423fdcf0, "r_1423fdcf0", fn_1423fdcf0 as *const ()), // sub_1423fdcf0
    Replacement::draft(0x1423fdd60, "r_1423fdd60", fn_1423fdd60 as *const ()), // sub_1423fdd60
    Replacement::draft(0x1423fddd0, "r_1423fddd0", fn_1423fddd0 as *const ()), // sub_1423fddd0
    Replacement::draft(0x1423fde80, "r_1423fde80", fn_1423fde80 as *const ()), // sub_1423fde80
    Replacement::draft(0x1423fdec0, "r_1423fdec0", fn_1423fdec0 as *const ()), // sub_1423fdec0
    Replacement::draft(0x1423fdf30, "r_1423fdf30", fn_1423fdf30 as *const ()), // sub_1423fdf30
    Replacement::draft(0x1423fdf90, "r_1423fdf90", fn_1423fdf90 as *const ()), // sub_1423fdf90
    Replacement::draft(0x1423fe2a0, "r_1423fe2a0", fn_1423fe2a0 as *const ()), // sub_1423fe2a0
    Replacement::draft(0x1423fe350, "r_1423fe350", fn_1423fe350 as *const ()), // sub_1423fe350
    Replacement::draft(0x1423fe400, "r_1423fe400", fn_1423fe400 as *const ()), // sub_1423fe400
    Replacement::draft(0x1423fe420, "r_1423fe420", fn_1423fe420 as *const ()), // sub_1423fe420
    Replacement::draft(0x1423fe650, "r_1423fe650", fn_1423fe650 as *const ()), // sub_1423fe650
    Replacement::draft(0x1423fe6c0, "r_1423fe6c0", fn_1423fe6c0 as *const ()), // sub_1423fe6c0
    Replacement::draft(0x1423fe710, "r_1423fe710", fn_1423fe710 as *const ()), // sub_1423fe710
    Replacement::draft(0x1423fe8f0, "r_1423fe8f0", fn_1423fe8f0 as *const ()), // sub_1423fe8f0
    Replacement::draft(0x1423feac0, "r_1423feac0", fn_1423feac0 as *const ()), // sub_1423feac0
    Replacement::draft(0x1423febc0, "r_1423febc0", fn_1423febc0 as *const ()), // sub_1423febc0
    Replacement::draft(0x1423fec60, "r_1423fec60", fn_1423fec60 as *const ()), // sub_1423fec60
    Replacement::draft(0x1423fed00, "r_1423fed00", fn_1423fed00 as *const ()), // sub_1423fed00
    Replacement::draft(0x1423fed50, "r_1423fed50", tpl_49772::<0x1423fe8f0, 0x143407648, 0x1426e7d15, 0x1405fc3c0, 0x14240ab10> as *const ()), // sub_1423fed50
    Replacement::draft(0x1423fef10, "r_1423fef10", fn_1423fef10 as *const ()), // sub_1423fef10
    Replacement::draft(0x1423ff5f0, "r_1423ff5f0", fn_1423ff5f0 as *const ()), // sub_1423ff5f0
    Replacement::draft(0x1423ffb10, "r_1423ffb10", fn_1423ffb10 as *const ()), // sub_1423ffb10
    Replacement::draft(0x1423ffb60, "r_1423ffb60", fn_1423ffb60 as *const ()), // sub_1423ffb60
    Replacement::draft(0x1423ffba0, "r_1423ffba0", fn_1423ffba0 as *const ()), // sub_1423ffba0
    Replacement::draft(0x1423ffc00, "r_1423ffc00", tpl_49776::<0x1434075c0, 0x1426e7d15, 0x1405fc3c0> as *const ()), // sub_1423ffc00
    Replacement::draft(0x1423ffc30, "r_1423ffc30", fn_1423ffc30 as *const ()), // sub_1423ffc30
    Replacement::draft(0x1423ffd20, "r_1423ffd20", fn_1423ffd20 as *const ()), // sub_1423ffd20
    Replacement::draft(0x1423fff50, "r_1423fff50", tpl_108525::<0x142646980, 0x141376a70> as *const ()), // sub_1423fff50
    Replacement::draft(0x142400190, "r_142400190", fn_142400190 as *const ()), // sub_142400190
    Replacement::draft(0x142400760, "r_142400760", fn_142400760 as *const ()), // sub_142400760
    Replacement::draft(0x142400be0, "r_142400be0", fn_142400be0 as *const ()), // sub_142400be0
    Replacement::draft(0x142400e10, "r_142400e10", fn_142400e10 as *const ()), // sub_142400e10
    Replacement::draft(0x142400fd0, "r_142400fd0", fn_142400fd0 as *const ()), // sub_142400fd0
    Replacement::draft(0x142401120, "r_142401120", fn_142401120 as *const ()), // sub_142401120
    Replacement::draft(0x142401330, "r_142401330", fn_142401330 as *const ()), // sub_142401330
    Replacement::draft(0x142401380, "r_142401380", fn_142401380 as *const ()), // sub_142401380
    Replacement::draft(0x1424014f0, "r_1424014f0", fn_1424014f0 as *const ()), // sub_1424014f0
    Replacement::draft(0x1424016d0, "r_1424016d0", tpl_108525::<0x142646980, 0x141376a70> as *const ()), // sub_1424016d0
    Replacement::draft(0x142401910, "r_142401910", tpl_108534::<0x1434075d0, 0x1426d01e0, 0x141121ad0> as *const ()), // sub_142401910
    Replacement::draft(0x142401a90, "r_142401a90", fn_142401a90 as *const ()), // sub_142401a90
    Replacement::draft(0x142401c50, "r_142401c50", tpl_108536::<0x1434075d0, 0x1426d01e0, 0x141376a70> as *const ()), // sub_142401c50
    Replacement::draft(0x142401dc0, "r_142401dc0", fn_142401dc0 as *const ()), // sub_142401dc0
    Replacement::draft(0x142402020, "r_142402020", fn_142402020 as *const ()), // sub_142402020
    Replacement::draft(0x1424020c0, "r_1424020c0", fn_1424020c0 as *const ()), // sub_1424020c0
    Replacement::draft(0x142402250, "r_142402250", tpl_49772::<0x1423fe8f0, 0x143407648, 0x1426e7d15, 0x1405fc3c0, 0x14240ab10> as *const ()), // sub_142402250
    Replacement::draft(0x142402410, "r_142402410", fn_142402410 as *const ()), // sub_142402410
    Replacement::draft(0x142402520, "r_142402520", tpl_49772::<0x1423fe8f0, 0x143407648, 0x1426e7d15, 0x1405fc3c0, 0x14240ab10> as *const ()), // sub_142402520
    Replacement::draft(0x1424026e0, "r_1424026e0", fn_1424026e0 as *const ()), // sub_1424026e0
    Replacement::draft(0x142402920, "r_142402920", tpl_108536::<0x1434075d0, 0x1426d01e0, 0x141374950> as *const ()), // sub_142402920
    Replacement::draft(0x142402a90, "r_142402a90", tpl_49789::<0x1433dd4f4, 0x1426e7d15, 0x1405fc3c0> as *const ()), // sub_142402a90
    Replacement::draft(0x142402ac0, "r_142402ac0", fn_142402ac0 as *const ()), // sub_142402ac0
    Replacement::draft(0x142402cd0, "r_142402cd0", tpl_49776::<0x1434075c8, 0x1426e7d15, 0x1405fc3c0> as *const ()), // sub_142402cd0
    Replacement::draft(0x142402d00, "r_142402d00", tpl_108534::<0x1434075d0, 0x1426d01e0, 0x141376bf0> as *const ()), // sub_142402d00
    Replacement::draft(0x142402e80, "r_142402e80", fn_142402e80 as *const ()), // sub_142402e80
    Replacement::draft(0x142402ef0, "r_142402ef0", fn_142402ef0 as *const ()), // sub_142402ef0
    Replacement::draft(0x142402f50, "r_142402f50", tpl_49789::<0x1431f4910, 0x1426e7d15, 0x1405fc3c0> as *const ()), // sub_142402f50
    Replacement::draft(0x142402f90, "r_142402f90", fn_142402f90 as *const ()), // r::reflection::detail::TypeInfoFactoryImpl::r::reflection::detail::TypeInfoFactoryImpl::vfunc_5
    Replacement::draft(0x142403070, "r_142403070", fn_142403070 as *const ()), // r::reflection::detail::TypeInfoFactoryImpl::r::reflection::detail::TypeInfoFactoryImpl::vfunc_4
    Replacement::draft(0x1424032f0, "r_1424032f0", fn_1424032f0 as *const ()), // r::reflection::detail::TypeInfoFactoryImpl::r::reflection::detail::TypeInfoFactoryImpl::vfunc_3
    Replacement::draft(0x1424033b0, "r_1424033b0", fn_1424033b0 as *const ()), // r::reflection::detail::TypeInfoFactoryImpl::r::reflection::detail::TypeInfoFactoryImpl::vfunc_2
    Replacement::draft(0x142403530, "r_142403530", fn_142403530 as *const ()), // r::reflection::detail::TypeInfoFactoryImpl::r::reflection::detail::TypeInfoFactoryImpl::vfunc_1
    Replacement::draft(0x1424035c0, "r_1424035c0", fn_1424035c0 as *const ()), // sub_1424035c0
    Replacement::draft(0x142403670, "r_142403670", tpl_40887::<0x143a00280, 0x142442de0> as *const ()), // sub_142403670
    Replacement::draft(0x1424036e0, "r_1424036e0", tpl_40934 as *const ()), // sub_1424036e0
    Replacement::draft(0x142405150, "r_142405150", fn_142405150 as *const ()), // sub_142405150
    Replacement::draft(0x142405190, "r_142405190", tpl_49806::<0x38, 0x8, 0x28, 0x142404ff0> as *const ()), // sub_142405190
    Replacement::draft(0x1424051d0, "r_1424051d0", tpl_49801::<0x50, 0x8, 0x1426d01e0> as *const ()), // sub_1424051d0
    Replacement::draft(0x142405230, "r_142405230", fn_142405230 as *const ()), // sub_142405230
    Replacement::draft(0x142405290, "r_142405290", fn_142405290 as *const ()), // sub_142405290
    Replacement::draft(0x1424052c0, "r_1424052c0", tpl_45951::<0x8, 0x10> as *const ()), // sub_1424052c0
    Replacement::draft(0x142405310, "r_142405310", tpl_49815::<0x8, 0x10, 0x142405640> as *const ()), // sub_142405310
    Replacement::draft(0x142405340, "r_142405340", tpl_4127::<0x140f366d0> as *const ()), // sub_142405340
    Replacement::draft(0x1424053f0, "r_1424053f0", tpl_49799::<0x48, 0x70, 0x10, 0x8, 0x142405640> as *const ()), // sub_1424053f0
    Replacement::draft(0x142405430, "r_142405430", tpl_49800::<0x48, 0x70, 0x8, 0x10, 0x142405640> as *const ()), // sub_142405430
    Replacement::draft(0x142405470, "r_142405470", fn_142405470 as *const ()), // sub_142405470
    Replacement::draft(0x1424054d0, "r_1424054d0", fn_1424054d0 as *const ()), // sub_1424054d0
    Replacement::draft(0x142405550, "r_142405550", fn_142405550 as *const ()), // sub_142405550
    Replacement::draft(0x142405640, "r_142405640", tpl_108550::<0x8, 0x10, 0x1426d01e0, 0x58, 0x50, 0x38, 0x28, 0x142404ff0> as *const ()), // sub_142405640
    Replacement::draft(0x142405730, "r_142405730", fn_142405730 as *const ()), // sub_142405730
    Replacement::draft(0x142405820, "r_142405820", tpl_49799::<0x48, 0x70, 0x10, 0x8, 0x142405640> as *const ()), // sub_142405820
    Replacement::draft(0x142405860, "r_142405860", tpl_49800::<0x48, 0x70, 0x8, 0x10, 0x142405640> as *const ()), // sub_142405860
    Replacement::draft(0x1424058a0, "r_1424058a0", fn_1424058a0 as *const ()), // sub_1424058a0
    Replacement::draft_throws(0x142405990, "r_142405990", fn_142405990 as *const (), Throws::Discard), // sub_142405990
    Replacement::draft(0x1424059f0, "r_1424059f0", fn_1424059f0 as *const ()), // sub_1424059f0
    Replacement::draft(0x142405b40, "r_142405b40", tpl_49803::<0x8, 0x142405640> as *const ()), // sub_142405b40
    Replacement::draft(0x142405b60, "r_142405b60", tpl_49804::<0x8> as *const ()), // sub_142405b60
    Replacement::draft(0x142405c30, "r_142405c30", tpl_49832::<0x8, 0x1424037c0, 0xc, 0x10, 0x18> as *const ()), // sub_142405c30
    Replacement::draft(0x142405d00, "r_142405d00", tpl_49832::<0x8, 0x1424037c0, 0xc, 0x10, 0x18> as *const ()), // sub_142405d00
    Replacement::draft(0x142405dd0, "r_142405dd0", fn_142405dd0 as *const ()), // sub_142405dd0
    Replacement::draft(0x142405ec0, "r_142405ec0", fn_142405ec0 as *const ()), // sub_142405ec0
    Replacement::draft_throws(0x142405f70, "r_142405f70", fn_142405f70 as *const (), Throws::Caught), // sub_142405f70
    Replacement::draft(0x142406430, "r_142406430", fn_142406430 as *const ()), // sub_142406430
    Replacement::draft(0x142406460, "r_142406460", fn_142406460 as *const ()), // sub_142406460
    Replacement::draft(0x1424064f0, "r_1424064f0", fn_1424064f0 as *const ()), // sub_1424064f0
    Replacement::draft(0x142406540, "r_142406540", fn_142406540 as *const ()), // sub_142406540
    Replacement::draft(0x142406660, "r_142406660", fn_142406660 as *const ()), // sub_142406660
    Replacement::draft(0x1424066f0, "r_1424066f0", fn_1424066f0 as *const ()), // sub_1424066f0
    Replacement::draft(0x1424067a0, "r_1424067a0", fn_1424067a0 as *const ()), // sub_1424067a0
    Replacement::draft(0x142406810, "r_142406810", fn_142406810 as *const ()), // sub_142406810
    Replacement::draft(0x1424068a0, "r_1424068a0", fn_1424068a0 as *const ()), // sub_1424068a0
    Replacement::draft_throws(0x142406a80, "r_142406a80", fn_142406a80 as *const (), Throws::Discard), // sub_142406a80
    Replacement::draft(0x142406b80, "r_142406b80", fn_142406b80 as *const ()), // sub_142406b80
    Replacement::draft_throws(0x142406bb0, "r_142406bb0", fn_142406bb0 as *const (), Throws::Caught), // sub_142406bb0
    Replacement::draft_throws(0x142406c80, "r_142406c80", fn_142406c80 as *const (), Throws::Caught), // sub_142406c80
    Replacement::draft_throws(0x142406d60, "r_142406d60", fn_142406d60 as *const (), Throws::Caught), // sub_142406d60
    Replacement::draft_throws(0x142406e60, "r_142406e60", fn_142406e60 as *const (), Throws::Caught), // sub_142406e60
    Replacement::draft_throws(0x142406f60, "r_142406f60", fn_142406f60 as *const (), Throws::Discard), // sub_142406f60
    Replacement::draft_throws(0x142407070, "r_142407070", fn_142407070 as *const (), Throws::Caught), // sub_142407070
    Replacement::draft(0x142407330, "r_142407330", tpl_45161::<0x130> as *const ()), // sub_142407330
    Replacement::draft_throws(0x142407340, "r_142407340", fn_142407340 as *const (), Throws::Discard), // sub_142407340
    Replacement::draft(0x142407390, "r_142407390", fn_142407390 as *const ()), // sub_142407390
    Replacement::draft(0x1424074d0, "r_1424074d0", fn_1424074d0 as *const ()), // sub_1424074d0
    Replacement::draft(0x142407500, "r_142407500", fn_142407500 as *const ()), // sub_142407500
    Replacement::draft(0x142407560, "r_142407560", crate::shards::subsystems::fn_142407560 as *const ()), // sub_142407560
    Replacement::draft_throws(0x142407590, "r_142407590", fn_142407590 as *const (), Throws::Caught), // sub_142407590
    Replacement::draft_throws(0x1424076e0, "r_1424076e0", fn_1424076e0 as *const (), Throws::Discard), // sub_1424076e0
    Replacement::draft_throws(0x1424078a0, "r_1424078a0", fn_1424078a0 as *const (), Throws::Caught), // sub_1424078a0
    Replacement::draft(0x1424079e0, "r_1424079e0", fn_1424079e0 as *const ()), // sub_1424079e0
    Replacement::draft(0x142407a40, "r_142407a40", fn_142407a40 as *const ()), // sub_142407a40
    Replacement::draft(0x142407ae0, "r_142407ae0", fn_142407ae0 as *const ()), // sub_142407ae0
    Replacement::draft(0x142407b30, "r_142407b30", fn_142407b30 as *const ()), // sub_142407b30
    Replacement::draft(0x142407bd0, "r_142407bd0", fn_142407bd0 as *const ()), // sub_142407bd0
    Replacement::draft(0x142407c20, "r_142407c20", fn_142407c20 as *const ()), // sub_142407c20
    Replacement::draft(0x142407d40, "r_142407d40", tpl_49865 as *const ()), // sub_142407d40
    Replacement::draft(0x142407d90, "r_142407d90", fn_142407d90 as *const ()), // sub_142407d90
    Replacement::draft(0x142407dc0, "r_142407dc0", fn_142407dc0 as *const ()), // sub_142407dc0
    Replacement::draft(0x142407dd0, "r_142407dd0", tpl_49865 as *const ()), // sub_142407dd0
    Replacement::draft(0x142407e20, "r_142407e20", fn_142407e20 as *const ()), // sub_142407e20
    Replacement::draft(0x142407e50, "r_142407e50", fn_142407e50 as *const ()), // sub_142407e50
    Replacement::draft(0x142407e70, "r_142407e70", fn_142407e70 as *const ()), // sub_142407e70
    Replacement::draft(0x142407ec0, "r_142407ec0", fn_142407ec0 as *const ()), // sub_142407ec0
    Replacement::draft(0x142407ee0, "r_142407ee0", fn_142407ee0 as *const ()), // sub_142407ee0
    Replacement::draft(0x142407f20, "r_142407f20", fn_142407f20 as *const ()), // sub_142407f20
    Replacement::draft(0x142407f50, "r_142407f50", fn_142407f50 as *const ()), // sub_142407f50
    Replacement::draft(0x142407fa0, "r_142407fa0", fn_142407fa0 as *const ()), // sub_142407fa0
    Replacement::draft(0x142408010, "r_142408010", fn_142408010 as *const ()), // sub_142408010
    Replacement::draft(0x142408080, "r_142408080", tpl_49877::<0x8> as *const ()), // sub_142408080
    Replacement::draft_throws(0x142408090, "r_142408090", fn_142408090 as *const (), Throws::Caught), // sub_142408090
    Replacement::draft(0x1424081f0, "r_1424081f0", fn_1424081f0 as *const ()), // sub_1424081f0
    Replacement::draft(0x142408260, "r_142408260", fn_142408260 as *const ()), // sub_142408260
    Replacement::draft(0x142408280, "r_142408280", fn_142408280 as *const ()), // sub_142408280
    Replacement::draft(0x1424082a0, "r_1424082a0", fn_1424082a0 as *const ()), // sub_1424082a0
    Replacement::draft_throws(0x142408310, "r_142408310", fn_142408310 as *const (), Throws::Discard), // sub_142408310
    Replacement::draft(0x1424083c0, "r_1424083c0", tpl_49877::<0xc> as *const ()), // sub_1424083c0
    Replacement::draft(0x1424083d0, "r_1424083d0", fn_1424083d0 as *const ()), // sub_1424083d0
    Replacement::draft(0x142408440, "r_142408440", fn_142408440 as *const ()), // sub_142408440
    Replacement::draft(0x142408470, "r_142408470", fn_142408470 as *const ()), // sub_142408470
    Replacement::draft(0x1424086d0, "r_1424086d0", fn_1424086d0 as *const ()), // sub_1424086d0
    Replacement::draft(0x142408830, "r_142408830", fn_142408830 as *const ()), // sub_142408830
    Replacement::draft(0x142408990, "r_142408990", fn_142408990 as *const ()), // sub_142408990
    Replacement::draft(0x142408af0, "r_142408af0", fn_142408af0 as *const ()), // sub_142408af0
    Replacement::draft(0x142408c50, "r_142408c50", fn_142408c50 as *const ()), // sub_142408c50
    Replacement::draft(0x142408c80, "r_142408c80", fn_142408c80 as *const ()), // sub_142408c80
    Replacement::draft(0x142408d40, "r_142408d40", fn_142408d40 as *const ()), // sub_142408d40
    Replacement::draft(0x142408e00, "r_142408e00", fn_142408e00 as *const ()), // sub_142408e00
    Replacement::draft(0x142408e10, "r_142408e10", fn_142408e10 as *const ()), // sub_142408e10
    Replacement::draft(0x142408ef0, "r_142408ef0", fn_142408ef0 as *const ()), // sub_142408ef0
    Replacement::draft_throws(0x142409700, "r_142409700", fn_142409700 as *const (), Throws::Discard), // sub_142409700
    Replacement::draft(0x14240a1d0, "r_14240a1d0", fn_14240a1d0 as *const ()), // sub_14240a1d0
    Replacement::draft(0x14240ac00, "r_14240ac00", tpl_49814::<0x8, 0x1424b16a0> as *const ()), // sub_14240ac00
    Replacement::draft_throws(0x14240ac30, "r_14240ac30", fn_14240ac30 as *const (), Throws::Caught), // sub_14240ac30
    Replacement::draft(0x14240aea0, "r_14240aea0", fn_14240aea0 as *const ()), // sub_14240aea0
    Replacement::draft_throws(0x14240b140, "r_14240b140", fn_14240b140 as *const (), Throws::Discard), // sub_14240b140
    Replacement::draft(0x14240b210, "r_14240b210", fn_14240b210 as *const ()), // sub_14240b210
    Replacement::draft(0x14240b250, "r_14240b250", fn_14240b250 as *const ()), // sub_14240b250
    Replacement::draft_throws(0x14240b380, "r_14240b380", fn_14240b380 as *const (), Throws::Discard), // sub_14240b380
    Replacement::draft(0x14240b560, "r_14240b560", fn_14240b560 as *const ()), // sub_14240b560
    Replacement::draft_throws(0x14240b5f0, "r_14240b5f0", fn_14240b5f0 as *const (), Throws::Discard), // sub_14240b5f0
    Replacement::draft(0x14240b630, "r_14240b630", fn_14240b630 as *const ()), // sub_14240b630
    Replacement::draft_throws(0x14240b750, "r_14240b750", fn_14240b750 as *const (), Throws::Discard), // sub_14240b750
    Replacement::draft(0x14240b930, "r_14240b930", fn_14240b930 as *const ()), // sub_14240b930
    Replacement::draft(0x14240b9a0, "r_14240b9a0", fn_14240b9a0 as *const ()), // sub_14240b9a0
    Replacement::draft(0x14240ba10, "r_14240ba10", fn_14240ba10 as *const ()), // sub_14240ba10
    Replacement::draft(0x14240bb70, "r_14240bb70", fn_14240bb70 as *const ()), // sub_14240bb70
    Replacement::draft(0x14240be20, "r_14240be20", fn_14240be20 as *const ()), // sub_14240be20
    Replacement::draft(0x14240c3f0, "r_14240c3f0", fn_14240c3f0 as *const ()), // r::StreamingFile::ChunkResource::r::StreamingFile::ChunkResource::vfunc_2
    Replacement::draft(0x14240c430, "r_14240c430", fn_14240c430 as *const ()), // r::StreamingFile::ChunkResource::r::StreamingFile::ChunkResource::vfunc_1
    Replacement::draft(0x14240c500, "r_14240c500", fn_14240c500 as *const ()), // sub_14240c500
    Replacement::draft(0x14240c570, "r_14240c570", fn_14240c570 as *const ()), // sub_14240c570
    Replacement::draft(0x14240c660, "r_14240c660", fn_14240c660 as *const ()), // r::StreamingFile::r::StreamingFile::vfunc_7
    Replacement::draft(0x14240c6a0, "r_14240c6a0", fn_14240c6a0 as *const ()), // r::StreamingFile::r::StreamingFile::vfunc_5
    Replacement::draft(0x14240c7b0, "r_14240c7b0", fn_14240c7b0 as *const ()), // sub_14240c7b0
    Replacement::draft(0x14240c870, "r_14240c870", fn_14240c870 as *const ()), // sub_14240c870
    Replacement::draft(0x14240c930, "r_14240c930", fn_14240c930 as *const ()), // sub_14240c930
    Replacement::draft(0x14240d100, "r_14240d100", fn_14240d100 as *const ()), // sub_14240d100
    Replacement::draft_throws(0x14240d230, "r_14240d230", fn_14240d230 as *const (), Throws::Discard), // sub_14240d230
    Replacement::draft(0x14240d480, "r_14240d480", fn_14240d480 as *const ()), // sub_14240d480
    Replacement::draft(0x14240d5c0, "r_14240d5c0", fn_14240d5c0 as *const ()), // sub_14240d5c0
    Replacement::draft_throws(0x14240d6a0, "r_14240d6a0", fn_14240d6a0 as *const (), Throws::Caught), // sub_14240d6a0
    Replacement::draft_throws(0x14240d6e0, "r_14240d6e0", fn_14240d6e0 as *const (), Throws::Caught), // sub_14240d6e0
    Replacement::draft(0x14240d9d0, "r_14240d9d0", fn_14240d9d0 as *const ()), // sub_14240d9d0
    Replacement::draft(0x14240da50, "r_14240da50", fn_14240da50 as *const ()), // sub_14240da50
    Replacement::draft(0x14240e2b0, "r_14240e2b0", fn_14240e2b0 as *const ()), // sub_14240e2b0
    Replacement::draft(0x14240f6c0, "r_14240f6c0", fn_14240f6c0 as *const ()), // sub_14240f6c0
    Replacement::draft(0x14240f700, "r_14240f700", fn_14240f700 as *const ()), // sub_14240f700
    Replacement::draft(0x14240f720, "r_14240f720", fn_14240f720 as *const ()), // sub_14240f720
    Replacement::draft(0x14240f7e0, "r_14240f7e0", fn_14240f7e0 as *const ()), // sub_14240f7e0
    Replacement::draft(0x14240f860, "r_14240f860", fn_14240f860 as *const ()), // sub_14240f860
    Replacement::draft(0x14240fb00, "r_14240fb00", fn_14240fb00 as *const ()), // sub_14240fb00
    Replacement::draft(0x14240fc30, "r_14240fc30", fn_14240fc30 as *const ()), // sub_14240fc30
    Replacement::draft(0x14240fc80, "r_14240fc80", fn_14240fc80 as *const ()), // sub_14240fc80
    Replacement::draft(0x14240fe90, "r_14240fe90", fn_14240fe90 as *const ()), // sub_14240fe90
    Replacement::draft(0x14240ff60, "r_14240ff60", fn_14240ff60 as *const ()), // sub_14240ff60
    Replacement::draft(0x142410050, "r_142410050", fn_142410050 as *const ()), // sub_142410050
    Replacement::draft(0x142410280, "r_142410280", fn_142410280 as *const ()), // sub_142410280
    Replacement::draft(0x1424109c0, "r_1424109c0", fn_1424109c0 as *const ()), // sub_1424109c0
    Replacement::draft(0x142410ff0, "r_142410ff0", fn_142410ff0 as *const ()), // sub_142410ff0
    Replacement::draft(0x142411020, "r_142411020", fn_142411020 as *const ()), // sub_142411020
    Replacement::draft(0x142411060, "r_142411060", fn_142411060 as *const ()), // sub_142411060
    Replacement::draft(0x1424110a0, "r_1424110a0", fn_1424110a0 as *const ()), // sub_1424110a0
    Replacement::draft_throws(0x142411130, "r_142411130", tpl_838::<0x1401ecac0> as *const (), Throws::Discard), // r::StateMachineState::r::StateMachineState::vfunc_8
    Replacement::draft(0x142411b70, "r_142411b70", fn_142411b70 as *const ()), // sub_142411b70
    Replacement::draft(0x142411bf0, "r_142411bf0", tpl_28265::<0x143407c40, 0x8> as *const ()), // sub_142411bf0
    Replacement::draft(0x142411c10, "r_142411c10", fn_142411c10 as *const ()), // sub_142411c10
    Replacement::draft(0x142411c40, "r_142411c40", tpl_49944::<0x10> as *const ()), // sub_142411c40
    Replacement::draft(0x142411c60, "r_142411c60", tpl_30771::<0x8, 0x10> as *const ()), // sub_142411c60
    Replacement::draft(0x142411c80, "r_142411c80", tpl_49944::<0x10> as *const ()), // sub_142411c80
    Replacement::draft(0x142411ca0, "r_142411ca0", fn_142411ca0 as *const ()), // sub_142411ca0
    Replacement::draft(0x142411ce0, "r_142411ce0", fn_142411ce0 as *const ()), // sub_142411ce0
    Replacement::draft(0x142411cf0, "r_142411cf0", fn_142411cf0 as *const ()), // sub_142411cf0
    Replacement::draft(0x142411d50, "r_142411d50", tpl_49948::<0x10, 0x18> as *const ()), // sub_142411d50
    Replacement::draft(0x142411d80, "r_142411d80", tpl_49948::<0x10, 0x18> as *const ()), // sub_142411d80
    Replacement::draft(0x142412cc0, "r_142412cc0", fn_142412cc0 as *const ()), // r::StateMachineDefinition::State::ClassTypeInfoImplementation::r::StateMachineDefinition::State::ClassTypeInfoImplementa
    Replacement::draft(0x142412d30, "r_142412d30", fn_142412d30 as *const ()), // r::StateMachineDefinition::State::ClassTypeInfoImplementation::r::StateMachineDefinition::State::ClassTypeInfoImplementa
    Replacement::draft(0x142412d70, "r_142412d70", fn_142412d70 as *const ()), // r::StateMachineDefinition::State::ClassTypeInfoImplementation::r::StateMachineDefinition::State::ClassTypeInfoImplementa
    Replacement::draft(0x142412da0, "r_142412da0", fn_142412da0 as *const ()), // r::StateMachineDefinition::State::ClassTypeInfoImplementation::r::StateMachineDefinition::State::ClassTypeInfoImplementa
    Replacement::draft(0x142412e20, "r_142412e20", fn_142412e20 as *const ()), // r::StateMachineDefinition::State::ClassTypeInfoImplementation::r::StateMachineDefinition::State::ClassTypeInfoImplementa
    Replacement::draft_throws(0x1424131a0, "r_1424131a0", tpl_39950::<0x143881f18, 0x143881f10, 0x1401e9b60, 0x14137e9c0, 0x1401e9a20> as *const (), Throws::Discard), // r::StateMachineDefinition::StateGroup::ClassTypeInfoImplementation::r::StateMachineDefinition::StateGroup::ClassTypeInfo
    Replacement::draft_throws(0x142413200, "r_142413200", tpl_39950::<0x143881f18, 0x143881f10, 0x1401e9b60, 0x14137e9c0, 0x1401e9a20> as *const (), Throws::Discard), // r::StateMachineDefinition::StateGroup::ClassTypeInfoImplementation::r::StateMachineDefinition::StateGroup::ClassTypeInfo
    Replacement::draft_throws(0x1424132a0, "r_1424132a0", tpl_39950::<0x143881f18, 0x143881f10, 0x1401e9fa0, 0x14137e5b0, 0x1401e9e70> as *const (), Throws::Discard), // r::StateMachineDefinition::StateGroup::ClassTypeInfoImplementation::r::StateMachineDefinition::StateGroup::ClassTypeInfo
    Replacement::draft_throws(0x142413300, "r_142413300", tpl_39950::<0x143881f18, 0x143881f10, 0x1401e9fa0, 0x14137e5b0, 0x1401e9e70> as *const (), Throws::Discard), // r::StateMachineDefinition::StateGroup::ClassTypeInfoImplementation::r::StateMachineDefinition::StateGroup::ClassTypeInfo
    Replacement::draft(0x142413d90, "r_142413d90", tpl_38844::<0x143881ef8, 0x143881ef0, 0x1401e9970, 0x338, 0xe0, 0x1401e9860> as *const ()), // r::StateMachineDefinition::ID::ClassTypeInfoImplementation::r::StateMachineDefinition::ID::ClassTypeInfoImplementation::
    Replacement::draft(0x142413df0, "r_142413df0", tpl_38844::<0x143881ef8, 0x143881ef0, 0x1401e9970, 0x338, 0xe0, 0x1401e9860> as *const ()), // r::StateMachineDefinition::ID::ClassTypeInfoImplementation::r::StateMachineDefinition::ID::ClassTypeInfoImplementation::
    Replacement::draft(0x1424146f0, "r_1424146f0", tpl_39509::<0x10, 0x14137e320, 0x1401de830> as *const ()), // r::StateMachineDefinition::ClassTypeInfoImplementation::r::StateMachineDefinition::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x142414720, "r_142414720", fn_142414720 as *const ()), // r::StateMachineDefinition::ClassTypeInfoImplementation::r::StateMachineDefinition::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x142414770, "r_142414770", tpl_39510::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x14137e320, 0x1401de830, 0x142442c80> as *const ()), // r::StateMachineDefinition::ClassTypeInfoImplementation::r::StateMachineDefinition::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1424147e0, "r_1424147e0", tpl_38774::<0x10, 0x14137e320, 0x1401de830, 0x142442de0> as *const ()), // r::StateMachineDefinition::ClassTypeInfoImplementation::r::StateMachineDefinition::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1424148c0, "r_1424148c0", fn_1424148c0 as *const ()), // sub_1424148c0
    Replacement::draft(0x1424148d0, "r_1424148d0", fn_1424148d0 as *const ()), // sub_1424148d0
    Replacement::draft_throws(0x142414950, "r_142414950", fn_142414950 as *const (), Throws::Discard), // sub_142414950
    Replacement::draft(0x1424149d0, "r_1424149d0", fn_1424149d0 as *const ()), // sub_1424149d0
    Replacement::draft(0x142414a60, "r_142414a60", fn_142414a60 as *const ()), // sub_142414a60
    Replacement::draft(0x142414ad0, "r_142414ad0", tpl_49960::<0x10, 0x4> as *const ()), // sub_142414ad0
    Replacement::draft(0x142414af0, "r_142414af0", fn_142414af0 as *const ()), // sub_142414af0
    Replacement::draft(0x142414bb0, "r_142414bb0", tpl_49960::<0x10, 0x4> as *const ()), // sub_142414bb0
    Replacement::draft(0x142414bd0, "r_142414bd0", fn_142414bd0 as *const ()), // sub_142414bd0
    Replacement::draft(0x142414c80, "r_142414c80", tpl_34045::<0x20> as *const ()), // sub_142414c80
    Replacement::draft(0x142414c90, "r_142414c90", tpl_49963::<0x50, 0x10> as *const ()), // sub_142414c90
    Replacement::draft(0x142414cb0, "r_142414cb0", tpl_49963::<0x38, 0x10> as *const ()), // sub_142414cb0
    Replacement::draft(0x142414cd0, "r_142414cd0", fn_142414cd0 as *const ()), // sub_142414cd0
    Replacement::draft(0x142414db0, "r_142414db0", fn_142414db0 as *const ()), // sub_142414db0
    Replacement::draft(0x142415150, "r_142415150", tpl_42593 as *const ()), // sub_142415150
    Replacement::draft(0x142415190, "r_142415190", fn_142415190 as *const ()), // sub_142415190
    Replacement::draft(0x1424151f0, "r_1424151f0", fn_1424151f0 as *const ()), // sub_1424151f0
    Replacement::draft(0x142415290, "r_142415290", fn_142415290 as *const ()), // sub_142415290
    Replacement::draft(0x142415330, "r_142415330", fn_142415330 as *const ()), // sub_142415330
    Replacement::draft(0x1424153e0, "r_1424153e0", fn_1424153e0 as *const ()), // sub_1424153e0
    Replacement::draft(0x142415490, "r_142415490", tpl_49971::<0x40, 0x48, 0x18, 0x20> as *const ()), // sub_142415490
    Replacement::draft(0x1424154d0, "r_1424154d0", tpl_49971::<0x40, 0x48, 0x8, 0x10> as *const ()), // sub_1424154d0
    Replacement::draft(0x142415510, "r_142415510", fn_142415510 as *const ()), // sub_142415510
    Replacement::draft(0x142415860, "r_142415860", fn_142415860 as *const ()), // sub_142415860
    Replacement::draft_throws(0x142415930, "r_142415930", tpl_39967::<0x1413970f0, 0x142415946> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415950, "r_142415950", tpl_39967::<0x1413970e0, 0x142415966> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415a70, "r_142415a70", tpl_39967::<0x141396e20, 0x142415a86> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415a90, "r_142415a90", tpl_39967::<0x141396e10, 0x142415aa6> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415ab0, "r_142415ab0", tpl_39967::<0x141396db0, 0x142415ac6> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415ad0, "r_142415ad0", tpl_39967::<0x141396da0, 0x142415ae6> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415af0, "r_142415af0", tpl_39967::<0x141396d40, 0x142415b06> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x142415b10, "r_142415b10", tpl_39967::<0x141396d30, 0x142415b26> as *const (), Throws::Discard), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft(0x142415d00, "r_142415d00", fn_142415d00 as *const ()), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft(0x142415d60, "r_142415d60", fn_142415d60 as *const ()), // r::StateMachine::StateStorage::ClassTypeInfoImplementation::r::StateMachine::StateStorage::ClassTypeInfoImplementation::
    Replacement::draft(0x142415e90, "r_142415e90", tpl_49976::<0x143408b78, 0x1, 0x14020f270> as *const ()), // r::StateMachine::ClassTypeInfoImplementation::r::StateMachine::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x142415ed0, "r_142415ed0", tpl_49976::<0x143408b78, 0x1, 0x14020f270> as *const ()), // r::StateMachine::ClassTypeInfoImplementation::r::StateMachine::ClassTypeInfoImplementation::vfunc_37
    Replacement::draft(0x142416330, "r_142416330", fn_142416330 as *const ()), // r::StateMachine::ClassTypeInfoImplementation::r::StateMachine::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1424163d0, "r_1424163d0", tpl_107277::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x142416d60, 0x142442c80> as *const ()), // r::StateMachine::ClassTypeInfoImplementation::r::StateMachine::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x142416430, "r_142416430", tpl_48628::<0x142416d60, 0x142442de0> as *const ()), // r::StateMachine::ClassTypeInfoImplementation::r::StateMachine::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x142416500, "r_142416500", fn_142416500 as *const ()), // sub_142416500
    Replacement::draft(0x1424165a0, "r_1424165a0", fn_1424165a0 as *const ()), // sub_1424165a0
    Replacement::draft(0x142416650, "r_142416650", fn_142416650 as *const ()), // sub_142416650
    Replacement::draft(0x142416810, "r_142416810", fn_142416810 as *const ()), // sub_142416810
    Replacement::draft(0x1424169d0, "r_1424169d0", fn_1424169d0 as *const ()), // sub_1424169d0
    Replacement::draft_throws(0x142416bf0, "r_142416bf0", fn_142416bf0 as *const (), Throws::Discard), // sub_142416bf0
    Replacement::draft(0x142416d60, "r_142416d60", fn_142416d60 as *const ()), // sub_142416d60
    Replacement::draft(0x142416e70, "r_142416e70", fn_142416e70 as *const ()), // sub_142416e70
    Replacement::draft(0x142416f10, "r_142416f10", fn_142416f10 as *const ()), // sub_142416f10
    Replacement::draft(0x142417150, "r_142417150", fn_142417150 as *const ()), // sub_142417150
    Replacement::draft(0x1424171a0, "r_1424171a0", fn_1424171a0 as *const ()), // sub_1424171a0
    Replacement::draft(0x142417200, "r_142417200", tpl_49987::<0xc0, 0xc8, 0x8, 0x28, 0x20> as *const ()), // sub_142417200
    Replacement::draft(0x142417250, "r_142417250", tpl_49987::<0xc0, 0xc8, 0x8, 0x18, 0x10> as *const ()), // sub_142417250
    Replacement::draft(0x1424172a0, "r_1424172a0", fn_1424172a0 as *const ()), // sub_1424172a0
    Replacement::draft(0x142417380, "r_142417380", fn_142417380 as *const ()), // sub_142417380
    Replacement::draft(0x142417630, "r_142417630", fn_142417630 as *const ()), // sub_142417630
    Replacement::draft(0x142417680, "r_142417680", fn_142417680 as *const ()), // sub_142417680
    Replacement::draft(0x1424176d0, "r_1424176d0", fn_1424176d0 as *const ()), // sub_1424176d0
    Replacement::draft_throws(0x1424177c0, "r_1424177c0", fn_1424177c0 as *const (), Throws::Caught), // sub_1424177c0
    Replacement::draft(0x1424178d0, "r_1424178d0", fn_1424178d0 as *const ()), // sub_1424178d0
    Replacement::draft(0x1424178e0, "r_1424178e0", tpl_49994::<0x142646438, 0x142646430> as *const ()), // sub_1424178e0
    Replacement::draft(0x142417920, "r_142417920", fn_142417920 as *const ()), // sub_142417920
    Replacement::draft(0x1424179a0, "r_1424179a0", tpl_49994::<0x142646438, 0x142646430> as *const ()), // sub_1424179a0
    Replacement::draft(0x1424179e0, "r_1424179e0", tpl_49996 as *const ()), // sub_1424179e0
    Replacement::draft_throws(0x1424179f0, "r_1424179f0", fn_1424179f0 as *const (), Throws::Discard), // r::SmallBlockMemoryResource::r::SmallBlockMemoryResource::vfunc_2
    Replacement::draft(0x142417a90, "r_142417a90", tpl_49996 as *const ()), // sub_142417a90
    Replacement::draft_throws(0x142417aa0, "r_142417aa0", fn_142417aa0 as *const (), Throws::Discard), // r::SmallBlockMemoryResource::r::SmallBlockMemoryResource::vfunc_1
    Replacement::draft(0x142417ca0, "r_142417ca0", fn_142417ca0 as *const ()), // sub_142417ca0
    Replacement::draft(0x142417d60, "r_142417d60", fn_142417d60 as *const ()), // sub_142417d60
    Replacement::draft_throws(0x142417e60, "r_142417e60", fn_142417e60 as *const (), Throws::Discard), // sub_142417e60
    Replacement::draft_throws(0x142418000, "r_142418000", fn_142418000 as *const (), Throws::Discard), // sub_142418000
    Replacement::draft(0x142418140, "r_142418140", fn_142418140 as *const ()), // sub_142418140
    Replacement::draft(0x1424182a0, "r_1424182a0", fn_1424182a0 as *const ()), // sub_1424182a0
    Replacement::draft(0x1424182c0, "r_1424182c0", tpl_50003::<0x338, 0xe8, 0x340, 0x30> as *const ()), // sub_1424182c0
    Replacement::draft(0x142418320, "r_142418320", tpl_50004::<0x338, 0x120, 0x340> as *const ()), // sub_142418320
    Replacement::draft(0x142418370, "r_142418370", tpl_50003::<0x338, 0xe8, 0x340, 0x30> as *const ()), // sub_142418370
    Replacement::draft(0x142418480, "r_142418480", fn_142418480 as *const ()), // sub_142418480
    Replacement::draft(0x142418490, "r_142418490", tpl_50003::<0x338, 0xe8, 0x340, 0x30> as *const ()), // sub_142418490
    Replacement::draft(0x1424184f0, "r_1424184f0", tpl_50004::<0x338, 0x120, 0x340> as *const ()), // sub_1424184f0
    Replacement::draft(0x142418540, "r_142418540", tpl_50003::<0x338, 0xe8, 0x340, 0x30> as *const ()), // sub_142418540
    Replacement::draft(0x1424185b0, "r_1424185b0", tpl_2460::<0x338, 0x1426d4580, 0x1> as *const ()), // sub_1424185b0
    Replacement::draft_throws(0x142418680, "r_142418680", fn_142418680 as *const (), Throws::Caught), // sub_142418680
    Replacement::draft_throws(0x142418720, "r_142418720", fn_142418720 as *const (), Throws::Caught), // sub_142418720
    Replacement::draft_throws(0x1424187d0, "r_1424187d0", fn_1424187d0 as *const (), Throws::Caught), // sub_1424187d0
    Replacement::draft(0x142418930, "r_142418930", fn_142418930 as *const ()), // sub_142418930
    Replacement::draft(0x142418970, "r_142418970", tpl_50010::<0x338, 0xe8, 0x340, 0x30> as *const ()), // sub_142418970
    Replacement::draft(0x1424189d0, "r_1424189d0", fn_1424189d0 as *const ()), // sub_1424189d0
    Replacement::draft(0x142418a20, "r_142418a20", tpl_50010::<0x338, 0xe8, 0x340, 0x30> as *const ()), // sub_142418a20
    Replacement::draft(0x142418ab0, "r_142418ab0", tpl_48416::<0x143a082a8, 0x14139d6b0, 0x142442de0> as *const ()), // sub_142418ab0
    Replacement::draft(0x142418af0, "r_142418af0", fn_142418af0 as *const ()), // sub_142418af0
    Replacement::draft(0x142418b30, "r_142418b30", fn_142418b30 as *const ()), // sub_142418b30
    Replacement::draft(0x142418c00, "r_142418c00", fn_142418c00 as *const ()), // sub_142418c00
    Replacement::draft(0x142418c80, "r_142418c80", fn_142418c80 as *const ()), // sub_142418c80
    Replacement::draft(0x142418d70, "r_142418d70", fn_142418d70 as *const ()), // sub_142418d70
    Replacement::draft(0x142418e30, "r_142418e30", fn_142418e30 as *const ()), // sub_142418e30
    Replacement::draft(0x142418e70, "r_142418e70", fn_142418e70 as *const ()), // sub_142418e70
    Replacement::draft(0x142418ea0, "r_142418ea0", fn_142418ea0 as *const ()), // sub_142418ea0
    Replacement::draft(0x142418ec0, "r_142418ec0", fn_142418ec0 as *const ()), // sub_142418ec0
    Replacement::draft(0x142418f00, "r_142418f00", fn_142418f00 as *const ()), // sub_142418f00
    Replacement::draft(0x142419bc0, "r_142419bc0", fn_142419bc0 as *const ()), // sub_142419bc0
    Replacement::draft(0x14241a4d0, "r_14241a4d0", fn_14241a4d0 as *const ()), // sub_14241a4d0
    Replacement::draft(0x14241b3a0, "r_14241b3a0", fn_14241b3a0 as *const ()), // sub_14241b3a0
    Replacement::draft(0x14241b540, "r_14241b540", fn_14241b540 as *const ()), // sub_14241b540
    Replacement::draft(0x14241bab0, "r_14241bab0", fn_14241bab0 as *const ()), // sub_14241bab0
    Replacement::draft_throws(0x14241bd10, "r_14241bd10", fn_14241bd10 as *const (), Throws::Discard), // sub_14241bd10
    Replacement::draft(0x14241be80, "r_14241be80", fn_14241be80 as *const ()), // sub_14241be80
    Replacement::draft_throws(0x14241bec0, "r_14241bec0", fn_14241bec0 as *const (), Throws::Discard), // sub_14241bec0
    Replacement::draft_throws(0x14241bff0, "r_14241bff0", fn_14241bff0 as *const (), Throws::Discard), // sub_14241bff0
    Replacement::draft_throws(0x14241c290, "r_14241c290", fn_14241c290 as *const (), Throws::Discard), // sub_14241c290
    Replacement::draft_throws(0x14241c2c0, "r_14241c2c0", fn_14241c2c0 as *const (), Throws::Discard), // sub_14241c2c0
    Replacement::draft(0x14241c400, "r_14241c400", fn_14241c400 as *const ()), // sub_14241c400
    Replacement::draft(0x14241c460, "r_14241c460", fn_14241c460 as *const ()), // sub_14241c460
    Replacement::draft(0x14241c8b0, "r_14241c8b0", fn_14241c8b0 as *const ()), // sub_14241c8b0
    Replacement::draft(0x14241c910, "r_14241c910", fn_14241c910 as *const ()), // sub_14241c910
    Replacement::draft(0x14241c980, "r_14241c980", fn_14241c980 as *const ()), // sub_14241c980
    Replacement::draft(0x14241ca40, "r_14241ca40", fn_14241ca40 as *const ()), // sub_14241ca40
    Replacement::draft(0x14241cac0, "r_14241cac0", fn_14241cac0 as *const ()), // sub_14241cac0
    Replacement::draft(0x14241cb10, "r_14241cb10", fn_14241cb10 as *const ()), // sub_14241cb10
    Replacement::draft(0x14241cba0, "r_14241cba0", fn_14241cba0 as *const ()), // sub_14241cba0
    Replacement::draft(0x14241cbc0, "r_14241cbc0", fn_14241cbc0 as *const ()), // sub_14241cbc0
    Replacement::draft(0x14241cc30, "r_14241cc30", fn_14241cc30 as *const ()), // sub_14241cc30
    Replacement::draft(0x14241cdb0, "r_14241cdb0", fn_14241cdb0 as *const ()), // sub_14241cdb0
    Replacement::draft(0x14241ce70, "r_14241ce70", fn_14241ce70 as *const ()), // sub_14241ce70
    Replacement::draft_throws(0x14241cf50, "r_14241cf50", fn_14241cf50 as *const (), Throws::Discard), // sub_14241cf50
    Replacement::draft_throws(0x14241d110, "r_14241d110", fn_14241d110 as *const (), Throws::Discard), // sub_14241d110
    Replacement::draft(0x14241d180, "r_14241d180", fn_14241d180 as *const ()), // sub_14241d180
    Replacement::draft(0x14241d4d0, "r_14241d4d0", fn_14241d4d0 as *const ()), // sub_14241d4d0
    Replacement::draft(0x14241d500, "r_14241d500", fn_14241d500 as *const ()), // sub_14241d500
    Replacement::draft_throws(0x14241db90, "r_14241db90", fn_14241db90 as *const (), Throws::Discard), // sub_14241db90
    Replacement::draft_throws(0x14241dc90, "r_14241dc90", fn_14241dc90 as *const (), Throws::Caught), // sub_14241dc90
    Replacement::draft(0x14241df40, "r_14241df40", fn_14241df40 as *const ()), // sub_14241df40
    Replacement::draft(0x14241df70, "r_14241df70", tpl_28645::<0x68, 0x28> as *const ()), // sub_14241df70
    Replacement::draft(0x14241df90, "r_14241df90", fn_14241df90 as *const ()), // sub_14241df90
    Replacement::draft(0x14241e120, "r_14241e120", tpl_4452::<0x8, 0x1401de830> as *const ()), // sub_14241e120
    Replacement::draft(0x14241e170, "r_14241e170", tpl_45951::<0x8, 0x10> as *const ()), // sub_14241e170
    Replacement::draft_throws(0x14241e2f0, "r_14241e2f0", fn_14241e2f0 as *const (), Throws::Discard), // r::JobCallable<r::MetadataProvider::reload::<lambda_1>>::r::JobCallable<r::MetadataProvider::reload::<lambda_1>>::vfunc_
    Replacement::draft(0x14241e560, "r_14241e560", fn_14241e560 as *const ()), // sub_14241e560
    Replacement::draft_throws(0x14241e610, "r_14241e610", fn_14241e610 as *const (), Throws::Caught), // sub_14241e610
    Replacement::draft_throws(0x14241e8f0, "r_14241e8f0", fn_14241e8f0 as *const (), Throws::Caught), // sub_14241e8f0
    Replacement::draft(0x14241f070, "r_14241f070", fn_14241f070 as *const ()), // sub_14241f070
    Replacement::draft_throws(0x14241f150, "r_14241f150", fn_14241f150 as *const (), Throws::Discard), // sub_14241f150
    Replacement::draft_throws(0x14241f1b0, "r_14241f1b0", tpl_50059::<0x10, 0x18, 0x142420c10, 0x142424b30> as *const (), Throws::Discard), // sub_14241f1b0
    Replacement::draft(0x14241f1f0, "r_14241f1f0", fn_14241f1f0 as *const ()), // sub_14241f1f0
    Replacement::draft(0x14241f310, "r_14241f310", fn_14241f310 as *const ()), // sub_14241f310
    Replacement::draft(0x14241f6a0, "r_14241f6a0", fn_14241f6a0 as *const ()), // sub_14241f6a0
    Replacement::draft_throws(0x14241f7a0, "r_14241f7a0", fn_14241f7a0 as *const (), Throws::Caught), // sub_14241f7a0
    Replacement::draft(0x14241fc40, "r_14241fc40", fn_14241fc40 as *const ()), // sub_14241fc40
    Replacement::draft_throws(0x14241fcb0, "r_14241fcb0", fn_14241fcb0 as *const (), Throws::Discard), // sub_14241fcb0
    Replacement::draft_throws(0x14241fdf0, "r_14241fdf0", fn_14241fdf0 as *const (), Throws::Caught), // sub_14241fdf0
    Replacement::draft_throws(0x142420b10, "r_142420b10", fn_142420b10 as *const (), Throws::Discard), // sub_142420b10
    Replacement::draft_throws(0x142420ba0, "r_142420ba0", fn_142420ba0 as *const (), Throws::Discard), // sub_142420ba0
    Replacement::draft_throws(0x142420c90, "r_142420c90", fn_142420c90 as *const (), Throws::Discard), // sub_142420c90
    Replacement::draft(0x142420d00, "r_142420d00", fn_142420d00 as *const ()), // sub_142420d00
    Replacement::draft(0x142420dd0, "r_142420dd0", fn_142420dd0 as *const ()), // sub_142420dd0
    Replacement::draft(0x142420ed0, "r_142420ed0", fn_142420ed0 as *const ()), // sub_142420ed0
    Replacement::draft(0x142420fd0, "r_142420fd0", fn_142420fd0 as *const ()), // sub_142420fd0
    Replacement::draft_throws(0x142421030, "r_142421030", fn_142421030 as *const (), Throws::Discard), // sub_142421030
    Replacement::draft_throws(0x142421080, "r_142421080", fn_142421080 as *const (), Throws::Caught), // sub_142421080
    Replacement::draft_throws(0x1424211e0, "r_1424211e0", fn_1424211e0 as *const (), Throws::Caught), // sub_1424211e0
    Replacement::draft_throws(0x142421300, "r_142421300", fn_142421300 as *const (), Throws::Discard), // sub_142421300
    Replacement::draft_throws(0x142421340, "r_142421340", fn_142421340 as *const (), Throws::Caught), // sub_142421340
    Replacement::draft_throws(0x142421490, "r_142421490", fn_142421490 as *const (), Throws::Discard), // sub_142421490
    Replacement::draft_throws(0x1424215e0, "r_1424215e0", fn_1424215e0 as *const (), Throws::Caught), // sub_1424215e0
    Replacement::draft_throws(0x142421bf0, "r_142421bf0", fn_142421bf0 as *const (), Throws::Discard), // sub_142421bf0
    Replacement::draft_throws(0x142421c60, "r_142421c60", fn_142421c60 as *const (), Throws::Discard), // sub_142421c60
    Replacement::draft(0x142421d10, "r_142421d10", fn_142421d10 as *const ()), // sub_142421d10
    Replacement::draft_throws(0x142421d30, "r_142421d30", fn_142421d30 as *const (), Throws::Discard), // sub_142421d30
    Replacement::draft_throws(0x142421d70, "r_142421d70", tpl_50059::<0x10, 0x18, 0x142420c10, 0x1424248a0> as *const (), Throws::Discard), // sub_142421d70
    Replacement::draft_throws(0x142421db0, "r_142421db0", fn_142421db0 as *const (), Throws::Discard), // sub_142421db0
    Replacement::draft(0x142421e20, "r_142421e20", fn_142421e20 as *const ()), // sub_142421e20
    Replacement::draft_throws(0x142421e50, "r_142421e50", fn_142421e50 as *const (), Throws::Discard), // sub_142421e50
    Replacement::draft_throws(0x142421fe0, "r_142421fe0", fn_142421fe0 as *const (), Throws::Discard), // sub_142421fe0
    Replacement::draft_throws(0x142422050, "r_142422050", fn_142422050 as *const (), Throws::Discard), // sub_142422050
    Replacement::draft(0x142422070, "r_142422070", fn_142422070 as *const ()), // sub_142422070
    Replacement::draft(0x142422130, "r_142422130", tpl_34269::<0x143409090, 0x18, 0x14243ef10, 0x142442de0> as *const ()), // sub_142422130
    Replacement::draft(0x142422170, "r_142422170", fn_142422170 as *const ()), // sub_142422170
    Replacement::draft(0x142422250, "r_142422250", fn_142422250 as *const ()), // sub_142422250
    Replacement::draft(0x142422ca0, "r_142422ca0", fn_142422ca0 as *const ()), // sub_142422ca0
    Replacement::draft_throws(0x142422d00, "r_142422d00", fn_142422d00 as *const (), Throws::Discard), // sub_142422d00
    Replacement::draft_throws(0x1424230a0, "r_1424230a0", fn_1424230a0 as *const (), Throws::Discard), // sub_1424230a0
    Replacement::draft(0x1424232a0, "r_1424232a0", fn_1424232a0 as *const ()), // sub_1424232a0
    Replacement::draft_throws(0x142423390, "r_142423390", fn_142423390 as *const (), Throws::Discard), // sub_142423390
    Replacement::draft_throws(0x142423420, "r_142423420", fn_142423420 as *const (), Throws::Caught), // sub_142423420
    Replacement::draft(0x142423640, "r_142423640", fn_142423640 as *const ()), // sub_142423640
    Replacement::draft_throws(0x1424236e0, "r_1424236e0", fn_1424236e0 as *const (), Throws::Discard), // sub_1424236e0
    Replacement::draft(0x142423b40, "r_142423b40", tpl_35199::<0x143881f70, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::ResourceID::ClassTypeInfoImplementation::r::ResourceID::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x142423ba0, "r_142423ba0", tpl_35199::<0x143881f70, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::ResourceID::ClassTypeInfoImplementation::r::ResourceID::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x142426c60, "r_142426c60", tpl_35199::<0x143881f80, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::Resource::ClassTypeInfoImplementation::r::Resource::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x142426d50, "r_142426d50", tpl_35199::<0x143881f80, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::Resource::ClassTypeInfoImplementation::r::Resource::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x142426f30, "r_142426f30", tpl_35199::<0x143881f80, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::Resource::ClassTypeInfoImplementation::r::Resource::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x142427020, "r_142427020", tpl_35199::<0x143881f80, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::Resource::ClassTypeInfoImplementation::r::Resource::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x1424270a0, "r_1424270a0", tpl_35199::<0x143881f80, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::Resource::ClassTypeInfoImplementation::r::Resource::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1424272e0, "r_1424272e0", fn_1424272e0 as *const ()), // sub_1424272e0
    Replacement::draft_throws(0x1424273b0, "r_1424273b0", fn_1424273b0 as *const (), Throws::Caught), // sub_1424273b0
    Replacement::draft(0x1424274c0, "r_1424274c0", fn_1424274c0 as *const ()), // sub_1424274c0
    Replacement::draft(0x142427570, "r_142427570", fn_142427570 as *const ()), // sub_142427570
    Replacement::draft(0x142427590, "r_142427590", fn_142427590 as *const ()), // sub_142427590
    Replacement::draft(0x1424275d0, "r_1424275d0", fn_1424275d0 as *const ()), // sub_1424275d0
    Replacement::draft(0x142427800, "r_142427800", tpl_28467::<0x30, 0x8, 0x10> as *const ()), // sub_142427800
    Replacement::draft_throws(0x142427820, "r_142427820", fn_142427820 as *const (), Throws::Discard), // sub_142427820
    Replacement::draft(0x142427b50, "r_142427b50", fn_142427b50 as *const ()), // sub_142427b50
    Replacement::draft(0x1424280a0, "r_1424280a0", fn_1424280a0 as *const ()), // sub_1424280a0
    Replacement::draft_throws(0x142428420, "r_142428420", fn_142428420 as *const (), Throws::Caught), // sub_142428420
    Replacement::draft_throws(0x142428500, "r_142428500", fn_142428500 as *const (), Throws::Caught), // sub_142428500
    Replacement::draft(0x1424286f0, "r_1424286f0", tpl_50142::<0x143a08320, 0x120> as *const ()), // sub_1424286f0
    Replacement::draft_throws(0x142428710, "r_142428710", fn_142428710 as *const (), Throws::Discard), // sub_142428710
    Replacement::draft(0x1424287f0, "r_1424287f0", fn_1424287f0 as *const ()), // r::Resource::r::Resource::vfunc_13
    Replacement::draft(0x1424288e0, "r_1424288e0", fn_1424288e0 as *const ()), // r::Resource::r::Resource::vfunc_12
    Replacement::draft_throws(0x142428910, "r_142428910", fn_142428910 as *const (), Throws::Caught), // sub_142428910
    Replacement::draft_throws(0x142428a00, "r_142428a00", fn_142428a00 as *const (), Throws::Caught), // sub_142428a00
    Replacement::draft(0x142428b00, "r_142428b00", tpl_43249::<0x80> as *const ()), // sub_142428b00
    Replacement::draft(0x142428b20, "r_142428b20", tpl_28467::<0x30, 0x8, 0x10> as *const ()), // r::Resource::r::Resource::vfunc_9
    Replacement::draft_throws(0x142428b40, "r_142428b40", fn_142428b40 as *const (), Throws::Caught), // r::Resource::r::Resource::vfunc_7
    Replacement::draft(0x142428c40, "r_142428c40", fn_142428c40 as *const ()), // r::Resource::r::Resource::vfunc_5
    Replacement::draft(0x142428c60, "r_142428c60", fn_142428c60 as *const ()), // r::Resource::r::Resource::vfunc_4
    Replacement::draft(0x142428ca0, "r_142428ca0", tpl_48427::<0x80> as *const ()), // r::Resource::r::Resource::vfunc_3
    Replacement::draft(0x142428cc0, "r_142428cc0", fn_142428cc0 as *const ()), // r::Resource::r::Resource::vfunc_1
    Replacement::draft(0x142428ce0, "r_142428ce0", fn_142428ce0 as *const ()), // sub_142428ce0
    Replacement::draft(0x142428d70, "r_142428d70", fn_142428d70 as *const ()), // sub_142428d70
    Replacement::draft(0x142428e20, "r_142428e20", fn_142428e20 as *const ()), // sub_142428e20
    Replacement::draft(0x142428e40, "r_142428e40", fn_142428e40 as *const ()), // sub_142428e40
    Replacement::draft(0x142428e70, "r_142428e70", fn_142428e70 as *const ()), // sub_142428e70
    Replacement::draft(0x142428ec0, "r_142428ec0", fn_142428ec0 as *const ()), // sub_142428ec0
    Replacement::draft(0x142428f00, "r_142428f00", fn_142428f00 as *const ()), // sub_142428f00
    Replacement::draft(0x142428f50, "r_142428f50", fn_142428f50 as *const ()), // sub_142428f50
    Replacement::draft(0x142428fa0, "r_142428fa0", fn_142428fa0 as *const ()), // sub_142428fa0
    Replacement::draft(0x142429060, "r_142429060", fn_142429060 as *const ()), // sub_142429060
    Replacement::draft_throws(0x1424290b0, "r_1424290b0", fn_1424290b0 as *const (), Throws::Caught), // sub_1424290b0
    Replacement::draft(0x142429300, "r_142429300", fn_142429300 as *const ()), // sub_142429300
    Replacement::draft_throws(0x1424293e0, "r_1424293e0", fn_1424293e0 as *const (), Throws::Caught), // sub_1424293e0
    Replacement::draft_throws(0x1424294b0, "r_1424294b0", fn_1424294b0 as *const (), Throws::Caught), // sub_1424294b0
    Replacement::draft(0x142429650, "r_142429650", fn_142429650 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_28
    Replacement::draft(0x142429690, "r_142429690", tpl_50167::<0x38, 0xd8> as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_27
    Replacement::draft_throws(0x1424296c0, "r_1424296c0", fn_1424296c0 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_26
    Replacement::draft(0x1424297b0, "r_1424297b0", fn_1424297b0 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_24
    Replacement::draft(0x142429880, "r_142429880", fn_142429880 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_23
    Replacement::draft(0x142429950, "r_142429950", fn_142429950 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_21
    Replacement::draft(0x142429a20, "r_142429a20", tpl_50172::<0x1426456f8, 0x142406920, 0x38, 0xa0, 0x142429300> as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_20
    Replacement::draft(0x142429af0, "r_142429af0", tpl_50172::<0x1426456f8, 0x142406920, 0x38, 0x98, 0x142429300> as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_19
    Replacement::draft(0x142429bc0, "r_142429bc0", fn_142429bc0 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_18
    Replacement::draft(0x142429c90, "r_142429c90", fn_142429c90 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_16
    Replacement::draft(0x142429d60, "r_142429d60", fn_142429d60 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_15
    Replacement::draft_throws(0x142429e30, "r_142429e30", fn_142429e30 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_14
    Replacement::draft_throws(0x142429f20, "r_142429f20", fn_142429f20 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_13
    Replacement::draft_throws(0x14242a020, "r_14242a020", fn_14242a020 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_12
    Replacement::draft_throws(0x14242a100, "r_14242a100", fn_14242a100 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_11
    Replacement::draft_throws(0x14242a1e0, "r_14242a1e0", fn_14242a1e0 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_10
    Replacement::draft_throws(0x14242a2c0, "r_14242a2c0", fn_14242a2c0 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_9
    Replacement::draft_throws(0x14242a3b0, "r_14242a3b0", fn_14242a3b0 as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_8
    Replacement::draft_throws(0x14242a490, "r_14242a490", tpl_50183::<0x1426456f8, 0x142406920, 0x38, 0x1424293e0> as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_7
    Replacement::draft_throws(0x14242a570, "r_14242a570", tpl_50183::<0x1426456f8, 0x142406920, 0x30, 0x1424293e0> as *const (), Throws::Discard), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_6
    Replacement::draft(0x14242a650, "r_14242a650", fn_14242a650 as *const ()), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_5
    Replacement::draft_throws(0x14242a790, "r_14242a790", fn_14242a790 as *const (), Throws::Caught), // r::ProfilingFileSystem::r::ProfilingFileSystem::vfunc_4
    Replacement::draft(0x14242a8d0, "r_14242a8d0", fn_14242a8d0 as *const ()), // sub_14242a8d0
    Replacement::draft(0x14242a950, "r_14242a950", fn_14242a950 as *const ()), // sub_14242a950
    Replacement::draft(0x14242aa10, "r_14242aa10", fn_14242aa10 as *const ()), // sub_14242aa10
    Replacement::draft(0x14242aaa0, "r_14242aaa0", fn_14242aaa0 as *const ()), // sub_14242aaa0
    Replacement::draft(0x14242aad0, "r_14242aad0", fn_14242aad0 as *const ()), // sub_14242aad0
    Replacement::draft(0x14242ab00, "r_14242ab00", fn_14242ab00 as *const ()), // sub_14242ab00
    Replacement::draft(0x14242ab40, "r_14242ab40", fn_14242ab40 as *const ()), // sub_14242ab40
    Replacement::draft(0x14242abd0, "r_14242abd0", fn_14242abd0 as *const ()), // sub_14242abd0
    Replacement::draft(0x14242ac10, "r_14242ac10", fn_14242ac10 as *const ()), // sub_14242ac10
    Replacement::draft(0x14242ace0, "r_14242ace0", fn_14242ace0 as *const ()), // sub_14242ace0
    Replacement::draft(0x14242adf0, "r_14242adf0", fn_14242adf0 as *const ()), // sub_14242adf0
    Replacement::draft(0x14242b190, "r_14242b190", fn_14242b190 as *const ()), // sub_14242b190
    Replacement::draft(0x14242b1c0, "r_14242b1c0", fn_14242b1c0 as *const ()), // sub_14242b1c0
    Replacement::draft(0x14242b240, "r_14242b240", fn_14242b240 as *const ()), // sub_14242b240
    Replacement::draft_throws(0x14242b280, "r_14242b280", fn_14242b280 as *const (), Throws::Discard), // r::ProcessMonitorThread::r::ProcessMonitorThread::vfunc_1
    Replacement::draft(0x14242b3b0, "r_14242b3b0", fn_14242b3b0 as *const ()), // sub_14242b3b0
    Replacement::draft_throws(0x14242b4b0, "r_14242b4b0", fn_14242b4b0 as *const (), Throws::Discard), // sub_14242b4b0
    Replacement::draft(0x14242b580, "r_14242b580", fn_14242b580 as *const ()), // sub_14242b580
    Replacement::draft(0x14242b600, "r_14242b600", fn_14242b600 as *const ()), // sub_14242b600
    Replacement::draft_throws(0x14242b6f0, "r_14242b6f0", fn_14242b6f0 as *const (), Throws::Caught), // sub_14242b6f0
    Replacement::draft(0x14242bbf0, "r_14242bbf0", fn_14242bbf0 as *const ()), // sub_14242bbf0
    Replacement::draft(0x14242bd80, "r_14242bd80", fn_14242bd80 as *const ()), // sub_14242bd80
    Replacement::draft(0x14242bde0, "r_14242bde0", fn_14242bde0 as *const ()), // sub_14242bde0
    Replacement::draft_throws(0x14242bfc0, "r_14242bfc0", fn_14242bfc0 as *const (), Throws::Caught), // sub_14242bfc0
    Replacement::draft(0x14242c560, "r_14242c560", fn_14242c560 as *const ()), // sub_14242c560
    Replacement::draft(0x14242cc00, "r_14242cc00", fn_14242cc00 as *const ()), // sub_14242cc00
    Replacement::draft(0x14242cc50, "r_14242cc50", tpl_44366::<0x142cb01d0> as *const ()), // sub_14242cc50
    Replacement::draft(0x14242cc60, "r_14242cc60", fn_14242cc60 as *const ()), // sub_14242cc60
    Replacement::draft(0x14242cd80, "r_14242cd80", fn_14242cd80 as *const ()), // sub_14242cd80
    Replacement::draft(0x14242ce80, "r_14242ce80", fn_14242ce80 as *const ()), // sub_14242ce80
    Replacement::draft(0x14242cf70, "r_14242cf70", fn_14242cf70 as *const ()), // sub_14242cf70
    Replacement::draft(0x14242d140, "r_14242d140", fn_14242d140 as *const ()), // sub_14242d140
    Replacement::draft(0x14242d4f0, "r_14242d4f0", fn_14242d4f0 as *const ()), // sub_14242d4f0
    Replacement::draft(0x14242d530, "r_14242d530", fn_14242d530 as *const ()), // sub_14242d530
    Replacement::draft(0x14242d990, "r_14242d990", fn_14242d990 as *const ()), // sub_14242d990
    Replacement::draft(0x14242d9d0, "r_14242d9d0", tpl_50216::<0x143426780, 0x14183ad8e, 0x14183adc4> as *const ()), // sub_14242d9d0
    Replacement::draft(0x14242da10, "r_14242da10", tpl_50217::<0x2d0, 0x14242cf70> as *const ()), // sub_14242da10
    Replacement::draft(0x14242da30, "r_14242da30", tpl_50217::<0x2e0, 0x14242cf70> as *const ()), // sub_14242da30
    Replacement::draft(0x14242da50, "r_14242da50", fn_14242da50 as *const ()), // sub_14242da50
    Replacement::draft(0x14242dad0, "r_14242dad0", tpl_586::<0x2ec> as *const ()), // sub_14242dad0
    Replacement::draft(0x14242dae0, "r_14242dae0", tpl_586::<0x2e0> as *const ()), // sub_14242dae0
    Replacement::draft(0x14242daf0, "r_14242daf0", tpl_50217::<0x2ec, 0x14242cf70> as *const ()), // sub_14242daf0
    Replacement::draft(0x14242db10, "r_14242db10", tpl_50217::<0x2e8, 0x14242cf70> as *const ()), // sub_14242db10
    Replacement::draft(0x14242db30, "r_14242db30", fn_14242db30 as *const ()), // sub_14242db30
    Replacement::draft(0x14242dbc0, "r_14242dbc0", tpl_586::<0x2e8> as *const ()), // sub_14242dbc0
    Replacement::draft(0x14242dbd0, "r_14242dbd0", tpl_50216::<0x143426780, 0x14183ad8e, 0x14183adc4> as *const ()), // sub_14242dbd0
    Replacement::draft(0x14242dc10, "r_14242dc10", fn_14242dc10 as *const ()), // sub_14242dc10
    Replacement::draft(0x14242dc70, "r_14242dc70", fn_14242dc70 as *const ()), // sub_14242dc70
    Replacement::draft(0x14242dcb0, "r_14242dcb0", tpl_586::<0x2e4> as *const ()), // sub_14242dcb0
    Replacement::draft(0x14242de10, "r_14242de10", fn_14242de10 as *const ()), // sub_14242de10
    Replacement::draft(0x14242de80, "r_14242de80", fn_14242de80 as *const ()), // sub_14242de80
    Replacement::draft(0x14242deb0, "r_14242deb0", fn_14242deb0 as *const ()), // sub_14242deb0
    Replacement::draft(0x14242e200, "r_14242e200", fn_14242e200 as *const ()), // sub_14242e200
    Replacement::draft(0x14242e2c0, "r_14242e2c0", fn_14242e2c0 as *const ()), // sub_14242e2c0
    Replacement::draft(0x14242e2d0, "r_14242e2d0", fn_14242e2d0 as *const ()), // sub_14242e2d0
    Replacement::draft(0x14242e330, "r_14242e330", fn_14242e330 as *const ()), // sub_14242e330
    Replacement::draft(0x14242e3b0, "r_14242e3b0", fn_14242e3b0 as *const ()), // sub_14242e3b0
    Replacement::draft(0x14242e430, "r_14242e430", fn_14242e430 as *const ()), // sub_14242e430
    Replacement::draft(0x14242e570, "r_14242e570", fn_14242e570 as *const ()), // sub_14242e570
    Replacement::draft(0x14242e660, "r_14242e660", fn_14242e660 as *const ()), // sub_14242e660
    Replacement::draft(0x14242e740, "r_14242e740", fn_14242e740 as *const ()), // sub_14242e740
    Replacement::draft(0x14242e950, "r_14242e950", tpl_50235::<0x14242e660, 0x30, 0x60> as *const ()), // sub_14242e950
    Replacement::draft(0x14242e9c0, "r_14242e9c0", tpl_50235::<0x14242e740, 0x30, 0x60> as *const ()), // sub_14242e9c0
    Replacement::draft_throws(0x14242ea50, "r_14242ea50", fn_14242ea50 as *const (), Throws::Caught), // sub_14242ea50
    Replacement::draft_throws(0x14242efe0, "r_14242efe0", fn_14242efe0 as *const (), Throws::Discard), // sub_14242efe0
    Replacement::draft_throws(0x14242f260, "r_14242f260", fn_14242f260 as *const (), Throws::Caught), // sub_14242f260
    Replacement::draft_throws(0x14242f990, "r_14242f990", fn_14242f990 as *const (), Throws::Caught), // sub_14242f990
    Replacement::draft_throws(0x14242feb0, "r_14242feb0", crate::shards::subsystems::fn_14242feb0 as *const (), Throws::Caught), // sub_14242feb0
    Replacement::draft_throws(0x14242fef0, "r_14242fef0", fn_14242fef0 as *const (), Throws::Caught), // sub_14242fef0
    Replacement::draft_throws(0x14242ffc0, "r_14242ffc0", fn_14242ffc0 as *const (), Throws::Caught), // sub_14242ffc0
    Replacement::draft_throws(0x142430090, "r_142430090", fn_142430090 as *const (), Throws::Caught), // sub_142430090
    Replacement::draft_throws(0x142430290, "r_142430290", fn_142430290 as *const (), Throws::Caught), // sub_142430290
    Replacement::draft_throws(0x142430540, "r_142430540", fn_142430540 as *const (), Throws::Caught), // sub_142430540
    Replacement::draft_throws(0x142430920, "r_142430920", crate::shards::subsystems::fn_142430920 as *const (), Throws::Caught), // sub_142430920
    Replacement::draft_throws(0x142430960, "r_142430960", fn_142430960 as *const (), Throws::Caught), // sub_142430960
    Replacement::draft(0x142430d50, "r_142430d50", fn_142430d50 as *const ()), // sub_142430d50
    Replacement::draft_throws(0x142430e50, "r_142430e50", fn_142430e50 as *const (), Throws::Discard), // sub_142430e50
    Replacement::draft(0x142430f50, "r_142430f50", tpl_50249::<0x18, 0x1413a4500, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationData<anonymous_namespace::OpenTOCHelper::load::<lambda_3>,void>::r::internal::ContinuationData<
    Replacement::draft(0x142431050, "r_142431050", fn_142431050 as *const ()), // sub_142431050
    Replacement::draft(0x1424310d0, "r_1424310d0", tpl_50251::<0x1401f3d00, 0x30, 0x1401de830, 0x20, 0x1402c3180, 0x10, 0x1401fb9f0, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationData<anonymous_namespace::OpenTOCHelper::openBlobs::<lambda_2>,void>::r::internal::Continuation
    Replacement::draft(0x142431140, "r_142431140", tpl_50252 as *const ()), // sub_142431140
    Replacement::draft(0x142431160, "r_142431160", tpl_50253::<0x18, 0x1413a4500, 0x8, 0x142432510, 0x143409af0, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::pack2::TOC::open::<lambda_1>,std::unique_ptr<const r::pack2::TOC,r::detail::Destroyer<c
    Replacement::draft(0x1424311c0, "r_1424311c0", tpl_34372::<0x143409ad8, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_1424311c0
    Replacement::draft(0x142431200, "r_142431200", tpl_34372::<0x143409b20, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_142431200
    Replacement::draft(0x142431240, "r_142431240", tpl_34372::<0x143409b50, 0x8, 0x140209650, 0x18, 0x20> as *const ()), // sub_142431240
    Replacement::draft(0x142431280, "r_142431280", tpl_34372::<0x143409b80, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_142431280
    Replacement::draft(0x1424312c0, "r_1424312c0", tpl_34372::<0x143409bb0, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_1424312c0
    Replacement::draft(0x142431300, "r_142431300", fn_142431300 as *const ()), // sub_142431300
    Replacement::draft(0x1424313b0, "r_1424313b0", fn_1424313b0 as *const ()), // sub_1424313b0
    Replacement::draft(0x142431560, "r_142431560", tpl_50256::<0x18, 0x1413a4500, 0x143391070> as *const ()), // sub_142431560
    Replacement::draft(0x1424315a0, "r_1424315a0", tpl_50256::<0x18, 0x1413a4500, 0x143391070> as *const ()), // sub_1424315a0
    Replacement::draft(0x1424315e0, "r_1424315e0", tpl_50249::<0x18, 0x1413a4500, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<anonymous_namespace::OpenTOCHelper::load::<lambda_3>,void>::r::internal::ContinuationImpl<
    Replacement::draft(0x1424317a0, "r_1424317a0", tpl_50257::<0x1401f3d00, 0x30, 0x1401de830, 0x20, 0x1402c3180, 0x10, 0x1401fb9f0, 0x143391070> as *const ()), // sub_1424317a0
    Replacement::draft(0x1424317f0, "r_1424317f0", tpl_50257::<0x1401f3d00, 0x30, 0x1401de830, 0x20, 0x1402c3180, 0x10, 0x1401fb9f0, 0x143391070> as *const ()), // sub_1424317f0
    Replacement::draft(0x142431840, "r_142431840", tpl_50251::<0x1401f3d00, 0x30, 0x1401de830, 0x20, 0x1402c3180, 0x10, 0x1401fb9f0, 0x143391070, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<anonymous_namespace::OpenTOCHelper::openBlobs::<lambda_2>,void>::r::internal::VoidCont
    Replacement::draft(0x1424318b0, "r_1424318b0", tpl_50258::<0x18, 0x1413a4500, 0x8, 0x142432510, 0x143409af0> as *const ()), // sub_1424318b0
    Replacement::draft(0x1424318f0, "r_1424318f0", tpl_50258::<0x18, 0x1413a4500, 0x8, 0x142432510, 0x143409af0> as *const ()), // sub_1424318f0
    Replacement::draft(0x142431930, "r_142431930", tpl_50253::<0x18, 0x1413a4500, 0x8, 0x142432510, 0x143409af0, 0x141834d30> as *const ()), // r::internal::ContinuationImpl<r::pack2::TOC::open::<lambda_1>,std::unique_ptr<const r::pack2::TOC,r::detail::Destroyer<c
    Replacement::draft(0x142431990, "r_142431990", tpl_34373::<0x143409ad8, 0x8, 0x1401f3d90, 0x18, 0x20, 0x143409b08> as *const ()), // sub_142431990
    Replacement::draft(0x1424319e0, "r_1424319e0", tpl_34373::<0x143409b20, 0x8, 0x1401f3d90, 0x18, 0x20, 0x143409b38> as *const ()), // sub_1424319e0
    Replacement::draft(0x142431a30, "r_142431a30", tpl_34373::<0x143409b50, 0x8, 0x140209650, 0x18, 0x20, 0x143409b68> as *const ()), // sub_142431a30
    Replacement::draft(0x142431a80, "r_142431a80", tpl_34373::<0x143409b80, 0x8, 0x1401f3d90, 0x18, 0x20, 0x143409b98> as *const ()), // sub_142431a80
    Replacement::draft(0x142431ad0, "r_142431ad0", tpl_34373::<0x143409bb0, 0x8, 0x1401f3d90, 0x18, 0x20, 0x143409bc8> as *const ()), // sub_142431ad0
    Replacement::draft(0x142431b20, "r_142431b20", fn_142431b20 as *const ()), // sub_142431b20
    Replacement::draft(0x142431be0, "r_142431be0", fn_142431be0 as *const ()), // sub_142431be0
    Replacement::draft(0x142431d50, "r_142431d50", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_142431d50
    Replacement::draft(0x142431d70, "r_142431d70", fn_142431d70 as *const ()), // r::internal::ContinuationImpl<anonymous_namespace::OpenTOCHelper::load::<lambda_1>,void>::r::internal::ContinuationImpl<
    Replacement::draft(0x142431e70, "r_142431e70", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_142431e70
    Replacement::draft_throws(0x142431e90, "r_142431e90", fn_142431e90 as *const (), Throws::Caught), // r::internal::ContinuationImpl<anonymous_namespace::OpenTOCHelper::load::<lambda_2>,void>::r::internal::ContinuationImpl<
    Replacement::draft(0x142431f90, "r_142431f90", tpl_35117::<0x18, 0x20, 0x8, 0x10> as *const ()), // sub_142431f90
    Replacement::draft_throws(0x142431fb0, "r_142431fb0", fn_142431fb0 as *const (), Throws::Discard), // r::internal::ContinuationImpl<anonymous_namespace::OpenTOCHelper::load::<lambda_3>,void>::r::internal::ContinuationImpl<
    Replacement::draft(0x142432120, "r_142432120", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_142432120
    Replacement::draft(0x142432140, "r_142432140", fn_142432140 as *const ()), // r::internal::VoidContinuationImpl<anonymous_namespace::OpenTOCHelper::loadHeader::<lambda_1>,void>::r::internal::VoidCon
    Replacement::draft(0x1424321f0, "r_1424321f0", tpl_34374::<0x18, 0x20, 0x8> as *const ()), // sub_1424321f0
    Replacement::draft(0x142432210, "r_142432210", fn_142432210 as *const ()), // r::internal::VoidContinuationImpl<anonymous_namespace::OpenTOCHelper::loadChunks::<lambda_1>,void>::r::internal::VoidCon
    Replacement::draft(0x1424322e0, "r_1424322e0", tpl_34374::<0x48, 0x50, 0x8> as *const ()), // sub_1424322e0
    Replacement::draft(0x1424323b0, "r_1424323b0", tpl_35117::<0x18, 0x20, 0x8, 0x10> as *const ()), // sub_1424323b0
    Replacement::draft(0x1424323d0, "r_1424323d0", fn_1424323d0 as *const ()), // r::internal::ContinuationImpl<r::pack2::TOC::open::<lambda_1>,std::unique_ptr<const r::pack2::TOC,r::detail::Destroyer<c
    Replacement::draft(0x142432460, "r_142432460", fn_142432460 as *const ()), // sub_142432460
    Replacement::draft(0x142432510, "r_142432510", fn_142432510 as *const ()), // sub_142432510
    Replacement::draft(0x1424325d0, "r_1424325d0", tpl_50269::<0xc, 0x48, 0x10, 0x8> as *const ()), // sub_1424325d0
    Replacement::draft(0x1424325f0, "r_1424325f0", tpl_50269::<0x14, 0x48, 0x18, 0x8> as *const ()), // sub_1424325f0
    Replacement::draft(0x142432610, "r_142432610", fn_142432610 as *const ()), // sub_142432610
    Replacement::draft(0x142432630, "r_142432630", fn_142432630 as *const ()), // sub_142432630
    Replacement::draft(0x142432650, "r_142432650", fn_142432650 as *const ()), // sub_142432650
    Replacement::draft(0x142432670, "r_142432670", fn_142432670 as *const ()), // sub_142432670
    Replacement::draft(0x1424326a0, "r_1424326a0", fn_1424326a0 as *const ()), // sub_1424326a0
    Replacement::draft(0x1424326e0, "r_1424326e0", tpl_50275::<0xc, 0x10, 0x8, 0x48, 0x1426d01e0> as *const ()), // sub_1424326e0
    Replacement::draft_throws(0x142432930, "r_142432930", fn_142432930 as *const (), Throws::Discard), // sub_142432930
    Replacement::draft(0x1424329d0, "r_1424329d0", tpl_50275::<0x14, 0x18, 0x8, 0x48, 0x1426d01e0> as *const ()), // sub_1424329d0
    Replacement::draft(0x1424331c0, "r_1424331c0", tpl_50252 as *const ()), // sub_1424331c0
    Replacement::draft(0x1424331e0, "r_1424331e0", fn_1424331e0 as *const ()), // sub_1424331e0
    Replacement::draft(0x1424332c0, "r_1424332c0", fn_1424332c0 as *const ()), // sub_1424332c0
    Replacement::draft_throws(0x142433300, "r_142433300", fn_142433300 as *const (), Throws::Discard), // sub_142433300
    Replacement::draft(0x142433390, "r_142433390", fn_142433390 as *const ()), // sub_142433390
    Replacement::draft_throws(0x142433420, "r_142433420", fn_142433420 as *const (), Throws::Caught), // sub_142433420
    Replacement::draft_throws(0x142433540, "r_142433540", fn_142433540 as *const (), Throws::Discard), // sub_142433540
    Replacement::draft_throws(0x1424335e0, "r_1424335e0", fn_1424335e0 as *const (), Throws::Caught), // sub_1424335e0
    Replacement::draft(0x142433dd0, "r_142433dd0", fn_142433dd0 as *const ()), // sub_142433dd0
    Replacement::draft(0x142433e20, "r_142433e20", fn_142433e20 as *const ()), // sub_142433e20
    Replacement::draft(0x1424341b0, "r_1424341b0", fn_1424341b0 as *const ()), // sub_1424341b0
    Replacement::draft(0x1424341d0, "r_1424341d0", fn_1424341d0 as *const ()), // sub_1424341d0
    Replacement::draft(0x1424343f0, "r_1424343f0", fn_1424343f0 as *const ()), // sub_1424343f0
    Replacement::draft_throws(0x1424344a0, "r_1424344a0", fn_1424344a0 as *const (), Throws::Discard), // sub_1424344a0
    Replacement::draft_throws(0x1424345a0, "r_1424345a0", fn_1424345a0 as *const (), Throws::Caught), // sub_1424345a0
    Replacement::draft(0x1424347c0, "r_1424347c0", fn_1424347c0 as *const ()), // sub_1424347c0
    Replacement::draft_throws(0x142434860, "r_142434860", fn_142434860 as *const (), Throws::Caught), // sub_142434860
    Replacement::draft(0x142434a20, "r_142434a20", fn_142434a20 as *const ()), // sub_142434a20
    Replacement::draft(0x142434a60, "r_142434a60", fn_142434a60 as *const ()), // sub_142434a60
    Replacement::draft(0x142434b10, "r_142434b10", fn_142434b10 as *const ()), // sub_142434b10
    Replacement::draft(0x142434b70, "r_142434b70", tpl_50297::<0x10, 0x28, 0x20, 0x18> as *const ()), // sub_142434b70
    Replacement::draft(0x142434bc0, "r_142434bc0", fn_142434bc0 as *const ()), // sub_142434bc0
    Replacement::draft(0x142434c30, "r_142434c30", tpl_50299::<0x3, 0x4, 0x8, 0x14240aa00, 0x108, 0x10c> as *const ()), // sub_142434c30
    Replacement::draft(0x142434d90, "r_142434d90", fn_142434d90 as *const ()), // sub_142434d90
    Replacement::draft(0x142434df0, "r_142434df0", tpl_40496 as *const ()), // sub_142434df0
    Replacement::draft(0x142434e10, "r_142434e10", tpl_50301::<0xc> as *const ()), // sub_142434e10
    Replacement::draft(0x142434f20, "r_142434f20", tpl_40497::<0xc> as *const ()), // sub_142434f20
    Replacement::draft(0x142434f40, "r_142434f40", fn_142434f40 as *const ()), // sub_142434f40
    Replacement::draft(0x142434f80, "r_142434f80", tpl_40498 as *const ()), // sub_142434f80
    Replacement::draft(0x142434fb0, "r_142434fb0", tpl_18072::<0x8, 0x14240ab10> as *const ()), // sub_142434fb0
    Replacement::draft(0x142435000, "r_142435000", tpl_50299::<0x3, 0x4, 0x8, 0x14240a730, 0x108, 0x10c> as *const ()), // sub_142435000
    Replacement::draft(0x1424350a0, "r_1424350a0", tpl_25480::<0x1401f3d00, 0x8, 0x1413a8610, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationData<r::Pack2FileSystem::submitBatchInternal::<lambda_1>,void>::r::internal::ContinuationData<r
    Replacement::draft(0x142435120, "r_142435120", tpl_49612::<0x143409de0, 0x8, 0x10, 0xc, 0x14, 0x1401f3d90, 0x20, 0x28> as *const ()), // sub_142435120
    Replacement::draft(0x142435190, "r_142435190", tpl_41197::<0x1401f3d00, 0x8, 0x1413a8610, 0x143391070> as *const ()), // sub_142435190
    Replacement::draft(0x1424351d0, "r_1424351d0", tpl_41197::<0x1401f3d00, 0x8, 0x1413a8610, 0x143391070> as *const ()), // sub_1424351d0
    Replacement::draft(0x142435210, "r_142435210", tpl_25480::<0x1401f3d00, 0x8, 0x1413a8610, 0x143391070, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<r::Pack2FileSystem::submitBatchInternal::<lambda_1>,void>::r::internal::VoidContinuati
    Replacement::draft(0x1424353f0, "r_1424353f0", tpl_49613::<0x143409de0, 0x8, 0x10, 0xc, 0x14, 0x1401f3d90, 0x20, 0x28, 0x143409dc8> as *const ()), // sub_1424353f0
    Replacement::draft(0x1424354e0, "r_1424354e0", fn_1424354e0 as *const ()), // sub_1424354e0
    Replacement::draft(0x142435560, "r_142435560", tpl_50301::<0x8> as *const ()), // sub_142435560
    Replacement::draft(0x1424355c0, "r_1424355c0", tpl_40497::<0x8> as *const ()), // sub_1424355c0
    Replacement::draft(0x142435610, "r_142435610", fn_142435610 as *const ()), // sub_142435610
    Replacement::draft(0x142435630, "r_142435630", fn_142435630 as *const ()), // sub_142435630
    Replacement::draft(0x1424356a0, "r_1424356a0", fn_1424356a0 as *const ()), // sub_1424356a0
    Replacement::draft(0x142435730, "r_142435730", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_142435730
    Replacement::draft(0x142435750, "r_142435750", fn_142435750 as *const ()), // r::internal::VoidContinuationImpl<r::Pack2FileSystem::submitBatchInternal::<lambda_1>,void>::r::internal::VoidContinuati
    Replacement::draft(0x1424357c0, "r_1424357c0", fn_1424357c0 as *const ()), // sub_1424357c0
    Replacement::draft(0x142435850, "r_142435850", tpl_40500 as *const ()), // sub_142435850
    Replacement::draft(0x142435900, "r_142435900", fn_142435900 as *const ()), // sub_142435900
    Replacement::draft(0x142435950, "r_142435950", fn_142435950 as *const ()), // sub_142435950
    Replacement::draft(0x142435b10, "r_142435b10", tpl_22188::<0x10, 0x28, 0x20, 0x18> as *const ()), // sub_142435b10
    Replacement::draft(0x142435b40, "r_142435b40", fn_142435b40 as *const ()), // sub_142435b40
    Replacement::draft(0x142435df0, "r_142435df0", fn_142435df0 as *const ()), // sub_142435df0
    Replacement::draft(0x142435ec0, "r_142435ec0", fn_142435ec0 as *const ()), // sub_142435ec0
    Replacement::draft_throws(0x142436140, "r_142436140", fn_142436140 as *const (), Throws::Caught), // sub_142436140
    Replacement::draft_throws(0x1424361e0, "r_1424361e0", fn_1424361e0 as *const (), Throws::Caught), // sub_1424361e0
    Replacement::draft_throws(0x1424362d0, "r_1424362d0", fn_1424362d0 as *const (), Throws::Caught), // sub_1424362d0
    Replacement::draft_throws(0x142436510, "r_142436510", fn_142436510 as *const (), Throws::Discard), // sub_142436510
    Replacement::draft(0x142436620, "r_142436620", tpl_1026::<0x1413a8540, 0x8, 0xc> as *const ()), // sub_142436620
    Replacement::draft(0x1424366c0, "r_1424366c0", fn_1424366c0 as *const ()), // sub_1424366c0
    Replacement::draft(0x1424366f0, "r_1424366f0", fn_1424366f0 as *const ()), // sub_1424366f0
    Replacement::draft(0x142436820, "r_142436820", fn_142436820 as *const ()), // sub_142436820
    Replacement::draft(0x1424368a0, "r_1424368a0", fn_1424368a0 as *const ()), // sub_1424368a0
    Replacement::draft_throws(0x142436930, "r_142436930", fn_142436930 as *const (), Throws::Caught), // sub_142436930
    Replacement::draft(0x142436a50, "r_142436a50", fn_142436a50 as *const ()), // sub_142436a50
    Replacement::draft_throws(0x142436af0, "r_142436af0", fn_142436af0 as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_4
    Replacement::draft(0x142436c10, "r_142436c10", fn_142436c10 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_5
    Replacement::draft_throws(0x142436dc0, "r_142436dc0", fn_142436dc0 as *const (), Throws::Caught), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_7
    Replacement::draft_throws(0x142436eb0, "r_142436eb0", fn_142436eb0 as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_8
    Replacement::draft_throws(0x142436f10, "r_142436f10", tpl_50330::<0x142436510, 0x40, 0x48> as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_9
    Replacement::draft_throws(0x142436f90, "r_142436f90", tpl_50331::<0x142436510, 0x40, 0x50> as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_10
    Replacement::draft_throws(0x142436ff0, "r_142436ff0", tpl_50331::<0x142436510, 0x40, 0x58> as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_11
    Replacement::draft_throws(0x142437050, "r_142437050", tpl_50331::<0x142436510, 0x40, 0x60> as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_12
    Replacement::draft_throws(0x1424370b0, "r_1424370b0", fn_1424370b0 as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_13
    Replacement::draft_throws(0x142437150, "r_142437150", fn_142437150 as *const (), Throws::Caught), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_14
    Replacement::draft(0x142437380, "r_142437380", fn_142437380 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_15
    Replacement::draft(0x1424373a0, "r_1424373a0", fn_1424373a0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_16
    Replacement::draft(0x1424373f0, "r_1424373f0", fn_1424373f0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_17
    Replacement::draft(0x142437440, "r_142437440", fn_142437440 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_18
    Replacement::draft(0x142437470, "r_142437470", fn_142437470 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_19
    Replacement::draft(0x1424378c0, "r_1424378c0", tpl_50339::<0x40, 0xa0> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_20
    Replacement::draft(0x1424378e0, "r_1424378e0", tpl_50340::<0x40, 0xa8> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_21
    Replacement::draft(0x142437900, "r_142437900", fn_142437900 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_22
    Replacement::draft(0x142437960, "r_142437960", fn_142437960 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_23
    Replacement::draft(0x1424379a0, "r_1424379a0", fn_1424379a0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_24
    Replacement::draft_throws(0x1424379c0, "r_1424379c0", tpl_50330::<0x142436510, 0x40, 0xc8> as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_25
    Replacement::draft_throws(0x142437a40, "r_142437a40", fn_142437a40 as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_26
    Replacement::draft_throws(0x142437b00, "r_142437b00", fn_142437b00 as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_27
    Replacement::draft_throws(0x142437be0, "r_142437be0", fn_142437be0 as *const (), Throws::Caught), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_28
    Replacement::draft(0x142438130, "r_142438130", fn_142438130 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_29
    Replacement::draft(0x1424381e0, "r_1424381e0", fn_1424381e0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_30
    Replacement::draft(0x142438240, "r_142438240", fn_142438240 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_31
    Replacement::draft(0x1424382b0, "r_1424382b0", fn_1424382b0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_32
    Replacement::draft(0x1424382e0, "r_1424382e0", tpl_50339::<0x40, 0x108> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_33
    Replacement::draft(0x142438300, "r_142438300", fn_142438300 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_34
    Replacement::draft(0x142438330, "r_142438330", tpl_50340::<0x40, 0x118> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_35
    Replacement::draft_throws(0x142438370, "r_142438370", fn_142438370 as *const (), Throws::Discard), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_37
    Replacement::draft(0x142438450, "r_142438450", fn_142438450 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_38
    Replacement::draft(0x1424384a0, "r_1424384a0", fn_1424384a0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_39
    Replacement::draft(0x142438710, "r_142438710", fn_142438710 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_40
    Replacement::draft(0x142438a20, "r_142438a20", fn_142438a20 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_41
    Replacement::draft(0x142438aa0, "r_142438aa0", fn_142438aa0 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_42
    Replacement::draft(0x142438b40, "r_142438b40", fn_142438b40 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_43
    Replacement::draft(0x142438c00, "r_142438c00", fn_142438c00 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_44
    Replacement::draft(0x142438c50, "r_142438c50", fn_142438c50 as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_45
    Replacement::draft(0x142438cc0, "r_142438cc0", tpl_50167::<0x40, 0x170> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_46
    Replacement::draft(0x142438cf0, "r_142438cf0", tpl_50167::<0x40, 0x178> as *const ()), // r::Pack2FileSystem::r::Pack2FileSystem::vfunc_47
    Replacement::draft(0x142438d20, "r_142438d20", fn_142438d20 as *const ()), // sub_142438d20
    Replacement::draft(0x142438e20, "r_142438e20", fn_142438e20 as *const ()), // sub_142438e20
    Replacement::draft(0x142438fd0, "r_142438fd0", fn_142438fd0 as *const ()), // sub_142438fd0
    Replacement::draft(0x1424390f0, "r_1424390f0", fn_1424390f0 as *const ()), // sub_1424390f0
    Replacement::draft(0x142439140, "r_142439140", fn_142439140 as *const ()), // sub_142439140
    Replacement::draft(0x142439360, "r_142439360", fn_142439360 as *const ()), // sub_142439360
    Replacement::draft(0x1424393f0, "r_1424393f0", fn_1424393f0 as *const ()), // sub_1424393f0
    Replacement::draft(0x142439580, "r_142439580", fn_142439580 as *const ()), // sub_142439580
    Replacement::draft_throws(0x1424395f0, "r_1424395f0", fn_1424395f0 as *const (), Throws::Caught), // sub_1424395f0
    Replacement::draft(0x1424396a0, "r_1424396a0", fn_1424396a0 as *const ()), // sub_1424396a0
    Replacement::draft(0x142439730, "r_142439730", fn_142439730 as *const ()), // sub_142439730
    Replacement::draft(0x1424399f0, "r_1424399f0", tpl_35237::<0x8> as *const ()), // r::OSPHackSIMDTransform::ClassTypeInfoImplementation::r::OSPHackSIMDTransform::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x142439a10, "r_142439a10", tpl_35237::<0x8> as *const ()), // r::OSPHackSIMDTransform::ClassTypeInfoImplementation::r::OSPHackSIMDTransform::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x142439b30, "r_142439b30", tpl_35199::<0x143881f90, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::OSPHackSIMDTransform::ClassTypeInfoImplementation::r::OSPHackSIMDTransform::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x142439b90, "r_142439b90", tpl_35199::<0x143881f90, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::OSPHackSIMDTransform::ClassTypeInfoImplementation::r::OSPHackSIMDTransform::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x142439c20, "r_142439c20", tpl_35239 as *const ()), // r::OSPHackSIMDTransform::ClassTypeInfoImplementation::r::OSPHackSIMDTransform::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x142439c60, "r_142439c60", fn_142439c60 as *const ()), // r::OSPHackSIMDTransform::ClassTypeInfoImplementation::r::OSPHackSIMDTransform::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x142439d80, "r_142439d80", fn_142439d80 as *const ()), // sub_142439d80
    Replacement::draft(0x142439f60, "r_142439f60", fn_142439f60 as *const ()), // sub_142439f60
    Replacement::draft(0x142439fd0, "r_142439fd0", fn_142439fd0 as *const ()), // sub_142439fd0
    Replacement::draft(0x14243a030, "r_14243a030", fn_14243a030 as *const ()), // sub_14243a030
    Replacement::draft(0x14243a080, "r_14243a080", fn_14243a080 as *const ()), // sub_14243a080
    Replacement::draft_throws(0x14243a0c0, "r_14243a0c0", crate::shards::subsystems::fn_14243a0c0 as *const (), Throws::Caught), // sub_14243a0c0
    Replacement::draft_throws(0x14243a160, "r_14243a160", fn_14243a160 as *const (), Throws::Caught), // sub_14243a160
    Replacement::draft(0x14243a260, "r_14243a260", fn_14243a260 as *const ()), // sub_14243a260
    Replacement::draft(0x14243a290, "r_14243a290", fn_14243a290 as *const ()), // sub_14243a290
    Replacement::draft(0x14243a340, "r_14243a340", fn_14243a340 as *const ()), // sub_14243a340
    Replacement::draft(0x14243a3a0, "r_14243a3a0", fn_14243a3a0 as *const ()), // sub_14243a3a0
    Replacement::draft(0x14243a410, "r_14243a410", fn_14243a410 as *const ()), // sub_14243a410
    Replacement::draft(0x14243a440, "r_14243a440", fn_14243a440 as *const ()), // sub_14243a440
    Replacement::draft_throws(0x14243a470, "r_14243a470", crate::shards::subsystems::fn_14243a470 as *const (), Throws::Caught), // sub_14243a470
    Replacement::draft_throws(0x14243a510, "r_14243a510", fn_14243a510 as *const (), Throws::Caught), // sub_14243a510
    Replacement::draft(0x14243d360, "r_14243d360", fn_14243d360 as *const ()), // r::MonotonicMemoryResource::r::MonotonicMemoryResource::vfunc_1
    Replacement::draft(0x14243d450, "r_14243d450", fn_14243d450 as *const ()), // sub_14243d450
    Replacement::draft_throws(0x14243d500, "r_14243d500", fn_14243d500 as *const (), Throws::Discard), // sub_14243d500
    Replacement::draft_throws(0x14243d530, "r_14243d530", fn_14243d530 as *const (), Throws::Discard), // sub_14243d530
    Replacement::draft_throws(0x14243d590, "r_14243d590", fn_14243d590 as *const (), Throws::Discard), // sub_14243d590
    Replacement::draft(0x14243d5e0, "r_14243d5e0", tpl_40083::<0x20> as *const ()), // sub_14243d5e0
    Replacement::draft_throws(0x14243d5f0, "r_14243d5f0", fn_14243d5f0 as *const (), Throws::Discard), // sub_14243d5f0
    Replacement::draft(0x14243d660, "r_14243d660", fn_14243d660 as *const ()), // sub_14243d660
    Replacement::draft(0x14243d690, "r_14243d690", fn_14243d690 as *const ()), // sub_14243d690
    Replacement::draft(0x14243d6c0, "r_14243d6c0", tpl_41056::<0x10> as *const ()), // sub_14243d6c0
    Replacement::draft(0x14243d6e0, "r_14243d6e0", fn_14243d6e0 as *const ()), // sub_14243d6e0
    Replacement::draft(0x14243d730, "r_14243d730", fn_14243d730 as *const ()), // sub_14243d730
    Replacement::draft(0x14243dbb0, "r_14243dbb0", fn_14243dbb0 as *const ()), // sub_14243dbb0
    Replacement::draft(0x14243dbe0, "r_14243dbe0", fn_14243dbe0 as *const ()), // sub_14243dbe0
    Replacement::draft(0x14243dc30, "r_14243dc30", fn_14243dc30 as *const ()), // sub_14243dc30
    Replacement::draft(0x14243dc60, "r_14243dc60", fn_14243dc60 as *const ()), // sub_14243dc60
    Replacement::draft_throws(0x14243dce0, "r_14243dce0", fn_14243dce0 as *const (), Throws::Caught), // sub_14243dce0
    Replacement::draft(0x14243de00, "r_14243de00", fn_14243de00 as *const ()), // sub_14243de00
    Replacement::draft(0x14243e330, "r_14243e330", fn_14243e330 as *const ()), // sub_14243e330
    Replacement::draft(0x14243e360, "r_14243e360", fn_14243e360 as *const ()), // sub_14243e360
    Replacement::draft_throws(0x14243e380, "r_14243e380", fn_14243e380 as *const (), Throws::Discard), // sub_14243e380
    Replacement::draft_throws(0x14243e3d0, "r_14243e3d0", fn_14243e3d0 as *const (), Throws::Caught), // sub_14243e3d0
    Replacement::draft(0x14243ef10, "r_14243ef10", fn_14243ef10 as *const ()), // sub_14243ef10
    Replacement::draft_throws(0x14243f020, "r_14243f020", fn_14243f020 as *const (), Throws::Caught), // sub_14243f020
    Replacement::draft(0x14243f340, "r_14243f340", fn_14243f340 as *const ()), // sub_14243f340
    Replacement::draft(0x14243f3b0, "r_14243f3b0", fn_14243f3b0 as *const ()), // sub_14243f3b0
    Replacement::draft(0x14243f470, "r_14243f470", fn_14243f470 as *const ()), // sub_14243f470
    Replacement::draft(0x14243f500, "r_14243f500", fn_14243f500 as *const ()), // sub_14243f500
    Replacement::draft(0x14243f710, "r_14243f710", fn_14243f710 as *const ()), // sub_14243f710
    Replacement::draft_throws(0x14243f8e0, "r_14243f8e0", fn_14243f8e0 as *const (), Throws::Discard), // sub_14243f8e0
    Replacement::draft(0x14243fc10, "r_14243fc10", fn_14243fc10 as *const ()), // sub_14243fc10
    Replacement::draft_throws(0x14243ffd0, "r_14243ffd0", fn_14243ffd0 as *const (), Throws::Caught), // sub_14243ffd0
    Replacement::draft(0x142440090, "r_142440090", tpl_42155::<0x1401e3e80, 0x68, 0x1413ad4c0, 0x8, 0x140227190> as *const ()), // sub_142440090
    Replacement::draft_throws(0x1424400d0, "r_1424400d0", fn_1424400d0 as *const (), Throws::Caught), // sub_1424400d0
    Replacement::draft(0x142440c80, "r_142440c80", tpl_39586::<0x8, 0x4, 0xc> as *const ()), // r::PackingMetadata::Region::ClassTypeInfoImplementation::r::PackingMetadata::Region::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x142440cd0, "r_142440cd0", tpl_39586::<0x8, 0x4, 0xc> as *const ()), // r::PackingMetadata::Region::ClassTypeInfoImplementation::r::PackingMetadata::Region::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x142440f40, "r_142440f40", fn_142440f40 as *const ()), // r::PackingMetadata::Region::ClassTypeInfoImplementation::r::PackingMetadata::Region::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x142441140, "r_142441140", tpl_47050::<0x143881fc0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::DevelopmentResourceMetadata::r::DevelopmentResourceMetadata::vfunc_17
    Replacement::draft(0x142441570, "r_142441570", tpl_35219::<0x143881fc0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::DevelopmentResourceMetadata::ClassTypeInfoImplementation::r::DevelopmentResourceMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x142441640, "r_142441640", fn_142441640 as *const ()), // r::DevelopmentResourceMetadata::ClassTypeInfoImplementation::r::DevelopmentResourceMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x142441da0, "r_142441da0", fn_142441da0 as *const ()), // r::ResourceMetadata::ClassTypeInfoImplementation::r::ResourceMetadata::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x142441e00, "r_142441e00", fn_142441e00 as *const ()), // r::ResourceMetadata::ClassTypeInfoImplementation::r::ResourceMetadata::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x142442020, "r_142442020", fn_142442020 as *const ()), // r::FileInfoMetadata::r::FileInfoMetadata::vfunc_12
    Replacement::draft(0x142442330, "r_142442330", fn_142442330 as *const ()), // r::FileInfoMetadata::ClassTypeInfoImplementation::r::FileInfoMetadata::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x142442500, "r_142442500", fn_142442500 as *const ()), // r::FileInfoMetadata::ClassTypeInfoImplementation::r::FileInfoMetadata::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x142442550, "r_142442550", fn_142442550 as *const ()), // r::FileInfoMetadata::ClassTypeInfoImplementation::r::FileInfoMetadata::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x142443330, "r_142443330", fn_142443330 as *const (), Throws::Caught), // r::MemoryFile::r::MemoryFile::vfunc_6
    Replacement::draft(0x142475cc0, "r_142475cc0", fn_142475cc0 as *const ()), // r::Loader_CallbackQueue::r::Loader_CallbackQueue::vfunc_1
    Replacement::draft(0x142475da0, "r_142475da0", tpl_1691::<0x78> as *const ()), // sub_142475da0
    Replacement::draft(0x142476070, "r_142476070", fn_142476070 as *const ()), // sub_142476070
    Replacement::draft(0x1424760e0, "r_1424760e0", fn_1424760e0 as *const ()), // sub_1424760e0
    Replacement::draft(0x142476190, "r_142476190", fn_142476190 as *const ()), // sub_142476190
    Replacement::draft(0x142476210, "r_142476210", fn_142476210 as *const ()), // sub_142476210
    Replacement::draft(0x142476290, "r_142476290", fn_142476290 as *const ()), // sub_142476290
    Replacement::draft(0x1424762c0, "r_1424762c0", fn_1424762c0 as *const ()), // sub_1424762c0
    Replacement::draft(0x142476330, "r_142476330", fn_142476330 as *const ()), // sub_142476330
    Replacement::draft(0x142476380, "r_142476380", tpl_50297::<0x10, 0x28, 0x20, 0x18> as *const ()), // sub_142476380
    Replacement::draft(0x1424765c0, "r_1424765c0", fn_1424765c0 as *const ()), // sub_1424765c0
    Replacement::draft(0x1424765f0, "r_1424765f0", fn_1424765f0 as *const ()), // sub_1424765f0
    Replacement::draft(0x142476610, "r_142476610", fn_142476610 as *const ()), // sub_142476610
    Replacement::draft(0x142476650, "r_142476650", tpl_46418 as *const ()), // sub_142476650
    Replacement::draft(0x1424766a0, "r_1424766a0", tpl_46419::<0xc> as *const ()), // sub_1424766a0
    Replacement::draft(0x1424766f0, "r_1424766f0", tpl_46420 as *const ()), // sub_1424766f0
    Replacement::draft(0x142476710, "r_142476710", tpl_43260::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_142476710
    Replacement::draft(0x142476770, "r_142476770", tpl_50685::<0x1401f3d00, 0x18, 0x1413bc3c0, 0x8, 0x1413bc2e0, 0x143391070, 0x141834d30> as *const ()), // r::internal::ContinuationData<anonymous_namespace::submitBatch::<lambda_1>,void>::r::internal::ContinuationData<anonymou
    Replacement::draft(0x1424767e0, "r_1424767e0", fn_1424767e0 as *const ()), // sub_1424767e0
    Replacement::draft(0x142476870, "r_142476870", tpl_50687::<0x1401f3d00, 0x18, 0x1413bc3c0, 0x8, 0x1413bc2e0, 0x143391070> as *const ()), // sub_142476870
    Replacement::draft(0x1424768c0, "r_1424768c0", tpl_50687::<0x1401f3d00, 0x18, 0x1413bc3c0, 0x8, 0x1413bc2e0, 0x143391070> as *const ()), // sub_1424768c0
    Replacement::draft(0x142476910, "r_142476910", tpl_50685::<0x1401f3d00, 0x18, 0x1413bc3c0, 0x8, 0x1413bc2e0, 0x143391070, 0x141834d30> as *const ()), // r::internal::VoidContinuationImpl<anonymous_namespace::submitBatch::<lambda_1>,void>::r::internal::VoidContinuationImpl<
    Replacement::draft(0x142476ac0, "r_142476ac0", tpl_27478 as *const ()), // sub_142476ac0
    Replacement::draft(0x142476ad0, "r_142476ad0", fn_142476ad0 as *const ()), // sub_142476ad0
    Replacement::draft(0x142476c60, "r_142476c60", tpl_46419::<0x8> as *const ()), // sub_142476c60
    Replacement::draft(0x142476d90, "r_142476d90", tpl_34374::<0x30, 0x38, 0x8> as *const ()), // sub_142476d90
    Replacement::draft(0x142476ee0, "r_142476ee0", fn_142476ee0 as *const ()), // r::Loader_Adaptor_Pack2::r::Loader_Adaptor_Pack2::vfunc_4
    Replacement::draft(0x142476f20, "r_142476f20", fn_142476f20 as *const ()), // r::Loader_Adaptor_Pack2::r::Loader_Adaptor_Pack2::vfunc_3
    Replacement::draft(0x142477960, "r_142477960", fn_142477960 as *const ()), // sub_142477960
    Replacement::draft(0x1424779a0, "r_1424779a0", tpl_22188::<0x10, 0x28, 0x20, 0x18> as *const ()), // sub_1424779a0
    Replacement::draft_throws(0x1424779d0, "r_1424779d0", fn_1424779d0 as *const (), Throws::Caught), // sub_1424779d0
    Replacement::draft_throws(0x1424781f0, "r_1424781f0", fn_1424781f0 as *const (), Throws::Caught), // sub_1424781f0
    Replacement::draft(0x1424782f0, "r_1424782f0", fn_1424782f0 as *const ()), // sub_1424782f0
    Replacement::draft(0x142478870, "r_142478870", fn_142478870 as *const ()), // sub_142478870
    Replacement::draft(0x142478d20, "r_142478d20", fn_142478d20 as *const ()), // sub_142478d20
    Replacement::draft(0x142478da0, "r_142478da0", fn_142478da0 as *const ()), // sub_142478da0
    Replacement::draft(0x142478f30, "r_142478f30", fn_142478f30 as *const ()), // sub_142478f30
    Replacement::draft(0x142479290, "r_142479290", fn_142479290 as *const ()), // sub_142479290
    Replacement::draft(0x142479330, "r_142479330", fn_142479330 as *const ()), // sub_142479330
    Replacement::draft_throws(0x142479410, "r_142479410", fn_142479410 as *const (), Throws::Discard), // sub_142479410
    Replacement::draft(0x142479440, "r_142479440", fn_142479440 as *const ()), // sub_142479440
    Replacement::draft_throws(0x142479480, "r_142479480", fn_142479480 as *const (), Throws::Caught), // sub_142479480
    Replacement::draft(0x1424795a0, "r_1424795a0", tpl_50711::<0x14340cb88, 0x1413bbe20> as *const ()), // sub_1424795a0
    Replacement::draft(0x1424795c0, "r_1424795c0", fn_1424795c0 as *const ()), // sub_1424795c0
    Replacement::draft_throws(0x142479740, "r_142479740", fn_142479740 as *const (), Throws::Caught), // r::Loader_Adaptor_HotloadDataCache::r::Loader_Adaptor_HotloadDataCache::vfunc_2
    Replacement::draft_throws(0x142479a60, "r_142479a60", fn_142479a60 as *const (), Throws::Discard), // sub_142479a60
    Replacement::draft_throws(0x142479c60, "r_142479c60", fn_142479c60 as *const (), Throws::Discard), // sub_142479c60
    Replacement::draft_throws(0x14247a030, "r_14247a030", fn_14247a030 as *const (), Throws::Caught), // sub_14247a030
    Replacement::draft_throws(0x14247a110, "r_14247a110", crate::shards::subsystems::fn_14247a110 as *const (), Throws::Caught), // sub_14247a110
    Replacement::draft(0x14247a130, "r_14247a130", fn_14247a130 as *const ()), // sub_14247a130
    Replacement::draft_throws(0x14247a260, "r_14247a260", fn_14247a260 as *const (), Throws::Discard), // sub_14247a260
    Replacement::draft(0x14247a720, "r_14247a720", fn_14247a720 as *const ()), // sub_14247a720
    Replacement::draft(0x14247a740, "r_14247a740", fn_14247a740 as *const ()), // sub_14247a740
    Replacement::draft_throws(0x14247a890, "r_14247a890", fn_14247a890 as *const (), Throws::Caught), // sub_14247a890
    Replacement::draft(0x14247a960, "r_14247a960", tpl_6495::<0x8, 0x14240ab10> as *const ()), // sub_14247a960
    Replacement::draft(0x14247a9b0, "r_14247a9b0", fn_14247a9b0 as *const ()), // sub_14247a9b0
    Replacement::draft(0x14247a9d0, "r_14247a9d0", fn_14247a9d0 as *const ()), // sub_14247a9d0
    Replacement::draft_throws(0x14247aa10, "r_14247aa10", fn_14247aa10 as *const (), Throws::Discard), // sub_14247aa10
    Replacement::draft(0x14247aad0, "r_14247aad0", fn_14247aad0 as *const ()), // sub_14247aad0
    Replacement::draft(0x14247ac70, "r_14247ac70", fn_14247ac70 as *const ()), // sub_14247ac70
    Replacement::draft(0x14247aca0, "r_14247aca0", fn_14247aca0 as *const ()), // sub_14247aca0
    Replacement::draft(0x14247ace0, "r_14247ace0", tpl_34105::<0x8> as *const ()), // sub_14247ace0
    Replacement::draft(0x14247af00, "r_14247af00", fn_14247af00 as *const ()), // sub_14247af00
    Replacement::draft(0x14247af90, "r_14247af90", fn_14247af90 as *const ()), // sub_14247af90
    Replacement::draft(0x14247b080, "r_14247b080", fn_14247b080 as *const ()), // sub_14247b080
    Replacement::draft(0x14247b250, "r_14247b250", fn_14247b250 as *const ()), // sub_14247b250
    Replacement::draft(0x14247b440, "r_14247b440", fn_14247b440 as *const ()), // sub_14247b440
    Replacement::draft(0x14247b4c0, "r_14247b4c0", fn_14247b4c0 as *const ()), // sub_14247b4c0
    Replacement::draft(0x14247b5e0, "r_14247b5e0", fn_14247b5e0 as *const ()), // sub_14247b5e0
    Replacement::draft(0x14247b600, "r_14247b600", fn_14247b600 as *const ()), // sub_14247b600
    Replacement::draft(0x14247b650, "r_14247b650", tpl_44130::<0x10, 0x8> as *const ()), // sub_14247b650
    Replacement::draft(0x14247b690, "r_14247b690", fn_14247b690 as *const ()), // sub_14247b690
    Replacement::draft(0x14247b6d0, "r_14247b6d0", fn_14247b6d0 as *const ()), // sub_14247b6d0
    Replacement::draft(0x14247b790, "r_14247b790", fn_14247b790 as *const ()), // sub_14247b790
    Replacement::draft(0x14247b7c0, "r_14247b7c0", tpl_44142::<0x8> as *const ()), // sub_14247b7c0
    Replacement::draft(0x14247b7e0, "r_14247b7e0", fn_14247b7e0 as *const ()), // sub_14247b7e0
    Replacement::draft(0x14247b800, "r_14247b800", tpl_44143::<0x8> as *const ()), // sub_14247b800
    Replacement::draft(0x14247b840, "r_14247b840", crate::shards::subsystems::fn_14247b840 as *const ()), // sub_14247b840
    Replacement::draft_throws(0x14247b8f0, "r_14247b8f0", fn_14247b8f0 as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_7
    Replacement::draft_throws(0x14247b920, "r_14247b920", fn_14247b920 as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_8
    Replacement::draft_throws(0x14247b950, "r_14247b950", fn_14247b950 as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_5
    Replacement::draft_throws(0x14247b980, "r_14247b980", fn_14247b980 as *const (), Throws::Discard), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_6
    Replacement::draft(0x14247b9b0, "r_14247b9b0", fn_14247b9b0 as *const ()), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_4
    Replacement::draft(0x14247b9e0, "r_14247b9e0", fn_14247b9e0 as *const ()), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_3
    Replacement::draft(0x14247ba10, "r_14247ba10", fn_14247ba10 as *const ()), // sub_14247ba10
    Replacement::draft_throws(0x14247ba60, "r_14247ba60", fn_14247ba60 as *const (), Throws::Caught), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_2
    Replacement::draft(0x14247bc80, "r_14247bc80", fn_14247bc80 as *const ()), // r::Loader_Adaptor_FileSystem::r::Loader_Adaptor_FileSystem::vfunc_1
    Replacement::draft(0x14247ef90, "r_14247ef90", fn_14247ef90 as *const ()), // r::LargeBlockMemoryResource::r::LargeBlockMemoryResource::vfunc_2
    Replacement::draft(0x142491960, "r_142491960", fn_142491960 as *const ()), // r::FolderWatcherMonitor::r::FolderWatcherMonitor::vfunc_1
    Replacement::draft(0x1424919b0, "r_1424919b0", tpl_51014::<0x134, 0x150, 0x148, 0x1424915e0, 0x38, 0xffffffffffffffff, 0x1426453d0, 0x1426455b0, 0x138, 0x142645748, 0x142442de0, 0x8, 0x14240ab10> as *const ()), // sub_1424919b0
    Replacement::draft_throws(0x142491a80, "r_142491a80", fn_142491a80 as *const (), Throws::Discard), // sub_142491a80
    Replacement::draft_throws(0x142491ad0, "r_142491ad0", fn_142491ad0 as *const (), Throws::Caught), // sub_142491ad0
    Replacement::draft(0x142491c30, "r_142491c30", fn_142491c30 as *const ()), // sub_142491c30
    Replacement::draft(0x142491c80, "r_142491c80", fn_142491c80 as *const ()), // sub_142491c80
    Replacement::draft(0x1424928a0, "r_1424928a0", fn_1424928a0 as *const ()), // sub_1424928a0
    Replacement::draft(0x1424928c0, "r_1424928c0", fn_1424928c0 as *const ()), // sub_1424928c0
    Replacement::draft(0x142492950, "r_142492950", fn_142492950 as *const ()), // sub_142492950
    Replacement::draft(0x1424929c0, "r_1424929c0", fn_1424929c0 as *const ()), // sub_1424929c0
    Replacement::draft(0x142492a50, "r_142492a50", fn_142492a50 as *const ()), // sub_142492a50
    Replacement::draft(0x142492a90, "r_142492a90", fn_142492a90 as *const ()), // sub_142492a90
    Replacement::draft(0x142492cf0, "r_142492cf0", fn_142492cf0 as *const ()), // sub_142492cf0
    Replacement::draft(0x142492e90, "r_142492e90", fn_142492e90 as *const ()), // sub_142492e90
    Replacement::draft(0x142492f30, "r_142492f30", fn_142492f30 as *const ()), // sub_142492f30
    Replacement::draft_throws(0x142492f90, "r_142492f90", fn_142492f90 as *const (), Throws::Caught), // sub_142492f90
    Replacement::draft(0x1424934c0, "r_1424934c0", fn_1424934c0 as *const ()), // sub_1424934c0
    Replacement::draft(0x142493580, "r_142493580", fn_142493580 as *const ()), // sub_142493580
    Replacement::draft_throws(0x1424936b0, "r_1424936b0", fn_1424936b0 as *const (), Throws::Caught), // sub_1424936b0
    Replacement::draft(0x1424937c0, "r_1424937c0", fn_1424937c0 as *const ()), // sub_1424937c0
    Replacement::draft(0x1424939d0, "r_1424939d0", fn_1424939d0 as *const ()), // sub_1424939d0
    Replacement::draft(0x142493a50, "r_142493a50", fn_142493a50 as *const ()), // sub_142493a50
    Replacement::draft_throws(0x142493e70, "r_142493e70", fn_142493e70 as *const (), Throws::Caught), // r::FileSystemWatcher::r::FileSystemWatcher::vfunc_1
    Replacement::draft_throws(0x142494120, "r_142494120", fn_142494120 as *const (), Throws::Discard), // sub_142494120
    Replacement::draft_throws(0x1424942a0, "r_1424942a0", fn_1424942a0 as *const (), Throws::Caught), // sub_1424942a0
    Replacement::draft(0x142494430, "r_142494430", fn_142494430 as *const ()), // sub_142494430
    Replacement::draft(0x142494520, "r_142494520", fn_142494520 as *const ()), // sub_142494520
    Replacement::draft_throws(0x1424946a0, "r_1424946a0", fn_1424946a0 as *const (), Throws::Caught), // sub_1424946a0
    Replacement::draft(0x142494980, "r_142494980", fn_142494980 as *const ()), // sub_142494980
    Replacement::draft_throws(0x142494a30, "r_142494a30", fn_142494a30 as *const (), Throws::Caught), // sub_142494a30
    Replacement::draft_throws(0x142494ad0, "r_142494ad0", fn_142494ad0 as *const (), Throws::Discard), // sub_142494ad0
    Replacement::draft_throws(0x142494b10, "r_142494b10", fn_142494b10 as *const (), Throws::Caught), // sub_142494b10
    Replacement::draft_throws(0x142494c40, "r_142494c40", fn_142494c40 as *const (), Throws::Caught), // sub_142494c40
    Replacement::draft_throws(0x142494dc0, "r_142494dc0", fn_142494dc0 as *const (), Throws::Discard), // sub_142494dc0
    Replacement::draft_throws(0x142494f10, "r_142494f10", fn_142494f10 as *const (), Throws::Caught), // sub_142494f10
    Replacement::draft_throws(0x1424950b0, "r_1424950b0", fn_1424950b0 as *const (), Throws::Caught), // sub_1424950b0
    Replacement::draft_throws(0x142495160, "r_142495160", fn_142495160 as *const (), Throws::Caught), // sub_142495160
    Replacement::draft(0x142495240, "r_142495240", fn_142495240 as *const ()), // sub_142495240
    Replacement::draft(0x142495330, "r_142495330", fn_142495330 as *const ()), // sub_142495330
    Replacement::draft(0x142495360, "r_142495360", fn_142495360 as *const ()), // sub_142495360
    Replacement::draft(0x1424953e0, "r_1424953e0", fn_1424953e0 as *const ()), // sub_1424953e0
    Replacement::draft(0x142495460, "r_142495460", fn_142495460 as *const ()), // sub_142495460
    Replacement::draft(0x142495560, "r_142495560", fn_142495560 as *const ()), // r::FileStream::r::FileStream::vfunc_20
    Replacement::draft(0x1424955c0, "r_1424955c0", fn_1424955c0 as *const ()), // r::FileStream::r::FileStream::vfunc_21
    Replacement::draft(0x142495610, "r_142495610", fn_142495610 as *const ()), // r::FileStream::r::FileStream::vfunc_22
    Replacement::draft(0x142495660, "r_142495660", fn_142495660 as *const ()), // r::FileStream::r::FileStream::vfunc_23
    Replacement::draft(0x1424956b0, "r_1424956b0", fn_1424956b0 as *const ()), // r::FileStream::r::FileStream::vfunc_24
    Replacement::draft(0x142495910, "r_142495910", fn_142495910 as *const ()), // r::FileStream::r::FileStream::vfunc_3
    Replacement::draft(0x1424959b0, "r_1424959b0", fn_1424959b0 as *const ()), // r::FileStream::r::FileStream::vfunc_4
    Replacement::draft(0x142495be0, "r_142495be0", fn_142495be0 as *const ()), // sub_142495be0
    Replacement::draft(0x142495c80, "r_142495c80", fn_142495c80 as *const ()), // sub_142495c80
    Replacement::draft(0x142495ed0, "r_142495ed0", tpl_51061::<0x142409b50, 0x142495c80, 0x10, 0x30, 0x40, 0x14240ab10> as *const ()), // sub_142495ed0
    Replacement::draft(0x142495fe0, "r_142495fe0", tpl_51061::<0x142409b50, 0x142495c80, 0x10, 0x30, 0x40, 0x14240ab10> as *const ()), // sub_142495fe0
    Replacement::draft(0x1424960f0, "r_1424960f0", tpl_51061::<0x142409b50, 0x142495c80, 0x10, 0x30, 0x40, 0x14240ab10> as *const ()), // sub_1424960f0
    Replacement::draft(0x142496200, "r_142496200", tpl_51061::<0x142409b50, 0x142495c80, 0x10, 0x30, 0x40, 0x14240ab10> as *const ()), // sub_142496200
    Replacement::draft(0x142496310, "r_142496310", fn_142496310 as *const ()), // sub_142496310
    Replacement::draft(0x1424963d0, "r_1424963d0", fn_1424963d0 as *const ()), // sub_1424963d0
    Replacement::draft(0x142496490, "r_142496490", fn_142496490 as *const ()), // sub_142496490
    Replacement::draft_throws(0x142496540, "r_142496540", fn_142496540 as *const (), Throws::Caught), // sub_142496540
    Replacement::draft(0x142496700, "r_142496700", fn_142496700 as *const ()), // sub_142496700
    Replacement::draft(0x142496730, "r_142496730", fn_142496730 as *const ()), // sub_142496730
    Replacement::draft(0x1424967d0, "r_1424967d0", fn_1424967d0 as *const ()), // sub_1424967d0
    Replacement::draft(0x142496800, "r_142496800", fn_142496800 as *const ()), // sub_142496800
    Replacement::draft(0x142496870, "r_142496870", fn_142496870 as *const ()), // sub_142496870
    Replacement::draft(0x1424968a0, "r_1424968a0", fn_1424968a0 as *const ()), // sub_1424968a0
    Replacement::draft(0x142496950, "r_142496950", fn_142496950 as *const ()), // sub_142496950
    Replacement::draft(0x1424969b0, "r_1424969b0", fn_1424969b0 as *const ()), // r::FileArchive::File::r::FileArchive::File::vfunc_8
    Replacement::draft_throws(0x1424969f0, "r_1424969f0", fn_1424969f0 as *const (), Throws::Caught), // r::FileArchive::File::r::FileArchive::File::vfunc_7
    Replacement::draft(0x142496a70, "r_142496a70", fn_142496a70 as *const ()), // r::FileArchive::File::r::FileArchive::File::vfunc_6
    Replacement::draft(0x142496b50, "r_142496b50", fn_142496b50 as *const ()), // sub_142496b50
    Replacement::draft(0x142496ba0, "r_142496ba0", fn_142496ba0 as *const ()), // sub_142496ba0
    Replacement::draft(0x142496c20, "r_142496c20", fn_142496c20 as *const ()), // sub_142496c20
    Replacement::draft(0x142496e30, "r_142496e30", fn_142496e30 as *const ()), // sub_142496e30
    Replacement::draft(0x1424970f0, "r_1424970f0", fn_1424970f0 as *const ()), // sub_1424970f0
    Replacement::draft(0x142497410, "r_142497410", fn_142497410 as *const ()), // sub_142497410
    Replacement::draft_throws(0x142497470, "r_142497470", fn_142497470 as *const (), Throws::Caught), // sub_142497470
    Replacement::draft(0x1424974e0, "r_1424974e0", fn_1424974e0 as *const ()), // sub_1424974e0
    Replacement::draft_throws(0x142497620, "r_142497620", fn_142497620 as *const (), Throws::Caught), // sub_142497620
    Replacement::draft(0x1424976f0, "r_1424976f0", fn_1424976f0 as *const ()), // sub_1424976f0
    Replacement::draft(0x142497770, "r_142497770", fn_142497770 as *const ()), // sub_142497770
    Replacement::draft(0x1424977f0, "r_1424977f0", fn_1424977f0 as *const ()), // sub_1424977f0
    Replacement::draft(0x142497910, "r_142497910", fn_142497910 as *const ()), // sub_142497910
    Replacement::draft(0x142497ad0, "r_142497ad0", fn_142497ad0 as *const ()), // sub_142497ad0
    Replacement::draft(0x142497b80, "r_142497b80", crate::shards::subsystems::fn_142497b80 as *const ()), // sub_142497b80
    Replacement::draft(0x142497be0, "r_142497be0", fn_142497be0 as *const ()), // sub_142497be0
    Replacement::draft_throws(0x142497c10, "r_142497c10", fn_142497c10 as *const (), Throws::Caught), // sub_142497c10
    Replacement::draft_throws(0x142497eb0, "r_142497eb0", fn_142497eb0 as *const (), Throws::Caught), // r::File::r::File::vfunc_2
    Replacement::draft_throws(0x142497fd0, "r_142497fd0", fn_142497fd0 as *const (), Throws::Caught), // sub_142497fd0
    Replacement::draft_throws(0x142498350, "r_142498350", fn_142498350 as *const (), Throws::Caught), // sub_142498350
    Replacement::draft_throws(0x1424983c0, "r_1424983c0", fn_1424983c0 as *const (), Throws::Caught), // sub_1424983c0
    Replacement::draft_throws(0x142498440, "r_142498440", fn_142498440 as *const (), Throws::Caught), // sub_142498440
    Replacement::draft(0x142498500, "r_142498500", fn_142498500 as *const ()), // sub_142498500
    Replacement::draft(0x142498600, "r_142498600", crate::shards::subsystems::fn_142498600 as *const ()), // sub_142498600
    Replacement::draft(0x142498620, "r_142498620", fn_142498620 as *const ()), // sub_142498620
    Replacement::draft_throws(0x1424986b0, "r_1424986b0", fn_1424986b0 as *const (), Throws::Caught), // r::File::r::File::vfunc_8
    Replacement::draft_throws(0x142498740, "r_142498740", fn_142498740 as *const (), Throws::Caught), // r::File::r::File::vfunc_6
    Replacement::draft_throws(0x142498af0, "r_142498af0", fn_142498af0 as *const (), Throws::Caught), // sub_142498af0
    Replacement::draft_throws(0x142498b70, "r_142498b70", fn_142498b70 as *const (), Throws::Caught), // sub_142498b70
    Replacement::draft(0x142498bc0, "r_142498bc0", fn_142498bc0 as *const ()), // sub_142498bc0
    Replacement::draft_throws(0x142498c90, "r_142498c90", fn_142498c90 as *const (), Throws::Caught), // sub_142498c90
    Replacement::draft(0x142498df0, "r_142498df0", fn_142498df0 as *const ()), // sub_142498df0
    Replacement::draft(0x142498e20, "r_142498e20", fn_142498e20 as *const ()), // sub_142498e20
    Replacement::draft(0x142498e60, "r_142498e60", fn_142498e60 as *const ()), // sub_142498e60
    Replacement::draft(0x142498f00, "r_142498f00", fn_142498f00 as *const ()), // sub_142498f00
    Replacement::draft(0x142498fc0, "r_142498fc0", tpl_604::<0x8> as *const ()), // sub_142498fc0
    Replacement::draft(0x142498fe0, "r_142498fe0", tpl_50488::<0x14340ed68, 0x14340ed48, 0x8> as *const ()), // sub_142498fe0
    Replacement::draft_throws(0x142499000, "r_142499000", fn_142499000 as *const (), Throws::Discard), // sub_142499000
    Replacement::draft_throws(0x142499280, "r_142499280", fn_142499280 as *const (), Throws::Caught), // sub_142499280
    Replacement::draft_throws(0x142499560, "r_142499560", fn_142499560 as *const (), Throws::Discard), // sub_142499560
    Replacement::draft(0x1424995a0, "r_1424995a0", fn_1424995a0 as *const ()), // sub_1424995a0
    Replacement::draft(0x142499680, "r_142499680", tpl_51109::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_32
    Replacement::draft(0x1424996d0, "r_1424996d0", fn_1424996d0 as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_19
    Replacement::draft(0x142499750, "r_142499750", tpl_51109::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_25
    Replacement::draft(0x1424997a0, "r_1424997a0", fn_1424997a0 as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_26
    Replacement::draft(0x142499820, "r_142499820", tpl_51112::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_27
    Replacement::draft(0x142499860, "r_142499860", tpl_51112::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_28
    Replacement::draft(0x1424998a0, "r_1424998a0", tpl_51112::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_29
    Replacement::draft(0x1424998e0, "r_1424998e0", tpl_51113::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_30
    Replacement::draft(0x142499920, "r_142499920", tpl_51113::<0x10, 0x8, 0x30> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_31
    Replacement::draft(0x142499960, "r_142499960", tpl_51114::<0x8, 0x28, 0x10, 0x1> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_12
    Replacement::draft(0x1424999b0, "r_1424999b0", tpl_51115::<0x8, 0x28, 0x10, 0x7, 0x1, 0x6, 0x2, 0x5, 0x3, 0x4> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_2
    Replacement::draft(0x142499a20, "r_142499a20", tpl_51114::<0x8, 0x28, 0x10, 0x1> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_5
    Replacement::draft(0x142499a70, "r_142499a70", tpl_51115::<0x8, 0x28, 0x10, 0x7, 0x1, 0x6, 0x2, 0x5, 0x3, 0x4> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_6
    Replacement::draft(0x142499ae0, "r_142499ae0", tpl_51116::<0x8, 0x28, 0x10, 0x3, 0x1, 0x2> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_7
    Replacement::draft(0x142499b40, "r_142499b40", tpl_51116::<0x8, 0x28, 0x10, 0x3, 0x1, 0x2> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_8
    Replacement::draft(0x142499ba0, "r_142499ba0", tpl_51116::<0x8, 0x28, 0x10, 0x3, 0x1, 0x2> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_9
    Replacement::draft(0x142499c00, "r_142499c00", tpl_51114::<0x8, 0x28, 0x10, 0x1> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_10
    Replacement::draft(0x142499c50, "r_142499c50", tpl_51114::<0x8, 0x28, 0x10, 0x1> as *const ()), // r::EndianFileStream::r::EndianFileStream::vfunc_11
    Replacement::draft(0x142499ca0, "r_142499ca0", fn_142499ca0 as *const ()), // sub_142499ca0
    Replacement::draft(0x142499cd0, "r_142499cd0", fn_142499cd0 as *const ()), // sub_142499cd0
    Replacement::draft(0x142499d20, "r_142499d20", fn_142499d20 as *const ()), // sub_142499d20
    Replacement::draft_throws(0x142499d40, "r_142499d40", fn_142499d40 as *const (), Throws::Discard), // sub_142499d40
    Replacement::draft(0x142499eb0, "r_142499eb0", fn_142499eb0 as *const ()), // sub_142499eb0
    Replacement::draft(0x142499f20, "r_142499f20", fn_142499f20 as *const ()), // sub_142499f20
    Replacement::draft_throws(0x14249a0c0, "r_14249a0c0", fn_14249a0c0 as *const (), Throws::Caught), // sub_14249a0c0
    Replacement::draft_throws(0x14249abf0, "r_14249abf0", fn_14249abf0 as *const (), Throws::Discard), // sub_14249abf0
    Replacement::draft_throws(0x14249ac20, "r_14249ac20", fn_14249ac20 as *const (), Throws::Caught), // sub_14249ac20
    Replacement::draft(0x14249ad60, "r_14249ad60", fn_14249ad60 as *const ()), // sub_14249ad60
    Replacement::draft_throws(0x14249ad80, "r_14249ad80", fn_14249ad80 as *const (), Throws::Discard), // sub_14249ad80
    Replacement::draft_throws(0x14249adf0, "r_14249adf0", fn_14249adf0 as *const (), Throws::Discard), // sub_14249adf0
    Replacement::draft_throws(0x14249ae10, "r_14249ae10", fn_14249ae10 as *const (), Throws::Discard), // sub_14249ae10
    Replacement::draft(0x14249af20, "r_14249af20", fn_14249af20 as *const ()), // sub_14249af20
    Replacement::draft_throws(0x14249afa0, "r_14249afa0", fn_14249afa0 as *const (), Throws::Caught), // sub_14249afa0
    Replacement::draft_throws(0x14249b0d0, "r_14249b0d0", fn_14249b0d0 as *const (), Throws::Discard), // sub_14249b0d0
    Replacement::draft_throws(0x14249b110, "r_14249b110", fn_14249b110 as *const (), Throws::Discard), // sub_14249b110
    Replacement::draft(0x14249b310, "r_14249b310", fn_14249b310 as *const ()), // sub_14249b310
    Replacement::draft(0x14249b370, "r_14249b370", fn_14249b370 as *const ()), // sub_14249b370
    Replacement::draft_throws(0x14249b3e0, "r_14249b3e0", fn_14249b3e0 as *const (), Throws::Discard), // sub_14249b3e0
    Replacement::draft_throws(0x14249b400, "r_14249b400", fn_14249b400 as *const (), Throws::Discard), // sub_14249b400
    Replacement::draft_throws(0x14249b420, "r_14249b420", fn_14249b420 as *const (), Throws::Discard), // sub_14249b420
    Replacement::draft_throws(0x14249b4a0, "r_14249b4a0", fn_14249b4a0 as *const (), Throws::Discard), // sub_14249b4a0
    Replacement::draft_throws(0x14249b540, "r_14249b540", fn_14249b540 as *const (), Throws::Discard), // sub_14249b540
    Replacement::draft(0x14249b580, "r_14249b580", fn_14249b580 as *const ()), // sub_14249b580
    Replacement::draft(0x14249b5f0, "r_14249b5f0", fn_14249b5f0 as *const ()), // sub_14249b5f0
    Replacement::draft_throws(0x14249b630, "r_14249b630", fn_14249b630 as *const (), Throws::Caught), // sub_14249b630
    Replacement::draft_throws(0x14249b810, "r_14249b810", fn_14249b810 as *const (), Throws::Caught), // sub_14249b810
    Replacement::draft(0x14249b8f0, "r_14249b8f0", fn_14249b8f0 as *const ()), // sub_14249b8f0
    Replacement::draft(0x14249b980, "r_14249b980", tpl_51145::<0x143412780, 0x14240a460, 0x30, 0x14240ab10> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_11
    Replacement::draft(0x14249ba30, "r_14249ba30", tpl_51145::<0x143412780, 0x14240a460, 0x20, 0x14240ab10> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_10
    Replacement::draft(0x14249bae0, "r_14249bae0", tpl_51145::<0x143412780, 0x14240a460, 0x10, 0x14240ab10> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_9
    Replacement::draft(0x14249bb90, "r_14249bb90", tpl_51146::<0x40> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_7
    Replacement::draft(0x14249bbc0, "r_14249bbc0", tpl_51146::<0x30> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_5
    Replacement::draft(0x14249bbf0, "r_14249bbf0", tpl_51146::<0x20> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_3
    Replacement::draft(0x14249bc20, "r_14249bc20", tpl_51146::<0x10> as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_1
    Replacement::draft(0x14249bc50, "r_14249bc50", fn_14249bc50 as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_8
    Replacement::draft(0x14249bd20, "r_14249bd20", fn_14249bd20 as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_6
    Replacement::draft(0x14249bdf0, "r_14249bdf0", fn_14249bdf0 as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_4
    Replacement::draft(0x14249bec0, "r_14249bec0", fn_14249bec0 as *const ()), // r::DefaultAppReporter::r::DefaultAppReporter::vfunc_2
    Replacement::draft(0x1424a7860, "r_1424a7860", fn_1424a7860 as *const ()), // r::ConvexVolume::ClassTypeInfoImplementation::r::ConvexVolume::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424a7900, "r_1424a7900", fn_1424a7900 as *const ()), // r::ConvexVolume::ClassTypeInfoImplementation::r::ConvexVolume::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1424a7a20, "r_1424a7a20", fn_1424a7a20 as *const ()), // sub_1424a7a20
    Replacement::draft(0x1424a7aa0, "r_1424a7aa0", fn_1424a7aa0 as *const ()), // sub_1424a7aa0
    Replacement::draft(0x1424a7b00, "r_1424a7b00", fn_1424a7b00 as *const ()), // sub_1424a7b00
    Replacement::draft(0x1424a7c50, "r_1424a7c50", fn_1424a7c50 as *const ()), // sub_1424a7c50
    Replacement::draft(0x1424a81b0, "r_1424a81b0", tpl_38874::<0x8, 0x1404b9990, 0x28, 0x140941b10> as *const ()), // r::ContentEntityBase::ClassTypeInfoImplementation::r::ContentEntityBase::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424a8220, "r_1424a8220", fn_1424a8220 as *const ()), // r::ContentEntityBase::ClassTypeInfoImplementation::r::ContentEntityBase::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1424a8500, "r_1424a8500", fn_1424a8500 as *const ()), // sub_1424a8500
    Replacement::draft(0x1424a8600, "r_1424a8600", fn_1424a8600 as *const ()), // sub_1424a8600
    Replacement::draft(0x1424a8800, "r_1424a8800", tpl_51291::<0x142409b50, 0x1424a8500, 0x14240ab10> as *const ()), // r::ConsoleLogWriter::r::ConsoleLogWriter::vfunc_4
    Replacement::draft(0x1424a8880, "r_1424a8880", tpl_51291::<0x142409b50, 0x1424a8500, 0x14240ab10> as *const ()), // r::ConsoleLogWriter::r::ConsoleLogWriter::vfunc_3
    Replacement::draft(0x1424a8900, "r_1424a8900", tpl_51291::<0x142409b50, 0x1424a8500, 0x14240ab10> as *const ()), // r::ConsoleLogWriter::r::ConsoleLogWriter::vfunc_2
    Replacement::draft(0x1424a8980, "r_1424a8980", tpl_51291::<0x142409b50, 0x1424a8500, 0x14240ab10> as *const ()), // r::ConsoleLogWriter::r::ConsoleLogWriter::vfunc_1
    Replacement::draft_throws(0x1424a9470, "r_1424a9470", fn_1424a9470 as *const (), Throws::Caught), // r::CommandLine::r::CommandLine::vfunc_3
    Replacement::draft(0x1424a9ff0, "r_1424a9ff0", fn_1424a9ff0 as *const ()), // r::CommandLine::r::CommandLine::vfunc_2
    Replacement::draft(0x1424aa640, "r_1424aa640", fn_1424aa640 as *const ()), // r::CommandLine::r::CommandLine::vfunc_1
    Replacement::draft_throws(0x1424ab5c0, "r_1424ab5c0", fn_1424ab5c0 as *const (), Throws::Caught), // r::CmdLineParameterMultiString::r::CmdLineParameterMultiString::vfunc_0
    Replacement::draft_throws(0x1424ab810, "r_1424ab810", fn_1424ab810 as *const (), Throws::Caught), // r::CmdLineParameterMultiString::r::CmdLineParameterMultiString::vfunc_4
    Replacement::draft_throws(0x1424aba80, "r_1424aba80", fn_1424aba80 as *const (), Throws::Caught), // r::CmdLineParameterInteger::r::CmdLineParameterInteger::vfunc_1
    Replacement::draft_throws(0x1424abf60, "r_1424abf60", fn_1424abf60 as *const (), Throws::Caught), // r::CmdLineParameterInteger::r::CmdLineParameterInteger::vfunc_4
    Replacement::draft(0x1424ac340, "r_1424ac340", fn_1424ac340 as *const ()), // r::CmdLineParameterBoolean::r::CmdLineParameterBoolean::vfunc_4
    Replacement::draft_throws(0x1424ac5e0, "r_1424ac5e0", fn_1424ac5e0 as *const (), Throws::Discard), // r::CmdLineParameterBoolean::r::CmdLineParameterBoolean::vfunc_0
    Replacement::draft_throws(0x1424ac670, "r_1424ac670", fn_1424ac670 as *const (), Throws::Caught), // r::CmdLineParameterOption::r::CmdLineParameterOption::vfunc_4
    Replacement::draft_throws(0x1424ac960, "r_1424ac960", tpl_51306::<0xb, 0x10, 0x8, 0x1434107b0, 0x138, 0x1413d81d0> as *const (), Throws::Discard), // r::CmdLineParameterOption::r::CmdLineParameterOption::vfunc_0
    Replacement::draft(0x1424aca00, "r_1424aca00", fn_1424aca00 as *const ()), // sub_1424aca00
    Replacement::draft_throws(0x1424acb40, "r_1424acb40", fn_1424acb40 as *const (), Throws::Caught), // r::CmdLineParameterString::r::CmdLineParameterString::vfunc_1
    Replacement::draft_throws(0x1424acf30, "r_1424acf30", tpl_51306::<0xb, 0x10, 0x8, 0x1434107b0, 0x138, 0x1413d81d0> as *const (), Throws::Discard), // r::CmdLineParameterString::r::CmdLineParameterString::vfunc_0
    Replacement::draft(0x1424acfd0, "r_1424acfd0", fn_1424acfd0 as *const ()), // r::CmdLineParameterString::r::CmdLineParameterString::vfunc_4
    Replacement::draft_throws(0x1424ad140, "r_1424ad140", fn_1424ad140 as *const (), Throws::Discard), // r::CmdLineParameterCategory::r::CmdLineParameterCategory::vfunc_0
    Replacement::draft_throws(0x1424b4690, "r_1424b4690", crate::shards::subsystems::fn_1424b4690 as *const (), Throws::Caught), // r::CachedFile::r::CachedFile::vfunc_7
    Replacement::draft_throws(0x1424b4710, "r_1424b4710", fn_1424b4710 as *const (), Throws::Caught), // r::CachedFile::r::CachedFile::vfunc_5
    Replacement::draft_throws(0x1424b4b30, "r_1424b4b30", fn_1424b4b30 as *const (), Throws::Caught), // sub_1424b4b30
    Replacement::draft_throws(0x1424b52c0, "r_1424b52c0", fn_1424b52c0 as *const (), Throws::Caught), // sub_1424b52c0
    Replacement::draft(0x1424b5390, "r_1424b5390", fn_1424b5390 as *const ()), // sub_1424b5390
    Replacement::draft(0x1424b54f0, "r_1424b54f0", fn_1424b54f0 as *const ()), // sub_1424b54f0
    Replacement::draft_throws(0x1424b55b0, "r_1424b55b0", fn_1424b55b0 as *const (), Throws::Caught), // sub_1424b55b0
    Replacement::draft(0x1424b56d0, "r_1424b56d0", fn_1424b56d0 as *const ()), // sub_1424b56d0
    Replacement::draft(0x1424b5740, "r_1424b5740", fn_1424b5740 as *const ()), // sub_1424b5740
    Replacement::draft_throws(0x1424b5820, "r_1424b5820", fn_1424b5820 as *const (), Throws::Caught), // r::BufferedMemoryFile::r::BufferedMemoryFile::vfunc_7
    Replacement::draft(0x1424b58f0, "r_1424b58f0", fn_1424b58f0 as *const ()), // r::BufferedMemoryFile::r::BufferedMemoryFile::vfunc_6
    Replacement::draft_throws(0x1424b5970, "r_1424b5970", fn_1424b5970 as *const (), Throws::Caught), // r::BufferedMemoryFile::r::BufferedMemoryFile::vfunc_5
    Replacement::draft(0x1424b5a30, "r_1424b5a30", tpl_49810::<0x128, 0x118> as *const ()), // r::BufferedMemoryFile::r::BufferedMemoryFile::vfunc_2
    Replacement::draft(0x1424b5a90, "r_1424b5a90", fn_1424b5a90 as *const ()), // sub_1424b5a90
    Replacement::draft_throws(0x1424b5b70, "r_1424b5b70", fn_1424b5b70 as *const (), Throws::Caught), // sub_1424b5b70
    Replacement::draft_throws(0x1424b5d70, "r_1424b5d70", fn_1424b5d70 as *const (), Throws::Caught), // sub_1424b5d70
    Replacement::draft_throws(0x1424b67b0, "r_1424b67b0", fn_1424b67b0 as *const (), Throws::Discard), // sub_1424b67b0
    Replacement::draft(0x1424b6aa0, "r_1424b6aa0", fn_1424b6aa0 as *const ()), // sub_1424b6aa0
    Replacement::draft(0x1424b6d30, "r_1424b6d30", fn_1424b6d30 as *const ()), // sub_1424b6d30
    Replacement::draft(0x1424b6d70, "r_1424b6d70", fn_1424b6d70 as *const ()), // sub_1424b6d70
    Replacement::draft_throws(0x1424b7040, "r_1424b7040", fn_1424b7040 as *const (), Throws::Caught), // sub_1424b7040
    Replacement::draft(0x1424b74b0, "r_1424b74b0", fn_1424b74b0 as *const ()), // sub_1424b74b0
    Replacement::draft(0x1424b7680, "r_1424b7680", fn_1424b7680 as *const ()), // sub_1424b7680
    Replacement::draft(0x1424b7ca0, "r_1424b7ca0", tpl_39008::<0x8, 0xc> as *const ()), // r::BoundSphere::ClassTypeInfoImplementation::r::BoundSphere::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x1424b7cd0, "r_1424b7cd0", tpl_39008::<0x8, 0xc> as *const ()), // r::BoundSphere::ClassTypeInfoImplementation::r::BoundSphere::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x1424b8000, "r_1424b8000", fn_1424b8000 as *const ()), // sub_1424b8000
    Replacement::draft(0x1424b8080, "r_1424b8080", fn_1424b8080 as *const ()), // sub_1424b8080
    Replacement::draft(0x1424b83d0, "r_1424b83d0", tpl_51418::<0x8, 0xc, 0x14> as *const ()), // r::BoundBox::ClassTypeInfoImplementation::r::BoundBox::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x1424b8410, "r_1424b8410", tpl_51418::<0x8, 0xc, 0x14> as *const ()), // r::BoundBox::ClassTypeInfoImplementation::r::BoundBox::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x1424b8550, "r_1424b8550", tpl_35199::<0x1438820c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::BoundBox::ClassTypeInfoImplementation::r::BoundBox::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1424b85b0, "r_1424b85b0", tpl_35199::<0x1438820c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // r::BoundBox::ClassTypeInfoImplementation::r::BoundBox::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1424b8640, "r_1424b8640", tpl_35226::<0x10> as *const ()), // r::BoundBox::ClassTypeInfoImplementation::r::BoundBox::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424b8680, "r_1424b8680", fn_1424b8680 as *const ()), // r::BoundBox::ClassTypeInfoImplementation::r::BoundBox::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1424b88d0, "r_1424b88d0", fn_1424b88d0 as *const ()), // sub_1424b88d0
    Replacement::draft(0x1424b8940, "r_1424b8940", fn_1424b8940 as *const ()), // sub_1424b8940
    Replacement::draft(0x1424b89a0, "r_1424b89a0", fn_1424b89a0 as *const ()), // sub_1424b89a0
    Replacement::draft(0x1424b8c40, "r_1424b8c40", fn_1424b8c40 as *const ()), // sub_1424b8c40
    Replacement::draft(0x1424b8df0, "r_1424b8df0", fn_1424b8df0 as *const ()), // sub_1424b8df0
    Replacement::draft(0x1424b8e50, "r_1424b8e50", fn_1424b8e50 as *const ()), // sub_1424b8e50
    Replacement::draft(0x1424b8fb0, "r_1424b8fb0", fn_1424b8fb0 as *const ()), // sub_1424b8fb0
    Replacement::draft(0x1424b90b0, "r_1424b90b0", fn_1424b90b0 as *const ()), // sub_1424b90b0
    Replacement::draft(0x1424b9270, "r_1424b9270", fn_1424b9270 as *const ()), // sub_1424b9270
    Replacement::draft(0x1424b9400, "r_1424b9400", fn_1424b9400 as *const ()), // sub_1424b9400
    Replacement::draft(0x1424b9a70, "r_1424b9a70", fn_1424b9a70 as *const ()), // sub_1424b9a70
    Replacement::draft(0x1424b9a90, "r_1424b9a90", fn_1424b9a90 as *const ()), // sub_1424b9a90
    Replacement::draft(0x1424b9b70, "r_1424b9b70", tpl_15228::<0x1432c9630, 0x10> as *const ()), // sub_1424b9b70
    Replacement::draft(0x1424b9b90, "r_1424b9b90", fn_1424b9b90 as *const ()), // sub_1424b9b90
    Replacement::draft(0x1424b9c60, "r_1424b9c60", fn_1424b9c60 as *const ()), // sub_1424b9c60
    Replacement::draft(0x1424b9c90, "r_1424b9c90", fn_1424b9c90 as *const ()), // sub_1424b9c90
    Replacement::draft(0x1424b9d40, "r_1424b9d40", fn_1424b9d40 as *const ()), // sub_1424b9d40
    Replacement::draft(0x1424b9db0, "r_1424b9db0", fn_1424b9db0 as *const ()), // sub_1424b9db0
    Replacement::draft(0x1424b9ea0, "r_1424b9ea0", fn_1424b9ea0 as *const ()), // r::BackgroundJobManager::r::BackgroundJobManager::vfunc_1
    Replacement::draft(0x1424f5230, "r_1424f5230", fn_1424f5230 as *const ()), // r::JobCallable<ui::System::OnResourceWorkAvailable::<lambda_1>>::r::JobCallable<ui::System::OnResourceWorkAvailable::<la
    Replacement::draft(0x1424f5290, "r_1424f5290", fn_1424f5290 as *const ()), // r::JobCallable<ui::System::OnLayoutWorkAvailable::<lambda_1>>::r::JobCallable<ui::System::OnLayoutWorkAvailable::<lambda
];
