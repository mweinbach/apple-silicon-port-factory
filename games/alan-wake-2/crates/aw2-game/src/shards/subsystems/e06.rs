#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft_throws(0x140bf9a30, "content_140bf9a30", fn_140bf9a30 as *const (), Throws::Discard), // sub_140bf9a30
    Replacement::draft_throws(0x140bf9ae0, "content_140bf9ae0", fn_140bf9ae0 as *const (), Throws::Caught), // sub_140bf9ae0
    Replacement::draft(0x140bf9ca0, "content_140bf9ca0", tpl_72438::<0x143426c20, 0x1432d64c0, 0x8, 0x1432d64e8, 0x1432d6500, 0x14020f270, 0x143426990, 0x1432d6510, 0xc, 0x1432d6480, 0x1432d6498> as *const ()), // sub_140bf9ca0
    Replacement::draft(0x140bf9d40, "content_140bf9d40", fn_140bf9d40 as *const ()), // sub_140bf9d40
    Replacement::draft(0x140bf9df0, "content_140bf9df0", fn_140bf9df0 as *const ()), // sub_140bf9df0
    Replacement::draft(0x140bf9ea0, "content_140bf9ea0", fn_140bf9ea0 as *const ()), // sub_140bf9ea0
    Replacement::draft(0x140bf9f10, "content_140bf9f10", tpl_10123::<0x1401e84a0, 0x18, 0x30, 0x48, 0x60, 0x78> as *const ()), // sub_140bf9f10
    Replacement::draft(0x140bf9f80, "content_140bf9f80", fn_140bf9f80 as *const ()), // sub_140bf9f80
    Replacement::draft(0x140bfa090, "content_140bfa090", fn_140bfa090 as *const ()), // sub_140bfa090
    Replacement::draft(0x140bfa150, "content_140bfa150", fn_140bfa150 as *const ()), // content::NLWeaponComponent::content::NLWeaponComponent::vfunc_0
    Replacement::draft(0x140bfa190, "content_140bfa190", fn_140bfa190 as *const ()), // content::AbilityPropertiesComponent::content::AbilityPropertiesComponent::vfunc_0
    Replacement::draft(0x140bfa1e0, "content_140bfa1e0", fn_140bfa1e0 as *const ()), // content::AvatarManagerSingletonComponent::content::AvatarManagerSingletonComponent::vfunc_0
    Replacement::draft(0x140bfa230, "content_140bfa230", fn_140bfa230 as *const ()), // content::FlashlightTriggersComponent::content::FlashlightTriggersComponent::vfunc_0
    Replacement::draft(0x140bfa2d0, "content_140bfa2d0", fn_140bfa2d0 as *const ()), // content::GameWeaponAudioComponent::content::GameWeaponAudioComponent::vfunc_0
    Replacement::draft(0x140bfa380, "content_140bfa380", tpl_15002::<0x14028d970, 0x8, 0x14240ab10, 0x143084e28, 0x141834d30> as *const ()), // content::MapVisualizationComponent::content::MapVisualizationComponent::vfunc_0
    Replacement::draft(0x140bfa3d0, "content_140bfa3d0", fn_140bfa3d0 as *const ()), // content::PlayerPeekingComponent::content::PlayerPeekingComponent::vfunc_0
    Replacement::draft(0x140bfa490, "content_140bfa490", fn_140bfa490 as *const ()), // content::UtilityObjectSpawnerSingletonComponent::content::UtilityObjectSpawnerSingletonComponent::vfunc_0
    Replacement::draft(0x140bfa4e0, "content_140bfa4e0", tpl_1446::<0x140bfa530, 0x8, 0x1401f2bc0, 0x143084e28, 0x141834d30> as *const ()), // content::NLAIVisionSensorComponent::content::NLAIVisionSensorComponent::vfunc_0
    Replacement::draft(0x140bfa530, "unknown_140bfa530", tpl_2030::<0x140244c70, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140bfa530
    Replacement::draft(0x140bfa5e0, "unknown_140bfa5e0", tpl_73446::<0x140bfac10, 0x140c0ad30, 0x1432ce898, 0x1401e84a0> as *const ()), // sub_140bfa5e0
    Replacement::draft(0x140bfa6e0, "unknown_140bfa6e0", tpl_2030::<0x140bfac10, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140bfa6e0
    Replacement::draft(0x140bfa790, "unknown_140bfa790", tpl_73446::<0x140bfac10, 0x140c0af60, 0x1432d2118, 0x1401e84a0> as *const ()), // sub_140bfa790
    Replacement::draft(0x140bfa890, "unknown_140bfa890", fn_140bfa890 as *const ()), // sub_140bfa890
    Replacement::draft(0x140bfa990, "unknown_140bfa990", fn_140bfa990 as *const ()), // sub_140bfa990
    Replacement::draft(0x140bfaa60, "unknown_140bfaa60", fn_140bfaa60 as *const ()), // sub_140bfaa60
    Replacement::draft(0x140bfaad0, "unknown_140bfaad0", fn_140bfaad0 as *const ()), // sub_140bfaad0
    Replacement::draft(0x140bfab90, "unknown_140bfab90", fn_140bfab90 as *const ()), // sub_140bfab90
    Replacement::draft(0x140bfac10, "unknown_140bfac10", fn_140bfac10 as *const ()), // sub_140bfac10
    Replacement::draft(0x140bfac80, "unknown_140bfac80", fn_140bfac80 as *const ()), // sub_140bfac80
    Replacement::draft_throws(0x140bfad50, "unknown_140bfad50", fn_140bfad50 as *const (), Throws::Caught), // sub_140bfad50
    Replacement::draft_throws(0x140bfaf00, "unknown_140bfaf00", fn_140bfaf00 as *const (), Throws::Caught), // sub_140bfaf00
    Replacement::draft_throws(0x140bfb0b0, "unknown_140bfb0b0", fn_140bfb0b0 as *const (), Throws::Caught), // sub_140bfb0b0
    Replacement::draft_throws(0x140bfb270, "unknown_140bfb270", fn_140bfb270 as *const (), Throws::Caught), // sub_140bfb270
    Replacement::draft_throws(0x140bfb460, "unknown_140bfb460", tpl_838::<0x140c0bf00> as *const (), Throws::Discard), // sub_140bfb460
    Replacement::draft(0x140bfb510, "unknown_140bfb510", fn_140bfb510 as *const ()), // sub_140bfb510
    Replacement::draft(0x140bfb5a0, "unknown_140bfb5a0", fn_140bfb5a0 as *const ()), // sub_140bfb5a0
    Replacement::draft_throws(0x140bfb610, "unknown_140bfb610", tpl_838::<0x140c0cfe0> as *const (), Throws::Discard), // sub_140bfb610
    Replacement::draft_throws(0x140bfb680, "unknown_140bfb680", fn_140bfb680 as *const (), Throws::Caught), // sub_140bfb680
    Replacement::draft_throws(0x140bfb830, "unknown_140bfb830", fn_140bfb830 as *const (), Throws::Caught), // sub_140bfb830
    Replacement::draft(0x140bfba60, "unknown_140bfba60", fn_140bfba60 as *const ()), // sub_140bfba60
    Replacement::draft_throws(0x140bfbb40, "unknown_140bfbb40", fn_140bfbb40 as *const (), Throws::Caught), // sub_140bfbb40
    Replacement::draft_throws(0x140bfbcf0, "unknown_140bfbcf0", fn_140bfbcf0 as *const (), Throws::Caught), // sub_140bfbcf0
    Replacement::draft_throws(0x140bfbf90, "unknown_140bfbf90", fn_140bfbf90 as *const (), Throws::Caught), // sub_140bfbf90
    Replacement::draft_throws(0x140bfc140, "unknown_140bfc140", fn_140bfc140 as *const (), Throws::Caught), // sub_140bfc140
    Replacement::draft_throws(0x140bfc310, "unknown_140bfc310", tpl_838::<0x140c110b0> as *const (), Throws::Discard), // sub_140bfc310
    Replacement::draft_throws(0x140bfc450, "unknown_140bfc450", fn_140bfc450 as *const (), Throws::Caught), // sub_140bfc450
    Replacement::draft_throws(0x140bfc600, "unknown_140bfc600", fn_140bfc600 as *const (), Throws::Caught), // sub_140bfc600
    Replacement::draft_throws(0x140bfc830, "unknown_140bfc830", fn_140bfc830 as *const (), Throws::Caught), // sub_140bfc830
    Replacement::draft_throws(0x140bfc960, "unknown_140bfc960", fn_140bfc960 as *const (), Throws::Caught), // sub_140bfc960
    Replacement::draft_throws(0x140bfca90, "unknown_140bfca90", fn_140bfca90 as *const (), Throws::Caught), // sub_140bfca90
    Replacement::draft_throws(0x140bfcbd0, "unknown_140bfcbd0", fn_140bfcbd0 as *const (), Throws::Caught), // sub_140bfcbd0
    Replacement::draft_throws(0x140bfcd10, "unknown_140bfcd10", fn_140bfcd10 as *const (), Throws::Caught), // sub_140bfcd10
    Replacement::draft_throws(0x140bfce50, "unknown_140bfce50", fn_140bfce50 as *const (), Throws::Caught), // sub_140bfce50
    Replacement::draft_throws(0x140bfd000, "unknown_140bfd000", fn_140bfd000 as *const (), Throws::Caught), // sub_140bfd000
    Replacement::draft_throws(0x140bfd1b0, "unknown_140bfd1b0", fn_140bfd1b0 as *const (), Throws::Caught), // sub_140bfd1b0
    Replacement::draft_throws(0x140bfd370, "unknown_140bfd370", fn_140bfd370 as *const (), Throws::Caught), // sub_140bfd370
    Replacement::draft_throws(0x140bfd680, "unknown_140bfd680", fn_140bfd680 as *const (), Throws::Caught), // sub_140bfd680
    Replacement::draft_throws(0x140bfd910, "unknown_140bfd910", fn_140bfd910 as *const (), Throws::Caught), // sub_140bfd910
    Replacement::draft_throws(0x140bfdba0, "unknown_140bfdba0", fn_140bfdba0 as *const (), Throws::Caught), // sub_140bfdba0
    Replacement::draft_throws(0x140bfde30, "unknown_140bfde30", fn_140bfde30 as *const (), Throws::Caught), // sub_140bfde30
    Replacement::draft_throws(0x140bfe0c0, "unknown_140bfe0c0", fn_140bfe0c0 as *const (), Throws::Caught), // sub_140bfe0c0
    Replacement::draft(0x140bfe350, "unknown_140bfe350", fn_140bfe350 as *const ()), // sub_140bfe350
    Replacement::draft_throws(0x140bfe440, "unknown_140bfe440", fn_140bfe440 as *const (), Throws::Caught), // sub_140bfe440
    Replacement::draft_throws(0x140bfe790, "unknown_140bfe790", fn_140bfe790 as *const (), Throws::Caught), // sub_140bfe790
    Replacement::draft_throws(0x140bfeae0, "unknown_140bfeae0", fn_140bfeae0 as *const (), Throws::Caught), // sub_140bfeae0
    Replacement::draft_throws(0x140bfee30, "unknown_140bfee30", fn_140bfee30 as *const (), Throws::Caught), // sub_140bfee30
    Replacement::draft_throws(0x140bff180, "unknown_140bff180", fn_140bff180 as *const (), Throws::Caught), // sub_140bff180
    Replacement::draft(0x140bff4d0, "unknown_140bff4d0", fn_140bff4d0 as *const ()), // sub_140bff4d0
    Replacement::draft(0x140bff510, "unknown_140bff510", fn_140bff510 as *const ()), // sub_140bff510
    Replacement::draft_throws(0x140bff630, "unknown_140bff630", fn_140bff630 as *const (), Throws::Caught), // sub_140bff630
    Replacement::draft_throws(0x140bff7e0, "unknown_140bff7e0", fn_140bff7e0 as *const (), Throws::Caught), // sub_140bff7e0
    Replacement::draft(0x140bffa10, "unknown_140bffa10", fn_140bffa10 as *const ()), // sub_140bffa10
    Replacement::draft_throws(0x140bffac0, "unknown_140bffac0", fn_140bffac0 as *const (), Throws::Caught), // sub_140bffac0
    Replacement::draft_throws(0x140bffd10, "unknown_140bffd10", fn_140bffd10 as *const (), Throws::Caught), // sub_140bffd10
    Replacement::draft_throws(0x140bfff60, "unknown_140bfff60", fn_140bfff60 as *const (), Throws::Caught), // sub_140bfff60
    Replacement::draft_throws(0x140c001b0, "unknown_140c001b0", fn_140c001b0 as *const (), Throws::Caught), // sub_140c001b0
    Replacement::draft_throws(0x140c00400, "unknown_140c00400", fn_140c00400 as *const (), Throws::Caught), // sub_140c00400
    Replacement::draft_throws(0x140c00650, "unknown_140c00650", fn_140c00650 as *const (), Throws::Caught), // sub_140c00650
    Replacement::draft(0x140c00780, "unknown_140c00780", fn_140c00780 as *const ()), // sub_140c00780
    Replacement::draft(0x140c008c0, "unknown_140c008c0", fn_140c008c0 as *const ()), // sub_140c008c0
    Replacement::draft_throws(0x140c00a50, "unknown_140c00a50", fn_140c00a50 as *const (), Throws::Caught), // sub_140c00a50
    Replacement::draft_throws(0x140c00b90, "unknown_140c00b90", fn_140c00b90 as *const (), Throws::Caught), // sub_140c00b90
    Replacement::draft_throws(0x140c00cd0, "unknown_140c00cd0", fn_140c00cd0 as *const (), Throws::Caught), // sub_140c00cd0
    Replacement::draft_throws(0x140c00e10, "unknown_140c00e10", fn_140c00e10 as *const (), Throws::Caught), // sub_140c00e10
    Replacement::draft_throws(0x140c00fc0, "unknown_140c00fc0", fn_140c00fc0 as *const (), Throws::Caught), // sub_140c00fc0
    Replacement::draft_throws(0x140c01260, "unknown_140c01260", fn_140c01260 as *const (), Throws::Caught), // sub_140c01260
    Replacement::draft_throws(0x140c01500, "unknown_140c01500", fn_140c01500 as *const (), Throws::Caught), // sub_140c01500
    Replacement::draft_throws(0x140c017a0, "unknown_140c017a0", fn_140c017a0 as *const (), Throws::Caught), // sub_140c017a0
    Replacement::draft_throws(0x140c01a50, "unknown_140c01a50", fn_140c01a50 as *const (), Throws::Caught), // sub_140c01a50
    Replacement::draft_throws(0x140c01d00, "unknown_140c01d00", fn_140c01d00 as *const (), Throws::Caught), // sub_140c01d00
    Replacement::draft_throws(0x140c01fb0, "unknown_140c01fb0", fn_140c01fb0 as *const (), Throws::Caught), // sub_140c01fb0
    Replacement::draft_throws(0x140c02570, "unknown_140c02570", fn_140c02570 as *const (), Throws::Caught), // sub_140c02570
    Replacement::draft_throws(0x140c029a0, "unknown_140c029a0", fn_140c029a0 as *const (), Throws::Caught), // sub_140c029a0
    Replacement::draft_throws(0x140c02f60, "unknown_140c02f60", fn_140c02f60 as *const (), Throws::Caught), // sub_140c02f60
    Replacement::draft_throws(0x140c03390, "unknown_140c03390", fn_140c03390 as *const (), Throws::Caught), // sub_140c03390
    Replacement::draft_throws(0x140c03950, "unknown_140c03950", fn_140c03950 as *const (), Throws::Caught), // sub_140c03950
    Replacement::draft_throws(0x140c03a90, "unknown_140c03a90", fn_140c03a90 as *const (), Throws::Caught), // sub_140c03a90
    Replacement::draft_throws(0x140c03ec0, "unknown_140c03ec0", fn_140c03ec0 as *const (), Throws::Caught), // sub_140c03ec0
    Replacement::draft_throws(0x140c04480, "unknown_140c04480", fn_140c04480 as *const (), Throws::Caught), // sub_140c04480
    Replacement::draft_throws(0x140c045c0, "unknown_140c045c0", fn_140c045c0 as *const (), Throws::Caught), // sub_140c045c0
    Replacement::draft_throws(0x140c049f0, "unknown_140c049f0", fn_140c049f0 as *const (), Throws::Caught), // sub_140c049f0
    Replacement::draft_throws(0x140c04fb0, "unknown_140c04fb0", fn_140c04fb0 as *const (), Throws::Caught), // sub_140c04fb0
    Replacement::draft_throws(0x140c050f0, "unknown_140c050f0", fn_140c050f0 as *const (), Throws::Caught), // sub_140c050f0
    Replacement::draft(0x140c05530, "unknown_140c05530", fn_140c05530 as *const ()), // sub_140c05530
    Replacement::draft(0x140c05560, "unknown_140c05560", fn_140c05560 as *const ()), // sub_140c05560
    Replacement::draft(0x140c05630, "unknown_140c05630", fn_140c05630 as *const ()), // sub_140c05630
    Replacement::draft(0x140c056c0, "unknown_140c056c0", fn_140c056c0 as *const ()), // sub_140c056c0
    Replacement::draft(0x140c05850, "unknown_140c05850", fn_140c05850 as *const ()), // sub_140c05850
    Replacement::draft_throws(0x140c05920, "unknown_140c05920", fn_140c05920 as *const (), Throws::Caught), // sub_140c05920
    Replacement::draft_throws(0x140c05a50, "unknown_140c05a50", fn_140c05a50 as *const (), Throws::Caught), // sub_140c05a50
    Replacement::draft_throws(0x140c05b80, "unknown_140c05b80", fn_140c05b80 as *const (), Throws::Caught), // sub_140c05b80
    Replacement::draft_throws(0x140c05cc0, "unknown_140c05cc0", fn_140c05cc0 as *const (), Throws::Caught), // sub_140c05cc0
    Replacement::draft_throws(0x140c05e00, "unknown_140c05e00", fn_140c05e00 as *const (), Throws::Caught), // sub_140c05e00
    Replacement::draft(0x140c05f40, "unknown_140c05f40", tpl_15057 as *const ()), // sub_140c05f40
    Replacement::draft(0x140c05f70, "unknown_140c05f70", tpl_15058 as *const ()), // sub_140c05f70
    Replacement::draft(0x140c05fa0, "unknown_140c05fa0", fn_140c05fa0 as *const ()), // sub_140c05fa0
    Replacement::draft(0x140c06030, "unknown_140c06030", fn_140c06030 as *const ()), // sub_140c06030
    Replacement::draft(0x140c06090, "unknown_140c06090", fn_140c06090 as *const ()), // sub_140c06090
    Replacement::draft(0x140c06130, "unknown_140c06130", fn_140c06130 as *const ()), // sub_140c06130
    Replacement::draft_throws(0x140c06210, "unknown_140c06210", fn_140c06210 as *const (), Throws::Caught), // sub_140c06210
    Replacement::draft_throws(0x140c063c0, "unknown_140c063c0", fn_140c063c0 as *const (), Throws::Caught), // sub_140c063c0
    Replacement::draft_throws(0x140c065f0, "unknown_140c065f0", fn_140c065f0 as *const (), Throws::Caught), // sub_140c065f0
    Replacement::draft_throws(0x140c067a0, "unknown_140c067a0", fn_140c067a0 as *const (), Throws::Caught), // sub_140c067a0
    Replacement::draft(0x140c069d0, "unknown_140c069d0", fn_140c069d0 as *const ()), // sub_140c069d0
    Replacement::draft(0x140c06a90, "unknown_140c06a90", fn_140c06a90 as *const ()), // sub_140c06a90
    Replacement::draft(0x140c06b50, "unknown_140c06b50", fn_140c06b50 as *const ()), // sub_140c06b50
    Replacement::draft(0x140c06c10, "unknown_140c06c10", fn_140c06c10 as *const ()), // sub_140c06c10
    Replacement::draft(0x140c06cf0, "unknown_140c06cf0", fn_140c06cf0 as *const ()), // sub_140c06cf0
    Replacement::draft_throws(0x140c06df0, "unknown_140c06df0", fn_140c06df0 as *const (), Throws::Caught), // sub_140c06df0
    Replacement::draft_throws(0x140c06fb0, "unknown_140c06fb0", fn_140c06fb0 as *const (), Throws::Caught), // sub_140c06fb0
    Replacement::draft_throws(0x140c07160, "unknown_140c07160", tpl_838::<0x140c27d10> as *const (), Throws::Discard), // sub_140c07160
    Replacement::draft_throws(0x140c07170, "unknown_140c07170", tpl_838::<0x140c28010> as *const (), Throws::Discard), // sub_140c07170
    Replacement::draft_throws(0x140c07190, "unknown_140c07190", tpl_6985::<0x20, 0x8, 0x140c28570> as *const (), Throws::Discard), // sub_140c07190
    Replacement::draft_throws(0x140c071e0, "unknown_140c071e0", fn_140c071e0 as *const (), Throws::Caught), // sub_140c071e0
    Replacement::draft_throws(0x140c07390, "unknown_140c07390", fn_140c07390 as *const (), Throws::Caught), // sub_140c07390
    Replacement::draft(0x140c075c0, "unknown_140c075c0", fn_140c075c0 as *const ()), // sub_140c075c0
    Replacement::draft(0x140c077b0, "unknown_140c077b0", fn_140c077b0 as *const ()), // sub_140c077b0
    Replacement::draft(0x140c07850, "unknown_140c07850", fn_140c07850 as *const ()), // sub_140c07850
    Replacement::draft(0x140c07a20, "unknown_140c07a20", fn_140c07a20 as *const ()), // sub_140c07a20
    Replacement::draft(0x140c07b00, "unknown_140c07b00", fn_140c07b00 as *const ()), // sub_140c07b00
    Replacement::draft_throws(0x140c07be0, "unknown_140c07be0", fn_140c07be0 as *const (), Throws::Caught), // sub_140c07be0
    Replacement::draft_throws(0x140c07d90, "unknown_140c07d90", fn_140c07d90 as *const (), Throws::Caught), // sub_140c07d90
    Replacement::draft_throws(0x140c07fc0, "unknown_140c07fc0", fn_140c07fc0 as *const (), Throws::Caught), // sub_140c07fc0
    Replacement::draft_throws(0x140c08170, "unknown_140c08170", fn_140c08170 as *const (), Throws::Caught), // sub_140c08170
    Replacement::draft_throws(0x140c08480, "unknown_140c08480", fn_140c08480 as *const (), Throws::Caught), // sub_140c08480
    Replacement::draft_throws(0x140c08630, "unknown_140c08630", fn_140c08630 as *const (), Throws::Caught), // sub_140c08630
    Replacement::draft_throws(0x140c087e0, "unknown_140c087e0", fn_140c087e0 as *const (), Throws::Caught), // sub_140c087e0
    Replacement::draft_throws(0x140c08990, "unknown_140c08990", fn_140c08990 as *const (), Throws::Caught), // sub_140c08990
    Replacement::draft_throws(0x140c08b50, "unknown_140c08b50", fn_140c08b50 as *const (), Throws::Caught), // sub_140c08b50
    Replacement::draft_throws(0x140c08d10, "unknown_140c08d10", fn_140c08d10 as *const (), Throws::Caught), // sub_140c08d10
    Replacement::draft(0x140c09090, "unknown_140c09090", fn_140c09090 as *const ()), // sub_140c09090
    Replacement::draft(0x140c09230, "unknown_140c09230", fn_140c09230 as *const ()), // sub_140c09230
    Replacement::draft(0x140c092f0, "unknown_140c092f0", fn_140c092f0 as *const ()), // sub_140c092f0
    Replacement::draft(0x140c093d0, "unknown_140c093d0", fn_140c093d0 as *const ()), // sub_140c093d0
    Replacement::draft(0x140c09500, "unknown_140c09500", tpl_73525::<0x1402211e0, 0x14183acc2, 0x14240ab10> as *const ()), // sub_140c09500
    Replacement::draft(0x140c09610, "unknown_140c09610", tpl_15094::<0x140c0ad30, 0x1432ce898> as *const ()), // sub_140c09610
    Replacement::draft(0x140c09700, "unknown_140c09700", tpl_15094::<0x140c0af60, 0x1432d2118> as *const ()), // sub_140c09700
    Replacement::draft(0x140c097f0, "unknown_140c097f0", fn_140c097f0 as *const ()), // sub_140c097f0
    Replacement::draft(0x140c09860, "unknown_140c09860", fn_140c09860 as *const ()), // sub_140c09860
    Replacement::draft(0x140c098e0, "unknown_140c098e0", fn_140c098e0 as *const ()), // sub_140c098e0
    Replacement::draft(0x140c099a0, "unknown_140c099a0", fn_140c099a0 as *const ()), // sub_140c099a0
    Replacement::draft(0x140c09a80, "unknown_140c09a80", fn_140c09a80 as *const ()), // sub_140c09a80
    Replacement::draft(0x140c09b70, "unknown_140c09b70", fn_140c09b70 as *const ()), // sub_140c09b70
    Replacement::draft(0x140c09bf0, "unknown_140c09bf0", fn_140c09bf0 as *const ()), // sub_140c09bf0
    Replacement::draft(0x140c09c50, "unknown_140c09c50", fn_140c09c50 as *const ()), // sub_140c09c50
    Replacement::draft(0x140c09d80, "unknown_140c09d80", fn_140c09d80 as *const ()), // sub_140c09d80
    Replacement::draft(0x140c09eb0, "unknown_140c09eb0", fn_140c09eb0 as *const ()), // sub_140c09eb0
    Replacement::draft(0x140c09f90, "unknown_140c09f90", tpl_73525::<0x1404de1c0, 0x14183acc2, 0x14240ab10> as *const ()), // sub_140c09f90
    Replacement::draft(0x140c0a0a0, "unknown_140c0a0a0", fn_140c0a0a0 as *const ()), // sub_140c0a0a0
    Replacement::draft(0x140c0a160, "unknown_140c0a160", fn_140c0a160 as *const ()), // sub_140c0a160
    Replacement::draft(0x140c0a270, "unknown_140c0a270", fn_140c0a270 as *const ()), // sub_140c0a270
    Replacement::draft(0x140c0a310, "unknown_140c0a310", fn_140c0a310 as *const ()), // sub_140c0a310
    Replacement::draft(0x140c0a400, "unknown_140c0a400", fn_140c0a400 as *const ()), // sub_140c0a400
    Replacement::draft(0x140c0a470, "unknown_140c0a470", fn_140c0a470 as *const ()), // sub_140c0a470
    Replacement::draft(0x140c0a570, "unknown_140c0a570", fn_140c0a570 as *const ()), // sub_140c0a570
    Replacement::draft(0x140c0a680, "unknown_140c0a680", fn_140c0a680 as *const ()), // sub_140c0a680
    Replacement::draft(0x140c0a760, "unknown_140c0a760", fn_140c0a760 as *const ()), // sub_140c0a760
    Replacement::draft(0x140c0a7c0, "unknown_140c0a7c0", fn_140c0a7c0 as *const ()), // sub_140c0a7c0
    Replacement::draft(0x140c0a830, "unknown_140c0a830", fn_140c0a830 as *const ()), // sub_140c0a830
    Replacement::draft(0x140c0a990, "unknown_140c0a990", fn_140c0a990 as *const ()), // sub_140c0a990
    Replacement::draft(0x140c0aa20, "unknown_140c0aa20", fn_140c0aa20 as *const ()), // sub_140c0aa20
    Replacement::draft(0x140c0aac0, "unknown_140c0aac0", fn_140c0aac0 as *const ()), // sub_140c0aac0
    Replacement::draft(0x140c0ab90, "unknown_140c0ab90", fn_140c0ab90 as *const ()), // sub_140c0ab90
    Replacement::draft(0x140c0ac60, "unknown_140c0ac60", fn_140c0ac60 as *const ()), // sub_140c0ac60
    Replacement::draft(0x140c0ad30, "unknown_140c0ad30", tpl_73552::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ce898, 0x1401e84a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140c0ad30
    Replacement::draft(0x140c0af60, "unknown_140c0af60", tpl_73552::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432d2118, 0x1401e84a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140c0af60
    Replacement::draft(0x140c0b190, "unknown_140c0b190", fn_140c0b190 as *const ()), // sub_140c0b190
    Replacement::draft(0x140c0b3c0, "unknown_140c0b3c0", fn_140c0b3c0 as *const ()), // sub_140c0b3c0
    Replacement::draft_throws(0x140c0b690, "unknown_140c0b690", fn_140c0b690 as *const (), Throws::Caught), // sub_140c0b690
    Replacement::draft_throws(0x140c0b970, "unknown_140c0b970", fn_140c0b970 as *const (), Throws::Caught), // sub_140c0b970
    Replacement::draft_throws(0x140c0bc50, "unknown_140c0bc50", fn_140c0bc50 as *const (), Throws::Caught), // sub_140c0bc50
    Replacement::draft_throws(0x140c0bf00, "unknown_140c0bf00", fn_140c0bf00 as *const (), Throws::Caught), // sub_140c0bf00
    Replacement::draft_throws(0x140c0c1b0, "unknown_140c0c1b0", fn_140c0c1b0 as *const (), Throws::Caught), // sub_140c0c1b0
    Replacement::draft_throws(0x140c0c450, "unknown_140c0c450", fn_140c0c450 as *const (), Throws::Caught), // sub_140c0c450
    Replacement::draft_throws(0x140c0c6f0, "unknown_140c0c6f0", fn_140c0c6f0 as *const (), Throws::Caught), // sub_140c0c6f0
    Replacement::draft_throws(0x140c0c7c0, "unknown_140c0c7c0", fn_140c0c7c0 as *const (), Throws::Caught), // sub_140c0c7c0
    Replacement::draft(0x140c0c890, "unknown_140c0c890", fn_140c0c890 as *const ()), // sub_140c0c890
    Replacement::draft_throws(0x140c0c8e0, "unknown_140c0c8e0", fn_140c0c8e0 as *const (), Throws::Caught), // sub_140c0c8e0
    Replacement::draft_throws(0x140c0cc60, "unknown_140c0cc60", fn_140c0cc60 as *const (), Throws::Caught), // sub_140c0cc60
    Replacement::draft_throws(0x140c0cfe0, "unknown_140c0cfe0", fn_140c0cfe0 as *const (), Throws::Caught), // sub_140c0cfe0
    Replacement::draft_throws(0x140c0d4d0, "unknown_140c0d4d0", fn_140c0d4d0 as *const (), Throws::Caught), // sub_140c0d4d0
    Replacement::draft_throws(0x140c0d990, "unknown_140c0d990", fn_140c0d990 as *const (), Throws::Caught), // sub_140c0d990
    Replacement::draft_throws(0x140c0de40, "unknown_140c0de40", fn_140c0de40 as *const (), Throws::Caught), // sub_140c0de40
    Replacement::draft(0x140c0df10, "unknown_140c0df10", fn_140c0df10 as *const ()), // sub_140c0df10
    Replacement::draft_throws(0x140c0e6c0, "unknown_140c0e6c0", fn_140c0e6c0 as *const (), Throws::Caught), // sub_140c0e6c0
    Replacement::draft_throws(0x140c0e9a0, "unknown_140c0e9a0", fn_140c0e9a0 as *const (), Throws::Caught), // sub_140c0e9a0
    Replacement::draft_throws(0x140c0ec50, "unknown_140c0ec50", fn_140c0ec50 as *const (), Throws::Caught), // sub_140c0ec50
    Replacement::draft_throws(0x140c0eef0, "unknown_140c0eef0", fn_140c0eef0 as *const (), Throws::Caught), // sub_140c0eef0
    Replacement::draft_throws(0x140c0efc0, "unknown_140c0efc0", fn_140c0efc0 as *const (), Throws::Caught), // sub_140c0efc0
    Replacement::draft_throws(0x140c0f2a0, "unknown_140c0f2a0", fn_140c0f2a0 as *const (), Throws::Caught), // sub_140c0f2a0
    Replacement::draft_throws(0x140c0f550, "unknown_140c0f550", fn_140c0f550 as *const (), Throws::Caught), // sub_140c0f550
    Replacement::draft_throws(0x140c0f7f0, "unknown_140c0f7f0", fn_140c0f7f0 as *const (), Throws::Caught), // sub_140c0f7f0
    Replacement::draft_throws(0x140c0f8c0, "unknown_140c0f8c0", fn_140c0f8c0 as *const (), Throws::Caught), // sub_140c0f8c0
    Replacement::draft_throws(0x140c0fbe0, "unknown_140c0fbe0", fn_140c0fbe0 as *const (), Throws::Caught), // sub_140c0fbe0
    Replacement::draft_throws(0x140c0ff00, "unknown_140c0ff00", fn_140c0ff00 as *const (), Throws::Caught), // sub_140c0ff00
    Replacement::draft_throws(0x140c103b0, "unknown_140c103b0", fn_140c103b0 as *const (), Throws::Caught), // sub_140c103b0
    Replacement::draft_throws(0x140c10810, "unknown_140c10810", fn_140c10810 as *const (), Throws::Caught), // sub_140c10810
    Replacement::draft_throws(0x140c10c60, "unknown_140c10c60", fn_140c10c60 as *const (), Throws::Caught), // sub_140c10c60
    Replacement::draft(0x140c10d30, "unknown_140c10d30", fn_140c10d30 as *const ()), // sub_140c10d30
    Replacement::draft_throws(0x140c10dd0, "unknown_140c10dd0", fn_140c10dd0 as *const (), Throws::Caught), // sub_140c10dd0
    Replacement::draft_throws(0x140c110b0, "unknown_140c110b0", fn_140c110b0 as *const (), Throws::Caught), // sub_140c110b0
    Replacement::draft_throws(0x140c11360, "unknown_140c11360", fn_140c11360 as *const (), Throws::Caught), // sub_140c11360
    Replacement::draft_throws(0x140c11600, "unknown_140c11600", fn_140c11600 as *const (), Throws::Caught), // sub_140c11600
    Replacement::draft_throws(0x140c116f0, "unknown_140c116f0", fn_140c116f0 as *const (), Throws::Caught), // sub_140c116f0
    Replacement::draft_throws(0x140c11a50, "unknown_140c11a50", fn_140c11a50 as *const (), Throws::Caught), // sub_140c11a50
    Replacement::draft_throws(0x140c11d80, "unknown_140c11d80", fn_140c11d80 as *const (), Throws::Caught), // sub_140c11d80
    Replacement::draft_throws(0x140c120a0, "unknown_140c120a0", fn_140c120a0 as *const (), Throws::Caught), // sub_140c120a0
    Replacement::draft_throws(0x140c12360, "unknown_140c12360", fn_140c12360 as *const (), Throws::Caught), // sub_140c12360
    Replacement::draft_throws(0x140c12620, "unknown_140c12620", fn_140c12620 as *const (), Throws::Caught), // sub_140c12620
    Replacement::draft_throws(0x140c12a50, "unknown_140c12a50", fn_140c12a50 as *const (), Throws::Caught), // sub_140c12a50
    Replacement::draft_throws(0x140c12e50, "unknown_140c12e50", fn_140c12e50 as *const (), Throws::Caught), // sub_140c12e50
    Replacement::draft_throws(0x140c13240, "unknown_140c13240", fn_140c13240 as *const (), Throws::Caught), // sub_140c13240
    Replacement::draft_throws(0x140c13310, "unknown_140c13310", fn_140c13310 as *const (), Throws::Caught), // sub_140c13310
    Replacement::draft_throws(0x140c135f0, "unknown_140c135f0", fn_140c135f0 as *const (), Throws::Caught), // sub_140c135f0
    Replacement::draft_throws(0x140c138a0, "unknown_140c138a0", fn_140c138a0 as *const (), Throws::Caught), // sub_140c138a0
    Replacement::draft_throws(0x140c13b40, "unknown_140c13b40", fn_140c13b40 as *const (), Throws::Caught), // sub_140c13b40
    Replacement::draft(0x140c13c10, "unknown_140c13c10", fn_140c13c10 as *const ()), // sub_140c13c10
    Replacement::draft_throws(0x140c13c50, "unknown_140c13c50", fn_140c13c50 as *const (), Throws::Caught), // sub_140c13c50
    Replacement::draft_throws(0x140c14200, "unknown_140c14200", fn_140c14200 as *const (), Throws::Caught), // sub_140c14200
    Replacement::draft_throws(0x140c14770, "unknown_140c14770", fn_140c14770 as *const (), Throws::Caught), // sub_140c14770
    Replacement::draft_throws(0x140c14bb0, "unknown_140c14bb0", fn_140c14bb0 as *const (), Throws::Caught), // sub_140c14bb0
    Replacement::draft_throws(0x140c14e90, "unknown_140c14e90", fn_140c14e90 as *const (), Throws::Caught), // sub_140c14e90
    Replacement::draft_throws(0x140c15170, "unknown_140c15170", fn_140c15170 as *const (), Throws::Caught), // sub_140c15170
    Replacement::draft_throws(0x140c15420, "unknown_140c15420", fn_140c15420 as *const (), Throws::Caught), // sub_140c15420
    Replacement::draft_throws(0x140c156d0, "unknown_140c156d0", fn_140c156d0 as *const (), Throws::Caught), // sub_140c156d0
    Replacement::draft_throws(0x140c15970, "unknown_140c15970", fn_140c15970 as *const (), Throws::Caught), // sub_140c15970
    Replacement::draft_throws(0x140c15c10, "unknown_140c15c10", fn_140c15c10 as *const (), Throws::Caught), // sub_140c15c10
    Replacement::draft_throws(0x140c15ce0, "unknown_140c15ce0", fn_140c15ce0 as *const (), Throws::Caught), // sub_140c15ce0
    Replacement::draft(0x140c15dc0, "unknown_140c15dc0", tpl_15153::<0x14384ea30, 0x1432d81e0, 0x1432d8280, 0x1432d82c0, 0x1401de410> as *const ()), // sub_140c15dc0
    Replacement::draft_throws(0x140c15e20, "unknown_140c15e20", fn_140c15e20 as *const (), Throws::Caught), // sub_140c15e20
    Replacement::draft_throws(0x140c16130, "unknown_140c16130", fn_140c16130 as *const (), Throws::Caught), // sub_140c16130
    Replacement::draft_throws(0x140c16440, "unknown_140c16440", fn_140c16440 as *const (), Throws::Caught), // sub_140c16440
    Replacement::draft_throws(0x140c168d0, "unknown_140c168d0", fn_140c168d0 as *const (), Throws::Caught), // sub_140c168d0
    Replacement::draft_throws(0x140c16d30, "unknown_140c16d30", fn_140c16d30 as *const (), Throws::Caught), // sub_140c16d30
    Replacement::draft_throws(0x140c17180, "unknown_140c17180", fn_140c17180 as *const (), Throws::Caught), // sub_140c17180
    Replacement::draft_throws(0x140c17270, "unknown_140c17270", fn_140c17270 as *const (), Throws::Caught), // sub_140c17270
    Replacement::draft_throws(0x140c17940, "unknown_140c17940", fn_140c17940 as *const (), Throws::Caught), // sub_140c17940
    Replacement::draft_throws(0x140c17fe0, "unknown_140c17fe0", fn_140c17fe0 as *const (), Throws::Caught), // sub_140c17fe0
    Replacement::draft_throws(0x140c18420, "unknown_140c18420", fn_140c18420 as *const (), Throws::Caught), // sub_140c18420
    Replacement::draft_throws(0x140c18be0, "unknown_140c18be0", fn_140c18be0 as *const (), Throws::Caught), // sub_140c18be0
    Replacement::draft_throws(0x140c19350, "unknown_140c19350", fn_140c19350 as *const (), Throws::Caught), // sub_140c19350
    Replacement::draft_throws(0x140c19790, "unknown_140c19790", fn_140c19790 as *const (), Throws::Caught), // sub_140c19790
    Replacement::draft_throws(0x140c19a70, "unknown_140c19a70", fn_140c19a70 as *const (), Throws::Caught), // sub_140c19a70
    Replacement::draft_throws(0x140c19d20, "unknown_140c19d20", fn_140c19d20 as *const (), Throws::Caught), // sub_140c19d20
    Replacement::draft_throws(0x140c19fc0, "unknown_140c19fc0", fn_140c19fc0 as *const (), Throws::Caught), // sub_140c19fc0
    Replacement::draft(0x140c1a090, "unknown_140c1a090", fn_140c1a090 as *const ()), // sub_140c1a090
    Replacement::draft_throws(0x140c1a0c0, "unknown_140c1a0c0", fn_140c1a0c0 as *const (), Throws::Caught), // sub_140c1a0c0
    Replacement::draft_throws(0x140c1a760, "unknown_140c1a760", fn_140c1a760 as *const (), Throws::Caught), // sub_140c1a760
    Replacement::draft_throws(0x140c1adc0, "unknown_140c1adc0", fn_140c1adc0 as *const (), Throws::Caught), // sub_140c1adc0
    Replacement::draft_throws(0x140c1b200, "unknown_140c1b200", fn_140c1b200 as *const (), Throws::Caught), // sub_140c1b200
    Replacement::draft_throws(0x140c1b530, "unknown_140c1b530", fn_140c1b530 as *const (), Throws::Caught), // sub_140c1b530
    Replacement::draft_throws(0x140c1b860, "unknown_140c1b860", fn_140c1b860 as *const (), Throws::Caught), // sub_140c1b860
    Replacement::draft_throws(0x140c1bcf0, "unknown_140c1bcf0", fn_140c1bcf0 as *const (), Throws::Caught), // sub_140c1bcf0
    Replacement::draft_throws(0x140c1c150, "unknown_140c1c150", fn_140c1c150 as *const (), Throws::Caught), // sub_140c1c150
    Replacement::draft_throws(0x140c1c5b0, "unknown_140c1c5b0", fn_140c1c5b0 as *const (), Throws::Caught), // sub_140c1c5b0
    Replacement::draft_throws(0x140c1c680, "unknown_140c1c680", fn_140c1c680 as *const (), Throws::Caught), // sub_140c1c680
    Replacement::draft_throws(0x140c1cc30, "unknown_140c1cc30", fn_140c1cc30 as *const (), Throws::Caught), // sub_140c1cc30
    Replacement::draft_throws(0x140c1d1a0, "unknown_140c1d1a0", fn_140c1d1a0 as *const (), Throws::Caught), // sub_140c1d1a0
    Replacement::draft_throws(0x140c1d5e0, "unknown_140c1d5e0", fn_140c1d5e0 as *const (), Throws::Caught), // sub_140c1d5e0
    Replacement::draft_throws(0x140c1d8c0, "unknown_140c1d8c0", fn_140c1d8c0 as *const (), Throws::Caught), // sub_140c1d8c0
    Replacement::draft_throws(0x140c1db70, "unknown_140c1db70", fn_140c1db70 as *const (), Throws::Caught), // sub_140c1db70
    Replacement::draft_throws(0x140c1de10, "unknown_140c1de10", fn_140c1de10 as *const (), Throws::Caught), // sub_140c1de10
    Replacement::draft(0x140c1dee0, "unknown_140c1dee0", fn_140c1dee0 as *const ()), // sub_140c1dee0
    Replacement::draft_throws(0x140c1df20, "unknown_140c1df20", fn_140c1df20 as *const (), Throws::Caught), // sub_140c1df20
    Replacement::draft_throws(0x140c1e200, "unknown_140c1e200", fn_140c1e200 as *const (), Throws::Caught), // sub_140c1e200
    Replacement::draft(0x140c1e4e0, "unknown_140c1e4e0", fn_140c1e4e0 as *const ()), // sub_140c1e4e0
    Replacement::draft_throws(0x140c1e810, "unknown_140c1e810", fn_140c1e810 as *const (), Throws::Caught), // sub_140c1e810
    Replacement::draft_throws(0x140c1ec60, "unknown_140c1ec60", fn_140c1ec60 as *const (), Throws::Caught), // sub_140c1ec60
    Replacement::draft_throws(0x140c1f080, "unknown_140c1f080", fn_140c1f080 as *const (), Throws::Caught), // sub_140c1f080
    Replacement::draft_throws(0x140c1f490, "unknown_140c1f490", fn_140c1f490 as *const (), Throws::Caught), // sub_140c1f490
    Replacement::draft(0x140c1f560, "unknown_140c1f560", fn_140c1f560 as *const ()), // sub_140c1f560
    Replacement::draft_throws(0x140c1f5b0, "unknown_140c1f5b0", fn_140c1f5b0 as *const (), Throws::Caught), // sub_140c1f5b0
    Replacement::draft_throws(0x140c1fcb0, "unknown_140c1fcb0", fn_140c1fcb0 as *const (), Throws::Caught), // sub_140c1fcb0
    Replacement::draft_throws(0x140c20370, "unknown_140c20370", fn_140c20370 as *const (), Throws::Caught), // sub_140c20370
    Replacement::draft_throws(0x140c207b0, "unknown_140c207b0", fn_140c207b0 as *const (), Throws::Caught), // sub_140c207b0
    Replacement::draft_throws(0x140c20eb0, "unknown_140c20eb0", fn_140c20eb0 as *const (), Throws::Caught), // sub_140c20eb0
    Replacement::draft_throws(0x140c21460, "unknown_140c21460", fn_140c21460 as *const (), Throws::Caught), // sub_140c21460
    Replacement::draft_throws(0x140c21dc0, "unknown_140c21dc0", fn_140c21dc0 as *const (), Throws::Caught), // sub_140c21dc0
    Replacement::draft_throws(0x140c22490, "unknown_140c22490", fn_140c22490 as *const (), Throws::Caught), // sub_140c22490
    Replacement::draft_throws(0x140c22a00, "unknown_140c22a00", fn_140c22a00 as *const (), Throws::Caught), // sub_140c22a00
    Replacement::draft_throws(0x140c23330, "unknown_140c23330", fn_140c23330 as *const (), Throws::Caught), // sub_140c23330
    Replacement::draft_throws(0x140c23770, "unknown_140c23770", fn_140c23770 as *const (), Throws::Caught), // sub_140c23770
    Replacement::draft_throws(0x140c23bb0, "unknown_140c23bb0", fn_140c23bb0 as *const (), Throws::Caught), // sub_140c23bb0
    Replacement::draft_throws(0x140c240a0, "unknown_140c240a0", fn_140c240a0 as *const (), Throws::Caught), // sub_140c240a0
    Replacement::draft_throws(0x140c24680, "unknown_140c24680", fn_140c24680 as *const (), Throws::Caught), // sub_140c24680
    Replacement::draft_throws(0x140c24c30, "unknown_140c24c30", fn_140c24c30 as *const (), Throws::Caught), // sub_140c24c30
    Replacement::draft_throws(0x140c251e0, "unknown_140c251e0", fn_140c251e0 as *const (), Throws::Caught), // sub_140c251e0
    Replacement::draft(0x140c252b0, "unknown_140c252b0", tpl_15153::<0x14384e908, 0x1432d7820, 0x1432d77d0, 0x1432d7810, 0x1401de410> as *const ()), // sub_140c252b0
    Replacement::draft_throws(0x140c25300, "unknown_140c25300", fn_140c25300 as *const (), Throws::Caught), // sub_140c25300
    Replacement::draft_throws(0x140c258b0, "unknown_140c258b0", fn_140c258b0 as *const (), Throws::Caught), // sub_140c258b0
    Replacement::draft_throws(0x140c25e20, "unknown_140c25e20", fn_140c25e20 as *const (), Throws::Caught), // sub_140c25e20
    Replacement::draft_throws(0x140c26260, "unknown_140c26260", fn_140c26260 as *const (), Throws::Caught), // sub_140c26260
    Replacement::draft_throws(0x140c26540, "unknown_140c26540", fn_140c26540 as *const (), Throws::Caught), // sub_140c26540
    Replacement::draft_throws(0x140c267f0, "unknown_140c267f0", fn_140c267f0 as *const (), Throws::Caught), // sub_140c267f0
    Replacement::draft_throws(0x140c26a90, "unknown_140c26a90", fn_140c26a90 as *const (), Throws::Caught), // sub_140c26a90
    Replacement::draft(0x140c26b60, "unknown_140c26b60", tpl_15153::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // sub_140c26b60
    Replacement::draft_throws(0x140c26bb0, "unknown_140c26bb0", fn_140c26bb0 as *const (), Throws::Caught), // sub_140c26bb0
    Replacement::draft_throws(0x140c26e90, "unknown_140c26e90", fn_140c26e90 as *const (), Throws::Caught), // sub_140c26e90
    Replacement::draft_throws(0x140c27140, "unknown_140c27140", fn_140c27140 as *const (), Throws::Caught), // sub_140c27140
    Replacement::draft_throws(0x140c273e0, "unknown_140c273e0", fn_140c273e0 as *const (), Throws::Caught), // sub_140c273e0
    Replacement::draft_throws(0x140c274b0, "unknown_140c274b0", fn_140c274b0 as *const (), Throws::Caught), // sub_140c274b0
    Replacement::draft_throws(0x140c277a0, "unknown_140c277a0", fn_140c277a0 as *const (), Throws::Caught), // sub_140c277a0
    Replacement::draft_throws(0x140c27a60, "unknown_140c27a60", fn_140c27a60 as *const (), Throws::Caught), // sub_140c27a60
    Replacement::draft_throws(0x140c27d10, "unknown_140c27d10", fn_140c27d10 as *const (), Throws::Caught), // sub_140c27d10
    Replacement::draft_throws(0x140c28010, "unknown_140c28010", fn_140c28010 as *const (), Throws::Caught), // sub_140c28010
    Replacement::draft_throws(0x140c282d0, "unknown_140c282d0", fn_140c282d0 as *const (), Throws::Caught), // sub_140c282d0
    Replacement::draft_throws(0x140c28570, "unknown_140c28570", fn_140c28570 as *const (), Throws::Caught), // sub_140c28570
    Replacement::draft_throws(0x140c28660, "unknown_140c28660", fn_140c28660 as *const (), Throws::Caught), // sub_140c28660
    Replacement::draft_throws(0x140c28940, "unknown_140c28940", fn_140c28940 as *const (), Throws::Caught), // sub_140c28940
    Replacement::draft_throws(0x140c28bf0, "unknown_140c28bf0", fn_140c28bf0 as *const (), Throws::Caught), // sub_140c28bf0
    Replacement::draft_throws(0x140c28e90, "unknown_140c28e90", fn_140c28e90 as *const (), Throws::Caught), // sub_140c28e90
    Replacement::draft_throws(0x140c28f60, "unknown_140c28f60", fn_140c28f60 as *const (), Throws::Caught), // sub_140c28f60
    Replacement::draft_throws(0x140c29220, "unknown_140c29220", fn_140c29220 as *const (), Throws::Caught), // sub_140c29220
    Replacement::draft_throws(0x140c294e0, "unknown_140c294e0", fn_140c294e0 as *const (), Throws::Caught), // sub_140c294e0
    Replacement::draft_throws(0x140c29920, "unknown_140c29920", fn_140c29920 as *const (), Throws::Caught), // sub_140c29920
    Replacement::draft_throws(0x140c29d30, "unknown_140c29d30", fn_140c29d30 as *const (), Throws::Caught), // sub_140c29d30
    Replacement::draft_throws(0x140c2a130, "unknown_140c2a130", fn_140c2a130 as *const (), Throws::Caught), // sub_140c2a130
    Replacement::draft(0x140c2a200, "unknown_140c2a200", tpl_15203::<0x1432d9680, 0x1432d93c8, 0x140c05f40> as *const ()), // sub_140c2a200
    Replacement::draft(0x140c2a240, "unknown_140c2a240", tpl_15203::<0x1432d9680, 0x1432d93c8, 0x140c05f70> as *const ()), // sub_140c2a240
    Replacement::draft_throws(0x140c2a280, "unknown_140c2a280", fn_140c2a280 as *const (), Throws::Caught), // sub_140c2a280
    Replacement::draft_throws(0x140c2a590, "unknown_140c2a590", fn_140c2a590 as *const (), Throws::Caught), // sub_140c2a590
    Replacement::draft_throws(0x140c2a8a0, "unknown_140c2a8a0", fn_140c2a8a0 as *const (), Throws::Caught), // sub_140c2a8a0
    Replacement::draft_throws(0x140c2ad50, "unknown_140c2ad50", fn_140c2ad50 as *const (), Throws::Caught), // sub_140c2ad50
    Replacement::draft_throws(0x140c2b190, "unknown_140c2b190", fn_140c2b190 as *const (), Throws::Caught), // sub_140c2b190
    Replacement::draft_throws(0x140c2b5c0, "unknown_140c2b5c0", fn_140c2b5c0 as *const (), Throws::Caught), // sub_140c2b5c0
    Replacement::draft(0x140c2b690, "unknown_140c2b690", fn_140c2b690 as *const ()), // sub_140c2b690
    Replacement::draft_throws(0x140c2b6d0, "unknown_140c2b6d0", fn_140c2b6d0 as *const (), Throws::Caught), // sub_140c2b6d0
    Replacement::draft_throws(0x140c2b9d0, "unknown_140c2b9d0", fn_140c2b9d0 as *const (), Throws::Caught), // sub_140c2b9d0
    Replacement::draft_throws(0x140c2bc90, "unknown_140c2bc90", fn_140c2bc90 as *const (), Throws::Caught), // sub_140c2bc90
    Replacement::draft_throws(0x140c2bf50, "unknown_140c2bf50", fn_140c2bf50 as *const (), Throws::Caught), // sub_140c2bf50
    Replacement::draft_throws(0x140c2c240, "unknown_140c2c240", fn_140c2c240 as *const (), Throws::Caught), // sub_140c2c240
    Replacement::draft_throws(0x140c2c530, "unknown_140c2c530", fn_140c2c530 as *const (), Throws::Caught), // sub_140c2c530
    Replacement::draft_throws(0x140c2c9c0, "unknown_140c2c9c0", fn_140c2c9c0 as *const (), Throws::Caught), // sub_140c2c9c0
    Replacement::draft_throws(0x140c2cde0, "unknown_140c2cde0", fn_140c2cde0 as *const (), Throws::Caught), // sub_140c2cde0
    Replacement::draft_throws(0x140c2d200, "unknown_140c2d200", fn_140c2d200 as *const (), Throws::Caught), // sub_140c2d200
    Replacement::draft(0x140c2d2d0, "unknown_140c2d2d0", fn_140c2d2d0 as *const ()), // sub_140c2d2d0
    Replacement::draft_throws(0x140c2d300, "unknown_140c2d300", fn_140c2d300 as *const (), Throws::Caught), // sub_140c2d300
    Replacement::draft_throws(0x140c2d5e0, "unknown_140c2d5e0", fn_140c2d5e0 as *const (), Throws::Caught), // sub_140c2d5e0
    Replacement::draft_throws(0x140c2d890, "unknown_140c2d890", fn_140c2d890 as *const (), Throws::Caught), // sub_140c2d890
    Replacement::draft_throws(0x140c2db30, "unknown_140c2db30", fn_140c2db30 as *const (), Throws::Caught), // sub_140c2db30
    Replacement::draft_throws(0x140c2dc00, "unknown_140c2dc00", fn_140c2dc00 as *const (), Throws::Caught), // sub_140c2dc00
    Replacement::draft_throws(0x140c2ded0, "unknown_140c2ded0", fn_140c2ded0 as *const (), Throws::Caught), // sub_140c2ded0
    Replacement::draft_throws(0x140c2e170, "unknown_140c2e170", fn_140c2e170 as *const (), Throws::Caught), // sub_140c2e170
    Replacement::draft_throws(0x140c2e400, "unknown_140c2e400", fn_140c2e400 as *const (), Throws::Caught), // sub_140c2e400
    Replacement::draft_throws(0x140c2e4d0, "unknown_140c2e4d0", fn_140c2e4d0 as *const (), Throws::Caught), // sub_140c2e4d0
    Replacement::draft_throws(0x140c2e7d0, "unknown_140c2e7d0", fn_140c2e7d0 as *const (), Throws::Caught), // sub_140c2e7d0
    Replacement::draft_throws(0x140c2ead0, "unknown_140c2ead0", fn_140c2ead0 as *const (), Throws::Caught), // sub_140c2ead0
    Replacement::draft_throws(0x140c2edd0, "unknown_140c2edd0", fn_140c2edd0 as *const (), Throws::Caught), // sub_140c2edd0
    Replacement::draft_throws(0x140c2f0d0, "unknown_140c2f0d0", fn_140c2f0d0 as *const (), Throws::Caught), // sub_140c2f0d0
    Replacement::draft_throws(0x140c2f580, "unknown_140c2f580", fn_140c2f580 as *const (), Throws::Caught), // sub_140c2f580
    Replacement::draft_throws(0x140c2fa30, "unknown_140c2fa30", fn_140c2fa30 as *const (), Throws::Caught), // sub_140c2fa30
    Replacement::draft_throws(0x140c2fe60, "unknown_140c2fe60", fn_140c2fe60 as *const (), Throws::Caught), // sub_140c2fe60
    Replacement::draft_throws(0x140c30290, "unknown_140c30290", fn_140c30290 as *const (), Throws::Caught), // sub_140c30290
    Replacement::draft_throws(0x140c306c0, "unknown_140c306c0", fn_140c306c0 as *const (), Throws::Caught), // sub_140c306c0
    Replacement::draft_throws(0x140c30af0, "unknown_140c30af0", fn_140c30af0 as *const (), Throws::Caught), // sub_140c30af0
    Replacement::draft_throws(0x140c30bc0, "unknown_140c30bc0", fn_140c30bc0 as *const (), Throws::Caught), // sub_140c30bc0
    Replacement::draft_throws(0x140c30c90, "unknown_140c30c90", fn_140c30c90 as *const (), Throws::Caught), // sub_140c30c90
    Replacement::draft_throws(0x140c30f70, "unknown_140c30f70", fn_140c30f70 as *const (), Throws::Caught), // sub_140c30f70
    Replacement::draft_throws(0x140c31250, "unknown_140c31250", fn_140c31250 as *const (), Throws::Caught), // sub_140c31250
    Replacement::draft_throws(0x140c31530, "unknown_140c31530", fn_140c31530 as *const (), Throws::Caught), // sub_140c31530
    Replacement::draft_throws(0x140c317e0, "unknown_140c317e0", fn_140c317e0 as *const (), Throws::Caught), // sub_140c317e0
    Replacement::draft_throws(0x140c31a90, "unknown_140c31a90", fn_140c31a90 as *const (), Throws::Caught), // sub_140c31a90
    Replacement::draft_throws(0x140c31d40, "unknown_140c31d40", fn_140c31d40 as *const (), Throws::Caught), // sub_140c31d40
    Replacement::draft_throws(0x140c31fe0, "unknown_140c31fe0", fn_140c31fe0 as *const (), Throws::Caught), // sub_140c31fe0
    Replacement::draft_throws(0x140c32280, "unknown_140c32280", fn_140c32280 as *const (), Throws::Caught), // sub_140c32280
    Replacement::draft_throws(0x140c32520, "unknown_140c32520", fn_140c32520 as *const (), Throws::Caught), // sub_140c32520
    Replacement::draft_throws(0x140c325f0, "unknown_140c325f0", fn_140c325f0 as *const (), Throws::Caught), // sub_140c325f0
    Replacement::draft_throws(0x140c326c0, "unknown_140c326c0", fn_140c326c0 as *const (), Throws::Caught), // sub_140c326c0
    Replacement::draft_throws(0x140c32790, "unknown_140c32790", fn_140c32790 as *const (), Throws::Caught), // sub_140c32790
    Replacement::draft_throws(0x140c32d70, "unknown_140c32d70", fn_140c32d70 as *const (), Throws::Caught), // sub_140c32d70
    Replacement::draft_throws(0x140c33320, "unknown_140c33320", fn_140c33320 as *const (), Throws::Caught), // sub_140c33320
    Replacement::draft_throws(0x140c338d0, "unknown_140c338d0", fn_140c338d0 as *const (), Throws::Caught), // sub_140c338d0
    Replacement::draft(0x140c339a0, "unknown_140c339a0", tpl_15153::<0x14384e908, 0x1430aee20, 0x1432d77d0, 0x1432d7810, 0x1401de410> as *const ()), // sub_140c339a0
    Replacement::draft(0x140c339f0, "unknown_140c339f0", fn_140c339f0 as *const ()), // sub_140c339f0
    Replacement::draft(0x140c33a60, "unknown_140c33a60", tpl_15228::<0x1432c9630, 0x10> as *const ()), // sub_140c33a60
    Replacement::draft(0x140c34ac0, "unknown_140c34ac0", fn_140c34ac0 as *const ()), // sub_140c34ac0
    Replacement::draft(0x140c35140, "unknown_140c35140", fn_140c35140 as *const ()), // sub_140c35140
    Replacement::draft(0x140c35850, "unknown_140c35850", fn_140c35850 as *const ()), // sub_140c35850
    Replacement::draft(0x140c35ed0, "unknown_140c35ed0", fn_140c35ed0 as *const ()), // sub_140c35ed0
    Replacement::draft(0x140c36280, "unknown_140c36280", fn_140c36280 as *const ()), // sub_140c36280
    Replacement::draft(0x140c364b0, "unknown_140c364b0", fn_140c364b0 as *const ()), // sub_140c364b0
    Replacement::draft(0x140c36610, "unknown_140c36610", fn_140c36610 as *const ()), // sub_140c36610
    Replacement::draft_throws(0x140c368e0, "unknown_140c368e0", fn_140c368e0 as *const (), Throws::Caught), // sub_140c368e0
    Replacement::draft_throws(0x140c36db0, "unknown_140c36db0", fn_140c36db0 as *const (), Throws::Caught), // sub_140c36db0
    Replacement::draft_throws(0x140c37280, "unknown_140c37280", fn_140c37280 as *const (), Throws::Caught), // sub_140c37280
    Replacement::draft_throws(0x140c37710, "unknown_140c37710", fn_140c37710 as *const (), Throws::Caught), // sub_140c37710
    Replacement::draft_throws(0x140c37ba0, "unknown_140c37ba0", fn_140c37ba0 as *const (), Throws::Caught), // sub_140c37ba0
    Replacement::draft_throws(0x140c38030, "unknown_140c38030", fn_140c38030 as *const (), Throws::Caught), // sub_140c38030
    Replacement::draft_throws(0x140c384c0, "unknown_140c384c0", fn_140c384c0 as *const (), Throws::Caught), // sub_140c384c0
    Replacement::draft_throws(0x140c38990, "unknown_140c38990", fn_140c38990 as *const (), Throws::Caught), // sub_140c38990
    Replacement::draft_throws(0x140c38e20, "unknown_140c38e20", fn_140c38e20 as *const (), Throws::Caught), // sub_140c38e20
    Replacement::draft_throws(0x140c392b0, "unknown_140c392b0", fn_140c392b0 as *const (), Throws::Caught), // sub_140c392b0
    Replacement::draft_throws(0x140c39780, "unknown_140c39780", fn_140c39780 as *const (), Throws::Caught), // sub_140c39780
    Replacement::draft_throws(0x140c39c10, "unknown_140c39c10", fn_140c39c10 as *const (), Throws::Caught), // sub_140c39c10
    Replacement::draft_throws(0x140c3a0a0, "unknown_140c3a0a0", fn_140c3a0a0 as *const (), Throws::Caught), // sub_140c3a0a0
    Replacement::draft_throws(0x140c3a570, "unknown_140c3a570", fn_140c3a570 as *const (), Throws::Caught), // sub_140c3a570
    Replacement::draft_throws(0x140c3aa00, "unknown_140c3aa00", fn_140c3aa00 as *const (), Throws::Caught), // sub_140c3aa00
    Replacement::draft_throws(0x140c3ae90, "unknown_140c3ae90", fn_140c3ae90 as *const (), Throws::Caught), // sub_140c3ae90
    Replacement::draft_throws(0x140c3b360, "unknown_140c3b360", fn_140c3b360 as *const (), Throws::Caught), // sub_140c3b360
    Replacement::draft_throws(0x140c3b7f0, "unknown_140c3b7f0", fn_140c3b7f0 as *const (), Throws::Caught), // sub_140c3b7f0
    Replacement::draft_throws(0x140c3bc80, "unknown_140c3bc80", fn_140c3bc80 as *const (), Throws::Caught), // sub_140c3bc80
    Replacement::draft_throws(0x140c3c150, "unknown_140c3c150", fn_140c3c150 as *const (), Throws::Caught), // sub_140c3c150
    Replacement::draft_throws(0x140c3c5e0, "unknown_140c3c5e0", fn_140c3c5e0 as *const (), Throws::Caught), // sub_140c3c5e0
    Replacement::draft_throws(0x140c3ca70, "unknown_140c3ca70", fn_140c3ca70 as *const (), Throws::Caught), // sub_140c3ca70
    Replacement::draft_throws(0x140c3cf40, "unknown_140c3cf40", fn_140c3cf40 as *const (), Throws::Caught), // sub_140c3cf40
    Replacement::draft_throws(0x140c3d3c0, "unknown_140c3d3c0", fn_140c3d3c0 as *const (), Throws::Caught), // sub_140c3d3c0
    Replacement::draft_throws(0x140c3d840, "unknown_140c3d840", fn_140c3d840 as *const (), Throws::Caught), // sub_140c3d840
    Replacement::draft_throws(0x140c3dd10, "unknown_140c3dd10", fn_140c3dd10 as *const (), Throws::Caught), // sub_140c3dd10
    Replacement::draft_throws(0x140c3e1a0, "unknown_140c3e1a0", fn_140c3e1a0 as *const (), Throws::Caught), // sub_140c3e1a0
    Replacement::draft(0x140c3e630, "unknown_140c3e630", fn_140c3e630 as *const ()), // sub_140c3e630
    Replacement::draft_throws(0x140c3e760, "unknown_140c3e760", fn_140c3e760 as *const (), Throws::Caught), // sub_140c3e760
    Replacement::draft(0x140c3e8b0, "unknown_140c3e8b0", fn_140c3e8b0 as *const ()), // sub_140c3e8b0
    Replacement::draft(0x140c3e9e0, "unknown_140c3e9e0", fn_140c3e9e0 as *const ()), // sub_140c3e9e0
    Replacement::draft_throws(0x140c3ead0, "unknown_140c3ead0", fn_140c3ead0 as *const (), Throws::Discard), // sub_140c3ead0
    Replacement::draft(0x140c3ebe0, "unknown_140c3ebe0", fn_140c3ebe0 as *const ()), // sub_140c3ebe0
    Replacement::draft(0x140c3ecb0, "unknown_140c3ecb0", fn_140c3ecb0 as *const ()), // sub_140c3ecb0
    Replacement::draft_throws(0x140c3ed90, "unknown_140c3ed90", fn_140c3ed90 as *const (), Throws::Discard), // sub_140c3ed90
    Replacement::draft_throws(0x140c3ee70, "unknown_140c3ee70", fn_140c3ee70 as *const (), Throws::Caught), // sub_140c3ee70
    Replacement::draft_throws(0x140c3f340, "unknown_140c3f340", fn_140c3f340 as *const (), Throws::Caught), // sub_140c3f340
    Replacement::draft_throws(0x140c3f810, "unknown_140c3f810", fn_140c3f810 as *const (), Throws::Caught), // sub_140c3f810
    Replacement::draft_throws(0x140c3fca0, "unknown_140c3fca0", fn_140c3fca0 as *const (), Throws::Caught), // sub_140c3fca0
    Replacement::draft_throws(0x140c40130, "unknown_140c40130", fn_140c40130 as *const (), Throws::Caught), // sub_140c40130
    Replacement::draft_throws(0x140c405c0, "unknown_140c405c0", fn_140c405c0 as *const (), Throws::Caught), // sub_140c405c0
    Replacement::draft_throws(0x140c40a50, "unknown_140c40a50", fn_140c40a50 as *const (), Throws::Caught), // sub_140c40a50
    Replacement::draft_throws(0x140c40f20, "unknown_140c40f20", fn_140c40f20 as *const (), Throws::Caught), // sub_140c40f20
    Replacement::draft_throws(0x140c413b0, "unknown_140c413b0", fn_140c413b0 as *const (), Throws::Caught), // sub_140c413b0
    Replacement::draft(0x140c41840, "unknown_140c41840", tpl_1060::<0x14387f308, 0x14243b0b0, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140c41840
    Replacement::draft(0x140c418d0, "unknown_140c418d0", fn_140c418d0 as *const ()), // sub_140c418d0
    Replacement::draft_throws(0x140c419e0, "unknown_140c419e0", fn_140c419e0 as *const (), Throws::Discard), // sub_140c419e0
    Replacement::draft(0x140c41ac0, "unknown_140c41ac0", fn_140c41ac0 as *const ()), // sub_140c41ac0
    Replacement::draft(0x140c41b60, "unknown_140c41b60", fn_140c41b60 as *const ()), // sub_140c41b60
    Replacement::draft_throws(0x140c41c10, "unknown_140c41c10", fn_140c41c10 as *const (), Throws::Discard), // sub_140c41c10
    Replacement::draft(0x140c41d10, "unknown_140c41d10", fn_140c41d10 as *const ()), // sub_140c41d10
    Replacement::draft(0x140c41dc0, "unknown_140c41dc0", fn_140c41dc0 as *const ()), // sub_140c41dc0
    Replacement::draft_throws(0x140c41e80, "unknown_140c41e80", fn_140c41e80 as *const (), Throws::Discard), // sub_140c41e80
    Replacement::draft(0x140c41f40, "unknown_140c41f40", fn_140c41f40 as *const ()), // sub_140c41f40
    Replacement::draft(0x140c42040, "unknown_140c42040", fn_140c42040 as *const ()), // sub_140c42040
    Replacement::draft_throws(0x140c42250, "unknown_140c42250", fn_140c42250 as *const (), Throws::Discard), // sub_140c42250
    Replacement::draft(0x140c42460, "unknown_140c42460", fn_140c42460 as *const ()), // sub_140c42460
    Replacement::draft(0x140c425c0, "unknown_140c425c0", fn_140c425c0 as *const ()), // sub_140c425c0
    Replacement::draft_throws(0x140c42710, "unknown_140c42710", fn_140c42710 as *const (), Throws::Discard), // sub_140c42710
    Replacement::draft(0x140c42940, "unknown_140c42940", fn_140c42940 as *const ()), // sub_140c42940
    Replacement::draft(0x140c429b0, "unknown_140c429b0", fn_140c429b0 as *const ()), // sub_140c429b0
    Replacement::draft(0x140c42b60, "unknown_140c42b60", fn_140c42b60 as *const ()), // sub_140c42b60
    Replacement::draft_throws(0x140c42d10, "unknown_140c42d10", fn_140c42d10 as *const (), Throws::Discard), // sub_140c42d10
    Replacement::draft_throws(0x140c42e90, "unknown_140c42e90", fn_140c42e90 as *const (), Throws::Caught), // sub_140c42e90
    Replacement::draft_throws(0x140c43360, "unknown_140c43360", fn_140c43360 as *const (), Throws::Caught), // sub_140c43360
    Replacement::draft_throws(0x140c437f0, "unknown_140c437f0", fn_140c437f0 as *const (), Throws::Caught), // sub_140c437f0
    Replacement::draft(0x140c43c80, "unknown_140c43c80", tpl_15275::<0x14387f460, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c43c80
    Replacement::draft_throws(0x140c43d20, "unknown_140c43d20", fn_140c43d20 as *const (), Throws::Discard), // sub_140c43d20
    Replacement::draft(0x140c43da0, "unknown_140c43da0", fn_140c43da0 as *const ()), // sub_140c43da0
    Replacement::draft(0x140c43e10, "unknown_140c43e10", tpl_2076::<0x14387f460, 0x14243b0b0, 0x338, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c43e10
    Replacement::draft_throws(0x140c43e80, "unknown_140c43e80", fn_140c43e80 as *const (), Throws::Discard), // sub_140c43e80
    Replacement::draft(0x140c43f10, "unknown_140c43f10", tpl_2080::<0x14387f2c0, 0x14387f460, 0x1401e9970, 0x338, 0xd8, 0x1401e9860> as *const ()), // sub_140c43f10
    Replacement::draft(0x140c43f80, "unknown_140c43f80", tpl_2076::<0x14387f460, 0x14243b0b0, 0x338, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c43f80
    Replacement::draft_throws(0x140c43ff0, "unknown_140c43ff0", tpl_15279::<0x14387f2c0, 0x14387f460, 0x1401e9770, 0x338, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140c43ff0
    Replacement::draft_throws(0x140c44060, "unknown_140c44060", fn_140c44060 as *const (), Throws::Caught), // sub_140c44060
    Replacement::draft_throws(0x140c44530, "unknown_140c44530", fn_140c44530 as *const (), Throws::Caught), // sub_140c44530
    Replacement::draft_throws(0x140c449c0, "unknown_140c449c0", fn_140c449c0 as *const (), Throws::Caught), // sub_140c449c0
    Replacement::draft(0x140c44e50, "unknown_140c44e50", fn_140c44e50 as *const ()), // sub_140c44e50
    Replacement::draft_throws(0x140c44f70, "unknown_140c44f70", fn_140c44f70 as *const (), Throws::Caught), // sub_140c44f70
    Replacement::draft(0x140c45090, "unknown_140c45090", fn_140c45090 as *const ()), // sub_140c45090
    Replacement::draft(0x140c451c0, "unknown_140c451c0", fn_140c451c0 as *const ()), // sub_140c451c0
    Replacement::draft_throws(0x140c452a0, "unknown_140c452a0", fn_140c452a0 as *const (), Throws::Discard), // sub_140c452a0
    Replacement::draft(0x140c45390, "unknown_140c45390", fn_140c45390 as *const ()), // sub_140c45390
    Replacement::draft(0x140c45440, "unknown_140c45440", fn_140c45440 as *const ()), // sub_140c45440
    Replacement::draft_throws(0x140c45500, "unknown_140c45500", fn_140c45500 as *const (), Throws::Discard), // sub_140c45500
    Replacement::draft_throws(0x140c455c0, "unknown_140c455c0", fn_140c455c0 as *const (), Throws::Caught), // sub_140c455c0
    Replacement::draft_throws(0x140c45a90, "unknown_140c45a90", fn_140c45a90 as *const (), Throws::Caught), // sub_140c45a90
    Replacement::draft_throws(0x140c45f20, "unknown_140c45f20", fn_140c45f20 as *const (), Throws::Caught), // sub_140c45f20
    Replacement::draft_throws(0x140c463b0, "unknown_140c463b0", fn_140c463b0 as *const (), Throws::Caught), // sub_140c463b0
    Replacement::draft_throws(0x140c46880, "unknown_140c46880", fn_140c46880 as *const (), Throws::Caught), // sub_140c46880
    Replacement::draft_throws(0x140c46d10, "unknown_140c46d10", fn_140c46d10 as *const (), Throws::Caught), // sub_140c46d10
    Replacement::draft(0x140c471a0, "unknown_140c471a0", tpl_1060::<0x14387f318, 0x14243b0b0, 0x318, 0x328, 0x320, 0x8, 0xc, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140c471a0
    Replacement::draft(0x140c47230, "unknown_140c47230", fn_140c47230 as *const ()), // sub_140c47230
    Replacement::draft_throws(0x140c47400, "unknown_140c47400", fn_140c47400 as *const (), Throws::Discard), // sub_140c47400
    Replacement::draft(0x140c47590, "unknown_140c47590", fn_140c47590 as *const ()), // sub_140c47590
    Replacement::draft(0x140c47650, "unknown_140c47650", fn_140c47650 as *const ()), // sub_140c47650
    Replacement::draft_throws(0x140c47760, "unknown_140c47760", fn_140c47760 as *const (), Throws::Discard), // sub_140c47760
    Replacement::draft(0x140c47910, "unknown_140c47910", fn_140c47910 as *const ()), // sub_140c47910
    Replacement::draft(0x140c47a20, "unknown_140c47a20", fn_140c47a20 as *const ()), // sub_140c47a20
    Replacement::draft_throws(0x140c47b40, "unknown_140c47b40", fn_140c47b40 as *const (), Throws::Discard), // sub_140c47b40
    Replacement::draft(0x140c47c60, "unknown_140c47c60", tpl_15299::<0x14387f3a0, 0x14243b0b0, 0x140ca1170, 0xd8, 0x1b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c47c60
    Replacement::draft(0x140c47cf0, "unknown_140c47cf0", fn_140c47cf0 as *const ()), // sub_140c47cf0
    Replacement::draft(0x140c47e20, "unknown_140c47e20", tpl_15299::<0x14387f3a0, 0x14243b0b0, 0x140ca1250, 0xd8, 0x1b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c47e20
    Replacement::draft(0x140c47eb0, "unknown_140c47eb0", tpl_10087::<0x14387f228, 0x14243b0b0, 0x1406dd2c0, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c47eb0
    Replacement::draft(0x140c47f30, "unknown_140c47f30", fn_140c47f30 as *const ()), // sub_140c47f30
    Replacement::draft_throws(0x140c48070, "unknown_140c48070", tpl_15302::<0x14387f3d4, 0x14387f3a0, 0x1401e9fa0, 0x140c65550, 0xd8, 0x1b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140c48070
    Replacement::draft(0x140c480f0, "unknown_140c480f0", tpl_15299::<0x14387f3a0, 0x14243b0b0, 0x140ca1560, 0xd8, 0x1b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c480f0
    Replacement::draft_throws(0x140c48180, "unknown_140c48180", fn_140c48180 as *const (), Throws::Discard), // sub_140c48180
    Replacement::draft(0x140c48290, "unknown_140c48290", fn_140c48290 as *const ()), // sub_140c48290
    Replacement::draft_throws(0x140c48360, "unknown_140c48360", fn_140c48360 as *const (), Throws::Discard), // sub_140c48360
    Replacement::draft(0x140c484e0, "unknown_140c484e0", fn_140c484e0 as *const ()), // sub_140c484e0
    Replacement::draft(0x140c48650, "unknown_140c48650", tpl_15299::<0x14387f3a0, 0x14243b0b0, 0x140ca15e0, 0xd8, 0x1b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c48650
    Replacement::draft(0x140c486e0, "unknown_140c486e0", fn_140c486e0 as *const ()), // sub_140c486e0
    Replacement::draft(0x140c48760, "unknown_140c48760", fn_140c48760 as *const ()), // sub_140c48760
    Replacement::draft_throws(0x140c488a0, "unknown_140c488a0", tpl_15302::<0x14387f3d4, 0x14387f3a0, 0x1401e9b60, 0x140c657c0, 0xd8, 0x1b0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140c488a0
    Replacement::draft_throws(0x140c48920, "unknown_140c48920", fn_140c48920 as *const (), Throws::Discard), // sub_140c48920
    Replacement::draft_throws(0x140c48a30, "unknown_140c48a30", fn_140c48a30 as *const (), Throws::Discard), // sub_140c48a30
    Replacement::draft(0x140c48bb0, "unknown_140c48bb0", fn_140c48bb0 as *const ()), // sub_140c48bb0
    Replacement::draft(0x140c48c40, "unknown_140c48c40", fn_140c48c40 as *const ()), // sub_140c48c40
    Replacement::draft(0x140c48cc0, "unknown_140c48cc0", fn_140c48cc0 as *const ()), // sub_140c48cc0
    Replacement::draft(0x140c48dd0, "unknown_140c48dd0", fn_140c48dd0 as *const ()), // sub_140c48dd0
    Replacement::draft(0x140c48f60, "unknown_140c48f60", tpl_15299::<0x14387f3a0, 0x14243b0b0, 0x140ca2b70, 0xd8, 0x1b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c48f60
    Replacement::draft(0x140c48ff0, "unknown_140c48ff0", fn_140c48ff0 as *const ()), // sub_140c48ff0
    Replacement::draft(0x140c49110, "unknown_140c49110", fn_140c49110 as *const ()), // sub_140c49110
    Replacement::draft_throws(0x140c492b0, "unknown_140c492b0", fn_140c492b0 as *const (), Throws::Discard), // sub_140c492b0
    Replacement::draft_throws(0x140c49330, "unknown_140c49330", fn_140c49330 as *const (), Throws::Discard), // sub_140c49330
    Replacement::draft_throws(0x140c493a0, "unknown_140c493a0", fn_140c493a0 as *const (), Throws::Discard), // sub_140c493a0
    Replacement::draft(0x140c494d0, "unknown_140c494d0", tpl_15316::<0x1432d5cb8, 0x1432d5cd8, 0x14088ba10, 0x1432d5c08, 0x18, 0x1432d5c28> as *const ()), // sub_140c494d0
    Replacement::draft_throws(0x140c49550, "unknown_140c49550", fn_140c49550 as *const (), Throws::Caught), // sub_140c49550
    Replacement::draft_throws(0x140c49960, "unknown_140c49960", fn_140c49960 as *const (), Throws::Caught), // sub_140c49960
    Replacement::draft_throws(0x140c49d70, "unknown_140c49d70", fn_140c49d70 as *const (), Throws::Caught), // sub_140c49d70
    Replacement::draft_throws(0x140c4a240, "unknown_140c4a240", fn_140c4a240 as *const (), Throws::Caught), // sub_140c4a240
    Replacement::draft_throws(0x140c4a6d0, "unknown_140c4a6d0", fn_140c4a6d0 as *const (), Throws::Caught), // sub_140c4a6d0
    Replacement::draft_throws(0x140c4ab60, "unknown_140c4ab60", fn_140c4ab60 as *const (), Throws::Caught), // sub_140c4ab60
    Replacement::draft_throws(0x140c4b030, "unknown_140c4b030", fn_140c4b030 as *const (), Throws::Caught), // sub_140c4b030
    Replacement::draft_throws(0x140c4b4c0, "unknown_140c4b4c0", fn_140c4b4c0 as *const (), Throws::Caught), // sub_140c4b4c0
    Replacement::draft_throws(0x140c4b950, "unknown_140c4b950", fn_140c4b950 as *const (), Throws::Caught), // sub_140c4b950
    Replacement::draft_throws(0x140c4be20, "unknown_140c4be20", fn_140c4be20 as *const (), Throws::Caught), // sub_140c4be20
    Replacement::draft_throws(0x140c4c2b0, "unknown_140c4c2b0", fn_140c4c2b0 as *const (), Throws::Caught), // sub_140c4c2b0
    Replacement::draft_throws(0x140c4c740, "unknown_140c4c740", fn_140c4c740 as *const (), Throws::Caught), // sub_140c4c740
    Replacement::draft_throws(0x140c4cc10, "unknown_140c4cc10", fn_140c4cc10 as *const (), Throws::Caught), // sub_140c4cc10
    Replacement::draft_throws(0x140c4d0a0, "unknown_140c4d0a0", fn_140c4d0a0 as *const (), Throws::Caught), // sub_140c4d0a0
    Replacement::draft_throws(0x140c4d530, "unknown_140c4d530", fn_140c4d530 as *const (), Throws::Caught), // sub_140c4d530
    Replacement::draft_throws(0x140c4da00, "unknown_140c4da00", fn_140c4da00 as *const (), Throws::Caught), // sub_140c4da00
    Replacement::draft_throws(0x140c4de90, "unknown_140c4de90", fn_140c4de90 as *const (), Throws::Caught), // sub_140c4de90
    Replacement::draft_throws(0x140c4e320, "unknown_140c4e320", fn_140c4e320 as *const (), Throws::Caught), // sub_140c4e320
    Replacement::draft_throws(0x140c4e7f0, "unknown_140c4e7f0", fn_140c4e7f0 as *const (), Throws::Caught), // sub_140c4e7f0
    Replacement::draft_throws(0x140c4ec80, "unknown_140c4ec80", fn_140c4ec80 as *const (), Throws::Caught), // sub_140c4ec80
    Replacement::draft_throws(0x140c4f110, "unknown_140c4f110", fn_140c4f110 as *const (), Throws::Caught), // sub_140c4f110
    Replacement::draft_throws(0x140c4f5e0, "unknown_140c4f5e0", fn_140c4f5e0 as *const (), Throws::Caught), // sub_140c4f5e0
    Replacement::draft_throws(0x140c4fa70, "unknown_140c4fa70", fn_140c4fa70 as *const (), Throws::Caught), // sub_140c4fa70
    Replacement::draft_throws(0x140c4ff00, "unknown_140c4ff00", fn_140c4ff00 as *const (), Throws::Caught), // sub_140c4ff00
    Replacement::draft_throws(0x140c503d0, "unknown_140c503d0", fn_140c503d0 as *const (), Throws::Caught), // sub_140c503d0
    Replacement::draft_throws(0x140c50860, "unknown_140c50860", fn_140c50860 as *const (), Throws::Caught), // sub_140c50860
    Replacement::draft_throws(0x140c50cf0, "unknown_140c50cf0", fn_140c50cf0 as *const (), Throws::Caught), // sub_140c50cf0
    Replacement::draft_throws(0x140c511c0, "unknown_140c511c0", fn_140c511c0 as *const (), Throws::Caught), // sub_140c511c0
    Replacement::draft_throws(0x140c51650, "unknown_140c51650", fn_140c51650 as *const (), Throws::Caught), // sub_140c51650
    Replacement::draft_throws(0x140c51ae0, "unknown_140c51ae0", fn_140c51ae0 as *const (), Throws::Caught), // sub_140c51ae0
    Replacement::draft_throws(0x140c51fb0, "unknown_140c51fb0", fn_140c51fb0 as *const (), Throws::Caught), // sub_140c51fb0
    Replacement::draft_throws(0x140c52430, "unknown_140c52430", fn_140c52430 as *const (), Throws::Caught), // sub_140c52430
    Replacement::draft_throws(0x140c528b0, "unknown_140c528b0", fn_140c528b0 as *const (), Throws::Caught), // sub_140c528b0
    Replacement::draft_throws(0x140c52d80, "unknown_140c52d80", fn_140c52d80 as *const (), Throws::Caught), // sub_140c52d80
    Replacement::draft_throws(0x140c53250, "unknown_140c53250", fn_140c53250 as *const (), Throws::Caught), // sub_140c53250
    Replacement::draft_throws(0x140c536e0, "unknown_140c536e0", fn_140c536e0 as *const (), Throws::Caught), // sub_140c536e0
    Replacement::draft_throws(0x140c53b70, "unknown_140c53b70", fn_140c53b70 as *const (), Throws::Caught), // sub_140c53b70
    Replacement::draft_throws(0x140c54000, "unknown_140c54000", fn_140c54000 as *const (), Throws::Caught), // sub_140c54000
    Replacement::draft_throws(0x140c54490, "unknown_140c54490", fn_140c54490 as *const (), Throws::Caught), // sub_140c54490
    Replacement::draft_throws(0x140c54960, "unknown_140c54960", fn_140c54960 as *const (), Throws::Caught), // sub_140c54960
    Replacement::draft_throws(0x140c54e30, "unknown_140c54e30", fn_140c54e30 as *const (), Throws::Caught), // sub_140c54e30
    Replacement::draft_throws(0x140c55300, "unknown_140c55300", fn_140c55300 as *const (), Throws::Caught), // sub_140c55300
    Replacement::draft_throws(0x140c55790, "unknown_140c55790", fn_140c55790 as *const (), Throws::Caught), // sub_140c55790
    Replacement::draft_throws(0x140c55c20, "unknown_140c55c20", fn_140c55c20 as *const (), Throws::Caught), // sub_140c55c20
    Replacement::draft_throws(0x140c560b0, "unknown_140c560b0", fn_140c560b0 as *const (), Throws::Caught), // sub_140c560b0
    Replacement::draft_throws(0x140c56540, "unknown_140c56540", fn_140c56540 as *const (), Throws::Caught), // sub_140c56540
    Replacement::draft_throws(0x140c569d0, "unknown_140c569d0", fn_140c569d0 as *const (), Throws::Caught), // sub_140c569d0
    Replacement::draft_throws(0x140c56e60, "unknown_140c56e60", fn_140c56e60 as *const (), Throws::Caught), // sub_140c56e60
    Replacement::draft_throws(0x140c57270, "unknown_140c57270", fn_140c57270 as *const (), Throws::Caught), // sub_140c57270
    Replacement::draft_throws(0x140c57680, "unknown_140c57680", fn_140c57680 as *const (), Throws::Caught), // sub_140c57680
    Replacement::draft_throws(0x140c57b50, "unknown_140c57b50", fn_140c57b50 as *const (), Throws::Caught), // sub_140c57b50
    Replacement::draft_throws(0x140c57fe0, "unknown_140c57fe0", fn_140c57fe0 as *const (), Throws::Caught), // sub_140c57fe0
    Replacement::draft(0x140c58470, "unknown_140c58470", fn_140c58470 as *const ()), // sub_140c58470
    Replacement::draft(0x140c58540, "unknown_140c58540", fn_140c58540 as *const ()), // sub_140c58540
    Replacement::draft(0x140c586c0, "unknown_140c586c0", fn_140c586c0 as *const ()), // sub_140c586c0
    Replacement::draft(0x140c58790, "unknown_140c58790", fn_140c58790 as *const ()), // sub_140c58790
    Replacement::draft(0x140c58c80, "unknown_140c58c80", fn_140c58c80 as *const ()), // sub_140c58c80
    Replacement::draft_throws(0x140c58e40, "unknown_140c58e40", fn_140c58e40 as *const (), Throws::Caught), // sub_140c58e40
    Replacement::draft_throws(0x140c58f80, "unknown_140c58f80", fn_140c58f80 as *const (), Throws::Caught), // sub_140c58f80
    Replacement::draft_throws(0x140c590c0, "unknown_140c590c0", fn_140c590c0 as *const (), Throws::Caught), // sub_140c590c0
    Replacement::draft_throws(0x140c59200, "unknown_140c59200", fn_140c59200 as *const (), Throws::Caught), // sub_140c59200
    Replacement::draft_throws(0x140c59340, "unknown_140c59340", fn_140c59340 as *const (), Throws::Caught), // sub_140c59340
    Replacement::draft_throws(0x140c59480, "unknown_140c59480", fn_140c59480 as *const (), Throws::Caught), // sub_140c59480
    Replacement::draft_throws(0x140c595c0, "unknown_140c595c0", fn_140c595c0 as *const (), Throws::Caught), // sub_140c595c0
    Replacement::draft_throws(0x140c597b0, "unknown_140c597b0", fn_140c597b0 as *const (), Throws::Caught), // sub_140c597b0
    Replacement::draft(0x140c599a0, "unknown_140c599a0", fn_140c599a0 as *const ()), // sub_140c599a0
    Replacement::draft(0x140c59a70, "unknown_140c59a70", fn_140c59a70 as *const ()), // sub_140c59a70
    Replacement::draft_throws(0x140c59c00, "unknown_140c59c00", fn_140c59c00 as *const (), Throws::Caught), // sub_140c59c00
    Replacement::draft_throws(0x140c59ed0, "unknown_140c59ed0", fn_140c59ed0 as *const (), Throws::Caught), // sub_140c59ed0
    Replacement::draft_throws(0x140c5a1a0, "unknown_140c5a1a0", fn_140c5a1a0 as *const (), Throws::Caught), // sub_140c5a1a0
    Replacement::draft_throws(0x140c5a470, "unknown_140c5a470", fn_140c5a470 as *const (), Throws::Caught), // sub_140c5a470
    Replacement::draft(0x140c5a660, "unknown_140c5a660", fn_140c5a660 as *const ()), // sub_140c5a660
    Replacement::draft_throws(0x140c5a930, "unknown_140c5a930", fn_140c5a930 as *const (), Throws::Caught), // sub_140c5a930
    Replacement::draft_throws(0x140c5aa70, "unknown_140c5aa70", fn_140c5aa70 as *const (), Throws::Caught), // sub_140c5aa70
    Replacement::draft_throws(0x140c5abb0, "unknown_140c5abb0", fn_140c5abb0 as *const (), Throws::Caught), // sub_140c5abb0
    Replacement::draft_throws(0x140c5acf0, "unknown_140c5acf0", fn_140c5acf0 as *const (), Throws::Caught), // sub_140c5acf0
    Replacement::draft(0x140c5aee0, "unknown_140c5aee0", fn_140c5aee0 as *const ()), // sub_140c5aee0
    Replacement::draft_throws(0x140c5b090, "unknown_140c5b090", fn_140c5b090 as *const (), Throws::Caught), // sub_140c5b090
    Replacement::draft_throws(0x140c5b1d0, "unknown_140c5b1d0", fn_140c5b1d0 as *const (), Throws::Caught), // sub_140c5b1d0
    Replacement::draft_throws(0x140c5b310, "unknown_140c5b310", fn_140c5b310 as *const (), Throws::Caught), // sub_140c5b310
    Replacement::draft_throws(0x140c5b450, "unknown_140c5b450", fn_140c5b450 as *const (), Throws::Caught), // sub_140c5b450
    Replacement::draft(0x140c5b640, "unknown_140c5b640", fn_140c5b640 as *const ()), // sub_140c5b640
    Replacement::draft(0x140c5b6d0, "unknown_140c5b6d0", fn_140c5b6d0 as *const ()), // sub_140c5b6d0
    Replacement::draft_throws(0x140c5b800, "unknown_140c5b800", fn_140c5b800 as *const (), Throws::Caught), // sub_140c5b800
    Replacement::draft_throws(0x140c5bab0, "unknown_140c5bab0", fn_140c5bab0 as *const (), Throws::Caught), // sub_140c5bab0
    Replacement::draft_throws(0x140c5bd60, "unknown_140c5bd60", fn_140c5bd60 as *const (), Throws::Caught), // sub_140c5bd60
    Replacement::draft_throws(0x140c5c010, "unknown_140c5c010", fn_140c5c010 as *const (), Throws::Caught), // sub_140c5c010
    Replacement::draft(0x140c5c200, "unknown_140c5c200", fn_140c5c200 as *const ()), // sub_140c5c200
    Replacement::draft(0x140c5c2d0, "unknown_140c5c2d0", fn_140c5c2d0 as *const ()), // sub_140c5c2d0
    Replacement::draft_throws(0x140c5c4d0, "unknown_140c5c4d0", fn_140c5c4d0 as *const (), Throws::Caught), // sub_140c5c4d0
    Replacement::draft_throws(0x140c5c610, "unknown_140c5c610", fn_140c5c610 as *const (), Throws::Caught), // sub_140c5c610
    Replacement::draft_throws(0x140c5c750, "unknown_140c5c750", fn_140c5c750 as *const (), Throws::Caught), // sub_140c5c750
    Replacement::draft_throws(0x140c5c890, "unknown_140c5c890", fn_140c5c890 as *const (), Throws::Caught), // sub_140c5c890
    Replacement::draft(0x140c5ca80, "unknown_140c5ca80", fn_140c5ca80 as *const ()), // sub_140c5ca80
    Replacement::draft(0x140c5cb90, "unknown_140c5cb90", fn_140c5cb90 as *const ()), // sub_140c5cb90
    Replacement::draft(0x140c5ccd0, "unknown_140c5ccd0", tpl_15275::<0x14387f278, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c5ccd0
    Replacement::draft_throws(0x140c5cd70, "unknown_140c5cd70", fn_140c5cd70 as *const (), Throws::Caught), // sub_140c5cd70
    Replacement::draft_throws(0x140c5cfc0, "unknown_140c5cfc0", fn_140c5cfc0 as *const (), Throws::Caught), // sub_140c5cfc0
    Replacement::draft_throws(0x140c5d210, "unknown_140c5d210", fn_140c5d210 as *const (), Throws::Caught), // sub_140c5d210
    Replacement::draft_throws(0x140c5d460, "unknown_140c5d460", fn_140c5d460 as *const (), Throws::Caught), // sub_140c5d460
    Replacement::draft(0x140c5d700, "unknown_140c5d700", fn_140c5d700 as *const ()), // sub_140c5d700
    Replacement::draft(0x140c5d7c0, "unknown_140c5d7c0", fn_140c5d7c0 as *const ()), // sub_140c5d7c0
    Replacement::draft_throws(0x140c5d930, "unknown_140c5d930", fn_140c5d930 as *const (), Throws::Caught), // sub_140c5d930
    Replacement::draft_throws(0x140c5da70, "unknown_140c5da70", fn_140c5da70 as *const (), Throws::Caught), // sub_140c5da70
    Replacement::draft_throws(0x140c5dbb0, "unknown_140c5dbb0", fn_140c5dbb0 as *const (), Throws::Caught), // sub_140c5dbb0
    Replacement::draft_throws(0x140c5dcf0, "unknown_140c5dcf0", fn_140c5dcf0 as *const (), Throws::Caught), // sub_140c5dcf0
    Replacement::draft_throws(0x140c5dee0, "unknown_140c5dee0", fn_140c5dee0 as *const (), Throws::Caught), // sub_140c5dee0
    Replacement::draft_throws(0x140c5e1b0, "unknown_140c5e1b0", fn_140c5e1b0 as *const (), Throws::Caught), // sub_140c5e1b0
    Replacement::draft_throws(0x140c5e450, "unknown_140c5e450", fn_140c5e450 as *const (), Throws::Caught), // sub_140c5e450
    Replacement::draft_throws(0x140c5e6f0, "unknown_140c5e6f0", fn_140c5e6f0 as *const (), Throws::Discard), // sub_140c5e6f0
    Replacement::draft(0x140c5e880, "unknown_140c5e880", fn_140c5e880 as *const ()), // sub_140c5e880
    Replacement::draft(0x140c5ea00, "unknown_140c5ea00", fn_140c5ea00 as *const ()), // sub_140c5ea00
    Replacement::draft(0x140c5edc0, "unknown_140c5edc0", fn_140c5edc0 as *const ()), // sub_140c5edc0
    Replacement::draft_throws(0x140c5efc0, "unknown_140c5efc0", fn_140c5efc0 as *const (), Throws::Caught), // sub_140c5efc0
    Replacement::draft_throws(0x140c5f100, "unknown_140c5f100", fn_140c5f100 as *const (), Throws::Caught), // sub_140c5f100
    Replacement::draft_throws(0x140c5f240, "unknown_140c5f240", fn_140c5f240 as *const (), Throws::Caught), // sub_140c5f240
    Replacement::draft_throws(0x140c5f380, "unknown_140c5f380", fn_140c5f380 as *const (), Throws::Caught), // sub_140c5f380
    Replacement::draft_throws(0x140c5f4c0, "unknown_140c5f4c0", fn_140c5f4c0 as *const (), Throws::Caught), // sub_140c5f4c0
    Replacement::draft_throws(0x140c5f600, "unknown_140c5f600", fn_140c5f600 as *const (), Throws::Caught), // sub_140c5f600
    Replacement::draft_throws(0x140c5f740, "unknown_140c5f740", fn_140c5f740 as *const (), Throws::Caught), // sub_140c5f740
    Replacement::draft_throws(0x140c5f930, "unknown_140c5f930", fn_140c5f930 as *const (), Throws::Caught), // sub_140c5f930
    Replacement::draft(0x140c5fb20, "unknown_140c5fb20", fn_140c5fb20 as *const ()), // sub_140c5fb20
    Replacement::draft(0x140c5fc20, "unknown_140c5fc20", fn_140c5fc20 as *const ()), // sub_140c5fc20
    Replacement::draft(0x140c5fd20, "unknown_140c5fd20", fn_140c5fd20 as *const ()), // sub_140c5fd20
    Replacement::draft(0x140c5fdb0, "unknown_140c5fdb0", fn_140c5fdb0 as *const ()), // sub_140c5fdb0
    Replacement::draft_throws(0x140c5ff50, "unknown_140c5ff50", fn_140c5ff50 as *const (), Throws::Caught), // sub_140c5ff50
    Replacement::draft_throws(0x140c601c0, "unknown_140c601c0", fn_140c601c0 as *const (), Throws::Caught), // sub_140c601c0
    Replacement::draft_throws(0x140c60430, "unknown_140c60430", fn_140c60430 as *const (), Throws::Caught), // sub_140c60430
    Replacement::draft_throws(0x140c606a0, "unknown_140c606a0", fn_140c606a0 as *const (), Throws::Caught), // sub_140c606a0
    Replacement::draft(0x140c60890, "unknown_140c60890", tpl_15354::<0x14384ea30, 0x1432d81e0, 0x1432d8060, 0x18, 0x1432d7f08, 0x1401de410, 0x8, 0x1426cdcb0, 0x140ca0a00> as *const ()), // sub_140c60890
    Replacement::draft(0x140c60940, "unknown_140c60940", tpl_15354::<0x14384ea30, 0x1432d81e0, 0x1432d8060, 0x18, 0x1432d7f08, 0x1401de410, 0x8, 0x1432d7b00, 0x140ca0a10> as *const ()), // sub_140c60940
    Replacement::draft_throws(0x140c609f0, "unknown_140c609f0", fn_140c609f0 as *const (), Throws::Caught), // sub_140c609f0
    Replacement::draft_throws(0x140c60b40, "unknown_140c60b40", fn_140c60b40 as *const (), Throws::Caught), // sub_140c60b40
    Replacement::draft_throws(0x140c60c90, "unknown_140c60c90", fn_140c60c90 as *const (), Throws::Caught), // sub_140c60c90
    Replacement::draft_throws(0x140c60de0, "unknown_140c60de0", fn_140c60de0 as *const (), Throws::Discard), // sub_140c60de0
    Replacement::draft_throws(0x140c60f20, "unknown_140c60f20", fn_140c60f20 as *const (), Throws::Caught), // sub_140c60f20
    Replacement::draft_throws(0x140c61170, "unknown_140c61170", fn_140c61170 as *const (), Throws::Caught), // sub_140c61170
    Replacement::draft_throws(0x140c613c0, "unknown_140c613c0", fn_140c613c0 as *const (), Throws::Caught), // sub_140c613c0
    Replacement::draft_throws(0x140c61610, "unknown_140c61610", fn_140c61610 as *const (), Throws::Caught), // sub_140c61610
    Replacement::draft(0x140c618d0, "unknown_140c618d0", fn_140c618d0 as *const ()), // sub_140c618d0
    Replacement::draft_throws(0x140c619b0, "unknown_140c619b0", fn_140c619b0 as *const (), Throws::Caught), // sub_140c619b0
    Replacement::draft_throws(0x140c61af0, "unknown_140c61af0", fn_140c61af0 as *const (), Throws::Caught), // sub_140c61af0
    Replacement::draft_throws(0x140c61c30, "unknown_140c61c30", fn_140c61c30 as *const (), Throws::Caught), // sub_140c61c30
    Replacement::draft_throws(0x140c61d70, "unknown_140c61d70", fn_140c61d70 as *const (), Throws::Caught), // sub_140c61d70
    Replacement::draft_throws(0x140c61f60, "unknown_140c61f60", fn_140c61f60 as *const (), Throws::Caught), // sub_140c61f60
    Replacement::draft_throws(0x140c62060, "unknown_140c62060", fn_140c62060 as *const (), Throws::Caught), // sub_140c62060
    Replacement::draft_throws(0x140c62160, "unknown_140c62160", fn_140c62160 as *const (), Throws::Caught), // sub_140c62160
    Replacement::draft_throws(0x140c62260, "unknown_140c62260", tpl_15361::<0x14387f460, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143426dd4, 0x143426c70, 0x1432d6030, 0x14325edbc, 0x1432d6288, 0x14020f240> as *const (), Throws::Discard), // sub_140c62260
    Replacement::draft(0x140c62350, "unknown_140c62350", fn_140c62350 as *const ()), // sub_140c62350
    Replacement::draft(0x140c623e0, "unknown_140c623e0", fn_140c623e0 as *const ()), // sub_140c623e0
    Replacement::draft_throws(0x140c625a0, "unknown_140c625a0", fn_140c625a0 as *const (), Throws::Caught), // sub_140c625a0
    Replacement::draft_throws(0x140c62840, "unknown_140c62840", fn_140c62840 as *const (), Throws::Caught), // sub_140c62840
    Replacement::draft_throws(0x140c62ad0, "unknown_140c62ad0", fn_140c62ad0 as *const (), Throws::Caught), // sub_140c62ad0
    Replacement::draft_throws(0x140c62d60, "unknown_140c62d60", fn_140c62d60 as *const (), Throws::Caught), // sub_140c62d60
    Replacement::draft_throws(0x140c62f50, "unknown_140c62f50", fn_140c62f50 as *const (), Throws::Caught), // sub_140c62f50
    Replacement::draft_throws(0x140c63200, "unknown_140c63200", fn_140c63200 as *const (), Throws::Caught), // sub_140c63200
    Replacement::draft_throws(0x140c63480, "unknown_140c63480", fn_140c63480 as *const (), Throws::Caught), // sub_140c63480
    Replacement::draft_throws(0x140c63700, "unknown_140c63700", fn_140c63700 as *const (), Throws::Discard), // sub_140c63700
    Replacement::draft(0x140c63870, "unknown_140c63870", fn_140c63870 as *const ()), // sub_140c63870
    Replacement::draft(0x140c638e0, "unknown_140c638e0", fn_140c638e0 as *const ()), // sub_140c638e0
    Replacement::draft_throws(0x140c639d0, "unknown_140c639d0", fn_140c639d0 as *const (), Throws::Caught), // sub_140c639d0
    Replacement::draft_throws(0x140c63b10, "unknown_140c63b10", fn_140c63b10 as *const (), Throws::Caught), // sub_140c63b10
    Replacement::draft_throws(0x140c63c50, "unknown_140c63c50", fn_140c63c50 as *const (), Throws::Caught), // sub_140c63c50
    Replacement::draft_throws(0x140c63d90, "unknown_140c63d90", fn_140c63d90 as *const (), Throws::Caught), // sub_140c63d90
    Replacement::draft(0x140c63f80, "unknown_140c63f80", fn_140c63f80 as *const ()), // sub_140c63f80
    Replacement::draft_throws(0x140c640e0, "unknown_140c640e0", fn_140c640e0 as *const (), Throws::Caught), // sub_140c640e0
    Replacement::draft_throws(0x140c64340, "unknown_140c64340", fn_140c64340 as *const (), Throws::Caught), // sub_140c64340
    Replacement::draft_throws(0x140c645a0, "unknown_140c645a0", fn_140c645a0 as *const (), Throws::Caught), // sub_140c645a0
    Replacement::draft_throws(0x140c64800, "unknown_140c64800", fn_140c64800 as *const (), Throws::Caught), // sub_140c64800
    Replacement::draft_throws(0x140c649f0, "unknown_140c649f0", fn_140c649f0 as *const (), Throws::Caught), // sub_140c649f0
    Replacement::draft_throws(0x140c64d10, "unknown_140c64d10", fn_140c64d10 as *const (), Throws::Caught), // sub_140c64d10
    Replacement::draft_throws(0x140c65000, "unknown_140c65000", fn_140c65000 as *const (), Throws::Caught), // sub_140c65000
    Replacement::draft_throws(0x140c652e0, "unknown_140c652e0", fn_140c652e0 as *const (), Throws::Discard), // sub_140c652e0
    Replacement::draft(0x140c65490, "unknown_140c65490", fn_140c65490 as *const ()), // sub_140c65490
    Replacement::draft_throws(0x140c65550, "unknown_140c65550", fn_140c65550 as *const (), Throws::Caught), // sub_140c65550
    Replacement::draft_throws(0x140c657c0, "unknown_140c657c0", fn_140c657c0 as *const (), Throws::Caught), // sub_140c657c0
    Replacement::draft_throws(0x140c65a30, "unknown_140c65a30", fn_140c65a30 as *const (), Throws::Caught), // sub_140c65a30
    Replacement::draft_throws(0x140c65b40, "unknown_140c65b40", fn_140c65b40 as *const (), Throws::Caught), // sub_140c65b40
    Replacement::draft_throws(0x140c65c00, "unknown_140c65c00", fn_140c65c00 as *const (), Throws::Caught), // sub_140c65c00
    Replacement::draft_throws(0x140c65cb0, "unknown_140c65cb0", fn_140c65cb0 as *const (), Throws::Caught), // sub_140c65cb0
    Replacement::draft_throws(0x140c65dc0, "unknown_140c65dc0", fn_140c65dc0 as *const (), Throws::Caught), // sub_140c65dc0
    Replacement::draft_throws(0x140c65e80, "unknown_140c65e80", fn_140c65e80 as *const (), Throws::Caught), // sub_140c65e80
    Replacement::draft_throws(0x140c65f30, "unknown_140c65f30", fn_140c65f30 as *const (), Throws::Caught), // sub_140c65f30
    Replacement::draft_throws(0x140c66040, "unknown_140c66040", fn_140c66040 as *const (), Throws::Caught), // sub_140c66040
    Replacement::draft_throws(0x140c66100, "unknown_140c66100", fn_140c66100 as *const (), Throws::Caught), // sub_140c66100
    Replacement::draft_throws(0x140c661b0, "unknown_140c661b0", fn_140c661b0 as *const (), Throws::Caught), // sub_140c661b0
    Replacement::draft_throws(0x140c66310, "unknown_140c66310", fn_140c66310 as *const (), Throws::Caught), // sub_140c66310
    Replacement::draft_throws(0x140c664c0, "unknown_140c664c0", fn_140c664c0 as *const (), Throws::Caught), // sub_140c664c0
    Replacement::draft(0x140c665d0, "unknown_140c665d0", fn_140c665d0 as *const ()), // sub_140c665d0
    Replacement::draft(0x140c66690, "unknown_140c66690", fn_140c66690 as *const ()), // sub_140c66690
    Replacement::draft_throws(0x140c66840, "unknown_140c66840", fn_140c66840 as *const (), Throws::Caught), // sub_140c66840
    Replacement::draft_throws(0x140c66c50, "unknown_140c66c50", fn_140c66c50 as *const (), Throws::Caught), // sub_140c66c50
    Replacement::draft_throws(0x140c67050, "unknown_140c67050", fn_140c67050 as *const (), Throws::Caught), // sub_140c67050
    Replacement::draft_throws(0x140c67450, "unknown_140c67450", fn_140c67450 as *const (), Throws::Caught), // sub_140c67450
    Replacement::draft(0x140c67640, "unknown_140c67640", fn_140c67640 as *const ()), // sub_140c67640
    Replacement::draft_throws(0x140c67740, "unknown_140c67740", fn_140c67740 as *const (), Throws::Caught), // sub_140c67740
    Replacement::draft_throws(0x140c67880, "unknown_140c67880", fn_140c67880 as *const (), Throws::Caught), // sub_140c67880
    Replacement::draft_throws(0x140c679c0, "unknown_140c679c0", fn_140c679c0 as *const (), Throws::Caught), // sub_140c679c0
    Replacement::draft_throws(0x140c67b00, "unknown_140c67b00", fn_140c67b00 as *const (), Throws::Caught), // sub_140c67b00
    Replacement::draft(0x140c67cf0, "unknown_140c67cf0", fn_140c67cf0 as *const ()), // sub_140c67cf0
    Replacement::draft_throws(0x140c67f80, "unknown_140c67f80", fn_140c67f80 as *const (), Throws::Caught), // sub_140c67f80
    Replacement::draft_throws(0x140c680c0, "unknown_140c680c0", fn_140c680c0 as *const (), Throws::Caught), // sub_140c680c0
    Replacement::draft_throws(0x140c68200, "unknown_140c68200", fn_140c68200 as *const (), Throws::Caught), // sub_140c68200
    Replacement::draft_throws(0x140c68340, "unknown_140c68340", fn_140c68340 as *const (), Throws::Caught), // sub_140c68340
    Replacement::draft(0x140c68530, "unknown_140c68530", fn_140c68530 as *const ()), // sub_140c68530
    Replacement::draft(0x140c685d0, "unknown_140c685d0", fn_140c685d0 as *const ()), // sub_140c685d0
    Replacement::draft_throws(0x140c68720, "unknown_140c68720", fn_140c68720 as *const (), Throws::Caught), // sub_140c68720
    Replacement::draft_throws(0x140c68860, "unknown_140c68860", fn_140c68860 as *const (), Throws::Caught), // sub_140c68860
    Replacement::draft_throws(0x140c689a0, "unknown_140c689a0", fn_140c689a0 as *const (), Throws::Caught), // sub_140c689a0
    Replacement::draft_throws(0x140c68ae0, "unknown_140c68ae0", fn_140c68ae0 as *const (), Throws::Caught), // sub_140c68ae0
    Replacement::draft(0x140c68cd0, "unknown_140c68cd0", fn_140c68cd0 as *const ()), // sub_140c68cd0
    Replacement::draft(0x140c68ed0, "unknown_140c68ed0", fn_140c68ed0 as *const ()), // sub_140c68ed0
    Replacement::draft(0x140c68fb0, "unknown_140c68fb0", fn_140c68fb0 as *const ()), // sub_140c68fb0
    Replacement::draft_throws(0x140c690c0, "unknown_140c690c0", fn_140c690c0 as *const (), Throws::Caught), // sub_140c690c0
    Replacement::draft_throws(0x140c69200, "unknown_140c69200", fn_140c69200 as *const (), Throws::Caught), // sub_140c69200
    Replacement::draft_throws(0x140c69340, "unknown_140c69340", fn_140c69340 as *const (), Throws::Caught), // sub_140c69340
    Replacement::draft_throws(0x140c69480, "unknown_140c69480", fn_140c69480 as *const (), Throws::Caught), // sub_140c69480
    Replacement::draft(0x140c69670, "unknown_140c69670", tpl_2365::<0x14387f218, 0x14243b0b0, 0x8, 0x1407de100, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c69670
    Replacement::draft(0x140c696e0, "unknown_140c696e0", fn_140c696e0 as *const ()), // sub_140c696e0
    Replacement::draft_throws(0x140c69790, "unknown_140c69790", fn_140c69790 as *const (), Throws::Caught), // sub_140c69790
    Replacement::draft_throws(0x140c699f0, "unknown_140c699f0", fn_140c699f0 as *const (), Throws::Caught), // sub_140c699f0
    Replacement::draft_throws(0x140c69c50, "unknown_140c69c50", fn_140c69c50 as *const (), Throws::Caught), // sub_140c69c50
    Replacement::draft_throws(0x140c69eb0, "unknown_140c69eb0", fn_140c69eb0 as *const (), Throws::Caught), // sub_140c69eb0
    Replacement::draft(0x140c6a0a0, "unknown_140c6a0a0", fn_140c6a0a0 as *const ()), // sub_140c6a0a0
    Replacement::draft_throws(0x140c6a1e0, "unknown_140c6a1e0", fn_140c6a1e0 as *const (), Throws::Caught), // sub_140c6a1e0
    Replacement::draft_throws(0x140c6a450, "unknown_140c6a450", fn_140c6a450 as *const (), Throws::Caught), // sub_140c6a450
    Replacement::draft_throws(0x140c6a6c0, "unknown_140c6a6c0", fn_140c6a6c0 as *const (), Throws::Caught), // sub_140c6a6c0
    Replacement::draft_throws(0x140c6a930, "unknown_140c6a930", fn_140c6a930 as *const (), Throws::Caught), // sub_140c6a930
    Replacement::draft(0x140c6ab20, "unknown_140c6ab20", fn_140c6ab20 as *const ()), // sub_140c6ab20
    Replacement::draft_throws(0x140c6abd0, "unknown_140c6abd0", fn_140c6abd0 as *const (), Throws::Caught), // sub_140c6abd0
    Replacement::draft_throws(0x140c6ae50, "unknown_140c6ae50", fn_140c6ae50 as *const (), Throws::Caught), // sub_140c6ae50
    Replacement::draft_throws(0x140c6b0d0, "unknown_140c6b0d0", fn_140c6b0d0 as *const (), Throws::Caught), // sub_140c6b0d0
    Replacement::draft_throws(0x140c6b350, "unknown_140c6b350", fn_140c6b350 as *const (), Throws::Caught), // sub_140c6b350
    Replacement::draft(0x140c6b540, "unknown_140c6b540", fn_140c6b540 as *const ()), // sub_140c6b540
    Replacement::draft(0x140c6b5b0, "unknown_140c6b5b0", fn_140c6b5b0 as *const ()), // sub_140c6b5b0
    Replacement::draft_throws(0x140c6b6e0, "unknown_140c6b6e0", fn_140c6b6e0 as *const (), Throws::Caught), // sub_140c6b6e0
    Replacement::draft_throws(0x140c6b820, "unknown_140c6b820", fn_140c6b820 as *const (), Throws::Caught), // sub_140c6b820
    Replacement::draft_throws(0x140c6b960, "unknown_140c6b960", fn_140c6b960 as *const (), Throws::Caught), // sub_140c6b960
    Replacement::draft_throws(0x140c6baa0, "unknown_140c6baa0", fn_140c6baa0 as *const (), Throws::Caught), // sub_140c6baa0
    Replacement::draft(0x140c6bc90, "unknown_140c6bc90", fn_140c6bc90 as *const ()), // sub_140c6bc90
    Replacement::draft_throws(0x140c6bd70, "unknown_140c6bd70", fn_140c6bd70 as *const (), Throws::Caught), // sub_140c6bd70
    Replacement::draft_throws(0x140c6beb0, "unknown_140c6beb0", fn_140c6beb0 as *const (), Throws::Caught), // sub_140c6beb0
    Replacement::draft_throws(0x140c6bff0, "unknown_140c6bff0", fn_140c6bff0 as *const (), Throws::Caught), // sub_140c6bff0
    Replacement::draft_throws(0x140c6c130, "unknown_140c6c130", fn_140c6c130 as *const (), Throws::Caught), // sub_140c6c130
    Replacement::draft(0x140c6c390, "unknown_140c6c390", tpl_15408::<0x14387f268, 0x14243b0b0, 0x8, 0x140a034e0, 0x338, 0x20, 0x1426d4580, 0x1, 0x24, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c6c390
    Replacement::draft(0x140c6c470, "unknown_140c6c470", tpl_15408::<0x14387f360, 0x14243b0b0, 0x8, 0x140a034e0, 0x338, 0x20, 0x1426d4580, 0x1, 0x24, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c6c470
    Replacement::draft_throws(0x140c6c550, "unknown_140c6c550", fn_140c6c550 as *const (), Throws::Caught), // sub_140c6c550
    Replacement::draft_throws(0x140c6c7d0, "unknown_140c6c7d0", fn_140c6c7d0 as *const (), Throws::Caught), // sub_140c6c7d0
    Replacement::draft_throws(0x140c6ca50, "unknown_140c6ca50", fn_140c6ca50 as *const (), Throws::Caught), // sub_140c6ca50
    Replacement::draft_throws(0x140c6ccd0, "unknown_140c6ccd0", fn_140c6ccd0 as *const (), Throws::Caught), // sub_140c6ccd0
    Replacement::draft_throws(0x140c6cf50, "unknown_140c6cf50", fn_140c6cf50 as *const (), Throws::Caught), // sub_140c6cf50
    Replacement::draft_throws(0x140c6d1d0, "unknown_140c6d1d0", fn_140c6d1d0 as *const (), Throws::Caught), // sub_140c6d1d0
    Replacement::draft_throws(0x140c6d450, "unknown_140c6d450", fn_140c6d450 as *const (), Throws::Caught), // sub_140c6d450
    Replacement::draft_throws(0x140c6d640, "unknown_140c6d640", fn_140c6d640 as *const (), Throws::Caught), // sub_140c6d640
    Replacement::draft(0x140c6d830, "unknown_140c6d830", fn_140c6d830 as *const ()), // sub_140c6d830
    Replacement::draft(0x140c6d8c0, "unknown_140c6d8c0", fn_140c6d8c0 as *const ()), // sub_140c6d8c0
    Replacement::draft(0x140c6da00, "unknown_140c6da00", fn_140c6da00 as *const ()), // sub_140c6da00
    Replacement::draft(0x140c6db60, "unknown_140c6db60", tpl_13639::<0x14387f2f0, 0x14243b0b0, 0x140c07850, 0x10, 0x140c99340, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c6db60
    Replacement::draft_throws(0x140c6dbe0, "unknown_140c6dbe0", fn_140c6dbe0 as *const (), Throws::Caught), // sub_140c6dbe0
    Replacement::draft_throws(0x140c6dd20, "unknown_140c6dd20", fn_140c6dd20 as *const (), Throws::Caught), // sub_140c6dd20
    Replacement::draft_throws(0x140c6de60, "unknown_140c6de60", fn_140c6de60 as *const (), Throws::Caught), // sub_140c6de60
    Replacement::draft_throws(0x140c6dfa0, "unknown_140c6dfa0", fn_140c6dfa0 as *const (), Throws::Caught), // sub_140c6dfa0
    Replacement::draft_throws(0x140c6e0e0, "unknown_140c6e0e0", fn_140c6e0e0 as *const (), Throws::Caught), // sub_140c6e0e0
    Replacement::draft_throws(0x140c6e220, "unknown_140c6e220", fn_140c6e220 as *const (), Throws::Caught), // sub_140c6e220
    Replacement::draft_throws(0x140c6e360, "unknown_140c6e360", fn_140c6e360 as *const (), Throws::Caught), // sub_140c6e360
    Replacement::draft_throws(0x140c6e4a0, "unknown_140c6e4a0", fn_140c6e4a0 as *const (), Throws::Caught), // sub_140c6e4a0
    Replacement::draft_throws(0x140c6e5e0, "unknown_140c6e5e0", fn_140c6e5e0 as *const (), Throws::Caught), // sub_140c6e5e0
    Replacement::draft_throws(0x140c6e720, "unknown_140c6e720", fn_140c6e720 as *const (), Throws::Caught), // sub_140c6e720
    Replacement::draft_throws(0x140c6e910, "unknown_140c6e910", fn_140c6e910 as *const (), Throws::Caught), // sub_140c6e910
    Replacement::draft_throws(0x140c6eb00, "unknown_140c6eb00", fn_140c6eb00 as *const (), Throws::Caught), // sub_140c6eb00
    Replacement::draft(0x140c6ed80, "unknown_140c6ed80", fn_140c6ed80 as *const ()), // sub_140c6ed80
    Replacement::draft(0x140c6ee40, "unknown_140c6ee40", fn_140c6ee40 as *const ()), // sub_140c6ee40
    Replacement::draft_throws(0x140c6eff0, "unknown_140c6eff0", fn_140c6eff0 as *const (), Throws::Caught), // sub_140c6eff0
    Replacement::draft_throws(0x140c6f400, "unknown_140c6f400", fn_140c6f400 as *const (), Throws::Caught), // sub_140c6f400
    Replacement::draft_throws(0x140c6f800, "unknown_140c6f800", fn_140c6f800 as *const (), Throws::Caught), // sub_140c6f800
    Replacement::draft_throws(0x140c6fc00, "unknown_140c6fc00", fn_140c6fc00 as *const (), Throws::Caught), // sub_140c6fc00
    Replacement::draft_throws(0x140c6fdf0, "unknown_140c6fdf0", fn_140c6fdf0 as *const (), Throws::Discard), // sub_140c6fdf0
    Replacement::draft_throws(0x140c705e0, "unknown_140c705e0", fn_140c705e0 as *const (), Throws::Discard), // sub_140c705e0
    Replacement::draft_throws(0x140c70c30, "unknown_140c70c30", fn_140c70c30 as *const (), Throws::Discard), // sub_140c70c30
    Replacement::draft_throws(0x140c711c0, "unknown_140c711c0", tpl_15419::<0x14387f1c8, 0x14387f298, 0x1401e9fa0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140c711c0
    Replacement::draft_throws(0x140c71350, "unknown_140c71350", fn_140c71350 as *const (), Throws::Discard), // sub_140c71350
    Replacement::draft(0x140c71400, "unknown_140c71400", fn_140c71400 as *const ()), // sub_140c71400
    Replacement::draft(0x140c71810, "unknown_140c71810", fn_140c71810 as *const ()), // sub_140c71810
    Replacement::draft(0x140c71cb0, "unknown_140c71cb0", fn_140c71cb0 as *const ()), // sub_140c71cb0
    Replacement::draft(0x140c720d0, "unknown_140c720d0", fn_140c720d0 as *const ()), // sub_140c720d0
    Replacement::draft(0x140c72270, "unknown_140c72270", fn_140c72270 as *const ()), // sub_140c72270
    Replacement::draft(0x140c722e0, "content_140c722e0", fn_140c722e0 as *const ()), // content::EffectsGameSpawnPool::content::EffectsGameSpawnPool::vfunc_0
    Replacement::draft(0x140c72320, "content_140c72320", fn_140c72320 as *const ()), // sub_140c72320
    Replacement::draft_throws(0x140c72380, "content_140c72380", fn_140c72380 as *const (), Throws::Caught), // sub_140c72380
    Replacement::draft(0x140c72790, "content_140c72790", fn_140c72790 as *const ()), // sub_140c72790
    Replacement::draft_throws(0x140c72ae0, "content_140c72ae0", fn_140c72ae0 as *const (), Throws::Caught), // sub_140c72ae0
    Replacement::draft(0x140c72c60, "content_140c72c60", fn_140c72c60 as *const ()), // sub_140c72c60
    Replacement::draft(0x140c72e00, "content_140c72e00", fn_140c72e00 as *const ()), // sub_140c72e00
    Replacement::draft(0x140c73110, "content_140c73110", fn_140c73110 as *const ()), // sub_140c73110
    Replacement::draft_throws(0x140c73250, "content_140c73250", fn_140c73250 as *const (), Throws::Discard), // sub_140c73250
    Replacement::draft_throws(0x140c73670, "content_140c73670", fn_140c73670 as *const (), Throws::Discard), // sub_140c73670
    Replacement::draft(0x140c737c0, "content_140c737c0", fn_140c737c0 as *const ()), // sub_140c737c0
    Replacement::draft(0x140c73970, "content_140c73970", fn_140c73970 as *const ()), // sub_140c73970
    Replacement::draft_throws(0x140c73a40, "content_140c73a40", fn_140c73a40 as *const (), Throws::Caught), // sub_140c73a40
    Replacement::draft_throws(0x140c73eb0, "content_140c73eb0", fn_140c73eb0 as *const (), Throws::Caught), // sub_140c73eb0
    Replacement::draft_throws(0x140c74320, "content_140c74320", fn_140c74320 as *const (), Throws::Caught), // sub_140c74320
    Replacement::draft_throws(0x140c74750, "content_140c74750", fn_140c74750 as *const (), Throws::Caught), // sub_140c74750
    Replacement::draft_throws(0x140c74b80, "content_140c74b80", fn_140c74b80 as *const (), Throws::Caught), // sub_140c74b80
    Replacement::draft_throws(0x140c74ed0, "content_140c74ed0", fn_140c74ed0 as *const (), Throws::Caught), // sub_140c74ed0
    Replacement::draft(0x140c75220, "content_140c75220", fn_140c75220 as *const ()), // sub_140c75220
    Replacement::draft(0x140c75510, "content_140c75510", fn_140c75510 as *const ()), // sub_140c75510
    Replacement::draft(0x140c75620, "content_140c75620", fn_140c75620 as *const ()), // sub_140c75620
    Replacement::draft(0x140c75920, "content_140c75920", fn_140c75920 as *const ()), // sub_140c75920
    Replacement::draft_throws(0x140c75a40, "content_140c75a40", fn_140c75a40 as *const (), Throws::Discard), // sub_140c75a40
    Replacement::draft_throws(0x140c75da0, "content_140c75da0", fn_140c75da0 as *const (), Throws::Discard), // sub_140c75da0
    Replacement::draft_throws(0x140c75ed0, "content_140c75ed0", fn_140c75ed0 as *const (), Throws::Discard), // sub_140c75ed0
    Replacement::draft(0x140c76080, "content_140c76080", fn_140c76080 as *const ()), // sub_140c76080
    Replacement::draft(0x140c76190, "content_140c76190", fn_140c76190 as *const ()), // sub_140c76190
    Replacement::draft_throws(0x140c762e0, "content_140c762e0", fn_140c762e0 as *const (), Throws::Discard), // sub_140c762e0
    Replacement::draft_throws(0x140c76480, "content_140c76480", fn_140c76480 as *const (), Throws::Caught), // sub_140c76480
    Replacement::draft_throws(0x140c76a90, "content_140c76a90", fn_140c76a90 as *const (), Throws::Caught), // sub_140c76a90
    Replacement::draft_throws(0x140c77070, "content_140c77070", fn_140c77070 as *const (), Throws::Caught), // sub_140c77070
    Replacement::draft(0x140c773c0, "content_140c773c0", fn_140c773c0 as *const ()), // sub_140c773c0
    Replacement::draft(0x140c77520, "content_140c77520", fn_140c77520 as *const ()), // sub_140c77520
    Replacement::draft_throws(0x140c77680, "content_140c77680", fn_140c77680 as *const (), Throws::Discard), // sub_140c77680
    Replacement::draft_throws(0x140c777b0, "content_140c777b0", fn_140c777b0 as *const (), Throws::Caught), // sub_140c777b0
    Replacement::draft(0x140c77a10, "content_140c77a10", fn_140c77a10 as *const ()), // sub_140c77a10
    Replacement::draft(0x140c77b70, "content_140c77b70", fn_140c77b70 as *const ()), // sub_140c77b70
    Replacement::draft_throws(0x140c77ce0, "content_140c77ce0", fn_140c77ce0 as *const (), Throws::Discard), // sub_140c77ce0
    Replacement::draft(0x140c77f80, "content_140c77f80", fn_140c77f80 as *const ()), // sub_140c77f80
    Replacement::draft_throws(0x140c78050, "content_140c78050", fn_140c78050 as *const (), Throws::Caught), // sub_140c78050
    Replacement::draft_throws(0x140c784c0, "content_140c784c0", fn_140c784c0 as *const (), Throws::Caught), // sub_140c784c0
    Replacement::draft_throws(0x140c788f0, "content_140c788f0", fn_140c788f0 as *const (), Throws::Caught), // sub_140c788f0
    Replacement::draft(0x140c78c40, "content_140c78c40", fn_140c78c40 as *const ()), // sub_140c78c40
    Replacement::draft(0x140c78db0, "content_140c78db0", fn_140c78db0 as *const ()), // sub_140c78db0
    Replacement::draft_throws(0x140c78f20, "content_140c78f20", fn_140c78f20 as *const (), Throws::Discard), // sub_140c78f20
    Replacement::draft_throws(0x140c790f0, "content_140c790f0", fn_140c790f0 as *const (), Throws::Discard), // sub_140c790f0
    Replacement::draft(0x140c79290, "content_140c79290", fn_140c79290 as *const ()), // sub_140c79290
    Replacement::draft(0x140c79390, "content_140c79390", tpl_15458::<0x14387f3c0, 0x14243b0b0, 0x338, 0x198, 0x20, 0x88, 0x110, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c79390
    Replacement::draft_throws(0x140c79440, "content_140c79440", fn_140c79440 as *const (), Throws::Discard), // sub_140c79440
    Replacement::draft(0x140c795d0, "content_140c795d0", fn_140c795d0 as *const ()), // sub_140c795d0
    Replacement::draft_throws(0x140c79650, "content_140c79650", fn_140c79650 as *const (), Throws::Caught), // sub_140c79650
    Replacement::draft_throws(0x140c79ac0, "content_140c79ac0", fn_140c79ac0 as *const (), Throws::Caught), // sub_140c79ac0
    Replacement::draft_throws(0x140c79ef0, "content_140c79ef0", fn_140c79ef0 as *const (), Throws::Caught), // sub_140c79ef0
    Replacement::draft(0x140c7a240, "content_140c7a240", fn_140c7a240 as *const ()), // sub_140c7a240
    Replacement::draft(0x140c7a2f0, "content_140c7a2f0", tpl_15458::<0x14387f3c0, 0x14243b0b0, 0x338, 0x198, 0xc0, 0x88, 0x110, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c7a2f0
    Replacement::draft_throws(0x140c7a3b0, "content_140c7a3b0", fn_140c7a3b0 as *const (), Throws::Discard), // sub_140c7a3b0
    Replacement::draft_throws(0x140c7a450, "content_140c7a450", fn_140c7a450 as *const (), Throws::Discard), // sub_140c7a450
    Replacement::draft(0x140c7a590, "content_140c7a590", fn_140c7a590 as *const ()), // sub_140c7a590
    Replacement::draft(0x140c7a660, "content_140c7a660", fn_140c7a660 as *const ()), // sub_140c7a660
    Replacement::draft_throws(0x140c7a720, "content_140c7a720", fn_140c7a720 as *const (), Throws::Discard), // sub_140c7a720
    Replacement::draft_throws(0x140c7a870, "content_140c7a870", fn_140c7a870 as *const (), Throws::Caught), // sub_140c7a870
    Replacement::draft_throws(0x140c7ae80, "content_140c7ae80", fn_140c7ae80 as *const (), Throws::Caught), // sub_140c7ae80
    Replacement::draft_throws(0x140c7b440, "content_140c7b440", fn_140c7b440 as *const (), Throws::Caught), // sub_140c7b440
    Replacement::draft(0x140c7b790, "content_140c7b790", fn_140c7b790 as *const ()), // sub_140c7b790
    Replacement::draft(0x140c7b8c0, "content_140c7b8c0", fn_140c7b8c0 as *const ()), // sub_140c7b8c0
    Replacement::draft_throws(0x140c7ba00, "content_140c7ba00", fn_140c7ba00 as *const (), Throws::Discard), // sub_140c7ba00
    Replacement::draft_throws(0x140c7bad0, "content_140c7bad0", fn_140c7bad0 as *const (), Throws::Discard), // sub_140c7bad0
    Replacement::draft(0x140c7bd20, "content_140c7bd20", fn_140c7bd20 as *const ()), // sub_140c7bd20
    Replacement::draft(0x140c7be10, "content_140c7be10", fn_140c7be10 as *const ()), // sub_140c7be10
    Replacement::draft_throws(0x140c7bf40, "content_140c7bf40", fn_140c7bf40 as *const (), Throws::Discard), // sub_140c7bf40
    Replacement::draft(0x140c7c1b0, "content_140c7c1b0", fn_140c7c1b0 as *const ()), // sub_140c7c1b0
    Replacement::draft_throws(0x140c7c270, "content_140c7c270", fn_140c7c270 as *const (), Throws::Caught), // sub_140c7c270
    Replacement::draft_throws(0x140c7c6e0, "content_140c7c6e0", fn_140c7c6e0 as *const (), Throws::Caught), // sub_140c7c6e0
    Replacement::draft_throws(0x140c7cb10, "content_140c7cb10", fn_140c7cb10 as *const (), Throws::Caught), // sub_140c7cb10
    Replacement::draft(0x140c7ce60, "content_140c7ce60", fn_140c7ce60 as *const ()), // sub_140c7ce60
    Replacement::draft(0x140c7d060, "content_140c7d060", fn_140c7d060 as *const ()), // sub_140c7d060
    Replacement::draft_throws(0x140c7d270, "content_140c7d270", fn_140c7d270 as *const (), Throws::Discard), // sub_140c7d270
    Replacement::draft_throws(0x140c7d3a0, "content_140c7d3a0", fn_140c7d3a0 as *const (), Throws::Discard), // sub_140c7d3a0
    Replacement::draft(0x140c7d420, "content_140c7d420", fn_140c7d420 as *const ()), // sub_140c7d420
    Replacement::draft(0x140c7d490, "content_140c7d490", tpl_2076::<0x14387f278, 0x14243b0b0, 0x338, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c7d490
    Replacement::draft_throws(0x140c7d500, "content_140c7d500", fn_140c7d500 as *const (), Throws::Discard), // sub_140c7d500
    Replacement::draft_throws(0x140c7d590, "content_140c7d590", fn_140c7d590 as *const (), Throws::Caught), // sub_140c7d590
    Replacement::draft_throws(0x140c7db00, "content_140c7db00", fn_140c7db00 as *const (), Throws::Caught), // sub_140c7db00
    Replacement::draft_throws(0x140c7e030, "content_140c7e030", fn_140c7e030 as *const (), Throws::Caught), // sub_140c7e030
    Replacement::draft(0x140c7e380, "content_140c7e380", tpl_2080::<0x14387f384, 0x14387f278, 0x1401e9970, 0x338, 0xd8, 0x1401e9860> as *const ()), // sub_140c7e380
    Replacement::draft(0x140c7e3f0, "content_140c7e3f0", tpl_2076::<0x14387f278, 0x14243b0b0, 0x338, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c7e3f0
    Replacement::draft_throws(0x140c7e460, "content_140c7e460", tpl_15279::<0x14387f384, 0x14387f278, 0x1401e9770, 0x338, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140c7e460
    Replacement::draft_throws(0x140c7e4d0, "content_140c7e4d0", fn_140c7e4d0 as *const (), Throws::Caught), // sub_140c7e4d0
    Replacement::draft(0x140c7e6b0, "content_140c7e6b0", fn_140c7e6b0 as *const ()), // sub_140c7e6b0
    Replacement::draft(0x140c7e7f0, "content_140c7e7f0", fn_140c7e7f0 as *const ()), // sub_140c7e7f0
    Replacement::draft(0x140c7e980, "content_140c7e980", fn_140c7e980 as *const ()), // sub_140c7e980
    Replacement::draft_throws(0x140c7eb00, "content_140c7eb00", fn_140c7eb00 as *const (), Throws::Discard), // sub_140c7eb00
    Replacement::draft(0x140c7ec50, "content_140c7ec50", fn_140c7ec50 as *const ()), // sub_140c7ec50
    Replacement::draft_throws(0x140c7ed10, "content_140c7ed10", fn_140c7ed10 as *const (), Throws::Caught), // sub_140c7ed10
    Replacement::draft_throws(0x140c7f180, "content_140c7f180", fn_140c7f180 as *const (), Throws::Caught), // sub_140c7f180
    Replacement::draft_throws(0x140c7f5b0, "content_140c7f5b0", fn_140c7f5b0 as *const (), Throws::Caught), // sub_140c7f5b0
    Replacement::draft(0x140c7f900, "content_140c7f900", fn_140c7f900 as *const ()), // sub_140c7f900
    Replacement::draft(0x140c7fa00, "content_140c7fa00", fn_140c7fa00 as *const ()), // sub_140c7fa00
    Replacement::draft_throws(0x140c7fb10, "content_140c7fb10", fn_140c7fb10 as *const (), Throws::Discard), // sub_140c7fb10
    Replacement::draft_throws(0x140c7fc10, "content_140c7fc10", fn_140c7fc10 as *const (), Throws::Caught), // sub_140c7fc10
    Replacement::draft(0x140c7ff40, "content_140c7ff40", fn_140c7ff40 as *const ()), // sub_140c7ff40
    Replacement::draft_throws(0x140c801a0, "content_140c801a0", fn_140c801a0 as *const (), Throws::Caught), // sub_140c801a0
    Replacement::draft(0x140c803b0, "content_140c803b0", fn_140c803b0 as *const ()), // sub_140c803b0
    Replacement::draft(0x140c804f0, "content_140c804f0", fn_140c804f0 as *const ()), // sub_140c804f0
    Replacement::draft(0x140c80780, "content_140c80780", fn_140c80780 as *const ()), // sub_140c80780
    Replacement::draft_throws(0x140c808f0, "content_140c808f0", fn_140c808f0 as *const (), Throws::Discard), // sub_140c808f0
    Replacement::draft_throws(0x140c80c20, "content_140c80c20", fn_140c80c20 as *const (), Throws::Discard), // sub_140c80c20
    Replacement::draft(0x140c80de0, "content_140c80de0", fn_140c80de0 as *const ()), // sub_140c80de0
    Replacement::draft_throws(0x140c80f90, "content_140c80f90", fn_140c80f90 as *const (), Throws::Caught), // sub_140c80f90
    Replacement::draft_throws(0x140c81400, "content_140c81400", fn_140c81400 as *const (), Throws::Caught), // sub_140c81400
    Replacement::draft_throws(0x140c81870, "content_140c81870", fn_140c81870 as *const (), Throws::Caught), // sub_140c81870
    Replacement::draft_throws(0x140c81ca0, "content_140c81ca0", fn_140c81ca0 as *const (), Throws::Caught), // sub_140c81ca0
    Replacement::draft_throws(0x140c820d0, "content_140c820d0", fn_140c820d0 as *const (), Throws::Caught), // sub_140c820d0
    Replacement::draft_throws(0x140c82420, "content_140c82420", fn_140c82420 as *const (), Throws::Caught), // sub_140c82420
    Replacement::draft(0x140c82770, "content_140c82770", fn_140c82770 as *const ()), // sub_140c82770
    Replacement::draft(0x140c82a00, "content_140c82a00", fn_140c82a00 as *const ()), // sub_140c82a00
    Replacement::draft(0x140c82b60, "content_140c82b60", fn_140c82b60 as *const ()), // sub_140c82b60
    Replacement::draft(0x140c82df0, "content_140c82df0", fn_140c82df0 as *const ()), // sub_140c82df0
    Replacement::draft_throws(0x140c82f70, "content_140c82f70", fn_140c82f70 as *const (), Throws::Discard), // sub_140c82f70
    Replacement::draft_throws(0x140c83210, "content_140c83210", fn_140c83210 as *const (), Throws::Discard), // sub_140c83210
    Replacement::draft_throws(0x140c83350, "content_140c83350", fn_140c83350 as *const (), Throws::Discard), // sub_140c83350
    Replacement::draft(0x140c83440, "content_140c83440", fn_140c83440 as *const ()), // sub_140c83440
    Replacement::draft(0x140c834f0, "content_140c834f0", fn_140c834f0 as *const ()), // sub_140c834f0
    Replacement::draft_throws(0x140c835e0, "content_140c835e0", fn_140c835e0 as *const (), Throws::Discard), // sub_140c835e0
    Replacement::draft(0x140c836e0, "content_140c836e0", fn_140c836e0 as *const ()), // sub_140c836e0
    Replacement::draft_throws(0x140c83790, "content_140c83790", fn_140c83790 as *const (), Throws::Caught), // sub_140c83790
    Replacement::draft_throws(0x140c83d50, "content_140c83d50", fn_140c83d50 as *const (), Throws::Caught), // sub_140c83d50
    Replacement::draft_throws(0x140c842d0, "content_140c842d0", fn_140c842d0 as *const (), Throws::Caught), // sub_140c842d0
    Replacement::draft(0x140c84620, "content_140c84620", fn_140c84620 as *const ()), // sub_140c84620
    Replacement::draft(0x140c84710, "content_140c84710", fn_140c84710 as *const ()), // sub_140c84710
    Replacement::draft_throws(0x140c84810, "content_140c84810", fn_140c84810 as *const (), Throws::Discard), // sub_140c84810
    Replacement::draft_throws(0x140c84900, "content_140c84900", fn_140c84900 as *const (), Throws::Discard), // sub_140c84900
    Replacement::draft(0x140c84a40, "content_140c84a40", fn_140c84a40 as *const ()), // sub_140c84a40
    Replacement::draft(0x140c84b10, "content_140c84b10", fn_140c84b10 as *const ()), // sub_140c84b10
    Replacement::draft_throws(0x140c84bb0, "content_140c84bb0", fn_140c84bb0 as *const (), Throws::Discard), // sub_140c84bb0
    Replacement::draft_throws(0x140c84cf0, "content_140c84cf0", fn_140c84cf0 as *const (), Throws::Caught), // sub_140c84cf0
    Replacement::draft_throws(0x140c85160, "content_140c85160", fn_140c85160 as *const (), Throws::Caught), // sub_140c85160
    Replacement::draft_throws(0x140c85590, "content_140c85590", fn_140c85590 as *const (), Throws::Caught), // sub_140c85590
    Replacement::draft(0x140c858e0, "content_140c858e0", fn_140c858e0 as *const ()), // sub_140c858e0
    Replacement::draft(0x140c859c0, "content_140c859c0", fn_140c859c0 as *const ()), // sub_140c859c0
    Replacement::draft_throws(0x140c85ab0, "content_140c85ab0", fn_140c85ab0 as *const (), Throws::Discard), // sub_140c85ab0
    Replacement::draft_throws(0x140c85b40, "content_140c85b40", fn_140c85b40 as *const (), Throws::Discard), // sub_140c85b40
    Replacement::draft(0x140c85cc0, "content_140c85cc0", fn_140c85cc0 as *const ()), // sub_140c85cc0
    Replacement::draft(0x140c85d90, "content_140c85d90", fn_140c85d90 as *const ()), // sub_140c85d90
    Replacement::draft_throws(0x140c85e80, "content_140c85e80", fn_140c85e80 as *const (), Throws::Discard), // sub_140c85e80
    Replacement::draft(0x140c86030, "content_140c86030", fn_140c86030 as *const ()), // sub_140c86030
    Replacement::draft_throws(0x140c860a0, "content_140c860a0", fn_140c860a0 as *const (), Throws::Caught), // sub_140c860a0
    Replacement::draft_throws(0x140c866a0, "content_140c866a0", fn_140c866a0 as *const (), Throws::Caught), // sub_140c866a0
    Replacement::draft_throws(0x140c86c60, "content_140c86c60", fn_140c86c60 as *const (), Throws::Caught), // sub_140c86c60
    Replacement::draft(0x140c86fb0, "content_140c86fb0", fn_140c86fb0 as *const ()), // sub_140c86fb0
    Replacement::draft(0x140c870a0, "content_140c870a0", fn_140c870a0 as *const ()), // sub_140c870a0
    Replacement::draft_throws(0x140c871a0, "content_140c871a0", fn_140c871a0 as *const (), Throws::Discard), // sub_140c871a0
    Replacement::draft_throws(0x140c872b0, "content_140c872b0", fn_140c872b0 as *const (), Throws::Discard), // sub_140c872b0
    Replacement::draft(0x140c87400, "content_140c87400", fn_140c87400 as *const ()), // sub_140c87400
    Replacement::draft(0x140c874e0, "content_140c874e0", fn_140c874e0 as *const ()), // sub_140c874e0
    Replacement::draft_throws(0x140c87590, "content_140c87590", fn_140c87590 as *const (), Throws::Discard), // sub_140c87590
    Replacement::draft_throws(0x140c876e0, "content_140c876e0", fn_140c876e0 as *const (), Throws::Caught), // sub_140c876e0
    Replacement::draft_throws(0x140c87b50, "content_140c87b50", fn_140c87b50 as *const (), Throws::Caught), // sub_140c87b50
    Replacement::draft_throws(0x140c87f80, "content_140c87f80", fn_140c87f80 as *const (), Throws::Caught), // sub_140c87f80
    Replacement::draft(0x140c882d0, "content_140c882d0", fn_140c882d0 as *const ()), // sub_140c882d0
    Replacement::draft(0x140c883c0, "content_140c883c0", fn_140c883c0 as *const ()), // sub_140c883c0
    Replacement::draft_throws(0x140c884c0, "content_140c884c0", fn_140c884c0 as *const (), Throws::Discard), // sub_140c884c0
    Replacement::draft_throws(0x140c88560, "content_140c88560", fn_140c88560 as *const (), Throws::Discard), // sub_140c88560
    Replacement::draft(0x140c88690, "content_140c88690", fn_140c88690 as *const ()), // sub_140c88690
    Replacement::draft(0x140c88710, "content_140c88710", fn_140c88710 as *const ()), // sub_140c88710
    Replacement::draft_throws(0x140c887c0, "content_140c887c0", fn_140c887c0 as *const (), Throws::Discard), // sub_140c887c0
    Replacement::draft_throws(0x140c88920, "content_140c88920", fn_140c88920 as *const (), Throws::Caught), // sub_140c88920
    Replacement::draft_throws(0x140c88ea0, "content_140c88ea0", fn_140c88ea0 as *const (), Throws::Caught), // sub_140c88ea0
    Replacement::draft_throws(0x140c893e0, "content_140c893e0", fn_140c893e0 as *const (), Throws::Caught), // sub_140c893e0
    Replacement::draft(0x140c89730, "content_140c89730", fn_140c89730 as *const ()), // sub_140c89730
    Replacement::draft(0x140c897f0, "content_140c897f0", fn_140c897f0 as *const ()), // sub_140c897f0
    Replacement::draft_throws(0x140c898c0, "content_140c898c0", fn_140c898c0 as *const (), Throws::Discard), // sub_140c898c0
    Replacement::draft_throws(0x140c899a0, "content_140c899a0", fn_140c899a0 as *const (), Throws::Caught), // sub_140c899a0
    Replacement::draft_throws(0x140c89c10, "content_140c89c10", fn_140c89c10 as *const (), Throws::Caught), // sub_140c89c10
    Replacement::draft_throws(0x140c89e80, "content_140c89e80", fn_140c89e80 as *const (), Throws::Caught), // sub_140c89e80
    Replacement::draft_throws(0x140c8a0f0, "content_140c8a0f0", fn_140c8a0f0 as *const (), Throws::Discard), // sub_140c8a0f0
    Replacement::draft(0x140c8a220, "content_140c8a220", fn_140c8a220 as *const ()), // sub_140c8a220
    Replacement::draft(0x140c8a2f0, "content_140c8a2f0", fn_140c8a2f0 as *const ()), // sub_140c8a2f0
    Replacement::draft_throws(0x140c8a3f0, "content_140c8a3f0", fn_140c8a3f0 as *const (), Throws::Discard), // sub_140c8a3f0
    Replacement::draft(0x140c8a560, "content_140c8a560", fn_140c8a560 as *const ()), // sub_140c8a560
    Replacement::draft_throws(0x140c8a5f0, "content_140c8a5f0", fn_140c8a5f0 as *const (), Throws::Caught), // sub_140c8a5f0
    Replacement::draft_throws(0x140c8abf0, "content_140c8abf0", fn_140c8abf0 as *const (), Throws::Caught), // sub_140c8abf0
    Replacement::draft_throws(0x140c8b1b0, "content_140c8b1b0", fn_140c8b1b0 as *const (), Throws::Caught), // sub_140c8b1b0
    Replacement::draft(0x140c8b500, "content_140c8b500", fn_140c8b500 as *const ()), // sub_140c8b500
    Replacement::draft(0x140c8b600, "content_140c8b600", fn_140c8b600 as *const ()), // sub_140c8b600
    Replacement::draft_throws(0x140c8b710, "content_140c8b710", fn_140c8b710 as *const (), Throws::Discard), // sub_140c8b710
    Replacement::draft_throws(0x140c8b810, "content_140c8b810", fn_140c8b810 as *const (), Throws::Caught), // sub_140c8b810
    Replacement::draft(0x140c8b960, "content_140c8b960", fn_140c8b960 as *const ()), // sub_140c8b960
    Replacement::draft(0x140c8ba90, "content_140c8ba90", fn_140c8ba90 as *const ()), // sub_140c8ba90
    Replacement::draft_throws(0x140c8bb70, "content_140c8bb70", fn_140c8bb70 as *const (), Throws::Discard), // sub_140c8bb70
    Replacement::draft_throws(0x140c8bc50, "content_140c8bc50", fn_140c8bc50 as *const (), Throws::Caught), // sub_140c8bc50
    Replacement::draft_throws(0x140c8c0c0, "content_140c8c0c0", fn_140c8c0c0 as *const (), Throws::Caught), // sub_140c8c0c0
    Replacement::draft_throws(0x140c8c4f0, "content_140c8c4f0", fn_140c8c4f0 as *const (), Throws::Caught), // sub_140c8c4f0
    Replacement::draft(0x140c8c840, "content_140c8c840", fn_140c8c840 as *const ()), // sub_140c8c840
    Replacement::draft(0x140c8c900, "content_140c8c900", fn_140c8c900 as *const ()), // sub_140c8c900
    Replacement::draft_throws(0x140c8c9d0, "content_140c8c9d0", fn_140c8c9d0 as *const (), Throws::Discard), // sub_140c8c9d0
    Replacement::draft_throws(0x140c8ca90, "content_140c8ca90", fn_140c8ca90 as *const (), Throws::Discard), // sub_140c8ca90
    Replacement::draft(0x140c8cca0, "content_140c8cca0", fn_140c8cca0 as *const ()), // sub_140c8cca0
    Replacement::draft(0x140c8cd90, "content_140c8cd90", fn_140c8cd90 as *const ()), // sub_140c8cd90
    Replacement::draft_throws(0x140c8ce90, "content_140c8ce90", fn_140c8ce90 as *const (), Throws::Discard), // sub_140c8ce90
    Replacement::draft(0x140c8d0f0, "content_140c8d0f0", fn_140c8d0f0 as *const ()), // sub_140c8d0f0
    Replacement::draft_throws(0x140c8d1b0, "content_140c8d1b0", fn_140c8d1b0 as *const (), Throws::Caught), // sub_140c8d1b0
    Replacement::draft_throws(0x140c8d620, "content_140c8d620", fn_140c8d620 as *const (), Throws::Caught), // sub_140c8d620
    Replacement::draft_throws(0x140c8da50, "content_140c8da50", fn_140c8da50 as *const (), Throws::Caught), // sub_140c8da50
    Replacement::draft(0x140c8dda0, "content_140c8dda0", fn_140c8dda0 as *const ()), // sub_140c8dda0
    Replacement::draft(0x140c8deb0, "content_140c8deb0", fn_140c8deb0 as *const ()), // sub_140c8deb0
    Replacement::draft_throws(0x140c8dfc0, "content_140c8dfc0", fn_140c8dfc0 as *const (), Throws::Discard), // sub_140c8dfc0
    Replacement::draft_throws(0x140c8e100, "content_140c8e100", fn_140c8e100 as *const (), Throws::Discard), // sub_140c8e100
    Replacement::draft(0x140c8e1f0, "content_140c8e1f0", fn_140c8e1f0 as *const ()), // sub_140c8e1f0
    Replacement::draft(0x140c8e2e0, "content_140c8e2e0", fn_140c8e2e0 as *const ()), // sub_140c8e2e0
    Replacement::draft_throws(0x140c8e3f0, "content_140c8e3f0", fn_140c8e3f0 as *const (), Throws::Discard), // sub_140c8e3f0
    Replacement::draft(0x140c8e4f0, "content_140c8e4f0", fn_140c8e4f0 as *const ()), // sub_140c8e4f0
    Replacement::draft_throws(0x140c8e5c0, "content_140c8e5c0", fn_140c8e5c0 as *const (), Throws::Caught), // sub_140c8e5c0
    Replacement::draft_throws(0x140c8ea30, "content_140c8ea30", fn_140c8ea30 as *const (), Throws::Caught), // sub_140c8ea30
    Replacement::draft_throws(0x140c8ee60, "content_140c8ee60", fn_140c8ee60 as *const (), Throws::Caught), // sub_140c8ee60
    Replacement::draft(0x140c8f1b0, "content_140c8f1b0", fn_140c8f1b0 as *const ()), // sub_140c8f1b0
    Replacement::draft(0x140c8f2f0, "content_140c8f2f0", fn_140c8f2f0 as *const ()), // sub_140c8f2f0
    Replacement::draft_throws(0x140c8f400, "content_140c8f400", fn_140c8f400 as *const (), Throws::Discard), // sub_140c8f400
    Replacement::draft_throws(0x140c8f510, "content_140c8f510", fn_140c8f510 as *const (), Throws::Caught), // sub_140c8f510
    Replacement::draft(0x140c8f630, "content_140c8f630", fn_140c8f630 as *const ()), // sub_140c8f630
    Replacement::draft(0x140c8f760, "content_140c8f760", fn_140c8f760 as *const ()), // sub_140c8f760
    Replacement::draft_throws(0x140c8f840, "content_140c8f840", fn_140c8f840 as *const (), Throws::Discard), // sub_140c8f840
    Replacement::draft_throws(0x140c8f920, "content_140c8f920", fn_140c8f920 as *const (), Throws::Caught), // sub_140c8f920
    Replacement::draft_throws(0x140c8fd90, "content_140c8fd90", fn_140c8fd90 as *const (), Throws::Caught), // sub_140c8fd90
    Replacement::draft_throws(0x140c901c0, "content_140c901c0", fn_140c901c0 as *const (), Throws::Caught), // sub_140c901c0
    Replacement::draft(0x140c90510, "content_140c90510", fn_140c90510 as *const ()), // sub_140c90510
    Replacement::draft(0x140c905d0, "content_140c905d0", fn_140c905d0 as *const ()), // sub_140c905d0
    Replacement::draft_throws(0x140c906a0, "content_140c906a0", fn_140c906a0 as *const (), Throws::Discard), // sub_140c906a0
    Replacement::draft_throws(0x140c90760, "content_140c90760", fn_140c90760 as *const (), Throws::Discard), // sub_140c90760
    Replacement::draft(0x140c907f0, "content_140c907f0", fn_140c907f0 as *const ()), // sub_140c907f0
    Replacement::draft(0x140c90860, "content_140c90860", tpl_2329::<0x14387f218, 0x14243b0b0, 0x338, 0x38, 0x8, 0x1407de580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c90860
    Replacement::draft_throws(0x140c908e0, "content_140c908e0", fn_140c908e0 as *const (), Throws::Discard), // sub_140c908e0
    Replacement::draft(0x140c90980, "content_140c90980", fn_140c90980 as *const ()), // sub_140c90980
    Replacement::draft_throws(0x140c90a10, "content_140c90a10", fn_140c90a10 as *const (), Throws::Caught), // sub_140c90a10
    Replacement::draft_throws(0x140c90f90, "content_140c90f90", fn_140c90f90 as *const (), Throws::Caught), // sub_140c90f90
    Replacement::draft_throws(0x140c914d0, "content_140c914d0", fn_140c914d0 as *const (), Throws::Caught), // sub_140c914d0
    Replacement::draft(0x140c91820, "content_140c91820", tpl_2342::<0x14387f448, 0x14387f218, 0x1401e9970, 0x338, 0xd8, 0x8, 0x1407df670, 0x1401e9860> as *const ()), // sub_140c91820
    Replacement::draft(0x140c91890, "content_140c91890", tpl_2329::<0x14387f218, 0x14243b0b0, 0x338, 0xd8, 0x8, 0x1407df7d0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c91890
    Replacement::draft_throws(0x140c91910, "content_140c91910", fn_140c91910 as *const (), Throws::Discard), // sub_140c91910
    Replacement::draft_throws(0x140c91990, "content_140c91990", fn_140c91990 as *const (), Throws::Discard), // sub_140c91990
    Replacement::draft(0x140c91b00, "content_140c91b00", fn_140c91b00 as *const ()), // sub_140c91b00
    Replacement::draft(0x140c91bb0, "content_140c91bb0", fn_140c91bb0 as *const ()), // sub_140c91bb0
    Replacement::draft_throws(0x140c91c90, "content_140c91c90", fn_140c91c90 as *const (), Throws::Discard), // sub_140c91c90
    Replacement::draft(0x140c91df0, "content_140c91df0", fn_140c91df0 as *const ()), // sub_140c91df0
    Replacement::draft_throws(0x140c91e60, "content_140c91e60", fn_140c91e60 as *const (), Throws::Caught), // sub_140c91e60
    Replacement::draft_throws(0x140c92450, "content_140c92450", fn_140c92450 as *const (), Throws::Caught), // sub_140c92450
    Replacement::draft_throws(0x140c929f0, "content_140c929f0", fn_140c929f0 as *const (), Throws::Caught), // sub_140c929f0
    Replacement::draft(0x140c92d40, "content_140c92d40", fn_140c92d40 as *const ()), // sub_140c92d40
    Replacement::draft(0x140c92e60, "content_140c92e60", fn_140c92e60 as *const ()), // sub_140c92e60
    Replacement::draft_throws(0x140c92f90, "content_140c92f90", fn_140c92f90 as *const (), Throws::Discard), // sub_140c92f90
    Replacement::draft_throws(0x140c93050, "content_140c93050", fn_140c93050 as *const (), Throws::Discard), // sub_140c93050
    Replacement::draft(0x140c93140, "content_140c93140", fn_140c93140 as *const ()), // sub_140c93140
    Replacement::draft(0x140c931e0, "content_140c931e0", fn_140c931e0 as *const ()), // sub_140c931e0
    Replacement::draft_throws(0x140c93260, "content_140c93260", fn_140c93260 as *const (), Throws::Discard), // sub_140c93260
    Replacement::draft_throws(0x140c93360, "content_140c93360", fn_140c93360 as *const (), Throws::Caught), // sub_140c93360
    Replacement::draft_throws(0x140c93940, "content_140c93940", fn_140c93940 as *const (), Throws::Caught), // sub_140c93940
    Replacement::draft_throws(0x140c93ec0, "content_140c93ec0", fn_140c93ec0 as *const (), Throws::Caught), // sub_140c93ec0
    Replacement::draft(0x140c94210, "content_140c94210", fn_140c94210 as *const ()), // sub_140c94210
    Replacement::draft(0x140c942e0, "content_140c942e0", fn_140c942e0 as *const ()), // sub_140c942e0
    Replacement::draft_throws(0x140c943c0, "content_140c943c0", fn_140c943c0 as *const (), Throws::Discard), // sub_140c943c0
    Replacement::draft_throws(0x140c94440, "content_140c94440", fn_140c94440 as *const (), Throws::Caught), // sub_140c94440
    Replacement::draft(0x140c94580, "content_140c94580", fn_140c94580 as *const ()), // sub_140c94580
    Replacement::draft(0x140c946c0, "content_140c946c0", fn_140c946c0 as *const ()), // sub_140c946c0
    Replacement::draft_throws(0x140c947b0, "content_140c947b0", fn_140c947b0 as *const (), Throws::Discard), // sub_140c947b0
    Replacement::draft_throws(0x140c948a0, "content_140c948a0", fn_140c948a0 as *const (), Throws::Caught), // sub_140c948a0
    Replacement::draft_throws(0x140c94d10, "content_140c94d10", fn_140c94d10 as *const (), Throws::Caught), // sub_140c94d10
    Replacement::draft_throws(0x140c95140, "content_140c95140", fn_140c95140 as *const (), Throws::Caught), // sub_140c95140
    Replacement::draft(0x140c95490, "content_140c95490", fn_140c95490 as *const ()), // sub_140c95490
    Replacement::draft(0x140c95550, "content_140c95550", fn_140c95550 as *const ()), // sub_140c95550
    Replacement::draft_throws(0x140c95620, "content_140c95620", fn_140c95620 as *const (), Throws::Discard), // sub_140c95620
    Replacement::draft_throws(0x140c956f0, "content_140c956f0", fn_140c956f0 as *const (), Throws::Caught), // sub_140c956f0
    Replacement::draft(0x140c95820, "content_140c95820", fn_140c95820 as *const ()), // sub_140c95820
    Replacement::draft(0x140c95a00, "content_140c95a00", fn_140c95a00 as *const ()), // sub_140c95a00
    Replacement::draft_throws(0x140c95b00, "content_140c95b00", fn_140c95b00 as *const (), Throws::Discard), // sub_140c95b00
    Replacement::draft_throws(0x140c95b80, "content_140c95b80", fn_140c95b80 as *const (), Throws::Caught), // sub_140c95b80
    Replacement::draft_throws(0x140c95ff0, "content_140c95ff0", fn_140c95ff0 as *const (), Throws::Caught), // sub_140c95ff0
    Replacement::draft_throws(0x140c96420, "content_140c96420", fn_140c96420 as *const (), Throws::Caught), // sub_140c96420
    Replacement::draft(0x140c96770, "content_140c96770", fn_140c96770 as *const ()), // sub_140c96770
    Replacement::draft(0x140c96830, "content_140c96830", fn_140c96830 as *const ()), // sub_140c96830
    Replacement::draft_throws(0x140c96900, "content_140c96900", fn_140c96900 as *const (), Throws::Discard), // sub_140c96900
    Replacement::draft_throws(0x140c969c0, "content_140c969c0", tpl_74013::<0x14387f310, 0x14387f268, 0x1401e9fa0, 0x328, 0x338, 0x1403c7b30, 0x369, 0x1401e84a0, 0x48, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140c969c0
    Replacement::draft(0x140c96ad0, "content_140c96ad0", tpl_15609::<0x14387f268, 0x14243b0b0, 0x8, 0x1401e84a0, 0x20, 0x24, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c96ad0
    Replacement::draft_throws(0x140c96b80, "content_140c96b80", tpl_74013::<0x14387f220, 0x14387f360, 0x1401e9fa0, 0x328, 0x338, 0x1403c7b30, 0x369, 0x1401e84a0, 0x48, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140c96b80
    Replacement::draft(0x140c96c90, "content_140c96c90", tpl_15609::<0x14387f360, 0x14243b0b0, 0x8, 0x1401e84a0, 0x20, 0x24, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c96c90
    Replacement::draft(0x140c96d40, "content_140c96d40", tpl_15610::<0x14387f268, 0x14243b0b0, 0x338, 0x8, 0x1403c7b30, 0x20, 0x48, 0x24, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c96d40
    Replacement::draft(0x140c96dd0, "content_140c96dd0", tpl_15610::<0x14387f360, 0x14243b0b0, 0x338, 0x8, 0x1403c7b30, 0x20, 0x48, 0x24, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140c96dd0
    Replacement::draft_throws(0x140c96e60, "content_140c96e60", tpl_74014::<0x14387f310, 0x14387f268, 0x1401e9b60, 0x1406f1f30, 0x1401e84a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140c96e60
    Replacement::draft_throws(0x140c96f90, "content_140c96f90", tpl_74014::<0x14387f220, 0x14387f360, 0x1401e9b60, 0x1406f1f30, 0x1401e84a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140c96f90
    Replacement::draft_throws(0x140c970c0, "content_140c970c0", fn_140c970c0 as *const (), Throws::Caught), // sub_140c970c0
    Replacement::draft_throws(0x140c976b0, "content_140c976b0", fn_140c976b0 as *const (), Throws::Caught), // sub_140c976b0
    Replacement::draft_throws(0x140c97ca0, "content_140c97ca0", fn_140c97ca0 as *const (), Throws::Caught), // sub_140c97ca0
    Replacement::draft_throws(0x140c98230, "content_140c98230", fn_140c98230 as *const (), Throws::Caught), // sub_140c98230
    Replacement::draft_throws(0x140c987c0, "content_140c987c0", fn_140c987c0 as *const (), Throws::Caught), // sub_140c987c0
    Replacement::draft_throws(0x140c98b10, "content_140c98b10", fn_140c98b10 as *const (), Throws::Caught), // sub_140c98b10
    Replacement::draft(0x140c98e60, "content_140c98e60", tpl_15613::<0x14387f310, 0x14387f268, 0x1401e9970, 0x338, 0x8, 0x88, 0xc0, 0x14240ab10, 0x20, 0xe8, 0x24, 0xd8, 0x1401e9860> as *const ()), // sub_140c98e60
    Replacement::draft(0x140c98f40, "content_140c98f40", tpl_15613::<0x14387f220, 0x14387f360, 0x1401e9970, 0x338, 0x8, 0x88, 0xc0, 0x14240ab10, 0x20, 0xe8, 0x24, 0xd8, 0x1401e9860> as *const ()), // sub_140c98f40
    Replacement::draft(0x140c99020, "content_140c99020", tpl_15614::<0x14387f268, 0x14243b0b0, 0x338, 0x8, 0x88, 0xc0, 0x14240ab10, 0x20, 0xe8, 0x24, 0xd8, 0xfffffffffffffff8> as *const ()), // sub_140c99020
    Replacement::draft(0x140c99110, "content_140c99110", tpl_15614::<0x14387f360, 0x14243b0b0, 0x338, 0x8, 0x88, 0xc0, 0x14240ab10, 0x20, 0xe8, 0x24, 0xd8, 0xfffffffffffffff8> as *const ()), // sub_140c99110
    Replacement::draft_throws(0x140c99200, "content_140c99200", tpl_15615::<0x14387f310, 0x14387f268, 0x1401e9770, 0x8, 0x1406deba0, 0x338, 0x20, 0x120, 0x24, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140c99200
    Replacement::draft_throws(0x140c992a0, "content_140c992a0", tpl_15615::<0x14387f220, 0x14387f360, 0x1401e9770, 0x8, 0x1406deba0, 0x338, 0x20, 0x120, 0x24, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140c992a0
    Replacement::draft(0x140c99340, "content_140c99340", fn_140c99340 as *const ()), // sub_140c99340
    Replacement::draft_throws(0x140c99410, "content_140c99410", fn_140c99410 as *const (), Throws::Discard), // sub_140c99410
    Replacement::draft(0x140c995b0, "content_140c995b0", fn_140c995b0 as *const ()), // sub_140c995b0
    Replacement::draft_throws(0x140c996b0, "content_140c996b0", fn_140c996b0 as *const (), Throws::Caught), // sub_140c996b0
    Replacement::draft(0x140c99850, "content_140c99850", fn_140c99850 as *const ()), // sub_140c99850
    Replacement::draft_throws(0x140c99a60, "content_140c99a60", fn_140c99a60 as *const (), Throws::Discard), // sub_140c99a60
    Replacement::draft(0x140c99ba0, "content_140c99ba0", fn_140c99ba0 as *const ()), // sub_140c99ba0
    Replacement::draft(0x140c99c50, "content_140c99c50", fn_140c99c50 as *const ()), // sub_140c99c50
    Replacement::draft(0x140c99d30, "content_140c99d30", fn_140c99d30 as *const ()), // sub_140c99d30
    Replacement::draft(0x140c99e60, "content_140c99e60", fn_140c99e60 as *const ()), // sub_140c99e60
    Replacement::draft_throws(0x140c99fb0, "content_140c99fb0", fn_140c99fb0 as *const (), Throws::Discard), // sub_140c99fb0
    Replacement::draft_throws(0x140c9a160, "content_140c9a160", fn_140c9a160 as *const (), Throws::Discard), // sub_140c9a160
    Replacement::draft_throws(0x140c9a270, "content_140c9a270", fn_140c9a270 as *const (), Throws::Discard), // sub_140c9a270
    Replacement::draft(0x140c9a3d0, "content_140c9a3d0", fn_140c9a3d0 as *const ()), // sub_140c9a3d0
    Replacement::draft(0x140c9a450, "content_140c9a450", fn_140c9a450 as *const ()), // sub_140c9a450
    Replacement::draft_throws(0x140c9a550, "content_140c9a550", fn_140c9a550 as *const (), Throws::Caught), // sub_140c9a550
    Replacement::draft_throws(0x140c9a9c0, "content_140c9a9c0", fn_140c9a9c0 as *const (), Throws::Caught), // sub_140c9a9c0
    Replacement::draft_throws(0x140c9ae30, "content_140c9ae30", fn_140c9ae30 as *const (), Throws::Caught), // sub_140c9ae30
    Replacement::draft_throws(0x140c9b2a0, "content_140c9b2a0", fn_140c9b2a0 as *const (), Throws::Caught), // sub_140c9b2a0
    Replacement::draft_throws(0x140c9b6d0, "content_140c9b6d0", fn_140c9b6d0 as *const (), Throws::Caught), // sub_140c9b6d0
    Replacement::draft_throws(0x140c9bb00, "content_140c9bb00", fn_140c9bb00 as *const (), Throws::Caught), // sub_140c9bb00
    Replacement::draft_throws(0x140c9bf30, "content_140c9bf30", fn_140c9bf30 as *const (), Throws::Caught), // sub_140c9bf30
    Replacement::draft_throws(0x140c9c280, "content_140c9c280", fn_140c9c280 as *const (), Throws::Caught), // sub_140c9c280
    Replacement::draft_throws(0x140c9c5d0, "content_140c9c5d0", fn_140c9c5d0 as *const (), Throws::Caught), // sub_140c9c5d0
    Replacement::draft(0x140c9c920, "content_140c9c920", fn_140c9c920 as *const ()), // sub_140c9c920
    Replacement::draft(0x140c9ca50, "content_140c9ca50", fn_140c9ca50 as *const ()), // sub_140c9ca50
    Replacement::draft(0x140c9cb40, "content_140c9cb40", fn_140c9cb40 as *const ()), // sub_140c9cb40
    Replacement::draft(0x140c9cc40, "content_140c9cc40", fn_140c9cc40 as *const ()), // sub_140c9cc40
    Replacement::draft(0x140c9cd80, "content_140c9cd80", fn_140c9cd80 as *const ()), // sub_140c9cd80
    Replacement::draft(0x140c9ce80, "content_140c9ce80", fn_140c9ce80 as *const ()), // sub_140c9ce80
    Replacement::draft_throws(0x140c9cf80, "content_140c9cf80", fn_140c9cf80 as *const (), Throws::Discard), // sub_140c9cf80
    Replacement::draft_throws(0x140c9d060, "content_140c9d060", fn_140c9d060 as *const (), Throws::Discard), // sub_140c9d060
    Replacement::draft_throws(0x140c9d150, "content_140c9d150", fn_140c9d150 as *const (), Throws::Discard), // sub_140c9d150
    Replacement::draft_throws(0x140c9d240, "content_140c9d240", fn_140c9d240 as *const (), Throws::Discard), // sub_140c9d240
    Replacement::draft(0x140c9d370, "content_140c9d370", fn_140c9d370 as *const ()), // sub_140c9d370
    Replacement::draft(0x140c9d440, "content_140c9d440", fn_140c9d440 as *const ()), // sub_140c9d440
    Replacement::draft_throws(0x140c9d540, "content_140c9d540", fn_140c9d540 as *const (), Throws::Discard), // sub_140c9d540
    Replacement::draft_throws(0x140c9d6b0, "content_140c9d6b0", fn_140c9d6b0 as *const (), Throws::Caught), // sub_140c9d6b0
    Replacement::draft_throws(0x140c9dcb0, "content_140c9dcb0", fn_140c9dcb0 as *const (), Throws::Caught), // sub_140c9dcb0
    Replacement::draft_throws(0x140c9e270, "content_140c9e270", fn_140c9e270 as *const (), Throws::Caught), // sub_140c9e270
    Replacement::draft(0x140c9e5c0, "content_140c9e5c0", fn_140c9e5c0 as *const ()), // sub_140c9e5c0
    Replacement::draft(0x140c9e6c0, "content_140c9e6c0", fn_140c9e6c0 as *const ()), // sub_140c9e6c0
    Replacement::draft_throws(0x140c9e7d0, "content_140c9e7d0", fn_140c9e7d0 as *const (), Throws::Discard), // sub_140c9e7d0
    Replacement::draft_throws(0x140c9e8d0, "content_140c9e8d0", fn_140c9e8d0 as *const (), Throws::Caught), // sub_140c9e8d0
    Replacement::draft_throws(0x140c9ea80, "content_140c9ea80", fn_140c9ea80 as *const (), Throws::Caught), // sub_140c9ea80
    Replacement::draft_throws(0x140c9ec40, "content_140c9ec40", fn_140c9ec40 as *const (), Throws::Discard), // sub_140c9ec40
    Replacement::draft_throws(0x140c9f100, "content_140c9f100", fn_140c9f100 as *const (), Throws::Discard), // sub_140c9f100
    Replacement::draft_throws(0x140c9f260, "content_140c9f260", fn_140c9f260 as *const (), Throws::Discard), // sub_140c9f260
    Replacement::draft_throws(0x140c9f3b0, "content_140c9f3b0", fn_140c9f3b0 as *const (), Throws::Discard), // sub_140c9f3b0
    Replacement::draft_throws(0x140c9f6d0, "content_140c9f6d0", fn_140c9f6d0 as *const (), Throws::Discard), // sub_140c9f6d0
    Replacement::draft_throws(0x140c9f840, "content_140c9f840", fn_140c9f840 as *const (), Throws::Caught), // sub_140c9f840
    Replacement::draft_throws(0x140c9f9b0, "content_140c9f9b0", fn_140c9f9b0 as *const (), Throws::Caught), // sub_140c9f9b0
    Replacement::draft_throws(0x140c9fcc0, "content_140c9fcc0", tpl_15361::<0x14387f278, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143426780, 0x143426a48, 0x143251120, 0x1432d6988, 0x1432a90cc, 0x14020f240> as *const (), Throws::Discard), // sub_140c9fcc0
    Replacement::draft_throws(0x140c9fdb0, "content_140c9fdb0", fn_140c9fdb0 as *const (), Throws::Discard), // sub_140c9fdb0
    Replacement::draft_throws(0x140c9fee0, "content_140c9fee0", fn_140c9fee0 as *const (), Throws::Discard), // sub_140c9fee0
    Replacement::draft_throws(0x140ca0290, "content_140ca0290", fn_140ca0290 as *const (), Throws::Discard), // sub_140ca0290
    Replacement::draft(0x140ca04d0, "content_140ca04d0", tpl_15153::<0x14384ea30, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // sub_140ca04d0
    Replacement::draft_throws(0x140ca0520, "content_140ca0520", fn_140ca0520 as *const (), Throws::Caught), // sub_140ca0520
    Replacement::draft_throws(0x140ca06d0, "content_140ca06d0", fn_140ca06d0 as *const (), Throws::Caught), // sub_140ca06d0
    Replacement::draft_throws(0x140ca0890, "content_140ca0890", fn_140ca0890 as *const (), Throws::Discard), // sub_140ca0890
    Replacement::draft_throws(0x140ca0a20, "content_140ca0a20", fn_140ca0a20 as *const (), Throws::Caught), // sub_140ca0a20
    Replacement::draft_throws(0x140ca0bc0, "content_140ca0bc0", fn_140ca0bc0 as *const (), Throws::Discard), // sub_140ca0bc0
    Replacement::draft_throws(0x140ca0e10, "content_140ca0e10", fn_140ca0e10 as *const (), Throws::Caught), // sub_140ca0e10
    Replacement::draft_throws(0x140ca0fb0, "content_140ca0fb0", fn_140ca0fb0 as *const (), Throws::Discard), // sub_140ca0fb0
    Replacement::draft(0x140ca1170, "content_140ca1170", tpl_9615::<0x14387f1d8, 0x14243b0b0, 0x14387f338, 0x8, 0x14387f2c8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x90> as *const ()), // sub_140ca1170
    Replacement::draft(0x140ca1250, "content_140ca1250", tpl_6798::<0x14387f1d8, 0x14243b0b0, 0x140cbf6c0, 0x90, 0x140cbf770, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ca1250
    Replacement::draft_throws(0x140ca12d0, "content_140ca12d0", fn_140ca12d0 as *const (), Throws::Caught), // sub_140ca12d0
    Replacement::draft(0x140ca1560, "content_140ca1560", tpl_6798::<0x14387f1d8, 0x14243b0b0, 0x140cbfa20, 0x90, 0x140cbfcf0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ca1560
    Replacement::draft(0x140ca15e0, "content_140ca15e0", tpl_6798::<0x14387f1d8, 0x14243b0b0, 0x140cbfde0, 0x90, 0x140cbfea0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ca15e0
    Replacement::draft_throws(0x140ca1660, "content_140ca1660", fn_140ca1660 as *const (), Throws::Caught), // sub_140ca1660
    Replacement::draft(0x140ca18f0, "content_140ca18f0", fn_140ca18f0 as *const ()), // sub_140ca18f0
    Replacement::draft_throws(0x140ca19b0, "content_140ca19b0", fn_140ca19b0 as *const (), Throws::Caught), // sub_140ca19b0
    Replacement::draft_throws(0x140ca2050, "content_140ca2050", fn_140ca2050 as *const (), Throws::Caught), // sub_140ca2050
    Replacement::draft_throws(0x140ca26c0, "content_140ca26c0", fn_140ca26c0 as *const (), Throws::Caught), // sub_140ca26c0
    Replacement::draft(0x140ca2b00, "content_140ca2b00", tpl_6804::<0x14387f44c, 0x14387f1d8, 0x1401e9970, 0x140cc0ab0, 0x90, 0x140cc0d20, 0x1401e9860> as *const ()), // sub_140ca2b00
    Replacement::draft(0x140ca2b70, "content_140ca2b70", tpl_6798::<0x14387f1d8, 0x14243b0b0, 0x140cc0e80, 0x90, 0x140cc10f0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ca2b70
    Replacement::draft_throws(0x140ca2bf0, "content_140ca2bf0", tpl_6805::<0x14387f44c, 0x14387f1d8, 0x1401e9770, 0x140cc1260, 0x90, 0x140cc1300, 0x1401e9590> as *const (), Throws::Discard), // sub_140ca2bf0
    Replacement::draft_throws(0x140ca2c60, "content_140ca2c60", fn_140ca2c60 as *const (), Throws::Discard), // sub_140ca2c60
    Replacement::draft_throws(0x140ca2dd0, "content_140ca2dd0", fn_140ca2dd0 as *const (), Throws::Discard), // sub_140ca2dd0
    Replacement::draft_throws(0x140ca2f00, "content_140ca2f00", fn_140ca2f00 as *const (), Throws::Discard), // sub_140ca2f00
    Replacement::draft_throws(0x140ca3200, "content_140ca3200", fn_140ca3200 as *const (), Throws::Caught), // sub_140ca3200
    Replacement::draft_throws(0x140ca33c0, "content_140ca33c0", fn_140ca33c0 as *const (), Throws::Caught), // sub_140ca33c0
    Replacement::draft_throws(0x140ca3580, "content_140ca3580", fn_140ca3580 as *const (), Throws::Discard), // sub_140ca3580
    Replacement::draft_throws(0x140ca3700, "content_140ca3700", fn_140ca3700 as *const (), Throws::Discard), // sub_140ca3700
    Replacement::draft_throws(0x140ca3850, "content_140ca3850", fn_140ca3850 as *const (), Throws::Caught), // sub_140ca3850
    Replacement::draft_throws(0x140ca3970, "content_140ca3970", fn_140ca3970 as *const (), Throws::Caught), // sub_140ca3970
    Replacement::draft_throws(0x140ca3ba0, "content_140ca3ba0", fn_140ca3ba0 as *const (), Throws::Caught), // sub_140ca3ba0
    Replacement::draft_throws(0x140ca3d00, "content_140ca3d00", fn_140ca3d00 as *const (), Throws::Discard), // sub_140ca3d00
    Replacement::draft_throws(0x140ca3e50, "content_140ca3e50", fn_140ca3e50 as *const (), Throws::Discard), // sub_140ca3e50
    Replacement::draft_throws(0x140ca3f20, "content_140ca3f20", fn_140ca3f20 as *const (), Throws::Caught), // sub_140ca3f20
    Replacement::draft_throws(0x140ca40e0, "content_140ca40e0", fn_140ca40e0 as *const (), Throws::Caught), // sub_140ca40e0
    Replacement::draft_throws(0x140ca42a0, "content_140ca42a0", fn_140ca42a0 as *const (), Throws::Caught), // sub_140ca42a0
    Replacement::draft_throws(0x140ca4490, "content_140ca4490", fn_140ca4490 as *const (), Throws::Discard), // sub_140ca4490
    Replacement::draft_throws(0x140ca4600, "content_140ca4600", fn_140ca4600 as *const (), Throws::Discard), // sub_140ca4600
    Replacement::draft_throws(0x140ca46f0, "content_140ca46f0", fn_140ca46f0 as *const (), Throws::Discard), // sub_140ca46f0
    Replacement::draft(0x140ca4860, "content_140ca4860", fn_140ca4860 as *const ()), // sub_140ca4860
    Replacement::draft(0x140ca4a20, "content_140ca4a20", fn_140ca4a20 as *const ()), // sub_140ca4a20
    Replacement::draft(0x140ca4aa0, "content_140ca4aa0", fn_140ca4aa0 as *const ()), // sub_140ca4aa0
    Replacement::draft(0x140ca4b70, "content_140ca4b70", fn_140ca4b70 as *const ()), // sub_140ca4b70
    Replacement::draft(0x140ca4be0, "content_140ca4be0", fn_140ca4be0 as *const ()), // sub_140ca4be0
    Replacement::draft(0x140ca4cc0, "content_140ca4cc0", fn_140ca4cc0 as *const ()), // sub_140ca4cc0
    Replacement::draft(0x140ca4d70, "content_140ca4d70", fn_140ca4d70 as *const ()), // sub_140ca4d70
    Replacement::draft(0x140ca4dd0, "content_140ca4dd0", fn_140ca4dd0 as *const ()), // sub_140ca4dd0
    Replacement::draft_throws(0x140ca4f10, "content_140ca4f10", fn_140ca4f10 as *const (), Throws::Caught), // sub_140ca4f10
    Replacement::draft_throws(0x140ca5200, "content_140ca5200", fn_140ca5200 as *const (), Throws::Caught), // sub_140ca5200
    Replacement::draft_throws(0x140ca54f0, "content_140ca54f0", fn_140ca54f0 as *const (), Throws::Caught), // sub_140ca54f0
    Replacement::draft_throws(0x140ca5e80, "content_140ca5e80", fn_140ca5e80 as *const (), Throws::Caught), // sub_140ca5e80
    Replacement::draft_throws(0x140ca6120, "content_140ca6120", fn_140ca6120 as *const (), Throws::Caught), // sub_140ca6120
    Replacement::draft_throws(0x140ca69f0, "content_140ca69f0", fn_140ca69f0 as *const (), Throws::Caught), // sub_140ca69f0
    Replacement::draft_throws(0x140ca6d50, "content_140ca6d50", fn_140ca6d50 as *const (), Throws::Caught), // sub_140ca6d50
    Replacement::draft_throws(0x140ca7640, "content_140ca7640", fn_140ca7640 as *const (), Throws::Caught), // sub_140ca7640
    Replacement::draft_throws(0x140ca7a10, "content_140ca7a10", fn_140ca7a10 as *const (), Throws::Caught), // sub_140ca7a10
    Replacement::draft_throws(0x140ca7dd0, "content_140ca7dd0", fn_140ca7dd0 as *const (), Throws::Caught), // sub_140ca7dd0
    Replacement::draft_throws(0x140ca8120, "content_140ca8120", fn_140ca8120 as *const (), Throws::Caught), // sub_140ca8120
    Replacement::draft_throws(0x140ca8450, "content_140ca8450", fn_140ca8450 as *const (), Throws::Caught), // sub_140ca8450
    Replacement::draft_throws(0x140ca8900, "content_140ca8900", fn_140ca8900 as *const (), Throws::Caught), // sub_140ca8900
    Replacement::draft_throws(0x140ca8d30, "content_140ca8d30", fn_140ca8d30 as *const (), Throws::Caught), // sub_140ca8d30
    Replacement::draft_throws(0x140ca9160, "content_140ca9160", fn_140ca9160 as *const (), Throws::Caught), // sub_140ca9160
    Replacement::draft_throws(0x140ca9830, "content_140ca9830", fn_140ca9830 as *const (), Throws::Caught), // sub_140ca9830
    Replacement::draft_throws(0x140ca9e40, "content_140ca9e40", fn_140ca9e40 as *const (), Throws::Caught), // sub_140ca9e40
    Replacement::draft_throws(0x140caa410, "content_140caa410", fn_140caa410 as *const (), Throws::Caught), // sub_140caa410
    Replacement::draft_throws(0x140caa760, "content_140caa760", fn_140caa760 as *const (), Throws::Caught), // sub_140caa760
    Replacement::draft_throws(0x140caaa40, "content_140caaa40", fn_140caaa40 as *const (), Throws::Caught), // sub_140caaa40
    Replacement::draft_throws(0x140caad20, "content_140caad20", fn_140caad20 as *const (), Throws::Caught), // sub_140caad20
    Replacement::draft_throws(0x140cab460, "content_140cab460", fn_140cab460 as *const (), Throws::Caught), // sub_140cab460
    Replacement::draft_throws(0x140cabad0, "content_140cabad0", fn_140cabad0 as *const (), Throws::Caught), // sub_140cabad0
    Replacement::draft_throws(0x140cac120, "content_140cac120", fn_140cac120 as *const (), Throws::Caught), // sub_140cac120
    Replacement::draft_throws(0x140cac220, "content_140cac220", fn_140cac220 as *const (), Throws::Caught), // sub_140cac220
    Replacement::draft_throws(0x140cac320, "content_140cac320", fn_140cac320 as *const (), Throws::Caught), // sub_140cac320
    Replacement::draft_throws(0x140cac420, "content_140cac420", fn_140cac420 as *const (), Throws::Caught), // sub_140cac420
    Replacement::draft_throws(0x140cac710, "content_140cac710", fn_140cac710 as *const (), Throws::Caught), // sub_140cac710
    Replacement::draft_throws(0x140caca00, "content_140caca00", fn_140caca00 as *const (), Throws::Caught), // sub_140caca00
    Replacement::draft_throws(0x140cacce0, "content_140cacce0", fn_140cacce0 as *const (), Throws::Caught), // sub_140cacce0
    Replacement::draft_throws(0x140cacf80, "content_140cacf80", fn_140cacf80 as *const (), Throws::Caught), // sub_140cacf80
    Replacement::draft_throws(0x140cad260, "content_140cad260", fn_140cad260 as *const (), Throws::Caught), // sub_140cad260
    Replacement::draft_throws(0x140cad510, "content_140cad510", fn_140cad510 as *const (), Throws::Caught), // sub_140cad510
    Replacement::draft_throws(0x140cad7c0, "content_140cad7c0", fn_140cad7c0 as *const (), Throws::Caught), // sub_140cad7c0
    Replacement::draft_throws(0x140cae000, "content_140cae000", fn_140cae000 as *const (), Throws::Caught), // sub_140cae000
    Replacement::draft_throws(0x140cae6b0, "content_140cae6b0", fn_140cae6b0 as *const (), Throws::Caught), // sub_140cae6b0
    Replacement::draft_throws(0x140caedd0, "content_140caedd0", fn_140caedd0 as *const (), Throws::Caught), // sub_140caedd0
    Replacement::draft_throws(0x140caf380, "content_140caf380", fn_140caf380 as *const (), Throws::Caught), // sub_140caf380
    Replacement::draft_throws(0x140cafaa0, "content_140cafaa0", fn_140cafaa0 as *const (), Throws::Caught), // sub_140cafaa0
    Replacement::draft_throws(0x140cb0070, "content_140cb0070", fn_140cb0070 as *const (), Throws::Caught), // sub_140cb0070
    Replacement::draft_throws(0x140cb0330, "content_140cb0330", fn_140cb0330 as *const (), Throws::Caught), // sub_140cb0330
    Replacement::draft_throws(0x140cb05c0, "content_140cb05c0", fn_140cb05c0 as *const (), Throws::Caught), // sub_140cb05c0
    Replacement::draft_throws(0x140cb0890, "content_140cb0890", fn_140cb0890 as *const (), Throws::Caught), // sub_140cb0890
    Replacement::draft_throws(0x140cb0b30, "content_140cb0b30", fn_140cb0b30 as *const (), Throws::Caught), // sub_140cb0b30
    Replacement::draft_throws(0x140cb0db0, "content_140cb0db0", fn_140cb0db0 as *const (), Throws::Caught), // sub_140cb0db0
    Replacement::draft_throws(0x140cb1020, "content_140cb1020", fn_140cb1020 as *const (), Throws::Caught), // sub_140cb1020
    Replacement::draft_throws(0x140cb1260, "content_140cb1260", fn_140cb1260 as *const (), Throws::Caught), // sub_140cb1260
    Replacement::draft_throws(0x140cb14a0, "content_140cb14a0", fn_140cb14a0 as *const (), Throws::Caught), // sub_140cb14a0
    Replacement::draft_throws(0x140cb1830, "content_140cb1830", fn_140cb1830 as *const (), Throws::Caught), // sub_140cb1830
    Replacement::draft_throws(0x140cb1930, "content_140cb1930", fn_140cb1930 as *const (), Throws::Caught), // sub_140cb1930
    Replacement::draft_throws(0x140cb1a30, "content_140cb1a30", fn_140cb1a30 as *const (), Throws::Caught), // sub_140cb1a30
    Replacement::draft_throws(0x140cb1b30, "content_140cb1b30", fn_140cb1b30 as *const (), Throws::Caught), // sub_140cb1b30
    Replacement::draft_throws(0x140cb1e40, "content_140cb1e40", fn_140cb1e40 as *const (), Throws::Caught), // sub_140cb1e40
    Replacement::draft_throws(0x140cb2120, "content_140cb2120", fn_140cb2120 as *const (), Throws::Caught), // sub_140cb2120
    Replacement::draft_throws(0x140cb2400, "content_140cb2400", fn_140cb2400 as *const (), Throws::Caught), // sub_140cb2400
    Replacement::draft_throws(0x140cb26f0, "content_140cb26f0", fn_140cb26f0 as *const (), Throws::Caught), // sub_140cb26f0
    Replacement::draft_throws(0x140cb29e0, "content_140cb29e0", fn_140cb29e0 as *const (), Throws::Caught), // sub_140cb29e0
    Replacement::draft_throws(0x140cb2ca0, "content_140cb2ca0", fn_140cb2ca0 as *const (), Throws::Caught), // sub_140cb2ca0
    Replacement::draft_throws(0x140cb2f30, "content_140cb2f30", fn_140cb2f30 as *const (), Throws::Caught), // sub_140cb2f30
    Replacement::draft_throws(0x140cb3200, "content_140cb3200", fn_140cb3200 as *const (), Throws::Caught), // sub_140cb3200
    Replacement::draft_throws(0x140cb35b0, "content_140cb35b0", fn_140cb35b0 as *const (), Throws::Caught), // sub_140cb35b0
    Replacement::draft_throws(0x140cb38f0, "content_140cb38f0", fn_140cb38f0 as *const (), Throws::Caught), // sub_140cb38f0
    Replacement::draft_throws(0x140cb3c20, "content_140cb3c20", fn_140cb3c20 as *const (), Throws::Caught), // sub_140cb3c20
    Replacement::draft_throws(0x140cb4200, "content_140cb4200", fn_140cb4200 as *const (), Throws::Caught), // sub_140cb4200
    Replacement::draft_throws(0x140cb4780, "content_140cb4780", fn_140cb4780 as *const (), Throws::Caught), // sub_140cb4780
    Replacement::draft_throws(0x140cb4ce0, "content_140cb4ce0", fn_140cb4ce0 as *const (), Throws::Caught), // sub_140cb4ce0
    Replacement::draft_throws(0x140cb4ec0, "content_140cb4ec0", fn_140cb4ec0 as *const (), Throws::Caught), // sub_140cb4ec0
    Replacement::draft_throws(0x140cb50a0, "content_140cb50a0", fn_140cb50a0 as *const (), Throws::Caught), // sub_140cb50a0
    Replacement::draft_throws(0x140cb52d0, "content_140cb52d0", fn_140cb52d0 as *const (), Throws::Caught), // sub_140cb52d0
    Replacement::draft_throws(0x140cb5560, "content_140cb5560", fn_140cb5560 as *const (), Throws::Caught), // sub_140cb5560
    Replacement::draft_throws(0x140cb57c0, "content_140cb57c0", fn_140cb57c0 as *const (), Throws::Caught), // sub_140cb57c0
    Replacement::draft_throws(0x140cb5a20, "content_140cb5a20", fn_140cb5a20 as *const (), Throws::Caught), // sub_140cb5a20
    Replacement::draft_throws(0x140cb5b20, "content_140cb5b20", fn_140cb5b20 as *const (), Throws::Caught), // sub_140cb5b20
    Replacement::draft_throws(0x140cb5c20, "content_140cb5c20", fn_140cb5c20 as *const (), Throws::Caught), // sub_140cb5c20
    Replacement::draft_throws(0x140cb5d20, "content_140cb5d20", fn_140cb5d20 as *const (), Throws::Caught), // sub_140cb5d20
    Replacement::draft_throws(0x140cb5fe0, "content_140cb5fe0", fn_140cb5fe0 as *const (), Throws::Caught), // sub_140cb5fe0
    Replacement::draft_throws(0x140cb6270, "content_140cb6270", fn_140cb6270 as *const (), Throws::Caught), // sub_140cb6270
    Replacement::draft_throws(0x140cb64f0, "content_140cb64f0", fn_140cb64f0 as *const (), Throws::Caught), // sub_140cb64f0
    Replacement::draft_throws(0x140cb65d0, "content_140cb65d0", fn_140cb65d0 as *const (), Throws::Caught), // sub_140cb65d0
    Replacement::draft_throws(0x140cb66b0, "content_140cb66b0", fn_140cb66b0 as *const (), Throws::Caught), // sub_140cb66b0
    Replacement::draft_throws(0x140cb6790, "content_140cb6790", fn_140cb6790 as *const (), Throws::Caught), // sub_140cb6790
    Replacement::draft_throws(0x140cb6a10, "content_140cb6a10", fn_140cb6a10 as *const (), Throws::Caught), // sub_140cb6a10
    Replacement::draft_throws(0x140cb6c60, "content_140cb6c60", fn_140cb6c60 as *const (), Throws::Caught), // sub_140cb6c60
    Replacement::draft_throws(0x140cb6eb0, "content_140cb6eb0", fn_140cb6eb0 as *const (), Throws::Caught), // sub_140cb6eb0
    Replacement::draft_throws(0x140cb71e0, "content_140cb71e0", fn_140cb71e0 as *const (), Throws::Caught), // sub_140cb71e0
    Replacement::draft_throws(0x140cb74b0, "content_140cb74b0", fn_140cb74b0 as *const (), Throws::Caught), // sub_140cb74b0
    Replacement::draft_throws(0x140cb7770, "content_140cb7770", fn_140cb7770 as *const (), Throws::Caught), // sub_140cb7770
    Replacement::draft_throws(0x140cb78b0, "content_140cb78b0", fn_140cb78b0 as *const (), Throws::Caught), // sub_140cb78b0
    Replacement::draft_throws(0x140cb79f0, "content_140cb79f0", fn_140cb79f0 as *const (), Throws::Caught), // sub_140cb79f0
    Replacement::draft_throws(0x140cb7b30, "content_140cb7b30", fn_140cb7b30 as *const (), Throws::Caught), // sub_140cb7b30
    Replacement::draft_throws(0x140cb7c70, "content_140cb7c70", fn_140cb7c70 as *const (), Throws::Caught), // sub_140cb7c70
    Replacement::draft_throws(0x140cb7db0, "content_140cb7db0", fn_140cb7db0 as *const (), Throws::Caught), // sub_140cb7db0
    Replacement::draft_throws(0x140cb7ef0, "content_140cb7ef0", fn_140cb7ef0 as *const (), Throws::Caught), // sub_140cb7ef0
    Replacement::draft_throws(0x140cb81e0, "content_140cb81e0", fn_140cb81e0 as *const (), Throws::Caught), // sub_140cb81e0
    Replacement::draft_throws(0x140cb86e0, "content_140cb86e0", fn_140cb86e0 as *const (), Throws::Caught), // sub_140cb86e0
    Replacement::draft_throws(0x140cb87d0, "content_140cb87d0", fn_140cb87d0 as *const (), Throws::Caught), // sub_140cb87d0
    Replacement::draft_throws(0x140cb8aa0, "content_140cb8aa0", fn_140cb8aa0 as *const (), Throws::Caught), // sub_140cb8aa0
    Replacement::draft_throws(0x140cb8f10, "content_140cb8f10", fn_140cb8f10 as *const (), Throws::Caught), // sub_140cb8f10
    Replacement::draft_throws(0x140cb9000, "content_140cb9000", fn_140cb9000 as *const (), Throws::Caught), // sub_140cb9000
    Replacement::draft_throws(0x140cb92c0, "content_140cb92c0", fn_140cb92c0 as *const (), Throws::Caught), // sub_140cb92c0
    Replacement::draft_throws(0x140cb9720, "content_140cb9720", fn_140cb9720 as *const (), Throws::Caught), // sub_140cb9720
    Replacement::draft_throws(0x140cb9850, "content_140cb9850", fn_140cb9850 as *const (), Throws::Caught), // sub_140cb9850
    Replacement::draft_throws(0x140cb9b40, "content_140cb9b40", fn_140cb9b40 as *const (), Throws::Caught), // sub_140cb9b40
    Replacement::draft_throws(0x140cb9e30, "content_140cb9e30", fn_140cb9e30 as *const (), Throws::Caught), // sub_140cb9e30
    Replacement::draft_throws(0x140cba0f0, "content_140cba0f0", fn_140cba0f0 as *const (), Throws::Caught), // sub_140cba0f0
    Replacement::draft_throws(0x140cba380, "content_140cba380", fn_140cba380 as *const (), Throws::Caught), // sub_140cba380
    Replacement::draft(0x140cba650, "content_140cba650", fn_140cba650 as *const ()), // sub_140cba650
    Replacement::draft(0x140cba6f0, "content_140cba6f0", fn_140cba6f0 as *const ()), // sub_140cba6f0
    Replacement::draft(0x140cba880, "content_140cba880", fn_140cba880 as *const ()), // sub_140cba880
    Replacement::draft_throws(0x140cba9a0, "content_140cba9a0", fn_140cba9a0 as *const (), Throws::Caught), // sub_140cba9a0
    Replacement::draft_throws(0x140cbaa70, "content_140cbaa70", fn_140cbaa70 as *const (), Throws::Caught), // sub_140cbaa70
    Replacement::draft(0x140cbab40, "content_140cbab40", fn_140cbab40 as *const ()), // sub_140cbab40
    Replacement::draft_throws(0x140cbac20, "content_140cbac20", fn_140cbac20 as *const (), Throws::Caught), // sub_140cbac20
    Replacement::draft(0x140cbacf0, "content_140cbacf0", fn_140cbacf0 as *const ()), // sub_140cbacf0
    Replacement::draft_throws(0x140cbadc0, "content_140cbadc0", fn_140cbadc0 as *const (), Throws::Caught), // sub_140cbadc0
    Replacement::draft(0x140cbae90, "content_140cbae90", fn_140cbae90 as *const ()), // sub_140cbae90
    Replacement::draft(0x140cbaf70, "content_140cbaf70", fn_140cbaf70 as *const ()), // sub_140cbaf70
    Replacement::draft_throws(0x140cbb180, "content_140cbb180", fn_140cbb180 as *const (), Throws::Caught), // sub_140cbb180
    Replacement::draft(0x140cbb250, "content_140cbb250", tpl_15775::<0x1438771b0, 0x1432dad20, 0x1432dace0, 0x10, 0x1432dacf8, 0x1, 0x1401de410, 0x1426cdcb0, 0x140ccb1a0> as *const ()), // sub_140cbb250
    Replacement::draft(0x140cbb300, "content_140cbb300", tpl_15775::<0x143877350, 0x1432dad20, 0x1432dace0, 0x10, 0x1432dacf8, 0x1, 0x1401de410, 0x1432dab10, 0x140ccb1a0> as *const ()), // sub_140cbb300
    Replacement::draft_throws(0x140cbb3b0, "content_140cbb3b0", fn_140cbb3b0 as *const (), Throws::Caught), // sub_140cbb3b0
    Replacement::draft_throws(0x140cbb490, "content_140cbb490", fn_140cbb490 as *const (), Throws::Caught), // sub_140cbb490
    Replacement::draft_throws(0x140cbb570, "content_140cbb570", fn_140cbb570 as *const (), Throws::Caught), // sub_140cbb570
    Replacement::draft_throws(0x140cbb650, "content_140cbb650", fn_140cbb650 as *const (), Throws::Caught), // sub_140cbb650
    Replacement::draft(0x140cbb760, "content_140cbb760", tpl_15778::<0x14387f150, 0x14243b0b0, 0x1426d4580> as *const ()), // sub_140cbb760
    Replacement::draft_throws(0x140cbb930, "content_140cbb930", tpl_838::<0x140cc17e0> as *const (), Throws::Discard), // sub_140cbb930
    Replacement::draft_throws(0x140cbb950, "content_140cbb950", fn_140cbb950 as *const (), Throws::Caught), // sub_140cbb950
    Replacement::draft(0x140cbba20, "content_140cbba20", fn_140cbba20 as *const ()), // sub_140cbba20
    Replacement::draft_throws(0x140cbbc10, "content_140cbbc10", fn_140cbbc10 as *const (), Throws::Caught), // sub_140cbbc10
    Replacement::draft(0x140cbbd40, "content_140cbbd40", fn_140cbbd40 as *const ()), // sub_140cbbd40
    Replacement::draft(0x140cbbe10, "content_140cbbe10", tpl_15778::<0x14387f4e0, 0x14243b0b0, 0x1426d4580> as *const ()), // sub_140cbbe10
    Replacement::draft_throws(0x140cbc000, "content_140cbc000", fn_140cbc000 as *const (), Throws::Caught), // sub_140cbc000
    Replacement::draft_throws(0x140cbc0d0, "content_140cbc0d0", fn_140cbc0d0 as *const (), Throws::Caught), // sub_140cbc0d0
    Replacement::draft_throws(0x140cbc3b0, "content_140cbc3b0", fn_140cbc3b0 as *const (), Throws::Caught), // sub_140cbc3b0
    Replacement::draft_throws(0x140cbc810, "content_140cbc810", fn_140cbc810 as *const (), Throws::Caught), // sub_140cbc810
    Replacement::draft_throws(0x140cbcac0, "content_140cbcac0", fn_140cbcac0 as *const (), Throws::Caught), // sub_140cbcac0
    Replacement::draft_throws(0x140cbcef0, "content_140cbcef0", fn_140cbcef0 as *const (), Throws::Caught), // sub_140cbcef0
    Replacement::draft_throws(0x140cbd190, "content_140cbd190", fn_140cbd190 as *const (), Throws::Caught), // sub_140cbd190
    Replacement::draft_throws(0x140cbd5b0, "content_140cbd5b0", fn_140cbd5b0 as *const (), Throws::Caught), // sub_140cbd5b0
    Replacement::draft_throws(0x140cbda50, "content_140cbda50", fn_140cbda50 as *const (), Throws::Caught), // sub_140cbda50
    Replacement::draft_throws(0x140cbde80, "content_140cbde80", fn_140cbde80 as *const (), Throws::Caught), // sub_140cbde80
    Replacement::draft_throws(0x140cbe2a0, "content_140cbe2a0", fn_140cbe2a0 as *const (), Throws::Caught), // sub_140cbe2a0
    Replacement::draft_throws(0x140cbe6c0, "content_140cbe6c0", fn_140cbe6c0 as *const (), Throws::Caught), // sub_140cbe6c0
    Replacement::draft_throws(0x140cbeab0, "content_140cbeab0", fn_140cbeab0 as *const (), Throws::Caught), // sub_140cbeab0
    Replacement::draft_throws(0x140cbee90, "content_140cbee90", fn_140cbee90 as *const (), Throws::Caught), // sub_140cbee90
    Replacement::draft_throws(0x140cbf170, "content_140cbf170", fn_140cbf170 as *const (), Throws::Caught), // sub_140cbf170
    Replacement::draft_throws(0x140cbf420, "content_140cbf420", fn_140cbf420 as *const (), Throws::Caught), // sub_140cbf420
    Replacement::draft(0x140cbf6c0, "content_140cbf6c0", tpl_10075::<0x14387f338, 0x14243b0b0, 0x1406dd2c0, 0x18, 0x30, 0x48, 0x60, 0x78, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cbf6c0
    Replacement::draft(0x140cbf770, "content_140cbf770", fn_140cbf770 as *const ()), // sub_140cbf770
    Replacement::draft_throws(0x140cbf7f0, "content_140cbf7f0", fn_140cbf7f0 as *const (), Throws::Discard), // sub_140cbf7f0
    Replacement::draft(0x140cbfa20, "content_140cbfa20", fn_140cbfa20 as *const ()), // sub_140cbfa20
    Replacement::draft_throws(0x140cbfb90, "content_140cbfb90", fn_140cbfb90 as *const (), Throws::Discard), // sub_140cbfb90
    Replacement::draft(0x140cbfcf0, "content_140cbfcf0", fn_140cbfcf0 as *const ()), // sub_140cbfcf0
    Replacement::draft(0x140cbfde0, "content_140cbfde0", fn_140cbfde0 as *const ()), // sub_140cbfde0
    Replacement::draft(0x140cbfea0, "content_140cbfea0", fn_140cbfea0 as *const ()), // sub_140cbfea0
    Replacement::draft_throws(0x140cbff30, "content_140cbff30", fn_140cbff30 as *const (), Throws::Discard), // sub_140cbff30
    Replacement::draft_throws(0x140cc0150, "content_140cc0150", fn_140cc0150 as *const (), Throws::Discard), // sub_140cc0150
    Replacement::draft(0x140cc02a0, "content_140cc02a0", fn_140cc02a0 as *const ()), // sub_140cc02a0
    Replacement::draft_throws(0x140cc0330, "content_140cc0330", fn_140cc0330 as *const (), Throws::Caught), // sub_140cc0330
    Replacement::draft_throws(0x140cc0470, "content_140cc0470", fn_140cc0470 as *const (), Throws::Caught), // sub_140cc0470
    Replacement::draft_throws(0x140cc05b0, "content_140cc05b0", fn_140cc05b0 as *const (), Throws::Caught), // sub_140cc05b0
    Replacement::draft_throws(0x140cc06f0, "content_140cc06f0", fn_140cc06f0 as *const (), Throws::Caught), // sub_140cc06f0
    Replacement::draft_throws(0x140cc0830, "content_140cc0830", fn_140cc0830 as *const (), Throws::Caught), // sub_140cc0830
    Replacement::draft_throws(0x140cc0970, "content_140cc0970", fn_140cc0970 as *const (), Throws::Caught), // sub_140cc0970
    Replacement::draft(0x140cc0ab0, "content_140cc0ab0", fn_140cc0ab0 as *const ()), // sub_140cc0ab0
    Replacement::draft(0x140cc0d20, "content_140cc0d20", fn_140cc0d20 as *const ()), // sub_140cc0d20
    Replacement::draft(0x140cc0e80, "content_140cc0e80", fn_140cc0e80 as *const ()), // sub_140cc0e80
    Replacement::draft(0x140cc10f0, "content_140cc10f0", fn_140cc10f0 as *const ()), // sub_140cc10f0
    Replacement::draft_throws(0x140cc1260, "content_140cc1260", fn_140cc1260 as *const (), Throws::Discard), // sub_140cc1260
    Replacement::draft_throws(0x140cc1300, "content_140cc1300", tpl_15302::<0x14387f178, 0x14387f2c8, 0x1401e9770, 0x1406deba0, 0x18, 0x30, 0x1401e9590> as *const (), Throws::Discard), // sub_140cc1300
    Replacement::draft_throws(0x140cc1380, "content_140cc1380", fn_140cc1380 as *const (), Throws::Caught), // sub_140cc1380
    Replacement::draft_throws(0x140cc17e0, "content_140cc17e0", fn_140cc17e0 as *const (), Throws::Caught), // sub_140cc17e0
    Replacement::draft_throws(0x140cc1c10, "content_140cc1c10", fn_140cc1c10 as *const (), Throws::Caught), // sub_140cc1c10
    Replacement::draft_throws(0x140cc2040, "content_140cc2040", fn_140cc2040 as *const (), Throws::Caught), // sub_140cc2040
    Replacement::draft_throws(0x140cc2310, "content_140cc2310", fn_140cc2310 as *const (), Throws::Caught), // sub_140cc2310
    Replacement::draft_throws(0x140cc25b0, "content_140cc25b0", fn_140cc25b0 as *const (), Throws::Caught), // sub_140cc25b0
    Replacement::draft_throws(0x140cc2840, "content_140cc2840", fn_140cc2840 as *const (), Throws::Caught), // sub_140cc2840
    Replacement::draft_throws(0x140cc2bc0, "content_140cc2bc0", fn_140cc2bc0 as *const (), Throws::Caught), // sub_140cc2bc0
    Replacement::draft_throws(0x140cc2f10, "content_140cc2f10", fn_140cc2f10 as *const (), Throws::Caught), // sub_140cc2f10
    Replacement::draft_throws(0x140cc3250, "content_140cc3250", fn_140cc3250 as *const (), Throws::Caught), // sub_140cc3250
    Replacement::draft_throws(0x140cc36b0, "content_140cc36b0", fn_140cc36b0 as *const (), Throws::Caught), // sub_140cc36b0
    Replacement::draft_throws(0x140cc3ae0, "content_140cc3ae0", fn_140cc3ae0 as *const (), Throws::Caught), // sub_140cc3ae0
    Replacement::draft_throws(0x140cc3f10, "content_140cc3f10", fn_140cc3f10 as *const (), Throws::Discard), // sub_140cc3f10
    Replacement::draft(0x140cc40a0, "content_140cc40a0", fn_140cc40a0 as *const ()), // sub_140cc40a0
    Replacement::draft_throws(0x140cc4170, "content_140cc4170", fn_140cc4170 as *const (), Throws::Discard), // sub_140cc4170
    Replacement::draft(0x140cc42b0, "content_140cc42b0", fn_140cc42b0 as *const ()), // sub_140cc42b0
    Replacement::draft(0x140cc4340, "content_140cc4340", fn_140cc4340 as *const ()), // sub_140cc4340
    Replacement::draft(0x140cc4430, "content_140cc4430", fn_140cc4430 as *const ()), // sub_140cc4430
    Replacement::draft_throws(0x140cc4540, "content_140cc4540", fn_140cc4540 as *const (), Throws::Discard), // sub_140cc4540
    Replacement::draft_throws(0x140cc46d0, "content_140cc46d0", fn_140cc46d0 as *const (), Throws::Discard), // sub_140cc46d0
    Replacement::draft(0x140cc4820, "content_140cc4820", fn_140cc4820 as *const ()), // sub_140cc4820
    Replacement::draft(0x140cc48a0, "content_140cc48a0", fn_140cc48a0 as *const ()), // sub_140cc48a0
    Replacement::draft_throws(0x140cc4960, "content_140cc4960", fn_140cc4960 as *const (), Throws::Caught), // sub_140cc4960
    Replacement::draft_throws(0x140cc4e30, "content_140cc4e30", fn_140cc4e30 as *const (), Throws::Caught), // sub_140cc4e30
    Replacement::draft_throws(0x140cc5300, "content_140cc5300", fn_140cc5300 as *const (), Throws::Caught), // sub_140cc5300
    Replacement::draft_throws(0x140cc5790, "content_140cc5790", fn_140cc5790 as *const (), Throws::Caught), // sub_140cc5790
    Replacement::draft_throws(0x140cc5c20, "content_140cc5c20", fn_140cc5c20 as *const (), Throws::Caught), // sub_140cc5c20
    Replacement::draft_throws(0x140cc60b0, "content_140cc60b0", fn_140cc60b0 as *const (), Throws::Caught), // sub_140cc60b0
    Replacement::draft(0x140cc6540, "content_140cc6540", fn_140cc6540 as *const ()), // sub_140cc6540
    Replacement::draft(0x140cc6630, "content_140cc6630", fn_140cc6630 as *const ()), // sub_140cc6630
    Replacement::draft(0x140cc66e0, "content_140cc66e0", fn_140cc66e0 as *const ()), // sub_140cc66e0
    Replacement::draft(0x140cc67e0, "content_140cc67e0", fn_140cc67e0 as *const ()), // sub_140cc67e0
    Replacement::draft_throws(0x140cc68a0, "content_140cc68a0", fn_140cc68a0 as *const (), Throws::Discard), // sub_140cc68a0
    Replacement::draft_throws(0x140cc6960, "content_140cc6960", fn_140cc6960 as *const (), Throws::Discard), // sub_140cc6960
    Replacement::draft_throws(0x140cc6a20, "content_140cc6a20", fn_140cc6a20 as *const (), Throws::Caught), // sub_140cc6a20
    Replacement::draft_throws(0x140cc6c10, "content_140cc6c10", fn_140cc6c10 as *const (), Throws::Caught), // sub_140cc6c10
    Replacement::draft_throws(0x140cc6e00, "content_140cc6e00", fn_140cc6e00 as *const (), Throws::Discard), // sub_140cc6e00
    Replacement::draft(0x140cc6f90, "content_140cc6f90", fn_140cc6f90 as *const ()), // sub_140cc6f90
    Replacement::draft(0x140cc7030, "content_140cc7030", fn_140cc7030 as *const ()), // sub_140cc7030
    Replacement::draft_throws(0x140cc7120, "content_140cc7120", fn_140cc7120 as *const (), Throws::Discard), // sub_140cc7120
    Replacement::draft_throws(0x140cc7270, "content_140cc7270", fn_140cc7270 as *const (), Throws::Caught), // sub_140cc7270
    Replacement::draft_throws(0x140cc7740, "content_140cc7740", fn_140cc7740 as *const (), Throws::Caught), // sub_140cc7740
    Replacement::draft_throws(0x140cc7bd0, "content_140cc7bd0", fn_140cc7bd0 as *const (), Throws::Caught), // sub_140cc7bd0
    Replacement::draft(0x140cc8060, "content_140cc8060", fn_140cc8060 as *const ()), // sub_140cc8060
    Replacement::draft(0x140cc8130, "content_140cc8130", fn_140cc8130 as *const ()), // sub_140cc8130
    Replacement::draft_throws(0x140cc8210, "content_140cc8210", fn_140cc8210 as *const (), Throws::Discard), // sub_140cc8210
    Replacement::draft_throws(0x140cc8290, "content_140cc8290", fn_140cc8290 as *const (), Throws::Caught), // sub_140cc8290
    Replacement::draft_throws(0x140cc8480, "content_140cc8480", fn_140cc8480 as *const (), Throws::Discard), // sub_140cc8480
    Replacement::draft(0x140cc8500, "content_140cc8500", fn_140cc8500 as *const ()), // sub_140cc8500
    Replacement::draft(0x140cc8570, "content_140cc8570", fn_140cc8570 as *const ()), // sub_140cc8570
    Replacement::draft_throws(0x140cc85e0, "content_140cc85e0", fn_140cc85e0 as *const (), Throws::Discard), // sub_140cc85e0
    Replacement::draft_throws(0x140cc8670, "content_140cc8670", fn_140cc8670 as *const (), Throws::Caught), // sub_140cc8670
    Replacement::draft_throws(0x140cc8b40, "content_140cc8b40", fn_140cc8b40 as *const (), Throws::Caught), // sub_140cc8b40
    Replacement::draft_throws(0x140cc8fc0, "content_140cc8fc0", fn_140cc8fc0 as *const (), Throws::Caught), // sub_140cc8fc0
    Replacement::draft(0x140cc9440, "content_140cc9440", fn_140cc9440 as *const ()), // sub_140cc9440
    Replacement::draft(0x140cc94b0, "content_140cc94b0", fn_140cc94b0 as *const ()), // sub_140cc94b0
    Replacement::draft_throws(0x140cc9530, "content_140cc9530", fn_140cc9530 as *const (), Throws::Discard), // sub_140cc9530
    Replacement::draft_throws(0x140cc95a0, "content_140cc95a0", fn_140cc95a0 as *const (), Throws::Caught), // sub_140cc95a0
    Replacement::draft_throws(0x140cc97f0, "content_140cc97f0", fn_140cc97f0 as *const (), Throws::Discard), // sub_140cc97f0
    Replacement::draft(0x140cc9970, "content_140cc9970", fn_140cc9970 as *const ()), // sub_140cc9970
    Replacement::draft(0x140cc9a60, "content_140cc9a60", fn_140cc9a60 as *const ()), // sub_140cc9a60
    Replacement::draft_throws(0x140cc9b70, "content_140cc9b70", fn_140cc9b70 as *const (), Throws::Discard), // sub_140cc9b70
    Replacement::draft(0x140cc9d10, "content_140cc9d10", fn_140cc9d10 as *const ()), // sub_140cc9d10
    Replacement::draft_throws(0x140cc9e50, "content_140cc9e50", fn_140cc9e50 as *const (), Throws::Caught), // sub_140cc9e50
    Replacement::draft_throws(0x140cca320, "content_140cca320", fn_140cca320 as *const (), Throws::Caught), // sub_140cca320
    Replacement::draft_throws(0x140cca7b0, "content_140cca7b0", fn_140cca7b0 as *const (), Throws::Caught), // sub_140cca7b0
    Replacement::draft(0x140ccac40, "content_140ccac40", fn_140ccac40 as *const ()), // sub_140ccac40
    Replacement::draft(0x140ccad50, "content_140ccad50", fn_140ccad50 as *const ()), // sub_140ccad50
    Replacement::draft_throws(0x140ccae80, "content_140ccae80", fn_140ccae80 as *const (), Throws::Discard), // sub_140ccae80
    Replacement::draft_throws(0x140ccafb0, "content_140ccafb0", fn_140ccafb0 as *const (), Throws::Caught), // sub_140ccafb0
    Replacement::draft(0x140ccb1a0, "content_140ccb1a0", fn_140ccb1a0 as *const ()), // sub_140ccb1a0
    Replacement::draft_throws(0x140ccb1f0, "content_140ccb1f0", fn_140ccb1f0 as *const (), Throws::Caught), // sub_140ccb1f0
    Replacement::draft_throws(0x140ccb7a0, "content_140ccb7a0", fn_140ccb7a0 as *const (), Throws::Caught), // sub_140ccb7a0
    Replacement::draft_throws(0x140ccbd50, "content_140ccbd50", fn_140ccbd50 as *const (), Throws::Caught), // sub_140ccbd50
    Replacement::draft_throws(0x140ccc2c0, "content_140ccc2c0", fn_140ccc2c0 as *const (), Throws::Caught), // sub_140ccc2c0
    Replacement::draft_throws(0x140ccc830, "content_140ccc830", fn_140ccc830 as *const (), Throws::Caught), // sub_140ccc830
    Replacement::draft_throws(0x140cccc70, "content_140cccc70", fn_140cccc70 as *const (), Throws::Caught), // sub_140cccc70
    Replacement::draft_throws(0x140ccd0b0, "content_140ccd0b0", tpl_15840::<0x14387f3fc, 0x14387f150, 0x1401e9fa0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140ccd0b0
    Replacement::draft(0x140ccd230, "content_140ccd230", tpl_15841::<0x14387f150, 0x14243b0b0, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ccd230
    Replacement::draft(0x140ccd2c0, "content_140ccd2c0", tpl_15842::<0x14387f150, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140ccd2c0
    Replacement::draft_throws(0x140ccd3a0, "content_140ccd3a0", tpl_15843::<0x14387f3fc, 0x14387f150, 0x1401e9b60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140ccd3a0
    Replacement::draft_throws(0x140ccd570, "content_140ccd570", fn_140ccd570 as *const (), Throws::Caught), // sub_140ccd570
    Replacement::draft_throws(0x140ccda40, "content_140ccda40", fn_140ccda40 as *const (), Throws::Caught), // sub_140ccda40
    Replacement::draft_throws(0x140ccded0, "content_140ccded0", fn_140ccded0 as *const (), Throws::Caught), // sub_140ccded0
    Replacement::draft(0x140cce360, "content_140cce360", tpl_15844::<0x14387f3fc, 0x14387f150, 0x1401e9970, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1401e9860> as *const ()), // sub_140cce360
    Replacement::draft(0x140cce440, "content_140cce440", tpl_15842::<0x14387f150, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140cce440
    Replacement::draft_throws(0x140cce530, "content_140cce530", tpl_15845::<0x14387f3fc, 0x14387f150, 0x1401e9770, 0x338, 0x120, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1401e9590> as *const (), Throws::Discard), // sub_140cce530
    Replacement::draft_throws(0x140cce650, "content_140cce650", fn_140cce650 as *const (), Throws::Caught), // sub_140cce650
    Replacement::draft_throws(0x140cce840, "content_140cce840", fn_140cce840 as *const (), Throws::Caught), // sub_140cce840
    Replacement::draft(0x140cce9f0, "content_140cce9f0", fn_140cce9f0 as *const ()), // sub_140cce9f0
    Replacement::draft(0x140cceb80, "content_140cceb80", fn_140cceb80 as *const ()), // sub_140cceb80
    Replacement::draft_throws(0x140ccecc0, "content_140ccecc0", fn_140ccecc0 as *const (), Throws::Caught), // sub_140ccecc0
    Replacement::draft_throws(0x140ccee70, "content_140ccee70", fn_140ccee70 as *const (), Throws::Caught), // sub_140ccee70
    Replacement::draft_throws(0x140ccf380, "content_140ccf380", fn_140ccf380 as *const (), Throws::Caught), // sub_140ccf380
    Replacement::draft_throws(0x140ccf840, "content_140ccf840", fn_140ccf840 as *const (), Throws::Caught), // sub_140ccf840
    Replacement::draft(0x140ccfd00, "content_140ccfd00", fn_140ccfd00 as *const ()), // sub_140ccfd00
    Replacement::draft(0x140ccfe70, "content_140ccfe70", fn_140ccfe70 as *const ()), // sub_140ccfe70
    Replacement::draft_throws(0x140ccffc0, "content_140ccffc0", fn_140ccffc0 as *const (), Throws::Caught), // sub_140ccffc0
    Replacement::draft_throws(0x140cd00f0, "content_140cd00f0", fn_140cd00f0 as *const (), Throws::Caught), // sub_140cd00f0
    Replacement::draft_throws(0x140cd02e0, "content_140cd02e0", tpl_15840::<0x14387f598, 0x14387f4e0, 0x1401e9fa0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140cd02e0
    Replacement::draft(0x140cd0460, "content_140cd0460", tpl_15841::<0x14387f4e0, 0x14243b0b0, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cd0460
    Replacement::draft(0x140cd04f0, "content_140cd04f0", tpl_15842::<0x14387f4e0, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140cd04f0
    Replacement::draft_throws(0x140cd05d0, "content_140cd05d0", tpl_15843::<0x14387f598, 0x14387f4e0, 0x1401e9b60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140cd05d0
    Replacement::draft_throws(0x140cd07a0, "content_140cd07a0", fn_140cd07a0 as *const (), Throws::Caught), // sub_140cd07a0
    Replacement::draft_throws(0x140cd0c70, "content_140cd0c70", fn_140cd0c70 as *const (), Throws::Caught), // sub_140cd0c70
    Replacement::draft_throws(0x140cd1100, "content_140cd1100", fn_140cd1100 as *const (), Throws::Caught), // sub_140cd1100
    Replacement::draft(0x140cd1590, "content_140cd1590", tpl_15844::<0x14387f598, 0x14387f4e0, 0x1401e9970, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1401e9860> as *const ()), // sub_140cd1590
    Replacement::draft(0x140cd1670, "content_140cd1670", tpl_15842::<0x14387f4e0, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140cd1670
    Replacement::draft_throws(0x140cd1760, "content_140cd1760", tpl_15845::<0x14387f598, 0x14387f4e0, 0x1401e9770, 0x338, 0x120, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1401e9590> as *const (), Throws::Discard), // sub_140cd1760
    Replacement::draft_throws(0x140cd1880, "content_140cd1880", fn_140cd1880 as *const (), Throws::Caught), // sub_140cd1880
    Replacement::draft_throws(0x140cd1a70, "content_140cd1a70", fn_140cd1a70 as *const (), Throws::Caught), // sub_140cd1a70
    Replacement::draft_throws(0x140cd1bb0, "content_140cd1bb0", fn_140cd1bb0 as *const (), Throws::Caught), // sub_140cd1bb0
    Replacement::draft_throws(0x140cd1e30, "content_140cd1e30", fn_140cd1e30 as *const (), Throws::Caught), // sub_140cd1e30
    Replacement::draft_throws(0x140cd1f70, "content_140cd1f70", fn_140cd1f70 as *const (), Throws::Caught), // sub_140cd1f70
    Replacement::draft_throws(0x140cd21e0, "content_140cd21e0", fn_140cd21e0 as *const (), Throws::Caught), // sub_140cd21e0
    Replacement::draft_throws(0x140cd2320, "content_140cd2320", fn_140cd2320 as *const (), Throws::Caught), // sub_140cd2320
    Replacement::draft_throws(0x140cd2590, "content_140cd2590", fn_140cd2590 as *const (), Throws::Caught), // sub_140cd2590
    Replacement::draft_throws(0x140cd2810, "content_140cd2810", fn_140cd2810 as *const (), Throws::Caught), // sub_140cd2810
    Replacement::draft_throws(0x140cd2a90, "content_140cd2a90", fn_140cd2a90 as *const (), Throws::Caught), // sub_140cd2a90
    Replacement::draft_throws(0x140cd2d10, "content_140cd2d10", fn_140cd2d10 as *const (), Throws::Caught), // sub_140cd2d10
    Replacement::draft_throws(0x140cd2f60, "content_140cd2f60", fn_140cd2f60 as *const (), Throws::Caught), // sub_140cd2f60
    Replacement::draft_throws(0x140cd31a0, "content_140cd31a0", fn_140cd31a0 as *const (), Throws::Caught), // sub_140cd31a0
    Replacement::draft_throws(0x140cd33e0, "content_140cd33e0", fn_140cd33e0 as *const (), Throws::Caught), // sub_140cd33e0
    Replacement::draft_throws(0x140cd3520, "content_140cd3520", fn_140cd3520 as *const (), Throws::Caught), // sub_140cd3520
    Replacement::draft_throws(0x140cd3660, "content_140cd3660", fn_140cd3660 as *const (), Throws::Caught), // sub_140cd3660
    Replacement::draft_throws(0x140cd37a0, "content_140cd37a0", fn_140cd37a0 as *const (), Throws::Caught), // sub_140cd37a0
    Replacement::draft_throws(0x140cd3a20, "content_140cd3a20", fn_140cd3a20 as *const (), Throws::Caught), // sub_140cd3a20
    Replacement::draft_throws(0x140cd3c90, "content_140cd3c90", fn_140cd3c90 as *const (), Throws::Caught), // sub_140cd3c90
    Replacement::draft_throws(0x140cd3f00, "content_140cd3f00", fn_140cd3f00 as *const (), Throws::Caught), // sub_140cd3f00
    Replacement::draft_throws(0x140cd4180, "content_140cd4180", fn_140cd4180 as *const (), Throws::Caught), // sub_140cd4180
    Replacement::draft_throws(0x140cd43f0, "content_140cd43f0", fn_140cd43f0 as *const (), Throws::Caught), // sub_140cd43f0
    Replacement::draft_throws(0x140cd4660, "content_140cd4660", fn_140cd4660 as *const (), Throws::Caught), // sub_140cd4660
    Replacement::draft_throws(0x140cd4ad0, "content_140cd4ad0", fn_140cd4ad0 as *const (), Throws::Caught), // sub_140cd4ad0
    Replacement::draft_throws(0x140cd5060, "content_140cd5060", fn_140cd5060 as *const (), Throws::Caught), // sub_140cd5060
    Replacement::draft_throws(0x140cd5490, "content_140cd5490", fn_140cd5490 as *const (), Throws::Caught), // sub_140cd5490
    Replacement::draft_throws(0x140cd59f0, "content_140cd59f0", fn_140cd59f0 as *const (), Throws::Caught), // sub_140cd59f0
    Replacement::draft_throws(0x140cd5d40, "content_140cd5d40", fn_140cd5d40 as *const (), Throws::Caught), // sub_140cd5d40
    Replacement::draft_throws(0x140cd6090, "content_140cd6090", fn_140cd6090 as *const (), Throws::Discard), // sub_140cd6090
    Replacement::draft_throws(0x140cd6230, "content_140cd6230", fn_140cd6230 as *const (), Throws::Discard), // sub_140cd6230
    Replacement::draft_throws(0x140cd6370, "content_140cd6370", fn_140cd6370 as *const (), Throws::Caught), // sub_140cd6370
    Replacement::draft_throws(0x140cd6960, "content_140cd6960", fn_140cd6960 as *const (), Throws::Caught), // sub_140cd6960
    Replacement::draft_throws(0x140cd6ee0, "content_140cd6ee0", fn_140cd6ee0 as *const (), Throws::Caught), // sub_140cd6ee0
    Replacement::draft_throws(0x140cd7230, "content_140cd7230", fn_140cd7230 as *const (), Throws::Caught), // sub_140cd7230
    Replacement::draft_throws(0x140cd73a0, "content_140cd73a0", fn_140cd73a0 as *const (), Throws::Caught), // sub_140cd73a0
    Replacement::draft_throws(0x140cd7b50, "content_140cd7b50", fn_140cd7b50 as *const (), Throws::Caught), // sub_140cd7b50
    Replacement::draft_throws(0x140cd8290, "content_140cd8290", fn_140cd8290 as *const (), Throws::Caught), // sub_140cd8290
    Replacement::draft_throws(0x140cd8780, "content_140cd8780", fn_140cd8780 as *const (), Throws::Discard), // sub_140cd8780
    Replacement::draft(0x140cd8830, "content_140cd8830", fn_140cd8830 as *const ()), // sub_140cd8830
    Replacement::draft_throws(0x140cd8900, "content_140cd8900", fn_140cd8900 as *const (), Throws::Caught), // sub_140cd8900
    Replacement::draft_throws(0x140cd8bb0, "content_140cd8bb0", fn_140cd8bb0 as *const (), Throws::Caught), // sub_140cd8bb0
    Replacement::draft_throws(0x140cd8e60, "content_140cd8e60", fn_140cd8e60 as *const (), Throws::Caught), // sub_140cd8e60
    Replacement::draft_throws(0x140cd92d0, "content_140cd92d0", fn_140cd92d0 as *const (), Throws::Caught), // sub_140cd92d0
    Replacement::draft_throws(0x140cd9700, "content_140cd9700", fn_140cd9700 as *const (), Throws::Caught), // sub_140cd9700
    Replacement::draft_throws(0x140cd9a50, "content_140cd9a50", fn_140cd9a50 as *const (), Throws::Discard), // sub_140cd9a50
    Replacement::draft_throws(0x140cd9c30, "content_140cd9c30", fn_140cd9c30 as *const (), Throws::Caught), // sub_140cd9c30
    Replacement::draft_throws(0x140cd9d60, "content_140cd9d60", fn_140cd9d60 as *const (), Throws::Caught), // sub_140cd9d60
    Replacement::draft_throws(0x140cd9e40, "content_140cd9e40", fn_140cd9e40 as *const (), Throws::Caught), // sub_140cd9e40
    Replacement::draft_throws(0x140cd9f70, "content_140cd9f70", fn_140cd9f70 as *const (), Throws::Caught), // sub_140cd9f70
    Replacement::draft_throws(0x140cda050, "content_140cda050", fn_140cda050 as *const (), Throws::Caught), // sub_140cda050
    Replacement::draft_throws(0x140cda180, "content_140cda180", fn_140cda180 as *const (), Throws::Caught), // sub_140cda180
    Replacement::draft_throws(0x140cda260, "content_140cda260", fn_140cda260 as *const (), Throws::Caught), // sub_140cda260
    Replacement::draft_throws(0x140cda670, "content_140cda670", fn_140cda670 as *const (), Throws::Caught), // sub_140cda670
    Replacement::draft_throws(0x140cda8c0, "content_140cda8c0", fn_140cda8c0 as *const (), Throws::Caught), // sub_140cda8c0
    Replacement::draft_throws(0x140cdae50, "content_140cdae50", fn_140cdae50 as *const (), Throws::Caught), // sub_140cdae50
    Replacement::draft_throws(0x140cdb3a0, "content_140cdb3a0", fn_140cdb3a0 as *const (), Throws::Caught), // sub_140cdb3a0
    Replacement::draft_throws(0x140cdb6f0, "content_140cdb6f0", tpl_15881::<0x14387f150, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1432dab30, 0x1432dab48, 0x14020f240, 0x1434268d0, 0x1432daa90, 0x1432daaa8, 0x1432daab8, 0x1432daad0, 0x143426d48, 0x1432dabd0, 0x1432dabe8, 0x143426d10, 0x1432dabf8, 0x1432dac08, 0x143426abc, 0x1432dab58, 0x1432dab70, 0x1434265d4, 0x1432dab80, 0x1432dabc0, 0x1432ba3e0> as *const (), Throws::Discard), // sub_140cdb6f0
    Replacement::draft_throws(0x140cdb960, "content_140cdb960", fn_140cdb960 as *const (), Throws::Caught), // sub_140cdb960
    Replacement::draft_throws(0x140cdbdd0, "content_140cdbdd0", fn_140cdbdd0 as *const (), Throws::Caught), // sub_140cdbdd0
    Replacement::draft_throws(0x140cdc200, "content_140cdc200", fn_140cdc200 as *const (), Throws::Caught), // sub_140cdc200
    Replacement::draft_throws(0x140cdc550, "content_140cdc550", fn_140cdc550 as *const (), Throws::Caught), // sub_140cdc550
    Replacement::draft_throws(0x140cdc6e0, "content_140cdc6e0", fn_140cdc6e0 as *const (), Throws::Caught), // sub_140cdc6e0
    Replacement::draft_throws(0x140cdcc70, "content_140cdcc70", fn_140cdcc70 as *const (), Throws::Caught), // sub_140cdcc70
    Replacement::draft_throws(0x140cdd1c0, "content_140cdd1c0", fn_140cdd1c0 as *const (), Throws::Caught), // sub_140cdd1c0
    Replacement::draft_throws(0x140cdd510, "content_140cdd510", tpl_15881::<0x14387f4e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1432dab30, 0x1432daa00, 0x14020f240, 0x1434268d0, 0x1432daa90, 0x1432daa10, 0x1432daab8, 0x1432daa18, 0x143426d48, 0x1432dabd0, 0x1432daa20, 0x143426d10, 0x1432dabf8, 0x1432da9e0, 0x143426abc, 0x1432dab58, 0x1432da9e8, 0x1434265d4, 0x1432dab80, 0x1432da9f0, 0x1432ba3e0> as *const (), Throws::Discard), // sub_140cdd510
    Replacement::draft_throws(0x140cdd7c0, "content_140cdd7c0", fn_140cdd7c0 as *const (), Throws::Caught), // sub_140cdd7c0
    Replacement::draft_throws(0x140cddbf0, "content_140cddbf0", fn_140cddbf0 as *const (), Throws::Caught), // sub_140cddbf0
    Replacement::draft_throws(0x140cddd30, "content_140cddd30", fn_140cddd30 as *const (), Throws::Caught), // sub_140cddd30
    Replacement::draft_throws(0x140cde100, "content_140cde100", fn_140cde100 as *const (), Throws::Caught), // sub_140cde100
    Replacement::draft_throws(0x140cde240, "content_140cde240", fn_140cde240 as *const (), Throws::Caught), // sub_140cde240
    Replacement::draft_throws(0x140cde710, "content_140cde710", fn_140cde710 as *const (), Throws::Caught), // sub_140cde710
    Replacement::draft_throws(0x140cde850, "content_140cde850", fn_140cde850 as *const (), Throws::Caught), // sub_140cde850
    Replacement::draft_throws(0x140cde9a0, "content_140cde9a0", fn_140cde9a0 as *const (), Throws::Caught), // sub_140cde9a0
    Replacement::draft_throws(0x140cdeaf0, "content_140cdeaf0", fn_140cdeaf0 as *const (), Throws::Caught), // sub_140cdeaf0
    Replacement::draft_throws(0x140cdec40, "content_140cdec40", fn_140cdec40 as *const (), Throws::Caught), // sub_140cdec40
    Replacement::draft_throws(0x140cdee30, "content_140cdee30", fn_140cdee30 as *const (), Throws::Caught), // sub_140cdee30
    Replacement::draft_throws(0x140cdeff0, "content_140cdeff0", fn_140cdeff0 as *const (), Throws::Caught), // sub_140cdeff0
    Replacement::draft_throws(0x140cdf1b0, "content_140cdf1b0", fn_140cdf1b0 as *const (), Throws::Caught), // sub_140cdf1b0
    Replacement::draft_throws(0x140cdf500, "content_140cdf500", fn_140cdf500 as *const (), Throws::Caught), // sub_140cdf500
    Replacement::draft_throws(0x140cdf820, "content_140cdf820", fn_140cdf820 as *const (), Throws::Caught), // sub_140cdf820
    Replacement::draft_throws(0x140cdfb30, "content_140cdfb30", fn_140cdfb30 as *const (), Throws::Caught), // sub_140cdfb30
    Replacement::draft_throws(0x140cdfc00, "content_140cdfc00", fn_140cdfc00 as *const (), Throws::Caught), // sub_140cdfc00
    Replacement::draft_throws(0x140cdfea0, "content_140cdfea0", fn_140cdfea0 as *const (), Throws::Caught), // sub_140cdfea0
    Replacement::draft_throws(0x140ce0140, "content_140ce0140", fn_140ce0140 as *const (), Throws::Caught), // sub_140ce0140
    Replacement::draft_throws(0x140ce03e0, "content_140ce03e0", fn_140ce03e0 as *const (), Throws::Caught), // sub_140ce03e0
    Replacement::draft_throws(0x140ce0570, "content_140ce0570", fn_140ce0570 as *const (), Throws::Caught), // sub_140ce0570
    Replacement::draft_throws(0x140ce0700, "content_140ce0700", fn_140ce0700 as *const (), Throws::Caught), // sub_140ce0700
    Replacement::draft_throws(0x140ce0890, "content_140ce0890", fn_140ce0890 as *const (), Throws::Caught), // sub_140ce0890
    Replacement::draft_throws(0x140ce0b30, "content_140ce0b30", fn_140ce0b30 as *const (), Throws::Caught), // sub_140ce0b30
    Replacement::draft_throws(0x140ce0dd0, "content_140ce0dd0", fn_140ce0dd0 as *const (), Throws::Caught), // sub_140ce0dd0
    Replacement::draft_throws(0x140ce1070, "content_140ce1070", fn_140ce1070 as *const (), Throws::Discard), // sub_140ce1070
    Replacement::draft(0x140ce1120, "content_140ce1120", fn_140ce1120 as *const ()), // sub_140ce1120
    Replacement::draft(0x140ce1190, "content_140ce1190", fn_140ce1190 as *const ()), // sub_140ce1190
    Replacement::draft_throws(0x140ce1210, "content_140ce1210", fn_140ce1210 as *const (), Throws::Discard), // sub_140ce1210
    Replacement::draft_throws(0x140ce12f0, "content_140ce12f0", tpl_838::<0x140ce18e0> as *const (), Throws::Discard), // sub_140ce12f0
    Replacement::draft(0x140ce1310, "content_140ce1310", fn_140ce1310 as *const ()), // sub_140ce1310
    Replacement::draft(0x140ce13a0, "content_140ce13a0", fn_140ce13a0 as *const ()), // sub_140ce13a0
    Replacement::draft_throws(0x140ce1430, "content_140ce1430", fn_140ce1430 as *const (), Throws::Discard), // sub_140ce1430
    Replacement::draft_throws(0x140ce14c0, "content_140ce14c0", fn_140ce14c0 as *const (), Throws::Caught), // sub_140ce14c0
    Replacement::draft_throws(0x140ce18e0, "content_140ce18e0", fn_140ce18e0 as *const (), Throws::Caught), // sub_140ce18e0
    Replacement::draft_throws(0x140ce1cd0, "content_140ce1cd0", fn_140ce1cd0 as *const (), Throws::Caught), // sub_140ce1cd0
    Replacement::draft_throws(0x140ce20b0, "content_140ce20b0", fn_140ce20b0 as *const (), Throws::Caught), // sub_140ce20b0
    Replacement::draft_throws(0x140ce2340, "content_140ce2340", fn_140ce2340 as *const (), Throws::Caught), // sub_140ce2340
    Replacement::draft_throws(0x140ce2810, "content_140ce2810", fn_140ce2810 as *const (), Throws::Caught), // sub_140ce2810
    Replacement::draft_throws(0x140ce2c90, "content_140ce2c90", fn_140ce2c90 as *const (), Throws::Caught), // sub_140ce2c90
    Replacement::draft_throws(0x140ce3110, "content_140ce3110", fn_140ce3110 as *const (), Throws::Caught), // sub_140ce3110
    Replacement::draft_throws(0x140ce3360, "content_140ce3360", fn_140ce3360 as *const (), Throws::Caught), // sub_140ce3360
    Replacement::draft_throws(0x140ce35a0, "content_140ce35a0", fn_140ce35a0 as *const (), Throws::Caught), // sub_140ce35a0
    Replacement::draft_throws(0x140ce37e0, "content_140ce37e0", fn_140ce37e0 as *const (), Throws::Discard), // sub_140ce37e0
    Replacement::draft_throws(0x140ce38d0, "content_140ce38d0", fn_140ce38d0 as *const (), Throws::Caught), // sub_140ce38d0
    Replacement::draft_throws(0x140ce40e0, "content_140ce40e0", fn_140ce40e0 as *const (), Throws::Caught), // sub_140ce40e0
    Replacement::draft_throws(0x140ce4880, "content_140ce4880", fn_140ce4880 as *const (), Throws::Caught), // sub_140ce4880
    Replacement::draft_throws(0x140ce4dd0, "content_140ce4dd0", fn_140ce4dd0 as *const (), Throws::Caught), // sub_140ce4dd0
    Replacement::draft_throws(0x140ce5010, "content_140ce5010", fn_140ce5010 as *const (), Throws::Caught), // sub_140ce5010
    Replacement::draft_throws(0x140ce5220, "content_140ce5220", fn_140ce5220 as *const (), Throws::Caught), // sub_140ce5220
    Replacement::draft(0x140ce5430, "content_140ce5430", fn_140ce5430 as *const ()), // sub_140ce5430
    Replacement::draft(0x140ce54d0, "content_140ce54d0", fn_140ce54d0 as *const ()), // sub_140ce54d0
    Replacement::draft_throws(0x140ce5580, "content_140ce5580", fn_140ce5580 as *const (), Throws::Discard), // sub_140ce5580
    Replacement::draft(0x140ce5830, "content_140ce5830", fn_140ce5830 as *const ()), // sub_140ce5830
    Replacement::draft(0x140ce5910, "content_140ce5910", fn_140ce5910 as *const ()), // sub_140ce5910
    Replacement::draft_throws(0x140ce5a30, "content_140ce5a30", fn_140ce5a30 as *const (), Throws::Discard), // sub_140ce5a30
    Replacement::draft(0x140ce5ae0, "content_140ce5ae0", fn_140ce5ae0 as *const ()), // sub_140ce5ae0
    Replacement::draft(0x140ce5b70, "content_140ce5b70", tpl_15924::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x2c, 0xffffffffffffffec, 0x4, 0xffffffffffffffe4, 0xfffffffffffffff4> as *const ()), // sub_140ce5b70
    Replacement::draft(0x140ce5c10, "content_140ce5c10", fn_140ce5c10 as *const ()), // sub_140ce5c10
    Replacement::draft(0x140ce5ca0, "content_140ce5ca0", fn_140ce5ca0 as *const ()), // sub_140ce5ca0
    Replacement::draft(0x140ce5d30, "content_140ce5d30", fn_140ce5d30 as *const ()), // sub_140ce5d30
    Replacement::draft(0x140ce5e60, "content_140ce5e60", fn_140ce5e60 as *const ()), // sub_140ce5e60
    Replacement::draft(0x140ce60b0, "content_140ce60b0", fn_140ce60b0 as *const ()), // sub_140ce60b0
    Replacement::draft(0x140ce6150, "content_140ce6150", fn_140ce6150 as *const ()), // sub_140ce6150
    Replacement::draft(0x140ce6210, "content_140ce6210", fn_140ce6210 as *const ()), // sub_140ce6210
    Replacement::draft(0x140ce62b0, "content_140ce62b0", fn_140ce62b0 as *const ()), // sub_140ce62b0
    Replacement::draft(0x140ce6370, "content_140ce6370", tpl_15933::<0x14387f488, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ce6370
    Replacement::draft(0x140ce6440, "content_140ce6440", tpl_2258::<0x14387f488, 0x14243b0b0, 0x338, 0x48, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ce6440
    Replacement::draft_throws(0x140ce64c0, "content_140ce64c0", fn_140ce64c0 as *const (), Throws::Discard), // sub_140ce64c0
    Replacement::draft_throws(0x140ce6590, "content_140ce6590", fn_140ce6590 as *const (), Throws::Caught), // sub_140ce6590
    Replacement::draft_throws(0x140ce66d0, "content_140ce66d0", fn_140ce66d0 as *const (), Throws::Caught), // sub_140ce66d0
    Replacement::draft_throws(0x140ce6810, "content_140ce6810", fn_140ce6810 as *const (), Throws::Caught), // sub_140ce6810
    Replacement::draft(0x140ce6950, "content_140ce6950", tpl_2262::<0x14387f2e0, 0x14387f488, 0x1401e9970, 0x338, 0xe8, 0x4, 0xd8, 0x1401e9860> as *const ()), // sub_140ce6950
    Replacement::draft(0x140ce69d0, "content_140ce69d0", tpl_2258::<0x14387f488, 0x14243b0b0, 0x338, 0xe8, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ce69d0
    Replacement::draft_throws(0x140ce6a60, "content_140ce6a60", fn_140ce6a60 as *const (), Throws::Discard), // sub_140ce6a60
    Replacement::draft_throws(0x140ce6af0, "content_140ce6af0", fn_140ce6af0 as *const (), Throws::Discard), // sub_140ce6af0
    Replacement::draft(0x140ce6c20, "content_140ce6c20", fn_140ce6c20 as *const ()), // sub_140ce6c20
    Replacement::draft(0x140ce6c80, "content_140ce6c80", fn_140ce6c80 as *const ()), // sub_140ce6c80
    Replacement::draft(0x140ce6da0, "content_140ce6da0", fn_140ce6da0 as *const ()), // sub_140ce6da0
    Replacement::draft(0x140ce6f10, "content_140ce6f10", fn_140ce6f10 as *const ()), // sub_140ce6f10
    Replacement::draft(0x140ce6fa0, "content_140ce6fa0", fn_140ce6fa0 as *const ()), // sub_140ce6fa0
    Replacement::draft(0x140ce7050, "content_140ce7050", fn_140ce7050 as *const ()), // sub_140ce7050
    Replacement::draft(0x140ce7100, "content_140ce7100", fn_140ce7100 as *const ()), // sub_140ce7100
    Replacement::draft_throws(0x140ce71b0, "content_140ce71b0", fn_140ce71b0 as *const (), Throws::Discard), // sub_140ce71b0
    Replacement::draft(0x140ce7350, "content_140ce7350", fn_140ce7350 as *const ()), // sub_140ce7350
    Replacement::draft(0x140ce73e0, "content_140ce73e0", fn_140ce73e0 as *const ()), // sub_140ce73e0
    Replacement::draft(0x140ce7500, "content_140ce7500", fn_140ce7500 as *const ()), // sub_140ce7500
    Replacement::draft(0x140ce7640, "content_140ce7640", tpl_15948::<0x143426a48, 0x1434269f8, 0x143251120, 0x1432d6988, 0x1432a90cc, 0x14020f270> as *const ()), // sub_140ce7640
    Replacement::draft(0x140ce76b0, "content_140ce76b0", fn_140ce76b0 as *const ()), // sub_140ce76b0
    Replacement::draft(0x140ce7740, "content_140ce7740", fn_140ce7740 as *const ()), // sub_140ce7740
    Replacement::draft_throws(0x140ce7860, "content_140ce7860", fn_140ce7860 as *const (), Throws::Discard), // sub_140ce7860
    Replacement::draft(0x140ce7920, "content_140ce7920", fn_140ce7920 as *const ()), // sub_140ce7920
    Replacement::draft(0x140ce79c0, "content_140ce79c0", fn_140ce79c0 as *const ()), // sub_140ce79c0
    Replacement::draft_throws(0x140ce7a80, "content_140ce7a80", fn_140ce7a80 as *const (), Throws::Discard), // sub_140ce7a80
    Replacement::draft(0x140ce7c40, "content_140ce7c40", tpl_15955::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ce898, 0x20, 0x8, 0x1c> as *const ()), // sub_140ce7c40
    Replacement::draft(0x140ce7ce0, "content_140ce7ce0", tpl_15956::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x1432ce898, 0x5, 0xd, 0xf, 0xffffffffffffffe8> as *const ()), // sub_140ce7ce0
    Replacement::draft(0x140ce7da0, "content_140ce7da0", fn_140ce7da0 as *const ()), // sub_140ce7da0
    Replacement::draft(0x140ce7e20, "content_140ce7e20", fn_140ce7e20 as *const ()), // sub_140ce7e20
    Replacement::draft(0x140ce7ec0, "content_140ce7ec0", fn_140ce7ec0 as *const ()), // sub_140ce7ec0
    Replacement::draft(0x140ce7f90, "content_140ce7f90", tpl_15960::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x20> as *const ()), // sub_140ce7f90
    Replacement::draft(0x140ce8020, "content_140ce8020", tpl_15924::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x34, 0xffffffffffffffdc, 0xffffffffffffffe4, 0xffffffffffffffec, 0xfffffffffffffff4> as *const ()), // sub_140ce8020
    Replacement::draft(0x140ce80c0, "content_140ce80c0", fn_140ce80c0 as *const ()), // sub_140ce80c0
    Replacement::draft(0x140ce81a0, "content_140ce81a0", fn_140ce81a0 as *const ()), // sub_140ce81a0
    Replacement::draft(0x140ce8230, "content_140ce8230", fn_140ce8230 as *const ()), // sub_140ce8230
    Replacement::draft(0x140ce82c0, "content_140ce82c0", fn_140ce82c0 as *const ()), // sub_140ce82c0
    Replacement::draft(0x140ce8370, "content_140ce8370", fn_140ce8370 as *const ()), // sub_140ce8370
    Replacement::draft(0x140ce84f0, "content_140ce84f0", fn_140ce84f0 as *const ()), // sub_140ce84f0
    Replacement::draft(0x140ce85b0, "content_140ce85b0", fn_140ce85b0 as *const ()), // sub_140ce85b0
    Replacement::draft(0x140ce8680, "content_140ce8680", fn_140ce8680 as *const ()), // sub_140ce8680
    Replacement::draft(0x140ce8750, "content_140ce8750", fn_140ce8750 as *const ()), // sub_140ce8750
    Replacement::draft_throws(0x140ce8930, "content_140ce8930", tpl_15969::<0x14387f3d8, 0x14387f2e8, 0x1401e9fa0, 0x1403c7ba0, 0x140bff510, 0x1401e9e70> as *const (), Throws::Discard), // sub_140ce8930
    Replacement::draft_throws(0x140ce8b00, "content_140ce8b00", tpl_15969::<0x14387f3d8, 0x14387f2e8, 0x1401e9b60, 0x1403c7ba0, 0x140bff510, 0x1401e9a20> as *const (), Throws::Discard), // sub_140ce8b00
    Replacement::draft_throws(0x140ce8cd0, "content_140ce8cd0", fn_140ce8cd0 as *const (), Throws::Caught), // sub_140ce8cd0
    Replacement::draft(0x140ce8d60, "content_140ce8d60", fn_140ce8d60 as *const ()), // sub_140ce8d60
    Replacement::draft_throws(0x140ce8f30, "content_140ce8f30", tpl_15969::<0x14387f3d8, 0x14387f2e8, 0x1401e9770, 0x1403c7ba0, 0x140bff510, 0x1401e9590> as *const (), Throws::Discard), // sub_140ce8f30
    Replacement::draft_throws(0x140ce9100, "content_140ce9100", tpl_15972::<0x14387f2e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140ce9100
    Replacement::draft(0x140ce9180, "content_140ce9180", tpl_15948::<0x143426c70, 0x143426b90, 0x1432d6030, 0x14325edbc, 0x1432d6288, 0x14020f270> as *const ()), // sub_140ce9180
    Replacement::draft(0x140ce91f0, "content_140ce91f0", fn_140ce91f0 as *const ()), // sub_140ce91f0
    Replacement::draft(0x140ce92c0, "content_140ce92c0", fn_140ce92c0 as *const ()), // sub_140ce92c0
    Replacement::draft(0x140ce9340, "content_140ce9340", fn_140ce9340 as *const ()), // sub_140ce9340
    Replacement::draft_throws(0x140ce9460, "content_140ce9460", fn_140ce9460 as *const (), Throws::Discard), // sub_140ce9460
    Replacement::draft(0x140ce9550, "content_140ce9550", fn_140ce9550 as *const ()), // sub_140ce9550
    Replacement::draft(0x140ce9600, "content_140ce9600", fn_140ce9600 as *const ()), // sub_140ce9600
    Replacement::draft(0x140ce96c0, "content_140ce96c0", fn_140ce96c0 as *const ()), // sub_140ce96c0
    Replacement::draft(0x140ce9770, "content_140ce9770", fn_140ce9770 as *const ()), // sub_140ce9770
    Replacement::draft(0x140ce9840, "content_140ce9840", fn_140ce9840 as *const ()), // sub_140ce9840
    Replacement::draft(0x140ce9930, "content_140ce9930", fn_140ce9930 as *const ()), // sub_140ce9930
    Replacement::draft(0x140ce99d0, "content_140ce99d0", fn_140ce99d0 as *const ()), // sub_140ce99d0
    Replacement::draft(0x140ce9a70, "content_140ce9a70", fn_140ce9a70 as *const ()), // sub_140ce9a70
    Replacement::draft(0x140ce9b30, "content_140ce9b30", fn_140ce9b30 as *const ()), // sub_140ce9b30
    Replacement::draft(0x140ce9bc0, "content_140ce9bc0", fn_140ce9bc0 as *const ()), // sub_140ce9bc0
    Replacement::draft(0x140ce9c70, "content_140ce9c70", fn_140ce9c70 as *const ()), // sub_140ce9c70
    Replacement::draft(0x140ce9d00, "content_140ce9d00", fn_140ce9d00 as *const ()), // sub_140ce9d00
    Replacement::draft_throws(0x140ce9da0, "content_140ce9da0", tpl_15419::<0x14387f1c8, 0x14387f298, 0x1401e9b60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140ce9da0
    Replacement::draft_throws(0x140ce9f30, "content_140ce9f30", fn_140ce9f30 as *const (), Throws::Caught), // sub_140ce9f30
    Replacement::draft(0x140ce9fc0, "content_140ce9fc0", fn_140ce9fc0 as *const ()), // sub_140ce9fc0
    Replacement::draft_throws(0x140cea150, "content_140cea150", tpl_15419::<0x14387f1c8, 0x14387f298, 0x1401e9770, 0x1401e9590> as *const (), Throws::Discard), // sub_140cea150
    Replacement::draft_throws(0x140cea2e0, "content_140cea2e0", tpl_15972::<0x14387f298, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140cea2e0
    Replacement::draft(0x140cea360, "content_140cea360", fn_140cea360 as *const ()), // sub_140cea360
    Replacement::draft(0x140cea3e0, "content_140cea3e0", tpl_10085::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x24, 0xffffffffffffffec, 0x4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140cea3e0
    Replacement::draft(0x140cea480, "content_140cea480", fn_140cea480 as *const ()), // sub_140cea480
    Replacement::draft(0x140cea560, "content_140cea560", tpl_15990::<0x1432dab30, 0x1426cdcb0, 0x1432dab48, 0x14020f270, 0x143426a48, 0x1432daa90, 0x1432daaa8, 0x1432daab8, 0x1432daad0, 0x143426b30, 0x1432dabd0, 0x1432dabe8, 0x143426b20, 0x1432dabf8, 0x1432dac08, 0x143426ac0, 0x1432dab58, 0x1432dab70, 0x143426970, 0x1432dab80, 0x1432ba3e0, 0x1432dabc0> as *const ()), // sub_140cea560
    Replacement::draft(0x140cea720, "content_140cea720", fn_140cea720 as *const ()), // sub_140cea720
    Replacement::draft(0x140cea8d0, "content_140cea8d0", fn_140cea8d0 as *const ()), // sub_140cea8d0
    Replacement::draft(0x140ceaca0, "content_140ceaca0", fn_140ceaca0 as *const ()), // sub_140ceaca0
    Replacement::draft(0x140ceb2b0, "content_140ceb2b0", fn_140ceb2b0 as *const ()), // sub_140ceb2b0
    Replacement::draft_throws(0x140ceb740, "content_140ceb740", fn_140ceb740 as *const (), Throws::Discard), // sub_140ceb740
    Replacement::draft_throws(0x140cebd60, "content_140cebd60", fn_140cebd60 as *const (), Throws::Caught), // sub_140cebd60
    Replacement::draft_throws(0x140cec1f0, "content_140cec1f0", fn_140cec1f0 as *const (), Throws::Caught), // sub_140cec1f0
    Replacement::draft_throws(0x140cec680, "content_140cec680", fn_140cec680 as *const (), Throws::Caught), // sub_140cec680
    Replacement::draft(0x140cecb10, "content_140cecb10", fn_140cecb10 as *const ()), // sub_140cecb10
    Replacement::draft(0x140cecfb0, "content_140cecfb0", fn_140cecfb0 as *const ()), // sub_140cecfb0
    Replacement::draft_throws(0x140ced460, "content_140ced460", fn_140ced460 as *const (), Throws::Discard), // sub_140ced460
    Replacement::draft_throws(0x140ced970, "content_140ced970", fn_140ced970 as *const (), Throws::Discard), // sub_140ced970
    Replacement::draft(0x140cedd90, "content_140cedd90", fn_140cedd90 as *const ()), // sub_140cedd90
    Replacement::draft(0x140cee0f0, "content_140cee0f0", fn_140cee0f0 as *const ()), // sub_140cee0f0
    Replacement::draft(0x140cee2a0, "content_140cee2a0", fn_140cee2a0 as *const ()), // sub_140cee2a0
    Replacement::draft(0x140cee6e0, "content_140cee6e0", fn_140cee6e0 as *const ()), // sub_140cee6e0
    Replacement::draft(0x140ceeda0, "content_140ceeda0", fn_140ceeda0 as *const ()), // sub_140ceeda0
    Replacement::draft_throws(0x140cef2d0, "content_140cef2d0", fn_140cef2d0 as *const (), Throws::Discard), // sub_140cef2d0
    Replacement::draft(0x140cef9d0, "content_140cef9d0", fn_140cef9d0 as *const ()), // sub_140cef9d0
    Replacement::draft_throws(0x140cefab0, "content_140cefab0", fn_140cefab0 as *const (), Throws::Caught), // sub_140cefab0
    Replacement::draft_throws(0x140ceffd0, "content_140ceffd0", fn_140ceffd0 as *const (), Throws::Caught), // sub_140ceffd0
    Replacement::draft_throws(0x140cf04f0, "content_140cf04f0", fn_140cf04f0 as *const (), Throws::Caught), // sub_140cf04f0
    Replacement::draft(0x140cf0a10, "content_140cf0a10", fn_140cf0a10 as *const ()), // sub_140cf0a10
    Replacement::draft(0x140cf0f50, "content_140cf0f50", fn_140cf0f50 as *const ()), // sub_140cf0f50
    Replacement::draft_throws(0x140cf14a0, "content_140cf14a0", fn_140cf14a0 as *const (), Throws::Discard), // sub_140cf14a0
    Replacement::draft_throws(0x140cf1a60, "content_140cf1a60", fn_140cf1a60 as *const (), Throws::Discard), // sub_140cf1a60
    Replacement::draft(0x140cf1f60, "content_140cf1f60", fn_140cf1f60 as *const ()), // sub_140cf1f60
    Replacement::draft(0x140cf2310, "content_140cf2310", tpl_15933::<0x14387f2b8, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf2310
    Replacement::draft_throws(0x140cf23e0, "content_140cf23e0", tpl_9957::<0x14387f398, 0x14387f2b8, 0x1401e9fa0, 0x328, 0x338, 0x38, 0x369, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_140cf23e0
    Replacement::draft(0x140cf2490, "content_140cf2490", tpl_9959::<0x14387f2b8, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf2490
    Replacement::draft_throws(0x140cf2510, "content_140cf2510", tpl_16013::<0x14387f398, 0x14387f2b8, 0x1401e9b60, 0x328, 0x338, 0x120, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140cf2510
    Replacement::draft_throws(0x140cf25e0, "content_140cf25e0", fn_140cf25e0 as *const (), Throws::Caught), // sub_140cf25e0
    Replacement::draft_throws(0x140cf2750, "content_140cf2750", fn_140cf2750 as *const (), Throws::Caught), // sub_140cf2750
    Replacement::draft_throws(0x140cf28c0, "content_140cf28c0", fn_140cf28c0 as *const (), Throws::Caught), // sub_140cf28c0
    Replacement::draft(0x140cf2a30, "content_140cf2a30", tpl_16015::<0x14387f2b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf2a30
    Replacement::draft(0x140cf2aa0, "content_140cf2aa0", tpl_9961::<0x14387f398, 0x14387f2b8, 0x1401e9970, 0x338, 0xd8, 0x4, 0x1401e9860> as *const ()), // sub_140cf2aa0
    Replacement::draft(0x140cf2b20, "content_140cf2b20", tpl_9959::<0x14387f2b8, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf2b20
    Replacement::draft_throws(0x140cf2bb0, "content_140cf2bb0", tpl_16016::<0x14387f398, 0x14387f2b8, 0x1401e9770, 0x338, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140cf2bb0
    Replacement::draft_throws(0x140cf2c40, "content_140cf2c40", fn_140cf2c40 as *const (), Throws::Discard), // sub_140cf2c40
    Replacement::draft(0x140cf2da0, "content_140cf2da0", fn_140cf2da0 as *const ()), // sub_140cf2da0
    Replacement::draft(0x140cf2e60, "content_140cf2e60", fn_140cf2e60 as *const ()), // sub_140cf2e60
    Replacement::draft(0x140cf2f20, "content_140cf2f20", fn_140cf2f20 as *const ()), // sub_140cf2f20
    Replacement::draft(0x140cf3040, "content_140cf3040", fn_140cf3040 as *const ()), // sub_140cf3040
    Replacement::draft(0x140cf31f0, "content_140cf31f0", fn_140cf31f0 as *const ()), // sub_140cf31f0
    Replacement::draft(0x140cf32f0, "content_140cf32f0", fn_140cf32f0 as *const ()), // sub_140cf32f0
    Replacement::draft(0x140cf3860, "content_140cf3860", fn_140cf3860 as *const ()), // sub_140cf3860
    Replacement::draft_throws(0x140cf3bb0, "content_140cf3bb0", fn_140cf3bb0 as *const (), Throws::Caught), // sub_140cf3bb0
    Replacement::draft(0x140cf4200, "content_140cf4200", fn_140cf4200 as *const ()), // sub_140cf4200
    Replacement::draft_throws(0x140cf4370, "content_140cf4370", fn_140cf4370 as *const (), Throws::Caught), // sub_140cf4370
    Replacement::draft_throws(0x140cf4c80, "content_140cf4c80", fn_140cf4c80 as *const (), Throws::Caught), // sub_140cf4c80
    Replacement::draft_throws(0x140cf5500, "content_140cf5500", fn_140cf5500 as *const (), Throws::Caught), // sub_140cf5500
    Replacement::draft(0x140cf59d0, "content_140cf59d0", fn_140cf59d0 as *const ()), // sub_140cf59d0
    Replacement::draft(0x140cf5df0, "content_140cf5df0", fn_140cf5df0 as *const ()), // sub_140cf5df0
    Replacement::draft(0x140cf6190, "content_140cf6190", fn_140cf6190 as *const ()), // sub_140cf6190
    Replacement::draft_throws(0x140cf6530, "content_140cf6530", fn_140cf6530 as *const (), Throws::Discard), // sub_140cf6530
    Replacement::draft_throws(0x140cf68c0, "content_140cf68c0", fn_140cf68c0 as *const (), Throws::Caught), // sub_140cf68c0
    Replacement::draft(0x140cf6ca0, "content_140cf6ca0", fn_140cf6ca0 as *const ()), // sub_140cf6ca0
    Replacement::draft(0x140cf6d40, "content_140cf6d40", tpl_8838::<0x14387f208, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf6d40
    Replacement::draft(0x140cf6de0, "content_140cf6de0", fn_140cf6de0 as *const ()), // sub_140cf6de0
    Replacement::draft_throws(0x140cf6e60, "content_140cf6e60", fn_140cf6e60 as *const (), Throws::Discard), // sub_140cf6e60
    Replacement::draft_throws(0x140cf6f10, "content_140cf6f10", fn_140cf6f10 as *const (), Throws::Caught), // sub_140cf6f10
    Replacement::draft_throws(0x140cf6fd0, "content_140cf6fd0", fn_140cf6fd0 as *const (), Throws::Caught), // sub_140cf6fd0
    Replacement::draft_throws(0x140cf7090, "content_140cf7090", fn_140cf7090 as *const (), Throws::Caught), // sub_140cf7090
    Replacement::draft(0x140cf7150, "content_140cf7150", fn_140cf7150 as *const ()), // sub_140cf7150
    Replacement::draft(0x140cf71c0, "content_140cf71c0", fn_140cf71c0 as *const ()), // sub_140cf71c0
    Replacement::draft(0x140cf7240, "content_140cf7240", fn_140cf7240 as *const ()), // sub_140cf7240
    Replacement::draft_throws(0x140cf72d0, "content_140cf72d0", tpl_13455::<0x14387f3e0, 0x14387f208, 0x1401e9770, 0x338, 0x120, 0x8, 0x1401e9590> as *const (), Throws::Discard), // sub_140cf72d0
    Replacement::draft_throws(0x140cf7360, "content_140cf7360", tpl_16042::<0x14387f208, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x8, 0x1430f4488, 0x1, 0x1402c4e50> as *const (), Throws::Discard), // sub_140cf7360
    Replacement::draft(0x140cf7420, "content_140cf7420", tpl_11191::<0x14387f458, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x48, 0x1426d4580, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140cf7420
    Replacement::draft(0x140cf7500, "content_140cf7500", tpl_69732::<0x14387f458, 0x14243b0b0, 0x338, 0x20, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf7500
    Replacement::draft_throws(0x140cf7580, "content_140cf7580", tpl_11193::<0x14387f1b8, 0x14387f458, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x120, 0x48, 0x1401e9a20> as *const (), Throws::Discard), // sub_140cf7580
    Replacement::draft_throws(0x140cf7660, "content_140cf7660", fn_140cf7660 as *const (), Throws::Caught), // sub_140cf7660
    Replacement::draft_throws(0x140cf7740, "content_140cf7740", fn_140cf7740 as *const (), Throws::Caught), // sub_140cf7740
    Replacement::draft_throws(0x140cf7820, "content_140cf7820", fn_140cf7820 as *const (), Throws::Caught), // sub_140cf7820
    Replacement::draft(0x140cf7900, "content_140cf7900", tpl_11195::<0x14387f458, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf7900
    Replacement::draft(0x140cf79b0, "content_140cf79b0", tpl_2262::<0x14387f1b8, 0x14387f458, 0x1401e9970, 0x338, 0xc0, 0x48, 0xe8, 0x1401e9860> as *const ()), // sub_140cf79b0
    Replacement::draft(0x140cf7a30, "content_140cf7a30", tpl_2258::<0x14387f458, 0x14243b0b0, 0x338, 0xc0, 0x48, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cf7a30
    Replacement::draft_throws(0x140cf7ac0, "content_140cf7ac0", tpl_11196::<0x14387f1b8, 0x14387f458, 0x1401e9770, 0x338, 0x1424b4cb0, 0x48, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140cf7ac0
    Replacement::draft_throws(0x140cf7b40, "content_140cf7b40", tpl_11197::<0x14387f458, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14307eae0, 0x1402c4730, 0x48, 0x1430f4488, 0x1402c4e50> as *const (), Throws::Discard), // sub_140cf7b40
    Replacement::draft(0x140cf7c30, "content_140cf7c30", fn_140cf7c30 as *const ()), // sub_140cf7c30
    Replacement::draft(0x140cf7cd0, "content_140cf7cd0", fn_140cf7cd0 as *const ()), // sub_140cf7cd0
    Replacement::draft(0x140cf7d60, "content_140cf7d60", fn_140cf7d60 as *const ()), // sub_140cf7d60
    Replacement::draft_throws(0x140cf7eb0, "content_140cf7eb0", fn_140cf7eb0 as *const (), Throws::Discard), // sub_140cf7eb0
    Replacement::draft(0x140cf7fb0, "content_140cf7fb0", fn_140cf7fb0 as *const ()), // sub_140cf7fb0
    Replacement::draft_throws(0x140cf8090, "content_140cf8090", fn_140cf8090 as *const (), Throws::Discard), // sub_140cf8090
    Replacement::draft(0x140cf81b0, "content_140cf81b0", fn_140cf81b0 as *const ()), // sub_140cf81b0
    Replacement::draft_throws(0x140cf8270, "content_140cf8270", fn_140cf8270 as *const (), Throws::Caught), // sub_140cf8270
    Replacement::draft_throws(0x140cf83c0, "content_140cf83c0", fn_140cf83c0 as *const (), Throws::Caught), // sub_140cf83c0
    Replacement::draft_throws(0x140cf8510, "content_140cf8510", fn_140cf8510 as *const (), Throws::Caught), // sub_140cf8510
    Replacement::draft(0x140cf8660, "content_140cf8660", fn_140cf8660 as *const ()), // sub_140cf8660
    Replacement::draft(0x140cf8710, "content_140cf8710", fn_140cf8710 as *const ()), // sub_140cf8710
    Replacement::draft(0x140cf87e0, "content_140cf87e0", fn_140cf87e0 as *const ()), // sub_140cf87e0
    Replacement::draft_throws(0x140cf88c0, "content_140cf88c0", fn_140cf88c0 as *const (), Throws::Discard), // sub_140cf88c0
    Replacement::draft_throws(0x140cf89a0, "content_140cf89a0", fn_140cf89a0 as *const (), Throws::Discard), // sub_140cf89a0
    Replacement::draft(0x140cf8ad0, "content_140cf8ad0", tpl_15316::<0x1432e2770, 0x1432e2788, 0x14020f270, 0x1432e2798, 0x4, 0x1432e27b0> as *const ()), // sub_140cf8ad0
    Replacement::draft(0x140cf8b50, "content_140cf8b50", fn_140cf8b50 as *const ()), // sub_140cf8b50
    Replacement::draft(0x140cf8bf0, "content_140cf8bf0", fn_140cf8bf0 as *const ()), // sub_140cf8bf0
    Replacement::draft_throws(0x140cf8cd0, "content_140cf8cd0", fn_140cf8cd0 as *const (), Throws::Discard), // sub_140cf8cd0
    Replacement::draft(0x140cf8d90, "content_140cf8d90", tpl_16055::<0x14387f170, 0x14243b0b0, 0x338, 0x38, 0x318, 0x328, 0x320, 0x8, 0x28, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140cf8d90
    Replacement::draft_throws(0x140cf8e50, "content_140cf8e50", fn_140cf8e50 as *const (), Throws::Discard), // sub_140cf8e50
    Replacement::draft_throws(0x140cf8f20, "content_140cf8f20", fn_140cf8f20 as *const (), Throws::Caught), // sub_140cf8f20
    Replacement::draft_throws(0x140cf9010, "content_140cf9010", fn_140cf9010 as *const (), Throws::Caught), // sub_140cf9010
    Replacement::draft_throws(0x140cf9100, "content_140cf9100", fn_140cf9100 as *const (), Throws::Caught), // sub_140cf9100
    Replacement::draft(0x140cf91f0, "content_140cf91f0", fn_140cf91f0 as *const ()), // sub_140cf91f0
    Replacement::draft(0x140cf92a0, "content_140cf92a0", fn_140cf92a0 as *const ()), // sub_140cf92a0
    Replacement::draft(0x140cf9350, "content_140cf9350", tpl_16055::<0x14387f170, 0x14243b0b0, 0x338, 0xd8, 0x318, 0x328, 0x320, 0x8, 0x28, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140cf9350
    Replacement::draft_throws(0x140cf9410, "content_140cf9410", fn_140cf9410 as *const (), Throws::Discard), // sub_140cf9410
    Replacement::draft_throws(0x140cf94d0, "content_140cf94d0", fn_140cf94d0 as *const (), Throws::Discard), // sub_140cf94d0
    Replacement::draft(0x140cf95b0, "content_140cf95b0", fn_140cf95b0 as *const ()), // sub_140cf95b0
    Replacement::draft(0x140cf9640, "content_140cf9640", fn_140cf9640 as *const ()), // sub_140cf9640
    Replacement::draft_throws(0x140cf9760, "content_140cf9760", fn_140cf9760 as *const (), Throws::Discard), // sub_140cf9760
    Replacement::draft(0x140cf9800, "content_140cf9800", fn_140cf9800 as *const ()), // sub_140cf9800
    Replacement::draft(0x140cf9880, "content_140cf9880", fn_140cf9880 as *const ()), // sub_140cf9880
    Replacement::draft(0x140cf9920, "content_140cf9920", fn_140cf9920 as *const ()), // sub_140cf9920
    Replacement::draft(0x140cf99e0, "content_140cf99e0", fn_140cf99e0 as *const ()), // sub_140cf99e0
    Replacement::draft(0x140cf9a80, "content_140cf9a80", fn_140cf9a80 as *const ()), // sub_140cf9a80
    Replacement::draft_throws(0x140cf9b30, "content_140cf9b30", fn_140cf9b30 as *const (), Throws::Discard), // sub_140cf9b30
    Replacement::draft(0x140cf9bf0, "content_140cf9bf0", fn_140cf9bf0 as *const ()), // sub_140cf9bf0
    Replacement::draft(0x140cf9da0, "content_140cf9da0", fn_140cf9da0 as *const ()), // sub_140cf9da0
    Replacement::draft(0x140cfa140, "content_140cfa140", fn_140cfa140 as *const ()), // sub_140cfa140
    Replacement::draft(0x140cfa970, "content_140cfa970", fn_140cfa970 as *const ()), // sub_140cfa970
    Replacement::draft_throws(0x140cfaf20, "content_140cfaf20", fn_140cfaf20 as *const (), Throws::Discard), // sub_140cfaf20
    Replacement::draft(0x140cfb860, "content_140cfb860", fn_140cfb860 as *const ()), // sub_140cfb860
    Replacement::draft_throws(0x140cfba40, "content_140cfba40", fn_140cfba40 as *const (), Throws::Caught), // sub_140cfba40
    Replacement::draft_throws(0x140cfc340, "content_140cfc340", fn_140cfc340 as *const (), Throws::Caught), // sub_140cfc340
    Replacement::draft_throws(0x140cfcc40, "content_140cfcc40", fn_140cfcc40 as *const (), Throws::Caught), // sub_140cfcc40
    Replacement::draft(0x140cfd540, "content_140cfd540", fn_140cfd540 as *const ()), // sub_140cfd540
    Replacement::draft(0x140cfdb20, "content_140cfdb20", fn_140cfdb20 as *const ()), // sub_140cfdb20
    Replacement::draft_throws(0x140cfe110, "content_140cfe110", fn_140cfe110 as *const (), Throws::Discard), // sub_140cfe110
    Replacement::draft_throws(0x140cfe7f0, "content_140cfe7f0", fn_140cfe7f0 as *const (), Throws::Discard), // sub_140cfe7f0
    Replacement::draft(0x140cff0d0, "content_140cff0d0", fn_140cff0d0 as *const ()), // sub_140cff0d0
    Replacement::draft(0x140cff500, "content_140cff500", tpl_15955::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432d2118, 0x20, 0x8, 0x1c> as *const ()), // sub_140cff500
    Replacement::draft(0x140cff5a0, "content_140cff5a0", tpl_15956::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x1432d2118, 0x5, 0xd, 0xf, 0xffffffffffffffe8> as *const ()), // sub_140cff5a0
    Replacement::draft(0x140cff660, "content_140cff660", fn_140cff660 as *const ()), // sub_140cff660
    Replacement::draft(0x140cff770, "content_140cff770", tpl_15933::<0x14387f370, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cff770
    Replacement::draft_throws(0x140cff840, "content_140cff840", tpl_9957::<0x14387f1bc, 0x14387f370, 0x1401e9fa0, 0x328, 0x338, 0x38, 0x369, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_140cff840
    Replacement::draft(0x140cff8f0, "content_140cff8f0", tpl_9959::<0x14387f370, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cff8f0
    Replacement::draft_throws(0x140cff970, "content_140cff970", tpl_16013::<0x14387f1bc, 0x14387f370, 0x1401e9b60, 0x328, 0x338, 0x120, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140cff970
    Replacement::draft_throws(0x140cffa40, "content_140cffa40", fn_140cffa40 as *const (), Throws::Caught), // sub_140cffa40
    Replacement::draft_throws(0x140cffba0, "content_140cffba0", fn_140cffba0 as *const (), Throws::Caught), // sub_140cffba0
    Replacement::draft_throws(0x140cffd00, "content_140cffd00", fn_140cffd00 as *const (), Throws::Caught), // sub_140cffd00
    Replacement::draft(0x140cffe60, "content_140cffe60", tpl_16015::<0x14387f370, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cffe60
    Replacement::draft(0x140cffed0, "content_140cffed0", tpl_9961::<0x14387f1bc, 0x14387f370, 0x1401e9970, 0x338, 0xd8, 0x4, 0x1401e9860> as *const ()), // sub_140cffed0
    Replacement::draft(0x140cfff50, "content_140cfff50", tpl_9959::<0x14387f370, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140cfff50
    Replacement::draft_throws(0x140cfffe0, "content_140cfffe0", tpl_16016::<0x14387f1bc, 0x14387f370, 0x1401e9770, 0x338, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140cfffe0
    Replacement::draft_throws(0x140d00070, "content_140d00070", fn_140d00070 as *const (), Throws::Discard), // sub_140d00070
    Replacement::draft(0x140d001c0, "content_140d001c0", fn_140d001c0 as *const ()), // sub_140d001c0
    Replacement::draft(0x140d00280, "content_140d00280", fn_140d00280 as *const ()), // sub_140d00280
    Replacement::draft(0x140d00340, "content_140d00340", fn_140d00340 as *const ()), // sub_140d00340
    Replacement::draft_throws(0x140d00490, "content_140d00490", fn_140d00490 as *const (), Throws::Discard), // sub_140d00490
    Replacement::draft(0x140d00590, "content_140d00590", fn_140d00590 as *const ()), // sub_140d00590
    Replacement::draft_throws(0x140d00670, "content_140d00670", fn_140d00670 as *const (), Throws::Discard), // sub_140d00670
    Replacement::draft(0x140d00790, "content_140d00790", fn_140d00790 as *const ()), // sub_140d00790
    Replacement::draft_throws(0x140d008a0, "content_140d008a0", fn_140d008a0 as *const (), Throws::Caught), // sub_140d008a0
    Replacement::draft_throws(0x140d009f0, "content_140d009f0", fn_140d009f0 as *const (), Throws::Caught), // sub_140d009f0
    Replacement::draft_throws(0x140d00b40, "content_140d00b40", fn_140d00b40 as *const (), Throws::Caught), // sub_140d00b40
    Replacement::draft(0x140d00c90, "content_140d00c90", fn_140d00c90 as *const ()), // sub_140d00c90
    Replacement::draft(0x140d00d80, "content_140d00d80", fn_140d00d80 as *const ()), // sub_140d00d80
    Replacement::draft(0x140d00e50, "content_140d00e50", fn_140d00e50 as *const ()), // sub_140d00e50
    Replacement::draft_throws(0x140d00f30, "content_140d00f30", fn_140d00f30 as *const (), Throws::Discard), // sub_140d00f30
    Replacement::draft_throws(0x140d01010, "content_140d01010", fn_140d01010 as *const (), Throws::Discard), // sub_140d01010
    Replacement::draft(0x140d01140, "content_140d01140", fn_140d01140 as *const ()), // sub_140d01140
    Replacement::draft(0x140d011c0, "content_140d011c0", fn_140d011c0 as *const ()), // sub_140d011c0
    Replacement::draft(0x140d01270, "content_140d01270", fn_140d01270 as *const ()), // sub_140d01270
    Replacement::draft(0x140d01340, "content_140d01340", fn_140d01340 as *const ()), // sub_140d01340
    Replacement::draft(0x140d01410, "content_140d01410", fn_140d01410 as *const ()), // sub_140d01410
    Replacement::draft(0x140d014e0, "content_140d014e0", tpl_16098::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8> as *const ()), // sub_140d014e0
    Replacement::draft(0x140d01560, "content_140d01560", fn_140d01560 as *const ()), // sub_140d01560
    Replacement::draft(0x140d01600, "content_140d01600", fn_140d01600 as *const ()), // sub_140d01600
    Replacement::draft(0x140d016c0, "content_140d016c0", fn_140d016c0 as *const ()), // sub_140d016c0
    Replacement::draft(0x140d017f0, "content_140d017f0", fn_140d017f0 as *const ()), // sub_140d017f0
    Replacement::draft(0x140d01880, "content_140d01880", fn_140d01880 as *const ()), // sub_140d01880
    Replacement::draft_throws(0x140d01930, "content_140d01930", fn_140d01930 as *const (), Throws::Caught), // sub_140d01930
    Replacement::draft_throws(0x140d01b00, "content_140d01b00", fn_140d01b00 as *const (), Throws::Caught), // sub_140d01b00
    Replacement::draft_throws(0x140d01db0, "content_140d01db0", fn_140d01db0 as *const (), Throws::Caught), // sub_140d01db0
    Replacement::draft_throws(0x140d020b0, "content_140d020b0", fn_140d020b0 as *const (), Throws::Caught), // sub_140d020b0
    Replacement::draft_throws(0x140d023b0, "content_140d023b0", fn_140d023b0 as *const (), Throws::Caught), // sub_140d023b0
    Replacement::draft_throws(0x140d02810, "content_140d02810", fn_140d02810 as *const (), Throws::Caught), // sub_140d02810
    Replacement::draft_throws(0x140d02c40, "content_140d02c40", fn_140d02c40 as *const (), Throws::Caught), // sub_140d02c40
    Replacement::draft_throws(0x140d03070, "content_140d03070", fn_140d03070 as *const (), Throws::Caught), // sub_140d03070
    Replacement::draft_throws(0x140d03140, "content_140d03140", fn_140d03140 as *const (), Throws::Caught), // sub_140d03140
    Replacement::draft_throws(0x140d03400, "content_140d03400", fn_140d03400 as *const (), Throws::Caught), // sub_140d03400
    Replacement::draft_throws(0x140d036c0, "content_140d036c0", fn_140d036c0 as *const (), Throws::Caught), // sub_140d036c0
    Replacement::draft_throws(0x140d03ae0, "content_140d03ae0", fn_140d03ae0 as *const (), Throws::Caught), // sub_140d03ae0
    Replacement::draft_throws(0x140d03ed0, "content_140d03ed0", fn_140d03ed0 as *const (), Throws::Caught), // sub_140d03ed0
    Replacement::draft_throws(0x140d042b0, "content_140d042b0", fn_140d042b0 as *const (), Throws::Caught), // sub_140d042b0
    Replacement::draft_throws(0x140d04380, "content_140d04380", fn_140d04380 as *const (), Throws::Caught), // sub_140d04380
    Replacement::draft_throws(0x140d04850, "content_140d04850", fn_140d04850 as *const (), Throws::Caught), // sub_140d04850
    Replacement::draft_throws(0x140d04cd0, "content_140d04cd0", fn_140d04cd0 as *const (), Throws::Caught), // sub_140d04cd0
    Replacement::draft_throws(0x140d05150, "content_140d05150", fn_140d05150 as *const (), Throws::Caught), // sub_140d05150
    Replacement::draft_throws(0x140d05620, "content_140d05620", fn_140d05620 as *const (), Throws::Caught), // sub_140d05620
    Replacement::draft_throws(0x140d05aa0, "content_140d05aa0", fn_140d05aa0 as *const (), Throws::Caught), // sub_140d05aa0
    Replacement::draft_throws(0x140d05f20, "content_140d05f20", fn_140d05f20 as *const (), Throws::Caught), // sub_140d05f20
    Replacement::draft_throws(0x140d061b0, "content_140d061b0", fn_140d061b0 as *const (), Throws::Caught), // sub_140d061b0
    Replacement::draft_throws(0x140d06440, "content_140d06440", fn_140d06440 as *const (), Throws::Caught), // sub_140d06440
    Replacement::draft_throws(0x140d066d0, "content_140d066d0", fn_140d066d0 as *const (), Throws::Caught), // sub_140d066d0
    Replacement::draft_throws(0x140d06960, "content_140d06960", fn_140d06960 as *const (), Throws::Caught), // sub_140d06960
    Replacement::draft_throws(0x140d06bb0, "content_140d06bb0", fn_140d06bb0 as *const (), Throws::Caught), // sub_140d06bb0
    Replacement::draft_throws(0x140d06e00, "content_140d06e00", fn_140d06e00 as *const (), Throws::Caught), // sub_140d06e00
    Replacement::draft_throws(0x140d07050, "content_140d07050", fn_140d07050 as *const (), Throws::Caught), // sub_140d07050
    Replacement::draft_throws(0x140d07240, "content_140d07240", fn_140d07240 as *const (), Throws::Caught), // sub_140d07240
    Replacement::draft_throws(0x140d07800, "content_140d07800", fn_140d07800 as *const (), Throws::Caught), // sub_140d07800
    Replacement::draft_throws(0x140d07d70, "content_140d07d70", fn_140d07d70 as *const (), Throws::Caught), // sub_140d07d70
    Replacement::draft_throws(0x140d080c0, "content_140d080c0", fn_140d080c0 as *const (), Throws::Caught), // sub_140d080c0
    Replacement::draft_throws(0x140d08630, "content_140d08630", fn_140d08630 as *const (), Throws::Caught), // sub_140d08630
    Replacement::draft_throws(0x140d08b60, "content_140d08b60", fn_140d08b60 as *const (), Throws::Caught), // sub_140d08b60
    Replacement::draft(0x140d08eb0, "content_140d08eb0", tpl_7826::<0x3, 0x4, 0x8, 0x1431eae88, 0x142409b50> as *const ()), // content::BehaviorTreeChargeAttackNode::content::BehaviorTreeChargeAttackNode::vfunc_22
    Replacement::draft_throws(0x140d08f10, "content_140d08f10", tpl_7307::<0x143cc4f20, 0x143cc4f80, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeChargeAttackNode::content::BehaviorTreeChargeAttackNode::vfunc_23
    Replacement::draft(0x140d09030, "content_140d09030", tpl_7395::<0x3, 0x4, 0x8, 0x1431ead30, 0x142409b50> as *const ()), // content::BehaviorTreeFollowScheduleNode::content::BehaviorTreeFollowScheduleNode::vfunc_22
    Replacement::draft_throws(0x140d09090, "content_140d09090", tpl_7307::<0x143cc4f30, 0x143cc4fc0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeFollowScheduleNode::content::BehaviorTreeFollowScheduleNode::vfunc_23
    Replacement::draft(0x140d091b0, "content_140d091b0", tpl_7391::<0x3, 0x4, 0x8, 0x1431ea820, 0x142409b50> as *const ()), // content::BehaviorTreeMoveToCanDestroyNode::content::BehaviorTreeMoveToCanDestroyNode::vfunc_22
    Replacement::draft_throws(0x140d09210, "content_140d09210", tpl_7307::<0x143cc4f38, 0x143cc4f88, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeMoveToCanDestroyNode::content::BehaviorTreeMoveToCanDestroyNode::vfunc_23
    Replacement::draft(0x140d09330, "content_140d09330", tpl_7387::<0x3, 0x4, 0x8, 0x1431ea7d0, 0x142409b50> as *const ()), // content::BehaviorTreeRushThroughNode::content::BehaviorTreeRushThroughNode::vfunc_22
    Replacement::draft_throws(0x140d09390, "content_140d09390", tpl_7307::<0x143cc4f5c, 0x143cc4f28, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeRushThroughNode::content::BehaviorTreeRushThroughNode::vfunc_23
    Replacement::draft(0x140d094b0, "content_140d094b0", tpl_7386::<0x3, 0x4, 0x8, 0x1431e9e28, 0x142409b50> as *const ()), // content::BehaviorTreeUseMeleeNode::content::BehaviorTreeUseMeleeNode::vfunc_22
    Replacement::draft_throws(0x140d09510, "content_140d09510", tpl_7307::<0x143cc4f58, 0x143cc4f90, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0xb0, 0x8, 0x3> as *const (), Throws::Discard), // content::BehaviorTreeUseMeleeNode::content::BehaviorTreeUseMeleeNode::vfunc_23
    Replacement::draft(0x140d09630, "content_140d09630", tpl_7400::<0x3, 0x4, 0x8, 0x143199118, 0x142409b50> as *const ()), // content::DecoratorCheckAttackCategories::content::DecoratorCheckAttackCategories::vfunc_1
    Replacement::draft_throws(0x140d09690, "content_140d09690", tpl_6639::<0x143cc4f40, 0x143cc4fb0, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::DecoratorCheckAttackCategories::content::DecoratorCheckAttackCategories::vfunc_2
    Replacement::draft(0x140d097b0, "content_140d097b0", tpl_7308::<0x3, 0x4, 0x8, 0x143180478, 0x142409b50> as *const ()), // content::EQSEnemyAdvantageZoneTask::content::EQSEnemyAdvantageZoneTask::vfunc_1
    Replacement::draft_throws(0x140d09810, "content_140d09810", tpl_6639::<0x143cc4f34, 0x143cc4f70, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSEnemyAdvantageZoneTask::content::EQSEnemyAdvantageZoneTask::vfunc_2
    Replacement::draft(0x140d09930, "content_140d09930", tpl_7402::<0x3, 0x4, 0x8, 0x143180458, 0x142409b50> as *const ()), // content::EQSLeashCoreZoneTask::content::EQSLeashCoreZoneTask::vfunc_1
    Replacement::draft_throws(0x140d09990, "content_140d09990", tpl_6639::<0x143cc4f64, 0x143cc4f98, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSLeashCoreZoneTask::content::EQSLeashCoreZoneTask::vfunc_2
    Replacement::draft(0x140d09ab0, "content_140d09ab0", tpl_8673::<0x3, 0x4, 0x8, 0x14317fc48, 0x142409b50> as *const ()), // content::EQSStealthAreaTask::content::EQSStealthAreaTask::vfunc_1
    Replacement::draft_throws(0x140d09b10, "content_140d09b10", tpl_6639::<0x143cc4f60, 0x143cc4fa8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::EQSStealthAreaTask::content::EQSStealthAreaTask::vfunc_2
    Replacement::draft(0x140d09c30, "content_140d09c30", fn_140d09c30 as *const ()), // content::LeadDecorator::content::LeadDecorator::vfunc_1
    Replacement::draft_throws(0x140d09c90, "content_140d09c90", tpl_6639::<0x143cc4fd0, 0x143cc4fc8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::LeadDecorator::content::LeadDecorator::vfunc_2
    Replacement::draft(0x140d09de0, "content_140d09de0", tpl_16123::<0x1432e4268, 0x140d0a430, 0x143267c10, 0x141834d30> as *const ()), // content::npcschedule::Schedule::content::npcschedule::Schedule::vfunc_0
    Replacement::draft(0x140d09e30, "content_140d09e30", tpl_7305::<0x3, 0x4, 0x8, 0x1430add38, 0x142409b50> as *const ()), // content::SendLuaEventDecorator::content::SendLuaEventDecorator::vfunc_1
    Replacement::draft_throws(0x140d09e90, "content_140d09e90", tpl_6639::<0x143cc4f3c, 0x143cc4fd8, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SendLuaEventDecorator::content::SendLuaEventDecorator::vfunc_2
    Replacement::draft(0x140d09fb0, "content_140d09fb0", tpl_7391::<0x3, 0x4, 0x8, 0x1430ad118, 0x142409b50> as *const ()), // content::SetCanEnterStealthAreas::content::SetCanEnterStealthAreas::vfunc_1
    Replacement::draft_throws(0x140d0a010, "content_140d0a010", tpl_6639::<0x143cc4fa0, 0x143cc4f78, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetCanEnterStealthAreas::content::SetCanEnterStealthAreas::vfunc_2
    Replacement::draft(0x140d0a130, "content_140d0a130", tpl_7398::<0x3, 0x4, 0x8, 0x1430ad0f8, 0x142409b50> as *const ()), // content::SetCombatStateDecorator::content::SetCombatStateDecorator::vfunc_1
    Replacement::draft_throws(0x140d0a190, "content_140d0a190", tpl_6639::<0x143cc4f68, 0x143cc4f48, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::SetCombatStateDecorator::content::SetCombatStateDecorator::vfunc_2
    Replacement::draft(0x140d0a2b0, "content_140d0a2b0", tpl_7826::<0x3, 0x4, 0x8, 0x1430aa390, 0x142409b50> as *const ()), // content::StartInteractableObjectSearchDecorator::content::StartInteractableObjectSearchDecorator::vfunc_1
    Replacement::draft_throws(0x140d0a310, "content_140d0a310", tpl_6639::<0x143cc4fb8, 0x143cc4f50, 0x4, 0x1424b0e40, 0x14240ab10, 0x141834a0c, 0x141834a78, 0x8, 0x3> as *const (), Throws::Discard), // content::StartInteractableObjectSearchDecorator::content::StartInteractableObjectSearchDecorator::vfunc_2
    Replacement::draft(0x140d0a430, "content_140d0a430", tpl_2030::<0x140d0a4e0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140d0a430
    Replacement::draft(0x140d0a4e0, "content_140d0a4e0", tpl_16124::<0x8, 0x1403cce10> as *const ()), // sub_140d0a4e0
    Replacement::draft(0x140d0a550, "content_140d0a550", fn_140d0a550 as *const ()), // sub_140d0a550
    Replacement::draft(0x140d0a5e0, "content_140d0a5e0", fn_140d0a5e0 as *const ()), // sub_140d0a5e0
    Replacement::draft(0x140d0a680, "content_140d0a680", fn_140d0a680 as *const ()), // sub_140d0a680
    Replacement::draft(0x140d0a750, "content_140d0a750", fn_140d0a750 as *const ()), // sub_140d0a750
    Replacement::draft_throws(0x140d0a8b0, "content_140d0a8b0", fn_140d0a8b0 as *const (), Throws::Discard), // sub_140d0a8b0
    Replacement::draft(0x140d0a970, "content_140d0a970", fn_140d0a970 as *const ()), // sub_140d0a970
    Replacement::draft_throws(0x140d0aa80, "content_140d0aa80", fn_140d0aa80 as *const (), Throws::Discard), // sub_140d0aa80
    Replacement::draft(0x140d0ab50, "content_140d0ab50", fn_140d0ab50 as *const ()), // sub_140d0ab50
    Replacement::draft_throws(0x140d0abe0, "content_140d0abe0", fn_140d0abe0 as *const (), Throws::Caught), // sub_140d0abe0
    Replacement::draft_throws(0x140d0ae00, "content_140d0ae00", fn_140d0ae00 as *const (), Throws::Caught), // sub_140d0ae00
    Replacement::draft_throws(0x140d0aff0, "content_140d0aff0", fn_140d0aff0 as *const (), Throws::Caught), // sub_140d0aff0
    Replacement::draft(0x140d0b1e0, "content_140d0b1e0", fn_140d0b1e0 as *const ()), // sub_140d0b1e0
    Replacement::draft(0x140d0b2e0, "content_140d0b2e0", fn_140d0b2e0 as *const ()), // sub_140d0b2e0
    Replacement::draft(0x140d0b3d0, "content_140d0b3d0", fn_140d0b3d0 as *const ()), // sub_140d0b3d0
    Replacement::draft_throws(0x140d0b4f0, "content_140d0b4f0", fn_140d0b4f0 as *const (), Throws::Discard), // sub_140d0b4f0
    Replacement::draft_throws(0x140d0b5e0, "content_140d0b5e0", fn_140d0b5e0 as *const (), Throws::Discard), // sub_140d0b5e0
    Replacement::draft(0x140d0b6e0, "content_140d0b6e0", fn_140d0b6e0 as *const ()), // sub_140d0b6e0
    Replacement::draft(0x140d0b7a0, "content_140d0b7a0", fn_140d0b7a0 as *const ()), // sub_140d0b7a0
    Replacement::draft(0x140d0b860, "content_140d0b860", fn_140d0b860 as *const ()), // sub_140d0b860
    Replacement::draft(0x140d0b920, "content_140d0b920", fn_140d0b920 as *const ()), // sub_140d0b920
    Replacement::draft_throws(0x140d0ba80, "content_140d0ba80", fn_140d0ba80 as *const (), Throws::Caught), // sub_140d0ba80
    Replacement::draft(0x140d0bb80, "content_140d0bb80", fn_140d0bb80 as *const ()), // sub_140d0bb80
    Replacement::draft_throws(0x140d0bcd0, "content_140d0bcd0", fn_140d0bcd0 as *const (), Throws::Discard), // sub_140d0bcd0
    Replacement::draft(0x140d0bd80, "content_140d0bd80", fn_140d0bd80 as *const ()), // sub_140d0bd80
    Replacement::draft_throws(0x140d0be50, "content_140d0be50", fn_140d0be50 as *const (), Throws::Caught), // sub_140d0be50
    Replacement::draft_throws(0x140d0c080, "content_140d0c080", fn_140d0c080 as *const (), Throws::Caught), // sub_140d0c080
    Replacement::draft_throws(0x140d0c280, "content_140d0c280", fn_140d0c280 as *const (), Throws::Caught), // sub_140d0c280
    Replacement::draft(0x140d0c480, "content_140d0c480", fn_140d0c480 as *const ()), // sub_140d0c480
    Replacement::draft(0x140d0c640, "content_140d0c640", fn_140d0c640 as *const ()), // sub_140d0c640
    Replacement::draft(0x140d0c750, "content_140d0c750", fn_140d0c750 as *const ()), // sub_140d0c750
    Replacement::draft_throws(0x140d0c870, "content_140d0c870", fn_140d0c870 as *const (), Throws::Discard), // sub_140d0c870
    Replacement::draft_throws(0x140d0c980, "content_140d0c980", fn_140d0c980 as *const (), Throws::Discard), // sub_140d0c980
    Replacement::draft(0x140d0ca70, "content_140d0ca70", fn_140d0ca70 as *const ()), // sub_140d0ca70
    Replacement::draft(0x140d0cb10, "content_140d0cb10", fn_140d0cb10 as *const ()), // sub_140d0cb10
    Replacement::draft(0x140d0cbb0, "content_140d0cbb0", fn_140d0cbb0 as *const ()), // sub_140d0cbb0
    Replacement::draft(0x140d0cc40, "content_140d0cc40", fn_140d0cc40 as *const ()), // sub_140d0cc40
    Replacement::draft(0x140d0ccf0, "content_140d0ccf0", fn_140d0ccf0 as *const ()), // sub_140d0ccf0
    Replacement::draft(0x140d0cdb0, "content_140d0cdb0", fn_140d0cdb0 as *const ()), // sub_140d0cdb0
    Replacement::draft_throws(0x140d0cf90, "content_140d0cf90", fn_140d0cf90 as *const (), Throws::Caught), // sub_140d0cf90
    Replacement::draft(0x140d0d140, "content_140d0d140", fn_140d0d140 as *const ()), // sub_140d0d140
    Replacement::draft_throws(0x140d0d2d0, "content_140d0d2d0", fn_140d0d2d0 as *const (), Throws::Discard), // sub_140d0d2d0
    Replacement::draft(0x140d0d430, "content_140d0d430", fn_140d0d430 as *const ()), // sub_140d0d430
    Replacement::draft_throws(0x140d0d540, "content_140d0d540", fn_140d0d540 as *const (), Throws::Caught), // sub_140d0d540
    Replacement::draft_throws(0x140d0da40, "content_140d0da40", fn_140d0da40 as *const (), Throws::Caught), // sub_140d0da40
    Replacement::draft_throws(0x140d0de80, "content_140d0de80", fn_140d0de80 as *const (), Throws::Caught), // sub_140d0de80
    Replacement::draft(0x140d0e2b0, "content_140d0e2b0", fn_140d0e2b0 as *const ()), // sub_140d0e2b0
    Replacement::draft(0x140d0e4c0, "content_140d0e4c0", fn_140d0e4c0 as *const ()), // sub_140d0e4c0
    Replacement::draft(0x140d0e620, "content_140d0e620", fn_140d0e620 as *const ()), // sub_140d0e620
    Replacement::draft_throws(0x140d0e7a0, "content_140d0e7a0", fn_140d0e7a0 as *const (), Throws::Discard), // sub_140d0e7a0
    Replacement::draft_throws(0x140d0e920, "content_140d0e920", fn_140d0e920 as *const (), Throws::Discard), // sub_140d0e920
    Replacement::draft(0x140d0eaa0, "content_140d0eaa0", fn_140d0eaa0 as *const ()), // sub_140d0eaa0
    Replacement::draft_throws(0x140d0eb80, "content_140d0eb80", fn_140d0eb80 as *const (), Throws::Discard), // sub_140d0eb80
    Replacement::draft(0x140d0ec40, "content_140d0ec40", fn_140d0ec40 as *const ()), // sub_140d0ec40
    Replacement::draft(0x140d0ed20, "content_140d0ed20", fn_140d0ed20 as *const ()), // sub_140d0ed20
    Replacement::draft(0x140d0edc0, "content_140d0edc0", fn_140d0edc0 as *const ()), // sub_140d0edc0
    Replacement::draft(0x140d0ee60, "content_140d0ee60", fn_140d0ee60 as *const ()), // sub_140d0ee60
    Replacement::draft(0x140d0ef60, "content_140d0ef60", fn_140d0ef60 as *const ()), // sub_140d0ef60
    Replacement::draft_throws(0x140d0f160, "content_140d0f160", fn_140d0f160 as *const (), Throws::Caught), // sub_140d0f160
    Replacement::draft(0x140d0f430, "content_140d0f430", fn_140d0f430 as *const ()), // sub_140d0f430
    Replacement::draft_throws(0x140d0f6c0, "content_140d0f6c0", fn_140d0f6c0 as *const (), Throws::Discard), // sub_140d0f6c0
    Replacement::draft(0x140d0f910, "content_140d0f910", fn_140d0f910 as *const ()), // sub_140d0f910
    Replacement::draft_throws(0x140d0fa10, "content_140d0fa10", fn_140d0fa10 as *const (), Throws::Caught), // sub_140d0fa10
    Replacement::draft_throws(0x140d10040, "content_140d10040", fn_140d10040 as *const (), Throws::Caught), // sub_140d10040
    Replacement::draft_throws(0x140d10640, "content_140d10640", fn_140d10640 as *const (), Throws::Caught), // sub_140d10640
    Replacement::draft(0x140d10c30, "content_140d10c30", fn_140d10c30 as *const ()), // sub_140d10c30
    Replacement::draft(0x140d10e30, "content_140d10e30", fn_140d10e30 as *const ()), // sub_140d10e30
    Replacement::draft(0x140d10f90, "content_140d10f90", fn_140d10f90 as *const ()), // sub_140d10f90
    Replacement::draft_throws(0x140d11110, "content_140d11110", fn_140d11110 as *const (), Throws::Discard), // sub_140d11110
    Replacement::draft_throws(0x140d11280, "content_140d11280", fn_140d11280 as *const (), Throws::Discard), // sub_140d11280
    Replacement::draft(0x140d11440, "content_140d11440", fn_140d11440 as *const ()), // sub_140d11440
    Replacement::draft_throws(0x140d11520, "content_140d11520", fn_140d11520 as *const (), Throws::Caught), // sub_140d11520
    Replacement::draft(0x140d11650, "content_140d11650", fn_140d11650 as *const ()), // sub_140d11650
    Replacement::draft_throws(0x140d11730, "content_140d11730", fn_140d11730 as *const (), Throws::Discard), // sub_140d11730
    Replacement::draft_throws(0x140d117f0, "content_140d117f0", fn_140d117f0 as *const (), Throws::Caught), // sub_140d117f0
    Replacement::draft_throws(0x140d11b50, "content_140d11b50", fn_140d11b50 as *const (), Throws::Caught), // sub_140d11b50
    Replacement::draft_throws(0x140d11e50, "content_140d11e50", fn_140d11e50 as *const (), Throws::Caught), // sub_140d11e50
    Replacement::draft(0x140d12140, "content_140d12140", fn_140d12140 as *const ()), // sub_140d12140
    Replacement::draft(0x140d122b0, "content_140d122b0", fn_140d122b0 as *const ()), // sub_140d122b0
    Replacement::draft(0x140d12370, "content_140d12370", fn_140d12370 as *const ()), // sub_140d12370
    Replacement::draft_throws(0x140d12440, "content_140d12440", fn_140d12440 as *const (), Throws::Discard), // sub_140d12440
    Replacement::draft_throws(0x140d124f0, "content_140d124f0", fn_140d124f0 as *const (), Throws::Discard), // sub_140d124f0
    Replacement::draft(0x140d125c0, "content_140d125c0", fn_140d125c0 as *const ()), // sub_140d125c0
    Replacement::draft(0x140d126a0, "content_140d126a0", tpl_15990::<0x1432dab30, 0x1426cdcb0, 0x1432daa00, 0x14020f270, 0x143426a48, 0x1432daa90, 0x1432daa10, 0x1432daab8, 0x1432daa18, 0x143426b30, 0x1432dabd0, 0x1432daa20, 0x143426b20, 0x1432dabf8, 0x1432da9e0, 0x143426ac0, 0x1432dab58, 0x1432da9e8, 0x143426970, 0x1432dab80, 0x1432ba3e0, 0x1432da9f0> as *const ()), // sub_140d126a0
    Replacement::draft(0x140d12860, "content_140d12860", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431f1d70> as *const ()), // sub_140d12860
    Replacement::draft(0x140d128c0, "content_140d128c0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431f1d70> as *const ()), // sub_140d128c0
    Replacement::draft_throws(0x140d129c0, "content_140d129c0", fn_140d129c0 as *const (), Throws::Caught), // sub_140d129c0
    Replacement::draft_throws(0x140d12a50, "content_140d12a50", tpl_6762::<0x14387f518, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140d12a50
    Replacement::draft(0x140d12b00, "content_140d12b00", tpl_16193::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x44, 0x4c, 0x58, 0x1432e4fe0, 0x8> as *const ()), // sub_140d12b00
    Replacement::draft(0x140d12bb0, "content_140d12bb0", fn_140d12bb0 as *const ()), // sub_140d12bb0
    Replacement::draft(0x140d12ce0, "content_140d12ce0", fn_140d12ce0 as *const ()), // sub_140d12ce0
    Replacement::draft_throws(0x140d12eb0, "content_140d12eb0", fn_140d12eb0 as *const (), Throws::Caught), // sub_140d12eb0
    Replacement::draft_throws(0x140d130e0, "content_140d130e0", fn_140d130e0 as *const (), Throws::Discard), // sub_140d130e0
    Replacement::draft_throws(0x140d13240, "content_140d13240", fn_140d13240 as *const (), Throws::Discard), // sub_140d13240
    Replacement::draft_throws(0x140d13420, "content_140d13420", fn_140d13420 as *const (), Throws::Discard), // sub_140d13420
    Replacement::draft_throws(0x140d134e0, "content_140d134e0", fn_140d134e0 as *const (), Throws::Caught), // sub_140d134e0
    Replacement::draft_throws(0x140d13a50, "content_140d13a50", fn_140d13a50 as *const (), Throws::Caught), // sub_140d13a50
    Replacement::draft_throws(0x140d13fc0, "content_140d13fc0", fn_140d13fc0 as *const (), Throws::Caught), // sub_140d13fc0
    Replacement::draft(0x140d14540, "content_140d14540", fn_140d14540 as *const ()), // sub_140d14540
    Replacement::draft(0x140d147c0, "content_140d147c0", fn_140d147c0 as *const ()), // sub_140d147c0
    Replacement::draft(0x140d14940, "content_140d14940", fn_140d14940 as *const ()), // sub_140d14940
    Replacement::draft_throws(0x140d14aa0, "content_140d14aa0", fn_140d14aa0 as *const (), Throws::Discard), // sub_140d14aa0
    Replacement::draft_throws(0x140d14c30, "content_140d14c30", fn_140d14c30 as *const (), Throws::Discard), // sub_140d14c30
    Replacement::draft(0x140d14e20, "content_140d14e20", tpl_7496::<0x1407b5410, 0x14384ecb8, 0x40, 0x1426cdcb0, 0x1432e37f0, 0x1401de410> as *const ()), // sub_140d14e20
    Replacement::draft(0x140d14e90, "content_140d14e90", fn_140d14e90 as *const ()), // sub_140d14e90
    Replacement::draft(0x140d14f10, "content_140d14f10", tpl_7496::<0x1407b54e0, 0x14384ecb8, 0x40, 0x1426cdcb0, 0x1432e37f0, 0x1401de410> as *const ()), // sub_140d14f10
    Replacement::draft(0x140d14f80, "content_140d14f80", tpl_8296::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432e50a8, 0x20, 0x28, 0x30> as *const ()), // sub_140d14f80
    Replacement::draft(0x140d15020, "content_140d15020", tpl_8297::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432e50a8, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4, 0x1c> as *const ()), // sub_140d15020
    Replacement::draft_throws(0x140d15340, "content_140d15340", fn_140d15340 as *const (), Throws::Caught), // sub_140d15340
    Replacement::draft_throws(0x140d153e0, "content_140d153e0", fn_140d153e0 as *const (), Throws::Caught), // sub_140d153e0
    Replacement::draft_throws(0x140d15480, "content_140d15480", fn_140d15480 as *const (), Throws::Caught), // sub_140d15480
    Replacement::draft_throws(0x140d15720, "content_140d15720", tpl_8493::<0x14387f500, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d160, 0x1407b5310> as *const (), Throws::Discard), // sub_140d15720
    Replacement::draft(0x140d157f0, "content_140d157f0", fn_140d157f0 as *const ()), // sub_140d157f0
    Replacement::draft(0x140d15930, "content_140d15930", fn_140d15930 as *const ()), // sub_140d15930
    Replacement::draft(0x140d15a80, "content_140d15a80", tpl_6797::<0x14387f4c8, 0x14243b0b0, 0x1407c0230, 0x14387d230, 0x50, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140d15a80
    Replacement::draft(0x140d15b20, "content_140d15b20", fn_140d15b20 as *const ()), // sub_140d15b20
    Replacement::draft_throws(0x140d15d10, "content_140d15d10", fn_140d15d10 as *const (), Throws::Caught), // sub_140d15d10
    Replacement::draft_throws(0x140d15f20, "content_140d15f20", fn_140d15f20 as *const (), Throws::Discard), // sub_140d15f20
    Replacement::draft_throws(0x140d16060, "content_140d16060", fn_140d16060 as *const (), Throws::Discard), // sub_140d16060
    Replacement::draft_throws(0x140d16260, "content_140d16260", fn_140d16260 as *const (), Throws::Discard), // sub_140d16260
    Replacement::draft_throws(0x140d16380, "content_140d16380", fn_140d16380 as *const (), Throws::Caught), // sub_140d16380
    Replacement::draft_throws(0x140d16a20, "content_140d16a20", fn_140d16a20 as *const (), Throws::Caught), // sub_140d16a20
    Replacement::draft_throws(0x140d17000, "content_140d17000", fn_140d17000 as *const (), Throws::Caught), // sub_140d17000
    Replacement::draft(0x140d175d0, "content_140d175d0", fn_140d175d0 as *const ()), // sub_140d175d0
    Replacement::draft(0x140d177a0, "content_140d177a0", fn_140d177a0 as *const ()), // sub_140d177a0
    Replacement::draft(0x140d17920, "content_140d17920", fn_140d17920 as *const ()), // sub_140d17920
    Replacement::draft_throws(0x140d17a80, "content_140d17a80", fn_140d17a80 as *const (), Throws::Discard), // sub_140d17a80
    Replacement::draft_throws(0x140d17c00, "content_140d17c00", fn_140d17c00 as *const (), Throws::Discard), // sub_140d17c00
    Replacement::draft(0x140d17e00, "content_140d17e00", tpl_16222::<0x1407b5410, 0x14384ebc8, 0x3c, 0x1432e37d8, 0x1, 0x1401de410, 0x14384ec98, 0x38, 0x1432e37c0, 0x14384ebe8, 0x48, 0x1426cdcb0, 0x1432e3948> as *const ()), // sub_140d17e00
    Replacement::draft(0x140d17ec0, "content_140d17ec0", fn_140d17ec0 as *const ()), // sub_140d17ec0
    Replacement::draft(0x140d17fa0, "content_140d17fa0", tpl_16222::<0x1407b54e0, 0x14384ebc8, 0x3c, 0x1432e37d8, 0x1, 0x1401de410, 0x14384ec98, 0x38, 0x1432e37c0, 0x14384ebe8, 0x48, 0x1426cdcb0, 0x1432e3948> as *const ()), // sub_140d17fa0
    Replacement::draft(0x140d18060, "content_140d18060", fn_140d18060 as *const ()), // sub_140d18060
    Replacement::draft(0x140d18190, "content_140d18190", fn_140d18190 as *const ()), // sub_140d18190
    Replacement::draft(0x140d182d0, "content_140d182d0", fn_140d182d0 as *const ()), // sub_140d182d0
    Replacement::draft(0x140d18370, "content_140d18370", fn_140d18370 as *const ()), // sub_140d18370
    Replacement::draft_throws(0x140d185f0, "content_140d185f0", fn_140d185f0 as *const (), Throws::Discard), // sub_140d185f0
    Replacement::draft_throws(0x140d18820, "content_140d18820", fn_140d18820 as *const (), Throws::Discard), // sub_140d18820
    Replacement::draft_throws(0x140d18950, "content_140d18950", fn_140d18950 as *const (), Throws::Discard), // sub_140d18950
    Replacement::draft_throws(0x140d18bf0, "content_140d18bf0", fn_140d18bf0 as *const (), Throws::Discard), // sub_140d18bf0
    Replacement::draft_throws(0x140d18d50, "content_140d18d50", fn_140d18d50 as *const (), Throws::Caught), // sub_140d18d50
    Replacement::draft_throws(0x140d193c0, "content_140d193c0", fn_140d193c0 as *const (), Throws::Caught), // sub_140d193c0
    Replacement::draft_throws(0x140d19960, "content_140d19960", fn_140d19960 as *const (), Throws::Caught), // sub_140d19960
    Replacement::draft(0x140d19f10, "content_140d19f10", fn_140d19f10 as *const ()), // sub_140d19f10
    Replacement::draft(0x140d1a020, "content_140d1a020", fn_140d1a020 as *const ()), // sub_140d1a020
    Replacement::draft(0x140d1a1a0, "content_140d1a1a0", fn_140d1a1a0 as *const ()), // sub_140d1a1a0
    Replacement::draft_throws(0x140d1a300, "content_140d1a300", fn_140d1a300 as *const (), Throws::Discard), // sub_140d1a300
    Replacement::draft_throws(0x140d1a4b0, "content_140d1a4b0", fn_140d1a4b0 as *const (), Throws::Discard), // sub_140d1a4b0
    Replacement::draft(0x140d1a780, "content_140d1a780", tpl_16222::<0x1407b5410, 0x14384ebc8, 0x3c, 0x1432e37d8, 0x1, 0x1401de410, 0x14384ec98, 0x38, 0x1432e37c0, 0x14384ebe8, 0x40, 0x1426cdcb0, 0x1432e3948> as *const ()), // sub_140d1a780
    Replacement::draft(0x140d1a840, "content_140d1a840", fn_140d1a840 as *const ()), // sub_140d1a840
    Replacement::draft(0x140d1aa20, "content_140d1aa20", tpl_16222::<0x1407b54e0, 0x14384ebc8, 0x3c, 0x1432e37d8, 0x1, 0x1401de410, 0x14384ec98, 0x38, 0x1432e37c0, 0x14384ebe8, 0x40, 0x1426cdcb0, 0x1432e3948> as *const ()), // sub_140d1aa20
    Replacement::draft(0x140d1aae0, "content_140d1aae0", tpl_16193::<0x68, 0x10, 0x18, 0xffffffffffffffb0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x44, 0x4c, 0x58, 0x1432e4d58, 0x8> as *const ()), // sub_140d1aae0
    Replacement::draft(0x140d1ab90, "content_140d1ab90", fn_140d1ab90 as *const ()), // sub_140d1ab90
    Replacement::draft(0x140d1ac50, "content_140d1ac50", fn_140d1ac50 as *const ()), // sub_140d1ac50
    Replacement::draft(0x140d1acc0, "content_140d1acc0", fn_140d1acc0 as *const ()), // sub_140d1acc0
    Replacement::draft_throws(0x140d1adb0, "content_140d1adb0", fn_140d1adb0 as *const (), Throws::Caught), // sub_140d1adb0
    Replacement::draft_throws(0x140d1aeb0, "content_140d1aeb0", fn_140d1aeb0 as *const (), Throws::Discard), // sub_140d1aeb0
    Replacement::draft_throws(0x140d1af80, "content_140d1af80", fn_140d1af80 as *const (), Throws::Discard), // sub_140d1af80
    Replacement::draft_throws(0x140d1b030, "content_140d1b030", fn_140d1b030 as *const (), Throws::Discard), // sub_140d1b030
    Replacement::draft_throws(0x140d1b0c0, "content_140d1b0c0", fn_140d1b0c0 as *const (), Throws::Caught), // sub_140d1b0c0
    Replacement::draft_throws(0x140d1b2f0, "content_140d1b2f0", fn_140d1b2f0 as *const (), Throws::Caught), // sub_140d1b2f0
    Replacement::draft_throws(0x140d1b5d0, "content_140d1b5d0", fn_140d1b5d0 as *const (), Throws::Caught), // sub_140d1b5d0
    Replacement::draft(0x140d1b8b0, "content_140d1b8b0", fn_140d1b8b0 as *const ()), // sub_140d1b8b0
    Replacement::draft(0x140d1ba10, "content_140d1ba10", fn_140d1ba10 as *const ()), // sub_140d1ba10
    Replacement::draft(0x140d1bad0, "content_140d1bad0", fn_140d1bad0 as *const ()), // sub_140d1bad0
    Replacement::draft_throws(0x140d1bba0, "content_140d1bba0", fn_140d1bba0 as *const (), Throws::Discard), // sub_140d1bba0
    Replacement::draft_throws(0x140d1bc50, "content_140d1bc50", fn_140d1bc50 as *const (), Throws::Discard), // sub_140d1bc50
    Replacement::draft(0x140d1bd30, "content_140d1bd30", fn_140d1bd30 as *const ()), // sub_140d1bd30
    Replacement::draft(0x140d1bdb0, "content_140d1bdb0", fn_140d1bdb0 as *const ()), // sub_140d1bdb0
    Replacement::draft(0x140d1bec0, "content_140d1bec0", fn_140d1bec0 as *const ()), // sub_140d1bec0
    Replacement::draft_throws(0x140d1bf90, "content_140d1bf90", fn_140d1bf90 as *const (), Throws::Discard), // sub_140d1bf90
    Replacement::draft(0x140d1c050, "content_140d1c050", tpl_16257::<0x14387f538, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xc, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1c050
    Replacement::draft_throws(0x140d1c0e0, "content_140d1c0e0", fn_140d1c0e0 as *const (), Throws::Discard), // sub_140d1c0e0
    Replacement::draft_throws(0x140d1c1b0, "content_140d1c1b0", fn_140d1c1b0 as *const (), Throws::Caught), // sub_140d1c1b0
    Replacement::draft_throws(0x140d1c2d0, "content_140d1c2d0", fn_140d1c2d0 as *const (), Throws::Caught), // sub_140d1c2d0
    Replacement::draft_throws(0x140d1c3f0, "content_140d1c3f0", fn_140d1c3f0 as *const (), Throws::Caught), // sub_140d1c3f0
    Replacement::draft(0x140d1c510, "content_140d1c510", fn_140d1c510 as *const ()), // sub_140d1c510
    Replacement::draft(0x140d1c590, "content_140d1c590", tpl_71593::<0x14387f50c, 0x14387f538, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0xc, 0xd8, 0x1401e9860> as *const ()), // sub_140d1c590
    Replacement::draft(0x140d1c620, "content_140d1c620", tpl_16257::<0x14387f538, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xc, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1c620
    Replacement::draft_throws(0x140d1c6b0, "content_140d1c6b0", fn_140d1c6b0 as *const (), Throws::Discard), // sub_140d1c6b0
    Replacement::draft_throws(0x140d1c750, "content_140d1c750", fn_140d1c750 as *const (), Throws::Discard), // sub_140d1c750
    Replacement::draft(0x140d1c890, "content_140d1c890", tpl_7861::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432e6cc0, 0x20> as *const ()), // sub_140d1c890
    Replacement::draft(0x140d1c920, "content_140d1c920", tpl_7862::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432e6cc0, 0xffffffffffffffec, 0x4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140d1c920
    Replacement::draft(0x140d1ca40, "content_140d1ca40", tpl_8069::<0x14387f520, 0x14243b0b0, 0x1406ea470, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1ca40
    Replacement::draft_throws(0x140d1caf0, "content_140d1caf0", tpl_7851::<0x14387f5e0, 0x14387f520, 0x1401e9fa0, 0x1406ea600, 0x328, 0x338, 0x20, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d1caf0
    Replacement::draft_throws(0x140d1cb80, "content_140d1cb80", tpl_7852::<0x14387f520, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140d1cb80
    Replacement::draft_throws(0x140d1cc00, "content_140d1cc00", tpl_7972::<0x14387f5e0, 0x14387f520, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x20, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d1cc00
    Replacement::draft_throws(0x140d1cc90, "content_140d1cc90", fn_140d1cc90 as *const (), Throws::Caught), // sub_140d1cc90
    Replacement::draft_throws(0x140d1cd60, "content_140d1cd60", fn_140d1cd60 as *const (), Throws::Caught), // sub_140d1cd60
    Replacement::draft_throws(0x140d1cf20, "content_140d1cf20", fn_140d1cf20 as *const (), Throws::Caught), // sub_140d1cf20
    Replacement::draft(0x140d1d0e0, "content_140d1d0e0", tpl_7855::<0x14387f520, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1d0e0
    Replacement::draft(0x140d1d160, "content_140d1d160", tpl_8074::<0x14387f5e0, 0x14387f520, 0x1401e9970, 0x1406ec510, 0x20, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140d1d160
    Replacement::draft(0x140d1d1e0, "content_140d1d1e0", tpl_8075::<0x14387f520, 0x14243b0b0, 0x1406ec600, 0x20, 0x338, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1d1e0
    Replacement::draft_throws(0x140d1d270, "content_140d1d270", tpl_7974::<0x14387f5e0, 0x14387f520, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140d1d270
    Replacement::draft_throws(0x140d1d2f0, "content_140d1d2f0", tpl_8076::<0x14387f520, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0, 0x20, 0x1432e36f0> as *const (), Throws::Discard), // sub_140d1d2f0
    Replacement::draft(0x140d1d3b0, "content_140d1d3b0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143197a78> as *const ()), // sub_140d1d3b0
    Replacement::draft(0x140d1d410, "content_140d1d410", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143197a78> as *const ()), // sub_140d1d410
    Replacement::draft_throws(0x140d1d510, "content_140d1d510", fn_140d1d510 as *const (), Throws::Caught), // sub_140d1d510
    Replacement::draft_throws(0x140d1d5a0, "content_140d1d5a0", tpl_6763::<0x14387f540, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140d1d5a0
    Replacement::draft(0x140d1d650, "content_140d1d650", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143180770> as *const ()), // sub_140d1d650
    Replacement::draft(0x140d1d6b0, "content_140d1d6b0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143180770> as *const ()), // sub_140d1d6b0
    Replacement::draft_throws(0x140d1d7b0, "content_140d1d7b0", fn_140d1d7b0 as *const (), Throws::Caught), // sub_140d1d7b0
    Replacement::draft_throws(0x140d1d840, "content_140d1d840", tpl_6763::<0x14387f558, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140d1d840
    Replacement::draft(0x140d1d8f0, "content_140d1d8f0", tpl_8768::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432e61b8> as *const ()), // sub_140d1d8f0
    Replacement::draft(0x140d1d980, "content_140d1d980", tpl_16266::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432e61b8, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_140d1d980
    Replacement::draft(0x140d1da90, "content_140d1da90", tpl_16267::<0x14387f498, 0x14243b0b0, 0x140758e80, 0x1426d4580> as *const ()), // sub_140d1da90
    Replacement::draft_throws(0x140d1dbd0, "content_140d1dbd0", tpl_16268::<0x14387f4b0, 0x14387f498, 0x1401e9fa0, 0x140758fc0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d1dbd0
    Replacement::draft(0x140d1dd10, "content_140d1dd10", tpl_16269::<0x14387f498, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x48, 0x14, 0x80, 0x15, 0x18, 0x1c, 0x38> as *const ()), // sub_140d1dd10
    Replacement::draft_throws(0x140d1dde0, "content_140d1dde0", tpl_16270::<0x14387f4b0, 0x14387f498, 0x1401e9b60, 0x140759220, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d1dde0
    Replacement::draft(0x140d1df40, "content_140d1df40", fn_140d1df40 as *const ()), // sub_140d1df40
    Replacement::draft_throws(0x140d1e000, "content_140d1e000", fn_140d1e000 as *const (), Throws::Caught), // sub_140d1e000
    Replacement::draft_throws(0x140d1e430, "content_140d1e430", fn_140d1e430 as *const (), Throws::Caught), // sub_140d1e430
    Replacement::draft_throws(0x140d1e7d0, "content_140d1e7d0", fn_140d1e7d0 as *const (), Throws::Caught), // sub_140d1e7d0
    Replacement::draft(0x140d1eb70, "content_140d1eb70", tpl_74644::<0x14387f498, 0x14243b0b0, 0x140759da0, 0x10, 0x14, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1eb70
    Replacement::draft(0x140d1ebf0, "content_140d1ebf0", tpl_16276::<0x14387f4b0, 0x14387f498, 0x1401e9970, 0x140759f40, 0x10, 0x338, 0xe8, 0x14, 0x120, 0x15, 0x18, 0x1c, 0x1401e9860> as *const ()), // sub_140d1ebf0
    Replacement::draft(0x140d1ecd0, "content_140d1ecd0", tpl_16277::<0x14387f498, 0x14243b0b0, 0x14075a030, 0x10, 0x338, 0xe8, 0x14, 0x120, 0x15, 0x18, 0x1c, 0xd8> as *const ()), // sub_140d1ecd0
    Replacement::draft_throws(0x140d1edb0, "content_140d1edb0", tpl_16278::<0x14387f4b0, 0x14387f498, 0x1401e9770, 0x14075a130, 0x10, 0x338, 0x120, 0x14, 0x15, 0x18, 0x1c, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140d1edb0
    Replacement::draft_throws(0x140d1eea0, "content_140d1eea0", tpl_16279::<0x14387f498, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14075a230, 0x1432e36e0, 0x1426cdcb0, 0x1432e3460, 0x1432e3440, 0x14020f0f0, 0x1432e3490, 0x1432e3470, 0x1432e3410, 0x1432e3438, 0x1432e3430, 0x14020f240> as *const (), Throws::Discard), // sub_140d1eea0
    Replacement::draft(0x140d1f040, "content_140d1f040", tpl_16280::<0x14075a380, 0x14384ec80, 0x1426cdcb0, 0x143245bf0, 0x1, 0x1432e36e0, 0x1401de410, 0x14384ec58, 0x1432e3420, 0x1432e3410> as *const ()), // sub_140d1f040
    Replacement::draft(0x140d1f0e0, "content_140d1f0e0", tpl_16281::<0x14075a400, 0x14384ec80, 0x1426cdcb0, 0x143245bf0, 0x10, 0x1, 0x1432e36e0, 0x1401de410, 0x14384ec58, 0x18, 0x1432e3420, 0x1432e3410, 0x1432e3438, 0x1432e3430, 0x14020f270> as *const ()), // sub_140d1f0e0
    Replacement::draft(0x140d1f1a0, "content_140d1f1a0", tpl_16280::<0x14075a4d0, 0x14384ec80, 0x1426cdcb0, 0x143245bf0, 0x1, 0x1432e36e0, 0x1401de410, 0x14384ec58, 0x1432e3420, 0x1432e3410> as *const ()), // sub_140d1f1a0
    Replacement::draft(0x140d1f240, "content_140d1f240", fn_140d1f240 as *const ()), // sub_140d1f240
    Replacement::draft(0x140d1f2e0, "content_140d1f2e0", fn_140d1f2e0 as *const ()), // sub_140d1f2e0
    Replacement::draft(0x140d1f390, "content_140d1f390", tpl_7312::<0x14387f5c8, 0x14243b0b0, 0x140758e00, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d1f390
    Replacement::draft(0x140d1f400, "content_140d1f400", fn_140d1f400 as *const ()), // sub_140d1f400
    Replacement::draft_throws(0x140d1f520, "content_140d1f520", fn_140d1f520 as *const (), Throws::Discard), // sub_140d1f520
    Replacement::draft(0x140d1f630, "content_140d1f630", tpl_16286::<0x14387f5c8, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x38, 0x14, 0x18, 0x80, 0x19, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140d1f630
    Replacement::draft_throws(0x140d1f6f0, "content_140d1f6f0", fn_140d1f6f0 as *const (), Throws::Discard), // sub_140d1f6f0
    Replacement::draft(0x140d1f820, "content_140d1f820", fn_140d1f820 as *const ()), // sub_140d1f820
    Replacement::draft_throws(0x140d1f8d0, "content_140d1f8d0", fn_140d1f8d0 as *const (), Throws::Caught), // sub_140d1f8d0
    Replacement::draft_throws(0x140d1fa60, "content_140d1fa60", fn_140d1fa60 as *const (), Throws::Caught), // sub_140d1fa60
    Replacement::draft_throws(0x140d1fbf0, "content_140d1fbf0", fn_140d1fbf0 as *const (), Throws::Caught), // sub_140d1fbf0
    Replacement::draft(0x140d1fd80, "content_140d1fd80", fn_140d1fd80 as *const ()), // sub_140d1fd80
    Replacement::draft(0x140d1fe30, "content_140d1fe30", fn_140d1fe30 as *const ()), // sub_140d1fe30
    Replacement::draft(0x140d1fee0, "content_140d1fee0", tpl_16286::<0x14387f5c8, 0x14243b0b0, 0x14075a030, 0x338, 0x10, 0xd8, 0x14, 0x18, 0x120, 0x19, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140d1fee0
    Replacement::draft_throws(0x140d1ffa0, "content_140d1ffa0", fn_140d1ffa0 as *const (), Throws::Discard), // sub_140d1ffa0
    Replacement::draft_throws(0x140d20070, "content_140d20070", fn_140d20070 as *const (), Throws::Discard), // sub_140d20070
    Replacement::draft(0x140d201f0, "content_140d201f0", fn_140d201f0 as *const ()), // sub_140d201f0
    Replacement::draft(0x140d20290, "content_140d20290", tpl_8768::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432e4918> as *const ()), // sub_140d20290
    Replacement::draft(0x140d20320, "content_140d20320", tpl_16266::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432e4918, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x20> as *const ()), // sub_140d20320
    Replacement::draft(0x140d20430, "content_140d20430", tpl_16267::<0x14387f4d0, 0x14243b0b0, 0x140758e80, 0x1426d4580> as *const ()), // sub_140d20430
    Replacement::draft_throws(0x140d20570, "content_140d20570", tpl_16268::<0x14387f594, 0x14387f4d0, 0x1401e9fa0, 0x140758fc0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d20570
    Replacement::draft(0x140d206b0, "content_140d206b0", tpl_16269::<0x14387f4d0, 0x14243b0b0, 0x140759110, 0x338, 0x10, 0x48, 0x14, 0x80, 0x15, 0x18, 0x1c, 0x38> as *const ()), // sub_140d206b0
    Replacement::draft_throws(0x140d20780, "content_140d20780", tpl_16270::<0x14387f594, 0x14387f4d0, 0x1401e9b60, 0x140759220, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d20780
    Replacement::draft_throws(0x140d208e0, "content_140d208e0", fn_140d208e0 as *const (), Throws::Caught), // sub_140d208e0
    Replacement::draft_throws(0x140d20d10, "content_140d20d10", fn_140d20d10 as *const (), Throws::Caught), // sub_140d20d10
    Replacement::draft_throws(0x140d210b0, "content_140d210b0", fn_140d210b0 as *const (), Throws::Caught), // sub_140d210b0
    Replacement::draft(0x140d21450, "content_140d21450", tpl_74644::<0x14387f4d0, 0x14243b0b0, 0x140759da0, 0x10, 0x14, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d21450
    Replacement::draft(0x140d214d0, "content_140d214d0", tpl_16276::<0x14387f594, 0x14387f4d0, 0x1401e9970, 0x140759f40, 0x10, 0x338, 0xe8, 0x14, 0x120, 0x15, 0x18, 0x1c, 0x1401e9860> as *const ()), // sub_140d214d0
    Replacement::draft(0x140d215b0, "content_140d215b0", tpl_16277::<0x14387f4d0, 0x14243b0b0, 0x14075a030, 0x10, 0x338, 0xe8, 0x14, 0x120, 0x15, 0x18, 0x1c, 0xd8> as *const ()), // sub_140d215b0
    Replacement::draft_throws(0x140d21690, "content_140d21690", tpl_16278::<0x14387f594, 0x14387f4d0, 0x1401e9770, 0x14075a130, 0x10, 0x338, 0x120, 0x14, 0x15, 0x18, 0x1c, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140d21690
    Replacement::draft_throws(0x140d21780, "content_140d21780", tpl_16279::<0x14387f4d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14075a230, 0x1432e34c0, 0x1426cdcb0, 0x1432e3460, 0x1432e34a0, 0x14020f0f0, 0x1432e3490, 0x1432e34e0, 0x1432e34d0, 0x1432e3438, 0x1432e32a8, 0x14020f240> as *const (), Throws::Discard), // sub_140d21780
    Replacement::draft(0x140d21920, "content_140d21920", tpl_16280::<0x14075a380, 0x14384ec80, 0x1426cdcb0, 0x143245bf0, 0x1, 0x1432e34c0, 0x1401de410, 0x14384ec58, 0x1432e3420, 0x1432e34d0> as *const ()), // sub_140d21920
    Replacement::draft(0x140d219c0, "content_140d219c0", tpl_16281::<0x14075a400, 0x14384ec80, 0x1426cdcb0, 0x143245bf0, 0x10, 0x1, 0x1432e34c0, 0x1401de410, 0x14384ec58, 0x18, 0x1432e3420, 0x1432e34d0, 0x1432e3438, 0x1432e32a8, 0x14020f270> as *const ()), // sub_140d219c0
    Replacement::draft(0x140d21a80, "content_140d21a80", tpl_16280::<0x14075a4d0, 0x14384ec80, 0x1426cdcb0, 0x143245bf0, 0x1, 0x1432e34c0, 0x1401de410, 0x14384ec58, 0x1432e3420, 0x1432e34d0> as *const ()), // sub_140d21a80
    Replacement::draft(0x140d21b20, "content_140d21b20", tpl_8270::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140d29f30> as *const ()), // sub_140d21b20
    Replacement::draft(0x140d21bb0, "content_140d21bb0", tpl_67091::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140d29f30> as *const ()), // sub_140d21bb0
    Replacement::draft(0x140d21c60, "content_140d21c60", fn_140d21c60 as *const ()), // sub_140d21c60
    Replacement::draft(0x140d21d50, "content_140d21d50", fn_140d21d50 as *const ()), // sub_140d21d50
    Replacement::draft_throws(0x140d21fe0, "content_140d21fe0", fn_140d21fe0 as *const (), Throws::Discard), // sub_140d21fe0
    Replacement::draft_throws(0x140d22250, "content_140d22250", fn_140d22250 as *const (), Throws::Discard), // sub_140d22250
    Replacement::draft_throws(0x140d223c0, "content_140d223c0", fn_140d223c0 as *const (), Throws::Discard), // sub_140d223c0
    Replacement::draft_throws(0x140d226d0, "content_140d226d0", fn_140d226d0 as *const (), Throws::Discard), // sub_140d226d0
    Replacement::draft_throws(0x140d228d0, "content_140d228d0", fn_140d228d0 as *const (), Throws::Caught), // sub_140d228d0
    Replacement::draft_throws(0x140d22c90, "content_140d22c90", fn_140d22c90 as *const (), Throws::Caught), // sub_140d22c90
    Replacement::draft_throws(0x140d23050, "content_140d23050", fn_140d23050 as *const (), Throws::Caught), // sub_140d23050
    Replacement::draft(0x140d23410, "content_140d23410", fn_140d23410 as *const ()), // sub_140d23410
    Replacement::draft(0x140d235a0, "content_140d235a0", fn_140d235a0 as *const ()), // sub_140d235a0
    Replacement::draft(0x140d23720, "content_140d23720", fn_140d23720 as *const ()), // sub_140d23720
    Replacement::draft_throws(0x140d238a0, "content_140d238a0", fn_140d238a0 as *const (), Throws::Discard), // sub_140d238a0
    Replacement::draft_throws(0x140d23a80, "content_140d23a80", fn_140d23a80 as *const (), Throws::Discard), // sub_140d23a80
    Replacement::draft(0x140d23dc0, "content_140d23dc0", fn_140d23dc0 as *const ()), // sub_140d23dc0
    Replacement::draft(0x140d23fd0, "content_140d23fd0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430ff190, 0x8> as *const ()), // sub_140d23fd0
    Replacement::draft(0x140d24050, "content_140d24050", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430ff190> as *const ()), // sub_140d24050
    Replacement::draft(0x140d240e0, "content_140d240e0", tpl_9031::<0x14387f5f0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d240e0
    Replacement::draft(0x140d24150, "content_140d24150", tpl_12078::<0x14387f5f0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d24150
    Replacement::draft_throws(0x140d241f0, "content_140d241f0", tpl_12079::<0x14387f56c, 0x14387f5f0, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d241f0
    Replacement::draft(0x140d24280, "content_140d24280", tpl_12080::<0x14387f5f0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d24280
    Replacement::draft_throws(0x140d24300, "content_140d24300", tpl_12081::<0x14387f56c, 0x14387f5f0, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d24300
    Replacement::draft_throws(0x140d24390, "content_140d24390", fn_140d24390 as *const (), Throws::Caught), // sub_140d24390
    Replacement::draft_throws(0x140d24470, "content_140d24470", fn_140d24470 as *const (), Throws::Caught), // sub_140d24470
    Replacement::draft_throws(0x140d24550, "content_140d24550", fn_140d24550 as *const (), Throws::Caught), // sub_140d24550
    Replacement::draft(0x140d24630, "content_140d24630", tpl_12083::<0x14387f5f0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d24630
    Replacement::draft(0x140d246a0, "content_140d246a0", tpl_12084::<0x14387f56c, 0x14387f5f0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140d246a0
    Replacement::draft(0x140d24710, "content_140d24710", tpl_12080::<0x14387f5f0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d24710
    Replacement::draft_throws(0x140d24790, "content_140d24790", tpl_12085::<0x14387f56c, 0x14387f5f0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140d24790
    Replacement::draft_throws(0x140d24810, "content_140d24810", tpl_12086::<0x14387f5f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432e32f0, 0x1432ba220, 0x1432e32d8, 0x14020f0f0> as *const (), Throws::Discard), // sub_140d24810
    Replacement::draft(0x140d24910, "content_140d24910", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1430fdc78> as *const ()), // sub_140d24910
    Replacement::draft(0x140d249a0, "content_140d249a0", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430fdc78, 0x18> as *const ()), // sub_140d249a0
    Replacement::draft(0x140d24a50, "content_140d24a50", fn_140d24a50 as *const ()), // sub_140d24a50
    Replacement::draft(0x140d24b30, "content_140d24b30", fn_140d24b30 as *const ()), // sub_140d24b30
    Replacement::draft_throws(0x140d24ce0, "content_140d24ce0", fn_140d24ce0 as *const (), Throws::Discard), // sub_140d24ce0
    Replacement::draft(0x140d24e60, "content_140d24e60", fn_140d24e60 as *const ()), // sub_140d24e60
    Replacement::draft_throws(0x140d24f80, "content_140d24f80", fn_140d24f80 as *const (), Throws::Discard), // sub_140d24f80
    Replacement::draft_throws(0x140d25130, "content_140d25130", fn_140d25130 as *const (), Throws::Caught), // sub_140d25130
    Replacement::draft_throws(0x140d25300, "content_140d25300", fn_140d25300 as *const (), Throws::Caught), // sub_140d25300
    Replacement::draft_throws(0x140d254d0, "content_140d254d0", fn_140d254d0 as *const (), Throws::Caught), // sub_140d254d0
    Replacement::draft(0x140d256a0, "content_140d256a0", fn_140d256a0 as *const ()), // sub_140d256a0
    Replacement::draft(0x140d25790, "content_140d25790", fn_140d25790 as *const ()), // sub_140d25790
    Replacement::draft(0x140d258b0, "content_140d258b0", fn_140d258b0 as *const ()), // sub_140d258b0
    Replacement::draft_throws(0x140d259e0, "content_140d259e0", fn_140d259e0 as *const (), Throws::Discard), // sub_140d259e0
    Replacement::draft_throws(0x140d25b20, "content_140d25b20", fn_140d25b20 as *const (), Throws::Discard), // sub_140d25b20
    Replacement::draft(0x140d25d00, "content_140d25d00", fn_140d25d00 as *const ()), // sub_140d25d00
    Replacement::draft(0x140d25de0, "content_140d25de0", tpl_7938::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432e6438> as *const ()), // sub_140d25de0
    Replacement::draft(0x140d25e70, "content_140d25e70", tpl_7939::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432e6438, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140d25e70
    Replacement::draft_throws(0x140d26060, "content_140d26060", fn_140d26060 as *const (), Throws::Caught), // sub_140d26060
    Replacement::draft_throws(0x140d26100, "content_140d26100", fn_140d26100 as *const (), Throws::Caught), // sub_140d26100
    Replacement::draft_throws(0x140d261a0, "content_140d261a0", fn_140d261a0 as *const (), Throws::Caught), // sub_140d261a0
    Replacement::draft_throws(0x140d26320, "content_140d26320", tpl_7326::<0x14387f4a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1406ec7e0> as *const (), Throws::Discard), // sub_140d26320
    Replacement::draft(0x140d263c0, "content_140d263c0", tpl_7861::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432e7000, 0x20> as *const ()), // sub_140d263c0
    Replacement::draft(0x140d26450, "content_140d26450", fn_140d26450 as *const ()), // sub_140d26450
    Replacement::draft(0x140d26570, "content_140d26570", fn_140d26570 as *const ()), // sub_140d26570
    Replacement::draft_throws(0x140d26650, "content_140d26650", fn_140d26650 as *const (), Throws::Discard), // sub_140d26650
    Replacement::draft_throws(0x140d26740, "content_140d26740", tpl_8136::<0x14387f5e8, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x80, 0x21, 0x24, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const (), Throws::Discard), // sub_140d26740
    Replacement::draft_throws(0x140d267e0, "content_140d267e0", fn_140d267e0 as *const (), Throws::Discard), // sub_140d267e0
    Replacement::draft_throws(0x140d268e0, "content_140d268e0", fn_140d268e0 as *const (), Throws::Discard), // sub_140d268e0
    Replacement::draft_throws(0x140d269b0, "content_140d269b0", fn_140d269b0 as *const (), Throws::Caught), // sub_140d269b0
    Replacement::draft_throws(0x140d26c30, "content_140d26c30", fn_140d26c30 as *const (), Throws::Caught), // sub_140d26c30
    Replacement::draft_throws(0x140d26e80, "content_140d26e80", fn_140d26e80 as *const (), Throws::Caught), // sub_140d26e80
    Replacement::draft(0x140d270d0, "content_140d270d0", fn_140d270d0 as *const ()), // sub_140d270d0
    Replacement::draft(0x140d27150, "content_140d27150", fn_140d27150 as *const ()), // sub_140d27150
    Replacement::draft(0x140d27200, "content_140d27200", fn_140d27200 as *const ()), // sub_140d27200
    Replacement::draft_throws(0x140d272b0, "content_140d272b0", tpl_6832::<0x14387f4b4, 0x14387f5e8, 0x1401e9770, 0x1406ec700, 0x338, 0x20, 0x120, 0x21, 0x24, 0x1401e9590> as *const (), Throws::Discard), // sub_140d272b0
    Replacement::draft_throws(0x140d27360, "content_140d27360", fn_140d27360 as *const (), Throws::Discard), // sub_140d27360
    Replacement::draft(0x140d274a0, "content_140d274a0", tpl_7496::<0x1406ec8f0, 0x14384ec30, 0x24, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140d274a0
    Replacement::draft(0x140d27510, "content_140d27510", tpl_7496::<0x1406eca20, 0x14384ec30, 0x24, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // sub_140d27510
    Replacement::draft(0x140d27580, "content_140d27580", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432e4318, 0x20> as *const ()), // sub_140d27580
    Replacement::draft(0x140d27610, "content_140d27610", tpl_7835::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432e4318, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0xffffffffffffffe4> as *const ()), // sub_140d27610
    Replacement::draft(0x140d27730, "content_140d27730", tpl_8185::<0x14387f4f8, 0x14243b0b0, 0x1406ea470, 0x338, 0x20, 0x1426d4580, 0x1, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d27730
    Replacement::draft_throws(0x140d27810, "content_140d27810", tpl_8179::<0x14387f4a8, 0x14387f4f8, 0x1401e9fa0, 0x1406ea600, 0x328, 0x20, 0x338, 0x48, 0x369, 0x24, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d27810
    Replacement::draft_throws(0x140d278d0, "content_140d278d0", tpl_8021::<0x14387f4f8, 0x14243b0b0, 0x1406ea780, 0x338, 0x20, 0x48, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140d278d0
    Replacement::draft_throws(0x140d27960, "content_140d27960", tpl_8186::<0x14387f4a8, 0x14387f4f8, 0x1401e9b60, 0x1406ea8d0, 0x328, 0x338, 0x120, 0x369, 0x20, 0x24, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d27960
    Replacement::draft_throws(0x140d27a30, "content_140d27a30", fn_140d27a30 as *const (), Throws::Discard), // sub_140d27a30
    Replacement::draft_throws(0x140d27ae0, "content_140d27ae0", fn_140d27ae0 as *const (), Throws::Caught), // sub_140d27ae0
    Replacement::draft_throws(0x140d27e20, "content_140d27e20", fn_140d27e20 as *const (), Throws::Caught), // sub_140d27e20
    Replacement::draft_throws(0x140d28100, "content_140d28100", fn_140d28100 as *const (), Throws::Caught), // sub_140d28100
    Replacement::draft(0x140d283e0, "content_140d283e0", tpl_8182::<0x14387f4f8, 0x14243b0b0, 0x1406ec310, 0x338, 0x20, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d283e0
    Replacement::draft(0x140d28470, "content_140d28470", fn_140d28470 as *const ()), // sub_140d28470
    Replacement::draft(0x140d28510, "content_140d28510", fn_140d28510 as *const ()), // sub_140d28510
    Replacement::draft_throws(0x140d285c0, "content_140d285c0", fn_140d285c0 as *const (), Throws::Discard), // sub_140d285c0
    Replacement::draft_throws(0x140d28650, "content_140d28650", fn_140d28650 as *const (), Throws::Discard), // sub_140d28650
    Replacement::draft(0x140d28730, "content_140d28730", tpl_8092::<0x1406ec8f0, 0x1426cdcb0, 0x14384eb80, 0x20, 0x1432d7820, 0x1, 0x1401de410, 0x14384ec30, 0x24, 0x143251798> as *const ()), // sub_140d28730
    Replacement::draft(0x140d287c0, "content_140d287c0", fn_140d287c0 as *const ()), // sub_140d287c0
    Replacement::draft(0x140d28850, "content_140d28850", tpl_8092::<0x1406eca20, 0x1426cdcb0, 0x14384eb80, 0x20, 0x1432d7820, 0x1, 0x1401de410, 0x14384ec30, 0x24, 0x143251798> as *const ()), // sub_140d28850
    Replacement::draft(0x140d288e0, "content_140d288e0", fn_140d288e0 as *const ()), // sub_140d288e0
    Replacement::draft(0x140d28980, "content_140d28980", fn_140d28980 as *const ()), // sub_140d28980
    Replacement::draft(0x140d28aa0, "content_140d28aa0", fn_140d28aa0 as *const ()), // sub_140d28aa0
    Replacement::draft_throws(0x140d28bb0, "content_140d28bb0", fn_140d28bb0 as *const (), Throws::Discard), // sub_140d28bb0
    Replacement::draft_throws(0x140d28ca0, "content_140d28ca0", fn_140d28ca0 as *const (), Throws::Discard), // sub_140d28ca0
    Replacement::draft_throws(0x140d28d40, "content_140d28d40", fn_140d28d40 as *const (), Throws::Discard), // sub_140d28d40
    Replacement::draft_throws(0x140d28e40, "content_140d28e40", fn_140d28e40 as *const (), Throws::Discard), // sub_140d28e40
    Replacement::draft_throws(0x140d28f00, "content_140d28f00", fn_140d28f00 as *const (), Throws::Caught), // sub_140d28f00
    Replacement::draft_throws(0x140d292a0, "content_140d292a0", fn_140d292a0 as *const (), Throws::Caught), // sub_140d292a0
    Replacement::draft_throws(0x140d295d0, "content_140d295d0", fn_140d295d0 as *const (), Throws::Caught), // sub_140d295d0
    Replacement::draft(0x140d298f0, "content_140d298f0", fn_140d298f0 as *const ()), // sub_140d298f0
    Replacement::draft(0x140d29990, "content_140d29990", fn_140d29990 as *const ()), // sub_140d29990
    Replacement::draft(0x140d29a40, "content_140d29a40", fn_140d29a40 as *const ()), // sub_140d29a40
    Replacement::draft_throws(0x140d29b00, "content_140d29b00", fn_140d29b00 as *const (), Throws::Discard), // sub_140d29b00
    Replacement::draft_throws(0x140d29bb0, "content_140d29bb0", fn_140d29bb0 as *const (), Throws::Discard), // sub_140d29bb0
    Replacement::draft(0x140d29ce0, "content_140d29ce0", tpl_16356::<0x1406ec8f0, 0x1426cdcb0, 0x14384ec10, 0x20, 0x1432b5510, 0x1401de410, 0x14384ec30, 0x28, 0x143251450> as *const ()), // sub_140d29ce0
    Replacement::draft(0x140d29d70, "content_140d29d70", fn_140d29d70 as *const ()), // sub_140d29d70
    Replacement::draft(0x140d29e40, "content_140d29e40", tpl_16356::<0x1406eca20, 0x1426cdcb0, 0x14384ec10, 0x20, 0x1432b5510, 0x1401de410, 0x14384ec30, 0x28, 0x143251450> as *const ()), // sub_140d29e40
    Replacement::draft(0x140d29ed0, "content_140d29ed0", fn_140d29ed0 as *const ()), // content::BehaviorTreeUseMeleeNode::content::BehaviorTreeUseMeleeNode::vfunc_0
    Replacement::draft(0x140d29f30, "content_140d29f30", tpl_8242::<0x1432e6f40, 0x8, 0x10, 0x18, 0x78, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88> as *const ()), // sub_140d29f30
    Replacement::draft(0x140d2a000, "content_140d2a000", fn_140d2a000 as *const ()), // content::BehaviorTreeMoveToCanDestroyNode::content::BehaviorTreeMoveToCanDestroyNode::vfunc_0
    Replacement::draft(0x140d2a080, "content_140d2a080", fn_140d2a080 as *const ()), // content::BehaviorTreeRushThroughNode::content::BehaviorTreeRushThroughNode::vfunc_0
    Replacement::draft(0x140d2a100, "content_140d2a100", tpl_16361::<0x140d0a4e0, 0x8, 0xc, 0x140d2c290, 0x140d2a1d0> as *const ()), // sub_140d2a100
    Replacement::draft(0x140d2a1d0, "content_140d2a1d0", fn_140d2a1d0 as *const ()), // sub_140d2a1d0
    Replacement::draft(0x140d2a2b0, "content_140d2a2b0", fn_140d2a2b0 as *const ()), // sub_140d2a2b0
    Replacement::draft_throws(0x140d2a3a0, "content_140d2a3a0", fn_140d2a3a0 as *const (), Throws::Caught), // sub_140d2a3a0
    Replacement::draft_throws(0x140d2a550, "content_140d2a550", fn_140d2a550 as *const (), Throws::Caught), // sub_140d2a550
    Replacement::draft_throws(0x140d2a710, "content_140d2a710", fn_140d2a710 as *const (), Throws::Caught), // sub_140d2a710
    Replacement::draft_throws(0x140d2ab90, "content_140d2ab90", fn_140d2ab90 as *const (), Throws::Caught), // sub_140d2ab90
    Replacement::draft_throws(0x140d2ad40, "content_140d2ad40", fn_140d2ad40 as *const (), Throws::Caught), // sub_140d2ad40
    Replacement::draft_throws(0x140d2af70, "content_140d2af70", fn_140d2af70 as *const (), Throws::Caught), // sub_140d2af70
    Replacement::draft_throws(0x140d2b120, "content_140d2b120", fn_140d2b120 as *const (), Throws::Caught), // sub_140d2b120
    Replacement::draft_throws(0x140d2b350, "content_140d2b350", fn_140d2b350 as *const (), Throws::Caught), // sub_140d2b350
    Replacement::draft_throws(0x140d2b500, "content_140d2b500", fn_140d2b500 as *const (), Throws::Caught), // sub_140d2b500
    Replacement::draft_throws(0x140d2b730, "content_140d2b730", fn_140d2b730 as *const (), Throws::Caught), // sub_140d2b730
    Replacement::draft_throws(0x140d2ba30, "content_140d2ba30", fn_140d2ba30 as *const (), Throws::Caught), // sub_140d2ba30
    Replacement::draft_throws(0x140d2bd30, "content_140d2bd30", fn_140d2bd30 as *const (), Throws::Caught), // sub_140d2bd30
    Replacement::draft(0x140d2c030, "content_140d2c030", tpl_16370::<0x8, 0xc, 0x140d2c290, 0x1403cce10> as *const ()), // sub_140d2c030
    Replacement::draft(0x140d2c0e0, "content_140d2c0e0", fn_140d2c0e0 as *const ()), // sub_140d2c0e0
    Replacement::draft(0x140d2c1b0, "content_140d2c1b0", fn_140d2c1b0 as *const ()), // sub_140d2c1b0
    Replacement::draft(0x140d2c290, "content_140d2c290", tpl_16373::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140d33ee0, 0x1403cce10, 0x1425371a0, 0x142533200> as *const ()), // sub_140d2c290
    Replacement::draft(0x140d2c490, "content_140d2c490", fn_140d2c490 as *const ()), // sub_140d2c490
    Replacement::draft(0x140d2c6b0, "content_140d2c6b0", fn_140d2c6b0 as *const ()), // sub_140d2c6b0
    Replacement::draft_throws(0x140d2c8a0, "content_140d2c8a0", fn_140d2c8a0 as *const (), Throws::Caught), // sub_140d2c8a0
    Replacement::draft_throws(0x140d2cb80, "content_140d2cb80", fn_140d2cb80 as *const (), Throws::Caught), // sub_140d2cb80
    Replacement::draft_throws(0x140d2ce30, "content_140d2ce30", fn_140d2ce30 as *const (), Throws::Caught), // sub_140d2ce30
    Replacement::draft_throws(0x140d2d0d0, "content_140d2d0d0", fn_140d2d0d0 as *const (), Throws::Caught), // sub_140d2d0d0
    Replacement::draft_throws(0x140d2d1c0, "content_140d2d1c0", fn_140d2d1c0 as *const (), Throws::Caught), // sub_140d2d1c0
    Replacement::draft_throws(0x140d2d4a0, "content_140d2d4a0", fn_140d2d4a0 as *const (), Throws::Caught), // sub_140d2d4a0
    Replacement::draft_throws(0x140d2d750, "content_140d2d750", fn_140d2d750 as *const (), Throws::Caught), // sub_140d2d750
    Replacement::draft_throws(0x140d2d9f0, "content_140d2d9f0", fn_140d2d9f0 as *const (), Throws::Caught), // sub_140d2d9f0
    Replacement::draft_throws(0x140d2dae0, "content_140d2dae0", fn_140d2dae0 as *const (), Throws::Caught), // sub_140d2dae0
    Replacement::draft_throws(0x140d2ddc0, "content_140d2ddc0", fn_140d2ddc0 as *const (), Throws::Caught), // sub_140d2ddc0
    Replacement::draft_throws(0x140d2e070, "content_140d2e070", fn_140d2e070 as *const (), Throws::Caught), // sub_140d2e070
    Replacement::draft_throws(0x140d2e310, "content_140d2e310", fn_140d2e310 as *const (), Throws::Caught), // sub_140d2e310
    Replacement::draft(0x140d2e3e0, "content_140d2e3e0", tpl_16378 as *const ()), // sub_140d2e3e0
    Replacement::draft(0x140d2e410, "content_140d2e410", tpl_16379 as *const ()), // sub_140d2e410
    Replacement::draft_throws(0x140d2e440, "content_140d2e440", fn_140d2e440 as *const (), Throws::Caught), // sub_140d2e440
    Replacement::draft_throws(0x140d2e720, "content_140d2e720", fn_140d2e720 as *const (), Throws::Caught), // sub_140d2e720
    Replacement::draft_throws(0x140d2e9d0, "content_140d2e9d0", fn_140d2e9d0 as *const (), Throws::Caught), // sub_140d2e9d0
    Replacement::draft_throws(0x140d2ec70, "content_140d2ec70", fn_140d2ec70 as *const (), Throws::Caught), // sub_140d2ec70
    Replacement::draft(0x140d2ed40, "content_140d2ed40", fn_140d2ed40 as *const ()), // sub_140d2ed40
    Replacement::draft(0x140d2ede0, "content_140d2ede0", fn_140d2ede0 as *const ()), // sub_140d2ede0
    Replacement::draft(0x140d2eea0, "content_140d2eea0", fn_140d2eea0 as *const ()), // sub_140d2eea0
    Replacement::draft(0x140d2eee0, "content_140d2eee0", tpl_4095::<0x8, 0x10, 0x1403ccec0> as *const ()), // sub_140d2eee0
    Replacement::draft(0x140d2ef20, "content_140d2ef20", tpl_16383::<0x8, 0x1403cce10> as *const ()), // sub_140d2ef20
    Replacement::draft_throws(0x140d2ef60, "content_140d2ef60", fn_140d2ef60 as *const (), Throws::Caught), // sub_140d2ef60
    Replacement::draft_throws(0x140d2f430, "content_140d2f430", fn_140d2f430 as *const (), Throws::Caught), // sub_140d2f430
    Replacement::draft_throws(0x140d2f8c0, "content_140d2f8c0", fn_140d2f8c0 as *const (), Throws::Caught), // sub_140d2f8c0
    Replacement::draft_throws(0x140d2fd50, "content_140d2fd50", fn_140d2fd50 as *const (), Throws::Caught), // sub_140d2fd50
    Replacement::draft_throws(0x140d30220, "content_140d30220", fn_140d30220 as *const (), Throws::Caught), // sub_140d30220
    Replacement::draft_throws(0x140d306b0, "content_140d306b0", fn_140d306b0 as *const (), Throws::Caught), // sub_140d306b0
    Replacement::draft_throws(0x140d30b40, "content_140d30b40", fn_140d30b40 as *const (), Throws::Caught), // sub_140d30b40
    Replacement::draft_throws(0x140d31010, "content_140d31010", fn_140d31010 as *const (), Throws::Caught), // sub_140d31010
    Replacement::draft_throws(0x140d314a0, "content_140d314a0", fn_140d314a0 as *const (), Throws::Caught), // sub_140d314a0
    Replacement::draft_throws(0x140d31930, "content_140d31930", fn_140d31930 as *const (), Throws::Caught), // sub_140d31930
    Replacement::draft_throws(0x140d31e00, "content_140d31e00", fn_140d31e00 as *const (), Throws::Caught), // sub_140d31e00
    Replacement::draft_throws(0x140d32290, "content_140d32290", fn_140d32290 as *const (), Throws::Caught), // sub_140d32290
    Replacement::draft_throws(0x140d32720, "content_140d32720", fn_140d32720 as *const (), Throws::Caught), // sub_140d32720
    Replacement::draft_throws(0x140d32860, "content_140d32860", fn_140d32860 as *const (), Throws::Caught), // sub_140d32860
    Replacement::draft_throws(0x140d329a0, "content_140d329a0", fn_140d329a0 as *const (), Throws::Caught), // sub_140d329a0
    Replacement::draft_throws(0x140d32ae0, "content_140d32ae0", fn_140d32ae0 as *const (), Throws::Caught), // sub_140d32ae0
    Replacement::draft_throws(0x140d32d60, "content_140d32d60", fn_140d32d60 as *const (), Throws::Caught), // sub_140d32d60
    Replacement::draft_throws(0x140d32ea0, "content_140d32ea0", fn_140d32ea0 as *const (), Throws::Caught), // sub_140d32ea0
    Replacement::draft_throws(0x140d32fe0, "content_140d32fe0", fn_140d32fe0 as *const (), Throws::Caught), // sub_140d32fe0
    Replacement::draft_throws(0x140d33120, "content_140d33120", fn_140d33120 as *const (), Throws::Caught), // sub_140d33120
    Replacement::draft_throws(0x140d33310, "content_140d33310", fn_140d33310 as *const (), Throws::Caught), // sub_140d33310
    Replacement::draft_throws(0x140d33450, "content_140d33450", fn_140d33450 as *const (), Throws::Caught), // sub_140d33450
    Replacement::draft_throws(0x140d33590, "content_140d33590", fn_140d33590 as *const (), Throws::Caught), // sub_140d33590
    Replacement::draft_throws(0x140d336d0, "content_140d336d0", fn_140d336d0 as *const (), Throws::Caught), // sub_140d336d0
    Replacement::draft_throws(0x140d338c0, "content_140d338c0", fn_140d338c0 as *const (), Throws::Caught), // sub_140d338c0
    Replacement::draft_throws(0x140d33a00, "content_140d33a00", fn_140d33a00 as *const (), Throws::Caught), // sub_140d33a00
    Replacement::draft_throws(0x140d33b40, "content_140d33b40", fn_140d33b40 as *const (), Throws::Caught), // sub_140d33b40
    Replacement::draft_throws(0x140d33c80, "content_140d33c80", fn_140d33c80 as *const (), Throws::Caught), // sub_140d33c80
    Replacement::draft(0x140d33ee0, "content_140d33ee0", tpl_855::<0x1403ccf70, 0x1425371a0, 0x142533200, 0x140d2c490, 0x140d2ed40> as *const ()), // sub_140d33ee0
    Replacement::draft(0x140d34040, "content_140d34040", tpl_855::<0x1403ccfe0, 0x1425371a0, 0x142533200, 0x140d2c6b0, 0x140d2ede0> as *const ()), // sub_140d34040
    Replacement::draft_throws(0x140d341a0, "content_140d341a0", fn_140d341a0 as *const (), Throws::Caught), // sub_140d341a0
    Replacement::draft_throws(0x140d34610, "content_140d34610", fn_140d34610 as *const (), Throws::Caught), // sub_140d34610
    Replacement::draft_throws(0x140d34a40, "content_140d34a40", fn_140d34a40 as *const (), Throws::Caught), // sub_140d34a40
    Replacement::draft_throws(0x140d34d90, "content_140d34d90", fn_140d34d90 as *const (), Throws::Caught), // sub_140d34d90
    Replacement::draft_throws(0x140d35200, "content_140d35200", fn_140d35200 as *const (), Throws::Caught), // sub_140d35200
    Replacement::draft_throws(0x140d35630, "content_140d35630", fn_140d35630 as *const (), Throws::Caught), // sub_140d35630
    Replacement::draft_throws(0x140d35980, "content_140d35980", fn_140d35980 as *const (), Throws::Caught), // sub_140d35980
    Replacement::draft_throws(0x140d35df0, "content_140d35df0", fn_140d35df0 as *const (), Throws::Caught), // sub_140d35df0
    Replacement::draft_throws(0x140d36220, "content_140d36220", fn_140d36220 as *const (), Throws::Caught), // sub_140d36220
    Replacement::draft_throws(0x140d36570, "content_140d36570", fn_140d36570 as *const (), Throws::Caught), // sub_140d36570
    Replacement::draft_throws(0x140d369e0, "content_140d369e0", fn_140d369e0 as *const (), Throws::Caught), // sub_140d369e0
    Replacement::draft_throws(0x140d36e10, "content_140d36e10", fn_140d36e10 as *const (), Throws::Caught), // sub_140d36e10
    Replacement::draft(0x140d37180, "content_140d37180", fn_140d37180 as *const ()), // sub_140d37180
    Replacement::draft(0x140d371e0, "content_140d371e0", fn_140d371e0 as *const ()), // sub_140d371e0
    Replacement::draft(0x140d37280, "content_140d37280", fn_140d37280 as *const ()), // sub_140d37280
    Replacement::draft_throws(0x140d37300, "content_140d37300", fn_140d37300 as *const (), Throws::Discard), // sub_140d37300
    Replacement::draft_throws(0x140d373d0, "content_140d373d0", fn_140d373d0 as *const (), Throws::Discard), // sub_140d373d0
    Replacement::draft_throws(0x140d374a0, "content_140d374a0", fn_140d374a0 as *const (), Throws::Discard), // sub_140d374a0
    Replacement::draft(0x140d37530, "content_140d37530", fn_140d37530 as *const ()), // sub_140d37530
    Replacement::draft_throws(0x140d375a0, "content_140d375a0", fn_140d375a0 as *const (), Throws::Caught), // sub_140d375a0
    Replacement::draft_throws(0x140d37690, "content_140d37690", fn_140d37690 as *const (), Throws::Caught), // sub_140d37690
    Replacement::draft_throws(0x140d37780, "content_140d37780", fn_140d37780 as *const (), Throws::Caught), // sub_140d37780
    Replacement::draft(0x140d37870, "content_140d37870", fn_140d37870 as *const ()), // sub_140d37870
    Replacement::draft(0x140d379a0, "content_140d379a0", fn_140d379a0 as *const ()), // sub_140d379a0
    Replacement::draft(0x140d37aa0, "content_140d37aa0", fn_140d37aa0 as *const ()), // sub_140d37aa0
    Replacement::draft_throws(0x140d37ba0, "content_140d37ba0", fn_140d37ba0 as *const (), Throws::Discard), // sub_140d37ba0
    Replacement::draft_throws(0x140d37cc0, "content_140d37cc0", fn_140d37cc0 as *const (), Throws::Discard), // sub_140d37cc0
    Replacement::draft(0x140d37d90, "content_140d37d90", fn_140d37d90 as *const ()), // sub_140d37d90
    Replacement::draft(0x140d37e80, "content_140d37e80", fn_140d37e80 as *const ()), // sub_140d37e80
    Replacement::draft_throws(0x140d37f40, "content_140d37f40", fn_140d37f40 as *const (), Throws::Caught), // sub_140d37f40
    Replacement::draft_throws(0x140d37fe0, "content_140d37fe0", fn_140d37fe0 as *const (), Throws::Caught), // sub_140d37fe0
    Replacement::draft_throws(0x140d38080, "content_140d38080", fn_140d38080 as *const (), Throws::Caught), // sub_140d38080
    Replacement::draft_throws(0x140d38130, "content_140d38130", tpl_16403::<0x20, 0x1402e3480> as *const (), Throws::Discard), // sub_140d38130
    Replacement::draft_throws(0x140d38160, "content_140d38160", fn_140d38160 as *const (), Throws::Caught), // sub_140d38160
    Replacement::draft(0x140d381d0, "content_140d381d0", fn_140d381d0 as *const ()), // sub_140d381d0
    Replacement::draft(0x140d38280, "content_140d38280", fn_140d38280 as *const ()), // sub_140d38280
    Replacement::draft(0x140d38370, "content_140d38370", fn_140d38370 as *const ()), // sub_140d38370
    Replacement::draft_throws(0x140d384a0, "content_140d384a0", fn_140d384a0 as *const (), Throws::Discard), // sub_140d384a0
    Replacement::draft(0x140d385d0, "content_140d385d0", tpl_16407::<0x14387f608, 0x14243b0b0, 0x338, 0x20, 0x30, 0xb8, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d385d0
    Replacement::draft_throws(0x140d38670, "content_140d38670", fn_140d38670 as *const (), Throws::Discard), // sub_140d38670
    Replacement::draft_throws(0x140d387b0, "content_140d387b0", fn_140d387b0 as *const (), Throws::Caught), // sub_140d387b0
    Replacement::draft_throws(0x140d38bd0, "content_140d38bd0", fn_140d38bd0 as *const (), Throws::Caught), // sub_140d38bd0
    Replacement::draft_throws(0x140d38f90, "content_140d38f90", fn_140d38f90 as *const (), Throws::Caught), // sub_140d38f90
    Replacement::draft(0x140d39330, "content_140d39330", fn_140d39330 as *const ()), // sub_140d39330
    Replacement::draft(0x140d39410, "content_140d39410", tpl_74766::<0x14387f610, 0x14387f608, 0x1401e9970, 0x338, 0xc0, 0x30, 0xb8, 0xd8, 0x1401e9860> as *const ()), // sub_140d39410
    Replacement::draft(0x140d394a0, "content_140d394a0", tpl_16407::<0x14387f608, 0x14243b0b0, 0x338, 0xc0, 0x30, 0xb8, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d394a0
    Replacement::draft_throws(0x140d39540, "content_140d39540", tpl_74767::<0x14387f610, 0x14387f608, 0x1401e9770, 0x338, 0x1424b4cb0, 0x30, 0xb8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140d39540
    Replacement::draft_throws(0x140d395d0, "content_140d395d0", fn_140d395d0 as *const (), Throws::Discard), // sub_140d395d0
    Replacement::draft(0x140d396e0, "content_140d396e0", fn_140d396e0 as *const ()), // sub_140d396e0
    Replacement::draft(0x140d39820, "content_140d39820", fn_140d39820 as *const ()), // sub_140d39820
    Replacement::draft(0x140d39930, "content_140d39930", fn_140d39930 as *const ()), // sub_140d39930
    Replacement::draft(0x140d39a40, "content_140d39a40", fn_140d39a40 as *const ()), // sub_140d39a40
    Replacement::draft(0x140d39ae0, "content_140d39ae0", fn_140d39ae0 as *const ()), // sub_140d39ae0
    Replacement::draft(0x140d39c20, "content_140d39c20", fn_140d39c20 as *const ()), // sub_140d39c20
    Replacement::draft(0x140d39d30, "content_140d39d30", fn_140d39d30 as *const ()), // sub_140d39d30
    Replacement::draft(0x140d39e40, "content_140d39e40", fn_140d39e40 as *const ()), // sub_140d39e40
    Replacement::draft(0x140d39ef0, "content_140d39ef0", tpl_4774::<0x1432e9138, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::ClothRenderComponent::content::ClothRenderComponent::vfunc_18
    Replacement::draft(0x140d39ff0, "content_140d39ff0", fn_140d39ff0 as *const ()), // content::ClothComponent::content::ClothComponent::vfunc_20
    Replacement::draft(0x140d3a050, "content_140d3a050", tpl_2003::<0x1432e8f50, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::CollisionResourceComponent::content::CollisionResourceComponent::vfunc_18
    Replacement::draft(0x140d3a1c0, "content_140d3a1c0", tpl_16424::<0x1432e8f78, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::CameraTriggerColliderComponent::content::CameraTriggerColliderComponent::vfunc_18
    Replacement::draft(0x140d3a2d0, "content_140d3a2d0", tpl_6246::<0x1432e8ee0, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::StaticCollisionComponent::content::StaticCollisionComponent::vfunc_18
    Replacement::draft(0x140d3a3e0, "content_140d3a3e0", tpl_4774::<0x1432e8f18, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::DynamicCollisionComponent::content::DynamicCollisionComponent::vfunc_18
    Replacement::draft(0x140d3a540, "content_140d3a540", tpl_6246::<0x1432e90f8, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::HeightfieldCollisionComponent::content::HeightfieldCollisionComponent::vfunc_18
    Replacement::draft(0x140d3a690, "content_140d3a690", fn_140d3a690 as *const ()), // content::TransitionTriggerScriptComponent::content::TransitionTriggerScriptComponent::vfunc_18
    Replacement::draft(0x140d3a810, "content_140d3a810", fn_140d3a810 as *const ()), // content::WindAreaComponent::content::WindAreaComponent::vfunc_18
    Replacement::draft(0x140d3a910, "content_140d3a910", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432eb7f8> as *const ()), // sub_140d3a910
    Replacement::draft(0x140d3a970, "content_140d3a970", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432eb7f8> as *const ()), // sub_140d3a970
    Replacement::draft_throws(0x140d3aa70, "content_140d3aa70", fn_140d3aa70 as *const (), Throws::Caught), // sub_140d3aa70
    Replacement::draft_throws(0x140d3ab00, "content_140d3ab00", tpl_6763::<0x14387f6b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140d3ab00
    Replacement::draft(0x140d3abb0, "content_140d3abb0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432eb950, 0x8> as *const ()), // sub_140d3abb0
    Replacement::draft(0x140d3ac40, "content_140d3ac40", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432eb950, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140d3ac40
    Replacement::draft(0x140d3acf0, "content_140d3acf0", fn_140d3acf0 as *const ()), // sub_140d3acf0
    Replacement::draft(0x140d3add0, "content_140d3add0", fn_140d3add0 as *const ()), // sub_140d3add0
    Replacement::draft_throws(0x140d3af20, "content_140d3af20", tpl_16429::<0x14387f648, 0x14387f690, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140ca3200, 0x140d49b50, 0x140d49c40, 0x318, 0x328, 0x320, 0x4, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d3af20
    Replacement::draft(0x140d3b000, "content_140d3b000", fn_140d3b000 as *const ()), // sub_140d3b000
    Replacement::draft_throws(0x140d3b130, "content_140d3b130", tpl_16429::<0x14387f648, 0x14387f690, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140ca33c0, 0x140d49b50, 0x140d49c40, 0x318, 0x328, 0x320, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d3b130
    Replacement::draft_throws(0x140d3b210, "content_140d3b210", fn_140d3b210 as *const (), Throws::Caught), // sub_140d3b210
    Replacement::draft_throws(0x140d3b2e0, "content_140d3b2e0", fn_140d3b2e0 as *const (), Throws::Caught), // sub_140d3b2e0
    Replacement::draft_throws(0x140d3b3b0, "content_140d3b3b0", fn_140d3b3b0 as *const (), Throws::Caught), // sub_140d3b3b0
    Replacement::draft(0x140d3b480, "content_140d3b480", fn_140d3b480 as *const ()), // sub_140d3b480
    Replacement::draft(0x140d3b5a0, "content_140d3b5a0", fn_140d3b5a0 as *const ()), // sub_140d3b5a0
    Replacement::draft(0x140d3b6c0, "content_140d3b6c0", fn_140d3b6c0 as *const ()), // sub_140d3b6c0
    Replacement::draft_throws(0x140d3b7f0, "content_140d3b7f0", fn_140d3b7f0 as *const (), Throws::Discard), // sub_140d3b7f0
    Replacement::draft_throws(0x140d3b910, "content_140d3b910", fn_140d3b910 as *const (), Throws::Discard), // sub_140d3b910
    Replacement::draft(0x140d3ba00, "content_140d3ba00", tpl_70565::<0x8, 0x10, 0x1432a2788, 0x140ca36f0> as *const ()), // sub_140d3ba00
    Replacement::draft(0x140d3ba80, "content_140d3ba80", tpl_6676::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1432e9cf0, 0x8> as *const ()), // sub_140d3ba80
    Replacement::draft(0x140d3bb10, "content_140d3bb10", tpl_6677::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432e9cf0, 0xfffffffffffffff8, 0x20> as *const ()), // sub_140d3bb10
    Replacement::draft(0x140d3bbb0, "content_140d3bbb0", fn_140d3bbb0 as *const ()), // sub_140d3bbb0
    Replacement::draft(0x140d3bc80, "content_140d3bc80", fn_140d3bc80 as *const ()), // sub_140d3bc80
    Replacement::draft_throws(0x140d3bdc0, "content_140d3bdc0", fn_140d3bdc0 as *const (), Throws::Discard), // sub_140d3bdc0
    Replacement::draft(0x140d3be60, "content_140d3be60", fn_140d3be60 as *const ()), // sub_140d3be60
    Replacement::draft_throws(0x140d3bf40, "content_140d3bf40", fn_140d3bf40 as *const (), Throws::Discard), // sub_140d3bf40
    Replacement::draft(0x140d3bfe0, "content_140d3bfe0", fn_140d3bfe0 as *const ()), // sub_140d3bfe0
    Replacement::draft_throws(0x140d3c0b0, "content_140d3c0b0", fn_140d3c0b0 as *const (), Throws::Caught), // sub_140d3c0b0
    Replacement::draft_throws(0x140d3c190, "content_140d3c190", fn_140d3c190 as *const (), Throws::Caught), // sub_140d3c190
    Replacement::draft_throws(0x140d3c270, "content_140d3c270", fn_140d3c270 as *const (), Throws::Caught), // sub_140d3c270
    Replacement::draft(0x140d3c350, "content_140d3c350", fn_140d3c350 as *const ()), // sub_140d3c350
    Replacement::draft(0x140d3c430, "content_140d3c430", fn_140d3c430 as *const ()), // sub_140d3c430
    Replacement::draft(0x140d3c500, "content_140d3c500", fn_140d3c500 as *const ()), // sub_140d3c500
    Replacement::draft_throws(0x140d3c5e0, "content_140d3c5e0", fn_140d3c5e0 as *const (), Throws::Discard), // sub_140d3c5e0
    Replacement::draft_throws(0x140d3c6b0, "content_140d3c6b0", fn_140d3c6b0 as *const (), Throws::Discard), // sub_140d3c6b0
    Replacement::draft(0x140d3c7b0, "content_140d3c7b0", tpl_11918::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432eb320, 0x8> as *const ()), // sub_140d3c7b0
    Replacement::draft(0x140d3c840, "content_140d3c840", tpl_11919::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432eb320, 0x18> as *const ()), // sub_140d3c840
    Replacement::draft(0x140d3c8f0, "content_140d3c8f0", tpl_13978::<0x14387f6a0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140d3c8f0
    Replacement::draft(0x140d3c990, "content_140d3c990", fn_140d3c990 as *const ()), // sub_140d3c990
    Replacement::draft_throws(0x140d3ca90, "content_140d3ca90", fn_140d3ca90 as *const (), Throws::Discard), // sub_140d3ca90
    Replacement::draft(0x140d3cb60, "content_140d3cb60", fn_140d3cb60 as *const ()), // sub_140d3cb60
    Replacement::draft_throws(0x140d3cc20, "content_140d3cc20", fn_140d3cc20 as *const (), Throws::Discard), // sub_140d3cc20
    Replacement::draft_throws(0x140d3cd00, "content_140d3cd00", fn_140d3cd00 as *const (), Throws::Caught), // sub_140d3cd00
    Replacement::draft_throws(0x140d3cf20, "content_140d3cf20", fn_140d3cf20 as *const (), Throws::Caught), // sub_140d3cf20
    Replacement::draft_throws(0x140d3d110, "content_140d3d110", fn_140d3d110 as *const (), Throws::Caught), // sub_140d3d110
    Replacement::draft(0x140d3d300, "content_140d3d300", fn_140d3d300 as *const ()), // sub_140d3d300
    Replacement::draft(0x140d3d3d0, "content_140d3d3d0", fn_140d3d3d0 as *const ()), // sub_140d3d3d0
    Replacement::draft(0x140d3d490, "content_140d3d490", fn_140d3d490 as *const ()), // sub_140d3d490
    Replacement::draft_throws(0x140d3d560, "content_140d3d560", fn_140d3d560 as *const (), Throws::Discard), // sub_140d3d560
    Replacement::draft_throws(0x140d3d620, "content_140d3d620", fn_140d3d620 as *const (), Throws::Caught), // sub_140d3d620
    Replacement::draft(0x140d3d7c0, "content_140d3d7c0", fn_140d3d7c0 as *const ()), // sub_140d3d7c0
    Replacement::draft(0x140d3d840, "content_140d3d840", fn_140d3d840 as *const ()), // sub_140d3d840
    Replacement::draft(0x140d3d8e0, "content_140d3d8e0", fn_140d3d8e0 as *const ()), // sub_140d3d8e0
    Replacement::draft(0x140d3d990, "content_140d3d990", fn_140d3d990 as *const ()), // sub_140d3d990
    Replacement::draft(0x140d3da50, "content_140d3da50", fn_140d3da50 as *const ()), // sub_140d3da50
    Replacement::draft_throws(0x140d3db20, "content_140d3db20", fn_140d3db20 as *const (), Throws::Caught), // sub_140d3db20
    Replacement::draft_throws(0x140d3dce0, "content_140d3dce0", fn_140d3dce0 as *const (), Throws::Discard), // sub_140d3dce0
    Replacement::draft_throws(0x140d3dd80, "content_140d3dd80", fn_140d3dd80 as *const (), Throws::Caught), // sub_140d3dd80
    Replacement::draft(0x140d3df50, "content_140d3df50", fn_140d3df50 as *const ()), // sub_140d3df50
    Replacement::draft_throws(0x140d3dfc0, "content_140d3dfc0", fn_140d3dfc0 as *const (), Throws::Caught), // sub_140d3dfc0
    Replacement::draft_throws(0x140d3e1e0, "content_140d3e1e0", fn_140d3e1e0 as *const (), Throws::Caught), // sub_140d3e1e0
    Replacement::draft_throws(0x140d3e3d0, "content_140d3e3d0", fn_140d3e3d0 as *const (), Throws::Caught), // sub_140d3e3d0
    Replacement::draft(0x140d3e5c0, "content_140d3e5c0", fn_140d3e5c0 as *const ()), // sub_140d3e5c0
    Replacement::draft(0x140d3e670, "content_140d3e670", fn_140d3e670 as *const ()), // sub_140d3e670
    Replacement::draft(0x140d3e710, "content_140d3e710", fn_140d3e710 as *const ()), // sub_140d3e710
    Replacement::draft_throws(0x140d3e7c0, "content_140d3e7c0", fn_140d3e7c0 as *const (), Throws::Discard), // sub_140d3e7c0
    Replacement::draft_throws(0x140d3e860, "content_140d3e860", fn_140d3e860 as *const (), Throws::Caught), // sub_140d3e860
    Replacement::draft(0x140d3ea00, "content_140d3ea00", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ea3a8> as *const ()), // sub_140d3ea00
    Replacement::draft(0x140d3ea60, "content_140d3ea60", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ea3a8> as *const ()), // sub_140d3ea60
    Replacement::draft_throws(0x140d3eb60, "content_140d3eb60", fn_140d3eb60 as *const (), Throws::Caught), // sub_140d3eb60
    Replacement::draft_throws(0x140d3ebf0, "content_140d3ebf0", tpl_6763::<0x14387f6a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140d3ebf0
    Replacement::draft(0x140d3eca0, "content_140d3eca0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432eaeb0, 0x8> as *const ()), // sub_140d3eca0
    Replacement::draft(0x140d3ed20, "content_140d3ed20", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432eaeb0> as *const ()), // sub_140d3ed20
    Replacement::draft(0x140d3edb0, "content_140d3edb0", tpl_7312::<0x14387f6e0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d3edb0
    Replacement::draft(0x140d3ee20, "content_140d3ee20", fn_140d3ee20 as *const ()), // sub_140d3ee20
    Replacement::draft_throws(0x140d3eed0, "content_140d3eed0", fn_140d3eed0 as *const (), Throws::Discard), // sub_140d3eed0
    Replacement::draft(0x140d3ef60, "content_140d3ef60", tpl_11823::<0x14387f6e0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d3ef60
    Replacement::draft_throws(0x140d3efe0, "content_140d3efe0", fn_140d3efe0 as *const (), Throws::Discard), // sub_140d3efe0
    Replacement::draft_throws(0x140d3f070, "content_140d3f070", fn_140d3f070 as *const (), Throws::Caught), // sub_140d3f070
    Replacement::draft_throws(0x140d3f260, "content_140d3f260", fn_140d3f260 as *const (), Throws::Caught), // sub_140d3f260
    Replacement::draft_throws(0x140d3f420, "content_140d3f420", fn_140d3f420 as *const (), Throws::Caught), // sub_140d3f420
    Replacement::draft(0x140d3f5e0, "content_140d3f5e0", fn_140d3f5e0 as *const ()), // sub_140d3f5e0
    Replacement::draft(0x140d3f650, "content_140d3f650", fn_140d3f650 as *const ()), // sub_140d3f650
    Replacement::draft(0x140d3f6d0, "content_140d3f6d0", fn_140d3f6d0 as *const ()), // sub_140d3f6d0
    Replacement::draft_throws(0x140d3f760, "content_140d3f760", fn_140d3f760 as *const (), Throws::Discard), // sub_140d3f760
    Replacement::draft_throws(0x140d3f7e0, "content_140d3f7e0", fn_140d3f7e0 as *const (), Throws::Discard), // sub_140d3f7e0
    Replacement::draft(0x140d3f8c0, "content_140d3f8c0", fn_140d3f8c0 as *const ()), // sub_140d3f8c0
    Replacement::draft(0x140d3f990, "content_140d3f990", fn_140d3f990 as *const ()), // sub_140d3f990
    Replacement::draft(0x140d3fa60, "content_140d3fa60", fn_140d3fa60 as *const ()), // sub_140d3fa60
    Replacement::draft(0x140d3fc00, "content_140d3fc00", fn_140d3fc00 as *const ()), // sub_140d3fc00
    Replacement::draft(0x140d40750, "content_140d40750", fn_140d40750 as *const ()), // sub_140d40750
    Replacement::draft_throws(0x140d40bc0, "content_140d40bc0", fn_140d40bc0 as *const (), Throws::Discard), // sub_140d40bc0
    Replacement::draft(0x140d411b0, "content_140d411b0", fn_140d411b0 as *const ()), // sub_140d411b0
    Replacement::draft_throws(0x140d41490, "content_140d41490", fn_140d41490 as *const (), Throws::Caught), // sub_140d41490
    Replacement::draft_throws(0x140d41ec0, "content_140d41ec0", fn_140d41ec0 as *const (), Throws::Caught), // sub_140d41ec0
    Replacement::draft_throws(0x140d427a0, "content_140d427a0", fn_140d427a0 as *const (), Throws::Caught), // sub_140d427a0
    Replacement::draft(0x140d43060, "content_140d43060", fn_140d43060 as *const ()), // sub_140d43060
    Replacement::draft(0x140d43390, "content_140d43390", fn_140d43390 as *const ()), // sub_140d43390
    Replacement::draft(0x140d437e0, "content_140d437e0", fn_140d437e0 as *const ()), // sub_140d437e0
    Replacement::draft_throws(0x140d43c50, "content_140d43c50", fn_140d43c50 as *const (), Throws::Discard), // sub_140d43c50
    Replacement::draft_throws(0x140d440e0, "content_140d440e0", fn_140d440e0 as *const (), Throws::Discard), // sub_140d440e0
    Replacement::draft_throws(0x140d44620, "content_140d44620", fn_140d44620 as *const (), Throws::Discard), // sub_140d44620
    Replacement::draft(0x140d44700, "content_140d44700", fn_140d44700 as *const ()), // sub_140d44700
    Replacement::draft(0x140d447f0, "content_140d447f0", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432e9fa0> as *const ()), // sub_140d447f0
    Replacement::draft(0x140d44880, "content_140d44880", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432e9fa0, 0x18> as *const ()), // sub_140d44880
    Replacement::draft(0x140d44930, "content_140d44930", tpl_6766::<0x14387f668, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d44930
    Replacement::draft(0x140d449a0, "content_140d449a0", fn_140d449a0 as *const ()), // sub_140d449a0
    Replacement::draft_throws(0x140d44ac0, "content_140d44ac0", fn_140d44ac0 as *const (), Throws::Discard), // sub_140d44ac0
    Replacement::draft(0x140d44bb0, "content_140d44bb0", fn_140d44bb0 as *const ()), // sub_140d44bb0
    Replacement::draft_throws(0x140d44c50, "content_140d44c50", fn_140d44c50 as *const (), Throws::Discard), // sub_140d44c50
    Replacement::draft_throws(0x140d44d60, "content_140d44d60", fn_140d44d60 as *const (), Throws::Caught), // sub_140d44d60
    Replacement::draft_throws(0x140d44ec0, "content_140d44ec0", fn_140d44ec0 as *const (), Throws::Caught), // sub_140d44ec0
    Replacement::draft_throws(0x140d45020, "content_140d45020", fn_140d45020 as *const (), Throws::Caught), // sub_140d45020
    Replacement::draft(0x140d45180, "content_140d45180", fn_140d45180 as *const ()), // sub_140d45180
    Replacement::draft(0x140d45210, "content_140d45210", tpl_16502::<0x14387f630, 0x14387f668, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x88, 0x1401e9860> as *const ()), // sub_140d45210
    Replacement::draft(0x140d452b0, "content_140d452b0", tpl_16503::<0x14387f668, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x88, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d452b0
    Replacement::draft_throws(0x140d45360, "content_140d45360", fn_140d45360 as *const (), Throws::Discard), // sub_140d45360
    Replacement::draft_throws(0x140d45420, "content_140d45420", fn_140d45420 as *const (), Throws::Discard), // sub_140d45420
    Replacement::draft(0x140d455a0, "content_140d455a0", fn_140d455a0 as *const ()), // sub_140d455a0
    Replacement::draft(0x140d45640, "content_140d45640", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ea500> as *const ()), // sub_140d45640
    Replacement::draft(0x140d456a0, "content_140d456a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ea500> as *const ()), // sub_140d456a0
    Replacement::draft_throws(0x140d457a0, "content_140d457a0", fn_140d457a0 as *const (), Throws::Caught), // sub_140d457a0
    Replacement::draft_throws(0x140d45830, "content_140d45830", tpl_6762::<0x14387f6c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140d45830
    Replacement::draft(0x140d458e0, "content_140d458e0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432eab98, 0x8> as *const ()), // sub_140d458e0
    Replacement::draft(0x140d45960, "content_140d45960", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432eab98> as *const ()), // sub_140d45960
    Replacement::draft(0x140d459f0, "content_140d459f0", tpl_6766::<0x14387f6c0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d459f0
    Replacement::draft(0x140d45a60, "content_140d45a60", fn_140d45a60 as *const ()), // sub_140d45a60
    Replacement::draft_throws(0x140d45b20, "content_140d45b20", fn_140d45b20 as *const (), Throws::Discard), // sub_140d45b20
    Replacement::draft(0x140d45be0, "content_140d45be0", tpl_16509::<0x14387f6c0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0x9, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d45be0
    Replacement::draft_throws(0x140d45c70, "content_140d45c70", fn_140d45c70 as *const (), Throws::Discard), // sub_140d45c70
    Replacement::draft_throws(0x140d45d40, "content_140d45d40", fn_140d45d40 as *const (), Throws::Caught), // sub_140d45d40
    Replacement::draft_throws(0x140d45e60, "content_140d45e60", fn_140d45e60 as *const (), Throws::Caught), // sub_140d45e60
    Replacement::draft_throws(0x140d45f80, "content_140d45f80", fn_140d45f80 as *const (), Throws::Caught), // sub_140d45f80
    Replacement::draft(0x140d460a0, "content_140d460a0", fn_140d460a0 as *const ()), // sub_140d460a0
    Replacement::draft(0x140d46110, "content_140d46110", fn_140d46110 as *const ()), // sub_140d46110
    Replacement::draft(0x140d461a0, "content_140d461a0", tpl_16509::<0x14387f6c0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0x9, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d461a0
    Replacement::draft_throws(0x140d46230, "content_140d46230", fn_140d46230 as *const (), Throws::Discard), // sub_140d46230
    Replacement::draft_throws(0x140d462c0, "content_140d462c0", fn_140d462c0 as *const (), Throws::Discard), // sub_140d462c0
    Replacement::draft(0x140d46400, "content_140d46400", tpl_9792::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ea250, 0x8> as *const ()), // sub_140d46400
    Replacement::draft(0x140d46490, "content_140d46490", fn_140d46490 as *const ()), // sub_140d46490
    Replacement::draft(0x140d465b0, "content_140d465b0", fn_140d465b0 as *const ()), // sub_140d465b0
    Replacement::draft_throws(0x140d46690, "content_140d46690", fn_140d46690 as *const (), Throws::Discard), // sub_140d46690
    Replacement::draft(0x140d46770, "content_140d46770", tpl_16518::<0x14387f638, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d46770
    Replacement::draft_throws(0x140d46820, "content_140d46820", fn_140d46820 as *const (), Throws::Discard), // sub_140d46820
    Replacement::draft_throws(0x140d46900, "content_140d46900", fn_140d46900 as *const (), Throws::Caught), // sub_140d46900
    Replacement::draft_throws(0x140d46b50, "content_140d46b50", fn_140d46b50 as *const (), Throws::Caught), // sub_140d46b50
    Replacement::draft_throws(0x140d46d70, "content_140d46d70", fn_140d46d70 as *const (), Throws::Caught), // sub_140d46d70
    Replacement::draft(0x140d46f80, "content_140d46f80", fn_140d46f80 as *const ()), // sub_140d46f80
    Replacement::draft(0x140d47010, "content_140d47010", fn_140d47010 as *const ()), // sub_140d47010
    Replacement::draft(0x140d470c0, "content_140d470c0", tpl_16518::<0x14387f638, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d470c0
    Replacement::draft_throws(0x140d47180, "content_140d47180", fn_140d47180 as *const (), Throws::Discard), // sub_140d47180
    Replacement::draft_throws(0x140d47220, "content_140d47220", fn_140d47220 as *const (), Throws::Discard), // sub_140d47220
    Replacement::draft_throws(0x140d47370, "content_140d47370", tpl_11802::<0x1434269f8, 0x1432e7ec0, 0x8, 0x1432e7e98, 0x1432e80c8, 0x14020f270, 0xc, 0x1432444f8, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // sub_140d47370
    Replacement::draft(0x140d473f0, "content_140d473f0", tpl_11779::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432ebc68> as *const ()), // sub_140d473f0
    Replacement::draft(0x140d47470, "content_140d47470", tpl_11780::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ebc68, 0xfffffffffffffff0> as *const ()), // sub_140d47470
    Replacement::draft(0x140d47500, "content_140d47500", tpl_7312::<0x14387f658, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d47500
    Replacement::draft(0x140d47570, "content_140d47570", fn_140d47570 as *const ()), // sub_140d47570
    Replacement::draft_throws(0x140d47630, "content_140d47630", fn_140d47630 as *const (), Throws::Discard), // sub_140d47630
    Replacement::draft(0x140d476f0, "content_140d476f0", tpl_11577::<0x14387f658, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0x9, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d476f0
    Replacement::draft_throws(0x140d47780, "content_140d47780", fn_140d47780 as *const (), Throws::Discard), // sub_140d47780
    Replacement::draft_throws(0x140d47850, "content_140d47850", fn_140d47850 as *const (), Throws::Caught), // sub_140d47850
    Replacement::draft_throws(0x140d47970, "content_140d47970", fn_140d47970 as *const (), Throws::Caught), // sub_140d47970
    Replacement::draft_throws(0x140d47a90, "content_140d47a90", fn_140d47a90 as *const (), Throws::Caught), // sub_140d47a90
    Replacement::draft(0x140d47bb0, "content_140d47bb0", fn_140d47bb0 as *const ()), // sub_140d47bb0
    Replacement::draft(0x140d47c20, "content_140d47c20", tpl_11581::<0x14387f68c, 0x14387f658, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x9, 0x1401e9860> as *const ()), // sub_140d47c20
    Replacement::draft(0x140d47cb0, "content_140d47cb0", tpl_11577::<0x14387f658, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0x9, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d47cb0
    Replacement::draft_throws(0x140d47d40, "content_140d47d40", fn_140d47d40 as *const (), Throws::Discard), // sub_140d47d40
    Replacement::draft_throws(0x140d47dd0, "content_140d47dd0", fn_140d47dd0 as *const (), Throws::Discard), // sub_140d47dd0
    Replacement::draft(0x140d47f00, "content_140d47f00", tpl_16534::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432eb1c8, 0x20, 0x8> as *const ()), // sub_140d47f00
    Replacement::draft(0x140d47f90, "content_140d47f90", fn_140d47f90 as *const ()), // sub_140d47f90
    Replacement::draft(0x140d48040, "content_140d48040", tpl_6766::<0x14387f650, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d48040
    Replacement::draft(0x140d480b0, "content_140d480b0", fn_140d480b0 as *const ()), // sub_140d480b0
    Replacement::draft_throws(0x140d48220, "content_140d48220", fn_140d48220 as *const (), Throws::Discard), // sub_140d48220
    Replacement::draft(0x140d48380, "content_140d48380", fn_140d48380 as *const ()), // sub_140d48380
    Replacement::draft_throws(0x140d48480, "content_140d48480", fn_140d48480 as *const (), Throws::Discard), // sub_140d48480
    Replacement::draft_throws(0x140d485f0, "content_140d485f0", fn_140d485f0 as *const (), Throws::Caught), // sub_140d485f0
    Replacement::draft_throws(0x140d48a30, "content_140d48a30", fn_140d48a30 as *const (), Throws::Caught), // sub_140d48a30
    Replacement::draft_throws(0x140d48df0, "content_140d48df0", fn_140d48df0 as *const (), Throws::Caught), // sub_140d48df0
    Replacement::draft(0x140d491a0, "content_140d491a0", fn_140d491a0 as *const ()), // sub_140d491a0
    Replacement::draft(0x140d49250, "content_140d49250", fn_140d49250 as *const ()), // sub_140d49250
    Replacement::draft(0x140d49360, "content_140d49360", fn_140d49360 as *const ()), // sub_140d49360
    Replacement::draft_throws(0x140d49480, "content_140d49480", fn_140d49480 as *const (), Throws::Discard), // sub_140d49480
    Replacement::draft_throws(0x140d49580, "content_140d49580", fn_140d49580 as *const (), Throws::Discard), // sub_140d49580
    Replacement::draft_throws(0x140d49770, "content_140d49770", fn_140d49770 as *const (), Throws::Discard), // sub_140d49770
    Replacement::draft(0x140d49860, "content_140d49860", tpl_1967::<0x140d4aa80, 0x143084e28, 0x141834d30> as *const ()), // content::ClothRenderComponent::content::ClothRenderComponent::vfunc_0
    Replacement::draft(0x140d498c0, "content_140d498c0", tpl_5739::<0x1401de830, 0x143084e28, 0x141834d30> as *const ()), // content::CollisionTemplateComponent::content::CollisionTemplateComponent::vfunc_0
    Replacement::draft(0x140d49910, "content_140d49910", fn_140d49910 as *const ()), // content::DynamicCollisionComponent::content::DynamicCollisionComponent::vfunc_0
    Replacement::draft(0x140d49980, "content_140d49980", fn_140d49980 as *const ()), // sub_140d49980
    Replacement::draft(0x140d499d0, "content_140d499d0", fn_140d499d0 as *const ()), // sub_140d499d0
    Replacement::draft(0x140d49b50, "content_140d49b50", fn_140d49b50 as *const ()), // sub_140d49b50
    Replacement::draft(0x140d49c40, "content_140d49c40", fn_140d49c40 as *const ()), // sub_140d49c40
    Replacement::draft_throws(0x140d49d80, "content_140d49d80", fn_140d49d80 as *const (), Throws::Caught), // sub_140d49d80
    Replacement::draft_throws(0x140d49f30, "content_140d49f30", fn_140d49f30 as *const (), Throws::Caught), // sub_140d49f30
    Replacement::draft_throws(0x140d4a120, "content_140d4a120", tpl_6980::<0x20, 0x8, 0x140d4b520> as *const (), Throws::Discard), // sub_140d4a120
    Replacement::draft_throws(0x140d4a170, "content_140d4a170", fn_140d4a170 as *const (), Throws::Caught), // sub_140d4a170
    Replacement::draft_throws(0x140d4a2b0, "content_140d4a2b0", fn_140d4a2b0 as *const (), Throws::Caught), // sub_140d4a2b0
    Replacement::draft_throws(0x140d4a3f0, "content_140d4a3f0", fn_140d4a3f0 as *const (), Throws::Caught), // sub_140d4a3f0
    Replacement::draft_throws(0x140d4a530, "content_140d4a530", fn_140d4a530 as *const (), Throws::Caught), // sub_140d4a530
    Replacement::draft_throws(0x140d4a6e0, "content_140d4a6e0", fn_140d4a6e0 as *const (), Throws::Caught), // sub_140d4a6e0
    Replacement::draft_throws(0x140d4a900, "content_140d4a900", fn_140d4a900 as *const (), Throws::Discard), // sub_140d4a900
    Replacement::draft_throws(0x140d4a940, "content_140d4a940", tpl_12629::<0x20, 0x8, 0x140d4d610> as *const (), Throws::Discard), // sub_140d4a940
    Replacement::draft(0x140d4a990, "content_140d4a990", fn_140d4a990 as *const ()), // sub_140d4a990
    Replacement::draft(0x140d4aa80, "content_140d4aa80", tpl_16556::<0x8, 0x10, 0x1401de830, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140d4aa80
    Replacement::draft(0x140d4ab50, "content_140d4ab50", fn_140d4ab50 as *const ()), // sub_140d4ab50
    Replacement::draft(0x140d4ac20, "content_140d4ac20", fn_140d4ac20 as *const ()), // sub_140d4ac20
    Replacement::draft_throws(0x140d4acd0, "content_140d4acd0", fn_140d4acd0 as *const (), Throws::Caught), // sub_140d4acd0
    Replacement::draft_throws(0x140d4afc0, "content_140d4afc0", fn_140d4afc0 as *const (), Throws::Caught), // sub_140d4afc0
    Replacement::draft_throws(0x140d4b280, "content_140d4b280", fn_140d4b280 as *const (), Throws::Caught), // sub_140d4b280
    Replacement::draft_throws(0x140d4b520, "content_140d4b520", fn_140d4b520 as *const (), Throws::Caught), // sub_140d4b520
    Replacement::draft_throws(0x140d4b5f0, "content_140d4b5f0", fn_140d4b5f0 as *const (), Throws::Caught), // sub_140d4b5f0
    Replacement::draft_throws(0x140d4bba0, "content_140d4bba0", fn_140d4bba0 as *const (), Throws::Caught), // sub_140d4bba0
    Replacement::draft_throws(0x140d4c110, "content_140d4c110", fn_140d4c110 as *const (), Throws::Caught), // sub_140d4c110
    Replacement::draft_throws(0x140d4c550, "content_140d4c550", fn_140d4c550 as *const (), Throws::Caught), // sub_140d4c550
    Replacement::draft_throws(0x140d4c840, "content_140d4c840", fn_140d4c840 as *const (), Throws::Caught), // sub_140d4c840
    Replacement::draft_throws(0x140d4cb40, "content_140d4cb40", fn_140d4cb40 as *const (), Throws::Caught), // sub_140d4cb40
    Replacement::draft_throws(0x140d4ce00, "content_140d4ce00", fn_140d4ce00 as *const (), Throws::Caught), // sub_140d4ce00
    Replacement::draft_throws(0x140d4d0c0, "content_140d4d0c0", fn_140d4d0c0 as *const (), Throws::Caught), // sub_140d4d0c0
    Replacement::draft_throws(0x140d4d370, "content_140d4d370", fn_140d4d370 as *const (), Throws::Caught), // sub_140d4d370
    Replacement::draft_throws(0x140d4d610, "content_140d4d610", fn_140d4d610 as *const (), Throws::Caught), // sub_140d4d610
    Replacement::draft(0x140d4d6e0, "content_140d4d6e0", fn_140d4d6e0 as *const ()), // sub_140d4d6e0
    Replacement::draft(0x140d4d710, "content_140d4d710", tpl_16563::<0x10, 0x1401de830> as *const ()), // sub_140d4d710
    Replacement::draft(0x140d4d750, "content_140d4d750", fn_140d4d750 as *const ()), // sub_140d4d750
    Replacement::draft(0x140d4d810, "content_140d4d810", fn_140d4d810 as *const ()), // sub_140d4d810
    Replacement::draft(0x140d4da30, "content_140d4da30", fn_140d4da30 as *const ()), // sub_140d4da30
    Replacement::draft_throws(0x140d4dcb0, "content_140d4dcb0", fn_140d4dcb0 as *const (), Throws::Caught), // sub_140d4dcb0
    Replacement::draft_throws(0x140d4e180, "content_140d4e180", fn_140d4e180 as *const (), Throws::Caught), // sub_140d4e180
    Replacement::draft_throws(0x140d4e610, "content_140d4e610", fn_140d4e610 as *const (), Throws::Caught), // sub_140d4e610
    Replacement::draft_throws(0x140d4eaa0, "content_140d4eaa0", fn_140d4eaa0 as *const (), Throws::Caught), // sub_140d4eaa0
    Replacement::draft_throws(0x140d4ef60, "content_140d4ef60", fn_140d4ef60 as *const (), Throws::Caught), // sub_140d4ef60
    Replacement::draft_throws(0x140d4f430, "content_140d4f430", fn_140d4f430 as *const (), Throws::Caught), // sub_140d4f430
    Replacement::draft_throws(0x140d4f8b0, "content_140d4f8b0", fn_140d4f8b0 as *const (), Throws::Caught), // sub_140d4f8b0
    Replacement::draft_throws(0x140d4fd40, "content_140d4fd40", fn_140d4fd40 as *const (), Throws::Caught), // sub_140d4fd40
    Replacement::draft_throws(0x140d501c0, "content_140d501c0", fn_140d501c0 as *const (), Throws::Caught), // sub_140d501c0
    Replacement::draft(0x140d50650, "content_140d50650", fn_140d50650 as *const ()), // sub_140d50650
    Replacement::draft(0x140d50720, "content_140d50720", fn_140d50720 as *const ()), // sub_140d50720
    Replacement::draft(0x140d50820, "content_140d50820", fn_140d50820 as *const ()), // sub_140d50820
    Replacement::draft_throws(0x140d50940, "content_140d50940", fn_140d50940 as *const (), Throws::Caught), // sub_140d50940
    Replacement::draft_throws(0x140d50a80, "content_140d50a80", fn_140d50a80 as *const (), Throws::Caught), // sub_140d50a80
    Replacement::draft_throws(0x140d50bc0, "content_140d50bc0", fn_140d50bc0 as *const (), Throws::Caught), // sub_140d50bc0
    Replacement::draft_throws(0x140d50d00, "content_140d50d00", fn_140d50d00 as *const (), Throws::Caught), // sub_140d50d00
    Replacement::draft(0x140d50f50, "content_140d50f50", fn_140d50f50 as *const ()), // sub_140d50f50
    Replacement::draft(0x140d50fe0, "content_140d50fe0", fn_140d50fe0 as *const ()), // sub_140d50fe0
    Replacement::draft_throws(0x140d51150, "content_140d51150", fn_140d51150 as *const (), Throws::Caught), // sub_140d51150
    Replacement::draft_throws(0x140d51630, "content_140d51630", fn_140d51630 as *const (), Throws::Caught), // sub_140d51630
    Replacement::draft_throws(0x140d51770, "content_140d51770", fn_140d51770 as *const (), Throws::Caught), // sub_140d51770
    Replacement::draft_throws(0x140d518b0, "content_140d518b0", fn_140d518b0 as *const (), Throws::Caught), // sub_140d518b0
    Replacement::draft_throws(0x140d519f0, "content_140d519f0", fn_140d519f0 as *const (), Throws::Caught), // sub_140d519f0
    Replacement::draft(0x140d51be0, "content_140d51be0", fn_140d51be0 as *const ()), // sub_140d51be0
    Replacement::draft_throws(0x140d51c60, "content_140d51c60", fn_140d51c60 as *const (), Throws::Discard), // sub_140d51c60
    Replacement::draft(0x140d51d90, "content_140d51d90", fn_140d51d90 as *const ()), // sub_140d51d90
    Replacement::draft(0x140d51e80, "content_140d51e80", fn_140d51e80 as *const ()), // sub_140d51e80
    Replacement::draft_throws(0x140d51f80, "content_140d51f80", fn_140d51f80 as *const (), Throws::Discard), // sub_140d51f80
    Replacement::draft(0x140d52070, "content_140d52070", fn_140d52070 as *const ()), // sub_140d52070
    Replacement::draft_throws(0x140d520f0, "content_140d520f0", fn_140d520f0 as *const (), Throws::Caught), // sub_140d520f0
    Replacement::draft_throws(0x140d52560, "content_140d52560", fn_140d52560 as *const (), Throws::Caught), // sub_140d52560
    Replacement::draft_throws(0x140d52990, "content_140d52990", fn_140d52990 as *const (), Throws::Caught), // sub_140d52990
    Replacement::draft(0x140d52ce0, "content_140d52ce0", fn_140d52ce0 as *const ()), // sub_140d52ce0
    Replacement::draft(0x140d52dc0, "content_140d52dc0", fn_140d52dc0 as *const ()), // sub_140d52dc0
    Replacement::draft_throws(0x140d52eb0, "content_140d52eb0", fn_140d52eb0 as *const (), Throws::Discard), // sub_140d52eb0
    Replacement::draft_throws(0x140d52fc0, "content_140d52fc0", fn_140d52fc0 as *const (), Throws::Discard), // sub_140d52fc0
    Replacement::draft(0x140d53140, "content_140d53140", fn_140d53140 as *const ()), // sub_140d53140
    Replacement::draft(0x140d53310, "content_140d53310", fn_140d53310 as *const ()), // sub_140d53310
    Replacement::draft_throws(0x140d53400, "content_140d53400", fn_140d53400 as *const (), Throws::Discard), // sub_140d53400
    Replacement::draft(0x140d535d0, "content_140d535d0", fn_140d535d0 as *const ()), // sub_140d535d0
    Replacement::draft_throws(0x140d53670, "content_140d53670", fn_140d53670 as *const (), Throws::Caught), // sub_140d53670
    Replacement::draft_throws(0x140d53ae0, "content_140d53ae0", fn_140d53ae0 as *const (), Throws::Caught), // sub_140d53ae0
    Replacement::draft_throws(0x140d53f10, "content_140d53f10", fn_140d53f10 as *const (), Throws::Caught), // sub_140d53f10
    Replacement::draft(0x140d54260, "content_140d54260", fn_140d54260 as *const ()), // sub_140d54260
    Replacement::draft(0x140d54330, "content_140d54330", fn_140d54330 as *const ()), // sub_140d54330
    Replacement::draft_throws(0x140d54410, "content_140d54410", fn_140d54410 as *const (), Throws::Discard), // sub_140d54410
    Replacement::draft_throws(0x140d54570, "content_140d54570", tpl_10122::<0x14387f700, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432e9240, 0x140d54b30, 0x10, 0x1432e9220, 0x1409fef90> as *const (), Throws::Discard), // sub_140d54570
    Replacement::draft_throws(0x140d54630, "content_140d54630", fn_140d54630 as *const (), Throws::Discard), // sub_140d54630
    Replacement::draft(0x140d547d0, "content_140d547d0", fn_140d547d0 as *const ()), // sub_140d547d0
    Replacement::draft(0x140d54870, "content_140d54870", fn_140d54870 as *const ()), // sub_140d54870
    Replacement::draft_throws(0x140d548f0, "content_140d548f0", fn_140d548f0 as *const (), Throws::Caught), // sub_140d548f0
    Replacement::draft_throws(0x140d549b0, "content_140d549b0", fn_140d549b0 as *const (), Throws::Caught), // sub_140d549b0
    Replacement::draft_throws(0x140d54a70, "content_140d54a70", fn_140d54a70 as *const (), Throws::Caught), // sub_140d54a70
    Replacement::draft_throws(0x140d54b30, "content_140d54b30", tpl_9695::<0x20, 0x8, 0x140d55100> as *const (), Throws::Discard), // sub_140d54b30
    Replacement::draft_throws(0x140d54b80, "content_140d54b80", fn_140d54b80 as *const (), Throws::Caught), // sub_140d54b80
    Replacement::draft_throws(0x140d54ce0, "content_140d54ce0", fn_140d54ce0 as *const (), Throws::Caught), // sub_140d54ce0
    Replacement::draft_throws(0x140d54e40, "content_140d54e40", fn_140d54e40 as *const (), Throws::Caught), // sub_140d54e40
    Replacement::draft_throws(0x140d55100, "content_140d55100", fn_140d55100 as *const (), Throws::Caught), // sub_140d55100
    Replacement::draft_throws(0x140d551c0, "content_140d551c0", fn_140d551c0 as *const (), Throws::Caught), // sub_140d551c0
    Replacement::draft_throws(0x140d554a0, "content_140d554a0", fn_140d554a0 as *const (), Throws::Caught), // sub_140d554a0
    Replacement::draft_throws(0x140d55750, "content_140d55750", fn_140d55750 as *const (), Throws::Caught), // sub_140d55750
    Replacement::draft_throws(0x140d55a00, "content_140d55a00", fn_140d55a00 as *const (), Throws::Caught), // sub_140d55a00
    Replacement::draft_throws(0x140d55f80, "content_140d55f80", fn_140d55f80 as *const (), Throws::Caught), // sub_140d55f80
    Replacement::draft_throws(0x140d56460, "content_140d56460", fn_140d56460 as *const (), Throws::Caught), // sub_140d56460
    Replacement::draft(0x140d56930, "content_140d56930", fn_140d56930 as *const ()), // sub_140d56930
    Replacement::draft_throws(0x140d56ac0, "content_140d56ac0", fn_140d56ac0 as *const (), Throws::Discard), // sub_140d56ac0
    Replacement::draft_throws(0x140d56c20, "content_140d56c20", fn_140d56c20 as *const (), Throws::Discard), // sub_140d56c20
    Replacement::draft_throws(0x140d56d00, "content_140d56d00", fn_140d56d00 as *const (), Throws::Discard), // sub_140d56d00
    Replacement::draft_throws(0x140d56e90, "content_140d56e90", fn_140d56e90 as *const (), Throws::Caught), // sub_140d56e90
    Replacement::draft_throws(0x140d57080, "content_140d57080", fn_140d57080 as *const (), Throws::Caught), // sub_140d57080
    Replacement::draft_throws(0x140d57270, "content_140d57270", fn_140d57270 as *const (), Throws::Caught), // sub_140d57270
    Replacement::draft(0x140d57460, "content_140d57460", fn_140d57460 as *const ()), // sub_140d57460
    Replacement::draft(0x140d57540, "content_140d57540", fn_140d57540 as *const ()), // sub_140d57540
    Replacement::draft(0x140d57660, "content_140d57660", fn_140d57660 as *const ()), // sub_140d57660
    Replacement::draft_throws(0x140d57790, "content_140d57790", fn_140d57790 as *const (), Throws::Discard), // sub_140d57790
    Replacement::draft_throws(0x140d578f0, "content_140d578f0", fn_140d578f0 as *const (), Throws::Discard), // sub_140d578f0
    Replacement::draft(0x140d57ab0, "content_140d57ab0", fn_140d57ab0 as *const ()), // sub_140d57ab0
    Replacement::draft(0x140d57b40, "content_140d57b40", fn_140d57b40 as *const ()), // sub_140d57b40
    Replacement::draft(0x140d57c30, "content_140d57c30", fn_140d57c30 as *const ()), // sub_140d57c30
    Replacement::draft(0x140d57cb0, "content_140d57cb0", tpl_11316::<0x1432ec4c0, 0x142409b50, 0x1401f22d0, 0x14240a730, 0x14240ab10, 0x1432ec4e0> as *const ()), // content::AnimationSceneComponent::content::AnimationSceneComponent::vfunc_18
    Replacement::draft(0x140d57e60, "content_140d57e60", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432ec510, 0x8> as *const ()), // sub_140d57e60
    Replacement::draft(0x140d57ee0, "content_140d57ee0", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ec510> as *const ()), // sub_140d57ee0
    Replacement::draft(0x140d57fe0, "content_140d57fe0", tpl_6896::<0x14387f718, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f4440, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d57fe0
    Replacement::draft_throws(0x140d58050, "content_140d58050", fn_140d58050 as *const (), Throws::Caught), // sub_140d58050
    Replacement::draft(0x140d581d0, "content_140d581d0", tpl_6896::<0x14387f718, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f47b0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d581d0
    Replacement::draft_throws(0x140d58240, "content_140d58240", fn_140d58240 as *const (), Throws::Caught), // sub_140d58240
    Replacement::draft_throws(0x140d583c0, "content_140d583c0", fn_140d583c0 as *const (), Throws::Caught), // sub_140d583c0
    Replacement::draft_throws(0x140d58490, "content_140d58490", fn_140d58490 as *const (), Throws::Caught), // sub_140d58490
    Replacement::draft_throws(0x140d58560, "content_140d58560", fn_140d58560 as *const (), Throws::Caught), // sub_140d58560
    Replacement::draft(0x140d58630, "content_140d58630", tpl_6896::<0x14387f718, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f5060, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d58630
    Replacement::draft(0x140d586a0, "content_140d586a0", tpl_6899::<0x14387f720, 0x14387f718, 0x1401e9970, 0x1406e1480, 0x8, 0x1408f51f0, 0x1401e9860> as *const ()), // sub_140d586a0
    Replacement::draft(0x140d58700, "content_140d58700", tpl_6896::<0x14387f718, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1408f5360, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d58700
    Replacement::draft_throws(0x140d58770, "content_140d58770", tpl_6900::<0x14387f720, 0x14387f718, 0x1401e9770, 0x1406e1570, 0x8, 0x1408f54b0, 0x1401e9590> as *const (), Throws::Discard), // sub_140d58770
    Replacement::draft_throws(0x140d587d0, "content_140d587d0", fn_140d587d0 as *const (), Throws::Caught), // sub_140d587d0
    Replacement::draft_throws(0x140d58950, "content_140d58950", fn_140d58950 as *const (), Throws::Caught), // sub_140d58950
    Replacement::draft_throws(0x140d58ed0, "content_140d58ed0", fn_140d58ed0 as *const (), Throws::Caught), // sub_140d58ed0
    Replacement::draft_throws(0x140d59420, "content_140d59420", fn_140d59420 as *const (), Throws::Caught), // sub_140d59420
    Replacement::draft(0x140d5a1b0, "content_140d5a1b0", fn_140d5a1b0 as *const ()), // sub_140d5a1b0
    Replacement::draft(0x140d5a290, "content_140d5a290", fn_140d5a290 as *const ()), // sub_140d5a290
    Replacement::draft(0x140d5a3b0, "content_140d5a3b0", tpl_13978::<0x14387f7e0, 0x14243b0b0, 0x14081bf10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140d5a3b0
    Replacement::draft(0x140d5a450, "content_140d5a450", fn_140d5a450 as *const ()), // sub_140d5a450
    Replacement::draft_throws(0x140d5a630, "content_140d5a630", fn_140d5a630 as *const (), Throws::Discard), // sub_140d5a630
    Replacement::draft(0x140d5a800, "content_140d5a800", fn_140d5a800 as *const ()), // sub_140d5a800
    Replacement::draft_throws(0x140d5a930, "content_140d5a930", fn_140d5a930 as *const (), Throws::Discard), // sub_140d5a930
    Replacement::draft(0x140d5ab50, "content_140d5ab50", fn_140d5ab50 as *const ()), // sub_140d5ab50
    Replacement::draft_throws(0x140d5ac40, "content_140d5ac40", fn_140d5ac40 as *const (), Throws::Caught), // sub_140d5ac40
    Replacement::draft_throws(0x140d5afd0, "content_140d5afd0", fn_140d5afd0 as *const (), Throws::Caught), // sub_140d5afd0
    Replacement::draft_throws(0x140d5b320, "content_140d5b320", fn_140d5b320 as *const (), Throws::Caught), // sub_140d5b320
    Replacement::draft(0x140d5b660, "content_140d5b660", fn_140d5b660 as *const ()), // sub_140d5b660
    Replacement::draft(0x140d5b7a0, "content_140d5b7a0", fn_140d5b7a0 as *const ()), // sub_140d5b7a0
    Replacement::draft(0x140d5b8e0, "content_140d5b8e0", fn_140d5b8e0 as *const ()), // sub_140d5b8e0
    Replacement::draft_throws(0x140d5ba30, "content_140d5ba30", fn_140d5ba30 as *const (), Throws::Discard), // sub_140d5ba30
    Replacement::draft_throws(0x140d5bb90, "content_140d5bb90", fn_140d5bb90 as *const (), Throws::Discard), // sub_140d5bb90
    Replacement::draft(0x140d5bd80, "content_140d5bd80", tpl_9044::<0x68, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc> as *const ()), // sub_140d5bd80
    Replacement::draft(0x140d5be20, "content_140d5be20", fn_140d5be20 as *const ()), // sub_140d5be20
    Replacement::draft(0x140d5be90, "content_140d5be90", fn_140d5be90 as *const ()), // sub_140d5be90
    Replacement::draft(0x140d5bf90, "content_140d5bf90", fn_140d5bf90 as *const ()), // sub_140d5bf90
    Replacement::draft(0x140d5c0f0, "content_140d5c0f0", fn_140d5c0f0 as *const ()), // sub_140d5c0f0
    Replacement::draft(0x140d5c180, "content_140d5c180", fn_140d5c180 as *const ()), // sub_140d5c180
    Replacement::draft_throws(0x140d5c280, "content_140d5c280", fn_140d5c280 as *const (), Throws::Discard), // sub_140d5c280
    Replacement::draft(0x140d5c360, "content_140d5c360", tpl_16627::<0x14387f838, 0x14243b0b0, 0x14081c750, 0x1, 0x1403c79b0, 0x338, 0x60, 0x40, 0x64, 0x38, 0x68, 0x140837b40> as *const ()), // sub_140d5c360
    Replacement::draft_throws(0x140d5c410, "content_140d5c410", fn_140d5c410 as *const (), Throws::Discard), // sub_140d5c410
    Replacement::draft(0x140d5c500, "content_140d5c500", fn_140d5c500 as *const ()), // sub_140d5c500
    Replacement::draft_throws(0x140d5c600, "content_140d5c600", fn_140d5c600 as *const (), Throws::Caught), // sub_140d5c600
    Replacement::draft_throws(0x140d5c730, "content_140d5c730", fn_140d5c730 as *const (), Throws::Caught), // sub_140d5c730
    Replacement::draft_throws(0x140d5c860, "content_140d5c860", fn_140d5c860 as *const (), Throws::Caught), // sub_140d5c860
    Replacement::draft(0x140d5c990, "content_140d5c990", fn_140d5c990 as *const ()), // sub_140d5c990
    Replacement::draft(0x140d5ca40, "content_140d5ca40", fn_140d5ca40 as *const ()), // sub_140d5ca40
    Replacement::draft(0x140d5cae0, "content_140d5cae0", tpl_16627::<0x14387f838, 0x14243b0b0, 0x14081dec0, 0x1, 0x1403c79b0, 0x338, 0x60, 0xe0, 0x64, 0xd8, 0x68, 0x140837fa0> as *const ()), // sub_140d5cae0
    Replacement::draft_throws(0x140d5cb90, "content_140d5cb90", fn_140d5cb90 as *const (), Throws::Discard), // sub_140d5cb90
    Replacement::draft_throws(0x140d5cc40, "content_140d5cc40", fn_140d5cc40 as *const (), Throws::Discard), // sub_140d5cc40
    Replacement::draft(0x140d5cd50, "content_140d5cd50", fn_140d5cd50 as *const ()), // sub_140d5cd50
    Replacement::draft(0x140d5ce40, "content_140d5ce40", fn_140d5ce40 as *const ()), // sub_140d5ce40
    Replacement::draft(0x140d5cfe0, "content_140d5cfe0", fn_140d5cfe0 as *const ()), // sub_140d5cfe0
    Replacement::draft_throws(0x140d5d090, "content_140d5d090", tpl_12387::<0x14387f8f4, 0x14387fb28, 0x1401e9fa0, 0x14081c440, 0x328, 0x338, 0x60, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d5d090
    Replacement::draft(0x140d5d140, "content_140d5d140", tpl_9064::<0x14387fb28, 0x14243b0b0, 0x14081c750, 0x338, 0x60, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d5d140
    Replacement::draft_throws(0x140d5d1c0, "content_140d5d1c0", tpl_12388::<0x14387f8f4, 0x14387fb28, 0x1401e9b60, 0x14081c990, 0x328, 0x60, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d5d1c0
    Replacement::draft(0x140d5d270, "content_140d5d270", fn_140d5d270 as *const ()), // sub_140d5d270
    Replacement::draft_throws(0x140d5d2f0, "content_140d5d2f0", fn_140d5d2f0 as *const (), Throws::Caught), // sub_140d5d2f0
    Replacement::draft_throws(0x140d5d560, "content_140d5d560", fn_140d5d560 as *const (), Throws::Caught), // sub_140d5d560
    Replacement::draft_throws(0x140d5d790, "content_140d5d790", fn_140d5d790 as *const (), Throws::Caught), // sub_140d5d790
    Replacement::draft(0x140d5d9b0, "content_140d5d9b0", fn_140d5d9b0 as *const ()), // sub_140d5d9b0
    Replacement::draft(0x140d5da50, "content_140d5da50", tpl_9203::<0x14387f8f4, 0x14387fb28, 0x1401e9970, 0x14081dc70, 0x338, 0x60, 0xc0, 0x1401e9860> as *const ()), // sub_140d5da50
    Replacement::draft(0x140d5dac0, "content_140d5dac0", tpl_9064::<0x14387fb28, 0x14243b0b0, 0x14081dec0, 0x338, 0x60, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d5dac0
    Replacement::draft_throws(0x140d5db40, "content_140d5db40", tpl_12393::<0x14387f8f4, 0x14387fb28, 0x1401e9770, 0x14081e120, 0x338, 0x60, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_140d5db40
    Replacement::draft_throws(0x140d5dbb0, "content_140d5dbb0", fn_140d5dbb0 as *const (), Throws::Discard), // sub_140d5dbb0
    Replacement::draft(0x140d5dc70, "content_140d5dc70", fn_140d5dc70 as *const ()), // sub_140d5dc70
    Replacement::draft(0x140d5dd70, "content_140d5dd70", fn_140d5dd70 as *const ()), // sub_140d5dd70
    Replacement::draft(0x140d5dea0, "content_140d5dea0", tpl_16646::<0x14387f960, 0x14243b0b0, 0x14081bf10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d5dea0
    Replacement::draft(0x140d5df10, "content_140d5df10", fn_140d5df10 as *const ()), // sub_140d5df10
    Replacement::draft_throws(0x140d5e150, "content_140d5e150", fn_140d5e150 as *const (), Throws::Discard), // sub_140d5e150
    Replacement::draft(0x140d5e3c0, "content_140d5e3c0", fn_140d5e3c0 as *const ()), // sub_140d5e3c0
    Replacement::draft_throws(0x140d5e570, "content_140d5e570", fn_140d5e570 as *const (), Throws::Discard), // sub_140d5e570
    Replacement::draft(0x140d5e830, "content_140d5e830", fn_140d5e830 as *const ()), // sub_140d5e830
    Replacement::draft_throws(0x140d5e990, "content_140d5e990", fn_140d5e990 as *const (), Throws::Caught), // sub_140d5e990
    Replacement::draft_throws(0x140d5f140, "content_140d5f140", fn_140d5f140 as *const (), Throws::Caught), // sub_140d5f140
    Replacement::draft_throws(0x140d5f7e0, "content_140d5f7e0", fn_140d5f7e0 as *const (), Throws::Caught), // sub_140d5f7e0
    Replacement::draft(0x140d5fe80, "content_140d5fe80", fn_140d5fe80 as *const ()), // sub_140d5fe80
    Replacement::draft(0x140d5ffc0, "content_140d5ffc0", fn_140d5ffc0 as *const ()), // sub_140d5ffc0
    Replacement::draft(0x140d60160, "content_140d60160", fn_140d60160 as *const ()), // sub_140d60160
    Replacement::draft_throws(0x140d60310, "content_140d60310", fn_140d60310 as *const (), Throws::Discard), // sub_140d60310
    Replacement::draft_throws(0x140d604b0, "content_140d604b0", fn_140d604b0 as *const (), Throws::Discard), // sub_140d604b0
    Replacement::draft_throws(0x140d60760, "content_140d60760", fn_140d60760 as *const (), Throws::Discard), // sub_140d60760
    Replacement::draft(0x140d60820, "content_140d60820", fn_140d60820 as *const ()), // sub_140d60820
    Replacement::draft(0x140d608f0, "content_140d608f0", fn_140d608f0 as *const ()), // sub_140d608f0
    Replacement::draft(0x140d609c0, "content_140d609c0", fn_140d609c0 as *const ()), // sub_140d609c0
    Replacement::draft(0x140d60a60, "content_140d60a60", fn_140d60a60 as *const ()), // sub_140d60a60
    Replacement::draft_throws(0x140d60b20, "content_140d60b20", fn_140d60b20 as *const (), Throws::Discard), // sub_140d60b20
    Replacement::draft_throws(0x140d60c90, "content_140d60c90", fn_140d60c90 as *const (), Throws::Discard), // sub_140d60c90
    Replacement::draft_throws(0x140d60de0, "content_140d60de0", fn_140d60de0 as *const (), Throws::Discard), // sub_140d60de0
    Replacement::draft_throws(0x140d60e90, "content_140d60e90", fn_140d60e90 as *const (), Throws::Discard), // sub_140d60e90
    Replacement::draft_throws(0x140d60f30, "content_140d60f30", fn_140d60f30 as *const (), Throws::Caught), // sub_140d60f30
    Replacement::draft_throws(0x140d61000, "content_140d61000", fn_140d61000 as *const (), Throws::Caught), // sub_140d61000
    Replacement::draft_throws(0x140d610d0, "content_140d610d0", fn_140d610d0 as *const (), Throws::Caught), // sub_140d610d0
    Replacement::draft(0x140d611a0, "content_140d611a0", fn_140d611a0 as *const ()), // sub_140d611a0
    Replacement::draft(0x140d612c0, "content_140d612c0", fn_140d612c0 as *const ()), // sub_140d612c0
    Replacement::draft(0x140d613d0, "content_140d613d0", fn_140d613d0 as *const ()), // sub_140d613d0
    Replacement::draft_throws(0x140d614f0, "content_140d614f0", fn_140d614f0 as *const (), Throws::Discard), // sub_140d614f0
    Replacement::draft_throws(0x140d61630, "content_140d61630", fn_140d61630 as *const (), Throws::Discard), // sub_140d61630
    Replacement::draft(0x140d61700, "content_140d61700", tpl_8298::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1432f09b8, 0x8> as *const ()), // sub_140d61700
    Replacement::draft(0x140d61790, "content_140d61790", tpl_8299::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432f09b8, 0xffffffffffffffec, 0xffffffffffffffe4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140d61790
    Replacement::draft(0x140d61830, "content_140d61830", fn_140d61830 as *const ()), // sub_140d61830
    Replacement::draft(0x140d61940, "content_140d61940", fn_140d61940 as *const ()), // sub_140d61940
    Replacement::draft_throws(0x140d61ad0, "content_140d61ad0", fn_140d61ad0 as *const (), Throws::Caught), // sub_140d61ad0
    Replacement::draft(0x140d61c10, "content_140d61c10", fn_140d61c10 as *const ()), // sub_140d61c10
    Replacement::draft_throws(0x140d61d40, "content_140d61d40", fn_140d61d40 as *const (), Throws::Caught), // sub_140d61d40
    Replacement::draft(0x140d61e80, "content_140d61e80", fn_140d61e80 as *const ()), // sub_140d61e80
    Replacement::draft_throws(0x140d61f30, "content_140d61f30", fn_140d61f30 as *const (), Throws::Caught), // sub_140d61f30
    Replacement::draft_throws(0x140d62190, "content_140d62190", fn_140d62190 as *const (), Throws::Caught), // sub_140d62190
    Replacement::draft_throws(0x140d623c0, "content_140d623c0", fn_140d623c0 as *const (), Throws::Caught), // sub_140d623c0
    Replacement::draft(0x140d625f0, "content_140d625f0", fn_140d625f0 as *const ()), // sub_140d625f0
    Replacement::draft(0x140d62700, "content_140d62700", fn_140d62700 as *const ()), // sub_140d62700
    Replacement::draft(0x140d62810, "content_140d62810", fn_140d62810 as *const ()), // sub_140d62810
    Replacement::draft_throws(0x140d62940, "content_140d62940", fn_140d62940 as *const (), Throws::Discard), // sub_140d62940
    Replacement::draft_throws(0x140d62a50, "content_140d62a50", fn_140d62a50 as *const (), Throws::Discard), // sub_140d62a50
    Replacement::draft(0x140d62b90, "content_140d62b90", fn_140d62b90 as *const ()), // sub_140d62b90
    Replacement::draft(0x140d62c20, "content_140d62c20", tpl_16534::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432f4e18, 0x20, 0x8> as *const ()), // sub_140d62c20
    Replacement::draft(0x140d62cb0, "content_140d62cb0", fn_140d62cb0 as *const ()), // sub_140d62cb0
    Replacement::draft(0x140d62d60, "content_140d62d60", fn_140d62d60 as *const ()), // sub_140d62d60
    Replacement::draft(0x140d62e00, "content_140d62e00", fn_140d62e00 as *const ()), // sub_140d62e00
    Replacement::draft_throws(0x140d62f70, "content_140d62f70", fn_140d62f70 as *const (), Throws::Discard), // sub_140d62f70
    Replacement::draft(0x140d630a0, "content_140d630a0", fn_140d630a0 as *const ()), // sub_140d630a0
    Replacement::draft_throws(0x140d63190, "content_140d63190", fn_140d63190 as *const (), Throws::Discard), // sub_140d63190
    Replacement::draft_throws(0x140d632d0, "content_140d632d0", fn_140d632d0 as *const (), Throws::Caught), // sub_140d632d0
    Replacement::draft_throws(0x140d634a0, "content_140d634a0", fn_140d634a0 as *const (), Throws::Caught), // sub_140d634a0
    Replacement::draft_throws(0x140d63670, "content_140d63670", fn_140d63670 as *const (), Throws::Caught), // sub_140d63670
    Replacement::draft(0x140d63840, "content_140d63840", fn_140d63840 as *const ()), // sub_140d63840
    Replacement::draft(0x140d63940, "content_140d63940", fn_140d63940 as *const ()), // sub_140d63940
    Replacement::draft(0x140d63a20, "content_140d63a20", fn_140d63a20 as *const ()), // sub_140d63a20
    Replacement::draft_throws(0x140d63b10, "content_140d63b10", fn_140d63b10 as *const (), Throws::Discard), // sub_140d63b10
    Replacement::draft_throws(0x140d63c00, "content_140d63c00", fn_140d63c00 as *const (), Throws::Discard), // sub_140d63c00
    Replacement::draft(0x140d63d80, "content_140d63d80", tpl_7173::<0x1432f8978, 0x1c, 0x3, 0x14020f270, 0x20, 0x1432f8938> as *const ()), // sub_140d63d80
    Replacement::draft(0x140d63df0, "content_140d63df0", tpl_16694::<0x1432ec430, 0x8, 0x1, 0x1409fe2d0, 0x10, 0x1432f8998, 0x4> as *const ()), // sub_140d63df0
    Replacement::draft(0x140d63e60, "content_140d63e60", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432f2388> as *const ()), // sub_140d63e60
    Replacement::draft(0x140d63ec0, "content_140d63ec0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f2388> as *const ()), // sub_140d63ec0
    Replacement::draft_throws(0x140d640f0, "content_140d640f0", fn_140d640f0 as *const (), Throws::Caught), // sub_140d640f0
    Replacement::draft_throws(0x140d64280, "content_140d64280", tpl_1905::<0x14387fa78, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387e8c8> as *const (), Throws::Discard), // sub_140d64280
    Replacement::draft(0x140d64330, "content_140d64330", tpl_16695::<0x190, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140d643d0> as *const ()), // sub_140d64330
    Replacement::draft(0x140d643d0, "content_140d643d0", fn_140d643d0 as *const ()), // sub_140d643d0
    Replacement::draft(0x140d644c0, "content_140d644c0", fn_140d644c0 as *const ()), // sub_140d644c0
    Replacement::draft(0x140d64590, "content_140d64590", fn_140d64590 as *const ()), // sub_140d64590
    Replacement::draft(0x140d64780, "content_140d64780", fn_140d64780 as *const ()), // sub_140d64780
    Replacement::draft_throws(0x140d64b10, "content_140d64b10", fn_140d64b10 as *const (), Throws::Discard), // sub_140d64b10
    Replacement::draft_throws(0x140d64e00, "content_140d64e00", fn_140d64e00 as *const (), Throws::Discard), // sub_140d64e00
    Replacement::draft_throws(0x140d65100, "content_140d65100", fn_140d65100 as *const (), Throws::Discard), // sub_140d65100
    Replacement::draft_throws(0x140d65370, "content_140d65370", fn_140d65370 as *const (), Throws::Discard), // sub_140d65370
    Replacement::draft_throws(0x140d654f0, "content_140d654f0", fn_140d654f0 as *const (), Throws::Caught), // sub_140d654f0
    Replacement::draft_throws(0x140d659d0, "content_140d659d0", fn_140d659d0 as *const (), Throws::Caught), // sub_140d659d0
    Replacement::draft_throws(0x140d65e50, "content_140d65e50", fn_140d65e50 as *const (), Throws::Caught), // sub_140d65e50
    Replacement::draft(0x140d662b0, "content_140d662b0", fn_140d662b0 as *const ()), // sub_140d662b0
    Replacement::draft(0x140d665e0, "content_140d665e0", fn_140d665e0 as *const ()), // sub_140d665e0
    Replacement::draft(0x140d66920, "content_140d66920", fn_140d66920 as *const ()), // sub_140d66920
    Replacement::draft_throws(0x140d66c70, "content_140d66c70", fn_140d66c70 as *const (), Throws::Discard), // sub_140d66c70
    Replacement::draft_throws(0x140d66f90, "content_140d66f90", fn_140d66f90 as *const (), Throws::Discard), // sub_140d66f90
    Replacement::draft(0x140d67160, "content_140d67160", fn_140d67160 as *const ()), // sub_140d67160
    Replacement::draft(0x140d67320, "content_140d67320", fn_140d67320 as *const ()), // sub_140d67320
    Replacement::draft(0x140d67510, "content_140d67510", fn_140d67510 as *const ()), // sub_140d67510
    Replacement::draft(0x140d67610, "content_140d67610", fn_140d67610 as *const ()), // sub_140d67610
    Replacement::draft(0x140d67770, "content_140d67770", fn_140d67770 as *const ()), // sub_140d67770
    Replacement::draft(0x140d67810, "content_140d67810", fn_140d67810 as *const ()), // sub_140d67810
    Replacement::draft_throws(0x140d67d70, "content_140d67d70", fn_140d67d70 as *const (), Throws::Discard), // sub_140d67d70
    Replacement::draft(0x140d68200, "content_140d68200", fn_140d68200 as *const ()), // sub_140d68200
    Replacement::draft_throws(0x140d68470, "content_140d68470", fn_140d68470 as *const (), Throws::Discard), // sub_140d68470
    Replacement::draft(0x140d689e0, "content_140d689e0", fn_140d689e0 as *const ()), // sub_140d689e0
    Replacement::draft_throws(0x140d68bc0, "content_140d68bc0", fn_140d68bc0 as *const (), Throws::Caught), // sub_140d68bc0
    Replacement::draft_throws(0x140d69220, "content_140d69220", fn_140d69220 as *const (), Throws::Caught), // sub_140d69220
    Replacement::draft_throws(0x140d697f0, "content_140d697f0", fn_140d697f0 as *const (), Throws::Caught), // sub_140d697f0
    Replacement::draft(0x140d69db0, "content_140d69db0", fn_140d69db0 as *const ()), // sub_140d69db0
    Replacement::draft(0x140d6a060, "content_140d6a060", fn_140d6a060 as *const ()), // sub_140d6a060
    Replacement::draft(0x140d6a2f0, "content_140d6a2f0", fn_140d6a2f0 as *const ()), // sub_140d6a2f0
    Replacement::draft_throws(0x140d6a590, "content_140d6a590", fn_140d6a590 as *const (), Throws::Discard), // sub_140d6a590
    Replacement::draft_throws(0x140d6a880, "content_140d6a880", fn_140d6a880 as *const (), Throws::Discard), // sub_140d6a880
    Replacement::draft_throws(0x140d6ad50, "content_140d6ad50", fn_140d6ad50 as *const (), Throws::Discard), // sub_140d6ad50
    Replacement::draft(0x140d6af40, "content_140d6af40", fn_140d6af40 as *const ()), // sub_140d6af40
    Replacement::draft(0x140d6aff0, "content_140d6aff0", fn_140d6aff0 as *const ()), // sub_140d6aff0
    Replacement::draft(0x140d6b120, "content_140d6b120", fn_140d6b120 as *const ()), // sub_140d6b120
    Replacement::draft_throws(0x140d6b270, "content_140d6b270", fn_140d6b270 as *const (), Throws::Discard), // sub_140d6b270
    Replacement::draft(0x140d6b3a0, "content_140d6b3a0", tpl_16725::<0x14387f940, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x20, 0x58, 0x40, 0x5c, 0x60, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140d6b3a0
    Replacement::draft_throws(0x140d6b450, "content_140d6b450", fn_140d6b450 as *const (), Throws::Discard), // sub_140d6b450
    Replacement::draft(0x140d6b5a0, "content_140d6b5a0", fn_140d6b5a0 as *const ()), // sub_140d6b5a0
    Replacement::draft_throws(0x140d6b610, "content_140d6b610", fn_140d6b610 as *const (), Throws::Caught), // sub_140d6b610
    Replacement::draft_throws(0x140d6b760, "content_140d6b760", fn_140d6b760 as *const (), Throws::Caught), // sub_140d6b760
    Replacement::draft_throws(0x140d6b8b0, "content_140d6b8b0", fn_140d6b8b0 as *const (), Throws::Caught), // sub_140d6b8b0
    Replacement::draft(0x140d6ba00, "content_140d6ba00", fn_140d6ba00 as *const ()), // sub_140d6ba00
    Replacement::draft(0x140d6bae0, "content_140d6bae0", fn_140d6bae0 as *const ()), // sub_140d6bae0
    Replacement::draft(0x140d6bb90, "content_140d6bb90", tpl_16725::<0x14387f940, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xc0, 0x58, 0xe0, 0x5c, 0x60, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140d6bb90
    Replacement::draft_throws(0x140d6bc50, "content_140d6bc50", fn_140d6bc50 as *const (), Throws::Discard), // sub_140d6bc50
    Replacement::draft_throws(0x140d6bd10, "content_140d6bd10", fn_140d6bd10 as *const (), Throws::Discard), // sub_140d6bd10
    Replacement::draft(0x140d6be60, "content_140d6be60", fn_140d6be60 as *const ()), // sub_140d6be60
    Replacement::draft(0x140d6bf00, "content_140d6bf00", tpl_16733::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f0588, 0x18, 0x20, 0x28, 0x30> as *const ()), // sub_140d6bf00
    Replacement::draft(0x140d6bfa0, "content_140d6bfa0", fn_140d6bfa0 as *const ()), // sub_140d6bfa0
    Replacement::draft(0x140d6c010, "content_140d6c010", fn_140d6c010 as *const ()), // sub_140d6c010
    Replacement::draft_throws(0x140d6c1e0, "content_140d6c1e0", fn_140d6c1e0 as *const (), Throws::Discard), // sub_140d6c1e0
    Replacement::draft(0x140d6c360, "content_140d6c360", fn_140d6c360 as *const ()), // sub_140d6c360
    Replacement::draft_throws(0x140d6c440, "content_140d6c440", fn_140d6c440 as *const (), Throws::Discard), // sub_140d6c440
    Replacement::draft_throws(0x140d6c600, "content_140d6c600", fn_140d6c600 as *const (), Throws::Caught), // sub_140d6c600
    Replacement::draft_throws(0x140d6ca60, "content_140d6ca60", fn_140d6ca60 as *const (), Throws::Caught), // sub_140d6ca60
    Replacement::draft_throws(0x140d6ce40, "content_140d6ce40", fn_140d6ce40 as *const (), Throws::Caught), // sub_140d6ce40
    Replacement::draft(0x140d6d210, "content_140d6d210", fn_140d6d210 as *const ()), // sub_140d6d210
    Replacement::draft(0x140d6d300, "content_140d6d300", fn_140d6d300 as *const ()), // sub_140d6d300
    Replacement::draft(0x140d6d400, "content_140d6d400", fn_140d6d400 as *const ()), // sub_140d6d400
    Replacement::draft_throws(0x140d6d510, "content_140d6d510", fn_140d6d510 as *const (), Throws::Discard), // sub_140d6d510
    Replacement::draft_throws(0x140d6d630, "content_140d6d630", fn_140d6d630 as *const (), Throws::Discard), // sub_140d6d630
    Replacement::draft(0x140d6d820, "content_140d6d820", fn_140d6d820 as *const ()), // sub_140d6d820
    Replacement::draft(0x140d6d8e0, "content_140d6d8e0", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432f5ed8> as *const ()), // sub_140d6d8e0
    Replacement::draft(0x140d6d970, "content_140d6d970", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f5ed8, 0x18> as *const ()), // sub_140d6d970
    Replacement::draft(0x140d6da20, "content_140d6da20", tpl_11859::<0x14387f818, 0x14243b0b0, 0x140d8fac0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d6da20
    Replacement::draft(0x140d6da90, "content_140d6da90", tpl_16748::<0x14387f818, 0x14243b0b0, 0x140d8fb60, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d6da90
    Replacement::draft_throws(0x140d6db40, "content_140d6db40", tpl_16749::<0x14387faf8, 0x14387f818, 0x1401e9fa0, 0x140d8fc40, 0x328, 0x338, 0x10, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d6db40
    Replacement::draft(0x140d6dbd0, "content_140d6dbd0", tpl_16750::<0x14387f818, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d6dbd0
    Replacement::draft_throws(0x140d6dc50, "content_140d6dc50", tpl_16751::<0x14387faf8, 0x14387f818, 0x1401e9b60, 0x140d8fdf0, 0x328, 0x338, 0x120, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d6dc50
    Replacement::draft_throws(0x140d6dce0, "content_140d6dce0", fn_140d6dce0 as *const (), Throws::Caught), // sub_140d6dce0
    Replacement::draft_throws(0x140d6ddb0, "content_140d6ddb0", fn_140d6ddb0 as *const (), Throws::Caught), // sub_140d6ddb0
    Replacement::draft_throws(0x140d6de80, "content_140d6de80", fn_140d6de80 as *const (), Throws::Caught), // sub_140d6de80
    Replacement::draft(0x140d6df50, "content_140d6df50", tpl_16753::<0x14387f818, 0x14243b0b0, 0x140d901e0, 0x338, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d6df50
    Replacement::draft(0x140d6dfd0, "content_140d6dfd0", tpl_16754::<0x14387faf8, 0x14387f818, 0x1401e9970, 0x140d902a0, 0x338, 0x10, 0xe0, 0x1401e9860> as *const ()), // sub_140d6dfd0
    Replacement::draft(0x140d6e040, "content_140d6e040", tpl_16750::<0x14387f818, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d6e040
    Replacement::draft_throws(0x140d6e0c0, "content_140d6e0c0", tpl_16755::<0x14387faf8, 0x14387f818, 0x1401e9770, 0x140d90420, 0x338, 0x10, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140d6e0c0
    Replacement::draft_throws(0x140d6e140, "content_140d6e140", fn_140d6e140 as *const (), Throws::Discard), // sub_140d6e140
    Replacement::draft(0x140d6e210, "content_140d6e210", fn_140d6e210 as *const ()), // sub_140d6e210
    Replacement::draft(0x140d6e2b0, "content_140d6e2b0", tpl_16733::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f1058, 0x18, 0x20, 0x28, 0x30> as *const ()), // sub_140d6e2b0
    Replacement::draft(0x140d6e350, "content_140d6e350", fn_140d6e350 as *const ()), // sub_140d6e350
    Replacement::draft(0x140d6e3c0, "content_140d6e3c0", fn_140d6e3c0 as *const ()), // sub_140d6e3c0
    Replacement::draft_throws(0x140d6e590, "content_140d6e590", fn_140d6e590 as *const (), Throws::Discard), // sub_140d6e590
    Replacement::draft(0x140d6e720, "content_140d6e720", fn_140d6e720 as *const ()), // sub_140d6e720
    Replacement::draft_throws(0x140d6e810, "content_140d6e810", fn_140d6e810 as *const (), Throws::Discard), // sub_140d6e810
    Replacement::draft_throws(0x140d6e9d0, "content_140d6e9d0", fn_140d6e9d0 as *const (), Throws::Caught), // sub_140d6e9d0
    Replacement::draft_throws(0x140d6ed30, "content_140d6ed30", fn_140d6ed30 as *const (), Throws::Caught), // sub_140d6ed30
    Replacement::draft_throws(0x140d6f050, "content_140d6f050", fn_140d6f050 as *const (), Throws::Caught), // sub_140d6f050
    Replacement::draft(0x140d6f370, "content_140d6f370", fn_140d6f370 as *const ()), // sub_140d6f370
    Replacement::draft(0x140d6f450, "content_140d6f450", fn_140d6f450 as *const ()), // sub_140d6f450
    Replacement::draft(0x140d6f550, "content_140d6f550", fn_140d6f550 as *const ()), // sub_140d6f550
    Replacement::draft_throws(0x140d6f650, "content_140d6f650", fn_140d6f650 as *const (), Throws::Discard), // sub_140d6f650
    Replacement::draft_throws(0x140d6f770, "content_140d6f770", fn_140d6f770 as *const (), Throws::Discard), // sub_140d6f770
    Replacement::draft(0x140d6f960, "content_140d6f960", fn_140d6f960 as *const ()), // sub_140d6f960
    Replacement::draft(0x140d6fa00, "content_140d6fa00", fn_140d6fa00 as *const ()), // sub_140d6fa00
    Replacement::draft(0x140d6faa0, "content_140d6faa0", fn_140d6faa0 as *const ()), // sub_140d6faa0
    Replacement::draft(0x140d6fb50, "content_140d6fb50", fn_140d6fb50 as *const ()), // sub_140d6fb50
    Replacement::draft(0x140d6fc20, "content_140d6fc20", fn_140d6fc20 as *const ()), // sub_140d6fc20
    Replacement::draft_throws(0x140d6fe00, "content_140d6fe00", fn_140d6fe00 as *const (), Throws::Discard), // sub_140d6fe00
    Replacement::draft(0x140d6ff90, "content_140d6ff90", fn_140d6ff90 as *const ()), // sub_140d6ff90
    Replacement::draft_throws(0x140d700c0, "content_140d700c0", fn_140d700c0 as *const (), Throws::Discard), // sub_140d700c0
    Replacement::draft(0x140d702a0, "content_140d702a0", fn_140d702a0 as *const ()), // sub_140d702a0
    Replacement::draft_throws(0x140d70400, "content_140d70400", fn_140d70400 as *const (), Throws::Caught), // sub_140d70400
    Replacement::draft_throws(0x140d70710, "content_140d70710", fn_140d70710 as *const (), Throws::Caught), // sub_140d70710
    Replacement::draft_throws(0x140d709f0, "content_140d709f0", fn_140d709f0 as *const (), Throws::Caught), // sub_140d709f0
    Replacement::draft(0x140d70cd0, "content_140d70cd0", fn_140d70cd0 as *const ()), // sub_140d70cd0
    Replacement::draft(0x140d70df0, "content_140d70df0", fn_140d70df0 as *const ()), // sub_140d70df0
    Replacement::draft(0x140d70f20, "content_140d70f20", fn_140d70f20 as *const ()), // sub_140d70f20
    Replacement::draft_throws(0x140d71060, "content_140d71060", fn_140d71060 as *const (), Throws::Discard), // sub_140d71060
    Replacement::draft_throws(0x140d711a0, "content_140d711a0", fn_140d711a0 as *const (), Throws::Discard), // sub_140d711a0
    Replacement::draft(0x140d71350, "content_140d71350", fn_140d71350 as *const ()), // sub_140d71350
    Replacement::draft(0x140d713f0, "content_140d713f0", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432f70f8, 0x20> as *const ()), // sub_140d713f0
    Replacement::draft(0x140d71480, "content_140d71480", tpl_16783::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f70f8, 0x18, 0x20, 0x28> as *const ()), // sub_140d71480
    Replacement::draft(0x140d715a0, "content_140d715a0", fn_140d715a0 as *const ()), // sub_140d715a0
    Replacement::draft_throws(0x140d716e0, "content_140d716e0", fn_140d716e0 as *const (), Throws::Discard), // sub_140d716e0
    Replacement::draft(0x140d71800, "content_140d71800", tpl_7561::<0x14387f728, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x14, 0x18, 0x1c, 0x80, 0x20, 0x140916ef0> as *const ()), // sub_140d71800
    Replacement::draft_throws(0x140d718c0, "content_140d718c0", fn_140d718c0 as *const (), Throws::Discard), // sub_140d718c0
    Replacement::draft_throws(0x140d71a00, "content_140d71a00", fn_140d71a00 as *const (), Throws::Caught), // sub_140d71a00
    Replacement::draft_throws(0x140d71bb0, "content_140d71bb0", fn_140d71bb0 as *const (), Throws::Caught), // sub_140d71bb0
    Replacement::draft_throws(0x140d71d60, "content_140d71d60", fn_140d71d60 as *const (), Throws::Caught), // sub_140d71d60
    Replacement::draft(0x140d71f10, "content_140d71f10", fn_140d71f10 as *const ()), // sub_140d71f10
    Replacement::draft(0x140d71fe0, "content_140d71fe0", tpl_7566::<0x14387f7ac, 0x14387f728, 0x1401e9970, 0x140d902a0, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x1c, 0x120, 0x20, 0x140917930, 0x1401e9860> as *const ()), // sub_140d71fe0
    Replacement::draft(0x140d720a0, "content_140d720a0", tpl_7561::<0x14387f728, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x1c, 0x120, 0x20, 0x140917aa0> as *const ()), // sub_140d720a0
    Replacement::draft_throws(0x140d72170, "content_140d72170", fn_140d72170 as *const (), Throws::Discard), // sub_140d72170
    Replacement::draft_throws(0x140d72250, "content_140d72250", fn_140d72250 as *const (), Throws::Discard), // sub_140d72250
    Replacement::draft(0x140d723c0, "content_140d723c0", fn_140d723c0 as *const ()), // sub_140d723c0
    Replacement::draft(0x140d72470, "content_140d72470", fn_140d72470 as *const ()), // sub_140d72470
    Replacement::draft(0x140d725a0, "content_140d725a0", fn_140d725a0 as *const ()), // sub_140d725a0
    Replacement::draft_throws(0x140d726c0, "content_140d726c0", fn_140d726c0 as *const (), Throws::Discard), // sub_140d726c0
    Replacement::draft(0x140d727d0, "content_140d727d0", tpl_16794::<0x14387f880, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x20, 0x38, 0x40, 0x3c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140d727d0
    Replacement::draft_throws(0x140d72870, "content_140d72870", fn_140d72870 as *const (), Throws::Discard), // sub_140d72870
    Replacement::draft(0x140d72990, "content_140d72990", fn_140d72990 as *const ()), // sub_140d72990
    Replacement::draft_throws(0x140d729f0, "content_140d729f0", fn_140d729f0 as *const (), Throws::Caught), // sub_140d729f0
    Replacement::draft_throws(0x140d72cc0, "content_140d72cc0", fn_140d72cc0 as *const (), Throws::Caught), // sub_140d72cc0
    Replacement::draft_throws(0x140d72f50, "content_140d72f50", fn_140d72f50 as *const (), Throws::Caught), // sub_140d72f50
    Replacement::draft(0x140d731f0, "content_140d731f0", fn_140d731f0 as *const ()), // sub_140d731f0
    Replacement::draft(0x140d732c0, "content_140d732c0", fn_140d732c0 as *const ()), // sub_140d732c0
    Replacement::draft(0x140d73360, "content_140d73360", tpl_16794::<0x14387f880, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xc0, 0x38, 0xe0, 0x3c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140d73360
    Replacement::draft_throws(0x140d73410, "content_140d73410", fn_140d73410 as *const (), Throws::Discard), // sub_140d73410
    Replacement::draft_throws(0x140d734b0, "content_140d734b0", fn_140d734b0 as *const (), Throws::Discard), // sub_140d734b0
    Replacement::draft(0x140d735e0, "content_140d735e0", fn_140d735e0 as *const ()), // sub_140d735e0
    Replacement::draft(0x140d736c0, "content_140d736c0", fn_140d736c0 as *const ()), // sub_140d736c0
    Replacement::draft(0x140d737c0, "content_140d737c0", fn_140d737c0 as *const ()), // sub_140d737c0
    Replacement::draft(0x140d73830, "content_140d73830", fn_140d73830 as *const ()), // sub_140d73830
    Replacement::draft_throws(0x140d739f0, "content_140d739f0", fn_140d739f0 as *const (), Throws::Discard), // sub_140d739f0
    Replacement::draft(0x140d73bc0, "content_140d73bc0", fn_140d73bc0 as *const ()), // sub_140d73bc0
    Replacement::draft_throws(0x140d73cd0, "content_140d73cd0", fn_140d73cd0 as *const (), Throws::Discard), // sub_140d73cd0
    Replacement::draft(0x140d73ec0, "content_140d73ec0", fn_140d73ec0 as *const ()), // sub_140d73ec0
    Replacement::draft_throws(0x140d73f80, "content_140d73f80", fn_140d73f80 as *const (), Throws::Caught), // sub_140d73f80
    Replacement::draft_throws(0x140d74380, "content_140d74380", fn_140d74380 as *const (), Throws::Caught), // sub_140d74380
    Replacement::draft_throws(0x140d74750, "content_140d74750", fn_140d74750 as *const (), Throws::Caught), // sub_140d74750
    Replacement::draft(0x140d74b10, "content_140d74b10", fn_140d74b10 as *const ()), // sub_140d74b10
    Replacement::draft(0x140d74c40, "content_140d74c40", fn_140d74c40 as *const ()), // sub_140d74c40
    Replacement::draft(0x140d74d40, "content_140d74d40", fn_140d74d40 as *const ()), // sub_140d74d40
    Replacement::draft_throws(0x140d74e50, "content_140d74e50", fn_140d74e50 as *const (), Throws::Discard), // sub_140d74e50
    Replacement::draft_throws(0x140d74f80, "content_140d74f80", fn_140d74f80 as *const (), Throws::Discard), // sub_140d74f80
    Replacement::draft(0x140d75190, "content_140d75190", fn_140d75190 as *const ()), // sub_140d75190
    Replacement::draft(0x140d752b0, "content_140d752b0", tpl_16694::<0x1432ec430, 0x10, 0x1, 0x1409fe2d0, 0x18, 0x1432f8998, 0x5> as *const ()), // sub_140d752b0
    Replacement::draft(0x140d75320, "content_140d75320", tpl_16820::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432f2918, 0x20, 0x28> as *const ()), // sub_140d75320
    Replacement::draft(0x140d753c0, "content_140d753c0", tpl_16821::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x1432f2918, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0xffffffffffffffe0> as *const ()), // sub_140d753c0
    Replacement::draft(0x140d75470, "content_140d75470", tpl_16822::<0x14387fb30, 0x14243b0b0, 0x140d8fac0, 0x20, 0x28, 0x14387f9e8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d75470
    Replacement::draft(0x140d75550, "content_140d75550", tpl_16823::<0x14387fb30, 0x14243b0b0, 0x140d8fb60, 0x1426d4580, 0x140da99e0> as *const ()), // sub_140d75550
    Replacement::draft_throws(0x140d756e0, "content_140d756e0", fn_140d756e0 as *const (), Throws::Caught), // sub_140d756e0
    Replacement::draft(0x140d75820, "content_140d75820", tpl_16825::<0x14387fb30, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x14, 0x18, 0x48, 0x20, 0x140dfbc80, 0x140da9c60> as *const ()), // sub_140d75820
    Replacement::draft_throws(0x140d75920, "content_140d75920", fn_140d75920 as *const (), Throws::Caught), // sub_140d75920
    Replacement::draft(0x140d75a70, "content_140d75a70", fn_140d75a70 as *const ()), // sub_140d75a70
    Replacement::draft_throws(0x140d75b50, "content_140d75b50", fn_140d75b50 as *const (), Throws::Caught), // sub_140d75b50
    Replacement::draft_throws(0x140d75cb0, "content_140d75cb0", fn_140d75cb0 as *const (), Throws::Caught), // sub_140d75cb0
    Replacement::draft_throws(0x140d75e10, "content_140d75e10", fn_140d75e10 as *const (), Throws::Caught), // sub_140d75e10
    Replacement::draft(0x140d75f70, "content_140d75f70", tpl_75182::<0x14387fb30, 0x14243b0b0, 0x140d901e0, 0x140daa140> as *const ()), // sub_140d75f70
    Replacement::draft(0x140d76070, "content_140d76070", tpl_75183::<0x14387f77c, 0x14387fb30, 0x1401e9970, 0x140d902a0, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x28, 0xe8, 0x20, 0x140daa1f0, 0x1401e9860> as *const ()), // sub_140d76070
    Replacement::draft(0x140d76160, "content_140d76160", tpl_75184::<0x14387fb30, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x28, 0xe8, 0x20, 0x140daa260> as *const ()), // sub_140d76160
    Replacement::draft_throws(0x140d76260, "content_140d76260", tpl_75185::<0x14387f77c, 0x14387fb30, 0x1401e9770, 0x140d90420, 0x338, 0x10, 0x120, 0x14, 0x18, 0x28, 0x20, 0x140daa2e0, 0x1401e9590> as *const (), Throws::Discard), // sub_140d76260
    Replacement::draft_throws(0x140d76360, "content_140d76360", tpl_16829::<0x14387fb30, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140d904e0, 0x1432f84b0, 0x14020f120, 0x1432f84c0, 0x1432f8660, 0x1432f8488, 0x140dce7f0> as *const (), Throws::Discard), // sub_140d76360
    Replacement::draft(0x140d764b0, "content_140d764b0", tpl_75186::<0x20, 0x28, 0x1426cdcb0, 0x140ddefe0> as *const ()), // sub_140d764b0
    Replacement::draft(0x140d76530, "content_140d76530", tpl_75186::<0x20, 0x28, 0x1432f8488, 0x140ddeff0> as *const ()), // sub_140d76530
    Replacement::draft(0x140d765b0, "content_140d765b0", tpl_8850::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x1432f29c8, 0x14, 0x8> as *const ()), // sub_140d765b0
    Replacement::draft(0x140d76640, "content_140d76640", tpl_16830::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f29c8, 0x14, 0x18> as *const ()), // sub_140d76640
    Replacement::draft(0x140d766f0, "content_140d766f0", tpl_9543::<0x14387fa68, 0x14243b0b0, 0x14387e8c8, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140d766f0
    Replacement::draft(0x140d76790, "content_140d76790", fn_140d76790 as *const ()), // sub_140d76790
    Replacement::draft_throws(0x140d768e0, "content_140d768e0", fn_140d768e0 as *const (), Throws::Discard), // sub_140d768e0
    Replacement::draft(0x140d769f0, "content_140d769f0", fn_140d769f0 as *const ()), // sub_140d769f0
    Replacement::draft_throws(0x140d76ac0, "content_140d76ac0", fn_140d76ac0 as *const (), Throws::Discard), // sub_140d76ac0
    Replacement::draft_throws(0x140d76be0, "content_140d76be0", fn_140d76be0 as *const (), Throws::Caught), // sub_140d76be0
    Replacement::draft_throws(0x140d76e50, "content_140d76e50", fn_140d76e50 as *const (), Throws::Caught), // sub_140d76e50
    Replacement::draft_throws(0x140d77090, "content_140d77090", fn_140d77090 as *const (), Throws::Caught), // sub_140d77090
    Replacement::draft(0x140d772d0, "content_140d772d0", fn_140d772d0 as *const ()), // sub_140d772d0
    Replacement::draft(0x140d773a0, "content_140d773a0", fn_140d773a0 as *const ()), // sub_140d773a0
    Replacement::draft(0x140d77470, "content_140d77470", fn_140d77470 as *const ()), // sub_140d77470
    Replacement::draft_throws(0x140d77560, "content_140d77560", fn_140d77560 as *const (), Throws::Discard), // sub_140d77560
    Replacement::draft_throws(0x140d77630, "content_140d77630", fn_140d77630 as *const (), Throws::Discard), // sub_140d77630
    Replacement::draft(0x140d77770, "content_140d77770", fn_140d77770 as *const ()), // sub_140d77770
    Replacement::draft(0x140d77810, "content_140d77810", fn_140d77810 as *const ()), // sub_140d77810
    Replacement::draft(0x140d778c0, "content_140d778c0", fn_140d778c0 as *const ()), // sub_140d778c0
    Replacement::draft(0x140d77940, "content_140d77940", fn_140d77940 as *const ()), // sub_140d77940
    Replacement::draft_throws(0x140d77ac0, "content_140d77ac0", fn_140d77ac0 as *const (), Throws::Discard), // sub_140d77ac0
    Replacement::draft(0x140d77c20, "content_140d77c20", fn_140d77c20 as *const ()), // sub_140d77c20
    Replacement::draft_throws(0x140d77d70, "content_140d77d70", fn_140d77d70 as *const (), Throws::Discard), // sub_140d77d70
    Replacement::draft(0x140d77e60, "content_140d77e60", fn_140d77e60 as *const ()), // sub_140d77e60
    Replacement::draft_throws(0x140d77ed0, "content_140d77ed0", fn_140d77ed0 as *const (), Throws::Caught), // sub_140d77ed0
    Replacement::draft_throws(0x140d78010, "content_140d78010", fn_140d78010 as *const (), Throws::Caught), // sub_140d78010
    Replacement::draft_throws(0x140d78150, "content_140d78150", fn_140d78150 as *const (), Throws::Caught), // sub_140d78150
    Replacement::draft(0x140d78290, "content_140d78290", fn_140d78290 as *const ()), // sub_140d78290
    Replacement::draft(0x140d783e0, "content_140d783e0", fn_140d783e0 as *const ()), // sub_140d783e0
    Replacement::draft(0x140d78520, "content_140d78520", fn_140d78520 as *const ()), // sub_140d78520
    Replacement::draft_throws(0x140d78660, "content_140d78660", fn_140d78660 as *const (), Throws::Discard), // sub_140d78660
    Replacement::draft_throws(0x140d78790, "content_140d78790", fn_140d78790 as *const (), Throws::Discard), // sub_140d78790
    Replacement::draft(0x140d788c0, "content_140d788c0", fn_140d788c0 as *const ()), // sub_140d788c0
    Replacement::draft(0x140d789c0, "content_140d789c0", fn_140d789c0 as *const ()), // sub_140d789c0
    Replacement::draft(0x140d78ae0, "content_140d78ae0", fn_140d78ae0 as *const ()), // sub_140d78ae0
    Replacement::draft(0x140d78c70, "content_140d78c70", fn_140d78c70 as *const ()), // sub_140d78c70
    Replacement::draft_throws(0x140d78e60, "content_140d78e60", fn_140d78e60 as *const (), Throws::Discard), // sub_140d78e60
    Replacement::draft(0x140d79090, "content_140d79090", fn_140d79090 as *const ()), // sub_140d79090
    Replacement::draft_throws(0x140d792a0, "content_140d792a0", fn_140d792a0 as *const (), Throws::Discard), // sub_140d792a0
    Replacement::draft(0x140d79540, "content_140d79540", fn_140d79540 as *const ()), // sub_140d79540
    Replacement::draft_throws(0x140d79690, "content_140d79690", fn_140d79690 as *const (), Throws::Caught), // sub_140d79690
    Replacement::draft_throws(0x140d799d0, "content_140d799d0", fn_140d799d0 as *const (), Throws::Caught), // sub_140d799d0
    Replacement::draft_throws(0x140d79ce0, "content_140d79ce0", fn_140d79ce0 as *const (), Throws::Caught), // sub_140d79ce0
    Replacement::draft(0x140d79ff0, "content_140d79ff0", fn_140d79ff0 as *const ()), // sub_140d79ff0
    Replacement::draft(0x140d7a230, "content_140d7a230", fn_140d7a230 as *const ()), // sub_140d7a230
    Replacement::draft(0x140d7a430, "content_140d7a430", fn_140d7a430 as *const ()), // sub_140d7a430
    Replacement::draft_throws(0x140d7a640, "content_140d7a640", fn_140d7a640 as *const (), Throws::Discard), // sub_140d7a640
    Replacement::draft_throws(0x140d7a840, "content_140d7a840", fn_140d7a840 as *const (), Throws::Discard), // sub_140d7a840
    Replacement::draft(0x140d7a9e0, "content_140d7a9e0", fn_140d7a9e0 as *const ()), // sub_140d7a9e0
    Replacement::draft(0x140d7ab40, "content_140d7ab40", fn_140d7ab40 as *const ()), // sub_140d7ab40
    Replacement::draft(0x140d7aca0, "content_140d7aca0", fn_140d7aca0 as *const ()), // sub_140d7aca0
    Replacement::draft(0x140d7ad50, "content_140d7ad50", fn_140d7ad50 as *const ()), // sub_140d7ad50
    Replacement::draft(0x140d7ae00, "content_140d7ae00", fn_140d7ae00 as *const ()), // sub_140d7ae00
    Replacement::draft(0x140d7aee0, "content_140d7aee0", fn_140d7aee0 as *const ()), // sub_140d7aee0
    Replacement::draft_throws(0x140d7b250, "content_140d7b250", fn_140d7b250 as *const (), Throws::Discard), // sub_140d7b250
    Replacement::draft(0x140d7b530, "content_140d7b530", fn_140d7b530 as *const ()), // sub_140d7b530
    Replacement::draft_throws(0x140d7b700, "content_140d7b700", fn_140d7b700 as *const (), Throws::Discard), // sub_140d7b700
    Replacement::draft(0x140d7baa0, "content_140d7baa0", fn_140d7baa0 as *const ()), // sub_140d7baa0
    Replacement::draft_throws(0x140d7bc20, "content_140d7bc20", fn_140d7bc20 as *const (), Throws::Caught), // sub_140d7bc20
    Replacement::draft_throws(0x140d7c240, "content_140d7c240", fn_140d7c240 as *const (), Throws::Caught), // sub_140d7c240
    Replacement::draft_throws(0x140d7c7a0, "content_140d7c7a0", fn_140d7c7a0 as *const (), Throws::Caught), // sub_140d7c7a0
    Replacement::draft(0x140d7cd00, "content_140d7cd00", fn_140d7cd00 as *const ()), // sub_140d7cd00
    Replacement::draft(0x140d7cea0, "content_140d7cea0", fn_140d7cea0 as *const ()), // sub_140d7cea0
    Replacement::draft(0x140d7d0c0, "content_140d7d0c0", fn_140d7d0c0 as *const ()), // sub_140d7d0c0
    Replacement::draft_throws(0x140d7d2c0, "content_140d7d2c0", fn_140d7d2c0 as *const (), Throws::Discard), // sub_140d7d2c0
    Replacement::draft_throws(0x140d7d530, "content_140d7d530", fn_140d7d530 as *const (), Throws::Discard), // sub_140d7d530
    Replacement::draft(0x140d7d8a0, "content_140d7d8a0", tpl_16880::<0x10, 0x18, 0x1426cdcb0, 0x14088eb40, 0x143877d30, 0x1432f8310, 0x1432f84d8, 0x1401de410, 0x143877a80, 0x1432f82d0, 0x1432f82f0> as *const ()), // sub_140d7d8a0
    Replacement::draft(0x140d7d990, "content_140d7d990", fn_140d7d990 as *const ()), // sub_140d7d990
    Replacement::draft(0x140d7db00, "content_140d7db00", tpl_16880::<0x10, 0x18, 0x1432f88e8, 0x140de3d50, 0x143877f38, 0x1432f8310, 0x1432f84d8, 0x1401de410, 0x143877f58, 0x1432f82d0, 0x1432f82f0> as *const ()), // sub_140d7db00
    Replacement::draft(0x140d7dbf0, "content_140d7dbf0", tpl_7834::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432edc68, 0x20> as *const ()), // sub_140d7dbf0
    Replacement::draft(0x140d7dc80, "content_140d7dc80", tpl_16783::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432edc68, 0x18, 0x20, 0x28> as *const ()), // sub_140d7dc80
    Replacement::draft(0x140d7dda0, "content_140d7dda0", fn_140d7dda0 as *const ()), // sub_140d7dda0
    Replacement::draft_throws(0x140d7dec0, "content_140d7dec0", fn_140d7dec0 as *const (), Throws::Discard), // sub_140d7dec0
    Replacement::draft(0x140d7dfc0, "content_140d7dfc0", tpl_16884::<0x14387faa8, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x14, 0x18, 0x20, 0x1409068b0, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140d7dfc0
    Replacement::draft_throws(0x140d7e070, "content_140d7e070", fn_140d7e070 as *const (), Throws::Discard), // sub_140d7e070
    Replacement::draft_throws(0x140d7e180, "content_140d7e180", fn_140d7e180 as *const (), Throws::Caught), // sub_140d7e180
    Replacement::draft_throws(0x140d7e300, "content_140d7e300", fn_140d7e300 as *const (), Throws::Caught), // sub_140d7e300
    Replacement::draft_throws(0x140d7e480, "content_140d7e480", fn_140d7e480 as *const (), Throws::Caught), // sub_140d7e480
    Replacement::draft(0x140d7e600, "content_140d7e600", fn_140d7e600 as *const ()), // sub_140d7e600
    Replacement::draft(0x140d7e6c0, "content_140d7e6c0", fn_140d7e6c0 as *const ()), // sub_140d7e6c0
    Replacement::draft(0x140d7e770, "content_140d7e770", tpl_16884::<0x14387faa8, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x20, 0x140907460, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140d7e770
    Replacement::draft_throws(0x140d7e820, "content_140d7e820", fn_140d7e820 as *const (), Throws::Discard), // sub_140d7e820
    Replacement::draft_throws(0x140d7e8e0, "content_140d7e8e0", fn_140d7e8e0 as *const (), Throws::Discard), // sub_140d7e8e0
    Replacement::draft(0x140d7ea20, "content_140d7ea20", fn_140d7ea20 as *const ()), // sub_140d7ea20
    Replacement::draft(0x140d7eaf0, "content_140d7eaf0", fn_140d7eaf0 as *const ()), // sub_140d7eaf0
    Replacement::draft(0x140d7ec00, "content_140d7ec00", tpl_9031::<0x14387f748, 0x14243b0b0, 0x140d8fac0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d7ec00
    Replacement::draft(0x140d7ec70, "content_140d7ec70", fn_140d7ec70 as *const ()), // sub_140d7ec70
    Replacement::draft_throws(0x140d7ef20, "content_140d7ef20", fn_140d7ef20 as *const (), Throws::Discard), // sub_140d7ef20
    Replacement::draft(0x140d7f150, "content_140d7f150", fn_140d7f150 as *const ()), // sub_140d7f150
    Replacement::draft_throws(0x140d7f270, "content_140d7f270", fn_140d7f270 as *const (), Throws::Discard), // sub_140d7f270
    Replacement::draft(0x140d7f500, "content_140d7f500", fn_140d7f500 as *const ()), // sub_140d7f500
    Replacement::draft_throws(0x140d7f5f0, "content_140d7f5f0", fn_140d7f5f0 as *const (), Throws::Caught), // sub_140d7f5f0
    Replacement::draft_throws(0x140d7f840, "content_140d7f840", fn_140d7f840 as *const (), Throws::Caught), // sub_140d7f840
    Replacement::draft_throws(0x140d7fa90, "content_140d7fa90", fn_140d7fa90 as *const (), Throws::Caught), // sub_140d7fa90
    Replacement::draft(0x140d7fce0, "content_140d7fce0", fn_140d7fce0 as *const ()), // sub_140d7fce0
    Replacement::draft(0x140d7fe50, "content_140d7fe50", fn_140d7fe50 as *const ()), // sub_140d7fe50
    Replacement::draft(0x140d7ff80, "content_140d7ff80", fn_140d7ff80 as *const ()), // sub_140d7ff80
    Replacement::draft_throws(0x140d800b0, "content_140d800b0", fn_140d800b0 as *const (), Throws::Discard), // sub_140d800b0
    Replacement::draft_throws(0x140d80210, "content_140d80210", fn_140d80210 as *const (), Throws::Discard), // sub_140d80210
    Replacement::draft(0x140d80470, "content_140d80470", fn_140d80470 as *const ()), // sub_140d80470
    Replacement::draft(0x140d804f0, "content_140d804f0", fn_140d804f0 as *const ()), // sub_140d804f0
    Replacement::draft(0x140d805c0, "content_140d805c0", fn_140d805c0 as *const ()), // sub_140d805c0
    Replacement::draft(0x140d80740, "content_140d80740", fn_140d80740 as *const ()), // sub_140d80740
    Replacement::draft_throws(0x140d80940, "content_140d80940", fn_140d80940 as *const (), Throws::Discard), // sub_140d80940
    Replacement::draft(0x140d80af0, "content_140d80af0", fn_140d80af0 as *const ()), // sub_140d80af0
    Replacement::draft_throws(0x140d80bd0, "content_140d80bd0", fn_140d80bd0 as *const (), Throws::Discard), // sub_140d80bd0
    Replacement::draft(0x140d80db0, "content_140d80db0", fn_140d80db0 as *const ()), // sub_140d80db0
    Replacement::draft_throws(0x140d80e60, "content_140d80e60", fn_140d80e60 as *const (), Throws::Caught), // sub_140d80e60
    Replacement::draft_throws(0x140d81000, "content_140d81000", fn_140d81000 as *const (), Throws::Caught), // sub_140d81000
    Replacement::draft_throws(0x140d811a0, "content_140d811a0", fn_140d811a0 as *const (), Throws::Caught), // sub_140d811a0
    Replacement::draft(0x140d81340, "content_140d81340", fn_140d81340 as *const ()), // sub_140d81340
    Replacement::draft(0x140d81460, "content_140d81460", fn_140d81460 as *const ()), // sub_140d81460
    Replacement::draft(0x140d81540, "content_140d81540", fn_140d81540 as *const ()), // sub_140d81540
    Replacement::draft_throws(0x140d81630, "content_140d81630", fn_140d81630 as *const (), Throws::Discard), // sub_140d81630
    Replacement::draft_throws(0x140d81720, "content_140d81720", fn_140d81720 as *const (), Throws::Discard), // sub_140d81720
    Replacement::draft(0x140d818d0, "content_140d818d0", fn_140d818d0 as *const ()), // sub_140d818d0
    Replacement::draft(0x140d819a0, "content_140d819a0", fn_140d819a0 as *const ()), // sub_140d819a0
    Replacement::draft(0x140d81a70, "content_140d81a70", fn_140d81a70 as *const ()), // sub_140d81a70
    Replacement::draft(0x140d81c20, "content_140d81c20", fn_140d81c20 as *const ()), // sub_140d81c20
    Replacement::draft_throws(0x140d81ee0, "content_140d81ee0", fn_140d81ee0 as *const (), Throws::Discard), // sub_140d81ee0
    Replacement::draft(0x140d82080, "content_140d82080", fn_140d82080 as *const ()), // sub_140d82080
    Replacement::draft_throws(0x140d82230, "content_140d82230", fn_140d82230 as *const (), Throws::Discard), // sub_140d82230
    Replacement::draft(0x140d82410, "content_140d82410", fn_140d82410 as *const ()), // sub_140d82410
    Replacement::draft_throws(0x140d826a0, "content_140d826a0", fn_140d826a0 as *const (), Throws::Caught), // sub_140d826a0
    Replacement::draft_throws(0x140d829f0, "content_140d829f0", fn_140d829f0 as *const (), Throws::Caught), // sub_140d829f0
    Replacement::draft_throws(0x140d82d00, "content_140d82d00", fn_140d82d00 as *const (), Throws::Caught), // sub_140d82d00
    Replacement::draft(0x140d83000, "content_140d83000", fn_140d83000 as *const ()), // sub_140d83000
    Replacement::draft(0x140d831d0, "content_140d831d0", fn_140d831d0 as *const ()), // sub_140d831d0
    Replacement::draft(0x140d83370, "content_140d83370", fn_140d83370 as *const ()), // sub_140d83370
    Replacement::draft_throws(0x140d83520, "content_140d83520", fn_140d83520 as *const (), Throws::Discard), // sub_140d83520
    Replacement::draft_throws(0x140d836c0, "content_140d836c0", fn_140d836c0 as *const (), Throws::Discard), // sub_140d836c0
    Replacement::draft(0x140d83870, "content_140d83870", tpl_75295::<0x10, 0x18, 0x1426cdcb0, 0x140de6420> as *const ()), // sub_140d83870
    Replacement::draft(0x140d838d0, "content_140d838d0", tpl_75295::<0x10, 0x18, 0x1432f95f0, 0x140de6460> as *const ()), // sub_140d838d0
    Replacement::draft(0x140d83930, "content_140d83930", tpl_8792::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432eeb68, 0x20, 0x28> as *const ()), // sub_140d83930
    Replacement::draft(0x140d839d0, "content_140d839d0", fn_140d839d0 as *const ()), // sub_140d839d0
    Replacement::draft(0x140d83a80, "content_140d83a80", fn_140d83a80 as *const ()), // sub_140d83a80
    Replacement::draft(0x140d83b50, "content_140d83b50", fn_140d83b50 as *const ()), // sub_140d83b50
    Replacement::draft_throws(0x140d83ce0, "content_140d83ce0", fn_140d83ce0 as *const (), Throws::Discard), // sub_140d83ce0
    Replacement::draft(0x140d83e20, "content_140d83e20", fn_140d83e20 as *const ()), // sub_140d83e20
    Replacement::draft_throws(0x140d83f20, "content_140d83f20", fn_140d83f20 as *const (), Throws::Discard), // sub_140d83f20
    Replacement::draft(0x140d840a0, "content_140d840a0", fn_140d840a0 as *const ()), // sub_140d840a0
    Replacement::draft_throws(0x140d84160, "content_140d84160", fn_140d84160 as *const (), Throws::Caught), // sub_140d84160
    Replacement::draft_throws(0x140d842c0, "content_140d842c0", fn_140d842c0 as *const (), Throws::Caught), // sub_140d842c0
    Replacement::draft_throws(0x140d84420, "content_140d84420", fn_140d84420 as *const (), Throws::Caught), // sub_140d84420
    Replacement::draft(0x140d84580, "content_140d84580", fn_140d84580 as *const ()), // sub_140d84580
    Replacement::draft(0x140d84680, "content_140d84680", fn_140d84680 as *const ()), // sub_140d84680
    Replacement::draft(0x140d847b0, "content_140d847b0", fn_140d847b0 as *const ()), // sub_140d847b0
    Replacement::draft_throws(0x140d848c0, "content_140d848c0", fn_140d848c0 as *const (), Throws::Discard), // sub_140d848c0
    Replacement::draft_throws(0x140d84a00, "content_140d84a00", fn_140d84a00 as *const (), Throws::Discard), // sub_140d84a00
    Replacement::draft(0x140d84b50, "content_140d84b50", tpl_16933::<0x18, 0x10, 0x1426cdcb0, 0x14088eb40> as *const ()), // sub_140d84b50
    Replacement::draft(0x140d84bd0, "content_140d84bd0", tpl_16933::<0x18, 0x10, 0x1432f88e8, 0x140de3d50> as *const ()), // sub_140d84bd0
    Replacement::draft(0x140d84c50, "content_140d84c50", tpl_11529::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432f4cb8> as *const ()), // sub_140d84c50
    Replacement::draft(0x140d84ce0, "content_140d84ce0", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f4cb8, 0x18, 0x20> as *const ()), // sub_140d84ce0
    Replacement::draft(0x140d84df0, "content_140d84df0", fn_140d84df0 as *const ()), // sub_140d84df0
    Replacement::draft_throws(0x140d84f00, "content_140d84f00", fn_140d84f00 as *const (), Throws::Discard), // sub_140d84f00
    Replacement::draft(0x140d84ff0, "content_140d84ff0", tpl_16935::<0x14387f7d0, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x14, 0x18, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140d84ff0
    Replacement::draft_throws(0x140d85090, "content_140d85090", fn_140d85090 as *const (), Throws::Discard), // sub_140d85090
    Replacement::draft_throws(0x140d85190, "content_140d85190", fn_140d85190 as *const (), Throws::Caught), // sub_140d85190
    Replacement::draft_throws(0x140d852d0, "content_140d852d0", fn_140d852d0 as *const (), Throws::Caught), // sub_140d852d0
    Replacement::draft_throws(0x140d85410, "content_140d85410", fn_140d85410 as *const (), Throws::Caught), // sub_140d85410
    Replacement::draft(0x140d85550, "content_140d85550", fn_140d85550 as *const ()), // sub_140d85550
    Replacement::draft(0x140d855f0, "content_140d855f0", fn_140d855f0 as *const ()), // sub_140d855f0
    Replacement::draft(0x140d85680, "content_140d85680", tpl_16935::<0x14387f7d0, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x14, 0x18, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140d85680
    Replacement::draft_throws(0x140d85720, "content_140d85720", fn_140d85720 as *const (), Throws::Discard), // sub_140d85720
    Replacement::draft_throws(0x140d857d0, "content_140d857d0", fn_140d857d0 as *const (), Throws::Discard), // sub_140d857d0
    Replacement::draft(0x140d85900, "content_140d85900", fn_140d85900 as *const ()), // sub_140d85900
    Replacement::draft(0x140d85a30, "content_140d85a30", fn_140d85a30 as *const ()), // sub_140d85a30
    Replacement::draft(0x140d85ba0, "content_140d85ba0", fn_140d85ba0 as *const ()), // sub_140d85ba0
    Replacement::draft(0x140d85c10, "content_140d85c10", fn_140d85c10 as *const ()), // sub_140d85c10
    Replacement::draft_throws(0x140d86080, "content_140d86080", fn_140d86080 as *const (), Throws::Discard), // sub_140d86080
    Replacement::draft(0x140d864f0, "content_140d864f0", fn_140d864f0 as *const ()), // sub_140d864f0
    Replacement::draft_throws(0x140d86710, "content_140d86710", fn_140d86710 as *const (), Throws::Discard), // sub_140d86710
    Replacement::draft(0x140d86c10, "content_140d86c10", fn_140d86c10 as *const ()), // sub_140d86c10
    Replacement::draft_throws(0x140d86dc0, "content_140d86dc0", fn_140d86dc0 as *const (), Throws::Caught), // sub_140d86dc0
    Replacement::draft_throws(0x140d876a0, "content_140d876a0", fn_140d876a0 as *const (), Throws::Caught), // sub_140d876a0
    Replacement::draft_throws(0x140d87f30, "content_140d87f30", fn_140d87f30 as *const (), Throws::Caught), // sub_140d87f30
    Replacement::draft(0x140d88790, "content_140d88790", fn_140d88790 as *const ()), // sub_140d88790
    Replacement::draft(0x140d88a30, "content_140d88a30", fn_140d88a30 as *const ()), // sub_140d88a30
    Replacement::draft(0x140d88c40, "content_140d88c40", fn_140d88c40 as *const ()), // sub_140d88c40
    Replacement::draft_throws(0x140d88e60, "content_140d88e60", fn_140d88e60 as *const (), Throws::Discard), // sub_140d88e60
    Replacement::draft_throws(0x140d890d0, "content_140d890d0", fn_140d890d0 as *const (), Throws::Discard), // sub_140d890d0
    Replacement::draft(0x140d89520, "content_140d89520", fn_140d89520 as *const ()), // sub_140d89520
    Replacement::draft(0x140d895e0, "content_140d895e0", fn_140d895e0 as *const ()), // sub_140d895e0
    Replacement::draft(0x140d89650, "content_140d89650", fn_140d89650 as *const ()), // sub_140d89650
    Replacement::draft(0x140d89710, "content_140d89710", fn_140d89710 as *const ()), // sub_140d89710
    Replacement::draft(0x140d897e0, "content_140d897e0", tpl_7312::<0x14387fb40, 0x14243b0b0, 0x140d8fac0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d897e0
    Replacement::draft(0x140d89850, "content_140d89850", fn_140d89850 as *const ()), // sub_140d89850
    Replacement::draft_throws(0x140d89990, "content_140d89990", fn_140d89990 as *const (), Throws::Discard), // sub_140d89990
    Replacement::draft(0x140d89ab0, "content_140d89ab0", fn_140d89ab0 as *const ()), // sub_140d89ab0
    Replacement::draft_throws(0x140d89b60, "content_140d89b60", fn_140d89b60 as *const (), Throws::Discard), // sub_140d89b60
    Replacement::draft(0x140d89c80, "content_140d89c80", fn_140d89c80 as *const ()), // sub_140d89c80
    Replacement::draft_throws(0x140d89d00, "content_140d89d00", fn_140d89d00 as *const (), Throws::Caught), // sub_140d89d00
    Replacement::draft_throws(0x140d8a110, "content_140d8a110", fn_140d8a110 as *const (), Throws::Caught), // sub_140d8a110
    Replacement::draft_throws(0x140d8a4a0, "content_140d8a4a0", fn_140d8a4a0 as *const (), Throws::Caught), // sub_140d8a4a0
    Replacement::draft(0x140d8a820, "content_140d8a820", fn_140d8a820 as *const ()), // sub_140d8a820
    Replacement::draft(0x140d8a8f0, "content_140d8a8f0", fn_140d8a8f0 as *const ()), // sub_140d8a8f0
    Replacement::draft(0x140d8a9a0, "content_140d8a9a0", fn_140d8a9a0 as *const ()), // sub_140d8a9a0
    Replacement::draft_throws(0x140d8aa60, "content_140d8aa60", fn_140d8aa60 as *const (), Throws::Discard), // sub_140d8aa60
    Replacement::draft_throws(0x140d8ab10, "content_140d8ab10", fn_140d8ab10 as *const (), Throws::Discard), // sub_140d8ab10
    Replacement::draft(0x140d8ac40, "content_140d8ac40", fn_140d8ac40 as *const ()), // sub_140d8ac40
    Replacement::draft(0x140d8aca0, "content_140d8aca0", fn_140d8aca0 as *const ()), // sub_140d8aca0
    Replacement::draft(0x140d8ad30, "content_140d8ad30", fn_140d8ad30 as *const ()), // sub_140d8ad30
    Replacement::draft(0x140d8ade0, "content_140d8ade0", fn_140d8ade0 as *const ()), // sub_140d8ade0
    Replacement::draft(0x140d8ae90, "content_140d8ae90", fn_140d8ae90 as *const ()), // sub_140d8ae90
    Replacement::draft(0x140d8b080, "content_140d8b080", fn_140d8b080 as *const ()), // sub_140d8b080
    Replacement::draft_throws(0x140d8b6e0, "content_140d8b6e0", fn_140d8b6e0 as *const (), Throws::Discard), // sub_140d8b6e0
    Replacement::draft(0x140d8bcd0, "content_140d8bcd0", fn_140d8bcd0 as *const ()), // sub_140d8bcd0
    Replacement::draft_throws(0x140d8c030, "content_140d8c030", fn_140d8c030 as *const (), Throws::Discard), // sub_140d8c030
    Replacement::draft(0x140d8c720, "content_140d8c720", fn_140d8c720 as *const ()), // sub_140d8c720
    Replacement::draft_throws(0x140d8c9d0, "content_140d8c9d0", fn_140d8c9d0 as *const (), Throws::Caught), // sub_140d8c9d0
    Replacement::draft_throws(0x140d8d100, "content_140d8d100", fn_140d8d100 as *const (), Throws::Caught), // sub_140d8d100
    Replacement::draft_throws(0x140d8da90, "content_140d8da90", fn_140d8da90 as *const (), Throws::Caught), // sub_140d8da90
    Replacement::draft(0x140d8e3d0, "content_140d8e3d0", fn_140d8e3d0 as *const ()), // sub_140d8e3d0
    Replacement::draft(0x140d8e820, "content_140d8e820", fn_140d8e820 as *const ()), // sub_140d8e820
    Replacement::draft(0x140d8eba0, "content_140d8eba0", fn_140d8eba0 as *const ()), // sub_140d8eba0
    Replacement::draft_throws(0x140d8ef20, "content_140d8ef20", fn_140d8ef20 as *const (), Throws::Discard), // sub_140d8ef20
    Replacement::draft_throws(0x140d8f310, "content_140d8f310", fn_140d8f310 as *const (), Throws::Discard), // sub_140d8f310
    Replacement::draft(0x140d8f820, "content_140d8f820", fn_140d8f820 as *const ()), // sub_140d8f820
    Replacement::draft(0x140d8f9b0, "content_140d8f9b0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432f4d68, 0x8> as *const ()), // sub_140d8f9b0
    Replacement::draft(0x140d8fa30, "content_140d8fa30", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f4d68> as *const ()), // sub_140d8fa30
    Replacement::draft(0x140d8fac0, "content_140d8fac0", fn_140d8fac0 as *const ()), // sub_140d8fac0
    Replacement::draft(0x140d8fb60, "content_140d8fb60", fn_140d8fb60 as *const ()), // sub_140d8fb60
    Replacement::draft_throws(0x140d8fc40, "content_140d8fc40", fn_140d8fc40 as *const (), Throws::Discard), // sub_140d8fc40
    Replacement::draft(0x140d8fd20, "content_140d8fd20", tpl_16988::<0x14387f8d0, 0x14243b0b0, 0x14387e740, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x80, 0x9> as *const ()), // sub_140d8fd20
    Replacement::draft_throws(0x140d8fdf0, "content_140d8fdf0", fn_140d8fdf0 as *const (), Throws::Discard), // sub_140d8fdf0
    Replacement::draft_throws(0x140d8fee0, "content_140d8fee0", fn_140d8fee0 as *const (), Throws::Caught), // sub_140d8fee0
    Replacement::draft_throws(0x140d8ffe0, "content_140d8ffe0", fn_140d8ffe0 as *const (), Throws::Caught), // sub_140d8ffe0
    Replacement::draft_throws(0x140d900e0, "content_140d900e0", fn_140d900e0 as *const (), Throws::Caught), // sub_140d900e0
    Replacement::draft(0x140d901e0, "content_140d901e0", fn_140d901e0 as *const ()), // sub_140d901e0
    Replacement::draft(0x140d902a0, "content_140d902a0", fn_140d902a0 as *const ()), // sub_140d902a0
    Replacement::draft(0x140d90350, "content_140d90350", tpl_16988::<0x14387f8d0, 0x14243b0b0, 0x14387e740, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x120, 0x9> as *const ()), // sub_140d90350
    Replacement::draft_throws(0x140d90420, "content_140d90420", fn_140d90420 as *const (), Throws::Discard), // sub_140d90420
    Replacement::draft_throws(0x140d904e0, "content_140d904e0", fn_140d904e0 as *const (), Throws::Discard), // sub_140d904e0
    Replacement::draft(0x140d90600, "content_140d90600", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432ef7f8> as *const ()), // sub_140d90600
    Replacement::draft(0x140d90690, "content_140d90690", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ef7f8, 0x18> as *const ()), // sub_140d90690
    Replacement::draft(0x140d90740, "content_140d90740", tpl_11859::<0x14387fa08, 0x14243b0b0, 0x140d8fac0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d90740
    Replacement::draft(0x140d907b0, "content_140d907b0", tpl_16748::<0x14387fa08, 0x14243b0b0, 0x140d8fb60, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d907b0
    Replacement::draft_throws(0x140d90860, "content_140d90860", tpl_16749::<0x14387fab4, 0x14387fa08, 0x1401e9fa0, 0x140d8fc40, 0x328, 0x338, 0x10, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140d90860
    Replacement::draft(0x140d908f0, "content_140d908f0", tpl_16750::<0x14387fa08, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d908f0
    Replacement::draft_throws(0x140d90970, "content_140d90970", tpl_16751::<0x14387fab4, 0x14387fa08, 0x1401e9b60, 0x140d8fdf0, 0x328, 0x338, 0x120, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140d90970
    Replacement::draft_throws(0x140d90a00, "content_140d90a00", fn_140d90a00 as *const (), Throws::Caught), // sub_140d90a00
    Replacement::draft_throws(0x140d90ad0, "content_140d90ad0", fn_140d90ad0 as *const (), Throws::Caught), // sub_140d90ad0
    Replacement::draft_throws(0x140d90ba0, "content_140d90ba0", fn_140d90ba0 as *const (), Throws::Caught), // sub_140d90ba0
    Replacement::draft(0x140d90c70, "content_140d90c70", tpl_16753::<0x14387fa08, 0x14243b0b0, 0x140d901e0, 0x338, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d90c70
    Replacement::draft(0x140d90cf0, "content_140d90cf0", tpl_16754::<0x14387fab4, 0x14387fa08, 0x1401e9970, 0x140d902a0, 0x338, 0x10, 0xe0, 0x1401e9860> as *const ()), // sub_140d90cf0
    Replacement::draft(0x140d90d60, "content_140d90d60", tpl_16750::<0x14387fa08, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d90d60
    Replacement::draft_throws(0x140d90de0, "content_140d90de0", tpl_16755::<0x14387fab4, 0x14387fa08, 0x1401e9770, 0x140d90420, 0x338, 0x10, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140d90de0
    Replacement::draft_throws(0x140d90e60, "content_140d90e60", fn_140d90e60 as *const (), Throws::Discard), // sub_140d90e60
    Replacement::draft(0x140d90f30, "content_140d90f30", fn_140d90f30 as *const ()), // sub_140d90f30
    Replacement::draft(0x140d90fd0, "content_140d90fd0", fn_140d90fd0 as *const ()), // sub_140d90fd0
    Replacement::draft(0x140d910f0, "content_140d910f0", fn_140d910f0 as *const ()), // sub_140d910f0
    Replacement::draft_throws(0x140d912f0, "content_140d912f0", fn_140d912f0 as *const (), Throws::Discard), // sub_140d912f0
    Replacement::draft(0x140d914f0, "content_140d914f0", fn_140d914f0 as *const ()), // sub_140d914f0
    Replacement::draft_throws(0x140d91620, "content_140d91620", fn_140d91620 as *const (), Throws::Discard), // sub_140d91620
    Replacement::draft(0x140d91870, "content_140d91870", fn_140d91870 as *const ()), // sub_140d91870
    Replacement::draft_throws(0x140d91950, "content_140d91950", fn_140d91950 as *const (), Throws::Caught), // sub_140d91950
    Replacement::draft_throws(0x140d91c80, "content_140d91c80", fn_140d91c80 as *const (), Throws::Caught), // sub_140d91c80
    Replacement::draft_throws(0x140d91fb0, "content_140d91fb0", fn_140d91fb0 as *const (), Throws::Caught), // sub_140d91fb0
    Replacement::draft(0x140d922e0, "content_140d922e0", fn_140d922e0 as *const ()), // sub_140d922e0
    Replacement::draft(0x140d92410, "content_140d92410", fn_140d92410 as *const ()), // sub_140d92410
    Replacement::draft(0x140d92540, "content_140d92540", fn_140d92540 as *const ()), // sub_140d92540
    Replacement::draft_throws(0x140d92680, "content_140d92680", fn_140d92680 as *const (), Throws::Discard), // sub_140d92680
    Replacement::draft_throws(0x140d927f0, "content_140d927f0", fn_140d927f0 as *const (), Throws::Discard), // sub_140d927f0
    Replacement::draft(0x140d92a80, "content_140d92a80", fn_140d92a80 as *const ()), // sub_140d92a80
    Replacement::draft(0x140d92b40, "content_140d92b40", tpl_7173::<0x1432ec430, 0x10, 0x1, 0x1409fe2d0, 0x18, 0x1432f8998> as *const ()), // sub_140d92b40
    Replacement::draft(0x140d92bb0, "content_140d92bb0", tpl_17011::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x50, 0x1432f6038, 0x23> as *const ()), // sub_140d92bb0
    Replacement::draft(0x140d92c70, "content_140d92c70", tpl_17012::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432f6038, 0xffffffffffffffe4, 0x4, 0x24, 0x34> as *const ()), // sub_140d92c70
    Replacement::draft(0x140d92d40, "content_140d92d40", fn_140d92d40 as *const ()), // sub_140d92d40
    Replacement::draft(0x140d92e20, "content_140d92e20", fn_140d92e20 as *const ()), // sub_140d92e20
    Replacement::draft_throws(0x140d92ff0, "content_140d92ff0", fn_140d92ff0 as *const (), Throws::Caught), // sub_140d92ff0
    Replacement::draft(0x140d93170, "content_140d93170", fn_140d93170 as *const ()), // sub_140d93170
    Replacement::draft_throws(0x140d93280, "content_140d93280", fn_140d93280 as *const (), Throws::Caught), // sub_140d93280
    Replacement::draft(0x140d93420, "content_140d93420", fn_140d93420 as *const ()), // sub_140d93420
    Replacement::draft_throws(0x140d93520, "content_140d93520", fn_140d93520 as *const (), Throws::Caught), // sub_140d93520
    Replacement::draft_throws(0x140d93830, "content_140d93830", fn_140d93830 as *const (), Throws::Caught), // sub_140d93830
    Replacement::draft_throws(0x140d93b00, "content_140d93b00", fn_140d93b00 as *const (), Throws::Caught), // sub_140d93b00
    Replacement::draft(0x140d93dc0, "content_140d93dc0", fn_140d93dc0 as *const ()), // sub_140d93dc0
    Replacement::draft(0x140d93f00, "content_140d93f00", fn_140d93f00 as *const ()), // sub_140d93f00
    Replacement::draft(0x140d94010, "content_140d94010", fn_140d94010 as *const ()), // sub_140d94010
    Replacement::draft_throws(0x140d94130, "content_140d94130", fn_140d94130 as *const (), Throws::Discard), // sub_140d94130
    Replacement::draft_throws(0x140d94240, "content_140d94240", fn_140d94240 as *const (), Throws::Discard), // sub_140d94240
    Replacement::draft(0x140d943a0, "content_140d943a0", tpl_16820::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432ed788, 0x20, 0x28> as *const ()), // sub_140d943a0
    Replacement::draft(0x140d94440, "content_140d94440", tpl_16821::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x1432ed788, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0xffffffffffffffe0> as *const ()), // sub_140d94440
    Replacement::draft(0x140d944f0, "content_140d944f0", tpl_16822::<0x14387f770, 0x14243b0b0, 0x140d8fac0, 0x20, 0x28, 0x14387f8e8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140d944f0
    Replacement::draft(0x140d945d0, "content_140d945d0", tpl_16823::<0x14387f770, 0x14243b0b0, 0x140d8fb60, 0x1426d4580, 0x140db6260> as *const ()), // sub_140d945d0
    Replacement::draft_throws(0x140d94760, "content_140d94760", fn_140d94760 as *const (), Throws::Caught), // sub_140d94760
    Replacement::draft(0x140d948a0, "content_140d948a0", tpl_16825::<0x14387f770, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x14, 0x18, 0x48, 0x20, 0x140dfbc80, 0x140db64e0> as *const ()), // sub_140d948a0
    Replacement::draft_throws(0x140d949a0, "content_140d949a0", fn_140d949a0 as *const (), Throws::Caught), // sub_140d949a0
    Replacement::draft_throws(0x140d94af0, "content_140d94af0", fn_140d94af0 as *const (), Throws::Caught), // sub_140d94af0
    Replacement::draft_throws(0x140d94c50, "content_140d94c50", fn_140d94c50 as *const (), Throws::Caught), // sub_140d94c50
    Replacement::draft_throws(0x140d94db0, "content_140d94db0", fn_140d94db0 as *const (), Throws::Caught), // sub_140d94db0
    Replacement::draft(0x140d94f10, "content_140d94f10", tpl_75182::<0x14387f770, 0x14243b0b0, 0x140d901e0, 0x140db69c0> as *const ()), // sub_140d94f10
    Replacement::draft(0x140d95010, "content_140d95010", tpl_75183::<0x14387fa84, 0x14387f770, 0x1401e9970, 0x140d902a0, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x28, 0xe8, 0x20, 0x140db6a70, 0x1401e9860> as *const ()), // sub_140d95010
    Replacement::draft(0x140d95100, "content_140d95100", tpl_75184::<0x14387f770, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x28, 0xe8, 0x20, 0x140db6ae0> as *const ()), // sub_140d95100
    Replacement::draft_throws(0x140d95200, "content_140d95200", tpl_75185::<0x14387fa84, 0x14387f770, 0x1401e9770, 0x140d90420, 0x338, 0x10, 0x120, 0x14, 0x18, 0x28, 0x20, 0x140db6b60, 0x1401e9590> as *const (), Throws::Discard), // sub_140d95200
    Replacement::draft_throws(0x140d95300, "content_140d95300", tpl_16829::<0x14387f770, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140d904e0, 0x1432f84b0, 0x14020f120, 0x1432f84c0, 0x1432f8660, 0x1432f8488, 0x140dd0070> as *const (), Throws::Discard), // sub_140d95300
    Replacement::draft(0x140d95450, "content_140d95450", tpl_75186::<0x20, 0x28, 0x1432f8488, 0x140ded810> as *const ()), // sub_140d95450
    Replacement::draft(0x140d954d0, "content_140d954d0", fn_140d954d0 as *const ()), // sub_140d954d0
    Replacement::draft(0x140d95560, "content_140d95560", fn_140d95560 as *const ()), // sub_140d95560
    Replacement::draft(0x140d95610, "content_140d95610", fn_140d95610 as *const ()), // sub_140d95610
    Replacement::draft(0x140d956f0, "content_140d956f0", fn_140d956f0 as *const ()), // sub_140d956f0
    Replacement::draft_throws(0x140d95860, "content_140d95860", fn_140d95860 as *const (), Throws::Discard), // sub_140d95860
    Replacement::draft(0x140d95930, "content_140d95930", fn_140d95930 as *const ()), // sub_140d95930
    Replacement::draft_throws(0x140d95a20, "content_140d95a20", fn_140d95a20 as *const (), Throws::Discard), // sub_140d95a20
    Replacement::draft(0x140d95b00, "content_140d95b00", fn_140d95b00 as *const ()), // sub_140d95b00
    Replacement::draft_throws(0x140d95bd0, "content_140d95bd0", fn_140d95bd0 as *const (), Throws::Caught), // sub_140d95bd0
    Replacement::draft_throws(0x140d95d00, "content_140d95d00", fn_140d95d00 as *const (), Throws::Caught), // sub_140d95d00
    Replacement::draft_throws(0x140d95e30, "content_140d95e30", fn_140d95e30 as *const (), Throws::Caught), // sub_140d95e30
    Replacement::draft(0x140d95f60, "content_140d95f60", fn_140d95f60 as *const ()), // sub_140d95f60
    Replacement::draft(0x140d96050, "content_140d96050", fn_140d96050 as *const ()), // sub_140d96050
    Replacement::draft(0x140d96140, "content_140d96140", fn_140d96140 as *const ()), // sub_140d96140
    Replacement::draft_throws(0x140d96240, "content_140d96240", fn_140d96240 as *const (), Throws::Discard), // sub_140d96240
    Replacement::draft_throws(0x140d96330, "content_140d96330", fn_140d96330 as *const (), Throws::Discard), // sub_140d96330
    Replacement::draft(0x140d96450, "content_140d96450", fn_140d96450 as *const ()), // sub_140d96450
    Replacement::draft(0x140d96540, "content_140d96540", fn_140d96540 as *const ()), // sub_140d96540
    Replacement::draft(0x140d96640, "content_140d96640", fn_140d96640 as *const ()), // sub_140d96640
    Replacement::draft(0x140d96800, "content_140d96800", fn_140d96800 as *const ()), // sub_140d96800
    Replacement::draft_throws(0x140d96c10, "content_140d96c10", fn_140d96c10 as *const (), Throws::Discard), // sub_140d96c10
    Replacement::draft(0x140d96fc0, "content_140d96fc0", fn_140d96fc0 as *const ()), // sub_140d96fc0
    Replacement::draft_throws(0x140d97240, "content_140d97240", fn_140d97240 as *const (), Throws::Discard), // sub_140d97240
    Replacement::draft(0x140d97670, "content_140d97670", fn_140d97670 as *const ()), // sub_140d97670
    Replacement::draft_throws(0x140d97860, "content_140d97860", fn_140d97860 as *const (), Throws::Caught), // sub_140d97860
    Replacement::draft_throws(0x140d98050, "content_140d98050", fn_140d98050 as *const (), Throws::Caught), // sub_140d98050
    Replacement::draft_throws(0x140d98790, "content_140d98790", fn_140d98790 as *const (), Throws::Caught), // sub_140d98790
    Replacement::draft(0x140d98ef0, "content_140d98ef0", fn_140d98ef0 as *const ()), // sub_140d98ef0
    Replacement::draft(0x140d99140, "content_140d99140", fn_140d99140 as *const ()), // sub_140d99140
    Replacement::draft(0x140d993c0, "content_140d993c0", fn_140d993c0 as *const ()), // sub_140d993c0
    Replacement::draft_throws(0x140d99650, "content_140d99650", fn_140d99650 as *const (), Throws::Discard), // sub_140d99650
    Replacement::draft_throws(0x140d99910, "content_140d99910", fn_140d99910 as *const (), Throws::Discard), // sub_140d99910
    Replacement::draft(0x140d99c00, "content_140d99c00", tpl_17042::<0x1432f8ec0, 0x30, 0x143877ff8, 0x1432f8eb0, 0x1401de410, 0x143877db8, 0x34, 0x1432f8e90, 0x1432f8ea0, 0x38, 0x1438777f8, 0x1432f84d8, 0x1432f8310> as *const ()), // sub_140d99c00
    Replacement::draft(0x140d99cc0, "content_140d99cc0", fn_140d99cc0 as *const ()), // sub_140d99cc0
    Replacement::draft(0x140d99e30, "content_140d99e30", tpl_17042::<0x1432f8ec0, 0x30, 0x143877698, 0x1432f8eb0, 0x1401de410, 0x143877b48, 0x34, 0x1432f8e90, 0x1432f8ea0, 0x38, 0x143877fa8, 0x1432f84d8, 0x1432f8310> as *const ()), // sub_140d99e30
    Replacement::draft(0x140d99ef0, "content_140d99ef0", fn_140d99ef0 as *const ()), // sub_140d99ef0
    Replacement::draft(0x140d99f90, "content_140d99f90", fn_140d99f90 as *const ()), // sub_140d99f90
    Replacement::draft(0x140d9a040, "content_140d9a040", fn_140d9a040 as *const ()), // sub_140d9a040
    Replacement::draft(0x140d9a110, "content_140d9a110", fn_140d9a110 as *const ()), // sub_140d9a110
    Replacement::draft_throws(0x140d9a370, "content_140d9a370", fn_140d9a370 as *const (), Throws::Discard), // sub_140d9a370
    Replacement::draft(0x140d9a530, "content_140d9a530", fn_140d9a530 as *const ()), // sub_140d9a530
    Replacement::draft_throws(0x140d9a690, "content_140d9a690", fn_140d9a690 as *const (), Throws::Discard), // sub_140d9a690
    Replacement::draft(0x140d9a890, "content_140d9a890", fn_140d9a890 as *const ()), // sub_140d9a890
    Replacement::draft_throws(0x140d9a9c0, "content_140d9a9c0", fn_140d9a9c0 as *const (), Throws::Caught), // sub_140d9a9c0
    Replacement::draft_throws(0x140d9aea0, "content_140d9aea0", fn_140d9aea0 as *const (), Throws::Caught), // sub_140d9aea0
    Replacement::draft_throws(0x140d9b2e0, "content_140d9b2e0", fn_140d9b2e0 as *const (), Throws::Caught), // sub_140d9b2e0
    Replacement::draft(0x140d9b710, "content_140d9b710", fn_140d9b710 as *const ()), // sub_140d9b710
    Replacement::draft(0x140d9b850, "content_140d9b850", fn_140d9b850 as *const ()), // sub_140d9b850
    Replacement::draft(0x140d9b9c0, "content_140d9b9c0", fn_140d9b9c0 as *const ()), // sub_140d9b9c0
    Replacement::draft_throws(0x140d9bb40, "content_140d9bb40", fn_140d9bb40 as *const (), Throws::Discard), // sub_140d9bb40
    Replacement::draft_throws(0x140d9bcd0, "content_140d9bcd0", fn_140d9bcd0 as *const (), Throws::Discard), // sub_140d9bcd0
    Replacement::draft(0x140d9bf10, "content_140d9bf10", fn_140d9bf10 as *const ()), // sub_140d9bf10
    Replacement::draft(0x140d9bfd0, "content_140d9bfd0", tpl_8792::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432f7368, 0x20, 0x28> as *const ()), // sub_140d9bfd0
    Replacement::draft(0x140d9c070, "content_140d9c070", fn_140d9c070 as *const ()), // sub_140d9c070
    Replacement::draft(0x140d9c120, "content_140d9c120", fn_140d9c120 as *const ()), // sub_140d9c120
    Replacement::draft(0x140d9c1f0, "content_140d9c1f0", fn_140d9c1f0 as *const ()), // sub_140d9c1f0
    Replacement::draft_throws(0x140d9c390, "content_140d9c390", fn_140d9c390 as *const (), Throws::Discard), // sub_140d9c390
    Replacement::draft(0x140d9c4f0, "content_140d9c4f0", fn_140d9c4f0 as *const ()), // sub_140d9c4f0
    Replacement::draft_throws(0x140d9c610, "content_140d9c610", fn_140d9c610 as *const (), Throws::Discard), // sub_140d9c610
    Replacement::draft(0x140d9c7c0, "content_140d9c7c0", fn_140d9c7c0 as *const ()), // sub_140d9c7c0
    Replacement::draft_throws(0x140d9c880, "content_140d9c880", fn_140d9c880 as *const (), Throws::Caught), // sub_140d9c880
    Replacement::draft_throws(0x140d9ca10, "content_140d9ca10", fn_140d9ca10 as *const (), Throws::Caught), // sub_140d9ca10
    Replacement::draft_throws(0x140d9cba0, "content_140d9cba0", fn_140d9cba0 as *const (), Throws::Caught), // sub_140d9cba0
    Replacement::draft(0x140d9cd30, "content_140d9cd30", fn_140d9cd30 as *const ()), // sub_140d9cd30
    Replacement::draft(0x140d9ce40, "content_140d9ce40", fn_140d9ce40 as *const ()), // sub_140d9ce40
    Replacement::draft(0x140d9cf80, "content_140d9cf80", fn_140d9cf80 as *const ()), // sub_140d9cf80
    Replacement::draft_throws(0x140d9d0a0, "content_140d9d0a0", fn_140d9d0a0 as *const (), Throws::Discard), // sub_140d9d0a0
    Replacement::draft_throws(0x140d9d1c0, "content_140d9d1c0", fn_140d9d1c0 as *const (), Throws::Discard), // sub_140d9d1c0
    Replacement::draft(0x140d9d340, "content_140d9d340", fn_140d9d340 as *const ()), // sub_140d9d340
    Replacement::draft(0x140d9d3d0, "content_140d9d3d0", fn_140d9d3d0 as *const ()), // sub_140d9d3d0
    Replacement::draft(0x140d9d4f0, "content_140d9d4f0", fn_140d9d4f0 as *const ()), // sub_140d9d4f0
    Replacement::draft_throws(0x140d9d5f0, "content_140d9d5f0", fn_140d9d5f0 as *const (), Throws::Discard), // sub_140d9d5f0
    Replacement::draft(0x140d9d6e0, "content_140d9d6e0", tpl_17066::<0x14387fad8, 0x14243b0b0, 0x140d8fd20, 0x10, 0x1408ff6f0, 0x338, 0x18, 0x48, 0x1c, 0x80, 0x20, 0x40> as *const ()), // sub_140d9d6e0
    Replacement::draft_throws(0x140d9d790, "content_140d9d790", fn_140d9d790 as *const (), Throws::Discard), // sub_140d9d790
    Replacement::draft_throws(0x140d9d890, "content_140d9d890", fn_140d9d890 as *const (), Throws::Caught), // sub_140d9d890
    Replacement::draft_throws(0x140d9da00, "content_140d9da00", fn_140d9da00 as *const (), Throws::Caught), // sub_140d9da00
    Replacement::draft_throws(0x140d9db70, "content_140d9db70", fn_140d9db70 as *const (), Throws::Caught), // sub_140d9db70
    Replacement::draft(0x140d9dce0, "content_140d9dce0", fn_140d9dce0 as *const ()), // sub_140d9dce0
    Replacement::draft(0x140d9dda0, "content_140d9dda0", fn_140d9dda0 as *const ()), // sub_140d9dda0
    Replacement::draft(0x140d9de40, "content_140d9de40", tpl_17066::<0x14387fad8, 0x14243b0b0, 0x140d90350, 0x10, 0x1409002a0, 0x338, 0x18, 0xe8, 0x1c, 0x120, 0x20, 0xe0> as *const ()), // sub_140d9de40
    Replacement::draft_throws(0x140d9def0, "content_140d9def0", fn_140d9def0 as *const (), Throws::Discard), // sub_140d9def0
    Replacement::draft_throws(0x140d9dfa0, "content_140d9dfa0", fn_140d9dfa0 as *const (), Throws::Discard), // sub_140d9dfa0
    Replacement::draft(0x140d9e0d0, "content_140d9e0d0", tpl_8792::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432ec828, 0x20, 0x28> as *const ()), // sub_140d9e0d0
    Replacement::draft(0x140d9e170, "content_140d9e170", fn_140d9e170 as *const ()), // sub_140d9e170
    Replacement::draft(0x140d9e280, "content_140d9e280", fn_140d9e280 as *const ()), // sub_140d9e280
    Replacement::draft_throws(0x140d9e3e0, "content_140d9e3e0", fn_140d9e3e0 as *const (), Throws::Discard), // sub_140d9e3e0
    Replacement::draft(0x140d9e540, "content_140d9e540", fn_140d9e540 as *const ()), // sub_140d9e540
    Replacement::draft_throws(0x140d9e620, "content_140d9e620", fn_140d9e620 as *const (), Throws::Discard), // sub_140d9e620
    Replacement::draft_throws(0x140d9e7a0, "content_140d9e7a0", fn_140d9e7a0 as *const (), Throws::Caught), // sub_140d9e7a0
    Replacement::draft_throws(0x140d9e9e0, "content_140d9e9e0", fn_140d9e9e0 as *const (), Throws::Caught), // sub_140d9e9e0
    Replacement::draft_throws(0x140d9ec20, "content_140d9ec20", fn_140d9ec20 as *const (), Throws::Caught), // sub_140d9ec20
    Replacement::draft(0x140d9ee60, "content_140d9ee60", fn_140d9ee60 as *const ()), // sub_140d9ee60
    Replacement::draft(0x140d9ef50, "content_140d9ef50", fn_140d9ef50 as *const ()), // sub_140d9ef50
    Replacement::draft(0x140d9f030, "content_140d9f030", fn_140d9f030 as *const ()), // sub_140d9f030
    Replacement::draft_throws(0x140d9f120, "content_140d9f120", fn_140d9f120 as *const (), Throws::Discard), // sub_140d9f120
    Replacement::draft_throws(0x140d9f220, "content_140d9f220", fn_140d9f220 as *const (), Throws::Discard), // sub_140d9f220
    Replacement::draft(0x140d9f3e0, "content_140d9f3e0", fn_140d9f3e0 as *const ()), // sub_140d9f3e0
    Replacement::draft(0x140d9f480, "content_140d9f480", fn_140d9f480 as *const ()), // sub_140d9f480
    Replacement::draft(0x140d9f5a0, "content_140d9f5a0", fn_140d9f5a0 as *const ()), // sub_140d9f5a0
    Replacement::draft_throws(0x140d9f740, "content_140d9f740", fn_140d9f740 as *const (), Throws::Discard), // sub_140d9f740
    Replacement::draft(0x140d9f900, "content_140d9f900", fn_140d9f900 as *const ()), // sub_140d9f900
    Replacement::draft_throws(0x140d9fa30, "content_140d9fa30", fn_140d9fa30 as *const (), Throws::Discard), // sub_140d9fa30
    Replacement::draft(0x140d9fbe0, "content_140d9fbe0", fn_140d9fbe0 as *const ()), // sub_140d9fbe0
    Replacement::draft_throws(0x140d9fc90, "content_140d9fc90", fn_140d9fc90 as *const (), Throws::Caught), // sub_140d9fc90
    Replacement::draft_throws(0x140da0100, "content_140da0100", fn_140da0100 as *const (), Throws::Caught), // sub_140da0100
    Replacement::draft_throws(0x140da04e0, "content_140da04e0", fn_140da04e0 as *const (), Throws::Caught), // sub_140da04e0
    Replacement::draft(0x140da08b0, "content_140da08b0", fn_140da08b0 as *const ()), // sub_140da08b0
    Replacement::draft(0x140da09a0, "content_140da09a0", fn_140da09a0 as *const ()), // sub_140da09a0
    Replacement::draft(0x140da0ae0, "content_140da0ae0", fn_140da0ae0 as *const ()), // sub_140da0ae0
    Replacement::draft_throws(0x140da0c30, "content_140da0c30", fn_140da0c30 as *const (), Throws::Discard), // sub_140da0c30
    Replacement::draft_throws(0x140da0d50, "content_140da0d50", fn_140da0d50 as *const (), Throws::Discard), // sub_140da0d50
    Replacement::draft_throws(0x140da0f60, "content_140da0f60", fn_140da0f60 as *const (), Throws::Discard), // sub_140da0f60
    Replacement::draft(0x140da1020, "content_140da1020", tpl_11529::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432f3f18> as *const ()), // sub_140da1020
    Replacement::draft(0x140da10b0, "content_140da10b0", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432f3f18, 0x18, 0x20> as *const ()), // sub_140da10b0
    Replacement::draft(0x140da11c0, "content_140da11c0", fn_140da11c0 as *const ()), // sub_140da11c0
    Replacement::draft_throws(0x140da12d0, "content_140da12d0", fn_140da12d0 as *const (), Throws::Discard), // sub_140da12d0
    Replacement::draft(0x140da13c0, "content_140da13c0", tpl_17100::<0x14387f8a8, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x40, 0x14, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da13c0
    Replacement::draft_throws(0x140da1460, "content_140da1460", fn_140da1460 as *const (), Throws::Discard), // sub_140da1460
    Replacement::draft_throws(0x140da1560, "content_140da1560", fn_140da1560 as *const (), Throws::Caught), // sub_140da1560
    Replacement::draft_throws(0x140da16a0, "content_140da16a0", fn_140da16a0 as *const (), Throws::Caught), // sub_140da16a0
    Replacement::draft_throws(0x140da17e0, "content_140da17e0", fn_140da17e0 as *const (), Throws::Caught), // sub_140da17e0
    Replacement::draft(0x140da1920, "content_140da1920", fn_140da1920 as *const ()), // sub_140da1920
    Replacement::draft(0x140da19c0, "content_140da19c0", fn_140da19c0 as *const ()), // sub_140da19c0
    Replacement::draft(0x140da1a50, "content_140da1a50", tpl_17100::<0x14387f8a8, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xe0, 0x14, 0x18, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da1a50
    Replacement::draft_throws(0x140da1af0, "content_140da1af0", fn_140da1af0 as *const (), Throws::Discard), // sub_140da1af0
    Replacement::draft_throws(0x140da1ba0, "content_140da1ba0", fn_140da1ba0 as *const (), Throws::Discard), // sub_140da1ba0
    Replacement::draft(0x140da1cc0, "content_140da1cc0", fn_140da1cc0 as *const ()), // sub_140da1cc0
    Replacement::draft(0x140da1da0, "content_140da1da0", fn_140da1da0 as *const ()), // sub_140da1da0
    Replacement::draft(0x140da1f30, "content_140da1f30", fn_140da1f30 as *const ()), // sub_140da1f30
    Replacement::draft_throws(0x140da21f0, "content_140da21f0", fn_140da21f0 as *const (), Throws::Discard), // sub_140da21f0
    Replacement::draft(0x140da2450, "content_140da2450", fn_140da2450 as *const ()), // sub_140da2450
    Replacement::draft_throws(0x140da25b0, "content_140da25b0", fn_140da25b0 as *const (), Throws::Discard), // sub_140da25b0
    Replacement::draft(0x140da2860, "content_140da2860", fn_140da2860 as *const ()), // sub_140da2860
    Replacement::draft_throws(0x140da2960, "content_140da2960", fn_140da2960 as *const (), Throws::Caught), // sub_140da2960
    Replacement::draft_throws(0x140da2d30, "content_140da2d30", fn_140da2d30 as *const (), Throws::Caught), // sub_140da2d30
    Replacement::draft_throws(0x140da30d0, "content_140da30d0", fn_140da30d0 as *const (), Throws::Caught), // sub_140da30d0
    Replacement::draft(0x140da3460, "content_140da3460", fn_140da3460 as *const ()), // sub_140da3460
    Replacement::draft(0x140da35e0, "content_140da35e0", fn_140da35e0 as *const ()), // sub_140da35e0
    Replacement::draft(0x140da3740, "content_140da3740", fn_140da3740 as *const ()), // sub_140da3740
    Replacement::draft_throws(0x140da38c0, "content_140da38c0", fn_140da38c0 as *const (), Throws::Discard), // sub_140da38c0
    Replacement::draft_throws(0x140da3a30, "content_140da3a30", fn_140da3a30 as *const (), Throws::Discard), // sub_140da3a30
    Replacement::draft_throws(0x140da3cf0, "content_140da3cf0", fn_140da3cf0 as *const (), Throws::Discard), // sub_140da3cf0
    Replacement::draft(0x140da3e00, "content_140da3e00", tpl_17011::<0x60, 0x10, 0x18, 0xffffffffffffffb8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x20, 0x40, 0x50, 0x1432ef158, 0x23> as *const ()), // sub_140da3e00
    Replacement::draft(0x140da3ec0, "content_140da3ec0", tpl_17012::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432ef158, 0xffffffffffffffe4, 0x4, 0x24, 0x34> as *const ()), // sub_140da3ec0
    Replacement::draft(0x140da3f90, "content_140da3f90", fn_140da3f90 as *const ()), // sub_140da3f90
    Replacement::draft(0x140da4070, "content_140da4070", fn_140da4070 as *const ()), // sub_140da4070
    Replacement::draft_throws(0x140da4250, "content_140da4250", fn_140da4250 as *const (), Throws::Discard), // sub_140da4250
    Replacement::draft(0x140da43c0, "content_140da43c0", fn_140da43c0 as *const ()), // sub_140da43c0
    Replacement::draft_throws(0x140da44e0, "content_140da44e0", fn_140da44e0 as *const (), Throws::Discard), // sub_140da44e0
    Replacement::draft(0x140da46a0, "content_140da46a0", fn_140da46a0 as *const ()), // sub_140da46a0
    Replacement::draft_throws(0x140da4790, "content_140da4790", fn_140da4790 as *const (), Throws::Caught), // sub_140da4790
    Replacement::draft_throws(0x140da4920, "content_140da4920", fn_140da4920 as *const (), Throws::Caught), // sub_140da4920
    Replacement::draft_throws(0x140da4ab0, "content_140da4ab0", fn_140da4ab0 as *const (), Throws::Caught), // sub_140da4ab0
    Replacement::draft(0x140da4c40, "content_140da4c40", fn_140da4c40 as *const ()), // sub_140da4c40
    Replacement::draft(0x140da4d60, "content_140da4d60", fn_140da4d60 as *const ()), // sub_140da4d60
    Replacement::draft(0x140da4e80, "content_140da4e80", fn_140da4e80 as *const ()), // sub_140da4e80
    Replacement::draft_throws(0x140da4fb0, "content_140da4fb0", fn_140da4fb0 as *const (), Throws::Discard), // sub_140da4fb0
    Replacement::draft_throws(0x140da50f0, "content_140da50f0", fn_140da50f0 as *const (), Throws::Discard), // sub_140da50f0
    Replacement::draft(0x140da5270, "content_140da5270", fn_140da5270 as *const ()), // sub_140da5270
    Replacement::draft(0x140da5300, "content_140da5300", fn_140da5300 as *const ()), // sub_140da5300
    Replacement::draft(0x140da53d0, "content_140da53d0", fn_140da53d0 as *const ()), // sub_140da53d0
    Replacement::draft(0x140da54d0, "content_140da54d0", fn_140da54d0 as *const ()), // sub_140da54d0
    Replacement::draft(0x140da5540, "content_140da5540", fn_140da5540 as *const ()), // sub_140da5540
    Replacement::draft_throws(0x140da5630, "content_140da5630", fn_140da5630 as *const (), Throws::Discard), // sub_140da5630
    Replacement::draft(0x140da5720, "content_140da5720", tpl_17130::<0x14387f980, 0x14243b0b0, 0x140d8fd20, 0x338, 0x10, 0x20, 0x98, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da5720
    Replacement::draft_throws(0x140da57b0, "content_140da57b0", fn_140da57b0 as *const (), Throws::Discard), // sub_140da57b0
    Replacement::draft_throws(0x140da58b0, "content_140da58b0", fn_140da58b0 as *const (), Throws::Caught), // sub_140da58b0
    Replacement::draft_throws(0x140da5b50, "content_140da5b50", fn_140da5b50 as *const (), Throws::Caught), // sub_140da5b50
    Replacement::draft_throws(0x140da5dc0, "content_140da5dc0", fn_140da5dc0 as *const (), Throws::Caught), // sub_140da5dc0
    Replacement::draft(0x140da6020, "content_140da6020", fn_140da6020 as *const ()), // sub_140da6020
    Replacement::draft(0x140da60e0, "content_140da60e0", fn_140da60e0 as *const ()), // sub_140da60e0
    Replacement::draft(0x140da6170, "content_140da6170", tpl_17130::<0x14387f980, 0x14243b0b0, 0x140d90350, 0x338, 0x10, 0xc0, 0x98, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da6170
    Replacement::draft_throws(0x140da6210, "content_140da6210", fn_140da6210 as *const (), Throws::Discard), // sub_140da6210
    Replacement::draft_throws(0x140da62a0, "content_140da62a0", fn_140da62a0 as *const (), Throws::Discard), // sub_140da62a0
    Replacement::draft(0x140da63a0, "content_140da63a0", fn_140da63a0 as *const ()), // sub_140da63a0
    Replacement::draft(0x140da6470, "content_140da6470", fn_140da6470 as *const ()), // sub_140da6470
    Replacement::draft(0x140da6560, "content_140da6560", tpl_11830::<0x14387f768, 0x14243b0b0, 0x140d8fac0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da6560
    Replacement::draft(0x140da65d0, "content_140da65d0", fn_140da65d0 as *const ()), // sub_140da65d0
    Replacement::draft_throws(0x140da6750, "content_140da6750", fn_140da6750 as *const (), Throws::Discard), // sub_140da6750
    Replacement::draft(0x140da68b0, "content_140da68b0", fn_140da68b0 as *const ()), // sub_140da68b0
    Replacement::draft_throws(0x140da6960, "content_140da6960", fn_140da6960 as *const (), Throws::Discard), // sub_140da6960
    Replacement::draft(0x140da6ad0, "content_140da6ad0", fn_140da6ad0 as *const ()), // sub_140da6ad0
    Replacement::draft_throws(0x140da6b50, "content_140da6b50", fn_140da6b50 as *const (), Throws::Caught), // sub_140da6b50
    Replacement::draft_throws(0x140da70c0, "content_140da70c0", fn_140da70c0 as *const (), Throws::Caught), // sub_140da70c0
    Replacement::draft_throws(0x140da7580, "content_140da7580", fn_140da7580 as *const (), Throws::Caught), // sub_140da7580
    Replacement::draft(0x140da7a20, "content_140da7a20", fn_140da7a20 as *const ()), // sub_140da7a20
    Replacement::draft(0x140da7b20, "content_140da7b20", fn_140da7b20 as *const ()), // sub_140da7b20
    Replacement::draft(0x140da7be0, "content_140da7be0", fn_140da7be0 as *const ()), // sub_140da7be0
    Replacement::draft_throws(0x140da7cb0, "content_140da7cb0", fn_140da7cb0 as *const (), Throws::Discard), // sub_140da7cb0
    Replacement::draft_throws(0x140da7d60, "content_140da7d60", fn_140da7d60 as *const (), Throws::Discard), // sub_140da7d60
    Replacement::draft(0x140da7eb0, "content_140da7eb0", fn_140da7eb0 as *const ()), // sub_140da7eb0
    Replacement::draft(0x140da7f60, "content_140da7f60", fn_140da7f60 as *const ()), // sub_140da7f60
    Replacement::draft(0x140da8020, "content_140da8020", fn_140da8020 as *const ()), // sub_140da8020
    Replacement::draft_throws(0x140da8100, "content_140da8100", fn_140da8100 as *const (), Throws::Discard), // sub_140da8100
    Replacement::draft(0x140da81f0, "content_140da81f0", fn_140da81f0 as *const ()), // sub_140da81f0
    Replacement::draft_throws(0x140da82f0, "content_140da82f0", fn_140da82f0 as *const (), Throws::Discard), // sub_140da82f0
    Replacement::draft(0x140da83e0, "content_140da83e0", fn_140da83e0 as *const ()), // sub_140da83e0
    Replacement::draft_throws(0x140da8490, "content_140da8490", fn_140da8490 as *const (), Throws::Caught), // sub_140da8490
    Replacement::draft_throws(0x140da8730, "content_140da8730", fn_140da8730 as *const (), Throws::Caught), // sub_140da8730
    Replacement::draft_throws(0x140da89a0, "content_140da89a0", fn_140da89a0 as *const (), Throws::Caught), // sub_140da89a0
    Replacement::draft(0x140da8c00, "content_140da8c00", fn_140da8c00 as *const ()), // sub_140da8c00
    Replacement::draft(0x140da8cb0, "content_140da8cb0", fn_140da8cb0 as *const ()), // sub_140da8cb0
    Replacement::draft(0x140da8d40, "content_140da8d40", fn_140da8d40 as *const ()), // sub_140da8d40
    Replacement::draft_throws(0x140da8dd0, "content_140da8dd0", fn_140da8dd0 as *const (), Throws::Discard), // sub_140da8dd0
    Replacement::draft_throws(0x140da8eb0, "content_140da8eb0", fn_140da8eb0 as *const (), Throws::Discard), // sub_140da8eb0
    Replacement::draft(0x140da8fb0, "content_140da8fb0", tpl_17166::<0x14387fa90, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da8fb0
    Replacement::draft_throws(0x140da9080, "content_140da9080", tpl_17167::<0x14387f8c4, 0x14387fa90, 0x1401e9fa0, 0x328, 0x338, 0x40, 0x369, 0x4, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140da9080
    Replacement::draft(0x140da9130, "content_140da9130", tpl_2810::<0x14387fa90, 0x14243b0b0, 0x338, 0x40, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da9130
    Replacement::draft_throws(0x140da91b0, "content_140da91b0", tpl_17168::<0x14387f8c4, 0x14387fa90, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_140da91b0
    Replacement::draft_throws(0x140da9270, "content_140da9270", fn_140da9270 as *const (), Throws::Caught), // sub_140da9270
    Replacement::draft_throws(0x140da9370, "content_140da9370", fn_140da9370 as *const (), Throws::Caught), // sub_140da9370
    Replacement::draft_throws(0x140da9470, "content_140da9470", fn_140da9470 as *const (), Throws::Caught), // sub_140da9470
    Replacement::draft(0x140da9570, "content_140da9570", tpl_17170::<0x14387fa90, 0x14243b0b0, 0x338, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da9570
    Replacement::draft(0x140da95f0, "content_140da95f0", tpl_2812::<0x14387f8c4, 0x14387fa90, 0x1401e9970, 0x338, 0xe0, 0x4, 0xd8, 0x1401e9860> as *const ()), // sub_140da95f0
    Replacement::draft(0x140da9670, "content_140da9670", tpl_2810::<0x14387fa90, 0x14243b0b0, 0x338, 0xe0, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da9670
    Replacement::draft_throws(0x140da9700, "content_140da9700", tpl_17171::<0x14387f8c4, 0x14387fa90, 0x1401e9770, 0x338, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140da9700
    Replacement::draft_throws(0x140da9790, "content_140da9790", tpl_17172::<0x14387fa90, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f8be8, 0x2, 0x14020f120, 0x4, 0x143250238, 0x14020f240> as *const (), Throws::Discard), // sub_140da9790
    Replacement::draft(0x140da9880, "content_140da9880", fn_140da9880 as *const ()), // sub_140da9880
    Replacement::draft(0x140da9920, "content_140da9920", fn_140da9920 as *const ()), // sub_140da9920
    Replacement::draft(0x140da99e0, "content_140da99e0", tpl_17175::<0x14387f9e8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x48, 0x14091f400, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da99e0
    Replacement::draft_throws(0x140da9aa0, "content_140da9aa0", fn_140da9aa0 as *const (), Throws::Caught), // sub_140da9aa0
    Replacement::draft(0x140da9c60, "content_140da9c60", tpl_2343::<0x14387f9e8, 0x14243b0b0, 0x338, 0x20, 0x48, 0x14091f770, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140da9c60
    Replacement::draft_throws(0x140da9ce0, "content_140da9ce0", fn_140da9ce0 as *const (), Throws::Caught), // sub_140da9ce0
    Replacement::draft_throws(0x140da9ea0, "content_140da9ea0", fn_140da9ea0 as *const (), Throws::Caught), // sub_140da9ea0
    Replacement::draft_throws(0x140da9f80, "content_140da9f80", fn_140da9f80 as *const (), Throws::Caught), // sub_140da9f80
    Replacement::draft_throws(0x140daa060, "content_140daa060", fn_140daa060 as *const (), Throws::Caught), // sub_140daa060
    Replacement::draft(0x140daa140, "content_140daa140", tpl_17179::<0x14387f9e8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x48, 0x140920020, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140daa140
    Replacement::draft(0x140daa1f0, "content_140daa1f0", tpl_2342::<0x14387f8f0, 0x14387f9e8, 0x1401e9970, 0x338, 0xc0, 0x48, 0x1409201b0, 0x1401e9860> as *const ()), // sub_140daa1f0
    Replacement::draft(0x140daa260, "content_140daa260", tpl_2343::<0x14387f9e8, 0x14243b0b0, 0x338, 0xc0, 0x48, 0x140920320, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140daa260
    Replacement::draft_throws(0x140daa2e0, "content_140daa2e0", tpl_17180::<0x14387f8f0, 0x14387f9e8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x48, 0x140920470, 0x1401e9590> as *const (), Throws::Discard), // sub_140daa2e0
    Replacement::draft_throws(0x140daa350, "content_140daa350", fn_140daa350 as *const (), Throws::Caught), // sub_140daa350
    Replacement::draft(0x140daa4b0, "content_140daa4b0", tpl_9044::<0x48, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc> as *const ()), // sub_140daa4b0
    Replacement::draft(0x140daa550, "content_140daa550", fn_140daa550 as *const ()), // sub_140daa550
    Replacement::draft(0x140daa610, "content_140daa610", fn_140daa610 as *const ()), // sub_140daa610
    Replacement::draft(0x140daa680, "content_140daa680", fn_140daa680 as *const ()), // sub_140daa680
    Replacement::draft_throws(0x140daa7c0, "content_140daa7c0", fn_140daa7c0 as *const (), Throws::Discard), // sub_140daa7c0
    Replacement::draft(0x140daa8f0, "content_140daa8f0", tpl_17186::<0x14387fa20, 0x14243b0b0, 0x338, 0x40, 0x4, 0x38, 0x8, 0x80, 0xc, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140daa8f0
    Replacement::draft_throws(0x140daa9a0, "content_140daa9a0", fn_140daa9a0 as *const (), Throws::Discard), // sub_140daa9a0
    Replacement::draft_throws(0x140daaad0, "content_140daaad0", fn_140daaad0 as *const (), Throws::Caught), // sub_140daaad0
    Replacement::draft_throws(0x140daadc0, "content_140daadc0", fn_140daadc0 as *const (), Throws::Caught), // sub_140daadc0
    Replacement::draft_throws(0x140dab080, "content_140dab080", fn_140dab080 as *const (), Throws::Caught), // sub_140dab080
    Replacement::draft(0x140dab330, "content_140dab330", fn_140dab330 as *const ()), // sub_140dab330
    Replacement::draft(0x140dab3f0, "content_140dab3f0", fn_140dab3f0 as *const ()), // sub_140dab3f0
    Replacement::draft(0x140dab490, "content_140dab490", tpl_17186::<0x14387fa20, 0x14243b0b0, 0x338, 0xe0, 0x4, 0xd8, 0x8, 0x120, 0xc, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140dab490
    Replacement::draft_throws(0x140dab540, "content_140dab540", fn_140dab540 as *const (), Throws::Discard), // sub_140dab540
    Replacement::draft_throws(0x140dab600, "content_140dab600", fn_140dab600 as *const (), Throws::Discard), // sub_140dab600
    Replacement::draft(0x140dab750, "content_140dab750", fn_140dab750 as *const ()), // sub_140dab750
    Replacement::draft(0x140dab810, "content_140dab810", fn_140dab810 as *const ()), // sub_140dab810
    Replacement::draft(0x140dab880, "content_140dab880", tpl_11191::<0x14387f918, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x88, 0x1426d4580, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140dab880
    Replacement::draft_throws(0x140dab970, "content_140dab970", fn_140dab970 as *const (), Throws::Discard), // sub_140dab970
    Replacement::draft(0x140daba50, "content_140daba50", tpl_2258::<0x14387f918, 0x14243b0b0, 0x338, 0x20, 0x88, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140daba50
    Replacement::draft_throws(0x140dabad0, "content_140dabad0", fn_140dabad0 as *const (), Throws::Discard), // sub_140dabad0
    Replacement::draft_throws(0x140dabbc0, "content_140dabbc0", fn_140dabbc0 as *const (), Throws::Caught), // sub_140dabbc0
    Replacement::draft_throws(0x140dabf50, "content_140dabf50", fn_140dabf50 as *const (), Throws::Caught), // sub_140dabf50
    Replacement::draft_throws(0x140dac280, "content_140dac280", fn_140dac280 as *const (), Throws::Caught), // sub_140dac280
    Replacement::draft(0x140dac5a0, "content_140dac5a0", fn_140dac5a0 as *const ()), // sub_140dac5a0
    Replacement::draft(0x140dac640, "content_140dac640", tpl_75597::<0x14387fa2c, 0x14387f918, 0x1401e9970, 0x338, 0xc0, 0x88, 0xe8, 0x1401e9860> as *const ()), // sub_140dac640
    Replacement::draft(0x140dac6d0, "content_140dac6d0", tpl_17203::<0x14387f918, 0x14243b0b0, 0x338, 0xc0, 0x88, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dac6d0
    Replacement::draft_throws(0x140dac770, "content_140dac770", tpl_11196::<0x14387fa2c, 0x14387f918, 0x1401e9770, 0x338, 0x1424b4cb0, 0x88, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140dac770
    Replacement::draft_throws(0x140dac7f0, "content_140dac7f0", fn_140dac7f0 as *const (), Throws::Discard), // sub_140dac7f0
    Replacement::draft(0x140dac8d0, "content_140dac8d0", fn_140dac8d0 as *const ()), // sub_140dac8d0
    Replacement::draft(0x140dac9d0, "content_140dac9d0", fn_140dac9d0 as *const ()), // sub_140dac9d0
    Replacement::draft(0x140daca50, "content_140daca50", fn_140daca50 as *const ()), // sub_140daca50
    Replacement::draft_throws(0x140dacbe0, "content_140dacbe0", fn_140dacbe0 as *const (), Throws::Discard), // sub_140dacbe0
    Replacement::draft(0x140dacd60, "content_140dacd60", fn_140dacd60 as *const ()), // sub_140dacd60
    Replacement::draft_throws(0x140dace10, "content_140dace10", fn_140dace10 as *const (), Throws::Discard), // sub_140dace10
    Replacement::draft(0x140dacf90, "content_140dacf90", fn_140dacf90 as *const ()), // sub_140dacf90
    Replacement::draft_throws(0x140dad000, "content_140dad000", fn_140dad000 as *const (), Throws::Caught), // sub_140dad000
    Replacement::draft_throws(0x140dad670, "content_140dad670", fn_140dad670 as *const (), Throws::Caught), // sub_140dad670
    Replacement::draft_throws(0x140dadc20, "content_140dadc20", fn_140dadc20 as *const (), Throws::Caught), // sub_140dadc20
    Replacement::draft(0x140dae1a0, "content_140dae1a0", fn_140dae1a0 as *const ()), // sub_140dae1a0
    Replacement::draft(0x140dae2b0, "content_140dae2b0", fn_140dae2b0 as *const ()), // sub_140dae2b0
    Replacement::draft(0x140dae370, "content_140dae370", fn_140dae370 as *const ()), // sub_140dae370
    Replacement::draft_throws(0x140dae440, "content_140dae440", fn_140dae440 as *const (), Throws::Discard), // sub_140dae440
    Replacement::draft_throws(0x140dae4f0, "content_140dae4f0", fn_140dae4f0 as *const (), Throws::Discard), // sub_140dae4f0
    Replacement::draft(0x140dae630, "content_140dae630", fn_140dae630 as *const ()), // sub_140dae630
    Replacement::draft(0x140dae6f0, "content_140dae6f0", fn_140dae6f0 as *const ()), // sub_140dae6f0
    Replacement::draft(0x140dae760, "content_140dae760", fn_140dae760 as *const ()), // sub_140dae760
    Replacement::draft(0x140dae800, "content_140dae800", fn_140dae800 as *const ()), // sub_140dae800
    Replacement::draft_throws(0x140dae8b0, "content_140dae8b0", fn_140dae8b0 as *const (), Throws::Discard), // sub_140dae8b0
    Replacement::draft(0x140dae940, "content_140dae940", tpl_75619::<0x14387f8b0, 0x14243b0b0, 0x338, 0x40, 0x4, 0x140dbe820, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dae940
    Replacement::draft_throws(0x140dae9c0, "content_140dae9c0", fn_140dae9c0 as *const (), Throws::Discard), // sub_140dae9c0
    Replacement::draft_throws(0x140daea50, "content_140daea50", fn_140daea50 as *const (), Throws::Caught), // sub_140daea50
    Replacement::draft_throws(0x140daeb30, "content_140daeb30", fn_140daeb30 as *const (), Throws::Caught), // sub_140daeb30
    Replacement::draft_throws(0x140daec10, "content_140daec10", fn_140daec10 as *const (), Throws::Caught), // sub_140daec10
    Replacement::draft(0x140daecf0, "content_140daecf0", fn_140daecf0 as *const ()), // sub_140daecf0
    Replacement::draft(0x140daed80, "content_140daed80", fn_140daed80 as *const ()), // sub_140daed80
    Replacement::draft(0x140daedf0, "content_140daedf0", tpl_75619::<0x14387f8b0, 0x14243b0b0, 0x338, 0xe0, 0x4, 0x140dbf4a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140daedf0
    Replacement::draft_throws(0x140daee70, "content_140daee70", fn_140daee70 as *const (), Throws::Discard), // sub_140daee70
    Replacement::draft_throws(0x140daeef0, "content_140daeef0", fn_140daeef0 as *const (), Throws::Caught), // sub_140daeef0
    Replacement::draft_throws(0x140daf110, "content_140daf110", tpl_17230::<0x14387fa14, 0x14387f970, 0x1401e9fa0, 0x140dccf80, 0x1401e9e70> as *const (), Throws::Discard), // sub_140daf110
    Replacement::draft_throws(0x140daf1e0, "content_140daf1e0", tpl_17230::<0x14387fa14, 0x14387f970, 0x1401e9b60, 0x140dcd390, 0x1401e9a20> as *const (), Throws::Discard), // sub_140daf1e0
    Replacement::draft_throws(0x140daf240, "content_140daf240", fn_140daf240 as *const (), Throws::Caught), // sub_140daf240
    Replacement::draft_throws(0x140daf320, "content_140daf320", fn_140daf320 as *const (), Throws::Caught), // sub_140daf320
    Replacement::draft_throws(0x140daf400, "content_140daf400", fn_140daf400 as *const (), Throws::Caught), // sub_140daf400
    Replacement::draft(0x140daf4e0, "content_140daf4e0", tpl_17232::<0x14387f970, 0x14243b0b0, 0x338, 0x140907160, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140daf4e0
    Replacement::draft_throws(0x140daf5d0, "content_140daf5d0", tpl_17233::<0x14387f970, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f9560, 0x140dcd900> as *const (), Throws::Discard), // sub_140daf5d0
    Replacement::draft(0x140daf670, "content_140daf670", fn_140daf670 as *const ()), // sub_140daf670
    Replacement::draft(0x140daf720, "content_140daf720", fn_140daf720 as *const ()), // sub_140daf720
    Replacement::draft(0x140daf790, "content_140daf790", fn_140daf790 as *const ()), // sub_140daf790
    Replacement::draft_throws(0x140daf8c0, "content_140daf8c0", fn_140daf8c0 as *const (), Throws::Discard), // sub_140daf8c0
    Replacement::draft(0x140daf9f0, "content_140daf9f0", tpl_16407::<0x14387fa50, 0x14243b0b0, 0x338, 0x20, 0x28, 0x50, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140daf9f0
    Replacement::draft_throws(0x140dafa80, "content_140dafa80", fn_140dafa80 as *const (), Throws::Discard), // sub_140dafa80
    Replacement::draft_throws(0x140dafbb0, "content_140dafbb0", fn_140dafbb0 as *const (), Throws::Caught), // sub_140dafbb0
    Replacement::draft_throws(0x140daffd0, "content_140daffd0", fn_140daffd0 as *const (), Throws::Caught), // sub_140daffd0
    Replacement::draft_throws(0x140db0390, "content_140db0390", fn_140db0390 as *const (), Throws::Caught), // sub_140db0390
    Replacement::draft(0x140db0730, "content_140db0730", fn_140db0730 as *const ()), // sub_140db0730
    Replacement::draft(0x140db0810, "content_140db0810", tpl_74766::<0x14387fa30, 0x14387fa50, 0x1401e9970, 0x338, 0xc0, 0x28, 0x50, 0xd8, 0x1401e9860> as *const ()), // sub_140db0810
    Replacement::draft(0x140db08a0, "content_140db08a0", tpl_16407::<0x14387fa50, 0x14243b0b0, 0x338, 0xc0, 0x28, 0x50, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db08a0
    Replacement::draft_throws(0x140db0940, "content_140db0940", tpl_74767::<0x14387fa30, 0x14387fa50, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x50, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140db0940
    Replacement::draft_throws(0x140db09d0, "content_140db09d0", fn_140db09d0 as *const (), Throws::Discard), // sub_140db09d0
    Replacement::draft(0x140db0af0, "content_140db0af0", fn_140db0af0 as *const ()), // sub_140db0af0
    Replacement::draft(0x140db0b80, "content_140db0b80", fn_140db0b80 as *const ()), // sub_140db0b80
    Replacement::draft(0x140db0c30, "content_140db0c30", fn_140db0c30 as *const ()), // sub_140db0c30
    Replacement::draft_throws(0x140db0e20, "content_140db0e20", fn_140db0e20 as *const (), Throws::Discard), // sub_140db0e20
    Replacement::draft(0x140db0fc0, "content_140db0fc0", fn_140db0fc0 as *const ()), // sub_140db0fc0
    Replacement::draft_throws(0x140db10a0, "content_140db10a0", fn_140db10a0 as *const (), Throws::Discard), // sub_140db10a0
    Replacement::draft(0x140db1290, "content_140db1290", fn_140db1290 as *const ()), // sub_140db1290
    Replacement::draft_throws(0x140db1320, "content_140db1320", fn_140db1320 as *const (), Throws::Caught), // sub_140db1320
    Replacement::draft_throws(0x140db17d0, "content_140db17d0", fn_140db17d0 as *const (), Throws::Caught), // sub_140db17d0
    Replacement::draft_throws(0x140db1c00, "content_140db1c00", fn_140db1c00 as *const (), Throws::Caught), // sub_140db1c00
    Replacement::draft(0x140db2020, "content_140db2020", fn_140db2020 as *const ()), // sub_140db2020
    Replacement::draft(0x140db2120, "content_140db2120", fn_140db2120 as *const ()), // sub_140db2120
    Replacement::draft(0x140db2210, "content_140db2210", fn_140db2210 as *const ()), // sub_140db2210
    Replacement::draft_throws(0x140db2310, "content_140db2310", fn_140db2310 as *const (), Throws::Discard), // sub_140db2310
    Replacement::draft_throws(0x140db2420, "content_140db2420", fn_140db2420 as *const (), Throws::Discard), // sub_140db2420
    Replacement::draft(0x140db2610, "content_140db2610", fn_140db2610 as *const ()), // sub_140db2610
    Replacement::draft(0x140db2750, "content_140db2750", fn_140db2750 as *const ()), // sub_140db2750
    Replacement::draft(0x140db27f0, "content_140db27f0", fn_140db27f0 as *const ()), // sub_140db27f0
    Replacement::draft(0x140db28a0, "content_140db28a0", tpl_17260::<0x14387f848, 0x14243b0b0, 0x1401ee7c0, 0x1426d4580> as *const ()), // sub_140db28a0
    Replacement::draft_throws(0x140db2a00, "content_140db2a00", fn_140db2a00 as *const (), Throws::Discard), // sub_140db2a00
    Replacement::draft(0x140db2b30, "content_140db2b30", tpl_17262::<0x14387f848, 0x14243b0b0, 0x338, 0x20, 0x28, 0x40, 0x2c, 0x30, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140db2b30
    Replacement::draft_throws(0x140db2bd0, "content_140db2bd0", fn_140db2bd0 as *const (), Throws::Discard), // sub_140db2bd0
    Replacement::draft(0x140db2d20, "content_140db2d20", fn_140db2d20 as *const ()), // sub_140db2d20
    Replacement::draft_throws(0x140db2d80, "content_140db2d80", fn_140db2d80 as *const (), Throws::Caught), // sub_140db2d80
    Replacement::draft_throws(0x140db3070, "content_140db3070", fn_140db3070 as *const (), Throws::Caught), // sub_140db3070
    Replacement::draft_throws(0x140db3320, "content_140db3320", fn_140db3320 as *const (), Throws::Caught), // sub_140db3320
    Replacement::draft(0x140db35c0, "content_140db35c0", fn_140db35c0 as *const ()), // sub_140db35c0
    Replacement::draft(0x140db3690, "content_140db3690", tpl_17269::<0x14387fa5c, 0x14387f848, 0x1401e9970, 0x338, 0xc0, 0x28, 0xe0, 0x2c, 0x30, 0xd8, 0x1401e9860> as *const ()), // sub_140db3690
    Replacement::draft(0x140db3730, "content_140db3730", tpl_17262::<0x14387f848, 0x14243b0b0, 0x338, 0xc0, 0x28, 0xe0, 0x2c, 0x30, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140db3730
    Replacement::draft_throws(0x140db37e0, "content_140db37e0", tpl_17270::<0x14387fa5c, 0x14387f848, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x120, 0x2c, 0x30, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140db37e0
    Replacement::draft_throws(0x140db38a0, "content_140db38a0", fn_140db38a0 as *const (), Throws::Discard), // sub_140db38a0
    Replacement::draft_throws(0x140db3a60, "content_140db3a60", tpl_17230::<0x14387f938, 0x14387f8a0, 0x1401e9fa0, 0x140dccf80, 0x1401e9e70> as *const (), Throws::Discard), // sub_140db3a60
    Replacement::draft_throws(0x140db3b30, "content_140db3b30", tpl_17230::<0x14387f938, 0x14387f8a0, 0x1401e9b60, 0x140dcd390, 0x1401e9a20> as *const (), Throws::Discard), // sub_140db3b30
    Replacement::draft_throws(0x140db3b90, "content_140db3b90", fn_140db3b90 as *const (), Throws::Caught), // sub_140db3b90
    Replacement::draft_throws(0x140db3c70, "content_140db3c70", fn_140db3c70 as *const (), Throws::Caught), // sub_140db3c70
    Replacement::draft_throws(0x140db3d50, "content_140db3d50", fn_140db3d50 as *const (), Throws::Caught), // sub_140db3d50
    Replacement::draft(0x140db3e30, "content_140db3e30", tpl_17232::<0x14387f8a0, 0x14243b0b0, 0x338, 0x140907160, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db3e30
    Replacement::draft_throws(0x140db3f20, "content_140db3f20", tpl_17233::<0x14387f8a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f9560, 0x140dcd900> as *const (), Throws::Discard), // sub_140db3f20
    Replacement::draft(0x140db3fc0, "content_140db3fc0", tpl_17260::<0x14387fa88, 0x14243b0b0, 0x1401ee7c0, 0x1426d4580> as *const ()), // sub_140db3fc0
    Replacement::draft_throws(0x140db4120, "content_140db4120", fn_140db4120 as *const (), Throws::Discard), // sub_140db4120
    Replacement::draft(0x140db4240, "content_140db4240", tpl_17262::<0x14387fa88, 0x14243b0b0, 0x338, 0x20, 0x28, 0x40, 0x2c, 0x30, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140db4240
    Replacement::draft_throws(0x140db42e0, "content_140db42e0", fn_140db42e0 as *const (), Throws::Discard), // sub_140db42e0
    Replacement::draft(0x140db4430, "content_140db4430", fn_140db4430 as *const ()), // sub_140db4430
    Replacement::draft_throws(0x140db4490, "content_140db4490", fn_140db4490 as *const (), Throws::Caught), // sub_140db4490
    Replacement::draft_throws(0x140db4780, "content_140db4780", fn_140db4780 as *const (), Throws::Caught), // sub_140db4780
    Replacement::draft_throws(0x140db4a30, "content_140db4a30", fn_140db4a30 as *const (), Throws::Caught), // sub_140db4a30
    Replacement::draft(0x140db4cd0, "content_140db4cd0", fn_140db4cd0 as *const ()), // sub_140db4cd0
    Replacement::draft(0x140db4da0, "content_140db4da0", tpl_17269::<0x14387f750, 0x14387fa88, 0x1401e9970, 0x338, 0xc0, 0x28, 0xe0, 0x2c, 0x30, 0xd8, 0x1401e9860> as *const ()), // sub_140db4da0
    Replacement::draft(0x140db4e40, "content_140db4e40", tpl_17262::<0x14387fa88, 0x14243b0b0, 0x338, 0xc0, 0x28, 0xe0, 0x2c, 0x30, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140db4e40
    Replacement::draft_throws(0x140db4ef0, "content_140db4ef0", tpl_17270::<0x14387f750, 0x14387fa88, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x120, 0x2c, 0x30, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140db4ef0
    Replacement::draft_throws(0x140db4fb0, "content_140db4fb0", fn_140db4fb0 as *const (), Throws::Discard), // sub_140db4fb0
    Replacement::draft(0x140db5100, "content_140db5100", tpl_9963::<0x14387f8c8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x30, 0x2b, 0x2c, 0x28, 0xfffffffffffffff8> as *const ()), // sub_140db5100
    Replacement::draft_throws(0x140db51f0, "content_140db51f0", tpl_68527::<0x14387f82c, 0x14387f8c8, 0x1401e9fa0, 0x328, 0x1426cdcb0, 0x338, 0x20, 0x369, 0x142409b50, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_140db51f0
    Replacement::draft(0x140db52f0, "content_140db52f0", tpl_9959::<0x14387f8c8, 0x14243b0b0, 0x338, 0x20, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db52f0
    Replacement::draft_throws(0x140db5370, "content_140db5370", tpl_68528::<0x14387f82c, 0x14387f8c8, 0x1401e9b60, 0x328, 0x1426cdcb0, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x28, 0x1401e9a20> as *const (), Throws::Discard), // sub_140db5370
    Replacement::draft_throws(0x140db5470, "content_140db5470", fn_140db5470 as *const (), Throws::Caught), // sub_140db5470
    Replacement::draft_throws(0x140db5850, "content_140db5850", fn_140db5850 as *const (), Throws::Caught), // sub_140db5850
    Replacement::draft_throws(0x140db5bd0, "content_140db5bd0", fn_140db5bd0 as *const (), Throws::Caught), // sub_140db5bd0
    Replacement::draft(0x140db5f30, "content_140db5f30", tpl_9964::<0x14387f8c8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db5f30
    Replacement::draft(0x140db6000, "content_140db6000", tpl_9961::<0x14387f82c, 0x14387f8c8, 0x1401e9970, 0x338, 0xc0, 0x28, 0x1401e9860> as *const ()), // sub_140db6000
    Replacement::draft(0x140db6080, "content_140db6080", tpl_9959::<0x14387f8c8, 0x14243b0b0, 0x338, 0xc0, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db6080
    Replacement::draft_throws(0x140db6110, "content_140db6110", tpl_9965::<0x14387f82c, 0x14387f8c8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x1401e9590> as *const (), Throws::Discard), // sub_140db6110
    Replacement::draft_throws(0x140db6180, "content_140db6180", tpl_10011::<0x14387f8c8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1432f9bb8, 0x1402c4730, 0x28, 0x1432f9b90> as *const (), Throws::Discard), // sub_140db6180
    Replacement::draft(0x140db6260, "content_140db6260", tpl_17175::<0x14387f8e8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x48, 0x14091dd40, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db6260
    Replacement::draft_throws(0x140db6320, "content_140db6320", fn_140db6320 as *const (), Throws::Caught), // sub_140db6320
    Replacement::draft(0x140db64e0, "content_140db64e0", tpl_2343::<0x14387f8e8, 0x14243b0b0, 0x338, 0x20, 0x48, 0x14091e0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db64e0
    Replacement::draft_throws(0x140db6560, "content_140db6560", fn_140db6560 as *const (), Throws::Caught), // sub_140db6560
    Replacement::draft_throws(0x140db6720, "content_140db6720", fn_140db6720 as *const (), Throws::Caught), // sub_140db6720
    Replacement::draft_throws(0x140db6800, "content_140db6800", fn_140db6800 as *const (), Throws::Caught), // sub_140db6800
    Replacement::draft_throws(0x140db68e0, "content_140db68e0", fn_140db68e0 as *const (), Throws::Caught), // sub_140db68e0
    Replacement::draft(0x140db69c0, "content_140db69c0", tpl_17179::<0x14387f8e8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x48, 0x14091e960, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db69c0
    Replacement::draft(0x140db6a70, "content_140db6a70", tpl_2342::<0x14387f994, 0x14387f8e8, 0x1401e9970, 0x338, 0xc0, 0x48, 0x14091eaf0, 0x1401e9860> as *const ()), // sub_140db6a70
    Replacement::draft(0x140db6ae0, "content_140db6ae0", tpl_2343::<0x14387f8e8, 0x14243b0b0, 0x338, 0xc0, 0x48, 0x14091ec60, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db6ae0
    Replacement::draft_throws(0x140db6b60, "content_140db6b60", tpl_17180::<0x14387f994, 0x14387f8e8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x48, 0x14091edb0, 0x1401e9590> as *const (), Throws::Discard), // sub_140db6b60
    Replacement::draft_throws(0x140db6bd0, "content_140db6bd0", fn_140db6bd0 as *const (), Throws::Caught), // sub_140db6bd0
    Replacement::draft(0x140db6d30, "content_140db6d30", fn_140db6d30 as *const ()), // sub_140db6d30
    Replacement::draft(0x140db6dd0, "content_140db6dd0", fn_140db6dd0 as *const ()), // sub_140db6dd0
    Replacement::draft(0x140db6e80, "content_140db6e80", fn_140db6e80 as *const ()), // sub_140db6e80
    Replacement::draft_throws(0x140db6f90, "content_140db6f90", fn_140db6f90 as *const (), Throws::Discard), // sub_140db6f90
    Replacement::draft(0x140db7090, "content_140db7090", tpl_9475::<0x14387f7a0, 0x14243b0b0, 0x338, 0x20, 0x28, 0x48, 0x2c, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140db7090
    Replacement::draft_throws(0x140db7120, "content_140db7120", fn_140db7120 as *const (), Throws::Discard), // sub_140db7120
    Replacement::draft_throws(0x140db7230, "content_140db7230", fn_140db7230 as *const (), Throws::Caught), // sub_140db7230
    Replacement::draft_throws(0x140db74d0, "content_140db74d0", fn_140db74d0 as *const (), Throws::Caught), // sub_140db74d0
    Replacement::draft_throws(0x140db7900, "content_140db7900", fn_140db7900 as *const (), Throws::Caught), // sub_140db7900
    Replacement::draft(0x140db7d10, "content_140db7d10", fn_140db7d10 as *const ()), // sub_140db7d10
    Replacement::draft(0x140db7dd0, "content_140db7dd0", fn_140db7dd0 as *const ()), // sub_140db7dd0
    Replacement::draft(0x140db7e70, "content_140db7e70", fn_140db7e70 as *const ()), // sub_140db7e70
    Replacement::draft_throws(0x140db7f20, "content_140db7f20", fn_140db7f20 as *const (), Throws::Discard), // sub_140db7f20
    Replacement::draft_throws(0x140db7fb0, "content_140db7fb0", fn_140db7fb0 as *const (), Throws::Discard), // sub_140db7fb0
    Replacement::draft(0x140db80b0, "content_140db80b0", fn_140db80b0 as *const ()), // sub_140db80b0
    Replacement::draft(0x140db8190, "content_140db8190", fn_140db8190 as *const ()), // sub_140db8190
    Replacement::draft(0x140db8210, "content_140db8210", fn_140db8210 as *const ()), // sub_140db8210
    Replacement::draft_throws(0x140db8480, "content_140db8480", fn_140db8480 as *const (), Throws::Discard), // sub_140db8480
    Replacement::draft(0x140db86f0, "content_140db86f0", fn_140db86f0 as *const ()), // sub_140db86f0
    Replacement::draft_throws(0x140db8860, "content_140db8860", fn_140db8860 as *const (), Throws::Discard), // sub_140db8860
    Replacement::draft(0x140db8af0, "content_140db8af0", fn_140db8af0 as *const ()), // sub_140db8af0
    Replacement::draft_throws(0x140db8bd0, "content_140db8bd0", fn_140db8bd0 as *const (), Throws::Caught), // sub_140db8bd0
    Replacement::draft_throws(0x140db8f40, "content_140db8f40", fn_140db8f40 as *const (), Throws::Caught), // sub_140db8f40
    Replacement::draft_throws(0x140db9370, "content_140db9370", fn_140db9370 as *const (), Throws::Caught), // sub_140db9370
    Replacement::draft(0x140db9780, "content_140db9780", fn_140db9780 as *const ()), // sub_140db9780
    Replacement::draft(0x140db98e0, "content_140db98e0", fn_140db98e0 as *const ()), // sub_140db98e0
    Replacement::draft(0x140db9a60, "content_140db9a60", fn_140db9a60 as *const ()), // sub_140db9a60
    Replacement::draft_throws(0x140db9bf0, "content_140db9bf0", fn_140db9bf0 as *const (), Throws::Discard), // sub_140db9bf0
    Replacement::draft_throws(0x140db9d40, "content_140db9d40", fn_140db9d40 as *const (), Throws::Discard), // sub_140db9d40
    Replacement::draft_throws(0x140db9fc0, "content_140db9fc0", fn_140db9fc0 as *const (), Throws::Discard), // sub_140db9fc0
    Replacement::draft(0x140dba040, "content_140dba040", fn_140dba040 as *const ()), // sub_140dba040
    Replacement::draft(0x140dba160, "content_140dba160", fn_140dba160 as *const ()), // sub_140dba160
    Replacement::draft_throws(0x140dba220, "content_140dba220", fn_140dba220 as *const (), Throws::Caught), // sub_140dba220
    Replacement::draft(0x140dba300, "content_140dba300", fn_140dba300 as *const ()), // sub_140dba300
    Replacement::draft_throws(0x140dba3d0, "content_140dba3d0", fn_140dba3d0 as *const (), Throws::Discard), // sub_140dba3d0
    Replacement::draft_throws(0x140dba450, "content_140dba450", fn_140dba450 as *const (), Throws::Caught), // sub_140dba450
    Replacement::draft_throws(0x140dba6a0, "content_140dba6a0", fn_140dba6a0 as *const (), Throws::Caught), // sub_140dba6a0
    Replacement::draft_throws(0x140dba8b0, "content_140dba8b0", fn_140dba8b0 as *const (), Throws::Caught), // sub_140dba8b0
    Replacement::draft(0x140dbaac0, "content_140dbaac0", fn_140dbaac0 as *const ()), // sub_140dbaac0
    Replacement::draft(0x140dbac20, "content_140dbac20", fn_140dbac20 as *const ()), // sub_140dbac20
    Replacement::draft(0x140dbacd0, "content_140dbacd0", fn_140dbacd0 as *const ()), // sub_140dbacd0
    Replacement::draft_throws(0x140dbad80, "content_140dbad80", fn_140dbad80 as *const (), Throws::Discard), // sub_140dbad80
    Replacement::draft_throws(0x140dbae20, "content_140dbae20", fn_140dbae20 as *const (), Throws::Discard), // sub_140dbae20
    Replacement::draft(0x140dbaee0, "content_140dbaee0", tpl_12520::<0x1432f8948, 0x8, 0x140dcdae0, 0x10, 0x1432f8908, 0x140dcdaf0> as *const ()), // sub_140dbaee0
    Replacement::draft(0x140dbaf50, "content_140dbaf50", tpl_17166::<0x14387f870, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbaf50
    Replacement::draft_throws(0x140dbb020, "content_140dbb020", tpl_17167::<0x14387fa80, 0x14387f870, 0x1401e9fa0, 0x328, 0x338, 0x40, 0x369, 0x4, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140dbb020
    Replacement::draft(0x140dbb0d0, "content_140dbb0d0", tpl_2810::<0x14387f870, 0x14243b0b0, 0x338, 0x40, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbb0d0
    Replacement::draft_throws(0x140dbb150, "content_140dbb150", tpl_17168::<0x14387fa80, 0x14387f870, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_140dbb150
    Replacement::draft_throws(0x140dbb210, "content_140dbb210", fn_140dbb210 as *const (), Throws::Caught), // sub_140dbb210
    Replacement::draft_throws(0x140dbb310, "content_140dbb310", fn_140dbb310 as *const (), Throws::Caught), // sub_140dbb310
    Replacement::draft_throws(0x140dbb410, "content_140dbb410", fn_140dbb410 as *const (), Throws::Caught), // sub_140dbb410
    Replacement::draft(0x140dbb510, "content_140dbb510", tpl_17170::<0x14387f870, 0x14243b0b0, 0x338, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbb510
    Replacement::draft(0x140dbb590, "content_140dbb590", tpl_2812::<0x14387fa80, 0x14387f870, 0x1401e9970, 0x338, 0xe0, 0x4, 0xd8, 0x1401e9860> as *const ()), // sub_140dbb590
    Replacement::draft(0x140dbb610, "content_140dbb610", tpl_2810::<0x14387f870, 0x14243b0b0, 0x338, 0xe0, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbb610
    Replacement::draft_throws(0x140dbb6a0, "content_140dbb6a0", tpl_17171::<0x14387fa80, 0x14387f870, 0x1401e9770, 0x338, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140dbb6a0
    Replacement::draft_throws(0x140dbb730, "content_140dbb730", tpl_17172::<0x14387f870, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f8be8, 0x2, 0x14020f120, 0x4, 0x143251128, 0x14020f240> as *const (), Throws::Discard), // sub_140dbb730
    Replacement::draft(0x140dbb820, "content_140dbb820", fn_140dbb820 as *const ()), // sub_140dbb820
    Replacement::draft_throws(0x140dbb900, "content_140dbb900", fn_140dbb900 as *const (), Throws::Caught), // sub_140dbb900
    Replacement::draft(0x140dbb9f0, "content_140dbb9f0", fn_140dbb9f0 as *const ()), // sub_140dbb9f0
    Replacement::draft_throws(0x140dbbac0, "content_140dbbac0", fn_140dbbac0 as *const (), Throws::Discard), // sub_140dbbac0
    Replacement::draft_throws(0x140dbbb60, "content_140dbbb60", fn_140dbbb60 as *const (), Throws::Caught), // sub_140dbbb60
    Replacement::draft_throws(0x140dbbd80, "content_140dbbd80", fn_140dbbd80 as *const (), Throws::Caught), // sub_140dbbd80
    Replacement::draft_throws(0x140dbbf70, "content_140dbbf70", fn_140dbbf70 as *const (), Throws::Caught), // sub_140dbbf70
    Replacement::draft(0x140dbc160, "content_140dbc160", fn_140dbc160 as *const ()), // sub_140dbc160
    Replacement::draft(0x140dbc2a0, "content_140dbc2a0", fn_140dbc2a0 as *const ()), // sub_140dbc2a0
    Replacement::draft(0x140dbc340, "content_140dbc340", fn_140dbc340 as *const ()), // sub_140dbc340
    Replacement::draft_throws(0x140dbc3f0, "content_140dbc3f0", fn_140dbc3f0 as *const (), Throws::Discard), // sub_140dbc3f0
    Replacement::draft_throws(0x140dbc490, "content_140dbc490", fn_140dbc490 as *const (), Throws::Discard), // sub_140dbc490
    Replacement::draft(0x140dbc560, "content_140dbc560", tpl_2074::<0x14387fa60, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbc560
    Replacement::draft_throws(0x140dbc600, "content_140dbc600", tpl_2075::<0x14387f794, 0x14387fa60, 0x1401e9fa0, 0x328, 0x338, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140dbc600
    Replacement::draft(0x140dbc680, "content_140dbc680", tpl_2076::<0x14387fa60, 0x14243b0b0, 0x338, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbc680
    Replacement::draft_throws(0x140dbc6f0, "content_140dbc6f0", tpl_2077::<0x14387f794, 0x14387fa60, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140dbc6f0
    Replacement::draft_throws(0x140dbc780, "content_140dbc780", fn_140dbc780 as *const (), Throws::Caught), // sub_140dbc780
    Replacement::draft_throws(0x140dbc840, "content_140dbc840", fn_140dbc840 as *const (), Throws::Caught), // sub_140dbc840
    Replacement::draft_throws(0x140dbc900, "content_140dbc900", fn_140dbc900 as *const (), Throws::Caught), // sub_140dbc900
    Replacement::draft(0x140dbc9c0, "content_140dbc9c0", tpl_2079::<0x14387fa60, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbc9c0
    Replacement::draft(0x140dbca30, "content_140dbca30", tpl_2080::<0x14387f794, 0x14387fa60, 0x1401e9970, 0x338, 0xe0, 0x1401e9860> as *const ()), // sub_140dbca30
    Replacement::draft(0x140dbcaa0, "content_140dbcaa0", tpl_2076::<0x14387fa60, 0x14243b0b0, 0x338, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbcaa0
    Replacement::draft_throws(0x140dbcb10, "content_140dbcb10", tpl_2081::<0x14387f794, 0x14387fa60, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140dbcb10
    Replacement::draft_throws(0x140dbcb80, "content_140dbcb80", tpl_2082::<0x14387fa60, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f8be8, 0x1, 0x14020f120> as *const (), Throws::Discard), // sub_140dbcb80
    Replacement::draft(0x140dbcc40, "content_140dbcc40", fn_140dbcc40 as *const ()), // sub_140dbcc40
    Replacement::draft(0x140dbccd0, "content_140dbccd0", fn_140dbccd0 as *const ()), // sub_140dbccd0
    Replacement::draft(0x140dbcd80, "content_140dbcd80", tpl_11191::<0x14387f9a8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x28, 0x1426d4580, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140dbcd80
    Replacement::draft_throws(0x140dbce60, "content_140dbce60", tpl_71725::<0x14387f978, 0x14387f9a8, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x28, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140dbce60
    Replacement::draft(0x140dbcf40, "content_140dbcf40", tpl_2258::<0x14387f9a8, 0x14243b0b0, 0x338, 0x20, 0x28, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbcf40
    Replacement::draft_throws(0x140dbcfc0, "content_140dbcfc0", fn_140dbcfc0 as *const (), Throws::Discard), // sub_140dbcfc0
    Replacement::draft_throws(0x140dbd0b0, "content_140dbd0b0", fn_140dbd0b0 as *const (), Throws::Caught), // sub_140dbd0b0
    Replacement::draft_throws(0x140dbd330, "content_140dbd330", fn_140dbd330 as *const (), Throws::Caught), // sub_140dbd330
    Replacement::draft_throws(0x140dbd590, "content_140dbd590", fn_140dbd590 as *const (), Throws::Caught), // sub_140dbd590
    Replacement::draft(0x140dbd7e0, "content_140dbd7e0", tpl_11195::<0x14387f9a8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbd7e0
    Replacement::draft(0x140dbd890, "content_140dbd890", tpl_2262::<0x14387f978, 0x14387f9a8, 0x1401e9970, 0x338, 0xc0, 0x28, 0xd8, 0x1401e9860> as *const ()), // sub_140dbd890
    Replacement::draft(0x140dbd910, "content_140dbd910", tpl_2258::<0x14387f9a8, 0x14243b0b0, 0x338, 0xc0, 0x28, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbd910
    Replacement::draft_throws(0x140dbd9a0, "content_140dbd9a0", tpl_13321::<0x14387f978, 0x14387f9a8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140dbd9a0
    Replacement::draft_throws(0x140dbda20, "content_140dbda20", tpl_13322::<0x14387f9a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14329eef8, 0x1402c4730, 0x28, 0x1432f99c0, 0x14020f240> as *const (), Throws::Discard), // sub_140dbda20
    Replacement::draft(0x140dbdb10, "content_140dbdb10", tpl_13647::<0x14387f738, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbdb10
    Replacement::draft_throws(0x140dbdbc0, "content_140dbdbc0", tpl_13648::<0x14387fae4, 0x14387f738, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140dbdbc0
    Replacement::draft(0x140dbdc60, "content_140dbdc60", tpl_2076::<0x14387f738, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbdc60
    Replacement::draft_throws(0x140dbdcd0, "content_140dbdcd0", tpl_13649::<0x14387fae4, 0x14387f738, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140dbdcd0
    Replacement::draft_throws(0x140dbdd70, "content_140dbdd70", fn_140dbdd70 as *const (), Throws::Caught), // sub_140dbdd70
    Replacement::draft_throws(0x140dbdfd0, "content_140dbdfd0", fn_140dbdfd0 as *const (), Throws::Caught), // sub_140dbdfd0
    Replacement::draft_throws(0x140dbe200, "content_140dbe200", fn_140dbe200 as *const (), Throws::Caught), // sub_140dbe200
    Replacement::draft(0x140dbe420, "content_140dbe420", tpl_13653::<0x14387f738, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbe420
    Replacement::draft(0x140dbe4b0, "content_140dbe4b0", tpl_2080::<0x14387fae4, 0x14387f738, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // sub_140dbe4b0
    Replacement::draft(0x140dbe520, "content_140dbe520", tpl_2076::<0x14387f738, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbe520
    Replacement::draft_throws(0x140dbe590, "content_140dbe590", tpl_13654::<0x14387f738, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14307eae0, 0x1402c4730> as *const (), Throws::Discard), // sub_140dbe590
    Replacement::draft(0x140dbe650, "content_140dbe650", tpl_11191::<0x14387f740, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x88, 0x1426d4580, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140dbe650
    Replacement::draft_throws(0x140dbe740, "content_140dbe740", tpl_71725::<0x14387fa28, 0x14387f740, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x88, 0x48, 0x1401e9e70> as *const (), Throws::Discard), // sub_140dbe740
    Replacement::draft(0x140dbe820, "content_140dbe820", tpl_2258::<0x14387f740, 0x14243b0b0, 0x338, 0x20, 0x88, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbe820
    Replacement::draft_throws(0x140dbe8a0, "content_140dbe8a0", tpl_11193::<0x14387fa28, 0x14387f740, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x120, 0x88, 0x1401e9a20> as *const (), Throws::Discard), // sub_140dbe8a0
    Replacement::draft_throws(0x140dbe990, "content_140dbe990", fn_140dbe990 as *const (), Throws::Caught), // sub_140dbe990
    Replacement::draft_throws(0x140dbed20, "content_140dbed20", fn_140dbed20 as *const (), Throws::Caught), // sub_140dbed20
    Replacement::draft_throws(0x140dbf050, "content_140dbf050", fn_140dbf050 as *const (), Throws::Caught), // sub_140dbf050
    Replacement::draft(0x140dbf370, "content_140dbf370", fn_140dbf370 as *const ()), // sub_140dbf370
    Replacement::draft(0x140dbf410, "content_140dbf410", tpl_75597::<0x14387fa28, 0x14387f740, 0x1401e9970, 0x338, 0xc0, 0x88, 0xe8, 0x1401e9860> as *const ()), // sub_140dbf410
    Replacement::draft(0x140dbf4a0, "content_140dbf4a0", tpl_17203::<0x14387f740, 0x14243b0b0, 0x338, 0xc0, 0x88, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dbf4a0
    Replacement::draft_throws(0x140dbf540, "content_140dbf540", tpl_11196::<0x14387fa28, 0x14387f740, 0x1401e9770, 0x338, 0x1424b4cb0, 0x88, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140dbf540
    Replacement::draft_throws(0x140dbf5c0, "content_140dbf5c0", fn_140dbf5c0 as *const (), Throws::Discard), // sub_140dbf5c0
    Replacement::draft(0x140dbf6a0, "content_140dbf6a0", fn_140dbf6a0 as *const ()), // sub_140dbf6a0
    Replacement::draft(0x140dbf740, "content_140dbf740", fn_140dbf740 as *const ()), // sub_140dbf740
    Replacement::draft(0x140dbf810, "content_140dbf810", fn_140dbf810 as *const ()), // sub_140dbf810
    Replacement::draft(0x140dbf960, "content_140dbf960", fn_140dbf960 as *const ()), // sub_140dbf960
    Replacement::draft_throws(0x140dbfa90, "content_140dbfa90", tpl_17350::<0x14387fae8, 0x14387f910, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x140e167b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140dbfa90
    Replacement::draft_throws(0x140dbfb30, "content_140dbfb30", tpl_17351::<0x14387f910, 0x14243b0b0, 0x338, 0x48, 0x140e17030, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140dbfb30
    Replacement::draft_throws(0x140dbfbe0, "content_140dbfbe0", tpl_17352::<0x14387fae8, 0x14387f910, 0x1401e9b60, 0x328, 0x338, 0x120, 0x140e179e0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140dbfbe0
    Replacement::draft_throws(0x140dbfc80, "content_140dbfc80", fn_140dbfc80 as *const (), Throws::Caught), // sub_140dbfc80
    Replacement::draft_throws(0x140dbfd20, "content_140dbfd20", fn_140dbfd20 as *const (), Throws::Caught), // sub_140dbfd20
    Replacement::draft_throws(0x140dbfdc0, "content_140dbfdc0", fn_140dbfdc0 as *const (), Throws::Caught), // sub_140dbfdc0
    Replacement::draft(0x140dbfe60, "content_140dbfe60", fn_140dbfe60 as *const ()), // sub_140dbfe60
    Replacement::draft(0x140dbffd0, "content_140dbffd0", fn_140dbffd0 as *const ()), // sub_140dbffd0
    Replacement::draft(0x140dc00d0, "content_140dc00d0", fn_140dc00d0 as *const ()), // sub_140dc00d0
    Replacement::draft_throws(0x140dc01e0, "content_140dc01e0", fn_140dc01e0 as *const (), Throws::Discard), // sub_140dc01e0
    Replacement::draft_throws(0x140dc02d0, "content_140dc02d0", tpl_17357::<0x14387f910, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f9a88, 0x140dd1a40> as *const (), Throws::Discard), // sub_140dc02d0
    Replacement::draft(0x140dc0370, "content_140dc0370", fn_140dc0370 as *const ()), // sub_140dc0370
    Replacement::draft(0x140dc03f0, "content_140dc03f0", fn_140dc03f0 as *const ()), // sub_140dc03f0
    Replacement::draft_throws(0x140dc04a0, "content_140dc04a0", fn_140dc04a0 as *const (), Throws::Discard), // sub_140dc04a0
    Replacement::draft(0x140dc0620, "content_140dc0620", fn_140dc0620 as *const ()), // sub_140dc0620
    Replacement::draft_throws(0x140dc07b0, "content_140dc07b0", fn_140dc07b0 as *const (), Throws::Discard), // sub_140dc07b0
    Replacement::draft(0x140dc08c0, "content_140dc08c0", fn_140dc08c0 as *const ()), // sub_140dc08c0
    Replacement::draft_throws(0x140dc09a0, "content_140dc09a0", fn_140dc09a0 as *const (), Throws::Caught), // sub_140dc09a0
    Replacement::draft_throws(0x140dc0a80, "content_140dc0a80", fn_140dc0a80 as *const (), Throws::Caught), // sub_140dc0a80
    Replacement::draft_throws(0x140dc0b60, "content_140dc0b60", fn_140dc0b60 as *const (), Throws::Caught), // sub_140dc0b60
    Replacement::draft(0x140dc0c40, "content_140dc0c40", fn_140dc0c40 as *const ()), // sub_140dc0c40
    Replacement::draft(0x140dc0da0, "content_140dc0da0", fn_140dc0da0 as *const ()), // sub_140dc0da0
    Replacement::draft(0x140dc0f10, "content_140dc0f10", fn_140dc0f10 as *const ()), // sub_140dc0f10
    Replacement::draft_throws(0x140dc1090, "content_140dc1090", fn_140dc1090 as *const (), Throws::Discard), // sub_140dc1090
    Replacement::draft_throws(0x140dc1210, "content_140dc1210", fn_140dc1210 as *const (), Throws::Discard), // sub_140dc1210
    Replacement::draft(0x140dc12e0, "content_140dc12e0", fn_140dc12e0 as *const ()), // sub_140dc12e0
    Replacement::draft(0x140dc13e0, "content_140dc13e0", fn_140dc13e0 as *const ()), // sub_140dc13e0
    Replacement::draft(0x140dc14e0, "content_140dc14e0", fn_140dc14e0 as *const ()), // sub_140dc14e0
    Replacement::draft(0x140dc1620, "content_140dc1620", fn_140dc1620 as *const ()), // sub_140dc1620
    Replacement::draft_throws(0x140dc18b0, "content_140dc18b0", fn_140dc18b0 as *const (), Throws::Caught), // sub_140dc18b0
    Replacement::draft(0x140dc1c80, "content_140dc1c80", fn_140dc1c80 as *const ()), // sub_140dc1c80
    Replacement::draft_throws(0x140dc1ec0, "content_140dc1ec0", fn_140dc1ec0 as *const (), Throws::Caught), // sub_140dc1ec0
    Replacement::draft(0x140dc22b0, "content_140dc22b0", fn_140dc22b0 as *const ()), // sub_140dc22b0
    Replacement::draft_throws(0x140dc23c0, "content_140dc23c0", fn_140dc23c0 as *const (), Throws::Caught), // sub_140dc23c0
    Replacement::draft_throws(0x140dc2b10, "content_140dc2b10", fn_140dc2b10 as *const (), Throws::Caught), // sub_140dc2b10
    Replacement::draft_throws(0x140dc3190, "content_140dc3190", fn_140dc3190 as *const (), Throws::Caught), // sub_140dc3190
    Replacement::draft(0x140dc37e0, "content_140dc37e0", fn_140dc37e0 as *const ()), // sub_140dc37e0
    Replacement::draft(0x140dc39e0, "content_140dc39e0", fn_140dc39e0 as *const ()), // sub_140dc39e0
    Replacement::draft(0x140dc3b70, "content_140dc3b70", fn_140dc3b70 as *const ()), // sub_140dc3b70
    Replacement::draft_throws(0x140dc3d10, "content_140dc3d10", fn_140dc3d10 as *const (), Throws::Discard), // sub_140dc3d10
    Replacement::draft_throws(0x140dc3e90, "content_140dc3e90", fn_140dc3e90 as *const (), Throws::Caught), // sub_140dc3e90
    Replacement::draft(0x140dc4090, "content_140dc4090", fn_140dc4090 as *const ()), // sub_140dc4090
    Replacement::draft(0x140dc41d0, "content_140dc41d0", fn_140dc41d0 as *const ()), // sub_140dc41d0
    Replacement::draft(0x140dc42e0, "content_140dc42e0", fn_140dc42e0 as *const ()), // sub_140dc42e0
    Replacement::draft(0x140dc43a0, "content_140dc43a0", fn_140dc43a0 as *const ()), // sub_140dc43a0
    Replacement::draft(0x140dc4410, "content_140dc4410", fn_140dc4410 as *const ()), // sub_140dc4410
    Replacement::draft_throws(0x140dc4510, "content_140dc4510", fn_140dc4510 as *const (), Throws::Discard), // sub_140dc4510
    Replacement::draft(0x140dc4630, "content_140dc4630", tpl_16503::<0x14387fa98, 0x14243b0b0, 0x1408ff6f0, 0x338, 0x8, 0x80, 0x9, 0xc, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dc4630
    Replacement::draft_throws(0x140dc46d0, "content_140dc46d0", fn_140dc46d0 as *const (), Throws::Discard), // sub_140dc46d0
    Replacement::draft_throws(0x140dc47e0, "content_140dc47e0", fn_140dc47e0 as *const (), Throws::Caught), // sub_140dc47e0
    Replacement::draft_throws(0x140dc4ac0, "content_140dc4ac0", fn_140dc4ac0 as *const (), Throws::Caught), // sub_140dc4ac0
    Replacement::draft_throws(0x140dc4d70, "content_140dc4d70", fn_140dc4d70 as *const (), Throws::Caught), // sub_140dc4d70
    Replacement::draft(0x140dc5020, "content_140dc5020", fn_140dc5020 as *const ()), // sub_140dc5020
    Replacement::draft(0x140dc50f0, "content_140dc50f0", tpl_16502::<0x14387fa00, 0x14387fa98, 0x1401e9970, 0x140900130, 0x338, 0x8, 0x120, 0x9, 0xc, 0xc0, 0x1401e9860> as *const ()), // sub_140dc50f0
    Replacement::draft(0x140dc5190, "content_140dc5190", tpl_16503::<0x14387fa98, 0x14243b0b0, 0x1409002a0, 0x338, 0x8, 0x120, 0x9, 0xc, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140dc5190
    Replacement::draft_throws(0x140dc5240, "content_140dc5240", fn_140dc5240 as *const (), Throws::Discard), // sub_140dc5240
    Replacement::draft_throws(0x140dc52e0, "content_140dc52e0", fn_140dc52e0 as *const (), Throws::Discard), // sub_140dc52e0
    Replacement::draft(0x140dc5410, "content_140dc5410", fn_140dc5410 as *const ()), // sub_140dc5410
    Replacement::draft(0x140dc54e0, "content_140dc54e0", fn_140dc54e0 as *const ()), // sub_140dc54e0
    Replacement::draft(0x140dc55c0, "content_140dc55c0", fn_140dc55c0 as *const ()), // sub_140dc55c0
    Replacement::draft(0x140dc5690, "content_140dc5690", fn_140dc5690 as *const ()), // sub_140dc5690
    Replacement::draft_throws(0x140dc5880, "content_140dc5880", fn_140dc5880 as *const (), Throws::Caught), // sub_140dc5880
    Replacement::draft(0x140dc5a40, "content_140dc5a40", fn_140dc5a40 as *const ()), // sub_140dc5a40
    Replacement::draft_throws(0x140dc5bc0, "content_140dc5bc0", fn_140dc5bc0 as *const (), Throws::Caught), // sub_140dc5bc0
    Replacement::draft(0x140dc5d90, "content_140dc5d90", fn_140dc5d90 as *const ()), // sub_140dc5d90
    Replacement::draft_throws(0x140dc5e70, "content_140dc5e70", fn_140dc5e70 as *const (), Throws::Caught), // sub_140dc5e70
    Replacement::draft_throws(0x140dc62e0, "content_140dc62e0", fn_140dc62e0 as *const (), Throws::Caught), // sub_140dc62e0
    Replacement::draft_throws(0x140dc66d0, "content_140dc66d0", fn_140dc66d0 as *const (), Throws::Caught), // sub_140dc66d0
    Replacement::draft(0x140dc6ab0, "content_140dc6ab0", fn_140dc6ab0 as *const ()), // sub_140dc6ab0
    Replacement::draft(0x140dc6bf0, "content_140dc6bf0", fn_140dc6bf0 as *const ()), // sub_140dc6bf0
    Replacement::draft(0x140dc6d00, "content_140dc6d00", fn_140dc6d00 as *const ()), // sub_140dc6d00
    Replacement::draft_throws(0x140dc6e20, "content_140dc6e20", fn_140dc6e20 as *const (), Throws::Discard), // sub_140dc6e20
    Replacement::draft_throws(0x140dc6f30, "content_140dc6f30", fn_140dc6f30 as *const (), Throws::Discard), // sub_140dc6f30
    Replacement::draft(0x140dc7090, "content_140dc7090", tpl_17405::<0x1432f9868, 0x4, 0x143877e98, 0x1432f9898, 0x3, 0x1401de410, 0x8, 0x10, 0x1426cdcb0, 0x140df9770> as *const ()), // sub_140dc7090
    Replacement::draft(0x140dc7130, "content_140dc7130", fn_140dc7130 as *const ()), // sub_140dc7130
    Replacement::draft(0x140dc71d0, "content_140dc71d0", tpl_17405::<0x1432f9868, 0x4, 0x143877a00, 0x1432f9898, 0x3, 0x1401de410, 0x8, 0x10, 0x1432f9878, 0x140df97e0> as *const ()), // sub_140dc71d0
    Replacement::draft(0x140dc7270, "content_140dc7270", fn_140dc7270 as *const ()), // sub_140dc7270
    Replacement::draft(0x140dc7330, "content_140dc7330", fn_140dc7330 as *const ()), // sub_140dc7330
    Replacement::draft(0x140dc73a0, "content_140dc73a0", fn_140dc73a0 as *const ()), // sub_140dc73a0
    Replacement::draft_throws(0x140dc75e0, "content_140dc75e0", fn_140dc75e0 as *const (), Throws::Discard), // sub_140dc75e0
    Replacement::draft(0x140dc7800, "content_140dc7800", fn_140dc7800 as *const ()), // sub_140dc7800
    Replacement::draft_throws(0x140dc7930, "content_140dc7930", fn_140dc7930 as *const (), Throws::Discard), // sub_140dc7930
    Replacement::draft(0x140dc7b90, "content_140dc7b90", fn_140dc7b90 as *const ()), // sub_140dc7b90
    Replacement::draft_throws(0x140dc7c40, "content_140dc7c40", fn_140dc7c40 as *const (), Throws::Caught), // sub_140dc7c40
    Replacement::draft_throws(0x140dc8380, "content_140dc8380", fn_140dc8380 as *const (), Throws::Caught), // sub_140dc8380
    Replacement::draft_throws(0x140dc89d0, "content_140dc89d0", fn_140dc89d0 as *const (), Throws::Caught), // sub_140dc89d0
    Replacement::draft(0x140dc9000, "content_140dc9000", fn_140dc9000 as *const ()), // sub_140dc9000
    Replacement::draft(0x140dc9110, "content_140dc9110", fn_140dc9110 as *const ()), // sub_140dc9110
    Replacement::draft(0x140dc9220, "content_140dc9220", fn_140dc9220 as *const ()), // sub_140dc9220
    Replacement::draft_throws(0x140dc9330, "content_140dc9330", fn_140dc9330 as *const (), Throws::Discard), // sub_140dc9330
    Replacement::draft_throws(0x140dc9450, "content_140dc9450", fn_140dc9450 as *const (), Throws::Discard), // sub_140dc9450
    Replacement::draft(0x140dc9660, "content_140dc9660", fn_140dc9660 as *const ()), // sub_140dc9660
    Replacement::draft(0x140dc97c0, "content_140dc97c0", fn_140dc97c0 as *const ()), // sub_140dc97c0
    Replacement::draft(0x140dc9860, "content_140dc9860", fn_140dc9860 as *const ()), // sub_140dc9860
    Replacement::draft(0x140dc9930, "content_140dc9930", fn_140dc9930 as *const ()), // sub_140dc9930
    Replacement::draft(0x140dc9ac0, "content_140dc9ac0", fn_140dc9ac0 as *const ()), // sub_140dc9ac0
    Replacement::draft_throws(0x140dc9cc0, "content_140dc9cc0", fn_140dc9cc0 as *const (), Throws::Caught), // sub_140dc9cc0
    Replacement::draft_throws(0x140dc9e60, "content_140dc9e60", fn_140dc9e60 as *const (), Throws::Discard), // sub_140dc9e60
    Replacement::draft_throws(0x140dc9fc0, "content_140dc9fc0", fn_140dc9fc0 as *const (), Throws::Caught), // sub_140dc9fc0
    Replacement::draft_throws(0x140dca150, "content_140dca150", fn_140dca150 as *const (), Throws::Caught), // sub_140dca150
    Replacement::draft_throws(0x140dca430, "content_140dca430", fn_140dca430 as *const (), Throws::Caught), // sub_140dca430
    Replacement::draft_throws(0x140dca6d0, "content_140dca6d0", fn_140dca6d0 as *const (), Throws::Caught), // sub_140dca6d0
    Replacement::draft(0x140dca960, "content_140dca960", fn_140dca960 as *const ()), // sub_140dca960
    Replacement::draft(0x140dcab70, "content_140dcab70", fn_140dcab70 as *const ()), // sub_140dcab70
    Replacement::draft(0x140dcace0, "content_140dcace0", fn_140dcace0 as *const ()), // sub_140dcace0
    Replacement::draft_throws(0x140dcae60, "content_140dcae60", fn_140dcae60 as *const (), Throws::Discard), // sub_140dcae60
    Replacement::draft_throws(0x140dcafd0, "content_140dcafd0", fn_140dcafd0 as *const (), Throws::Discard), // sub_140dcafd0
    Replacement::draft(0x140dcb0b0, "content_140dcb0b0", fn_140dcb0b0 as *const ()), // content::AnimationMovementTimelineEvent::content::AnimationMovementTimelineEvent::vfunc_0
    Replacement::draft(0x140dcb100, "content_140dcb100", fn_140dcb100 as *const ()), // content::AnimationParameterTimelineEvent::content::AnimationParameterTimelineEvent::vfunc_0
    Replacement::draft(0x140dcb180, "content_140dcb180", fn_140dcb180 as *const ()), // content::AnimationSourceTimelineEvent::content::AnimationSourceTimelineEvent::vfunc_0
    Replacement::draft(0x140dcb1e0, "content_140dcb1e0", fn_140dcb1e0 as *const ()), // content::DialogueTimelineEvent::content::DialogueTimelineEvent::vfunc_0
    Replacement::draft(0x140dcb230, "content_140dcb230", fn_140dcb230 as *const ()), // content::AnimationTimelineTrack::content::AnimationTimelineTrack::vfunc_0
    Replacement::draft(0x140dcb290, "content_140dcb290", tpl_570::<0x140dd21b0, 0x1432766b0, 0x141834d30> as *const ()), // content::AnimationBlendSpace::content::AnimationBlendSpace::vfunc_0
    Replacement::draft(0x140dcb2f0, "content_140dcb2f0", fn_140dcb2f0 as *const ()), // content::AnimationGraph::content::AnimationGraph::vfunc_0
    Replacement::draft(0x140dcb3c0, "content_140dcb3c0", fn_140dcb3c0 as *const ()), // sub_140dcb3c0
    Replacement::draft(0x140dcb5f0, "content_140dcb5f0", fn_140dcb5f0 as *const ()), // content::AnimationGraphBlendNNode::content::AnimationGraphBlendNNode::vfunc_0
    Replacement::draft(0x140dcb640, "content_140dcb640", tpl_5108::<0x140dd25d0, 0x14324ab18, 0x141834d30> as *const ()), // content::AnimationGraphClothProfileNode::content::AnimationGraphClothProfileNode::vfunc_0
    Replacement::draft(0x140dcb6a0, "content_140dcb6a0", tpl_6942::<0x140dd2780, 0x18, 0x14324ab18, 0x141834d30> as *const ()), // content::AnimationGraphEmitNode::content::AnimationGraphEmitNode::vfunc_0
    Replacement::draft(0x140dcb700, "content_140dcb700", fn_140dcb700 as *const ()), // content::AnimationGraphExpressionNode::content::AnimationGraphExpressionNode::vfunc_0
    Replacement::draft(0x140dcb7a0, "content_140dcb7a0", fn_140dcb7a0 as *const ()), // content::AnimationGraphFeatherBlendNode::content::AnimationGraphFeatherBlendNode::vfunc_0
    Replacement::draft(0x140dcb7f0, "content_140dcb7f0", fn_140dcb7f0 as *const ()), // content::AnimationGraphFullBodyIKNode::content::AnimationGraphFullBodyIKNode::vfunc_0
    Replacement::draft(0x140dcb880, "content_140dcb880", fn_140dcb880 as *const ()), // content::AnimationGraphMatchNode::content::AnimationGraphMatchNode::vfunc_0
    Replacement::draft(0x140dcb930, "content_140dcb930", fn_140dcb930 as *const ()), // content::AnimationGraphMotionMatchingNode::content::AnimationGraphMotionMatchingNode::vfunc_0
    Replacement::draft(0x140dcb9b0, "content_140dcb9b0", tpl_17448::<0x14240ab10, 0x10, 0x140ab7f10, 0x14324ab18, 0x141834d30> as *const ()), // content::AnimationGraphRBFControlNode::content::AnimationGraphRBFControlNode::vfunc_0
    Replacement::draft(0x140dcba00, "content_140dcba00", fn_140dcba00 as *const ()), // content::AnimationGraphReparentNode::content::AnimationGraphReparentNode::vfunc_0
    Replacement::draft(0x140dcba60, "content_140dcba60", fn_140dcba60 as *const ()), // content::AnimationGraphSelectNode::content::AnimationGraphSelectNode::vfunc_0
    Replacement::draft(0x140dcbab0, "content_140dcbab0", fn_140dcbab0 as *const ()), // content::AnimationGraphSelectActionNode::content::AnimationGraphSelectActionNode::vfunc_0
    Replacement::draft(0x140dcbb00, "content_140dcbb00", fn_140dcbb00 as *const ()), // content::AnimationGraphSequenceNode::content::AnimationGraphSequenceNode::vfunc_0
    Replacement::draft(0x140dcbb50, "content_140dcbb50", tpl_17448::<0x14240ab10, 0x10, 0x140dd35a0, 0x14324ab18, 0x141834d30> as *const ()), // content::AnimationGraphUnevenTerrainNode::content::AnimationGraphUnevenTerrainNode::vfunc_0
    Replacement::draft(0x140dcbbb0, "content_140dcbbb0", fn_140dcbbb0 as *const ()), // content::AnimationGraphInputPoseNode::content::AnimationGraphInputPoseNode::vfunc_0
    Replacement::draft(0x140dcbc00, "content_140dcbc00", fn_140dcbc00 as *const ()), // sub_140dcbc00
    Replacement::draft(0x140dcbcb0, "content_140dcbcb0", fn_140dcbcb0 as *const ()), // sub_140dcbcb0
    Replacement::draft(0x140dcbd50, "content_140dcbd50", fn_140dcbd50 as *const ()), // content::AnimationGraphAimNode::content::AnimationGraphAimNode::vfunc_0
    Replacement::draft(0x140dcbed0, "content_140dcbed0", fn_140dcbed0 as *const ()), // content::AnimationGraphFootLockNode::content::AnimationGraphFootLockNode::vfunc_0
    Replacement::draft(0x140dcbf20, "content_140dcbf20", fn_140dcbf20 as *const ()), // content::AnimationGraphFootPredictionNode::content::AnimationGraphFootPredictionNode::vfunc_0
    Replacement::draft(0x140dcc020, "content_140dcc020", fn_140dcc020 as *const ()), // content::AnimationGraphTwoBoneIKNode::content::AnimationGraphTwoBoneIKNode::vfunc_0
    Replacement::draft(0x140dcc0c0, "content_140dcc0c0", fn_140dcc0c0 as *const ()), // sub_140dcc0c0
    Replacement::draft(0x140dcc190, "content_140dcc190", fn_140dcc190 as *const ()), // sub_140dcc190
    Replacement::draft_throws(0x140dcc260, "content_140dcc260", fn_140dcc260 as *const (), Throws::Caught), // sub_140dcc260
    Replacement::draft_throws(0x140dcc560, "content_140dcc560", fn_140dcc560 as *const (), Throws::Caught), // sub_140dcc560
    Replacement::draft_throws(0x140dcc860, "content_140dcc860", fn_140dcc860 as *const (), Throws::Caught), // sub_140dcc860
    Replacement::draft_throws(0x140dccb90, "content_140dccb90", tpl_9697::<0x20, 0x8, 0x140dd4d90> as *const (), Throws::Discard), // sub_140dccb90
    Replacement::draft(0x140dccbe0, "content_140dccbe0", fn_140dccbe0 as *const ()), // sub_140dccbe0
    Replacement::draft(0x140dccc70, "content_140dccc70", fn_140dccc70 as *const ()), // sub_140dccc70
    Replacement::draft(0x140dccda0, "content_140dccda0", fn_140dccda0 as *const ()), // sub_140dccda0
    Replacement::draft_throws(0x140dcce50, "content_140dcce50", fn_140dcce50 as *const (), Throws::Caught), // sub_140dcce50
    Replacement::draft_throws(0x140dccf80, "content_140dccf80", fn_140dccf80 as *const (), Throws::Caught), // sub_140dccf80
    Replacement::draft_throws(0x140dcd0b0, "content_140dcd0b0", fn_140dcd0b0 as *const (), Throws::Caught), // sub_140dcd0b0
    Replacement::draft_throws(0x140dcd260, "content_140dcd260", fn_140dcd260 as *const (), Throws::Caught), // sub_140dcd260
    Replacement::draft_throws(0x140dcd390, "content_140dcd390", fn_140dcd390 as *const (), Throws::Caught), // sub_140dcd390
    Replacement::draft_throws(0x140dcd4c0, "content_140dcd4c0", tpl_2506::<0x328, 0x338, 0x120, 0x140dd2280, 0x140dd5620, 0x369> as *const (), Throws::Discard), // sub_140dcd4c0
    Replacement::draft_throws(0x140dcd550, "content_140dcd550", fn_140dcd550 as *const (), Throws::Caught), // sub_140dcd550
    Replacement::draft(0x140dcd710, "content_140dcd710", fn_140dcd710 as *const ()), // sub_140dcd710
    Replacement::draft_throws(0x140dcd830, "content_140dcd830", fn_140dcd830 as *const (), Throws::Caught), // sub_140dcd830
    Replacement::draft_throws(0x140dcd900, "content_140dcd900", fn_140dcd900 as *const (), Throws::Caught), // sub_140dcd900
    Replacement::draft_throws(0x140dcd9e0, "content_140dcd9e0", tpl_17468::<0x20, 0x8, 0x140ddb440> as *const (), Throws::Discard), // sub_140dcd9e0
    Replacement::draft_throws(0x140dcda30, "content_140dcda30", tpl_17468::<0x20, 0x8, 0x140ddb510> as *const (), Throws::Discard), // sub_140dcda30
    Replacement::draft_throws(0x140dcda90, "content_140dcda90", tpl_2507::<0x20, 0x8, 0x140ddb830> as *const (), Throws::Discard), // sub_140dcda90
    Replacement::draft_throws(0x140dcdb00, "content_140dcdb00", fn_140dcdb00 as *const (), Throws::Caught), // sub_140dcdb00
    Replacement::draft_throws(0x140dcde00, "content_140dcde00", fn_140dcde00 as *const (), Throws::Caught), // sub_140dcde00
    Replacement::draft_throws(0x140dce100, "content_140dce100", fn_140dce100 as *const (), Throws::Caught), // sub_140dce100
    Replacement::draft_throws(0x140dce480, "content_140dce480", fn_140dce480 as *const (), Throws::Caught), // sub_140dce480
    Replacement::draft_throws(0x140dce5b0, "content_140dce5b0", fn_140dce5b0 as *const (), Throws::Caught), // sub_140dce5b0
    Replacement::draft_throws(0x140dce6e0, "content_140dce6e0", fn_140dce6e0 as *const (), Throws::Caught), // sub_140dce6e0
    Replacement::draft_throws(0x140dce7f0, "content_140dce7f0", tpl_6967::<0x20, 0x8, 0x140ddef10> as *const (), Throws::Discard), // sub_140dce7f0
    Replacement::draft(0x140dce840, "content_140dce840", fn_140dce840 as *const ()), // sub_140dce840
    Replacement::draft(0x140dce8d0, "content_140dce8d0", fn_140dce8d0 as *const ()), // sub_140dce8d0
    Replacement::draft_throws(0x140dce980, "content_140dce980", tpl_2506::<0x328, 0x338, 0x120, 0x140dd26a0, 0x140ddf320, 0x369> as *const (), Throws::Discard), // sub_140dce980
    Replacement::draft(0x140dcea10, "content_140dcea10", fn_140dcea10 as *const ()), // sub_140dcea10
    Replacement::draft_throws(0x140dcead0, "content_140dcead0", fn_140dcead0 as *const (), Throws::Discard), // sub_140dcead0
    Replacement::draft(0x140dceb20, "content_140dceb20", fn_140dceb20 as *const ()), // sub_140dceb20
    Replacement::draft_throws(0x140dcebd0, "content_140dcebd0", fn_140dcebd0 as *const (), Throws::Caught), // sub_140dcebd0
    Replacement::draft_throws(0x140dced80, "content_140dced80", fn_140dced80 as *const (), Throws::Caught), // sub_140dced80
    Replacement::draft(0x140dcef40, "content_140dcef40", fn_140dcef40 as *const ()), // sub_140dcef40
    Replacement::draft_throws(0x140dcf040, "content_140dcf040", tpl_17477::<0x20, 0x8, 0x140de2760> as *const (), Throws::Discard), // sub_140dcf040
    Replacement::draft_throws(0x140dcf090, "content_140dcf090", fn_140dcf090 as *const (), Throws::Discard), // sub_140dcf090
    Replacement::draft(0x140dcf0e0, "content_140dcf0e0", fn_140dcf0e0 as *const ()), // sub_140dcf0e0
    Replacement::draft_throws(0x140dcf210, "content_140dcf210", fn_140dcf210 as *const (), Throws::Caught), // sub_140dcf210
    Replacement::draft_throws(0x140dcf3c0, "content_140dcf3c0", fn_140dcf3c0 as *const (), Throws::Caught), // sub_140dcf3c0
    Replacement::draft_throws(0x140dcf570, "content_140dcf570", fn_140dcf570 as *const (), Throws::Caught), // sub_140dcf570
    Replacement::draft_throws(0x140dcf730, "content_140dcf730", fn_140dcf730 as *const (), Throws::Caught), // sub_140dcf730
    Replacement::draft_throws(0x140dcf980, "content_140dcf980", fn_140dcf980 as *const (), Throws::Discard), // sub_140dcf980
    Replacement::draft_throws(0x140dcf9d0, "content_140dcf9d0", tpl_6961::<0x20, 0x8, 0x140de6280> as *const (), Throws::Discard), // sub_140dcf9d0
    Replacement::draft_throws(0x140dcfa20, "content_140dcfa20", tpl_12664::<0x20, 0x8, 0x140de6350> as *const (), Throws::Discard), // sub_140dcfa20
    Replacement::draft_throws(0x140dcfae0, "content_140dcfae0", fn_140dcfae0 as *const (), Throws::Caught), // sub_140dcfae0
    Replacement::draft_throws(0x140dcfc10, "content_140dcfc10", fn_140dcfc10 as *const (), Throws::Caught), // sub_140dcfc10
    Replacement::draft_throws(0x140dcfd40, "content_140dcfd40", tpl_17484::<0x54, 0x18, 0x20, 0x40, 0x140de7d60> as *const (), Throws::Discard), // sub_140dcfd40
    Replacement::draft_throws(0x140dcfd90, "content_140dcfd90", tpl_17484::<0x54, 0x18, 0x20, 0x40, 0x140de82b0> as *const (), Throws::Discard), // sub_140dcfd90
    Replacement::draft_throws(0x140dcfde0, "content_140dcfde0", fn_140dcfde0 as *const (), Throws::Caught), // sub_140dcfde0
    Replacement::draft_throws(0x140dcff20, "content_140dcff20", tpl_6967::<0x20, 0x8, 0x140deae10> as *const (), Throws::Discard), // sub_140dcff20
    Replacement::draft_throws(0x140dcff70, "content_140dcff70", tpl_12664::<0x20, 0x8, 0x140deaee0> as *const (), Throws::Discard), // sub_140dcff70
    Replacement::draft_throws(0x140dcfff0, "content_140dcfff0", tpl_6985::<0x20, 0x8, 0x140dec300> as *const (), Throws::Discard), // sub_140dcfff0
    Replacement::draft_throws(0x140dd0070, "content_140dd0070", tpl_6967::<0x20, 0x8, 0x140ded740> as *const (), Throws::Discard), // sub_140dd0070
    Replacement::draft_throws(0x140dd00c0, "content_140dd00c0", fn_140dd00c0 as *const (), Throws::Caught), // sub_140dd00c0
    Replacement::draft_throws(0x140dd0270, "content_140dd0270", fn_140dd0270 as *const (), Throws::Caught), // sub_140dd0270
    Replacement::draft_throws(0x140dd0460, "content_140dd0460", fn_140dd0460 as *const (), Throws::Discard), // sub_140dd0460
    Replacement::draft_throws(0x140dd0530, "content_140dd0530", fn_140dd0530 as *const (), Throws::Caught), // sub_140dd0530
    Replacement::draft_throws(0x140dd06e0, "content_140dd06e0", fn_140dd06e0 as *const (), Throws::Caught), // sub_140dd06e0
    Replacement::draft_throws(0x140dd0950, "content_140dd0950", tpl_2502::<0x20, 0x8, 0x140df0ea0> as *const (), Throws::Discard), // sub_140dd0950
    Replacement::draft_throws(0x140dd09d0, "content_140dd09d0", tpl_13213::<0x20, 0x8, 0x140df2290> as *const (), Throws::Discard), // sub_140dd09d0
    Replacement::draft(0x140dd0a30, "content_140dd0a30", fn_140dd0a30 as *const ()), // sub_140dd0a30
    Replacement::draft(0x140dd0ab0, "content_140dd0ab0", fn_140dd0ab0 as *const ()), // sub_140dd0ab0
    Replacement::draft(0x140dd0b20, "content_140dd0b20", fn_140dd0b20 as *const ()), // sub_140dd0b20
    Replacement::draft(0x140dd0c00, "content_140dd0c00", fn_140dd0c00 as *const ()), // sub_140dd0c00
    Replacement::draft_throws(0x140dd0c70, "content_140dd0c70", fn_140dd0c70 as *const (), Throws::Caught), // sub_140dd0c70
    Replacement::draft_throws(0x140dd0ed0, "content_140dd0ed0", fn_140dd0ed0 as *const (), Throws::Caught), // sub_140dd0ed0
    Replacement::draft_throws(0x140dd1130, "content_140dd1130", fn_140dd1130 as *const (), Throws::Caught), // sub_140dd1130
    Replacement::draft_throws(0x140dd1390, "content_140dd1390", fn_140dd1390 as *const (), Throws::Caught), // sub_140dd1390
    Replacement::draft_throws(0x140dd15f0, "content_140dd15f0", fn_140dd15f0 as *const (), Throws::Caught), // sub_140dd15f0
    Replacement::draft(0x140dd1850, "content_140dd1850", tpl_17495::<0x143877920, 0x143264170, 0x1432a72f0, 0x1401de410> as *const ()), // sub_140dd1850
    Replacement::draft(0x140dd18a0, "content_140dd18a0", tpl_17495::<0x143878020, 0x143264170, 0x1432a72f0, 0x1401de410> as *const ()), // sub_140dd18a0
    Replacement::draft(0x140dd18f0, "content_140dd18f0", fn_140dd18f0 as *const ()), // sub_140dd18f0
    Replacement::draft(0x140dd1970, "content_140dd1970", fn_140dd1970 as *const ()), // sub_140dd1970
    Replacement::draft_throws(0x140dd1a50, "content_140dd1a50", tpl_838::<0x140df7600> as *const (), Throws::Discard), // sub_140dd1a50
    Replacement::draft_throws(0x140dd1a60, "content_140dd1a60", tpl_838::<0x140df8010> as *const (), Throws::Discard), // sub_140dd1a60
    Replacement::draft_throws(0x140dd1a80, "content_140dd1a80", tpl_12639::<0x20, 0x8, 0x140df8d50> as *const (), Throws::Discard), // sub_140dd1a80
    Replacement::draft(0x140dd1ae0, "content_140dd1ae0", fn_140dd1ae0 as *const ()), // sub_140dd1ae0
    Replacement::draft_throws(0x140dd1b50, "content_140dd1b50", fn_140dd1b50 as *const (), Throws::Caught), // sub_140dd1b50
    Replacement::draft_throws(0x140dd1d00, "content_140dd1d00", fn_140dd1d00 as *const (), Throws::Caught), // sub_140dd1d00
    Replacement::draft_throws(0x140dd1ef0, "content_140dd1ef0", fn_140dd1ef0 as *const (), Throws::Discard), // sub_140dd1ef0
    Replacement::draft(0x140dd1f40, "content_140dd1f40", fn_140dd1f40 as *const ()), // sub_140dd1f40
    Replacement::draft(0x140dd2090, "content_140dd2090", fn_140dd2090 as *const ()), // sub_140dd2090
    Replacement::draft(0x140dd2130, "content_140dd2130", fn_140dd2130 as *const ()), // sub_140dd2130
    Replacement::draft(0x140dd21b0, "content_140dd21b0", fn_140dd21b0 as *const ()), // sub_140dd21b0
    Replacement::draft(0x140dd2280, "content_140dd2280", fn_140dd2280 as *const ()), // sub_140dd2280
    Replacement::draft(0x140dd2360, "content_140dd2360", fn_140dd2360 as *const ()), // sub_140dd2360
    Replacement::draft(0x140dd2430, "content_140dd2430", fn_140dd2430 as *const ()), // sub_140dd2430
    Replacement::draft(0x140dd2530, "content_140dd2530", tpl_68695::<0x8, 0xc, 0x1408746b0, 0x14183acc2> as *const ()), // sub_140dd2530
    Replacement::draft(0x140dd25d0, "content_140dd25d0", tpl_17505::<0x8, 0x14240ab10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140dd25d0
    Replacement::draft(0x140dd26a0, "content_140dd26a0", fn_140dd26a0 as *const ()), // sub_140dd26a0
    Replacement::draft(0x140dd2780, "content_140dd2780", fn_140dd2780 as *const ()), // sub_140dd2780
    Replacement::draft(0x140dd2850, "content_140dd2850", fn_140dd2850 as *const ()), // sub_140dd2850
    Replacement::draft(0x140dd2930, "content_140dd2930", fn_140dd2930 as *const ()), // sub_140dd2930
    Replacement::draft(0x140dd2a10, "content_140dd2a10", fn_140dd2a10 as *const ()), // sub_140dd2a10
    Replacement::draft(0x140dd2af0, "content_140dd2af0", fn_140dd2af0 as *const ()), // sub_140dd2af0
    Replacement::draft(0x140dd2bb0, "content_140dd2bb0", fn_140dd2bb0 as *const ()), // sub_140dd2bb0
    Replacement::draft(0x140dd2cc0, "content_140dd2cc0", fn_140dd2cc0 as *const ()), // sub_140dd2cc0
    Replacement::draft(0x140dd2d90, "content_140dd2d90", fn_140dd2d90 as *const ()), // sub_140dd2d90
    Replacement::draft(0x140dd2e60, "content_140dd2e60", fn_140dd2e60 as *const ()), // sub_140dd2e60
    Replacement::draft(0x140dd2f30, "content_140dd2f30", fn_140dd2f30 as *const ()), // sub_140dd2f30
    Replacement::draft(0x140dd3060, "content_140dd3060", tpl_71136::<0x8, 0xc, 0x140e21010, 0x140a32d80, 0x140a32b30> as *const ()), // sub_140dd3060
    Replacement::draft(0x140dd3140, "content_140dd3140", fn_140dd3140 as *const ()), // sub_140dd3140
    Replacement::draft(0x140dd3220, "content_140dd3220", fn_140dd3220 as *const ()), // sub_140dd3220
    Replacement::draft(0x140dd3300, "content_140dd3300", fn_140dd3300 as *const ()), // sub_140dd3300
    Replacement::draft(0x140dd33d0, "content_140dd33d0", fn_140dd33d0 as *const ()), // sub_140dd33d0
    Replacement::draft(0x140dd3470, "content_140dd3470", fn_140dd3470 as *const ()), // sub_140dd3470
    Replacement::draft(0x140dd35a0, "content_140dd35a0", fn_140dd35a0 as *const ()), // sub_140dd35a0
    Replacement::draft(0x140dd3670, "content_140dd3670", fn_140dd3670 as *const ()), // sub_140dd3670
    Replacement::draft(0x140dd3750, "content_140dd3750", fn_140dd3750 as *const ()), // sub_140dd3750
    Replacement::draft(0x140dd3820, "content_140dd3820", fn_140dd3820 as *const ()), // sub_140dd3820
    Replacement::draft_throws(0x140dd3860, "content_140dd3860", fn_140dd3860 as *const (), Throws::Caught), // sub_140dd3860
    Replacement::draft_throws(0x140dd3bc0, "content_140dd3bc0", fn_140dd3bc0 as *const (), Throws::Caught), // sub_140dd3bc0
    Replacement::draft_throws(0x140dd3f20, "content_140dd3f20", fn_140dd3f20 as *const (), Throws::Caught), // sub_140dd3f20
    Replacement::draft_throws(0x140dd4420, "content_140dd4420", fn_140dd4420 as *const (), Throws::Caught), // sub_140dd4420
    Replacement::draft_throws(0x140dd48e0, "content_140dd48e0", fn_140dd48e0 as *const (), Throws::Caught), // sub_140dd48e0
    Replacement::draft_throws(0x140dd4d90, "content_140dd4d90", fn_140dd4d90 as *const (), Throws::Caught), // sub_140dd4d90
    Replacement::draft(0x140dd4e60, "content_140dd4e60", fn_140dd4e60 as *const ()), // sub_140dd4e60
    Replacement::draft_throws(0x140dd5040, "content_140dd5040", fn_140dd5040 as *const (), Throws::Caught), // sub_140dd5040
    Replacement::draft_throws(0x140dd5330, "content_140dd5330", fn_140dd5330 as *const (), Throws::Caught), // sub_140dd5330
    Replacement::draft_throws(0x140dd5620, "content_140dd5620", fn_140dd5620 as *const (), Throws::Caught), // sub_140dd5620
    Replacement::draft_throws(0x140dd5910, "content_140dd5910", fn_140dd5910 as *const (), Throws::Caught), // sub_140dd5910
    Replacement::draft_throws(0x140dd5c00, "content_140dd5c00", fn_140dd5c00 as *const (), Throws::Discard), // sub_140dd5c00
    Replacement::draft_throws(0x140dd5e30, "content_140dd5e30", fn_140dd5e30 as *const (), Throws::Caught), // sub_140dd5e30
    Replacement::draft_throws(0x140dd63b0, "content_140dd63b0", fn_140dd63b0 as *const (), Throws::Caught), // sub_140dd63b0
    Replacement::draft_throws(0x140dd6960, "content_140dd6960", fn_140dd6960 as *const (), Throws::Caught), // sub_140dd6960
    Replacement::draft_throws(0x140dd6df0, "content_140dd6df0", fn_140dd6df0 as *const (), Throws::Caught), // sub_140dd6df0
    Replacement::draft_throws(0x140dd7280, "content_140dd7280", fn_140dd7280 as *const (), Throws::Caught), // sub_140dd7280
    Replacement::draft_throws(0x140dd7920, "content_140dd7920", fn_140dd7920 as *const (), Throws::Caught), // sub_140dd7920
    Replacement::draft_throws(0x140dd7c00, "content_140dd7c00", fn_140dd7c00 as *const (), Throws::Caught), // sub_140dd7c00
    Replacement::draft_throws(0x140dd8150, "content_140dd8150", fn_140dd8150 as *const (), Throws::Caught), // sub_140dd8150
    Replacement::draft_throws(0x140dd86c0, "content_140dd86c0", fn_140dd86c0 as *const (), Throws::Caught), // sub_140dd86c0
    Replacement::draft_throws(0x140dd8b10, "content_140dd8b10", fn_140dd8b10 as *const (), Throws::Caught), // sub_140dd8b10
    Replacement::draft_throws(0x140dd8f60, "content_140dd8f60", fn_140dd8f60 as *const (), Throws::Caught), // sub_140dd8f60
    Replacement::draft_throws(0x140dd95b0, "content_140dd95b0", fn_140dd95b0 as *const (), Throws::Caught), // sub_140dd95b0
    Replacement::draft_throws(0x140dd9860, "content_140dd9860", fn_140dd9860 as *const (), Throws::Caught), // sub_140dd9860
    Replacement::draft_throws(0x140dd9ca0, "content_140dd9ca0", fn_140dd9ca0 as *const (), Throws::Caught), // sub_140dd9ca0
    Replacement::draft_throws(0x140dda0e0, "content_140dda0e0", fn_140dda0e0 as *const (), Throws::Caught), // sub_140dda0e0
    Replacement::draft_throws(0x140dda520, "content_140dda520", fn_140dda520 as *const (), Throws::Caught), // sub_140dda520
    Replacement::draft_throws(0x140dda960, "content_140dda960", fn_140dda960 as *const (), Throws::Caught), // sub_140dda960
    Replacement::draft_throws(0x140ddafb0, "content_140ddafb0", fn_140ddafb0 as *const (), Throws::Caught), // sub_140ddafb0
    Replacement::draft(0x140ddb250, "content_140ddb250", fn_140ddb250 as *const ()), // sub_140ddb250
    Replacement::draft_throws(0x140ddb440, "content_140ddb440", fn_140ddb440 as *const (), Throws::Caught), // sub_140ddb440
    Replacement::draft_throws(0x140ddb510, "content_140ddb510", fn_140ddb510 as *const (), Throws::Caught), // sub_140ddb510
    Replacement::draft_throws(0x140ddb5e0, "content_140ddb5e0", fn_140ddb5e0 as *const (), Throws::Caught), // sub_140ddb5e0
    Replacement::draft_throws(0x140ddb830, "content_140ddb830", fn_140ddb830 as *const (), Throws::Caught), // sub_140ddb830
    Replacement::draft(0x140ddb900, "content_140ddb900", tpl_17533::<0x143877eb0, 0x143264170, 0x1432a72f0, 0x1401de410> as *const ()), // sub_140ddb900
    Replacement::draft(0x140ddb950, "content_140ddb950", fn_140ddb950 as *const ()), // sub_140ddb950
    Replacement::draft(0x140ddb980, "content_140ddb980", tpl_17533::<0x1438777b8, 0x143264170, 0x1432a72f0, 0x1401de410> as *const ()), // sub_140ddb980
    Replacement::draft_throws(0x140ddb9e0, "content_140ddb9e0", fn_140ddb9e0 as *const (), Throws::Caught), // sub_140ddb9e0
    Replacement::draft_throws(0x140ddbca0, "content_140ddbca0", fn_140ddbca0 as *const (), Throws::Caught), // sub_140ddbca0
    Replacement::draft_throws(0x140ddbf60, "content_140ddbf60", fn_140ddbf60 as *const (), Throws::Caught), // sub_140ddbf60
    Replacement::draft_throws(0x140ddc3a0, "content_140ddc3a0", fn_140ddc3a0 as *const (), Throws::Caught), // sub_140ddc3a0
    Replacement::draft_throws(0x140ddc7b0, "content_140ddc7b0", fn_140ddc7b0 as *const (), Throws::Caught), // sub_140ddc7b0
    Replacement::draft_throws(0x140ddcbc0, "content_140ddcbc0", fn_140ddcbc0 as *const (), Throws::Caught), // sub_140ddcbc0
    Replacement::draft_throws(0x140ddcc90, "content_140ddcc90", fn_140ddcc90 as *const (), Throws::Caught), // sub_140ddcc90
    Replacement::draft_throws(0x140ddd210, "content_140ddd210", fn_140ddd210 as *const (), Throws::Caught), // sub_140ddd210
    Replacement::draft_throws(0x140ddd760, "content_140ddd760", fn_140ddd760 as *const (), Throws::Caught), // sub_140ddd760
    Replacement::draft_throws(0x140dddba0, "content_140dddba0", fn_140dddba0 as *const (), Throws::Caught), // sub_140dddba0
    Replacement::draft_throws(0x140dddeb0, "content_140dddeb0", fn_140dddeb0 as *const (), Throws::Caught), // sub_140dddeb0
    Replacement::draft_throws(0x140dde1c0, "content_140dde1c0", fn_140dde1c0 as *const (), Throws::Caught), // sub_140dde1c0
    Replacement::draft_throws(0x140dde660, "content_140dde660", fn_140dde660 as *const (), Throws::Caught), // sub_140dde660
    Replacement::draft_throws(0x140ddeac0, "content_140ddeac0", fn_140ddeac0 as *const (), Throws::Caught), // sub_140ddeac0
    Replacement::draft_throws(0x140ddef10, "content_140ddef10", fn_140ddef10 as *const (), Throws::Caught), // sub_140ddef10
    Replacement::draft_throws(0x140ddf020, "content_140ddf020", fn_140ddf020 as *const (), Throws::Caught), // sub_140ddf020
    Replacement::draft_throws(0x140ddf320, "content_140ddf320", fn_140ddf320 as *const (), Throws::Caught), // sub_140ddf320
    Replacement::draft_throws(0x140ddf620, "content_140ddf620", fn_140ddf620 as *const (), Throws::Caught), // sub_140ddf620
    Replacement::draft_throws(0x140ddfad0, "content_140ddfad0", fn_140ddfad0 as *const (), Throws::Caught), // sub_140ddfad0
    Replacement::draft_throws(0x140ddff50, "content_140ddff50", fn_140ddff50 as *const (), Throws::Caught), // sub_140ddff50
    Replacement::draft_throws(0x140de03c0, "content_140de03c0", fn_140de03c0 as *const (), Throws::Caught), // sub_140de03c0
    Replacement::draft_throws(0x140de0490, "content_140de0490", fn_140de0490 as *const (), Throws::Caught), // sub_140de0490
    Replacement::draft_throws(0x140de0900, "content_140de0900", fn_140de0900 as *const (), Throws::Caught), // sub_140de0900
    Replacement::draft_throws(0x140de0d70, "content_140de0d70", fn_140de0d70 as *const (), Throws::Caught), // sub_140de0d70
    Replacement::draft_throws(0x140de1380, "content_140de1380", fn_140de1380 as *const (), Throws::Caught), // sub_140de1380
    Replacement::draft_throws(0x140de1680, "content_140de1680", fn_140de1680 as *const (), Throws::Caught), // sub_140de1680
    Replacement::draft_throws(0x140de1c50, "content_140de1c50", fn_140de1c50 as *const (), Throws::Caught), // sub_140de1c50
    Replacement::draft_throws(0x140de1f10, "content_140de1f10", fn_140de1f10 as *const (), Throws::Caught), // sub_140de1f10
    Replacement::draft_throws(0x140de24c0, "content_140de24c0", fn_140de24c0 as *const (), Throws::Caught), // sub_140de24c0
    Replacement::draft_throws(0x140de2760, "content_140de2760", fn_140de2760 as *const (), Throws::Caught), // sub_140de2760
    Replacement::draft_throws(0x140de2830, "content_140de2830", fn_140de2830 as *const (), Throws::Caught), // sub_140de2830
    Replacement::draft(0x140de2900, "content_140de2900", tpl_17559::<0x1432f8c88, 0x1, 0x140dd1850> as *const ()), // sub_140de2900
    Replacement::draft(0x140de2930, "content_140de2930", tpl_17560::<0x143877f18, 0x1432f8c70, 0x1432f8cb8, 0x1401de410> as *const ()), // sub_140de2930
    Replacement::draft(0x140de2980, "content_140de2980", tpl_17559::<0x1432f8c88, 0x1, 0x140dd18a0> as *const ()), // sub_140de2980
    Replacement::draft(0x140de29b0, "content_140de29b0", tpl_17560::<0x1438778c8, 0x1432f8c70, 0x1432f8cb8, 0x1401de410> as *const ()), // sub_140de29b0
    Replacement::draft_throws(0x140de2a00, "content_140de2a00", fn_140de2a00 as *const (), Throws::Caught), // sub_140de2a00
    Replacement::draft_throws(0x140de2d10, "content_140de2d10", fn_140de2d10 as *const (), Throws::Caught), // sub_140de2d10
    Replacement::draft_throws(0x140de3020, "content_140de3020", fn_140de3020 as *const (), Throws::Caught), // sub_140de3020
    Replacement::draft_throws(0x140de3460, "content_140de3460", fn_140de3460 as *const (), Throws::Caught), // sub_140de3460
    Replacement::draft_throws(0x140de3870, "content_140de3870", fn_140de3870 as *const (), Throws::Caught), // sub_140de3870
    Replacement::draft_throws(0x140de3c80, "content_140de3c80", fn_140de3c80 as *const (), Throws::Caught), // sub_140de3c80
    Replacement::draft_throws(0x140de3d80, "content_140de3d80", fn_140de3d80 as *const (), Throws::Caught), // sub_140de3d80
    Replacement::draft_throws(0x140de4080, "content_140de4080", fn_140de4080 as *const (), Throws::Caught), // sub_140de4080
    Replacement::draft_throws(0x140de4380, "content_140de4380", fn_140de4380 as *const (), Throws::Caught), // sub_140de4380
    Replacement::draft_throws(0x140de4670, "content_140de4670", fn_140de4670 as *const (), Throws::Caught), // sub_140de4670
    Replacement::draft_throws(0x140de4960, "content_140de4960", fn_140de4960 as *const (), Throws::Caught), // sub_140de4960
    Replacement::draft_throws(0x140de4e10, "content_140de4e10", fn_140de4e10 as *const (), Throws::Caught), // sub_140de4e10
    Replacement::draft_throws(0x140de50d0, "content_140de50d0", fn_140de50d0 as *const (), Throws::Caught), // sub_140de50d0
    Replacement::draft_throws(0x140de5390, "content_140de5390", fn_140de5390 as *const (), Throws::Caught), // sub_140de5390
    Replacement::draft_throws(0x140de5800, "content_140de5800", fn_140de5800 as *const (), Throws::Caught), // sub_140de5800
    Replacement::draft_throws(0x140de5aa0, "content_140de5aa0", fn_140de5aa0 as *const (), Throws::Caught), // sub_140de5aa0
    Replacement::draft_throws(0x140de5d40, "content_140de5d40", fn_140de5d40 as *const (), Throws::Caught), // sub_140de5d40
    Replacement::draft_throws(0x140de61b0, "content_140de61b0", fn_140de61b0 as *const (), Throws::Caught), // sub_140de61b0
    Replacement::draft_throws(0x140de6280, "content_140de6280", fn_140de6280 as *const (), Throws::Caught), // sub_140de6280
    Replacement::draft_throws(0x140de6350, "content_140de6350", fn_140de6350 as *const (), Throws::Caught), // sub_140de6350
    Replacement::draft(0x140de6420, "content_140de6420", tpl_17575::<0x143877960, 0x1432f9ab8, 0x2, 0x1401de410> as *const ()), // sub_140de6420
    Replacement::draft(0x140de6460, "content_140de6460", tpl_17575::<0x1438779c0, 0x1432f9ab8, 0x2, 0x1401de410> as *const ()), // sub_140de6460
    Replacement::draft_throws(0x140de64a0, "content_140de64a0", fn_140de64a0 as *const (), Throws::Caught), // sub_140de64a0
    Replacement::draft_throws(0x140de67b0, "content_140de67b0", fn_140de67b0 as *const (), Throws::Caught), // sub_140de67b0
    Replacement::draft_throws(0x140de6ac0, "content_140de6ac0", fn_140de6ac0 as *const (), Throws::Caught), // sub_140de6ac0
    Replacement::draft_throws(0x140de6f00, "content_140de6f00", fn_140de6f00 as *const (), Throws::Caught), // sub_140de6f00
    Replacement::draft_throws(0x140de7300, "content_140de7300", fn_140de7300 as *const (), Throws::Caught), // sub_140de7300
    Replacement::draft_throws(0x140de7710, "content_140de7710", fn_140de7710 as *const (), Throws::Caught), // sub_140de7710
    Replacement::draft_throws(0x140de77e0, "content_140de77e0", fn_140de77e0 as *const (), Throws::Caught), // sub_140de77e0
    Replacement::draft_throws(0x140de7d60, "content_140de7d60", fn_140de7d60 as *const (), Throws::Caught), // sub_140de7d60
    Replacement::draft_throws(0x140de82b0, "content_140de82b0", fn_140de82b0 as *const (), Throws::Caught), // sub_140de82b0
    Replacement::draft_throws(0x140de86f0, "content_140de86f0", fn_140de86f0 as *const (), Throws::Caught), // sub_140de86f0
    Replacement::draft_throws(0x140de8a10, "content_140de8a10", fn_140de8a10 as *const (), Throws::Caught), // sub_140de8a10
    Replacement::draft_throws(0x140de8d10, "content_140de8d10", fn_140de8d10 as *const (), Throws::Caught), // sub_140de8d10
    Replacement::draft_throws(0x140de9030, "content_140de9030", fn_140de9030 as *const (), Throws::Caught), // sub_140de9030
    Replacement::draft_throws(0x140de9330, "content_140de9330", fn_140de9330 as *const (), Throws::Caught), // sub_140de9330
    Replacement::draft_throws(0x140de97d0, "content_140de97d0", fn_140de97d0 as *const (), Throws::Caught), // sub_140de97d0
    Replacement::draft_throws(0x140de9c80, "content_140de9c80", fn_140de9c80 as *const (), Throws::Caught), // sub_140de9c80
    Replacement::draft_throws(0x140dea0e0, "content_140dea0e0", fn_140dea0e0 as *const (), Throws::Caught), // sub_140dea0e0
    Replacement::draft_throws(0x140dea550, "content_140dea550", fn_140dea550 as *const (), Throws::Caught), // sub_140dea550
    Replacement::draft_throws(0x140dea9b0, "content_140dea9b0", fn_140dea9b0 as *const (), Throws::Caught), // sub_140dea9b0
    Replacement::draft_throws(0x140deae10, "content_140deae10", fn_140deae10 as *const (), Throws::Caught), // sub_140deae10
    Replacement::draft_throws(0x140deaee0, "content_140deaee0", fn_140deaee0 as *const (), Throws::Caught), // sub_140deaee0
    Replacement::draft_throws(0x140deafb0, "content_140deafb0", fn_140deafb0 as *const (), Throws::Caught), // sub_140deafb0
    Replacement::draft_throws(0x140deb2b0, "content_140deb2b0", fn_140deb2b0 as *const (), Throws::Caught), // sub_140deb2b0
    Replacement::draft_throws(0x140deb5b0, "content_140deb5b0", fn_140deb5b0 as *const (), Throws::Caught), // sub_140deb5b0
    Replacement::draft_throws(0x140deba50, "content_140deba50", fn_140deba50 as *const (), Throws::Caught), // sub_140deba50
    Replacement::draft_throws(0x140debeb0, "content_140debeb0", fn_140debeb0 as *const (), Throws::Caught), // sub_140debeb0
    Replacement::draft_throws(0x140dec300, "content_140dec300", fn_140dec300 as *const (), Throws::Caught), // sub_140dec300
    Replacement::draft_throws(0x140dec3d0, "content_140dec3d0", fn_140dec3d0 as *const (), Throws::Caught), // sub_140dec3d0
    Replacement::draft_throws(0x140dec6e0, "content_140dec6e0", fn_140dec6e0 as *const (), Throws::Caught), // sub_140dec6e0
    Replacement::draft_throws(0x140dec9f0, "content_140dec9f0", fn_140dec9f0 as *const (), Throws::Caught), // sub_140dec9f0
    Replacement::draft_throws(0x140dece90, "content_140dece90", fn_140dece90 as *const (), Throws::Caught), // sub_140dece90
    Replacement::draft_throws(0x140ded2f0, "content_140ded2f0", fn_140ded2f0 as *const (), Throws::Caught), // sub_140ded2f0
    Replacement::draft_throws(0x140ded740, "content_140ded740", fn_140ded740 as *const (), Throws::Caught), // sub_140ded740
    Replacement::draft_throws(0x140ded840, "content_140ded840", fn_140ded840 as *const (), Throws::Caught), // sub_140ded840
    Replacement::draft_throws(0x140dedb40, "content_140dedb40", fn_140dedb40 as *const (), Throws::Caught), // sub_140dedb40
    Replacement::draft_throws(0x140dede00, "content_140dede00", fn_140dede00 as *const (), Throws::Caught), // sub_140dede00
    Replacement::draft_throws(0x140dee0a0, "content_140dee0a0", fn_140dee0a0 as *const (), Throws::Caught), // sub_140dee0a0
    Replacement::draft_throws(0x140dee170, "content_140dee170", fn_140dee170 as *const (), Throws::Caught), // sub_140dee170
    Replacement::draft_throws(0x140dee430, "content_140dee430", fn_140dee430 as *const (), Throws::Caught), // sub_140dee430
    Replacement::draft_throws(0x140dee6f0, "content_140dee6f0", fn_140dee6f0 as *const (), Throws::Caught), // sub_140dee6f0
    Replacement::draft_throws(0x140deeb30, "content_140deeb30", fn_140deeb30 as *const (), Throws::Caught), // sub_140deeb30
    Replacement::draft_throws(0x140deef40, "content_140deef40", fn_140deef40 as *const (), Throws::Caught), // sub_140deef40
    Replacement::draft_throws(0x140def350, "content_140def350", fn_140def350 as *const (), Throws::Caught), // sub_140def350
    Replacement::draft_throws(0x140def420, "content_140def420", fn_140def420 as *const (), Throws::Caught), // sub_140def420
    Replacement::draft_throws(0x140def6f0, "content_140def6f0", fn_140def6f0 as *const (), Throws::Caught), // sub_140def6f0
    Replacement::draft_throws(0x140def990, "content_140def990", fn_140def990 as *const (), Throws::Caught), // sub_140def990
    Replacement::draft_throws(0x140defc20, "content_140defc20", fn_140defc20 as *const (), Throws::Caught), // sub_140defc20
    Replacement::draft_throws(0x140defcf0, "content_140defcf0", fn_140defcf0 as *const (), Throws::Caught), // sub_140defcf0
    Replacement::draft_throws(0x140deffb0, "content_140deffb0", fn_140deffb0 as *const (), Throws::Caught), // sub_140deffb0
    Replacement::draft_throws(0x140df0270, "content_140df0270", fn_140df0270 as *const (), Throws::Caught), // sub_140df0270
    Replacement::draft_throws(0x140df06a0, "content_140df06a0", fn_140df06a0 as *const (), Throws::Caught), // sub_140df06a0
    Replacement::draft_throws(0x140df0aa0, "content_140df0aa0", fn_140df0aa0 as *const (), Throws::Caught), // sub_140df0aa0
    Replacement::draft_throws(0x140df0ea0, "content_140df0ea0", fn_140df0ea0 as *const (), Throws::Caught), // sub_140df0ea0
    Replacement::draft_throws(0x140df0f70, "content_140df0f70", fn_140df0f70 as *const (), Throws::Caught), // sub_140df0f70
    Replacement::draft_throws(0x140df1270, "content_140df1270", fn_140df1270 as *const (), Throws::Caught), // sub_140df1270
    Replacement::draft_throws(0x140df1570, "content_140df1570", fn_140df1570 as *const (), Throws::Caught), // sub_140df1570
    Replacement::draft_throws(0x140df1a00, "content_140df1a00", fn_140df1a00 as *const (), Throws::Caught), // sub_140df1a00
    Replacement::draft_throws(0x140df1e50, "content_140df1e50", fn_140df1e50 as *const (), Throws::Caught), // sub_140df1e50
    Replacement::draft_throws(0x140df2290, "content_140df2290", fn_140df2290 as *const (), Throws::Caught), // sub_140df2290
    Replacement::draft_throws(0x140df2360, "content_140df2360", fn_140df2360 as *const (), Throws::Caught), // sub_140df2360
    Replacement::draft_throws(0x140df28e0, "content_140df28e0", fn_140df28e0 as *const (), Throws::Caught), // sub_140df28e0
    Replacement::draft_throws(0x140df2e30, "content_140df2e30", fn_140df2e30 as *const (), Throws::Caught), // sub_140df2e30
    Replacement::draft_throws(0x140df3270, "content_140df3270", fn_140df3270 as *const (), Throws::Caught), // sub_140df3270
    Replacement::draft_throws(0x140df3920, "content_140df3920", fn_140df3920 as *const (), Throws::Caught), // sub_140df3920
    Replacement::draft_throws(0x140df3f90, "content_140df3f90", fn_140df3f90 as *const (), Throws::Caught), // sub_140df3f90
    Replacement::draft_throws(0x140df43d0, "content_140df43d0", fn_140df43d0 as *const (), Throws::Caught), // sub_140df43d0
    Replacement::draft_throws(0x140df4950, "content_140df4950", fn_140df4950 as *const (), Throws::Caught), // sub_140df4950
    Replacement::draft_throws(0x140df4ea0, "content_140df4ea0", fn_140df4ea0 as *const (), Throws::Caught), // sub_140df4ea0
    Replacement::draft_throws(0x140df52e0, "content_140df52e0", fn_140df52e0 as *const (), Throws::Discard), // sub_140df52e0
    Replacement::draft_throws(0x140df54d0, "content_140df54d0", fn_140df54d0 as *const (), Throws::Caught), // sub_140df54d0
    Replacement::draft_throws(0x140df5b70, "content_140df5b70", fn_140df5b70 as *const (), Throws::Caught), // sub_140df5b70
    Replacement::draft_throws(0x140df61c0, "content_140df61c0", fn_140df61c0 as *const (), Throws::Caught), // sub_140df61c0
    Replacement::draft_throws(0x140df6810, "content_140df6810", fn_140df6810 as *const (), Throws::Caught), // sub_140df6810
    Replacement::draft_throws(0x140df6a60, "content_140df6a60", fn_140df6a60 as *const (), Throws::Caught), // sub_140df6a60
    Replacement::draft_throws(0x140df6d70, "content_140df6d70", fn_140df6d70 as *const (), Throws::Caught), // sub_140df6d70
    Replacement::draft_throws(0x140df7080, "content_140df7080", fn_140df7080 as *const (), Throws::Caught), // sub_140df7080
    Replacement::draft_throws(0x140df7600, "content_140df7600", fn_140df7600 as *const (), Throws::Caught), // sub_140df7600
    Replacement::draft_throws(0x140df7ac0, "content_140df7ac0", fn_140df7ac0 as *const (), Throws::Caught), // sub_140df7ac0
    Replacement::draft_throws(0x140df8010, "content_140df8010", fn_140df8010 as *const (), Throws::Caught), // sub_140df8010
    Replacement::draft_throws(0x140df8490, "content_140df8490", fn_140df8490 as *const (), Throws::Caught), // sub_140df8490
    Replacement::draft_throws(0x140df88d0, "content_140df88d0", fn_140df88d0 as *const (), Throws::Caught), // sub_140df88d0
    Replacement::draft_throws(0x140df8d50, "content_140df8d50", fn_140df8d50 as *const (), Throws::Caught), // sub_140df8d50
    Replacement::draft_throws(0x140df8e50, "content_140df8e50", fn_140df8e50 as *const (), Throws::Caught), // sub_140df8e50
    Replacement::draft_throws(0x140df9140, "content_140df9140", fn_140df9140 as *const (), Throws::Caught), // sub_140df9140
    Replacement::draft_throws(0x140df9400, "content_140df9400", fn_140df9400 as *const (), Throws::Caught), // sub_140df9400
    Replacement::draft_throws(0x140df96a0, "content_140df96a0", fn_140df96a0 as *const (), Throws::Caught), // sub_140df96a0
    Replacement::draft(0x140df9770, "content_140df9770", tpl_17618::<0x143877820, 0x1432f9680, 0x1432f9678, 0x1, 0x1401de410, 0x143877e20, 0x14325e2f8, 0x1432f9658> as *const ()), // sub_140df9770
    Replacement::draft(0x140df97e0, "content_140df97e0", tpl_17618::<0x143877710, 0x1432f9680, 0x1432f9678, 0x1, 0x1401de410, 0x143877d50, 0x14325e2f8, 0x1432f9658> as *const ()), // sub_140df97e0
    Replacement::draft_throws(0x140df9850, "content_140df9850", fn_140df9850 as *const (), Throws::Discard), // sub_140df9850
    Replacement::draft_throws(0x140df9a70, "content_140df9a70", fn_140df9a70 as *const (), Throws::Caught), // sub_140df9a70
    Replacement::draft_throws(0x140dfa110, "content_140dfa110", fn_140dfa110 as *const (), Throws::Caught), // sub_140dfa110
    Replacement::draft_throws(0x140dfa760, "content_140dfa760", fn_140dfa760 as *const (), Throws::Caught), // sub_140dfa760
    Replacement::draft_throws(0x140dfadb0, "content_140dfadb0", fn_140dfadb0 as *const (), Throws::Caught), // sub_140dfadb0
    Replacement::draft(0x140dfaf80, "content_140dfaf80", fn_140dfaf80 as *const ()), // sub_140dfaf80
    Replacement::draft(0x140dfb070, "content_140dfb070", tpl_4433::<0x19, 0x140dd2130, 0x20, 0x140dfb230, 0x10, 0x18, 0x8, 0x140dfb070> as *const ()), // sub_140dfb070
    Replacement::draft(0x140dfb130, "content_140dfb130", fn_140dfb130 as *const ()), // sub_140dfb130
    Replacement::draft(0x140dfb230, "content_140dfb230", fn_140dfb230 as *const ()), // sub_140dfb230
    Replacement::draft(0x140dfb2d0, "content_140dfb2d0", fn_140dfb2d0 as *const ()), // sub_140dfb2d0
    Replacement::draft(0x140dfb320, "content_140dfb320", fn_140dfb320 as *const ()), // sub_140dfb320
    Replacement::draft(0x140dfb380, "content_140dfb380", fn_140dfb380 as *const ()), // sub_140dfb380
    Replacement::draft(0x140dfb3c0, "content_140dfb3c0", tpl_1227::<0x8, 0xc, 0x14240ab10> as *const ()), // sub_140dfb3c0
    Replacement::draft(0x140dfb400, "content_140dfb400", fn_140dfb400 as *const ()), // sub_140dfb400
    Replacement::draft(0x140dfb450, "content_140dfb450", fn_140dfb450 as *const ()), // sub_140dfb450
    Replacement::draft(0x140dfb490, "content_140dfb490", fn_140dfb490 as *const ()), // sub_140dfb490
    Replacement::draft(0x140dfb4d0, "content_140dfb4d0", fn_140dfb4d0 as *const ()), // sub_140dfb4d0
    Replacement::draft(0x140dfb510, "content_140dfb510", fn_140dfb510 as *const ()), // sub_140dfb510
    Replacement::draft(0x140dfb550, "content_140dfb550", tpl_5949::<0x8, 0x14240ab10> as *const ()), // sub_140dfb550
    Replacement::draft(0x140dfb590, "content_140dfb590", fn_140dfb590 as *const ()), // sub_140dfb590
    Replacement::draft(0x140dfb5d0, "content_140dfb5d0", tpl_3502::<0x8, 0x4, 0x14240ab10> as *const ()), // sub_140dfb5d0
    Replacement::draft(0x140dfb610, "content_140dfb610", fn_140dfb610 as *const ()), // sub_140dfb610
    Replacement::draft(0x140dfb670, "content_140dfb670", fn_140dfb670 as *const ()), // sub_140dfb670
    Replacement::draft(0x140dfb6b0, "content_140dfb6b0", fn_140dfb6b0 as *const ()), // sub_140dfb6b0
    Replacement::draft(0x140dfb6f0, "content_140dfb6f0", fn_140dfb6f0 as *const ()), // sub_140dfb6f0
    Replacement::draft_throws(0x140dfb730, "content_140dfb730", fn_140dfb730 as *const (), Throws::Discard), // sub_140dfb730
    Replacement::draft_throws(0x140dfb890, "content_140dfb890", fn_140dfb890 as *const (), Throws::Discard), // sub_140dfb890
    Replacement::draft(0x140dfb9e0, "content_140dfb9e0", fn_140dfb9e0 as *const ()), // sub_140dfb9e0
    Replacement::draft(0x140dfba20, "content_140dfba20", fn_140dfba20 as *const ()), // sub_140dfba20
    Replacement::draft(0x140dfba80, "content_140dfba80", fn_140dfba80 as *const ()), // sub_140dfba80
    Replacement::draft(0x140dfbac0, "content_140dfbac0", fn_140dfbac0 as *const ()), // sub_140dfbac0
    Replacement::draft(0x140dfbb00, "content_140dfbb00", fn_140dfbb00 as *const ()), // sub_140dfbb00
    Replacement::draft(0x140dfbc80, "content_140dfbc80", fn_140dfbc80 as *const ()), // sub_140dfbc80
    Replacement::draft(0x140dfbde0, "content_140dfbde0", fn_140dfbde0 as *const ()), // sub_140dfbde0
    Replacement::draft(0x140dfbf30, "content_140dfbf30", fn_140dfbf30 as *const ()), // sub_140dfbf30
    Replacement::draft(0x140dfc370, "content_140dfc370", fn_140dfc370 as *const ()), // sub_140dfc370
    Replacement::draft(0x140dfc5f0, "content_140dfc5f0", fn_140dfc5f0 as *const ()), // sub_140dfc5f0
    Replacement::draft(0x140dfc840, "content_140dfc840", fn_140dfc840 as *const ()), // sub_140dfc840
    Replacement::draft(0x140dfcb10, "content_140dfcb10", fn_140dfcb10 as *const ()), // sub_140dfcb10
    Replacement::draft(0x140dfcdd0, "content_140dfcdd0", fn_140dfcdd0 as *const ()), // sub_140dfcdd0
    Replacement::draft(0x140dfd1a0, "content_140dfd1a0", fn_140dfd1a0 as *const ()), // sub_140dfd1a0
    Replacement::draft(0x140dfd470, "content_140dfd470", fn_140dfd470 as *const ()), // sub_140dfd470
    Replacement::draft(0x140dfd740, "content_140dfd740", fn_140dfd740 as *const ()), // sub_140dfd740
    Replacement::draft(0x140dfd9e0, "content_140dfd9e0", fn_140dfd9e0 as *const ()), // sub_140dfd9e0
    Replacement::draft(0x140dfdc90, "content_140dfdc90", fn_140dfdc90 as *const ()), // sub_140dfdc90
    Replacement::draft(0x140dfdf00, "content_140dfdf00", fn_140dfdf00 as *const ()), // sub_140dfdf00
    Replacement::draft(0x140dfe3f0, "content_140dfe3f0", fn_140dfe3f0 as *const ()), // sub_140dfe3f0
    Replacement::draft(0x140dfe640, "content_140dfe640", fn_140dfe640 as *const ()), // sub_140dfe640
    Replacement::draft(0x140dfe910, "content_140dfe910", fn_140dfe910 as *const ()), // sub_140dfe910
    Replacement::draft_throws(0x140dfecb0, "content_140dfecb0", fn_140dfecb0 as *const (), Throws::Caught), // sub_140dfecb0
    Replacement::draft_throws(0x140dff180, "content_140dff180", fn_140dff180 as *const (), Throws::Caught), // sub_140dff180
    Replacement::draft_throws(0x140dff610, "content_140dff610", fn_140dff610 as *const (), Throws::Caught), // sub_140dff610
    Replacement::draft(0x140dffaa0, "content_140dffaa0", fn_140dffaa0 as *const ()), // sub_140dffaa0
    Replacement::draft_throws(0x140dffb70, "content_140dffb70", fn_140dffb70 as *const (), Throws::Caught), // sub_140dffb70
    Replacement::draft_throws(0x140e00600, "content_140e00600", fn_140e00600 as *const (), Throws::Discard), // sub_140e00600
    Replacement::draft_throws(0x140e00ec0, "content_140e00ec0", fn_140e00ec0 as *const (), Throws::Caught), // sub_140e00ec0
    Replacement::draft_throws(0x140e018f0, "content_140e018f0", fn_140e018f0 as *const (), Throws::Caught), // sub_140e018f0
    Replacement::draft_throws(0x140e01dc0, "content_140e01dc0", fn_140e01dc0 as *const (), Throws::Caught), // sub_140e01dc0
    Replacement::draft_throws(0x140e02290, "content_140e02290", fn_140e02290 as *const (), Throws::Caught), // sub_140e02290
    Replacement::draft_throws(0x140e02c30, "content_140e02c30", fn_140e02c30 as *const (), Throws::Caught), // sub_140e02c30
    Replacement::draft_throws(0x140e03100, "content_140e03100", fn_140e03100 as *const (), Throws::Caught), // sub_140e03100
    Replacement::draft_throws(0x140e03590, "content_140e03590", fn_140e03590 as *const (), Throws::Caught), // sub_140e03590
    Replacement::draft_throws(0x140e03a20, "content_140e03a20", fn_140e03a20 as *const (), Throws::Caught), // sub_140e03a20
    Replacement::draft_throws(0x140e043c0, "content_140e043c0", fn_140e043c0 as *const (), Throws::Caught), // sub_140e043c0
    Replacement::draft_throws(0x140e04850, "content_140e04850", fn_140e04850 as *const (), Throws::Caught), // sub_140e04850
    Replacement::draft_throws(0x140e04ce0, "content_140e04ce0", fn_140e04ce0 as *const (), Throws::Caught), // sub_140e04ce0
    Replacement::draft_throws(0x140e05170, "content_140e05170", fn_140e05170 as *const (), Throws::Caught), // sub_140e05170
    Replacement::draft_throws(0x140e05b10, "content_140e05b10", fn_140e05b10 as *const (), Throws::Caught), // sub_140e05b10
    Replacement::draft_throws(0x140e05fa0, "content_140e05fa0", fn_140e05fa0 as *const (), Throws::Caught), // sub_140e05fa0
    Replacement::draft_throws(0x140e06470, "content_140e06470", fn_140e06470 as *const (), Throws::Caught), // sub_140e06470
    Replacement::draft_throws(0x140e068f0, "content_140e068f0", fn_140e068f0 as *const (), Throws::Caught), // sub_140e068f0
    Replacement::draft_throws(0x140e06d70, "content_140e06d70", fn_140e06d70 as *const (), Throws::Caught), // sub_140e06d70
    Replacement::draft_throws(0x140e07240, "content_140e07240", fn_140e07240 as *const (), Throws::Caught), // sub_140e07240
    Replacement::draft_throws(0x140e076d0, "content_140e076d0", fn_140e076d0 as *const (), Throws::Caught), // sub_140e076d0
    Replacement::draft_throws(0x140e07b60, "content_140e07b60", fn_140e07b60 as *const (), Throws::Caught), // sub_140e07b60
    Replacement::draft_throws(0x140e08030, "content_140e08030", fn_140e08030 as *const (), Throws::Caught), // sub_140e08030
    Replacement::draft_throws(0x140e084c0, "content_140e084c0", fn_140e084c0 as *const (), Throws::Caught), // sub_140e084c0
    Replacement::draft_throws(0x140e08950, "content_140e08950", fn_140e08950 as *const (), Throws::Caught), // sub_140e08950
    Replacement::draft_throws(0x140e08e20, "content_140e08e20", fn_140e08e20 as *const (), Throws::Caught), // sub_140e08e20
    Replacement::draft_throws(0x140e092f0, "content_140e092f0", fn_140e092f0 as *const (), Throws::Caught), // sub_140e092f0
    Replacement::draft_throws(0x140e09780, "content_140e09780", fn_140e09780 as *const (), Throws::Caught), // sub_140e09780
    Replacement::draft_throws(0x140e09c10, "content_140e09c10", fn_140e09c10 as *const (), Throws::Caught), // sub_140e09c10
    Replacement::draft_throws(0x140e0a0a0, "content_140e0a0a0", fn_140e0a0a0 as *const (), Throws::Caught), // sub_140e0a0a0
    Replacement::draft_throws(0x140e0a530, "content_140e0a530", fn_140e0a530 as *const (), Throws::Caught), // sub_140e0a530
    Replacement::draft_throws(0x140e0aa00, "content_140e0aa00", fn_140e0aa00 as *const (), Throws::Caught), // sub_140e0aa00
    Replacement::draft_throws(0x140e0ae80, "content_140e0ae80", fn_140e0ae80 as *const (), Throws::Caught), // sub_140e0ae80
    Replacement::draft_throws(0x140e0b300, "content_140e0b300", fn_140e0b300 as *const (), Throws::Caught), // sub_140e0b300
    Replacement::draft_throws(0x140e0b7d0, "content_140e0b7d0", fn_140e0b7d0 as *const (), Throws::Caught), // sub_140e0b7d0
    Replacement::draft_throws(0x140e0bca0, "content_140e0bca0", fn_140e0bca0 as *const (), Throws::Caught), // sub_140e0bca0
    Replacement::draft_throws(0x140e0c170, "content_140e0c170", fn_140e0c170 as *const (), Throws::Caught), // sub_140e0c170
    Replacement::draft_throws(0x140e0c600, "content_140e0c600", fn_140e0c600 as *const (), Throws::Caught), // sub_140e0c600
    Replacement::draft_throws(0x140e0ca90, "content_140e0ca90", fn_140e0ca90 as *const (), Throws::Caught), // sub_140e0ca90
    Replacement::draft_throws(0x140e0cf20, "content_140e0cf20", fn_140e0cf20 as *const (), Throws::Caught), // sub_140e0cf20
    Replacement::draft_throws(0x140e0d3b0, "content_140e0d3b0", fn_140e0d3b0 as *const (), Throws::Caught), // sub_140e0d3b0
    Replacement::draft_throws(0x140e0d840, "content_140e0d840", fn_140e0d840 as *const (), Throws::Caught), // sub_140e0d840
    Replacement::draft_throws(0x140e0dcd0, "content_140e0dcd0", fn_140e0dcd0 as *const (), Throws::Caught), // sub_140e0dcd0
    Replacement::draft_throws(0x140e0e1a0, "content_140e0e1a0", fn_140e0e1a0 as *const (), Throws::Caught), // sub_140e0e1a0
    Replacement::draft_throws(0x140e0e620, "content_140e0e620", fn_140e0e620 as *const (), Throws::Caught), // sub_140e0e620
    Replacement::draft_throws(0x140e0eaa0, "content_140e0eaa0", fn_140e0eaa0 as *const (), Throws::Caught), // sub_140e0eaa0
    Replacement::draft_throws(0x140e0ef70, "content_140e0ef70", fn_140e0ef70 as *const (), Throws::Caught), // sub_140e0ef70
    Replacement::draft_throws(0x140e0f440, "content_140e0f440", fn_140e0f440 as *const (), Throws::Caught), // sub_140e0f440
    Replacement::draft_throws(0x140e0f8d0, "content_140e0f8d0", fn_140e0f8d0 as *const (), Throws::Caught), // sub_140e0f8d0
    Replacement::draft_throws(0x140e0fd60, "content_140e0fd60", fn_140e0fd60 as *const (), Throws::Caught), // sub_140e0fd60
    Replacement::draft_throws(0x140e101f0, "content_140e101f0", fn_140e101f0 as *const (), Throws::Caught), // sub_140e101f0
    Replacement::draft_throws(0x140e10680, "content_140e10680", fn_140e10680 as *const (), Throws::Caught), // sub_140e10680
    Replacement::draft_throws(0x140e10b50, "content_140e10b50", fn_140e10b50 as *const (), Throws::Caught), // sub_140e10b50
    Replacement::draft_throws(0x140e10fe0, "content_140e10fe0", fn_140e10fe0 as *const (), Throws::Caught), // sub_140e10fe0
    Replacement::draft_throws(0x140e11470, "content_140e11470", fn_140e11470 as *const (), Throws::Caught), // sub_140e11470
    Replacement::draft_throws(0x140e11940, "content_140e11940", fn_140e11940 as *const (), Throws::Caught), // sub_140e11940
    Replacement::draft_throws(0x140e11dd0, "content_140e11dd0", fn_140e11dd0 as *const (), Throws::Caught), // sub_140e11dd0
    Replacement::draft_throws(0x140e12260, "content_140e12260", fn_140e12260 as *const (), Throws::Caught), // sub_140e12260
    Replacement::draft_throws(0x140e12730, "content_140e12730", fn_140e12730 as *const (), Throws::Caught), // sub_140e12730
    Replacement::draft_throws(0x140e12bc0, "content_140e12bc0", fn_140e12bc0 as *const (), Throws::Caught), // sub_140e12bc0
    Replacement::draft_throws(0x140e13050, "content_140e13050", fn_140e13050 as *const (), Throws::Caught), // sub_140e13050
    Replacement::draft_throws(0x140e13520, "content_140e13520", fn_140e13520 as *const (), Throws::Caught), // sub_140e13520
    Replacement::draft_throws(0x140e139a0, "content_140e139a0", fn_140e139a0 as *const (), Throws::Caught), // sub_140e139a0
    Replacement::draft_throws(0x140e13e20, "content_140e13e20", fn_140e13e20 as *const (), Throws::Caught), // sub_140e13e20
    Replacement::draft_throws(0x140e142f0, "content_140e142f0", fn_140e142f0 as *const (), Throws::Caught), // sub_140e142f0
    Replacement::draft_throws(0x140e14770, "content_140e14770", fn_140e14770 as *const (), Throws::Caught), // sub_140e14770
    Replacement::draft_throws(0x140e14bf0, "content_140e14bf0", fn_140e14bf0 as *const (), Throws::Caught), // sub_140e14bf0
    Replacement::draft_throws(0x140e150c0, "content_140e150c0", fn_140e150c0 as *const (), Throws::Caught), // sub_140e150c0
    Replacement::draft_throws(0x140e15540, "content_140e15540", fn_140e15540 as *const (), Throws::Caught), // sub_140e15540
    Replacement::draft_throws(0x140e159c0, "content_140e159c0", fn_140e159c0 as *const (), Throws::Caught), // sub_140e159c0
    Replacement::draft_throws(0x140e15e90, "content_140e15e90", fn_140e15e90 as *const (), Throws::Caught), // sub_140e15e90
    Replacement::draft_throws(0x140e16320, "content_140e16320", fn_140e16320 as *const (), Throws::Caught), // sub_140e16320
    Replacement::draft_throws(0x140e167b0, "content_140e167b0", fn_140e167b0 as *const (), Throws::Caught), // sub_140e167b0
    Replacement::draft_throws(0x140e17030, "content_140e17030", fn_140e17030 as *const (), Throws::Discard), // sub_140e17030
    Replacement::draft_throws(0x140e179e0, "content_140e179e0", fn_140e179e0 as *const (), Throws::Caught), // sub_140e179e0
    Replacement::draft_throws(0x140e182e0, "content_140e182e0", fn_140e182e0 as *const (), Throws::Caught), // sub_140e182e0
    Replacement::draft_throws(0x140e18d30, "content_140e18d30", fn_140e18d30 as *const (), Throws::Caught), // sub_140e18d30
    Replacement::draft_throws(0x140e19780, "content_140e19780", fn_140e19780 as *const (), Throws::Caught), // sub_140e19780
    Replacement::draft_throws(0x140e1a1d0, "content_140e1a1d0", fn_140e1a1d0 as *const (), Throws::Caught), // sub_140e1a1d0
    Replacement::draft_throws(0x140e1a6a0, "content_140e1a6a0", fn_140e1a6a0 as *const (), Throws::Caught), // sub_140e1a6a0
    Replacement::draft_throws(0x140e1ab30, "content_140e1ab30", fn_140e1ab30 as *const (), Throws::Caught), // sub_140e1ab30
    Replacement::draft_throws(0x140e1afc0, "content_140e1afc0", fn_140e1afc0 as *const (), Throws::Caught), // sub_140e1afc0
    Replacement::draft_throws(0x140e1b490, "content_140e1b490", fn_140e1b490 as *const (), Throws::Caught), // sub_140e1b490
    Replacement::draft_throws(0x140e1b920, "content_140e1b920", fn_140e1b920 as *const (), Throws::Caught), // sub_140e1b920
    Replacement::draft_throws(0x140e1bdb0, "content_140e1bdb0", fn_140e1bdb0 as *const (), Throws::Discard), // sub_140e1bdb0
    Replacement::draft_throws(0x140e1c6f0, "content_140e1c6f0", fn_140e1c6f0 as *const (), Throws::Discard), // sub_140e1c6f0
    Replacement::draft_throws(0x140e1d0e0, "content_140e1d0e0", fn_140e1d0e0 as *const (), Throws::Discard), // sub_140e1d0e0
    Replacement::draft_throws(0x140e1da60, "content_140e1da60", fn_140e1da60 as *const (), Throws::Caught), // sub_140e1da60
    Replacement::draft_throws(0x140e1e5b0, "content_140e1e5b0", fn_140e1e5b0 as *const (), Throws::Caught), // sub_140e1e5b0
    Replacement::draft_throws(0x140e1f100, "content_140e1f100", fn_140e1f100 as *const (), Throws::Caught), // sub_140e1f100
    Replacement::draft_throws(0x140e1fc50, "content_140e1fc50", fn_140e1fc50 as *const (), Throws::Discard), // sub_140e1fc50
    Replacement::draft(0x140e1fe20, "content_140e1fe20", fn_140e1fe20 as *const ()), // sub_140e1fe20
    Replacement::draft(0x140e20000, "content_140e20000", fn_140e20000 as *const ()), // sub_140e20000
    Replacement::draft(0x140e201e0, "content_140e201e0", fn_140e201e0 as *const ()), // sub_140e201e0
    Replacement::draft(0x140e20390, "content_140e20390", fn_140e20390 as *const ()), // sub_140e20390
    Replacement::draft(0x140e20540, "content_140e20540", fn_140e20540 as *const ()), // sub_140e20540
    Replacement::draft(0x140e20700, "content_140e20700", fn_140e20700 as *const ()), // sub_140e20700
    Replacement::draft(0x140e208c0, "content_140e208c0", fn_140e208c0 as *const ()), // sub_140e208c0
    Replacement::draft(0x140e20ae0, "content_140e20ae0", fn_140e20ae0 as *const ()), // sub_140e20ae0
    Replacement::draft(0x140e20ca0, "content_140e20ca0", fn_140e20ca0 as *const ()), // sub_140e20ca0
    Replacement::draft(0x140e20e60, "content_140e20e60", fn_140e20e60 as *const ()), // sub_140e20e60
    Replacement::draft(0x140e21010, "content_140e21010", fn_140e21010 as *const ()), // sub_140e21010
    Replacement::draft(0x140e211e0, "content_140e211e0", fn_140e211e0 as *const ()), // sub_140e211e0
    Replacement::draft(0x140e21390, "content_140e21390", fn_140e21390 as *const ()), // sub_140e21390
    Replacement::draft(0x140e215e0, "content_140e215e0", fn_140e215e0 as *const ()), // sub_140e215e0
    Replacement::draft(0x140e21780, "content_140e21780", fn_140e21780 as *const ()), // sub_140e21780
    Replacement::draft(0x140e21940, "content_140e21940", fn_140e21940 as *const ()), // sub_140e21940
    Replacement::draft_throws(0x140e21b30, "content_140e21b30", fn_140e21b30 as *const (), Throws::Caught), // sub_140e21b30
    Replacement::draft_throws(0x140e21e00, "content_140e21e00", fn_140e21e00 as *const (), Throws::Caught), // sub_140e21e00
    Replacement::draft_throws(0x140e220d0, "content_140e220d0", fn_140e220d0 as *const (), Throws::Caught), // sub_140e220d0
    Replacement::draft_throws(0x140e223a0, "content_140e223a0", fn_140e223a0 as *const (), Throws::Caught), // sub_140e223a0
    Replacement::draft_throws(0x140e22590, "content_140e22590", fn_140e22590 as *const (), Throws::Caught), // sub_140e22590
    Replacement::draft_throws(0x140e226d0, "content_140e226d0", fn_140e226d0 as *const (), Throws::Discard), // sub_140e226d0
    Replacement::draft_throws(0x140e228c0, "content_140e228c0", fn_140e228c0 as *const (), Throws::Caught), // sub_140e228c0
    Replacement::draft_throws(0x140e22a00, "content_140e22a00", fn_140e22a00 as *const (), Throws::Caught), // sub_140e22a00
    Replacement::draft_throws(0x140e22c60, "content_140e22c60", fn_140e22c60 as *const (), Throws::Caught), // sub_140e22c60
    Replacement::draft_throws(0x140e22ec0, "content_140e22ec0", fn_140e22ec0 as *const (), Throws::Caught), // sub_140e22ec0
    Replacement::draft_throws(0x140e23000, "content_140e23000", fn_140e23000 as *const (), Throws::Caught), // sub_140e23000
    Replacement::draft_throws(0x140e23260, "content_140e23260", fn_140e23260 as *const (), Throws::Caught), // sub_140e23260
    Replacement::draft_throws(0x140e234c0, "content_140e234c0", fn_140e234c0 as *const (), Throws::Caught), // sub_140e234c0
    Replacement::draft_throws(0x140e23600, "content_140e23600", fn_140e23600 as *const (), Throws::Caught), // sub_140e23600
    Replacement::draft_throws(0x140e23860, "content_140e23860", fn_140e23860 as *const (), Throws::Caught), // sub_140e23860
    Replacement::draft_throws(0x140e23ac0, "content_140e23ac0", fn_140e23ac0 as *const (), Throws::Caught), // sub_140e23ac0
    Replacement::draft_throws(0x140e23c00, "content_140e23c00", fn_140e23c00 as *const (), Throws::Caught), // sub_140e23c00
    Replacement::draft_throws(0x140e23e80, "content_140e23e80", fn_140e23e80 as *const (), Throws::Caught), // sub_140e23e80
    Replacement::draft_throws(0x140e24100, "content_140e24100", fn_140e24100 as *const (), Throws::Caught), // sub_140e24100
    Replacement::draft_throws(0x140e24350, "content_140e24350", fn_140e24350 as *const (), Throws::Caught), // sub_140e24350
    Replacement::draft_throws(0x140e245b0, "content_140e245b0", fn_140e245b0 as *const (), Throws::Caught), // sub_140e245b0
    Replacement::draft_throws(0x140e24800, "content_140e24800", fn_140e24800 as *const (), Throws::Caught), // sub_140e24800
    Replacement::draft_throws(0x140e24a50, "content_140e24a50", fn_140e24a50 as *const (), Throws::Caught), // sub_140e24a50
    Replacement::draft_throws(0x140e24cf0, "content_140e24cf0", fn_140e24cf0 as *const (), Throws::Caught), // sub_140e24cf0
    Replacement::draft_throws(0x140e24f70, "content_140e24f70", fn_140e24f70 as *const (), Throws::Caught), // sub_140e24f70
    Replacement::draft_throws(0x140e251e0, "content_140e251e0", fn_140e251e0 as *const (), Throws::Caught), // sub_140e251e0
    Replacement::draft_throws(0x140e25450, "content_140e25450", fn_140e25450 as *const (), Throws::Caught), // sub_140e25450
    Replacement::draft_throws(0x140e25640, "content_140e25640", fn_140e25640 as *const (), Throws::Caught), // sub_140e25640
    Replacement::draft_throws(0x140e258c0, "content_140e258c0", fn_140e258c0 as *const (), Throws::Caught), // sub_140e258c0
    Replacement::draft_throws(0x140e25b30, "content_140e25b30", fn_140e25b30 as *const (), Throws::Caught), // sub_140e25b30
    Replacement::draft_throws(0x140e25da0, "content_140e25da0", fn_140e25da0 as *const (), Throws::Caught), // sub_140e25da0
    Replacement::draft_throws(0x140e25f90, "content_140e25f90", fn_140e25f90 as *const (), Throws::Caught), // sub_140e25f90
    Replacement::draft_throws(0x140e262f0, "content_140e262f0", fn_140e262f0 as *const (), Throws::Caught), // sub_140e262f0
    Replacement::draft_throws(0x140e26430, "content_140e26430", fn_140e26430 as *const (), Throws::Caught), // sub_140e26430
    Replacement::draft_throws(0x140e26790, "content_140e26790", fn_140e26790 as *const (), Throws::Caught), // sub_140e26790
    Replacement::draft_throws(0x140e268d0, "content_140e268d0", fn_140e268d0 as *const (), Throws::Caught), // sub_140e268d0
    Replacement::draft_throws(0x140e26c30, "content_140e26c30", fn_140e26c30 as *const (), Throws::Caught), // sub_140e26c30
    Replacement::draft_throws(0x140e26d70, "content_140e26d70", fn_140e26d70 as *const (), Throws::Caught), // sub_140e26d70
    Replacement::draft_throws(0x140e26f60, "content_140e26f60", fn_140e26f60 as *const (), Throws::Caught), // sub_140e26f60
    Replacement::draft_throws(0x140e27150, "content_140e27150", fn_140e27150 as *const (), Throws::Caught), // sub_140e27150
    Replacement::draft_throws(0x140e273c0, "content_140e273c0", fn_140e273c0 as *const (), Throws::Caught), // sub_140e273c0
    Replacement::draft_throws(0x140e27620, "content_140e27620", fn_140e27620 as *const (), Throws::Caught), // sub_140e27620
    Replacement::draft_throws(0x140e27880, "content_140e27880", fn_140e27880 as *const (), Throws::Caught), // sub_140e27880
    Replacement::draft_throws(0x140e27ab0, "content_140e27ab0", fn_140e27ab0 as *const (), Throws::Caught), // sub_140e27ab0
    Replacement::draft_throws(0x140e27bf0, "content_140e27bf0", fn_140e27bf0 as *const (), Throws::Caught), // sub_140e27bf0
    Replacement::draft_throws(0x140e27d30, "content_140e27d30", fn_140e27d30 as *const (), Throws::Caught), // sub_140e27d30
    Replacement::draft_throws(0x140e27fb0, "content_140e27fb0", fn_140e27fb0 as *const (), Throws::Caught), // sub_140e27fb0
    Replacement::draft_throws(0x140e280f0, "content_140e280f0", fn_140e280f0 as *const (), Throws::Caught), // sub_140e280f0
    Replacement::draft_throws(0x140e28230, "content_140e28230", fn_140e28230 as *const (), Throws::Caught), // sub_140e28230
    Replacement::draft_throws(0x140e284b0, "content_140e284b0", fn_140e284b0 as *const (), Throws::Caught), // sub_140e284b0
    Replacement::draft_throws(0x140e285f0, "content_140e285f0", fn_140e285f0 as *const (), Throws::Caught), // sub_140e285f0
    Replacement::draft_throws(0x140e28730, "content_140e28730", fn_140e28730 as *const (), Throws::Caught), // sub_140e28730
    Replacement::draft_throws(0x140e289b0, "content_140e289b0", fn_140e289b0 as *const (), Throws::Caught), // sub_140e289b0
    Replacement::draft_throws(0x140e28ba0, "content_140e28ba0", fn_140e28ba0 as *const (), Throws::Caught), // sub_140e28ba0
    Replacement::draft_throws(0x140e28d90, "content_140e28d90", fn_140e28d90 as *const (), Throws::Caught), // sub_140e28d90
    Replacement::draft_throws(0x140e28f80, "content_140e28f80", fn_140e28f80 as *const (), Throws::Caught), // sub_140e28f80
    Replacement::draft_throws(0x140e291f0, "content_140e291f0", fn_140e291f0 as *const (), Throws::Caught), // sub_140e291f0
    Replacement::draft_throws(0x140e29450, "content_140e29450", fn_140e29450 as *const (), Throws::Caught), // sub_140e29450
    Replacement::draft_throws(0x140e296b0, "content_140e296b0", fn_140e296b0 as *const (), Throws::Caught), // sub_140e296b0
    Replacement::draft_throws(0x140e298e0, "content_140e298e0", fn_140e298e0 as *const (), Throws::Caught), // sub_140e298e0
    Replacement::draft_throws(0x140e29b70, "content_140e29b70", fn_140e29b70 as *const (), Throws::Caught), // sub_140e29b70
    Replacement::draft_throws(0x140e29df0, "content_140e29df0", fn_140e29df0 as *const (), Throws::Caught), // sub_140e29df0
    Replacement::draft_throws(0x140e2a070, "content_140e2a070", fn_140e2a070 as *const (), Throws::Caught), // sub_140e2a070
    Replacement::draft_throws(0x140e2a2f0, "content_140e2a2f0", fn_140e2a2f0 as *const (), Throws::Caught), // sub_140e2a2f0
    Replacement::draft_throws(0x140e2a570, "content_140e2a570", fn_140e2a570 as *const (), Throws::Caught), // sub_140e2a570
    Replacement::draft_throws(0x140e2a7f0, "content_140e2a7f0", fn_140e2a7f0 as *const (), Throws::Caught), // sub_140e2a7f0
    Replacement::draft_throws(0x140e2aa70, "content_140e2aa70", fn_140e2aa70 as *const (), Throws::Caught), // sub_140e2aa70
    Replacement::draft_throws(0x140e2ac60, "content_140e2ac60", fn_140e2ac60 as *const (), Throws::Caught), // sub_140e2ac60
    Replacement::draft_throws(0x140e2aed0, "content_140e2aed0", fn_140e2aed0 as *const (), Throws::Caught), // sub_140e2aed0
    Replacement::draft_throws(0x140e2b130, "content_140e2b130", fn_140e2b130 as *const (), Throws::Caught), // sub_140e2b130
    Replacement::draft_throws(0x140e2b390, "content_140e2b390", fn_140e2b390 as *const (), Throws::Caught), // sub_140e2b390
    Replacement::draft_throws(0x140e2b580, "content_140e2b580", fn_140e2b580 as *const (), Throws::Caught), // sub_140e2b580
    Replacement::draft_throws(0x140e2b800, "content_140e2b800", fn_140e2b800 as *const (), Throws::Caught), // sub_140e2b800
    Replacement::draft_throws(0x140e2ba70, "content_140e2ba70", fn_140e2ba70 as *const (), Throws::Caught), // sub_140e2ba70
    Replacement::draft_throws(0x140e2bce0, "content_140e2bce0", fn_140e2bce0 as *const (), Throws::Caught), // sub_140e2bce0
    Replacement::draft_throws(0x140e2bed0, "content_140e2bed0", fn_140e2bed0 as *const (), Throws::Caught), // sub_140e2bed0
    Replacement::draft_throws(0x140e2c010, "content_140e2c010", fn_140e2c010 as *const (), Throws::Caught), // sub_140e2c010
    Replacement::draft_throws(0x140e2c150, "content_140e2c150", fn_140e2c150 as *const (), Throws::Caught), // sub_140e2c150
    Replacement::draft_throws(0x140e2c290, "content_140e2c290", fn_140e2c290 as *const (), Throws::Caught), // sub_140e2c290
    Replacement::draft_throws(0x140e2c480, "content_140e2c480", fn_140e2c480 as *const (), Throws::Caught), // sub_140e2c480
    Replacement::draft_throws(0x140e2c6e0, "content_140e2c6e0", fn_140e2c6e0 as *const (), Throws::Caught), // sub_140e2c6e0
    Replacement::draft_throws(0x140e2c930, "content_140e2c930", fn_140e2c930 as *const (), Throws::Caught), // sub_140e2c930
    Replacement::draft_throws(0x140e2cb80, "content_140e2cb80", fn_140e2cb80 as *const (), Throws::Caught), // sub_140e2cb80
    Replacement::draft_throws(0x140e2ce20, "content_140e2ce20", fn_140e2ce20 as *const (), Throws::Caught), // sub_140e2ce20
    Replacement::draft_throws(0x140e2cf60, "content_140e2cf60", fn_140e2cf60 as *const (), Throws::Caught), // sub_140e2cf60
    Replacement::draft_throws(0x140e2d0a0, "content_140e2d0a0", fn_140e2d0a0 as *const (), Throws::Caught), // sub_140e2d0a0
    Replacement::draft_throws(0x140e2d1e0, "content_140e2d1e0", fn_140e2d1e0 as *const (), Throws::Caught), // sub_140e2d1e0
    Replacement::draft_throws(0x140e2d450, "content_140e2d450", fn_140e2d450 as *const (), Throws::Caught), // sub_140e2d450
    Replacement::draft_throws(0x140e2d6b0, "content_140e2d6b0", fn_140e2d6b0 as *const (), Throws::Caught), // sub_140e2d6b0
    Replacement::draft_throws(0x140e2d900, "content_140e2d900", fn_140e2d900 as *const (), Throws::Caught), // sub_140e2d900
    Replacement::draft_throws(0x140e2db50, "content_140e2db50", fn_140e2db50 as *const (), Throws::Caught), // sub_140e2db50
    Replacement::draft_throws(0x140e2ddb0, "content_140e2ddb0", fn_140e2ddb0 as *const (), Throws::Caught), // sub_140e2ddb0
    Replacement::draft_throws(0x140e2e030, "content_140e2e030", fn_140e2e030 as *const (), Throws::Caught), // sub_140e2e030
    Replacement::draft_throws(0x140e2e2a0, "content_140e2e2a0", fn_140e2e2a0 as *const (), Throws::Caught), // sub_140e2e2a0
    Replacement::draft_throws(0x140e2e510, "content_140e2e510", fn_140e2e510 as *const (), Throws::Caught), // sub_140e2e510
    Replacement::draft_throws(0x140e2e7a0, "content_140e2e7a0", fn_140e2e7a0 as *const (), Throws::Discard), // sub_140e2e7a0
    Replacement::draft_throws(0x140e2e920, "content_140e2e920", fn_140e2e920 as *const (), Throws::Caught), // sub_140e2e920
    Replacement::draft_throws(0x140e2eba0, "content_140e2eba0", fn_140e2eba0 as *const (), Throws::Caught), // sub_140e2eba0
    Replacement::draft_throws(0x140e2ee20, "content_140e2ee20", fn_140e2ee20 as *const (), Throws::Caught), // sub_140e2ee20
    Replacement::draft_throws(0x140e2f0a0, "content_140e2f0a0", fn_140e2f0a0 as *const (), Throws::Caught), // sub_140e2f0a0
    Replacement::draft_throws(0x140e2f290, "content_140e2f290", fn_140e2f290 as *const (), Throws::Caught), // sub_140e2f290
    Replacement::draft_throws(0x140e2f3d0, "content_140e2f3d0", fn_140e2f3d0 as *const (), Throws::Caught), // sub_140e2f3d0
    Replacement::draft_throws(0x140e2f510, "content_140e2f510", fn_140e2f510 as *const (), Throws::Caught), // sub_140e2f510
    Replacement::draft_throws(0x140e2f650, "content_140e2f650", fn_140e2f650 as *const (), Throws::Caught), // sub_140e2f650
    Replacement::draft_throws(0x140e2f840, "content_140e2f840", fn_140e2f840 as *const (), Throws::Discard), // sub_140e2f840
    Replacement::draft_throws(0x140e2f990, "content_140e2f990", fn_140e2f990 as *const (), Throws::Discard), // sub_140e2f990
    Replacement::draft_throws(0x140e2fa30, "content_140e2fa30", fn_140e2fa30 as *const (), Throws::Discard), // sub_140e2fa30
    Replacement::draft_throws(0x140e2faa0, "content_140e2faa0", fn_140e2faa0 as *const (), Throws::Discard), // sub_140e2faa0
    Replacement::draft_throws(0x140e2fb10, "content_140e2fb10", fn_140e2fb10 as *const (), Throws::Discard), // sub_140e2fb10
    Replacement::draft_throws(0x140e2fb70, "content_140e2fb70", fn_140e2fb70 as *const (), Throws::Caught), // sub_140e2fb70
    Replacement::draft_throws(0x140e30190, "content_140e30190", fn_140e30190 as *const (), Throws::Caught), // sub_140e30190
    Replacement::draft_throws(0x140e30780, "content_140e30780", fn_140e30780 as *const (), Throws::Caught), // sub_140e30780
    Replacement::draft_throws(0x140e30ad0, "content_140e30ad0", fn_140e30ad0 as *const (), Throws::Caught), // sub_140e30ad0
    Replacement::draft_throws(0x140e31090, "content_140e31090", fn_140e31090 as *const (), Throws::Caught), // sub_140e31090
    Replacement::draft_throws(0x140e31650, "content_140e31650", fn_140e31650 as *const (), Throws::Caught), // sub_140e31650
    Replacement::draft_throws(0x140e31790, "content_140e31790", fn_140e31790 as *const (), Throws::Caught), // sub_140e31790
    Replacement::draft_throws(0x140e31c00, "content_140e31c00", fn_140e31c00 as *const (), Throws::Caught), // sub_140e31c00
    Replacement::draft_throws(0x140e32190, "content_140e32190", fn_140e32190 as *const (), Throws::Caught), // sub_140e32190
    Replacement::draft_throws(0x140e32720, "content_140e32720", fn_140e32720 as *const (), Throws::Caught), // sub_140e32720
    Replacement::draft_throws(0x140e32860, "content_140e32860", fn_140e32860 as *const (), Throws::Caught), // sub_140e32860
    Replacement::draft_throws(0x140e32c90, "content_140e32c90", fn_140e32c90 as *const (), Throws::Caught), // sub_140e32c90
    Replacement::draft_throws(0x140e32fe0, "content_140e32fe0", fn_140e32fe0 as *const (), Throws::Caught), // sub_140e32fe0
    Replacement::draft_throws(0x140e33330, "content_140e33330", fn_140e33330 as *const (), Throws::Caught), // sub_140e33330
    Replacement::draft_throws(0x140e33470, "content_140e33470", fn_140e33470 as *const (), Throws::Caught), // sub_140e33470
    Replacement::draft_throws(0x140e337c0, "content_140e337c0", fn_140e337c0 as *const (), Throws::Caught), // sub_140e337c0
    Replacement::draft_throws(0x140e33d40, "content_140e33d40", fn_140e33d40 as *const (), Throws::Caught), // sub_140e33d40
    Replacement::draft_throws(0x140e34270, "content_140e34270", fn_140e34270 as *const (), Throws::Caught), // sub_140e34270
    Replacement::draft_throws(0x140e345c0, "content_140e345c0", fn_140e345c0 as *const (), Throws::Caught), // sub_140e345c0
    Replacement::draft_throws(0x140e34dd0, "content_140e34dd0", fn_140e34dd0 as *const (), Throws::Caught), // sub_140e34dd0
    Replacement::draft_throws(0x140e35560, "content_140e35560", fn_140e35560 as *const (), Throws::Caught), // sub_140e35560
    Replacement::draft_throws(0x140e35b00, "content_140e35b00", fn_140e35b00 as *const (), Throws::Caught), // sub_140e35b00
    Replacement::draft_throws(0x140e360d0, "content_140e360d0", fn_140e360d0 as *const (), Throws::Caught), // sub_140e360d0
    Replacement::draft_throws(0x140e36670, "content_140e36670", fn_140e36670 as *const (), Throws::Caught), // sub_140e36670
    Replacement::draft_throws(0x140e369c0, "content_140e369c0", fn_140e369c0 as *const (), Throws::Caught), // sub_140e369c0
    Replacement::draft_throws(0x140e37080, "content_140e37080", fn_140e37080 as *const (), Throws::Caught), // sub_140e37080
    Replacement::draft_throws(0x140e374f0, "content_140e374f0", fn_140e374f0 as *const (), Throws::Caught), // sub_140e374f0
    Replacement::draft_throws(0x140e37b80, "content_140e37b80", fn_140e37b80 as *const (), Throws::Caught), // sub_140e37b80
    Replacement::draft_throws(0x140e37fb0, "content_140e37fb0", fn_140e37fb0 as *const (), Throws::Caught), // sub_140e37fb0
    Replacement::draft_throws(0x140e38300, "content_140e38300", fn_140e38300 as *const (), Throws::Caught), // sub_140e38300
    Replacement::draft_throws(0x140e38650, "content_140e38650", fn_140e38650 as *const (), Throws::Caught), // sub_140e38650
    Replacement::draft_throws(0x140e38cc0, "content_140e38cc0", fn_140e38cc0 as *const (), Throws::Caught), // sub_140e38cc0
    Replacement::draft_throws(0x140e392f0, "content_140e392f0", fn_140e392f0 as *const (), Throws::Caught), // sub_140e392f0
    Replacement::draft_throws(0x140e39700, "content_140e39700", fn_140e39700 as *const (), Throws::Caught), // sub_140e39700
    Replacement::draft_throws(0x140e39b70, "content_140e39b70", fn_140e39b70 as *const (), Throws::Caught), // sub_140e39b70
    Replacement::draft_throws(0x140e39fe0, "content_140e39fe0", fn_140e39fe0 as *const (), Throws::Caught), // sub_140e39fe0
    Replacement::draft_throws(0x140e3a5c0, "content_140e3a5c0", fn_140e3a5c0 as *const (), Throws::Caught), // sub_140e3a5c0
    Replacement::draft_throws(0x140e3a9f0, "content_140e3a9f0", fn_140e3a9f0 as *const (), Throws::Caught), // sub_140e3a9f0
    Replacement::draft_throws(0x140e3ae20, "content_140e3ae20", fn_140e3ae20 as *const (), Throws::Caught), // sub_140e3ae20
    Replacement::draft_throws(0x140e3b3d0, "content_140e3b3d0", fn_140e3b3d0 as *const (), Throws::Caught), // sub_140e3b3d0
    Replacement::draft_throws(0x140e3b720, "content_140e3b720", fn_140e3b720 as *const (), Throws::Caught), // sub_140e3b720
    Replacement::draft_throws(0x140e3ba70, "content_140e3ba70", fn_140e3ba70 as *const (), Throws::Caught), // sub_140e3ba70
    Replacement::draft_throws(0x140e3bdc0, "content_140e3bdc0", fn_140e3bdc0 as *const (), Throws::Caught), // sub_140e3bdc0
    Replacement::draft_throws(0x140e3c430, "content_140e3c430", fn_140e3c430 as *const (), Throws::Caught), // sub_140e3c430
    Replacement::draft_throws(0x140e3ca60, "content_140e3ca60", fn_140e3ca60 as *const (), Throws::Caught), // sub_140e3ca60
    Replacement::draft_throws(0x140e3ce70, "content_140e3ce70", fn_140e3ce70 as *const (), Throws::Caught), // sub_140e3ce70
    Replacement::draft_throws(0x140e3d440, "content_140e3d440", fn_140e3d440 as *const (), Throws::Caught), // sub_140e3d440
    Replacement::draft_throws(0x140e3da20, "content_140e3da20", fn_140e3da20 as *const (), Throws::Caught), // sub_140e3da20
    Replacement::draft_throws(0x140e3dfc0, "content_140e3dfc0", fn_140e3dfc0 as *const (), Throws::Caught), // sub_140e3dfc0
    Replacement::draft_throws(0x140e3e570, "content_140e3e570", fn_140e3e570 as *const (), Throws::Caught), // sub_140e3e570
    Replacement::draft_throws(0x140e3e8c0, "content_140e3e8c0", fn_140e3e8c0 as *const (), Throws::Caught), // sub_140e3e8c0
    Replacement::draft_throws(0x140e3ec10, "content_140e3ec10", fn_140e3ec10 as *const (), Throws::Caught), // sub_140e3ec10
    Replacement::draft_throws(0x140e3f1e0, "content_140e3f1e0", fn_140e3f1e0 as *const (), Throws::Caught), // sub_140e3f1e0
    Replacement::draft_throws(0x140e3f770, "content_140e3f770", fn_140e3f770 as *const (), Throws::Caught), // sub_140e3f770
    Replacement::draft_throws(0x140e3fac0, "content_140e3fac0", fn_140e3fac0 as *const (), Throws::Caught), // sub_140e3fac0
    Replacement::draft_throws(0x140e402d0, "content_140e402d0", fn_140e402d0 as *const (), Throws::Caught), // sub_140e402d0
    Replacement::draft_throws(0x140e40a60, "content_140e40a60", fn_140e40a60 as *const (), Throws::Caught), // sub_140e40a60
    Replacement::draft_throws(0x140e41000, "content_140e41000", fn_140e41000 as *const (), Throws::Caught), // sub_140e41000
    Replacement::draft_throws(0x140e41470, "content_140e41470", fn_140e41470 as *const (), Throws::Caught), // sub_140e41470
    Replacement::draft_throws(0x140e418a0, "content_140e418a0", fn_140e418a0 as *const (), Throws::Caught), // sub_140e418a0
    Replacement::draft_throws(0x140e41bf0, "content_140e41bf0", fn_140e41bf0 as *const (), Throws::Caught), // sub_140e41bf0
    Replacement::draft_throws(0x140e42170, "content_140e42170", fn_140e42170 as *const (), Throws::Caught), // sub_140e42170
    Replacement::draft_throws(0x140e426a0, "content_140e426a0", fn_140e426a0 as *const (), Throws::Caught), // sub_140e426a0
    Replacement::draft_throws(0x140e429f0, "content_140e429f0", fn_140e429f0 as *const (), Throws::Caught), // sub_140e429f0
    Replacement::draft_throws(0x140e430f0, "content_140e430f0", fn_140e430f0 as *const (), Throws::Caught), // sub_140e430f0
    Replacement::draft_throws(0x140e43770, "content_140e43770", fn_140e43770 as *const (), Throws::Caught), // sub_140e43770
    Replacement::draft_throws(0x140e43ca0, "content_140e43ca0", fn_140e43ca0 as *const (), Throws::Caught), // sub_140e43ca0
    Replacement::draft_throws(0x140e44210, "content_140e44210", fn_140e44210 as *const (), Throws::Caught), // sub_140e44210
    Replacement::draft_throws(0x140e44740, "content_140e44740", fn_140e44740 as *const (), Throws::Caught), // sub_140e44740
    Replacement::draft_throws(0x140e44a90, "content_140e44a90", fn_140e44a90 as *const (), Throws::Caught), // sub_140e44a90
    Replacement::draft_throws(0x140e45070, "content_140e45070", fn_140e45070 as *const (), Throws::Caught), // sub_140e45070
    Replacement::draft_throws(0x140e45610, "content_140e45610", fn_140e45610 as *const (), Throws::Caught), // sub_140e45610
    Replacement::draft_throws(0x140e45960, "content_140e45960", fn_140e45960 as *const (), Throws::Caught), // sub_140e45960
    Replacement::draft_throws(0x140e45f40, "content_140e45f40", fn_140e45f40 as *const (), Throws::Caught), // sub_140e45f40
    Replacement::draft_throws(0x140e464f0, "content_140e464f0", fn_140e464f0 as *const (), Throws::Caught), // sub_140e464f0
    Replacement::draft_throws(0x140e46840, "content_140e46840", fn_140e46840 as *const (), Throws::Caught), // sub_140e46840
    Replacement::draft_throws(0x140e46cb0, "content_140e46cb0", fn_140e46cb0 as *const (), Throws::Caught), // sub_140e46cb0
    Replacement::draft_throws(0x140e470e0, "content_140e470e0", fn_140e470e0 as *const (), Throws::Caught), // sub_140e470e0
    Replacement::draft_throws(0x140e47430, "content_140e47430", fn_140e47430 as *const (), Throws::Caught), // sub_140e47430
    Replacement::draft_throws(0x140e47aa0, "content_140e47aa0", fn_140e47aa0 as *const (), Throws::Caught), // sub_140e47aa0
    Replacement::draft_throws(0x140e480d0, "content_140e480d0", fn_140e480d0 as *const (), Throws::Caught), // sub_140e480d0
    Replacement::draft(0x140e485b0, "content_140e485b0", fn_140e485b0 as *const ()), // sub_140e485b0
    Replacement::draft(0x140e48640, "content_140e48640", fn_140e48640 as *const ()), // sub_140e48640
    Replacement::draft(0x140e48670, "content_140e48670", fn_140e48670 as *const ()), // sub_140e48670
    Replacement::draft(0x140e48750, "content_140e48750", fn_140e48750 as *const ()), // sub_140e48750
    Replacement::draft(0x140e487c0, "content_140e487c0", fn_140e487c0 as *const ()), // sub_140e487c0
    Replacement::draft(0x140e48990, "content_140e48990", fn_140e48990 as *const ()), // sub_140e48990
    Replacement::draft(0x140e48a50, "content_140e48a50", fn_140e48a50 as *const ()), // sub_140e48a50
    Replacement::draft(0x140e48ac0, "content_140e48ac0", fn_140e48ac0 as *const ()), // sub_140e48ac0
    Replacement::draft(0x140e48b30, "content_140e48b30", fn_140e48b30 as *const ()), // sub_140e48b30
    Replacement::draft(0x140e48bf0, "content_140e48bf0", fn_140e48bf0 as *const ()), // sub_140e48bf0
    Replacement::draft(0x140e48c60, "content_140e48c60", fn_140e48c60 as *const ()), // sub_140e48c60
    Replacement::draft(0x140e48cd0, "content_140e48cd0", fn_140e48cd0 as *const ()), // sub_140e48cd0
    Replacement::draft_throws(0x140e48d80, "content_140e48d80", fn_140e48d80 as *const (), Throws::Discard), // sub_140e48d80
    Replacement::draft(0x140e48e70, "content_140e48e70", fn_140e48e70 as *const ()), // sub_140e48e70
    Replacement::draft_throws(0x140e48f10, "content_140e48f10", fn_140e48f10 as *const (), Throws::Discard), // sub_140e48f10
    Replacement::draft(0x140e48fb0, "content_140e48fb0", fn_140e48fb0 as *const ()), // sub_140e48fb0
    Replacement::draft_throws(0x140e49010, "content_140e49010", fn_140e49010 as *const (), Throws::Caught), // sub_140e49010
    Replacement::draft_throws(0x140e49350, "content_140e49350", fn_140e49350 as *const (), Throws::Caught), // sub_140e49350
    Replacement::draft_throws(0x140e49660, "content_140e49660", fn_140e49660 as *const (), Throws::Caught), // sub_140e49660
    Replacement::draft(0x140e49750, "content_140e49750", fn_140e49750 as *const ()), // sub_140e49750
    Replacement::draft(0x140e497f0, "content_140e497f0", fn_140e497f0 as *const ()), // sub_140e497f0
    Replacement::draft(0x140e49870, "content_140e49870", fn_140e49870 as *const ()), // sub_140e49870
    Replacement::draft_throws(0x140e49900, "content_140e49900", fn_140e49900 as *const (), Throws::Discard), // sub_140e49900
    Replacement::draft_throws(0x140e49990, "content_140e49990", fn_140e49990 as *const (), Throws::Caught), // sub_140e49990
    Replacement::draft(0x140e49ad0, "content_140e49ad0", fn_140e49ad0 as *const ()), // sub_140e49ad0
    Replacement::draft(0x140e49b70, "content_140e49b70", fn_140e49b70 as *const ()), // sub_140e49b70
    Replacement::draft(0x140e49c30, "content_140e49c30", fn_140e49c30 as *const ()), // sub_140e49c30
    Replacement::draft_throws(0x140e49d80, "content_140e49d80", fn_140e49d80 as *const (), Throws::Discard), // sub_140e49d80
    Replacement::draft(0x140e49ea0, "content_140e49ea0", fn_140e49ea0 as *const ()), // sub_140e49ea0
    Replacement::draft_throws(0x140e49f40, "content_140e49f40", fn_140e49f40 as *const (), Throws::Discard), // sub_140e49f40
    Replacement::draft_throws(0x140e4a070, "content_140e4a070", fn_140e4a070 as *const (), Throws::Caught), // sub_140e4a070
    Replacement::draft_throws(0x140e4a1b0, "content_140e4a1b0", fn_140e4a1b0 as *const (), Throws::Caught), // sub_140e4a1b0
    Replacement::draft_throws(0x140e4a2f0, "content_140e4a2f0", fn_140e4a2f0 as *const (), Throws::Caught), // sub_140e4a2f0
    Replacement::draft(0x140e4a430, "content_140e4a430", fn_140e4a430 as *const ()), // sub_140e4a430
    Replacement::draft(0x140e4a500, "content_140e4a500", fn_140e4a500 as *const ()), // sub_140e4a500
    Replacement::draft(0x140e4a5a0, "content_140e4a5a0", fn_140e4a5a0 as *const ()), // sub_140e4a5a0
    Replacement::draft_throws(0x140e4a650, "content_140e4a650", fn_140e4a650 as *const (), Throws::Discard), // sub_140e4a650
    Replacement::draft_throws(0x140e4a700, "content_140e4a700", fn_140e4a700 as *const (), Throws::Discard), // sub_140e4a700
    Replacement::draft(0x140e4a840, "content_140e4a840", fn_140e4a840 as *const ()), // sub_140e4a840
    Replacement::draft(0x140e4a8e0, "content_140e4a8e0", fn_140e4a8e0 as *const ()), // sub_140e4a8e0
    Replacement::draft(0x140e4a990, "content_140e4a990", fn_140e4a990 as *const ()), // sub_140e4a990
    Replacement::draft_throws(0x140e4ac30, "content_140e4ac30", fn_140e4ac30 as *const (), Throws::Discard), // sub_140e4ac30
    Replacement::draft(0x140e4ae10, "content_140e4ae10", fn_140e4ae10 as *const ()), // sub_140e4ae10
    Replacement::draft_throws(0x140e4af20, "content_140e4af20", fn_140e4af20 as *const (), Throws::Caught), // sub_140e4af20
    Replacement::draft(0x140e4b130, "content_140e4b130", fn_140e4b130 as *const ()), // sub_140e4b130
    Replacement::draft_throws(0x140e4b1f0, "content_140e4b1f0", fn_140e4b1f0 as *const (), Throws::Caught), // sub_140e4b1f0
    Replacement::draft_throws(0x140e4b320, "content_140e4b320", fn_140e4b320 as *const (), Throws::Caught), // sub_140e4b320
    Replacement::draft_throws(0x140e4b450, "content_140e4b450", fn_140e4b450 as *const (), Throws::Caught), // sub_140e4b450
    Replacement::draft(0x140e4b580, "content_140e4b580", fn_140e4b580 as *const ()), // sub_140e4b580
    Replacement::draft(0x140e4b700, "content_140e4b700", fn_140e4b700 as *const ()), // sub_140e4b700
    Replacement::draft(0x140e4b800, "content_140e4b800", fn_140e4b800 as *const ()), // sub_140e4b800
    Replacement::draft_throws(0x140e4b910, "content_140e4b910", fn_140e4b910 as *const (), Throws::Discard), // sub_140e4b910
    Replacement::draft_throws(0x140e4ba10, "content_140e4ba10", fn_140e4ba10 as *const (), Throws::Discard), // sub_140e4ba10
    Replacement::draft(0x140e4bb40, "content_140e4bb40", fn_140e4bb40 as *const ()), // sub_140e4bb40
    Replacement::draft(0x140e4bbf0, "content_140e4bbf0", fn_140e4bbf0 as *const ()), // sub_140e4bbf0
    Replacement::draft(0x140e4bd20, "content_140e4bd20", fn_140e4bd20 as *const ()), // sub_140e4bd20
    Replacement::draft(0x140e4bda0, "content_140e4bda0", tpl_17815::<0x14387fbd0, 0x14243b0b0, 0x14028a610, 0x18, 0x140e59e80, 0x28, 0x38, 0x48, 0x58, 0x70, 0x88, 0xa0> as *const ()), // sub_140e4bda0
    Replacement::draft_throws(0x140e4be70, "content_140e4be70", fn_140e4be70 as *const (), Throws::Discard), // sub_140e4be70
    Replacement::draft(0x140e4c0c0, "content_140e4c0c0", fn_140e4c0c0 as *const ()), // sub_140e4c0c0
    Replacement::draft_throws(0x140e4c200, "content_140e4c200", fn_140e4c200 as *const (), Throws::Discard), // sub_140e4c200
    Replacement::draft(0x140e4c2c0, "content_140e4c2c0", fn_140e4c2c0 as *const ()), // sub_140e4c2c0
    Replacement::draft_throws(0x140e4c470, "content_140e4c470", fn_140e4c470 as *const (), Throws::Caught), // sub_140e4c470
    Replacement::draft_throws(0x140e4c5c0, "content_140e4c5c0", fn_140e4c5c0 as *const (), Throws::Caught), // sub_140e4c5c0
    Replacement::draft_throws(0x140e4c710, "content_140e4c710", fn_140e4c710 as *const (), Throws::Caught), // sub_140e4c710
    Replacement::draft(0x140e4c860, "content_140e4c860", tpl_17815::<0x14387fbd0, 0x14243b0b0, 0x1409fd120, 0x18, 0x140e5a6b0, 0x28, 0x38, 0x48, 0x58, 0x70, 0x88, 0xa0> as *const ()), // sub_140e4c860
    Replacement::draft(0x140e4c930, "content_140e4c930", fn_140e4c930 as *const ()), // sub_140e4c930
    Replacement::draft(0x140e4ca20, "content_140e4ca20", fn_140e4ca20 as *const ()), // sub_140e4ca20
    Replacement::draft_throws(0x140e4cb20, "content_140e4cb20", fn_140e4cb20 as *const (), Throws::Discard), // sub_140e4cb20
    Replacement::draft_throws(0x140e4cd70, "content_140e4cd70", fn_140e4cd70 as *const (), Throws::Discard), // sub_140e4cd70
    Replacement::draft(0x140e4cf20, "content_140e4cf20", fn_140e4cf20 as *const ()), // sub_140e4cf20
    Replacement::draft(0x140e4d040, "content_140e4d040", fn_140e4d040 as *const ()), // sub_140e4d040
    Replacement::draft_throws(0x140e4d2b0, "content_140e4d2b0", fn_140e4d2b0 as *const (), Throws::Discard), // sub_140e4d2b0
    Replacement::draft(0x140e4d490, "content_140e4d490", fn_140e4d490 as *const ()), // sub_140e4d490
    Replacement::draft_throws(0x140e4d5d0, "content_140e4d5d0", fn_140e4d5d0 as *const (), Throws::Caught), // sub_140e4d5d0
    Replacement::draft(0x140e4d870, "content_140e4d870", fn_140e4d870 as *const ()), // sub_140e4d870
    Replacement::draft_throws(0x140e4d940, "content_140e4d940", fn_140e4d940 as *const (), Throws::Caught), // sub_140e4d940
    Replacement::draft_throws(0x140e4da00, "content_140e4da00", fn_140e4da00 as *const (), Throws::Caught), // sub_140e4da00
    Replacement::draft_throws(0x140e4dac0, "content_140e4dac0", fn_140e4dac0 as *const (), Throws::Caught), // sub_140e4dac0
    Replacement::draft(0x140e4db80, "content_140e4db80", fn_140e4db80 as *const ()), // sub_140e4db80
    Replacement::draft(0x140e4dd70, "content_140e4dd70", fn_140e4dd70 as *const ()), // sub_140e4dd70
    Replacement::draft(0x140e4de80, "content_140e4de80", fn_140e4de80 as *const ()), // sub_140e4de80
    Replacement::draft_throws(0x140e4dfa0, "content_140e4dfa0", fn_140e4dfa0 as *const (), Throws::Discard), // sub_140e4dfa0
    Replacement::draft_throws(0x140e4e0a0, "content_140e4e0a0", fn_140e4e0a0 as *const (), Throws::Discard), // sub_140e4e0a0
    Replacement::draft(0x140e4e160, "content_140e4e160", fn_140e4e160 as *const ()), // sub_140e4e160
    Replacement::draft(0x140e4e230, "content_140e4e230", fn_140e4e230 as *const ()), // sub_140e4e230
    Replacement::draft(0x140e4e320, "content_140e4e320", tpl_17836::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x140e5a8d0> as *const ()), // sub_140e4e320
    Replacement::draft(0x140e4e3a0, "content_140e4e3a0", fn_140e4e3a0 as *const ()), // sub_140e4e3a0
    Replacement::draft(0x140e4e480, "content_140e4e480", fn_140e4e480 as *const ()), // sub_140e4e480
    Replacement::draft(0x140e4ea70, "content_140e4ea70", fn_140e4ea70 as *const ()), // sub_140e4ea70
    Replacement::draft_throws(0x140e4ec10, "content_140e4ec10", fn_140e4ec10 as *const (), Throws::Caught), // sub_140e4ec10
    Replacement::draft(0x140e4f020, "content_140e4f020", fn_140e4f020 as *const ()), // sub_140e4f020
    Replacement::draft_throws(0x140e4f130, "content_140e4f130", fn_140e4f130 as *const (), Throws::Caught), // sub_140e4f130
    Replacement::draft_throws(0x140e4f8a0, "content_140e4f8a0", fn_140e4f8a0 as *const (), Throws::Caught), // sub_140e4f8a0
    Replacement::draft_throws(0x140e4ffb0, "content_140e4ffb0", fn_140e4ffb0 as *const (), Throws::Caught), // sub_140e4ffb0
    Replacement::draft(0x140e503b0, "content_140e503b0", fn_140e503b0 as *const ()), // sub_140e503b0
    Replacement::draft(0x140e50600, "content_140e50600", fn_140e50600 as *const ()), // sub_140e50600
    Replacement::draft(0x140e507c0, "content_140e507c0", fn_140e507c0 as *const ()), // sub_140e507c0
    Replacement::draft_throws(0x140e50910, "content_140e50910", fn_140e50910 as *const (), Throws::Discard), // sub_140e50910
    Replacement::draft_throws(0x140e50aa0, "content_140e50aa0", fn_140e50aa0 as *const (), Throws::Discard), // sub_140e50aa0
    Replacement::draft(0x140e50d50, "content_140e50d50", fn_140e50d50 as *const ()), // sub_140e50d50
    Replacement::draft(0x140e50e00, "content_140e50e00", fn_140e50e00 as *const ()), // sub_140e50e00
    Replacement::draft(0x140e50ea0, "content_140e50ea0", tpl_16694::<0x1432f9e10, 0x48, 0x10, 0x140947eb0, 0x60, 0x1432f9dd8, 0xb> as *const ()), // sub_140e50ea0
    Replacement::draft(0x140e50f10, "content_140e50f10", fn_140e50f10 as *const ()), // sub_140e50f10
    Replacement::draft(0x140e50fb0, "content_140e50fb0", fn_140e50fb0 as *const ()), // sub_140e50fb0
    Replacement::draft(0x140e51060, "content_140e51060", tpl_17854::<0x14387fb50, 0x14243b0b0, 0x1426d4580> as *const ()), // sub_140e51060
    Replacement::draft_throws(0x140e512a0, "content_140e512a0", fn_140e512a0 as *const (), Throws::Discard), // sub_140e512a0
    Replacement::draft(0x140e51440, "content_140e51440", fn_140e51440 as *const ()), // sub_140e51440
    Replacement::draft_throws(0x140e51550, "content_140e51550", fn_140e51550 as *const (), Throws::Caught), // sub_140e51550
    Replacement::draft(0x140e51730, "content_140e51730", tpl_17858::<0x8, 0x1401e84a0, 0x140e5b1d0> as *const ()), // sub_140e51730
    Replacement::draft_throws(0x140e517c0, "content_140e517c0", fn_140e517c0 as *const (), Throws::Caught), // sub_140e517c0
    Replacement::draft_throws(0x140e519e0, "content_140e519e0", fn_140e519e0 as *const (), Throws::Caught), // sub_140e519e0
    Replacement::draft_throws(0x140e51bd0, "content_140e51bd0", fn_140e51bd0 as *const (), Throws::Caught), // sub_140e51bd0
    Replacement::draft(0x140e51dc0, "content_140e51dc0", fn_140e51dc0 as *const ()), // sub_140e51dc0
    Replacement::draft(0x140e51f30, "content_140e51f30", fn_140e51f30 as *const ()), // sub_140e51f30
    Replacement::draft(0x140e52010, "content_140e52010", fn_140e52010 as *const ()), // sub_140e52010
    Replacement::draft_throws(0x140e52100, "content_140e52100", tpl_17863::<0x14387fbe0, 0x14387fb50, 0x1401e9770, 0x338, 0x120, 0x8, 0x28, 0x20, 0x18, 0x1401e9590> as *const (), Throws::Discard), // sub_140e52100
    Replacement::draft_throws(0x140e521e0, "content_140e521e0", fn_140e521e0 as *const (), Throws::Discard), // sub_140e521e0
    Replacement::draft(0x140e522d0, "content_140e522d0", tpl_17865::<0x1432f9eb0, 0x8, 0x140947eb0, 0x28, 0x20, 0x1432f9e68> as *const ()), // sub_140e522d0
    Replacement::draft(0x140e52360, "content_140e52360", fn_140e52360 as *const ()), // sub_140e52360
    Replacement::draft_throws(0x140e52430, "content_140e52430", fn_140e52430 as *const (), Throws::Discard), // sub_140e52430
    Replacement::draft(0x140e524e0, "content_140e524e0", tpl_76215::<0x14387fbe8, 0x14243b0b0, 0x338, 0x40, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e524e0
    Replacement::draft_throws(0x140e52560, "content_140e52560", fn_140e52560 as *const (), Throws::Discard), // sub_140e52560
    Replacement::draft_throws(0x140e52630, "content_140e52630", fn_140e52630 as *const (), Throws::Caught), // sub_140e52630
    Replacement::draft_throws(0x140e52730, "content_140e52730", fn_140e52730 as *const (), Throws::Caught), // sub_140e52730
    Replacement::draft_throws(0x140e52830, "content_140e52830", fn_140e52830 as *const (), Throws::Caught), // sub_140e52830
    Replacement::draft(0x140e52930, "content_140e52930", fn_140e52930 as *const ()), // sub_140e52930
    Replacement::draft(0x140e529c0, "content_140e529c0", fn_140e529c0 as *const ()), // sub_140e529c0
    Replacement::draft(0x140e52a40, "content_140e52a40", tpl_76215::<0x14387fbe8, 0x14243b0b0, 0x338, 0xe0, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e52a40
    Replacement::draft_throws(0x140e52ad0, "content_140e52ad0", fn_140e52ad0 as *const (), Throws::Discard), // sub_140e52ad0
    Replacement::draft_throws(0x140e52b60, "content_140e52b60", fn_140e52b60 as *const (), Throws::Discard), // sub_140e52b60
    Replacement::draft(0x140e52c50, "content_140e52c50", fn_140e52c50 as *const ()), // sub_140e52c50
    Replacement::draft_throws(0x140e52d80, "content_140e52d80", fn_140e52d80 as *const (), Throws::Discard), // sub_140e52d80
    Replacement::draft(0x140e52e70, "content_140e52e70", fn_140e52e70 as *const ()), // sub_140e52e70
    Replacement::draft_throws(0x140e52f00, "content_140e52f00", fn_140e52f00 as *const (), Throws::Caught), // sub_140e52f00
    Replacement::draft_throws(0x140e53060, "content_140e53060", fn_140e53060 as *const (), Throws::Caught), // sub_140e53060
    Replacement::draft_throws(0x140e53390, "content_140e53390", fn_140e53390 as *const (), Throws::Caught), // sub_140e53390
    Replacement::draft_throws(0x140e53680, "content_140e53680", fn_140e53680 as *const (), Throws::Caught), // sub_140e53680
    Replacement::draft(0x140e53760, "content_140e53760", fn_140e53760 as *const ()), // sub_140e53760
    Replacement::draft(0x140e53870, "content_140e53870", tpl_9437::<0x14387fbc8, 0x14387fb70, 0x1401e9970, 0x338, 0x88, 0x18, 0xd8, 0x1401e9860> as *const ()), // sub_140e53870
    Replacement::draft(0x140e538e0, "content_140e538e0", tpl_9506::<0x14387fb70, 0x14243b0b0, 0x338, 0x88, 0x18, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e538e0
    Replacement::draft_throws(0x140e53960, "content_140e53960", fn_140e53960 as *const (), Throws::Discard), // sub_140e53960
    Replacement::draft_throws(0x140e539f0, "content_140e539f0", fn_140e539f0 as *const (), Throws::Discard), // sub_140e539f0
    Replacement::draft(0x140e53ae0, "content_140e53ae0", fn_140e53ae0 as *const ()), // sub_140e53ae0
    Replacement::draft(0x140e53ba0, "content_140e53ba0", fn_140e53ba0 as *const ()), // sub_140e53ba0
    Replacement::draft(0x140e53c20, "content_140e53c20", tpl_12891::<0x14387fbf0, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x110, 0x10b, 0x10c, 0x108, 0xfffffffffffffff8> as *const ()), // sub_140e53c20
];
