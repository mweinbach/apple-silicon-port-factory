#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x14028a1d0, "ecs_14028a1d0", fn_14028a1d0 as *const ()), // ecs::core::QueryRemoved<bigfish::component::Inspect,1>::ecs::core::QueryRemoved<bigfish::component::Inspect,1>::vfunc_3
    Replacement::draft(0x14028a230, "ecs_14028a230", fn_14028a230 as *const ()), // ecs::core::QueryRemoved<bigfish::component::Inspect,1>::ecs::core::QueryRemoved<bigfish::component::Inspect,1>::vfunc_1
    Replacement::draft(0x14028a250, "ecs_14028a250", tpl_1995::<0x1430ae378, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::ScenePromptOwner,1>::ecs::core::QueryRemoved<bigfish::component::ScenePrompt
    Replacement::draft(0x140379b50, "ecs_140379b50", fn_140379b50 as *const ()), // ecs::core::QueryRemoved<bigfish::component::LaunchProjectile,0>::ecs::core::QueryRemoved<bigfish::component::LaunchProje
    Replacement::draft(0x140379c30, "ecs_140379c30", tpl_3190::<0x10, 0x14028a2a0, 0x140379d10, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::LaunchProjectile,0>::ecs::core::QueryRemoved<bigfish::component::LaunchProje
    Replacement::draft(0x140379cc0, "ecs_140379cc0", tpl_3191::<0x1430bf408, 0x140379ec0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::LaunchProjectile,0>::ecs::core::QueryRemoved<bigfish::component::LaunchProje
    Replacement::draft(0x1403a51a0, "ecs_1403a51a0", tpl_1995::<0x1430e2d90, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::ActiveAvatar,1>::ecs::core::QueryRemoved<coregame::component::ActiveAvatar,
    Replacement::draft(0x1403d7000, "ecs_1403d7000", tpl_1995::<0x1430fb1e8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::NotificationQueueUser,1>::ecs::core::QueryRemoved<bigfish::component::Notifi
    Replacement::draft(0x1403e9800, "ecs_1403e9800", fn_1403e9800 as *const ()), // ecs::core::QueryRemoved<bigfish::map::component::MapVisualization,0>::ecs::core::QueryRemoved<bigfish::map::component::M
    Replacement::draft(0x1403e98f0, "ecs_1403e98f0", fn_1403e98f0 as *const ()), // ecs::core::QueryRemoved<bigfish::map::component::MapVisualization,0>::ecs::core::QueryRemoved<bigfish::map::component::M
    Replacement::draft(0x1403e9980, "ecs_1403e9980", tpl_3191::<0x143102850, 0x1403e9bb0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::map::component::MapVisualization,0>::ecs::core::QueryRemoved<bigfish::map::component::M
    Replacement::draft(0x1404631d0, "ecs_1404631d0", fn_1404631d0 as *const ()), // ecs::core::QueryRemoved<bigfish::component::InventoryItemQueue,0>::ecs::core::QueryRemoved<bigfish::component::Inventory
    Replacement::draft(0x140463290, "ecs_140463290", tpl_4350::<0x10, 0x14028a2a0, 0x140463370, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::InventoryItemQueue,0>::ecs::core::QueryRemoved<bigfish::component::Inventory
    Replacement::draft(0x140463320, "ecs_140463320", tpl_3191::<0x143139320, 0x1404634f0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::InventoryItemQueue,0>::ecs::core::QueryRemoved<bigfish::component::Inventory
    Replacement::draft(0x140477d20, "ecs_140477d20", tpl_1995::<0x14313fe68, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::Inspect,1>::ecs::core::QueryRemoved<bigfish::component::Inspect,1>::vfunc_0
    Replacement::draft(0x140489880, "ecs_140489880", fn_140489880 as *const ()), // ecs::core::QueryRemoved<bigfish::component::HealthCondition,0>::ecs::core::QueryRemoved<bigfish::component::HealthCondit
    Replacement::draft(0x140489910, "ecs_140489910", tpl_4494::<0x10, 0x14028a2a0, 0x140489ca0, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::HealthCondition,0>::ecs::core::QueryRemoved<bigfish::component::HealthCondit
    Replacement::draft(0x140489c50, "ecs_140489c50", tpl_3191::<0x143144ca0, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::HealthCondition,0>::ecs::core::QueryRemoved<bigfish::component::HealthCondit
    Replacement::draft(0x140497740, "ecs_140497740", fn_140497740 as *const ()), // ecs::core::QueryRemoved<coregame::component::Trigger,0>::ecs::core::QueryRemoved<coregame::component::Trigger,0>::vfunc_
    Replacement::draft(0x1404977d0, "ecs_1404977d0", tpl_4536::<0x10, 0x14028a2a0, 0x1404978b0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::SDFVolume,0>::ecs::core::QueryRemoved<coregame::component::SDFVolume,0>::vf
    Replacement::draft(0x140497860, "ecs_140497860", tpl_3191::<0x14314f7c8, 0x140497a00, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::GameSpecificNavigation,0>::ecs::core::QueryRemoved<bigfish::component::GameS
    Replacement::draft(0x1404c34b0, "ecs_1404c34b0", tpl_4818::<0x8, 0x14183acc8, 0x1401ff5b0> as *const ()), // ecs::core::QueryRemoved<bigfish::component::GameArmoury,0>::ecs::core::QueryRemoved<bigfish::component::GameArmoury,0>::
    Replacement::draft(0x1404c3550, "ecs_1404c3550", tpl_4494::<0x10, 0x14028a2a0, 0x1404c3680, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::GameArmoury,0>::ecs::core::QueryRemoved<bigfish::component::GameArmoury,0>::
    Replacement::draft(0x1404c35e0, "ecs_1404c35e0", fn_1404c35e0 as *const ()), // sub_1404c35e0
    Replacement::draft(0x1404c3630, "ecs_1404c3630", tpl_3191::<0x14315f0e8, 0x1404c37e0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::GameArmoury,0>::ecs::core::QueryRemoved<bigfish::component::GameArmoury,0>::
    Replacement::draft(0x1404e8130, "ecs_1404e8130", fn_1404e8130 as *const ()), // ecs::core::QueryRemoved<bigfish::component::LightSourceOwnerState,0>::ecs::core::QueryRemoved<bigfish::component::LightS
    Replacement::draft(0x1404e81e0, "ecs_1404e81e0", tpl_4936::<0x10, 0x14028a2a0, 0x1404e8310, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::LightSourceOwnerState,0>::ecs::core::QueryRemoved<bigfish::component::LightS
    Replacement::draft(0x1404e8270, "ecs_1404e8270", tpl_1995::<0x1431740b0, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::LightSourceLightSpawned,1>::ecs::core::QueryRemoved<bigfish::component::Ligh
    Replacement::draft(0x1404e82c0, "ecs_1404e82c0", tpl_3191::<0x1431743d8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::LightSourceOwnerState,0>::ecs::core::QueryRemoved<bigfish::component::LightS
    Replacement::draft(0x140514f40, "ecs_140514f40", fn_140514f40 as *const ()), // ecs::core::QueryRemoved<bigfish::component::Grapple,0>::ecs::core::QueryRemoved<bigfish::component::Grapple,0>::vfunc_2
    Replacement::draft(0x140514ff0, "ecs_140514ff0", tpl_3190::<0x10, 0x14028a2a0, 0x1405150d0, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::Grapple,0>::ecs::core::QueryRemoved<bigfish::component::Grapple,0>::vfunc_1
    Replacement::draft(0x140515080, "ecs_140515080", tpl_3191::<0x14318c008, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::door::component::Navigation,0>::ecs::core::QueryRemoved<bigfish::door::component::Navig
    Replacement::draft(0x14053ab70, "ecs_14053ab70", fn_14053ab70 as *const ()), // ecs::core::QueryRemoved<bigfish::component::DarknessShieldEntities,0>::ecs::core::QueryRemoved<bigfish::component::Darkn
    Replacement::draft(0x14053ac30, "ecs_14053ac30", tpl_4350::<0x10, 0x14028a2a0, 0x14053ae40, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::DarknessShieldEntities,0>::ecs::core::QueryRemoved<bigfish::component::Darkn
    Replacement::draft(0x14053adf0, "ecs_14053adf0", tpl_3191::<0x143199740, 0x14053afb0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::DarknessShieldEntities,0>::ecs::core::QueryRemoved<bigfish::component::Darkn
    Replacement::draft(0x140558c30, "ecs_140558c30", fn_140558c30 as *const ()), // ecs::core::QueryRemoved<bigfish::component::CrowdArea,0>::ecs::core::QueryRemoved<bigfish::component::CrowdArea,0>::vfun
    Replacement::draft(0x140558ce0, "ecs_140558ce0", tpl_4494::<0x10, 0x14028a2a0, 0x140559220, 0x8> as *const ()), // ecs::core::QueryRemoved<bigfish::component::CrowdArea,0>::ecs::core::QueryRemoved<bigfish::component::CrowdArea,0>::vfun
    Replacement::draft(0x140558d70, "ecs_140558d70", fn_140558d70 as *const ()), // sub_140558d70
    Replacement::draft_throws(0x140558dc0, "ecs_140558dc0", fn_140558dc0 as *const (), Throws::Caught), // sub_140558dc0
    Replacement::draft_throws(0x140558e40, "ecs_140558e40", fn_140558e40 as *const (), Throws::Caught), // sub_140558e40
    Replacement::draft_throws(0x140559020, "ecs_140559020", fn_140559020 as *const (), Throws::Caught), // sub_140559020
    Replacement::draft_throws(0x1405590f0, "ecs_1405590f0", fn_1405590f0 as *const (), Throws::Caught), // sub_1405590f0
    Replacement::draft(0x1405591d0, "ecs_1405591d0", tpl_3191::<0x1431ab238, 0x1405593a0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::CrowdArea,0>::ecs::core::QueryRemoved<bigfish::component::CrowdArea,0>::vfun
    Replacement::draft(0x14056d130, "ecs_14056d130", tpl_3191::<0x1431b2978, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::EncounterComponent,0>::ecs::core::QueryRemoved<bigfish::component::Encounter
    Replacement::draft(0x1405875b0, "ecs_1405875b0", tpl_3191::<0x1431bf538, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<component::CharacterLightRigComponent,0>::ecs::core::QueryRemoved<component::CharacterLightRigCo
    Replacement::draft(0x1405ccc80, "ecs_1405ccc80", fn_1405ccc80 as *const ()), // ecs::core::QueryRemoved<bigfish::cameramixer::CameraMixerState,0>::ecs::core::QueryRemoved<bigfish::cameramixer::CameraM
    Replacement::draft(0x1405cced0, "ecs_1405cced0", fn_1405cced0 as *const ()), // ecs::core::QueryRemoved<bigfish::cameramixer::CameraMixerState,0>::ecs::core::QueryRemoved<bigfish::cameramixer::CameraM
    Replacement::draft(0x1405ccf60, "ecs_1405ccf60", tpl_3191::<0x1431d8a08, 0x1405cd120, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::cameramixer::CameraMixerState,0>::ecs::core::QueryRemoved<bigfish::cameramixer::CameraM
    Replacement::draft(0x1405e7950, "ecs_1405e7950", fn_1405e7950 as *const ()), // ecs::core::QueryRemoved<content::AvatarSwapperComponent,0>::ecs::core::QueryRemoved<content::AvatarSwapperComponent,0>::
    Replacement::draft(0x1405e7a90, "ecs_1405e7a90", fn_1405e7a90 as *const ()), // ecs::core::QueryRemoved<content::AvatarSwapperComponent,0>::ecs::core::QueryRemoved<content::AvatarSwapperComponent,0>::
    Replacement::draft(0x1405e7b20, "ecs_1405e7b20", tpl_3191::<0x1431ec4e8, 0x1405e7ce0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<content::AvatarSwapperComponent,0>::ecs::core::QueryRemoved<content::AvatarSwapperComponent,0>::
    Replacement::draft(0x1405f2f10, "ecs_1405f2f10", fn_1405f2f10 as *const ()), // ecs::core::QueryRemoved<bonsai::component::AITicketNPCData,0>::ecs::core::QueryRemoved<bonsai::component::AITicketNPCDat
    Replacement::draft(0x1405f3070, "ecs_1405f3070", tpl_5946::<0x10, 0x14028a2a0, 0x1405f3150, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::AITicketNPCData,0>::ecs::core::QueryRemoved<bonsai::component::AITicketNPCDat
    Replacement::draft(0x1405f3100, "ecs_1405f3100", tpl_3191::<0x1431f0300, 0x1405f32c0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::AITicketNPCData,0>::ecs::core::QueryRemoved<bonsai::component::AITicketNPCDat
    Replacement::draft(0x1405fa2a0, "ecs_1405fa2a0", fn_1405fa2a0 as *const ()), // ecs::core::QueryRemoved<content::AIInteractableObjectComponent,0>::ecs::core::QueryRemoved<content::AIInteractableObject
    Replacement::draft(0x1405fa330, "ecs_1405fa330", tpl_4494::<0x10, 0x14028a2a0, 0x1405fa410, 0x8> as *const ()), // ecs::core::QueryRemoved<content::AIInteractableObjectComponent,0>::ecs::core::QueryRemoved<content::AIInteractableObject
    Replacement::draft(0x1405fa3c0, "ecs_1405fa3c0", tpl_3191::<0x1431f4040, 0x1405fa560, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<content::AIInteractableObjectComponent,0>::ecs::core::QueryRemoved<content::AIInteractableObject
    Replacement::draft(0x14061a2a0, "ecs_14061a2a0", tpl_3191::<0x143201f40, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bigfish::component::Grapple,0>::ecs::core::QueryRemoved<bigfish::component::Grapple,0>::vfunc_0
    Replacement::draft(0x140635370, "ecs_140635370", fn_140635370 as *const ()), // ecs::core::QueryRemoved<bonsai::component::QuadrupedBodyObstacle,0>::ecs::core::QueryRemoved<bonsai::component::Quadrupe
    Replacement::draft(0x140635430, "ecs_140635430", tpl_3190::<0x10, 0x14028a2a0, 0x140635510, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::QuadrupedBodyObstacle,0>::ecs::core::QueryRemoved<bonsai::component::Quadrupe
    Replacement::draft(0x1406354c0, "ecs_1406354c0", tpl_3191::<0x143214988, 0x140635690, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::QuadrupedBodyObstacle,0>::ecs::core::QueryRemoved<bonsai::component::Quadrupe
    Replacement::draft(0x1406396a0, "ecs_1406396a0", fn_1406396a0 as *const ()), // ecs::core::QueryRemoved<bonsai::component::PlayerObstacle,0>::ecs::core::QueryRemoved<bonsai::component::PlayerObstacle,
    Replacement::draft(0x140639740, "ecs_140639740", tpl_4494::<0x10, 0x14028a2a0, 0x140639820, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::PlayerObstacle,0>::ecs::core::QueryRemoved<bonsai::component::PlayerObstacle,
    Replacement::draft(0x1406397d0, "ecs_1406397d0", tpl_3191::<0x143216b38, 0x140639980, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::PlayerObstacle,0>::ecs::core::QueryRemoved<bonsai::component::PlayerObstacle,
    Replacement::draft(0x1406423a0, "ecs_1406423a0", fn_1406423a0 as *const ()), // ecs::core::QueryRemoved<navigation::component::Navmesh,0>::ecs::core::QueryRemoved<navigation::component::Navmesh,0>::vf
    Replacement::draft(0x1406424e0, "ecs_1406424e0", tpl_6208::<0x10, 0x14028a2a0, 0x140642920, 0x8> as *const ()), // ecs::core::QueryRemoved<navigation::component::Navmesh,0>::ecs::core::QueryRemoved<navigation::component::Navmesh,0>::vf
    Replacement::draft(0x1406428d0, "ecs_1406428d0", tpl_3191::<0x14321ad98, 0x140642a80, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<navigation::component::Navmesh,0>::ecs::core::QueryRemoved<navigation::component::Navmesh,0>::vf
    Replacement::draft(0x1406439f0, "ecs_1406439f0", fn_1406439f0 as *const ()), // ecs::core::QueryRemoved<navigation::component::AiCover,0>::ecs::core::QueryRemoved<navigation::component::AiCover,0>::vf
    Replacement::draft(0x140643a90, "ecs_140643a90", tpl_4494::<0x10, 0x14028a2a0, 0x140643b70, 0x8> as *const ()), // ecs::core::QueryRemoved<navigation::component::AiCover,0>::ecs::core::QueryRemoved<navigation::component::AiCover,0>::vf
    Replacement::draft(0x140643b20, "ecs_140643b20", tpl_3191::<0x14321adc0, 0x140643ce0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<navigation::component::AiCover,0>::ecs::core::QueryRemoved<navigation::component::AiCover,0>::vf
    Replacement::draft(0x140647610, "ecs_140647610", fn_140647610 as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavMeshModifierZone,0>::ecs::core::QueryRemoved<bonsai::component::NavMeshMod
    Replacement::draft(0x1406477c0, "ecs_1406477c0", tpl_6221::<0x10, 0x14028a2a0, 0x1406478a0, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavMeshModifierZone,0>::ecs::core::QueryRemoved<bonsai::component::NavMeshMod
    Replacement::draft(0x140647850, "ecs_140647850", tpl_3191::<0x14321c050, 0x140647a10, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavMeshModifierZone,0>::ecs::core::QueryRemoved<bonsai::component::NavMeshMod
    Replacement::draft(0x140650340, "ecs_140650340", fn_140650340 as *const ()), // ecs::core::QueryRemoved<bonsai::component::PhysicsNavigationObstacleData,0>::ecs::core::QueryRemoved<bonsai::component::
    Replacement::draft(0x140650400, "ecs_140650400", tpl_4350::<0x10, 0x14028a2a0, 0x140650670, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::PhysicsNavigationObstacleData,0>::ecs::core::QueryRemoved<bonsai::component::
    Replacement::draft(0x140650490, "ecs_140650490", tpl_6239::<0x8, 0x14183acc8, 0x20, 0x40> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavigationObstacle,0>::ecs::core::QueryRemoved<bonsai::component::NavigationO
    Replacement::draft(0x140650540, "ecs_140650540", tpl_6240::<0x10, 0x14028a2a0, 0x140650920, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavigationObstacle,0>::ecs::core::QueryRemoved<bonsai::component::NavigationO
    Replacement::draft(0x1406505d0, "ecs_1406505d0", tpl_3191::<0x143220228, 0x140650800, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::PhysicsNavigationObstacleData,0>::ecs::core::QueryRemoved<bonsai::component::
    Replacement::draft(0x140650620, "ecs_140650620", tpl_3191::<0x143220250, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavigationObstacle,0>::ecs::core::QueryRemoved<bonsai::component::NavigationO
    Replacement::draft(0x140654650, "ecs_140654650", fn_140654650 as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavigationLink,0>::ecs::core::QueryRemoved<bonsai::component::NavigationLink,
    Replacement::draft(0x140654810, "ecs_140654810", fn_140654810 as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavigationLink,0>::ecs::core::QueryRemoved<bonsai::component::NavigationLink,
    Replacement::draft(0x140654a00, "ecs_140654a00", tpl_3191::<0x143222258, 0x140654ba0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavigationLink,0>::ecs::core::QueryRemoved<bonsai::component::NavigationLink,
    Replacement::draft(0x140678ee0, "ecs_140678ee0", fn_140678ee0 as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavMeshPathPoints,0>::ecs::core::QueryRemoved<bonsai::component::NavMeshPathP
    Replacement::draft(0x140678ff0, "ecs_140678ff0", tpl_6240::<0x10, 0x14028a2a0, 0x1406790d0, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavMeshPathPoints,0>::ecs::core::QueryRemoved<bonsai::component::NavMeshPathP
    Replacement::draft(0x140679080, "ecs_140679080", tpl_3191::<0x143231020, 0x140679290, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::NavMeshPathPoints,0>::ecs::core::QueryRemoved<bonsai::component::NavMeshPathP
    Replacement::draft(0x1406847b0, "ecs_1406847b0", fn_1406847b0 as *const ()), // ecs::core::QueryRemoved<bonsai::component::BodyController,0>::ecs::core::QueryRemoved<bonsai::component::BodyController,
    Replacement::draft(0x140684950, "ecs_140684950", fn_140684950 as *const ()), // ecs::core::QueryRemoved<bonsai::component::BodyController,0>::ecs::core::QueryRemoved<bonsai::component::BodyController,
    Replacement::draft(0x1406849e0, "ecs_1406849e0", tpl_3796::<0x143c8ff58, 0x142489ee0, 0x143cc25cc, 0x143cc239c> as *const ()), // sub_1406849e0
    Replacement::draft(0x140684d50, "ecs_140684d50", tpl_3191::<0x143231940, 0x140684ef0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::BodyController,0>::ecs::core::QueryRemoved<bonsai::component::BodyController,
    Replacement::draft(0x14068bdc0, "ecs_14068bdc0", fn_14068bdc0 as *const ()), // ecs::core::QueryRemoved<bonsai::component::BehaviorTree,0>::ecs::core::QueryRemoved<bonsai::component::BehaviorTree,0>::
    Replacement::draft(0x14068bf40, "ecs_14068bf40", tpl_6491::<0x10, 0x14028a2a0, 0x14068c2a0, 0x8> as *const ()), // ecs::core::QueryRemoved<bonsai::component::BehaviorTree,0>::ecs::core::QueryRemoved<bonsai::component::BehaviorTree,0>::
    Replacement::draft_throws(0x14068c100, "ecs_14068c100", tpl_3757::<0x143c89eb8, 0x143c89ec4, 0x10, 0x1420a25b0, 0x141834a78, 0x143ca78d0, 0x143c8d164, 0x143c89ec0, 0x143c89ed0, 0x141834a0c> as *const (), Throws::Discard), // sub_14068c100
    Replacement::draft(0x14068c200, "ecs_14068c200", tpl_3191::<0x143239ab8, 0x14068c410, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::BehaviorTree,0>::ecs::core::QueryRemoved<bonsai::component::BehaviorTree,0>::
    Replacement::draft(0x14068c250, "ecs_14068c250", tpl_3191::<0x143239a80, 0x14068c530, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<bonsai::component::BehaviorTreeResource,0>::ecs::core::QueryRemoved<bonsai::component::BehaviorT
    Replacement::draft(0x140f31ca0, "ecs_140f31ca0", tpl_1995::<0x14330b800, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::WindEntity,1>::ecs::core::QueryRemoved<coregame::component::WindEntity,1>::
    Replacement::draft(0x140f5b8a0, "ecs_140f5b8a0", tpl_3191::<0x143313278, 0x140497a00, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::ui_page::component::Page,0>::ecs::core::QueryRemoved<coregame::ui_page::component::Pag
    Replacement::draft(0x140f7a250, "ecs_140f7a250", tpl_4536::<0x10, 0x14028a2a0, 0x140f7a550, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::Trigger,0>::ecs::core::QueryRemoved<coregame::component::Trigger,0>::vfunc_
    Replacement::draft(0x140f7a500, "ecs_140f7a500", tpl_3191::<0x14331c578, 0x140f7a690, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::Trigger,0>::ecs::core::QueryRemoved<coregame::component::Trigger,0>::vfunc_
    Replacement::draft(0x140f82570, "ecs_140f82570", fn_140f82570 as *const ()), // ecs::core::QueryRemoved<coregame::component::GlobalID,0>::ecs::core::QueryRemoved<coregame::component::GlobalID,0>::vfun
    Replacement::draft(0x140f82600, "ecs_140f82600", tpl_4536::<0x10, 0x14028a2a0, 0x140f82850, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::GlobalID,0>::ecs::core::QueryRemoved<coregame::component::GlobalID,0>::vfun
    Replacement::draft(0x140f82690, "ecs_140f82690", tpl_3191::<0x14331f870, 0x140f82730, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::ResolvedChildren,0>::ecs::core::QueryRemoved<coregame::component::ResolvedC
    Replacement::draft(0x140f826e0, "ecs_140f826e0", tpl_3191::<0x14331f898, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::ResolvedParent,0>::ecs::core::QueryRemoved<coregame::component::ResolvedPar
    Replacement::draft(0x140f93f40, "ecs_140f93f40", fn_140f93f40 as *const ()), // ecs::core::QueryRemoved<coregame::component::Timeline,0>::ecs::core::QueryRemoved<coregame::component::Timeline,0>::vfun
    Replacement::draft(0x140f94070, "ecs_140f94070", fn_140f94070 as *const ()), // ecs::core::QueryRemoved<coregame::component::Timeline,0>::ecs::core::QueryRemoved<coregame::component::Timeline,0>::vfun
    Replacement::draft(0x140f94100, "ecs_140f94100", tpl_3191::<0x143322720, 0x140f942c0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::Timeline,0>::ecs::core::QueryRemoved<coregame::component::Timeline,0>::vfun
    Replacement::draft(0x140fa2700, "ecs_140fa2700", fn_140fa2700 as *const ()), // ecs::core::QueryRemoved<coregame::component::EntityTags,0>::ecs::core::QueryRemoved<coregame::component::EntityTags,0>::
    Replacement::draft(0x140fa2830, "ecs_140fa2830", tpl_3190::<0x10, 0x14028a2a0, 0x140fa2910, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::EntityTags,0>::ecs::core::QueryRemoved<coregame::component::EntityTags,0>::
    Replacement::draft(0x140fa28c0, "ecs_140fa28c0", tpl_3191::<0x14332b8f8, 0x140fa2a70, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::EntityTags,0>::ecs::core::QueryRemoved<coregame::component::EntityTags,0>::
    Replacement::draft(0x140fb4bf0, "ecs_140fb4bf0", fn_140fb4bf0 as *const ()), // ecs::core::QueryRemoved<coregame::component::BroadphaseVolume,0>::ecs::core::QueryRemoved<coregame::component::Broadphas
    Replacement::draft(0x140fb4cf0, "ecs_140fb4cf0", tpl_3190::<0x10, 0x14028a2a0, 0x140fb4dd0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::BroadphaseVolume,0>::ecs::core::QueryRemoved<coregame::component::Broadphas
    Replacement::draft(0x140fb4d80, "ecs_140fb4d80", tpl_3191::<0x143333000, 0x140fb4f90, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::BroadphaseVolume,0>::ecs::core::QueryRemoved<coregame::component::Broadphas
    Replacement::draft(0x140fc1230, "ecs_140fc1230", fn_140fc1230 as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::SpawnedEntity,0>::ecs::core::QueryRemoved<coregame::spawn
    Replacement::draft(0x140fc1320, "ecs_140fc1320", tpl_4936::<0x10, 0x14028a2a0, 0x140fc1400, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::SpawnedEntity,0>::ecs::core::QueryRemoved<coregame::spawn
    Replacement::draft(0x140fc13b0, "ecs_140fc13b0", tpl_3191::<0x1433352b8, 0x140fc15c0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::SpawnedEntity,0>::ecs::core::QueryRemoved<coregame::spawn
    Replacement::draft(0x140fc3670, "ecs_140fc3670", tpl_3191::<0x1433358f8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::SpatiallyActivatedSoundLoop,0>::ecs::core::QueryRemoved<coregame::component
    Replacement::draft(0x140fc4e90, "ecs_140fc4e90", tpl_1995::<0x143336320, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::module::spatial::InSpatialStructure,1>::ecs::core::QueryRemoved<coregame::module::spat
    Replacement::draft(0x140fca5a0, "ecs_140fca5a0", fn_140fca5a0 as *const ()), // ecs::core::QueryRemoved<coregame::component::SoundSpawner,0>::ecs::core::QueryRemoved<coregame::component::SoundSpawner,
    Replacement::draft(0x140fca680, "ecs_140fca680", tpl_3190::<0x10, 0x14028a2a0, 0x140fca760, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::SoundSpawner,0>::ecs::core::QueryRemoved<coregame::component::SoundSpawner,
    Replacement::draft(0x140fca710, "ecs_140fca710", tpl_3191::<0x143338688, 0x140fca8f0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::SoundSpawner,0>::ecs::core::QueryRemoved<coregame::component::SoundSpawner,
    Replacement::draft(0x140fd02a0, "ecs_140fd02a0", fn_140fd02a0 as *const ()), // ecs::core::QueryRemoved<coregame::component::SoundEmitter,0>::ecs::core::QueryRemoved<coregame::component::SoundEmitter,
    Replacement::draft(0x140fd0360, "ecs_140fd0360", tpl_4350::<0x10, 0x14028a2a0, 0x140fd0440, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::SoundEmitter,0>::ecs::core::QueryRemoved<coregame::component::SoundEmitter,
    Replacement::draft(0x140fd03f0, "ecs_140fd03f0", tpl_3191::<0x14333a710, 0x140650800, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::SoundEmitter,0>::ecs::core::QueryRemoved<coregame::component::SoundEmitter,
    Replacement::draft(0x140fd7810, "ecs_140fd7810", tpl_3191::<0x14333ccb8, 0x140fd7860, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::SDFVolume,0>::ecs::core::QueryRemoved<coregame::component::SDFVolume,0>::vf
    Replacement::draft(0x140fe0000, "ecs_140fe0000", fn_140fe0000 as *const ()), // ecs::core::QueryRemoved<coregame::component::ChildrenRegistry,0>::ecs::core::QueryRemoved<coregame::component::ChildrenR
    Replacement::draft(0x140fe00c0, "ecs_140fe00c0", tpl_19724::<0x10, 0x14028a2a0, 0x140fe02f0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::ChildrenRegistry,0>::ecs::core::QueryRemoved<coregame::component::ChildrenR
    Replacement::draft(0x140fe02a0, "ecs_140fe02a0", tpl_3191::<0x14333f878, 0x140fe0480, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::ChildrenRegistry,0>::ecs::core::QueryRemoved<coregame::component::ChildrenR
    Replacement::draft(0x140ffe200, "ecs_140ffe200", fn_140ffe200 as *const ()), // ecs::core::QueryRemoved<coregame::component::CollisionTemplate,0>::ecs::core::QueryRemoved<coregame::component::Collisio
    Replacement::draft(0x140ffe350, "ecs_140ffe350", tpl_6240::<0x10, 0x14028a2a0, 0x140ffe5b0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollisionTemplate,0>::ecs::core::QueryRemoved<coregame::component::Collisio
    Replacement::draft(0x140ffe3e0, "ecs_140ffe3e0", fn_140ffe3e0 as *const ()), // ecs::core::QueryRemoved<coregame::component::CollisionResource,0>::ecs::core::QueryRemoved<coregame::component::Collisio
    Replacement::draft(0x140ffe480, "ecs_140ffe480", tpl_4494::<0x10, 0x14028a2a0, 0x140ffe840, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollisionResource,0>::ecs::core::QueryRemoved<coregame::component::Collisio
    Replacement::draft(0x140ffe510, "ecs_140ffe510", tpl_3191::<0x143347798, 0x140ffe720, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollisionTemplate,0>::ecs::core::QueryRemoved<coregame::component::Collisio
    Replacement::draft(0x140ffe560, "ecs_140ffe560", tpl_3191::<0x143347770, 0x140ffe9b0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollisionResource,0>::ecs::core::QueryRemoved<coregame::component::Collisio
    Replacement::draft(0x1410144b0, "ecs_1410144b0", fn_1410144b0 as *const ()), // ecs::core::QueryRemoved<coregame::component::Physics,0>::ecs::core::QueryRemoved<coregame::component::Physics,0>::vfunc_
    Replacement::draft(0x141014580, "ecs_141014580", tpl_19724::<0x10, 0x14028a2a0, 0x141014a50, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::Physics,0>::ecs::core::QueryRemoved<coregame::component::Physics,0>::vfunc_
    Replacement::draft(0x1410149b0, "ecs_1410149b0", tpl_3191::<0x143354a28, 0x141014be0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::Physics,0>::ecs::core::QueryRemoved<coregame::component::Physics,0>::vfunc_
    Replacement::draft(0x141014a00, "ecs_141014a00", tpl_3191::<0x143354a00, 0x141014d20, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::PhysicsPackageState,0>::ecs::core::QueryRemoved<coregame::component::Physic
    Replacement::draft(0x141016a60, "ecs_141016a60", tpl_4818::<0x8, 0x14183acc8, 0x1410125f0> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollapsedLodPhysics,0>::ecs::core::QueryRemoved<coregame::component::Collap
    Replacement::draft(0x141016b00, "ecs_141016b00", tpl_4494::<0x10, 0x14028a2a0, 0x141016be0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollapsedLodPhysics,0>::ecs::core::QueryRemoved<coregame::component::Collap
    Replacement::draft(0x141016b90, "ecs_141016b90", tpl_3191::<0x1433549a8, 0x141016d40, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CollapsedLodPhysics,0>::ecs::core::QueryRemoved<coregame::component::Collap
    Replacement::draft(0x14102ac90, "ecs_14102ac90", fn_14102ac90 as *const ()), // ecs::core::QueryRemoved<coregame::component::ParticleSystemInstance,0>::ecs::core::QueryRemoved<coregame::component::Par
    Replacement::draft(0x14102ad40, "ecs_14102ad40", tpl_4350::<0x10, 0x14028a2a0, 0x14102ae20, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::ParticleSystemInstance,0>::ecs::core::QueryRemoved<coregame::component::Par
    Replacement::draft(0x14102add0, "ecs_14102add0", tpl_3191::<0x143356a48, 0x14102b070, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::ParticleSystemInstance,0>::ecs::core::QueryRemoved<coregame::component::Par
    Replacement::draft(0x141047b10, "ecs_141047b10", fn_141047b10 as *const ()), // ecs::core::QueryRemoved<coregame::component::RenderObject,0>::ecs::core::QueryRemoved<coregame::component::RenderObject,
    Replacement::draft(0x141047bc0, "ecs_141047bc0", tpl_4350::<0x10, 0x14028a2a0, 0x141048cc0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::RenderObject,0>::ecs::core::QueryRemoved<coregame::component::RenderObject,
    Replacement::draft_throws(0x141047eb0, "ecs_141047eb0", tpl_1330::<0x8, 0x14104beb0, 0x143cd4460, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_141047eb0
    Replacement::draft(0x141048c70, "ecs_141048c70", tpl_3191::<0x143365010, 0x141048e30, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::RenderObject,0>::ecs::core::QueryRemoved<coregame::component::RenderObject,
    Replacement::draft(0x141056560, "ecs_141056560", tpl_3191::<0x1433667f8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::LuaSystemsGlueRegistered,0>::ecs::core::QueryRemoved<coregame::component::L
    Replacement::draft(0x141060840, "ecs_141060840", fn_141060840 as *const ()), // ecs::core::QueryRemoved<coregame::component::LuaScript,0>::ecs::core::QueryRemoved<coregame::component::LuaScript,0>::vf
    Replacement::draft(0x141060900, "ecs_141060900", tpl_19724::<0x10, 0x14028a2a0, 0x1410609e0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::LuaScript,0>::ecs::core::QueryRemoved<coregame::component::LuaScript,0>::vf
    Replacement::draft(0x141060990, "ecs_141060990", tpl_3191::<0x14336a568, 0x141060b60, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::LuaScript,0>::ecs::core::QueryRemoved<coregame::component::LuaScript,0>::vf
    Replacement::draft(0x1410632d0, "ecs_1410632d0", tpl_3191::<0x14336a5e0, 0x140f82730, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::LuaDialogueResourceIDs,0>::ecs::core::QueryRemoved<coregame::component::Lua
    Replacement::draft(0x14107e230, "ecs_14107e230", fn_14107e230 as *const ()), // ecs::core::QueryRemoved<coregame::component::LogicAnimationState,0>::ecs::core::QueryRemoved<coregame::component::LogicA
    Replacement::draft(0x14107e3d0, "ecs_14107e3d0", fn_14107e3d0 as *const ()), // ecs::core::QueryRemoved<coregame::component::LogicAnimationState,0>::ecs::core::QueryRemoved<coregame::component::LogicA
    Replacement::draft(0x14107e560, "ecs_14107e560", tpl_3191::<0x143377828, 0x14107e720, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::LogicAnimationState,0>::ecs::core::QueryRemoved<coregame::component::LogicA
    Replacement::draft(0x141096070, "ecs_141096070", fn_141096070 as *const ()), // ecs::core::QueryRemoved<coregame::component::PointLightRenderObject,0>::ecs::core::QueryRemoved<coregame::component::Poi
    Replacement::draft(0x141096110, "ecs_141096110", tpl_4350::<0x10, 0x14028a2a0, 0x141096cb0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::PointLightRenderObject,0>::ecs::core::QueryRemoved<coregame::component::Poi
    Replacement::draft(0x1410961a0, "ecs_1410961a0", fn_1410961a0 as *const ()), // ecs::core::QueryRemoved<coregame::component::SpotLightRenderObject,0>::ecs::core::QueryRemoved<coregame::component::Spot
    Replacement::draft(0x141096240, "ecs_141096240", tpl_6208::<0x10, 0x14028a2a0, 0x141096de0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::SpotLightRenderObject,0>::ecs::core::QueryRemoved<coregame::component::Spot
    Replacement::draft(0x1410962d0, "ecs_1410962d0", fn_1410962d0 as *const ()), // ecs::core::QueryRemoved<coregame::component::SpotLightResources,0>::ecs::core::QueryRemoved<coregame::component::SpotLig
    Replacement::draft(0x141096390, "ecs_141096390", tpl_19724::<0x10, 0x14028a2a0, 0x141096f10, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::SpotLightResources,0>::ecs::core::QueryRemoved<coregame::component::SpotLig
    Replacement::draft(0x141096420, "ecs_141096420", fn_141096420 as *const ()), // ecs::core::QueryRemoved<coregame::component::DirectionalLightRenderObject,0>::ecs::core::QueryRemoved<coregame::componen
    Replacement::draft(0x1410964b0, "ecs_1410964b0", tpl_19724::<0x10, 0x14028a2a0, 0x1410971c0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::DirectionalLightRenderObject,0>::ecs::core::QueryRemoved<coregame::componen
    Replacement::draft(0x141096b70, "ecs_141096b70", tpl_3191::<0x143380a40, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::PointLightRenderObject,0>::ecs::core::QueryRemoved<coregame::component::Poi
    Replacement::draft(0x141096bc0, "ecs_141096bc0", tpl_3191::<0x143380968, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::SpotLightRenderObject,0>::ecs::core::QueryRemoved<coregame::component::Spot
    Replacement::draft_throws(0x141096c10, "ecs_141096c10", fn_141096c10 as *const (), Throws::Discard), // ecs::core::QueryRemoved<coregame::component::SpotLightResources,0>::ecs::core::QueryRemoved<coregame::component::SpotLig
    Replacement::draft(0x141096c60, "ecs_141096c60", tpl_3191::<0x1433809c8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::DirectionalLightRenderObject,0>::ecs::core::QueryRemoved<coregame::componen
    Replacement::draft(0x1410a59c0, "ecs_1410a59c0", fn_1410a59c0 as *const ()), // ecs::core::QueryRemoved<coregame::component::IlluminationVolumeLight,0>::ecs::core::QueryRemoved<coregame::component::Il
    Replacement::draft(0x1410a5ab0, "ecs_1410a5ab0", tpl_6240::<0x10, 0x14028a2a0, 0x1410a5ce0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::IlluminationVolumeLight,0>::ecs::core::QueryRemoved<coregame::component::Il
    Replacement::draft(0x1410a5c90, "ecs_1410a5c90", tpl_3191::<0x143384930, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::IlluminationVolumeLight,0>::ecs::core::QueryRemoved<coregame::component::Il
    Replacement::draft(0x1410b2730, "ecs_1410b2730", fn_1410b2730 as *const ()), // ecs::core::QueryRemoved<coregame::component::StreamInAndSwitch,0>::ecs::core::QueryRemoved<coregame::component::StreamIn
    Replacement::draft(0x1410b2860, "ecs_1410b2860", fn_1410b2860 as *const ()), // ecs::core::QueryRemoved<coregame::component::StreamInAndSwitch,0>::ecs::core::QueryRemoved<coregame::component::StreamIn
    Replacement::draft(0x1410b28f0, "ecs_1410b28f0", fn_1410b28f0 as *const ()), // ecs::core::QueryRemoved<coregame::component::HierarchyLOD,0>::ecs::core::QueryRemoved<coregame::component::HierarchyLOD,
    Replacement::draft(0x1410b2a00, "ecs_1410b2a00", tpl_5946::<0x10, 0x14028a2a0, 0x1410b2d90, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::HierarchyLOD,0>::ecs::core::QueryRemoved<coregame::component::HierarchyLOD,
    Replacement::draft(0x1410b2a90, "ecs_1410b2a90", tpl_3191::<0x1433899d8, 0x1410b2c90, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::StreamInAndSwitch,0>::ecs::core::QueryRemoved<coregame::component::StreamIn
    Replacement::draft(0x1410b2ae0, "ecs_1410b2ae0", tpl_3191::<0x143389950, 0x1410b2ef0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::HierarchyLOD,0>::ecs::core::QueryRemoved<coregame::component::HierarchyLOD,
    Replacement::draft(0x1410ba2c0, "ecs_1410ba2c0", fn_1410ba2c0 as *const ()), // ecs::core::QueryRemoved<coregame::component::HeightfieldLayer,0>::ecs::core::QueryRemoved<coregame::component::Heightfie
    Replacement::draft(0x1410ba370, "ecs_1410ba370", tpl_4350::<0x10, 0x14028a2a0, 0x1410ba4a0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::HeightfieldLayer,0>::ecs::core::QueryRemoved<coregame::component::Heightfie
    Replacement::draft(0x1410ba400, "ecs_1410ba400", tpl_3191::<0x14338c5d8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::HeightfieldStack,0>::ecs::core::QueryRemoved<coregame::component::Heightfie
    Replacement::draft(0x1410ba450, "ecs_1410ba450", tpl_3191::<0x14338c570, 0x1410ba610, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::HeightfieldLayer,0>::ecs::core::QueryRemoved<coregame::component::Heightfie
    Replacement::draft(0x1410bd970, "ecs_1410bd970", tpl_3191::<0x14338cdd0, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::GlobalID,0>::ecs::core::QueryRemoved<coregame::component::GlobalID,0>::vfun
    Replacement::draft(0x1410c9220, "ecs_1410c9220", fn_1410c9220 as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphInstance,0>::ecs::core::QueryRemoved<coregame::component::GfxGraphI
    Replacement::draft(0x1410c92c0, "ecs_1410c92c0", tpl_4494::<0x10, 0x14028a2a0, 0x1410c97a0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphInstance,0>::ecs::core::QueryRemoved<coregame::component::GfxGraphI
    Replacement::draft(0x1410c9350, "ecs_1410c9350", fn_1410c9350 as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphDrawSetup,0>::ecs::core::QueryRemoved<coregame::component::GfxGraph
    Replacement::draft(0x1410c94c0, "ecs_1410c94c0", tpl_6491::<0x10, 0x14028a2a0, 0x1410c9a20, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphDrawSetup,0>::ecs::core::QueryRemoved<coregame::component::GfxGraph
    Replacement::draft(0x1410c9550, "ecs_1410c9550", tpl_381::<0x1410c9cb0, 0x1410c9fc0> as *const ()), // sub_1410c9550
    Replacement::draft(0x1410c96b0, "ecs_1410c96b0", tpl_3191::<0x14338cdf8, 0x140f82730, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphSpawner,0>::ecs::core::QueryRemoved<coregame::component::GfxGraphSp
    Replacement::draft(0x1410c9700, "ecs_1410c9700", tpl_3191::<0x14338ce60, 0x1410c9900, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphInstance,0>::ecs::core::QueryRemoved<coregame::component::GfxGraphI
    Replacement::draft(0x1410c9750, "ecs_1410c9750", tpl_3191::<0x14338ce88, 0x1410c9b70, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::GfxGraphDrawSetup,0>::ecs::core::QueryRemoved<coregame::component::GfxGraph
    Replacement::draft(0x1410db050, "ecs_1410db050", fn_1410db050 as *const ()), // ecs::core::QueryRemoved<coregame::component::FreeCameraState,0>::ecs::core::QueryRemoved<coregame::component::FreeCamera
    Replacement::draft(0x1410db100, "ecs_1410db100", tpl_20813::<0x10, 0x14028a2a0, 0x1410db1e0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::FreeCameraState,0>::ecs::core::QueryRemoved<coregame::component::FreeCamera
    Replacement::draft(0x1410db190, "ecs_1410db190", tpl_3191::<0x143391ce0, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::FreeCameraState,0>::ecs::core::QueryRemoved<coregame::component::FreeCamera
    Replacement::draft(0x1410e49b0, "ecs_1410e49b0", fn_1410e49b0 as *const ()), // ecs::core::QueryRemoved<coregame::component::FogVolume,0>::ecs::core::QueryRemoved<coregame::component::FogVolume,0>::vf
    Replacement::draft(0x1410e4b10, "ecs_1410e4b10", tpl_6221::<0x10, 0x14028a2a0, 0x1410e4e80, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::FogVolume,0>::ecs::core::QueryRemoved<coregame::component::FogVolume,0>::vf
    Replacement::draft(0x1410e4ba0, "ecs_1410e4ba0", fn_1410e4ba0 as *const ()), // ecs::core::QueryRemoved<coregame::component::FogBlocker,0>::ecs::core::QueryRemoved<coregame::component::FogBlocker,0>::
    Replacement::draft(0x1410e4ca0, "ecs_1410e4ca0", tpl_6491::<0x10, 0x14028a2a0, 0x1410e4fc0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::FogBlocker,0>::ecs::core::QueryRemoved<coregame::component::FogBlocker,0>::
    Replacement::draft(0x1410e4d30, "ecs_1410e4d30", tpl_3191::<0x143394060, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::FogVolume,0>::ecs::core::QueryRemoved<coregame::component::FogVolume,0>::vf
    Replacement::draft(0x1410e4d80, "ecs_1410e4d80", fn_1410e4d80 as *const ()), // sub_1410e4d80
    Replacement::draft(0x1410e4e30, "ecs_1410e4e30", tpl_3191::<0x143394038, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::FogBlocker,0>::ecs::core::QueryRemoved<coregame::component::FogBlocker,0>::
    Replacement::draft(0x1410ec2e0, "ecs_1410ec2e0", fn_1410ec2e0 as *const ()), // ecs::core::QueryRemoved<coregame::component::FlareRenderObject,0>::ecs::core::QueryRemoved<coregame::component::FlareRen
    Replacement::draft(0x1410ec390, "ecs_1410ec390", tpl_4350::<0x10, 0x14028a2a0, 0x1410ec470, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::FlareRenderObject,0>::ecs::core::QueryRemoved<coregame::component::FlareRen
    Replacement::draft(0x1410ec420, "ecs_1410ec420", tpl_3191::<0x143398a48, 0x1410ec5d0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::FlareRenderObject,0>::ecs::core::QueryRemoved<coregame::component::FlareRen
    Replacement::draft(0x141107070, "ecs_141107070", fn_141107070 as *const ()), // ecs::core::QueryRemoved<coregame::component::DialogueSource,0>::ecs::core::QueryRemoved<coregame::component::DialogueSou
    Replacement::draft(0x1411071a0, "ecs_1411071a0", tpl_5946::<0x10, 0x14028a2a0, 0x141107280, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::DialogueSource,0>::ecs::core::QueryRemoved<coregame::component::DialogueSou
    Replacement::draft(0x141107230, "ecs_141107230", tpl_3191::<0x14339f7d0, 0x1411073e0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::DialogueSource,0>::ecs::core::QueryRemoved<coregame::component::DialogueSou
    Replacement::draft(0x141112c00, "ecs_141112c00", fn_141112c00 as *const ()), // ecs::core::QueryRemoved<coregame::component::Decal,0>::ecs::core::QueryRemoved<coregame::component::Decal,0>::vfunc_2
    Replacement::draft(0x141112cd0, "ecs_141112cd0", fn_141112cd0 as *const ()), // ecs::core::QueryRemoved<coregame::component::Decal,0>::ecs::core::QueryRemoved<coregame::component::Decal,0>::vfunc_1
    Replacement::draft(0x141112d60, "ecs_141112d60", fn_141112d60 as *const ()), // ecs::core::QueryRemoved<coregame::component::DecalPhysics,0>::ecs::core::QueryRemoved<coregame::component::DecalPhysics,
    Replacement::draft(0x141112e70, "ecs_141112e70", tpl_6491::<0x10, 0x14028a2a0, 0x1411136e0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::DecalPhysics,0>::ecs::core::QueryRemoved<coregame::component::DecalPhysics,
    Replacement::draft(0x141113000, "ecs_141113000", tpl_3796::<0x143cde790, 0x142489ee0, 0x143cdeb24, 0x143c9da04> as *const ()), // sub_141113000
    Replacement::draft(0x141113370, "ecs_141113370", tpl_3191::<0x1433a3400, 0x1411135b0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::Decal,0>::ecs::core::QueryRemoved<coregame::component::Decal,0>::vfunc_0
    Replacement::draft(0x1411133c0, "ecs_1411133c0", tpl_3191::<0x1433a33b8, 0x141113820, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::DecalPhysics,0>::ecs::core::QueryRemoved<coregame::component::DecalPhysics,
    Replacement::draft(0x141117b30, "ecs_141117b30", fn_141117b30 as *const ()), // ecs::core::QueryRemoved<content::DCCLiveCameraComponent,0>::ecs::core::QueryRemoved<content::DCCLiveCameraComponent,0>::
    Replacement::draft(0x141117bb0, "ecs_141117bb0", tpl_4536::<0x10, 0x14028a2a0, 0x141117c90, 0x8> as *const ()), // ecs::core::QueryRemoved<content::DCCLiveCameraComponent,0>::ecs::core::QueryRemoved<content::DCCLiveCameraComponent,0>::
    Replacement::draft(0x141117c40, "ecs_141117c40", tpl_3191::<0x1433a3ef0, 0x141117dc0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<content::DCCLiveCameraComponent,0>::ecs::core::QueryRemoved<content::DCCLiveCameraComponent,0>::
    Replacement::draft(0x14111fac0, "ecs_14111fac0", fn_14111fac0 as *const ()), // ecs::core::QueryRemoved<coregame::component::CoherentTextureViewBindings,0>::ecs::core::QueryRemoved<coregame::component
    Replacement::draft(0x14111fb90, "ecs_14111fb90", tpl_19724::<0x10, 0x14028a2a0, 0x14111fcb0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::CoherentTextureViewBindings,0>::ecs::core::QueryRemoved<coregame::component
    Replacement::draft(0x14111fc20, "ecs_14111fc20", fn_14111fc20 as *const ()), // sub_14111fc20
    Replacement::draft(0x14111fc60, "ecs_14111fc60", tpl_3191::<0x1433a5218, 0x14111fe50, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CoherentTextureViewBindings,0>::ecs::core::QueryRemoved<coregame::component
    Replacement::draft(0x141142380, "ecs_141142380", fn_141142380 as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterFootstep,0>::ecs::core::QueryRemoved<coregame::component::Characte
    Replacement::draft(0x1411424f0, "ecs_1411424f0", fn_1411424f0 as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterFootstep,0>::ecs::core::QueryRemoved<coregame::component::Characte
    Replacement::draft(0x141142580, "ecs_141142580", tpl_3191::<0x1433b3760, 0x141142740, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterFootstep,0>::ecs::core::QueryRemoved<coregame::component::Characte
    Replacement::draft(0x141148b20, "ecs_141148b20", fn_141148b20 as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterController,0>::ecs::core::QueryRemoved<coregame::component::Charac
    Replacement::draft(0x141148bd0, "ecs_141148bd0", tpl_4350::<0x10, 0x14028a2a0, 0x141148d00, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterController,0>::ecs::core::QueryRemoved<coregame::component::Charac
    Replacement::draft(0x141148c60, "ecs_141148c60", tpl_3191::<0x1433b6638, 0x141148f60, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterController,0>::ecs::core::QueryRemoved<coregame::component::Charac
    Replacement::draft(0x141148cb0, "ecs_141148cb0", tpl_3191::<0x1433b6610, 0x1411490a0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::CharacterControllerBody,0>::ecs::core::QueryRemoved<coregame::component::Ch
    Replacement::draft(0x14114f4e0, "ecs_14114f4e0", fn_14114f4e0 as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::BundleRoot,0>::ecs::core::QueryRemoved<coregame::spawning
    Replacement::draft(0x14114f5c0, "ecs_14114f5c0", tpl_3190::<0x10, 0x14028a2a0, 0x14114f6f0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::BundleRoot,0>::ecs::core::QueryRemoved<coregame::spawning
    Replacement::draft(0x14114f650, "ecs_14114f650", tpl_3191::<0x1433b8410, 0x14114f890, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::BundleRoot,0>::ecs::core::QueryRemoved<coregame::spawning
    Replacement::draft(0x14114f6a0, "ecs_14114f6a0", tpl_3191::<0x1433b83d8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::spawning::detail::component::BundleChild,0>::ecs::core::QueryRemoved<coregame::spawnin
    Replacement::draft(0x14115a7d0, "ecs_14115a7d0", fn_14115a7d0 as *const ()), // ecs::core::QueryRemoved<coregame::component::BoneFoley,0>::ecs::core::QueryRemoved<coregame::component::BoneFoley,0>::vf
    Replacement::draft(0x14115a8c0, "ecs_14115a8c0", tpl_20813::<0x10, 0x14028a2a0, 0x14115a9a0, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::BoneFoley,0>::ecs::core::QueryRemoved<coregame::component::BoneFoley,0>::vf
    Replacement::draft(0x14115a950, "ecs_14115a950", tpl_3191::<0x1433ba840, 0x14115ab00, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::BoneFoley,0>::ecs::core::QueryRemoved<coregame::component::BoneFoley,0>::vf
    Replacement::draft(0x141167470, "ecs_141167470", fn_141167470 as *const ()), // ecs::core::QueryRemoved<coregame::component::AudioSubSources,0>::ecs::core::QueryRemoved<coregame::component::AudioSubSo
    Replacement::draft(0x141167540, "ecs_141167540", tpl_4350::<0x10, 0x14028a2a0, 0x141167670, 0x8> as *const ()), // ecs::core::QueryRemoved<coregame::component::AudioSubSources,0>::ecs::core::QueryRemoved<coregame::component::AudioSubSo
    Replacement::draft(0x1411675d0, "ecs_1411675d0", tpl_3191::<0x1433bf2e8, 0x140489dd0, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::AudioObject,0>::ecs::core::QueryRemoved<coregame::component::AudioObject,0>
    Replacement::draft(0x141167620, "ecs_141167620", tpl_3191::<0x1433bf320, 0x141167800, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<coregame::component::AudioSubSources,0>::ecs::core::QueryRemoved<coregame::component::AudioSubSo
    Replacement::draft(0x14116ea10, "ecs_14116ea10", fn_14116ea10 as *const ()), // ecs::core::QueryRemoved<contentcore::ContactSoundOverrideComponent,0>::ecs::core::QueryRemoved<contentcore::ContactSound
    Replacement::draft(0x14116eb20, "ecs_14116eb20", tpl_4936::<0x10, 0x14028a2a0, 0x14116ecc0, 0x8> as *const ()), // ecs::core::QueryRemoved<contentcore::ContactSoundOverrideComponent,0>::ecs::core::QueryRemoved<contentcore::ContactSound
    Replacement::draft(0x14116ebb0, "ecs_14116ebb0", tpl_21430::<0x14116a6e0, 0x1401e66f0> as *const ()), // sub_14116ebb0
    Replacement::draft(0x14116ebd0, "ecs_14116ebd0", tpl_3191::<0x1433c13a8, 0x14116ee30, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<contentcore::ContactSoundOverrideComponent,0>::ecs::core::QueryRemoved<contentcore::ContactSound
    Replacement::draft(0x141b38ae0, "ecs_141b38ae0", fn_141b38ae0 as *const ()), // sub_141b38ae0
    Replacement::draft(0x141b38bb0, "ecs_141b38bb0", fn_141b38bb0 as *const ()), // sub_141b38bb0
    Replacement::draft(0x141b38cd0, "ecs_141b38cd0", tpl_34667::<0x20> as *const ()), // sub_141b38cd0
    Replacement::draft(0x141b38d40, "ecs_141b38d40", tpl_37677::<0x20, 0x40, 0x50, 0x58> as *const ()), // sub_141b38d40
    Replacement::draft_throws(0x141b38d80, "ecs_141b38d80", tpl_1331::<0x8, 0x140612b40, 0x14384cf6c, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141b38d80
    Replacement::draft(0x141b38e30, "ecs_141b38e30", tpl_37192::<0x10, 0x40, 0x8, 0x18, 0x143cbcbe0, 0x1806c, 0x18058, 0x142489ee0, 0xc03> as *const ()), // sub_141b38e30
    Replacement::draft_throws(0x141b38f00, "ecs_141b38f00", tpl_1331::<0x8, 0x140612a50, 0x14384dc70, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141b38f00
    Replacement::draft(0x141b38fb0, "ecs_141b38fb0", tpl_37678::<0x10, 0x8, 0x18> as *const ()), // sub_141b38fb0
    Replacement::draft(0x141b38fd0, "ecs_141b38fd0", tpl_37679::<0x18, 0x8, 0x10> as *const ()), // sub_141b38fd0
    Replacement::draft(0x141b39500, "ecs_141b39500", tpl_37680::<0x18, 0x8, 0x10> as *const ()), // sub_141b39500
    Replacement::draft(0x141b39560, "ecs_141b39560", tpl_37681::<0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x8, 0x10> as *const ()), // sub_141b39560
    Replacement::draft_throws(0x141b39600, "ecs_141b39600", tpl_1331::<0x8, 0x1406127f0, 0x14384c564, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141b39600
    Replacement::draft(0x141b397d0, "ecs_141b397d0", tpl_37682::<0x18, 0x8, 0x10, 0x20, 0x40, 0x38, 0x60, 0x58> as *const ()), // sub_141b397d0
    Replacement::draft(0x141b39820, "ecs_141b39820", tpl_37683::<0x20, 0x18, 0x40, 0x60, 0x38, 0x8, 0x58, 0x10> as *const ()), // sub_141b39820
    Replacement::draft(0x141b398e0, "ecs_141b398e0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x14384cf68, 0x158, 0x14020e8f0> as *const ()), // sub_141b398e0
    Replacement::draft(0x141b39a00, "ecs_141b39a00", fn_141b39a00 as *const ()), // sub_141b39a00
    Replacement::draft_throws(0x141b39ae0, "ecs_141b39ae0", fn_141b39ae0 as *const (), Throws::Discard), // sub_141b39ae0
    Replacement::draft_throws(0x141b39db0, "ecs_141b39db0", fn_141b39db0 as *const (), Throws::Discard), // sub_141b39db0
    Replacement::draft_throws(0x141b39e60, "ecs_141b39e60", fn_141b39e60 as *const (), Throws::Discard), // sub_141b39e60
    Replacement::draft(0x141b3a460, "ecs_141b3a460", fn_141b3a460 as *const ()), // sub_141b3a460
    Replacement::draft(0x141b3a7c0, "ecs_141b3a7c0", fn_141b3a7c0 as *const ()), // sub_141b3a7c0
    Replacement::draft_throws(0x141b3a7f0, "ecs_141b3a7f0", fn_141b3a7f0 as *const (), Throws::Discard), // sub_141b3a7f0
    Replacement::draft(0x141b3acc0, "ecs_141b3acc0", tpl_94428::<0x143201f88, 0x8> as *const ()), // sub_141b3acc0
    Replacement::draft(0x141b3ae30, "ecs_141b3ae30", tpl_34613 as *const ()), // sub_141b3ae30
    Replacement::draft(0x141b3aec0, "ecs_141b3aec0", fn_141b3aec0 as *const ()), // sub_141b3aec0
    Replacement::draft(0x141b3b0d0, "ecs_141b3b0d0", tpl_37689 as *const ()), // sub_141b3b0d0
    Replacement::draft(0x141b3b180, "ecs_141b3b180", tpl_34667::<0x20> as *const ()), // sub_141b3b180
    Replacement::draft(0x141b3c530, "ecs_141b3c530", tpl_35031::<0x142407c80, 0x143c9de20, 0x14020e8f0> as *const ()), // sub_141b3c530
    Replacement::draft(0x141b3c660, "ecs_141b3c660", fn_141b3c660 as *const ()), // sub_141b3c660
    Replacement::draft(0x141b3c790, "ecs_141b3c790", tpl_35031::<0x142407c80, 0x143cbd4c8, 0x14020e8f0> as *const ()), // sub_141b3c790
    Replacement::draft(0x141b3c8c0, "ecs_141b3c8c0", tpl_35032::<0x580f0, 0x142407c80, 0x10, 0x8, 0x140615430> as *const ()), // sub_141b3c8c0
    Replacement::draft(0x141b3c950, "ecs_141b3c950", fn_141b3c950 as *const ()), // sub_141b3c950
    Replacement::draft(0x141b3caa0, "ecs_141b3caa0", fn_141b3caa0 as *const ()), // sub_141b3caa0
    Replacement::draft_throws(0x141b3ce10, "ecs_141b3ce10", fn_141b3ce10 as *const (), Throws::Discard), // sub_141b3ce10
    Replacement::draft(0x141b3cf60, "ecs_141b3cf60", fn_141b3cf60 as *const ()), // sub_141b3cf60
    Replacement::draft_throws(0x141b3cf80, "ecs_141b3cf80", fn_141b3cf80 as *const (), Throws::Caught), // sub_141b3cf80
    Replacement::draft_throws(0x141b3d9a0, "ecs_141b3d9a0", fn_141b3d9a0 as *const (), Throws::Discard), // sub_141b3d9a0
    Replacement::draft(0x141b3dc10, "ecs_141b3dc10", fn_141b3dc10 as *const ()), // sub_141b3dc10
    Replacement::draft(0x141b3dcc0, "ecs_141b3dcc0", fn_141b3dcc0 as *const ()), // sub_141b3dcc0
    Replacement::draft(0x141b3dfb0, "ecs_141b3dfb0", fn_141b3dfb0 as *const ()), // sub_141b3dfb0
    Replacement::draft_throws(0x141b3e1b0, "ecs_141b3e1b0", fn_141b3e1b0 as *const (), Throws::Caught), // sub_141b3e1b0
    Replacement::draft(0x141b3ecf0, "ecs_141b3ecf0", fn_141b3ecf0 as *const ()), // sub_141b3ecf0
    Replacement::draft_throws(0x141b3ef60, "ecs_141b3ef60", fn_141b3ef60 as *const (), Throws::Discard), // sub_141b3ef60
    Replacement::draft(0x141b3f4b0, "ecs_141b3f4b0", fn_141b3f4b0 as *const ()), // sub_141b3f4b0
    Replacement::draft(0x141b3f640, "ecs_141b3f640", fn_141b3f640 as *const ()), // sub_141b3f640
    Replacement::draft(0x141b3f670, "ecs_141b3f670", fn_141b3f670 as *const ()), // sub_141b3f670
    Replacement::draft(0x141b3fa00, "ecs_141b3fa00", fn_141b3fa00 as *const ()), // sub_141b3fa00
    Replacement::draft(0x141b3fb20, "ecs_141b3fb20", fn_141b3fb20 as *const ()), // sub_141b3fb20
    Replacement::draft_throws(0x141b40660, "ecs_141b40660", fn_141b40660 as *const (), Throws::Discard), // sub_141b40660
    Replacement::draft_throws(0x141b40b00, "ecs_141b40b00", fn_141b40b00 as *const (), Throws::Discard), // sub_141b40b00
    Replacement::draft_throws(0x141b40bb0, "ecs_141b40bb0", fn_141b40bb0 as *const (), Throws::Discard), // sub_141b40bb0
    Replacement::draft(0x141b410a0, "ecs_141b410a0", fn_141b410a0 as *const ()), // sub_141b410a0
    Replacement::draft(0x141b412a0, "ecs_141b412a0", fn_141b412a0 as *const ()), // sub_141b412a0
    Replacement::draft_throws(0x141b41300, "ecs_141b41300", fn_141b41300 as *const (), Throws::Discard), // sub_141b41300
    Replacement::draft(0x141b41bf0, "ecs_141b41bf0", tpl_34611::<0x8> as *const ()), // sub_141b41bf0
    Replacement::draft(0x141b41c80, "ecs_141b41c80", fn_141b41c80 as *const ()), // sub_141b41c80
    Replacement::draft(0x141b41d40, "ecs_141b41d40", fn_141b41d40 as *const ()), // sub_141b41d40
    Replacement::draft(0x141b42490, "ecs_141b42490", tpl_35031::<0x142407c80, 0x143cbe098, 0x14020e8f0> as *const ()), // sub_141b42490
    Replacement::draft_throws(0x141b425c0, "ecs_141b425c0", fn_141b425c0 as *const (), Throws::Caught), // sub_141b425c0
    Replacement::draft_throws(0x141b426b0, "ecs_141b426b0", fn_141b426b0 as *const (), Throws::Discard), // sub_141b426b0
    Replacement::draft_throws(0x141b42c20, "ecs_141b42c20", fn_141b42c20 as *const (), Throws::Discard), // sub_141b42c20
    Replacement::draft_throws(0x141b43520, "ecs_141b43520", fn_141b43520 as *const (), Throws::Discard), // sub_141b43520
    Replacement::draft(0x141b43810, "ecs_141b43810", fn_141b43810 as *const ()), // sub_141b43810
    Replacement::draft(0x141b43d50, "ecs_141b43d50", fn_141b43d50 as *const ()), // sub_141b43d50
    Replacement::draft_throws(0x141b43db0, "ecs_141b43db0", fn_141b43db0 as *const (), Throws::Discard), // sub_141b43db0
    Replacement::draft(0x141b43ec0, "ecs_141b43ec0", fn_141b43ec0 as *const ()), // sub_141b43ec0
    Replacement::draft(0x141b43ee0, "ecs_141b43ee0", fn_141b43ee0 as *const ()), // sub_141b43ee0
    Replacement::draft(0x141b43f10, "ecs_141b43f10", fn_141b43f10 as *const ()), // sub_141b43f10
    Replacement::draft(0x141b44080, "ecs_141b44080", fn_141b44080 as *const ()), // sub_141b44080
    Replacement::draft(0x141b44120, "ecs_141b44120", fn_141b44120 as *const ()), // sub_141b44120
    Replacement::draft_throws(0x141b44180, "ecs_141b44180", fn_141b44180 as *const (), Throws::Discard), // sub_141b44180
    Replacement::draft(0x141f7c610, "ecs_141f7c610", tpl_34667::<0x20> as *const ()), // sub_141f7c610
    Replacement::draft(0x141f7c780, "ecs_141f7c780", tpl_35099::<0x20, 0x40> as *const ()), // sub_141f7c780
    Replacement::draft(0x141f7c7c0, "ecs_141f7c7c0", fn_141f7c7c0 as *const ()), // sub_141f7c7c0
    Replacement::draft(0x141f7c820, "ecs_141f7c820", fn_141f7c820 as *const ()), // sub_141f7c820
    Replacement::draft(0x141f7c880, "ecs_141f7c880", fn_141f7c880 as *const ()), // sub_141f7c880
    Replacement::draft(0x141f7c8a0, "ecs_141f7c8a0", tpl_40948::<0x10, 0x18, 0x8> as *const ()), // sub_141f7c8a0
    Replacement::draft(0x141f7c8c0, "ecs_141f7c8c0", fn_141f7c8c0 as *const ()), // sub_141f7c8c0
    Replacement::draft(0x141f7c8f0, "ecs_141f7c8f0", tpl_34694::<0x10> as *const ()), // sub_141f7c8f0
    Replacement::draft(0x141f7c910, "ecs_141f7c910", fn_141f7c910 as *const ()), // sub_141f7c910
    Replacement::draft(0x141f7ca20, "ecs_141f7ca20", tpl_4495::<0x1401de830, 0x18, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x1425371a0, 0x142533200> as *const ()), // sub_141f7ca20
    Replacement::draft(0x141f7cad0, "ecs_141f7cad0", tpl_65350::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc8, 0x142442de0> as *const ()), // sub_141f7cad0
    Replacement::draft(0x141f7cc10, "ecs_141f7cc10", tpl_3191::<0x14339a868, 0x141f7ca20, 0x8, 0x1401de830, 0x141834d30> as *const ()), // ecs::core::QueryRemoved<anonymous_namespace::EditorCameraState,0>::ecs::core::QueryRemoved<anonymous_namespace::EditorCa
    Replacement::draft(0x141f7cc60, "ecs_141f7cc60", tpl_34693::<0x10, 0x18, 0x20> as *const ()), // sub_141f7cc60
    Replacement::draft(0x141f7cc90, "ecs_141f7cc90", fn_141f7cc90 as *const ()), // sub_141f7cc90
    Replacement::draft(0x141f7ccc0, "ecs_141f7ccc0", tpl_19620::<0x8, 0x141f7ca20> as *const ()), // sub_141f7ccc0
    Replacement::draft(0x141f7cd50, "ecs_141f7cd50", fn_141f7cd50 as *const ()), // sub_141f7cd50
    Replacement::draft(0x141f7cd90, "ecs_141f7cd90", tpl_6240::<0x10, 0x14028a2a0, 0x141f7cad0, 0x8> as *const ()), // ecs::core::QueryRemoved<anonymous_namespace::EditorCameraState,0>::ecs::core::QueryRemoved<anonymous_namespace::EditorCa
    Replacement::draft(0x141f7ce30, "ecs_141f7ce30", tpl_6239::<0x8, 0x14183acc8, 0x20, 0x40> as *const ()), // ecs::core::QueryRemoved<anonymous_namespace::EditorCameraState,0>::ecs::core::QueryRemoved<anonymous_namespace::EditorCa
];
