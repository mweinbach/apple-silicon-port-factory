#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x14081a2c0, "content_14081a2c0", fn_14081a2c0 as *const ()), // sub_14081a2c0
    Replacement::draft_throws(0x14081a350, "content_14081a350", fn_14081a350 as *const (), Throws::Caught), // sub_14081a350
    Replacement::draft_throws(0x14081a480, "content_14081a480", fn_14081a480 as *const (), Throws::Caught), // sub_14081a480
    Replacement::draft_throws(0x14081a5b0, "content_14081a5b0", fn_14081a5b0 as *const (), Throws::Caught), // sub_14081a5b0
    Replacement::draft(0x14081a6e0, "content_14081a6e0", fn_14081a6e0 as *const ()), // sub_14081a6e0
    Replacement::draft(0x14081a780, "content_14081a780", fn_14081a780 as *const ()), // sub_14081a780
    Replacement::draft(0x14081a820, "content_14081a820", tpl_9149::<0x14387d3c0, 0x14243b0b0, 0x14081dec0, 0x338, 0x60, 0xe0, 0x64, 0x68, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081a820
    Replacement::draft_throws(0x14081a8d0, "content_14081a8d0", fn_14081a8d0 as *const (), Throws::Discard), // sub_14081a8d0
    Replacement::draft_throws(0x14081a980, "content_14081a980", fn_14081a980 as *const (), Throws::Discard), // sub_14081a980
    Replacement::draft(0x14081aab0, "content_14081aab0", tpl_9075::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x8, 0x143267b40, 0xb> as *const ()), // sub_14081aab0
    Replacement::draft(0x14081ab80, "content_14081ab80", tpl_9076::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143267b40, 0xfffffffffffffff4, 0x14, 0x34, 0xfffffffffffffffc, 0x4> as *const ()), // sub_14081ab80
    Replacement::draft_throws(0x14081adb0, "content_14081adb0", fn_14081adb0 as *const (), Throws::Caught), // sub_14081adb0
    Replacement::draft_throws(0x14081ae50, "content_14081ae50", fn_14081ae50 as *const (), Throws::Caught), // sub_14081ae50
    Replacement::draft_throws(0x14081aef0, "content_14081aef0", fn_14081aef0 as *const (), Throws::Caught), // sub_14081aef0
    Replacement::draft_throws(0x14081b070, "content_14081b070", fn_14081b070 as *const (), Throws::Discard), // sub_14081b070
    Replacement::draft(0x14081b110, "content_14081b110", fn_14081b110 as *const ()), // sub_14081b110
    Replacement::draft(0x14081b210, "content_14081b210", fn_14081b210 as *const ()), // sub_14081b210
    Replacement::draft(0x14081b370, "content_14081b370", tpl_9161::<0x14387d460, 0x14243b0b0, 0x14081bf10, 0x60, 0x1408375a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081b370
    Replacement::draft(0x14081b3f0, "content_14081b3f0", fn_14081b3f0 as *const ()), // sub_14081b3f0
    Replacement::draft_throws(0x14081b490, "content_14081b490", fn_14081b490 as *const (), Throws::Discard), // sub_14081b490
    Replacement::draft(0x14081b530, "content_14081b530", tpl_9164::<0x14387d460, 0x14243b0b0, 0x14081c750, 0x60, 0x140837b40, 0x338, 0x90, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081b530
    Replacement::draft_throws(0x14081b5c0, "content_14081b5c0", fn_14081b5c0 as *const (), Throws::Discard), // sub_14081b5c0
    Replacement::draft(0x14081b670, "content_14081b670", fn_14081b670 as *const ()), // sub_14081b670
    Replacement::draft_throws(0x14081b770, "content_14081b770", fn_14081b770 as *const (), Throws::Caught), // sub_14081b770
    Replacement::draft_throws(0x14081b860, "content_14081b860", fn_14081b860 as *const (), Throws::Caught), // sub_14081b860
    Replacement::draft_throws(0x14081b950, "content_14081b950", fn_14081b950 as *const (), Throws::Caught), // sub_14081b950
    Replacement::draft(0x14081ba40, "content_14081ba40", fn_14081ba40 as *const ()), // sub_14081ba40
    Replacement::draft(0x14081bae0, "content_14081bae0", fn_14081bae0 as *const ()), // sub_14081bae0
    Replacement::draft(0x14081bb60, "content_14081bb60", tpl_9164::<0x14387d460, 0x14243b0b0, 0x14081dec0, 0x60, 0x140837fa0, 0x338, 0x90, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081bb60
    Replacement::draft_throws(0x14081bbf0, "content_14081bbf0", fn_14081bbf0 as *const (), Throws::Discard), // sub_14081bbf0
    Replacement::draft_throws(0x14081bc80, "content_14081bc80", fn_14081bc80 as *const (), Throws::Discard), // sub_14081bc80
    Replacement::draft(0x14081bd60, "content_14081bd60", fn_14081bd60 as *const ()), // sub_14081bd60
    Replacement::draft(0x14081be30, "content_14081be30", fn_14081be30 as *const ()), // sub_14081be30
    Replacement::draft(0x14081bf10, "content_14081bf10", fn_14081bf10 as *const ()), // sub_14081bf10
    Replacement::draft(0x14081c0d0, "content_14081c0d0", fn_14081c0d0 as *const ()), // sub_14081c0d0
    Replacement::draft_throws(0x14081c440, "content_14081c440", fn_14081c440 as *const (), Throws::Discard), // sub_14081c440
    Replacement::draft(0x14081c750, "content_14081c750", fn_14081c750 as *const ()), // sub_14081c750
    Replacement::draft_throws(0x14081c990, "content_14081c990", fn_14081c990 as *const (), Throws::Discard), // sub_14081c990
    Replacement::draft(0x14081ccd0, "content_14081ccd0", fn_14081ccd0 as *const ()), // sub_14081ccd0
    Replacement::draft_throws(0x14081cd70, "content_14081cd70", fn_14081cd70 as *const (), Throws::Caught), // sub_14081cd70
    Replacement::draft_throws(0x14081d1f0, "content_14081d1f0", fn_14081d1f0 as *const (), Throws::Caught), // sub_14081d1f0
    Replacement::draft_throws(0x14081d5f0, "content_14081d5f0", fn_14081d5f0 as *const (), Throws::Caught), // sub_14081d5f0
    Replacement::draft(0x14081d9f0, "content_14081d9f0", fn_14081d9f0 as *const ()), // sub_14081d9f0
    Replacement::draft(0x14081dc70, "content_14081dc70", fn_14081dc70 as *const ()), // sub_14081dc70
    Replacement::draft(0x14081dec0, "content_14081dec0", fn_14081dec0 as *const ()), // sub_14081dec0
    Replacement::draft_throws(0x14081e120, "content_14081e120", fn_14081e120 as *const (), Throws::Discard), // sub_14081e120
    Replacement::draft_throws(0x14081e380, "content_14081e380", fn_14081e380 as *const (), Throws::Discard), // sub_14081e380
    Replacement::draft(0x14081e540, "content_14081e540", fn_14081e540 as *const ()), // sub_14081e540
    Replacement::draft(0x14081e610, "content_14081e610", fn_14081e610 as *const ()), // sub_14081e610
    Replacement::draft(0x14081e700, "content_14081e700", fn_14081e700 as *const ()), // sub_14081e700
    Replacement::draft_throws(0x14081e7f0, "content_14081e7f0", fn_14081e7f0 as *const (), Throws::Discard), // sub_14081e7f0
    Replacement::draft(0x14081e8f0, "content_14081e8f0", tpl_9190::<0x14387d3c8, 0x14243b0b0, 0x338, 0x8, 0x20, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081e8f0
    Replacement::draft_throws(0x14081e970, "content_14081e970", fn_14081e970 as *const (), Throws::Discard), // sub_14081e970
    Replacement::draft_throws(0x14081ea70, "content_14081ea70", fn_14081ea70 as *const (), Throws::Caught), // sub_14081ea70
    Replacement::draft_throws(0x14081ece0, "content_14081ece0", fn_14081ece0 as *const (), Throws::Caught), // sub_14081ece0
    Replacement::draft_throws(0x14081ef20, "content_14081ef20", fn_14081ef20 as *const (), Throws::Caught), // sub_14081ef20
    Replacement::draft(0x14081f150, "content_14081f150", fn_14081f150 as *const ()), // sub_14081f150
    Replacement::draft(0x14081f220, "content_14081f220", fn_14081f220 as *const ()), // sub_14081f220
    Replacement::draft(0x14081f290, "content_14081f290", tpl_9190::<0x14387d3c8, 0x14243b0b0, 0x338, 0x8, 0xc0, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081f290
    Replacement::draft_throws(0x14081f310, "content_14081f310", fn_14081f310 as *const (), Throws::Discard), // sub_14081f310
    Replacement::draft_throws(0x14081f380, "content_14081f380", fn_14081f380 as *const (), Throws::Discard), // sub_14081f380
    Replacement::draft(0x14081f460, "content_14081f460", fn_14081f460 as *const ()), // sub_14081f460
    Replacement::draft(0x14081f530, "content_14081f530", fn_14081f530 as *const ()), // sub_14081f530
    Replacement::draft(0x14081f640, "content_14081f640", tpl_636::<0x14387d450, 0x14243b0b0, 0x14387d3c8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14081f640
    Replacement::draft(0x14081f6d0, "content_14081f6d0", fn_14081f6d0 as *const ()), // sub_14081f6d0
    Replacement::draft_throws(0x14081f770, "content_14081f770", tpl_9063::<0x14387d45c, 0x14387d450, 0x1401e9fa0, 0x14081e7f0, 0x328, 0x338, 0x80, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14081f770
    Replacement::draft(0x14081f800, "content_14081f800", tpl_9064::<0x14387d450, 0x14243b0b0, 0x14081e8f0, 0x338, 0x80, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081f800
    Replacement::draft_throws(0x14081f880, "content_14081f880", fn_14081f880 as *const (), Throws::Discard), // sub_14081f880
    Replacement::draft_throws(0x14081f920, "content_14081f920", fn_14081f920 as *const (), Throws::Caught), // sub_14081f920
    Replacement::draft_throws(0x14081fa00, "content_14081fa00", fn_14081fa00 as *const (), Throws::Caught), // sub_14081fa00
    Replacement::draft_throws(0x14081fae0, "content_14081fae0", fn_14081fae0 as *const (), Throws::Caught), // sub_14081fae0
    Replacement::draft(0x14081fbc0, "content_14081fbc0", tpl_9070::<0x14387d450, 0x14243b0b0, 0x14081f150, 0x338, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081fbc0
    Replacement::draft(0x14081fc40, "content_14081fc40", tpl_9203::<0x14387d45c, 0x14387d450, 0x1401e9970, 0x14081f220, 0x338, 0x80, 0xd8, 0x1401e9860> as *const ()), // sub_14081fc40
    Replacement::draft(0x14081fcb0, "content_14081fcb0", tpl_9064::<0x14387d450, 0x14243b0b0, 0x14081f290, 0x338, 0x80, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14081fcb0
    Replacement::draft_throws(0x14081fd30, "content_14081fd30", tpl_9204::<0x14387d45c, 0x14387d450, 0x1401e9770, 0x14081f310, 0x338, 0x80, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_14081fd30
    Replacement::draft_throws(0x14081fdb0, "content_14081fdb0", fn_14081fdb0 as *const (), Throws::Discard), // sub_14081fdb0
    Replacement::draft(0x14081fe90, "content_14081fe90", fn_14081fe90 as *const ()), // sub_14081fe90
    Replacement::draft(0x14081ff90, "content_14081ff90", fn_14081ff90 as *const ()), // sub_14081ff90
    Replacement::draft(0x1408200e0, "content_1408200e0", tpl_636::<0x14387d448, 0x14243b0b0, 0x14387d3c8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1408200e0
    Replacement::draft(0x140820170, "content_140820170", fn_140820170 as *const ()), // sub_140820170
    Replacement::draft_throws(0x140820300, "content_140820300", fn_140820300 as *const (), Throws::Discard), // sub_140820300
    Replacement::draft(0x140820460, "content_140820460", tpl_9209::<0x14387d448, 0x14243b0b0, 0x14081e8f0, 0x338, 0x80, 0x20, 0x188, 0x48, 0x18c, 0x190, 0x194, 0x8> as *const ()), // sub_140820460
    Replacement::draft_throws(0x140820530, "content_140820530", fn_140820530 as *const (), Throws::Discard), // sub_140820530
    Replacement::draft(0x1408206c0, "content_1408206c0", fn_1408206c0 as *const ()), // sub_1408206c0
    Replacement::draft_throws(0x140820780, "content_140820780", fn_140820780 as *const (), Throws::Caught), // sub_140820780
    Replacement::draft_throws(0x140820900, "content_140820900", fn_140820900 as *const (), Throws::Caught), // sub_140820900
    Replacement::draft_throws(0x140820a80, "content_140820a80", fn_140820a80 as *const (), Throws::Caught), // sub_140820a80
    Replacement::draft(0x140820c00, "content_140820c00", fn_140820c00 as *const ()), // sub_140820c00
    Replacement::draft(0x140820cf0, "content_140820cf0", fn_140820cf0 as *const ()), // sub_140820cf0
    Replacement::draft(0x140820dc0, "content_140820dc0", tpl_9209::<0x14387d448, 0x14243b0b0, 0x14081f290, 0x338, 0x80, 0xc0, 0x188, 0xe8, 0x18c, 0x190, 0x194, 0x8> as *const ()), // sub_140820dc0
    Replacement::draft_throws(0x140820ea0, "content_140820ea0", fn_140820ea0 as *const (), Throws::Discard), // sub_140820ea0
    Replacement::draft_throws(0x140820f80, "content_140820f80", fn_140820f80 as *const (), Throws::Discard), // sub_140820f80
    Replacement::draft(0x140821100, "content_140821100", fn_140821100 as *const ()), // sub_140821100
    Replacement::draft(0x140821220, "content_140821220", fn_140821220 as *const ()), // sub_140821220
    Replacement::draft(0x1408213b0, "content_1408213b0", tpl_9096::<0x14387d3d8, 0x14243b0b0, 0x14081bf10, 0xa0, 0x1408375a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408213b0
    Replacement::draft(0x140821430, "content_140821430", fn_140821430 as *const ()), // sub_140821430
    Replacement::draft_throws(0x1408215e0, "content_1408215e0", fn_1408215e0 as *const (), Throws::Discard), // sub_1408215e0
    Replacement::draft(0x140821850, "content_140821850", fn_140821850 as *const ()), // sub_140821850
    Replacement::draft_throws(0x1408219f0, "content_1408219f0", fn_1408219f0 as *const (), Throws::Discard), // sub_1408219f0
    Replacement::draft(0x140821c10, "content_140821c10", fn_140821c10 as *const ()), // sub_140821c10
    Replacement::draft_throws(0x140821da0, "content_140821da0", fn_140821da0 as *const (), Throws::Caught), // sub_140821da0
    Replacement::draft_throws(0x140822210, "content_140822210", fn_140822210 as *const (), Throws::Caught), // sub_140822210
    Replacement::draft_throws(0x1408227f0, "content_1408227f0", fn_1408227f0 as *const (), Throws::Caught), // sub_1408227f0
    Replacement::draft(0x140822d80, "content_140822d80", fn_140822d80 as *const ()), // sub_140822d80
    Replacement::draft(0x140822eb0, "content_140822eb0", fn_140822eb0 as *const ()), // sub_140822eb0
    Replacement::draft(0x140823070, "content_140823070", fn_140823070 as *const ()), // sub_140823070
    Replacement::draft_throws(0x140823240, "content_140823240", fn_140823240 as *const (), Throws::Discard), // sub_140823240
    Replacement::draft_throws(0x140823390, "content_140823390", fn_140823390 as *const (), Throws::Discard), // sub_140823390
    Replacement::draft_throws(0x1408235e0, "content_1408235e0", tpl_2253::<0x143264550, 0x64, 0x1, 0x1402c4e80, 0x80, 0x143264580> as *const (), Throws::Discard), // sub_1408235e0
    Replacement::draft(0x140823650, "content_140823650", fn_140823650 as *const ()), // sub_140823650
    Replacement::draft(0x140823730, "content_140823730", fn_140823730 as *const ()), // sub_140823730
    Replacement::draft(0x140823890, "content_140823890", fn_140823890 as *const ()), // sub_140823890
    Replacement::draft(0x140823900, "content_140823900", fn_140823900 as *const ()), // sub_140823900
    Replacement::draft_throws(0x140823a50, "content_140823a50", fn_140823a50 as *const (), Throws::Discard), // sub_140823a50
    Replacement::draft(0x140823ba0, "content_140823ba0", fn_140823ba0 as *const ()), // sub_140823ba0
    Replacement::draft_throws(0x140823c70, "content_140823c70", fn_140823c70 as *const (), Throws::Discard), // sub_140823c70
    Replacement::draft(0x140823de0, "content_140823de0", fn_140823de0 as *const ()), // sub_140823de0
    Replacement::draft_throws(0x140823ec0, "content_140823ec0", fn_140823ec0 as *const (), Throws::Caught), // sub_140823ec0
    Replacement::draft_throws(0x140824200, "content_140824200", fn_140824200 as *const (), Throws::Caught), // sub_140824200
    Replacement::draft_throws(0x140824510, "content_140824510", fn_140824510 as *const (), Throws::Caught), // sub_140824510
    Replacement::draft(0x140824810, "content_140824810", fn_140824810 as *const ()), // sub_140824810
    Replacement::draft(0x140824910, "content_140824910", fn_140824910 as *const ()), // sub_140824910
    Replacement::draft(0x1408249d0, "content_1408249d0", fn_1408249d0 as *const ()), // sub_1408249d0
    Replacement::draft_throws(0x140824aa0, "content_140824aa0", fn_140824aa0 as *const (), Throws::Discard), // sub_140824aa0
    Replacement::draft_throws(0x140824b70, "content_140824b70", fn_140824b70 as *const (), Throws::Caught), // sub_140824b70
    Replacement::draft(0x140824d90, "content_140824d90", fn_140824d90 as *const ()), // sub_140824d90
    Replacement::draft(0x140824e30, "content_140824e30", fn_140824e30 as *const ()), // sub_140824e30
    Replacement::draft(0x140824ee0, "content_140824ee0", fn_140824ee0 as *const ()), // sub_140824ee0
    Replacement::draft(0x140824f80, "content_140824f80", fn_140824f80 as *const ()), // sub_140824f80
    Replacement::draft_throws(0x140825120, "content_140825120", fn_140825120 as *const (), Throws::Caught), // sub_140825120
    Replacement::draft(0x1408252a0, "content_1408252a0", fn_1408252a0 as *const ()), // sub_1408252a0
    Replacement::draft_throws(0x1408253a0, "content_1408253a0", fn_1408253a0 as *const (), Throws::Caught), // sub_1408253a0
    Replacement::draft(0x140825530, "content_140825530", fn_140825530 as *const ()), // sub_140825530
    Replacement::draft_throws(0x1408255e0, "content_1408255e0", fn_1408255e0 as *const (), Throws::Caught), // sub_1408255e0
    Replacement::draft_throws(0x140825710, "content_140825710", fn_140825710 as *const (), Throws::Caught), // sub_140825710
    Replacement::draft_throws(0x140825990, "content_140825990", fn_140825990 as *const (), Throws::Caught), // sub_140825990
    Replacement::draft(0x140825c00, "content_140825c00", fn_140825c00 as *const ()), // sub_140825c00
    Replacement::draft(0x140825d20, "content_140825d20", fn_140825d20 as *const ()), // sub_140825d20
    Replacement::draft(0x140825e10, "content_140825e10", fn_140825e10 as *const ()), // sub_140825e10
    Replacement::draft_throws(0x140825f10, "content_140825f10", fn_140825f10 as *const (), Throws::Caught), // sub_140825f10
    Replacement::draft_throws(0x140826000, "content_140826000", fn_140826000 as *const (), Throws::Caught), // sub_140826000
    Replacement::draft_throws(0x1408261b0, "content_1408261b0", fn_1408261b0 as *const (), Throws::Discard), // sub_1408261b0
    Replacement::draft(0x140826220, "content_140826220", fn_140826220 as *const ()), // content::CameraTimelineEvent::content::CameraTimelineEvent::vfunc_0
    Replacement::draft(0x140826280, "content_140826280", fn_140826280 as *const ()), // content::CinematicLightTimelineEvent::content::CinematicLightTimelineEvent::vfunc_0
    Replacement::draft(0x140826310, "content_140826310", fn_140826310 as *const ()), // sub_140826310
    Replacement::draft(0x1408263b0, "content_1408263b0", fn_1408263b0 as *const ()), // sub_1408263b0
    Replacement::draft(0x1408264e0, "content_1408264e0", fn_1408264e0 as *const ()), // content::HideTimelineEvent::content::HideTimelineEvent::vfunc_0
    Replacement::draft(0x140826530, "content_140826530", fn_140826530 as *const ()), // content::GfxGraphVariableTimelineEvent::content::GfxGraphVariableTimelineEvent::vfunc_0
    Replacement::draft(0x1408265b0, "content_1408265b0", fn_1408265b0 as *const ()), // sub_1408265b0
    Replacement::draft(0x1408266b0, "content_1408266b0", fn_1408266b0 as *const ()), // sub_1408266b0
    Replacement::draft(0x140826790, "content_140826790", fn_140826790 as *const ()), // content::ParameterCurveTimelineEvent::content::ParameterCurveTimelineEvent::vfunc_0
    Replacement::draft(0x140826830, "content_140826830", fn_140826830 as *const ()), // content::SoundTimelineEvent::content::SoundTimelineEvent::vfunc_0
    Replacement::draft(0x140826880, "content_140826880", fn_140826880 as *const ()), // content::ScriptTimelineEvent::content::ScriptTimelineEvent::vfunc_0
    Replacement::draft(0x1408268e0, "content_1408268e0", fn_1408268e0 as *const ()), // content::TimelineTimelineEvent::content::TimelineTimelineEvent::vfunc_0
    Replacement::draft(0x140826960, "content_140826960", fn_140826960 as *const ()), // content::TimelineMessageStartRecording::content::TimelineMessageStartRecording::vfunc_0
    Replacement::draft(0x1408269b0, "content_1408269b0", fn_1408269b0 as *const ()), // content::TimelineMessageBase::content::TimelineMessageBase::vfunc_0
    Replacement::draft(0x140826a00, "content_140826a00", fn_140826a00 as *const ()), // content::TransformTimelineEvent::content::TransformTimelineEvent::vfunc_0
    Replacement::draft(0x140826a80, "content_140826a80", fn_140826a80 as *const ()), // content::VideoTimelineEvent::content::VideoTimelineEvent::vfunc_0
    Replacement::draft(0x140826ae0, "content_140826ae0", fn_140826ae0 as *const ()), // content::TimelineMessageJumpToFrame::content::TimelineMessageJumpToFrame::vfunc_0
    Replacement::draft_throws(0x140826b30, "unknown_140826b30", fn_140826b30 as *const (), Throws::Caught), // sub_140826b30
    Replacement::draft_throws(0x140826d10, "unknown_140826d10", fn_140826d10 as *const (), Throws::Caught), // sub_140826d10
    Replacement::draft_throws(0x140826e00, "unknown_140826e00", fn_140826e00 as *const (), Throws::Caught), // sub_140826e00
    Replacement::draft_throws(0x140826f90, "unknown_140826f90", fn_140826f90 as *const (), Throws::Caught), // sub_140826f90
    Replacement::draft_throws(0x140827080, "unknown_140827080", fn_140827080 as *const (), Throws::Caught), // sub_140827080
    Replacement::draft(0x1408271e0, "unknown_1408271e0", fn_1408271e0 as *const ()), // sub_1408271e0
    Replacement::draft(0x140827260, "unknown_140827260", fn_140827260 as *const ()), // sub_140827260
    Replacement::draft(0x1408273d0, "unknown_1408273d0", fn_1408273d0 as *const ()), // sub_1408273d0
    Replacement::draft_throws(0x1408274d0, "unknown_1408274d0", fn_1408274d0 as *const (), Throws::Caught), // sub_1408274d0
    Replacement::draft_throws(0x1408277d0, "unknown_1408277d0", fn_1408277d0 as *const (), Throws::Caught), // sub_1408277d0
    Replacement::draft_throws(0x140827ad0, "unknown_140827ad0", fn_140827ad0 as *const (), Throws::Caught), // sub_140827ad0
    Replacement::draft(0x140827de0, "unknown_140827de0", fn_140827de0 as *const ()), // sub_140827de0
    Replacement::draft_throws(0x140827e90, "unknown_140827e90", fn_140827e90 as *const (), Throws::Caught), // sub_140827e90
    Replacement::draft_throws(0x140828110, "unknown_140828110", fn_140828110 as *const (), Throws::Caught), // sub_140828110
    Replacement::draft_throws(0x140828390, "unknown_140828390", fn_140828390 as *const (), Throws::Caught), // sub_140828390
    Replacement::draft_throws(0x140828620, "unknown_140828620", fn_140828620 as *const (), Throws::Caught), // sub_140828620
    Replacement::draft_throws(0x1408288b0, "unknown_1408288b0", fn_1408288b0 as *const (), Throws::Caught), // sub_1408288b0
    Replacement::draft_throws(0x140828b60, "unknown_140828b60", fn_140828b60 as *const (), Throws::Caught), // sub_140828b60
    Replacement::draft_throws(0x140828ef0, "unknown_140828ef0", fn_140828ef0 as *const (), Throws::Caught), // sub_140828ef0
    Replacement::draft_throws(0x140829280, "unknown_140829280", fn_140829280 as *const (), Throws::Caught), // sub_140829280
    Replacement::draft_throws(0x140829610, "unknown_140829610", fn_140829610 as *const (), Throws::Caught), // sub_140829610
    Replacement::draft_throws(0x1408299a0, "unknown_1408299a0", fn_1408299a0 as *const (), Throws::Caught), // sub_1408299a0
    Replacement::draft_throws(0x140829d30, "unknown_140829d30", fn_140829d30 as *const (), Throws::Caught), // sub_140829d30
    Replacement::draft_throws(0x140829e10, "unknown_140829e10", fn_140829e10 as *const (), Throws::Caught), // sub_140829e10
    Replacement::draft_throws(0x140829f40, "unknown_140829f40", fn_140829f40 as *const (), Throws::Caught), // sub_140829f40
    Replacement::draft_throws(0x14082a070, "unknown_14082a070", fn_14082a070 as *const (), Throws::Caught), // sub_14082a070
    Replacement::draft(0x14082a160, "unknown_14082a160", fn_14082a160 as *const ()), // sub_14082a160
    Replacement::draft_throws(0x14082a250, "unknown_14082a250", fn_14082a250 as *const (), Throws::Caught), // sub_14082a250
    Replacement::draft_throws(0x14082a550, "unknown_14082a550", fn_14082a550 as *const (), Throws::Caught), // sub_14082a550
    Replacement::draft_throws(0x14082a850, "unknown_14082a850", fn_14082a850 as *const (), Throws::Caught), // sub_14082a850
    Replacement::draft_throws(0x14082ab50, "unknown_14082ab50", fn_14082ab50 as *const (), Throws::Caught), // sub_14082ab50
    Replacement::draft_throws(0x14082ad20, "unknown_14082ad20", fn_14082ad20 as *const (), Throws::Caught), // sub_14082ad20
    Replacement::draft_throws(0x14082aec0, "unknown_14082aec0", fn_14082aec0 as *const (), Throws::Caught), // sub_14082aec0
    Replacement::draft_throws(0x14082b050, "unknown_14082b050", fn_14082b050 as *const (), Throws::Caught), // sub_14082b050
    Replacement::draft_throws(0x14082b180, "unknown_14082b180", fn_14082b180 as *const (), Throws::Caught), // sub_14082b180
    Replacement::draft_throws(0x14082b2c0, "unknown_14082b2c0", fn_14082b2c0 as *const (), Throws::Caught), // sub_14082b2c0
    Replacement::draft_throws(0x14082baa0, "unknown_14082baa0", fn_14082baa0 as *const (), Throws::Caught), // sub_14082baa0
    Replacement::draft_throws(0x14082c180, "unknown_14082c180", fn_14082c180 as *const (), Throws::Caught), // sub_14082c180
    Replacement::draft_throws(0x14082c7d0, "unknown_14082c7d0", fn_14082c7d0 as *const (), Throws::Caught), // sub_14082c7d0
    Replacement::draft_throws(0x14082caf0, "unknown_14082caf0", tpl_66331::<0x142442de0, 0x143a3a678, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_14082caf0
    Replacement::draft_throws(0x14082cd50, "unknown_14082cd50", fn_14082cd50 as *const (), Throws::Caught), // sub_14082cd50
    Replacement::draft_throws(0x14082d070, "unknown_14082d070", fn_14082d070 as *const (), Throws::Caught), // sub_14082d070
    Replacement::draft_throws(0x14082d360, "unknown_14082d360", fn_14082d360 as *const (), Throws::Caught), // sub_14082d360
    Replacement::draft_throws(0x14082d620, "unknown_14082d620", fn_14082d620 as *const (), Throws::Caught), // sub_14082d620
    Replacement::draft_throws(0x14082d8e0, "unknown_14082d8e0", fn_14082d8e0 as *const (), Throws::Caught), // sub_14082d8e0
    Replacement::draft_throws(0x14082d9b0, "unknown_14082d9b0", fn_14082d9b0 as *const (), Throws::Caught), // sub_14082d9b0
    Replacement::draft_throws(0x14082df30, "unknown_14082df30", fn_14082df30 as *const (), Throws::Caught), // sub_14082df30
    Replacement::draft_throws(0x14082e480, "unknown_14082e480", fn_14082e480 as *const (), Throws::Caught), // sub_14082e480
    Replacement::draft_throws(0x14082e8c0, "unknown_14082e8c0", fn_14082e8c0 as *const (), Throws::Caught), // sub_14082e8c0
    Replacement::draft_throws(0x14082efa0, "unknown_14082efa0", fn_14082efa0 as *const (), Throws::Caught), // sub_14082efa0
    Replacement::draft_throws(0x14082f650, "unknown_14082f650", fn_14082f650 as *const (), Throws::Caught), // sub_14082f650
    Replacement::draft_throws(0x14082fa90, "unknown_14082fa90", fn_14082fa90 as *const (), Throws::Caught), // sub_14082fa90
    Replacement::draft_throws(0x1408303a0, "unknown_1408303a0", fn_1408303a0 as *const (), Throws::Caught), // sub_1408303a0
    Replacement::draft_throws(0x140830c80, "unknown_140830c80", fn_140830c80 as *const (), Throws::Caught), // sub_140830c80
    Replacement::draft_throws(0x140831170, "unknown_140831170", fn_140831170 as *const (), Throws::Caught), // sub_140831170
    Replacement::draft_throws(0x1408316f0, "unknown_1408316f0", fn_1408316f0 as *const (), Throws::Caught), // sub_1408316f0
    Replacement::draft_throws(0x140831c40, "unknown_140831c40", fn_140831c40 as *const (), Throws::Caught), // sub_140831c40
    Replacement::draft_throws(0x140832080, "unknown_140832080", fn_140832080 as *const (), Throws::Caught), // sub_140832080
    Replacement::draft_throws(0x140832630, "unknown_140832630", fn_140832630 as *const (), Throws::Caught), // sub_140832630
    Replacement::draft_throws(0x140832ba0, "unknown_140832ba0", fn_140832ba0 as *const (), Throws::Caught), // sub_140832ba0
    Replacement::draft(0x140832fe0, "unknown_140832fe0", fn_140832fe0 as *const ()), // sub_140832fe0
    Replacement::draft_throws(0x140833280, "unknown_140833280", fn_140833280 as *const (), Throws::Caught), // sub_140833280
    Replacement::draft_throws(0x140833830, "unknown_140833830", fn_140833830 as *const (), Throws::Caught), // sub_140833830
    Replacement::draft_throws(0x140833da0, "unknown_140833da0", fn_140833da0 as *const (), Throws::Caught), // sub_140833da0
    Replacement::draft(0x1408341f0, "unknown_1408341f0", tpl_1149::<0x8, 0x14083fed0, 0x1401e66f0> as *const ()), // sub_1408341f0
    Replacement::draft_throws(0x140834220, "unknown_140834220", tpl_7370::<0x1401ffe00, 0x140708f60, 0x14083ff60, 0x142442de0, 0x143a3a678, 0x143243118, 0x1424b0060> as *const (), Throws::Discard), // sub_140834220
    Replacement::draft(0x1408344c0, "unknown_1408344c0", fn_1408344c0 as *const ()), // sub_1408344c0
    Replacement::draft_throws(0x1408345a0, "unknown_1408345a0", fn_1408345a0 as *const (), Throws::Caught), // sub_1408345a0
    Replacement::draft_throws(0x140834a70, "unknown_140834a70", fn_140834a70 as *const (), Throws::Caught), // sub_140834a70
    Replacement::draft_throws(0x140834ef0, "unknown_140834ef0", fn_140834ef0 as *const (), Throws::Caught), // sub_140834ef0
    Replacement::draft_throws(0x140835370, "unknown_140835370", fn_140835370 as *const (), Throws::Caught), // sub_140835370
    Replacement::draft(0x140835570, "unknown_140835570", fn_140835570 as *const ()), // sub_140835570
    Replacement::draft_throws(0x140835730, "unknown_140835730", fn_140835730 as *const (), Throws::Caught), // sub_140835730
    Replacement::draft(0x1408358e0, "unknown_1408358e0", fn_1408358e0 as *const ()), // sub_1408358e0
    Replacement::draft(0x140835a70, "unknown_140835a70", fn_140835a70 as *const ()), // sub_140835a70
    Replacement::draft_throws(0x140835bb0, "unknown_140835bb0", fn_140835bb0 as *const (), Throws::Caught), // sub_140835bb0
    Replacement::draft(0x140835d60, "unknown_140835d60", fn_140835d60 as *const ()), // sub_140835d60
    Replacement::draft(0x140835ed0, "unknown_140835ed0", fn_140835ed0 as *const ()), // sub_140835ed0
    Replacement::draft_throws(0x140836020, "unknown_140836020", fn_140836020 as *const (), Throws::Caught), // sub_140836020
    Replacement::draft(0x140836150, "unknown_140836150", fn_140836150 as *const ()), // sub_140836150
    Replacement::draft(0x1408361f0, "unknown_1408361f0", fn_1408361f0 as *const ()), // sub_1408361f0
    Replacement::draft(0x140836260, "unknown_140836260", fn_140836260 as *const ()), // sub_140836260
    Replacement::draft_throws(0x140836470, "unknown_140836470", fn_140836470 as *const (), Throws::Caught), // sub_140836470
    Replacement::draft(0x1408367f0, "unknown_1408367f0", fn_1408367f0 as *const ()), // sub_1408367f0
    Replacement::draft(0x1408368a0, "unknown_1408368a0", fn_1408368a0 as *const ()), // sub_1408368a0
    Replacement::draft_throws(0x140836a40, "unknown_140836a40", fn_140836a40 as *const (), Throws::Caught), // sub_140836a40
    Replacement::draft(0x140836e30, "unknown_140836e30", fn_140836e30 as *const ()), // sub_140836e30
    Replacement::draft(0x140836f40, "unknown_140836f40", fn_140836f40 as *const ()), // sub_140836f40
    Replacement::draft(0x1408370d0, "unknown_1408370d0", fn_1408370d0 as *const ()), // sub_1408370d0
    Replacement::draft_throws(0x140837270, "unknown_140837270", fn_140837270 as *const (), Throws::Discard), // sub_140837270
    Replacement::draft(0x140837400, "unknown_140837400", tpl_9332::<0x143264338, 0x1, 0x14083e5b0, 0x143873d60, 0xbc, 0x143264260, 0x1432642a8, 0x143264290, 0x1401de410, 0x143873cf8, 0xc0, 0x1432642b8, 0x1432642e8, 0x1432642d0> as *const ()), // sub_140837400
    Replacement::draft(0x1408374d0, "unknown_1408374d0", tpl_9332::<0x143264338, 0x1, 0x14083e5d0, 0x143873c90, 0xbc, 0x143264260, 0x1432642a8, 0x143264290, 0x1401de410, 0x143873cb0, 0xc0, 0x1432642b8, 0x1432642e8, 0x1432642d0> as *const ()), // sub_1408374d0
    Replacement::draft(0x1408375a0, "unknown_1408375a0", fn_1408375a0 as *const ()), // sub_1408375a0
    Replacement::draft(0x140837730, "unknown_140837730", fn_140837730 as *const ()), // sub_140837730
    Replacement::draft_throws(0x1408378d0, "unknown_1408378d0", tpl_9335::<0x14387e848, 0x14387e738, 0x1401e9fa0, 0x1408271e0, 0x14083e5f0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408378d0
    Replacement::draft(0x1408379e0, "unknown_1408379e0", fn_1408379e0 as *const ()), // sub_1408379e0
    Replacement::draft(0x140837b40, "unknown_140837b40", fn_140837b40 as *const ()), // sub_140837b40
    Replacement::draft_throws(0x140837cb0, "unknown_140837cb0", tpl_9335::<0x14387e848, 0x14387e738, 0x1401e9b60, 0x1408271e0, 0x14083e7a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140837cb0
    Replacement::draft(0x140837dc0, "unknown_140837dc0", fn_140837dc0 as *const ()), // sub_140837dc0
    Replacement::draft(0x140837e40, "unknown_140837e40", fn_140837e40 as *const ()), // sub_140837e40
    Replacement::draft(0x140837fa0, "unknown_140837fa0", fn_140837fa0 as *const ()), // sub_140837fa0
    Replacement::draft_throws(0x140838110, "unknown_140838110", fn_140838110 as *const (), Throws::Discard), // sub_140838110
    Replacement::draft(0x140838260, "unknown_140838260", fn_140838260 as *const ()), // sub_140838260
    Replacement::draft_throws(0x140838420, "unknown_140838420", fn_140838420 as *const (), Throws::Caught), // sub_140838420
    Replacement::draft(0x1408385d0, "unknown_1408385d0", fn_1408385d0 as *const ()), // sub_1408385d0
    Replacement::draft(0x140838760, "unknown_140838760", fn_140838760 as *const ()), // sub_140838760
    Replacement::draft_throws(0x1408388a0, "unknown_1408388a0", fn_1408388a0 as *const (), Throws::Caught), // sub_1408388a0
    Replacement::draft(0x140838a50, "unknown_140838a50", fn_140838a50 as *const ()), // sub_140838a50
    Replacement::draft(0x140838bc0, "unknown_140838bc0", fn_140838bc0 as *const ()), // sub_140838bc0
    Replacement::draft_throws(0x140838d10, "unknown_140838d10", fn_140838d10 as *const (), Throws::Caught), // sub_140838d10
    Replacement::draft(0x140838e40, "unknown_140838e40", fn_140838e40 as *const ()), // sub_140838e40
    Replacement::draft(0x140838ee0, "unknown_140838ee0", fn_140838ee0 as *const ()), // sub_140838ee0
    Replacement::draft_throws(0x1408390a0, "unknown_1408390a0", fn_1408390a0 as *const (), Throws::Caught), // sub_1408390a0
    Replacement::draft(0x140839250, "unknown_140839250", fn_140839250 as *const ()), // sub_140839250
    Replacement::draft(0x1408393e0, "unknown_1408393e0", fn_1408393e0 as *const ()), // sub_1408393e0
    Replacement::draft_throws(0x140839520, "unknown_140839520", fn_140839520 as *const (), Throws::Caught), // sub_140839520
    Replacement::draft(0x1408396d0, "unknown_1408396d0", fn_1408396d0 as *const ()), // sub_1408396d0
    Replacement::draft(0x140839840, "unknown_140839840", fn_140839840 as *const ()), // sub_140839840
    Replacement::draft_throws(0x140839990, "unknown_140839990", fn_140839990 as *const (), Throws::Caught), // sub_140839990
    Replacement::draft(0x140839ac0, "unknown_140839ac0", fn_140839ac0 as *const ()), // sub_140839ac0
    Replacement::draft(0x140839b60, "unknown_140839b60", fn_140839b60 as *const ()), // sub_140839b60
    Replacement::draft_throws(0x140839d20, "unknown_140839d20", fn_140839d20 as *const (), Throws::Caught), // sub_140839d20
    Replacement::draft(0x140839ed0, "unknown_140839ed0", fn_140839ed0 as *const ()), // sub_140839ed0
    Replacement::draft(0x14083a060, "unknown_14083a060", fn_14083a060 as *const ()), // sub_14083a060
    Replacement::draft_throws(0x14083a1a0, "unknown_14083a1a0", fn_14083a1a0 as *const (), Throws::Caught), // sub_14083a1a0
    Replacement::draft(0x14083a350, "unknown_14083a350", fn_14083a350 as *const ()), // sub_14083a350
    Replacement::draft(0x14083a4c0, "unknown_14083a4c0", fn_14083a4c0 as *const ()), // sub_14083a4c0
    Replacement::draft_throws(0x14083a610, "unknown_14083a610", fn_14083a610 as *const (), Throws::Caught), // sub_14083a610
    Replacement::draft(0x14083a740, "unknown_14083a740", fn_14083a740 as *const ()), // sub_14083a740
    Replacement::draft(0x14083a7e0, "unknown_14083a7e0", fn_14083a7e0 as *const ()), // sub_14083a7e0
    Replacement::draft(0x14083a860, "unknown_14083a860", fn_14083a860 as *const ()), // sub_14083a860
    Replacement::draft_throws(0x14083a8e0, "unknown_14083a8e0", fn_14083a8e0 as *const (), Throws::Caught), // sub_14083a8e0
    Replacement::draft_throws(0x14083b3a0, "unknown_14083b3a0", fn_14083b3a0 as *const (), Throws::Caught), // sub_14083b3a0
    Replacement::draft_throws(0x14083be30, "unknown_14083be30", fn_14083be30 as *const (), Throws::Caught), // sub_14083be30
    Replacement::draft_throws(0x14083c8b0, "unknown_14083c8b0", fn_14083c8b0 as *const (), Throws::Caught), // sub_14083c8b0
    Replacement::draft_throws(0x14083ca40, "unknown_14083ca40", fn_14083ca40 as *const (), Throws::Caught), // sub_14083ca40
    Replacement::draft_throws(0x14083cbd0, "unknown_14083cbd0", fn_14083cbd0 as *const (), Throws::Caught), // sub_14083cbd0
    Replacement::draft_throws(0x14083cd60, "unknown_14083cd60", fn_14083cd60 as *const (), Throws::Caught), // sub_14083cd60
    Replacement::draft_throws(0x14083cef0, "unknown_14083cef0", fn_14083cef0 as *const (), Throws::Caught), // sub_14083cef0
    Replacement::draft_throws(0x14083d5c0, "unknown_14083d5c0", fn_14083d5c0 as *const (), Throws::Caught), // sub_14083d5c0
    Replacement::draft_throws(0x14083dbd0, "unknown_14083dbd0", fn_14083dbd0 as *const (), Throws::Caught), // sub_14083dbd0
    Replacement::draft_throws(0x14083e1c0, "unknown_14083e1c0", fn_14083e1c0 as *const (), Throws::Caught), // sub_14083e1c0
    Replacement::draft(0x14083e5b0, "unknown_14083e5b0", tpl_8671::<0xc0> as *const ()), // sub_14083e5b0
    Replacement::draft(0x14083e5d0, "unknown_14083e5d0", tpl_8671::<0xb0> as *const ()), // sub_14083e5d0
    Replacement::draft_throws(0x14083e5f0, "unknown_14083e5f0", fn_14083e5f0 as *const (), Throws::Caught), // sub_14083e5f0
    Replacement::draft_throws(0x14083e7a0, "unknown_14083e7a0", fn_14083e7a0 as *const (), Throws::Caught), // sub_14083e7a0
    Replacement::draft_throws(0x14083e960, "unknown_14083e960", fn_14083e960 as *const (), Throws::Caught), // sub_14083e960
    Replacement::draft_throws(0x14083ea10, "unknown_14083ea10", fn_14083ea10 as *const (), Throws::Caught), // sub_14083ea10
    Replacement::draft_throws(0x14083eac0, "unknown_14083eac0", fn_14083eac0 as *const (), Throws::Caught), // sub_14083eac0
    Replacement::draft_throws(0x14083eb70, "unknown_14083eb70", fn_14083eb70 as *const (), Throws::Discard), // sub_14083eb70
    Replacement::draft_throws(0x14083ec10, "unknown_14083ec10", fn_14083ec10 as *const (), Throws::Caught), // sub_14083ec10
    Replacement::draft_throws(0x14083eda0, "unknown_14083eda0", fn_14083eda0 as *const (), Throws::Caught), // sub_14083eda0
    Replacement::draft_throws(0x14083ef30, "unknown_14083ef30", fn_14083ef30 as *const (), Throws::Caught), // sub_14083ef30
    Replacement::draft_throws(0x14083f0c0, "unknown_14083f0c0", fn_14083f0c0 as *const (), Throws::Caught), // sub_14083f0c0
    Replacement::draft_throws(0x14083f250, "unknown_14083f250", fn_14083f250 as *const (), Throws::Caught), // sub_14083f250
    Replacement::draft_throws(0x14083f3e0, "unknown_14083f3e0", fn_14083f3e0 as *const (), Throws::Caught), // sub_14083f3e0
    Replacement::draft_throws(0x14083f570, "unknown_14083f570", fn_14083f570 as *const (), Throws::Caught), // sub_14083f570
    Replacement::draft_throws(0x14083f700, "unknown_14083f700", fn_14083f700 as *const (), Throws::Caught), // sub_14083f700
    Replacement::draft_throws(0x14083f890, "unknown_14083f890", fn_14083f890 as *const (), Throws::Caught), // sub_14083f890
    Replacement::draft_throws(0x14083fa20, "unknown_14083fa20", fn_14083fa20 as *const (), Throws::Caught), // sub_14083fa20
    Replacement::draft_throws(0x14083fbb0, "unknown_14083fbb0", fn_14083fbb0 as *const (), Throws::Caught), // sub_14083fbb0
    Replacement::draft_throws(0x14083fd40, "unknown_14083fd40", fn_14083fd40 as *const (), Throws::Caught), // sub_14083fd40
    Replacement::draft(0x14083fed0, "unknown_14083fed0", tpl_64713::<0x19, 0x10, 0x14083fed0, 0xa4, 0x14240ab10, 0x1c, 0x1401e66f0> as *const ()), // sub_14083fed0
    Replacement::draft_throws(0x14083ff60, "unknown_14083ff60", tpl_65670::<0x142442de0, 0x143a3a678, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_14083ff60
    Replacement::draft(0x140840040, "unknown_140840040", fn_140840040 as *const ()), // sub_140840040
    Replacement::draft(0x140840130, "unknown_140840130", fn_140840130 as *const ()), // sub_140840130
    Replacement::draft_throws(0x140840190, "unknown_140840190", fn_140840190 as *const (), Throws::Caught), // sub_140840190
    Replacement::draft_throws(0x1408402d0, "unknown_1408402d0", fn_1408402d0 as *const (), Throws::Caught), // sub_1408402d0
    Replacement::draft_throws(0x140840420, "unknown_140840420", fn_140840420 as *const (), Throws::Caught), // sub_140840420
    Replacement::draft_throws(0x140840570, "unknown_140840570", fn_140840570 as *const (), Throws::Caught), // sub_140840570
    Replacement::draft_throws(0x1408406f0, "unknown_1408406f0", tpl_6980::<0x20, 0x8, 0x1408424a0> as *const (), Throws::Discard), // sub_1408406f0
    Replacement::draft_throws(0x140840740, "unknown_140840740", fn_140840740 as *const (), Throws::Caught), // sub_140840740
    Replacement::draft_throws(0x140840cf0, "unknown_140840cf0", fn_140840cf0 as *const (), Throws::Caught), // sub_140840cf0
    Replacement::draft_throws(0x1408412a0, "unknown_1408412a0", fn_1408412a0 as *const (), Throws::Caught), // sub_1408412a0
    Replacement::draft_throws(0x140841820, "unknown_140841820", fn_140841820 as *const (), Throws::Caught), // sub_140841820
    Replacement::draft_throws(0x140841c70, "unknown_140841c70", fn_140841c70 as *const (), Throws::Caught), // sub_140841c70
    Replacement::draft_throws(0x140841f50, "unknown_140841f50", fn_140841f50 as *const (), Throws::Caught), // sub_140841f50
    Replacement::draft_throws(0x140842200, "unknown_140842200", fn_140842200 as *const (), Throws::Caught), // sub_140842200
    Replacement::draft_throws(0x1408424a0, "unknown_1408424a0", fn_1408424a0 as *const (), Throws::Caught), // sub_1408424a0
    Replacement::draft(0x140842570, "unknown_140842570", fn_140842570 as *const ()), // sub_140842570
    Replacement::draft(0x1408425e0, "unknown_1408425e0", fn_1408425e0 as *const ()), // sub_1408425e0
    Replacement::draft_throws(0x140842790, "unknown_140842790", fn_140842790 as *const (), Throws::Caught), // sub_140842790
    Replacement::draft_throws(0x140842c60, "unknown_140842c60", fn_140842c60 as *const (), Throws::Caught), // sub_140842c60
    Replacement::draft_throws(0x1408430f0, "unknown_1408430f0", fn_1408430f0 as *const (), Throws::Caught), // sub_1408430f0
    Replacement::draft(0x140843760, "unknown_140843760", fn_140843760 as *const ()), // sub_140843760
    Replacement::draft(0x140843860, "unknown_140843860", fn_140843860 as *const ()), // sub_140843860
    Replacement::draft_throws(0x140843970, "unknown_140843970", fn_140843970 as *const (), Throws::Discard), // sub_140843970
    Replacement::draft_throws(0x140843b10, "unknown_140843b10", fn_140843b10 as *const (), Throws::Caught), // sub_140843b10
    Replacement::draft_throws(0x140843c50, "unknown_140843c50", fn_140843c50 as *const (), Throws::Caught), // sub_140843c50
    Replacement::draft_throws(0x140843d90, "unknown_140843d90", fn_140843d90 as *const (), Throws::Caught), // sub_140843d90
    Replacement::draft(0x140843ed0, "unknown_140843ed0", fn_140843ed0 as *const ()), // sub_140843ed0
    Replacement::draft(0x140844000, "unknown_140844000", fn_140844000 as *const ()), // sub_140844000
    Replacement::draft_throws(0x140844140, "unknown_140844140", fn_140844140 as *const (), Throws::Discard), // sub_140844140
    Replacement::draft_throws(0x140844230, "unknown_140844230", fn_140844230 as *const (), Throws::Caught), // sub_140844230
    Replacement::draft_throws(0x140844420, "unknown_140844420", fn_140844420 as *const (), Throws::Caught), // sub_140844420
    Replacement::draft_throws(0x140844890, "unknown_140844890", fn_140844890 as *const (), Throws::Caught), // sub_140844890
    Replacement::draft_throws(0x140844cc0, "unknown_140844cc0", fn_140844cc0 as *const (), Throws::Caught), // sub_140844cc0
    Replacement::draft_throws(0x140845010, "unknown_140845010", fn_140845010 as *const (), Throws::Discard), // sub_140845010
    Replacement::draft_throws(0x140845190, "unknown_140845190", fn_140845190 as *const (), Throws::Caught), // sub_140845190
    Replacement::draft_throws(0x140845850, "unknown_140845850", fn_140845850 as *const (), Throws::Caught), // sub_140845850
    Replacement::draft_throws(0x140845e20, "unknown_140845e20", fn_140845e20 as *const (), Throws::Caught), // sub_140845e20
    Replacement::draft(0x140846430, "unknown_140846430", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143267c10> as *const ()), // sub_140846430
    Replacement::draft(0x140846490, "unknown_140846490", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143267c10> as *const ()), // sub_140846490
    Replacement::draft_throws(0x140846520, "unknown_140846520", fn_140846520 as *const (), Throws::Caught), // sub_140846520
    Replacement::draft_throws(0x1408465b0, "unknown_1408465b0", tpl_1908::<0x14387d4a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1408465b0
    Replacement::draft(0x140846630, "unknown_140846630", fn_140846630 as *const ()), // sub_140846630
    Replacement::draft(0x1408466f0, "unknown_1408466f0", fn_1408466f0 as *const ()), // sub_1408466f0
    Replacement::draft(0x1408467d0, "unknown_1408467d0", tpl_9414::<0x14387d4c0, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408467d0
    Replacement::draft_throws(0x140846880, "unknown_140846880", tpl_9415::<0x14387d588, 0x14387d4c0, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140846880
    Replacement::draft(0x140846920, "unknown_140846920", tpl_9416::<0x14387d4c0, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140846920
    Replacement::draft_throws(0x140846990, "unknown_140846990", tpl_9417::<0x14387d588, 0x14387d4c0, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140846990
    Replacement::draft_throws(0x140846a30, "unknown_140846a30", fn_140846a30 as *const (), Throws::Caught), // sub_140846a30
    Replacement::draft_throws(0x140846c90, "unknown_140846c90", fn_140846c90 as *const (), Throws::Caught), // sub_140846c90
    Replacement::draft_throws(0x140846eb0, "unknown_140846eb0", fn_140846eb0 as *const (), Throws::Caught), // sub_140846eb0
    Replacement::draft(0x1408470c0, "unknown_1408470c0", tpl_9421::<0x14387d4c0, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408470c0
    Replacement::draft(0x140847150, "unknown_140847150", tpl_9416::<0x14387d4c0, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140847150
    Replacement::draft_throws(0x1408471c0, "unknown_1408471c0", tpl_9422::<0x14387d4c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269eb8, 0x1402c4730> as *const (), Throws::Discard), // sub_1408471c0
    Replacement::draft(0x140847390, "unknown_140847390", tpl_9423::<0x14387d500, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140847390
    Replacement::draft_throws(0x140847430, "unknown_140847430", tpl_9424::<0x14387d4fc, 0x14387d500, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140847430
    Replacement::draft(0x1408474b0, "unknown_1408474b0", tpl_9425::<0x14387d500, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408474b0
    Replacement::draft_throws(0x140847520, "unknown_140847520", tpl_9426::<0x14387d4fc, 0x14387d500, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140847520
    Replacement::draft_throws(0x1408475b0, "unknown_1408475b0", fn_1408475b0 as *const (), Throws::Caught), // sub_1408475b0
    Replacement::draft_throws(0x140847680, "unknown_140847680", fn_140847680 as *const (), Throws::Caught), // sub_140847680
    Replacement::draft_throws(0x140847750, "unknown_140847750", fn_140847750 as *const (), Throws::Caught), // sub_140847750
    Replacement::draft(0x140847820, "unknown_140847820", tpl_9428::<0x14387d500, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140847820
    Replacement::draft(0x140847890, "unknown_140847890", tpl_9416::<0x14387d500, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140847890
    Replacement::draft_throws(0x140847900, "unknown_140847900", tpl_9429::<0x14387d4fc, 0x14387d500, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140847900
    Replacement::draft_throws(0x140847970, "unknown_140847970", tpl_9430::<0x14387d500, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143269ec8, 0x1402c4aa0> as *const (), Throws::Discard), // sub_140847970
    Replacement::draft(0x140847a20, "unknown_140847a20", fn_140847a20 as *const ()), // sub_140847a20
    Replacement::draft_throws(0x140847ae0, "unknown_140847ae0", fn_140847ae0 as *const (), Throws::Discard), // sub_140847ae0
    Replacement::draft(0x140847b90, "unknown_140847b90", fn_140847b90 as *const ()), // sub_140847b90
    Replacement::draft_throws(0x140847c10, "unknown_140847c10", fn_140847c10 as *const (), Throws::Discard), // sub_140847c10
    Replacement::draft_throws(0x140847cd0, "unknown_140847cd0", fn_140847cd0 as *const (), Throws::Caught), // sub_140847cd0
    Replacement::draft_throws(0x140847dd0, "unknown_140847dd0", fn_140847dd0 as *const (), Throws::Caught), // sub_140847dd0
    Replacement::draft_throws(0x140847ed0, "unknown_140847ed0", fn_140847ed0 as *const (), Throws::Caught), // sub_140847ed0
    Replacement::draft(0x140847fd0, "unknown_140847fd0", fn_140847fd0 as *const ()), // sub_140847fd0
    Replacement::draft(0x140848050, "unknown_140848050", tpl_9437::<0x14387d540, 0x14387d518, 0x1401e9970, 0x338, 0x88, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140848050
    Replacement::draft(0x1408480c0, "unknown_1408480c0", fn_1408480c0 as *const ()), // sub_1408480c0
    Replacement::draft_throws(0x140848140, "unknown_140848140", fn_140848140 as *const (), Throws::Discard), // sub_140848140
    Replacement::draft_throws(0x1408481c0, "unknown_1408481c0", fn_1408481c0 as *const (), Throws::Discard), // sub_1408481c0
    Replacement::draft(0x1408482a0, "unknown_1408482a0", fn_1408482a0 as *const ()), // sub_1408482a0
    Replacement::draft(0x140848320, "unknown_140848320", fn_140848320 as *const ()), // sub_140848320
    Replacement::draft(0x1408483c0, "unknown_1408483c0", tpl_9443::<0x14387d538, 0x14243b0b0, 0x8, 0x10, 0x143881f70, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408483c0
    Replacement::draft(0x140848490, "unknown_140848490", fn_140848490 as *const ()), // sub_140848490
    Replacement::draft_throws(0x140848590, "unknown_140848590", fn_140848590 as *const (), Throws::Discard), // sub_140848590
    Replacement::draft(0x140848660, "unknown_140848660", fn_140848660 as *const ()), // sub_140848660
    Replacement::draft_throws(0x140848730, "unknown_140848730", fn_140848730 as *const (), Throws::Discard), // sub_140848730
    Replacement::draft(0x140848820, "unknown_140848820", fn_140848820 as *const ()), // sub_140848820
    Replacement::draft_throws(0x140848890, "unknown_140848890", fn_140848890 as *const (), Throws::Caught), // sub_140848890
    Replacement::draft_throws(0x140848970, "unknown_140848970", fn_140848970 as *const (), Throws::Caught), // sub_140848970
    Replacement::draft_throws(0x140848a50, "unknown_140848a50", fn_140848a50 as *const (), Throws::Caught), // sub_140848a50
    Replacement::draft(0x140848b30, "unknown_140848b30", fn_140848b30 as *const ()), // sub_140848b30
    Replacement::draft(0x140848bf0, "unknown_140848bf0", fn_140848bf0 as *const ()), // sub_140848bf0
    Replacement::draft(0x140848c90, "unknown_140848c90", tpl_68175::<0x14387d538, 0x14243b0b0, 0x338, 0x120, 0x10, 0xe8, 0x8, 0x14055a1a0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140848c90
    Replacement::draft_throws(0x140848d60, "unknown_140848d60", fn_140848d60 as *const (), Throws::Discard), // sub_140848d60
    Replacement::draft_throws(0x140848e20, "unknown_140848e20", fn_140848e20 as *const (), Throws::Discard), // sub_140848e20
    Replacement::draft(0x140848ef0, "unknown_140848ef0", fn_140848ef0 as *const ()), // sub_140848ef0
    Replacement::draft(0x140848fd0, "unknown_140848fd0", fn_140848fd0 as *const ()), // sub_140848fd0
    Replacement::draft(0x1408490f0, "unknown_1408490f0", fn_1408490f0 as *const ()), // sub_1408490f0
    Replacement::draft_throws(0x140849290, "unknown_140849290", fn_140849290 as *const (), Throws::Discard), // sub_140849290
    Replacement::draft(0x140849430, "unknown_140849430", tpl_9453::<0x14387d568, 0x14243b0b0, 0x338, 0x20, 0x48, 0x90, 0xd8, 0x80, 0xd9, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140849430
    Replacement::draft_throws(0x1408494f0, "unknown_1408494f0", fn_1408494f0 as *const (), Throws::Discard), // sub_1408494f0
    Replacement::draft(0x140849690, "unknown_140849690", fn_140849690 as *const ()), // sub_140849690
    Replacement::draft_throws(0x140849710, "unknown_140849710", fn_140849710 as *const (), Throws::Caught), // sub_140849710
    Replacement::draft_throws(0x140849860, "unknown_140849860", fn_140849860 as *const (), Throws::Caught), // sub_140849860
    Replacement::draft_throws(0x1408499b0, "unknown_1408499b0", fn_1408499b0 as *const (), Throws::Caught), // sub_1408499b0
    Replacement::draft(0x140849b00, "unknown_140849b00", fn_140849b00 as *const ()), // sub_140849b00
    Replacement::draft(0x140849c20, "unknown_140849c20", fn_140849c20 as *const ()), // sub_140849c20
    Replacement::draft(0x140849ce0, "unknown_140849ce0", tpl_9453::<0x14387d568, 0x14243b0b0, 0x338, 0xc0, 0x48, 0x90, 0xd8, 0x120, 0xd9, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140849ce0
    Replacement::draft_throws(0x140849db0, "unknown_140849db0", fn_140849db0 as *const (), Throws::Discard), // sub_140849db0
    Replacement::draft_throws(0x140849e70, "unknown_140849e70", fn_140849e70 as *const (), Throws::Discard), // sub_140849e70
    Replacement::draft(0x140849fe0, "unknown_140849fe0", fn_140849fe0 as *const ()), // sub_140849fe0
    Replacement::draft(0x14084a0b0, "unknown_14084a0b0", fn_14084a0b0 as *const ()), // sub_14084a0b0
    Replacement::draft(0x14084a210, "unknown_14084a210", tpl_9462::<0x14387d4b8, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084a210
    Replacement::draft_throws(0x14084a2b0, "unknown_14084a2b0", tpl_9463::<0x14387d590, 0x14387d4b8, 0x1401e9fa0, 0x143427660, 0x328, 0x338, 0x140715850, 0x369, 0x8, 0xc, 0x24, 0x18, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084a2b0
    Replacement::draft(0x14084a380, "unknown_14084a380", tpl_9464::<0x14387d4b8, 0x14243b0b0, 0x338, 0x140715850, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084a380
    Replacement::draft_throws(0x14084a3f0, "unknown_14084a3f0", tpl_9465::<0x14387d590, 0x14387d4b8, 0x1401e9b60, 0x143427660, 0x328, 0x338, 0x120, 0x8, 0xc, 0x14, 0x18, 0x20, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084a3f0
    Replacement::draft_throws(0x14084a510, "unknown_14084a510", fn_14084a510 as *const (), Throws::Caught), // sub_14084a510
    Replacement::draft_throws(0x14084a5e0, "unknown_14084a5e0", fn_14084a5e0 as *const (), Throws::Caught), // sub_14084a5e0
    Replacement::draft_throws(0x14084a6b0, "unknown_14084a6b0", fn_14084a6b0 as *const (), Throws::Caught), // sub_14084a6b0
    Replacement::draft(0x14084a8c0, "unknown_14084a8c0", tpl_9468::<0x14387d4b8, 0x14243b0b0, 0x143427660, 0x338, 0x8, 0xc, 0x24, 0x18, 0x2c, 0x14, 0x20, 0xfffffffffffffff8> as *const ()), // sub_14084a8c0
    Replacement::draft(0x14084a990, "unknown_14084a990", tpl_9464::<0x14387d4b8, 0x14243b0b0, 0x338, 0x140715760, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084a990
    Replacement::draft_throws(0x14084aa00, "unknown_14084aa00", tpl_9469::<0x14387d590, 0x14387d4b8, 0x1401e9770, 0x338, 0x14, 0xc, 0x8, 0x120, 0x18, 0x20, 0x24, 0x2c, 0x1401e9590> as *const (), Throws::Discard), // sub_14084aa00
    Replacement::draft_throws(0x14084aaa0, "unknown_14084aaa0", tpl_9470::<0x14387d4b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x20, 0x143428b00, 0x143427660, 0x1404a4e70, 0x24, 0x143269e18, 0x1413c48e0> as *const (), Throws::Discard), // sub_14084aaa0
    Replacement::draft(0x14084aba0, "unknown_14084aba0", fn_14084aba0 as *const ()), // sub_14084aba0
    Replacement::draft(0x14084ac70, "unknown_14084ac70", fn_14084ac70 as *const ()), // sub_14084ac70
    Replacement::draft(0x14084adb0, "unknown_14084adb0", fn_14084adb0 as *const ()), // sub_14084adb0
    Replacement::draft_throws(0x14084aef0, "unknown_14084aef0", fn_14084aef0 as *const (), Throws::Discard), // sub_14084aef0
    Replacement::draft(0x14084b010, "unknown_14084b010", tpl_9475::<0x14387d4d8, 0x14243b0b0, 0x338, 0x48, 0x4, 0x20, 0x8c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084b010
    Replacement::draft_throws(0x14084b0a0, "unknown_14084b0a0", fn_14084b0a0 as *const (), Throws::Discard), // sub_14084b0a0
    Replacement::draft_throws(0x14084b1c0, "unknown_14084b1c0", fn_14084b1c0 as *const (), Throws::Caught), // sub_14084b1c0
    Replacement::draft_throws(0x14084b6d0, "unknown_14084b6d0", fn_14084b6d0 as *const (), Throws::Caught), // sub_14084b6d0
    Replacement::draft_throws(0x14084bb50, "unknown_14084bb50", fn_14084bb50 as *const (), Throws::Caught), // sub_14084bb50
    Replacement::draft(0x14084bfb0, "unknown_14084bfb0", fn_14084bfb0 as *const ()), // sub_14084bfb0
    Replacement::draft(0x14084c090, "unknown_14084c090", fn_14084c090 as *const ()), // sub_14084c090
    Replacement::draft(0x14084c130, "unknown_14084c130", fn_14084c130 as *const ()), // sub_14084c130
    Replacement::draft_throws(0x14084c1e0, "unknown_14084c1e0", fn_14084c1e0 as *const (), Throws::Discard), // sub_14084c1e0
    Replacement::draft_throws(0x14084c270, "unknown_14084c270", fn_14084c270 as *const (), Throws::Discard), // sub_14084c270
    Replacement::draft(0x14084c370, "unknown_14084c370", fn_14084c370 as *const ()), // sub_14084c370
    Replacement::draft(0x14084c4d0, "unknown_14084c4d0", tpl_9485::<0x14387d578, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084c4d0
    Replacement::draft_throws(0x14084c570, "unknown_14084c570", tpl_9486::<0x14387d4b0, 0x14387d578, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084c570
    Replacement::draft(0x14084c5f0, "unknown_14084c5f0", tpl_9416::<0x14387d578, 0x14243b0b0, 0x338, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084c5f0
    Replacement::draft_throws(0x14084c660, "unknown_14084c660", tpl_9487::<0x14387d4b0, 0x14387d578, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084c660
    Replacement::draft_throws(0x14084c6f0, "unknown_14084c6f0", fn_14084c6f0 as *const (), Throws::Caught), // sub_14084c6f0
    Replacement::draft_throws(0x14084c8e0, "unknown_14084c8e0", fn_14084c8e0 as *const (), Throws::Caught), // sub_14084c8e0
    Replacement::draft_throws(0x14084caa0, "unknown_14084caa0", fn_14084caa0 as *const (), Throws::Caught), // sub_14084caa0
    Replacement::draft(0x14084cc60, "unknown_14084cc60", tpl_9491::<0x14387d578, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084cc60
    Replacement::draft(0x14084ccd0, "unknown_14084ccd0", fn_14084ccd0 as *const ()), // sub_14084ccd0
    Replacement::draft(0x14084cd40, "unknown_14084cd40", fn_14084cd40 as *const ()), // sub_14084cd40
    Replacement::draft_throws(0x14084cdc0, "unknown_14084cdc0", tpl_9494::<0x14387d4b0, 0x14387d578, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14084cdc0
    Replacement::draft_throws(0x14084ce30, "unknown_14084ce30", fn_14084ce30 as *const (), Throws::Discard), // sub_14084ce30
    Replacement::draft(0x14084cee0, "unknown_14084cee0", fn_14084cee0 as *const ()), // sub_14084cee0
    Replacement::draft(0x14084cf80, "unknown_14084cf80", fn_14084cf80 as *const ()), // sub_14084cf80
    Replacement::draft(0x14084d030, "unknown_14084d030", tpl_9414::<0x14387d598, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084d030
    Replacement::draft_throws(0x14084d0e0, "unknown_14084d0e0", tpl_9415::<0x14387d558, 0x14387d598, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084d0e0
    Replacement::draft(0x14084d180, "unknown_14084d180", tpl_9416::<0x14387d598, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084d180
    Replacement::draft_throws(0x14084d1f0, "unknown_14084d1f0", tpl_9417::<0x14387d558, 0x14387d598, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084d1f0
    Replacement::draft_throws(0x14084d290, "unknown_14084d290", fn_14084d290 as *const (), Throws::Caught), // sub_14084d290
    Replacement::draft_throws(0x14084d330, "unknown_14084d330", fn_14084d330 as *const (), Throws::Caught), // sub_14084d330
    Replacement::draft_throws(0x14084d3d0, "unknown_14084d3d0", fn_14084d3d0 as *const (), Throws::Caught), // sub_14084d3d0
    Replacement::draft(0x14084d470, "unknown_14084d470", tpl_9421::<0x14387d598, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084d470
    Replacement::draft(0x14084d500, "unknown_14084d500", tpl_9416::<0x14387d598, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084d500
    Replacement::draft_throws(0x14084d570, "unknown_14084d570", tpl_9422::<0x14387d598, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269e38, 0x1402c4730> as *const (), Throws::Discard), // sub_14084d570
    Replacement::draft_throws(0x14084d630, "unknown_14084d630", fn_14084d630 as *const (), Throws::Caught), // sub_14084d630
    Replacement::draft_throws(0x14084d6b0, "unknown_14084d6b0", tpl_649::<0x14387d548, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_14084d6b0
    Replacement::draft(0x14084d730, "unknown_14084d730", tpl_9414::<0x14387d4e8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084d730
    Replacement::draft_throws(0x14084d7e0, "unknown_14084d7e0", tpl_9415::<0x14387d50c, 0x14387d4e8, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084d7e0
    Replacement::draft(0x14084d880, "unknown_14084d880", tpl_9416::<0x14387d4e8, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084d880
    Replacement::draft_throws(0x14084d8f0, "unknown_14084d8f0", tpl_9417::<0x14387d50c, 0x14387d4e8, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084d8f0
    Replacement::draft_throws(0x14084d990, "unknown_14084d990", fn_14084d990 as *const (), Throws::Caught), // sub_14084d990
    Replacement::draft_throws(0x14084dbf0, "unknown_14084dbf0", fn_14084dbf0 as *const (), Throws::Caught), // sub_14084dbf0
    Replacement::draft_throws(0x14084de10, "unknown_14084de10", fn_14084de10 as *const (), Throws::Caught), // sub_14084de10
    Replacement::draft(0x14084e020, "unknown_14084e020", tpl_9421::<0x14387d4e8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084e020
    Replacement::draft(0x14084e0b0, "unknown_14084e0b0", tpl_9416::<0x14387d4e8, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084e0b0
    Replacement::draft_throws(0x14084e120, "unknown_14084e120", tpl_9422::<0x14387d4e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269e48, 0x1402c4730> as *const (), Throws::Discard), // sub_14084e120
    Replacement::draft(0x14084e1e0, "unknown_14084e1e0", fn_14084e1e0 as *const ()), // sub_14084e1e0
    Replacement::draft(0x14084e290, "unknown_14084e290", fn_14084e290 as *const ()), // sub_14084e290
    Replacement::draft(0x14084e360, "unknown_14084e360", tpl_9504::<0x14387d510, 0x14243b0b0, 0xc, 0x338, 0x1426d4580, 0x7, 0x8, 0x4, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084e360
    Replacement::draft_throws(0x14084e430, "unknown_14084e430", tpl_9505::<0x14387d508, 0x14387d510, 0x1401e9fa0, 0x328, 0x338, 0x80, 0x369, 0x4, 0x20, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084e430
    Replacement::draft(0x14084e500, "unknown_14084e500", tpl_9506::<0x14387d510, 0x14243b0b0, 0x338, 0x80, 0x4, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084e500
    Replacement::draft_throws(0x14084e580, "unknown_14084e580", tpl_9507::<0x14387d508, 0x14387d510, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x4, 0x1424b4da0, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084e580
    Replacement::draft_throws(0x14084e660, "unknown_14084e660", fn_14084e660 as *const (), Throws::Caught), // sub_14084e660
    Replacement::draft_throws(0x14084e8e0, "unknown_14084e8e0", fn_14084e8e0 as *const (), Throws::Caught), // sub_14084e8e0
    Replacement::draft_throws(0x14084eb30, "unknown_14084eb30", fn_14084eb30 as *const (), Throws::Caught), // sub_14084eb30
    Replacement::draft(0x14084ed70, "unknown_14084ed70", tpl_9511::<0x14387d510, 0x14243b0b0, 0x338, 0x4, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084ed70
    Replacement::draft(0x14084ee10, "unknown_14084ee10", tpl_9437::<0x14387d508, 0x14387d510, 0x1401e9970, 0x338, 0x120, 0x4, 0xc0, 0x1401e9860> as *const ()), // sub_14084ee10
    Replacement::draft(0x14084ee80, "unknown_14084ee80", tpl_9506::<0x14387d510, 0x14243b0b0, 0x338, 0x120, 0x4, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084ee80
    Replacement::draft_throws(0x14084ef00, "unknown_14084ef00", tpl_9512::<0x14387d508, 0x14387d510, 0x1401e9770, 0x338, 0x120, 0x4, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_14084ef00
    Replacement::draft_throws(0x14084ef80, "unknown_14084ef80", tpl_9513::<0x14387d510, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143269e58, 0x14020f0f0, 0x1426cdcb0, 0x4, 0x143269eb8, 0x1402c4730> as *const (), Throws::Discard), // sub_14084ef80
    Replacement::draft(0x14084f070, "unknown_14084f070", tpl_9504::<0x14387d4e0, 0x14243b0b0, 0xc, 0x338, 0x1426d4580, 0x7, 0x8, 0x4, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084f070
    Replacement::draft_throws(0x14084f140, "unknown_14084f140", tpl_9505::<0x14387d530, 0x14387d4e0, 0x1401e9fa0, 0x328, 0x338, 0x80, 0x369, 0x4, 0x20, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084f140
    Replacement::draft(0x14084f210, "unknown_14084f210", tpl_9506::<0x14387d4e0, 0x14243b0b0, 0x338, 0x80, 0x4, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084f210
    Replacement::draft_throws(0x14084f290, "unknown_14084f290", tpl_9507::<0x14387d530, 0x14387d4e0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x4, 0x1424b4da0, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084f290
    Replacement::draft_throws(0x14084f370, "unknown_14084f370", fn_14084f370 as *const (), Throws::Caught), // sub_14084f370
    Replacement::draft_throws(0x14084f5f0, "unknown_14084f5f0", fn_14084f5f0 as *const (), Throws::Caught), // sub_14084f5f0
    Replacement::draft_throws(0x14084f840, "unknown_14084f840", fn_14084f840 as *const (), Throws::Caught), // sub_14084f840
    Replacement::draft(0x14084fa80, "unknown_14084fa80", tpl_9511::<0x14387d4e0, 0x14243b0b0, 0x338, 0x4, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084fa80
    Replacement::draft(0x14084fb20, "unknown_14084fb20", tpl_9437::<0x14387d530, 0x14387d4e0, 0x1401e9970, 0x338, 0x120, 0x4, 0xc0, 0x1401e9860> as *const ()), // sub_14084fb20
    Replacement::draft(0x14084fb90, "unknown_14084fb90", tpl_9506::<0x14387d4e0, 0x14243b0b0, 0x338, 0x120, 0x4, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084fb90
    Replacement::draft_throws(0x14084fc10, "unknown_14084fc10", tpl_9512::<0x14387d530, 0x14387d4e0, 0x1401e9770, 0x338, 0x120, 0x4, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_14084fc10
    Replacement::draft_throws(0x14084fc90, "unknown_14084fc90", tpl_9513::<0x14387d4e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143269e68, 0x14020f0f0, 0x1426cdcb0, 0x4, 0x143269e78, 0x1402c4730> as *const (), Throws::Discard), // sub_14084fc90
    Replacement::draft(0x14084fd80, "unknown_14084fd80", tpl_9414::<0x14387d580, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084fd80
    Replacement::draft_throws(0x14084fe30, "unknown_14084fe30", tpl_9415::<0x14387d550, 0x14387d580, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14084fe30
    Replacement::draft(0x14084fed0, "unknown_14084fed0", tpl_9416::<0x14387d580, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14084fed0
    Replacement::draft_throws(0x14084ff40, "unknown_14084ff40", tpl_9417::<0x14387d550, 0x14387d580, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14084ff40
    Replacement::draft_throws(0x14084ffe0, "unknown_14084ffe0", fn_14084ffe0 as *const (), Throws::Caught), // sub_14084ffe0
    Replacement::draft_throws(0x140850240, "unknown_140850240", fn_140850240 as *const (), Throws::Caught), // sub_140850240
    Replacement::draft_throws(0x140850460, "unknown_140850460", fn_140850460 as *const (), Throws::Caught), // sub_140850460
    Replacement::draft(0x140850670, "unknown_140850670", tpl_9421::<0x14387d580, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140850670
    Replacement::draft(0x140850700, "unknown_140850700", tpl_9416::<0x14387d580, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140850700
    Replacement::draft_throws(0x140850770, "unknown_140850770", tpl_9422::<0x14387d580, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269e88, 0x1402c4730> as *const (), Throws::Discard), // sub_140850770
    Replacement::draft(0x140850830, "unknown_140850830", tpl_9414::<0x14387d560, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140850830
    Replacement::draft_throws(0x1408508e0, "unknown_1408508e0", tpl_9415::<0x14387d570, 0x14387d560, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408508e0
    Replacement::draft(0x140850980, "unknown_140850980", tpl_9416::<0x14387d560, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140850980
    Replacement::draft_throws(0x1408509f0, "unknown_1408509f0", tpl_9417::<0x14387d570, 0x14387d560, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_1408509f0
    Replacement::draft_throws(0x140850a90, "unknown_140850a90", fn_140850a90 as *const (), Throws::Caught), // sub_140850a90
    Replacement::draft_throws(0x140850cf0, "unknown_140850cf0", fn_140850cf0 as *const (), Throws::Caught), // sub_140850cf0
    Replacement::draft_throws(0x140850f10, "unknown_140850f10", fn_140850f10 as *const (), Throws::Caught), // sub_140850f10
    Replacement::draft(0x140851120, "unknown_140851120", tpl_9421::<0x14387d560, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140851120
    Replacement::draft(0x1408511b0, "unknown_1408511b0", tpl_9416::<0x14387d560, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408511b0
    Replacement::draft_throws(0x140851220, "unknown_140851220", tpl_9422::<0x14387d560, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269e98, 0x1402c4730> as *const (), Throws::Discard), // sub_140851220
    Replacement::draft(0x1408512e0, "unknown_1408512e0", fn_1408512e0 as *const ()), // sub_1408512e0
    Replacement::draft(0x140851390, "unknown_140851390", fn_140851390 as *const ()), // sub_140851390
    Replacement::draft(0x140851460, "unknown_140851460", fn_140851460 as *const ()), // sub_140851460
    Replacement::draft_throws(0x140851530, "unknown_140851530", fn_140851530 as *const (), Throws::Discard), // sub_140851530
    Replacement::draft(0x140851610, "unknown_140851610", tpl_9506::<0x14387d4c8, 0x14243b0b0, 0x338, 0x20, 0x88, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140851610
    Replacement::draft_throws(0x140851690, "unknown_140851690", fn_140851690 as *const (), Throws::Discard), // sub_140851690
    Replacement::draft_throws(0x140851780, "unknown_140851780", fn_140851780 as *const (), Throws::Caught), // sub_140851780
    Replacement::draft_throws(0x140851a00, "unknown_140851a00", fn_140851a00 as *const (), Throws::Caught), // sub_140851a00
    Replacement::draft_throws(0x140851c50, "unknown_140851c50", fn_140851c50 as *const (), Throws::Caught), // sub_140851c50
    Replacement::draft(0x140851e90, "unknown_140851e90", fn_140851e90 as *const ()), // sub_140851e90
    Replacement::draft(0x140851f40, "unknown_140851f40", tpl_9437::<0x14387d520, 0x14387d4c8, 0x1401e9970, 0x338, 0xc0, 0x88, 0x120, 0x1401e9860> as *const ()), // sub_140851f40
    Replacement::draft(0x140851fc0, "unknown_140851fc0", tpl_9506::<0x14387d4c8, 0x14243b0b0, 0x338, 0xc0, 0x88, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140851fc0
    Replacement::draft_throws(0x140852040, "unknown_140852040", fn_140852040 as *const (), Throws::Discard), // sub_140852040
    Replacement::draft_throws(0x1408520c0, "unknown_1408520c0", fn_1408520c0 as *const (), Throws::Discard), // sub_1408520c0
    Replacement::draft(0x1408521b0, "unknown_1408521b0", tpl_9414::<0x14387d4f0, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408521b0
    Replacement::draft_throws(0x140852260, "unknown_140852260", tpl_9415::<0x14387d4f8, 0x14387d4f0, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140852260
    Replacement::draft(0x140852300, "unknown_140852300", tpl_9416::<0x14387d4f0, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140852300
    Replacement::draft_throws(0x140852370, "unknown_140852370", tpl_9417::<0x14387d4f8, 0x14387d4f0, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140852370
    Replacement::draft_throws(0x140852410, "unknown_140852410", fn_140852410 as *const (), Throws::Caught), // sub_140852410
    Replacement::draft_throws(0x140852670, "unknown_140852670", fn_140852670 as *const (), Throws::Caught), // sub_140852670
    Replacement::draft_throws(0x140852890, "unknown_140852890", fn_140852890 as *const (), Throws::Caught), // sub_140852890
    Replacement::draft(0x140852aa0, "unknown_140852aa0", tpl_9421::<0x14387d4f0, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140852aa0
    Replacement::draft(0x140852b30, "unknown_140852b30", tpl_9416::<0x14387d4f0, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140852b30
    Replacement::draft_throws(0x140852ba0, "unknown_140852ba0", tpl_9422::<0x14387d4f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269e98, 0x1402c4730> as *const (), Throws::Discard), // sub_140852ba0
    Replacement::draft(0x140852c60, "unknown_140852c60", tpl_9414::<0x14387d528, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140852c60
    Replacement::draft_throws(0x140852d10, "unknown_140852d10", tpl_9415::<0x14387d58c, 0x14387d528, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140852d10
    Replacement::draft(0x140852db0, "unknown_140852db0", tpl_9416::<0x14387d528, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140852db0
    Replacement::draft_throws(0x140852e20, "unknown_140852e20", tpl_9417::<0x14387d58c, 0x14387d528, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140852e20
    Replacement::draft_throws(0x140852ec0, "unknown_140852ec0", fn_140852ec0 as *const (), Throws::Caught), // sub_140852ec0
    Replacement::draft_throws(0x140853120, "unknown_140853120", fn_140853120 as *const (), Throws::Caught), // sub_140853120
    Replacement::draft_throws(0x140853340, "unknown_140853340", fn_140853340 as *const (), Throws::Caught), // sub_140853340
    Replacement::draft(0x140853550, "unknown_140853550", tpl_9421::<0x14387d528, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140853550
    Replacement::draft(0x1408535e0, "unknown_1408535e0", tpl_9416::<0x14387d528, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408535e0
    Replacement::draft_throws(0x140853650, "unknown_140853650", tpl_9422::<0x14387d528, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143269ea8, 0x1402c4730> as *const (), Throws::Discard), // sub_140853650
    Replacement::draft(0x140853710, "unknown_140853710", tpl_9530::<0x8, 0x1426d4580, 0x338, 0x1, 0x140559ea0> as *const ()), // sub_140853710
    Replacement::draft(0x140853830, "unknown_140853830", fn_140853830 as *const ()), // sub_140853830
    Replacement::draft_throws(0x1408538c0, "unknown_1408538c0", fn_1408538c0 as *const (), Throws::Caught), // sub_1408538c0
    Replacement::draft_throws(0x140853b90, "unknown_140853b90", fn_140853b90 as *const (), Throws::Caught), // sub_140853b90
    Replacement::draft_throws(0x140853e60, "unknown_140853e60", fn_140853e60 as *const (), Throws::Caught), // sub_140853e60
    Replacement::draft_throws(0x140854290, "unknown_140854290", fn_140854290 as *const (), Throws::Caught), // sub_140854290
    Replacement::draft_throws(0x140854690, "unknown_140854690", fn_140854690 as *const (), Throws::Caught), // sub_140854690
    Replacement::draft(0x140854a90, "unknown_140854a90", fn_140854a90 as *const ()), // sub_140854a90
    Replacement::draft_throws(0x140854b10, "unknown_140854b10", fn_140854b10 as *const (), Throws::Caught), // sub_140854b10
    Replacement::draft_throws(0x140854be0, "unknown_140854be0", fn_140854be0 as *const (), Throws::Caught), // sub_140854be0
    Replacement::draft(0x140854c40, "unknown_140854c40", fn_140854c40 as *const ()), // sub_140854c40
    Replacement::draft_throws(0x140854c90, "unknown_140854c90", fn_140854c90 as *const (), Throws::Caught), // sub_140854c90
    Replacement::draft_throws(0x140855320, "unknown_140855320", fn_140855320 as *const (), Throws::Caught), // sub_140855320
    Replacement::draft_throws(0x140855930, "unknown_140855930", fn_140855930 as *const (), Throws::Caught), // sub_140855930
    Replacement::draft(0x140855f00, "content_140855f00", fn_140855f00 as *const ()), // content::LuaScriptResource::content::LuaScriptResource::vfunc_0
    Replacement::draft(0x140855f50, "content_140855f50", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14326a3c8, 0x8> as *const ()), // sub_140855f50
    Replacement::draft(0x140855fd0, "content_140855fd0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326a3c8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140855fd0
    Replacement::draft_throws(0x140856070, "content_140856070", fn_140856070 as *const (), Throws::Caught), // sub_140856070
    Replacement::draft_throws(0x140856100, "content_140856100", tpl_1908::<0x14387d5a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140856100
    Replacement::draft(0x140856180, "content_140856180", fn_140856180 as *const ()), // sub_140856180
    Replacement::draft(0x140856240, "content_140856240", fn_140856240 as *const ()), // sub_140856240
    Replacement::draft(0x140856320, "content_140856320", tpl_9543::<0x14387d5b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140856320
    Replacement::draft(0x1408563c0, "content_1408563c0", fn_1408563c0 as *const ()), // sub_1408563c0
    Replacement::draft_throws(0x1408564d0, "content_1408564d0", fn_1408564d0 as *const (), Throws::Discard), // sub_1408564d0
    Replacement::draft(0x1408565b0, "content_1408565b0", tpl_9546::<0x14387d5b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x338, 0x20> as *const ()), // sub_1408565b0
    Replacement::draft_throws(0x140856660, "content_140856660", fn_140856660 as *const (), Throws::Discard), // sub_140856660
    Replacement::draft_throws(0x140856730, "content_140856730", fn_140856730 as *const (), Throws::Caught), // sub_140856730
    Replacement::draft_throws(0x140856990, "content_140856990", fn_140856990 as *const (), Throws::Caught), // sub_140856990
    Replacement::draft_throws(0x140856bc0, "content_140856bc0", fn_140856bc0 as *const (), Throws::Caught), // sub_140856bc0
    Replacement::draft(0x140856de0, "content_140856de0", fn_140856de0 as *const ()), // sub_140856de0
    Replacement::draft(0x140856ec0, "content_140856ec0", fn_140856ec0 as *const ()), // sub_140856ec0
    Replacement::draft(0x140856f60, "content_140856f60", tpl_9546::<0x14387d5b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x338, 0xc0> as *const ()), // sub_140856f60
    Replacement::draft_throws(0x140857020, "content_140857020", fn_140857020 as *const (), Throws::Discard), // sub_140857020
    Replacement::draft_throws(0x1408570c0, "content_1408570c0", fn_1408570c0 as *const (), Throws::Discard), // sub_1408570c0
    Replacement::draft_throws(0x1408571b0, "content_1408571b0", fn_1408571b0 as *const (), Throws::Discard), // sub_1408571b0
    Replacement::draft(0x140857470, "content_140857470", tpl_9555::<0x14397d1d0, 0x14388ec58, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::InplaceString<64>,r::Allocator<r::InplaceString<64>,0>>>::content::LuaScri
    Replacement::draft(0x140857520, "content_140857520", tpl_9555::<0x14397d1d0, 0x14388eeb8, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<content::ScriptReference,r::Allocator<content::ScriptReference,0>>>::content:
    Replacement::draft(0x1408575d0, "content_1408575d0", tpl_9555::<0x14397d1d0, 0x14388eae8, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::GlobalID,r::Allocator<r::GlobalID,0>>>::content::LuaScriptProperty<r::Dyna
    Replacement::draft(0x140857680, "content_140857680", tpl_9555::<0x14397d1d0, 0x14388ee28, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::ResourceID,r::Allocator<r::ResourceID,0>>>::content::LuaScriptProperty<r::
    Replacement::draft(0x140857730, "content_140857730", tpl_9555::<0x14397d1d0, 0x14388ed78, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<m::Vector4Template<float>,r::Allocator<m::Vector4Template<float>,0>>>::conten
    Replacement::draft(0x1408577e0, "content_1408577e0", tpl_9555::<0x14397d1d0, 0x14388ecc8, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<m::Vector3Template<float>,r::Allocator<m::Vector3Template<float>,0>>>::conten
    Replacement::draft(0x140857890, "content_140857890", tpl_9555::<0x14397d1d0, 0x14388efc8, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<m::Vector2Template<float>,r::Allocator<m::Vector2Template<float>,0>>>::conten
    Replacement::draft(0x140857940, "content_140857940", tpl_9555::<0x14397d1d0, 0x14388ed38, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<float,r::Allocator<float,0>>>::content::LuaScriptProperty<r::DynamicArray<flo
    Replacement::draft(0x1408579f0, "content_1408579f0", tpl_9555::<0x14397d1d0, 0x14388ef58, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<unsigned int,r::Allocator<unsigned int,0>>>::content::LuaScriptProperty<r::Dy
    Replacement::draft(0x140857aa0, "content_140857aa0", tpl_9555::<0x14397d1d0, 0x14388ede8, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<int,r::Allocator<int,0>>>::content::LuaScriptProperty<r::DynamicArray<int,r::
    Replacement::draft(0x140857b50, "content_140857b50", tpl_9555::<0x14397d1d0, 0x14388eb88, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::DynamicArray<bool,r::Allocator<bool,0>>>::content::LuaScriptProperty<r::DynamicArray<bool,
    Replacement::draft(0x140857c00, "content_140857c00", tpl_9555::<0x14397d1d0, 0x14388ee88, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::InplaceString<64>>::content::LuaScriptProperty<r::InplaceString<64>>::vfunc_1
    Replacement::draft(0x140857cb0, "content_140857cb0", tpl_9555::<0x14397d1d0, 0x14388eab0, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<content::ScriptReference>::content::LuaScriptProperty<content::ScriptReference>::vfunc_1
    Replacement::draft(0x140857d60, "content_140857d60", tpl_9555::<0x14397d1d0, 0x14388ef30, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::GlobalID>::content::LuaScriptProperty<r::GlobalID>::vfunc_1
    Replacement::draft(0x140857e10, "content_140857e10", tpl_9555::<0x14397d1d0, 0x14388ec30, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<r::ResourceID>::content::LuaScriptProperty<r::ResourceID>::vfunc_1
    Replacement::draft(0x140857ec0, "content_140857ec0", tpl_9555::<0x14397d1d0, 0x14388eb40, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<m::Vector4Template<float>>::content::LuaScriptProperty<m::Vector4Template<float>>::vfunc_1
    Replacement::draft(0x140857f70, "content_140857f70", tpl_9555::<0x14397d1d0, 0x14388ef98, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<m::Vector3Template<float>>::content::LuaScriptProperty<m::Vector3Template<float>>::vfunc_1
    Replacement::draft(0x140858020, "content_140858020", tpl_9555::<0x14397d1d0, 0x14388ec00, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<m::Vector2Template<float>>::content::LuaScriptProperty<m::Vector2Template<float>>::vfunc_1
    Replacement::draft(0x1408580d0, "content_1408580d0", tpl_9555::<0x14397d1d0, 0x14388eb70, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<float>::content::LuaScriptProperty<float>::vfunc_1
    Replacement::draft(0x140858180, "content_140858180", tpl_9555::<0x14397d1d0, 0x14388ebd0, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<unsigned int>::content::LuaScriptProperty<unsigned int>::vfunc_1
    Replacement::draft(0x140858230, "content_140858230", tpl_9555::<0x14397d1d0, 0x14388ebe8, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<int>::content::LuaScriptProperty<int>::vfunc_1
    Replacement::draft(0x1408582e0, "content_1408582e0", tpl_9555::<0x14397d1d0, 0x14388f038, 0x142646358, 0x50> as *const ()), // content::LuaScriptProperty<bool>::content::LuaScriptProperty<bool>::vfunc_1
    Replacement::draft(0x140858360, "content_140858360", tpl_2084::<0x14387d6e0, 0x14243b0b0, 0x14028a610, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140858360
    Replacement::draft_throws(0x1408583d0, "content_1408583d0", tpl_2085::<0x14387d610, 0x14387d6e0, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408583d0
    Replacement::draft(0x140858450, "content_140858450", tpl_2086::<0x14387d6e0, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140858450
    Replacement::draft_throws(0x1408584c0, "content_1408584c0", tpl_2087::<0x14387d610, 0x14387d6e0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1408584c0
    Replacement::draft_throws(0x140858550, "content_140858550", fn_140858550 as *const (), Throws::Caught), // sub_140858550
    Replacement::draft_throws(0x140858600, "content_140858600", fn_140858600 as *const (), Throws::Caught), // sub_140858600
    Replacement::draft_throws(0x1408586b0, "content_1408586b0", fn_1408586b0 as *const (), Throws::Caught), // sub_1408586b0
    Replacement::draft(0x140858760, "content_140858760", tpl_2089::<0x14387d6e0, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140858760
    Replacement::draft(0x1408587d0, "content_1408587d0", tpl_2080::<0x14387d610, 0x14387d6e0, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // sub_1408587d0
    Replacement::draft(0x140858840, "content_140858840", tpl_2076::<0x14387d6e0, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140858840
    Replacement::draft_throws(0x1408588b0, "content_1408588b0", tpl_2090::<0x14387d610, 0x14387d6e0, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1408588b0
    Replacement::draft_throws(0x140858920, "content_140858920", tpl_2091::<0x14387d6e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14326a7b8, 0x1402bf9b0> as *const (), Throws::Discard), // sub_140858920
    Replacement::draft(0x1408589c0, "content_1408589c0", fn_1408589c0 as *const ()), // sub_1408589c0
    Replacement::draft_throws(0x140858a70, "content_140858a70", tpl_9557::<0x14387d67c, 0x14387d5c8, 0x1401e9fa0, 0x328, 0x338, 0x8, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140858a70
    Replacement::draft(0x140858b10, "content_140858b10", tpl_8840::<0x14387d5c8, 0x14243b0b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140858b10
    Replacement::draft_throws(0x140858b80, "content_140858b80", tpl_9558::<0x14387d67c, 0x14387d5c8, 0x1401e9b60, 0x328, 0x8, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140858b80
    Replacement::draft_throws(0x140858c20, "content_140858c20", fn_140858c20 as *const (), Throws::Caught), // sub_140858c20
    Replacement::draft_throws(0x140858cd0, "content_140858cd0", fn_140858cd0 as *const (), Throws::Caught), // sub_140858cd0
    Replacement::draft_throws(0x140858d80, "content_140858d80", fn_140858d80 as *const (), Throws::Caught), // sub_140858d80
    Replacement::draft(0x140858e30, "content_140858e30", fn_140858e30 as *const ()), // sub_140858e30
    Replacement::draft(0x140858ed0, "content_140858ed0", tpl_9561::<0x14387d67c, 0x14387d5c8, 0x1401e9970, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_140858ed0
    Replacement::draft(0x140858f40, "content_140858f40", tpl_8840::<0x14387d5c8, 0x14243b0b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140858f40
    Replacement::draft_throws(0x140858fb0, "content_140858fb0", tpl_9562::<0x14387d67c, 0x14387d5c8, 0x1401e9770, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_140858fb0
    Replacement::draft_throws(0x140859010, "content_140859010", tpl_9563::<0x14387d5c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730> as *const (), Throws::Discard), // sub_140859010
    Replacement::draft(0x1408590d0, "content_1408590d0", tpl_9564::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326cd78, 0xb, 0xc> as *const ()), // sub_1408590d0
    Replacement::draft(0x140859170, "content_140859170", tpl_9565::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326cd78, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_140859170
    Replacement::draft(0x1408592c0, "content_1408592c0", tpl_8125::<0x14387d6e8, 0x14243b0b0, 0x1408589c0, 0x50, 0x338, 0x1426d4580, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408592c0
    Replacement::draft_throws(0x140859360, "content_140859360", tpl_8126::<0x14387d6b8, 0x14387d6e8, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x50, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140859360
    Replacement::draft(0x1408593f0, "content_1408593f0", tpl_7857::<0x14387d6e8, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408593f0
    Replacement::draft_throws(0x140859470, "content_140859470", tpl_8127::<0x14387d6b8, 0x14387d6e8, 0x1401e9b60, 0x140858b80, 0x328, 0x338, 0x120, 0x50, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140859470
    Replacement::draft_throws(0x140859500, "content_140859500", fn_140859500 as *const (), Throws::Caught), // sub_140859500
    Replacement::draft_throws(0x1408595d0, "content_1408595d0", fn_1408595d0 as *const (), Throws::Caught), // sub_1408595d0
    Replacement::draft_throws(0x1408596a0, "content_1408596a0", fn_1408596a0 as *const (), Throws::Caught), // sub_1408596a0
    Replacement::draft(0x140859770, "content_140859770", tpl_8129::<0x14387d6e8, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140859770
    Replacement::draft(0x1408597f0, "content_1408597f0", tpl_9566::<0x14387d6b8, 0x14387d6e8, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x50, 0x120> as *const ()), // sub_1408597f0
    Replacement::draft(0x1408598a0, "content_1408598a0", tpl_7857::<0x14387d6e8, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408598a0
    Replacement::draft_throws(0x140859920, "content_140859920", fn_140859920 as *const (), Throws::Discard), // sub_140859920
    Replacement::draft_throws(0x1408599d0, "content_1408599d0", fn_1408599d0 as *const (), Throws::Discard), // sub_1408599d0
    Replacement::draft(0x140859af0, "content_140859af0", tpl_9564::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326e2a8, 0xb, 0xc> as *const ()), // sub_140859af0
    Replacement::draft(0x140859b90, "content_140859b90", tpl_9565::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326e2a8, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_140859b90
    Replacement::draft(0x140859ce0, "content_140859ce0", tpl_9569::<0x14387d600, 0x14243b0b0, 0x1408589c0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140859ce0
    Replacement::draft_throws(0x140859d90, "content_140859d90", tpl_7851::<0x14387d5d4, 0x14387d600, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x50, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140859d90
    Replacement::draft(0x140859e20, "content_140859e20", tpl_7857::<0x14387d600, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140859e20
    Replacement::draft_throws(0x140859ea0, "content_140859ea0", tpl_7972::<0x14387d5d4, 0x14387d600, 0x1401e9b60, 0x140858b80, 0x328, 0x338, 0x120, 0x50, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140859ea0
    Replacement::draft_throws(0x140859f30, "content_140859f30", fn_140859f30 as *const (), Throws::Caught), // sub_140859f30
    Replacement::draft_throws(0x14085a000, "content_14085a000", fn_14085a000 as *const (), Throws::Caught), // sub_14085a000
    Replacement::draft_throws(0x14085a0d0, "content_14085a0d0", fn_14085a0d0 as *const (), Throws::Caught), // sub_14085a0d0
    Replacement::draft(0x14085a1a0, "content_14085a1a0", tpl_7855::<0x14387d600, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085a1a0
    Replacement::draft(0x14085a220, "content_14085a220", tpl_9566::<0x14387d5d4, 0x14387d600, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x50, 0xe8> as *const ()), // sub_14085a220
    Replacement::draft(0x14085a2d0, "content_14085a2d0", tpl_7857::<0x14387d600, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085a2d0
    Replacement::draft_throws(0x14085a350, "content_14085a350", tpl_9570::<0x14387d5d4, 0x14387d600, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x50, 0x120> as *const (), Throws::Discard), // sub_14085a350
    Replacement::draft_throws(0x14085a400, "content_14085a400", tpl_9571::<0x14387d600, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x1402c4e50> as *const (), Throws::Discard), // sub_14085a400
    Replacement::draft(0x14085a520, "content_14085a520", tpl_9564::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326b4e0, 0xb, 0xc> as *const ()), // sub_14085a520
    Replacement::draft(0x14085a5c0, "content_14085a5c0", tpl_9565::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326b4e0, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_14085a5c0
    Replacement::draft(0x14085a710, "content_14085a710", tpl_9569::<0x14387d688, 0x14243b0b0, 0x1408589c0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085a710
    Replacement::draft_throws(0x14085a7c0, "content_14085a7c0", tpl_7851::<0x14387d684, 0x14387d688, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x50, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14085a7c0
    Replacement::draft(0x14085a850, "content_14085a850", tpl_7857::<0x14387d688, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085a850
    Replacement::draft_throws(0x14085a8d0, "content_14085a8d0", tpl_7972::<0x14387d684, 0x14387d688, 0x1401e9b60, 0x140858b80, 0x328, 0x338, 0x120, 0x50, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14085a8d0
    Replacement::draft_throws(0x14085a960, "content_14085a960", fn_14085a960 as *const (), Throws::Caught), // sub_14085a960
    Replacement::draft_throws(0x14085aa30, "content_14085aa30", fn_14085aa30 as *const (), Throws::Caught), // sub_14085aa30
    Replacement::draft_throws(0x14085ab00, "content_14085ab00", fn_14085ab00 as *const (), Throws::Caught), // sub_14085ab00
    Replacement::draft(0x14085abd0, "content_14085abd0", tpl_7855::<0x14387d688, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085abd0
    Replacement::draft(0x14085ac50, "content_14085ac50", tpl_9566::<0x14387d684, 0x14387d688, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x50, 0xe0> as *const ()), // sub_14085ac50
    Replacement::draft(0x14085ad00, "content_14085ad00", tpl_7857::<0x14387d688, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085ad00
    Replacement::draft_throws(0x14085ad80, "content_14085ad80", tpl_9570::<0x14387d684, 0x14387d688, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x50, 0x120> as *const (), Throws::Discard), // sub_14085ad80
    Replacement::draft_throws(0x14085ae30, "content_14085ae30", tpl_9571::<0x14387d688, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14020f120> as *const (), Throws::Discard), // sub_14085ae30
    Replacement::draft(0x14085af50, "content_14085af50", tpl_9564::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326ce30, 0xb, 0xc> as *const ()), // sub_14085af50
    Replacement::draft(0x14085aff0, "content_14085aff0", tpl_9565::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326ce30, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_14085aff0
    Replacement::draft(0x14085b140, "content_14085b140", fn_14085b140 as *const ()), // sub_14085b140
    Replacement::draft_throws(0x14085b1f0, "content_14085b1f0", tpl_7851::<0x14387d6d0, 0x14387d5d8, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x50, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14085b1f0
    Replacement::draft(0x14085b280, "content_14085b280", tpl_7857::<0x14387d5d8, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085b280
    Replacement::draft_throws(0x14085b300, "content_14085b300", tpl_7853::<0x14387d6d0, 0x14387d5d8, 0x1401e9b60, 0x140858b80, 0x328, 0x338, 0x120, 0x50, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14085b300
    Replacement::draft_throws(0x14085b3a0, "content_14085b3a0", fn_14085b3a0 as *const (), Throws::Caught), // sub_14085b3a0
    Replacement::draft_throws(0x14085b470, "content_14085b470", fn_14085b470 as *const (), Throws::Caught), // sub_14085b470
    Replacement::draft_throws(0x14085b540, "content_14085b540", fn_14085b540 as *const (), Throws::Caught), // sub_14085b540
    Replacement::draft(0x14085b610, "content_14085b610", tpl_7855::<0x14387d5d8, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085b610
    Replacement::draft(0x14085b690, "content_14085b690", tpl_9566::<0x14387d6d0, 0x14387d5d8, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x50, 0xd8> as *const ()), // sub_14085b690
    Replacement::draft(0x14085b740, "content_14085b740", tpl_7857::<0x14387d5d8, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085b740
    Replacement::draft_throws(0x14085b7c0, "content_14085b7c0", fn_14085b7c0 as *const (), Throws::Discard), // sub_14085b7c0
    Replacement::draft_throws(0x14085b870, "content_14085b870", fn_14085b870 as *const (), Throws::Discard), // sub_14085b870
    Replacement::draft(0x14085b990, "content_14085b990", tpl_9575::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326ccc0, 0xb, 0xc, 0x50> as *const ()), // sub_14085b990
    Replacement::draft(0x14085ba30, "content_14085ba30", fn_14085ba30 as *const ()), // sub_14085ba30
    Replacement::draft(0x14085bb50, "content_14085bb50", fn_14085bb50 as *const ()), // sub_14085bb50
    Replacement::draft_throws(0x14085bc00, "content_14085bc00", fn_14085bc00 as *const (), Throws::Discard), // sub_14085bc00
    Replacement::draft(0x14085bcc0, "content_14085bcc0", tpl_9579::<0x14387d660, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x38, 0x54, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085bcc0
    Replacement::draft_throws(0x14085bd60, "content_14085bd60", fn_14085bd60 as *const (), Throws::Discard), // sub_14085bd60
    Replacement::draft_throws(0x14085be10, "content_14085be10", fn_14085be10 as *const (), Throws::Caught), // sub_14085be10
    Replacement::draft_throws(0x14085c030, "content_14085c030", fn_14085c030 as *const (), Throws::Caught), // sub_14085c030
    Replacement::draft_throws(0x14085c220, "content_14085c220", fn_14085c220 as *const (), Throws::Caught), // sub_14085c220
    Replacement::draft(0x14085c400, "content_14085c400", fn_14085c400 as *const ()), // sub_14085c400
    Replacement::draft(0x14085c490, "content_14085c490", fn_14085c490 as *const ()), // sub_14085c490
    Replacement::draft(0x14085c560, "content_14085c560", tpl_9579::<0x14387d660, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xd8, 0x54, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085c560
    Replacement::draft_throws(0x14085c600, "content_14085c600", tpl_9586::<0x14387d628, 0x14387d660, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x50, 0x120> as *const (), Throws::Discard), // sub_14085c600
    Replacement::draft_throws(0x14085c6b0, "content_14085c6b0", fn_14085c6b0 as *const (), Throws::Discard), // sub_14085c6b0
    Replacement::draft(0x14085c7d0, "content_14085c7d0", fn_14085c7d0 as *const ()), // sub_14085c7d0
    Replacement::draft(0x14085c870, "content_14085c870", fn_14085c870 as *const ()), // sub_14085c870
    Replacement::draft(0x14085c990, "content_14085c990", fn_14085c990 as *const ()), // sub_14085c990
    Replacement::draft_throws(0x14085ca40, "content_14085ca40", fn_14085ca40 as *const (), Throws::Discard), // sub_14085ca40
    Replacement::draft(0x14085caf0, "content_14085caf0", tpl_9592::<0x14387d638, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x38, 0x54, 0x58, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085caf0
    Replacement::draft_throws(0x14085cb90, "content_14085cb90", fn_14085cb90 as *const (), Throws::Discard), // sub_14085cb90
    Replacement::draft_throws(0x14085cc30, "content_14085cc30", fn_14085cc30 as *const (), Throws::Caught), // sub_14085cc30
    Replacement::draft_throws(0x14085ce30, "content_14085ce30", fn_14085ce30 as *const (), Throws::Caught), // sub_14085ce30
    Replacement::draft_throws(0x14085d000, "content_14085d000", fn_14085d000 as *const (), Throws::Caught), // sub_14085d000
    Replacement::draft(0x14085d1d0, "content_14085d1d0", fn_14085d1d0 as *const ()), // sub_14085d1d0
    Replacement::draft(0x14085d250, "content_14085d250", fn_14085d250 as *const ()), // sub_14085d250
    Replacement::draft(0x14085d330, "content_14085d330", tpl_9592::<0x14387d638, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xd8, 0x54, 0x58, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085d330
    Replacement::draft_throws(0x14085d3e0, "content_14085d3e0", fn_14085d3e0 as *const (), Throws::Discard), // sub_14085d3e0
    Replacement::draft_throws(0x14085d4a0, "content_14085d4a0", fn_14085d4a0 as *const (), Throws::Discard), // sub_14085d4a0
    Replacement::draft(0x14085d5d0, "content_14085d5d0", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326e360, 0xb, 0xc, 0x50> as *const ()), // sub_14085d5d0
    Replacement::draft(0x14085d670, "content_14085d670", fn_14085d670 as *const ()), // sub_14085d670
    Replacement::draft(0x14085d790, "content_14085d790", fn_14085d790 as *const ()), // sub_14085d790
    Replacement::draft_throws(0x14085d840, "content_14085d840", fn_14085d840 as *const (), Throws::Discard), // sub_14085d840
    Replacement::draft(0x14085d900, "content_14085d900", tpl_9605::<0x14387d630, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x38, 0x54, 0x58, 0x5c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_14085d900
    Replacement::draft_throws(0x14085d9b0, "content_14085d9b0", fn_14085d9b0 as *const (), Throws::Discard), // sub_14085d9b0
    Replacement::draft_throws(0x14085da50, "content_14085da50", fn_14085da50 as *const (), Throws::Caught), // sub_14085da50
    Replacement::draft_throws(0x14085dc50, "content_14085dc50", fn_14085dc50 as *const (), Throws::Caught), // sub_14085dc50
    Replacement::draft_throws(0x14085de20, "content_14085de20", fn_14085de20 as *const (), Throws::Caught), // sub_14085de20
    Replacement::draft(0x14085dff0, "content_14085dff0", fn_14085dff0 as *const ()), // sub_14085dff0
    Replacement::draft(0x14085e070, "content_14085e070", fn_14085e070 as *const ()), // sub_14085e070
    Replacement::draft(0x14085e160, "content_14085e160", tpl_9605::<0x14387d630, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xd8, 0x54, 0x58, 0x5c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_14085e160
    Replacement::draft_throws(0x14085e220, "content_14085e220", fn_14085e220 as *const (), Throws::Discard), // sub_14085e220
    Replacement::draft_throws(0x14085e2d0, "content_14085e2d0", fn_14085e2d0 as *const (), Throws::Discard), // sub_14085e2d0
    Replacement::draft(0x14085e410, "content_14085e410", tpl_9575::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326bf10, 0xb, 0xc, 0x50> as *const ()), // sub_14085e410
    Replacement::draft(0x14085e4b0, "content_14085e4b0", tpl_9614::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326bf10, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_14085e4b0
    Replacement::draft(0x14085e560, "content_14085e560", tpl_9615::<0x14387d6f8, 0x14243b0b0, 0x14387d5c8, 0x8, 0x143881f70, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x50> as *const ()), // sub_14085e560
    Replacement::draft(0x14085e640, "content_14085e640", tpl_6798::<0x14387d6f8, 0x14243b0b0, 0x1408589c0, 0x50, 0x140559ea0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085e640
    Replacement::draft_throws(0x14085e6c0, "content_14085e6c0", tpl_6799::<0x14387d6a8, 0x14387d6f8, 0x1401e9fa0, 0x140858a70, 0x50, 0x140557390, 0x1401e9e70> as *const (), Throws::Discard), // sub_14085e6c0
    Replacement::draft(0x14085e730, "content_14085e730", tpl_6798::<0x14387d6f8, 0x14243b0b0, 0x140858b10, 0x50, 0x14055a030, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085e730
    Replacement::draft_throws(0x14085e7b0, "content_14085e7b0", tpl_6799::<0x14387d6a8, 0x14387d6f8, 0x1401e9b60, 0x140858b80, 0x50, 0x1405575f0, 0x1401e9a20> as *const (), Throws::Discard), // sub_14085e7b0
    Replacement::draft_throws(0x14085e820, "content_14085e820", fn_14085e820 as *const (), Throws::Caught), // sub_14085e820
    Replacement::draft_throws(0x14085e8e0, "content_14085e8e0", fn_14085e8e0 as *const (), Throws::Caught), // sub_14085e8e0
    Replacement::draft_throws(0x14085e9a0, "content_14085e9a0", fn_14085e9a0 as *const (), Throws::Caught), // sub_14085e9a0
    Replacement::draft(0x14085ea60, "content_14085ea60", tpl_6803::<0x14387d6f8, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x140559fc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085ea60
    Replacement::draft(0x14085eae0, "content_14085eae0", tpl_9616::<0x14387d6a8, 0x14387d6f8, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x143881f78, 0x50, 0x143881f70> as *const ()), // sub_14085eae0
    Replacement::draft(0x14085ebc0, "content_14085ebc0", tpl_6798::<0x14387d6f8, 0x14243b0b0, 0x140858f40, 0x50, 0x14055a1a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085ebc0
    Replacement::draft_throws(0x14085ec40, "content_14085ec40", tpl_9617::<0x14387d6a8, 0x14387d6f8, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x50, 0x14055a210> as *const (), Throws::Discard), // sub_14085ec40
    Replacement::draft_throws(0x14085ece0, "content_14085ece0", tpl_9618::<0x14387d6f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x140558dc0> as *const (), Throws::Discard), // sub_14085ece0
    Replacement::draft(0x14085ede0, "content_14085ede0", tpl_9575::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326e690, 0xb, 0xc, 0x50> as *const ()), // sub_14085ede0
    Replacement::draft(0x14085ee80, "content_14085ee80", tpl_9614::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326e690, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_14085ee80
    Replacement::draft(0x14085efd0, "content_14085efd0", tpl_9619::<0x14387d690, 0x14243b0b0, 0x1408589c0, 0x50, 0x14028a610, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085efd0
    Replacement::draft_throws(0x14085f050, "content_14085f050", fn_14085f050 as *const (), Throws::Discard), // sub_14085f050
    Replacement::draft(0x14085f0e0, "content_14085f0e0", fn_14085f0e0 as *const ()), // sub_14085f0e0
    Replacement::draft_throws(0x14085f160, "content_14085f160", fn_14085f160 as *const (), Throws::Discard), // sub_14085f160
    Replacement::draft_throws(0x14085f200, "content_14085f200", fn_14085f200 as *const (), Throws::Caught), // sub_14085f200
    Replacement::draft_throws(0x14085f2c0, "content_14085f2c0", fn_14085f2c0 as *const (), Throws::Caught), // sub_14085f2c0
    Replacement::draft_throws(0x14085f380, "content_14085f380", fn_14085f380 as *const (), Throws::Caught), // sub_14085f380
    Replacement::draft(0x14085f440, "content_14085f440", fn_14085f440 as *const ()), // sub_14085f440
    Replacement::draft(0x14085f4c0, "content_14085f4c0", tpl_9566::<0x14387d5f8, 0x14387d690, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x50, 0x88> as *const ()), // sub_14085f4c0
    Replacement::draft(0x14085f570, "content_14085f570", tpl_7857::<0x14387d690, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085f570
    Replacement::draft_throws(0x14085f5f0, "content_14085f5f0", tpl_9586::<0x14387d5f8, 0x14387d690, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x50, 0x120> as *const (), Throws::Discard), // sub_14085f5f0
    Replacement::draft_throws(0x14085f6a0, "content_14085f6a0", tpl_9618::<0x14387d690, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x1402bf9b0> as *const (), Throws::Discard), // sub_14085f6a0
    Replacement::draft(0x14085f7a0, "content_14085f7a0", tpl_9575::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326bc98, 0xb, 0xc, 0x50> as *const ()), // sub_14085f7a0
    Replacement::draft(0x14085f840, "content_14085f840", tpl_9614::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326bc98, 0xfffffffffffffffc, 0x4, 0x58, 0xffffffffffffffa4, 0xffffffffffffffa7> as *const ()), // sub_14085f840
    Replacement::draft(0x14085f8f0, "content_14085f8f0", tpl_9615::<0x14387d618, 0x14243b0b0, 0x14387d5c8, 0x8, 0x14387d6e0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x50> as *const ()), // sub_14085f8f0
    Replacement::draft(0x14085f9d0, "content_14085f9d0", tpl_6798::<0x14387d618, 0x14243b0b0, 0x1408589c0, 0x50, 0x140858360, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085f9d0
    Replacement::draft_throws(0x14085fa50, "content_14085fa50", tpl_8772::<0x14387d5e4, 0x14387d618, 0x1401e9fa0, 0x140858a70, 0x50, 0x14086a120, 0x1401e9e70> as *const (), Throws::Discard), // sub_14085fa50
    Replacement::draft(0x14085fac0, "content_14085fac0", tpl_6798::<0x14387d618, 0x14243b0b0, 0x140858b10, 0x50, 0x140858450, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085fac0
    Replacement::draft_throws(0x14085fb40, "content_14085fb40", tpl_8772::<0x14387d5e4, 0x14387d618, 0x1401e9b60, 0x140858b80, 0x50, 0x14086a380, 0x1401e9a20> as *const (), Throws::Discard), // sub_14085fb40
    Replacement::draft_throws(0x14085fbb0, "content_14085fbb0", fn_14085fbb0 as *const (), Throws::Caught), // sub_14085fbb0
    Replacement::draft_throws(0x14085fc70, "content_14085fc70", fn_14085fc70 as *const (), Throws::Caught), // sub_14085fc70
    Replacement::draft_throws(0x14085fd30, "content_14085fd30", fn_14085fd30 as *const (), Throws::Caught), // sub_14085fd30
    Replacement::draft(0x14085fdf0, "content_14085fdf0", tpl_6803::<0x14387d618, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x140858760, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085fdf0
    Replacement::draft(0x14085fe70, "content_14085fe70", tpl_9616::<0x14387d5e4, 0x14387d618, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x14387d610, 0x50, 0x14387d6e0> as *const ()), // sub_14085fe70
    Replacement::draft(0x14085ff50, "content_14085ff50", tpl_6798::<0x14387d618, 0x14243b0b0, 0x140858f40, 0x50, 0x140858840, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14085ff50
    Replacement::draft_throws(0x14085ffd0, "content_14085ffd0", tpl_9617::<0x14387d5e4, 0x14387d618, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x50, 0x1408588b0> as *const (), Throws::Discard), // sub_14085ffd0
    Replacement::draft_throws(0x140860070, "content_140860070", fn_140860070 as *const (), Throws::Caught), // sub_140860070
    Replacement::draft(0x140860210, "content_140860210", fn_140860210 as *const ()), // sub_140860210
    Replacement::draft(0x1408602d0, "content_1408602d0", fn_1408602d0 as *const ()), // sub_1408602d0
    Replacement::draft(0x140860440, "content_140860440", fn_140860440 as *const ()), // sub_140860440
    Replacement::draft_throws(0x140860500, "content_140860500", tpl_9629::<0x14387d5d0, 0x14387d698, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x50, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140860500
    Replacement::draft(0x1408605b0, "content_1408605b0", tpl_7857::<0x14387d698, 0x14243b0b0, 0x140858b10, 0x338, 0x50, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408605b0
    Replacement::draft_throws(0x140860630, "content_140860630", tpl_9630::<0x14387d5d0, 0x14387d698, 0x1401e9b60, 0x140858b80, 0x328, 0x50, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140860630
    Replacement::draft_throws(0x1408606e0, "content_1408606e0", fn_1408606e0 as *const (), Throws::Caught), // sub_1408606e0
    Replacement::draft_throws(0x1408607a0, "content_1408607a0", fn_1408607a0 as *const (), Throws::Caught), // sub_1408607a0
    Replacement::draft_throws(0x140860860, "content_140860860", fn_140860860 as *const (), Throws::Caught), // sub_140860860
    Replacement::draft(0x140860920, "content_140860920", fn_140860920 as *const ()), // sub_140860920
    Replacement::draft(0x1408609c0, "content_1408609c0", fn_1408609c0 as *const ()), // sub_1408609c0
    Replacement::draft(0x140860a70, "content_140860a70", tpl_7857::<0x14387d698, 0x14243b0b0, 0x140858f40, 0x338, 0x50, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140860a70
    Replacement::draft_throws(0x140860af0, "content_140860af0", fn_140860af0 as *const (), Throws::Discard), // sub_140860af0
    Replacement::draft_throws(0x140860b90, "content_140860b90", fn_140860b90 as *const (), Throws::Discard), // sub_140860b90
    Replacement::draft(0x140860ca0, "content_140860ca0", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326ae30, 0xb, 0xc, 0x50> as *const ()), // sub_140860ca0
    Replacement::draft(0x140860d40, "content_140860d40", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326ae30, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_140860d40
    Replacement::draft(0x140860e90, "content_140860e90", tpl_9619::<0x14387d640, 0x14243b0b0, 0x1408589c0, 0x50, 0x1402c4c00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140860e90
    Replacement::draft_throws(0x140860f10, "content_140860f10", tpl_9636::<0x14387d6d8, 0x14387d640, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x14028e490, 0x340, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_140860f10
    Replacement::draft(0x140860fc0, "content_140860fc0", fn_140860fc0 as *const ()), // sub_140860fc0
    Replacement::draft_throws(0x140861080, "content_140861080", tpl_6799::<0x14387d6d8, 0x14387d640, 0x1401e9b60, 0x140858b80, 0x50, 0x1402c4ca0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140861080
    Replacement::draft_throws(0x1408610f0, "content_1408610f0", fn_1408610f0 as *const (), Throws::Caught), // sub_1408610f0
    Replacement::draft_throws(0x1408611b0, "content_1408611b0", fn_1408611b0 as *const (), Throws::Caught), // sub_1408611b0
    Replacement::draft_throws(0x140861270, "content_140861270", fn_140861270 as *const (), Throws::Caught), // sub_140861270
    Replacement::draft(0x140861330, "content_140861330", fn_140861330 as *const ()), // sub_140861330
    Replacement::draft(0x1408613e0, "content_1408613e0", fn_1408613e0 as *const ()), // sub_1408613e0
    Replacement::draft(0x1408614b0, "content_1408614b0", fn_1408614b0 as *const ()), // sub_1408614b0
    Replacement::draft_throws(0x140861560, "content_140861560", fn_140861560 as *const (), Throws::Discard), // sub_140861560
    Replacement::draft_throws(0x140861650, "content_140861650", tpl_9618::<0x14387d640, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x1402c4e10> as *const (), Throws::Discard), // sub_140861650
    Replacement::draft(0x140861750, "content_140861750", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326dd50, 0xb, 0xc, 0x50> as *const ()), // sub_140861750
    Replacement::draft(0x1408617f0, "content_1408617f0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326dd50, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_1408617f0
    Replacement::draft(0x140861940, "content_140861940", tpl_9619::<0x14387d5f0, 0x14243b0b0, 0x1408589c0, 0x50, 0x140805af0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140861940
    Replacement::draft_throws(0x1408619c0, "content_1408619c0", tpl_9641::<0x14387d5e0, 0x14387d5f0, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x1406dcc50, 0x340, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408619c0
    Replacement::draft(0x140861a70, "content_140861a70", tpl_9642::<0x14387d5f0, 0x14243b0b0, 0x140858b10, 0x338, 0x48, 0x50, 0x1406dcc50, 0x340, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140861a70
    Replacement::draft_throws(0x140861b30, "content_140861b30", tpl_6799::<0x14387d5e0, 0x14387d5f0, 0x1401e9b60, 0x140858b80, 0x50, 0x1406d7b60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140861b30
    Replacement::draft(0x140861ba0, "content_140861ba0", fn_140861ba0 as *const ()), // sub_140861ba0
    Replacement::draft_throws(0x140861c10, "content_140861c10", fn_140861c10 as *const (), Throws::Caught), // sub_140861c10
    Replacement::draft_throws(0x140861cd0, "content_140861cd0", fn_140861cd0 as *const (), Throws::Caught), // sub_140861cd0
    Replacement::draft_throws(0x140861d90, "content_140861d90", fn_140861d90 as *const (), Throws::Caught), // sub_140861d90
    Replacement::draft(0x140861e50, "content_140861e50", tpl_9644::<0x14387d5f0, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x58, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140861e50
    Replacement::draft(0x140861f00, "content_140861f00", tpl_9645::<0x14387d5e0, 0x14387d5f0, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x58, 0xe8, 0x340> as *const ()), // sub_140861f00
    Replacement::draft(0x140861fe0, "content_140861fe0", tpl_9646::<0x14387d5f0, 0x14243b0b0, 0x140858f40, 0x58, 0x338, 0xe8, 0x340, 0x50, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140861fe0
    Replacement::draft_throws(0x140862090, "content_140862090", tpl_68314::<0x14387d5e0, 0x14387d5f0, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x1424b4cb0, 0x1401e9590, 0x120, 0x340, 0x14183acbc> as *const (), Throws::Discard), // sub_140862090
    Replacement::draft_throws(0x140862180, "content_140862180", tpl_9618::<0x14387d5f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x140806e80> as *const (), Throws::Discard), // sub_140862180
    Replacement::draft(0x140862280, "content_140862280", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326aee8, 0xb, 0xc, 0x50> as *const ()), // sub_140862280
    Replacement::draft(0x140862320, "content_140862320", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326aee8, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_140862320
    Replacement::draft(0x140862470, "content_140862470", tpl_9619::<0x14387d5e8, 0x14243b0b0, 0x1408589c0, 0x50, 0x140805af0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140862470
    Replacement::draft_throws(0x1408624f0, "content_1408624f0", tpl_9641::<0x14387d5c0, 0x14387d5e8, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x1406dcc50, 0x340, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408624f0
    Replacement::draft(0x1408625a0, "content_1408625a0", tpl_9642::<0x14387d5e8, 0x14243b0b0, 0x140858b10, 0x338, 0x48, 0x50, 0x1406dcc50, 0x340, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1408625a0
    Replacement::draft_throws(0x140862660, "content_140862660", tpl_6799::<0x14387d5c0, 0x14387d5e8, 0x1401e9b60, 0x140858b80, 0x50, 0x1406d7b60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140862660
    Replacement::draft_throws(0x1408626d0, "content_1408626d0", fn_1408626d0 as *const (), Throws::Caught), // sub_1408626d0
    Replacement::draft_throws(0x140862790, "content_140862790", fn_140862790 as *const (), Throws::Caught), // sub_140862790
    Replacement::draft_throws(0x140862850, "content_140862850", fn_140862850 as *const (), Throws::Caught), // sub_140862850
    Replacement::draft(0x140862910, "content_140862910", tpl_9644::<0x14387d5e8, 0x14243b0b0, 0x140858e30, 0x338, 0x50, 0x58, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140862910
    Replacement::draft(0x1408629c0, "content_1408629c0", tpl_9645::<0x14387d5c0, 0x14387d5e8, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x58, 0xe8, 0x340> as *const ()), // sub_1408629c0
    Replacement::draft(0x140862aa0, "content_140862aa0", tpl_9646::<0x14387d5e8, 0x14243b0b0, 0x140858f40, 0x58, 0x338, 0xe8, 0x340, 0x50, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140862aa0
    Replacement::draft_throws(0x140862b50, "content_140862b50", tpl_68314::<0x14387d5c0, 0x14387d5e8, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x1424b4cb0, 0x1401e9590, 0x120, 0x340, 0x14183acbc> as *const (), Throws::Discard), // sub_140862b50
    Replacement::draft_throws(0x140862c40, "content_140862c40", tpl_9618::<0x14387d5e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14086ad60> as *const (), Throws::Discard), // sub_140862c40
    Replacement::draft(0x140862d40, "content_140862d40", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326de08, 0xb, 0xc, 0x50> as *const ()), // sub_140862d40
    Replacement::draft(0x140862de0, "content_140862de0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326de08, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_140862de0
    Replacement::draft(0x140862f30, "content_140862f30", tpl_9619::<0x14387d670, 0x14243b0b0, 0x1408589c0, 0x50, 0x1406d7a90, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140862f30
    Replacement::draft_throws(0x140862fb0, "content_140862fb0", tpl_9641::<0x14387d678, 0x14387d670, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x1406dcc50, 0x340, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_140862fb0
    Replacement::draft(0x140863060, "content_140863060", tpl_9642::<0x14387d670, 0x14243b0b0, 0x140858b10, 0x338, 0x48, 0x50, 0x1406dcc50, 0x340, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140863060
    Replacement::draft_throws(0x140863120, "content_140863120", tpl_6799::<0x14387d678, 0x14387d670, 0x1401e9b60, 0x140858b80, 0x50, 0x1406d7b60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140863120
    Replacement::draft_throws(0x140863190, "content_140863190", fn_140863190 as *const (), Throws::Caught), // sub_140863190
    Replacement::draft_throws(0x140863250, "content_140863250", fn_140863250 as *const (), Throws::Caught), // sub_140863250
    Replacement::draft_throws(0x140863310, "content_140863310", fn_140863310 as *const (), Throws::Caught), // sub_140863310
    Replacement::draft(0x1408633d0, "content_1408633d0", fn_1408633d0 as *const ()), // sub_1408633d0
    Replacement::draft(0x140863480, "content_140863480", tpl_9645::<0x14387d678, 0x14387d670, 0x1401e9970, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0xc0, 0x1401e9860, 0x58, 0xe8, 0x340> as *const ()), // sub_140863480
    Replacement::draft(0x140863560, "content_140863560", tpl_9646::<0x14387d670, 0x14243b0b0, 0x140858f40, 0x58, 0x338, 0xe8, 0x340, 0x50, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140863560
    Replacement::draft_throws(0x140863610, "content_140863610", tpl_68314::<0x14387d678, 0x14387d670, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x1424b4cb0, 0x1401e9590, 0x120, 0x340, 0x14183acbc> as *const (), Throws::Discard), // sub_140863610
    Replacement::draft_throws(0x140863700, "content_140863700", tpl_9618::<0x14387d670, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x1406d7c40> as *const (), Throws::Discard), // sub_140863700
    Replacement::draft(0x140863800, "content_140863800", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326dc98, 0xb, 0xc, 0x50> as *const ()), // sub_140863800
    Replacement::draft(0x1408638a0, "content_1408638a0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326dc98, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_1408638a0
    Replacement::draft(0x1408639f0, "content_1408639f0", tpl_9619::<0x14387d650, 0x14243b0b0, 0x1408589c0, 0x50, 0x1406d93e0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408639f0
    Replacement::draft_throws(0x140863a70, "content_140863a70", fn_140863a70 as *const (), Throws::Discard), // sub_140863a70
    Replacement::draft(0x140863b20, "content_140863b20", fn_140863b20 as *const ()), // sub_140863b20
    Replacement::draft_throws(0x140863be0, "content_140863be0", tpl_6799::<0x14387d648, 0x14387d650, 0x1401e9b60, 0x140858b80, 0x50, 0x1406d94b0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140863be0
    Replacement::draft(0x140863c50, "content_140863c50", fn_140863c50 as *const ()), // sub_140863c50
    Replacement::draft_throws(0x140863cc0, "content_140863cc0", fn_140863cc0 as *const (), Throws::Caught), // sub_140863cc0
    Replacement::draft_throws(0x140863d80, "content_140863d80", fn_140863d80 as *const (), Throws::Caught), // sub_140863d80
    Replacement::draft_throws(0x140863e40, "content_140863e40", fn_140863e40 as *const (), Throws::Caught), // sub_140863e40
    Replacement::draft(0x140863f00, "content_140863f00", fn_140863f00 as *const ()), // sub_140863f00
    Replacement::draft(0x140863fd0, "content_140863fd0", fn_140863fd0 as *const ()), // sub_140863fd0
    Replacement::draft(0x1408640b0, "content_1408640b0", fn_1408640b0 as *const ()), // sub_1408640b0
    Replacement::draft_throws(0x140864160, "content_140864160", fn_140864160 as *const (), Throws::Discard), // sub_140864160
    Replacement::draft_throws(0x140864250, "content_140864250", tpl_9618::<0x14387d650, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x1406d9590> as *const (), Throws::Discard), // sub_140864250
    Replacement::draft(0x140864350, "content_140864350", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326df78, 0xb, 0xc, 0x50> as *const ()), // sub_140864350
    Replacement::draft(0x1408643f0, "content_1408643f0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326df78, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_1408643f0
    Replacement::draft(0x140864540, "content_140864540", tpl_9619::<0x14387d6b0, 0x14243b0b0, 0x1408589c0, 0x50, 0x14086adb0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140864540
    Replacement::draft_throws(0x1408645c0, "content_1408645c0", fn_1408645c0 as *const (), Throws::Discard), // sub_1408645c0
    Replacement::draft(0x140864680, "content_140864680", fn_140864680 as *const ()), // sub_140864680
    Replacement::draft_throws(0x140864740, "content_140864740", tpl_6799::<0x14387d668, 0x14387d6b0, 0x1401e9b60, 0x140858b80, 0x50, 0x14086ae80, 0x1401e9a20> as *const (), Throws::Discard), // sub_140864740
    Replacement::draft(0x1408647b0, "content_1408647b0", fn_1408647b0 as *const ()), // sub_1408647b0
    Replacement::draft_throws(0x140864820, "content_140864820", fn_140864820 as *const (), Throws::Caught), // sub_140864820
    Replacement::draft_throws(0x1408648e0, "content_1408648e0", fn_1408648e0 as *const (), Throws::Caught), // sub_1408648e0
    Replacement::draft_throws(0x1408649a0, "content_1408649a0", fn_1408649a0 as *const (), Throws::Caught), // sub_1408649a0
    Replacement::draft(0x140864a60, "content_140864a60", fn_140864a60 as *const ()), // sub_140864a60
    Replacement::draft(0x140864b40, "content_140864b40", fn_140864b40 as *const ()), // sub_140864b40
    Replacement::draft(0x140864c20, "content_140864c20", fn_140864c20 as *const ()), // sub_140864c20
    Replacement::draft_throws(0x140864cd0, "content_140864cd0", fn_140864cd0 as *const (), Throws::Discard), // sub_140864cd0
    Replacement::draft_throws(0x140864dc0, "content_140864dc0", tpl_9618::<0x14387d6b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14086af60> as *const (), Throws::Discard), // sub_140864dc0
    Replacement::draft(0x140864ec0, "content_140864ec0", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326c888, 0xb, 0xc, 0x50> as *const ()), // sub_140864ec0
    Replacement::draft(0x140864f60, "content_140864f60", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326c888, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_140864f60
    Replacement::draft(0x1408650b0, "content_1408650b0", fn_1408650b0 as *const ()), // sub_1408650b0
    Replacement::draft_throws(0x1408651d0, "content_1408651d0", fn_1408651d0 as *const (), Throws::Discard), // sub_1408651d0
    Replacement::draft(0x140865280, "content_140865280", fn_140865280 as *const ()), // sub_140865280
    Replacement::draft_throws(0x140865340, "content_140865340", fn_140865340 as *const (), Throws::Discard), // sub_140865340
    Replacement::draft(0x140865430, "content_140865430", fn_140865430 as *const ()), // sub_140865430
    Replacement::draft_throws(0x1408654a0, "content_1408654a0", fn_1408654a0 as *const (), Throws::Caught), // sub_1408654a0
    Replacement::draft_throws(0x140865560, "content_140865560", fn_140865560 as *const (), Throws::Caught), // sub_140865560
    Replacement::draft_throws(0x140865620, "content_140865620", fn_140865620 as *const (), Throws::Caught), // sub_140865620
    Replacement::draft(0x1408656e0, "content_1408656e0", fn_1408656e0 as *const ()), // sub_1408656e0
    Replacement::draft(0x1408657a0, "content_1408657a0", fn_1408657a0 as *const ()), // sub_1408657a0
    Replacement::draft(0x140865880, "content_140865880", fn_140865880 as *const ()), // sub_140865880
    Replacement::draft_throws(0x140865930, "content_140865930", fn_140865930 as *const (), Throws::Discard), // sub_140865930
    Replacement::draft_throws(0x140865a20, "content_140865a20", tpl_9618::<0x14387d620, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14086afe0> as *const (), Throws::Discard), // sub_140865a20
    Replacement::draft(0x140865b20, "content_140865b20", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326dbe0, 0xb, 0xc, 0x50> as *const ()), // sub_140865b20
    Replacement::draft(0x140865bc0, "content_140865bc0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326dbe0, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_140865bc0
    Replacement::draft(0x140865c70, "content_140865c70", tpl_9668::<0x14387d6c8, 0x14243b0b0, 0x14387d5c8, 0x8, 0xfffffffffffffff8, 0x328, 0x50, 0x58, 0x143881f70, 0xffffffffffffffff> as *const ()), // sub_140865c70
    Replacement::draft(0x140865d80, "content_140865d80", tpl_9619::<0x14387d6c8, 0x14243b0b0, 0x1408589c0, 0x50, 0x140853710, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140865d80
    Replacement::draft_throws(0x140865e00, "content_140865e00", tpl_9669::<0x14387d700, 0x14387d6c8, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x1406a55b0, 0x1408538c0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140865e00
    Replacement::draft(0x140865eb0, "content_140865eb0", tpl_68339::<0x14387d6c8, 0x14243b0b0, 0x140858b10, 0x338, 0x48, 0x50, 0x1406a55b0, 0x14055a030, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140865eb0
    Replacement::draft_throws(0x140865f80, "content_140865f80", tpl_9670::<0x14387d700, 0x14387d6c8, 0x1401e9b60, 0x140858b80, 0x50, 0x328, 0x338, 0x120, 0x1406a55b0, 0x140853b90, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_140865f80
    Replacement::draft(0x140866050, "content_140866050", tpl_9671::<0x50, 0x8, 0x14240aa00, 0x1406a55b0> as *const ()), // sub_140866050
    Replacement::draft_throws(0x1408660d0, "content_1408660d0", fn_1408660d0 as *const (), Throws::Caught), // sub_1408660d0
    Replacement::draft_throws(0x140866190, "content_140866190", fn_140866190 as *const (), Throws::Caught), // sub_140866190
    Replacement::draft_throws(0x140866250, "content_140866250", fn_140866250 as *const (), Throws::Caught), // sub_140866250
    Replacement::draft(0x140866310, "content_140866310", tpl_68340::<0x14387d6c8, 0x14243b0b0, 0x140858e30, 0x338, 0x58, 0x50, 0x140559fc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140866310
    Replacement::draft(0x1408663d0, "content_1408663d0", fn_1408663d0 as *const ()), // sub_1408663d0
    Replacement::draft(0x1408664a0, "content_1408664a0", tpl_68342::<0x14387d6c8, 0x14243b0b0, 0x140858f40, 0x58, 0x338, 0xe8, 0x50, 0x14055a1a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408664a0
    Replacement::draft_throws(0x140866570, "content_140866570", tpl_68343::<0x14387d700, 0x14387d6c8, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x58, 0x120, 0x50, 0x14055a210> as *const (), Throws::Discard), // sub_140866570
    Replacement::draft_throws(0x140866650, "content_140866650", tpl_9618::<0x14387d6c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x1408537e0> as *const (), Throws::Discard), // sub_140866650
    Replacement::draft(0x140866750, "content_140866750", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326e030, 0xb, 0xc, 0x50> as *const ()), // sub_140866750
    Replacement::draft(0x1408667f0, "content_1408667f0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326e030, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_1408667f0
    Replacement::draft(0x140866940, "content_140866940", fn_140866940 as *const ()), // sub_140866940
    Replacement::draft_throws(0x140866ab0, "content_140866ab0", fn_140866ab0 as *const (), Throws::Discard), // sub_140866ab0
    Replacement::draft(0x140866ba0, "content_140866ba0", fn_140866ba0 as *const ()), // sub_140866ba0
    Replacement::draft_throws(0x140866c70, "content_140866c70", fn_140866c70 as *const (), Throws::Discard), // sub_140866c70
    Replacement::draft_throws(0x140866d90, "content_140866d90", fn_140866d90 as *const (), Throws::Caught), // sub_140866d90
    Replacement::draft_throws(0x140866e50, "content_140866e50", fn_140866e50 as *const (), Throws::Caught), // sub_140866e50
    Replacement::draft_throws(0x140866f10, "content_140866f10", fn_140866f10 as *const (), Throws::Caught), // sub_140866f10
    Replacement::draft(0x140866fd0, "content_140866fd0", fn_140866fd0 as *const ()), // sub_140866fd0
    Replacement::draft(0x140867090, "content_140867090", fn_140867090 as *const ()), // sub_140867090
    Replacement::draft(0x140867190, "content_140867190", fn_140867190 as *const ()), // sub_140867190
    Replacement::draft_throws(0x140867260, "content_140867260", fn_140867260 as *const (), Throws::Discard), // sub_140867260
    Replacement::draft_throws(0x140867350, "content_140867350", tpl_9618::<0x14387d6f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14086b060> as *const (), Throws::Discard), // sub_140867350
    Replacement::draft(0x140867450, "content_140867450", tpl_68352::<0x50, 0x58, 0x143251120, 0x14028a830> as *const ()), // sub_140867450
    Replacement::draft(0x1408674d0, "content_1408674d0", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326cee8, 0xb, 0xc, 0x50> as *const ()), // sub_1408674d0
    Replacement::draft(0x140867570, "content_140867570", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326cee8, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_140867570
    Replacement::draft(0x140867620, "content_140867620", tpl_9668::<0x14387d6a0, 0x14243b0b0, 0x14387d5c8, 0x8, 0xfffffffffffffff8, 0x328, 0x50, 0x58, 0x14387d6e0, 0xffffffffffffffff> as *const ()), // sub_140867620
    Replacement::draft(0x140867730, "content_140867730", fn_140867730 as *const ()), // sub_140867730
    Replacement::draft_throws(0x140867820, "content_140867820", tpl_9677::<0x14387d6c0, 0x14387d6a0, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x14086b460, 0x14086ebb0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140867820
    Replacement::draft(0x1408678d0, "content_1408678d0", tpl_68339::<0x14387d6a0, 0x14243b0b0, 0x140858b10, 0x338, 0x48, 0x50, 0x14086b460, 0x140858450, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408678d0
    Replacement::draft_throws(0x1408679a0, "content_1408679a0", tpl_9678::<0x14387d6c0, 0x14387d6a0, 0x1401e9b60, 0x140858b80, 0x50, 0x328, 0x338, 0x120, 0x14086b460, 0x14086ee70, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_1408679a0
    Replacement::draft(0x140867a60, "content_140867a60", tpl_9671::<0x50, 0x8, 0x14240aa00, 0x14086b460> as *const ()), // sub_140867a60
    Replacement::draft_throws(0x140867ae0, "content_140867ae0", fn_140867ae0 as *const (), Throws::Caught), // sub_140867ae0
    Replacement::draft_throws(0x140867ba0, "content_140867ba0", fn_140867ba0 as *const (), Throws::Caught), // sub_140867ba0
    Replacement::draft_throws(0x140867c60, "content_140867c60", fn_140867c60 as *const (), Throws::Caught), // sub_140867c60
    Replacement::draft(0x140867d20, "content_140867d20", tpl_68340::<0x14387d6a0, 0x14243b0b0, 0x140858e30, 0x338, 0x58, 0x50, 0x140858760, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140867d20
    Replacement::draft(0x140867de0, "content_140867de0", fn_140867de0 as *const ()), // sub_140867de0
    Replacement::draft(0x140867f10, "content_140867f10", tpl_68342::<0x14387d6a0, 0x14243b0b0, 0x140858f40, 0x58, 0x338, 0xe8, 0x50, 0x140858840, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140867f10
    Replacement::draft_throws(0x140867fe0, "content_140867fe0", tpl_68343::<0x14387d6c0, 0x14387d6a0, 0x1401e9770, 0x14387d67c, 0x14387d5c8, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590, 0x58, 0x120, 0x50, 0x1408588b0> as *const (), Throws::Discard), // sub_140867fe0
    Replacement::draft_throws(0x1408680c0, "content_1408680c0", tpl_9618::<0x14387d6a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14086b0e0> as *const (), Throws::Discard), // sub_1408680c0
    Replacement::draft(0x1408681c0, "content_1408681c0", tpl_68352::<0x50, 0x58, 0x143251120, 0x14086ad00> as *const ()), // sub_1408681c0
    Replacement::draft(0x140868240, "content_140868240", tpl_9601::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14326dec0, 0xb, 0xc, 0x50> as *const ()), // sub_140868240
    Replacement::draft(0x1408682e0, "content_1408682e0", tpl_9635::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326dec0, 0xfffffffffffffffc, 0x4, 0x60, 0xffffffffffffff9c, 0xffffffffffffff9f> as *const ()), // sub_1408682e0
    Replacement::draft(0x140868430, "content_140868430", tpl_9619::<0x14387d608, 0x14243b0b0, 0x1408589c0, 0x50, 0x14086b130, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140868430
    Replacement::draft_throws(0x1408684b0, "content_1408684b0", tpl_9669::<0x14387d66c, 0x14387d608, 0x1401e9fa0, 0x140858a70, 0x328, 0x338, 0x48, 0x50, 0x14086b510, 0x1402daa00, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408684b0
    Replacement::draft(0x140868560, "content_140868560", fn_140868560 as *const ()), // sub_140868560
    Replacement::draft_throws(0x140868630, "content_140868630", tpl_9670::<0x14387d66c, 0x14387d608, 0x1401e9b60, 0x140858b80, 0x50, 0x328, 0x338, 0x120, 0x14086b510, 0x1402daaa0, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_140868630
    Replacement::draft(0x140868700, "content_140868700", fn_140868700 as *const ()), // sub_140868700
    Replacement::draft_throws(0x140868780, "content_140868780", fn_140868780 as *const (), Throws::Caught), // sub_140868780
    Replacement::draft_throws(0x140868840, "content_140868840", fn_140868840 as *const (), Throws::Caught), // sub_140868840
    Replacement::draft_throws(0x140868900, "content_140868900", fn_140868900 as *const (), Throws::Caught), // sub_140868900
    Replacement::draft(0x1408689c0, "content_1408689c0", fn_1408689c0 as *const ()), // sub_1408689c0
    Replacement::draft(0x140868a60, "content_140868a60", fn_140868a60 as *const ()), // sub_140868a60
    Replacement::draft(0x140868b60, "content_140868b60", fn_140868b60 as *const ()), // sub_140868b60
    Replacement::draft_throws(0x140868c30, "content_140868c30", fn_140868c30 as *const (), Throws::Discard), // sub_140868c30
    Replacement::draft_throws(0x140868d20, "content_140868d20", tpl_9618::<0x14387d608, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d5c8, 0x1426cdcb0, 0x8, 0x14307eae0, 0x1402c4730, 0x143251120, 0x14086b240> as *const (), Throws::Discard), // sub_140868d20
    Replacement::draft(0x140868e20, "content_140868e20", tpl_9681::<0x140874640, 0x14183acc8> as *const ()), // sub_140868e20
    Replacement::draft(0x140868f20, "content_140868f20", tpl_3669::<0x1408746b0, 0x14183acc8> as *const ()), // sub_140868f20
    Replacement::draft_throws(0x140869010, "content_140869010", fn_140869010 as *const (), Throws::Caught), // sub_140869010
    Replacement::draft_throws(0x140869130, "content_140869130", fn_140869130 as *const (), Throws::Caught), // sub_140869130
    Replacement::draft_throws(0x140869260, "content_140869260", fn_140869260 as *const (), Throws::Caught), // sub_140869260
    Replacement::draft(0x1408693d0, "content_1408693d0", tpl_9685::<0x14022a610, 0x8, 0x14240ab10, 0x141834d30> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::InplaceString<64>,r::Allocator<r::InplaceString<64>,0>>>::content::LuaScri
    Replacement::draft(0x140869420, "content_140869420", tpl_9685::<0x1401de830, 0x8, 0x14240ab10, 0x141834d30> as *const ()), // content::LuaScriptProperty<r::DynamicArray<int,r::Allocator<int,0>>>::content::LuaScriptProperty<r::DynamicArray<int,r::
    Replacement::draft(0x140869470, "content_140869470", fn_140869470 as *const ()), // content::LuaScriptProperty<r::InplaceString<64>>::content::LuaScriptProperty<r::InplaceString<64>>::vfunc_0
    Replacement::draft(0x1408694c0, "content_1408694c0", fn_1408694c0 as *const ()), // content::LuaScriptProperty<int>::content::LuaScriptProperty<int>::vfunc_0
    Replacement::draft(0x140869500, "content_140869500", fn_140869500 as *const ()), // content::LuaScriptProperty<m::Vector3Template<float>>::content::LuaScriptProperty<m::Vector3Template<float>>::vfunc_0
    Replacement::draft(0x140869540, "content_140869540", tpl_9689::<0x14240ab10, 0x141834d30> as *const ()), // content::LuaScriptPropertyBase::content::LuaScriptPropertyBase::vfunc_0
    Replacement::draft_throws(0x140869580, "content_140869580", fn_140869580 as *const (), Throws::Caught), // sub_140869580
    Replacement::draft_throws(0x140869880, "content_140869880", fn_140869880 as *const (), Throws::Caught), // sub_140869880
    Replacement::draft_throws(0x140869b80, "content_140869b80", fn_140869b80 as *const (), Throws::Caught), // sub_140869b80
    Replacement::draft(0x140869e80, "content_140869e80", fn_140869e80 as *const ()), // sub_140869e80
    Replacement::draft(0x140869fc0, "content_140869fc0", fn_140869fc0 as *const ()), // sub_140869fc0
    Replacement::draft_throws(0x14086a120, "content_14086a120", fn_14086a120 as *const (), Throws::Caught), // sub_14086a120
    Replacement::draft_throws(0x14086a380, "content_14086a380", fn_14086a380 as *const (), Throws::Caught), // sub_14086a380
    Replacement::draft_throws(0x14086a5e0, "content_14086a5e0", fn_14086a5e0 as *const (), Throws::Caught), // sub_14086a5e0
    Replacement::draft_throws(0x14086a840, "content_14086a840", fn_14086a840 as *const (), Throws::Caught), // sub_14086a840
    Replacement::draft_throws(0x14086aaa0, "content_14086aaa0", fn_14086aaa0 as *const (), Throws::Caught), // sub_14086aaa0
    Replacement::draft_throws(0x14086ad60, "content_14086ad60", tpl_2502::<0x20, 0x8, 0x1402e3480> as *const (), Throws::Discard), // sub_14086ad60
    Replacement::draft(0x14086adb0, "content_14086adb0", fn_14086adb0 as *const ()), // sub_14086adb0
    Replacement::draft(0x14086ae80, "content_14086ae80", fn_14086ae80 as *const ()), // sub_14086ae80
    Replacement::draft_throws(0x14086af60, "content_14086af60", tpl_9694::<0x20, 0x8, 0x14086d940> as *const (), Throws::Discard), // sub_14086af60
    Replacement::draft_throws(0x14086afe0, "content_14086afe0", tpl_9695::<0x20, 0x8, 0x14086e240> as *const (), Throws::Discard), // sub_14086afe0
    Replacement::draft(0x14086b130, "content_14086b130", fn_14086b130 as *const ()), // sub_14086b130
    Replacement::draft_throws(0x14086b240, "content_14086b240", tpl_9697::<0x20, 0x8, 0x1402db440> as *const (), Throws::Discard), // sub_14086b240
    Replacement::draft(0x14086b390, "content_14086b390", tpl_68372::<0x8, 0xc, 0x14062ee30, 0x10> as *const ()), // sub_14086b390
    Replacement::draft(0x14086b460, "content_14086b460", tpl_65675::<0x8, 0xc, 0x1408746b0, 0x14183acc2> as *const ()), // sub_14086b460
    Replacement::draft(0x14086b510, "content_14086b510", fn_14086b510 as *const ()), // sub_14086b510
    Replacement::draft(0x14086b630, "content_14086b630", fn_14086b630 as *const ()), // sub_14086b630
    Replacement::draft_throws(0x14086b770, "content_14086b770", fn_14086b770 as *const (), Throws::Caught), // sub_14086b770
    Replacement::draft_throws(0x14086be00, "content_14086be00", fn_14086be00 as *const (), Throws::Caught), // sub_14086be00
    Replacement::draft_throws(0x14086c460, "content_14086c460", fn_14086c460 as *const (), Throws::Caught), // sub_14086c460
    Replacement::draft_throws(0x14086c8a0, "content_14086c8a0", fn_14086c8a0 as *const (), Throws::Caught), // sub_14086c8a0
    Replacement::draft_throws(0x14086cb60, "content_14086cb60", fn_14086cb60 as *const (), Throws::Caught), // sub_14086cb60
    Replacement::draft_throws(0x14086cdf0, "content_14086cdf0", fn_14086cdf0 as *const (), Throws::Caught), // sub_14086cdf0
    Replacement::draft_throws(0x14086d080, "content_14086d080", fn_14086d080 as *const (), Throws::Caught), // sub_14086d080
    Replacement::draft_throws(0x14086d390, "content_14086d390", fn_14086d390 as *const (), Throws::Caught), // sub_14086d390
    Replacement::draft_throws(0x14086d670, "content_14086d670", fn_14086d670 as *const (), Throws::Caught), // sub_14086d670
    Replacement::draft_throws(0x14086d940, "content_14086d940", fn_14086d940 as *const (), Throws::Caught), // sub_14086d940
    Replacement::draft_throws(0x14086da00, "content_14086da00", fn_14086da00 as *const (), Throws::Caught), // sub_14086da00
    Replacement::draft_throws(0x14086dce0, "content_14086dce0", fn_14086dce0 as *const (), Throws::Caught), // sub_14086dce0
    Replacement::draft_throws(0x14086df90, "content_14086df90", fn_14086df90 as *const (), Throws::Caught), // sub_14086df90
    Replacement::draft_throws(0x14086e240, "content_14086e240", fn_14086e240 as *const (), Throws::Caught), // sub_14086e240
    Replacement::draft_throws(0x14086e300, "content_14086e300", fn_14086e300 as *const (), Throws::Caught), // sub_14086e300
    Replacement::draft_throws(0x14086e5c0, "content_14086e5c0", fn_14086e5c0 as *const (), Throws::Caught), // sub_14086e5c0
    Replacement::draft_throws(0x14086e850, "content_14086e850", fn_14086e850 as *const (), Throws::Caught), // sub_14086e850
    Replacement::draft_throws(0x14086eae0, "content_14086eae0", fn_14086eae0 as *const (), Throws::Caught), // sub_14086eae0
    Replacement::draft_throws(0x14086ebb0, "content_14086ebb0", fn_14086ebb0 as *const (), Throws::Caught), // sub_14086ebb0
    Replacement::draft_throws(0x14086ee70, "content_14086ee70", fn_14086ee70 as *const (), Throws::Caught), // sub_14086ee70
    Replacement::draft_throws(0x14086f130, "content_14086f130", fn_14086f130 as *const (), Throws::Caught), // sub_14086f130
    Replacement::draft_throws(0x14086f560, "content_14086f560", fn_14086f560 as *const (), Throws::Caught), // sub_14086f560
    Replacement::draft_throws(0x14086f960, "content_14086f960", fn_14086f960 as *const (), Throws::Caught), // sub_14086f960
    Replacement::draft_throws(0x14086fd60, "content_14086fd60", fn_14086fd60 as *const (), Throws::Caught), // sub_14086fd60
    Replacement::draft_throws(0x14086fe30, "content_14086fe30", fn_14086fe30 as *const (), Throws::Caught), // sub_14086fe30
    Replacement::draft_throws(0x140870140, "content_140870140", fn_140870140 as *const (), Throws::Caught), // sub_140870140
    Replacement::draft_throws(0x140870410, "content_140870410", fn_140870410 as *const (), Throws::Caught), // sub_140870410
    Replacement::draft_throws(0x1408706e0, "content_1408706e0", fn_1408706e0 as *const (), Throws::Caught), // sub_1408706e0
    Replacement::draft(0x1408707a0, "content_1408707a0", fn_1408707a0 as *const ()), // sub_1408707a0
    Replacement::draft_throws(0x1408707e0, "content_1408707e0", fn_1408707e0 as *const (), Throws::Caught), // sub_1408707e0
    Replacement::draft_throws(0x140870d50, "content_140870d50", fn_140870d50 as *const (), Throws::Caught), // sub_140870d50
    Replacement::draft_throws(0x140871230, "content_140871230", fn_140871230 as *const (), Throws::Caught), // sub_140871230
    Replacement::draft_throws(0x140871700, "content_140871700", fn_140871700 as *const (), Throws::Caught), // sub_140871700
    Replacement::draft_throws(0x140871c80, "content_140871c80", fn_140871c80 as *const (), Throws::Caught), // sub_140871c80
    Replacement::draft_throws(0x140872160, "content_140872160", fn_140872160 as *const (), Throws::Caught), // sub_140872160
    Replacement::draft_throws(0x140872630, "content_140872630", fn_140872630 as *const (), Throws::Caught), // sub_140872630
    Replacement::draft_throws(0x140872b00, "content_140872b00", fn_140872b00 as *const (), Throws::Caught), // sub_140872b00
    Replacement::draft_throws(0x140872f80, "content_140872f80", fn_140872f80 as *const (), Throws::Caught), // sub_140872f80
    Replacement::draft_throws(0x140873570, "content_140873570", fn_140873570 as *const (), Throws::Caught), // sub_140873570
    Replacement::draft_throws(0x140873a40, "content_140873a40", fn_140873a40 as *const (), Throws::Caught), // sub_140873a40
    Replacement::draft_throws(0x140873ec0, "content_140873ec0", fn_140873ec0 as *const (), Throws::Caught), // sub_140873ec0
    Replacement::draft_throws(0x140874340, "content_140874340", fn_140874340 as *const (), Throws::Caught), // sub_140874340
    Replacement::draft_throws(0x1408744a0, "content_1408744a0", fn_1408744a0 as *const (), Throws::Caught), // sub_1408744a0
    Replacement::draft(0x140874640, "content_140874640", fn_140874640 as *const ()), // sub_140874640
    Replacement::draft_throws(0x1408747e0, "content_1408747e0", fn_1408747e0 as *const (), Throws::Caught), // sub_1408747e0
    Replacement::draft_throws(0x140874a40, "content_140874a40", fn_140874a40 as *const (), Throws::Caught), // sub_140874a40
    Replacement::draft_throws(0x140874c90, "content_140874c90", fn_140874c90 as *const (), Throws::Caught), // sub_140874c90
    Replacement::draft_throws(0x140874ee0, "content_140874ee0", fn_140874ee0 as *const (), Throws::Caught), // sub_140874ee0
    Replacement::draft_throws(0x140875120, "content_140875120", fn_140875120 as *const (), Throws::Caught), // sub_140875120
    Replacement::draft_throws(0x140875690, "content_140875690", fn_140875690 as *const (), Throws::Caught), // sub_140875690
    Replacement::draft_throws(0x140875bc0, "content_140875bc0", fn_140875bc0 as *const (), Throws::Caught), // sub_140875bc0
    Replacement::draft(0x140875f10, "content_140875f10", fn_140875f10 as *const ()), // sub_140875f10
    Replacement::draft(0x140875fb0, "content_140875fb0", fn_140875fb0 as *const ()), // sub_140875fb0
    Replacement::draft(0x140876060, "content_140876060", fn_140876060 as *const ()), // sub_140876060
    Replacement::draft_throws(0x140876180, "content_140876180", fn_140876180 as *const (), Throws::Discard), // sub_140876180
    Replacement::draft(0x140876280, "content_140876280", fn_140876280 as *const ()), // sub_140876280
    Replacement::draft_throws(0x140876310, "content_140876310", fn_140876310 as *const (), Throws::Discard), // sub_140876310
    Replacement::draft_throws(0x140876410, "content_140876410", fn_140876410 as *const (), Throws::Caught), // sub_140876410
    Replacement::draft_throws(0x1408766c0, "content_1408766c0", fn_1408766c0 as *const (), Throws::Caught), // sub_1408766c0
    Replacement::draft_throws(0x140876940, "content_140876940", fn_140876940 as *const (), Throws::Caught), // sub_140876940
    Replacement::draft(0x140876bc0, "content_140876bc0", fn_140876bc0 as *const ()), // sub_140876bc0
    Replacement::draft(0x140876c70, "content_140876c70", fn_140876c70 as *const ()), // sub_140876c70
    Replacement::draft(0x140876d00, "content_140876d00", fn_140876d00 as *const ()), // sub_140876d00
    Replacement::draft_throws(0x140876da0, "content_140876da0", fn_140876da0 as *const (), Throws::Discard), // sub_140876da0
    Replacement::draft_throws(0x140876e40, "content_140876e40", fn_140876e40 as *const (), Throws::Discard), // sub_140876e40
    Replacement::draft(0x140876f60, "content_140876f60", fn_140876f60 as *const ()), // sub_140876f60
    Replacement::draft(0x1408774b0, "content_1408774b0", fn_1408774b0 as *const ()), // sub_1408774b0
    Replacement::draft(0x1408774d0, "content_1408774d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432706f8> as *const ()), // sub_1408774d0
    Replacement::draft(0x140877530, "content_140877530", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432706f8> as *const ()), // sub_140877530
    Replacement::draft_throws(0x1408775c0, "content_1408775c0", fn_1408775c0 as *const (), Throws::Caught), // sub_1408775c0
    Replacement::draft_throws(0x140877650, "content_140877650", tpl_1908::<0x14387d870, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140877650
    Replacement::draft(0x1408776d0, "content_1408776d0", tpl_9749::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143272d48> as *const ()), // sub_1408776d0
    Replacement::draft(0x140877740, "content_140877740", tpl_9750::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272d48> as *const ()), // sub_140877740
    Replacement::draft(0x140877870, "content_140877870", fn_140877870 as *const ()), // sub_140877870
    Replacement::draft_throws(0x140877940, "content_140877940", fn_140877940 as *const (), Throws::Discard), // sub_140877940
    Replacement::draft(0x1408779f0, "content_1408779f0", tpl_9753::<0x14387d850, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x80> as *const ()), // sub_1408779f0
    Replacement::draft_throws(0x140877aa0, "content_140877aa0", fn_140877aa0 as *const (), Throws::Discard), // sub_140877aa0
    Replacement::draft_throws(0x140877b60, "content_140877b60", fn_140877b60 as *const (), Throws::Caught), // sub_140877b60
    Replacement::draft_throws(0x140877c30, "content_140877c30", fn_140877c30 as *const (), Throws::Caught), // sub_140877c30
    Replacement::draft_throws(0x140877d00, "content_140877d00", fn_140877d00 as *const (), Throws::Caught), // sub_140877d00
    Replacement::draft(0x140877dd0, "content_140877dd0", fn_140877dd0 as *const ()), // sub_140877dd0
    Replacement::draft(0x140877e80, "content_140877e80", tpl_9757::<0x14387d7a0, 0x14387d850, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0x120> as *const ()), // sub_140877e80
    Replacement::draft(0x140877f20, "content_140877f20", tpl_9753::<0x14387d850, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x120> as *const ()), // sub_140877f20
    Replacement::draft_throws(0x140877fd0, "content_140877fd0", fn_140877fd0 as *const (), Throws::Discard), // sub_140877fd0
    Replacement::draft_throws(0x140878070, "content_140878070", fn_140878070 as *const (), Throws::Discard), // sub_140878070
    Replacement::draft(0x140878160, "content_140878160", tpl_9749::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14326f488> as *const ()), // sub_140878160
    Replacement::draft(0x1408781d0, "content_1408781d0", tpl_9750::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326f488> as *const ()), // sub_1408781d0
    Replacement::draft(0x140878300, "content_140878300", tpl_9760::<0x14387d7b8, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140878300
    Replacement::draft_throws(0x1408783e0, "content_1408783e0", tpl_9761::<0x14387d840, 0x14387d7b8, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x338, 0x8, 0x48, 0x369> as *const (), Throws::Discard), // sub_1408783e0
    Replacement::draft(0x140878490, "content_140878490", tpl_9753::<0x14387d7b8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x48> as *const ()), // sub_140878490
    Replacement::draft_throws(0x140878540, "content_140878540", tpl_9762::<0x14387d840, 0x14387d7b8, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369> as *const (), Throws::Discard), // sub_140878540
    Replacement::draft_throws(0x140878600, "content_140878600", fn_140878600 as *const (), Throws::Caught), // sub_140878600
    Replacement::draft_throws(0x1408786d0, "content_1408786d0", fn_1408786d0 as *const (), Throws::Caught), // sub_1408786d0
    Replacement::draft_throws(0x1408787a0, "content_1408787a0", fn_1408787a0 as *const (), Throws::Caught), // sub_1408787a0
    Replacement::draft(0x140878870, "content_140878870", tpl_9764::<0x14387d7b8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338> as *const ()), // sub_140878870
    Replacement::draft(0x140878920, "content_140878920", tpl_9757::<0x14387d840, 0x14387d7b8, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xe8> as *const ()), // sub_140878920
    Replacement::draft(0x1408789c0, "content_1408789c0", tpl_9753::<0x14387d7b8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xe8> as *const ()), // sub_1408789c0
    Replacement::draft_throws(0x140878a70, "content_140878a70", tpl_9765::<0x14387d840, 0x14387d7b8, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_140878a70
    Replacement::draft_throws(0x140878b10, "content_140878b10", fn_140878b10 as *const (), Throws::Discard), // sub_140878b10
    Replacement::draft(0x140878c00, "content_140878c00", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432702c8, 0x8> as *const ()), // sub_140878c00
    Replacement::draft(0x140878c80, "content_140878c80", tpl_9767::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432702c8, 0xfffffffffffffff8> as *const ()), // sub_140878c80
    Replacement::draft(0x140878db0, "content_140878db0", tpl_9768::<0x14387d7c8, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140878db0
    Replacement::draft_throws(0x140878e90, "content_140878e90", fn_140878e90 as *const (), Throws::Discard), // sub_140878e90
    Replacement::draft(0x140878f60, "content_140878f60", tpl_9770::<0x14387d7c8, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x48, 0xc> as *const ()), // sub_140878f60
    Replacement::draft_throws(0x140879030, "content_140879030", fn_140879030 as *const (), Throws::Discard), // sub_140879030
    Replacement::draft_throws(0x1408790f0, "content_1408790f0", fn_1408790f0 as *const (), Throws::Caught), // sub_1408790f0
    Replacement::draft_throws(0x140879300, "content_140879300", fn_140879300 as *const (), Throws::Caught), // sub_140879300
    Replacement::draft_throws(0x1408794e0, "content_1408794e0", fn_1408794e0 as *const (), Throws::Caught), // sub_1408794e0
    Replacement::draft(0x1408796b0, "content_1408796b0", fn_1408796b0 as *const ()), // sub_1408796b0
    Replacement::draft(0x140879770, "content_140879770", tpl_9776::<0x14387d87c, 0x14387d7c8, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xe8, 0xc> as *const ()), // sub_140879770
    Replacement::draft(0x140879820, "content_140879820", tpl_9770::<0x14387d7c8, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xe8, 0xc> as *const ()), // sub_140879820
    Replacement::draft_throws(0x1408798f0, "content_1408798f0", tpl_9777::<0x14387d87c, 0x14387d7c8, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_1408798f0
    Replacement::draft_throws(0x140879990, "content_140879990", fn_140879990 as *const (), Throws::Discard), // sub_140879990
    Replacement::draft(0x140879a70, "content_140879a70", tpl_9779::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143272548, 0x8> as *const ()), // sub_140879a70
    Replacement::draft(0x140879b00, "content_140879b00", tpl_9780::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272548, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140879b00
    Replacement::draft(0x140879c50, "content_140879c50", fn_140879c50 as *const ()), // sub_140879c50
    Replacement::draft_throws(0x140879d30, "content_140879d30", tpl_9782::<0x14387d718, 0x14387d748, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x8, 0x338, 0x48, 0x4, 0x369> as *const (), Throws::Discard), // sub_140879d30
    Replacement::draft(0x140879e10, "content_140879e10", tpl_9783::<0x14387d748, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x48, 0xc, 0x10> as *const ()), // sub_140879e10
    Replacement::draft_throws(0x140879ef0, "content_140879ef0", tpl_9784::<0x14387d718, 0x14387d748, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369> as *const (), Throws::Discard), // sub_140879ef0
    Replacement::draft_throws(0x140879fc0, "content_140879fc0", fn_140879fc0 as *const (), Throws::Caught), // sub_140879fc0
    Replacement::draft_throws(0x14087a1c0, "content_14087a1c0", fn_14087a1c0 as *const (), Throws::Caught), // sub_14087a1c0
    Replacement::draft_throws(0x14087a390, "content_14087a390", fn_14087a390 as *const (), Throws::Caught), // sub_14087a390
    Replacement::draft(0x14087a550, "content_14087a550", tpl_9788::<0x14387d748, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x10> as *const ()), // sub_14087a550
    Replacement::draft(0x14087a600, "content_14087a600", tpl_9789::<0x14387d718, 0x14387d748, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xe8, 0xc, 0x10> as *const ()), // sub_14087a600
    Replacement::draft(0x14087a6c0, "content_14087a6c0", tpl_9783::<0x14387d748, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xe8, 0xc, 0x10> as *const ()), // sub_14087a6c0
    Replacement::draft_throws(0x14087a7a0, "content_14087a7a0", tpl_9790::<0x14387d718, 0x14387d748, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120, 0x10> as *const (), Throws::Discard), // sub_14087a7a0
    Replacement::draft_throws(0x14087a850, "content_14087a850", fn_14087a850 as *const (), Throws::Discard), // sub_14087a850
    Replacement::draft(0x14087a940, "content_14087a940", tpl_9792::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143270008, 0x8> as *const ()), // sub_14087a940
    Replacement::draft(0x14087a9d0, "content_14087a9d0", tpl_9793::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143270008, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_14087a9d0
    Replacement::draft(0x14087ab20, "content_14087ab20", tpl_9794::<0x14387d788, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_14087ab20
    Replacement::draft_throws(0x14087ac00, "content_14087ac00", fn_14087ac00 as *const (), Throws::Discard), // sub_14087ac00
    Replacement::draft(0x14087acf0, "content_14087acf0", fn_14087acf0 as *const ()), // sub_14087acf0
    Replacement::draft_throws(0x14087add0, "content_14087add0", tpl_9797::<0x14387d75c, 0x14387d788, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369, 0x10> as *const (), Throws::Discard), // sub_14087add0
    Replacement::draft_throws(0x14087ae90, "content_14087ae90", fn_14087ae90 as *const (), Throws::Caught), // sub_14087ae90
    Replacement::draft_throws(0x14087b090, "content_14087b090", fn_14087b090 as *const (), Throws::Caught), // sub_14087b090
    Replacement::draft_throws(0x14087b260, "content_14087b260", fn_14087b260 as *const (), Throws::Caught), // sub_14087b260
    Replacement::draft(0x14087b420, "content_14087b420", tpl_9801::<0x14387d788, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x10> as *const ()), // sub_14087b420
    Replacement::draft(0x14087b4e0, "content_14087b4e0", fn_14087b4e0 as *const ()), // sub_14087b4e0
    Replacement::draft(0x14087b5b0, "content_14087b5b0", fn_14087b5b0 as *const ()), // sub_14087b5b0
    Replacement::draft_throws(0x14087b6a0, "content_14087b6a0", tpl_9804::<0x14387d75c, 0x14387d788, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_14087b6a0
    Replacement::draft_throws(0x14087b740, "content_14087b740", fn_14087b740 as *const (), Throws::Discard), // sub_14087b740
    Replacement::draft(0x14087b830, "content_14087b830", tpl_9749::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14326f218> as *const ()), // sub_14087b830
    Replacement::draft(0x14087b8a0, "content_14087b8a0", tpl_9750::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326f218> as *const ()), // sub_14087b8a0
    Replacement::draft(0x14087b9d0, "content_14087b9d0", tpl_9806::<0x14387d730, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_14087b9d0
    Replacement::draft_throws(0x14087bab0, "content_14087bab0", tpl_9761::<0x14387d824, 0x14387d730, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x338, 0x8, 0x38, 0x369> as *const (), Throws::Discard), // sub_14087bab0
    Replacement::draft(0x14087bb60, "content_14087bb60", tpl_9753::<0x14387d730, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x38> as *const ()), // sub_14087bb60
    Replacement::draft_throws(0x14087bc10, "content_14087bc10", tpl_9807::<0x14387d824, 0x14387d730, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x4, 0x369> as *const (), Throws::Discard), // sub_14087bc10
    Replacement::draft_throws(0x14087bcd0, "content_14087bcd0", fn_14087bcd0 as *const (), Throws::Caught), // sub_14087bcd0
    Replacement::draft_throws(0x14087bda0, "content_14087bda0", fn_14087bda0 as *const (), Throws::Caught), // sub_14087bda0
    Replacement::draft_throws(0x14087be70, "content_14087be70", fn_14087be70 as *const (), Throws::Caught), // sub_14087be70
    Replacement::draft(0x14087bf40, "content_14087bf40", tpl_9764::<0x14387d730, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338> as *const ()), // sub_14087bf40
    Replacement::draft(0x14087bff0, "content_14087bff0", tpl_9757::<0x14387d824, 0x14387d730, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xd8> as *const ()), // sub_14087bff0
    Replacement::draft(0x14087c090, "content_14087c090", tpl_9753::<0x14387d730, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xd8> as *const ()), // sub_14087c090
    Replacement::draft_throws(0x14087c140, "content_14087c140", tpl_9809::<0x14387d824, 0x14387d730, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120, 0x4> as *const (), Throws::Discard), // sub_14087c140
    Replacement::draft_throws(0x14087c1e0, "content_14087c1e0", fn_14087c1e0 as *const (), Throws::Discard), // sub_14087c1e0
    Replacement::draft(0x14087c2d0, "content_14087c2d0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432700b8, 0x8> as *const ()), // sub_14087c2d0
    Replacement::draft(0x14087c350, "content_14087c350", tpl_9767::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432700b8, 0xfffffffffffffff8> as *const ()), // sub_14087c350
    Replacement::draft(0x14087c480, "content_14087c480", tpl_9811::<0x14387d868, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_14087c480
    Replacement::draft_throws(0x14087c560, "content_14087c560", tpl_9812::<0x14387d83c, 0x14387d868, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x8, 0x338, 0x38, 0x4, 0x369> as *const (), Throws::Discard), // sub_14087c560
    Replacement::draft(0x14087c640, "content_14087c640", tpl_9770::<0x14387d868, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x38, 0xc> as *const ()), // sub_14087c640
    Replacement::draft_throws(0x14087c710, "content_14087c710", tpl_9813::<0x14387d83c, 0x14387d868, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369> as *const (), Throws::Discard), // sub_14087c710
    Replacement::draft_throws(0x14087c7e0, "content_14087c7e0", fn_14087c7e0 as *const (), Throws::Caught), // sub_14087c7e0
    Replacement::draft_throws(0x14087ca00, "content_14087ca00", fn_14087ca00 as *const (), Throws::Caught), // sub_14087ca00
    Replacement::draft_throws(0x14087cbf0, "content_14087cbf0", fn_14087cbf0 as *const (), Throws::Caught), // sub_14087cbf0
    Replacement::draft(0x14087cdd0, "content_14087cdd0", fn_14087cdd0 as *const ()), // sub_14087cdd0
    Replacement::draft(0x14087ce90, "content_14087ce90", tpl_9776::<0x14387d83c, 0x14387d868, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xd8, 0xc> as *const ()), // sub_14087ce90
    Replacement::draft(0x14087cf40, "content_14087cf40", tpl_9770::<0x14387d868, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xd8, 0xc> as *const ()), // sub_14087cf40
    Replacement::draft_throws(0x14087d010, "content_14087d010", tpl_9777::<0x14387d83c, 0x14387d868, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_14087d010
    Replacement::draft_throws(0x14087d0b0, "content_14087d0b0", tpl_9818::<0x14387d868, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x20, 0x8, 0x4, 0x143251120, 0x1413c3e10> as *const (), Throws::Discard), // sub_14087d0b0
    Replacement::draft(0x14087d1a0, "content_14087d1a0", tpl_9779::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143272228, 0x8> as *const ()), // sub_14087d1a0
    Replacement::draft(0x14087d230, "content_14087d230", tpl_9780::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272228, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_14087d230
    Replacement::draft(0x14087d380, "content_14087d380", tpl_9819::<0x14387d7d0, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_14087d380
    Replacement::draft_throws(0x14087d460, "content_14087d460", tpl_9782::<0x14387d740, 0x14387d7d0, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x8, 0x338, 0x38, 0x4, 0x369> as *const (), Throws::Discard), // sub_14087d460
    Replacement::draft(0x14087d540, "content_14087d540", tpl_9783::<0x14387d7d0, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x38, 0xc, 0x10> as *const ()), // sub_14087d540
    Replacement::draft_throws(0x14087d620, "content_14087d620", tpl_9784::<0x14387d740, 0x14387d7d0, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369> as *const (), Throws::Discard), // sub_14087d620
    Replacement::draft_throws(0x14087d6f0, "content_14087d6f0", fn_14087d6f0 as *const (), Throws::Caught), // sub_14087d6f0
    Replacement::draft_throws(0x14087d8f0, "content_14087d8f0", fn_14087d8f0 as *const (), Throws::Caught), // sub_14087d8f0
    Replacement::draft_throws(0x14087dac0, "content_14087dac0", fn_14087dac0 as *const (), Throws::Caught), // sub_14087dac0
    Replacement::draft(0x14087dc80, "content_14087dc80", tpl_9788::<0x14387d7d0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x10> as *const ()), // sub_14087dc80
    Replacement::draft(0x14087dd30, "content_14087dd30", tpl_9789::<0x14387d740, 0x14387d7d0, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xd8, 0xc, 0x10> as *const ()), // sub_14087dd30
    Replacement::draft(0x14087ddf0, "content_14087ddf0", tpl_9783::<0x14387d7d0, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xd8, 0xc, 0x10> as *const ()), // sub_14087ddf0
    Replacement::draft_throws(0x14087ded0, "content_14087ded0", tpl_9790::<0x14387d740, 0x14387d7d0, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120, 0x10> as *const (), Throws::Discard), // sub_14087ded0
    Replacement::draft_throws(0x14087df80, "content_14087df80", tpl_9820::<0x14387d7d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x20, 0x8, 0x4, 0x143251120, 0x1413c41a0> as *const (), Throws::Discard), // sub_14087df80
    Replacement::draft(0x14087e080, "content_14087e080", tpl_9792::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432707a8, 0x8> as *const ()), // sub_14087e080
    Replacement::draft(0x14087e110, "content_14087e110", tpl_9793::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432707a8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_14087e110
    Replacement::draft(0x14087e260, "content_14087e260", tpl_9821::<0x14387d780, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_14087e260
    Replacement::draft_throws(0x14087e340, "content_14087e340", fn_14087e340 as *const (), Throws::Discard), // sub_14087e340
    Replacement::draft(0x14087e430, "content_14087e430", tpl_9823::<0x14387d780, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x38, 0xc, 0x10, 0x14> as *const ()), // sub_14087e430
    Replacement::draft_throws(0x14087e510, "content_14087e510", tpl_9797::<0x14387d728, 0x14387d780, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369, 0x10> as *const (), Throws::Discard), // sub_14087e510
    Replacement::draft_throws(0x14087e5d0, "content_14087e5d0", fn_14087e5d0 as *const (), Throws::Caught), // sub_14087e5d0
    Replacement::draft_throws(0x14087e7d0, "content_14087e7d0", fn_14087e7d0 as *const (), Throws::Caught), // sub_14087e7d0
    Replacement::draft_throws(0x14087e9a0, "content_14087e9a0", fn_14087e9a0 as *const (), Throws::Caught), // sub_14087e9a0
    Replacement::draft(0x14087eb60, "content_14087eb60", tpl_9801::<0x14387d780, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x10> as *const ()), // sub_14087eb60
    Replacement::draft(0x14087ec20, "content_14087ec20", tpl_9824::<0x14387d728, 0x14387d780, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14> as *const ()), // sub_14087ec20
    Replacement::draft(0x14087ecf0, "content_14087ecf0", tpl_9825::<0x14387d780, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xd8, 0xc, 0x10, 0x14> as *const ()), // sub_14087ecf0
    Replacement::draft_throws(0x14087ede0, "content_14087ede0", tpl_9804::<0x14387d728, 0x14387d780, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_14087ede0
    Replacement::draft_throws(0x14087ee80, "content_14087ee80", tpl_9826::<0x14387d780, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x20, 0x8, 0x4, 0xc, 0x143251120, 0x1413c4540> as *const (), Throws::Discard), // sub_14087ee80
    Replacement::draft(0x14087ef90, "content_14087ef90", fn_14087ef90 as *const ()), // sub_14087ef90
    Replacement::draft(0x14087f030, "content_14087f030", fn_14087f030 as *const ()), // sub_14087f030
    Replacement::draft_throws(0x14087f180, "content_14087f180", fn_14087f180 as *const (), Throws::Discard), // sub_14087f180
    Replacement::draft_throws(0x14087f280, "content_14087f280", fn_14087f280 as *const (), Throws::Discard), // sub_14087f280
    Replacement::draft(0x14087f350, "content_14087f350", tpl_9753::<0x14387d7d8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x20> as *const ()), // sub_14087f350
    Replacement::draft_throws(0x14087f400, "content_14087f400", fn_14087f400 as *const (), Throws::Discard), // sub_14087f400
    Replacement::draft_throws(0x14087f4d0, "content_14087f4d0", fn_14087f4d0 as *const (), Throws::Caught), // sub_14087f4d0
    Replacement::draft_throws(0x14087f740, "content_14087f740", fn_14087f740 as *const (), Throws::Caught), // sub_14087f740
    Replacement::draft_throws(0x14087f970, "content_14087f970", fn_14087f970 as *const (), Throws::Caught), // sub_14087f970
    Replacement::draft(0x14087fb90, "content_14087fb90", fn_14087fb90 as *const ()), // sub_14087fb90
    Replacement::draft(0x14087fc60, "content_14087fc60", tpl_9757::<0x14387d820, 0x14387d7d8, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0xc0> as *const ()), // sub_14087fc60
    Replacement::draft(0x14087fd00, "content_14087fd00", tpl_9753::<0x14387d7d8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xc0> as *const ()), // sub_14087fd00
    Replacement::draft_throws(0x14087fdb0, "content_14087fdb0", fn_14087fdb0 as *const (), Throws::Discard), // sub_14087fdb0
    Replacement::draft_throws(0x14087fe40, "content_14087fe40", fn_14087fe40 as *const (), Throws::Discard), // sub_14087fe40
    Replacement::draft(0x14087ff30, "content_14087ff30", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143270168, 0x8> as *const ()), // sub_14087ff30
    Replacement::draft(0x14087ffb0, "content_14087ffb0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143270168> as *const ()), // sub_14087ffb0
    Replacement::draft(0x1408800e0, "content_1408800e0", tpl_9838::<0x14387d7a8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x140839b60> as *const ()), // sub_1408800e0
    Replacement::draft_throws(0x140880190, "content_140880190", tpl_9839::<0x14387d760, 0x14387d7a8, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14082b050> as *const (), Throws::Discard), // sub_140880190
    Replacement::draft(0x140880230, "content_140880230", tpl_9838::<0x14387d7a8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14083a060> as *const ()), // sub_140880230
    Replacement::draft_throws(0x1408802e0, "content_1408802e0", tpl_9839::<0x14387d760, 0x14387d7a8, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14082b180> as *const (), Throws::Discard), // sub_1408802e0
    Replacement::draft_throws(0x140880380, "content_140880380", fn_140880380 as *const (), Throws::Caught), // sub_140880380
    Replacement::draft_throws(0x140880450, "content_140880450", fn_140880450 as *const (), Throws::Caught), // sub_140880450
    Replacement::draft_throws(0x140880520, "content_140880520", fn_140880520 as *const (), Throws::Caught), // sub_140880520
    Replacement::draft(0x1408805f0, "content_1408805f0", tpl_9841::<0x14387d7a8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x140839ed0> as *const ()), // sub_1408805f0
    Replacement::draft(0x1408806b0, "content_1408806b0", tpl_9842::<0x14387d760, 0x14387d7a8, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14083a350> as *const ()), // sub_1408806b0
    Replacement::draft(0x140880740, "content_140880740", tpl_9838::<0x14387d7a8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14083a4c0> as *const ()), // sub_140880740
    Replacement::draft_throws(0x1408807f0, "content_1408807f0", tpl_9843::<0x14387d760, 0x14387d7a8, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14083a610> as *const (), Throws::Discard), // sub_1408807f0
    Replacement::draft_throws(0x140880880, "content_140880880", fn_140880880 as *const (), Throws::Caught), // sub_140880880
    Replacement::draft(0x1408809f0, "content_1408809f0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143270218, 0x8> as *const ()), // sub_1408809f0
    Replacement::draft(0x140880a70, "content_140880a70", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143270218> as *const ()), // sub_140880a70
    Replacement::draft(0x140880ba0, "content_140880ba0", tpl_9838::<0x14387d768, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089c0e0> as *const ()), // sub_140880ba0
    Replacement::draft_throws(0x140880c50, "content_140880c50", tpl_9839::<0x14387d778, 0x14387d768, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088b020> as *const (), Throws::Discard), // sub_140880c50
    Replacement::draft(0x140880cf0, "content_140880cf0", tpl_9838::<0x14387d768, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089c5e0> as *const ()), // sub_140880cf0
    Replacement::draft_throws(0x140880da0, "content_140880da0", tpl_9839::<0x14387d778, 0x14387d768, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088b150> as *const (), Throws::Discard), // sub_140880da0
    Replacement::draft_throws(0x140880e40, "content_140880e40", fn_140880e40 as *const (), Throws::Caught), // sub_140880e40
    Replacement::draft_throws(0x140880f10, "content_140880f10", fn_140880f10 as *const (), Throws::Caught), // sub_140880f10
    Replacement::draft_throws(0x140880fe0, "content_140880fe0", fn_140880fe0 as *const (), Throws::Caught), // sub_140880fe0
    Replacement::draft(0x1408810b0, "content_1408810b0", tpl_9841::<0x14387d768, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089c450> as *const ()), // sub_1408810b0
    Replacement::draft(0x140881170, "content_140881170", tpl_9842::<0x14387d778, 0x14387d768, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089c8d0> as *const ()), // sub_140881170
    Replacement::draft(0x140881200, "content_140881200", tpl_9838::<0x14387d768, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089ca40> as *const ()), // sub_140881200
    Replacement::draft_throws(0x1408812b0, "content_1408812b0", tpl_9843::<0x14387d778, 0x14387d768, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089cb90> as *const (), Throws::Discard), // sub_1408812b0
    Replacement::draft_throws(0x140881340, "content_140881340", tpl_9845::<0x14387d768, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x8, 0x143251120, 0x14088b280> as *const (), Throws::Discard), // sub_140881340
    Replacement::draft(0x140881410, "content_140881410", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143272498, 0x8> as *const ()), // sub_140881410
    Replacement::draft(0x140881490, "content_140881490", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272498> as *const ()), // sub_140881490
    Replacement::draft_throws(0x140881670, "content_140881670", tpl_9839::<0x14387d738, 0x14387d830, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088b370> as *const (), Throws::Discard), // sub_140881670
    Replacement::draft(0x140881710, "content_140881710", tpl_9838::<0x14387d830, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089d260> as *const ()), // sub_140881710
    Replacement::draft_throws(0x1408817c0, "content_1408817c0", tpl_9839::<0x14387d738, 0x14387d830, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088b4a0> as *const (), Throws::Discard), // sub_1408817c0
    Replacement::draft_throws(0x140881860, "content_140881860", fn_140881860 as *const (), Throws::Caught), // sub_140881860
    Replacement::draft_throws(0x140881930, "content_140881930", fn_140881930 as *const (), Throws::Caught), // sub_140881930
    Replacement::draft_throws(0x140881a00, "content_140881a00", fn_140881a00 as *const (), Throws::Caught), // sub_140881a00
    Replacement::draft(0x140881ad0, "content_140881ad0", tpl_9841::<0x14387d830, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089d0d0> as *const ()), // sub_140881ad0
    Replacement::draft(0x140881b90, "content_140881b90", tpl_9842::<0x14387d738, 0x14387d830, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089d550> as *const ()), // sub_140881b90
    Replacement::draft(0x140881c20, "content_140881c20", tpl_9838::<0x14387d830, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089d6c0> as *const ()), // sub_140881c20
    Replacement::draft_throws(0x140881cd0, "content_140881cd0", tpl_9843::<0x14387d738, 0x14387d830, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089d810> as *const (), Throws::Discard), // sub_140881cd0
    Replacement::draft_throws(0x140881d60, "content_140881d60", tpl_9845::<0x14387d830, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x8, 0x143251120, 0x14088b5d0> as *const (), Throws::Discard), // sub_140881d60
    Replacement::draft(0x140881e30, "content_140881e30", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143272c98, 0x8> as *const ()), // sub_140881e30
    Replacement::draft(0x140881eb0, "content_140881eb0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272c98> as *const ()), // sub_140881eb0
    Replacement::draft(0x140881fe0, "content_140881fe0", tpl_9838::<0x14387d808, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089d9e0> as *const ()), // sub_140881fe0
    Replacement::draft_throws(0x140882090, "content_140882090", tpl_9846::<0x14387d828, 0x14387d808, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088b6c0> as *const (), Throws::Discard), // sub_140882090
    Replacement::draft(0x140882120, "content_140882120", tpl_9838::<0x14387d808, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089dee0> as *const ()), // sub_140882120
    Replacement::draft_throws(0x1408821d0, "content_1408821d0", tpl_9846::<0x14387d828, 0x14387d808, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088b7f0> as *const (), Throws::Discard), // sub_1408821d0
    Replacement::draft_throws(0x140882260, "content_140882260", fn_140882260 as *const (), Throws::Caught), // sub_140882260
    Replacement::draft_throws(0x140882330, "content_140882330", fn_140882330 as *const (), Throws::Caught), // sub_140882330
    Replacement::draft_throws(0x140882400, "content_140882400", fn_140882400 as *const (), Throws::Caught), // sub_140882400
    Replacement::draft(0x1408824d0, "content_1408824d0", tpl_9841::<0x14387d808, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089dd50> as *const ()), // sub_1408824d0
    Replacement::draft(0x140882590, "content_140882590", tpl_9842::<0x14387d828, 0x14387d808, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089e1d0> as *const ()), // sub_140882590
    Replacement::draft(0x140882620, "content_140882620", tpl_9838::<0x14387d808, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089e340> as *const ()), // sub_140882620
    Replacement::draft_throws(0x1408826d0, "content_1408826d0", tpl_9843::<0x14387d828, 0x14387d808, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089e490> as *const (), Throws::Discard), // sub_1408826d0
    Replacement::draft_throws(0x140882760, "content_140882760", tpl_9845::<0x14387d808, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x8, 0x143251120, 0x14088b920> as *const (), Throws::Discard), // sub_140882760
    Replacement::draft(0x140882830, "content_140882830", tpl_6379::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143271f08, 0x8, 0x1c> as *const ()), // sub_140882830
    Replacement::draft(0x1408828c0, "content_1408828c0", tpl_9847::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x143271f08, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x4> as *const ()), // sub_1408828c0
    Replacement::draft(0x140882a00, "content_140882a00", tpl_9848::<0x14387d7f8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1406dd2c0> as *const ()), // sub_140882a00
    Replacement::draft_throws(0x140882ab0, "content_140882ab0", tpl_9849::<0x14387d73c, 0x14387d7f8, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x8, 0x338, 0x1403c7b30, 0x369, 0x1401e84a0> as *const (), Throws::Discard), // sub_140882ab0
    Replacement::draft(0x140882b90, "content_140882b90", tpl_9850::<0x14387d7f8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x1403c7b30> as *const ()), // sub_140882b90
    Replacement::draft_throws(0x140882c40, "content_140882c40", tpl_9851::<0x14387d73c, 0x14387d7f8, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x8, 0x338, 0x1406f1f30, 0x369, 0x1401e84a0> as *const (), Throws::Discard), // sub_140882c40
    Replacement::draft_throws(0x140882d10, "content_140882d10", fn_140882d10 as *const (), Throws::Caught), // sub_140882d10
    Replacement::draft_throws(0x140882dc0, "content_140882dc0", fn_140882dc0 as *const (), Throws::Caught), // sub_140882dc0
    Replacement::draft_throws(0x140882e70, "content_140882e70", fn_140882e70 as *const (), Throws::Caught), // sub_140882e70
    Replacement::draft(0x140882f20, "content_140882f20", fn_140882f20 as *const ()), // sub_140882f20
    Replacement::draft(0x140883010, "content_140883010", tpl_9853::<0x14387d73c, 0x14387d7f8, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0x88, 0xc0, 0x14240ab10> as *const ()), // sub_140883010
    Replacement::draft(0x140883100, "content_140883100", tpl_9854::<0x14387d7f8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x88, 0xc0, 0x14240ab10> as *const ()), // sub_140883100
    Replacement::draft_throws(0x140883200, "content_140883200", tpl_9846::<0x14387d73c, 0x14387d7f8, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x1406deba0> as *const (), Throws::Discard), // sub_140883200
    Replacement::draft_throws(0x140883290, "content_140883290", fn_140883290 as *const (), Throws::Caught), // sub_140883290
    Replacement::draft(0x1408833d0, "content_1408833d0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143272fb8, 0x8> as *const ()), // sub_1408833d0
    Replacement::draft(0x140883450, "content_140883450", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272fb8> as *const ()), // sub_140883450
    Replacement::draft(0x1408834e0, "content_1408834e0", tpl_9856::<0x14387d848, 0x14243b0b0, 0x14387d870, 0x8, 0x143881f70, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408834e0
    Replacement::draft(0x1408835c0, "content_1408835c0", tpl_9838::<0x14387d848, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x140559ea0> as *const ()), // sub_1408835c0
    Replacement::draft_throws(0x140883670, "content_140883670", tpl_9839::<0x14387d770, 0x14387d848, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x140557390> as *const (), Throws::Discard), // sub_140883670
    Replacement::draft(0x140883710, "content_140883710", tpl_9838::<0x14387d848, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14055a030> as *const ()), // sub_140883710
    Replacement::draft_throws(0x1408837c0, "content_1408837c0", tpl_9839::<0x14387d770, 0x14387d848, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x1405575f0> as *const (), Throws::Discard), // sub_1408837c0
    Replacement::draft_throws(0x140883860, "content_140883860", fn_140883860 as *const (), Throws::Caught), // sub_140883860
    Replacement::draft_throws(0x140883910, "content_140883910", fn_140883910 as *const (), Throws::Caught), // sub_140883910
    Replacement::draft_throws(0x1408839c0, "content_1408839c0", fn_1408839c0 as *const (), Throws::Caught), // sub_1408839c0
    Replacement::draft(0x140883a70, "content_140883a70", tpl_9841::<0x14387d848, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x140559fc0> as *const ()), // sub_140883a70
    Replacement::draft(0x140883b30, "content_140883b30", fn_140883b30 as *const ()), // sub_140883b30
    Replacement::draft(0x140883c00, "content_140883c00", tpl_9838::<0x14387d848, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14055a1a0> as *const ()), // sub_140883c00
    Replacement::draft_throws(0x140883cb0, "content_140883cb0", tpl_9843::<0x14387d770, 0x14387d848, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14055a210> as *const (), Throws::Discard), // sub_140883cb0
    Replacement::draft_throws(0x140883d40, "content_140883d40", tpl_9858::<0x14387d848, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x8, 0x143251120, 0x140558dc0> as *const (), Throws::Discard), // sub_140883d40
    Replacement::draft(0x140883e10, "content_140883e10", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432727b8, 0x8> as *const ()), // sub_140883e10
    Replacement::draft(0x140883e90, "content_140883e90", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432727b8> as *const ()), // sub_140883e90
    Replacement::draft(0x140883fc0, "content_140883fc0", tpl_9848::<0x14387d7e0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14028a610> as *const ()), // sub_140883fc0
    Replacement::draft_throws(0x140884070, "content_140884070", tpl_9859::<0x14387d810, 0x14387d7e0, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x338, 0x8, 0x369> as *const (), Throws::Discard), // sub_140884070
    Replacement::draft(0x140884120, "content_140884120", tpl_9860::<0x14387d7e0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338> as *const ()), // sub_140884120
    Replacement::draft_throws(0x1408841d0, "content_1408841d0", tpl_9861::<0x14387d810, 0x14387d7e0, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369> as *const (), Throws::Discard), // sub_1408841d0
    Replacement::draft_throws(0x140884290, "content_140884290", fn_140884290 as *const (), Throws::Caught), // sub_140884290
    Replacement::draft_throws(0x140884350, "content_140884350", fn_140884350 as *const (), Throws::Caught), // sub_140884350
    Replacement::draft_throws(0x140884410, "content_140884410", fn_140884410 as *const (), Throws::Caught), // sub_140884410
    Replacement::draft(0x1408844d0, "content_1408844d0", tpl_9863::<0x14387d7e0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338> as *const ()), // sub_1408844d0
    Replacement::draft(0x140884580, "content_140884580", tpl_9757::<0x14387d810, 0x14387d7e0, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0x88> as *const ()), // sub_140884580
    Replacement::draft(0x140884620, "content_140884620", fn_140884620 as *const ()), // sub_140884620
    Replacement::draft_throws(0x1408846d0, "content_1408846d0", tpl_9777::<0x14387d810, 0x14387d7e0, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_1408846d0
    Replacement::draft_throws(0x140884770, "content_140884770", tpl_9858::<0x14387d7e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x8, 0x143251120, 0x1402bf9b0> as *const (), Throws::Discard), // sub_140884770
    Replacement::draft(0x140884840, "content_140884840", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143272a28, 0x8> as *const ()), // sub_140884840
    Replacement::draft(0x1408848d0, "content_1408848d0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143272a28, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1408848d0
    Replacement::draft(0x140884a20, "content_140884a20", tpl_9794::<0x14387d858, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140884a20
    Replacement::draft_throws(0x140884b00, "content_140884b00", fn_140884b00 as *const (), Throws::Discard), // sub_140884b00
    Replacement::draft(0x140884bd0, "content_140884bd0", fn_140884bd0 as *const ()), // sub_140884bd0
    Replacement::draft_throws(0x140884ca0, "content_140884ca0", fn_140884ca0 as *const (), Throws::Discard), // sub_140884ca0
    Replacement::draft_throws(0x140884d70, "content_140884d70", fn_140884d70 as *const (), Throws::Caught), // sub_140884d70
    Replacement::draft_throws(0x140884e20, "content_140884e20", fn_140884e20 as *const (), Throws::Caught), // sub_140884e20
    Replacement::draft_throws(0x140884ed0, "content_140884ed0", fn_140884ed0 as *const (), Throws::Caught), // sub_140884ed0
    Replacement::draft(0x140884f80, "content_140884f80", tpl_9801::<0x14387d858, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x10> as *const ()), // sub_140884f80
    Replacement::draft(0x140885040, "content_140885040", tpl_9776::<0x14387d77c, 0x14387d858, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0x88, 0x10> as *const ()), // sub_140885040
    Replacement::draft(0x1408850f0, "content_1408850f0", tpl_9770::<0x14387d858, 0x14243b0b0, 0x14387d870, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x88, 0x10> as *const ()), // sub_1408850f0
    Replacement::draft_throws(0x1408851c0, "content_1408851c0", fn_1408851c0 as *const (), Throws::Discard), // sub_1408851c0
    Replacement::draft_throws(0x140885280, "content_140885280", tpl_9845::<0x14387d858, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d870, 0x8, 0x143251120, 0x140827080> as *const (), Throws::Discard), // sub_140885280
    Replacement::draft(0x140885350, "content_140885350", tpl_9749::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14326fa18> as *const ()), // sub_140885350
    Replacement::draft(0x1408853c0, "content_1408853c0", tpl_9750::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14326fa18> as *const ()), // sub_1408853c0
    Replacement::draft(0x1408854f0, "content_1408854f0", tpl_9768::<0x14387d798, 0x14243b0b0, 0x14387d870, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_1408854f0
    Replacement::draft_throws(0x1408855d0, "content_1408855d0", tpl_9859::<0x14387d838, 0x14387d798, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x328, 0x338, 0x8, 0x369> as *const (), Throws::Discard), // sub_1408855d0
    Replacement::draft(0x140885680, "content_140885680", tpl_9860::<0x14387d798, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338> as *const ()), // sub_140885680
    Replacement::draft_throws(0x140885730, "content_140885730", tpl_9861::<0x14387d838, 0x14387d798, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369> as *const (), Throws::Discard), // sub_140885730
    Replacement::draft_throws(0x1408857f0, "content_1408857f0", fn_1408857f0 as *const (), Throws::Caught), // sub_1408857f0
    Replacement::draft_throws(0x1408858c0, "content_1408858c0", fn_1408858c0 as *const (), Throws::Caught), // sub_1408858c0
    Replacement::draft_throws(0x140885990, "content_140885990", fn_140885990 as *const (), Throws::Caught), // sub_140885990
    Replacement::draft(0x140885a60, "content_140885a60", tpl_9863::<0x14387d798, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338> as *const ()), // sub_140885a60
    Replacement::draft(0x140885b10, "content_140885b10", tpl_9757::<0x14387d838, 0x14387d798, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x338, 0x8, 0x88> as *const ()), // sub_140885b10
    Replacement::draft(0x140885bb0, "content_140885bb0", tpl_9753::<0x14387d798, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x88> as *const ()), // sub_140885bb0
    Replacement::draft_throws(0x140885c60, "content_140885c60", tpl_9777::<0x14387d838, 0x14387d798, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_140885c60
    Replacement::draft_throws(0x140885d00, "content_140885d00", fn_140885d00 as *const (), Throws::Discard), // sub_140885d00
    Replacement::draft(0x140885df0, "content_140885df0", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432712d8, 0x8> as *const ()), // sub_140885df0
    Replacement::draft(0x140885e80, "content_140885e80", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432712d8, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140885e80
    Replacement::draft(0x140885f30, "content_140885f30", tpl_9856::<0x14387d750, 0x14243b0b0, 0x14387d870, 0x8, 0x14387d898, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140885f30
    Replacement::draft(0x140886010, "content_140886010", tpl_9838::<0x14387d750, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089e660> as *const ()), // sub_140886010
    Replacement::draft_throws(0x1408860c0, "content_1408860c0", tpl_9846::<0x14387d860, 0x14387d750, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088ba70> as *const (), Throws::Discard), // sub_1408860c0
    Replacement::draft(0x140886150, "content_140886150", tpl_9838::<0x14387d750, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089e8d0> as *const ()), // sub_140886150
    Replacement::draft_throws(0x140886200, "content_140886200", tpl_9846::<0x14387d860, 0x14387d750, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088bcd0> as *const (), Throws::Discard), // sub_140886200
    Replacement::draft_throws(0x140886290, "content_140886290", fn_140886290 as *const (), Throws::Caught), // sub_140886290
    Replacement::draft_throws(0x140886340, "content_140886340", fn_140886340 as *const (), Throws::Caught), // sub_140886340
    Replacement::draft_throws(0x1408863f0, "content_1408863f0", fn_1408863f0 as *const (), Throws::Caught), // sub_1408863f0
    Replacement::draft(0x1408864a0, "content_1408864a0", tpl_9841::<0x14387d750, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089e840> as *const ()), // sub_1408864a0
    Replacement::draft(0x140886560, "content_140886560", tpl_9842::<0x14387d860, 0x14387d750, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089ea50> as *const ()), // sub_140886560
    Replacement::draft(0x1408865f0, "content_1408865f0", tpl_9838::<0x14387d750, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089eae0> as *const ()), // sub_1408865f0
    Replacement::draft_throws(0x1408866a0, "content_1408866a0", tpl_9843::<0x14387d860, 0x14387d750, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089eb80> as *const (), Throws::Discard), // sub_1408866a0
    Replacement::draft_throws(0x140886730, "content_140886730", fn_140886730 as *const (), Throws::Caught), // sub_140886730
    Replacement::draft(0x140886850, "content_140886850", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143271438, 0x8> as *const ()), // sub_140886850
    Replacement::draft(0x1408868d0, "content_1408868d0", tpl_9767::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143271438, 0xfffffffffffffff8> as *const ()), // sub_1408868d0
    Replacement::draft(0x140886960, "content_140886960", tpl_9856::<0x14387d7b0, 0x14243b0b0, 0x14387d870, 0x8, 0x14387d8a0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140886960
    Replacement::draft(0x140886a40, "content_140886a40", tpl_9838::<0x14387d7b0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089ec20> as *const ()), // sub_140886a40
    Replacement::draft_throws(0x140886af0, "content_140886af0", tpl_9846::<0x14387d758, 0x14387d7b0, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088c650> as *const (), Throws::Discard), // sub_140886af0
    Replacement::draft(0x140886b80, "content_140886b80", tpl_9838::<0x14387d7b0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089ee20> as *const ()), // sub_140886b80
    Replacement::draft_throws(0x140886c30, "content_140886c30", tpl_9846::<0x14387d758, 0x14387d7b0, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088c8b0> as *const (), Throws::Discard), // sub_140886c30
    Replacement::draft_throws(0x140886cc0, "content_140886cc0", fn_140886cc0 as *const (), Throws::Caught), // sub_140886cc0
    Replacement::draft_throws(0x140886d70, "content_140886d70", fn_140886d70 as *const (), Throws::Caught), // sub_140886d70
    Replacement::draft_throws(0x140886e20, "content_140886e20", fn_140886e20 as *const (), Throws::Caught), // sub_140886e20
    Replacement::draft(0x140886ed0, "content_140886ed0", tpl_9841::<0x14387d7b0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089eda0> as *const ()), // sub_140886ed0
    Replacement::draft(0x140886f90, "content_140886f90", tpl_9842::<0x14387d758, 0x14387d7b0, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089ef60> as *const ()), // sub_140886f90
    Replacement::draft(0x140887020, "content_140887020", tpl_9838::<0x14387d7b0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089efe0> as *const ()), // sub_140887020
    Replacement::draft_throws(0x1408870d0, "content_1408870d0", tpl_9843::<0x14387d758, 0x14387d7b0, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089f070> as *const (), Throws::Discard), // sub_1408870d0
    Replacement::draft_throws(0x140887160, "content_140887160", fn_140887160 as *const (), Throws::Caught), // sub_140887160
    Replacement::draft(0x1408872f0, "content_1408872f0", fn_1408872f0 as *const ()), // sub_1408872f0
    Replacement::draft(0x1408873b0, "content_1408873b0", fn_1408873b0 as *const ()), // sub_1408873b0
    Replacement::draft(0x140887490, "content_140887490", tpl_9856::<0x14387d7c0, 0x14243b0b0, 0x14387d870, 0x8, 0x14387d948, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140887490
    Replacement::draft(0x140887570, "content_140887570", tpl_9838::<0x14387d7c0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089f100> as *const ()), // sub_140887570
    Replacement::draft_throws(0x140887620, "content_140887620", tpl_9846::<0x14387d774, 0x14387d7c0, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088d230> as *const (), Throws::Discard), // sub_140887620
    Replacement::draft(0x1408876b0, "content_1408876b0", tpl_9838::<0x14387d7c0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089f3c0> as *const ()), // sub_1408876b0
    Replacement::draft_throws(0x140887760, "content_140887760", tpl_9846::<0x14387d774, 0x14387d7c0, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088d4b0> as *const (), Throws::Discard), // sub_140887760
    Replacement::draft_throws(0x1408877f0, "content_1408877f0", fn_1408877f0 as *const (), Throws::Caught), // sub_1408877f0
    Replacement::draft_throws(0x1408878a0, "content_1408878a0", fn_1408878a0 as *const (), Throws::Caught), // sub_1408878a0
    Replacement::draft_throws(0x140887950, "content_140887950", fn_140887950 as *const (), Throws::Caught), // sub_140887950
    Replacement::draft(0x140887a00, "content_140887a00", tpl_9841::<0x14387d7c0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089f2f0> as *const ()), // sub_140887a00
    Replacement::draft(0x140887ac0, "content_140887ac0", tpl_9842::<0x14387d774, 0x14387d7c0, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089f540> as *const ()), // sub_140887ac0
    Replacement::draft(0x140887b50, "content_140887b50", tpl_9838::<0x14387d7c0, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089f5c0> as *const ()), // sub_140887b50
    Replacement::draft_throws(0x140887c00, "content_140887c00", tpl_9843::<0x14387d774, 0x14387d7c0, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089f650> as *const (), Throws::Discard), // sub_140887c00
    Replacement::draft_throws(0x140887c90, "content_140887c90", fn_140887c90 as *const (), Throws::Caught), // sub_140887c90
    Replacement::draft(0x140887e40, "content_140887e40", fn_140887e40 as *const ()), // sub_140887e40
    Replacement::draft(0x140887ee0, "content_140887ee0", fn_140887ee0 as *const ()), // sub_140887ee0
    Replacement::draft(0x140887f90, "content_140887f90", tpl_9856::<0x14387d7e8, 0x14243b0b0, 0x14387d870, 0x8, 0x14387d8d0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140887f90
    Replacement::draft(0x140888070, "content_140888070", tpl_9838::<0x14387d7e8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089f6c0> as *const ()), // sub_140888070
    Replacement::draft_throws(0x140888120, "content_140888120", tpl_9846::<0x14387d7f0, 0x14387d7e8, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088dee0> as *const (), Throws::Discard), // sub_140888120
    Replacement::draft(0x1408881b0, "content_1408881b0", tpl_9838::<0x14387d7e8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089f8e0> as *const ()), // sub_1408881b0
    Replacement::draft_throws(0x140888260, "content_140888260", tpl_9846::<0x14387d7f0, 0x14387d7e8, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088e150> as *const (), Throws::Discard), // sub_140888260
    Replacement::draft_throws(0x1408882f0, "content_1408882f0", fn_1408882f0 as *const (), Throws::Caught), // sub_1408882f0
    Replacement::draft_throws(0x1408883a0, "content_1408883a0", fn_1408883a0 as *const (), Throws::Caught), // sub_1408883a0
    Replacement::draft_throws(0x140888450, "content_140888450", fn_140888450 as *const (), Throws::Caught), // sub_140888450
    Replacement::draft(0x140888500, "content_140888500", tpl_9841::<0x14387d7e8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089f830> as *const ()), // sub_140888500
    Replacement::draft(0x1408885c0, "content_1408885c0", tpl_9842::<0x14387d7f0, 0x14387d7e8, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089fa10> as *const ()), // sub_1408885c0
    Replacement::draft(0x140888650, "content_140888650", tpl_9838::<0x14387d7e8, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089fa80> as *const ()), // sub_140888650
    Replacement::draft_throws(0x140888700, "content_140888700", tpl_9843::<0x14387d7f0, 0x14387d7e8, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089fb00> as *const (), Throws::Discard), // sub_140888700
    Replacement::draft_throws(0x140888790, "content_140888790", fn_140888790 as *const (), Throws::Caught), // sub_140888790
    Replacement::draft(0x140888930, "content_140888930", fn_140888930 as *const ()), // sub_140888930
    Replacement::draft(0x1408889b0, "content_1408889b0", fn_1408889b0 as *const ()), // sub_1408889b0
    Replacement::draft(0x140888a40, "content_140888a40", tpl_9856::<0x14387d720, 0x14243b0b0, 0x14387d870, 0x8, 0x14387d970, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140888a40
    Replacement::draft(0x140888b20, "content_140888b20", tpl_9838::<0x14387d720, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089fb70> as *const ()), // sub_140888b20
    Replacement::draft_throws(0x140888bd0, "content_140888bd0", tpl_9846::<0x14387d878, 0x14387d720, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088eb80> as *const (), Throws::Discard), // sub_140888bd0
    Replacement::draft(0x140888c60, "content_140888c60", tpl_9838::<0x14387d720, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089fd70> as *const ()), // sub_140888c60
    Replacement::draft_throws(0x140888d10, "content_140888d10", tpl_9846::<0x14387d878, 0x14387d720, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088ede0> as *const (), Throws::Discard), // sub_140888d10
    Replacement::draft_throws(0x140888da0, "content_140888da0", fn_140888da0 as *const (), Throws::Caught), // sub_140888da0
    Replacement::draft_throws(0x140888e50, "content_140888e50", fn_140888e50 as *const (), Throws::Caught), // sub_140888e50
    Replacement::draft_throws(0x140888f00, "content_140888f00", fn_140888f00 as *const (), Throws::Caught), // sub_140888f00
    Replacement::draft(0x140888fb0, "content_140888fb0", tpl_9841::<0x14387d720, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x14089fcf0> as *const ()), // sub_140888fb0
    Replacement::draft(0x140889070, "content_140889070", tpl_9842::<0x14387d878, 0x14387d720, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x14089feb0> as *const ()), // sub_140889070
    Replacement::draft(0x140889100, "content_140889100", tpl_9838::<0x14387d720, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089ff30> as *const ()), // sub_140889100
    Replacement::draft_throws(0x1408891b0, "content_1408891b0", tpl_9843::<0x14387d878, 0x14387d720, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x14089ffc0> as *const (), Throws::Discard), // sub_1408891b0
    Replacement::draft_throws(0x140889240, "content_140889240", fn_140889240 as *const (), Throws::Caught), // sub_140889240
    Replacement::draft(0x1408893d0, "content_1408893d0", fn_1408893d0 as *const ()), // sub_1408893d0
    Replacement::draft(0x140889450, "content_140889450", fn_140889450 as *const ()), // sub_140889450
    Replacement::draft(0x1408894f0, "content_1408894f0", tpl_9838::<0x14387d818, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1408a0050> as *const ()), // sub_1408894f0
    Replacement::draft(0x1408895a0, "content_1408895a0", tpl_9838::<0x14387d818, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1408a00f0> as *const ()), // sub_1408895a0
    Replacement::draft_throws(0x140889650, "content_140889650", tpl_9846::<0x14387d714, 0x14387d818, 0x1401e9fa0, 0x14387d764, 0x14387d870, 0x1401e9e70, 0x8, 0x14088f760> as *const (), Throws::Discard), // sub_140889650
    Replacement::draft_throws(0x140889790, "content_140889790", tpl_9846::<0x14387d714, 0x14387d818, 0x1401e9b60, 0x14387d764, 0x14387d870, 0x1401e9a20, 0x8, 0x14088faf0> as *const (), Throws::Discard), // sub_140889790
    Replacement::draft_throws(0x140889820, "content_140889820", fn_140889820 as *const (), Throws::Caught), // sub_140889820
    Replacement::draft_throws(0x1408898d0, "content_1408898d0", fn_1408898d0 as *const (), Throws::Caught), // sub_1408898d0
    Replacement::draft_throws(0x140889980, "content_140889980", fn_140889980 as *const (), Throws::Caught), // sub_140889980
    Replacement::draft(0x140889a30, "content_140889a30", tpl_9841::<0x14387d818, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x1408a0230> as *const ()), // sub_140889a30
    Replacement::draft(0x140889af0, "content_140889af0", tpl_9842::<0x14387d714, 0x14387d818, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x1408a03e0> as *const ()), // sub_140889af0
    Replacement::draft(0x140889b80, "content_140889b80", tpl_9838::<0x14387d818, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1408a0490> as *const ()), // sub_140889b80
    Replacement::draft_throws(0x140889c30, "content_140889c30", tpl_9843::<0x14387d714, 0x14387d818, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x1408a0510> as *const (), Throws::Discard), // sub_140889c30
    Replacement::draft_throws(0x140889cc0, "content_140889cc0", fn_140889cc0 as *const (), Throws::Caught), // sub_140889cc0
    Replacement::draft(0x140889e60, "content_140889e60", fn_140889e60 as *const ()), // sub_140889e60
    Replacement::draft(0x140889ee0, "content_140889ee0", fn_140889ee0 as *const ()), // sub_140889ee0
    Replacement::draft(0x140889f70, "content_140889f70", tpl_9856::<0x14387d790, 0x14243b0b0, 0x14387d870, 0x8, 0x14387d8f0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140889f70
    Replacement::draft(0x14088a050, "content_14088a050", tpl_9838::<0x14387d790, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1408a0590> as *const ()), // sub_14088a050
    Replacement::draft_throws(0x14088a100, "content_14088a100", fn_14088a100 as *const (), Throws::Caught), // sub_14088a100
    Replacement::draft(0x14088a2b0, "content_14088a2b0", tpl_9838::<0x14387d790, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1408a0830> as *const ()), // sub_14088a2b0
    Replacement::draft_throws(0x14088a360, "content_14088a360", fn_14088a360 as *const (), Throws::Caught), // sub_14088a360
    Replacement::draft_throws(0x14088a510, "content_14088a510", fn_14088a510 as *const (), Throws::Caught), // sub_14088a510
    Replacement::draft_throws(0x14088a5c0, "content_14088a5c0", fn_14088a5c0 as *const (), Throws::Caught), // sub_14088a5c0
    Replacement::draft_throws(0x14088a670, "content_14088a670", fn_14088a670 as *const (), Throws::Caught), // sub_14088a670
    Replacement::draft(0x14088a720, "content_14088a720", tpl_9841::<0x14387d790, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x1408a0700> as *const ()), // sub_14088a720
    Replacement::draft(0x14088a7e0, "content_14088a7e0", tpl_9842::<0x14387d800, 0x14387d790, 0x1401e9970, 0x14387d764, 0x14387d870, 0x1401e9860, 0x8, 0x1408a0960> as *const ()), // sub_14088a7e0
    Replacement::draft(0x14088a870, "content_14088a870", tpl_9838::<0x14387d790, 0x14243b0b0, 0x14387d870, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1408a09f0> as *const ()), // sub_14088a870
    Replacement::draft_throws(0x14088a920, "content_14088a920", tpl_9843::<0x14387d800, 0x14387d790, 0x1401e9770, 0x14387d764, 0x14387d870, 0x1401e9590, 0x8, 0x1408a0a90> as *const (), Throws::Discard), // sub_14088a920
    Replacement::draft_throws(0x14088a9b0, "content_14088a9b0", fn_14088a9b0 as *const (), Throws::Caught), // sub_14088a9b0
    Replacement::draft(0x14088ab10, "content_14088ab10", fn_14088ab10 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::content::GfxGraphValue<content::gfxgraph::Temporar
    Replacement::draft(0x14088ab50, "content_14088ab50", fn_14088ab50 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryEngineShaderRef>::content::GfxGraphValue<content::gfxgraph::Temporary
    Replacement::draft(0x14088aba0, "content_14088aba0", fn_14088aba0 as *const ()), // content::GfxGraphValue<r::GlobalIDRef<content::Curve>>::content::GfxGraphValue<r::GlobalIDRef<content::Curve>>::vfunc_0
    Replacement::draft(0x14088abd0, "content_14088abd0", fn_14088abd0 as *const ()), // content::GfxGraphValue<r::InplaceString<128>>::content::GfxGraphValue<r::InplaceString<128>>::vfunc_0
    Replacement::draft_throws(0x14088ac10, "content_14088ac10", fn_14088ac10 as *const (), Throws::Caught), // sub_14088ac10
    Replacement::draft_throws(0x14088b020, "content_14088b020", fn_14088b020 as *const (), Throws::Caught), // sub_14088b020
    Replacement::draft_throws(0x14088b150, "content_14088b150", fn_14088b150 as *const (), Throws::Caught), // sub_14088b150
    Replacement::draft_throws(0x14088b280, "content_14088b280", fn_14088b280 as *const (), Throws::Caught), // sub_14088b280
    Replacement::draft_throws(0x14088b370, "content_14088b370", fn_14088b370 as *const (), Throws::Caught), // sub_14088b370
    Replacement::draft_throws(0x14088b4a0, "content_14088b4a0", fn_14088b4a0 as *const (), Throws::Caught), // sub_14088b4a0
    Replacement::draft_throws(0x14088b5d0, "content_14088b5d0", fn_14088b5d0 as *const (), Throws::Caught), // sub_14088b5d0
    Replacement::draft_throws(0x14088b6c0, "content_14088b6c0", fn_14088b6c0 as *const (), Throws::Caught), // sub_14088b6c0
    Replacement::draft_throws(0x14088b7f0, "content_14088b7f0", fn_14088b7f0 as *const (), Throws::Caught), // sub_14088b7f0
    Replacement::draft_throws(0x14088b920, "content_14088b920", fn_14088b920 as *const (), Throws::Caught), // sub_14088b920
    Replacement::draft(0x14088ba10, "content_14088ba10", fn_14088ba10 as *const ()), // sub_14088ba10
    Replacement::draft_throws(0x14088ba70, "content_14088ba70", fn_14088ba70 as *const (), Throws::Caught), // sub_14088ba70
    Replacement::draft_throws(0x14088bcd0, "content_14088bcd0", fn_14088bcd0 as *const (), Throws::Caught), // sub_14088bcd0
    Replacement::draft_throws(0x14088bf30, "content_14088bf30", fn_14088bf30 as *const (), Throws::Caught), // sub_14088bf30
    Replacement::draft_throws(0x14088c190, "content_14088c190", fn_14088c190 as *const (), Throws::Caught), // sub_14088c190
    Replacement::draft_throws(0x14088c3f0, "content_14088c3f0", fn_14088c3f0 as *const (), Throws::Caught), // sub_14088c3f0
    Replacement::draft_throws(0x14088c650, "content_14088c650", fn_14088c650 as *const (), Throws::Caught), // sub_14088c650
    Replacement::draft_throws(0x14088c8b0, "content_14088c8b0", fn_14088c8b0 as *const (), Throws::Caught), // sub_14088c8b0
    Replacement::draft_throws(0x14088cb10, "content_14088cb10", fn_14088cb10 as *const (), Throws::Caught), // sub_14088cb10
    Replacement::draft_throws(0x14088cd70, "content_14088cd70", fn_14088cd70 as *const (), Throws::Caught), // sub_14088cd70
    Replacement::draft_throws(0x14088cfd0, "content_14088cfd0", fn_14088cfd0 as *const (), Throws::Caught), // sub_14088cfd0
    Replacement::draft_throws(0x14088d230, "content_14088d230", fn_14088d230 as *const (), Throws::Caught), // sub_14088d230
    Replacement::draft_throws(0x14088d4b0, "content_14088d4b0", fn_14088d4b0 as *const (), Throws::Caught), // sub_14088d4b0
    Replacement::draft_throws(0x14088d730, "content_14088d730", fn_14088d730 as *const (), Throws::Caught), // sub_14088d730
    Replacement::draft_throws(0x14088d9c0, "content_14088d9c0", fn_14088d9c0 as *const (), Throws::Caught), // sub_14088d9c0
    Replacement::draft_throws(0x14088dc50, "content_14088dc50", fn_14088dc50 as *const (), Throws::Caught), // sub_14088dc50
    Replacement::draft_throws(0x14088dee0, "content_14088dee0", fn_14088dee0 as *const (), Throws::Caught), // sub_14088dee0
    Replacement::draft_throws(0x14088e150, "content_14088e150", fn_14088e150 as *const (), Throws::Caught), // sub_14088e150
    Replacement::draft_throws(0x14088e3c0, "content_14088e3c0", fn_14088e3c0 as *const (), Throws::Caught), // sub_14088e3c0
    Replacement::draft_throws(0x14088e640, "content_14088e640", fn_14088e640 as *const (), Throws::Caught), // sub_14088e640
    Replacement::draft_throws(0x14088e8c0, "content_14088e8c0", fn_14088e8c0 as *const (), Throws::Caught), // sub_14088e8c0
    Replacement::draft(0x14088eb40, "content_14088eb40", fn_14088eb40 as *const ()), // sub_14088eb40
    Replacement::draft_throws(0x14088eb80, "content_14088eb80", fn_14088eb80 as *const (), Throws::Caught), // sub_14088eb80
    Replacement::draft_throws(0x14088ede0, "content_14088ede0", fn_14088ede0 as *const (), Throws::Caught), // sub_14088ede0
    Replacement::draft_throws(0x14088f040, "content_14088f040", fn_14088f040 as *const (), Throws::Caught), // sub_14088f040
    Replacement::draft_throws(0x14088f2a0, "content_14088f2a0", fn_14088f2a0 as *const (), Throws::Caught), // sub_14088f2a0
    Replacement::draft_throws(0x14088f500, "content_14088f500", fn_14088f500 as *const (), Throws::Caught), // sub_14088f500
    Replacement::draft_throws(0x14088f760, "content_14088f760", fn_14088f760 as *const (), Throws::Caught), // sub_14088f760
    Replacement::draft_throws(0x14088faf0, "content_14088faf0", fn_14088faf0 as *const (), Throws::Caught), // sub_14088faf0
    Replacement::draft_throws(0x14088fe80, "content_14088fe80", fn_14088fe80 as *const (), Throws::Caught), // sub_14088fe80
    Replacement::draft_throws(0x140890220, "content_140890220", fn_140890220 as *const (), Throws::Caught), // sub_140890220
    Replacement::draft_throws(0x1408905c0, "content_1408905c0", fn_1408905c0 as *const (), Throws::Caught), // sub_1408905c0
    Replacement::draft(0x140890960, "content_140890960", fn_140890960 as *const ()), // sub_140890960
    Replacement::draft_throws(0x1408909b0, "content_1408909b0", fn_1408909b0 as *const (), Throws::Caught), // sub_1408909b0
    Replacement::draft_throws(0x140890af0, "content_140890af0", fn_140890af0 as *const (), Throws::Caught), // sub_140890af0
    Replacement::draft_throws(0x140890c30, "content_140890c30", fn_140890c30 as *const (), Throws::Caught), // sub_140890c30
    Replacement::draft_throws(0x140890d70, "content_140890d70", fn_140890d70 as *const (), Throws::Caught), // sub_140890d70
    Replacement::draft_throws(0x140891320, "content_140891320", fn_140891320 as *const (), Throws::Caught), // sub_140891320
    Replacement::draft_throws(0x140891890, "content_140891890", fn_140891890 as *const (), Throws::Caught), // sub_140891890
    Replacement::draft_throws(0x140891cd0, "content_140891cd0", fn_140891cd0 as *const (), Throws::Caught), // sub_140891cd0
    Replacement::draft_throws(0x140892280, "content_140892280", fn_140892280 as *const (), Throws::Caught), // sub_140892280
    Replacement::draft_throws(0x1408927f0, "content_1408927f0", fn_1408927f0 as *const (), Throws::Caught), // sub_1408927f0
    Replacement::draft_throws(0x140892c30, "content_140892c30", fn_140892c30 as *const (), Throws::Caught), // sub_140892c30
    Replacement::draft_throws(0x1408931e0, "content_1408931e0", fn_1408931e0 as *const (), Throws::Caught), // sub_1408931e0
    Replacement::draft_throws(0x140893750, "content_140893750", fn_140893750 as *const (), Throws::Caught), // sub_140893750
    Replacement::draft_throws(0x140893b90, "content_140893b90", fn_140893b90 as *const (), Throws::Caught), // sub_140893b90
    Replacement::draft_throws(0x140894230, "content_140894230", fn_140894230 as *const (), Throws::Caught), // sub_140894230
    Replacement::draft_throws(0x140894890, "content_140894890", fn_140894890 as *const (), Throws::Caught), // sub_140894890
    Replacement::draft_throws(0x140894cd0, "content_140894cd0", fn_140894cd0 as *const (), Throws::Caught), // sub_140894cd0
    Replacement::draft_throws(0x140895360, "content_140895360", fn_140895360 as *const (), Throws::Caught), // sub_140895360
    Replacement::draft_throws(0x1408959c0, "content_1408959c0", fn_1408959c0 as *const (), Throws::Caught), // sub_1408959c0
    Replacement::draft_throws(0x140895e00, "content_140895e00", fn_140895e00 as *const (), Throws::Caught), // sub_140895e00
    Replacement::draft_throws(0x1408964d0, "content_1408964d0", fn_1408964d0 as *const (), Throws::Caught), // sub_1408964d0
    Replacement::draft_throws(0x140896b70, "content_140896b70", fn_140896b70 as *const (), Throws::Caught), // sub_140896b70
    Replacement::draft_throws(0x140896fb0, "content_140896fb0", fn_140896fb0 as *const (), Throws::Caught), // sub_140896fb0
    Replacement::draft_throws(0x140897810, "content_140897810", fn_140897810 as *const (), Throws::Caught), // sub_140897810
    Replacement::draft_throws(0x140898000, "content_140898000", fn_140898000 as *const (), Throws::Caught), // sub_140898000
    Replacement::draft_throws(0x1408986e0, "content_1408986e0", fn_1408986e0 as *const (), Throws::Caught), // sub_1408986e0
    Replacement::draft_throws(0x140898d80, "content_140898d80", fn_140898d80 as *const (), Throws::Caught), // sub_140898d80
    Replacement::draft_throws(0x1408993e0, "content_1408993e0", fn_1408993e0 as *const (), Throws::Caught), // sub_1408993e0
    Replacement::draft_throws(0x140899820, "content_140899820", fn_140899820 as *const (), Throws::Caught), // sub_140899820
    Replacement::draft_throws(0x140899fc0, "content_140899fc0", fn_140899fc0 as *const (), Throws::Caught), // sub_140899fc0
    Replacement::draft_throws(0x14089a730, "content_14089a730", fn_14089a730 as *const (), Throws::Caught), // sub_14089a730
    Replacement::draft_throws(0x14089ab70, "content_14089ab70", fn_14089ab70 as *const (), Throws::Caught), // sub_14089ab70
    Replacement::draft_throws(0x14089b340, "content_14089b340", fn_14089b340 as *const (), Throws::Caught), // sub_14089b340
    Replacement::draft_throws(0x14089baa0, "content_14089baa0", fn_14089baa0 as *const (), Throws::Caught), // sub_14089baa0
    Replacement::draft(0x14089c0e0, "content_14089c0e0", fn_14089c0e0 as *const ()), // sub_14089c0e0
    Replacement::draft_throws(0x14089c2a0, "content_14089c2a0", fn_14089c2a0 as *const (), Throws::Caught), // sub_14089c2a0
    Replacement::draft(0x14089c450, "content_14089c450", fn_14089c450 as *const ()), // sub_14089c450
    Replacement::draft(0x14089c5e0, "content_14089c5e0", fn_14089c5e0 as *const ()), // sub_14089c5e0
    Replacement::draft_throws(0x14089c720, "content_14089c720", fn_14089c720 as *const (), Throws::Caught), // sub_14089c720
    Replacement::draft(0x14089c8d0, "content_14089c8d0", fn_14089c8d0 as *const ()), // sub_14089c8d0
    Replacement::draft(0x14089ca40, "content_14089ca40", fn_14089ca40 as *const ()), // sub_14089ca40
    Replacement::draft_throws(0x14089cb90, "content_14089cb90", fn_14089cb90 as *const (), Throws::Caught), // sub_14089cb90
    Replacement::draft(0x14089ccc0, "content_14089ccc0", fn_14089ccc0 as *const ()), // sub_14089ccc0
    Replacement::draft(0x14089cd60, "content_14089cd60", fn_14089cd60 as *const ()), // sub_14089cd60
    Replacement::draft_throws(0x14089cf20, "content_14089cf20", fn_14089cf20 as *const (), Throws::Caught), // sub_14089cf20
    Replacement::draft(0x14089d0d0, "content_14089d0d0", fn_14089d0d0 as *const ()), // sub_14089d0d0
    Replacement::draft(0x14089d260, "content_14089d260", fn_14089d260 as *const ()), // sub_14089d260
    Replacement::draft_throws(0x14089d3a0, "content_14089d3a0", fn_14089d3a0 as *const (), Throws::Caught), // sub_14089d3a0
    Replacement::draft(0x14089d550, "content_14089d550", fn_14089d550 as *const ()), // sub_14089d550
    Replacement::draft(0x14089d6c0, "content_14089d6c0", fn_14089d6c0 as *const ()), // sub_14089d6c0
    Replacement::draft_throws(0x14089d810, "content_14089d810", fn_14089d810 as *const (), Throws::Caught), // sub_14089d810
    Replacement::draft(0x14089d940, "content_14089d940", fn_14089d940 as *const ()), // sub_14089d940
    Replacement::draft(0x14089d9e0, "content_14089d9e0", fn_14089d9e0 as *const ()), // sub_14089d9e0
    Replacement::draft_throws(0x14089dba0, "content_14089dba0", fn_14089dba0 as *const (), Throws::Caught), // sub_14089dba0
    Replacement::draft(0x14089dd50, "content_14089dd50", fn_14089dd50 as *const ()), // sub_14089dd50
    Replacement::draft(0x14089dee0, "content_14089dee0", fn_14089dee0 as *const ()), // sub_14089dee0
    Replacement::draft_throws(0x14089e020, "content_14089e020", fn_14089e020 as *const (), Throws::Caught), // sub_14089e020
    Replacement::draft(0x14089e1d0, "content_14089e1d0", fn_14089e1d0 as *const ()), // sub_14089e1d0
    Replacement::draft(0x14089e340, "content_14089e340", fn_14089e340 as *const ()), // sub_14089e340
    Replacement::draft_throws(0x14089e490, "content_14089e490", fn_14089e490 as *const (), Throws::Caught), // sub_14089e490
    Replacement::draft(0x14089e5c0, "content_14089e5c0", fn_14089e5c0 as *const ()), // sub_14089e5c0
    Replacement::draft(0x14089e660, "content_14089e660", fn_14089e660 as *const ()), // sub_14089e660
    Replacement::draft_throws(0x14089e760, "content_14089e760", fn_14089e760 as *const (), Throws::Discard), // sub_14089e760
    Replacement::draft(0x14089e840, "content_14089e840", fn_14089e840 as *const ()), // sub_14089e840
    Replacement::draft(0x14089e8d0, "content_14089e8d0", fn_14089e8d0 as *const ()), // sub_14089e8d0
    Replacement::draft_throws(0x14089e960, "content_14089e960", fn_14089e960 as *const (), Throws::Discard), // sub_14089e960
    Replacement::draft(0x14089ea50, "content_14089ea50", tpl_68520::<0x14387d880, 0x14387d898, 0x1401e9970, 0x338, 0x88, 0x8, 0xe0, 0xc, 0x1401e9860> as *const ()), // sub_14089ea50
    Replacement::draft(0x14089eae0, "content_14089eae0", fn_14089eae0 as *const ()), // sub_14089eae0
    Replacement::draft_throws(0x14089eb80, "content_14089eb80", fn_14089eb80 as *const (), Throws::Discard), // sub_14089eb80
    Replacement::draft(0x14089ec20, "content_14089ec20", fn_14089ec20 as *const ()), // sub_14089ec20
    Replacement::draft_throws(0x14089ecf0, "content_14089ecf0", tpl_9957::<0x14387d8e8, 0x14387d8a0, 0x1401e9fa0, 0x328, 0x338, 0x40, 0x369, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_14089ecf0
    Replacement::draft(0x14089eda0, "content_14089eda0", fn_14089eda0 as *const ()), // sub_14089eda0
    Replacement::draft(0x14089ee20, "content_14089ee20", tpl_9959::<0x14387d8a0, 0x14243b0b0, 0x338, 0x40, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089ee20
    Replacement::draft_throws(0x14089eea0, "content_14089eea0", fn_14089eea0 as *const (), Throws::Discard), // sub_14089eea0
    Replacement::draft(0x14089ef60, "content_14089ef60", tpl_9961::<0x14387d8e8, 0x14387d8a0, 0x1401e9970, 0x338, 0xe0, 0x4, 0x1401e9860> as *const ()), // sub_14089ef60
    Replacement::draft(0x14089efe0, "content_14089efe0", tpl_9959::<0x14387d8a0, 0x14243b0b0, 0x338, 0xe0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089efe0
    Replacement::draft_throws(0x14089f070, "content_14089f070", fn_14089f070 as *const (), Throws::Discard), // sub_14089f070
    Replacement::draft(0x14089f100, "content_14089f100", tpl_9963::<0x14387d948, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x50, 0x4b, 0x4c, 0x48, 0xfffffffffffffff8> as *const ()), // sub_14089f100
    Replacement::draft_throws(0x14089f1f0, "content_14089f1f0", tpl_68527::<0x14387d91c, 0x14387d948, 0x1401e9fa0, 0x328, 0x1426cdcb0, 0x338, 0x20, 0x369, 0x142409b50, 0x48, 0x1401e9e70> as *const (), Throws::Discard), // sub_14089f1f0
    Replacement::draft(0x14089f2f0, "content_14089f2f0", tpl_9964::<0x14387d948, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089f2f0
    Replacement::draft(0x14089f3c0, "content_14089f3c0", tpl_9959::<0x14387d948, 0x14243b0b0, 0x338, 0x20, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089f3c0
    Replacement::draft_throws(0x14089f440, "content_14089f440", tpl_68528::<0x14387d91c, 0x14387d948, 0x1401e9b60, 0x328, 0x1426cdcb0, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x48, 0x1401e9a20> as *const (), Throws::Discard), // sub_14089f440
    Replacement::draft(0x14089f540, "content_14089f540", tpl_9961::<0x14387d91c, 0x14387d948, 0x1401e9970, 0x338, 0xc0, 0x48, 0x1401e9860> as *const ()), // sub_14089f540
    Replacement::draft(0x14089f5c0, "content_14089f5c0", tpl_9959::<0x14387d948, 0x14243b0b0, 0x338, 0xc0, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089f5c0
    Replacement::draft_throws(0x14089f650, "content_14089f650", tpl_9965::<0x14387d91c, 0x14387d948, 0x1401e9770, 0x338, 0x1424b4cb0, 0x48, 0x1401e9590> as *const (), Throws::Discard), // sub_14089f650
    Replacement::draft(0x14089f6c0, "content_14089f6c0", tpl_9966::<0x14387d8d0, 0x14243b0b0, 0x14089d9e0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089f6c0
    Replacement::draft_throws(0x14089f780, "content_14089f780", fn_14089f780 as *const (), Throws::Discard), // sub_14089f780
    Replacement::draft(0x14089f830, "content_14089f830", fn_14089f830 as *const ()), // sub_14089f830
    Replacement::draft(0x14089f8e0, "content_14089f8e0", tpl_9969::<0x14387d8d0, 0x14243b0b0, 0x14089dee0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089f8e0
    Replacement::draft_throws(0x14089f960, "content_14089f960", fn_14089f960 as *const (), Throws::Discard), // sub_14089f960
    Replacement::draft(0x14089fa10, "content_14089fa10", tpl_7856::<0x14387d958, 0x14387d8d0, 0x1401e9970, 0x14089e1d0, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_14089fa10
    Replacement::draft(0x14089fa80, "content_14089fa80", tpl_9969::<0x14387d8d0, 0x14243b0b0, 0x14089e340, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089fa80
    Replacement::draft_throws(0x14089fb00, "content_14089fb00", tpl_9971::<0x14387d958, 0x14387d8d0, 0x1401e9770, 0x14089e490, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_14089fb00
    Replacement::draft(0x14089fb70, "content_14089fb70", tpl_9972::<0x14387d970, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089fb70
    Replacement::draft_throws(0x14089fc40, "content_14089fc40", tpl_9973::<0x14387d918, 0x14387d970, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x8, 0x1401e9e70> as *const (), Throws::Discard), // sub_14089fc40
    Replacement::draft(0x14089fcf0, "content_14089fcf0", tpl_9974::<0x14387d970, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089fcf0
    Replacement::draft(0x14089fd70, "content_14089fd70", tpl_68532::<0x14387d970, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089fd70
    Replacement::draft_throws(0x14089fdf0, "content_14089fdf0", tpl_9976::<0x14387d918, 0x14387d970, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x8, 0x1401e9a20> as *const (), Throws::Discard), // sub_14089fdf0
    Replacement::draft(0x14089feb0, "content_14089feb0", tpl_9961::<0x14387d918, 0x14387d970, 0x1401e9970, 0x338, 0x88, 0x8, 0x1401e9860> as *const ()), // sub_14089feb0
    Replacement::draft(0x14089ff30, "content_14089ff30", tpl_68533::<0x14387d970, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14089ff30
    Replacement::draft_throws(0x14089ffc0, "content_14089ffc0", tpl_9978::<0x14387d918, 0x14387d970, 0x1401e9770, 0x338, 0x120, 0x8, 0x1401e9590> as *const (), Throws::Discard), // sub_14089ffc0
    Replacement::draft(0x1408a00f0, "content_1408a00f0", fn_1408a00f0 as *const ()), // sub_1408a00f0
    Replacement::draft_throws(0x1408a01a0, "content_1408a01a0", fn_1408a01a0 as *const (), Throws::Discard), // sub_1408a01a0
    Replacement::draft(0x1408a0230, "content_1408a0230", fn_1408a0230 as *const ()), // sub_1408a0230
    Replacement::draft(0x1408a02c0, "content_1408a02c0", fn_1408a02c0 as *const ()), // sub_1408a02c0
    Replacement::draft_throws(0x1408a0340, "content_1408a0340", fn_1408a0340 as *const (), Throws::Discard), // sub_1408a0340
    Replacement::draft(0x1408a03e0, "content_1408a03e0", fn_1408a03e0 as *const ()), // sub_1408a03e0
    Replacement::draft(0x1408a0490, "content_1408a0490", tpl_9969::<0x14387d8e0, 0x14243b0b0, 0x14055a1a0, 0x338, 0x8, 0x88, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a0490
    Replacement::draft_throws(0x1408a0510, "content_1408a0510", tpl_9985::<0x14387d8f8, 0x14387d8e0, 0x1401e9770, 0x14055a210, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1408a0510
    Replacement::draft(0x1408a0590, "content_1408a0590", fn_1408a0590 as *const ()), // sub_1408a0590
    Replacement::draft_throws(0x1408a0640, "content_1408a0640", fn_1408a0640 as *const (), Throws::Caught), // sub_1408a0640
    Replacement::draft(0x1408a0700, "content_1408a0700", fn_1408a0700 as *const ()), // sub_1408a0700
    Replacement::draft(0x1408a0830, "content_1408a0830", fn_1408a0830 as *const ()), // sub_1408a0830
    Replacement::draft_throws(0x1408a08f0, "content_1408a08f0", fn_1408a08f0 as *const (), Throws::Discard), // sub_1408a08f0
    Replacement::draft(0x1408a0960, "content_1408a0960", fn_1408a0960 as *const ()), // sub_1408a0960
    Replacement::draft(0x1408a09f0, "content_1408a09f0", fn_1408a09f0 as *const ()), // sub_1408a09f0
    Replacement::draft_throws(0x1408a0a90, "content_1408a0a90", fn_1408a0a90 as *const (), Throws::Discard), // sub_1408a0a90
    Replacement::draft_throws(0x1408a0b20, "content_1408a0b20", fn_1408a0b20 as *const (), Throws::Caught), // sub_1408a0b20
    Replacement::draft_throws(0x1408a0cb0, "content_1408a0cb0", fn_1408a0cb0 as *const (), Throws::Caught), // sub_1408a0cb0
    Replacement::draft_throws(0x1408a0e40, "content_1408a0e40", fn_1408a0e40 as *const (), Throws::Caught), // sub_1408a0e40
    Replacement::draft_throws(0x1408a0fd0, "content_1408a0fd0", fn_1408a0fd0 as *const (), Throws::Caught), // sub_1408a0fd0
    Replacement::draft_throws(0x1408a1160, "content_1408a1160", fn_1408a1160 as *const (), Throws::Caught), // sub_1408a1160
    Replacement::draft_throws(0x1408a12f0, "content_1408a12f0", fn_1408a12f0 as *const (), Throws::Caught), // sub_1408a12f0
    Replacement::draft_throws(0x1408a1480, "content_1408a1480", fn_1408a1480 as *const (), Throws::Caught), // sub_1408a1480
    Replacement::draft_throws(0x1408a1610, "content_1408a1610", fn_1408a1610 as *const (), Throws::Caught), // sub_1408a1610
    Replacement::draft_throws(0x1408a17a0, "content_1408a17a0", fn_1408a17a0 as *const (), Throws::Caught), // sub_1408a17a0
    Replacement::draft_throws(0x1408a1930, "content_1408a1930", fn_1408a1930 as *const (), Throws::Caught), // sub_1408a1930
    Replacement::draft_throws(0x1408a1ac0, "content_1408a1ac0", fn_1408a1ac0 as *const (), Throws::Caught), // sub_1408a1ac0
    Replacement::draft_throws(0x1408a1c50, "content_1408a1c50", fn_1408a1c50 as *const (), Throws::Caught), // sub_1408a1c50
    Replacement::draft_throws(0x1408a1de0, "content_1408a1de0", fn_1408a1de0 as *const (), Throws::Caught), // sub_1408a1de0
    Replacement::draft_throws(0x1408a1f10, "content_1408a1f10", fn_1408a1f10 as *const (), Throws::Caught), // sub_1408a1f10
    Replacement::draft_throws(0x1408a2040, "content_1408a2040", fn_1408a2040 as *const (), Throws::Caught), // sub_1408a2040
    Replacement::draft_throws(0x1408a2170, "content_1408a2170", fn_1408a2170 as *const (), Throws::Discard), // sub_1408a2170
    Replacement::draft_throws(0x1408a2290, "content_1408a2290", fn_1408a2290 as *const (), Throws::Caught), // sub_1408a2290
    Replacement::draft_throws(0x1408a2390, "content_1408a2390", fn_1408a2390 as *const (), Throws::Caught), // sub_1408a2390
    Replacement::draft_throws(0x1408a2490, "content_1408a2490", fn_1408a2490 as *const (), Throws::Caught), // sub_1408a2490
    Replacement::draft_throws(0x1408a2590, "content_1408a2590", fn_1408a2590 as *const (), Throws::Discard), // sub_1408a2590
    Replacement::draft_throws(0x1408a2680, "content_1408a2680", fn_1408a2680 as *const (), Throws::Caught), // sub_1408a2680
    Replacement::draft_throws(0x1408a2740, "content_1408a2740", fn_1408a2740 as *const (), Throws::Caught), // sub_1408a2740
    Replacement::draft_throws(0x1408a2800, "content_1408a2800", fn_1408a2800 as *const (), Throws::Caught), // sub_1408a2800
    Replacement::draft_throws(0x1408a28c0, "content_1408a28c0", tpl_10011::<0x14387d948, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14326ea48, 0x1402c4730, 0x48, 0x14326ea38> as *const (), Throws::Discard), // sub_1408a28c0
    Replacement::draft_throws(0x1408a29a0, "content_1408a29a0", fn_1408a29a0 as *const (), Throws::Caught), // sub_1408a29a0
    Replacement::draft_throws(0x1408a2a90, "content_1408a2a90", fn_1408a2a90 as *const (), Throws::Caught), // sub_1408a2a90
    Replacement::draft_throws(0x1408a2b80, "content_1408a2b80", fn_1408a2b80 as *const (), Throws::Caught), // sub_1408a2b80
    Replacement::draft_throws(0x1408a2c70, "content_1408a2c70", fn_1408a2c70 as *const (), Throws::Discard), // sub_1408a2c70
    Replacement::draft_throws(0x1408a2d40, "content_1408a2d40", fn_1408a2d40 as *const (), Throws::Caught), // sub_1408a2d40
    Replacement::draft_throws(0x1408a2e40, "content_1408a2e40", fn_1408a2e40 as *const (), Throws::Caught), // sub_1408a2e40
    Replacement::draft_throws(0x1408a2f40, "content_1408a2f40", fn_1408a2f40 as *const (), Throws::Caught), // sub_1408a2f40
    Replacement::draft_throws(0x1408a3040, "content_1408a3040", tpl_10015::<0x14387d970, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14326ea58, 0x1, 0x1402c4aa0, 0x8, 0x14326ea90> as *const (), Throws::Discard), // sub_1408a3040
    Replacement::draft_throws(0x1408a3130, "content_1408a3130", fn_1408a3130 as *const (), Throws::Caught), // sub_1408a3130
    Replacement::draft_throws(0x1408a3210, "content_1408a3210", fn_1408a3210 as *const (), Throws::Caught), // sub_1408a3210
    Replacement::draft_throws(0x1408a32f0, "content_1408a32f0", fn_1408a32f0 as *const (), Throws::Caught), // sub_1408a32f0
    Replacement::draft_throws(0x1408a33d0, "content_1408a33d0", tpl_2311::<0x14387d8e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14326ea78, 0x140558dc0, 0x8, 0x14326ea90, 0x1, 0x1402c4aa0> as *const (), Throws::Discard), // sub_1408a33d0
    Replacement::draft_throws(0x1408a34a0, "content_1408a34a0", fn_1408a34a0 as *const (), Throws::Caught), // sub_1408a34a0
    Replacement::draft_throws(0x1408a3690, "content_1408a3690", fn_1408a3690 as *const (), Throws::Caught), // sub_1408a3690
    Replacement::draft_throws(0x1408a3850, "content_1408a3850", fn_1408a3850 as *const (), Throws::Caught), // sub_1408a3850
    Replacement::draft_throws(0x1408a3a10, "content_1408a3a10", fn_1408a3a10 as *const (), Throws::Discard), // sub_1408a3a10
    Replacement::draft(0x1408a3ab0, "content_1408a3ab0", fn_1408a3ab0 as *const ()), // sub_1408a3ab0
    Replacement::draft(0x1408a3c00, "content_1408a3c00", tpl_9972::<0x14387d968, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a3c00
    Replacement::draft_throws(0x1408a3cd0, "content_1408a3cd0", tpl_9973::<0x14387d894, 0x14387d968, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x8, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408a3cd0
    Replacement::draft(0x1408a3d80, "content_1408a3d80", tpl_68532::<0x14387d968, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a3d80
    Replacement::draft_throws(0x1408a3e00, "content_1408a3e00", tpl_9976::<0x14387d894, 0x14387d968, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x8, 0x1401e9a20> as *const (), Throws::Discard), // sub_1408a3e00
    Replacement::draft_throws(0x1408a3ec0, "content_1408a3ec0", fn_1408a3ec0 as *const (), Throws::Caught), // sub_1408a3ec0
    Replacement::draft_throws(0x1408a3fc0, "content_1408a3fc0", fn_1408a3fc0 as *const (), Throws::Caught), // sub_1408a3fc0
    Replacement::draft_throws(0x1408a40c0, "content_1408a40c0", fn_1408a40c0 as *const (), Throws::Caught), // sub_1408a40c0
    Replacement::draft(0x1408a41c0, "content_1408a41c0", tpl_9974::<0x14387d968, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a41c0
    Replacement::draft(0x1408a4240, "content_1408a4240", tpl_9961::<0x14387d894, 0x14387d968, 0x1401e9970, 0x338, 0x88, 0x8, 0x1401e9860> as *const ()), // sub_1408a4240
    Replacement::draft(0x1408a42c0, "content_1408a42c0", tpl_68533::<0x14387d968, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a42c0
    Replacement::draft_throws(0x1408a4350, "content_1408a4350", tpl_9978::<0x14387d894, 0x14387d968, 0x1401e9770, 0x338, 0x120, 0x8, 0x1401e9590> as *const (), Throws::Discard), // sub_1408a4350
    Replacement::draft_throws(0x1408a43e0, "content_1408a43e0", tpl_10015::<0x14387d968, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143273700, 0x1, 0x1402c4aa0, 0x8, 0x14326e9b8> as *const (), Throws::Discard), // sub_1408a43e0
    Replacement::draft(0x1408a44d0, "content_1408a44d0", fn_1408a44d0 as *const ()), // sub_1408a44d0
    Replacement::draft(0x1408a4520, "content_1408a4520", fn_1408a4520 as *const ()), // sub_1408a4520
    Replacement::draft(0x1408a4630, "content_1408a4630", fn_1408a4630 as *const ()), // sub_1408a4630
    Replacement::draft_throws(0x1408a4700, "content_1408a4700", fn_1408a4700 as *const (), Throws::Discard), // sub_1408a4700
    Replacement::draft(0x1408a47b0, "content_1408a47b0", tpl_9959::<0x14387d8a8, 0x14243b0b0, 0x338, 0x50, 0x2, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a47b0
    Replacement::draft_throws(0x1408a4830, "content_1408a4830", fn_1408a4830 as *const (), Throws::Discard), // sub_1408a4830
    Replacement::draft_throws(0x1408a4900, "content_1408a4900", fn_1408a4900 as *const (), Throws::Caught), // sub_1408a4900
    Replacement::draft_throws(0x1408a4a00, "content_1408a4a00", fn_1408a4a00 as *const (), Throws::Caught), // sub_1408a4a00
    Replacement::draft_throws(0x1408a4b00, "content_1408a4b00", fn_1408a4b00 as *const (), Throws::Caught), // sub_1408a4b00
    Replacement::draft(0x1408a4c00, "content_1408a4c00", fn_1408a4c00 as *const ()), // sub_1408a4c00
    Replacement::draft(0x1408a4c80, "content_1408a4c80", tpl_9961::<0x14387d928, 0x14387d8a8, 0x1401e9970, 0x338, 0xf0, 0x2, 0x1401e9860> as *const ()), // sub_1408a4c80
    Replacement::draft(0x1408a4d00, "content_1408a4d00", tpl_9959::<0x14387d8a8, 0x14243b0b0, 0x338, 0xf0, 0x2, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a4d00
    Replacement::draft_throws(0x1408a4d90, "content_1408a4d90", fn_1408a4d90 as *const (), Throws::Discard), // sub_1408a4d90
    Replacement::draft_throws(0x1408a4e20, "content_1408a4e20", fn_1408a4e20 as *const (), Throws::Discard), // sub_1408a4e20
    Replacement::draft(0x1408a4f10, "content_1408a4f10", tpl_10030::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8> as *const ()), // sub_1408a4f10
    Replacement::draft(0x1408a4f90, "content_1408a4f90", fn_1408a4f90 as *const ()), // sub_1408a4f90
    Replacement::draft(0x1408a5030, "content_1408a5030", fn_1408a5030 as *const ()), // sub_1408a5030
    Replacement::draft(0x1408a5100, "content_1408a5100", fn_1408a5100 as *const ()), // sub_1408a5100
    Replacement::draft_throws(0x1408a5210, "content_1408a5210", tpl_10034::<0x14387d938, 0x14387d8d8, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x48, 0x8, 0x1408b4a10, 0x1408b5040, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408a5210
    Replacement::draft(0x1408a52e0, "content_1408a52e0", fn_1408a52e0 as *const ()), // sub_1408a52e0
    Replacement::draft_throws(0x1408a53b0, "content_1408a53b0", tpl_10035::<0x14387d938, 0x14387d8d8, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x8, 0x1408b4a10, 0x1408b5300, 0x1401e9a20> as *const (), Throws::Discard), // sub_1408a53b0
    Replacement::draft(0x1408a54a0, "content_1408a54a0", fn_1408a54a0 as *const ()), // sub_1408a54a0
    Replacement::draft_throws(0x1408a5530, "content_1408a5530", fn_1408a5530 as *const (), Throws::Caught), // sub_1408a5530
    Replacement::draft_throws(0x1408a5610, "content_1408a5610", fn_1408a5610 as *const (), Throws::Caught), // sub_1408a5610
    Replacement::draft_throws(0x1408a56f0, "content_1408a56f0", fn_1408a56f0 as *const (), Throws::Caught), // sub_1408a56f0
    Replacement::draft(0x1408a57d0, "content_1408a57d0", fn_1408a57d0 as *const ()), // sub_1408a57d0
    Replacement::draft(0x1408a5890, "content_1408a5890", fn_1408a5890 as *const ()), // sub_1408a5890
    Replacement::draft(0x1408a5950, "content_1408a5950", fn_1408a5950 as *const ()), // sub_1408a5950
    Replacement::draft_throws(0x1408a5a20, "content_1408a5a20", fn_1408a5a20 as *const (), Throws::Discard), // sub_1408a5a20
    Replacement::draft_throws(0x1408a5ae0, "content_1408a5ae0", tpl_10038::<0x14387d8d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143273710, 0x1, 0x1402c4aa0, 0x8, 0x143273740, 0x1408b0910> as *const (), Throws::Discard), // sub_1408a5ae0
    Replacement::draft(0x1408a5bb0, "content_1408a5bb0", fn_1408a5bb0 as *const ()), // sub_1408a5bb0
    Replacement::draft(0x1408a5cc0, "content_1408a5cc0", fn_1408a5cc0 as *const ()), // sub_1408a5cc0
    Replacement::draft(0x1408a5d40, "content_1408a5d40", fn_1408a5d40 as *const ()), // sub_1408a5d40
    Replacement::draft_throws(0x1408a5e00, "content_1408a5e00", fn_1408a5e00 as *const (), Throws::Discard), // sub_1408a5e00
    Replacement::draft_throws(0x1408a5e90, "content_1408a5e90", fn_1408a5e90 as *const (), Throws::Discard), // sub_1408a5e90
    Replacement::draft_throws(0x1408a5f10, "content_1408a5f10", fn_1408a5f10 as *const (), Throws::Discard), // sub_1408a5f10
    Replacement::draft_throws(0x1408a5fb0, "content_1408a5fb0", fn_1408a5fb0 as *const (), Throws::Caught), // sub_1408a5fb0
    Replacement::draft_throws(0x1408a6090, "content_1408a6090", fn_1408a6090 as *const (), Throws::Caught), // sub_1408a6090
    Replacement::draft_throws(0x1408a6170, "content_1408a6170", fn_1408a6170 as *const (), Throws::Caught), // sub_1408a6170
    Replacement::draft(0x1408a6250, "content_1408a6250", fn_1408a6250 as *const ()), // sub_1408a6250
    Replacement::draft(0x1408a6300, "content_1408a6300", fn_1408a6300 as *const ()), // sub_1408a6300
    Replacement::draft(0x1408a6370, "content_1408a6370", fn_1408a6370 as *const ()), // sub_1408a6370
    Replacement::draft_throws(0x1408a63f0, "content_1408a63f0", fn_1408a63f0 as *const (), Throws::Discard), // sub_1408a63f0
    Replacement::draft_throws(0x1408a6470, "content_1408a6470", tpl_10038::<0x14387d908, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143273700, 0x1, 0x1402c4aa0, 0x8, 0x14310c38c, 0x1408b33a0> as *const (), Throws::Discard), // sub_1408a6470
    Replacement::draft(0x1408a6540, "content_1408a6540", fn_1408a6540 as *const ()), // sub_1408a6540
    Replacement::draft(0x1408a6590, "content_1408a6590", fn_1408a6590 as *const ()), // sub_1408a6590
    Replacement::draft_throws(0x1408a6690, "content_1408a6690", fn_1408a6690 as *const (), Throws::Discard), // sub_1408a6690
    Replacement::draft(0x1408a6770, "content_1408a6770", tpl_10051::<0x14387d950, 0x14243b0b0, 0x338, 0x40, 0x4, 0x50, 0x6, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a6770
    Replacement::draft_throws(0x1408a6800, "content_1408a6800", fn_1408a6800 as *const (), Throws::Discard), // sub_1408a6800
    Replacement::draft_throws(0x1408a68f0, "content_1408a68f0", fn_1408a68f0 as *const (), Throws::Caught), // sub_1408a68f0
    Replacement::draft_throws(0x1408a6a20, "content_1408a6a20", fn_1408a6a20 as *const (), Throws::Caught), // sub_1408a6a20
    Replacement::draft_throws(0x1408a6b50, "content_1408a6b50", fn_1408a6b50 as *const (), Throws::Caught), // sub_1408a6b50
    Replacement::draft(0x1408a6c80, "content_1408a6c80", fn_1408a6c80 as *const ()), // sub_1408a6c80
    Replacement::draft(0x1408a6d10, "content_1408a6d10", tpl_68520::<0x14387d8b0, 0x14387d950, 0x1401e9970, 0x338, 0xe0, 0x4, 0xf0, 0x6, 0x1401e9860> as *const ()), // sub_1408a6d10
    Replacement::draft(0x1408a6da0, "content_1408a6da0", tpl_10051::<0x14387d950, 0x14243b0b0, 0x338, 0xe0, 0x4, 0xf0, 0x6, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a6da0
    Replacement::draft_throws(0x1408a6e40, "content_1408a6e40", fn_1408a6e40 as *const (), Throws::Discard), // sub_1408a6e40
    Replacement::draft_throws(0x1408a6ef0, "content_1408a6ef0", fn_1408a6ef0 as *const (), Throws::Discard), // sub_1408a6ef0
    Replacement::draft(0x1408a7010, "content_1408a7010", fn_1408a7010 as *const ()), // sub_1408a7010
    Replacement::draft(0x1408a70e0, "content_1408a70e0", fn_1408a70e0 as *const ()), // sub_1408a70e0
    Replacement::draft(0x1408a71c0, "content_1408a71c0", fn_1408a71c0 as *const ()), // sub_1408a71c0
    Replacement::draft(0x1408a7270, "content_1408a7270", fn_1408a7270 as *const ()), // sub_1408a7270
    Replacement::draft(0x1408a7330, "content_1408a7330", fn_1408a7330 as *const ()), // sub_1408a7330
    Replacement::draft(0x1408a73c0, "content_1408a73c0", fn_1408a73c0 as *const ()), // sub_1408a73c0
    Replacement::draft(0x1408a74e0, "content_1408a74e0", fn_1408a74e0 as *const ()), // sub_1408a74e0
    Replacement::draft_throws(0x1408a75f0, "content_1408a75f0", fn_1408a75f0 as *const (), Throws::Discard), // sub_1408a75f0
    Replacement::draft(0x1408a7700, "content_1408a7700", fn_1408a7700 as *const ()), // sub_1408a7700
    Replacement::draft_throws(0x1408a77b0, "content_1408a77b0", fn_1408a77b0 as *const (), Throws::Discard), // sub_1408a77b0
    Replacement::draft_throws(0x1408a78d0, "content_1408a78d0", fn_1408a78d0 as *const (), Throws::Caught), // sub_1408a78d0
    Replacement::draft_throws(0x1408a7a30, "content_1408a7a30", fn_1408a7a30 as *const (), Throws::Caught), // sub_1408a7a30
    Replacement::draft_throws(0x1408a7b90, "content_1408a7b90", fn_1408a7b90 as *const (), Throws::Caught), // sub_1408a7b90
    Replacement::draft(0x1408a7cf0, "content_1408a7cf0", fn_1408a7cf0 as *const ()), // sub_1408a7cf0
    Replacement::draft(0x1408a7d90, "content_1408a7d90", fn_1408a7d90 as *const ()), // sub_1408a7d90
    Replacement::draft(0x1408a7e30, "content_1408a7e30", fn_1408a7e30 as *const ()), // sub_1408a7e30
    Replacement::draft_throws(0x1408a7ee0, "content_1408a7ee0", fn_1408a7ee0 as *const (), Throws::Discard), // sub_1408a7ee0
    Replacement::draft_throws(0x1408a7fa0, "content_1408a7fa0", fn_1408a7fa0 as *const (), Throws::Discard), // sub_1408a7fa0
    Replacement::draft(0x1408a80f0, "content_1408a80f0", fn_1408a80f0 as *const ()), // sub_1408a80f0
    Replacement::draft(0x1408a81a0, "content_1408a81a0", fn_1408a81a0 as *const ()), // sub_1408a81a0
    Replacement::draft(0x1408a8260, "content_1408a8260", tpl_10075::<0x14387d930, 0x14243b0b0, 0x1408b35a0, 0x10, 0x20, 0x30, 0x40, 0x50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a8260
    Replacement::draft_throws(0x1408a8310, "content_1408a8310", fn_1408a8310 as *const (), Throws::Discard), // sub_1408a8310
    Replacement::draft(0x1408a8580, "content_1408a8580", fn_1408a8580 as *const ()), // sub_1408a8580
    Replacement::draft_throws(0x1408a87a0, "content_1408a87a0", fn_1408a87a0 as *const (), Throws::Discard), // sub_1408a87a0
    Replacement::draft(0x1408a8860, "content_1408a8860", fn_1408a8860 as *const ()), // sub_1408a8860
    Replacement::draft_throws(0x1408a89a0, "content_1408a89a0", fn_1408a89a0 as *const (), Throws::Caught), // sub_1408a89a0
    Replacement::draft_throws(0x1408a8ad0, "content_1408a8ad0", fn_1408a8ad0 as *const (), Throws::Caught), // sub_1408a8ad0
    Replacement::draft_throws(0x1408a8c00, "content_1408a8c00", fn_1408a8c00 as *const (), Throws::Caught), // sub_1408a8c00
    Replacement::draft(0x1408a8d30, "content_1408a8d30", fn_1408a8d30 as *const ()), // sub_1408a8d30
    Replacement::draft(0x1408a8f20, "content_1408a8f20", fn_1408a8f20 as *const ()), // sub_1408a8f20
    Replacement::draft(0x1408a9100, "content_1408a9100", fn_1408a9100 as *const ()), // sub_1408a9100
    Replacement::draft_throws(0x1408a92f0, "content_1408a92f0", fn_1408a92f0 as *const (), Throws::Discard), // sub_1408a92f0
    Replacement::draft_throws(0x1408a9510, "content_1408a9510", fn_1408a9510 as *const (), Throws::Discard), // sub_1408a9510
    Replacement::draft(0x1408a9630, "content_1408a9630", tpl_10084::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8> as *const ()), // sub_1408a9630
    Replacement::draft(0x1408a96b0, "content_1408a96b0", tpl_10085::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x2c, 0xffffffffffffffe4, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_1408a96b0
    Replacement::draft(0x1408a9750, "content_1408a9750", fn_1408a9750 as *const ()), // sub_1408a9750
    Replacement::draft(0x1408a9890, "content_1408a9890", tpl_10087::<0x14387d940, 0x14243b0b0, 0x140853710, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408a9890
    Replacement::draft_throws(0x1408a9910, "content_1408a9910", fn_1408a9910 as *const (), Throws::Discard), // sub_1408a9910
    Replacement::draft(0x1408a9a10, "content_1408a9a10", fn_1408a9a10 as *const ()), // sub_1408a9a10
    Replacement::draft_throws(0x1408a9b30, "content_1408a9b30", fn_1408a9b30 as *const (), Throws::Discard), // sub_1408a9b30
    Replacement::draft(0x1408a9c70, "content_1408a9c70", fn_1408a9c70 as *const ()), // sub_1408a9c70
    Replacement::draft_throws(0x1408a9d20, "content_1408a9d20", fn_1408a9d20 as *const (), Throws::Caught), // sub_1408a9d20
    Replacement::draft_throws(0x1408a9de0, "content_1408a9de0", fn_1408a9de0 as *const (), Throws::Caught), // sub_1408a9de0
    Replacement::draft_throws(0x1408a9ea0, "content_1408a9ea0", fn_1408a9ea0 as *const (), Throws::Caught), // sub_1408a9ea0
    Replacement::draft(0x1408a9f60, "content_1408a9f60", fn_1408a9f60 as *const ()), // sub_1408a9f60
    Replacement::draft(0x1408aa060, "content_1408aa060", fn_1408aa060 as *const ()), // sub_1408aa060
    Replacement::draft(0x1408aa120, "content_1408aa120", fn_1408aa120 as *const ()), // sub_1408aa120
    Replacement::draft_throws(0x1408aa230, "content_1408aa230", fn_1408aa230 as *const (), Throws::Discard), // sub_1408aa230
    Replacement::draft_throws(0x1408aa320, "content_1408aa320", fn_1408aa320 as *const (), Throws::Discard), // sub_1408aa320
    Replacement::draft(0x1408aa3e0, "content_1408aa3e0", fn_1408aa3e0 as *const ()), // sub_1408aa3e0
    Replacement::draft(0x1408aa480, "content_1408aa480", fn_1408aa480 as *const ()), // sub_1408aa480
    Replacement::draft(0x1408aa610, "content_1408aa610", fn_1408aa610 as *const ()), // sub_1408aa610
    Replacement::draft(0x1408aa6e0, "content_1408aa6e0", fn_1408aa6e0 as *const ()), // sub_1408aa6e0
    Replacement::draft(0x1408aa980, "content_1408aa980", fn_1408aa980 as *const ()), // sub_1408aa980
    Replacement::draft_throws(0x1408aac60, "content_1408aac60", fn_1408aac60 as *const (), Throws::Caught), // sub_1408aac60
    Replacement::draft_throws(0x1408ab3d0, "content_1408ab3d0", fn_1408ab3d0 as *const (), Throws::Discard), // sub_1408ab3d0
    Replacement::draft_throws(0x1408ab870, "content_1408ab870", fn_1408ab870 as *const (), Throws::Caught), // sub_1408ab870
    Replacement::draft_throws(0x1408abe30, "content_1408abe30", fn_1408abe30 as *const (), Throws::Discard), // sub_1408abe30
    Replacement::draft_throws(0x1408ac3d0, "content_1408ac3d0", fn_1408ac3d0 as *const (), Throws::Caught), // sub_1408ac3d0
    Replacement::draft_throws(0x1408ac660, "content_1408ac660", fn_1408ac660 as *const (), Throws::Caught), // sub_1408ac660
    Replacement::draft_throws(0x1408ac8f0, "content_1408ac8f0", fn_1408ac8f0 as *const (), Throws::Caught), // sub_1408ac8f0
    Replacement::draft(0x1408acb80, "content_1408acb80", fn_1408acb80 as *const ()), // sub_1408acb80
    Replacement::draft(0x1408acff0, "content_1408acff0", fn_1408acff0 as *const ()), // sub_1408acff0
    Replacement::draft(0x1408ad430, "content_1408ad430", fn_1408ad430 as *const ()), // sub_1408ad430
    Replacement::draft_throws(0x1408ad880, "content_1408ad880", fn_1408ad880 as *const (), Throws::Discard), // sub_1408ad880
    Replacement::draft_throws(0x1408add10, "content_1408add10", fn_1408add10 as *const (), Throws::Caught), // sub_1408add10
    Replacement::draft(0x1408ae060, "content_1408ae060", fn_1408ae060 as *const ()), // sub_1408ae060
    Replacement::draft(0x1408ae110, "content_1408ae110", fn_1408ae110 as *const ()), // sub_1408ae110
    Replacement::draft(0x1408ae190, "content_1408ae190", tpl_5361::<0x14387d8b8, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408ae190
    Replacement::draft_throws(0x1408ae230, "content_1408ae230", tpl_2085::<0x14387d8c8, 0x14387d8b8, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408ae230
    Replacement::draft(0x1408ae2b0, "content_1408ae2b0", tpl_2086::<0x14387d8b8, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408ae2b0
    Replacement::draft_throws(0x1408ae320, "content_1408ae320", tpl_2087::<0x14387d8c8, 0x14387d8b8, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1408ae320
    Replacement::draft_throws(0x1408ae3b0, "content_1408ae3b0", fn_1408ae3b0 as *const (), Throws::Caught), // sub_1408ae3b0
    Replacement::draft_throws(0x1408ae470, "content_1408ae470", fn_1408ae470 as *const (), Throws::Caught), // sub_1408ae470
    Replacement::draft_throws(0x1408ae530, "content_1408ae530", fn_1408ae530 as *const (), Throws::Caught), // sub_1408ae530
    Replacement::draft(0x1408ae5f0, "content_1408ae5f0", tpl_2089::<0x14387d8b8, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408ae5f0
    Replacement::draft(0x1408ae660, "content_1408ae660", tpl_2080::<0x14387d8c8, 0x14387d8b8, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // sub_1408ae660
    Replacement::draft(0x1408ae6d0, "content_1408ae6d0", tpl_2076::<0x14387d8b8, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408ae6d0
    Replacement::draft_throws(0x1408ae740, "content_1408ae740", tpl_2090::<0x14387d8c8, 0x14387d8b8, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1408ae740
    Replacement::draft_throws(0x1408ae7b0, "content_1408ae7b0", fn_1408ae7b0 as *const (), Throws::Discard), // sub_1408ae7b0
    Replacement::draft(0x1408ae870, "content_1408ae870", tpl_9443::<0x14387d978, 0x14243b0b0, 0x8, 0x10, 0x14387d8b8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408ae870
    Replacement::draft(0x1408ae940, "content_1408ae940", fn_1408ae940 as *const ()), // sub_1408ae940
    Replacement::draft_throws(0x1408aea50, "content_1408aea50", tpl_10034::<0x14387d920, 0x14387d978, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x48, 0x8, 0x1408b4fa0, 0x1408c0270, 0x1401e9e70> as *const (), Throws::Discard), // sub_1408aea50
    Replacement::draft(0x1408aeb20, "content_1408aeb20", fn_1408aeb20 as *const ()), // sub_1408aeb20
    Replacement::draft_throws(0x1408aebf0, "content_1408aebf0", tpl_10035::<0x14387d920, 0x14387d978, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x8, 0x1408b4fa0, 0x1408c0530, 0x1401e9a20> as *const (), Throws::Discard), // sub_1408aebf0
    Replacement::draft(0x1408aece0, "content_1408aece0", fn_1408aece0 as *const ()), // sub_1408aece0
    Replacement::draft_throws(0x1408aed50, "content_1408aed50", fn_1408aed50 as *const (), Throws::Caught), // sub_1408aed50
    Replacement::draft_throws(0x1408aee30, "content_1408aee30", fn_1408aee30 as *const (), Throws::Caught), // sub_1408aee30
    Replacement::draft_throws(0x1408aef10, "content_1408aef10", fn_1408aef10 as *const (), Throws::Caught), // sub_1408aef10
    Replacement::draft(0x1408aeff0, "content_1408aeff0", fn_1408aeff0 as *const ()), // sub_1408aeff0
    Replacement::draft(0x1408af0b0, "content_1408af0b0", fn_1408af0b0 as *const ()), // sub_1408af0b0
    Replacement::draft(0x1408af1a0, "content_1408af1a0", tpl_68175::<0x14387d978, 0x14243b0b0, 0x338, 0x88, 0x10, 0xe8, 0x8, 0x1408ae6d0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408af1a0
    Replacement::draft_throws(0x1408af270, "content_1408af270", fn_1408af270 as *const (), Throws::Discard), // sub_1408af270
    Replacement::draft_throws(0x1408af330, "content_1408af330", tpl_10038::<0x14387d978, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432735c8, 0x1, 0x1402c4aa0, 0x8, 0x1432735b0, 0x1408b4940> as *const (), Throws::Discard), // sub_1408af330
    Replacement::draft(0x1408af400, "content_1408af400", fn_1408af400 as *const ()), // sub_1408af400
    Replacement::draft(0x1408af490, "content_1408af490", fn_1408af490 as *const ()), // sub_1408af490
    Replacement::draft(0x1408af4f0, "content_1408af4f0", fn_1408af4f0 as *const ()), // sub_1408af4f0
    Replacement::draft(0x1408af5b0, "content_1408af5b0", fn_1408af5b0 as *const ()), // sub_1408af5b0
    Replacement::draft(0x1408af660, "content_1408af660", fn_1408af660 as *const ()), // sub_1408af660
    Replacement::draft_throws(0x1408af760, "content_1408af760", fn_1408af760 as *const (), Throws::Caught), // sub_1408af760
    Replacement::draft_throws(0x1408af890, "content_1408af890", fn_1408af890 as *const (), Throws::Discard), // sub_1408af890
    Replacement::draft_throws(0x1408af9c0, "content_1408af9c0", fn_1408af9c0 as *const (), Throws::Caught), // sub_1408af9c0
    Replacement::draft_throws(0x1408afad0, "content_1408afad0", fn_1408afad0 as *const (), Throws::Discard), // sub_1408afad0
    Replacement::draft_throws(0x1408afba0, "content_1408afba0", fn_1408afba0 as *const (), Throws::Caught), // sub_1408afba0
    Replacement::draft_throws(0x1408afc60, "content_1408afc60", fn_1408afc60 as *const (), Throws::Caught), // sub_1408afc60
    Replacement::draft_throws(0x1408afd20, "content_1408afd20", fn_1408afd20 as *const (), Throws::Caught), // sub_1408afd20
    Replacement::draft(0x1408afde0, "content_1408afde0", fn_1408afde0 as *const ()), // sub_1408afde0
    Replacement::draft(0x1408afeb0, "content_1408afeb0", fn_1408afeb0 as *const ()), // sub_1408afeb0
    Replacement::draft(0x1408aff80, "content_1408aff80", fn_1408aff80 as *const ()), // sub_1408aff80
    Replacement::draft_throws(0x1408b0050, "content_1408b0050", fn_1408b0050 as *const (), Throws::Discard), // sub_1408b0050
    Replacement::draft_throws(0x1408b0110, "content_1408b0110", tpl_10122::<0x14387d8c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432735f0, 0x1408b49c0, 0x10, 0x1432735d8, 0x1408b48d0> as *const (), Throws::Discard), // sub_1408b0110
    Replacement::draft(0x1408b01d0, "content_1408b01d0", tpl_10123::<0x1403d79a0, 0x10, 0x20, 0x30, 0x40, 0x50> as *const ()), // sub_1408b01d0
    Replacement::draft_throws(0x1408b0240, "content_1408b0240", fn_1408b0240 as *const (), Throws::Discard), // sub_1408b0240
    Replacement::draft(0x1408b03b0, "content_1408b03b0", tpl_3646::<0x8, 0xc, 0x1408746b0, 0x14183acc8> as *const ()), // sub_1408b03b0
    Replacement::draft(0x1408b0490, "content_1408b0490", tpl_10125::<0x1408ce8a0, 0x14183acc8> as *const ()), // sub_1408b0490
    Replacement::draft(0x1408b0590, "content_1408b0590", fn_1408b0590 as *const ()), // sub_1408b0590
    Replacement::draft_throws(0x1408b0670, "content_1408b0670", tpl_65670::<0x142442de0, 0x143a3ae90, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_1408b0670
    Replacement::draft(0x1408b0750, "content_1408b0750", tpl_10126::<0x8, 0x1408b4e10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1408b0750
    Replacement::draft(0x1408b0810, "content_1408b0810", tpl_10127::<0x8, 0x1401de830, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1408b0810
    Replacement::draft_throws(0x1408b0910, "content_1408b0910", tpl_2502::<0x20, 0x8, 0x1408b6230> as *const (), Throws::Discard), // sub_1408b0910
    Replacement::draft_throws(0x1408b0960, "content_1408b0960", fn_1408b0960 as *const (), Throws::Caught), // sub_1408b0960
    Replacement::draft_throws(0x1408b0c50, "content_1408b0c50", tpl_65672::<0x142442de0, 0x143a3ae90, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_1408b0c50
    Replacement::draft_throws(0x1408b0e70, "content_1408b0e70", fn_1408b0e70 as *const (), Throws::Caught), // sub_1408b0e70
    Replacement::draft_throws(0x1408b1160, "content_1408b1160", fn_1408b1160 as *const (), Throws::Discard), // sub_1408b1160
    Replacement::draft_throws(0x1408b1200, "content_1408b1200", fn_1408b1200 as *const (), Throws::Caught), // sub_1408b1200
    Replacement::draft_throws(0x1408b1cc0, "content_1408b1cc0", fn_1408b1cc0 as *const (), Throws::Caught), // sub_1408b1cc0
    Replacement::draft_throws(0x1408b2750, "content_1408b2750", fn_1408b2750 as *const (), Throws::Caught), // sub_1408b2750
    Replacement::draft(0x1408b31d0, "content_1408b31d0", tpl_68679::<0x338, 0x120, 0x143a3ae90, 0x28, 0x8, 0xe0, 0x38> as *const ()), // sub_1408b31d0
    Replacement::draft(0x1408b3270, "content_1408b3270", tpl_68679::<0x338, 0x120, 0x143a3ae90, 0x28, 0x8, 0xe0, 0x30> as *const ()), // sub_1408b3270
    Replacement::draft(0x1408b3310, "content_1408b3310", fn_1408b3310 as *const ()), // sub_1408b3310
    Replacement::draft_throws(0x1408b33a0, "content_1408b33a0", fn_1408b33a0 as *const (), Throws::Caught), // sub_1408b33a0
    Replacement::draft(0x1408b35a0, "content_1408b35a0", fn_1408b35a0 as *const ()), // sub_1408b35a0
    Replacement::draft(0x1408b3670, "content_1408b3670", tpl_6966::<0x1408b4ad0, 0x14183acbc> as *const ()), // sub_1408b3670
    Replacement::draft(0x1408b3750, "content_1408b3750", tpl_2503::<0x8, 0x14387d960, 0x14243b0b0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1408b3750
    Replacement::draft(0x1408b37d0, "content_1408b37d0", fn_1408b37d0 as *const ()), // sub_1408b37d0
    Replacement::draft(0x1408b38b0, "content_1408b38b0", fn_1408b38b0 as *const ()), // sub_1408b38b0
    Replacement::draft(0x1408b3960, "content_1408b3960", tpl_2504::<0x8, 0x1426d4580, 0x338, 0x1, 0x1408a74e0> as *const ()), // sub_1408b3960
    Replacement::draft(0x1408b3a10, "content_1408b3a10", fn_1408b3a10 as *const ()), // sub_1408b3a10
    Replacement::draft_throws(0x1408b3b30, "content_1408b3b30", fn_1408b3b30 as *const (), Throws::Discard), // sub_1408b3b30
    Replacement::draft_throws(0x1408b3c10, "content_1408b3c10", fn_1408b3c10 as *const (), Throws::Caught), // sub_1408b3c10
    Replacement::draft_throws(0x1408b3d50, "content_1408b3d50", fn_1408b3d50 as *const (), Throws::Caught), // sub_1408b3d50
    Replacement::draft_throws(0x1408b3ee0, "content_1408b3ee0", fn_1408b3ee0 as *const (), Throws::Caught), // sub_1408b3ee0
    Replacement::draft_throws(0x1408b4020, "content_1408b4020", fn_1408b4020 as *const (), Throws::Caught), // sub_1408b4020
    Replacement::draft_throws(0x1408b41b0, "content_1408b41b0", fn_1408b41b0 as *const (), Throws::Caught), // sub_1408b41b0
    Replacement::draft_throws(0x1408b42f0, "content_1408b42f0", fn_1408b42f0 as *const (), Throws::Caught), // sub_1408b42f0
    Replacement::draft(0x1408b4430, "content_1408b4430", fn_1408b4430 as *const ()), // sub_1408b4430
    Replacement::draft(0x1408b4570, "content_1408b4570", fn_1408b4570 as *const ()), // sub_1408b4570
    Replacement::draft(0x1408b4660, "content_1408b4660", fn_1408b4660 as *const ()), // sub_1408b4660
    Replacement::draft_throws(0x1408b4790, "content_1408b4790", tpl_9695::<0x20, 0x8, 0x1408bfd50> as *const (), Throws::Discard), // sub_1408b4790
    Replacement::draft_throws(0x1408b47e0, "content_1408b47e0", tpl_9695::<0x20, 0x8, 0x1408bfe20> as *const (), Throws::Discard), // sub_1408b47e0
    Replacement::draft_throws(0x1408b4830, "content_1408b4830", tpl_2507::<0x20, 0x8, 0x1408bfef0> as *const (), Throws::Discard), // sub_1408b4830
    Replacement::draft_throws(0x1408b4880, "content_1408b4880", tpl_2507::<0x20, 0x8, 0x1408bffc0> as *const (), Throws::Discard), // sub_1408b4880
    Replacement::draft(0x1408b48e0, "content_1408b48e0", fn_1408b48e0 as *const ()), // sub_1408b48e0
    Replacement::draft_throws(0x1408b49c0, "content_1408b49c0", tpl_2507::<0x20, 0x8, 0x1408c2710> as *const (), Throws::Discard), // sub_1408b49c0
    Replacement::draft(0x1408b4a10, "content_1408b4a10", fn_1408b4a10 as *const ()), // sub_1408b4a10
    Replacement::draft(0x1408b4b90, "content_1408b4b90", tpl_66024::<0x8, 0xc, 0x1408746b0> as *const ()), // sub_1408b4b90
    Replacement::draft(0x1408b4c50, "content_1408b4c50", fn_1408b4c50 as *const ()), // sub_1408b4c50
    Replacement::draft(0x1408b4d20, "content_1408b4d20", fn_1408b4d20 as *const ()), // sub_1408b4d20
    Replacement::draft(0x1408b4e10, "content_1408b4e10", fn_1408b4e10 as *const ()), // sub_1408b4e10
    Replacement::draft(0x1408b4e80, "content_1408b4e80", fn_1408b4e80 as *const ()), // sub_1408b4e80
    Replacement::draft(0x1408b4fa0, "content_1408b4fa0", tpl_68695::<0x8, 0xc, 0x1401deba0, 0x14183acc2> as *const ()), // sub_1408b4fa0
    Replacement::draft_throws(0x1408b5040, "content_1408b5040", fn_1408b5040 as *const (), Throws::Caught), // sub_1408b5040
    Replacement::draft_throws(0x1408b5300, "content_1408b5300", fn_1408b5300 as *const (), Throws::Caught), // sub_1408b5300
    Replacement::draft_throws(0x1408b55c0, "content_1408b55c0", fn_1408b55c0 as *const (), Throws::Caught), // sub_1408b55c0
    Replacement::draft_throws(0x1408b5a00, "content_1408b5a00", fn_1408b5a00 as *const (), Throws::Caught), // sub_1408b5a00
    Replacement::draft_throws(0x1408b5e20, "content_1408b5e20", fn_1408b5e20 as *const (), Throws::Caught), // sub_1408b5e20
    Replacement::draft_throws(0x1408b6230, "content_1408b6230", fn_1408b6230 as *const (), Throws::Caught), // sub_1408b6230
    Replacement::draft_throws(0x1408b6300, "content_1408b6300", fn_1408b6300 as *const (), Throws::Caught), // sub_1408b6300
    Replacement::draft_throws(0x1408b6440, "content_1408b6440", fn_1408b6440 as *const (), Throws::Caught), // sub_1408b6440
    Replacement::draft_throws(0x1408b6700, "content_1408b6700", fn_1408b6700 as *const (), Throws::Caught), // sub_1408b6700
    Replacement::draft_throws(0x1408b6990, "content_1408b6990", fn_1408b6990 as *const (), Throws::Caught), // sub_1408b6990
    Replacement::draft_throws(0x1408b6c20, "content_1408b6c20", fn_1408b6c20 as *const (), Throws::Caught), // sub_1408b6c20
    Replacement::draft_throws(0x1408b6ee0, "content_1408b6ee0", fn_1408b6ee0 as *const (), Throws::Caught), // sub_1408b6ee0
    Replacement::draft_throws(0x1408b71b0, "content_1408b71b0", fn_1408b71b0 as *const (), Throws::Caught), // sub_1408b71b0
    Replacement::draft_throws(0x1408b7490, "content_1408b7490", fn_1408b7490 as *const (), Throws::Caught), // sub_1408b7490
    Replacement::draft_throws(0x1408b7760, "content_1408b7760", fn_1408b7760 as *const (), Throws::Caught), // sub_1408b7760
    Replacement::draft_throws(0x1408b7a20, "content_1408b7a20", fn_1408b7a20 as *const (), Throws::Caught), // sub_1408b7a20
    Replacement::draft_throws(0x1408b7cf0, "content_1408b7cf0", fn_1408b7cf0 as *const (), Throws::Caught), // sub_1408b7cf0
    Replacement::draft_throws(0x1408b7fd0, "content_1408b7fd0", fn_1408b7fd0 as *const (), Throws::Caught), // sub_1408b7fd0
    Replacement::draft_throws(0x1408b82a0, "content_1408b82a0", fn_1408b82a0 as *const (), Throws::Discard), // sub_1408b82a0
    Replacement::draft_throws(0x1408b8460, "content_1408b8460", fn_1408b8460 as *const (), Throws::Caught), // sub_1408b8460
    Replacement::draft_throws(0x1408b88c0, "content_1408b88c0", fn_1408b88c0 as *const (), Throws::Caught), // sub_1408b88c0
    Replacement::draft_throws(0x1408b8d10, "content_1408b8d10", fn_1408b8d10 as *const (), Throws::Caught), // sub_1408b8d10
    Replacement::draft_throws(0x1408b91a0, "content_1408b91a0", fn_1408b91a0 as *const (), Throws::Caught), // sub_1408b91a0
    Replacement::draft_throws(0x1408b97f0, "content_1408b97f0", fn_1408b97f0 as *const (), Throws::Caught), // sub_1408b97f0
    Replacement::draft_throws(0x1408b9c70, "content_1408b9c70", fn_1408b9c70 as *const (), Throws::Caught), // sub_1408b9c70
    Replacement::draft_throws(0x1408ba320, "content_1408ba320", fn_1408ba320 as *const (), Throws::Caught), // sub_1408ba320
    Replacement::draft_throws(0x1408ba8d0, "content_1408ba8d0", fn_1408ba8d0 as *const (), Throws::Caught), // sub_1408ba8d0
    Replacement::draft_throws(0x1408bae80, "content_1408bae80", fn_1408bae80 as *const (), Throws::Caught), // sub_1408bae80
    Replacement::draft_throws(0x1408bb2b0, "content_1408bb2b0", fn_1408bb2b0 as *const (), Throws::Caught), // sub_1408bb2b0
    Replacement::draft_throws(0x1408bb6d0, "content_1408bb6d0", fn_1408bb6d0 as *const (), Throws::Caught), // sub_1408bb6d0
    Replacement::draft_throws(0x1408bbb20, "content_1408bbb20", fn_1408bbb20 as *const (), Throws::Caught), // sub_1408bbb20
    Replacement::draft_throws(0x1408bc130, "content_1408bc130", fn_1408bc130 as *const (), Throws::Caught), // sub_1408bc130
    Replacement::draft_throws(0x1408bc580, "content_1408bc580", fn_1408bc580 as *const (), Throws::Caught), // sub_1408bc580
    Replacement::draft_throws(0x1408bcbf0, "content_1408bcbf0", fn_1408bcbf0 as *const (), Throws::Caught), // sub_1408bcbf0
    Replacement::draft_throws(0x1408bd160, "content_1408bd160", fn_1408bd160 as *const (), Throws::Caught), // sub_1408bd160
    Replacement::draft_throws(0x1408bd6d0, "content_1408bd6d0", fn_1408bd6d0 as *const (), Throws::Caught), // sub_1408bd6d0
    Replacement::draft_throws(0x1408bdaf0, "content_1408bdaf0", fn_1408bdaf0 as *const (), Throws::Caught), // sub_1408bdaf0
    Replacement::draft_throws(0x1408bdf10, "content_1408bdf10", fn_1408bdf10 as *const (), Throws::Caught), // sub_1408bdf10
    Replacement::draft_throws(0x1408be350, "content_1408be350", fn_1408be350 as *const (), Throws::Caught), // sub_1408be350
    Replacement::draft_throws(0x1408be950, "content_1408be950", fn_1408be950 as *const (), Throws::Caught), // sub_1408be950
    Replacement::draft_throws(0x1408bed90, "content_1408bed90", fn_1408bed90 as *const (), Throws::Caught), // sub_1408bed90
    Replacement::draft_throws(0x1408bf400, "content_1408bf400", fn_1408bf400 as *const (), Throws::Caught), // sub_1408bf400
    Replacement::draft_throws(0x1408bf840, "content_1408bf840", fn_1408bf840 as *const (), Throws::Caught), // sub_1408bf840
    Replacement::draft_throws(0x1408bfc80, "content_1408bfc80", fn_1408bfc80 as *const (), Throws::Caught), // sub_1408bfc80
    Replacement::draft_throws(0x1408bfd50, "content_1408bfd50", fn_1408bfd50 as *const (), Throws::Caught), // sub_1408bfd50
    Replacement::draft_throws(0x1408bfe20, "content_1408bfe20", fn_1408bfe20 as *const (), Throws::Caught), // sub_1408bfe20
    Replacement::draft_throws(0x1408bfef0, "content_1408bfef0", fn_1408bfef0 as *const (), Throws::Caught), // sub_1408bfef0
    Replacement::draft_throws(0x1408bffc0, "content_1408bffc0", fn_1408bffc0 as *const (), Throws::Caught), // sub_1408bffc0
    Replacement::draft_throws(0x1408c0090, "content_1408c0090", fn_1408c0090 as *const (), Throws::Caught), // sub_1408c0090
    Replacement::draft(0x1408c0230, "content_1408c0230", tpl_10167::<0x8, 0x18> as *const ()), // sub_1408c0230
    Replacement::draft(0x1408c0250, "content_1408c0250", fn_1408c0250 as *const ()), // sub_1408c0250
    Replacement::draft_throws(0x1408c0270, "content_1408c0270", fn_1408c0270 as *const (), Throws::Caught), // sub_1408c0270
    Replacement::draft_throws(0x1408c0530, "content_1408c0530", fn_1408c0530 as *const (), Throws::Caught), // sub_1408c0530
    Replacement::draft_throws(0x1408c07f0, "content_1408c07f0", fn_1408c07f0 as *const (), Throws::Caught), // sub_1408c07f0
    Replacement::draft_throws(0x1408c0c20, "content_1408c0c20", fn_1408c0c20 as *const (), Throws::Caught), // sub_1408c0c20
    Replacement::draft_throws(0x1408c1020, "content_1408c1020", fn_1408c1020 as *const (), Throws::Caught), // sub_1408c1020
    Replacement::draft_throws(0x1408c1420, "content_1408c1420", fn_1408c1420 as *const (), Throws::Caught), // sub_1408c1420
    Replacement::draft_throws(0x1408c14f0, "content_1408c14f0", fn_1408c14f0 as *const (), Throws::Caught), // sub_1408c14f0
    Replacement::draft_throws(0x1408c1b30, "content_1408c1b30", fn_1408c1b30 as *const (), Throws::Caught), // sub_1408c1b30
    Replacement::draft_throws(0x1408c2130, "content_1408c2130", fn_1408c2130 as *const (), Throws::Caught), // sub_1408c2130
    Replacement::draft_throws(0x1408c2710, "content_1408c2710", fn_1408c2710 as *const (), Throws::Caught), // sub_1408c2710
    Replacement::draft(0x1408c27e0, "content_1408c27e0", fn_1408c27e0 as *const ()), // sub_1408c27e0
    Replacement::draft(0x1408c28c0, "content_1408c28c0", fn_1408c28c0 as *const ()), // sub_1408c28c0
    Replacement::draft(0x1408c2930, "content_1408c2930", tpl_10174::<0x8, 0x1408a3ab0> as *const ()), // sub_1408c2930
    Replacement::draft(0x1408c2970, "content_1408c2970", fn_1408c2970 as *const ()), // sub_1408c2970
    Replacement::draft(0x1408c29d0, "content_1408c29d0", fn_1408c29d0 as *const ()), // sub_1408c29d0
    Replacement::draft_throws(0x1408c2b10, "content_1408c2b10", fn_1408c2b10 as *const (), Throws::Discard), // sub_1408c2b10
    Replacement::draft(0x1408c2cd0, "content_1408c2cd0", fn_1408c2cd0 as *const ()), // sub_1408c2cd0
    Replacement::draft(0x1408c2f90, "content_1408c2f90", fn_1408c2f90 as *const ()), // sub_1408c2f90
    Replacement::draft_throws(0x1408c3250, "content_1408c3250", fn_1408c3250 as *const (), Throws::Caught), // sub_1408c3250
    Replacement::draft_throws(0x1408c3720, "content_1408c3720", fn_1408c3720 as *const (), Throws::Caught), // sub_1408c3720
    Replacement::draft_throws(0x1408c3ba0, "content_1408c3ba0", fn_1408c3ba0 as *const (), Throws::Caught), // sub_1408c3ba0
    Replacement::draft_throws(0x1408c4020, "content_1408c4020", fn_1408c4020 as *const (), Throws::Caught), // sub_1408c4020
    Replacement::draft_throws(0x1408c45d0, "content_1408c45d0", fn_1408c45d0 as *const (), Throws::Caught), // sub_1408c45d0
    Replacement::draft_throws(0x1408c49d0, "content_1408c49d0", fn_1408c49d0 as *const (), Throws::Discard), // sub_1408c49d0
    Replacement::draft_throws(0x1408c5180, "content_1408c5180", fn_1408c5180 as *const (), Throws::Discard), // sub_1408c5180
    Replacement::draft_throws(0x1408c5850, "content_1408c5850", fn_1408c5850 as *const (), Throws::Caught), // sub_1408c5850
    Replacement::draft_throws(0x1408c5c50, "content_1408c5c50", fn_1408c5c50 as *const (), Throws::Discard), // sub_1408c5c50
    Replacement::draft_throws(0x1408c63e0, "content_1408c63e0", fn_1408c63e0 as *const (), Throws::Caught), // sub_1408c63e0
    Replacement::draft_throws(0x1408c68b0, "content_1408c68b0", fn_1408c68b0 as *const (), Throws::Caught), // sub_1408c68b0
    Replacement::draft_throws(0x1408c6d80, "content_1408c6d80", fn_1408c6d80 as *const (), Throws::Caught), // sub_1408c6d80
    Replacement::draft_throws(0x1408c7250, "content_1408c7250", fn_1408c7250 as *const (), Throws::Caught), // sub_1408c7250
    Replacement::draft_throws(0x1408c7720, "content_1408c7720", fn_1408c7720 as *const (), Throws::Caught), // sub_1408c7720
    Replacement::draft_throws(0x1408c7bf0, "content_1408c7bf0", fn_1408c7bf0 as *const (), Throws::Caught), // sub_1408c7bf0
    Replacement::draft_throws(0x1408c8390, "content_1408c8390", fn_1408c8390 as *const (), Throws::Caught), // sub_1408c8390
    Replacement::draft_throws(0x1408c8810, "content_1408c8810", fn_1408c8810 as *const (), Throws::Caught), // sub_1408c8810
    Replacement::draft_throws(0x1408c8ca0, "content_1408c8ca0", fn_1408c8ca0 as *const (), Throws::Caught), // sub_1408c8ca0
    Replacement::draft_throws(0x1408c9130, "content_1408c9130", fn_1408c9130 as *const (), Throws::Caught), // sub_1408c9130
    Replacement::draft_throws(0x1408c95c0, "content_1408c95c0", fn_1408c95c0 as *const (), Throws::Caught), // sub_1408c95c0
    Replacement::draft_throws(0x1408c9a50, "content_1408c9a50", fn_1408c9a50 as *const (), Throws::Caught), // sub_1408c9a50
    Replacement::draft_throws(0x1408ca1f0, "content_1408ca1f0", fn_1408ca1f0 as *const (), Throws::Caught), // sub_1408ca1f0
    Replacement::draft_throws(0x1408ca670, "content_1408ca670", fn_1408ca670 as *const (), Throws::Caught), // sub_1408ca670
    Replacement::draft_throws(0x1408cab00, "content_1408cab00", fn_1408cab00 as *const (), Throws::Caught), // sub_1408cab00
    Replacement::draft_throws(0x1408caf90, "content_1408caf90", fn_1408caf90 as *const (), Throws::Caught), // sub_1408caf90
    Replacement::draft_throws(0x1408cb420, "content_1408cb420", fn_1408cb420 as *const (), Throws::Caught), // sub_1408cb420
    Replacement::draft_throws(0x1408cb8b0, "content_1408cb8b0", fn_1408cb8b0 as *const (), Throws::Caught), // sub_1408cb8b0
    Replacement::draft_throws(0x1408cc050, "content_1408cc050", fn_1408cc050 as *const (), Throws::Caught), // sub_1408cc050
    Replacement::draft_throws(0x1408cc520, "content_1408cc520", fn_1408cc520 as *const (), Throws::Caught), // sub_1408cc520
    Replacement::draft_throws(0x1408cc9a0, "content_1408cc9a0", fn_1408cc9a0 as *const (), Throws::Caught), // sub_1408cc9a0
    Replacement::draft_throws(0x1408cce20, "content_1408cce20", fn_1408cce20 as *const (), Throws::Caught), // sub_1408cce20
    Replacement::draft_throws(0x1408cd210, "content_1408cd210", fn_1408cd210 as *const (), Throws::Caught), // sub_1408cd210
    Replacement::draft_throws(0x1408cd600, "content_1408cd600", fn_1408cd600 as *const (), Throws::Caught), // sub_1408cd600
    Replacement::draft_throws(0x1408cdad0, "content_1408cdad0", fn_1408cdad0 as *const (), Throws::Caught), // sub_1408cdad0
    Replacement::draft_throws(0x1408cdf60, "content_1408cdf60", fn_1408cdf60 as *const (), Throws::Caught), // sub_1408cdf60
    Replacement::draft(0x1408ce400, "content_1408ce400", fn_1408ce400 as *const ()), // sub_1408ce400
    Replacement::draft(0x1408ce5a0, "content_1408ce5a0", tpl_10188::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1408b0750> as *const ()), // sub_1408ce5a0
    Replacement::draft(0x1408ce710, "content_1408ce710", tpl_10189::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1408b0810> as *const ()), // sub_1408ce710
    Replacement::draft_throws(0x1408ce9e0, "content_1408ce9e0", fn_1408ce9e0 as *const (), Throws::Caught), // sub_1408ce9e0
    Replacement::draft_throws(0x1408cec40, "content_1408cec40", fn_1408cec40 as *const (), Throws::Caught), // sub_1408cec40
    Replacement::draft_throws(0x1408cee90, "content_1408cee90", fn_1408cee90 as *const (), Throws::Caught), // sub_1408cee90
    Replacement::draft_throws(0x1408cf0e0, "content_1408cf0e0", fn_1408cf0e0 as *const (), Throws::Caught), // sub_1408cf0e0
    Replacement::draft_throws(0x1408cf370, "content_1408cf370", fn_1408cf370 as *const (), Throws::Discard), // sub_1408cf370
    Replacement::draft_throws(0x1408cf4e0, "content_1408cf4e0", fn_1408cf4e0 as *const (), Throws::Caught), // sub_1408cf4e0
    Replacement::draft_throws(0x1408cf740, "content_1408cf740", fn_1408cf740 as *const (), Throws::Caught), // sub_1408cf740
    Replacement::draft_throws(0x1408cf9a0, "content_1408cf9a0", fn_1408cf9a0 as *const (), Throws::Caught), // sub_1408cf9a0
    Replacement::draft_throws(0x1408cfc20, "content_1408cfc20", fn_1408cfc20 as *const (), Throws::Caught), // sub_1408cfc20
    Replacement::draft_throws(0x1408d0020, "content_1408d0020", fn_1408d0020 as *const (), Throws::Caught), // sub_1408d0020
    Replacement::draft_throws(0x1408d0280, "content_1408d0280", fn_1408d0280 as *const (), Throws::Caught), // sub_1408d0280
    Replacement::draft_throws(0x1408d04d0, "content_1408d04d0", fn_1408d04d0 as *const (), Throws::Caught), // sub_1408d04d0
    Replacement::draft_throws(0x1408d0730, "content_1408d0730", fn_1408d0730 as *const (), Throws::Caught), // sub_1408d0730
    Replacement::draft_throws(0x1408d09b0, "content_1408d09b0", fn_1408d09b0 as *const (), Throws::Caught), // sub_1408d09b0
    Replacement::draft_throws(0x1408d0db0, "content_1408d0db0", fn_1408d0db0 as *const (), Throws::Caught), // sub_1408d0db0
    Replacement::draft_throws(0x1408d1010, "content_1408d1010", fn_1408d1010 as *const (), Throws::Caught), // sub_1408d1010
    Replacement::draft_throws(0x1408d1260, "content_1408d1260", fn_1408d1260 as *const (), Throws::Caught), // sub_1408d1260
    Replacement::draft_throws(0x1408d14c0, "content_1408d14c0", fn_1408d14c0 as *const (), Throws::Caught), // sub_1408d14c0
    Replacement::draft_throws(0x1408d1740, "content_1408d1740", fn_1408d1740 as *const (), Throws::Caught), // sub_1408d1740
    Replacement::draft_throws(0x1408d1b40, "content_1408d1b40", fn_1408d1b40 as *const (), Throws::Caught), // sub_1408d1b40
    Replacement::draft_throws(0x1408d1da0, "content_1408d1da0", fn_1408d1da0 as *const (), Throws::Caught), // sub_1408d1da0
    Replacement::draft_throws(0x1408d1f90, "content_1408d1f90", fn_1408d1f90 as *const (), Throws::Caught), // sub_1408d1f90
    Replacement::draft_throws(0x1408d2220, "content_1408d2220", fn_1408d2220 as *const (), Throws::Caught), // sub_1408d2220
    Replacement::draft_throws(0x1408d2490, "content_1408d2490", fn_1408d2490 as *const (), Throws::Caught), // sub_1408d2490
    Replacement::draft_throws(0x1408d2700, "content_1408d2700", fn_1408d2700 as *const (), Throws::Caught), // sub_1408d2700
    Replacement::draft_throws(0x1408d28f0, "content_1408d28f0", fn_1408d28f0 as *const (), Throws::Caught), // sub_1408d28f0
    Replacement::draft_throws(0x1408d2b50, "content_1408d2b50", fn_1408d2b50 as *const (), Throws::Caught), // sub_1408d2b50
    Replacement::draft_throws(0x1408d2da0, "content_1408d2da0", fn_1408d2da0 as *const (), Throws::Caught), // sub_1408d2da0
    Replacement::draft_throws(0x1408d2ff0, "content_1408d2ff0", fn_1408d2ff0 as *const (), Throws::Caught), // sub_1408d2ff0
    Replacement::draft_throws(0x1408d3250, "content_1408d3250", fn_1408d3250 as *const (), Throws::Caught), // sub_1408d3250
    Replacement::draft_throws(0x1408d3640, "content_1408d3640", fn_1408d3640 as *const (), Throws::Caught), // sub_1408d3640
    Replacement::draft_throws(0x1408d3a30, "content_1408d3a30", fn_1408d3a30 as *const (), Throws::Caught), // sub_1408d3a30
    Replacement::draft_throws(0x1408d3e20, "content_1408d3e20", fn_1408d3e20 as *const (), Throws::Caught), // sub_1408d3e20
    Replacement::draft_throws(0x1408d4090, "content_1408d4090", fn_1408d4090 as *const (), Throws::Discard), // sub_1408d4090
    Replacement::draft_throws(0x1408d4100, "content_1408d4100", fn_1408d4100 as *const (), Throws::Caught), // sub_1408d4100
    Replacement::draft_throws(0x1408d4670, "content_1408d4670", fn_1408d4670 as *const (), Throws::Caught), // sub_1408d4670
    Replacement::draft_throws(0x1408d4ba0, "content_1408d4ba0", fn_1408d4ba0 as *const (), Throws::Caught), // sub_1408d4ba0
    Replacement::draft_throws(0x1408d4ef0, "content_1408d4ef0", fn_1408d4ef0 as *const (), Throws::Caught), // sub_1408d4ef0
    Replacement::draft_throws(0x1408d5460, "content_1408d5460", fn_1408d5460 as *const (), Throws::Caught), // sub_1408d5460
    Replacement::draft_throws(0x1408d59d0, "content_1408d59d0", fn_1408d59d0 as *const (), Throws::Caught), // sub_1408d59d0
    Replacement::draft_throws(0x1408d5f60, "content_1408d5f60", fn_1408d5f60 as *const (), Throws::Caught), // sub_1408d5f60
    Replacement::draft_throws(0x1408d66b0, "content_1408d66b0", fn_1408d66b0 as *const (), Throws::Caught), // sub_1408d66b0
    Replacement::draft_throws(0x1408d6c20, "content_1408d6c20", fn_1408d6c20 as *const (), Throws::Caught), // sub_1408d6c20
    Replacement::draft_throws(0x1408d7150, "content_1408d7150", fn_1408d7150 as *const (), Throws::Caught), // sub_1408d7150
    Replacement::draft_throws(0x1408d7690, "content_1408d7690", fn_1408d7690 as *const (), Throws::Caught), // sub_1408d7690
    Replacement::draft_throws(0x1408d7bf0, "content_1408d7bf0", fn_1408d7bf0 as *const (), Throws::Caught), // sub_1408d7bf0
    Replacement::draft_throws(0x1408d8300, "content_1408d8300", fn_1408d8300 as *const (), Throws::Caught), // sub_1408d8300
    Replacement::draft_throws(0x1408d8840, "content_1408d8840", fn_1408d8840 as *const (), Throws::Caught), // sub_1408d8840
    Replacement::draft_throws(0x1408d8b90, "content_1408d8b90", fn_1408d8b90 as *const (), Throws::Caught), // sub_1408d8b90
    Replacement::draft_throws(0x1408d8ee0, "content_1408d8ee0", fn_1408d8ee0 as *const (), Throws::Caught), // sub_1408d8ee0
    Replacement::draft_throws(0x1408d9230, "content_1408d9230", fn_1408d9230 as *const (), Throws::Caught), // sub_1408d9230
    Replacement::draft_throws(0x1408d9580, "content_1408d9580", fn_1408d9580 as *const (), Throws::Caught), // sub_1408d9580
    Replacement::draft_throws(0x1408d98d0, "content_1408d98d0", fn_1408d98d0 as *const (), Throws::Caught), // sub_1408d98d0
    Replacement::draft_throws(0x1408d9e40, "content_1408d9e40", fn_1408d9e40 as *const (), Throws::Caught), // sub_1408d9e40
    Replacement::draft_throws(0x1408da370, "content_1408da370", fn_1408da370 as *const (), Throws::Caught), // sub_1408da370
    Replacement::draft_throws(0x1408da6c0, "content_1408da6c0", fn_1408da6c0 as *const (), Throws::Caught), // sub_1408da6c0
    Replacement::draft_throws(0x1408dadf0, "content_1408dadf0", fn_1408dadf0 as *const (), Throws::Caught), // sub_1408dadf0
    Replacement::draft_throws(0x1408db4e0, "content_1408db4e0", fn_1408db4e0 as *const (), Throws::Caught), // sub_1408db4e0
    Replacement::draft(0x1408db960, "content_1408db960", fn_1408db960 as *const ()), // sub_1408db960
    Replacement::draft(0x1408db9c0, "content_1408db9c0", fn_1408db9c0 as *const ()), // sub_1408db9c0
    Replacement::draft(0x1408dba40, "content_1408dba40", fn_1408dba40 as *const ()), // sub_1408dba40
    Replacement::draft(0x1408dbb30, "content_1408dbb30", fn_1408dbb30 as *const ()), // sub_1408dbb30
    Replacement::draft(0x1408dbbb0, "content_1408dbbb0", fn_1408dbbb0 as *const ()), // sub_1408dbbb0
    Replacement::draft(0x1408dbc30, "content_1408dbc30", fn_1408dbc30 as *const ()), // sub_1408dbc30
    Replacement::draft_throws(0x1408dbe00, "content_1408dbe00", fn_1408dbe00 as *const (), Throws::Discard), // sub_1408dbe00
    Replacement::draft_throws(0x1408dbfc0, "content_1408dbfc0", fn_1408dbfc0 as *const (), Throws::Discard), // sub_1408dbfc0
    Replacement::draft_throws(0x1408dc0c0, "content_1408dc0c0", fn_1408dc0c0 as *const (), Throws::Discard), // sub_1408dc0c0
    Replacement::draft_throws(0x1408dc2a0, "content_1408dc2a0", fn_1408dc2a0 as *const (), Throws::Discard), // sub_1408dc2a0
    Replacement::draft_throws(0x1408dc330, "content_1408dc330", fn_1408dc330 as *const (), Throws::Caught), // sub_1408dc330
    Replacement::draft_throws(0x1408dc8e0, "content_1408dc8e0", fn_1408dc8e0 as *const (), Throws::Caught), // sub_1408dc8e0
    Replacement::draft_throws(0x1408dce00, "content_1408dce00", fn_1408dce00 as *const (), Throws::Caught), // sub_1408dce00
    Replacement::draft(0x1408dd310, "content_1408dd310", fn_1408dd310 as *const ()), // sub_1408dd310
    Replacement::draft(0x1408dd450, "content_1408dd450", fn_1408dd450 as *const ()), // sub_1408dd450
    Replacement::draft(0x1408dd550, "content_1408dd550", fn_1408dd550 as *const ()), // sub_1408dd550
    Replacement::draft_throws(0x1408dd660, "content_1408dd660", fn_1408dd660 as *const (), Throws::Discard), // sub_1408dd660
    Replacement::draft_throws(0x1408dd750, "content_1408dd750", fn_1408dd750 as *const (), Throws::Discard), // sub_1408dd750
    Replacement::draft(0x1408dd8c0, "content_1408dd8c0", tpl_10237::<0x128, 0x18, 0x143873dc0, 0x130, 0x143275de8, 0x2, 0x1401de410> as *const ()), // sub_1408dd8c0
    Replacement::draft(0x1408dd930, "content_1408dd930", tpl_10237::<0x128, 0x8, 0x143873da0, 0x130, 0x143275de8, 0x2, 0x1401de410> as *const ()), // sub_1408dd930
    Replacement::draft(0x1408dd9a0, "content_1408dd9a0", fn_1408dd9a0 as *const ()), // sub_1408dd9a0
    Replacement::draft(0x1408ddad0, "content_1408ddad0", fn_1408ddad0 as *const ()), // sub_1408ddad0
    Replacement::draft(0x1408ddb50, "content_1408ddb50", fn_1408ddb50 as *const ()), // sub_1408ddb50
    Replacement::draft(0x1408ddbf0, "content_1408ddbf0", fn_1408ddbf0 as *const ()), // sub_1408ddbf0
    Replacement::draft_throws(0x1408ddec0, "content_1408ddec0", fn_1408ddec0 as *const (), Throws::Discard), // sub_1408ddec0
    Replacement::draft_throws(0x1408de1b0, "content_1408de1b0", fn_1408de1b0 as *const (), Throws::Discard), // sub_1408de1b0
    Replacement::draft_throws(0x1408de340, "content_1408de340", fn_1408de340 as *const (), Throws::Discard), // sub_1408de340
    Replacement::draft_throws(0x1408de660, "content_1408de660", fn_1408de660 as *const (), Throws::Discard), // sub_1408de660
    Replacement::draft_throws(0x1408de760, "content_1408de760", fn_1408de760 as *const (), Throws::Caught), // sub_1408de760
    Replacement::draft_throws(0x1408def30, "content_1408def30", fn_1408def30 as *const (), Throws::Caught), // sub_1408def30
    Replacement::draft_throws(0x1408df6a0, "content_1408df6a0", fn_1408df6a0 as *const (), Throws::Caught), // sub_1408df6a0
    Replacement::draft(0x1408dfde0, "content_1408dfde0", fn_1408dfde0 as *const ()), // sub_1408dfde0
    Replacement::draft(0x1408dffd0, "content_1408dffd0", fn_1408dffd0 as *const ()), // sub_1408dffd0
    Replacement::draft(0x1408e0150, "content_1408e0150", fn_1408e0150 as *const ()), // sub_1408e0150
    Replacement::draft_throws(0x1408e02e0, "content_1408e02e0", fn_1408e02e0 as *const (), Throws::Discard), // sub_1408e02e0
    Replacement::draft_throws(0x1408e0470, "content_1408e0470", fn_1408e0470 as *const (), Throws::Discard), // sub_1408e0470
    Replacement::draft(0x1408e06d0, "content_1408e06d0", fn_1408e06d0 as *const ()), // sub_1408e06d0
    Replacement::draft(0x1408e07d0, "content_1408e07d0", fn_1408e07d0 as *const ()), // sub_1408e07d0
    Replacement::draft(0x1408e0930, "content_1408e0930", fn_1408e0930 as *const ()), // sub_1408e0930
    Replacement::draft(0x1408e0a60, "content_1408e0a60", fn_1408e0a60 as *const ()), // sub_1408e0a60
    Replacement::draft_throws(0x1408e0c00, "content_1408e0c00", fn_1408e0c00 as *const (), Throws::Discard), // sub_1408e0c00
    Replacement::draft_throws(0x1408e0da0, "content_1408e0da0", fn_1408e0da0 as *const (), Throws::Discard), // sub_1408e0da0
    Replacement::draft_throws(0x1408e0f70, "content_1408e0f70", fn_1408e0f70 as *const (), Throws::Discard), // sub_1408e0f70
    Replacement::draft_throws(0x1408e1110, "content_1408e1110", fn_1408e1110 as *const (), Throws::Discard), // sub_1408e1110
    Replacement::draft_throws(0x1408e1190, "content_1408e1190", fn_1408e1190 as *const (), Throws::Caught), // sub_1408e1190
    Replacement::draft_throws(0x1408e1610, "content_1408e1610", fn_1408e1610 as *const (), Throws::Caught), // sub_1408e1610
    Replacement::draft_throws(0x1408e1a30, "content_1408e1a30", fn_1408e1a30 as *const (), Throws::Caught), // sub_1408e1a30
    Replacement::draft(0x1408e1e20, "content_1408e1e20", fn_1408e1e20 as *const ()), // sub_1408e1e20
    Replacement::draft(0x1408e2000, "content_1408e2000", fn_1408e2000 as *const ()), // sub_1408e2000
    Replacement::draft(0x1408e2190, "content_1408e2190", fn_1408e2190 as *const ()), // sub_1408e2190
    Replacement::draft_throws(0x1408e2340, "content_1408e2340", fn_1408e2340 as *const (), Throws::Discard), // sub_1408e2340
    Replacement::draft_throws(0x1408e24d0, "content_1408e24d0", fn_1408e24d0 as *const (), Throws::Discard), // sub_1408e24d0
    Replacement::draft(0x1408e2650, "content_1408e2650", fn_1408e2650 as *const ()), // sub_1408e2650
    Replacement::draft(0x1408e2730, "content_1408e2730", fn_1408e2730 as *const ()), // sub_1408e2730
    Replacement::draft(0x1408e27c0, "content_1408e27c0", fn_1408e27c0 as *const ()), // sub_1408e27c0
    Replacement::draft(0x1408e2910, "content_1408e2910", fn_1408e2910 as *const ()), // sub_1408e2910
    Replacement::draft_throws(0x1408e2a30, "content_1408e2a30", fn_1408e2a30 as *const (), Throws::Discard), // sub_1408e2a30
    Replacement::draft(0x1408e2b50, "content_1408e2b50", fn_1408e2b50 as *const ()), // sub_1408e2b50
    Replacement::draft_throws(0x1408e2c50, "content_1408e2c50", fn_1408e2c50 as *const (), Throws::Discard), // sub_1408e2c50
    Replacement::draft_throws(0x1408e2d80, "content_1408e2d80", fn_1408e2d80 as *const (), Throws::Caught), // sub_1408e2d80
    Replacement::draft_throws(0x1408e2e50, "content_1408e2e50", fn_1408e2e50 as *const (), Throws::Caught), // sub_1408e2e50
    Replacement::draft_throws(0x1408e2f20, "content_1408e2f20", fn_1408e2f20 as *const (), Throws::Caught), // sub_1408e2f20
    Replacement::draft(0x1408e2ff0, "content_1408e2ff0", fn_1408e2ff0 as *const ()), // sub_1408e2ff0
    Replacement::draft(0x1408e30c0, "content_1408e30c0", fn_1408e30c0 as *const ()), // sub_1408e30c0
    Replacement::draft(0x1408e31b0, "content_1408e31b0", fn_1408e31b0 as *const ()), // sub_1408e31b0
    Replacement::draft_throws(0x1408e32c0, "content_1408e32c0", fn_1408e32c0 as *const (), Throws::Discard), // sub_1408e32c0
    Replacement::draft_throws(0x1408e33b0, "content_1408e33b0", fn_1408e33b0 as *const (), Throws::Discard), // sub_1408e33b0
    Replacement::draft(0x1408e3490, "content_1408e3490", tpl_8433::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143276440, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_1408e3490
    Replacement::draft(0x1408e3530, "content_1408e3530", tpl_8434::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143276440, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1408e3530
    Replacement::draft(0x1408e35e0, "content_1408e35e0", fn_1408e35e0 as *const ()), // sub_1408e35e0
    Replacement::draft(0x1408e36c0, "content_1408e36c0", fn_1408e36c0 as *const ()), // sub_1408e36c0
    Replacement::draft_throws(0x1408e3810, "content_1408e3810", tpl_10278::<0x14387d9a8, 0x14387d9b0, 0x1401e9fa0, 0x14387e8f8, 0x14387e768, 0x1401e9e70, 0x8, 0x1408e5720, 0x18, 0x1408e58d0, 0x28> as *const (), Throws::Discard), // sub_1408e3810
    Replacement::draft_throws(0x1408e38c0, "content_1408e38c0", fn_1408e38c0 as *const (), Throws::Discard), // sub_1408e38c0
    Replacement::draft_throws(0x1408e3a70, "content_1408e3a70", tpl_10278::<0x14387d9a8, 0x14387d9b0, 0x1401e9b60, 0x14387e8f8, 0x14387e768, 0x1401e9a20, 0x8, 0x1408e5960, 0x18, 0x1408e5b20, 0x28> as *const (), Throws::Discard), // sub_1408e3a70
    Replacement::draft_throws(0x1408e3b20, "content_1408e3b20", fn_1408e3b20 as *const (), Throws::Discard), // sub_1408e3b20
    Replacement::draft_throws(0x1408e3d10, "content_1408e3d10", fn_1408e3d10 as *const (), Throws::Caught), // sub_1408e3d10
    Replacement::draft_throws(0x1408e3df0, "content_1408e3df0", fn_1408e3df0 as *const (), Throws::Caught), // sub_1408e3df0
    Replacement::draft_throws(0x1408e3ed0, "content_1408e3ed0", fn_1408e3ed0 as *const (), Throws::Caught), // sub_1408e3ed0
    Replacement::draft(0x1408e3fb0, "content_1408e3fb0", fn_1408e3fb0 as *const ()), // sub_1408e3fb0
    Replacement::draft(0x1408e4140, "content_1408e4140", fn_1408e4140 as *const ()), // sub_1408e4140
    Replacement::draft(0x1408e42c0, "content_1408e42c0", fn_1408e42c0 as *const ()), // sub_1408e42c0
    Replacement::draft_throws(0x1408e4450, "content_1408e4450", fn_1408e4450 as *const (), Throws::Discard), // sub_1408e4450
    Replacement::draft_throws(0x1408e45b0, "content_1408e45b0", fn_1408e45b0 as *const (), Throws::Discard), // sub_1408e45b0
    Replacement::draft(0x1408e46b0, "content_1408e46b0", fn_1408e46b0 as *const ()), // sub_1408e46b0
    Replacement::draft(0x1408e4760, "content_1408e4760", fn_1408e4760 as *const ()), // sub_1408e4760
    Replacement::draft_throws(0x1408e4870, "content_1408e4870", fn_1408e4870 as *const (), Throws::Discard), // sub_1408e4870
    Replacement::draft_throws(0x1408e48f0, "content_1408e48f0", fn_1408e48f0 as *const (), Throws::Discard), // sub_1408e48f0
    Replacement::draft(0x1408e49b0, "content_1408e49b0", fn_1408e49b0 as *const ()), // content::GfxGraphNode::content::GfxGraphNode::vfunc_0
    Replacement::draft(0x1408e4a30, "content_1408e4a30", fn_1408e4a30 as *const ()), // content::GfxGraph::content::GfxGraph::vfunc_0
    Replacement::draft(0x1408e4a90, "unknown_1408e4a90", tpl_170::<0x1408e6420> as *const ()), // sub_1408e4a90
    Replacement::draft(0x1408e4aa0, "d3d_1408e4aa0", tpl_10290::<0x141834d30> as *const ()), // d3d::MeshShader::d3d::MeshShader::vfunc_0
    Replacement::draft(0x1408e4ad0, "unknown_1408e4ad0", fn_1408e4ad0 as *const ()), // sub_1408e4ad0
    Replacement::draft(0x1408e4ba0, "unknown_1408e4ba0", fn_1408e4ba0 as *const ()), // sub_1408e4ba0
    Replacement::draft(0x1408e4cc0, "unknown_1408e4cc0", fn_1408e4cc0 as *const ()), // sub_1408e4cc0
    Replacement::draft_throws(0x1408e4d70, "unknown_1408e4d70", fn_1408e4d70 as *const (), Throws::Caught), // sub_1408e4d70
    Replacement::draft_throws(0x1408e4f20, "unknown_1408e4f20", fn_1408e4f20 as *const (), Throws::Caught), // sub_1408e4f20
    Replacement::draft_throws(0x1408e50e0, "unknown_1408e50e0", fn_1408e50e0 as *const (), Throws::Discard), // sub_1408e50e0
    Replacement::draft_throws(0x1408e5160, "unknown_1408e5160", tpl_838::<0x1408e64f0> as *const (), Throws::Discard), // sub_1408e5160
    Replacement::draft_throws(0x1408e5170, "unknown_1408e5170", tpl_838::<0x1408e67f0> as *const (), Throws::Discard), // sub_1408e5170
    Replacement::draft_throws(0x1408e5180, "unknown_1408e5180", tpl_838::<0x1408e6ab0> as *const (), Throws::Discard), // sub_1408e5180
    Replacement::draft_throws(0x1408e5190, "unknown_1408e5190", fn_1408e5190 as *const (), Throws::Discard), // sub_1408e5190
    Replacement::draft(0x1408e51e0, "unknown_1408e51e0", fn_1408e51e0 as *const ()), // sub_1408e51e0
    Replacement::draft_throws(0x1408e5270, "unknown_1408e5270", fn_1408e5270 as *const (), Throws::Caught), // sub_1408e5270
    Replacement::draft(0x1408e5680, "unknown_1408e5680", tpl_9530::<0x8, 0x1426d4580, 0x338, 0x1, 0x1408ec530> as *const ()), // sub_1408e5680
    Replacement::draft_throws(0x1408e5720, "unknown_1408e5720", fn_1408e5720 as *const (), Throws::Caught), // sub_1408e5720
    Replacement::draft_throws(0x1408e58d0, "unknown_1408e58d0", fn_1408e58d0 as *const (), Throws::Discard), // sub_1408e58d0
    Replacement::draft_throws(0x1408e5960, "unknown_1408e5960", fn_1408e5960 as *const (), Throws::Caught), // sub_1408e5960
    Replacement::draft_throws(0x1408e5b20, "unknown_1408e5b20", fn_1408e5b20 as *const (), Throws::Discard), // sub_1408e5b20
    Replacement::draft_throws(0x1408e5bc0, "unknown_1408e5bc0", fn_1408e5bc0 as *const (), Throws::Caught), // sub_1408e5bc0
    Replacement::draft_throws(0x1408e5fd0, "unknown_1408e5fd0", tpl_838::<0x1408e7000> as *const (), Throws::Discard), // sub_1408e5fd0
    Replacement::draft_throws(0x1408e5fe0, "unknown_1408e5fe0", tpl_838::<0x1408e7300> as *const (), Throws::Discard), // sub_1408e5fe0
    Replacement::draft_throws(0x1408e5ff0, "unknown_1408e5ff0", tpl_838::<0x1408e75d0> as *const (), Throws::Discard), // sub_1408e5ff0
    Replacement::draft_throws(0x1408e6000, "unknown_1408e6000", tpl_838::<0x1408e7890> as *const (), Throws::Discard), // sub_1408e6000
    Replacement::draft_throws(0x1408e6010, "unknown_1408e6010", tpl_838::<0x1408e7b30> as *const (), Throws::Discard), // sub_1408e6010
    Replacement::draft_throws(0x1408e6020, "unknown_1408e6020", tpl_838::<0x1408e7dd0> as *const (), Throws::Discard), // sub_1408e6020
    Replacement::draft_throws(0x1408e6030, "unknown_1408e6030", fn_1408e6030 as *const (), Throws::Discard), // sub_1408e6030
    Replacement::draft(0x1408e60d0, "unknown_1408e60d0", fn_1408e60d0 as *const ()), // sub_1408e60d0
    Replacement::draft(0x1408e6190, "unknown_1408e6190", fn_1408e6190 as *const ()), // sub_1408e6190
    Replacement::draft(0x1408e6270, "unknown_1408e6270", fn_1408e6270 as *const ()), // sub_1408e6270
    Replacement::draft(0x1408e6340, "unknown_1408e6340", fn_1408e6340 as *const ()), // sub_1408e6340
    Replacement::draft(0x1408e6420, "unknown_1408e6420", fn_1408e6420 as *const ()), // sub_1408e6420
    Replacement::draft_throws(0x1408e64f0, "unknown_1408e64f0", fn_1408e64f0 as *const (), Throws::Caught), // sub_1408e64f0
    Replacement::draft_throws(0x1408e67f0, "unknown_1408e67f0", fn_1408e67f0 as *const (), Throws::Caught), // sub_1408e67f0
    Replacement::draft_throws(0x1408e6ab0, "unknown_1408e6ab0", fn_1408e6ab0 as *const (), Throws::Caught), // sub_1408e6ab0
    Replacement::draft_throws(0x1408e6d50, "unknown_1408e6d50", fn_1408e6d50 as *const (), Throws::Caught), // sub_1408e6d50
    Replacement::draft(0x1408e6e20, "unknown_1408e6e20", tpl_10167::<0x238, 0x18> as *const ()), // sub_1408e6e20
    Replacement::draft(0x1408e6e40, "unknown_1408e6e40", tpl_10167::<0x238, 0x8> as *const ()), // sub_1408e6e40
    Replacement::draft_throws(0x1408e6e60, "unknown_1408e6e60", fn_1408e6e60 as *const (), Throws::Caught), // sub_1408e6e60
    Replacement::draft_throws(0x1408e6ef0, "unknown_1408e6ef0", tpl_1908::<0x14387e8c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1408e6ef0
    Replacement::draft_throws(0x1408e6f70, "unknown_1408e6f70", fn_1408e6f70 as *const (), Throws::Caught), // sub_1408e6f70
    Replacement::draft_throws(0x1408e7000, "unknown_1408e7000", fn_1408e7000 as *const (), Throws::Caught), // sub_1408e7000
    Replacement::draft_throws(0x1408e7300, "unknown_1408e7300", fn_1408e7300 as *const (), Throws::Caught), // sub_1408e7300
    Replacement::draft_throws(0x1408e75d0, "unknown_1408e75d0", fn_1408e75d0 as *const (), Throws::Caught), // sub_1408e75d0
    Replacement::draft_throws(0x1408e7890, "unknown_1408e7890", fn_1408e7890 as *const (), Throws::Caught), // sub_1408e7890
    Replacement::draft_throws(0x1408e7b30, "unknown_1408e7b30", fn_1408e7b30 as *const (), Throws::Caught), // sub_1408e7b30
    Replacement::draft_throws(0x1408e7dd0, "unknown_1408e7dd0", fn_1408e7dd0 as *const (), Throws::Caught), // sub_1408e7dd0
    Replacement::draft_throws(0x1408e8060, "unknown_1408e8060", tpl_1908::<0x14387e768, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1408e8060
    Replacement::draft_throws(0x1408e80e0, "unknown_1408e80e0", fn_1408e80e0 as *const (), Throws::Caught), // sub_1408e80e0
    Replacement::draft_throws(0x1408e81b0, "unknown_1408e81b0", fn_1408e81b0 as *const (), Throws::Caught), // sub_1408e81b0
    Replacement::draft(0x1408e8310, "unknown_1408e8310", fn_1408e8310 as *const ()), // sub_1408e8310
    Replacement::draft(0x1408e8350, "unknown_1408e8350", fn_1408e8350 as *const ()), // sub_1408e8350
    Replacement::draft_throws(0x1408e8390, "unknown_1408e8390", fn_1408e8390 as *const (), Throws::Discard), // sub_1408e8390
    Replacement::draft_throws(0x1408e89d0, "unknown_1408e89d0", fn_1408e89d0 as *const (), Throws::Discard), // sub_1408e89d0
    Replacement::draft_throws(0x1408e8df0, "unknown_1408e8df0", fn_1408e8df0 as *const (), Throws::Caught), // sub_1408e8df0
    Replacement::draft_throws(0x1408e92c0, "unknown_1408e92c0", fn_1408e92c0 as *const (), Throws::Caught), // sub_1408e92c0
    Replacement::draft_throws(0x1408e9750, "unknown_1408e9750", fn_1408e9750 as *const (), Throws::Caught), // sub_1408e9750
    Replacement::draft_throws(0x1408e9be0, "unknown_1408e9be0", fn_1408e9be0 as *const (), Throws::Caught), // sub_1408e9be0
    Replacement::draft_throws(0x1408e9d10, "unknown_1408e9d10", fn_1408e9d10 as *const (), Throws::Caught), // sub_1408e9d10
    Replacement::draft_throws(0x1408e9e40, "unknown_1408e9e40", fn_1408e9e40 as *const (), Throws::Caught), // sub_1408e9e40
    Replacement::draft_throws(0x1408ea310, "unknown_1408ea310", fn_1408ea310 as *const (), Throws::Caught), // sub_1408ea310
    Replacement::draft_throws(0x1408ea820, "unknown_1408ea820", fn_1408ea820 as *const (), Throws::Caught), // sub_1408ea820
    Replacement::draft_throws(0x1408eacb0, "unknown_1408eacb0", fn_1408eacb0 as *const (), Throws::Caught), // sub_1408eacb0
    Replacement::draft_throws(0x1408eb170, "unknown_1408eb170", fn_1408eb170 as *const (), Throws::Caught), // sub_1408eb170
    Replacement::draft_throws(0x1408eb600, "unknown_1408eb600", fn_1408eb600 as *const (), Throws::Caught), // sub_1408eb600
    Replacement::draft(0x1408ebac0, "unknown_1408ebac0", fn_1408ebac0 as *const ()), // sub_1408ebac0
    Replacement::draft(0x1408ebd60, "unknown_1408ebd60", fn_1408ebd60 as *const ()), // sub_1408ebd60
    Replacement::draft_throws(0x1408ebf80, "unknown_1408ebf80", fn_1408ebf80 as *const (), Throws::Caught), // sub_1408ebf80
    Replacement::draft_throws(0x1408ec0c0, "unknown_1408ec0c0", fn_1408ec0c0 as *const (), Throws::Caught), // sub_1408ec0c0
    Replacement::draft_throws(0x1408ec200, "unknown_1408ec200", fn_1408ec200 as *const (), Throws::Caught), // sub_1408ec200
    Replacement::draft_throws(0x1408ec340, "unknown_1408ec340", fn_1408ec340 as *const (), Throws::Caught), // sub_1408ec340
    Replacement::draft(0x1408ec530, "unknown_1408ec530", fn_1408ec530 as *const ()), // sub_1408ec530
    Replacement::draft_throws(0x1408ec6f0, "unknown_1408ec6f0", fn_1408ec6f0 as *const (), Throws::Caught), // sub_1408ec6f0
    Replacement::draft_throws(0x1408ec830, "unknown_1408ec830", fn_1408ec830 as *const (), Throws::Caught), // sub_1408ec830
    Replacement::draft_throws(0x1408ec970, "unknown_1408ec970", fn_1408ec970 as *const (), Throws::Caught), // sub_1408ec970
    Replacement::draft_throws(0x1408ecab0, "unknown_1408ecab0", fn_1408ecab0 as *const (), Throws::Caught), // sub_1408ecab0
    Replacement::draft_throws(0x1408ecca0, "unknown_1408ecca0", fn_1408ecca0 as *const (), Throws::Caught), // sub_1408ecca0
    Replacement::draft(0x1408ece90, "unknown_1408ece90", fn_1408ece90 as *const ()), // sub_1408ece90
    Replacement::draft_throws(0x1408ecf30, "unknown_1408ecf30", fn_1408ecf30 as *const (), Throws::Caught), // sub_1408ecf30
    Replacement::draft_throws(0x1408ed3a0, "unknown_1408ed3a0", fn_1408ed3a0 as *const (), Throws::Caught), // sub_1408ed3a0
    Replacement::draft_throws(0x1408ed7d0, "unknown_1408ed7d0", fn_1408ed7d0 as *const (), Throws::Caught), // sub_1408ed7d0
    Replacement::draft_throws(0x1408edb20, "unknown_1408edb20", fn_1408edb20 as *const (), Throws::Caught), // sub_1408edb20
    Replacement::draft(0x1408edcd0, "unknown_1408edcd0", fn_1408edcd0 as *const ()), // sub_1408edcd0
    Replacement::draft(0x1408ede60, "unknown_1408ede60", fn_1408ede60 as *const ()), // sub_1408ede60
    Replacement::draft_throws(0x1408edfa0, "unknown_1408edfa0", fn_1408edfa0 as *const (), Throws::Caught), // sub_1408edfa0
    Replacement::draft_throws(0x1408ee150, "unknown_1408ee150", fn_1408ee150 as *const (), Throws::Caught), // sub_1408ee150
    Replacement::draft_throws(0x1408ee5c0, "unknown_1408ee5c0", fn_1408ee5c0 as *const (), Throws::Caught), // sub_1408ee5c0
    Replacement::draft_throws(0x1408eea30, "unknown_1408eea30", fn_1408eea30 as *const (), Throws::Caught), // sub_1408eea30
    Replacement::draft_throws(0x1408eee60, "unknown_1408eee60", fn_1408eee60 as *const (), Throws::Caught), // sub_1408eee60
    Replacement::draft_throws(0x1408ef290, "unknown_1408ef290", fn_1408ef290 as *const (), Throws::Caught), // sub_1408ef290
    Replacement::draft_throws(0x1408ef5e0, "unknown_1408ef5e0", fn_1408ef5e0 as *const (), Throws::Caught), // sub_1408ef5e0
    Replacement::draft(0x1408ef930, "unknown_1408ef930", fn_1408ef930 as *const ()), // sub_1408ef930
    Replacement::draft(0x1408efaa0, "unknown_1408efaa0", fn_1408efaa0 as *const ()), // sub_1408efaa0
    Replacement::draft_throws(0x1408efbf0, "unknown_1408efbf0", fn_1408efbf0 as *const (), Throws::Caught), // sub_1408efbf0
    Replacement::draft_throws(0x1408efd20, "unknown_1408efd20", fn_1408efd20 as *const (), Throws::Caught), // sub_1408efd20
    Replacement::draft_throws(0x1408efeb0, "unknown_1408efeb0", fn_1408efeb0 as *const (), Throws::Caught), // sub_1408efeb0
    Replacement::draft_throws(0x1408f0040, "unknown_1408f0040", fn_1408f0040 as *const (), Throws::Caught), // sub_1408f0040
    Replacement::draft_throws(0x1408f01d0, "unknown_1408f01d0", fn_1408f01d0 as *const (), Throws::Caught), // sub_1408f01d0
    Replacement::draft(0x1408f0360, "unknown_1408f0360", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143084e28> as *const ()), // sub_1408f0360
    Replacement::draft(0x1408f03c0, "unknown_1408f03c0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143084e28> as *const ()), // sub_1408f03c0
    Replacement::draft(0x1408f04d0, "unknown_1408f04d0", fn_1408f04d0 as *const ()), // sub_1408f04d0
    Replacement::draft(0x1408f0520, "content_1408f0520", fn_1408f0520 as *const ()), // content::GenericEntity::content::GenericEntity::vfunc_21
    Replacement::draft(0x1408f05b0, "content_1408f05b0", fn_1408f05b0 as *const ()), // content::GenericEntity::content::GenericEntity::vfunc_0
    Replacement::draft(0x1408f0640, "content_1408f0640", fn_1408f0640 as *const ()), // content::GIVolumeEntity::content::GIVolumeEntity::vfunc_26
    Replacement::draft(0x1408f0840, "content_1408f0840", fn_1408f0840 as *const ()), // content::GIVolumeEntity::content::GIVolumeEntity::vfunc_30
    Replacement::draft(0x1408f0960, "content_1408f0960", fn_1408f0960 as *const ()), // content::HierarchyLODComponent::content::HierarchyLODComponent::vfunc_4
    Replacement::draft(0x1408f0970, "content_1408f0970", fn_1408f0970 as *const ()), // content::HierarchyLODComponent::content::HierarchyLODComponent::vfunc_18
    Replacement::draft(0x1408f0b20, "content_1408f0b20", fn_1408f0b20 as *const ()), // sub_1408f0b20
    Replacement::draft(0x1408f0c30, "content_1408f0c30", fn_1408f0c30 as *const ()), // sub_1408f0c30
    Replacement::draft(0x1408f0d80, "content_1408f0d80", fn_1408f0d80 as *const ()), // sub_1408f0d80
    Replacement::draft(0x1408f0ec0, "content_1408f0ec0", fn_1408f0ec0 as *const ()), // sub_1408f0ec0
    Replacement::draft(0x1408f0fd0, "content_1408f0fd0", fn_1408f0fd0 as *const ()), // sub_1408f0fd0
    Replacement::draft(0x1408f10e0, "content_1408f10e0", fn_1408f10e0 as *const ()), // sub_1408f10e0
    Replacement::draft(0x1408f1180, "content_1408f1180", fn_1408f1180 as *const ()), // sub_1408f1180
    Replacement::draft(0x1408f12c0, "content_1408f12c0", fn_1408f12c0 as *const ()), // sub_1408f12c0
    Replacement::draft(0x1408f13d0, "content_1408f13d0", fn_1408f13d0 as *const ()), // sub_1408f13d0
    Replacement::draft(0x1408f14e0, "content_1408f14e0", fn_1408f14e0 as *const ()), // sub_1408f14e0
    Replacement::draft(0x1408f1580, "content_1408f1580", fn_1408f1580 as *const ()), // sub_1408f1580
    Replacement::draft(0x1408f16c0, "content_1408f16c0", fn_1408f16c0 as *const ()), // sub_1408f16c0
    Replacement::draft_throws(0x1408f1880, "content_1408f1880", fn_1408f1880 as *const (), Throws::Caught), // sub_1408f1880
    Replacement::draft(0x1408f1a30, "content_1408f1a30", fn_1408f1a30 as *const ()), // sub_1408f1a30
    Replacement::draft_throws(0x1408f1b70, "content_1408f1b70", fn_1408f1b70 as *const (), Throws::Caught), // sub_1408f1b70
    Replacement::draft(0x1408f1d20, "content_1408f1d20", fn_1408f1d20 as *const ()), // sub_1408f1d20
    Replacement::draft_throws(0x1408f1e30, "content_1408f1e30", fn_1408f1e30 as *const (), Throws::Caught), // sub_1408f1e30
    Replacement::draft_throws(0x1408f1fc0, "content_1408f1fc0", fn_1408f1fc0 as *const (), Throws::Caught), // sub_1408f1fc0
    Replacement::draft_throws(0x1408f2150, "content_1408f2150", fn_1408f2150 as *const (), Throws::Caught), // sub_1408f2150
    Replacement::draft(0x1408f22e0, "content_1408f22e0", fn_1408f22e0 as *const ()), // sub_1408f22e0
    Replacement::draft(0x1408f2470, "content_1408f2470", fn_1408f2470 as *const ()), // sub_1408f2470
    Replacement::draft(0x1408f25e0, "content_1408f25e0", fn_1408f25e0 as *const ()), // sub_1408f25e0
    Replacement::draft_throws(0x1408f2730, "content_1408f2730", fn_1408f2730 as *const (), Throws::Caught), // sub_1408f2730
    Replacement::draft_throws(0x1408f2860, "content_1408f2860", fn_1408f2860 as *const (), Throws::Caught), // sub_1408f2860
    Replacement::draft(0x1408f29f0, "content_1408f29f0", fn_1408f29f0 as *const ()), // sub_1408f29f0
    Replacement::draft(0x1408f2b00, "content_1408f2b00", fn_1408f2b00 as *const ()), // sub_1408f2b00
    Replacement::draft(0x1408f2ba0, "content_1408f2ba0", fn_1408f2ba0 as *const ()), // sub_1408f2ba0
    Replacement::draft(0x1408f2c40, "content_1408f2c40", fn_1408f2c40 as *const ()), // sub_1408f2c40
    Replacement::draft(0x1408f2d80, "content_1408f2d80", fn_1408f2d80 as *const ()), // sub_1408f2d80
    Replacement::draft_throws(0x1408f2f40, "content_1408f2f40", fn_1408f2f40 as *const (), Throws::Caught), // sub_1408f2f40
    Replacement::draft(0x1408f30f0, "content_1408f30f0", fn_1408f30f0 as *const ()), // sub_1408f30f0
    Replacement::draft_throws(0x1408f3230, "content_1408f3230", fn_1408f3230 as *const (), Throws::Caught), // sub_1408f3230
    Replacement::draft(0x1408f33e0, "content_1408f33e0", fn_1408f33e0 as *const ()), // sub_1408f33e0
    Replacement::draft_throws(0x1408f34f0, "content_1408f34f0", fn_1408f34f0 as *const (), Throws::Caught), // sub_1408f34f0
    Replacement::draft_throws(0x1408f3680, "content_1408f3680", fn_1408f3680 as *const (), Throws::Caught), // sub_1408f3680
    Replacement::draft_throws(0x1408f3810, "content_1408f3810", fn_1408f3810 as *const (), Throws::Caught), // sub_1408f3810
    Replacement::draft(0x1408f39a0, "content_1408f39a0", fn_1408f39a0 as *const ()), // sub_1408f39a0
    Replacement::draft(0x1408f3b30, "content_1408f3b30", fn_1408f3b30 as *const ()), // sub_1408f3b30
    Replacement::draft(0x1408f3ca0, "content_1408f3ca0", fn_1408f3ca0 as *const ()), // sub_1408f3ca0
    Replacement::draft_throws(0x1408f3df0, "content_1408f3df0", fn_1408f3df0 as *const (), Throws::Caught), // sub_1408f3df0
    Replacement::draft_throws(0x1408f3f20, "content_1408f3f20", fn_1408f3f20 as *const (), Throws::Caught), // sub_1408f3f20
    Replacement::draft(0x1408f40b0, "content_1408f40b0", fn_1408f40b0 as *const ()), // sub_1408f40b0
    Replacement::draft(0x1408f41c0, "content_1408f41c0", fn_1408f41c0 as *const ()), // sub_1408f41c0
    Replacement::draft(0x1408f4260, "content_1408f4260", fn_1408f4260 as *const ()), // sub_1408f4260
    Replacement::draft(0x1408f4300, "content_1408f4300", fn_1408f4300 as *const ()), // sub_1408f4300
    Replacement::draft(0x1408f4440, "content_1408f4440", fn_1408f4440 as *const ()), // sub_1408f4440
    Replacement::draft_throws(0x1408f4600, "content_1408f4600", fn_1408f4600 as *const (), Throws::Caught), // sub_1408f4600
    Replacement::draft(0x1408f47b0, "content_1408f47b0", fn_1408f47b0 as *const ()), // sub_1408f47b0
    Replacement::draft_throws(0x1408f48f0, "content_1408f48f0", fn_1408f48f0 as *const (), Throws::Caught), // sub_1408f48f0
    Replacement::draft(0x1408f4aa0, "content_1408f4aa0", fn_1408f4aa0 as *const ()), // sub_1408f4aa0
    Replacement::draft_throws(0x1408f4bb0, "content_1408f4bb0", fn_1408f4bb0 as *const (), Throws::Caught), // sub_1408f4bb0
    Replacement::draft_throws(0x1408f4d40, "content_1408f4d40", fn_1408f4d40 as *const (), Throws::Caught), // sub_1408f4d40
    Replacement::draft_throws(0x1408f4ed0, "content_1408f4ed0", fn_1408f4ed0 as *const (), Throws::Caught), // sub_1408f4ed0
    Replacement::draft(0x1408f5060, "content_1408f5060", fn_1408f5060 as *const ()), // sub_1408f5060
    Replacement::draft(0x1408f51f0, "content_1408f51f0", fn_1408f51f0 as *const ()), // sub_1408f51f0
    Replacement::draft(0x1408f5360, "content_1408f5360", fn_1408f5360 as *const ()), // sub_1408f5360
    Replacement::draft_throws(0x1408f54b0, "content_1408f54b0", fn_1408f54b0 as *const (), Throws::Caught), // sub_1408f54b0
    Replacement::draft_throws(0x1408f55e0, "content_1408f55e0", fn_1408f55e0 as *const (), Throws::Caught), // sub_1408f55e0
    Replacement::draft(0x1408f5770, "content_1408f5770", fn_1408f5770 as *const ()), // sub_1408f5770
    Replacement::draft(0x1408f5880, "content_1408f5880", fn_1408f5880 as *const ()), // sub_1408f5880
    Replacement::draft(0x1408f5920, "content_1408f5920", fn_1408f5920 as *const ()), // sub_1408f5920
    Replacement::draft(0x1408f59c0, "content_1408f59c0", fn_1408f59c0 as *const ()), // sub_1408f59c0
    Replacement::draft(0x1408f5b00, "content_1408f5b00", fn_1408f5b00 as *const ()), // sub_1408f5b00
    Replacement::draft_throws(0x1408f5cc0, "content_1408f5cc0", fn_1408f5cc0 as *const (), Throws::Caught), // sub_1408f5cc0
    Replacement::draft(0x1408f5e70, "content_1408f5e70", fn_1408f5e70 as *const ()), // sub_1408f5e70
    Replacement::draft_throws(0x1408f5fb0, "content_1408f5fb0", fn_1408f5fb0 as *const (), Throws::Caught), // sub_1408f5fb0
    Replacement::draft(0x1408f6160, "content_1408f6160", fn_1408f6160 as *const ()), // sub_1408f6160
    Replacement::draft_throws(0x1408f6270, "content_1408f6270", fn_1408f6270 as *const (), Throws::Caught), // sub_1408f6270
    Replacement::draft_throws(0x1408f6400, "content_1408f6400", fn_1408f6400 as *const (), Throws::Caught), // sub_1408f6400
    Replacement::draft_throws(0x1408f6590, "content_1408f6590", fn_1408f6590 as *const (), Throws::Caught), // sub_1408f6590
    Replacement::draft(0x1408f6720, "content_1408f6720", fn_1408f6720 as *const ()), // sub_1408f6720
    Replacement::draft(0x1408f68b0, "content_1408f68b0", fn_1408f68b0 as *const ()), // sub_1408f68b0
    Replacement::draft(0x1408f6a20, "content_1408f6a20", fn_1408f6a20 as *const ()), // sub_1408f6a20
    Replacement::draft_throws(0x1408f6b70, "content_1408f6b70", fn_1408f6b70 as *const (), Throws::Caught), // sub_1408f6b70
    Replacement::draft_throws(0x1408f6ca0, "content_1408f6ca0", fn_1408f6ca0 as *const (), Throws::Caught), // sub_1408f6ca0
    Replacement::draft(0x1408f6e30, "content_1408f6e30", fn_1408f6e30 as *const ()), // sub_1408f6e30
    Replacement::draft(0x1408f6f40, "content_1408f6f40", fn_1408f6f40 as *const ()), // sub_1408f6f40
    Replacement::draft(0x1408f6fe0, "content_1408f6fe0", fn_1408f6fe0 as *const ()), // sub_1408f6fe0
    Replacement::draft(0x1408f7080, "content_1408f7080", fn_1408f7080 as *const ()), // sub_1408f7080
    Replacement::draft(0x1408f71c0, "content_1408f71c0", fn_1408f71c0 as *const ()), // sub_1408f71c0
    Replacement::draft(0x1408f72d0, "content_1408f72d0", fn_1408f72d0 as *const ()), // sub_1408f72d0
    Replacement::draft(0x1408f73e0, "content_1408f73e0", fn_1408f73e0 as *const ()), // sub_1408f73e0
    Replacement::draft(0x1408f7480, "content_1408f7480", fn_1408f7480 as *const ()), // sub_1408f7480
    Replacement::draft(0x1408f75c0, "content_1408f75c0", fn_1408f75c0 as *const ()), // sub_1408f75c0
    Replacement::draft_throws(0x1408f7780, "content_1408f7780", fn_1408f7780 as *const (), Throws::Caught), // sub_1408f7780
    Replacement::draft(0x1408f7930, "content_1408f7930", fn_1408f7930 as *const ()), // sub_1408f7930
    Replacement::draft_throws(0x1408f7a70, "content_1408f7a70", fn_1408f7a70 as *const (), Throws::Caught), // sub_1408f7a70
    Replacement::draft(0x1408f7c20, "content_1408f7c20", fn_1408f7c20 as *const ()), // sub_1408f7c20
    Replacement::draft_throws(0x1408f7d30, "content_1408f7d30", fn_1408f7d30 as *const (), Throws::Caught), // sub_1408f7d30
    Replacement::draft_throws(0x1408f7ec0, "content_1408f7ec0", fn_1408f7ec0 as *const (), Throws::Caught), // sub_1408f7ec0
    Replacement::draft_throws(0x1408f8050, "content_1408f8050", fn_1408f8050 as *const (), Throws::Caught), // sub_1408f8050
    Replacement::draft(0x1408f81e0, "content_1408f81e0", fn_1408f81e0 as *const ()), // sub_1408f81e0
    Replacement::draft(0x1408f8370, "content_1408f8370", fn_1408f8370 as *const ()), // sub_1408f8370
    Replacement::draft(0x1408f84e0, "content_1408f84e0", fn_1408f84e0 as *const ()), // sub_1408f84e0
    Replacement::draft_throws(0x1408f8630, "content_1408f8630", fn_1408f8630 as *const (), Throws::Caught), // sub_1408f8630
    Replacement::draft_throws(0x1408f8760, "content_1408f8760", fn_1408f8760 as *const (), Throws::Caught), // sub_1408f8760
    Replacement::draft(0x1408f88f0, "content_1408f88f0", fn_1408f88f0 as *const ()), // sub_1408f88f0
    Replacement::draft(0x1408f8a00, "content_1408f8a00", fn_1408f8a00 as *const ()), // sub_1408f8a00
    Replacement::draft(0x1408f8aa0, "content_1408f8aa0", fn_1408f8aa0 as *const ()), // sub_1408f8aa0
    Replacement::draft(0x1408f8b40, "content_1408f8b40", fn_1408f8b40 as *const ()), // sub_1408f8b40
    Replacement::draft(0x1408f8c80, "content_1408f8c80", fn_1408f8c80 as *const ()), // sub_1408f8c80
    Replacement::draft_throws(0x1408f8e40, "content_1408f8e40", fn_1408f8e40 as *const (), Throws::Caught), // sub_1408f8e40
    Replacement::draft(0x1408f8ff0, "content_1408f8ff0", fn_1408f8ff0 as *const ()), // sub_1408f8ff0
    Replacement::draft_throws(0x1408f9130, "content_1408f9130", fn_1408f9130 as *const (), Throws::Caught), // sub_1408f9130
    Replacement::draft(0x1408f92e0, "content_1408f92e0", fn_1408f92e0 as *const ()), // sub_1408f92e0
    Replacement::draft_throws(0x1408f93f0, "content_1408f93f0", fn_1408f93f0 as *const (), Throws::Caught), // sub_1408f93f0
    Replacement::draft_throws(0x1408f9580, "content_1408f9580", fn_1408f9580 as *const (), Throws::Caught), // sub_1408f9580
    Replacement::draft_throws(0x1408f9710, "content_1408f9710", fn_1408f9710 as *const (), Throws::Caught), // sub_1408f9710
    Replacement::draft(0x1408f98a0, "content_1408f98a0", fn_1408f98a0 as *const ()), // sub_1408f98a0
    Replacement::draft(0x1408f9a30, "content_1408f9a30", fn_1408f9a30 as *const ()), // sub_1408f9a30
    Replacement::draft(0x1408f9ba0, "content_1408f9ba0", fn_1408f9ba0 as *const ()), // sub_1408f9ba0
    Replacement::draft_throws(0x1408f9cf0, "content_1408f9cf0", fn_1408f9cf0 as *const (), Throws::Caught), // sub_1408f9cf0
    Replacement::draft_throws(0x1408f9e20, "content_1408f9e20", fn_1408f9e20 as *const (), Throws::Caught), // sub_1408f9e20
    Replacement::draft(0x1408f9fb0, "content_1408f9fb0", fn_1408f9fb0 as *const ()), // sub_1408f9fb0
    Replacement::draft(0x1408fa0c0, "content_1408fa0c0", fn_1408fa0c0 as *const ()), // sub_1408fa0c0
    Replacement::draft(0x1408fa160, "content_1408fa160", fn_1408fa160 as *const ()), // sub_1408fa160
    Replacement::draft(0x1408fa200, "content_1408fa200", fn_1408fa200 as *const ()), // sub_1408fa200
    Replacement::draft(0x1408fa340, "content_1408fa340", fn_1408fa340 as *const ()), // sub_1408fa340
    Replacement::draft(0x1408fa450, "content_1408fa450", fn_1408fa450 as *const ()), // sub_1408fa450
    Replacement::draft(0x1408fa560, "content_1408fa560", fn_1408fa560 as *const ()), // sub_1408fa560
    Replacement::draft(0x1408fa600, "content_1408fa600", fn_1408fa600 as *const ()), // sub_1408fa600
    Replacement::draft(0x1408fa740, "content_1408fa740", fn_1408fa740 as *const ()), // sub_1408fa740
    Replacement::draft(0x1408fa850, "content_1408fa850", fn_1408fa850 as *const ()), // sub_1408fa850
    Replacement::draft(0x1408fa960, "content_1408fa960", fn_1408fa960 as *const ()), // sub_1408fa960
    Replacement::draft(0x1408faa00, "content_1408faa00", fn_1408faa00 as *const ()), // sub_1408faa00
    Replacement::draft(0x1408fab40, "content_1408fab40", fn_1408fab40 as *const ()), // sub_1408fab40
    Replacement::draft(0x1408fac50, "content_1408fac50", fn_1408fac50 as *const ()), // sub_1408fac50
    Replacement::draft(0x1408fad60, "content_1408fad60", fn_1408fad60 as *const ()), // sub_1408fad60
    Replacement::draft(0x1408fae00, "content_1408fae00", fn_1408fae00 as *const ()), // sub_1408fae00
    Replacement::draft(0x1408faf40, "content_1408faf40", fn_1408faf40 as *const ()), // sub_1408faf40
    Replacement::draft_throws(0x1408fb100, "content_1408fb100", fn_1408fb100 as *const (), Throws::Caught), // sub_1408fb100
    Replacement::draft(0x1408fb2b0, "content_1408fb2b0", fn_1408fb2b0 as *const ()), // sub_1408fb2b0
    Replacement::draft_throws(0x1408fb3f0, "content_1408fb3f0", fn_1408fb3f0 as *const (), Throws::Caught), // sub_1408fb3f0
    Replacement::draft(0x1408fb5a0, "content_1408fb5a0", fn_1408fb5a0 as *const ()), // sub_1408fb5a0
    Replacement::draft_throws(0x1408fb6b0, "content_1408fb6b0", fn_1408fb6b0 as *const (), Throws::Caught), // sub_1408fb6b0
    Replacement::draft_throws(0x1408fb840, "content_1408fb840", fn_1408fb840 as *const (), Throws::Caught), // sub_1408fb840
    Replacement::draft_throws(0x1408fb9d0, "content_1408fb9d0", fn_1408fb9d0 as *const (), Throws::Caught), // sub_1408fb9d0
    Replacement::draft(0x1408fbb60, "content_1408fbb60", fn_1408fbb60 as *const ()), // sub_1408fbb60
    Replacement::draft(0x1408fbcf0, "content_1408fbcf0", fn_1408fbcf0 as *const ()), // sub_1408fbcf0
    Replacement::draft(0x1408fbe60, "content_1408fbe60", fn_1408fbe60 as *const ()), // sub_1408fbe60
    Replacement::draft_throws(0x1408fbfb0, "content_1408fbfb0", fn_1408fbfb0 as *const (), Throws::Caught), // sub_1408fbfb0
    Replacement::draft_throws(0x1408fc0e0, "content_1408fc0e0", fn_1408fc0e0 as *const (), Throws::Caught), // sub_1408fc0e0
    Replacement::draft(0x1408fc270, "content_1408fc270", fn_1408fc270 as *const ()), // sub_1408fc270
    Replacement::draft(0x1408fc380, "content_1408fc380", fn_1408fc380 as *const ()), // sub_1408fc380
    Replacement::draft(0x1408fc420, "content_1408fc420", fn_1408fc420 as *const ()), // sub_1408fc420
    Replacement::draft(0x1408fc4c0, "content_1408fc4c0", fn_1408fc4c0 as *const ()), // sub_1408fc4c0
    Replacement::draft(0x1408fc600, "content_1408fc600", fn_1408fc600 as *const ()), // sub_1408fc600
    Replacement::draft_throws(0x1408fc7c0, "content_1408fc7c0", fn_1408fc7c0 as *const (), Throws::Caught), // sub_1408fc7c0
    Replacement::draft(0x1408fc970, "content_1408fc970", fn_1408fc970 as *const ()), // sub_1408fc970
    Replacement::draft_throws(0x1408fcab0, "content_1408fcab0", fn_1408fcab0 as *const (), Throws::Caught), // sub_1408fcab0
    Replacement::draft(0x1408fcc60, "content_1408fcc60", fn_1408fcc60 as *const ()), // sub_1408fcc60
    Replacement::draft_throws(0x1408fcd70, "content_1408fcd70", fn_1408fcd70 as *const (), Throws::Caught), // sub_1408fcd70
    Replacement::draft_throws(0x1408fcf00, "content_1408fcf00", fn_1408fcf00 as *const (), Throws::Caught), // sub_1408fcf00
    Replacement::draft_throws(0x1408fd090, "content_1408fd090", fn_1408fd090 as *const (), Throws::Caught), // sub_1408fd090
    Replacement::draft(0x1408fd220, "content_1408fd220", fn_1408fd220 as *const ()), // sub_1408fd220
    Replacement::draft(0x1408fd3b0, "content_1408fd3b0", fn_1408fd3b0 as *const ()), // sub_1408fd3b0
    Replacement::draft(0x1408fd520, "content_1408fd520", fn_1408fd520 as *const ()), // sub_1408fd520
    Replacement::draft_throws(0x1408fd670, "content_1408fd670", fn_1408fd670 as *const (), Throws::Caught), // sub_1408fd670
    Replacement::draft_throws(0x1408fd7a0, "content_1408fd7a0", fn_1408fd7a0 as *const (), Throws::Caught), // sub_1408fd7a0
    Replacement::draft(0x1408fd930, "content_1408fd930", fn_1408fd930 as *const ()), // sub_1408fd930
    Replacement::draft(0x1408fda40, "content_1408fda40", fn_1408fda40 as *const ()), // sub_1408fda40
    Replacement::draft(0x1408fdae0, "content_1408fdae0", fn_1408fdae0 as *const ()), // sub_1408fdae0
    Replacement::draft(0x1408fdb80, "content_1408fdb80", fn_1408fdb80 as *const ()), // sub_1408fdb80
    Replacement::draft(0x1408fdcc0, "content_1408fdcc0", fn_1408fdcc0 as *const ()), // sub_1408fdcc0
    Replacement::draft_throws(0x1408fde80, "content_1408fde80", fn_1408fde80 as *const (), Throws::Caught), // sub_1408fde80
    Replacement::draft(0x1408fe030, "content_1408fe030", fn_1408fe030 as *const ()), // sub_1408fe030
    Replacement::draft_throws(0x1408fe170, "content_1408fe170", fn_1408fe170 as *const (), Throws::Caught), // sub_1408fe170
    Replacement::draft(0x1408fe320, "content_1408fe320", fn_1408fe320 as *const ()), // sub_1408fe320
    Replacement::draft_throws(0x1408fe430, "content_1408fe430", fn_1408fe430 as *const (), Throws::Caught), // sub_1408fe430
    Replacement::draft_throws(0x1408fe5c0, "content_1408fe5c0", fn_1408fe5c0 as *const (), Throws::Caught), // sub_1408fe5c0
    Replacement::draft_throws(0x1408fe750, "content_1408fe750", fn_1408fe750 as *const (), Throws::Caught), // sub_1408fe750
    Replacement::draft(0x1408fe8e0, "content_1408fe8e0", fn_1408fe8e0 as *const ()), // sub_1408fe8e0
    Replacement::draft(0x1408fea70, "content_1408fea70", fn_1408fea70 as *const ()), // sub_1408fea70
    Replacement::draft(0x1408febe0, "content_1408febe0", fn_1408febe0 as *const ()), // sub_1408febe0
    Replacement::draft_throws(0x1408fed30, "content_1408fed30", fn_1408fed30 as *const (), Throws::Caught), // sub_1408fed30
    Replacement::draft_throws(0x1408fee60, "content_1408fee60", fn_1408fee60 as *const (), Throws::Caught), // sub_1408fee60
    Replacement::draft(0x1408feff0, "content_1408feff0", fn_1408feff0 as *const ()), // sub_1408feff0
    Replacement::draft(0x1408ff100, "content_1408ff100", fn_1408ff100 as *const ()), // sub_1408ff100
    Replacement::draft(0x1408ff1a0, "content_1408ff1a0", fn_1408ff1a0 as *const ()), // sub_1408ff1a0
    Replacement::draft(0x1408ff240, "content_1408ff240", fn_1408ff240 as *const ()), // sub_1408ff240
    Replacement::draft(0x1408ff380, "content_1408ff380", fn_1408ff380 as *const ()), // sub_1408ff380
    Replacement::draft_throws(0x1408ff540, "content_1408ff540", fn_1408ff540 as *const (), Throws::Caught), // sub_1408ff540
    Replacement::draft(0x1408ff6f0, "content_1408ff6f0", fn_1408ff6f0 as *const ()), // sub_1408ff6f0
    Replacement::draft_throws(0x1408ff830, "content_1408ff830", fn_1408ff830 as *const (), Throws::Caught), // sub_1408ff830
    Replacement::draft(0x1408ff9e0, "content_1408ff9e0", fn_1408ff9e0 as *const ()), // sub_1408ff9e0
    Replacement::draft_throws(0x1408ffaf0, "content_1408ffaf0", fn_1408ffaf0 as *const (), Throws::Caught), // sub_1408ffaf0
    Replacement::draft_throws(0x1408ffc80, "content_1408ffc80", fn_1408ffc80 as *const (), Throws::Caught), // sub_1408ffc80
    Replacement::draft_throws(0x1408ffe10, "content_1408ffe10", fn_1408ffe10 as *const (), Throws::Caught), // sub_1408ffe10
    Replacement::draft(0x1408fffa0, "content_1408fffa0", fn_1408fffa0 as *const ()), // sub_1408fffa0
    Replacement::draft(0x140900130, "content_140900130", fn_140900130 as *const ()), // sub_140900130
    Replacement::draft(0x1409002a0, "content_1409002a0", fn_1409002a0 as *const ()), // sub_1409002a0
    Replacement::draft_throws(0x1409003f0, "content_1409003f0", fn_1409003f0 as *const (), Throws::Caught), // sub_1409003f0
    Replacement::draft_throws(0x140900520, "content_140900520", fn_140900520 as *const (), Throws::Caught), // sub_140900520
    Replacement::draft(0x1409006b0, "content_1409006b0", fn_1409006b0 as *const ()), // sub_1409006b0
    Replacement::draft(0x1409007c0, "content_1409007c0", fn_1409007c0 as *const ()), // sub_1409007c0
    Replacement::draft(0x140900860, "content_140900860", fn_140900860 as *const ()), // sub_140900860
    Replacement::draft(0x140900900, "content_140900900", fn_140900900 as *const ()), // sub_140900900
    Replacement::draft(0x140900a40, "content_140900a40", fn_140900a40 as *const ()), // sub_140900a40
    Replacement::draft_throws(0x140900c00, "content_140900c00", fn_140900c00 as *const (), Throws::Caught), // sub_140900c00
    Replacement::draft(0x140900db0, "content_140900db0", fn_140900db0 as *const ()), // sub_140900db0
    Replacement::draft_throws(0x140900ef0, "content_140900ef0", fn_140900ef0 as *const (), Throws::Caught), // sub_140900ef0
    Replacement::draft(0x1409010a0, "content_1409010a0", fn_1409010a0 as *const ()), // sub_1409010a0
    Replacement::draft_throws(0x1409011b0, "content_1409011b0", fn_1409011b0 as *const (), Throws::Caught), // sub_1409011b0
    Replacement::draft_throws(0x140901340, "content_140901340", fn_140901340 as *const (), Throws::Caught), // sub_140901340
    Replacement::draft_throws(0x1409014d0, "content_1409014d0", fn_1409014d0 as *const (), Throws::Caught), // sub_1409014d0
    Replacement::draft(0x140901660, "content_140901660", fn_140901660 as *const ()), // sub_140901660
    Replacement::draft(0x1409017f0, "content_1409017f0", fn_1409017f0 as *const ()), // sub_1409017f0
    Replacement::draft(0x140901960, "content_140901960", fn_140901960 as *const ()), // sub_140901960
    Replacement::draft_throws(0x140901ab0, "content_140901ab0", fn_140901ab0 as *const (), Throws::Caught), // sub_140901ab0
    Replacement::draft_throws(0x140901be0, "content_140901be0", fn_140901be0 as *const (), Throws::Caught), // sub_140901be0
    Replacement::draft(0x140901d70, "content_140901d70", fn_140901d70 as *const ()), // sub_140901d70
    Replacement::draft(0x140901e80, "content_140901e80", fn_140901e80 as *const ()), // sub_140901e80
    Replacement::draft(0x140901f20, "content_140901f20", fn_140901f20 as *const ()), // sub_140901f20
    Replacement::draft(0x140901fc0, "content_140901fc0", fn_140901fc0 as *const ()), // sub_140901fc0
    Replacement::draft(0x140902100, "content_140902100", fn_140902100 as *const ()), // sub_140902100
    Replacement::draft_throws(0x1409022c0, "content_1409022c0", fn_1409022c0 as *const (), Throws::Caught), // sub_1409022c0
    Replacement::draft(0x140902470, "content_140902470", fn_140902470 as *const ()), // sub_140902470
    Replacement::draft_throws(0x1409025b0, "content_1409025b0", fn_1409025b0 as *const (), Throws::Caught), // sub_1409025b0
    Replacement::draft(0x140902760, "content_140902760", fn_140902760 as *const ()), // sub_140902760
    Replacement::draft_throws(0x140902870, "content_140902870", fn_140902870 as *const (), Throws::Caught), // sub_140902870
    Replacement::draft_throws(0x140902a00, "content_140902a00", fn_140902a00 as *const (), Throws::Caught), // sub_140902a00
    Replacement::draft_throws(0x140902b90, "content_140902b90", fn_140902b90 as *const (), Throws::Caught), // sub_140902b90
    Replacement::draft(0x140902d20, "content_140902d20", fn_140902d20 as *const ()), // sub_140902d20
    Replacement::draft(0x140902eb0, "content_140902eb0", fn_140902eb0 as *const ()), // sub_140902eb0
    Replacement::draft(0x140903020, "content_140903020", fn_140903020 as *const ()), // sub_140903020
    Replacement::draft_throws(0x140903170, "content_140903170", fn_140903170 as *const (), Throws::Caught), // sub_140903170
    Replacement::draft_throws(0x1409032a0, "content_1409032a0", fn_1409032a0 as *const (), Throws::Caught), // sub_1409032a0
    Replacement::draft(0x140903430, "content_140903430", fn_140903430 as *const ()), // sub_140903430
    Replacement::draft(0x140903540, "content_140903540", fn_140903540 as *const ()), // sub_140903540
    Replacement::draft(0x1409035e0, "content_1409035e0", fn_1409035e0 as *const ()), // sub_1409035e0
    Replacement::draft(0x140903680, "content_140903680", fn_140903680 as *const ()), // sub_140903680
    Replacement::draft(0x1409037c0, "content_1409037c0", fn_1409037c0 as *const ()), // sub_1409037c0
    Replacement::draft_throws(0x140903980, "content_140903980", fn_140903980 as *const (), Throws::Caught), // sub_140903980
    Replacement::draft(0x140903b30, "content_140903b30", fn_140903b30 as *const ()), // sub_140903b30
    Replacement::draft_throws(0x140903c70, "content_140903c70", fn_140903c70 as *const (), Throws::Caught), // sub_140903c70
    Replacement::draft(0x140903e20, "content_140903e20", fn_140903e20 as *const ()), // sub_140903e20
    Replacement::draft_throws(0x140903f30, "content_140903f30", fn_140903f30 as *const (), Throws::Caught), // sub_140903f30
    Replacement::draft_throws(0x1409040c0, "content_1409040c0", fn_1409040c0 as *const (), Throws::Caught), // sub_1409040c0
    Replacement::draft_throws(0x140904250, "content_140904250", fn_140904250 as *const (), Throws::Caught), // sub_140904250
    Replacement::draft(0x1409043e0, "content_1409043e0", fn_1409043e0 as *const ()), // sub_1409043e0
    Replacement::draft(0x140904570, "content_140904570", fn_140904570 as *const ()), // sub_140904570
    Replacement::draft(0x1409046e0, "content_1409046e0", fn_1409046e0 as *const ()), // sub_1409046e0
    Replacement::draft_throws(0x140904830, "content_140904830", fn_140904830 as *const (), Throws::Caught), // sub_140904830
    Replacement::draft_throws(0x140904960, "content_140904960", fn_140904960 as *const (), Throws::Caught), // sub_140904960
    Replacement::draft(0x140904af0, "content_140904af0", fn_140904af0 as *const ()), // sub_140904af0
    Replacement::draft(0x140904c00, "content_140904c00", fn_140904c00 as *const ()), // sub_140904c00
    Replacement::draft(0x140904ca0, "content_140904ca0", fn_140904ca0 as *const ()), // sub_140904ca0
    Replacement::draft(0x140904d40, "content_140904d40", fn_140904d40 as *const ()), // sub_140904d40
    Replacement::draft(0x140904e80, "content_140904e80", fn_140904e80 as *const ()), // sub_140904e80
    Replacement::draft_throws(0x140905040, "content_140905040", fn_140905040 as *const (), Throws::Caught), // sub_140905040
    Replacement::draft(0x1409051f0, "content_1409051f0", fn_1409051f0 as *const ()), // sub_1409051f0
    Replacement::draft_throws(0x140905330, "content_140905330", fn_140905330 as *const (), Throws::Caught), // sub_140905330
    Replacement::draft(0x1409054e0, "content_1409054e0", fn_1409054e0 as *const ()), // sub_1409054e0
    Replacement::draft_throws(0x1409055f0, "content_1409055f0", fn_1409055f0 as *const (), Throws::Caught), // sub_1409055f0
    Replacement::draft_throws(0x140905780, "content_140905780", fn_140905780 as *const (), Throws::Caught), // sub_140905780
    Replacement::draft_throws(0x140905910, "content_140905910", fn_140905910 as *const (), Throws::Caught), // sub_140905910
    Replacement::draft(0x140905aa0, "content_140905aa0", fn_140905aa0 as *const ()), // sub_140905aa0
    Replacement::draft(0x140905c30, "content_140905c30", fn_140905c30 as *const ()), // sub_140905c30
    Replacement::draft(0x140905da0, "content_140905da0", fn_140905da0 as *const ()), // sub_140905da0
    Replacement::draft_throws(0x140905ef0, "content_140905ef0", fn_140905ef0 as *const (), Throws::Caught), // sub_140905ef0
    Replacement::draft_throws(0x140906020, "content_140906020", fn_140906020 as *const (), Throws::Caught), // sub_140906020
    Replacement::draft(0x1409061b0, "content_1409061b0", fn_1409061b0 as *const ()), // sub_1409061b0
    Replacement::draft(0x1409062c0, "content_1409062c0", fn_1409062c0 as *const ()), // sub_1409062c0
    Replacement::draft(0x140906360, "content_140906360", fn_140906360 as *const ()), // sub_140906360
    Replacement::draft(0x140906400, "content_140906400", fn_140906400 as *const ()), // sub_140906400
    Replacement::draft(0x140906540, "content_140906540", fn_140906540 as *const ()), // sub_140906540
    Replacement::draft_throws(0x140906700, "content_140906700", fn_140906700 as *const (), Throws::Caught), // sub_140906700
    Replacement::draft(0x1409068b0, "content_1409068b0", fn_1409068b0 as *const ()), // sub_1409068b0
    Replacement::draft_throws(0x1409069f0, "content_1409069f0", fn_1409069f0 as *const (), Throws::Caught), // sub_1409069f0
    Replacement::draft(0x140906ba0, "content_140906ba0", fn_140906ba0 as *const ()), // sub_140906ba0
    Replacement::draft_throws(0x140906cb0, "content_140906cb0", fn_140906cb0 as *const (), Throws::Caught), // sub_140906cb0
    Replacement::draft_throws(0x140906e40, "content_140906e40", fn_140906e40 as *const (), Throws::Caught), // sub_140906e40
    Replacement::draft_throws(0x140906fd0, "content_140906fd0", fn_140906fd0 as *const (), Throws::Caught), // sub_140906fd0
    Replacement::draft(0x140907160, "content_140907160", fn_140907160 as *const ()), // sub_140907160
    Replacement::draft(0x1409072f0, "content_1409072f0", fn_1409072f0 as *const ()), // sub_1409072f0
    Replacement::draft(0x140907460, "content_140907460", fn_140907460 as *const ()), // sub_140907460
    Replacement::draft_throws(0x1409075b0, "content_1409075b0", fn_1409075b0 as *const (), Throws::Caught), // sub_1409075b0
    Replacement::draft_throws(0x1409076e0, "content_1409076e0", fn_1409076e0 as *const (), Throws::Caught), // sub_1409076e0
    Replacement::draft(0x140907870, "content_140907870", fn_140907870 as *const ()), // sub_140907870
    Replacement::draft(0x140907980, "content_140907980", fn_140907980 as *const ()), // sub_140907980
    Replacement::draft(0x140907a20, "content_140907a20", fn_140907a20 as *const ()), // sub_140907a20
    Replacement::draft(0x140907ac0, "content_140907ac0", fn_140907ac0 as *const ()), // sub_140907ac0
    Replacement::draft(0x140907c00, "content_140907c00", fn_140907c00 as *const ()), // sub_140907c00
    Replacement::draft_throws(0x140907dc0, "content_140907dc0", fn_140907dc0 as *const (), Throws::Caught), // sub_140907dc0
    Replacement::draft(0x140907f70, "content_140907f70", fn_140907f70 as *const ()), // sub_140907f70
    Replacement::draft_throws(0x1409080b0, "content_1409080b0", fn_1409080b0 as *const (), Throws::Caught), // sub_1409080b0
    Replacement::draft(0x140908260, "content_140908260", fn_140908260 as *const ()), // sub_140908260
    Replacement::draft_throws(0x140908370, "content_140908370", fn_140908370 as *const (), Throws::Caught), // sub_140908370
    Replacement::draft_throws(0x140908500, "content_140908500", fn_140908500 as *const (), Throws::Caught), // sub_140908500
    Replacement::draft_throws(0x140908690, "content_140908690", fn_140908690 as *const (), Throws::Caught), // sub_140908690
    Replacement::draft(0x140908820, "content_140908820", fn_140908820 as *const ()), // sub_140908820
    Replacement::draft(0x1409089b0, "content_1409089b0", fn_1409089b0 as *const ()), // sub_1409089b0
    Replacement::draft(0x140908b20, "content_140908b20", fn_140908b20 as *const ()), // sub_140908b20
    Replacement::draft_throws(0x140908c70, "content_140908c70", fn_140908c70 as *const (), Throws::Caught), // sub_140908c70
    Replacement::draft_throws(0x140908da0, "content_140908da0", fn_140908da0 as *const (), Throws::Caught), // sub_140908da0
    Replacement::draft(0x140908f30, "content_140908f30", fn_140908f30 as *const ()), // sub_140908f30
    Replacement::draft(0x140909040, "content_140909040", fn_140909040 as *const ()), // sub_140909040
    Replacement::draft(0x1409090e0, "content_1409090e0", fn_1409090e0 as *const ()), // sub_1409090e0
    Replacement::draft(0x140909180, "content_140909180", fn_140909180 as *const ()), // sub_140909180
    Replacement::draft(0x1409092c0, "content_1409092c0", fn_1409092c0 as *const ()), // sub_1409092c0
    Replacement::draft_throws(0x140909480, "content_140909480", fn_140909480 as *const (), Throws::Caught), // sub_140909480
    Replacement::draft(0x140909630, "content_140909630", fn_140909630 as *const ()), // sub_140909630
    Replacement::draft_throws(0x140909770, "content_140909770", fn_140909770 as *const (), Throws::Caught), // sub_140909770
    Replacement::draft(0x140909920, "content_140909920", fn_140909920 as *const ()), // sub_140909920
    Replacement::draft_throws(0x140909a30, "content_140909a30", fn_140909a30 as *const (), Throws::Caught), // sub_140909a30
    Replacement::draft_throws(0x140909bc0, "content_140909bc0", fn_140909bc0 as *const (), Throws::Caught), // sub_140909bc0
    Replacement::draft_throws(0x140909d50, "content_140909d50", fn_140909d50 as *const (), Throws::Caught), // sub_140909d50
    Replacement::draft(0x140909ee0, "content_140909ee0", fn_140909ee0 as *const ()), // sub_140909ee0
    Replacement::draft(0x14090a070, "content_14090a070", fn_14090a070 as *const ()), // sub_14090a070
    Replacement::draft(0x14090a1e0, "content_14090a1e0", fn_14090a1e0 as *const ()), // sub_14090a1e0
    Replacement::draft_throws(0x14090a330, "content_14090a330", fn_14090a330 as *const (), Throws::Caught), // sub_14090a330
    Replacement::draft_throws(0x14090a460, "content_14090a460", fn_14090a460 as *const (), Throws::Caught), // sub_14090a460
    Replacement::draft(0x14090a5f0, "content_14090a5f0", fn_14090a5f0 as *const ()), // sub_14090a5f0
    Replacement::draft(0x14090a700, "content_14090a700", fn_14090a700 as *const ()), // sub_14090a700
    Replacement::draft(0x14090a7a0, "content_14090a7a0", fn_14090a7a0 as *const ()), // sub_14090a7a0
    Replacement::draft(0x14090a840, "content_14090a840", fn_14090a840 as *const ()), // sub_14090a840
    Replacement::draft(0x14090a980, "content_14090a980", fn_14090a980 as *const ()), // sub_14090a980
    Replacement::draft(0x14090aa90, "content_14090aa90", fn_14090aa90 as *const ()), // sub_14090aa90
    Replacement::draft(0x14090aba0, "content_14090aba0", fn_14090aba0 as *const ()), // sub_14090aba0
    Replacement::draft(0x14090ac40, "content_14090ac40", fn_14090ac40 as *const ()), // sub_14090ac40
    Replacement::draft(0x14090ad80, "content_14090ad80", fn_14090ad80 as *const ()), // sub_14090ad80
    Replacement::draft_throws(0x14090af40, "content_14090af40", fn_14090af40 as *const (), Throws::Caught), // sub_14090af40
    Replacement::draft(0x14090b0f0, "content_14090b0f0", fn_14090b0f0 as *const ()), // sub_14090b0f0
    Replacement::draft_throws(0x14090b230, "content_14090b230", fn_14090b230 as *const (), Throws::Caught), // sub_14090b230
    Replacement::draft(0x14090b3e0, "content_14090b3e0", fn_14090b3e0 as *const ()), // sub_14090b3e0
    Replacement::draft_throws(0x14090b4f0, "content_14090b4f0", fn_14090b4f0 as *const (), Throws::Caught), // sub_14090b4f0
    Replacement::draft_throws(0x14090b680, "content_14090b680", fn_14090b680 as *const (), Throws::Caught), // sub_14090b680
    Replacement::draft_throws(0x14090b810, "content_14090b810", fn_14090b810 as *const (), Throws::Caught), // sub_14090b810
    Replacement::draft(0x14090b9a0, "content_14090b9a0", fn_14090b9a0 as *const ()), // sub_14090b9a0
    Replacement::draft(0x14090bb30, "content_14090bb30", fn_14090bb30 as *const ()), // sub_14090bb30
    Replacement::draft(0x14090bca0, "content_14090bca0", fn_14090bca0 as *const ()), // sub_14090bca0
    Replacement::draft_throws(0x14090bdf0, "content_14090bdf0", fn_14090bdf0 as *const (), Throws::Caught), // sub_14090bdf0
    Replacement::draft_throws(0x14090bf20, "content_14090bf20", fn_14090bf20 as *const (), Throws::Caught), // sub_14090bf20
    Replacement::draft(0x14090c0b0, "content_14090c0b0", fn_14090c0b0 as *const ()), // sub_14090c0b0
    Replacement::draft(0x14090c1c0, "content_14090c1c0", fn_14090c1c0 as *const ()), // sub_14090c1c0
    Replacement::draft(0x14090c260, "content_14090c260", fn_14090c260 as *const ()), // sub_14090c260
    Replacement::draft(0x14090c300, "content_14090c300", fn_14090c300 as *const ()), // sub_14090c300
    Replacement::draft(0x14090c440, "content_14090c440", fn_14090c440 as *const ()), // sub_14090c440
    Replacement::draft_throws(0x14090c600, "content_14090c600", fn_14090c600 as *const (), Throws::Caught), // sub_14090c600
    Replacement::draft(0x14090c7b0, "content_14090c7b0", fn_14090c7b0 as *const ()), // sub_14090c7b0
    Replacement::draft_throws(0x14090c8f0, "content_14090c8f0", fn_14090c8f0 as *const (), Throws::Caught), // sub_14090c8f0
    Replacement::draft(0x14090caa0, "content_14090caa0", fn_14090caa0 as *const ()), // sub_14090caa0
    Replacement::draft_throws(0x14090cbb0, "content_14090cbb0", fn_14090cbb0 as *const (), Throws::Caught), // sub_14090cbb0
    Replacement::draft_throws(0x14090cd40, "content_14090cd40", fn_14090cd40 as *const (), Throws::Caught), // sub_14090cd40
    Replacement::draft_throws(0x14090ced0, "content_14090ced0", fn_14090ced0 as *const (), Throws::Caught), // sub_14090ced0
    Replacement::draft(0x14090d060, "content_14090d060", fn_14090d060 as *const ()), // sub_14090d060
    Replacement::draft(0x14090d1f0, "content_14090d1f0", fn_14090d1f0 as *const ()), // sub_14090d1f0
    Replacement::draft(0x14090d360, "content_14090d360", fn_14090d360 as *const ()), // sub_14090d360
    Replacement::draft_throws(0x14090d4b0, "content_14090d4b0", fn_14090d4b0 as *const (), Throws::Caught), // sub_14090d4b0
    Replacement::draft_throws(0x14090d5e0, "content_14090d5e0", fn_14090d5e0 as *const (), Throws::Caught), // sub_14090d5e0
    Replacement::draft(0x14090d770, "content_14090d770", fn_14090d770 as *const ()), // sub_14090d770
    Replacement::draft(0x14090d880, "content_14090d880", fn_14090d880 as *const ()), // sub_14090d880
    Replacement::draft(0x14090d920, "content_14090d920", fn_14090d920 as *const ()), // sub_14090d920
    Replacement::draft(0x14090d9c0, "content_14090d9c0", fn_14090d9c0 as *const ()), // sub_14090d9c0
    Replacement::draft(0x14090db00, "content_14090db00", fn_14090db00 as *const ()), // sub_14090db00
    Replacement::draft_throws(0x14090dcc0, "content_14090dcc0", fn_14090dcc0 as *const (), Throws::Caught), // sub_14090dcc0
    Replacement::draft(0x14090de70, "content_14090de70", fn_14090de70 as *const ()), // sub_14090de70
    Replacement::draft_throws(0x14090dfb0, "content_14090dfb0", fn_14090dfb0 as *const (), Throws::Caught), // sub_14090dfb0
    Replacement::draft(0x14090e160, "content_14090e160", fn_14090e160 as *const ()), // sub_14090e160
    Replacement::draft_throws(0x14090e270, "content_14090e270", fn_14090e270 as *const (), Throws::Caught), // sub_14090e270
    Replacement::draft_throws(0x14090e400, "content_14090e400", fn_14090e400 as *const (), Throws::Caught), // sub_14090e400
    Replacement::draft_throws(0x14090e590, "content_14090e590", fn_14090e590 as *const (), Throws::Caught), // sub_14090e590
    Replacement::draft(0x14090e720, "content_14090e720", fn_14090e720 as *const ()), // sub_14090e720
    Replacement::draft(0x14090e8b0, "content_14090e8b0", fn_14090e8b0 as *const ()), // sub_14090e8b0
    Replacement::draft(0x14090ea20, "content_14090ea20", fn_14090ea20 as *const ()), // sub_14090ea20
    Replacement::draft_throws(0x14090eb70, "content_14090eb70", fn_14090eb70 as *const (), Throws::Caught), // sub_14090eb70
    Replacement::draft_throws(0x14090eca0, "content_14090eca0", fn_14090eca0 as *const (), Throws::Caught), // sub_14090eca0
    Replacement::draft(0x14090ee30, "content_14090ee30", fn_14090ee30 as *const ()), // sub_14090ee30
    Replacement::draft(0x14090ef40, "content_14090ef40", fn_14090ef40 as *const ()), // sub_14090ef40
    Replacement::draft(0x14090efe0, "content_14090efe0", fn_14090efe0 as *const ()), // sub_14090efe0
    Replacement::draft(0x14090f080, "content_14090f080", fn_14090f080 as *const ()), // sub_14090f080
    Replacement::draft(0x14090f1c0, "content_14090f1c0", fn_14090f1c0 as *const ()), // sub_14090f1c0
    Replacement::draft_throws(0x14090f380, "content_14090f380", fn_14090f380 as *const (), Throws::Caught), // sub_14090f380
    Replacement::draft(0x14090f530, "content_14090f530", fn_14090f530 as *const ()), // sub_14090f530
    Replacement::draft_throws(0x14090f670, "content_14090f670", fn_14090f670 as *const (), Throws::Caught), // sub_14090f670
    Replacement::draft(0x14090f820, "content_14090f820", fn_14090f820 as *const ()), // sub_14090f820
    Replacement::draft_throws(0x14090f930, "content_14090f930", fn_14090f930 as *const (), Throws::Caught), // sub_14090f930
    Replacement::draft_throws(0x14090fac0, "content_14090fac0", fn_14090fac0 as *const (), Throws::Caught), // sub_14090fac0
    Replacement::draft_throws(0x14090fc50, "content_14090fc50", fn_14090fc50 as *const (), Throws::Caught), // sub_14090fc50
    Replacement::draft(0x14090fde0, "content_14090fde0", fn_14090fde0 as *const ()), // sub_14090fde0
    Replacement::draft(0x14090ff70, "content_14090ff70", fn_14090ff70 as *const ()), // sub_14090ff70
    Replacement::draft(0x1409100e0, "content_1409100e0", fn_1409100e0 as *const ()), // sub_1409100e0
    Replacement::draft_throws(0x140910230, "content_140910230", fn_140910230 as *const (), Throws::Caught), // sub_140910230
    Replacement::draft_throws(0x140910360, "content_140910360", fn_140910360 as *const (), Throws::Caught), // sub_140910360
    Replacement::draft(0x1409104f0, "content_1409104f0", fn_1409104f0 as *const ()), // sub_1409104f0
    Replacement::draft(0x140910600, "content_140910600", fn_140910600 as *const ()), // sub_140910600
    Replacement::draft(0x1409106a0, "content_1409106a0", fn_1409106a0 as *const ()), // sub_1409106a0
    Replacement::draft(0x140910740, "content_140910740", fn_140910740 as *const ()), // sub_140910740
    Replacement::draft(0x140910880, "content_140910880", fn_140910880 as *const ()), // sub_140910880
    Replacement::draft_throws(0x140910a40, "content_140910a40", fn_140910a40 as *const (), Throws::Caught), // sub_140910a40
    Replacement::draft(0x140910bf0, "content_140910bf0", fn_140910bf0 as *const ()), // sub_140910bf0
    Replacement::draft_throws(0x140910d30, "content_140910d30", fn_140910d30 as *const (), Throws::Caught), // sub_140910d30
    Replacement::draft(0x140910ee0, "content_140910ee0", fn_140910ee0 as *const ()), // sub_140910ee0
    Replacement::draft_throws(0x140910ff0, "content_140910ff0", fn_140910ff0 as *const (), Throws::Caught), // sub_140910ff0
    Replacement::draft_throws(0x140911180, "content_140911180", fn_140911180 as *const (), Throws::Caught), // sub_140911180
    Replacement::draft_throws(0x140911310, "content_140911310", fn_140911310 as *const (), Throws::Caught), // sub_140911310
    Replacement::draft(0x1409114a0, "content_1409114a0", fn_1409114a0 as *const ()), // sub_1409114a0
    Replacement::draft(0x140911630, "content_140911630", fn_140911630 as *const ()), // sub_140911630
    Replacement::draft(0x1409117a0, "content_1409117a0", fn_1409117a0 as *const ()), // sub_1409117a0
    Replacement::draft_throws(0x1409118f0, "content_1409118f0", fn_1409118f0 as *const (), Throws::Caught), // sub_1409118f0
    Replacement::draft_throws(0x140911a20, "content_140911a20", fn_140911a20 as *const (), Throws::Caught), // sub_140911a20
    Replacement::draft(0x140911bb0, "content_140911bb0", fn_140911bb0 as *const ()), // sub_140911bb0
    Replacement::draft(0x140911cc0, "content_140911cc0", fn_140911cc0 as *const ()), // sub_140911cc0
    Replacement::draft(0x140911d60, "content_140911d60", fn_140911d60 as *const ()), // sub_140911d60
    Replacement::draft(0x140911e00, "content_140911e00", fn_140911e00 as *const ()), // sub_140911e00
    Replacement::draft(0x140911f40, "content_140911f40", fn_140911f40 as *const ()), // sub_140911f40
    Replacement::draft_throws(0x140912100, "content_140912100", fn_140912100 as *const (), Throws::Caught), // sub_140912100
    Replacement::draft(0x1409122b0, "content_1409122b0", fn_1409122b0 as *const ()), // sub_1409122b0
    Replacement::draft_throws(0x1409123f0, "content_1409123f0", fn_1409123f0 as *const (), Throws::Caught), // sub_1409123f0
    Replacement::draft(0x1409125a0, "content_1409125a0", fn_1409125a0 as *const ()), // sub_1409125a0
    Replacement::draft_throws(0x1409126b0, "content_1409126b0", fn_1409126b0 as *const (), Throws::Caught), // sub_1409126b0
    Replacement::draft_throws(0x140912840, "content_140912840", fn_140912840 as *const (), Throws::Caught), // sub_140912840
    Replacement::draft_throws(0x1409129d0, "content_1409129d0", fn_1409129d0 as *const (), Throws::Caught), // sub_1409129d0
    Replacement::draft(0x140912b60, "content_140912b60", fn_140912b60 as *const ()), // sub_140912b60
    Replacement::draft(0x140912cf0, "content_140912cf0", fn_140912cf0 as *const ()), // sub_140912cf0
    Replacement::draft(0x140912e60, "content_140912e60", fn_140912e60 as *const ()), // sub_140912e60
    Replacement::draft_throws(0x140912fb0, "content_140912fb0", fn_140912fb0 as *const (), Throws::Caught), // sub_140912fb0
    Replacement::draft_throws(0x1409130e0, "content_1409130e0", fn_1409130e0 as *const (), Throws::Caught), // sub_1409130e0
    Replacement::draft(0x140913270, "content_140913270", fn_140913270 as *const ()), // sub_140913270
    Replacement::draft(0x140913380, "content_140913380", fn_140913380 as *const ()), // sub_140913380
    Replacement::draft(0x140913420, "content_140913420", fn_140913420 as *const ()), // sub_140913420
    Replacement::draft(0x1409134c0, "content_1409134c0", fn_1409134c0 as *const ()), // sub_1409134c0
    Replacement::draft(0x140913600, "content_140913600", fn_140913600 as *const ()), // sub_140913600
    Replacement::draft(0x140913710, "content_140913710", fn_140913710 as *const ()), // sub_140913710
    Replacement::draft(0x140913820, "content_140913820", fn_140913820 as *const ()), // sub_140913820
    Replacement::draft(0x1409138c0, "content_1409138c0", fn_1409138c0 as *const ()), // sub_1409138c0
    Replacement::draft(0x140913a00, "content_140913a00", fn_140913a00 as *const ()), // sub_140913a00
    Replacement::draft(0x140913b10, "content_140913b10", fn_140913b10 as *const ()), // sub_140913b10
    Replacement::draft(0x140913c20, "content_140913c20", fn_140913c20 as *const ()), // sub_140913c20
    Replacement::draft(0x140913cc0, "content_140913cc0", fn_140913cc0 as *const ()), // sub_140913cc0
    Replacement::draft(0x140913e00, "content_140913e00", fn_140913e00 as *const ()), // sub_140913e00
    Replacement::draft_throws(0x140913fc0, "content_140913fc0", fn_140913fc0 as *const (), Throws::Caught), // sub_140913fc0
    Replacement::draft(0x140914170, "content_140914170", fn_140914170 as *const ()), // sub_140914170
    Replacement::draft_throws(0x1409142b0, "content_1409142b0", fn_1409142b0 as *const (), Throws::Caught), // sub_1409142b0
    Replacement::draft(0x140914460, "content_140914460", fn_140914460 as *const ()), // sub_140914460
    Replacement::draft_throws(0x140914570, "content_140914570", fn_140914570 as *const (), Throws::Caught), // sub_140914570
    Replacement::draft_throws(0x140914700, "content_140914700", fn_140914700 as *const (), Throws::Caught), // sub_140914700
    Replacement::draft_throws(0x140914890, "content_140914890", fn_140914890 as *const (), Throws::Caught), // sub_140914890
    Replacement::draft(0x140914a20, "content_140914a20", fn_140914a20 as *const ()), // sub_140914a20
    Replacement::draft(0x140914bb0, "content_140914bb0", fn_140914bb0 as *const ()), // sub_140914bb0
    Replacement::draft(0x140914d20, "content_140914d20", fn_140914d20 as *const ()), // sub_140914d20
    Replacement::draft_throws(0x140914e70, "content_140914e70", fn_140914e70 as *const (), Throws::Caught), // sub_140914e70
    Replacement::draft_throws(0x140914fa0, "content_140914fa0", fn_140914fa0 as *const (), Throws::Caught), // sub_140914fa0
    Replacement::draft(0x140915130, "content_140915130", fn_140915130 as *const ()), // sub_140915130
    Replacement::draft(0x140915240, "content_140915240", fn_140915240 as *const ()), // sub_140915240
    Replacement::draft(0x1409152e0, "content_1409152e0", fn_1409152e0 as *const ()), // sub_1409152e0
    Replacement::draft(0x140915380, "content_140915380", fn_140915380 as *const ()), // sub_140915380
    Replacement::draft(0x1409154c0, "content_1409154c0", fn_1409154c0 as *const ()), // sub_1409154c0
    Replacement::draft_throws(0x140915680, "content_140915680", fn_140915680 as *const (), Throws::Caught), // sub_140915680
    Replacement::draft(0x140915830, "content_140915830", fn_140915830 as *const ()), // sub_140915830
    Replacement::draft_throws(0x140915970, "content_140915970", fn_140915970 as *const (), Throws::Caught), // sub_140915970
    Replacement::draft(0x140915b20, "content_140915b20", fn_140915b20 as *const ()), // sub_140915b20
    Replacement::draft_throws(0x140915c30, "content_140915c30", fn_140915c30 as *const (), Throws::Caught), // sub_140915c30
    Replacement::draft_throws(0x140915dc0, "content_140915dc0", fn_140915dc0 as *const (), Throws::Caught), // sub_140915dc0
    Replacement::draft_throws(0x140915f50, "content_140915f50", fn_140915f50 as *const (), Throws::Caught), // sub_140915f50
    Replacement::draft(0x1409160e0, "content_1409160e0", fn_1409160e0 as *const ()), // sub_1409160e0
    Replacement::draft(0x140916270, "content_140916270", fn_140916270 as *const ()), // sub_140916270
    Replacement::draft(0x1409163e0, "content_1409163e0", fn_1409163e0 as *const ()), // sub_1409163e0
    Replacement::draft_throws(0x140916530, "content_140916530", fn_140916530 as *const (), Throws::Caught), // sub_140916530
    Replacement::draft_throws(0x140916660, "content_140916660", fn_140916660 as *const (), Throws::Caught), // sub_140916660
    Replacement::draft(0x1409167f0, "content_1409167f0", fn_1409167f0 as *const ()), // sub_1409167f0
    Replacement::draft(0x140916900, "content_140916900", fn_140916900 as *const ()), // sub_140916900
    Replacement::draft(0x1409169a0, "content_1409169a0", fn_1409169a0 as *const ()), // sub_1409169a0
    Replacement::draft(0x140916a40, "content_140916a40", fn_140916a40 as *const ()), // sub_140916a40
    Replacement::draft(0x140916b80, "content_140916b80", fn_140916b80 as *const ()), // sub_140916b80
    Replacement::draft_throws(0x140916d40, "content_140916d40", fn_140916d40 as *const (), Throws::Caught), // sub_140916d40
    Replacement::draft(0x140916ef0, "content_140916ef0", fn_140916ef0 as *const ()), // sub_140916ef0
    Replacement::draft_throws(0x140917030, "content_140917030", fn_140917030 as *const (), Throws::Caught), // sub_140917030
    Replacement::draft(0x1409171e0, "content_1409171e0", fn_1409171e0 as *const ()), // sub_1409171e0
    Replacement::draft_throws(0x1409172f0, "content_1409172f0", fn_1409172f0 as *const (), Throws::Caught), // sub_1409172f0
    Replacement::draft_throws(0x140917480, "content_140917480", fn_140917480 as *const (), Throws::Caught), // sub_140917480
    Replacement::draft_throws(0x140917610, "content_140917610", fn_140917610 as *const (), Throws::Caught), // sub_140917610
    Replacement::draft(0x1409177a0, "content_1409177a0", fn_1409177a0 as *const ()), // sub_1409177a0
    Replacement::draft(0x140917930, "content_140917930", fn_140917930 as *const ()), // sub_140917930
    Replacement::draft(0x140917aa0, "content_140917aa0", fn_140917aa0 as *const ()), // sub_140917aa0
    Replacement::draft_throws(0x140917bf0, "content_140917bf0", fn_140917bf0 as *const (), Throws::Caught), // sub_140917bf0
    Replacement::draft_throws(0x140917d20, "content_140917d20", fn_140917d20 as *const (), Throws::Caught), // sub_140917d20
    Replacement::draft(0x140917eb0, "content_140917eb0", fn_140917eb0 as *const ()), // sub_140917eb0
    Replacement::draft(0x140917fc0, "content_140917fc0", fn_140917fc0 as *const ()), // sub_140917fc0
    Replacement::draft(0x140918060, "content_140918060", fn_140918060 as *const ()), // sub_140918060
    Replacement::draft(0x140918100, "content_140918100", fn_140918100 as *const ()), // sub_140918100
    Replacement::draft(0x140918240, "content_140918240", fn_140918240 as *const ()), // sub_140918240
    Replacement::draft_throws(0x140918400, "content_140918400", fn_140918400 as *const (), Throws::Caught), // sub_140918400
    Replacement::draft(0x1409185b0, "content_1409185b0", fn_1409185b0 as *const ()), // sub_1409185b0
    Replacement::draft_throws(0x1409186f0, "content_1409186f0", fn_1409186f0 as *const (), Throws::Caught), // sub_1409186f0
    Replacement::draft(0x1409188a0, "content_1409188a0", fn_1409188a0 as *const ()), // sub_1409188a0
    Replacement::draft_throws(0x1409189b0, "content_1409189b0", fn_1409189b0 as *const (), Throws::Caught), // sub_1409189b0
    Replacement::draft_throws(0x140918b40, "content_140918b40", fn_140918b40 as *const (), Throws::Caught), // sub_140918b40
    Replacement::draft_throws(0x140918cd0, "content_140918cd0", fn_140918cd0 as *const (), Throws::Caught), // sub_140918cd0
    Replacement::draft(0x140918e60, "content_140918e60", fn_140918e60 as *const ()), // sub_140918e60
    Replacement::draft(0x140918ff0, "content_140918ff0", fn_140918ff0 as *const ()), // sub_140918ff0
    Replacement::draft(0x140919160, "content_140919160", fn_140919160 as *const ()), // sub_140919160
    Replacement::draft_throws(0x1409192b0, "content_1409192b0", fn_1409192b0 as *const (), Throws::Caught), // sub_1409192b0
    Replacement::draft_throws(0x1409193e0, "content_1409193e0", fn_1409193e0 as *const (), Throws::Caught), // sub_1409193e0
    Replacement::draft(0x140919570, "content_140919570", fn_140919570 as *const ()), // sub_140919570
    Replacement::draft(0x140919680, "content_140919680", fn_140919680 as *const ()), // sub_140919680
    Replacement::draft(0x140919720, "content_140919720", fn_140919720 as *const ()), // sub_140919720
    Replacement::draft(0x1409197c0, "content_1409197c0", fn_1409197c0 as *const ()), // sub_1409197c0
    Replacement::draft(0x140919900, "content_140919900", fn_140919900 as *const ()), // sub_140919900
    Replacement::draft_throws(0x140919ac0, "content_140919ac0", fn_140919ac0 as *const (), Throws::Caught), // sub_140919ac0
    Replacement::draft(0x140919c70, "content_140919c70", fn_140919c70 as *const ()), // sub_140919c70
    Replacement::draft_throws(0x140919db0, "content_140919db0", fn_140919db0 as *const (), Throws::Caught), // sub_140919db0
    Replacement::draft(0x140919f60, "content_140919f60", fn_140919f60 as *const ()), // sub_140919f60
    Replacement::draft_throws(0x14091a070, "content_14091a070", fn_14091a070 as *const (), Throws::Caught), // sub_14091a070
    Replacement::draft_throws(0x14091a200, "content_14091a200", fn_14091a200 as *const (), Throws::Caught), // sub_14091a200
    Replacement::draft_throws(0x14091a390, "content_14091a390", fn_14091a390 as *const (), Throws::Caught), // sub_14091a390
    Replacement::draft(0x14091a520, "content_14091a520", fn_14091a520 as *const ()), // sub_14091a520
    Replacement::draft(0x14091a6b0, "content_14091a6b0", fn_14091a6b0 as *const ()), // sub_14091a6b0
    Replacement::draft(0x14091a820, "content_14091a820", fn_14091a820 as *const ()), // sub_14091a820
    Replacement::draft_throws(0x14091a970, "content_14091a970", fn_14091a970 as *const (), Throws::Caught), // sub_14091a970
    Replacement::draft_throws(0x14091aaa0, "content_14091aaa0", fn_14091aaa0 as *const (), Throws::Caught), // sub_14091aaa0
    Replacement::draft(0x14091ac30, "content_14091ac30", fn_14091ac30 as *const ()), // sub_14091ac30
    Replacement::draft(0x14091ad40, "content_14091ad40", fn_14091ad40 as *const ()), // sub_14091ad40
    Replacement::draft(0x14091ade0, "content_14091ade0", fn_14091ade0 as *const ()), // sub_14091ade0
    Replacement::draft(0x14091ae80, "content_14091ae80", fn_14091ae80 as *const ()), // sub_14091ae80
    Replacement::draft(0x14091afc0, "content_14091afc0", fn_14091afc0 as *const ()), // sub_14091afc0
    Replacement::draft_throws(0x14091b180, "content_14091b180", fn_14091b180 as *const (), Throws::Caught), // sub_14091b180
    Replacement::draft(0x14091b330, "content_14091b330", fn_14091b330 as *const ()), // sub_14091b330
    Replacement::draft_throws(0x14091b470, "content_14091b470", fn_14091b470 as *const (), Throws::Caught), // sub_14091b470
    Replacement::draft(0x14091b620, "content_14091b620", fn_14091b620 as *const ()), // sub_14091b620
    Replacement::draft_throws(0x14091b730, "content_14091b730", fn_14091b730 as *const (), Throws::Caught), // sub_14091b730
    Replacement::draft_throws(0x14091b8c0, "content_14091b8c0", fn_14091b8c0 as *const (), Throws::Caught), // sub_14091b8c0
    Replacement::draft_throws(0x14091ba50, "content_14091ba50", fn_14091ba50 as *const (), Throws::Caught), // sub_14091ba50
    Replacement::draft(0x14091bbe0, "content_14091bbe0", fn_14091bbe0 as *const ()), // sub_14091bbe0
    Replacement::draft(0x14091bd70, "content_14091bd70", fn_14091bd70 as *const ()), // sub_14091bd70
    Replacement::draft(0x14091bee0, "content_14091bee0", fn_14091bee0 as *const ()), // sub_14091bee0
    Replacement::draft_throws(0x14091c030, "content_14091c030", fn_14091c030 as *const (), Throws::Caught), // sub_14091c030
    Replacement::draft_throws(0x14091c160, "content_14091c160", fn_14091c160 as *const (), Throws::Caught), // sub_14091c160
    Replacement::draft(0x14091c2f0, "content_14091c2f0", fn_14091c2f0 as *const ()), // sub_14091c2f0
    Replacement::draft(0x14091c400, "content_14091c400", fn_14091c400 as *const ()), // sub_14091c400
    Replacement::draft(0x14091c4a0, "content_14091c4a0", fn_14091c4a0 as *const ()), // sub_14091c4a0
    Replacement::draft(0x14091c540, "content_14091c540", fn_14091c540 as *const ()), // sub_14091c540
    Replacement::draft(0x14091c680, "content_14091c680", fn_14091c680 as *const ()), // sub_14091c680
    Replacement::draft_throws(0x14091c840, "content_14091c840", fn_14091c840 as *const (), Throws::Caught), // sub_14091c840
    Replacement::draft(0x14091c9f0, "content_14091c9f0", fn_14091c9f0 as *const ()), // sub_14091c9f0
    Replacement::draft_throws(0x14091cb30, "content_14091cb30", fn_14091cb30 as *const (), Throws::Caught), // sub_14091cb30
    Replacement::draft(0x14091cce0, "content_14091cce0", fn_14091cce0 as *const ()), // sub_14091cce0
    Replacement::draft_throws(0x14091cdf0, "content_14091cdf0", fn_14091cdf0 as *const (), Throws::Caught), // sub_14091cdf0
    Replacement::draft_throws(0x14091cf80, "content_14091cf80", fn_14091cf80 as *const (), Throws::Caught), // sub_14091cf80
    Replacement::draft_throws(0x14091d110, "content_14091d110", fn_14091d110 as *const (), Throws::Caught), // sub_14091d110
    Replacement::draft(0x14091d2a0, "content_14091d2a0", fn_14091d2a0 as *const ()), // sub_14091d2a0
    Replacement::draft(0x14091d430, "content_14091d430", fn_14091d430 as *const ()), // sub_14091d430
    Replacement::draft(0x14091d5a0, "content_14091d5a0", fn_14091d5a0 as *const ()), // sub_14091d5a0
    Replacement::draft_throws(0x14091d6f0, "content_14091d6f0", fn_14091d6f0 as *const (), Throws::Caught), // sub_14091d6f0
    Replacement::draft_throws(0x14091d820, "content_14091d820", fn_14091d820 as *const (), Throws::Caught), // sub_14091d820
    Replacement::draft(0x14091d9b0, "content_14091d9b0", fn_14091d9b0 as *const ()), // sub_14091d9b0
    Replacement::draft(0x14091dac0, "content_14091dac0", fn_14091dac0 as *const ()), // sub_14091dac0
    Replacement::draft(0x14091db60, "content_14091db60", fn_14091db60 as *const ()), // sub_14091db60
    Replacement::draft(0x14091dc00, "content_14091dc00", fn_14091dc00 as *const ()), // sub_14091dc00
    Replacement::draft(0x14091dd40, "content_14091dd40", fn_14091dd40 as *const ()), // sub_14091dd40
    Replacement::draft_throws(0x14091df00, "content_14091df00", fn_14091df00 as *const (), Throws::Caught), // sub_14091df00
    Replacement::draft(0x14091e0b0, "content_14091e0b0", fn_14091e0b0 as *const ()), // sub_14091e0b0
    Replacement::draft_throws(0x14091e1f0, "content_14091e1f0", fn_14091e1f0 as *const (), Throws::Caught), // sub_14091e1f0
    Replacement::draft(0x14091e3a0, "content_14091e3a0", fn_14091e3a0 as *const ()), // sub_14091e3a0
    Replacement::draft_throws(0x14091e4b0, "content_14091e4b0", fn_14091e4b0 as *const (), Throws::Caught), // sub_14091e4b0
    Replacement::draft_throws(0x14091e640, "content_14091e640", fn_14091e640 as *const (), Throws::Caught), // sub_14091e640
    Replacement::draft_throws(0x14091e7d0, "content_14091e7d0", fn_14091e7d0 as *const (), Throws::Caught), // sub_14091e7d0
    Replacement::draft(0x14091e960, "content_14091e960", fn_14091e960 as *const ()), // sub_14091e960
    Replacement::draft(0x14091eaf0, "content_14091eaf0", fn_14091eaf0 as *const ()), // sub_14091eaf0
    Replacement::draft(0x14091ec60, "content_14091ec60", fn_14091ec60 as *const ()), // sub_14091ec60
    Replacement::draft_throws(0x14091edb0, "content_14091edb0", fn_14091edb0 as *const (), Throws::Caught), // sub_14091edb0
    Replacement::draft_throws(0x14091eee0, "content_14091eee0", fn_14091eee0 as *const (), Throws::Caught), // sub_14091eee0
    Replacement::draft(0x14091f070, "content_14091f070", fn_14091f070 as *const ()), // sub_14091f070
    Replacement::draft(0x14091f180, "content_14091f180", fn_14091f180 as *const ()), // sub_14091f180
    Replacement::draft(0x14091f220, "content_14091f220", fn_14091f220 as *const ()), // sub_14091f220
    Replacement::draft(0x14091f2c0, "content_14091f2c0", fn_14091f2c0 as *const ()), // sub_14091f2c0
    Replacement::draft(0x14091f400, "content_14091f400", fn_14091f400 as *const ()), // sub_14091f400
    Replacement::draft_throws(0x14091f5c0, "content_14091f5c0", fn_14091f5c0 as *const (), Throws::Caught), // sub_14091f5c0
    Replacement::draft(0x14091f770, "content_14091f770", fn_14091f770 as *const ()), // sub_14091f770
    Replacement::draft_throws(0x14091f8b0, "content_14091f8b0", fn_14091f8b0 as *const (), Throws::Caught), // sub_14091f8b0
    Replacement::draft(0x14091fa60, "content_14091fa60", fn_14091fa60 as *const ()), // sub_14091fa60
    Replacement::draft_throws(0x14091fb70, "content_14091fb70", fn_14091fb70 as *const (), Throws::Caught), // sub_14091fb70
    Replacement::draft_throws(0x14091fd00, "content_14091fd00", fn_14091fd00 as *const (), Throws::Caught), // sub_14091fd00
    Replacement::draft_throws(0x14091fe90, "content_14091fe90", fn_14091fe90 as *const (), Throws::Caught), // sub_14091fe90
    Replacement::draft(0x140920020, "content_140920020", fn_140920020 as *const ()), // sub_140920020
    Replacement::draft(0x1409201b0, "content_1409201b0", fn_1409201b0 as *const ()), // sub_1409201b0
    Replacement::draft(0x140920320, "content_140920320", fn_140920320 as *const ()), // sub_140920320
    Replacement::draft_throws(0x140920470, "content_140920470", fn_140920470 as *const (), Throws::Caught), // sub_140920470
    Replacement::draft_throws(0x1409205a0, "content_1409205a0", fn_1409205a0 as *const (), Throws::Caught), // sub_1409205a0
    Replacement::draft(0x140920730, "content_140920730", fn_140920730 as *const ()), // sub_140920730
    Replacement::draft(0x140920840, "content_140920840", fn_140920840 as *const ()), // sub_140920840
    Replacement::draft(0x1409208e0, "content_1409208e0", fn_1409208e0 as *const ()), // sub_1409208e0
    Replacement::draft(0x140920980, "content_140920980", fn_140920980 as *const ()), // sub_140920980
    Replacement::draft(0x140920ac0, "content_140920ac0", fn_140920ac0 as *const ()), // sub_140920ac0
    Replacement::draft_throws(0x140920c80, "content_140920c80", fn_140920c80 as *const (), Throws::Caught), // sub_140920c80
    Replacement::draft(0x140920e30, "content_140920e30", fn_140920e30 as *const ()), // sub_140920e30
    Replacement::draft_throws(0x140920f70, "content_140920f70", fn_140920f70 as *const (), Throws::Caught), // sub_140920f70
    Replacement::draft(0x140921120, "content_140921120", fn_140921120 as *const ()), // sub_140921120
    Replacement::draft_throws(0x140921230, "content_140921230", fn_140921230 as *const (), Throws::Caught), // sub_140921230
    Replacement::draft_throws(0x1409213c0, "content_1409213c0", fn_1409213c0 as *const (), Throws::Caught), // sub_1409213c0
    Replacement::draft_throws(0x140921550, "content_140921550", fn_140921550 as *const (), Throws::Caught), // sub_140921550
    Replacement::draft(0x1409216e0, "content_1409216e0", fn_1409216e0 as *const ()), // sub_1409216e0
    Replacement::draft(0x140921870, "content_140921870", fn_140921870 as *const ()), // sub_140921870
    Replacement::draft(0x1409219e0, "content_1409219e0", fn_1409219e0 as *const ()), // sub_1409219e0
    Replacement::draft_throws(0x140921b30, "content_140921b30", fn_140921b30 as *const (), Throws::Caught), // sub_140921b30
    Replacement::draft_throws(0x140921c60, "content_140921c60", fn_140921c60 as *const (), Throws::Caught), // sub_140921c60
    Replacement::draft(0x140921df0, "content_140921df0", fn_140921df0 as *const ()), // sub_140921df0
    Replacement::draft(0x140921f00, "content_140921f00", fn_140921f00 as *const ()), // sub_140921f00
    Replacement::draft(0x140921fa0, "content_140921fa0", fn_140921fa0 as *const ()), // sub_140921fa0
    Replacement::draft(0x140922040, "content_140922040", fn_140922040 as *const ()), // sub_140922040
    Replacement::draft(0x140922180, "content_140922180", fn_140922180 as *const ()), // sub_140922180
    Replacement::draft_throws(0x140922340, "content_140922340", fn_140922340 as *const (), Throws::Caught), // sub_140922340
    Replacement::draft(0x1409224f0, "content_1409224f0", fn_1409224f0 as *const ()), // sub_1409224f0
    Replacement::draft_throws(0x140922630, "content_140922630", fn_140922630 as *const (), Throws::Caught), // sub_140922630
    Replacement::draft(0x1409227e0, "content_1409227e0", fn_1409227e0 as *const ()), // sub_1409227e0
    Replacement::draft_throws(0x1409228f0, "content_1409228f0", fn_1409228f0 as *const (), Throws::Caught), // sub_1409228f0
    Replacement::draft_throws(0x140922a80, "content_140922a80", fn_140922a80 as *const (), Throws::Caught), // sub_140922a80
    Replacement::draft_throws(0x140922c10, "content_140922c10", fn_140922c10 as *const (), Throws::Caught), // sub_140922c10
    Replacement::draft(0x140922da0, "content_140922da0", fn_140922da0 as *const ()), // sub_140922da0
    Replacement::draft(0x140922f30, "content_140922f30", fn_140922f30 as *const ()), // sub_140922f30
    Replacement::draft(0x1409230a0, "content_1409230a0", fn_1409230a0 as *const ()), // sub_1409230a0
    Replacement::draft_throws(0x1409231f0, "content_1409231f0", fn_1409231f0 as *const (), Throws::Caught), // sub_1409231f0
    Replacement::draft_throws(0x140923320, "content_140923320", fn_140923320 as *const (), Throws::Caught), // sub_140923320
    Replacement::draft(0x1409234b0, "content_1409234b0", fn_1409234b0 as *const ()), // sub_1409234b0
    Replacement::draft(0x1409235c0, "content_1409235c0", fn_1409235c0 as *const ()), // sub_1409235c0
    Replacement::draft(0x140923660, "content_140923660", fn_140923660 as *const ()), // sub_140923660
    Replacement::draft(0x140923700, "content_140923700", fn_140923700 as *const ()), // sub_140923700
    Replacement::draft(0x140923840, "content_140923840", fn_140923840 as *const ()), // sub_140923840
    Replacement::draft_throws(0x140923a00, "content_140923a00", fn_140923a00 as *const (), Throws::Caught), // sub_140923a00
    Replacement::draft(0x140923bb0, "content_140923bb0", fn_140923bb0 as *const ()), // sub_140923bb0
    Replacement::draft_throws(0x140923cf0, "content_140923cf0", fn_140923cf0 as *const (), Throws::Caught), // sub_140923cf0
    Replacement::draft(0x140923ea0, "content_140923ea0", fn_140923ea0 as *const ()), // sub_140923ea0
    Replacement::draft_throws(0x140923fb0, "content_140923fb0", fn_140923fb0 as *const (), Throws::Caught), // sub_140923fb0
    Replacement::draft_throws(0x140924140, "content_140924140", fn_140924140 as *const (), Throws::Caught), // sub_140924140
    Replacement::draft_throws(0x1409242d0, "content_1409242d0", fn_1409242d0 as *const (), Throws::Caught), // sub_1409242d0
    Replacement::draft(0x140924460, "content_140924460", fn_140924460 as *const ()), // sub_140924460
    Replacement::draft(0x1409245f0, "content_1409245f0", fn_1409245f0 as *const ()), // sub_1409245f0
    Replacement::draft(0x140924760, "content_140924760", fn_140924760 as *const ()), // sub_140924760
    Replacement::draft_throws(0x1409248b0, "content_1409248b0", fn_1409248b0 as *const (), Throws::Caught), // sub_1409248b0
    Replacement::draft_throws(0x1409249e0, "content_1409249e0", fn_1409249e0 as *const (), Throws::Caught), // sub_1409249e0
    Replacement::draft(0x140924b70, "content_140924b70", fn_140924b70 as *const ()), // sub_140924b70
    Replacement::draft(0x140924c80, "content_140924c80", fn_140924c80 as *const ()), // sub_140924c80
    Replacement::draft(0x140924d20, "content_140924d20", fn_140924d20 as *const ()), // sub_140924d20
    Replacement::draft(0x140924dc0, "content_140924dc0", fn_140924dc0 as *const ()), // sub_140924dc0
    Replacement::draft(0x140924f00, "content_140924f00", fn_140924f00 as *const ()), // sub_140924f00
    Replacement::draft_throws(0x1409250c0, "content_1409250c0", fn_1409250c0 as *const (), Throws::Caught), // sub_1409250c0
    Replacement::draft(0x140925270, "content_140925270", fn_140925270 as *const ()), // sub_140925270
    Replacement::draft_throws(0x1409253b0, "content_1409253b0", fn_1409253b0 as *const (), Throws::Caught), // sub_1409253b0
    Replacement::draft(0x140925560, "content_140925560", fn_140925560 as *const ()), // sub_140925560
    Replacement::draft_throws(0x140925670, "content_140925670", fn_140925670 as *const (), Throws::Caught), // sub_140925670
    Replacement::draft_throws(0x140925800, "content_140925800", fn_140925800 as *const (), Throws::Caught), // sub_140925800
    Replacement::draft_throws(0x140925990, "content_140925990", fn_140925990 as *const (), Throws::Caught), // sub_140925990
    Replacement::draft(0x140925b20, "content_140925b20", fn_140925b20 as *const ()), // sub_140925b20
    Replacement::draft(0x140925cb0, "content_140925cb0", fn_140925cb0 as *const ()), // sub_140925cb0
    Replacement::draft(0x140925e20, "content_140925e20", fn_140925e20 as *const ()), // sub_140925e20
    Replacement::draft_throws(0x140925f70, "content_140925f70", fn_140925f70 as *const (), Throws::Caught), // sub_140925f70
    Replacement::draft_throws(0x1409260a0, "content_1409260a0", fn_1409260a0 as *const (), Throws::Caught), // sub_1409260a0
    Replacement::draft(0x140926230, "content_140926230", fn_140926230 as *const ()), // sub_140926230
    Replacement::draft(0x140926340, "content_140926340", fn_140926340 as *const ()), // sub_140926340
    Replacement::draft(0x1409263e0, "content_1409263e0", fn_1409263e0 as *const ()), // sub_1409263e0
    Replacement::draft(0x140926480, "content_140926480", fn_140926480 as *const ()), // sub_140926480
    Replacement::draft(0x1409265c0, "content_1409265c0", fn_1409265c0 as *const ()), // sub_1409265c0
    Replacement::draft_throws(0x140926780, "content_140926780", fn_140926780 as *const (), Throws::Caught), // sub_140926780
    Replacement::draft(0x140926930, "content_140926930", fn_140926930 as *const ()), // sub_140926930
    Replacement::draft_throws(0x140926a70, "content_140926a70", fn_140926a70 as *const (), Throws::Caught), // sub_140926a70
    Replacement::draft(0x140926c20, "content_140926c20", fn_140926c20 as *const ()), // sub_140926c20
    Replacement::draft_throws(0x140926d30, "content_140926d30", fn_140926d30 as *const (), Throws::Caught), // sub_140926d30
    Replacement::draft_throws(0x140926ec0, "content_140926ec0", fn_140926ec0 as *const (), Throws::Caught), // sub_140926ec0
    Replacement::draft_throws(0x140927050, "content_140927050", fn_140927050 as *const (), Throws::Caught), // sub_140927050
    Replacement::draft(0x1409271e0, "content_1409271e0", fn_1409271e0 as *const ()), // sub_1409271e0
    Replacement::draft(0x140927370, "content_140927370", fn_140927370 as *const ()), // sub_140927370
    Replacement::draft(0x1409274e0, "content_1409274e0", fn_1409274e0 as *const ()), // sub_1409274e0
    Replacement::draft_throws(0x140927630, "content_140927630", fn_140927630 as *const (), Throws::Caught), // sub_140927630
    Replacement::draft_throws(0x140927760, "content_140927760", fn_140927760 as *const (), Throws::Caught), // sub_140927760
    Replacement::draft(0x1409278f0, "content_1409278f0", fn_1409278f0 as *const ()), // sub_1409278f0
    Replacement::draft(0x140927a00, "content_140927a00", fn_140927a00 as *const ()), // sub_140927a00
    Replacement::draft(0x140927aa0, "content_140927aa0", fn_140927aa0 as *const ()), // sub_140927aa0
    Replacement::draft(0x140927b40, "content_140927b40", fn_140927b40 as *const ()), // sub_140927b40
    Replacement::draft(0x140927c80, "content_140927c80", fn_140927c80 as *const ()), // sub_140927c80
    Replacement::draft(0x140927d90, "content_140927d90", fn_140927d90 as *const ()), // sub_140927d90
    Replacement::draft(0x140927ea0, "content_140927ea0", fn_140927ea0 as *const ()), // sub_140927ea0
    Replacement::draft(0x140927f40, "content_140927f40", fn_140927f40 as *const ()), // sub_140927f40
    Replacement::draft(0x140928080, "content_140928080", fn_140928080 as *const ()), // sub_140928080
    Replacement::draft(0x140928190, "content_140928190", fn_140928190 as *const ()), // sub_140928190
    Replacement::draft(0x1409282a0, "content_1409282a0", fn_1409282a0 as *const ()), // sub_1409282a0
    Replacement::draft(0x140928340, "content_140928340", fn_140928340 as *const ()), // sub_140928340
    Replacement::draft(0x140928480, "content_140928480", fn_140928480 as *const ()), // sub_140928480
    Replacement::draft_throws(0x140928640, "content_140928640", fn_140928640 as *const (), Throws::Caught), // sub_140928640
    Replacement::draft(0x1409287f0, "content_1409287f0", fn_1409287f0 as *const ()), // sub_1409287f0
    Replacement::draft_throws(0x140928930, "content_140928930", fn_140928930 as *const (), Throws::Caught), // sub_140928930
    Replacement::draft(0x140928ae0, "content_140928ae0", fn_140928ae0 as *const ()), // sub_140928ae0
    Replacement::draft_throws(0x140928bf0, "content_140928bf0", fn_140928bf0 as *const (), Throws::Caught), // sub_140928bf0
    Replacement::draft_throws(0x140928d80, "content_140928d80", fn_140928d80 as *const (), Throws::Caught), // sub_140928d80
    Replacement::draft_throws(0x140928f10, "content_140928f10", fn_140928f10 as *const (), Throws::Caught), // sub_140928f10
    Replacement::draft(0x1409290a0, "content_1409290a0", fn_1409290a0 as *const ()), // sub_1409290a0
    Replacement::draft(0x140929230, "content_140929230", fn_140929230 as *const ()), // sub_140929230
    Replacement::draft(0x1409293a0, "content_1409293a0", fn_1409293a0 as *const ()), // sub_1409293a0
    Replacement::draft_throws(0x1409294f0, "content_1409294f0", fn_1409294f0 as *const (), Throws::Caught), // sub_1409294f0
    Replacement::draft_throws(0x140929620, "content_140929620", fn_140929620 as *const (), Throws::Caught), // sub_140929620
    Replacement::draft(0x1409297b0, "content_1409297b0", fn_1409297b0 as *const ()), // sub_1409297b0
    Replacement::draft(0x1409298c0, "content_1409298c0", fn_1409298c0 as *const ()), // sub_1409298c0
    Replacement::draft(0x140929960, "content_140929960", fn_140929960 as *const ()), // sub_140929960
    Replacement::draft(0x140929a00, "content_140929a00", fn_140929a00 as *const ()), // sub_140929a00
    Replacement::draft(0x140929b40, "content_140929b40", fn_140929b40 as *const ()), // sub_140929b40
    Replacement::draft_throws(0x140929d00, "content_140929d00", fn_140929d00 as *const (), Throws::Caught), // sub_140929d00
    Replacement::draft(0x140929eb0, "content_140929eb0", fn_140929eb0 as *const ()), // sub_140929eb0
    Replacement::draft_throws(0x140929ff0, "content_140929ff0", fn_140929ff0 as *const (), Throws::Caught), // sub_140929ff0
    Replacement::draft(0x14092a1a0, "content_14092a1a0", fn_14092a1a0 as *const ()), // sub_14092a1a0
    Replacement::draft_throws(0x14092a2b0, "content_14092a2b0", fn_14092a2b0 as *const (), Throws::Caught), // sub_14092a2b0
    Replacement::draft_throws(0x14092a440, "content_14092a440", fn_14092a440 as *const (), Throws::Caught), // sub_14092a440
    Replacement::draft_throws(0x14092a5d0, "content_14092a5d0", fn_14092a5d0 as *const (), Throws::Caught), // sub_14092a5d0
    Replacement::draft(0x14092a760, "content_14092a760", fn_14092a760 as *const ()), // sub_14092a760
    Replacement::draft(0x14092a8f0, "content_14092a8f0", fn_14092a8f0 as *const ()), // sub_14092a8f0
    Replacement::draft(0x14092aa60, "content_14092aa60", fn_14092aa60 as *const ()), // sub_14092aa60
    Replacement::draft_throws(0x14092abb0, "content_14092abb0", fn_14092abb0 as *const (), Throws::Caught), // sub_14092abb0
    Replacement::draft_throws(0x14092ace0, "content_14092ace0", fn_14092ace0 as *const (), Throws::Caught), // sub_14092ace0
    Replacement::draft(0x14092ae70, "content_14092ae70", fn_14092ae70 as *const ()), // sub_14092ae70
    Replacement::draft(0x14092af80, "content_14092af80", fn_14092af80 as *const ()), // sub_14092af80
    Replacement::draft(0x14092b020, "content_14092b020", fn_14092b020 as *const ()), // sub_14092b020
    Replacement::draft(0x14092b0c0, "content_14092b0c0", fn_14092b0c0 as *const ()), // sub_14092b0c0
    Replacement::draft(0x14092b200, "content_14092b200", fn_14092b200 as *const ()), // sub_14092b200
    Replacement::draft(0x14092b310, "content_14092b310", fn_14092b310 as *const ()), // sub_14092b310
    Replacement::draft(0x14092b420, "content_14092b420", fn_14092b420 as *const ()), // sub_14092b420
    Replacement::draft(0x14092b4c0, "content_14092b4c0", fn_14092b4c0 as *const ()), // sub_14092b4c0
    Replacement::draft(0x14092b600, "content_14092b600", fn_14092b600 as *const ()), // sub_14092b600
    Replacement::draft(0x14092b710, "content_14092b710", fn_14092b710 as *const ()), // sub_14092b710
    Replacement::draft(0x14092b820, "content_14092b820", fn_14092b820 as *const ()), // sub_14092b820
    Replacement::draft(0x14092b8c0, "content_14092b8c0", fn_14092b8c0 as *const ()), // sub_14092b8c0
    Replacement::draft(0x14092ba00, "content_14092ba00", fn_14092ba00 as *const ()), // sub_14092ba00
    Replacement::draft_throws(0x14092bbc0, "content_14092bbc0", fn_14092bbc0 as *const (), Throws::Caught), // sub_14092bbc0
    Replacement::draft(0x14092bd70, "content_14092bd70", fn_14092bd70 as *const ()), // sub_14092bd70
    Replacement::draft_throws(0x14092beb0, "content_14092beb0", fn_14092beb0 as *const (), Throws::Caught), // sub_14092beb0
    Replacement::draft(0x14092c060, "content_14092c060", fn_14092c060 as *const ()), // sub_14092c060
    Replacement::draft_throws(0x14092c170, "content_14092c170", fn_14092c170 as *const (), Throws::Caught), // sub_14092c170
    Replacement::draft_throws(0x14092c300, "content_14092c300", fn_14092c300 as *const (), Throws::Caught), // sub_14092c300
    Replacement::draft_throws(0x14092c490, "content_14092c490", fn_14092c490 as *const (), Throws::Caught), // sub_14092c490
    Replacement::draft(0x14092c620, "content_14092c620", fn_14092c620 as *const ()), // sub_14092c620
    Replacement::draft(0x14092c7b0, "content_14092c7b0", fn_14092c7b0 as *const ()), // sub_14092c7b0
    Replacement::draft(0x14092c920, "content_14092c920", fn_14092c920 as *const ()), // sub_14092c920
    Replacement::draft_throws(0x14092ca70, "content_14092ca70", fn_14092ca70 as *const (), Throws::Caught), // sub_14092ca70
    Replacement::draft_throws(0x14092cba0, "content_14092cba0", fn_14092cba0 as *const (), Throws::Caught), // sub_14092cba0
    Replacement::draft(0x14092cd30, "content_14092cd30", fn_14092cd30 as *const ()), // sub_14092cd30
    Replacement::draft(0x14092ce40, "content_14092ce40", fn_14092ce40 as *const ()), // sub_14092ce40
    Replacement::draft(0x14092cee0, "content_14092cee0", fn_14092cee0 as *const ()), // sub_14092cee0
    Replacement::draft(0x14092cf80, "content_14092cf80", fn_14092cf80 as *const ()), // sub_14092cf80
    Replacement::draft(0x14092d0c0, "content_14092d0c0", fn_14092d0c0 as *const ()), // sub_14092d0c0
    Replacement::draft_throws(0x14092d280, "content_14092d280", fn_14092d280 as *const (), Throws::Caught), // sub_14092d280
    Replacement::draft(0x14092d430, "content_14092d430", fn_14092d430 as *const ()), // sub_14092d430
    Replacement::draft_throws(0x14092d570, "content_14092d570", fn_14092d570 as *const (), Throws::Caught), // sub_14092d570
    Replacement::draft(0x14092d720, "content_14092d720", fn_14092d720 as *const ()), // sub_14092d720
    Replacement::draft_throws(0x14092d830, "content_14092d830", fn_14092d830 as *const (), Throws::Caught), // sub_14092d830
    Replacement::draft_throws(0x14092d9c0, "content_14092d9c0", fn_14092d9c0 as *const (), Throws::Caught), // sub_14092d9c0
    Replacement::draft_throws(0x14092db50, "content_14092db50", fn_14092db50 as *const (), Throws::Caught), // sub_14092db50
    Replacement::draft(0x14092dce0, "content_14092dce0", fn_14092dce0 as *const ()), // sub_14092dce0
    Replacement::draft(0x14092de70, "content_14092de70", fn_14092de70 as *const ()), // sub_14092de70
    Replacement::draft(0x14092dfe0, "content_14092dfe0", fn_14092dfe0 as *const ()), // sub_14092dfe0
    Replacement::draft_throws(0x14092e130, "content_14092e130", fn_14092e130 as *const (), Throws::Caught), // sub_14092e130
    Replacement::draft_throws(0x14092e260, "content_14092e260", fn_14092e260 as *const (), Throws::Caught), // sub_14092e260
    Replacement::draft(0x14092e3f0, "content_14092e3f0", fn_14092e3f0 as *const ()), // sub_14092e3f0
    Replacement::draft(0x14092e500, "content_14092e500", fn_14092e500 as *const ()), // sub_14092e500
    Replacement::draft(0x14092e5a0, "content_14092e5a0", fn_14092e5a0 as *const ()), // sub_14092e5a0
    Replacement::draft(0x14092e640, "content_14092e640", fn_14092e640 as *const ()), // sub_14092e640
    Replacement::draft(0x14092e780, "content_14092e780", fn_14092e780 as *const ()), // sub_14092e780
    Replacement::draft_throws(0x14092e940, "content_14092e940", fn_14092e940 as *const (), Throws::Caught), // sub_14092e940
    Replacement::draft(0x14092eaf0, "content_14092eaf0", fn_14092eaf0 as *const ()), // sub_14092eaf0
    Replacement::draft_throws(0x14092ec30, "content_14092ec30", fn_14092ec30 as *const (), Throws::Caught), // sub_14092ec30
    Replacement::draft(0x14092ede0, "content_14092ede0", fn_14092ede0 as *const ()), // sub_14092ede0
    Replacement::draft_throws(0x14092eef0, "content_14092eef0", fn_14092eef0 as *const (), Throws::Caught), // sub_14092eef0
    Replacement::draft_throws(0x14092f080, "content_14092f080", fn_14092f080 as *const (), Throws::Caught), // sub_14092f080
    Replacement::draft_throws(0x14092f210, "content_14092f210", fn_14092f210 as *const (), Throws::Caught), // sub_14092f210
    Replacement::draft(0x14092f3a0, "content_14092f3a0", fn_14092f3a0 as *const ()), // sub_14092f3a0
    Replacement::draft(0x14092f530, "content_14092f530", fn_14092f530 as *const ()), // sub_14092f530
    Replacement::draft(0x14092f6a0, "content_14092f6a0", fn_14092f6a0 as *const ()), // sub_14092f6a0
    Replacement::draft_throws(0x14092f7f0, "content_14092f7f0", fn_14092f7f0 as *const (), Throws::Caught), // sub_14092f7f0
    Replacement::draft_throws(0x14092f920, "content_14092f920", fn_14092f920 as *const (), Throws::Caught), // sub_14092f920
    Replacement::draft(0x14092fab0, "content_14092fab0", fn_14092fab0 as *const ()), // sub_14092fab0
    Replacement::draft(0x14092fbc0, "content_14092fbc0", fn_14092fbc0 as *const ()), // sub_14092fbc0
    Replacement::draft(0x14092fc60, "content_14092fc60", fn_14092fc60 as *const ()), // sub_14092fc60
    Replacement::draft(0x14092fd00, "content_14092fd00", fn_14092fd00 as *const ()), // sub_14092fd00
    Replacement::draft(0x14092fe40, "content_14092fe40", fn_14092fe40 as *const ()), // sub_14092fe40
    Replacement::draft_throws(0x140930000, "content_140930000", fn_140930000 as *const (), Throws::Caught), // sub_140930000
    Replacement::draft(0x1409301b0, "content_1409301b0", fn_1409301b0 as *const ()), // sub_1409301b0
    Replacement::draft_throws(0x1409302f0, "content_1409302f0", fn_1409302f0 as *const (), Throws::Caught), // sub_1409302f0
    Replacement::draft(0x1409304a0, "content_1409304a0", fn_1409304a0 as *const ()), // sub_1409304a0
    Replacement::draft_throws(0x1409305b0, "content_1409305b0", fn_1409305b0 as *const (), Throws::Caught), // sub_1409305b0
    Replacement::draft_throws(0x140930740, "content_140930740", fn_140930740 as *const (), Throws::Caught), // sub_140930740
    Replacement::draft_throws(0x1409308d0, "content_1409308d0", fn_1409308d0 as *const (), Throws::Caught), // sub_1409308d0
    Replacement::draft(0x140930a60, "content_140930a60", fn_140930a60 as *const ()), // sub_140930a60
    Replacement::draft(0x140930bf0, "content_140930bf0", fn_140930bf0 as *const ()), // sub_140930bf0
    Replacement::draft(0x140930d60, "content_140930d60", fn_140930d60 as *const ()), // sub_140930d60
    Replacement::draft_throws(0x140930eb0, "content_140930eb0", fn_140930eb0 as *const (), Throws::Caught), // sub_140930eb0
    Replacement::draft_throws(0x140930fe0, "content_140930fe0", fn_140930fe0 as *const (), Throws::Caught), // sub_140930fe0
    Replacement::draft(0x140931170, "content_140931170", fn_140931170 as *const ()), // sub_140931170
    Replacement::draft(0x140931280, "content_140931280", fn_140931280 as *const ()), // sub_140931280
    Replacement::draft(0x140931320, "content_140931320", fn_140931320 as *const ()), // sub_140931320
    Replacement::draft(0x1409313c0, "content_1409313c0", fn_1409313c0 as *const ()), // sub_1409313c0
    Replacement::draft(0x140931500, "content_140931500", fn_140931500 as *const ()), // sub_140931500
    Replacement::draft(0x140931610, "content_140931610", fn_140931610 as *const ()), // sub_140931610
    Replacement::draft(0x140931720, "content_140931720", fn_140931720 as *const ()), // sub_140931720
    Replacement::draft(0x1409317c0, "content_1409317c0", fn_1409317c0 as *const ()), // sub_1409317c0
    Replacement::draft(0x140931900, "content_140931900", fn_140931900 as *const ()), // sub_140931900
    Replacement::draft(0x140931a10, "content_140931a10", fn_140931a10 as *const ()), // sub_140931a10
    Replacement::draft(0x140931b20, "content_140931b20", fn_140931b20 as *const ()), // sub_140931b20
    Replacement::draft(0x140931bc0, "content_140931bc0", fn_140931bc0 as *const ()), // sub_140931bc0
    Replacement::draft(0x140931d00, "content_140931d00", fn_140931d00 as *const ()), // sub_140931d00
    Replacement::draft(0x140931e10, "content_140931e10", fn_140931e10 as *const ()), // sub_140931e10
    Replacement::draft(0x140931f20, "content_140931f20", fn_140931f20 as *const ()), // sub_140931f20
    Replacement::draft(0x140931fc0, "content_140931fc0", fn_140931fc0 as *const ()), // sub_140931fc0
    Replacement::draft(0x140932100, "content_140932100", fn_140932100 as *const ()), // sub_140932100
    Replacement::draft_throws(0x1409322c0, "content_1409322c0", fn_1409322c0 as *const (), Throws::Caught), // sub_1409322c0
    Replacement::draft(0x140932470, "content_140932470", fn_140932470 as *const ()), // sub_140932470
    Replacement::draft_throws(0x1409325b0, "content_1409325b0", fn_1409325b0 as *const (), Throws::Caught), // sub_1409325b0
    Replacement::draft(0x140932760, "content_140932760", fn_140932760 as *const ()), // sub_140932760
    Replacement::draft_throws(0x140932870, "content_140932870", fn_140932870 as *const (), Throws::Caught), // sub_140932870
    Replacement::draft_throws(0x140932a00, "content_140932a00", fn_140932a00 as *const (), Throws::Caught), // sub_140932a00
    Replacement::draft_throws(0x140932b90, "content_140932b90", fn_140932b90 as *const (), Throws::Caught), // sub_140932b90
    Replacement::draft(0x140932d20, "content_140932d20", fn_140932d20 as *const ()), // sub_140932d20
    Replacement::draft(0x140932eb0, "content_140932eb0", fn_140932eb0 as *const ()), // sub_140932eb0
    Replacement::draft(0x140933020, "content_140933020", fn_140933020 as *const ()), // sub_140933020
    Replacement::draft_throws(0x140933170, "content_140933170", fn_140933170 as *const (), Throws::Caught), // sub_140933170
    Replacement::draft_throws(0x1409332a0, "content_1409332a0", fn_1409332a0 as *const (), Throws::Caught), // sub_1409332a0
    Replacement::draft(0x140933430, "content_140933430", fn_140933430 as *const ()), // sub_140933430
    Replacement::draft(0x140933540, "content_140933540", fn_140933540 as *const ()), // sub_140933540
    Replacement::draft(0x1409335e0, "content_1409335e0", fn_1409335e0 as *const ()), // sub_1409335e0
    Replacement::draft(0x140933680, "content_140933680", fn_140933680 as *const ()), // sub_140933680
    Replacement::draft(0x1409337c0, "content_1409337c0", fn_1409337c0 as *const ()), // sub_1409337c0
    Replacement::draft(0x140933b30, "content_140933b30", fn_140933b30 as *const ()), // sub_140933b30
    Replacement::draft_throws(0x140933c70, "content_140933c70", fn_140933c70 as *const (), Throws::Caught), // sub_140933c70
    Replacement::draft(0x140933e20, "content_140933e20", fn_140933e20 as *const ()), // sub_140933e20
    Replacement::draft_throws(0x140933f30, "content_140933f30", fn_140933f30 as *const (), Throws::Caught), // sub_140933f30
    Replacement::draft_throws(0x1409340c0, "content_1409340c0", fn_1409340c0 as *const (), Throws::Caught), // sub_1409340c0
    Replacement::draft_throws(0x140934250, "content_140934250", fn_140934250 as *const (), Throws::Caught), // sub_140934250
    Replacement::draft(0x1409343e0, "content_1409343e0", fn_1409343e0 as *const ()), // sub_1409343e0
    Replacement::draft(0x140934570, "content_140934570", fn_140934570 as *const ()), // sub_140934570
    Replacement::draft(0x1409346e0, "content_1409346e0", fn_1409346e0 as *const ()), // sub_1409346e0
    Replacement::draft_throws(0x140934830, "content_140934830", fn_140934830 as *const (), Throws::Caught), // sub_140934830
    Replacement::draft_throws(0x140934960, "content_140934960", fn_140934960 as *const (), Throws::Caught), // sub_140934960
    Replacement::draft(0x140934af0, "content_140934af0", fn_140934af0 as *const ()), // sub_140934af0
    Replacement::draft(0x140934c00, "content_140934c00", fn_140934c00 as *const ()), // sub_140934c00
    Replacement::draft(0x140934ca0, "content_140934ca0", fn_140934ca0 as *const ()), // sub_140934ca0
    Replacement::draft(0x140934d40, "content_140934d40", fn_140934d40 as *const ()), // sub_140934d40
    Replacement::draft(0x140934dd0, "content_140934dd0", fn_140934dd0 as *const ()), // sub_140934dd0
    Replacement::draft(0x140934f60, "content_140934f60", fn_140934f60 as *const ()), // sub_140934f60
    Replacement::draft(0x140935010, "content_140935010", fn_140935010 as *const ()), // sub_140935010
    Replacement::draft(0x1409351d0, "content_1409351d0", fn_1409351d0 as *const ()), // sub_1409351d0
    Replacement::draft_throws(0x1409356f0, "content_1409356f0", fn_1409356f0 as *const (), Throws::Caught), // sub_1409356f0
    Replacement::draft(0x140935eb0, "content_140935eb0", fn_140935eb0 as *const ()), // sub_140935eb0
    Replacement::draft_throws(0x140936520, "content_140936520", fn_140936520 as *const (), Throws::Caught), // sub_140936520
    Replacement::draft(0x140936e50, "content_140936e50", fn_140936e50 as *const ()), // sub_140936e50
    Replacement::draft_throws(0x1409372f0, "content_1409372f0", fn_1409372f0 as *const (), Throws::Caught), // sub_1409372f0
    Replacement::draft_throws(0x140937920, "content_140937920", fn_140937920 as *const (), Throws::Caught), // sub_140937920
    Replacement::draft_throws(0x140937f20, "content_140937f20", fn_140937f20 as *const (), Throws::Caught), // sub_140937f20
    Replacement::draft(0x140938510, "content_140938510", fn_140938510 as *const ()), // sub_140938510
    Replacement::draft(0x140938a30, "content_140938a30", fn_140938a30 as *const ()), // sub_140938a30
    Replacement::draft(0x140939050, "content_140939050", fn_140939050 as *const ()), // sub_140939050
    Replacement::draft_throws(0x140939670, "content_140939670", fn_140939670 as *const (), Throws::Discard), // sub_140939670
    Replacement::draft_throws(0x140939c10, "content_140939c10", fn_140939c10 as *const (), Throws::Discard), // sub_140939c10
    Replacement::draft(0x14093a020, "content_14093a020", fn_14093a020 as *const ()), // sub_14093a020
    Replacement::draft(0x14093a0c0, "content_14093a0c0", tpl_11191::<0x14387dbf8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x48, 0x1426d4580, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_14093a0c0
    Replacement::draft_throws(0x14093a1a0, "content_14093a1a0", tpl_62882::<0x14387db08, 0x14387dbf8, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x48, 0x1401e9e70> as *const (), Throws::Discard), // sub_14093a1a0
    Replacement::draft(0x14093a280, "content_14093a280", tpl_69732::<0x14387dbf8, 0x14243b0b0, 0x338, 0x20, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14093a280
    Replacement::draft_throws(0x14093a300, "content_14093a300", tpl_11193::<0x14387db08, 0x14387dbf8, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x120, 0x48, 0x1401e9a20> as *const (), Throws::Discard), // sub_14093a300
    Replacement::draft_throws(0x14093a3e0, "content_14093a3e0", fn_14093a3e0 as *const (), Throws::Caught), // sub_14093a3e0
    Replacement::draft_throws(0x14093a4c0, "content_14093a4c0", fn_14093a4c0 as *const (), Throws::Caught), // sub_14093a4c0
    Replacement::draft_throws(0x14093a5a0, "content_14093a5a0", fn_14093a5a0 as *const (), Throws::Caught), // sub_14093a5a0
    Replacement::draft(0x14093a680, "content_14093a680", tpl_11195::<0x14387dbf8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14093a680
    Replacement::draft(0x14093a730, "content_14093a730", tpl_2262::<0x14387db08, 0x14387dbf8, 0x1401e9970, 0x338, 0xc0, 0x48, 0xe8, 0x1401e9860> as *const ()), // sub_14093a730
    Replacement::draft(0x14093a7b0, "content_14093a7b0", tpl_2258::<0x14387dbf8, 0x14243b0b0, 0x338, 0xc0, 0x48, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14093a7b0
    Replacement::draft_throws(0x14093a840, "content_14093a840", tpl_11196::<0x14387db08, 0x14387dbf8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x48, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14093a840
    Replacement::draft_throws(0x14093a8c0, "content_14093a8c0", tpl_11197::<0x14387dbf8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14307eae0, 0x1402c4730, 0x48, 0x1432769f0, 0x1402c4e50> as *const (), Throws::Discard), // sub_14093a8c0
    Replacement::draft(0x14093a9b0, "content_14093a9b0", fn_14093a9b0 as *const ()), // sub_14093a9b0
    Replacement::draft(0x14093aa60, "content_14093aa60", fn_14093aa60 as *const ()), // sub_14093aa60
    Replacement::draft(0x14093aae0, "content_14093aae0", tpl_9963::<0x14387dab8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x38, 0x33, 0x34, 0x30, 0xfffffffffffffff8> as *const ()), // sub_14093aae0
    Replacement::draft_throws(0x14093abd0, "content_14093abd0", tpl_68527::<0x14387d9f0, 0x14387dab8, 0x1401e9fa0, 0x328, 0x1426cdcb0, 0x338, 0x20, 0x369, 0x142409b50, 0x30, 0x1401e9e70> as *const (), Throws::Discard), // sub_14093abd0
    Replacement::draft(0x14093acd0, "content_14093acd0", tpl_9959::<0x14387dab8, 0x14243b0b0, 0x338, 0x20, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14093acd0
    Replacement::draft_throws(0x14093ad50, "content_14093ad50", tpl_68528::<0x14387d9f0, 0x14387dab8, 0x1401e9b60, 0x328, 0x1426cdcb0, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x30, 0x1401e9a20> as *const (), Throws::Discard), // sub_14093ad50
    Replacement::draft_throws(0x14093ae50, "content_14093ae50", fn_14093ae50 as *const (), Throws::Caught), // sub_14093ae50
    Replacement::draft_throws(0x14093b220, "content_14093b220", fn_14093b220 as *const (), Throws::Caught), // sub_14093b220
    Replacement::draft_throws(0x14093b590, "content_14093b590", fn_14093b590 as *const (), Throws::Caught), // sub_14093b590
    Replacement::draft(0x14093b8f0, "content_14093b8f0", tpl_9964::<0x14387dab8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14093b8f0
    Replacement::draft(0x14093b9c0, "content_14093b9c0", tpl_9961::<0x14387d9f0, 0x14387dab8, 0x1401e9970, 0x338, 0xc0, 0x30, 0x1401e9860> as *const ()), // sub_14093b9c0
    Replacement::draft(0x14093ba40, "content_14093ba40", tpl_9959::<0x14387dab8, 0x14243b0b0, 0x338, 0xc0, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14093ba40
    Replacement::draft_throws(0x14093bad0, "content_14093bad0", tpl_9965::<0x14387d9f0, 0x14387dab8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x30, 0x1401e9590> as *const (), Throws::Discard), // sub_14093bad0
    Replacement::draft_throws(0x14093bb40, "content_14093bb40", tpl_10011::<0x14387dab8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1431aa088, 0x1402c4730, 0x30, 0x143276a30> as *const (), Throws::Discard), // sub_14093bb40
    Replacement::draft(0x14093bc20, "content_14093bc20", tpl_11203::<0xe8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14093bce0, 0x143278778, 0xc0, 0xc8> as *const ()), // sub_14093bc20
    Replacement::draft(0x14093bce0, "content_14093bce0", fn_14093bce0 as *const ()), // sub_14093bce0
    Replacement::draft(0x14093bda0, "content_14093bda0", fn_14093bda0 as *const ()), // sub_14093bda0
    Replacement::draft(0x14093be90, "content_14093be90", fn_14093be90 as *const ()), // sub_14093be90
    Replacement::draft(0x14093bf00, "content_14093bf00", fn_14093bf00 as *const ()), // sub_14093bf00
    Replacement::draft_throws(0x14093c0b0, "content_14093c0b0", fn_14093c0b0 as *const (), Throws::Discard), // sub_14093c0b0
    Replacement::draft(0x14093c240, "content_14093c240", fn_14093c240 as *const ()), // sub_14093c240
    Replacement::draft_throws(0x14093c350, "content_14093c350", fn_14093c350 as *const (), Throws::Discard), // sub_14093c350
    Replacement::draft(0x14093c500, "content_14093c500", fn_14093c500 as *const ()), // sub_14093c500
    Replacement::draft_throws(0x14093c5c0, "content_14093c5c0", fn_14093c5c0 as *const (), Throws::Caught), // sub_14093c5c0
    Replacement::draft_throws(0x14093ca30, "content_14093ca30", fn_14093ca30 as *const (), Throws::Caught), // sub_14093ca30
    Replacement::draft_throws(0x14093ce10, "content_14093ce10", fn_14093ce10 as *const (), Throws::Caught), // sub_14093ce10
    Replacement::draft(0x14093d1f0, "content_14093d1f0", fn_14093d1f0 as *const ()), // sub_14093d1f0
    Replacement::draft(0x14093d2a0, "content_14093d2a0", fn_14093d2a0 as *const ()), // sub_14093d2a0
    Replacement::draft(0x14093d3c0, "content_14093d3c0", fn_14093d3c0 as *const ()), // sub_14093d3c0
    Replacement::draft_throws(0x14093d4f0, "content_14093d4f0", fn_14093d4f0 as *const (), Throws::Discard), // sub_14093d4f0
    Replacement::draft_throws(0x14093d600, "content_14093d600", fn_14093d600 as *const (), Throws::Discard), // sub_14093d600
    Replacement::draft_throws(0x14093d7f0, "content_14093d7f0", fn_14093d7f0 as *const (), Throws::Discard), // sub_14093d7f0
    Replacement::draft(0x14093d950, "content_14093d950", fn_14093d950 as *const ()), // sub_14093d950
    Replacement::draft(0x14093d9c0, "content_14093d9c0", fn_14093d9c0 as *const ()), // sub_14093d9c0
    Replacement::draft(0x14093da60, "content_14093da60", fn_14093da60 as *const ()), // sub_14093da60
    Replacement::draft(0x14093db20, "content_14093db20", fn_14093db20 as *const ()), // sub_14093db20
    Replacement::draft(0x14093dc00, "content_14093dc00", fn_14093dc00 as *const ()), // sub_14093dc00
    Replacement::draft_throws(0x14093dd00, "content_14093dd00", fn_14093dd00 as *const (), Throws::Discard), // sub_14093dd00
    Replacement::draft(0x14093de00, "content_14093de00", tpl_11224::<0x14387da08, 0x14243b0b0, 0x14093eef0, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x338, 0xc0, 0x80> as *const ()), // sub_14093de00
    Replacement::draft_throws(0x14093def0, "content_14093def0", fn_14093def0 as *const (), Throws::Discard), // sub_14093def0
    Replacement::draft_throws(0x14093dff0, "content_14093dff0", fn_14093dff0 as *const (), Throws::Caught), // sub_14093dff0
    Replacement::draft_throws(0x14093e0d0, "content_14093e0d0", fn_14093e0d0 as *const (), Throws::Caught), // sub_14093e0d0
    Replacement::draft_throws(0x14093e1b0, "content_14093e1b0", fn_14093e1b0 as *const (), Throws::Caught), // sub_14093e1b0
    Replacement::draft(0x14093e290, "content_14093e290", fn_14093e290 as *const ()), // sub_14093e290
    Replacement::draft(0x14093e370, "content_14093e370", fn_14093e370 as *const ()), // sub_14093e370
    Replacement::draft(0x14093e450, "content_14093e450", tpl_11224::<0x14387da08, 0x14243b0b0, 0x140940140, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x338, 0xc0, 0x120> as *const ()), // sub_14093e450
    Replacement::draft_throws(0x14093e540, "content_14093e540", fn_14093e540 as *const (), Throws::Discard), // sub_14093e540
    Replacement::draft_throws(0x14093e630, "content_14093e630", fn_14093e630 as *const (), Throws::Discard), // sub_14093e630
    Replacement::draft(0x14093e710, "content_14093e710", fn_14093e710 as *const ()), // sub_14093e710
    Replacement::draft(0x14093e7a0, "content_14093e7a0", fn_14093e7a0 as *const ()), // sub_14093e7a0
    Replacement::draft(0x14093e850, "content_14093e850", fn_14093e850 as *const ()), // sub_14093e850
    Replacement::draft(0x14093e960, "content_14093e960", fn_14093e960 as *const ()), // sub_14093e960
    Replacement::draft(0x14093eef0, "content_14093eef0", fn_14093eef0 as *const ()), // sub_14093eef0
    Replacement::draft_throws(0x14093f090, "content_14093f090", fn_14093f090 as *const (), Throws::Caught), // sub_14093f090
    Replacement::draft(0x14093f400, "content_14093f400", fn_14093f400 as *const ()), // sub_14093f400
    Replacement::draft_throws(0x14093f540, "content_14093f540", fn_14093f540 as *const (), Throws::Caught), // sub_14093f540
    Replacement::draft_throws(0x14093f680, "content_14093f680", fn_14093f680 as *const (), Throws::Caught), // sub_14093f680
    Replacement::draft_throws(0x14093f940, "content_14093f940", fn_14093f940 as *const (), Throws::Caught), // sub_14093f940
    Replacement::draft(0x14093fd30, "content_14093fd30", fn_14093fd30 as *const ()), // sub_14093fd30
    Replacement::draft(0x14093ffc0, "content_14093ffc0", fn_14093ffc0 as *const ()), // sub_14093ffc0
    Replacement::draft(0x140940140, "content_140940140", fn_140940140 as *const ()), // sub_140940140
    Replacement::draft_throws(0x1409402d0, "content_1409402d0", fn_1409402d0 as *const (), Throws::Discard), // sub_1409402d0
    Replacement::draft_throws(0x140940490, "content_140940490", fn_140940490 as *const (), Throws::Discard), // sub_140940490
    Replacement::draft(0x140940630, "content_140940630", fn_140940630 as *const ()), // sub_140940630
    Replacement::draft(0x1409406c0, "content_1409406c0", tpl_11203::<0xe8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x14093bce0, 0x14327da78, 0xc8, 0xd0> as *const ()), // sub_1409406c0
    Replacement::draft(0x140940780, "content_140940780", fn_140940780 as *const ()), // sub_140940780
    Replacement::draft(0x140940870, "content_140940870", tpl_11241::<0x14387da40, 0x14243b0b0, 0x14093e850, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140940870
    Replacement::draft(0x1409408e0, "content_1409408e0", fn_1409408e0 as *const ()), // sub_1409408e0
    Replacement::draft_throws(0x1409409d0, "content_1409409d0", fn_1409409d0 as *const (), Throws::Discard), // sub_1409409d0
    Replacement::draft(0x140940af0, "content_140940af0", tpl_11244::<0x14387da40, 0x14243b0b0, 0x14093eef0, 0x338, 0xc0, 0x38, 0xc4, 0x80, 0xc8, 0x1406daa00, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140940af0
    Replacement::draft_throws(0x140940ba0, "content_140940ba0", fn_140940ba0 as *const (), Throws::Discard), // sub_140940ba0
    Replacement::draft(0x140940c90, "content_140940c90", fn_140940c90 as *const ()), // sub_140940c90
    Replacement::draft_throws(0x140940d40, "content_140940d40", fn_140940d40 as *const (), Throws::Caught), // sub_140940d40
    Replacement::draft_throws(0x140940e90, "content_140940e90", fn_140940e90 as *const (), Throws::Caught), // sub_140940e90
    Replacement::draft_throws(0x140940fe0, "content_140940fe0", fn_140940fe0 as *const (), Throws::Caught), // sub_140940fe0
    Replacement::draft(0x140941130, "content_140941130", fn_140941130 as *const ()), // sub_140941130
    Replacement::draft(0x1409411f0, "content_1409411f0", fn_1409411f0 as *const ()), // sub_1409411f0
    Replacement::draft(0x140941290, "content_140941290", tpl_11244::<0x14387da40, 0x14243b0b0, 0x140940140, 0x338, 0xc0, 0xd8, 0xc4, 0x120, 0xc8, 0x1406db020, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140941290
    Replacement::draft_throws(0x140941340, "content_140941340", fn_140941340 as *const (), Throws::Discard), // sub_140941340
    Replacement::draft_throws(0x140941440, "content_140941440", fn_140941440 as *const (), Throws::Discard), // sub_140941440
    Replacement::draft_throws(0x140941580, "content_140941580", fn_140941580 as *const (), Throws::Discard), // sub_140941580
    Replacement::draft(0x140941610, "content_140941610", fn_140941610 as *const ()), // sub_140941610
    Replacement::draft(0x140941900, "content_140941900", fn_140941900 as *const ()), // content::GIVolumeEntity::content::GIVolumeEntity::vfunc_0
    Replacement::draft(0x140941970, "content_140941970", fn_140941970 as *const ()), // sub_140941970
    Replacement::draft(0x140941a30, "content_140941a30", fn_140941a30 as *const ()), // content::HierarchyGroup::content::HierarchyGroup::vfunc_0
    Replacement::draft(0x140941aa0, "content_140941aa0", fn_140941aa0 as *const ()), // content::HierarchyLOD::content::HierarchyLOD::vfunc_0
    Replacement::draft(0x140941b10, "content_140941b10", fn_140941b10 as *const ()), // sub_140941b10
    Replacement::draft(0x140941bd0, "content_140941bd0", fn_140941bd0 as *const ()), // sub_140941bd0
    Replacement::draft(0x140941ca0, "content_140941ca0", fn_140941ca0 as *const ()), // sub_140941ca0
    Replacement::draft(0x140941d80, "content_140941d80", fn_140941d80 as *const ()), // sub_140941d80
    Replacement::draft_throws(0x140941ea0, "content_140941ea0", fn_140941ea0 as *const (), Throws::Caught), // sub_140941ea0
    Replacement::draft_throws(0x140942040, "content_140942040", fn_140942040 as *const (), Throws::Caught), // sub_140942040
    Replacement::draft_throws(0x140942190, "content_140942190", fn_140942190 as *const (), Throws::Caught), // sub_140942190
    Replacement::draft_throws(0x1409422b0, "content_1409422b0", tpl_11264::<0x20, 0x8, 0x1402e3560> as *const (), Throws::Discard), // sub_1409422b0
    Replacement::draft_throws(0x140942300, "content_140942300", tpl_11265::<0x20, 0x8, 0x1409466a0> as *const (), Throws::Discard), // sub_140942300
    Replacement::draft_throws(0x140942350, "content_140942350", fn_140942350 as *const (), Throws::Discard), // sub_140942350
    Replacement::draft(0x1409423a0, "content_1409423a0", fn_1409423a0 as *const ()), // sub_1409423a0
    Replacement::draft_throws(0x140942440, "content_140942440", fn_140942440 as *const (), Throws::Caught), // sub_140942440
    Replacement::draft_throws(0x140942740, "content_140942740", fn_140942740 as *const (), Throws::Caught), // sub_140942740
    Replacement::draft_throws(0x140942a40, "content_140942a40", fn_140942a40 as *const (), Throws::Caught), // sub_140942a40
    Replacement::draft(0x140942d40, "content_140942d40", fn_140942d40 as *const ()), // sub_140942d40
    Replacement::draft_throws(0x140942db0, "content_140942db0", fn_140942db0 as *const (), Throws::Caught), // sub_140942db0
    Replacement::draft_throws(0x1409430b0, "content_1409430b0", fn_1409430b0 as *const (), Throws::Caught), // sub_1409430b0
    Replacement::draft_throws(0x140943260, "content_140943260", fn_140943260 as *const (), Throws::Caught), // sub_140943260
    Replacement::draft_throws(0x140943570, "content_140943570", fn_140943570 as *const (), Throws::Caught), // sub_140943570
    Replacement::draft_throws(0x140943880, "content_140943880", tpl_2507::<0x20, 0x8, 0x140947d20> as *const (), Throws::Discard), // sub_140943880
    Replacement::draft(0x1409438d0, "content_1409438d0", fn_1409438d0 as *const ()), // sub_1409438d0
    Replacement::draft(0x140943a00, "content_140943a00", fn_140943a00 as *const ()), // sub_140943a00
    Replacement::draft(0x140943b30, "content_140943b30", tpl_69810::<0x14094b900, 0x14240ab10> as *const ()), // sub_140943b30
    Replacement::draft(0x140943db0, "content_140943db0", fn_140943db0 as *const ()), // sub_140943db0
    Replacement::draft(0x140944060, "content_140944060", fn_140944060 as *const ()), // sub_140944060
    Replacement::draft_throws(0x140944100, "content_140944100", fn_140944100 as *const (), Throws::Caught), // sub_140944100
    Replacement::draft(0x140944400, "content_140944400", fn_140944400 as *const ()), // sub_140944400
    Replacement::draft_throws(0x1409444a0, "content_1409444a0", fn_1409444a0 as *const (), Throws::Caught), // sub_1409444a0
    Replacement::draft_throws(0x1409447a0, "content_1409447a0", fn_1409447a0 as *const (), Throws::Caught), // sub_1409447a0
    Replacement::draft_throws(0x140944ab0, "content_140944ab0", fn_140944ab0 as *const (), Throws::Caught), // sub_140944ab0
    Replacement::draft_throws(0x140944dc0, "content_140944dc0", fn_140944dc0 as *const (), Throws::Caught), // sub_140944dc0
    Replacement::draft_throws(0x140945250, "content_140945250", fn_140945250 as *const (), Throws::Caught), // sub_140945250
    Replacement::draft_throws(0x140945520, "content_140945520", fn_140945520 as *const (), Throws::Caught), // sub_140945520
    Replacement::draft_throws(0x1409457f0, "content_1409457f0", fn_1409457f0 as *const (), Throws::Caught), // sub_1409457f0
    Replacement::draft_throws(0x140945c40, "content_140945c40", fn_140945c40 as *const (), Throws::Caught), // sub_140945c40
    Replacement::draft_throws(0x140945f00, "content_140945f00", fn_140945f00 as *const (), Throws::Caught), // sub_140945f00
    Replacement::draft_throws(0x1409461c0, "content_1409461c0", fn_1409461c0 as *const (), Throws::Caught), // sub_1409461c0
    Replacement::draft(0x140946600, "content_140946600", fn_140946600 as *const ()), // sub_140946600
    Replacement::draft_throws(0x1409466a0, "content_1409466a0", fn_1409466a0 as *const (), Throws::Caught), // sub_1409466a0
    Replacement::draft_throws(0x140946790, "content_140946790", fn_140946790 as *const (), Throws::Caught), // sub_140946790
    Replacement::draft(0x140946860, "content_140946860", fn_140946860 as *const ()), // sub_140946860
    Replacement::draft(0x140946a90, "content_140946a90", fn_140946a90 as *const ()), // sub_140946a90
    Replacement::draft_throws(0x140946b90, "content_140946b90", fn_140946b90 as *const (), Throws::Discard), // sub_140946b90
    Replacement::draft(0x140946c50, "content_140946c50", fn_140946c50 as *const ()), // sub_140946c50
    Replacement::draft_throws(0x140946cd0, "content_140946cd0", fn_140946cd0 as *const (), Throws::Caught), // sub_140946cd0
    Replacement::draft_throws(0x140946d90, "content_140946d90", fn_140946d90 as *const (), Throws::Caught), // sub_140946d90
    Replacement::draft_throws(0x140947110, "content_140947110", fn_140947110 as *const (), Throws::Caught), // sub_140947110
    Replacement::draft_throws(0x1409471d0, "content_1409471d0", fn_1409471d0 as *const (), Throws::Caught), // sub_1409471d0
    Replacement::draft_throws(0x140947500, "content_140947500", fn_140947500 as *const (), Throws::Caught), // sub_140947500
    Replacement::draft_throws(0x1409475c0, "content_1409475c0", fn_1409475c0 as *const (), Throws::Caught), // sub_1409475c0
    Replacement::draft(0x1409478d0, "content_1409478d0", fn_1409478d0 as *const ()), // sub_1409478d0
    Replacement::draft(0x140947a00, "content_140947a00", fn_140947a00 as *const ()), // sub_140947a00
    Replacement::draft(0x140947ac0, "content_140947ac0", fn_140947ac0 as *const ()), // sub_140947ac0
    Replacement::draft_throws(0x140947b90, "content_140947b90", fn_140947b90 as *const (), Throws::Discard), // sub_140947b90
    Replacement::draft_throws(0x140947c50, "content_140947c50", fn_140947c50 as *const (), Throws::Discard), // sub_140947c50
    Replacement::draft_throws(0x140947d20, "content_140947d20", fn_140947d20 as *const (), Throws::Caught), // sub_140947d20
    Replacement::draft(0x140947e10, "content_140947e10", fn_140947e10 as *const ()), // sub_140947e10
    Replacement::draft(0x140947eb0, "content_140947eb0", fn_140947eb0 as *const ()), // sub_140947eb0
    Replacement::draft(0x140947fd0, "content_140947fd0", fn_140947fd0 as *const ()), // sub_140947fd0
    Replacement::draft(0x140948010, "content_140948010", fn_140948010 as *const ()), // sub_140948010
    Replacement::draft(0x140948050, "content_140948050", fn_140948050 as *const ()), // sub_140948050
    Replacement::draft(0x1409482a0, "content_1409482a0", fn_1409482a0 as *const ()), // sub_1409482a0
    Replacement::draft(0x1409484b0, "content_1409484b0", fn_1409484b0 as *const ()), // sub_1409484b0
    Replacement::draft(0x140948700, "content_140948700", tpl_69838::<0x14094baa0, 0x1401e84a0> as *const ()), // sub_140948700
    Replacement::draft(0x140948920, "content_140948920", tpl_1201::<0x3, 0x4, 0x8, 0x14240aa00> as *const ()), // sub_140948920
    Replacement::draft_throws(0x1409489a0, "content_1409489a0", fn_1409489a0 as *const (), Throws::Caught), // sub_1409489a0
    Replacement::draft_throws(0x140948e70, "content_140948e70", fn_140948e70 as *const (), Throws::Caught), // sub_140948e70
    Replacement::draft_throws(0x140949340, "content_140949340", fn_140949340 as *const (), Throws::Caught), // sub_140949340
    Replacement::draft_throws(0x1409497d0, "content_1409497d0", fn_1409497d0 as *const (), Throws::Caught), // sub_1409497d0
    Replacement::draft_throws(0x140949c60, "content_140949c60", fn_140949c60 as *const (), Throws::Caught), // sub_140949c60
    Replacement::draft_throws(0x14094a0f0, "content_14094a0f0", fn_14094a0f0 as *const (), Throws::Caught), // sub_14094a0f0
    Replacement::draft_throws(0x14094a590, "content_14094a590", fn_14094a590 as *const (), Throws::Caught), // sub_14094a590
    Replacement::draft_throws(0x14094ac50, "content_14094ac50", fn_14094ac50 as *const (), Throws::Caught), // sub_14094ac50
    Replacement::draft_throws(0x14094b290, "content_14094b290", fn_14094b290 as *const (), Throws::Caught), // sub_14094b290
    Replacement::draft(0x14094b900, "content_14094b900", fn_14094b900 as *const ()), // sub_14094b900
    Replacement::draft_throws(0x14094bc20, "content_14094bc20", fn_14094bc20 as *const (), Throws::Caught), // sub_14094bc20
    Replacement::draft_throws(0x14094bcd0, "content_14094bcd0", fn_14094bcd0 as *const (), Throws::Caught), // sub_14094bcd0
    Replacement::draft_throws(0x14094bf40, "content_14094bf40", fn_14094bf40 as *const (), Throws::Caught), // sub_14094bf40
    Replacement::draft_throws(0x14094c000, "content_14094c000", fn_14094c000 as *const (), Throws::Caught), // sub_14094c000
    Replacement::draft_throws(0x14094c260, "content_14094c260", fn_14094c260 as *const (), Throws::Caught), // sub_14094c260
    Replacement::draft_throws(0x14094c320, "content_14094c320", fn_14094c320 as *const (), Throws::Caught), // sub_14094c320
    Replacement::draft_throws(0x14094c580, "content_14094c580", fn_14094c580 as *const (), Throws::Caught), // sub_14094c580
    Replacement::draft_throws(0x14094c810, "content_14094c810", fn_14094c810 as *const (), Throws::Caught), // sub_14094c810
    Replacement::draft_throws(0x14094cb60, "content_14094cb60", fn_14094cb60 as *const (), Throws::Caught), // sub_14094cb60
    Replacement::draft_throws(0x14094ce70, "content_14094ce70", fn_14094ce70 as *const (), Throws::Caught), // sub_14094ce70
    Replacement::draft_throws(0x14094d170, "content_14094d170", fn_14094d170 as *const (), Throws::Caught), // sub_14094d170
    Replacement::draft_throws(0x14094d740, "content_14094d740", fn_14094d740 as *const (), Throws::Caught), // sub_14094d740
    Replacement::draft_throws(0x14094dcd0, "content_14094dcd0", fn_14094dcd0 as *const (), Throws::Caught), // sub_14094dcd0
    Replacement::draft(0x14094e150, "content_14094e150", tpl_4761::<0x14329c5e0, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::AnimationGraphComponent::content::AnimationGraphComponent::vfunc_18
    Replacement::draft(0x14094e290, "content_14094e290", tpl_11316::<0x14329c598, 0x142409b50, 0x1401f22d0, 0x14240a730, 0x14240ab10, 0x14329c5b8> as *const ()), // content::AnimationSlaveComponent::content::AnimationSlaveComponent::vfunc_18
    Replacement::draft(0x14094e490, "content_14094e490", fn_14094e490 as *const ()), // content::AnimationSystemComponent::content::AnimationSystemComponent::vfunc_18
    Replacement::draft(0x14094e660, "content_14094e660", fn_14094e660 as *const ()), // content::AreaShapeComponent::content::AreaShapeComponent::vfunc_18
    Replacement::draft(0x14094e8c0, "content_14094e8c0", fn_14094e8c0 as *const ()), // content::AreaVertexComponent::content::AreaVertexComponent::vfunc_5
    Replacement::draft(0x14094e8d0, "content_14094e8d0", fn_14094e8d0 as *const ()), // content::AreaVertexComponent::content::AreaVertexComponent::vfunc_4
    Replacement::draft(0x14094e8f0, "content_14094e8f0", fn_14094e8f0 as *const ()), // content::AreaVertexComponent::content::AreaVertexComponent::vfunc_10
    Replacement::draft(0x14094e9b0, "content_14094e9b0", tpl_4774::<0x14329c488, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::AttachmentSocketComponent::content::AttachmentSocketComponent::vfunc_18
    Replacement::draft(0x14094eba0, "content_14094eba0", fn_14094eba0 as *const ()), // content::AudioSwitchesComponent::content::AudioSwitchesComponent::vfunc_20
    Replacement::draft(0x14094ec40, "content_14094ec40", fn_14094ec40 as *const ()), // content::BoneFoleyComponent::content::BoneFoleyComponent::vfunc_20
    Replacement::draft(0x14094ecd0, "content_14094ecd0", fn_14094ecd0 as *const ()), // content::BoxComponent::content::BoxComponent::vfunc_10
    Replacement::draft(0x14094ecf0, "content_14094ecf0", fn_14094ecf0 as *const ()), // content::BoxComponent::content::BoxComponent::vfunc_4
    Replacement::draft(0x14094ef00, "content_14094ef00", fn_14094ef00 as *const ()), // content::CharacterLookAtComponent::content::CharacterLookAtComponent::vfunc_20
    Replacement::draft(0x14094f0f0, "content_14094f0f0", tpl_1892::<0x14329c330, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::DebugTextComponent::content::DebugTextComponent::vfunc_18
    Replacement::draft(0x14094f1f0, "content_14094f1f0", fn_14094f1f0 as *const ()), // sub_14094f1f0
    Replacement::draft(0x14094f290, "content_14094f290", fn_14094f290 as *const ()), // content::DecalComponent::content::DecalComponent::vfunc_18
    Replacement::draft(0x14094f4f0, "content_14094f4f0", tpl_4774::<0x14329c420, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::DirectionalLightComponent::content::DirectionalLightComponent::vfunc_18
    Replacement::draft(0x14094f720, "content_14094f720", fn_14094f720 as *const ()), // content::FlareComponent::content::FlareComponent::vfunc_4
    Replacement::draft(0x14094f7d0, "content_14094f7d0", tpl_11330::<0x14329c210, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::FogVolumeComponent::content::FogVolumeComponent::vfunc_18
    Replacement::draft(0x14094f8f0, "content_14094f8f0", fn_14094f8f0 as *const ()), // content::FogBlockerComponent::content::FogBlockerComponent::vfunc_18
    Replacement::draft(0x14094fa30, "content_14094fa30", fn_14094fa30 as *const ()), // content::GfxGraphComponent::content::GfxGraphComponent::vfunc_18
    Replacement::draft(0x14094fcc0, "content_14094fcc0", tpl_6077::<0x14329d4f0, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::HeightfieldExportVolumeComponent::content::HeightfieldExportVolumeComponent::vfunc_18
    Replacement::draft(0x14094fdf0, "content_14094fdf0", fn_14094fdf0 as *const ()), // content::HeightfieldLayerComponent::content::HeightfieldLayerComponent::vfunc_18
    Replacement::draft(0x140950030, "content_140950030", fn_140950030 as *const ()), // content::IlluminationVolumeComponent::content::IlluminationVolumeComponent::vfunc_18
    Replacement::draft(0x140950250, "content_140950250", tpl_1044::<0x1434266b8> as *const ()), // content::LogicAnimationComponent::content::LogicAnimationComponent::vfunc_8
    Replacement::draft(0x140950290, "content_140950290", fn_140950290 as *const ()), // content::LogicAnimationComponent::content::LogicAnimationComponent::vfunc_18
    Replacement::draft(0x140950520, "content_140950520", tpl_11336::<0x14329d390, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::LuaScriptNameComponent::content::LuaScriptNameComponent::vfunc_18
    Replacement::draft(0x140950640, "content_140950640", fn_140950640 as *const ()), // content::MeshComponent::content::MeshComponent::vfunc_18
    Replacement::draft(0x1409508a0, "content_1409508a0", fn_1409508a0 as *const ()), // content::OrthographicViewComponent::content::OrthographicViewComponent::vfunc_10
    Replacement::draft(0x140950920, "content_140950920", fn_140950920 as *const ()), // content::OrthographicViewComponent::content::OrthographicViewComponent::vfunc_4
    Replacement::draft(0x140950930, "content_140950930", fn_140950930 as *const ()), // content::OrthographicViewComponent::content::OrthographicViewComponent::vfunc_5
    Replacement::draft(0x140950990, "content_140950990", fn_140950990 as *const ()), // content::ParticleSystemComponent::content::ParticleSystemComponent::vfunc_4
    Replacement::draft(0x1409509a0, "content_1409509a0", fn_1409509a0 as *const ()), // content::ParticleSystemComponent::content::ParticleSystemComponent::vfunc_18
    Replacement::draft(0x140950ab0, "content_140950ab0", fn_140950ab0 as *const ()), // content::PerspectiveViewComponent::content::PerspectiveViewComponent::vfunc_12
    Replacement::draft(0x140950ac0, "content_140950ac0", fn_140950ac0 as *const ()), // content::PerspectiveViewComponent::content::PerspectiveViewComponent::vfunc_13
    Replacement::draft(0x140950af0, "content_140950af0", tpl_586::<0x8> as *const ()), // content::SphereShapeComponent::content::SphereShapeComponent::vfunc_8
    Replacement::draft(0x140950b00, "content_140950b00", tpl_586::<0xc> as *const ()), // content::PerspectiveViewComponent::content::PerspectiveViewComponent::vfunc_15
    Replacement::draft(0x140950c10, "content_140950c10", fn_140950c10 as *const ()), // content::PlaneShapeComponent::content::PlaneShapeComponent::vfunc_10
    Replacement::draft(0x140950ce0, "content_140950ce0", tpl_500::<0x8> as *const ()), // content::PointLightComponent::content::PointLightComponent::vfunc_4
    Replacement::draft(0x140950cf0, "content_140950cf0", tpl_586::<0x14> as *const ()), // content::PointLightComponent::content::PointLightComponent::vfunc_8
    Replacement::draft(0x140950d00, "content_140950d00", fn_140950d00 as *const ()), // content::PointLightComponent::content::PointLightComponent::vfunc_18
    Replacement::draft(0x140950ed0, "content_140950ed0", fn_140950ed0 as *const ()), // content::PointOfInterestComponent::content::PointOfInterestComponent::vfunc_10
    Replacement::draft(0x140950fb0, "content_140950fb0", fn_140950fb0 as *const ()), // content::PointOfInterestComponent::content::PointOfInterestComponent::vfunc_18
    Replacement::draft(0x1409511b0, "content_1409511b0", fn_1409511b0 as *const ()), // content::RagdollControllerComponent::content::RagdollControllerComponent::vfunc_18
    Replacement::draft(0x1409512f0, "content_1409512f0", tpl_11330::<0x14329d0c8, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::RenderBoxComponent::content::RenderBoxComponent::vfunc_18
    Replacement::draft(0x140951410, "content_140951410", fn_140951410 as *const ()), // content::SDFVolumeComponent::content::SDFVolumeComponent::vfunc_18
    Replacement::draft(0x140951640, "content_140951640", tpl_11336::<0x14329cf50, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::ScatterPointsComponent::content::ScatterPointsComponent::vfunc_18
    Replacement::draft(0x1409517b0, "content_1409517b0", fn_1409517b0 as *const ()), // content::SkeletonComponent::content::SkeletonComponent::vfunc_18
    Replacement::draft(0x1409519b0, "content_1409519b0", fn_1409519b0 as *const ()), // content::SoundComponent::content::SoundComponent::vfunc_18
    Replacement::draft(0x140951b80, "content_140951b80", tpl_11336::<0x14329d028, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::SoundSpawnerComponent::content::SoundSpawnerComponent::vfunc_18
    Replacement::draft(0x140951c90, "kaim_140951c90", fn_140951c90 as *const ()), // Kaim::TriangleFromPosQuery::Kaim::TriangleFromPosQuery::vfunc_1
    Replacement::draft(0x140951ca0, "content_140951ca0", fn_140951ca0 as *const ()), // content::SphereShapeComponent::content::SphereShapeComponent::vfunc_9
    Replacement::draft(0x140951d20, "content_140951d20", tpl_11316::<0x14329cce0, 0x142409b50, 0x1401f22d0, 0x14240a730, 0x14240ab10, 0x14329cd00> as *const ()), // content::StreamingStateComponent::content::StreamingStateComponent::vfunc_18
    Replacement::draft(0x140951f10, "content_140951f10", tpl_11355::<0x8, 0x10> as *const ()), // content::TagComponent::content::TagComponent::vfunc_20
    Replacement::draft(0x140951f40, "content_140951f40", fn_140951f40 as *const ()), // content::TagComponent::content::TagComponent::vfunc_18
    Replacement::draft(0x1409520f0, "content_1409520f0", tpl_2003::<0x14329cd28, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::TransformComponent::content::TransformComponent::vfunc_18
    Replacement::draft(0x140952200, "content_140952200", tpl_11357::<0x14329cd40, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::TriggerComponent::content::TriggerComponent::vfunc_18
    Replacement::draft(0x1409529d0, "unknown_1409529d0", fn_1409529d0 as *const ()), // sub_1409529d0
    Replacement::draft(0x140952a20, "unknown_140952a20", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143280130> as *const ()), // sub_140952a20
    Replacement::draft(0x140952a80, "unknown_140952a80", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143280130> as *const ()), // sub_140952a80
    Replacement::draft_throws(0x140952b80, "unknown_140952b80", fn_140952b80 as *const (), Throws::Caught), // sub_140952b80
    Replacement::draft_throws(0x140952c10, "unknown_140952c10", tpl_6762::<0x14387df08, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140952c10
    Replacement::draft(0x140952cc0, "unknown_140952cc0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143286a40, 0x8> as *const ()), // sub_140952cc0
    Replacement::draft(0x140952d40, "unknown_140952d40", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143286a40> as *const ()), // sub_140952d40
    Replacement::draft(0x140952e40, "unknown_140952e40", fn_140952e40 as *const ()), // sub_140952e40
    Replacement::draft_throws(0x140952ee0, "unknown_140952ee0", fn_140952ee0 as *const (), Throws::Discard), // sub_140952ee0
    Replacement::draft(0x140952f60, "unknown_140952f60", tpl_11361::<0x14387e170, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140952f60
    Replacement::draft_throws(0x140952fe0, "unknown_140952fe0", fn_140952fe0 as *const (), Throws::Discard), // sub_140952fe0
    Replacement::draft_throws(0x140953070, "unknown_140953070", fn_140953070 as *const (), Throws::Caught), // sub_140953070
    Replacement::draft_throws(0x140953160, "unknown_140953160", fn_140953160 as *const (), Throws::Caught), // sub_140953160
    Replacement::draft_throws(0x140953250, "unknown_140953250", fn_140953250 as *const (), Throws::Caught), // sub_140953250
    Replacement::draft(0x140953340, "unknown_140953340", fn_140953340 as *const ()), // sub_140953340
    Replacement::draft(0x1409533b0, "unknown_1409533b0", tpl_9203::<0x14387df00, 0x14387e170, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_1409533b0
    Replacement::draft(0x140953420, "unknown_140953420", tpl_11361::<0x14387e170, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140953420
    Replacement::draft_throws(0x1409534a0, "unknown_1409534a0", tpl_9204::<0x14387df00, 0x14387e170, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_1409534a0
    Replacement::draft_throws(0x140953520, "unknown_140953520", fn_140953520 as *const (), Throws::Discard), // sub_140953520
    Replacement::draft(0x140953630, "unknown_140953630", tpl_6676::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143299e10, 0x8> as *const ()), // sub_140953630
    Replacement::draft(0x1409536c0, "unknown_1409536c0", tpl_8299::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143299e10, 0x4, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_1409536c0
    Replacement::draft(0x1409537d0, "unknown_1409537d0", fn_1409537d0 as *const ()), // sub_1409537d0
    Replacement::draft_throws(0x140953920, "unknown_140953920", fn_140953920 as *const (), Throws::Discard), // sub_140953920
    Replacement::draft(0x140953a70, "unknown_140953a70", fn_140953a70 as *const ()), // sub_140953a70
    Replacement::draft_throws(0x140953b60, "unknown_140953b60", fn_140953b60 as *const (), Throws::Discard), // sub_140953b60
    Replacement::draft(0x140953ca0, "unknown_140953ca0", fn_140953ca0 as *const ()), // sub_140953ca0
    Replacement::draft_throws(0x140953d40, "unknown_140953d40", fn_140953d40 as *const (), Throws::Caught), // sub_140953d40
    Replacement::draft_throws(0x140953e40, "unknown_140953e40", fn_140953e40 as *const (), Throws::Caught), // sub_140953e40
    Replacement::draft_throws(0x140953f40, "unknown_140953f40", fn_140953f40 as *const (), Throws::Caught), // sub_140953f40
    Replacement::draft(0x140954040, "unknown_140954040", fn_140954040 as *const ()), // sub_140954040
    Replacement::draft(0x140954110, "unknown_140954110", fn_140954110 as *const ()), // sub_140954110
    Replacement::draft(0x1409541d0, "unknown_1409541d0", fn_1409541d0 as *const ()), // sub_1409541d0
    Replacement::draft_throws(0x1409542a0, "unknown_1409542a0", fn_1409542a0 as *const (), Throws::Discard), // sub_1409542a0
    Replacement::draft_throws(0x140954360, "unknown_140954360", fn_140954360 as *const (), Throws::Discard), // sub_140954360
    Replacement::draft(0x140954470, "unknown_140954470", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14328f900, 0x8> as *const ()), // sub_140954470
    Replacement::draft(0x1409544f0, "unknown_1409544f0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328f900> as *const ()), // sub_1409544f0
    Replacement::draft(0x1409545f0, "unknown_1409545f0", tpl_11372::<0x14387e450, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140920ac0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409545f0
    Replacement::draft_throws(0x140954670, "unknown_140954670", fn_140954670 as *const (), Throws::Caught), // sub_140954670
    Replacement::draft(0x1409547f0, "unknown_1409547f0", tpl_11372::<0x14387e450, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140920e30, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409547f0
    Replacement::draft_throws(0x140954870, "unknown_140954870", fn_140954870 as *const (), Throws::Caught), // sub_140954870
    Replacement::draft_throws(0x1409549f0, "unknown_1409549f0", fn_1409549f0 as *const (), Throws::Caught), // sub_1409549f0
    Replacement::draft_throws(0x140954ac0, "unknown_140954ac0", fn_140954ac0 as *const (), Throws::Caught), // sub_140954ac0
    Replacement::draft_throws(0x140954b90, "unknown_140954b90", fn_140954b90 as *const (), Throws::Caught), // sub_140954b90
    Replacement::draft(0x140954c60, "unknown_140954c60", tpl_11372::<0x14387e450, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1409216e0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140954c60
    Replacement::draft(0x140954ce0, "unknown_140954ce0", tpl_6804::<0x14387dedc, 0x14387e450, 0x1401e9970, 0x1406e1480, 0x8, 0x140921870, 0x1401e9860> as *const ()), // sub_140954ce0
    Replacement::draft(0x140954d50, "unknown_140954d50", tpl_11372::<0x14387e450, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1409219e0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140954d50
    Replacement::draft_throws(0x140954dd0, "unknown_140954dd0", tpl_6805::<0x14387dedc, 0x14387e450, 0x1401e9770, 0x1406e1570, 0x8, 0x140921b30, 0x1401e9590> as *const (), Throws::Discard), // sub_140954dd0
    Replacement::draft_throws(0x140954e40, "unknown_140954e40", fn_140954e40 as *const (), Throws::Caught), // sub_140954e40
    Replacement::draft(0x140954fb0, "unknown_140954fb0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14327f7e8> as *const ()), // sub_140954fb0
    Replacement::draft(0x140955010, "unknown_140955010", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14327f7e8> as *const ()), // sub_140955010
    Replacement::draft(0x1409550a0, "unknown_1409550a0", fn_1409550a0 as *const ()), // sub_1409550a0
    Replacement::draft_throws(0x140955150, "unknown_140955150", tpl_11377::<0x14387e1c0, 0x14387e068, 0x1401e9fa0, 0x1406e02b0, 0x2, 0x1406d7a10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140955150
    Replacement::draft_throws(0x1409551f0, "unknown_1409551f0", tpl_11377::<0x14387e1c0, 0x14387e068, 0x1401e9b60, 0x1406e03a0, 0x2, 0x1406d7a10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409551f0
    Replacement::draft_throws(0x140955290, "unknown_140955290", fn_140955290 as *const (), Throws::Caught), // sub_140955290
    Replacement::draft(0x140955320, "unknown_140955320", fn_140955320 as *const ()), // sub_140955320
    Replacement::draft_throws(0x1409553c0, "unknown_1409553c0", tpl_11377::<0x14387e1c0, 0x14387e068, 0x1401e9770, 0x1406e1570, 0x2, 0x1406d7a10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x1401e9590> as *const (), Throws::Discard), // sub_1409553c0
    Replacement::draft_throws(0x140955460, "unknown_140955460", tpl_6823::<0x14387e068, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140955460
    Replacement::draft(0x140955510, "unknown_140955510", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143284e68, 0x8> as *const ()), // sub_140955510
    Replacement::draft(0x140955590, "unknown_140955590", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143284e68> as *const ()), // sub_140955590
    Replacement::draft(0x140955690, "unknown_140955690", tpl_6896::<0x14387de18, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408fc600, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140955690
    Replacement::draft_throws(0x140955700, "unknown_140955700", tpl_6897::<0x14387e2d0, 0x14387de18, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1409f9d10, 0x1401e9e70> as *const (), Throws::Discard), // sub_140955700
    Replacement::draft(0x140955770, "unknown_140955770", tpl_6896::<0x14387de18, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408fc970, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140955770
    Replacement::draft_throws(0x1409557e0, "unknown_1409557e0", tpl_6897::<0x14387e2d0, 0x14387de18, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1409f9e40, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409557e0
    Replacement::draft_throws(0x140955850, "unknown_140955850", fn_140955850 as *const (), Throws::Caught), // sub_140955850
    Replacement::draft_throws(0x140955920, "unknown_140955920", fn_140955920 as *const (), Throws::Caught), // sub_140955920
    Replacement::draft_throws(0x1409559f0, "unknown_1409559f0", fn_1409559f0 as *const (), Throws::Caught), // sub_1409559f0
    Replacement::draft(0x140955ac0, "unknown_140955ac0", tpl_6896::<0x14387de18, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408fd220, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140955ac0
    Replacement::draft(0x140955b30, "unknown_140955b30", tpl_6899::<0x14387e2d0, 0x14387de18, 0x1401e9970, 0x1406e1480, 0x8, 0x1408fd3b0, 0x1401e9860> as *const ()), // sub_140955b30
    Replacement::draft(0x140955b90, "unknown_140955b90", tpl_6896::<0x14387de18, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408fd520, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140955b90
    Replacement::draft_throws(0x140955c00, "unknown_140955c00", tpl_6900::<0x14387e2d0, 0x14387de18, 0x1401e9770, 0x1406e1570, 0x8, 0x1408fd670, 0x1401e9590> as *const (), Throws::Discard), // sub_140955c00
    Replacement::draft_throws(0x140955c60, "unknown_140955c60", tpl_11380::<0x14387de18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x14329ef20, 0x1409f9f70> as *const (), Throws::Discard), // sub_140955c60
    Replacement::draft(0x140955d30, "unknown_140955d30", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143291ef0> as *const ()), // sub_140955d30
    Replacement::draft(0x140955d90, "unknown_140955d90", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143291ef0> as *const ()), // sub_140955d90
    Replacement::draft_throws(0x140955e90, "unknown_140955e90", fn_140955e90 as *const (), Throws::Caught), // sub_140955e90
    Replacement::draft_throws(0x140955f20, "unknown_140955f20", tpl_6763::<0x14387e360, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140955f20
    Replacement::draft(0x140955fd0, "unknown_140955fd0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14329b2e8, 0x8> as *const ()), // sub_140955fd0
    Replacement::draft(0x140956050, "unknown_140956050", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329b2e8> as *const ()), // sub_140956050
    Replacement::draft(0x1409560e0, "unknown_1409560e0", fn_1409560e0 as *const ()), // sub_1409560e0
    Replacement::draft(0x140956160, "unknown_140956160", fn_140956160 as *const ()), // sub_140956160
    Replacement::draft_throws(0x140956280, "unknown_140956280", fn_140956280 as *const (), Throws::Caught), // sub_140956280
    Replacement::draft(0x1409563f0, "unknown_1409563f0", fn_1409563f0 as *const ()), // sub_1409563f0
    Replacement::draft_throws(0x140956500, "unknown_140956500", fn_140956500 as *const (), Throws::Discard), // sub_140956500
    Replacement::draft_throws(0x140956620, "unknown_140956620", fn_140956620 as *const (), Throws::Caught), // sub_140956620
    Replacement::draft_throws(0x1409568e0, "unknown_1409568e0", fn_1409568e0 as *const (), Throws::Caught), // sub_1409568e0
    Replacement::draft_throws(0x140956b70, "unknown_140956b70", fn_140956b70 as *const (), Throws::Caught), // sub_140956b70
    Replacement::draft(0x140956e00, "unknown_140956e00", fn_140956e00 as *const ()), // sub_140956e00
    Replacement::draft(0x140956f50, "unknown_140956f50", fn_140956f50 as *const ()), // sub_140956f50
    Replacement::draft(0x140957030, "unknown_140957030", fn_140957030 as *const ()), // sub_140957030
    Replacement::draft_throws(0x140957120, "unknown_140957120", fn_140957120 as *const (), Throws::Discard), // sub_140957120
    Replacement::draft_throws(0x140957220, "unknown_140957220", fn_140957220 as *const (), Throws::Discard), // sub_140957220
    Replacement::draft(0x1409573d0, "unknown_1409573d0", fn_1409573d0 as *const ()), // sub_1409573d0
    Replacement::draft(0x140957480, "unknown_140957480", fn_140957480 as *const ()), // sub_140957480
    Replacement::draft(0x140957540, "unknown_140957540", fn_140957540 as *const ()), // sub_140957540
    Replacement::draft(0x140957680, "unknown_140957680", fn_140957680 as *const ()), // sub_140957680
    Replacement::draft_throws(0x1409577a0, "unknown_1409577a0", fn_1409577a0 as *const (), Throws::Discard), // sub_1409577a0
    Replacement::draft_throws(0x140957900, "unknown_140957900", fn_140957900 as *const (), Throws::Discard), // sub_140957900
    Replacement::draft_throws(0x140957a10, "unknown_140957a10", fn_140957a10 as *const (), Throws::Discard), // sub_140957a10
    Replacement::draft_throws(0x140957b80, "unknown_140957b80", fn_140957b80 as *const (), Throws::Caught), // sub_140957b80
    Replacement::draft_throws(0x140957cc0, "unknown_140957cc0", fn_140957cc0 as *const (), Throws::Caught), // sub_140957cc0
    Replacement::draft_throws(0x140957e00, "unknown_140957e00", fn_140957e00 as *const (), Throws::Caught), // sub_140957e00
    Replacement::draft(0x140957f40, "unknown_140957f40", fn_140957f40 as *const ()), // sub_140957f40
    Replacement::draft(0x140958090, "unknown_140958090", fn_140958090 as *const ()), // sub_140958090
    Replacement::draft(0x140958240, "unknown_140958240", fn_140958240 as *const ()), // sub_140958240
    Replacement::draft_throws(0x140958380, "unknown_140958380", fn_140958380 as *const (), Throws::Discard), // sub_140958380
    Replacement::draft_throws(0x1409584c0, "unknown_1409584c0", fn_1409584c0 as *const (), Throws::Discard), // sub_1409584c0
    Replacement::draft(0x140958610, "unknown_140958610", fn_140958610 as *const ()), // sub_140958610
    Replacement::draft(0x1409586c0, "unknown_1409586c0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432953d0, 0x8> as *const ()), // sub_1409586c0
    Replacement::draft(0x140958740, "unknown_140958740", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432953d0> as *const ()), // sub_140958740
    Replacement::draft(0x1409587d0, "unknown_1409587d0", fn_1409587d0 as *const ()), // sub_1409587d0
    Replacement::draft(0x1409588a0, "unknown_1409588a0", tpl_11409::<0x14387ddc0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x1409037c0, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1409588a0
    Replacement::draft_throws(0x140958980, "unknown_140958980", fn_140958980 as *const (), Throws::Caught), // sub_140958980
    Replacement::draft(0x140958b70, "unknown_140958b70", tpl_11409::<0x14387ddc0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x140903b30, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140958b70
    Replacement::draft_throws(0x140958c50, "unknown_140958c50", fn_140958c50 as *const (), Throws::Caught), // sub_140958c50
    Replacement::draft_throws(0x140958e40, "unknown_140958e40", fn_140958e40 as *const (), Throws::Caught), // sub_140958e40
    Replacement::draft_throws(0x140958f10, "unknown_140958f10", fn_140958f10 as *const (), Throws::Caught), // sub_140958f10
    Replacement::draft_throws(0x140958fe0, "unknown_140958fe0", fn_140958fe0 as *const (), Throws::Caught), // sub_140958fe0
    Replacement::draft(0x1409590b0, "unknown_1409590b0", fn_1409590b0 as *const ()), // sub_1409590b0
    Replacement::draft(0x140959190, "unknown_140959190", fn_140959190 as *const ()), // sub_140959190
    Replacement::draft(0x140959260, "unknown_140959260", tpl_11409::<0x14387ddc0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x1409046e0, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140959260
    Replacement::draft_throws(0x140959340, "unknown_140959340", fn_140959340 as *const (), Throws::Discard), // sub_140959340
    Replacement::draft_throws(0x140959410, "unknown_140959410", fn_140959410 as *const (), Throws::Caught), // sub_140959410
    Replacement::draft(0x140959580, "unknown_140959580", tpl_6676::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143299280, 0x8> as *const ()), // sub_140959580
    Replacement::draft(0x140959610, "unknown_140959610", tpl_6677::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143299280, 0xfffffffffffffff8, 0x20> as *const ()), // sub_140959610
    Replacement::draft(0x140959720, "unknown_140959720", fn_140959720 as *const ()), // sub_140959720
    Replacement::draft_throws(0x1409597e0, "unknown_1409597e0", fn_1409597e0 as *const (), Throws::Discard), // sub_1409597e0
    Replacement::draft(0x1409598e0, "unknown_1409598e0", fn_1409598e0 as *const ()), // sub_1409598e0
    Replacement::draft_throws(0x1409599b0, "unknown_1409599b0", fn_1409599b0 as *const (), Throws::Discard), // sub_1409599b0
    Replacement::draft(0x140959a60, "unknown_140959a60", fn_140959a60 as *const ()), // sub_140959a60
    Replacement::draft_throws(0x140959ad0, "unknown_140959ad0", fn_140959ad0 as *const (), Throws::Caught), // sub_140959ad0
    Replacement::draft_throws(0x140959bd0, "unknown_140959bd0", fn_140959bd0 as *const (), Throws::Caught), // sub_140959bd0
    Replacement::draft_throws(0x140959cd0, "unknown_140959cd0", fn_140959cd0 as *const (), Throws::Caught), // sub_140959cd0
    Replacement::draft(0x140959dd0, "unknown_140959dd0", fn_140959dd0 as *const ()), // sub_140959dd0
    Replacement::draft(0x140959ea0, "unknown_140959ea0", fn_140959ea0 as *const ()), // sub_140959ea0
    Replacement::draft(0x140959f50, "unknown_140959f50", fn_140959f50 as *const ()), // sub_140959f50
    Replacement::draft_throws(0x14095a010, "unknown_14095a010", fn_14095a010 as *const (), Throws::Discard), // sub_14095a010
    Replacement::draft_throws(0x14095a0f0, "unknown_14095a0f0", fn_14095a0f0 as *const (), Throws::Discard), // sub_14095a0f0
    Replacement::draft(0x14095a210, "unknown_14095a210", fn_14095a210 as *const ()), // sub_14095a210
    Replacement::draft(0x14095a2a0, "unknown_14095a2a0", fn_14095a2a0 as *const ()), // sub_14095a2a0
    Replacement::draft_throws(0x14095a3c0, "unknown_14095a3c0", fn_14095a3c0 as *const (), Throws::Caught), // sub_14095a3c0
    Replacement::draft_throws(0x14095a450, "unknown_14095a450", tpl_6762::<0x14387de08, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14095a450
    Replacement::draft(0x14095a500, "unknown_14095a500", tpl_11427::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432946a0, 0x20, 0x28, 0x30> as *const ()), // sub_14095a500
    Replacement::draft(0x14095a5c0, "unknown_14095a5c0", tpl_11428::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432946a0, 0xfffffffffffffff4, 0x14, 0x34, 0xfffffffffffffffc, 0x4> as *const ()), // sub_14095a5c0
    Replacement::draft(0x14095a680, "unknown_14095a680", fn_14095a680 as *const ()), // sub_14095a680
    Replacement::draft(0x14095a770, "unknown_14095a770", fn_14095a770 as *const ()), // sub_14095a770
    Replacement::draft_throws(0x14095a8a0, "unknown_14095a8a0", fn_14095a8a0 as *const (), Throws::Discard), // sub_14095a8a0
    Replacement::draft(0x14095a9d0, "unknown_14095a9d0", fn_14095a9d0 as *const ()), // sub_14095a9d0
    Replacement::draft_throws(0x14095aad0, "unknown_14095aad0", fn_14095aad0 as *const (), Throws::Discard), // sub_14095aad0
    Replacement::draft_throws(0x14095abf0, "unknown_14095abf0", fn_14095abf0 as *const (), Throws::Caught), // sub_14095abf0
    Replacement::draft_throws(0x14095aca0, "unknown_14095aca0", fn_14095aca0 as *const (), Throws::Caught), // sub_14095aca0
    Replacement::draft_throws(0x14095ad50, "unknown_14095ad50", fn_14095ad50 as *const (), Throws::Caught), // sub_14095ad50
    Replacement::draft(0x14095af70, "unknown_14095af70", fn_14095af70 as *const ()), // sub_14095af70
    Replacement::draft(0x14095b080, "unknown_14095b080", fn_14095b080 as *const ()), // sub_14095b080
    Replacement::draft(0x14095b170, "unknown_14095b170", fn_14095b170 as *const ()), // sub_14095b170
    Replacement::draft_throws(0x14095b270, "unknown_14095b270", fn_14095b270 as *const (), Throws::Discard), // sub_14095b270
    Replacement::draft_throws(0x14095b360, "unknown_14095b360", fn_14095b360 as *const (), Throws::Discard), // sub_14095b360
    Replacement::draft(0x14095b460, "unknown_14095b460", tpl_11441::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14329a6f0, 0x8, 0x1c> as *const ()), // sub_14095b460
    Replacement::draft(0x14095b4f0, "unknown_14095b4f0", tpl_11442::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x14329a6f0, 0x5, 0x7, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_14095b4f0
    Replacement::draft(0x14095b610, "unknown_14095b610", tpl_11443::<0x14387e4f8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140a034e0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095b610
    Replacement::draft_throws(0x14095b690, "unknown_14095b690", fn_14095b690 as *const (), Throws::Discard), // sub_14095b690
    Replacement::draft(0x14095b740, "unknown_14095b740", fn_14095b740 as *const ()), // sub_14095b740
    Replacement::draft_throws(0x14095b7c0, "unknown_14095b7c0", fn_14095b7c0 as *const (), Throws::Discard), // sub_14095b7c0
    Replacement::draft_throws(0x14095b870, "unknown_14095b870", fn_14095b870 as *const (), Throws::Caught), // sub_14095b870
    Replacement::draft_throws(0x14095b920, "unknown_14095b920", fn_14095b920 as *const (), Throws::Caught), // sub_14095b920
    Replacement::draft_throws(0x14095b9d0, "unknown_14095b9d0", fn_14095b9d0 as *const (), Throws::Caught), // sub_14095b9d0
    Replacement::draft(0x14095ba80, "unknown_14095ba80", fn_14095ba80 as *const ()), // sub_14095ba80
    Replacement::draft(0x14095bb30, "unknown_14095bb30", fn_14095bb30 as *const ()), // sub_14095bb30
    Replacement::draft(0x14095bbf0, "unknown_14095bbf0", fn_14095bbf0 as *const ()), // sub_14095bbf0
    Replacement::draft_throws(0x14095bcc0, "unknown_14095bcc0", tpl_8772::<0x14387df24, 0x14387e4f8, 0x1401e9770, 0x1406e1570, 0x8, 0x1406deba0, 0x1401e9590> as *const (), Throws::Discard), // sub_14095bcc0
    Replacement::draft_throws(0x14095bd30, "unknown_14095bd30", fn_14095bd30 as *const (), Throws::Caught), // sub_14095bd30
    Replacement::draft(0x14095be70, "unknown_14095be70", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143285430, 0x8> as *const ()), // sub_14095be70
    Replacement::draft(0x14095bef0, "unknown_14095bef0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143285430> as *const ()), // sub_14095bef0
    Replacement::draft(0x14095bff0, "unknown_14095bff0", fn_14095bff0 as *const ()), // sub_14095bff0
    Replacement::draft_throws(0x14095c0d0, "unknown_14095c0d0", fn_14095c0d0 as *const (), Throws::Discard), // sub_14095c0d0
    Replacement::draft(0x14095c1c0, "unknown_14095c1c0", tpl_6827::<0x14387e348, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0x9, 0xc, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095c1c0
    Replacement::draft_throws(0x14095c260, "unknown_14095c260", fn_14095c260 as *const (), Throws::Discard), // sub_14095c260
    Replacement::draft_throws(0x14095c360, "unknown_14095c360", fn_14095c360 as *const (), Throws::Caught), // sub_14095c360
    Replacement::draft_throws(0x14095c4d0, "unknown_14095c4d0", fn_14095c4d0 as *const (), Throws::Caught), // sub_14095c4d0
    Replacement::draft_throws(0x14095c640, "unknown_14095c640", fn_14095c640 as *const (), Throws::Caught), // sub_14095c640
    Replacement::draft(0x14095c7b0, "unknown_14095c7b0", fn_14095c7b0 as *const ()), // sub_14095c7b0
    Replacement::draft(0x14095c830, "unknown_14095c830", tpl_6831::<0x14387e3c0, 0x14387e348, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x9, 0xc, 0xd8, 0x1401e9860> as *const ()), // sub_14095c830
    Replacement::draft(0x14095c8d0, "unknown_14095c8d0", tpl_6827::<0x14387e348, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0x9, 0xc, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095c8d0
    Replacement::draft_throws(0x14095c980, "unknown_14095c980", fn_14095c980 as *const (), Throws::Discard), // sub_14095c980
    Replacement::draft_throws(0x14095ca30, "unknown_14095ca30", fn_14095ca30 as *const (), Throws::Discard), // sub_14095ca30
    Replacement::draft(0x14095cbc0, "unknown_14095cbc0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143284138> as *const ()), // sub_14095cbc0
    Replacement::draft(0x14095cc20, "unknown_14095cc20", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143284138> as *const ()), // sub_14095cc20
    Replacement::draft_throws(0x14095cd20, "unknown_14095cd20", fn_14095cd20 as *const (), Throws::Caught), // sub_14095cd20
    Replacement::draft_throws(0x14095cdb0, "unknown_14095cdb0", tpl_6763::<0x14387e120, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14095cdb0
    Replacement::draft(0x14095ce60, "unknown_14095ce60", tpl_11457::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x143291380, 0x8> as *const ()), // sub_14095ce60
    Replacement::draft(0x14095cef0, "unknown_14095cef0", tpl_11458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143291380, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_14095cef0
    Replacement::draft(0x14095d010, "unknown_14095d010", tpl_11459::<0x14387e480, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095d010
    Replacement::draft_throws(0x14095d0c0, "unknown_14095d0c0", tpl_11460::<0x14387e378, 0x14387e480, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x8, 0x338, 0x38, 0x4, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_14095d0c0
    Replacement::draft(0x14095d170, "unknown_14095d170", tpl_11461::<0x14387e480, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095d170
    Replacement::draft_throws(0x14095d210, "unknown_14095d210", tpl_11462::<0x14387e378, 0x14387e480, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14095d210
    Replacement::draft_throws(0x14095d2b0, "unknown_14095d2b0", fn_14095d2b0 as *const (), Throws::Caught), // sub_14095d2b0
    Replacement::draft_throws(0x14095d4b0, "unknown_14095d4b0", fn_14095d4b0 as *const (), Throws::Caught), // sub_14095d4b0
    Replacement::draft_throws(0x14095d680, "unknown_14095d680", fn_14095d680 as *const (), Throws::Caught), // sub_14095d680
    Replacement::draft(0x14095d840, "unknown_14095d840", fn_14095d840 as *const ()), // sub_14095d840
    Replacement::draft(0x14095d8b0, "unknown_14095d8b0", tpl_11467::<0x14387e378, 0x14387e480, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1401e9860> as *const ()), // sub_14095d8b0
    Replacement::draft(0x14095d950, "unknown_14095d950", tpl_11461::<0x14387e480, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095d950
    Replacement::draft_throws(0x14095da00, "unknown_14095da00", tpl_11468::<0x14387e378, 0x14387e480, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x10, 0x1401e9590> as *const (), Throws::Discard), // sub_14095da00
    Replacement::draft_throws(0x14095da80, "unknown_14095da80", tpl_11469::<0x14387e480, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x4, 0x14329ecc8, 0x1413c41a0> as *const (), Throws::Discard), // sub_14095da80
    Replacement::draft(0x14095db80, "unknown_14095db80", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143281bb0> as *const ()), // sub_14095db80
    Replacement::draft(0x14095dbe0, "unknown_14095dbe0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143281bb0> as *const ()), // sub_14095dbe0
    Replacement::draft_throws(0x14095dce0, "unknown_14095dce0", fn_14095dce0 as *const (), Throws::Caught), // sub_14095dce0
    Replacement::draft_throws(0x14095dd70, "unknown_14095dd70", tpl_6762::<0x14387e440, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14095dd70
    Replacement::draft(0x14095de20, "unknown_14095de20", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432923c8, 0x8> as *const ()), // sub_14095de20
    Replacement::draft(0x14095deb0, "unknown_14095deb0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432923c8, 0xfffffffffffffff8, 0x18> as *const ()), // sub_14095deb0
    Replacement::draft(0x14095df60, "unknown_14095df60", fn_14095df60 as *const ()), // sub_14095df60
    Replacement::draft(0x14095e040, "unknown_14095e040", fn_14095e040 as *const ()), // sub_14095e040
    Replacement::draft_throws(0x14095e130, "unknown_14095e130", tpl_6937::<0x14387e0ac, 0x14387e190, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1409fa290, 0x1401e9e70> as *const (), Throws::Discard), // sub_14095e130
    Replacement::draft(0x14095e190, "unknown_14095e190", fn_14095e190 as *const ()), // sub_14095e190
    Replacement::draft_throws(0x14095e250, "unknown_14095e250", tpl_6937::<0x14387e0ac, 0x14387e190, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1409fa440, 0x1401e9a20> as *const (), Throws::Discard), // sub_14095e250
    Replacement::draft(0x14095e2b0, "unknown_14095e2b0", fn_14095e2b0 as *const ()), // sub_14095e2b0
    Replacement::draft_throws(0x14095e330, "unknown_14095e330", fn_14095e330 as *const (), Throws::Caught), // sub_14095e330
    Replacement::draft_throws(0x14095e3f0, "unknown_14095e3f0", fn_14095e3f0 as *const (), Throws::Caught), // sub_14095e3f0
    Replacement::draft_throws(0x14095e4b0, "unknown_14095e4b0", fn_14095e4b0 as *const (), Throws::Caught), // sub_14095e4b0
    Replacement::draft(0x14095e570, "unknown_14095e570", fn_14095e570 as *const ()), // sub_14095e570
    Replacement::draft(0x14095e620, "unknown_14095e620", fn_14095e620 as *const ()), // sub_14095e620
    Replacement::draft(0x14095e6d0, "unknown_14095e6d0", fn_14095e6d0 as *const ()), // sub_14095e6d0
    Replacement::draft_throws(0x14095e790, "unknown_14095e790", fn_14095e790 as *const (), Throws::Discard), // sub_14095e790
    Replacement::draft_throws(0x14095e830, "unknown_14095e830", tpl_11476::<0x14387e190, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x14329c4c8, 0x8, 0x14329ed78, 0x14329ed98, 0x1409fa630> as *const (), Throws::Discard), // sub_14095e830
    Replacement::draft(0x14095e920, "unknown_14095e920", fn_14095e920 as *const ()), // sub_14095e920
    Replacement::draft(0x14095ea10, "unknown_14095ea10", fn_14095ea10 as *const ()), // sub_14095ea10
    Replacement::draft(0x14095eb20, "unknown_14095eb20", tpl_7312::<0x14387de90, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095eb20
    Replacement::draft(0x14095eb90, "unknown_14095eb90", fn_14095eb90 as *const ()), // sub_14095eb90
    Replacement::draft_throws(0x14095ece0, "unknown_14095ece0", fn_14095ece0 as *const (), Throws::Discard), // sub_14095ece0
    Replacement::draft(0x14095ee30, "unknown_14095ee30", tpl_11480::<0x14387de90, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x30, 0x58, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14095ee30
    Replacement::draft_throws(0x14095eed0, "unknown_14095eed0", fn_14095eed0 as *const (), Throws::Discard), // sub_14095eed0
    Replacement::draft_throws(0x14095f020, "unknown_14095f020", fn_14095f020 as *const (), Throws::Caught), // sub_14095f020
    Replacement::draft_throws(0x14095f590, "unknown_14095f590", fn_14095f590 as *const (), Throws::Caught), // sub_14095f590
    Replacement::draft_throws(0x14095fa70, "unknown_14095fa70", fn_14095fa70 as *const (), Throws::Caught), // sub_14095fa70
    Replacement::draft(0x14095ff30, "unknown_14095ff30", fn_14095ff30 as *const ()), // sub_14095ff30
    Replacement::draft(0x140960010, "unknown_140960010", tpl_11485::<0x14387dd88, 0x14387de90, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x30, 0x58, 0x1401e9860> as *const ()), // sub_140960010
    Replacement::draft(0x1409600b0, "unknown_1409600b0", tpl_11480::<0x14387de90, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x30, 0x58, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409600b0
    Replacement::draft_throws(0x140960160, "unknown_140960160", fn_140960160 as *const (), Throws::Discard), // sub_140960160
    Replacement::draft_throws(0x1409601f0, "unknown_1409601f0", fn_1409601f0 as *const (), Throws::Discard), // sub_1409601f0
    Replacement::draft(0x140960350, "unknown_140960350", fn_140960350 as *const ()), // sub_140960350
    Replacement::draft(0x140960410, "unknown_140960410", fn_140960410 as *const ()), // sub_140960410
    Replacement::draft(0x1409604d0, "unknown_1409604d0", fn_1409604d0 as *const ()), // sub_1409604d0
    Replacement::draft(0x140960730, "unknown_140960730", fn_140960730 as *const ()), // sub_140960730
    Replacement::draft_throws(0x1409609f0, "unknown_1409609f0", fn_1409609f0 as *const (), Throws::Caught), // sub_1409609f0
    Replacement::draft_throws(0x140960be0, "unknown_140960be0", fn_140960be0 as *const (), Throws::Caught), // sub_140960be0
    Replacement::draft_throws(0x140960eb0, "unknown_140960eb0", fn_140960eb0 as *const (), Throws::Caught), // sub_140960eb0
    Replacement::draft(0x140961180, "unknown_140961180", fn_140961180 as *const ()), // sub_140961180
    Replacement::draft_throws(0x140961500, "unknown_140961500", fn_140961500 as *const (), Throws::Caught), // sub_140961500
    Replacement::draft_throws(0x1409619f0, "unknown_1409619f0", fn_1409619f0 as *const (), Throws::Caught), // sub_1409619f0
    Replacement::draft_throws(0x140961eb0, "unknown_140961eb0", fn_140961eb0 as *const (), Throws::Caught), // sub_140961eb0
    Replacement::draft(0x140962300, "unknown_140962300", fn_140962300 as *const ()), // sub_140962300
    Replacement::draft(0x1409624c0, "unknown_1409624c0", fn_1409624c0 as *const ()), // sub_1409624c0
    Replacement::draft(0x1409626c0, "unknown_1409626c0", fn_1409626c0 as *const ()), // sub_1409626c0
    Replacement::draft_throws(0x1409628c0, "unknown_1409628c0", fn_1409628c0 as *const (), Throws::Caught), // sub_1409628c0
    Replacement::draft_throws(0x140962a90, "unknown_140962a90", fn_140962a90 as *const (), Throws::Caught), // sub_140962a90
    Replacement::draft(0x140962c80, "unknown_140962c80", tpl_11504::<0x48, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc, 0x14240ab10> as *const ()), // sub_140962c80
    Replacement::draft(0x140962d30, "unknown_140962d30", tpl_11457::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x143296cf8, 0x8> as *const ()), // sub_140962d30
    Replacement::draft(0x140962dc0, "unknown_140962dc0", tpl_11458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143296cf8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140962dc0
    Replacement::draft(0x140962ee0, "unknown_140962ee0", tpl_11505::<0x14387dee8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140962ee0
    Replacement::draft_throws(0x140962f80, "unknown_140962f80", tpl_11506::<0x14387ddc8, 0x14387dee8, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x328, 0x338, 0x38, 0x4, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140962f80
    Replacement::draft(0x140963040, "unknown_140963040", tpl_11507::<0x14387dee8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140963040
    Replacement::draft_throws(0x1409630e0, "unknown_1409630e0", tpl_11508::<0x14387ddc8, 0x14387dee8, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409630e0
    Replacement::draft_throws(0x140963180, "unknown_140963180", fn_140963180 as *const (), Throws::Caught), // sub_140963180
    Replacement::draft_throws(0x140963380, "unknown_140963380", fn_140963380 as *const (), Throws::Caught), // sub_140963380
    Replacement::draft_throws(0x140963550, "unknown_140963550", fn_140963550 as *const (), Throws::Caught), // sub_140963550
    Replacement::draft(0x140963720, "unknown_140963720", tpl_70013::<0x14387dee8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140963720
    Replacement::draft(0x1409637a0, "unknown_1409637a0", tpl_11513::<0x14387ddc8, 0x14387dee8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1401e9860> as *const ()), // sub_1409637a0
    Replacement::draft(0x140963840, "unknown_140963840", tpl_11507::<0x14387dee8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140963840
    Replacement::draft_throws(0x1409638f0, "unknown_1409638f0", tpl_70014::<0x14387ddc8, 0x14387dee8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x10, 0x1401e9590> as *const (), Throws::Discard), // sub_1409638f0
    Replacement::draft_throws(0x140963970, "unknown_140963970", tpl_11515::<0x14387dee8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x4, 0x14329e980, 0x1413c41a0> as *const (), Throws::Discard), // sub_140963970
    Replacement::draft(0x140963a70, "unknown_140963a70", tpl_11457::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x14311f060, 0x8> as *const ()), // sub_140963a70
    Replacement::draft(0x140963b00, "unknown_140963b00", tpl_11458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14311f060, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140963b00
    Replacement::draft(0x140963bb0, "unknown_140963bb0", fn_140963bb0 as *const ()), // sub_140963bb0
    Replacement::draft(0x140963d60, "unknown_140963d60", fn_140963d60 as *const ()), // sub_140963d60
    Replacement::draft_throws(0x140963f50, "unknown_140963f50", fn_140963f50 as *const (), Throws::Discard), // sub_140963f50
    Replacement::draft(0x140964160, "unknown_140964160", fn_140964160 as *const ()), // sub_140964160
    Replacement::draft_throws(0x140964350, "unknown_140964350", fn_140964350 as *const (), Throws::Discard), // sub_140964350
    Replacement::draft_throws(0x140964540, "unknown_140964540", fn_140964540 as *const (), Throws::Caught), // sub_140964540
    Replacement::draft_throws(0x140964b60, "unknown_140964b60", fn_140964b60 as *const (), Throws::Caught), // sub_140964b60
    Replacement::draft_throws(0x1409650a0, "unknown_1409650a0", fn_1409650a0 as *const (), Throws::Caught), // sub_1409650a0
    Replacement::draft(0x140965590, "unknown_140965590", fn_140965590 as *const ()), // sub_140965590
    Replacement::draft(0x140965750, "unknown_140965750", fn_140965750 as *const ()), // sub_140965750
    Replacement::draft(0x140965940, "unknown_140965940", fn_140965940 as *const ()), // sub_140965940
    Replacement::draft_throws(0x140965b30, "unknown_140965b30", fn_140965b30 as *const (), Throws::Discard), // sub_140965b30
    Replacement::draft_throws(0x140965cf0, "unknown_140965cf0", fn_140965cf0 as *const (), Throws::Discard), // sub_140965cf0
    Replacement::draft_throws(0x140965df0, "unknown_140965df0", fn_140965df0 as *const (), Throws::Discard), // sub_140965df0
    Replacement::draft(0x140965e60, "unknown_140965e60", tpl_11529::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x14311ecf8> as *const ()), // sub_140965e60
    Replacement::draft(0x140965ef0, "unknown_140965ef0", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14311ecf8, 0x18, 0x20> as *const ()), // sub_140965ef0
    Replacement::draft(0x140965f90, "unknown_140965f90", fn_140965f90 as *const ()), // sub_140965f90
    Replacement::draft(0x140966030, "unknown_140966030", fn_140966030 as *const ()), // sub_140966030
    Replacement::draft_throws(0x1409661c0, "unknown_1409661c0", fn_1409661c0 as *const (), Throws::Discard), // sub_1409661c0
    Replacement::draft(0x140966330, "unknown_140966330", fn_140966330 as *const ()), // sub_140966330
    Replacement::draft_throws(0x140966430, "unknown_140966430", fn_140966430 as *const (), Throws::Discard), // sub_140966430
    Replacement::draft_throws(0x1409665d0, "unknown_1409665d0", fn_1409665d0 as *const (), Throws::Caught), // sub_1409665d0
    Replacement::draft_throws(0x1409667c0, "unknown_1409667c0", fn_1409667c0 as *const (), Throws::Caught), // sub_1409667c0
    Replacement::draft_throws(0x1409669b0, "unknown_1409669b0", fn_1409669b0 as *const (), Throws::Caught), // sub_1409669b0
    Replacement::draft(0x140966ba0, "unknown_140966ba0", fn_140966ba0 as *const ()), // sub_140966ba0
    Replacement::draft(0x140966c60, "unknown_140966c60", fn_140966c60 as *const ()), // sub_140966c60
    Replacement::draft(0x140966d50, "unknown_140966d50", fn_140966d50 as *const ()), // sub_140966d50
    Replacement::draft_throws(0x140966e50, "unknown_140966e50", fn_140966e50 as *const (), Throws::Discard), // sub_140966e50
    Replacement::draft_throws(0x140966f70, "unknown_140966f70", fn_140966f70 as *const (), Throws::Discard), // sub_140966f70
    Replacement::draft(0x140967180, "unknown_140967180", fn_140967180 as *const ()), // sub_140967180
    Replacement::draft(0x140967250, "unknown_140967250", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143299128> as *const ()), // sub_140967250
    Replacement::draft(0x1409672b0, "unknown_1409672b0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143299128> as *const ()), // sub_1409672b0
    Replacement::draft_throws(0x1409673b0, "unknown_1409673b0", fn_1409673b0 as *const (), Throws::Caught), // sub_1409673b0
    Replacement::draft_throws(0x140967440, "unknown_140967440", tpl_6763::<0x14387dfd0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140967440
    Replacement::draft(0x1409674f0, "unknown_1409674f0", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143159a80> as *const ()), // sub_1409674f0
    Replacement::draft(0x140967580, "unknown_140967580", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143159a80, 0x18> as *const ()), // sub_140967580
    Replacement::draft(0x140967630, "unknown_140967630", fn_140967630 as *const ()), // sub_140967630
    Replacement::draft(0x140967740, "unknown_140967740", fn_140967740 as *const ()), // sub_140967740
    Replacement::draft_throws(0x140967920, "unknown_140967920", fn_140967920 as *const (), Throws::Discard), // sub_140967920
    Replacement::draft(0x140967b00, "unknown_140967b00", fn_140967b00 as *const ()), // sub_140967b00
    Replacement::draft_throws(0x140967c60, "unknown_140967c60", fn_140967c60 as *const (), Throws::Discard), // sub_140967c60
    Replacement::draft_throws(0x140967e70, "unknown_140967e70", fn_140967e70 as *const (), Throws::Caught), // sub_140967e70
    Replacement::draft_throws(0x140968060, "unknown_140968060", fn_140968060 as *const (), Throws::Caught), // sub_140968060
    Replacement::draft_throws(0x140968250, "unknown_140968250", fn_140968250 as *const (), Throws::Caught), // sub_140968250
    Replacement::draft(0x140968440, "unknown_140968440", fn_140968440 as *const ()), // sub_140968440
    Replacement::draft(0x140968560, "unknown_140968560", fn_140968560 as *const ()), // sub_140968560
    Replacement::draft(0x1409686c0, "unknown_1409686c0", fn_1409686c0 as *const ()), // sub_1409686c0
    Replacement::draft_throws(0x140968830, "unknown_140968830", fn_140968830 as *const (), Throws::Discard), // sub_140968830
    Replacement::draft_throws(0x1409689c0, "unknown_1409689c0", fn_1409689c0 as *const (), Throws::Discard), // sub_1409689c0
    Replacement::draft(0x140968bd0, "unknown_140968bd0", fn_140968bd0 as *const ()), // sub_140968bd0
    Replacement::draft(0x140968cb0, "unknown_140968cb0", tpl_6676::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14328aad0, 0x8> as *const ()), // sub_140968cb0
    Replacement::draft(0x140968d40, "unknown_140968d40", tpl_6677::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328aad0, 0xfffffffffffffff8, 0x20> as *const ()), // sub_140968d40
    Replacement::draft(0x140968de0, "unknown_140968de0", fn_140968de0 as *const ()), // sub_140968de0
    Replacement::draft(0x140968ec0, "unknown_140968ec0", fn_140968ec0 as *const ()), // sub_140968ec0
    Replacement::draft_throws(0x140969030, "unknown_140969030", fn_140969030 as *const (), Throws::Discard), // sub_140969030
    Replacement::draft_throws(0x140969130, "unknown_140969130", fn_140969130 as *const (), Throws::Discard), // sub_140969130
    Replacement::draft_throws(0x140969260, "unknown_140969260", fn_140969260 as *const (), Throws::Discard), // sub_140969260
    Replacement::draft(0x140969370, "unknown_140969370", fn_140969370 as *const ()), // sub_140969370
    Replacement::draft_throws(0x140969460, "unknown_140969460", fn_140969460 as *const (), Throws::Caught), // sub_140969460
    Replacement::draft_throws(0x1409695b0, "unknown_1409695b0", fn_1409695b0 as *const (), Throws::Caught), // sub_1409695b0
    Replacement::draft_throws(0x140969700, "unknown_140969700", fn_140969700 as *const (), Throws::Caught), // sub_140969700
    Replacement::draft(0x140969850, "unknown_140969850", fn_140969850 as *const ()), // sub_140969850
    Replacement::draft(0x140969960, "unknown_140969960", fn_140969960 as *const ()), // sub_140969960
    Replacement::draft(0x140969a80, "unknown_140969a80", fn_140969a80 as *const ()), // sub_140969a80
    Replacement::draft_throws(0x140969bb0, "unknown_140969bb0", fn_140969bb0 as *const (), Throws::Discard), // sub_140969bb0
    Replacement::draft_throws(0x140969ce0, "unknown_140969ce0", fn_140969ce0 as *const (), Throws::Discard), // sub_140969ce0
    Replacement::draft(0x140969e50, "unknown_140969e50", tpl_70062::<0x8, 0x10, 0x1426cdcb0, 0x140a0b370> as *const ()), // sub_140969e50
    Replacement::draft(0x140969ed0, "unknown_140969ed0", tpl_70062::<0x8, 0x10, 0x14329fff0, 0x140a0b380> as *const ()), // sub_140969ed0
    Replacement::draft(0x140969f50, "unknown_140969f50", fn_140969f50 as *const ()), // sub_140969f50
    Replacement::draft(0x14096a030, "unknown_14096a030", fn_14096a030 as *const ()), // sub_14096a030
    Replacement::draft(0x14096a140, "unknown_14096a140", fn_14096a140 as *const ()), // sub_14096a140
    Replacement::draft(0x14096a260, "unknown_14096a260", fn_14096a260 as *const ()), // sub_14096a260
    Replacement::draft_throws(0x14096a480, "unknown_14096a480", fn_14096a480 as *const (), Throws::Discard), // sub_14096a480
    Replacement::draft(0x14096a660, "unknown_14096a660", fn_14096a660 as *const ()), // sub_14096a660
    Replacement::draft_throws(0x14096a7e0, "unknown_14096a7e0", fn_14096a7e0 as *const (), Throws::Discard), // sub_14096a7e0
    Replacement::draft(0x14096a9f0, "unknown_14096a9f0", fn_14096a9f0 as *const ()), // sub_14096a9f0
    Replacement::draft_throws(0x14096aa90, "unknown_14096aa90", fn_14096aa90 as *const (), Throws::Caught), // sub_14096aa90
    Replacement::draft_throws(0x14096adb0, "unknown_14096adb0", fn_14096adb0 as *const (), Throws::Caught), // sub_14096adb0
    Replacement::draft_throws(0x14096b0a0, "unknown_14096b0a0", fn_14096b0a0 as *const (), Throws::Caught), // sub_14096b0a0
    Replacement::draft(0x14096b390, "unknown_14096b390", fn_14096b390 as *const ()), // sub_14096b390
    Replacement::draft(0x14096b500, "unknown_14096b500", fn_14096b500 as *const ()), // sub_14096b500
    Replacement::draft(0x14096b670, "unknown_14096b670", fn_14096b670 as *const ()), // sub_14096b670
    Replacement::draft_throws(0x14096b7f0, "unknown_14096b7f0", fn_14096b7f0 as *const (), Throws::Discard), // sub_14096b7f0
    Replacement::draft_throws(0x14096b970, "unknown_14096b970", fn_14096b970 as *const (), Throws::Discard), // sub_14096b970
    Replacement::draft(0x14096bb30, "unknown_14096bb30", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143282720> as *const ()), // sub_14096bb30
    Replacement::draft(0x14096bb90, "unknown_14096bb90", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143282720> as *const ()), // sub_14096bb90
    Replacement::draft_throws(0x14096bc90, "unknown_14096bc90", fn_14096bc90 as *const (), Throws::Caught), // sub_14096bc90
    Replacement::draft_throws(0x14096bd20, "unknown_14096bd20", tpl_6763::<0x14387e3a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14096bd20
    Replacement::draft(0x14096bdd0, "unknown_14096bdd0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14328e6d0, 0x8> as *const ()), // sub_14096bdd0
    Replacement::draft(0x14096be50, "unknown_14096be50", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328e6d0> as *const ()), // sub_14096be50
    Replacement::draft(0x14096bee0, "unknown_14096bee0", tpl_7312::<0x14387e140, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096bee0
    Replacement::draft(0x14096bf50, "unknown_14096bf50", tpl_11575::<0x14387e140, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x1426d4580, 0x1, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096bf50
    Replacement::draft_throws(0x14096c030, "unknown_14096c030", fn_14096c030 as *const (), Throws::Discard), // sub_14096c030
    Replacement::draft(0x14096c0f0, "unknown_14096c0f0", tpl_11577::<0x14387e140, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096c0f0
    Replacement::draft_throws(0x14096c180, "unknown_14096c180", fn_14096c180 as *const (), Throws::Discard), // sub_14096c180
    Replacement::draft_throws(0x14096c260, "unknown_14096c260", fn_14096c260 as *const (), Throws::Caught), // sub_14096c260
    Replacement::draft_throws(0x14096c390, "unknown_14096c390", fn_14096c390 as *const (), Throws::Caught), // sub_14096c390
    Replacement::draft_throws(0x14096c4c0, "unknown_14096c4c0", fn_14096c4c0 as *const (), Throws::Caught), // sub_14096c4c0
    Replacement::draft(0x14096c5f0, "unknown_14096c5f0", fn_14096c5f0 as *const ()), // sub_14096c5f0
    Replacement::draft(0x14096c670, "unknown_14096c670", tpl_11581::<0x14387dfc4, 0x14387e140, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x1401e9860> as *const ()), // sub_14096c670
    Replacement::draft(0x14096c700, "unknown_14096c700", tpl_11577::<0x14387e140, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096c700
    Replacement::draft_throws(0x14096c790, "unknown_14096c790", tpl_11582::<0x14387dfc4, 0x14387e140, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0xc, 0x1401e9590> as *const (), Throws::Discard), // sub_14096c790
    Replacement::draft_throws(0x14096c830, "unknown_14096c830", fn_14096c830 as *const (), Throws::Discard), // sub_14096c830
    Replacement::draft(0x14096c980, "unknown_14096c980", tpl_65793::<0x143426a68, 0x14329fdd0, 0x8, 0x14329fe08, 0x14329fd78, 0x14020f270, 0x143426a08, 0x14329fd90, 0xc, 0x14329fee8, 0x14329ff18> as *const ()), // sub_14096c980
    Replacement::draft(0x14096ca20, "unknown_14096ca20", tpl_11584::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14328f7a8, 0x20, 0x28, 0xb> as *const ()), // sub_14096ca20
    Replacement::draft(0x14096cad0, "unknown_14096cad0", tpl_11585::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328f7a8, 0xfffffffffffffff4, 0x14, 0xfffffffffffffffc, 0x4, 0x30> as *const ()), // sub_14096cad0
    Replacement::draft(0x14096cc00, "unknown_14096cc00", tpl_9966::<0x14387ded0, 0x14243b0b0, 0x1406e01b0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096cc00
    Replacement::draft_throws(0x14096ccc0, "unknown_14096ccc0", tpl_9629::<0x14387e3d8, 0x14387ded0, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14096ccc0
    Replacement::draft(0x14096cd70, "unknown_14096cd70", tpl_9969::<0x14387ded0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096cd70
    Replacement::draft_throws(0x14096cdf0, "unknown_14096cdf0", tpl_9630::<0x14387e3d8, 0x14387ded0, 0x1401e9b60, 0x1406e03a0, 0x328, 0x8, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14096cdf0
    Replacement::draft_throws(0x14096cea0, "unknown_14096cea0", fn_14096cea0 as *const (), Throws::Caught), // sub_14096cea0
    Replacement::draft_throws(0x14096d110, "unknown_14096d110", fn_14096d110 as *const (), Throws::Caught), // sub_14096d110
    Replacement::draft_throws(0x14096d340, "unknown_14096d340", fn_14096d340 as *const (), Throws::Caught), // sub_14096d340
    Replacement::draft(0x14096d560, "unknown_14096d560", tpl_11589::<0x14387ded0, 0x14243b0b0, 0x1406e01b0, 0x1426cdcb0, 0x8, 0x142409b50, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096d560
    Replacement::draft(0x14096d5f0, "unknown_14096d5f0", tpl_7856::<0x14387e3d8, 0x14387ded0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_14096d5f0
    Replacement::draft(0x14096d660, "unknown_14096d660", tpl_9969::<0x14387ded0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096d660
    Replacement::draft_throws(0x14096d6e0, "unknown_14096d6e0", tpl_9971::<0x14387e3d8, 0x14387ded0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_14096d6e0
    Replacement::draft_throws(0x14096d750, "unknown_14096d750", tpl_11590::<0x14387ded0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1426cdcb0, 0x8, 0x14329fe40, 0x14329ff58, 0x14329fea0, 0x1402c4730> as *const (), Throws::Discard), // sub_14096d750
    Replacement::draft(0x14096d850, "unknown_14096d850", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143296888, 0x8> as *const ()), // sub_14096d850
    Replacement::draft(0x14096d8d0, "unknown_14096d8d0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143296888> as *const ()), // sub_14096d8d0
    Replacement::draft(0x14096d9d0, "unknown_14096d9d0", fn_14096d9d0 as *const ()), // sub_14096d9d0
    Replacement::draft_throws(0x14096dab0, "unknown_14096dab0", fn_14096dab0 as *const (), Throws::Discard), // sub_14096dab0
    Replacement::draft(0x14096db70, "unknown_14096db70", tpl_11593::<0x14387df40, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096db70
    Replacement::draft_throws(0x14096dc00, "unknown_14096dc00", fn_14096dc00 as *const (), Throws::Discard), // sub_14096dc00
    Replacement::draft_throws(0x14096dce0, "unknown_14096dce0", fn_14096dce0 as *const (), Throws::Caught), // sub_14096dce0
    Replacement::draft_throws(0x14096de20, "unknown_14096de20", fn_14096de20 as *const (), Throws::Caught), // sub_14096de20
    Replacement::draft_throws(0x14096df60, "unknown_14096df60", fn_14096df60 as *const (), Throws::Caught), // sub_14096df60
    Replacement::draft(0x14096e0a0, "unknown_14096e0a0", fn_14096e0a0 as *const ()), // sub_14096e0a0
    Replacement::draft(0x14096e120, "unknown_14096e120", tpl_8026::<0x14387e418, 0x14387df40, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x1401e9860> as *const ()), // sub_14096e120
    Replacement::draft(0x14096e1b0, "unknown_14096e1b0", tpl_11593::<0x14387df40, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096e1b0
    Replacement::draft_throws(0x14096e240, "unknown_14096e240", tpl_8028::<0x14387e418, 0x14387df40, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0xc, 0x1401e9590> as *const (), Throws::Discard), // sub_14096e240
    Replacement::draft_throws(0x14096e2e0, "unknown_14096e2e0", fn_14096e2e0 as *const (), Throws::Discard), // sub_14096e2e0
    Replacement::draft(0x14096e430, "unknown_14096e430", fn_14096e430 as *const ()), // sub_14096e430
    Replacement::draft(0x14096e4d0, "unknown_14096e4d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14329bd88> as *const ()), // sub_14096e4d0
    Replacement::draft(0x14096e530, "unknown_14096e530", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329bd88> as *const ()), // sub_14096e530
    Replacement::draft_throws(0x14096e630, "unknown_14096e630", fn_14096e630 as *const (), Throws::Caught), // sub_14096e630
    Replacement::draft_throws(0x14096e6c0, "unknown_14096e6c0", tpl_6763::<0x14387e3d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14096e6c0
    Replacement::draft(0x14096e770, "unknown_14096e770", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432876a0> as *const ()), // sub_14096e770
    Replacement::draft(0x14096e7d0, "unknown_14096e7d0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432876a0> as *const ()), // sub_14096e7d0
    Replacement::draft_throws(0x14096e8d0, "unknown_14096e8d0", fn_14096e8d0 as *const (), Throws::Caught), // sub_14096e8d0
    Replacement::draft_throws(0x14096e960, "unknown_14096e960", tpl_6763::<0x14387e518, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14096e960
    Replacement::draft(0x14096ea10, "unknown_14096ea10", fn_14096ea10 as *const ()), // sub_14096ea10
    Replacement::draft(0x14096eaf0, "unknown_14096eaf0", fn_14096eaf0 as *const ()), // sub_14096eaf0
    Replacement::draft(0x14096ec00, "unknown_14096ec00", tpl_7312::<0x14387de80, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14096ec00
    Replacement::draft(0x14096ec70, "unknown_14096ec70", fn_14096ec70 as *const ()), // sub_14096ec70
    Replacement::draft_throws(0x14096eed0, "unknown_14096eed0", fn_14096eed0 as *const (), Throws::Discard), // sub_14096eed0
    Replacement::draft(0x14096f140, "unknown_14096f140", fn_14096f140 as *const ()), // sub_14096f140
    Replacement::draft_throws(0x14096f2c0, "unknown_14096f2c0", fn_14096f2c0 as *const (), Throws::Discard), // sub_14096f2c0
    Replacement::draft(0x14096f530, "unknown_14096f530", fn_14096f530 as *const ()), // sub_14096f530
    Replacement::draft_throws(0x14096f610, "unknown_14096f610", fn_14096f610 as *const (), Throws::Caught), // sub_14096f610
    Replacement::draft_throws(0x14096fbb0, "unknown_14096fbb0", fn_14096fbb0 as *const (), Throws::Caught), // sub_14096fbb0
    Replacement::draft_throws(0x1409700f0, "unknown_1409700f0", fn_1409700f0 as *const (), Throws::Caught), // sub_1409700f0
    Replacement::draft(0x140970610, "unknown_140970610", fn_140970610 as *const ()), // sub_140970610
    Replacement::draft(0x1409706f0, "unknown_1409706f0", fn_1409706f0 as *const ()), // sub_1409706f0
    Replacement::draft(0x140970880, "unknown_140970880", fn_140970880 as *const ()), // sub_140970880
    Replacement::draft_throws(0x140970a20, "unknown_140970a20", fn_140970a20 as *const (), Throws::Discard), // sub_140970a20
    Replacement::draft_throws(0x140970b80, "unknown_140970b80", fn_140970b80 as *const (), Throws::Discard), // sub_140970b80
    Replacement::draft_throws(0x140970e90, "unknown_140970e90", fn_140970e90 as *const (), Throws::Discard), // sub_140970e90
    Replacement::draft(0x140971000, "unknown_140971000", fn_140971000 as *const ()), // sub_140971000
    Replacement::draft(0x1409710a0, "unknown_1409710a0", fn_1409710a0 as *const ()), // sub_1409710a0
    Replacement::draft(0x140971170, "unknown_140971170", tpl_6798::<0x14387e2b8, 0x14243b0b0, 0x1406e01b0, 0x10, 0x140a3bd60, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140971170
    Replacement::draft(0x1409711f0, "unknown_1409711f0", tpl_6798::<0x14387e2b8, 0x14243b0b0, 0x1406e01b0, 0x10, 0x140a3c000, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409711f0
    Replacement::draft_throws(0x140971270, "unknown_140971270", fn_140971270 as *const (), Throws::Caught), // sub_140971270
    Replacement::draft(0x1409713f0, "unknown_1409713f0", tpl_6798::<0x14387e2b8, 0x14243b0b0, 0x1406e01b0, 0x10, 0x140a3d3a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409713f0
    Replacement::draft_throws(0x140971470, "unknown_140971470", fn_140971470 as *const (), Throws::Caught), // sub_140971470
    Replacement::draft_throws(0x1409715f0, "unknown_1409715f0", fn_1409715f0 as *const (), Throws::Caught), // sub_1409715f0
    Replacement::draft_throws(0x1409716a0, "unknown_1409716a0", fn_1409716a0 as *const (), Throws::Caught), // sub_1409716a0
    Replacement::draft_throws(0x140971750, "unknown_140971750", fn_140971750 as *const (), Throws::Caught), // sub_140971750
    Replacement::draft(0x140971800, "unknown_140971800", tpl_6803::<0x14387e2b8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x10, 0x140a3cec0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140971800
    Replacement::draft(0x140971880, "unknown_140971880", tpl_6804::<0x14387de58, 0x14387e2b8, 0x1401e9970, 0x1406e1480, 0x10, 0x140a3e400, 0x1401e9860> as *const ()), // sub_140971880
    Replacement::draft(0x1409718f0, "unknown_1409718f0", tpl_6798::<0x14387e2b8, 0x14243b0b0, 0x1406e01b0, 0x10, 0x140a3e8a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409718f0
    Replacement::draft_throws(0x140971970, "unknown_140971970", tpl_6805::<0x14387de58, 0x14387e2b8, 0x1401e9770, 0x1406e1570, 0x10, 0x140a3ed10, 0x1401e9590> as *const (), Throws::Discard), // sub_140971970
    Replacement::draft_throws(0x1409719e0, "unknown_1409719e0", fn_1409719e0 as *const (), Throws::Caught), // sub_1409719e0
    Replacement::draft(0x140971b00, "unknown_140971b00", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328a418> as *const ()), // sub_140971b00
    Replacement::draft(0x140971b60, "unknown_140971b60", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328a418> as *const ()), // sub_140971b60
    Replacement::draft_throws(0x140971c60, "unknown_140971c60", fn_140971c60 as *const (), Throws::Caught), // sub_140971c60
    Replacement::draft_throws(0x140971cf0, "unknown_140971cf0", tpl_6762::<0x14387dec0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140971cf0
    Replacement::draft(0x140971da0, "unknown_140971da0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143280920> as *const ()), // sub_140971da0
    Replacement::draft(0x140971e00, "unknown_140971e00", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143280920> as *const ()), // sub_140971e00
    Replacement::draft_throws(0x140971f00, "unknown_140971f00", fn_140971f00 as *const (), Throws::Caught), // sub_140971f00
    Replacement::draft_throws(0x140971f90, "unknown_140971f90", tpl_6763::<0x14387e058, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140971f90
    Replacement::draft(0x140972040, "unknown_140972040", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328efb8> as *const ()), // sub_140972040
    Replacement::draft(0x1409720a0, "unknown_1409720a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328efb8> as *const ()), // sub_1409720a0
    Replacement::draft_throws(0x1409721a0, "unknown_1409721a0", fn_1409721a0 as *const (), Throws::Caught), // sub_1409721a0
    Replacement::draft_throws(0x140972230, "unknown_140972230", tpl_6763::<0x14387e548, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140972230
    Replacement::draft(0x1409722e0, "unknown_1409722e0", fn_1409722e0 as *const ()), // sub_1409722e0
    Replacement::draft(0x1409723d0, "unknown_1409723d0", fn_1409723d0 as *const ()), // sub_1409723d0
    Replacement::draft(0x1409724a0, "unknown_1409724a0", fn_1409724a0 as *const ()), // sub_1409724a0
    Replacement::draft(0x140972520, "unknown_140972520", fn_140972520 as *const ()), // sub_140972520
    Replacement::draft_throws(0x140972780, "unknown_140972780", fn_140972780 as *const (), Throws::Discard), // sub_140972780
    Replacement::draft(0x1409729b0, "unknown_1409729b0", fn_1409729b0 as *const ()), // sub_1409729b0
    Replacement::draft_throws(0x140972b60, "unknown_140972b60", fn_140972b60 as *const (), Throws::Caught), // sub_140972b60
    Replacement::draft(0x140972e20, "unknown_140972e20", fn_140972e20 as *const ()), // sub_140972e20
    Replacement::draft_throws(0x140972f40, "unknown_140972f40", fn_140972f40 as *const (), Throws::Caught), // sub_140972f40
    Replacement::draft_throws(0x1409732a0, "unknown_1409732a0", fn_1409732a0 as *const (), Throws::Caught), // sub_1409732a0
    Replacement::draft_throws(0x1409735d0, "unknown_1409735d0", fn_1409735d0 as *const (), Throws::Caught), // sub_1409735d0
    Replacement::draft(0x1409736e0, "unknown_1409736e0", fn_1409736e0 as *const ()), // sub_1409736e0
    Replacement::draft(0x140973920, "unknown_140973920", fn_140973920 as *const ()), // sub_140973920
    Replacement::draft(0x140973ab0, "unknown_140973ab0", fn_140973ab0 as *const ()), // sub_140973ab0
    Replacement::draft_throws(0x140973c50, "unknown_140973c50", fn_140973c50 as *const (), Throws::Discard), // sub_140973c50
    Replacement::draft_throws(0x140973dd0, "unknown_140973dd0", fn_140973dd0 as *const (), Throws::Discard), // sub_140973dd0
    Replacement::draft(0x140973f30, "unknown_140973f30", fn_140973f30 as *const ()), // sub_140973f30
    Replacement::draft(0x140974090, "unknown_140974090", fn_140974090 as *const ()), // sub_140974090
    Replacement::draft(0x1409741b0, "unknown_1409741b0", fn_1409741b0 as *const ()), // sub_1409741b0
    Replacement::draft(0x140974260, "unknown_140974260", fn_140974260 as *const ()), // sub_140974260
    Replacement::draft(0x140974310, "unknown_140974310", fn_140974310 as *const ()), // sub_140974310
    Replacement::draft(0x140974380, "unknown_140974380", fn_140974380 as *const ()), // sub_140974380
    Replacement::draft_throws(0x1409745c0, "unknown_1409745c0", fn_1409745c0 as *const (), Throws::Discard), // sub_1409745c0
    Replacement::draft(0x1409747e0, "unknown_1409747e0", fn_1409747e0 as *const ()), // sub_1409747e0
    Replacement::draft_throws(0x140974930, "unknown_140974930", fn_140974930 as *const (), Throws::Discard), // sub_140974930
    Replacement::draft_throws(0x140974b70, "unknown_140974b70", fn_140974b70 as *const (), Throws::Caught), // sub_140974b70
    Replacement::draft_throws(0x140975130, "unknown_140975130", fn_140975130 as *const (), Throws::Caught), // sub_140975130
    Replacement::draft_throws(0x140975660, "unknown_140975660", fn_140975660 as *const (), Throws::Caught), // sub_140975660
    Replacement::draft(0x140975b80, "unknown_140975b80", fn_140975b80 as *const ()), // sub_140975b80
    Replacement::draft(0x140975c40, "unknown_140975c40", fn_140975c40 as *const ()), // sub_140975c40
    Replacement::draft(0x140975db0, "unknown_140975db0", fn_140975db0 as *const ()), // sub_140975db0
    Replacement::draft_throws(0x140975f30, "unknown_140975f30", fn_140975f30 as *const (), Throws::Discard), // sub_140975f30
    Replacement::draft_throws(0x140976080, "unknown_140976080", fn_140976080 as *const (), Throws::Caught), // sub_140976080
    Replacement::draft(0x140976470, "unknown_140976470", fn_140976470 as *const ()), // sub_140976470
    Replacement::draft_throws(0x140976560, "unknown_140976560", fn_140976560 as *const (), Throws::Discard), // sub_140976560
    Replacement::draft(0x1409767a0, "unknown_1409767a0", fn_1409767a0 as *const ()), // sub_1409767a0
    Replacement::draft(0x140976840, "unknown_140976840", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143281110> as *const ()), // sub_140976840
    Replacement::draft(0x1409768a0, "unknown_1409768a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143281110> as *const ()), // sub_1409768a0
    Replacement::draft_throws(0x1409769a0, "unknown_1409769a0", fn_1409769a0 as *const (), Throws::Caught), // sub_1409769a0
    Replacement::draft_throws(0x140976a30, "unknown_140976a30", tpl_6762::<0x14387e410, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140976a30
    Replacement::draft(0x140976ae0, "unknown_140976ae0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432894a0> as *const ()), // sub_140976ae0
    Replacement::draft(0x140976b40, "unknown_140976b40", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432894a0> as *const ()), // sub_140976b40
    Replacement::draft_throws(0x140976c40, "unknown_140976c40", fn_140976c40 as *const (), Throws::Caught), // sub_140976c40
    Replacement::draft_throws(0x140976cd0, "unknown_140976cd0", tpl_6763::<0x14387deb8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140976cd0
    Replacement::draft(0x140976d80, "unknown_140976d80", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328e828> as *const ()), // sub_140976d80
    Replacement::draft(0x140976de0, "unknown_140976de0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328e828> as *const ()), // sub_140976de0
    Replacement::draft_throws(0x140976ee0, "unknown_140976ee0", fn_140976ee0 as *const (), Throws::Caught), // sub_140976ee0
    Replacement::draft_throws(0x140976f70, "unknown_140976f70", tpl_6762::<0x14387e150, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140976f70
    Replacement::draft(0x140977020, "unknown_140977020", tpl_11650::<0x98, 0x10, 0x18, 0xffffffffffffff80, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x28, 0x48, 0x68, 0x88, 0x14328e260> as *const ()), // sub_140977020
    Replacement::draft(0x1409770e0, "unknown_1409770e0", tpl_11651::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328e260, 0xfffffffffffffffc, 0x90, 0xffffffffffffff75, 0xffffffffffffff95, 0xffffffffffffffb5> as *const ()), // sub_1409770e0
    Replacement::draft(0x140977240, "unknown_140977240", tpl_9966::<0x14387ddb0, 0x14243b0b0, 0x1406e01b0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140977240
    Replacement::draft_throws(0x140977300, "unknown_140977300", tpl_9629::<0x14387e0e8, 0x14387ddb0, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140977300
    Replacement::draft(0x1409773b0, "unknown_1409773b0", tpl_9969::<0x14387ddb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409773b0
    Replacement::draft_throws(0x140977430, "unknown_140977430", tpl_9630::<0x14387e0e8, 0x14387ddb0, 0x1401e9b60, 0x1406e03a0, 0x328, 0x8, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140977430
    Replacement::draft_throws(0x1409774e0, "unknown_1409774e0", fn_1409774e0 as *const (), Throws::Caught), // sub_1409774e0
    Replacement::draft_throws(0x140977750, "unknown_140977750", fn_140977750 as *const (), Throws::Caught), // sub_140977750
    Replacement::draft_throws(0x140977980, "unknown_140977980", fn_140977980 as *const (), Throws::Caught), // sub_140977980
    Replacement::draft(0x140977ba0, "unknown_140977ba0", fn_140977ba0 as *const ()), // sub_140977ba0
    Replacement::draft(0x140977c40, "unknown_140977c40", tpl_7856::<0x14387e0e8, 0x14387ddb0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_140977c40
    Replacement::draft(0x140977cb0, "unknown_140977cb0", tpl_9969::<0x14387ddb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140977cb0
    Replacement::draft_throws(0x140977d30, "unknown_140977d30", tpl_9971::<0x14387e0e8, 0x14387ddb0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_140977d30
    Replacement::draft_throws(0x140977da0, "unknown_140977da0", fn_140977da0 as *const (), Throws::Discard), // sub_140977da0
    Replacement::draft(0x140977e90, "unknown_140977e90", tpl_11441::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x14328ff30, 0x8, 0x1c> as *const ()), // sub_140977e90
    Replacement::draft(0x140977f20, "unknown_140977f20", tpl_11442::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x14328ff30, 0x5, 0x7, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140977f20
    Replacement::draft(0x140977fd0, "unknown_140977fd0", fn_140977fd0 as *const ()), // sub_140977fd0
    Replacement::draft(0x1409780b0, "unknown_1409780b0", fn_1409780b0 as *const ()), // sub_1409780b0
    Replacement::draft_throws(0x1409781a0, "unknown_1409781a0", fn_1409781a0 as *const (), Throws::Discard), // sub_1409781a0
    Replacement::draft(0x1409782c0, "unknown_1409782c0", fn_1409782c0 as *const ()), // sub_1409782c0
    Replacement::draft_throws(0x1409783b0, "unknown_1409783b0", fn_1409783b0 as *const (), Throws::Discard), // sub_1409783b0
    Replacement::draft_throws(0x1409784d0, "unknown_1409784d0", fn_1409784d0 as *const (), Throws::Caught), // sub_1409784d0
    Replacement::draft_throws(0x140978580, "unknown_140978580", fn_140978580 as *const (), Throws::Caught), // sub_140978580
    Replacement::draft_throws(0x140978630, "unknown_140978630", fn_140978630 as *const (), Throws::Caught), // sub_140978630
    Replacement::draft(0x1409786e0, "unknown_1409786e0", fn_1409786e0 as *const ()), // sub_1409786e0
    Replacement::draft(0x140978800, "unknown_140978800", fn_140978800 as *const ()), // sub_140978800
    Replacement::draft(0x140978930, "unknown_140978930", fn_140978930 as *const ()), // sub_140978930
    Replacement::draft_throws(0x140978a70, "unknown_140978a70", fn_140978a70 as *const (), Throws::Discard), // sub_140978a70
    Replacement::draft_throws(0x140978b50, "unknown_140978b50", fn_140978b50 as *const (), Throws::Caught), // sub_140978b50
    Replacement::draft(0x140978c90, "unknown_140978c90", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143293f18> as *const ()), // sub_140978c90
    Replacement::draft(0x140978cf0, "unknown_140978cf0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143293f18> as *const ()), // sub_140978cf0
    Replacement::draft_throws(0x140978df0, "unknown_140978df0", fn_140978df0 as *const (), Throws::Caught), // sub_140978df0
    Replacement::draft_throws(0x140978e80, "unknown_140978e80", tpl_6763::<0x14387e1c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140978e80
    Replacement::draft(0x140978f30, "unknown_140978f30", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143285838, 0x8> as *const ()), // sub_140978f30
    Replacement::draft(0x140978fb0, "unknown_140978fb0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143285838> as *const ()), // sub_140978fb0
    Replacement::draft(0x1409790b0, "unknown_1409790b0", tpl_11667::<0x14387e078, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409790b0
    Replacement::draft_throws(0x140979150, "unknown_140979150", tpl_11668::<0x14387e2f0, 0x14387e078, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140979150
    Replacement::draft(0x1409791d0, "unknown_1409791d0", tpl_11361::<0x14387e078, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409791d0
    Replacement::draft_throws(0x140979250, "unknown_140979250", tpl_11669::<0x14387e2f0, 0x14387e078, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140979250
    Replacement::draft_throws(0x1409792e0, "unknown_1409792e0", fn_1409792e0 as *const (), Throws::Caught), // sub_1409792e0
    Replacement::draft_throws(0x1409794e0, "unknown_1409794e0", fn_1409794e0 as *const (), Throws::Caught), // sub_1409794e0
    Replacement::draft_throws(0x1409796b0, "unknown_1409796b0", fn_1409796b0 as *const (), Throws::Caught), // sub_1409796b0
    Replacement::draft(0x140979880, "unknown_140979880", tpl_11673::<0x14387e078, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140979880
    Replacement::draft(0x1409798f0, "unknown_1409798f0", tpl_67834::<0x14387e2f0, 0x14387e078, 0x1401e9970, 0x1406e1480, 0x8, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_1409798f0
    Replacement::draft(0x140979970, "unknown_140979970", tpl_11674::<0x14387e078, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140979970
    Replacement::draft_throws(0x140979a00, "unknown_140979a00", tpl_9073::<0x14387e2f0, 0x14387e078, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140979a00
    Replacement::draft_throws(0x140979a70, "unknown_140979a70", fn_140979a70 as *const (), Throws::Discard), // sub_140979a70
    Replacement::draft(0x140979b50, "unknown_140979b50", fn_140979b50 as *const ()), // sub_140979b50
    Replacement::draft(0x140979bf0, "unknown_140979bf0", fn_140979bf0 as *const ()), // sub_140979bf0
    Replacement::draft(0x140979ca0, "unknown_140979ca0", fn_140979ca0 as *const ()), // sub_140979ca0
    Replacement::draft(0x140979d50, "unknown_140979d50", fn_140979d50 as *const ()), // sub_140979d50
    Replacement::draft_throws(0x140979ed0, "unknown_140979ed0", fn_140979ed0 as *const (), Throws::Caught), // sub_140979ed0
    Replacement::draft(0x14097a190, "unknown_14097a190", fn_14097a190 as *const ()), // sub_14097a190
    Replacement::draft_throws(0x14097a2e0, "unknown_14097a2e0", fn_14097a2e0 as *const (), Throws::Caught), // sub_14097a2e0
    Replacement::draft_throws(0x14097a5a0, "unknown_14097a5a0", fn_14097a5a0 as *const (), Throws::Caught), // sub_14097a5a0
    Replacement::draft_throws(0x14097a9c0, "unknown_14097a9c0", fn_14097a9c0 as *const (), Throws::Caught), // sub_14097a9c0
    Replacement::draft_throws(0x14097ad60, "unknown_14097ad60", fn_14097ad60 as *const (), Throws::Caught), // sub_14097ad60
    Replacement::draft(0x14097b0f0, "unknown_14097b0f0", fn_14097b0f0 as *const ()), // sub_14097b0f0
    Replacement::draft(0x14097b1c0, "unknown_14097b1c0", fn_14097b1c0 as *const ()), // sub_14097b1c0
    Replacement::draft(0x14097b310, "unknown_14097b310", fn_14097b310 as *const ()), // sub_14097b310
    Replacement::draft_throws(0x14097b470, "unknown_14097b470", fn_14097b470 as *const (), Throws::Discard), // sub_14097b470
    Replacement::draft_throws(0x14097b5a0, "unknown_14097b5a0", fn_14097b5a0 as *const (), Throws::Caught), // sub_14097b5a0
    Replacement::draft_throws(0x14097b830, "unknown_14097b830", fn_14097b830 as *const (), Throws::Discard), // sub_14097b830
    Replacement::draft(0x14097b900, "unknown_14097b900", tpl_11529::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x143288b58> as *const ()), // sub_14097b900
    Replacement::draft(0x14097b990, "unknown_14097b990", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143288b58, 0x18, 0x20> as *const ()), // sub_14097b990
    Replacement::draft(0x14097baa0, "unknown_14097baa0", fn_14097baa0 as *const ()), // sub_14097baa0
    Replacement::draft_throws(0x14097bc10, "unknown_14097bc10", fn_14097bc10 as *const (), Throws::Discard), // sub_14097bc10
    Replacement::draft(0x14097bd50, "unknown_14097bd50", tpl_11694::<0x14387e110, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_14097bd50
    Replacement::draft_throws(0x14097be10, "unknown_14097be10", fn_14097be10 as *const (), Throws::Discard), // sub_14097be10
    Replacement::draft_throws(0x14097bf80, "unknown_14097bf80", fn_14097bf80 as *const (), Throws::Caught), // sub_14097bf80
    Replacement::draft_throws(0x14097c1e0, "unknown_14097c1e0", fn_14097c1e0 as *const (), Throws::Caught), // sub_14097c1e0
    Replacement::draft_throws(0x14097c440, "unknown_14097c440", fn_14097c440 as *const (), Throws::Caught), // sub_14097c440
    Replacement::draft(0x14097c6a0, "unknown_14097c6a0", fn_14097c6a0 as *const ()), // sub_14097c6a0
    Replacement::draft(0x14097c720, "unknown_14097c720", fn_14097c720 as *const ()), // sub_14097c720
    Replacement::draft(0x14097c7e0, "unknown_14097c7e0", tpl_11694::<0x14387e110, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_14097c7e0
    Replacement::draft_throws(0x14097c8b0, "unknown_14097c8b0", fn_14097c8b0 as *const (), Throws::Discard), // sub_14097c8b0
    Replacement::draft_throws(0x14097c9a0, "unknown_14097c9a0", fn_14097c9a0 as *const (), Throws::Discard), // sub_14097c9a0
    Replacement::draft(0x14097cc20, "unknown_14097cc20", fn_14097cc20 as *const ()), // sub_14097cc20
    Replacement::draft(0x14097cd90, "unknown_14097cd90", fn_14097cd90 as *const ()), // sub_14097cd90
    Replacement::draft(0x14097ce70, "unknown_14097ce70", fn_14097ce70 as *const ()), // sub_14097ce70
    Replacement::draft(0x14097cf70, "unknown_14097cf70", tpl_7572::<0x14387e4f0, 0x14243b0b0, 0x1406e01b0, 0x1438820c0, 0x14, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14097cf70
    Replacement::draft(0x14097d010, "unknown_14097d010", fn_14097d010 as *const ()), // sub_14097d010
    Replacement::draft_throws(0x14097d220, "unknown_14097d220", fn_14097d220 as *const (), Throws::Discard), // sub_14097d220
    Replacement::draft(0x14097d410, "unknown_14097d410", fn_14097d410 as *const ()), // sub_14097d410
    Replacement::draft_throws(0x14097d560, "unknown_14097d560", fn_14097d560 as *const (), Throws::Discard), // sub_14097d560
    Replacement::draft(0x14097d750, "unknown_14097d750", fn_14097d750 as *const ()), // sub_14097d750
    Replacement::draft_throws(0x14097d800, "unknown_14097d800", fn_14097d800 as *const (), Throws::Caught), // sub_14097d800
    Replacement::draft_throws(0x14097dd10, "unknown_14097dd10", fn_14097dd10 as *const (), Throws::Caught), // sub_14097dd10
    Replacement::draft_throws(0x14097e1b0, "unknown_14097e1b0", fn_14097e1b0 as *const (), Throws::Caught), // sub_14097e1b0
    Replacement::draft(0x14097e640, "unknown_14097e640", fn_14097e640 as *const ()), // sub_14097e640
    Replacement::draft(0x14097e6f0, "unknown_14097e6f0", fn_14097e6f0 as *const ()), // sub_14097e6f0
    Replacement::draft(0x14097e860, "unknown_14097e860", fn_14097e860 as *const ()), // sub_14097e860
    Replacement::draft_throws(0x14097e9e0, "unknown_14097e9e0", fn_14097e9e0 as *const (), Throws::Discard), // sub_14097e9e0
    Replacement::draft_throws(0x14097eb20, "unknown_14097eb20", fn_14097eb20 as *const (), Throws::Caught), // sub_14097eb20
    Replacement::draft_throws(0x14097ee20, "unknown_14097ee20", fn_14097ee20 as *const (), Throws::Discard), // sub_14097ee20
    Replacement::draft(0x14097f000, "unknown_14097f000", fn_14097f000 as *const ()), // sub_14097f000
    Replacement::draft(0x14097f0c0, "unknown_14097f0c0", fn_14097f0c0 as *const ()), // sub_14097f0c0
    Replacement::draft(0x14097f190, "unknown_14097f190", fn_14097f190 as *const ()), // sub_14097f190
    Replacement::draft(0x14097f230, "unknown_14097f230", tpl_11372::<0x14387e210, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406fe630, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14097f230
    Replacement::draft_throws(0x14097f2b0, "unknown_14097f2b0", tpl_6799::<0x14387e328, 0x14387e210, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1406db430, 0x1401e9e70> as *const (), Throws::Discard), // sub_14097f2b0
    Replacement::draft(0x14097f320, "unknown_14097f320", tpl_11372::<0x14387e210, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406fe890, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14097f320
    Replacement::draft_throws(0x14097f3a0, "unknown_14097f3a0", tpl_6799::<0x14387e328, 0x14387e210, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1406db670, 0x1401e9a20> as *const (), Throws::Discard), // sub_14097f3a0
    Replacement::draft_throws(0x14097f410, "unknown_14097f410", fn_14097f410 as *const (), Throws::Caught), // sub_14097f410
    Replacement::draft_throws(0x14097f4c0, "unknown_14097f4c0", fn_14097f4c0 as *const (), Throws::Caught), // sub_14097f4c0
    Replacement::draft_throws(0x14097f570, "unknown_14097f570", fn_14097f570 as *const (), Throws::Caught), // sub_14097f570
    Replacement::draft(0x14097f620, "unknown_14097f620", fn_14097f620 as *const ()), // sub_14097f620
    Replacement::draft(0x14097f6a0, "unknown_14097f6a0", tpl_6804::<0x14387e328, 0x14387e210, 0x1401e9970, 0x1406e1480, 0x8, 0x1406fea30, 0x1401e9860> as *const ()), // sub_14097f6a0
    Replacement::draft(0x14097f710, "unknown_14097f710", tpl_11372::<0x14387e210, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1406feb00, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14097f710
    Replacement::draft_throws(0x14097f790, "unknown_14097f790", tpl_6805::<0x14387e328, 0x14387e210, 0x1401e9770, 0x1406e1570, 0x8, 0x1406febe0, 0x1401e9590> as *const (), Throws::Discard), // sub_14097f790
    Replacement::draft_throws(0x14097f800, "unknown_14097f800", fn_14097f800 as *const (), Throws::Caught), // sub_14097f800
    Replacement::draft(0x14097f920, "unknown_14097f920", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143289fa8> as *const ()), // sub_14097f920
    Replacement::draft(0x14097f980, "unknown_14097f980", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143289fa8> as *const ()), // sub_14097f980
    Replacement::draft_throws(0x14097fa80, "unknown_14097fa80", fn_14097fa80 as *const (), Throws::Caught), // sub_14097fa80
    Replacement::draft_throws(0x14097fb10, "unknown_14097fb10", tpl_6762::<0x14387e020, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14097fb10
    Replacement::draft(0x14097fbc0, "unknown_14097fbc0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143298af8> as *const ()), // sub_14097fbc0
    Replacement::draft(0x14097fc20, "unknown_14097fc20", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143298af8> as *const ()), // sub_14097fc20
    Replacement::draft_throws(0x14097fd20, "unknown_14097fd20", fn_14097fd20 as *const (), Throws::Caught), // sub_14097fd20
    Replacement::draft_throws(0x14097fdb0, "unknown_14097fdb0", tpl_6763::<0x14387e3c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14097fdb0
    Replacement::draft(0x14097fe60, "unknown_14097fe60", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328ee58> as *const ()), // sub_14097fe60
    Replacement::draft(0x14097fec0, "unknown_14097fec0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328ee58> as *const ()), // sub_14097fec0
    Replacement::draft_throws(0x14097ffc0, "unknown_14097ffc0", fn_14097ffc0 as *const (), Throws::Caught), // sub_14097ffc0
    Replacement::draft_throws(0x140980050, "unknown_140980050", tpl_6763::<0x14387e4a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140980050
    Replacement::draft(0x140980100, "unknown_140980100", fn_140980100 as *const ()), // sub_140980100
    Replacement::draft(0x140980190, "unknown_140980190", fn_140980190 as *const ()), // sub_140980190
    Replacement::draft(0x140980240, "unknown_140980240", fn_140980240 as *const ()), // sub_140980240
    Replacement::draft(0x1409802f0, "unknown_1409802f0", fn_1409802f0 as *const ()), // sub_1409802f0
    Replacement::draft_throws(0x140980410, "unknown_140980410", fn_140980410 as *const (), Throws::Discard), // sub_140980410
    Replacement::draft_throws(0x140980500, "unknown_140980500", fn_140980500 as *const (), Throws::Discard), // sub_140980500
    Replacement::draft_throws(0x1409805f0, "unknown_1409805f0", fn_1409805f0 as *const (), Throws::Discard), // sub_1409805f0
    Replacement::draft_throws(0x140980700, "unknown_140980700", fn_140980700 as *const (), Throws::Discard), // sub_140980700
    Replacement::draft_throws(0x1409807a0, "unknown_1409807a0", fn_1409807a0 as *const (), Throws::Caught), // sub_1409807a0
    Replacement::draft_throws(0x1409808f0, "unknown_1409808f0", fn_1409808f0 as *const (), Throws::Caught), // sub_1409808f0
    Replacement::draft_throws(0x140980a40, "unknown_140980a40", fn_140980a40 as *const (), Throws::Caught), // sub_140980a40
    Replacement::draft(0x140980b90, "unknown_140980b90", fn_140980b90 as *const ()), // sub_140980b90
    Replacement::draft(0x140980c60, "unknown_140980c60", fn_140980c60 as *const ()), // sub_140980c60
    Replacement::draft(0x140980d40, "unknown_140980d40", fn_140980d40 as *const ()), // sub_140980d40
    Replacement::draft_throws(0x140980e30, "unknown_140980e30", fn_140980e30 as *const (), Throws::Discard), // sub_140980e30
    Replacement::draft_throws(0x140980f10, "unknown_140980f10", fn_140980f10 as *const (), Throws::Discard), // sub_140980f10
    Replacement::draft(0x140981050, "unknown_140981050", fn_140981050 as *const ()), // sub_140981050
    Replacement::draft(0x140981140, "unknown_140981140", fn_140981140 as *const ()), // sub_140981140
    Replacement::draft(0x1409811f0, "unknown_1409811f0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143285118, 0x8> as *const ()), // sub_1409811f0
    Replacement::draft(0x140981280, "unknown_140981280", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143285118, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140981280
    Replacement::draft(0x140981330, "unknown_140981330", fn_140981330 as *const ()), // sub_140981330
    Replacement::draft(0x140981410, "unknown_140981410", fn_140981410 as *const ()), // sub_140981410
    Replacement::draft_throws(0x140981510, "unknown_140981510", tpl_9677::<0x14387dd78, 0x14387e388, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x48, 0x8, 0x140a02200, 0x140a12af0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140981510
    Replacement::draft(0x1409815c0, "unknown_1409815c0", fn_1409815c0 as *const ()), // sub_1409815c0
    Replacement::draft_throws(0x140981690, "unknown_140981690", tpl_9678::<0x14387dd78, 0x14387e388, 0x1401e9b60, 0x1406e03a0, 0x8, 0x328, 0x338, 0x120, 0x140a02200, 0x140a12df0, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_140981690
    Replacement::draft(0x140981750, "unknown_140981750", fn_140981750 as *const ()), // sub_140981750
    Replacement::draft_throws(0x1409817f0, "unknown_1409817f0", fn_1409817f0 as *const (), Throws::Caught), // sub_1409817f0
    Replacement::draft_throws(0x1409818b0, "unknown_1409818b0", fn_1409818b0 as *const (), Throws::Caught), // sub_1409818b0
    Replacement::draft_throws(0x140981970, "unknown_140981970", fn_140981970 as *const (), Throws::Caught), // sub_140981970
    Replacement::draft(0x140981a30, "unknown_140981a30", fn_140981a30 as *const ()), // sub_140981a30
    Replacement::draft(0x140981af0, "unknown_140981af0", fn_140981af0 as *const ()), // sub_140981af0
    Replacement::draft(0x140981ba0, "unknown_140981ba0", fn_140981ba0 as *const ()), // sub_140981ba0
    Replacement::draft_throws(0x140981c70, "unknown_140981c70", fn_140981c70 as *const (), Throws::Discard), // sub_140981c70
    Replacement::draft_throws(0x140981d20, "unknown_140981d20", tpl_6745::<0x14387e388, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a15f8, 0x8, 0x1432a1598, 0x1409fcb50> as *const (), Throws::Discard), // sub_140981d20
    Replacement::draft(0x140981e00, "unknown_140981e00", tpl_70232::<0x8, 0x10, 0x1426cdcb0, 0x140a13ee0> as *const ()), // sub_140981e00
    Replacement::draft(0x140981e80, "unknown_140981e80", tpl_70232::<0x8, 0x10, 0x1432a1598, 0x140a13ef0> as *const ()), // sub_140981e80
    Replacement::draft(0x140981f00, "unknown_140981f00", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143283e20> as *const ()), // sub_140981f00
    Replacement::draft(0x140981f60, "unknown_140981f60", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143283e20> as *const ()), // sub_140981f60
    Replacement::draft_throws(0x140982060, "unknown_140982060", fn_140982060 as *const (), Throws::Caught), // sub_140982060
    Replacement::draft_throws(0x1409820f0, "unknown_1409820f0", tpl_6763::<0x14387e390, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409820f0
    Replacement::draft(0x1409821a0, "unknown_1409821a0", fn_1409821a0 as *const ()), // sub_1409821a0
    Replacement::draft(0x140982250, "unknown_140982250", fn_140982250 as *const ()), // sub_140982250
    Replacement::draft(0x140982310, "unknown_140982310", fn_140982310 as *const ()), // sub_140982310
    Replacement::draft(0x140982440, "unknown_140982440", fn_140982440 as *const ()), // sub_140982440
    Replacement::draft_throws(0x1409825c0, "unknown_1409825c0", fn_1409825c0 as *const (), Throws::Discard), // sub_1409825c0
    Replacement::draft(0x140982730, "unknown_140982730", fn_140982730 as *const ()), // sub_140982730
    Replacement::draft_throws(0x140982870, "unknown_140982870", fn_140982870 as *const (), Throws::Discard), // sub_140982870
    Replacement::draft_throws(0x1409829e0, "unknown_1409829e0", fn_1409829e0 as *const (), Throws::Caught), // sub_1409829e0
    Replacement::draft_throws(0x140982c50, "unknown_140982c50", fn_140982c50 as *const (), Throws::Caught), // sub_140982c50
    Replacement::draft_throws(0x140982e80, "unknown_140982e80", fn_140982e80 as *const (), Throws::Caught), // sub_140982e80
    Replacement::draft(0x1409830a0, "unknown_1409830a0", fn_1409830a0 as *const ()), // sub_1409830a0
    Replacement::draft(0x1409831f0, "unknown_1409831f0", fn_1409831f0 as *const ()), // sub_1409831f0
    Replacement::draft(0x140983320, "unknown_140983320", fn_140983320 as *const ()), // sub_140983320
    Replacement::draft_throws(0x140983460, "unknown_140983460", fn_140983460 as *const (), Throws::Discard), // sub_140983460
    Replacement::draft_throws(0x140983590, "unknown_140983590", fn_140983590 as *const (), Throws::Discard), // sub_140983590
    Replacement::draft(0x140983690, "unknown_140983690", fn_140983690 as *const ()), // sub_140983690
    Replacement::draft(0x140983760, "unknown_140983760", fn_140983760 as *const ()), // sub_140983760
    Replacement::draft(0x140983850, "unknown_140983850", tpl_6748::<0x14387e238, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140983850
    Replacement::draft(0x1409838c0, "unknown_1409838c0", fn_1409838c0 as *const ()), // sub_1409838c0
    Replacement::draft_throws(0x140983ae0, "unknown_140983ae0", fn_140983ae0 as *const (), Throws::Caught), // sub_140983ae0
    Replacement::draft(0x140983e20, "unknown_140983e20", fn_140983e20 as *const ()), // sub_140983e20
    Replacement::draft_throws(0x140983f80, "unknown_140983f80", fn_140983f80 as *const (), Throws::Caught), // sub_140983f80
    Replacement::draft(0x140984310, "unknown_140984310", fn_140984310 as *const ()), // sub_140984310
    Replacement::draft_throws(0x1409843f0, "unknown_1409843f0", fn_1409843f0 as *const (), Throws::Caught), // sub_1409843f0
    Replacement::draft_throws(0x140984ba0, "unknown_140984ba0", fn_140984ba0 as *const (), Throws::Caught), // sub_140984ba0
    Replacement::draft_throws(0x140985270, "unknown_140985270", fn_140985270 as *const (), Throws::Caught), // sub_140985270
    Replacement::draft(0x140985930, "unknown_140985930", fn_140985930 as *const ()), // sub_140985930
    Replacement::draft(0x140985a20, "unknown_140985a20", fn_140985a20 as *const ()), // sub_140985a20
    Replacement::draft(0x140985b80, "unknown_140985b80", fn_140985b80 as *const ()), // sub_140985b80
    Replacement::draft_throws(0x140985cf0, "unknown_140985cf0", fn_140985cf0 as *const (), Throws::Discard), // sub_140985cf0
    Replacement::draft_throws(0x140985e30, "unknown_140985e30", fn_140985e30 as *const (), Throws::Caught), // sub_140985e30
    Replacement::draft(0x140986280, "unknown_140986280", fn_140986280 as *const ()), // sub_140986280
    Replacement::draft_throws(0x1409864a0, "unknown_1409864a0", fn_1409864a0 as *const (), Throws::Discard), // sub_1409864a0
    Replacement::draft(0x1409865f0, "unknown_1409865f0", fn_1409865f0 as *const ()), // sub_1409865f0
    Replacement::draft(0x140986730, "unknown_140986730", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14329ae10, 0x8> as *const ()), // sub_140986730
    Replacement::draft(0x1409867b0, "unknown_1409867b0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329ae10> as *const ()), // sub_1409867b0
    Replacement::draft(0x140986840, "unknown_140986840", tpl_6766::<0x14387e460, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140986840
    Replacement::draft(0x1409868b0, "unknown_1409868b0", fn_1409868b0 as *const ()), // sub_1409868b0
    Replacement::draft_throws(0x1409869e0, "unknown_1409869e0", fn_1409869e0 as *const (), Throws::Discard), // sub_1409869e0
    Replacement::draft(0x140986b20, "unknown_140986b20", tpl_11772::<0x14387e460, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x80, 0xd, 0xe, 0xf, 0xfffffffffffffff8> as *const ()), // sub_140986b20
    Replacement::draft_throws(0x140986bf0, "unknown_140986bf0", fn_140986bf0 as *const (), Throws::Discard), // sub_140986bf0
    Replacement::draft_throws(0x140986d50, "unknown_140986d50", fn_140986d50 as *const (), Throws::Caught), // sub_140986d50
    Replacement::draft_throws(0x140986f30, "unknown_140986f30", fn_140986f30 as *const (), Throws::Caught), // sub_140986f30
    Replacement::draft_throws(0x140987110, "unknown_140987110", fn_140987110 as *const (), Throws::Caught), // sub_140987110
    Replacement::draft(0x1409872f0, "unknown_1409872f0", fn_1409872f0 as *const ()), // sub_1409872f0
    Replacement::draft(0x140987360, "unknown_140987360", fn_140987360 as *const ()), // sub_140987360
    Replacement::draft(0x140987420, "unknown_140987420", tpl_11772::<0x14387e460, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x120, 0xd, 0xe, 0xf, 0xfffffffffffffff8> as *const ()), // sub_140987420
    Replacement::draft_throws(0x1409874f0, "unknown_1409874f0", fn_1409874f0 as *const (), Throws::Discard), // sub_1409874f0
    Replacement::draft_throws(0x1409875e0, "unknown_1409875e0", fn_1409875e0 as *const (), Throws::Discard), // sub_1409875e0
    Replacement::draft(0x1409877e0, "unknown_1409877e0", tpl_11779::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143283068> as *const ()), // sub_1409877e0
    Replacement::draft(0x140987860, "unknown_140987860", tpl_11780::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143283068, 0xfffffffffffffff0> as *const ()), // sub_140987860
    Replacement::draft(0x140987960, "unknown_140987960", tpl_11781::<0x14387dfc8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140987960
    Replacement::draft_throws(0x1409879f0, "unknown_1409879f0", tpl_11782::<0x14387e32c, 0x14387dfc8, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409879f0
    Replacement::draft(0x140987a70, "unknown_140987a70", tpl_11361::<0x14387dfc8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140987a70
    Replacement::draft_throws(0x140987af0, "unknown_140987af0", tpl_11783::<0x14387e32c, 0x14387dfc8, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140987af0
    Replacement::draft_throws(0x140987b80, "unknown_140987b80", fn_140987b80 as *const (), Throws::Caught), // sub_140987b80
    Replacement::draft_throws(0x140987c50, "unknown_140987c50", fn_140987c50 as *const (), Throws::Caught), // sub_140987c50
    Replacement::draft_throws(0x140987d20, "unknown_140987d20", fn_140987d20 as *const (), Throws::Caught), // sub_140987d20
    Replacement::draft(0x140987df0, "unknown_140987df0", fn_140987df0 as *const ()), // sub_140987df0
    Replacement::draft(0x140987e60, "unknown_140987e60", tpl_9203::<0x14387e32c, 0x14387dfc8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140987e60
    Replacement::draft(0x140987ed0, "unknown_140987ed0", tpl_11361::<0x14387dfc8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140987ed0
    Replacement::draft_throws(0x140987f50, "unknown_140987f50", tpl_11786::<0x14387e32c, 0x14387dfc8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140987f50
    Replacement::draft_throws(0x140987fc0, "unknown_140987fc0", fn_140987fc0 as *const (), Throws::Discard), // sub_140987fc0
    Replacement::draft(0x1409880c0, "unknown_1409880c0", fn_1409880c0 as *const ()), // sub_1409880c0
    Replacement::draft(0x140988150, "unknown_140988150", fn_140988150 as *const ()), // sub_140988150
    Replacement::draft(0x140988200, "unknown_140988200", fn_140988200 as *const ()), // sub_140988200
    Replacement::draft(0x140988410, "unknown_140988410", fn_140988410 as *const ()), // sub_140988410
    Replacement::draft_throws(0x1409886e0, "unknown_1409886e0", fn_1409886e0 as *const (), Throws::Caught), // sub_1409886e0
    Replacement::draft(0x140988ab0, "unknown_140988ab0", fn_140988ab0 as *const ()), // sub_140988ab0
    Replacement::draft_throws(0x140988d30, "unknown_140988d30", fn_140988d30 as *const (), Throws::Caught), // sub_140988d30
    Replacement::draft(0x1409890f0, "unknown_1409890f0", fn_1409890f0 as *const ()), // sub_1409890f0
    Replacement::draft_throws(0x1409891f0, "unknown_1409891f0", fn_1409891f0 as *const (), Throws::Caught), // sub_1409891f0
    Replacement::draft_throws(0x140989440, "unknown_140989440", fn_140989440 as *const (), Throws::Caught), // sub_140989440
    Replacement::draft_throws(0x140989690, "unknown_140989690", fn_140989690 as *const (), Throws::Caught), // sub_140989690
    Replacement::draft(0x140989a30, "unknown_140989a30", fn_140989a30 as *const ()), // sub_140989a30
    Replacement::draft(0x140989ce0, "unknown_140989ce0", fn_140989ce0 as *const ()), // sub_140989ce0
    Replacement::draft(0x140989f40, "unknown_140989f40", fn_140989f40 as *const ()), // sub_140989f40
    Replacement::draft_throws(0x14098a1b0, "unknown_14098a1b0", fn_14098a1b0 as *const (), Throws::Caught), // sub_14098a1b0
    Replacement::draft_throws(0x14098a490, "unknown_14098a490", fn_14098a490 as *const (), Throws::Caught), // sub_14098a490
    Replacement::draft_throws(0x14098a760, "unknown_14098a760", tpl_11802::<0x143426b90, 0x143277a70, 0x8, 0x143277b20, 0x1432643e8, 0x14020f270, 0x60, 0x1432a09f0, 0x6, 0x1407f52b0> as *const (), Throws::Discard), // sub_14098a760
    Replacement::draft(0x14098a7e0, "unknown_14098a7e0", tpl_7173::<0x1432a09a8, 0x10, 0x7, 0x140947eb0, 0x28, 0x1432a0968> as *const ()), // sub_14098a7e0
    Replacement::draft(0x14098a850, "unknown_14098a850", fn_14098a850 as *const ()), // sub_14098a850
    Replacement::draft(0x14098a940, "unknown_14098a940", fn_14098a940 as *const ()), // sub_14098a940
    Replacement::draft(0x14098aa50, "unknown_14098aa50", fn_14098aa50 as *const ()), // sub_14098aa50
    Replacement::draft(0x14098aac0, "unknown_14098aac0", fn_14098aac0 as *const ()), // sub_14098aac0
    Replacement::draft_throws(0x14098ac30, "unknown_14098ac30", fn_14098ac30 as *const (), Throws::Discard), // sub_14098ac30
    Replacement::draft(0x14098ade0, "unknown_14098ade0", fn_14098ade0 as *const ()), // sub_14098ade0
    Replacement::draft_throws(0x14098aec0, "unknown_14098aec0", fn_14098aec0 as *const (), Throws::Discard), // sub_14098aec0
    Replacement::draft(0x14098b0e0, "unknown_14098b0e0", fn_14098b0e0 as *const ()), // sub_14098b0e0
    Replacement::draft_throws(0x14098b1a0, "unknown_14098b1a0", fn_14098b1a0 as *const (), Throws::Caught), // sub_14098b1a0
    Replacement::draft_throws(0x14098b460, "unknown_14098b460", fn_14098b460 as *const (), Throws::Caught), // sub_14098b460
    Replacement::draft_throws(0x14098b720, "unknown_14098b720", fn_14098b720 as *const (), Throws::Caught), // sub_14098b720
    Replacement::draft(0x14098bb00, "unknown_14098bb00", fn_14098bb00 as *const ()), // sub_14098bb00
    Replacement::draft(0x14098bc10, "unknown_14098bc10", fn_14098bc10 as *const ()), // sub_14098bc10
    Replacement::draft(0x14098bcf0, "unknown_14098bcf0", fn_14098bcf0 as *const ()), // sub_14098bcf0
    Replacement::draft_throws(0x14098bdd0, "unknown_14098bdd0", fn_14098bdd0 as *const (), Throws::Discard), // sub_14098bdd0
    Replacement::draft_throws(0x14098bf10, "unknown_14098bf10", fn_14098bf10 as *const (), Throws::Discard), // sub_14098bf10
    Replacement::draft(0x14098c1d0, "unknown_14098c1d0", fn_14098c1d0 as *const ()), // sub_14098c1d0
    Replacement::draft_throws(0x14098c2f0, "unknown_14098c2f0", fn_14098c2f0 as *const (), Throws::Discard), // sub_14098c2f0
    Replacement::draft(0x14098c420, "unknown_14098c420", fn_14098c420 as *const ()), // sub_14098c420
    Replacement::draft(0x14098c4a0, "unknown_14098c4a0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328ccb8> as *const ()), // sub_14098c4a0
    Replacement::draft(0x14098c500, "unknown_14098c500", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328ccb8> as *const ()), // sub_14098c500
    Replacement::draft_throws(0x14098c600, "unknown_14098c600", fn_14098c600 as *const (), Throws::Caught), // sub_14098c600
    Replacement::draft_throws(0x14098c690, "unknown_14098c690", tpl_6762::<0x14387de98, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14098c690
    Replacement::draft(0x14098c740, "unknown_14098c740", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432914d8, 0x8> as *const ()), // sub_14098c740
    Replacement::draft(0x14098c7c0, "unknown_14098c7c0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432914d8> as *const ()), // sub_14098c7c0
    Replacement::draft(0x14098c850, "unknown_14098c850", tpl_7312::<0x14387e430, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14098c850
    Replacement::draft(0x14098c8c0, "unknown_14098c8c0", tpl_11821::<0x14387e430, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14098c8c0
    Replacement::draft_throws(0x14098c960, "unknown_14098c960", fn_14098c960 as *const (), Throws::Discard), // sub_14098c960
    Replacement::draft(0x14098c9f0, "unknown_14098c9f0", tpl_11823::<0x14387e430, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14098c9f0
    Replacement::draft_throws(0x14098ca70, "unknown_14098ca70", fn_14098ca70 as *const (), Throws::Discard), // sub_14098ca70
    Replacement::draft_throws(0x14098cb00, "unknown_14098cb00", fn_14098cb00 as *const (), Throws::Caught), // sub_14098cb00
    Replacement::draft_throws(0x14098cbe0, "unknown_14098cbe0", fn_14098cbe0 as *const (), Throws::Caught), // sub_14098cbe0
    Replacement::draft_throws(0x14098ccc0, "unknown_14098ccc0", fn_14098ccc0 as *const (), Throws::Caught), // sub_14098ccc0
    Replacement::draft(0x14098cda0, "unknown_14098cda0", fn_14098cda0 as *const ()), // sub_14098cda0
    Replacement::draft(0x14098ce10, "unknown_14098ce10", tpl_7947::<0x14387e050, 0x14387e430, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_14098ce10
    Replacement::draft(0x14098ce80, "unknown_14098ce80", tpl_11823::<0x14387e430, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14098ce80
    Replacement::draft_throws(0x14098cf00, "unknown_14098cf00", tpl_7949::<0x14387e050, 0x14387e430, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_14098cf00
    Replacement::draft_throws(0x14098cf80, "unknown_14098cf80", fn_14098cf80 as *const (), Throws::Discard), // sub_14098cf80
    Replacement::draft(0x14098d080, "unknown_14098d080", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14311fae8> as *const ()), // sub_14098d080
    Replacement::draft(0x14098d0e0, "unknown_14098d0e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14311fae8> as *const ()), // sub_14098d0e0
    Replacement::draft_throws(0x14098d1e0, "unknown_14098d1e0", fn_14098d1e0 as *const (), Throws::Caught), // sub_14098d1e0
    Replacement::draft_throws(0x14098d270, "unknown_14098d270", tpl_6763::<0x14387ddf0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14098d270
    Replacement::draft(0x14098d320, "unknown_14098d320", fn_14098d320 as *const ()), // sub_14098d320
    Replacement::draft(0x14098d3b0, "unknown_14098d3b0", tpl_11829::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328a6c8, 0x18, 0x20> as *const ()), // sub_14098d3b0
    Replacement::draft(0x14098d450, "unknown_14098d450", tpl_11830::<0x14387e398, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14098d450
    Replacement::draft(0x14098d4c0, "unknown_14098d4c0", fn_14098d4c0 as *const ()), // sub_14098d4c0
    Replacement::draft_throws(0x14098d620, "unknown_14098d620", fn_14098d620 as *const (), Throws::Discard), // sub_14098d620
    Replacement::draft(0x14098d770, "unknown_14098d770", fn_14098d770 as *const ()), // sub_14098d770
    Replacement::draft_throws(0x14098d840, "unknown_14098d840", fn_14098d840 as *const (), Throws::Discard), // sub_14098d840
    Replacement::draft_throws(0x14098d9c0, "unknown_14098d9c0", fn_14098d9c0 as *const (), Throws::Caught), // sub_14098d9c0
    Replacement::draft_throws(0x14098dc20, "unknown_14098dc20", fn_14098dc20 as *const (), Throws::Caught), // sub_14098dc20
    Replacement::draft_throws(0x14098de80, "unknown_14098de80", fn_14098de80 as *const (), Throws::Caught), // sub_14098de80
    Replacement::draft(0x14098e0e0, "unknown_14098e0e0", fn_14098e0e0 as *const ()), // sub_14098e0e0
    Replacement::draft(0x14098e170, "unknown_14098e170", fn_14098e170 as *const ()), // sub_14098e170
    Replacement::draft(0x14098e240, "unknown_14098e240", fn_14098e240 as *const ()), // sub_14098e240
    Replacement::draft_throws(0x14098e320, "unknown_14098e320", fn_14098e320 as *const (), Throws::Discard), // sub_14098e320
    Replacement::draft_throws(0x14098e420, "unknown_14098e420", fn_14098e420 as *const (), Throws::Discard), // sub_14098e420
    Replacement::draft(0x14098e650, "unknown_14098e650", fn_14098e650 as *const ()), // sub_14098e650
    Replacement::draft(0x14098e730, "unknown_14098e730", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143284fc0> as *const ()), // sub_14098e730
    Replacement::draft(0x14098e790, "unknown_14098e790", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143284fc0> as *const ()), // sub_14098e790
    Replacement::draft_throws(0x14098e890, "unknown_14098e890", fn_14098e890 as *const (), Throws::Caught), // sub_14098e890
    Replacement::draft_throws(0x14098e920, "unknown_14098e920", tpl_6762::<0x14387e468, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_14098e920
    Replacement::draft(0x14098e9d0, "unknown_14098e9d0", fn_14098e9d0 as *const ()), // sub_14098e9d0
    Replacement::draft(0x14098eab0, "unknown_14098eab0", fn_14098eab0 as *const ()), // sub_14098eab0
    Replacement::draft(0x14098eba0, "unknown_14098eba0", fn_14098eba0 as *const ()), // sub_14098eba0
    Replacement::draft(0x14098ed30, "unknown_14098ed30", fn_14098ed30 as *const ()), // sub_14098ed30
    Replacement::draft_throws(0x14098ef20, "unknown_14098ef20", fn_14098ef20 as *const (), Throws::Caught), // sub_14098ef20
    Replacement::draft_throws(0x14098f1a0, "unknown_14098f1a0", fn_14098f1a0 as *const (), Throws::Discard), // sub_14098f1a0
    Replacement::draft_throws(0x14098f420, "unknown_14098f420", fn_14098f420 as *const (), Throws::Caught), // sub_14098f420
    Replacement::draft_throws(0x14098f6b0, "unknown_14098f6b0", fn_14098f6b0 as *const (), Throws::Discard), // sub_14098f6b0
    Replacement::draft_throws(0x14098f930, "unknown_14098f930", fn_14098f930 as *const (), Throws::Caught), // sub_14098f930
    Replacement::draft_throws(0x14098fb00, "unknown_14098fb00", fn_14098fb00 as *const (), Throws::Caught), // sub_14098fb00
    Replacement::draft_throws(0x14098fcd0, "unknown_14098fcd0", fn_14098fcd0 as *const (), Throws::Caught), // sub_14098fcd0
    Replacement::draft(0x14098fea0, "unknown_14098fea0", fn_14098fea0 as *const ()), // sub_14098fea0
    Replacement::draft(0x140990150, "unknown_140990150", fn_140990150 as *const ()), // sub_140990150
    Replacement::draft(0x140990450, "unknown_140990450", fn_140990450 as *const ()), // sub_140990450
    Replacement::draft_throws(0x140990770, "unknown_140990770", fn_140990770 as *const (), Throws::Discard), // sub_140990770
    Replacement::draft_throws(0x140990a50, "unknown_140990a50", fn_140990a50 as *const (), Throws::Discard), // sub_140990a50
    Replacement::draft(0x140990c20, "unknown_140990c20", fn_140990c20 as *const ()), // sub_140990c20
    Replacement::draft(0x140990d10, "unknown_140990d10", fn_140990d10 as *const ()), // sub_140990d10
    Replacement::draft(0x140990e30, "unknown_140990e30", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328a100, 0x8> as *const ()), // sub_140990e30
    Replacement::draft(0x140990ec0, "unknown_140990ec0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328a100, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140990ec0
    Replacement::draft(0x140990fe0, "unknown_140990fe0", tpl_11443::<0x14387e248, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1402c4c00, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140990fe0
    Replacement::draft_throws(0x140991060, "unknown_140991060", tpl_9636::<0x14387e408, 0x14387e248, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x48, 0x8, 0x14028e490, 0x340, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_140991060
    Replacement::draft(0x140991110, "unknown_140991110", fn_140991110 as *const ()), // sub_140991110
    Replacement::draft_throws(0x1409911d0, "unknown_1409911d0", tpl_6799::<0x14387e408, 0x14387e248, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1402c4ca0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409911d0
    Replacement::draft_throws(0x140991240, "unknown_140991240", fn_140991240 as *const (), Throws::Caught), // sub_140991240
    Replacement::draft_throws(0x1409912f0, "unknown_1409912f0", fn_1409912f0 as *const (), Throws::Caught), // sub_1409912f0
    Replacement::draft_throws(0x1409913a0, "unknown_1409913a0", fn_1409913a0 as *const (), Throws::Caught), // sub_1409913a0
    Replacement::draft(0x140991450, "unknown_140991450", fn_140991450 as *const ()), // sub_140991450
    Replacement::draft(0x140991500, "unknown_140991500", fn_140991500 as *const ()), // sub_140991500
    Replacement::draft(0x1409915a0, "unknown_1409915a0", fn_1409915a0 as *const ()), // sub_1409915a0
    Replacement::draft_throws(0x140991650, "unknown_140991650", fn_140991650 as *const (), Throws::Discard), // sub_140991650
    Replacement::draft_throws(0x140991710, "unknown_140991710", tpl_11858::<0x14387e248, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1430af558, 0x1409fde90> as *const (), Throws::Discard), // sub_140991710
    Replacement::draft(0x1409917e0, "unknown_1409917e0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143296e50, 0x8> as *const ()), // sub_1409917e0
    Replacement::draft(0x140991870, "unknown_140991870", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143296e50, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140991870
    Replacement::draft(0x140991920, "unknown_140991920", tpl_11859::<0x14387e218, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140991920
    Replacement::draft(0x140991990, "unknown_140991990", fn_140991990 as *const ()), // sub_140991990
    Replacement::draft_throws(0x140991a10, "unknown_140991a10", fn_140991a10 as *const (), Throws::Discard), // sub_140991a10
    Replacement::draft(0x140991ac0, "unknown_140991ac0", fn_140991ac0 as *const ()), // sub_140991ac0
    Replacement::draft_throws(0x140991b80, "unknown_140991b80", fn_140991b80 as *const (), Throws::Discard), // sub_140991b80
    Replacement::draft_throws(0x140991bf0, "unknown_140991bf0", fn_140991bf0 as *const (), Throws::Caught), // sub_140991bf0
    Replacement::draft_throws(0x140991ca0, "unknown_140991ca0", fn_140991ca0 as *const (), Throws::Caught), // sub_140991ca0
    Replacement::draft_throws(0x140991d50, "unknown_140991d50", fn_140991d50 as *const (), Throws::Caught), // sub_140991d50
    Replacement::draft(0x140991e00, "unknown_140991e00", fn_140991e00 as *const ()), // sub_140991e00
    Replacement::draft(0x140991eb0, "unknown_140991eb0", fn_140991eb0 as *const ()), // sub_140991eb0
    Replacement::draft(0x140991f50, "unknown_140991f50", fn_140991f50 as *const ()), // sub_140991f50
    Replacement::draft_throws(0x140992000, "unknown_140992000", fn_140992000 as *const (), Throws::Discard), // sub_140992000
    Replacement::draft_throws(0x1409920c0, "unknown_1409920c0", tpl_11868::<0x14387e218, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1430af558, 0x1409fde90> as *const (), Throws::Discard), // sub_1409920c0
    Replacement::draft(0x140992190, "unknown_140992190", tpl_11650::<0x98, 0x10, 0x18, 0xffffffffffffff80, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x28, 0x48, 0x68, 0x88, 0x143297888> as *const ()), // sub_140992190
    Replacement::draft(0x140992250, "unknown_140992250", tpl_11651::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143297888, 0xfffffffffffffffc, 0x90, 0xffffffffffffff75, 0xffffffffffffff95, 0xffffffffffffffb5> as *const ()), // sub_140992250
    Replacement::draft(0x140992340, "unknown_140992340", tpl_7312::<0x14387e2d8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140992340
    Replacement::draft(0x1409923b0, "unknown_1409923b0", fn_1409923b0 as *const ()), // sub_1409923b0
    Replacement::draft_throws(0x1409924a0, "unknown_1409924a0", fn_1409924a0 as *const (), Throws::Discard), // sub_1409924a0
    Replacement::draft(0x140992590, "unknown_140992590", fn_140992590 as *const ()), // sub_140992590
    Replacement::draft_throws(0x140992630, "unknown_140992630", fn_140992630 as *const (), Throws::Discard), // sub_140992630
    Replacement::draft_throws(0x140992720, "unknown_140992720", fn_140992720 as *const (), Throws::Caught), // sub_140992720
    Replacement::draft_throws(0x1409929c0, "unknown_1409929c0", fn_1409929c0 as *const (), Throws::Caught), // sub_1409929c0
    Replacement::draft_throws(0x140992c40, "unknown_140992c40", fn_140992c40 as *const (), Throws::Caught), // sub_140992c40
    Replacement::draft(0x140992eb0, "unknown_140992eb0", fn_140992eb0 as *const ()), // sub_140992eb0
    Replacement::draft(0x140992f80, "unknown_140992f80", fn_140992f80 as *const ()), // sub_140992f80
    Replacement::draft(0x140993010, "unknown_140993010", fn_140993010 as *const ()), // sub_140993010
    Replacement::draft_throws(0x1409930b0, "unknown_1409930b0", fn_1409930b0 as *const (), Throws::Discard), // sub_1409930b0
    Replacement::draft_throws(0x140993150, "unknown_140993150", fn_140993150 as *const (), Throws::Discard), // sub_140993150
    Replacement::draft(0x140993290, "unknown_140993290", fn_140993290 as *const ()), // sub_140993290
    Replacement::draft(0x140993330, "unknown_140993330", fn_140993330 as *const ()), // sub_140993330
    Replacement::draft(0x1409933e0, "unknown_1409933e0", fn_1409933e0 as *const ()), // sub_1409933e0
    Replacement::draft(0x1409935d0, "unknown_1409935d0", fn_1409935d0 as *const ()), // sub_1409935d0
    Replacement::draft(0x140993db0, "unknown_140993db0", fn_140993db0 as *const ()), // sub_140993db0
    Replacement::draft_throws(0x140994100, "unknown_140994100", fn_140994100 as *const (), Throws::Discard), // sub_140994100
    Replacement::draft(0x140994530, "unknown_140994530", fn_140994530 as *const ()), // sub_140994530
    Replacement::draft_throws(0x1409945f0, "unknown_1409945f0", fn_1409945f0 as *const (), Throws::Caught), // sub_1409945f0
    Replacement::draft_throws(0x140994cf0, "unknown_140994cf0", fn_140994cf0 as *const (), Throws::Caught), // sub_140994cf0
    Replacement::draft_throws(0x140995300, "unknown_140995300", fn_140995300 as *const (), Throws::Caught), // sub_140995300
    Replacement::draft(0x1409958f0, "unknown_1409958f0", fn_1409958f0 as *const ()), // sub_1409958f0
    Replacement::draft(0x140995bf0, "unknown_140995bf0", fn_140995bf0 as *const ()), // sub_140995bf0
    Replacement::draft(0x140995ef0, "unknown_140995ef0", fn_140995ef0 as *const ()), // sub_140995ef0
    Replacement::draft_throws(0x140996200, "unknown_140996200", fn_140996200 as *const (), Throws::Discard), // sub_140996200
    Replacement::draft_throws(0x140996510, "unknown_140996510", fn_140996510 as *const (), Throws::Discard), // sub_140996510
    Replacement::draft(0x140996800, "unknown_140996800", fn_140996800 as *const ()), // sub_140996800
    Replacement::draft_throws(0x1409968f0, "unknown_1409968f0", fn_1409968f0 as *const (), Throws::Discard), // sub_1409968f0
    Replacement::draft(0x140996a40, "unknown_140996a40", fn_140996a40 as *const ()), // sub_140996a40
    Replacement::draft(0x140996ae0, "unknown_140996ae0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143299b60> as *const ()), // sub_140996ae0
    Replacement::draft(0x140996b40, "unknown_140996b40", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143299b60> as *const ()), // sub_140996b40
    Replacement::draft(0x140996bd0, "unknown_140996bd0", fn_140996bd0 as *const ()), // sub_140996bd0
    Replacement::draft_throws(0x140996d10, "unknown_140996d10", tpl_11900::<0x14387e008, 0x14387dd68, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140996d10
    Replacement::draft_throws(0x140996e40, "unknown_140996e40", tpl_11900::<0x14387e008, 0x14387dd68, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140996e40
    Replacement::draft_throws(0x140996f70, "unknown_140996f70", fn_140996f70 as *const (), Throws::Caught), // sub_140996f70
    Replacement::draft(0x140997000, "unknown_140997000", fn_140997000 as *const ()), // sub_140997000
    Replacement::draft_throws(0x140997130, "unknown_140997130", tpl_11900::<0x14387e008, 0x14387dd68, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // sub_140997130
    Replacement::draft_throws(0x140997260, "unknown_140997260", tpl_11903::<0x14387dd68, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140997260
    Replacement::draft(0x140997310, "unknown_140997310", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143288210> as *const ()), // sub_140997310
    Replacement::draft(0x140997370, "unknown_140997370", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143288210> as *const ()), // sub_140997370
    Replacement::draft_throws(0x140997470, "unknown_140997470", fn_140997470 as *const (), Throws::Caught), // sub_140997470
    Replacement::draft_throws(0x140997500, "unknown_140997500", tpl_6762::<0x14387e3e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140997500
    Replacement::draft(0x1409975b0, "unknown_1409975b0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328fc18> as *const ()), // sub_1409975b0
    Replacement::draft(0x140997610, "unknown_140997610", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328fc18> as *const ()), // sub_140997610
    Replacement::draft_throws(0x140997710, "unknown_140997710", fn_140997710 as *const (), Throws::Caught), // sub_140997710
    Replacement::draft_throws(0x1409977a0, "unknown_1409977a0", tpl_6762::<0x14387e1d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409977a0
    Replacement::draft(0x140997850, "unknown_140997850", fn_140997850 as *const ()), // sub_140997850
    Replacement::draft(0x1409978e0, "unknown_1409978e0", tpl_11905::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328c058, 0x18> as *const ()), // sub_1409978e0
    Replacement::draft(0x140997a00, "unknown_140997a00", fn_140997a00 as *const ()), // sub_140997a00
    Replacement::draft_throws(0x140997b10, "unknown_140997b10", fn_140997b10 as *const (), Throws::Discard), // sub_140997b10
    Replacement::draft(0x140997c20, "unknown_140997c20", tpl_11908::<0x14387e3b8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140997c20
    Replacement::draft_throws(0x140997ce0, "unknown_140997ce0", fn_140997ce0 as *const (), Throws::Discard), // sub_140997ce0
    Replacement::draft_throws(0x140997e00, "unknown_140997e00", fn_140997e00 as *const (), Throws::Caught), // sub_140997e00
    Replacement::draft_throws(0x1409980c0, "unknown_1409980c0", fn_1409980c0 as *const (), Throws::Caught), // sub_1409980c0
    Replacement::draft_throws(0x140998350, "unknown_140998350", fn_140998350 as *const (), Throws::Caught), // sub_140998350
    Replacement::draft(0x1409985d0, "unknown_1409985d0", fn_1409985d0 as *const ()), // sub_1409985d0
    Replacement::draft(0x140998670, "unknown_140998670", fn_140998670 as *const ()), // sub_140998670
    Replacement::draft(0x140998730, "unknown_140998730", tpl_11908::<0x14387e3b8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140998730
    Replacement::draft_throws(0x1409987f0, "unknown_1409987f0", fn_1409987f0 as *const (), Throws::Discard), // sub_1409987f0
    Replacement::draft_throws(0x1409988b0, "unknown_1409988b0", fn_1409988b0 as *const (), Throws::Discard), // sub_1409988b0
    Replacement::draft_throws(0x140998a40, "unknown_140998a40", fn_140998a40 as *const (), Throws::Discard), // sub_140998a40
    Replacement::draft(0x140998b30, "unknown_140998b30", tpl_11918::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14329a598, 0x8> as *const ()), // sub_140998b30
    Replacement::draft(0x140998bc0, "unknown_140998bc0", tpl_11919::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329a598, 0x18> as *const ()), // sub_140998bc0
    Replacement::draft(0x140998ce0, "unknown_140998ce0", fn_140998ce0 as *const ()), // sub_140998ce0
    Replacement::draft_throws(0x140998d80, "unknown_140998d80", fn_140998d80 as *const (), Throws::Discard), // sub_140998d80
    Replacement::draft(0x140998e20, "unknown_140998e20", tpl_11922::<0x14387e090, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408393e0, 0x338, 0x10, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140998e20
    Replacement::draft_throws(0x140998eb0, "unknown_140998eb0", fn_140998eb0 as *const (), Throws::Discard), // sub_140998eb0
    Replacement::draft_throws(0x140998f50, "unknown_140998f50", fn_140998f50 as *const (), Throws::Caught), // sub_140998f50
    Replacement::draft_throws(0x140999070, "unknown_140999070", fn_140999070 as *const (), Throws::Caught), // sub_140999070
    Replacement::draft_throws(0x140999190, "unknown_140999190", fn_140999190 as *const (), Throws::Caught), // sub_140999190
    Replacement::draft(0x1409992b0, "unknown_1409992b0", fn_1409992b0 as *const ()), // sub_1409992b0
    Replacement::draft(0x140999330, "unknown_140999330", tpl_70397::<0x14387e56c, 0x14387e090, 0x1401e9970, 0x1406e1480, 0x8, 0x1408396d0, 0x338, 0x10, 0x120, 0x1401e9860> as *const ()), // sub_140999330
    Replacement::draft(0x1409993b0, "unknown_1409993b0", tpl_11922::<0x14387e090, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140839840, 0x338, 0x10, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409993b0
    Replacement::draft_throws(0x140999440, "unknown_140999440", fn_140999440 as *const (), Throws::Discard), // sub_140999440
    Replacement::draft_throws(0x1409994c0, "unknown_1409994c0", fn_1409994c0 as *const (), Throws::Discard), // sub_1409994c0
    Replacement::draft(0x1409995d0, "unknown_1409995d0", tpl_11929::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432839b0> as *const ()), // sub_1409995d0
    Replacement::draft(0x140999660, "unknown_140999660", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432839b0, 0x18, 0x20> as *const ()), // sub_140999660
    Replacement::draft(0x140999770, "unknown_140999770", fn_140999770 as *const ()), // sub_140999770
    Replacement::draft_throws(0x1409998b0, "unknown_1409998b0", fn_1409998b0 as *const (), Throws::Discard), // sub_1409998b0
    Replacement::draft(0x1409999f0, "unknown_1409999f0", tpl_11932::<0x14387dd70, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1409999f0
    Replacement::draft_throws(0x140999ac0, "unknown_140999ac0", fn_140999ac0 as *const (), Throws::Discard), // sub_140999ac0
    Replacement::draft_throws(0x140999c10, "unknown_140999c10", fn_140999c10 as *const (), Throws::Caught), // sub_140999c10
    Replacement::draft_throws(0x140999f20, "unknown_140999f20", fn_140999f20 as *const (), Throws::Caught), // sub_140999f20
    Replacement::draft_throws(0x14099a200, "unknown_14099a200", fn_14099a200 as *const (), Throws::Caught), // sub_14099a200
    Replacement::draft(0x14099a4d0, "unknown_14099a4d0", fn_14099a4d0 as *const ()), // sub_14099a4d0
    Replacement::draft(0x14099a570, "unknown_14099a570", fn_14099a570 as *const ()), // sub_14099a570
    Replacement::draft(0x14099a640, "unknown_14099a640", tpl_11932::<0x14387dd70, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_14099a640
    Replacement::draft_throws(0x14099a720, "unknown_14099a720", fn_14099a720 as *const (), Throws::Discard), // sub_14099a720
    Replacement::draft_throws(0x14099a7f0, "unknown_14099a7f0", fn_14099a7f0 as *const (), Throws::Discard), // sub_14099a7f0
    Replacement::draft_throws(0x14099a9c0, "unknown_14099a9c0", fn_14099a9c0 as *const (), Throws::Discard), // sub_14099a9c0
    Replacement::draft(0x14099aaf0, "unknown_14099aaf0", fn_14099aaf0 as *const ()), // sub_14099aaf0
    Replacement::draft(0x14099ab80, "unknown_14099ab80", fn_14099ab80 as *const ()), // sub_14099ab80
    Replacement::draft(0x14099ac90, "unknown_14099ac90", fn_14099ac90 as *const ()), // sub_14099ac90
    Replacement::draft_throws(0x14099ada0, "unknown_14099ada0", fn_14099ada0 as *const (), Throws::Discard), // sub_14099ada0
    Replacement::draft(0x14099ae90, "unknown_14099ae90", tpl_11945::<0x14387dd90, 0x14243b0b0, 0x1406e01b0, 0x338, 0x10, 0x38, 0x14, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14099ae90
    Replacement::draft_throws(0x14099af30, "unknown_14099af30", fn_14099af30 as *const (), Throws::Discard), // sub_14099af30
    Replacement::draft_throws(0x14099b040, "unknown_14099b040", fn_14099b040 as *const (), Throws::Caught), // sub_14099b040
    Replacement::draft_throws(0x14099b1c0, "unknown_14099b1c0", fn_14099b1c0 as *const (), Throws::Caught), // sub_14099b1c0
    Replacement::draft_throws(0x14099b340, "unknown_14099b340", fn_14099b340 as *const (), Throws::Caught), // sub_14099b340
    Replacement::draft(0x14099b4c0, "unknown_14099b4c0", fn_14099b4c0 as *const ()), // sub_14099b4c0
    Replacement::draft(0x14099b550, "unknown_14099b550", tpl_11949::<0x14387e500, 0x14387dd90, 0x1401e9970, 0x1406e1480, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x1401e9860> as *const ()), // sub_14099b550
    Replacement::draft(0x14099b5f0, "unknown_14099b5f0", tpl_11945::<0x14387dd90, 0x14243b0b0, 0x1406e01b0, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14099b5f0
    Replacement::draft_throws(0x14099b6a0, "unknown_14099b6a0", fn_14099b6a0 as *const (), Throws::Discard), // sub_14099b6a0
    Replacement::draft_throws(0x14099b760, "unknown_14099b760", fn_14099b760 as *const (), Throws::Discard), // sub_14099b760
    Replacement::draft(0x14099b8f0, "unknown_14099b8f0", fn_14099b8f0 as *const ()), // sub_14099b8f0
    Replacement::draft(0x14099b9c0, "unknown_14099b9c0", tpl_11953::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431d8a50, 0x30, 0x8, 0x1c, 0x20> as *const ()), // sub_14099b9c0
    Replacement::draft(0x14099ba70, "unknown_14099ba70", tpl_11954::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x1431d8a50, 0x5, 0x7, 0x1d, 0x1f> as *const ()), // sub_14099ba70
    Replacement::draft(0x14099bb30, "unknown_14099bb30", tpl_7312::<0x14387e578, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14099bb30
    Replacement::draft(0x14099bba0, "unknown_14099bba0", fn_14099bba0 as *const ()), // sub_14099bba0
    Replacement::draft_throws(0x14099bd30, "unknown_14099bd30", fn_14099bd30 as *const (), Throws::Discard), // sub_14099bd30
    Replacement::draft(0x14099be60, "unknown_14099be60", fn_14099be60 as *const ()), // sub_14099be60
    Replacement::draft_throws(0x14099bf20, "unknown_14099bf20", fn_14099bf20 as *const (), Throws::Discard), // sub_14099bf20
    Replacement::draft_throws(0x14099bfa0, "unknown_14099bfa0", fn_14099bfa0 as *const (), Throws::Caught), // sub_14099bfa0
    Replacement::draft_throws(0x14099c070, "unknown_14099c070", fn_14099c070 as *const (), Throws::Caught), // sub_14099c070
    Replacement::draft_throws(0x14099c140, "unknown_14099c140", fn_14099c140 as *const (), Throws::Caught), // sub_14099c140
    Replacement::draft(0x14099c210, "unknown_14099c210", fn_14099c210 as *const ()), // sub_14099c210
    Replacement::draft(0x14099c380, "unknown_14099c380", tpl_11581::<0x14387df14, 0x14387e578, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x88, 0x20, 0x1401e9860> as *const ()), // sub_14099c380
    Replacement::draft(0x14099c410, "unknown_14099c410", tpl_11577::<0x14387e578, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x88, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14099c410
    Replacement::draft_throws(0x14099c4a0, "unknown_14099c4a0", fn_14099c4a0 as *const (), Throws::Discard), // sub_14099c4a0
    Replacement::draft_throws(0x14099c540, "unknown_14099c540", fn_14099c540 as *const (), Throws::Discard), // sub_14099c540
    Replacement::draft(0x14099c660, "unknown_14099c660", fn_14099c660 as *const ()), // sub_14099c660
    Replacement::draft(0x14099c6e0, "unknown_14099c6e0", fn_14099c6e0 as *const ()), // sub_14099c6e0
    Replacement::draft(0x14099c790, "unknown_14099c790", fn_14099c790 as *const ()), // sub_14099c790
    Replacement::draft(0x14099c840, "unknown_14099c840", tpl_6748::<0x14387e0a0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14099c840
    Replacement::draft(0x14099c8b0, "unknown_14099c8b0", fn_14099c8b0 as *const ()), // sub_14099c8b0
    Replacement::draft_throws(0x14099cd20, "unknown_14099cd20", fn_14099cd20 as *const (), Throws::Discard), // sub_14099cd20
    Replacement::draft(0x14099d1b0, "unknown_14099d1b0", fn_14099d1b0 as *const ()), // sub_14099d1b0
    Replacement::draft_throws(0x14099d480, "unknown_14099d480", fn_14099d480 as *const (), Throws::Discard), // sub_14099d480
    Replacement::draft(0x14099d980, "unknown_14099d980", fn_14099d980 as *const ()), // sub_14099d980
    Replacement::draft_throws(0x14099daf0, "unknown_14099daf0", fn_14099daf0 as *const (), Throws::Caught), // sub_14099daf0
    Replacement::draft_throws(0x14099e3a0, "unknown_14099e3a0", fn_14099e3a0 as *const (), Throws::Caught), // sub_14099e3a0
    Replacement::draft_throws(0x14099ec20, "unknown_14099ec20", fn_14099ec20 as *const (), Throws::Caught), // sub_14099ec20
    Replacement::draft(0x14099f4a0, "unknown_14099f4a0", fn_14099f4a0 as *const ()), // sub_14099f4a0
    Replacement::draft(0x14099f5c0, "unknown_14099f5c0", fn_14099f5c0 as *const ()), // sub_14099f5c0
    Replacement::draft(0x14099f840, "unknown_14099f840", fn_14099f840 as *const ()), // sub_14099f840
    Replacement::draft_throws(0x14099fac0, "unknown_14099fac0", fn_14099fac0 as *const (), Throws::Discard), // sub_14099fac0
    Replacement::draft_throws(0x14099fda0, "unknown_14099fda0", fn_14099fda0 as *const (), Throws::Caught), // sub_14099fda0
    Replacement::draft(0x1409a04e0, "unknown_1409a04e0", fn_1409a04e0 as *const ()), // sub_1409a04e0
    Replacement::draft(0x1409a05c0, "unknown_1409a05c0", fn_1409a05c0 as *const ()), // sub_1409a05c0
    Replacement::draft(0x1409a08c0, "unknown_1409a08c0", fn_1409a08c0 as *const ()), // sub_1409a08c0
    Replacement::draft(0x1409a0940, "unknown_1409a0940", fn_1409a0940 as *const ()), // sub_1409a0940
    Replacement::draft(0x1409a09e0, "unknown_1409a09e0", fn_1409a09e0 as *const ()), // sub_1409a09e0
    Replacement::draft(0x1409a0a90, "unknown_1409a0a90", fn_1409a0a90 as *const ()), // sub_1409a0a90
    Replacement::draft(0x1409a0c40, "unknown_1409a0c40", fn_1409a0c40 as *const ()), // sub_1409a0c40
    Replacement::draft_throws(0x1409a0ef0, "unknown_1409a0ef0", fn_1409a0ef0 as *const (), Throws::Discard), // sub_1409a0ef0
    Replacement::draft(0x1409a11a0, "unknown_1409a11a0", fn_1409a11a0 as *const ()), // sub_1409a11a0
    Replacement::draft_throws(0x1409a13f0, "unknown_1409a13f0", fn_1409a13f0 as *const (), Throws::Discard), // sub_1409a13f0
    Replacement::draft_throws(0x1409a1690, "unknown_1409a1690", fn_1409a1690 as *const (), Throws::Caught), // sub_1409a1690
    Replacement::draft_throws(0x1409a1a70, "unknown_1409a1a70", fn_1409a1a70 as *const (), Throws::Caught), // sub_1409a1a70
    Replacement::draft_throws(0x1409a1de0, "unknown_1409a1de0", fn_1409a1de0 as *const (), Throws::Caught), // sub_1409a1de0
    Replacement::draft(0x1409a2140, "unknown_1409a2140", fn_1409a2140 as *const ()), // sub_1409a2140
    Replacement::draft(0x1409a2330, "unknown_1409a2330", fn_1409a2330 as *const ()), // sub_1409a2330
    Replacement::draft(0x1409a2580, "unknown_1409a2580", fn_1409a2580 as *const ()), // sub_1409a2580
    Replacement::draft_throws(0x1409a27e0, "unknown_1409a27e0", fn_1409a27e0 as *const (), Throws::Discard), // sub_1409a27e0
    Replacement::draft_throws(0x1409a2a10, "unknown_1409a2a10", fn_1409a2a10 as *const (), Throws::Discard), // sub_1409a2a10
    Replacement::draft_throws(0x1409a2bd0, "unknown_1409a2bd0", fn_1409a2bd0 as *const (), Throws::Discard), // sub_1409a2bd0
    Replacement::draft(0x1409a2c70, "unknown_1409a2c70", fn_1409a2c70 as *const ()), // sub_1409a2c70
    Replacement::draft(0x1409a2d00, "unknown_1409a2d00", fn_1409a2d00 as *const ()), // sub_1409a2d00
    Replacement::draft(0x1409a2db0, "unknown_1409a2db0", tpl_11830::<0x14387e448, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a2db0
    Replacement::draft(0x1409a2e20, "unknown_1409a2e20", fn_1409a2e20 as *const ()), // sub_1409a2e20
    Replacement::draft_throws(0x1409a2f30, "unknown_1409a2f30", fn_1409a2f30 as *const (), Throws::Discard), // sub_1409a2f30
    Replacement::draft(0x1409a3040, "unknown_1409a3040", fn_1409a3040 as *const ()), // sub_1409a3040
    Replacement::draft_throws(0x1409a3120, "unknown_1409a3120", fn_1409a3120 as *const (), Throws::Discard), // sub_1409a3120
    Replacement::draft_throws(0x1409a3240, "unknown_1409a3240", fn_1409a3240 as *const (), Throws::Caught), // sub_1409a3240
    Replacement::draft_throws(0x1409a3530, "unknown_1409a3530", fn_1409a3530 as *const (), Throws::Caught), // sub_1409a3530
    Replacement::draft_throws(0x1409a37f0, "unknown_1409a37f0", fn_1409a37f0 as *const (), Throws::Caught), // sub_1409a37f0
    Replacement::draft(0x1409a3ab0, "unknown_1409a3ab0", fn_1409a3ab0 as *const ()), // sub_1409a3ab0
    Replacement::draft(0x1409a3b50, "unknown_1409a3b50", fn_1409a3b50 as *const ()), // sub_1409a3b50
    Replacement::draft(0x1409a3c00, "unknown_1409a3c00", fn_1409a3c00 as *const ()), // sub_1409a3c00
    Replacement::draft_throws(0x1409a3cc0, "unknown_1409a3cc0", fn_1409a3cc0 as *const (), Throws::Discard), // sub_1409a3cc0
    Replacement::draft_throws(0x1409a3d70, "unknown_1409a3d70", fn_1409a3d70 as *const (), Throws::Discard), // sub_1409a3d70
    Replacement::draft(0x1409a3ec0, "unknown_1409a3ec0", fn_1409a3ec0 as *const ()), // sub_1409a3ec0
    Replacement::draft(0x1409a4010, "unknown_1409a4010", fn_1409a4010 as *const ()), // sub_1409a4010
    Replacement::draft(0x1409a40c0, "unknown_1409a40c0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432897b8> as *const ()), // sub_1409a40c0
    Replacement::draft(0x1409a4120, "unknown_1409a4120", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432897b8> as *const ()), // sub_1409a4120
    Replacement::draft(0x1409a41b0, "unknown_1409a41b0", fn_1409a41b0 as *const ()), // sub_1409a41b0
    Replacement::draft_throws(0x1409a4240, "unknown_1409a4240", tpl_12009::<0x14387e3b0, 0x14387df70, 0x1401e9fa0, 0x1406e02b0, 0x1409fe2e0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409a4240
    Replacement::draft_throws(0x1409a42c0, "unknown_1409a42c0", tpl_12009::<0x14387e3b0, 0x14387df70, 0x1401e9b60, 0x1406e03a0, 0x1409fe2e0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409a42c0
    Replacement::draft_throws(0x1409a4340, "unknown_1409a4340", fn_1409a4340 as *const (), Throws::Caught), // sub_1409a4340
    Replacement::draft(0x1409a43d0, "unknown_1409a43d0", fn_1409a43d0 as *const ()), // sub_1409a43d0
    Replacement::draft_throws(0x1409a4450, "unknown_1409a4450", tpl_12009::<0x14387e3b0, 0x14387df70, 0x1401e9770, 0x1406e1570, 0x1409fe2e0, 0x1401e9590> as *const (), Throws::Discard), // sub_1409a4450
    Replacement::draft_throws(0x1409a44d0, "unknown_1409a44d0", tpl_6823::<0x14387df70, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409a44d0
    Replacement::draft(0x1409a4580, "unknown_1409a4580", tpl_11584::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143290408, 0x20, 0x28, 0xb> as *const ()), // sub_1409a4580
    Replacement::draft(0x1409a4630, "unknown_1409a4630", tpl_11585::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143290408, 0xfffffffffffffff4, 0x14, 0xfffffffffffffffc, 0x4, 0x30> as *const ()), // sub_1409a4630
    Replacement::draft(0x1409a4760, "unknown_1409a4760", tpl_9966::<0x14387e010, 0x14243b0b0, 0x1406e01b0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a4760
    Replacement::draft_throws(0x1409a4820, "unknown_1409a4820", fn_1409a4820 as *const (), Throws::Discard), // sub_1409a4820
    Replacement::draft(0x1409a48d0, "unknown_1409a48d0", tpl_9969::<0x14387e010, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a48d0
    Replacement::draft_throws(0x1409a4950, "unknown_1409a4950", fn_1409a4950 as *const (), Throws::Discard), // sub_1409a4950
    Replacement::draft_throws(0x1409a4a00, "unknown_1409a4a00", fn_1409a4a00 as *const (), Throws::Caught), // sub_1409a4a00
    Replacement::draft_throws(0x1409a4c70, "unknown_1409a4c70", fn_1409a4c70 as *const (), Throws::Caught), // sub_1409a4c70
    Replacement::draft_throws(0x1409a4ea0, "unknown_1409a4ea0", fn_1409a4ea0 as *const (), Throws::Caught), // sub_1409a4ea0
    Replacement::draft(0x1409a50c0, "unknown_1409a50c0", fn_1409a50c0 as *const ()), // sub_1409a50c0
    Replacement::draft(0x1409a5160, "unknown_1409a5160", tpl_7856::<0x14387e070, 0x14387e010, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_1409a5160
    Replacement::draft(0x1409a51d0, "unknown_1409a51d0", tpl_9969::<0x14387e010, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a51d0
    Replacement::draft_throws(0x1409a5250, "unknown_1409a5250", tpl_9971::<0x14387e070, 0x14387e010, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_1409a5250
    Replacement::draft_throws(0x1409a52c0, "unknown_1409a52c0", tpl_11590::<0x14387e010, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a3590, 0x8, 0x1426cdcb0, 0x1432a3658, 0x1432a3638, 0x1402c4730> as *const (), Throws::Discard), // sub_1409a52c0
    Replacement::draft(0x1409a53c0, "unknown_1409a53c0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14328f650, 0x8> as *const ()), // sub_1409a53c0
    Replacement::draft(0x1409a5440, "unknown_1409a5440", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328f650> as *const ()), // sub_1409a5440
    Replacement::draft(0x1409a5540, "unknown_1409a5540", tpl_11372::<0x14387ddf8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f16c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a5540
    Replacement::draft_throws(0x1409a55c0, "unknown_1409a55c0", tpl_8772::<0x14387e3a0, 0x14387ddf8, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1409fe390, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409a55c0
    Replacement::draft(0x1409a5630, "unknown_1409a5630", tpl_11372::<0x14387ddf8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f1a30, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a5630
    Replacement::draft_throws(0x1409a56b0, "unknown_1409a56b0", tpl_8772::<0x14387e3a0, 0x14387ddf8, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1409fe4c0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409a56b0
    Replacement::draft_throws(0x1409a5720, "unknown_1409a5720", fn_1409a5720 as *const (), Throws::Caught), // sub_1409a5720
    Replacement::draft_throws(0x1409a57f0, "unknown_1409a57f0", fn_1409a57f0 as *const (), Throws::Caught), // sub_1409a57f0
    Replacement::draft_throws(0x1409a58c0, "unknown_1409a58c0", fn_1409a58c0 as *const (), Throws::Caught), // sub_1409a58c0
    Replacement::draft(0x1409a5990, "unknown_1409a5990", tpl_11372::<0x14387ddf8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f22e0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a5990
    Replacement::draft(0x1409a5a10, "unknown_1409a5a10", tpl_6804::<0x14387e3a0, 0x14387ddf8, 0x1401e9970, 0x1406e1480, 0x8, 0x1408f2470, 0x1401e9860> as *const ()), // sub_1409a5a10
    Replacement::draft(0x1409a5a80, "unknown_1409a5a80", tpl_11372::<0x14387ddf8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f25e0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a5a80
    Replacement::draft_throws(0x1409a5b00, "unknown_1409a5b00", tpl_6805::<0x14387e3a0, 0x14387ddf8, 0x1401e9770, 0x1406e1570, 0x8, 0x1408f2730, 0x1401e9590> as *const (), Throws::Discard), // sub_1409a5b00
    Replacement::draft_throws(0x1409a5b70, "unknown_1409a5b70", fn_1409a5b70 as *const (), Throws::Discard), // sub_1409a5b70
    Replacement::draft(0x1409a5c30, "unknown_1409a5c30", tpl_7618::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143293880, 0x20> as *const ()), // sub_1409a5c30
    Replacement::draft(0x1409a5cc0, "unknown_1409a5cc0", tpl_12018::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143293880, 0x18, 0x20, 0x28> as *const ()), // sub_1409a5cc0
    Replacement::draft(0x1409a5de0, "unknown_1409a5de0", fn_1409a5de0 as *const ()), // sub_1409a5de0
    Replacement::draft_throws(0x1409a5fb0, "unknown_1409a5fb0", fn_1409a5fb0 as *const (), Throws::Discard), // sub_1409a5fb0
    Replacement::draft(0x1409a6130, "unknown_1409a6130", fn_1409a6130 as *const ()), // sub_1409a6130
    Replacement::draft_throws(0x1409a6210, "unknown_1409a6210", fn_1409a6210 as *const (), Throws::Discard), // sub_1409a6210
    Replacement::draft_throws(0x1409a63d0, "unknown_1409a63d0", fn_1409a63d0 as *const (), Throws::Caught), // sub_1409a63d0
    Replacement::draft_throws(0x1409a6700, "unknown_1409a6700", fn_1409a6700 as *const (), Throws::Caught), // sub_1409a6700
    Replacement::draft_throws(0x1409a6a30, "unknown_1409a6a30", fn_1409a6a30 as *const (), Throws::Caught), // sub_1409a6a30
    Replacement::draft(0x1409a6d60, "unknown_1409a6d60", fn_1409a6d60 as *const ()), // sub_1409a6d60
    Replacement::draft(0x1409a6df0, "unknown_1409a6df0", fn_1409a6df0 as *const ()), // sub_1409a6df0
    Replacement::draft(0x1409a6ee0, "unknown_1409a6ee0", fn_1409a6ee0 as *const ()), // sub_1409a6ee0
    Replacement::draft_throws(0x1409a6fe0, "unknown_1409a6fe0", fn_1409a6fe0 as *const (), Throws::Discard), // sub_1409a6fe0
    Replacement::draft_throws(0x1409a7100, "unknown_1409a7100", fn_1409a7100 as *const (), Throws::Discard), // sub_1409a7100
    Replacement::draft(0x1409a7450, "unknown_1409a7450", fn_1409a7450 as *const ()), // sub_1409a7450
    Replacement::draft(0x1409a7580, "unknown_1409a7580", tpl_11457::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x143295bc0, 0x8> as *const ()), // sub_1409a7580
    Replacement::draft(0x1409a7610, "unknown_1409a7610", tpl_11458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143295bc0, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409a7610
    Replacement::draft(0x1409a7730, "unknown_1409a7730", tpl_11459::<0x14387e0b0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a7730
    Replacement::draft_throws(0x1409a77e0, "unknown_1409a77e0", tpl_11460::<0x14387dda8, 0x14387e0b0, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x8, 0x338, 0x38, 0x4, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409a77e0
    Replacement::draft(0x1409a7890, "unknown_1409a7890", tpl_11461::<0x14387e0b0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a7890
    Replacement::draft_throws(0x1409a7930, "unknown_1409a7930", tpl_11462::<0x14387dda8, 0x14387e0b0, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409a7930
    Replacement::draft_throws(0x1409a79d0, "unknown_1409a79d0", fn_1409a79d0 as *const (), Throws::Caught), // sub_1409a79d0
    Replacement::draft_throws(0x1409a7bd0, "unknown_1409a7bd0", fn_1409a7bd0 as *const (), Throws::Caught), // sub_1409a7bd0
    Replacement::draft_throws(0x1409a7da0, "unknown_1409a7da0", fn_1409a7da0 as *const (), Throws::Caught), // sub_1409a7da0
    Replacement::draft(0x1409a7f60, "unknown_1409a7f60", fn_1409a7f60 as *const ()), // sub_1409a7f60
    Replacement::draft(0x1409a7fe0, "unknown_1409a7fe0", tpl_11467::<0x14387dda8, 0x14387e0b0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1401e9860> as *const ()), // sub_1409a7fe0
    Replacement::draft(0x1409a8080, "unknown_1409a8080", tpl_11461::<0x14387e0b0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409a8080
    Replacement::draft_throws(0x1409a8130, "unknown_1409a8130", tpl_11468::<0x14387dda8, 0x14387e0b0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x10, 0x1401e9590> as *const (), Throws::Discard), // sub_1409a8130
    Replacement::draft_throws(0x1409a81b0, "unknown_1409a81b0", tpl_11469::<0x14387e0b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x4, 0x1432a3528, 0x1413c41a0> as *const (), Throws::Discard), // sub_1409a81b0
    Replacement::draft(0x1409a82b0, "unknown_1409a82b0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143283cc8> as *const ()), // sub_1409a82b0
    Replacement::draft(0x1409a8310, "unknown_1409a8310", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143283cc8> as *const ()), // sub_1409a8310
    Replacement::draft_throws(0x1409a8410, "unknown_1409a8410", fn_1409a8410 as *const (), Throws::Caught), // sub_1409a8410
    Replacement::draft_throws(0x1409a84a0, "unknown_1409a84a0", tpl_6763::<0x14387ddd0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409a84a0
    Replacement::draft(0x1409a8550, "unknown_1409a8550", fn_1409a8550 as *const ()), // sub_1409a8550
    Replacement::draft(0x1409a8600, "unknown_1409a8600", fn_1409a8600 as *const ()), // sub_1409a8600
    Replacement::draft(0x1409a86d0, "unknown_1409a86d0", fn_1409a86d0 as *const ()), // sub_1409a86d0
    Replacement::draft(0x1409a87e0, "unknown_1409a87e0", fn_1409a87e0 as *const ()), // sub_1409a87e0
    Replacement::draft_throws(0x1409a8b50, "unknown_1409a8b50", fn_1409a8b50 as *const (), Throws::Discard), // sub_1409a8b50
    Replacement::draft(0x1409a8e30, "unknown_1409a8e30", fn_1409a8e30 as *const ()), // sub_1409a8e30
    Replacement::draft_throws(0x1409a9050, "unknown_1409a9050", fn_1409a9050 as *const (), Throws::Discard), // sub_1409a9050
    Replacement::draft(0x1409a9380, "unknown_1409a9380", fn_1409a9380 as *const ()), // sub_1409a9380
    Replacement::draft_throws(0x1409a9570, "unknown_1409a9570", fn_1409a9570 as *const (), Throws::Caught), // sub_1409a9570
    Replacement::draft_throws(0x1409a9c00, "unknown_1409a9c00", fn_1409a9c00 as *const (), Throws::Caught), // sub_1409a9c00
    Replacement::draft_throws(0x1409aa1e0, "unknown_1409aa1e0", fn_1409aa1e0 as *const (), Throws::Caught), // sub_1409aa1e0
    Replacement::draft(0x1409aa7b0, "unknown_1409aa7b0", fn_1409aa7b0 as *const ()), // sub_1409aa7b0
    Replacement::draft(0x1409aa8e0, "unknown_1409aa8e0", fn_1409aa8e0 as *const ()), // sub_1409aa8e0
    Replacement::draft(0x1409aab10, "unknown_1409aab10", fn_1409aab10 as *const ()), // sub_1409aab10
    Replacement::draft_throws(0x1409aad50, "unknown_1409aad50", fn_1409aad50 as *const (), Throws::Discard), // sub_1409aad50
    Replacement::draft_throws(0x1409aafb0, "unknown_1409aafb0", fn_1409aafb0 as *const (), Throws::Caught), // sub_1409aafb0
    Replacement::draft(0x1409ab4a0, "unknown_1409ab4a0", fn_1409ab4a0 as *const ()), // sub_1409ab4a0
    Replacement::draft_throws(0x1409ab590, "unknown_1409ab590", fn_1409ab590 as *const (), Throws::Discard), // sub_1409ab590
    Replacement::draft(0x1409ab750, "unknown_1409ab750", fn_1409ab750 as *const ()), // sub_1409ab750
    Replacement::draft(0x1409ab7f0, "unknown_1409ab7f0", fn_1409ab7f0 as *const ()), // sub_1409ab7f0
    Replacement::draft(0x1409ab8d0, "unknown_1409ab8d0", fn_1409ab8d0 as *const ()), // sub_1409ab8d0
    Replacement::draft(0x1409ab9b0, "unknown_1409ab9b0", fn_1409ab9b0 as *const ()), // sub_1409ab9b0
    Replacement::draft(0x1409abac0, "unknown_1409abac0", fn_1409abac0 as *const ()), // sub_1409abac0
    Replacement::draft_throws(0x1409abe90, "unknown_1409abe90", fn_1409abe90 as *const (), Throws::Discard), // sub_1409abe90
    Replacement::draft(0x1409ac250, "unknown_1409ac250", fn_1409ac250 as *const ()), // sub_1409ac250
    Replacement::draft_throws(0x1409ac590, "unknown_1409ac590", fn_1409ac590 as *const (), Throws::Discard), // sub_1409ac590
    Replacement::draft(0x1409ac830, "unknown_1409ac830", fn_1409ac830 as *const ()), // sub_1409ac830
    Replacement::draft_throws(0x1409aca50, "unknown_1409aca50", fn_1409aca50 as *const (), Throws::Caught), // sub_1409aca50
    Replacement::draft_throws(0x1409acbf0, "unknown_1409acbf0", fn_1409acbf0 as *const (), Throws::Caught), // sub_1409acbf0
    Replacement::draft_throws(0x1409acd90, "unknown_1409acd90", fn_1409acd90 as *const (), Throws::Caught), // sub_1409acd90
    Replacement::draft(0x1409acf30, "unknown_1409acf30", fn_1409acf30 as *const ()), // sub_1409acf30
    Replacement::draft(0x1409ad230, "unknown_1409ad230", fn_1409ad230 as *const ()), // sub_1409ad230
    Replacement::draft(0x1409ad530, "unknown_1409ad530", fn_1409ad530 as *const ()), // sub_1409ad530
    Replacement::draft_throws(0x1409ad840, "unknown_1409ad840", fn_1409ad840 as *const (), Throws::Discard), // sub_1409ad840
    Replacement::draft_throws(0x1409adb90, "unknown_1409adb90", fn_1409adb90 as *const (), Throws::Discard), // sub_1409adb90
    Replacement::draft(0x1409add50, "unknown_1409add50", fn_1409add50 as *const ()), // sub_1409add50
    Replacement::draft(0x1409adeb0, "unknown_1409adeb0", fn_1409adeb0 as *const ()), // sub_1409adeb0
    Replacement::draft(0x1409adf80, "unknown_1409adf80", tpl_11918::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432928a0, 0x8> as *const ()), // sub_1409adf80
    Replacement::draft(0x1409ae010, "unknown_1409ae010", tpl_11919::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432928a0, 0x18> as *const ()), // sub_1409ae010
    Replacement::draft(0x1409ae0c0, "unknown_1409ae0c0", tpl_12060::<0x14387e0d0, 0x14243b0b0, 0x1406e01b0, 0x1409ff110, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409ae0c0
    Replacement::draft(0x1409ae130, "unknown_1409ae130", fn_1409ae130 as *const ()), // sub_1409ae130
    Replacement::draft_throws(0x1409ae220, "unknown_1409ae220", fn_1409ae220 as *const (), Throws::Discard), // sub_1409ae220
    Replacement::draft(0x1409ae2f0, "unknown_1409ae2f0", tpl_12063::<0x14387e0d0, 0x14243b0b0, 0x1406e01b0, 0x1409ff110, 0x8, 0x1409122b0, 0x338, 0x10, 0x80, 0x14, 0x38, 0xfffffffffffffff8> as *const ()), // sub_1409ae2f0
    Replacement::draft_throws(0x1409ae390, "unknown_1409ae390", fn_1409ae390 as *const (), Throws::Discard), // sub_1409ae390
    Replacement::draft_throws(0x1409ae480, "unknown_1409ae480", fn_1409ae480 as *const (), Throws::Caught), // sub_1409ae480
    Replacement::draft_throws(0x1409ae5f0, "unknown_1409ae5f0", fn_1409ae5f0 as *const (), Throws::Caught), // sub_1409ae5f0
    Replacement::draft_throws(0x1409ae760, "unknown_1409ae760", fn_1409ae760 as *const (), Throws::Caught), // sub_1409ae760
    Replacement::draft(0x1409ae8d0, "unknown_1409ae8d0", fn_1409ae8d0 as *const ()), // sub_1409ae8d0
    Replacement::draft(0x1409ae960, "unknown_1409ae960", fn_1409ae960 as *const ()), // sub_1409ae960
    Replacement::draft(0x1409aea00, "unknown_1409aea00", tpl_12063::<0x14387e0d0, 0x14243b0b0, 0x1406e01b0, 0x1409ff110, 0x8, 0x140912e60, 0x338, 0x10, 0x120, 0x14, 0xd8, 0xfffffffffffffff8> as *const ()), // sub_1409aea00
    Replacement::draft_throws(0x1409aeab0, "unknown_1409aeab0", fn_1409aeab0 as *const (), Throws::Discard), // sub_1409aeab0
    Replacement::draft_throws(0x1409aeb60, "unknown_1409aeb60", fn_1409aeb60 as *const (), Throws::Discard), // sub_1409aeb60
    Replacement::draft(0x1409aecb0, "unknown_1409aecb0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143282878> as *const ()), // sub_1409aecb0
    Replacement::draft(0x1409aed10, "unknown_1409aed10", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143282878> as *const ()), // sub_1409aed10
    Replacement::draft_throws(0x1409aee10, "unknown_1409aee10", fn_1409aee10 as *const (), Throws::Caught), // sub_1409aee10
    Replacement::draft_throws(0x1409aeea0, "unknown_1409aeea0", tpl_6763::<0x14387e4c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409aeea0
    Replacement::draft(0x1409aef50, "unknown_1409aef50", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143285588, 0x8> as *const ()), // sub_1409aef50
    Replacement::draft(0x1409aefd0, "unknown_1409aefd0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143285588> as *const ()), // sub_1409aefd0
    Replacement::draft(0x1409af060, "unknown_1409af060", fn_1409af060 as *const ()), // sub_1409af060
    Replacement::draft(0x1409af180, "unknown_1409af180", tpl_12071::<0x14387e570, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x2, 0x1409fa060, 0x8, 0x140559ea0, 0xfffffffffffffff8> as *const ()), // sub_1409af180
    Replacement::draft_throws(0x1409af270, "unknown_1409af270", tpl_12072::<0x14387e1b8, 0x14387e570, 0x1401e9fa0, 0x1406e02b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x2, 0x1409fa060, 0x8, 0x140557390, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409af270
    Replacement::draft(0x1409af360, "unknown_1409af360", tpl_12071::<0x14387e570, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x2, 0x1409fa060, 0x8, 0x14055a030, 0xfffffffffffffff8> as *const ()), // sub_1409af360
    Replacement::draft_throws(0x1409af450, "unknown_1409af450", tpl_12072::<0x14387e1b8, 0x14387e570, 0x1401e9b60, 0x1406e03a0, 0x318, 0x328, 0x320, 0x1406e3600, 0x2, 0x1409fa060, 0x8, 0x1405575f0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409af450
    Replacement::draft_throws(0x1409af540, "unknown_1409af540", fn_1409af540 as *const (), Throws::Caught), // sub_1409af540
    Replacement::draft_throws(0x1409af5f0, "unknown_1409af5f0", fn_1409af5f0 as *const (), Throws::Caught), // sub_1409af5f0
    Replacement::draft_throws(0x1409af6a0, "unknown_1409af6a0", fn_1409af6a0 as *const (), Throws::Caught), // sub_1409af6a0
    Replacement::draft(0x1409af750, "unknown_1409af750", fn_1409af750 as *const ()), // sub_1409af750
    Replacement::draft(0x1409af850, "unknown_1409af850", fn_1409af850 as *const ()), // sub_1409af850
    Replacement::draft(0x1409af970, "unknown_1409af970", tpl_12071::<0x14387e570, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x2, 0x1409fa060, 0x8, 0x14055a1a0, 0xfffffffffffffff8> as *const ()), // sub_1409af970
    Replacement::draft_throws(0x1409afa60, "unknown_1409afa60", fn_1409afa60 as *const (), Throws::Discard), // sub_1409afa60
    Replacement::draft_throws(0x1409afb50, "unknown_1409afb50", fn_1409afb50 as *const (), Throws::Discard), // sub_1409afb50
    Replacement::draft(0x1409afc20, "unknown_1409afc20", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143284450, 0x8> as *const ()), // sub_1409afc20
    Replacement::draft(0x1409afca0, "unknown_1409afca0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143284450> as *const ()), // sub_1409afca0
    Replacement::draft(0x1409afd30, "unknown_1409afd30", tpl_9031::<0x14387e300, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409afd30
    Replacement::draft(0x1409afda0, "unknown_1409afda0", tpl_12078::<0x14387e300, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409afda0
    Replacement::draft_throws(0x1409afe40, "unknown_1409afe40", tpl_12079::<0x14387e15c, 0x14387e300, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409afe40
    Replacement::draft(0x1409afed0, "unknown_1409afed0", tpl_12080::<0x14387e300, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409afed0
    Replacement::draft_throws(0x1409aff50, "unknown_1409aff50", tpl_12081::<0x14387e15c, 0x14387e300, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409aff50
    Replacement::draft_throws(0x1409affe0, "unknown_1409affe0", fn_1409affe0 as *const (), Throws::Caught), // sub_1409affe0
    Replacement::draft_throws(0x1409b00c0, "unknown_1409b00c0", fn_1409b00c0 as *const (), Throws::Caught), // sub_1409b00c0
    Replacement::draft_throws(0x1409b01a0, "unknown_1409b01a0", fn_1409b01a0 as *const (), Throws::Caught), // sub_1409b01a0
    Replacement::draft(0x1409b0280, "unknown_1409b0280", tpl_12083::<0x14387e300, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409b0280
    Replacement::draft(0x1409b02f0, "unknown_1409b02f0", tpl_12084::<0x14387e15c, 0x14387e300, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_1409b02f0
    Replacement::draft(0x1409b0360, "unknown_1409b0360", tpl_12080::<0x14387e300, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409b0360
    Replacement::draft_throws(0x1409b03e0, "unknown_1409b03e0", tpl_12085::<0x14387e15c, 0x14387e300, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409b03e0
    Replacement::draft_throws(0x1409b0460, "unknown_1409b0460", tpl_12086::<0x14387e300, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x14329ef90, 0x14329ef58, 0x1432a2b48, 0x14020f0f0> as *const (), Throws::Discard), // sub_1409b0460
    Replacement::draft(0x1409b0560, "unknown_1409b0560", fn_1409b0560 as *const ()), // sub_1409b0560
    Replacement::draft(0x1409b0660, "unknown_1409b0660", fn_1409b0660 as *const ()), // sub_1409b0660
    Replacement::draft(0x1409b07e0, "unknown_1409b07e0", fn_1409b07e0 as *const ()), // sub_1409b07e0
    Replacement::draft(0x1409b08b0, "unknown_1409b08b0", fn_1409b08b0 as *const ()), // sub_1409b08b0
    Replacement::draft_throws(0x1409b0bb0, "unknown_1409b0bb0", fn_1409b0bb0 as *const (), Throws::Discard), // sub_1409b0bb0
    Replacement::draft(0x1409b0eb0, "unknown_1409b0eb0", fn_1409b0eb0 as *const ()), // sub_1409b0eb0
    Replacement::draft_throws(0x1409b10a0, "unknown_1409b10a0", fn_1409b10a0 as *const (), Throws::Discard), // sub_1409b10a0
    Replacement::draft(0x1409b13d0, "unknown_1409b13d0", fn_1409b13d0 as *const ()), // sub_1409b13d0
    Replacement::draft_throws(0x1409b14f0, "unknown_1409b14f0", fn_1409b14f0 as *const (), Throws::Caught), // sub_1409b14f0
    Replacement::draft_throws(0x1409b1bc0, "unknown_1409b1bc0", fn_1409b1bc0 as *const (), Throws::Caught), // sub_1409b1bc0
    Replacement::draft_throws(0x1409b2210, "unknown_1409b2210", fn_1409b2210 as *const (), Throws::Caught), // sub_1409b2210
    Replacement::draft(0x1409b2820, "unknown_1409b2820", fn_1409b2820 as *const ()), // sub_1409b2820
    Replacement::draft(0x1409b29d0, "unknown_1409b29d0", fn_1409b29d0 as *const ()), // sub_1409b29d0
    Replacement::draft(0x1409b2bd0, "unknown_1409b2bd0", fn_1409b2bd0 as *const ()), // sub_1409b2bd0
    Replacement::draft_throws(0x1409b2dd0, "unknown_1409b2dd0", fn_1409b2dd0 as *const (), Throws::Discard), // sub_1409b2dd0
    Replacement::draft_throws(0x1409b2fd0, "unknown_1409b2fd0", fn_1409b2fd0 as *const (), Throws::Discard), // sub_1409b2fd0
    Replacement::draft(0x1409b32c0, "unknown_1409b32c0", fn_1409b32c0 as *const ()), // sub_1409b32c0
    Replacement::draft_throws(0x1409b3380, "unknown_1409b3380", fn_1409b3380 as *const (), Throws::Discard), // sub_1409b3380
    Replacement::draft(0x1409b3440, "unknown_1409b3440", tpl_70565::<0x130, 0x138, 0x1432a2b80, 0x1409fb590> as *const ()), // sub_1409b3440
    Replacement::draft(0x1409b34c0, "unknown_1409b34c0", fn_1409b34c0 as *const ()), // sub_1409b34c0
    Replacement::draft(0x1409b3570, "unknown_1409b3570", fn_1409b3570 as *const ()), // sub_1409b3570
    Replacement::draft(0x1409b3630, "unknown_1409b3630", tpl_11241::<0x14387e2a8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409b3630
    Replacement::draft(0x1409b36a0, "unknown_1409b36a0", fn_1409b36a0 as *const ()), // sub_1409b36a0
    Replacement::draft_throws(0x1409b3b30, "unknown_1409b3b30", fn_1409b3b30 as *const (), Throws::Discard), // sub_1409b3b30
    Replacement::draft(0x1409b3f30, "unknown_1409b3f30", fn_1409b3f30 as *const ()), // sub_1409b3f30
    Replacement::draft_throws(0x1409b4170, "unknown_1409b4170", fn_1409b4170 as *const (), Throws::Discard), // sub_1409b4170
    Replacement::draft(0x1409b4620, "unknown_1409b4620", fn_1409b4620 as *const ()), // sub_1409b4620
    Replacement::draft_throws(0x1409b47b0, "unknown_1409b47b0", fn_1409b47b0 as *const (), Throws::Caught), // sub_1409b47b0
    Replacement::draft_throws(0x1409b52b0, "unknown_1409b52b0", fn_1409b52b0 as *const (), Throws::Caught), // sub_1409b52b0
    Replacement::draft_throws(0x1409b5c70, "unknown_1409b5c70", fn_1409b5c70 as *const (), Throws::Caught), // sub_1409b5c70
    Replacement::draft(0x1409b6620, "unknown_1409b6620", fn_1409b6620 as *const ()), // sub_1409b6620
    Replacement::draft(0x1409b66f0, "unknown_1409b66f0", fn_1409b66f0 as *const ()), // sub_1409b66f0
    Replacement::draft(0x1409b6970, "unknown_1409b6970", fn_1409b6970 as *const ()), // sub_1409b6970
    Replacement::draft_throws(0x1409b6c10, "unknown_1409b6c10", fn_1409b6c10 as *const (), Throws::Discard), // sub_1409b6c10
    Replacement::draft_throws(0x1409b6f00, "unknown_1409b6f00", fn_1409b6f00 as *const (), Throws::Discard), // sub_1409b6f00
    Replacement::draft_throws(0x1409b7540, "unknown_1409b7540", fn_1409b7540 as *const (), Throws::Discard), // sub_1409b7540
    Replacement::draft(0x1409b7990, "unknown_1409b7990", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14329a280, 0x8> as *const ()), // sub_1409b7990
    Replacement::draft(0x1409b7a20, "unknown_1409b7a20", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329a280, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409b7a20
    Replacement::draft(0x1409b7b40, "unknown_1409b7b40", fn_1409b7b40 as *const ()), // sub_1409b7b40
    Replacement::draft_throws(0x1409b7cb0, "unknown_1409b7cb0", fn_1409b7cb0 as *const (), Throws::Discard), // sub_1409b7cb0
    Replacement::draft(0x1409b7df0, "unknown_1409b7df0", fn_1409b7df0 as *const ()), // sub_1409b7df0
    Replacement::draft_throws(0x1409b7ee0, "unknown_1409b7ee0", fn_1409b7ee0 as *const (), Throws::Caught), // sub_1409b7ee0
    Replacement::draft(0x1409b8090, "unknown_1409b8090", tpl_12118::<0x8, 0x140a02850, 0x1401e84a0> as *const ()), // sub_1409b8090
    Replacement::draft_throws(0x1409b8100, "unknown_1409b8100", fn_1409b8100 as *const (), Throws::Caught), // sub_1409b8100
    Replacement::draft_throws(0x1409b81b0, "unknown_1409b81b0", fn_1409b81b0 as *const (), Throws::Caught), // sub_1409b81b0
    Replacement::draft_throws(0x1409b8260, "unknown_1409b8260", fn_1409b8260 as *const (), Throws::Caught), // sub_1409b8260
    Replacement::draft(0x1409b8310, "unknown_1409b8310", fn_1409b8310 as *const ()), // sub_1409b8310
    Replacement::draft(0x1409b8470, "unknown_1409b8470", fn_1409b8470 as *const ()), // sub_1409b8470
    Replacement::draft(0x1409b8530, "unknown_1409b8530", fn_1409b8530 as *const ()), // sub_1409b8530
    Replacement::draft_throws(0x1409b8600, "unknown_1409b8600", fn_1409b8600 as *const (), Throws::Discard), // sub_1409b8600
    Replacement::draft_throws(0x1409b86b0, "unknown_1409b86b0", tpl_11858::<0x14387de38, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432a44b0, 0x140943880> as *const (), Throws::Discard), // sub_1409b86b0
    Replacement::draft(0x1409b8780, "unknown_1409b8780", fn_1409b8780 as *const ()), // sub_1409b8780
    Replacement::draft(0x1409b8800, "unknown_1409b8800", tpl_11953::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328e578, 0x30, 0x8, 0x1c, 0x20> as *const ()), // sub_1409b8800
    Replacement::draft(0x1409b88b0, "unknown_1409b88b0", tpl_11954::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x14328e578, 0x5, 0x7, 0x1d, 0x1f> as *const ()), // sub_1409b88b0
    Replacement::draft(0x1409b89e0, "unknown_1409b89e0", fn_1409b89e0 as *const ()), // sub_1409b89e0
    Replacement::draft_throws(0x1409b8b00, "unknown_1409b8b00", fn_1409b8b00 as *const (), Throws::Discard), // sub_1409b8b00
    Replacement::draft(0x1409b8c30, "unknown_1409b8c30", fn_1409b8c30 as *const ()), // sub_1409b8c30
    Replacement::draft_throws(0x1409b8cf0, "unknown_1409b8cf0", fn_1409b8cf0 as *const (), Throws::Caught), // sub_1409b8cf0
    Replacement::draft_throws(0x1409b8e20, "unknown_1409b8e20", fn_1409b8e20 as *const (), Throws::Caught), // sub_1409b8e20
    Replacement::draft_throws(0x1409b9370, "unknown_1409b9370", fn_1409b9370 as *const (), Throws::Caught), // sub_1409b9370
    Replacement::draft_throws(0x1409b9850, "unknown_1409b9850", fn_1409b9850 as *const (), Throws::Caught), // sub_1409b9850
    Replacement::draft(0x1409b9900, "unknown_1409b9900", fn_1409b9900 as *const ()), // sub_1409b9900
    Replacement::draft(0x1409b9a20, "unknown_1409b9a20", tpl_8026::<0x14387e438, 0x14387e3f0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x88, 0x20, 0x1401e9860> as *const ()), // sub_1409b9a20
    Replacement::draft(0x1409b9ab0, "unknown_1409b9ab0", tpl_11593::<0x14387e3f0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x88, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409b9ab0
    Replacement::draft_throws(0x1409b9b40, "unknown_1409b9b40", fn_1409b9b40 as *const (), Throws::Discard), // sub_1409b9b40
    Replacement::draft_throws(0x1409b9be0, "unknown_1409b9be0", fn_1409b9be0 as *const (), Throws::Discard), // sub_1409b9be0
    Replacement::draft(0x1409b9cf0, "unknown_1409b9cf0", tpl_7173::<0x1432a4530, 0x8, 0x1, 0x140947eb0, 0x20, 0x1432a4540> as *const ()), // sub_1409b9cf0
    Replacement::draft(0x1409b9d60, "unknown_1409b9d60", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432993d8, 0x8> as *const ()), // sub_1409b9d60
    Replacement::draft(0x1409b9de0, "unknown_1409b9de0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432993d8> as *const ()), // sub_1409b9de0
    Replacement::draft(0x1409b9ee0, "unknown_1409b9ee0", tpl_12130::<0x14387de10, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409b9ee0
    Replacement::draft_throws(0x1409b9f90, "unknown_1409b9f90", tpl_7951::<0x14387def0, 0x14387de10, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x38, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409b9f90
    Replacement::draft(0x1409ba020, "unknown_1409ba020", tpl_9969::<0x14387de10, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409ba020
    Replacement::draft_throws(0x1409ba0a0, "unknown_1409ba0a0", tpl_7952::<0x14387def0, 0x14387de10, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409ba0a0
    Replacement::draft_throws(0x1409ba140, "unknown_1409ba140", fn_1409ba140 as *const (), Throws::Caught), // sub_1409ba140
    Replacement::draft_throws(0x1409ba220, "unknown_1409ba220", fn_1409ba220 as *const (), Throws::Caught), // sub_1409ba220
    Replacement::draft_throws(0x1409ba300, "unknown_1409ba300", fn_1409ba300 as *const (), Throws::Caught), // sub_1409ba300
    Replacement::draft(0x1409ba3e0, "unknown_1409ba3e0", fn_1409ba3e0 as *const ()), // sub_1409ba3e0
    Replacement::draft(0x1409ba450, "unknown_1409ba450", tpl_7856::<0x14387def0, 0x14387de10, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_1409ba450
    Replacement::draft(0x1409ba4c0, "unknown_1409ba4c0", tpl_9969::<0x14387de10, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409ba4c0
    Replacement::draft_throws(0x1409ba540, "unknown_1409ba540", tpl_7858::<0x14387def0, 0x14387de10, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_1409ba540
    Replacement::draft_throws(0x1409ba5c0, "unknown_1409ba5c0", tpl_12133::<0x14387de10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1434268d0, 0x8, 0x1432a44f0, 0x143250720, 0x1432a4520, 0x14020f240> as *const (), Throws::Discard), // sub_1409ba5c0
    Replacement::draft(0x1409ba6c0, "unknown_1409ba6c0", fn_1409ba6c0 as *const ()), // sub_1409ba6c0
    Replacement::draft(0x1409ba7a0, "unknown_1409ba7a0", fn_1409ba7a0 as *const ()), // sub_1409ba7a0
    Replacement::draft(0x1409ba880, "unknown_1409ba880", fn_1409ba880 as *const ()), // sub_1409ba880
    Replacement::draft(0x1409ba8f0, "unknown_1409ba8f0", fn_1409ba8f0 as *const ()), // sub_1409ba8f0
    Replacement::draft_throws(0x1409baf60, "unknown_1409baf60", fn_1409baf60 as *const (), Throws::Discard), // sub_1409baf60
    Replacement::draft(0x1409bb680, "unknown_1409bb680", fn_1409bb680 as *const ()), // sub_1409bb680
    Replacement::draft_throws(0x1409bba60, "unknown_1409bba60", fn_1409bba60 as *const (), Throws::Discard), // sub_1409bba60
    Replacement::draft(0x1409bc330, "unknown_1409bc330", fn_1409bc330 as *const ()), // sub_1409bc330
    Replacement::draft_throws(0x1409bc5d0, "unknown_1409bc5d0", fn_1409bc5d0 as *const (), Throws::Caught), // sub_1409bc5d0
    Replacement::draft_throws(0x1409bd6e0, "unknown_1409bd6e0", fn_1409bd6e0 as *const (), Throws::Caught), // sub_1409bd6e0
    Replacement::draft_throws(0x1409be790, "unknown_1409be790", fn_1409be790 as *const (), Throws::Caught), // sub_1409be790
    Replacement::draft(0x1409bf880, "unknown_1409bf880", fn_1409bf880 as *const ()), // sub_1409bf880
    Replacement::draft(0x1409bfa00, "unknown_1409bfa00", fn_1409bfa00 as *const ()), // sub_1409bfa00
    Replacement::draft(0x1409bfe00, "unknown_1409bfe00", fn_1409bfe00 as *const ()), // sub_1409bfe00
    Replacement::draft_throws(0x1409c0210, "unknown_1409c0210", fn_1409c0210 as *const (), Throws::Discard), // sub_1409c0210
    Replacement::draft_throws(0x1409c06b0, "unknown_1409c06b0", fn_1409c06b0 as *const (), Throws::Caught), // sub_1409c06b0
    Replacement::draft(0x1409c13a0, "unknown_1409c13a0", fn_1409c13a0 as *const ()), // sub_1409c13a0
    Replacement::draft_throws(0x1409c1690, "unknown_1409c1690", fn_1409c1690 as *const (), Throws::Discard), // sub_1409c1690
    Replacement::draft(0x1409c1f00, "unknown_1409c1f00", fn_1409c1f00 as *const ()), // sub_1409c1f00
    Replacement::draft(0x1409c2140, "unknown_1409c2140", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432873f0, 0x8> as *const ()), // sub_1409c2140
    Replacement::draft(0x1409c21c0, "unknown_1409c21c0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432873f0> as *const ()), // sub_1409c21c0
    Replacement::draft(0x1409c2250, "unknown_1409c2250", tpl_9031::<0x14387e370, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c2250
    Replacement::draft(0x1409c22c0, "unknown_1409c22c0", tpl_12078::<0x14387e370, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c22c0
    Replacement::draft_throws(0x1409c2360, "unknown_1409c2360", fn_1409c2360 as *const (), Throws::Discard), // sub_1409c2360
    Replacement::draft(0x1409c23f0, "unknown_1409c23f0", tpl_12080::<0x14387e370, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c23f0
    Replacement::draft_throws(0x1409c2470, "unknown_1409c2470", fn_1409c2470 as *const (), Throws::Discard), // sub_1409c2470
    Replacement::draft_throws(0x1409c2500, "unknown_1409c2500", fn_1409c2500 as *const (), Throws::Caught), // sub_1409c2500
    Replacement::draft_throws(0x1409c25d0, "unknown_1409c25d0", fn_1409c25d0 as *const (), Throws::Caught), // sub_1409c25d0
    Replacement::draft_throws(0x1409c26a0, "unknown_1409c26a0", fn_1409c26a0 as *const (), Throws::Caught), // sub_1409c26a0
    Replacement::draft(0x1409c2770, "unknown_1409c2770", fn_1409c2770 as *const ()), // sub_1409c2770
    Replacement::draft(0x1409c27f0, "unknown_1409c27f0", tpl_12084::<0x14387df78, 0x14387e370, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_1409c27f0
    Replacement::draft(0x1409c2860, "unknown_1409c2860", tpl_12080::<0x14387e370, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c2860
    Replacement::draft_throws(0x1409c28e0, "unknown_1409c28e0", tpl_12085::<0x14387df78, 0x14387e370, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409c28e0
    Replacement::draft_throws(0x1409c2960, "unknown_1409c2960", fn_1409c2960 as *const (), Throws::Discard), // sub_1409c2960
    Replacement::draft(0x1409c2a50, "unknown_1409c2a50", tpl_8433::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328b328, 0x8, 0x20, 0x28, 0x30> as *const ()), // sub_1409c2a50
    Replacement::draft(0x1409c2af0, "unknown_1409c2af0", tpl_8434::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14328b328, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1409c2af0
    Replacement::draft(0x1409c2ba0, "unknown_1409c2ba0", tpl_12151::<0x14387e290, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1409ffa40, 0x18, 0x28, 0x1409ffad0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c2ba0
    Replacement::draft(0x1409c2c50, "unknown_1409c2c50", tpl_12151::<0x14387e290, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1409ffc20, 0x18, 0x28, 0x1409ffad0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c2c50
    Replacement::draft_throws(0x1409c2d00, "unknown_1409c2d00", fn_1409c2d00 as *const (), Throws::Discard), // sub_1409c2d00
    Replacement::draft(0x1409c2e80, "unknown_1409c2e80", fn_1409c2e80 as *const ()), // sub_1409c2e80
    Replacement::draft_throws(0x1409c3010, "unknown_1409c3010", fn_1409c3010 as *const (), Throws::Discard), // sub_1409c3010
    Replacement::draft(0x1409c31f0, "unknown_1409c31f0", fn_1409c31f0 as *const ()), // sub_1409c31f0
    Replacement::draft_throws(0x1409c3340, "unknown_1409c3340", fn_1409c3340 as *const (), Throws::Caught), // sub_1409c3340
    Replacement::draft_throws(0x1409c3420, "unknown_1409c3420", fn_1409c3420 as *const (), Throws::Caught), // sub_1409c3420
    Replacement::draft_throws(0x1409c3500, "unknown_1409c3500", fn_1409c3500 as *const (), Throws::Caught), // sub_1409c3500
    Replacement::draft(0x1409c35e0, "unknown_1409c35e0", fn_1409c35e0 as *const ()), // sub_1409c35e0
    Replacement::draft(0x1409c3770, "unknown_1409c3770", fn_1409c3770 as *const ()), // sub_1409c3770
    Replacement::draft(0x1409c38f0, "unknown_1409c38f0", fn_1409c38f0 as *const ()), // sub_1409c38f0
    Replacement::draft_throws(0x1409c3a80, "unknown_1409c3a80", fn_1409c3a80 as *const (), Throws::Discard), // sub_1409c3a80
    Replacement::draft_throws(0x1409c3bf0, "unknown_1409c3bf0", fn_1409c3bf0 as *const (), Throws::Discard), // sub_1409c3bf0
    Replacement::draft(0x1409c3cf0, "unknown_1409c3cf0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143283858, 0x8> as *const ()), // sub_1409c3cf0
    Replacement::draft(0x1409c3d70, "unknown_1409c3d70", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143283858> as *const ()), // sub_1409c3d70
    Replacement::draft(0x1409c3e70, "unknown_1409c3e70", tpl_12161::<0x14387e558, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c3e70
    Replacement::draft_throws(0x1409c3f20, "unknown_1409c3f20", tpl_7851::<0x14387e1d0, 0x14387e558, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409c3f20
    Replacement::draft(0x1409c3fb0, "unknown_1409c3fb0", tpl_9969::<0x14387e558, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c3fb0
    Replacement::draft_throws(0x1409c4030, "unknown_1409c4030", tpl_7972::<0x14387e1d0, 0x14387e558, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409c4030
    Replacement::draft_throws(0x1409c40c0, "unknown_1409c40c0", fn_1409c40c0 as *const (), Throws::Caught), // sub_1409c40c0
    Replacement::draft_throws(0x1409c4190, "unknown_1409c4190", fn_1409c4190 as *const (), Throws::Caught), // sub_1409c4190
    Replacement::draft_throws(0x1409c4260, "unknown_1409c4260", fn_1409c4260 as *const (), Throws::Caught), // sub_1409c4260
    Replacement::draft(0x1409c4330, "unknown_1409c4330", tpl_12163::<0x14387e558, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c4330
    Replacement::draft(0x1409c43a0, "unknown_1409c43a0", tpl_7856::<0x14387e1d0, 0x14387e558, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xe8, 0x1401e9860> as *const ()), // sub_1409c43a0
    Replacement::draft(0x1409c4410, "unknown_1409c4410", tpl_9969::<0x14387e558, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c4410
    Replacement::draft_throws(0x1409c4490, "unknown_1409c4490", tpl_7974::<0x14387e1d0, 0x14387e558, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409c4490
    Replacement::draft_throws(0x1409c4510, "unknown_1409c4510", fn_1409c4510 as *const (), Throws::Discard), // sub_1409c4510
    Replacement::draft(0x1409c4600, "unknown_1409c4600", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143287298, 0x8> as *const ()), // sub_1409c4600
    Replacement::draft(0x1409c4690, "unknown_1409c4690", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143287298, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409c4690
    Replacement::draft(0x1409c4740, "unknown_1409c4740", fn_1409c4740 as *const ()), // sub_1409c4740
    Replacement::draft(0x1409c47e0, "unknown_1409c47e0", fn_1409c47e0 as *const ()), // sub_1409c47e0
    Replacement::draft_throws(0x1409c4890, "unknown_1409c4890", fn_1409c4890 as *const (), Throws::Discard), // sub_1409c4890
    Replacement::draft(0x1409c4980, "unknown_1409c4980", fn_1409c4980 as *const ()), // sub_1409c4980
    Replacement::draft_throws(0x1409c4a60, "unknown_1409c4a60", fn_1409c4a60 as *const (), Throws::Discard), // sub_1409c4a60
    Replacement::draft_throws(0x1409c4b60, "unknown_1409c4b60", fn_1409c4b60 as *const (), Throws::Caught), // sub_1409c4b60
    Replacement::draft_throws(0x1409c4c10, "unknown_1409c4c10", fn_1409c4c10 as *const (), Throws::Caught), // sub_1409c4c10
    Replacement::draft_throws(0x1409c4cc0, "unknown_1409c4cc0", fn_1409c4cc0 as *const (), Throws::Caught), // sub_1409c4cc0
    Replacement::draft(0x1409c4d70, "unknown_1409c4d70", fn_1409c4d70 as *const ()), // sub_1409c4d70
    Replacement::draft(0x1409c4e40, "unknown_1409c4e40", fn_1409c4e40 as *const ()), // sub_1409c4e40
    Replacement::draft(0x1409c4f10, "unknown_1409c4f10", fn_1409c4f10 as *const ()), // sub_1409c4f10
    Replacement::draft_throws(0x1409c4ff0, "unknown_1409c4ff0", fn_1409c4ff0 as *const (), Throws::Discard), // sub_1409c4ff0
    Replacement::draft_throws(0x1409c50b0, "unknown_1409c50b0", tpl_11868::<0x14387e118, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432a5830, 0x1402ca880> as *const (), Throws::Discard), // sub_1409c50b0
    Replacement::draft(0x1409c5180, "unknown_1409c5180", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432979e0> as *const ()), // sub_1409c5180
    Replacement::draft(0x1409c51e0, "unknown_1409c51e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432979e0> as *const ()), // sub_1409c51e0
    Replacement::draft_throws(0x1409c52e0, "unknown_1409c52e0", fn_1409c52e0 as *const (), Throws::Caught), // sub_1409c52e0
    Replacement::draft_throws(0x1409c5370, "unknown_1409c5370", tpl_6763::<0x14387dd80, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409c5370
    Replacement::draft(0x1409c5420, "unknown_1409c5420", fn_1409c5420 as *const ()), // sub_1409c5420
    Replacement::draft(0x1409c54c0, "unknown_1409c54c0", fn_1409c54c0 as *const ()), // sub_1409c54c0
    Replacement::draft(0x1409c5570, "unknown_1409c5570", fn_1409c5570 as *const ()), // sub_1409c5570
    Replacement::draft(0x1409c5620, "unknown_1409c5620", fn_1409c5620 as *const ()), // sub_1409c5620
    Replacement::draft_throws(0x1409c5780, "unknown_1409c5780", fn_1409c5780 as *const (), Throws::Caught), // sub_1409c5780
    Replacement::draft(0x1409c59a0, "unknown_1409c59a0", fn_1409c59a0 as *const ()), // sub_1409c59a0
    Replacement::draft_throws(0x1409c5ac0, "unknown_1409c5ac0", fn_1409c5ac0 as *const (), Throws::Caught), // sub_1409c5ac0
    Replacement::draft(0x1409c5d00, "unknown_1409c5d00", fn_1409c5d00 as *const ()), // sub_1409c5d00
    Replacement::draft_throws(0x1409c5df0, "unknown_1409c5df0", fn_1409c5df0 as *const (), Throws::Caught), // sub_1409c5df0
    Replacement::draft_throws(0x1409c6000, "unknown_1409c6000", fn_1409c6000 as *const (), Throws::Caught), // sub_1409c6000
    Replacement::draft_throws(0x1409c6210, "unknown_1409c6210", fn_1409c6210 as *const (), Throws::Caught), // sub_1409c6210
    Replacement::draft(0x1409c6420, "unknown_1409c6420", fn_1409c6420 as *const ()), // sub_1409c6420
    Replacement::draft(0x1409c6500, "unknown_1409c6500", fn_1409c6500 as *const ()), // sub_1409c6500
    Replacement::draft(0x1409c6610, "unknown_1409c6610", fn_1409c6610 as *const ()), // sub_1409c6610
    Replacement::draft_throws(0x1409c6730, "unknown_1409c6730", fn_1409c6730 as *const (), Throws::Discard), // sub_1409c6730
    Replacement::draft_throws(0x1409c6850, "unknown_1409c6850", fn_1409c6850 as *const (), Throws::Caught), // sub_1409c6850
    Replacement::draft(0x1409c6ad0, "unknown_1409c6ad0", fn_1409c6ad0 as *const ()), // sub_1409c6ad0
    Replacement::draft(0x1409c6c40, "unknown_1409c6c40", fn_1409c6c40 as *const ()), // sub_1409c6c40
    Replacement::draft(0x1409c6d20, "unknown_1409c6d20", fn_1409c6d20 as *const ()), // sub_1409c6d20
    Replacement::draft(0x1409c6de0, "unknown_1409c6de0", fn_1409c6de0 as *const ()), // sub_1409c6de0
    Replacement::draft(0x1409c6eb0, "unknown_1409c6eb0", fn_1409c6eb0 as *const ()), // sub_1409c6eb0
    Replacement::draft(0x1409c6f50, "unknown_1409c6f50", fn_1409c6f50 as *const ()), // sub_1409c6f50
    Replacement::draft_throws(0x1409c7010, "unknown_1409c7010", fn_1409c7010 as *const (), Throws::Caught), // sub_1409c7010
    Replacement::draft_throws(0x1409c7180, "unknown_1409c7180", fn_1409c7180 as *const (), Throws::Discard), // sub_1409c7180
    Replacement::draft_throws(0x1409c7260, "unknown_1409c7260", fn_1409c7260 as *const (), Throws::Discard), // sub_1409c7260
    Replacement::draft_throws(0x1409c7320, "unknown_1409c7320", fn_1409c7320 as *const (), Throws::Caught), // sub_1409c7320
    Replacement::draft_throws(0x1409c73f0, "unknown_1409c73f0", fn_1409c73f0 as *const (), Throws::Caught), // sub_1409c73f0
    Replacement::draft_throws(0x1409c74c0, "unknown_1409c74c0", fn_1409c74c0 as *const (), Throws::Caught), // sub_1409c74c0
    Replacement::draft(0x1409c7590, "unknown_1409c7590", fn_1409c7590 as *const ()), // sub_1409c7590
    Replacement::draft(0x1409c76f0, "unknown_1409c76f0", fn_1409c76f0 as *const ()), // sub_1409c76f0
    Replacement::draft(0x1409c77d0, "unknown_1409c77d0", fn_1409c77d0 as *const ()), // sub_1409c77d0
    Replacement::draft_throws(0x1409c78c0, "unknown_1409c78c0", fn_1409c78c0 as *const (), Throws::Discard), // sub_1409c78c0
    Replacement::draft_throws(0x1409c79b0, "unknown_1409c79b0", fn_1409c79b0 as *const (), Throws::Discard), // sub_1409c79b0
    Replacement::draft(0x1409c7ab0, "unknown_1409c7ab0", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432908e0> as *const ()), // sub_1409c7ab0
    Replacement::draft(0x1409c7b40, "unknown_1409c7b40", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432908e0, 0x18> as *const ()), // sub_1409c7b40
    Replacement::draft(0x1409c7bf0, "unknown_1409c7bf0", tpl_6748::<0x14387e318, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c7bf0
    Replacement::draft(0x1409c7c60, "unknown_1409c7c60", fn_1409c7c60 as *const ()), // sub_1409c7c60
    Replacement::draft_throws(0x1409c7db0, "unknown_1409c7db0", fn_1409c7db0 as *const (), Throws::Discard), // sub_1409c7db0
    Replacement::draft(0x1409c7f40, "unknown_1409c7f40", fn_1409c7f40 as *const ()), // sub_1409c7f40
    Replacement::draft_throws(0x1409c8030, "unknown_1409c8030", fn_1409c8030 as *const (), Throws::Discard), // sub_1409c8030
    Replacement::draft_throws(0x1409c81f0, "unknown_1409c81f0", fn_1409c81f0 as *const (), Throws::Caught), // sub_1409c81f0
    Replacement::draft_throws(0x1409c8430, "unknown_1409c8430", fn_1409c8430 as *const (), Throws::Caught), // sub_1409c8430
    Replacement::draft_throws(0x1409c8670, "unknown_1409c8670", fn_1409c8670 as *const (), Throws::Caught), // sub_1409c8670
    Replacement::draft(0x1409c88b0, "unknown_1409c88b0", fn_1409c88b0 as *const ()), // sub_1409c88b0
    Replacement::draft(0x1409c8950, "unknown_1409c8950", fn_1409c8950 as *const ()), // sub_1409c8950
    Replacement::draft(0x1409c8a40, "unknown_1409c8a40", fn_1409c8a40 as *const ()), // sub_1409c8a40
    Replacement::draft_throws(0x1409c8b40, "unknown_1409c8b40", fn_1409c8b40 as *const (), Throws::Discard), // sub_1409c8b40
    Replacement::draft_throws(0x1409c8c60, "unknown_1409c8c60", fn_1409c8c60 as *const (), Throws::Discard), // sub_1409c8c60
    Replacement::draft(0x1409c8eb0, "unknown_1409c8eb0", tpl_11427::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14328cb60, 0x20, 0x28, 0x30> as *const ()), // sub_1409c8eb0
    Replacement::draft(0x1409c8f70, "unknown_1409c8f70", tpl_11428::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328cb60, 0xfffffffffffffff4, 0x14, 0x34, 0xfffffffffffffffc, 0x4> as *const ()), // sub_1409c8f70
    Replacement::draft(0x1409c9030, "unknown_1409c9030", tpl_7312::<0x14387e588, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c9030
    Replacement::draft(0x1409c90a0, "unknown_1409c90a0", fn_1409c90a0 as *const ()), // sub_1409c90a0
    Replacement::draft_throws(0x1409c9160, "unknown_1409c9160", fn_1409c9160 as *const (), Throws::Discard), // sub_1409c9160
    Replacement::draft(0x1409c9210, "unknown_1409c9210", tpl_11823::<0x14387e588, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c9210
    Replacement::draft_throws(0x1409c9290, "unknown_1409c9290", fn_1409c9290 as *const (), Throws::Discard), // sub_1409c9290
    Replacement::draft_throws(0x1409c9340, "unknown_1409c9340", fn_1409c9340 as *const (), Throws::Caught), // sub_1409c9340
    Replacement::draft_throws(0x1409c93f0, "unknown_1409c93f0", fn_1409c93f0 as *const (), Throws::Caught), // sub_1409c93f0
    Replacement::draft_throws(0x1409c94a0, "unknown_1409c94a0", fn_1409c94a0 as *const (), Throws::Caught), // sub_1409c94a0
    Replacement::draft(0x1409c96c0, "unknown_1409c96c0", fn_1409c96c0 as *const ()), // sub_1409c96c0
    Replacement::draft(0x1409c9750, "unknown_1409c9750", tpl_7947::<0x14387df68, 0x14387e588, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_1409c9750
    Replacement::draft(0x1409c97c0, "unknown_1409c97c0", tpl_11823::<0x14387e588, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409c97c0
    Replacement::draft_throws(0x1409c9840, "unknown_1409c9840", fn_1409c9840 as *const (), Throws::Discard), // sub_1409c9840
    Replacement::draft_throws(0x1409c98b0, "unknown_1409c98b0", fn_1409c98b0 as *const (), Throws::Discard), // sub_1409c98b0
    Replacement::draft(0x1409c99b0, "unknown_1409c99b0", fn_1409c99b0 as *const ()), // sub_1409c99b0
    Replacement::draft(0x1409c9a70, "unknown_1409c9a70", fn_1409c9a70 as *const ()), // sub_1409c9a70
    Replacement::draft(0x1409c9bc0, "unknown_1409c9bc0", fn_1409c9bc0 as *const ()), // sub_1409c9bc0
    Replacement::draft_throws(0x1409c9cc0, "unknown_1409c9cc0", fn_1409c9cc0 as *const (), Throws::Discard), // sub_1409c9cc0
    Replacement::draft(0x1409c9dd0, "unknown_1409c9dd0", fn_1409c9dd0 as *const ()), // sub_1409c9dd0
    Replacement::draft_throws(0x1409c9e60, "unknown_1409c9e60", tpl_67896::<0x14387dddc, 0x14387df88, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1426cdcb0, 0x328, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x20, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409c9e60
    Replacement::draft_throws(0x1409c9f60, "unknown_1409c9f60", fn_1409c9f60 as *const (), Throws::Caught), // sub_1409c9f60
    Replacement::draft_throws(0x1409ca350, "unknown_1409ca350", fn_1409ca350 as *const (), Throws::Caught), // sub_1409ca350
    Replacement::draft_throws(0x1409ca6d0, "unknown_1409ca6d0", fn_1409ca6d0 as *const (), Throws::Caught), // sub_1409ca6d0
    Replacement::draft(0x1409caa30, "unknown_1409caa30", fn_1409caa30 as *const ()), // sub_1409caa30
    Replacement::draft(0x1409caaf0, "unknown_1409caaf0", tpl_67902::<0x14387dddc, 0x14387df88, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x20, 0x1401e9860> as *const ()), // sub_1409caaf0
    Replacement::draft(0x1409cab70, "unknown_1409cab70", tpl_12226::<0x14387df88, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409cab70
    Replacement::draft_throws(0x1409cac00, "unknown_1409cac00", tpl_67903::<0x14387dddc, 0x14387df88, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x20, 0x1401e9590> as *const (), Throws::Discard), // sub_1409cac00
    Replacement::draft_throws(0x1409cac80, "unknown_1409cac80", fn_1409cac80 as *const (), Throws::Discard), // sub_1409cac80
    Replacement::draft(0x1409cada0, "unknown_1409cada0", tpl_11667::<0x14387e508, 0x14243b0b0, 0x1408f8c80, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409cada0
    Replacement::draft_throws(0x1409cae40, "unknown_1409cae40", fn_1409cae40 as *const (), Throws::Discard), // sub_1409cae40
    Replacement::draft(0x1409caed0, "unknown_1409caed0", tpl_11361::<0x14387e508, 0x14243b0b0, 0x1408f8ff0, 0x338, 0x8, 0x40, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409caed0
    Replacement::draft_throws(0x1409caf50, "unknown_1409caf50", tpl_12229::<0x14387e308, 0x14387e508, 0x1401e9b60, 0x1409fb200, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409caf50
    Replacement::draft_throws(0x1409caff0, "unknown_1409caff0", fn_1409caff0 as *const (), Throws::Caught), // sub_1409caff0
    Replacement::draft_throws(0x1409cb0f0, "unknown_1409cb0f0", fn_1409cb0f0 as *const (), Throws::Caught), // sub_1409cb0f0
    Replacement::draft_throws(0x1409cb1f0, "unknown_1409cb1f0", fn_1409cb1f0 as *const (), Throws::Caught), // sub_1409cb1f0
    Replacement::draft(0x1409cb2f0, "unknown_1409cb2f0", fn_1409cb2f0 as *const ()), // sub_1409cb2f0
    Replacement::draft(0x1409cb370, "unknown_1409cb370", tpl_9203::<0x14387e308, 0x14387e508, 0x1401e9970, 0x1408f9a30, 0x338, 0x8, 0xe0, 0x1401e9860> as *const ()), // sub_1409cb370
    Replacement::draft(0x1409cb3e0, "unknown_1409cb3e0", tpl_11361::<0x14387e508, 0x14243b0b0, 0x1408f9ba0, 0x338, 0x8, 0xe0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409cb3e0
    Replacement::draft_throws(0x1409cb460, "unknown_1409cb460", tpl_9073::<0x14387e308, 0x14387e508, 0x1401e9770, 0x1408f9cf0, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409cb460
    Replacement::draft_throws(0x1409cb4d0, "unknown_1409cb4d0", fn_1409cb4d0 as *const (), Throws::Discard), // sub_1409cb4d0
    Replacement::draft(0x1409cb590, "unknown_1409cb590", fn_1409cb590 as *const ()), // sub_1409cb590
    Replacement::draft(0x1409cb630, "unknown_1409cb630", fn_1409cb630 as *const ()), // sub_1409cb630
    Replacement::draft(0x1409cb6c0, "unknown_1409cb6c0", fn_1409cb6c0 as *const ()), // sub_1409cb6c0
    Replacement::draft(0x1409cb7e0, "unknown_1409cb7e0", fn_1409cb7e0 as *const ()), // sub_1409cb7e0
    Replacement::draft_throws(0x1409cb8d0, "unknown_1409cb8d0", fn_1409cb8d0 as *const (), Throws::Discard), // sub_1409cb8d0
    Replacement::draft(0x1409cb9c0, "unknown_1409cb9c0", fn_1409cb9c0 as *const ()), // sub_1409cb9c0
    Replacement::draft_throws(0x1409cba60, "unknown_1409cba60", fn_1409cba60 as *const (), Throws::Discard), // sub_1409cba60
    Replacement::draft_throws(0x1409cbb60, "unknown_1409cbb60", fn_1409cbb60 as *const (), Throws::Caught), // sub_1409cbb60
    Replacement::draft_throws(0x1409cbe10, "unknown_1409cbe10", fn_1409cbe10 as *const (), Throws::Caught), // sub_1409cbe10
    Replacement::draft_throws(0x1409cc080, "unknown_1409cc080", fn_1409cc080 as *const (), Throws::Caught), // sub_1409cc080
    Replacement::draft(0x1409cc2f0, "unknown_1409cc2f0", fn_1409cc2f0 as *const ()), // sub_1409cc2f0
    Replacement::draft(0x1409cc370, "unknown_1409cc370", fn_1409cc370 as *const ()), // sub_1409cc370
    Replacement::draft(0x1409cc410, "unknown_1409cc410", fn_1409cc410 as *const ()), // sub_1409cc410
    Replacement::draft_throws(0x1409cc4c0, "unknown_1409cc4c0", fn_1409cc4c0 as *const (), Throws::Discard), // sub_1409cc4c0
    Replacement::draft_throws(0x1409cc570, "unknown_1409cc570", fn_1409cc570 as *const (), Throws::Discard), // sub_1409cc570
];
