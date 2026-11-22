#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1412871a0, "unknown_1412871a0", fn_1412871a0 as *const ()), // sub_1412871a0
    Replacement::draft(0x141287250, "unknown_141287250", fn_141287250 as *const ()), // sub_141287250
    Replacement::draft_throws(0x141287410, "unknown_141287410", fn_141287410 as *const (), Throws::Discard), // sub_141287410
    Replacement::draft(0x141287580, "unknown_141287580", fn_141287580 as *const ()), // sub_141287580
    Replacement::draft_throws(0x141287680, "unknown_141287680", fn_141287680 as *const (), Throws::Discard), // sub_141287680
    Replacement::draft_throws(0x141287820, "unknown_141287820", fn_141287820 as *const (), Throws::Caught), // sub_141287820
    Replacement::draft_throws(0x141287c70, "unknown_141287c70", fn_141287c70 as *const (), Throws::Caught), // sub_141287c70
    Replacement::draft_throws(0x141288010, "unknown_141288010", fn_141288010 as *const (), Throws::Caught), // sub_141288010
    Replacement::draft(0x1412883b0, "unknown_1412883b0", fn_1412883b0 as *const ()), // sub_1412883b0
    Replacement::draft(0x1412884b0, "unknown_1412884b0", fn_1412884b0 as *const ()), // sub_1412884b0
    Replacement::draft(0x1412885c0, "unknown_1412885c0", fn_1412885c0 as *const ()), // sub_1412885c0
    Replacement::draft_throws(0x1412886d0, "unknown_1412886d0", fn_1412886d0 as *const (), Throws::Discard), // sub_1412886d0
    Replacement::draft_throws(0x141288800, "unknown_141288800", fn_141288800 as *const (), Throws::Discard), // sub_141288800
    Replacement::draft(0x141288980, "unknown_141288980", fn_141288980 as *const ()), // sub_141288980
    Replacement::draft(0x141288a30, "unknown_141288a30", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433e6648, 0x8> as *const ()), // sub_141288a30
    Replacement::draft(0x141288ab0, "unknown_141288ab0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433e6648, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_141288ab0
    Replacement::draft_throws(0x141288b50, "unknown_141288b50", fn_141288b50 as *const (), Throws::Caught), // sub_141288b50
    Replacement::draft_throws(0x141288be0, "unknown_141288be0", tpl_1908::<0x143881748, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_141288be0
    Replacement::draft(0x141288c60, "unknown_141288c60", fn_141288c60 as *const ()), // sub_141288c60
    Replacement::draft(0x141288d00, "unknown_141288d00", fn_141288d00 as *const ()), // sub_141288d00
    Replacement::draft(0x141288db0, "unknown_141288db0", tpl_6135::<0x143881740, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_141288db0
    Replacement::draft_throws(0x141288e50, "unknown_141288e50", tpl_6136::<0x143881760, 0x143881740, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_141288e50
    Replacement::draft_throws(0x141288ed0, "unknown_141288ed0", tpl_6136::<0x143881760, 0x143881740, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_141288ed0
    Replacement::draft_throws(0x141288f50, "unknown_141288f50", fn_141288f50 as *const (), Throws::Caught), // sub_141288f50
    Replacement::draft(0x141288fe0, "unknown_141288fe0", tpl_6137::<0x143881760, 0x143881740, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_141288fe0
    Replacement::draft_throws(0x141289060, "unknown_141289060", tpl_6136::<0x143881760, 0x143881740, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_141289060
    Replacement::draft_throws(0x1412890e0, "unknown_1412890e0", tpl_1905::<0x143881740, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412890e0
    Replacement::draft(0x141289190, "physx_141289190", tpl_1674::<0x1433e6be0> as *const ()), // sub_141289190
    Replacement::draft(0x1412891a0, "physx_1412891a0", tpl_893::<0x1433e6be0, 0x141834d30> as *const ()), // physx::PxConstraintConnector::physx::PxConstraintConnector::vfunc_9
    Replacement::draft(0x1412891d0, "physx_1412891d0", tpl_1674::<0x1433e6c90> as *const ()), // sub_1412891d0
    Replacement::draft(0x1412891e0, "physx_1412891e0", tpl_893::<0x1433e6c90, 0x141834d30> as *const ()), // physx::PxDeletionListener::physx::PxDeletionListener::vfunc_1
    Replacement::draft(0x141289210, "unknown_141289210", fn_141289210 as *const ()), // sub_141289210
    Replacement::draft(0x141289410, "physics_141289410", fn_141289410 as *const ()), // physics::BodyPushBridge::physics::BodyPushBridge::vfunc_2
    Replacement::draft(0x141289450, "physics_141289450", fn_141289450 as *const ()), // physics::BodyPushBridge::physics::BodyPushBridge::vfunc_5
    Replacement::draft(0x141289460, "physics_141289460", fn_141289460 as *const ()), // physics::BodyPushBridge::physics::BodyPushBridge::vfunc_9
    Replacement::draft(0x1412894c0, "physics_1412894c0", fn_1412894c0 as *const ()), // physics::CharacterRigidBodyPusher::physics::CharacterRigidBodyPusher::vfunc_1
    Replacement::draft(0x1412895b0, "unknown_1412895b0", fn_1412895b0 as *const ()), // sub_1412895b0
    Replacement::draft_throws(0x1412896c0, "unknown_1412896c0", fn_1412896c0 as *const (), Throws::Discard), // sub_1412896c0
    Replacement::draft_throws(0x141289740, "unknown_141289740", tpl_23196::<0x10, 0x14241c190, 0x8, 0xc> as *const (), Throws::Discard), // sub_141289740
    Replacement::draft_throws(0x141289780, "unknown_141289780", tpl_23196::<0x10, 0x14241c190, 0x8, 0xc> as *const (), Throws::Discard), // sub_141289780
    Replacement::draft_throws(0x1412897c0, "unknown_1412897c0", fn_1412897c0 as *const (), Throws::Discard), // sub_1412897c0
    Replacement::draft_throws(0x141289800, "unknown_141289800", fn_141289800 as *const (), Throws::Discard), // sub_141289800
    Replacement::draft(0x1412898e0, "unknown_1412898e0", fn_1412898e0 as *const ()), // sub_1412898e0
    Replacement::draft(0x141289c20, "unknown_141289c20", fn_141289c20 as *const ()), // sub_141289c20
    Replacement::draft_throws(0x141289c30, "unknown_141289c30", fn_141289c30 as *const (), Throws::Discard), // sub_141289c30
    Replacement::draft_throws(0x141289cb0, "unknown_141289cb0", fn_141289cb0 as *const (), Throws::Discard), // sub_141289cb0
    Replacement::draft_throws(0x141289de0, "unknown_141289de0", fn_141289de0 as *const (), Throws::Discard), // sub_141289de0
    Replacement::draft(0x141289e90, "unknown_141289e90", fn_141289e90 as *const ()), // sub_141289e90
    Replacement::draft(0x14128a280, "unknown_14128a280", fn_14128a280 as *const ()), // sub_14128a280
    Replacement::draft(0x14128a3f0, "unknown_14128a3f0", fn_14128a3f0 as *const ()), // sub_14128a3f0
    Replacement::draft(0x14128a600, "unknown_14128a600", fn_14128a600 as *const ()), // sub_14128a600
    Replacement::draft(0x14128a830, "unknown_14128a830", fn_14128a830 as *const ()), // sub_14128a830
    Replacement::draft(0x14128ab40, "unknown_14128ab40", fn_14128ab40 as *const ()), // sub_14128ab40
    Replacement::draft(0x14128ad20, "unknown_14128ad20", fn_14128ad20 as *const ()), // sub_14128ad20
    Replacement::draft(0x14128adf0, "unknown_14128adf0", fn_14128adf0 as *const ()), // sub_14128adf0
    Replacement::draft(0x14128afc0, "unknown_14128afc0", fn_14128afc0 as *const ()), // sub_14128afc0
    Replacement::draft(0x14128b0a0, "unknown_14128b0a0", fn_14128b0a0 as *const ()), // sub_14128b0a0
    Replacement::draft(0x14128b260, "unknown_14128b260", fn_14128b260 as *const ()), // sub_14128b260
    Replacement::draft(0x14128b330, "unknown_14128b330", fn_14128b330 as *const ()), // sub_14128b330
    Replacement::draft(0x14128b410, "unknown_14128b410", fn_14128b410 as *const ()), // sub_14128b410
    Replacement::draft(0x14128b750, "unknown_14128b750", fn_14128b750 as *const ()), // sub_14128b750
    Replacement::draft(0x14128bc80, "unknown_14128bc80", fn_14128bc80 as *const ()), // sub_14128bc80
    Replacement::draft(0x14128bf30, "unknown_14128bf30", fn_14128bf30 as *const ()), // sub_14128bf30
    Replacement::draft(0x14128c1d0, "unknown_14128c1d0", fn_14128c1d0 as *const ()), // sub_14128c1d0
    Replacement::draft(0x14128c340, "physx_14128c340", tpl_893::<0x1433e6f80, 0x141834d30> as *const ()), // physx::PxUserControllerHitReport::physx::PxUserControllerHitReport::vfunc_3
    Replacement::draft(0x14128c370, "physx_14128c370", tpl_893::<0x1433e6ee8, 0x141834d30> as *const ()), // physx::PxControllerBehaviorCallback::physx::PxControllerBehaviorCallback::vfunc_3
    Replacement::draft(0x14128c3a0, "physics_14128c3a0", fn_14128c3a0 as *const ()), // physics::PhysXFilter::physics::PhysXFilter::vfunc_0
    Replacement::draft(0x14128c4f0, "physics_14128c4f0", fn_14128c4f0 as *const ()), // physics::ControllerHitReport::physics::ControllerHitReport::vfunc_3
    Replacement::draft(0x14128c540, "physics_14128c540", fn_14128c540 as *const ()), // physics::ControllerBehaviorCallback::physics::ControllerBehaviorCallback::vfunc_2
    Replacement::draft(0x14128c5d0, "physics_14128c5d0", fn_14128c5d0 as *const ()), // physics::ControllerBehaviorCallback::physics::ControllerBehaviorCallback::vfunc_1
    Replacement::draft(0x14128c5e0, "physics_14128c5e0", fn_14128c5e0 as *const ()), // physics::ControllerBehaviorCallback::physics::ControllerBehaviorCallback::vfunc_0
    Replacement::draft(0x14128c600, "physics_14128c600", fn_14128c600 as *const ()), // sub_14128c600
    Replacement::draft(0x14128c680, "physics_14128c680", fn_14128c680 as *const ()), // sub_14128c680
    Replacement::draft(0x14128c760, "physics_14128c760", fn_14128c760 as *const ()), // sub_14128c760
    Replacement::draft(0x14128c800, "physics_14128c800", fn_14128c800 as *const ()), // sub_14128c800
    Replacement::draft(0x14128c8b0, "physics_14128c8b0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433e6ff0, 0x8> as *const ()), // sub_14128c8b0
    Replacement::draft(0x14128c930, "physics_14128c930", fn_14128c930 as *const ()), // sub_14128c930
    Replacement::draft(0x14128ca70, "unknown_14128ca70", fn_14128ca70 as *const ()), // sub_14128ca70
    Replacement::draft(0x14128cae0, "physx_14128cae0", fn_14128cae0 as *const ()), // sub_14128cae0
    Replacement::draft(0x14128cb30, "physics_14128cb30", tpl_19913::<0x8, 0x38, 0x20, 0x1433e4400, 0x141834d30> as *const ()), // physics::PhysXFilter::physics::PhysXFilter::vfunc_2
    Replacement::draft(0x14128cba0, "unknown_14128cba0", fn_14128cba0 as *const ()), // sub_14128cba0
    Replacement::draft(0x14128cd20, "unknown_14128cd20", tpl_5539::<0x8, 0x14022a610, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14128cd20
    Replacement::draft(0x14128cdf0, "unknown_14128cdf0", tpl_21238::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1434281d0, 0x20, 0x28, 0x30> as *const ()), // sub_14128cdf0
    Replacement::draft(0x14128ce80, "unknown_14128ce80", fn_14128ce80 as *const ()), // sub_14128ce80
    Replacement::draft(0x14128cf30, "unknown_14128cf30", fn_14128cf30 as *const ()), // sub_14128cf30
    Replacement::draft_throws(0x14128cff0, "unknown_14128cff0", fn_14128cff0 as *const (), Throws::Discard), // sub_14128cff0
    Replacement::draft_throws(0x14128d0f0, "unknown_14128d0f0", fn_14128d0f0 as *const (), Throws::Discard), // sub_14128d0f0
    Replacement::draft_throws(0x14128d1a0, "unknown_14128d1a0", fn_14128d1a0 as *const (), Throws::Discard), // sub_14128d1a0
    Replacement::draft_throws(0x14128d2b0, "unknown_14128d2b0", fn_14128d2b0 as *const (), Throws::Caught), // sub_14128d2b0
    Replacement::draft_throws(0x14128d3f0, "unknown_14128d3f0", fn_14128d3f0 as *const (), Throws::Caught), // sub_14128d3f0
    Replacement::draft_throws(0x14128d530, "unknown_14128d530", fn_14128d530 as *const (), Throws::Caught), // sub_14128d530
    Replacement::draft(0x14128d670, "unknown_14128d670", fn_14128d670 as *const ()), // sub_14128d670
    Replacement::draft(0x14128d730, "unknown_14128d730", fn_14128d730 as *const ()), // sub_14128d730
    Replacement::draft(0x14128d7f0, "unknown_14128d7f0", fn_14128d7f0 as *const ()), // sub_14128d7f0
    Replacement::draft_throws(0x14128d8c0, "unknown_14128d8c0", fn_14128d8c0 as *const (), Throws::Discard), // sub_14128d8c0
    Replacement::draft_throws(0x14128d9a0, "unknown_14128d9a0", fn_14128d9a0 as *const (), Throws::Discard), // sub_14128d9a0
    Replacement::draft(0x14128dac0, "unknown_14128dac0", fn_14128dac0 as *const ()), // sub_14128dac0
    Replacement::draft_throws(0x14128dc90, "unknown_14128dc90", fn_14128dc90 as *const (), Throws::Discard), // sub_14128dc90
    Replacement::draft_throws(0x14128dcd0, "unknown_14128dcd0", fn_14128dcd0 as *const (), Throws::Discard), // sub_14128dcd0
    Replacement::draft_throws(0x14128dd10, "unknown_14128dd10", fn_14128dd10 as *const (), Throws::Discard), // sub_14128dd10
    Replacement::draft(0x14128de70, "physics_14128de70", fn_14128de70 as *const ()), // sub_14128de70
    Replacement::draft(0x14128df10, "physics_14128df10", tpl_18006::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1433e7058, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_14128df10
    Replacement::draft(0x14128dfd0, "physics_14128dfd0", fn_14128dfd0 as *const ()), // sub_14128dfd0
    Replacement::draft_throws(0x14128e0d0, "physics_14128e0d0", fn_14128e0d0 as *const (), Throws::Discard), // sub_14128e0d0
    Replacement::draft_throws(0x14128e220, "physics_14128e220", fn_14128e220 as *const (), Throws::Discard), // sub_14128e220
    Replacement::draft_throws(0x14128e380, "physics_14128e380", fn_14128e380 as *const (), Throws::Discard), // sub_14128e380
    Replacement::draft(0x14128e4c0, "physics_14128e4c0", fn_14128e4c0 as *const ()), // sub_14128e4c0
    Replacement::draft_throws(0x14128e5e0, "physics_14128e5e0", fn_14128e5e0 as *const (), Throws::Caught), // sub_14128e5e0
    Replacement::draft_throws(0x14128e6c0, "physics_14128e6c0", fn_14128e6c0 as *const (), Throws::Caught), // sub_14128e6c0
    Replacement::draft_throws(0x14128e7a0, "physics_14128e7a0", fn_14128e7a0 as *const (), Throws::Caught), // sub_14128e7a0
    Replacement::draft(0x14128e880, "physics_14128e880", fn_14128e880 as *const ()), // sub_14128e880
    Replacement::draft(0x14128e950, "physics_14128e950", fn_14128e950 as *const ()), // sub_14128e950
    Replacement::draft(0x14128ea80, "physics_14128ea80", fn_14128ea80 as *const ()), // sub_14128ea80
    Replacement::draft_throws(0x14128ebc0, "physics_14128ebc0", fn_14128ebc0 as *const (), Throws::Discard), // sub_14128ebc0
    Replacement::draft_throws(0x14128ecf0, "physics_14128ecf0", fn_14128ecf0 as *const (), Throws::Discard), // sub_14128ecf0
    Replacement::draft(0x14128edc0, "physics_14128edc0", tpl_1204::<0x14128f020, 0x20, 0x1401de830, 0x10, 0x14022a610, 0x1430b6bd0, 0x141834d30> as *const ()), // physics::AnimSceneMetadata::physics::AnimSceneMetadata::vfunc_0
    Replacement::draft(0x14128ee20, "unknown_14128ee20", tpl_170::<0x14128f020> as *const ()), // sub_14128ee20
    Replacement::draft_throws(0x14128ee30, "unknown_14128ee30", tpl_838::<0x14128f0f0> as *const (), Throws::Discard), // sub_14128ee30
    Replacement::draft_throws(0x14128ee40, "unknown_14128ee40", tpl_838::<0x14128f3c0> as *const (), Throws::Discard), // sub_14128ee40
    Replacement::draft_throws(0x14128ee50, "unknown_14128ee50", tpl_838::<0x14128f650> as *const (), Throws::Discard), // sub_14128ee50
    Replacement::draft_throws(0x14128ee60, "unknown_14128ee60", fn_14128ee60 as *const (), Throws::Discard), // sub_14128ee60
    Replacement::draft(0x14128eeb0, "unknown_14128eeb0", fn_14128eeb0 as *const ()), // sub_14128eeb0
    Replacement::draft(0x14128f020, "unknown_14128f020", tpl_23253::<0x8, 0x14240ab10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14128f020
    Replacement::draft_throws(0x14128f0f0, "unknown_14128f0f0", fn_14128f0f0 as *const (), Throws::Caught), // sub_14128f0f0
    Replacement::draft_throws(0x14128f3c0, "unknown_14128f3c0", fn_14128f3c0 as *const (), Throws::Caught), // sub_14128f3c0
    Replacement::draft_throws(0x14128f650, "unknown_14128f650", fn_14128f650 as *const (), Throws::Caught), // sub_14128f650
    Replacement::draft(0x14128f8e0, "unknown_14128f8e0", fn_14128f8e0 as *const ()), // sub_14128f8e0
    Replacement::draft(0x14128fb40, "unknown_14128fb40", fn_14128fb40 as *const ()), // sub_14128fb40
    Replacement::draft_throws(0x14128fbf0, "std_14128fbf0", fn_14128fbf0 as *const (), Throws::Discard), // sub_14128fbf0
    Replacement::draft(0x14128fc90, "unknown_14128fc90", tpl_746::<0x1402bf830, 0x14128fd40, 0x8, 0xc, 0x10, 0x142442de0> as *const ()), // sub_14128fc90
    Replacement::draft(0x14128fd40, "unknown_14128fd40", fn_14128fd40 as *const ()), // sub_14128fd40
    Replacement::draft(0x14128fe70, "unknown_14128fe70", fn_14128fe70 as *const ()), // sub_14128fe70
    Replacement::draft(0x14128ff80, "unknown_14128ff80", fn_14128ff80 as *const ()), // sub_14128ff80
    Replacement::draft(0x141290010, "unknown_141290010", fn_141290010 as *const ()), // sub_141290010
    Replacement::draft(0x141290030, "unknown_141290030", tpl_725::<0x18, 0x20, 0x8, 0x142442de0> as *const ()), // sub_141290030
    Replacement::draft(0x1412900a0, "unknown_1412900a0", fn_1412900a0 as *const ()), // sub_1412900a0
    Replacement::draft(0x141290130, "unknown_141290130", fn_141290130 as *const ()), // sub_141290130
    Replacement::draft(0x141290190, "unknown_141290190", fn_141290190 as *const ()), // sub_141290190
    Replacement::draft(0x1412901c0, "unknown_1412901c0", fn_1412901c0 as *const ()), // sub_1412901c0
    Replacement::draft(0x1412904d0, "unknown_1412904d0", fn_1412904d0 as *const ()), // sub_1412904d0
    Replacement::draft(0x1412907c0, "unknown_1412907c0", tpl_746::<0x1412900a0, 0x141290a00, 0x8, 0x18, 0x20, 0x142442de0> as *const ()), // sub_1412907c0
    Replacement::draft(0x141290870, "unknown_141290870", tpl_3310::<0x38, 0x10, 0x18, 0x1433e7348, 0x8, 0x1401f4c80, 0x20, 0x28, 0x1433e7300, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_141290870
    Replacement::draft(0x141290940, "unknown_141290940", tpl_20079::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433e7330, 0x1402bf830, 0x1433e7318> as *const ()), // sub_141290940
    Replacement::draft(0x141290a00, "unknown_141290a00", fn_141290a00 as *const ()), // sub_141290a00
    Replacement::draft(0x141290b40, "unknown_141290b40", fn_141290b40 as *const ()), // sub_141290b40
    Replacement::draft(0x141290c10, "unknown_141290c10", fn_141290c10 as *const ()), // sub_141290c10
    Replacement::draft(0x141290c20, "unknown_141290c20", fn_141290c20 as *const ()), // sub_141290c20
    Replacement::draft(0x141290e10, "unknown_141290e10", fn_141290e10 as *const ()), // sub_141290e10
    Replacement::draft(0x141290fb0, "unknown_141290fb0", fn_141290fb0 as *const ()), // sub_141290fb0
    Replacement::draft(0x141291290, "unknown_141291290", fn_141291290 as *const ()), // sub_141291290
    Replacement::draft(0x141291570, "unknown_141291570", fn_141291570 as *const ()), // sub_141291570
    Replacement::draft(0x141291630, "unknown_141291630", fn_141291630 as *const ()), // sub_141291630
    Replacement::draft(0x141291740, "unknown_141291740", fn_141291740 as *const ()), // sub_141291740
    Replacement::draft(0x141291780, "unknown_141291780", fn_141291780 as *const ()), // sub_141291780
    Replacement::draft(0x141291940, "unknown_141291940", fn_141291940 as *const ()), // sub_141291940
    Replacement::draft(0x141291b10, "unknown_141291b10", fn_141291b10 as *const ()), // sub_141291b10
    Replacement::draft(0x141291bf0, "unknown_141291bf0", tpl_3310::<0x38, 0x10, 0x18, 0x1433e7390, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433e7360, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_141291bf0
    Replacement::draft(0x141291fd0, "unknown_141291fd0", fn_141291fd0 as *const ()), // sub_141291fd0
    Replacement::draft(0x141292150, "unknown_141292150", fn_141292150 as *const ()), // sub_141292150
    Replacement::draft(0x1412921c0, "unknown_1412921c0", fn_1412921c0 as *const ()), // sub_1412921c0
    Replacement::draft(0x141292320, "unknown_141292320", fn_141292320 as *const ()), // sub_141292320
    Replacement::draft(0x141292440, "unknown_141292440", fn_141292440 as *const ()), // sub_141292440
    Replacement::draft(0x141292640, "unknown_141292640", fn_141292640 as *const ()), // sub_141292640
    Replacement::draft(0x141292700, "unknown_141292700", tpl_64835::<0x8, 0x10, 0x14, 0x1411daad0, 0xc> as *const ()), // sub_141292700
    Replacement::draft(0x141292790, "unknown_141292790", fn_141292790 as *const ()), // sub_141292790
    Replacement::draft(0x1412927f0, "unknown_1412927f0", fn_1412927f0 as *const ()), // sub_1412927f0
    Replacement::draft(0x141292870, "unknown_141292870", tpl_19827::<0x8, 0x141290e10> as *const ()), // sub_141292870
    Replacement::draft(0x1412928e0, "unknown_1412928e0", fn_1412928e0 as *const ()), // sub_1412928e0
    Replacement::draft(0x141292960, "unknown_141292960", fn_141292960 as *const ()), // sub_141292960
    Replacement::draft(0x1412929d0, "unknown_1412929d0", fn_1412929d0 as *const ()), // sub_1412929d0
    Replacement::draft(0x141292b90, "unknown_141292b90", fn_141292b90 as *const ()), // sub_141292b90
    Replacement::draft(0x141292c70, "unknown_141292c70", fn_141292c70 as *const ()), // sub_141292c70
    Replacement::draft_throws(0x141292f00, "unknown_141292f00", fn_141292f00 as *const (), Throws::Caught), // sub_141292f00
    Replacement::draft(0x141293480, "unknown_141293480", tpl_21293::<0x8, 0x141290e10> as *const ()), // sub_141293480
    Replacement::draft(0x1412934c0, "unknown_1412934c0", fn_1412934c0 as *const ()), // sub_1412934c0
    Replacement::draft(0x141293690, "unknown_141293690", fn_141293690 as *const ()), // sub_141293690
    Replacement::draft(0x141293bd0, "unknown_141293bd0", fn_141293bd0 as *const ()), // sub_141293bd0
    Replacement::draft(0x141293ee0, "unknown_141293ee0", fn_141293ee0 as *const ()), // sub_141293ee0
    Replacement::draft_throws(0x1412941f0, "unknown_1412941f0", fn_1412941f0 as *const (), Throws::Caught), // sub_1412941f0
    Replacement::draft(0x141294360, "unknown_141294360", fn_141294360 as *const ()), // sub_141294360
    Replacement::draft(0x1412943b0, "unknown_1412943b0", fn_1412943b0 as *const ()), // sub_1412943b0
    Replacement::draft(0x1412943e0, "unknown_1412943e0", fn_1412943e0 as *const ()), // sub_1412943e0
    Replacement::draft(0x1412944b0, "unknown_1412944b0", fn_1412944b0 as *const ()), // sub_1412944b0
    Replacement::draft_throws(0x141294580, "unknown_141294580", fn_141294580 as *const (), Throws::Discard), // sub_141294580
    Replacement::draft_throws(0x141294680, "unknown_141294680", fn_141294680 as *const (), Throws::Discard), // sub_141294680
    Replacement::draft(0x1412946f0, "unknown_1412946f0", fn_1412946f0 as *const ()), // sub_1412946f0
    Replacement::draft(0x141294990, "unknown_141294990", fn_141294990 as *const ()), // sub_141294990
    Replacement::draft(0x141294b10, "unknown_141294b10", tpl_23299::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142186740, 0x1412944b0> as *const ()), // sub_141294b10
    Replacement::draft(0x141294c90, "unknown_141294c90", fn_141294c90 as *const ()), // sub_141294c90
    Replacement::draft(0x141294e60, "unknown_141294e60", fn_141294e60 as *const ()), // sub_141294e60
    Replacement::draft(0x1412950d0, "unknown_1412950d0", fn_1412950d0 as *const ()), // sub_1412950d0
    Replacement::draft(0x1412958e0, "unknown_1412958e0", fn_1412958e0 as *const ()), // sub_1412958e0
    Replacement::draft(0x141295a60, "unknown_141295a60", fn_141295a60 as *const ()), // sub_141295a60
    Replacement::draft(0x141295b70, "unknown_141295b70", fn_141295b70 as *const ()), // sub_141295b70
    Replacement::draft(0x141295be0, "unknown_141295be0", fn_141295be0 as *const ()), // sub_141295be0
    Replacement::draft(0x141295c20, "unknown_141295c20", fn_141295c20 as *const ()), // sub_141295c20
    Replacement::draft_throws(0x141295ca0, "unknown_141295ca0", fn_141295ca0 as *const (), Throws::Caught), // sub_141295ca0
    Replacement::draft(0x141295e70, "unknown_141295e70", fn_141295e70 as *const ()), // sub_141295e70
    Replacement::draft(0x141295f30, "unknown_141295f30", fn_141295f30 as *const ()), // sub_141295f30
    Replacement::draft(0x141295f40, "unknown_141295f40", fn_141295f40 as *const ()), // sub_141295f40
    Replacement::draft(0x141295f70, "unknown_141295f70", fn_141295f70 as *const ()), // sub_141295f70
    Replacement::draft(0x141296050, "unknown_141296050", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x1401ea690> as *const ()), // sub_141296050
    Replacement::draft(0x1412960d0, "unknown_1412960d0", tpl_833::<0x28, 0x30, 0x8, 0x142442de0, 0x20, 0x18> as *const ()), // sub_1412960d0
    Replacement::draft(0x141296150, "unknown_141296150", fn_141296150 as *const ()), // sub_141296150
    Replacement::draft(0x1412961e0, "unknown_1412961e0", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x140fe3a40> as *const ()), // sub_1412961e0
    Replacement::draft_throws(0x141296260, "platform_141296260", fn_141296260 as *const (), Throws::Discard), // platform::internal::operations_queue::internal::BackgroundWorkerThread<platform::dlc::internal::OperationStorage>::platf
    Replacement::draft(0x141296780, "unknown_141296780", tpl_23315::<0x14129c220, 0x1426455a8, 0x142645598, 0x1426455c0, 0x142442de0, 0x140fe3a40> as *const ()), // sub_141296780
    Replacement::draft_throws(0x141296870, "unknown_141296870", fn_141296870 as *const (), Throws::Caught), // sub_141296870
    Replacement::draft_throws(0x141296bd0, "unknown_141296bd0", fn_141296bd0 as *const (), Throws::Caught), // sub_141296bd0
    Replacement::draft_throws(0x141296ed0, "unknown_141296ed0", fn_141296ed0 as *const (), Throws::Caught), // sub_141296ed0
    Replacement::draft_throws(0x141297230, "unknown_141297230", fn_141297230 as *const (), Throws::Caught), // sub_141297230
    Replacement::draft_throws(0x141297540, "unknown_141297540", fn_141297540 as *const (), Throws::Caught), // sub_141297540
    Replacement::draft_throws(0x141297960, "unknown_141297960", fn_141297960 as *const (), Throws::Caught), // sub_141297960
    Replacement::draft_throws(0x141297d00, "unknown_141297d00", fn_141297d00 as *const (), Throws::Caught), // sub_141297d00
    Replacement::draft(0x141298060, "unknown_141298060", fn_141298060 as *const ()), // sub_141298060
    Replacement::draft_throws(0x141298160, "unknown_141298160", fn_141298160 as *const (), Throws::Caught), // sub_141298160
    Replacement::draft_throws(0x1412984d0, "unknown_1412984d0", fn_1412984d0 as *const (), Throws::Caught), // sub_1412984d0
    Replacement::draft_throws(0x141298860, "unknown_141298860", fn_141298860 as *const (), Throws::Caught), // sub_141298860
    Replacement::draft_throws(0x141298b70, "unknown_141298b70", fn_141298b70 as *const (), Throws::Caught), // sub_141298b70
    Replacement::draft_throws(0x141299040, "unknown_141299040", fn_141299040 as *const (), Throws::Caught), // sub_141299040
    Replacement::draft_throws(0x1412993b0, "unknown_1412993b0", fn_1412993b0 as *const (), Throws::Caught), // sub_1412993b0
    Replacement::draft(0x1412996a0, "unknown_1412996a0", tpl_746::<0x1402bf830, 0x14129bf90, 0x8, 0xc, 0x10, 0x142442de0> as *const ()), // sub_1412996a0
    Replacement::draft(0x141299750, "unknown_141299750", fn_141299750 as *const ()), // sub_141299750
    Replacement::draft_throws(0x141299810, "unknown_141299810", fn_141299810 as *const (), Throws::Caught), // sub_141299810
    Replacement::draft(0x141299aa0, "unknown_141299aa0", tpl_746::<0x1412900a0, 0x14129c090, 0x8, 0x18, 0x20, 0x142442de0> as *const ()), // sub_141299aa0
    Replacement::draft(0x141299b50, "unknown_141299b50", fn_141299b50 as *const ()), // sub_141299b50
    Replacement::draft(0x141299cd0, "unknown_141299cd0", fn_141299cd0 as *const ()), // sub_141299cd0
    Replacement::draft_throws(0x141299d10, "unknown_141299d10", fn_141299d10 as *const (), Throws::Caught), // sub_141299d10
    Replacement::draft(0x141299f40, "unknown_141299f40", fn_141299f40 as *const ()), // sub_141299f40
    Replacement::draft(0x14129a010, "unknown_14129a010", fn_14129a010 as *const ()), // sub_14129a010
    Replacement::draft(0x14129a220, "unknown_14129a220", fn_14129a220 as *const ()), // sub_14129a220
    Replacement::draft(0x14129a320, "unknown_14129a320", fn_14129a320 as *const ()), // sub_14129a320
    Replacement::draft(0x14129a420, "unknown_14129a420", fn_14129a420 as *const ()), // sub_14129a420
    Replacement::draft(0x14129a520, "unknown_14129a520", fn_14129a520 as *const ()), // sub_14129a520
    Replacement::draft(0x14129a610, "unknown_14129a610", tpl_23315::<0x14129d0c0, 0x1426455a8, 0x142645598, 0x1426455c0, 0x142442de0, 0x1401ea690> as *const ()), // sub_14129a610
    Replacement::draft(0x14129a700, "unknown_14129a700", fn_14129a700 as *const ()), // sub_14129a700
    Replacement::draft(0x14129a7f0, "unknown_14129a7f0", tpl_23327::<0x143cee2ec, 0x143cee2ac, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129a7f0
    Replacement::draft(0x14129a860, "unknown_14129a860", tpl_23327::<0x143cee2c8, 0x143cee2bc, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129a860
    Replacement::draft(0x14129a8d0, "unknown_14129a8d0", tpl_23327::<0x143cee2a0, 0x143cee2e4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129a8d0
    Replacement::draft(0x14129a940, "unknown_14129a940", tpl_23327::<0x143cee2b8, 0x143cee2cc, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129a940
    Replacement::draft(0x14129a9b0, "unknown_14129a9b0", tpl_23327::<0x143cee2f4, 0x143cee2e0, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129a9b0
    Replacement::draft(0x14129aa20, "unknown_14129aa20", tpl_23327::<0x143cee2a8, 0x143cee2fc, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129aa20
    Replacement::draft(0x14129aa90, "unknown_14129aa90", tpl_23327::<0x143cee2b0, 0x143cee2f8, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129aa90
    Replacement::draft(0x14129ab00, "unknown_14129ab00", tpl_23327::<0x143cee2c0, 0x143cee2d4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129ab00
    Replacement::draft(0x14129ab70, "unknown_14129ab70", tpl_23327::<0x143cee2dc, 0x143cee2b4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129ab70
    Replacement::draft(0x14129abe0, "unknown_14129abe0", tpl_23327::<0x143cee298, 0x143cee2c4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129abe0
    Replacement::draft(0x14129ac50, "unknown_14129ac50", tpl_23327::<0x143cee2e8, 0x143cee2d0, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129ac50
    Replacement::draft(0x14129acc0, "unknown_14129acc0", tpl_23327::<0x143cee294, 0x143cee2a4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129acc0
    Replacement::draft(0x14129ad30, "unknown_14129ad30", tpl_23327::<0x143cee300, 0x143cee2d8, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129ad30
    Replacement::draft(0x14129ada0, "unknown_14129ada0", tpl_23327::<0x143cee2f0, 0x143cee29c, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_14129ada0
    Replacement::draft(0x14129ae10, "unknown_14129ae10", tpl_23328::<0x1412996a0, 0x14243b450, 0x14028b790, 0x1402bf830, 0x14129a860, 0x14129bf30, 0x1402bf7c0, 0x14240ab10, 0x14243b3e0, 0x142442de0> as *const ()), // sub_14129ae10
    Replacement::draft(0x14129b000, "unknown_14129b000", fn_14129b000 as *const ()), // sub_14129b000
    Replacement::draft_throws(0x14129b410, "unknown_14129b410", fn_14129b410 as *const (), Throws::Caught), // sub_14129b410
    Replacement::draft(0x14129b810, "unknown_14129b810", fn_14129b810 as *const ()), // sub_14129b810
    Replacement::draft(0x14129bb20, "unknown_14129bb20", tpl_23328::<0x14128fc90, 0x14243b450, 0x14028b790, 0x1402bf830, 0x14129ab00, 0x14129bf30, 0x1402bf7c0, 0x14240ab10, 0x14243b3e0, 0x142442de0> as *const ()), // sub_14129bb20
    Replacement::draft(0x14129bd10, "unknown_14129bd10", fn_14129bd10 as *const ()), // sub_14129bd10
    Replacement::draft(0x14129bf30, "unknown_14129bf30", fn_14129bf30 as *const ()), // sub_14129bf30
    Replacement::draft(0x14129bf90, "unknown_14129bf90", fn_14129bf90 as *const ()), // sub_14129bf90
    Replacement::draft(0x14129c090, "unknown_14129c090", fn_14129c090 as *const ()), // sub_14129c090
    Replacement::draft(0x14129c190, "unknown_14129c190", fn_14129c190 as *const ()), // sub_14129c190
    Replacement::draft(0x14129c220, "unknown_14129c220", fn_14129c220 as *const ()), // sub_14129c220
    Replacement::draft(0x14129c400, "unknown_14129c400", fn_14129c400 as *const ()), // sub_14129c400
    Replacement::draft(0x14129c580, "unknown_14129c580", fn_14129c580 as *const ()), // sub_14129c580
    Replacement::draft(0x14129c6a0, "unknown_14129c6a0", tpl_23340::<0x14129d0c0, 0x1426455a8, 0x142645598, 0x1426455c0, 0x142442de0, 0x1401ea690> as *const ()), // sub_14129c6a0
    Replacement::draft(0x14129c7c0, "std_14129c7c0", tpl_1::<0x143893980> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c7d0, "std_14129c7d0", fn_14129c7d0 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c840, "std_14129c840", tpl_23342::<0x1433e7768, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c860, "std_14129c860", tpl_1::<0x143893e20> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c870, "std_14129c870", tpl_23343::<0x8, 0x10, 0x90, 0x14049eb20, 0x30, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c8e0, "std_14129c8e0", tpl_23342::<0x1433e7730, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c900, "std_14129c900", tpl_1::<0x1438934d0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c910, "std_14129c910", tpl_23343::<0x8, 0x10, 0x78, 0x141296050, 0x30, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c970, "std_14129c970", tpl_23342::<0x1433e7598, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c990, "std_14129c990", tpl_1::<0x143893f50> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129c9a0, "std_14129c9a0", tpl_23344::<0x8, 0x10, 0xa0, 0x1402bf7c0, 0x78, 0x14240ab10, 0x30, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ca10, "std_14129ca10", tpl_23342::<0x1433e7510, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ca30, "std_14129ca30", tpl_1::<0x143893020> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ca40, "std_14129ca40", fn_14129ca40 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cab0, "std_14129cab0", tpl_23342::<0x1433e7450, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cad0, "std_14129cad0", tpl_1::<0x143893600> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cae0, "std_14129cae0", fn_14129cae0 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cb60, "std_14129cb60", tpl_23342::<0x1433e7658, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cb80, "std_14129cb80", tpl_1::<0x1438933a0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cb90, "std_14129cb90", tpl_23343::<0x8, 0x10, 0x78, 0x1402bf7c0, 0x30, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cbf0, "std_14129cbf0", tpl_23342::<0x1433e74c0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cc10, "std_14129cc10", tpl_1::<0x143893730> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cc20, "std_14129cc20", fn_14129cc20 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ccb0, "std_14129ccb0", tpl_23342::<0x1433e76e0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ccd0, "std_14129ccd0", tpl_1::<0x143893d00> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cce0, "std_14129cce0", tpl_23344::<0x8, 0x10, 0xa0, 0x1412960d0, 0x78, 0x14240ab10, 0x30, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cd50, "std_14129cd50", tpl_23342::<0x1433e7418, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cd70, "std_14129cd70", tpl_1::<0x143893850> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cd80, "std_14129cd80", tpl_23344::<0x8, 0x10, 0xa0, 0x141290030, 0x78, 0x14240ab10, 0x30, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cdf0, "std_14129cdf0", tpl_23342::<0x1433e7560, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ce10, "std_14129ce10", tpl_1::<0x143893ab0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ce20, "std_14129ce20", tpl_23343::<0x8, 0x10, 0x78, 0x141290030, 0x30, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ce80, "std_14129ce80", tpl_23342::<0x1433e7608, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cea0, "std_14129cea0", tpl_1::<0x143893150> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ceb0, "std_14129ceb0", tpl_23342::<0x1433e7488, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129ced0, "std_14129ced0", tpl_1::<0x143893280> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cee0, "std_14129cee0", tpl_23342::<0x1433e75d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cf00, "std_14129cf00", tpl_1::<0x143893bd0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cf10, "std_14129cf10", tpl_23348::<0x8, 0x10, 0x1412961e0, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cf60, "std_14129cf60", tpl_23342::<0x1433e76a8, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::storage::internal::OperationSto
    Replacement::draft(0x14129cf80, "std_14129cf80", fn_14129cf80 as *const ()), // sub_14129cf80
    Replacement::draft(0x14129d060, "std_14129d060", fn_14129d060 as *const ()), // sub_14129d060
    Replacement::draft(0x14129d0c0, "std_14129d0c0", fn_14129d0c0 as *const ()), // sub_14129d0c0
    Replacement::draft(0x14129d2a0, "std_14129d2a0", tpl_554::<0x1433e7808, 0x10, 0x8, 0x14266af30> as *const ()), // sub_14129d2a0
    Replacement::draft_throws(0x14129d2d0, "unknown_14129d2d0", tpl_556::<0x14129d2a0, 0x1437fdac0, 0x14183acce, 0x14129d2ef> as *const (), Throws::Discard), // sub_14129d2d0
    Replacement::draft(0x14129d2f0, "unknown_14129d2f0", tpl_402::<0x1426680a0, 0x8, 0x142646338, 0x14266af30> as *const ()), // sub_14129d2f0
    Replacement::draft(0x14129d330, "unknown_14129d330", tpl_23352::<0x8, 0x10> as *const ()), // sub_14129d330
    Replacement::draft_throws(0x14129d360, "unknown_14129d360", fn_14129d360 as *const (), Throws::Discard), // sub_14129d360
    Replacement::draft(0x14129d4c0, "unknown_14129d4c0", fn_14129d4c0 as *const ()), // sub_14129d4c0
    Replacement::draft(0x14129d530, "unknown_14129d530", fn_14129d530 as *const ()), // sub_14129d530
    Replacement::draft(0x14129d630, "std_14129d630", tpl_6019::<0x14266af80, 0x141834d30> as *const ()), // std::_Node_base::std::_Node_base::vfunc_0
    Replacement::draft(0x14129d660, "std_14129d660", tpl_23356::<0x14266af80, 0x141834d30> as *const ()), // std::_Root_node::std::_Root_node::vfunc_0
    Replacement::draft(0x14129d690, "std_14129d690", tpl_23357::<0x14266af80, 0x141834d30> as *const ()), // std::_Node_back::std::_Node_back::vfunc_0
    Replacement::draft(0x14129d6c0, "std_14129d6c0", fn_14129d6c0 as *const ()), // std::_Node_assert::std::_Node_assert::vfunc_0
    Replacement::draft(0x14129d750, "std_14129d750", fn_14129d750 as *const ()), // std::_Node_if::std::_Node_if::vfunc_0
    Replacement::draft(0x14129d810, "std_14129d810", tpl_23358::<0x14266af80, 0x141834d30> as *const ()), // std::_Node_rep::std::_Node_rep::vfunc_0
    Replacement::draft(0x14129d840, "std_14129d840", fn_14129d840 as *const ()), // sub_14129d840
    Replacement::draft(0x14129d920, "std_14129d920", fn_14129d920 as *const ()), // sub_14129d920
    Replacement::draft(0x14129d960, "std_14129d960", fn_14129d960 as *const ()), // sub_14129d960
    Replacement::draft(0x14129d9a0, "std_14129d9a0", fn_14129d9a0 as *const ()), // std::collate<char>::std::collate<char>::vfunc_5
    Replacement::draft_throws(0x14129d9e0, "std_14129d9e0", fn_14129d9e0 as *const (), Throws::Caught), // std::collate<char>::std::collate<char>::vfunc_4
    Replacement::draft(0x14129daf0, "std_14129daf0", fn_14129daf0 as *const ()), // std::collate<char>::std::collate<char>::vfunc_3
    Replacement::draft(0x14129db40, "std_14129db40", tpl_170::<0x14129dc70> as *const ()), // sub_14129db40
    Replacement::draft(0x14129db50, "std_14129db50", fn_14129db50 as *const ()), // sub_14129db50
    Replacement::draft(0x14129db70, "std_14129db70", tpl_23352::<0x18, 0x10> as *const ()), // sub_14129db70
    Replacement::draft(0x14129dba0, "std_14129dba0", fn_14129dba0 as *const ()), // std::collate<char>::std::collate<char>::vfunc_0
    Replacement::draft(0x14129dbf0, "std_14129dbf0", fn_14129dbf0 as *const ()), // sub_14129dbf0
    Replacement::draft(0x14129dc70, "std_14129dc70", tpl_808::<0x14129e2d0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14129dc70
    Replacement::draft(0x14129dd20, "std_14129dd20", fn_14129dd20 as *const ()), // sub_14129dd20
    Replacement::draft_throws(0x14129de30, "std_14129de30", fn_14129de30 as *const (), Throws::Discard), // sub_14129de30
    Replacement::draft_throws(0x14129dea0, "std_14129dea0", fn_14129dea0 as *const (), Throws::Discard), // sub_14129dea0
    Replacement::draft_throws(0x14129dff0, "std_14129dff0", fn_14129dff0 as *const (), Throws::Discard), // sub_14129dff0
    Replacement::draft(0x14129e160, "std_14129e160", fn_14129e160 as *const ()), // sub_14129e160
    Replacement::draft(0x14129e2d0, "std_14129e2d0", fn_14129e2d0 as *const ()), // sub_14129e2d0
    Replacement::draft_throws(0x14129e440, "std_14129e440", fn_14129e440 as *const (), Throws::Caught), // sub_14129e440
    Replacement::draft_throws(0x14129e4f0, "std_14129e4f0", fn_14129e4f0 as *const (), Throws::Caught), // sub_14129e4f0
    Replacement::draft_throws(0x14129e980, "std_14129e980", fn_14129e980 as *const (), Throws::Discard), // sub_14129e980
    Replacement::draft_throws(0x14129eaa0, "std_14129eaa0", fn_14129eaa0 as *const (), Throws::Caught), // sub_14129eaa0
    Replacement::draft(0x14129ec10, "std_14129ec10", tpl_543::<0x140203cf0> as *const ()), // sub_14129ec10
    Replacement::draft(0x14129ec20, "std_14129ec20", fn_14129ec20 as *const ()), // sub_14129ec20
    Replacement::draft(0x14129eca0, "std_14129eca0", fn_14129eca0 as *const ()), // sub_14129eca0
    Replacement::draft_throws(0x14129ecf0, "std_14129ecf0", fn_14129ecf0 as *const (), Throws::Caught), // sub_14129ecf0
    Replacement::draft(0x14129edb0, "std_14129edb0", fn_14129edb0 as *const ()), // sub_14129edb0
    Replacement::draft(0x14129ee30, "std_14129ee30", fn_14129ee30 as *const ()), // sub_14129ee30
    Replacement::draft(0x14129ee70, "std_14129ee70", fn_14129ee70 as *const ()), // sub_14129ee70
    Replacement::draft(0x14129ee90, "std_14129ee90", tpl_1742::<0x140203cf0> as *const ()), // sub_14129ee90
    Replacement::draft(0x14129eea0, "std_14129eea0", fn_14129eea0 as *const ()), // sub_14129eea0
    Replacement::draft_throws(0x14129ef00, "std_14129ef00", fn_14129ef00 as *const (), Throws::Caught), // sub_14129ef00
    Replacement::draft_throws(0x14129f040, "std_14129f040", fn_14129f040 as *const (), Throws::Discard), // sub_14129f040
    Replacement::draft(0x14129f050, "std_14129f050", fn_14129f050 as *const ()), // sub_14129f050
    Replacement::draft_throws(0x14129f260, "std_14129f260", fn_14129f260 as *const (), Throws::Discard), // sub_14129f260
    Replacement::draft_throws(0x14129f4c0, "std_14129f4c0", fn_14129f4c0 as *const (), Throws::Discard), // sub_14129f4c0
    Replacement::draft_throws(0x14129f570, "std_14129f570", fn_14129f570 as *const (), Throws::Discard), // sub_14129f570
    Replacement::draft(0x14129f600, "std_14129f600", fn_14129f600 as *const ()), // sub_14129f600
    Replacement::draft_throws(0x14129f670, "std_14129f670", fn_14129f670 as *const (), Throws::Discard), // sub_14129f670
    Replacement::draft_throws(0x14129f880, "std_14129f880", fn_14129f880 as *const (), Throws::Discard), // sub_14129f880
    Replacement::draft_throws(0x14129f940, "std_14129f940", fn_14129f940 as *const (), Throws::Discard), // sub_14129f940
    Replacement::draft_throws(0x14129fbd0, "std_14129fbd0", fn_14129fbd0 as *const (), Throws::Discard), // sub_14129fbd0
    Replacement::draft(0x14129fdd0, "std_14129fdd0", fn_14129fdd0 as *const ()), // sub_14129fdd0
    Replacement::draft(0x14129fe10, "std_14129fe10", fn_14129fe10 as *const ()), // sub_14129fe10
    Replacement::draft_throws(0x14129fe60, "std_14129fe60", fn_14129fe60 as *const (), Throws::Discard), // sub_14129fe60
    Replacement::draft(0x1412a04e0, "std_1412a04e0", fn_1412a04e0 as *const ()), // sub_1412a04e0
    Replacement::draft_throws(0x1412a0520, "std_1412a0520", fn_1412a0520 as *const (), Throws::Discard), // sub_1412a0520
    Replacement::draft_throws(0x1412a05a0, "std_1412a05a0", fn_1412a05a0 as *const (), Throws::Caught), // sub_1412a05a0
    Replacement::draft_throws(0x1412a0ba0, "std_1412a0ba0", fn_1412a0ba0 as *const (), Throws::Discard), // sub_1412a0ba0
    Replacement::draft_throws(0x1412a0c90, "std_1412a0c90", fn_1412a0c90 as *const (), Throws::Discard), // sub_1412a0c90
    Replacement::draft_throws(0x1412a0e00, "std_1412a0e00", fn_1412a0e00 as *const (), Throws::Discard), // sub_1412a0e00
    Replacement::draft_throws(0x1412a0fa0, "std_1412a0fa0", fn_1412a0fa0 as *const (), Throws::Discard), // sub_1412a0fa0
    Replacement::draft_throws(0x1412a1080, "std_1412a1080", fn_1412a1080 as *const (), Throws::Caught), // sub_1412a1080
    Replacement::draft_throws(0x1412a1270, "std_1412a1270", fn_1412a1270 as *const (), Throws::Caught), // sub_1412a1270
    Replacement::draft(0x1412a14d0, "std_1412a14d0", fn_1412a14d0 as *const ()), // sub_1412a14d0
    Replacement::draft(0x1412a1660, "std_1412a1660", fn_1412a1660 as *const ()), // sub_1412a1660
    Replacement::draft_throws(0x1412a1790, "std_1412a1790", fn_1412a1790 as *const (), Throws::Discard), // sub_1412a1790
    Replacement::draft_throws(0x1412a1930, "std_1412a1930", fn_1412a1930 as *const (), Throws::Discard), // sub_1412a1930
    Replacement::draft_throws(0x1412a1ac0, "std_1412a1ac0", fn_1412a1ac0 as *const (), Throws::Caught), // sub_1412a1ac0
    Replacement::draft_throws(0x1412a1b90, "std_1412a1b90", fn_1412a1b90 as *const (), Throws::Discard), // sub_1412a1b90
    Replacement::draft(0x1412a1c00, "std_1412a1c00", fn_1412a1c00 as *const ()), // sub_1412a1c00
    Replacement::draft_throws(0x1412a1c90, "std_1412a1c90", fn_1412a1c90 as *const (), Throws::Discard), // sub_1412a1c90
    Replacement::draft(0x1412a1d70, "std_1412a1d70", fn_1412a1d70 as *const ()), // std::_Node_class<char,std::regex_traits<char>>::std::_Node_class<char,std::regex_traits<char>>::vfunc_0
    Replacement::draft_throws(0x1412a1e80, "std_1412a1e80", fn_1412a1e80 as *const (), Throws::Discard), // sub_1412a1e80
    Replacement::draft_throws(0x1412a1f70, "std_1412a1f70", fn_1412a1f70 as *const (), Throws::Discard), // sub_1412a1f70
    Replacement::draft_throws(0x1412a2020, "std_1412a2020", fn_1412a2020 as *const (), Throws::Discard), // sub_1412a2020
    Replacement::draft_throws(0x1412a2180, "std_1412a2180", fn_1412a2180 as *const (), Throws::Discard), // sub_1412a2180
    Replacement::draft_throws(0x1412a2510, "std_1412a2510", fn_1412a2510 as *const (), Throws::Discard), // sub_1412a2510
    Replacement::draft_throws(0x1412a2560, "std_1412a2560", fn_1412a2560 as *const (), Throws::Caught), // sub_1412a2560
    Replacement::draft_throws(0x1412a2640, "std_1412a2640", fn_1412a2640 as *const (), Throws::Discard), // sub_1412a2640
    Replacement::draft_throws(0x1412a26d0, "std_1412a26d0", fn_1412a26d0 as *const (), Throws::Discard), // sub_1412a26d0
    Replacement::draft_throws(0x1412a2750, "std_1412a2750", fn_1412a2750 as *const (), Throws::Discard), // sub_1412a2750
    Replacement::draft_throws(0x1412a2a30, "std_1412a2a30", fn_1412a2a30 as *const (), Throws::Caught), // sub_1412a2a30
    Replacement::draft(0x1412a2d40, "std_1412a2d40", fn_1412a2d40 as *const ()), // sub_1412a2d40
    Replacement::draft(0x1412a3150, "std_1412a3150", fn_1412a3150 as *const ()), // std::_Node_str<char>::std::_Node_str<char>::vfunc_0
    Replacement::draft_throws(0x1412a31a0, "unknown_1412a31a0", fn_1412a31a0 as *const (), Throws::Discard), // sub_1412a31a0
    Replacement::draft_throws(0x1412a3290, "unknown_1412a3290", fn_1412a3290 as *const (), Throws::Discard), // sub_1412a3290
    Replacement::draft(0x1412a3450, "unknown_1412a3450", fn_1412a3450 as *const ()), // sub_1412a3450
    Replacement::draft_throws(0x1412a34a0, "unknown_1412a34a0", fn_1412a34a0 as *const (), Throws::Discard), // sub_1412a34a0
    Replacement::draft_throws(0x1412a3620, "unknown_1412a3620", fn_1412a3620 as *const (), Throws::Discard), // sub_1412a3620
    Replacement::draft(0x1412a3700, "unknown_1412a3700", fn_1412a3700 as *const ()), // sub_1412a3700
    Replacement::draft_throws(0x1412a3850, "unknown_1412a3850", fn_1412a3850 as *const (), Throws::Discard), // sub_1412a3850
    Replacement::draft_throws(0x1412a39f0, "unknown_1412a39f0", fn_1412a39f0 as *const (), Throws::Caught), // sub_1412a39f0
    Replacement::draft_throws(0x1412a3b80, "unknown_1412a3b80", fn_1412a3b80 as *const (), Throws::Discard), // sub_1412a3b80
    Replacement::draft_throws(0x1412a3c60, "unknown_1412a3c60", fn_1412a3c60 as *const (), Throws::Caught), // sub_1412a3c60
    Replacement::draft(0x1412a3e70, "unknown_1412a3e70", fn_1412a3e70 as *const ()), // sub_1412a3e70
    Replacement::draft(0x1412a3fc0, "unknown_1412a3fc0", fn_1412a3fc0 as *const ()), // sub_1412a3fc0
    Replacement::draft(0x1412a40a0, "unknown_1412a40a0", fn_1412a40a0 as *const ()), // sub_1412a40a0
    Replacement::draft(0x1412a41d0, "unknown_1412a41d0", fn_1412a41d0 as *const ()), // sub_1412a41d0
    Replacement::draft(0x1412a4300, "unknown_1412a4300", fn_1412a4300 as *const ()), // sub_1412a4300
    Replacement::draft(0x1412a4410, "unknown_1412a4410", fn_1412a4410 as *const ()), // sub_1412a4410
    Replacement::draft(0x1412a4520, "platform_1412a4520", fn_1412a4520 as *const ()), // platform::Services::platform::Services::vfunc_17
    Replacement::draft(0x1412a4560, "platform_1412a4560", fn_1412a4560 as *const ()), // sub_1412a4560
    Replacement::draft(0x1412a4720, "platform_1412a4720", fn_1412a4720 as *const ()), // sub_1412a4720
    Replacement::draft_throws(0x1412a4ff0, "platform_1412a4ff0", fn_1412a4ff0 as *const (), Throws::Caught), // platform::TServices<platform::system::epic::Context,platform::nativesystems::internal::Context,platform::network::intern
    Replacement::draft_throws(0x1412a55f0, "platform_1412a55f0", fn_1412a55f0 as *const (), Throws::Caught), // platform::TServices<platform::system::epic::Context,platform::nativesystems::internal::Context,platform::network::intern
    Replacement::draft(0x1412a5dc0, "platform_1412a5dc0", tpl_1008::<0x88, 0x142442de0> as *const ()), // sub_1412a5dc0
    Replacement::draft(0x1412a5df0, "platform_1412a5df0", fn_1412a5df0 as *const ()), // sub_1412a5df0
    Replacement::draft(0x1412a5e90, "platform_1412a5e90", fn_1412a5e90 as *const ()), // sub_1412a5e90
    Replacement::draft(0x1412a5ed0, "platform_1412a5ed0", fn_1412a5ed0 as *const ()), // sub_1412a5ed0
    Replacement::draft(0x1412a5fd0, "platform_1412a5fd0", fn_1412a5fd0 as *const ()), // sub_1412a5fd0
    Replacement::draft(0x1412a6060, "platform_1412a6060", fn_1412a6060 as *const ()), // sub_1412a6060
    Replacement::draft(0x1412a6100, "platform_1412a6100", fn_1412a6100 as *const ()), // sub_1412a6100
    Replacement::draft(0x1412a64e0, "platform_1412a64e0", fn_1412a64e0 as *const ()), // sub_1412a64e0
    Replacement::draft(0x1412a6710, "platform_1412a6710", fn_1412a6710 as *const ()), // sub_1412a6710
    Replacement::draft(0x1412a6780, "platform_1412a6780", fn_1412a6780 as *const ()), // sub_1412a6780
    Replacement::draft(0x1412a67e0, "platform_1412a67e0", fn_1412a67e0 as *const ()), // sub_1412a67e0
    Replacement::draft(0x1412a6930, "platform_1412a6930", fn_1412a6930 as *const ()), // sub_1412a6930
    Replacement::draft(0x1412a71f0, "platform_1412a71f0", fn_1412a71f0 as *const ()), // sub_1412a71f0
    Replacement::draft(0x1412a7290, "platform_1412a7290", fn_1412a7290 as *const ()), // sub_1412a7290
    Replacement::draft(0x1412a73a0, "platform_1412a73a0", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x1412a7d80> as *const ()), // sub_1412a73a0
    Replacement::draft(0x1412a7420, "platform_1412a7420", fn_1412a7420 as *const ()), // sub_1412a7420
    Replacement::draft(0x1412a7540, "platform_1412a7540", tpl_23439::<0x20, 0x28, 0x8, 0x142442de0, 0x1412a7420> as *const ()), // sub_1412a7540
    Replacement::draft(0x1412a75b0, "platform_1412a75b0", fn_1412a75b0 as *const ()), // sub_1412a75b0
    Replacement::draft(0x1412a76c0, "platform_1412a76c0", tpl_23439::<0x20, 0x28, 0x8, 0x142442de0, 0x1412a75b0> as *const ()), // sub_1412a76c0
    Replacement::draft_throws(0x1412a7730, "platform_1412a7730", fn_1412a7730 as *const (), Throws::Discard), // platform::TServices<platform::system::epic::Context,platform::nativesystems::internal::Context,platform::network::intern
    Replacement::draft_throws(0x1412a7770, "unknown_1412a7770", fn_1412a7770 as *const (), Throws::Discard), // sub_1412a7770
    Replacement::draft(0x1412a79e0, "unknown_1412a79e0", fn_1412a79e0 as *const ()), // sub_1412a79e0
    Replacement::draft(0x1412a7a30, "unknown_1412a7a30", fn_1412a7a30 as *const ()), // sub_1412a7a30
    Replacement::draft_throws(0x1412a7c10, "unknown_1412a7c10", fn_1412a7c10 as *const (), Throws::Discard), // sub_1412a7c10
    Replacement::draft(0x1412a7c60, "unknown_1412a7c60", fn_1412a7c60 as *const ()), // sub_1412a7c60
    Replacement::draft(0x1412a7c90, "unknown_1412a7c90", tpl_170::<0x1412a7ca0> as *const ()), // sub_1412a7c90
    Replacement::draft(0x1412a7ca0, "unknown_1412a7ca0", fn_1412a7ca0 as *const ()), // sub_1412a7ca0
    Replacement::draft(0x1412a7d80, "unknown_1412a7d80", fn_1412a7d80 as *const ()), // sub_1412a7d80
    Replacement::draft(0x1412a7e60, "unknown_1412a7e60", tpl_18985::<0x8, 0x1402bf7c0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1412a7e60
    Replacement::draft(0x1412a7f30, "unknown_1412a7f30", tpl_60956::<0x19, 0x10, 0x1412a7f30, 0x30, 0x14240ab10, 0x1401e66f0> as *const ()), // sub_1412a7f30
    Replacement::draft(0x1412a7fb0, "unknown_1412a7fb0", fn_1412a7fb0 as *const ()), // sub_1412a7fb0
    Replacement::draft(0x1412a80c0, "unknown_1412a80c0", fn_1412a80c0 as *const ()), // sub_1412a80c0
    Replacement::draft(0x1412a8100, "unknown_1412a8100", fn_1412a8100 as *const ()), // sub_1412a8100
    Replacement::draft(0x1412a81d0, "unknown_1412a81d0", fn_1412a81d0 as *const ()), // sub_1412a81d0
    Replacement::draft(0x1412a8260, "unknown_1412a8260", fn_1412a8260 as *const ()), // sub_1412a8260
    Replacement::draft(0x1412a8330, "unknown_1412a8330", fn_1412a8330 as *const ()), // sub_1412a8330
    Replacement::draft(0x1412a8420, "unknown_1412a8420", tpl_23327::<0x143cee314, 0x143cee324, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a8420
    Replacement::draft(0x1412a8490, "unknown_1412a8490", tpl_23327::<0x143cee31c, 0x143cee310, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a8490
    Replacement::draft(0x1412a8500, "unknown_1412a8500", tpl_23327::<0x143cee320, 0x143cee318, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a8500
    Replacement::draft(0x1412a8570, "unknown_1412a8570", fn_1412a8570 as *const ()), // sub_1412a8570
    Replacement::draft(0x1412a86e0, "std_1412a86e0", tpl_1::<0x1438940e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a86f0, "std_1412a86f0", tpl_23456::<0x8, 0x10, 0x1402bf7c0, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412a8740, "std_1412a8740", tpl_23342::<0x1433e99d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a8760, "std_1412a8760", tpl_1::<0x143894230> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a8770, "std_1412a8770", tpl_81848::<0x8, 0x10, 0x168, 0x1412a81d0, 0xd4, 0x14240ab10, 0x48, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a87f0, "std_1412a87f0", tpl_23342::<0x1433e9988, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a8810, "std_1412a8810", tpl_1::<0x143894380> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a8820, "std_1412a8820", tpl_81848::<0x8, 0x10, 0x160, 0x1412a81d0, 0xd4, 0x14240ab10, 0x48, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a88a0, "std_1412a88a0", tpl_23342::<0x1433e9950, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::multiplayer::internal::Operatio
    Replacement::draft(0x1412a88c0, "std_1412a88c0", fn_1412a88c0 as *const ()), // sub_1412a88c0
    Replacement::draft(0x1412a89d0, "std_1412a89d0", fn_1412a89d0 as *const ()), // sub_1412a89d0
    Replacement::draft(0x1412a89f0, "std_1412a89f0", fn_1412a89f0 as *const ()), // sub_1412a89f0
    Replacement::draft(0x1412a8a40, "std_1412a8a40", fn_1412a8a40 as *const ()), // sub_1412a8a40
    Replacement::draft(0x1412a8a80, "std_1412a8a80", tpl_23461::<0x10, 0x1401fb9f0> as *const ()), // sub_1412a8a80
    Replacement::draft(0x1412a8aa0, "std_1412a8aa0", tpl_23461::<0x10, 0x1401de830> as *const ()), // sub_1412a8aa0
    Replacement::draft_throws(0x1412a8ac0, "std_1412a8ac0", fn_1412a8ac0 as *const (), Throws::Caught), // sub_1412a8ac0
    Replacement::draft(0x1412a8cb0, "std_1412a8cb0", fn_1412a8cb0 as *const ()), // sub_1412a8cb0
    Replacement::draft(0x1412a8cd0, "std_1412a8cd0", fn_1412a8cd0 as *const ()), // sub_1412a8cd0
    Replacement::draft(0x1412a8fe0, "std_1412a8fe0", fn_1412a8fe0 as *const ()), // sub_1412a8fe0
    Replacement::draft(0x1412a91e0, "std_1412a91e0", tpl_365::<0x140260f30> as *const ()), // sub_1412a91e0
    Replacement::draft(0x1412a91f0, "std_1412a91f0", fn_1412a91f0 as *const ()), // sub_1412a91f0
    Replacement::draft(0x1412a9290, "std_1412a9290", fn_1412a9290 as *const ()), // sub_1412a9290
    Replacement::draft(0x1412a9330, "std_1412a9330", tpl_725::<0x30, 0x38, 0x8, 0x142442de0> as *const ()), // sub_1412a9330
    Replacement::draft(0x1412a93a0, "std_1412a93a0", fn_1412a93a0 as *const ()), // sub_1412a93a0
    Replacement::draft(0x1412a9430, "std_1412a9430", fn_1412a9430 as *const ()), // sub_1412a9430
    Replacement::draft(0x1412a9520, "std_1412a9520", fn_1412a9520 as *const ()), // sub_1412a9520
    Replacement::draft(0x1412a9610, "std_1412a9610", tpl_23327::<0x143cee338, 0x143cee364, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a9610
    Replacement::draft(0x1412a9680, "std_1412a9680", tpl_23327::<0x143cee35c, 0x143cee350, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a9680
    Replacement::draft(0x1412a96f0, "std_1412a96f0", tpl_23327::<0x143cee354, 0x143cee330, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a96f0
    Replacement::draft(0x1412a9760, "std_1412a9760", tpl_23327::<0x143cee358, 0x143cee344, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a9760
    Replacement::draft(0x1412a97d0, "std_1412a97d0", tpl_23327::<0x143cee340, 0x143cee334, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a97d0
    Replacement::draft(0x1412a9840, "std_1412a9840", tpl_23327::<0x143cee32c, 0x143cee360, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a9840
    Replacement::draft(0x1412a98b0, "std_1412a98b0", tpl_23327::<0x143cee33c, 0x143cee348, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a98b0
    Replacement::draft(0x1412a9920, "std_1412a9920", tpl_23327::<0x143cee328, 0x143cee34c, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412a9920
    Replacement::draft(0x1412a9990, "std_1412a9990", fn_1412a9990 as *const ()), // sub_1412a9990
    Replacement::draft(0x1412a9bd0, "std_1412a9bd0", fn_1412a9bd0 as *const ()), // sub_1412a9bd0
    Replacement::draft(0x1412a9e00, "std_1412a9e00", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x1401fb9f0> as *const ()), // sub_1412a9e00
    Replacement::draft(0x1412a9e80, "std_1412a9e80", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x1401de830> as *const ()), // sub_1412a9e80
    Replacement::draft(0x1412a9f00, "std_1412a9f00", fn_1412a9f00 as *const ()), // sub_1412a9f00
    Replacement::draft(0x1412aa030, "std_1412aa030", fn_1412aa030 as *const ()), // sub_1412aa030
    Replacement::draft(0x1412aa230, "std_1412aa230", tpl_23340::<0x1412aa4b0, 0x1426455a8, 0x142645598, 0x1426455c0, 0x142442de0, 0x1401fb9f0> as *const ()), // sub_1412aa230
    Replacement::draft(0x1412aa350, "std_1412aa350", tpl_746::<0x1402bf830, 0x1412a9520, 0x8, 0xc, 0x10, 0x142442de0> as *const ()), // sub_1412aa350
    Replacement::draft(0x1412aa400, "std_1412aa400", tpl_746::<0x1412a93a0, 0x1412a9f00, 0x8, 0x30, 0x38, 0x142442de0> as *const ()), // sub_1412aa400
    Replacement::draft(0x1412aa4b0, "std_1412aa4b0", fn_1412aa4b0 as *const ()), // sub_1412aa4b0
    Replacement::draft(0x1412aa5e0, "std_1412aa5e0", tpl_1::<0x1438949d0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa5f0, "std_1412aa5f0", tpl_23342::<0x1433e9ad0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa610, "std_1412aa610", tpl_1::<0x143894890> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa620, "std_1412aa620", tpl_23456::<0x8, 0x10, 0x1412a9e00, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa670, "std_1412aa670", tpl_23342::<0x1433e9a48, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa690, "std_1412aa690", tpl_1::<0x143894610> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa6a0, "std_1412aa6a0", tpl_23456::<0x8, 0x10, 0x1412a9e80, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa6f0, "std_1412aa6f0", tpl_23342::<0x1433e9b08, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa710, "std_1412aa710", tpl_1::<0x143894c60> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa720, "std_1412aa720", tpl_23342::<0x1433e9b40, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa740, "std_1412aa740", tpl_1::<0x143894b20> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa750, "std_1412aa750", tpl_23348::<0x8, 0x10, 0x1402bf7c0, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412aa7a0, "std_1412aa7a0", tpl_23342::<0x1433e9a98, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa7c0, "std_1412aa7c0", tpl_1::<0x143894750> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa7d0, "std_1412aa7d0", tpl_23476::<0x8, 0x10, 0x20, 0x1412a9330, 0x18, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa840, "std_1412aa840", tpl_23342::<0x1433e9bb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa860, "std_1412aa860", tpl_1::<0x1438944d0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa870, "std_1412aa870", fn_1412aa870 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa8d0, "std_1412aa8d0", tpl_23342::<0x1433e9b78, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa8f0, "std_1412aa8f0", tpl_1::<0x143894da0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa900, "std_1412aa900", tpl_23476::<0x8, 0x10, 0x20, 0x1402bf7c0, 0x18, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa970, "std_1412aa970", tpl_23342::<0x1433e9a10, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::installation::internal::Operati
    Replacement::draft(0x1412aa990, "std_1412aa990", fn_1412aa990 as *const ()), // sub_1412aa990
    Replacement::draft(0x1412aab70, "std_1412aab70", fn_1412aab70 as *const ()), // sub_1412aab70
    Replacement::draft(0x1412aabd0, "std_1412aabd0", fn_1412aabd0 as *const ()), // sub_1412aabd0
    Replacement::draft(0x1412aac90, "std_1412aac90", tpl_23327::<0x143cee36c, 0x143cee368, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412aac90
    Replacement::draft(0x1412aad00, "std_1412aad00", fn_1412aad00 as *const ()), // sub_1412aad00
    Replacement::draft(0x1412aae60, "std_1412aae60", tpl_1::<0x143894ee0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::http::internal::OperationStorag
    Replacement::draft(0x1412aae70, "std_1412aae70", fn_1412aae70 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::http::internal::OperationStorag
    Replacement::draft(0x1412aaf10, "std_1412aaf10", tpl_23342::<0x1433e9c28, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::http::internal::OperationStorag
    Replacement::draft(0x1412aaf30, "std_1412aaf30", fn_1412aaf30 as *const ()), // sub_1412aaf30
    Replacement::draft(0x1412aafc0, "std_1412aafc0", tpl_23327::<0x143cee370, 0x143cee374, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412aafc0
    Replacement::draft(0x1412ab030, "std_1412ab030", tpl_1::<0x143895000> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::friends::internal::OperationSto
    Replacement::draft(0x1412ab040, "std_1412ab040", tpl_23342::<0x1433e9c60, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::friends::internal::OperationSto
    Replacement::draft(0x1412ab060, "unknown_1412ab060", tpl_23484::<0x8, 0x1412abf40, 0x1401f33e0> as *const ()), // sub_1412ab060
    Replacement::draft(0x1412ab0b0, "unknown_1412ab0b0", tpl_725::<0x28, 0x30, 0x8, 0x142442de0> as *const ()), // sub_1412ab0b0
    Replacement::draft(0x1412ab120, "unknown_1412ab120", tpl_170::<0x1412ab0b0> as *const ()), // sub_1412ab120
    Replacement::draft(0x1412b4050, "unknown_1412b4050", fn_1412b4050 as *const ()), // sub_1412b4050
    Replacement::draft(0x1412b40d0, "unknown_1412b40d0", fn_1412b40d0 as *const ()), // sub_1412b40d0
    Replacement::draft(0x1412b4320, "unknown_1412b4320", fn_1412b4320 as *const ()), // sub_1412b4320
    Replacement::draft(0x1412b4390, "unknown_1412b4390", tpl_365::<0x1412b45c0> as *const ()), // sub_1412b4390
    Replacement::draft(0x1412b43a0, "unknown_1412b43a0", tpl_23505::<0x3, 0x4, 0x8, 0x14240aa00, 0x50, 0x54, 0x58, 0x68, 0x1412b4050> as *const ()), // sub_1412b43a0
    Replacement::draft(0x1412b4420, "unknown_1412b4420", fn_1412b4420 as *const ()), // sub_1412b4420
    Replacement::draft(0x1412b4500, "unknown_1412b4500", fn_1412b4500 as *const ()), // sub_1412b4500
    Replacement::draft(0x1412b45c0, "unknown_1412b45c0", fn_1412b45c0 as *const ()), // sub_1412b45c0
    Replacement::draft(0x1412b46b0, "unknown_1412b46b0", fn_1412b46b0 as *const ()), // sub_1412b46b0
    Replacement::draft(0x1412b4750, "unknown_1412b4750", tpl_725::<0x20, 0x28, 0x8, 0x142442de0> as *const ()), // sub_1412b4750
    Replacement::draft(0x1412b47c0, "unknown_1412b47c0", tpl_20064::<0xa8, 0xb0, 0x8, 0x142442de0, 0xa0, 0x98, 0x50, 0x14240ab10> as *const ()), // sub_1412b47c0
    Replacement::draft(0x1412b4850, "unknown_1412b4850", fn_1412b4850 as *const ()), // sub_1412b4850
    Replacement::draft(0x1412b48f0, "unknown_1412b48f0", fn_1412b48f0 as *const ()), // sub_1412b48f0
    Replacement::draft(0x1412b49f0, "unknown_1412b49f0", fn_1412b49f0 as *const ()), // sub_1412b49f0
    Replacement::draft(0x1412b4b20, "unknown_1412b4b20", fn_1412b4b20 as *const ()), // sub_1412b4b20
    Replacement::draft(0x1412b4c50, "unknown_1412b4c50", fn_1412b4c50 as *const ()), // sub_1412b4c50
    Replacement::draft(0x1412b4d40, "unknown_1412b4d40", tpl_23315::<0x1412b74b0, 0x1426455a8, 0x142645598, 0x1426455c0, 0x142442de0, 0x1412a7d80> as *const ()), // sub_1412b4d40
    Replacement::draft(0x1412b4e30, "unknown_1412b4e30", fn_1412b4e30 as *const ()), // sub_1412b4e30
    Replacement::draft(0x1412b4ea0, "unknown_1412b4ea0", fn_1412b4ea0 as *const ()), // sub_1412b4ea0
    Replacement::draft(0x1412b5070, "unknown_1412b5070", fn_1412b5070 as *const ()), // sub_1412b5070
    Replacement::draft(0x1412b5160, "unknown_1412b5160", fn_1412b5160 as *const ()), // sub_1412b5160
    Replacement::draft(0x1412b52b0, "unknown_1412b52b0", tpl_23327::<0x143cee3c4, 0x143cee3e8, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b52b0
    Replacement::draft(0x1412b5320, "unknown_1412b5320", tpl_23327::<0x143cee3b0, 0x143cee3a0, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5320
    Replacement::draft(0x1412b5390, "unknown_1412b5390", tpl_23327::<0x143cee3f8, 0x143cee384, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5390
    Replacement::draft(0x1412b5400, "unknown_1412b5400", tpl_23327::<0x143cee3f0, 0x143cee3b8, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5400
    Replacement::draft(0x1412b5470, "unknown_1412b5470", tpl_23327::<0x143cee390, 0x143cee378, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5470
    Replacement::draft(0x1412b54e0, "unknown_1412b54e0", tpl_23327::<0x143cee3ac, 0x143cee3bc, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b54e0
    Replacement::draft(0x1412b5550, "unknown_1412b5550", tpl_23327::<0x143cee3dc, 0x143cee398, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5550
    Replacement::draft(0x1412b55c0, "unknown_1412b55c0", tpl_23327::<0x143cee3ec, 0x143cee3e0, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b55c0
    Replacement::draft(0x1412b5630, "unknown_1412b5630", tpl_23327::<0x143cee3b4, 0x143cee38c, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5630
    Replacement::draft(0x1412b56a0, "unknown_1412b56a0", tpl_23327::<0x143cee3f4, 0x143cee3d0, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b56a0
    Replacement::draft(0x1412b5710, "unknown_1412b5710", tpl_23327::<0x143cee3d4, 0x143cee388, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5710
    Replacement::draft(0x1412b5780, "unknown_1412b5780", tpl_23327::<0x143cee3d8, 0x143cee3c0, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5780
    Replacement::draft(0x1412b57f0, "unknown_1412b57f0", tpl_23327::<0x143cee394, 0x143cee380, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b57f0
    Replacement::draft(0x1412b5860, "unknown_1412b5860", tpl_23327::<0x143cee3cc, 0x143cee3a4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5860
    Replacement::draft(0x1412b58d0, "unknown_1412b58d0", tpl_23327::<0x143cee3fc, 0x143cee3e4, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b58d0
    Replacement::draft(0x1412b5940, "unknown_1412b5940", tpl_23327::<0x143cee37c, 0x143cee39c, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b5940
    Replacement::draft(0x1412b59b0, "unknown_1412b59b0", tpl_23327::<0x143cee3c8, 0x143cee3a8, 0x141834a78, 0x143c8c6a0, 0x141834a0c> as *const ()), // sub_1412b59b0
    Replacement::draft(0x1412b5a20, "unknown_1412b5a20", fn_1412b5a20 as *const ()), // sub_1412b5a20
    Replacement::draft(0x1412b5cb0, "unknown_1412b5cb0", tpl_23552::<0x14128fc90, 0x14243b450, 0x14240aa00, 0x1402bf830, 0x14240ab10, 0x1412b5940, 0x14129bf30, 0x1402bf7c0, 0x14243b3e0, 0x142442de0> as *const ()), // sub_1412b5cb0
    Replacement::draft(0x1412b5f00, "unknown_1412b5f00", tpl_23552::<0x1412b7020, 0x14243b450, 0x14240aa00, 0x1402bf830, 0x14240ab10, 0x1412b5390, 0x14129bf30, 0x1402bf7c0, 0x14243b3e0, 0x142442de0> as *const ()), // sub_1412b5f00
    Replacement::draft(0x1412b6150, "unknown_1412b6150", tpl_23553::<0x1412b70d0, 0x14243b450, 0x14240aa00, 0x1412b4850, 0x14240ab10, 0x1412b5470, 0x14129bf30, 0x1412b47c0, 0x14243b3e0, 0x142442de0> as *const ()), // sub_1412b6150
    Replacement::draft(0x1412b6420, "unknown_1412b6420", fn_1412b6420 as *const ()), // sub_1412b6420
    Replacement::draft(0x1412b66c0, "unknown_1412b66c0", tpl_23553::<0x1412b70d0, 0x14243b450, 0x14240aa00, 0x1412b4850, 0x14240ab10, 0x1412b5780, 0x14129bf30, 0x1412b47c0, 0x14243b3e0, 0x142442de0> as *const ()), // sub_1412b66c0
    Replacement::draft(0x1412b6990, "unknown_1412b6990", fn_1412b6990 as *const ()), // sub_1412b6990
    Replacement::draft(0x1412b6b60, "unknown_1412b6b60", fn_1412b6b60 as *const ()), // sub_1412b6b60
    Replacement::draft(0x1412b6d20, "unknown_1412b6d20", fn_1412b6d20 as *const ()), // sub_1412b6d20
    Replacement::draft(0x1412b6d80, "unknown_1412b6d80", fn_1412b6d80 as *const ()), // sub_1412b6d80
    Replacement::draft(0x1412b6eb0, "unknown_1412b6eb0", fn_1412b6eb0 as *const ()), // sub_1412b6eb0
    Replacement::draft(0x1412b6f20, "unknown_1412b6f20", fn_1412b6f20 as *const ()), // sub_1412b6f20
    Replacement::draft(0x1412b7020, "unknown_1412b7020", tpl_746::<0x1402bf830, 0x1412b6d80, 0x8, 0xc, 0x10, 0x142442de0> as *const ()), // sub_1412b7020
    Replacement::draft(0x1412b70d0, "unknown_1412b70d0", fn_1412b70d0 as *const ()), // sub_1412b70d0
    Replacement::draft(0x1412b71b0, "unknown_1412b71b0", tpl_23561::<0x1412b46b0, 0x1412b5070, 0x8, 0x110, 0x14102bc60> as *const ()), // sub_1412b71b0
    Replacement::draft(0x1412b7220, "unknown_1412b7220", fn_1412b7220 as *const ()), // sub_1412b7220
    Replacement::draft(0x1412b7340, "unknown_1412b7340", fn_1412b7340 as *const ()), // sub_1412b7340
    Replacement::draft(0x1412b74b0, "unknown_1412b74b0", fn_1412b74b0 as *const ()), // sub_1412b74b0
    Replacement::draft(0x1412b76a0, "unknown_1412b76a0", tpl_20795::<0x8, 0x1412b6d20> as *const ()), // sub_1412b76a0
    Replacement::draft(0x1412b76b0, "unknown_1412b76b0", fn_1412b76b0 as *const ()), // sub_1412b76b0
    Replacement::draft(0x1412b76f0, "std_1412b76f0", tpl_1::<0x143895a40> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7700, "std_1412b7700", tpl_23342::<0x1433ea300, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7720, "std_1412b7720", tpl_1::<0x1438957e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7730, "std_1412b7730", tpl_23566::<0x8, 0x10, 0x50, 0x1402bf7c0, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7790, "std_1412b7790", tpl_23342::<0x1433ea370, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b77b0, "std_1412b77b0", tpl_1::<0x143896000> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b77c0, "std_1412b77c0", tpl_23342::<0x1433ea488, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b77e0, "std_1412b77e0", tpl_1::<0x143895b60> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b77f0, "std_1412b77f0", tpl_23342::<0x1433ea4c0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7810, "std_1412b7810", tpl_1::<0x143895250> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7820, "std_1412b7820", fn_1412b7820 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7870, "std_1412b7870", tpl_23342::<0x1433ea2b0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7890, "std_1412b7890", tpl_1::<0x143895490> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b78a0, "std_1412b78a0", tpl_23566::<0x8, 0x10, 0x50, 0x1412b47c0, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7900, "std_1412b7900", tpl_23342::<0x1433ea198, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7920, "std_1412b7920", tpl_1::<0x143895920> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7930, "std_1412b7930", tpl_23342::<0x1433ea128, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7950, "std_1412b7950", tpl_1::<0x143895370> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7960, "std_1412b7960", tpl_23342::<0x1433ea240, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7980, "std_1412b7980", tpl_1::<0x143896120> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7990, "std_1412b7990", tpl_23348::<0x8, 0x10, 0x1412a73a0, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b79e0, "std_1412b79e0", tpl_23342::<0x1433ea3e0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7a00, "std_1412b7a00", tpl_1::<0x143895130> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7a10, "std_1412b7a10", fn_1412b7a10 as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7a80, "std_1412b7a80", tpl_23342::<0x1433ea278, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7aa0, "std_1412b7aa0", tpl_1::<0x143895ed0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7ab0, "std_1412b7ab0", tpl_23566::<0x8, 0x10, 0x50, 0x1412b4750, 0x14240ab10, 0xa8, 0x14243b3e0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7b10, "std_1412b7b10", tpl_23342::<0x1433ea208, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7b30, "std_1412b7b30", tpl_1::<0x143895da0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7b40, "std_1412b7b40", tpl_23342::<0x1433ea160, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7b60, "std_1412b7b60", tpl_1::<0x143896240> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7b70, "std_1412b7b70", tpl_23342::<0x1433ea3a8, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7b90, "std_1412b7b90", tpl_1::<0x143895c80> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7ba0, "std_1412b7ba0", tpl_23342::<0x1433ea418, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7bc0, "std_1412b7bc0", tpl_1::<0x1438956c0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7bd0, "std_1412b7bd0", tpl_23342::<0x1433ea338, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7bf0, "std_1412b7bf0", tpl_1::<0x143896360> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7c00, "std_1412b7c00", tpl_23342::<0x1433ea1d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7c20, "std_1412b7c20", tpl_1::<0x1438955a0> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7c30, "std_1412b7c30", tpl_23342::<0x1433ea450, 0x8> as *const ()), // std::_Func_impl_no_alloc<platform::internal::operations_queue::registerHandler<platform::dlc::internal::OperationStorage
    Replacement::draft(0x1412b7c50, "unknown_1412b7c50", fn_1412b7c50 as *const ()), // sub_1412b7c50
    Replacement::draft(0x1412b7cb0, "unknown_1412b7cb0", fn_1412b7cb0 as *const ()), // sub_1412b7cb0
    Replacement::draft(0x1412b7cc0, "unknown_1412b7cc0", fn_1412b7cc0 as *const ()), // sub_1412b7cc0
    Replacement::draft(0x1412b7db0, "unknown_1412b7db0", tpl_2133::<0x4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8> as *const ()), // sub_1412b7db0
    Replacement::draft(0x1412b7e10, "unknown_1412b7e10", fn_1412b7e10 as *const ()), // sub_1412b7e10
    Replacement::draft_throws(0x1412b7f10, "unknown_1412b7f10", fn_1412b7f10 as *const (), Throws::Discard), // sub_1412b7f10
    Replacement::draft(0x1412b7ff0, "unknown_1412b7ff0", tpl_23573::<0x143881780, 0x14243b0b0, 0x338, 0x40, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412b7ff0
    Replacement::draft_throws(0x1412b8080, "unknown_1412b8080", fn_1412b8080 as *const (), Throws::Discard), // sub_1412b8080
    Replacement::draft_throws(0x1412b8170, "unknown_1412b8170", fn_1412b8170 as *const (), Throws::Caught), // sub_1412b8170
    Replacement::draft_throws(0x1412b82a0, "unknown_1412b82a0", fn_1412b82a0 as *const (), Throws::Caught), // sub_1412b82a0
    Replacement::draft_throws(0x1412b83d0, "unknown_1412b83d0", fn_1412b83d0 as *const (), Throws::Caught), // sub_1412b83d0
    Replacement::draft(0x1412b8500, "unknown_1412b8500", fn_1412b8500 as *const ()), // sub_1412b8500
    Replacement::draft(0x1412b8590, "unknown_1412b8590", fn_1412b8590 as *const ()), // sub_1412b8590
    Replacement::draft(0x1412b8620, "unknown_1412b8620", tpl_23573::<0x143881780, 0x14243b0b0, 0x338, 0xe0, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412b8620
    Replacement::draft_throws(0x1412b86c0, "unknown_1412b86c0", fn_1412b86c0 as *const (), Throws::Discard), // sub_1412b86c0
    Replacement::draft_throws(0x1412b8760, "unknown_1412b8760", fn_1412b8760 as *const (), Throws::Discard), // sub_1412b8760
    Replacement::draft(0x1412b8890, "unknown_1412b8890", fn_1412b8890 as *const ()), // sub_1412b8890
    Replacement::draft(0x1412b8910, "unknown_1412b8910", fn_1412b8910 as *const ()), // sub_1412b8910
    Replacement::draft(0x1412b89e0, "unknown_1412b89e0", fn_1412b89e0 as *const ()), // sub_1412b89e0
    Replacement::draft_throws(0x1412b8ad0, "unknown_1412b8ad0", tpl_8703::<0x14388179c, 0x1438817a8, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x1412bdae0, 0x1412bdd40, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412b8ad0
    Replacement::draft(0x1412b8b70, "unknown_1412b8b70", fn_1412b8b70 as *const ()), // sub_1412b8b70
    Replacement::draft_throws(0x1412b8c30, "unknown_1412b8c30", tpl_8704::<0x14388179c, 0x1438817a8, 0x1401e9b60, 0x328, 0x338, 0x120, 0x1412bdae0, 0x1412be010, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_1412b8c30
    Replacement::draft(0x1412b8ce0, "unknown_1412b8ce0", fn_1412b8ce0 as *const ()), // sub_1412b8ce0
    Replacement::draft_throws(0x1412b8d50, "unknown_1412b8d50", fn_1412b8d50 as *const (), Throws::Caught), // sub_1412b8d50
    Replacement::draft_throws(0x1412b8e00, "unknown_1412b8e00", fn_1412b8e00 as *const (), Throws::Caught), // sub_1412b8e00
    Replacement::draft_throws(0x1412b8eb0, "unknown_1412b8eb0", fn_1412b8eb0 as *const (), Throws::Caught), // sub_1412b8eb0
    Replacement::draft(0x1412b8f60, "unknown_1412b8f60", fn_1412b8f60 as *const ()), // sub_1412b8f60
    Replacement::draft(0x1412b9010, "unknown_1412b9010", fn_1412b9010 as *const ()), // sub_1412b9010
    Replacement::draft(0x1412b90c0, "unknown_1412b90c0", fn_1412b90c0 as *const ()), // sub_1412b90c0
    Replacement::draft_throws(0x1412b9180, "unknown_1412b9180", fn_1412b9180 as *const (), Throws::Discard), // sub_1412b9180
    Replacement::draft_throws(0x1412b9220, "unknown_1412b9220", tpl_2091::<0x1438817a8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1433ea790, 0x1412bd090> as *const (), Throws::Discard), // sub_1412b9220
    Replacement::draft(0x1412b92c0, "unknown_1412b92c0", fn_1412b92c0 as *const ()), // sub_1412b92c0
    Replacement::draft(0x1412b9350, "unknown_1412b9350", fn_1412b9350 as *const ()), // sub_1412b9350
    Replacement::draft(0x1412b9410, "unknown_1412b9410", fn_1412b9410 as *const ()), // sub_1412b9410
    Replacement::draft(0x1412b94a0, "unknown_1412b94a0", fn_1412b94a0 as *const ()), // sub_1412b94a0
    Replacement::draft_throws(0x1412b9750, "unknown_1412b9750", fn_1412b9750 as *const (), Throws::Discard), // sub_1412b9750
    Replacement::draft(0x1412b99a0, "unknown_1412b99a0", fn_1412b99a0 as *const ()), // sub_1412b99a0
    Replacement::draft_throws(0x1412b9ac0, "unknown_1412b9ac0", fn_1412b9ac0 as *const (), Throws::Discard), // sub_1412b9ac0
    Replacement::draft(0x1412b9d10, "unknown_1412b9d10", fn_1412b9d10 as *const ()), // sub_1412b9d10
    Replacement::draft_throws(0x1412b9dc0, "unknown_1412b9dc0", fn_1412b9dc0 as *const (), Throws::Caught), // sub_1412b9dc0
    Replacement::draft_throws(0x1412ba560, "unknown_1412ba560", fn_1412ba560 as *const (), Throws::Caught), // sub_1412ba560
    Replacement::draft_throws(0x1412bac40, "unknown_1412bac40", fn_1412bac40 as *const (), Throws::Caught), // sub_1412bac40
    Replacement::draft(0x1412bb2f0, "unknown_1412bb2f0", fn_1412bb2f0 as *const ()), // sub_1412bb2f0
    Replacement::draft(0x1412bb440, "unknown_1412bb440", fn_1412bb440 as *const ()), // sub_1412bb440
    Replacement::draft(0x1412bb570, "unknown_1412bb570", fn_1412bb570 as *const ()), // sub_1412bb570
    Replacement::draft_throws(0x1412bb6b0, "unknown_1412bb6b0", fn_1412bb6b0 as *const (), Throws::Discard), // sub_1412bb6b0
    Replacement::draft_throws(0x1412bb7e0, "unknown_1412bb7e0", fn_1412bb7e0 as *const (), Throws::Discard), // sub_1412bb7e0
    Replacement::draft(0x1412bba20, "unknown_1412bba20", fn_1412bba20 as *const ()), // sub_1412bba20
    Replacement::draft(0x1412bbac0, "unknown_1412bbac0", fn_1412bbac0 as *const ()), // sub_1412bbac0
    Replacement::draft(0x1412bbbb0, "unknown_1412bbbb0", fn_1412bbbb0 as *const ()), // sub_1412bbbb0
    Replacement::draft(0x1412bbc70, "unknown_1412bbc70", fn_1412bbc70 as *const ()), // sub_1412bbc70
    Replacement::draft_throws(0x1412bbcf0, "unknown_1412bbcf0", fn_1412bbcf0 as *const (), Throws::Caught), // sub_1412bbcf0
    Replacement::draft_throws(0x1412bbda0, "unknown_1412bbda0", fn_1412bbda0 as *const (), Throws::Caught), // sub_1412bbda0
    Replacement::draft_throws(0x1412bbe50, "unknown_1412bbe50", fn_1412bbe50 as *const (), Throws::Caught), // sub_1412bbe50
    Replacement::draft(0x1412bbf00, "unknown_1412bbf00", fn_1412bbf00 as *const ()), // sub_1412bbf00
    Replacement::draft(0x1412bbfc0, "unknown_1412bbfc0", fn_1412bbfc0 as *const ()), // sub_1412bbfc0
    Replacement::draft(0x1412bc070, "unknown_1412bc070", fn_1412bc070 as *const ()), // sub_1412bc070
    Replacement::draft_throws(0x1412bc130, "unknown_1412bc130", fn_1412bc130 as *const (), Throws::Discard), // sub_1412bc130
    Replacement::draft_throws(0x1412bc1e0, "unknown_1412bc1e0", tpl_2091::<0x143881790, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1433ea558, 0x1412bd500> as *const (), Throws::Discard), // sub_1412bc1e0
    Replacement::draft_throws(0x1412bc280, "unknown_1412bc280", fn_1412bc280 as *const (), Throws::Caught), // sub_1412bc280
    Replacement::draft_throws(0x1412bc560, "unknown_1412bc560", fn_1412bc560 as *const (), Throws::Caught), // sub_1412bc560
    Replacement::draft(0x1412bc850, "unknown_1412bc850", fn_1412bc850 as *const ()), // sub_1412bc850
    Replacement::draft(0x1412bcb80, "unknown_1412bcb80", fn_1412bcb80 as *const ()), // sub_1412bcb80
    Replacement::draft(0x1412bceb0, "unknown_1412bceb0", tpl_9681::<0x1412bdbe0, 0x14183acc8> as *const ()), // sub_1412bceb0
    Replacement::draft_throws(0x1412bd060, "unknown_1412bd060", tpl_838::<0x1412be2e0> as *const (), Throws::Discard), // sub_1412bd060
    Replacement::draft_throws(0x1412bd070, "unknown_1412bd070", tpl_838::<0x1412be780> as *const (), Throws::Discard), // sub_1412bd070
    Replacement::draft_throws(0x1412bd080, "unknown_1412bd080", tpl_838::<0x1412bebf0> as *const (), Throws::Discard), // sub_1412bd080
    Replacement::draft_throws(0x1412bd090, "unknown_1412bd090", tpl_9694::<0x20, 0x8, 0x1412bf050> as *const (), Throws::Discard), // sub_1412bd090
    Replacement::draft(0x1412bd0e0, "unknown_1412bd0e0", fn_1412bd0e0 as *const ()), // sub_1412bd0e0
    Replacement::draft_throws(0x1412bd160, "unknown_1412bd160", fn_1412bd160 as *const (), Throws::Caught), // sub_1412bd160
    Replacement::draft_throws(0x1412bd310, "unknown_1412bd310", fn_1412bd310 as *const (), Throws::Caught), // sub_1412bd310
    Replacement::draft_throws(0x1412bd4d0, "unknown_1412bd4d0", tpl_838::<0x1412bf120> as *const (), Throws::Discard), // sub_1412bd4d0
    Replacement::draft_throws(0x1412bd4e0, "unknown_1412bd4e0", tpl_838::<0x1412bf420> as *const (), Throws::Discard), // sub_1412bd4e0
    Replacement::draft_throws(0x1412bd4f0, "unknown_1412bd4f0", tpl_838::<0x1412bf6e0> as *const (), Throws::Discard), // sub_1412bd4f0
    Replacement::draft_throws(0x1412bd500, "unknown_1412bd500", fn_1412bd500 as *const (), Throws::Discard), // sub_1412bd500
    Replacement::draft(0x1412bd550, "unknown_1412bd550", tpl_3310::<0x38, 0x10, 0x18, 0x1433eacd0, 0x8, 0x1401f3d90, 0x20, 0x28, 0x1433eaca0, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412bd550
    Replacement::draft(0x1412bd620, "unknown_1412bd620", tpl_3310::<0x38, 0x10, 0x18, 0x1433eace8, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433eacb8, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412bd620
    Replacement::draft(0x1412bd6f0, "unknown_1412bd6f0", tpl_3310::<0x38, 0x10, 0x18, 0x1433ea8f0, 0x8, 0x1412900a0, 0x20, 0x28, 0x1433eb080, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412bd6f0
    Replacement::draft(0x1412bd7c0, "unknown_1412bd7c0", tpl_3310::<0x38, 0x10, 0x18, 0x1433ea908, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433eb098, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412bd7c0
    Replacement::draft(0x1412bd890, "unknown_1412bd890", fn_1412bd890 as *const ()), // sub_1412bd890
    Replacement::draft(0x1412bd9e0, "unknown_1412bd9e0", fn_1412bd9e0 as *const ()), // sub_1412bd9e0
    Replacement::draft(0x1412bdae0, "unknown_1412bdae0", fn_1412bdae0 as *const ()), // sub_1412bdae0
    Replacement::draft_throws(0x1412bdd40, "unknown_1412bdd40", fn_1412bdd40 as *const (), Throws::Caught), // sub_1412bdd40
    Replacement::draft_throws(0x1412be010, "unknown_1412be010", fn_1412be010 as *const (), Throws::Caught), // sub_1412be010
    Replacement::draft_throws(0x1412be2e0, "unknown_1412be2e0", fn_1412be2e0 as *const (), Throws::Caught), // sub_1412be2e0
    Replacement::draft_throws(0x1412be780, "unknown_1412be780", fn_1412be780 as *const (), Throws::Caught), // sub_1412be780
    Replacement::draft_throws(0x1412bebf0, "unknown_1412bebf0", fn_1412bebf0 as *const (), Throws::Caught), // sub_1412bebf0
    Replacement::draft_throws(0x1412bf050, "unknown_1412bf050", fn_1412bf050 as *const (), Throws::Caught), // sub_1412bf050
    Replacement::draft_throws(0x1412bf120, "unknown_1412bf120", fn_1412bf120 as *const (), Throws::Caught), // sub_1412bf120
    Replacement::draft_throws(0x1412bf420, "unknown_1412bf420", fn_1412bf420 as *const (), Throws::Caught), // sub_1412bf420
    Replacement::draft_throws(0x1412bf6e0, "unknown_1412bf6e0", fn_1412bf6e0 as *const (), Throws::Caught), // sub_1412bf6e0
    Replacement::draft_throws(0x1412bf980, "unknown_1412bf980", fn_1412bf980 as *const (), Throws::Caught), // sub_1412bf980
    Replacement::draft(0x1412bfa50, "unknown_1412bfa50", fn_1412bfa50 as *const ()), // sub_1412bfa50
    Replacement::draft_throws(0x1412bfc80, "unknown_1412bfc80", fn_1412bfc80 as *const (), Throws::Caught), // sub_1412bfc80
    Replacement::draft_throws(0x1412c0150, "unknown_1412c0150", fn_1412c0150 as *const (), Throws::Caught), // sub_1412c0150
    Replacement::draft_throws(0x1412c05e0, "unknown_1412c05e0", fn_1412c05e0 as *const (), Throws::Caught), // sub_1412c05e0
    Replacement::draft_throws(0x1412c0a70, "unknown_1412c0a70", fn_1412c0a70 as *const (), Throws::Caught), // sub_1412c0a70
    Replacement::draft_throws(0x1412c0f40, "unknown_1412c0f40", fn_1412c0f40 as *const (), Throws::Caught), // sub_1412c0f40
    Replacement::draft_throws(0x1412c13d0, "unknown_1412c13d0", fn_1412c13d0 as *const (), Throws::Caught), // sub_1412c13d0
    Replacement::draft_throws(0x1412c1860, "unknown_1412c1860", fn_1412c1860 as *const (), Throws::Caught), // sub_1412c1860
    Replacement::draft_throws(0x1412c1ac0, "unknown_1412c1ac0", fn_1412c1ac0 as *const (), Throws::Caught), // sub_1412c1ac0
    Replacement::draft_throws(0x1412c1d20, "unknown_1412c1d20", fn_1412c1d20 as *const (), Throws::Caught), // sub_1412c1d20
    Replacement::draft_throws(0x1412c1f80, "unknown_1412c1f80", fn_1412c1f80 as *const (), Throws::Caught), // sub_1412c1f80
    Replacement::draft_throws(0x1412c2170, "unknown_1412c2170", fn_1412c2170 as *const (), Throws::Caught), // sub_1412c2170
    Replacement::draft_throws(0x1412c22b0, "unknown_1412c22b0", fn_1412c22b0 as *const (), Throws::Caught), // sub_1412c22b0
    Replacement::draft_throws(0x1412c23f0, "unknown_1412c23f0", fn_1412c23f0 as *const (), Throws::Caught), // sub_1412c23f0
    Replacement::draft_throws(0x1412c2530, "unknown_1412c2530", fn_1412c2530 as *const (), Throws::Caught), // sub_1412c2530
    Replacement::draft_throws(0x1412c2720, "unknown_1412c2720", fn_1412c2720 as *const (), Throws::Caught), // sub_1412c2720
    Replacement::draft_throws(0x1412c2c90, "unknown_1412c2c90", fn_1412c2c90 as *const (), Throws::Caught), // sub_1412c2c90
    Replacement::draft_throws(0x1412c31d0, "unknown_1412c31d0", fn_1412c31d0 as *const (), Throws::Caught), // sub_1412c31d0
    Replacement::draft_throws(0x1412c3520, "unknown_1412c3520", fn_1412c3520 as *const (), Throws::Caught), // sub_1412c3520
    Replacement::draft_throws(0x1412c3990, "unknown_1412c3990", fn_1412c3990 as *const (), Throws::Caught), // sub_1412c3990
    Replacement::draft_throws(0x1412c3dc0, "unknown_1412c3dc0", fn_1412c3dc0 as *const (), Throws::Caught), // sub_1412c3dc0
    Replacement::draft(0x1412c4180, "puppet_1412c4180", fn_1412c4180 as *const ()), // puppet::RBFResource::puppet::RBFResource::vfunc_0
    Replacement::draft(0x1412c41f0, "puppet_1412c41f0", tpl_23623::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433eb350, 0x8> as *const ()), // sub_1412c41f0
    Replacement::draft(0x1412c4290, "puppet_1412c4290", tpl_13469::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1433eb350, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_1412c4290
    Replacement::draft(0x1412c4340, "puppet_1412c4340", tpl_23624::<0x1438817c0, 0x14243b0b0, 0x10, 0x1402c93e0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412c4340
    Replacement::draft_throws(0x1412c43b0, "puppet_1412c43b0", tpl_23625::<0x1438817b4, 0x1438817c0, 0x1401e9fa0, 0x10, 0x1402c98f0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412c43b0
    Replacement::draft(0x1412c4410, "puppet_1412c4410", tpl_23624::<0x1438817c0, 0x14243b0b0, 0x10, 0x1402c99a0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412c4410
    Replacement::draft_throws(0x1412c4480, "puppet_1412c4480", tpl_23625::<0x1438817b4, 0x1438817c0, 0x1401e9b60, 0x10, 0x1402c9dc0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1412c4480
    Replacement::draft(0x1412c44e0, "puppet_1412c44e0", fn_1412c44e0 as *const ()), // sub_1412c44e0
    Replacement::draft_throws(0x1412c4560, "puppet_1412c4560", fn_1412c4560 as *const (), Throws::Caught), // sub_1412c4560
    Replacement::draft_throws(0x1412c4610, "puppet_1412c4610", fn_1412c4610 as *const (), Throws::Caught), // sub_1412c4610
    Replacement::draft_throws(0x1412c46c0, "puppet_1412c46c0", fn_1412c46c0 as *const (), Throws::Caught), // sub_1412c46c0
    Replacement::draft(0x1412c4770, "puppet_1412c4770", fn_1412c4770 as *const ()), // sub_1412c4770
    Replacement::draft(0x1412c4820, "puppet_1412c4820", fn_1412c4820 as *const ()), // sub_1412c4820
    Replacement::draft(0x1412c48b0, "puppet_1412c48b0", fn_1412c48b0 as *const ()), // sub_1412c48b0
    Replacement::draft_throws(0x1412c4950, "puppet_1412c4950", fn_1412c4950 as *const (), Throws::Discard), // sub_1412c4950
    Replacement::draft_throws(0x1412c4a00, "puppet_1412c4a00", fn_1412c4a00 as *const (), Throws::Discard), // sub_1412c4a00
    Replacement::draft(0x1412c4aa0, "puppet_1412c4aa0", tpl_23631::<0x10, 0xf8, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142428d70, 0x1433eb5c0, 0xb8, 0xc0, 0xc8, 0xd0> as *const ()), // sub_1412c4aa0
    Replacement::draft(0x1412c4b70, "puppet_1412c4b70", tpl_23632::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0xd0, 0x1433eb5c0, 0xffffffffffffff30, 0x142428d70, 0xffffffffffffffe8> as *const ()), // sub_1412c4b70
    Replacement::draft_throws(0x1412c4e00, "puppet_1412c4e00", fn_1412c4e00 as *const (), Throws::Caught), // sub_1412c4e00
    Replacement::draft_throws(0x1412c4f90, "puppet_1412c4f90", tpl_1905::<0x1438817b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412c4f90
    Replacement::draft(0x1412c5040, "puppet_1412c5040", tpl_570::<0x1401f2bc0, 0x1430b6bd0, 0x141834d30> as *const ()), // puppet::SkeletonMetadata::puppet::SkeletonMetadata::vfunc_0
    Replacement::draft(0x1412c50f0, "puppet_1412c50f0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433eb708, 0x8> as *const ()), // sub_1412c50f0
    Replacement::draft(0x1412c5170, "puppet_1412c5170", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433eb708, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412c5170
    Replacement::draft_throws(0x1412c5210, "puppet_1412c5210", fn_1412c5210 as *const (), Throws::Caught), // sub_1412c5210
    Replacement::draft_throws(0x1412c52a0, "puppet_1412c52a0", tpl_649::<0x1438817d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412c52a0
    Replacement::draft(0x1412c5320, "puppet_1412c5320", tpl_23631::<0x10, 0xf8, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142428d70, 0x1433eb9e8, 0xb8, 0xc0, 0xc8, 0xd0> as *const ()), // sub_1412c5320
    Replacement::draft(0x1412c53f0, "puppet_1412c53f0", fn_1412c53f0 as *const ()), // sub_1412c53f0
    Replacement::draft(0x1412c54e0, "puppet_1412c54e0", tpl_21357::<0x1438817e0, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1412c54e0
    Replacement::draft_throws(0x1412c5580, "puppet_1412c5580", tpl_21358::<0x1438817c8, 0x1438817e0, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412c5580
    Replacement::draft_throws(0x1412c5600, "puppet_1412c5600", tpl_21358::<0x1438817c8, 0x1438817e0, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1412c5600
    Replacement::draft_throws(0x1412c5680, "puppet_1412c5680", fn_1412c5680 as *const (), Throws::Caught), // sub_1412c5680
    Replacement::draft(0x1412c5710, "puppet_1412c5710", tpl_21359::<0x1438817c8, 0x1438817e0, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1412c5710
    Replacement::draft_throws(0x1412c5790, "puppet_1412c5790", tpl_21358::<0x1438817c8, 0x1438817e0, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1412c5790
    Replacement::draft_throws(0x1412c5810, "puppet_1412c5810", tpl_1905::<0x1438817e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412c5810
    Replacement::draft(0x1412c5920, "puppet_1412c5920", fn_1412c5920 as *const ()), // puppet::MotionDatabaseResource::puppet::MotionDatabaseResource::vfunc_0
    Replacement::draft(0x1412c59b0, "puppet_1412c59b0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433ebd78, 0x8> as *const ()), // sub_1412c59b0
    Replacement::draft(0x1412c5a30, "puppet_1412c5a30", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433ebd78, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412c5a30
    Replacement::draft_throws(0x1412c5ad0, "puppet_1412c5ad0", fn_1412c5ad0 as *const (), Throws::Caught), // sub_1412c5ad0
    Replacement::draft_throws(0x1412c5b60, "puppet_1412c5b60", tpl_1908::<0x1438817e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412c5b60
    Replacement::draft(0x1412c5be0, "puppet_1412c5be0", tpl_23029::<0x100, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1421c82e0> as *const ()), // sub_1412c5be0
    Replacement::draft(0x1412c5c60, "puppet_1412c5c60", tpl_23634::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1421c82e0> as *const ()), // sub_1412c5c60
    Replacement::draft_throws(0x1412c5ea0, "puppet_1412c5ea0", fn_1412c5ea0 as *const (), Throws::Caught), // sub_1412c5ea0
    Replacement::draft_throws(0x1412c6030, "puppet_1412c6030", tpl_1905::<0x1438817f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412c6030
    Replacement::draft(0x1412c6150, "puppet_1412c6150", fn_1412c6150 as *const ()), // puppet::MixerResource::puppet::MixerResource::vfunc_0
    Replacement::draft(0x1412c61d0, "puppet_1412c61d0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433ec828, 0x8> as *const ()), // sub_1412c61d0
    Replacement::draft(0x1412c6250, "puppet_1412c6250", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433ec828, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412c6250
    Replacement::draft_throws(0x1412c62f0, "puppet_1412c62f0", fn_1412c62f0 as *const (), Throws::Caught), // sub_1412c62f0
    Replacement::draft_throws(0x1412c6380, "puppet_1412c6380", tpl_1908::<0x143881800, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412c6380
    Replacement::draft(0x1412c6400, "puppet_1412c6400", fn_1412c6400 as *const ()), // sub_1412c6400
    Replacement::draft(0x1412c6480, "puppet_1412c6480", fn_1412c6480 as *const ()), // sub_1412c6480
    Replacement::draft_throws(0x1412c66c0, "puppet_1412c66c0", fn_1412c66c0 as *const (), Throws::Caught), // sub_1412c66c0
    Replacement::draft_throws(0x1412c6850, "puppet_1412c6850", tpl_1905::<0x143881810, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412c6850
    Replacement::draft(0x1412c6910, "puppet_1412c6910", fn_1412c6910 as *const ()), // sub_1412c6910
    Replacement::draft(0x1412c6ac0, "puppet_1412c6ac0", fn_1412c6ac0 as *const ()), // sub_1412c6ac0
    Replacement::draft(0x1412c6d30, "puppet_1412c6d30", tpl_19431::<0x140215e80, 0x1425371a0, 0x142533200> as *const ()), // sub_1412c6d30
    Replacement::draft(0x1412c6ec0, "puppet_1412c6ec0", fn_1412c6ec0 as *const ()), // sub_1412c6ec0
    Replacement::draft(0x1412c71f0, "puppet_1412c71f0", fn_1412c71f0 as *const ()), // sub_1412c71f0
    Replacement::draft(0x1412c7350, "puppet_1412c7350", fn_1412c7350 as *const ()), // sub_1412c7350
    Replacement::draft(0x1412c7610, "puppet_1412c7610", fn_1412c7610 as *const ()), // sub_1412c7610
    Replacement::draft(0x1412c7710, "puppet_1412c7710", fn_1412c7710 as *const ()), // sub_1412c7710
    Replacement::draft(0x1412c77e0, "puppet_1412c77e0", fn_1412c77e0 as *const ()), // puppet::GraphResource::puppet::GraphResource::vfunc_0
    Replacement::draft(0x1412c7890, "puppet_1412c7890", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433ece88, 0x8> as *const ()), // sub_1412c7890
    Replacement::draft(0x1412c7910, "puppet_1412c7910", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433ece88, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412c7910
    Replacement::draft_throws(0x1412c79b0, "puppet_1412c79b0", fn_1412c79b0 as *const (), Throws::Caught), // sub_1412c79b0
    Replacement::draft_throws(0x1412c7a40, "puppet_1412c7a40", tpl_1908::<0x143881828, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412c7a40
    Replacement::draft(0x1412c7ac0, "puppet_1412c7ac0", fn_1412c7ac0 as *const ()), // sub_1412c7ac0
    Replacement::draft(0x1412c7b40, "puppet_1412c7b40", fn_1412c7b40 as *const ()), // sub_1412c7b40
    Replacement::draft_throws(0x1412c7d80, "puppet_1412c7d80", fn_1412c7d80 as *const (), Throws::Caught), // sub_1412c7d80
    Replacement::draft_throws(0x1412c7f10, "puppet_1412c7f10", tpl_1905::<0x143881830, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412c7f10
    Replacement::draft(0x1412c7fd0, "puppet_1412c7fd0", fn_1412c7fd0 as *const ()), // sub_1412c7fd0
    Replacement::draft(0x1412c8000, "puppet_1412c8000", fn_1412c8000 as *const ()), // sub_1412c8000
    Replacement::draft(0x1412c84a0, "puppet_1412c84a0", fn_1412c84a0 as *const ()), // sub_1412c84a0
    Replacement::draft(0x1412c8950, "puppet_1412c8950", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433ed528, 0x8> as *const ()), // sub_1412c8950
    Replacement::draft(0x1412c89d0, "puppet_1412c89d0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433ed528, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412c89d0
    Replacement::draft_throws(0x1412c8a70, "puppet_1412c8a70", fn_1412c8a70 as *const (), Throws::Caught), // sub_1412c8a70
    Replacement::draft_throws(0x1412c8b00, "puppet_1412c8b00", tpl_1908::<0x143881858, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412c8b00
    Replacement::draft(0x1412c8b80, "puppet_1412c8b80", fn_1412c8b80 as *const ()), // sub_1412c8b80
    Replacement::draft(0x1412c8c30, "puppet_1412c8c30", fn_1412c8c30 as *const ()), // sub_1412c8c30
    Replacement::draft(0x1412c8d30, "puppet_1412c8d30", fn_1412c8d30 as *const ()), // sub_1412c8d30
    Replacement::draft_throws(0x1412c8de0, "puppet_1412c8de0", tpl_9557::<0x14388184c, 0x143881840, 0x1401e9fa0, 0x328, 0x338, 0x10, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412c8de0
    Replacement::draft(0x1412c8e80, "puppet_1412c8e80", tpl_19442::<0x143881840, 0x14243b0b0, 0x338, 0x10, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412c8e80
    Replacement::draft_throws(0x1412c8ef0, "puppet_1412c8ef0", tpl_9558::<0x14388184c, 0x143881840, 0x1401e9b60, 0x328, 0x10, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_1412c8ef0
    Replacement::draft_throws(0x1412c8f90, "puppet_1412c8f90", fn_1412c8f90 as *const (), Throws::Caught), // sub_1412c8f90
    Replacement::draft_throws(0x1412c9040, "puppet_1412c9040", fn_1412c9040 as *const (), Throws::Caught), // sub_1412c9040
    Replacement::draft_throws(0x1412c90f0, "puppet_1412c90f0", fn_1412c90f0 as *const (), Throws::Caught), // sub_1412c90f0
    Replacement::draft(0x1412c91a0, "puppet_1412c91a0", fn_1412c91a0 as *const ()), // sub_1412c91a0
    Replacement::draft(0x1412c9240, "puppet_1412c9240", tpl_9561::<0x14388184c, 0x143881840, 0x1401e9970, 0x338, 0x10, 0xc0, 0x1401e9860> as *const ()), // sub_1412c9240
    Replacement::draft(0x1412c92b0, "puppet_1412c92b0", tpl_19442::<0x143881840, 0x14243b0b0, 0x338, 0x10, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412c92b0
    Replacement::draft_throws(0x1412c9320, "puppet_1412c9320", tpl_9562::<0x14388184c, 0x143881840, 0x1401e9770, 0x338, 0x10, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_1412c9320
    Replacement::draft_throws(0x1412c9380, "puppet_1412c9380", tpl_9563::<0x143881840, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x10, 0x1433ecf30, 0x1402c4730> as *const (), Throws::Discard), // sub_1412c9380
    Replacement::draft(0x1412c9440, "puppet_1412c9440", tpl_23631::<0x10, 0xf8, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142428d70, 0x1433ed410, 0xb8, 0xc0, 0xc8, 0xd0> as *const ()), // sub_1412c9440
    Replacement::draft(0x1412c9510, "puppet_1412c9510", tpl_23632::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0xd0, 0x1433ed410, 0xffffffffffffff30, 0x142428d70, 0xffffffffffffffe8> as *const ()), // sub_1412c9510
    Replacement::draft_throws(0x1412c97a0, "puppet_1412c97a0", fn_1412c97a0 as *const (), Throws::Caught), // sub_1412c97a0
    Replacement::draft_throws(0x1412c9930, "puppet_1412c9930", tpl_1905::<0x143881850, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412c9930
    Replacement::draft(0x1412c9a90, "puppet_1412c9a90", fn_1412c9a90 as *const ()), // puppet::BlendSpaceResource::puppet::BlendSpaceResource::vfunc_0
    Replacement::draft(0x1412c9b00, "puppet_1412c9b00", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433edd48, 0x8> as *const ()), // sub_1412c9b00
    Replacement::draft(0x1412c9b80, "puppet_1412c9b80", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433edd48, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412c9b80
    Replacement::draft_throws(0x1412c9c20, "puppet_1412c9c20", fn_1412c9c20 as *const (), Throws::Caught), // sub_1412c9c20
    Replacement::draft_throws(0x1412c9cb0, "puppet_1412c9cb0", tpl_1908::<0x143881870, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412c9cb0
    Replacement::draft(0x1412c9d30, "puppet_1412c9d30", tpl_23029::<0x100, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1421df530> as *const ()), // sub_1412c9d30
    Replacement::draft(0x1412c9db0, "puppet_1412c9db0", tpl_23634::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1421df530> as *const ()), // sub_1412c9db0
    Replacement::draft_throws(0x1412c9ff0, "puppet_1412c9ff0", fn_1412c9ff0 as *const (), Throws::Caught), // sub_1412c9ff0
    Replacement::draft_throws(0x1412ca180, "puppet_1412ca180", tpl_1905::<0x143881860, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412ca180
    Replacement::draft(0x1412ca240, "unknown_1412ca240", fn_1412ca240 as *const ()), // sub_1412ca240
    Replacement::draft(0x1412ca310, "unknown_1412ca310", fn_1412ca310 as *const ()), // sub_1412ca310
    Replacement::draft(0x1412ca420, "unknown_1412ca420", fn_1412ca420 as *const ()), // sub_1412ca420
    Replacement::draft(0x1412ca4f0, "unknown_1412ca4f0", fn_1412ca4f0 as *const ()), // sub_1412ca4f0
    Replacement::draft(0x1412caa30, "unknown_1412caa30", fn_1412caa30 as *const ()), // sub_1412caa30
    Replacement::draft(0x1412cafc0, "unknown_1412cafc0", fn_1412cafc0 as *const ()), // sub_1412cafc0
    Replacement::draft(0x1412ccd70, "unknown_1412ccd70", fn_1412ccd70 as *const ()), // sub_1412ccd70
    Replacement::draft(0x1412ccfb0, "unknown_1412ccfb0", fn_1412ccfb0 as *const ()), // sub_1412ccfb0
    Replacement::draft(0x1412cd270, "unknown_1412cd270", fn_1412cd270 as *const ()), // sub_1412cd270
    Replacement::draft(0x1412cd590, "unknown_1412cd590", fn_1412cd590 as *const ()), // sub_1412cd590
    Replacement::draft(0x1412cd7e0, "unknown_1412cd7e0", fn_1412cd7e0 as *const ()), // sub_1412cd7e0
    Replacement::draft(0x1412cd990, "unknown_1412cd990", fn_1412cd990 as *const ()), // sub_1412cd990
    Replacement::draft(0x1412cdb30, "unknown_1412cdb30", fn_1412cdb30 as *const ()), // sub_1412cdb30
    Replacement::draft(0x1412cdce0, "unknown_1412cdce0", fn_1412cdce0 as *const ()), // sub_1412cdce0
    Replacement::draft(0x1412ce020, "unknown_1412ce020", fn_1412ce020 as *const ()), // sub_1412ce020
    Replacement::draft(0x1412ce250, "unknown_1412ce250", fn_1412ce250 as *const ()), // sub_1412ce250
    Replacement::draft(0x1412ce9d0, "unknown_1412ce9d0", fn_1412ce9d0 as *const ()), // sub_1412ce9d0
    Replacement::draft(0x1412cece0, "unknown_1412cece0", fn_1412cece0 as *const ()), // sub_1412cece0
    Replacement::draft(0x1412cef60, "unknown_1412cef60", fn_1412cef60 as *const ()), // sub_1412cef60
    Replacement::draft(0x1412cf050, "unknown_1412cf050", fn_1412cf050 as *const ()), // sub_1412cf050
    Replacement::draft(0x1412cf080, "unknown_1412cf080", fn_1412cf080 as *const ()), // sub_1412cf080
    Replacement::draft(0x1412cf0e0, "unknown_1412cf0e0", fn_1412cf0e0 as *const ()), // sub_1412cf0e0
    Replacement::draft(0x1412cf150, "unknown_1412cf150", fn_1412cf150 as *const ()), // sub_1412cf150
    Replacement::draft(0x1412cf260, "unknown_1412cf260", fn_1412cf260 as *const ()), // sub_1412cf260
    Replacement::draft(0x1412cf2f0, "unknown_1412cf2f0", fn_1412cf2f0 as *const ()), // sub_1412cf2f0
    Replacement::draft(0x1412cf300, "unknown_1412cf300", fn_1412cf300 as *const ()), // sub_1412cf300
    Replacement::draft(0x1412cf360, "unknown_1412cf360", fn_1412cf360 as *const ()), // sub_1412cf360
    Replacement::draft(0x1412cf3d0, "unknown_1412cf3d0", fn_1412cf3d0 as *const ()), // sub_1412cf3d0
    Replacement::draft_throws(0x1412cf460, "unknown_1412cf460", fn_1412cf460 as *const (), Throws::Caught), // sub_1412cf460
    Replacement::draft_throws(0x1412cf5e0, "unknown_1412cf5e0", fn_1412cf5e0 as *const (), Throws::Caught), // sub_1412cf5e0
    Replacement::draft_throws(0x1412cf610, "unknown_1412cf610", tpl_23670::<0x14397ff18, 0x142258080> as *const (), Throws::Discard), // sub_1412cf610
    Replacement::draft(0x1412cf630, "unknown_1412cf630", fn_1412cf630 as *const ()), // sub_1412cf630
    Replacement::draft(0x1412cf6a0, "unknown_1412cf6a0", fn_1412cf6a0 as *const ()), // sub_1412cf6a0
    Replacement::draft(0x1412cf730, "unknown_1412cf730", fn_1412cf730 as *const ()), // sub_1412cf730
    Replacement::draft_throws(0x1412cf770, "unknown_1412cf770", fn_1412cf770 as *const (), Throws::Discard), // sub_1412cf770
    Replacement::draft_throws(0x1412cf7b0, "viewvolumelighting_1412cf7b0", fn_1412cf7b0 as *const (), Throws::Caught), // ViewVolumeLighting::ViewVolumeLighting::m_pLighting
    Replacement::draft_throws(0x1412cf870, "viewvolumelighting_1412cf870", fn_1412cf870 as *const (), Throws::Caught), // ViewVolumeLighting::ViewVolumeLighting::m_pFiltering
    Replacement::draft_throws(0x1412cf930, "viewvolumelighting_1412cf930", fn_1412cf930 as *const (), Throws::Caught), // ViewVolumeLighting::ViewVolumeLighting::m_pParticipatingMedia
    Replacement::draft(0x1412cf9f0, "unknown_1412cf9f0", fn_1412cf9f0 as *const ()), // sub_1412cf9f0
    Replacement::draft(0x1412cfa80, "unknown_1412cfa80", fn_1412cfa80 as *const ()), // sub_1412cfa80
    Replacement::draft(0x1412cfb40, "unknown_1412cfb40", tpl_23678::<0x1412cfa80, 0x140f6c9e0, 0x1434268d0, 0x1421f0050> as *const ()), // sub_1412cfb40
    Replacement::draft(0x1412cfda0, "unknown_1412cfda0", tpl_23678::<0x1403b2e70, 0x140f6c9e0, 0x1434268d0, 0x1421f0050> as *const ()), // sub_1412cfda0
    Replacement::draft(0x1412d0000, "unknown_1412d0000", fn_1412d0000 as *const ()), // sub_1412d0000
    Replacement::draft(0x1412d0310, "rend_1412d0310", fn_1412d0310 as *const ()), // rend::VideoStreamerFactory::rend::VideoStreamerFactory::vfunc_0
    Replacement::draft(0x1412d03d0, "rend_1412d03d0", fn_1412d03d0 as *const ()), // sub_1412d03d0
    Replacement::draft_throws(0x1412d03e0, "rend_1412d03e0", fn_1412d03e0 as *const (), Throws::Discard), // sub_1412d03e0
    Replacement::draft(0x1412d0480, "rend_1412d0480", fn_1412d0480 as *const ()), // sub_1412d0480
    Replacement::draft_throws(0x1412d0520, "rend_1412d0520", fn_1412d0520 as *const (), Throws::Discard), // rend::VideoStreamer::rend::VideoStreamer::vfunc_0
    Replacement::draft(0x1412d05a0, "unknown_1412d05a0", fn_1412d05a0 as *const ()), // sub_1412d05a0
    Replacement::draft_throws(0x1412d06d0, "unknown_1412d06d0", fn_1412d06d0 as *const (), Throws::Caught), // sub_1412d06d0
    Replacement::draft(0x1412d0890, "unknown_1412d0890", fn_1412d0890 as *const ()), // sub_1412d0890
    Replacement::draft(0x1412dce90, "unknown_1412dce90", fn_1412dce90 as *const ()), // sub_1412dce90
    Replacement::draft(0x1412dcf30, "unknown_1412dcf30", fn_1412dcf30 as *const ()), // sub_1412dcf30
    Replacement::draft(0x1412dd010, "unknown_1412dd010", fn_1412dd010 as *const ()), // sub_1412dd010
    Replacement::draft_throws(0x1412dd070, "unknown_1412dd070", fn_1412dd070 as *const (), Throws::Discard), // sub_1412dd070
    Replacement::draft(0x1412dd110, "std_1412dd110", fn_1412dd110 as *const ()), // std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringbuf<char,std::char_traits<char>
    Replacement::draft(0x1412dd240, "std_1412dd240", fn_1412dd240 as *const ()), // std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringbuf<char,std::char_traits<char>
    Replacement::draft(0x1412dd3d0, "std_1412dd3d0", fn_1412dd3d0 as *const ()), // std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringbuf<char,std::char_traits<char>
    Replacement::draft(0x1412dd440, "std_1412dd440", fn_1412dd440 as *const ()), // std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringbuf<char,std::char_traits<char>
    Replacement::draft_throws(0x1412dd4a0, "std_1412dd4a0", fn_1412dd4a0 as *const (), Throws::Discard), // std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringbuf<char,std::char_traits<char>
    Replacement::draft(0x1412dd620, "std_1412dd620", fn_1412dd620 as *const ()), // sub_1412dd620
    Replacement::draft(0x1412dd710, "std_1412dd710", fn_1412dd710 as *const ()), // sub_1412dd710
    Replacement::draft(0x1412dd770, "std_1412dd770", fn_1412dd770 as *const ()), // sub_1412dd770
    Replacement::draft(0x1412dd830, "std_1412dd830", tpl_23741::<0x1412dd620, 0x141834d30> as *const ()), // std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringbuf<char,std::char_traits<char>
    Replacement::draft(0x1412dd870, "unknown_1412dd870", fn_1412dd870 as *const ()), // sub_1412dd870
    Replacement::draft(0x1412dd8b0, "unknown_1412dd8b0", fn_1412dd8b0 as *const ()), // sub_1412dd8b0
    Replacement::draft_throws(0x1412dd990, "unknown_1412dd990", fn_1412dd990 as *const (), Throws::Discard), // sub_1412dd990
    Replacement::draft(0x1412dd9b0, "opentelemetry_1412dd9b0", fn_1412dd9b0 as *const ()), // sub_1412dd9b0
    Replacement::draft(0x1412dda50, "unknown_1412dda50", fn_1412dda50 as *const ()), // sub_1412dda50
    Replacement::draft_throws(0x1412dda90, "unknown_1412dda90", fn_1412dda90 as *const (), Throws::Discard), // sub_1412dda90
    Replacement::draft_throws(0x1412ddc30, "unknown_1412ddc30", fn_1412ddc30 as *const (), Throws::Discard), // sub_1412ddc30
    Replacement::draft_throws(0x1412ddd60, "unknown_1412ddd60", fn_1412ddd60 as *const (), Throws::Discard), // sub_1412ddd60
    Replacement::draft(0x1412dde20, "global_1412dde20", fn_1412dde20 as *const ()), // FID_conflict:_Change_array
    Replacement::draft(0x1412ddeb0, "unknown_1412ddeb0", fn_1412ddeb0 as *const ()), // sub_1412ddeb0
    Replacement::draft(0x1412ddf34, "std_1412ddf34", tpl_23749::<0xfffffffffffffffc, 0x1412dd870> as *const ()), // std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::std::basic_stringstream<char,std::char_traits
    Replacement::draft(0x1412ddf40, "std_1412ddf40", tpl_23750::<0x800> as *const ()), // sub_1412ddf40
    Replacement::draft(0x1412ddf50, "std_1412ddf50", tpl_23751::<0x140, 0x1412de4d0> as *const ()), // sub_1412ddf50
    Replacement::draft(0x1412ddfb0, "std_1412ddfb0", tpl_23751::<0x1b8, 0x1412de3f0> as *const ()), // sub_1412ddfb0
    Replacement::draft(0x1412de010, "std_1412de010", fn_1412de010 as *const ()), // sub_1412de010
    Replacement::draft(0x1412de0c0, "std_1412de0c0", fn_1412de0c0 as *const ()), // sub_1412de0c0
    Replacement::draft(0x1412de170, "std_1412de170", fn_1412de170 as *const ()), // sub_1412de170
    Replacement::draft(0x1412de2b0, "std_1412de2b0", fn_1412de2b0 as *const ()), // sub_1412de2b0
    Replacement::draft(0x1412de3f0, "std_1412de3f0", fn_1412de3f0 as *const ()), // sub_1412de3f0
    Replacement::draft(0x1412de4d0, "std_1412de4d0", fn_1412de4d0 as *const ()), // sub_1412de4d0
    Replacement::draft(0x1412de810, "std_1412de810", fn_1412de810 as *const ()), // sub_1412de810
    Replacement::draft(0x1412de9a0, "std_1412de9a0", fn_1412de9a0 as *const ()), // sub_1412de9a0
    Replacement::draft(0x1412deb50, "std_1412deb50", tpl_170::<0x1412df190> as *const ()), // sub_1412deb50
    Replacement::draft(0x1412deb60, "std_1412deb60", tpl_22260::<0x1412deb50, 0x1, 0x141834ee8> as *const ()), // sub_1412deb60
    Replacement::draft(0x1412deb80, "std_1412deb80", fn_1412deb80 as *const ()), // sub_1412deb80
    Replacement::draft(0x1412debd0, "std_1412debd0", fn_1412debd0 as *const ()), // sub_1412debd0
    Replacement::draft(0x1412dec60, "std_1412dec60", fn_1412dec60 as *const ()), // sub_1412dec60
    Replacement::draft(0x1412df190, "std_1412df190", tpl_10126::<0x8, 0x1412df250, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1412df190
    Replacement::draft_throws(0x1412df2c0, "std_1412df2c0", fn_1412df2c0 as *const (), Throws::Discard), // sub_1412df2c0
    Replacement::draft(0x1412df410, "std_1412df410", fn_1412df410 as *const ()), // sub_1412df410
    Replacement::draft(0x1412df790, "std_1412df790", tpl_22213::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1412df190> as *const ()), // sub_1412df790
    Replacement::draft(0x1412df900, "std_1412df900", fn_1412df900 as *const ()), // std::_Future_error_category2::std::_Future_error_category2::vfunc_3
    Replacement::draft(0x1412df910, "std_1412df910", fn_1412df910 as *const ()), // std::_System_error_category::std::_System_error_category::vfunc_5
    Replacement::draft(0x1412df950, "std_1412df950", fn_1412df950 as *const ()), // std::_System_error_category::std::_System_error_category::vfunc_4
    Replacement::draft(0x1412df970, "std_1412df970", fn_1412df970 as *const ()), // sub_1412df970
    Replacement::draft(0x1412df990, "std_1412df990", fn_1412df990 as *const ()), // sub_1412df990
    Replacement::draft_throws(0x1412dfa60, "std_1412dfa60", fn_1412dfa60 as *const (), Throws::Discard), // sub_1412dfa60
    Replacement::draft(0x1412dfaa0, "std_1412dfaa0", fn_1412dfaa0 as *const ()), // sub_1412dfaa0
    Replacement::draft(0x1412dfb00, "std_1412dfb00", fn_1412dfb00 as *const ()), // sub_1412dfb00
    Replacement::draft(0x1412dfb60, "std_1412dfb60", tpl_1::<0x143106a78> as *const ()), // std::_Generic_error_category::std::_Generic_error_category::vfunc_1
    Replacement::draft_throws(0x1412dfb70, "std_1412dfb70", fn_1412dfb70 as *const (), Throws::Discard), // std::_Generic_error_category::std::_Generic_error_category::vfunc_2
    Replacement::draft_throws(0x1412dfc20, "rend_1412dfc20", fn_1412dfc20 as *const (), Throws::Discard), // rend::TextureResource::rend::TextureResource::vfunc_0
    Replacement::draft(0x1412dfca0, "rend_1412dfca0", fn_1412dfca0 as *const ()), // sub_1412dfca0
    Replacement::draft_throws(0x1412dfcb0, "rend_1412dfcb0", fn_1412dfcb0 as *const (), Throws::Discard), // sub_1412dfcb0
    Replacement::draft_throws(0x1412dfd10, "rend_1412dfd10", tpl_23777::<0x8, 0x14206b020, 0x142442de0> as *const (), Throws::Discard), // sub_1412dfd10
    Replacement::draft(0x1412dfea0, "rend_1412dfea0", fn_1412dfea0 as *const ()), // sub_1412dfea0
    Replacement::draft_throws(0x1412e0360, "rend_1412e0360", fn_1412e0360 as *const (), Throws::Discard), // sub_1412e0360
    Replacement::draft(0x1412e0700, "rend_1412e0700", fn_1412e0700 as *const ()), // sub_1412e0700
    Replacement::draft(0x1412e0d40, "rend_1412e0d40", fn_1412e0d40 as *const ()), // sub_1412e0d40
    Replacement::draft_throws(0x1412e0eb0, "rend_1412e0eb0", fn_1412e0eb0 as *const (), Throws::Caught), // sub_1412e0eb0
    Replacement::draft_throws(0x1412e1230, "rend_1412e1230", fn_1412e1230 as *const (), Throws::Caught), // sub_1412e1230
    Replacement::draft_throws(0x1412e15b0, "rend_1412e15b0", fn_1412e15b0 as *const (), Throws::Caught), // sub_1412e15b0
    Replacement::draft(0x1412e1930, "rend_1412e1930", fn_1412e1930 as *const ()), // sub_1412e1930
    Replacement::draft(0x1412e1a80, "rend_1412e1a80", fn_1412e1a80 as *const ()), // sub_1412e1a80
    Replacement::draft(0x1412e1c80, "rend_1412e1c80", fn_1412e1c80 as *const ()), // sub_1412e1c80
    Replacement::draft_throws(0x1412e1e80, "rend_1412e1e80", fn_1412e1e80 as *const (), Throws::Discard), // sub_1412e1e80
    Replacement::draft_throws(0x1412e20d0, "rend_1412e20d0", fn_1412e20d0 as *const (), Throws::Discard), // sub_1412e20d0
    Replacement::draft(0x1412e2420, "rend_1412e2420", fn_1412e2420 as *const ()), // sub_1412e2420
    Replacement::draft(0x1412e24e0, "rend_1412e24e0", tpl_666::<0x1e0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1422098e0> as *const ()), // sub_1412e24e0
    Replacement::draft(0x1412e2560, "rend_1412e2560", fn_1412e2560 as *const ()), // sub_1412e2560
    Replacement::draft_throws(0x1412e27a0, "rend_1412e27a0", fn_1412e27a0 as *const (), Throws::Caught), // sub_1412e27a0
    Replacement::draft_throws(0x1412e2930, "rend_1412e2930", tpl_1905::<0x143881960, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412e2930
    Replacement::draft(0x1412e29e0, "rend_1412e29e0", fn_1412e29e0 as *const ()), // rend::TextureMetadata::rend::TextureMetadata::vfunc_0
    Replacement::draft_throws(0x1412e2a20, "rend_1412e2a20", fn_1412e2a20 as *const (), Throws::Caught), // sub_1412e2a20
    Replacement::draft_throws(0x1412e2b90, "rend_1412e2b90", fn_1412e2b90 as *const (), Throws::Caught), // sub_1412e2b90
    Replacement::draft_throws(0x1412e2cc0, "rend_1412e2cc0", fn_1412e2cc0 as *const (), Throws::Caught), // sub_1412e2cc0
    Replacement::draft_throws(0x1412e2df0, "rend_1412e2df0", fn_1412e2df0 as *const (), Throws::Caught), // sub_1412e2df0
    Replacement::draft_throws(0x1412e2f10, "rend_1412e2f10", fn_1412e2f10 as *const (), Throws::Discard), // sub_1412e2f10
    Replacement::draft_throws(0x1412e2f50, "rend_1412e2f50", fn_1412e2f50 as *const (), Throws::Caught), // sub_1412e2f50
    Replacement::draft_throws(0x1412e30b0, "rend_1412e30b0", fn_1412e30b0 as *const (), Throws::Caught), // sub_1412e30b0
    Replacement::draft_throws(0x1412e3210, "rend_1412e3210", fn_1412e3210 as *const (), Throws::Caught), // sub_1412e3210
    Replacement::draft_throws(0x1412e33f0, "rend_1412e33f0", fn_1412e33f0 as *const (), Throws::Caught), // sub_1412e33f0
    Replacement::draft_throws(0x1412e35d0, "rend_1412e35d0", fn_1412e35d0 as *const (), Throws::Caught), // sub_1412e35d0
    Replacement::draft_throws(0x1412e3870, "rend_1412e3870", fn_1412e3870 as *const (), Throws::Caught), // sub_1412e3870
    Replacement::draft(0x1412e3940, "rend_1412e3940", fn_1412e3940 as *const ()), // sub_1412e3940
    Replacement::draft_throws(0x1412e3ac0, "rend_1412e3ac0", fn_1412e3ac0 as *const (), Throws::Caught), // sub_1412e3ac0
    Replacement::draft(0x1412e3fc0, "rend_1412e3fc0", fn_1412e3fc0 as *const ()), // sub_1412e3fc0
    Replacement::draft_throws(0x1412e4150, "rend_1412e4150", fn_1412e4150 as *const (), Throws::Caught), // sub_1412e4150
    Replacement::draft(0x1412e4610, "rend_1412e4610", fn_1412e4610 as *const ()), // sub_1412e4610
    Replacement::draft_throws(0x1412e4760, "rend_1412e4760", fn_1412e4760 as *const (), Throws::Caught), // sub_1412e4760
    Replacement::draft_throws(0x1412e4c20, "rend_1412e4c20", fn_1412e4c20 as *const (), Throws::Discard), // sub_1412e4c20
    Replacement::draft_throws(0x1412e4c50, "rend_1412e4c50", tpl_23800::<0x20, 0x8, 0x1412e4c70> as *const (), Throws::Discard), // sub_1412e4c50
    Replacement::draft_throws(0x1412e4c70, "rend_1412e4c70", fn_1412e4c70 as *const (), Throws::Caught), // sub_1412e4c70
    Replacement::draft_throws(0x1412e4d40, "rend_1412e4d40", fn_1412e4d40 as *const (), Throws::Caught), // sub_1412e4d40
    Replacement::draft_throws(0x1412e5210, "rend_1412e5210", fn_1412e5210 as *const (), Throws::Caught), // sub_1412e5210
    Replacement::draft_throws(0x1412e56a0, "rend_1412e56a0", fn_1412e56a0 as *const (), Throws::Caught), // sub_1412e56a0
    Replacement::draft_throws(0x1412e5b30, "rend_1412e5b30", fn_1412e5b30 as *const (), Throws::Discard), // rend::TextureAtlasInterface::rend::TextureAtlasInterface::vfunc_0
    Replacement::draft(0x1412e5c70, "unknown_1412e5c70", tpl_365::<0x1412e5cd0> as *const ()), // sub_1412e5c70
    Replacement::draft(0x1412e5c80, "unknown_1412e5c80", tpl_20552::<0x8, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_1412e5c80
    Replacement::draft(0x1412e5cd0, "unknown_1412e5cd0", fn_1412e5cd0 as *const ()), // sub_1412e5cd0
    Replacement::draft(0x1412e5db0, "unknown_1412e5db0", fn_1412e5db0 as *const ()), // sub_1412e5db0
    Replacement::draft(0x1412e5e30, "unknown_1412e5e30", fn_1412e5e30 as *const ()), // sub_1412e5e30
    Replacement::draft_throws(0x1412e5f70, "sparsehash_1412e5f70", fn_1412e5f70 as *const (), Throws::Caught), // sub_1412e5f70
    Replacement::draft(0x1412e61b0, "unknown_1412e61b0", fn_1412e61b0 as *const ()), // sub_1412e61b0
    Replacement::draft(0x1412e6220, "unknown_1412e6220", fn_1412e6220 as *const ()), // sub_1412e6220
    Replacement::draft(0x1412e6280, "unknown_1412e6280", fn_1412e6280 as *const ()), // sub_1412e6280
    Replacement::draft(0x1412e6300, "unknown_1412e6300", fn_1412e6300 as *const ()), // sub_1412e6300
    Replacement::draft_throws(0x1412e65d0, "unknown_1412e65d0", fn_1412e65d0 as *const (), Throws::Caught), // sub_1412e65d0
    Replacement::draft_throws(0x1412e6790, "unknown_1412e6790", fn_1412e6790 as *const (), Throws::Caught), // sub_1412e6790
    Replacement::draft(0x1412e6ac0, "unknown_1412e6ac0", fn_1412e6ac0 as *const ()), // sub_1412e6ac0
    Replacement::draft(0x1412e6b70, "unknown_1412e6b70", fn_1412e6b70 as *const ()), // sub_1412e6b70
    Replacement::draft(0x1412e6ce0, "unknown_1412e6ce0", fn_1412e6ce0 as *const ()), // sub_1412e6ce0
    Replacement::draft_throws(0x1412e6d60, "rend_1412e6d60", fn_1412e6d60 as *const (), Throws::Caught), // sub_1412e6d60
    Replacement::draft(0x1412e6f10, "rend_1412e6f10", tpl_6429::<0xc, 0x8, 0x1412e7250> as *const ()), // sub_1412e6f10
    Replacement::draft(0x1412e6f90, "rend_1412e6f90", tpl_23818::<0x8, 0x14224dc30, 0x142442de0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1412e6f90
    Replacement::draft(0x1412e7070, "rend_1412e7070", fn_1412e7070 as *const ()), // sub_1412e7070
    Replacement::draft(0x1412e7250, "rend_1412e7250", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1412e6f90> as *const ()), // sub_1412e7250
    Replacement::draft(0x1412e73c0, "rend_1412e73c0", fn_1412e73c0 as *const ()), // sub_1412e73c0
    Replacement::draft(0x1412e7550, "rend_1412e7550", fn_1412e7550 as *const ()), // sub_1412e7550
    Replacement::draft(0x1412e76f0, "rend_1412e76f0", fn_1412e76f0 as *const ()), // sub_1412e76f0
    Replacement::draft(0x1412e79c0, "rend_1412e79c0", fn_1412e79c0 as *const ()), // sub_1412e79c0
    Replacement::draft(0x1412e7b40, "rend_1412e7b40", fn_1412e7b40 as *const ()), // sub_1412e7b40
    Replacement::draft(0x1412e7bc0, "rend_1412e7bc0", fn_1412e7bc0 as *const ()), // sub_1412e7bc0
    Replacement::draft(0x1412e7d70, "rend_1412e7d70", tpl_22001::<0x143867dd0, 0x143867de8, 0x143867df8, 0x143867de0, 0x1412e7ef0, 0x1412e81d0> as *const ()), // sub_1412e7d70
    Replacement::draft(0x1412e7ef0, "rend_1412e7ef0", tpl_80366::<0x143867dd0, 0x143867de8, 0x1434286b0, 0x143428770, 0x143867dd8, 0x143867de0, 0x143867df8> as *const ()), // sub_1412e7ef0
    Replacement::draft(0x1412e81d0, "rend_1412e81d0", tpl_80367::<0x143867dd0, 0x143867dd8, 0x143867de8, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x143867de0, 0x143867df8, 0x1425371a0, 0x142533200> as *const ()), // sub_1412e81d0
    Replacement::draft(0x1412e8510, "rend_1412e8510", fn_1412e8510 as *const ()), // sub_1412e8510
    Replacement::draft_throws(0x1412e85c0, "rend_1412e85c0", fn_1412e85c0 as *const (), Throws::Caught), // sub_1412e85c0
    Replacement::draft(0x1412e8690, "rend_1412e8690", tpl_23827::<0x14340ed48, 0x8, 0x1433f0538, 0x142498f00> as *const ()), // sub_1412e8690
    Replacement::draft(0x1412e86d0, "rend_1412e86d0", fn_1412e86d0 as *const ()), // sub_1412e86d0
    Replacement::draft(0x1412e8870, "rend_1412e8870", fn_1412e8870 as *const ()), // sub_1412e8870
    Replacement::draft_throws(0x1412e88c0, "rend_1412e88c0", fn_1412e88c0 as *const (), Throws::Caught), // sub_1412e88c0
    Replacement::draft(0x1412e9310, "rend_1412e9310", fn_1412e9310 as *const ()), // sub_1412e9310
    Replacement::draft(0x1412e96c0, "rend_1412e96c0", fn_1412e96c0 as *const ()), // sub_1412e96c0
    Replacement::draft(0x1412e9cd0, "rend_1412e9cd0", fn_1412e9cd0 as *const ()), // sub_1412e9cd0
    Replacement::draft(0x1412e9e40, "rend_1412e9e40", fn_1412e9e40 as *const ()), // sub_1412e9e40
    Replacement::draft(0x1412ea050, "rend_1412ea050", fn_1412ea050 as *const ()), // sub_1412ea050
    Replacement::draft(0x1412ea140, "rend_1412ea140", fn_1412ea140 as *const ()), // sub_1412ea140
    Replacement::draft_throws(0x1412ea310, "rend_1412ea310", fn_1412ea310 as *const (), Throws::Discard), // sub_1412ea310
    Replacement::draft_throws(0x1412ea3a0, "rend_1412ea3a0", fn_1412ea3a0 as *const (), Throws::Caught), // sub_1412ea3a0
    Replacement::draft(0x1412ea510, "rend_1412ea510", fn_1412ea510 as *const ()), // sub_1412ea510
    Replacement::draft(0x1412ea580, "rend_1412ea580", tpl_6154::<0x8, 0xc, 0x1, 0x14062ee30> as *const ()), // sub_1412ea580
    Replacement::draft(0x1412ea5f0, "rend_1412ea5f0", fn_1412ea5f0 as *const ()), // sub_1412ea5f0
    Replacement::draft(0x1412ea7e0, "rend_1412ea7e0", fn_1412ea7e0 as *const ()), // rend::SSAA::rend::SSAA::vfunc_0
    Replacement::draft(0x1412ea860, "rend_1412ea860", fn_1412ea860 as *const ()), // sub_1412ea860
    Replacement::draft_throws(0x1412ea880, "rend_1412ea880", fn_1412ea880 as *const (), Throws::Discard), // sub_1412ea880
    Replacement::draft(0x1412ea8c0, "rend_1412ea8c0", fn_1412ea8c0 as *const ()), // sub_1412ea8c0
    Replacement::draft(0x1412ead60, "rend_1412ead60", fn_1412ead60 as *const ()), // sub_1412ead60
    Replacement::draft(0x1412eae40, "rend_1412eae40", fn_1412eae40 as *const ()), // sub_1412eae40
    Replacement::draft(0x1412eaef0, "rend_1412eaef0", fn_1412eaef0 as *const ()), // sub_1412eaef0
    Replacement::draft_throws(0x1412eaf80, "rend_1412eaf80", fn_1412eaf80 as *const (), Throws::Discard), // sub_1412eaf80
    Replacement::draft(0x1412eb0c0, "rend_1412eb0c0", tpl_23850::<0x142326f60, 0x142442de0> as *const ()), // sub_1412eb0c0
    Replacement::draft(0x1412eb110, "rend_1412eb110", tpl_23850::<0x1412eb160, 0x142442de0> as *const ()), // sub_1412eb110
    Replacement::draft(0x1412eb160, "rend_1412eb160", fn_1412eb160 as *const ()), // sub_1412eb160
    Replacement::draft(0x1412eb1f0, "rend_1412eb1f0", fn_1412eb1f0 as *const ()), // sub_1412eb1f0
    Replacement::draft(0x1412eb2e0, "rend_1412eb2e0", tpl_790::<0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBase::rend::SparseUploadBase::vfunc_0
    Replacement::draft(0x1412eb320, "rend_1412eb320", fn_1412eb320 as *const ()), // sub_1412eb320
    Replacement::draft(0x1412eb3b0, "rend_1412eb3b0", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1433f11b8, 0x1412eb320> as *const ()), // sub_1412eb3b0
    Replacement::draft(0x1412eb400, "rend_1412eb400", fn_1412eb400 as *const ()), // sub_1412eb400
    Replacement::draft(0x1412eb4e0, "rend_1412eb4e0", fn_1412eb4e0 as *const ()), // sub_1412eb4e0
    Replacement::draft(0x1412eb520, "rend_1412eb520", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1433f11b8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<unsigned int>::rend::SparseUploadBuffer<unsigned int>::vfunc_0
    Replacement::draft(0x1412eb590, "rend_1412eb590", fn_1412eb590 as *const ()), // sub_1412eb590
    Replacement::draft(0x1412eb9c0, "rend_1412eb9c0", tpl_23860::<0x90, 0xa0> as *const ()), // rend::SparseUploadBuffer<unsigned int>::rend::SparseUploadBuffer<unsigned int>::vfunc_3
    Replacement::draft(0x1412ebb00, "rend_1412ebb00", tpl_23861::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x1412ebe80, 0x1412ebe10, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<unsigned int>::rend::SparseUploadBuffer<unsigned int>::vfunc_5
    Replacement::draft(0x1412ebb80, "rend_1412ebb80", tpl_23862::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x1412ebe80, 0x1412ebe10, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<unsigned int>::rend::SparseUploadBuffer<unsigned int>::vfunc_6
    Replacement::draft_throws(0x1412ebc10, "rend_1412ebc10", fn_1412ebc10 as *const (), Throws::Discard), // rend::SparseUploadInternal<unsigned int>::rend::SparseUploadInternal<unsigned int>::vfunc_1
    Replacement::draft_throws(0x1412ebc50, "rend_1412ebc50", fn_1412ebc50 as *const (), Throws::Discard), // rend::SparseUploadInternal<unsigned int>::rend::SparseUploadInternal<unsigned int>::vfunc_2
    Replacement::draft(0x1412ebc80, "rend_1412ebc80", tpl_23865::<0x1433f11b8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<unsigned int>::rend::SparseUploadInternal<unsigned int>::vfunc_0
    Replacement::draft(0x1412ebe80, "rend_1412ebe80", fn_1412ebe80 as *const ()), // sub_1412ebe80
    Replacement::draft(0x1412ebf30, "rend_1412ebf30", fn_1412ebf30 as *const ()), // sub_1412ebf30
    Replacement::draft(0x1412ec030, "rend_1412ec030", fn_1412ec030 as *const ()), // sub_1412ec030
    Replacement::draft_throws(0x1412ec110, "rend_1412ec110", fn_1412ec110 as *const (), Throws::Discard), // sub_1412ec110
    Replacement::draft(0x1412ec400, "rend_1412ec400", fn_1412ec400 as *const ()), // sub_1412ec400
    Replacement::draft(0x1412ec640, "rend_1412ec640", fn_1412ec640 as *const ()), // sub_1412ec640
    Replacement::draft(0x1412ec820, "rend_1412ec820", fn_1412ec820 as *const ()), // sub_1412ec820
    Replacement::draft(0x1412ec900, "rend_1412ec900", tpl_60956::<0x19, 0x10, 0x1412ec900, 0x28, 0x1423875a0, 0x1401e66f0> as *const ()), // sub_1412ec900
    Replacement::draft(0x1412ec980, "rend_1412ec980", fn_1412ec980 as *const ()), // sub_1412ec980
    Replacement::draft(0x1412eca10, "rend_1412eca10", tpl_17984::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1410f6460> as *const ()), // sub_1412eca10
    Replacement::draft(0x1412ecb90, "rend_1412ecb90", fn_1412ecb90 as *const ()), // sub_1412ecb90
    Replacement::draft(0x1412ece40, "rend_1412ece40", fn_1412ece40 as *const ()), // sub_1412ece40
    Replacement::draft(0x1412ecfd0, "rend_1412ecfd0", fn_1412ecfd0 as *const ()), // sub_1412ecfd0
    Replacement::draft(0x1412ed0f0, "rend_1412ed0f0", fn_1412ed0f0 as *const ()), // sub_1412ed0f0
    Replacement::draft(0x1412ed110, "rend_1412ed110", fn_1412ed110 as *const ()), // sub_1412ed110
    Replacement::draft(0x1412ed1e0, "rend_1412ed1e0", fn_1412ed1e0 as *const ()), // sub_1412ed1e0
    Replacement::draft(0x1412ed420, "rend_1412ed420", fn_1412ed420 as *const ()), // sub_1412ed420
    Replacement::draft(0x1412ed6e0, "rend_1412ed6e0", fn_1412ed6e0 as *const ()), // sub_1412ed6e0
    Replacement::draft(0x1412ed7d0, "rend_1412ed7d0", fn_1412ed7d0 as *const ()), // sub_1412ed7d0
    Replacement::draft(0x1412ed800, "rend_1412ed800", fn_1412ed800 as *const ()), // sub_1412ed800
    Replacement::draft(0x1412ed840, "rend_1412ed840", fn_1412ed840 as *const ()), // sub_1412ed840
    Replacement::draft(0x1412ed9a0, "rend_1412ed9a0", fn_1412ed9a0 as *const ()), // sub_1412ed9a0
    Replacement::draft(0x1412eda20, "rend_1412eda20", crate::simd_ops::fn_1412eda20 as *const ()), // sub_1412eda20
    Replacement::draft(0x1412edae0, "rend_1412edae0", fn_1412edae0 as *const ()), // sub_1412edae0
    Replacement::draft(0x1412edba0, "rend_1412edba0", fn_1412edba0 as *const ()), // sub_1412edba0
    Replacement::draft(0x1412edc60, "rend_1412edc60", tpl_23888::<0x1412eef50, 0x1412edc60, 0x1412eeb30, 0x1412f0d80, 0x1412f0cc0> as *const ()), // sub_1412edc60
    Replacement::draft(0x1412edff0, "rend_1412edff0", tpl_23888::<0x1412efe20, 0x1412edff0, 0x1412efa00, 0x1412f1220, 0x1412f0cc0> as *const ()), // sub_1412edff0
    Replacement::draft(0x1412ee380, "rend_1412ee380", fn_1412ee380 as *const ()), // sub_1412ee380
    Replacement::draft(0x1412ee580, "rend_1412ee580", fn_1412ee580 as *const ()), // sub_1412ee580
    Replacement::draft(0x1412eef50, "rend_1412eef50", fn_1412eef50 as *const ()), // sub_1412eef50
    Replacement::draft(0x1412efa00, "rend_1412efa00", fn_1412efa00 as *const ()), // sub_1412efa00
    Replacement::draft(0x1412efe20, "rend_1412efe20", fn_1412efe20 as *const ()), // sub_1412efe20
    Replacement::draft(0x1412f09b0, "rend_1412f09b0", fn_1412f09b0 as *const ()), // sub_1412f09b0
    Replacement::draft(0x1412f0cc0, "rend_1412f0cc0", fn_1412f0cc0 as *const ()), // sub_1412f0cc0
    Replacement::draft(0x1412f0d80, "rend_1412f0d80", fn_1412f0d80 as *const ()), // sub_1412f0d80
    Replacement::draft(0x1412f10d0, "rend_1412f10d0", fn_1412f10d0 as *const ()), // sub_1412f10d0
    Replacement::draft(0x1412f1220, "rend_1412f1220", fn_1412f1220 as *const ()), // sub_1412f1220
    Replacement::draft(0x1412f1590, "rend_1412f1590", fn_1412f1590 as *const ()), // sub_1412f1590
    Replacement::draft(0x1412f1870, "rend_1412f1870", tpl_4305::<0x8, 0x1412f10d0> as *const ()), // sub_1412f1870
    Replacement::draft(0x1412f18e0, "rend_1412f18e0", fn_1412f18e0 as *const ()), // sub_1412f18e0
    Replacement::draft(0x1412f1980, "rend_1412f1980", fn_1412f1980 as *const ()), // sub_1412f1980
    Replacement::draft(0x1412f1b20, "rend_1412f1b20", fn_1412f1b20 as *const ()), // sub_1412f1b20
    Replacement::draft(0x1412f1b40, "rend_1412f1b40", fn_1412f1b40 as *const ()), // sub_1412f1b40
    Replacement::draft(0x1412f1cc0, "rend_1412f1cc0", fn_1412f1cc0 as *const ()), // sub_1412f1cc0
    Replacement::draft(0x1412f1cd0, "rend_1412f1cd0", tpl_1043::<0x28> as *const ()), // rend::ShadowMap::rend::ShadowMap::vfunc_6
    Replacement::draft_throws(0x1412f1d90, "rend_1412f1d90", fn_1412f1d90 as *const (), Throws::Discard), // rend::ShadowInterfaceDepth::rend::ShadowInterfaceDepth::vfunc_0
    Replacement::draft(0x1412f1dd0, "unknown_1412f1dd0", fn_1412f1dd0 as *const ()), // sub_1412f1dd0
    Replacement::draft_throws(0x1412f1e70, "shadowinterfacedepth_1412f1e70", fn_1412f1e70 as *const (), Throws::Caught), // ShadowInterfaceDepth::ShadowInterfaceDepth::m_pShadowAtlas
    Replacement::draft(0x1412f1f50, "rend_1412f1f50", fn_1412f1f50 as *const ()), // sub_1412f1f50
    Replacement::draft_throws(0x1412f2000, "rend_1412f2000", fn_1412f2000 as *const (), Throws::Discard), // rend::ShadowInterfaceColor::rend::ShadowInterfaceColor::vfunc_0
    Replacement::draft(0x1412f2120, "rend_1412f2120", fn_1412f2120 as *const ()), // sub_1412f2120
    Replacement::draft_throws(0x1412f21b0, "rend_1412f21b0", fn_1412f21b0 as *const (), Throws::Caught), // sub_1412f21b0
    Replacement::draft_throws(0x1412f2260, "rend_1412f2260", fn_1412f2260 as *const (), Throws::Caught), // sub_1412f2260
    Replacement::draft_throws(0x1412f2320, "rend_1412f2320", fn_1412f2320 as *const (), Throws::Caught), // sub_1412f2320
    Replacement::draft(0x1412f23d0, "rend_1412f23d0", fn_1412f23d0 as *const ()), // sub_1412f23d0
    Replacement::draft(0x1412f27c0, "rend_1412f27c0", fn_1412f27c0 as *const ()), // sub_1412f27c0
    Replacement::draft(0x1412f2920, "rend_1412f2920", fn_1412f2920 as *const ()), // sub_1412f2920
    Replacement::draft(0x1412f2cd0, "rend_1412f2cd0", fn_1412f2cd0 as *const ()), // sub_1412f2cd0
    Replacement::draft(0x1412f2df0, "rend_1412f2df0", fn_1412f2df0 as *const ()), // sub_1412f2df0
    Replacement::draft(0x1412f2f10, "rend_1412f2f10", fn_1412f2f10 as *const ()), // sub_1412f2f10
    Replacement::draft(0x1412f3000, "rend_1412f3000", fn_1412f3000 as *const ()), // sub_1412f3000
    Replacement::draft(0x1412f3110, "rend_1412f3110", fn_1412f3110 as *const ()), // sub_1412f3110
    Replacement::draft(0x1412f3220, "rend_1412f3220", fn_1412f3220 as *const ()), // rend::ShaderResource::rend::ShaderResource::vfunc_0
    Replacement::draft(0x1412f3280, "rend_1412f3280", tpl_16695::<0x190, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1412f3320> as *const ()), // sub_1412f3280
    Replacement::draft(0x1412f3320, "rend_1412f3320", fn_1412f3320 as *const ()), // sub_1412f3320
    Replacement::draft_throws(0x1412f3400, "rend_1412f3400", fn_1412f3400 as *const (), Throws::Caught), // sub_1412f3400
    Replacement::draft(0x1412f34c0, "rend_1412f34c0", fn_1412f34c0 as *const ()), // sub_1412f34c0
    Replacement::draft(0x1412f3540, "rend_1412f3540", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433f2178, 0x8> as *const ()), // sub_1412f3540
    Replacement::draft(0x1412f35c0, "rend_1412f35c0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433f2178, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412f35c0
    Replacement::draft_throws(0x1412f3660, "rend_1412f3660", fn_1412f3660 as *const (), Throws::Caught), // sub_1412f3660
    Replacement::draft_throws(0x1412f36f0, "rend_1412f36f0", tpl_649::<0x143881a80, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412f36f0
    Replacement::draft(0x1412f3770, "rend_1412f3770", tpl_23920::<0xc8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x1433f1e68, 0xb8> as *const ()), // sub_1412f3770
    Replacement::draft(0x1412f3820, "rend_1412f3820", tpl_23921::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433f1e68, 0x14183acc2, 0x142428d70, 0xb8> as *const ()), // sub_1412f3820
    Replacement::draft(0x1412f38f0, "rend_1412f38f0", tpl_6868::<0x143881a90, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1412f38f0
    Replacement::draft_throws(0x1412f3980, "rend_1412f3980", tpl_23922::<0x143881a78, 0x143881a90, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412f3980
    Replacement::draft_throws(0x1412f3a00, "rend_1412f3a00", tpl_23922::<0x143881a78, 0x143881a90, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1412f3a00
    Replacement::draft_throws(0x1412f3a80, "rend_1412f3a80", fn_1412f3a80 as *const (), Throws::Caught), // sub_1412f3a80
    Replacement::draft(0x1412f3b10, "rend_1412f3b10", tpl_23923::<0x143881a78, 0x143881a90, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1412f3b10
    Replacement::draft_throws(0x1412f3b90, "rend_1412f3b90", tpl_23922::<0x143881a78, 0x143881a90, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1412f3b90
    Replacement::draft_throws(0x1412f3c10, "rend_1412f3c10", tpl_23924::<0x143881a90, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412f3c10
    Replacement::draft(0x1412f3cc0, "rend_1412f3cc0", fn_1412f3cc0 as *const ()), // sub_1412f3cc0
    Replacement::draft(0x1412f3d90, "rend_1412f3d90", tpl_23926::<0x8, 0x1412f43b0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1412f3d90
    Replacement::draft(0x1412f3e50, "rend_1412f3e50", fn_1412f3e50 as *const ()), // sub_1412f3e50
    Replacement::draft_throws(0x1412f3f20, "rend_1412f3f20", fn_1412f3f20 as *const (), Throws::Caught), // sub_1412f3f20
    Replacement::draft(0x1412f40a0, "rend_1412f40a0", tpl_23929::<0x142407c80, 0x143c4aa00, 0x1412f34c0, 0x142442de0, 0x1426455a8, 0x142645598, 0x14224bd50, 0x142645580> as *const ()), // sub_1412f40a0
    Replacement::draft(0x1412f4210, "rend_1412f4210", tpl_23929::<0x142407c80, 0x143c4aa00, 0x1423983f0, 0x142442de0, 0x1426455a8, 0x142645598, 0x1412f4380, 0x142645580> as *const ()), // sub_1412f4210
    Replacement::draft(0x1412f4380, "rend_1412f4380", tpl_23930::<0x1423983f0, 0x142442de0> as *const ()), // sub_1412f4380
    Replacement::draft(0x1412f43b0, "rend_1412f43b0", fn_1412f43b0 as *const ()), // sub_1412f43b0
    Replacement::draft(0x1412f4410, "rend_1412f4410", fn_1412f4410 as *const ()), // sub_1412f4410
    Replacement::draft(0x1412f4530, "unknown_1412f4530", tpl_23933::<0x20, 0x8, 0xc, 0x10> as *const ()), // sub_1412f4530
    Replacement::draft(0x1412f45c0, "unknown_1412f45c0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433f26d0, 0x8> as *const ()), // sub_1412f45c0
    Replacement::draft(0x1412f4640, "unknown_1412f4640", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433f26d0, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1412f4640
    Replacement::draft_throws(0x1412f46e0, "unknown_1412f46e0", fn_1412f46e0 as *const (), Throws::Caught), // sub_1412f46e0
    Replacement::draft_throws(0x1412f4770, "unknown_1412f4770", tpl_649::<0x143881aa0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1412f4770
    Replacement::draft(0x1412f47f0, "unknown_1412f47f0", fn_1412f47f0 as *const ()), // sub_1412f47f0
    Replacement::draft(0x1412f4880, "unknown_1412f4880", fn_1412f4880 as *const ()), // sub_1412f4880
    Replacement::draft(0x1412f4920, "unknown_1412f4920", tpl_6868::<0x143881aa8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1412f4920
    Replacement::draft_throws(0x1412f49b0, "unknown_1412f49b0", tpl_23922::<0x143881a9c, 0x143881aa8, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1412f49b0
    Replacement::draft_throws(0x1412f4a30, "unknown_1412f4a30", tpl_23922::<0x143881a9c, 0x143881aa8, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1412f4a30
    Replacement::draft_throws(0x1412f4ab0, "unknown_1412f4ab0", fn_1412f4ab0 as *const (), Throws::Caught), // sub_1412f4ab0
    Replacement::draft(0x1412f4b40, "unknown_1412f4b40", tpl_23923::<0x143881a9c, 0x143881aa8, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1412f4b40
    Replacement::draft_throws(0x1412f4bc0, "unknown_1412f4bc0", tpl_23922::<0x143881a9c, 0x143881aa8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1412f4bc0
    Replacement::draft_throws(0x1412f4c40, "unknown_1412f4c40", tpl_23924::<0x143881aa8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1412f4c40
    Replacement::draft(0x1412f4cf0, "unknown_1412f4cf0", fn_1412f4cf0 as *const ()), // sub_1412f4cf0
    Replacement::draft_throws(0x1412f4df0, "unknown_1412f4df0", fn_1412f4df0 as *const (), Throws::Caught), // sub_1412f4df0
    Replacement::draft(0x1412f4fe0, "unknown_1412f4fe0", fn_1412f4fe0 as *const ()), // sub_1412f4fe0
    Replacement::draft_throws(0x1412f5160, "unknown_1412f5160", fn_1412f5160 as *const (), Throws::Discard), // sub_1412f5160
    Replacement::draft_throws(0x1412f5360, "unknown_1412f5360", fn_1412f5360 as *const (), Throws::Discard), // sub_1412f5360
    Replacement::draft(0x1412f5570, "unknown_1412f5570", fn_1412f5570 as *const ()), // sub_1412f5570
    Replacement::draft_throws(0x1412f56d0, "std_1412f56d0", fn_1412f56d0 as *const (), Throws::Discard), // std::_Ref_count_obj_alloc3<rend::DistanceFieldGPUData,r::Allocator<rend::DistanceFieldGPUData,0>>::std::_Ref_count_obj_a
    Replacement::draft(0x1412f5720, "std_1412f5720", fn_1412f5720 as *const ()), // std::_Ref_count_obj_alloc3<rend::DistanceFieldData,r::Allocator<rend::DistanceFieldData,0>>::std::_Ref_count_obj_alloc3<
    Replacement::draft(0x1412f5770, "std_1412f5770", tpl_23943::<0x1433f2ab8, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<rend::DistanceFieldGPUData,r::Allocator<rend::DistanceFieldGPUData,0>>::std::_Ref_count_obj_a
    Replacement::draft(0x1412f57a0, "std_1412f57a0", fn_1412f57a0 as *const ()), // std::_Ref_count_obj_alloc3<rend::DistanceFieldData,r::Allocator<rend::DistanceFieldData,0>>::std::_Ref_count_obj_alloc3<
    Replacement::draft(0x1412f57d0, "unknown_1412f57d0", tpl_23945::<0x143881db0, 0x14243b0b0, 0x338, 0x48, 0x4, 0x8, 0xc, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1412f57d0
    Replacement::draft(0x1412f5870, "unknown_1412f5870", fn_1412f5870 as *const ()), // sub_1412f5870
    Replacement::draft(0x1412f5940, "unknown_1412f5940", fn_1412f5940 as *const ()), // sub_1412f5940
    Replacement::draft(0x1412f5a70, "rend_1412f5a70", fn_1412f5a70 as *const ()), // rend::SDFVolumeDataStreamer::rend::SDFVolumeDataStreamer::vfunc_0
    Replacement::draft(0x1412f5ae0, "unknown_1412f5ae0", tpl_21690::<0x1401e5380, 0x8, 0x1412f6080, 0x1401e66f0> as *const ()), // sub_1412f5ae0
    Replacement::draft(0x1412f5b10, "unknown_1412f5b10", tpl_170::<0x1412f6170> as *const ()), // sub_1412f5b10
    Replacement::draft(0x1412f5b20, "unknown_1412f5b20", tpl_60671::<0x10, 0x8, 0xc> as *const ()), // sub_1412f5b20
    Replacement::draft_throws(0x1412f5b70, "std_1412f5b70", fn_1412f5b70 as *const (), Throws::Discard), // sub_1412f5b70
    Replacement::draft(0x1412f5c60, "std_1412f5c60", tpl_21430::<0x1412f6080, 0x1401e66f0> as *const ()), // sub_1412f5c60
    Replacement::draft(0x1412f5c80, "std_1412f5c80", fn_1412f5c80 as *const ()), // sub_1412f5c80
    Replacement::draft(0x1412f5d30, "std_1412f5d30", tpl_170::<0x1412f6240> as *const ()), // sub_1412f5d30
    Replacement::draft_throws(0x1412f5d40, "std_1412f5d40", fn_1412f5d40 as *const (), Throws::Discard), // sub_1412f5d40
    Replacement::draft(0x1412f6080, "unknown_1412f6080", fn_1412f6080 as *const ()), // sub_1412f6080
    Replacement::draft(0x1412f6100, "unknown_1412f6100", fn_1412f6100 as *const ()), // sub_1412f6100
    Replacement::draft(0x1412f6170, "unknown_1412f6170", tpl_620::<0x8, 0x1412f4530, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1412f6170
    Replacement::draft(0x1412f62f0, "unknown_1412f62f0", fn_1412f62f0 as *const ()), // sub_1412f62f0
    Replacement::draft(0x1412f6440, "unknown_1412f6440", fn_1412f6440 as *const ()), // sub_1412f6440
    Replacement::draft(0x1412f64a0, "unknown_1412f64a0", fn_1412f64a0 as *const ()), // sub_1412f64a0
    Replacement::draft_throws(0x1412f67b0, "rend_1412f67b0", fn_1412f67b0 as *const (), Throws::Caught), // sub_1412f67b0
    Replacement::draft(0x1412f6870, "rend_1412f6870", tpl_22063::<0x142257320, 0x141834d30> as *const ()), // rend::RenderTargetManagerTrimHandler::rend::RenderTargetManagerTrimHandler::vfunc_0
    Replacement::draft(0x1412f68b0, "rend_1412f68b0", tpl_22065::<0x143bbcfec, 0x143bbcfe8, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143bbcfe0, 0x14183acc8, 0x1401de830> as *const ()), // sub_1412f68b0
    Replacement::draft_throws(0x1412f6a00, "rend_1412f6a00", fn_1412f6a00 as *const (), Throws::Discard), // rend::RenderTargetManager::rend::RenderTargetManager::vfunc_0
    Replacement::draft_throws(0x1412f6a40, "rend_1412f6a40", fn_1412f6a40 as *const (), Throws::Caught), // sub_1412f6a40
    Replacement::draft_throws(0x1412f6ae0, "rend_1412f6ae0", fn_1412f6ae0 as *const (), Throws::Caught), // sub_1412f6ae0
    Replacement::draft(0x1412f6b70, "rend_1412f6b70", fn_1412f6b70 as *const ()), // sub_1412f6b70
    Replacement::draft(0x1412f6cc0, "rend_1412f6cc0", fn_1412f6cc0 as *const ()), // sub_1412f6cc0
    Replacement::draft(0x1412f6d90, "rend_1412f6d90", fn_1412f6d90 as *const ()), // sub_1412f6d90
    Replacement::draft(0x1412f6f00, "rend_1412f6f00", fn_1412f6f00 as *const ()), // sub_1412f6f00
    Replacement::draft(0x1412f6fd0, "rend_1412f6fd0", fn_1412f6fd0 as *const ()), // sub_1412f6fd0
    Replacement::draft(0x1412f7170, "rend_1412f7170", fn_1412f7170 as *const ()), // sub_1412f7170
    Replacement::draft(0x1412f7310, "rend_1412f7310", tpl_23964::<0x1, 0x3, 0x4, 0x8, 0x14240aa00, 0x2c> as *const ()), // sub_1412f7310
    Replacement::draft(0x1412f7370, "rend_1412f7370", fn_1412f7370 as *const ()), // sub_1412f7370
    Replacement::draft(0x1412f73b0, "rend_1412f73b0", tpl_23966::<0x8, 0x3c, 0x14240ab10> as *const ()), // sub_1412f73b0
    Replacement::draft(0x1412f73f0, "rend_1412f73f0", tpl_23964::<0x1, 0x3, 0x4, 0x8, 0x14240a730, 0x2c> as *const ()), // sub_1412f73f0
    Replacement::draft(0x1412f7450, "rend_1412f7450", fn_1412f7450 as *const ()), // sub_1412f7450
    Replacement::draft(0x1412f7650, "rend_1412f7650", fn_1412f7650 as *const ()), // sub_1412f7650
    Replacement::draft(0x1412f7950, "rend_1412f7950", fn_1412f7950 as *const ()), // sub_1412f7950
    Replacement::draft(0x1412f7ac0, "rend_1412f7ac0", tpl_19784::<0x142407c80, 0x143c4aa00, 0x142258760, 0x1426455a8, 0x142645598, 0x142258610, 0x142645580> as *const ()), // sub_1412f7ac0
    Replacement::draft(0x1412f7c10, "rend_1412f7c10", tpl_23970::<0x1401de830, 0x1433f3340, 0x1412eb320> as *const ()), // sub_1412f7c10
    Replacement::draft(0x1412f7c50, "rend_1412f7c50", fn_1412f7c50 as *const ()), // sub_1412f7c50
    Replacement::draft(0x1412f7d50, "rend_1412f7d50", fn_1412f7d50 as *const ()), // sub_1412f7d50
    Replacement::draft(0x1412f7dc0, "rend_1412f7dc0", fn_1412f7dc0 as *const ()), // sub_1412f7dc0
    Replacement::draft(0x1412f7ec0, "rend_1412f7ec0", fn_1412f7ec0 as *const ()), // sub_1412f7ec0
    Replacement::draft(0x1412f8010, "rend_1412f8010", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1433f3308, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<r::BoundSphere>::rend::SparseUploadBuffer<r::BoundSphere>::vfunc_0
    Replacement::draft(0x1412f8080, "rend_1412f8080", fn_1412f8080 as *const ()), // rend::GPUOnlySparseUploadBuffer<rend::GPUTransform>::rend::GPUOnlySparseUploadBuffer<rend::GPUTransform>::vfunc_0
    Replacement::draft(0x1412f80c0, "rend_1412f80c0", tpl_23977::<0x1401de830, 0x1433f3340, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedGPUOnlySparseUploadBuffer<rend::GPUTransform>::rend::TrackedGPUOnlySparseUploadBuffer<rend::GPUTransform>::
    Replacement::draft_throws(0x1412f8110, "rend_1412f8110", fn_1412f8110 as *const (), Throws::Discard), // rend::TrackedGPUOnlySparseUploadBuffer<rend::GPUTransform>::rend::TrackedGPUOnlySparseUploadBuffer<rend::GPUTransform>::
    Replacement::draft(0x1412f81b0, "rend_1412f81b0", fn_1412f81b0 as *const ()), // sub_1412f81b0
    Replacement::draft(0x1412f8250, "rend_1412f8250", fn_1412f8250 as *const ()), // sub_1412f8250
    Replacement::draft_throws(0x1412f82e0, "rend_1412f82e0", fn_1412f82e0 as *const (), Throws::Discard), // sub_1412f82e0
    Replacement::draft_throws(0x1412f8420, "rend_1412f8420", fn_1412f8420 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GPUTransform>::rend::SparseUploadInternal<rend::GPUTransform>::vfunc_1
    Replacement::draft_throws(0x1412f8460, "rend_1412f8460", fn_1412f8460 as *const (), Throws::Discard), // sub_1412f8460
    Replacement::draft_throws(0x1412f8520, "rend_1412f8520", fn_1412f8520 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GPUTransform>::rend::SparseUploadInternal<rend::GPUTransform>::vfunc_2
    Replacement::draft_throws(0x1412f8560, "rend_1412f8560", fn_1412f8560 as *const (), Throws::Discard), // rend::GPUOnlySparseUploadBuffer<rend::GPUTransform>::rend::GPUOnlySparseUploadBuffer<rend::GPUTransform>::vfunc_1
    Replacement::draft(0x1412f8600, "rend_1412f8600", fn_1412f8600 as *const ()), // sub_1412f8600
    Replacement::draft(0x1412f89d0, "rend_1412f89d0", fn_1412f89d0 as *const ()), // rend::SparseUploadBuffer<r::BoundSphere>::rend::SparseUploadBuffer<r::BoundSphere>::vfunc_4
    Replacement::draft(0x1412f8af0, "rend_1412f8af0", tpl_23984::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x1412f8f30, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<r::BoundSphere>::rend::SparseUploadBuffer<r::BoundSphere>::vfunc_5
    Replacement::draft(0x1412f8b80, "rend_1412f8b80", tpl_23985::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x1412f8f30, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<r::BoundSphere>::rend::SparseUploadBuffer<r::BoundSphere>::vfunc_6
    Replacement::draft_throws(0x1412f8c20, "rend_1412f8c20", tpl_23986::<0x20, 0x29, 0x10, 0x8, 0x1422605c0> as *const (), Throws::Discard), // rend::SparseUploadInternal<r::BoundSphere>::rend::SparseUploadInternal<r::BoundSphere>::vfunc_1
    Replacement::draft_throws(0x1412f8c60, "rend_1412f8c60", tpl_23987::<0x29, 0x20, 0x10, 0x8, 0x1422605c0> as *const (), Throws::Discard), // rend::SparseUploadInternal<r::BoundSphere>::rend::SparseUploadInternal<r::BoundSphere>::vfunc_2
    Replacement::draft(0x1412f8d90, "rend_1412f8d90", fn_1412f8d90 as *const ()), // sub_1412f8d90
    Replacement::draft(0x1412f8eb0, "rend_1412f8eb0", tpl_23865::<0x1433f3340, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::GPUTransform>::rend::SparseUploadInternal<rend::GPUTransform>::vfunc_0
    Replacement::draft(0x1412f8ef0, "rend_1412f8ef0", tpl_23865::<0x1433f3308, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<r::BoundSphere>::rend::SparseUploadInternal<r::BoundSphere>::vfunc_0
    Replacement::draft(0x1412f8f30, "rend_1412f8f30", fn_1412f8f30 as *const ()), // sub_1412f8f30
    Replacement::draft_throws(0x1412f9440, "rend_1412f9440", fn_1412f9440 as *const (), Throws::Discard), // sub_1412f9440
    Replacement::draft(0x1412f9540, "rend_1412f9540", tpl_23991::<0xffffffffffffffff, 0x8, 0x4, 0x2> as *const ()), // sub_1412f9540
    Replacement::draft(0x1412f9600, "rend_1412f9600", tpl_23992 as *const ()), // sub_1412f9600
    Replacement::draft(0x1412f9770, "rend_1412f9770", tpl_23993::<0x38, 0x20> as *const ()), // sub_1412f9770
    Replacement::draft_throws(0x1412f9780, "rend_1412f9780", fn_1412f9780 as *const (), Throws::Caught), // sub_1412f9780
    Replacement::draft_throws(0x1412f9a30, "rend_1412f9a30", fn_1412f9a30 as *const (), Throws::Caught), // sub_1412f9a30
    Replacement::draft(0x1412f9ba0, "rend_1412f9ba0", fn_1412f9ba0 as *const ()), // sub_1412f9ba0
    Replacement::draft_throws(0x1412f9c30, "rend_1412f9c30", fn_1412f9c30 as *const (), Throws::Discard), // rend::RendererInterface::rend::RendererInterface::vfunc_0
    Replacement::draft(0x1412f9c70, "unknown_1412f9c70", tpl_170::<0x141303f00> as *const ()), // sub_1412f9c70
    Replacement::draft(0x1412f9c80, "unknown_1412f9c80", fn_1412f9c80 as *const ()), // sub_1412f9c80
    Replacement::draft(0x1412f9d10, "unknown_1412f9d10", tpl_23999 as *const ()), // sub_1412f9d10
    Replacement::draft(0x1412f9d20, "unknown_1412f9d20", tpl_24000 as *const ()), // sub_1412f9d20
    Replacement::draft(0x1412f9d30, "d3d_1412f9d30", tpl_24001::<0x142645790, 0x8, 0x1426455a8, 0xc, 0x20, 0x24, 0x10, 0x1411daad0, 0x1c, 0x142645598> as *const ()), // d3d::SafeResourcePool<d3d::RasterizerStateManager,d3d::AllocatorBase<d3d::RasterizerStateManager>>::addReleaseTask::Boun
    Replacement::draft(0x1412f9dd0, "unknown_1412f9dd0", tpl_24002::<0x142645790, 0x1426455a8, 0x1402b02a0, 0x1411daad0, 0x142645598> as *const ()), // sub_1412f9dd0
    Replacement::draft_throws(0x1412f9ed0, "unknown_1412f9ed0", tpl_23670::<0x14397ff18, 0x142257af0> as *const (), Throws::Discard), // sub_1412f9ed0
    Replacement::draft(0x1412f9ef0, "unknown_1412f9ef0", fn_1412f9ef0 as *const ()), // sub_1412f9ef0
    Replacement::draft(0x1412f9f20, "unknown_1412f9f20", fn_1412f9f20 as *const ()), // sub_1412f9f20
    Replacement::draft(0x1412f9f60, "unknown_1412f9f60", fn_1412f9f60 as *const ()), // sub_1412f9f60
    Replacement::draft(0x1412f9f70, "unknown_1412f9f70", fn_1412f9f70 as *const ()), // sub_1412f9f70
    Replacement::draft(0x1412f9fb0, "unknown_1412f9fb0", fn_1412f9fb0 as *const ()), // sub_1412f9fb0
    Replacement::draft_throws(0x1412f9ff0, "unknown_1412f9ff0", fn_1412f9ff0 as *const (), Throws::Discard), // sub_1412f9ff0
    Replacement::draft_throws(0x1412fa030, "unknown_1412fa030", fn_1412fa030 as *const (), Throws::Discard), // sub_1412fa030
    Replacement::draft_throws(0x1412fa070, "unknown_1412fa070", tpl_23777::<0x8, 0x14204f860, 0x142442de0> as *const (), Throws::Discard), // sub_1412fa070
    Replacement::draft(0x1412fa0d0, "unknown_1412fa0d0", fn_1412fa0d0 as *const ()), // sub_1412fa0d0
    Replacement::draft(0x1412fa150, "unknown_1412fa150", fn_1412fa150 as *const ()), // sub_1412fa150
    Replacement::draft_throws(0x1412fa1c0, "unknown_1412fa1c0", fn_1412fa1c0 as *const (), Throws::Discard), // sub_1412fa1c0
    Replacement::draft_throws(0x1412fa250, "unknown_1412fa250", fn_1412fa250 as *const (), Throws::Discard), // sub_1412fa250
    Replacement::draft(0x1412fa2e0, "unknown_1412fa2e0", fn_1412fa2e0 as *const ()), // sub_1412fa2e0
    Replacement::draft(0x1412fa2f0, "unknown_1412fa2f0", fn_1412fa2f0 as *const ()), // sub_1412fa2f0
    Replacement::draft(0x1412fa380, "unknown_1412fa380", fn_1412fa380 as *const ()), // sub_1412fa380
    Replacement::draft(0x1412fa3f0, "unknown_1412fa3f0", fn_1412fa3f0 as *const ()), // sub_1412fa3f0
    Replacement::draft(0x1412fa490, "unknown_1412fa490", fn_1412fa490 as *const ()), // sub_1412fa490
    Replacement::draft_throws(0x1412fa530, "unknown_1412fa530", fn_1412fa530 as *const (), Throws::Discard), // sub_1412fa530
    Replacement::draft(0x1412fa5e0, "unknown_1412fa5e0", fn_1412fa5e0 as *const ()), // sub_1412fa5e0
    Replacement::draft(0x1412fa5f0, "unknown_1412fa5f0", fn_1412fa5f0 as *const ()), // sub_1412fa5f0
    Replacement::draft(0x1412fa630, "unknown_1412fa630", fn_1412fa630 as *const ()), // sub_1412fa630
    Replacement::draft(0x1412fa6b0, "unknown_1412fa6b0", fn_1412fa6b0 as *const ()), // sub_1412fa6b0
    Replacement::draft(0x1412fa700, "unknown_1412fa700", fn_1412fa700 as *const ()), // sub_1412fa700
    Replacement::draft(0x1412fa710, "unknown_1412fa710", fn_1412fa710 as *const ()), // sub_1412fa710
    Replacement::draft(0x1412fa7a0, "unknown_1412fa7a0", fn_1412fa7a0 as *const ()), // sub_1412fa7a0
    Replacement::draft(0x1412fa7e0, "unknown_1412fa7e0", tpl_4771::<0x40, 0x1401de830, 0x142442de0> as *const ()), // sub_1412fa7e0
    Replacement::draft(0x1412fa810, "unknown_1412fa810", tpl_429::<0x143a001f0, 0x142442de0> as *const ()), // sub_1412fa810
    Replacement::draft(0x1412fa840, "unknown_1412fa840", fn_1412fa840 as *const ()), // sub_1412fa840
    Replacement::draft_throws(0x1412fa8a0, "unknown_1412fa8a0", fn_1412fa8a0 as *const (), Throws::Discard), // sub_1412fa8a0
    Replacement::draft(0x1412fa900, "unknown_1412fa900", fn_1412fa900 as *const ()), // sub_1412fa900
    Replacement::draft(0x1412fa910, "unknown_1412fa910", fn_1412fa910 as *const ()), // sub_1412fa910
    Replacement::draft(0x1412fa920, "unknown_1412fa920", tpl_429::<0x143a00238, 0x142442de0> as *const ()), // sub_1412fa920
    Replacement::draft(0x1412fa9e0, "unknown_1412fa9e0", fn_1412fa9e0 as *const ()), // sub_1412fa9e0
    Replacement::draft_throws(0x1412faa20, "unknown_1412faa20", fn_1412faa20 as *const (), Throws::Discard), // sub_1412faa20
    Replacement::draft(0x1412faa60, "unknown_1412faa60", fn_1412faa60 as *const ()), // sub_1412faa60
    Replacement::draft(0x1412faaa0, "unknown_1412faaa0", fn_1412faaa0 as *const ()), // sub_1412faaa0
    Replacement::draft(0x1412fab20, "unknown_1412fab20", fn_1412fab20 as *const ()), // sub_1412fab20
    Replacement::draft(0x1412fab90, "unknown_1412fab90", fn_1412fab90 as *const ()), // sub_1412fab90
    Replacement::draft(0x1412fabd0, "unknown_1412fabd0", fn_1412fabd0 as *const ()), // sub_1412fabd0
    Replacement::draft(0x1412fac10, "unknown_1412fac10", fn_1412fac10 as *const ()), // sub_1412fac10
    Replacement::draft(0x1412fac20, "unknown_1412fac20", fn_1412fac20 as *const ()), // sub_1412fac20
    Replacement::draft(0x1412fac30, "unknown_1412fac30", fn_1412fac30 as *const ()), // sub_1412fac30
    Replacement::draft(0x1412facf0, "unknown_1412facf0", fn_1412facf0 as *const ()), // sub_1412facf0
    Replacement::draft_throws(0x1412fad00, "unknown_1412fad00", fn_1412fad00 as *const (), Throws::Discard), // sub_1412fad00
    Replacement::draft(0x1412fad30, "unknown_1412fad30", fn_1412fad30 as *const ()), // sub_1412fad30
    Replacement::draft(0x1412fad40, "unknown_1412fad40", tpl_24028::<0x8> as *const ()), // sub_1412fad40
    Replacement::draft(0x1412fad60, "unknown_1412fad60", fn_1412fad60 as *const ()), // sub_1412fad60
    Replacement::draft(0x1412fae00, "unknown_1412fae00", fn_1412fae00 as *const ()), // sub_1412fae00
    Replacement::draft(0x1412faf90, "unknown_1412faf90", fn_1412faf90 as *const ()), // sub_1412faf90
    Replacement::draft_throws(0x1412fb010, "unknown_1412fb010", fn_1412fb010 as *const (), Throws::Caught), // sub_1412fb010
    Replacement::draft(0x1412fb130, "unknown_1412fb130", fn_1412fb130 as *const ()), // sub_1412fb130
    Replacement::draft(0x1412fb1c0, "unknown_1412fb1c0", fn_1412fb1c0 as *const ()), // sub_1412fb1c0
    Replacement::draft(0x1412fb270, "unknown_1412fb270", fn_1412fb270 as *const ()), // sub_1412fb270
    Replacement::draft(0x1412fb300, "unknown_1412fb300", fn_1412fb300 as *const ()), // sub_1412fb300
    Replacement::draft(0x1412fb380, "unknown_1412fb380", fn_1412fb380 as *const ()), // sub_1412fb380
    Replacement::draft(0x1412fb400, "unknown_1412fb400", fn_1412fb400 as *const ()), // sub_1412fb400
    Replacement::draft(0x1412fb4a0, "unknown_1412fb4a0", fn_1412fb4a0 as *const ()), // sub_1412fb4a0
    Replacement::draft(0x1412fb540, "unknown_1412fb540", fn_1412fb540 as *const ()), // sub_1412fb540
    Replacement::draft(0x1412fb670, "unknown_1412fb670", fn_1412fb670 as *const ()), // sub_1412fb670
    Replacement::draft_throws(0x1412fb7c0, "unknown_1412fb7c0", fn_1412fb7c0 as *const (), Throws::Discard), // sub_1412fb7c0
    Replacement::draft_throws(0x1412fb860, "unknown_1412fb860", fn_1412fb860 as *const (), Throws::Discard), // sub_1412fb860
    Replacement::draft(0x1412fb960, "unknown_1412fb960", tpl_23850::<0x142324020, 0x142442de0> as *const ()), // sub_1412fb960
    Replacement::draft_throws(0x1412fb9b0, "unknown_1412fb9b0", fn_1412fb9b0 as *const (), Throws::Discard), // sub_1412fb9b0
    Replacement::draft_throws(0x1412fbab0, "unknown_1412fbab0", fn_1412fbab0 as *const (), Throws::Discard), // sub_1412fbab0
    Replacement::draft_throws(0x1412fbbb0, "unknown_1412fbbb0", fn_1412fbbb0 as *const (), Throws::Discard), // sub_1412fbbb0
    Replacement::draft_throws(0x1412fbcb0, "unknown_1412fbcb0", fn_1412fbcb0 as *const (), Throws::Discard), // sub_1412fbcb0
    Replacement::draft_throws(0x1412fbdb0, "unknown_1412fbdb0", fn_1412fbdb0 as *const (), Throws::Discard), // sub_1412fbdb0
    Replacement::draft_throws(0x1412fbed0, "unknown_1412fbed0", fn_1412fbed0 as *const (), Throws::Discard), // sub_1412fbed0
    Replacement::draft_throws(0x1412fbfd0, "unknown_1412fbfd0", fn_1412fbfd0 as *const (), Throws::Discard), // sub_1412fbfd0
    Replacement::draft_throws(0x1412fc0f0, "unknown_1412fc0f0", fn_1412fc0f0 as *const (), Throws::Discard), // sub_1412fc0f0
    Replacement::draft_throws(0x1412fc1f0, "unknown_1412fc1f0", fn_1412fc1f0 as *const (), Throws::Discard), // sub_1412fc1f0
    Replacement::draft_throws(0x1412fc310, "unknown_1412fc310", fn_1412fc310 as *const (), Throws::Discard), // sub_1412fc310
    Replacement::draft_throws(0x1412fc430, "unknown_1412fc430", fn_1412fc430 as *const (), Throws::Discard), // sub_1412fc430
    Replacement::draft_throws(0x1412fc530, "unknown_1412fc530", fn_1412fc530 as *const (), Throws::Discard), // sub_1412fc530
    Replacement::draft(0x1412fc650, "unknown_1412fc650", fn_1412fc650 as *const ()), // sub_1412fc650
    Replacement::draft(0x1412fc6d0, "unknown_1412fc6d0", fn_1412fc6d0 as *const ()), // sub_1412fc6d0
    Replacement::draft_throws(0x1412fc730, "unknown_1412fc730", fn_1412fc730 as *const (), Throws::Discard), // sub_1412fc730
    Replacement::draft(0x1412fc7e0, "unknown_1412fc7e0", fn_1412fc7e0 as *const ()), // sub_1412fc7e0
    Replacement::draft(0x1412fc880, "unknown_1412fc880", tpl_18979::<0x30, 0x10, 0x18, 0x142302590, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412fc880
    Replacement::draft(0x1412fc920, "unknown_1412fc920", fn_1412fc920 as *const ()), // sub_1412fc920
    Replacement::draft_throws(0x1412fc9c0, "unknown_1412fc9c0", fn_1412fc9c0 as *const (), Throws::Caught), // sub_1412fc9c0
    Replacement::draft_throws(0x1412fca60, "unknown_1412fca60", fn_1412fca60 as *const (), Throws::Caught), // sub_1412fca60
    Replacement::draft_throws(0x1412fcbb0, "unknown_1412fcbb0", fn_1412fcbb0 as *const (), Throws::Caught), // sub_1412fcbb0
    Replacement::draft(0x1412fcc40, "unknown_1412fcc40", fn_1412fcc40 as *const ()), // sub_1412fcc40
    Replacement::draft_throws(0x1412fcce0, "unknown_1412fcce0", fn_1412fcce0 as *const (), Throws::Caught), // sub_1412fcce0
    Replacement::draft(0x1412fcd70, "unknown_1412fcd70", tpl_18926::<0x78, 0x10, 0x18, 0x1423cfa20, 0xffffffffffffffa0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412fcd70
    Replacement::draft_throws(0x1412fce10, "unknown_1412fce10", fn_1412fce10 as *const (), Throws::Caught), // sub_1412fce10
    Replacement::draft(0x1412fcf10, "unknown_1412fcf10", fn_1412fcf10 as *const ()), // sub_1412fcf10
    Replacement::draft(0x1412fcfb0, "unknown_1412fcfb0", fn_1412fcfb0 as *const ()), // sub_1412fcfb0
    Replacement::draft(0x1412fd050, "unknown_1412fd050", fn_1412fd050 as *const ()), // sub_1412fd050
    Replacement::draft(0x1412fd1e0, "unknown_1412fd1e0", fn_1412fd1e0 as *const ()), // sub_1412fd1e0
    Replacement::draft_throws(0x1412fd2f0, "unknown_1412fd2f0", fn_1412fd2f0 as *const (), Throws::Caught), // sub_1412fd2f0
    Replacement::draft(0x1412fd390, "unknown_1412fd390", tpl_22165::<0x148, 0x10, 0x18, 0x142200a90, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1412fd390
    Replacement::draft_throws(0x1412fd430, "unknown_1412fd430", fn_1412fd430 as *const (), Throws::Caught), // sub_1412fd430
    Replacement::draft(0x1412fd610, "unknown_1412fd610", fn_1412fd610 as *const ()), // sub_1412fd610
    Replacement::draft(0x1412fd6a0, "unknown_1412fd6a0", fn_1412fd6a0 as *const ()), // sub_1412fd6a0
    Replacement::draft(0x1412fd710, "unknown_1412fd710", fn_1412fd710 as *const ()), // sub_1412fd710
    Replacement::draft(0x1412fd7b0, "unknown_1412fd7b0", fn_1412fd7b0 as *const ()), // sub_1412fd7b0
    Replacement::draft_throws(0x1412fd830, "unknown_1412fd830", fn_1412fd830 as *const (), Throws::Caught), // sub_1412fd830
    Replacement::draft_throws(0x1412fd8d0, "unknown_1412fd8d0", fn_1412fd8d0 as *const (), Throws::Caught), // sub_1412fd8d0
    Replacement::draft(0x1412fd970, "unknown_1412fd970", fn_1412fd970 as *const ()), // sub_1412fd970
    Replacement::draft_throws(0x1412fdaa0, "unknown_1412fdaa0", fn_1412fdaa0 as *const (), Throws::Caught), // sub_1412fdaa0
    Replacement::draft_throws(0x1412fdb40, "unknown_1412fdb40", fn_1412fdb40 as *const (), Throws::Discard), // sub_1412fdb40
    Replacement::draft_throws(0x1412fdbe0, "unknown_1412fdbe0", fn_1412fdbe0 as *const (), Throws::Discard), // sub_1412fdbe0
    Replacement::draft(0x1412fdc60, "unknown_1412fdc60", fn_1412fdc60 as *const ()), // sub_1412fdc60
    Replacement::draft(0x1412fddb0, "unknown_1412fddb0", fn_1412fddb0 as *const ()), // sub_1412fddb0
    Replacement::draft_throws(0x1412fdeb0, "unknown_1412fdeb0", fn_1412fdeb0 as *const (), Throws::Discard), // sub_1412fdeb0
    Replacement::draft_throws(0x1412fdf50, "unknown_1412fdf50", fn_1412fdf50 as *const (), Throws::Caught), // sub_1412fdf50
    Replacement::draft_throws(0x1412fdff0, "unknown_1412fdff0", fn_1412fdff0 as *const (), Throws::Caught), // sub_1412fdff0
    Replacement::draft(0x1412fe090, "unknown_1412fe090", fn_1412fe090 as *const ()), // sub_1412fe090
    Replacement::draft_throws(0x1412fe120, "unknown_1412fe120", fn_1412fe120 as *const (), Throws::Caught), // sub_1412fe120
    Replacement::draft(0x1412fe210, "unknown_1412fe210", fn_1412fe210 as *const ()), // sub_1412fe210
    Replacement::draft(0x1412fe320, "unknown_1412fe320", fn_1412fe320 as *const ()), // sub_1412fe320
    Replacement::draft_throws(0x1412fe3f0, "unknown_1412fe3f0", fn_1412fe3f0 as *const (), Throws::Caught), // sub_1412fe3f0
    Replacement::draft(0x1412fe4a0, "unknown_1412fe4a0", fn_1412fe4a0 as *const ()), // sub_1412fe4a0
    Replacement::draft(0x1412fe570, "unknown_1412fe570", fn_1412fe570 as *const ()), // sub_1412fe570
    Replacement::draft(0x1412fe650, "unknown_1412fe650", fn_1412fe650 as *const ()), // sub_1412fe650
    Replacement::draft_throws(0x1412fe6f0, "unknown_1412fe6f0", fn_1412fe6f0 as *const (), Throws::Discard), // sub_1412fe6f0
    Replacement::draft(0x1412fe770, "unknown_1412fe770", fn_1412fe770 as *const ()), // sub_1412fe770
    Replacement::draft(0x1412fe810, "unknown_1412fe810", fn_1412fe810 as *const ()), // sub_1412fe810
    Replacement::draft(0x1412fe8c0, "unknown_1412fe8c0", fn_1412fe8c0 as *const ()), // sub_1412fe8c0
    Replacement::draft(0x1412fe960, "unknown_1412fe960", fn_1412fe960 as *const ()), // sub_1412fe960
    Replacement::draft(0x1412fea00, "unknown_1412fea00", fn_1412fea00 as *const ()), // sub_1412fea00
    Replacement::draft(0x1412fea80, "unknown_1412fea80", fn_1412fea80 as *const ()), // sub_1412fea80
    Replacement::draft(0x1412feaf0, "unknown_1412feaf0", fn_1412feaf0 as *const ()), // sub_1412feaf0
    Replacement::draft(0x1412feb90, "unknown_1412feb90", fn_1412feb90 as *const ()), // sub_1412feb90
    Replacement::draft(0x1412fec30, "unknown_1412fec30", fn_1412fec30 as *const ()), // sub_1412fec30
    Replacement::draft_throws(0x1412fecb0, "unknown_1412fecb0", fn_1412fecb0 as *const (), Throws::Caught), // sub_1412fecb0
    Replacement::draft(0x1412fed50, "unknown_1412fed50", fn_1412fed50 as *const ()), // sub_1412fed50
    Replacement::draft(0x1412fedf0, "unknown_1412fedf0", fn_1412fedf0 as *const ()), // sub_1412fedf0
    Replacement::draft(0x1412feea0, "unknown_1412feea0", fn_1412feea0 as *const ()), // sub_1412feea0
    Replacement::draft(0x1412fef70, "unknown_1412fef70", fn_1412fef70 as *const ()), // sub_1412fef70
    Replacement::draft(0x1412ff040, "unknown_1412ff040", fn_1412ff040 as *const ()), // sub_1412ff040
    Replacement::draft(0x1412ff1b0, "unknown_1412ff1b0", tpl_24112::<0x1433f4628, 0x1433f4634, 0x208, 0x8> as *const ()), // sub_1412ff1b0
    Replacement::draft(0x1412ff230, "unknown_1412ff230", tpl_24113::<0x1433f5a2c, 0x1433f5a20, 0x1433f5b13, 0x1433f5b08, 0x1433f5b7e, 0x1433f5b70, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4> as *const ()), // sub_1412ff230
    Replacement::draft(0x1412ff320, "unknown_1412ff320", fn_1412ff320 as *const ()), // sub_1412ff320
    Replacement::draft(0x1412ff3c0, "unknown_1412ff3c0", tpl_24113::<0x1433f5a1c, 0x1433f5a10, 0x1433f4634, 0x1433f4628, 0x1433f5b68, 0x1433f5b60, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4> as *const ()), // sub_1412ff3c0
    Replacement::draft(0x1412ff4b0, "unknown_1412ff4b0", tpl_24112::<0x1433f5b60, 0x1433f5b68, 0x208, 0x8> as *const ()), // sub_1412ff4b0
    Replacement::draft(0x1412ff530, "unknown_1412ff530", fn_1412ff530 as *const ()), // sub_1412ff530
    Replacement::draft(0x1412ff640, "unknown_1412ff640", tpl_24116::<0x1433f3b68, 0x10, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff640
    Replacement::draft(0x1412ff6d0, "unknown_1412ff6d0", tpl_24117::<0x1433f4298, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff6d0
    Replacement::draft(0x1412ff740, "unknown_1412ff740", tpl_24117::<0x1433f4168, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff740
    Replacement::draft(0x1412ff7b0, "unknown_1412ff7b0", tpl_24117::<0x1433f4260, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff7b0
    Replacement::draft(0x1412ff820, "unknown_1412ff820", tpl_24117::<0x1433f3600, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff820
    Replacement::draft(0x1412ff890, "unknown_1412ff890", tpl_24117::<0x1433f3c48, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff890
    Replacement::draft(0x1412ff900, "unknown_1412ff900", tpl_24118::<0x1433f3a08, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff900
    Replacement::draft(0x1412ff970, "unknown_1412ff970", tpl_24119::<0x1433f38c0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ff970
    Replacement::draft_throws(0x1412ff9e0, "unknown_1412ff9e0", fn_1412ff9e0 as *const (), Throws::Caught), // sub_1412ff9e0
    Replacement::draft(0x1412ffa90, "unknown_1412ffa90", tpl_24121::<0x1433f3498, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ffa90
    Replacement::draft(0x1412ffb10, "unknown_1412ffb10", tpl_24119::<0x1433f37a8, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ffb10
    Replacement::draft(0x1412ffb80, "unknown_1412ffb80", tpl_24122::<0x1433f3520, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ffb80
    Replacement::draft(0x1412ffc00, "unknown_1412ffc00", tpl_24117::<0x1433f33b8, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ffc00
    Replacement::draft(0x1412ffc70, "unknown_1412ffc70", tpl_24123::<0x1433f4634, 0x1433f4628, 0x1433f627c, 0x1433f6268, 0x143bbd1c0, 0x1413100d0, 0x8, 0x54, 0x60, 0x68> as *const ()), // sub_1412ffc70
    Replacement::draft(0x1412ffd10, "unknown_1412ffd10", fn_1412ffd10 as *const ()), // sub_1412ffd10
    Replacement::draft(0x1412ffdb0, "unknown_1412ffdb0", tpl_24123::<0x1433f557f, 0x1433f5570, 0x1433f5a43, 0x1433f5a30, 0x143bbd1c0, 0x1413100d0, 0x8, 0x54, 0x60, 0x68> as *const ()), // sub_1412ffdb0
    Replacement::draft(0x1412ffe50, "unknown_1412ffe50", tpl_24121::<0x1433f3960, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ffe50
    Replacement::draft(0x1412ffed0, "unknown_1412ffed0", tpl_24117::<0x1433f37e0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1412ffed0
    Replacement::draft_throws(0x1412fff40, "unknown_1412fff40", fn_1412fff40 as *const (), Throws::Discard), // sub_1412fff40
    Replacement::draft_throws(0x1412fffe0, "unknown_1412fffe0", fn_1412fffe0 as *const (), Throws::Discard), // sub_1412fffe0
    Replacement::draft_throws(0x141300080, "unknown_141300080", fn_141300080 as *const (), Throws::Caught), // sub_141300080
    Replacement::draft(0x141300130, "unknown_141300130", fn_141300130 as *const ()), // sub_141300130
    Replacement::draft(0x141300210, "unknown_141300210", tpl_24112::<0x1433ef040, 0x1433ef05f, 0x208, 0x8> as *const ()), // sub_141300210
    Replacement::draft(0x141300290, "unknown_141300290", tpl_24121::<0x1433f3fa8, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300290
    Replacement::draft(0x141300310, "unknown_141300310", fn_141300310 as *const ()), // sub_141300310
    Replacement::draft(0x1413003a0, "unknown_1413003a0", tpl_24116::<0x1433f4050, 0x10, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413003a0
    Replacement::draft(0x141300430, "unknown_141300430", tpl_24130::<0x1433f5c10, 0x1433f5c1a, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4, 0x18, 0x28> as *const ()), // sub_141300430
    Replacement::draft(0x1413004f0, "unknown_1413004f0", tpl_24116::<0x1433f3770, 0x10, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413004f0
    Replacement::draft(0x141300580, "unknown_141300580", fn_141300580 as *const ()), // sub_141300580
    Replacement::draft(0x141300640, "unknown_141300640", tpl_24123::<0x1433f5682, 0x1433f5668, 0x1433f6242, 0x1433f6220, 0x143bbd1c0, 0x1413100d0, 0x8, 0x54, 0x60, 0x68> as *const ()), // sub_141300640
    Replacement::draft(0x1413006e0, "unknown_1413006e0", tpl_24117::<0x1433f3cb8, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413006e0
    Replacement::draft(0x141300750, "unknown_141300750", tpl_24119::<0x1433f3850, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300750
    Replacement::draft(0x1413007c0, "unknown_1413007c0", fn_1413007c0 as *const ()), // sub_1413007c0
    Replacement::draft(0x141300830, "unknown_141300830", tpl_24118::<0x1433f4308, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300830
    Replacement::draft(0x1413008a0, "unknown_1413008a0", tpl_24122::<0x1433f3998, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413008a0
    Replacement::draft(0x141300920, "unknown_141300920", tpl_24121::<0x1433f3d60, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300920
    Replacement::draft(0x1413009a0, "unknown_1413009a0", tpl_24117::<0x1433f36a0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413009a0
    Replacement::draft(0x141300a10, "unknown_141300a10", tpl_24117::<0x1433f3f58, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300a10
    Replacement::draft(0x141300a80, "unknown_141300a80", tpl_24117::<0x1433f41f0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300a80
    Replacement::draft(0x141300af0, "unknown_141300af0", tpl_24121::<0x1433f34d0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300af0
    Replacement::draft(0x141300b70, "unknown_141300b70", tpl_24121::<0x1433f3cf0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300b70
    Replacement::draft(0x141300bf0, "unknown_141300bf0", fn_141300bf0 as *const ()), // sub_141300bf0
    Replacement::draft(0x141300c80, "unknown_141300c80", tpl_24112::<0x1433f54c8, 0x1433f54d8, 0x208, 0x8> as *const ()), // sub_141300c80
    Replacement::draft(0x141300d00, "unknown_141300d00", fn_141300d00 as *const ()), // sub_141300d00
    Replacement::draft(0x141300d90, "unknown_141300d90", tpl_24117::<0x1433f3d98, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300d90
    Replacement::draft(0x141300e00, "unknown_141300e00", tpl_24123::<0x1433f620d, 0x1433f61f8, 0x1433f580f, 0x1433f5800, 0x143bbd1c0, 0x1413100d0, 0x8, 0x54, 0x60, 0x68> as *const ()), // sub_141300e00
    Replacement::draft(0x141300ea0, "unknown_141300ea0", tpl_24117::<0x1433f3dd0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141300ea0
    Replacement::draft(0x141300f10, "unknown_141300f10", fn_141300f10 as *const ()), // sub_141300f10
    Replacement::draft_throws(0x141301020, "unknown_141301020", fn_141301020 as *const (), Throws::Discard), // sub_141301020
    Replacement::draft(0x1413010d0, "unknown_1413010d0", tpl_24123::<0x1433f5428, 0x1433f5410, 0x1433f5450, 0x1433f5430, 0x143bbd1c0, 0x1413100d0, 0x8, 0x54, 0x60, 0x68> as *const ()), // sub_1413010d0
    Replacement::draft(0x141301170, "unknown_141301170", tpl_24123::<0x1433f549d, 0x1433f5480, 0x1433f547d, 0x1433f5458, 0x143bbd1c0, 0x1413100d0, 0x8, 0x54, 0x60, 0x68> as *const ()), // sub_141301170
    Replacement::draft(0x141301210, "unknown_141301210", fn_141301210 as *const ()), // sub_141301210
    Replacement::draft(0x1413012a0, "unknown_1413012a0", tpl_24112::<0x1433f5790, 0x1433f57a8, 0x208, 0x8> as *const ()), // sub_1413012a0
    Replacement::draft(0x141301320, "unknown_141301320", tpl_24121::<0x1433f3a78, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301320
    Replacement::draft(0x1413013a0, "unknown_1413013a0", tpl_24117::<0x1433f35c8, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413013a0
    Replacement::draft(0x141301410, "unknown_141301410", tpl_24116::<0x1433f3ee8, 0x10, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301410
    Replacement::draft(0x1413014a0, "unknown_1413014a0", tpl_24117::<0x1433f4130, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413014a0
    Replacement::draft(0x141301510, "unknown_141301510", tpl_24122::<0x1433f3888, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301510
    Replacement::draft(0x141301590, "unknown_141301590", tpl_24116::<0x1433f3c10, 0x10, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301590
    Replacement::draft(0x141301620, "unknown_141301620", tpl_24117::<0x1433f40c0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301620
    Replacement::draft(0x141301690, "unknown_141301690", tpl_24121::<0x1433f39d0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301690
    Replacement::draft(0x141301710, "unknown_141301710", tpl_24117::<0x1433f3428, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141301710
    Replacement::draft(0x141301780, "unknown_141301780", tpl_24113::<0x1433f5f2b, 0x1433f5f18, 0x1433f5b13, 0x1433f5b08, 0x1433f4ddb, 0x1433f4dc8, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4> as *const ()), // sub_141301780
    Replacement::draft(0x141301870, "unknown_141301870", tpl_24113::<0x1433f620d, 0x1433f61f8, 0x1433f5efd, 0x1433f5ef0, 0x1433f4dc5, 0x1433f4db0, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4> as *const ()), // sub_141301870
    Replacement::draft(0x141301960, "unknown_141301960", tpl_24130::<0x1433f5480, 0x1433f549d, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4, 0x18, 0x28> as *const ()), // sub_141301960
    Replacement::draft(0x141301a20, "unknown_141301a20", tpl_24130::<0x1433f4f58, 0x1433f4f71, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4, 0x18, 0x28> as *const ()), // sub_141301a20
    Replacement::draft(0x141301ae0, "unknown_141301ae0", tpl_24130::<0x1433f45b0, 0x1433f45b9, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4, 0x18, 0x28> as *const ()), // sub_141301ae0
    Replacement::draft(0x141301ba0, "unknown_141301ba0", fn_141301ba0 as *const ()), // sub_141301ba0
    Replacement::draft(0x141301bb0, "unknown_141301bb0", tpl_24113::<0x1433f5b9a, 0x1433f5b90, 0x1433f4634, 0x1433f4628, 0x1433f615b, 0x1433f6150, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4> as *const ()), // sub_141301bb0
    Replacement::draft(0x141301ca0, "unknown_141301ca0", tpl_24139::<0x1433ef030, 0x1433ef03f, 0x1433f5ebe, 0x1433f5ea8, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141301ca0
    Replacement::draft(0x141301d90, "unknown_141301d90", tpl_24139::<0x1433ef060, 0x1433ef078, 0x1433f5ea7, 0x1433f5e88, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141301d90
    Replacement::draft(0x141301e80, "unknown_141301e80", tpl_24139::<0x1433ef040, 0x1433ef05f, 0x1433f5e86, 0x1433f5e60, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141301e80
    Replacement::draft(0x141301f70, "unknown_141301f70", tpl_24139::<0x1433ef338, 0x1433ef34d, 0x1433ef03f, 0x1433ef030, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141301f70
    Replacement::draft(0x141302060, "unknown_141302060", tpl_24139::<0x1433ef350, 0x1433ef36e, 0x1433ef078, 0x1433ef060, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141302060
    Replacement::draft(0x141302150, "unknown_141302150", tpl_24139::<0x1433ef2f8, 0x1433ef31d, 0x1433ef05f, 0x1433ef040, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141302150
    Replacement::draft(0x141302240, "unknown_141302240", tpl_24139::<0x1433f6160, 0x1433f6170, 0x1433f5ab3, 0x1433f5aa8, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x18, 0x4, 0x20, 0x28> as *const ()), // sub_141302240
    Replacement::draft(0x141302330, "unknown_141302330", fn_141302330 as *const ()), // sub_141302330
    Replacement::draft(0x1413023c0, "unknown_1413023c0", fn_1413023c0 as *const ()), // sub_1413023c0
    Replacement::draft(0x141302480, "unknown_141302480", tpl_24117::<0x1433f4340, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141302480
    Replacement::draft(0x1413024f0, "unknown_1413024f0", tpl_24113::<0x1433f5e47, 0x1433f5e38, 0x1433f5b13, 0x1433f5b08, 0x1433f5b7e, 0x1433f5b70, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4> as *const ()), // sub_1413024f0
    Replacement::draft(0x1413025e0, "unknown_1413025e0", tpl_24117::<0x1433f3eb0, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413025e0
    Replacement::draft(0x141302650, "unknown_141302650", tpl_24117::<0x1433f3f20, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141302650
    Replacement::draft(0x1413026c0, "unknown_1413026c0", tpl_24117::<0x1433f3650, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_1413026c0
    Replacement::draft(0x141302730, "unknown_141302730", tpl_24119::<0x1433f36d8, 0x143bbd168, 0x143bbd180, 0x14128cba0, 0x20> as *const ()), // sub_141302730
    Replacement::draft(0x1413027a0, "unknown_1413027a0", fn_1413027a0 as *const ()), // sub_1413027a0
    Replacement::draft(0x141302840, "unknown_141302840", tpl_24130::<0x1433ef040, 0x1433ef05f, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4, 0x18, 0x28> as *const ()), // sub_141302840
    Replacement::draft(0x141302900, "unknown_141302900", fn_141302900 as *const ()), // sub_141302900
    Replacement::draft_throws(0x1413029a0, "unknown_1413029a0", fn_1413029a0 as *const (), Throws::Caught), // sub_1413029a0
    Replacement::draft_throws(0x141302a50, "unknown_141302a50", fn_141302a50 as *const (), Throws::Caught), // sub_141302a50
    Replacement::draft_throws(0x141302b00, "unknown_141302b00", fn_141302b00 as *const (), Throws::Caught), // sub_141302b00
    Replacement::draft_throws(0x141302bc0, "unknown_141302bc0", fn_141302bc0 as *const (), Throws::Caught), // sub_141302bc0
    Replacement::draft(0x141302c80, "unknown_141302c80", fn_141302c80 as *const ()), // sub_141302c80
    Replacement::draft(0x141302d60, "unknown_141302d60", fn_141302d60 as *const ()), // sub_141302d60
    Replacement::draft(0x141302e60, "unknown_141302e60", fn_141302e60 as *const ()), // sub_141302e60
    Replacement::draft(0x141302f50, "unknown_141302f50", fn_141302f50 as *const ()), // sub_141302f50
    Replacement::draft(0x141302ff0, "unknown_141302ff0", tpl_24148::<0x18, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0, 0x10, 0x8> as *const ()), // sub_141302ff0
    Replacement::draft_throws(0x141303090, "unknown_141303090", fn_141303090 as *const (), Throws::Discard), // sub_141303090
    Replacement::draft_throws(0x1413030f0, "unknown_1413030f0", fn_1413030f0 as *const (), Throws::Discard), // sub_1413030f0
    Replacement::draft(0x141303180, "unknown_141303180", fn_141303180 as *const ()), // sub_141303180
    Replacement::draft_throws(0x141303210, "unknown_141303210", fn_141303210 as *const (), Throws::Discard), // sub_141303210
    Replacement::draft_throws(0x1413032b0, "unknown_1413032b0", fn_1413032b0 as *const (), Throws::Discard), // sub_1413032b0
    Replacement::draft_throws(0x1413033e0, "unknown_1413033e0", fn_1413033e0 as *const (), Throws::Discard), // sub_1413033e0
    Replacement::draft_throws(0x1413034f0, "unknown_1413034f0", fn_1413034f0 as *const (), Throws::Discard), // sub_1413034f0
    Replacement::draft(0x1413035a0, "unknown_1413035a0", fn_1413035a0 as *const ()), // sub_1413035a0
    Replacement::draft(0x141303600, "unknown_141303600", fn_141303600 as *const ()), // sub_141303600
    Replacement::draft(0x141303720, "unknown_141303720", fn_141303720 as *const ()), // sub_141303720
    Replacement::draft_throws(0x141303780, "unknown_141303780", fn_141303780 as *const (), Throws::Discard), // sub_141303780
    Replacement::draft_throws(0x1413037e0, "unknown_1413037e0", fn_1413037e0 as *const (), Throws::Discard), // sub_1413037e0
    Replacement::draft_throws(0x1413038d0, "unknown_1413038d0", fn_1413038d0 as *const (), Throws::Discard), // sub_1413038d0
    Replacement::draft(0x1413039a0, "unknown_1413039a0", fn_1413039a0 as *const ()), // sub_1413039a0
    Replacement::draft(0x141303a40, "unknown_141303a40", fn_141303a40 as *const ()), // sub_141303a40
    Replacement::draft(0x141303b30, "unknown_141303b30", fn_141303b30 as *const ()), // sub_141303b30
    Replacement::draft(0x141303ce0, "unknown_141303ce0", fn_141303ce0 as *const ()), // sub_141303ce0
    Replacement::draft_throws(0x141303d40, "unknown_141303d40", fn_141303d40 as *const (), Throws::Discard), // sub_141303d40
    Replacement::draft(0x141303e40, "unknown_141303e40", fn_141303e40 as *const ()), // sub_141303e40
    Replacement::draft(0x141303f00, "unknown_141303f00", fn_141303f00 as *const ()), // sub_141303f00
    Replacement::draft(0x141303fd0, "unknown_141303fd0", fn_141303fd0 as *const ()), // sub_141303fd0
    Replacement::draft_throws(0x1413040a0, "unknown_1413040a0", fn_1413040a0 as *const (), Throws::Discard), // sub_1413040a0
    Replacement::draft_throws(0x141304370, "unknown_141304370", fn_141304370 as *const (), Throws::Discard), // sub_141304370
    Replacement::draft_throws(0x141304400, "unknown_141304400", fn_141304400 as *const (), Throws::Caught), // sub_141304400
    Replacement::draft(0x141304500, "unknown_141304500", tpl_24173::<0x38, 0x1433f3a40, 0x8, 0x10, 0x18> as *const ()), // sub_141304500
    Replacement::draft(0x141304560, "unknown_141304560", fn_141304560 as *const ()), // sub_141304560
    Replacement::draft(0x141304590, "unknown_141304590", tpl_24173::<0x38, 0x1433f3bd8, 0x8, 0x10, 0x18> as *const ()), // sub_141304590
    Replacement::draft(0x1413045f0, "unknown_1413045f0", tpl_24173::<0x38, 0x1433f3e08, 0x8, 0x10, 0x18> as *const ()), // sub_1413045f0
    Replacement::draft(0x141304650, "unknown_141304650", tpl_24173::<0x38, 0x1433f3fe0, 0x8, 0x10, 0x18> as *const ()), // sub_141304650
    Replacement::draft(0x141304680, "unknown_141304680", fn_141304680 as *const ()), // sub_141304680
    Replacement::draft(0x1413046a0, "unknown_1413046a0", tpl_24173::<0x38, 0x1433f3e40, 0x8, 0x10, 0x18> as *const ()), // sub_1413046a0
    Replacement::draft(0x141304700, "unknown_141304700", fn_141304700 as *const ()), // sub_141304700
    Replacement::draft(0x1413048d0, "unknown_1413048d0", fn_1413048d0 as *const ()), // sub_1413048d0
    Replacement::draft_throws(0x141304950, "rend_141304950", fn_141304950 as *const (), Throws::Discard), // rend::RendererThreading::rend::RendererThreading::vfunc_0
    Replacement::draft(0x141304a70, "rend_141304a70", fn_141304a70 as *const ()), // sub_141304a70
    Replacement::draft(0x141304ab0, "rend_141304ab0", fn_141304ab0 as *const ()), // sub_141304ab0
    Replacement::draft(0x141304b90, "rend_141304b90", fn_141304b90 as *const ()), // sub_141304b90
    Replacement::draft(0x141304cf0, "rend_141304cf0", tpl_19776::<0x142407c80, 0x143c4aa00, 0x1426455a8, 0x142645598, 0x1422a3690, 0x142645580> as *const ()), // sub_141304cf0
    Replacement::draft(0x141304e30, "rend_141304e30", tpl_19776::<0x142407c80, 0x143c4aa00, 0x1426455a8, 0x142645598, 0x1422a3670, 0x142645580> as *const ()), // sub_141304e30
    Replacement::draft(0x141304f70, "rend_141304f70", fn_141304f70 as *const ()), // sub_141304f70
    Replacement::draft(0x1413050d0, "rend_1413050d0", fn_1413050d0 as *const ()), // sub_1413050d0
    Replacement::draft_throws(0x141305250, "rend_141305250", fn_141305250 as *const (), Throws::Caught), // sub_141305250
    Replacement::draft(0x1413053f0, "rend_1413053f0", fn_1413053f0 as *const ()), // sub_1413053f0
    Replacement::draft(0x141305680, "rend_141305680", fn_141305680 as *const ()), // sub_141305680
    Replacement::draft(0x1413057f0, "rend_1413057f0", fn_1413057f0 as *const ()), // sub_1413057f0
    Replacement::draft(0x141305870, "rend_141305870", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1433f68d8, 0x1412eb320> as *const ()), // sub_141305870
    Replacement::draft(0x1413058c0, "rend_1413058c0", tpl_24188::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1433f68b8, 0x1412eb320> as *const ()), // sub_1413058c0
    Replacement::draft(0x141305930, "rend_141305930", tpl_24189::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1433f68b8, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft(0x1413059b0, "rend_1413059b0", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1433f68d8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::vfun
    Replacement::draft(0x141305a20, "rend_141305a20", fn_141305a20 as *const ()), // sub_141305a20
    Replacement::draft_throws(0x141305bb0, "rend_141305bb0", fn_141305bb0 as *const (), Throws::Discard), // rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::vfun
    Replacement::draft(0x141305f20, "rend_141305f20", fn_141305f20 as *const ()), // rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::vfun
    Replacement::draft(0x141306060, "rend_141306060", fn_141306060 as *const ()), // rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::vfun
    Replacement::draft(0x141306100, "rend_141306100", fn_141306100 as *const ()), // rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadBuffer<d3d::RaytracingGPUInstanceDesc>::vfun
    Replacement::draft_throws(0x1413061a0, "rend_1413061a0", fn_1413061a0 as *const (), Throws::Discard), // rend::SparseUploadInternal<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadInternal<d3d::RaytracingGPUInstanceDesc>::
    Replacement::draft_throws(0x1413061e0, "rend_1413061e0", fn_1413061e0 as *const (), Throws::Discard), // rend::SparseUploadInternal<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadInternal<d3d::RaytracingGPUInstanceDesc>::
    Replacement::draft(0x141306220, "rend_141306220", fn_141306220 as *const ()), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft(0x141306240, "rend_141306240", fn_141306240 as *const ()), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft(0x141306340, "rend_141306340", tpl_24197::<0x58, 0xffffffffffffffff, 0x142229d10, 0x90, 0x98, 0x1412f8f30, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft(0x1413063e0, "rend_1413063e0", tpl_24198::<0x58, 0xffffffffffffffff, 0x142229c00, 0x90, 0x98, 0x1412f8f30, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft(0x1413064d0, "rend_1413064d0", tpl_24199::<0x142229a00, 0x1412f8f30, 0x140a5b830, 0x140fff1e0, 0x1412f8d90> as *const ()), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft_throws(0x141306620, "rend_141306620", fn_141306620 as *const (), Throws::Discard), // rend::TrackedSparseUploadBuffer<rend::RaytracingBindingData>::rend::TrackedSparseUploadBuffer<rend::RaytracingBindingDat
    Replacement::draft_throws(0x1413068f0, "rend_1413068f0", tpl_23986::<0x20, 0x29, 0x10, 0x8, 0x1422ab7b0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::RaytracingBindingData>::rend::SparseUploadInternal<rend::RaytracingBindingData>::vfunc_
    Replacement::draft_throws(0x141306930, "rend_141306930", tpl_23987::<0x29, 0x20, 0x10, 0x8, 0x1422ab7b0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::RaytracingBindingData>::rend::SparseUploadInternal<rend::RaytracingBindingData>::vfunc_
    Replacement::draft_throws(0x141306970, "rend_141306970", fn_141306970 as *const (), Throws::Discard), // sub_141306970
    Replacement::draft_throws(0x141306c00, "rend_141306c00", fn_141306c00 as *const (), Throws::Discard), // sub_141306c00
    Replacement::draft(0x141306e50, "rend_141306e50", fn_141306e50 as *const ()), // sub_141306e50
    Replacement::draft_throws(0x141307060, "rend_141307060", fn_141307060 as *const (), Throws::Discard), // sub_141307060
    Replacement::draft(0x141307190, "rend_141307190", tpl_23865::<0x1433f68d8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<d3d::RaytracingGPUInstanceDesc>::rend::SparseUploadInternal<d3d::RaytracingGPUInstanceDesc>::
    Replacement::draft(0x1413071d0, "rend_1413071d0", tpl_23865::<0x1433f68b8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::RaytracingBindingData>::rend::SparseUploadInternal<rend::RaytracingBindingData>::vfunc_
    Replacement::draft(0x141307210, "rend_141307210", fn_141307210 as *const ()), // sub_141307210
    Replacement::draft(0x1413072b0, "rend_1413072b0", fn_1413072b0 as *const ()), // sub_1413072b0
    Replacement::draft(0x141307360, "rend_141307360", fn_141307360 as *const ()), // sub_141307360
    Replacement::draft(0x141307420, "rend_141307420", fn_141307420 as *const ()), // sub_141307420
    Replacement::draft(0x141307490, "rend_141307490", fn_141307490 as *const ()), // sub_141307490
    Replacement::draft(0x141307860, "rend_141307860", fn_141307860 as *const ()), // sub_141307860
    Replacement::draft(0x141307be0, "rend_141307be0", fn_141307be0 as *const ()), // sub_141307be0
    Replacement::draft(0x141307ff0, "rend_141307ff0", fn_141307ff0 as *const ()), // sub_141307ff0
    Replacement::draft(0x141308530, "rend_141308530", fn_141308530 as *const ()), // sub_141308530
    Replacement::draft(0x1413086e0, "rend_1413086e0", tpl_19938::<0x8, 0x1413087a0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413086e0
    Replacement::draft(0x1413087a0, "rend_1413087a0", fn_1413087a0 as *const ()), // sub_1413087a0
    Replacement::draft(0x141308810, "rend_141308810", fn_141308810 as *const ()), // sub_141308810
    Replacement::draft(0x1413089b0, "rend_1413089b0", fn_1413089b0 as *const ()), // sub_1413089b0
    Replacement::draft(0x141308a00, "rend_141308a00", fn_141308a00 as *const ()), // sub_141308a00
    Replacement::draft_throws(0x141308a10, "rend_141308a10", fn_141308a10 as *const (), Throws::Discard), // sub_141308a10
    Replacement::draft(0x141308af0, "rend_141308af0", fn_141308af0 as *const ()), // sub_141308af0
    Replacement::draft(0x141308d30, "rend_141308d30", tpl_24212::<0x142330420, 0x141308ea0, 0x141308f40, 0x142330720> as *const ()), // sub_141308d30
    Replacement::draft(0x141308ea0, "rend_141308ea0", fn_141308ea0 as *const ()), // sub_141308ea0
    Replacement::draft(0x141308f40, "rend_141308f40", tpl_381::<0x1413090a0, 0x141309310> as *const ()), // sub_141308f40
    Replacement::draft(0x1413090a0, "rend_1413090a0", tpl_82529::<0x1434286b0, 0x143428770, 0x142330420, 0x1413095a0> as *const ()), // sub_1413090a0
    Replacement::draft(0x141309310, "rend_141309310", tpl_24214::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142330420, 0x1413095a0, 0x1425371a0, 0x142533200> as *const ()), // sub_141309310
    Replacement::draft(0x1413095a0, "rend_1413095a0", fn_1413095a0 as *const ()), // sub_1413095a0
    Replacement::draft_throws(0x141309760, "rend_141309760", tpl_22276::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_141309760
    Replacement::draft(0x1413097e0, "rend_1413097e0", fn_1413097e0 as *const ()), // sub_1413097e0
    Replacement::draft_throws(0x1413098b0, "rend_1413098b0", fn_1413098b0 as *const (), Throws::Discard), // sub_1413098b0
    Replacement::draft_throws(0x141309a00, "rend_141309a00", fn_141309a00 as *const (), Throws::Caught), // sub_141309a00
    Replacement::draft_throws(0x141309b70, "rend_141309b70", fn_141309b70 as *const (), Throws::Caught), // sub_141309b70
    Replacement::draft_throws(0x141309e20, "rend_141309e20", fn_141309e20 as *const (), Throws::Caught), // sub_141309e20
    Replacement::draft_throws(0x14130a0d0, "rend_14130a0d0", fn_14130a0d0 as *const (), Throws::Caught), // sub_14130a0d0
    Replacement::draft(0x14130a240, "rend_14130a240", tpl_24130::<0x1433f5668, 0x1433f5682, 0x143bbd170, 0x1413103c0, 0x143bbd168, 0x10, 0x20, 0x4, 0x18, 0x28> as *const ()), // sub_14130a240
    Replacement::draft(0x14130a300, "rend_14130a300", fn_14130a300 as *const ()), // sub_14130a300
    Replacement::draft(0x14130a340, "rend_14130a340", fn_14130a340 as *const ()), // sub_14130a340
    Replacement::draft_throws(0x14130a3f0, "rend_14130a3f0", fn_14130a3f0 as *const (), Throws::Discard), // sub_14130a3f0
    Replacement::draft(0x14130a610, "rend_14130a610", tpl_24212::<0x142330420, 0x141308ea0, 0x14130a780, 0x142330720> as *const ()), // sub_14130a610
    Replacement::draft(0x14130a780, "rend_14130a780", tpl_381::<0x14130a8e0, 0x14130ab50> as *const ()), // sub_14130a780
    Replacement::draft(0x14130a8e0, "rend_14130a8e0", tpl_82529::<0x1434286b0, 0x143428770, 0x142330420, 0x14130ade0> as *const ()), // sub_14130a8e0
    Replacement::draft(0x14130ab50, "rend_14130ab50", tpl_24214::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142330420, 0x14130ade0, 0x1425371a0, 0x142533200> as *const ()), // sub_14130ab50
    Replacement::draft(0x14130ade0, "rend_14130ade0", fn_14130ade0 as *const ()), // sub_14130ade0
    Replacement::draft_throws(0x14130af50, "rend_14130af50", tpl_22276::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_14130af50
    Replacement::draft(0x14130afd0, "rend_14130afd0", fn_14130afd0 as *const ()), // sub_14130afd0
    Replacement::draft(0x14130b0d0, "rend_14130b0d0", fn_14130b0d0 as *const ()), // sub_14130b0d0
    Replacement::draft(0x14130b160, "rend_14130b160", fn_14130b160 as *const ()), // sub_14130b160
    Replacement::draft(0x14130b240, "rend_14130b240", fn_14130b240 as *const ()), // sub_14130b240
    Replacement::draft(0x14130b3b0, "rend_14130b3b0", fn_14130b3b0 as *const ()), // sub_14130b3b0
    Replacement::draft(0x14130b550, "rend_14130b550", fn_14130b550 as *const ()), // sub_14130b550
    Replacement::draft(0x14130b660, "rend_14130b660", tpl_24189::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1433f8f18, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::TrackedSparseUploadBu
    Replacement::draft(0x14130b6e0, "rend_14130b6e0", tpl_24226::<0xffffffffffffffff, 0x142229d10, 0x14130c9c0, 0x14130c930, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::TrackedSparseUploadBuffer<rend::
    Replacement::draft(0x14130b7c0, "rend_14130b7c0", tpl_24227::<0x58, 0xffffffffffffffff, 0x142229d10, 0x90, 0x98, 0x14130ca90, 0x14130c930, 0xa8, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::TrackedSparseUploadBuffer<rend::
    Replacement::draft(0x14130b840, "rend_14130b840", tpl_24228::<0xffffffffffffffff, 0x142229c00, 0x14130ca90, 0x14130c930, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::TrackedSparseUploadBuffer<rend::
    Replacement::draft(0x14130be00, "rend_14130be00", fn_14130be00 as *const ()), // rend::SparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadBuffer<rend::PreSkinnedMesh
    Replacement::draft(0x14130be80, "rend_14130be80", tpl_23861::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x14130ca90, 0x14130c930, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadBuffer<rend::PreSkinnedMesh
    Replacement::draft(0x14130bf00, "rend_14130bf00", tpl_23862::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x14130ca90, 0x14130c930, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadBuffer<rend::PreSkinnedMesh
    Replacement::draft_throws(0x14130bf80, "rend_14130bf80", fn_14130bf80 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadInternal<rend::PreSkinned
    Replacement::draft_throws(0x14130bfc0, "rend_14130bfc0", fn_14130bfc0 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadInternal<rend::PreSkinned
    Replacement::draft(0x14130c000, "rend_14130c000", tpl_24226::<0xffffffffffffffff, 0x142229d10, 0x14130cfb0, 0x14130cf40, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::TrackedSparseUploadBu
    Replacement::draft(0x14130c0e0, "rend_14130c0e0", tpl_24227::<0x58, 0xffffffffffffffff, 0x142229d10, 0x90, 0x98, 0x14130d050, 0x14130cf40, 0xa8, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::TrackedSparseUploadBu
    Replacement::draft(0x14130c160, "rend_14130c160", tpl_24228::<0xffffffffffffffff, 0x142229c00, 0x14130d050, 0x14130cf40, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::TrackedSparseUploadBu
    Replacement::draft(0x14130c230, "rend_14130c230", fn_14130c230 as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::TrackedSparseUploadBu
    Replacement::draft_throws(0x14130c370, "rend_14130c370", fn_14130c370 as *const (), Throws::Discard), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::TrackedSparseUploadBu
    Replacement::draft_throws(0x14130c610, "rend_14130c610", tpl_24234::<0x20, 0x29, 0x10, 0x8, 0x1422e1ac0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::SparseUploadInternal<rend:
    Replacement::draft_throws(0x14130c650, "rend_14130c650", tpl_24235::<0x29, 0x20, 0x10, 0x8, 0x1422e1ac0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::SparseUploadInternal<rend:
    Replacement::draft(0x14130c690, "rend_14130c690", fn_14130c690 as *const ()), // sub_14130c690
    Replacement::draft_throws(0x14130c840, "rend_14130c840", fn_14130c840 as *const (), Throws::Discard), // sub_14130c840
    Replacement::draft_throws(0x14130c860, "rend_14130c860", fn_14130c860 as *const (), Throws::Discard), // sub_14130c860
    Replacement::draft(0x14130c8b0, "rend_14130c8b0", tpl_23865::<0x1433f8f38, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadInternal<rend::PreSkinned
    Replacement::draft(0x14130c8f0, "rend_14130c8f0", tpl_23865::<0x1433f8f18, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::PreSkinnedMeshManager::MeshletBoundsPage::GeometryId>::rend::SparseUploadInternal<rend:
    Replacement::draft(0x14130c930, "rend_14130c930", fn_14130c930 as *const ()), // sub_14130c930
    Replacement::draft(0x14130c9c0, "rend_14130c9c0", fn_14130c9c0 as *const ()), // sub_14130c9c0
    Replacement::draft(0x14130cba0, "rend_14130cba0", tpl_24240::<0x8, 0x14130e2a0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14130cba0
    Replacement::draft(0x14130cc70, "rend_14130cc70", tpl_2437::<0x8, 0x14130e360, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14130cc70
    Replacement::draft(0x14130cd40, "rend_14130cd40", fn_14130cd40 as *const ()), // sub_14130cd40
    Replacement::draft(0x14130cf40, "rend_14130cf40", fn_14130cf40 as *const ()), // sub_14130cf40
    Replacement::draft(0x14130cfb0, "rend_14130cfb0", tpl_82555::<0x8, 0xc, 0x1401deba0> as *const ()), // sub_14130cfb0
    Replacement::draft(0x14130d050, "rend_14130d050", fn_14130d050 as *const ()), // sub_14130d050
    Replacement::draft(0x14130d110, "rend_14130d110", fn_14130d110 as *const ()), // sub_14130d110
    Replacement::draft(0x14130d260, "rend_14130d260", tpl_24240::<0x8, 0x14130e3d0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14130d260
    Replacement::draft(0x14130d330, "rend_14130d330", fn_14130d330 as *const ()), // sub_14130d330
    Replacement::draft(0x14130d690, "rend_14130d690", fn_14130d690 as *const ()), // sub_14130d690
    Replacement::draft(0x14130d850, "rend_14130d850", fn_14130d850 as *const ()), // sub_14130d850
    Replacement::draft(0x14130da50, "rend_14130da50", fn_14130da50 as *const ()), // sub_14130da50
    Replacement::draft(0x14130dd10, "rend_14130dd10", fn_14130dd10 as *const ()), // sub_14130dd10
    Replacement::draft(0x14130de30, "rend_14130de30", fn_14130de30 as *const ()), // sub_14130de30
    Replacement::draft(0x14130df90, "rend_14130df90", tpl_24189::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1433f8f38, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::TrackedSparseUploadBuffer<rend::
    Replacement::draft(0x14130e010, "rend_14130e010", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1433f8f38, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::rend::SparseUploadBuffer<rend::PreSkinnedMesh
    Replacement::draft(0x14130e080, "rend_14130e080", fn_14130e080 as *const ()), // sub_14130e080
    Replacement::draft(0x14130e1f0, "rend_14130e1f0", fn_14130e1f0 as *const ()), // sub_14130e1f0
    Replacement::draft(0x14130e260, "rend_14130e260", tpl_24250::<0x8, 0x14130e3d0> as *const ()), // sub_14130e260
    Replacement::draft(0x14130e2a0, "rend_14130e2a0", fn_14130e2a0 as *const ()), // sub_14130e2a0
    Replacement::draft(0x14130e360, "rend_14130e360", fn_14130e360 as *const ()), // sub_14130e360
    Replacement::draft(0x14130e3d0, "rend_14130e3d0", fn_14130e3d0 as *const ()), // sub_14130e3d0
    Replacement::draft(0x14130e490, "rend_14130e490", fn_14130e490 as *const ()), // sub_14130e490
    Replacement::draft(0x14130e5c0, "rend_14130e5c0", fn_14130e5c0 as *const ()), // sub_14130e5c0
    Replacement::draft(0x14130e930, "rend_14130e930", tpl_24256::<0x270, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1423145a0> as *const ()), // sub_14130e930
    Replacement::draft(0x14130e9c0, "rend_14130e9c0", fn_14130e9c0 as *const ()), // sub_14130e9c0
    Replacement::draft_throws(0x14130eb40, "rend_14130eb40", fn_14130eb40 as *const (), Throws::Discard), // sub_14130eb40
    Replacement::draft_throws(0x14130ec10, "rend_14130ec10", fn_14130ec10 as *const (), Throws::Discard), // sub_14130ec10
    Replacement::draft_throws(0x14130eca0, "rend_14130eca0", fn_14130eca0 as *const (), Throws::Discard), // sub_14130eca0
    Replacement::draft(0x14130ed40, "rend_14130ed40", fn_14130ed40 as *const ()), // rend::ParticleSystemResource::rend::ParticleSystemResource::vfunc_0
    Replacement::draft(0x14130ed90, "rend_14130ed90", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433fa098, 0x8> as *const ()), // sub_14130ed90
    Replacement::draft(0x14130ee10, "rend_14130ee10", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433fa098, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_14130ee10
    Replacement::draft(0x14130eeb0, "rend_14130eeb0", fn_14130eeb0 as *const ()), // sub_14130eeb0
    Replacement::draft_throws(0x14130efa0, "rend_14130efa0", tpl_24263::<0x143881ba4, 0x143881ba8, 0x1401e9fa0, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_14130efa0
    Replacement::draft_throws(0x14130f080, "rend_14130f080", tpl_24263::<0x143881ba4, 0x143881ba8, 0x1401e9b60, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x1401e9a20> as *const (), Throws::Discard), // sub_14130f080
    Replacement::draft_throws(0x14130f160, "rend_14130f160", fn_14130f160 as *const (), Throws::Caught), // sub_14130f160
    Replacement::draft(0x14130f1f0, "rend_14130f1f0", fn_14130f1f0 as *const ()), // sub_14130f1f0
    Replacement::draft_throws(0x14130f2d0, "rend_14130f2d0", tpl_24263::<0x143881ba4, 0x143881ba8, 0x1401e9770, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x1401e9590> as *const (), Throws::Discard), // sub_14130f2d0
    Replacement::draft_throws(0x14130f3b0, "rend_14130f3b0", fn_14130f3b0 as *const (), Throws::Discard), // sub_14130f3b0
    Replacement::draft(0x14130f430, "rend_14130f430", fn_14130f430 as *const ()), // sub_14130f430
    Replacement::draft(0x14130f4d0, "rend_14130f4d0", fn_14130f4d0 as *const ()), // sub_14130f4d0
    Replacement::draft_throws(0x14130f740, "rend_14130f740", fn_14130f740 as *const (), Throws::Caught), // sub_14130f740
    Replacement::draft_throws(0x14130f8d0, "rend_14130f8d0", tpl_1905::<0x143881bb0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_14130f8d0
    Replacement::draft(0x14130f980, "rend_14130f980", tpl_23929::<0x142407c80, 0x143c4aa00, 0x140ea7e30, 0x142442de0, 0x1426455a8, 0x142645598, 0x1422f53f0, 0x142645580> as *const ()), // sub_14130f980
    Replacement::draft(0x14130fb30, "rend_14130fb30", fn_14130fb30 as *const ()), // sub_14130fb30
    Replacement::draft(0x14130fbe0, "rend_14130fbe0", fn_14130fbe0 as *const ()), // rend::ParticleEngine::rend::ParticleEngine::vfunc_0
    Replacement::draft(0x14130fd20, "rend_14130fd20", fn_14130fd20 as *const ()), // sub_14130fd20
    Replacement::draft(0x14130fd80, "rend_14130fd80", fn_14130fd80 as *const ()), // sub_14130fd80
    Replacement::draft(0x14130fe00, "rend_14130fe00", fn_14130fe00 as *const ()), // sub_14130fe00
    Replacement::draft(0x14130fef0, "rend_14130fef0", fn_14130fef0 as *const ()), // sub_14130fef0
    Replacement::draft(0x14130ff90, "unknown_14130ff90", tpl_170::<0x14130ffa0> as *const ()), // sub_14130ff90
    Replacement::draft(0x14130ffa0, "unknown_14130ffa0", tpl_13347::<0x8, 0x141310070, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14130ffa0
    Replacement::draft(0x141310070, "unknown_141310070", fn_141310070 as *const ()), // sub_141310070
    Replacement::draft(0x1413100d0, "unknown_1413100d0", fn_1413100d0 as *const ()), // sub_1413100d0
    Replacement::draft_throws(0x141310190, "unknown_141310190", fn_141310190 as *const (), Throws::Discard), // sub_141310190
    Replacement::draft_throws(0x1413103a0, "std_1413103a0", tpl_373::<0x1433fa5e0, 0x142645890, 0x1413103b1> as *const (), Throws::Discard), // sub_1413103a0
    Replacement::draft(0x141310450, "unknown_141310450", fn_141310450 as *const ()), // sub_141310450
    Replacement::draft(0x141310580, "unknown_141310580", fn_141310580 as *const ()), // sub_141310580
    Replacement::draft(0x141310840, "unknown_141310840", fn_141310840 as *const ()), // sub_141310840
    Replacement::draft(0x141310b10, "unknown_141310b10", fn_141310b10 as *const ()), // sub_141310b10
    Replacement::draft(0x141310ca0, "unknown_141310ca0", fn_141310ca0 as *const ()), // sub_141310ca0
    Replacement::draft(0x141311020, "unknown_141311020", fn_141311020 as *const ()), // sub_141311020
    Replacement::draft(0x141311160, "unknown_141311160", fn_141311160 as *const ()), // sub_141311160
    Replacement::draft(0x141311300, "unknown_141311300", fn_141311300 as *const ()), // sub_141311300
    Replacement::draft_throws(0x1413114b0, "unknown_1413114b0", fn_1413114b0 as *const (), Throws::Discard), // sub_1413114b0
    Replacement::draft(0x1413115b0, "unknown_1413115b0", fn_1413115b0 as *const ()), // sub_1413115b0
    Replacement::draft(0x141311660, "unknown_141311660", fn_141311660 as *const ()), // sub_141311660
    Replacement::draft(0x141311700, "rend_141311700", fn_141311700 as *const ()), // sub_141311700
    Replacement::draft(0x141311790, "rend_141311790", fn_141311790 as *const ()), // sub_141311790
    Replacement::draft(0x1413119e0, "rend_1413119e0", fn_1413119e0 as *const ()), // sub_1413119e0
    Replacement::draft(0x141311bc0, "rend_141311bc0", tpl_13432::<0x8, 0x141310070> as *const ()), // sub_141311bc0
    Replacement::draft(0x141311c00, "rend_141311c00", fn_141311c00 as *const ()), // sub_141311c00
    Replacement::draft(0x141311d10, "rend_141311d10", fn_141311d10 as *const ()), // sub_141311d10
    Replacement::draft(0x141311eb0, "rend_141311eb0", tpl_18313::<0x1413129e0, 0x14183acc8> as *const ()), // sub_141311eb0
    Replacement::draft(0x141311fa0, "rend_141311fa0", fn_141311fa0 as *const ()), // sub_141311fa0
    Replacement::draft(0x141312050, "rend_141312050", fn_141312050 as *const ()), // sub_141312050
    Replacement::draft(0x141312110, "rend_141312110", fn_141312110 as *const ()), // sub_141312110
    Replacement::draft(0x1413121f0, "rend_1413121f0", fn_1413121f0 as *const ()), // sub_1413121f0
    Replacement::draft(0x141312250, "rend_141312250", fn_141312250 as *const ()), // sub_141312250
    Replacement::draft(0x141312410, "rend_141312410", fn_141312410 as *const ()), // sub_141312410
    Replacement::draft(0x1413124e0, "rend_1413124e0", fn_1413124e0 as *const ()), // sub_1413124e0
    Replacement::draft(0x141312660, "rend_141312660", fn_141312660 as *const ()), // sub_141312660
    Replacement::draft(0x141312740, "rend_141312740", fn_141312740 as *const ()), // sub_141312740
    Replacement::draft(0x1413129e0, "rend_1413129e0", fn_1413129e0 as *const ()), // sub_1413129e0
    Replacement::draft(0x141312a40, "rend_141312a40", fn_141312a40 as *const ()), // sub_141312a40
    Replacement::draft(0x141312ad0, "rend_141312ad0", fn_141312ad0 as *const ()), // sub_141312ad0
    Replacement::draft(0x1413130a0, "rend_1413130a0", fn_1413130a0 as *const ()), // sub_1413130a0
    Replacement::draft(0x1413131e0, "rend_1413131e0", fn_1413131e0 as *const ()), // rend::MeshMetadata::rend::MeshMetadata::vfunc_0
    Replacement::draft(0x1413132a0, "rend_1413132a0", fn_1413132a0 as *const ()), // rend::MeshResource::rend::MeshResource::vfunc_0
    Replacement::draft(0x141313370, "rend_141313370", fn_141313370 as *const ()), // sub_141313370
    Replacement::draft(0x1413133e0, "rend_1413133e0", fn_1413133e0 as *const ()), // sub_1413133e0
    Replacement::draft(0x1413134d0, "rend_1413134d0", fn_1413134d0 as *const ()), // sub_1413134d0
    Replacement::draft(0x1413134f0, "rend_1413134f0", fn_1413134f0 as *const ()), // sub_1413134f0
    Replacement::draft(0x141313530, "rend_141313530", fn_141313530 as *const ()), // sub_141313530
    Replacement::draft(0x1413135c0, "rend_1413135c0", fn_1413135c0 as *const ()), // sub_1413135c0
    Replacement::draft(0x1413136f0, "rend_1413136f0", fn_1413136f0 as *const ()), // sub_1413136f0
    Replacement::draft_throws(0x1413139d0, "rend_1413139d0", fn_1413139d0 as *const (), Throws::Discard), // sub_1413139d0
    Replacement::draft(0x141313c10, "rend_141313c10", fn_141313c10 as *const ()), // sub_141313c10
    Replacement::draft(0x141314010, "rend_141314010", fn_141314010 as *const ()), // sub_141314010
    Replacement::draft_throws(0x1413140f0, "rend_1413140f0", fn_1413140f0 as *const (), Throws::Caught), // sub_1413140f0
    Replacement::draft_throws(0x1413143e0, "rend_1413143e0", fn_1413143e0 as *const (), Throws::Caught), // sub_1413143e0
    Replacement::draft_throws(0x1413146d0, "rend_1413146d0", fn_1413146d0 as *const (), Throws::Caught), // sub_1413146d0
    Replacement::draft(0x1413149c0, "rend_1413149c0", fn_1413149c0 as *const ()), // sub_1413149c0
    Replacement::draft(0x141314ae0, "rend_141314ae0", fn_141314ae0 as *const ()), // sub_141314ae0
    Replacement::draft(0x141314c40, "rend_141314c40", fn_141314c40 as *const ()), // sub_141314c40
    Replacement::draft_throws(0x141314da0, "rend_141314da0", fn_141314da0 as *const (), Throws::Discard), // sub_141314da0
    Replacement::draft_throws(0x141314f50, "rend_141314f50", fn_141314f50 as *const (), Throws::Discard), // sub_141314f50
    Replacement::draft(0x141315220, "rend_141315220", fn_141315220 as *const ()), // sub_141315220
    Replacement::draft(0x1413152e0, "rend_1413152e0", fn_1413152e0 as *const ()), // sub_1413152e0
    Replacement::draft(0x141315350, "rend_141315350", fn_141315350 as *const ()), // sub_141315350
    Replacement::draft(0x141315460, "rend_141315460", fn_141315460 as *const ()), // sub_141315460
    Replacement::draft_throws(0x1413155f0, "rend_1413155f0", fn_1413155f0 as *const (), Throws::Discard), // sub_1413155f0
    Replacement::draft(0x1413156e0, "rend_1413156e0", fn_1413156e0 as *const ()), // sub_1413156e0
    Replacement::draft(0x141315900, "rend_141315900", fn_141315900 as *const ()), // sub_141315900
    Replacement::draft_throws(0x141315a70, "rend_141315a70", fn_141315a70 as *const (), Throws::Caught), // sub_141315a70
    Replacement::draft_throws(0x141315bd0, "rend_141315bd0", fn_141315bd0 as *const (), Throws::Caught), // sub_141315bd0
    Replacement::draft_throws(0x141315d30, "rend_141315d30", fn_141315d30 as *const (), Throws::Caught), // sub_141315d30
    Replacement::draft(0x141315e90, "rend_141315e90", fn_141315e90 as *const ()), // sub_141315e90
    Replacement::draft(0x141315fa0, "rend_141315fa0", fn_141315fa0 as *const ()), // sub_141315fa0
    Replacement::draft(0x1413160a0, "rend_1413160a0", fn_1413160a0 as *const ()), // sub_1413160a0
    Replacement::draft_throws(0x1413161b0, "rend_1413161b0", fn_1413161b0 as *const (), Throws::Discard), // sub_1413161b0
    Replacement::draft_throws(0x1413162c0, "rend_1413162c0", fn_1413162c0 as *const (), Throws::Caught), // sub_1413162c0
    Replacement::draft(0x141316460, "rend_141316460", tpl_24256::<0x270, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1413130a0> as *const ()), // sub_141316460
    Replacement::draft(0x1413164f0, "rend_1413164f0", fn_1413164f0 as *const ()), // sub_1413164f0
    Replacement::draft(0x141316590, "rend_141316590", fn_141316590 as *const ()), // sub_141316590
    Replacement::draft(0x141316780, "rend_141316780", fn_141316780 as *const ()), // sub_141316780
    Replacement::draft_throws(0x141316b50, "rend_141316b50", fn_141316b50 as *const (), Throws::Caught), // sub_141316b50
    Replacement::draft_throws(0x141316ff0, "rend_141316ff0", fn_141316ff0 as *const (), Throws::Caught), // sub_141316ff0
    Replacement::draft(0x141317750, "rend_141317750", fn_141317750 as *const ()), // sub_141317750
    Replacement::draft_throws(0x141317910, "rend_141317910", fn_141317910 as *const (), Throws::Caught), // sub_141317910
    Replacement::draft_throws(0x141317e90, "rend_141317e90", fn_141317e90 as *const (), Throws::Caught), // sub_141317e90
    Replacement::draft_throws(0x1413183f0, "rend_1413183f0", fn_1413183f0 as *const (), Throws::Caught), // sub_1413183f0
    Replacement::draft(0x141318940, "rend_141318940", fn_141318940 as *const ()), // sub_141318940
    Replacement::draft(0x141318b90, "rend_141318b90", fn_141318b90 as *const ()), // sub_141318b90
    Replacement::draft(0x141318e50, "rend_141318e50", fn_141318e50 as *const ()), // sub_141318e50
    Replacement::draft_throws(0x141319060, "rend_141319060", fn_141319060 as *const (), Throws::Discard), // sub_141319060
    Replacement::draft_throws(0x1413192b0, "rend_1413192b0", fn_1413192b0 as *const (), Throws::Caught), // sub_1413192b0
    Replacement::draft(0x1413196a0, "rend_1413196a0", fn_1413196a0 as *const ()), // sub_1413196a0
    Replacement::draft(0x1413197c0, "rend_1413197c0", fn_1413197c0 as *const ()), // sub_1413197c0
    Replacement::draft(0x141319890, "rend_141319890", fn_141319890 as *const ()), // sub_141319890
    Replacement::draft(0x1413199a0, "rend_1413199a0", fn_1413199a0 as *const ()), // sub_1413199a0
    Replacement::draft_throws(0x141319c50, "rend_141319c50", fn_141319c50 as *const (), Throws::Discard), // sub_141319c50
    Replacement::draft(0x141319e80, "rend_141319e80", fn_141319e80 as *const ()), // sub_141319e80
    Replacement::draft_throws(0x14131a010, "rend_14131a010", fn_14131a010 as *const (), Throws::Discard), // sub_14131a010
    Replacement::draft(0x14131a260, "rend_14131a260", fn_14131a260 as *const ()), // sub_14131a260
    Replacement::draft_throws(0x14131a390, "rend_14131a390", fn_14131a390 as *const (), Throws::Caught), // sub_14131a390
    Replacement::draft_throws(0x14131a740, "rend_14131a740", fn_14131a740 as *const (), Throws::Caught), // sub_14131a740
    Replacement::draft_throws(0x14131aac0, "rend_14131aac0", fn_14131aac0 as *const (), Throws::Caught), // sub_14131aac0
    Replacement::draft(0x14131ae30, "rend_14131ae30", fn_14131ae30 as *const ()), // sub_14131ae30
    Replacement::draft(0x14131afa0, "rend_14131afa0", fn_14131afa0 as *const ()), // sub_14131afa0
    Replacement::draft(0x14131b130, "rend_14131b130", fn_14131b130 as *const ()), // sub_14131b130
    Replacement::draft_throws(0x14131b2d0, "rend_14131b2d0", fn_14131b2d0 as *const (), Throws::Discard), // sub_14131b2d0
    Replacement::draft_throws(0x14131b490, "rend_14131b490", fn_14131b490 as *const (), Throws::Discard), // sub_14131b490
    Replacement::draft(0x14131b690, "rend_14131b690", fn_14131b690 as *const ()), // sub_14131b690
    Replacement::draft(0x14131b7a0, "rend_14131b7a0", fn_14131b7a0 as *const ()), // sub_14131b7a0
    Replacement::draft(0x14131b8e0, "rend_14131b8e0", fn_14131b8e0 as *const ()), // sub_14131b8e0
    Replacement::draft(0x14131b970, "rend_14131b970", fn_14131b970 as *const ()), // sub_14131b970
    Replacement::draft_throws(0x14131bb20, "rend_14131bb20", fn_14131bb20 as *const (), Throws::Discard), // sub_14131bb20
    Replacement::draft(0x14131bea0, "rend_14131bea0", fn_14131bea0 as *const ()), // sub_14131bea0
    Replacement::draft_throws(0x14131c1e0, "rend_14131c1e0", fn_14131c1e0 as *const (), Throws::Discard), // sub_14131c1e0
    Replacement::draft(0x14131c5f0, "rend_14131c5f0", fn_14131c5f0 as *const ()), // sub_14131c5f0
    Replacement::draft_throws(0x14131c850, "rend_14131c850", fn_14131c850 as *const (), Throws::Caught), // sub_14131c850
    Replacement::draft_throws(0x14131ca40, "rend_14131ca40", fn_14131ca40 as *const (), Throws::Caught), // sub_14131ca40
    Replacement::draft_throws(0x14131cc30, "rend_14131cc30", fn_14131cc30 as *const (), Throws::Caught), // sub_14131cc30
    Replacement::draft(0x14131ce20, "rend_14131ce20", fn_14131ce20 as *const ()), // sub_14131ce20
    Replacement::draft(0x14131d0a0, "rend_14131d0a0", fn_14131d0a0 as *const ()), // sub_14131d0a0
    Replacement::draft(0x14131d3d0, "rend_14131d3d0", fn_14131d3d0 as *const ()), // sub_14131d3d0
    Replacement::draft_throws(0x14131d700, "rend_14131d700", fn_14131d700 as *const (), Throws::Discard), // sub_14131d700
    Replacement::draft_throws(0x14131da30, "rend_14131da30", fn_14131da30 as *const (), Throws::Discard), // sub_14131da30
    Replacement::draft(0x14131dc20, "rend_14131dc20", fn_14131dc20 as *const ()), // sub_14131dc20
    Replacement::draft(0x14131dce0, "rend_14131dce0", fn_14131dce0 as *const ()), // sub_14131dce0
    Replacement::draft_throws(0x14131df60, "rend_14131df60", fn_14131df60 as *const (), Throws::Caught), // sub_14131df60
    Replacement::draft_throws(0x14131e0f0, "rend_14131e0f0", tpl_1905::<0x143881bd8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_14131e0f0
    Replacement::draft(0x14131e1a0, "rend_14131e1a0", fn_14131e1a0 as *const ()), // rend::MeshStatisticsMetadata::rend::MeshStatisticsMetadata::vfunc_0
    Replacement::draft(0x14131e200, "rend_14131e200", fn_14131e200 as *const ()), // rend::MeshBonesMetadata::rend::MeshBonesMetadata::vfunc_0
    Replacement::draft(0x14131e290, "rend_14131e290", fn_14131e290 as *const ()), // sub_14131e290
    Replacement::draft_throws(0x14131e3d0, "rend_14131e3d0", fn_14131e3d0 as *const (), Throws::Caught), // sub_14131e3d0
    Replacement::draft_throws(0x14131e580, "rend_14131e580", fn_14131e580 as *const (), Throws::Caught), // sub_14131e580
    Replacement::draft_throws(0x14131e6b0, "rend_14131e6b0", fn_14131e6b0 as *const (), Throws::Caught), // sub_14131e6b0
    Replacement::draft_throws(0x14131e870, "rend_14131e870", fn_14131e870 as *const (), Throws::Caught), // sub_14131e870
    Replacement::draft_throws(0x14131e9b0, "rend_14131e9b0", fn_14131e9b0 as *const (), Throws::Caught), // sub_14131e9b0
    Replacement::draft_throws(0x14131eb00, "rend_14131eb00", fn_14131eb00 as *const (), Throws::Caught), // sub_14131eb00
    Replacement::draft_throws(0x14131ec50, "rend_14131ec50", fn_14131ec50 as *const (), Throws::Caught), // sub_14131ec50
    Replacement::draft_throws(0x14131ed90, "rend_14131ed90", tpl_12664::<0x20, 0x8, 0x141321d60> as *const (), Throws::Discard), // sub_14131ed90
    Replacement::draft_throws(0x14131ede0, "rend_14131ede0", fn_14131ede0 as *const (), Throws::Caught), // sub_14131ede0
    Replacement::draft_throws(0x14131f010, "rend_14131f010", fn_14131f010 as *const (), Throws::Caught), // sub_14131f010
    Replacement::draft_throws(0x14131f240, "rend_14131f240", fn_14131f240 as *const (), Throws::Caught), // sub_14131f240
    Replacement::draft_throws(0x14131f480, "rend_14131f480", fn_14131f480 as *const (), Throws::Caught), // sub_14131f480
    Replacement::draft_throws(0x14131f5c0, "rend_14131f5c0", fn_14131f5c0 as *const (), Throws::Caught), // sub_14131f5c0
    Replacement::draft_throws(0x14131f800, "rend_14131f800", fn_14131f800 as *const (), Throws::Caught), // sub_14131f800
    Replacement::draft_throws(0x14131f940, "rend_14131f940", fn_14131f940 as *const (), Throws::Caught), // sub_14131f940
    Replacement::draft_throws(0x14131fb80, "rend_14131fb80", fn_14131fb80 as *const (), Throws::Caught), // sub_14131fb80
    Replacement::draft_throws(0x14131fcc0, "rend_14131fcc0", fn_14131fcc0 as *const (), Throws::Caught), // sub_14131fcc0
    Replacement::draft_throws(0x14131fd30, "rend_14131fd30", fn_14131fd30 as *const (), Throws::Discard), // sub_14131fd30
    Replacement::draft_throws(0x14131fd90, "rend_14131fd90", fn_14131fd90 as *const (), Throws::Discard), // sub_14131fd90
    Replacement::draft_throws(0x14131fde0, "rend_14131fde0", tpl_546::<0x142407c80, 0x143c4aa00, 0x14230d2b0, 0x1426455a8, 0x142645598, 0x14230c620, 0x142645580> as *const (), Throws::Discard), // sub_14131fde0
    Replacement::draft_throws(0x14131ff40, "rend_14131ff40", fn_14131ff40 as *const (), Throws::Caught), // sub_14131ff40
    Replacement::draft(0x1413201d0, "rend_1413201d0", tpl_23688::<0x142407c80, 0x143c4aa00, 0x14230c810, 0x1426455a8, 0x142645598, 0x14230c520, 0x142645580> as *const ()), // sub_1413201d0
    Replacement::draft(0x141320320, "rend_141320320", fn_141320320 as *const ()), // sub_141320320
    Replacement::draft(0x141320490, "rend_141320490", fn_141320490 as *const ()), // sub_141320490
    Replacement::draft(0x1413204c0, "rend_1413204c0", fn_1413204c0 as *const ()), // sub_1413204c0
    Replacement::draft_throws(0x1413205d0, "rend_1413205d0", fn_1413205d0 as *const (), Throws::Caught), // sub_1413205d0
    Replacement::draft_throws(0x1413208b0, "rend_1413208b0", fn_1413208b0 as *const (), Throws::Caught), // sub_1413208b0
    Replacement::draft_throws(0x141320e60, "rend_141320e60", fn_141320e60 as *const (), Throws::Caught), // sub_141320e60
    Replacement::draft_throws(0x141321110, "rend_141321110", fn_141321110 as *const (), Throws::Caught), // sub_141321110
    Replacement::draft_throws(0x141321680, "rend_141321680", fn_141321680 as *const (), Throws::Caught), // sub_141321680
    Replacement::draft_throws(0x141321920, "rend_141321920", fn_141321920 as *const (), Throws::Caught), // sub_141321920
    Replacement::draft_throws(0x141321d60, "rend_141321d60", fn_141321d60 as *const (), Throws::Caught), // sub_141321d60
    Replacement::draft_throws(0x141321e30, "rend_141321e30", fn_141321e30 as *const (), Throws::Caught), // sub_141321e30
    Replacement::draft_throws(0x1413226a0, "rend_1413226a0", fn_1413226a0 as *const (), Throws::Caught), // sub_1413226a0
    Replacement::draft_throws(0x141322c50, "rend_141322c50", fn_141322c50 as *const (), Throws::Caught), // sub_141322c50
    Replacement::draft_throws(0x141323450, "rend_141323450", fn_141323450 as *const (), Throws::Caught), // sub_141323450
    Replacement::draft_throws(0x1413239c0, "rend_1413239c0", fn_1413239c0 as *const (), Throws::Caught), // sub_1413239c0
    Replacement::draft_throws(0x141324030, "rend_141324030", fn_141324030 as *const (), Throws::Caught), // sub_141324030
    Replacement::draft_throws(0x141324470, "rend_141324470", fn_141324470 as *const (), Throws::Caught), // sub_141324470
    Replacement::draft_throws(0x141324780, "rend_141324780", fn_141324780 as *const (), Throws::Caught), // sub_141324780
    Replacement::draft_throws(0x141324a50, "rend_141324a50", fn_141324a50 as *const (), Throws::Caught), // sub_141324a50
    Replacement::draft(0x141324d10, "rend_141324d10", fn_141324d10 as *const ()), // sub_141324d10
    Replacement::draft(0x141324d50, "rend_141324d50", fn_141324d50 as *const ()), // sub_141324d50
    Replacement::draft_throws(0x141324d90, "rend_141324d90", fn_141324d90 as *const (), Throws::Caught), // sub_141324d90
    Replacement::draft_throws(0x141325260, "rend_141325260", fn_141325260 as *const (), Throws::Caught), // sub_141325260
    Replacement::draft_throws(0x1413256f0, "rend_1413256f0", fn_1413256f0 as *const (), Throws::Caught), // sub_1413256f0
    Replacement::draft(0x141325b80, "rend_141325b80", tpl_12555::<0x1438820b0, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0xc, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141325b80
    Replacement::draft_throws(0x141325c60, "rend_141325c60", tpl_71031::<0x1438820b8, 0x1438820b0, 0x1401e9fa0, 0x328, 0x338, 0x38, 0x4, 0x8, 0x369, 0xc, 0x1401e9e70> as *const (), Throws::Discard), // sub_141325c60
    Replacement::draft(0x141325d30, "rend_141325d30", tpl_18531::<0x1438820b0, 0x14243b0b0, 0x338, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141325d30
    Replacement::draft(0x141325dc0, "rend_141325dc0", tpl_12556::<0x1438820b0, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141325dc0
    Replacement::draft_throws(0x141325e60, "rend_141325e60", tpl_12557::<0x1438820b8, 0x1438820b0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x8, 0x369, 0xc, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_141325e60
    Replacement::draft(0x141325f30, "rend_141325f30", tpl_12562::<0x1438820b8, 0x1438820b0, 0x1401e9970, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x1401e9860> as *const ()), // sub_141325f30
    Replacement::draft(0x141325fd0, "rend_141325fd0", tpl_12556::<0x1438820b0, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141325fd0
    Replacement::draft_throws(0x141326080, "rend_141326080", tpl_12563::<0x1438820b8, 0x1438820b0, 0x1401e9770, 0x338, 0x120, 0x8, 0xc, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_141326080
    Replacement::draft_throws(0x141326110, "rend_141326110", fn_141326110 as *const (), Throws::Caught), // sub_141326110
    Replacement::draft_throws(0x141326250, "rend_141326250", fn_141326250 as *const (), Throws::Caught), // sub_141326250
    Replacement::draft_throws(0x141326390, "rend_141326390", fn_141326390 as *const (), Throws::Caught), // sub_141326390
    Replacement::draft_throws(0x1413264d0, "rend_1413264d0", fn_1413264d0 as *const (), Throws::Caught), // sub_1413264d0
    Replacement::draft_throws(0x1413266c0, "rend_1413266c0", fn_1413266c0 as *const (), Throws::Caught), // sub_1413266c0
    Replacement::draft_throws(0x1413268e0, "rend_1413268e0", fn_1413268e0 as *const (), Throws::Caught), // sub_1413268e0
    Replacement::draft_throws(0x141326ad0, "rend_141326ad0", fn_141326ad0 as *const (), Throws::Caught), // sub_141326ad0
    Replacement::draft_throws(0x141326cc0, "rend_141326cc0", tpl_18545::<0x1438820b0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x20, 0x4, 0x8, 0x1433fa948, 0x1413c41a0, 0x143250720, 0x14020f240> as *const (), Throws::Discard), // sub_141326cc0
    Replacement::draft_throws(0x141326dd0, "rend_141326dd0", fn_141326dd0 as *const (), Throws::Caught), // sub_141326dd0
    Replacement::draft_throws(0x141327240, "rend_141327240", fn_141327240 as *const (), Throws::Caught), // sub_141327240
    Replacement::draft_throws(0x141327670, "rend_141327670", fn_141327670 as *const (), Throws::Caught), // sub_141327670
    Replacement::draft_throws(0x1413279c0, "rend_1413279c0", fn_1413279c0 as *const (), Throws::Discard), // sub_1413279c0
    Replacement::draft_throws(0x141327b30, "rend_141327b30", fn_141327b30 as *const (), Throws::Discard), // sub_141327b30
    Replacement::draft(0x141327e10, "rend_141327e10", tpl_60956::<0x19, 0x10, 0x141327e10, 0x1c, 0x14240ab10, 0x1403e8060> as *const ()), // sub_141327e10
    Replacement::draft(0x141327e90, "rend_141327e90", fn_141327e90 as *const ()), // sub_141327e90
    Replacement::draft(0x141327f30, "rend_141327f30", fn_141327f30 as *const ()), // sub_141327f30
    Replacement::draft(0x141327fa0, "rend_141327fa0", fn_141327fa0 as *const ()), // sub_141327fa0
    Replacement::draft(0x141328030, "rend_141328030", tpl_24256::<0x270, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1423145a0> as *const ()), // sub_141328030
    Replacement::draft(0x1413280c0, "rend_1413280c0", fn_1413280c0 as *const ()), // sub_1413280c0
    Replacement::draft(0x141328180, "rend_141328180", fn_141328180 as *const ()), // sub_141328180
    Replacement::draft(0x1413282c0, "rend_1413282c0", fn_1413282c0 as *const ()), // sub_1413282c0
    Replacement::draft_throws(0x1413285a0, "rend_1413285a0", fn_1413285a0 as *const (), Throws::Caught), // sub_1413285a0
    Replacement::draft_throws(0x1413287f0, "rend_1413287f0", fn_1413287f0 as *const (), Throws::Caught), // sub_1413287f0
    Replacement::draft_throws(0x141328a50, "rend_141328a50", fn_141328a50 as *const (), Throws::Caught), // sub_141328a50
    Replacement::draft(0x141328c50, "rend_141328c50", fn_141328c50 as *const ()), // sub_141328c50
    Replacement::draft_throws(0x141328ed0, "rend_141328ed0", fn_141328ed0 as *const (), Throws::Caught), // sub_141328ed0
    Replacement::draft_throws(0x141329310, "rend_141329310", fn_141329310 as *const (), Throws::Caught), // sub_141329310
    Replacement::draft_throws(0x1413296d0, "rend_1413296d0", fn_1413296d0 as *const (), Throws::Caught), // sub_1413296d0
    Replacement::draft(0x141329a80, "rend_141329a80", fn_141329a80 as *const ()), // sub_141329a80
    Replacement::draft(0x141329c80, "rend_141329c80", fn_141329c80 as *const ()), // sub_141329c80
    Replacement::draft(0x141329e80, "rend_141329e80", fn_141329e80 as *const ()), // sub_141329e80
    Replacement::draft_throws(0x14132a0a0, "rend_14132a0a0", fn_14132a0a0 as *const (), Throws::Discard), // sub_14132a0a0
    Replacement::draft_throws(0x14132a2d0, "rend_14132a2d0", fn_14132a2d0 as *const (), Throws::Discard), // sub_14132a2d0
    Replacement::draft(0x14132a4a0, "rend_14132a4a0", fn_14132a4a0 as *const ()), // sub_14132a4a0
    Replacement::draft(0x14132a580, "rend_14132a580", tpl_17836::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14132df50> as *const ()), // sub_14132a580
    Replacement::draft(0x14132a600, "rend_14132a600", fn_14132a600 as *const ()), // sub_14132a600
    Replacement::draft(0x14132a6c0, "rend_14132a6c0", fn_14132a6c0 as *const ()), // sub_14132a6c0
    Replacement::draft_throws(0x14132a970, "rend_14132a970", fn_14132a970 as *const (), Throws::Discard), // sub_14132a970
    Replacement::draft(0x14132aba0, "rend_14132aba0", fn_14132aba0 as *const ()), // sub_14132aba0
    Replacement::draft_throws(0x14132ad30, "rend_14132ad30", fn_14132ad30 as *const (), Throws::Discard), // sub_14132ad30
    Replacement::draft(0x14132af30, "rend_14132af30", fn_14132af30 as *const ()), // sub_14132af30
    Replacement::draft_throws(0x14132b000, "rend_14132b000", fn_14132b000 as *const (), Throws::Caught), // sub_14132b000
    Replacement::draft_throws(0x14132b340, "rend_14132b340", fn_14132b340 as *const (), Throws::Caught), // sub_14132b340
    Replacement::draft_throws(0x14132b650, "rend_14132b650", fn_14132b650 as *const (), Throws::Caught), // sub_14132b650
    Replacement::draft(0x14132b950, "rend_14132b950", fn_14132b950 as *const ()), // sub_14132b950
    Replacement::draft(0x14132baf0, "rend_14132baf0", fn_14132baf0 as *const ()), // sub_14132baf0
    Replacement::draft(0x14132bc70, "rend_14132bc70", fn_14132bc70 as *const ()), // sub_14132bc70
    Replacement::draft_throws(0x14132bdf0, "rend_14132bdf0", fn_14132bdf0 as *const (), Throws::Discard), // sub_14132bdf0
    Replacement::draft_throws(0x14132bfa0, "rend_14132bfa0", fn_14132bfa0 as *const (), Throws::Discard), // sub_14132bfa0
    Replacement::draft(0x14132c140, "rend_14132c140", tpl_24432::<0x98, 0x10, 0x18, 0xffffffffffffff80, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141327e90> as *const ()), // sub_14132c140
    Replacement::draft(0x14132c1c0, "rend_14132c1c0", fn_14132c1c0 as *const ()), // sub_14132c1c0
    Replacement::draft(0x14132c2e0, "rend_14132c2e0", fn_14132c2e0 as *const ()), // sub_14132c2e0
    Replacement::draft(0x14132c380, "rend_14132c380", fn_14132c380 as *const ()), // sub_14132c380
    Replacement::draft_throws(0x14132c4a0, "rend_14132c4a0", fn_14132c4a0 as *const (), Throws::Discard), // sub_14132c4a0
    Replacement::draft(0x14132c610, "rend_14132c610", fn_14132c610 as *const ()), // sub_14132c610
    Replacement::draft_throws(0x14132c6d0, "rend_14132c6d0", fn_14132c6d0 as *const (), Throws::Discard), // sub_14132c6d0
    Replacement::draft(0x14132c8a0, "rend_14132c8a0", fn_14132c8a0 as *const ()), // sub_14132c8a0
    Replacement::draft_throws(0x14132c960, "rend_14132c960", fn_14132c960 as *const (), Throws::Caught), // sub_14132c960
    Replacement::draft_throws(0x14132ca90, "rend_14132ca90", fn_14132ca90 as *const (), Throws::Caught), // sub_14132ca90
    Replacement::draft_throws(0x14132cbc0, "rend_14132cbc0", fn_14132cbc0 as *const (), Throws::Caught), // sub_14132cbc0
    Replacement::draft(0x14132ce30, "rend_14132ce30", fn_14132ce30 as *const ()), // sub_14132ce30
    Replacement::draft(0x14132cf30, "rend_14132cf30", fn_14132cf30 as *const ()), // sub_14132cf30
    Replacement::draft(0x14132d000, "rend_14132d000", fn_14132d000 as *const ()), // sub_14132d000
    Replacement::draft_throws(0x14132d0e0, "rend_14132d0e0", fn_14132d0e0 as *const (), Throws::Discard), // sub_14132d0e0
    Replacement::draft_throws(0x14132d210, "rend_14132d210", fn_14132d210 as *const (), Throws::Discard), // sub_14132d210
    Replacement::draft(0x14132d370, "rend_14132d370", fn_14132d370 as *const ()), // sub_14132d370
    Replacement::draft_throws(0x14132d410, "rend_14132d410", fn_14132d410 as *const (), Throws::Discard), // sub_14132d410
    Replacement::draft(0x14132d4f0, "rend_14132d4f0", fn_14132d4f0 as *const ()), // sub_14132d4f0
    Replacement::draft_throws(0x14132d5b0, "rend_14132d5b0", tpl_12229::<0x143881c38, 0x143881c10, 0x1401e9b60, 0x1402c4ca0, 0x328, 0x338, 0x120, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_14132d5b0
    Replacement::draft_throws(0x14132d650, "rend_14132d650", fn_14132d650 as *const (), Throws::Caught), // sub_14132d650
    Replacement::draft_throws(0x14132d730, "rend_14132d730", fn_14132d730 as *const (), Throws::Caught), // sub_14132d730
    Replacement::draft_throws(0x14132d810, "rend_14132d810", fn_14132d810 as *const (), Throws::Caught), // sub_14132d810
    Replacement::draft(0x14132d8f0, "rend_14132d8f0", fn_14132d8f0 as *const ()), // sub_14132d8f0
    Replacement::draft(0x14132d9b0, "rend_14132d9b0", fn_14132d9b0 as *const ()), // sub_14132d9b0
    Replacement::draft(0x14132da50, "rend_14132da50", fn_14132da50 as *const ()), // sub_14132da50
    Replacement::draft_throws(0x14132db00, "rend_14132db00", fn_14132db00 as *const (), Throws::Discard), // sub_14132db00
    Replacement::draft_throws(0x14132dbc0, "rend_14132dbc0", fn_14132dbc0 as *const (), Throws::Discard), // sub_14132dbc0
    Replacement::draft(0x14132dc90, "rend_14132dc90", fn_14132dc90 as *const ()), // sub_14132dc90
    Replacement::draft_throws(0x14132dd60, "rend_14132dd60", fn_14132dd60 as *const (), Throws::Caught), // sub_14132dd60
    Replacement::draft(0x14132de20, "rend_14132de20", fn_14132de20 as *const ()), // sub_14132de20
    Replacement::draft(0x14132df00, "rend_14132df00", fn_14132df00 as *const ()), // sub_14132df00
    Replacement::draft(0x14132df50, "rend_14132df50", fn_14132df50 as *const ()), // sub_14132df50
    Replacement::draft_throws(0x14132dfe0, "rend_14132dfe0", fn_14132dfe0 as *const (), Throws::Caught), // sub_14132dfe0
    Replacement::draft_throws(0x14132e190, "rend_14132e190", fn_14132e190 as *const (), Throws::Caught), // sub_14132e190
    Replacement::draft_throws(0x14132e340, "rend_14132e340", fn_14132e340 as *const (), Throws::Caught), // sub_14132e340
    Replacement::draft_throws(0x14132e500, "rend_14132e500", fn_14132e500 as *const (), Throws::Caught), // sub_14132e500
    Replacement::draft_throws(0x14132e740, "rend_14132e740", tpl_22963::<0x20, 0x8, 0x1413321b0> as *const (), Throws::Discard), // sub_14132e740
    Replacement::draft_throws(0x14132e790, "rend_14132e790", tpl_17477::<0x20, 0x8, 0x141332280> as *const (), Throws::Discard), // sub_14132e790
    Replacement::draft_throws(0x14132e7e0, "rend_14132e7e0", fn_14132e7e0 as *const (), Throws::Caught), // sub_14132e7e0
    Replacement::draft_throws(0x14132e9b0, "rend_14132e9b0", fn_14132e9b0 as *const (), Throws::Caught), // sub_14132e9b0
    Replacement::draft_throws(0x14132eb60, "rend_14132eb60", fn_14132eb60 as *const (), Throws::Caught), // sub_14132eb60
    Replacement::draft(0x14132edf0, "rend_14132edf0", fn_14132edf0 as *const ()), // sub_14132edf0
    Replacement::draft(0x14132ef50, "rend_14132ef50", tpl_81305::<0x8, 0xc, 0x14132f030, 0x14132df50, 0x14132df00> as *const ()), // sub_14132ef50
    Replacement::draft(0x14132f030, "rend_14132f030", fn_14132f030 as *const ()), // sub_14132f030
    Replacement::draft(0x14132f0e0, "rend_14132f0e0", fn_14132f0e0 as *const ()), // sub_14132f0e0
    Replacement::draft(0x14132f1d0, "rend_14132f1d0", fn_14132f1d0 as *const ()), // sub_14132f1d0
    Replacement::draft(0x14132f3c0, "rend_14132f3c0", fn_14132f3c0 as *const ()), // sub_14132f3c0
    Replacement::draft(0x14132f6b0, "rend_14132f6b0", fn_14132f6b0 as *const ()), // sub_14132f6b0
    Replacement::draft_throws(0x14132fca0, "rend_14132fca0", fn_14132fca0 as *const (), Throws::Caught), // sub_14132fca0
    Replacement::draft_throws(0x14132ffa0, "rend_14132ffa0", fn_14132ffa0 as *const (), Throws::Caught), // sub_14132ffa0
    Replacement::draft_throws(0x1413302a0, "rend_1413302a0", fn_1413302a0 as *const (), Throws::Caught), // sub_1413302a0
    Replacement::draft_throws(0x141330740, "rend_141330740", fn_141330740 as *const (), Throws::Caught), // sub_141330740
    Replacement::draft_throws(0x141330a40, "rend_141330a40", fn_141330a40 as *const (), Throws::Caught), // sub_141330a40
    Replacement::draft_throws(0x141330d40, "rend_141330d40", fn_141330d40 as *const (), Throws::Caught), // sub_141330d40
    Replacement::draft_throws(0x1413311b0, "rend_1413311b0", fn_1413311b0 as *const (), Throws::Caught), // sub_1413311b0
    Replacement::draft_throws(0x141331470, "rend_141331470", fn_141331470 as *const (), Throws::Caught), // sub_141331470
    Replacement::draft_throws(0x141331730, "rend_141331730", fn_141331730 as *const (), Throws::Caught), // sub_141331730
    Replacement::draft_throws(0x141331ba0, "rend_141331ba0", fn_141331ba0 as *const (), Throws::Caught), // sub_141331ba0
    Replacement::draft_throws(0x141331e40, "rend_141331e40", fn_141331e40 as *const (), Throws::Caught), // sub_141331e40
    Replacement::draft_throws(0x1413320e0, "rend_1413320e0", fn_1413320e0 as *const (), Throws::Caught), // sub_1413320e0
    Replacement::draft_throws(0x1413321b0, "rend_1413321b0", fn_1413321b0 as *const (), Throws::Caught), // sub_1413321b0
    Replacement::draft_throws(0x141332280, "rend_141332280", fn_141332280 as *const (), Throws::Caught), // sub_141332280
    Replacement::draft(0x141332350, "rend_141332350", fn_141332350 as *const ()), // sub_141332350
    Replacement::draft(0x141332420, "rend_141332420", fn_141332420 as *const ()), // sub_141332420
    Replacement::draft(0x141332460, "rend_141332460", fn_141332460 as *const ()), // sub_141332460
    Replacement::draft_throws(0x1413324b0, "rend_1413324b0", fn_1413324b0 as *const (), Throws::Caught), // sub_1413324b0
    Replacement::draft_throws(0x141332720, "rend_141332720", fn_141332720 as *const (), Throws::Caught), // sub_141332720
    Replacement::draft_throws(0x141332bf0, "rend_141332bf0", fn_141332bf0 as *const (), Throws::Caught), // sub_141332bf0
    Replacement::draft_throws(0x1413330c0, "rend_1413330c0", fn_1413330c0 as *const (), Throws::Caught), // sub_1413330c0
    Replacement::draft_throws(0x141333320, "rend_141333320", fn_141333320 as *const (), Throws::Caught), // sub_141333320
    Replacement::draft_throws(0x1413337b0, "rend_1413337b0", fn_1413337b0 as *const (), Throws::Caught), // sub_1413337b0
    Replacement::draft_throws(0x141333c40, "rend_141333c40", fn_141333c40 as *const (), Throws::Caught), // sub_141333c40
    Replacement::draft_throws(0x141333ea0, "rend_141333ea0", fn_141333ea0 as *const (), Throws::Caught), // sub_141333ea0
    Replacement::draft_throws(0x141334330, "rend_141334330", fn_141334330 as *const (), Throws::Caught), // sub_141334330
    Replacement::draft_throws(0x1413347c0, "rend_1413347c0", fn_1413347c0 as *const (), Throws::Caught), // sub_1413347c0
    Replacement::draft_throws(0x141334d70, "rend_141334d70", fn_141334d70 as *const (), Throws::Caught), // sub_141334d70
    Replacement::draft_throws(0x141334eb0, "rend_141334eb0", fn_141334eb0 as *const (), Throws::Caught), // sub_141334eb0
    Replacement::draft_throws(0x141334ff0, "rend_141334ff0", fn_141334ff0 as *const (), Throws::Caught), // sub_141334ff0
    Replacement::draft_throws(0x141335560, "rend_141335560", fn_141335560 as *const (), Throws::Caught), // sub_141335560
    Replacement::draft_throws(0x1413356a0, "rend_1413356a0", fn_1413356a0 as *const (), Throws::Caught), // sub_1413356a0
    Replacement::draft_throws(0x1413357e0, "rend_1413357e0", fn_1413357e0 as *const (), Throws::Caught), // sub_1413357e0
    Replacement::draft_throws(0x141335b30, "rend_141335b30", fn_141335b30 as *const (), Throws::Caught), // sub_141335b30
    Replacement::draft_throws(0x141335c70, "rend_141335c70", fn_141335c70 as *const (), Throws::Caught), // sub_141335c70
    Replacement::draft_throws(0x141335db0, "rend_141335db0", fn_141335db0 as *const (), Throws::Caught), // sub_141335db0
    Replacement::draft_throws(0x141336020, "rend_141336020", fn_141336020 as *const (), Throws::Caught), // sub_141336020
    Replacement::draft_throws(0x141336210, "rend_141336210", fn_141336210 as *const (), Throws::Caught), // sub_141336210
    Replacement::draft_throws(0x141336400, "rend_141336400", fn_141336400 as *const (), Throws::Caught), // sub_141336400
    Replacement::draft_throws(0x141336870, "rend_141336870", fn_141336870 as *const (), Throws::Caught), // sub_141336870
    Replacement::draft_throws(0x141336ce0, "rend_141336ce0", fn_141336ce0 as *const (), Throws::Caught), // sub_141336ce0
    Replacement::draft_throws(0x141337110, "rend_141337110", fn_141337110 as *const (), Throws::Caught), // sub_141337110
    Replacement::draft_throws(0x141337540, "rend_141337540", fn_141337540 as *const (), Throws::Caught), // sub_141337540
    Replacement::draft_throws(0x141337890, "rend_141337890", fn_141337890 as *const (), Throws::Caught), // sub_141337890
    Replacement::draft(0x141337be0, "rend_141337be0", fn_141337be0 as *const ()), // sub_141337be0
    Replacement::draft(0x141337c40, "rend_141337c40", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1433fcd40, 0x1412eb320> as *const ()), // sub_141337c40
    Replacement::draft(0x141337c90, "rend_141337c90", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1433fccd8, 0x1412eb320> as *const ()), // sub_141337c90
    Replacement::draft(0x141337ce0, "rend_141337ce0", fn_141337ce0 as *const ()), // sub_141337ce0
    Replacement::draft(0x141337d50, "rend_141337d50", fn_141337d50 as *const ()), // sub_141337d50
    Replacement::draft(0x141337dd0, "rend_141337dd0", fn_141337dd0 as *const ()), // sub_141337dd0
    Replacement::draft(0x141337e00, "rend_141337e00", fn_141337e00 as *const ()), // sub_141337e00
    Replacement::draft(0x141337e40, "rend_141337e40", fn_141337e40 as *const ()), // sub_141337e40
    Replacement::draft(0x141337e70, "rend_141337e70", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1433fccd8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectCommandVS>::rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectComm
    Replacement::draft(0x141337ee0, "rend_141337ee0", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1433fcd40, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDr
    Replacement::draft_throws(0x141337f60, "rend_141337f60", tpl_24489::<0x1408ce8a0, 0x143868280, 0x142261020, 0x1412f9440, 0x14209c560, 0x14183acbc, 0x14209d6f0, 0x142320000, 0x143868298> as *const (), Throws::Discard), // rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDr
    Replacement::draft(0x141338220, "rend_141338220", tpl_23984::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x1413399b0, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDr
    Replacement::draft(0x1413382b0, "rend_1413382b0", tpl_23985::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x1413399b0, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadBuffer<rend::MeshEngineGPU::PerDr
    Replacement::draft_throws(0x141338350, "rend_141338350", tpl_23986::<0x20, 0x29, 0x10, 0x8, 0x142320000> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadInternal<rend::MeshEngineGPU::P
    Replacement::draft_throws(0x141338390, "rend_141338390", tpl_23987::<0x29, 0x20, 0x10, 0x8, 0x142320000> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadInternal<rend::MeshEngineGPU::P
    Replacement::draft(0x141338720, "rend_141338720", fn_141338720 as *const ()), // rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectCommandVS>::rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectComm
    Replacement::draft(0x141338860, "rend_141338860", fn_141338860 as *const ()), // rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectCommandVS>::rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectComm
    Replacement::draft_throws(0x141338990, "rend_141338990", fn_141338990 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::MeshEngineGPUIndirectCommandVS>::rend::SparseUploadInternal<rend::MeshEngineGPUIndirect
    Replacement::draft_throws(0x1413389d0, "rend_1413389d0", fn_1413389d0 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::MeshEngineGPUIndirectCommandVS>::rend::SparseUploadInternal<rend::MeshEngineGPUIndirect
    Replacement::draft(0x141338a10, "rend_141338a10", fn_141338a10 as *const ()), // sub_141338a10
    Replacement::draft(0x141338c60, "rend_141338c60", fn_141338c60 as *const ()), // sub_141338c60
    Replacement::draft(0x141338f00, "rend_141338f00", fn_141338f00 as *const ()), // sub_141338f00
    Replacement::draft(0x1413391a0, "rend_1413391a0", fn_1413391a0 as *const ()), // sub_1413391a0
    Replacement::draft(0x141339440, "rend_141339440", fn_141339440 as *const ()), // sub_141339440
    Replacement::draft_throws(0x1413396e0, "rend_1413396e0", fn_1413396e0 as *const (), Throws::Discard), // sub_1413396e0
    Replacement::draft(0x141339750, "rend_141339750", tpl_23865::<0x1433fcd40, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::MeshEngineGPU::PerDrawRootConstants>::rend::SparseUploadInternal<rend::MeshEngineGPU::P
    Replacement::draft(0x141339790, "rend_141339790", tpl_23865::<0x1433fccd8, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::MeshEngineGPUIndirectCommandVS>::rend::SparseUploadInternal<rend::MeshEngineGPUIndirect
    Replacement::draft(0x1413397d0, "rend_1413397d0", fn_1413397d0 as *const ()), // sub_1413397d0
    Replacement::draft(0x1413397f0, "rend_1413397f0", fn_1413397f0 as *const ()), // sub_1413397f0
    Replacement::draft(0x141339800, "rend_141339800", fn_141339800 as *const ()), // sub_141339800
    Replacement::draft(0x1413398c0, "rend_1413398c0", fn_1413398c0 as *const ()), // sub_1413398c0
    Replacement::draft(0x1413399b0, "rend_1413399b0", fn_1413399b0 as *const ()), // sub_1413399b0
    Replacement::draft(0x141339a60, "rend_141339a60", fn_141339a60 as *const ()), // sub_141339a60
    Replacement::draft(0x141339b20, "rend_141339b20", fn_141339b20 as *const ()), // sub_141339b20
    Replacement::draft(0x141339c80, "rend_141339c80", fn_141339c80 as *const ()), // sub_141339c80
    Replacement::draft_throws(0x14133a0c0, "rend_14133a0c0", fn_14133a0c0 as *const (), Throws::Discard), // sub_14133a0c0
    Replacement::draft(0x14133a170, "rend_14133a170", fn_14133a170 as *const ()), // sub_14133a170
    Replacement::draft(0x14133a3b0, "rend_14133a3b0", fn_14133a3b0 as *const ()), // sub_14133a3b0
    Replacement::draft(0x14133a5b0, "rend_14133a5b0", fn_14133a5b0 as *const ()), // sub_14133a5b0
    Replacement::draft(0x14133a5f0, "rend_14133a5f0", fn_14133a5f0 as *const ()), // sub_14133a5f0
    Replacement::draft(0x14133a8d0, "rend_14133a8d0", fn_14133a8d0 as *const ()), // sub_14133a8d0
    Replacement::draft(0x14133aaa0, "rend_14133aaa0", tpl_24505::<0x8, 0x10, 0x14240ab10, 0x1401de830> as *const ()), // sub_14133aaa0
    Replacement::draft(0x14133aae0, "rend_14133aae0", tpl_24505::<0x8, 0x28, 0x1401de830, 0x14240ab10> as *const ()), // sub_14133aae0
    Replacement::draft(0x14133ab20, "rend_14133ab20", tpl_23991::<0xffffffffffffffff, 0x8, 0x4, 0x2> as *const ()), // sub_14133ab20
    Replacement::draft(0x14133abe0, "rend_14133abe0", tpl_23992 as *const ()), // sub_14133abe0
    Replacement::draft(0x14133ad50, "rend_14133ad50", tpl_24506::<0x14133aec0, 0x14133b0c0> as *const ()), // sub_14133ad50
    Replacement::draft(0x14133aec0, "rend_14133aec0", fn_14133aec0 as *const ()), // sub_14133aec0
    Replacement::draft(0x14133b0c0, "rend_14133b0c0", fn_14133b0c0 as *const ()), // sub_14133b0c0
    Replacement::draft(0x14133b240, "rend_14133b240", fn_14133b240 as *const ()), // sub_14133b240
    Replacement::draft(0x14133b2c0, "rend_14133b2c0", fn_14133b2c0 as *const ()), // sub_14133b2c0
    Replacement::draft(0x14133b360, "rend_14133b360", fn_14133b360 as *const ()), // sub_14133b360
    Replacement::draft(0x14133b610, "rend_14133b610", fn_14133b610 as *const ()), // sub_14133b610
    Replacement::draft(0x14133b900, "rend_14133b900", fn_14133b900 as *const ()), // sub_14133b900
    Replacement::draft(0x14133b9c0, "rend_14133b9c0", fn_14133b9c0 as *const ()), // sub_14133b9c0
    Replacement::draft(0x14133bbc0, "rend_14133bbc0", tpl_24514::<0x14340ed48, 0x8, 0x1433fcee0, 0x1433fce10, 0x142498f00> as *const ()), // sub_14133bbc0
    Replacement::draft(0x14133bc30, "rend_14133bc30", tpl_24514::<0x14340ed48, 0x8, 0x1433fcf28, 0x1433fce70, 0x142498f00> as *const ()), // sub_14133bc30
    Replacement::draft(0x14133bc80, "rend_14133bc80", fn_14133bc80 as *const ()), // sub_14133bc80
    Replacement::draft(0x14133bcf0, "rend_14133bcf0", fn_14133bcf0 as *const ()), // sub_14133bcf0
    Replacement::draft(0x14133bfe0, "rend_14133bfe0", fn_14133bfe0 as *const ()), // sub_14133bfe0
    Replacement::draft_throws(0x14133c0b0, "rend_14133c0b0", fn_14133c0b0 as *const (), Throws::Discard), // rend::Mesh::rend::Mesh::vfunc_0
    Replacement::draft(0x14133c0f0, "rend_14133c0f0", fn_14133c0f0 as *const ()), // sub_14133c0f0
    Replacement::draft(0x14133c150, "rend_14133c150", fn_14133c150 as *const ()), // sub_14133c150
    Replacement::draft(0x14133c180, "rend_14133c180", fn_14133c180 as *const ()), // sub_14133c180
    Replacement::draft(0x14133c2b0, "rend_14133c2b0", tpl_19541::<0x1401de830, 0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_14133c2b0
    Replacement::draft(0x14133c3d0, "rend_14133c3d0", fn_14133c3d0 as *const ()), // sub_14133c3d0
    Replacement::draft(0x14133c4d0, "rend_14133c4d0", fn_14133c4d0 as *const ()), // sub_14133c4d0
    Replacement::draft(0x14133c5f0, "rend_14133c5f0", fn_14133c5f0 as *const ()), // sub_14133c5f0
    Replacement::draft(0x14133c730, "rend_14133c730", fn_14133c730 as *const ()), // sub_14133c730
    Replacement::draft(0x14133c870, "rend_14133c870", tpl_66164::<0x8, 0xc, 0x14133dd80, 0x14183acc2, 0x14133d280> as *const ()), // sub_14133c870
    Replacement::draft(0x14133c930, "rend_14133c930", fn_14133c930 as *const ()), // sub_14133c930
    Replacement::draft(0x14133c9b0, "rend_14133c9b0", fn_14133c9b0 as *const ()), // sub_14133c9b0
    Replacement::draft_throws(0x14133cba0, "rend_14133cba0", fn_14133cba0 as *const (), Throws::Discard), // sub_14133cba0
    Replacement::draft_throws(0x14133cca0, "rend_14133cca0", fn_14133cca0 as *const (), Throws::Caught), // sub_14133cca0
    Replacement::draft_throws(0x14133cd10, "rend_14133cd10", fn_14133cd10 as *const (), Throws::Caught), // sub_14133cd10
    Replacement::draft(0x14133cd80, "rend_14133cd80", fn_14133cd80 as *const ()), // sub_14133cd80
    Replacement::draft(0x14133cee0, "rend_14133cee0", fn_14133cee0 as *const ()), // sub_14133cee0
    Replacement::draft(0x14133cf80, "rend_14133cf80", fn_14133cf80 as *const ()), // sub_14133cf80
    Replacement::draft(0x14133d040, "rend_14133d040", fn_14133d040 as *const ()), // sub_14133d040
    Replacement::draft(0x14133d110, "rend_14133d110", fn_14133d110 as *const ()), // sub_14133d110
    Replacement::draft(0x14133d1d0, "rend_14133d1d0", tpl_867::<0x8, 0x14133d280, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14133d1d0
    Replacement::draft(0x14133d280, "rend_14133d280", fn_14133d280 as *const ()), // sub_14133d280
    Replacement::draft(0x14133d2e0, "rend_14133d2e0", fn_14133d2e0 as *const ()), // sub_14133d2e0
    Replacement::draft(0x14133d460, "rend_14133d460", fn_14133d460 as *const ()), // sub_14133d460
    Replacement::draft(0x14133d5a0, "rend_14133d5a0", fn_14133d5a0 as *const ()), // sub_14133d5a0
    Replacement::draft(0x14133d6f0, "rend_14133d6f0", fn_14133d6f0 as *const ()), // sub_14133d6f0
    Replacement::draft(0x14133d760, "rend_14133d760", fn_14133d760 as *const ()), // sub_14133d760
    Replacement::draft(0x14133d910, "rend_14133d910", fn_14133d910 as *const ()), // sub_14133d910
    Replacement::draft(0x14133da90, "rend_14133da90", fn_14133da90 as *const ()), // sub_14133da90
    Replacement::draft(0x14133dbf0, "rend_14133dbf0", fn_14133dbf0 as *const ()), // sub_14133dbf0
    Replacement::draft(0x14133dd80, "rend_14133dd80", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14133d1d0> as *const ()), // sub_14133dd80
    Replacement::draft(0x14133def0, "rend_14133def0", fn_14133def0 as *const ()), // sub_14133def0
    Replacement::draft(0x14133e0a0, "rend_14133e0a0", tpl_21589::<0x14386c5d0, 0x14386c5b8, 0x14386c5e0, 0x14386c5c8, 0x14133e220, 0x14133e5b0> as *const ()), // sub_14133e0a0
    Replacement::draft(0x14133e220, "rend_14133e220", fn_14133e220 as *const ()), // sub_14133e220
    Replacement::draft(0x14133e5b0, "rend_14133e5b0", fn_14133e5b0 as *const ()), // sub_14133e5b0
    Replacement::draft(0x14133e8e0, "rend_14133e8e0", fn_14133e8e0 as *const ()), // sub_14133e8e0
    Replacement::draft(0x14133ea40, "rend_14133ea40", fn_14133ea40 as *const ()), // rend::MaterialResource::rend::MaterialResource::vfunc_0
    Replacement::draft(0x14133eb20, "rend_14133eb20", fn_14133eb20 as *const ()), // sub_14133eb20
    Replacement::draft(0x14133ec00, "rend_14133ec00", fn_14133ec00 as *const ()), // sub_14133ec00
    Replacement::draft(0x14133ec80, "rend_14133ec80", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433fd488, 0x8> as *const ()), // sub_14133ec80
    Replacement::draft(0x14133ed00, "rend_14133ed00", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433fd488, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_14133ed00
    Replacement::draft_throws(0x14133eda0, "rend_14133eda0", fn_14133eda0 as *const (), Throws::Caught), // sub_14133eda0
    Replacement::draft_throws(0x14133ee30, "rend_14133ee30", tpl_649::<0x143881c40, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_14133ee30
    Replacement::draft(0x14133eeb0, "rend_14133eeb0", fn_14133eeb0 as *const ()), // sub_14133eeb0
    Replacement::draft(0x14133ef70, "rend_14133ef70", fn_14133ef70 as *const ()), // sub_14133ef70
    Replacement::draft(0x14133f050, "rend_14133f050", tpl_6868::<0x143881c48, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14133f050
    Replacement::draft_throws(0x14133f0e0, "rend_14133f0e0", tpl_23922::<0x143881c50, 0x143881c48, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_14133f0e0
    Replacement::draft_throws(0x14133f160, "rend_14133f160", tpl_23922::<0x143881c50, 0x143881c48, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_14133f160
    Replacement::draft_throws(0x14133f1e0, "rend_14133f1e0", fn_14133f1e0 as *const (), Throws::Caught), // sub_14133f1e0
    Replacement::draft(0x14133f270, "rend_14133f270", tpl_23923::<0x143881c50, 0x143881c48, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_14133f270
    Replacement::draft_throws(0x14133f2f0, "rend_14133f2f0", tpl_23922::<0x143881c50, 0x143881c48, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_14133f2f0
    Replacement::draft_throws(0x14133f370, "rend_14133f370", tpl_23924::<0x143881c48, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_14133f370
    Replacement::draft(0x14133f420, "rend_14133f420", fn_14133f420 as *const ()), // sub_14133f420
    Replacement::draft(0x14133f480, "rend_14133f480", fn_14133f480 as *const ()), // sub_14133f480
    Replacement::draft(0x14133f610, "rend_14133f610", fn_14133f610 as *const ()), // sub_14133f610
    Replacement::draft(0x14133f780, "rend_14133f780", fn_14133f780 as *const ()), // sub_14133f780
    Replacement::draft(0x14133f920, "rend_14133f920", tpl_23929::<0x142407c80, 0x143c4aa00, 0x142338630, 0x142442de0, 0x1426455a8, 0x142645598, 0x14133fa90, 0x142645580> as *const ()), // sub_14133f920
    Replacement::draft(0x14133fa90, "rend_14133fa90", tpl_23930::<0x142338630, 0x142442de0> as *const ()), // sub_14133fa90
    Replacement::draft(0x14133fac0, "rend_14133fac0", fn_14133fac0 as *const ()), // sub_14133fac0
    Replacement::draft(0x14133fc70, "rend_14133fc70", fn_14133fc70 as *const ()), // sub_14133fc70
    Replacement::draft(0x141340000, "rend_141340000", fn_141340000 as *const ()), // sub_141340000
    Replacement::draft(0x141340130, "rend_141340130", fn_141340130 as *const ()), // sub_141340130
    Replacement::draft(0x141340420, "rend_141340420", fn_141340420 as *const ()), // sub_141340420
    Replacement::draft(0x141340520, "rend_141340520", fn_141340520 as *const ()), // sub_141340520
    Replacement::draft(0x1413405e0, "rend_1413405e0", fn_1413405e0 as *const ()), // sub_1413405e0
    Replacement::draft(0x141340740, "rend_141340740", fn_141340740 as *const ()), // sub_141340740
    Replacement::draft(0x141340830, "rend_141340830", fn_141340830 as *const ()), // sub_141340830
    Replacement::draft(0x1413408b0, "rend_1413408b0", fn_1413408b0 as *const ()), // rend::TextureResourceListener::rend::TextureResourceListener::vfunc_3
    Replacement::draft(0x141340950, "rend_141340950", fn_141340950 as *const ()), // rend::TextureResourceListener::rend::TextureResourceListener::vfunc_4
    Replacement::draft(0x141340a20, "rend_141340a20", fn_141340a20 as *const ()), // sub_141340a20
    Replacement::draft(0x141340a60, "rend_141340a60", tpl_24564::<0x1433fd938, 0x141340830, 0x10, 0x1412df190, 0x1433d6e58, 0x141834d30> as *const ()), // rend::TextureResourceListener::rend::TextureResourceListener::vfunc_0
    Replacement::draft(0x141340ac0, "rend_141340ac0", fn_141340ac0 as *const ()), // sub_141340ac0
    Replacement::draft(0x141340b20, "rend_141340b20", fn_141340b20 as *const ()), // rend::MaterialResourceListener::rend::MaterialResourceListener::vfunc_3
    Replacement::draft(0x141340b60, "rend_141340b60", tpl_24564::<0x1433fd968, 0x141340ac0, 0x10, 0x1410c05d0, 0x1433d6e58, 0x141834d30> as *const ()), // rend::MaterialResourceListener::rend::MaterialResourceListener::vfunc_0
    Replacement::draft(0x141340bc0, "rend_141340bc0", tpl_24567::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1433fd968> as *const ()), // sub_141340bc0
    Replacement::draft(0x141340c60, "rend_141340c60", tpl_24567::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1433fd938> as *const ()), // sub_141340c60
    Replacement::draft(0x141340d00, "rend_141340d00", fn_141340d00 as *const ()), // sub_141340d00
    Replacement::draft(0x141340df0, "rend_141340df0", fn_141340df0 as *const ()), // sub_141340df0
    Replacement::draft(0x141340f60, "rend_141340f60", fn_141340f60 as *const ()), // sub_141340f60
    Replacement::draft(0x141341070, "rend_141341070", tpl_20192::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433fdee0, 0x14248cd50> as *const ()), // sub_141341070
    Replacement::draft(0x141341110, "rend_141341110", fn_141341110 as *const ()), // sub_141341110
    Replacement::draft(0x141341120, "rend_141341120", fn_141341120 as *const ()), // sub_141341120
    Replacement::draft_throws(0x141341240, "rend_141341240", fn_141341240 as *const (), Throws::Discard), // sub_141341240
    Replacement::draft(0x141341380, "rend_141341380", fn_141341380 as *const ()), // sub_141341380
    Replacement::draft(0x141341430, "rend_141341430", tpl_5539::<0x8, 0x141341500, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141341430
    Replacement::draft(0x141341500, "rend_141341500", fn_141341500 as *const ()), // sub_141341500
    Replacement::draft_throws(0x1413415d0, "rend_1413415d0", fn_1413415d0 as *const (), Throws::Caught), // sub_1413415d0
    Replacement::draft(0x141341640, "rend_141341640", fn_141341640 as *const ()), // sub_141341640
    Replacement::draft(0x141341910, "rend_141341910", fn_141341910 as *const ()), // sub_141341910
    Replacement::draft(0x1413419f0, "rend_1413419f0", tpl_17984::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141341430> as *const ()), // sub_1413419f0
    Replacement::draft(0x141341ba0, "rend_141341ba0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433fea30, 0x8> as *const ()), // sub_141341ba0
    Replacement::draft(0x141341c20, "rend_141341c20", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433fea30, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_141341c20
    Replacement::draft_throws(0x141341cc0, "rend_141341cc0", fn_141341cc0 as *const (), Throws::Caught), // sub_141341cc0
    Replacement::draft_throws(0x141341d50, "rend_141341d50", tpl_1908::<0x143881ca0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_141341d50
    Replacement::draft(0x141341dd0, "rend_141341dd0", fn_141341dd0 as *const ()), // sub_141341dd0
    Replacement::draft(0x141341e90, "rend_141341e90", fn_141341e90 as *const ()), // rend::IlluminationVolumeTreeResource::rend::IlluminationVolumeTreeResource::vfunc_0
    Replacement::draft(0x141341ef0, "unknown_141341ef0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433ff0c8, 0x8> as *const ()), // sub_141341ef0
    Replacement::draft(0x141341f70, "unknown_141341f70", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433ff0c8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_141341f70
    Replacement::draft_throws(0x141342010, "unknown_141342010", fn_141342010 as *const (), Throws::Caught), // sub_141342010
    Replacement::draft_throws(0x1413420a0, "unknown_1413420a0", tpl_649::<0x143881cb0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413420a0
    Replacement::draft(0x141342120, "unknown_141342120", fn_141342120 as *const ()), // sub_141342120
    Replacement::draft(0x1413421a0, "unknown_1413421a0", fn_1413421a0 as *const ()), // sub_1413421a0
    Replacement::draft(0x141342240, "unknown_141342240", tpl_6868::<0x143881cb8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_141342240
    Replacement::draft_throws(0x1413422d0, "unknown_1413422d0", tpl_23922::<0x143881ca8, 0x143881cb8, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1413422d0
    Replacement::draft_throws(0x141342350, "unknown_141342350", tpl_23922::<0x143881ca8, 0x143881cb8, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_141342350
    Replacement::draft_throws(0x1413423d0, "unknown_1413423d0", fn_1413423d0 as *const (), Throws::Caught), // sub_1413423d0
    Replacement::draft(0x141342460, "unknown_141342460", tpl_23923::<0x143881ca8, 0x143881cb8, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_141342460
    Replacement::draft_throws(0x1413424e0, "unknown_1413424e0", tpl_23922::<0x143881ca8, 0x143881cb8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1413424e0
    Replacement::draft_throws(0x141342560, "unknown_141342560", tpl_23924::<0x143881cb8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_141342560
    Replacement::draft_throws(0x141342610, "unknown_141342610", fn_141342610 as *const (), Throws::Discard), // sub_141342610
    Replacement::draft_throws(0x1413427e0, "unknown_1413427e0", tpl_546::<0x142407c80, 0x143c4aa00, 0x1423428c0, 0x1426455a8, 0x142645598, 0x1423427c0, 0x142645580> as *const (), Throws::Discard), // sub_1413427e0
    Replacement::draft(0x141342940, "unknown_141342940", tpl_65251::<0x8, 0xc, 0x140a5b830, 0x14183acc2> as *const ()), // sub_141342940
    Replacement::draft_throws(0x1413429f0, "unknown_1413429f0", fn_1413429f0 as *const (), Throws::Caught), // sub_1413429f0
    Replacement::draft_throws(0x141342cb0, "unknown_141342cb0", fn_141342cb0 as *const (), Throws::Discard), // sub_141342cb0
    Replacement::draft(0x141342db0, "unknown_141342db0", fn_141342db0 as *const ()), // sub_141342db0
    Replacement::draft(0x141342e50, "unknown_141342e50", fn_141342e50 as *const ()), // sub_141342e50
    Replacement::draft(0x141342ef0, "unknown_141342ef0", fn_141342ef0 as *const ()), // sub_141342ef0
    Replacement::draft(0x141342f90, "unknown_141342f90", fn_141342f90 as *const ()), // sub_141342f90
    Replacement::draft(0x1413430c0, "unknown_1413430c0", fn_1413430c0 as *const ()), // sub_1413430c0
    Replacement::draft(0x1413432a0, "unknown_1413432a0", fn_1413432a0 as *const ()), // sub_1413432a0
    Replacement::draft_throws(0x141343440, "unknown_141343440", fn_141343440 as *const (), Throws::Discard), // sub_141343440
    Replacement::draft(0x141343600, "unknown_141343600", fn_141343600 as *const ()), // sub_141343600
    Replacement::draft_throws(0x1413436f0, "unknown_1413436f0", fn_1413436f0 as *const (), Throws::Caught), // sub_1413436f0
    Replacement::draft_throws(0x141343980, "unknown_141343980", fn_141343980 as *const (), Throws::Caught), // sub_141343980
    Replacement::draft_throws(0x141343be0, "unknown_141343be0", fn_141343be0 as *const (), Throws::Caught), // sub_141343be0
    Replacement::draft(0x141343e40, "unknown_141343e40", fn_141343e40 as *const ()), // sub_141343e40
    Replacement::draft(0x141343fc0, "unknown_141343fc0", fn_141343fc0 as *const ()), // sub_141343fc0
    Replacement::draft(0x141344150, "unknown_141344150", fn_141344150 as *const ()), // sub_141344150
    Replacement::draft_throws(0x1413442e0, "unknown_1413442e0", fn_1413442e0 as *const (), Throws::Discard), // sub_1413442e0
    Replacement::draft_throws(0x141344470, "unknown_141344470", fn_141344470 as *const (), Throws::Discard), // sub_141344470
    Replacement::draft(0x1413445c0, "unknown_1413445c0", fn_1413445c0 as *const ()), // sub_1413445c0
    Replacement::draft(0x1413446f0, "unknown_1413446f0", tpl_24597::<0x143881ce0, 0x14243b0b0, 0x338, 0x40, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1413446f0
    Replacement::draft_throws(0x141344790, "unknown_141344790", fn_141344790 as *const (), Throws::Discard), // sub_141344790
    Replacement::draft_throws(0x1413448b0, "unknown_1413448b0", fn_1413448b0 as *const (), Throws::Caught), // sub_1413448b0
    Replacement::draft_throws(0x141344a10, "unknown_141344a10", fn_141344a10 as *const (), Throws::Caught), // sub_141344a10
    Replacement::draft_throws(0x141344b70, "unknown_141344b70", fn_141344b70 as *const (), Throws::Caught), // sub_141344b70
    Replacement::draft(0x141344cd0, "unknown_141344cd0", fn_141344cd0 as *const ()), // sub_141344cd0
    Replacement::draft(0x141344d70, "unknown_141344d70", tpl_24597::<0x143881ce0, 0x14243b0b0, 0x338, 0xe0, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141344d70
    Replacement::draft_throws(0x141344e20, "unknown_141344e20", fn_141344e20 as *const (), Throws::Discard), // sub_141344e20
    Replacement::draft_throws(0x141344ee0, "unknown_141344ee0", fn_141344ee0 as *const (), Throws::Discard), // sub_141344ee0
    Replacement::draft(0x141345030, "unknown_141345030", fn_141345030 as *const ()), // sub_141345030
    Replacement::draft_throws(0x141345100, "unknown_141345100", tpl_71031::<0x143881cc4, 0x143881ce8, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x4, 0x8, 0x369, 0xc, 0x1401e9e70> as *const (), Throws::Discard), // sub_141345100
    Replacement::draft(0x1413451d0, "unknown_1413451d0", tpl_12556::<0x143881ce8, 0x14243b0b0, 0x338, 0x48, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1413451d0
    Replacement::draft_throws(0x141345270, "unknown_141345270", fn_141345270 as *const (), Throws::Discard), // sub_141345270
    Replacement::draft_throws(0x141345340, "unknown_141345340", fn_141345340 as *const (), Throws::Caught), // sub_141345340
    Replacement::draft_throws(0x141345560, "unknown_141345560", fn_141345560 as *const (), Throws::Caught), // sub_141345560
    Replacement::draft_throws(0x141345750, "unknown_141345750", fn_141345750 as *const (), Throws::Caught), // sub_141345750
    Replacement::draft(0x141345940, "unknown_141345940", tpl_18531::<0x143881ce8, 0x14243b0b0, 0x338, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141345940
    Replacement::draft(0x1413459d0, "unknown_1413459d0", tpl_12562::<0x143881cc4, 0x143881ce8, 0x1401e9970, 0x338, 0xe8, 0x4, 0x8, 0xc, 0x1401e9860> as *const ()), // sub_1413459d0
    Replacement::draft(0x141345a70, "unknown_141345a70", tpl_12556::<0x143881ce8, 0x14243b0b0, 0x338, 0xe8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141345a70
    Replacement::draft_throws(0x141345b20, "unknown_141345b20", fn_141345b20 as *const (), Throws::Discard), // sub_141345b20
    Replacement::draft_throws(0x141345bb0, "unknown_141345bb0", fn_141345bb0 as *const (), Throws::Discard), // sub_141345bb0
    Replacement::draft(0x141345c90, "unknown_141345c90", fn_141345c90 as *const ()), // sub_141345c90
    Replacement::draft(0x141345d20, "unknown_141345d20", fn_141345d20 as *const ()), // sub_141345d20
    Replacement::draft(0x141345dc0, "unknown_141345dc0", fn_141345dc0 as *const ()), // sub_141345dc0
    Replacement::draft_throws(0x141345e80, "unknown_141345e80", fn_141345e80 as *const (), Throws::Discard), // sub_141345e80
    Replacement::draft(0x141345f60, "unknown_141345f60", fn_141345f60 as *const ()), // sub_141345f60
    Replacement::draft_throws(0x141346020, "unknown_141346020", fn_141346020 as *const (), Throws::Discard), // sub_141346020
    Replacement::draft_throws(0x1413460c0, "unknown_1413460c0", fn_1413460c0 as *const (), Throws::Caught), // sub_1413460c0
    Replacement::draft_throws(0x1413461a0, "unknown_1413461a0", fn_1413461a0 as *const (), Throws::Caught), // sub_1413461a0
    Replacement::draft_throws(0x141346280, "unknown_141346280", fn_141346280 as *const (), Throws::Caught), // sub_141346280
    Replacement::draft(0x141346360, "unknown_141346360", fn_141346360 as *const ()), // sub_141346360
    Replacement::draft(0x141346420, "unknown_141346420", fn_141346420 as *const ()), // sub_141346420
    Replacement::draft(0x1413464c0, "unknown_1413464c0", fn_1413464c0 as *const ()), // sub_1413464c0
    Replacement::draft_throws(0x141346570, "unknown_141346570", fn_141346570 as *const (), Throws::Discard), // sub_141346570
    Replacement::draft_throws(0x141346640, "unknown_141346640", fn_141346640 as *const (), Throws::Discard), // sub_141346640
    Replacement::draft_throws(0x141346750, "unknown_141346750", tpl_9695::<0x20, 0x8, 0x141347770> as *const (), Throws::Discard), // sub_141346750
    Replacement::draft_throws(0x1413467a0, "unknown_1413467a0", fn_1413467a0 as *const (), Throws::Caught), // sub_1413467a0
    Replacement::draft_throws(0x141346a60, "unknown_141346a60", fn_141346a60 as *const (), Throws::Caught), // sub_141346a60
    Replacement::draft_throws(0x141346ee0, "unknown_141346ee0", fn_141346ee0 as *const (), Throws::Caught), // sub_141346ee0
    Replacement::draft_throws(0x141347330, "unknown_141347330", fn_141347330 as *const (), Throws::Caught), // sub_141347330
    Replacement::draft_throws(0x141347770, "unknown_141347770", fn_141347770 as *const (), Throws::Caught), // sub_141347770
    Replacement::draft_throws(0x141347840, "unknown_141347840", fn_141347840 as *const (), Throws::Caught), // sub_141347840
    Replacement::draft_throws(0x141347d10, "unknown_141347d10", fn_141347d10 as *const (), Throws::Caught), // sub_141347d10
    Replacement::draft_throws(0x1413481a0, "unknown_1413481a0", fn_1413481a0 as *const (), Throws::Caught), // sub_1413481a0
    Replacement::draft_throws(0x141348630, "unknown_141348630", fn_141348630 as *const (), Throws::Caught), // sub_141348630
    Replacement::draft_throws(0x141348890, "unknown_141348890", fn_141348890 as *const (), Throws::Caught), // sub_141348890
    Replacement::draft_throws(0x141348ae0, "unknown_141348ae0", fn_141348ae0 as *const (), Throws::Caught), // sub_141348ae0
    Replacement::draft_throws(0x141348d30, "unknown_141348d30", fn_141348d30 as *const (), Throws::Caught), // sub_141348d30
    Replacement::draft_throws(0x141348f20, "unknown_141348f20", fn_141348f20 as *const (), Throws::Caught), // sub_141348f20
    Replacement::draft_throws(0x141349490, "unknown_141349490", fn_141349490 as *const (), Throws::Caught), // sub_141349490
    Replacement::draft_throws(0x1413499c0, "unknown_1413499c0", fn_1413499c0 as *const (), Throws::Caught), // sub_1413499c0
    Replacement::draft(0x141349d40, "unknown_141349d40", fn_141349d40 as *const ()), // sub_141349d40
    Replacement::draft(0x141349dc0, "unknown_141349dc0", fn_141349dc0 as *const ()), // sub_141349dc0
    Replacement::draft_throws(0x141349ee0, "illuminationvolumemanager_141349ee0", fn_141349ee0 as *const (), Throws::Caught), // IlluminationVolumeManager::IlluminationVolumeManager::m_pGIBrickTextures
    Replacement::draft(0x141349fa0, "unknown_141349fa0", fn_141349fa0 as *const ()), // sub_141349fa0
    Replacement::draft(0x141349fe0, "unknown_141349fe0", tpl_6537::<0x14134a120, 0x14183acc8, 0x1401dec00, 0x1401de830> as *const ()), // sub_141349fe0
    Replacement::draft(0x14134a120, "unknown_14134a120", fn_14134a120 as *const ()), // sub_14134a120
    Replacement::draft_throws(0x14134a180, "unknown_14134a180", fn_14134a180 as *const (), Throws::Discard), // sub_14134a180
    Replacement::draft_throws(0x14134a2f0, "unknown_14134a2f0", fn_14134a2f0 as *const (), Throws::Discard), // sub_14134a2f0
    Replacement::draft(0x14134a460, "unknown_14134a460", fn_14134a460 as *const ()), // sub_14134a460
    Replacement::draft(0x14134a6a0, "rend_14134a6a0", tpl_24634::<0x14234b590, 0x141834d30> as *const ()), // rend::IlluminationVolumeGIDataLoadingSlot::CachedMemoryResource::rend::IlluminationVolumeGIDataLoadingSlot::CachedMemory
    Replacement::draft(0x14134a6e0, "rend_14134a6e0", fn_14134a6e0 as *const ()), // rend::IlluminationVolumeGIDataStreamer::rend::IlluminationVolumeGIDataStreamer::vfunc_0
    Replacement::draft(0x14134a780, "rend_14134a780", fn_14134a780 as *const ()), // sub_14134a780
    Replacement::draft(0x14134a7f0, "rend_14134a7f0", fn_14134a7f0 as *const ()), // sub_14134a7f0
    Replacement::draft(0x14134a8f0, "rend_14134a8f0", fn_14134a8f0 as *const ()), // sub_14134a8f0
    Replacement::draft_throws(0x14134a9d0, "rend_14134a9d0", fn_14134a9d0 as *const (), Throws::Discard), // sub_14134a9d0
    Replacement::draft(0x14134aa80, "rend_14134aa80", tpl_24640::<0x14134a8f0, 0x14134abf0, 0x14134aa80, 0x14134b020> as *const ()), // sub_14134aa80
    Replacement::draft(0x14134abf0, "rend_14134abf0", fn_14134abf0 as *const ()), // sub_14134abf0
    Replacement::draft(0x14134b020, "rend_14134b020", fn_14134b020 as *const ()), // sub_14134b020
    Replacement::draft(0x14134b1b0, "rend_14134b1b0", fn_14134b1b0 as *const ()), // sub_14134b1b0
    Replacement::draft(0x14134b410, "rend_14134b410", fn_14134b410 as *const ()), // sub_14134b410
    Replacement::draft(0x14134b640, "rend_14134b640", tpl_24506::<0x14134b7b0, 0x14134b020> as *const ()), // sub_14134b640
    Replacement::draft(0x14134b7b0, "rend_14134b7b0", fn_14134b7b0 as *const ()), // sub_14134b7b0
    Replacement::draft(0x14134b9b0, "rend_14134b9b0", fn_14134b9b0 as *const ()), // sub_14134b9b0
    Replacement::draft(0x14134ba40, "rend_14134ba40", fn_14134ba40 as *const ()), // rend::IlluminationVolumeGIDataResource::rend::IlluminationVolumeGIDataResource::vfunc_0
    Replacement::draft(0x14134ba90, "rend_14134ba90", fn_14134ba90 as *const ()), // sub_14134ba90
    Replacement::draft(0x14134bd30, "rend_14134bd30", fn_14134bd30 as *const ()), // sub_14134bd30
    Replacement::draft(0x14134be50, "rend_14134be50", fn_14134be50 as *const ()), // sub_14134be50
    Replacement::draft(0x14134bef0, "rend_14134bef0", fn_14134bef0 as *const ()), // sub_14134bef0
    Replacement::draft(0x14134bfa0, "rend_14134bfa0", fn_14134bfa0 as *const ()), // sub_14134bfa0
    Replacement::draft_throws(0x14134c040, "rend_14134c040", fn_14134c040 as *const (), Throws::Discard), // sub_14134c040
    Replacement::draft(0x14134c0d0, "rend_14134c0d0", fn_14134c0d0 as *const ()), // sub_14134c0d0
    Replacement::draft_throws(0x14134c150, "rend_14134c150", fn_14134c150 as *const (), Throws::Discard), // sub_14134c150
    Replacement::draft_throws(0x14134c1f0, "rend_14134c1f0", fn_14134c1f0 as *const (), Throws::Caught), // sub_14134c1f0
    Replacement::draft_throws(0x14134c2d0, "rend_14134c2d0", fn_14134c2d0 as *const (), Throws::Caught), // sub_14134c2d0
    Replacement::draft_throws(0x14134c3b0, "rend_14134c3b0", fn_14134c3b0 as *const (), Throws::Caught), // sub_14134c3b0
    Replacement::draft(0x14134c490, "rend_14134c490", fn_14134c490 as *const ()), // sub_14134c490
    Replacement::draft(0x14134c500, "rend_14134c500", fn_14134c500 as *const ()), // sub_14134c500
    Replacement::draft(0x14134c570, "rend_14134c570", fn_14134c570 as *const ()), // sub_14134c570
    Replacement::draft_throws(0x14134c5e0, "rend_14134c5e0", fn_14134c5e0 as *const (), Throws::Discard), // sub_14134c5e0
    Replacement::draft_throws(0x14134c650, "rend_14134c650", tpl_16042::<0x143881d00, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x10, 0x1433ffc10, 0x1, 0x14020f120> as *const (), Throws::Discard), // sub_14134c650
    Replacement::draft(0x14134c710, "rend_14134c710", fn_14134c710 as *const ()), // sub_14134c710
    Replacement::draft(0x14134c7a0, "rend_14134c7a0", fn_14134c7a0 as *const ()), // sub_14134c7a0
    Replacement::draft(0x14134c860, "rend_14134c860", tpl_6868::<0x143881d10, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14134c860
    Replacement::draft_throws(0x14134c8f0, "rend_14134c8f0", tpl_23922::<0x143881d08, 0x143881d10, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_14134c8f0
    Replacement::draft_throws(0x14134c970, "rend_14134c970", tpl_23922::<0x143881d08, 0x143881d10, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_14134c970
    Replacement::draft_throws(0x14134c9f0, "rend_14134c9f0", fn_14134c9f0 as *const (), Throws::Caught), // sub_14134c9f0
    Replacement::draft(0x14134ca80, "rend_14134ca80", tpl_23923::<0x143881d08, 0x143881d10, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_14134ca80
    Replacement::draft_throws(0x14134cb00, "rend_14134cb00", tpl_23922::<0x143881d08, 0x143881d10, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_14134cb00
    Replacement::draft_throws(0x14134cb80, "rend_14134cb80", tpl_23924::<0x143881d10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_14134cb80
    Replacement::draft_throws(0x14134cc60, "rend_14134cc60", fn_14134cc60 as *const (), Throws::Discard), // sub_14134cc60
    Replacement::draft(0x14134cdd0, "rend_14134cdd0", fn_14134cdd0 as *const ()), // sub_14134cdd0
    Replacement::draft(0x14134cf70, "rend_14134cf70", fn_14134cf70 as *const ()), // sub_14134cf70
    Replacement::draft(0x14134cfc0, "rend_14134cfc0", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x143400290, 0x1412eb320> as *const ()), // sub_14134cfc0
    Replacement::draft(0x14134d010, "rend_14134d010", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1434001e0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::heightfield::Vertex>::rend::SparseUploadBuffer<rend::heightfield::Vertex>::vfunc_0
    Replacement::draft(0x14134d080, "rend_14134d080", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x143400290, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::heightfield::TriangleVertices>::rend::SparseUploadBuffer<rend::heightfield::TriangleVerti
    Replacement::draft(0x14134d390, "rend_14134d390", fn_14134d390 as *const ()), // rend::SparseUploadBuffer<rend::heightfield::TriangleVertices>::rend::SparseUploadBuffer<rend::heightfield::TriangleVerti
    Replacement::draft(0x14134d4b0, "rend_14134d4b0", fn_14134d4b0 as *const ()), // rend::SparseUploadBuffer<rend::heightfield::TriangleVertices>::rend::SparseUploadBuffer<rend::heightfield::TriangleVerti
    Replacement::draft(0x14134d550, "rend_14134d550", fn_14134d550 as *const ()), // rend::SparseUploadBuffer<rend::heightfield::TriangleVertices>::rend::SparseUploadBuffer<rend::heightfield::TriangleVerti
    Replacement::draft_throws(0x14134d5f0, "rend_14134d5f0", tpl_24665::<0x20, 0x29, 0x10, 0x8, 0x142350fd0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::heightfield::TriangleVertices>::rend::SparseUploadInternal<rend::heightfield::TriangleV
    Replacement::draft_throws(0x14134d630, "rend_14134d630", tpl_24666::<0x29, 0x20, 0x10, 0x8, 0x142350fd0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::heightfield::TriangleVertices>::rend::SparseUploadInternal<rend::heightfield::TriangleV
    Replacement::draft(0x14134d670, "rend_14134d670", fn_14134d670 as *const ()), // sub_14134d670
    Replacement::draft_throws(0x14134d760, "rend_14134d760", fn_14134d760 as *const (), Throws::Discard), // rend::SparseUploadBuffer<rend::heightfield::Vertex>::rend::SparseUploadBuffer<rend::heightfield::Vertex>::vfunc_1
    Replacement::draft(0x14134d9e0, "rend_14134d9e0", fn_14134d9e0 as *const ()), // rend::SparseUploadBuffer<rend::heightfield::Vertex>::rend::SparseUploadBuffer<rend::heightfield::Vertex>::vfunc_4
    Replacement::draft(0x14134db00, "rend_14134db00", tpl_24668::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x14134df00, 0xa0, 0xa4, 0x1408746b0, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::heightfield::Vertex>::rend::SparseUploadBuffer<rend::heightfield::Vertex>::vfunc_5
    Replacement::draft(0x14134dba0, "rend_14134dba0", tpl_24669::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x14134df00, 0xa0, 0xa4, 0x1408746b0, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::heightfield::Vertex>::rend::SparseUploadBuffer<rend::heightfield::Vertex>::vfunc_6
    Replacement::draft_throws(0x14134dc40, "rend_14134dc40", tpl_24234::<0x20, 0x29, 0x10, 0x8, 0x142350fd0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::heightfield::Vertex>::rend::SparseUploadInternal<rend::heightfield::Vertex>::vfunc_1
    Replacement::draft_throws(0x14134dc80, "rend_14134dc80", tpl_24235::<0x29, 0x20, 0x10, 0x8, 0x142350fd0> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::heightfield::Vertex>::rend::SparseUploadInternal<rend::heightfield::Vertex>::vfunc_2
    Replacement::draft(0x14134dcc0, "rend_14134dcc0", fn_14134dcc0 as *const ()), // sub_14134dcc0
    Replacement::draft(0x14134dd90, "rend_14134dd90", tpl_23865::<0x143400290, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::heightfield::TriangleVertices>::rend::SparseUploadInternal<rend::heightfield::TriangleV
    Replacement::draft(0x14134ddd0, "rend_14134ddd0", tpl_23865::<0x1434001e0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::heightfield::Vertex>::rend::SparseUploadInternal<rend::heightfield::Vertex>::vfunc_0
    Replacement::draft(0x14134de10, "rend_14134de10", fn_14134de10 as *const ()), // sub_14134de10
    Replacement::draft(0x14134df00, "rend_14134df00", fn_14134df00 as *const ()), // sub_14134df00
    Replacement::draft(0x14134e150, "rend_14134e150", fn_14134e150 as *const ()), // rend::HeightfieldResource::rend::HeightfieldResource::vfunc_0
    Replacement::draft(0x14134e250, "rend_14134e250", fn_14134e250 as *const ()), // sub_14134e250
    Replacement::draft(0x14134e370, "rend_14134e370", tpl_4771::<0x48, 0x140a02a70, 0x142442de0> as *const ()), // sub_14134e370
    Replacement::draft(0x14134e3a0, "rend_14134e3a0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143400328, 0x8> as *const ()), // sub_14134e3a0
    Replacement::draft(0x14134e420, "rend_14134e420", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143400328, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_14134e420
    Replacement::draft_throws(0x14134e4c0, "rend_14134e4c0", fn_14134e4c0 as *const (), Throws::Caught), // sub_14134e4c0
    Replacement::draft_throws(0x14134e550, "rend_14134e550", tpl_649::<0x143881d18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_14134e550
    Replacement::draft(0x14134e5d0, "rend_14134e5d0", fn_14134e5d0 as *const ()), // sub_14134e5d0
    Replacement::draft(0x14134e6b0, "rend_14134e6b0", fn_14134e6b0 as *const ()), // sub_14134e6b0
    Replacement::draft(0x14134e7b0, "rend_14134e7b0", tpl_6868::<0x143881d20, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_14134e7b0
    Replacement::draft_throws(0x14134e840, "rend_14134e840", tpl_23922::<0x143881d28, 0x143881d20, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_14134e840
    Replacement::draft_throws(0x14134e8c0, "rend_14134e8c0", tpl_23922::<0x143881d28, 0x143881d20, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_14134e8c0
    Replacement::draft_throws(0x14134e940, "rend_14134e940", fn_14134e940 as *const (), Throws::Caught), // sub_14134e940
    Replacement::draft(0x14134e9d0, "rend_14134e9d0", tpl_23923::<0x143881d28, 0x143881d20, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_14134e9d0
    Replacement::draft_throws(0x14134ea50, "rend_14134ea50", tpl_23922::<0x143881d28, 0x143881d20, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_14134ea50
    Replacement::draft_throws(0x14134ead0, "rend_14134ead0", tpl_23924::<0x143881d20, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_14134ead0
    Replacement::draft(0x14134eb80, "rend_14134eb80", fn_14134eb80 as *const ()), // sub_14134eb80
    Replacement::draft(0x14134ec40, "rend_14134ec40", fn_14134ec40 as *const ()), // sub_14134ec40
    Replacement::draft(0x14134ec80, "rend_14134ec80", fn_14134ec80 as *const ()), // sub_14134ec80
    Replacement::draft(0x14134f150, "rend_14134f150", fn_14134f150 as *const ()), // sub_14134f150
    Replacement::draft_throws(0x14134f1c0, "rend_14134f1c0", fn_14134f1c0 as *const (), Throws::Discard), // sub_14134f1c0
    Replacement::draft(0x14134f240, "rend_14134f240", fn_14134f240 as *const ()), // sub_14134f240
    Replacement::draft_throws(0x14134f2e0, "rend_14134f2e0", fn_14134f2e0 as *const (), Throws::Discard), // sub_14134f2e0
    Replacement::draft_throws(0x14134f380, "rend_14134f380", fn_14134f380 as *const (), Throws::Discard), // sub_14134f380
    Replacement::draft(0x14134f480, "rend_14134f480", tpl_24148::<0x28, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0, 0x20, 0x18> as *const ()), // sub_14134f480
    Replacement::draft(0x14134f520, "rend_14134f520", fn_14134f520 as *const ()), // sub_14134f520
    Replacement::draft(0x14134f650, "rend_14134f650", fn_14134f650 as *const ()), // sub_14134f650
    Replacement::draft_throws(0x14134f850, "rend_14134f850", fn_14134f850 as *const (), Throws::Discard), // sub_14134f850
    Replacement::draft(0x14134f990, "rend_14134f990", fn_14134f990 as *const ()), // sub_14134f990
    Replacement::draft(0x14134fc10, "rend_14134fc10", fn_14134fc10 as *const ()), // sub_14134fc10
    Replacement::draft(0x14134fc70, "rend_14134fc70", fn_14134fc70 as *const ()), // rend::GfxGraphRenderHeightfieldDepthCommand::rend::GfxGraphRenderHeightfieldDepthCommand::vfunc_3
    Replacement::draft(0x14134fcc0, "rend_14134fcc0", fn_14134fcc0 as *const ()), // rend::GfxGraphDrawCommand::rend::GfxGraphDrawCommand::vfunc_0
    Replacement::draft(0x14134fd20, "rend_14134fd20", fn_14134fd20 as *const ()), // rend::GfxGraphDispatchComputeShaderCommand::rend::GfxGraphDispatchComputeShaderCommand::vfunc_3
    Replacement::draft(0x14134fd90, "rend_14134fd90", fn_14134fd90 as *const ()), // rend::GfxGraphExecuteCullingCommand::rend::GfxGraphExecuteCullingCommand::vfunc_3
    Replacement::draft(0x14134fde0, "rend_14134fde0", tpl_20552::<0x18, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14134fde0
    Replacement::draft(0x14134fe30, "rend_14134fe30", fn_14134fe30 as *const ()), // rend::GfxGraphDrawQuadsCommand::rend::GfxGraphDrawQuadsCommand::vfunc_0
    Replacement::draft(0x14134fe90, "rend_14134fe90", fn_14134fe90 as *const ()), // rend::GfxGraphIndirectDrawQuadsCommand::rend::GfxGraphIndirectDrawQuadsCommand::vfunc_0
    Replacement::draft(0x14134fef0, "rend_14134fef0", fn_14134fef0 as *const ()), // rend::GfxGraphDrawMeshCommand::rend::GfxGraphDrawMeshCommand::vfunc_0
    Replacement::draft(0x14134ff90, "rend_14134ff90", fn_14134ff90 as *const ()), // rend::GfxGraphDrawVisibleSetCommand::rend::GfxGraphDrawVisibleSetCommand::vfunc_0
    Replacement::draft(0x141350010, "rend_141350010", fn_141350010 as *const ()), // rend::GfxGraphDrawObjectsCommand::rend::GfxGraphDrawObjectsCommand::vfunc_0
    Replacement::draft(0x141350070, "rend_141350070", fn_141350070 as *const ()), // rend::GfxGraphDrawObjectPrimitiveCommand::rend::GfxGraphDrawObjectPrimitiveCommand::vfunc_0
    Replacement::draft(0x1413500d0, "rend_1413500d0", fn_1413500d0 as *const ()), // rend::GfxGraphDrawMeshPrimitiveCommand::rend::GfxGraphDrawMeshPrimitiveCommand::vfunc_0
    Replacement::draft(0x141350130, "rend_141350130", fn_141350130 as *const ()), // sub_141350130
    Replacement::draft(0x1413501c0, "rend_1413501c0", fn_1413501c0 as *const ()), // rend::GfxGraphDrawParticleSystemsCommand::rend::GfxGraphDrawParticleSystemsCommand::vfunc_0
    Replacement::draft(0x141350320, "rend_141350320", fn_141350320 as *const ()), // sub_141350320
    Replacement::draft(0x141350380, "rend_141350380", fn_141350380 as *const ()), // rend::GfxGraphCommandAllocator::rend::GfxGraphCommandAllocator::vfunc_0
    Replacement::draft_throws(0x141350470, "rend_141350470", fn_141350470 as *const (), Throws::Discard), // rend::GfxGraphEngine::rend::GfxGraphEngine::vfunc_0
    Replacement::draft_throws(0x1413504f0, "rend_1413504f0", tpl_24702::<0x58, 0x14397ff18, 0x142258080> as *const (), Throws::Discard), // sub_1413504f0
    Replacement::draft_throws(0x141350510, "rend_141350510", fn_141350510 as *const (), Throws::Discard), // sub_141350510
    Replacement::draft_throws(0x141350540, "rend_141350540", tpl_24702::<0x28, 0x14397ff18, 0x142257af0> as *const (), Throws::Discard), // sub_141350540
    Replacement::draft(0x141350560, "rend_141350560", fn_141350560 as *const ()), // sub_141350560
    Replacement::draft(0x1413505c0, "rend_1413505c0", fn_1413505c0 as *const ()), // sub_1413505c0
    Replacement::draft(0x141350690, "rend_141350690", fn_141350690 as *const ()), // sub_141350690
    Replacement::draft_throws(0x141350710, "rend_141350710", fn_141350710 as *const (), Throws::Discard), // sub_141350710
    Replacement::draft(0x141350790, "rend_141350790", fn_141350790 as *const ()), // sub_141350790
    Replacement::draft(0x141350890, "rend_141350890", fn_141350890 as *const ()), // sub_141350890
    Replacement::draft_throws(0x1413508d0, "rend_1413508d0", fn_1413508d0 as *const (), Throws::Caught), // sub_1413508d0
    Replacement::draft_throws(0x141350cb0, "rend_141350cb0", fn_141350cb0 as *const (), Throws::Caught), // sub_141350cb0
    Replacement::draft(0x141351100, "rend_141351100", fn_141351100 as *const ()), // sub_141351100
    Replacement::draft(0x141351170, "rend_141351170", fn_141351170 as *const ()), // sub_141351170
    Replacement::draft(0x141351270, "rend_141351270", fn_141351270 as *const ()), // sub_141351270
    Replacement::draft_throws(0x141351340, "rend_141351340", fn_141351340 as *const (), Throws::Discard), // sub_141351340
    Replacement::draft(0x141351410, "rend_141351410", fn_141351410 as *const ()), // sub_141351410
    Replacement::draft(0x141351500, "rend_141351500", tpl_19783::<0x8, 0x1413522d0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141351500
    Replacement::draft_throws(0x1413515d0, "rend_1413515d0", fn_1413515d0 as *const (), Throws::Discard), // sub_1413515d0
    Replacement::draft_throws(0x141351680, "rend_141351680", fn_141351680 as *const (), Throws::Caught), // sub_141351680
    Replacement::draft_throws(0x1413516a0, "rend_1413516a0", fn_1413516a0 as *const (), Throws::Caught), // sub_1413516a0
    Replacement::draft_throws(0x1413516c0, "rend_1413516c0", fn_1413516c0 as *const (), Throws::Caught), // sub_1413516c0
    Replacement::draft(0x141351b30, "rend_141351b30", fn_141351b30 as *const ()), // sub_141351b30
    Replacement::draft_throws(0x141351ce0, "rend_141351ce0", fn_141351ce0 as *const (), Throws::Discard), // sub_141351ce0
    Replacement::draft(0x141351f70, "rend_141351f70", fn_141351f70 as *const ()), // sub_141351f70
    Replacement::draft(0x1413520f0, "rend_1413520f0", fn_1413520f0 as *const ()), // sub_1413520f0
    Replacement::draft(0x141352260, "rend_141352260", fn_141352260 as *const ()), // sub_141352260
    Replacement::draft(0x1413522d0, "rend_1413522d0", fn_1413522d0 as *const ()), // sub_1413522d0
    Replacement::draft(0x141352340, "rend_141352340", fn_141352340 as *const ()), // sub_141352340
    Replacement::draft(0x141352440, "rend_141352440", fn_141352440 as *const ()), // sub_141352440
    Replacement::draft(0x1413524b0, "rend_1413524b0", tpl_24725::<0x8, 0xc, 0x1, 0x1408746b0> as *const ()), // sub_1413524b0
    Replacement::draft(0x141352520, "rend_141352520", fn_141352520 as *const ()), // sub_141352520
    Replacement::draft(0x1413525a0, "rend_1413525a0", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x143402238, 0x1412eb320> as *const ()), // sub_1413525a0
    Replacement::draft(0x1413525f0, "rend_1413525f0", tpl_23970::<0x1401de830, 0x143402060, 0x1412eb320> as *const ()), // sub_1413525f0
    Replacement::draft(0x141352620, "rend_141352620", tpl_24188::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1434021b0, 0x1412eb320> as *const ()), // sub_141352620
    Replacement::draft(0x141352680, "rend_141352680", tpl_24188::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1434021d0, 0x1412eb320> as *const ()), // sub_141352680
    Replacement::draft(0x1413526e0, "rend_1413526e0", tpl_24188::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1434020d0, 0x1412eb320> as *const ()), // sub_1413526e0
    Replacement::draft(0x141352760, "rend_141352760", tpl_24189::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1434020d0, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::BoundsUpdateInstance>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft(0x1413527e0, "rend_1413527e0", tpl_24189::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1434021d0, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::CompressedGeometryId>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft(0x141352860, "rend_141352860", tpl_24189::<0x1401de830, 0xc0, 0xa8, 0x98, 0x1434021b0, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::LeafBone>::rend::TrackedSparseUploadBuffer<rend::GeometryTransf
    Replacement::draft(0x1413528e0, "rend_1413528e0", tpl_23977::<0x1401de830, 0x143402060, 0x1412eb320, 0x141834d30> as *const ()), // rend::TrackedGPUOnlySparseUploadBuffer<rend::BoneTransform>::rend::TrackedGPUOnlySparseUploadBuffer<rend::BoneTransform>
    Replacement::draft(0x141352930, "rend_141352930", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x143402238, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadBuffer<rend::GeometryTransform::Me
    Replacement::draft_throws(0x1413529a0, "rend_1413529a0", fn_1413529a0 as *const (), Throws::Discard), // rend::SparseUploadBuffer<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadBuffer<rend::GeometryTransform::Me
    Replacement::draft(0x141352c80, "rend_141352c80", fn_141352c80 as *const ()), // rend::SparseUploadBuffer<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadBuffer<rend::GeometryTransform::Me
    Replacement::draft(0x141352da0, "rend_141352da0", tpl_24728::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x1413545d0, 0xa0, 0xa4, 0x140fb1b60, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadBuffer<rend::GeometryTransform::Me
    Replacement::draft(0x141352e30, "rend_141352e30", tpl_24729::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x1413545d0, 0xa0, 0xa4, 0x140fb1b60, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadBuffer<rend::GeometryTransform::Me
    Replacement::draft_throws(0x141352ed0, "rend_141352ed0", tpl_24730::<0x20, 0x29, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadInternal<rend::GeometryTransform
    Replacement::draft_throws(0x141352f10, "rend_141352f10", tpl_24731::<0x29, 0x20, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadInternal<rend::GeometryTransform
    Replacement::draft_throws(0x141352f50, "rend_141352f50", fn_141352f50 as *const (), Throws::Discard), // rend::TrackedGPUOnlySparseUploadBuffer<rend::BoneTransform>::rend::TrackedGPUOnlySparseUploadBuffer<rend::BoneTransform>
    Replacement::draft_throws(0x141352ff0, "rend_141352ff0", fn_141352ff0 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::BoneTransform>::rend::SparseUploadInternal<rend::BoneTransform>::vfunc_1
    Replacement::draft_throws(0x141353030, "rend_141353030", fn_141353030 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::BoneTransform>::rend::SparseUploadInternal<rend::BoneTransform>::vfunc_2
    Replacement::draft(0x141353070, "rend_141353070", fn_141353070 as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::LeafBone>::rend::TrackedSparseUploadBuffer<rend::GeometryTransf
    Replacement::draft(0x141353170, "rend_141353170", fn_141353170 as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::LeafBone>::rend::TrackedSparseUploadBuffer<rend::GeometryTransf
    Replacement::draft(0x141353220, "rend_141353220", fn_141353220 as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::LeafBone>::rend::TrackedSparseUploadBuffer<rend::GeometryTransf
    Replacement::draft_throws(0x141353700, "rend_141353700", tpl_24665::<0x20, 0x29, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::LeafBone>::rend::SparseUploadInternal<rend::GeometryTransform::LeafB
    Replacement::draft_throws(0x141353740, "rend_141353740", tpl_24666::<0x29, 0x20, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::LeafBone>::rend::SparseUploadInternal<rend::GeometryTransform::LeafB
    Replacement::draft(0x141353780, "rend_141353780", fn_141353780 as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::CompressedGeometryId>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft(0x141353880, "rend_141353880", fn_141353880 as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::CompressedGeometryId>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft(0x141353920, "rend_141353920", fn_141353920 as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::CompressedGeometryId>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft_throws(0x141353df0, "rend_141353df0", tpl_24234::<0x20, 0x29, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::CompressedGeometryId>::rend::SparseUploadInternal<rend::GeometryTran
    Replacement::draft_throws(0x141353e30, "rend_141353e30", tpl_24235::<0x29, 0x20, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::CompressedGeometryId>::rend::SparseUploadInternal<rend::GeometryTran
    Replacement::draft(0x141353e70, "rend_141353e70", tpl_24197::<0x58, 0xffffffffffffffff, 0x142229d10, 0x90, 0x98, 0x1413549a0, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::BoundsUpdateInstance>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft(0x141353f10, "rend_141353f10", tpl_24198::<0x58, 0xffffffffffffffff, 0x142229c00, 0x90, 0x98, 0x1413549a0, 0xa0, 0xa4, 0x140a5b830, 0xa8, 0x140fff1e0> as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::BoundsUpdateInstance>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft(0x141354000, "rend_141354000", tpl_24199::<0x142229a00, 0x1413549a0, 0x140a5b830, 0x140fff1e0, 0x1412f8d90> as *const ()), // rend::TrackedSparseUploadBuffer<rend::GeometryTransform::BoundsUpdateInstance>::rend::TrackedSparseUploadBuffer<rend::Ge
    Replacement::draft_throws(0x141354410, "rend_141354410", tpl_23986::<0x20, 0x29, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::BoundsUpdateInstance>::rend::SparseUploadInternal<rend::GeometryTran
    Replacement::draft_throws(0x141354450, "rend_141354450", tpl_23987::<0x29, 0x20, 0x10, 0x8, 0x14237d500> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryTransform::BoundsUpdateInstance>::rend::SparseUploadInternal<rend::GeometryTran
    Replacement::draft(0x141354490, "rend_141354490", tpl_23865::<0x143402238, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::GeometryTransform::MeshInstanceGPU>::rend::SparseUploadInternal<rend::GeometryTransform
    Replacement::draft(0x1413544d0, "rend_1413544d0", tpl_23865::<0x143402060, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::BoneTransform>::rend::SparseUploadInternal<rend::BoneTransform>::vfunc_0
    Replacement::draft(0x141354510, "rend_141354510", tpl_23865::<0x1434021b0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::GeometryTransform::LeafBone>::rend::SparseUploadInternal<rend::GeometryTransform::LeafB
    Replacement::draft(0x141354550, "rend_141354550", tpl_23865::<0x1434021d0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::GeometryTransform::CompressedGeometryId>::rend::SparseUploadInternal<rend::GeometryTran
    Replacement::draft(0x141354590, "rend_141354590", tpl_23865::<0x1434020d0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::GeometryTransform::BoundsUpdateInstance>::rend::SparseUploadInternal<rend::GeometryTran
    Replacement::draft(0x1413545d0, "rend_1413545d0", fn_1413545d0 as *const ()), // sub_1413545d0
    Replacement::draft(0x141354680, "rend_141354680", fn_141354680 as *const ()), // sub_141354680
    Replacement::draft(0x141354750, "rend_141354750", fn_141354750 as *const ()), // sub_141354750
    Replacement::draft(0x141354840, "rend_141354840", tpl_82555::<0x8, 0xc, 0x1408746b0> as *const ()), // sub_141354840
    Replacement::draft(0x1413548e0, "rend_1413548e0", fn_1413548e0 as *const ()), // sub_1413548e0
    Replacement::draft(0x1413549a0, "rend_1413549a0", fn_1413549a0 as *const ()), // sub_1413549a0
    Replacement::draft(0x141354a60, "rend_141354a60", tpl_5539::<0x8, 0x1403c92b0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141354a60
    Replacement::draft(0x141354c70, "rend_141354c70", fn_141354c70 as *const ()), // sub_141354c70
    Replacement::draft(0x141354d40, "rend_141354d40", fn_141354d40 as *const ()), // sub_141354d40
    Replacement::draft(0x141354ea0, "rend_141354ea0", fn_141354ea0 as *const ()), // sub_141354ea0
    Replacement::draft(0x141355030, "rend_141355030", fn_141355030 as *const ()), // sub_141355030
    Replacement::draft(0x1413551f0, "rend_1413551f0", tpl_17984::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141354a60> as *const ()), // sub_1413551f0
    Replacement::draft(0x141355370, "rend_141355370", fn_141355370 as *const ()), // sub_141355370
    Replacement::draft(0x1413554f0, "rend_1413554f0", fn_1413554f0 as *const ()), // sub_1413554f0
    Replacement::draft(0x141355530, "rend_141355530", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x143402338, 0x1412eb320> as *const ()), // sub_141355530
    Replacement::draft(0x141355580, "rend_141355580", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1434023c0, 0x1412eb320> as *const ()), // sub_141355580
    Replacement::draft(0x1413555d0, "rend_1413555d0", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1434023e0, 0x1412eb320> as *const ()), // sub_1413555d0
    Replacement::draft(0x141355620, "rend_141355620", tpl_23854::<0x1401de830, 0xa8, 0x98, 0x1434022d0, 0x1412eb320> as *const ()), // sub_141355620
    Replacement::draft(0x141355670, "rend_141355670", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1434023a0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::GeometryData>::rend::SparseUploadBuffer<rend::GeometryData>::vfunc_0
    Replacement::draft(0x1413556e0, "rend_1413556e0", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1434022d0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::ClusterBindingData>::rend::SparseUploadBuffer<rend::ClusterBindingData>::vfunc_0
    Replacement::draft(0x141355750, "rend_141355750", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1434023e0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::MeshletBindingData>::rend::SparseUploadBuffer<rend::MeshletBindingData>::vfunc_0
    Replacement::draft(0x1413557c0, "rend_1413557c0", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x1434023c0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::IndexBindingData>::rend::SparseUploadBuffer<rend::IndexBindingData>::vfunc_0
    Replacement::draft(0x141355830, "rend_141355830", tpl_23857::<0x1401de830, 0xa8, 0x98, 0x143402338, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadBuffer<rend::VertexBindingData>::rend::SparseUploadBuffer<rend::VertexBindingData>::vfunc_0
    Replacement::draft(0x141355c00, "rend_141355c00", fn_141355c00 as *const ()), // rend::SparseUploadBuffer<rend::VertexBindingData>::rend::SparseUploadBuffer<rend::VertexBindingData>::vfunc_4
    Replacement::draft(0x141355d30, "rend_141355d30", fn_141355d30 as *const ()), // rend::SparseUploadBuffer<rend::VertexBindingData>::rend::SparseUploadBuffer<rend::VertexBindingData>::vfunc_5
    Replacement::draft(0x141355dd0, "rend_141355dd0", fn_141355dd0 as *const ()), // rend::SparseUploadBuffer<rend::VertexBindingData>::rend::SparseUploadBuffer<rend::VertexBindingData>::vfunc_6
    Replacement::draft_throws(0x141355e70, "rend_141355e70", fn_141355e70 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::VertexBindingData>::rend::SparseUploadInternal<rend::VertexBindingData>::vfunc_1
    Replacement::draft_throws(0x141355eb0, "rend_141355eb0", fn_141355eb0 as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::VertexBindingData>::rend::SparseUploadInternal<rend::VertexBindingData>::vfunc_2
    Replacement::draft_throws(0x141355ef0, "rend_141355ef0", tpl_24489::<0x1408ce8a0, 0x143868280, 0x142261020, 0x1412f9440, 0x14209c560, 0x14183acbc, 0x14209d6f0, 0x142389840, 0x143868298> as *const (), Throws::Discard), // rend::SparseUploadBuffer<rend::IndexBindingData>::rend::SparseUploadBuffer<rend::IndexBindingData>::vfunc_1
    Replacement::draft_throws(0x1413561b0, "rend_1413561b0", tpl_23986::<0x20, 0x29, 0x10, 0x8, 0x142389840> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::IndexBindingData>::rend::SparseUploadInternal<rend::IndexBindingData>::vfunc_1
    Replacement::draft_throws(0x1413561f0, "rend_1413561f0", tpl_23987::<0x29, 0x20, 0x10, 0x8, 0x142389840> as *const (), Throws::Discard), // rend::SparseUploadBuffer<rend::IndexBindingData>::rend::SparseUploadBuffer<rend::IndexBindingData>::vfunc_2
    Replacement::draft_throws(0x141356230, "rend_141356230", fn_141356230 as *const (), Throws::Discard), // rend::SparseUploadBuffer<rend::ClusterBindingData>::rend::SparseUploadBuffer<rend::ClusterBindingData>::vfunc_1
    Replacement::draft(0x1413564a0, "rend_1413564a0", tpl_24668::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x141356ce0, 0xa0, 0xa4, 0x1408746b0, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::ClusterBindingData>::rend::SparseUploadBuffer<rend::ClusterBindingData>::vfunc_5
    Replacement::draft(0x141356540, "rend_141356540", tpl_24669::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x141356ce0, 0xa0, 0xa4, 0x1408746b0, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::ClusterBindingData>::rend::SparseUploadBuffer<rend::ClusterBindingData>::vfunc_6
    Replacement::draft_throws(0x1413565e0, "rend_1413565e0", tpl_24234::<0x20, 0x29, 0x10, 0x8, 0x142389840> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::ClusterBindingData>::rend::SparseUploadInternal<rend::ClusterBindingData>::vfunc_1
    Replacement::draft_throws(0x141356620, "rend_141356620", tpl_24235::<0x29, 0x20, 0x10, 0x8, 0x142389840> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::ClusterBindingData>::rend::SparseUploadInternal<rend::ClusterBindingData>::vfunc_2
    Replacement::draft(0x141356930, "rend_141356930", tpl_24728::<0x58, 0xffffffffffffffff, 0x90, 0x98, 0x141356d70, 0xa0, 0xa4, 0x140fb1b60, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::GeometryData>::rend::SparseUploadBuffer<rend::GeometryData>::vfunc_5
    Replacement::draft(0x1413569c0, "rend_1413569c0", tpl_24729::<0x90, 0x58, 0xffffffffffffffff, 0x98, 0x141356d70, 0xa0, 0xa4, 0x140fb1b60, 0xa8, 0x140fff1e0> as *const ()), // rend::SparseUploadBuffer<rend::GeometryData>::rend::SparseUploadBuffer<rend::GeometryData>::vfunc_6
    Replacement::draft_throws(0x141356a60, "rend_141356a60", tpl_24730::<0x20, 0x29, 0x10, 0x8, 0x142389840> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryData>::rend::SparseUploadInternal<rend::GeometryData>::vfunc_1
    Replacement::draft_throws(0x141356aa0, "rend_141356aa0", tpl_24731::<0x29, 0x20, 0x10, 0x8, 0x142389840> as *const (), Throws::Discard), // rend::SparseUploadInternal<rend::GeometryData>::rend::SparseUploadInternal<rend::GeometryData>::vfunc_2
    Replacement::draft(0x141356ae0, "rend_141356ae0", tpl_23865::<0x143402338, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::VertexBindingData>::rend::SparseUploadInternal<rend::VertexBindingData>::vfunc_0
    Replacement::draft(0x141356b20, "rend_141356b20", tpl_23865::<0x1434023c0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::IndexBindingData>::rend::SparseUploadInternal<rend::IndexBindingData>::vfunc_0
    Replacement::draft(0x141356b60, "rend_141356b60", tpl_23865::<0x1434023e0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::MeshletBindingData>::rend::SparseUploadInternal<rend::MeshletBindingData>::vfunc_0
    Replacement::draft(0x141356ba0, "rend_141356ba0", tpl_23865::<0x1434022d0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::ClusterBindingData>::rend::SparseUploadInternal<rend::ClusterBindingData>::vfunc_0
    Replacement::draft(0x141356be0, "rend_141356be0", tpl_23865::<0x1434023a0, 0x1412eb320, 0x141834d30> as *const ()), // rend::SparseUploadInternal<rend::GeometryData>::rend::SparseUploadInternal<rend::GeometryData>::vfunc_0
    Replacement::draft(0x141356c20, "rend_141356c20", fn_141356c20 as *const ()), // sub_141356c20
    Replacement::draft(0x141356ce0, "rend_141356ce0", fn_141356ce0 as *const ()), // sub_141356ce0
    Replacement::draft(0x141356d70, "rend_141356d70", fn_141356d70 as *const ()), // sub_141356d70
    Replacement::draft(0x141356e90, "rend_141356e90", fn_141356e90 as *const ()), // sub_141356e90
    Replacement::draft(0x141356ee0, "unknown_141356ee0", tpl_365::<0x141356f20> as *const ()), // sub_141356ee0
    Replacement::draft_throws(0x141356ef0, "unknown_141356ef0", fn_141356ef0 as *const (), Throws::Discard), // sub_141356ef0
    Replacement::draft(0x141356f20, "unknown_141356f20", tpl_422::<0x48, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141356f20
    Replacement::draft(0x141356fb0, "unknown_141356fb0", fn_141356fb0 as *const ()), // sub_141356fb0
    Replacement::draft_throws(0x1413570d0, "sparsehash_1413570d0", fn_1413570d0 as *const (), Throws::Discard), // sub_1413570d0
    Replacement::draft(0x141357310, "unknown_141357310", fn_141357310 as *const ()), // sub_141357310
    Replacement::draft_throws(0x141357470, "unknown_141357470", fn_141357470 as *const (), Throws::Discard), // sub_141357470
    Replacement::draft_throws(0x1413575e0, "unknown_1413575e0", fn_1413575e0 as *const (), Throws::Discard), // sub_1413575e0
    Replacement::draft_throws(0x141357880, "unknown_141357880", fn_141357880 as *const (), Throws::Caught), // sub_141357880
    Replacement::draft(0x141357900, "unknown_141357900", fn_141357900 as *const ()), // sub_141357900
    Replacement::draft_throws(0x141357a20, "rend_141357a20", fn_141357a20 as *const (), Throws::Caught), // sub_141357a20
    Replacement::draft_throws(0x141357ad0, "rend_141357ad0", fn_141357ad0 as *const (), Throws::Caught), // sub_141357ad0
    Replacement::draft_throws(0x141357b40, "rend_141357b40", fn_141357b40 as *const (), Throws::Caught), // sub_141357b40
    Replacement::draft(0x141357c00, "rend_141357c00", fn_141357c00 as *const ()), // sub_141357c00
    Replacement::draft(0x141357d30, "rend_141357d30", fn_141357d30 as *const ()), // sub_141357d30
    Replacement::draft(0x141357f20, "rend_141357f20", fn_141357f20 as *const ()), // sub_141357f20
    Replacement::draft(0x1413580c0, "rend_1413580c0", fn_1413580c0 as *const ()), // sub_1413580c0
    Replacement::draft(0x1413582c0, "rend_1413582c0", fn_1413582c0 as *const ()), // sub_1413582c0
    Replacement::draft(0x1413584d0, "rend_1413584d0", fn_1413584d0 as *const ()), // rend::FlareResource::rend::FlareResource::vfunc_0
    Replacement::draft(0x141358540, "unknown_141358540", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1434030d0, 0x8> as *const ()), // sub_141358540
    Replacement::draft(0x1413585c0, "unknown_1413585c0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1434030d0, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1413585c0
    Replacement::draft_throws(0x141358660, "unknown_141358660", fn_141358660 as *const (), Throws::Caught), // sub_141358660
    Replacement::draft_throws(0x1413586f0, "unknown_1413586f0", tpl_649::<0x143881d78, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413586f0
    Replacement::draft(0x141358770, "unknown_141358770", tpl_23920::<0xc8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x142428d70, 0x143402c00, 0xb8> as *const ()), // sub_141358770
    Replacement::draft(0x141358820, "unknown_141358820", tpl_23921::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143402c00, 0x14183acc2, 0x142428d70, 0xb8> as *const ()), // sub_141358820
    Replacement::draft(0x1413588f0, "unknown_1413588f0", tpl_6868::<0x143881d88, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1413588f0
    Replacement::draft_throws(0x141358980, "unknown_141358980", tpl_23922::<0x143881d90, 0x143881d88, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_141358980
    Replacement::draft_throws(0x141358a00, "unknown_141358a00", tpl_23922::<0x143881d90, 0x143881d88, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_141358a00
    Replacement::draft_throws(0x141358a80, "unknown_141358a80", fn_141358a80 as *const (), Throws::Caught), // sub_141358a80
    Replacement::draft(0x141358b10, "unknown_141358b10", tpl_23923::<0x143881d90, 0x143881d88, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_141358b10
    Replacement::draft_throws(0x141358b90, "unknown_141358b90", tpl_23922::<0x143881d90, 0x143881d88, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_141358b90
    Replacement::draft_throws(0x141358c10, "unknown_141358c10", tpl_23924::<0x143881d88, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_141358c10
    Replacement::draft(0x141358cd0, "unknown_141358cd0", fn_141358cd0 as *const ()), // sub_141358cd0
    Replacement::draft(0x141358d50, "unknown_141358d50", fn_141358d50 as *const ()), // sub_141358d50
    Replacement::draft(0x141358f50, "unknown_141358f50", fn_141358f50 as *const ()), // sub_141358f50
    Replacement::draft(0x141359010, "unknown_141359010", fn_141359010 as *const ()), // sub_141359010
    Replacement::draft(0x141359080, "unknown_141359080", fn_141359080 as *const ()), // sub_141359080
    Replacement::draft(0x141359230, "unknown_141359230", fn_141359230 as *const ()), // sub_141359230
    Replacement::draft(0x1413593b0, "unknown_1413593b0", fn_1413593b0 as *const ()), // sub_1413593b0
    Replacement::draft_throws(0x1413593d0, "defaultloadercallback_1413593d0", fn_1413593d0 as *const (), Throws::Discard), // DefaultLoaderCallback::DefaultLoaderCallback::vfunc_0
    Replacement::draft_throws(0x1413593f0, "defaultloadercallback_1413593f0", fn_1413593f0 as *const (), Throws::Caught), // sub_1413593f0
    Replacement::draft(0x1413594e0, "unknown_1413594e0", fn_1413594e0 as *const ()), // sub_1413594e0
    Replacement::draft_throws(0x141359500, "unknown_141359500", fn_141359500 as *const (), Throws::Discard), // sub_141359500
    Replacement::draft(0x141359770, "unknown_141359770", fn_141359770 as *const ()), // sub_141359770
    Replacement::draft_throws(0x141359790, "unknown_141359790", fn_141359790 as *const (), Throws::Caught), // sub_141359790
    Replacement::draft(0x141359840, "unknown_141359840", tpl_23818::<0x8, 0x1423983f0, 0x142442de0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141359840
    Replacement::draft_throws(0x141359920, "unknown_141359920", fn_141359920 as *const (), Throws::Discard), // sub_141359920
    Replacement::draft(0x141359990, "unknown_141359990", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141359840> as *const ()), // sub_141359990
    Replacement::draft(0x141359b00, "unknown_141359b00", tpl_22065::<0x143bbde8c, 0x143bbde88, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143bbde80, 0x14183acc8, 0x1401de830> as *const ()), // sub_141359b00
    Replacement::draft(0x141359c50, "unknown_141359c50", fn_141359c50 as *const ()), // sub_141359c50
    Replacement::draft(0x141359e30, "unknown_141359e30", fn_141359e30 as *const ()), // sub_141359e30
    Replacement::draft_throws(0x141359f20, "unknown_141359f20", fn_141359f20 as *const (), Throws::Caught), // sub_141359f20
    Replacement::draft(0x14135a030, "unknown_14135a030", fn_14135a030 as *const ()), // sub_14135a030
    Replacement::draft(0x14135a130, "unknown_14135a130", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14135a030> as *const ()), // sub_14135a130
    Replacement::draft_throws(0x14135a2a0, "distancefieldgpudata_14135a2a0", fn_14135a2a0 as *const (), Throws::Caught), // DistanceFieldGPUData::DistanceFieldGPUData::m_pEmpty
    Replacement::draft_throws(0x14135a360, "unknown_14135a360", fn_14135a360 as *const (), Throws::Caught), // sub_14135a360
    Replacement::draft(0x14135a420, "unknown_14135a420", fn_14135a420 as *const ()), // sub_14135a420
    Replacement::draft(0x14135a4a0, "unknown_14135a4a0", fn_14135a4a0 as *const ()), // sub_14135a4a0
    Replacement::draft(0x14135a5d0, "unknown_14135a5d0", fn_14135a5d0 as *const ()), // sub_14135a5d0
    Replacement::draft_throws(0x14135a700, "unknown_14135a700", fn_14135a700 as *const (), Throws::Discard), // sub_14135a700
    Replacement::draft_throws(0x14135a800, "unknown_14135a800", fn_14135a800 as *const (), Throws::Discard), // sub_14135a800
    Replacement::draft_throws(0x14135a920, "unknown_14135a920", fn_14135a920 as *const (), Throws::Caught), // sub_14135a920
    Replacement::draft_throws(0x14135aa80, "unknown_14135aa80", fn_14135aa80 as *const (), Throws::Caught), // sub_14135aa80
    Replacement::draft_throws(0x14135abe0, "unknown_14135abe0", fn_14135abe0 as *const (), Throws::Caught), // sub_14135abe0
    Replacement::draft(0x14135ad40, "unknown_14135ad40", fn_14135ad40 as *const ()), // sub_14135ad40
    Replacement::draft(0x14135ade0, "unknown_14135ade0", fn_14135ade0 as *const ()), // sub_14135ade0
    Replacement::draft(0x14135ae80, "unknown_14135ae80", tpl_23945::<0x143881db0, 0x14243b0b0, 0x338, 0xe8, 0x4, 0x8, 0xc, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14135ae80
    Replacement::draft_throws(0x14135af30, "unknown_14135af30", fn_14135af30 as *const (), Throws::Discard), // sub_14135af30
    Replacement::draft_throws(0x14135aff0, "unknown_14135aff0", fn_14135aff0 as *const (), Throws::Discard), // sub_14135aff0
    Replacement::draft(0x14135b140, "unknown_14135b140", fn_14135b140 as *const ()), // sub_14135b140
    Replacement::draft(0x14135b1e0, "unknown_14135b1e0", fn_14135b1e0 as *const ()), // sub_14135b1e0
    Replacement::draft(0x14135b2b0, "unknown_14135b2b0", fn_14135b2b0 as *const ()), // sub_14135b2b0
    Replacement::draft(0x14135b490, "unknown_14135b490", fn_14135b490 as *const ()), // sub_14135b490
    Replacement::draft_throws(0x14135b670, "unknown_14135b670", fn_14135b670 as *const (), Throws::Caught), // sub_14135b670
    Replacement::draft_throws(0x14135b7e0, "unknown_14135b7e0", fn_14135b7e0 as *const (), Throws::Caught), // sub_14135b7e0
    Replacement::draft(0x14135b990, "unknown_14135b990", fn_14135b990 as *const ()), // sub_14135b990
    Replacement::draft_throws(0x14135ba90, "unknown_14135ba90", fn_14135ba90 as *const (), Throws::Caught), // sub_14135ba90
    Replacement::draft_throws(0x14135bb70, "unknown_14135bb70", fn_14135bb70 as *const (), Throws::Caught), // sub_14135bb70
    Replacement::draft_throws(0x14135bc50, "unknown_14135bc50", fn_14135bc50 as *const (), Throws::Caught), // sub_14135bc50
    Replacement::draft(0x14135bd30, "unknown_14135bd30", fn_14135bd30 as *const ()), // sub_14135bd30
    Replacement::draft(0x14135be40, "unknown_14135be40", fn_14135be40 as *const ()), // sub_14135be40
    Replacement::draft(0x14135c030, "unknown_14135c030", fn_14135c030 as *const ()), // sub_14135c030
    Replacement::draft_throws(0x14135c230, "unknown_14135c230", fn_14135c230 as *const (), Throws::Discard), // sub_14135c230
    Replacement::draft_throws(0x14135c320, "unknown_14135c320", fn_14135c320 as *const (), Throws::Caught), // sub_14135c320
    Replacement::draft(0x14135c480, "unknown_14135c480", fn_14135c480 as *const ()), // sub_14135c480
    Replacement::draft(0x14135c510, "unknown_14135c510", fn_14135c510 as *const ()), // sub_14135c510
    Replacement::draft(0x14135c5b0, "unknown_14135c5b0", fn_14135c5b0 as *const ()), // sub_14135c5b0
    Replacement::draft_throws(0x14135c6c0, "unknown_14135c6c0", fn_14135c6c0 as *const (), Throws::Discard), // sub_14135c6c0
    Replacement::draft_throws(0x14135c840, "unknown_14135c840", fn_14135c840 as *const (), Throws::Discard), // sub_14135c840
    Replacement::draft(0x14135c950, "unknown_14135c950", fn_14135c950 as *const ()), // sub_14135c950
    Replacement::draft_throws(0x14135c9f0, "unknown_14135c9f0", fn_14135c9f0 as *const (), Throws::Caught), // sub_14135c9f0
    Replacement::draft_throws(0x14135cdb0, "unknown_14135cdb0", fn_14135cdb0 as *const (), Throws::Caught), // sub_14135cdb0
    Replacement::draft_throws(0x14135d0f0, "unknown_14135d0f0", fn_14135d0f0 as *const (), Throws::Caught), // sub_14135d0f0
    Replacement::draft(0x14135d420, "unknown_14135d420", fn_14135d420 as *const ()), // sub_14135d420
    Replacement::draft(0x14135d530, "unknown_14135d530", fn_14135d530 as *const ()), // sub_14135d530
    Replacement::draft(0x14135d660, "unknown_14135d660", fn_14135d660 as *const ()), // sub_14135d660
    Replacement::draft_throws(0x14135d7a0, "unknown_14135d7a0", fn_14135d7a0 as *const (), Throws::Discard), // sub_14135d7a0
    Replacement::draft_throws(0x14135d8c0, "unknown_14135d8c0", fn_14135d8c0 as *const (), Throws::Discard), // sub_14135d8c0
    Replacement::draft(0x14135d9f0, "unknown_14135d9f0", fn_14135d9f0 as *const ()), // sub_14135d9f0
    Replacement::draft_throws(0x14135dc00, "unknown_14135dc00", fn_14135dc00 as *const (), Throws::Caught), // sub_14135dc00
    Replacement::draft_throws(0x14135ddd0, "unknown_14135ddd0", fn_14135ddd0 as *const (), Throws::Caught), // sub_14135ddd0
    Replacement::draft_throws(0x14135e030, "unknown_14135e030", fn_14135e030 as *const (), Throws::Caught), // sub_14135e030
    Replacement::draft_throws(0x14135e200, "unknown_14135e200", fn_14135e200 as *const (), Throws::Caught), // sub_14135e200
    Replacement::draft_throws(0x14135e460, "unknown_14135e460", fn_14135e460 as *const (), Throws::Caught), // sub_14135e460
    Replacement::draft_throws(0x14135e6c0, "unknown_14135e6c0", fn_14135e6c0 as *const (), Throws::Caught), // sub_14135e6c0
    Replacement::draft_throws(0x14135e920, "unknown_14135e920", fn_14135e920 as *const (), Throws::Caught), // sub_14135e920
    Replacement::draft(0x14135eb80, "unknown_14135eb80", fn_14135eb80 as *const ()), // sub_14135eb80
    Replacement::draft(0x14135ecf0, "unknown_14135ecf0", fn_14135ecf0 as *const ()), // sub_14135ecf0
    Replacement::draft_throws(0x14135eea0, "unknown_14135eea0", fn_14135eea0 as *const (), Throws::Caught), // sub_14135eea0
    Replacement::draft_throws(0x14135f340, "unknown_14135f340", fn_14135f340 as *const (), Throws::Caught), // sub_14135f340
    Replacement::draft_throws(0x14135f7e0, "unknown_14135f7e0", fn_14135f7e0 as *const (), Throws::Caught), // sub_14135f7e0
    Replacement::draft_throws(0x14135fe80, "unknown_14135fe80", fn_14135fe80 as *const (), Throws::Caught), // sub_14135fe80
    Replacement::draft_throws(0x1413602f0, "unknown_1413602f0", fn_1413602f0 as *const (), Throws::Caught), // sub_1413602f0
    Replacement::draft_throws(0x141360760, "unknown_141360760", fn_141360760 as *const (), Throws::Caught), // sub_141360760
    Replacement::draft_throws(0x141360dd0, "unknown_141360dd0", fn_141360dd0 as *const (), Throws::Caught), // sub_141360dd0
    Replacement::draft_throws(0x141361240, "unknown_141361240", fn_141361240 as *const (), Throws::Caught), // sub_141361240
    Replacement::draft_throws(0x1413616b0, "unknown_1413616b0", fn_1413616b0 as *const (), Throws::Caught), // sub_1413616b0
    Replacement::draft(0x141361af0, "unknown_141361af0", fn_141361af0 as *const ()), // sub_141361af0
    Replacement::draft_throws(0x141361c40, "unknown_141361c40", fn_141361c40 as *const (), Throws::Caught), // sub_141361c40
    Replacement::draft_throws(0x141361d10, "unknown_141361d10", fn_141361d10 as *const (), Throws::Caught), // sub_141361d10
    Replacement::draft(0x141361dd0, "unknown_141361dd0", fn_141361dd0 as *const ()), // sub_141361dd0
    Replacement::draft_throws(0x141361e40, "unknown_141361e40", fn_141361e40 as *const (), Throws::Caught), // sub_141361e40
    Replacement::draft_throws(0x141361f90, "unknown_141361f90", fn_141361f90 as *const (), Throws::Caught), // sub_141361f90
    Replacement::draft_throws(0x1413620e0, "unknown_1413620e0", fn_1413620e0 as *const (), Throws::Caught), // sub_1413620e0
    Replacement::draft_throws(0x141362230, "unknown_141362230", fn_141362230 as *const (), Throws::Discard), // sub_141362230
    Replacement::draft_throws(0x141362270, "unknown_141362270", fn_141362270 as *const (), Throws::Caught), // sub_141362270
    Replacement::draft_throws(0x1413626e0, "unknown_1413626e0", fn_1413626e0 as *const (), Throws::Caught), // sub_1413626e0
    Replacement::draft_throws(0x1413629a0, "unknown_1413629a0", fn_1413629a0 as *const (), Throws::Caught), // sub_1413629a0
    Replacement::draft_throws(0x141362dd0, "unknown_141362dd0", fn_141362dd0 as *const (), Throws::Caught), // sub_141362dd0
    Replacement::draft_throws(0x141363060, "unknown_141363060", fn_141363060 as *const (), Throws::Caught), // sub_141363060
    Replacement::draft_throws(0x1413633b0, "unknown_1413633b0", fn_1413633b0 as *const (), Throws::Caught), // sub_1413633b0
    Replacement::draft_throws(0x141363640, "unknown_141363640", fn_141363640 as *const (), Throws::Caught), // sub_141363640
    Replacement::draft_throws(0x141363830, "unknown_141363830", fn_141363830 as *const (), Throws::Caught), // sub_141363830
    Replacement::draft_throws(0x141363910, "unknown_141363910", fn_141363910 as *const (), Throws::Caught), // sub_141363910
    Replacement::draft_throws(0x141363e00, "unknown_141363e00", fn_141363e00 as *const (), Throws::Caught), // sub_141363e00
    Replacement::draft_throws(0x1413642b0, "unknown_1413642b0", fn_1413642b0 as *const (), Throws::Caught), // sub_1413642b0
    Replacement::draft_throws(0x141364760, "unknown_141364760", fn_141364760 as *const (), Throws::Discard), // sub_141364760
    Replacement::draft_throws(0x141364790, "unknown_141364790", fn_141364790 as *const (), Throws::Discard), // sub_141364790
    Replacement::draft_throws(0x1413647d0, "unknown_1413647d0", fn_1413647d0 as *const (), Throws::Discard), // sub_1413647d0
    Replacement::draft(0x141364820, "unknown_141364820", fn_141364820 as *const ()), // sub_141364820
    Replacement::draft_throws(0x141364890, "unknown_141364890", fn_141364890 as *const (), Throws::Discard), // sub_141364890
    Replacement::draft_throws(0x141364920, "unknown_141364920", fn_141364920 as *const (), Throws::Discard), // sub_141364920
    Replacement::draft(0x141364a60, "unknown_141364a60", fn_141364a60 as *const ()), // sub_141364a60
    Replacement::draft(0x141364b70, "unknown_141364b70", tpl_24432::<0x98, 0x10, 0x18, 0xffffffffffffff80, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141364a60> as *const ()), // sub_141364b70
    Replacement::draft(0x141364bf0, "unknown_141364bf0", fn_141364bf0 as *const ()), // sub_141364bf0
    Replacement::draft(0x141364d00, "unknown_141364d00", fn_141364d00 as *const ()), // sub_141364d00
    Replacement::draft(0x141364da0, "unknown_141364da0", fn_141364da0 as *const ()), // sub_141364da0
    Replacement::draft_throws(0x141364f70, "unknown_141364f70", fn_141364f70 as *const (), Throws::Discard), // sub_141364f70
    Replacement::draft(0x141365190, "unknown_141365190", tpl_24854::<0x143881dc0, 0x14243b0b0, 0x338, 0x140715850, 0x141366760, 0x60, 0x48, 0x64, 0x68, 0x38, 0x6c, 0x70, 0x1406fe890> as *const ()), // sub_141365190
    Replacement::draft_throws(0x141365270, "unknown_141365270", fn_141365270 as *const (), Throws::Discard), // sub_141365270
    Replacement::draft(0x141365550, "unknown_141365550", fn_141365550 as *const ()), // sub_141365550
    Replacement::draft_throws(0x141365650, "unknown_141365650", fn_141365650 as *const (), Throws::Caught), // sub_141365650
    Replacement::draft_throws(0x141365860, "unknown_141365860", fn_141365860 as *const (), Throws::Caught), // sub_141365860
    Replacement::draft_throws(0x141365a70, "unknown_141365a70", fn_141365a70 as *const (), Throws::Caught), // sub_141365a70
    Replacement::draft(0x141365ee0, "unknown_141365ee0", fn_141365ee0 as *const ()), // sub_141365ee0
    Replacement::draft(0x141366090, "unknown_141366090", fn_141366090 as *const ()), // sub_141366090
    Replacement::draft(0x141366170, "unknown_141366170", tpl_24854::<0x143881dc0, 0x14243b0b0, 0x338, 0x140715760, 0x141366760, 0x60, 0xe8, 0x64, 0x68, 0xd8, 0x6c, 0x70, 0x1406feb00> as *const ()), // sub_141366170
    Replacement::draft_throws(0x141366250, "unknown_141366250", fn_141366250 as *const (), Throws::Discard), // sub_141366250
    Replacement::draft_throws(0x1413663e0, "unknown_1413663e0", fn_1413663e0 as *const (), Throws::Caught), // sub_1413663e0
    Replacement::draft_throws(0x141366690, "unknown_141366690", fn_141366690 as *const (), Throws::Discard), // sub_141366690
    Replacement::draft(0x141366760, "unknown_141366760", fn_141366760 as *const ()), // sub_141366760
    Replacement::draft_throws(0x141366870, "unknown_141366870", fn_141366870 as *const (), Throws::Discard), // sub_141366870
    Replacement::draft_throws(0x141366e50, "unknown_141366e50", fn_141366e50 as *const (), Throws::Discard), // sub_141366e50
    Replacement::draft_throws(0x141367430, "unknown_141367430", fn_141367430 as *const (), Throws::Caught), // sub_141367430
    Replacement::draft_throws(0x1413674e0, "unknown_1413674e0", fn_1413674e0 as *const (), Throws::Caught), // sub_1413674e0
    Replacement::draft(0x141367590, "unknown_141367590", fn_141367590 as *const ()), // sub_141367590
    Replacement::draft(0x141367610, "unknown_141367610", fn_141367610 as *const ()), // sub_141367610
    Replacement::draft(0x141367940, "unknown_141367940", fn_141367940 as *const ()), // sub_141367940
    Replacement::draft(0x141367c70, "unknown_141367c70", fn_141367c70 as *const ()), // sub_141367c70
    Replacement::draft(0x141367f40, "unknown_141367f40", fn_141367f40 as *const ()), // sub_141367f40
    Replacement::draft(0x141368230, "unknown_141368230", fn_141368230 as *const ()), // sub_141368230
    Replacement::draft(0x1413683e0, "unknown_1413683e0", fn_1413683e0 as *const ()), // sub_1413683e0
    Replacement::draft(0x1413686b0, "unknown_1413686b0", tpl_24867::<0xffffffffffffffff, 0x8, 0x10, 0xc0, 0x2, 0xfffffffffffffff8> as *const ()), // sub_1413686b0
    Replacement::draft(0x141368790, "unknown_141368790", fn_141368790 as *const ()), // sub_141368790
    Replacement::draft(0x141368850, "unknown_141368850", tpl_24867::<0xffffffffffffffff, 0x8, 0x10, 0x13b8, 0x2, 0xfffffffffffffff8> as *const ()), // sub_141368850
    Replacement::draft(0x141368930, "unknown_141368930", tpl_24869::<0xc0> as *const ()), // sub_141368930
    Replacement::draft(0x1413689c0, "unknown_1413689c0", tpl_24869::<0x13b8> as *const ()), // sub_1413689c0
    Replacement::draft_throws(0x141368a60, "unknown_141368a60", fn_141368a60 as *const (), Throws::Discard), // sub_141368a60
    Replacement::draft(0x141368b50, "unknown_141368b50", fn_141368b50 as *const ()), // sub_141368b50
    Replacement::draft_throws(0x141368c20, "unknown_141368c20", fn_141368c20 as *const (), Throws::Discard), // sub_141368c20
    Replacement::draft(0x141368e00, "unknown_141368e00", fn_141368e00 as *const ()), // sub_141368e00
    Replacement::draft_throws(0x141369010, "unknown_141369010", fn_141369010 as *const (), Throws::Discard), // sub_141369010
    Replacement::draft(0x1413690e0, "unknown_1413690e0", fn_1413690e0 as *const ()), // sub_1413690e0
    Replacement::draft(0x141369200, "unknown_141369200", fn_141369200 as *const ()), // sub_141369200
    Replacement::draft(0x141369370, "unknown_141369370", fn_141369370 as *const ()), // sub_141369370
    Replacement::draft(0x1413697a0, "unknown_1413697a0", fn_1413697a0 as *const ()), // sub_1413697a0
    Replacement::draft(0x141369960, "unknown_141369960", fn_141369960 as *const ()), // sub_141369960
    Replacement::draft(0x141369ad0, "unknown_141369ad0", fn_141369ad0 as *const ()), // sub_141369ad0
    Replacement::draft(0x141369d20, "unknown_141369d20", fn_141369d20 as *const ()), // sub_141369d20
    Replacement::draft(0x141369e70, "unknown_141369e70", fn_141369e70 as *const ()), // sub_141369e70
    Replacement::draft(0x14136a0a0, "unknown_14136a0a0", fn_14136a0a0 as *const ()), // sub_14136a0a0
    Replacement::draft_throws(0x14136a210, "unknown_14136a210", fn_14136a210 as *const (), Throws::Discard), // sub_14136a210
    Replacement::draft(0x14136a390, "unknown_14136a390", fn_14136a390 as *const ()), // sub_14136a390
    Replacement::draft(0x14136a530, "unknown_14136a530", fn_14136a530 as *const ()), // sub_14136a530
    Replacement::draft(0x14136a6c0, "unknown_14136a6c0", fn_14136a6c0 as *const ()), // sub_14136a6c0
    Replacement::draft(0x14136a8d0, "unknown_14136a8d0", fn_14136a8d0 as *const ()), // sub_14136a8d0
    Replacement::draft(0x14136aa90, "unknown_14136aa90", fn_14136aa90 as *const ()), // sub_14136aa90
    Replacement::draft(0x14136ab70, "unknown_14136ab70", fn_14136ab70 as *const ()), // sub_14136ab70
    Replacement::draft(0x14136abb0, "unknown_14136abb0", fn_14136abb0 as *const ()), // sub_14136abb0
    Replacement::draft(0x14136ac00, "unknown_14136ac00", fn_14136ac00 as *const ()), // sub_14136ac00
    Replacement::draft_throws(0x14136ac40, "unknown_14136ac40", fn_14136ac40 as *const (), Throws::Caught), // sub_14136ac40
    Replacement::draft(0x14136b090, "unknown_14136b090", fn_14136b090 as *const ()), // sub_14136b090
    Replacement::draft(0x14136b160, "unknown_14136b160", tpl_10030::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8> as *const ()), // sub_14136b160
    Replacement::draft(0x14136b1e0, "commandbufferthreading_14136b1e0", fn_14136b1e0 as *const ()), // CommandBufferThreading::CommandBufferThreading::vfunc_0
    Replacement::draft(0x14136b210, "commandbufferthreading_14136b210", tpl_24891::<0x143a383e8, 0x8, 0xc, 0x1411dba80, 0x10, 0x143879648, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // CommandBufferThreading::CommandBufferThreading::vfunc_1
    Replacement::draft_throws(0x14136b2d0, "unknown_14136b2d0", fn_14136b2d0 as *const (), Throws::Caught), // sub_14136b2d0
    Replacement::draft(0x14136b360, "unknown_14136b360", fn_14136b360 as *const ()), // sub_14136b360
    Replacement::draft(0x14136b4d0, "unknown_14136b4d0", fn_14136b4d0 as *const ()), // sub_14136b4d0
    Replacement::draft(0x14136b600, "unknown_14136b600", fn_14136b600 as *const ()), // sub_14136b600
    Replacement::draft(0x14136ba70, "unknown_14136ba70", fn_14136ba70 as *const ()), // sub_14136ba70
    Replacement::draft(0x14136bd30, "unknown_14136bd30", fn_14136bd30 as *const ()), // sub_14136bd30
    Replacement::draft(0x14136be30, "unknown_14136be30", fn_14136be30 as *const ()), // sub_14136be30
    Replacement::draft(0x14136bea0, "unknown_14136bea0", tpl_365::<0x14136c160> as *const ()), // sub_14136bea0
    Replacement::draft(0x14136bec0, "unknown_14136bec0", tpl_6558::<0x140587600> as *const ()), // sub_14136bec0
    Replacement::draft(0x14136bee0, "unknown_14136bee0", fn_14136bee0 as *const ()), // sub_14136bee0
    Replacement::draft_throws(0x14136c080, "unknown_14136c080", fn_14136c080 as *const (), Throws::Caught), // sub_14136c080
    Replacement::draft(0x14136c160, "unknown_14136c160", fn_14136c160 as *const ()), // sub_14136c160
    Replacement::draft_throws(0x14136c250, "unknown_14136c250", fn_14136c250 as *const (), Throws::Discard), // sub_14136c250
    Replacement::draft(0x14136c300, "sparsehash_14136c300", fn_14136c300 as *const ()), // sub_14136c300
    Replacement::draft_throws(0x14136c450, "sparsehash_14136c450", fn_14136c450 as *const (), Throws::Discard), // sub_14136c450
    Replacement::draft(0x14136c690, "sparsehash_14136c690", fn_14136c690 as *const ()), // sub_14136c690
    Replacement::draft_throws(0x14136c790, "sparsehash_14136c790", fn_14136c790 as *const (), Throws::Caught), // sub_14136c790
    Replacement::draft(0x14136c890, "sparsehash_14136c890", fn_14136c890 as *const ()), // sub_14136c890
    Replacement::draft_throws(0x14136cac0, "sparsehash_14136cac0", fn_14136cac0 as *const (), Throws::Caught), // sub_14136cac0
    Replacement::draft_throws(0x14136cc80, "sparsehash_14136cc80", fn_14136cc80 as *const (), Throws::Caught), // sub_14136cc80
    Replacement::draft(0x14136ce50, "unknown_14136ce50", fn_14136ce50 as *const ()), // sub_14136ce50
    Replacement::draft(0x14136d030, "unknown_14136d030", fn_14136d030 as *const ()), // sub_14136d030
    Replacement::draft(0x14136d0d0, "unknown_14136d0d0", fn_14136d0d0 as *const ()), // sub_14136d0d0
    Replacement::draft(0x14136d1d0, "unknown_14136d1d0", fn_14136d1d0 as *const ()), // sub_14136d1d0
    Replacement::draft_throws(0x14136d290, "unknown_14136d290", fn_14136d290 as *const (), Throws::Caught), // sub_14136d290
    Replacement::draft_throws(0x14136d340, "unknown_14136d340", fn_14136d340 as *const (), Throws::Caught), // sub_14136d340
    Replacement::draft(0x14136d3f0, "unknown_14136d3f0", fn_14136d3f0 as *const ()), // sub_14136d3f0
    Replacement::draft_throws(0x14136d530, "rend_14136d530", fn_14136d530 as *const (), Throws::Discard), // rend::AutoExposureHistogram::rend::AutoExposureHistogram::vfunc_0
    Replacement::draft(0x14136d5f0, "rend_14136d5f0", tpl_24915 as *const ()), // sub_14136d5f0
    Replacement::draft_throws(0x14136d600, "rend_14136d600", fn_14136d600 as *const (), Throws::Caught), // sub_14136d600
    Replacement::draft_throws(0x14136d7a0, "rend_14136d7a0", fn_14136d7a0 as *const (), Throws::Caught), // sub_14136d7a0
    Replacement::draft_throws(0x14136d950, "rend_14136d950", fn_14136d950 as *const (), Throws::Caught), // sub_14136d950
    Replacement::draft_throws(0x14136db00, "rend_14136db00", fn_14136db00 as *const (), Throws::Caught), // sub_14136db00
    Replacement::draft_throws(0x14136dcb0, "rend_14136dcb0", fn_14136dcb0 as *const (), Throws::Caught), // sub_14136dcb0
    Replacement::draft_throws(0x14136de60, "unknown_14136de60", fn_14136de60 as *const (), Throws::Caught), // sub_14136de60
    Replacement::draft_throws(0x14136e010, "rend_14136e010", fn_14136e010 as *const (), Throws::Caught), // sub_14136e010
    Replacement::draft_throws(0x14136e1c0, "rend_14136e1c0", fn_14136e1c0 as *const (), Throws::Caught), // sub_14136e1c0
    Replacement::draft_throws(0x14136e370, "rend_14136e370", fn_14136e370 as *const (), Throws::Caught), // sub_14136e370
    Replacement::draft_throws(0x14136e520, "rend_14136e520", fn_14136e520 as *const (), Throws::Caught), // sub_14136e520
    Replacement::draft_throws(0x14136e6c0, "rend_14136e6c0", fn_14136e6c0 as *const (), Throws::Caught), // sub_14136e6c0
    Replacement::draft_throws(0x14136e870, "rend_14136e870", fn_14136e870 as *const (), Throws::Discard), // rend::AmbientProbe::rend::AmbientProbe::vfunc_0
    Replacement::draft_throws(0x14136e8c0, "unknown_14136e8c0", fn_14136e8c0 as *const (), Throws::Discard), // sub_14136e8c0
    Replacement::draft_throws(0x14136e920, "unknown_14136e920", fn_14136e920 as *const (), Throws::Discard), // sub_14136e920
    Replacement::draft_throws(0x14136ea20, "unknown_14136ea20", fn_14136ea20 as *const (), Throws::Discard), // sub_14136ea20
    Replacement::draft_throws(0x14136ea90, "unknown_14136ea90", fn_14136ea90 as *const (), Throws::Caught), // sub_14136ea90
    Replacement::draft_throws(0x14136eb30, "unknown_14136eb30", fn_14136eb30 as *const (), Throws::Discard), // sub_14136eb30
    Replacement::draft_throws(0x14136eba0, "unknown_14136eba0", fn_14136eba0 as *const (), Throws::Caught), // sub_14136eba0
    Replacement::draft_throws(0x14136ec50, "ambientprobe_14136ec50", fn_14136ec50 as *const (), Throws::Caught), // AmbientProbe::AmbientProbe::m_pDepthBuffer
    Replacement::draft_throws(0x14136ed00, "unknown_14136ed00", fn_14136ed00 as *const (), Throws::Discard), // sub_14136ed00
    Replacement::draft(0x14136ed90, "unknown_14136ed90", fn_14136ed90 as *const ()), // sub_14136ed90
    Replacement::draft(0x14136ee20, "unknown_14136ee20", fn_14136ee20 as *const ()), // sub_14136ee20
    Replacement::draft_throws(0x14136eec0, "unknown_14136eec0", fn_14136eec0 as *const (), Throws::Caught), // sub_14136eec0
    Replacement::draft(0x14136ef30, "std_14136ef30", tpl_1::<0x143406d00> as *const ()), // std::_System_error_category::std::_System_error_category::vfunc_1
    Replacement::draft(0x14136ef40, "std_14136ef40", tpl_6190::<0x1418344ec> as *const ()), // sub_14136ef40
    Replacement::draft_throws(0x14136ef50, "std_14136ef50", fn_14136ef50 as *const (), Throws::Caught), // std::_System_error_category::std::_System_error_category::vfunc_2
    Replacement::draft(0x14136eff0, "std_14136eff0", fn_14136eff0 as *const ()), // std::_System_error_category::std::_System_error_category::vfunc_3
    Replacement::draft(0x14136f070, "unknown_14136f070", tpl_22065::<0x143c161c4, 0x143c161c0, 0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143c161b8, 0x14183acc8, 0x1401de830> as *const ()), // sub_14136f070
    Replacement::draft(0x14136f1c0, "unknown_14136f1c0", tpl_24940::<0x14340ed48, 0x8, 0x143406e80, 0x143406e40, 0x142498f00> as *const ()), // sub_14136f1c0
    Replacement::draft_throws(0x14136f220, "unknown_14136f220", fn_14136f220 as *const (), Throws::Caught), // sub_14136f220
    Replacement::draft(0x141371d60, "unknown_141371d60", fn_141371d60 as *const ()), // sub_141371d60
    Replacement::draft(0x141371f00, "unknown_141371f00", fn_141371f00 as *const ()), // sub_141371f00
    Replacement::draft(0x1413722f0, "unknown_1413722f0", fn_1413722f0 as *const ()), // sub_1413722f0
    Replacement::draft(0x141372450, "std_141372450", fn_141372450 as *const ()), // sub_141372450
    Replacement::draft(0x14137253c, "unknown_14137253c", tpl_5167::<0x48> as *const ()), // sub_14137253c
    Replacement::draft(0x141372544, "unknown_141372544", tpl_5167::<0x58> as *const ()), // sub_141372544
    Replacement::draft(0x14137254c, "unknown_14137254c", tpl_5167::<0x68> as *const ()), // sub_14137254c
    Replacement::draft(0x141372554, "unknown_141372554", tpl_5167::<0x78> as *const ()), // sub_141372554
    Replacement::draft(0x14137255c, "unknown_14137255c", tpl_5167::<0x10> as *const ()), // sub_14137255c
    Replacement::draft(0x141372564, "unknown_141372564", fn_141372564 as *const ()), // sub_141372564
    Replacement::draft(0x14137256c, "unknown_14137256c", tpl_5167::<0x30> as *const ()), // sub_14137256c
    Replacement::draft(0x141372574, "unknown_141372574", tpl_5167::<0x50> as *const ()), // sub_141372574
    Replacement::draft(0x14137257c, "unknown_14137257c", tpl_5167::<0x60> as *const ()), // sub_14137257c
    Replacement::draft(0x141372584, "unknown_141372584", tpl_5167::<0x70> as *const ()), // sub_141372584
    Replacement::draft(0x14137258c, "unknown_14137258c", tpl_5167::<0x80> as *const ()), // sub_14137258c
    Replacement::draft(0x141372598, "unknown_141372598", tpl_5167::<0x18> as *const ()), // sub_141372598
    Replacement::draft(0x1413725a0, "unknown_1413725a0", fn_1413725a0 as *const ()), // sub_1413725a0
    Replacement::draft(0x1413725a8, "unknown_1413725a8", tpl_5167::<0x38> as *const ()), // sub_1413725a8
    Replacement::draft(0x141376e30, "unknown_141376e30", fn_141376e30 as *const ()), // sub_141376e30
    Replacement::draft(0x141376f10, "unknown_141376f10", fn_141376f10 as *const ()), // sub_141376f10
    Replacement::draft(0x141377100, "unknown_141377100", fn_141377100 as *const ()), // sub_141377100
    Replacement::draft_throws(0x141377260, "unknown_141377260", fn_141377260 as *const (), Throws::Caught), // sub_141377260
    Replacement::draft(0x141377460, "unknown_141377460", fn_141377460 as *const ()), // sub_141377460
    Replacement::draft(0x1413776c0, "unknown_1413776c0", fn_1413776c0 as *const ()), // sub_1413776c0
    Replacement::draft(0x1413778a0, "unknown_1413778a0", fn_1413778a0 as *const ()), // sub_1413778a0
    Replacement::draft(0x141378020, "unknown_141378020", fn_141378020 as *const ()), // sub_141378020
    Replacement::draft(0x1413780c0, "unknown_1413780c0", fn_1413780c0 as *const ()), // sub_1413780c0
    Replacement::draft(0x141378320, "unknown_141378320", fn_141378320 as *const ()), // sub_141378320
    Replacement::draft(0x141378410, "d_141378410", tpl_22067::<0x142406460, 0x141834d30> as *const ()), // d::BaseTweakable::d::BaseTweakable::vfunc_0
    Replacement::draft(0x141378480, "unknown_141378480", fn_141378480 as *const ()), // sub_141378480
    Replacement::draft_throws(0x141378490, "unknown_141378490", fn_141378490 as *const (), Throws::Caught), // sub_141378490
    Replacement::draft(0x141378580, "unknown_141378580", fn_141378580 as *const ()), // sub_141378580
    Replacement::draft_throws(0x141378730, "unknown_141378730", fn_141378730 as *const (), Throws::Discard), // sub_141378730
    Replacement::draft_throws(0x141378850, "unknown_141378850", fn_141378850 as *const (), Throws::Caught), // sub_141378850
    Replacement::draft_throws(0x141378930, "unknown_141378930", tpl_22036::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141378850> as *const (), Throws::Discard), // sub_141378930
    Replacement::draft_throws(0x141378aa0, "unknown_141378aa0", fn_141378aa0 as *const (), Throws::Discard), // sub_141378aa0
    Replacement::draft(0x141378c80, "unknown_141378c80", fn_141378c80 as *const ()), // sub_141378c80
    Replacement::draft(0x141378dc0, "unknown_141378dc0", fn_141378dc0 as *const ()), // sub_141378dc0
    Replacement::draft_throws(0x141379190, "std_141379190", fn_141379190 as *const (), Throws::Discard), // sub_141379190
    Replacement::draft_throws(0x141379c80, "unknown_141379c80", fn_141379c80 as *const (), Throws::Caught), // sub_141379c80
    Replacement::draft(0x141379cf0, "global_141379cf0", tpl_83353::<0x1405eb9e0, 0x1426468b8> as *const ()), // FID_conflict:sprintf_s
    Replacement::draft(0x14139bb80, "unknown_14139bb80", fn_14139bb80 as *const ()), // sub_14139bb80
    Replacement::draft(0x14139bbf0, "unknown_14139bbf0", fn_14139bbf0 as *const ()), // sub_14139bbf0
    Replacement::draft(0x14139bcb0, "unknown_14139bcb0", tpl_25139::<0x142646558> as *const ()), // sub_14139bcb0
    Replacement::draft(0x14139bcc0, "unknown_14139bcc0", fn_14139bcc0 as *const ()), // sub_14139bcc0
    Replacement::draft_throws(0x14139bd20, "unknown_14139bd20", fn_14139bd20 as *const (), Throws::Discard), // sub_14139bd20
    Replacement::draft_throws(0x14139bdf0, "unknown_14139bdf0", fn_14139bdf0 as *const (), Throws::Discard), // sub_14139bdf0
    Replacement::draft_throws(0x14139bf60, "unknown_14139bf60", fn_14139bf60 as *const (), Throws::Discard), // sub_14139bf60
    Replacement::draft(0x14139c120, "unknown_14139c120", fn_14139c120 as *const ()), // sub_14139c120
    Replacement::draft(0x14139c190, "unknown_14139c190", fn_14139c190 as *const ()), // sub_14139c190
    Replacement::draft(0x14139c2b0, "unknown_14139c2b0", fn_14139c2b0 as *const ()), // sub_14139c2b0
    Replacement::draft_throws(0x14139c310, "unknown_14139c310", fn_14139c310 as *const (), Throws::Discard), // sub_14139c310
    Replacement::draft(0x14139c480, "unknown_14139c480", fn_14139c480 as *const ()), // sub_14139c480
    Replacement::draft(0x14139c490, "unknown_14139c490", tpl_25145::<0x142646558> as *const ()), // sub_14139c490
    Replacement::draft(0x14139c4a0, "unknown_14139c4a0", fn_14139c4a0 as *const ()), // sub_14139c4a0
    Replacement::draft_throws(0x14139c560, "unknown_14139c560", fn_14139c560 as *const (), Throws::Caught), // sub_14139c560
    Replacement::draft_throws(0x14139c600, "unknown_14139c600", fn_14139c600 as *const (), Throws::Caught), // sub_14139c600
    Replacement::draft_throws(0x14139c6b0, "unknown_14139c6b0", tpl_16403::<0x20, 0x1402da920> as *const (), Throws::Discard), // sub_14139c6b0
    Replacement::draft_throws(0x14139c6e0, "unknown_14139c6e0", fn_14139c6e0 as *const (), Throws::Discard), // sub_14139c6e0
    Replacement::draft_throws(0x14139c710, "unknown_14139c710", tpl_23800::<0x20, 0x8, 0x14139d5e0> as *const (), Throws::Discard), // sub_14139c710
    Replacement::draft_throws(0x14139c730, "unknown_14139c730", fn_14139c730 as *const (), Throws::Caught), // sub_14139c730
    Replacement::draft_throws(0x14139cc40, "unknown_14139cc40", fn_14139cc40 as *const (), Throws::Caught), // sub_14139cc40
    Replacement::draft_throws(0x14139d110, "unknown_14139d110", fn_14139d110 as *const (), Throws::Caught), // sub_14139d110
    Replacement::draft_throws(0x14139d5e0, "unknown_14139d5e0", fn_14139d5e0 as *const (), Throws::Caught), // sub_14139d5e0
    Replacement::draft(0x14139d6b0, "unknown_14139d6b0", fn_14139d6b0 as *const ()), // sub_14139d6b0
    Replacement::draft(0x14139d720, "unknown_14139d720", tpl_170::<0x14139d7c0> as *const ()), // sub_14139d720
    Replacement::draft(0x14139d730, "unknown_14139d730", fn_14139d730 as *const ()), // sub_14139d730
    Replacement::draft(0x14139d7c0, "unknown_14139d7c0", fn_14139d7c0 as *const ()), // sub_14139d7c0
    Replacement::draft(0x14139d890, "unknown_14139d890", fn_14139d890 as *const ()), // sub_14139d890
    Replacement::draft(0x14139da10, "unknown_14139da10", tpl_25157::<0x1434286c0, 0x143428920, 0x143427f00, 0x143428940, 0x1434289b0, 0x143428790> as *const ()), // sub_14139da10
    Replacement::draft(0x14139da90, "unknown_14139da90", tpl_25157::<0x143428700, 0x143428780, 0x1434285c0, 0x143428930, 0x143427eb0, 0x143428660> as *const ()), // sub_14139da90
    Replacement::draft(0x14139db10, "unknown_14139db10", fn_14139db10 as *const ()), // sub_14139db10
    Replacement::draft(0x14139dd10, "unknown_14139dd10", fn_14139dd10 as *const ()), // sub_14139dd10
    Replacement::draft_throws(0x14139ddb0, "unknown_14139ddb0", fn_14139ddb0 as *const (), Throws::Discard), // sub_14139ddb0
    Replacement::draft_throws(0x14139de60, "unknown_14139de60", fn_14139de60 as *const (), Throws::Discard), // sub_14139de60
    Replacement::draft(0x14139dec0, "std_14139dec0", tpl_1::<0x14389a408> as *const ()), // std::_Func_impl_no_alloc<void (*)(void*),void,void*>::std::_Func_impl_no_alloc<void (*)(void*),void,void*>::vfunc_3
    Replacement::draft(0x14139ded0, "std_14139ded0", tpl_550::<0x143409000, 0x8> as *const ()), // std::_Func_impl_no_alloc<void (*)(void*),void,void*>::std::_Func_impl_no_alloc<void (*)(void*),void,void*>::vfunc_0
    Replacement::draft(0x14139def0, "std_14139def0", tpl_1::<0x14389a3e8> as *const ()), // std::_Func_impl_no_alloc<void* (*)(unsigned __int64),void*,unsigned __int64>::std::_Func_impl_no_alloc<void* (*)(unsigne
    Replacement::draft(0x14139df00, "std_14139df00", fn_14139df00 as *const ()), // std::_Func_impl_no_alloc<void* (*)(unsigned __int64),void*,unsigned __int64>::std::_Func_impl_no_alloc<void* (*)(unsigne
    Replacement::draft(0x14139df10, "std_14139df10", tpl_550::<0x143408f98, 0x8> as *const ()), // std::_Func_impl_no_alloc<void* (*)(unsigned __int64),void*,unsigned __int64>::std::_Func_impl_no_alloc<void* (*)(unsigne
    Replacement::draft_throws(0x14139df30, "unknown_14139df30", fn_14139df30 as *const (), Throws::Discard), // sub_14139df30
    Replacement::draft(0x14139e0a0, "unknown_14139e0a0", fn_14139e0a0 as *const ()), // sub_14139e0a0
    Replacement::draft(0x14139e210, "unknown_14139e210", fn_14139e210 as *const ()), // sub_14139e210
    Replacement::draft(0x14139e2b0, "unknown_14139e2b0", fn_14139e2b0 as *const ()), // sub_14139e2b0
    Replacement::draft(0x14139e2e0, "unknown_14139e2e0", tpl_22235::<0x18, 0x8> as *const ()), // sub_14139e2e0
    Replacement::draft(0x14139e300, "unknown_14139e300", fn_14139e300 as *const ()), // sub_14139e300
    Replacement::draft_throws(0x14139e3e0, "sparsehash_14139e3e0", fn_14139e3e0 as *const (), Throws::Discard), // sub_14139e3e0
    Replacement::draft_throws(0x14139e560, "sparsehash_14139e560", tpl_20204::<0x14139e660, 0x1410531f0, 0x140227190> as *const (), Throws::Discard), // sub_14139e560
    Replacement::draft_throws(0x14139e660, "sparsehash_14139e660", tpl_1271::<0x1401e2020, 0x143426fc8, 0x14139e7d0> as *const (), Throws::Discard), // sub_14139e660
    Replacement::draft_throws(0x14139e7d0, "sparsehash_14139e7d0", fn_14139e7d0 as *const (), Throws::Discard), // sub_14139e7d0
    Replacement::draft_throws(0x14139ea50, "sparsehash_14139ea50", fn_14139ea50 as *const (), Throws::Discard), // sub_14139ea50
    Replacement::draft_throws(0x14139ed40, "unknown_14139ed40", fn_14139ed40 as *const (), Throws::Discard), // sub_14139ed40
    Replacement::draft_throws(0x14139ee40, "unknown_14139ee40", fn_14139ee40 as *const (), Throws::Caught), // sub_14139ee40
    Replacement::draft(0x14139eed0, "unknown_14139eed0", fn_14139eed0 as *const ()), // sub_14139eed0
    Replacement::draft(0x14139eee0, "unknown_14139eee0", fn_14139eee0 as *const ()), // sub_14139eee0
    Replacement::draft(0x14139ef20, "unknown_14139ef20", fn_14139ef20 as *const ()), // sub_14139ef20
    Replacement::draft_throws(0x14139f000, "unknown_14139f000", fn_14139f000 as *const (), Throws::Discard), // sub_14139f000
    Replacement::draft_throws(0x14139f160, "unknown_14139f160", fn_14139f160 as *const (), Throws::Discard), // sub_14139f160
    Replacement::draft_throws(0x14139f520, "unknown_14139f520", fn_14139f520 as *const (), Throws::Caught), // sub_14139f520
    Replacement::draft(0x14139f5c0, "unknown_14139f5c0", fn_14139f5c0 as *const ()), // sub_14139f5c0
    Replacement::draft(0x14139f6f0, "unknown_14139f6f0", tpl_10127::<0x8, 0x1401de830, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14139f6f0
    Replacement::draft(0x14139f7c0, "unknown_14139f7c0", fn_14139f7c0 as *const ()), // sub_14139f7c0
    Replacement::draft(0x14139fa30, "unknown_14139fa30", tpl_10189::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14139f6f0> as *const ()), // sub_14139fa30
    Replacement::draft(0x14139fbc0, "unknown_14139fbc0", fn_14139fbc0 as *const ()), // sub_14139fbc0
    Replacement::draft(0x14139fd50, "unknown_14139fd50", fn_14139fd50 as *const ()), // sub_14139fd50
    Replacement::draft(0x1413a00c0, "unknown_1413a00c0", fn_1413a00c0 as *const ()), // sub_1413a00c0
    Replacement::draft(0x1413a0190, "unknown_1413a0190", tpl_24640::<0x1413a00c0, 0x1413a0440, 0x1413a0190, 0x1413a0880> as *const ()), // sub_1413a0190
    Replacement::draft(0x1413a0300, "unknown_1413a0300", fn_1413a0300 as *const ()), // sub_1413a0300
    Replacement::draft(0x1413a0880, "unknown_1413a0880", fn_1413a0880 as *const ()), // sub_1413a0880
    Replacement::draft(0x1413a0a00, "unknown_1413a0a00", fn_1413a0a00 as *const ()), // sub_1413a0a00
    Replacement::draft(0x1413a0af0, "unknown_1413a0af0", fn_1413a0af0 as *const ()), // sub_1413a0af0
    Replacement::draft(0x1413a0bb0, "unknown_1413a0bb0", fn_1413a0bb0 as *const ()), // sub_1413a0bb0
    Replacement::draft(0x1413a0d60, "unknown_1413a0d60", fn_1413a0d60 as *const ()), // sub_1413a0d60
    Replacement::draft_throws(0x1413a0fc0, "unknown_1413a0fc0", tpl_20284::<0x1413a1120, 0x14139f000> as *const (), Throws::Discard), // sub_1413a0fc0
    Replacement::draft(0x1413a1120, "unknown_1413a1120", fn_1413a1120 as *const ()), // sub_1413a1120
    Replacement::draft(0x1413a1350, "unknown_1413a1350", fn_1413a1350 as *const ()), // sub_1413a1350
    Replacement::draft(0x1413a14c0, "unknown_1413a14c0", fn_1413a14c0 as *const ()), // sub_1413a14c0
    Replacement::draft(0x1413a1690, "unknown_1413a1690", tpl_24506::<0x14134b7b0, 0x1413a0880> as *const ()), // sub_1413a1690
    Replacement::draft(0x1413a1a60, "sparsehash_1413a1a60", fn_1413a1a60 as *const ()), // sub_1413a1a60
    Replacement::draft_throws(0x1413a1e70, "sparsehash_1413a1e70", fn_1413a1e70 as *const (), Throws::Discard), // sub_1413a1e70
    Replacement::draft_throws(0x1413a2190, "sparsehash_1413a2190", fn_1413a2190 as *const (), Throws::Caught), // sub_1413a2190
    Replacement::draft_throws(0x1413a2250, "sparsehash_1413a2250", fn_1413a2250 as *const (), Throws::Caught), // sub_1413a2250
    Replacement::draft_throws(0x1413a2310, "sparsehash_1413a2310", fn_1413a2310 as *const (), Throws::Caught), // sub_1413a2310
    Replacement::draft_throws(0x1413a23d0, "sparsehash_1413a23d0", fn_1413a23d0 as *const (), Throws::Discard), // sub_1413a23d0
    Replacement::draft_throws(0x1413a2680, "sparsehash_1413a2680", fn_1413a2680 as *const (), Throws::Discard), // sub_1413a2680
    Replacement::draft_throws(0x1413a28c0, "sparsehash_1413a28c0", tpl_77984::<0x14139e660, 0x1410531f0, 0x140227190, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1413a2af2> as *const (), Throws::Discard), // sub_1413a28c0
    Replacement::draft(0x1413a2b00, "unknown_1413a2b00", fn_1413a2b00 as *const ()), // sub_1413a2b00
    Replacement::draft(0x1413a2ba0, "unknown_1413a2ba0", fn_1413a2ba0 as *const ()), // sub_1413a2ba0
    Replacement::draft(0x1413a2e30, "unknown_1413a2e30", tpl_381::<0x1413a1120, 0x1413a2ba0> as *const ()), // sub_1413a2e30
    Replacement::draft(0x1413a2f90, "unknown_1413a2f90", fn_1413a2f90 as *const ()), // sub_1413a2f90
    Replacement::draft(0x1413a31e0, "unknown_1413a31e0", fn_1413a31e0 as *const ()), // sub_1413a31e0
    Replacement::draft(0x1413a3560, "unknown_1413a3560", fn_1413a3560 as *const ()), // sub_1413a3560
    Replacement::draft(0x1413a3610, "unknown_1413a3610", tpl_381::<0x1413a3770, 0x1413a31e0> as *const ()), // sub_1413a3610
    Replacement::draft(0x1413a3770, "unknown_1413a3770", fn_1413a3770 as *const ()), // sub_1413a3770
    Replacement::draft(0x1413a7ce0, "unknown_1413a7ce0", fn_1413a7ce0 as *const ()), // sub_1413a7ce0
    Replacement::draft(0x1413a8240, "unknown_1413a8240", fn_1413a8240 as *const ()), // sub_1413a8240
    Replacement::draft(0x1413a8540, "unknown_1413a8540", tpl_23253::<0x8, 0x14240ab10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413a8540
    Replacement::draft(0x1413a8610, "unknown_1413a8610", fn_1413a8610 as *const ()), // sub_1413a8610
    Replacement::draft(0x1413a86f0, "unknown_1413a86f0", fn_1413a86f0 as *const ()), // sub_1413a86f0
    Replacement::draft(0x1413a89a0, "unknown_1413a89a0", tpl_24966::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143409de0, 0x8, 0xc, 0x14, 0x1401f3d90, 0x143409dc8> as *const ()), // sub_1413a89a0
    Replacement::draft(0x1413a8a90, "unknown_1413a8a90", fn_1413a8a90 as *const ()), // sub_1413a8a90
    Replacement::draft(0x1413a8ca0, "unknown_1413a8ca0", fn_1413a8ca0 as *const ()), // sub_1413a8ca0
    Replacement::draft(0x1413a8f40, "unknown_1413a8f40", fn_1413a8f40 as *const ()), // sub_1413a8f40
    Replacement::draft(0x1413a96f0, "unknown_1413a96f0", fn_1413a96f0 as *const ()), // sub_1413a96f0
    Replacement::draft(0x1413a98b0, "unknown_1413a98b0", fn_1413a98b0 as *const ()), // sub_1413a98b0
    Replacement::draft(0x1413a9a80, "unknown_1413a9a80", fn_1413a9a80 as *const ()), // sub_1413a9a80
    Replacement::draft(0x1413a9b60, "unknown_1413a9b60", tpl_20448::<0x1402174f0, 0x1413a9a80> as *const ()), // sub_1413a9b60
    Replacement::draft(0x1413a9c00, "unknown_1413a9c00", fn_1413a9c00 as *const ()), // sub_1413a9c00
    Replacement::draft(0x1413a9c80, "unknown_1413a9c80", fn_1413a9c80 as *const ()), // sub_1413a9c80
    Replacement::draft(0x1413a9dd0, "unknown_1413a9dd0", tpl_25254::<0x14340ed48, 0x8, 0x14340a0b8, 0x143409ff0, 0x142498f00> as *const ()), // sub_1413a9dd0
    Replacement::draft(0x1413a9e30, "unknown_1413a9e30", tpl_25254::<0x14340ed48, 0x8, 0x14340a0a0, 0x14340a040, 0x142498f00> as *const ()), // sub_1413a9e30
    Replacement::draft_throws(0x1413a9e70, "unknown_1413a9e70", fn_1413a9e70 as *const (), Throws::Caught), // sub_1413a9e70
    Replacement::draft_throws(0x1413a9ee0, "unknown_1413a9ee0", fn_1413a9ee0 as *const (), Throws::Caught), // sub_1413a9ee0
    Replacement::draft_throws(0x1413aa080, "unknown_1413aa080", fn_1413aa080 as *const (), Throws::Caught), // sub_1413aa080
    Replacement::draft_throws(0x1413aa270, "unknown_1413aa270", fn_1413aa270 as *const (), Throws::Caught), // sub_1413aa270
    Replacement::draft_throws(0x1413aa2e0, "unknown_1413aa2e0", fn_1413aa2e0 as *const (), Throws::Caught), // sub_1413aa2e0
    Replacement::draft_throws(0x1413aa360, "unknown_1413aa360", tpl_649::<0x143881fa0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413aa360
    Replacement::draft(0x1413aa540, "unknown_1413aa540", fn_1413aa540 as *const ()), // sub_1413aa540
    Replacement::draft(0x1413aa5f0, "unknown_1413aa5f0", fn_1413aa5f0 as *const ()), // sub_1413aa5f0
    Replacement::draft(0x1413aa6a0, "unknown_1413aa6a0", tpl_25261::<0x14183acda> as *const ()), // sub_1413aa6a0
    Replacement::draft(0x1413aa6c0, "net_1413aa6c0", tpl_25262::<0x20> as *const ()), // net::NativeCommPipeConnection::net::NativeCommPipeConnection::vfunc_10
    Replacement::draft(0x1413aa6d0, "net_1413aa6d0", tpl_25262::<0x10> as *const ()), // net::NativeCommPipeConnection::net::NativeCommPipeConnection::vfunc_12
    Replacement::draft_throws(0x1413aa6e0, "net_1413aa6e0", fn_1413aa6e0 as *const (), Throws::Discard), // net::NativeCommPipeConnection::net::NativeCommPipeConnection::vfunc_0
    Replacement::draft(0x1413ac0b0, "unknown_1413ac0b0", fn_1413ac0b0 as *const ()), // sub_1413ac0b0
    Replacement::draft(0x1413ad2d0, "unknown_1413ad2d0", tpl_170::<0x1413ad4c0> as *const ()), // sub_1413ad2d0
    Replacement::draft(0x1413ad2e0, "unknown_1413ad2e0", fn_1413ad2e0 as *const ()), // sub_1413ad2e0
    Replacement::draft_throws(0x1413ad300, "sparsehash_1413ad300", fn_1413ad300 as *const (), Throws::Discard), // sub_1413ad300
    Replacement::draft(0x1413ad4c0, "unknown_1413ad4c0", fn_1413ad4c0 as *const ()), // sub_1413ad4c0
    Replacement::draft(0x1413ad5b0, "unknown_1413ad5b0", fn_1413ad5b0 as *const ()), // sub_1413ad5b0
    Replacement::draft(0x1413b58f0, "unknown_1413b58f0", fn_1413b58f0 as *const ()), // sub_1413b58f0
    Replacement::draft_throws(0x1413b5a30, "unknown_1413b5a30", fn_1413b5a30 as *const (), Throws::Discard), // sub_1413b5a30
    Replacement::draft(0x1413b5b70, "unknown_1413b5b70", fn_1413b5b70 as *const ()), // sub_1413b5b70
    Replacement::draft(0x1413b5bd0, "unknown_1413b5bd0", fn_1413b5bd0 as *const ()), // sub_1413b5bd0
    Replacement::draft_throws(0x1413b5c00, "unknown_1413b5c00", fn_1413b5c00 as *const (), Throws::Caught), // sub_1413b5c00
    Replacement::draft_throws(0x1413b5ca0, "unknown_1413b5ca0", fn_1413b5ca0 as *const (), Throws::Caught), // sub_1413b5ca0
    Replacement::draft(0x1413b5d40, "unknown_1413b5d40", tpl_25383::<0x30> as *const ()), // sub_1413b5d40
    Replacement::draft(0x1413b5d60, "unknown_1413b5d60", tpl_25383::<0x28> as *const ()), // sub_1413b5d60
    Replacement::draft(0x1413b5d80, "unknown_1413b5d80", fn_1413b5d80 as *const ()), // sub_1413b5d80
    Replacement::draft(0x1413b5db0, "unknown_1413b5db0", fn_1413b5db0 as *const ()), // sub_1413b5db0
    Replacement::draft(0x1413b5df0, "unknown_1413b5df0", fn_1413b5df0 as *const ()), // sub_1413b5df0
    Replacement::draft_throws(0x1413b5ea0, "unknown_1413b5ea0", fn_1413b5ea0 as *const (), Throws::Discard), // sub_1413b5ea0
    Replacement::draft_throws(0x1413b5f00, "unknown_1413b5f00", fn_1413b5f00 as *const (), Throws::Discard), // sub_1413b5f00
    Replacement::draft_throws(0x1413b6030, "unknown_1413b6030", fn_1413b6030 as *const (), Throws::Discard), // sub_1413b6030
    Replacement::draft_throws(0x1413b6120, "unknown_1413b6120", fn_1413b6120 as *const (), Throws::Discard), // sub_1413b6120
    Replacement::draft(0x1413b6260, "unknown_1413b6260", fn_1413b6260 as *const ()), // sub_1413b6260
    Replacement::draft_throws(0x1413b62f0, "unknown_1413b62f0", fn_1413b62f0 as *const (), Throws::Discard), // sub_1413b62f0
    Replacement::draft_throws(0x1413b63c0, "unknown_1413b63c0", fn_1413b63c0 as *const (), Throws::Discard), // sub_1413b63c0
    Replacement::draft_throws(0x1413b6610, "std_1413b6610", tpl_373::<0x14340b5d8, 0x142645890, 0x1413b6621> as *const (), Throws::Discard), // sub_1413b6610
    Replacement::draft_throws(0x1413b6630, "unknown_1413b6630", fn_1413b6630 as *const (), Throws::Discard), // sub_1413b6630
    Replacement::draft_throws(0x1413b66f0, "luau_1413b66f0", fn_1413b66f0 as *const (), Throws::Discard), // sub_1413b66f0
    Replacement::draft_throws(0x1413b67d0, "unknown_1413b67d0", tpl_25391::<0x14340c7c8, 0x142116b30, 0x1413b67e0> as *const (), Throws::Discard), // sub_1413b67d0
    Replacement::draft_throws(0x1413b67f0, "unknown_1413b67f0", fn_1413b67f0 as *const (), Throws::Discard), // sub_1413b67f0
    Replacement::draft_throws(0x1413b69b0, "unknown_1413b69b0", fn_1413b69b0 as *const (), Throws::Discard), // sub_1413b69b0
    Replacement::draft_throws(0x1413b6a50, "unknown_1413b6a50", fn_1413b6a50 as *const (), Throws::Discard), // sub_1413b6a50
    Replacement::draft_throws(0x1413b6b00, "unknown_1413b6b00", fn_1413b6b00 as *const (), Throws::Discard), // sub_1413b6b00
    Replacement::draft_throws(0x1413b6bc0, "unknown_1413b6bc0", fn_1413b6bc0 as *const (), Throws::Discard), // sub_1413b6bc0
    Replacement::draft_throws(0x1413b6cc0, "unknown_1413b6cc0", fn_1413b6cc0 as *const (), Throws::Discard), // sub_1413b6cc0
    Replacement::draft_throws(0x1413b6d60, "unknown_1413b6d60", fn_1413b6d60 as *const (), Throws::Discard), // sub_1413b6d60
    Replacement::draft_throws(0x1413b6e00, "unknown_1413b6e00", fn_1413b6e00 as *const (), Throws::Discard), // sub_1413b6e00
    Replacement::draft_throws(0x1413b6eb0, "unknown_1413b6eb0", fn_1413b6eb0 as *const (), Throws::Discard), // sub_1413b6eb0
    Replacement::draft_throws(0x1413b6f20, "unknown_1413b6f20", fn_1413b6f20 as *const (), Throws::Discard), // sub_1413b6f20
    Replacement::draft_throws(0x1413b6fa0, "unknown_1413b6fa0", fn_1413b6fa0 as *const (), Throws::Discard), // sub_1413b6fa0
    Replacement::draft_throws(0x1413b7030, "unknown_1413b7030", fn_1413b7030 as *const (), Throws::Discard), // sub_1413b7030
    Replacement::draft_throws(0x1413b7100, "unknown_1413b7100", fn_1413b7100 as *const (), Throws::Discard), // sub_1413b7100
    Replacement::draft_throws(0x1413b71f0, "unknown_1413b71f0", fn_1413b71f0 as *const (), Throws::Discard), // sub_1413b71f0
    Replacement::draft_throws(0x1413b7300, "unknown_1413b7300", fn_1413b7300 as *const (), Throws::Discard), // sub_1413b7300
    Replacement::draft_throws(0x1413b7430, "unknown_1413b7430", fn_1413b7430 as *const (), Throws::Discard), // sub_1413b7430
    Replacement::draft_throws(0x1413b7570, "unknown_1413b7570", fn_1413b7570 as *const (), Throws::Discard), // sub_1413b7570
    Replacement::draft_throws(0x1413b76c0, "unknown_1413b76c0", fn_1413b76c0 as *const (), Throws::Discard), // sub_1413b76c0
    Replacement::draft_throws(0x1413b7790, "unknown_1413b7790", fn_1413b7790 as *const (), Throws::Discard), // sub_1413b7790
    Replacement::draft_throws(0x1413b7880, "unknown_1413b7880", fn_1413b7880 as *const (), Throws::Discard), // sub_1413b7880
    Replacement::draft_throws(0x1413b7980, "unknown_1413b7980", fn_1413b7980 as *const (), Throws::Discard), // sub_1413b7980
    Replacement::draft_throws(0x1413b7a30, "unknown_1413b7a30", fn_1413b7a30 as *const (), Throws::Discard), // sub_1413b7a30
    Replacement::draft_throws(0x1413b7af0, "unknown_1413b7af0", fn_1413b7af0 as *const (), Throws::Discard), // sub_1413b7af0
    Replacement::draft_throws(0x1413b7bc0, "unknown_1413b7bc0", fn_1413b7bc0 as *const (), Throws::Discard), // sub_1413b7bc0
    Replacement::draft_throws(0x1413b7d60, "unknown_1413b7d60", fn_1413b7d60 as *const (), Throws::Discard), // sub_1413b7d60
    Replacement::draft_throws(0x1413b7f70, "unknown_1413b7f70", fn_1413b7f70 as *const (), Throws::Discard), // sub_1413b7f70
    Replacement::draft_throws(0x1413b8180, "unknown_1413b8180", fn_1413b8180 as *const (), Throws::Discard), // sub_1413b8180
    Replacement::draft_throws(0x1413b8390, "unknown_1413b8390", fn_1413b8390 as *const (), Throws::Discard), // sub_1413b8390
    Replacement::draft_throws(0x1413b85f0, "unknown_1413b85f0", fn_1413b85f0 as *const (), Throws::Discard), // sub_1413b85f0
    Replacement::draft_throws(0x1413b87b0, "unknown_1413b87b0", fn_1413b87b0 as *const (), Throws::Discard), // sub_1413b87b0
    Replacement::draft_throws(0x1413b8960, "unknown_1413b8960", fn_1413b8960 as *const (), Throws::Discard), // sub_1413b8960
    Replacement::draft_throws(0x1413b8a30, "unknown_1413b8a30", fn_1413b8a30 as *const (), Throws::Discard), // sub_1413b8a30
    Replacement::draft(0x1413b8b00, "unknown_1413b8b00", fn_1413b8b00 as *const ()), // sub_1413b8b00
    Replacement::draft_throws(0x1413b8de0, "unknown_1413b8de0", fn_1413b8de0 as *const (), Throws::Discard), // sub_1413b8de0
    Replacement::draft_throws(0x1413b8f00, "unknown_1413b8f00", fn_1413b8f00 as *const (), Throws::Discard), // sub_1413b8f00
    Replacement::draft_throws(0x1413b9150, "unknown_1413b9150", fn_1413b9150 as *const (), Throws::Discard), // sub_1413b9150
    Replacement::draft_throws(0x1413b92d0, "unknown_1413b92d0", fn_1413b92d0 as *const (), Throws::Discard), // sub_1413b92d0
    Replacement::draft_throws(0x1413b9620, "unknown_1413b9620", fn_1413b9620 as *const (), Throws::Discard), // sub_1413b9620
    Replacement::draft(0x1413b9c60, "unknown_1413b9c60", fn_1413b9c60 as *const ()), // sub_1413b9c60
    Replacement::draft(0x1413b9da0, "unknown_1413b9da0", fn_1413b9da0 as *const ()), // sub_1413b9da0
    Replacement::draft(0x1413b9dd0, "unknown_1413b9dd0", tpl_365::<0x1413b9f00> as *const ()), // sub_1413b9dd0
    Replacement::draft(0x1413b9de0, "unknown_1413b9de0", tpl_365::<0x1411d5ee0> as *const ()), // sub_1413b9de0
    Replacement::draft(0x1413b9df0, "unknown_1413b9df0", fn_1413b9df0 as *const ()), // sub_1413b9df0
    Replacement::draft(0x1413b9e70, "unknown_1413b9e70", fn_1413b9e70 as *const ()), // sub_1413b9e70
    Replacement::draft(0x1413b9f00, "unknown_1413b9f00", fn_1413b9f00 as *const ()), // sub_1413b9f00
    Replacement::draft(0x1413b9fc0, "global_1413b9fc0", fn_1413b9fc0 as *const ()), // deallocate
    Replacement::draft(0x1413ba010, "unknown_1413ba010", fn_1413ba010 as *const ()), // sub_1413ba010
    Replacement::draft(0x1413ba060, "unknown_1413ba060", fn_1413ba060 as *const ()), // sub_1413ba060
    Replacement::draft(0x1413ba1b0, "unknown_1413ba1b0", fn_1413ba1b0 as *const ()), // sub_1413ba1b0
    Replacement::draft(0x1413ba1d0, "unknown_1413ba1d0", fn_1413ba1d0 as *const ()), // sub_1413ba1d0
    Replacement::draft(0x1413ba210, "unknown_1413ba210", fn_1413ba210 as *const ()), // sub_1413ba210
    Replacement::draft(0x1413ba270, "unknown_1413ba270", fn_1413ba270 as *const ()), // sub_1413ba270
    Replacement::draft(0x1413ba2e0, "unknown_1413ba2e0", fn_1413ba2e0 as *const ()), // sub_1413ba2e0
    Replacement::draft(0x1413ba320, "unknown_1413ba320", tpl_22001::<0x143871520, 0x143871538, 0x143871548, 0x143871530, 0x1413ba4a0, 0x1413ba790> as *const ()), // sub_1413ba320
    Replacement::draft(0x1413ba4a0, "unknown_1413ba4a0", fn_1413ba4a0 as *const ()), // sub_1413ba4a0
    Replacement::draft(0x1413ba790, "unknown_1413ba790", fn_1413ba790 as *const ()), // sub_1413ba790
    Replacement::draft(0x1413baab0, "unknown_1413baab0", fn_1413baab0 as *const ()), // sub_1413baab0
    Replacement::draft(0x1413bac40, "unknown_1413bac40", fn_1413bac40 as *const ()), // sub_1413bac40
    Replacement::draft_throws(0x1413bacb0, "unknown_1413bacb0", fn_1413bacb0 as *const (), Throws::Caught), // sub_1413bacb0
    Replacement::draft(0x1413bad80, "unknown_1413bad80", fn_1413bad80 as *const ()), // sub_1413bad80
    Replacement::draft_throws(0x1413bae00, "unknown_1413bae00", tpl_3734::<0x8, 0x1413bb270, 0x14241c190> as *const (), Throws::Discard), // sub_1413bae00
    Replacement::draft(0x1413bae30, "unknown_1413bae30", tpl_25426::<0x10, 0x1423ef2b0, 0xc, 0x1426469d0> as *const ()), // sub_1413bae30
    Replacement::draft(0x1413bae80, "unknown_1413bae80", fn_1413bae80 as *const ()), // sub_1413bae80
    Replacement::draft(0x1413baf20, "unknown_1413baf20", fn_1413baf20 as *const ()), // sub_1413baf20
    Replacement::draft(0x1413bafb0, "unknown_1413bafb0", fn_1413bafb0 as *const ()), // sub_1413bafb0
    Replacement::draft_throws(0x1413bb090, "unknown_1413bb090", tpl_1032::<0x14241c0a0, 0x14020b8e0, 0x14240ab10, 0x14241c190, 0x14240aa00> as *const (), Throws::Discard), // sub_1413bb090
    Replacement::draft(0x1413bb1f0, "unknown_1413bb1f0", fn_1413bb1f0 as *const ()), // sub_1413bb1f0
    Replacement::draft_throws(0x1413bb270, "unknown_1413bb270", tpl_19083::<0x19, 0x10, 0x1413bb270, 0x20, 0x1413bad80, 0x14241c190> as *const (), Throws::Discard), // sub_1413bb270
    Replacement::draft(0x1413bb2f0, "unknown_1413bb2f0", fn_1413bb2f0 as *const ()), // sub_1413bb2f0
    Replacement::draft(0x1413bb5b0, "unknown_1413bb5b0", fn_1413bb5b0 as *const ()), // sub_1413bb5b0
    Replacement::draft(0x1413bb7e0, "unknown_1413bb7e0", fn_1413bb7e0 as *const ()), // sub_1413bb7e0
    Replacement::draft(0x1413bb850, "unknown_1413bb850", fn_1413bb850 as *const ()), // sub_1413bb850
    Replacement::draft(0x1413bb8a0, "unknown_1413bb8a0", fn_1413bb8a0 as *const ()), // sub_1413bb8a0
    Replacement::draft(0x1413bb900, "unknown_1413bb900", fn_1413bb900 as *const ()), // sub_1413bb900
    Replacement::draft_throws(0x1413bb930, "unknown_1413bb930", fn_1413bb930 as *const (), Throws::Caught), // sub_1413bb930
    Replacement::draft_throws(0x1413c1770, "unknown_1413c1770", fn_1413c1770 as *const (), Throws::Caught), // sub_1413c1770
    Replacement::draft_throws(0x1413c1a50, "unknown_1413c1a50", fn_1413c1a50 as *const (), Throws::Caught), // sub_1413c1a50
    Replacement::draft_throws(0x1413c1b80, "unknown_1413c1b80", fn_1413c1b80 as *const (), Throws::Caught), // sub_1413c1b80
    Replacement::draft_throws(0x1413c1c30, "unknown_1413c1c30", fn_1413c1c30 as *const (), Throws::Caught), // sub_1413c1c30
    Replacement::draft_throws(0x1413c1ce0, "unknown_1413c1ce0", fn_1413c1ce0 as *const (), Throws::Caught), // sub_1413c1ce0
    Replacement::draft_throws(0x1413c1d90, "unknown_1413c1d90", fn_1413c1d90 as *const (), Throws::Caught), // sub_1413c1d90
    Replacement::draft_throws(0x1413c1e40, "unknown_1413c1e40", fn_1413c1e40 as *const (), Throws::Caught), // sub_1413c1e40
    Replacement::draft_throws(0x1413c1fa0, "unknown_1413c1fa0", fn_1413c1fa0 as *const (), Throws::Caught), // sub_1413c1fa0
    Replacement::draft_throws(0x1413c2050, "unknown_1413c2050", fn_1413c2050 as *const (), Throws::Caught), // sub_1413c2050
    Replacement::draft_throws(0x1413c2210, "unknown_1413c2210", fn_1413c2210 as *const (), Throws::Caught), // sub_1413c2210
    Replacement::draft_throws(0x1413c2280, "unknown_1413c2280", fn_1413c2280 as *const (), Throws::Caught), // sub_1413c2280
    Replacement::draft_throws(0x1413c2400, "unknown_1413c2400", fn_1413c2400 as *const (), Throws::Caught), // sub_1413c2400
    Replacement::draft_throws(0x1413c2610, "unknown_1413c2610", fn_1413c2610 as *const (), Throws::Caught), // sub_1413c2610
    Replacement::draft_throws(0x1413c2880, "unknown_1413c2880", fn_1413c2880 as *const (), Throws::Caught), // sub_1413c2880
    Replacement::draft_throws(0x1413c28f0, "unknown_1413c28f0", fn_1413c28f0 as *const (), Throws::Caught), // sub_1413c28f0
    Replacement::draft_throws(0x1413c2960, "unknown_1413c2960", fn_1413c2960 as *const (), Throws::Caught), // sub_1413c2960
    Replacement::draft_throws(0x1413c2a90, "unknown_1413c2a90", fn_1413c2a90 as *const (), Throws::Caught), // sub_1413c2a90
    Replacement::draft_throws(0x1413c2ba0, "unknown_1413c2ba0", fn_1413c2ba0 as *const (), Throws::Caught), // sub_1413c2ba0
    Replacement::draft_throws(0x1413c2cc0, "unknown_1413c2cc0", fn_1413c2cc0 as *const (), Throws::Caught), // sub_1413c2cc0
    Replacement::draft_throws(0x1413c2dc0, "unknown_1413c2dc0", fn_1413c2dc0 as *const (), Throws::Caught), // sub_1413c2dc0
    Replacement::draft_throws(0x1413c2ec0, "unknown_1413c2ec0", fn_1413c2ec0 as *const (), Throws::Caught), // sub_1413c2ec0
    Replacement::draft_throws(0x1413c2fe0, "unknown_1413c2fe0", fn_1413c2fe0 as *const (), Throws::Caught), // sub_1413c2fe0
    Replacement::draft_throws(0x1413c3120, "unknown_1413c3120", fn_1413c3120 as *const (), Throws::Caught), // sub_1413c3120
    Replacement::draft(0x1413c5010, "rapidjson_1413c5010", fn_1413c5010 as *const ()), // sub_1413c5010
    Replacement::draft(0x1413c5070, "rapidjson_1413c5070", fn_1413c5070 as *const ()), // sub_1413c5070
    Replacement::draft(0x1413c50e0, "rapidjson_1413c50e0", fn_1413c50e0 as *const ()), // sub_1413c50e0
    Replacement::draft(0x1413c5140, "rapidjson_1413c5140", fn_1413c5140 as *const ()), // sub_1413c5140
    Replacement::draft(0x1413c53b0, "rapidjson_1413c53b0", fn_1413c53b0 as *const ()), // sub_1413c53b0
    Replacement::draft(0x1413c5a00, "rapidjson_1413c5a00", fn_1413c5a00 as *const ()), // sub_1413c5a00
    Replacement::draft(0x1413c5d70, "rapidjson_1413c5d70", fn_1413c5d70 as *const ()), // sub_1413c5d70
    Replacement::draft(0x1413c5f60, "rapidjson_1413c5f60", fn_1413c5f60 as *const ()), // sub_1413c5f60
    Replacement::draft(0x1413c5ff0, "rapidjson_1413c5ff0", fn_1413c5ff0 as *const ()), // sub_1413c5ff0
    Replacement::draft(0x1413c61b0, "rapidjson_1413c61b0", fn_1413c61b0 as *const ()), // sub_1413c61b0
    Replacement::draft(0x1413c6250, "rapidjson_1413c6250", fn_1413c6250 as *const ()), // sub_1413c6250
    Replacement::draft(0x1413c6280, "rapidjson_1413c6280", fn_1413c6280 as *const ()), // sub_1413c6280
    Replacement::draft_throws(0x1413c6300, "rapidjson_1413c6300", fn_1413c6300 as *const (), Throws::Caught), // sub_1413c6300
    Replacement::draft_throws(0x1413c6470, "rapidjson_1413c6470", fn_1413c6470 as *const (), Throws::Caught), // sub_1413c6470
    Replacement::draft_throws(0x1413c65c0, "rapidjson_1413c65c0", fn_1413c65c0 as *const (), Throws::Caught), // sub_1413c65c0
    Replacement::draft_throws(0x1413c6630, "rapidjson_1413c6630", fn_1413c6630 as *const (), Throws::Caught), // sub_1413c6630
    Replacement::draft_throws(0x1413c67a0, "rapidjson_1413c67a0", fn_1413c67a0 as *const (), Throws::Caught), // sub_1413c67a0
    Replacement::draft_throws(0x1413c6810, "rapidjson_1413c6810", fn_1413c6810 as *const (), Throws::Discard), // sub_1413c6810
    Replacement::draft_throws(0x1413c6830, "rapidjson_1413c6830", fn_1413c6830 as *const (), Throws::Discard), // sub_1413c6830
    Replacement::draft_throws(0x1413c6a80, "rapidjson_1413c6a80", fn_1413c6a80 as *const (), Throws::Discard), // sub_1413c6a80
    Replacement::draft_throws(0x1413c6f60, "rapidjson_1413c6f60", fn_1413c6f60 as *const (), Throws::Discard), // sub_1413c6f60
    Replacement::draft(0x1413c7440, "rapidjson_1413c7440", fn_1413c7440 as *const ()), // sub_1413c7440
    Replacement::draft_throws(0x1413c7450, "rapidjson_1413c7450", fn_1413c7450 as *const (), Throws::Discard), // sub_1413c7450
    Replacement::draft_throws(0x1413c74b0, "rapidjson_1413c74b0", fn_1413c74b0 as *const (), Throws::Discard), // sub_1413c74b0
    Replacement::draft_throws(0x1413c7590, "rapidjson_1413c7590", fn_1413c7590 as *const (), Throws::Discard), // sub_1413c7590
    Replacement::draft_throws(0x1413c76a0, "rapidjson_1413c76a0", fn_1413c76a0 as *const (), Throws::Discard), // sub_1413c76a0
    Replacement::draft_throws(0x1413c7730, "rapidjson_1413c7730", fn_1413c7730 as *const (), Throws::Discard), // sub_1413c7730
    Replacement::draft_throws(0x1413c77a0, "rapidjson_1413c77a0", fn_1413c77a0 as *const (), Throws::Discard), // sub_1413c77a0
    Replacement::draft_throws(0x1413c7830, "rapidjson_1413c7830", fn_1413c7830 as *const (), Throws::Discard), // sub_1413c7830
    Replacement::draft(0x1413c78b0, "rapidjson_1413c78b0", fn_1413c78b0 as *const ()), // sub_1413c78b0
    Replacement::draft(0x1413c7910, "rapidjson_1413c7910", fn_1413c7910 as *const ()), // sub_1413c7910
    Replacement::draft(0x1413c79f0, "rapidjson_1413c79f0", fn_1413c79f0 as *const ()), // sub_1413c79f0
    Replacement::draft(0x1413c7b00, "rapidjson_1413c7b00", fn_1413c7b00 as *const ()), // sub_1413c7b00
    Replacement::draft(0x1413c7b90, "rapidjson_1413c7b90", fn_1413c7b90 as *const ()), // sub_1413c7b90
    Replacement::draft(0x1413c7c00, "rapidjson_1413c7c00", fn_1413c7c00 as *const ()), // sub_1413c7c00
    Replacement::draft(0x1413c7c90, "rapidjson_1413c7c90", fn_1413c7c90 as *const ()), // sub_1413c7c90
    Replacement::draft_throws(0x1413c7d10, "rapidjson_1413c7d10", fn_1413c7d10 as *const (), Throws::Discard), // sub_1413c7d10
    Replacement::draft_throws(0x1413c8210, "rapidjson_1413c8210", fn_1413c8210 as *const (), Throws::Discard), // sub_1413c8210
    Replacement::draft(0x1413c86f0, "rapidjson_1413c86f0", fn_1413c86f0 as *const ()), // sub_1413c86f0
    Replacement::draft_throws(0x1413c87a0, "rapidjson_1413c87a0", fn_1413c87a0 as *const (), Throws::Discard), // sub_1413c87a0
    Replacement::draft_throws(0x1413c8c00, "rapidjson_1413c8c00", fn_1413c8c00 as *const (), Throws::Discard), // sub_1413c8c00
    Replacement::draft_throws(0x1413c93e0, "rapidjson_1413c93e0", fn_1413c93e0 as *const (), Throws::Discard), // sub_1413c93e0
    Replacement::draft_throws(0x1413c9680, "rapidjson_1413c9680", fn_1413c9680 as *const (), Throws::Discard), // sub_1413c9680
    Replacement::draft_throws(0x1413c9860, "rapidjson_1413c9860", fn_1413c9860 as *const (), Throws::Discard), // sub_1413c9860
    Replacement::draft_throws(0x1413c9e80, "rapidjson_1413c9e80", fn_1413c9e80 as *const (), Throws::Discard), // sub_1413c9e80
    Replacement::draft_throws(0x1413c9eb0, "rapidjson_1413c9eb0", fn_1413c9eb0 as *const (), Throws::Discard), // sub_1413c9eb0
    Replacement::draft_throws(0x1413c9f00, "rapidjson_1413c9f00", fn_1413c9f00 as *const (), Throws::Discard), // sub_1413c9f00
    Replacement::draft_throws(0x1413c9f30, "rapidjson_1413c9f30", fn_1413c9f30 as *const (), Throws::Discard), // sub_1413c9f30
    Replacement::draft_throws(0x1413c9f90, "rapidjson_1413c9f90", fn_1413c9f90 as *const (), Throws::Discard), // sub_1413c9f90
    Replacement::draft_throws(0x1413ca020, "rapidjson_1413ca020", fn_1413ca020 as *const (), Throws::Discard), // sub_1413ca020
    Replacement::draft_throws(0x1413ca070, "rapidjson_1413ca070", fn_1413ca070 as *const (), Throws::Discard), // sub_1413ca070
    Replacement::draft_throws(0x1413ca0d0, "rapidjson_1413ca0d0", fn_1413ca0d0 as *const (), Throws::Discard), // sub_1413ca0d0
    Replacement::draft_throws(0x1413ca130, "rapidjson_1413ca130", fn_1413ca130 as *const (), Throws::Discard), // sub_1413ca130
    Replacement::draft_throws(0x1413ca180, "rapidjson_1413ca180", fn_1413ca180 as *const (), Throws::Discard), // sub_1413ca180
    Replacement::draft_throws(0x1413ca1c0, "rapidjson_1413ca1c0", fn_1413ca1c0 as *const (), Throws::Discard), // sub_1413ca1c0
    Replacement::draft(0x1413ca2a0, "rapidjson_1413ca2a0", fn_1413ca2a0 as *const ()), // sub_1413ca2a0
    Replacement::draft_throws(0x1413ca3a0, "rapidjson_1413ca3a0", fn_1413ca3a0 as *const (), Throws::Discard), // sub_1413ca3a0
    Replacement::draft(0x1413ca500, "rapidjson_1413ca500", fn_1413ca500 as *const ()), // sub_1413ca500
    Replacement::draft(0x1413ca590, "rapidjson_1413ca590", fn_1413ca590 as *const ()), // sub_1413ca590
    Replacement::draft_throws(0x1413ca630, "unknown_1413ca630", fn_1413ca630 as *const (), Throws::Discard), // sub_1413ca630
    Replacement::draft_throws(0x1413ca700, "unknown_1413ca700", fn_1413ca700 as *const (), Throws::Discard), // sub_1413ca700
    Replacement::draft_throws(0x1413ca770, "unknown_1413ca770", fn_1413ca770 as *const (), Throws::Caught), // sub_1413ca770
    Replacement::draft_throws(0x1413ca8d0, "unknown_1413ca8d0", fn_1413ca8d0 as *const (), Throws::Caught), // sub_1413ca8d0
    Replacement::draft_throws(0x1413caa30, "unknown_1413caa30", fn_1413caa30 as *const (), Throws::Caught), // sub_1413caa30
    Replacement::draft_throws(0x1413cabb0, "unknown_1413cabb0", fn_1413cabb0 as *const (), Throws::Caught), // sub_1413cabb0
    Replacement::draft_throws(0x1413cad10, "unknown_1413cad10", fn_1413cad10 as *const (), Throws::Caught), // sub_1413cad10
    Replacement::draft_throws(0x1413cae90, "unknown_1413cae90", fn_1413cae90 as *const (), Throws::Caught), // sub_1413cae90
    Replacement::draft_throws(0x1413caff0, "unknown_1413caff0", fn_1413caff0 as *const (), Throws::Caught), // sub_1413caff0
    Replacement::draft_throws(0x1413cb150, "unknown_1413cb150", fn_1413cb150 as *const (), Throws::Caught), // sub_1413cb150
    Replacement::draft_throws(0x1413cb2b0, "unknown_1413cb2b0", fn_1413cb2b0 as *const (), Throws::Caught), // sub_1413cb2b0
    Replacement::draft_throws(0x1413cb440, "unknown_1413cb440", fn_1413cb440 as *const (), Throws::Caught), // sub_1413cb440
    Replacement::draft(0x1413cb4c0, "unknown_1413cb4c0", fn_1413cb4c0 as *const ()), // sub_1413cb4c0
    Replacement::draft_throws(0x1413cb500, "unknown_1413cb500", tpl_22028::<0x1424076e0, 0x142442de0> as *const (), Throws::Discard), // sub_1413cb500
    Replacement::draft_throws(0x1413cb530, "unknown_1413cb530", fn_1413cb530 as *const (), Throws::Caught), // sub_1413cb530
    Replacement::draft(0x1413cb6d0, "unknown_1413cb6d0", fn_1413cb6d0 as *const ()), // sub_1413cb6d0
    Replacement::draft(0x1413cc020, "unknown_1413cc020", tpl_6580::<0x14340ed48, 0x8, 0x14340e4f8, 0x14340e4d0, 0x142498f00> as *const ()), // sub_1413cc020
    Replacement::draft_throws(0x1413cc060, "unknown_1413cc060", fn_1413cc060 as *const (), Throws::Caught), // sub_1413cc060
    Replacement::draft(0x1413cc0d0, "unknown_1413cc0d0", tpl_3511::<0x14340ed48, 0x8, 0x14340e510, 0x14340e598, 0x142498f00> as *const ()), // sub_1413cc0d0
    Replacement::draft(0x1413cc130, "g_1413cc130", fn_1413cc130 as *const ()), // g::Image::g::Image::vfunc_0
    Replacement::draft_throws(0x1413cc1a0, "unknown_1413cc1a0", fn_1413cc1a0 as *const (), Throws::Caught), // sub_1413cc1a0
    Replacement::draft(0x1413cc210, "unknown_1413cc210", fn_1413cc210 as *const ()), // sub_1413cc210
    Replacement::draft(0x1413cc2b0, "unknown_1413cc2b0", fn_1413cc2b0 as *const ()), // sub_1413cc2b0
    Replacement::draft(0x1413cc360, "unknown_1413cc360", fn_1413cc360 as *const ()), // sub_1413cc360
    Replacement::draft_throws(0x1413cc4e0, "unknown_1413cc4e0", fn_1413cc4e0 as *const (), Throws::Caught), // sub_1413cc4e0
    Replacement::draft(0x1413cc620, "unknown_1413cc620", fn_1413cc620 as *const ()), // sub_1413cc620
    Replacement::draft_throws(0x1413cc730, "unknown_1413cc730", fn_1413cc730 as *const (), Throws::Caught), // sub_1413cc730
    Replacement::draft(0x1413cc8b0, "unknown_1413cc8b0", fn_1413cc8b0 as *const ()), // sub_1413cc8b0
    Replacement::draft_throws(0x1413cc950, "unknown_1413cc950", fn_1413cc950 as *const (), Throws::Caught), // sub_1413cc950
    Replacement::draft_throws(0x1413cca70, "unknown_1413cca70", fn_1413cca70 as *const (), Throws::Caught), // sub_1413cca70
    Replacement::draft_throws(0x1413ccb90, "unknown_1413ccb90", fn_1413ccb90 as *const (), Throws::Caught), // sub_1413ccb90
    Replacement::draft(0x1413cccb0, "unknown_1413cccb0", fn_1413cccb0 as *const ()), // sub_1413cccb0
    Replacement::draft(0x1413ccd90, "unknown_1413ccd90", fn_1413ccd90 as *const ()), // sub_1413ccd90
    Replacement::draft(0x1413cce50, "unknown_1413cce50", fn_1413cce50 as *const ()), // sub_1413cce50
    Replacement::draft_throws(0x1413ccf20, "unknown_1413ccf20", fn_1413ccf20 as *const (), Throws::Discard), // sub_1413ccf20
    Replacement::draft_throws(0x1413cd010, "unknown_1413cd010", fn_1413cd010 as *const (), Throws::Discard), // sub_1413cd010
    Replacement::draft_throws(0x1413cd140, "unknown_1413cd140", tpl_2502::<0x20, 0x8, 0x1413cd9c0> as *const (), Throws::Discard), // sub_1413cd140
    Replacement::draft_throws(0x1413cd190, "unknown_1413cd190", fn_1413cd190 as *const (), Throws::Caught), // sub_1413cd190
    Replacement::draft_throws(0x1413cd470, "unknown_1413cd470", fn_1413cd470 as *const (), Throws::Caught), // sub_1413cd470
    Replacement::draft_throws(0x1413cd720, "unknown_1413cd720", fn_1413cd720 as *const (), Throws::Caught), // sub_1413cd720
    Replacement::draft_throws(0x1413cd9c0, "unknown_1413cd9c0", fn_1413cd9c0 as *const (), Throws::Caught), // sub_1413cd9c0
    Replacement::draft_throws(0x1413cda80, "unknown_1413cda80", fn_1413cda80 as *const (), Throws::Caught), // sub_1413cda80
    Replacement::draft_throws(0x1413cdf50, "unknown_1413cdf50", fn_1413cdf50 as *const (), Throws::Caught), // sub_1413cdf50
    Replacement::draft_throws(0x1413ce3d0, "unknown_1413ce3d0", fn_1413ce3d0 as *const (), Throws::Caught), // sub_1413ce3d0
    Replacement::draft_throws(0x1413ce850, "unknown_1413ce850", fn_1413ce850 as *const (), Throws::Discard), // sub_1413ce850
    Replacement::draft_throws(0x1413ce900, "unknown_1413ce900", fn_1413ce900 as *const (), Throws::Discard), // sub_1413ce900
    Replacement::draft_throws(0x1413ce960, "unknown_1413ce960", fn_1413ce960 as *const (), Throws::Discard), // sub_1413ce960
    Replacement::draft(0x1413ce9b0, "unknown_1413ce9b0", fn_1413ce9b0 as *const ()), // sub_1413ce9b0
    Replacement::draft(0x1413cea10, "unknown_1413cea10", fn_1413cea10 as *const ()), // sub_1413cea10
    Replacement::draft(0x1413cea60, "unknown_1413cea60", fn_1413cea60 as *const ()), // sub_1413cea60
    Replacement::draft_throws(0x1413ced60, "unknown_1413ced60", fn_1413ced60 as *const (), Throws::Caught), // sub_1413ced60
    Replacement::draft(0x1413cef10, "unknown_1413cef10", fn_1413cef10 as *const ()), // sub_1413cef10
    Replacement::draft(0x1413cef60, "unknown_1413cef60", fn_1413cef60 as *const ()), // sub_1413cef60
    Replacement::draft_throws(0x1413cf160, "unknown_1413cf160", tpl_21456::<0x1401e2020, 0x143426fc8, 0x1413cf2c0> as *const (), Throws::Discard), // sub_1413cf160
    Replacement::draft_throws(0x1413cf2c0, "unknown_1413cf2c0", fn_1413cf2c0 as *const (), Throws::Discard), // sub_1413cf2c0
    Replacement::draft_throws(0x1413cf570, "sparsehash_1413cf570", tpl_79801::<0x141172a00, 0x1413cf7f0, 0x14307aa70, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1413cf7c6, 0x1413cf7e9> as *const (), Throws::Discard), // sub_1413cf570
    Replacement::draft_throws(0x1413cf7f0, "sparsehash_1413cf7f0", tpl_79803::<0x1413cf160, 0x1411731c0, 0x141172650, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1413cfa22> as *const (), Throws::Discard), // sub_1413cf7f0
    Replacement::draft(0x1413cfa30, "sparsehash_1413cfa30", fn_1413cfa30 as *const ()), // sub_1413cfa30
    Replacement::draft_throws(0x1413cfbe0, "sparsehash_1413cfbe0", tpl_20204::<0x140227220, 0x140fbf1e0, 0x140227190> as *const (), Throws::Discard), // sub_1413cfbe0
    Replacement::draft(0x1413cfe90, "unknown_1413cfe90", tpl_422::<0x68, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413cfe90
    Replacement::draft(0x1413cff20, "unknown_1413cff20", fn_1413cff20 as *const ()), // sub_1413cff20
    Replacement::draft(0x1413cfff0, "unknown_1413cfff0", fn_1413cfff0 as *const ()), // sub_1413cfff0
    Replacement::draft(0x1413d0950, "unknown_1413d0950", fn_1413d0950 as *const ()), // sub_1413d0950
    Replacement::draft_throws(0x1413d0a10, "unknown_1413d0a10", fn_1413d0a10 as *const (), Throws::Caught), // sub_1413d0a10
    Replacement::draft(0x1413d0ac0, "unknown_1413d0ac0", fn_1413d0ac0 as *const ()), // sub_1413d0ac0
    Replacement::draft(0x1413d0b20, "unknown_1413d0b20", fn_1413d0b20 as *const ()), // sub_1413d0b20
    Replacement::draft(0x1413d0be0, "unknown_1413d0be0", fn_1413d0be0 as *const ()), // sub_1413d0be0
    Replacement::draft(0x1413d0d30, "unknown_1413d0d30", fn_1413d0d30 as *const ()), // sub_1413d0d30
    Replacement::draft(0x1413d0d90, "unknown_1413d0d90", fn_1413d0d90 as *const ()), // sub_1413d0d90
    Replacement::draft(0x1413d0e50, "unknown_1413d0e50", fn_1413d0e50 as *const ()), // sub_1413d0e50
    Replacement::draft(0x1413d1110, "unknown_1413d1110", fn_1413d1110 as *const ()), // sub_1413d1110
    Replacement::draft(0x1413d13e0, "unknown_1413d13e0", fn_1413d13e0 as *const ()), // sub_1413d13e0
    Replacement::draft(0x1413d1450, "unknown_1413d1450", fn_1413d1450 as *const ()), // sub_1413d1450
    Replacement::draft(0x1413d14c0, "unknown_1413d14c0", fn_1413d14c0 as *const ()), // sub_1413d14c0
    Replacement::draft(0x1413d1560, "unknown_1413d1560", fn_1413d1560 as *const ()), // sub_1413d1560
    Replacement::draft(0x1413d1600, "unknown_1413d1600", tpl_20795::<0x8, 0x1413d13e0> as *const ()), // sub_1413d1600
    Replacement::draft(0x1413d1610, "unknown_1413d1610", tpl_20795::<0x8, 0x1413d1450> as *const ()), // sub_1413d1610
    Replacement::draft_throws(0x1413d1620, "unknown_1413d1620", fn_1413d1620 as *const (), Throws::Discard), // sub_1413d1620
    Replacement::draft(0x1413d16f0, "unknown_1413d16f0", fn_1413d16f0 as *const ()), // sub_1413d16f0
    Replacement::draft_throws(0x1413d17e0, "sparsehash_1413d17e0", fn_1413d17e0 as *const (), Throws::Discard), // sub_1413d17e0
    Replacement::draft(0x1413d1890, "unknown_1413d1890", fn_1413d1890 as *const ()), // sub_1413d1890
    Replacement::draft(0x1413d1a80, "unknown_1413d1a80", fn_1413d1a80 as *const ()), // sub_1413d1a80
    Replacement::draft(0x1413d1c10, "unknown_1413d1c10", fn_1413d1c10 as *const ()), // sub_1413d1c10
    Replacement::draft(0x1413d23b0, "unknown_1413d23b0", fn_1413d23b0 as *const ()), // sub_1413d23b0
    Replacement::draft_throws(0x1413d2430, "unknown_1413d2430", fn_1413d2430 as *const (), Throws::Caught), // sub_1413d2430
    Replacement::draft_throws(0x1413d24a0, "unknown_1413d24a0", fn_1413d24a0 as *const (), Throws::Caught), // sub_1413d24a0
    Replacement::draft_throws(0x1413d2510, "unknown_1413d2510", fn_1413d2510 as *const (), Throws::Caught), // sub_1413d2510
    Replacement::draft_throws(0x1413d2580, "xml_1413d2580", fn_1413d2580 as *const (), Throws::Discard), // xml::ElementDataWriter::xml::ElementDataWriter::vfunc_0
    Replacement::draft(0x1413d25c0, "xml_1413d25c0", tpl_790::<0x142499eb0, 0x141834d30> as *const ()), // xml::ElementDataReader::xml::ElementDataReader::vfunc_0
    Replacement::draft(0x1413d2630, "unknown_1413d2630", fn_1413d2630 as *const ()), // sub_1413d2630
    Replacement::draft(0x1413d26f0, "unknown_1413d26f0", fn_1413d26f0 as *const ()), // sub_1413d26f0
    Replacement::draft_throws(0x1413d2780, "unknown_1413d2780", fn_1413d2780 as *const (), Throws::Discard), // sub_1413d2780
    Replacement::draft_throws(0x1413d2900, "unknown_1413d2900", fn_1413d2900 as *const (), Throws::Discard), // sub_1413d2900
    Replacement::draft_throws(0x1413d29a0, "unknown_1413d29a0", fn_1413d29a0 as *const (), Throws::Caught), // sub_1413d29a0
    Replacement::draft_throws(0x1413d2a10, "unknown_1413d2a10", fn_1413d2a10 as *const (), Throws::Caught), // sub_1413d2a10
    Replacement::draft(0x1413d2aa0, "unknown_1413d2aa0", tpl_24940::<0x14340ed48, 0x8, 0x14340f010, 0x14340ef78, 0x142498f00> as *const ()), // sub_1413d2aa0
    Replacement::draft(0x1413d2ae0, "unknown_1413d2ae0", fn_1413d2ae0 as *const ()), // sub_1413d2ae0
    Replacement::draft(0x1413d2af0, "unknown_1413d2af0", fn_1413d2af0 as *const ()), // sub_1413d2af0
    Replacement::draft(0x1413d2b00, "unknown_1413d2b00", tpl_3511::<0x14340ed48, 0x8, 0x14340eff8, 0x14340efb8, 0x142498f00> as *const ()), // sub_1413d2b00
    Replacement::draft_throws(0x1413d2b40, "unknown_1413d2b40", fn_1413d2b40 as *const (), Throws::Caught), // sub_1413d2b40
    Replacement::draft_throws(0x1413d2bb0, "unknown_1413d2bb0", fn_1413d2bb0 as *const (), Throws::Caught), // sub_1413d2bb0
    Replacement::draft_throws(0x1413d6bb0, "net_1413d6bb0", fn_1413d6bb0 as *const (), Throws::Discard), // net::CommunicationPipeManager::net::CommunicationPipeManager::vfunc_0
    Replacement::draft(0x1413d6c50, "unknown_1413d6c50", fn_1413d6c50 as *const ()), // sub_1413d6c50
    Replacement::draft(0x1413d6d80, "unknown_1413d6d80", fn_1413d6d80 as *const ()), // sub_1413d6d80
    Replacement::draft(0x1413d6e70, "unknown_1413d6e70", fn_1413d6e70 as *const ()), // sub_1413d6e70
    Replacement::draft_throws(0x1413d8030, "unknown_1413d8030", fn_1413d8030 as *const (), Throws::Caught), // sub_1413d8030
    Replacement::draft_throws(0x1413d81d0, "unknown_1413d81d0", fn_1413d81d0 as *const (), Throws::Caught), // sub_1413d81d0
    Replacement::draft_throws(0x1413d83a0, "unknown_1413d83a0", fn_1413d83a0 as *const (), Throws::Caught), // sub_1413d83a0
    Replacement::draft(0x1413d85b0, "unknown_1413d85b0", fn_1413d85b0 as *const ()), // sub_1413d85b0
    Replacement::draft(0x1413d8950, "unknown_1413d8950", tpl_25747::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1413d8b10, 0x30, 0x38, 0x1401e5730> as *const ()), // sub_1413d8950
    Replacement::draft(0x1413d8a30, "unknown_1413d8a30", tpl_25747::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1401e5930, 0x30, 0x38, 0x1401e5730> as *const ()), // sub_1413d8a30
    Replacement::draft(0x1413d8b10, "unknown_1413d8b10", fn_1413d8b10 as *const ()), // sub_1413d8b10
    Replacement::draft(0x1413d8b90, "unknown_1413d8b90", fn_1413d8b90 as *const ()), // sub_1413d8b90
    Replacement::draft_throws(0x1413d8bf0, "std_1413d8bf0", fn_1413d8bf0 as *const (), Throws::Discard), // sub_1413d8bf0
    Replacement::draft_throws(0x1413d8ee0, "std_1413d8ee0", fn_1413d8ee0 as *const (), Throws::Discard), // sub_1413d8ee0
    Replacement::draft(0x1413d96f0, "unknown_1413d96f0", tpl_23827::<0x14340ed48, 0x8, 0x143412360, 0x142498f00> as *const ()), // sub_1413d96f0
    Replacement::draft(0x1413d9730, "unknown_1413d9730", fn_1413d9730 as *const ()), // sub_1413d9730
    Replacement::draft_throws(0x1413d9780, "unknown_1413d9780", fn_1413d9780 as *const (), Throws::Caught), // sub_1413d9780
    Replacement::draft(0x1413d97f0, "unknown_1413d97f0", fn_1413d97f0 as *const ()), // sub_1413d97f0
    Replacement::draft(0x1413d9860, "unknown_1413d9860", fn_1413d9860 as *const ()), // sub_1413d9860
    Replacement::draft_throws(0x1413d9880, "unknown_1413d9880", fn_1413d9880 as *const (), Throws::Discard), // sub_1413d9880
    Replacement::draft(0x1413d9a70, "unknown_1413d9a70", fn_1413d9a70 as *const ()), // sub_1413d9a70
    Replacement::draft(0x1413d9a90, "unknown_1413d9a90", fn_1413d9a90 as *const ()), // sub_1413d9a90
    Replacement::draft_throws(0x1413d9ab0, "unknown_1413d9ab0", fn_1413d9ab0 as *const (), Throws::Discard), // sub_1413d9ab0
    Replacement::draft_throws(0x1413d9ca0, "unknown_1413d9ca0", fn_1413d9ca0 as *const (), Throws::Caught), // sub_1413d9ca0
    Replacement::draft(0x1413d9dd0, "unknown_1413d9dd0", fn_1413d9dd0 as *const ()), // sub_1413d9dd0
    Replacement::draft_throws(0x1413d9df0, "unknown_1413d9df0", fn_1413d9df0 as *const (), Throws::Caught), // sub_1413d9df0
    Replacement::draft(0x1413d9f20, "unknown_1413d9f20", fn_1413d9f20 as *const ()), // sub_1413d9f20
    Replacement::draft(0x1413d9f40, "ak_1413d9f40", tpl_893::<0x143412880, 0x141834d30> as *const ()), // AK::StreamMgr::IAkLowLevelIOHook::AK::StreamMgr::IAkLowLevelIOHook::vfunc_0
    Replacement::draft(0x1413d9f70, "ak_1413d9f70", tpl_83915::<0x10, 0x40, 0x18, 0x4> as *const ()), // AK::StreamMgr::IAkIOHookDeferred::AK::StreamMgr::IAkIOHookDeferred::vfunc_5
    Replacement::draft(0x1413da010, "ak_1413da010", tpl_83915::<0x10, 0x48, 0x18, 0x4> as *const ()), // AK::StreamMgr::IAkIOHookDeferred::AK::StreamMgr::IAkIOHookDeferred::vfunc_6
    Replacement::draft(0x1413da0b0, "ak_1413da0b0", fn_1413da0b0 as *const ()), // AK::StreamMgr::IAkIOHookDeferred::AK::StreamMgr::IAkIOHookDeferred::vfunc_7
    Replacement::draft(0x1413da130, "ak_1413da130", tpl_893::<0x1434129c8, 0x141834d30> as *const ()), // AK::StreamMgr::IAkFileLocationResolver::AK::StreamMgr::IAkFileLocationResolver::vfunc_0
    Replacement::draft(0x1413da160, "unknown_1413da160", fn_1413da160 as *const ()), // sub_1413da160
    Replacement::draft_throws(0x1413da1a0, "unknown_1413da1a0", fn_1413da1a0 as *const (), Throws::Caught), // sub_1413da1a0
    Replacement::draft_throws(0x1413da510, "unknown_1413da510", fn_1413da510 as *const (), Throws::Caught), // sub_1413da510
    Replacement::draft_throws(0x1413da5e0, "snd_1413da5e0", fn_1413da5e0 as *const (), Throws::Discard), // snd::WwiseIOHook::snd::WwiseIOHook::vfunc_0
    Replacement::draft(0x1413da650, "sparsehash_1413da650", tpl_25773::<0x143c4cd80, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x143c4cd58, 0x143c4cd68, 0x8, 0x10> as *const ()), // sub_1413da650
    Replacement::draft(0x1413da730, "unknown_1413da730", fn_1413da730 as *const ()), // sub_1413da730
    Replacement::draft_throws(0x1413da890, "std_1413da890", fn_1413da890 as *const (), Throws::Discard), // sub_1413da890
    Replacement::draft_throws(0x1413dac70, "std_1413dac70", tpl_83923::<0x14307e970, 0x142645870, 0x143c4cd58, 0x143c4cd68, 0x1401e5730, 0x143c4cd80, 0x143c4cd88, 0x1413dac9f> as *const (), Throws::Discard), // sub_1413dac70
    Replacement::draft(0x1413dae80, "ak_1413dae80", fn_1413dae80 as *const ()), // sub_1413dae80
    Replacement::draft_throws(0x1413daec0, "ak_1413daec0", fn_1413daec0 as *const (), Throws::Caught), // sub_1413daec0
    Replacement::draft(0x1413daf40, "ak_1413daf40", fn_1413daf40 as *const ()), // sub_1413daf40
    Replacement::draft(0x1413dafe0, "ak_1413dafe0", fn_1413dafe0 as *const ()), // sub_1413dafe0
    Replacement::draft(0x1413db040, "ak_1413db040", fn_1413db040 as *const ()), // sub_1413db040
    Replacement::draft_throws(0x1413db0c0, "ak_1413db0c0", fn_1413db0c0 as *const (), Throws::Caught), // sub_1413db0c0
    Replacement::draft(0x1413db2a0, "ak_1413db2a0", tpl_365::<0x1413db040> as *const ()), // sub_1413db2a0
    Replacement::draft(0x1413db2b0, "ak_1413db2b0", fn_1413db2b0 as *const ()), // sub_1413db2b0
    Replacement::draft(0x1413db2e0, "ak_1413db2e0", fn_1413db2e0 as *const ()), // sub_1413db2e0
    Replacement::draft(0x1413db310, "ak_1413db310", fn_1413db310 as *const ()), // sub_1413db310
    Replacement::draft(0x1413db350, "ak_1413db350", tpl_25783::<0x4, 0x3, 0x8, 0x14240a730, 0x50, 0x98, 0xe0> as *const ()), // sub_1413db350
    Replacement::draft(0x1413db440, "ak_1413db440", tpl_170::<0x1413dca50> as *const ()), // sub_1413db440
    Replacement::draft(0x1413db450, "ak_1413db450", tpl_25783::<0x4, 0x3, 0x8, 0x14240aa00, 0x50, 0x98, 0xe0> as *const ()), // sub_1413db450
    Replacement::draft(0x1413db540, "ak_1413db540", fn_1413db540 as *const ()), // sub_1413db540
    Replacement::draft_throws(0x1413db580, "ak_1413db580", fn_1413db580 as *const (), Throws::Caught), // sub_1413db580
    Replacement::draft_throws(0x1413db700, "ak_1413db700", fn_1413db700 as *const (), Throws::Caught), // sub_1413db700
    Replacement::draft_throws(0x1413db850, "unknown_1413db850", fn_1413db850 as *const (), Throws::Discard), // sub_1413db850
    Replacement::draft(0x1413db890, "unknown_1413db890", tpl_365::<0x1413dbe20> as *const ()), // sub_1413db890
    Replacement::draft_throws(0x1413db8a0, "ak_1413db8a0", fn_1413db8a0 as *const (), Throws::Caught), // sub_1413db8a0
    Replacement::draft(0x1413db9b0, "ak_1413db9b0", fn_1413db9b0 as *const ()), // sub_1413db9b0
    Replacement::draft(0x1413dbac0, "ak_1413dbac0", fn_1413dbac0 as *const ()), // sub_1413dbac0
    Replacement::draft(0x1413dbb50, "ak_1413dbb50", tpl_22426::<0x143412ca0, 0x1413dbee0> as *const ()), // sub_1413dbb50
    Replacement::draft_throws(0x1413dbb60, "ak_1413dbb60", fn_1413dbb60 as *const (), Throws::Discard), // sub_1413dbb60
    Replacement::draft_throws(0x1413dbc60, "ak_1413dbc60", fn_1413dbc60 as *const (), Throws::Discard), // sub_1413dbc60
    Replacement::draft_throws(0x1413dbcf0, "ak_1413dbcf0", fn_1413dbcf0 as *const (), Throws::Discard), // sub_1413dbcf0
    Replacement::draft(0x1413dbd70, "ak_1413dbd70", fn_1413dbd70 as *const ()), // sub_1413dbd70
    Replacement::draft(0x1413dbde0, "ak_1413dbde0", fn_1413dbde0 as *const ()), // AK::WwiseAuthoringAPI::AkJsonBase<AK::WwiseAuthoringAPI::AkVariant,std::basic_string<char,std::char_traits<char>,std::al
    Replacement::draft(0x1413dbe20, "ak_1413dbe20", fn_1413dbe20 as *const ()), // sub_1413dbe20
    Replacement::draft(0x1413dbee0, "ak_1413dbee0", fn_1413dbee0 as *const ()), // sub_1413dbee0
    Replacement::draft_throws(0x1413dbf70, "ak_1413dbf70", fn_1413dbf70 as *const (), Throws::Caught), // sub_1413dbf70
    Replacement::draft_throws(0x1413dc030, "ak_1413dc030", fn_1413dc030 as *const (), Throws::Discard), // sub_1413dc030
    Replacement::draft_throws(0x1413dc080, "ak_1413dc080", fn_1413dc080 as *const (), Throws::Caught), // sub_1413dc080
    Replacement::draft_throws(0x1413dc270, "unknown_1413dc270", fn_1413dc270 as *const (), Throws::Discard), // sub_1413dc270
    Replacement::draft_throws(0x1413dc2f0, "unknown_1413dc2f0", fn_1413dc2f0 as *const (), Throws::Discard), // sub_1413dc2f0
    Replacement::draft(0x1413dc360, "global_1413dc360", fn_1413dc360 as *const ()), // deallocate
    Replacement::draft(0x1413dc3b0, "unknown_1413dc3b0", fn_1413dc3b0 as *const ()), // sub_1413dc3b0
    Replacement::draft_throws(0x1413dc420, "unknown_1413dc420", fn_1413dc420 as *const (), Throws::Caught), // sub_1413dc420
    Replacement::draft_throws(0x1413dc4d0, "ak_1413dc4d0", fn_1413dc4d0 as *const (), Throws::Discard), // sub_1413dc4d0
    Replacement::draft_throws(0x1413dc520, "std_1413dc520", fn_1413dc520 as *const (), Throws::Discard), // sub_1413dc520
    Replacement::draft(0x1413dc890, "unknown_1413dc890", fn_1413dc890 as *const ()), // sub_1413dc890
    Replacement::draft(0x1413dc8d0, "ak_1413dc8d0", fn_1413dc8d0 as *const ()), // sub_1413dc8d0
    Replacement::draft(0x1413dc960, "ak_1413dc960", fn_1413dc960 as *const ()), // sub_1413dc960
    Replacement::draft(0x1413dc970, "ak_1413dc970", fn_1413dc970 as *const ()), // sub_1413dc970
    Replacement::draft(0x1413dca50, "ak_1413dca50", fn_1413dca50 as *const ()), // sub_1413dca50
    Replacement::draft(0x1413dcb10, "ak_1413dcb10", fn_1413dcb10 as *const ()), // sub_1413dcb10
    Replacement::draft(0x1413dcb80, "ak_1413dcb80", fn_1413dcb80 as *const ()), // sub_1413dcb80
    Replacement::draft_throws(0x1413dcc50, "ak_1413dcc50", fn_1413dcc50 as *const (), Throws::Caught), // sub_1413dcc50
    Replacement::draft_throws(0x1413dccb0, "ak_1413dccb0", fn_1413dccb0 as *const (), Throws::Caught), // sub_1413dccb0
    Replacement::draft_throws(0x1413dced0, "ak_1413dced0", fn_1413dced0 as *const (), Throws::Discard), // sub_1413dced0
    Replacement::draft(0x1413dd120, "ak_1413dd120", fn_1413dd120 as *const ()), // sub_1413dd120
    Replacement::draft_throws(0x1413dd160, "ak_1413dd160", fn_1413dd160 as *const (), Throws::Discard), // sub_1413dd160
    Replacement::draft(0x1413dd190, "ak_1413dd190", fn_1413dd190 as *const ()), // sub_1413dd190
    Replacement::draft(0x1413dd310, "ak_1413dd310", fn_1413dd310 as *const ()), // sub_1413dd310
    Replacement::draft(0x1413dd360, "ak_1413dd360", fn_1413dd360 as *const ()), // sub_1413dd360
    Replacement::draft_throws(0x1413dd5d0, "ak_1413dd5d0", fn_1413dd5d0 as *const (), Throws::Caught), // sub_1413dd5d0
    Replacement::draft(0x1413dd660, "unknown_1413dd660", fn_1413dd660 as *const ()), // sub_1413dd660
    Replacement::draft(0x1413dd7f0, "unknown_1413dd7f0", fn_1413dd7f0 as *const ()), // sub_1413dd7f0
    Replacement::draft(0x1413dd8c0, "snd_1413dd8c0", fn_1413dd8c0 as *const ()), // sub_1413dd8c0
    Replacement::draft_throws(0x1413dd930, "snd_1413dd930", fn_1413dd930 as *const (), Throws::Caught), // sub_1413dd930
    Replacement::draft(0x1413de0e0, "snd_1413de0e0", fn_1413de0e0 as *const ()), // sub_1413de0e0
    Replacement::draft_throws(0x1413de1a0, "snd_1413de1a0", fn_1413de1a0 as *const (), Throws::Caught), // sub_1413de1a0
    Replacement::draft_throws(0x1413de940, "snd_1413de940", fn_1413de940 as *const (), Throws::Caught), // sub_1413de940
    Replacement::draft(0x1413df0e0, "snd_1413df0e0", fn_1413df0e0 as *const ()), // sub_1413df0e0
    Replacement::draft_throws(0x1413df190, "snd_1413df190", fn_1413df190 as *const (), Throws::Caught), // sub_1413df190
    Replacement::draft_throws(0x1413df940, "snd_1413df940", fn_1413df940 as *const (), Throws::Caught), // sub_1413df940
    Replacement::draft_throws(0x1413e00f0, "snd_1413e00f0", fn_1413e00f0 as *const (), Throws::Caught), // sub_1413e00f0
    Replacement::draft_throws(0x1413e08a0, "snd_1413e08a0", fn_1413e08a0 as *const (), Throws::Caught), // sub_1413e08a0
    Replacement::draft(0x1413e1050, "snd_1413e1050", fn_1413e1050 as *const ()), // sub_1413e1050
    Replacement::draft_throws(0x1413e1120, "snd_1413e1120", fn_1413e1120 as *const (), Throws::Caught), // sub_1413e1120
    Replacement::draft(0x1413e18d0, "snd_1413e18d0", fn_1413e18d0 as *const ()), // sub_1413e18d0
    Replacement::draft_throws(0x1413e19d0, "snd_1413e19d0", fn_1413e19d0 as *const (), Throws::Caught), // sub_1413e19d0
    Replacement::draft(0x1413e2180, "snd_1413e2180", fn_1413e2180 as *const ()), // sub_1413e2180
    Replacement::draft_throws(0x1413e2240, "snd_1413e2240", fn_1413e2240 as *const (), Throws::Caught), // sub_1413e2240
    Replacement::draft(0x1413e29e0, "snd_1413e29e0", tpl_20036::<0x14, 0x20, 0x143a08300, 0x14248b0c0, 0x18, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x140236fa0, 0x142442c80, 0x8, 0x142442de0> as *const ()), // sub_1413e29e0
    Replacement::draft(0x1413e2ab0, "snd_1413e2ab0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1434131f8, 0x8> as *const ()), // sub_1413e2ab0
    Replacement::draft(0x1413e2b30, "snd_1413e2b30", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1434131f8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1413e2b30
    Replacement::draft_throws(0x1413e2bd0, "snd_1413e2bd0", fn_1413e2bd0 as *const (), Throws::Caught), // sub_1413e2bd0
    Replacement::draft_throws(0x1413e2c60, "snd_1413e2c60", tpl_649::<0x1438820e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413e2c60
    Replacement::draft(0x1413e2ce0, "snd_1413e2ce0", fn_1413e2ce0 as *const ()), // sub_1413e2ce0
    Replacement::draft(0x1413e2d70, "snd_1413e2d70", fn_1413e2d70 as *const ()), // sub_1413e2d70
    Replacement::draft(0x1413e2e20, "snd_1413e2e20", tpl_6868::<0x1438820f8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1413e2e20
    Replacement::draft_throws(0x1413e2eb0, "snd_1413e2eb0", tpl_23922::<0x1438820f0, 0x1438820f8, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1413e2eb0
    Replacement::draft_throws(0x1413e2f30, "snd_1413e2f30", tpl_23922::<0x1438820f0, 0x1438820f8, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1413e2f30
    Replacement::draft_throws(0x1413e2fb0, "snd_1413e2fb0", fn_1413e2fb0 as *const (), Throws::Caught), // sub_1413e2fb0
    Replacement::draft(0x1413e3040, "snd_1413e3040", tpl_23923::<0x1438820f0, 0x1438820f8, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1413e3040
    Replacement::draft_throws(0x1413e30c0, "snd_1413e30c0", tpl_23922::<0x1438820f0, 0x1438820f8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1413e30c0
    Replacement::draft_throws(0x1413e3140, "snd_1413e3140", tpl_23924::<0x1438820f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1413e3140
    Replacement::draft(0x1413e31f0, "snd_1413e31f0", fn_1413e31f0 as *const ()), // sub_1413e31f0
    Replacement::draft(0x1413e33a0, "snd_1413e33a0", fn_1413e33a0 as *const ()), // snd::SystemicDialogueResource::snd::SystemicDialogueResource::vfunc_0
    Replacement::draft_throws(0x1413e3460, "snd_1413e3460", fn_1413e3460 as *const (), Throws::Discard), // sub_1413e3460
    Replacement::draft(0x1413e35e0, "snd_1413e35e0", fn_1413e35e0 as *const ()), // snd::SoundToolProcess::snd::SoundToolProcess::vfunc_0
    Replacement::draft(0x1413e3620, "unknown_1413e3620", fn_1413e3620 as *const ()), // sub_1413e3620
    Replacement::draft(0x1413e3710, "unknown_1413e3710", fn_1413e3710 as *const ()), // sub_1413e3710
    Replacement::draft(0x1413e38b0, "unknown_1413e38b0", fn_1413e38b0 as *const ()), // sub_1413e38b0
    Replacement::draft_throws(0x1413e3960, "unknown_1413e3960", fn_1413e3960 as *const (), Throws::Caught), // sub_1413e3960
    Replacement::draft_throws(0x1413e39f0, "sparsehash_1413e39f0", fn_1413e39f0 as *const (), Throws::Discard), // sub_1413e39f0
    Replacement::draft(0x1413e3b40, "unknown_1413e3b40", tpl_170::<0x1413e3ee0> as *const ()), // sub_1413e3b40
    Replacement::draft(0x1413e3b50, "unknown_1413e3b50", fn_1413e3b50 as *const ()), // sub_1413e3b50
    Replacement::draft(0x1413e3be0, "std_1413e3be0", tpl_21600::<0x1412f6440> as *const ()), // sub_1413e3be0
    Replacement::draft(0x1413e3d00, "std_1413e3d00", fn_1413e3d00 as *const ()), // sub_1413e3d00
    Replacement::draft(0x1413e3e60, "std_1413e3e60", fn_1413e3e60 as *const ()), // sub_1413e3e60
    Replacement::draft(0x1413e3ee0, "std_1413e3ee0", fn_1413e3ee0 as *const ()), // sub_1413e3ee0
    Replacement::draft(0x1413e3fc0, "std_1413e3fc0", fn_1413e3fc0 as *const ()), // sub_1413e3fc0
    Replacement::draft(0x1413e40a0, "std_1413e40a0", tpl_23357::<0x143413340, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<snd::PlayingInstanceBuffer,r::Allocator<snd::PlayingInstanceBuffer,0>>::std::_Ref_count_obj_a
    Replacement::draft(0x1413e40d0, "unknown_1413e40d0", fn_1413e40d0 as *const ()), // sub_1413e40d0
    Replacement::draft(0x1413e4290, "unknown_1413e4290", fn_1413e4290 as *const ()), // sub_1413e4290
    Replacement::draft(0x1413e4550, "unknown_1413e4550", tpl_170::<0x1413e4950> as *const ()), // sub_1413e4550
    Replacement::draft(0x1413e4560, "unknown_1413e4560", fn_1413e4560 as *const ()), // sub_1413e4560
    Replacement::draft(0x1413e45c0, "unknown_1413e45c0", fn_1413e45c0 as *const ()), // sub_1413e45c0
    Replacement::draft_throws(0x1413e4770, "unknown_1413e4770", tpl_13418::<0x1413e4a70> as *const (), Throws::Discard), // sub_1413e4770
    Replacement::draft(0x1413e4780, "unknown_1413e4780", tpl_23750::<0x78> as *const ()), // sub_1413e4780
    Replacement::draft(0x1413e4790, "unknown_1413e4790", fn_1413e4790 as *const ()), // sub_1413e4790
    Replacement::draft(0x1413e4950, "unknown_1413e4950", tpl_23926::<0x8, 0x1413e4a10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413e4950
    Replacement::draft(0x1413e4a10, "unknown_1413e4a10", fn_1413e4a10 as *const ()), // sub_1413e4a10
    Replacement::draft_throws(0x1413e4a70, "unknown_1413e4a70", fn_1413e4a70 as *const (), Throws::Discard), // sub_1413e4a70
    Replacement::draft(0x1413e4ad0, "unknown_1413e4ad0", fn_1413e4ad0 as *const ()), // sub_1413e4ad0
    Replacement::draft(0x1413e4dd0, "unknown_1413e4dd0", fn_1413e4dd0 as *const ()), // sub_1413e4dd0
    Replacement::draft(0x1413e50d0, "unknown_1413e50d0", fn_1413e50d0 as *const ()), // sub_1413e50d0
    Replacement::draft(0x1413e53e0, "unknown_1413e53e0", fn_1413e53e0 as *const ()), // sub_1413e53e0
    Replacement::draft(0x1413e5570, "unknown_1413e5570", tpl_23299::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1413e62b0, 0x1413e4950> as *const ()), // sub_1413e5570
    Replacement::draft_throws(0x1413e56f0, "unknown_1413e56f0", fn_1413e56f0 as *const (), Throws::Discard), // sub_1413e56f0
    Replacement::draft(0x1413e57b0, "unknown_1413e57b0", fn_1413e57b0 as *const ()), // sub_1413e57b0
    Replacement::draft(0x1413e5aa0, "unknown_1413e5aa0", fn_1413e5aa0 as *const ()), // sub_1413e5aa0
    Replacement::draft(0x1413e5d90, "unknown_1413e5d90", fn_1413e5d90 as *const ()), // sub_1413e5d90
    Replacement::draft(0x1413e5f50, "unknown_1413e5f50", fn_1413e5f50 as *const ()), // sub_1413e5f50
    Replacement::draft(0x1413e5fb0, "unknown_1413e5fb0", tpl_25859::<0xffffffffffffffff, 0x10, 0x8, 0x1c> as *const ()), // sub_1413e5fb0
    Replacement::draft(0x1413e6090, "unknown_1413e6090", tpl_25859::<0xffffffffffffffff, 0x10, 0x8, 0x18> as *const ()), // sub_1413e6090
    Replacement::draft(0x1413e6170, "unknown_1413e6170", fn_1413e6170 as *const ()), // sub_1413e6170
    Replacement::draft(0x1413e61e0, "unknown_1413e61e0", fn_1413e61e0 as *const ()), // sub_1413e61e0
    Replacement::draft(0x1413e6250, "unknown_1413e6250", fn_1413e6250 as *const ()), // sub_1413e6250
    Replacement::draft(0x1413e62b0, "unknown_1413e62b0", fn_1413e62b0 as *const ()), // sub_1413e62b0
    Replacement::draft(0x1413e6420, "snd_1413e6420", fn_1413e6420 as *const ()), // snd::SoundBankResource::snd::SoundBankResource::vfunc_0
    Replacement::draft(0x1413e6460, "snd_1413e6460", fn_1413e6460 as *const ()), // sub_1413e6460
    Replacement::draft(0x1413e6520, "snd_1413e6520", fn_1413e6520 as *const ()), // sub_1413e6520
    Replacement::draft(0x1413e65f0, "snd_1413e65f0", fn_1413e65f0 as *const ()), // sub_1413e65f0
    Replacement::draft_throws(0x1413e6700, "snd_1413e6700", fn_1413e6700 as *const (), Throws::Discard), // sub_1413e6700
    Replacement::draft(0x1413e6800, "snd_1413e6800", tpl_25868::<0x143882110, 0x14243b0b0, 0x338, 0x58, 0x40, 0x5c, 0x80, 0x10, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1413e6800
    Replacement::draft_throws(0x1413e68a0, "snd_1413e68a0", fn_1413e68a0 as *const (), Throws::Discard), // sub_1413e68a0
    Replacement::draft_throws(0x1413e69a0, "snd_1413e69a0", fn_1413e69a0 as *const (), Throws::Caught), // sub_1413e69a0
    Replacement::draft_throws(0x1413e6ac0, "snd_1413e6ac0", fn_1413e6ac0 as *const (), Throws::Caught), // sub_1413e6ac0
    Replacement::draft_throws(0x1413e6be0, "snd_1413e6be0", fn_1413e6be0 as *const (), Throws::Caught), // sub_1413e6be0
    Replacement::draft(0x1413e6d00, "snd_1413e6d00", fn_1413e6d00 as *const ()), // sub_1413e6d00
    Replacement::draft(0x1413e6db0, "snd_1413e6db0", fn_1413e6db0 as *const ()), // sub_1413e6db0
    Replacement::draft(0x1413e6e40, "snd_1413e6e40", tpl_25868::<0x143882110, 0x14243b0b0, 0x338, 0x58, 0xe0, 0x5c, 0x120, 0x10, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1413e6e40
    Replacement::draft_throws(0x1413e6ee0, "snd_1413e6ee0", fn_1413e6ee0 as *const (), Throws::Discard), // sub_1413e6ee0
    Replacement::draft_throws(0x1413e6f80, "snd_1413e6f80", fn_1413e6f80 as *const (), Throws::Discard), // sub_1413e6f80
    Replacement::draft(0x1413e70a0, "snd_1413e70a0", fn_1413e70a0 as *const ()), // sub_1413e70a0
    Replacement::draft(0x1413e7150, "snd_1413e7150", fn_1413e7150 as *const ()), // sub_1413e7150
    Replacement::draft_throws(0x1413e73c0, "snd_1413e73c0", fn_1413e73c0 as *const (), Throws::Caught), // sub_1413e73c0
    Replacement::draft_throws(0x1413e7550, "snd_1413e7550", tpl_1905::<0x143882108, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1413e7550
    Replacement::draft(0x1413e7650, "unknown_1413e7650", fn_1413e7650 as *const ()), // sub_1413e7650
    Replacement::draft(0x1413e77a0, "unknown_1413e77a0", fn_1413e77a0 as *const ()), // sub_1413e77a0
    Replacement::draft(0x1413e7800, "unknown_1413e7800", tpl_5914::<0x10, 0x38, 0x8, 0x1413e78d0, 0x18, 0x1401de830, 0x1401e66f0, 0x20, 0x1401e6780> as *const ()), // sub_1413e7800
    Replacement::draft(0x1413e78d0, "std_1413e78d0", fn_1413e78d0 as *const ()), // sub_1413e78d0
    Replacement::draft(0x1413e7b30, "unknown_1413e7b30", fn_1413e7b30 as *const ()), // sub_1413e7b30
    Replacement::draft(0x1413e7bc0, "unknown_1413e7bc0", fn_1413e7bc0 as *const ()), // sub_1413e7bc0
    Replacement::draft(0x1413e7d00, "sparsehash_1413e7d00", tpl_25773::<0x143c4cf30, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x143c4cf08, 0x143c4cf18, 0x8, 0x10> as *const ()), // sub_1413e7d00
    Replacement::draft(0x1413e7f30, "unknown_1413e7f30", tpl_18985::<0x8, 0x140203870, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1413e7f30
    Replacement::draft(0x1413e8000, "unknown_1413e8000", fn_1413e8000 as *const ()), // sub_1413e8000
    Replacement::draft_throws(0x1413e80e0, "std_1413e80e0", fn_1413e80e0 as *const (), Throws::Discard), // sub_1413e80e0
    Replacement::draft(0x1413e8410, "std_1413e8410", fn_1413e8410 as *const ()), // std::_Ref_count_obj_alloc3<snd::music_callbacks::MusicInstance,r::Allocator<snd::music_callbacks::MusicInstance,0>>::std
    Replacement::draft(0x1413e8480, "std_1413e8480", fn_1413e8480 as *const ()), // std::_Ref_count_obj_alloc3<snd::music_callbacks::MusicInstance,r::Allocator<snd::music_callbacks::MusicInstance,0>>::std
    Replacement::draft_throws(0x1413e84b0, "std_1413e84b0", tpl_83923::<0x14307e970, 0x142645870, 0x143c4cf08, 0x143c4cf18, 0x1401e5730, 0x143c4cf30, 0x143c4cf38, 0x1413e84df> as *const (), Throws::Discard), // sub_1413e84b0
    Replacement::draft(0x1413e86b0, "unknown_1413e86b0", fn_1413e86b0 as *const ()), // sub_1413e86b0
    Replacement::draft(0x1413e8860, "unknown_1413e8860", fn_1413e8860 as *const ()), // sub_1413e8860
    Replacement::draft(0x1413e8930, "unknown_1413e8930", fn_1413e8930 as *const ()), // sub_1413e8930
    Replacement::draft(0x1413e8970, "unknown_1413e8970", fn_1413e8970 as *const ()), // sub_1413e8970
    Replacement::draft(0x1413e8af0, "snd_1413e8af0", fn_1413e8af0 as *const ()), // snd::ExternalSourceResource::snd::ExternalSourceResource::vfunc_0
    Replacement::draft(0x1413e8b70, "snd_1413e8b70", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1434143b0, 0x8> as *const ()), // sub_1413e8b70
    Replacement::draft(0x1413e8bf0, "snd_1413e8bf0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1434143b0, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1413e8bf0
    Replacement::draft_throws(0x1413e8c90, "snd_1413e8c90", fn_1413e8c90 as *const (), Throws::Caught), // sub_1413e8c90
    Replacement::draft_throws(0x1413e8d20, "snd_1413e8d20", tpl_1908::<0x143882138, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413e8d20
    Replacement::draft(0x1413e8da0, "snd_1413e8da0", fn_1413e8da0 as *const ()), // sub_1413e8da0
    Replacement::draft(0x1413e8e20, "snd_1413e8e20", fn_1413e8e20 as *const ()), // sub_1413e8e20
    Replacement::draft_throws(0x1413e9060, "snd_1413e9060", fn_1413e9060 as *const (), Throws::Caught), // sub_1413e9060
    Replacement::draft_throws(0x1413e91f0, "snd_1413e91f0", tpl_1905::<0x143882128, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1413e91f0
    Replacement::draft(0x1413e9300, "snd_1413e9300", fn_1413e9300 as *const ()), // snd::DialogueStringTableResource::snd::DialogueStringTableResource::vfunc_0
    Replacement::draft_throws(0x1413e9350, "snd_1413e9350", fn_1413e9350 as *const (), Throws::Caught), // sub_1413e9350
    Replacement::draft(0x1413e9b00, "snd_1413e9b00", tpl_78389::<0x14, 0x20, 0x143a08300, 0x14248b0c0, 0x18, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x108, 0x14240ab10, 0x142442c80, 0x8, 0x142442de0> as *const ()), // sub_1413e9b00
    Replacement::draft(0x1413e9be0, "snd_1413e9be0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143414868, 0x8> as *const ()), // sub_1413e9be0
    Replacement::draft(0x1413e9c60, "snd_1413e9c60", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143414868, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1413e9c60
    Replacement::draft_throws(0x1413e9d00, "snd_1413e9d00", fn_1413e9d00 as *const (), Throws::Caught), // sub_1413e9d00
    Replacement::draft_throws(0x1413e9d90, "snd_1413e9d90", tpl_649::<0x143882150, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413e9d90
    Replacement::draft(0x1413e9e10, "snd_1413e9e10", fn_1413e9e10 as *const ()), // sub_1413e9e10
    Replacement::draft(0x1413e9ee0, "snd_1413e9ee0", fn_1413e9ee0 as *const ()), // sub_1413e9ee0
    Replacement::draft(0x1413e9fe0, "snd_1413e9fe0", tpl_6868::<0x143882140, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1413e9fe0
    Replacement::draft_throws(0x1413ea070, "snd_1413ea070", tpl_23922::<0x143882158, 0x143882140, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1413ea070
    Replacement::draft_throws(0x1413ea0f0, "snd_1413ea0f0", tpl_23922::<0x143882158, 0x143882140, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1413ea0f0
    Replacement::draft_throws(0x1413ea170, "snd_1413ea170", fn_1413ea170 as *const (), Throws::Caught), // sub_1413ea170
    Replacement::draft(0x1413ea200, "snd_1413ea200", tpl_23923::<0x143882158, 0x143882140, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1413ea200
    Replacement::draft_throws(0x1413ea280, "snd_1413ea280", tpl_23922::<0x143882158, 0x143882140, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1413ea280
    Replacement::draft_throws(0x1413ea300, "snd_1413ea300", tpl_23924::<0x143882140, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1413ea300
    Replacement::draft(0x1413ea410, "snd_1413ea410", fn_1413ea410 as *const ()), // snd::DialogueResource::snd::DialogueResource::vfunc_0
    Replacement::draft(0x1413ea470, "snd_1413ea470", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143414e20, 0x8> as *const ()), // sub_1413ea470
    Replacement::draft(0x1413ea4f0, "snd_1413ea4f0", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143414e20, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1413ea4f0
    Replacement::draft_throws(0x1413ea590, "snd_1413ea590", fn_1413ea590 as *const (), Throws::Caught), // sub_1413ea590
    Replacement::draft_throws(0x1413ea620, "snd_1413ea620", tpl_649::<0x143882160, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413ea620
    Replacement::draft(0x1413ea6a0, "snd_1413ea6a0", tpl_7909::<0x120, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x1413eabb0> as *const ()), // sub_1413ea6a0
    Replacement::draft(0x1413ea730, "snd_1413ea730", fn_1413ea730 as *const ()), // sub_1413ea730
    Replacement::draft(0x1413ea7e0, "snd_1413ea7e0", tpl_6868::<0x143882168, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1413ea7e0
    Replacement::draft_throws(0x1413ea870, "snd_1413ea870", tpl_23922::<0x14388215c, 0x143882168, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1413ea870
    Replacement::draft_throws(0x1413ea8f0, "snd_1413ea8f0", tpl_23922::<0x14388215c, 0x143882168, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20> as *const (), Throws::Discard), // sub_1413ea8f0
    Replacement::draft_throws(0x1413ea970, "snd_1413ea970", fn_1413ea970 as *const (), Throws::Caught), // sub_1413ea970
    Replacement::draft(0x1413eaa00, "snd_1413eaa00", tpl_23923::<0x14388215c, 0x143882168, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860> as *const ()), // sub_1413eaa00
    Replacement::draft_throws(0x1413eaa80, "snd_1413eaa80", tpl_23922::<0x14388215c, 0x143882168, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590> as *const (), Throws::Discard), // sub_1413eaa80
    Replacement::draft_throws(0x1413eab00, "snd_1413eab00", tpl_23924::<0x143882168, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_1413eab00
    Replacement::draft(0x1413eabb0, "snd_1413eabb0", fn_1413eabb0 as *const ()), // sub_1413eabb0
    Replacement::draft(0x1413eac30, "unknown_1413eac30", fn_1413eac30 as *const ()), // sub_1413eac30
    Replacement::draft(0x1413ead60, "unknown_1413ead60", tpl_25626::<0x50, 0x68, 0x142442de0, 0x58, 0x5c, 0x60> as *const ()), // sub_1413ead60
    Replacement::draft(0x1413eada0, "unknown_1413eada0", fn_1413eada0 as *const ()), // sub_1413eada0
    Replacement::draft(0x1413eae70, "unknown_1413eae70", fn_1413eae70 as *const ()), // sub_1413eae70
    Replacement::draft(0x1413eaef0, "unknown_1413eaef0", tpl_21430::<0x1413eb610, 0x1401e66f0> as *const ()), // sub_1413eaef0
    Replacement::draft(0x1413eaf10, "unknown_1413eaf10", fn_1413eaf10 as *const ()), // sub_1413eaf10
    Replacement::draft(0x1413eb150, "unknown_1413eb150", fn_1413eb150 as *const ()), // sub_1413eb150
    Replacement::draft(0x1413eb1d0, "unknown_1413eb1d0", fn_1413eb1d0 as *const ()), // sub_1413eb1d0
    Replacement::draft_throws(0x1413eb2e0, "std_1413eb2e0", fn_1413eb2e0 as *const (), Throws::Discard), // sub_1413eb2e0
    Replacement::draft(0x1413eb610, "unknown_1413eb610", fn_1413eb610 as *const ()), // sub_1413eb610
    Replacement::draft(0x1413eb690, "unknown_1413eb690", fn_1413eb690 as *const ()), // sub_1413eb690
    Replacement::draft(0x1413eb6f0, "unknown_1413eb6f0", fn_1413eb6f0 as *const ()), // sub_1413eb6f0
    Replacement::draft(0x1413eb810, "unknown_1413eb810", fn_1413eb810 as *const ()), // sub_1413eb810
    Replacement::draft(0x1413eba20, "unknown_1413eba20", fn_1413eba20 as *const ()), // sub_1413eba20
    Replacement::draft(0x1413ebaa0, "unknown_1413ebaa0", fn_1413ebaa0 as *const ()), // sub_1413ebaa0
    Replacement::draft(0x1413ebb60, "snd_1413ebb60", fn_1413ebb60 as *const ()), // snd::AudioEnvironmentResource::snd::AudioEnvironmentResource::vfunc_0
    Replacement::draft(0x1413ebbe0, "unknown_1413ebbe0", fn_1413ebbe0 as *const ()), // sub_1413ebbe0
    Replacement::draft(0x1413ebc60, "unknown_1413ebc60", fn_1413ebc60 as *const ()), // sub_1413ebc60
    Replacement::draft(0x1413ebd70, "unknown_1413ebd70", fn_1413ebd70 as *const ()), // sub_1413ebd70
    Replacement::draft_throws(0x1413ebea0, "unknown_1413ebea0", fn_1413ebea0 as *const (), Throws::Discard), // sub_1413ebea0
    Replacement::draft(0x1413ebfa0, "unknown_1413ebfa0", tpl_25916::<0x1438821b0, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0x40, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1413ebfa0
    Replacement::draft_throws(0x1413ec040, "unknown_1413ec040", fn_1413ec040 as *const (), Throws::Discard), // sub_1413ec040
    Replacement::draft_throws(0x1413ec160, "unknown_1413ec160", fn_1413ec160 as *const (), Throws::Caught), // sub_1413ec160
    Replacement::draft_throws(0x1413ec2f0, "unknown_1413ec2f0", fn_1413ec2f0 as *const (), Throws::Caught), // sub_1413ec2f0
    Replacement::draft_throws(0x1413ec480, "unknown_1413ec480", fn_1413ec480 as *const (), Throws::Caught), // sub_1413ec480
    Replacement::draft(0x1413ec610, "unknown_1413ec610", tpl_18714::<0x1438821b0, 0x14243b0b0, 0x338, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1413ec610
    Replacement::draft(0x1413ec6b0, "unknown_1413ec6b0", fn_1413ec6b0 as *const ()), // sub_1413ec6b0
    Replacement::draft(0x1413ec750, "unknown_1413ec750", tpl_25916::<0x1438821b0, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xe0, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1413ec750
    Replacement::draft_throws(0x1413ec800, "unknown_1413ec800", fn_1413ec800 as *const (), Throws::Discard), // sub_1413ec800
    Replacement::draft_throws(0x1413ec8c0, "unknown_1413ec8c0", fn_1413ec8c0 as *const (), Throws::Discard), // sub_1413ec8c0
    Replacement::draft(0x1413eca20, "unknown_1413eca20", fn_1413eca20 as *const ()), // sub_1413eca20
    Replacement::draft(0x1413eca90, "unknown_1413eca90", fn_1413eca90 as *const ()), // sub_1413eca90
    Replacement::draft(0x1413ecb10, "unknown_1413ecb10", fn_1413ecb10 as *const ()), // sub_1413ecb10
    Replacement::draft(0x1413ecc20, "unknown_1413ecc20", fn_1413ecc20 as *const ()), // sub_1413ecc20
    Replacement::draft_throws(0x1413ecdb0, "unknown_1413ecdb0", fn_1413ecdb0 as *const (), Throws::Discard), // sub_1413ecdb0
    Replacement::draft(0x1413ecf00, "unknown_1413ecf00", tpl_25927::<0x143882198, 0x14243b0b0, 0x338, 0x38, 0x4, 0x10, 0x40, 0x14, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1413ecf00
    Replacement::draft_throws(0x1413ecfd0, "unknown_1413ecfd0", fn_1413ecfd0 as *const (), Throws::Discard), // sub_1413ecfd0
    Replacement::draft_throws(0x1413ed150, "unknown_1413ed150", fn_1413ed150 as *const (), Throws::Caught), // sub_1413ed150
    Replacement::draft_throws(0x1413ed340, "unknown_1413ed340", fn_1413ed340 as *const (), Throws::Caught), // sub_1413ed340
    Replacement::draft_throws(0x1413ed530, "unknown_1413ed530", fn_1413ed530 as *const (), Throws::Caught), // sub_1413ed530
    Replacement::draft(0x1413ed720, "unknown_1413ed720", fn_1413ed720 as *const ()), // sub_1413ed720
    Replacement::draft(0x1413ed7e0, "unknown_1413ed7e0", fn_1413ed7e0 as *const ()), // sub_1413ed7e0
    Replacement::draft(0x1413ed8b0, "unknown_1413ed8b0", tpl_25927::<0x143882198, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x10, 0xe0, 0x14, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_1413ed8b0
    Replacement::draft_throws(0x1413ed990, "unknown_1413ed990", fn_1413ed990 as *const (), Throws::Discard), // sub_1413ed990
    Replacement::draft_throws(0x1413eda90, "unknown_1413eda90", fn_1413eda90 as *const (), Throws::Discard), // sub_1413eda90
    Replacement::draft(0x1413edc50, "unknown_1413edc50", tpl_13574::<0x1434154c8, 0x1, 0x14020f270, 0x4, 0x1434154e8, 0x8, 0x143415518, 0xc, 0x143415558> as *const ()), // sub_1413edc50
    Replacement::draft(0x1413edd00, "unknown_1413edd00", fn_1413edd00 as *const ()), // sub_1413edd00
    Replacement::draft(0x1413eddd0, "unknown_1413eddd0", tpl_8688::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1413f0ba0> as *const ()), // sub_1413eddd0
    Replacement::draft(0x1413ede40, "unknown_1413ede40", fn_1413ede40 as *const ()), // sub_1413ede40
    Replacement::draft(0x1413edf90, "unknown_1413edf90", fn_1413edf90 as *const ()), // sub_1413edf90
    Replacement::draft_throws(0x1413ee190, "unknown_1413ee190", fn_1413ee190 as *const (), Throws::Discard), // sub_1413ee190
    Replacement::draft(0x1413ee350, "unknown_1413ee350", fn_1413ee350 as *const ()), // sub_1413ee350
    Replacement::draft_throws(0x1413ee4f0, "unknown_1413ee4f0", fn_1413ee4f0 as *const (), Throws::Discard), // sub_1413ee4f0
    Replacement::draft(0x1413ee720, "unknown_1413ee720", fn_1413ee720 as *const ()), // sub_1413ee720
    Replacement::draft_throws(0x1413ee810, "unknown_1413ee810", fn_1413ee810 as *const (), Throws::Caught), // sub_1413ee810
    Replacement::draft_throws(0x1413eebf0, "unknown_1413eebf0", fn_1413eebf0 as *const (), Throws::Caught), // sub_1413eebf0
    Replacement::draft_throws(0x1413eef70, "unknown_1413eef70", fn_1413eef70 as *const (), Throws::Caught), // sub_1413eef70
    Replacement::draft(0x1413ef2e0, "unknown_1413ef2e0", fn_1413ef2e0 as *const ()), // sub_1413ef2e0
    Replacement::draft(0x1413ef460, "unknown_1413ef460", fn_1413ef460 as *const ()), // sub_1413ef460
    Replacement::draft(0x1413ef5e0, "unknown_1413ef5e0", fn_1413ef5e0 as *const ()), // sub_1413ef5e0
    Replacement::draft_throws(0x1413ef780, "unknown_1413ef780", fn_1413ef780 as *const (), Throws::Discard), // sub_1413ef780
    Replacement::draft_throws(0x1413ef8f0, "unknown_1413ef8f0", fn_1413ef8f0 as *const (), Throws::Discard), // sub_1413ef8f0
    Replacement::draft(0x1413efa20, "unknown_1413efa20", fn_1413efa20 as *const ()), // sub_1413efa20
    Replacement::draft(0x1413efac0, "unknown_1413efac0", fn_1413efac0 as *const ()), // sub_1413efac0
    Replacement::draft(0x1413efb80, "unknown_1413efb80", fn_1413efb80 as *const ()), // sub_1413efb80
    Replacement::draft(0x1413efc60, "unknown_1413efc60", fn_1413efc60 as *const ()), // sub_1413efc60
    Replacement::draft_throws(0x1413efd90, "unknown_1413efd90", tpl_9846::<0x143882174, 0x1438821a0, 0x1401e9fa0, 0x143881f88, 0x143881f80, 0x1401e9e70, 0xb8, 0x1413f0d30> as *const (), Throws::Discard), // sub_1413efd90
    Replacement::draft(0x1413efe20, "unknown_1413efe20", fn_1413efe20 as *const ()), // sub_1413efe20
    Replacement::draft_throws(0x1413eff30, "unknown_1413eff30", tpl_9846::<0x143882174, 0x1438821a0, 0x1401e9b60, 0x143881f88, 0x143881f80, 0x1401e9a20, 0xb8, 0x1413f0ee0> as *const (), Throws::Discard), // sub_1413eff30
    Replacement::draft(0x1413effc0, "unknown_1413effc0", fn_1413effc0 as *const ()), // sub_1413effc0
    Replacement::draft_throws(0x1413f0040, "unknown_1413f0040", fn_1413f0040 as *const (), Throws::Caught), // sub_1413f0040
    Replacement::draft_throws(0x1413f0100, "unknown_1413f0100", fn_1413f0100 as *const (), Throws::Caught), // sub_1413f0100
    Replacement::draft_throws(0x1413f01c0, "unknown_1413f01c0", fn_1413f01c0 as *const (), Throws::Caught), // sub_1413f01c0
    Replacement::draft(0x1413f0280, "unknown_1413f0280", fn_1413f0280 as *const ()), // sub_1413f0280
    Replacement::draft(0x1413f0380, "unknown_1413f0380", fn_1413f0380 as *const ()), // sub_1413f0380
    Replacement::draft(0x1413f0460, "unknown_1413f0460", fn_1413f0460 as *const ()), // sub_1413f0460
    Replacement::draft_throws(0x1413f0560, "unknown_1413f0560", fn_1413f0560 as *const (), Throws::Discard), // sub_1413f0560
    Replacement::draft_throws(0x1413f0640, "unknown_1413f0640", tpl_9858::<0x1438821a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80, 0xb8, 0x1434155a8, 0x1413f10d0> as *const (), Throws::Discard), // sub_1413f0640
    Replacement::draft(0x1413f0710, "unknown_1413f0710", tpl_84116::<0xb8, 0xc0, 0x1426cdcb0, 0x1413f4b20> as *const ()), // sub_1413f0710
    Replacement::draft(0x1413f0780, "unknown_1413f0780", tpl_84116::<0xb8, 0xc0, 0x1434155a8, 0x1413f4b70> as *const ()), // sub_1413f0780
    Replacement::draft(0x1413f07f0, "unknown_1413f07f0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1434155c8, 0x8> as *const ()), // sub_1413f07f0
    Replacement::draft(0x1413f0870, "unknown_1413f0870", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1434155c8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_1413f0870
    Replacement::draft_throws(0x1413f0910, "unknown_1413f0910", fn_1413f0910 as *const (), Throws::Caught), // sub_1413f0910
    Replacement::draft_throws(0x1413f09a0, "unknown_1413f09a0", tpl_649::<0x143882180, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_1413f09a0
    Replacement::draft(0x1413f0a20, "unknown_1413f0a20", fn_1413f0a20 as *const ()), // sub_1413f0a20
    Replacement::draft(0x1413f0aa0, "unknown_1413f0aa0", fn_1413f0aa0 as *const ()), // sub_1413f0aa0
    Replacement::draft(0x1413f0ba0, "unknown_1413f0ba0", fn_1413f0ba0 as *const ()), // sub_1413f0ba0
    Replacement::draft_throws(0x1413f0c30, "unknown_1413f0c30", tpl_838::<0x1413f25e0> as *const (), Throws::Discard), // sub_1413f0c30
    Replacement::draft_throws(0x1413f0c40, "unknown_1413f0c40", tpl_838::<0x1413f2a60> as *const (), Throws::Discard), // sub_1413f0c40
    Replacement::draft_throws(0x1413f0c50, "unknown_1413f0c50", tpl_838::<0x1413f2f00> as *const (), Throws::Discard), // sub_1413f0c50
    Replacement::draft_throws(0x1413f0c60, "unknown_1413f0c60", tpl_838::<0x1413f3350> as *const (), Throws::Discard), // sub_1413f0c60
    Replacement::draft_throws(0x1413f0c70, "unknown_1413f0c70", tpl_838::<0x1413f37c0> as *const (), Throws::Discard), // sub_1413f0c70
    Replacement::draft_throws(0x1413f0c80, "unknown_1413f0c80", tpl_838::<0x1413f3c00> as *const (), Throws::Discard), // sub_1413f0c80
    Replacement::draft_throws(0x1413f0c90, "unknown_1413f0c90", tpl_11264::<0x20, 0x8, 0x1413f4060> as *const (), Throws::Discard), // sub_1413f0c90
    Replacement::draft_throws(0x1413f0ce0, "unknown_1413f0ce0", tpl_11264::<0x20, 0x8, 0x1413f4130> as *const (), Throws::Discard), // sub_1413f0ce0
    Replacement::draft_throws(0x1413f0d30, "unknown_1413f0d30", fn_1413f0d30 as *const (), Throws::Caught), // sub_1413f0d30
    Replacement::draft_throws(0x1413f0ee0, "unknown_1413f0ee0", fn_1413f0ee0 as *const (), Throws::Caught), // sub_1413f0ee0
    Replacement::draft_throws(0x1413f10a0, "unknown_1413f10a0", tpl_838::<0x1413f4200> as *const (), Throws::Discard), // sub_1413f10a0
    Replacement::draft_throws(0x1413f10b0, "unknown_1413f10b0", tpl_838::<0x1413f44f0> as *const (), Throws::Discard), // sub_1413f10b0
    Replacement::draft_throws(0x1413f10c0, "unknown_1413f10c0", tpl_838::<0x1413f47b0> as *const (), Throws::Discard), // sub_1413f10c0
    Replacement::draft_throws(0x1413f10d0, "unknown_1413f10d0", tpl_8937::<0x20, 0x8, 0x1413f4a50> as *const (), Throws::Discard), // sub_1413f10d0
    Replacement::draft(0x1413f1120, "unknown_1413f1120", fn_1413f1120 as *const ()), // sub_1413f1120
    Replacement::draft(0x1413f1220, "unknown_1413f1220", fn_1413f1220 as *const ()), // sub_1413f1220
    Replacement::draft(0x1413f1320, "unknown_1413f1320", tpl_67717::<0x8, 0xc, 0x1413e5570, 0x1413f0ba0, 0x1413e4a10> as *const ()), // sub_1413f1320
    Replacement::draft_throws(0x1413f13f0, "unknown_1413f13f0", fn_1413f13f0 as *const (), Throws::Caught), // sub_1413f13f0
    Replacement::draft_throws(0x1413f16c0, "unknown_1413f16c0", fn_1413f16c0 as *const (), Throws::Caught), // sub_1413f16c0
    Replacement::draft_throws(0x1413f19a0, "unknown_1413f19a0", fn_1413f19a0 as *const (), Throws::Caught), // sub_1413f19a0
    Replacement::draft_throws(0x1413f1c70, "unknown_1413f1c70", fn_1413f1c70 as *const (), Throws::Caught), // sub_1413f1c70
    Replacement::draft(0x1413f1f50, "unknown_1413f1f50", fn_1413f1f50 as *const ()), // sub_1413f1f50
    Replacement::draft(0x1413f2200, "unknown_1413f2200", fn_1413f2200 as *const ()), // sub_1413f2200
    Replacement::draft_throws(0x1413f25e0, "unknown_1413f25e0", fn_1413f25e0 as *const (), Throws::Caught), // sub_1413f25e0
    Replacement::draft_throws(0x1413f2a60, "unknown_1413f2a60", fn_1413f2a60 as *const (), Throws::Caught), // sub_1413f2a60
    Replacement::draft_throws(0x1413f2f00, "unknown_1413f2f00", fn_1413f2f00 as *const (), Throws::Caught), // sub_1413f2f00
    Replacement::draft_throws(0x1413f3350, "unknown_1413f3350", fn_1413f3350 as *const (), Throws::Caught), // sub_1413f3350
    Replacement::draft_throws(0x1413f37c0, "unknown_1413f37c0", fn_1413f37c0 as *const (), Throws::Caught), // sub_1413f37c0
    Replacement::draft_throws(0x1413f3c00, "unknown_1413f3c00", fn_1413f3c00 as *const (), Throws::Caught), // sub_1413f3c00
    Replacement::draft_throws(0x1413f4060, "unknown_1413f4060", fn_1413f4060 as *const (), Throws::Caught), // sub_1413f4060
    Replacement::draft_throws(0x1413f4130, "unknown_1413f4130", fn_1413f4130 as *const (), Throws::Caught), // sub_1413f4130
    Replacement::draft_throws(0x1413f4200, "unknown_1413f4200", fn_1413f4200 as *const (), Throws::Caught), // sub_1413f4200
    Replacement::draft_throws(0x1413f44f0, "unknown_1413f44f0", fn_1413f44f0 as *const (), Throws::Caught), // sub_1413f44f0
    Replacement::draft_throws(0x1413f47b0, "unknown_1413f47b0", fn_1413f47b0 as *const (), Throws::Caught), // sub_1413f47b0
    Replacement::draft_throws(0x1413f4a50, "unknown_1413f4a50", fn_1413f4a50 as *const (), Throws::Caught), // sub_1413f4a50
    Replacement::draft(0x1413f4b20, "unknown_1413f4b20", tpl_25963::<0x1438796d0, 0x143415578, 0x143415538, 0x1401de410> as *const ()), // sub_1413f4b20
    Replacement::draft(0x1413f4b70, "unknown_1413f4b70", tpl_25963::<0x143879710, 0x143415578, 0x143415538, 0x1401de410> as *const ()), // sub_1413f4b70
    Replacement::draft_throws(0x1413f4bc0, "unknown_1413f4bc0", fn_1413f4bc0 as *const (), Throws::Caught), // sub_1413f4bc0
    Replacement::draft_throws(0x1413f5090, "unknown_1413f5090", fn_1413f5090 as *const (), Throws::Caught), // sub_1413f5090
    Replacement::draft_throws(0x1413f5560, "unknown_1413f5560", fn_1413f5560 as *const (), Throws::Caught), // sub_1413f5560
    Replacement::draft_throws(0x1413f59f0, "unknown_1413f59f0", fn_1413f59f0 as *const (), Throws::Caught), // sub_1413f59f0
    Replacement::draft_throws(0x1413f5e80, "unknown_1413f5e80", fn_1413f5e80 as *const (), Throws::Caught), // sub_1413f5e80
    Replacement::draft_throws(0x1413f6310, "unknown_1413f6310", fn_1413f6310 as *const (), Throws::Caught), // sub_1413f6310
    Replacement::draft_throws(0x1413f67a0, "unknown_1413f67a0", fn_1413f67a0 as *const (), Throws::Caught), // sub_1413f67a0
    Replacement::draft_throws(0x1413f6c70, "unknown_1413f6c70", fn_1413f6c70 as *const (), Throws::Caught), // sub_1413f6c70
    Replacement::draft_throws(0x1413f7100, "unknown_1413f7100", fn_1413f7100 as *const (), Throws::Caught), // sub_1413f7100
    Replacement::draft(0x1413f7590, "unknown_1413f7590", fn_1413f7590 as *const ()), // sub_1413f7590
    Replacement::draft_throws(0x1413f7600, "unknown_1413f7600", fn_1413f7600 as *const (), Throws::Caught), // sub_1413f7600
    Replacement::draft_throws(0x1413f7860, "unknown_1413f7860", fn_1413f7860 as *const (), Throws::Caught), // sub_1413f7860
    Replacement::draft_throws(0x1413f7ac0, "unknown_1413f7ac0", fn_1413f7ac0 as *const (), Throws::Caught), // sub_1413f7ac0
    Replacement::draft_throws(0x1413f7d20, "unknown_1413f7d20", fn_1413f7d20 as *const (), Throws::Caught), // sub_1413f7d20
    Replacement::draft_throws(0x1413f7f80, "unknown_1413f7f80", fn_1413f7f80 as *const (), Throws::Caught), // sub_1413f7f80
    Replacement::draft_throws(0x1413f81e0, "unknown_1413f81e0", fn_1413f81e0 as *const (), Throws::Caught), // sub_1413f81e0
    Replacement::draft_throws(0x1413f8440, "unknown_1413f8440", fn_1413f8440 as *const (), Throws::Caught), // sub_1413f8440
    Replacement::draft_throws(0x1413f8630, "unknown_1413f8630", fn_1413f8630 as *const (), Throws::Caught), // sub_1413f8630
    Replacement::draft_throws(0x1413f8820, "unknown_1413f8820", fn_1413f8820 as *const (), Throws::Caught), // sub_1413f8820
    Replacement::draft_throws(0x1413f8960, "unknown_1413f8960", fn_1413f8960 as *const (), Throws::Caught), // sub_1413f8960
    Replacement::draft_throws(0x1413f8aa0, "unknown_1413f8aa0", fn_1413f8aa0 as *const (), Throws::Caught), // sub_1413f8aa0
    Replacement::draft_throws(0x1413f8be0, "unknown_1413f8be0", fn_1413f8be0 as *const (), Throws::Caught), // sub_1413f8be0
    Replacement::draft_throws(0x1413f8dd0, "unknown_1413f8dd0", fn_1413f8dd0 as *const (), Throws::Caught), // sub_1413f8dd0
    Replacement::draft_throws(0x1413f9350, "unknown_1413f9350", fn_1413f9350 as *const (), Throws::Caught), // sub_1413f9350
    Replacement::draft_throws(0x1413f98d0, "unknown_1413f98d0", fn_1413f98d0 as *const (), Throws::Caught), // sub_1413f98d0
    Replacement::draft_throws(0x1413f9e10, "unknown_1413f9e10", fn_1413f9e10 as *const (), Throws::Caught), // sub_1413f9e10
    Replacement::draft_throws(0x1413fa350, "unknown_1413fa350", fn_1413fa350 as *const (), Throws::Caught), // sub_1413fa350
    Replacement::draft_throws(0x1413fa6a0, "unknown_1413fa6a0", fn_1413fa6a0 as *const (), Throws::Caught), // sub_1413fa6a0
    Replacement::draft_throws(0x1413fa9f0, "unknown_1413fa9f0", fn_1413fa9f0 as *const (), Throws::Caught), // sub_1413fa9f0
    Replacement::draft_throws(0x1413fae60, "unknown_1413fae60", fn_1413fae60 as *const (), Throws::Caught), // sub_1413fae60
    Replacement::draft_throws(0x1413fb290, "unknown_1413fb290", fn_1413fb290 as *const (), Throws::Caught), // sub_1413fb290
    Replacement::draft(0x1413fb5e0, "unknown_1413fb5e0", fn_1413fb5e0 as *const ()), // sub_1413fb5e0
    Replacement::draft(0x1413fb840, "unknown_1413fb840", fn_1413fb840 as *const ()), // sub_1413fb840
    Replacement::draft_throws(0x1413fb980, "std_1413fb980", fn_1413fb980 as *const (), Throws::Discard), // sub_1413fb980
    Replacement::draft(0x1413fbcd0, "std_1413fbcd0", fn_1413fbcd0 as *const ()), // sub_1413fbcd0
    Replacement::draft(0x1413fbe00, "std_1413fbe00", fn_1413fbe00 as *const ()), // sub_1413fbe00
    Replacement::draft(0x1413fbf40, "std_1413fbf40", tpl_170::<0x1413fc650> as *const ()), // sub_1413fbf40
    Replacement::draft(0x1413fbf50, "std_1413fbf50", tpl_170::<0x1413fc720> as *const ()), // sub_1413fbf50
    Replacement::draft(0x1413fbf60, "std_1413fbf60", fn_1413fbf60 as *const ()), // sub_1413fbf60
    Replacement::draft(0x1413fc020, "std_1413fc020", fn_1413fc020 as *const ()), // sub_1413fc020
    Replacement::draft_throws(0x1413fc0b0, "std_1413fc0b0", fn_1413fc0b0 as *const (), Throws::Discard), // sub_1413fc0b0
    Replacement::draft(0x1413fc170, "std_1413fc170", fn_1413fc170 as *const ()), // sub_1413fc170
    Replacement::draft_throws(0x1413fc330, "std_1413fc330", fn_1413fc330 as *const (), Throws::Discard), // sub_1413fc330
    Replacement::draft(0x1413fc650, "unknown_1413fc650", fn_1413fc650 as *const ()), // sub_1413fc650
    Replacement::draft(0x1413fc720, "unknown_1413fc720", fn_1413fc720 as *const ()), // sub_1413fc720
    Replacement::draft(0x1413fc7f0, "unknown_1413fc7f0", fn_1413fc7f0 as *const ()), // sub_1413fc7f0
    Replacement::draft(0x1413fc930, "unknown_1413fc930", fn_1413fc930 as *const ()), // sub_1413fc930
    Replacement::draft(0x1413fcbc0, "unknown_1413fcbc0", fn_1413fcbc0 as *const ()), // sub_1413fcbc0
    Replacement::draft(0x1413fcf40, "unknown_1413fcf40", fn_1413fcf40 as *const ()), // sub_1413fcf40
    Replacement::draft(0x1413fd2e0, "unknown_1413fd2e0", fn_1413fd2e0 as *const ()), // sub_1413fd2e0
    Replacement::draft(0x1413fd4e0, "unknown_1413fd4e0", fn_1413fd4e0 as *const ()), // sub_1413fd4e0
    Replacement::draft(0x1413fd690, "unknown_1413fd690", fn_1413fd690 as *const ()), // sub_1413fd690
    Replacement::draft(0x1413fd990, "unknown_1413fd990", fn_1413fd990 as *const ()), // sub_1413fd990
    Replacement::draft(0x1413fdd10, "unknown_1413fdd10", fn_1413fdd10 as *const ()), // sub_1413fdd10
    Replacement::draft(0x1413fddf0, "unknown_1413fddf0", fn_1413fddf0 as *const ()), // sub_1413fddf0
    Replacement::draft(0x1413fdf00, "unknown_1413fdf00", fn_1413fdf00 as *const ()), // sub_1413fdf00
    Replacement::draft(0x1413fe030, "unknown_1413fe030", fn_1413fe030 as *const ()), // sub_1413fe030
    Replacement::draft(0x1413fe0a0, "unknown_1413fe0a0", fn_1413fe0a0 as *const ()), // sub_1413fe0a0
    Replacement::draft_throws(0x1413fe180, "unknown_1413fe180", fn_1413fe180 as *const (), Throws::Caught), // sub_1413fe180
    Replacement::draft(0x1413fe2b0, "cohtml_1413fe2b0", fn_1413fe2b0 as *const ()), // cohtml::EventHandler<int (ui::VideoController::*)(const r::InplaceString<256>&)>::cohtml::EventHandler<int (ui::VideoCon
    Replacement::draft(0x1413fe350, "cohtml_1413fe350", fn_1413fe350 as *const ()), // cohtml::EventHandler<bool (ui::VideoController::*)(const r::InplaceString<256>&)>::cohtml::EventHandler<bool (ui::VideoC
    Replacement::draft(0x1413fe3f0, "cohtml_1413fe3f0", fn_1413fe3f0 as *const ()), // cohtml::EventHandler<void (ui::VideoController::*)(const r::InplaceString<256>&,int)>::cohtml::EventHandler<void (ui::Vi
    Replacement::draft(0x1413fe4a0, "cohtml_1413fe4a0", fn_1413fe4a0 as *const ()), // cohtml::EventHandler<void (ui::VideoController::*)(const r::InplaceString<256>&)>::cohtml::EventHandler<void (ui::VideoC
    Replacement::draft_throws(0x1413fe520, "unknown_1413fe520", fn_1413fe520 as *const (), Throws::Discard), // sub_1413fe520
    Replacement::draft_throws(0x1413fe5b0, "std_1413fe5b0", fn_1413fe5b0 as *const (), Throws::Discard), // sub_1413fe5b0
    Replacement::draft(0x1413fe8a0, "renoir_1413fe8a0", tpl_1674::<0x1434168e0> as *const ()), // sub_1413fe8a0
    Replacement::draft(0x1413fe8b0, "renoir_1413fe8b0", tpl_893::<0x1434168e0, 0x141834d30> as *const ()), // renoir::RendererBackend::renoir::RendererBackend::vfunc_0
    Replacement::draft(0x1413fe8e0, "ui_1413fe8e0", fn_1413fe8e0 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_0
    Replacement::draft_throws(0x1413fe9d0, "ui_1413fe9d0", fn_1413fe9d0 as *const (), Throws::Caught), // sub_1413fe9d0
    Replacement::draft(0x1413fea60, "ui_1413fea60", fn_1413fea60 as *const ()), // sub_1413fea60
    Replacement::draft(0x1413fea90, "ui_1413fea90", fn_1413fea90 as *const ()), // sub_1413fea90
    Replacement::draft(0x1413feb40, "ui_1413feb40", fn_1413feb40 as *const ()), // sub_1413feb40
    Replacement::draft(0x1413fec10, "ui_1413fec10", tpl_26006::<0x1413ff0d0, 0x10, 0x8, 0x1413ffd00, 0x18, 0x141400510> as *const ()), // sub_1413fec10
    Replacement::draft(0x1413fecb0, "ui_1413fecb0", tpl_26006::<0x1413ff0d0, 0x10, 0x8, 0x1413ffd00, 0x18, 0x1414003c0> as *const ()), // sub_1413fecb0
    Replacement::draft(0x1413fed50, "ui_1413fed50", fn_1413fed50 as *const ()), // sub_1413fed50
    Replacement::draft(0x1413fedf0, "ui_1413fedf0", fn_1413fedf0 as *const ()), // sub_1413fedf0
    Replacement::draft(0x1413feec0, "ui_1413feec0", fn_1413feec0 as *const ()), // sub_1413feec0
    Replacement::draft(0x1413fef90, "ui_1413fef90", tpl_84176::<0x10, 0x8, 0x18, 0x1413ff070, 0xffffffffffffffff> as *const ()), // sub_1413fef90
    Replacement::draft(0x1413ff070, "ui_1413ff070", tpl_26010::<0x8, 0x18, 0x10, 0x141400660> as *const ()), // sub_1413ff070
    Replacement::draft(0x1413ff0d0, "ui_1413ff0d0", fn_1413ff0d0 as *const ()), // sub_1413ff0d0
    Replacement::draft(0x1413ff1b0, "ui_1413ff1b0", fn_1413ff1b0 as *const ()), // sub_1413ff1b0
    Replacement::draft(0x1413ff210, "ui_1413ff210", fn_1413ff210 as *const ()), // sub_1413ff210
    Replacement::draft(0x1413ff2f0, "ui_1413ff2f0", fn_1413ff2f0 as *const ()), // sub_1413ff2f0
    Replacement::draft(0x1413ff350, "ui_1413ff350", tpl_84176::<0x10, 0x8, 0x18, 0x1413ff430, 0xffffffffffffffff> as *const ()), // sub_1413ff350
    Replacement::draft(0x1413ff430, "ui_1413ff430", tpl_26010::<0x8, 0x18, 0x10, 0x1414010d0> as *const ()), // sub_1413ff430
    Replacement::draft(0x1413ff490, "ui_1413ff490", tpl_84176::<0x10, 0x8, 0x18, 0x1413ff570, 0xffffffffffffffff> as *const ()), // sub_1413ff490
    Replacement::draft(0x1413ff570, "ui_1413ff570", tpl_26010::<0x8, 0x18, 0x10, 0x141401380> as *const ()), // sub_1413ff570
    Replacement::draft(0x1413ff5d0, "ui_1413ff5d0", fn_1413ff5d0 as *const ()), // sub_1413ff5d0
    Replacement::draft(0x1413ff6b0, "ui_1413ff6b0", fn_1413ff6b0 as *const ()), // sub_1413ff6b0
    Replacement::draft(0x1413ff710, "ui_1413ff710", fn_1413ff710 as *const ()), // sub_1413ff710
    Replacement::draft(0x1413ff7f0, "ui_1413ff7f0", fn_1413ff7f0 as *const ()), // sub_1413ff7f0
    Replacement::draft(0x1413ff850, "ui_1413ff850", fn_1413ff850 as *const ()), // sub_1413ff850
    Replacement::draft(0x1413ff8f0, "ui_1413ff8f0", fn_1413ff8f0 as *const ()), // sub_1413ff8f0
    Replacement::draft(0x1413ff9e0, "ui_1413ff9e0", fn_1413ff9e0 as *const ()), // sub_1413ff9e0
    Replacement::draft(0x1413ffad0, "ui_1413ffad0", fn_1413ffad0 as *const ()), // sub_1413ffad0
    Replacement::draft(0x1413ffbc0, "ui_1413ffbc0", fn_1413ffbc0 as *const ()), // sub_1413ffbc0
    Replacement::draft(0x1413ffd00, "ui_1413ffd00", fn_1413ffd00 as *const ()), // sub_1413ffd00
    Replacement::draft(0x1413ffe40, "ui_1413ffe40", fn_1413ffe40 as *const ()), // sub_1413ffe40
    Replacement::draft(0x1413fffe0, "ui_1413fffe0", fn_1413fffe0 as *const ()), // sub_1413fffe0
    Replacement::draft(0x141400180, "ui_141400180", fn_141400180 as *const ()), // sub_141400180
    Replacement::draft(0x1414003c0, "ui_1414003c0", fn_1414003c0 as *const ()), // sub_1414003c0
    Replacement::draft(0x141400510, "ui_141400510", fn_141400510 as *const ()), // sub_141400510
    Replacement::draft(0x141400660, "ui_141400660", tpl_26028::<0x141402300, 0x141400660, 0x1414020a0, 0x1414021f0, 0x141404370> as *const ()), // sub_141400660
    Replacement::draft(0x1414008a0, "ui_1414008a0", fn_1414008a0 as *const ()), // sub_1414008a0
    Replacement::draft(0x141400cb0, "ui_141400cb0", fn_141400cb0 as *const ()), // sub_141400cb0
    Replacement::draft(0x1414010d0, "ui_1414010d0", fn_1414010d0 as *const ()), // sub_1414010d0
    Replacement::draft(0x141401380, "ui_141401380", tpl_26028::<0x1414035d0, 0x141401380, 0x141403390, 0x1414034c0, 0x141404370> as *const ()), // sub_141401380
    Replacement::draft(0x1414015c0, "ui_1414015c0", fn_1414015c0 as *const ()), // sub_1414015c0
    Replacement::draft(0x141401910, "ui_141401910", fn_141401910 as *const ()), // sub_141401910
    Replacement::draft(0x141401c60, "ui_141401c60", fn_141401c60 as *const ()), // sub_141401c60
    Replacement::draft(0x141401de0, "ui_141401de0", fn_141401de0 as *const ()), // sub_141401de0
    Replacement::draft(0x1414020a0, "ui_1414020a0", fn_1414020a0 as *const ()), // sub_1414020a0
    Replacement::draft(0x1414021f0, "ui_1414021f0", fn_1414021f0 as *const ()), // sub_1414021f0
    Replacement::draft(0x141402300, "ui_141402300", fn_141402300 as *const ()), // sub_141402300
    Replacement::draft(0x1414026a0, "ui_1414026a0", fn_1414026a0 as *const ()), // sub_1414026a0
    Replacement::draft(0x141402960, "ui_141402960", fn_141402960 as *const ()), // sub_141402960
    Replacement::draft(0x141402c20, "ui_141402c20", fn_141402c20 as *const ()), // sub_141402c20
    Replacement::draft(0x141402e30, "ui_141402e30", fn_141402e30 as *const ()), // sub_141402e30
    Replacement::draft(0x141402ff0, "ui_141402ff0", fn_141402ff0 as *const ()), // sub_141402ff0
    Replacement::draft(0x141403390, "ui_141403390", fn_141403390 as *const ()), // sub_141403390
    Replacement::draft(0x1414034c0, "ui_1414034c0", fn_1414034c0 as *const ()), // sub_1414034c0
    Replacement::draft(0x1414035d0, "ui_1414035d0", fn_1414035d0 as *const ()), // sub_1414035d0
    Replacement::draft(0x141403970, "ui_141403970", fn_141403970 as *const ()), // sub_141403970
    Replacement::draft(0x141403b10, "ui_141403b10", fn_141403b10 as *const ()), // sub_141403b10
    Replacement::draft(0x141403e70, "ui_141403e70", fn_141403e70 as *const ()), // sub_141403e70
    Replacement::draft(0x141404010, "ui_141404010", fn_141404010 as *const ()), // sub_141404010
    Replacement::draft(0x141404370, "ui_141404370", fn_141404370 as *const ()), // sub_141404370
    Replacement::draft(0x141404470, "ui_141404470", fn_141404470 as *const ()), // sub_141404470
    Replacement::draft(0x1414045a0, "ui_1414045a0", fn_1414045a0 as *const ()), // sub_1414045a0
    Replacement::draft(0x141404780, "ui_141404780", fn_141404780 as *const ()), // sub_141404780
    Replacement::draft(0x1414048c0, "ui_1414048c0", fn_1414048c0 as *const ()), // sub_1414048c0
    Replacement::draft(0x141404aa0, "ui_141404aa0", fn_141404aa0 as *const ()), // sub_141404aa0
    Replacement::draft(0x141404c60, "ui_141404c60", fn_141404c60 as *const ()), // sub_141404c60
    Replacement::draft(0x141404de0, "ui_141404de0", fn_141404de0 as *const ()), // sub_141404de0
    Replacement::draft(0x141404fd0, "ui_141404fd0", fn_141404fd0 as *const ()), // sub_141404fd0
    Replacement::draft(0x141405130, "ui_141405130", fn_141405130 as *const ()), // sub_141405130
    Replacement::draft(0x141405320, "ui_141405320", fn_141405320 as *const ()), // sub_141405320
    Replacement::draft(0x1414053d0, "ui_1414053d0", fn_1414053d0 as *const ()), // sub_1414053d0
    Replacement::draft(0x141405490, "ui_141405490", fn_141405490 as *const ()), // sub_141405490
    Replacement::draft(0x141405550, "ui_141405550", fn_141405550 as *const ()), // sub_141405550
    Replacement::draft(0x141405650, "ui_141405650", fn_141405650 as *const ()), // sub_141405650
    Replacement::draft_throws(0x141405750, "ui_141405750", fn_141405750 as *const (), Throws::Discard), // ui::UIRenderer::ui::UIRenderer::vfunc_4
    Replacement::draft(0x141405890, "ui_141405890", fn_141405890 as *const ()), // sub_141405890
    Replacement::draft(0x141405940, "ui_141405940", fn_141405940 as *const ()), // sub_141405940
    Replacement::draft_throws(0x141405a60, "ui_141405a60", fn_141405a60 as *const (), Throws::Discard), // sub_141405a60
    Replacement::draft(0x141405c00, "ui_141405c00", fn_141405c00 as *const ()), // sub_141405c00
    Replacement::draft(0x141405e60, "ui_141405e60", fn_141405e60 as *const ()), // sub_141405e60
    Replacement::draft(0x141405ee0, "ui_141405ee0", fn_141405ee0 as *const ()), // sub_141405ee0
    Replacement::draft_throws(0x141405f80, "ui_141405f80", fn_141405f80 as *const (), Throws::Caught), // sub_141405f80
    Replacement::draft_throws(0x1414060b0, "ui_1414060b0", fn_1414060b0 as *const (), Throws::Discard), // sub_1414060b0
    Replacement::draft(0x141406130, "ui_141406130", fn_141406130 as *const ()), // sub_141406130
    Replacement::draft(0x141406200, "ui_141406200", fn_141406200 as *const ()), // sub_141406200
    Replacement::draft(0x141406420, "ui_141406420", fn_141406420 as *const ()), // sub_141406420
    Replacement::draft(0x141406600, "ui_141406600", fn_141406600 as *const ()), // sub_141406600
    Replacement::draft(0x141406af0, "ui_141406af0", fn_141406af0 as *const ()), // sub_141406af0
    Replacement::draft(0x141406b60, "ui_141406b60", fn_141406b60 as *const ()), // sub_141406b60
    Replacement::draft(0x141406d20, "ui_141406d20", fn_141406d20 as *const ()), // sub_141406d20
    Replacement::draft(0x141407010, "ui_141407010", fn_141407010 as *const ()), // sub_141407010
    Replacement::draft(0x1414074d0, "ui_1414074d0", fn_1414074d0 as *const ()), // sub_1414074d0
    Replacement::draft(0x141407510, "ui_141407510", fn_141407510 as *const ()), // sub_141407510
    Replacement::draft(0x141407580, "ui_141407580", fn_141407580 as *const ()), // sub_141407580
    Replacement::draft(0x141407810, "ui_141407810", fn_141407810 as *const ()), // sub_141407810
    Replacement::draft(0x141407a20, "ui_141407a20", fn_141407a20 as *const ()), // sub_141407a20
    Replacement::draft(0x141407b30, "ui_141407b30", fn_141407b30 as *const ()), // sub_141407b30
    Replacement::draft(0x141407c40, "ui_141407c40", tpl_22001::<0x143872ac0, 0x143872ad8, 0x143872ae8, 0x143872ad0, 0x141407dc0, 0x1414081a0> as *const ()), // sub_141407c40
    Replacement::draft(0x141407dc0, "ui_141407dc0", fn_141407dc0 as *const ()), // sub_141407dc0
    Replacement::draft(0x1414081a0, "ui_1414081a0", fn_1414081a0 as *const ()), // sub_1414081a0
    Replacement::draft(0x1414085b0, "ui_1414085b0", fn_1414085b0 as *const ()), // sub_1414085b0
    Replacement::draft(0x141408620, "ui_141408620", fn_141408620 as *const ()), // sub_141408620
    Replacement::draft(0x1414086c0, "ui_1414086c0", fn_1414086c0 as *const ()), // sub_1414086c0
    Replacement::draft(0x141408780, "ui_141408780", fn_141408780 as *const ()), // sub_141408780
    Replacement::draft_throws(0x141408900, "ui_141408900", fn_141408900 as *const (), Throws::Discard), // sub_141408900
    Replacement::draft(0x141408a50, "ui_141408a50", tpl_26083::<0x1438821c0, 0x14243b0b0, 0x338, 0x20, 0x108, 0x40, 0x10c, 0x110, 0x114, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_141408a50
    Replacement::draft_throws(0x141408b10, "ui_141408b10", fn_141408b10 as *const (), Throws::Discard), // sub_141408b10
    Replacement::draft(0x141408c90, "ui_141408c90", fn_141408c90 as *const ()), // sub_141408c90
    Replacement::draft_throws(0x141408d10, "ui_141408d10", fn_141408d10 as *const (), Throws::Caught), // sub_141408d10
    Replacement::draft_throws(0x141408e90, "ui_141408e90", fn_141408e90 as *const (), Throws::Caught), // sub_141408e90
    Replacement::draft_throws(0x141409010, "ui_141409010", fn_141409010 as *const (), Throws::Caught), // sub_141409010
    Replacement::draft(0x141409190, "ui_141409190", fn_141409190 as *const ()), // sub_141409190
    Replacement::draft(0x141409270, "ui_141409270", fn_141409270 as *const ()), // sub_141409270
    Replacement::draft(0x141409330, "ui_141409330", tpl_26083::<0x1438821c0, 0x14243b0b0, 0x338, 0xc0, 0x108, 0xe0, 0x10c, 0x110, 0x114, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_141409330
    Replacement::draft_throws(0x141409400, "ui_141409400", fn_141409400 as *const (), Throws::Discard), // sub_141409400
    Replacement::draft_throws(0x1414094e0, "ui_1414094e0", fn_1414094e0 as *const (), Throws::Discard), // sub_1414094e0
    Replacement::draft(0x141409660, "ui_141409660", fn_141409660 as *const ()), // sub_141409660
    Replacement::draft(0x141409720, "ui_141409720", fn_141409720 as *const ()), // sub_141409720
    Replacement::draft(0x141409800, "ui_141409800", fn_141409800 as *const ()), // sub_141409800
    Replacement::draft(0x1414098e0, "ui_1414098e0", fn_1414098e0 as *const ()), // sub_1414098e0
    Replacement::draft_throws(0x141409a90, "ui_141409a90", fn_141409a90 as *const (), Throws::Discard), // sub_141409a90
    Replacement::draft(0x141409ba0, "ui_141409ba0", fn_141409ba0 as *const ()), // sub_141409ba0
    Replacement::draft_throws(0x141409ca0, "ui_141409ca0", fn_141409ca0 as *const (), Throws::Discard), // sub_141409ca0
    Replacement::draft(0x141409dd0, "ui_141409dd0", fn_141409dd0 as *const ()), // sub_141409dd0
    Replacement::draft_throws(0x141409eb0, "ui_141409eb0", fn_141409eb0 as *const (), Throws::Caught), // sub_141409eb0
    Replacement::draft_throws(0x141409fe0, "ui_141409fe0", fn_141409fe0 as *const (), Throws::Caught), // sub_141409fe0
    Replacement::draft_throws(0x14140a110, "ui_14140a110", fn_14140a110 as *const (), Throws::Caught), // sub_14140a110
    Replacement::draft(0x14140a240, "ui_14140a240", fn_14140a240 as *const ()), // sub_14140a240
    Replacement::draft(0x14140a360, "ui_14140a360", fn_14140a360 as *const ()), // sub_14140a360
    Replacement::draft(0x14140a460, "ui_14140a460", fn_14140a460 as *const ()), // sub_14140a460
    Replacement::draft_throws(0x14140a570, "ui_14140a570", fn_14140a570 as *const (), Throws::Discard), // sub_14140a570
    Replacement::draft_throws(0x14140a670, "ui_14140a670", fn_14140a670 as *const (), Throws::Discard), // sub_14140a670
    Replacement::draft(0x14140a7a0, "ui_14140a7a0", fn_14140a7a0 as *const ()), // sub_14140a7a0
    Replacement::draft_throws(0x14140a8a0, "ui_14140a8a0", fn_14140a8a0 as *const (), Throws::Discard), // sub_14140a8a0
    Replacement::draft(0x14140a9d0, "ui_14140a9d0", fn_14140a9d0 as *const ()), // sub_14140a9d0
    Replacement::draft_throws(0x14140aad0, "ui_14140aad0", fn_14140aad0 as *const (), Throws::Discard), // sub_14140aad0
    Replacement::draft(0x14140ac20, "ui_14140ac20", fn_14140ac20 as *const ()), // sub_14140ac20
    Replacement::draft_throws(0x14140acc0, "ui_14140acc0", fn_14140acc0 as *const (), Throws::Caught), // sub_14140acc0
    Replacement::draft_throws(0x14140ae00, "ui_14140ae00", fn_14140ae00 as *const (), Throws::Caught), // sub_14140ae00
    Replacement::draft_throws(0x14140af40, "ui_14140af40", fn_14140af40 as *const (), Throws::Caught), // sub_14140af40
    Replacement::draft(0x14140b080, "ui_14140b080", fn_14140b080 as *const ()), // sub_14140b080
    Replacement::draft(0x14140b150, "ui_14140b150", fn_14140b150 as *const ()), // sub_14140b150
    Replacement::draft(0x14140b240, "ui_14140b240", fn_14140b240 as *const ()), // sub_14140b240
    Replacement::draft_throws(0x14140b340, "ui_14140b340", fn_14140b340 as *const (), Throws::Discard), // sub_14140b340
    Replacement::draft_throws(0x14140b440, "ui_14140b440", fn_14140b440 as *const (), Throws::Discard), // sub_14140b440
    Replacement::draft(0x14140b570, "ui_14140b570", fn_14140b570 as *const ()), // sub_14140b570
    Replacement::draft_throws(0x14140b670, "ui_14140b670", fn_14140b670 as *const (), Throws::Caught), // sub_14140b670
    Replacement::draft_throws(0x14140b820, "ui_14140b820", fn_14140b820 as *const (), Throws::Caught), // sub_14140b820
    Replacement::draft(0x14140ba50, "ui_14140ba50", fn_14140ba50 as *const ()), // sub_14140ba50
    Replacement::draft(0x14140bb30, "ui_14140bb30", fn_14140bb30 as *const ()), // sub_14140bb30
    Replacement::draft_throws(0x14140bd20, "ui_14140bd20", fn_14140bd20 as *const (), Throws::Caught), // sub_14140bd20
    Replacement::draft_throws(0x14140c000, "ui_14140c000", fn_14140c000 as *const (), Throws::Caught), // sub_14140c000
    Replacement::draft_throws(0x14140c2b0, "ui_14140c2b0", fn_14140c2b0 as *const (), Throws::Caught), // sub_14140c2b0
    Replacement::draft_throws(0x14140c550, "ui_14140c550", fn_14140c550 as *const (), Throws::Caught), // sub_14140c550
    Replacement::draft(0x14140c620, "ui_14140c620", fn_14140c620 as *const ()), // sub_14140c620
    Replacement::draft(0x14140c710, "ui_14140c710", fn_14140c710 as *const ()), // sub_14140c710
    Replacement::draft(0x14140c760, "ui_14140c760", fn_14140c760 as *const ()), // sub_14140c760
    Replacement::draft_throws(0x14140c7a0, "ui_14140c7a0", fn_14140c7a0 as *const (), Throws::Caught), // sub_14140c7a0
    Replacement::draft_throws(0x14140cc70, "ui_14140cc70", fn_14140cc70 as *const (), Throws::Caught), // sub_14140cc70
    Replacement::draft_throws(0x14140d100, "ui_14140d100", fn_14140d100 as *const (), Throws::Caught), // sub_14140d100
    Replacement::draft_throws(0x14140d590, "ui_14140d590", fn_14140d590 as *const (), Throws::Caught), // sub_14140d590
    Replacement::draft_throws(0x14140d6d0, "ui_14140d6d0", fn_14140d6d0 as *const (), Throws::Caught), // sub_14140d6d0
    Replacement::draft_throws(0x14140d810, "ui_14140d810", fn_14140d810 as *const (), Throws::Caught), // sub_14140d810
    Replacement::draft_throws(0x14140d950, "ui_14140d950", fn_14140d950 as *const (), Throws::Caught), // sub_14140d950
    Replacement::draft_throws(0x14140db40, "ui_14140db40", fn_14140db40 as *const (), Throws::Caught), // sub_14140db40
    Replacement::draft_throws(0x14140dfb0, "ui_14140dfb0", fn_14140dfb0 as *const (), Throws::Caught), // sub_14140dfb0
    Replacement::draft_throws(0x14140e3e0, "ui_14140e3e0", fn_14140e3e0 as *const (), Throws::Caught), // sub_14140e3e0
    Replacement::draft_throws(0x14140e730, "ui_14140e730", tpl_1276::<0x140fd8970, 0x1437fd950, 0x14183acce, 0x14140e752> as *const (), Throws::Discard), // sub_14140e730
    Replacement::draft(0x14140e760, "ui_14140e760", fn_14140e760 as *const ()), // ui::TextTransformationManager::ui::TextTransformationManager::vfunc_0
    Replacement::draft(0x14140e7a0, "std_14140e7a0", fn_14140e7a0 as *const ()), // std::codecvt_utf8<char32_t,1114111,0>::std::codecvt_utf8<char32_t,1114111,0>::vfunc_9
    Replacement::draft(0x14140e910, "std_14140e910", fn_14140e910 as *const ()), // std::codecvt_utf8<char32_t,1114111,0>::std::codecvt_utf8<char32_t,1114111,0>::vfunc_8
    Replacement::draft(0x14140e920, "std_14140e920", fn_14140e920 as *const ()), // std::codecvt_utf8<char32_t,1114111,0>::std::codecvt_utf8<char32_t,1114111,0>::vfunc_7
    Replacement::draft(0x14140ea70, "std_14140ea70", fn_14140ea70 as *const ()), // std::codecvt_utf8<char32_t,1114111,0>::std::codecvt_utf8<char32_t,1114111,0>::vfunc_6
    Replacement::draft_throws(0x14140ebd0, "std_14140ebd0", fn_14140ebd0 as *const (), Throws::Caught), // sub_14140ebd0
    Replacement::draft(0x14140f120, "std_14140f120", fn_14140f120 as *const ()), // sub_14140f120
    Replacement::draft(0x14140f180, "std_14140f180", fn_14140f180 as *const ()), // sub_14140f180
    Replacement::draft(0x14140f1f0, "std_14140f1f0", fn_14140f1f0 as *const ()), // std::codecvt_utf8<char32_t,1114111,0>::std::codecvt_utf8<char32_t,1114111,0>::vfunc_0
    Replacement::draft(0x14140f230, "std_14140f230", fn_14140f230 as *const ()), // std::wstring_convert<std::codecvt_utf8<char32_t,1114111,0>,char32_t,std::allocator<char32_t>,std::allocator<char>>::std:
    Replacement::draft_throws(0x14140f2a0, "std_14140f2a0", fn_14140f2a0 as *const (), Throws::Caught), // sub_14140f2a0
    Replacement::draft_throws(0x14140f850, "std_14140f850", fn_14140f850 as *const (), Throws::Discard), // sub_14140f850
    Replacement::draft_throws(0x14140f950, "unknown_14140f950", fn_14140f950 as *const (), Throws::Discard), // sub_14140f950
    Replacement::draft_throws(0x14140fac0, "unknown_14140fac0", fn_14140fac0 as *const (), Throws::Discard), // sub_14140fac0
    Replacement::draft(0x14140fc50, "renoir_14140fc50", tpl_893::<0x143417390, 0x141834d30> as *const ()), // renoir::ISubLayerCompositor::renoir::ISubLayerCompositor::vfunc_0
    Replacement::draft(0x14140fc80, "ui_14140fc80", fn_14140fc80 as *const ()), // ui::BlobResourceHandler::ui::BlobResourceHandler::vfunc_0
    Replacement::draft(0x14140fd50, "ui_14140fd50", fn_14140fd50 as *const ()), // ui::SublayerCompositor::ui::SublayerCompositor::vfunc_1
    Replacement::draft(0x14140ff80, "ui_14140ff80", tpl_26135::<0x142409b50, 0x1401df680, 0x14183acc8, 0x142408f50, 0x14183acbc, 0x14397ff58, 0x14240aa00, 0x1414115e0, 0x14240ab10> as *const ()), // ui::SublayerCompositor::ui::SublayerCompositor::vfunc_2
    Replacement::draft(0x141410240, "ui_141410240", fn_141410240 as *const ()), // sub_141410240
    Replacement::draft(0x141410300, "ui_141410300", tpl_26135::<0x142409b50, 0x1401df680, 0x14183acc8, 0x142408f50, 0x14183acbc, 0x14397ff58, 0x14240aa00, 0x141411780, 0x14240ab10> as *const ()), // ui::SublayerCompositor::ui::SublayerCompositor::vfunc_5
    Replacement::draft(0x1414105c0, "ui_1414105c0", fn_1414105c0 as *const ()), // ui::PageContent::ui::PageContent::vfunc_6
    Replacement::draft(0x141410670, "ui_141410670", fn_141410670 as *const ()), // ui::PageContent::ui::PageContent::vfunc_0
    Replacement::draft_throws(0x1414106b0, "ui_1414106b0", fn_1414106b0 as *const (), Throws::Discard), // ui::System::ui::System::vfunc_0
    Replacement::draft(0x1414106f0, "ui_1414106f0", tpl_2289::<0xf, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_1414106f0
    Replacement::draft(0x141410740, "ui_141410740", fn_141410740 as *const ()), // sub_141410740
    Replacement::draft(0x141410910, "ui_141410910", fn_141410910 as *const ()), // sub_141410910
    Replacement::draft(0x1414109f0, "ui_1414109f0", fn_1414109f0 as *const ()), // sub_1414109f0
    Replacement::draft(0x141410a50, "ui_141410a50", fn_141410a50 as *const ()), // sub_141410a50
    Replacement::draft(0x141410b40, "ui_141410b40", tpl_26143::<0x8, 0x10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142646cc8, 0x1434173c8> as *const ()), // sub_141410b40
    Replacement::draft(0x141410bd0, "ui_141410bd0", fn_141410bd0 as *const ()), // sub_141410bd0
    Replacement::draft(0x141410d00, "ui_141410d00", tpl_26143::<0x8, 0x10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142646d30, 0x143417410> as *const ()), // sub_141410d00
    Replacement::draft(0x141410d90, "ui_141410d90", tpl_26143::<0x8, 0x10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142646c90, 0x143417558> as *const ()), // sub_141410d90
    Replacement::draft(0x141410e20, "ui_141410e20", fn_141410e20 as *const ()), // sub_141410e20
    Replacement::draft(0x141410ee0, "ui_141410ee0", tpl_26143::<0x8, 0x10, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142646d78, 0x143417220> as *const ()), // sub_141410ee0
    Replacement::draft(0x141410f70, "ui_141410f70", fn_141410f70 as *const ()), // sub_141410f70
    Replacement::draft(0x141411050, "ui_141411050", fn_141411050 as *const ()), // sub_141411050
    Replacement::draft(0x141411150, "ui_141411150", fn_141411150 as *const ()), // sub_141411150
    Replacement::draft(0x1414112c0, "ui_1414112c0", fn_1414112c0 as *const ()), // sub_1414112c0
    Replacement::draft(0x141411330, "ui_141411330", tpl_26150::<0x142646c98, 0x141834d30> as *const ()), // ui::PerformanceHandler::ui::PerformanceHandler::vfunc_0
    Replacement::draft(0x141411370, "ui_141411370", tpl_26150::<0x142646d80, 0x141834d30> as *const ()), // ui::LocalizationManager::ui::LocalizationManager::vfunc_0
    Replacement::draft(0x1414113b0, "ui_1414113b0", tpl_26150::<0x142646d70, 0x141834d30> as *const ()), // ui::Logger::ui::Logger::vfunc_0
    Replacement::draft(0x1414113f0, "ui_1414113f0", fn_1414113f0 as *const ()), // ui::FileSystemReader::ui::FileSystemReader::vfunc_0
    Replacement::draft(0x141411440, "ui_141411440", fn_141411440 as *const ()), // sub_141411440
    Replacement::draft(0x1414115e0, "ui_1414115e0", fn_1414115e0 as *const ()), // sub_1414115e0
    Replacement::draft(0x141411780, "ui_141411780", fn_141411780 as *const ()), // sub_141411780
    Replacement::draft(0x141411910, "ui_141411910", fn_141411910 as *const ()), // sub_141411910
    Replacement::draft(0x141411940, "ui_141411940", tpl_26156::<0x142407c80, 0x143c4aa00, 0x1424f77a0, 0x1426455a8, 0x142645598, 0x1424f5620, 0x142645580> as *const ()), // sub_141411940
    Replacement::draft_throws(0x141411aa0, "ui_141411aa0", fn_141411aa0 as *const (), Throws::Discard), // sub_141411aa0
    Replacement::draft(0x141411c20, "ui_141411c20", tpl_26156::<0x142407c80, 0x143c4aa00, 0x1424f72b0, 0x1426455a8, 0x142645598, 0x1424f5580, 0x142645580> as *const ()), // sub_141411c20
    Replacement::draft_throws(0x141411d80, "ui_141411d80", fn_141411d80 as *const (), Throws::Discard), // sub_141411d80
    Replacement::draft_throws(0x141411ef0, "ui_141411ef0", fn_141411ef0 as *const (), Throws::Discard), // sub_141411ef0
    Replacement::draft(0x141412050, "ui_141412050", fn_141412050 as *const ()), // sub_141412050
    Replacement::draft(0x1414121c0, "ui_1414121c0", tpl_26156::<0x142407c80, 0x143c4aa00, 0x1424f6730, 0x1426455a8, 0x142645598, 0x1424f5470, 0x142645580> as *const ()), // sub_1414121c0
    Replacement::draft_throws(0x141412470, "ui_141412470", fn_141412470 as *const (), Throws::Discard), // sub_141412470
    Replacement::draft(0x1414126a0, "ui_1414126a0", tpl_5534::<0x19, 0x10, 0x1414126a0, 0x128, 0x1401f2bc0, 0x20, 0x14240ab10, 0x1401e66f0> as *const ()), // sub_1414126a0
    Replacement::draft(0x141412730, "ui_141412730", fn_141412730 as *const ()), // sub_141412730
    Replacement::draft(0x141412a80, "ui_141412a80", tpl_26163::<0x140, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_141412a80
    Replacement::draft(0x141412b00, "ui_141412b00", fn_141412b00 as *const ()), // sub_141412b00
    Replacement::draft(0x141412c90, "ui_141412c90", fn_141412c90 as *const ()), // sub_141412c90
    Replacement::draft(0x141413000, "ui_141413000", fn_141413000 as *const ()), // sub_141413000
    Replacement::draft(0x141413170, "ui_141413170", fn_141413170 as *const ()), // sub_141413170
    Replacement::draft(0x141413360, "ui_141413360", fn_141413360 as *const ()), // sub_141413360
    Replacement::draft(0x141413490, "ui_141413490", fn_141413490 as *const ()), // ui::SyncStreamReader::ui::SyncStreamReader::vfunc_0
    Replacement::draft(0x141413500, "ui_141413500", fn_141413500 as *const ()), // sub_141413500
    Replacement::draft(0x141413620, "ui_141413620", fn_141413620 as *const ()), // sub_141413620
    Replacement::draft(0x1414137e0, "ui_1414137e0", fn_1414137e0 as *const ()), // sub_1414137e0
    Replacement::draft(0x141413990, "ui_141413990", fn_141413990 as *const ()), // ui::PageResource::ui::PageResource::vfunc_0
    Replacement::draft(0x141413a60, "ui_141413a60", tpl_2030::<0x141413dd0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141413a60
    Replacement::draft(0x141413b10, "ui_141413b10", tpl_2030::<0x141413e40, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141413b10
    Replacement::draft(0x141413bc0, "ui_141413bc0", tpl_2030::<0x140f5c7f0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141413bc0
    Replacement::draft(0x141413c70, "ui_141413c70", tpl_2030::<0x141413ec0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141413c70
    Replacement::draft(0x141413d20, "ui_141413d20", tpl_2030::<0x141413f40, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141413d20
    Replacement::draft(0x141413dd0, "ui_141413dd0", tpl_19159::<0x8, 0x28, 0x1401f2bc0, 0x14240ab10> as *const ()), // sub_141413dd0
    Replacement::draft(0x141413e40, "ui_141413e40", fn_141413e40 as *const ()), // sub_141413e40
    Replacement::draft(0x141413ec0, "ui_141413ec0", fn_141413ec0 as *const ()), // sub_141413ec0
    Replacement::draft(0x141413f40, "ui_141413f40", fn_141413f40 as *const ()), // sub_141413f40
    Replacement::draft(0x141413fb0, "ui_141413fb0", fn_141413fb0 as *const ()), // sub_141413fb0
    Replacement::draft(0x141414030, "ui_141414030", fn_141414030 as *const ()), // sub_141414030
    Replacement::draft(0x1414140d0, "ui_1414140d0", fn_1414140d0 as *const ()), // sub_1414140d0
    Replacement::draft(0x1414144d0, "ui_1414144d0", fn_1414144d0 as *const ()), // sub_1414144d0
    Replacement::draft_throws(0x141414a40, "ui_141414a40", fn_141414a40 as *const (), Throws::Discard), // sub_141414a40
    Replacement::draft(0x141414c00, "ui_141414c00", fn_141414c00 as *const ()), // sub_141414c00
    Replacement::draft_throws(0x141414fb0, "ui_141414fb0", fn_141414fb0 as *const (), Throws::Discard), // sub_141414fb0
    Replacement::draft(0x141415190, "ui_141415190", fn_141415190 as *const ()), // sub_141415190
    Replacement::draft_throws(0x141415810, "ui_141415810", fn_141415810 as *const (), Throws::Caught), // sub_141415810
    Replacement::draft_throws(0x1414159e0, "ui_1414159e0", fn_1414159e0 as *const (), Throws::Caught), // sub_1414159e0
    Replacement::draft_throws(0x141415bb0, "ui_141415bb0", fn_141415bb0 as *const (), Throws::Caught), // sub_141415bb0
    Replacement::draft(0x141415d80, "ui_141415d80", fn_141415d80 as *const ()), // sub_141415d80
    Replacement::draft(0x1414161a0, "ui_1414161a0", fn_1414161a0 as *const ()), // sub_1414161a0
    Replacement::draft(0x141416560, "ui_141416560", fn_141416560 as *const ()), // sub_141416560
    Replacement::draft_throws(0x141416940, "ui_141416940", fn_141416940 as *const (), Throws::Discard), // sub_141416940
    Replacement::draft_throws(0x141416d10, "ui_141416d10", fn_141416d10 as *const (), Throws::Discard), // sub_141416d10
    Replacement::draft(0x141416f10, "ui_141416f10", fn_141416f10 as *const ()), // sub_141416f10
    Replacement::draft(0x141416fa0, "ui_141416fa0", fn_141416fa0 as *const ()), // sub_141416fa0
    Replacement::draft(0x141417050, "ui_141417050", fn_141417050 as *const ()), // sub_141417050
    Replacement::draft(0x141417130, "ui_141417130", fn_141417130 as *const ()), // sub_141417130
    Replacement::draft(0x1414171d0, "ui_1414171d0", fn_1414171d0 as *const ()), // sub_1414171d0
    Replacement::draft_throws(0x141417300, "ui_141417300", fn_141417300 as *const (), Throws::Discard), // sub_141417300
    Replacement::draft(0x141417420, "ui_141417420", fn_141417420 as *const ()), // sub_141417420
    Replacement::draft_throws(0x1414174d0, "ui_1414174d0", fn_1414174d0 as *const (), Throws::Discard), // sub_1414174d0
    Replacement::draft(0x141417610, "ui_141417610", fn_141417610 as *const ()), // sub_141417610
    Replacement::draft_throws(0x141417690, "ui_141417690", fn_141417690 as *const (), Throws::Caught), // sub_141417690
    Replacement::draft_throws(0x1414177d0, "ui_1414177d0", fn_1414177d0 as *const (), Throws::Caught), // sub_1414177d0
    Replacement::draft_throws(0x141417910, "ui_141417910", fn_141417910 as *const (), Throws::Caught), // sub_141417910
    Replacement::draft(0x141417a50, "ui_141417a50", fn_141417a50 as *const ()), // sub_141417a50
    Replacement::draft(0x141417b30, "ui_141417b30", fn_141417b30 as *const ()), // sub_141417b30
    Replacement::draft(0x141417c20, "ui_141417c20", fn_141417c20 as *const ()), // sub_141417c20
    Replacement::draft_throws(0x141417cd0, "ui_141417cd0", fn_141417cd0 as *const (), Throws::Discard), // sub_141417cd0
    Replacement::draft_throws(0x141417d80, "ui_141417d80", fn_141417d80 as *const (), Throws::Discard), // sub_141417d80
    Replacement::draft(0x141417ec0, "ui_141417ec0", fn_141417ec0 as *const ()), // sub_141417ec0
    Replacement::draft(0x141417fa0, "ui_141417fa0", fn_141417fa0 as *const ()), // sub_141417fa0
    Replacement::draft(0x1414180b0, "ui_1414180b0", fn_1414180b0 as *const ()), // sub_1414180b0
    Replacement::draft(0x1414182c0, "ui_1414182c0", fn_1414182c0 as *const ()), // sub_1414182c0
    Replacement::draft_throws(0x141418730, "ui_141418730", fn_141418730 as *const (), Throws::Discard), // sub_141418730
    Replacement::draft_throws(0x141418ab0, "ui_141418ab0", fn_141418ab0 as *const (), Throws::Discard), // sub_141418ab0
    Replacement::draft_throws(0x141418dd0, "ui_141418dd0", fn_141418dd0 as *const (), Throws::Discard), // sub_141418dd0
    Replacement::draft(0x1414191e0, "ui_1414191e0", fn_1414191e0 as *const ()), // sub_1414191e0
    Replacement::draft_throws(0x1414193f0, "ui_1414193f0", fn_1414193f0 as *const (), Throws::Caught), // sub_1414193f0
    Replacement::draft_throws(0x141419820, "ui_141419820", fn_141419820 as *const (), Throws::Caught), // sub_141419820
    Replacement::draft_throws(0x141419c50, "ui_141419c50", fn_141419c50 as *const (), Throws::Caught), // sub_141419c50
    Replacement::draft(0x14141a080, "ui_14141a080", fn_14141a080 as *const ()), // sub_14141a080
    Replacement::draft(0x14141a2a0, "ui_14141a2a0", fn_14141a2a0 as *const ()), // sub_14141a2a0
    Replacement::draft(0x14141a5b0, "ui_14141a5b0", fn_14141a5b0 as *const ()), // sub_14141a5b0
    Replacement::draft_throws(0x14141a8c0, "ui_14141a8c0", fn_14141a8c0 as *const (), Throws::Discard), // sub_14141a8c0
    Replacement::draft_throws(0x14141ac60, "ui_14141ac60", fn_14141ac60 as *const (), Throws::Discard), // sub_14141ac60
    Replacement::draft(0x14141b040, "ui_14141b040", fn_14141b040 as *const ()), // sub_14141b040
    Replacement::draft(0x14141b0c0, "ui_14141b0c0", fn_14141b0c0 as *const ()), // sub_14141b0c0
    Replacement::draft(0x14141b180, "ui_14141b180", fn_14141b180 as *const ()), // sub_14141b180
    Replacement::draft(0x14141b240, "ui_14141b240", fn_14141b240 as *const ()), // sub_14141b240
    Replacement::draft(0x14141b340, "ui_14141b340", fn_14141b340 as *const ()), // sub_14141b340
    Replacement::draft(0x14141b3e0, "ui_14141b3e0", fn_14141b3e0 as *const ()), // sub_14141b3e0
    Replacement::draft_throws(0x14141b6f0, "ui_14141b6f0", fn_14141b6f0 as *const (), Throws::Discard), // sub_14141b6f0
    Replacement::draft(0x14141b8a0, "ui_14141b8a0", fn_14141b8a0 as *const ()), // sub_14141b8a0
    Replacement::draft_throws(0x14141bae0, "ui_14141bae0", fn_14141bae0 as *const (), Throws::Discard), // sub_14141bae0
    Replacement::draft(0x14141bcb0, "ui_14141bcb0", fn_14141bcb0 as *const ()), // sub_14141bcb0
    Replacement::draft_throws(0x14141be10, "ui_14141be10", fn_14141be10 as *const (), Throws::Caught), // sub_14141be10
    Replacement::draft_throws(0x14141c240, "ui_14141c240", fn_14141c240 as *const (), Throws::Caught), // sub_14141c240
    Replacement::draft_throws(0x14141c630, "ui_14141c630", fn_14141c630 as *const (), Throws::Caught), // sub_14141c630
    Replacement::draft(0x14141ca00, "ui_14141ca00", fn_14141ca00 as *const ()), // sub_14141ca00
    Replacement::draft(0x14141cbe0, "ui_14141cbe0", fn_14141cbe0 as *const ()), // sub_14141cbe0
    Replacement::draft(0x14141ce00, "ui_14141ce00", fn_14141ce00 as *const ()), // sub_14141ce00
    Replacement::draft_throws(0x14141d040, "ui_14141d040", fn_14141d040 as *const (), Throws::Discard), // sub_14141d040
    Replacement::draft_throws(0x14141d220, "ui_14141d220", fn_14141d220 as *const (), Throws::Discard), // sub_14141d220
    Replacement::draft(0x14141d3c0, "ui_14141d3c0", fn_14141d3c0 as *const ()), // sub_14141d3c0
    Replacement::draft(0x14141d6d0, "ui_14141d6d0", fn_14141d6d0 as *const ()), // sub_14141d6d0
    Replacement::draft(0x14141d850, "ui_14141d850", fn_14141d850 as *const ()), // sub_14141d850
    Replacement::draft(0x14141d8f0, "ui_14141d8f0", fn_14141d8f0 as *const ()), // sub_14141d8f0
    Replacement::draft(0x14141d9b0, "ui_14141d9b0", tpl_18422::<0x143882260, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x28, 0x1406dd2c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_14141d9b0
    Replacement::draft_throws(0x14141da60, "ui_14141da60", fn_14141da60 as *const (), Throws::Discard), // sub_14141da60
    Replacement::draft(0x14141db60, "ui_14141db60", fn_14141db60 as *const ()), // sub_14141db60
    Replacement::draft_throws(0x14141dbe0, "ui_14141dbe0", fn_14141dbe0 as *const (), Throws::Discard), // sub_14141dbe0
    Replacement::draft_throws(0x14141dce0, "ui_14141dce0", fn_14141dce0 as *const (), Throws::Caught), // sub_14141dce0
    Replacement::draft_throws(0x14141df50, "ui_14141df50", fn_14141df50 as *const (), Throws::Caught), // sub_14141df50
    Replacement::draft_throws(0x14141e190, "ui_14141e190", fn_14141e190 as *const (), Throws::Caught), // sub_14141e190
    Replacement::draft(0x14141e3c0, "ui_14141e3c0", fn_14141e3c0 as *const ()), // sub_14141e3c0
    Replacement::draft(0x14141e480, "ui_14141e480", fn_14141e480 as *const ()), // sub_14141e480
    Replacement::draft(0x14141e550, "ui_14141e550", fn_14141e550 as *const ()), // sub_14141e550
    Replacement::draft_throws(0x14141e630, "ui_14141e630", fn_14141e630 as *const (), Throws::Discard), // sub_14141e630
    Replacement::draft_throws(0x14141e6a0, "ui_14141e6a0", fn_14141e6a0 as *const (), Throws::Caught), // sub_14141e6a0
    Replacement::draft(0x14141e800, "ui_14141e800", fn_14141e800 as *const ()), // sub_14141e800
    Replacement::draft(0x14141e930, "ui_14141e930", fn_14141e930 as *const ()), // sub_14141e930
    Replacement::draft(0x14141ea80, "ui_14141ea80", tpl_26237::<0x143882278, 0x14243b0b0, 0x1401ee7c0, 0x1426d4580> as *const ()), // sub_14141ea80
    Replacement::draft_throws(0x14141ec90, "ui_14141ec90", tpl_84434::<0x1438821e0, 0x143882278, 0x1401e9fa0, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_14141ec90
    Replacement::draft(0x14141eeb0, "ui_14141eeb0", tpl_26238::<0x143882278, 0x14243b0b0> as *const ()), // sub_14141eeb0
    Replacement::draft_throws(0x14141efc0, "ui_14141efc0", tpl_84435::<0x1438821e0, 0x143882278, 0x1401e9b60, 0x1426cdcb0, 0x1424b4da0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_14141efc0
    Replacement::draft(0x14141f1b0, "ui_14141f1b0", fn_14141f1b0 as *const ()), // sub_14141f1b0
    Replacement::draft_throws(0x14141f240, "ui_14141f240", fn_14141f240 as *const (), Throws::Caught), // sub_14141f240
    Replacement::draft_throws(0x14141f900, "ui_14141f900", fn_14141f900 as *const (), Throws::Caught), // sub_14141f900
    Replacement::draft_throws(0x14141fed0, "ui_14141fed0", fn_14141fed0 as *const (), Throws::Caught), // sub_14141fed0
    Replacement::draft(0x141420480, "ui_141420480", tpl_26243::<0x143882278, 0x14243b0b0, 0x1426cdcb0, 0x142409b50> as *const ()), // sub_141420480
    Replacement::draft(0x1414205d0, "ui_1414205d0", tpl_26244::<0x1438821e0, 0x143882278, 0x1401e9970, 0xc0, 0x28, 0x50, 0x78, 0xe8, 0x7c, 0xd8, 0x80, 0x84, 0x1401e9860> as *const ()), // sub_1414205d0
    Replacement::draft(0x141420700, "ui_141420700", tpl_26245::<0x143882278, 0x14243b0b0> as *const ()), // sub_141420700
    Replacement::draft_throws(0x141420840, "ui_141420840", tpl_26246::<0x1438821e0, 0x143882278, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x50, 0x78, 0x120, 0x8c, 0x7c, 0x1401e9590> as *const (), Throws::Discard), // sub_141420840
    Replacement::draft_throws(0x141420910, "ui_141420910", tpl_26247::<0x143882278, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143419964, 0x1402c4730, 0x143419958, 0x143419948, 0x1430b0324, 0x143419930, 0x1413c4540, 0x1434199a0> as *const (), Throws::Discard), // sub_141420910
    Replacement::draft(0x141420ac0, "ui_141420ac0", fn_141420ac0 as *const ()), // sub_141420ac0
    Replacement::draft(0x141420ba0, "ui_141420ba0", fn_141420ba0 as *const ()), // sub_141420ba0
    Replacement::draft(0x141420cc0, "ui_141420cc0", fn_141420cc0 as *const ()), // sub_141420cc0
    Replacement::draft_throws(0x141420e70, "ui_141420e70", fn_141420e70 as *const (), Throws::Discard), // sub_141420e70
    Replacement::draft(0x141421030, "ui_141421030", fn_141421030 as *const ()), // sub_141421030
    Replacement::draft_throws(0x141421130, "ui_141421130", fn_141421130 as *const (), Throws::Discard), // sub_141421130
    Replacement::draft(0x1414212d0, "ui_1414212d0", fn_1414212d0 as *const ()), // sub_1414212d0
    Replacement::draft_throws(0x141421350, "ui_141421350", fn_141421350 as *const (), Throws::Caught), // sub_141421350
    Replacement::draft_throws(0x141421990, "ui_141421990", fn_141421990 as *const (), Throws::Caught), // sub_141421990
    Replacement::draft_throws(0x141421ef0, "ui_141421ef0", fn_141421ef0 as *const (), Throws::Caught), // sub_141421ef0
    Replacement::draft(0x141422420, "ui_141422420", fn_141422420 as *const ()), // sub_141422420
    Replacement::draft(0x141422540, "ui_141422540", fn_141422540 as *const ()), // sub_141422540
    Replacement::draft(0x141422650, "ui_141422650", fn_141422650 as *const ()), // sub_141422650
    Replacement::draft_throws(0x141422770, "ui_141422770", fn_141422770 as *const (), Throws::Discard), // sub_141422770
    Replacement::draft_throws(0x141422830, "ui_141422830", fn_141422830 as *const (), Throws::Discard), // sub_141422830
    Replacement::draft(0x1414229b0, "ui_1414229b0", tpl_26237::<0x1438821f8, 0x14243b0b0, 0x1401ee7c0, 0x1426d4580> as *const ()), // sub_1414229b0
    Replacement::draft_throws(0x141422bc0, "ui_141422bc0", tpl_84434::<0x143882234, 0x1438821f8, 0x1401e9fa0, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_141422bc0
    Replacement::draft(0x141422de0, "ui_141422de0", tpl_26238::<0x1438821f8, 0x14243b0b0> as *const ()), // sub_141422de0
    Replacement::draft_throws(0x141422ef0, "ui_141422ef0", tpl_84435::<0x143882234, 0x1438821f8, 0x1401e9b60, 0x1426cdcb0, 0x1424b4da0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_141422ef0
    Replacement::draft_throws(0x1414230e0, "ui_1414230e0", fn_1414230e0 as *const (), Throws::Caught), // sub_1414230e0
    Replacement::draft_throws(0x1414237a0, "ui_1414237a0", fn_1414237a0 as *const (), Throws::Caught), // sub_1414237a0
    Replacement::draft_throws(0x141423d70, "ui_141423d70", fn_141423d70 as *const (), Throws::Caught), // sub_141423d70
    Replacement::draft(0x141424320, "ui_141424320", tpl_26243::<0x1438821f8, 0x14243b0b0, 0x1426cdcb0, 0x142409b50> as *const ()), // sub_141424320
    Replacement::draft(0x141424470, "ui_141424470", tpl_26244::<0x143882234, 0x1438821f8, 0x1401e9970, 0xc0, 0x28, 0x50, 0x78, 0xe8, 0x7c, 0xd8, 0x80, 0x84, 0x1401e9860> as *const ()), // sub_141424470
    Replacement::draft(0x1414245a0, "ui_1414245a0", tpl_26245::<0x1438821f8, 0x14243b0b0> as *const ()), // sub_1414245a0
    Replacement::draft_throws(0x1414246e0, "ui_1414246e0", tpl_26246::<0x143882234, 0x1438821f8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x50, 0x78, 0x120, 0x8c, 0x7c, 0x1401e9590> as *const (), Throws::Discard), // sub_1414246e0
    Replacement::draft_throws(0x1414247b0, "ui_1414247b0", tpl_26247::<0x1438821f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143419958, 0x1402c4730, 0x143419990, 0x143419980, 0x1430b0324, 0x143419930, 0x1413c4540, 0x1434199a0> as *const (), Throws::Discard), // sub_1414247b0
    Replacement::draft(0x141424960, "ui_141424960", fn_141424960 as *const ()), // sub_141424960
    Replacement::draft(0x141424a10, "ui_141424a10", fn_141424a10 as *const ()), // sub_141424a10
    Replacement::draft(0x141424ae0, "ui_141424ae0", fn_141424ae0 as *const ()), // sub_141424ae0
    Replacement::draft_throws(0x141424c00, "ui_141424c00", fn_141424c00 as *const (), Throws::Discard), // sub_141424c00
    Replacement::draft(0x141424d40, "ui_141424d40", fn_141424d40 as *const ()), // sub_141424d40
    Replacement::draft_throws(0x141424e20, "ui_141424e20", fn_141424e20 as *const (), Throws::Discard), // sub_141424e20
    Replacement::draft_throws(0x141424f30, "ui_141424f30", fn_141424f30 as *const (), Throws::Caught), // sub_141424f30
    Replacement::draft_throws(0x141425280, "ui_141425280", fn_141425280 as *const (), Throws::Caught), // sub_141425280
    Replacement::draft_throws(0x141425570, "ui_141425570", fn_141425570 as *const (), Throws::Caught), // sub_141425570
    Replacement::draft(0x141425860, "ui_141425860", fn_141425860 as *const ()), // sub_141425860
    Replacement::draft(0x141425920, "ui_141425920", fn_141425920 as *const ()), // sub_141425920
    Replacement::draft(0x141425a00, "ui_141425a00", fn_141425a00 as *const ()), // sub_141425a00
    Replacement::draft_throws(0x141425af0, "ui_141425af0", fn_141425af0 as *const (), Throws::Discard), // sub_141425af0
    Replacement::draft_throws(0x141425b90, "ui_141425b90", fn_141425b90 as *const (), Throws::Discard), // sub_141425b90
    Replacement::draft(0x141425cc0, "ui_141425cc0", fn_141425cc0 as *const ()), // sub_141425cc0
    Replacement::draft(0x141425d60, "ui_141425d60", fn_141425d60 as *const ()), // sub_141425d60
    Replacement::draft(0x141425e10, "ui_141425e10", fn_141425e10 as *const ()), // sub_141425e10
    Replacement::draft(0x141425ed0, "ui_141425ed0", fn_141425ed0 as *const ()), // sub_141425ed0
    Replacement::draft_throws(0x141426010, "ui_141426010", fn_141426010 as *const (), Throws::Discard), // sub_141426010
    Replacement::draft(0x141426100, "ui_141426100", fn_141426100 as *const ()), // sub_141426100
    Replacement::draft_throws(0x1414261d0, "ui_1414261d0", fn_1414261d0 as *const (), Throws::Discard), // sub_1414261d0
    Replacement::draft(0x1414262e0, "ui_1414262e0", fn_1414262e0 as *const ()), // sub_1414262e0
    Replacement::draft_throws(0x141426380, "ui_141426380", fn_141426380 as *const (), Throws::Caught), // sub_141426380
    Replacement::draft_throws(0x141426600, "ui_141426600", fn_141426600 as *const (), Throws::Caught), // sub_141426600
    Replacement::draft_throws(0x141426840, "ui_141426840", fn_141426840 as *const (), Throws::Caught), // sub_141426840
    Replacement::draft(0x141426a70, "ui_141426a70", fn_141426a70 as *const ()), // sub_141426a70
    Replacement::draft(0x141426b50, "ui_141426b50", fn_141426b50 as *const ()), // sub_141426b50
    Replacement::draft(0x141426c10, "ui_141426c10", fn_141426c10 as *const ()), // sub_141426c10
    Replacement::draft_throws(0x141426ce0, "ui_141426ce0", fn_141426ce0 as *const (), Throws::Discard), // sub_141426ce0
    Replacement::draft_throws(0x141426d90, "ui_141426d90", tpl_7203::<0x143882280, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143419958, 0x1402c4730, 0x28, 0x143303898, 0x14142e060> as *const (), Throws::Discard), // sub_141426d90
    Replacement::draft(0x141426e60, "ui_141426e60", fn_141426e60 as *const ()), // sub_141426e60
    Replacement::draft_throws(0x141426f30, "ui_141426f30", fn_141426f30 as *const (), Throws::Discard), // sub_141426f30
    Replacement::draft(0x141427010, "ui_141427010", tpl_2258::<0x1438821f0, 0x14243b0b0, 0x338, 0x20, 0x28, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_141427010
    Replacement::draft_throws(0x141427090, "ui_141427090", fn_141427090 as *const (), Throws::Discard), // sub_141427090
    Replacement::draft_throws(0x141427170, "ui_141427170", fn_141427170 as *const (), Throws::Caught), // sub_141427170
    Replacement::draft_throws(0x1414273f0, "ui_1414273f0", fn_1414273f0 as *const (), Throws::Caught), // sub_1414273f0
    Replacement::draft_throws(0x141427640, "ui_141427640", fn_141427640 as *const (), Throws::Caught), // sub_141427640
    Replacement::draft(0x141427880, "ui_141427880", fn_141427880 as *const ()), // sub_141427880
    Replacement::draft(0x141427920, "ui_141427920", tpl_2262::<0x143882220, 0x1438821f0, 0x1401e9970, 0x338, 0xc0, 0x28, 0x120, 0x1401e9860> as *const ()), // sub_141427920
    Replacement::draft(0x1414279a0, "ui_1414279a0", tpl_2258::<0x1438821f0, 0x14243b0b0, 0x338, 0xc0, 0x28, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1414279a0
    Replacement::draft_throws(0x141427a30, "ui_141427a30", fn_141427a30 as *const (), Throws::Discard), // sub_141427a30
    Replacement::draft_throws(0x141427ab0, "ui_141427ab0", tpl_13627::<0x1438821f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1434199d8, 0x1402c4730, 0x28, 0x1434199c8, 0x14020f0f0> as *const (), Throws::Discard), // sub_141427ab0
    Replacement::draft(0x141427ba0, "ui_141427ba0", fn_141427ba0 as *const ()), // sub_141427ba0
    Replacement::draft(0x141427c60, "ui_141427c60", fn_141427c60 as *const ()), // sub_141427c60
    Replacement::draft(0x141427d30, "ui_141427d30", fn_141427d30 as *const ()), // sub_141427d30
    Replacement::draft_throws(0x141427ee0, "ui_141427ee0", fn_141427ee0 as *const (), Throws::Discard), // sub_141427ee0
    Replacement::draft(0x141428070, "ui_141428070", fn_141428070 as *const ()), // sub_141428070
    Replacement::draft_throws(0x141428140, "ui_141428140", fn_141428140 as *const (), Throws::Discard), // sub_141428140
    Replacement::draft(0x141428300, "ui_141428300", fn_141428300 as *const ()), // sub_141428300
    Replacement::draft_throws(0x141428390, "ui_141428390", fn_141428390 as *const (), Throws::Caught), // sub_141428390
    Replacement::draft_throws(0x141428950, "ui_141428950", fn_141428950 as *const (), Throws::Caught), // sub_141428950
    Replacement::draft_throws(0x141428e60, "ui_141428e60", fn_141428e60 as *const (), Throws::Caught), // sub_141428e60
    Replacement::draft(0x141429350, "ui_141429350", fn_141429350 as *const ()), // sub_141429350
    Replacement::draft(0x141429460, "ui_141429460", fn_141429460 as *const ()), // sub_141429460
    Replacement::draft(0x141429540, "ui_141429540", fn_141429540 as *const ()), // sub_141429540
    Replacement::draft_throws(0x141429620, "ui_141429620", fn_141429620 as *const (), Throws::Discard), // sub_141429620
    Replacement::draft_throws(0x141429700, "ui_141429700", fn_141429700 as *const (), Throws::Discard), // sub_141429700
    Replacement::draft(0x1414298a0, "ui_1414298a0", fn_1414298a0 as *const ()), // sub_1414298a0
    Replacement::draft_throws(0x1414299e0, "ui_1414299e0", fn_1414299e0 as *const (), Throws::Discard), // sub_1414299e0
    Replacement::draft(0x141429b10, "ui_141429b10", fn_141429b10 as *const ()), // sub_141429b10
    Replacement::draft_throws(0x141429bf0, "ui_141429bf0", fn_141429bf0 as *const (), Throws::Discard), // sub_141429bf0
    Replacement::draft(0x141429d40, "ui_141429d40", fn_141429d40 as *const ()), // sub_141429d40
    Replacement::draft_throws(0x141429dc0, "ui_141429dc0", fn_141429dc0 as *const (), Throws::Caught), // sub_141429dc0
    Replacement::draft_throws(0x14142a040, "ui_14142a040", fn_14142a040 as *const (), Throws::Caught), // sub_14142a040
    Replacement::draft_throws(0x14142a280, "ui_14142a280", fn_14142a280 as *const (), Throws::Caught), // sub_14142a280
    Replacement::draft(0x14142a4b0, "ui_14142a4b0", fn_14142a4b0 as *const ()), // sub_14142a4b0
    Replacement::draft(0x14142a590, "ui_14142a590", fn_14142a590 as *const ()), // sub_14142a590
    Replacement::draft(0x14142a660, "ui_14142a660", fn_14142a660 as *const ()), // sub_14142a660
    Replacement::draft_throws(0x14142a750, "ui_14142a750", fn_14142a750 as *const (), Throws::Discard), // sub_14142a750
    Replacement::draft_throws(0x14142a810, "ui_14142a810", tpl_7203::<0x1438821e8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1430af560, 0x1402c4730, 0x28, 0x1434199f8, 0x14142e0d0> as *const (), Throws::Discard), // sub_14142a810
    Replacement::draft(0x14142a8e0, "ui_14142a8e0", fn_14142a8e0 as *const ()), // ui::PageMetadata::ui::PageMetadata::vfunc_0
    Replacement::draft(0x14142a940, "ui_14142a940", fn_14142a940 as *const ()), // sub_14142a940
    Replacement::draft(0x14142aa20, "ui_14142aa20", tpl_2030::<0x14142ac70, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14142aa20
    Replacement::draft(0x14142aad0, "ui_14142aad0", fn_14142aad0 as *const ()), // sub_14142aad0
    Replacement::draft(0x14142ab90, "ui_14142ab90", fn_14142ab90 as *const ()), // sub_14142ab90
    Replacement::draft(0x14142ac70, "ui_14142ac70", fn_14142ac70 as *const ()), // sub_14142ac70
    Replacement::draft_throws(0x14142acf0, "ui_14142acf0", fn_14142acf0 as *const (), Throws::Caught), // sub_14142acf0
    Replacement::draft_throws(0x14142aea0, "ui_14142aea0", fn_14142aea0 as *const (), Throws::Caught), // sub_14142aea0
    Replacement::draft_throws(0x14142b050, "ui_14142b050", fn_14142b050 as *const (), Throws::Caught), // sub_14142b050
    Replacement::draft_throws(0x14142b200, "ui_14142b200", fn_14142b200 as *const (), Throws::Caught), // sub_14142b200
    Replacement::draft_throws(0x14142b3b0, "ui_14142b3b0", fn_14142b3b0 as *const (), Throws::Caught), // sub_14142b3b0
    Replacement::draft_throws(0x14142b560, "ui_14142b560", fn_14142b560 as *const (), Throws::Caught), // sub_14142b560
    Replacement::draft_throws(0x14142b710, "ui_14142b710", fn_14142b710 as *const (), Throws::Caught), // sub_14142b710
    Replacement::draft_throws(0x14142b8c0, "ui_14142b8c0", fn_14142b8c0 as *const (), Throws::Caught), // sub_14142b8c0
    Replacement::draft_throws(0x14142ba70, "ui_14142ba70", fn_14142ba70 as *const (), Throws::Caught), // sub_14142ba70
    Replacement::draft_throws(0x14142bc30, "ui_14142bc30", fn_14142bc30 as *const (), Throws::Caught), // sub_14142bc30
    Replacement::draft_throws(0x14142bdf0, "ui_14142bdf0", fn_14142bdf0 as *const (), Throws::Caught), // sub_14142bdf0
    Replacement::draft_throws(0x14142bfb0, "ui_14142bfb0", fn_14142bfb0 as *const (), Throws::Caught), // sub_14142bfb0
    Replacement::draft_throws(0x14142c170, "ui_14142c170", fn_14142c170 as *const (), Throws::Caught), // sub_14142c170
    Replacement::draft_throws(0x14142c330, "ui_14142c330", fn_14142c330 as *const (), Throws::Caught), // sub_14142c330
    Replacement::draft_throws(0x14142c4f0, "ui_14142c4f0", fn_14142c4f0 as *const (), Throws::Caught), // sub_14142c4f0
    Replacement::draft_throws(0x14142c6b0, "ui_14142c6b0", fn_14142c6b0 as *const (), Throws::Caught), // sub_14142c6b0
    Replacement::draft_throws(0x14142cc60, "ui_14142cc60", fn_14142cc60 as *const (), Throws::Caught), // sub_14142cc60
    Replacement::draft_throws(0x14142ce10, "ui_14142ce10", fn_14142ce10 as *const (), Throws::Caught), // sub_14142ce10
    Replacement::draft_throws(0x14142cfc0, "ui_14142cfc0", fn_14142cfc0 as *const (), Throws::Caught), // sub_14142cfc0
    Replacement::draft_throws(0x14142d180, "ui_14142d180", fn_14142d180 as *const (), Throws::Caught), // sub_14142d180
    Replacement::draft_throws(0x14142d420, "ui_14142d420", fn_14142d420 as *const (), Throws::Caught), // sub_14142d420
    Replacement::draft_throws(0x14142d5d0, "ui_14142d5d0", fn_14142d5d0 as *const (), Throws::Caught), // sub_14142d5d0
    Replacement::draft_throws(0x14142d780, "ui_14142d780", fn_14142d780 as *const (), Throws::Caught), // sub_14142d780
    Replacement::draft_throws(0x14142d930, "ui_14142d930", fn_14142d930 as *const (), Throws::Caught), // sub_14142d930
    Replacement::draft_throws(0x14142daf0, "ui_14142daf0", fn_14142daf0 as *const (), Throws::Caught), // sub_14142daf0
    Replacement::draft_throws(0x14142dcb0, "ui_14142dcb0", fn_14142dcb0 as *const (), Throws::Caught), // sub_14142dcb0
    Replacement::draft_throws(0x14142de80, "ui_14142de80", tpl_838::<0x141437390> as *const (), Throws::Discard), // sub_14142de80
    Replacement::draft(0x14142e110, "ui_14142e110", fn_14142e110 as *const ()), // sub_14142e110
    Replacement::draft(0x14142e260, "ui_14142e260", fn_14142e260 as *const ()), // sub_14142e260
    Replacement::draft(0x14142e360, "ui_14142e360", fn_14142e360 as *const ()), // sub_14142e360
    Replacement::draft(0x14142e450, "ui_14142e450", tpl_26343::<0x8, 0xc, 0x14142f420, 0xffffffffffffffd5, 0xffffffffffffffdd, 0xffffffffffffffe5, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffcc, 0xffffffffffffffcf, 0xffffffffffffffd4, 0x1401f2bc0, 0x14240ab10> as *const ()), // sub_14142e450
    Replacement::draft(0x14142e540, "ui_14142e540", fn_14142e540 as *const ()), // sub_14142e540
    Replacement::draft(0x14142e680, "ui_14142e680", fn_14142e680 as *const ()), // sub_14142e680
    Replacement::draft(0x14142e760, "ui_14142e760", tpl_26343::<0x8, 0xc, 0x14142fb00, 0xffffffffffffffd5, 0xffffffffffffffdd, 0xffffffffffffffe5, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffcc, 0xffffffffffffffcf, 0xffffffffffffffd4, 0x14115d8c0, 0x14240ab10> as *const ()), // sub_14142e760
    Replacement::draft(0x14142e850, "ui_14142e850", fn_14142e850 as *const ()), // sub_14142e850
    Replacement::draft(0x14142e9a0, "ui_14142e9a0", fn_14142e9a0 as *const ()), // sub_14142e9a0
    Replacement::draft(0x14142eac0, "ui_14142eac0", fn_14142eac0 as *const ()), // sub_14142eac0
    Replacement::draft(0x14142ebe0, "ui_14142ebe0", fn_14142ebe0 as *const ()), // sub_14142ebe0
    Replacement::draft(0x14142ece0, "ui_14142ece0", fn_14142ece0 as *const ()), // sub_14142ece0
    Replacement::draft(0x14142ede0, "ui_14142ede0", fn_14142ede0 as *const ()), // sub_14142ede0
    Replacement::draft(0x14142f030, "ui_14142f030", fn_14142f030 as *const ()), // sub_14142f030
    Replacement::draft(0x14142f240, "ui_14142f240", fn_14142f240 as *const ()), // sub_14142f240
    Replacement::draft(0x14142f420, "ui_14142f420", fn_14142f420 as *const ()), // sub_14142f420
    Replacement::draft(0x14142f640, "ui_14142f640", fn_14142f640 as *const ()), // sub_14142f640
    Replacement::draft(0x14142f8d0, "ui_14142f8d0", fn_14142f8d0 as *const ()), // sub_14142f8d0
    Replacement::draft(0x14142fb00, "ui_14142fb00", fn_14142fb00 as *const ()), // sub_14142fb00
    Replacement::draft(0x14142fd50, "ui_14142fd50", fn_14142fd50 as *const ()), // sub_14142fd50
    Replacement::draft(0x14142ffc0, "ui_14142ffc0", fn_14142ffc0 as *const ()), // sub_14142ffc0
    Replacement::draft_throws(0x141430220, "ui_141430220", fn_141430220 as *const (), Throws::Caught), // sub_141430220
    Replacement::draft_throws(0x141430520, "ui_141430520", fn_141430520 as *const (), Throws::Caught), // sub_141430520
    Replacement::draft_throws(0x141430820, "ui_141430820", fn_141430820 as *const (), Throws::Caught), // sub_141430820
    Replacement::draft_throws(0x141430b00, "ui_141430b00", fn_141430b00 as *const (), Throws::Caught), // sub_141430b00
    Replacement::draft_throws(0x141430de0, "ui_141430de0", fn_141430de0 as *const (), Throws::Caught), // sub_141430de0
    Replacement::draft_throws(0x1414310c0, "ui_1414310c0", fn_1414310c0 as *const (), Throws::Caught), // sub_1414310c0
    Replacement::draft_throws(0x1414313a0, "ui_1414313a0", fn_1414313a0 as *const (), Throws::Caught), // sub_1414313a0
    Replacement::draft_throws(0x141431680, "ui_141431680", fn_141431680 as *const (), Throws::Caught), // sub_141431680
    Replacement::draft_throws(0x141431960, "ui_141431960", fn_141431960 as *const (), Throws::Caught), // sub_141431960
    Replacement::draft_throws(0x141431de0, "ui_141431de0", fn_141431de0 as *const (), Throws::Caught), // sub_141431de0
    Replacement::draft_throws(0x1414320c0, "ui_1414320c0", fn_1414320c0 as *const (), Throws::Caught), // sub_1414320c0
    Replacement::draft_throws(0x1414323a0, "ui_1414323a0", fn_1414323a0 as *const (), Throws::Caught), // sub_1414323a0
    Replacement::draft_throws(0x141432650, "ui_141432650", fn_141432650 as *const (), Throws::Caught), // sub_141432650
    Replacement::draft_throws(0x141432900, "ui_141432900", fn_141432900 as *const (), Throws::Caught), // sub_141432900
    Replacement::draft_throws(0x141432bb0, "ui_141432bb0", fn_141432bb0 as *const (), Throws::Caught), // sub_141432bb0
    Replacement::draft_throws(0x141432e60, "ui_141432e60", fn_141432e60 as *const (), Throws::Caught), // sub_141432e60
    Replacement::draft_throws(0x141433110, "ui_141433110", fn_141433110 as *const (), Throws::Caught), // sub_141433110
    Replacement::draft_throws(0x1414333c0, "ui_1414333c0", fn_1414333c0 as *const (), Throws::Caught), // sub_1414333c0
    Replacement::draft_throws(0x141433810, "ui_141433810", fn_141433810 as *const (), Throws::Caught), // sub_141433810
    Replacement::draft_throws(0x141433ac0, "ui_141433ac0", fn_141433ac0 as *const (), Throws::Caught), // sub_141433ac0
    Replacement::draft_throws(0x141433d70, "ui_141433d70", fn_141433d70 as *const (), Throws::Caught), // sub_141433d70
    Replacement::draft_throws(0x141434010, "ui_141434010", fn_141434010 as *const (), Throws::Caught), // sub_141434010
    Replacement::draft_throws(0x1414342b0, "ui_1414342b0", fn_1414342b0 as *const (), Throws::Caught), // sub_1414342b0
    Replacement::draft_throws(0x141434550, "ui_141434550", fn_141434550 as *const (), Throws::Caught), // sub_141434550
    Replacement::draft_throws(0x1414347f0, "ui_1414347f0", fn_1414347f0 as *const (), Throws::Caught), // sub_1414347f0
    Replacement::draft_throws(0x141434a90, "ui_141434a90", fn_141434a90 as *const (), Throws::Caught), // sub_141434a90
    Replacement::draft_throws(0x141434d30, "ui_141434d30", fn_141434d30 as *const (), Throws::Caught), // sub_141434d30
    Replacement::draft_throws(0x141435170, "ui_141435170", fn_141435170 as *const (), Throws::Caught), // sub_141435170
    Replacement::draft_throws(0x141435410, "ui_141435410", fn_141435410 as *const (), Throws::Caught), // sub_141435410
    Replacement::draft_throws(0x1414356b0, "ui_1414356b0", fn_1414356b0 as *const (), Throws::Caught), // sub_1414356b0
    Replacement::draft_throws(0x141435780, "ui_141435780", fn_141435780 as *const (), Throws::Caught), // sub_141435780
    Replacement::draft_throws(0x141435850, "ui_141435850", fn_141435850 as *const (), Throws::Caught), // sub_141435850
    Replacement::draft_throws(0x141435920, "ui_141435920", fn_141435920 as *const (), Throws::Caught), // sub_141435920
    Replacement::draft_throws(0x1414359f0, "ui_1414359f0", fn_1414359f0 as *const (), Throws::Caught), // sub_1414359f0
    Replacement::draft_throws(0x141435ac0, "ui_141435ac0", fn_141435ac0 as *const (), Throws::Caught), // sub_141435ac0
    Replacement::draft_throws(0x141435b90, "ui_141435b90", fn_141435b90 as *const (), Throws::Caught), // sub_141435b90
    Replacement::draft_throws(0x141435c60, "ui_141435c60", fn_141435c60 as *const (), Throws::Caught), // sub_141435c60
    Replacement::draft_throws(0x141435d30, "ui_141435d30", fn_141435d30 as *const (), Throws::Caught), // sub_141435d30
    Replacement::draft(0x141435e00, "ui_141435e00", tpl_12631::<0x108, 0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // sub_141435e00
    Replacement::draft_throws(0x141435e60, "ui_141435e60", fn_141435e60 as *const (), Throws::Caught), // sub_141435e60
    Replacement::draft_throws(0x141436140, "ui_141436140", fn_141436140 as *const (), Throws::Caught), // sub_141436140
    Replacement::draft_throws(0x141436420, "ui_141436420", fn_141436420 as *const (), Throws::Caught), // sub_141436420
    Replacement::draft_throws(0x1414366d0, "ui_1414366d0", fn_1414366d0 as *const (), Throws::Caught), // sub_1414366d0
    Replacement::draft_throws(0x141436980, "ui_141436980", fn_141436980 as *const (), Throws::Caught), // sub_141436980
    Replacement::draft_throws(0x141436c20, "ui_141436c20", fn_141436c20 as *const (), Throws::Caught), // sub_141436c20
    Replacement::draft_throws(0x141436ec0, "ui_141436ec0", fn_141436ec0 as *const (), Throws::Caught), // sub_141436ec0
    Replacement::draft_throws(0x141436f90, "ui_141436f90", fn_141436f90 as *const (), Throws::Caught), // sub_141436f90
    Replacement::draft(0x141437080, "ui_141437080", fn_141437080 as *const ()), // sub_141437080
    Replacement::draft_throws(0x1414370c0, "ui_1414370c0", fn_1414370c0 as *const (), Throws::Caught), // sub_1414370c0
    Replacement::draft_throws(0x141437390, "ui_141437390", fn_141437390 as *const (), Throws::Caught), // sub_141437390
    Replacement::draft_throws(0x141437660, "ui_141437660", fn_141437660 as *const (), Throws::Caught), // sub_141437660
    Replacement::draft_throws(0x141437930, "ui_141437930", fn_141437930 as *const (), Throws::Caught), // sub_141437930
    Replacement::draft_throws(0x141437c00, "ui_141437c00", fn_141437c00 as *const (), Throws::Caught), // sub_141437c00
    Replacement::draft_throws(0x141437ea0, "ui_141437ea0", fn_141437ea0 as *const (), Throws::Caught), // sub_141437ea0
    Replacement::draft_throws(0x141438140, "ui_141438140", fn_141438140 as *const (), Throws::Caught), // sub_141438140
    Replacement::draft_throws(0x1414383e0, "ui_1414383e0", fn_1414383e0 as *const (), Throws::Caught), // sub_1414383e0
    Replacement::draft_throws(0x141438680, "ui_141438680", fn_141438680 as *const (), Throws::Caught), // sub_141438680
    Replacement::draft_throws(0x141438910, "ui_141438910", fn_141438910 as *const (), Throws::Caught), // sub_141438910
    Replacement::draft_throws(0x141438ba0, "ui_141438ba0", fn_141438ba0 as *const (), Throws::Caught), // sub_141438ba0
    Replacement::draft_throws(0x141438e30, "ui_141438e30", fn_141438e30 as *const (), Throws::Caught), // sub_141438e30
    Replacement::draft_throws(0x1414390c0, "ui_1414390c0", fn_1414390c0 as *const (), Throws::Caught), // sub_1414390c0
    Replacement::draft_throws(0x1414391f0, "ui_1414391f0", fn_1414391f0 as *const (), Throws::Caught), // sub_1414391f0
    Replacement::draft_throws(0x141439320, "ui_141439320", fn_141439320 as *const (), Throws::Caught), // sub_141439320
    Replacement::draft_throws(0x141439450, "ui_141439450", fn_141439450 as *const (), Throws::Caught), // sub_141439450
    Replacement::draft_throws(0x1414395b0, "ui_1414395b0", fn_1414395b0 as *const (), Throws::Caught), // sub_1414395b0
    Replacement::draft_throws(0x1414398b0, "ui_1414398b0", fn_1414398b0 as *const (), Throws::Caught), // sub_1414398b0
    Replacement::draft_throws(0x141439bb0, "ui_141439bb0", fn_141439bb0 as *const (), Throws::Caught), // sub_141439bb0
    Replacement::draft_throws(0x14143a040, "ui_14143a040", fn_14143a040 as *const (), Throws::Caught), // sub_14143a040
    Replacement::draft_throws(0x14143a490, "ui_14143a490", fn_14143a490 as *const (), Throws::Caught), // sub_14143a490
    Replacement::draft_throws(0x14143a8d0, "ui_14143a8d0", fn_14143a8d0 as *const (), Throws::Caught), // sub_14143a8d0
    Replacement::draft_throws(0x14143a9a0, "ui_14143a9a0", fn_14143a9a0 as *const (), Throws::Caught), // sub_14143a9a0
    Replacement::draft_throws(0x14143ac60, "ui_14143ac60", fn_14143ac60 as *const (), Throws::Caught), // sub_14143ac60
    Replacement::draft_throws(0x14143aef0, "ui_14143aef0", fn_14143aef0 as *const (), Throws::Caught), // sub_14143aef0
    Replacement::draft(0x14143b180, "ui_14143b180", fn_14143b180 as *const ()), // sub_14143b180
    Replacement::draft(0x14143b1e0, "ui_14143b1e0", fn_14143b1e0 as *const ()), // sub_14143b1e0
    Replacement::draft(0x14143b290, "ui_14143b290", tpl_24505::<0x8, 0x28, 0x1401f2bc0, 0x14240ab10> as *const ()), // sub_14143b290
    Replacement::draft(0x14143b2d0, "ui_14143b2d0", tpl_24505::<0x8, 0x28, 0x14115d8c0, 0x14240ab10> as *const ()), // sub_14143b2d0
    Replacement::draft(0x14143b310, "ui_14143b310", fn_14143b310 as *const ()), // sub_14143b310
    Replacement::draft(0x14143b350, "ui_14143b350", fn_14143b350 as *const ()), // sub_14143b350
    Replacement::draft(0x14143b3b0, "ui_14143b3b0", tpl_24250::<0x8, 0x14240ab10> as *const ()), // sub_14143b3b0
    Replacement::draft_throws(0x14143b3f0, "ui_14143b3f0", fn_14143b3f0 as *const (), Throws::Caught), // sub_14143b3f0
    Replacement::draft_throws(0x14143b8c0, "ui_14143b8c0", fn_14143b8c0 as *const (), Throws::Caught), // sub_14143b8c0
    Replacement::draft_throws(0x14143bd90, "ui_14143bd90", fn_14143bd90 as *const (), Throws::Caught), // sub_14143bd90
    Replacement::draft_throws(0x14143c260, "ui_14143c260", fn_14143c260 as *const (), Throws::Caught), // sub_14143c260
    Replacement::draft_throws(0x14143c730, "ui_14143c730", fn_14143c730 as *const (), Throws::Caught), // sub_14143c730
    Replacement::draft_throws(0x14143cc00, "ui_14143cc00", fn_14143cc00 as *const (), Throws::Caught), // sub_14143cc00
    Replacement::draft_throws(0x14143d0d0, "ui_14143d0d0", fn_14143d0d0 as *const (), Throws::Caught), // sub_14143d0d0
    Replacement::draft_throws(0x14143d5a0, "ui_14143d5a0", fn_14143d5a0 as *const (), Throws::Caught), // sub_14143d5a0
    Replacement::draft_throws(0x14143da70, "ui_14143da70", fn_14143da70 as *const (), Throws::Caught), // sub_14143da70
    Replacement::draft_throws(0x14143df40, "ui_14143df40", fn_14143df40 as *const (), Throws::Caught), // sub_14143df40
    Replacement::draft_throws(0x14143e3d0, "ui_14143e3d0", fn_14143e3d0 as *const (), Throws::Caught), // sub_14143e3d0
    Replacement::draft_throws(0x14143e860, "ui_14143e860", fn_14143e860 as *const (), Throws::Caught), // sub_14143e860
    Replacement::draft_throws(0x14143ecf0, "ui_14143ecf0", fn_14143ecf0 as *const (), Throws::Caught), // sub_14143ecf0
    Replacement::draft_throws(0x14143f180, "ui_14143f180", fn_14143f180 as *const (), Throws::Caught), // sub_14143f180
    Replacement::draft_throws(0x14143f610, "ui_14143f610", fn_14143f610 as *const (), Throws::Caught), // sub_14143f610
    Replacement::draft_throws(0x14143faa0, "ui_14143faa0", fn_14143faa0 as *const (), Throws::Caught), // sub_14143faa0
    Replacement::draft_throws(0x14143ff30, "ui_14143ff30", fn_14143ff30 as *const (), Throws::Caught), // sub_14143ff30
    Replacement::draft_throws(0x1414403c0, "ui_1414403c0", fn_1414403c0 as *const (), Throws::Caught), // sub_1414403c0
    Replacement::draft_throws(0x141440850, "ui_141440850", fn_141440850 as *const (), Throws::Caught), // sub_141440850
    Replacement::draft_throws(0x141440ce0, "ui_141440ce0", fn_141440ce0 as *const (), Throws::Caught), // sub_141440ce0
    Replacement::draft_throws(0x141441170, "ui_141441170", fn_141441170 as *const (), Throws::Caught), // sub_141441170
    Replacement::draft_throws(0x141441600, "ui_141441600", fn_141441600 as *const (), Throws::Caught), // sub_141441600
    Replacement::draft_throws(0x141441a90, "ui_141441a90", fn_141441a90 as *const (), Throws::Caught), // sub_141441a90
    Replacement::draft_throws(0x141441f20, "ui_141441f20", fn_141441f20 as *const (), Throws::Caught), // sub_141441f20
    Replacement::draft_throws(0x1414423b0, "ui_1414423b0", fn_1414423b0 as *const (), Throws::Caught), // sub_1414423b0
    Replacement::draft_throws(0x141442840, "ui_141442840", fn_141442840 as *const (), Throws::Caught), // sub_141442840
    Replacement::draft_throws(0x141442cd0, "ui_141442cd0", fn_141442cd0 as *const (), Throws::Caught), // sub_141442cd0
    Replacement::draft_throws(0x141443160, "ui_141443160", fn_141443160 as *const (), Throws::Caught), // sub_141443160
    Replacement::draft_throws(0x141443630, "ui_141443630", fn_141443630 as *const (), Throws::Caught), // sub_141443630
    Replacement::draft_throws(0x141443b00, "ui_141443b00", fn_141443b00 as *const (), Throws::Caught), // sub_141443b00
    Replacement::draft_throws(0x141443f90, "ui_141443f90", fn_141443f90 as *const (), Throws::Caught), // sub_141443f90
    Replacement::draft_throws(0x141444420, "ui_141444420", fn_141444420 as *const (), Throws::Caught), // sub_141444420
    Replacement::draft_throws(0x1414448b0, "ui_1414448b0", fn_1414448b0 as *const (), Throws::Caught), // sub_1414448b0
    Replacement::draft_throws(0x141444d40, "ui_141444d40", fn_141444d40 as *const (), Throws::Caught), // sub_141444d40
    Replacement::draft_throws(0x141445250, "ui_141445250", fn_141445250 as *const (), Throws::Caught), // sub_141445250
    Replacement::draft_throws(0x141445760, "ui_141445760", fn_141445760 as *const (), Throws::Caught), // sub_141445760
    Replacement::draft_throws(0x141445c70, "ui_141445c70", fn_141445c70 as *const (), Throws::Caught), // sub_141445c70
    Replacement::draft_throws(0x141446180, "ui_141446180", fn_141446180 as *const (), Throws::Caught), // sub_141446180
    Replacement::draft_throws(0x141446640, "ui_141446640", fn_141446640 as *const (), Throws::Caught), // sub_141446640
    Replacement::draft_throws(0x141446b00, "ui_141446b00", fn_141446b00 as *const (), Throws::Caught), // sub_141446b00
    Replacement::draft_throws(0x141446fc0, "ui_141446fc0", fn_141446fc0 as *const (), Throws::Caught), // sub_141446fc0
    Replacement::draft_throws(0x141447480, "ui_141447480", fn_141447480 as *const (), Throws::Caught), // sub_141447480
    Replacement::draft_throws(0x141447940, "ui_141447940", fn_141447940 as *const (), Throws::Caught), // sub_141447940
    Replacement::draft_throws(0x141447e00, "ui_141447e00", fn_141447e00 as *const (), Throws::Caught), // sub_141447e00
    Replacement::draft_throws(0x1414482c0, "ui_1414482c0", fn_1414482c0 as *const (), Throws::Caught), // sub_1414482c0
    Replacement::draft_throws(0x141448780, "ui_141448780", fn_141448780 as *const (), Throws::Caught), // sub_141448780
    Replacement::draft_throws(0x141448c50, "ui_141448c50", fn_141448c50 as *const (), Throws::Caught), // sub_141448c50
    Replacement::draft_throws(0x1414490e0, "ui_1414490e0", fn_1414490e0 as *const (), Throws::Caught), // sub_1414490e0
    Replacement::draft_throws(0x141449570, "ui_141449570", fn_141449570 as *const (), Throws::Caught), // sub_141449570
    Replacement::draft_throws(0x141449ae0, "ui_141449ae0", fn_141449ae0 as *const (), Throws::Caught), // sub_141449ae0
    Replacement::draft_throws(0x141449fc0, "ui_141449fc0", fn_141449fc0 as *const (), Throws::Caught), // sub_141449fc0
    Replacement::draft(0x14144a490, "ui_14144a490", fn_14144a490 as *const ()), // sub_14144a490
    Replacement::draft(0x14144a540, "ui_14144a540", fn_14144a540 as *const ()), // sub_14144a540
    Replacement::draft(0x14144a660, "ui_14144a660", fn_14144a660 as *const ()), // sub_14144a660
    Replacement::draft(0x14144a730, "ui_14144a730", fn_14144a730 as *const ()), // sub_14144a730
    Replacement::draft(0x14144a810, "ui_14144a810", fn_14144a810 as *const ()), // sub_14144a810
    Replacement::draft(0x14144a920, "ui_14144a920", fn_14144a920 as *const ()), // sub_14144a920
    Replacement::draft_throws(0x14144aa20, "ui_14144aa20", fn_14144aa20 as *const (), Throws::Caught), // sub_14144aa20
    Replacement::draft_throws(0x14144ab60, "ui_14144ab60", fn_14144ab60 as *const (), Throws::Caught), // sub_14144ab60
    Replacement::draft_throws(0x14144aca0, "ui_14144aca0", fn_14144aca0 as *const (), Throws::Caught), // sub_14144aca0
    Replacement::draft_throws(0x14144ade0, "ui_14144ade0", fn_14144ade0 as *const (), Throws::Caught), // sub_14144ade0
    Replacement::draft_throws(0x14144af20, "ui_14144af20", fn_14144af20 as *const (), Throws::Caught), // sub_14144af20
    Replacement::draft_throws(0x14144b060, "ui_14144b060", fn_14144b060 as *const (), Throws::Caught), // sub_14144b060
    Replacement::draft_throws(0x14144b1a0, "ui_14144b1a0", fn_14144b1a0 as *const (), Throws::Caught), // sub_14144b1a0
    Replacement::draft_throws(0x14144b410, "ui_14144b410", fn_14144b410 as *const (), Throws::Caught), // sub_14144b410
    Replacement::draft_throws(0x14144b550, "ui_14144b550", fn_14144b550 as *const (), Throws::Caught), // sub_14144b550
    Replacement::draft_throws(0x14144b690, "ui_14144b690", fn_14144b690 as *const (), Throws::Caught), // sub_14144b690
    Replacement::draft_throws(0x14144b7d0, "ui_14144b7d0", fn_14144b7d0 as *const (), Throws::Caught), // sub_14144b7d0
    Replacement::draft_throws(0x14144b910, "ui_14144b910", fn_14144b910 as *const (), Throws::Caught), // sub_14144b910
    Replacement::draft_throws(0x14144ba50, "ui_14144ba50", fn_14144ba50 as *const (), Throws::Caught), // sub_14144ba50
    Replacement::draft_throws(0x14144bb90, "ui_14144bb90", fn_14144bb90 as *const (), Throws::Caught), // sub_14144bb90
    Replacement::draft_throws(0x14144bcd0, "ui_14144bcd0", fn_14144bcd0 as *const (), Throws::Caught), // sub_14144bcd0
    Replacement::draft_throws(0x14144be10, "ui_14144be10", fn_14144be10 as *const (), Throws::Caught), // sub_14144be10
    Replacement::draft_throws(0x14144c070, "ui_14144c070", fn_14144c070 as *const (), Throws::Caught), // sub_14144c070
    Replacement::draft_throws(0x14144c1b0, "ui_14144c1b0", fn_14144c1b0 as *const (), Throws::Caught), // sub_14144c1b0
    Replacement::draft_throws(0x14144c2f0, "ui_14144c2f0", fn_14144c2f0 as *const (), Throws::Caught), // sub_14144c2f0
    Replacement::draft_throws(0x14144c430, "ui_14144c430", fn_14144c430 as *const (), Throws::Caught), // sub_14144c430
    Replacement::draft_throws(0x14144c570, "ui_14144c570", fn_14144c570 as *const (), Throws::Caught), // sub_14144c570
    Replacement::draft_throws(0x14144c6b0, "ui_14144c6b0", fn_14144c6b0 as *const (), Throws::Caught), // sub_14144c6b0
    Replacement::draft_throws(0x14144c7f0, "ui_14144c7f0", fn_14144c7f0 as *const (), Throws::Caught), // sub_14144c7f0
    Replacement::draft_throws(0x14144c930, "ui_14144c930", fn_14144c930 as *const (), Throws::Caught), // sub_14144c930
    Replacement::draft_throws(0x14144ca70, "ui_14144ca70", fn_14144ca70 as *const (), Throws::Caught), // sub_14144ca70
    Replacement::draft_throws(0x14144ccd0, "ui_14144ccd0", fn_14144ccd0 as *const (), Throws::Caught), // sub_14144ccd0
    Replacement::draft_throws(0x14144ce10, "ui_14144ce10", fn_14144ce10 as *const (), Throws::Caught), // sub_14144ce10
    Replacement::draft_throws(0x14144cf50, "ui_14144cf50", fn_14144cf50 as *const (), Throws::Caught), // sub_14144cf50
    Replacement::draft_throws(0x14144d140, "ui_14144d140", fn_14144d140 as *const (), Throws::Caught), // sub_14144d140
    Replacement::draft_throws(0x14144d330, "ui_14144d330", fn_14144d330 as *const (), Throws::Caught), // sub_14144d330
    Replacement::draft_throws(0x14144d520, "ui_14144d520", fn_14144d520 as *const (), Throws::Caught), // sub_14144d520
    Replacement::draft_throws(0x14144d710, "ui_14144d710", fn_14144d710 as *const (), Throws::Caught), // sub_14144d710
    Replacement::draft_throws(0x14144d900, "ui_14144d900", fn_14144d900 as *const (), Throws::Caught), // sub_14144d900
    Replacement::draft_throws(0x14144db70, "ui_14144db70", fn_14144db70 as *const (), Throws::Caught), // sub_14144db70
    Replacement::draft_throws(0x14144de00, "ui_14144de00", fn_14144de00 as *const (), Throws::Caught), // sub_14144de00
    Replacement::draft_throws(0x14144dff0, "ui_14144dff0", fn_14144dff0 as *const (), Throws::Caught), // sub_14144dff0
    Replacement::draft_throws(0x14144e260, "ui_14144e260", fn_14144e260 as *const (), Throws::Caught), // sub_14144e260
    Replacement::draft_throws(0x14144e3a0, "ui_14144e3a0", fn_14144e3a0 as *const (), Throws::Caught), // sub_14144e3a0
    Replacement::draft_throws(0x14144e4e0, "ui_14144e4e0", fn_14144e4e0 as *const (), Throws::Caught), // sub_14144e4e0
    Replacement::draft_throws(0x14144e620, "ui_14144e620", fn_14144e620 as *const (), Throws::Caught), // sub_14144e620
    Replacement::draft_throws(0x14144e760, "ui_14144e760", fn_14144e760 as *const (), Throws::Caught), // sub_14144e760
    Replacement::draft_throws(0x14144e8a0, "ui_14144e8a0", fn_14144e8a0 as *const (), Throws::Caught), // sub_14144e8a0
    Replacement::draft_throws(0x14144e9e0, "ui_14144e9e0", fn_14144e9e0 as *const (), Throws::Caught), // sub_14144e9e0
    Replacement::draft_throws(0x14144ebd0, "ui_14144ebd0", fn_14144ebd0 as *const (), Throws::Caught), // sub_14144ebd0
    Replacement::draft_throws(0x14144edc0, "ui_14144edc0", fn_14144edc0 as *const (), Throws::Caught), // sub_14144edc0
    Replacement::draft_throws(0x14144efb0, "ui_14144efb0", fn_14144efb0 as *const (), Throws::Caught), // sub_14144efb0
    Replacement::draft_throws(0x14144f1a0, "ui_14144f1a0", fn_14144f1a0 as *const (), Throws::Caught), // sub_14144f1a0
    Replacement::draft_throws(0x14144f390, "ui_14144f390", fn_14144f390 as *const (), Throws::Caught), // sub_14144f390
    Replacement::draft_throws(0x14144f580, "ui_14144f580", fn_14144f580 as *const (), Throws::Caught), // sub_14144f580
    Replacement::draft_throws(0x14144f7f0, "ui_14144f7f0", fn_14144f7f0 as *const (), Throws::Caught), // sub_14144f7f0
    Replacement::draft_throws(0x14144fa60, "ui_14144fa60", fn_14144fa60 as *const (), Throws::Caught), // sub_14144fa60
    Replacement::draft_throws(0x14144fcd0, "ui_14144fcd0", fn_14144fcd0 as *const (), Throws::Caught), // sub_14144fcd0
    Replacement::draft(0x14144fec0, "ui_14144fec0", fn_14144fec0 as *const ()), // sub_14144fec0
    Replacement::draft(0x1414501b0, "ui_1414501b0", fn_1414501b0 as *const ()), // sub_1414501b0
    Replacement::draft(0x1414502b0, "ui_1414502b0", tpl_855::<0x14115d970, 0x1425371a0, 0x142533200, 0x14142f240, 0x14143b1e0> as *const ()), // sub_1414502b0
    Replacement::draft(0x141450410, "ui_141450410", tpl_855::<0x1414085b0, 0x1425371a0, 0x142533200, 0x14140bb30, 0x14140c620> as *const ()), // sub_141450410
    Replacement::draft_throws(0x141450570, "ui_141450570", fn_141450570 as *const (), Throws::Caught), // sub_141450570
    Replacement::draft_throws(0x1414509e0, "ui_1414509e0", fn_1414509e0 as *const (), Throws::Caught), // sub_1414509e0
    Replacement::draft_throws(0x141450e50, "ui_141450e50", fn_141450e50 as *const (), Throws::Caught), // sub_141450e50
    Replacement::draft_throws(0x1414512c0, "ui_1414512c0", fn_1414512c0 as *const (), Throws::Caught), // sub_1414512c0
    Replacement::draft_throws(0x141451730, "ui_141451730", fn_141451730 as *const (), Throws::Caught), // sub_141451730
    Replacement::draft_throws(0x141451ba0, "ui_141451ba0", fn_141451ba0 as *const (), Throws::Caught), // sub_141451ba0
    Replacement::draft_throws(0x141452010, "ui_141452010", fn_141452010 as *const (), Throws::Caught), // sub_141452010
    Replacement::draft_throws(0x1414525e0, "ui_1414525e0", fn_1414525e0 as *const (), Throws::Caught), // sub_1414525e0
    Replacement::draft_throws(0x141452a50, "ui_141452a50", fn_141452a50 as *const (), Throws::Caught), // sub_141452a50
    Replacement::draft_throws(0x141452ec0, "ui_141452ec0", fn_141452ec0 as *const (), Throws::Caught), // sub_141452ec0
    Replacement::draft_throws(0x1414532f0, "ui_1414532f0", fn_1414532f0 as *const (), Throws::Caught), // sub_1414532f0
    Replacement::draft_throws(0x141453720, "ui_141453720", fn_141453720 as *const (), Throws::Caught), // sub_141453720
    Replacement::draft_throws(0x141453b50, "ui_141453b50", fn_141453b50 as *const (), Throws::Caught), // sub_141453b50
    Replacement::draft_throws(0x141453f80, "ui_141453f80", fn_141453f80 as *const (), Throws::Caught), // sub_141453f80
    Replacement::draft_throws(0x1414543b0, "ui_1414543b0", fn_1414543b0 as *const (), Throws::Caught), // sub_1414543b0
    Replacement::draft_throws(0x1414547e0, "ui_1414547e0", fn_1414547e0 as *const (), Throws::Caught), // sub_1414547e0
    Replacement::draft_throws(0x141454d70, "ui_141454d70", fn_141454d70 as *const (), Throws::Caught), // sub_141454d70
    Replacement::draft_throws(0x1414551a0, "ui_1414551a0", fn_1414551a0 as *const (), Throws::Caught), // sub_1414551a0
    Replacement::draft_throws(0x1414555d0, "ui_1414555d0", fn_1414555d0 as *const (), Throws::Caught), // sub_1414555d0
    Replacement::draft_throws(0x141455920, "ui_141455920", fn_141455920 as *const (), Throws::Caught), // sub_141455920
    Replacement::draft_throws(0x141455c70, "ui_141455c70", fn_141455c70 as *const (), Throws::Caught), // sub_141455c70
    Replacement::draft_throws(0x141455fc0, "ui_141455fc0", fn_141455fc0 as *const (), Throws::Caught), // sub_141455fc0
    Replacement::draft_throws(0x141456310, "ui_141456310", fn_141456310 as *const (), Throws::Caught), // sub_141456310
    Replacement::draft_throws(0x141456660, "ui_141456660", fn_141456660 as *const (), Throws::Caught), // sub_141456660
    Replacement::draft_throws(0x1414569b0, "ui_1414569b0", fn_1414569b0 as *const (), Throws::Caught), // sub_1414569b0
    Replacement::draft_throws(0x141456d00, "ui_141456d00", fn_141456d00 as *const (), Throws::Caught), // sub_141456d00
    Replacement::draft_throws(0x141457050, "ui_141457050", fn_141457050 as *const (), Throws::Caught), // sub_141457050
    Replacement::draft_throws(0x1414573a0, "ui_1414573a0", fn_1414573a0 as *const (), Throws::Caught), // sub_1414573a0
    Replacement::draft_throws(0x141457810, "ui_141457810", fn_141457810 as *const (), Throws::Caught), // sub_141457810
    Replacement::draft_throws(0x141457c80, "ui_141457c80", fn_141457c80 as *const (), Throws::Caught), // sub_141457c80
    Replacement::draft_throws(0x1414580b0, "ui_1414580b0", fn_1414580b0 as *const (), Throws::Caught), // sub_1414580b0
    Replacement::draft_throws(0x1414584e0, "ui_1414584e0", fn_1414584e0 as *const (), Throws::Caught), // sub_1414584e0
    Replacement::draft_throws(0x141458830, "ui_141458830", fn_141458830 as *const (), Throws::Caught), // sub_141458830
    Replacement::draft_throws(0x141458b80, "ui_141458b80", fn_141458b80 as *const (), Throws::Caught), // sub_141458b80
    Replacement::draft_throws(0x141458ff0, "ui_141458ff0", fn_141458ff0 as *const (), Throws::Caught), // sub_141458ff0
    Replacement::draft_throws(0x141459460, "ui_141459460", fn_141459460 as *const (), Throws::Caught), // sub_141459460
    Replacement::draft_throws(0x1414598d0, "ui_1414598d0", fn_1414598d0 as *const (), Throws::Caught), // sub_1414598d0
    Replacement::draft_throws(0x141459d40, "ui_141459d40", fn_141459d40 as *const (), Throws::Caught), // sub_141459d40
    Replacement::draft_throws(0x14145a170, "ui_14145a170", fn_14145a170 as *const (), Throws::Caught), // sub_14145a170
    Replacement::draft_throws(0x14145a5a0, "ui_14145a5a0", fn_14145a5a0 as *const (), Throws::Caught), // sub_14145a5a0
    Replacement::draft_throws(0x14145a9d0, "ui_14145a9d0", fn_14145a9d0 as *const (), Throws::Caught), // sub_14145a9d0
    Replacement::draft_throws(0x14145ae00, "ui_14145ae00", fn_14145ae00 as *const (), Throws::Caught), // sub_14145ae00
    Replacement::draft_throws(0x14145b150, "ui_14145b150", fn_14145b150 as *const (), Throws::Caught), // sub_14145b150
    Replacement::draft_throws(0x14145b4a0, "ui_14145b4a0", fn_14145b4a0 as *const (), Throws::Caught), // sub_14145b4a0
    Replacement::draft_throws(0x14145b7f0, "ui_14145b7f0", fn_14145b7f0 as *const (), Throws::Caught), // sub_14145b7f0
    Replacement::draft_throws(0x14145bb40, "ui_14145bb40", fn_14145bb40 as *const (), Throws::Caught), // sub_14145bb40
    Replacement::draft_throws(0x14145c110, "ui_14145c110", fn_14145c110 as *const (), Throws::Caught), // sub_14145c110
    Replacement::draft_throws(0x14145c6b0, "ui_14145c6b0", fn_14145c6b0 as *const (), Throws::Caught), // sub_14145c6b0
    Replacement::draft(0x14145ca00, "ui_14145ca00", fn_14145ca00 as *const ()), // sub_14145ca00
    Replacement::draft(0x14145cad0, "ui_14145cad0", fn_14145cad0 as *const ()), // ui::Page::ui::Page::vfunc_0
    Replacement::draft(0x14145cb10, "unknown_14145cb10", fn_14145cb10 as *const ()), // sub_14145cb10
    Replacement::draft(0x14145cd10, "unknown_14145cd10", fn_14145cd10 as *const ()), // sub_14145cd10
    Replacement::draft(0x14145ce20, "unknown_14145ce20", fn_14145ce20 as *const ()), // sub_14145ce20
    Replacement::draft(0x14145ce90, "unknown_14145ce90", tpl_2030::<0x14145ce20, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14145ce90
    Replacement::draft(0x14145cf40, "unknown_14145cf40", fn_14145cf40 as *const ()), // sub_14145cf40
    Replacement::draft(0x14145cfd0, "unknown_14145cfd0", fn_14145cfd0 as *const ()), // sub_14145cfd0
    Replacement::draft(0x14145d090, "unknown_14145d090", fn_14145d090 as *const ()), // sub_14145d090
    Replacement::draft(0x14145d100, "unknown_14145d100", tpl_19156::<0x3, 0x4, 0x8, 0x14240aa00, 0x28, 0x140f60310> as *const ()), // sub_14145d100
    Replacement::draft(0x14145d170, "unknown_14145d170", tpl_24505::<0x8, 0x28, 0x1401eedc0, 0x14240ab10> as *const ()), // sub_14145d170
    Replacement::draft(0x14145d1b0, "cohtml_14145d1b0", tpl_1676::<0x142646d58, 0x14309f708, 0x143419b40, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<64> ui::Page::PageContext::*>::cohtml::FieldProperty<r::InplaceString<64> ui::Pag
    Replacement::draft(0x14145d210, "cohtml_14145d210", fn_14145d210 as *const ()), // sub_14145d210
    Replacement::draft(0x14145d280, "cohtml_14145d280", fn_14145d280 as *const ()), // sub_14145d280
    Replacement::draft(0x14145d4c0, "cohtml_14145d4c0", fn_14145d4c0 as *const ()), // sub_14145d4c0
    Replacement::draft(0x14145d680, "cohtml_14145d680", fn_14145d680 as *const ()), // sub_14145d680
    Replacement::draft(0x14145d920, "cohtml_14145d920", fn_14145d920 as *const ()), // sub_14145d920
    Replacement::draft(0x14145dbb0, "cohtml_14145dbb0", fn_14145dbb0 as *const ()), // cohtml::EventHandler<bool (ui::Page::*)(unsigned int)>::cohtml::EventHandler<bool (ui::Page::*)(unsigned int)>::vfunc_1
    Replacement::draft(0x14145dc40, "cohtml_14145dc40", fn_14145dc40 as *const ()), // cohtml::EventHandler<unsigned int (ui::Page::*)(const r::InplaceString<32>&)>::cohtml::EventHandler<unsigned int (ui::Pa
    Replacement::draft(0x14145dce0, "cohtml_14145dce0", fn_14145dce0 as *const ()), // cohtml::EventHandler<void (ui::Page::*)(unsigned int)>::cohtml::EventHandler<void (ui::Page::*)(unsigned int)>::vfunc_1
    Replacement::draft(0x14145dd40, "unknown_14145dd40", tpl_1689::<0x28, 0x1425015d0, 0x48> as *const ()), // sub_14145dd40
    Replacement::draft(0x14145dd80, "unknown_14145dd80", fn_14145dd80 as *const ()), // sub_14145dd80
    Replacement::draft(0x14145de20, "unknown_14145de20", tpl_20046::<0x14145d210> as *const ()), // sub_14145de20
    Replacement::draft(0x14145de80, "unknown_14145de80", fn_14145de80 as *const ()), // sub_14145de80
    Replacement::draft(0x14145dee0, "unknown_14145dee0", fn_14145dee0 as *const ()), // sub_14145dee0
    Replacement::draft(0x14145df20, "unknown_14145df20", tpl_23966::<0x8, 0x4, 0x14240ab10> as *const ()), // sub_14145df20
    Replacement::draft(0x14145df60, "renoir_14145df60", tpl_893::<0x143419fc8, 0x141834d30> as *const ()), // renoir::IBackendAllocator::renoir::IBackendAllocator::vfunc_0
    Replacement::draft(0x14145df90, "ui_14145df90", fn_14145df90 as *const ()), // ui::MemoryAllocator::ui::MemoryAllocator::vfunc_7
    Replacement::draft(0x14145dfe0, "ui_14145dfe0", tpl_26429::<0x142442de0> as *const ()), // ui::BreakIterator::ui::BreakIterator::vfunc_1
    Replacement::draft(0x14145e070, "ui_14145e070", fn_14145e070 as *const ()), // ui::BreakIterator::ui::BreakIterator::vfunc_11
    Replacement::draft(0x14145e150, "ui_14145e150", fn_14145e150 as *const ()), // ui::BreakIterator::ui::BreakIterator::vfunc_0
    Replacement::draft(0x14145e1c0, "ui_14145e1c0", fn_14145e1c0 as *const ()), // ui::InternationalizationManager::ui::InternationalizationManager::vfunc_0
    Replacement::draft(0x14145e270, "ui_14145e270", fn_14145e270 as *const ()), // sub_14145e270
    Replacement::draft(0x14145e380, "ui_14145e380", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14341a158, 0x8> as *const ()), // sub_14145e380
    Replacement::draft(0x14145e400, "ui_14145e400", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14341a158, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_14145e400
    Replacement::draft_throws(0x14145e4a0, "ui_14145e4a0", fn_14145e4a0 as *const (), Throws::Caught), // sub_14145e4a0
    Replacement::draft_throws(0x14145e530, "ui_14145e530", tpl_1908::<0x143882298, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_14145e530
    Replacement::draft(0x14145e5b0, "ui_14145e5b0", fn_14145e5b0 as *const ()), // sub_14145e5b0
    Replacement::draft(0x14145e690, "ui_14145e690", fn_14145e690 as *const ()), // sub_14145e690
    Replacement::draft_throws(0x14145e950, "ui_14145e950", fn_14145e950 as *const (), Throws::Caught), // sub_14145e950
    Replacement::draft_throws(0x14145eae0, "ui_14145eae0", tpl_1905::<0x1438822a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143881f80> as *const (), Throws::Discard), // sub_14145eae0
    Replacement::draft(0x14145eb90, "ui_14145eb90", fn_14145eb90 as *const ()), // ui::FontResource::ui::FontResource::vfunc_0
    Replacement::draft(0x14145ec00, "unknown_14145ec00", fn_14145ec00 as *const ()), // sub_14145ec00
    Replacement::draft(0x14145ed60, "unknown_14145ed60", fn_14145ed60 as *const ()), // sub_14145ed60
    Replacement::draft(0x14145ef50, "unknown_14145ef50", fn_14145ef50 as *const ()), // sub_14145ef50
    Replacement::draft_throws(0x14145efe0, "unknown_14145efe0", fn_14145efe0 as *const (), Throws::Discard), // sub_14145efe0
    Replacement::draft(0x14145f020, "unknown_14145f020", fn_14145f020 as *const ()), // sub_14145f020
    Replacement::draft_throws(0x14145f060, "std_14145f060", fn_14145f060 as *const (), Throws::Discard), // sub_14145f060
    Replacement::draft_throws(0x14145f150, "unknown_14145f150", fn_14145f150 as *const (), Throws::Caught), // sub_14145f150
    Replacement::draft_throws(0x14145f310, "unknown_14145f310", fn_14145f310 as *const (), Throws::Caught), // sub_14145f310
    Replacement::draft_throws(0x14145f3c0, "rapidjson_14145f3c0", fn_14145f3c0 as *const (), Throws::Discard), // sub_14145f3c0
    Replacement::draft(0x14145f8b0, "unknown_14145f8b0", fn_14145f8b0 as *const ()), // sub_14145f8b0
    Replacement::draft(0x14145f910, "unknown_14145f910", fn_14145f910 as *const ()), // sub_14145f910
    Replacement::draft(0x14145f9c0, "unknown_14145f9c0", fn_14145f9c0 as *const ()), // sub_14145f9c0
    Replacement::draft(0x14145fa60, "unknown_14145fa60", fn_14145fa60 as *const ()), // sub_14145fa60
    Replacement::draft_throws(0x14145fba0, "unknown_14145fba0", fn_14145fba0 as *const (), Throws::Discard), // sub_14145fba0
    Replacement::draft_throws(0x14145fcb0, "unknown_14145fcb0", fn_14145fcb0 as *const (), Throws::Caught), // sub_14145fcb0
    Replacement::draft(0x14145ffa0, "unknown_14145ffa0", fn_14145ffa0 as *const ()), // sub_14145ffa0
    Replacement::draft_throws(0x141460180, "unknown_141460180", fn_141460180 as *const (), Throws::Caught), // sub_141460180
    Replacement::draft(0x1414616f0, "unknown_1414616f0", tpl_365::<0x141461990> as *const ()), // sub_1414616f0
    Replacement::draft(0x141461700, "unknown_141461700", fn_141461700 as *const ()), // sub_141461700
    Replacement::draft(0x141461740, "unknown_141461740", fn_141461740 as *const ()), // sub_141461740
    Replacement::draft(0x141461770, "unknown_141461770", fn_141461770 as *const ()), // sub_141461770
    Replacement::draft(0x141461860, "std_141461860", fn_141461860 as *const ()), // sub_141461860
    Replacement::draft(0x141461990, "unknown_141461990", tpl_26448::<0x14341efb0, 0x8, 0x20, 0x18, 0xfffffffffffffff8, 0x141834d30, 0x1426467f0, 0x1414619eb> as *const ()), // sub_141461990
    Replacement::draft(0x1414619f0, "unknown_1414619f0", fn_1414619f0 as *const ()), // sub_1414619f0
    Replacement::draft(0x141461a50, "unknown_141461a50", fn_141461a50 as *const ()), // sub_141461a50
    Replacement::draft_throws(0x141461a90, "std_141461a90", tpl_26451::<0x28, 0x10, 0x18, 0x1402050d0> as *const (), Throws::Discard), // std::numpunct<char>::std::numpunct<char>::vfunc_7
    Replacement::draft_throws(0x141461ad0, "std_141461ad0", tpl_26451::<0x20, 0x10, 0x18, 0x1402050d0> as *const (), Throws::Discard), // std::numpunct<char>::std::numpunct<char>::vfunc_6
    Replacement::draft_throws(0x141461b10, "std_141461b10", fn_141461b10 as *const (), Throws::Discard), // std::numpunct<char>::std::numpunct<char>::vfunc_5
    Replacement::draft(0x141461b50, "std_141461b50", tpl_4544::<0x19> as *const ()), // std::numpunct<char>::std::numpunct<char>::vfunc_4
    Replacement::draft(0x141461b60, "std_141461b60", tpl_4544::<0x18> as *const ()), // std::numpunct<char>::std::numpunct<char>::vfunc_3
    Replacement::draft(0x141461b70, "std_141461b70", fn_141461b70 as *const ()), // std::numpunct<char>::std::numpunct<char>::vfunc_0
    Replacement::draft_throws(0x141461bd0, "fmt_141461bd0", fn_141461bd0 as *const (), Throws::Discard), // fmt::v9::basic_memory_buffer<char,500,std::allocator<char>>::fmt::v9::basic_memory_buffer<char,500,std::allocator<char>>
    Replacement::draft_throws(0x141461c80, "fmt_141461c80", fn_141461c80 as *const (), Throws::Caught), // sub_141461c80
    Replacement::draft_throws(0x141461dd0, "fmt_141461dd0", fn_141461dd0 as *const (), Throws::Discard), // fmt::v9::basic_memory_buffer<int,500,std::allocator<int>>::fmt::v9::basic_memory_buffer<int,500,std::allocator<int>>::vf
    Replacement::draft(0x141461ec0, "fmt_141461ec0", fn_141461ec0 as *const ()), // sub_141461ec0
    Replacement::draft(0x141461f30, "fmt_141461f30", fn_141461f30 as *const ()), // sub_141461f30
    Replacement::draft_throws(0x141462180, "fmt_141462180", fn_141462180 as *const (), Throws::Caught), // sub_141462180
    Replacement::draft_throws(0x141462950, "fmt_141462950", fn_141462950 as *const (), Throws::Caught), // sub_141462950
    Replacement::draft_throws(0x141462a00, "fmt_141462a00", fn_141462a00 as *const (), Throws::Discard), // sub_141462a00
    Replacement::draft(0x141462ac0, "fmt_141462ac0", tpl_402::<0x1426680a0, 0x8, 0x142646338, 0x14341f118> as *const ()), // sub_141462ac0
    Replacement::draft(0x141462b00, "fmt_141462b00", fn_141462b00 as *const ()), // fmt::v9::format_facet<std::locale>::fmt::v9::format_facet<std::locale>::vfunc_0
    Replacement::draft_throws(0x141462b60, "unknown_141462b60", fn_141462b60 as *const (), Throws::Caught), // sub_141462b60
    Replacement::draft(0x141462d20, "unknown_141462d20", fn_141462d20 as *const ()), // sub_141462d20
    Replacement::draft(0x141462d60, "unknown_141462d60", fn_141462d60 as *const ()), // sub_141462d60
    Replacement::draft_throws(0x1414634f0, "unknown_1414634f0", fn_1414634f0 as *const (), Throws::Caught), // sub_1414634f0
    Replacement::draft_throws(0x1414635a0, "unknown_1414635a0", fn_1414635a0 as *const (), Throws::Caught), // sub_1414635a0
    Replacement::draft_throws(0x141463800, "unknown_141463800", fn_141463800 as *const (), Throws::Caught), // sub_141463800
    Replacement::draft_throws(0x141464020, "unknown_141464020", fn_141464020 as *const (), Throws::Caught), // sub_141464020
    Replacement::draft(0x1414643a0, "unknown_1414643a0", fn_1414643a0 as *const ()), // sub_1414643a0
    Replacement::draft_throws(0x141464500, "unknown_141464500", fn_141464500 as *const (), Throws::Caught), // sub_141464500
    Replacement::draft(0x1414645d0, "unknown_1414645d0", fn_1414645d0 as *const ()), // sub_1414645d0
    Replacement::draft_throws(0x1414645f0, "unknown_1414645f0", fn_1414645f0 as *const (), Throws::Caught), // sub_1414645f0
    Replacement::draft_throws(0x141464680, "std_141464680", fn_141464680 as *const (), Throws::Caught), // sub_141464680
    Replacement::draft_throws(0x1414647e0, "unknown_1414647e0", fn_1414647e0 as *const (), Throws::Caught), // sub_1414647e0
    Replacement::draft_throws(0x141464a60, "unknown_141464a60", fn_141464a60 as *const (), Throws::Caught), // sub_141464a60
    Replacement::draft(0x141465190, "unknown_141465190", fn_141465190 as *const ()), // sub_141465190
    Replacement::draft(0x1414652c0, "unknown_1414652c0", fn_1414652c0 as *const ()), // sub_1414652c0
    Replacement::draft_throws(0x141465580, "unknown_141465580", fn_141465580 as *const (), Throws::Discard), // sub_141465580
    Replacement::draft_throws(0x141465a30, "unknown_141465a30", fn_141465a30 as *const (), Throws::Discard), // sub_141465a30
    Replacement::draft_throws(0x141465ae0, "unknown_141465ae0", fn_141465ae0 as *const (), Throws::Caught), // sub_141465ae0
    Replacement::draft_throws(0x141465be0, "unknown_141465be0", fn_141465be0 as *const (), Throws::Caught), // sub_141465be0
    Replacement::draft_throws(0x141465da0, "unknown_141465da0", fn_141465da0 as *const (), Throws::Caught), // sub_141465da0
    Replacement::draft_throws(0x141465f60, "unknown_141465f60", fn_141465f60 as *const (), Throws::Discard), // sub_141465f60
    Replacement::draft_throws(0x141466030, "unknown_141466030", fn_141466030 as *const (), Throws::Discard), // sub_141466030
    Replacement::draft_throws(0x141466100, "unknown_141466100", fn_141466100 as *const (), Throws::Discard), // sub_141466100
    Replacement::draft_throws(0x1414661d0, "unknown_1414661d0", fn_1414661d0 as *const (), Throws::Caught), // sub_1414661d0
    Replacement::draft(0x141466360, "unknown_141466360", fn_141466360 as *const ()), // sub_141466360
    Replacement::draft(0x1414663f0, "nlohmann_1414663f0", fn_1414663f0 as *const ()), // sub_1414663f0
    Replacement::draft(0x141466410, "unknown_141466410", fn_141466410 as *const ()), // sub_141466410
    Replacement::draft(0x141466640, "unknown_141466640", fn_141466640 as *const ()), // sub_141466640
    Replacement::draft(0x141466800, "unknown_141466800", fn_141466800 as *const ()), // sub_141466800
    Replacement::draft(0x141466910, "unknown_141466910", fn_141466910 as *const ()), // sub_141466910
    Replacement::draft_throws(0x141466a40, "unknown_141466a40", fn_141466a40 as *const (), Throws::Caught), // sub_141466a40
    Replacement::draft_throws(0x141466c60, "unknown_141466c60", fn_141466c60 as *const (), Throws::Caught), // sub_141466c60
    Replacement::draft_throws(0x141466e80, "unknown_141466e80", fn_141466e80 as *const (), Throws::Caught), // sub_141466e80
    Replacement::draft_throws(0x141467210, "unknown_141467210", fn_141467210 as *const (), Throws::Caught), // sub_141467210
    Replacement::draft_throws(0x141467590, "unknown_141467590", fn_141467590 as *const (), Throws::Caught), // sub_141467590
    Replacement::draft_throws(0x141467910, "unknown_141467910", fn_141467910 as *const (), Throws::Caught), // sub_141467910
    Replacement::draft(0x141467ae0, "unknown_141467ae0", tpl_26448::<0x14341f248, 0x8, 0x20, 0x18, 0xfffffffffffffff8, 0x141834d30, 0x1426467f0, 0x141467b3b> as *const ()), // sub_141467ae0
    Replacement::draft_throws(0x141467b40, "unknown_141467b40", fn_141467b40 as *const (), Throws::Caught), // sub_141467b40
    Replacement::draft(0x141468130, "curl_141468130", fn_141468130 as *const ()), // sub_141468130
    Replacement::draft(0x141468280, "curl_141468280", fn_141468280 as *const ()), // sub_141468280
    Replacement::draft(0x1414683b0, "curl_1414683b0", fn_1414683b0 as *const ()), // sub_1414683b0
    Replacement::draft(0x141468900, "curl_141468900", fn_141468900 as *const ()), // sub_141468900
    Replacement::draft(0x141468a10, "curl_141468a10", fn_141468a10 as *const ()), // sub_141468a10
    Replacement::draft(0x141468c50, "curl_141468c50", fn_141468c50 as *const ()), // sub_141468c50
    Replacement::draft(0x141468e30, "curl_141468e30", fn_141468e30 as *const ()), // sub_141468e30
    Replacement::draft(0x141469400, "curl_141469400", fn_141469400 as *const ()), // sub_141469400
    Replacement::draft(0x141469510, "curl_141469510", fn_141469510 as *const ()), // sub_141469510
    Replacement::draft(0x141469750, "curl_141469750", fn_141469750 as *const ()), // sub_141469750
    Replacement::draft(0x141469930, "curl_141469930", fn_141469930 as *const ()), // sub_141469930
    Replacement::draft(0x141469ab0, "curl_141469ab0", fn_141469ab0 as *const ()), // sub_141469ab0
    Replacement::draft_throws(0x141469c10, "unknown_141469c10", fn_141469c10 as *const (), Throws::Caught), // sub_141469c10
    Replacement::draft(0x14146a200, "unknown_14146a200", fn_14146a200 as *const ()), // sub_14146a200
    Replacement::draft(0x14146a330, "unknown_14146a330", fn_14146a330 as *const ()), // sub_14146a330
    Replacement::draft_throws(0x14146a450, "unknown_14146a450", fn_14146a450 as *const (), Throws::Caught), // sub_14146a450
    Replacement::draft(0x14146a5b0, "unknown_14146a5b0", fn_14146a5b0 as *const ()), // sub_14146a5b0
    Replacement::draft_throws(0x14146a6d0, "nlohmann_14146a6d0", fn_14146a6d0 as *const (), Throws::Discard), // sub_14146a6d0
    Replacement::draft_throws(0x14146ab30, "unknown_14146ab30", fn_14146ab30 as *const (), Throws::Caught), // sub_14146ab30
    Replacement::draft_throws(0x14146af70, "unknown_14146af70", fn_14146af70 as *const (), Throws::Caught), // sub_14146af70
    Replacement::draft(0x14146b070, "unknown_14146b070", fn_14146b070 as *const ()), // sub_14146b070
    Replacement::draft(0x14146b140, "unknown_14146b140", fn_14146b140 as *const ()), // sub_14146b140
    Replacement::draft(0x14146b1e0, "unknown_14146b1e0", fn_14146b1e0 as *const ()), // sub_14146b1e0
    Replacement::draft(0x14146b350, "unknown_14146b350", fn_14146b350 as *const ()), // sub_14146b350
    Replacement::draft(0x14146b5e0, "unknown_14146b5e0", fn_14146b5e0 as *const ()), // sub_14146b5e0
    Replacement::draft_throws(0x14146b860, "unknown_14146b860", fn_14146b860 as *const (), Throws::Caught), // sub_14146b860
    Replacement::draft(0x14146bc80, "unknown_14146bc80", fn_14146bc80 as *const ()), // sub_14146bc80
    Replacement::draft(0x14146bde0, "unknown_14146bde0", fn_14146bde0 as *const ()), // sub_14146bde0
    Replacement::draft(0x14146c020, "unknown_14146c020", fn_14146c020 as *const ()), // sub_14146c020
    Replacement::draft(0x14146c290, "unknown_14146c290", fn_14146c290 as *const ()), // sub_14146c290
    Replacement::draft(0x14146c400, "unknown_14146c400", fn_14146c400 as *const ()), // sub_14146c400
    Replacement::draft(0x14146c570, "unknown_14146c570", fn_14146c570 as *const ()), // sub_14146c570
    Replacement::draft(0x14146c740, "unknown_14146c740", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141468900> as *const ()), // sub_14146c740
    Replacement::draft(0x14146c8a0, "unknown_14146c8a0", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141468a10> as *const ()), // sub_14146c8a0
    Replacement::draft(0x14146ca00, "unknown_14146ca00", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141468c50> as *const ()), // sub_14146ca00
    Replacement::draft(0x14146cb60, "unknown_14146cb60", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141468e30> as *const ()), // sub_14146cb60
    Replacement::draft(0x14146ccc0, "unknown_14146ccc0", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141469400> as *const ()), // sub_14146ccc0
    Replacement::draft(0x14146ce20, "unknown_14146ce20", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141469510> as *const ()), // sub_14146ce20
    Replacement::draft(0x14146cf80, "unknown_14146cf80", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141469750> as *const ()), // sub_14146cf80
    Replacement::draft(0x14146d0e0, "unknown_14146d0e0", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x141469930> as *const ()), // sub_14146d0e0
    Replacement::draft(0x14146d240, "unknown_14146d240", fn_14146d240 as *const ()), // sub_14146d240
    Replacement::draft(0x14146d3a0, "unknown_14146d3a0", fn_14146d3a0 as *const ()), // sub_14146d3a0
    Replacement::draft(0x14146d5a0, "unknown_14146d5a0", tpl_84722::<0x14307e0a8, 0x1401f9050, 0x141468130> as *const ()), // sub_14146d5a0
    Replacement::draft(0x14146d790, "unknown_14146d790", tpl_84722::<0x14307e0a8, 0x1401f9050, 0x141468280> as *const ()), // sub_14146d790
    Replacement::draft(0x14146d980, "unknown_14146d980", tpl_84722::<0x14307e0a8, 0x1401f9050, 0x1414683b0> as *const ()), // sub_14146d980
    Replacement::draft(0x14146db70, "unknown_14146db70", fn_14146db70 as *const ()), // sub_14146db70
    Replacement::draft(0x14146dd70, "unknown_14146dd70", fn_14146dd70 as *const ()), // sub_14146dd70
    Replacement::draft(0x14146df70, "unknown_14146df70", tpl_84722::<0x14307e0a8, 0x1401f9050, 0x14146a200> as *const ()), // sub_14146df70
    Replacement::draft(0x14146e160, "unknown_14146e160", tpl_84722::<0x14307e0a8, 0x1401f9050, 0x14146a330> as *const ()), // sub_14146e160
    Replacement::draft(0x14146e350, "unknown_14146e350", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x14146bc80> as *const ()), // sub_14146e350
    Replacement::draft(0x14146e4b0, "unknown_14146e4b0", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x14146bde0> as *const ()), // sub_14146e4b0
    Replacement::draft(0x14146e610, "unknown_14146e610", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x14146c020> as *const ()), // sub_14146e610
    Replacement::draft(0x14146e770, "unknown_14146e770", tpl_84719::<0x14307e0a8, 0x1401f9050, 0x14146c290> as *const ()), // sub_14146e770
    Replacement::draft(0x14146e8d0, "luau_14146e8d0", fn_14146e8d0 as *const ()), // Luau::ParseError::Luau::ParseError::vfunc_0
    Replacement::draft(0x14146e920, "unknown_14146e920", fn_14146e920 as *const ()), // sub_14146e920
    Replacement::draft_throws(0x14146e950, "unknown_14146e950", fn_14146e950 as *const (), Throws::Caught), // sub_14146e950
    Replacement::draft(0x14146e9c0, "unknown_14146e9c0", fn_14146e9c0 as *const ()), // sub_14146e9c0
    Replacement::draft(0x14146eb20, "unknown_14146eb20", tpl_19735::<0x14146ec60, 0x28, 0x14146ef90, 0x10, 0x14146ece0> as *const ()), // sub_14146eb20
    Replacement::draft(0x14146eb50, "unknown_14146eb50", fn_14146eb50 as *const ()), // sub_14146eb50
    Replacement::draft(0x14146eb60, "unknown_14146eb60", tpl_26520::<0x140203690> as *const ()), // sub_14146eb60
    Replacement::draft(0x14146eb70, "unknown_14146eb70", fn_14146eb70 as *const ()), // sub_14146eb70
    Replacement::draft_throws(0x14146ebd0, "unknown_14146ebd0", fn_14146ebd0 as *const (), Throws::Discard), // sub_14146ebd0
    Replacement::draft(0x14146ec60, "global_14146ec60", fn_14146ec60 as *const ()), // FID_conflict:~vector<GSISymbolEntry,std::allocator<GSISymbolEntry>_>
    Replacement::draft(0x14146ece0, "unknown_14146ece0", fn_14146ece0 as *const ()), // sub_14146ece0
    Replacement::draft(0x14146ed90, "unknown_14146ed90", tpl_365::<0x14146ef90> as *const ()), // sub_14146ed90
    Replacement::draft_throws(0x14146eda0, "unknown_14146eda0", fn_14146eda0 as *const (), Throws::Caught), // sub_14146eda0
    Replacement::draft(0x14146ee80, "unknown_14146ee80", fn_14146ee80 as *const ()), // sub_14146ee80
    Replacement::draft(0x14146ef00, "unknown_14146ef00", fn_14146ef00 as *const ()), // sub_14146ef00
    Replacement::draft(0x14146ef90, "unknown_14146ef90", fn_14146ef90 as *const ()), // sub_14146ef90
    Replacement::draft_throws(0x14146f050, "unknown_14146f050", fn_14146f050 as *const (), Throws::Discard), // sub_14146f050
    Replacement::draft(0x14146f1e0, "global_14146f1e0", fn_14146f1e0 as *const ()), // deallocate
    Replacement::draft(0x14146f230, "unknown_14146f230", fn_14146f230 as *const ()), // sub_14146f230
    Replacement::draft(0x14146f250, "unknown_14146f250", fn_14146f250 as *const ()), // sub_14146f250
    Replacement::draft_throws(0x14146f270, "luau_14146f270", tpl_26529::<0x8, 0x2000, 0x28, 0x141834d64, 0x143422e38, 0x143d015d0, 0x143422d48, 0xc, 0x20> as *const (), Throws::Discard), // sub_14146f270
    Replacement::draft_throws(0x14146f320, "luau_14146f320", tpl_26529::<0x8, 0x2000, 0x28, 0x141834d64, 0x143422e38, 0x143d01730, 0x143422870, 0xc, 0x20> as *const (), Throws::Discard), // sub_14146f320
    Replacement::draft(0x14146f3d0, "luau_14146f3d0", fn_14146f3d0 as *const ()), // sub_14146f3d0
    Replacement::draft(0x14146f3f0, "luau_14146f3f0", fn_14146f3f0 as *const ()), // sub_14146f3f0
    Replacement::draft_throws(0x14146f410, "luau_14146f410", fn_14146f410 as *const (), Throws::Discard), // sub_14146f410
    Replacement::draft_throws(0x14146f4f0, "luau_14146f4f0", fn_14146f4f0 as *const (), Throws::Discard), // sub_14146f4f0
    Replacement::draft_throws(0x14146f5c0, "luau_14146f5c0", tpl_26534::<0x8, 0x2000, 0x28, 0x141834d64, 0x143422e38, 0x143d01574, 0x143422848, 0xc, 0x20> as *const (), Throws::Discard), // sub_14146f5c0
    Replacement::draft_throws(0x14146f680, "luau_14146f680", fn_14146f680 as *const (), Throws::Discard), // sub_14146f680
    Replacement::draft_throws(0x14146f740, "luau_14146f740", tpl_26534::<0x8, 0x2000, 0x28, 0x141834d64, 0x143422e38, 0x143d015b4, 0x143422a28, 0xc, 0x20> as *const (), Throws::Discard), // sub_14146f740
    Replacement::draft_throws(0x14146f800, "luau_14146f800", fn_14146f800 as *const (), Throws::Discard), // sub_14146f800
    Replacement::draft_throws(0x14146f900, "luau_14146f900", fn_14146f900 as *const (), Throws::Discard), // sub_14146f900
    Replacement::draft(0x14146f9d0, "luau_14146f9d0", fn_14146f9d0 as *const ()), // sub_14146f9d0
    Replacement::draft_throws(0x14146f9f0, "luau_14146f9f0", fn_14146f9f0 as *const (), Throws::Discard), // sub_14146f9f0
    Replacement::draft_throws(0x14146fa70, "luau_14146fa70", fn_14146fa70 as *const (), Throws::Discard), // sub_14146fa70
    Replacement::draft(0x14146fb00, "luau_14146fb00", fn_14146fb00 as *const ()), // sub_14146fb00
    Replacement::draft(0x14146fb40, "luau_14146fb40", fn_14146fb40 as *const ()), // sub_14146fb40
    Replacement::draft(0x14146fb50, "luau_14146fb50", fn_14146fb50 as *const ()), // sub_14146fb50
    Replacement::draft_throws(0x14146fbe0, "luau_14146fbe0", fn_14146fbe0 as *const (), Throws::Discard), // sub_14146fbe0
    Replacement::draft_throws(0x14146fd00, "luau_14146fd00", fn_14146fd00 as *const (), Throws::Caught), // sub_14146fd00
    Replacement::draft_throws(0x14146feb0, "luau_14146feb0", fn_14146feb0 as *const (), Throws::Discard), // sub_14146feb0
    Replacement::draft_throws(0x14146ff90, "luau_14146ff90", fn_14146ff90 as *const (), Throws::Discard), // sub_14146ff90
    Replacement::draft_throws(0x141470170, "luau_141470170", fn_141470170 as *const (), Throws::Discard), // sub_141470170
    Replacement::draft_throws(0x1414703a0, "luau_1414703a0", fn_1414703a0 as *const (), Throws::Caught), // sub_1414703a0
    Replacement::draft(0x141470510, "luau_141470510", fn_141470510 as *const ()), // sub_141470510
    Replacement::draft(0x1414705f0, "luau_1414705f0", fn_1414705f0 as *const ()), // sub_1414705f0
    Replacement::draft(0x1414706e0, "luau_1414706e0", fn_1414706e0 as *const ()), // sub_1414706e0
    Replacement::draft_throws(0x141470760, "luau_141470760", fn_141470760 as *const (), Throws::Discard), // sub_141470760
    Replacement::draft_throws(0x1414708d0, "luau_1414708d0", fn_1414708d0 as *const (), Throws::Discard), // sub_1414708d0
    Replacement::draft_throws(0x141470ab0, "luau_141470ab0", fn_141470ab0 as *const (), Throws::Discard), // sub_141470ab0
    Replacement::draft_throws(0x141470c10, "luau_141470c10", fn_141470c10 as *const (), Throws::Discard), // sub_141470c10
    Replacement::draft_throws(0x141470d30, "luau_141470d30", fn_141470d30 as *const (), Throws::Discard), // sub_141470d30
    Replacement::draft_throws(0x141470eb0, "luau_141470eb0", fn_141470eb0 as *const (), Throws::Discard), // sub_141470eb0
    Replacement::draft(0x141470f70, "luau_141470f70", fn_141470f70 as *const ()), // sub_141470f70
    Replacement::draft(0x141471080, "luau_141471080", fn_141471080 as *const ()), // sub_141471080
    Replacement::draft_throws(0x141471180, "luau_141471180", fn_141471180 as *const (), Throws::Discard), // sub_141471180
    Replacement::draft(0x141471420, "luau_141471420", fn_141471420 as *const ()), // sub_141471420
    Replacement::draft(0x141471560, "luau_141471560", fn_141471560 as *const ()), // sub_141471560
    Replacement::draft(0x141471620, "luau_141471620", fn_141471620 as *const ()), // sub_141471620
    Replacement::draft(0x1414716e0, "luau_1414716e0", fn_1414716e0 as *const ()), // sub_1414716e0
    Replacement::draft(0x141471800, "luau_141471800", fn_141471800 as *const ()), // sub_141471800
    Replacement::draft(0x141471940, "luau_141471940", fn_141471940 as *const ()), // sub_141471940
    Replacement::draft(0x141471a30, "luau_141471a30", fn_141471a30 as *const ()), // sub_141471a30
    Replacement::draft(0x141471b20, "luau_141471b20", fn_141471b20 as *const ()), // sub_141471b20
    Replacement::draft_throws(0x141471b80, "luau_141471b80", fn_141471b80 as *const (), Throws::Discard), // sub_141471b80
    Replacement::draft(0x141471ec0, "luau_141471ec0", fn_141471ec0 as *const ()), // sub_141471ec0
    Replacement::draft_throws(0x141472040, "luau_141472040", fn_141472040 as *const (), Throws::Discard), // sub_141472040
    Replacement::draft_throws(0x141472200, "luau_141472200", fn_141472200 as *const (), Throws::Discard), // sub_141472200
    Replacement::draft_throws(0x141472540, "luau_141472540", fn_141472540 as *const (), Throws::Discard), // sub_141472540
    Replacement::draft_throws(0x141472790, "luau_141472790", fn_141472790 as *const (), Throws::Discard), // sub_141472790
    Replacement::draft_throws(0x141472810, "luau_141472810", fn_141472810 as *const (), Throws::Discard), // sub_141472810
    Replacement::draft_throws(0x1414728a0, "luau_1414728a0", fn_1414728a0 as *const (), Throws::Discard), // sub_1414728a0
    Replacement::draft(0x141472940, "luau_141472940", fn_141472940 as *const ()), // sub_141472940
    Replacement::draft(0x1414729d0, "luau_1414729d0", fn_1414729d0 as *const ()), // sub_1414729d0
    Replacement::draft_throws(0x141472a90, "luau_141472a90", fn_141472a90 as *const (), Throws::Discard), // sub_141472a90
    Replacement::draft_throws(0x141472c00, "luau_141472c00", fn_141472c00 as *const (), Throws::Caught), // sub_141472c00
    Replacement::draft(0x141472e40, "luau_141472e40", fn_141472e40 as *const ()), // sub_141472e40
    Replacement::draft_throws(0x1414731f0, "luau_1414731f0", fn_1414731f0 as *const (), Throws::Discard), // sub_1414731f0
    Replacement::draft_throws(0x1414733f0, "luau_1414733f0", fn_1414733f0 as *const (), Throws::Discard), // sub_1414733f0
    Replacement::draft_throws(0x141473510, "luau_141473510", fn_141473510 as *const (), Throws::Discard), // sub_141473510
    Replacement::draft(0x141473630, "luau_141473630", fn_141473630 as *const ()), // sub_141473630
    Replacement::draft_throws(0x1414737c0, "luau_1414737c0", fn_1414737c0 as *const (), Throws::Caught), // sub_1414737c0
    Replacement::draft_throws(0x141473930, "luau_141473930", fn_141473930 as *const (), Throws::Discard), // sub_141473930
    Replacement::draft(0x141473a10, "luau_141473a10", fn_141473a10 as *const ()), // sub_141473a10
    Replacement::draft_throws(0x141473a90, "luau_141473a90", fn_141473a90 as *const (), Throws::Caught), // sub_141473a90
    Replacement::draft(0x141473b20, "luau_141473b20", fn_141473b20 as *const ()), // sub_141473b20
    Replacement::draft(0x141473be0, "luau_141473be0", fn_141473be0 as *const ()), // sub_141473be0
    Replacement::draft(0x141473d60, "luau_141473d60", tpl_1674::<0x1434245a8> as *const ()), // sub_141473d60
    Replacement::draft(0x141473d70, "luau_141473d70", tpl_5167::<0x1c0> as *const ()), // Luau::AstVisitor::Luau::AstVisitor::vfunc_35
    Replacement::draft(0x141473d80, "luau_141473d80", tpl_5167::<0x1b8> as *const ()), // Luau::AstVisitor::Luau::AstVisitor::vfunc_36
    Replacement::draft(0x141473d90, "luau_141473d90", tpl_5167::<0x118> as *const ()), // Luau::AstVisitor::Luau::AstVisitor::vfunc_15
    Replacement::draft(0x141473da0, "luau_141473da0", tpl_5167::<0x70> as *const ()), // Luau::AstVisitor::Luau::AstVisitor::vfunc_5
    Replacement::draft(0x141473db0, "luau_141473db0", tpl_5167::<0x20> as *const ()), // Luau::AstVisitor::Luau::AstVisitor::vfunc_1
    Replacement::draft(0x141473dc0, "luau_141473dc0", tpl_893::<0x1434245a8, 0x141834d30> as *const ()), // Luau::AstVisitor::Luau::AstVisitor::vfunc_0
    Replacement::draft(0x141473df0, "luau_141473df0", fn_141473df0 as *const ()), // sub_141473df0
    Replacement::draft(0x141473e90, "luau_141473e90", fn_141473e90 as *const ()), // Luau::CompileError::Luau::CompileError::vfunc_0
    Replacement::draft_throws(0x141473ef0, "luau_141473ef0", fn_141473ef0 as *const (), Throws::Caught), // sub_141473ef0
    Replacement::draft_throws(0x141473f60, "luau_141473f60", fn_141473f60 as *const (), Throws::Caught), // sub_141473f60
    Replacement::draft(0x141474160, "luau_141474160", fn_141474160 as *const ()), // sub_141474160
    Replacement::draft_throws(0x141474200, "luau_141474200", fn_141474200 as *const (), Throws::Discard), // sub_141474200
    Replacement::draft(0x141474390, "luau_141474390", fn_141474390 as *const ()), // sub_141474390
    Replacement::draft_throws(0x141474430, "luau_141474430", fn_141474430 as *const (), Throws::Discard), // sub_141474430
    Replacement::draft(0x141474490, "luau_141474490", fn_141474490 as *const ()), // sub_141474490
    Replacement::draft_throws(0x141474560, "luau_141474560", fn_141474560 as *const (), Throws::Caught), // sub_141474560
    Replacement::draft(0x141474ba0, "luau_141474ba0", fn_141474ba0 as *const ()), // sub_141474ba0
    Replacement::draft_throws(0x141474cc0, "luau_141474cc0", fn_141474cc0 as *const (), Throws::Caught), // sub_141474cc0
    Replacement::draft_throws(0x141474dd0, "luau_141474dd0", fn_141474dd0 as *const (), Throws::Caught), // sub_141474dd0
    Replacement::draft_throws(0x1414750b0, "luau_1414750b0", fn_1414750b0 as *const (), Throws::Caught), // sub_1414750b0
    Replacement::draft_throws(0x1414756d0, "luau_1414756d0", fn_1414756d0 as *const (), Throws::Discard), // sub_1414756d0
    Replacement::draft_throws(0x1414759f0, "luau_1414759f0", fn_1414759f0 as *const (), Throws::Caught), // sub_1414759f0
    Replacement::draft_throws(0x141476670, "luau_141476670", fn_141476670 as *const (), Throws::Caught), // sub_141476670
    Replacement::draft(0x1414770b0, "luau_1414770b0", fn_1414770b0 as *const ()), // sub_1414770b0
    Replacement::draft_throws(0x141477270, "luau_141477270", fn_141477270 as *const (), Throws::Caught), // sub_141477270
    Replacement::draft(0x1414778b0, "luau_1414778b0", fn_1414778b0 as *const ()), // sub_1414778b0
    Replacement::draft(0x141477960, "luau_141477960", fn_141477960 as *const ()), // sub_141477960
    Replacement::draft(0x141477a20, "luau_141477a20", fn_141477a20 as *const ()), // sub_141477a20
    Replacement::draft(0x141477ae0, "luau_141477ae0", fn_141477ae0 as *const ()), // sub_141477ae0
    Replacement::draft_throws(0x141477ba0, "luau_141477ba0", fn_141477ba0 as *const (), Throws::Caught), // sub_141477ba0
    Replacement::draft_throws(0x141477ea0, "luau_141477ea0", fn_141477ea0 as *const (), Throws::Discard), // sub_141477ea0
    Replacement::draft_throws(0x141477fa0, "luau_141477fa0", fn_141477fa0 as *const (), Throws::Discard), // sub_141477fa0
    Replacement::draft_throws(0x141478140, "luau_141478140", fn_141478140 as *const (), Throws::Caught), // sub_141478140
    Replacement::draft(0x1414785d0, "luau_1414785d0", fn_1414785d0 as *const ()), // sub_1414785d0
    Replacement::draft_throws(0x141478710, "luau_141478710", fn_141478710 as *const (), Throws::Caught), // sub_141478710
    Replacement::draft_throws(0x141478a80, "luau_141478a80", fn_141478a80 as *const (), Throws::Discard), // sub_141478a80
    Replacement::draft_throws(0x141478b00, "luau_141478b00", fn_141478b00 as *const (), Throws::Caught), // sub_141478b00
    Replacement::draft_throws(0x141478e30, "luau_141478e30", fn_141478e30 as *const (), Throws::Caught), // sub_141478e30
    Replacement::draft_throws(0x1414790b0, "luau_1414790b0", fn_1414790b0 as *const (), Throws::Caught), // sub_1414790b0
    Replacement::draft_throws(0x1414799a0, "luau_1414799a0", fn_1414799a0 as *const (), Throws::Caught), // sub_1414799a0
    Replacement::draft_throws(0x14147a5e0, "luau_14147a5e0", fn_14147a5e0 as *const (), Throws::Caught), // sub_14147a5e0
    Replacement::draft_throws(0x14147a960, "luau_14147a960", fn_14147a960 as *const (), Throws::Caught), // sub_14147a960
    Replacement::draft_throws(0x14147ab80, "luau_14147ab80", fn_14147ab80 as *const (), Throws::Discard), // sub_14147ab80
    Replacement::draft_throws(0x14147ad70, "luau_14147ad70", fn_14147ad70 as *const (), Throws::Discard), // sub_14147ad70
    Replacement::draft_throws(0x14147af70, "luau_14147af70", fn_14147af70 as *const (), Throws::Caught), // sub_14147af70
    Replacement::draft_throws(0x14147b480, "luau_14147b480", fn_14147b480 as *const (), Throws::Discard), // sub_14147b480
    Replacement::draft_throws(0x14147b520, "luau_14147b520", fn_14147b520 as *const (), Throws::Discard), // sub_14147b520
    Replacement::draft_throws(0x14147b660, "luau_14147b660", fn_14147b660 as *const (), Throws::Caught), // sub_14147b660
    Replacement::draft_throws(0x14147b890, "luau_14147b890", fn_14147b890 as *const (), Throws::Discard), // sub_14147b890
    Replacement::draft_throws(0x14147b9b0, "luau_14147b9b0", fn_14147b9b0 as *const (), Throws::Discard), // sub_14147b9b0
    Replacement::draft_throws(0x14147bbd0, "luau_14147bbd0", fn_14147bbd0 as *const (), Throws::Discard), // sub_14147bbd0
    Replacement::draft(0x14147be50, "luau_14147be50", fn_14147be50 as *const ()), // sub_14147be50
    Replacement::draft(0x14147bec0, "luau_14147bec0", fn_14147bec0 as *const ()), // sub_14147bec0
    Replacement::draft_throws(0x14147bf10, "luau_14147bf10", fn_14147bf10 as *const (), Throws::Caught), // sub_14147bf10
    Replacement::draft_throws(0x14147c500, "luau_14147c500", fn_14147c500 as *const (), Throws::Caught), // sub_14147c500
    Replacement::draft_throws(0x14147c7d0, "luau_14147c7d0", fn_14147c7d0 as *const (), Throws::Caught), // sub_14147c7d0
    Replacement::draft_throws(0x14147cb50, "luau_14147cb50", fn_14147cb50 as *const (), Throws::Caught), // sub_14147cb50
    Replacement::draft_throws(0x14147cca0, "luau_14147cca0", fn_14147cca0 as *const (), Throws::Caught), // sub_14147cca0
    Replacement::draft_throws(0x14147cfd0, "luau_14147cfd0", fn_14147cfd0 as *const (), Throws::Discard), // sub_14147cfd0
    Replacement::draft_throws(0x14147d290, "luau_14147d290", fn_14147d290 as *const (), Throws::Discard), // sub_14147d290
    Replacement::draft_throws(0x14147d500, "luau_14147d500", fn_14147d500 as *const (), Throws::Discard), // sub_14147d500
    Replacement::draft_throws(0x14147db40, "luau_14147db40", fn_14147db40 as *const (), Throws::Caught), // sub_14147db40
    Replacement::draft_throws(0x14147df10, "luau_14147df10", fn_14147df10 as *const (), Throws::Caught), // sub_14147df10
    Replacement::draft_throws(0x14147e480, "luau_14147e480", fn_14147e480 as *const (), Throws::Caught), // sub_14147e480
    Replacement::draft(0x14147e820, "luau_14147e820", fn_14147e820 as *const ()), // Luau::Compiler::resolveAssignConflicts::Visitor::Luau::Compiler::resolveAssignConflicts::Visitor::vfunc_49
    Replacement::draft_throws(0x14147e870, "luau_14147e870", fn_14147e870 as *const (), Throws::Caught), // sub_14147e870
    Replacement::draft_throws(0x14147f090, "luau_14147f090", fn_14147f090 as *const (), Throws::Caught), // sub_14147f090
    Replacement::draft_throws(0x14147f370, "luau_14147f370", fn_14147f370 as *const (), Throws::Caught), // sub_14147f370
    Replacement::draft_throws(0x14147f850, "luau_14147f850", fn_14147f850 as *const (), Throws::Caught), // sub_14147f850
    Replacement::draft_throws(0x14147f8c0, "luau_14147f8c0", fn_14147f8c0 as *const (), Throws::Caught), // sub_14147f8c0
    Replacement::draft_throws(0x14147faf0, "luau_14147faf0", fn_14147faf0 as *const (), Throws::Discard), // sub_14147faf0
    Replacement::draft(0x14147fbd0, "luau_14147fbd0", fn_14147fbd0 as *const ()), // sub_14147fbd0
    Replacement::draft_throws(0x14147fc60, "luau_14147fc60", fn_14147fc60 as *const (), Throws::Discard), // sub_14147fc60
    Replacement::draft_throws(0x14147fd70, "luau_14147fd70", fn_14147fd70 as *const (), Throws::Discard), // sub_14147fd70
    Replacement::draft_throws(0x14147fed0, "luau_14147fed0", fn_14147fed0 as *const (), Throws::Discard), // sub_14147fed0
    Replacement::draft_throws(0x14147ff70, "luau_14147ff70", fn_14147ff70 as *const (), Throws::Discard), // sub_14147ff70
    Replacement::draft_throws(0x141480050, "luau_141480050", fn_141480050 as *const (), Throws::Discard), // sub_141480050
    Replacement::draft_throws(0x141480220, "luau_141480220", fn_141480220 as *const (), Throws::Discard), // sub_141480220
    Replacement::draft(0x141480280, "luau_141480280", fn_141480280 as *const ()), // Luau::Compiler::FenvVisitor::Luau::Compiler::FenvVisitor::vfunc_48
    Replacement::draft_throws(0x1414802f0, "luau_1414802f0", fn_1414802f0 as *const (), Throws::Discard), // Luau::Compiler::FunctionVisitor::Luau::Compiler::FunctionVisitor::vfunc_43
    Replacement::draft_throws(0x141480350, "luau_141480350", fn_141480350 as *const (), Throws::Discard), // sub_141480350
    Replacement::draft_throws(0x1414803d0, "luau_1414803d0", fn_1414803d0 as *const (), Throws::Discard), // Luau::Compiler::UndefinedLocalVisitor::Luau::Compiler::UndefinedLocalVisitor::vfunc_49
    Replacement::draft_throws(0x1414803f0, "luau_1414803f0", fn_1414803f0 as *const (), Throws::Discard), // Luau::Compiler::UndefinedLocalVisitor::Luau::Compiler::UndefinedLocalVisitor::vfunc_43
    Replacement::draft_throws(0x141480530, "luau_141480530", fn_141480530 as *const (), Throws::Discard), // Luau::Compiler::ConstUpvalueVisitor::Luau::Compiler::ConstUpvalueVisitor::vfunc_49
    Replacement::draft(0x141480590, "luau_141480590", fn_141480590 as *const ()), // Luau::Compiler::ReturnVisitor::Luau::Compiler::ReturnVisitor::vfunc_28
    Replacement::draft(0x1414805d0, "luau_1414805d0", fn_1414805d0 as *const ()), // sub_1414805d0
    Replacement::draft(0x1414805e0, "luau_1414805e0", tpl_365::<0x141480b80> as *const ()), // sub_1414805e0
];
