#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x7000a84a20, fn_7000a84a20 as *const (), 0), // +0x398a20 sub_398a20
    Lifted::new(0x7000a84a60, tpl_741::<0x700078de90> as *const (), 0), // +0x398a60 sub_398a60
    Lifted::new(0x7000a84ab0, tpl_741::<0x7000749580> as *const (), 0), // +0x398ab0 sub_398ab0
    Lifted::new(0x7000a84b00, tpl_741::<0x700072df70> as *const (), 0), // +0x398b00 sub_398b00
    Lifted::new(0x7000a84b60, tpl_733::<0x7000753060> as *const (), 0), // +0x398b60 sub_398b60
    Lifted::new(0x7000a84ba0, tpl_741::<0x70007504e0> as *const (), 0), // +0x398ba0 sub_398ba0
    Lifted::new(0x7000a84bf0, tpl_741::<0x7000733b70> as *const (), 0), // +0x398bf0 sub_398bf0
    Lifted::new(0x7000a84c40, tpl_741::<0x70007850f0> as *const (), 0), // +0x398c40 sub_398c40
    Lifted::new(0x7000a84c90, fn_7000a84c90 as *const (), 1), // +0x398c90 sub_398c90
    Lifted::new(0x7000a84cc0, fn_7000a84cc0 as *const (), 1), // +0x398cc0 sub_398cc0
    Lifted::new(0x7000a84cf0, tpl_172::<0x20> as *const (), 1), // +0x398cf0 sub_398cf0
    Lifted::new(0x7000a84d00, fn_7000a84d00 as *const (), 0), // +0x398d00 sub_398d00
    Lifted::new(0x7000a84d80, tpl_742::<0x7001c6abd8, 0x7000a824a0, 0x8, 0x700070e080> as *const (), 2), // +0x398d80 sub_398d80
    Lifted::new(0x7000a84e10, fn_7000a84e10 as *const (), 2), // +0x398e10 sub_398e10
    Lifted::new(0x7000a84ee0, fn_7000a84ee0 as *const (), 0), // +0x398ee0 sub_398ee0
    Lifted::new(0x7000a84f20, fn_7000a84f20 as *const (), 2), // +0x398f20 sub_398f20
    Lifted::new(0x7000a84fa0, fn_7000a84fa0 as *const (), 2), // +0x398fa0 sub_398fa0
    Lifted::new(0x7000a85030, fn_7000a85030 as *const (), 2), // +0x399030 sub_399030
    Lifted::new(0x7000a850c0, fn_7000a850c0 as *const (), 0), // +0x3990c0 sub_3990c0
    Lifted::new(0x7000a85110, tpl_742::<0x7001c6abd8, 0x7000a824a0, 0x8, 0x7000727f30> as *const (), 2), // +0x399110 sub_399110
    Lifted::new(0x7000a851a0, tpl_742::<0x7001c6abd8, 0x7000a824a0, 0x8, 0x7000799650> as *const (), 2), // +0x3991a0 sub_3991a0
    Lifted::new(0x7000a85230, fn_7000a85230 as *const (), 2), // +0x399230 sub_399230
    Lifted::new(0x7000a852e0, fn_7000a852e0 as *const (), 2), // +0x3992e0 sub_3992e0
    Lifted::new(0x7000a85390, tpl_140::<0xd0, 0x1> as *const (), 1), // +0x399390 sub_399390
    Lifted::new(0x7000a853b0, tpl_743::<0x700070a310> as *const (), 0), // +0x3993b0 sub_3993b0
    Lifted::new(0x7000a853e0, fn_7000a853e0 as *const (), 0), // +0x3993e0 sub_3993e0
    Lifted::new(0x7000a855d0, fn_7000a855d0 as *const (), 0), // +0x3995d0 sub_3995d0
    Lifted::new(0x7000a85710, fn_7000a85710 as *const (), 0), // +0x399710 sub_399710
    Lifted::new(0x7000a85940, fn_7000a85940 as *const (), 2), // +0x399940 sub_399940
    Lifted::new(0x7000a85a50, fn_7000a85a50 as *const (), 2), // +0x399a50 sub_399a50
    Lifted::new(0x7000a85b60, fn_7000a85b60 as *const (), 2), // +0x399b60 sub_399b60
    Lifted::new(0x7000a85c10, tpl_741::<0x70007504e0> as *const (), 0), // +0x399c10 sub_399c10
    Lifted::new(0x7000a85c60, tpl_741::<0x70007850f0> as *const (), 0), // +0x399c60 sub_399c60
    Lifted::new(0x7000a85cb0, tpl_741::<0x70007850f0> as *const (), 0), // +0x399cb0 sub_399cb0
    Lifted::new(0x7000a85d10, tpl_731::<0x7000757b00> as *const (), 0), // +0x399d10 sub_399d10
    Lifted::new(0x7000a85d40, fn_7000a85d40 as *const (), 1), // +0x399d40 sub_399d40
    Lifted::new(0x7000a85da0, fn_7000a85da0 as *const (), 0), // +0x399da0 sub_399da0
    Lifted::new(0x7000a85ea0, fn_7000a85ea0 as *const (), 0), // +0x399ea0 sub_399ea0
    Lifted::new(0x7000a85f50, tpl_744::<0x7001c6abd8, 0x7000a85230, 0x70006f3410, 0x70007507b0> as *const (), 0), // +0x399f50 sub_399f50
    Lifted::new(0x7000a85ff0, fn_7000a85ff0 as *const (), 0), // +0x399ff0 sub_399ff0
    Lifted::new(0x7000a860a0, fn_7000a860a0 as *const (), 0), // +0x39a0a0 sub_39a0a0
    Lifted::new(0x7000a86140, tpl_744::<0x7001c6abd8, 0x7000a85230, 0x70006f3410, 0x70007507b0> as *const (), 0), // +0x39a140 sub_39a140
    Lifted::new(0x7000a861e0, fn_7000a861e0 as *const (), 0), // +0x39a1e0 sub_39a1e0
    Lifted::new(0x7000a86290, fn_7000a86290 as *const (), 0), // +0x39a290 sub_39a290
    Lifted::new(0x7000a86320, fn_7000a86320 as *const (), 0), // +0x39a320 sub_39a320
    Lifted::new(0x7000a863d0, tpl_718::<0x7001c6abd8, 0x70006f3410, 0x70007507b0> as *const (), 0), // +0x39a3d0 sub_39a3d0
    Lifted::new(0x7000a86460, tpl_718::<0x7001c6abd8, 0x70006f3410, 0x70007507b0> as *const (), 0), // +0x39a460 sub_39a460
    Lifted::new(0x7000a864f0, tpl_745 as *const (), 1), // +0x39a4f0 sub_39a4f0
    Lifted::new(0x7000a86500, tpl_745 as *const (), 1), // +0x39a500 sub_39a500
    Lifted::new(0x7000a86510, tpl_745 as *const (), 1), // +0x39a510 sub_39a510
    Lifted::new(0x7000a86520, fn_7000a86520 as *const (), 0), // +0x39a520 sub_39a520
    Lifted::new(0x7000a86580, fn_7000a86580 as *const (), 0), // +0x39a580 sub_39a580
    Lifted::new(0x7000a86610, fn_7000a86610 as *const (), 2), // +0x39a610 sub_39a610
    Lifted::new(0x7000a86710, fn_7000a86710 as *const (), 2), // +0x39a710 sub_39a710
    Lifted::new(0x7000a86790, tpl_44 as *const (), 1), // +0x39a790 sub_39a790
    Lifted::new(0x7000a867a0, fn_7000a867a0 as *const (), 2), // +0x39a7a0 sub_39a7a0
    Lifted::new(0x7000a869f0, tpl_741::<0x700073c720> as *const (), 0), // +0x39a9f0 sub_39a9f0
    Lifted::new(0x7000a86a40, tpl_743::<0x700071d050> as *const (), 0), // +0x39aa40 sub_39aa40
    Lifted::new(0x7000a86a70, tpl_741::<0x70006fa410> as *const (), 0), // +0x39aa70 sub_39aa70
    Lifted::new(0x7000a86ac0, tpl_741::<0x700078de90> as *const (), 0), // +0x39aac0 sub_39aac0
    Lifted::new(0x7000a86b10, tpl_743::<0x700071dc00> as *const (), 0), // +0x39ab10 sub_39ab10
    Lifted::new(0x7000a86b40, tpl_741::<0x7000749580> as *const (), 0), // +0x39ab40 sub_39ab40
    Lifted::new(0x7000a86b90, tpl_741::<0x700072df70> as *const (), 0), // +0x39ab90 sub_39ab90
    Lifted::new(0x7000a86bf0, tpl_740::<0x7000704f00> as *const (), 0), // +0x39abf0 sub_39abf0
    Lifted::new(0x7000a86c30, tpl_741::<0x70007504e0> as *const (), 0), // +0x39ac30 sub_39ac30
    Lifted::new(0x7000a86c80, tpl_743::<0x700070a310> as *const (), 0), // +0x39ac80 sub_39ac80
    Lifted::new(0x7000a86cb0, tpl_741::<0x70007850f0> as *const (), 0), // +0x39acb0 sub_39acb0
    Lifted::new(0x7000a86d00, tpl_745 as *const (), 1), // +0x39ad00 sub_39ad00
    Lifted::new(0x7000a86d10, tpl_745 as *const (), 1), // +0x39ad10 sub_39ad10
    Lifted::new(0x7000a86d20, tpl_745 as *const (), 1), // +0x39ad20 sub_39ad20
    Lifted::new(0x7000a86d30, fn_7000a86d30 as *const (), 0), // +0x39ad30 sub_39ad30
    Lifted::new(0x7000a86e20, tpl_746::<0x1f0, 0x70006ff9d0, 0x4, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x39ae20 sub_39ae20
    Lifted::new(0x7000a86ea0, tpl_746::<0x1f0, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x39aea0 sub_39aea0
    Lifted::new(0x7000a86f20, fn_7000a86f20 as *const (), 2), // +0x39af20 sub_39af20
    Lifted::new(0x7000a87200, fn_7000a87200 as *const (), 0), // +0x39b200 sub_39b200
    Lifted::new(0x7000a872b0, fn_7000a872b0 as *const (), 0), // +0x39b2b0 sub_39b2b0
    Lifted::new(0x7000a87300, fn_7000a87300 as *const (), 0), // +0x39b300 sub_39b300
    Lifted::new(0x7000a87390, fn_7000a87390 as *const (), 0), // +0x39b390 sub_39b390
    Lifted::new(0x7000a87420, fn_7000a87420 as *const (), 0), // +0x39b420 sub_39b420
    Lifted::new(0x7000a874b0, fn_7000a874b0 as *const (), 0), // +0x39b4b0 sub_39b4b0
    Lifted::new(0x7000a87540, fn_7000a87540 as *const (), 0), // +0x39b540 sub_39b540
    Lifted::new(0x7000a87590, fn_7000a87590 as *const (), 0), // +0x39b590 sub_39b590
    Lifted::new(0x7000a87620, fn_7000a87620 as *const (), 0), // +0x39b620 sub_39b620
    Lifted::new(0x7000a876b0, fn_7000a876b0 as *const (), 0), // +0x39b6b0 sub_39b6b0
    Lifted::new(0x7000a87740, fn_7000a87740 as *const (), 0), // +0x39b740 sub_39b740
    Lifted::new(0x7000a877d0, fn_7000a877d0 as *const (), 0), // +0x39b7d0 sub_39b7d0
    Lifted::new(0x7000a878f0, fn_7000a878f0 as *const (), 0), // +0x39b8f0 sub_39b8f0
    Lifted::new(0x7000a87a30, fn_7000a87a30 as *const (), 0), // +0x39ba30 sub_39ba30
    Lifted::new(0x7000a87bb0, fn_7000a87bb0 as *const (), 0), // +0x39bbb0 sub_39bbb0
    Lifted::new(0x7000a87d70, fn_7000a87d70 as *const (), 0), // +0x39bd70 sub_39bd70
    Lifted::new(0x7000a87f80, fn_7000a87f80 as *const (), 0), // +0x39bf80 sub_39bf80
    Lifted::new(0x7000a880a0, fn_7000a880a0 as *const (), 0), // +0x39c0a0 sub_39c0a0
    Lifted::new(0x7000a88110, fn_7000a88110 as *const (), 0), // +0x39c110 sub_39c110
    Lifted::new(0x7000a881f0, fn_7000a881f0 as *const (), 0), // +0x39c1f0 sub_39c1f0
    Lifted::new(0x7000a88350, fn_7000a88350 as *const (), 0), // +0x39c350 sub_39c350
    Lifted::new(0x7000a88500, fn_7000a88500 as *const (), 0), // +0x39c500 sub_39c500
    Lifted::new(0x7000a886f0, fn_7000a886f0 as *const (), 0), // +0x39c6f0 sub_39c6f0
    Lifted::new(0x7000a888b0, fn_7000a888b0 as *const (), 0), // +0x39c8b0 sub_39c8b0
    Lifted::new(0x7000a88a20, fn_7000a88a20 as *const (), 0), // +0x39ca20 sub_39ca20
    Lifted::new(0x7000a88b30, fn_7000a88b30 as *const (), 2), // +0x39cb30 sub_39cb30
    Lifted::new(0x7000a88c50, fn_7000a88c50 as *const (), 0), // +0x39cc50 sub_39cc50
    Lifted::new(0x7000a88da0, fn_7000a88da0 as *const (), 0), // +0x39cda0 sub_39cda0
    Lifted::new(0x7000a88f80, fn_7000a88f80 as *const (), 0), // +0x39cf80 sub_39cf80
    Lifted::new(0x7000a88fe0, fn_7000a88fe0 as *const (), 2), // +0x39cfe0 sub_39cfe0
    Lifted::new(0x7000a89270, fn_7000a89270 as *const (), 0), // +0x39d270 sub_39d270
    Lifted::new(0x7000a892c0, fn_7000a892c0 as *const (), 0), // +0x39d2c0 sub_39d2c0
    Lifted::new(0x7000a89350, fn_7000a89350 as *const (), 2), // +0x39d350 sub_39d350
    Lifted::new(0x7000a895b0, fn_7000a895b0 as *const (), 2), // +0x39d5b0 sub_39d5b0
    Lifted::new(0x7000a896c0, tpl_705::<0x7000784c80, 0x1f0, 0x7000a85230, 0x7000a82130> as *const (), 0), // +0x39d6c0 sub_39d6c0
    Lifted::new(0x7000a89710, fn_7000a89710 as *const (), 0), // +0x39d710 sub_39d710
    Lifted::new(0x7000a89860, fn_7000a89860 as *const (), 0), // +0x39d860 sub_39d860
    Lifted::new(0x7000a899b0, fn_7000a899b0 as *const (), 2), // +0x39d9b0 sub_39d9b0
    Lifted::new(0x7000a89b90, fn_7000a89b90 as *const (), 2), // +0x39db90 sub_39db90
    Lifted::new(0x7000a89e20, fn_7000a89e20 as *const (), 2), // +0x39de20 sub_39de20
    Lifted::new(0x7000a89fc0, fn_7000a89fc0 as *const (), 2), // +0x39dfc0 sub_39dfc0
    Lifted::new(0x7000a8a0c0, fn_7000a8a0c0 as *const (), 2), // +0x39e0c0 sub_39e0c0
    Lifted::new(0x7000a8a1c0, fn_7000a8a1c0 as *const (), 0), // +0x39e1c0 sub_39e1c0
    Lifted::new(0x7000a8a250, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a261> as *const (), 0), // +0x39e250 sub_39e250
    Lifted::new(0x7000a8a270, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a281> as *const (), 0), // +0x39e270 sub_39e270
    Lifted::new(0x7000a8a290, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a2a1> as *const (), 0), // +0x39e290 sub_39e290
    Lifted::new(0x7000a8a2b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a2c1> as *const (), 0), // +0x39e2b0 sub_39e2b0
    Lifted::new(0x7000a8a2d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a2e1> as *const (), 0), // +0x39e2d0 sub_39e2d0
    Lifted::new(0x7000a8a2f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a301> as *const (), 0), // +0x39e2f0 sub_39e2f0
    Lifted::new(0x7000a8a310, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a321> as *const (), 0), // +0x39e310 sub_39e310
    Lifted::new(0x7000a8a330, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a341> as *const (), 0), // +0x39e330 sub_39e330
    Lifted::new(0x7000a8a350, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a361> as *const (), 0), // +0x39e350 sub_39e350
    Lifted::new(0x7000a8a370, fn_7000a8a370 as *const (), 0), // +0x39e370 sub_39e370
    Lifted::new(0x7000a8a3f0, fn_7000a8a3f0 as *const (), 0), // +0x39e3f0 sub_39e3f0
    Lifted::new(0x7000a8a440, fn_7000a8a440 as *const (), 0), // +0x39e440 sub_39e440
    Lifted::new(0x7000a8a500, fn_7000a8a500 as *const (), 0), // +0x39e500 sub_39e500
    Lifted::new(0x7000a8a5c0, fn_7000a8a5c0 as *const (), 0), // +0x39e5c0 sub_39e5c0
    Lifted::new(0x7000a8a650, fn_7000a8a650 as *const (), 0), // +0x39e650 sub_39e650
    Lifted::new(0x7000a8a6e0, fn_7000a8a6e0 as *const (), 0), // +0x39e6e0 sub_39e6e0
    Lifted::new(0x7000a8a770, fn_7000a8a770 as *const (), 0), // +0x39e770 sub_39e770
    Lifted::new(0x7000a8a800, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8a811> as *const (), 0), // +0x39e800 sub_39e800
    Lifted::new(0x7000a8a820, tpl_747::<0x70017d976c, 0x70017d98f0, 0x7000a85da0> as *const (), 0), // +0x39e820 sub_39e820
    Lifted::new(0x7000a8a870, fn_7000a8a870 as *const (), 0), // +0x39e870 sub_39e870
    Lifted::new(0x7000a8a8f0, fn_7000a8a8f0 as *const (), 2), // +0x39e8f0 sub_39e8f0
    Lifted::new(0x7000a8aad0, fn_7000a8aad0 as *const (), 2), // +0x39ead0 sub_39ead0
    Lifted::new(0x7000a8ac60, fn_7000a8ac60 as *const (), 0), // +0x39ec60 sub_39ec60
    Lifted::new(0x7000a8ace0, fn_7000a8ace0 as *const (), 0), // +0x39ece0 sub_39ece0
    Lifted::new(0x7000a8ade0, fn_7000a8ade0 as *const (), 0), // +0x39ede0 sub_39ede0
    Lifted::new(0x7000a8aeb0, fn_7000a8aeb0 as *const (), 2), // +0x39eeb0 sub_39eeb0
    Lifted::new(0x7000a8b090, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a8b0a1> as *const (), 0), // +0x39f090 sub_39f090
    Lifted::new(0x7000a8b0b0, fn_7000a8b0b0 as *const (), 0), // +0x39f0b0 sub_39f0b0
    Lifted::new(0x7000a8b100, fn_7000a8b100 as *const (), 2), // +0x39f100 sub_39f100
    Lifted::new(0x7000a8b260, tpl_711::<0x7001c6abd8, 0x7000a82f90, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39f260 sub_39f260
    Lifted::new(0x7000a8b410, tpl_712::<0x7001c6abd8, 0x7000a83060, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39f410 sub_39f410
    Lifted::new(0x7000a8b5c0, tpl_708::<0x7001c6abd8, 0x7000a83130, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39f5c0 sub_39f5c0
    Lifted::new(0x7000a8b770, tpl_709::<0x7001c6abd8, 0x7000a83200, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39f770 sub_39f770
    Lifted::new(0x7000a8b920, tpl_710::<0x7001c6abd8, 0x7000a832d0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39f920 sub_39f920
    Lifted::new(0x7000a8bad0, tpl_703::<0x7001c6abd8, 0x7000a833a0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39fad0 sub_39fad0
    Lifted::new(0x7000a8bc80, tpl_704::<0x7001c6abd8, 0x7000a83470, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39fc80 sub_39fc80
    Lifted::new(0x7000a8be30, tpl_701::<0x7001c6abd8, 0x7000a83540, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39fe30 sub_39fe30
    Lifted::new(0x7000a8bfe0, tpl_700::<0x7001c6abd8, 0x7000a83610, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x39ffe0 sub_39ffe0
    Lifted::new(0x7000a8c190, tpl_702::<0x7001c6abd8, 0x7000a836e0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x3a0190 sub_3a0190
    Lifted::new(0x7000a8c340, tpl_707::<0x7001c6abd8, 0x7000a83930, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x3a0340 sub_3a0340
    Lifted::new(0x7000a8c4f0, tpl_706::<0x1f0, 0x7000a85230, 0x7000a82130> as *const (), 0), // +0x3a04f0 sub_3a04f0
    Lifted::new(0x7000a8c530, tpl_686::<0x7001c6abd8, 0x7000a83ad0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x3a0530 sub_3a0530
    Lifted::new(0x7000a8c6e0, tpl_687::<0x7001c6abd8, 0x7000a83a00, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x3a06e0 sub_3a06e0
    Lifted::new(0x7000a8c890, fn_7000a8c890 as *const (), 2), // +0x3a0890 sub_3a0890
    Lifted::new(0x7000a8c930, fn_7000a8c930 as *const (), 2), // +0x3a0930 sub_3a0930
    Lifted::new(0x7000a8c9e0, fn_7000a8c9e0 as *const (), 2), // +0x3a09e0 sub_3a09e0
    Lifted::new(0x7000a8d9c0, tpl_748::<0x1f0, 0x8, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a19c0 sub_3a19c0
    Lifted::new(0x7000a8da40, tpl_748::<0x1f0, 0x8, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1a40 sub_3a1a40
    Lifted::new(0x7000a8dac0, fn_7000a8dac0 as *const (), 0), // +0x3a1ac0 sub_3a1ac0
    Lifted::new(0x7000a8dbf0, tpl_748::<0x1f0, 0x6, 0x70006ff9d0, 0x4, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1bf0 sub_3a1bf0
    Lifted::new(0x7000a8dc70, tpl_748::<0x1f0, 0x6, 0x70006ff9d0, 0x4, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1c70 sub_3a1c70
    Lifted::new(0x7000a8dcf0, tpl_749::<0x1f0, 0x4, 0x70006ff9d0, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1cf0 sub_3a1cf0
    Lifted::new(0x7000a8dd70, tpl_749::<0x1f0, 0x4, 0x70006ff9d0, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1d70 sub_3a1d70
    Lifted::new(0x7000a8ddf0, tpl_748::<0x1f0, 0x6, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1df0 sub_3a1df0
    Lifted::new(0x7000a8de70, tpl_748::<0x1f0, 0x6, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1e70 sub_3a1e70
    Lifted::new(0x7000a8def0, tpl_750::<0x1f0, 0x70006f3410, 0x7000a82130, 0x2, 0x7000722ae0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1ef0 sub_3a1ef0
    Lifted::new(0x7000a8dfb0, tpl_750::<0x1f0, 0x70006f3410, 0x7000a82130, 0x2, 0x7000722ae0, 0x4, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a1fb0 sub_3a1fb0
    Lifted::new(0x7000a8e070, tpl_748::<0x1f0, 0x7, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a2070 sub_3a2070
    Lifted::new(0x7000a8e0f0, tpl_748::<0x1f0, 0x7, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a20f0 sub_3a20f0
    Lifted::new(0x7000a8e170, tpl_748::<0x1f0, 0x4, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a2170 sub_3a2170
    Lifted::new(0x7000a8e1f0, tpl_748::<0x1f0, 0x4, 0x70006ff9d0, 0x5, 0x700079e7f0, 0x7000a86d30, 0x700072a920> as *const (), 0), // +0x3a21f0 sub_3a21f0
    Lifted::new(0x7000a8e270, fn_7000a8e270 as *const (), 0), // +0x3a2270 sub_3a2270
    Lifted::new(0x7000a8e360, fn_7000a8e360 as *const (), 2), // +0x3a2360 sub_3a2360
    Lifted::new(0x7000a8e570, fn_7000a8e570 as *const (), 0), // +0x3a2570 sub_3a2570
    Lifted::new(0x7000a8e5c0, fn_7000a8e5c0 as *const (), 2), // +0x3a25c0 sub_3a25c0
    Lifted::new(0x7000a8e760, fn_7000a8e760 as *const (), 2), // +0x3a2760 sub_3a2760
    Lifted::new(0x7000a8e8a0, fn_7000a8e8a0 as *const (), 0), // +0x3a28a0 sub_3a28a0
    Lifted::new(0x7000a8e960, fn_7000a8e960 as *const (), 0), // +0x3a2960 sub_3a2960
    Lifted::new(0x7000a8ea40, fn_7000a8ea40 as *const (), 0), // +0x3a2a40 sub_3a2a40
    Lifted::new(0x7000a8eb20, fn_7000a8eb20 as *const (), 0), // +0x3a2b20 sub_3a2b20
    Lifted::new(0x7000a8ebe0, fn_7000a8ebe0 as *const (), 0), // +0x3a2be0 sub_3a2be0
    Lifted::new(0x7000a8eca0, fn_7000a8eca0 as *const (), 0), // +0x3a2ca0 sub_3a2ca0
    Lifted::new(0x7000a8ed60, fn_7000a8ed60 as *const (), 0), // +0x3a2d60 sub_3a2d60
    Lifted::new(0x7000a8ee20, fn_7000a8ee20 as *const (), 0), // +0x3a2e20 sub_3a2e20
    Lifted::new(0x7000a8ee70, fn_7000a8ee70 as *const (), 0), // +0x3a2e70 sub_3a2e70
    Lifted::new(0x7000a8eec0, fn_7000a8eec0 as *const (), 2), // +0x3a2ec0 sub_3a2ec0
    Lifted::new(0x7000a8f150, fn_7000a8f150 as *const (), 0), // +0x3a3150 sub_3a3150
    Lifted::new(0x7000a8f260, fn_7000a8f260 as *const (), 0), // +0x3a3260 sub_3a3260
    Lifted::new(0x7000a8f3c0, fn_7000a8f3c0 as *const (), 0), // +0x3a33c0 sub_3a33c0
    Lifted::new(0x7000a8f480, fn_7000a8f480 as *const (), 0), // +0x3a3480 sub_3a3480
    Lifted::new(0x7000a8f4d0, tpl_747::<0x70017d976c, 0x70017d97f0, 0x7000a85da0> as *const (), 0), // +0x3a34d0 sub_3a34d0
    Lifted::new(0x7000a8f520, fn_7000a8f520 as *const (), 0), // +0x3a3520 sub_3a3520
    Lifted::new(0x7000a8f5b0, fn_7000a8f5b0 as *const (), 0), // +0x3a35b0 sub_3a35b0
    Lifted::new(0x7000a8f640, fn_7000a8f640 as *const (), 0), // +0x3a3640 sub_3a3640
    Lifted::new(0x7000a8f720, fn_7000a8f720 as *const (), 0), // +0x3a3720 sub_3a3720
    Lifted::new(0x7000a8f7b0, fn_7000a8f7b0 as *const (), 0), // +0x3a37b0 sub_3a37b0
    Lifted::new(0x7000a8f840, fn_7000a8f840 as *const (), 0), // +0x3a3840 sub_3a3840
    Lifted::new(0x7000a8f890, fn_7000a8f890 as *const (), 0), // +0x3a3890 sub_3a3890
    Lifted::new(0x7000a8f960, fn_7000a8f960 as *const (), 0), // +0x3a3960 sub_3a3960
    Lifted::new(0x7000a8fa50, fn_7000a8fa50 as *const (), 0), // +0x3a3a50 sub_3a3a50
    Lifted::new(0x7000a8fa90, fn_7000a8fa90 as *const (), 0), // +0x3a3a90 sub_3a3a90
    Lifted::new(0x7000a8fbb0, fn_7000a8fbb0 as *const (), 2), // +0x3a3bb0 sub_3a3bb0
    Lifted::new(0x7000a8fcf0, fn_7000a8fcf0 as *const (), 2), // +0x3a3cf0 sub_3a3cf0
    Lifted::new(0x7000a8ff50, fn_7000a8ff50 as *const (), 0), // +0x3a3f50 sub_3a3f50
    Lifted::new(0x7000a8ffe0, fn_7000a8ffe0 as *const (), 0), // +0x3a3fe0 sub_3a3fe0
    Lifted::new(0x7000a90070, fn_7000a90070 as *const (), 0), // +0x3a4070 sub_3a4070
    Lifted::new(0x7000a90100, fn_7000a90100 as *const (), 0), // +0x3a4100 sub_3a4100
    Lifted::new(0x7000a90190, fn_7000a90190 as *const (), 0), // +0x3a4190 sub_3a4190
    Lifted::new(0x7000a90220, fn_7000a90220 as *const (), 0), // +0x3a4220 sub_3a4220
    Lifted::new(0x7000a902b0, fn_7000a902b0 as *const (), 2), // +0x3a42b0 sub_3a42b0
    Lifted::new(0x7000a93400, fn_7000a93400 as *const (), 2), // +0x3a7400 sub_3a7400
    Lifted::new(0x7000a93530, fn_7000a93530 as *const (), 0), // +0x3a7530 sub_3a7530
    Lifted::new(0x7000a935c0, fn_7000a935c0 as *const (), 2), // +0x3a75c0 sub_3a75c0
    Lifted::new(0x7000a93780, fn_7000a93780 as *const (), 0), // +0x3a7780 sub_3a7780
    Lifted::new(0x7000a93860, fn_7000a93860 as *const (), 0), // +0x3a7860 sub_3a7860
    Lifted::new(0x7000a93920, fn_7000a93920 as *const (), 0), // +0x3a7920 sub_3a7920
    Lifted::new(0x7000a939e0, fn_7000a939e0 as *const (), 0), // +0x3a79e0 sub_3a79e0
    Lifted::new(0x7000a93ae0, fn_7000a93ae0 as *const (), 2), // +0x3a7ae0 sub_3a7ae0
    Lifted::new(0x7000a93d20, fn_7000a93d20 as *const (), 0), // +0x3a7d20 sub_3a7d20
    Lifted::new(0x7000a93e30, fn_7000a93e30 as *const (), 0), // +0x3a7e30 sub_3a7e30
    Lifted::new(0x7000a93e50, tpl_751::<0xa, 0x7000a852e0> as *const (), 0), // +0x3a7e50 sub_3a7e50
    Lifted::new(0x7000a93e70, tpl_751::<0xb, 0x7000a852e0> as *const (), 0), // +0x3a7e70 sub_3a7e70
    Lifted::new(0x7000a93e90, tpl_751::<0xc, 0x7000a852e0> as *const (), 0), // +0x3a7e90 sub_3a7e90
    Lifted::new(0x7000a93eb0, tpl_751::<0xd, 0x7000a852e0> as *const (), 0), // +0x3a7eb0 sub_3a7eb0
    Lifted::new(0x7000a93ed0, tpl_751::<0xe, 0x7000a852e0> as *const (), 0), // +0x3a7ed0 sub_3a7ed0
    Lifted::new(0x7000a93ef0, tpl_751::<0xf, 0x7000a852e0> as *const (), 0), // +0x3a7ef0 sub_3a7ef0
    Lifted::new(0x7000a93f10, tpl_751::<0x1, 0x7000a852e0> as *const (), 0), // +0x3a7f10 sub_3a7f10
    Lifted::new(0x7000a93f30, tpl_751::<0x2, 0x7000a852e0> as *const (), 0), // +0x3a7f30 sub_3a7f30
    Lifted::new(0x7000a93f50, tpl_751::<0x3, 0x7000a852e0> as *const (), 0), // +0x3a7f50 sub_3a7f50
    Lifted::new(0x7000a93f70, tpl_751::<0x4, 0x7000a852e0> as *const (), 0), // +0x3a7f70 sub_3a7f70
    Lifted::new(0x7000a93f90, tpl_751::<0x5, 0x7000a852e0> as *const (), 0), // +0x3a7f90 sub_3a7f90
    Lifted::new(0x7000a93fb0, tpl_751::<0x6, 0x7000a852e0> as *const (), 0), // +0x3a7fb0 sub_3a7fb0
    Lifted::new(0x7000a93fd0, tpl_751::<0x7, 0x7000a852e0> as *const (), 0), // +0x3a7fd0 sub_3a7fd0
    Lifted::new(0x7000a93ff0, tpl_751::<0x8, 0x7000a852e0> as *const (), 0), // +0x3a7ff0 sub_3a7ff0
    Lifted::new(0x7000a94010, tpl_751::<0x9, 0x7000a852e0> as *const (), 0), // +0x3a8010 sub_3a8010
    Lifted::new(0x7000a94030, fn_7000a94030 as *const (), 0), // +0x3a8030 sub_3a8030
    Lifted::new(0x7000a94080, fn_7000a94080 as *const (), 0), // +0x3a8080 sub_3a8080
    Lifted::new(0x7000a94110, fn_7000a94110 as *const (), 0), // +0x3a8110 sub_3a8110
    Lifted::new(0x7000a941a0, fn_7000a941a0 as *const (), 0), // +0x3a81a0 sub_3a81a0
    Lifted::new(0x7000a94370, fn_7000a94370 as *const (), 2), // +0x3a8370 sub_3a8370
    Lifted::new(0x7000a947a0, fn_7000a947a0 as *const (), 0), // +0x3a87a0 sub_3a87a0
    Lifted::new(0x7000a94a90, fn_7000a94a90 as *const (), 0), // +0x3a8a90 sub_3a8a90
    Lifted::new(0x7000a94b20, fn_7000a94b20 as *const (), 0), // +0x3a8b20 sub_3a8b20
    Lifted::new(0x7000a94bb0, fn_7000a94bb0 as *const (), 0), // +0x3a8bb0 sub_3a8bb0
    Lifted::new(0x7000a94c40, fn_7000a94c40 as *const (), 0), // +0x3a8c40 sub_3a8c40
    Lifted::new(0x7000a94cd0, fn_7000a94cd0 as *const (), 0), // +0x3a8cd0 sub_3a8cd0
    Lifted::new(0x7000a94db0, fn_7000a94db0 as *const (), 0), // +0x3a8db0 sub_3a8db0
    Lifted::new(0x7000a94e80, fn_7000a94e80 as *const (), 0), // +0x3a8e80 sub_3a8e80
    Lifted::new(0x7000a94f10, fn_7000a94f10 as *const (), 0), // +0x3a8f10 sub_3a8f10
    Lifted::new(0x7000a94fa0, tpl_747::<0x70017d976c, 0x70017d9870, 0x7000a85da0> as *const (), 0), // +0x3a8fa0 sub_3a8fa0
    Lifted::new(0x7000a94ff0, tpl_747::<0x70017d976c, 0x70017d9830, 0x7000a85da0> as *const (), 0), // +0x3a8ff0 sub_3a8ff0
    Lifted::new(0x7000a95040, fn_7000a95040 as *const (), 0), // +0x3a9040 sub_3a9040
    Lifted::new(0x7000a956f0, tpl_747::<0x70017d976c, 0x70017d98b0, 0x7000a85da0> as *const (), 0), // +0x3a96f0 sub_3a96f0
    Lifted::new(0x7000a95740, fn_7000a95740 as *const (), 2), // +0x3a9740 sub_3a9740
    Lifted::new(0x7000a958e0, fn_7000a958e0 as *const (), 0), // +0x3a98e0 sub_3a98e0
    Lifted::new(0x7000a95920, fn_7000a95920 as *const (), 2), // +0x3a9920 sub_3a9920
    Lifted::new(0x7000a95b60, fn_7000a95b60 as *const (), 0), // +0x3a9b60 sub_3a9b60
    Lifted::new(0x7000a95c10, fn_7000a95c10 as *const (), 0), // +0x3a9c10 sub_3a9c10
    Lifted::new(0x7000a95cd0, fn_7000a95cd0 as *const (), 0), // +0x3a9cd0 sub_3a9cd0
    Lifted::new(0x7000a95d90, tpl_747::<0x70017d976c, 0x70017d97b0, 0x7000a85da0> as *const (), 0), // +0x3a9d90 sub_3a9d90
    Lifted::new(0x7000a95de0, fn_7000a95de0 as *const (), 0), // +0x3a9de0 sub_3a9de0
    Lifted::new(0x7000a95e90, fn_7000a95e90 as *const (), 0), // +0x3a9e90 sub_3a9e90
    Lifted::new(0x7000a95ee0, fn_7000a95ee0 as *const (), 0), // +0x3a9ee0 sub_3a9ee0
    Lifted::new(0x7000a95f30, fn_7000a95f30 as *const (), 0), // +0x3a9f30 sub_3a9f30
    Lifted::new(0x7000a95fe0, fn_7000a95fe0 as *const (), 0), // +0x3a9fe0 sub_3a9fe0
    Lifted::new(0x7000a96040, fn_7000a96040 as *const (), 0), // +0x3aa040 sub_3aa040
    Lifted::new(0x7000a960a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a960b1> as *const (), 0), // +0x3aa0a0 sub_3aa0a0
    Lifted::new(0x7000a960c0, tpl_741::<0x700074f560> as *const (), 0), // +0x3aa0c0 sub_3aa0c0
    Lifted::new(0x7000a96110, tpl_741::<0x700074f560> as *const (), 0), // +0x3aa110 sub_3aa110
    Lifted::new(0x7000a96160, tpl_741::<0x700074f560> as *const (), 0), // +0x3aa160 sub_3aa160
    Lifted::new(0x7000a961b0, tpl_741::<0x700073e490> as *const (), 0), // +0x3aa1b0 sub_3aa1b0
    Lifted::new(0x7000a96200, tpl_741::<0x700073e490> as *const (), 0), // +0x3aa200 sub_3aa200
    Lifted::new(0x7000a96250, tpl_741::<0x700073e490> as *const (), 0), // +0x3aa250 sub_3aa250
    Lifted::new(0x7000a962a0, tpl_741::<0x700075fbd0> as *const (), 0), // +0x3aa2a0 sub_3aa2a0
    Lifted::new(0x7000a962f0, tpl_741::<0x70006fc870> as *const (), 0), // +0x3aa2f0 sub_3aa2f0
    Lifted::new(0x7000a96340, tpl_741::<0x70006fc870> as *const (), 0), // +0x3aa340 sub_3aa340
    Lifted::new(0x7000a96390, tpl_741::<0x70007a2490> as *const (), 0), // +0x3aa390 sub_3aa390
    Lifted::new(0x7000a963e0, tpl_741::<0x70006edce0> as *const (), 0), // +0x3aa3e0 sub_3aa3e0
    Lifted::new(0x7000a96430, tpl_741::<0x70006edce0> as *const (), 0), // +0x3aa430 sub_3aa430
    Lifted::new(0x7000a96480, tpl_752::<0x700071de10> as *const (), 0), // +0x3aa480 sub_3aa480
    Lifted::new(0x7000a964c0, tpl_752::<0x700071de10> as *const (), 0), // +0x3aa4c0 sub_3aa4c0
    Lifted::new(0x7000a96500, tpl_741::<0x7000709d50> as *const (), 0), // +0x3aa500 sub_3aa500
    Lifted::new(0x7000a96550, tpl_741::<0x70007a0b60> as *const (), 0), // +0x3aa550 sub_3aa550
    Lifted::new(0x7000a965a0, tpl_741::<0x70007a0b60> as *const (), 0), // +0x3aa5a0 sub_3aa5a0
    Lifted::new(0x7000a965f0, tpl_741::<0x7000795bd0> as *const (), 0), // +0x3aa5f0 sub_3aa5f0
    Lifted::new(0x7000a96640, tpl_741::<0x700073a8b0> as *const (), 0), // +0x3aa640 sub_3aa640
    Lifted::new(0x7000a96690, tpl_741::<0x700073a8b0> as *const (), 0), // +0x3aa690 sub_3aa690
    Lifted::new(0x7000a966e0, tpl_741::<0x70007384b0> as *const (), 0), // +0x3aa6e0 sub_3aa6e0
    Lifted::new(0x7000a96730, tpl_741::<0x70007384b0> as *const (), 0), // +0x3aa730 sub_3aa730
    Lifted::new(0x7000a96780, tpl_752::<0x700077d520> as *const (), 0), // +0x3aa780 sub_3aa780
    Lifted::new(0x7000a967c0, tpl_741::<0x7000708d30> as *const (), 0), // +0x3aa7c0 sub_3aa7c0
    Lifted::new(0x7000a96810, tpl_752::<0x7000736910> as *const (), 0), // +0x3aa810 sub_3aa810
    Lifted::new(0x7000a96850, tpl_741::<0x7000722e00> as *const (), 0), // +0x3aa850 sub_3aa850
    Lifted::new(0x7000a968a0, tpl_741::<0x70006ff270> as *const (), 0), // +0x3aa8a0 sub_3aa8a0
    Lifted::new(0x7000a968f0, tpl_741::<0x70006ff270> as *const (), 0), // +0x3aa8f0 sub_3aa8f0
    Lifted::new(0x7000a96940, tpl_752::<0x7000746d30> as *const (), 0), // +0x3aa940 sub_3aa940
    Lifted::new(0x7000a96980, tpl_741::<0x700070ecb0> as *const (), 0), // +0x3aa980 sub_3aa980
    Lifted::new(0x7000a969d0, tpl_753::<0x70017d9830, 0x8> as *const (), 1), // +0x3aa9d0 sub_3aa9d0
    Lifted::new(0x7000a969f0, tpl_753::<0x70017d97b0, 0x8> as *const (), 1), // +0x3aa9f0 sub_3aa9f0
    Lifted::new(0x7000a96a10, tpl_753::<0x70017d97f0, 0x8> as *const (), 1), // +0x3aaa10 sub_3aaa10
    Lifted::new(0x7000a96a30, tpl_753::<0x70017d9870, 0x8> as *const (), 1), // +0x3aaa30 sub_3aaa30
    Lifted::new(0x7000a96a50, tpl_753::<0x70017d98f0, 0x8> as *const (), 1), // +0x3aaa50 sub_3aaa50
    Lifted::new(0x7000a96a70, tpl_753::<0x70017d98b0, 0x8> as *const (), 1), // +0x3aaa70 sub_3aaa70
    Lifted::new(0x7000a96a90, tpl_754::<0x700177c312> as *const (), 0), // +0x3aaa90 sub_3aaa90
    Lifted::new(0x7000a96ab0, tpl_754::<0x700177c312> as *const (), 0), // +0x3aaab0 sub_3aaab0
    Lifted::new(0x7000a96ad0, tpl_754::<0x700177c312> as *const (), 0), // +0x3aaad0 sub_3aaad0
    Lifted::new(0x7000a96af0, tpl_754::<0x700177c312> as *const (), 0), // +0x3aaaf0 sub_3aaaf0
    Lifted::new(0x7000a96b10, tpl_754::<0x700177c312> as *const (), 0), // +0x3aab10 sub_3aab10
    Lifted::new(0x7000a96b30, tpl_754::<0x700177c312> as *const (), 0), // +0x3aab30 sub_3aab30
    Lifted::new(0x7000a96b50, fn_7000a96b50 as *const (), 2), // +0x3aab50 sub_3aab50
    Lifted::new(0x7000a96c60, fn_7000a96c60 as *const (), 0), // +0x3aac60 sub_3aac60
    Lifted::new(0x7000a96ce0, fn_7000a96ce0 as *const (), 0), // +0x3aace0 sub_3aace0
    Lifted::new(0x7000a96d50, tpl_755::<0x8, 0x1f0, 0x6, 0x70006ff9d0, 0x700079e7f0> as *const (), 0), // +0x3aad50 sub_3aad50
    Lifted::new(0x7000a96dc0, fn_7000a96dc0 as *const (), 2), // +0x3aadc0 sub_3aadc0
    Lifted::new(0x7000a96f20, tpl_755::<0x8, 0x1f0, 0x4, 0x70006ff9d0, 0x700079e7f0> as *const (), 0), // +0x3aaf20 sub_3aaf20
    Lifted::new(0x7000a96f90, tpl_52::<0x8> as *const (), 1), // +0x3aaf90 sub_3aaf90
    Lifted::new(0x7000a96fa0, tpl_52::<0x8> as *const (), 1), // +0x3aafa0 sub_3aafa0
    Lifted::new(0x7000a96fb0, tpl_52::<0x8> as *const (), 1), // +0x3aafb0 sub_3aafb0
    Lifted::new(0x7000a96fc0, tpl_52::<0x8> as *const (), 1), // +0x3aafc0 sub_3aafc0
    Lifted::new(0x7000a96fd0, tpl_52::<0x8> as *const (), 1), // +0x3aafd0 sub_3aafd0
    Lifted::new(0x7000a96fe0, tpl_52::<0x8> as *const (), 1), // +0x3aafe0 sub_3aafe0
    Lifted::new(0x7000a96ff0, tpl_753::<0x70017d9830, 0x8> as *const (), 1), // +0x3aaff0 sub_3aaff0
    Lifted::new(0x7000a97010, tpl_753::<0x70017d97b0, 0x8> as *const (), 1), // +0x3ab010 sub_3ab010
    Lifted::new(0x7000a97030, tpl_753::<0x70017d97f0, 0x8> as *const (), 1), // +0x3ab030 sub_3ab030
    Lifted::new(0x7000a97050, tpl_753::<0x70017d9870, 0x8> as *const (), 1), // +0x3ab050 sub_3ab050
    Lifted::new(0x7000a97070, tpl_753::<0x70017d98f0, 0x8> as *const (), 1), // +0x3ab070 sub_3ab070
    Lifted::new(0x7000a97090, tpl_753::<0x70017d98b0, 0x8> as *const (), 1), // +0x3ab090 sub_3ab090
    Lifted::new(0x7000a970b0, tpl_315::<0x7001c6b190> as *const (), 1), // +0x3ab0b0 sub_3ab0b0
    Lifted::new(0x7000a970c0, tpl_315::<0x7001c6b230> as *const (), 1), // +0x3ab0c0 sub_3ab0c0
    Lifted::new(0x7000a970d0, tpl_315::<0x7001c6b1e0> as *const (), 1), // +0x3ab0d0 sub_3ab0d0
    Lifted::new(0x7000a970e0, tpl_315::<0x7001c6b140> as *const (), 1), // +0x3ab0e0 sub_3ab0e0
    Lifted::new(0x7000a970f0, tpl_315::<0x7001c6b0a0> as *const (), 1), // +0x3ab0f0 sub_3ab0f0
    Lifted::new(0x7000a97100, tpl_315::<0x7001c6b0f0> as *const (), 1), // +0x3ab100 sub_3ab100
    Lifted::new(0x7000a97110, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a97120> as *const (), 0), // +0x3ab110 sub_3ab110
    Lifted::new(0x7000a97130, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a97140> as *const (), 0), // +0x3ab130 sub_3ab130
    Lifted::new(0x7000a97150, tpl_147::<0x14> as *const (), 1), // +0x3ab150 sub_3ab150
    Lifted::new(0x7000a97160, tpl_39::<0x14> as *const (), 1), // +0x3ab160 sub_3ab160
    Lifted::new(0x7000a97170, fn_7000a97170 as *const (), 1), // +0x3ab170 sub_3ab170
    Lifted::new(0x7000a97190, fn_7000a97190 as *const (), 0), // +0x3ab190 sub_3ab190
    Lifted::new(0x7000a971b0, fn_7000a971b0 as *const (), 0), // +0x3ab1b0 sub_3ab1b0
    Lifted::new(0x7000a971d0, tpl_30 as *const (), 1), // +0x3ab1d0 sub_3ab1d0
    Lifted::new(0x7000a971e0, fn_7000a971e0 as *const (), 0), // +0x3ab1e0 sub_3ab1e0
    Lifted::new(0x7000a97260, tpl_50 as *const (), 1), // +0x3ab260 sub_3ab260
    Lifted::new(0x7000a97270, tpl_43 as *const (), 1), // +0x3ab270 sub_3ab270
    Lifted::new(0x7000a97280, fn_7000a97280 as *const (), 2), // +0x3ab280 sub_3ab280
    Lifted::new(0x7000a97350, fn_7000a97350 as *const (), 0), // +0x3ab350 sub_3ab350
    Lifted::new(0x7000a973a0, fn_7000a973a0 as *const (), 1), // +0x3ab3a0 sub_3ab3a0
    Lifted::new(0x7000a973c0, fn_7000a973c0 as *const (), 1), // +0x3ab3c0 sub_3ab3c0
    Lifted::new(0x7000a973f0, tpl_756::<0x20> as *const (), 1), // +0x3ab3f0 sub_3ab3f0
    Lifted::new(0x7000a97410, fn_7000a97410 as *const (), 1), // +0x3ab410 sub_3ab410
    Lifted::new(0x7000a97430, tpl_757::<0x20> as *const (), 1), // +0x3ab430 sub_3ab430
    Lifted::new(0x7000a97460, tpl_757::<0x20> as *const (), 1), // +0x3ab460 sub_3ab460
    Lifted::new(0x7000a97490, tpl_756::<0x20> as *const (), 1), // +0x3ab490 sub_3ab490
    Lifted::new(0x7000a974b0, fn_7000a974b0 as *const (), 1), // +0x3ab4b0 sub_3ab4b0
    Lifted::new(0x7000a974d0, tpl_757::<0x20> as *const (), 1), // +0x3ab4d0 sub_3ab4d0
    Lifted::new(0x7000a97500, fn_7000a97500 as *const (), 1), // +0x3ab500 sub_3ab500
    Lifted::new(0x7000a97520, fn_7000a97520 as *const (), 3), // +0x3ab520 sub_3ab520
    Lifted::new(0x7000a97590, fn_7000a97590 as *const (), 1), // +0x3ab590 sub_3ab590
    Lifted::new(0x7000a975b0, fn_7000a975b0 as *const (), 1), // +0x3ab5b0 sub_3ab5b0
    Lifted::new(0x7000a975d0, fn_7000a975d0 as *const (), 1), // +0x3ab5d0 sub_3ab5d0
    Lifted::new(0x7000a975f0, fn_7000a975f0 as *const (), 1), // +0x3ab5f0 sub_3ab5f0
    Lifted::new(0x7000a97610, fn_7000a97610 as *const (), 1), // +0x3ab610 sub_3ab610
    Lifted::new(0x7000a97630, fn_7000a97630 as *const (), 1), // +0x3ab630 sub_3ab630
    Lifted::new(0x7000a97650, fn_7000a97650 as *const (), 1), // +0x3ab650 sub_3ab650
    Lifted::new(0x7000a97670, fn_7000a97670 as *const (), 1), // +0x3ab670 sub_3ab670
    Lifted::new(0x7000a97690, tpl_758::<0x20> as *const (), 1), // +0x3ab690 sub_3ab690
    Lifted::new(0x7000a976b0, tpl_758::<0x20> as *const (), 1), // +0x3ab6b0 sub_3ab6b0
    Lifted::new(0x7000a976d0, fn_7000a976d0 as *const (), 1), // +0x3ab6d0 sub_3ab6d0
    Lifted::new(0x7000a976f0, tpl_758::<0x20> as *const (), 1), // +0x3ab6f0 sub_3ab6f0
    Lifted::new(0x7000a97710, fn_7000a97710 as *const (), 0), // +0x3ab710 sub_3ab710
    Lifted::new(0x7000a977e0, fn_7000a977e0 as *const (), 1), // +0x3ab7e0 sub_3ab7e0
    Lifted::new(0x7000a97810, fn_7000a97810 as *const (), 1), // +0x3ab810 sub_3ab810
    Lifted::new(0x7000a97840, fn_7000a97840 as *const (), 1), // +0x3ab840 sub_3ab840
    Lifted::new(0x7000a97870, fn_7000a97870 as *const (), 0), // +0x3ab870 sub_3ab870
    Lifted::new(0x7000a978b0, fn_7000a978b0 as *const (), 3), // +0x3ab8b0 sub_3ab8b0
    Lifted::new(0x7000a97990, fn_7000a97990 as *const (), 1), // +0x3ab990 sub_3ab990
    Lifted::new(0x7000a97a50, fn_7000a97a50 as *const (), 3), // +0x3aba50 sub_3aba50
    Lifted::new(0x7000a97b30, fn_7000a97b30 as *const (), 1), // +0x3abb30 sub_3abb30
    Lifted::new(0x7000a97b40, fn_7000a97b40 as *const (), 1), // +0x3abb40 sub_3abb40
    Lifted::new(0x7000a97b50, fn_7000a97b50 as *const (), 1), // +0x3abb50 sub_3abb50
    Lifted::new(0x7000a97b60, tpl_759 as *const (), 1), // +0x3abb60 sub_3abb60
    Lifted::new(0x7000a97b70, tpl_759 as *const (), 1), // +0x3abb70 sub_3abb70
    Lifted::new(0x7000a97b80, tpl_760 as *const (), 1), // +0x3abb80 sub_3abb80
    Lifted::new(0x7000a97b90, tpl_760 as *const (), 1), // +0x3abb90 sub_3abb90
    Lifted::new(0x7000a97ba0, tpl_759 as *const (), 1), // +0x3abba0 sub_3abba0
    Lifted::new(0x7000a97bb0, tpl_759 as *const (), 1), // +0x3abbb0 sub_3abbb0
    Lifted::new(0x7000a97bc0, fn_7000a97bc0 as *const (), 1), // +0x3abbc0 sub_3abbc0
    Lifted::new(0x7000a97bd0, tpl_185::<0xc> as *const (), 1), // +0x3abbd0 sub_3abbd0
    Lifted::new(0x7000a97be0, tpl_349 as *const (), 1), // +0x3abbe0 sub_3abbe0
    Lifted::new(0x7000a97bf0, tpl_172::<0x8> as *const (), 1), // +0x3abbf0 sub_3abbf0
    Lifted::new(0x7000a97c00, tpl_761 as *const (), 1), // +0x3abc00 sub_3abc00
    Lifted::new(0x7000a97c10, tpl_762 as *const (), 1), // +0x3abc10 sub_3abc10
    Lifted::new(0x7000a97c20, fn_7000a97c20 as *const (), 1), // +0x3abc20 sub_3abc20
    Lifted::new(0x7000a97c30, fn_7000a97c30 as *const (), 1), // +0x3abc30 sub_3abc30
    Lifted::new(0x7000a97c40, fn_7000a97c40 as *const (), 1), // +0x3abc40 sub_3abc40
    Lifted::new(0x7000a97c50, fn_7000a97c50 as *const (), 1), // +0x3abc50 sub_3abc50
    Lifted::new(0x7000a97c60, tpl_36::<0x10> as *const (), 1), // +0x3abc60 sub_3abc60
    Lifted::new(0x7000a97c70, tpl_39::<0x8> as *const (), 1), // +0x3abc70 sub_3abc70
    Lifted::new(0x7000a97c80, fn_7000a97c80 as *const (), 1), // +0x3abc80 sub_3abc80
    Lifted::new(0x7000a97ca0, fn_7000a97ca0 as *const (), 1), // +0x3abca0 sub_3abca0
    Lifted::new(0x7000a97cc0, tpl_41 as *const (), 1), // +0x3abcc0 sub_3abcc0
    Lifted::new(0x7000a97cd0, tpl_763::<0x10> as *const (), 3), // +0x3abcd0 sub_3abcd0
    Lifted::new(0x7000a97d20, tpl_763::<0x10> as *const (), 3), // +0x3abd20 sub_3abd20
    Lifted::new(0x7000a97d70, tpl_763::<0x10> as *const (), 3), // +0x3abd70 sub_3abd70
    Lifted::new(0x7000a97dc0, tpl_763::<0x10> as *const (), 3), // +0x3abdc0 sub_3abdc0
    Lifted::new(0x7000a97e10, tpl_763::<0x10> as *const (), 3), // +0x3abe10 sub_3abe10
    Lifted::new(0x7000a97e60, tpl_763::<0x10> as *const (), 3), // +0x3abe60 sub_3abe60
    Lifted::new(0x7000a97eb0, fn_7000a97eb0 as *const (), 1), // +0x3abeb0 sub_3abeb0
    Lifted::new(0x7000a97f10, tpl_144 as *const (), 1), // +0x3abf10 sub_3abf10
    Lifted::new(0x7000a97f20, fn_7000a97f20 as *const (), 1), // +0x3abf20 sub_3abf20
    Lifted::new(0x7000a97f30, tpl_39::<0x4> as *const (), 1), // +0x3abf30 sub_3abf30
    Lifted::new(0x7000a97f40, fn_7000a97f40 as *const (), 1), // +0x3abf40 sub_3abf40
    Lifted::new(0x7000a97f60, fn_7000a97f60 as *const (), 1), // +0x3abf60 sub_3abf60
    Lifted::new(0x7000a97f70, fn_7000a97f70 as *const (), 0), // +0x3abf70 sub_3abf70
    Lifted::new(0x7000a97fa0, tpl_39::<0x10> as *const (), 1), // +0x3abfa0 sub_3abfa0
    Lifted::new(0x7000a97fb0, tpl_30 as *const (), 1), // +0x3abfb0 sub_3abfb0
    Lifted::new(0x7000a97fc0, fn_7000a97fc0 as *const (), 1), // +0x3abfc0 sub_3abfc0
    Lifted::new(0x7000a97fd0, fn_7000a97fd0 as *const (), 0), // +0x3abfd0 sub_3abfd0
    Lifted::new(0x7000a98080, fn_7000a98080 as *const (), 0), // +0x3ac080 sub_3ac080
    Lifted::new(0x7000a98190, fn_7000a98190 as *const (), 2), // +0x3ac190 sub_3ac190
    Lifted::new(0x7000a98630, fn_7000a98630 as *const (), 2), // +0x3ac630 sub_3ac630
    Lifted::new(0x7000a989b0, fn_7000a989b0 as *const (), 0), // +0x3ac9b0 sub_3ac9b0
    Lifted::new(0x7000a98a70, fn_7000a98a70 as *const (), 0), // +0x3aca70 sub_3aca70
    Lifted::new(0x7000a98ae0, fn_7000a98ae0 as *const (), 0), // +0x3acae0 sub_3acae0
    Lifted::new(0x7000a98b20, fn_7000a98b20 as *const (), 1), // +0x3acb20 sub_3acb20
    Lifted::new(0x7000a98bc0, fn_7000a98bc0 as *const (), 1), // +0x3acbc0 sub_3acbc0
    Lifted::new(0x7000a98be0, fn_7000a98be0 as *const (), 0), // +0x3acbe0 sub_3acbe0
    Lifted::new(0x7000a98ce0, fn_7000a98ce0 as *const (), 0), // +0x3acce0 sub_3acce0
    Lifted::new(0x7000a98ea0, fn_7000a98ea0 as *const (), 2), // +0x3acea0 sub_3acea0
    Lifted::new(0x7000a99260, fn_7000a99260 as *const (), 0), // +0x3ad260 sub_3ad260
    Lifted::new(0x7000a993f0, fn_7000a993f0 as *const (), 2), // +0x3ad3f0 sub_3ad3f0
    Lifted::new(0x7000a99570, fn_7000a99570 as *const (), 2), // +0x3ad570 sub_3ad570
    Lifted::new(0x7000a999f0, fn_7000a999f0 as *const (), 2), // +0x3ad9f0 sub_3ad9f0
    Lifted::new(0x7000a99bb0, fn_7000a99bb0 as *const (), 2), // +0x3adbb0 sub_3adbb0
    Lifted::new(0x7000a99cf0, fn_7000a99cf0 as *const (), 0), // +0x3adcf0 sub_3adcf0
    Lifted::new(0x7000a99dc0, fn_7000a99dc0 as *const (), 0), // +0x3addc0 sub_3addc0
    Lifted::new(0x7000a9a040, fn_7000a9a040 as *const (), 0), // +0x3ae040 sub_3ae040
    Lifted::new(0x7000a9a120, fn_7000a9a120 as *const (), 0), // +0x3ae120 sub_3ae120
    Lifted::new(0x7000a9a300, fn_7000a9a300 as *const (), 2), // +0x3ae300 sub_3ae300
    Lifted::new(0x7000a9a8e0, fn_7000a9a8e0 as *const (), 0), // +0x3ae8e0 sub_3ae8e0
    Lifted::new(0x7000a9ab70, fn_7000a9ab70 as *const (), 0), // +0x3aeb70 sub_3aeb70
    Lifted::new(0x7000a9ad50, fn_7000a9ad50 as *const (), 2), // +0x3aed50 sub_3aed50
    Lifted::new(0x7000a9b0b0, fn_7000a9b0b0 as *const (), 0), // +0x3af0b0 sub_3af0b0
    Lifted::new(0x7000a9b1a0, fn_7000a9b1a0 as *const (), 0), // +0x3af1a0 sub_3af1a0
    Lifted::new(0x7000a9b280, fn_7000a9b280 as *const (), 2), // +0x3af280 sub_3af280
    Lifted::new(0x7000a9b420, fn_7000a9b420 as *const (), 0), // +0x3af420 sub_3af420
    Lifted::new(0x7000a9b4f0, fn_7000a9b4f0 as *const (), 2), // +0x3af4f0 sub_3af4f0
    Lifted::new(0x7000a9b630, fn_7000a9b630 as *const (), 2), // +0x3af630 sub_3af630
    Lifted::new(0x7000a9b770, fn_7000a9b770 as *const (), 2), // +0x3af770 sub_3af770
    Lifted::new(0x7000a9ba70, fn_7000a9ba70 as *const (), 0), // +0x3afa70 sub_3afa70
    Lifted::new(0x7000a9bcd0, fn_7000a9bcd0 as *const (), 0), // +0x3afcd0 sub_3afcd0
    Lifted::new(0x7000a9bd90, fn_7000a9bd90 as *const (), 2), // +0x3afd90 sub_3afd90
    Lifted::new(0x7000a9c2b0, tpl_764::<0xcb8, 0x7000705900, 0x7000753d50> as *const (), 0), // +0x3b02b0 sub_3b02b0
    Lifted::new(0x7000a9c2f0, tpl_765::<0x70006fcf60, 0x818, 0x70006f5460, 0x7000753d50> as *const (), 0), // +0x3b02f0 sub_3b02f0
    Lifted::new(0x7000a9c330, tpl_765::<0x7000743310, 0x818, 0x7000729200, 0x7000753d50> as *const (), 0), // +0x3b0330 sub_3b0330
    Lifted::new(0x7000a9c370, tpl_765::<0x7000743310, 0x890, 0x7000735560, 0x7000753d50> as *const (), 0), // +0x3b0370 sub_3b0370
    Lifted::new(0x7000a9c3b0, tpl_765::<0x700075ffc0, 0xa68, 0x7000738c70, 0x7000753d50> as *const (), 0), // +0x3b03b0 sub_3b03b0
    Lifted::new(0x7000a9c3f0, tpl_764::<0x4f0, 0x70006faaa0, 0x7000753d50> as *const (), 0), // +0x3b03f0 sub_3b03f0
    Lifted::new(0x7000a9c430, tpl_764::<0x5b8, 0x7000765d20, 0x7000753d50> as *const (), 0), // +0x3b0430 sub_3b0430
    Lifted::new(0x7000a9c470, tpl_765::<0x7000743310, 0x818, 0x700079a280, 0x7000753d50> as *const (), 0), // +0x3b0470 sub_3b0470
    Lifted::new(0x7000a9c4b0, tpl_765::<0x7000743310, 0x890, 0x70006f6520, 0x7000753d50> as *const (), 0), // +0x3b04b0 sub_3b04b0
    Lifted::new(0x7000a9c4f0, tpl_765::<0x7000762d10, 0x980, 0x700079cdd0, 0x7000753d50> as *const (), 0), // +0x3b04f0 sub_3b04f0
    Lifted::new(0x7000a9c530, fn_7000a9c530 as *const (), 0), // +0x3b0530 sub_3b0530
    Lifted::new(0x7000a9c570, tpl_764::<0x818, 0x7000724700, 0x7000753d50> as *const (), 0), // +0x3b0570 sub_3b0570
    Lifted::new(0x7000a9c5b0, fn_7000a9c5b0 as *const (), 0), // +0x3b05b0 sub_3b05b0
    Lifted::new(0x7000a9c680, fn_7000a9c680 as *const (), 0), // +0x3b0680 sub_3b0680
    Lifted::new(0x7000a9c7c0, fn_7000a9c7c0 as *const (), 2), // +0x3b07c0 sub_3b07c0
    Lifted::new(0x7000a9c940, fn_7000a9c940 as *const (), 0), // +0x3b0940 sub_3b0940
    Lifted::new(0x7000a9cb50, fn_7000a9cb50 as *const (), 2), // +0x3b0b50 sub_3b0b50
    Lifted::new(0x7000a9cd10, fn_7000a9cd10 as *const (), 0), // +0x3b0d10 sub_3b0d10
    Lifted::new(0x7000a9cdc0, fn_7000a9cdc0 as *const (), 0), // +0x3b0dc0 sub_3b0dc0
    Lifted::new(0x7000a9d190, fn_7000a9d190 as *const (), 0), // +0x3b1190 sub_3b1190
    Lifted::new(0x7000a9d5f0, tpl_134::<0x8, 0x70017d2c68, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x3b15f0 sub_3b15f0
    Lifted::new(0x7000a9d640, tpl_766::<0x7001c73174, 0x7000a9d950, 0x7000a9d750> as *const (), 0), // +0x3b1640 sub_3b1640
    Lifted::new(0x7000a9d690, fn_7000a9d690 as *const (), 0), // +0x3b1690 sub_3b1690
    Lifted::new(0x7000a9d6f0, fn_7000a9d6f0 as *const (), 0), // +0x3b16f0 sub_3b16f0
    Lifted::new(0x7000a9d750, fn_7000a9d750 as *const (), 2), // +0x3b1750 sub_3b1750
    Lifted::new(0x7000a9d950, fn_7000a9d950 as *const (), 2), // +0x3b1950 sub_3b1950
    Lifted::new(0x7000a9dbb0, fn_7000a9dbb0 as *const (), 2), // +0x3b1bb0 sub_3b1bb0
    Lifted::new(0x7000a9de20, fn_7000a9de20 as *const (), 2), // +0x3b1e20 sub_3b1e20
    Lifted::new(0x7000a9e090, fn_7000a9e090 as *const (), 2), // +0x3b2090 sub_3b2090
    Lifted::new(0x7000a9e2a0, fn_7000a9e2a0 as *const (), 2), // +0x3b22a0 sub_3b22a0
    Lifted::new(0x7000a9e450, fn_7000a9e450 as *const (), 0), // +0x3b2450 sub_3b2450
    Lifted::new(0x7000a9e780, fn_7000a9e780 as *const (), 2), // +0x3b2780 sub_3b2780
    Lifted::new(0x7000a9eb50, fn_7000a9eb50 as *const (), 0), // +0x3b2b50 sub_3b2b50
    Lifted::new(0x7000a9ebc0, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x3b2bc0 sub_3b2bc0
    Lifted::new(0x7000a9ecd0, fn_7000a9ecd0 as *const (), 2), // +0x3b2cd0 sub_3b2cd0
    Lifted::new(0x7000a9ede0, fn_7000a9ede0 as *const (), 1), // +0x3b2de0 sub_3b2de0
    Lifted::new(0x7000a9ef80, fn_7000a9ef80 as *const (), 0), // +0x3b2f80 sub_3b2f80
    Lifted::new(0x7000a9f490, tpl_20 as *const (), 1), // +0x3b3490 sub_3b3490
    Lifted::new(0x7000a9f4a0, fn_7000a9f4a0 as *const (), 0), // +0x3b34a0 sub_3b34a0
    Lifted::new(0x7000a9f5b0, tpl_766::<0x7001c73174, 0x7000aa0a80, 0x7000a9f8c0> as *const (), 0), // +0x3b35b0 sub_3b35b0
    Lifted::new(0x7000a9f600, tpl_766::<0x7001c73174, 0x7000aa0ce0, 0x7000a9fca0> as *const (), 0), // +0x3b3600 sub_3b3600
    Lifted::new(0x7000a9f650, tpl_766::<0x7001c73174, 0x7000aa0f40, 0x7000a9fff0> as *const (), 0), // +0x3b3650 sub_3b3650
    Lifted::new(0x7000a9f6a0, tpl_766::<0x7001c73174, 0x7000aa11a0, 0x7000aa0560> as *const (), 0), // +0x3b36a0 sub_3b36a0
    Lifted::new(0x7000a9f6f0, tpl_766::<0x7001c73174, 0x7000aa1400, 0x7000aa07d0> as *const (), 0), // +0x3b36f0 sub_3b36f0
    Lifted::new(0x7000a9f740, fn_7000a9f740 as *const (), 0), // +0x3b3740 sub_3b3740
    Lifted::new(0x7000a9f8c0, fn_7000a9f8c0 as *const (), 0), // +0x3b38c0 sub_3b38c0
    Lifted::new(0x7000a9fca0, fn_7000a9fca0 as *const (), 2), // +0x3b3ca0 sub_3b3ca0
    Lifted::new(0x7000a9fff0, fn_7000a9fff0 as *const (), 2), // +0x3b3ff0 sub_3b3ff0
    Lifted::new(0x7000aa0560, fn_7000aa0560 as *const (), 2), // +0x3b4560 sub_3b4560
    Lifted::new(0x7000aa07d0, fn_7000aa07d0 as *const (), 2), // +0x3b47d0 sub_3b47d0
    Lifted::new(0x7000aa0a80, fn_7000aa0a80 as *const (), 2), // +0x3b4a80 sub_3b4a80
    Lifted::new(0x7000aa0ce0, fn_7000aa0ce0 as *const (), 2), // +0x3b4ce0 sub_3b4ce0
    Lifted::new(0x7000aa0f40, fn_7000aa0f40 as *const (), 2), // +0x3b4f40 sub_3b4f40
    Lifted::new(0x7000aa11a0, fn_7000aa11a0 as *const (), 2), // +0x3b51a0 sub_3b51a0
    Lifted::new(0x7000aa1400, fn_7000aa1400 as *const (), 2), // +0x3b5400 sub_3b5400
    Lifted::new(0x7000aa1660, fn_7000aa1660 as *const (), 2), // +0x3b5660 sub_3b5660
    Lifted::new(0x7000aa19d0, fn_7000aa19d0 as *const (), 2), // +0x3b59d0 sub_3b59d0
    Lifted::new(0x7000aa2070, fn_7000aa2070 as *const (), 0), // +0x3b6070 sub_3b6070
    Lifted::new(0x7000aa2190, fn_7000aa2190 as *const (), 2), // +0x3b6190 sub_3b6190
    Lifted::new(0x7000aa2430, fn_7000aa2430 as *const (), 0), // +0x3b6430 sub_3b6430
    Lifted::new(0x7000aa2790, fn_7000aa2790 as *const (), 2), // +0x3b6790 sub_3b6790
    Lifted::new(0x7000aa2af0, fn_7000aa2af0 as *const (), 2), // +0x3b6af0 sub_3b6af0
    Lifted::new(0x7000aa2f60, fn_7000aa2f60 as *const (), 2), // +0x3b6f60 sub_3b6f60
    Lifted::new(0x7000aa3530, fn_7000aa3530 as *const (), 2), // +0x3b7530 sub_3b7530
    Lifted::new(0x7000aa3640, fn_7000aa3640 as *const (), 2), // +0x3b7640 sub_3b7640
    Lifted::new(0x7000aa3790, fn_7000aa3790 as *const (), 0), // +0x3b7790 sub_3b7790
    Lifted::new(0x7000aa3950, tpl_767::<0xff0, 0xb> as *const (), 1), // +0x3b7950 sub_3b7950
    Lifted::new(0x7000aa3980, tpl_767::<0xfa0, 0xb> as *const (), 1), // +0x3b7980 sub_3b7980
    Lifted::new(0x7000aa39b0, tpl_767::<0xfe8, 0xb> as *const (), 1), // +0x3b79b0 sub_3b79b0
    Lifted::new(0x7000aa39e0, tpl_767::<0xfc0, 0xb> as *const (), 1), // +0x3b79e0 sub_3b79e0
    Lifted::new(0x7000aa3a10, fn_7000aa3a10 as *const (), 0), // +0x3b7a10 sub_3b7a10
    Lifted::new(0x7000aa3c80, tpl_767::<0xfb8, 0xb> as *const (), 1), // +0x3b7c80 sub_3b7c80
    Lifted::new(0x7000aa3cb0, tpl_767::<0x1008, 0xb> as *const (), 1), // +0x3b7cb0 sub_3b7cb0
    Lifted::new(0x7000aa3ce0, tpl_767::<0xfb0, 0xb> as *const (), 1), // +0x3b7ce0 sub_3b7ce0
    Lifted::new(0x7000aa3d10, tpl_767::<0xfa8, 0xb> as *const (), 1), // +0x3b7d10 sub_3b7d10
    Lifted::new(0x7000aa3d40, tpl_767::<0xff8, 0xb> as *const (), 1), // +0x3b7d40 sub_3b7d40
    Lifted::new(0x7000aa3d70, tpl_767::<0x1000, 0xb> as *const (), 1), // +0x3b7d70 sub_3b7d70
    Lifted::new(0x7000aa3da0, tpl_767::<0xfd0, 0xb> as *const (), 1), // +0x3b7da0 sub_3b7da0
    Lifted::new(0x7000aa3dd0, tpl_767::<0x1010, 0xb> as *const (), 1), // +0x3b7dd0 sub_3b7dd0
    Lifted::new(0x7000aa3e00, tpl_767::<0xfd8, 0xb> as *const (), 1), // +0x3b7e00 sub_3b7e00
    Lifted::new(0x7000aa3e30, tpl_767::<0x1018, 0xb> as *const (), 1), // +0x3b7e30 sub_3b7e30
    Lifted::new(0x7000aa3e60, tpl_767::<0x1020, 0xb> as *const (), 1), // +0x3b7e60 sub_3b7e60
    Lifted::new(0x7000aa3e90, tpl_767::<0xfe0, 0xb> as *const (), 1), // +0x3b7e90 sub_3b7e90
    Lifted::new(0x7000aa3ec0, tpl_767::<0xfc8, 0xb> as *const (), 1), // +0x3b7ec0 sub_3b7ec0
    Lifted::new(0x7000aa3ef0, fn_7000aa3ef0 as *const (), 2), // +0x3b7ef0 sub_3b7ef0
    Lifted::new(0x7000aa4e80, fn_7000aa4e80 as *const (), 2), // +0x3b8e80 sub_3b8e80
    Lifted::new(0x7000aa53f0, fn_7000aa53f0 as *const (), 0), // +0x3b93f0 sub_3b93f0
    Lifted::new(0x7000aa5570, fn_7000aa5570 as *const (), 1), // +0x3b9570 sub_3b9570
    Lifted::new(0x7000aa5630, fn_7000aa5630 as *const (), 0), // +0x3b9630 sub_3b9630
    Lifted::new(0x7000aa59a0, fn_7000aa59a0 as *const (), 2), // +0x3b99a0 sub_3b99a0
    Lifted::new(0x7000aa5ad0, fn_7000aa5ad0 as *const (), 2), // +0x3b9ad0 sub_3b9ad0
    Lifted::new(0x7000aa68a0, fn_7000aa68a0 as *const (), 2), // +0x3ba8a0 sub_3ba8a0
    Lifted::new(0x7000aa6b10, fn_7000aa6b10 as *const (), 2), // +0x3bab10 sub_3bab10
    Lifted::new(0x7000aa6de0, fn_7000aa6de0 as *const (), 0), // +0x3bade0 sub_3bade0
    Lifted::new(0x7000aa6e30, fn_7000aa6e30 as *const (), 0), // +0x3bae30 sub_3bae30
    Lifted::new(0x7000aa70c0, tpl_766::<0x7001c73174, 0x7000aa7b30, 0x7000aa7220> as *const (), 0), // +0x3bb0c0 sub_3bb0c0
    Lifted::new(0x7000aa7110, tpl_766::<0x7001c73174, 0x7000aa7d90, 0x7000aa75d0> as *const (), 0), // +0x3bb110 sub_3bb110
    Lifted::new(0x7000aa7160, tpl_768::<0x7001c73174, 0x7000aa7ff0, 0x70017d9f58, 0x7000aa91c0> as *const (), 0), // +0x3bb160 sub_3bb160
    Lifted::new(0x7000aa71c0, tpl_768::<0x7001c73174, 0x7000aa8260, 0x70017d9e48, 0x7000aa9ac0> as *const (), 0), // +0x3bb1c0 sub_3bb1c0
    Lifted::new(0x7000aa7220, fn_7000aa7220 as *const (), 2), // +0x3bb220 sub_3bb220
    Lifted::new(0x7000aa75d0, fn_7000aa75d0 as *const (), 0), // +0x3bb5d0 sub_3bb5d0
    Lifted::new(0x7000aa77b0, fn_7000aa77b0 as *const (), 2), // +0x3bb7b0 sub_3bb7b0
    Lifted::new(0x7000aa7b30, fn_7000aa7b30 as *const (), 2), // +0x3bbb30 sub_3bbb30
    Lifted::new(0x7000aa7d90, fn_7000aa7d90 as *const (), 2), // +0x3bbd90 sub_3bbd90
    Lifted::new(0x7000aa7ff0, fn_7000aa7ff0 as *const (), 2), // +0x3bbff0 sub_3bbff0
    Lifted::new(0x7000aa8260, fn_7000aa8260 as *const (), 2), // +0x3bc260 sub_3bc260
    Lifted::new(0x7000aa84d0, fn_7000aa84d0 as *const (), 2), // +0x3bc4d0 sub_3bc4d0
    Lifted::new(0x7000aa8730, fn_7000aa8730 as *const (), 2), // +0x3bc730 sub_3bc730
    Lifted::new(0x7000aa89a0, fn_7000aa89a0 as *const (), 2), // +0x3bc9a0 sub_3bc9a0
    Lifted::new(0x7000aa8c10, tpl_766::<0x7001c73174, 0x7000aa84d0, 0x7000aa77b0> as *const (), 0), // +0x3bcc10 sub_3bcc10
    Lifted::new(0x7000aa8c60, tpl_769::<0x7001c73174, 0x7000aa8730, 0x70017d9fb8, 0x7000aa91c0> as *const (), 0), // +0x3bcc60 sub_3bcc60
    Lifted::new(0x7000aa8cc0, tpl_769::<0x7001c73174, 0x7000aa89a0, 0x70017d9df0, 0x7000aa9ac0> as *const (), 0), // +0x3bccc0 sub_3bccc0
    Lifted::new(0x7000aa8d20, fn_7000aa8d20 as *const (), 2), // +0x3bcd20 sub_3bcd20
    Lifted::new(0x7000aa91c0, fn_7000aa91c0 as *const (), 2), // +0x3bd1c0 sub_3bd1c0
    Lifted::new(0x7000aa9ac0, fn_7000aa9ac0 as *const (), 2), // +0x3bdac0 sub_3bdac0
    Lifted::new(0x7000aaa760, fn_7000aaa760 as *const (), 3), // +0x3be760 sub_3be760
    Lifted::new(0x7000aaa7f0, fn_7000aaa7f0 as *const (), 0), // +0x3be7f0 sub_3be7f0
    Lifted::new(0x7000aaa8c0, fn_7000aaa8c0 as *const (), 0), // +0x3be8c0 sub_3be8c0
    Lifted::new(0x7000aaa9a0, fn_7000aaa9a0 as *const (), 2), // +0x3be9a0 sub_3be9a0
    Lifted::new(0x7000aaaca0, fn_7000aaaca0 as *const (), 2), // +0x3beca0 sub_3beca0
    Lifted::new(0x7000aaafa0, fn_7000aaafa0 as *const (), 2), // +0x3befa0 sub_3befa0
    Lifted::new(0x7000aab120, tpl_766::<0x7001c73174, 0x7000aaba90, 0x7000aab4e0> as *const (), 0), // +0x3bf120 sub_3bf120
    Lifted::new(0x7000aab170, tpl_766::<0x7001c73174, 0x7000aabcf0, 0x7000aab6a0> as *const (), 0), // +0x3bf170 sub_3bf170
    Lifted::new(0x7000aab1c0, tpl_766::<0x7001c73174, 0x7000aabf50, 0x7000aab870> as *const (), 0), // +0x3bf1c0 sub_3bf1c0
    Lifted::new(0x7000aab210, fn_7000aab210 as *const (), 0), // +0x3bf210 sub_3bf210
    Lifted::new(0x7000aab2f0, fn_7000aab2f0 as *const (), 0), // +0x3bf2f0 sub_3bf2f0
    Lifted::new(0x7000aab3f0, fn_7000aab3f0 as *const (), 0), // +0x3bf3f0 sub_3bf3f0
    Lifted::new(0x7000aab4e0, fn_7000aab4e0 as *const (), 2), // +0x3bf4e0 sub_3bf4e0
    Lifted::new(0x7000aab6a0, fn_7000aab6a0 as *const (), 2), // +0x3bf6a0 sub_3bf6a0
    Lifted::new(0x7000aab870, fn_7000aab870 as *const (), 0), // +0x3bf870 sub_3bf870
    Lifted::new(0x7000aaba90, fn_7000aaba90 as *const (), 2), // +0x3bfa90 sub_3bfa90
    Lifted::new(0x7000aabcf0, fn_7000aabcf0 as *const (), 2), // +0x3bfcf0 sub_3bfcf0
    Lifted::new(0x7000aabf50, fn_7000aabf50 as *const (), 2), // +0x3bff50 sub_3bff50
    Lifted::new(0x7000aac1b0, fn_7000aac1b0 as *const (), 2), // +0x3c01b0 sub_3c01b0
    Lifted::new(0x7000aac4e0, fn_7000aac4e0 as *const (), 2), // +0x3c04e0 sub_3c04e0
    Lifted::new(0x7000aac820, fn_7000aac820 as *const (), 2), // +0x3c0820 sub_3c0820
    Lifted::new(0x7000aacb60, fn_7000aacb60 as *const (), 0), // +0x3c0b60 sub_3c0b60
    Lifted::new(0x7000aacd80, tpl_766::<0x7001c73174, 0x7000ab0d50, 0x7000aad410> as *const (), 0), // +0x3c0d80 sub_3c0d80
    Lifted::new(0x7000aacdd0, tpl_766::<0x7001c73174, 0x7000ab0fb0, 0x7000aad6d0> as *const (), 0), // +0x3c0dd0 sub_3c0dd0
    Lifted::new(0x7000aace20, tpl_766::<0x7001c73174, 0x7000ab1210, 0x7000aad990> as *const (), 0), // +0x3c0e20 sub_3c0e20
    Lifted::new(0x7000aace70, tpl_766::<0x7001c73174, 0x7000ab1470, 0x7000aadc50> as *const (), 0), // +0x3c0e70 sub_3c0e70
    Lifted::new(0x7000aacec0, tpl_766::<0x7001c73174, 0x7000ab16d0, 0x7000aadee0> as *const (), 0), // +0x3c0ec0 sub_3c0ec0
    Lifted::new(0x7000aacf10, tpl_766::<0x7001c73174, 0x7000ab1930, 0x7000aae180> as *const (), 0), // +0x3c0f10 sub_3c0f10
    Lifted::new(0x7000aacf60, tpl_766::<0x7001c73174, 0x7000ab1b90, 0x7000aae4a0> as *const (), 0), // +0x3c0f60 sub_3c0f60
    Lifted::new(0x7000aacfb0, tpl_766::<0x7001c73174, 0x7000ab1df0, 0x7000aae730> as *const (), 0), // +0x3c0fb0 sub_3c0fb0
    Lifted::new(0x7000aad000, tpl_766::<0x7001c73174, 0x7000ab2050, 0x7000aae9f0> as *const (), 0), // +0x3c1000 sub_3c1000
    Lifted::new(0x7000aad050, tpl_766::<0x7001c73174, 0x7000ab22b0, 0x7000aaec80> as *const (), 0), // +0x3c1050 sub_3c1050
    Lifted::new(0x7000aad0a0, tpl_766::<0x7001c73174, 0x7000ab2510, 0x7000aaef10> as *const (), 0), // +0x3c10a0 sub_3c10a0
    Lifted::new(0x7000aad0f0, tpl_766::<0x7001c73174, 0x7000ab2770, 0x7000aaf1f0> as *const (), 0), // +0x3c10f0 sub_3c10f0
    Lifted::new(0x7000aad140, tpl_766::<0x7001c73174, 0x7000ab29d0, 0x7000aaf490> as *const (), 0), // +0x3c1140 sub_3c1140
    Lifted::new(0x7000aad190, tpl_766::<0x7001c73174, 0x7000ab2c30, 0x7000aaf7b0> as *const (), 0), // +0x3c1190 sub_3c1190
    Lifted::new(0x7000aad1e0, tpl_766::<0x7001c73174, 0x7000ab2e90, 0x7000aafa40> as *const (), 0), // +0x3c11e0 sub_3c11e0
    Lifted::new(0x7000aad230, tpl_766::<0x7001c73174, 0x7000ab30f0, 0x7000aafcf0> as *const (), 0), // +0x3c1230 sub_3c1230
    Lifted::new(0x7000aad280, tpl_766::<0x7001c73174, 0x7000ab3350, 0x7000aaffa0> as *const (), 0), // +0x3c1280 sub_3c1280
    Lifted::new(0x7000aad2d0, tpl_766::<0x7001c73174, 0x7000ab35b0, 0x7000ab0260> as *const (), 0), // +0x3c12d0 sub_3c12d0
    Lifted::new(0x7000aad320, tpl_766::<0x7001c73174, 0x7000ab3810, 0x7000ab0520> as *const (), 0), // +0x3c1320 sub_3c1320
    Lifted::new(0x7000aad370, tpl_766::<0x7001c73174, 0x7000ab3a70, 0x7000ab07e0> as *const (), 0), // +0x3c1370 sub_3c1370
    Lifted::new(0x7000aad3c0, tpl_766::<0x7001c73174, 0x7000ab3cd0, 0x7000ab0aa0> as *const (), 0), // +0x3c13c0 sub_3c13c0
    Lifted::new(0x7000aad410, fn_7000aad410 as *const (), 0), // +0x3c1410 sub_3c1410
    Lifted::new(0x7000aad6d0, fn_7000aad6d0 as *const (), 0), // +0x3c16d0 sub_3c16d0
    Lifted::new(0x7000aad990, fn_7000aad990 as *const (), 0), // +0x3c1990 sub_3c1990
    Lifted::new(0x7000aadc50, tpl_770::<0x7000710c10, 0x70017da348, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000aadd4c, 0x7000740ce0, 0x7000f1a4b0, 0x7000aaddde, 0x70007a6400, 0x700079cfd0, 0x7000745eb0> as *const (), 0), // +0x3c1c50 sub_3c1c50
    Lifted::new(0x7000aadee0, fn_7000aadee0 as *const (), 0), // +0x3c1ee0 sub_3c1ee0
    Lifted::new(0x7000aae180, fn_7000aae180 as *const (), 0), // +0x3c2180 sub_3c2180
    Lifted::new(0x7000aae4a0, fn_7000aae4a0 as *const (), 0), // +0x3c24a0 sub_3c24a0
    Lifted::new(0x7000aae730, fn_7000aae730 as *const (), 0), // +0x3c2730 sub_3c2730
    Lifted::new(0x7000aae9f0, tpl_770::<0x7000710c10, 0x70017da4a8, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000aaeaec, 0x7000740ce0, 0x7000f1a8d0, 0x7000aaeb7e, 0x70007a6400, 0x700079cfd0, 0x7000745eb0> as *const (), 0), // +0x3c29f0 sub_3c29f0
    Lifted::new(0x7000aaec80, fn_7000aaec80 as *const (), 0), // +0x3c2c80 sub_3c2c80
    Lifted::new(0x7000aaef10, fn_7000aaef10 as *const (), 0), // +0x3c2f10 sub_3c2f10
    Lifted::new(0x7000aaf1f0, fn_7000aaf1f0 as *const (), 0), // +0x3c31f0 sub_3c31f0
    Lifted::new(0x7000aaf490, fn_7000aaf490 as *const (), 0), // +0x3c3490 sub_3c3490
    Lifted::new(0x7000aaf7b0, fn_7000aaf7b0 as *const (), 0), // +0x3c37b0 sub_3c37b0
    Lifted::new(0x7000aafa40, fn_7000aafa40 as *const (), 0), // +0x3c3a40 sub_3c3a40
    Lifted::new(0x7000aafcf0, fn_7000aafcf0 as *const (), 0), // +0x3c3cf0 sub_3c3cf0
    Lifted::new(0x7000aaffa0, fn_7000aaffa0 as *const (), 0), // +0x3c3fa0 sub_3c3fa0
    Lifted::new(0x7000ab0260, fn_7000ab0260 as *const (), 0), // +0x3c4260 sub_3c4260
    Lifted::new(0x7000ab0520, fn_7000ab0520 as *const (), 0), // +0x3c4520 sub_3c4520
    Lifted::new(0x7000ab07e0, fn_7000ab07e0 as *const (), 0), // +0x3c47e0 sub_3c47e0
    Lifted::new(0x7000ab0aa0, fn_7000ab0aa0 as *const (), 0), // +0x3c4aa0 sub_3c4aa0
    Lifted::new(0x7000ab0d50, fn_7000ab0d50 as *const (), 2), // +0x3c4d50 sub_3c4d50
    Lifted::new(0x7000ab0fb0, fn_7000ab0fb0 as *const (), 2), // +0x3c4fb0 sub_3c4fb0
    Lifted::new(0x7000ab1210, fn_7000ab1210 as *const (), 2), // +0x3c5210 sub_3c5210
    Lifted::new(0x7000ab1470, fn_7000ab1470 as *const (), 2), // +0x3c5470 sub_3c5470
    Lifted::new(0x7000ab16d0, fn_7000ab16d0 as *const (), 2), // +0x3c56d0 sub_3c56d0
    Lifted::new(0x7000ab1930, fn_7000ab1930 as *const (), 2), // +0x3c5930 sub_3c5930
    Lifted::new(0x7000ab1b90, fn_7000ab1b90 as *const (), 2), // +0x3c5b90 sub_3c5b90
    Lifted::new(0x7000ab1df0, fn_7000ab1df0 as *const (), 2), // +0x3c5df0 sub_3c5df0
    Lifted::new(0x7000ab2050, fn_7000ab2050 as *const (), 2), // +0x3c6050 sub_3c6050
    Lifted::new(0x7000ab22b0, fn_7000ab22b0 as *const (), 2), // +0x3c62b0 sub_3c62b0
    Lifted::new(0x7000ab2510, fn_7000ab2510 as *const (), 2), // +0x3c6510 sub_3c6510
    Lifted::new(0x7000ab2770, fn_7000ab2770 as *const (), 2), // +0x3c6770 sub_3c6770
    Lifted::new(0x7000ab29d0, fn_7000ab29d0 as *const (), 2), // +0x3c69d0 sub_3c69d0
    Lifted::new(0x7000ab2c30, fn_7000ab2c30 as *const (), 2), // +0x3c6c30 sub_3c6c30
    Lifted::new(0x7000ab2e90, fn_7000ab2e90 as *const (), 2), // +0x3c6e90 sub_3c6e90
    Lifted::new(0x7000ab30f0, fn_7000ab30f0 as *const (), 2), // +0x3c70f0 sub_3c70f0
    Lifted::new(0x7000ab3350, fn_7000ab3350 as *const (), 2), // +0x3c7350 sub_3c7350
    Lifted::new(0x7000ab35b0, fn_7000ab35b0 as *const (), 2), // +0x3c75b0 sub_3c75b0
    Lifted::new(0x7000ab3810, fn_7000ab3810 as *const (), 2), // +0x3c7810 sub_3c7810
    Lifted::new(0x7000ab3a70, fn_7000ab3a70 as *const (), 2), // +0x3c7a70 sub_3c7a70
    Lifted::new(0x7000ab3cd0, fn_7000ab3cd0 as *const (), 2), // +0x3c7cd0 sub_3c7cd0
    Lifted::new(0x7000ab3f30, fn_7000ab3f30 as *const (), 0), // +0x3c7f30 sub_3c7f30
    Lifted::new(0x7000ab40d0, fn_7000ab40d0 as *const (), 0), // +0x3c80d0 sub_3c80d0
    Lifted::new(0x7000ab4270, fn_7000ab4270 as *const (), 2), // +0x3c8270 sub_3c8270
    Lifted::new(0x7000ab44d0, fn_7000ab44d0 as *const (), 2), // +0x3c84d0 sub_3c84d0
    Lifted::new(0x7000ab4730, tpl_766::<0x7001c73174, 0x7000ab4270, 0x7000ab3f30> as *const (), 0), // +0x3c8730 sub_3c8730
    Lifted::new(0x7000ab4780, tpl_766::<0x7001c73174, 0x7000ab44d0, 0x7000ab40d0> as *const (), 0), // +0x3c8780 sub_3c8780
    Lifted::new(0x7000ab47d0, tpl_260::<0x8> as *const (), 0), // +0x3c87d0 sub_3c87d0
    Lifted::new(0x7000ab47e0, tpl_771 as *const (), 0), // +0x3c87e0 sub_3c87e0
    Lifted::new(0x7000ab47f0, tpl_260::<0x10> as *const (), 0), // +0x3c87f0 sub_3c87f0
    Lifted::new(0x7000ab4800, tpl_260::<0x18> as *const (), 0), // +0x3c8800 sub_3c8800
    Lifted::new(0x7000ab4810, tpl_260::<0x20> as *const (), 0), // +0x3c8810 sub_3c8810
    Lifted::new(0x7000ab4820, tpl_260::<0x28> as *const (), 0), // +0x3c8820 sub_3c8820
    Lifted::new(0x7000ab4830, tpl_260::<0x30> as *const (), 0), // +0x3c8830 sub_3c8830
    Lifted::new(0x7000ab4840, tpl_260::<0x38> as *const (), 0), // +0x3c8840 sub_3c8840
    Lifted::new(0x7000ab4850, tpl_260::<0x40> as *const (), 0), // +0x3c8850 sub_3c8850
    Lifted::new(0x7000ab4860, tpl_260::<0x48> as *const (), 0), // +0x3c8860 sub_3c8860
    Lifted::new(0x7000ab4870, tpl_260::<0x50> as *const (), 0), // +0x3c8870 sub_3c8870
    Lifted::new(0x7000ab4880, tpl_260::<0x58> as *const (), 0), // +0x3c8880 sub_3c8880
    Lifted::new(0x7000ab4890, tpl_260::<0x60> as *const (), 0), // +0x3c8890 sub_3c8890
    Lifted::new(0x7000ab48a0, tpl_260::<0x68> as *const (), 0), // +0x3c88a0 sub_3c88a0
    Lifted::new(0x7000ab48b0, tpl_260::<0x70> as *const (), 0), // +0x3c88b0 sub_3c88b0
    Lifted::new(0x7000ab48c0, tpl_260::<0x78> as *const (), 0), // +0x3c88c0 sub_3c88c0
    Lifted::new(0x7000ab48d0, tpl_260::<0x80> as *const (), 0), // +0x3c88d0 sub_3c88d0
    Lifted::new(0x7000ab48e0, tpl_260::<0x88> as *const (), 0), // +0x3c88e0 sub_3c88e0
    Lifted::new(0x7000ab48f0, tpl_260::<0x90> as *const (), 0), // +0x3c88f0 sub_3c88f0
    Lifted::new(0x7000ab4900, tpl_260::<0x98> as *const (), 0), // +0x3c8900 sub_3c8900
    Lifted::new(0x7000ab4910, tpl_260::<0xa0> as *const (), 0), // +0x3c8910 sub_3c8910
    Lifted::new(0x7000ab4920, tpl_260::<0xa8> as *const (), 0), // +0x3c8920 sub_3c8920
    Lifted::new(0x7000ab4930, tpl_772::<0x7001c73174, 0x7000ab5ca0, 0x70007452f0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8930 sub_3c8930
    Lifted::new(0x7000ab49c0, tpl_772::<0x7001c73174, 0x7000ab5f40, 0x7000741400, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c89c0 sub_3c89c0
    Lifted::new(0x7000ab4a50, tpl_766::<0x7001c73174, 0x7000ab61e0, 0x7000ab5660> as *const (), 0), // +0x3c8a50 sub_3c8a50
    Lifted::new(0x7000ab4aa0, tpl_772::<0x7001c73174, 0x7000ab6440, 0x700079ddb0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8aa0 sub_3c8aa0
    Lifted::new(0x7000ab4b30, tpl_772::<0x7001c73174, 0x7000ab66e0, 0x700074eeb0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8b30 sub_3c8b30
    Lifted::new(0x7000ab4bc0, tpl_772::<0x7001c73174, 0x7000ab6980, 0x7000734420, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8bc0 sub_3c8bc0
    Lifted::new(0x7000ab4c50, tpl_772::<0x7001c73174, 0x7000ab6c20, 0x70006f1490, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8c50 sub_3c8c50
    Lifted::new(0x7000ab4ce0, tpl_772::<0x7001c73174, 0x7000ab6ec0, 0x7000797360, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8ce0 sub_3c8ce0
    Lifted::new(0x7000ab4d70, tpl_772::<0x7001c73174, 0x7000ab7160, 0x7000712910, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8d70 sub_3c8d70
    Lifted::new(0x7000ab4e00, tpl_772::<0x7001c73174, 0x7000ab7400, 0x7000781570, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8e00 sub_3c8e00
    Lifted::new(0x7000ab4e90, tpl_772::<0x7001c73174, 0x7000ab76a0, 0x700078f690, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8e90 sub_3c8e90
    Lifted::new(0x7000ab4f20, tpl_772::<0x7001c73174, 0x7000ab7940, 0x7000708720, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8f20 sub_3c8f20
    Lifted::new(0x7000ab4fb0, tpl_772::<0x7001c73174, 0x7000ab7be0, 0x7000742be0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c8fb0 sub_3c8fb0
    Lifted::new(0x7000ab5040, tpl_772::<0x7001c73174, 0x7000ab7e80, 0x70007523c0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c9040 sub_3c9040
    Lifted::new(0x7000ab50d0, tpl_772::<0x7001c73174, 0x7000ab8120, 0x70007383d0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c90d0 sub_3c90d0
    Lifted::new(0x7000ab5160, tpl_772::<0x7001c73174, 0x7000ab83c0, 0x700071de00, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c9160 sub_3c9160
    Lifted::new(0x7000ab51f0, tpl_772::<0x7001c73174, 0x7000ab8660, 0x7000780240, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c91f0 sub_3c91f0
    Lifted::new(0x7000ab5280, fn_7000ab5280 as *const (), 2), // +0x3c9280 sub_3c9280
    Lifted::new(0x7000ab5330, fn_7000ab5330 as *const (), 2), // +0x3c9330 sub_3c9330
    Lifted::new(0x7000ab53f0, tpl_772::<0x7001c73174, 0x7000ab8e70, 0x7000776340, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c93f0 sub_3c93f0
    Lifted::new(0x7000ab5480, fn_7000ab5480 as *const (), 2), // +0x3c9480 sub_3c9480
    Lifted::new(0x7000ab5540, tpl_772::<0x7001c73174, 0x7000ab93d0, 0x70007465b0, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c9540 sub_3c9540
    Lifted::new(0x7000ab55d0, tpl_772::<0x7001c73174, 0x7000ab9670, 0x7000724f70, 0x7000ab9910, 0xa8, 0x3040, 0x70006f10e0, 0xa0> as *const (), 0), // +0x3c95d0 sub_3c95d0
    Lifted::new(0x7000ab5660, fn_7000ab5660 as *const (), 0), // +0x3c9660 sub_3c9660
    Lifted::new(0x7000ab5ca0, fn_7000ab5ca0 as *const (), 2), // +0x3c9ca0 sub_3c9ca0
    Lifted::new(0x7000ab5f40, fn_7000ab5f40 as *const (), 2), // +0x3c9f40 sub_3c9f40
    Lifted::new(0x7000ab61e0, fn_7000ab61e0 as *const (), 2), // +0x3ca1e0 sub_3ca1e0
    Lifted::new(0x7000ab6440, fn_7000ab6440 as *const (), 2), // +0x3ca440 sub_3ca440
    Lifted::new(0x7000ab66e0, fn_7000ab66e0 as *const (), 2), // +0x3ca6e0 sub_3ca6e0
    Lifted::new(0x7000ab6980, fn_7000ab6980 as *const (), 2), // +0x3ca980 sub_3ca980
    Lifted::new(0x7000ab6c20, fn_7000ab6c20 as *const (), 2), // +0x3cac20 sub_3cac20
    Lifted::new(0x7000ab6ec0, fn_7000ab6ec0 as *const (), 2), // +0x3caec0 sub_3caec0
    Lifted::new(0x7000ab7160, fn_7000ab7160 as *const (), 2), // +0x3cb160 sub_3cb160
    Lifted::new(0x7000ab7400, fn_7000ab7400 as *const (), 2), // +0x3cb400 sub_3cb400
    Lifted::new(0x7000ab76a0, fn_7000ab76a0 as *const (), 2), // +0x3cb6a0 sub_3cb6a0
    Lifted::new(0x7000ab7940, fn_7000ab7940 as *const (), 2), // +0x3cb940 sub_3cb940
    Lifted::new(0x7000ab7be0, fn_7000ab7be0 as *const (), 2), // +0x3cbbe0 sub_3cbbe0
    Lifted::new(0x7000ab7e80, fn_7000ab7e80 as *const (), 2), // +0x3cbe80 sub_3cbe80
    Lifted::new(0x7000ab8120, fn_7000ab8120 as *const (), 2), // +0x3cc120 sub_3cc120
    Lifted::new(0x7000ab83c0, fn_7000ab83c0 as *const (), 2), // +0x3cc3c0 sub_3cc3c0
    Lifted::new(0x7000ab8660, fn_7000ab8660 as *const (), 2), // +0x3cc660 sub_3cc660
    Lifted::new(0x7000ab8900, fn_7000ab8900 as *const (), 2), // +0x3cc900 sub_3cc900
    Lifted::new(0x7000ab8bb0, fn_7000ab8bb0 as *const (), 2), // +0x3ccbb0 sub_3ccbb0
    Lifted::new(0x7000ab8e70, fn_7000ab8e70 as *const (), 2), // +0x3cce70 sub_3cce70
    Lifted::new(0x7000ab9110, fn_7000ab9110 as *const (), 2), // +0x3cd110 sub_3cd110
    Lifted::new(0x7000ab93d0, fn_7000ab93d0 as *const (), 2), // +0x3cd3d0 sub_3cd3d0
    Lifted::new(0x7000ab9670, fn_7000ab9670 as *const (), 2), // +0x3cd670 sub_3cd670
    Lifted::new(0x7000ab9910, fn_7000ab9910 as *const (), 0), // +0x3cd910 sub_3cd910
    Lifted::new(0x7000ab9b20, fn_7000ab9b20 as *const (), 0), // +0x3cdb20 sub_3cdb20
    Lifted::new(0x7000ab9bd0, fn_7000ab9bd0 as *const (), 0), // +0x3cdbd0 sub_3cdbd0
    Lifted::new(0x7000ab9f90, fn_7000ab9f90 as *const (), 0), // +0x3cdf90 sub_3cdf90
    Lifted::new(0x7000aba140, tpl_766::<0x7001c73174, 0x7000aba8d0, 0x7000aba190> as *const (), 0), // +0x3ce140 sub_3ce140
    Lifted::new(0x7000aba190, fn_7000aba190 as *const (), 2), // +0x3ce190 sub_3ce190
    Lifted::new(0x7000aba8d0, fn_7000aba8d0 as *const (), 2), // +0x3ce8d0 sub_3ce8d0
    Lifted::new(0x7000abab30, fn_7000abab30 as *const (), 2), // +0x3ceb30 sub_3ceb30
    Lifted::new(0x7000abb330, fn_7000abb330 as *const (), 2), // +0x3cf330 sub_3cf330
    Lifted::new(0x7000abbb30, fn_7000abbb30 as *const (), 2), // +0x3cfb30 sub_3cfb30
    Lifted::new(0x7000abc4e0, fn_7000abc4e0 as *const (), 2), // +0x3d04e0 sub_3d04e0
    Lifted::new(0x7000abce90, fn_7000abce90 as *const (), 0), // +0x3d0e90 sub_3d0e90
    Lifted::new(0x7000abcef0, tpl_33::<0x700177c312> as *const (), 0), // +0x3d0ef0 sub_3d0ef0
    Lifted::new(0x7000abcf20, tpl_33::<0x700177c312> as *const (), 0), // +0x3d0f20 sub_3d0f20
    Lifted::new(0x7000abcf50, tpl_766::<0x7001c73174, 0x7000ac4b30, 0x7000abd950> as *const (), 0), // +0x3d0f50 sub_3d0f50
    Lifted::new(0x7000abcfa0, tpl_773::<0x7001c73174, 0x7000ac4d90, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000e81670, 0x70009c9cb0, 0x7000745eb0> as *const (), 2), // +0x3d0fa0 sub_3d0fa0
    Lifted::new(0x7000abd070, fn_7000abd070 as *const (), 2), // +0x3d1070 sub_3d1070
    Lifted::new(0x7000abd1d0, tpl_766::<0x7001c73174, 0x7000ac53d0, 0x7000abe330> as *const (), 0), // +0x3d11d0 sub_3d11d0
    Lifted::new(0x7000abd220, tpl_766::<0x7001c73174, 0x7000ac5630, 0x7000abe5a0> as *const (), 0), // +0x3d1220 sub_3d1220
    Lifted::new(0x7000abd270, tpl_766::<0x7001c73174, 0x7000ac5890, 0x7000abe910> as *const (), 0), // +0x3d1270 sub_3d1270
    Lifted::new(0x7000abd2c0, tpl_766::<0x7001c73174, 0x7000ac5af0, 0x7000abeec0> as *const (), 0), // +0x3d12c0 sub_3d12c0
    Lifted::new(0x7000abd310, tpl_766::<0x7001c73174, 0x7000ac5d50, 0x7000abf580> as *const (), 0), // +0x3d1310 sub_3d1310
    Lifted::new(0x7000abd360, tpl_766::<0x7001c73174, 0x7000ac5fb0, 0x7000abf960> as *const (), 0), // +0x3d1360 sub_3d1360
    Lifted::new(0x7000abd3b0, tpl_766::<0x7001c73174, 0x7000ac6210, 0x7000abff00> as *const (), 0), // +0x3d13b0 sub_3d13b0
    Lifted::new(0x7000abd400, tpl_766::<0x7001c73174, 0x7000ac6470, 0x7000ac0370> as *const (), 0), // +0x3d1400 sub_3d1400
    Lifted::new(0x7000abd450, tpl_766::<0x7001c73174, 0x7000ac66d0, 0x7000ac0850> as *const (), 0), // +0x3d1450 sub_3d1450
    Lifted::new(0x7000abd4a0, tpl_766::<0x7001c73174, 0x7000ac6930, 0x7000ac0ab0> as *const (), 0), // +0x3d14a0 sub_3d14a0
    Lifted::new(0x7000abd4f0, tpl_766::<0x7001c73174, 0x7000ac6b90, 0x7000ac0e60> as *const (), 0), // +0x3d14f0 sub_3d14f0
    Lifted::new(0x7000abd540, tpl_766::<0x7001c73174, 0x7000ac6df0, 0x7000ac1410> as *const (), 0), // +0x3d1540 sub_3d1540
    Lifted::new(0x7000abd590, tpl_766::<0x7001c73174, 0x7000ac7050, 0x7000ac1a80> as *const (), 0), // +0x3d1590 sub_3d1590
    Lifted::new(0x7000abd5e0, tpl_766::<0x7001c73174, 0x7000ac72b0, 0x7000ac1e50> as *const (), 0), // +0x3d15e0 sub_3d15e0
    Lifted::new(0x7000abd630, tpl_766::<0x7001c73174, 0x7000ac7510, 0x7000ac23d0> as *const (), 0), // +0x3d1630 sub_3d1630
    Lifted::new(0x7000abd680, tpl_766::<0x7001c73174, 0x7000ac7770, 0x7000ac2850> as *const (), 0), // +0x3d1680 sub_3d1680
    Lifted::new(0x7000abd6d0, tpl_766::<0x7001c73174, 0x7000ac79d0, 0x7000ac2d20> as *const (), 0), // +0x3d16d0 sub_3d16d0
    Lifted::new(0x7000abd720, tpl_766::<0x7001c73174, 0x7000ac7c30, 0x7000ac31f0> as *const (), 0), // +0x3d1720 sub_3d1720
    Lifted::new(0x7000abd770, tpl_766::<0x7001c73174, 0x7000ac7e90, 0x7000ac3470> as *const (), 0), // +0x3d1770 sub_3d1770
    Lifted::new(0x7000abd7c0, tpl_766::<0x7001c73174, 0x7000ac80f0, 0x7000ac3800> as *const (), 0), // +0x3d17c0 sub_3d17c0
    Lifted::new(0x7000abd810, tpl_766::<0x7001c73174, 0x7000ac8350, 0x7000ac3ba0> as *const (), 0), // +0x3d1810 sub_3d1810
    Lifted::new(0x7000abd860, tpl_766::<0x7001c73174, 0x7000ac85b0, 0x7000ac3e20> as *const (), 0), // +0x3d1860 sub_3d1860
    Lifted::new(0x7000abd8b0, tpl_766::<0x7001c73174, 0x7000ac8810, 0x7000ac40a0> as *const (), 0), // +0x3d18b0 sub_3d18b0
    Lifted::new(0x7000abd900, tpl_766::<0x7001c73174, 0x7000ac8a70, 0x7000ac4460> as *const (), 0), // +0x3d1900 sub_3d1900
    Lifted::new(0x7000abd950, fn_7000abd950 as *const (), 2), // +0x3d1950 sub_3d1950
    Lifted::new(0x7000abe330, fn_7000abe330 as *const (), 2), // +0x3d2330 sub_3d2330
    Lifted::new(0x7000abe5a0, fn_7000abe5a0 as *const (), 2), // +0x3d25a0 sub_3d25a0
    Lifted::new(0x7000abe910, fn_7000abe910 as *const (), 2), // +0x3d2910 sub_3d2910
    Lifted::new(0x7000abeec0, fn_7000abeec0 as *const (), 2), // +0x3d2ec0 sub_3d2ec0
    Lifted::new(0x7000abf580, fn_7000abf580 as *const (), 2), // +0x3d3580 sub_3d3580
    Lifted::new(0x7000abf960, fn_7000abf960 as *const (), 2), // +0x3d3960 sub_3d3960
    Lifted::new(0x7000abff00, fn_7000abff00 as *const (), 2), // +0x3d3f00 sub_3d3f00
    Lifted::new(0x7000ac0370, fn_7000ac0370 as *const (), 2), // +0x3d4370 sub_3d4370
    Lifted::new(0x7000ac0850, fn_7000ac0850 as *const (), 2), // +0x3d4850 sub_3d4850
    Lifted::new(0x7000ac0ab0, fn_7000ac0ab0 as *const (), 2), // +0x3d4ab0 sub_3d4ab0
    Lifted::new(0x7000ac0e60, fn_7000ac0e60 as *const (), 2), // +0x3d4e60 sub_3d4e60
    Lifted::new(0x7000ac1410, fn_7000ac1410 as *const (), 2), // +0x3d5410 sub_3d5410
    Lifted::new(0x7000ac1a80, fn_7000ac1a80 as *const (), 2), // +0x3d5a80 sub_3d5a80
    Lifted::new(0x7000ac1e50, fn_7000ac1e50 as *const (), 2), // +0x3d5e50 sub_3d5e50
    Lifted::new(0x7000ac23d0, fn_7000ac23d0 as *const (), 2), // +0x3d63d0 sub_3d63d0
    Lifted::new(0x7000ac2850, fn_7000ac2850 as *const (), 2), // +0x3d6850 sub_3d6850
    Lifted::new(0x7000ac2d20, fn_7000ac2d20 as *const (), 2), // +0x3d6d20 sub_3d6d20
    Lifted::new(0x7000ac31f0, fn_7000ac31f0 as *const (), 2), // +0x3d71f0 sub_3d71f0
    Lifted::new(0x7000ac3470, fn_7000ac3470 as *const (), 2), // +0x3d7470 sub_3d7470
    Lifted::new(0x7000ac3800, fn_7000ac3800 as *const (), 2), // +0x3d7800 sub_3d7800
    Lifted::new(0x7000ac3ba0, fn_7000ac3ba0 as *const (), 2), // +0x3d7ba0 sub_3d7ba0
    Lifted::new(0x7000ac3e20, fn_7000ac3e20 as *const (), 2), // +0x3d7e20 sub_3d7e20
    Lifted::new(0x7000ac40a0, fn_7000ac40a0 as *const (), 2), // +0x3d80a0 sub_3d80a0
    Lifted::new(0x7000ac4460, fn_7000ac4460 as *const (), 2), // +0x3d8460 sub_3d8460
    Lifted::new(0x7000ac4b30, fn_7000ac4b30 as *const (), 2), // +0x3d8b30 sub_3d8b30
    Lifted::new(0x7000ac4d90, fn_7000ac4d90 as *const (), 2), // +0x3d8d90 sub_3d8d90
    Lifted::new(0x7000ac5080, fn_7000ac5080 as *const (), 2), // +0x3d9080 sub_3d9080
    Lifted::new(0x7000ac53d0, fn_7000ac53d0 as *const (), 2), // +0x3d93d0 sub_3d93d0
    Lifted::new(0x7000ac5630, fn_7000ac5630 as *const (), 2), // +0x3d9630 sub_3d9630
    Lifted::new(0x7000ac5890, fn_7000ac5890 as *const (), 2), // +0x3d9890 sub_3d9890
    Lifted::new(0x7000ac5af0, fn_7000ac5af0 as *const (), 2), // +0x3d9af0 sub_3d9af0
    Lifted::new(0x7000ac5d50, fn_7000ac5d50 as *const (), 2), // +0x3d9d50 sub_3d9d50
    Lifted::new(0x7000ac5fb0, fn_7000ac5fb0 as *const (), 2), // +0x3d9fb0 sub_3d9fb0
    Lifted::new(0x7000ac6210, fn_7000ac6210 as *const (), 2), // +0x3da210 sub_3da210
    Lifted::new(0x7000ac6470, fn_7000ac6470 as *const (), 2), // +0x3da470 sub_3da470
    Lifted::new(0x7000ac66d0, fn_7000ac66d0 as *const (), 2), // +0x3da6d0 sub_3da6d0
    Lifted::new(0x7000ac6930, fn_7000ac6930 as *const (), 2), // +0x3da930 sub_3da930
    Lifted::new(0x7000ac6b90, fn_7000ac6b90 as *const (), 2), // +0x3dab90 sub_3dab90
    Lifted::new(0x7000ac6df0, fn_7000ac6df0 as *const (), 2), // +0x3dadf0 sub_3dadf0
    Lifted::new(0x7000ac7050, fn_7000ac7050 as *const (), 2), // +0x3db050 sub_3db050
    Lifted::new(0x7000ac72b0, fn_7000ac72b0 as *const (), 2), // +0x3db2b0 sub_3db2b0
    Lifted::new(0x7000ac7510, fn_7000ac7510 as *const (), 2), // +0x3db510 sub_3db510
    Lifted::new(0x7000ac7770, fn_7000ac7770 as *const (), 2), // +0x3db770 sub_3db770
    Lifted::new(0x7000ac79d0, fn_7000ac79d0 as *const (), 2), // +0x3db9d0 sub_3db9d0
    Lifted::new(0x7000ac7c30, fn_7000ac7c30 as *const (), 2), // +0x3dbc30 sub_3dbc30
    Lifted::new(0x7000ac7e90, fn_7000ac7e90 as *const (), 2), // +0x3dbe90 sub_3dbe90
    Lifted::new(0x7000ac80f0, fn_7000ac80f0 as *const (), 2), // +0x3dc0f0 sub_3dc0f0
    Lifted::new(0x7000ac8350, fn_7000ac8350 as *const (), 2), // +0x3dc350 sub_3dc350
    Lifted::new(0x7000ac85b0, fn_7000ac85b0 as *const (), 2), // +0x3dc5b0 sub_3dc5b0
    Lifted::new(0x7000ac8810, fn_7000ac8810 as *const (), 2), // +0x3dc810 sub_3dc810
    Lifted::new(0x7000ac8a70, fn_7000ac8a70 as *const (), 2), // +0x3dca70 sub_3dca70
    Lifted::new(0x7000ac8cd0, fn_7000ac8cd0 as *const (), 2), // +0x3dccd0 sub_3dccd0
    Lifted::new(0x7000ac8d80, tpl_774::<0x10, 0x8, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7000ac8dd9, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x3dcd80 sub_3dcd80
    Lifted::new(0x7000ac8e50, fn_7000ac8e50 as *const (), 2), // +0x3dce50 sub_3dce50
    Lifted::new(0x7000ac8ed0, fn_7000ac8ed0 as *const (), 2), // +0x3dced0 sub_3dced0
    Lifted::new(0x7000ac90f0, fn_7000ac90f0 as *const (), 2), // +0x3dd0f0 sub_3dd0f0
    Lifted::new(0x7000ac91f0, tpl_775::<0x8, 0x10, 0x7000ac9560> as *const (), 2), // +0x3dd1f0 sub_3dd1f0
    Lifted::new(0x7000ac9250, tpl_775::<0x8, 0x10, 0x7000ac9980> as *const (), 2), // +0x3dd250 sub_3dd250
    Lifted::new(0x7000ac92b0, fn_7000ac92b0 as *const (), 1), // +0x3dd2b0 sub_3dd2b0
    Lifted::new(0x7000ac92f0, fn_7000ac92f0 as *const (), 1), // +0x3dd2f0 sub_3dd2f0
    Lifted::new(0x7000ac9330, fn_7000ac9330 as *const (), 2), // +0x3dd330 sub_3dd330
    Lifted::new(0x7000ac9560, fn_7000ac9560 as *const (), 0), // +0x3dd560 sub_3dd560
    Lifted::new(0x7000ac9980, fn_7000ac9980 as *const (), 0), // +0x3dd980 sub_3dd980
    Lifted::new(0x7000ac9da0, fn_7000ac9da0 as *const (), 2), // +0x3ddda0 sub_3ddda0
    Lifted::new(0x7000ac9e60, fn_7000ac9e60 as *const (), 1), // +0x3dde60 sub_3dde60
    Lifted::new(0x7000ac9eb0, fn_7000ac9eb0 as *const (), 1), // +0x3ddeb0 sub_3ddeb0
    Lifted::new(0x7000ac9f00, tpl_776::<0x8, 0x10, 0x7000ac9560> as *const (), 2), // +0x3ddf00 sub_3ddf00
    Lifted::new(0x7000ac9f50, tpl_776::<0x8, 0x10, 0x7000ac9980> as *const (), 2), // +0x3ddf50 sub_3ddf50
    Lifted::new(0x7000ac9fa0, fn_7000ac9fa0 as *const (), 2), // +0x3ddfa0 sub_3ddfa0
    Lifted::new(0x7000aca650, fn_7000aca650 as *const (), 0), // +0x3de650 sub_3de650
    Lifted::new(0x7000aca6c0, fn_7000aca6c0 as *const (), 0), // +0x3de6c0 sub_3de6c0
    Lifted::new(0x7000aca730, fn_7000aca730 as *const (), 0), // +0x3de730 sub_3de730
    Lifted::new(0x7000aca7d0, tpl_766::<0x7001c73174, 0x7000acae00, 0x7000aca950> as *const (), 0), // +0x3de7d0 sub_3de7d0
    Lifted::new(0x7000aca820, tpl_766::<0x7001c73174, 0x7000acb060, 0x7000acaca0> as *const (), 0), // +0x3de820 sub_3de820
    Lifted::new(0x7000aca870, fn_7000aca870 as *const (), 0), // +0x3de870 sub_3de870
    Lifted::new(0x7000aca950, fn_7000aca950 as *const (), 0), // +0x3de950 sub_3de950
    Lifted::new(0x7000acaca0, fn_7000acaca0 as *const (), 0), // +0x3deca0 sub_3deca0
    Lifted::new(0x7000acae00, fn_7000acae00 as *const (), 2), // +0x3dee00 sub_3dee00
    Lifted::new(0x7000acb060, fn_7000acb060 as *const (), 2), // +0x3df060 sub_3df060
    Lifted::new(0x7000acb3b0, fn_7000acb3b0 as *const (), 2), // +0x3df3b0 sub_3df3b0
    Lifted::new(0x7000acb6e0, tpl_766::<0x7001c73174, 0x7000acc010, 0x7000acba10> as *const (), 0), // +0x3df6e0 sub_3df6e0
    Lifted::new(0x7000acb730, tpl_766::<0x7001c73174, 0x7000acc270, 0x7000acbbe0> as *const (), 0), // +0x3df730 sub_3df730
    Lifted::new(0x7000acb780, tpl_777::<0x7001c73174, 0x7000acc4d0, 0xa6e8, 0x70017d90c8, 0xa6d8, 0xa6e0, 0x7000accf50, 0x180, 0x7000745eb0> as *const (), 0), // +0x3df780 sub_3df780
    Lifted::new(0x7000acb870, fn_7000acb870 as *const (), 0), // +0x3df870 sub_3df870
    Lifted::new(0x7000acb8d0, tpl_766::<0x7001c73174, 0x7000acca10, 0x7000acbdb0> as *const (), 0), // +0x3df8d0 sub_3df8d0
    Lifted::new(0x7000acb920, tpl_777::<0x7001c73174, 0x7000accc70, 0xa6e8, 0x70017dbb00, 0xa6d8, 0xa6e0, 0x7000accf50, 0x180, 0x7000745eb0> as *const (), 0), // +0x3df920 sub_3df920
    Lifted::new(0x7000acba10, tpl_778::<0x70017dbb40, 0x7000accf50, 0x70006f2a70, 0x70006f9a60, 0x7000edaf60, 0x7000745eb0> as *const (), 2), // +0x3dfa10 sub_3dfa10
    Lifted::new(0x7000acbbe0, tpl_778::<0x70017dbb90, 0x7000accf50, 0x70006f2a70, 0x70006f9a60, 0x7000edaf60, 0x7000745eb0> as *const (), 2), // +0x3dfbe0 sub_3dfbe0
    Lifted::new(0x7000acbdb0, fn_7000acbdb0 as *const (), 0), // +0x3dfdb0 sub_3dfdb0
    Lifted::new(0x7000acc010, fn_7000acc010 as *const (), 2), // +0x3e0010 sub_3e0010
    Lifted::new(0x7000acc270, fn_7000acc270 as *const (), 2), // +0x3e0270 sub_3e0270
    Lifted::new(0x7000acc4d0, fn_7000acc4d0 as *const (), 2), // +0x3e04d0 sub_3e04d0
    Lifted::new(0x7000acc7b0, fn_7000acc7b0 as *const (), 2), // +0x3e07b0 sub_3e07b0
    Lifted::new(0x7000acca10, fn_7000acca10 as *const (), 2), // +0x3e0a10 sub_3e0a10
    Lifted::new(0x7000accc70, fn_7000accc70 as *const (), 2), // +0x3e0c70 sub_3e0c70
    Lifted::new(0x7000accf50, fn_7000accf50 as *const (), 0), // +0x3e0f50 sub_3e0f50
    Lifted::new(0x7000acda90, fn_7000acda90 as *const (), 2), // +0x3e1a90 sub_3e1a90
    Lifted::new(0x7000ace220, tpl_779::<0x7001c73174, 0x7000acede0, 0x700076f690, 0x70010e9d60, 0x7000745eb0> as *const (), 0), // +0x3e2220 sub_3e2220
    Lifted::new(0x7000ace380, tpl_780::<0x7001c73174, 0x7000acf130, 0x700076f690, 0x70010e9d60, 0x7000745eb0> as *const (), 0), // +0x3e2380 sub_3e2380
    Lifted::new(0x7000ace4e0, tpl_779::<0x7001c73174, 0x7000acf480, 0x700076f690, 0x70010ea790, 0x7000745eb0> as *const (), 0), // +0x3e24e0 sub_3e24e0
    Lifted::new(0x7000ace640, tpl_780::<0x7001c73174, 0x7000acf7d0, 0x700076f690, 0x70010ea790, 0x7000745eb0> as *const (), 0), // +0x3e2640 sub_3e2640
    Lifted::new(0x7000ace7a0, tpl_781::<0x7001c73174, 0x7000acfb20, 0x700076f690, 0x70010eaef0, 0x7000745eb0> as *const (), 0), // +0x3e27a0 sub_3e27a0
    Lifted::new(0x7000ace8f0, tpl_766::<0x7001c73174, 0x7000acfe70, 0x7000acea90> as *const (), 0), // +0x3e28f0 sub_3e28f0
    Lifted::new(0x7000ace940, tpl_781::<0x7001c73174, 0x7000ad00d0, 0x700076f690, 0x70010ebb60, 0x7000745eb0> as *const (), 0), // +0x3e2940 sub_3e2940
    Lifted::new(0x7000acea90, fn_7000acea90 as *const (), 0), // +0x3e2a90 sub_3e2a90
    Lifted::new(0x7000acede0, fn_7000acede0 as *const (), 2), // +0x3e2de0 sub_3e2de0
    Lifted::new(0x7000acf130, fn_7000acf130 as *const (), 2), // +0x3e3130 sub_3e3130
    Lifted::new(0x7000acf480, fn_7000acf480 as *const (), 2), // +0x3e3480 sub_3e3480
    Lifted::new(0x7000acf7d0, fn_7000acf7d0 as *const (), 2), // +0x3e37d0 sub_3e37d0
    Lifted::new(0x7000acfb20, fn_7000acfb20 as *const (), 2), // +0x3e3b20 sub_3e3b20
    Lifted::new(0x7000acfe70, fn_7000acfe70 as *const (), 2), // +0x3e3e70 sub_3e3e70
    Lifted::new(0x7000ad00d0, fn_7000ad00d0 as *const (), 2), // +0x3e40d0 sub_3e40d0
    Lifted::new(0x7000ad0420, tpl_782::<0x8> as *const (), 1), // +0x3e4420 sub_3e4420
    Lifted::new(0x7000ad0440, fn_7000ad0440 as *const (), 0), // +0x3e4440 sub_3e4440
    Lifted::new(0x7000ad04a0, tpl_70 as *const (), 1), // +0x3e44a0 sub_3e44a0
    Lifted::new(0x7000ad04b0, tpl_783::<0x7001c73174, 0x7000ad0550, 0xa0> as *const (), 0), // +0x3e44b0 sub_3e44b0
    Lifted::new(0x7000ad04d0, fn_7000ad04d0 as *const (), 2), // +0x3e44d0 sub_3e44d0
    Lifted::new(0x7000ad0530, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ad0541> as *const (), 0), // +0x3e4530 sub_3e4530
    Lifted::new(0x7000ad0550, fn_7000ad0550 as *const (), 2), // +0x3e4550 sub_3e4550
    Lifted::new(0x7000ad07e0, fn_7000ad07e0 as *const (), 2), // +0x3e47e0 sub_3e47e0
    Lifted::new(0x7000ad09d0, fn_7000ad09d0 as *const (), 2), // +0x3e49d0 sub_3e49d0
    Lifted::new(0x7000ad0cb0, fn_7000ad0cb0 as *const (), 2), // +0x3e4cb0 sub_3e4cb0
    Lifted::new(0x7000ad0f90, fn_7000ad0f90 as *const (), 0), // +0x3e4f90 sub_3e4f90
    Lifted::new(0x7000ad1070, tpl_784::<0x7001c73174, 0x7000ad0cb0, 0xa6e8, 0xa6d8, 0xa6e0, 0x7, 0x70006fc980, 0x700079cfd0, 0x7000745eb0> as *const (), 0), // +0x3e5070 sub_3e5070
    Lifted::new(0x7000ad1150, fn_7000ad1150 as *const (), 1), // +0x3e5150 sub_3e5150
    Lifted::new(0x7000ad1170, tpl_349 as *const (), 1), // +0x3e5170 sub_3e5170
    Lifted::new(0x7000ad1180, tpl_30 as *const (), 1), // +0x3e5180 sub_3e5180
    Lifted::new(0x7000ad1190, tpl_50 as *const (), 1), // +0x3e5190 sub_3e5190
    Lifted::new(0x7000ad11a0, fn_7000ad11a0 as *const (), 0), // +0x3e51a0 sub_3e51a0
    Lifted::new(0x7000ad14a0, fn_7000ad14a0 as *const (), 2), // +0x3e54a0 sub_3e54a0
    Lifted::new(0x7000ad1700, fn_7000ad1700 as *const (), 2), // +0x3e5700 sub_3e5700
    Lifted::new(0x7000ad1aa0, tpl_766::<0x7001c73174, 0x7000ad14a0, 0x7000ad11a0> as *const (), 0), // +0x3e5aa0 sub_3e5aa0
    Lifted::new(0x7000ad1af0, fn_7000ad1af0 as *const (), 0), // +0x3e5af0 sub_3e5af0
    Lifted::new(0x7000ad1c30, fn_7000ad1c30 as *const (), 2), // +0x3e5c30 sub_3e5c30
    Lifted::new(0x7000ad2120, fn_7000ad2120 as *const (), 2), // +0x3e6120 sub_3e6120
    Lifted::new(0x7000ad25c0, fn_7000ad25c0 as *const (), 0), // +0x3e65c0 sub_3e65c0
    Lifted::new(0x7000ad2810, fn_7000ad2810 as *const (), 2), // +0x3e6810 sub_3e6810
    Lifted::new(0x7000ad2cb0, fn_7000ad2cb0 as *const (), 2), // +0x3e6cb0 sub_3e6cb0
    Lifted::new(0x7000ad2f10, fn_7000ad2f10 as *const (), 2), // +0x3e6f10 sub_3e6f10
    Lifted::new(0x7000ad3170, fn_7000ad3170 as *const (), 2), // +0x3e7170 sub_3e7170
    Lifted::new(0x7000ad33d0, fn_7000ad33d0 as *const (), 2), // +0x3e73d0 sub_3e73d0
    Lifted::new(0x7000ad3630, tpl_766::<0x7001c73174, 0x7000ad2cb0, 0x7000ad1c30> as *const (), 0), // +0x3e7630 sub_3e7630
    Lifted::new(0x7000ad3680, tpl_766::<0x7001c73174, 0x7000ad2f10, 0x7000ad2120> as *const (), 0), // +0x3e7680 sub_3e7680
    Lifted::new(0x7000ad36d0, tpl_766::<0x7001c73174, 0x7000ad3170, 0x7000ad25c0> as *const (), 0), // +0x3e76d0 sub_3e76d0
    Lifted::new(0x7000ad3720, tpl_766::<0x7001c73174, 0x7000ad33d0, 0x7000ad2810> as *const (), 0), // +0x3e7720 sub_3e7720
    Lifted::new(0x7000ad3770, fn_7000ad3770 as *const (), 2), // +0x3e7770 sub_3e7770
    Lifted::new(0x7000ad39a0, fn_7000ad39a0 as *const (), 2), // +0x3e79a0 sub_3e79a0
    Lifted::new(0x7000ad3bd0, fn_7000ad3bd0 as *const (), 2), // +0x3e7bd0 sub_3e7bd0
    Lifted::new(0x7000ad3e40, fn_7000ad3e40 as *const (), 2), // +0x3e7e40 sub_3e7e40
    Lifted::new(0x7000ad4260, fn_7000ad4260 as *const (), 0), // +0x3e8260 sub_3e8260
    Lifted::new(0x7000ad43a0, fn_7000ad43a0 as *const (), 0), // +0x3e83a0 sub_3e83a0
    Lifted::new(0x7000ad44e0, fn_7000ad44e0 as *const (), 0), // +0x3e84e0 sub_3e84e0
    Lifted::new(0x7000ad48c0, fn_7000ad48c0 as *const (), 0), // +0x3e88c0 sub_3e88c0
    Lifted::new(0x7000ad4bc0, fn_7000ad4bc0 as *const (), 0), // +0x3e8bc0 sub_3e8bc0
    Lifted::new(0x7000ad4dd0, fn_7000ad4dd0 as *const (), 2), // +0x3e8dd0 sub_3e8dd0
    Lifted::new(0x7000ad5110, fn_7000ad5110 as *const (), 2), // +0x3e9110 sub_3e9110
    Lifted::new(0x7000ad5450, fn_7000ad5450 as *const (), 2), // +0x3e9450 sub_3e9450
    Lifted::new(0x7000ad5790, fn_7000ad5790 as *const (), 2), // +0x3e9790 sub_3e9790
    Lifted::new(0x7000ad5ad0, fn_7000ad5ad0 as *const (), 2), // +0x3e9ad0 sub_3e9ad0
    Lifted::new(0x7000ad5e10, fn_7000ad5e10 as *const (), 2), // +0x3e9e10 sub_3e9e10
    Lifted::new(0x7000ad6070, fn_7000ad6070 as *const (), 2), // +0x3ea070 sub_3ea070
    Lifted::new(0x7000ad62e0, fn_7000ad62e0 as *const (), 2), // +0x3ea2e0 sub_3ea2e0
    Lifted::new(0x7000ad6540, fn_7000ad6540 as *const (), 2), // +0x3ea540 sub_3ea540
    Lifted::new(0x7000ad67a0, fn_7000ad67a0 as *const (), 2), // +0x3ea7a0 sub_3ea7a0
    Lifted::new(0x7000ad6ad0, fn_7000ad6ad0 as *const (), 2), // +0x3eaad0 sub_3eaad0
    Lifted::new(0x7000ad6e00, fn_7000ad6e00 as *const (), 2), // +0x3eae00 sub_3eae00
    Lifted::new(0x7000ad7060, fn_7000ad7060 as *const (), 2), // +0x3eb060 sub_3eb060
    Lifted::new(0x7000ad72c0, fn_7000ad72c0 as *const (), 2), // +0x3eb2c0 sub_3eb2c0
    Lifted::new(0x7000ad7520, fn_7000ad7520 as *const (), 2), // +0x3eb520 sub_3eb520
    Lifted::new(0x7000ad7860, tpl_785::<0x7001c73174, 0x7000ad4dd0, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000ad39a0, 0x7000745eb0> as *const (), 0), // +0x3eb860 sub_3eb860
    Lifted::new(0x7000ad7940, fn_7000ad7940 as *const (), 0), // +0x3eb940 sub_3eb940
    Lifted::new(0x7000ad7a30, fn_7000ad7a30 as *const (), 0), // +0x3eba30 sub_3eba30
    Lifted::new(0x7000ad7b10, tpl_785::<0x7001c73174, 0x7000ad5790, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000ad3770, 0x7000745eb0> as *const (), 0), // +0x3ebb10 sub_3ebb10
    Lifted::new(0x7000ad7bf0, tpl_786::<0x7001c73174, 0x7000ad5ad0, 0xa6e8, 0xfffffffffffffffc, 0xa6d8, 0xa6e0, 0xa0, 0xffffffffffffffff, 0x7, 0x5, 0x7000e92860, 0x180, 0x7000745eb0> as *const (), 0), // +0x3ebbf0 sub_3ebbf0
    Lifted::new(0x7000ad7d20, tpl_766::<0x7001c73174, 0x7000ad5e10, 0x7000ad3e40> as *const (), 0), // +0x3ebd20 sub_3ebd20
    Lifted::new(0x7000ad7d70, fn_7000ad7d70 as *const (), 0), // +0x3ebd70 sub_3ebd70
    Lifted::new(0x7000ad7dd0, tpl_766::<0x7001c73174, 0x7000ad62e0, 0x7000ad4260> as *const (), 0), // +0x3ebdd0 sub_3ebdd0
    Lifted::new(0x7000ad7e20, tpl_766::<0x7001c73174, 0x7000ad6540, 0x7000ad43a0> as *const (), 0), // +0x3ebe20 sub_3ebe20
    Lifted::new(0x7000ad7e70, fn_7000ad7e70 as *const (), 0), // +0x3ebe70 sub_3ebe70
    Lifted::new(0x7000ad7f50, fn_7000ad7f50 as *const (), 0), // +0x3ebf50 sub_3ebf50
    Lifted::new(0x7000ad8030, tpl_766::<0x7001c73174, 0x7000ad6e00, 0x7000ad44e0> as *const (), 0), // +0x3ec030 sub_3ec030
    Lifted::new(0x7000ad8080, tpl_766::<0x7001c73174, 0x7000ad7060, 0x7000ad48c0> as *const (), 0), // +0x3ec080 sub_3ec080
    Lifted::new(0x7000ad80d0, tpl_766::<0x7001c73174, 0x7000ad72c0, 0x7000ad4bc0> as *const (), 0), // +0x3ec0d0 sub_3ec0d0
    Lifted::new(0x7000ad8120, tpl_786::<0x7001c73174, 0x7000ad7520, 0xa6e8, 0xfffffffffffffffc, 0xa6d8, 0xa6e0, 0xa0, 0xffffffffffffffff, 0x7, 0x4, 0x7000e92860, 0x180, 0x7000745eb0> as *const (), 0), // +0x3ec120 sub_3ec120
    Lifted::new(0x7000ad8250, fn_7000ad8250 as *const (), 0), // +0x3ec250 sub_3ec250
    Lifted::new(0x7000ad83f0, fn_7000ad83f0 as *const (), 2), // +0x3ec3f0 sub_3ec3f0
    Lifted::new(0x7000ad87f0, fn_7000ad87f0 as *const (), 2), // +0x3ec7f0 sub_3ec7f0
    Lifted::new(0x7000ad8ad0, fn_7000ad8ad0 as *const (), 2), // +0x3ecad0 sub_3ecad0
    Lifted::new(0x7000ad8d60, fn_7000ad8d60 as *const (), 0), // +0x3ecd60 sub_3ecd60
    Lifted::new(0x7000ad8f60, fn_7000ad8f60 as *const (), 0), // +0x3ecf60 sub_3ecf60
    Lifted::new(0x7000ad9350, fn_7000ad9350 as *const (), 2), // +0x3ed350 sub_3ed350
    Lifted::new(0x7000ad95b0, fn_7000ad95b0 as *const (), 2), // +0x3ed5b0 sub_3ed5b0
    Lifted::new(0x7000ad9810, fn_7000ad9810 as *const (), 2), // +0x3ed810 sub_3ed810
    Lifted::new(0x7000ad9a70, fn_7000ad9a70 as *const (), 2), // +0x3eda70 sub_3eda70
    Lifted::new(0x7000ad9cd0, tpl_766::<0x7001c73174, 0x7000ad9350, 0x7000ad87f0> as *const (), 0), // +0x3edcd0 sub_3edcd0
    Lifted::new(0x7000ad9d20, tpl_766::<0x7001c73174, 0x7000ad95b0, 0x7000ad8ad0> as *const (), 0), // +0x3edd20 sub_3edd20
    Lifted::new(0x7000ad9d70, tpl_766::<0x7001c73174, 0x7000ad9810, 0x7000ad8d60> as *const (), 0), // +0x3edd70 sub_3edd70
    Lifted::new(0x7000ad9dc0, tpl_766::<0x7001c73174, 0x7000ad9a70, 0x7000ad8f60> as *const (), 0), // +0x3eddc0 sub_3eddc0
    Lifted::new(0x7000ad9e10, fn_7000ad9e10 as *const (), 0), // +0x3ede10 sub_3ede10
    Lifted::new(0x7000ad9f60, fn_7000ad9f60 as *const (), 0), // +0x3edf60 sub_3edf60
    Lifted::new(0x7000ada0b0, fn_7000ada0b0 as *const (), 0), // +0x3ee0b0 sub_3ee0b0
    Lifted::new(0x7000ada200, fn_7000ada200 as *const (), 0), // +0x3ee200 sub_3ee200
    Lifted::new(0x7000ada350, fn_7000ada350 as *const (), 0), // +0x3ee350 sub_3ee350
    Lifted::new(0x7000ada4a0, fn_7000ada4a0 as *const (), 0), // +0x3ee4a0 sub_3ee4a0
    Lifted::new(0x7000ada5f0, fn_7000ada5f0 as *const (), 0), // +0x3ee5f0 sub_3ee5f0
    Lifted::new(0x7000ada740, fn_7000ada740 as *const (), 0), // +0x3ee740 sub_3ee740
    Lifted::new(0x7000ada890, fn_7000ada890 as *const (), 0), // +0x3ee890 sub_3ee890
    Lifted::new(0x7000ada9e0, fn_7000ada9e0 as *const (), 0), // +0x3ee9e0 sub_3ee9e0
    Lifted::new(0x7000adab80, fn_7000adab80 as *const (), 0), // +0x3eeb80 sub_3eeb80
    Lifted::new(0x7000adadb0, fn_7000adadb0 as *const (), 0), // +0x3eedb0 sub_3eedb0
    Lifted::new(0x7000adaef0, fn_7000adaef0 as *const (), 0), // +0x3eeef0 sub_3eeef0
    Lifted::new(0x7000adb070, fn_7000adb070 as *const (), 0), // +0x3ef070 sub_3ef070
    Lifted::new(0x7000adb230, fn_7000adb230 as *const (), 0), // +0x3ef230 sub_3ef230
    Lifted::new(0x7000adb860, fn_7000adb860 as *const (), 0), // +0x3ef860 sub_3ef860
    Lifted::new(0x7000adba30, fn_7000adba30 as *const (), 2), // +0x3efa30 sub_3efa30
    Lifted::new(0x7000adbc90, fn_7000adbc90 as *const (), 2), // +0x3efc90 sub_3efc90
    Lifted::new(0x7000adbef0, fn_7000adbef0 as *const (), 2), // +0x3efef0 sub_3efef0
    Lifted::new(0x7000adc150, fn_7000adc150 as *const (), 2), // +0x3f0150 sub_3f0150
    Lifted::new(0x7000adc3b0, fn_7000adc3b0 as *const (), 2), // +0x3f03b0 sub_3f03b0
    Lifted::new(0x7000adc610, fn_7000adc610 as *const (), 2), // +0x3f0610 sub_3f0610
    Lifted::new(0x7000adc870, fn_7000adc870 as *const (), 2), // +0x3f0870 sub_3f0870
    Lifted::new(0x7000adcad0, fn_7000adcad0 as *const (), 2), // +0x3f0ad0 sub_3f0ad0
    Lifted::new(0x7000adcd30, fn_7000adcd30 as *const (), 2), // +0x3f0d30 sub_3f0d30
    Lifted::new(0x7000adcf90, fn_7000adcf90 as *const (), 2), // +0x3f0f90 sub_3f0f90
    Lifted::new(0x7000add1f0, fn_7000add1f0 as *const (), 2), // +0x3f11f0 sub_3f11f0
    Lifted::new(0x7000add450, fn_7000add450 as *const (), 2), // +0x3f1450 sub_3f1450
    Lifted::new(0x7000add6b0, fn_7000add6b0 as *const (), 2), // +0x3f16b0 sub_3f16b0
    Lifted::new(0x7000add910, fn_7000add910 as *const (), 2), // +0x3f1910 sub_3f1910
    Lifted::new(0x7000addb70, fn_7000addb70 as *const (), 2), // +0x3f1b70 sub_3f1b70
    Lifted::new(0x7000adddd0, fn_7000adddd0 as *const (), 2), // +0x3f1dd0 sub_3f1dd0
    Lifted::new(0x7000ade030, tpl_766::<0x7001c73174, 0x7000adba30, 0x7000ad9e10> as *const (), 0), // +0x3f2030 sub_3f2030
    Lifted::new(0x7000ade080, tpl_766::<0x7001c73174, 0x7000adbc90, 0x7000ad9f60> as *const (), 0), // +0x3f2080 sub_3f2080
    Lifted::new(0x7000ade0d0, tpl_766::<0x7001c73174, 0x7000adbef0, 0x7000ada0b0> as *const (), 0), // +0x3f20d0 sub_3f20d0
    Lifted::new(0x7000ade120, tpl_766::<0x7001c73174, 0x7000adc150, 0x7000ada200> as *const (), 0), // +0x3f2120 sub_3f2120
    Lifted::new(0x7000ade170, tpl_766::<0x7001c73174, 0x7000adc3b0, 0x7000ada350> as *const (), 0), // +0x3f2170 sub_3f2170
    Lifted::new(0x7000ade1c0, tpl_766::<0x7001c73174, 0x7000adc610, 0x7000ada4a0> as *const (), 0), // +0x3f21c0 sub_3f21c0
    Lifted::new(0x7000ade210, tpl_766::<0x7001c73174, 0x7000adc870, 0x7000ada5f0> as *const (), 0), // +0x3f2210 sub_3f2210
    Lifted::new(0x7000ade260, tpl_766::<0x7001c73174, 0x7000adcad0, 0x7000ada740> as *const (), 0), // +0x3f2260 sub_3f2260
    Lifted::new(0x7000ade2b0, tpl_766::<0x7001c73174, 0x7000adcd30, 0x7000ada890> as *const (), 0), // +0x3f22b0 sub_3f22b0
    Lifted::new(0x7000ade300, tpl_766::<0x7001c73174, 0x7000adcf90, 0x7000ada9e0> as *const (), 0), // +0x3f2300 sub_3f2300
    Lifted::new(0x7000ade350, tpl_766::<0x7001c73174, 0x7000add1f0, 0x7000adab80> as *const (), 0), // +0x3f2350 sub_3f2350
    Lifted::new(0x7000ade3a0, tpl_766::<0x7001c73174, 0x7000add450, 0x7000adadb0> as *const (), 0), // +0x3f23a0 sub_3f23a0
    Lifted::new(0x7000ade3f0, tpl_766::<0x7001c73174, 0x7000add6b0, 0x7000adaef0> as *const (), 0), // +0x3f23f0 sub_3f23f0
    Lifted::new(0x7000ade440, tpl_766::<0x7001c73174, 0x7000add910, 0x7000adb070> as *const (), 0), // +0x3f2440 sub_3f2440
    Lifted::new(0x7000ade490, tpl_766::<0x7001c73174, 0x7000addb70, 0x7000adb230> as *const (), 0), // +0x3f2490 sub_3f2490
    Lifted::new(0x7000ade4e0, tpl_766::<0x7001c73174, 0x7000adddd0, 0x7000adb860> as *const (), 0), // +0x3f24e0 sub_3f24e0
    Lifted::new(0x7000ade530, tpl_766::<0x7001c73174, 0x7000adef00, 0x7000ade670> as *const (), 0), // +0x3f2530 sub_3f2530
    Lifted::new(0x7000ade580, tpl_766::<0x7001c73174, 0x7000adf160, 0x7000ade820> as *const (), 0), // +0x3f2580 sub_3f2580
    Lifted::new(0x7000ade5d0, tpl_766::<0x7001c73174, 0x7000adf3c0, 0x7000adec00> as *const (), 0), // +0x3f25d0 sub_3f25d0
    Lifted::new(0x7000ade620, tpl_766::<0x7001c73174, 0x7000adf620, 0x7000aded80> as *const (), 0), // +0x3f2620 sub_3f2620
    Lifted::new(0x7000ade670, fn_7000ade670 as *const (), 2), // +0x3f2670 sub_3f2670
    Lifted::new(0x7000ade820, fn_7000ade820 as *const (), 2), // +0x3f2820 sub_3f2820
    Lifted::new(0x7000adec00, fn_7000adec00 as *const (), 0), // +0x3f2c00 sub_3f2c00
    Lifted::new(0x7000aded80, fn_7000aded80 as *const (), 0), // +0x3f2d80 sub_3f2d80
    Lifted::new(0x7000adef00, fn_7000adef00 as *const (), 2), // +0x3f2f00 sub_3f2f00
    Lifted::new(0x7000adf160, fn_7000adf160 as *const (), 2), // +0x3f3160 sub_3f3160
    Lifted::new(0x7000adf3c0, fn_7000adf3c0 as *const (), 2), // +0x3f33c0 sub_3f33c0
    Lifted::new(0x7000adf620, fn_7000adf620 as *const (), 2), // +0x3f3620 sub_3f3620
    Lifted::new(0x7000adf880, fn_7000adf880 as *const (), 2), // +0x3f3880 sub_3f3880
    Lifted::new(0x7000adfda0, fn_7000adfda0 as *const (), 2), // +0x3f3da0 sub_3f3da0
    Lifted::new(0x7000adff20, fn_7000adff20 as *const (), 0), // +0x3f3f20 sub_3f3f20
    Lifted::new(0x7000ae0140, tpl_787::<0x70009e75c0, 0x7000779320, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000ae01f6, 0x70007756d0, 0x70010e1f70, 0x7000ae02a4, 0x700076a260, 0x7000ae02cb, 0x7000ae07a0> as *const (), 0), // +0x3f4140 sub_3f4140
    Lifted::new(0x7000ae0470, tpl_787::<0x70009e75c0, 0x7000779320, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000ae0526, 0x70007756d0, 0x70010e2110, 0x7000ae05d4, 0x700076a260, 0x7000ae05fb, 0x7000ae0f80> as *const (), 0), // +0x3f4470 sub_3f4470
    Lifted::new(0x7000ae07a0, fn_7000ae07a0 as *const (), 0), // +0x3f47a0 sub_3f47a0
    Lifted::new(0x7000ae0f80, fn_7000ae0f80 as *const (), 0), // +0x3f4f80 sub_3f4f80
    Lifted::new(0x7000ae1830, fn_7000ae1830 as *const (), 0), // +0x3f5830 sub_3f5830
    Lifted::new(0x7000ae1ab0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x3f5ab0 sub_3f5ab0
    Lifted::new(0x7000ae1af0, tpl_393::<0x7001779e20, 0x7001d54968, 0x7000ae1bb7, 0x700177c0d0, 0x7000a36670, 0x70009a4dd0, 0x7000ae1c3b, 0x70007f0910, 0x7000ae1c41> as *const (), 0), // +0x3f5af0 sub_3f5af0
    Lifted::new(0x7000ae1ca0, fn_7000ae1ca0 as *const (), 0), // +0x3f5ca0 sub_3f5ca0
    Lifted::new(0x7000ae1eb0, fn_7000ae1eb0 as *const (), 2), // +0x3f5eb0 sub_3f5eb0
    Lifted::new(0x7000ae2110, fn_7000ae2110 as *const (), 2), // +0x3f6110 sub_3f6110
    Lifted::new(0x7000ae2460, fn_7000ae2460 as *const (), 2), // +0x3f6460 sub_3f6460
    Lifted::new(0x7000ae26c0, fn_7000ae26c0 as *const (), 2), // +0x3f66c0 sub_3f66c0
    Lifted::new(0x7000ae2920, fn_7000ae2920 as *const (), 2), // +0x3f6920 sub_3f6920
    Lifted::new(0x7000ae2b80, fn_7000ae2b80 as *const (), 2), // +0x3f6b80 sub_3f6b80
    Lifted::new(0x7000ae2de0, fn_7000ae2de0 as *const (), 2), // +0x3f6de0 sub_3f6de0
    Lifted::new(0x7000ae3040, fn_7000ae3040 as *const (), 2), // +0x3f7040 sub_3f7040
    Lifted::new(0x7000ae32a0, fn_7000ae32a0 as *const (), 2), // +0x3f72a0 sub_3f72a0
    Lifted::new(0x7000ae3500, fn_7000ae3500 as *const (), 2), // +0x3f7500 sub_3f7500
    Lifted::new(0x7000ae3f00, fn_7000ae3f00 as *const (), 0), // +0x3f7f00 sub_3f7f00
    Lifted::new(0x7000ae4430, fn_7000ae4430 as *const (), 0), // +0x3f8430 sub_3f8430
    Lifted::new(0x7000ae48a0, fn_7000ae48a0 as *const (), 0), // +0x3f88a0 sub_3f88a0
    Lifted::new(0x7000ae4a70, fn_7000ae4a70 as *const (), 0), // +0x3f8a70 sub_3f8a70
    Lifted::new(0x7000ae4c40, fn_7000ae4c40 as *const (), 0), // +0x3f8c40 sub_3f8c40
    Lifted::new(0x7000ae4e10, fn_7000ae4e10 as *const (), 0), // +0x3f8e10 sub_3f8e10
    Lifted::new(0x7000ae4fe0, fn_7000ae4fe0 as *const (), 2), // +0x3f8fe0 sub_3f8fe0
    Lifted::new(0x7000ae5730, tpl_766::<0x7001c73174, 0x7000ae1eb0, 0x7000ae3500> as *const (), 0), // +0x3f9730 sub_3f9730
    Lifted::new(0x7000ae5780, fn_7000ae5780 as *const (), 0), // +0x3f9780 sub_3f9780
    Lifted::new(0x7000ae5880, tpl_766::<0x7001c73174, 0x7000ae2460, 0x7000ae3f00> as *const (), 0), // +0x3f9880 sub_3f9880
    Lifted::new(0x7000ae58d0, tpl_766::<0x7001c73174, 0x7000ae26c0, 0x7000ae4430> as *const (), 0), // +0x3f98d0 sub_3f98d0
    Lifted::new(0x7000ae5920, tpl_766::<0x7001c73174, 0x7000ae2920, 0x7000ae48a0> as *const (), 0), // +0x3f9920 sub_3f9920
    Lifted::new(0x7000ae5970, tpl_766::<0x7001c73174, 0x7000ae2b80, 0x7000ae4a70> as *const (), 0), // +0x3f9970 sub_3f9970
    Lifted::new(0x7000ae59c0, tpl_766::<0x7001c73174, 0x7000ae2de0, 0x7000ae4c40> as *const (), 0), // +0x3f99c0 sub_3f99c0
    Lifted::new(0x7000ae5a10, tpl_766::<0x7001c73174, 0x7000ae3040, 0x7000ae4e10> as *const (), 0), // +0x3f9a10 sub_3f9a10
    Lifted::new(0x7000ae5a60, tpl_766::<0x7001c73174, 0x7000ae32a0, 0x7000ae4fe0> as *const (), 0), // +0x3f9a60 sub_3f9a60
    Lifted::new(0x7000ae5ab0, tpl_788::<0x70007410e0> as *const (), 0), // +0x3f9ab0 sub_3f9ab0
    Lifted::new(0x7000ae5b70, tpl_788::<0x700076b010> as *const (), 0), // +0x3f9b70 sub_3f9b70
    Lifted::new(0x7000ae5c30, fn_7000ae5c30 as *const (), 0), // +0x3f9c30 sub_3f9c30
    Lifted::new(0x7000ae5cc0, fn_7000ae5cc0 as *const (), 2), // +0x3f9cc0 sub_3f9cc0
    Lifted::new(0x7000ae5e80, fn_7000ae5e80 as *const (), 2), // +0x3f9e80 sub_3f9e80
    Lifted::new(0x7000ae5fc0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000ae5fd0> as *const (), 0), // +0x3f9fc0 sub_3f9fc0
    Lifted::new(0x7000ae5fe0, tpl_789::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000ae6030, 0x70007f0910, 0x7000ae604b> as *const (), 0), // +0x3f9fe0 sub_3f9fe0
    Lifted::new(0x7000ae6070, tpl_790::<0x70007410e0> as *const (), 0), // +0x3fa070 sub_3fa070
    Lifted::new(0x7000ae6150, tpl_790::<0x700076b010> as *const (), 0), // +0x3fa150 sub_3fa150
    Lifted::new(0x7000ae6230, fn_7000ae6230 as *const (), 2), // +0x3fa230 sub_3fa230
    Lifted::new(0x7000ae6490, fn_7000ae6490 as *const (), 2), // +0x3fa490 sub_3fa490
    Lifted::new(0x7000ae67e0, fn_7000ae67e0 as *const (), 2), // +0x3fa7e0 sub_3fa7e0
    Lifted::new(0x7000ae6b20, fn_7000ae6b20 as *const (), 0), // +0x3fab20 sub_3fab20
    Lifted::new(0x7000ae6d60, tpl_766::<0x7001c73174, 0x7000ae6230, 0x7000ae6b20> as *const (), 0), // +0x3fad60 sub_3fad60
    Lifted::new(0x7000ae6db0, fn_7000ae6db0 as *const (), 0), // +0x3fadb0 sub_3fadb0
    Lifted::new(0x7000ae6f10, fn_7000ae6f10 as *const (), 0), // +0x3faf10 sub_3faf10
    Lifted::new(0x7000ae7080, fn_7000ae7080 as *const (), 0), // +0x3fb080 sub_3fb080
    Lifted::new(0x7000ae7270, fn_7000ae7270 as *const (), 0), // +0x3fb270 sub_3fb270
    Lifted::new(0x7000ae73f0, tpl_33::<0x700177c312> as *const (), 0), // +0x3fb3f0 sub_3fb3f0
    Lifted::new(0x7000ae7420, fn_7000ae7420 as *const (), 0), // +0x3fb420 sub_3fb420
    Lifted::new(0x7000ae74c0, fn_7000ae74c0 as *const (), 0), // +0x3fb4c0 sub_3fb4c0
    Lifted::new(0x7000ae74f0, fn_7000ae74f0 as *const (), 0), // +0x3fb4f0 sub_3fb4f0
    Lifted::new(0x7000ae7650, fn_7000ae7650 as *const (), 2), // +0x3fb650 sub_3fb650
    Lifted::new(0x7000ae7990, fn_7000ae7990 as *const (), 2), // +0x3fb990 sub_3fb990
    Lifted::new(0x7000ae7bf0, fn_7000ae7bf0 as *const (), 2), // +0x3fbbf0 sub_3fbbf0
    Lifted::new(0x7000ae8220, tpl_766::<0x7001c73174, 0x7000ae7650, 0x7000ae74f0> as *const (), 0), // +0x3fc220 sub_3fc220
    Lifted::new(0x7000ae8270, tpl_766::<0x7001c73174, 0x7000ae7990, 0x7000ae7bf0> as *const (), 0), // +0x3fc270 sub_3fc270
    Lifted::new(0x7000ae82c0, fn_7000ae82c0 as *const (), 0), // +0x3fc2c0 sub_3fc2c0
    Lifted::new(0x7000ae8350, fn_7000ae8350 as *const (), 0), // +0x3fc350 sub_3fc350
    Lifted::new(0x7000ae8400, fn_7000ae8400 as *const (), 2), // +0x3fc400 sub_3fc400
    Lifted::new(0x7000ae8500, fn_7000ae8500 as *const (), 2), // +0x3fc500 sub_3fc500
    Lifted::new(0x7000ae8760, fn_7000ae8760 as *const (), 2), // +0x3fc760 sub_3fc760
    Lifted::new(0x7000ae89c0, fn_7000ae89c0 as *const (), 2), // +0x3fc9c0 sub_3fc9c0
    Lifted::new(0x7000ae8c20, fn_7000ae8c20 as *const (), 2), // +0x3fcc20 sub_3fcc20
    Lifted::new(0x7000ae8e80, fn_7000ae8e80 as *const (), 2), // +0x3fce80 sub_3fce80
    Lifted::new(0x7000ae90e0, fn_7000ae90e0 as *const (), 2), // +0x3fd0e0 sub_3fd0e0
    Lifted::new(0x7000ae9340, fn_7000ae9340 as *const (), 2), // +0x3fd340 sub_3fd340
    Lifted::new(0x7000ae95a0, fn_7000ae95a0 as *const (), 0), // +0x3fd5a0 sub_3fd5a0
    Lifted::new(0x7000ae9740, fn_7000ae9740 as *const (), 2), // +0x3fd740 sub_3fd740
    Lifted::new(0x7000ae9ea0, fn_7000ae9ea0 as *const (), 0), // +0x3fdea0 sub_3fdea0
    Lifted::new(0x7000aea4a0, fn_7000aea4a0 as *const (), 0), // +0x3fe4a0 sub_3fe4a0
    Lifted::new(0x7000aea8b0, fn_7000aea8b0 as *const (), 2), // +0x3fe8b0 sub_3fe8b0
    Lifted::new(0x7000aead40, fn_7000aead40 as *const (), 2), // +0x3fed40 sub_3fed40
    Lifted::new(0x7000aeb1f0, fn_7000aeb1f0 as *const (), 0), // +0x3ff1f0 sub_3ff1f0
    Lifted::new(0x7000aeb4f0, tpl_766::<0x7001c73174, 0x7000ae8500, 0x7000ae95a0> as *const (), 0), // +0x3ff4f0 sub_3ff4f0
    Lifted::new(0x7000aeb540, tpl_766::<0x7001c73174, 0x7000ae8760, 0x7000ae9740> as *const (), 0), // +0x3ff540 sub_3ff540
    Lifted::new(0x7000aeb590, tpl_766::<0x7001c73174, 0x7000ae89c0, 0x7000ae9ea0> as *const (), 0), // +0x3ff590 sub_3ff590
    Lifted::new(0x7000aeb5e0, tpl_766::<0x7001c73174, 0x7000ae8c20, 0x7000aea4a0> as *const (), 0), // +0x3ff5e0 sub_3ff5e0
    Lifted::new(0x7000aeb630, tpl_766::<0x7001c73174, 0x7000ae8e80, 0x7000aea8b0> as *const (), 0), // +0x3ff630 sub_3ff630
    Lifted::new(0x7000aeb680, tpl_766::<0x7001c73174, 0x7000ae90e0, 0x7000aead40> as *const (), 0), // +0x3ff680 sub_3ff680
    Lifted::new(0x7000aeb6d0, tpl_766::<0x7001c73174, 0x7000ae9340, 0x7000aeb1f0> as *const (), 0), // +0x3ff6d0 sub_3ff6d0
    Lifted::new(0x7000aeb720, fn_7000aeb720 as *const (), 3), // +0x3ff720 sub_3ff720
    Lifted::new(0x7000aeb7a0, fn_7000aeb7a0 as *const (), 0), // +0x3ff7a0 sub_3ff7a0
    Lifted::new(0x7000aeba00, fn_7000aeba00 as *const (), 2), // +0x3ffa00 sub_3ffa00
    Lifted::new(0x7000aec020, tpl_766::<0x7001c73174, 0x7000aec2c0, 0x7000aec070> as *const (), 0), // +0x400020 sub_400020
    Lifted::new(0x7000aec070, fn_7000aec070 as *const (), 0), // +0x400070 sub_400070
    Lifted::new(0x7000aec2c0, fn_7000aec2c0 as *const (), 2), // +0x4002c0 sub_4002c0
    Lifted::new(0x7000aec520, fn_7000aec520 as *const (), 2), // +0x400520 sub_400520
    Lifted::new(0x7000aec8e0, tpl_377::<0x17, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x4008e0 sub_4008e0
    Lifted::new(0x7000aec990, tpl_377::<0x13, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x400990 sub_400990
    Lifted::new(0x7000aeca40, fn_7000aeca40 as *const (), 2), // +0x400a40 sub_400a40
    Lifted::new(0x7000aecb20, fn_7000aecb20 as *const (), 0), // +0x400b20 sub_400b20
    Lifted::new(0x7000aecc30, fn_7000aecc30 as *const (), 0), // +0x400c30 sub_400c30
    Lifted::new(0x7000aecca0, fn_7000aecca0 as *const (), 0), // +0x400ca0 sub_400ca0
    Lifted::new(0x7000aecd10, fn_7000aecd10 as *const (), 0), // +0x400d10 sub_400d10
    Lifted::new(0x7000aef7c0, fn_7000aef7c0 as *const (), 0), // +0x4037c0 sub_4037c0
    Lifted::new(0x7000aef830, fn_7000aef830 as *const (), 0), // +0x403830 sub_403830
    Lifted::new(0x7000aef960, tpl_791::<0x7001c4a180> as *const (), 1), // +0x403960 sub_403960
    Lifted::new(0x7000aef980, fn_7000aef980 as *const (), 0), // +0x403980 sub_403980
    Lifted::new(0x7000aefc70, fn_7000aefc70 as *const (), 0), // +0x403c70 sub_403c70
    Lifted::new(0x7000aefda0, fn_7000aefda0 as *const (), 2), // +0x403da0 sub_403da0
    Lifted::new(0x7000aeff40, fn_7000aeff40 as *const (), 2), // +0x403f40 sub_403f40
    Lifted::new(0x7000af02b0, tpl_160::<0xffffffffffffffff> as *const (), 1), // +0x4042b0 sub_4042b0
    Lifted::new(0x7000af02c0, fn_7000af02c0 as *const (), 1), // +0x4042c0 sub_4042c0
    Lifted::new(0x7000af02d0, fn_7000af02d0 as *const (), 1), // +0x4042d0 sub_4042d0
    Lifted::new(0x7000af02f0, fn_7000af02f0 as *const (), 0), // +0x4042f0 sub_4042f0
    Lifted::new(0x7000af0320, fn_7000af0320 as *const (), 0), // +0x404320 sub_404320
    Lifted::new(0x7000af0430, fn_7000af0430 as *const (), 0), // +0x404430 sub_404430
    Lifted::new(0x7000af04c0, fn_7000af04c0 as *const (), 1), // +0x4044c0 sub_4044c0
    Lifted::new(0x7000af04e0, fn_7000af04e0 as *const (), 1), // +0x4044e0 sub_4044e0
    Lifted::new(0x7000af04f0, fn_7000af04f0 as *const (), 0), // +0x4044f0 sub_4044f0
    Lifted::new(0x7000af05d0, fn_7000af05d0 as *const (), 1), // +0x4045d0 sub_4045d0
    Lifted::new(0x7000af05f0, fn_7000af05f0 as *const (), 0), // +0x4045f0 sub_4045f0
    Lifted::new(0x7000af0750, fn_7000af0750 as *const (), 0), // +0x404750 sub_404750
    Lifted::new(0x7000af07c0, fn_7000af07c0 as *const (), 0), // +0x4047c0 sub_4047c0
    Lifted::new(0x7000af0830, fn_7000af0830 as *const (), 0), // +0x404830 sub_404830
    Lifted::new(0x7000af09c0, tpl_168::<0x8> as *const (), 1), // +0x4049c0 sub_4049c0
    Lifted::new(0x7000af09d0, fn_7000af09d0 as *const (), 0), // +0x4049d0 sub_4049d0
    Lifted::new(0x7000af0a00, fn_7000af0a00 as *const (), 0), // +0x404a00 sub_404a00
    Lifted::new(0x7000af0a30, tpl_791::<0x7001c4a170> as *const (), 1), // +0x404a30 sub_404a30
    Lifted::new(0x7000af0a50, fn_7000af0a50 as *const (), 0), // +0x404a50 sub_404a50
    Lifted::new(0x7000af0a70, fn_7000af0a70 as *const (), 1), // +0x404a70 sub_404a70
    Lifted::new(0x7000af0ac0, tpl_792::<0x70017ea278, 0x70007844b0, 0x7000770500> as *const (), 0), // +0x404ac0 sub_404ac0
    Lifted::new(0x7000af0af0, tpl_793::<0x70017ea278, 0x70007844b0, 0x7000770500, 0x700177c312> as *const (), 0), // +0x404af0 sub_404af0
    Lifted::new(0x7000af0b50, fn_7000af0b50 as *const (), 0), // +0x404b50 sub_404b50
    Lifted::new(0x7000af0ba0, tpl_432::<0x7000701460> as *const (), 0), // +0x404ba0 sub_404ba0
    Lifted::new(0x7000af0bb0, fn_7000af0bb0 as *const (), 2), // +0x404bb0 sub_404bb0
    Lifted::new(0x7000af0ed0, tpl_794::<0x70006f4fa0> as *const (), 0), // +0x404ed0 sub_404ed0
    Lifted::new(0x7000af0ee0, fn_7000af0ee0 as *const (), 0), // +0x404ee0 sub_404ee0
    Lifted::new(0x7000af0f40, fn_7000af0f40 as *const (), 0), // +0x404f40 sub_404f40
    Lifted::new(0x7000af0fa0, fn_7000af0fa0 as *const (), 1), // +0x404fa0 sub_404fa0
    Lifted::new(0x7000af0fc0, tpl_112::<0x7001d55090, 0x7000af0fca> as *const (), 0), // +0x404fc0 sub_404fc0
    Lifted::new(0x7000af0fd0, tpl_112::<0x7001d55090, 0x7000af0fda> as *const (), 0), // +0x404fd0 sub_404fd0
    Lifted::new(0x7000af0fe0, fn_7000af0fe0 as *const (), 1), // +0x404fe0 sub_404fe0
    Lifted::new(0x7000af1000, tpl_19 as *const (), 3), // +0x405000 sub_405000
    Lifted::new(0x7000af1010, tpl_112::<0x7001d55090, 0x7000af101a> as *const (), 0), // +0x405010 sub_405010
    Lifted::new(0x7000af1020, tpl_112::<0x7001d55090, 0x7000af102a> as *const (), 0), // +0x405020 sub_405020
    Lifted::new(0x7000af1030, fn_7000af1030 as *const (), 0), // +0x405030 sub_405030
    Lifted::new(0x7000af1060, fn_7000af1060 as *const (), 0), // +0x405060 sub_405060
    Lifted::new(0x7000af1150, fn_7000af1150 as *const (), 1), // +0x405150 sub_405150
    Lifted::new(0x7000af11f0, fn_7000af11f0 as *const (), 1), // +0x4051f0 sub_4051f0
    Lifted::new(0x7000af1200, fn_7000af1200 as *const (), 1), // +0x405200 sub_405200
    Lifted::new(0x7000af1230, fn_7000af1230 as *const (), 1), // +0x405230 sub_405230
    Lifted::new(0x7000af1240, tpl_39::<0xc> as *const (), 1), // +0x405240 sub_405240
    Lifted::new(0x7000af1250, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000af1424, 0x7000af3340, 0x7000af142a, 0x70007f0910, 0x7000af1430> as *const (), 0), // +0x405250 sub_405250
    Lifted::new(0x7000af14b0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000af1684, 0x7000af3360, 0x7000af168a, 0x70007f0910, 0x7000af1690> as *const (), 0), // +0x4054b0 sub_4054b0
    Lifted::new(0x7000af1710, fn_7000af1710 as *const (), 2), // +0x405710 sub_405710
    Lifted::new(0x7000af1a30, tpl_68 as *const (), 1), // +0x405a30 sub_405a30
    Lifted::new(0x7000af1b10, fn_7000af1b10 as *const (), 2), // +0x405b10 sub_405b10
    Lifted::new(0x7000af1d40, fn_7000af1d40 as *const (), 1), // +0x405d40 sub_405d40
    Lifted::new(0x7000af1d60, fn_7000af1d60 as *const (), 1), // +0x405d60 sub_405d60
    Lifted::new(0x7000af1d80, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000af1e23> as *const (), 0), // +0x405d80 sub_405d80
    Lifted::new(0x7000af1e50, fn_7000af1e50 as *const (), 0), // +0x405e50 sub_405e50
    Lifted::new(0x7000af1fb0, fn_7000af1fb0 as *const (), 0), // +0x405fb0 sub_405fb0
    Lifted::new(0x7000af21f0, fn_7000af21f0 as *const (), 0), // +0x4061f0 sub_4061f0
    Lifted::new(0x7000af22c0, tpl_604::<0xfffffffffffffffc, 0x7000af2430> as *const (), 0), // +0x4062c0 sub_4062c0
    Lifted::new(0x7000af22d0, tpl_133::<0x70007a2950, 0x7000701460> as *const (), 0), // +0x4062d0 sub_4062d0
    Lifted::new(0x7000af2310, tpl_33::<0x700177c312> as *const (), 0), // +0x406310 sub_406310
    Lifted::new(0x7000af2340, tpl_795::<0x8, 0x700177c312> as *const (), 0), // +0x406340 sub_406340
    Lifted::new(0x7000af2390, fn_7000af2390 as *const (), 0), // +0x406390 sub_406390
    Lifted::new(0x7000af2400, tpl_33::<0x700177c312> as *const (), 0), // +0x406400 sub_406400
    Lifted::new(0x7000af2430, fn_7000af2430 as *const (), 0), // +0x406430 sub_406430
    Lifted::new(0x7000af24b0, fn_7000af24b0 as *const (), 0), // +0x4064b0 sub_4064b0
    Lifted::new(0x7000af2540, fn_7000af2540 as *const (), 0), // +0x406540 sub_406540
    Lifted::new(0x7000af2690, fn_7000af2690 as *const (), 0), // +0x406690 sub_406690
    Lifted::new(0x7000af26e0, fn_7000af26e0 as *const (), 1), // +0x4066e0 sub_4066e0
    Lifted::new(0x7000af2790, fn_7000af2790 as *const (), 3), // +0x406790 sub_406790
    Lifted::new(0x7000af2870, fn_7000af2870 as *const (), 0), // +0x406870 sub_406870
    Lifted::new(0x7000af2930, tpl_796::<0x78> as *const (), 1), // +0x406930 sub_406930
    Lifted::new(0x7000af2950, tpl_796::<0x90> as *const (), 1), // +0x406950 sub_406950
    Lifted::new(0x7000af2970, fn_7000af2970 as *const (), 0), // +0x406970 sub_406970
    Lifted::new(0x7000af2a40, tpl_177::<0x70017ea408, 0x7001d54ef8, 0x7000af2a51> as *const (), 0), // +0x406a40 sub_406a40
    Lifted::new(0x7000af2a60, fn_7000af2a60 as *const (), 0), // +0x406a60 sub_406a60
    Lifted::new(0x7000af2bb0, tpl_144 as *const (), 1), // +0x406bb0 sub_406bb0
    Lifted::new(0x7000af2bc0, tpl_44 as *const (), 1), // +0x406bc0 sub_406bc0
    Lifted::new(0x7000af2bd0, fn_7000af2bd0 as *const (), 0), // +0x406bd0 sub_406bd0
    Lifted::new(0x7000af2ca0, fn_7000af2ca0 as *const (), 0), // +0x406ca0 sub_406ca0
    Lifted::new(0x7000af2d60, tpl_196 as *const (), 1), // +0x406d60 sub_406d60
    Lifted::new(0x7000af2d70, tpl_39::<0x1c> as *const (), 1), // +0x406d70 sub_406d70
    Lifted::new(0x7000af2d80, fn_7000af2d80 as *const (), 2), // +0x406d80 sub_406d80
    Lifted::new(0x7000af2ed0, fn_7000af2ed0 as *const (), 0), // +0x406ed0 sub_406ed0
    Lifted::new(0x7000af2f80, fn_7000af2f80 as *const (), 2), // +0x406f80 sub_406f80
    Lifted::new(0x7000af2fe0, tpl_196 as *const (), 1), // +0x406fe0 sub_406fe0
    Lifted::new(0x7000af2ff0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000af30f7, 0x7000af1a30, 0x70007f0910, 0x7000af3113> as *const (), 0), // +0x406ff0 sub_406ff0
    Lifted::new(0x7000af3160, fn_7000af3160 as *const (), 0), // +0x407160 sub_407160
    Lifted::new(0x7000af3340, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000af3350> as *const (), 0), // +0x407340 sub_407340
    Lifted::new(0x7000af3360, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000af3370> as *const (), 0), // +0x407360 sub_407360
    Lifted::new(0x7000af3380, tpl_42 as *const (), 1), // +0x407380 sub_407380
    Lifted::new(0x7000af3390, tpl_191::<0x10> as *const (), 1), // +0x407390 sub_407390
    Lifted::new(0x7000af33a0, tpl_39::<0x10> as *const (), 1), // +0x4073a0 sub_4073a0
    Lifted::new(0x7000af33b0, tpl_39::<0x10> as *const (), 1), // +0x4073b0 sub_4073b0
    Lifted::new(0x7000af33c0, tpl_39::<0x18> as *const (), 1), // +0x4073c0 sub_4073c0
    Lifted::new(0x7000af33d0, tpl_36::<0x8> as *const (), 1), // +0x4073d0 sub_4073d0
    Lifted::new(0x7000af33e0, tpl_36::<0x8> as *const (), 1), // +0x4073e0 sub_4073e0
    Lifted::new(0x7000af33f0, fn_7000af33f0 as *const (), 1), // +0x4073f0 sub_4073f0
    Lifted::new(0x7000af3400, tpl_189::<0x7001c55ad0> as *const (), 1), // +0x407400 sub_407400
    Lifted::new(0x7000af3420, tpl_189::<0x7001c55a50> as *const (), 1), // +0x407420 sub_407420
    Lifted::new(0x7000af3440, fn_7000af3440 as *const (), 1), // +0x407440 sub_407440
    Lifted::new(0x7000af3450, fn_7000af3450 as *const (), 2), // +0x407450 sub_407450
    Lifted::new(0x7000af3740, tpl_797::<0x7001c6abd8, 0x7001d54390, 0x2c, 0x10, 0x3c, 0x20, 0x4c, 0x28, 0x54, 0x7001d54388, 0x7000af3e80, 0x7001d54380, 0x7001d547b8, 0x70017eb100, 0x70017eb110, 0x70017eb118, 0x7001779250, 0x7000af380f> as *const (), 2), // +0x407740 sub_407740
    Lifted::new(0x7000af3850, tpl_797::<0x7001c6abd8, 0x7001d54390, 0x2c, 0x10, 0x3c, 0x20, 0x4c, 0x28, 0x54, 0x7001d54388, 0x7000af3e80, 0x7001d54380, 0x7001d547b8, 0x70017eb100, 0x70017eb110, 0x70017eb118, 0x7001779250, 0x7000af391f> as *const (), 2), // +0x407850 sub_407850
    Lifted::new(0x7000af3960, fn_7000af3960 as *const (), 0), // +0x407960 sub_407960
    Lifted::new(0x7000af39f0, fn_7000af39f0 as *const (), 3), // +0x4079f0 sub_4079f0
    Lifted::new(0x7000af3a80, fn_7000af3a80 as *const (), 1), // +0x407a80 sub_407a80
    Lifted::new(0x7000af3aa0, tpl_414::<0x20, 0x8, 0xfffffffffffffff8, 0x700177c312, 0x18, 0x7001d54968, 0x7000af3b00> as *const (), 0), // +0x407aa0 sub_407aa0
    Lifted::new(0x7000af3b20, fn_7000af3b20 as *const (), 2), // +0x407b20 sub_407b20
    Lifted::new(0x7000af3c80, fn_7000af3c80 as *const (), 0), // +0x407c80 sub_407c80
    Lifted::new(0x7000af3dc0, fn_7000af3dc0 as *const (), 0), // +0x407dc0 sub_407dc0
    Lifted::new(0x7000af3e20, tpl_360::<0x10, 0x4> as *const (), 1), // +0x407e20 sub_407e20
    Lifted::new(0x7000af3e30, fn_7000af3e30 as *const (), 1), // +0x407e30 sub_407e30
    Lifted::new(0x7000af3e40, fn_7000af3e40 as *const (), 1), // +0x407e40 sub_407e40
    Lifted::new(0x7000af3e60, fn_7000af3e60 as *const (), 1), // +0x407e60 sub_407e60
    Lifted::new(0x7000af3e80, fn_7000af3e80 as *const (), 0), // +0x407e80 sub_407e80
    Lifted::new(0x7000af3f10, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000af3f20> as *const (), 0), // +0x407f10 sub_407f10
    Lifted::new(0x7000af3f90, tpl_39::<0x8> as *const (), 1), // +0x407f90 sub_407f90
    Lifted::new(0x7000af3fa0, fn_7000af3fa0 as *const (), 0), // +0x407fa0 sub_407fa0
    Lifted::new(0x7000af4070, fn_7000af4070 as *const (), 0), // +0x408070 sub_408070
    Lifted::new(0x7000af40a0, fn_7000af40a0 as *const (), 0), // +0x4080a0 sub_4080a0
    Lifted::new(0x7000af4220, fn_7000af4220 as *const (), 0), // +0x408220 sub_408220
    Lifted::new(0x7000af43a0, fn_7000af43a0 as *const (), 0), // +0x4083a0 sub_4083a0
    Lifted::new(0x7000af45b0, tpl_798::<0xaf40, 0x7000aecc30, 0x7001c55fa0, 0x8> as *const (), 0), // +0x4085b0 sub_4085b0
    Lifted::new(0x7000af4600, fn_7000af4600 as *const (), 0), // +0x408600 sub_408600
    Lifted::new(0x7000af4630, fn_7000af4630 as *const (), 0), // +0x408630 sub_408630
    Lifted::new(0x7000af4660, tpl_798::<0xaf40, 0x7000aecca0, 0x7001c55fa0, 0x8> as *const (), 0), // +0x408660 sub_408660
    Lifted::new(0x7000af46b0, fn_7000af46b0 as *const (), 0), // +0x4086b0 sub_4086b0
    Lifted::new(0x7000af46e0, fn_7000af46e0 as *const (), 0), // +0x4086e0 sub_4086e0
    Lifted::new(0x7000af4710, fn_7000af4710 as *const (), 0), // +0x408710 sub_408710
    Lifted::new(0x7000af4740, fn_7000af4740 as *const (), 0), // +0x408740 sub_408740
    Lifted::new(0x7000af4770, fn_7000af4770 as *const (), 0), // +0x408770 sub_408770
    Lifted::new(0x7000af47f0, fn_7000af47f0 as *const (), 0), // +0x4087f0 sub_4087f0
    Lifted::new(0x7000af4820, fn_7000af4820 as *const (), 0), // +0x408820 sub_408820
    Lifted::new(0x7000af4850, fn_7000af4850 as *const (), 0), // +0x408850 sub_408850
    Lifted::new(0x7000af4880, fn_7000af4880 as *const (), 0), // +0x408880 sub_408880
    Lifted::new(0x7000af48b0, fn_7000af48b0 as *const (), 0), // +0x4088b0 sub_4088b0
    Lifted::new(0x7000af48e0, fn_7000af48e0 as *const (), 0), // +0x4088e0 sub_4088e0
    Lifted::new(0x7000af4910, fn_7000af4910 as *const (), 0), // +0x408910 sub_408910
    Lifted::new(0x7000af4980, fn_7000af4980 as *const (), 0), // +0x408980 sub_408980
    Lifted::new(0x7000af4a30, fn_7000af4a30 as *const (), 0), // +0x408a30 sub_408a30
    Lifted::new(0x7000af4a60, fn_7000af4a60 as *const (), 0), // +0x408a60 sub_408a60
    Lifted::new(0x7000af4a90, fn_7000af4a90 as *const (), 0), // +0x408a90 sub_408a90
    Lifted::new(0x7000af4b20, fn_7000af4b20 as *const (), 0), // +0x408b20 sub_408b20
    Lifted::new(0x7000af4ba0, fn_7000af4ba0 as *const (), 2), // +0x408ba0 sub_408ba0
    Lifted::new(0x7000af4bf0, fn_7000af4bf0 as *const (), 2), // +0x408bf0 sub_408bf0
    Lifted::new(0x7000af4c50, tpl_798::<0xaf40, 0x7000af0750, 0x7001c56b08, 0x8> as *const (), 0), // +0x408c50 sub_408c50
    Lifted::new(0x7000af4ca0, tpl_798::<0xaf40, 0x7000af07c0, 0x7001c56b08, 0x8> as *const (), 0), // +0x408ca0 sub_408ca0
    Lifted::new(0x7000af4cf0, fn_7000af4cf0 as *const (), 0), // +0x408cf0 sub_408cf0
    Lifted::new(0x7000af4d20, fn_7000af4d20 as *const (), 0), // +0x408d20 sub_408d20
    Lifted::new(0x7000af4d80, fn_7000af4d80 as *const (), 0), // +0x408d80 sub_408d80
    Lifted::new(0x7000af4db0, fn_7000af4db0 as *const (), 0), // +0x408db0 sub_408db0
    Lifted::new(0x7000af4de0, fn_7000af4de0 as *const (), 0), // +0x408de0 sub_408de0
    Lifted::new(0x7000af4e60, fn_7000af4e60 as *const (), 0), // +0x408e60 sub_408e60
    Lifted::new(0x7000af4f20, fn_7000af4f20 as *const (), 0), // +0x408f20 sub_408f20
    Lifted::new(0x7000af4f90, fn_7000af4f90 as *const (), 0), // +0x408f90 sub_408f90
    Lifted::new(0x7000af5070, fn_7000af5070 as *const (), 0), // +0x409070 sub_409070
    Lifted::new(0x7000af5110, fn_7000af5110 as *const (), 0), // +0x409110 sub_409110
    Lifted::new(0x7000af51a0, fn_7000af51a0 as *const (), 0), // +0x4091a0 sub_4091a0
    Lifted::new(0x7000af5230, fn_7000af5230 as *const (), 0), // +0x409230 sub_409230
    Lifted::new(0x7000af52d0, fn_7000af52d0 as *const (), 0), // +0x4092d0 sub_4092d0
    Lifted::new(0x7000af5350, fn_7000af5350 as *const (), 0), // +0x409350 sub_409350
    Lifted::new(0x7000af53d0, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x4093d0 sub_4093d0
    Lifted::new(0x7000af54e0, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x4094e0 sub_4094e0
    Lifted::new(0x7000af55f0, fn_7000af55f0 as *const (), 1), // +0x4095f0 sub_4095f0
    Lifted::new(0x7000af5720, tpl_20 as *const (), 1), // +0x409720 sub_409720
    Lifted::new(0x7000af5730, tpl_20 as *const (), 1), // +0x409730 sub_409730
    Lifted::new(0x7000af5740, tpl_799::<0x8, 0xa0, 0x18, 0x700079cb70, 0x7000dbf940> as *const (), 0), // +0x409740 sub_409740
    Lifted::new(0x7000af5790, fn_7000af5790 as *const (), 1), // +0x409790 sub_409790
    Lifted::new(0x7000af57f0, fn_7000af57f0 as *const (), 0), // +0x4097f0 sub_4097f0
    Lifted::new(0x7000af58b0, tpl_799::<0x8, 0xa0, 0x18, 0x700079cb70, 0x7000dbf940> as *const (), 0), // +0x4098b0 sub_4098b0
    Lifted::new(0x7000af5900, fn_7000af5900 as *const (), 1), // +0x409900 sub_409900
    Lifted::new(0x7000af5950, fn_7000af5950 as *const (), 1), // +0x409950 sub_409950
    Lifted::new(0x7000af5990, fn_7000af5990 as *const (), 1), // +0x409990 sub_409990
    Lifted::new(0x7000af5a00, tpl_171::<0xc8> as *const (), 1), // +0x409a00 sub_409a00
    Lifted::new(0x7000af5a10, fn_7000af5a10 as *const (), 0), // +0x409a10 sub_409a10
    Lifted::new(0x7000af5b00, fn_7000af5b00 as *const (), 0), // +0x409b00 sub_409b00
    Lifted::new(0x7000af5e90, fn_7000af5e90 as *const (), 1), // +0x409e90 sub_409e90
    Lifted::new(0x7000af5eb0, fn_7000af5eb0 as *const (), 0), // +0x409eb0 sub_409eb0
    Lifted::new(0x7000af5f30, fn_7000af5f30 as *const (), 0), // +0x409f30 sub_409f30
    Lifted::new(0x7000af5f70, fn_7000af5f70 as *const (), 2), // +0x409f70 sub_409f70
    Lifted::new(0x7000af6140, fn_7000af6140 as *const (), 0), // +0x40a140 sub_40a140
    Lifted::new(0x7000af6530, fn_7000af6530 as *const (), 0), // +0x40a530 sub_40a530
    Lifted::new(0x7000af67b0, fn_7000af67b0 as *const (), 2), // +0x40a7b0 sub_40a7b0
    Lifted::new(0x7000af69d0, fn_7000af69d0 as *const (), 0), // +0x40a9d0 sub_40a9d0
    Lifted::new(0x7000af6a00, fn_7000af6a00 as *const (), 0), // +0x40aa00 sub_40aa00
    Lifted::new(0x7000af6a60, fn_7000af6a60 as *const (), 0), // +0x40aa60 sub_40aa60
    Lifted::new(0x7000af6aa0, fn_7000af6aa0 as *const (), 0), // +0x40aaa0 sub_40aaa0
    Lifted::new(0x7000af6bb0, fn_7000af6bb0 as *const (), 0), // +0x40abb0 sub_40abb0
    Lifted::new(0x7000af6c90, fn_7000af6c90 as *const (), 0), // +0x40ac90 sub_40ac90
    Lifted::new(0x7000af6d80, fn_7000af6d80 as *const (), 0), // +0x40ad80 sub_40ad80
    Lifted::new(0x7000af6fc0, fn_7000af6fc0 as *const (), 0), // +0x40afc0 sub_40afc0
    Lifted::new(0x7000af6fd0, fn_7000af6fd0 as *const (), 0), // +0x40afd0 sub_40afd0
    Lifted::new(0x7000af7120, fn_7000af7120 as *const (), 0), // +0x40b120 sub_40b120
    Lifted::new(0x7000af7190, fn_7000af7190 as *const (), 0), // +0x40b190 sub_40b190
    Lifted::new(0x7000af7360, tpl_30 as *const (), 1), // +0x40b360 sub_40b360
    Lifted::new(0x7000af7370, fn_7000af7370 as *const (), 1), // +0x40b370 sub_40b370
    Lifted::new(0x7000af7390, tpl_70 as *const (), 1), // +0x40b390 sub_40b390
    Lifted::new(0x7000af73a0, fn_7000af73a0 as *const (), 0), // +0x40b3a0 sub_40b3a0
    Lifted::new(0x7000af7530, fn_7000af7530 as *const (), 0), // +0x40b530 sub_40b530
    Lifted::new(0x7000af7670, fn_7000af7670 as *const (), 0), // +0x40b670 sub_40b670
    Lifted::new(0x7000af78c0, fn_7000af78c0 as *const (), 0), // +0x40b8c0 sub_40b8c0
    Lifted::new(0x7000af79a0, fn_7000af79a0 as *const (), 0), // +0x40b9a0 sub_40b9a0
    Lifted::new(0x7000af7af0, fn_7000af7af0 as *const (), 0), // +0x40baf0 sub_40baf0
    Lifted::new(0x7000af7dc0, fn_7000af7dc0 as *const (), 0), // +0x40bdc0 sub_40bdc0
    Lifted::new(0x7000af80b0, fn_7000af80b0 as *const (), 2), // +0x40c0b0 sub_40c0b0
    Lifted::new(0x7000af8340, fn_7000af8340 as *const (), 2), // +0x40c340 sub_40c340
    Lifted::new(0x7000af85e0, fn_7000af85e0 as *const (), 2), // +0x40c5e0 sub_40c5e0
    Lifted::new(0x7000af8c40, fn_7000af8c40 as *const (), 2), // +0x40cc40 sub_40cc40
    Lifted::new(0x7000af9370, tpl_800::<0x70018da5bc, 0x700177aa00, 0x7000724770, 0x18, 0x70017d8c38, 0x7000723490, 0x7000af93ec> as *const (), 0), // +0x40d370 sub_40d370
    Lifted::new(0x7000af9420, fn_7000af9420 as *const (), 0), // +0x40d420 sub_40d420
    Lifted::new(0x7000af94b0, fn_7000af94b0 as *const (), 2), // +0x40d4b0 sub_40d4b0
    Lifted::new(0x7000af96f0, fn_7000af96f0 as *const (), 0), // +0x40d6f0 sub_40d6f0
    Lifted::new(0x7000af9810, fn_7000af9810 as *const (), 2), // +0x40d810 sub_40d810
    Lifted::new(0x7000af9a70, tpl_20 as *const (), 1), // +0x40da70 sub_40da70
    Lifted::new(0x7000af9a80, tpl_20 as *const (), 1), // +0x40da80 sub_40da80
    Lifted::new(0x7000af9a90, fn_7000af9a90 as *const (), 0), // +0x40da90 sub_40da90
    Lifted::new(0x7000af9b30, fn_7000af9b30 as *const (), 0), // +0x40db30 sub_40db30
    Lifted::new(0x7000af9c10, fn_7000af9c10 as *const (), 0), // +0x40dc10 sub_40dc10
    Lifted::new(0x7000af9ca0, tpl_395::<0x7000b092a0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000af9de5, 0x7000a36d80, 0x7000af9deb> as *const (), 0), // +0x40dca0 sub_40dca0
    Lifted::new(0x7000af9e40, tpl_20 as *const (), 1), // +0x40de40 sub_40de40
    Lifted::new(0x7000af9e50, tpl_20 as *const (), 1), // +0x40de50 sub_40de50
    Lifted::new(0x7000af9e60, tpl_20 as *const (), 1), // +0x40de60 sub_40de60
    Lifted::new(0x7000af9e70, tpl_396::<0x7001779e20, 0x70007031a0> as *const (), 0), // +0x40de70 sub_40de70
    Lifted::new(0x7000af9ef0, fn_7000af9ef0 as *const (), 2), // +0x40def0 sub_40def0
    Lifted::new(0x7000af9f90, tpl_20 as *const (), 1), // +0x40df90 sub_40df90
    Lifted::new(0x7000af9fa0, fn_7000af9fa0 as *const (), 2), // +0x40dfa0 sub_40dfa0
    Lifted::new(0x7000afa1c0, fn_7000afa1c0 as *const (), 2), // +0x40e1c0 sub_40e1c0
    Lifted::new(0x7000afa4d0, fn_7000afa4d0 as *const (), 0), // +0x40e4d0 sub_40e4d0
    Lifted::new(0x7000afa590, fn_7000afa590 as *const (), 0), // +0x40e590 sub_40e590
    Lifted::new(0x7000afa5c0, fn_7000afa5c0 as *const (), 0), // +0x40e5c0 sub_40e5c0
    Lifted::new(0x7000afa660, fn_7000afa660 as *const (), 0), // +0x40e660 sub_40e660
    Lifted::new(0x7000afa730, fn_7000afa730 as *const (), 0), // +0x40e730 sub_40e730
    Lifted::new(0x7000afa910, fn_7000afa910 as *const (), 0), // +0x40e910 sub_40e910
    Lifted::new(0x7000afa960, fn_7000afa960 as *const (), 0), // +0x40e960 sub_40e960
    Lifted::new(0x7000afaab0, fn_7000afaab0 as *const (), 0), // +0x40eab0 sub_40eab0
    Lifted::new(0x7000afab00, fn_7000afab00 as *const (), 3), // +0x40eb00 sub_40eb00
    Lifted::new(0x7000afaba0, fn_7000afaba0 as *const (), 2), // +0x40eba0 sub_40eba0
    Lifted::new(0x7000afac10, fn_7000afac10 as *const (), 0), // +0x40ec10 sub_40ec10
    Lifted::new(0x7000afac60, fn_7000afac60 as *const (), 0), // +0x40ec60 sub_40ec60
    Lifted::new(0x7000afad20, fn_7000afad20 as *const (), 0), // +0x40ed20 sub_40ed20
    Lifted::new(0x7000afae90, fn_7000afae90 as *const (), 0), // +0x40ee90 sub_40ee90
    Lifted::new(0x7000afaf50, fn_7000afaf50 as *const (), 0), // +0x40ef50 sub_40ef50
    Lifted::new(0x7000afb010, fn_7000afb010 as *const (), 0), // +0x40f010 sub_40f010
    Lifted::new(0x7000afb3c0, tpl_801::<0x10, 0x7000749860, 0x700177c312> as *const (), 0), // +0x40f3c0 sub_40f3c0
    Lifted::new(0x7000afb420, tpl_107::<0x40, 0x48, 0x50> as *const (), 1), // +0x40f420 sub_40f420
    Lifted::new(0x7000afb440, tpl_30 as *const (), 1), // +0x40f440 sub_40f440
    Lifted::new(0x7000afb450, tpl_420::<0x20, 0x70017d3d30, 0x8, 0x700075ca60, 0x700177c312> as *const (), 0), // +0x40f450 sub_40f450
    Lifted::new(0x7000afb4b0, tpl_795::<0x10, 0x700177c312> as *const (), 0), // +0x40f4b0 sub_40f4b0
    Lifted::new(0x7000afb500, fn_7000afb500 as *const (), 0), // +0x40f500 sub_40f500
    Lifted::new(0x7000afb550, tpl_802::<0x18, 0x700078d640, 0x8> as *const (), 0), // +0x40f550 sub_40f550
    Lifted::new(0x7000afb580, fn_7000afb580 as *const (), 0), // +0x40f580 sub_40f580
    Lifted::new(0x7000afb750, fn_7000afb750 as *const (), 0), // +0x40f750 sub_40f750
    Lifted::new(0x7000afb800, tpl_244::<0x10> as *const (), 1), // +0x40f800 sub_40f800
    Lifted::new(0x7000afb810, tpl_244::<0x20> as *const (), 1), // +0x40f810 sub_40f810
    Lifted::new(0x7000afb820, fn_7000afb820 as *const (), 2), // +0x40f820 sub_40f820
    Lifted::new(0x7000afbc10, fn_7000afbc10 as *const (), 2), // +0x40fc10 sub_40fc10
    Lifted::new(0x7000afc3d0, fn_7000afc3d0 as *const (), 2), // +0x4103d0 sub_4103d0
    Lifted::new(0x7000afc860, fn_7000afc860 as *const (), 2), // +0x410860 sub_410860
    Lifted::new(0x7000afd1c0, fn_7000afd1c0 as *const (), 2), // +0x4111c0 sub_4111c0
    Lifted::new(0x7000afd5c0, fn_7000afd5c0 as *const (), 0), // +0x4115c0 sub_4115c0
    Lifted::new(0x7000afd710, fn_7000afd710 as *const (), 0), // +0x411710 sub_411710
    Lifted::new(0x7000afd750, fn_7000afd750 as *const (), 2), // +0x411750 sub_411750
    Lifted::new(0x7000afda50, fn_7000afda50 as *const (), 0), // +0x411a50 sub_411a50
    Lifted::new(0x7000afdba0, fn_7000afdba0 as *const (), 2), // +0x411ba0 sub_411ba0
    Lifted::new(0x7000afe090, fn_7000afe090 as *const (), 2), // +0x412090 sub_412090
    Lifted::new(0x7000afe2d0, fn_7000afe2d0 as *const (), 2), // +0x4122d0 sub_4122d0
    Lifted::new(0x7000afe940, fn_7000afe940 as *const (), 2), // +0x412940 sub_412940
    Lifted::new(0x7000aff1f0, fn_7000aff1f0 as *const (), 0), // +0x4131f0 sub_4131f0
    Lifted::new(0x7000aff230, fn_7000aff230 as *const (), 0), // +0x413230 sub_413230
    Lifted::new(0x7000aff500, fn_7000aff500 as *const (), 0), // +0x413500 sub_413500
    Lifted::new(0x7000aff520, tpl_803::<0xb8> as *const (), 1), // +0x413520 sub_413520
    Lifted::new(0x7000aff540, tpl_803::<0xb0> as *const (), 1), // +0x413540 sub_413540
    Lifted::new(0x7000aff560, tpl_483::<0x7001c5d441> as *const (), 1), // +0x413560 sub_413560
    Lifted::new(0x7000aff570, fn_7000aff570 as *const (), 0), // +0x413570 sub_413570
    Lifted::new(0x7000aff610, fn_7000aff610 as *const (), 0), // +0x413610 sub_413610
    Lifted::new(0x7000aff690, fn_7000aff690 as *const (), 0), // +0x413690 sub_413690
    Lifted::new(0x7000aff7f0, fn_7000aff7f0 as *const (), 0), // +0x4137f0 sub_4137f0
    Lifted::new(0x7000aff8d0, fn_7000aff8d0 as *const (), 0), // +0x4138d0 sub_4138d0
    Lifted::new(0x7000affa30, fn_7000affa30 as *const (), 2), // +0x413a30 sub_413a30
    Lifted::new(0x7000affe70, fn_7000affe70 as *const (), 0), // +0x413e70 sub_413e70
    Lifted::new(0x7000afff10, fn_7000afff10 as *const (), 0), // +0x413f10 sub_413f10
    Lifted::new(0x7000afffd0, fn_7000afffd0 as *const (), 0), // +0x413fd0 sub_413fd0
    Lifted::new(0x7000b00070, fn_7000b00070 as *const (), 2), // +0x414070 sub_414070
    Lifted::new(0x7000b00730, fn_7000b00730 as *const (), 0), // +0x414730 sub_414730
    Lifted::new(0x7000b00930, fn_7000b00930 as *const (), 0), // +0x414930 sub_414930
    Lifted::new(0x7000b00a40, fn_7000b00a40 as *const (), 0), // +0x414a40 sub_414a40
    Lifted::new(0x7000b00a90, fn_7000b00a90 as *const (), 2), // +0x414a90 sub_414a90
    Lifted::new(0x7000b00df0, fn_7000b00df0 as *const (), 1), // +0x414df0 sub_414df0
    Lifted::new(0x7000b00e00, fn_7000b00e00 as *const (), 2), // +0x414e00 sub_414e00
    Lifted::new(0x7000b01840, fn_7000b01840 as *const (), 0), // +0x415840 sub_415840
    Lifted::new(0x7000b018e0, fn_7000b018e0 as *const (), 0), // +0x4158e0 sub_4158e0
    Lifted::new(0x7000b01b00, fn_7000b01b00 as *const (), 1), // +0x415b00 sub_415b00
    Lifted::new(0x7000b01b30, fn_7000b01b30 as *const (), 2), // +0x415b30 sub_415b30
    Lifted::new(0x7000b02570, fn_7000b02570 as *const (), 0), // +0x416570 sub_416570
    Lifted::new(0x7000b025d0, fn_7000b025d0 as *const (), 2), // +0x4165d0 sub_4165d0
    Lifted::new(0x7000b02af0, fn_7000b02af0 as *const (), 2), // +0x416af0 sub_416af0
    Lifted::new(0x7000b02fa0, fn_7000b02fa0 as *const (), 0), // +0x416fa0 sub_416fa0
    Lifted::new(0x7000b03070, tpl_39::<0x4> as *const (), 1), // +0x417070 sub_417070
    Lifted::new(0x7000b03080, fn_7000b03080 as *const (), 0), // +0x417080 sub_417080
    Lifted::new(0x7000b03110, fn_7000b03110 as *const (), 2), // +0x417110 sub_417110
    Lifted::new(0x7000b037b0, fn_7000b037b0 as *const (), 2), // +0x4177b0 sub_4177b0
    Lifted::new(0x7000b04500, fn_7000b04500 as *const (), 2), // +0x418500 sub_418500
    Lifted::new(0x7000b04db0, fn_7000b04db0 as *const (), 0), // +0x418db0 sub_418db0
    Lifted::new(0x7000b04e00, fn_7000b04e00 as *const (), 1), // +0x418e00 sub_418e00
    Lifted::new(0x7000b04e10, fn_7000b04e10 as *const (), 0), // +0x418e10 sub_418e10
    Lifted::new(0x7000b05140, fn_7000b05140 as *const (), 2), // +0x419140 sub_419140
    Lifted::new(0x7000b05660, fn_7000b05660 as *const (), 1), // +0x419660 sub_419660
    Lifted::new(0x7000b05670, fn_7000b05670 as *const (), 0), // +0x419670 sub_419670
    Lifted::new(0x7000b056c0, fn_7000b056c0 as *const (), 1), // +0x4196c0 sub_4196c0
    Lifted::new(0x7000b056d0, fn_7000b056d0 as *const (), 1), // +0x4196d0 sub_4196d0
    Lifted::new(0x7000b056e0, fn_7000b056e0 as *const (), 2), // +0x4196e0 sub_4196e0
    Lifted::new(0x7000b05d10, fn_7000b05d10 as *const (), 2), // +0x419d10 sub_419d10
    Lifted::new(0x7000b05e70, fn_7000b05e70 as *const (), 0), // +0x419e70 sub_419e70
    Lifted::new(0x7000b06370, fn_7000b06370 as *const (), 0), // +0x41a370 sub_41a370
    Lifted::new(0x7000b065c0, fn_7000b065c0 as *const (), 0), // +0x41a5c0 sub_41a5c0
    Lifted::new(0x7000b06710, fn_7000b06710 as *const (), 2), // +0x41a710 sub_41a710
    Lifted::new(0x7000b06c80, fn_7000b06c80 as *const (), 2), // +0x41ac80 sub_41ac80
    Lifted::new(0x7000b06df0, fn_7000b06df0 as *const (), 0), // +0x41adf0 sub_41adf0
    Lifted::new(0x7000b070a0, fn_7000b070a0 as *const (), 2), // +0x41b0a0 sub_41b0a0
    Lifted::new(0x7000b073f0, fn_7000b073f0 as *const (), 2), // +0x41b3f0 sub_41b3f0
    Lifted::new(0x7000b07580, fn_7000b07580 as *const (), 0), // +0x41b580 sub_41b580
    Lifted::new(0x7000b075c0, fn_7000b075c0 as *const (), 0), // +0x41b5c0 sub_41b5c0
    Lifted::new(0x7000b07930, tpl_802::<0x18, 0x70007852e0, 0x8> as *const (), 0), // +0x41b930 sub_41b930
    Lifted::new(0x7000b07960, fn_7000b07960 as *const (), 2), // +0x41b960 sub_41b960
    Lifted::new(0x7000b081f0, fn_7000b081f0 as *const (), 0), // +0x41c1f0 sub_41c1f0
    Lifted::new(0x7000b084a0, tpl_488::<0x38, 0x10, 0x7000752be0> as *const (), 0), // +0x41c4a0 sub_41c4a0
    Lifted::new(0x7000b084b0, tpl_51::<0x30> as *const (), 1), // +0x41c4b0 sub_41c4b0
    Lifted::new(0x7000b084c0, tpl_51::<0x28> as *const (), 1), // +0x41c4c0 sub_41c4c0
    Lifted::new(0x7000b084d0, fn_7000b084d0 as *const (), 1), // +0x41c4d0 sub_41c4d0
    Lifted::new(0x7000b084e0, tpl_51::<0x58> as *const (), 1), // +0x41c4e0 sub_41c4e0
    Lifted::new(0x7000b084f0, tpl_245::<0x4> as *const (), 1), // +0x41c4f0 sub_41c4f0
    Lifted::new(0x7000b08500, fn_7000b08500 as *const (), 0), // +0x41c500 sub_41c500
    Lifted::new(0x7000b08710, fn_7000b08710 as *const (), 2), // +0x41c710 sub_41c710
    Lifted::new(0x7000b08980, fn_7000b08980 as *const (), 0), // +0x41c980 sub_41c980
    Lifted::new(0x7000b08a60, fn_7000b08a60 as *const (), 1), // +0x41ca60 sub_41ca60
    Lifted::new(0x7000b08b10, fn_7000b08b10 as *const (), 0), // +0x41cb10 sub_41cb10
    Lifted::new(0x7000b08cb0, fn_7000b08cb0 as *const (), 0), // +0x41ccb0 sub_41ccb0
    Lifted::new(0x7000b08d70, fn_7000b08d70 as *const (), 2), // +0x41cd70 sub_41cd70
    Lifted::new(0x7000b090f0, fn_7000b090f0 as *const (), 0), // +0x41d0f0 sub_41d0f0
    Lifted::new(0x7000b091f0, tpl_20 as *const (), 1), // +0x41d1f0 sub_41d1f0
    Lifted::new(0x7000b09200, fn_7000b09200 as *const (), 1), // +0x41d200 sub_41d200
    Lifted::new(0x7000b09260, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b09270> as *const (), 0), // +0x41d260 sub_41d260
    Lifted::new(0x7000b09280, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b09290> as *const (), 0), // +0x41d280 sub_41d280
    Lifted::new(0x7000b092a0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b092f4, 0x70007f0910, 0x7000b0930f> as *const (), 0), // +0x41d2a0 sub_41d2a0
    Lifted::new(0x7000b09330, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b0937d, 0x70007f0910, 0x7000b0939b> as *const (), 0), // +0x41d330 sub_41d330
    Lifted::new(0x7000b093c0, tpl_300::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b0940d, 0x70007f0910, 0x7000b0942b> as *const (), 0), // +0x41d3c0 sub_41d3c0
    Lifted::new(0x7000b09450, tpl_346 as *const (), 1), // +0x41d450 sub_41d450
    Lifted::new(0x7000b09460, tpl_319 as *const (), 1), // +0x41d460 sub_41d460
    Lifted::new(0x7000b09470, tpl_191::<0x30> as *const (), 1), // +0x41d470 sub_41d470
    Lifted::new(0x7000b09480, tpl_351 as *const (), 1), // +0x41d480 sub_41d480
    Lifted::new(0x7000b09490, tpl_47::<0x60> as *const (), 1), // +0x41d490 sub_41d490
    Lifted::new(0x7000b094a0, tpl_39::<0xc> as *const (), 1), // +0x41d4a0 sub_41d4a0
    Lifted::new(0x7000b094b0, tpl_191::<0x10> as *const (), 1), // +0x41d4b0 sub_41d4b0
    Lifted::new(0x7000b094c0, tpl_191::<0x28> as *const (), 1), // +0x41d4c0 sub_41d4c0
    Lifted::new(0x7000b094d0, tpl_326 as *const (), 1), // +0x41d4d0 sub_41d4d0
    Lifted::new(0x7000b094e0, tpl_191::<0x20> as *const (), 1), // +0x41d4e0 sub_41d4e0
    Lifted::new(0x7000b094f0, tpl_191::<0x28> as *const (), 1), // +0x41d4f0 sub_41d4f0
    Lifted::new(0x7000b09500, tpl_39::<0x8> as *const (), 1), // +0x41d500 sub_41d500
    Lifted::new(0x7000b09510, tpl_36::<0x28> as *const (), 1), // +0x41d510 sub_41d510
    Lifted::new(0x7000b09520, tpl_302 as *const (), 1), // +0x41d520 sub_41d520
    Lifted::new(0x7000b09530, tpl_191::<0x20> as *const (), 1), // +0x41d530 sub_41d530
    Lifted::new(0x7000b09540, tpl_348 as *const (), 1), // +0x41d540 sub_41d540
    Lifted::new(0x7000b09550, tpl_352 as *const (), 1), // +0x41d550 sub_41d550
    Lifted::new(0x7000b09560, tpl_52::<0x38> as *const (), 1), // +0x41d560 sub_41d560
    Lifted::new(0x7000b09570, tpl_47::<0x74> as *const (), 1), // +0x41d570 sub_41d570
    Lifted::new(0x7000b09580, tpl_20 as *const (), 1), // +0x41d580 sub_41d580
    Lifted::new(0x7000b09590, tpl_194 as *const (), 1), // +0x41d590 sub_41d590
    Lifted::new(0x7000b095a0, tpl_172::<0x30> as *const (), 1), // +0x41d5a0 sub_41d5a0
    Lifted::new(0x7000b095b0, tpl_172::<0x10> as *const (), 1), // +0x41d5b0 sub_41d5b0
    Lifted::new(0x7000b095c0, tpl_172::<0x28> as *const (), 1), // +0x41d5c0 sub_41d5c0
    Lifted::new(0x7000b095d0, tpl_172::<0x20> as *const (), 1), // +0x41d5d0 sub_41d5d0
    Lifted::new(0x7000b095e0, tpl_172::<0x18> as *const (), 1), // +0x41d5e0 sub_41d5e0
    Lifted::new(0x7000b095f0, tpl_172::<0x18> as *const (), 1), // +0x41d5f0 sub_41d5f0
    Lifted::new(0x7000b09600, fn_7000b09600 as *const (), 1), // +0x41d600 sub_41d600
    Lifted::new(0x7000b09610, tpl_39::<0x84> as *const (), 1), // +0x41d610 sub_41d610
    Lifted::new(0x7000b09620, tpl_52::<0x68> as *const (), 1), // +0x41d620 sub_41d620
    Lifted::new(0x7000b09630, tpl_318 as *const (), 1), // +0x41d630 sub_41d630
    Lifted::new(0x7000b09640, tpl_502::<0x40> as *const (), 1), // +0x41d640 sub_41d640
    Lifted::new(0x7000b09650, tpl_36::<0x10> as *const (), 1), // +0x41d650 sub_41d650
    Lifted::new(0x7000b09660, tpl_304 as *const (), 1), // +0x41d660 sub_41d660
    Lifted::new(0x7000b09670, tpl_36::<0x58> as *const (), 1), // +0x41d670 sub_41d670
    Lifted::new(0x7000b09680, tpl_39::<0x80> as *const (), 1), // +0x41d680 sub_41d680
    Lifted::new(0x7000b09690, tpl_36::<0x48> as *const (), 1), // +0x41d690 sub_41d690
    Lifted::new(0x7000b096a0, tpl_185::<0x40> as *const (), 1), // +0x41d6a0 sub_41d6a0
    Lifted::new(0x7000b096b0, tpl_313 as *const (), 1), // +0x41d6b0 sub_41d6b0
    Lifted::new(0x7000b096c0, tpl_36::<0x30> as *const (), 1), // +0x41d6c0 sub_41d6c0
    Lifted::new(0x7000b096d0, fn_7000b096d0 as *const (), 0), // +0x41d6d0 sub_41d6d0
    Lifted::new(0x7000b09910, fn_7000b09910 as *const (), 1), // +0x41d910 sub_41d910
    Lifted::new(0x7000b09920, fn_7000b09920 as *const (), 1), // +0x41d920 sub_41d920
    Lifted::new(0x7000b09930, fn_7000b09930 as *const (), 1), // +0x41d930 sub_41d930
    Lifted::new(0x7000b09940, tpl_245::<0xc> as *const (), 1), // +0x41d940 sub_41d940
    Lifted::new(0x7000b09950, fn_7000b09950 as *const (), 1), // +0x41d950 sub_41d950
    Lifted::new(0x7000b09960, fn_7000b09960 as *const (), 0), // +0x41d960 sub_41d960
    Lifted::new(0x7000b099d0, fn_7000b099d0 as *const (), 1), // +0x41d9d0 sub_41d9d0
    Lifted::new(0x7000b099e0, tpl_51::<0x20> as *const (), 1), // +0x41d9e0 sub_41d9e0
    Lifted::new(0x7000b099f0, fn_7000b099f0 as *const (), 1), // +0x41d9f0 sub_41d9f0
    Lifted::new(0x7000b09a00, fn_7000b09a00 as *const (), 1), // +0x41da00 sub_41da00
    Lifted::new(0x7000b09a10, fn_7000b09a10 as *const (), 0), // +0x41da10 sub_41da10
    Lifted::new(0x7000b09ac0, fn_7000b09ac0 as *const (), 0), // +0x41dac0 sub_41dac0
    Lifted::new(0x7000b09b10, fn_7000b09b10 as *const (), 1), // +0x41db10 sub_41db10
    Lifted::new(0x7000b09b20, tpl_804::<0x3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x41db20 sub_41db20
    Lifted::new(0x7000b09bf0, fn_7000b09bf0 as *const (), 1), // +0x41dbf0 sub_41dbf0
    Lifted::new(0x7000b09c00, tpl_245::<0x84> as *const (), 1), // +0x41dc00 sub_41dc00
    Lifted::new(0x7000b09c10, fn_7000b09c10 as *const (), 1), // +0x41dc10 sub_41dc10
    Lifted::new(0x7000b09c20, tpl_51::<0x10> as *const (), 1), // +0x41dc20 sub_41dc20
    Lifted::new(0x7000b09c30, fn_7000b09c30 as *const (), 1), // +0x41dc30 sub_41dc30
    Lifted::new(0x7000b09c40, tpl_805::<0x70007a58c0, 0x700177c312> as *const (), 0), // +0x41dc40 sub_41dc40
    Lifted::new(0x7000b09cc0, fn_7000b09cc0 as *const (), 1), // +0x41dcc0 sub_41dcc0
    Lifted::new(0x7000b09cd0, tpl_51::<0x30> as *const (), 1), // +0x41dcd0 sub_41dcd0
    Lifted::new(0x7000b09ce0, tpl_806::<0xf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x41dce0 sub_41dce0
    Lifted::new(0x7000b09d90, fn_7000b09d90 as *const (), 1), // +0x41dd90 sub_41dd90
    Lifted::new(0x7000b09da0, tpl_51::<0x18> as *const (), 1), // +0x41dda0 sub_41dda0
    Lifted::new(0x7000b09db0, fn_7000b09db0 as *const (), 1), // +0x41ddb0 sub_41ddb0
    Lifted::new(0x7000b09dc0, fn_7000b09dc0 as *const (), 1), // +0x41ddc0 sub_41ddc0
    Lifted::new(0x7000b09dd0, fn_7000b09dd0 as *const (), 1), // +0x41ddd0 sub_41ddd0
    Lifted::new(0x7000b09de0, fn_7000b09de0 as *const (), 1), // +0x41dde0 sub_41dde0
    Lifted::new(0x7000b09df0, fn_7000b09df0 as *const (), 0), // +0x41ddf0 sub_41ddf0
    Lifted::new(0x7000b09e50, fn_7000b09e50 as *const (), 1), // +0x41de50 sub_41de50
    Lifted::new(0x7000b09e60, fn_7000b09e60 as *const (), 1), // +0x41de60 sub_41de60
    Lifted::new(0x7000b09e70, fn_7000b09e70 as *const (), 1), // +0x41de70 sub_41de70
    Lifted::new(0x7000b09e80, fn_7000b09e80 as *const (), 1), // +0x41de80 sub_41de80
    Lifted::new(0x7000b09e90, fn_7000b09e90 as *const (), 1), // +0x41de90 sub_41de90
    Lifted::new(0x7000b09ea0, tpl_191::<0x18> as *const (), 1), // +0x41dea0 sub_41dea0
    Lifted::new(0x7000b09eb0, tpl_362 as *const (), 1), // +0x41deb0 sub_41deb0
    Lifted::new(0x7000b09ec0, tpl_36::<0x18> as *const (), 1), // +0x41dec0 sub_41dec0
    Lifted::new(0x7000b09ed0, tpl_361 as *const (), 1), // +0x41ded0 sub_41ded0
    Lifted::new(0x7000b09ee0, tpl_324 as *const (), 1), // +0x41dee0 sub_41dee0
    Lifted::new(0x7000b09ef0, tpl_52::<0xa0> as *const (), 1), // +0x41def0 sub_41def0
    Lifted::new(0x7000b09f00, tpl_303 as *const (), 1), // +0x41df00 sub_41df00
    Lifted::new(0x7000b09f10, tpl_193::<0x2> as *const (), 1), // +0x41df10 sub_41df10
    Lifted::new(0x7000b09f20, tpl_36::<0x98> as *const (), 1), // +0x41df20 sub_41df20
    Lifted::new(0x7000b09f30, tpl_341 as *const (), 1), // +0x41df30 sub_41df30
    Lifted::new(0x7000b09f40, tpl_366 as *const (), 1), // +0x41df40 sub_41df40
    Lifted::new(0x7000b09f50, tpl_358 as *const (), 1), // +0x41df50 sub_41df50
    Lifted::new(0x7000b09f60, tpl_350 as *const (), 1), // +0x41df60 sub_41df60
    Lifted::new(0x7000b09f70, tpl_323 as *const (), 1), // +0x41df70 sub_41df70
    Lifted::new(0x7000b09f80, tpl_36::<0x50> as *const (), 1), // +0x41df80 sub_41df80
    Lifted::new(0x7000b09f90, tpl_807::<0x10, 0x8, 0x7000b0a6e0> as *const (), 0), // +0x41df90 sub_41df90
    Lifted::new(0x7000b0a080, tpl_808::<0x10, 0x8, 0x7000b0a6e0> as *const (), 0), // +0x41e080 sub_41e080
    Lifted::new(0x7000b0a1d0, tpl_562::<0x8, 0x10, 0x7000b0a8a0> as *const (), 0), // +0x41e1d0 sub_41e1d0
    Lifted::new(0x7000b0a200, fn_7000b0a200 as *const (), 0), // +0x41e200 sub_41e200
    Lifted::new(0x7000b0a610, fn_7000b0a610 as *const (), 0), // +0x41e610 sub_41e610
    Lifted::new(0x7000b0a6e0, fn_7000b0a6e0 as *const (), 0), // +0x41e6e0 sub_41e6e0
    Lifted::new(0x7000b0a8a0, tpl_809::<0x7001d55060, 0x8, 0x70017d8de0, 0x7001d54ef8, 0x7000b0a8d3, 0x10, 0x18, 0xfffffffffffffff8, 0x14> as *const (), 0), // +0x41e8a0 sub_41e8a0
    Lifted::new(0x7000b0a9a0, tpl_807::<0x10, 0x8, 0x7000b0a6e0> as *const (), 0), // +0x41e9a0 sub_41e9a0
    Lifted::new(0x7000b0ab40, fn_7000b0ab40 as *const (), 0), // +0x41eb40 sub_41eb40
    Lifted::new(0x7000b0ac40, fn_7000b0ac40 as *const (), 1), // +0x41ec40 sub_41ec40
    Lifted::new(0x7000b0ad00, fn_7000b0ad00 as *const (), 0), // +0x41ed00 sub_41ed00
    Lifted::new(0x7000b0ada0, fn_7000b0ada0 as *const (), 0), // +0x41eda0 sub_41eda0
    Lifted::new(0x7000b0ae70, fn_7000b0ae70 as *const (), 0), // +0x41ee70 sub_41ee70
    Lifted::new(0x7000b0af30, fn_7000b0af30 as *const (), 0), // +0x41ef30 sub_41ef30
    Lifted::new(0x7000b0b540, fn_7000b0b540 as *const (), 0), // +0x41f540 sub_41f540
    Lifted::new(0x7000b0c5a0, fn_7000b0c5a0 as *const (), 0), // +0x4205a0 sub_4205a0
    Lifted::new(0x7000b0ca70, fn_7000b0ca70 as *const (), 0), // +0x420a70 sub_420a70
    Lifted::new(0x7000b0cb40, fn_7000b0cb40 as *const (), 0), // +0x420b40 sub_420b40
    Lifted::new(0x7000b0ccd0, fn_7000b0ccd0 as *const (), 0), // +0x420cd0 sub_420cd0
    Lifted::new(0x7000b0cd30, fn_7000b0cd30 as *const (), 0), // +0x420d30 sub_420d30
    Lifted::new(0x7000b0cdb0, fn_7000b0cdb0 as *const (), 0), // +0x420db0 sub_420db0
    Lifted::new(0x7000b0d460, fn_7000b0d460 as *const (), 0), // +0x421460 sub_421460
    Lifted::new(0x7000b0d660, tpl_315::<0x70017f6f60> as *const (), 1), // +0x421660 sub_421660
    Lifted::new(0x7000b0d670, tpl_810::<0x70007848e0, 0x7000772540, 0x700077a190, 0x7000b0f5e0> as *const (), 2), // +0x421670 sub_421670
    Lifted::new(0x7000b0d730, fn_7000b0d730 as *const (), 2), // +0x421730 sub_421730
    Lifted::new(0x7000b0d8a0, tpl_810::<0x700072bcf0, 0x700073e520, 0x70007519a0, 0x7000b0f890> as *const (), 2), // +0x4218a0 sub_4218a0
    Lifted::new(0x7000b0d960, tpl_810::<0x70006ee0d0, 0x70006f7e70, 0x700074e860, 0x7000b0fe60> as *const (), 2), // +0x421960 sub_421960
    Lifted::new(0x7000b0da20, tpl_811::<0x70007848e0, 0xffffffffffffff06, 0x7000772540, 0x700077a190, 0x7000b0f5e0> as *const (), 2), // +0x421a20 sub_421a20
    Lifted::new(0x7000b0daf0, fn_7000b0daf0 as *const (), 2), // +0x421af0 sub_421af0
    Lifted::new(0x7000b0dc60, tpl_811::<0x700072bcf0, 0xffffffffffffff06, 0x700073e520, 0x70007519a0, 0x7000b0f890> as *const (), 2), // +0x421c60 sub_421c60
    Lifted::new(0x7000b0dd30, tpl_811::<0x70006ee0d0, 0xffffffffffffff06, 0x70006f7e70, 0x700074e860, 0x7000b0fe60> as *const (), 2), // +0x421d30 sub_421d30
    Lifted::new(0x7000b0df80, tpl_20 as *const (), 1), // +0x421f80 sub_421f80
    Lifted::new(0x7000b0df90, tpl_20 as *const (), 1), // +0x421f90 sub_421f90
    Lifted::new(0x7000b0dfa0, fn_7000b0dfa0 as *const (), 0), // +0x421fa0 sub_421fa0
    Lifted::new(0x7000b0e020, fn_7000b0e020 as *const (), 1), // +0x422020 sub_422020
    Lifted::new(0x7000b0e040, fn_7000b0e040 as *const (), 0), // +0x422040 sub_422040
    Lifted::new(0x7000b0e0d0, fn_7000b0e0d0 as *const (), 0), // +0x4220d0 sub_4220d0
    Lifted::new(0x7000b0e2b0, fn_7000b0e2b0 as *const (), 2), // +0x4222b0 sub_4222b0
    Lifted::new(0x7000b0e4a0, fn_7000b0e4a0 as *const (), 2), // +0x4224a0 sub_4224a0
    Lifted::new(0x7000b0e680, fn_7000b0e680 as *const (), 2), // +0x422680 sub_422680
    Lifted::new(0x7000b0e840, fn_7000b0e840 as *const (), 2), // +0x422840 sub_422840
    Lifted::new(0x7000b0ea10, fn_7000b0ea10 as *const (), 3), // +0x422a10 sub_422a10
    Lifted::new(0x7000b0ebc0, fn_7000b0ebc0 as *const (), 3), // +0x422bc0 sub_422bc0
    Lifted::new(0x7000b0ed70, fn_7000b0ed70 as *const (), 3), // +0x422d70 sub_422d70
    Lifted::new(0x7000b0eef0, fn_7000b0eef0 as *const (), 3), // +0x422ef0 sub_422ef0
    Lifted::new(0x7000b0f090, fn_7000b0f090 as *const (), 1), // +0x423090 sub_423090
    Lifted::new(0x7000b0f100, tpl_28 as *const (), 1), // +0x423100 sub_423100
    Lifted::new(0x7000b0f110, fn_7000b0f110 as *const (), 0), // +0x423110 sub_423110
    Lifted::new(0x7000b0f160, fn_7000b0f160 as *const (), 0), // +0x423160 sub_423160
    Lifted::new(0x7000b0f1a0, fn_7000b0f1a0 as *const (), 0), // +0x4231a0 sub_4231a0
    Lifted::new(0x7000b0f1f0, fn_7000b0f1f0 as *const (), 0), // +0x4231f0 sub_4231f0
    Lifted::new(0x7000b0f220, tpl_14 as *const (), 1), // +0x423220 sub_423220
    Lifted::new(0x7000b0f230, fn_7000b0f230 as *const (), 0), // +0x423230 sub_423230
    Lifted::new(0x7000b0f3a0, fn_7000b0f3a0 as *const (), 0), // +0x4233a0 sub_4233a0
    Lifted::new(0x7000b0f480, fn_7000b0f480 as *const (), 0), // +0x423480 sub_423480
    Lifted::new(0x7000b0f580, fn_7000b0f580 as *const (), 1), // +0x423580 sub_423580
    Lifted::new(0x7000b0f5a0, tpl_14 as *const (), 1), // +0x4235a0 sub_4235a0
    Lifted::new(0x7000b0f5b0, fn_7000b0f5b0 as *const (), 0), // +0x4235b0 sub_4235b0
    Lifted::new(0x7000b0f5e0, fn_7000b0f5e0 as *const (), 2), // +0x4235e0 sub_4235e0
    Lifted::new(0x7000b0f890, fn_7000b0f890 as *const (), 2), // +0x423890 sub_423890
    Lifted::new(0x7000b0fbb0, fn_7000b0fbb0 as *const (), 2), // +0x423bb0 sub_423bb0
    Lifted::new(0x7000b0fe60, fn_7000b0fe60 as *const (), 2), // +0x423e60 sub_423e60
    Lifted::new(0x7000b10180, fn_7000b10180 as *const (), 0), // +0x424180 sub_424180
    Lifted::new(0x7000b101a0, fn_7000b101a0 as *const (), 0), // +0x4241a0 sub_4241a0
    Lifted::alias(0x7000b101d0, fn_7000ee2c20 as *const (), 0, 0x7f6c20), // +0x4241d0 sub_4241d0 (thunk -> +0x7f6c20)
    Lifted::new(0x7000b101e0, fn_7000b101e0 as *const (), 2), // +0x4241e0 sub_4241e0
    Lifted::new(0x7000b10340, fn_7000b10340 as *const (), 2), // +0x424340 sub_424340
    Lifted::new(0x7000b104f0, fn_7000b104f0 as *const (), 2), // +0x4244f0 sub_4244f0
    Lifted::new(0x7000b10670, fn_7000b10670 as *const (), 2), // +0x424670 sub_424670
    Lifted::new(0x7000b10830, fn_7000b10830 as *const (), 0), // +0x424830 sub_424830
    Lifted::new(0x7000b10930, fn_7000b10930 as *const (), 2), // +0x424930 sub_424930
    Lifted::new(0x7000b10a10, fn_7000b10a10 as *const (), 0), // +0x424a10 sub_424a10
    Lifted::new(0x7000b10a80, tpl_812::<0x20, 0x10, 0xffffffffffffffff, 0xcb78, 0x700177c0e0, 0xcb98, 0xffffffffffffffe0, 0xfffffffffffffff0, 0x40, 0x8, 0x1> as *const (), 2), // +0x424a80 sub_424a80
    Lifted::new(0x7000b10b70, tpl_812::<0x20, 0x10, 0xffffffffffffffff, 0xcb78, 0x700177c0e0, 0xcb98, 0xffffffffffffffe0, 0xfffffffffffffff0, 0x40, 0x8, 0x1> as *const (), 2), // +0x424b70 sub_424b70
    Lifted::new(0x7000b10c60, tpl_813::<0x20, 0x10, 0xffffffffffffffff, 0xcb78, 0x700177c0e0, 0xcb98, 0xffffffffffffffe0, 0xfffffffffffffff0, 0x40, 0x8, 0x2, 0xfffffffffffffffe> as *const (), 2), // +0x424c60 sub_424c60
    Lifted::new(0x7000b10d60, tpl_813::<0x20, 0x10, 0xffffffffffffffff, 0xcb78, 0x700177c0e0, 0xcb98, 0xffffffffffffffe0, 0xfffffffffffffff0, 0x40, 0x8, 0x2, 0xfffffffffffffffe> as *const (), 2), // +0x424d60 sub_424d60
    Lifted::new(0x7000b10e60, tpl_814 as *const (), 1), // +0x424e60 sub_424e60
    Lifted::new(0x7000b11020, tpl_814 as *const (), 1), // +0x425020 sub_425020
    Lifted::new(0x7000b111e0, tpl_815 as *const (), 1), // +0x4251e0 sub_4251e0
    Lifted::new(0x7000b113a0, tpl_815 as *const (), 1), // +0x4253a0 sub_4253a0
    Lifted::new(0x7000b11570, fn_7000b11570 as *const (), 2), // +0x425570 sub_425570
    Lifted::new(0x7000b11630, fn_7000b11630 as *const (), 2), // +0x425630 sub_425630
    Lifted::new(0x7000b11750, fn_7000b11750 as *const (), 2), // +0x425750 sub_425750
    Lifted::new(0x7000b11850, fn_7000b11850 as *const (), 2), // +0x425850 sub_425850
    Lifted::alias(0x7000b11980, tpl_1668::<0x7000716490, 0x7001d550a8> as *const (), 2, 0x838820), // +0x425980 sub_425980 (thunk -> +0x838820)
    Lifted::alias(0x7000b11990, tpl_1668::<0x70007701e0, 0x7001d550a8> as *const (), 2, 0x838cb0), // +0x425990 sub_425990 (thunk -> +0x838cb0)
    Lifted::new(0x7000b119a0, tpl_7::<0x700073f9a0> as *const (), 1), // +0x4259a0 sub_4259a0
    Lifted::new(0x7000b119c0, fn_7000b119c0 as *const (), 0), // +0x4259c0 sub_4259c0
    Lifted::new(0x7000b11a20, tpl_7::<0x7001c5d3f1> as *const (), 1), // +0x425a20 sub_425a20
    Lifted::new(0x7000b11a40, tpl_7::<0x70017f86e0> as *const (), 1), // +0x425a40 sub_425a40
    Lifted::new(0x7000b11a60, tpl_7::<0x70017f8700> as *const (), 1), // +0x425a60 sub_425a60
    Lifted::new(0x7000b11a80, tpl_7::<0x7001c7227c> as *const (), 1), // +0x425a80 sub_425a80
    Lifted::new(0x7000b11aa0, tpl_7::<0x70017f86a0> as *const (), 1), // +0x425aa0 sub_425aa0
    Lifted::new(0x7000b11ac0, tpl_7::<0x70017f86c0> as *const (), 1), // +0x425ac0 sub_425ac0
    Lifted::new(0x7000b11ae0, fn_7000b11ae0 as *const (), 1), // +0x425ae0 sub_425ae0
    Lifted::new(0x7000b11b00, tpl_37::<0x60> as *const (), 1), // +0x425b00 sub_425b00
    Lifted::new(0x7000b11b10, tpl_816::<0x7000796450> as *const (), 0), // +0x425b10 sub_425b10
    Lifted::new(0x7000b11b40, tpl_7::<0x70017f8660> as *const (), 1), // +0x425b40 sub_425b40
    Lifted::new(0x7000b11b60, tpl_7::<0x7001c722f8> as *const (), 1), // +0x425b60 sub_425b60
    Lifted::new(0x7000b11b80, tpl_7::<0x70017f8670> as *const (), 1), // +0x425b80 sub_425b80
    Lifted::new(0x7000b11ba0, tpl_37::<0x3030> as *const (), 1), // +0x425ba0 sub_425ba0
    Lifted::new(0x7000b11bc0, fn_7000b11bc0 as *const (), 1), // +0x425bc0 sub_425bc0
    Lifted::new(0x7000b11be0, tpl_37::<0x50> as *const (), 1), // +0x425be0 sub_425be0
    Lifted::new(0x7000b11bf0, fn_7000b11bf0 as *const (), 1), // +0x425bf0 sub_425bf0
    Lifted::new(0x7000b11c10, fn_7000b11c10 as *const (), 1), // +0x425c10 sub_425c10
    Lifted::new(0x7000b11c30, tpl_7::<0x7001c73174> as *const (), 1), // +0x425c30 sub_425c30
    Lifted::new(0x7000b11c50, tpl_37::<0xc978> as *const (), 1), // +0x425c50 sub_425c50
    Lifted::new(0x7000b11c70, tpl_816::<0x700074ccf0> as *const (), 0), // +0x425c70 sub_425c70
    Lifted::new(0x7000b11ca0, tpl_7::<0x70017f8680> as *const (), 1), // +0x425ca0 sub_425ca0
    Lifted::new(0x7000b11cc0, tpl_7::<0x70017f8690> as *const (), 1), // +0x425cc0 sub_425cc0
    Lifted::new(0x7000b11ce0, tpl_7::<0x70017f8820> as *const (), 1), // +0x425ce0 sub_425ce0
    Lifted::new(0x7000b11d00, tpl_7::<0x70017f8800> as *const (), 1), // +0x425d00 sub_425d00
    Lifted::new(0x7000b11d20, tpl_7::<0x70017f87e0> as *const (), 1), // +0x425d20 sub_425d20
    Lifted::new(0x7000b11d40, tpl_7::<0x70017f8740> as *const (), 1), // +0x425d40 sub_425d40
    Lifted::new(0x7000b11d60, tpl_7::<0x70017f8760> as *const (), 1), // +0x425d60 sub_425d60
    Lifted::new(0x7000b11d80, tpl_7::<0x70017f8780> as *const (), 1), // +0x425d80 sub_425d80
    Lifted::new(0x7000b11da0, tpl_7::<0x70017f87a0> as *const (), 1), // +0x425da0 sub_425da0
    Lifted::new(0x7000b11dc0, tpl_7::<0x70017f87c0> as *const (), 1), // +0x425dc0 sub_425dc0
    Lifted::new(0x7000b11de0, tpl_7::<0x70017f8720> as *const (), 1), // +0x425de0 sub_425de0
    Lifted::new(0x7000b11e00, tpl_7::<0x70017f8840> as *const (), 1), // +0x425e00 sub_425e00
    Lifted::new(0x7000b11e20, tpl_7::<0x70017f8860> as *const (), 1), // +0x425e20 sub_425e20
    Lifted::new(0x7000b11e40, tpl_37::<0x9db8> as *const (), 1), // +0x425e40 sub_425e40
    Lifted::new(0x7000b11e60, tpl_37::<0xd910> as *const (), 1), // +0x425e60 sub_425e60
    Lifted::new(0x7000b11e80, fn_7000b11e80 as *const (), 1), // +0x425e80 sub_425e80
    Lifted::new(0x7000b11ea0, tpl_7::<0x7000b11e80> as *const (), 1), // +0x425ea0 sub_425ea0
    Lifted::new(0x7000b11ec0, fn_7000b11ec0 as *const (), 1), // +0x425ec0 sub_425ec0
    Lifted::new(0x7000b11ef0, tpl_7::<0x7000b11ec0> as *const (), 1), // +0x425ef0 sub_425ef0
    Lifted::new(0x7000b11f10, fn_7000b11f10 as *const (), 1), // +0x425f10 sub_425f10
    Lifted::new(0x7000b11f50, tpl_7::<0x7000b11f10> as *const (), 1), // +0x425f50 sub_425f50
    Lifted::new(0x7000b11f70, fn_7000b11f70 as *const (), 1), // +0x425f70 sub_425f70
    Lifted::new(0x7000b11f90, tpl_7::<0x7000b11f70> as *const (), 1), // +0x425f90 sub_425f90
    Lifted::new(0x7000b11fb0, tpl_30 as *const (), 1), // +0x425fb0 sub_425fb0
    Lifted::new(0x7000b11fc0, tpl_7::<0x7000b11fb0> as *const (), 1), // +0x425fc0 sub_425fc0
    Lifted::new(0x7000b11fe0, fn_7000b11fe0 as *const (), 1), // +0x425fe0 sub_425fe0
    Lifted::new(0x7000b12010, tpl_7::<0x7000b11fe0> as *const (), 1), // +0x426010 sub_426010
    Lifted::new(0x7000b12030, fn_7000b12030 as *const (), 1), // +0x426030 sub_426030
    Lifted::new(0x7000b12050, tpl_7::<0x7000b12030> as *const (), 1), // +0x426050 sub_426050
    Lifted::new(0x7000b12070, fn_7000b12070 as *const (), 1), // +0x426070 sub_426070
    Lifted::new(0x7000b12090, tpl_7::<0x7000b12070> as *const (), 1), // +0x426090 sub_426090
    Lifted::new(0x7000b120b0, fn_7000b120b0 as *const (), 1), // +0x4260b0 sub_4260b0
    Lifted::new(0x7000b120e0, tpl_7::<0x7000b120b0> as *const (), 1), // +0x4260e0 sub_4260e0
    Lifted::new(0x7000b12100, fn_7000b12100 as *const (), 1), // +0x426100 sub_426100
    Lifted::new(0x7000b12110, tpl_7::<0x7000b0e040> as *const (), 1), // +0x426110 sub_426110
    Lifted::new(0x7000b12130, tpl_7::<0x7000b0e0d0> as *const (), 1), // +0x426130 sub_426130
    Lifted::new(0x7000b12150, fn_7000b12150 as *const (), 0), // +0x426150 sub_426150
    Lifted::new(0x7000b12180, tpl_7::<0x700182e6a0> as *const (), 1), // +0x426180 sub_426180
    Lifted::new(0x7000b121a0, tpl_7::<0x70006f74d0> as *const (), 1), // +0x4261a0 sub_4261a0
    Lifted::new(0x7000b121c0, tpl_7::<0x7000b0f230> as *const (), 1), // +0x4261c0 sub_4261c0
    Lifted::new(0x7000b121e0, tpl_39::<0x228> as *const (), 1), // +0x4261e0 sub_4261e0
    Lifted::new(0x7000b121f0, tpl_7::<0x7000791d00> as *const (), 1), // +0x4261f0 sub_4261f0
    Lifted::new(0x7000b12210, fn_7000b12210 as *const (), 1), // +0x426210 sub_426210
    Lifted::new(0x7000b12220, tpl_7::<0x7000b0f090> as *const (), 1), // +0x426220 sub_426220
    Lifted::new(0x7000b12240, tpl_7::<0x70007607b0> as *const (), 1), // +0x426240 sub_426240
    Lifted::new(0x7000b12260, tpl_7::<0x70007a3900> as *const (), 1), // +0x426260 sub_426260
    Lifted::new(0x7000b12280, tpl_7::<0x7000714780> as *const (), 1), // +0x426280 sub_426280
    Lifted::new(0x7000b122a0, tpl_7::<0x70006fae00> as *const (), 1), // +0x4262a0 sub_4262a0
    Lifted::new(0x7000b122c0, tpl_7::<0x7001c6fa5c> as *const (), 1), // +0x4262c0 sub_4262c0
    Lifted::new(0x7000b122e0, fn_7000b122e0 as *const (), 1), // +0x4262e0 sub_4262e0
    Lifted::new(0x7000b122f0, tpl_817::<0xc778> as *const (), 1), // +0x4262f0 sub_4262f0
    Lifted::new(0x7000b12310, tpl_37::<0xc941> as *const (), 1), // +0x426310 sub_426310
    Lifted::new(0x7000b12330, fn_7000b12330 as *const (), 1), // +0x426330 sub_426330
    Lifted::new(0x7000b12350, tpl_817::<0xc7f0> as *const (), 1), // +0x426350 sub_426350
    Lifted::new(0x7000b12370, fn_7000b12370 as *const (), 1), // +0x426370 sub_426370
    Lifted::new(0x7000b12390, tpl_7::<0x7000745eb0> as *const (), 1), // +0x426390 sub_426390
    Lifted::new(0x7000b123b0, tpl_7::<0x700077c5e0> as *const (), 1), // +0x4263b0 sub_4263b0
    Lifted::new(0x7000b123d0, tpl_7::<0x7000b0f3a0> as *const (), 1), // +0x4263d0 sub_4263d0
    Lifted::new(0x7000b123f0, tpl_7::<0x7000b0f480> as *const (), 1), // +0x4263f0 sub_4263f0
    Lifted::new(0x7000b12410, tpl_818::<0x7001d54ca0> as *const (), 2), // +0x426410 sub_426410
    Lifted::new(0x7000b12440, tpl_7::<0x7000b12410> as *const (), 1), // +0x426440 sub_426440
    Lifted::new(0x7000b12460, tpl_818::<0x7001d54c90> as *const (), 2), // +0x426460 sub_426460
    Lifted::new(0x7000b12490, tpl_7::<0x7000b12460> as *const (), 1), // +0x426490 sub_426490
    Lifted::new(0x7000b124b0, fn_7000b124b0 as *const (), 2), // +0x4264b0 sub_4264b0
    Lifted::new(0x7000b124e0, tpl_7::<0x7000b124b0> as *const (), 1), // +0x4264e0 sub_4264e0
    Lifted::new(0x7000b12500, tpl_37::<0x30> as *const (), 1), // +0x426500 sub_426500
    Lifted::new(0x7000b12510, tpl_37::<0x20> as *const (), 1), // +0x426510 sub_426510
    Lifted::new(0x7000b12520, tpl_37::<0x28> as *const (), 1), // +0x426520 sub_426520
    Lifted::new(0x7000b12530, tpl_37::<0xd76c> as *const (), 1), // +0x426530 sub_426530
    Lifted::new(0x7000b12550, tpl_7::<0x700072b670> as *const (), 1), // +0x426550 sub_426550
    Lifted::new(0x7000b12570, tpl_7::<0x7000711f60> as *const (), 1), // +0x426570 sub_426570
    Lifted::new(0x7000b12590, tpl_37::<0xa6f8> as *const (), 1), // +0x426590 sub_426590
    Lifted::new(0x7000b125b0, tpl_816::<0x7000beba10> as *const (), 0), // +0x4265b0 sub_4265b0
    Lifted::new(0x7000b125e0, tpl_816::<0x7000beba20> as *const (), 0), // +0x4265e0 sub_4265e0
    Lifted::new(0x7000b12610, tpl_816::<0x7000beba30> as *const (), 0), // +0x426610 sub_426610
    Lifted::new(0x7000b12640, fn_7000b12640 as *const (), 0), // +0x426640 sub_426640
    Lifted::new(0x7000b12660, tpl_37::<0xa358> as *const (), 1), // +0x426660 sub_426660
    Lifted::new(0x7000b12680, tpl_819::<0x7001d54ca0> as *const (), 1), // +0x426680 sub_426680
    Lifted::new(0x7000b126a0, tpl_819::<0x7001d54c98> as *const (), 1), // +0x4266a0 sub_4266a0
    Lifted::new(0x7000b126c0, tpl_819::<0x7001d54c90> as *const (), 1), // +0x4266c0 sub_4266c0
    Lifted::new(0x7000b126e0, tpl_819::<0x7001d55050> as *const (), 1), // +0x4266e0 sub_4266e0
    Lifted::new(0x7000b12700, tpl_819::<0x7001d54c78> as *const (), 1), // +0x426700 sub_426700
    Lifted::new(0x7000b12720, tpl_819::<0x7001d54c80> as *const (), 1), // +0x426720 sub_426720
    Lifted::new(0x7000b12740, tpl_819::<0x7001d54c60> as *const (), 1), // +0x426740 sub_426740
    Lifted::new(0x7000b12760, tpl_819::<0x7001d54c38> as *const (), 1), // +0x426760 sub_426760
    Lifted::new(0x7000b12780, tpl_819::<0x7001d54c70> as *const (), 1), // +0x426780 sub_426780
    Lifted::new(0x7000b127a0, tpl_819::<0x7001d54c10> as *const (), 1), // +0x4267a0 sub_4267a0
    Lifted::new(0x7000b127c0, tpl_819::<0x7001d54c68> as *const (), 1), // +0x4267c0 sub_4267c0
    Lifted::new(0x7000b127e0, tpl_819::<0x7001d54c30> as *const (), 1), // +0x4267e0 sub_4267e0
    Lifted::new(0x7000b12800, tpl_819::<0x7001d54c40> as *const (), 1), // +0x426800 sub_426800
    Lifted::new(0x7000b12820, tpl_819::<0x7001d54c50> as *const (), 1), // +0x426820 sub_426820
    Lifted::new(0x7000b12840, tpl_819::<0x7001d54c48> as *const (), 1), // +0x426840 sub_426840
    Lifted::new(0x7000b12860, tpl_819::<0x7001d54c58> as *const (), 1), // +0x426860 sub_426860
    Lifted::new(0x7000b12880, tpl_819::<0x7001d54c28> as *const (), 1), // +0x426880 sub_426880
    Lifted::new(0x7000b128a0, tpl_819::<0x7001d54c20> as *const (), 1), // +0x4268a0 sub_4268a0
    Lifted::new(0x7000b128c0, tpl_819::<0x7001d54c08> as *const (), 1), // +0x4268c0 sub_4268c0
    Lifted::new(0x7000b128e0, tpl_819::<0x7001d54c18> as *const (), 1), // +0x4268e0 sub_4268e0
    Lifted::new(0x7000b12900, tpl_819::<0x7001d54c00> as *const (), 1), // +0x426900 sub_426900
    Lifted::new(0x7000b12920, tpl_7::<0x7000742160> as *const (), 1), // +0x426920 sub_426920
    Lifted::new(0x7000b12940, tpl_820::<0xc810, 0x1810> as *const (), 1), // +0x426940 sub_426940
    Lifted::new(0x7000b12960, fn_7000b12960 as *const (), 1), // +0x426960 sub_426960
    Lifted::new(0x7000b12980, tpl_7::<0x7000b10180> as *const (), 1), // +0x426980 sub_426980
    Lifted::new(0x7000b129a0, tpl_7::<0x7000793950> as *const (), 1), // +0x4269a0 sub_4269a0
    Lifted::new(0x7000b129c0, tpl_7::<0x7000769570> as *const (), 1), // +0x4269c0 sub_4269c0
    Lifted::new(0x7000b129e0, tpl_37::<0xc7e0> as *const (), 1), // +0x4269e0 sub_4269e0
    Lifted::new(0x7000b12a00, tpl_36::<0x1d8> as *const (), 1), // +0x426a00 sub_426a00
    Lifted::new(0x7000b12a10, tpl_14 as *const (), 1), // +0x426a10 sub_426a10
    Lifted::new(0x7000b12a20, tpl_14 as *const (), 1), // +0x426a20 sub_426a20
    Lifted::new(0x7000b12a30, tpl_37::<0xc971> as *const (), 1), // +0x426a30 sub_426a30
    Lifted::new(0x7000b12a50, tpl_7::<0x7000767b00> as *const (), 1), // +0x426a50 sub_426a50
    Lifted::new(0x7000b12a70, tpl_7::<0x7000754d10> as *const (), 1), // +0x426a70 sub_426a70
    Lifted::new(0x7000b12a90, tpl_7::<0x7000b101a0> as *const (), 1), // +0x426a90 sub_426a90
    Lifted::new(0x7000b12ab0, tpl_821::<0x7000712500, 0x7000b12abe, 0x7000b12abf, 0x7000b12ac0, 0x7000b12ac1, 0x7000b12ac2, 0x7000b12ac3, 0x7000b12ac4, 0x7000b12ac5, 0x7000b12ac6, 0x7000b12ac7, 0x7000b12ac8, 0x7000b12ac9, 0x7000b12aca, 0x7000b12acb, 0x7000b12acc, 0x7000b12acd, 0x7000b12ace, 0x7000b12acf, 0x700177c110> as *const (), 0), // +0x426ab0 sub_426ab0
    Lifted::new(0x7000b12ae0, tpl_7::<0x70007997f0> as *const (), 1), // +0x426ae0 sub_426ae0
    Lifted::new(0x7000b12b00, tpl_13::<0x700177c0b0> as *const (), 0), // +0x426b00 sub_426b00
    Lifted::new(0x7000b12b20, tpl_821::<0x700077b7b0, 0x7000b12b2e, 0x7000b12b2f, 0x7000b12b30, 0x7000b12b31, 0x7000b12b32, 0x7000b12b33, 0x7000b12b34, 0x7000b12b35, 0x7000b12b36, 0x7000b12b37, 0x7000b12b38, 0x7000b12b39, 0x7000b12b3a, 0x7000b12b3b, 0x7000b12b3c, 0x7000b12b3d, 0x7000b12b3e, 0x7000b12b3f, 0x700177c0d0> as *const (), 0), // +0x426b20 sub_426b20
    Lifted::new(0x7000b12b50, tpl_7::<0x7000748830> as *const (), 1), // +0x426b50 sub_426b50
    Lifted::new(0x7000b12b70, tpl_13::<0x700177c0e0> as *const (), 0), // +0x426b70 sub_426b70
    Lifted::new(0x7000b12b90, tpl_7::<0x700070ff90> as *const (), 1), // +0x426b90 sub_426b90
    Lifted::new(0x7000b12bb0, tpl_821::<0x700075f700, 0x7000b12bbe, 0x7000b12bbf, 0x7000b12bc0, 0x7000b12bc1, 0x7000b12bc2, 0x7000b12bc3, 0x7000b12bc4, 0x7000b12bc5, 0x7000b12bc6, 0x7000b12bc7, 0x7000b12bc8, 0x7000b12bc9, 0x7000b12bca, 0x7000b12bcb, 0x7000b12bcc, 0x7000b12bcd, 0x7000b12bce, 0x7000b12bcf, 0x7000b10930> as *const (), 0), // +0x426bb0 sub_426bb0
    Lifted::new(0x7000b12be0, tpl_7::<0x70006f2480> as *const (), 1), // +0x426be0 sub_426be0
    Lifted::new(0x7000b12c00, tpl_7::<0x7000781880> as *const (), 1), // +0x426c00 sub_426c00
    Lifted::new(0x7000b12c20, tpl_7::<0x700078a730> as *const (), 1), // +0x426c20 sub_426c20
    Lifted::new(0x7000b12c40, tpl_7::<0x70006ed930> as *const (), 1), // +0x426c40 sub_426c40
    Lifted::new(0x7000b12c60, tpl_822::<0x9878, 0xf0> as *const (), 1), // +0x426c60 sub_426c60
    Lifted::new(0x7000b12c90, tpl_822::<0x9878, 0xe8> as *const (), 1), // +0x426c90 sub_426c90
    Lifted::new(0x7000b12cc0, fn_7000b12cc0 as *const (), 1), // +0x426cc0 sub_426cc0
    Lifted::new(0x7000b12ce0, fn_7000b12ce0 as *const (), 1), // +0x426ce0 sub_426ce0
    Lifted::new(0x7000b12d00, tpl_7::<0x7000772df0> as *const (), 1), // +0x426d00 sub_426d00
    Lifted::new(0x7000b12d20, tpl_7::<0x700073cb40> as *const (), 1), // +0x426d20 sub_426d20
    Lifted::new(0x7000b12d40, tpl_37::<0xc7a8> as *const (), 1), // +0x426d40 sub_426d40
    Lifted::new(0x7000b12d60, tpl_37::<0xd918> as *const (), 1), // +0x426d60 sub_426d60
    Lifted::new(0x7000b12d80, fn_7000b12d80 as *const (), 0), // +0x426d80 sub_426d80
    Lifted::new(0x7000b12e70, tpl_7::<0x7000738090> as *const (), 1), // +0x426e70 sub_426e70
    Lifted::new(0x7000b12e90, tpl_7::<0x700070d440> as *const (), 1), // +0x426e90 sub_426e90
    Lifted::new(0x7000b12eb0, tpl_7::<0x7000768570> as *const (), 1), // +0x426eb0 sub_426eb0
    Lifted::new(0x7000b12ed0, tpl_7::<0x70006f4ec0> as *const (), 1), // +0x426ed0 sub_426ed0
    Lifted::new(0x7000b12ef0, tpl_7::<0x7000759af0> as *const (), 1), // +0x426ef0 sub_426ef0
    Lifted::new(0x7000b12f10, tpl_7::<0x700078a1e0> as *const (), 1), // +0x426f10 sub_426f10
    Lifted::new(0x7000b12f30, tpl_7::<0x7001865320> as *const (), 1), // +0x426f30 sub_426f30
    Lifted::new(0x7000b12f50, tpl_7::<0x700078b4a0> as *const (), 1), // +0x426f50 sub_426f50
    Lifted::new(0x7000b12f70, tpl_823::<0x1> as *const (), 1), // +0x426f70 sub_426f70
    Lifted::new(0x7000b13000, tpl_7::<0x7000797d10> as *const (), 1), // +0x427000 sub_427000
    Lifted::new(0x7000b13020, fn_7000b13020 as *const (), 1), // +0x427020 sub_427020
    Lifted::new(0x7000b13170, tpl_7::<0x7000728ce0> as *const (), 1), // +0x427170 sub_427170
    Lifted::new(0x7000b13190, tpl_816::<0x70010c0620> as *const (), 0), // +0x427190 sub_427190
    Lifted::new(0x7000b131c0, tpl_816::<0x70010c0620> as *const (), 0), // +0x4271c0 sub_4271c0
    Lifted::new(0x7000b131f0, tpl_37::<0x3040> as *const (), 1), // +0x4271f0 sub_4271f0
    Lifted::new(0x7000b13210, tpl_7::<0x7000731310> as *const (), 1), // +0x427210 sub_427210
    Lifted::new(0x7000b13230, tpl_7::<0x7000707a00> as *const (), 1), // +0x427230 sub_427230
    Lifted::new(0x7000b13250, tpl_7::<0x7000783a90> as *const (), 1), // +0x427250 sub_427250
    Lifted::new(0x7000b13270, tpl_7::<0x700074b490> as *const (), 1), // +0x427270 sub_427270
    Lifted::new(0x7000b13290, tpl_7::<0x7000b101d0> as *const (), 1), // +0x427290 sub_427290
    Lifted::new(0x7000b132b0, tpl_37::<0x9748> as *const (), 1), // +0x4272b0 sub_4272b0
    Lifted::new(0x7000b132d0, tpl_37::<0xd768> as *const (), 1), // +0x4272d0 sub_4272d0
    Lifted::new(0x7000b132f0, tpl_7::<0x7000781a80> as *const (), 1), // +0x4272f0 sub_4272f0
    Lifted::new(0x7000b13310, tpl_7::<0x7000b11980> as *const (), 1), // +0x427310 sub_427310
    Lifted::new(0x7000b13330, tpl_7::<0x7000b11990> as *const (), 1), // +0x427330 sub_427330
    Lifted::new(0x7000b13350, tpl_7::<0x7001c6fa69> as *const (), 1), // +0x427350 sub_427350
    Lifted::new(0x7000b13370, tpl_37::<0x3090> as *const (), 1), // +0x427370 sub_427370
    Lifted::new(0x7000b13390, tpl_7::<0x70007a6b90> as *const (), 1), // +0x427390 sub_427390
    Lifted::new(0x7000b133b0, tpl_7::<0x7000707290> as *const (), 1), // +0x4273b0 sub_4273b0
    Lifted::new(0x7000b133d0, tpl_7::<0x7000794a50> as *const (), 1), // +0x4273d0 sub_4273d0
    Lifted::new(0x7000b133f0, tpl_7::<0x7000749910> as *const (), 1), // +0x4273f0 sub_4273f0
    Lifted::new(0x7000b13410, tpl_7::<0x7000732090> as *const (), 1), // +0x427410 sub_427410
    Lifted::new(0x7000b13430, tpl_7::<0x7000703820> as *const (), 1), // +0x427430 sub_427430
    Lifted::new(0x7000b13450, tpl_7::<0x700078cd00> as *const (), 1), // +0x427450 sub_427450
    Lifted::new(0x7000b13470, tpl_7::<0x700076b8f0> as *const (), 1), // +0x427470 sub_427470
    Lifted::new(0x7000b13490, tpl_7::<0x7000717800> as *const (), 1), // +0x427490 sub_427490
    Lifted::new(0x7000b134b0, tpl_7::<0x70006f52d0> as *const (), 1), // +0x4274b0 sub_4274b0
    Lifted::new(0x7000b134d0, tpl_7::<0x700070a080> as *const (), 1), // +0x4274d0 sub_4274d0
    Lifted::new(0x7000b134f0, tpl_7::<0x7000769a20> as *const (), 1), // +0x4274f0 sub_4274f0
    Lifted::new(0x7000b13510, tpl_7::<0x7000782450> as *const (), 1), // +0x427510 sub_427510
    Lifted::new(0x7000b13530, tpl_7::<0x70006fb770> as *const (), 1), // +0x427530 sub_427530
    Lifted::new(0x7000b13550, tpl_7::<0x7000748ca0> as *const (), 1), // +0x427550 sub_427550
    Lifted::new(0x7000b13570, tpl_7::<0x70006f0b40> as *const (), 1), // +0x427570 sub_427570
    Lifted::new(0x7000b13590, tpl_7::<0x700074a220> as *const (), 1), // +0x427590 sub_427590
    Lifted::new(0x7000b135b0, tpl_7::<0x70007a7560> as *const (), 1), // +0x4275b0 sub_4275b0
    Lifted::new(0x7000b135d0, tpl_7::<0x700071c000> as *const (), 1), // +0x4275d0 sub_4275d0
    Lifted::new(0x7000b135f0, tpl_7::<0x70007221c0> as *const (), 1), // +0x4275f0 sub_4275f0
    Lifted::new(0x7000b13610, tpl_7::<0x70006ed920> as *const (), 1), // +0x427610 sub_427610
    Lifted::new(0x7000b13630, tpl_7::<0x700072f0d0> as *const (), 1), // +0x427630 sub_427630
    Lifted::new(0x7000b13650, tpl_7::<0x7000772b10> as *const (), 1), // +0x427650 sub_427650
    Lifted::new(0x7000b13670, tpl_7::<0x700070ad50> as *const (), 1), // +0x427670 sub_427670
    Lifted::new(0x7000b13690, tpl_7::<0x70006efd80> as *const (), 1), // +0x427690 sub_427690
    Lifted::new(0x7000b136b0, tpl_7::<0x7000716a10> as *const (), 1), // +0x4276b0 sub_4276b0
    Lifted::new(0x7000b136d0, tpl_7::<0x7000774a10> as *const (), 1), // +0x4276d0 sub_4276d0
    Lifted::new(0x7000b136f0, tpl_7::<0x7000727c50> as *const (), 1), // +0x4276f0 sub_4276f0
    Lifted::new(0x7000b13710, tpl_7::<0x7000755760> as *const (), 1), // +0x427710 sub_427710
    Lifted::new(0x7000b13730, tpl_7::<0x7000776db0> as *const (), 1), // +0x427730 sub_427730
    Lifted::new(0x7000b13750, tpl_7::<0x700079c290> as *const (), 1), // +0x427750 sub_427750
    Lifted::new(0x7000b13770, tpl_7::<0x70007839c0> as *const (), 1), // +0x427770 sub_427770
    Lifted::new(0x7000b13790, tpl_7::<0x700071dcb0> as *const (), 1), // +0x427790 sub_427790
    Lifted::new(0x7000b137b0, tpl_7::<0x70007395b0> as *const (), 1), // +0x4277b0 sub_4277b0
    Lifted::new(0x7000b137d0, tpl_7::<0x700074d8e0> as *const (), 1), // +0x4277d0 sub_4277d0
    Lifted::new(0x7000b137f0, tpl_7::<0x700076dee0> as *const (), 1), // +0x4277f0 sub_4277f0
    Lifted::new(0x7000b13810, tpl_7::<0x70007650b0> as *const (), 1), // +0x427810 sub_427810
    Lifted::new(0x7000b13830, tpl_7::<0x7000705ba0> as *const (), 1), // +0x427830 sub_427830
    Lifted::new(0x7000b13850, tpl_7::<0x700078af00> as *const (), 1), // +0x427850 sub_427850
    Lifted::new(0x7000b13870, tpl_7::<0x7000797660> as *const (), 1), // +0x427870 sub_427870
    Lifted::new(0x7000b13890, tpl_7::<0x700079c4d0> as *const (), 1), // +0x427890 sub_427890
    Lifted::new(0x7000b138b0, tpl_7::<0x700075ba30> as *const (), 1), // +0x4278b0 sub_4278b0
    Lifted::new(0x7000b138d0, tpl_7::<0x700076e190> as *const (), 1), // +0x4278d0 sub_4278d0
    Lifted::new(0x7000b138f0, tpl_7::<0x7000735b70> as *const (), 1), // +0x4278f0 sub_4278f0
    Lifted::new(0x7000b13910, tpl_7::<0x700070cf70> as *const (), 1), // +0x427910 sub_427910
    Lifted::new(0x7000b13930, tpl_7::<0x700071f570> as *const (), 1), // +0x427930 sub_427930
    Lifted::new(0x7000b13950, tpl_7::<0x70007874c0> as *const (), 1), // +0x427950 sub_427950
    Lifted::new(0x7000b13970, tpl_315::<0x7001c71ce0> as *const (), 1), // +0x427970 sub_427970
    Lifted::new(0x7000b13980, fn_7000b13980 as *const (), 0), // +0x427980 sub_427980
    Lifted::new(0x7000b139f0, fn_7000b139f0 as *const (), 0), // +0x4279f0 sub_4279f0
    Lifted::new(0x7000b13a10, fn_7000b13a10 as *const (), 0), // +0x427a10 sub_427a10
    Lifted::new(0x7000b13a80, fn_7000b13a80 as *const (), 1), // +0x427a80 sub_427a80
    Lifted::new(0x7000b13ab0, fn_7000b13ab0 as *const (), 1), // +0x427ab0 sub_427ab0
    Lifted::new(0x7000b13b00, fn_7000b13b00 as *const (), 0), // +0x427b00 sub_427b00
    Lifted::new(0x7000b13b60, fn_7000b13b60 as *const (), 0), // +0x427b60 sub_427b60
    Lifted::new(0x7000b13bc0, fn_7000b13bc0 as *const (), 0), // +0x427bc0 sub_427bc0
    Lifted::new(0x7000b13c10, fn_7000b13c10 as *const (), 0), // +0x427c10 sub_427c10
    Lifted::new(0x7000b13c40, fn_7000b13c40 as *const (), 0), // +0x427c40 sub_427c40
    Lifted::new(0x7000b13c80, tpl_824::<0x8, 0xc> as *const (), 1), // +0x427c80 sub_427c80
    Lifted::new(0x7000b13ca0, tpl_824::<0x8, 0x4> as *const (), 1), // +0x427ca0 sub_427ca0
    Lifted::new(0x7000b13cc0, fn_7000b13cc0 as *const (), 1), // +0x427cc0 sub_427cc0
    Lifted::new(0x7000b13ce0, fn_7000b13ce0 as *const (), 1), // +0x427ce0 sub_427ce0
    Lifted::new(0x7000b13d00, fn_7000b13d00 as *const (), 1), // +0x427d00 sub_427d00
    Lifted::new(0x7000b13d20, fn_7000b13d20 as *const (), 1), // +0x427d20 sub_427d20
    Lifted::new(0x7000b13d40, fn_7000b13d40 as *const (), 1), // +0x427d40 sub_427d40
    Lifted::new(0x7000b13d60, tpl_825 as *const (), 1), // +0x427d60 sub_427d60
    Lifted::new(0x7000b13d70, fn_7000b13d70 as *const (), 1), // +0x427d70 sub_427d70
    Lifted::new(0x7000b13e20, fn_7000b13e20 as *const (), 3), // +0x427e20 sub_427e20
    Lifted::new(0x7000b13f40, tpl_41 as *const (), 1), // +0x427f40 sub_427f40
    Lifted::new(0x7000b13f50, tpl_41 as *const (), 1), // +0x427f50 sub_427f50
    Lifted::new(0x7000b13f60, tpl_826::<0x8, 0xc> as *const (), 1), // +0x427f60 sub_427f60
    Lifted::new(0x7000b13f80, tpl_826::<0x8, 0x4> as *const (), 1), // +0x427f80 sub_427f80
    Lifted::new(0x7000b13fa0, fn_7000b13fa0 as *const (), 1), // +0x427fa0 sub_427fa0
    Lifted::new(0x7000b13fd0, fn_7000b13fd0 as *const (), 1), // +0x427fd0 sub_427fd0
    Lifted::new(0x7000b13ff0, fn_7000b13ff0 as *const (), 3), // +0x427ff0 sub_427ff0
    Lifted::new(0x7000b14050, fn_7000b14050 as *const (), 3), // +0x428050 sub_428050
    Lifted::new(0x7000b140c0, fn_7000b140c0 as *const (), 3), // +0x4280c0 sub_4280c0
    Lifted::new(0x7000b14150, fn_7000b14150 as *const (), 3), // +0x428150 sub_428150
    Lifted::new(0x7000b14250, tpl_386::<0x70018da5bc, 0x700177aa00, 0x7000724770, 0x18, 0x70017d8c38, 0x7000723490, 0x7000b142a1> as *const (), 0), // +0x428250 sub_428250
    Lifted::new(0x7000b142c0, fn_7000b142c0 as *const (), 0), // +0x4282c0 sub_4282c0
    Lifted::new(0x7000b143c0, fn_7000b143c0 as *const (), 0), // +0x4283c0 sub_4283c0
    Lifted::new(0x7000b14480, fn_7000b14480 as *const (), 0), // +0x428480 sub_428480
    Lifted::new(0x7000b15d00, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bca8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x429d00 sub_429d00
    Lifted::new(0x7000b15e30, tpl_828::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b6a8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x429e30 sub_429e30
    Lifted::new(0x7000b15f60, fn_7000b15f60 as *const (), 0), // +0x429f60 sub_429f60
    Lifted::new(0x7000b16090, tpl_829::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b6f0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42a090 sub_42a090
    Lifted::new(0x7000b161d0, tpl_830::<0x7001c6abd8, 0x5, 0x2, 0x7001c5b720, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42a1d0 sub_42a1d0
    Lifted::new(0x7000b16300, tpl_830::<0x7001c6abd8, 0x5, 0x2, 0x7001c5b708, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42a300 sub_42a300
    Lifted::new(0x7000b16430, tpl_829::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b6d8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42a430 sub_42a430
    Lifted::new(0x7000b16570, tpl_828::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b738, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42a570 sub_42a570
    Lifted::new(0x7000b166a0, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b258, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b167e1> as *const (), 0), // +0x42a6a0 sub_42a6a0
    Lifted::new(0x7000b16840, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b240, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b16981> as *const (), 0), // +0x42a840 sub_42a840
    Lifted::new(0x7000b169e0, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b228, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b16b21> as *const (), 0), // +0x42a9e0 sub_42a9e0
    Lifted::new(0x7000b16b80, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b2a0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b16cc1> as *const (), 0), // +0x42ab80 sub_42ab80
    Lifted::new(0x7000b16d20, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b288, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b16e61> as *const (), 0), // +0x42ad20 sub_42ad20
    Lifted::new(0x7000b16ec0, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b270, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17001> as *const (), 0), // +0x42aec0 sub_42aec0
    Lifted::new(0x7000b17060, fn_7000b17060 as *const (), 0), // +0x42b060 sub_42b060
    Lifted::new(0x7000b17190, tpl_829::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b810, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42b190 sub_42b190
    Lifted::new(0x7000b172d0, tpl_829::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b7f8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42b2d0 sub_42b2d0
    Lifted::new(0x7000b17410, tpl_828::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b828, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42b410 sub_42b410
    Lifted::new(0x7000b17540, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c440, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42b540 sub_42b540
    Lifted::new(0x7000b17660, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b378, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17797> as *const (), 0), // +0x42b660 sub_42b660
    Lifted::new(0x7000b177f0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b390, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17927> as *const (), 0), // +0x42b7f0 sub_42b7f0
    Lifted::new(0x7000b17980, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b330, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17ab7> as *const (), 0), // +0x42b980 sub_42b980
    Lifted::new(0x7000b17b10, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b348, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17c4c> as *const (), 0), // +0x42bb10 sub_42bb10
    Lifted::new(0x7000b17ca0, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b360, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17ddc> as *const (), 0), // +0x42bca0 sub_42bca0
    Lifted::new(0x7000b17e30, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b600, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b17f67> as *const (), 0), // +0x42be30 sub_42be30
    Lifted::new(0x7000b17fc0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b618, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b180f7> as *const (), 0), // +0x42bfc0 sub_42bfc0
    Lifted::new(0x7000b18150, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b5a0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b18287> as *const (), 0), // +0x42c150 sub_42c150
    Lifted::new(0x7000b182e0, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b588, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1841c> as *const (), 0), // +0x42c2e0 sub_42c2e0
    Lifted::new(0x7000b18470, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b5e8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b185a2> as *const (), 0), // +0x42c470 sub_42c470
    Lifted::new(0x7000b185f0, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b5d0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1872c> as *const (), 0), // +0x42c5f0 sub_42c5f0
    Lifted::new(0x7000b18780, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b5b8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b188bc> as *const (), 0), // +0x42c780 sub_42c780
    Lifted::new(0x7000b18910, tpl_836::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b8b8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42c910 sub_42c910
    Lifted::new(0x7000b18a40, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b8a0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42ca40 sub_42ca40
    Lifted::new(0x7000b18b70, tpl_836::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b8e8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42cb70 sub_42cb70
    Lifted::new(0x7000b18ca0, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b8d0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42cca0 sub_42cca0
    Lifted::new(0x7000b18dd0, tpl_838::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b900, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42cdd0 sub_42cdd0
    Lifted::new(0x7000b18ef0, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bde0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42cef0 sub_42cef0
    Lifted::new(0x7000b19020, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bd50, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d020 sub_42d020
    Lifted::new(0x7000b19140, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bdf8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d140 sub_42d140
    Lifted::new(0x7000b19270, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5be10, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d270 sub_42d270
    Lifted::new(0x7000b193a0, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bbe8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d3a0 sub_42d3a0
    Lifted::new(0x7000b194c0, fn_7000b194c0 as *const (), 0), // +0x42d4c0 sub_42d4c0
    Lifted::new(0x7000b195f0, tpl_840::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cad0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d5f0 sub_42d5f0
    Lifted::new(0x7000b19720, tpl_840::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb00, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d720 sub_42d720
    Lifted::new(0x7000b19850, tpl_840::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cae8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d850 sub_42d850
    Lifted::new(0x7000b19980, tpl_840::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c3c8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42d980 sub_42d980
    Lifted::new(0x7000b19ab0, fn_7000b19ab0 as *const (), 0), // +0x42dab0 sub_42dab0
    Lifted::new(0x7000b19be0, tpl_836::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b888, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42dbe0 sub_42dbe0
    Lifted::new(0x7000b19d10, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b2b8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b19e42> as *const (), 0), // +0x42dd10 sub_42dd10
    Lifted::new(0x7000b19e90, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b2d0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b19fc2> as *const (), 0), // +0x42de90 sub_42de90
    Lifted::new(0x7000b1a010, fn_7000b1a010 as *const (), 0), // +0x42e010 sub_42e010
    Lifted::new(0x7000b1a140, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b1f8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1a277> as *const (), 0), // +0x42e140 sub_42e140
    Lifted::new(0x7000b1a2d0, fn_7000b1a2d0 as *const (), 0), // +0x42e2d0 sub_42e2d0
    Lifted::new(0x7000b1a400, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b198, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1a532> as *const (), 0), // +0x42e400 sub_42e400
    Lifted::new(0x7000b1a580, tpl_841::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bc60, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42e580 sub_42e580
    Lifted::new(0x7000b1a6b0, tpl_842::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ba08, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42e6b0 sub_42e6b0
    Lifted::new(0x7000b1a7e0, fn_7000b1a7e0 as *const (), 0), // +0x42e7e0 sub_42e7e0
    Lifted::new(0x7000b1a8f0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b3c0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1aa27> as *const (), 0), // +0x42e8f0 sub_42e8f0
    Lifted::new(0x7000b1aa80, fn_7000b1aa80 as *const (), 0), // +0x42ea80 sub_42ea80
    Lifted::new(0x7000b1abb0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b3a8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1ace7> as *const (), 0), // +0x42ebb0 sub_42ebb0
    Lifted::new(0x7000b1ad40, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5beb8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42ed40 sub_42ed40
    Lifted::new(0x7000b1ae60, fn_7000b1ae60 as *const (), 0), // +0x42ee60 sub_42ee60
    Lifted::new(0x7000b1af90, tpl_843::<0x7001c6abd8, 0x5, 0x2, 0x7001c5b9f0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42ef90 sub_42ef90
    Lifted::new(0x7000b1b0c0, tpl_843::<0x7001c6abd8, 0x5, 0x2, 0x7001c5ba20, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42f0c0 sub_42f0c0
    Lifted::new(0x7000b1b1f0, fn_7000b1b1f0 as *const (), 0), // +0x42f1f0 sub_42f1f0
    Lifted::new(0x7000b1b320, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bd68, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42f320 sub_42f320
    Lifted::new(0x7000b1b440, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b1e0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1b57c> as *const (), 0), // +0x42f440 sub_42f440
    Lifted::new(0x7000b1b5d0, tpl_844::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c380, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42f5d0 sub_42f5d0
    Lifted::new(0x7000b1b700, tpl_844::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c368, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42f700 sub_42f700
    Lifted::new(0x7000b1b830, tpl_844::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c350, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42f830 sub_42f830
    Lifted::new(0x7000b1b960, tpl_844::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb48, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42f960 sub_42f960
    Lifted::new(0x7000b1ba90, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bcf0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x42fa90 sub_42fa90
    Lifted::new(0x7000b1bbc0, fn_7000b1bbc0 as *const (), 2), // +0x42fbc0 sub_42fbc0
    Lifted::new(0x7000b1bdc0, fn_7000b1bdc0 as *const (), 2), // +0x42fdc0 sub_42fdc0
    Lifted::new(0x7000b1bfd0, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b1b0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1c102> as *const (), 0), // +0x42ffd0 sub_42ffd0
    Lifted::new(0x7000b1c150, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5be88, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x430150 sub_430150
    Lifted::new(0x7000b1c280, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bd20, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x430280 sub_430280
    Lifted::new(0x7000b1c3b0, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b2e8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1c4ec> as *const (), 0), // +0x4303b0 sub_4303b0
    Lifted::new(0x7000b1c540, fn_7000b1c540 as *const (), 0), // +0x430540 sub_430540
    Lifted::new(0x7000b1c670, fn_7000b1c670 as *const (), 0), // +0x430670 sub_430670
    Lifted::new(0x7000b1c7a0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b168, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1c8d7> as *const (), 0), // +0x4307a0 sub_4307a0
    Lifted::new(0x7000b1c930, tpl_845::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b9a8, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x430930 sub_430930
    Lifted::new(0x7000b1ca60, tpl_845::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b9c0, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x430a60 sub_430a60
    Lifted::new(0x7000b1cb90, tpl_838::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c200, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x430b90 sub_430b90
    Lifted::new(0x7000b1ccb0, tpl_838::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c1d0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x430cb0 sub_430cb0
    Lifted::new(0x7000b1cdd0, tpl_838::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c1e8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x430dd0 sub_430dd0
    Lifted::new(0x7000b1cef0, tpl_846::<0x7001c6abd8, 0x5, 0x2, 0x7001c5c140, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x430ef0 sub_430ef0
    Lifted::new(0x7000b1d020, tpl_846::<0x7001c6abd8, 0x5, 0x2, 0x7001c5c158, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x431020 sub_431020
    Lifted::new(0x7000b1d150, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b480, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1d287> as *const (), 0), // +0x431150 sub_431150
    Lifted::new(0x7000b1d2e0, fn_7000b1d2e0 as *const (), 0), // +0x4312e0 sub_4312e0
    Lifted::new(0x7000b1d490, fn_7000b1d490 as *const (), 0), // +0x431490 sub_431490
    Lifted::new(0x7000b1d630, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b4c8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1d762> as *const (), 0), // +0x431630 sub_431630
    Lifted::new(0x7000b1d7b0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b4e0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1d8e7> as *const (), 0), // +0x4317b0 sub_4317b0
    Lifted::new(0x7000b1d940, fn_7000b1d940 as *const (), 0), // +0x431940 sub_431940
    Lifted::new(0x7000b1da90, tpl_842::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bed0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x431a90 sub_431a90
    Lifted::new(0x7000b1dbc0, tpl_847::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c410, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x431bc0 sub_431bc0
    Lifted::new(0x7000b1dd00, tpl_847::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c428, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x431d00 sub_431d00
    Lifted::new(0x7000b1de40, tpl_841::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c248, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x431e40 sub_431e40
    Lifted::new(0x7000b1df70, tpl_848::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5cab8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x431f70 sub_431f70
    Lifted::new(0x7000b1e0a0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bc18, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x4320a0 sub_4320a0
    Lifted::new(0x7000b1e1b0, tpl_850::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bc48, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4321b0 sub_4321b0
    Lifted::new(0x7000b1e2e0, fn_7000b1e2e0 as *const (), 0), // +0x4322e0 sub_4322e0
    Lifted::new(0x7000b1e410, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b660, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1e551> as *const (), 0), // +0x432410 sub_432410
    Lifted::new(0x7000b1e5b0, fn_7000b1e5b0 as *const (), 0), // +0x4325b0 sub_4325b0
    Lifted::new(0x7000b1e6f0, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5be58, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4326f0 sub_4326f0
    Lifted::new(0x7000b1e820, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5be70, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x432820 sub_432820
    Lifted::new(0x7000b1e950, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b1c8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1ea87> as *const (), 0), // +0x432950 sub_432950
    Lifted::new(0x7000b1eae0, tpl_851::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bbb8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x432ae0 sub_432ae0
    Lifted::new(0x7000b1ec10, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bd80, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x432c10 sub_432c10
    Lifted::new(0x7000b1ed30, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b3f0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1ee67> as *const (), 0), // +0x432d30 sub_432d30
    Lifted::new(0x7000b1eec0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b450, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1eff7> as *const (), 0), // +0x432ec0 sub_432ec0
    Lifted::new(0x7000b1f050, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b420, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1f182> as *const (), 0), // +0x433050 sub_433050
    Lifted::new(0x7000b1f1d0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b3d8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1f307> as *const (), 0), // +0x4331d0 sub_4331d0
    Lifted::new(0x7000b1f360, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b438, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1f497> as *const (), 0), // +0x433360 sub_433360
    Lifted::new(0x7000b1f4f0, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b408, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b1f622> as *const (), 0), // +0x4334f0 sub_4334f0
    Lifted::new(0x7000b1f670, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b7c8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x433670 sub_433670
    Lifted::new(0x7000b1f7a0, fn_7000b1f7a0 as *const (), 0), // +0x4337a0 sub_4337a0
    Lifted::new(0x7000b1f8d0, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5be28, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4338d0 sub_4338d0
    Lifted::new(0x7000b1fa00, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5be40, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x433a00 sub_433a00
    Lifted::new(0x7000b1fb30, fn_7000b1fb30 as *const (), 0), // +0x433b30 sub_433b30
    Lifted::new(0x7000b1fc40, tpl_852::<0x7001c6abd8, 0x8, 0x7001c5aa50, 0xc, 0x4, 0x18, 0x7000707c70> as *const (), 0), // +0x433c40 sub_433c40
    Lifted::new(0x7000b1fd40, fn_7000b1fd40 as *const (), 0), // +0x433d40 sub_433d40
    Lifted::new(0x7000b1fe50, tpl_853::<0x7001c6abd8, 0x7001c5aa90, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x433e50 sub_433e50
    Lifted::new(0x7000b1ff60, tpl_853::<0x7001c6abd8, 0x7001c5aa70, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x433f60 sub_433f60
    Lifted::new(0x7000b20070, fn_7000b20070 as *const (), 0), // +0x434070 sub_434070
    Lifted::new(0x7000b20170, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b750, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434170 sub_434170
    Lifted::new(0x7000b202a0, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b780, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4342a0 sub_4342a0
    Lifted::new(0x7000b203d0, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b768, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4343d0 sub_4343d0
    Lifted::new(0x7000b20500, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c998, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434500 sub_434500
    Lifted::new(0x7000b20630, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ca28, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434630 sub_434630
    Lifted::new(0x7000b20760, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ca10, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434760 sub_434760
    Lifted::new(0x7000b20890, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c9c8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434890 sub_434890
    Lifted::new(0x7000b209c0, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c9e0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4349c0 sub_4349c0
    Lifted::new(0x7000b20af0, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ca88, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434af0 sub_434af0
    Lifted::new(0x7000b20c20, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c9b0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434c20 sub_434c20
    Lifted::new(0x7000b20d50, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ca58, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434d50 sub_434d50
    Lifted::new(0x7000b20e80, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ca70, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434e80 sub_434e80
    Lifted::new(0x7000b20fb0, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5caa0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x434fb0 sub_434fb0
    Lifted::new(0x7000b210e0, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c9f8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4350e0 sub_4350e0
    Lifted::new(0x7000b21210, tpl_854::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ca40, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435210 sub_435210
    Lifted::new(0x7000b21340, tpl_852::<0x7001c6abd8, 0x8, 0x7001c5aa80, 0xc, 0x4, 0x18, 0x7000707c70> as *const (), 0), // +0x435340 sub_435340
    Lifted::new(0x7000b21440, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ccc8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435440 sub_435440
    Lifted::new(0x7000b21580, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc80, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435580 sub_435580
    Lifted::new(0x7000b216c0, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cba8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4356c0 sub_4356c0
    Lifted::new(0x7000b21800, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cbf0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435800 sub_435800
    Lifted::new(0x7000b21940, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cde8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435940 sub_435940
    Lifted::new(0x7000b21a80, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb60, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435a80 sub_435a80
    Lifted::new(0x7000b21bc0, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cd58, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435bc0 sub_435bc0
    Lifted::new(0x7000b21d00, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cda0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435d00 sub_435d00
    Lifted::new(0x7000b21e40, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ce30, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435e40 sub_435e40
    Lifted::new(0x7000b21f80, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc38, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x435f80 sub_435f80
    Lifted::new(0x7000b220c0, tpl_855::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cd10, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4360c0 sub_4360c0
    Lifted::new(0x7000b22200, fn_7000b22200 as *const (), 0), // +0x436200 sub_436200
    Lifted::new(0x7000b22330, fn_7000b22330 as *const (), 0), // +0x436330 sub_436330
    Lifted::new(0x7000b22470, fn_7000b22470 as *const (), 0), // +0x436470 sub_436470
    Lifted::new(0x7000b225b0, tpl_856::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c2f0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4365b0 sub_4365b0
    Lifted::new(0x7000b226e0, tpl_856::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c2d8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4366e0 sub_4366e0
    Lifted::new(0x7000b22810, tpl_856::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c2c0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x436810 sub_436810
    Lifted::new(0x7000b22940, tpl_856::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb18, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x436940 sub_436940
    Lifted::new(0x7000b22a70, tpl_850::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5aab0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x436a70 sub_436a70
    Lifted::new(0x7000b22ba0, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b498, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b22cd2> as *const (), 0), // +0x436ba0 sub_436ba0
    Lifted::new(0x7000b22d20, tpl_844::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c3f8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x436d20 sub_436d20
    Lifted::new(0x7000b22e50, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bd08, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x436e50 sub_436e50
    Lifted::new(0x7000b22f80, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bcd8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x436f80 sub_436f80
    Lifted::new(0x7000b230b0, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bd38, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4370b0 sub_4370b0
    Lifted::new(0x7000b231d0, tpl_857::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bf00, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4371d0 sub_4371d0
    Lifted::new(0x7000b23300, tpl_858::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c1b8, 0xc, 0x18, 0x4, 0x8, 0x70017f89c4, 0x70017f89c8, 0x7000707c70> as *const (), 0), // +0x437300 sub_437300
    Lifted::new(0x7000b23450, tpl_858::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c188, 0xc, 0x18, 0x4, 0x8, 0x70017f89c4, 0x70017f89c8, 0x7000707c70> as *const (), 0), // +0x437450 sub_437450
    Lifted::new(0x7000b235a0, tpl_859::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c290, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4375a0 sub_4375a0
    Lifted::new(0x7000b236c0, tpl_858::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c1a0, 0xc, 0x18, 0x4, 0x8, 0x70017f89c4, 0x70017f89c8, 0x7000707c70> as *const (), 0), // +0x4376c0 sub_4376c0
    Lifted::new(0x7000b23810, fn_7000b23810 as *const (), 0), // +0x437810 sub_437810
    Lifted::new(0x7000b238c0, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5ba68, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4378c0 sub_4378c0
    Lifted::new(0x7000b239e0, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5ba80, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4379e0 sub_4379e0
    Lifted::new(0x7000b23b00, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bb40, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x437b00 sub_437b00
    Lifted::new(0x7000b23c20, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bb58, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x437c20 sub_437c20
    Lifted::new(0x7000b23d40, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bb70, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x437d40 sub_437d40
    Lifted::new(0x7000b23e60, tpl_860::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bab0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x437e60 sub_437e60
    Lifted::new(0x7000b23f80, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b4b0, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b240b2> as *const (), 0), // +0x437f80 sub_437f80
    Lifted::new(0x7000b24100, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b180, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b24237> as *const (), 0), // +0x438100 sub_438100
    Lifted::new(0x7000b24290, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bb88, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438290 sub_438290
    Lifted::new(0x7000b243b0, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bba0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4383b0 sub_4383b0
    Lifted::new(0x7000b244d0, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bc90, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4384d0 sub_4384d0
    Lifted::new(0x7000b24600, fn_7000b24600 as *const (), 0), // +0x438600 sub_438600
    Lifted::new(0x7000b24730, tpl_861::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bf30, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438730 sub_438730
    Lifted::new(0x7000b24860, tpl_861::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bf48, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438860 sub_438860
    Lifted::new(0x7000b24990, tpl_862::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bf60, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438990 sub_438990
    Lifted::new(0x7000b24ac0, tpl_863::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bf90, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438ac0 sub_438ac0
    Lifted::new(0x7000b24bf0, fn_7000b24bf0 as *const (), 0), // +0x438bf0 sub_438bf0
    Lifted::new(0x7000b24d20, fn_7000b24d20 as *const (), 0), // +0x438d20 sub_438d20
    Lifted::new(0x7000b24e50, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bfd8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438e50 sub_438e50
    Lifted::new(0x7000b24f70, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bff0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x438f70 sub_438f70
    Lifted::new(0x7000b250a0, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c008, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4390a0 sub_4390a0
    Lifted::new(0x7000b251c0, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c020, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4391c0 sub_4391c0
    Lifted::new(0x7000b252f0, tpl_846::<0x7001c6abd8, 0x5, 0x2, 0x7001c5c038, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4392f0 sub_4392f0
    Lifted::new(0x7000b25420, fn_7000b25420 as *const (), 0), // +0x439420 sub_439420
    Lifted::new(0x7000b25550, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b4f8, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b25691> as *const (), 0), // +0x439550 sub_439550
    Lifted::new(0x7000b256f0, tpl_831::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b510, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b25831> as *const (), 0), // +0x4396f0 sub_4396f0
    Lifted::new(0x7000b25890, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c080, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x439890 sub_439890
    Lifted::new(0x7000b259c0, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c068, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x4399c0 sub_4399c0
    Lifted::new(0x7000b25af0, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c0e0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x439af0 sub_439af0
    Lifted::new(0x7000b25c20, tpl_840::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c098, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x439c20 sub_439c20
    Lifted::new(0x7000b25d50, tpl_837::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c0b0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x439d50 sub_439d50
    Lifted::new(0x7000b25e80, tpl_863::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c0c8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x439e80 sub_439e80
    Lifted::new(0x7000b25fb0, fn_7000b25fb0 as *const (), 0), // +0x439fb0 sub_439fb0
    Lifted::new(0x7000b260e0, tpl_862::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bf78, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43a0e0 sub_43a0e0
    Lifted::new(0x7000b26210, fn_7000b26210 as *const (), 0), // +0x43a210 sub_43a210
    Lifted::new(0x7000b26340, fn_7000b26340 as *const (), 0), // +0x43a340 sub_43a340
    Lifted::new(0x7000b26450, fn_7000b26450 as *const (), 0), // +0x43a450 sub_43a450
    Lifted::new(0x7000b26580, fn_7000b26580 as *const (), 0), // +0x43a580 sub_43a580
    Lifted::new(0x7000b266a0, fn_7000b266a0 as *const (), 0), // +0x43a6a0 sub_43a6a0
    Lifted::new(0x7000b267b0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b210, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b268e7> as *const (), 0), // +0x43a7b0 sub_43a7b0
    Lifted::new(0x7000b26940, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b528, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b26a72> as *const (), 0), // +0x43a940 sub_43a940
    Lifted::new(0x7000b26ac0, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b678, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b26bfc> as *const (), 0), // +0x43aac0 sub_43aac0
    Lifted::new(0x7000b26c50, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b690, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b26d8c> as *const (), 0), // +0x43ac50 sub_43ac50
    Lifted::new(0x7000b26de0, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bd98, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43ade0 sub_43ade0
    Lifted::new(0x7000b26f10, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bdb0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43af10 sub_43af10
    Lifted::new(0x7000b27040, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bdc8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43b040 sub_43b040
    Lifted::new(0x7000b27170, fn_7000b27170 as *const (), 0), // +0x43b170 sub_43b170
    Lifted::new(0x7000b27240, tpl_864::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c398, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43b240 sub_43b240
    Lifted::new(0x7000b27370, tpl_864::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c3b0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43b370 sub_43b370
    Lifted::new(0x7000b274a0, fn_7000b274a0 as *const (), 0), // +0x43b4a0 sub_43b4a0
    Lifted::new(0x7000b275e0, fn_7000b275e0 as *const (), 0), // +0x43b5e0 sub_43b5e0
    Lifted::new(0x7000b27720, fn_7000b27720 as *const (), 0), // +0x43b720 sub_43b720
    Lifted::new(0x7000b27830, fn_7000b27830 as *const (), 0), // +0x43b830 sub_43b830
    Lifted::new(0x7000b27930, fn_7000b27930 as *const (), 0), // +0x43b930 sub_43b930
    Lifted::new(0x7000b27a70, fn_7000b27a70 as *const (), 2), // +0x43ba70 sub_43ba70
    Lifted::new(0x7000b27bc0, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ccf8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43bbc0 sub_43bbc0
    Lifted::new(0x7000b27d00, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ccb0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43bd00 sub_43bd00
    Lifted::new(0x7000b27e40, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cbd8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43be40 sub_43be40
    Lifted::new(0x7000b27f80, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc20, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43bf80 sub_43bf80
    Lifted::new(0x7000b280c0, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ce18, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c0c0 sub_43c0c0
    Lifted::new(0x7000b28200, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb90, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c200 sub_43c200
    Lifted::new(0x7000b28340, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cd88, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c340 sub_43c340
    Lifted::new(0x7000b28480, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cdd0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c480 sub_43c480
    Lifted::new(0x7000b285c0, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ce60, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c5c0 sub_43c5c0
    Lifted::new(0x7000b28700, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc68, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c700 sub_43c700
    Lifted::new(0x7000b28840, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cd40, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c840 sub_43c840
    Lifted::new(0x7000b28980, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cce0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43c980 sub_43c980
    Lifted::new(0x7000b28ac0, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc98, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43cac0 sub_43cac0
    Lifted::new(0x7000b28c00, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cbc0, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43cc00 sub_43cc00
    Lifted::new(0x7000b28d40, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc08, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43cd40 sub_43cd40
    Lifted::new(0x7000b28e80, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ce00, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43ce80 sub_43ce80
    Lifted::new(0x7000b28fc0, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb78, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43cfc0 sub_43cfc0
    Lifted::new(0x7000b29100, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cd70, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43d100 sub_43d100
    Lifted::new(0x7000b29240, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cdb8, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43d240 sub_43d240
    Lifted::new(0x7000b29380, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ce48, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43d380 sub_43d380
    Lifted::new(0x7000b294c0, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cc50, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43d4c0 sub_43d4c0
    Lifted::new(0x7000b29600, tpl_865::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cd28, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43d600 sub_43d600
    Lifted::new(0x7000b29740, fn_7000b29740 as *const (), 0), // +0x43d740 sub_43d740
    Lifted::new(0x7000b29870, tpl_866::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c338, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43d870 sub_43d870
    Lifted::new(0x7000b299a0, tpl_866::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c320, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43d9a0 sub_43d9a0
    Lifted::new(0x7000b29ad0, tpl_866::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c308, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43dad0 sub_43dad0
    Lifted::new(0x7000b29c00, tpl_866::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5cb30, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43dc00 sub_43dc00
    Lifted::new(0x7000b29d30, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bea0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43dd30 sub_43dd30
    Lifted::new(0x7000b29e60, tpl_857::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b948, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43de60 sub_43de60
    Lifted::new(0x7000b29f90, tpl_842::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b960, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43df90 sub_43df90
    Lifted::new(0x7000b2a0c0, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b630, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b2a1f7> as *const (), 0), // +0x43e0c0 sub_43e0c0
    Lifted::new(0x7000b2a250, fn_7000b2a250 as *const (), 0), // +0x43e250 sub_43e250
    Lifted::new(0x7000b2a380, fn_7000b2a380 as *const (), 0), // +0x43e380 sub_43e380
    Lifted::new(0x7000b2a4b0, tpl_845::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b978, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x43e4b0 sub_43e4b0
    Lifted::new(0x7000b2a5e0, fn_7000b2a5e0 as *const (), 0), // +0x43e5e0 sub_43e5e0
    Lifted::new(0x7000b2a710, tpl_835::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b468, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b2a842> as *const (), 0), // +0x43e710 sub_43e710
    Lifted::new(0x7000b2a890, tpl_851::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c110, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43e890 sub_43e890
    Lifted::new(0x7000b2a9c0, tpl_867::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c218, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43e9c0 sub_43e9c0
    Lifted::new(0x7000b2aaf0, fn_7000b2aaf0 as *const (), 0), // +0x43eaf0 sub_43eaf0
    Lifted::new(0x7000b2ac20, tpl_838::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b930, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43ec20 sub_43ec20
    Lifted::new(0x7000b2ad40, tpl_832::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5ba50, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43ed40 sub_43ed40
    Lifted::new(0x7000b2ae60, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b648, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b2af9c> as *const (), 0), // +0x43ee60 sub_43ee60
    Lifted::new(0x7000b2aff0, tpl_827::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5bcc0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43eff0 sub_43eff0
    Lifted::new(0x7000b2b120, fn_7000b2b120 as *const (), 0), // +0x43f120 sub_43f120
    Lifted::new(0x7000b2b260, tpl_868::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c260, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43f260 sub_43f260
    Lifted::new(0x7000b2b390, tpl_868::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c278, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x43f390 sub_43f390
    Lifted::new(0x7000b2b4c0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c968, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43f4c0 sub_43f4c0
    Lifted::new(0x7000b2b5d0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c980, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43f5d0 sub_43f5d0
    Lifted::new(0x7000b2b6e0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c8d8, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43f6e0 sub_43f6e0
    Lifted::new(0x7000b2b7f0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c860, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43f7f0 sub_43f7f0
    Lifted::new(0x7000b2b900, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c878, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43f900 sub_43f900
    Lifted::new(0x7000b2ba10, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c8f0, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43fa10 sub_43fa10
    Lifted::new(0x7000b2bb20, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c8a8, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43fb20 sub_43fb20
    Lifted::new(0x7000b2bc30, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c8c0, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43fc30 sub_43fc30
    Lifted::new(0x7000b2bd40, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c950, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43fd40 sub_43fd40
    Lifted::new(0x7000b2be50, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c830, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43fe50 sub_43fe50
    Lifted::new(0x7000b2bf60, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c938, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x43ff60 sub_43ff60
    Lifted::new(0x7000b2c070, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c890, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x440070 sub_440070
    Lifted::new(0x7000b2c180, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c920, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x440180 sub_440180
    Lifted::new(0x7000b2c290, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c908, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x440290 sub_440290
    Lifted::new(0x7000b2c3a0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c848, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x4403a0 sub_4403a0
    Lifted::new(0x7000b2c4b0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c818, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x4404b0 sub_4404b0
    Lifted::new(0x7000b2c5c0, fn_7000b2c5c0 as *const (), 0), // +0x4405c0 sub_4405c0
    Lifted::new(0x7000b2c6f0, fn_7000b2c6f0 as *const (), 0), // +0x4406f0 sub_4406f0
    Lifted::new(0x7000b2c820, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b840, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x440820 sub_440820
    Lifted::new(0x7000b2c940, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5baf8, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x440940 sub_440940
    Lifted::new(0x7000b2ca60, tpl_860::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bb10, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x440a60 sub_440a60
    Lifted::new(0x7000b2cb80, tpl_839::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5ba98, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x440b80 sub_440b80
    Lifted::new(0x7000b2cca0, tpl_860::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5bb28, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x440ca0 sub_440ca0
    Lifted::new(0x7000b2cdc0, tpl_860::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5b918, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x440dc0 sub_440dc0
    Lifted::new(0x7000b2cee0, tpl_869::<0x7001c6abd8, 0xc, 0x7001c71cec, 0x4, 0x8, 0x2, 0x18, 0x7000707c70> as *const (), 0), // +0x440ee0 sub_440ee0
    Lifted::new(0x7000b2cfb0, fn_7000b2cfb0 as *const (), 0), // +0x440fb0 sub_440fb0
    Lifted::new(0x7000b2d090, tpl_870::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5ac38, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x441090 sub_441090
    Lifted::new(0x7000b2d1c0, fn_7000b2d1c0 as *const (), 0), // +0x4411c0 sub_4411c0
    Lifted::new(0x7000b2d310, tpl_869::<0x7001c6abd8, 0xc, 0x7001c5ac68, 0x4, 0x8, 0x2, 0x18, 0x7000707c70> as *const (), 0), // +0x441310 sub_441310
    Lifted::new(0x7000b2d3e0, tpl_871::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c788, 0xc, 0x18, 0x4, 0x8, 0x70017f89bc, 0x7000707c70> as *const (), 0), // +0x4413e0 sub_4413e0
    Lifted::new(0x7000b2d520, tpl_870::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5b150, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x441520 sub_441520
    Lifted::new(0x7000b2d650, fn_7000b2d650 as *const (), 0), // +0x441650 sub_441650
    Lifted::new(0x7000b2d780, tpl_872::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c6e0, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x441780 sub_441780
    Lifted::new(0x7000b2d8c0, tpl_872::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c6c8, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x4418c0 sub_4418c0
    Lifted::new(0x7000b2da00, tpl_872::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c6b0, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x441a00 sub_441a00
    Lifted::new(0x7000b2db40, tpl_873::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c560, 0xc, 0x18, 0x4, 0x8, 0x70017f89c8, 0x7000707c70> as *const (), 0), // +0x441b40 sub_441b40
    Lifted::new(0x7000b2dc80, tpl_874::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c680, 0xc, 0x18, 0x4, 0x8, 0x70017f89c8, 0x7000707c70> as *const (), 0), // +0x441c80 sub_441c80
    Lifted::new(0x7000b2ddc0, tpl_859::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c638, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x441dc0 sub_441dc0
    Lifted::new(0x7000b2dee0, tpl_860::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c728, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x441ee0 sub_441ee0
    Lifted::new(0x7000b2e000, fn_7000b2e000 as *const (), 0), // +0x442000 sub_442000
    Lifted::new(0x7000b2e130, tpl_874::<0x7001c6abd8, 0x5, 0xfffffffffffffffc, 0x7001c5c668, 0xc, 0x18, 0x4, 0x8, 0x70017f89c8, 0x7000707c70> as *const (), 0), // +0x442130 sub_442130
    Lifted::new(0x7000b2e270, tpl_867::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c698, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x442270 sub_442270
    Lifted::new(0x7000b2e3a0, fn_7000b2e3a0 as *const (), 0), // +0x4423a0 sub_4423a0
    Lifted::new(0x7000b2e4f0, fn_7000b2e4f0 as *const (), 0), // +0x4424f0 sub_4424f0
    Lifted::new(0x7000b2e640, fn_7000b2e640 as *const (), 0), // +0x442640 sub_442640
    Lifted::new(0x7000b2e780, fn_7000b2e780 as *const (), 0), // +0x442780 sub_442780
    Lifted::new(0x7000b2e8b0, fn_7000b2e8b0 as *const (), 0), // +0x4428b0 sub_4428b0
    Lifted::new(0x7000b2e9e0, fn_7000b2e9e0 as *const (), 0), // +0x4429e0 sub_4429e0
    Lifted::new(0x7000b2eb10, fn_7000b2eb10 as *const (), 0), // +0x442b10 sub_442b10
    Lifted::new(0x7000b2ec50, fn_7000b2ec50 as *const (), 0), // +0x442c50 sub_442c50
    Lifted::new(0x7000b2edb0, fn_7000b2edb0 as *const (), 0), // +0x442db0 sub_442db0
    Lifted::new(0x7000b2eef0, fn_7000b2eef0 as *const (), 0), // +0x442ef0 sub_442ef0
    Lifted::new(0x7000b2f040, tpl_873::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c458, 0xc, 0x18, 0x4, 0x8, 0x70017f89cc, 0x7000707c70> as *const (), 0), // +0x443040 sub_443040
    Lifted::new(0x7000b2f180, tpl_871::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c758, 0xc, 0x18, 0x4, 0x8, 0x70017f89cc, 0x7000707c70> as *const (), 0), // +0x443180 sub_443180
    Lifted::new(0x7000b2f2c0, fn_7000b2f2c0 as *const (), 0), // +0x4432c0 sub_4432c0
    Lifted::new(0x7000b2f400, tpl_871::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c548, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x443400 sub_443400
    Lifted::new(0x7000b2f540, tpl_841::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c590, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x443540 sub_443540
    Lifted::new(0x7000b2f670, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c5c0, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x443670 sub_443670
    Lifted::new(0x7000b2f780, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c5d8, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x443780 sub_443780
    Lifted::new(0x7000b2f890, fn_7000b2f890 as *const (), 0), // +0x443890 sub_443890
    Lifted::new(0x7000b2f9d0, tpl_875::<0x7001c6abd8, 0x2, 0x7001c5c4d0, 0x8, 0x18, 0xc, 0x4, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x4439d0 sub_4439d0
    Lifted::new(0x7000b2fb20, fn_7000b2fb20 as *const (), 0), // +0x443b20 sub_443b20
    Lifted::new(0x7000b2fc60, fn_7000b2fc60 as *const (), 0), // +0x443c60 sub_443c60
    Lifted::new(0x7000b2fdb0, fn_7000b2fdb0 as *const (), 0), // +0x443db0 sub_443db0
    Lifted::new(0x7000b2fef0, tpl_875::<0x7001c6abd8, 0x2, 0x7001c5c4b8, 0x8, 0x18, 0xc, 0x4, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x443ef0 sub_443ef0
    Lifted::new(0x7000b30040, fn_7000b30040 as *const (), 0), // +0x444040 sub_444040
    Lifted::new(0x7000b30190, fn_7000b30190 as *const (), 0), // +0x444190 sub_444190
    Lifted::new(0x7000b302c0, fn_7000b302c0 as *const (), 0), // +0x4442c0 sub_4442c0
    Lifted::new(0x7000b303f0, fn_7000b303f0 as *const (), 0), // +0x4443f0 sub_4443f0
    Lifted::new(0x7000b30520, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c608, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x444520 sub_444520
    Lifted::new(0x7000b30630, tpl_848::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c620, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x444630 sub_444630
    Lifted::new(0x7000b30760, tpl_848::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c5f0, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x444760 sub_444760
    Lifted::new(0x7000b30890, tpl_848::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c800, 0xc, 0x18, 0x4, 0x8, 0x7000707c70> as *const (), 0), // +0x444890 sub_444890
    Lifted::new(0x7000b309c0, tpl_849::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c5a8, 0xc, 0x18, 0x4, 0x7000707c70> as *const (), 0), // +0x4449c0 sub_4449c0
    Lifted::new(0x7000b30ad0, tpl_871::<0x7001c6abd8, 0x2, 0xfffffffffffffffc, 0x7001c5c770, 0xc, 0x18, 0x4, 0x8, 0x70017f89c0, 0x7000707c70> as *const (), 0), // +0x444ad0 sub_444ad0
    Lifted::new(0x7000b30c10, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b558, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b30d47> as *const (), 0), // +0x444c10 sub_444c10
    Lifted::new(0x7000b30da0, tpl_834::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b570, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b30edc> as *const (), 0), // +0x444da0 sub_444da0
    Lifted::new(0x7000b30f30, tpl_833::<0x7001c6abd8, 0x70018da5bc, 0x7001c5b540, 0x700177aa00, 0x7000724770, 0x70017d8c38, 0x7000723490, 0x7000b31067> as *const (), 0), // +0x444f30 sub_444f30
    Lifted::new(0x7000b310c0, fn_7000b310c0 as *const (), 0), // +0x4450c0 sub_4450c0
    Lifted::new(0x7000b31310, fn_7000b31310 as *const (), 2), // +0x445310 sub_445310
    Lifted::new(0x7000b36d80, fn_7000b36d80 as *const (), 1), // +0x44ad80 sub_44ad80
    Lifted::new(0x7000b36db0, fn_7000b36db0 as *const (), 0), // +0x44adb0 sub_44adb0
    Lifted::new(0x7000b36e10, fn_7000b36e10 as *const (), 0), // +0x44ae10 sub_44ae10
    Lifted::new(0x7000b36ff0, tpl_44 as *const (), 1), // +0x44aff0 sub_44aff0
    Lifted::new(0x7000b37000, fn_7000b37000 as *const (), 0), // +0x44b000 sub_44b000
    Lifted::new(0x7000b37030, fn_7000b37030 as *const (), 0), // +0x44b030 sub_44b030
    Lifted::new(0x7000b37090, fn_7000b37090 as *const (), 0), // +0x44b090 sub_44b090
    Lifted::new(0x7000b37130, fn_7000b37130 as *const (), 0), // +0x44b130 sub_44b130
    Lifted::new(0x7000b37160, fn_7000b37160 as *const (), 0), // +0x44b160 sub_44b160
    Lifted::new(0x7000b37270, fn_7000b37270 as *const (), 1), // +0x44b270 sub_44b270
    Lifted::new(0x7000b372b0, fn_7000b372b0 as *const (), 0), // +0x44b2b0 sub_44b2b0
    Lifted::new(0x7000b37420, tpl_876::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b37621, 0x7000b38790, 0x7000b37627, 0x70007f0910, 0x7000b3762d> as *const (), 2), // +0x44b420 sub_44b420
    Lifted::new(0x7000b376c0, fn_7000b376c0 as *const (), 1), // +0x44b6c0 sub_44b6c0
    Lifted::new(0x7000b37770, tpl_877::<0x70017fd0b0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x70007ed7e0, 0x70017fd0f8> as *const (), 0), // +0x44b770 sub_44b770
    Lifted::new(0x7000b377e0, tpl_877::<0x70017fd0b0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x70007ed7e0, 0x70017fd0f8> as *const (), 0), // +0x44b7e0 sub_44b7e0
    Lifted::new(0x7000b37870, fn_7000b37870 as *const (), 0), // +0x44b870 sub_44b870
    Lifted::new(0x7000b37b80, fn_7000b37b80 as *const (), 2), // +0x44bb80 sub_44bb80
    Lifted::new(0x7000b37cf0, fn_7000b37cf0 as *const (), 1), // +0x44bcf0 sub_44bcf0
    Lifted::new(0x7000b37d10, fn_7000b37d10 as *const (), 0), // +0x44bd10 sub_44bd10
    Lifted::new(0x7000b37d30, fn_7000b37d30 as *const (), 0), // +0x44bd30 sub_44bd30
    Lifted::new(0x7000b37e90, tpl_878::<0x8, 0x30, 0x10, 0x18, 0x20, 0x28, 0x70009a6500> as *const (), 0), // +0x44be90 sub_44be90
    Lifted::new(0x7000b37f00, tpl_878::<0x8, 0x30, 0x10, 0x18, 0x20, 0x28, 0x70009a6500> as *const (), 0), // +0x44bf00 sub_44bf00
    Lifted::new(0x7000b37f70, fn_7000b37f70 as *const (), 1), // +0x44bf70 sub_44bf70
    Lifted::new(0x7000b37f90, tpl_28 as *const (), 1), // +0x44bf90 sub_44bf90
    Lifted::new(0x7000b37fa0, tpl_20 as *const (), 1), // +0x44bfa0 sub_44bfa0
    Lifted::new(0x7000b37fb0, tpl_20 as *const (), 1), // +0x44bfb0 sub_44bfb0
    Lifted::new(0x7000b37fc0, fn_7000b37fc0 as *const (), 0), // +0x44bfc0 sub_44bfc0
    Lifted::new(0x7000b38090, tpl_879::<0x70007497e0, 0x700177c312> as *const (), 0), // +0x44c090 sub_44c090
    Lifted::new(0x7000b380d0, tpl_879::<0x70007497e0, 0x700177c312> as *const (), 0), // +0x44c0d0 sub_44c0d0
    Lifted::new(0x7000b38110, fn_7000b38110 as *const (), 1), // +0x44c110 sub_44c110
    Lifted::new(0x7000b38120, fn_7000b38120 as *const (), 2), // +0x44c120 sub_44c120
    Lifted::new(0x7000b381b0, fn_7000b381b0 as *const (), 0), // +0x44c1b0 sub_44c1b0
    Lifted::new(0x7000b382f0, fn_7000b382f0 as *const (), 2), // +0x44c2f0 sub_44c2f0
    Lifted::new(0x7000b383f0, fn_7000b383f0 as *const (), 0), // +0x44c3f0 sub_44c3f0
    Lifted::new(0x7000b384b0, tpl_803::<0x38> as *const (), 1), // +0x44c4b0 sub_44c4b0
    Lifted::new(0x7000b384d0, fn_7000b384d0 as *const (), 0), // +0x44c4d0 sub_44c4d0
    Lifted::new(0x7000b38620, fn_7000b38620 as *const (), 1), // +0x44c620 sub_44c620
    Lifted::new(0x7000b38630, tpl_51::<0x28> as *const (), 1), // +0x44c630 sub_44c630
    Lifted::new(0x7000b38640, fn_7000b38640 as *const (), 1), // +0x44c640 sub_44c640
    Lifted::new(0x7000b386b0, fn_7000b386b0 as *const (), 0), // +0x44c6b0 sub_44c6b0
    Lifted::new(0x7000b38790, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b387a0> as *const (), 0), // +0x44c790 sub_44c790
    Lifted::new(0x7000b387b0, fn_7000b387b0 as *const (), 1), // +0x44c7b0 sub_44c7b0
    Lifted::new(0x7000b387e0, tpl_327::<0x10> as *const (), 1), // +0x44c7e0 sub_44c7e0
    Lifted::new(0x7000b387f0, tpl_47::<0x5> as *const (), 1), // +0x44c7f0 sub_44c7f0
    Lifted::new(0x7000b38800, tpl_41 as *const (), 1), // +0x44c800 sub_44c800
    Lifted::new(0x7000b38810, fn_7000b38810 as *const (), 0), // +0x44c810 sub_44c810
    Lifted::new(0x7000b38880, tpl_172::<0x20> as *const (), 1), // +0x44c880 sub_44c880
    Lifted::new(0x7000b38890, tpl_880::<0x20, 0xf, 0xffffffffffffffff, 0x13> as *const (), 1), // +0x44c890 sub_44c890
    Lifted::new(0x7000b388e0, tpl_880::<0x20, 0xf, 0xffffffffffffffff, 0x13> as *const (), 1), // +0x44c8e0 sub_44c8e0
    Lifted::new(0x7000b38930, fn_7000b38930 as *const (), 1), // +0x44c930 sub_44c930
    Lifted::new(0x7000b38940, tpl_47::<0x8> as *const (), 1), // +0x44c940 sub_44c940
    Lifted::new(0x7000b38950, tpl_30 as *const (), 1), // +0x44c950 sub_44c950
    Lifted::new(0x7000b38960, fn_7000b38960 as *const (), 1), // +0x44c960 sub_44c960
    Lifted::new(0x7000b389b0, fn_7000b389b0 as *const (), 0), // +0x44c9b0 sub_44c9b0
    Lifted::new(0x7000b38a00, fn_7000b38a00 as *const (), 3), // +0x44ca00 sub_44ca00
    Lifted::new(0x7000b38a80, tpl_15 as *const (), 1), // +0x44ca80 sub_44ca80
    Lifted::new(0x7000b38a90, fn_7000b38a90 as *const (), 3), // +0x44ca90 sub_44ca90
    Lifted::new(0x7000b38ae0, fn_7000b38ae0 as *const (), 0), // +0x44cae0 sub_44cae0
    Lifted::new(0x7000b38b30, tpl_36::<0x10> as *const (), 1), // +0x44cb30 sub_44cb30
    Lifted::new(0x7000b38b40, tpl_47::<0x4> as *const (), 1), // +0x44cb40 sub_44cb40
    Lifted::new(0x7000b38b50, tpl_20 as *const (), 1), // +0x44cb50 sub_44cb50
    Lifted::new(0x7000b38b60, fn_7000b38b60 as *const (), 0), // +0x44cb60 sub_44cb60
    Lifted::new(0x7000b38e10, fn_7000b38e10 as *const (), 0), // +0x44ce10 sub_44ce10
    Lifted::new(0x7000b39040, fn_7000b39040 as *const (), 0), // +0x44d040 sub_44d040
    Lifted::new(0x7000b39350, fn_7000b39350 as *const (), 0), // +0x44d350 sub_44d350
    Lifted::new(0x7000b396e0, fn_7000b396e0 as *const (), 3), // +0x44d6e0 sub_44d6e0
    Lifted::new(0x7000b39870, fn_7000b39870 as *const (), 1), // +0x44d870 sub_44d870
    Lifted::new(0x7000b39b20, tpl_15 as *const (), 1), // +0x44db20 sub_44db20
    Lifted::new(0x7000b39b30, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x44db30 sub_44db30
    Lifted::new(0x7000b39b80, tpl_33::<0x700177c312> as *const (), 0), // +0x44db80 sub_44db80
    Lifted::new(0x7000b39bb0, tpl_33::<0x700177c312> as *const (), 0), // +0x44dbb0 sub_44dbb0
    Lifted::new(0x7000b39be0, tpl_33::<0x700177c312> as *const (), 0), // +0x44dbe0 sub_44dbe0
    Lifted::new(0x7000b39c10, fn_7000b39c10 as *const (), 0), // +0x44dc10 sub_44dc10
    Lifted::new(0x7000b39c70, fn_7000b39c70 as *const (), 1), // +0x44dc70 sub_44dc70
    Lifted::new(0x7000b39cb0, fn_7000b39cb0 as *const (), 2), // +0x44dcb0 sub_44dcb0
    Lifted::new(0x7000b39ea0, fn_7000b39ea0 as *const (), 1), // +0x44dea0 sub_44dea0
    Lifted::new(0x7000b39f00, tpl_881::<0xc0> as *const (), 1), // +0x44df00 sub_44df00
    Lifted::new(0x7000b39f20, tpl_882::<0x40> as *const (), 1), // +0x44df20 sub_44df20
    Lifted::new(0x7000b39f30, tpl_881::<0x38> as *const (), 1), // +0x44df30 sub_44df30
    Lifted::new(0x7000b39f40, tpl_882::<0xc8> as *const (), 1), // +0x44df40 sub_44df40
    Lifted::new(0x7000b39f50, tpl_883::<0x30> as *const (), 1), // +0x44df50 sub_44df50
    Lifted::new(0x7000b39f60, tpl_883::<0x2c> as *const (), 1), // +0x44df60 sub_44df60
    Lifted::new(0x7000b39f70, tpl_883::<0x28> as *const (), 1), // +0x44df70 sub_44df70
    Lifted::new(0x7000b39f80, tpl_883::<0x34> as *const (), 1), // +0x44df80 sub_44df80
    Lifted::new(0x7000b39f90, fn_7000b39f90 as *const (), 2), // +0x44df90 sub_44df90
    Lifted::new(0x7000b3a1d0, fn_7000b3a1d0 as *const (), 0), // +0x44e1d0 sub_44e1d0
    Lifted::new(0x7000b3a4f0, tpl_36::<0xc0> as *const (), 1), // +0x44e4f0 sub_44e4f0
    Lifted::new(0x7000b3a500, tpl_39::<0x30> as *const (), 1), // +0x44e500 sub_44e500
    Lifted::new(0x7000b3a510, tpl_52::<0x40> as *const (), 1), // +0x44e510 sub_44e510
    Lifted::new(0x7000b3a520, tpl_39::<0x2c> as *const (), 1), // +0x44e520 sub_44e520
    Lifted::new(0x7000b3a530, tpl_36::<0x38> as *const (), 1), // +0x44e530 sub_44e530
    Lifted::new(0x7000b3a540, tpl_39::<0x28> as *const (), 1), // +0x44e540 sub_44e540
    Lifted::new(0x7000b3a550, tpl_52::<0xc8> as *const (), 1), // +0x44e550 sub_44e550
    Lifted::new(0x7000b3a560, tpl_39::<0x148> as *const (), 1), // +0x44e560 sub_44e560
    Lifted::new(0x7000b3a570, tpl_39::<0x20> as *const (), 1), // +0x44e570 sub_44e570
    Lifted::new(0x7000b3a580, tpl_39::<0x1c> as *const (), 1), // +0x44e580 sub_44e580
    Lifted::new(0x7000b3a590, tpl_39::<0x18> as *const (), 1), // +0x44e590 sub_44e590
    Lifted::new(0x7000b3a5a0, tpl_39::<0x24> as *const (), 1), // +0x44e5a0 sub_44e5a0
    Lifted::new(0x7000b3a5b0, tpl_39::<0x10> as *const (), 1), // +0x44e5b0 sub_44e5b0
    Lifted::new(0x7000b3a5c0, tpl_39::<0xc> as *const (), 1), // +0x44e5c0 sub_44e5c0
    Lifted::new(0x7000b3a5d0, tpl_39::<0x8> as *const (), 1), // +0x44e5d0 sub_44e5d0
    Lifted::new(0x7000b3a5e0, tpl_39::<0x14> as *const (), 1), // +0x44e5e0 sub_44e5e0
    Lifted::new(0x7000b3a5f0, fn_7000b3a5f0 as *const (), 0), // +0x44e5f0 sub_44e5f0
    Lifted::new(0x7000b3a660, fn_7000b3a660 as *const (), 0), // +0x44e660 sub_44e660
    Lifted::new(0x7000b3a6e0, fn_7000b3a6e0 as *const (), 0), // +0x44e6e0 sub_44e6e0
    Lifted::new(0x7000b3aa00, fn_7000b3aa00 as *const (), 0), // +0x44ea00 sub_44ea00
    Lifted::new(0x7000b3aa70, fn_7000b3aa70 as *const (), 0), // +0x44ea70 sub_44ea70
    Lifted::new(0x7000b3aac0, fn_7000b3aac0 as *const (), 0), // +0x44eac0 sub_44eac0
    Lifted::new(0x7000b3abc0, fn_7000b3abc0 as *const (), 0), // +0x44ebc0 sub_44ebc0
    Lifted::new(0x7000b3ac90, fn_7000b3ac90 as *const (), 1), // +0x44ec90 sub_44ec90
    Lifted::new(0x7000b3aca0, fn_7000b3aca0 as *const (), 1), // +0x44eca0 sub_44eca0
    Lifted::new(0x7000b3ad50, fn_7000b3ad50 as *const (), 1), // +0x44ed50 sub_44ed50
    Lifted::new(0x7000b3ada0, fn_7000b3ada0 as *const (), 1), // +0x44eda0 sub_44eda0
    Lifted::new(0x7000b3ae00, fn_7000b3ae00 as *const (), 1), // +0x44ee00 sub_44ee00
    Lifted::new(0x7000b3ae20, fn_7000b3ae20 as *const (), 1), // +0x44ee20 sub_44ee20
    Lifted::new(0x7000b3ae50, tpl_144 as *const (), 1), // +0x44ee50 sub_44ee50
    Lifted::new(0x7000b3ae60, fn_7000b3ae60 as *const (), 1), // +0x44ee60 sub_44ee60
    Lifted::new(0x7000b3ae70, fn_7000b3ae70 as *const (), 1), // +0x44ee70 sub_44ee70
    Lifted::new(0x7000b3ae90, fn_7000b3ae90 as *const (), 0), // +0x44ee90 sub_44ee90
    Lifted::new(0x7000b3aee0, fn_7000b3aee0 as *const (), 0), // +0x44eee0 sub_44eee0
    Lifted::new(0x7000b3af60, fn_7000b3af60 as *const (), 0), // +0x44ef60 sub_44ef60
    Lifted::new(0x7000b3b170, fn_7000b3b170 as *const (), 1), // +0x44f170 sub_44f170
    Lifted::new(0x7000b3b1e0, fn_7000b3b1e0 as *const (), 1), // +0x44f1e0 sub_44f1e0
    Lifted::new(0x7000b3b410, fn_7000b3b410 as *const (), 0), // +0x44f410 sub_44f410
    Lifted::new(0x7000b3b4f0, tpl_884::<0x7000763f70> as *const (), 0), // +0x44f4f0 sub_44f4f0
    Lifted::new(0x7000b3b520, tpl_884::<0x7000763f70> as *const (), 0), // +0x44f520 sub_44f520
    Lifted::new(0x7000b3b550, fn_7000b3b550 as *const (), 1), // +0x44f550 sub_44f550
    Lifted::new(0x7000b3b570, fn_7000b3b570 as *const (), 0), // +0x44f570 sub_44f570
    Lifted::new(0x7000b3b5a0, fn_7000b3b5a0 as *const (), 1), // +0x44f5a0 sub_44f5a0
    Lifted::new(0x7000b3b600, fn_7000b3b600 as *const (), 0), // +0x44f600 sub_44f600
    Lifted::new(0x7000b3b680, fn_7000b3b680 as *const (), 1), // +0x44f680 sub_44f680
    Lifted::new(0x7000b3b6b0, tpl_15 as *const (), 1), // +0x44f6b0 sub_44f6b0
    Lifted::new(0x7000b3b6c0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x44f6c0 sub_44f6c0
    Lifted::new(0x7000b3b710, tpl_33::<0x700177c312> as *const (), 0), // +0x44f710 sub_44f710
    Lifted::new(0x7000b3b740, fn_7000b3b740 as *const (), 2), // +0x44f740 sub_44f740
    Lifted::new(0x7000b3b990, fn_7000b3b990 as *const (), 0), // +0x44f990 sub_44f990
    Lifted::new(0x7000b3bd30, fn_7000b3bd30 as *const (), 0), // +0x44fd30 sub_44fd30
    Lifted::new(0x7000b3be90, tpl_39::<0x20> as *const (), 1), // +0x44fe90 sub_44fe90
    Lifted::new(0x7000b3bea0, fn_7000b3bea0 as *const (), 3), // +0x44fea0 sub_44fea0
    Lifted::new(0x7000b3c020, fn_7000b3c020 as *const (), 0), // +0x450020 sub_450020
    Lifted::new(0x7000b3c090, fn_7000b3c090 as *const (), 0), // +0x450090 sub_450090
    Lifted::new(0x7000b3c1f0, fn_7000b3c1f0 as *const (), 3), // +0x4501f0 sub_4501f0
    Lifted::new(0x7000b3c570, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000b3c581> as *const (), 0), // +0x450570 sub_450570
    Lifted::new(0x7000b3c5d0, fn_7000b3c5d0 as *const (), 1), // +0x4505d0 sub_4505d0
    Lifted::new(0x7000b3c820, fn_7000b3c820 as *const (), 0), // +0x450820 sub_450820
    Lifted::new(0x7000b3c980, fn_7000b3c980 as *const (), 0), // +0x450980 sub_450980
    Lifted::new(0x7000b3c9f0, fn_7000b3c9f0 as *const (), 2), // +0x4509f0 sub_4509f0
    Lifted::new(0x7000b3cb00, fn_7000b3cb00 as *const (), 0), // +0x450b00 sub_450b00
    Lifted::new(0x7000b3cd00, fn_7000b3cd00 as *const (), 0), // +0x450d00 sub_450d00
    Lifted::new(0x7000b3cf00, tpl_726::<0x10, 0x18, 0x3d8, 0x700075f630, 0x70009fc7f0> as *const (), 0), // +0x450f00 sub_450f00
    Lifted::new(0x7000b3cfa0, fn_7000b3cfa0 as *const (), 0), // +0x450fa0 sub_450fa0
    Lifted::new(0x7000b3d050, fn_7000b3d050 as *const (), 1), // +0x451050 sub_451050
    Lifted::new(0x7000b3d080, fn_7000b3d080 as *const (), 0), // +0x451080 sub_451080
    Lifted::new(0x7000b3d0f0, fn_7000b3d0f0 as *const (), 2), // +0x4510f0 sub_4510f0
    Lifted::new(0x7000b3d160, fn_7000b3d160 as *const (), 0), // +0x451160 sub_451160
    Lifted::new(0x7000b3d1e0, fn_7000b3d1e0 as *const (), 2), // +0x4511e0 sub_4511e0
    Lifted::new(0x7000b3d290, fn_7000b3d290 as *const (), 2), // +0x451290 sub_451290
    Lifted::new(0x7000b3d330, fn_7000b3d330 as *const (), 3), // +0x451330 sub_451330
    Lifted::new(0x7000b3d530, fn_7000b3d530 as *const (), 0), // +0x451530 sub_451530
    Lifted::new(0x7000b3d570, fn_7000b3d570 as *const (), 0), // +0x451570 sub_451570
    Lifted::new(0x7000b3d630, tpl_876::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b3d831, 0x7000b3e800, 0x7000b3d837, 0x70007f0910, 0x7000b3d83d> as *const (), 2), // +0x451630 sub_451630
    Lifted::new(0x7000b3d8d0, fn_7000b3d8d0 as *const (), 2), // +0x4518d0 sub_4518d0
    Lifted::new(0x7000b3dab0, fn_7000b3dab0 as *const (), 0), // +0x451ab0 sub_451ab0
    Lifted::new(0x7000b3dc20, fn_7000b3dc20 as *const (), 0), // +0x451c20 sub_451c20
    Lifted::new(0x7000b3dde0, fn_7000b3dde0 as *const (), 0), // +0x451de0 sub_451de0
    Lifted::new(0x7000b3dee0, fn_7000b3dee0 as *const (), 2), // +0x451ee0 sub_451ee0
    Lifted::new(0x7000b3dff0, fn_7000b3dff0 as *const (), 2), // +0x451ff0 sub_451ff0
    Lifted::new(0x7000b3e2f0, fn_7000b3e2f0 as *const (), 2), // +0x4522f0 sub_4522f0
    Lifted::new(0x7000b3e4f0, fn_7000b3e4f0 as *const (), 2), // +0x4524f0 sub_4524f0
    Lifted::new(0x7000b3e6e0, fn_7000b3e6e0 as *const (), 0), // +0x4526e0 sub_4526e0
    Lifted::new(0x7000b3e800, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b3e810> as *const (), 0), // +0x452800 sub_452800
    Lifted::new(0x7000b3e830, fn_7000b3e830 as *const (), 0), // +0x452830 sub_452830
    Lifted::new(0x7000b3e870, fn_7000b3e870 as *const (), 0), // +0x452870 sub_452870
    Lifted::new(0x7000b3e8a0, fn_7000b3e8a0 as *const (), 2), // +0x4528a0 sub_4528a0
    Lifted::new(0x7000b3e970, fn_7000b3e970 as *const (), 0), // +0x452970 sub_452970
    Lifted::new(0x7000b3e9e0, fn_7000b3e9e0 as *const (), 0), // +0x4529e0 sub_4529e0
    Lifted::new(0x7000b3eaa0, fn_7000b3eaa0 as *const (), 0), // +0x452aa0 sub_452aa0
    Lifted::new(0x7000b3eb40, fn_7000b3eb40 as *const (), 2), // +0x452b40 sub_452b40
    Lifted::new(0x7000b3ecb0, fn_7000b3ecb0 as *const (), 0), // +0x452cb0 sub_452cb0
    Lifted::new(0x7000b3ed60, fn_7000b3ed60 as *const (), 0), // +0x452d60 sub_452d60
    Lifted::new(0x7000b3edf0, fn_7000b3edf0 as *const (), 2), // +0x452df0 sub_452df0
    Lifted::new(0x7000b3ee50, fn_7000b3ee50 as *const (), 0), // +0x452e50 sub_452e50
    Lifted::new(0x7000b3eeb0, fn_7000b3eeb0 as *const (), 2), // +0x452eb0 sub_452eb0
    Lifted::new(0x7000b3efb0, tpl_51::<0x8> as *const (), 1), // +0x452fb0 sub_452fb0
    Lifted::new(0x7000b3efc0, tpl_244::<0x8> as *const (), 1), // +0x452fc0 sub_452fc0
    Lifted::new(0x7000b3efd0, fn_7000b3efd0 as *const (), 0), // +0x452fd0 sub_452fd0
    Lifted::new(0x7000b3f020, fn_7000b3f020 as *const (), 0), // +0x453020 sub_453020
    Lifted::new(0x7000b3f150, tpl_605::<0xfffffffffffffff8, 0x7000718060, 0x700177c312, 0x7000701460> as *const (), 0), // +0x453150 sub_453150
    Lifted::new(0x7000b3f220, tpl_133::<0x7000718060, 0x7000701460> as *const (), 0), // +0x453220 sub_453220
    Lifted::new(0x7000b3f260, fn_7000b3f260 as *const (), 0), // +0x453260 sub_453260
    Lifted::new(0x7000b3f300, fn_7000b3f300 as *const (), 0), // +0x453300 sub_453300
    Lifted::new(0x7000b3f490, fn_7000b3f490 as *const (), 0), // +0x453490 sub_453490
    Lifted::new(0x7000b3f590, fn_7000b3f590 as *const (), 1), // +0x453590 sub_453590
    Lifted::new(0x7000b3f5b0, tpl_161::<0x63f0> as *const (), 1), // +0x4535b0 sub_4535b0
    Lifted::new(0x7000b3f5c0, fn_7000b3f5c0 as *const (), 1), // +0x4535c0 sub_4535c0
    Lifted::new(0x7000b3f5d0, fn_7000b3f5d0 as *const (), 0), // +0x4535d0 sub_4535d0
    Lifted::new(0x7000b3f650, fn_7000b3f650 as *const (), 1), // +0x453650 sub_453650
    Lifted::new(0x7000b3f660, fn_7000b3f660 as *const (), 2), // +0x453660 sub_453660
    Lifted::new(0x7000b3f6e0, fn_7000b3f6e0 as *const (), 0), // +0x4536e0 sub_4536e0
    Lifted::new(0x7000b3f720, tpl_533::<0x10, 0x28, 0x84> as *const (), 1), // +0x453720 sub_453720
    Lifted::new(0x7000b3f740, fn_7000b3f740 as *const (), 1), // +0x453740 sub_453740
    Lifted::new(0x7000b3f760, tpl_275::<0x10, 0x28> as *const (), 1), // +0x453760 sub_453760
    Lifted::new(0x7000b3f770, fn_7000b3f770 as *const (), 1), // +0x453770 sub_453770
    Lifted::new(0x7000b3f7b0, fn_7000b3f7b0 as *const (), 1), // +0x4537b0 sub_4537b0
    Lifted::new(0x7000b3f7f0, fn_7000b3f7f0 as *const (), 1), // +0x4537f0 sub_4537f0
    Lifted::new(0x7000b3f830, fn_7000b3f830 as *const (), 1), // +0x453830 sub_453830
    Lifted::new(0x7000b3f870, fn_7000b3f870 as *const (), 1), // +0x453870 sub_453870
    Lifted::new(0x7000b3f8b0, fn_7000b3f8b0 as *const (), 1), // +0x4538b0 sub_4538b0
    Lifted::new(0x7000b3f8f0, fn_7000b3f8f0 as *const (), 1), // +0x4538f0 sub_4538f0
    Lifted::new(0x7000b3f930, fn_7000b3f930 as *const (), 1), // +0x453930 sub_453930
    Lifted::new(0x7000b3f970, fn_7000b3f970 as *const (), 1), // +0x453970 sub_453970
    Lifted::new(0x7000b3f9b0, fn_7000b3f9b0 as *const (), 1), // +0x4539b0 sub_4539b0
    Lifted::new(0x7000b3f9f0, fn_7000b3f9f0 as *const (), 1), // +0x4539f0 sub_4539f0
    Lifted::new(0x7000b3fa30, fn_7000b3fa30 as *const (), 1), // +0x453a30 sub_453a30
    Lifted::new(0x7000b3fa70, fn_7000b3fa70 as *const (), 1), // +0x453a70 sub_453a70
    Lifted::new(0x7000b3fab0, tpl_401::<0xc975, 0x8> as *const (), 1), // +0x453ab0 sub_453ab0
    Lifted::new(0x7000b3fad0, tpl_401::<0xc974, 0x8> as *const (), 1), // +0x453ad0 sub_453ad0
    Lifted::new(0x7000b3faf0, tpl_401::<0xc976, 0x8> as *const (), 1), // +0x453af0 sub_453af0
    Lifted::new(0x7000b3fb10, tpl_401::<0xc971, 0x8> as *const (), 1), // +0x453b10 sub_453b10
    Lifted::new(0x7000b3fb30, tpl_885::<0xc975, 0x8> as *const (), 1), // +0x453b30 sub_453b30
    Lifted::new(0x7000b3fb50, tpl_885::<0xc974, 0x8> as *const (), 1), // +0x453b50 sub_453b50
    Lifted::new(0x7000b3fb70, tpl_885::<0xc976, 0x8> as *const (), 1), // +0x453b70 sub_453b70
    Lifted::new(0x7000b3fb90, tpl_885::<0xc971, 0x8> as *const (), 1), // +0x453b90 sub_453b90
    Lifted::new(0x7000b3fbb0, tpl_885::<0xc973, 0x8> as *const (), 1), // +0x453bb0 sub_453bb0
    Lifted::new(0x7000b3fbd0, tpl_401::<0xc973, 0x8> as *const (), 1), // +0x453bd0 sub_453bd0
    Lifted::new(0x7000b3fbf0, tpl_401::<0xc972, 0x8> as *const (), 1), // +0x453bf0 sub_453bf0
    Lifted::new(0x7000b3fc10, tpl_885::<0xc972, 0x8> as *const (), 1), // +0x453c10 sub_453c10
    Lifted::new(0x7000b3fc30, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453c30 sub_453c30
    Lifted::new(0x7000b3fc70, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453c70 sub_453c70
    Lifted::new(0x7000b3fcb0, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453cb0 sub_453cb0
    Lifted::new(0x7000b3fcf0, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453cf0 sub_453cf0
    Lifted::new(0x7000b3fd30, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453d30 sub_453d30
    Lifted::new(0x7000b3fd70, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453d70 sub_453d70
    Lifted::new(0x7000b3fdb0, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453db0 sub_453db0
    Lifted::new(0x7000b3fdf0, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453df0 sub_453df0
    Lifted::new(0x7000b3fe30, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453e30 sub_453e30
    Lifted::new(0x7000b3fe70, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453e70 sub_453e70
    Lifted::new(0x7000b3feb0, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453eb0 sub_453eb0
    Lifted::new(0x7000b3fef0, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453ef0 sub_453ef0
    Lifted::new(0x7000b3ff30, tpl_886::<0x7001c8e33c, 0x4> as *const (), 1), // +0x453f30 sub_453f30
    Lifted::new(0x7000b3ff70, tpl_412::<0x8, 0xc975> as *const (), 1), // +0x453f70 sub_453f70
    Lifted::new(0x7000b3ff90, tpl_412::<0x8, 0xc974> as *const (), 1), // +0x453f90 sub_453f90
    Lifted::new(0x7000b3ffb0, tpl_412::<0x8, 0xc976> as *const (), 1), // +0x453fb0 sub_453fb0
    Lifted::new(0x7000b3ffd0, tpl_412::<0x8, 0xc971> as *const (), 1), // +0x453fd0 sub_453fd0
    Lifted::new(0x7000b3fff0, tpl_412::<0x8, 0xc975> as *const (), 1), // +0x453ff0 sub_453ff0
    Lifted::new(0x7000b40010, tpl_412::<0x8, 0xc974> as *const (), 1), // +0x454010 sub_454010
    Lifted::new(0x7000b40030, tpl_412::<0x8, 0xc976> as *const (), 1), // +0x454030 sub_454030
    Lifted::new(0x7000b40050, tpl_412::<0x8, 0xc971> as *const (), 1), // +0x454050 sub_454050
    Lifted::new(0x7000b40070, tpl_412::<0x8, 0xc973> as *const (), 1), // +0x454070 sub_454070
    Lifted::new(0x7000b40090, tpl_412::<0x8, 0xc973> as *const (), 1), // +0x454090 sub_454090
    Lifted::new(0x7000b400b0, tpl_412::<0x8, 0xc972> as *const (), 1), // +0x4540b0 sub_4540b0
    Lifted::new(0x7000b400d0, tpl_412::<0x8, 0xc972> as *const (), 1), // +0x4540d0 sub_4540d0
    Lifted::new(0x7000b400f0, tpl_49::<0xc975> as *const (), 1), // +0x4540f0 sub_4540f0
    Lifted::new(0x7000b40100, tpl_49::<0xc974> as *const (), 1), // +0x454100 sub_454100
    Lifted::new(0x7000b40110, tpl_49::<0xc976> as *const (), 1), // +0x454110 sub_454110
    Lifted::new(0x7000b40120, tpl_49::<0xc971> as *const (), 1), // +0x454120 sub_454120
    Lifted::new(0x7000b40130, tpl_49::<0xc975> as *const (), 1), // +0x454130 sub_454130
    Lifted::new(0x7000b40140, tpl_49::<0xc974> as *const (), 1), // +0x454140 sub_454140
    Lifted::new(0x7000b40150, tpl_49::<0xc976> as *const (), 1), // +0x454150 sub_454150
    Lifted::new(0x7000b40160, tpl_49::<0xc971> as *const (), 1), // +0x454160 sub_454160
    Lifted::new(0x7000b40170, tpl_49::<0xc973> as *const (), 1), // +0x454170 sub_454170
    Lifted::new(0x7000b40180, tpl_49::<0xc973> as *const (), 1), // +0x454180 sub_454180
    Lifted::new(0x7000b40190, tpl_49::<0xc972> as *const (), 1), // +0x454190 sub_454190
    Lifted::new(0x7000b401a0, tpl_49::<0xc972> as *const (), 1), // +0x4541a0 sub_4541a0
    Lifted::new(0x7000b401b0, tpl_887::<0x7001c8e33c> as *const (), 1), // +0x4541b0 sub_4541b0
    Lifted::new(0x7000b401e0, tpl_887::<0x7001c8e33c> as *const (), 1), // +0x4541e0 sub_4541e0
    Lifted::new(0x7000b40210, tpl_888::<0x7001c8e33c> as *const (), 1), // +0x454210 sub_454210
    Lifted::new(0x7000b40240, tpl_888::<0x7001c8e33c> as *const (), 1), // +0x454240 sub_454240
    Lifted::new(0x7000b40270, tpl_889::<0x7001c8e33c> as *const (), 1), // +0x454270 sub_454270
    Lifted::new(0x7000b402a0, tpl_889::<0x7001c8e33c> as *const (), 1), // +0x4542a0 sub_4542a0
    Lifted::new(0x7000b402d0, tpl_890::<0x7001c8e33c> as *const (), 1), // +0x4542d0 sub_4542d0
    Lifted::new(0x7000b40300, tpl_890::<0x7001c8e33c> as *const (), 1), // +0x454300 sub_454300
    Lifted::new(0x7000b40330, tpl_891::<0x7001c8e33c> as *const (), 1), // +0x454330 sub_454330
    Lifted::new(0x7000b40360, tpl_891::<0x7001c8e33c> as *const (), 1), // +0x454360 sub_454360
    Lifted::new(0x7000b40390, fn_7000b40390 as *const (), 1), // +0x454390 sub_454390
    Lifted::new(0x7000b403c0, fn_7000b403c0 as *const (), 1), // +0x4543c0 sub_4543c0
    Lifted::new(0x7000b403f0, fn_7000b403f0 as *const (), 1), // +0x4543f0 sub_4543f0
    Lifted::new(0x7000b40450, fn_7000b40450 as *const (), 1), // +0x454450 sub_454450
    Lifted::new(0x7000b404a0, fn_7000b404a0 as *const (), 1), // +0x4544a0 sub_4544a0
    Lifted::new(0x7000b404d0, tpl_47::<0xc972> as *const (), 1), // +0x4544d0 sub_4544d0
    Lifted::new(0x7000b404e0, tpl_892::<0xc975> as *const (), 1), // +0x4544e0 sub_4544e0
    Lifted::new(0x7000b40500, tpl_892::<0xc974> as *const (), 1), // +0x454500 sub_454500
    Lifted::new(0x7000b40520, tpl_892::<0xc976> as *const (), 1), // +0x454520 sub_454520
    Lifted::new(0x7000b40540, tpl_892::<0xc971> as *const (), 1), // +0x454540 sub_454540
    Lifted::new(0x7000b40560, tpl_893::<0xc975> as *const (), 1), // +0x454560 sub_454560
    Lifted::new(0x7000b40580, tpl_893::<0xc974> as *const (), 1), // +0x454580 sub_454580
    Lifted::new(0x7000b405a0, tpl_893::<0xc976> as *const (), 1), // +0x4545a0 sub_4545a0
    Lifted::new(0x7000b405c0, tpl_893::<0xc971> as *const (), 1), // +0x4545c0 sub_4545c0
    Lifted::new(0x7000b405e0, tpl_893::<0xc973> as *const (), 1), // +0x4545e0 sub_4545e0
    Lifted::new(0x7000b40600, tpl_892::<0xc973> as *const (), 1), // +0x454600 sub_454600
    Lifted::new(0x7000b40620, tpl_892::<0xc972> as *const (), 1), // +0x454620 sub_454620
    Lifted::new(0x7000b40640, tpl_893::<0xc972> as *const (), 1), // +0x454640 sub_454640
    Lifted::new(0x7000b40660, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40680, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454660 sub_454660
    Lifted::new(0x7000b406d0, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b406f0, 0x7001c8e33c, 0x4> as *const (), 0), // +0x4546d0 sub_4546d0
    Lifted::new(0x7000b40740, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40760, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454740 sub_454740
    Lifted::new(0x7000b407b0, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b407d0, 0x7001c8e33c, 0x4> as *const (), 0), // +0x4547b0 sub_4547b0
    Lifted::new(0x7000b40820, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40840, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454820 sub_454820
    Lifted::new(0x7000b40890, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b408b0, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454890 sub_454890
    Lifted::new(0x7000b40900, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40920, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454900 sub_454900
    Lifted::new(0x7000b40970, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40990, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454970 sub_454970
    Lifted::new(0x7000b409e0, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40a00, 0x7001c8e33c, 0x4> as *const (), 0), // +0x4549e0 sub_4549e0
    Lifted::new(0x7000b40a50, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40a70, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454a50 sub_454a50
    Lifted::new(0x7000b40ac0, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40ae0, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454ac0 sub_454ac0
    Lifted::new(0x7000b40b30, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40b50, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454b30 sub_454b30
    Lifted::new(0x7000b40ba0, tpl_894::<0x70017fd770, 0x70017a0290, 0x7001d54ef8, 0x7000b40bc0, 0x7001c8e33c, 0x4> as *const (), 0), // +0x454ba0 sub_454ba0
    Lifted::new(0x7000b40c10, fn_7000b40c10 as *const (), 0), // +0x454c10 sub_454c10
    Lifted::new(0x7000b40de0, tpl_895::<0x7001779e20, 0x7000b44ab0, 0x70017d37c0, 0x70017790e0, 0x7000b4104f> as *const (), 2), // +0x454de0 sub_454de0
    Lifted::new(0x7000b410f0, tpl_895::<0x7001779e20, 0x7000b44ab0, 0x70017d37c0, 0x70017790e0, 0x7000b4135f> as *const (), 2), // +0x4550f0 sub_4550f0
    Lifted::new(0x7000b41400, tpl_68 as *const (), 1), // +0x455400 sub_455400
    Lifted::new(0x7000b414b0, tpl_896::<0x1, 0x30, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x18, 0x10> as *const (), 1), // +0x4554b0 sub_4554b0
    Lifted::new(0x7000b415c0, fn_7000b415c0 as *const (), 0), // +0x4555c0 sub_4555c0
    Lifted::new(0x7000b41660, tpl_80::<0x8, 0x10, 0x18, 0x7001779e20, 0x30, 0x38, 0x7000b43dc0> as *const (), 0), // +0x455660 sub_455660
    Lifted::new(0x7000b41710, fn_7000b41710 as *const (), 0), // +0x455710 sub_455710
    Lifted::new(0x7000b41900, tpl_792::<0x70017fd7a8, 0x70007844b0, 0x7000770500> as *const (), 0), // +0x455900 sub_455900
    Lifted::new(0x7000b41930, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000b419d3> as *const (), 0), // +0x455930 sub_455930
    Lifted::new(0x7000b41a00, tpl_408::<0x7000749860, 0x700177c312> as *const (), 0), // +0x455a00 sub_455a00
    Lifted::new(0x7000b41a40, fn_7000b41a40 as *const (), 0), // +0x455a40 sub_455a40
    Lifted::new(0x7000b41b90, tpl_793::<0x70017fd7a8, 0x70007844b0, 0x7000770500, 0x700177c312> as *const (), 0), // +0x455b90 sub_455b90
    Lifted::new(0x7000b41bf0, fn_7000b41bf0 as *const (), 0), // +0x455bf0 sub_455bf0
    Lifted::new(0x7000b41d40, fn_7000b41d40 as *const (), 0), // +0x455d40 sub_455d40
    Lifted::new(0x7000b41fa0, tpl_432::<0x7000701460> as *const (), 0), // +0x455fa0 sub_455fa0
    Lifted::new(0x7000b41fb0, fn_7000b41fb0 as *const (), 2), // +0x455fb0 sub_455fb0
    Lifted::new(0x7000b425a0, fn_7000b425a0 as *const (), 2), // +0x4565a0 sub_4565a0
    Lifted::new(0x7000b429f0, fn_7000b429f0 as *const (), 2), // +0x4569f0 sub_4569f0
    Lifted::new(0x7000b42d50, fn_7000b42d50 as *const (), 2), // +0x456d50 sub_456d50
    Lifted::new(0x7000b430d0, fn_7000b430d0 as *const (), 0), // +0x4570d0 sub_4570d0
    Lifted::new(0x7000b43170, fn_7000b43170 as *const (), 2), // +0x457170 sub_457170
    Lifted::new(0x7000b43310, tpl_483::<0x7001c72242> as *const (), 1), // +0x457310 sub_457310
    Lifted::new(0x7000b43320, fn_7000b43320 as *const (), 0), // +0x457320 sub_457320
    Lifted::new(0x7000b433d0, fn_7000b433d0 as *const (), 1), // +0x4573d0 sub_4573d0
    Lifted::new(0x7000b43420, tpl_794::<0x70006f4fa0> as *const (), 0), // +0x457420 sub_457420
    Lifted::new(0x7000b43430, fn_7000b43430 as *const (), 0), // +0x457430 sub_457430
    Lifted::new(0x7000b43570, fn_7000b43570 as *const (), 0), // +0x457570 sub_457570
    Lifted::new(0x7000b43690, fn_7000b43690 as *const (), 0), // +0x457690 sub_457690
    Lifted::new(0x7000b43770, fn_7000b43770 as *const (), 2), // +0x457770 sub_457770
    Lifted::new(0x7000b43ab0, fn_7000b43ab0 as *const (), 2), // +0x457ab0 sub_457ab0
    Lifted::new(0x7000b43dc0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b43ec7, 0x7000b41400, 0x70007f0910, 0x7000b43ee3> as *const (), 0), // +0x457dc0 sub_457dc0
    Lifted::new(0x7000b43f30, tpl_753::<0x70017fdb00, 0x8> as *const (), 1), // +0x457f30 sub_457f30
    Lifted::new(0x7000b43f50, tpl_753::<0x70017fdb40, 0x8> as *const (), 1), // +0x457f50 sub_457f50
    Lifted::new(0x7000b43f70, tpl_754::<0x700177c312> as *const (), 0), // +0x457f70 sub_457f70
    Lifted::new(0x7000b43f90, tpl_754::<0x700177c312> as *const (), 0), // +0x457f90 sub_457f90
    Lifted::new(0x7000b43fb0, fn_7000b43fb0 as *const (), 2), // +0x457fb0 sub_457fb0
    Lifted::new(0x7000b43fc0, tpl_143::<0x8, 0x700070da10> as *const (), 0), // +0x457fc0 sub_457fc0
    Lifted::new(0x7000b43fd0, fn_7000b43fd0 as *const (), 0), // +0x457fd0 sub_457fd0
    Lifted::new(0x7000b44450, tpl_286::<0x7000b43dc0, 0x70017d37a0, 0x70017790e0, 0x7000b4463b> as *const (), 0), // +0x458450 sub_458450
    Lifted::new(0x7000b446c0, tpl_52::<0x8> as *const (), 1), // +0x4586c0 sub_4586c0
    Lifted::new(0x7000b446d0, tpl_52::<0x8> as *const (), 1), // +0x4586d0 sub_4586d0
    Lifted::new(0x7000b446e0, tpl_287 as *const (), 1), // +0x4586e0 sub_4586e0
    Lifted::new(0x7000b44a00, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x458a00 sub_458a00
    Lifted::new(0x7000b44a70, tpl_753::<0x70017fdb00, 0x8> as *const (), 1), // +0x458a70 sub_458a70
    Lifted::new(0x7000b44a90, tpl_753::<0x70017fdb40, 0x8> as *const (), 1), // +0x458a90 sub_458a90
    Lifted::new(0x7000b44ab0, tpl_897::<0x10, 0x38, 0x700177c170, 0x70017d37ec, 0x7000b44450> as *const (), 2), // +0x458ab0 sub_458ab0
    Lifted::new(0x7000b44b90, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x458b90 sub_458b90
    Lifted::new(0x7000b44c00, tpl_315::<0x7001c6b2d0> as *const (), 1), // +0x458c00 sub_458c00
    Lifted::new(0x7000b44c10, tpl_315::<0x7001c6b280> as *const (), 1), // +0x458c10 sub_458c10
    Lifted::new(0x7000b44c20, fn_7000b44c20 as *const (), 0), // +0x458c20 sub_458c20
    Lifted::new(0x7000b44db0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x458db0 sub_458db0
    Lifted::new(0x7000b44df0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x458df0 sub_458df0
    Lifted::new(0x7000b44e30, tpl_392::<0x700177c0e0> as *const (), 0), // +0x458e30 sub_458e30
    Lifted::new(0x7000b44e60, fn_7000b44e60 as *const (), 0), // +0x458e60 sub_458e60
    Lifted::new(0x7000b44f20, tpl_55::<0x700177c0d0> as *const (), 0), // +0x458f20 sub_458f20
    Lifted::new(0x7000b44f60, tpl_603::<0x20, 0xffffffffffffffff, 0x18, 0x10, 0x8, 0x700177c312, 0xfffffffffffffff8, 0x7001d54968, 0x7000b45023> as *const (), 0), // +0x458f60 sub_458f60
    Lifted::new(0x7000b45070, fn_7000b45070 as *const (), 0), // +0x459070 sub_459070
    Lifted::new(0x7000b450d0, tpl_899::<0x7000b450e0> as *const (), 0), // +0x4590d0 sub_4590d0
    Lifted::new(0x7000b450e0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x4590e0 sub_4590e0
    Lifted::new(0x7000b45120, fn_7000b45120 as *const (), 0), // +0x459120 sub_459120
    Lifted::new(0x7000b451a0, fn_7000b451a0 as *const (), 0), // +0x4591a0 sub_4591a0
    Lifted::new(0x7000b45300, fn_7000b45300 as *const (), 0), // +0x459300 sub_459300
    Lifted::new(0x7000b454d0, fn_7000b454d0 as *const (), 0), // +0x4594d0 sub_4594d0
    Lifted::new(0x7000b455a0, fn_7000b455a0 as *const (), 2), // +0x4595a0 sub_4595a0
    Lifted::new(0x7000b45830, fn_7000b45830 as *const (), 0), // +0x459830 sub_459830
    Lifted::new(0x7000b45900, fn_7000b45900 as *const (), 0), // +0x459900 sub_459900
    Lifted::new(0x7000b459c0, fn_7000b459c0 as *const (), 1), // +0x4599c0 sub_4599c0
    Lifted::new(0x7000b459e0, fn_7000b459e0 as *const (), 0), // +0x4599e0 sub_4599e0
    Lifted::new(0x7000b45c40, fn_7000b45c40 as *const (), 0), // +0x459c40 sub_459c40
    Lifted::new(0x7000b45cd0, fn_7000b45cd0 as *const (), 0), // +0x459cd0 sub_459cd0
    Lifted::new(0x7000b45e00, fn_7000b45e00 as *const (), 2), // +0x459e00 sub_459e00
    Lifted::new(0x7000b461a0, fn_7000b461a0 as *const (), 0), // +0x45a1a0 sub_45a1a0
    Lifted::new(0x7000b462b0, fn_7000b462b0 as *const (), 0), // +0x45a2b0 sub_45a2b0
    Lifted::new(0x7000b463d0, tpl_900::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000b46598, 0x70007f0910, 0x7000b4659e, 0x7000b46620, 0x7000b465a7> as *const (), 0), // +0x45a3d0 sub_45a3d0
    Lifted::new(0x7000b46620, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000b46630> as *const (), 0), // +0x45a620 sub_45a620
    Lifted::new(0x7000b46640, fn_7000b46640 as *const (), 0), // +0x45a640 sub_45a640
    Lifted::new(0x7000b46690, fn_7000b46690 as *const (), 0), // +0x45a690 sub_45a690
    Lifted::new(0x7000b46810, fn_7000b46810 as *const (), 1), // +0x45a810 sub_45a810
    Lifted::new(0x7000b46830, fn_7000b46830 as *const (), 0), // +0x45a830 sub_45a830
    Lifted::new(0x7000b46f70, fn_7000b46f70 as *const (), 1), // +0x45af70 sub_45af70
    Lifted::new(0x7000b470c0, fn_7000b470c0 as *const (), 0), // +0x45b0c0 sub_45b0c0
    Lifted::new(0x7000b47190, fn_7000b47190 as *const (), 2), // +0x45b190 sub_45b190
    Lifted::new(0x7000b47270, fn_7000b47270 as *const (), 1), // +0x45b270 sub_45b270
    Lifted::new(0x7000b47600, fn_7000b47600 as *const (), 0), // +0x45b600 sub_45b600
    Lifted::new(0x7000b47890, fn_7000b47890 as *const (), 0), // +0x45b890 sub_45b890
    Lifted::new(0x7000b47d20, fn_7000b47d20 as *const (), 2), // +0x45bd20 sub_45bd20
    Lifted::new(0x7000b47de0, fn_7000b47de0 as *const (), 1), // +0x45bde0 sub_45bde0
    Lifted::new(0x7000b48030, fn_7000b48030 as *const (), 1), // +0x45c030 sub_45c030
    Lifted::new(0x7000b480d0, fn_7000b480d0 as *const (), 1), // +0x45c0d0 sub_45c0d0
    Lifted::new(0x7000b48130, fn_7000b48130 as *const (), 3), // +0x45c130 sub_45c130
    Lifted::new(0x7000b48290, fn_7000b48290 as *const (), 3), // +0x45c290 sub_45c290
    Lifted::new(0x7000b483a0, fn_7000b483a0 as *const (), 3), // +0x45c3a0 sub_45c3a0
    Lifted::new(0x7000b48430, fn_7000b48430 as *const (), 0), // +0x45c430 sub_45c430
    Lifted::new(0x7000b48510, fn_7000b48510 as *const (), 0), // +0x45c510 sub_45c510
    Lifted::new(0x7000b48670, tpl_901::<0x7000b4f070, 0x7000b4ed80, 0x7000b4ef90, 0x7000b48823> as *const (), 2), // +0x45c670 sub_45c670
    Lifted::new(0x7000b488a0, fn_7000b488a0 as *const (), 0), // +0x45c8a0 sub_45c8a0
    Lifted::new(0x7000b48b40, fn_7000b48b40 as *const (), 0), // +0x45cb40 sub_45cb40
    Lifted::new(0x7000b48de0, tpl_901::<0x7000b4f070, 0x7000b4ed80, 0x7000b4ef90, 0x7000b48f93> as *const (), 2), // +0x45cde0 sub_45cde0
    Lifted::new(0x7000b49010, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b491e4, 0x7000b4f010, 0x7000b491ea, 0x70007f0910, 0x7000b491f0> as *const (), 0), // +0x45d010 sub_45d010
    Lifted::new(0x7000b49270, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b49444, 0x7000b4f030, 0x7000b4944a, 0x70007f0910, 0x7000b49450> as *const (), 0), // +0x45d270 sub_45d270
    Lifted::new(0x7000b494d0, fn_7000b494d0 as *const (), 2), // +0x45d4d0 sub_45d4d0
    Lifted::new(0x7000b49850, fn_7000b49850 as *const (), 2), // +0x45d850 sub_45d850
    Lifted::new(0x7000b49a80, fn_7000b49a80 as *const (), 3), // +0x45da80 sub_45da80
    Lifted::new(0x7000b49d80, fn_7000b49d80 as *const (), 2), // +0x45dd80 sub_45dd80
    Lifted::new(0x7000b49e60, fn_7000b49e60 as *const (), 3), // +0x45de60 sub_45de60
    Lifted::new(0x7000b4a060, fn_7000b4a060 as *const (), 2), // +0x45e060 sub_45e060
    Lifted::new(0x7000b4a430, fn_7000b4a430 as *const (), 2), // +0x45e430 sub_45e430
    Lifted::new(0x7000b4aba0, fn_7000b4aba0 as *const (), 2), // +0x45eba0 sub_45eba0
    Lifted::new(0x7000b4ad40, fn_7000b4ad40 as *const (), 3), // +0x45ed40 sub_45ed40
    Lifted::new(0x7000b4afa0, fn_7000b4afa0 as *const (), 2), // +0x45efa0 sub_45efa0
    Lifted::new(0x7000b4b4b0, fn_7000b4b4b0 as *const (), 2), // +0x45f4b0 sub_45f4b0
    Lifted::new(0x7000b4b6f0, fn_7000b4b6f0 as *const (), 1), // +0x45f6f0 sub_45f6f0
    Lifted::new(0x7000b4b7b0, fn_7000b4b7b0 as *const (), 2), // +0x45f7b0 sub_45f7b0
    Lifted::new(0x7000b4b8d0, tpl_562::<0x8, 0x10, 0x7000b4df60> as *const (), 0), // +0x45f8d0 sub_45f8d0
    Lifted::new(0x7000b4b920, fn_7000b4b920 as *const (), 0), // +0x45f920 sub_45f920
    Lifted::new(0x7000b4ba00, tpl_902::<0x8, 0x7000b4b920, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000b4ba8f> as *const (), 0), // +0x45fa00 sub_45fa00
    Lifted::new(0x7000b4bac0, fn_7000b4bac0 as *const (), 0), // +0x45fac0 sub_45fac0
    Lifted::new(0x7000b4bc00, fn_7000b4bc00 as *const (), 2), // +0x45fc00 sub_45fc00
    Lifted::new(0x7000b4ca00, fn_7000b4ca00 as *const (), 0), // +0x460a00 sub_460a00
    Lifted::new(0x7000b4cfc0, fn_7000b4cfc0 as *const (), 0), // +0x460fc0 sub_460fc0
    Lifted::new(0x7000b4d030, fn_7000b4d030 as *const (), 2), // +0x461030 sub_461030
    Lifted::new(0x7000b4dab0, fn_7000b4dab0 as *const (), 0), // +0x461ab0 sub_461ab0
    Lifted::new(0x7000b4db80, fn_7000b4db80 as *const (), 1), // +0x461b80 sub_461b80
    Lifted::new(0x7000b4dba0, fn_7000b4dba0 as *const (), 0), // +0x461ba0 sub_461ba0
    Lifted::new(0x7000b4dd60, fn_7000b4dd60 as *const (), 2), // +0x461d60 sub_461d60
    Lifted::new(0x7000b4df60, tpl_809::<0x7001d55060, 0x8, 0x70017d8de0, 0x7001d54ef8, 0x7000b4df93, 0x10, 0x18, 0xfffffffffffffff8, 0x14> as *const (), 0), // +0x461f60 sub_461f60
    Lifted::new(0x7000b4e060, fn_7000b4e060 as *const (), 2), // +0x462060 sub_462060
    Lifted::new(0x7000b4e1f0, fn_7000b4e1f0 as *const (), 2), // +0x4621f0 sub_4621f0
    Lifted::new(0x7000b4e620, fn_7000b4e620 as *const (), 0), // +0x462620 sub_462620
    Lifted::new(0x7000b4e720, fn_7000b4e720 as *const (), 2), // +0x462720 sub_462720
    Lifted::new(0x7000b4e8d0, fn_7000b4e8d0 as *const (), 2), // +0x4628d0 sub_4628d0
    Lifted::new(0x7000b4ed80, fn_7000b4ed80 as *const (), 0), // +0x462d80 sub_462d80
    Lifted::new(0x7000b4ee60, tpl_407::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b4eed9> as *const (), 0), // +0x462e60 sub_462e60
    Lifted::new(0x7000b4ef00, tpl_903::<0x10, 0xfffffffffffffff0, 0x20, 0xffffffffffffffd8, 0xffffffffffffffe8, 0xffffffffffffffe0> as *const (), 1), // +0x462f00 sub_462f00
    Lifted::new(0x7000b4ef90, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b4efa0> as *const (), 0), // +0x462f90 sub_462f90
    Lifted::new(0x7000b4efb0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b4efc0> as *const (), 0), // +0x462fb0 sub_462fb0
    Lifted::new(0x7000b4efd0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b4efe0> as *const (), 0), // +0x462fd0 sub_462fd0
    Lifted::new(0x7000b4eff0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b4f000> as *const (), 0), // +0x462ff0 sub_462ff0
    Lifted::new(0x7000b4f010, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b4f020> as *const (), 0), // +0x463010 sub_463010
    Lifted::new(0x7000b4f030, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b4f040> as *const (), 0), // +0x463030 sub_463030
    Lifted::new(0x7000b4f050, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000b4f060> as *const (), 0), // +0x463050 sub_463050
    Lifted::new(0x7000b4f070, tpl_904::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b4f0c8, 0x70007f0910, 0x7000b4f0e3> as *const (), 0), // +0x463070 sub_463070
    Lifted::new(0x7000b4f110, fn_7000b4f110 as *const (), 1), // +0x463110 sub_463110
    Lifted::new(0x7000b4f1c0, fn_7000b4f1c0 as *const (), 1), // +0x4631c0 sub_4631c0
    Lifted::new(0x7000b4f270, tpl_389::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b4f4ef, 0x7000b51b50, 0x7000b4f4f5, 0x70007f0910, 0x7000b4f4fb> as *const (), 2), // +0x463270 sub_463270
    Lifted::new(0x7000b4f5a0, fn_7000b4f5a0 as *const (), 2), // +0x4635a0 sub_4635a0
    Lifted::new(0x7000b4f940, tpl_100::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b4f9ba> as *const (), 0), // +0x463940 sub_463940
    Lifted::new(0x7000b4f9e0, fn_7000b4f9e0 as *const (), 0), // +0x4639e0 sub_4639e0
    Lifted::new(0x7000b4fb50, fn_7000b4fb50 as *const (), 0), // +0x463b50 sub_463b50
    Lifted::new(0x7000b50140, fn_7000b50140 as *const (), 0), // +0x464140 sub_464140
    Lifted::new(0x7000b501a0, fn_7000b501a0 as *const (), 0), // +0x4641a0 sub_4641a0
    Lifted::new(0x7000b502a0, fn_7000b502a0 as *const (), 0), // +0x4642a0 sub_4642a0
    Lifted::new(0x7000b508b0, fn_7000b508b0 as *const (), 0), // +0x4648b0 sub_4648b0
    Lifted::new(0x7000b50ab0, fn_7000b50ab0 as *const (), 2), // +0x464ab0 sub_464ab0
    Lifted::new(0x7000b50c40, fn_7000b50c40 as *const (), 0), // +0x464c40 sub_464c40
    Lifted::new(0x7000b50f80, fn_7000b50f80 as *const (), 0), // +0x464f80 sub_464f80
    Lifted::new(0x7000b51440, fn_7000b51440 as *const (), 0), // +0x465440 sub_465440
    Lifted::new(0x7000b51700, fn_7000b51700 as *const (), 0), // +0x465700 sub_465700
    Lifted::new(0x7000b51b50, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b51b60> as *const (), 0), // +0x465b50 sub_465b50
    Lifted::new(0x7000b51b70, fn_7000b51b70 as *const (), 0), // +0x465b70 sub_465b70
    Lifted::new(0x7000b51c20, fn_7000b51c20 as *const (), 0), // +0x465c20 sub_465c20
    Lifted::new(0x7000b51e60, fn_7000b51e60 as *const (), 1), // +0x465e60 sub_465e60
    Lifted::new(0x7000b51e80, fn_7000b51e80 as *const (), 0), // +0x465e80 sub_465e80
    Lifted::new(0x7000b51eb0, fn_7000b51eb0 as *const (), 0), // +0x465eb0 sub_465eb0
    Lifted::new(0x7000b51fa0, tpl_33::<0x700177c312> as *const (), 0), // +0x465fa0 sub_465fa0
    Lifted::new(0x7000b51fd0, tpl_33::<0x700177c312> as *const (), 0), // +0x465fd0 sub_465fd0
    Lifted::new(0x7000b52000, fn_7000b52000 as *const (), 0), // +0x466000 sub_466000
    Lifted::new(0x7000b520c0, fn_7000b520c0 as *const (), 0), // +0x4660c0 sub_4660c0
    Lifted::new(0x7000b52190, fn_7000b52190 as *const (), 0), // +0x466190 sub_466190
    Lifted::new(0x7000b522b0, fn_7000b522b0 as *const (), 0), // +0x4662b0 sub_4662b0
    Lifted::new(0x7000b52380, fn_7000b52380 as *const (), 0), // +0x466380 sub_466380
    Lifted::new(0x7000b523a0, fn_7000b523a0 as *const (), 0), // +0x4663a0 sub_4663a0
    Lifted::new(0x7000b523c0, fn_7000b523c0 as *const (), 0), // +0x4663c0 sub_4663c0
    Lifted::new(0x7000b52410, fn_7000b52410 as *const (), 0), // +0x466410 sub_466410
    Lifted::new(0x7000b52570, tpl_30 as *const (), 1), // +0x466570 sub_466570
    Lifted::new(0x7000b52580, tpl_905::<0x8, 0x7001779e20, 0x70017fe3e8, 0xc, 0x10> as *const (), 0), // +0x466580 sub_466580
    Lifted::new(0x7000b52620, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x466620 sub_466620
    Lifted::new(0x7000b52750, fn_7000b52750 as *const (), 0), // +0x466750 sub_466750
    Lifted::new(0x7000b527e0, fn_7000b527e0 as *const (), 2), // +0x4667e0 sub_4667e0
    Lifted::new(0x7000b52a40, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b52c14, 0x7000b5ae40, 0x7000b52c1a, 0x70007f0910, 0x7000b52c20> as *const (), 0), // +0x466a40 sub_466a40
    Lifted::new(0x7000b52ca0, tpl_906::<0x7000b5af00, 0x700177c312, 0x7001d54968, 0x7000b52e24, 0x7000b5ae20, 0x7000b52e2a> as *const (), 0), // +0x466ca0 sub_466ca0
    Lifted::new(0x7000b52e90, fn_7000b52e90 as *const (), 0), // +0x466e90 sub_466e90
    Lifted::new(0x7000b53160, fn_7000b53160 as *const (), 2), // +0x467160 sub_467160
    Lifted::new(0x7000b53240, fn_7000b53240 as *const (), 2), // +0x467240 sub_467240
    Lifted::new(0x7000b535c0, fn_7000b535c0 as *const (), 2), // +0x4675c0 sub_4675c0
    Lifted::new(0x7000b53750, fn_7000b53750 as *const (), 2), // +0x467750 sub_467750
    Lifted::new(0x7000b53bc0, fn_7000b53bc0 as *const (), 0), // +0x467bc0 sub_467bc0
    Lifted::new(0x7000b53c50, tpl_392::<0x700177c0e0> as *const (), 0), // +0x467c50 sub_467c50
    Lifted::new(0x7000b53c80, fn_7000b53c80 as *const (), 0), // +0x467c80 sub_467c80
    Lifted::new(0x7000b53cf0, tpl_93::<0x8> as *const (), 1), // +0x467cf0 sub_467cf0
    Lifted::new(0x7000b53d00, tpl_6::<0x8> as *const (), 1), // +0x467d00 sub_467d00
    Lifted::new(0x7000b53d10, fn_7000b53d10 as *const (), 1), // +0x467d10 sub_467d10
    Lifted::new(0x7000b53d40, fn_7000b53d40 as *const (), 1), // +0x467d40 sub_467d40
    Lifted::new(0x7000b53d90, fn_7000b53d90 as *const (), 0), // +0x467d90 sub_467d90
    Lifted::new(0x7000b53e00, fn_7000b53e00 as *const (), 1), // +0x467e00 sub_467e00
    Lifted::new(0x7000b53e20, fn_7000b53e20 as *const (), 1), // +0x467e20 sub_467e20
    Lifted::new(0x7000b53e40, fn_7000b53e40 as *const (), 0), // +0x467e40 sub_467e40
    Lifted::new(0x7000b53ed0, tpl_907::<0x8, 0x10, 0x7000b5aec0, 0x7000b53f21> as *const (), 0), // +0x467ed0 sub_467ed0
    Lifted::new(0x7000b53f40, tpl_907::<0x8, 0x10, 0x7000b5aee0, 0x7000b53f91> as *const (), 0), // +0x467f40 sub_467f40
    Lifted::new(0x7000b53fe0, fn_7000b53fe0 as *const (), 0), // +0x467fe0 sub_467fe0
    Lifted::new(0x7000b54020, tpl_18::<0x8> as *const (), 1), // +0x468020 sub_468020
    Lifted::new(0x7000b54040, tpl_19 as *const (), 3), // +0x468040 sub_468040
    Lifted::new(0x7000b54050, tpl_33::<0x700177c312> as *const (), 0), // +0x468050 sub_468050
    Lifted::new(0x7000b54080, tpl_33::<0x700177c312> as *const (), 0), // +0x468080 sub_468080
    Lifted::new(0x7000b540b0, tpl_33::<0x700177c312> as *const (), 0), // +0x4680b0 sub_4680b0
    Lifted::new(0x7000b540e0, fn_7000b540e0 as *const (), 0), // +0x4680e0 sub_4680e0
    Lifted::new(0x7000b54240, fn_7000b54240 as *const (), 1), // +0x468240 sub_468240
    Lifted::new(0x7000b54270, fn_7000b54270 as *const (), 0), // +0x468270 sub_468270
    Lifted::new(0x7000b542c0, fn_7000b542c0 as *const (), 0), // +0x4682c0 sub_4682c0
    Lifted::new(0x7000b543c0, fn_7000b543c0 as *const (), 0), // +0x4683c0 sub_4683c0
    Lifted::new(0x7000b54660, fn_7000b54660 as *const (), 0), // +0x468660 sub_468660
    Lifted::new(0x7000b546e0, fn_7000b546e0 as *const (), 0), // +0x4686e0 sub_4686e0
    Lifted::new(0x7000b54740, fn_7000b54740 as *const (), 0), // +0x468740 sub_468740
    Lifted::new(0x7000b54780, fn_7000b54780 as *const (), 0), // +0x468780 sub_468780
    Lifted::new(0x7000b547e0, fn_7000b547e0 as *const (), 0), // +0x4687e0 sub_4687e0
    Lifted::new(0x7000b54830, fn_7000b54830 as *const (), 0), // +0x468830 sub_468830
    Lifted::new(0x7000b54880, tpl_143::<0xc7f0, 0x7000787a20> as *const (), 0), // +0x468880 sub_468880
    Lifted::new(0x7000b54890, tpl_908::<0x3080, 0xc7f0, 0x70006fb950> as *const (), 0), // +0x468890 sub_468890
    Lifted::new(0x7000b548e0, fn_7000b548e0 as *const (), 1), // +0x4688e0 sub_4688e0
    Lifted::new(0x7000b54920, fn_7000b54920 as *const (), 0), // +0x468920 sub_468920
    Lifted::new(0x7000b549f0, tpl_909::<0x7000b4cfc0, 0x7000b52580, 0x8, 0x7000b5a9e0, 0x700177c312> as *const (), 0), // +0x4689f0 sub_4689f0
    Lifted::new(0x7000b54a80, tpl_909::<0x7000b4dab0, 0x7000b52580, 0x8, 0x7000b5a9e0, 0x700177c312> as *const (), 0), // +0x468a80 sub_468a80
    Lifted::new(0x7000b54b10, fn_7000b54b10 as *const (), 0), // +0x468b10 sub_468b10
    Lifted::new(0x7000b54e00, tpl_146::<0xf> as *const (), 1), // +0x468e00 sub_468e00
    Lifted::new(0x7000b54e10, fn_7000b54e10 as *const (), 1), // +0x468e10 sub_468e10
    Lifted::new(0x7000b54e20, fn_7000b54e20 as *const (), 2), // +0x468e20 sub_468e20
    Lifted::new(0x7000b55060, fn_7000b55060 as *const (), 1), // +0x469060 sub_469060
    Lifted::new(0x7000b550e0, tpl_147::<0x8> as *const (), 1), // +0x4690e0 sub_4690e0
    Lifted::new(0x7000b550f0, tpl_147::<0x8> as *const (), 1), // +0x4690f0 sub_4690f0
    Lifted::new(0x7000b55100, fn_7000b55100 as *const (), 0), // +0x469100 sub_469100
    Lifted::new(0x7000b552e0, fn_7000b552e0 as *const (), 2), // +0x4692e0 sub_4692e0
    Lifted::new(0x7000b555d0, tpl_147::<0x4> as *const (), 1), // +0x4695d0 sub_4695d0
    Lifted::new(0x7000b555e0, tpl_147::<0x4> as *const (), 1), // +0x4695e0 sub_4695e0
    Lifted::new(0x7000b555f0, fn_7000b555f0 as *const (), 0), // +0x4695f0 sub_4695f0
    Lifted::new(0x7000b55730, fn_7000b55730 as *const (), 0), // +0x469730 sub_469730
    Lifted::new(0x7000b557d0, fn_7000b557d0 as *const (), 0), // +0x4697d0 sub_4697d0
    Lifted::new(0x7000b55890, tpl_910::<0x700077b960, 0x70006ff430, 0x7000b50ab0, 0x7000751820, 0x7000790d50, 0x70009d20b0, 0x7000746e10> as *const (), 0), // +0x469890 sub_469890
    Lifted::new(0x7000b55a90, tpl_910::<0x700077b960, 0x70006ff430, 0x7000716960, 0x7000751820, 0x7000790d50, 0x70009d20b0, 0x7000746e10> as *const (), 0), // +0x469a90 sub_469a90
    Lifted::new(0x7000b55c90, fn_7000b55c90 as *const (), 0), // +0x469c90 sub_469c90
    Lifted::new(0x7000b55de0, fn_7000b55de0 as *const (), 0), // +0x469de0 sub_469de0
    Lifted::new(0x7000b55e10, tpl_434::<0xb, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x469e10 sub_469e10
    Lifted::new(0x7000b55ec0, fn_7000b55ec0 as *const (), 1), // +0x469ec0 sub_469ec0
    Lifted::new(0x7000b55ef0, fn_7000b55ef0 as *const (), 2), // +0x469ef0 sub_469ef0
    Lifted::new(0x7000b561f0, fn_7000b561f0 as *const (), 0), // +0x46a1f0 sub_46a1f0
    Lifted::new(0x7000b56280, fn_7000b56280 as *const (), 0), // +0x46a280 sub_46a280
    Lifted::new(0x7000b56610, tpl_39::<0x4> as *const (), 1), // +0x46a610 sub_46a610
    Lifted::new(0x7000b56620, fn_7000b56620 as *const (), 0), // +0x46a620 sub_46a620
    Lifted::new(0x7000b56660, fn_7000b56660 as *const (), 1), // +0x46a660 sub_46a660
    Lifted::new(0x7000b566a0, fn_7000b566a0 as *const (), 0), // +0x46a6a0 sub_46a6a0
    Lifted::new(0x7000b56720, fn_7000b56720 as *const (), 1), // +0x46a720 sub_46a720
    Lifted::new(0x7000b56790, fn_7000b56790 as *const (), 0), // +0x46a790 sub_46a790
    Lifted::new(0x7000b567f0, fn_7000b567f0 as *const (), 0), // +0x46a7f0 sub_46a7f0
    Lifted::new(0x7000b56870, fn_7000b56870 as *const (), 0), // +0x46a870 sub_46a870
    Lifted::new(0x7000b56900, fn_7000b56900 as *const (), 0), // +0x46a900 sub_46a900
    Lifted::new(0x7000b56a40, fn_7000b56a40 as *const (), 0), // +0x46aa40 sub_46aa40
    Lifted::new(0x7000b56e70, fn_7000b56e70 as *const (), 0), // +0x46ae70 sub_46ae70
    Lifted::new(0x7000b56fe0, tpl_36::<0xc950> as *const (), 1), // +0x46afe0 sub_46afe0
    Lifted::new(0x7000b56ff0, fn_7000b56ff0 as *const (), 0), // +0x46aff0 sub_46aff0
    Lifted::new(0x7000b57080, tpl_462::<0x10> as *const (), 1), // +0x46b080 sub_46b080
    Lifted::new(0x7000b570a0, tpl_41 as *const (), 1), // +0x46b0a0 sub_46b0a0
    Lifted::new(0x7000b570b0, fn_7000b570b0 as *const (), 0), // +0x46b0b0 sub_46b0b0
    Lifted::new(0x7000b572e0, fn_7000b572e0 as *const (), 0), // +0x46b2e0 sub_46b2e0
    Lifted::new(0x7000b573f0, fn_7000b573f0 as *const (), 0), // +0x46b3f0 sub_46b3f0
    Lifted::new(0x7000b57410, fn_7000b57410 as *const (), 2), // +0x46b410 sub_46b410
    Lifted::new(0x7000b578d0, fn_7000b578d0 as *const (), 2), // +0x46b8d0 sub_46b8d0
    Lifted::new(0x7000b58310, tpl_911 as *const (), 1), // +0x46c310 sub_46c310
    Lifted::new(0x7000b58320, tpl_911 as *const (), 1), // +0x46c320 sub_46c320
    Lifted::new(0x7000b58330, tpl_912::<0x8, 0x10, 0x7000b5aec0, 0x7000b5837b> as *const (), 0), // +0x46c330 sub_46c330
    Lifted::new(0x7000b58390, tpl_912::<0x8, 0x10, 0x7000b5aee0, 0x7000b583db> as *const (), 0), // +0x46c390 sub_46c390
    Lifted::new(0x7000b583f0, fn_7000b583f0 as *const (), 2), // +0x46c3f0 sub_46c3f0
    Lifted::new(0x7000b58450, fn_7000b58450 as *const (), 0), // +0x46c450 sub_46c450
    Lifted::new(0x7000b58690, fn_7000b58690 as *const (), 0), // +0x46c690 sub_46c690
    Lifted::new(0x7000b589c0, tpl_314::<0x30> as *const (), 1), // +0x46c9c0 sub_46c9c0
    Lifted::new(0x7000b589d0, tpl_146::<0x1f> as *const (), 1), // +0x46c9d0 sub_46c9d0
    Lifted::new(0x7000b589e0, fn_7000b589e0 as *const (), 1), // +0x46c9e0 sub_46c9e0
    Lifted::new(0x7000b58a20, fn_7000b58a20 as *const (), 1), // +0x46ca20 sub_46ca20
    Lifted::new(0x7000b58a80, tpl_47::<0x8> as *const (), 1), // +0x46ca80 sub_46ca80
    Lifted::new(0x7000b58a90, tpl_913::<0x2f> as *const (), 1), // +0x46ca90 sub_46ca90
    Lifted::new(0x7000b58ab0, fn_7000b58ab0 as *const (), 1), // +0x46cab0 sub_46cab0
    Lifted::new(0x7000b58ad0, fn_7000b58ad0 as *const (), 1), // +0x46cad0 sub_46cad0
    Lifted::new(0x7000b58af0, fn_7000b58af0 as *const (), 1), // +0x46caf0 sub_46caf0
    Lifted::new(0x7000b58b30, fn_7000b58b30 as *const (), 0), // +0x46cb30 sub_46cb30
    Lifted::new(0x7000b58df0, tpl_146::<0xb> as *const (), 1), // +0x46cdf0 sub_46cdf0
    Lifted::new(0x7000b58e00, tpl_534::<0x10> as *const (), 1), // +0x46ce00 sub_46ce00
    Lifted::new(0x7000b58e20, fn_7000b58e20 as *const (), 0), // +0x46ce20 sub_46ce20
    Lifted::new(0x7000b59020, fn_7000b59020 as *const (), 2), // +0x46d020 sub_46d020
    Lifted::new(0x7000b591c0, fn_7000b591c0 as *const (), 1), // +0x46d1c0 sub_46d1c0
    Lifted::new(0x7000b591e0, fn_7000b591e0 as *const (), 0), // +0x46d1e0 sub_46d1e0
    Lifted::new(0x7000b59270, fn_7000b59270 as *const (), 0), // +0x46d270 sub_46d270
    Lifted::new(0x7000b59300, fn_7000b59300 as *const (), 0), // +0x46d300 sub_46d300
    Lifted::new(0x7000b59380, fn_7000b59380 as *const (), 0), // +0x46d380 sub_46d380
    Lifted::new(0x7000b593b0, fn_7000b593b0 as *const (), 0), // +0x46d3b0 sub_46d3b0
    Lifted::new(0x7000b594b0, fn_7000b594b0 as *const (), 0), // +0x46d4b0 sub_46d4b0
    Lifted::new(0x7000b59600, fn_7000b59600 as *const (), 1), // +0x46d600 sub_46d600
    Lifted::new(0x7000b59620, fn_7000b59620 as *const (), 0), // +0x46d620 sub_46d620
    Lifted::new(0x7000b59660, fn_7000b59660 as *const (), 2), // +0x46d660 sub_46d660
    Lifted::new(0x7000b59960, tpl_370::<0xd910, 0x70007a1040> as *const (), 0), // +0x46d960 sub_46d960
    Lifted::new(0x7000b59970, tpl_143::<0xc7f0, 0x70007315f0> as *const (), 0), // +0x46d970 sub_46d970
    Lifted::new(0x7000b59980, fn_7000b59980 as *const (), 1), // +0x46d980 sub_46d980
    Lifted::new(0x7000b599a0, fn_7000b599a0 as *const (), 2), // +0x46d9a0 sub_46d9a0
    Lifted::new(0x7000b59a90, fn_7000b59a90 as *const (), 0), // +0x46da90 sub_46da90
    Lifted::new(0x7000b59bf0, fn_7000b59bf0 as *const (), 1), // +0x46dbf0 sub_46dbf0
    Lifted::new(0x7000b59c10, fn_7000b59c10 as *const (), 1), // +0x46dc10 sub_46dc10
    Lifted::new(0x7000b59c30, tpl_143::<0xc7f0, 0x7000746d20> as *const (), 0), // +0x46dc30 sub_46dc30
    Lifted::new(0x7000b59c40, fn_7000b59c40 as *const (), 0), // +0x46dc40 sub_46dc40
    Lifted::new(0x7000b59d70, tpl_51::<0xc950> as *const (), 1), // +0x46dd70 sub_46dd70
    Lifted::new(0x7000b59d80, fn_7000b59d80 as *const (), 1), // +0x46dd80 sub_46dd80
    Lifted::new(0x7000b59da0, fn_7000b59da0 as *const (), 0), // +0x46dda0 sub_46dda0
    Lifted::new(0x7000b59df0, tpl_908::<0x3080, 0xc7f0, 0x70007335d0> as *const (), 0), // +0x46ddf0 sub_46ddf0
    Lifted::new(0x7000b59e40, fn_7000b59e40 as *const (), 0), // +0x46de40 sub_46de40
    Lifted::new(0x7000b5a040, fn_7000b5a040 as *const (), 0), // +0x46e040 sub_46e040
    Lifted::new(0x7000b5a240, tpl_738 as *const (), 1), // +0x46e240 sub_46e240
    Lifted::new(0x7000b5a250, fn_7000b5a250 as *const (), 0), // +0x46e250 sub_46e250
    Lifted::new(0x7000b5a450, tpl_738 as *const (), 1), // +0x46e450 sub_46e450
    Lifted::new(0x7000b5a460, tpl_738 as *const (), 1), // +0x46e460 sub_46e460
    Lifted::new(0x7000b5a470, fn_7000b5a470 as *const (), 2), // +0x46e470 sub_46e470
    Lifted::new(0x7000b5a6e0, tpl_472::<0x700076b0b0, 0x70007773c0> as *const (), 0), // +0x46e6e0 sub_46e6e0
    Lifted::new(0x7000b5a710, tpl_472::<0x700076b0b0, 0x70007a0970> as *const (), 0), // +0x46e710 sub_46e710
    Lifted::new(0x7000b5a740, fn_7000b5a740 as *const (), 0), // +0x46e740 sub_46e740
    Lifted::new(0x7000b5a7e0, tpl_484::<0x2f> as *const (), 1), // +0x46e7e0 sub_46e7e0
    Lifted::new(0x7000b5a7f0, tpl_914::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b5a877> as *const (), 0), // +0x46e7f0 sub_46e7f0
    Lifted::new(0x7000b5a8a0, tpl_530::<0x10> as *const (), 0), // +0x46e8a0 sub_46e8a0
    Lifted::new(0x7000b5a8c0, tpl_530::<0x10> as *const (), 0), // +0x46e8c0 sub_46e8c0
    Lifted::new(0x7000b5a8e0, tpl_531::<0x10, 0x7000b5a9e0, 0x700177c312> as *const (), 0), // +0x46e8e0 sub_46e8e0
    Lifted::new(0x7000b5a920, tpl_531::<0x10, 0x7000b5ac30, 0x700177c312> as *const (), 0), // +0x46e920 sub_46e920
    Lifted::new(0x7000b5a960, tpl_532::<0x8, 0x7001c6b388, 0x700177c100, 0x10> as *const (), 0), // +0x46e960 sub_46e960
    Lifted::new(0x7000b5a9a0, tpl_532::<0x8, 0x7001c6b328, 0x700177c100, 0x10> as *const (), 0), // +0x46e9a0 sub_46e9a0
    Lifted::new(0x7000b5a9e0, fn_7000b5a9e0 as *const (), 0), // +0x46e9e0 sub_46e9e0
    Lifted::new(0x7000b5ac30, fn_7000b5ac30 as *const (), 0), // +0x46ec30 sub_46ec30
    Lifted::new(0x7000b5ae00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b5ae10> as *const (), 0), // +0x46ee00 sub_46ee00
    Lifted::new(0x7000b5ae20, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b5ae30> as *const (), 0), // +0x46ee20 sub_46ee20
    Lifted::new(0x7000b5ae40, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b5ae50> as *const (), 0), // +0x46ee40 sub_46ee40
    Lifted::new(0x7000b5ae60, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b5ae70> as *const (), 0), // +0x46ee60 sub_46ee60
    Lifted::new(0x7000b5ae80, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b5ae90> as *const (), 0), // +0x46ee80 sub_46ee80
    Lifted::new(0x7000b5aea0, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000b5aeb0> as *const (), 0), // +0x46eea0 sub_46eea0
    Lifted::new(0x7000b5aec0, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000b5aed0> as *const (), 0), // +0x46eec0 sub_46eec0
    Lifted::new(0x7000b5aee0, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000b5aef0> as *const (), 0), // +0x46eee0 sub_46eee0
    Lifted::new(0x7000b5af00, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b5af54, 0x70007f0910, 0x7000b5af6f> as *const (), 0), // +0x46ef00 sub_46ef00
    Lifted::new(0x7000b5af90, tpl_434::<0x3, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x46ef90 sub_46ef90
    Lifted::new(0x7000b5b040, fn_7000b5b040 as *const (), 0), // +0x46f040 sub_46f040
    Lifted::new(0x7000b5b100, tpl_434::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x46f100 sub_46f100
    Lifted::new(0x7000b5b1b0, tpl_434::<0xb, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x46f1b0 sub_46f1b0
    Lifted::new(0x7000b5b260, fn_7000b5b260 as *const (), 0), // +0x46f260 sub_46f260
    Lifted::new(0x7000b5b2e0, fn_7000b5b2e0 as *const (), 0), // +0x46f2e0 sub_46f2e0
    Lifted::new(0x7000b5b4a0, fn_7000b5b4a0 as *const (), 0), // +0x46f4a0 sub_46f4a0
    Lifted::new(0x7000b5b500, fn_7000b5b500 as *const (), 0), // +0x46f500 sub_46f500
    Lifted::new(0x7000b5b6e0, fn_7000b5b6e0 as *const (), 0), // +0x46f6e0 sub_46f6e0
    Lifted::new(0x7000b5b940, tpl_47::<0x61> as *const (), 1), // +0x46f940 sub_46f940
    Lifted::new(0x7000b5b950, fn_7000b5b950 as *const (), 0), // +0x46f950 sub_46f950
    Lifted::new(0x7000b5ba60, fn_7000b5ba60 as *const (), 0), // +0x46fa60 sub_46fa60
    Lifted::new(0x7000b5bc10, fn_7000b5bc10 as *const (), 0), // +0x46fc10 sub_46fc10
    Lifted::new(0x7000b5bd00, fn_7000b5bd00 as *const (), 2), // +0x46fd00 sub_46fd00
    Lifted::new(0x7000b5be10, fn_7000b5be10 as *const (), 0), // +0x46fe10 sub_46fe10
    Lifted::new(0x7000b5be50, fn_7000b5be50 as *const (), 0), // +0x46fe50 sub_46fe50
    Lifted::new(0x7000b5be90, fn_7000b5be90 as *const (), 0), // +0x46fe90 sub_46fe90
    Lifted::new(0x7000b5bf30, fn_7000b5bf30 as *const (), 0), // +0x46ff30 sub_46ff30
    Lifted::new(0x7000b5bf70, tpl_47::<0x60> as *const (), 1), // +0x46ff70 sub_46ff70
    Lifted::new(0x7000b5bf80, tpl_743::<0x7000b5bfb0> as *const (), 0), // +0x46ff80 sub_46ff80
    Lifted::new(0x7000b5bfb0, fn_7000b5bfb0 as *const (), 0), // +0x46ffb0 sub_46ffb0
    Lifted::new(0x7000b5c0a0, fn_7000b5c0a0 as *const (), 1), // +0x4700a0 sub_4700a0
    Lifted::new(0x7000b5c0f0, fn_7000b5c0f0 as *const (), 0), // +0x4700f0 sub_4700f0
    Lifted::new(0x7000b5c170, tpl_915::<0x70017fe4d0, 0x7000b5c9b0, 0x30, 0x8, 0x7000b5fc50, 0x7000b5d310, 0x7000b5c2b0> as *const (), 0), // +0x470170 sub_470170
    Lifted::new(0x7000b5c1f0, tpl_915::<0x70017fe4d0, 0x7000b5cb70, 0x30, 0x8, 0x7000b5fc50, 0x7000b5d310, 0x7000b5c2b0> as *const (), 0), // +0x4701f0 sub_4701f0
    Lifted::new(0x7000b5c270, tpl_916::<0x7000b5ce50, 0x700177c312> as *const (), 0), // +0x470270 sub_470270
    Lifted::new(0x7000b5c2b0, fn_7000b5c2b0 as *const (), 0), // +0x4702b0 sub_4702b0
    Lifted::new(0x7000b5c330, fn_7000b5c330 as *const (), 0), // +0x470330 sub_470330
    Lifted::new(0x7000b5c470, fn_7000b5c470 as *const (), 0), // +0x470470 sub_470470
    Lifted::new(0x7000b5c540, tpl_208::<0x30> as *const (), 1), // +0x470540 sub_470540
    Lifted::new(0x7000b5c550, tpl_917::<0x38, 0x7000b616f0, 0x700077bcd0> as *const (), 0), // +0x470550 sub_470550
    Lifted::new(0x7000b5c5a0, fn_7000b5c5a0 as *const (), 0), // +0x4705a0 sub_4705a0
    Lifted::new(0x7000b5c5e0, fn_7000b5c5e0 as *const (), 0), // +0x4705e0 sub_4705e0
    Lifted::new(0x7000b5c620, fn_7000b5c620 as *const (), 1), // +0x470620 sub_470620
    Lifted::new(0x7000b5c630, tpl_917::<0x38, 0x7000b61790, 0x700077bcd0> as *const (), 0), // +0x470630 sub_470630
    Lifted::new(0x7000b5c680, tpl_110::<0x7000b5fc50> as *const (), 0), // +0x470680 sub_470680
    Lifted::new(0x7000b5c690, tpl_110::<0x7000b5d670> as *const (), 0), // +0x470690 sub_470690
    Lifted::new(0x7000b5c6a0, tpl_110::<0x7000b5eef0> as *const (), 0), // +0x4706a0 sub_4706a0
    Lifted::new(0x7000b5c6b0, fn_7000b5c6b0 as *const (), 0), // +0x4706b0 sub_4706b0
    Lifted::new(0x7000b5c700, fn_7000b5c700 as *const (), 0), // +0x470700 sub_470700
    Lifted::new(0x7000b5c8b0, fn_7000b5c8b0 as *const (), 0), // +0x4708b0 sub_4708b0
    Lifted::new(0x7000b5c9b0, fn_7000b5c9b0 as *const (), 0), // +0x4709b0 sub_4709b0
    Lifted::new(0x7000b5cb70, fn_7000b5cb70 as *const (), 0), // +0x470b70 sub_470b70
    Lifted::new(0x7000b5ce50, tpl_531::<0x8, 0x7000730410, 0x700177c312> as *const (), 0), // +0x470e50 sub_470e50
    Lifted::new(0x7000b5ce90, fn_7000b5ce90 as *const (), 0), // +0x470e90 sub_470e90
    Lifted::new(0x7000b5cee0, fn_7000b5cee0 as *const (), 0), // +0x470ee0 sub_470ee0
    Lifted::new(0x7000b5d030, fn_7000b5d030 as *const (), 0), // +0x471030 sub_471030
    Lifted::new(0x7000b5d120, fn_7000b5d120 as *const (), 0), // +0x471120 sub_471120
    Lifted::new(0x7000b5d230, fn_7000b5d230 as *const (), 0), // +0x471230 sub_471230
    Lifted::new(0x7000b5d270, fn_7000b5d270 as *const (), 0), // +0x471270 sub_471270
    Lifted::new(0x7000b5d310, fn_7000b5d310 as *const (), 0), // +0x471310 sub_471310
    Lifted::new(0x7000b5d3f0, fn_7000b5d3f0 as *const (), 0), // +0x4713f0 sub_4713f0
    Lifted::new(0x7000b5d540, fn_7000b5d540 as *const (), 0), // +0x471540 sub_471540
    Lifted::new(0x7000b5d630, fn_7000b5d630 as *const (), 1), // +0x471630 sub_471630
    Lifted::new(0x7000b5d670, fn_7000b5d670 as *const (), 1), // +0x471670 sub_471670
    Lifted::new(0x7000b5d6c0, fn_7000b5d6c0 as *const (), 0), // +0x4716c0 sub_4716c0
    Lifted::new(0x7000b5d7b0, fn_7000b5d7b0 as *const (), 0), // +0x4717b0 sub_4717b0
    Lifted::new(0x7000b5dc30, fn_7000b5dc30 as *const (), 1), // +0x471c30 sub_471c30
    Lifted::new(0x7000b5dc70, fn_7000b5dc70 as *const (), 0), // +0x471c70 sub_471c70
    Lifted::new(0x7000b5dff0, fn_7000b5dff0 as *const (), 0), // +0x471ff0 sub_471ff0
    Lifted::new(0x7000b5e040, fn_7000b5e040 as *const (), 0), // +0x472040 sub_472040
    Lifted::new(0x7000b5e0d0, fn_7000b5e0d0 as *const (), 2), // +0x4720d0 sub_4720d0
    Lifted::new(0x7000b5e340, fn_7000b5e340 as *const (), 0), // +0x472340 sub_472340
    Lifted::new(0x7000b5e570, fn_7000b5e570 as *const (), 0), // +0x472570 sub_472570
    Lifted::new(0x7000b5e6c0, fn_7000b5e6c0 as *const (), 0), // +0x4726c0 sub_4726c0
    Lifted::new(0x7000b5eb70, fn_7000b5eb70 as *const (), 0), // +0x472b70 sub_472b70
    Lifted::new(0x7000b5ec90, fn_7000b5ec90 as *const (), 0), // +0x472c90 sub_472c90
    Lifted::new(0x7000b5eef0, fn_7000b5eef0 as *const (), 0), // +0x472ef0 sub_472ef0
    Lifted::new(0x7000b5f330, fn_7000b5f330 as *const (), 2), // +0x473330 sub_473330
    Lifted::new(0x7000b5fc50, fn_7000b5fc50 as *const (), 0), // +0x473c50 sub_473c50
    Lifted::new(0x7000b5fea0, fn_7000b5fea0 as *const (), 0), // +0x473ea0 sub_473ea0
    Lifted::new(0x7000b5ffd0, fn_7000b5ffd0 as *const (), 0), // +0x473fd0 sub_473fd0
    Lifted::new(0x7000b601f0, fn_7000b601f0 as *const (), 0), // +0x4741f0 sub_4741f0
    Lifted::new(0x7000b60730, fn_7000b60730 as *const (), 0), // +0x474730 sub_474730
    Lifted::new(0x7000b60db0, fn_7000b60db0 as *const (), 0), // +0x474db0 sub_474db0
    Lifted::new(0x7000b61090, fn_7000b61090 as *const (), 0), // +0x475090 sub_475090
    Lifted::new(0x7000b611a0, fn_7000b611a0 as *const (), 0), // +0x4751a0 sub_4751a0
    Lifted::new(0x7000b61420, fn_7000b61420 as *const (), 0), // +0x475420 sub_475420
    Lifted::new(0x7000b61540, tpl_753::<0x70017fe5d0, 0x8> as *const (), 1), // +0x475540 sub_475540
    Lifted::new(0x7000b61560, tpl_918::<0x70017fe610, 0x8> as *const (), 3), // +0x475560 sub_475560
    Lifted::new(0x7000b61580, tpl_754::<0x700177c312> as *const (), 0), // +0x475580 sub_475580
    Lifted::new(0x7000b615a0, tpl_919::<0x700177c312> as *const (), 0), // +0x4755a0 sub_4755a0
    Lifted::new(0x7000b615c0, fn_7000b615c0 as *const (), 1), // +0x4755c0 sub_4755c0
    Lifted::new(0x7000b615d0, fn_7000b615d0 as *const (), 0), // +0x4755d0 sub_4755d0
    Lifted::new(0x7000b61670, tpl_52::<0x8> as *const (), 1), // +0x475670 sub_475670
    Lifted::new(0x7000b61680, tpl_52::<0x8> as *const (), 1), // +0x475680 sub_475680
    Lifted::new(0x7000b61690, tpl_753::<0x70017fe5d0, 0x8> as *const (), 1), // +0x475690 sub_475690
    Lifted::new(0x7000b616b0, tpl_918::<0x70017fe610, 0x8> as *const (), 3), // +0x4756b0 sub_4756b0
    Lifted::new(0x7000b616d0, tpl_315::<0x7001c6b420> as *const (), 1), // +0x4756d0 sub_4756d0
    Lifted::new(0x7000b616e0, tpl_315::<0x7001c6b3d0> as *const (), 1), // +0x4756e0 sub_4756e0
    Lifted::new(0x7000b616f0, tpl_920::<0x20, 0x50, 0x6c, 0x28, 0xffffffffffffffff, 0x7, 0x7000dc35b0, 0x7000738270, 0x7000f08280> as *const (), 0), // +0x4756f0 sub_4756f0
    Lifted::new(0x7000b61790, tpl_920::<0x20, 0x50, 0x68, 0x28, 0xffffffffffffffff, 0x7, 0x7000dc35b0, 0x7000738270, 0x7000f09be0> as *const (), 0), // +0x475790 sub_475790
    Lifted::new(0x7000b61830, tpl_921::<0x7001779e20, 0x7000b5c0f0> as *const (), 0), // +0x475830 sub_475830
    Lifted::new(0x7000b618b0, fn_7000b618b0 as *const (), 2), // +0x4758b0 sub_4758b0
    Lifted::new(0x7000b61a70, fn_7000b61a70 as *const (), 0), // +0x475a70 sub_475a70
    Lifted::new(0x7000b61ae0, fn_7000b61ae0 as *const (), 0), // +0x475ae0 sub_475ae0
    Lifted::new(0x7000b61b60, tpl_420::<0x20, 0x70017d3d30, 0x8, 0x700075ca60, 0x700177c312> as *const (), 0), // +0x475b60 sub_475b60
    Lifted::new(0x7000b61bc0, fn_7000b61bc0 as *const (), 2), // +0x475bc0 sub_475bc0
    Lifted::new(0x7000b61e40, fn_7000b61e40 as *const (), 2), // +0x475e40 sub_475e40
    Lifted::new(0x7000b62050, tpl_208::<0x648> as *const (), 1), // +0x476050 sub_476050
    Lifted::new(0x7000b62060, fn_7000b62060 as *const (), 0), // +0x476060 sub_476060
    Lifted::new(0x7000b62160, fn_7000b62160 as *const (), 0), // +0x476160 sub_476160
    Lifted::new(0x7000b62210, fn_7000b62210 as *const (), 0), // +0x476210 sub_476210
    Lifted::new(0x7000b62280, fn_7000b62280 as *const (), 0), // +0x476280 sub_476280
    Lifted::new(0x7000b622c0, fn_7000b622c0 as *const (), 0), // +0x4762c0 sub_4762c0
    Lifted::new(0x7000b625c0, fn_7000b625c0 as *const (), 0), // +0x4765c0 sub_4765c0
    Lifted::new(0x7000b626d0, fn_7000b626d0 as *const (), 0), // +0x4766d0 sub_4766d0
    Lifted::new(0x7000b627a0, fn_7000b627a0 as *const (), 1), // +0x4767a0 sub_4767a0
    Lifted::new(0x7000b62800, fn_7000b62800 as *const (), 0), // +0x476800 sub_476800
    Lifted::new(0x7000b62890, fn_7000b62890 as *const (), 0), // +0x476890 sub_476890
    Lifted::new(0x7000b62940, fn_7000b62940 as *const (), 0), // +0x476940 sub_476940
    Lifted::new(0x7000b62be0, fn_7000b62be0 as *const (), 0), // +0x476be0 sub_476be0
    Lifted::new(0x7000b62de0, fn_7000b62de0 as *const (), 2), // +0x476de0 sub_476de0
    Lifted::new(0x7000b63540, fn_7000b63540 as *const (), 0), // +0x477540 sub_477540
    Lifted::new(0x7000b63760, tpl_293::<0x8, 0x7000b62890, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000b637ca> as *const (), 0), // +0x477760 sub_477760
    Lifted::new(0x7000b637f0, tpl_903::<0x10, 0xfffffffffffffff0, 0x20, 0xffffffffffffffd8, 0xffffffffffffffe8, 0xffffffffffffffe0> as *const (), 1), // +0x4777f0 sub_4777f0
    Lifted::new(0x7000b63880, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b63890> as *const (), 0), // +0x477880 sub_477880
    Lifted::new(0x7000b638a0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b638b0> as *const (), 0), // +0x4778a0 sub_4778a0
    Lifted::new(0x7000b638c0, tpl_300::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b6390d, 0x70007f0910, 0x7000b6392b> as *const (), 0), // +0x4778c0 sub_4778c0
    Lifted::new(0x7000b63950, tpl_55::<0x700177c0d0> as *const (), 0), // +0x477950 sub_477950
    Lifted::new(0x7000b63990, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b63b68, 0x7000b73520, 0x7000b63b6e, 0x70007f0910, 0x7000b63b74> as *const (), 2), // +0x477990 sub_477990
    Lifted::new(0x7000b63bf0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b63dc4, 0x7000b734e0, 0x7000b63dca, 0x70007f0910, 0x7000b63dd0> as *const (), 0), // +0x477bf0 sub_477bf0
    Lifted::new(0x7000b63e50, tpl_876::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b64051, 0x7000b73500, 0x7000b64057, 0x70007f0910, 0x7000b6405d> as *const (), 2), // +0x477e50 sub_477e50
    Lifted::new(0x7000b640f0, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000b731b0, 0x70017d37c0, 0x70017790e0, 0x7000b643d1> as *const (), 2), // +0x4780f0 sub_4780f0
    Lifted::new(0x7000b64490, tpl_68 as *const (), 1), // +0x478490 sub_478490
    Lifted::new(0x7000b64570, fn_7000b64570 as *const (), 0), // +0x478570 sub_478570
    Lifted::new(0x7000b64690, fn_7000b64690 as *const (), 1), // +0x478690 sub_478690
    Lifted::new(0x7000b64740, fn_7000b64740 as *const (), 0), // +0x478740 sub_478740
    Lifted::new(0x7000b64790, fn_7000b64790 as *const (), 0), // +0x478790 sub_478790
    Lifted::new(0x7000b64860, fn_7000b64860 as *const (), 0), // +0x478860 sub_478860
    Lifted::new(0x7000b64900, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000b649a3> as *const (), 0), // +0x478900 sub_478900
    Lifted::new(0x7000b649d0, fn_7000b649d0 as *const (), 0), // +0x4789d0 sub_4789d0
    Lifted::new(0x7000b649f0, fn_7000b649f0 as *const (), 0), // +0x4789f0 sub_4789f0
    Lifted::new(0x7000b64a20, fn_7000b64a20 as *const (), 0), // +0x478a20 sub_478a20
    Lifted::new(0x7000b64ad0, fn_7000b64ad0 as *const (), 1), // +0x478ad0 sub_478ad0
    Lifted::new(0x7000b64af0, fn_7000b64af0 as *const (), 0), // +0x478af0 sub_478af0
    Lifted::new(0x7000b64b20, tpl_116::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b64bba> as *const (), 0), // +0x478b20 sub_478b20
    Lifted::new(0x7000b64bf0, tpl_33::<0x700177c312> as *const (), 0), // +0x478bf0 sub_478bf0
    Lifted::new(0x7000b64c20, tpl_33::<0x700177c312> as *const (), 0), // +0x478c20 sub_478c20
    Lifted::new(0x7000b64c50, fn_7000b64c50 as *const (), 0), // +0x478c50 sub_478c50
    Lifted::new(0x7000b64ca0, fn_7000b64ca0 as *const (), 2), // +0x478ca0 sub_478ca0
    Lifted::new(0x7000b64e40, fn_7000b64e40 as *const (), 2), // +0x478e40 sub_478e40
    Lifted::new(0x7000b64fd0, fn_7000b64fd0 as *const (), 0), // +0x478fd0 sub_478fd0
    Lifted::new(0x7000b65000, fn_7000b65000 as *const (), 2), // +0x479000 sub_479000
    Lifted::new(0x7000b65290, fn_7000b65290 as *const (), 2), // +0x479290 sub_479290
    Lifted::new(0x7000b653e0, fn_7000b653e0 as *const (), 3), // +0x4793e0 sub_4793e0
    Lifted::new(0x7000b65410, fn_7000b65410 as *const (), 1), // +0x479410 sub_479410
    Lifted::new(0x7000b65420, fn_7000b65420 as *const (), 2), // +0x479420 sub_479420
    Lifted::new(0x7000b659d0, fn_7000b659d0 as *const (), 0), // +0x4799d0 sub_4799d0
    Lifted::new(0x7000b65a60, fn_7000b65a60 as *const (), 0), // +0x479a60 sub_479a60
    Lifted::new(0x7000b65bf0, fn_7000b65bf0 as *const (), 2), // +0x479bf0 sub_479bf0
    Lifted::new(0x7000b65ce0, fn_7000b65ce0 as *const (), 1), // +0x479ce0 sub_479ce0
    Lifted::new(0x7000b65d00, fn_7000b65d00 as *const (), 2), // +0x479d00 sub_479d00
    Lifted::new(0x7000b65f40, fn_7000b65f40 as *const (), 2), // +0x479f40 sub_479f40
    Lifted::new(0x7000b66130, fn_7000b66130 as *const (), 2), // +0x47a130 sub_47a130
    Lifted::new(0x7000b66390, fn_7000b66390 as *const (), 2), // +0x47a390 sub_47a390
    Lifted::new(0x7000b664e0, fn_7000b664e0 as *const (), 0), // +0x47a4e0 sub_47a4e0
    Lifted::new(0x7000b66560, fn_7000b66560 as *const (), 1), // +0x47a560 sub_47a560
    Lifted::new(0x7000b66590, fn_7000b66590 as *const (), 2), // +0x47a590 sub_47a590
    Lifted::new(0x7000b667c0, fn_7000b667c0 as *const (), 2), // +0x47a7c0 sub_47a7c0
    Lifted::new(0x7000b66a10, fn_7000b66a10 as *const (), 0), // +0x47aa10 sub_47aa10
    Lifted::new(0x7000b66a90, fn_7000b66a90 as *const (), 2), // +0x47aa90 sub_47aa90
    Lifted::new(0x7000b66b80, fn_7000b66b80 as *const (), 2), // +0x47ab80 sub_47ab80
    Lifted::new(0x7000b66e60, fn_7000b66e60 as *const (), 2), // +0x47ae60 sub_47ae60
    Lifted::new(0x7000b66f40, fn_7000b66f40 as *const (), 2), // +0x47af40 sub_47af40
    Lifted::new(0x7000b67100, fn_7000b67100 as *const (), 2), // +0x47b100 sub_47b100
    Lifted::new(0x7000b67330, fn_7000b67330 as *const (), 0), // +0x47b330 sub_47b330
    Lifted::new(0x7000b673f0, fn_7000b673f0 as *const (), 2), // +0x47b3f0 sub_47b3f0
    Lifted::new(0x7000b675b0, fn_7000b675b0 as *const (), 2), // +0x47b5b0 sub_47b5b0
    Lifted::new(0x7000b67800, fn_7000b67800 as *const (), 2), // +0x47b800 sub_47b800
    Lifted::new(0x7000b67af0, fn_7000b67af0 as *const (), 2), // +0x47baf0 sub_47baf0
    Lifted::new(0x7000b67c40, fn_7000b67c40 as *const (), 2), // +0x47bc40 sub_47bc40
    Lifted::new(0x7000b67d70, fn_7000b67d70 as *const (), 2), // +0x47bd70 sub_47bd70
    Lifted::new(0x7000b67e40, fn_7000b67e40 as *const (), 2), // +0x47be40 sub_47be40
    Lifted::new(0x7000b68240, fn_7000b68240 as *const (), 2), // +0x47c240 sub_47c240
    Lifted::new(0x7000b683c0, fn_7000b683c0 as *const (), 2), // +0x47c3c0 sub_47c3c0
    Lifted::new(0x7000b68850, fn_7000b68850 as *const (), 2), // +0x47c850 sub_47c850
    Lifted::new(0x7000b69060, fn_7000b69060 as *const (), 2), // +0x47d060 sub_47d060
    Lifted::new(0x7000b69240, fn_7000b69240 as *const (), 2), // +0x47d240 sub_47d240
    Lifted::new(0x7000b693e0, fn_7000b693e0 as *const (), 2), // +0x47d3e0 sub_47d3e0
    Lifted::new(0x7000b694f0, fn_7000b694f0 as *const (), 0), // +0x47d4f0 sub_47d4f0
    Lifted::new(0x7000b696f0, fn_7000b696f0 as *const (), 1), // +0x47d6f0 sub_47d6f0
    Lifted::new(0x7000b69780, fn_7000b69780 as *const (), 2), // +0x47d780 sub_47d780
    Lifted::new(0x7000b69930, fn_7000b69930 as *const (), 0), // +0x47d930 sub_47d930
    Lifted::new(0x7000b69970, fn_7000b69970 as *const (), 2), // +0x47d970 sub_47d970
    Lifted::new(0x7000b69df0, fn_7000b69df0 as *const (), 2), // +0x47ddf0 sub_47ddf0
    Lifted::new(0x7000b6a120, fn_7000b6a120 as *const (), 2), // +0x47e120 sub_47e120
    Lifted::new(0x7000b6a310, fn_7000b6a310 as *const (), 2), // +0x47e310 sub_47e310
    Lifted::new(0x7000b6a7a0, fn_7000b6a7a0 as *const (), 2), // +0x47e7a0 sub_47e7a0
    Lifted::new(0x7000b6aaa0, fn_7000b6aaa0 as *const (), 1), // +0x47eaa0 sub_47eaa0
    Lifted::new(0x7000b6aac0, fn_7000b6aac0 as *const (), 2), // +0x47eac0 sub_47eac0
    Lifted::new(0x7000b6af00, fn_7000b6af00 as *const (), 0), // +0x47ef00 sub_47ef00
    Lifted::new(0x7000b6b020, fn_7000b6b020 as *const (), 0), // +0x47f020 sub_47f020
    Lifted::new(0x7000b6b220, fn_7000b6b220 as *const (), 2), // +0x47f220 sub_47f220
    Lifted::new(0x7000b6b390, fn_7000b6b390 as *const (), 2), // +0x47f390 sub_47f390
    Lifted::new(0x7000b6b990, fn_7000b6b990 as *const (), 2), // +0x47f990 sub_47f990
    Lifted::new(0x7000b6bec0, fn_7000b6bec0 as *const (), 2), // +0x47fec0 sub_47fec0
    Lifted::new(0x7000b6c170, fn_7000b6c170 as *const (), 1), // +0x480170 sub_480170
    Lifted::new(0x7000b6c190, fn_7000b6c190 as *const (), 2), // +0x480190 sub_480190
    Lifted::new(0x7000b6c2f0, fn_7000b6c2f0 as *const (), 2), // +0x4802f0 sub_4802f0
    Lifted::new(0x7000b6c4f0, fn_7000b6c4f0 as *const (), 2), // +0x4804f0 sub_4804f0
    Lifted::new(0x7000b6cae0, fn_7000b6cae0 as *const (), 0), // +0x480ae0 sub_480ae0
    Lifted::new(0x7000b6cb50, fn_7000b6cb50 as *const (), 0), // +0x480b50 sub_480b50
    Lifted::new(0x7000b6cbc0, fn_7000b6cbc0 as *const (), 0), // +0x480bc0 sub_480bc0
    Lifted::new(0x7000b6cc30, tpl_15 as *const (), 1), // +0x480c30 sub_480c30
    Lifted::new(0x7000b6cc40, fn_7000b6cc40 as *const (), 0), // +0x480c40 sub_480c40
    Lifted::new(0x7000b6cfe0, fn_7000b6cfe0 as *const (), 0), // +0x480fe0 sub_480fe0
    Lifted::new(0x7000b6d0f0, tpl_252::<0x700078ebd0> as *const (), 0), // +0x4810f0 sub_4810f0
    Lifted::new(0x7000b6d100, fn_7000b6d100 as *const (), 0), // +0x481100 sub_481100
    Lifted::new(0x7000b6d110, fn_7000b6d110 as *const (), 2), // +0x481110 sub_481110
    Lifted::new(0x7000b6d670, fn_7000b6d670 as *const (), 2), // +0x481670 sub_481670
    Lifted::new(0x7000b6dad0, fn_7000b6dad0 as *const (), 2), // +0x481ad0 sub_481ad0
    Lifted::new(0x7000b6dd80, fn_7000b6dd80 as *const (), 2), // +0x481d80 sub_481d80
    Lifted::new(0x7000b6e100, fn_7000b6e100 as *const (), 2), // +0x482100 sub_482100
    Lifted::new(0x7000b6e510, fn_7000b6e510 as *const (), 2), // +0x482510 sub_482510
    Lifted::new(0x7000b6e840, fn_7000b6e840 as *const (), 2), // +0x482840 sub_482840
    Lifted::new(0x7000b6ebf0, fn_7000b6ebf0 as *const (), 2), // +0x482bf0 sub_482bf0
    Lifted::new(0x7000b6ed40, fn_7000b6ed40 as *const (), 2), // +0x482d40 sub_482d40
    Lifted::new(0x7000b6ef90, fn_7000b6ef90 as *const (), 2), // +0x482f90 sub_482f90
    Lifted::new(0x7000b6f8c0, fn_7000b6f8c0 as *const (), 2), // +0x4838c0 sub_4838c0
    Lifted::new(0x7000b6fb20, fn_7000b6fb20 as *const (), 2), // +0x483b20 sub_483b20
    Lifted::new(0x7000b6fd40, fn_7000b6fd40 as *const (), 2), // +0x483d40 sub_483d40
    Lifted::new(0x7000b70430, fn_7000b70430 as *const (), 2), // +0x484430 sub_484430
    Lifted::new(0x7000b70880, fn_7000b70880 as *const (), 2), // +0x484880 sub_484880
    Lifted::new(0x7000b70ab0, fn_7000b70ab0 as *const (), 0), // +0x484ab0 sub_484ab0
    Lifted::new(0x7000b70b30, fn_7000b70b30 as *const (), 2), // +0x484b30 sub_484b30
    Lifted::new(0x7000b70c40, fn_7000b70c40 as *const (), 2), // +0x484c40 sub_484c40
    Lifted::new(0x7000b70d10, fn_7000b70d10 as *const (), 2), // +0x484d10 sub_484d10
    Lifted::new(0x7000b70dd0, fn_7000b70dd0 as *const (), 2), // +0x484dd0 sub_484dd0
    Lifted::new(0x7000b710e0, fn_7000b710e0 as *const (), 1), // +0x4850e0 sub_4850e0
    Lifted::new(0x7000b710f0, fn_7000b710f0 as *const (), 2), // +0x4850f0 sub_4850f0
    Lifted::new(0x7000b71360, fn_7000b71360 as *const (), 2), // +0x485360 sub_485360
    Lifted::new(0x7000b715e0, fn_7000b715e0 as *const (), 2), // +0x4855e0 sub_4855e0
    Lifted::new(0x7000b71890, fn_7000b71890 as *const (), 0), // +0x485890 sub_485890
    Lifted::new(0x7000b71a20, fn_7000b71a20 as *const (), 0), // +0x485a20 sub_485a20
    Lifted::new(0x7000b71a30, fn_7000b71a30 as *const (), 2), // +0x485a30 sub_485a30
    Lifted::new(0x7000b71b30, fn_7000b71b30 as *const (), 2), // +0x485b30 sub_485b30
    Lifted::new(0x7000b71dc0, fn_7000b71dc0 as *const (), 1), // +0x485dc0 sub_485dc0
    Lifted::new(0x7000b71dd0, fn_7000b71dd0 as *const (), 2), // +0x485dd0 sub_485dd0
    Lifted::new(0x7000b71e90, fn_7000b71e90 as *const (), 2), // +0x485e90 sub_485e90
    Lifted::new(0x7000b72180, fn_7000b72180 as *const (), 0), // +0x486180 sub_486180
    Lifted::new(0x7000b721e0, fn_7000b721e0 as *const (), 2), // +0x4861e0 sub_4861e0
    Lifted::new(0x7000b72500, fn_7000b72500 as *const (), 2), // +0x486500 sub_486500
    Lifted::new(0x7000b727c0, fn_7000b727c0 as *const (), 1), // +0x4867c0 sub_4867c0
    Lifted::new(0x7000b72830, fn_7000b72830 as *const (), 0), // +0x486830 sub_486830
    Lifted::new(0x7000b72880, fn_7000b72880 as *const (), 2), // +0x486880 sub_486880
    Lifted::new(0x7000b72940, fn_7000b72940 as *const (), 2), // +0x486940 sub_486940
    Lifted::new(0x7000b72b00, fn_7000b72b00 as *const (), 1), // +0x486b00 sub_486b00
    Lifted::new(0x7000b72b30, tpl_15 as *const (), 1), // +0x486b30 sub_486b30
    Lifted::new(0x7000b72b40, fn_7000b72b40 as *const (), 2), // +0x486b40 sub_486b40
    Lifted::new(0x7000b72c80, fn_7000b72c80 as *const (), 0), // +0x486c80 sub_486c80
    Lifted::new(0x7000b72ec0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b72fc7, 0x7000b64490, 0x70007f0910, 0x7000b72fe3> as *const (), 0), // +0x486ec0 sub_486ec0
    Lifted::new(0x7000b73030, tpl_923::<0x70017feac8> as *const (), 1), // +0x487030 sub_487030
    Lifted::new(0x7000b73050, tpl_753::<0x70017fea88, 0x8> as *const (), 1), // +0x487050 sub_487050
    Lifted::new(0x7000b73070, tpl_753::<0x70017fea48, 0x8> as *const (), 1), // +0x487070 sub_487070
    Lifted::new(0x7000b73090, tpl_754::<0x700177c312> as *const (), 0), // +0x487090 sub_487090
    Lifted::new(0x7000b730b0, tpl_754::<0x700177c312> as *const (), 0), // +0x4870b0 sub_4870b0
    Lifted::new(0x7000b730d0, tpl_754::<0x700177c312> as *const (), 0), // +0x4870d0 sub_4870d0
    Lifted::new(0x7000b730f0, fn_7000b730f0 as *const (), 1), // +0x4870f0 sub_4870f0
    Lifted::new(0x7000b73110, fn_7000b73110 as *const (), 0), // +0x487110 sub_487110
    Lifted::new(0x7000b73150, fn_7000b73150 as *const (), 0), // +0x487150 sub_487150
    Lifted::new(0x7000b731b0, tpl_286::<0x7000b72ec0, 0x70017d37a0, 0x70017790e0, 0x7000b7339b> as *const (), 0), // +0x4871b0 sub_4871b0
    Lifted::new(0x7000b73420, tpl_52::<0x8> as *const (), 1), // +0x487420 sub_487420
    Lifted::new(0x7000b73430, tpl_52::<0x8> as *const (), 1), // +0x487430 sub_487430
    Lifted::new(0x7000b73440, tpl_52::<0x8> as *const (), 1), // +0x487440 sub_487440
    Lifted::new(0x7000b73450, tpl_923::<0x70017feac8> as *const (), 1), // +0x487450 sub_487450
    Lifted::new(0x7000b73470, tpl_753::<0x70017fea88, 0x8> as *const (), 1), // +0x487470 sub_487470
    Lifted::new(0x7000b73490, tpl_753::<0x70017fea48, 0x8> as *const (), 1), // +0x487490 sub_487490
    Lifted::new(0x7000b734b0, tpl_315::<0x7001c6b470> as *const (), 1), // +0x4874b0 sub_4874b0
    Lifted::new(0x7000b734c0, tpl_315::<0x7001c6b4c0> as *const (), 1), // +0x4874c0 sub_4874c0
    Lifted::new(0x7000b734d0, tpl_315::<0x7001c6b510> as *const (), 1), // +0x4874d0 sub_4874d0
    Lifted::new(0x7000b734e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b734f0> as *const (), 0), // +0x4874e0 sub_4874e0
    Lifted::new(0x7000b73500, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b73510> as *const (), 0), // +0x487500 sub_487500
    Lifted::new(0x7000b73520, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b73530> as *const (), 0), // +0x487520 sub_487520
    Lifted::new(0x7000b73540, tpl_298::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b73594, 0x70007f0910, 0x7000b735af> as *const (), 0), // +0x487540 sub_487540
    Lifted::new(0x7000b735d0, fn_7000b735d0 as *const (), 1), // +0x4875d0 sub_4875d0
    Lifted::new(0x7000b735f0, fn_7000b735f0 as *const (), 0), // +0x4875f0 sub_4875f0
    Lifted::new(0x7000b73690, fn_7000b73690 as *const (), 1), // +0x487690 sub_487690
    Lifted::new(0x7000b736c0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4876c0 sub_4876c0
    Lifted::new(0x7000b73700, fn_7000b73700 as *const (), 2), // +0x487700 sub_487700
    Lifted::new(0x7000b738e0, tpl_924::<0x7001779e20, 0x70006f2a70, 0x70006f9a60, 0x700177c312, 0x7001d54968, 0x7000b73af6, 0x7000b4f010, 0x7000b73afc, 0x70007f0910, 0x7000b73b02> as *const (), 0), // +0x4878e0 sub_4878e0
    Lifted::new(0x7000b73b90, tpl_924::<0x7001779e20, 0x70006f2a70, 0x70006f9a60, 0x700177c312, 0x7001d54968, 0x7000b73da6, 0x7000b7e830, 0x7000b73dac, 0x70007f0910, 0x7000b73db2> as *const (), 0), // +0x487b90 sub_487b90
    Lifted::new(0x7000b73e40, tpl_925::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000b73fcb, 0x7000b7e7f0, 0x7000b73fd1, 0x70007f0910, 0x7000b73fd7> as *const (), 2), // +0x487e40 sub_487e40
    Lifted::new(0x7000b74040, tpl_64::<0x19, 0x10, 0x7000b74040, 0x700177c312> as *const (), 0), // +0x488040 sub_488040
    Lifted::new(0x7000b740b0, tpl_64::<0x19, 0x10, 0x7000b740b0, 0x700177c312> as *const (), 0), // +0x4880b0 sub_4880b0
    Lifted::new(0x7000b74120, tpl_63::<0x19, 0x10, 0x7000b74120, 0x700177c312> as *const (), 0), // +0x488120 sub_488120
    Lifted::new(0x7000b74190, fn_7000b74190 as *const (), 0), // +0x488190 sub_488190
    Lifted::new(0x7000b74340, fn_7000b74340 as *const (), 2), // +0x488340 sub_488340
    Lifted::new(0x7000b743e0, fn_7000b743e0 as *const (), 2), // +0x4883e0 sub_4883e0
    Lifted::new(0x7000b746c0, fn_7000b746c0 as *const (), 2), // +0x4886c0 sub_4886c0
    Lifted::new(0x7000b74810, fn_7000b74810 as *const (), 2), // +0x488810 sub_488810
    Lifted::new(0x7000b74be0, tpl_65::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000b7d1b0, 0x70017d37c0, 0x70017790e0, 0x7000b74eca> as *const (), 2), // +0x488be0 sub_488be0
    Lifted::new(0x7000b74f90, tpl_926::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000b7d690, 0x70017d37c0, 0x70017790e0, 0x7000b7527a> as *const (), 2), // +0x488f90 sub_488f90
    Lifted::new(0x7000b75340, tpl_927::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000b7cf80, 0x70017d37c0, 0x70017790e0, 0x7000b755f9> as *const (), 2), // +0x489340 sub_489340
    Lifted::new(0x7000b756b0, fn_7000b756b0 as *const (), 2), // +0x4896b0 sub_4896b0
    Lifted::new(0x7000b75a70, tpl_68 as *const (), 1), // +0x489a70 sub_489a70
    Lifted::new(0x7000b75b20, tpl_68 as *const (), 1), // +0x489b20 sub_489b20
    Lifted::new(0x7000b75bd0, tpl_68 as *const (), 1), // +0x489bd0 sub_489bd0
    Lifted::new(0x7000b75c80, tpl_68 as *const (), 1), // +0x489c80 sub_489c80
    Lifted::new(0x7000b75d30, tpl_896::<0x1, 0x30, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x18, 0x10> as *const (), 1), // +0x489d30 sub_489d30
    Lifted::new(0x7000b75e40, tpl_928::<0x7000a627f0> as *const (), 0), // +0x489e40 sub_489e40
    Lifted::new(0x7000b75f00, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x7000b7cb30> as *const (), 0), // +0x489f00 sub_489f00
    Lifted::new(0x7000b75f90, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000b76033> as *const (), 0), // +0x489f90 sub_489f90
    Lifted::new(0x7000b76060, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000b76103> as *const (), 0), // +0x48a060 sub_48a060
    Lifted::new(0x7000b76130, tpl_98::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000b761d3> as *const (), 0), // +0x48a130 sub_48a130
    Lifted::new(0x7000b76200, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000b762a3> as *const (), 0), // +0x48a200 sub_48a200
    Lifted::new(0x7000b762d0, tpl_929::<0x8, 0x19, 0x10, 0x7000b74040, 0x700177c312> as *const (), 0), // +0x48a2d0 sub_48a2d0
    Lifted::new(0x7000b76350, tpl_103::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b763ab> as *const (), 0), // +0x48a350 sub_48a350
    Lifted::new(0x7000b763d0, tpl_101::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b7642b> as *const (), 0), // +0x48a3d0 sub_48a3d0
    Lifted::new(0x7000b76450, tpl_409::<0x28, 0x38, 0xfffffffffffffff8, 0x700177c312, 0x30, 0x10, 0x20, 0x18, 0x7001d54968, 0x7000b764fc> as *const (), 0), // +0x48a450 sub_48a450
    Lifted::new(0x7000b76530, fn_7000b76530 as *const (), 0), // +0x48a530 sub_48a530
    Lifted::new(0x7000b76590, fn_7000b76590 as *const (), 0), // +0x48a590 sub_48a590
    Lifted::new(0x7000b76600, tpl_33::<0x700177c312> as *const (), 0), // +0x48a600 sub_48a600
    Lifted::new(0x7000b76630, tpl_33::<0x700177c312> as *const (), 0), // +0x48a630 sub_48a630
    Lifted::new(0x7000b76660, tpl_33::<0x700177c312> as *const (), 0), // +0x48a660 sub_48a660
    Lifted::new(0x7000b76690, tpl_33::<0x700177c312> as *const (), 0), // +0x48a690 sub_48a690
    Lifted::new(0x7000b766c0, tpl_33::<0x700177c312> as *const (), 0), // +0x48a6c0 sub_48a6c0
    Lifted::new(0x7000b766f0, tpl_33::<0x700177c312> as *const (), 0), // +0x48a6f0 sub_48a6f0
    Lifted::new(0x7000b76720, tpl_33::<0x700177c312> as *const (), 0), // +0x48a720 sub_48a720
    Lifted::new(0x7000b76750, tpl_33::<0x700177c312> as *const (), 0), // +0x48a750 sub_48a750
    Lifted::new(0x7000b76780, fn_7000b76780 as *const (), 0), // +0x48a780 sub_48a780
    Lifted::new(0x7000b76a60, fn_7000b76a60 as *const (), 0), // +0x48aa60 sub_48aa60
    Lifted::new(0x7000b76af0, fn_7000b76af0 as *const (), 2), // +0x48aaf0 sub_48aaf0
    Lifted::new(0x7000b76c70, fn_7000b76c70 as *const (), 0), // +0x48ac70 sub_48ac70
    Lifted::new(0x7000b76d80, fn_7000b76d80 as *const (), 2), // +0x48ad80 sub_48ad80
    Lifted::new(0x7000b778e0, fn_7000b778e0 as *const (), 0), // +0x48b8e0 sub_48b8e0
    Lifted::new(0x7000b779f0, fn_7000b779f0 as *const (), 2), // +0x48b9f0 sub_48b9f0
    Lifted::new(0x7000b77e40, fn_7000b77e40 as *const (), 0), // +0x48be40 sub_48be40
    Lifted::new(0x7000b77fe0, fn_7000b77fe0 as *const (), 0), // +0x48bfe0 sub_48bfe0
    Lifted::new(0x7000b78280, fn_7000b78280 as *const (), 0), // +0x48c280 sub_48c280
    Lifted::new(0x7000b783c0, fn_7000b783c0 as *const (), 0), // +0x48c3c0 sub_48c3c0
    Lifted::new(0x7000b789e0, fn_7000b789e0 as *const (), 2), // +0x48c9e0 sub_48c9e0
    Lifted::new(0x7000b78bd0, tpl_149::<0x8, 0xffffffffffffffff> as *const (), 1), // +0x48cbd0 sub_48cbd0
    Lifted::new(0x7000b78c10, tpl_39::<0x40> as *const (), 1), // +0x48cc10 sub_48cc10
    Lifted::new(0x7000b78c20, tpl_39::<0x14> as *const (), 1), // +0x48cc20 sub_48cc20
    Lifted::new(0x7000b78c30, tpl_39::<0x44> as *const (), 1), // +0x48cc30 sub_48cc30
    Lifted::new(0x7000b78c40, tpl_39::<0x24> as *const (), 1), // +0x48cc40 sub_48cc40
    Lifted::new(0x7000b78c50, fn_7000b78c50 as *const (), 1), // +0x48cc50 sub_48cc50
    Lifted::new(0x7000b78cf0, fn_7000b78cf0 as *const (), 2), // +0x48ccf0 sub_48ccf0
    Lifted::new(0x7000b78fc0, fn_7000b78fc0 as *const (), 0), // +0x48cfc0 sub_48cfc0
    Lifted::new(0x7000b79130, fn_7000b79130 as *const (), 1), // +0x48d130 sub_48d130
    Lifted::new(0x7000b791b0, fn_7000b791b0 as *const (), 0), // +0x48d1b0 sub_48d1b0
    Lifted::new(0x7000b79230, fn_7000b79230 as *const (), 0), // +0x48d230 sub_48d230
    Lifted::new(0x7000b79390, fn_7000b79390 as *const (), 0), // +0x48d390 sub_48d390
    Lifted::new(0x7000b79580, fn_7000b79580 as *const (), 2), // +0x48d580 sub_48d580
    Lifted::new(0x7000b7b0b0, fn_7000b7b0b0 as *const (), 0), // +0x48f0b0 sub_48f0b0
    Lifted::new(0x7000b7b2a0, tpl_930::<0x38, 0x40> as *const (), 1), // +0x48f2a0 sub_48f2a0
    Lifted::new(0x7000b7b2b0, tpl_245::<0x40> as *const (), 1), // +0x48f2b0 sub_48f2b0
    Lifted::new(0x7000b7b2c0, tpl_930::<0x3c, 0x44> as *const (), 1), // +0x48f2c0 sub_48f2c0
    Lifted::new(0x7000b7b2d0, tpl_245::<0x44> as *const (), 1), // +0x48f2d0 sub_48f2d0
    Lifted::new(0x7000b7b2e0, fn_7000b7b2e0 as *const (), 1), // +0x48f2e0 sub_48f2e0
    Lifted::new(0x7000b7b380, fn_7000b7b380 as *const (), 0), // +0x48f380 sub_48f380
    Lifted::new(0x7000b7b710, fn_7000b7b710 as *const (), 0), // +0x48f710 sub_48f710
    Lifted::new(0x7000b7b9b0, tpl_552::<0x8, 0x700077aad0, 0x7000b7ba10> as *const (), 0), // +0x48f9b0 sub_48f9b0
    Lifted::new(0x7000b7ba10, fn_7000b7ba10 as *const (), 0), // +0x48fa10 sub_48fa10
    Lifted::new(0x7000b7c680, fn_7000b7c680 as *const (), 2), // +0x490680 sub_490680
    Lifted::new(0x7000b7c9c0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b7cac7, 0x7000b75a70, 0x70007f0910, 0x7000b7cae3> as *const (), 0), // +0x4909c0 sub_4909c0
    Lifted::new(0x7000b7cb30, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b7cc37, 0x7000b75b20, 0x70007f0910, 0x7000b7cc53> as *const (), 0), // +0x490b30 sub_490b30
    Lifted::new(0x7000b7cca0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b7cda7, 0x7000b75bd0, 0x70007f0910, 0x7000b7cdc3> as *const (), 0), // +0x490ca0 sub_490ca0
    Lifted::new(0x7000b7ce10, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b7cf17, 0x7000b75c80, 0x70007f0910, 0x7000b7cf33> as *const (), 0), // +0x490e10 sub_490e10
    Lifted::new(0x7000b7cf80, tpl_284::<0x7000b7c9c0, 0x70017d37a0, 0x70017790e0, 0x7000b7d13d> as *const (), 0), // +0x490f80 sub_490f80
    Lifted::new(0x7000b7d1b0, tpl_286::<0x7000b7cb30, 0x70017d37a0, 0x70017790e0, 0x7000b7d39b> as *const (), 0), // +0x4911b0 sub_4911b0
    Lifted::new(0x7000b7d420, tpl_286::<0x7000b7cca0, 0x70017d37a0, 0x70017790e0, 0x7000b7d60b> as *const (), 0), // +0x491420 sub_491420
    Lifted::new(0x7000b7d690, tpl_286::<0x7000b7ce10, 0x70017d37a0, 0x70017790e0, 0x7000b7d87b> as *const (), 0), // +0x491690 sub_491690
    Lifted::new(0x7000b7d900, tpl_931::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000b7dac5, 0x70007f0910, 0x7000b7dacb, 0x7000b7e7d0, 0x7000b7dad4> as *const (), 0), // +0x491900 sub_491900
    Lifted::new(0x7000b7db50, tpl_287 as *const (), 1), // +0x491b50 sub_491b50
    Lifted::new(0x7000b7de70, tpl_287 as *const (), 1), // +0x491e70 sub_491e70
    Lifted::new(0x7000b7e190, tpl_287 as *const (), 1), // +0x492190 sub_492190
    Lifted::new(0x7000b7e4b0, tpl_287 as *const (), 1), // +0x4924b0 sub_4924b0
    Lifted::new(0x7000b7e7d0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000b7e7e0> as *const (), 0), // +0x4927d0 sub_4927d0
    Lifted::new(0x7000b7e7f0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b7e800> as *const (), 0), // +0x4927f0 sub_4927f0
    Lifted::new(0x7000b7e810, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b7e820> as *const (), 0), // +0x492810 sub_492810
    Lifted::new(0x7000b7e830, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b7e840> as *const (), 0), // +0x492830 sub_492830
    Lifted::new(0x7000b7e850, tpl_932::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b7e8a8, 0x70007f0910, 0x7000b7e8c3> as *const (), 0), // +0x492850 sub_492850
    Lifted::new(0x7000b7e8f0, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b7e93d, 0x70007f0910, 0x7000b7e95b> as *const (), 0), // +0x4928f0 sub_4928f0
    Lifted::new(0x7000b7e980, tpl_334::<0x7> as *const (), 1), // +0x492980 sub_492980
    Lifted::new(0x7000b7e9b0, tpl_933::<0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000b7e9f2> as *const (), 0), // +0x4929b0 sub_4929b0
    Lifted::new(0x7000b7ea10, tpl_933::<0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000b7ea52> as *const (), 0), // +0x492a10 sub_492a10
    Lifted::new(0x7000b7ea70, fn_7000b7ea70 as *const (), 1), // +0x492a70 sub_492a70
    Lifted::new(0x7000b7ead0, tpl_149::<0x8, 0xffffffffffffffff> as *const (), 1), // +0x492ad0 sub_492ad0
    Lifted::new(0x7000b7eb10, tpl_934::<0x7> as *const (), 1), // +0x492b10 sub_492b10
    Lifted::new(0x7000b7eb20, fn_7000b7eb20 as *const (), 1), // +0x492b20 sub_492b20
    Lifted::new(0x7000b7ebb0, tpl_333::<0xffffffffffffffff> as *const (), 1), // +0x492bb0 sub_492bb0
    Lifted::new(0x7000b7ebd0, fn_7000b7ebd0 as *const (), 0), // +0x492bd0 sub_492bd0
    Lifted::new(0x7000b7ec20, tpl_935::<0x1f> as *const (), 1), // +0x492c20 sub_492c20
    Lifted::new(0x7000b7ec40, tpl_377::<0x13, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x492c40 sub_492c40
    Lifted::new(0x7000b7ecf0, fn_7000b7ecf0 as *const (), 0), // +0x492cf0 sub_492cf0
    Lifted::new(0x7000b7ee50, tpl_377::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x492e50 sub_492e50
    Lifted::new(0x7000b7ef00, fn_7000b7ef00 as *const (), 2), // +0x492f00 sub_492f00
    Lifted::new(0x7000b7f040, tpl_237::<0xb> as *const (), 1), // +0x493040 sub_493040
    Lifted::new(0x7000b7f070, fn_7000b7f070 as *const (), 1), // +0x493070 sub_493070
    Lifted::new(0x7000b7f0d0, fn_7000b7f0d0 as *const (), 0), // +0x4930d0 sub_4930d0
    Lifted::new(0x7000b7f660, tpl_189::<0x7001c5d1c0> as *const (), 1), // +0x493660 sub_493660
    Lifted::new(0x7000b7f680, tpl_181 as *const (), 1), // +0x493680 sub_493680
    Lifted::new(0x7000b7f690, tpl_936::<0x7001779e20, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000b7f7ff, 0x7000b7fc90, 0x7000b7f805, 0x70007f0910, 0x7000b7f80b> as *const (), 0), // +0x493690 sub_493690
    Lifted::new(0x7000b7f870, fn_7000b7f870 as *const (), 0), // +0x493870 sub_493870
    Lifted::new(0x7000b7fc90, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b7fca0> as *const (), 0), // +0x493c90 sub_493c90
    Lifted::new(0x7000b7fcb0, fn_7000b7fcb0 as *const (), 0), // +0x493cb0 sub_493cb0
    Lifted::new(0x7000b7fda0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x493da0 sub_493da0
    Lifted::new(0x7000b7fde0, fn_7000b7fde0 as *const (), 0), // +0x493de0 sub_493de0
    Lifted::new(0x7000b800f0, tpl_64::<0x19, 0x10, 0x7000b800f0, 0x700177c312> as *const (), 0), // +0x4940f0 sub_4940f0
    Lifted::new(0x7000b80160, fn_7000b80160 as *const (), 0), // +0x494160 sub_494160
    Lifted::new(0x7000b80330, tpl_898::<0x700177c0e0> as *const (), 0), // +0x494330 sub_494330
    Lifted::new(0x7000b80370, tpl_392::<0x700177c0e0> as *const (), 0), // +0x494370 sub_494370
    Lifted::new(0x7000b803b0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4943b0 sub_4943b0
    Lifted::new(0x7000b803f0, fn_7000b803f0 as *const (), 0), // +0x4943f0 sub_4943f0
    Lifted::new(0x7000b809f0, fn_7000b809f0 as *const (), 1), // +0x4949f0 sub_4949f0
    Lifted::new(0x7000b80b40, tpl_929::<0x8, 0x19, 0x10, 0x7000b800f0, 0x700177c312> as *const (), 0), // +0x494b40 sub_494b40
    Lifted::new(0x7000b80bc0, tpl_603::<0x20, 0xffffffffffffffff, 0x18, 0x10, 0x8, 0x700177c312, 0xfffffffffffffff8, 0x7001d54968, 0x7000b80c83> as *const (), 0), // +0x494bc0 sub_494bc0
    Lifted::new(0x7000b80cc0, tpl_937::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000b80d3a> as *const (), 0), // +0x494cc0 sub_494cc0
    Lifted::new(0x7000b80d60, fn_7000b80d60 as *const (), 0), // +0x494d60 sub_494d60
    Lifted::new(0x7000b80e40, fn_7000b80e40 as *const (), 0), // +0x494e40 sub_494e40
    Lifted::new(0x7000b80ef0, tpl_33::<0x700177c312> as *const (), 0), // +0x494ef0 sub_494ef0
    Lifted::new(0x7000b80f90, fn_7000b80f90 as *const (), 0), // +0x494f90 sub_494f90
    Lifted::new(0x7000b81040, fn_7000b81040 as *const (), 0), // +0x495040 sub_495040
    Lifted::new(0x7000b81120, fn_7000b81120 as *const (), 0), // +0x495120 sub_495120
    Lifted::new(0x7000b81270, fn_7000b81270 as *const (), 0), // +0x495270 sub_495270
    Lifted::new(0x7000b81310, fn_7000b81310 as *const (), 2), // +0x495310 sub_495310
    Lifted::new(0x7000b81740, fn_7000b81740 as *const (), 2), // +0x495740 sub_495740
    Lifted::new(0x7000b81b60, fn_7000b81b60 as *const (), 2), // +0x495b60 sub_495b60
    Lifted::new(0x7000b81ed0, fn_7000b81ed0 as *const (), 2), // +0x495ed0 sub_495ed0
    Lifted::new(0x7000b825b0, fn_7000b825b0 as *const (), 0), // +0x4965b0 sub_4965b0
    Lifted::new(0x7000b82880, fn_7000b82880 as *const (), 2), // +0x496880 sub_496880
    Lifted::new(0x7000b83ce0, fn_7000b83ce0 as *const (), 0), // +0x497ce0 sub_497ce0
    Lifted::new(0x7000b846c0, fn_7000b846c0 as *const (), 2), // +0x4986c0 sub_4986c0
    Lifted::new(0x7000b84e30, fn_7000b84e30 as *const (), 0), // +0x498e30 sub_498e30
    Lifted::new(0x7000b85dc0, fn_7000b85dc0 as *const (), 0), // +0x499dc0 sub_499dc0
    Lifted::new(0x7000b85f60, fn_7000b85f60 as *const (), 0), // +0x499f60 sub_499f60
    Lifted::new(0x7000b85fd0, fn_7000b85fd0 as *const (), 0), // +0x499fd0 sub_499fd0
    Lifted::new(0x7000b861e0, fn_7000b861e0 as *const (), 0), // +0x49a1e0 sub_49a1e0
    Lifted::new(0x7000b86220, fn_7000b86220 as *const (), 0), // +0x49a220 sub_49a220
    Lifted::new(0x7000b86290, fn_7000b86290 as *const (), 1), // +0x49a290 sub_49a290
    Lifted::new(0x7000b86360, fn_7000b86360 as *const (), 0), // +0x49a360 sub_49a360
    Lifted::new(0x7000b86430, fn_7000b86430 as *const (), 0), // +0x49a430 sub_49a430
    Lifted::new(0x7000b864a0, fn_7000b864a0 as *const (), 0), // +0x49a4a0 sub_49a4a0
    Lifted::new(0x7000b86550, fn_7000b86550 as *const (), 0), // +0x49a550 sub_49a550
    Lifted::new(0x7000b86800, fn_7000b86800 as *const (), 1), // +0x49a800 sub_49a800
    Lifted::new(0x7000b86870, fn_7000b86870 as *const (), 0), // +0x49a870 sub_49a870
    Lifted::new(0x7000b86920, tpl_938::<0x18, 0x8, 0x7001319ba0, 0x10, 0x70017ff940, 0x110, 0x70017ff8d8, 0x90, 0x7000790b70> as *const (), 0), // +0x49a920 sub_49a920
    Lifted::new(0x7000b869b0, fn_7000b869b0 as *const (), 0), // +0x49a9b0 sub_49a9b0
    Lifted::new(0x7000b86a30, tpl_938::<0x18, 0x8, 0x7001319b90, 0x10, 0x70017ff940, 0x110, 0x70017ff8e8, 0x90, 0x7000790b70> as *const (), 0), // +0x49aa30 sub_49aa30
    Lifted::new(0x7000b86ac0, fn_7000b86ac0 as *const (), 0), // +0x49aac0 sub_49aac0
    Lifted::new(0x7000b86bc0, fn_7000b86bc0 as *const (), 0), // +0x49abc0 sub_49abc0
    Lifted::new(0x7000b87060, fn_7000b87060 as *const (), 0), // +0x49b060 sub_49b060
    Lifted::new(0x7000b871c0, fn_7000b871c0 as *const (), 0), // +0x49b1c0 sub_49b1c0
    Lifted::new(0x7000b87260, tpl_939::<0x7001c7223b, 0x7000771f10, 0x7001c72317, 0x90, 0x70017d95bc, 0x7001d55040, 0x70017d95c0, 0x70017a0290, 0x7001d54ef8, 0x7000b872c1, 0x98, 0x70017ffd48, 0x7000790b70, 0x7001d54ab0> as *const (), 0), // +0x49b260 sub_49b260
    Lifted::new(0x7000b87340, fn_7000b87340 as *const (), 0), // +0x49b340 sub_49b340
    Lifted::new(0x7000b87620, tpl_939::<0x7001c7223b, 0x7000771f10, 0x7001c72317, 0x90, 0x70017d95bc, 0x7001d55040, 0x70017d95c0, 0x70017a0290, 0x7001d54ef8, 0x7000b87681, 0x98, 0x70017ffd78, 0x7000790b70, 0x7001d54ab0> as *const (), 0), // +0x49b620 sub_49b620
    Lifted::new(0x7000b87700, fn_7000b87700 as *const (), 0), // +0x49b700 sub_49b700
    Lifted::new(0x7000b87830, fn_7000b87830 as *const (), 0), // +0x49b830 sub_49b830
    Lifted::new(0x7000b87b50, fn_7000b87b50 as *const (), 0), // +0x49bb50 sub_49bb50
    Lifted::new(0x7000b87ba0, fn_7000b87ba0 as *const (), 2), // +0x49bba0 sub_49bba0
    Lifted::new(0x7000b87ce0, fn_7000b87ce0 as *const (), 0), // +0x49bce0 sub_49bce0
    Lifted::new(0x7000b87e70, fn_7000b87e70 as *const (), 0), // +0x49be70 sub_49be70
    Lifted::new(0x7000b882f0, tpl_931::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000b884b5, 0x70007f0910, 0x7000b884bb, 0x7000b889c0, 0x7000b884c4> as *const (), 0), // +0x49c2f0 sub_49c2f0
    Lifted::new(0x7000b88540, tpl_287 as *const (), 1), // +0x49c540 sub_49c540
    Lifted::new(0x7000b88860, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x49c860 sub_49c860
    Lifted::new(0x7000b888d0, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x49c8d0 sub_49c8d0
    Lifted::new(0x7000b88940, fn_7000b88940 as *const (), 1), // +0x49c940 sub_49c940
    Lifted::new(0x7000b889c0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000b889d0> as *const (), 0), // +0x49c9c0 sub_49c9c0
    Lifted::new(0x7000b889e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b889f0> as *const (), 0), // +0x49c9e0 sub_49c9e0
    Lifted::new(0x7000b88a00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b88a10> as *const (), 0), // +0x49ca00 sub_49ca00
    Lifted::new(0x7000b88a20, tpl_300::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b88a6d, 0x70007f0910, 0x7000b88a8b> as *const (), 0), // +0x49ca20 sub_49ca20
    Lifted::new(0x7000b88ab0, fn_7000b88ab0 as *const (), 0), // +0x49cab0 sub_49cab0
    Lifted::new(0x7000b88b40, fn_7000b88b40 as *const (), 0), // +0x49cb40 sub_49cb40
    Lifted::new(0x7000b88e00, fn_7000b88e00 as *const (), 0), // +0x49ce00 sub_49ce00
    Lifted::new(0x7000b88e90, fn_7000b88e90 as *const (), 1), // +0x49ce90 sub_49ce90
    Lifted::new(0x7000b88ee0, fn_7000b88ee0 as *const (), 1), // +0x49cee0 sub_49cee0
    Lifted::new(0x7000b88f00, fn_7000b88f00 as *const (), 2), // +0x49cf00 sub_49cf00
    Lifted::new(0x7000b89290, fn_7000b89290 as *const (), 0), // +0x49d290 sub_49d290
    Lifted::new(0x7000b89440, fn_7000b89440 as *const (), 0), // +0x49d440 sub_49d440
    Lifted::new(0x7000b894d0, fn_7000b894d0 as *const (), 2), // +0x49d4d0 sub_49d4d0
    Lifted::new(0x7000b897a0, fn_7000b897a0 as *const (), 0), // +0x49d7a0 sub_49d7a0
    Lifted::new(0x7000b89920, tpl_940::<0x8, 0x4> as *const (), 0), // +0x49d920 sub_49d920
    Lifted::new(0x7000b89950, tpl_55::<0x700177c0d0> as *const (), 0), // +0x49d950 sub_49d950
    Lifted::new(0x7000b89990, tpl_55::<0x700177c0d0> as *const (), 0), // +0x49d990 sub_49d990
    Lifted::new(0x7000b899d0, fn_7000b899d0 as *const (), 2), // +0x49d9d0 sub_49d9d0
    Lifted::new(0x7000b89aa0, fn_7000b89aa0 as *const (), 2), // +0x49daa0 sub_49daa0
    Lifted::new(0x7000b89ce0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x49dce0 sub_49dce0
    Lifted::new(0x7000b89d20, tpl_898::<0x700177c0e0> as *const (), 0), // +0x49dd20 sub_49dd20
    Lifted::new(0x7000b89d60, tpl_392::<0x700177c0e0> as *const (), 0), // +0x49dd60 sub_49dd60
    Lifted::new(0x7000b89d90, tpl_392::<0x700177c0e0> as *const (), 0), // +0x49dd90 sub_49dd90
    Lifted::new(0x7000b89dc0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x49ddc0 sub_49ddc0
    Lifted::new(0x7000b89e00, tpl_55::<0x700177c0d0> as *const (), 0), // +0x49de00 sub_49de00
    Lifted::new(0x7000b89e40, fn_7000b89e40 as *const (), 0), // +0x49de40 sub_49de40
    Lifted::new(0x7000b89f70, fn_7000b89f70 as *const (), 0), // +0x49df70 sub_49df70
    Lifted::new(0x7000b8a180, fn_7000b8a180 as *const (), 1), // +0x49e180 sub_49e180
    Lifted::new(0x7000b8a1d0, fn_7000b8a1d0 as *const (), 0), // +0x49e1d0 sub_49e1d0
    Lifted::new(0x7000b8a240, fn_7000b8a240 as *const (), 0), // +0x49e240 sub_49e240
    Lifted::new(0x7000b8a2e0, fn_7000b8a2e0 as *const (), 0), // +0x49e2e0 sub_49e2e0
    Lifted::new(0x7000b8a370, fn_7000b8a370 as *const (), 0), // +0x49e370 sub_49e370
    Lifted::new(0x7000b8a410, fn_7000b8a410 as *const (), 0), // +0x49e410 sub_49e410
    Lifted::new(0x7000b8a4b0, fn_7000b8a4b0 as *const (), 2), // +0x49e4b0 sub_49e4b0
    Lifted::new(0x7000b8a9e0, fn_7000b8a9e0 as *const (), 0), // +0x49e9e0 sub_49e9e0
    Lifted::new(0x7000b8b300, fn_7000b8b300 as *const (), 2), // +0x49f300 sub_49f300
    Lifted::new(0x7000b8bf70, fn_7000b8bf70 as *const (), 0), // +0x49ff70 sub_49ff70
    Lifted::new(0x7000b8c000, fn_7000b8c000 as *const (), 0), // +0x4a0000 sub_4a0000
    Lifted::new(0x7000b8c8f0, fn_7000b8c8f0 as *const (), 0), // +0x4a08f0 sub_4a08f0
    Lifted::new(0x7000b8cbd0, fn_7000b8cbd0 as *const (), 0), // +0x4a0bd0 sub_4a0bd0
    Lifted::new(0x7000b8cf80, fn_7000b8cf80 as *const (), 2), // +0x4a0f80 sub_4a0f80
    Lifted::new(0x7000b8d220, fn_7000b8d220 as *const (), 0), // +0x4a1220 sub_4a1220
    Lifted::new(0x7000b8d580, fn_7000b8d580 as *const (), 0), // +0x4a1580 sub_4a1580
    Lifted::new(0x7000b8d780, fn_7000b8d780 as *const (), 1), // +0x4a1780 sub_4a1780
    Lifted::new(0x7000b8d800, fn_7000b8d800 as *const (), 2), // +0x4a1800 sub_4a1800
    Lifted::new(0x7000b8dad0, fn_7000b8dad0 as *const (), 0), // +0x4a1ad0 sub_4a1ad0
    Lifted::new(0x7000b8db30, fn_7000b8db30 as *const (), 0), // +0x4a1b30 sub_4a1b30
    Lifted::new(0x7000b8dc00, fn_7000b8dc00 as *const (), 0), // +0x4a1c00 sub_4a1c00
    Lifted::new(0x7000b8dc70, fn_7000b8dc70 as *const (), 2), // +0x4a1c70 sub_4a1c70
    Lifted::new(0x7000b8e030, fn_7000b8e030 as *const (), 0), // +0x4a2030 sub_4a2030
    Lifted::new(0x7000b8e120, fn_7000b8e120 as *const (), 0), // +0x4a2120 sub_4a2120
    Lifted::new(0x7000b8e1c0, fn_7000b8e1c0 as *const (), 0), // +0x4a21c0 sub_4a21c0
    Lifted::new(0x7000b8e270, fn_7000b8e270 as *const (), 0), // +0x4a2270 sub_4a2270
    Lifted::new(0x7000b8e400, fn_7000b8e400 as *const (), 0), // +0x4a2400 sub_4a2400
    Lifted::new(0x7000b8e4b0, fn_7000b8e4b0 as *const (), 0), // +0x4a24b0 sub_4a24b0
    Lifted::new(0x7000b8edb0, fn_7000b8edb0 as *const (), 0), // +0x4a2db0 sub_4a2db0
    Lifted::new(0x7000b8f350, fn_7000b8f350 as *const (), 2), // +0x4a3350 sub_4a3350
    Lifted::new(0x7000b8f7b0, fn_7000b8f7b0 as *const (), 2), // +0x4a37b0 sub_4a37b0
    Lifted::new(0x7000b8fa50, fn_7000b8fa50 as *const (), 2), // +0x4a3a50 sub_4a3a50
    Lifted::new(0x7000b8fd40, fn_7000b8fd40 as *const (), 1), // +0x4a3d40 sub_4a3d40
    Lifted::new(0x7000b8fd50, fn_7000b8fd50 as *const (), 1), // +0x4a3d50 sub_4a3d50
    Lifted::new(0x7000b8fd70, fn_7000b8fd70 as *const (), 0), // +0x4a3d70 sub_4a3d70
    Lifted::new(0x7000b8fe30, fn_7000b8fe30 as *const (), 0), // +0x4a3e30 sub_4a3e30
    Lifted::new(0x7000b8fed0, fn_7000b8fed0 as *const (), 0), // +0x4a3ed0 sub_4a3ed0
    Lifted::new(0x7000b8ff70, fn_7000b8ff70 as *const (), 2), // +0x4a3f70 sub_4a3f70
    Lifted::new(0x7000b901e0, fn_7000b901e0 as *const (), 0), // +0x4a41e0 sub_4a41e0
    Lifted::new(0x7000b90330, fn_7000b90330 as *const (), 0), // +0x4a4330 sub_4a4330
    Lifted::new(0x7000b90450, fn_7000b90450 as *const (), 1), // +0x4a4450 sub_4a4450
    Lifted::new(0x7000b904b0, fn_7000b904b0 as *const (), 1), // +0x4a44b0 sub_4a44b0
    Lifted::new(0x7000b904d0, fn_7000b904d0 as *const (), 1), // +0x4a44d0 sub_4a44d0
    Lifted::new(0x7000b90510, fn_7000b90510 as *const (), 1), // +0x4a4510 sub_4a4510
    Lifted::new(0x7000b90550, fn_7000b90550 as *const (), 1), // +0x4a4550 sub_4a4550
    Lifted::new(0x7000b90590, fn_7000b90590 as *const (), 0), // +0x4a4590 sub_4a4590
    Lifted::new(0x7000b907d0, fn_7000b907d0 as *const (), 0), // +0x4a47d0 sub_4a47d0
    Lifted::new(0x7000b908b0, fn_7000b908b0 as *const (), 0), // +0x4a48b0 sub_4a48b0
    Lifted::new(0x7000b90980, fn_7000b90980 as *const (), 0), // +0x4a4980 sub_4a4980
    Lifted::new(0x7000b90ab0, fn_7000b90ab0 as *const (), 2), // +0x4a4ab0 sub_4a4ab0
    Lifted::new(0x7000b911b0, fn_7000b911b0 as *const (), 0), // +0x4a51b0 sub_4a51b0
    Lifted::new(0x7000b92000, fn_7000b92000 as *const (), 0), // +0x4a6000 sub_4a6000
    Lifted::new(0x7000b920b0, fn_7000b920b0 as *const (), 0), // +0x4a60b0 sub_4a60b0
    Lifted::new(0x7000b92330, tpl_15 as *const (), 1), // +0x4a6330 sub_4a6330
    Lifted::new(0x7000b92340, fn_7000b92340 as *const (), 0), // +0x4a6340 sub_4a6340
    Lifted::new(0x7000b92470, fn_7000b92470 as *const (), 0), // +0x4a6470 sub_4a6470
    Lifted::new(0x7000b926c0, tpl_900::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000b92888, 0x70007f0910, 0x7000b9288e, 0x7000b92a20, 0x7000b92897> as *const (), 0), // +0x4a66c0 sub_4a66c0
    Lifted::new(0x7000b92910, fn_7000b92910 as *const (), 0), // +0x4a6910 sub_4a6910
    Lifted::new(0x7000b92a00, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000b92a10> as *const (), 0), // +0x4a6a00 sub_4a6a00
    Lifted::new(0x7000b92a20, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000b92a30> as *const (), 0), // +0x4a6a20 sub_4a6a20
    Lifted::new(0x7000b92a40, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b92a50> as *const (), 0), // +0x4a6a40 sub_4a6a40
    Lifted::new(0x7000b92a60, fn_7000b92a60 as *const (), 0), // +0x4a6a60 sub_4a6a60
    Lifted::new(0x7000b92b60, fn_7000b92b60 as *const (), 0), // +0x4a6b60 sub_4a6b60
    Lifted::new(0x7000b92c90, fn_7000b92c90 as *const (), 0), // +0x4a6c90 sub_4a6c90
    Lifted::new(0x7000b92ef0, tpl_941::<0x700076bf20, 0x7000b95540, 0x7000b93051> as *const (), 0), // +0x4a6ef0 sub_4a6ef0
    Lifted::new(0x7000b930b0, fn_7000b930b0 as *const (), 0), // +0x4a70b0 sub_4a70b0
    Lifted::new(0x7000b93180, fn_7000b93180 as *const (), 0), // +0x4a7180 sub_4a7180
    Lifted::new(0x7000b93260, fn_7000b93260 as *const (), 0), // +0x4a7260 sub_4a7260
    Lifted::new(0x7000b93420, fn_7000b93420 as *const (), 0), // +0x4a7420 sub_4a7420
    Lifted::new(0x7000b93490, fn_7000b93490 as *const (), 0), // +0x4a7490 sub_4a7490
    Lifted::new(0x7000b935b0, fn_7000b935b0 as *const (), 0), // +0x4a75b0 sub_4a75b0
    Lifted::new(0x7000b93670, fn_7000b93670 as *const (), 0), // +0x4a7670 sub_4a7670
    Lifted::new(0x7000b936d0, fn_7000b936d0 as *const (), 0), // +0x4a76d0 sub_4a76d0
    Lifted::new(0x7000b937f0, fn_7000b937f0 as *const (), 0), // +0x4a77f0 sub_4a77f0
    Lifted::new(0x7000b93970, fn_7000b93970 as *const (), 0), // +0x4a7970 sub_4a7970
    Lifted::new(0x7000b93a50, fn_7000b93a50 as *const (), 0), // +0x4a7a50 sub_4a7a50
    Lifted::new(0x7000b93bd0, fn_7000b93bd0 as *const (), 0), // +0x4a7bd0 sub_4a7bd0
    Lifted::new(0x7000b93df0, fn_7000b93df0 as *const (), 0), // +0x4a7df0 sub_4a7df0
    Lifted::new(0x7000b93fe0, fn_7000b93fe0 as *const (), 0), // +0x4a7fe0 sub_4a7fe0
    Lifted::new(0x7000b94160, fn_7000b94160 as *const (), 0), // +0x4a8160 sub_4a8160
    Lifted::new(0x7000b942e0, fn_7000b942e0 as *const (), 0), // +0x4a82e0 sub_4a82e0
    Lifted::new(0x7000b943c0, fn_7000b943c0 as *const (), 1), // +0x4a83c0 sub_4a83c0
    Lifted::new(0x7000b94400, fn_7000b94400 as *const (), 1), // +0x4a8400 sub_4a8400
    Lifted::new(0x7000b944b0, fn_7000b944b0 as *const (), 0), // +0x4a84b0 sub_4a84b0
    Lifted::new(0x7000b94590, fn_7000b94590 as *const (), 0), // +0x4a8590 sub_4a8590
    Lifted::new(0x7000b94670, fn_7000b94670 as *const (), 0), // +0x4a8670 sub_4a8670
    Lifted::new(0x7000b94750, fn_7000b94750 as *const (), 0), // +0x4a8750 sub_4a8750
    Lifted::new(0x7000b94830, fn_7000b94830 as *const (), 0), // +0x4a8830 sub_4a8830
    Lifted::new(0x7000b94910, fn_7000b94910 as *const (), 0), // +0x4a8910 sub_4a8910
    Lifted::new(0x7000b949f0, fn_7000b949f0 as *const (), 0), // +0x4a89f0 sub_4a89f0
    Lifted::new(0x7000b94ad0, fn_7000b94ad0 as *const (), 0), // +0x4a8ad0 sub_4a8ad0
    Lifted::new(0x7000b94bb0, fn_7000b94bb0 as *const (), 0), // +0x4a8bb0 sub_4a8bb0
    Lifted::new(0x7000b94c90, fn_7000b94c90 as *const (), 0), // +0x4a8c90 sub_4a8c90
    Lifted::new(0x7000b94d70, fn_7000b94d70 as *const (), 0), // +0x4a8d70 sub_4a8d70
    Lifted::new(0x7000b94e20, fn_7000b94e20 as *const (), 0), // +0x4a8e20 sub_4a8e20
    Lifted::new(0x7000b94f00, fn_7000b94f00 as *const (), 0), // +0x4a8f00 sub_4a8f00
    Lifted::new(0x7000b94fe0, fn_7000b94fe0 as *const (), 0), // +0x4a8fe0 sub_4a8fe0
    Lifted::new(0x7000b950c0, fn_7000b950c0 as *const (), 0), // +0x4a90c0 sub_4a90c0
    Lifted::new(0x7000b951a0, fn_7000b951a0 as *const (), 0), // +0x4a91a0 sub_4a91a0
    Lifted::new(0x7000b95280, fn_7000b95280 as *const (), 0), // +0x4a9280 sub_4a9280
    Lifted::new(0x7000b954d0, tpl_942::<0x700177c0e0> as *const (), 0), // +0x4a94d0 sub_4a94d0
    Lifted::new(0x7000b95540, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b95550> as *const (), 0), // +0x4a9540 sub_4a9540
    Lifted::new(0x7000b95560, fn_7000b95560 as *const (), 0), // +0x4a9560 sub_4a9560
    Lifted::new(0x7000b95910, tpl_943::<0x30, 0x8, 0x18, 0x10, 0x28, 0x20, 0x700177c0c0> as *const (), 0), // +0x4a9910 sub_4a9910
    Lifted::new(0x7000b95a10, fn_7000b95a10 as *const (), 0), // +0x4a9a10 sub_4a9a10
    Lifted::new(0x7000b95b30, fn_7000b95b30 as *const (), 3), // +0x4a9b30 sub_4a9b30
    Lifted::new(0x7000b95d40, fn_7000b95d40 as *const (), 3), // +0x4a9d40 sub_4a9d40
    Lifted::new(0x7000b95e90, fn_7000b95e90 as *const (), 2), // +0x4a9e90 sub_4a9e90
    Lifted::new(0x7000b963e0, fn_7000b963e0 as *const (), 3), // +0x4aa3e0 sub_4aa3e0
    Lifted::new(0x7000b965a0, fn_7000b965a0 as *const (), 0), // +0x4aa5a0 sub_4aa5a0
    Lifted::new(0x7000b96790, fn_7000b96790 as *const (), 2), // +0x4aa790 sub_4aa790
    Lifted::new(0x7000b96a80, fn_7000b96a80 as *const (), 2), // +0x4aaa80 sub_4aaa80
    Lifted::new(0x7000b96e00, tpl_67::<0x7000b73540, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000b96f43, 0x7000b5ae00, 0x7000b96f49> as *const (), 0), // +0x4aae00 sub_4aae00
    Lifted::new(0x7000b96fa0, fn_7000b96fa0 as *const (), 0), // +0x4aafa0 sub_4aafa0
    Lifted::new(0x7000b96ff0, tpl_68 as *const (), 1), // +0x4aaff0 sub_4aaff0
    Lifted::new(0x7000b970a0, fn_7000b970a0 as *const (), 0), // +0x4ab0a0 sub_4ab0a0
    Lifted::new(0x7000b97290, fn_7000b97290 as *const (), 0), // +0x4ab290 sub_4ab290
    Lifted::new(0x7000b97320, fn_7000b97320 as *const (), 0), // +0x4ab320 sub_4ab320
    Lifted::new(0x7000b973b0, fn_7000b973b0 as *const (), 0), // +0x4ab3b0 sub_4ab3b0
    Lifted::new(0x7000b97530, fn_7000b97530 as *const (), 0), // +0x4ab530 sub_4ab530
    Lifted::new(0x7000b97730, fn_7000b97730 as *const (), 2), // +0x4ab730 sub_4ab730
    Lifted::new(0x7000b97c80, fn_7000b97c80 as *const (), 0), // +0x4abc80 sub_4abc80
    Lifted::new(0x7000b97f40, fn_7000b97f40 as *const (), 0), // +0x4abf40 sub_4abf40
    Lifted::new(0x7000b982d0, fn_7000b982d0 as *const (), 0), // +0x4ac2d0 sub_4ac2d0
    Lifted::new(0x7000b98380, fn_7000b98380 as *const (), 2), // +0x4ac380 sub_4ac380
    Lifted::new(0x7000b98740, fn_7000b98740 as *const (), 1), // +0x4ac740 sub_4ac740
    Lifted::new(0x7000b98770, fn_7000b98770 as *const (), 0), // +0x4ac770 sub_4ac770
    Lifted::new(0x7000b98870, fn_7000b98870 as *const (), 0), // +0x4ac870 sub_4ac870
    Lifted::new(0x7000b98950, fn_7000b98950 as *const (), 2), // +0x4ac950 sub_4ac950
    Lifted::new(0x7000b994c0, fn_7000b994c0 as *const (), 0), // +0x4ad4c0 sub_4ad4c0
    Lifted::new(0x7000b99740, fn_7000b99740 as *const (), 1), // +0x4ad740 sub_4ad740
    Lifted::new(0x7000b99790, fn_7000b99790 as *const (), 1), // +0x4ad790 sub_4ad790
    Lifted::new(0x7000b997a0, fn_7000b997a0 as *const (), 2), // +0x4ad7a0 sub_4ad7a0
    Lifted::new(0x7000b99970, fn_7000b99970 as *const (), 0), // +0x4ad970 sub_4ad970
    Lifted::new(0x7000b999c0, tpl_245::<0x90> as *const (), 1), // +0x4ad9c0 sub_4ad9c0
    Lifted::new(0x7000b999d0, fn_7000b999d0 as *const (), 2), // +0x4ad9d0 sub_4ad9d0
    Lifted::new(0x7000b99ba0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000b99ca7, 0x7000b96ff0, 0x70007f0910, 0x7000b99cc3> as *const (), 0), // +0x4adba0 sub_4adba0
    Lifted::new(0x7000b99d10, tpl_944::<0x7000b99ba0, 0x700177c0c0, 0x70017d37a0, 0x70017790e0, 0x7000b99f82> as *const (), 0), // +0x4add10 sub_4add10
    Lifted::new(0x7000b9a020, fn_7000b9a020 as *const (), 2), // +0x4ae020 sub_4ae020
    Lifted::new(0x7000b9a400, tpl_942::<0x700177c0e0> as *const (), 0), // +0x4ae400 sub_4ae400
    Lifted::new(0x7000b9a470, tpl_292::<0x70018023f8, 0x70017790e0, 0x7000b9a480> as *const (), 0), // +0x4ae470 sub_4ae470
    Lifted::new(0x7000b9a490, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b9a4a0> as *const (), 0), // +0x4ae490 sub_4ae490
    Lifted::new(0x7000b9a4b0, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000b9a4fd, 0x70007f0910, 0x7000b9a51b> as *const (), 0), // +0x4ae4b0 sub_4ae4b0
    Lifted::new(0x7000b9a540, fn_7000b9a540 as *const (), 3), // +0x4ae540 sub_4ae540
    Lifted::new(0x7000b9a5f0, fn_7000b9a5f0 as *const (), 0), // +0x4ae5f0 sub_4ae5f0
    Lifted::new(0x7000b9a650, fn_7000b9a650 as *const (), 0), // +0x4ae650 sub_4ae650
    Lifted::new(0x7000b9a670, tpl_414::<0x38, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x30, 0x7001d54968, 0x7000b9a6d0> as *const (), 0), // +0x4ae670 sub_4ae670
    Lifted::new(0x7000b9a6f0, fn_7000b9a6f0 as *const (), 0), // +0x4ae6f0 sub_4ae6f0
    Lifted::new(0x7000b9a780, fn_7000b9a780 as *const (), 0), // +0x4ae780 sub_4ae780
    Lifted::new(0x7000b9ade0, fn_7000b9ade0 as *const (), 2), // +0x4aede0 sub_4aede0
    Lifted::new(0x7000b9b300, fn_7000b9b300 as *const (), 2), // +0x4af300 sub_4af300
    Lifted::new(0x7000b9b940, fn_7000b9b940 as *const (), 0), // +0x4af940 sub_4af940
    Lifted::new(0x7000b9b9e0, fn_7000b9b9e0 as *const (), 2), // +0x4af9e0 sub_4af9e0
    Lifted::new(0x7000b9bbf0, tpl_287 as *const (), 1), // +0x4afbf0 sub_4afbf0
    Lifted::new(0x7000b9bf10, tpl_287 as *const (), 1), // +0x4aff10 sub_4aff10
    Lifted::new(0x7000b9c230, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b9c240> as *const (), 0), // +0x4b0230 sub_4b0230
    Lifted::new(0x7000b9c250, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000b9c260> as *const (), 0), // +0x4b0250 sub_4b0250
    Lifted::new(0x7000b9c270, fn_7000b9c270 as *const (), 0), // +0x4b0270 sub_4b0270
    Lifted::new(0x7000b9c300, fn_7000b9c300 as *const (), 0), // +0x4b0300 sub_4b0300
    Lifted::new(0x7000b9c330, tpl_945::<0x700076bf20, 0x700177c0d0, 0x70009a4db0, 0x7000b9c550> as *const (), 0), // +0x4b0330 sub_4b0330
    Lifted::new(0x7000b9c5e0, fn_7000b9c5e0 as *const (), 1), // +0x4b05e0 sub_4b05e0
    Lifted::new(0x7000b9c600, fn_7000b9c600 as *const (), 1), // +0x4b0600 sub_4b0600
    Lifted::new(0x7000b9c630, tpl_107::<0x20, 0x28, 0x30> as *const (), 1), // +0x4b0630 sub_4b0630
    Lifted::new(0x7000b9c650, tpl_20 as *const (), 1), // +0x4b0650 sub_4b0650
    Lifted::new(0x7000b9c660, tpl_20 as *const (), 1), // +0x4b0660 sub_4b0660
    Lifted::new(0x7000b9c670, tpl_23::<0x8, 0x10> as *const (), 1), // +0x4b0670 sub_4b0670
    Lifted::new(0x7000b9c690, tpl_21::<0x10> as *const (), 3), // +0x4b0690 sub_4b0690
    Lifted::new(0x7000b9c6b0, fn_7000b9c6b0 as *const (), 0), // +0x4b06b0 sub_4b06b0
    Lifted::new(0x7000b9c7d0, fn_7000b9c7d0 as *const (), 1), // +0x4b07d0 sub_4b07d0
    Lifted::new(0x7000b9c850, fn_7000b9c850 as *const (), 1), // +0x4b0850 sub_4b0850
    Lifted::new(0x7000b9c8a0, tpl_206::<0x10, 0x8> as *const (), 1), // +0x4b08a0 sub_4b08a0
    Lifted::new(0x7000b9c8b0, fn_7000b9c8b0 as *const (), 0), // +0x4b08b0 sub_4b08b0
    Lifted::new(0x7000b9c9c0, tpl_946::<0x10> as *const (), 1), // +0x4b09c0 sub_4b09c0
    Lifted::new(0x7000b9c9d0, tpl_946::<0x8> as *const (), 1), // +0x4b09d0 sub_4b09d0
    Lifted::new(0x7000b9c9e0, fn_7000b9c9e0 as *const (), 1), // +0x4b09e0 sub_4b09e0
    Lifted::new(0x7000b9ca00, tpl_947::<0x8> as *const (), 1), // +0x4b0a00 sub_4b0a00
    Lifted::new(0x7000b9ca20, tpl_947::<0x8> as *const (), 1), // +0x4b0a20 sub_4b0a20
    Lifted::new(0x7000b9ca40, fn_7000b9ca40 as *const (), 1), // +0x4b0a40 sub_4b0a40
    Lifted::new(0x7000b9caa0, fn_7000b9caa0 as *const (), 1), // +0x4b0aa0 sub_4b0aa0
    Lifted::new(0x7000b9cac0, fn_7000b9cac0 as *const (), 1), // +0x4b0ac0 sub_4b0ac0
    Lifted::new(0x7000b9cae0, fn_7000b9cae0 as *const (), 1), // +0x4b0ae0 sub_4b0ae0
    Lifted::new(0x7000b9cb30, fn_7000b9cb30 as *const (), 1), // +0x4b0b30 sub_4b0b30
    Lifted::new(0x7000b9cb40, fn_7000b9cb40 as *const (), 1), // +0x4b0b40 sub_4b0b40
    Lifted::new(0x7000b9cb50, fn_7000b9cb50 as *const (), 0), // +0x4b0b50 sub_4b0b50
    Lifted::new(0x7000b9cb90, fn_7000b9cb90 as *const (), 0), // +0x4b0b90 sub_4b0b90
    Lifted::new(0x7000b9cc80, tpl_641::<0x20> as *const (), 1), // +0x4b0c80 sub_4b0c80
    Lifted::new(0x7000b9cc90, fn_7000b9cc90 as *const (), 0), // +0x4b0c90 sub_4b0c90
    Lifted::new(0x7000b9ccf0, fn_7000b9ccf0 as *const (), 0), // +0x4b0cf0 sub_4b0cf0
    Lifted::new(0x7000b9cd60, fn_7000b9cd60 as *const (), 0), // +0x4b0d60 sub_4b0d60
    Lifted::new(0x7000b9cda0, fn_7000b9cda0 as *const (), 0), // +0x4b0da0 sub_4b0da0
    Lifted::new(0x7000b9cdf0, fn_7000b9cdf0 as *const (), 0), // +0x4b0df0 sub_4b0df0
    Lifted::new(0x7000b9ce90, fn_7000b9ce90 as *const (), 0), // +0x4b0e90 sub_4b0e90
    Lifted::new(0x7000b9ced0, fn_7000b9ced0 as *const (), 0), // +0x4b0ed0 sub_4b0ed0
    Lifted::new(0x7000b9cf10, fn_7000b9cf10 as *const (), 0), // +0x4b0f10 sub_4b0f10
    Lifted::new(0x7000b9cf70, fn_7000b9cf70 as *const (), 0), // +0x4b0f70 sub_4b0f70
    Lifted::new(0x7000b9cff0, fn_7000b9cff0 as *const (), 1), // +0x4b0ff0 sub_4b0ff0
    Lifted::new(0x7000b9d000, fn_7000b9d000 as *const (), 1), // +0x4b1000 sub_4b1000
    Lifted::new(0x7000b9d020, fn_7000b9d020 as *const (), 1), // +0x4b1020 sub_4b1020
    Lifted::new(0x7000b9d040, fn_7000b9d040 as *const (), 0), // +0x4b1040 sub_4b1040
    Lifted::new(0x7000b9d1f0, fn_7000b9d1f0 as *const (), 0), // +0x4b11f0 sub_4b11f0
    Lifted::new(0x7000b9d230, fn_7000b9d230 as *const (), 0), // +0x4b1230 sub_4b1230
    Lifted::new(0x7000b9d3e0, fn_7000b9d3e0 as *const (), 0), // +0x4b13e0 sub_4b13e0
    Lifted::new(0x7000b9d530, fn_7000b9d530 as *const (), 2), // +0x4b1530 sub_4b1530
    Lifted::new(0x7000b9d5c0, fn_7000b9d5c0 as *const (), 0), // +0x4b15c0 sub_4b15c0
    Lifted::new(0x7000b9d6c0, fn_7000b9d6c0 as *const (), 0), // +0x4b16c0 sub_4b16c0
    Lifted::new(0x7000b9d700, fn_7000b9d700 as *const (), 0), // +0x4b1700 sub_4b1700
    Lifted::new(0x7000b9d750, fn_7000b9d750 as *const (), 0), // +0x4b1750 sub_4b1750
    Lifted::new(0x7000b9d7a0, fn_7000b9d7a0 as *const (), 0), // +0x4b17a0 sub_4b17a0
    Lifted::new(0x7000b9d9d0, fn_7000b9d9d0 as *const (), 0), // +0x4b19d0 sub_4b19d0
    Lifted::new(0x7000b9dbd0, tpl_39::<0x10> as *const (), 1), // +0x4b1bd0 sub_4b1bd0
    Lifted::new(0x7000b9dbe0, tpl_185::<0xc> as *const (), 1), // +0x4b1be0 sub_4b1be0
    Lifted::new(0x7000b9dbf0, tpl_36::<0x8> as *const (), 1), // +0x4b1bf0 sub_4b1bf0
    Lifted::new(0x7000b9dc00, tpl_379::<0x28, 0x20> as *const (), 1), // +0x4b1c00 sub_4b1c00
    Lifted::new(0x7000b9dc10, tpl_41 as *const (), 1), // +0x4b1c10 sub_4b1c10
    Lifted::new(0x7000b9dc20, tpl_39::<0x4> as *const (), 1), // +0x4b1c20 sub_4b1c20
    Lifted::new(0x7000b9dc30, fn_7000b9dc30 as *const (), 0), // +0x4b1c30 sub_4b1c30
    Lifted::new(0x7000b9dc60, tpl_20 as *const (), 1), // +0x4b1c60 sub_4b1c60
    Lifted::new(0x7000b9dc70, tpl_20 as *const (), 1), // +0x4b1c70 sub_4b1c70
    Lifted::new(0x7000b9dc80, fn_7000b9dc80 as *const (), 1), // +0x4b1c80 sub_4b1c80
    Lifted::new(0x7000b9dd10, fn_7000b9dd10 as *const (), 2), // +0x4b1d10 sub_4b1d10
    Lifted::new(0x7000b9de00, fn_7000b9de00 as *const (), 1), // +0x4b1e00 sub_4b1e00
    Lifted::new(0x7000b9de30, fn_7000b9de30 as *const (), 0), // +0x4b1e30 sub_4b1e30
    Lifted::new(0x7000b9e6c0, fn_7000b9e6c0 as *const (), 2), // +0x4b26c0 sub_4b26c0
    Lifted::new(0x7000b9e850, fn_7000b9e850 as *const (), 0), // +0x4b2850 sub_4b2850
    Lifted::new(0x7000b9e9a0, fn_7000b9e9a0 as *const (), 0), // +0x4b29a0 sub_4b29a0
    Lifted::new(0x7000b9eb50, fn_7000b9eb50 as *const (), 0), // +0x4b2b50 sub_4b2b50
    Lifted::new(0x7000b9edd0, fn_7000b9edd0 as *const (), 0), // +0x4b2dd0 sub_4b2dd0
    Lifted::new(0x7000b9ee30, fn_7000b9ee30 as *const (), 1), // +0x4b2e30 sub_4b2e30
    Lifted::new(0x7000b9ee50, fn_7000b9ee50 as *const (), 0), // +0x4b2e50 sub_4b2e50
    Lifted::new(0x7000b9eec0, fn_7000b9eec0 as *const (), 0), // +0x4b2ec0 sub_4b2ec0
    Lifted::new(0x7000b9f140, tpl_15 as *const (), 1), // +0x4b3140 sub_4b3140
    Lifted::new(0x7000b9f150, fn_7000b9f150 as *const (), 0), // +0x4b3150 sub_4b3150
    Lifted::new(0x7000b9f1b0, fn_7000b9f1b0 as *const (), 0), // +0x4b31b0 sub_4b31b0
    Lifted::new(0x7000b9f200, tpl_15 as *const (), 1), // +0x4b3200 sub_4b3200
    Lifted::new(0x7000b9f210, fn_7000b9f210 as *const (), 0), // +0x4b3210 sub_4b3210
    Lifted::new(0x7000b9f290, fn_7000b9f290 as *const (), 1), // +0x4b3290 sub_4b3290
    Lifted::new(0x7000b9f300, fn_7000b9f300 as *const (), 1), // +0x4b3300 sub_4b3300
    Lifted::new(0x7000b9f390, fn_7000b9f390 as *const (), 0), // +0x4b3390 sub_4b3390
    Lifted::new(0x7000b9f610, fn_7000b9f610 as *const (), 0), // +0x4b3610 sub_4b3610
    Lifted::new(0x7000b9f660, fn_7000b9f660 as *const (), 2), // +0x4b3660 sub_4b3660
    Lifted::new(0x7000b9f770, fn_7000b9f770 as *const (), 2), // +0x4b3770 sub_4b3770
    Lifted::new(0x7000b9f8a0, fn_7000b9f8a0 as *const (), 2), // +0x4b38a0 sub_4b38a0
    Lifted::new(0x7000b9fb40, fn_7000b9fb40 as *const (), 2), // +0x4b3b40 sub_4b3b40
    Lifted::new(0x7000ba0260, fn_7000ba0260 as *const (), 0), // +0x4b4260 sub_4b4260
    Lifted::new(0x7000ba03c0, fn_7000ba03c0 as *const (), 2), // +0x4b43c0 sub_4b43c0
    Lifted::new(0x7000ba0490, fn_7000ba0490 as *const (), 2), // +0x4b4490 sub_4b4490
    Lifted::new(0x7000ba0550, fn_7000ba0550 as *const (), 0), // +0x4b4550 sub_4b4550
    Lifted::new(0x7000ba0df0, fn_7000ba0df0 as *const (), 0), // +0x4b4df0 sub_4b4df0
    Lifted::new(0x7000ba0ef0, fn_7000ba0ef0 as *const (), 2), // +0x4b4ef0 sub_4b4ef0
    Lifted::new(0x7000ba1020, fn_7000ba1020 as *const (), 2), // +0x4b5020 sub_4b5020
    Lifted::new(0x7000ba10e0, fn_7000ba10e0 as *const (), 0), // +0x4b50e0 sub_4b50e0
    Lifted::new(0x7000ba1290, fn_7000ba1290 as *const (), 0), // +0x4b5290 sub_4b5290
    Lifted::new(0x7000ba1320, fn_7000ba1320 as *const (), 2), // +0x4b5320 sub_4b5320
    Lifted::new(0x7000ba16b0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000ba1884, 0x7000baa8b0, 0x7000ba188a, 0x70007f0910, 0x7000ba1890> as *const (), 0), // +0x4b56b0 sub_4b56b0
    Lifted::new(0x7000ba1920, fn_7000ba1920 as *const (), 0), // +0x4b5920 sub_4b5920
    Lifted::new(0x7000ba1ac0, fn_7000ba1ac0 as *const (), 0), // +0x4b5ac0 sub_4b5ac0
    Lifted::new(0x7000ba1c00, fn_7000ba1c00 as *const (), 1), // +0x4b5c00 sub_4b5c00
    Lifted::new(0x7000ba1c30, fn_7000ba1c30 as *const (), 0), // +0x4b5c30 sub_4b5c30
    Lifted::new(0x7000ba1df0, fn_7000ba1df0 as *const (), 2), // +0x4b5df0 sub_4b5df0
    Lifted::new(0x7000ba23a0, fn_7000ba23a0 as *const (), 0), // +0x4b63a0 sub_4b63a0
    Lifted::new(0x7000ba23d0, fn_7000ba23d0 as *const (), 2), // +0x4b63d0 sub_4b63d0
    Lifted::new(0x7000ba2510, fn_7000ba2510 as *const (), 1), // +0x4b6510 sub_4b6510
    Lifted::new(0x7000ba2c90, fn_7000ba2c90 as *const (), 0), // +0x4b6c90 sub_4b6c90
    Lifted::new(0x7000ba2d00, fn_7000ba2d00 as *const (), 0), // +0x4b6d00 sub_4b6d00
    Lifted::new(0x7000ba2de0, fn_7000ba2de0 as *const (), 0), // +0x4b6de0 sub_4b6de0
    Lifted::new(0x7000ba2e90, fn_7000ba2e90 as *const (), 0), // +0x4b6e90 sub_4b6e90
    Lifted::new(0x7000ba2ec0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6ec0 sub_4b6ec0
    Lifted::new(0x7000ba2ef0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6ef0 sub_4b6ef0
    Lifted::new(0x7000ba2f20, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6f20 sub_4b6f20
    Lifted::new(0x7000ba2f50, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6f50 sub_4b6f50
    Lifted::new(0x7000ba2f80, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6f80 sub_4b6f80
    Lifted::new(0x7000ba2fb0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6fb0 sub_4b6fb0
    Lifted::new(0x7000ba2fe0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b6fe0 sub_4b6fe0
    Lifted::new(0x7000ba3010, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7010 sub_4b7010
    Lifted::new(0x7000ba3040, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7040 sub_4b7040
    Lifted::new(0x7000ba3070, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7070 sub_4b7070
    Lifted::new(0x7000ba30a0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b70a0 sub_4b70a0
    Lifted::new(0x7000ba30d0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b70d0 sub_4b70d0
    Lifted::new(0x7000ba3100, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7100 sub_4b7100
    Lifted::new(0x7000ba3130, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7130 sub_4b7130
    Lifted::new(0x7000ba3160, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7160 sub_4b7160
    Lifted::new(0x7000ba3190, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7190 sub_4b7190
    Lifted::new(0x7000ba31c0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b71c0 sub_4b71c0
    Lifted::new(0x7000ba31f0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b71f0 sub_4b71f0
    Lifted::new(0x7000ba3220, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7220 sub_4b7220
    Lifted::new(0x7000ba3250, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7250 sub_4b7250
    Lifted::new(0x7000ba3280, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7280 sub_4b7280
    Lifted::new(0x7000ba32b0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b72b0 sub_4b72b0
    Lifted::new(0x7000ba32e0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b72e0 sub_4b72e0
    Lifted::new(0x7000ba3310, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7310 sub_4b7310
    Lifted::new(0x7000ba3340, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7340 sub_4b7340
    Lifted::new(0x7000ba3370, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7370 sub_4b7370
    Lifted::new(0x7000ba33a0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b73a0 sub_4b73a0
    Lifted::new(0x7000ba33d0, tpl_33::<0x700177c312> as *const (), 0), // +0x4b73d0 sub_4b73d0
    Lifted::new(0x7000ba3400, tpl_33::<0x700177c312> as *const (), 0), // +0x4b7400 sub_4b7400
    Lifted::new(0x7000ba3430, fn_7000ba3430 as *const (), 0), // +0x4b7430 sub_4b7430
    Lifted::new(0x7000ba3480, fn_7000ba3480 as *const (), 0), // +0x4b7480 sub_4b7480
    Lifted::new(0x7000ba35f0, fn_7000ba35f0 as *const (), 2), // +0x4b75f0 sub_4b75f0
    Lifted::new(0x7000ba36e0, fn_7000ba36e0 as *const (), 0), // +0x4b76e0 sub_4b76e0
    Lifted::new(0x7000ba3730, fn_7000ba3730 as *const (), 2), // +0x4b7730 sub_4b7730
    Lifted::new(0x7000ba39b0, fn_7000ba39b0 as *const (), 0), // +0x4b79b0 sub_4b79b0
    Lifted::new(0x7000ba39c0, fn_7000ba39c0 as *const (), 1), // +0x4b79c0 sub_4b79c0
    Lifted::new(0x7000ba3bc0, fn_7000ba3bc0 as *const (), 2), // +0x4b7bc0 sub_4b7bc0
    Lifted::new(0x7000ba3d70, fn_7000ba3d70 as *const (), 0), // +0x4b7d70 sub_4b7d70
    Lifted::new(0x7000ba3df0, tpl_260::<0x38> as *const (), 0), // +0x4b7df0 sub_4b7df0
    Lifted::new(0x7000ba3e00, fn_7000ba3e00 as *const (), 0), // +0x4b7e00 sub_4b7e00
    Lifted::new(0x7000ba3e80, fn_7000ba3e80 as *const (), 0), // +0x4b7e80 sub_4b7e80
    Lifted::new(0x7000ba3ef0, fn_7000ba3ef0 as *const (), 0), // +0x4b7ef0 sub_4b7ef0
    Lifted::new(0x7000ba3f60, fn_7000ba3f60 as *const (), 1), // +0x4b7f60 sub_4b7f60
    Lifted::new(0x7000ba3f70, tpl_948::<0x20, 0xffffffffffffffe8> as *const (), 1), // +0x4b7f70 sub_4b7f70
    Lifted::new(0x7000ba3f80, fn_7000ba3f80 as *const (), 0), // +0x4b7f80 sub_4b7f80
    Lifted::new(0x7000ba3fc0, tpl_948::<0x20, 0xffffffffffffffd0> as *const (), 1), // +0x4b7fc0 sub_4b7fc0
    Lifted::new(0x7000ba3fd0, fn_7000ba3fd0 as *const (), 0), // +0x4b7fd0 sub_4b7fd0
    Lifted::new(0x7000ba4080, fn_7000ba4080 as *const (), 0), // +0x4b8080 sub_4b8080
    Lifted::new(0x7000ba43f0, fn_7000ba43f0 as *const (), 0), // +0x4b83f0 sub_4b83f0
    Lifted::new(0x7000ba44b0, fn_7000ba44b0 as *const (), 0), // +0x4b84b0 sub_4b84b0
    Lifted::new(0x7000ba4560, fn_7000ba4560 as *const (), 0), // +0x4b8560 sub_4b8560
    Lifted::new(0x7000ba45b0, fn_7000ba45b0 as *const (), 2), // +0x4b85b0 sub_4b85b0
    Lifted::new(0x7000ba4680, fn_7000ba4680 as *const (), 2), // +0x4b8680 sub_4b8680
    Lifted::new(0x7000ba47d0, tpl_196 as *const (), 1), // +0x4b87d0 sub_4b87d0
    Lifted::new(0x7000ba47e0, tpl_360::<0x20, 0xffffffffffffffe8> as *const (), 1), // +0x4b87e0 sub_4b87e0
    Lifted::new(0x7000ba47f0, tpl_949::<0x70007209e0> as *const (), 0), // +0x4b87f0 sub_4b87f0
    Lifted::new(0x7000ba4810, fn_7000ba4810 as *const (), 0), // +0x4b8810 sub_4b8810
    Lifted::new(0x7000ba4840, fn_7000ba4840 as *const (), 0), // +0x4b8840 sub_4b8840
    Lifted::new(0x7000ba49e0, fn_7000ba49e0 as *const (), 1), // +0x4b89e0 sub_4b89e0
    Lifted::new(0x7000ba49f0, fn_7000ba49f0 as *const (), 0), // +0x4b89f0 sub_4b89f0
    Lifted::new(0x7000ba4a20, fn_7000ba4a20 as *const (), 0), // +0x4b8a20 sub_4b8a20
    Lifted::new(0x7000ba4bb0, tpl_950::<0x20> as *const (), 1), // +0x4b8bb0 sub_4b8bb0
    Lifted::new(0x7000ba4bc0, tpl_196 as *const (), 1), // +0x4b8bc0 sub_4b8bc0
    Lifted::new(0x7000ba4bd0, tpl_950::<0x20> as *const (), 1), // +0x4b8bd0 sub_4b8bd0
    Lifted::new(0x7000ba4be0, tpl_951::<0x20, 0xfffffffffffffff0, 0x7000ba5810> as *const (), 0), // +0x4b8be0 sub_4b8be0
    Lifted::new(0x7000ba4bf0, tpl_951::<0x20, 0xffffffffffffff10, 0x7000ba5810> as *const (), 0), // +0x4b8bf0 sub_4b8bf0
    Lifted::new(0x7000ba4c10, fn_7000ba4c10 as *const (), 0), // +0x4b8c10 sub_4b8c10
    Lifted::new(0x7000ba4c50, fn_7000ba4c50 as *const (), 0), // +0x4b8c50 sub_4b8c50
    Lifted::new(0x7000ba4e30, fn_7000ba4e30 as *const (), 1), // +0x4b8e30 sub_4b8e30
    Lifted::new(0x7000ba4e50, fn_7000ba4e50 as *const (), 1), // +0x4b8e50 sub_4b8e50
    Lifted::new(0x7000ba4e70, fn_7000ba4e70 as *const (), 2), // +0x4b8e70 sub_4b8e70
    Lifted::new(0x7000ba5020, fn_7000ba5020 as *const (), 0), // +0x4b9020 sub_4b9020
    Lifted::new(0x7000ba50a0, fn_7000ba50a0 as *const (), 0), // +0x4b90a0 sub_4b90a0
    Lifted::new(0x7000ba5380, fn_7000ba5380 as *const (), 0), // +0x4b9380 sub_4b9380
    Lifted::new(0x7000ba5460, fn_7000ba5460 as *const (), 1), // +0x4b9460 sub_4b9460
    Lifted::new(0x7000ba5490, fn_7000ba5490 as *const (), 0), // +0x4b9490 sub_4b9490
    Lifted::new(0x7000ba54e0, fn_7000ba54e0 as *const (), 2), // +0x4b94e0 sub_4b94e0
    Lifted::new(0x7000ba56d0, fn_7000ba56d0 as *const (), 1), // +0x4b96d0 sub_4b96d0
    Lifted::new(0x7000ba56e0, fn_7000ba56e0 as *const (), 2), // +0x4b96e0 sub_4b96e0
    Lifted::new(0x7000ba5800, tpl_163 as *const (), 1), // +0x4b9800 sub_4b9800
    Lifted::new(0x7000ba5810, fn_7000ba5810 as *const (), 0), // +0x4b9810 sub_4b9810
    Lifted::new(0x7000ba58d0, fn_7000ba58d0 as *const (), 2), // +0x4b98d0 sub_4b98d0
    Lifted::new(0x7000ba5a00, fn_7000ba5a00 as *const (), 2), // +0x4b9a00 sub_4b9a00
    Lifted::new(0x7000ba5ac0, fn_7000ba5ac0 as *const (), 1), // +0x4b9ac0 sub_4b9ac0
    Lifted::new(0x7000ba5ae0, tpl_952::<0x20, 0xfffffffffffffff8> as *const (), 1), // +0x4b9ae0 sub_4b9ae0
    Lifted::new(0x7000ba5b00, tpl_952::<0x20, 0xfffffffffffffff8> as *const (), 1), // +0x4b9b00 sub_4b9b00
    Lifted::new(0x7000ba5b20, fn_7000ba5b20 as *const (), 0), // +0x4b9b20 sub_4b9b20
    Lifted::new(0x7000ba5c20, fn_7000ba5c20 as *const (), 0), // +0x4b9c20 sub_4b9c20
    Lifted::new(0x7000ba5d50, fn_7000ba5d50 as *const (), 2), // +0x4b9d50 sub_4b9d50
    Lifted::new(0x7000ba5eb0, fn_7000ba5eb0 as *const (), 0), // +0x4b9eb0 sub_4b9eb0
    Lifted::new(0x7000ba5f60, fn_7000ba5f60 as *const (), 0), // +0x4b9f60 sub_4b9f60
    Lifted::new(0x7000ba6040, fn_7000ba6040 as *const (), 0), // +0x4ba040 sub_4ba040
    Lifted::new(0x7000ba60e0, tpl_953::<0x28, 0x10, 0xa710, 0x7000ba4a20, 0x40, 0x8, 0x7000ba6c30> as *const (), 0), // +0x4ba0e0 sub_4ba0e0
    Lifted::new(0x7000ba6150, tpl_953::<0x28, 0x10, 0xa710, 0x7000ba4a20, 0x40, 0x8, 0x7000ba6c30> as *const (), 0), // +0x4ba150 sub_4ba150
    Lifted::new(0x7000ba61c0, fn_7000ba61c0 as *const (), 0), // +0x4ba1c0 sub_4ba1c0
    Lifted::new(0x7000ba6260, tpl_954::<0x7000ba6b90, 0x28, 0x10, 0xa710, 0x7000ba4a20, 0x40, 0x8, 0x7000ba6c30> as *const (), 0), // +0x4ba260 sub_4ba260
    Lifted::new(0x7000ba62d0, fn_7000ba62d0 as *const (), 0), // +0x4ba2d0 sub_4ba2d0
    Lifted::new(0x7000ba6380, tpl_384::<0x7000ba6383, 0x7000ba6384, 0x7000ba6385, 0x7000ba6386, 0x7000ba6387, 0x7000ba6388, 0x7000ba6389, 0x7000ba638a, 0x7000ba638b, 0x7000ba638c, 0x7000ba638d, 0x7000ba638e, 0x7000ba638f, 0x7000ba6640, 0x7000ba6395, 0x7000ba6396, 0x7000ba6397, 0x7000ba6398, 0x7000ba6399, 0x7000ba639a, 0x7000ba639b, 0x7000ba639c, 0x7000ba639d, 0x7000ba639e, 0x7000ba639f> as *const (), 0), // +0x4ba380 sub_4ba380
    Lifted::new(0x7000ba63b0, tpl_954::<0x7000ba6b90, 0x28, 0x10, 0xa710, 0x7000ba4a20, 0x40, 0x8, 0x7000ba6c30> as *const (), 0), // +0x4ba3b0 sub_4ba3b0
    Lifted::new(0x7000ba6420, fn_7000ba6420 as *const (), 0), // +0x4ba420 sub_4ba420
    Lifted::new(0x7000ba6490, fn_7000ba6490 as *const (), 0), // +0x4ba490 sub_4ba490
    Lifted::new(0x7000ba6640, fn_7000ba6640 as *const (), 2), // +0x4ba640 sub_4ba640
    Lifted::new(0x7000ba6b90, fn_7000ba6b90 as *const (), 0), // +0x4bab90 sub_4bab90
    Lifted::new(0x7000ba6c30, fn_7000ba6c30 as *const (), 0), // +0x4bac30 sub_4bac30
    Lifted::new(0x7000ba6d30, fn_7000ba6d30 as *const (), 0), // +0x4bad30 sub_4bad30
    Lifted::new(0x7000ba6d70, tpl_479 as *const (), 1), // +0x4bad70 sub_4bad70
    Lifted::new(0x7000ba6d80, fn_7000ba6d80 as *const (), 0), // +0x4bad80 sub_4bad80
    Lifted::new(0x7000ba6ea0, fn_7000ba6ea0 as *const (), 0), // +0x4baea0 sub_4baea0
    Lifted::new(0x7000ba6f50, fn_7000ba6f50 as *const (), 0), // +0x4baf50 sub_4baf50
    Lifted::new(0x7000ba6fd0, fn_7000ba6fd0 as *const (), 0), // +0x4bafd0 sub_4bafd0
    Lifted::new(0x7000ba7070, fn_7000ba7070 as *const (), 0), // +0x4bb070 sub_4bb070
    Lifted::new(0x7000ba70a0, fn_7000ba70a0 as *const (), 0), // +0x4bb0a0 sub_4bb0a0
    Lifted::new(0x7000ba70d0, tpl_518::<0x20, 0xfffffffffffffff8> as *const (), 1), // +0x4bb0d0 sub_4bb0d0
    Lifted::new(0x7000ba70e0, fn_7000ba70e0 as *const (), 0), // +0x4bb0e0 sub_4bb0e0
    Lifted::new(0x7000ba7350, fn_7000ba7350 as *const (), 2), // +0x4bb350 sub_4bb350
    Lifted::new(0x7000ba82f0, fn_7000ba82f0 as *const (), 0), // +0x4bc2f0 sub_4bc2f0
    Lifted::new(0x7000ba85f0, fn_7000ba85f0 as *const (), 0), // +0x4bc5f0 sub_4bc5f0
    Lifted::new(0x7000ba8720, fn_7000ba8720 as *const (), 0), // +0x4bc720 sub_4bc720
    Lifted::new(0x7000ba8b90, tpl_15 as *const (), 1), // +0x4bcb90 sub_4bcb90
    Lifted::new(0x7000ba8ba0, fn_7000ba8ba0 as *const (), 0), // +0x4bcba0 sub_4bcba0
    Lifted::new(0x7000ba8e20, fn_7000ba8e20 as *const (), 2), // +0x4bce20 sub_4bce20
    Lifted::new(0x7000ba9140, fn_7000ba9140 as *const (), 0), // +0x4bd140 sub_4bd140
    Lifted::new(0x7000ba9420, fn_7000ba9420 as *const (), 0), // +0x4bd420 sub_4bd420
    Lifted::new(0x7000ba9450, fn_7000ba9450 as *const (), 2), // +0x4bd450 sub_4bd450
    Lifted::new(0x7000ba94c0, fn_7000ba94c0 as *const (), 2), // +0x4bd4c0 sub_4bd4c0
    Lifted::new(0x7000ba95e0, fn_7000ba95e0 as *const (), 0), // +0x4bd5e0 sub_4bd5e0
    Lifted::new(0x7000ba9650, fn_7000ba9650 as *const (), 0), // +0x4bd650 sub_4bd650
    Lifted::new(0x7000ba9690, tpl_955::<0x7001c71e00> as *const (), 1), // +0x4bd690 sub_4bd690
    Lifted::new(0x7000ba96a0, fn_7000ba96a0 as *const (), 0), // +0x4bd6a0 sub_4bd6a0
    Lifted::new(0x7000ba9820, fn_7000ba9820 as *const (), 2), // +0x4bd820 sub_4bd820
    Lifted::new(0x7000ba9860, tpl_956::<0x8, 0x70017d2c10, 0x7001d54ef8, 0x7000ba9880, 0x7000ba98f0, 0x20, 0x28, 0x7000dbedc0> as *const (), 0), // +0x4bd860 sub_4bd860
    Lifted::new(0x7000ba98c0, tpl_957::<0x20, 0x28, 0x7000dbedc0> as *const (), 0), // +0x4bd8c0 sub_4bd8c0
    Lifted::new(0x7000ba98f0, fn_7000ba98f0 as *const (), 0), // +0x4bd8f0 sub_4bd8f0
    Lifted::new(0x7000ba9980, tpl_956::<0x8, 0x70017d2c10, 0x7001d54ef8, 0x7000ba99a0, 0x7000ba98f0, 0x20, 0x28, 0x7000dbf010> as *const (), 0), // +0x4bd980 sub_4bd980
    Lifted::new(0x7000ba99e0, tpl_957::<0x20, 0x28, 0x7000dbf010> as *const (), 0), // +0x4bd9e0 sub_4bd9e0
    Lifted::new(0x7000ba9a40, fn_7000ba9a40 as *const (), 1), // +0x4bda40 sub_4bda40
    Lifted::new(0x7000ba9a60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ba9a71> as *const (), 0), // +0x4bda60 sub_4bda60
    Lifted::new(0x7000ba9a80, fn_7000ba9a80 as *const (), 2), // +0x4bda80 sub_4bda80
    Lifted::new(0x7000ba9cc0, fn_7000ba9cc0 as *const (), 2), // +0x4bdcc0 sub_4bdcc0
    Lifted::new(0x7000baa450, fn_7000baa450 as *const (), 2), // +0x4be450 sub_4be450
    Lifted::new(0x7000baa650, fn_7000baa650 as *const (), 2), // +0x4be650 sub_4be650
    Lifted::new(0x7000baa800, tpl_918::<0x7001804460, 0x8> as *const (), 3), // +0x4be800 sub_4be800
    Lifted::new(0x7000baa820, tpl_919::<0x700177c312> as *const (), 0), // +0x4be820 sub_4be820
    Lifted::new(0x7000baa840, tpl_958::<0x10, 0x8> as *const (), 0), // +0x4be840 sub_4be840
    Lifted::new(0x7000baa850, tpl_52::<0x8> as *const (), 1), // +0x4be850 sub_4be850
    Lifted::new(0x7000baa860, tpl_918::<0x7001804460, 0x8> as *const (), 3), // +0x4be860 sub_4be860
    Lifted::new(0x7000baa880, tpl_315::<0x7001c6b560> as *const (), 1), // +0x4be880 sub_4be880
    Lifted::new(0x7000baa890, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000baa8a0> as *const (), 0), // +0x4be890 sub_4be890
    Lifted::new(0x7000baa8b0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000baa8c0> as *const (), 0), // +0x4be8b0 sub_4be8b0
    Lifted::new(0x7000baa8d0, fn_7000baa8d0 as *const (), 0), // +0x4be8d0 sub_4be8d0
    Lifted::new(0x7000baa970, fn_7000baa970 as *const (), 0), // +0x4be970 sub_4be970
    Lifted::new(0x7000baa9c0, tpl_959::<0x28, 0x20, 0x700076c380> as *const (), 0), // +0x4be9c0 sub_4be9c0
    Lifted::new(0x7000baa9d0, fn_7000baa9d0 as *const (), 0), // +0x4be9d0 sub_4be9d0
    Lifted::new(0x7000baaa00, tpl_540::<0x10, 0xa0> as *const (), 1), // +0x4bea00 sub_4bea00
    Lifted::new(0x7000baaa20, tpl_540::<0x20, 0xffffffffffffffe0> as *const (), 1), // +0x4bea20 sub_4bea20
    Lifted::new(0x7000baaa40, tpl_540::<0x20, 0xfffffffffffffff8> as *const (), 1), // +0x4bea40 sub_4bea40
    Lifted::new(0x7000baaa60, fn_7000baaa60 as *const (), 1), // +0x4bea60 sub_4bea60
    Lifted::new(0x7000baaa90, tpl_540::<0x20, 0x18> as *const (), 1), // +0x4bea90 sub_4bea90
    Lifted::new(0x7000baaab0, tpl_540::<0x20, 0xfffffffffffffff0> as *const (), 1), // +0x4beab0 sub_4beab0
    Lifted::new(0x7000baaad0, tpl_540::<0x20, 0xfffffffffffffff0> as *const (), 1), // +0x4bead0 sub_4bead0
    Lifted::new(0x7000baaaf0, tpl_540::<0x20, 0xfffffffffffffff0> as *const (), 1), // +0x4beaf0 sub_4beaf0
    Lifted::new(0x7000baab10, fn_7000baab10 as *const (), 0), // +0x4beb10 sub_4beb10
    Lifted::new(0x7000baab60, tpl_360::<0x20, 0x30> as *const (), 1), // +0x4beb60 sub_4beb60
    Lifted::new(0x7000baab70, fn_7000baab70 as *const (), 0), // +0x4beb70 sub_4beb70
    Lifted::new(0x7000baabb0, fn_7000baabb0 as *const (), 0), // +0x4bebb0 sub_4bebb0
    Lifted::new(0x7000baac30, fn_7000baac30 as *const (), 0), // +0x4bec30 sub_4bec30
    Lifted::new(0x7000baaca0, fn_7000baaca0 as *const (), 0), // +0x4beca0 sub_4beca0
    Lifted::new(0x7000baad20, fn_7000baad20 as *const (), 0), // +0x4bed20 sub_4bed20
    Lifted::new(0x7000baae30, fn_7000baae30 as *const (), 0), // +0x4bee30 sub_4bee30
    Lifted::new(0x7000baaf00, fn_7000baaf00 as *const (), 0), // +0x4bef00 sub_4bef00
    Lifted::new(0x7000baafb0, fn_7000baafb0 as *const (), 1), // +0x4befb0 sub_4befb0
    Lifted::new(0x7000baaff0, fn_7000baaff0 as *const (), 0), // +0x4beff0 sub_4beff0
    Lifted::new(0x7000bab090, fn_7000bab090 as *const (), 0), // +0x4bf090 sub_4bf090
    Lifted::new(0x7000bab0e0, fn_7000bab0e0 as *const (), 0), // +0x4bf0e0 sub_4bf0e0
    Lifted::new(0x7000bab210, tpl_540::<0x20, 0x30> as *const (), 1), // +0x4bf210 sub_4bf210
    Lifted::new(0x7000bab230, fn_7000bab230 as *const (), 0), // +0x4bf230 sub_4bf230
    Lifted::new(0x7000bab260, fn_7000bab260 as *const (), 0), // +0x4bf260 sub_4bf260
    Lifted::new(0x7000bab380, fn_7000bab380 as *const (), 0), // +0x4bf380 sub_4bf380
    Lifted::new(0x7000bab450, fn_7000bab450 as *const (), 0), // +0x4bf450 sub_4bf450
    Lifted::new(0x7000bab5f0, fn_7000bab5f0 as *const (), 0), // +0x4bf5f0 sub_4bf5f0
    Lifted::new(0x7000bab680, fn_7000bab680 as *const (), 0), // +0x4bf680 sub_4bf680
    Lifted::new(0x7000bab7a0, fn_7000bab7a0 as *const (), 0), // +0x4bf7a0 sub_4bf7a0
    Lifted::new(0x7000bab870, tpl_233 as *const (), 1), // +0x4bf870 sub_4bf870
    Lifted::new(0x7000bab880, fn_7000bab880 as *const (), 1), // +0x4bf880 sub_4bf880
    Lifted::new(0x7000bab890, fn_7000bab890 as *const (), 1), // +0x4bf890 sub_4bf890
    Lifted::new(0x7000bab8a0, tpl_198 as *const (), 1), // +0x4bf8a0 sub_4bf8a0
    Lifted::new(0x7000bab8b0, tpl_148 as *const (), 1), // +0x4bf8b0 sub_4bf8b0
    Lifted::new(0x7000bab8c0, tpl_238 as *const (), 1), // +0x4bf8c0 sub_4bf8c0
    Lifted::new(0x7000bab8d0, fn_7000bab8d0 as *const (), 1), // +0x4bf8d0 sub_4bf8d0
    Lifted::new(0x7000bab8e0, tpl_53 as *const (), 1), // +0x4bf8e0 sub_4bf8e0
    Lifted::new(0x7000bab8f0, tpl_197 as *const (), 1), // +0x4bf8f0 sub_4bf8f0
    Lifted::new(0x7000bab900, fn_7000bab900 as *const (), 1), // +0x4bf900 sub_4bf900
    Lifted::new(0x7000bab910, fn_7000bab910 as *const (), 1), // +0x4bf910 sub_4bf910
    Lifted::new(0x7000bab920, tpl_164 as *const (), 1), // +0x4bf920 sub_4bf920
    Lifted::new(0x7000bab930, tpl_960 as *const (), 1), // +0x4bf930 sub_4bf930
    Lifted::new(0x7000bab940, fn_7000bab940 as *const (), 1), // +0x4bf940 sub_4bf940
    Lifted::new(0x7000bab950, fn_7000bab950 as *const (), 1), // +0x4bf950 sub_4bf950
    Lifted::new(0x7000bab960, fn_7000bab960 as *const (), 1), // +0x4bf960 sub_4bf960
    Lifted::new(0x7000bab970, fn_7000bab970 as *const (), 1), // +0x4bf970 sub_4bf970
    Lifted::new(0x7000bab980, tpl_762 as *const (), 1), // +0x4bf980 sub_4bf980
    Lifted::new(0x7000bab990, tpl_961 as *const (), 1), // +0x4bf990 sub_4bf990
    Lifted::new(0x7000bab9a0, tpl_761 as *const (), 1), // +0x4bf9a0 sub_4bf9a0
    Lifted::new(0x7000bab9b0, tpl_43 as *const (), 1), // +0x4bf9b0 sub_4bf9b0
    Lifted::new(0x7000bab9c0, tpl_962 as *const (), 1), // +0x4bf9c0 sub_4bf9c0
    Lifted::new(0x7000bab9d0, fn_7000bab9d0 as *const (), 0), // +0x4bf9d0 sub_4bf9d0
    Lifted::new(0x7000baba20, fn_7000baba20 as *const (), 0), // +0x4bfa20 sub_4bfa20
    Lifted::new(0x7000baba60, fn_7000baba60 as *const (), 0), // +0x4bfa60 sub_4bfa60
    Lifted::new(0x7000babaa0, fn_7000babaa0 as *const (), 0), // +0x4bfaa0 sub_4bfaa0
    Lifted::new(0x7000babad0, tpl_42 as *const (), 1), // +0x4bfad0 sub_4bfad0
    Lifted::new(0x7000babae0, tpl_540::<0x20, 0xfffffffffffffff0> as *const (), 1), // +0x4bfae0 sub_4bfae0
    Lifted::new(0x7000babb00, fn_7000babb00 as *const (), 0), // +0x4bfb00 sub_4bfb00
    Lifted::new(0x7000babb50, fn_7000babb50 as *const (), 1), // +0x4bfb50 sub_4bfb50
    Lifted::new(0x7000babb90, fn_7000babb90 as *const (), 2), // +0x4bfb90 sub_4bfb90
    Lifted::new(0x7000babc50, fn_7000babc50 as *const (), 0), // +0x4bfc50 sub_4bfc50
    Lifted::new(0x7000babd00, fn_7000babd00 as *const (), 2), // +0x4bfd00 sub_4bfd00
    Lifted::new(0x7000babdd0, fn_7000babdd0 as *const (), 0), // +0x4bfdd0 sub_4bfdd0
    Lifted::new(0x7000babe80, fn_7000babe80 as *const (), 2), // +0x4bfe80 sub_4bfe80
    Lifted::new(0x7000bac6f0, fn_7000bac6f0 as *const (), 2), // +0x4c06f0 sub_4c06f0
    Lifted::new(0x7000bacf70, fn_7000bacf70 as *const (), 2), // +0x4c0f70 sub_4c0f70
    Lifted::new(0x7000bad830, fn_7000bad830 as *const (), 2), // +0x4c1830 sub_4c1830
    Lifted::new(0x7000bae100, tpl_963::<0x19, 0x10, 0x7000bae100, 0x700177c312> as *const (), 0), // +0x4c2100 sub_4c2100
    Lifted::new(0x7000bae170, tpl_963::<0x19, 0x10, 0x7000bae170, 0x700177c312> as *const (), 0), // +0x4c2170 sub_4c2170
    Lifted::new(0x7000bae1e0, tpl_964::<0x7001779e20, 0x7000713f20, 0x70018044e0, 0x28, 0x70018044f8, 0x20> as *const (), 0), // +0x4c21e0 sub_4c21e0
    Lifted::new(0x7000bae280, tpl_964::<0x7001779e20, 0x7000713f20, 0x70018044b0, 0x28, 0x70018044c8, 0x20> as *const (), 0), // +0x4c2280 sub_4c2280
    Lifted::new(0x7000bae320, fn_7000bae320 as *const (), 0), // +0x4c2320 sub_4c2320
    Lifted::new(0x7000bae5e0, fn_7000bae5e0 as *const (), 0), // +0x4c25e0 sub_4c25e0
    Lifted::new(0x7000bae6a0, tpl_899::<0x7000bae6c0> as *const (), 0), // +0x4c26a0 sub_4c26a0
    Lifted::new(0x7000bae6b0, tpl_899::<0x7000bae7f0> as *const (), 0), // +0x4c26b0 sub_4c26b0
    Lifted::new(0x7000bae6c0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x4c26c0 sub_4c26c0
    Lifted::new(0x7000bae700, fn_7000bae700 as *const (), 0), // +0x4c2700 sub_4c2700
    Lifted::new(0x7000bae7f0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x4c27f0 sub_4c27f0
    Lifted::new(0x7000bae830, fn_7000bae830 as *const (), 2), // +0x4c2830 sub_4c2830
    Lifted::new(0x7000bae9c0, fn_7000bae9c0 as *const (), 0), // +0x4c29c0 sub_4c29c0
    Lifted::new(0x7000baea10, fn_7000baea10 as *const (), 0), // +0x4c2a10 sub_4c2a10
    Lifted::new(0x7000baebd0, fn_7000baebd0 as *const (), 2), // +0x4c2bd0 sub_4c2bd0
    Lifted::new(0x7000baedf0, fn_7000baedf0 as *const (), 0), // +0x4c2df0 sub_4c2df0
    Lifted::new(0x7000baee90, fn_7000baee90 as *const (), 2), // +0x4c2e90 sub_4c2e90
    Lifted::new(0x7000baf3a0, fn_7000baf3a0 as *const (), 2), // +0x4c33a0 sub_4c33a0
    Lifted::new(0x7000baf6f0, fn_7000baf6f0 as *const (), 2), // +0x4c36f0 sub_4c36f0
    Lifted::new(0x7000baf7e0, fn_7000baf7e0 as *const (), 2), // +0x4c37e0 sub_4c37e0
    Lifted::new(0x7000bafa10, fn_7000bafa10 as *const (), 2), // +0x4c3a10 sub_4c3a10
    Lifted::new(0x7000bafd50, fn_7000bafd50 as *const (), 2), // +0x4c3d50 sub_4c3d50
    Lifted::new(0x7000bb0080, fn_7000bb0080 as *const (), 0), // +0x4c4080 sub_4c4080
    Lifted::new(0x7000bb0160, fn_7000bb0160 as *const (), 0), // +0x4c4160 sub_4c4160
    Lifted::new(0x7000bb0250, fn_7000bb0250 as *const (), 2), // +0x4c4250 sub_4c4250
    Lifted::new(0x7000bb04c0, fn_7000bb04c0 as *const (), 2), // +0x4c44c0 sub_4c44c0
    Lifted::new(0x7000bb06e0, fn_7000bb06e0 as *const (), 0), // +0x4c46e0 sub_4c46e0
    Lifted::new(0x7000bb0710, fn_7000bb0710 as *const (), 2), // +0x4c4710 sub_4c4710
    Lifted::new(0x7000bb0800, fn_7000bb0800 as *const (), 2), // +0x4c4800 sub_4c4800
    Lifted::new(0x7000bb08f0, fn_7000bb08f0 as *const (), 0), // +0x4c48f0 sub_4c48f0
    Lifted::new(0x7000bb0980, fn_7000bb0980 as *const (), 0), // +0x4c4980 sub_4c4980
    Lifted::new(0x7000bb09d0, fn_7000bb09d0 as *const (), 0), // +0x4c49d0 sub_4c49d0
    Lifted::new(0x7000bb0a20, fn_7000bb0a20 as *const (), 2), // +0x4c4a20 sub_4c4a20
    Lifted::new(0x7000bb0bc0, fn_7000bb0bc0 as *const (), 2), // +0x4c4bc0 sub_4c4bc0
    Lifted::new(0x7000bb1030, fn_7000bb1030 as *const (), 0), // +0x4c5030 sub_4c5030
    Lifted::new(0x7000bb14b0, fn_7000bb14b0 as *const (), 0), // +0x4c54b0 sub_4c54b0
    Lifted::new(0x7000bb1930, tpl_287 as *const (), 1), // +0x4c5930 sub_4c5930
    Lifted::new(0x7000bb1c50, tpl_287 as *const (), 1), // +0x4c5c50 sub_4c5c50
    Lifted::new(0x7000bb1f70, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x4c5f70 sub_4c5f70
    Lifted::new(0x7000bb1fe0, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x4c5fe0 sub_4c5fe0
    Lifted::new(0x7000bb2050, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x4c6050 sub_4c6050
    Lifted::new(0x7000bb20c0, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x4c60c0 sub_4c60c0
    Lifted::new(0x7000bb2130, fn_7000bb2130 as *const (), 0), // +0x4c6130 sub_4c6130
    Lifted::new(0x7000bb2190, fn_7000bb2190 as *const (), 1), // +0x4c6190 sub_4c6190
    Lifted::new(0x7000bb21e0, fn_7000bb21e0 as *const (), 0), // +0x4c61e0 sub_4c61e0
    Lifted::new(0x7000bb2200, fn_7000bb2200 as *const (), 0), // +0x4c6200 sub_4c6200
    Lifted::new(0x7000bb24e0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4c64e0 sub_4c64e0
    Lifted::new(0x7000bb2520, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4c6520 sub_4c6520
    Lifted::new(0x7000bb2560, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4c6560 sub_4c6560
    Lifted::new(0x7000bb25a0, tpl_924::<0x7001779e20, 0x70006f2a70, 0x70006f9a60, 0x700177c312, 0x7001d54968, 0x7000bb27b6, 0x7000bcd9a0, 0x7000bb27bc, 0x70007f0910, 0x7000bb27c2> as *const (), 0), // +0x4c65a0 sub_4c65a0
    Lifted::new(0x7000bb2850, tpl_388::<0x7000bcda70, 0x700177c312, 0x7001d54968, 0x7000bb29da, 0x7000bcd980, 0x7000bb29e0> as *const (), 2), // +0x4c6850 sub_4c6850
    Lifted::new(0x7000bb2a50, tpl_906::<0x7000bcdb00, 0x700177c312, 0x7001d54968, 0x7000bb2bd4, 0x7000bcd9c0, 0x7000bb2bda> as *const (), 0), // +0x4c6a50 sub_4c6a50
    Lifted::new(0x7000bb2c40, fn_7000bb2c40 as *const (), 0), // +0x4c6c40 sub_4c6c40
    Lifted::new(0x7000bb2d30, fn_7000bb2d30 as *const (), 0), // +0x4c6d30 sub_4c6d30
    Lifted::new(0x7000bb2e30, fn_7000bb2e30 as *const (), 0), // +0x4c6e30 sub_4c6e30
    Lifted::new(0x7000bb30b0, fn_7000bb30b0 as *const (), 0), // +0x4c70b0 sub_4c70b0
    Lifted::new(0x7000bb32b0, fn_7000bb32b0 as *const (), 0), // +0x4c72b0 sub_4c72b0
    Lifted::new(0x7000bb3580, tpl_898::<0x700177c0e0> as *const (), 0), // +0x4c7580 sub_4c7580
    Lifted::new(0x7000bb35c0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x4c75c0 sub_4c75c0
    Lifted::new(0x7000bb3600, tpl_392::<0x700177c0e0> as *const (), 0), // +0x4c7600 sub_4c7600
    Lifted::new(0x7000bb3630, tpl_392::<0x700177c0e0> as *const (), 0), // +0x4c7630 sub_4c7630
    Lifted::new(0x7000bb3680, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000bcc5a0, 0x70017d37c0, 0x70017790e0, 0x7000bb3961> as *const (), 2), // +0x4c7680 sub_4c7680
    Lifted::new(0x7000bb3a20, fn_7000bb3a20 as *const (), 2), // +0x4c7a20 sub_4c7a20
    Lifted::new(0x7000bb3e30, fn_7000bb3e30 as *const (), 2), // +0x4c7e30 sub_4c7e30
    Lifted::new(0x7000bb41b0, tpl_395::<0x7000bcd9e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000bb42f5, 0x7000bcd960, 0x7000bb42fb> as *const (), 0), // +0x4c81b0 sub_4c81b0
    Lifted::new(0x7000bb4350, tpl_68 as *const (), 1), // +0x4c8350 sub_4c8350
    Lifted::new(0x7000bb4400, tpl_68 as *const (), 1), // +0x4c8400 sub_4c8400
    Lifted::new(0x7000bb44b0, tpl_68 as *const (), 1), // +0x4c84b0 sub_4c84b0
    Lifted::new(0x7000bb4560, tpl_68 as *const (), 1), // +0x4c8560 sub_4c8560
    Lifted::new(0x7000bb4610, fn_7000bb4610 as *const (), 2), // +0x4c8610 sub_4c8610
    Lifted::new(0x7000bb47e0, fn_7000bb47e0 as *const (), 0), // +0x4c87e0 sub_4c87e0
    Lifted::new(0x7000bb4830, fn_7000bb4830 as *const (), 0), // +0x4c8830 sub_4c8830
    Lifted::new(0x7000bb48a0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4c88a0 sub_4c88a0
    Lifted::new(0x7000bb48e0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4c88e0 sub_4c88e0
    Lifted::new(0x7000bb49e0, fn_7000bb49e0 as *const (), 0), // +0x4c89e0 sub_4c89e0
    Lifted::new(0x7000bb4af0, fn_7000bb4af0 as *const (), 1), // +0x4c8af0 sub_4c8af0
    Lifted::new(0x7000bb51a0, fn_7000bb51a0 as *const (), 1), // +0x4c91a0 sub_4c91a0
    Lifted::new(0x7000bb58a0, fn_7000bb58a0 as *const (), 1), // +0x4c98a0 sub_4c98a0
    Lifted::new(0x7000bb6230, fn_7000bb6230 as *const (), 2), // +0x4ca230 sub_4ca230
    Lifted::new(0x7000bb7070, fn_7000bb7070 as *const (), 0), // +0x4cb070 sub_4cb070
    Lifted::new(0x7000bb70c0, fn_7000bb70c0 as *const (), 0), // +0x4cb0c0 sub_4cb0c0
    Lifted::new(0x7000bb7180, fn_7000bb7180 as *const (), 0), // +0x4cb180 sub_4cb180
    Lifted::new(0x7000bb7300, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000bb73a3> as *const (), 0), // +0x4cb300 sub_4cb300
    Lifted::new(0x7000bb73d0, tpl_965::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000bb7a70, 0x7001d54968, 0x7000bb7430> as *const (), 0), // +0x4cb3d0 sub_4cb3d0
    Lifted::new(0x7000bb7450, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000bb74f3> as *const (), 0), // +0x4cb450 sub_4cb450
    Lifted::new(0x7000bb7520, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000bb75c3> as *const (), 0), // +0x4cb520 sub_4cb520
    Lifted::new(0x7000bb7620, fn_7000bb7620 as *const (), 0), // +0x4cb620 sub_4cb620
    Lifted::new(0x7000bb7700, fn_7000bb7700 as *const (), 0), // +0x4cb700 sub_4cb700
    Lifted::new(0x7000bb7780, fn_7000bb7780 as *const (), 0), // +0x4cb780 sub_4cb780
    Lifted::new(0x7000bb7820, tpl_603::<0x20, 0xffffffffffffffff, 0x18, 0x10, 0x8, 0x700177c312, 0xfffffffffffffff8, 0x7001d54968, 0x7000bb78e3> as *const (), 0), // +0x4cb820 sub_4cb820
    Lifted::new(0x7000bb7920, fn_7000bb7920 as *const (), 0), // +0x4cb920 sub_4cb920
    Lifted::new(0x7000bb7a70, tpl_966::<0x8, 0x18, 0x7000751790, 0x700177c312> as *const (), 0), // +0x4cba70 sub_4cba70
    Lifted::new(0x7000bb7b10, fn_7000bb7b10 as *const (), 0), // +0x4cbb10 sub_4cbb10
    Lifted::new(0x7000bb7b70, fn_7000bb7b70 as *const (), 0), // +0x4cbb70 sub_4cbb70
    Lifted::new(0x7000bb8580, fn_7000bb8580 as *const (), 1), // +0x4cc580 sub_4cc580
    Lifted::new(0x7000bb85b0, fn_7000bb85b0 as *const (), 0), // +0x4cc5b0 sub_4cc5b0
    Lifted::new(0x7000bb8630, tpl_967::<0x88, 0x4, 0x70017d94c8, 0xffffffffffffff78, 0xffffffffffffff80, 0x70017d9438, 0xffffffffffffff74, 0x7000bcd500, 0x7001d54470, 0xffffffffffffff88, 0x7001d54460, 0x7001d54468> as *const (), 0), // +0x4cc630 sub_4cc630
    Lifted::new(0x7000bb86d0, tpl_967::<0x98, 0x4, 0x7001805548, 0xffffffffffffff68, 0xffffffffffffff80, 0x70017d9438, 0xffffffffffffff64, 0x7000bcd500, 0x7001d54470, 0xffffffffffffff88, 0x7001d542a8, 0x7001d54468> as *const (), 0), // +0x4cc6d0 sub_4cc6d0
    Lifted::new(0x7000bb8770, tpl_604::<0xfffffffffffffffc, 0x7000bb8860> as *const (), 0), // +0x4cc770 sub_4cc770
    Lifted::new(0x7000bb8780, tpl_604::<0xfffffffffffffffc, 0x7000bb88b0> as *const (), 0), // +0x4cc780 sub_4cc780
    Lifted::new(0x7000bb8790, tpl_35::<0x7001805908, 0x700177c312> as *const (), 0), // +0x4cc790 sub_4cc790
    Lifted::new(0x7000bb87c0, tpl_35::<0x7001805d98, 0x700177c312> as *const (), 0), // +0x4cc7c0 sub_4cc7c0
    Lifted::new(0x7000bb87f0, tpl_35::<0x7001805930, 0x700177c312> as *const (), 0), // +0x4cc7f0 sub_4cc7f0
    Lifted::new(0x7000bb8820, tpl_133::<0x7000bb7620, 0x700177c312> as *const (), 0), // +0x4cc820 sub_4cc820
    Lifted::new(0x7000bb8860, tpl_968::<0xffffffffffffff58, 0x7000bb85b0, 0x700177c312> as *const (), 0), // +0x4cc860 sub_4cc860
    Lifted::new(0x7000bb88b0, tpl_968::<0xffffffffffffff68, 0x7000bb86d0, 0x700177c312> as *const (), 0), // +0x4cc8b0 sub_4cc8b0
    Lifted::new(0x7000bb8900, tpl_33::<0x700177c312> as *const (), 0), // +0x4cc900 sub_4cc900
    Lifted::new(0x7000bb8930, fn_7000bb8930 as *const (), 0), // +0x4cc930 sub_4cc930
    Lifted::new(0x7000bb8980, tpl_33::<0x700177c312> as *const (), 0), // +0x4cc980 sub_4cc980
    Lifted::new(0x7000bb89b0, tpl_33::<0x700177c312> as *const (), 0), // +0x4cc9b0 sub_4cc9b0
    Lifted::new(0x7000bb89e0, tpl_969::<0x7001804af0, 0x8, 0x700177c312> as *const (), 0), // +0x4cc9e0 sub_4cc9e0
    Lifted::new(0x7000bb8a30, tpl_33::<0x700177c312> as *const (), 0), // +0x4cca30 sub_4cca30
    Lifted::new(0x7000bb8a60, tpl_133::<0x7000758f80, 0x700177c312> as *const (), 0), // +0x4cca60 sub_4cca60
    Lifted::new(0x7000bb8aa0, fn_7000bb8aa0 as *const (), 0), // +0x4ccaa0 sub_4ccaa0
    Lifted::new(0x7000bb8b10, tpl_133::<0x7000fbcfb0, 0x700177c312> as *const (), 0), // +0x4ccb10 sub_4ccb10
    Lifted::new(0x7000bb8b50, tpl_575::<0xd758, 0x700075ce50> as *const (), 0), // +0x4ccb50 sub_4ccb50
    Lifted::new(0x7000bb8b70, fn_7000bb8b70 as *const (), 0), // +0x4ccb70 sub_4ccb70
    Lifted::new(0x7000bb8bc0, fn_7000bb8bc0 as *const (), 0), // +0x4ccbc0 sub_4ccbc0
    Lifted::new(0x7000bb8c10, tpl_15 as *const (), 1), // +0x4ccc10 sub_4ccc10
    Lifted::new(0x7000bb8c20, fn_7000bb8c20 as *const (), 2), // +0x4ccc20 sub_4ccc20
    Lifted::new(0x7000bb8e70, fn_7000bb8e70 as *const (), 0), // +0x4cce70 sub_4cce70
    Lifted::new(0x7000bb8ea0, fn_7000bb8ea0 as *const (), 0), // +0x4ccea0 sub_4ccea0
    Lifted::new(0x7000bb90d0, fn_7000bb90d0 as *const (), 0), // +0x4cd0d0 sub_4cd0d0
    Lifted::new(0x7000bb91f0, fn_7000bb91f0 as *const (), 0), // +0x4cd1f0 sub_4cd1f0
    Lifted::new(0x7000bb92b0, fn_7000bb92b0 as *const (), 0), // +0x4cd2b0 sub_4cd2b0
    Lifted::new(0x7000bb9460, fn_7000bb9460 as *const (), 0), // +0x4cd460 sub_4cd460
    Lifted::new(0x7000bb94f0, fn_7000bb94f0 as *const (), 0), // +0x4cd4f0 sub_4cd4f0
    Lifted::new(0x7000bb9580, fn_7000bb9580 as *const (), 0), // +0x4cd580 sub_4cd580
    Lifted::new(0x7000bb9980, fn_7000bb9980 as *const (), 2), // +0x4cd980 sub_4cd980
    Lifted::new(0x7000bb9e50, fn_7000bb9e50 as *const (), 0), // +0x4cde50 sub_4cde50
    Lifted::new(0x7000bb9ff0, fn_7000bb9ff0 as *const (), 0), // +0x4cdff0 sub_4cdff0
    Lifted::new(0x7000bba1e0, fn_7000bba1e0 as *const (), 3), // +0x4ce1e0 sub_4ce1e0
    Lifted::new(0x7000bba290, fn_7000bba290 as *const (), 0), // +0x4ce290 sub_4ce290
    Lifted::new(0x7000bba2b0, fn_7000bba2b0 as *const (), 0), // +0x4ce2b0 sub_4ce2b0
    Lifted::new(0x7000bba300, tpl_970::<0x7001805a98, 0x7000bbe5a0> as *const (), 2), // +0x4ce300 sub_4ce300
    Lifted::new(0x7000bba350, fn_7000bba350 as *const (), 1), // +0x4ce350 sub_4ce350
    Lifted::new(0x7000bba3b0, fn_7000bba3b0 as *const (), 1), // +0x4ce3b0 sub_4ce3b0
    Lifted::new(0x7000bba440, fn_7000bba440 as *const (), 2), // +0x4ce440 sub_4ce440
    Lifted::new(0x7000bba5a0, fn_7000bba5a0 as *const (), 0), // +0x4ce5a0 sub_4ce5a0
    Lifted::new(0x7000bba670, fn_7000bba670 as *const (), 0), // +0x4ce670 sub_4ce670
    Lifted::new(0x7000bbb040, fn_7000bbb040 as *const (), 2), // +0x4cf040 sub_4cf040
    Lifted::new(0x7000bbb0f0, fn_7000bbb0f0 as *const (), 2), // +0x4cf0f0 sub_4cf0f0
    Lifted::new(0x7000bbb1d0, fn_7000bbb1d0 as *const (), 2), // +0x4cf1d0 sub_4cf1d0
    Lifted::new(0x7000bbbd90, fn_7000bbbd90 as *const (), 2), // +0x4cfd90 sub_4cfd90
    Lifted::new(0x7000bbc140, tpl_15 as *const (), 1), // +0x4d0140 sub_4d0140
    Lifted::new(0x7000bbc150, fn_7000bbc150 as *const (), 0), // +0x4d0150 sub_4d0150
    Lifted::new(0x7000bbc2a0, tpl_430::<0x7000c46870> as *const (), 0), // +0x4d02a0 sub_4d02a0
    Lifted::new(0x7000bbc2b0, fn_7000bbc2b0 as *const (), 0), // +0x4d02b0 sub_4d02b0
    Lifted::new(0x7000bbc330, fn_7000bbc330 as *const (), 2), // +0x4d0330 sub_4d0330
    Lifted::new(0x7000bbc3d0, tpl_971::<0x7001805958, 0x7000bbe5a0> as *const (), 2), // +0x4d03d0 sub_4d03d0
    Lifted::new(0x7000bbc440, tpl_971::<0x7001805998, 0x7000bbe5a0> as *const (), 2), // +0x4d0440 sub_4d0440
    Lifted::new(0x7000bbc4b0, fn_7000bbc4b0 as *const (), 2), // +0x4d04b0 sub_4d04b0
    Lifted::new(0x7000bbc520, fn_7000bbc520 as *const (), 2), // +0x4d0520 sub_4d0520
    Lifted::new(0x7000bbc5c0, fn_7000bbc5c0 as *const (), 2), // +0x4d05c0 sub_4d05c0
    Lifted::new(0x7000bbc640, fn_7000bbc640 as *const (), 2), // +0x4d0640 sub_4d0640
    Lifted::new(0x7000bbc6b0, tpl_970::<0x7001805c98, 0x7000bbe5a0> as *const (), 2), // +0x4d06b0 sub_4d06b0
    Lifted::new(0x7000bbc700, tpl_970::<0x7001805b98, 0x7000bbe5a0> as *const (), 2), // +0x4d0700 sub_4d0700
    Lifted::new(0x7000bbc750, tpl_972::<0x7001805c58, 0x7000bbe5a0> as *const (), 0), // +0x4d0750 sub_4d0750
    Lifted::new(0x7000bbc790, fn_7000bbc790 as *const (), 2), // +0x4d0790 sub_4d0790
    Lifted::new(0x7000bbcc00, fn_7000bbcc00 as *const (), 2), // +0x4d0c00 sub_4d0c00
    Lifted::new(0x7000bbcee0, fn_7000bbcee0 as *const (), 2), // +0x4d0ee0 sub_4d0ee0
    Lifted::new(0x7000bbd1a0, fn_7000bbd1a0 as *const (), 0), // +0x4d11a0 sub_4d11a0
    Lifted::new(0x7000bbd3b0, fn_7000bbd3b0 as *const (), 0), // +0x4d13b0 sub_4d13b0
    Lifted::new(0x7000bbd470, fn_7000bbd470 as *const (), 2), // +0x4d1470 sub_4d1470
    Lifted::new(0x7000bbd890, fn_7000bbd890 as *const (), 2), // +0x4d1890 sub_4d1890
    Lifted::new(0x7000bbda40, fn_7000bbda40 as *const (), 0), // +0x4d1a40 sub_4d1a40
    Lifted::new(0x7000bbdb30, tpl_455::<0x7001c71e58> as *const (), 1), // +0x4d1b30 sub_4d1b30
    Lifted::new(0x7000bbdb40, tpl_450::<0x7001c71e64> as *const (), 1), // +0x4d1b40 sub_4d1b40
    Lifted::new(0x7000bbdb50, tpl_455::<0x7001c71e68> as *const (), 1), // +0x4d1b50 sub_4d1b50
    Lifted::new(0x7000bbdb60, tpl_450::<0x7001c71e74> as *const (), 1), // +0x4d1b60 sub_4d1b60
    Lifted::new(0x7000bbdb70, fn_7000bbdb70 as *const (), 1), // +0x4d1b70 sub_4d1b70
    Lifted::new(0x7000bbdb90, fn_7000bbdb90 as *const (), 0), // +0x4d1b90 sub_4d1b90
    Lifted::new(0x7000bbdc00, fn_7000bbdc00 as *const (), 0), // +0x4d1c00 sub_4d1c00
    Lifted::new(0x7000bbe170, fn_7000bbe170 as *const (), 0), // +0x4d2170 sub_4d2170
    Lifted::new(0x7000bbe270, tpl_973::<0x8, 0xc, 0x700177c312> as *const (), 0), // +0x4d2270 sub_4d2270
    Lifted::new(0x7000bbe2f0, fn_7000bbe2f0 as *const (), 0), // +0x4d22f0 sub_4d22f0
    Lifted::new(0x7000bbe340, fn_7000bbe340 as *const (), 2), // +0x4d2340 sub_4d2340
    Lifted::new(0x7000bbe3e0, fn_7000bbe3e0 as *const (), 0), // +0x4d23e0 sub_4d23e0
    Lifted::new(0x7000bbe5a0, fn_7000bbe5a0 as *const (), 0), // +0x4d25a0 sub_4d25a0
    Lifted::new(0x7000bbe670, fn_7000bbe670 as *const (), 2), // +0x4d2670 sub_4d2670
    Lifted::new(0x7000bbe9c0, fn_7000bbe9c0 as *const (), 2), // +0x4d29c0 sub_4d29c0
    Lifted::new(0x7000bbf210, fn_7000bbf210 as *const (), 0), // +0x4d3210 sub_4d3210
    Lifted::new(0x7000bbf310, fn_7000bbf310 as *const (), 0), // +0x4d3310 sub_4d3310
    Lifted::new(0x7000bbf3a0, fn_7000bbf3a0 as *const (), 0), // +0x4d33a0 sub_4d33a0
    Lifted::new(0x7000bbf3d0, fn_7000bbf3d0 as *const (), 0), // +0x4d33d0 sub_4d33d0
    Lifted::new(0x7000bbf500, fn_7000bbf500 as *const (), 0), // +0x4d3500 sub_4d3500
    Lifted::new(0x7000bbf570, fn_7000bbf570 as *const (), 0), // +0x4d3570 sub_4d3570
    Lifted::new(0x7000bbf5e0, fn_7000bbf5e0 as *const (), 0), // +0x4d35e0 sub_4d35e0
    Lifted::new(0x7000bbf7d0, fn_7000bbf7d0 as *const (), 2), // +0x4d37d0 sub_4d37d0
    Lifted::new(0x7000bbf970, fn_7000bbf970 as *const (), 0), // +0x4d3970 sub_4d3970
    Lifted::new(0x7000bbfa60, fn_7000bbfa60 as *const (), 2), // +0x4d3a60 sub_4d3a60
    Lifted::new(0x7000bbfd30, tpl_36::<0xd9f0> as *const (), 1), // +0x4d3d30 sub_4d3d30
    Lifted::new(0x7000bbfd40, fn_7000bbfd40 as *const (), 2), // +0x4d3d40 sub_4d3d40
    Lifted::new(0x7000bbfea0, fn_7000bbfea0 as *const (), 0), // +0x4d3ea0 sub_4d3ea0
    Lifted::new(0x7000bc0010, fn_7000bc0010 as *const (), 1), // +0x4d4010 sub_4d4010
    Lifted::new(0x7000bc0050, fn_7000bc0050 as *const (), 0), // +0x4d4050 sub_4d4050
    Lifted::new(0x7000bc0160, fn_7000bc0160 as *const (), 0), // +0x4d4160 sub_4d4160
    Lifted::new(0x7000bc0190, fn_7000bc0190 as *const (), 0), // +0x4d4190 sub_4d4190
    Lifted::new(0x7000bc01b0, fn_7000bc01b0 as *const (), 2), // +0x4d41b0 sub_4d41b0
    Lifted::new(0x7000bc0860, fn_7000bc0860 as *const (), 0), // +0x4d4860 sub_4d4860
    Lifted::new(0x7000bc09e0, fn_7000bc09e0 as *const (), 1), // +0x4d49e0 sub_4d49e0
    Lifted::new(0x7000bc0a40, fn_7000bc0a40 as *const (), 0), // +0x4d4a40 sub_4d4a40
    Lifted::new(0x7000bc0c30, fn_7000bc0c30 as *const (), 0), // +0x4d4c30 sub_4d4c30
    Lifted::new(0x7000bc0f50, fn_7000bc0f50 as *const (), 2), // +0x4d4f50 sub_4d4f50
    Lifted::new(0x7000bc1170, fn_7000bc1170 as *const (), 0), // +0x4d5170 sub_4d5170
    Lifted::new(0x7000bc13b0, fn_7000bc13b0 as *const (), 2), // +0x4d53b0 sub_4d53b0
    Lifted::new(0x7000bc14d0, tpl_970::<0x7001805ad8, 0x7000bbe5a0> as *const (), 2), // +0x4d54d0 sub_4d54d0
    Lifted::new(0x7000bc1520, tpl_172::<0xd970> as *const (), 1), // +0x4d5520 sub_4d5520
    Lifted::new(0x7000bc1530, fn_7000bc1530 as *const (), 0), // +0x4d5530 sub_4d5530
    Lifted::new(0x7000bc15f0, fn_7000bc15f0 as *const (), 0), // +0x4d55f0 sub_4d55f0
    Lifted::new(0x7000bc1650, fn_7000bc1650 as *const (), 2), // +0x4d5650 sub_4d5650
    Lifted::new(0x7000bc2b90, fn_7000bc2b90 as *const (), 0), // +0x4d6b90 sub_4d6b90
    Lifted::new(0x7000bc2bc0, fn_7000bc2bc0 as *const (), 0), // +0x4d6bc0 sub_4d6bc0
    Lifted::new(0x7000bc2c60, fn_7000bc2c60 as *const (), 0), // +0x4d6c60 sub_4d6c60
    Lifted::new(0x7000bc2e10, fn_7000bc2e10 as *const (), 2), // +0x4d6e10 sub_4d6e10
    Lifted::new(0x7000bc2fc0, fn_7000bc2fc0 as *const (), 0), // +0x4d6fc0 sub_4d6fc0
    Lifted::new(0x7000bc3020, fn_7000bc3020 as *const (), 0), // +0x4d7020 sub_4d7020
    Lifted::new(0x7000bc3050, fn_7000bc3050 as *const (), 0), // +0x4d7050 sub_4d7050
    Lifted::new(0x7000bc30a0, fn_7000bc30a0 as *const (), 0), // +0x4d70a0 sub_4d70a0
    Lifted::new(0x7000bc30f0, fn_7000bc30f0 as *const (), 0), // +0x4d70f0 sub_4d70f0
    Lifted::new(0x7000bc3220, fn_7000bc3220 as *const (), 0), // +0x4d7220 sub_4d7220
    Lifted::new(0x7000bc3280, tpl_144 as *const (), 1), // +0x4d7280 sub_4d7280
    Lifted::new(0x7000bc3290, fn_7000bc3290 as *const (), 2), // +0x4d7290 sub_4d7290
    Lifted::new(0x7000bc34c0, tpl_47::<0x20> as *const (), 1), // +0x4d74c0 sub_4d74c0
    Lifted::new(0x7000bc34d0, fn_7000bc34d0 as *const (), 1), // +0x4d74d0 sub_4d74d0
    Lifted::new(0x7000bc3570, fn_7000bc3570 as *const (), 1), // +0x4d7570 sub_4d7570
    Lifted::new(0x7000bc3590, fn_7000bc3590 as *const (), 0), // +0x4d7590 sub_4d7590
    Lifted::new(0x7000bc3620, fn_7000bc3620 as *const (), 1), // +0x4d7620 sub_4d7620
    Lifted::new(0x7000bc3650, fn_7000bc3650 as *const (), 1), // +0x4d7650 sub_4d7650
    Lifted::new(0x7000bc36a0, fn_7000bc36a0 as *const (), 1), // +0x4d76a0 sub_4d76a0
    Lifted::new(0x7000bc3720, fn_7000bc3720 as *const (), 1), // +0x4d7720 sub_4d7720
    Lifted::new(0x7000bc3770, fn_7000bc3770 as *const (), 0), // +0x4d7770 sub_4d7770
    Lifted::new(0x7000bc38c0, fn_7000bc38c0 as *const (), 0), // +0x4d78c0 sub_4d78c0
    Lifted::new(0x7000bc3900, fn_7000bc3900 as *const (), 0), // +0x4d7900 sub_4d7900
    Lifted::new(0x7000bc3a20, fn_7000bc3a20 as *const (), 0), // +0x4d7a20 sub_4d7a20
    Lifted::new(0x7000bc3a40, fn_7000bc3a40 as *const (), 0), // +0x4d7a40 sub_4d7a40
    Lifted::new(0x7000bc3a60, tpl_168::<0xc7e5> as *const (), 1), // +0x4d7a60 sub_4d7a60
    Lifted::new(0x7000bc3a70, fn_7000bc3a70 as *const (), 0), // +0x4d7a70 sub_4d7a70
    Lifted::new(0x7000bc3aa0, tpl_974::<0x2fd8, 0x700076e3f0> as *const (), 0), // +0x4d7aa0 sub_4d7aa0
    Lifted::new(0x7000bc3ab0, fn_7000bc3ab0 as *const (), 0), // +0x4d7ab0 sub_4d7ab0
    Lifted::new(0x7000bc3c20, fn_7000bc3c20 as *const (), 0), // +0x4d7c20 sub_4d7c20
    Lifted::new(0x7000bc3c40, fn_7000bc3c40 as *const (), 0), // +0x4d7c40 sub_4d7c40
    Lifted::new(0x7000bc3c80, fn_7000bc3c80 as *const (), 2), // +0x4d7c80 sub_4d7c80
    Lifted::new(0x7000bc3ce0, fn_7000bc3ce0 as *const (), 0), // +0x4d7ce0 sub_4d7ce0
    Lifted::new(0x7000bc3d70, fn_7000bc3d70 as *const (), 1), // +0x4d7d70 sub_4d7d70
    Lifted::new(0x7000bc3e30, fn_7000bc3e30 as *const (), 0), // +0x4d7e30 sub_4d7e30
    Lifted::new(0x7000bc3e50, fn_7000bc3e50 as *const (), 0), // +0x4d7e50 sub_4d7e50
    Lifted::new(0x7000bc40d0, fn_7000bc40d0 as *const (), 2), // +0x4d80d0 sub_4d80d0
    Lifted::new(0x7000bc4430, fn_7000bc4430 as *const (), 0), // +0x4d8430 sub_4d8430
    Lifted::new(0x7000bc44d0, tpl_970::<0x7001805c18, 0x7000bbe5a0> as *const (), 2), // +0x4d84d0 sub_4d84d0
    Lifted::new(0x7000bc4520, fn_7000bc4520 as *const (), 0), // +0x4d8520 sub_4d8520
    Lifted::new(0x7000bc4560, fn_7000bc4560 as *const (), 0), // +0x4d8560 sub_4d8560
    Lifted::new(0x7000bc45f0, fn_7000bc45f0 as *const (), 0), // +0x4d85f0 sub_4d85f0
    Lifted::new(0x7000bc4600, fn_7000bc4600 as *const (), 0), // +0x4d8600 sub_4d8600
    Lifted::new(0x7000bc4670, fn_7000bc4670 as *const (), 0), // +0x4d8670 sub_4d8670
    Lifted::new(0x7000bc4740, fn_7000bc4740 as *const (), 0), // +0x4d8740 sub_4d8740
    Lifted::new(0x7000bc4780, tpl_144 as *const (), 1), // +0x4d8780 sub_4d8780
    Lifted::new(0x7000bc4790, fn_7000bc4790 as *const (), 0), // +0x4d8790 sub_4d8790
    Lifted::new(0x7000bc4850, fn_7000bc4850 as *const (), 0), // +0x4d8850 sub_4d8850
    Lifted::new(0x7000bc49a0, fn_7000bc49a0 as *const (), 0), // +0x4d89a0 sub_4d89a0
    Lifted::new(0x7000bc4a00, fn_7000bc4a00 as *const (), 2), // +0x4d8a00 sub_4d8a00
    Lifted::new(0x7000bc4e70, fn_7000bc4e70 as *const (), 0), // +0x4d8e70 sub_4d8e70
    Lifted::new(0x7000bc5290, fn_7000bc5290 as *const (), 0), // +0x4d9290 sub_4d9290
    Lifted::new(0x7000bc52d0, fn_7000bc52d0 as *const (), 0), // +0x4d92d0 sub_4d92d0
    Lifted::new(0x7000bc5300, fn_7000bc5300 as *const (), 0), // +0x4d9300 sub_4d9300
    Lifted::new(0x7000bc5350, fn_7000bc5350 as *const (), 2), // +0x4d9350 sub_4d9350
    Lifted::new(0x7000bc5570, fn_7000bc5570 as *const (), 0), // +0x4d9570 sub_4d9570
    Lifted::new(0x7000bc5610, fn_7000bc5610 as *const (), 0), // +0x4d9610 sub_4d9610
    Lifted::new(0x7000bc57d0, fn_7000bc57d0 as *const (), 0), // +0x4d97d0 sub_4d97d0
    Lifted::new(0x7000bc5970, fn_7000bc5970 as *const (), 2), // +0x4d9970 sub_4d9970
    Lifted::new(0x7000bc5a00, fn_7000bc5a00 as *const (), 0), // +0x4d9a00 sub_4d9a00
    Lifted::new(0x7000bc5e00, fn_7000bc5e00 as *const (), 0), // +0x4d9e00 sub_4d9e00
    Lifted::new(0x7000bc6180, fn_7000bc6180 as *const (), 0), // +0x4da180 sub_4da180
    Lifted::new(0x7000bc61f0, fn_7000bc61f0 as *const (), 1), // +0x4da1f0 sub_4da1f0
    Lifted::new(0x7000bc6230, fn_7000bc6230 as *const (), 0), // +0x4da230 sub_4da230
    Lifted::new(0x7000bc63b0, fn_7000bc63b0 as *const (), 0), // +0x4da3b0 sub_4da3b0
    Lifted::new(0x7000bc6450, fn_7000bc6450 as *const (), 0), // +0x4da450 sub_4da450
    Lifted::new(0x7000bc64c0, fn_7000bc64c0 as *const (), 1), // +0x4da4c0 sub_4da4c0
    Lifted::new(0x7000bc64e0, tpl_970::<0x7001805b58, 0x7000bbe5a0> as *const (), 2), // +0x4da4e0 sub_4da4e0
    Lifted::new(0x7000bc6530, fn_7000bc6530 as *const (), 0), // +0x4da530 sub_4da530
    Lifted::new(0x7000bc65b0, tpl_51::<0x2fd8> as *const (), 1), // +0x4da5b0 sub_4da5b0
    Lifted::new(0x7000bc65c0, fn_7000bc65c0 as *const (), 0), // +0x4da5c0 sub_4da5c0
    Lifted::new(0x7000bc6680, fn_7000bc6680 as *const (), 0), // +0x4da680 sub_4da680
    Lifted::new(0x7000bc67d0, fn_7000bc67d0 as *const (), 0), // +0x4da7d0 sub_4da7d0
    Lifted::new(0x7000bc6830, fn_7000bc6830 as *const (), 0), // +0x4da830 sub_4da830
    Lifted::new(0x7000bc68d0, tpl_975::<0xd780, 0xd778, 0x8, 0x700177c0d0> as *const (), 0), // +0x4da8d0 sub_4da8d0
    Lifted::new(0x7000bc6940, tpl_975::<0xd798, 0xd790, 0x8, 0x700177c0d0> as *const (), 0), // +0x4da940 sub_4da940
    Lifted::new(0x7000bc69b0, tpl_15 as *const (), 1), // +0x4da9b0 sub_4da9b0
    Lifted::new(0x7000bc69c0, fn_7000bc69c0 as *const (), 0), // +0x4da9c0 sub_4da9c0
    Lifted::new(0x7000bc6b80, fn_7000bc6b80 as *const (), 0), // +0x4dab80 sub_4dab80
    Lifted::new(0x7000bc6e40, fn_7000bc6e40 as *const (), 0), // +0x4dae40 sub_4dae40
    Lifted::new(0x7000bc7060, fn_7000bc7060 as *const (), 0), // +0x4db060 sub_4db060
    Lifted::new(0x7000bc7370, fn_7000bc7370 as *const (), 2), // +0x4db370 sub_4db370
    Lifted::new(0x7000bc73d0, fn_7000bc73d0 as *const (), 0), // +0x4db3d0 sub_4db3d0
    Lifted::new(0x7000bc74f0, tpl_544::<0xd76e> as *const (), 1), // +0x4db4f0 sub_4db4f0
    Lifted::new(0x7000bc7500, tpl_976::<0x18, 0x3030> as *const (), 1), // +0x4db500 sub_4db500
    Lifted::new(0x7000bc7510, fn_7000bc7510 as *const (), 3), // +0x4db510 sub_4db510
    Lifted::new(0x7000bc75c0, fn_7000bc75c0 as *const (), 0), // +0x4db5c0 sub_4db5c0
    Lifted::new(0x7000bc76f0, fn_7000bc76f0 as *const (), 2), // +0x4db6f0 sub_4db6f0
    Lifted::new(0x7000bc77d0, fn_7000bc77d0 as *const (), 0), // +0x4db7d0 sub_4db7d0
    Lifted::new(0x7000bc7ad0, fn_7000bc7ad0 as *const (), 0), // +0x4dbad0 sub_4dbad0
    Lifted::new(0x7000bc7c50, fn_7000bc7c50 as *const (), 0), // +0x4dbc50 sub_4dbc50
    Lifted::new(0x7000bc7cf0, fn_7000bc7cf0 as *const (), 0), // +0x4dbcf0 sub_4dbcf0
    Lifted::new(0x7000bc7d60, fn_7000bc7d60 as *const (), 2), // +0x4dbd60 sub_4dbd60
    Lifted::new(0x7000bc8280, fn_7000bc8280 as *const (), 0), // +0x4dc280 sub_4dc280
    Lifted::new(0x7000bc82f0, tpl_51::<0xd928> as *const (), 1), // +0x4dc2f0 sub_4dc2f0
    Lifted::new(0x7000bc8300, tpl_49::<0x20> as *const (), 1), // +0x4dc300 sub_4dc300
    Lifted::new(0x7000bc8310, tpl_370::<0xda28, 0x70007a2d10> as *const (), 0), // +0x4dc310 sub_4dc310
    Lifted::new(0x7000bc8320, tpl_255::<0xc798, 0xc7a0> as *const (), 1), // +0x4dc320 sub_4dc320
    Lifted::new(0x7000bc8340, tpl_977::<0xd960, 0x28> as *const (), 0), // +0x4dc340 sub_4dc340
    Lifted::new(0x7000bc8360, fn_7000bc8360 as *const (), 1), // +0x4dc360 sub_4dc360
    Lifted::new(0x7000bc8380, tpl_51::<0xd9f0> as *const (), 1), // +0x4dc380 sub_4dc380
    Lifted::new(0x7000bc8390, fn_7000bc8390 as *const (), 0), // +0x4dc390 sub_4dc390
    Lifted::new(0x7000bc8430, tpl_51::<0x3088> as *const (), 1), // +0x4dc430 sub_4dc430
    Lifted::new(0x7000bc8440, tpl_51::<0x1130> as *const (), 1), // +0x4dc440 sub_4dc440
    Lifted::new(0x7000bc8450, tpl_51::<0xc7b0> as *const (), 1), // +0x4dc450 sub_4dc450
    Lifted::new(0x7000bc8460, tpl_51::<0xc7b8> as *const (), 1), // +0x4dc460 sub_4dc460
    Lifted::new(0x7000bc8470, tpl_51::<0xc7c8> as *const (), 1), // +0x4dc470 sub_4dc470
    Lifted::new(0x7000bc8480, fn_7000bc8480 as *const (), 1), // +0x4dc480 sub_4dc480
    Lifted::new(0x7000bc84c0, fn_7000bc84c0 as *const (), 0), // +0x4dc4c0 sub_4dc4c0
    Lifted::new(0x7000bc8500, tpl_51::<0xd970> as *const (), 1), // +0x4dc500 sub_4dc500
    Lifted::new(0x7000bc8510, fn_7000bc8510 as *const (), 0), // +0x4dc510 sub_4dc510
    Lifted::new(0x7000bc8580, tpl_51::<0xc918> as *const (), 1), // +0x4dc580 sub_4dc580
    Lifted::new(0x7000bc8590, fn_7000bc8590 as *const (), 2), // +0x4dc590 sub_4dc590
    Lifted::new(0x7000bc86c0, tpl_171::<0x21> as *const (), 1), // +0x4dc6c0 sub_4dc6c0
    Lifted::new(0x7000bc86d0, fn_7000bc86d0 as *const (), 1), // +0x4dc6d0 sub_4dc6d0
    Lifted::new(0x7000bc8710, fn_7000bc8710 as *const (), 0), // +0x4dc710 sub_4dc710
    Lifted::new(0x7000bc8830, tpl_51::<0xd7a8> as *const (), 1), // +0x4dc830 sub_4dc830
    Lifted::new(0x7000bc8840, tpl_970::<0x7001805b18, 0x7000bbe5a0> as *const (), 2), // +0x4dc840 sub_4dc840
    Lifted::new(0x7000bc8890, tpl_970::<0x7001805bd8, 0x7000bbe5a0> as *const (), 2), // +0x4dc890 sub_4dc890
    Lifted::new(0x7000bc88e0, fn_7000bc88e0 as *const (), 1), // +0x4dc8e0 sub_4dc8e0
    Lifted::new(0x7000bc88f0, fn_7000bc88f0 as *const (), 0), // +0x4dc8f0 sub_4dc8f0
    Lifted::new(0x7000bc8b90, fn_7000bc8b90 as *const (), 0), // +0x4dcb90 sub_4dcb90
    Lifted::new(0x7000bc8d30, fn_7000bc8d30 as *const (), 1), // +0x4dcd30 sub_4dcd30
    Lifted::new(0x7000bc8d40, fn_7000bc8d40 as *const (), 0), // +0x4dcd40 sub_4dcd40
    Lifted::new(0x7000bc9080, fn_7000bc9080 as *const (), 2), // +0x4dd080 sub_4dd080
    Lifted::new(0x7000bc9240, fn_7000bc9240 as *const (), 0), // +0x4dd240 sub_4dd240
    Lifted::new(0x7000bc9270, fn_7000bc9270 as *const (), 0), // +0x4dd270 sub_4dd270
    Lifted::new(0x7000bc94c0, fn_7000bc94c0 as *const (), 0), // +0x4dd4c0 sub_4dd4c0
    Lifted::new(0x7000bc9530, fn_7000bc9530 as *const (), 0), // +0x4dd530 sub_4dd530
    Lifted::new(0x7000bc9910, fn_7000bc9910 as *const (), 0), // +0x4dd910 sub_4dd910
    Lifted::new(0x7000bc9970, fn_7000bc9970 as *const (), 0), // +0x4dd970 sub_4dd970
    Lifted::new(0x7000bc99d0, fn_7000bc99d0 as *const (), 0), // +0x4dd9d0 sub_4dd9d0
    Lifted::new(0x7000bc9b80, fn_7000bc9b80 as *const (), 0), // +0x4ddb80 sub_4ddb80
    Lifted::new(0x7000bc9c20, tpl_976::<0x8, 0x2fd8> as *const (), 1), // +0x4ddc20 sub_4ddc20
    Lifted::new(0x7000bc9c30, fn_7000bc9c30 as *const (), 0), // +0x4ddc30 sub_4ddc30
    Lifted::new(0x7000bca920, fn_7000bca920 as *const (), 2), // +0x4de920 sub_4de920
    Lifted::new(0x7000bca980, fn_7000bca980 as *const (), 1), // +0x4de980 sub_4de980
    Lifted::new(0x7000bca9c0, fn_7000bca9c0 as *const (), 2), // +0x4de9c0 sub_4de9c0
    Lifted::new(0x7000bcad80, fn_7000bcad80 as *const (), 0), // +0x4ded80 sub_4ded80
    Lifted::new(0x7000bcae30, fn_7000bcae30 as *const (), 0), // +0x4dee30 sub_4dee30
    Lifted::new(0x7000bcaed0, fn_7000bcaed0 as *const (), 0), // +0x4deed0 sub_4deed0
    Lifted::new(0x7000bcaef0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000bcaf01> as *const (), 0), // +0x4deef0 sub_4deef0
    Lifted::new(0x7000bcaf10, tpl_972::<0x7001805d58, 0x7000bbe5a0> as *const (), 0), // +0x4def10 sub_4def10
    Lifted::new(0x7000bcaf50, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000bcb057, 0x7000bb4350, 0x70007f0910, 0x7000bcb073> as *const (), 0), // +0x4def50 sub_4def50
    Lifted::new(0x7000bcb0c0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000bcb1c7, 0x7000bb4400, 0x70007f0910, 0x7000bcb1e3> as *const (), 0), // +0x4df0c0 sub_4df0c0
    Lifted::new(0x7000bcb230, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000bcb337, 0x7000bb44b0, 0x70007f0910, 0x7000bcb353> as *const (), 0), // +0x4df230 sub_4df230
    Lifted::new(0x7000bcb3a0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000bcb4a7, 0x7000bb4560, 0x70007f0910, 0x7000bcb4c3> as *const (), 0), // +0x4df3a0 sub_4df3a0
    Lifted::new(0x7000bcb510, tpl_278::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000bcb59d> as *const (), 0), // +0x4df510 sub_4df510
    Lifted::new(0x7000bcb5d0, tpl_918::<0x7001805c98, 0x8> as *const (), 3), // +0x4df5d0 sub_4df5d0
    Lifted::new(0x7000bcb5f0, tpl_918::<0x7001805bd8, 0x8> as *const (), 3), // +0x4df5f0 sub_4df5f0
    Lifted::new(0x7000bcb610, tpl_918::<0x7001805ad8, 0x8> as *const (), 3), // +0x4df610 sub_4df610
    Lifted::new(0x7000bcb630, tpl_923::<0x7001805d58> as *const (), 1), // +0x4df630 sub_4df630
    Lifted::new(0x7000bcb650, tpl_978::<0x7001805a18, 0x8, 0x18, 0x28> as *const (), 3), // +0x4df650 sub_4df650
    Lifted::new(0x7000bcb690, tpl_918::<0x7001805b58, 0x8> as *const (), 3), // +0x4df690 sub_4df690
    Lifted::new(0x7000bcb6b0, tpl_979::<0x70018059d8, 0x8, 0x18> as *const (), 3), // +0x4df6b0 sub_4df6b0
    Lifted::new(0x7000bcb6e0, tpl_918::<0x7001805b18, 0x8> as *const (), 3), // +0x4df6e0 sub_4df6e0
    Lifted::new(0x7000bcb700, tpl_918::<0x7001805a98, 0x8> as *const (), 3), // +0x4df700 sub_4df700
    Lifted::new(0x7000bcb720, tpl_980::<0x7001805998, 0x8, 0x18> as *const (), 3), // +0x4df720 sub_4df720
    Lifted::new(0x7000bcb750, tpl_918::<0x7001805c18, 0x8> as *const (), 3), // +0x4df750 sub_4df750
    Lifted::new(0x7000bcb770, tpl_980::<0x7001805d18, 0x8, 0x18> as *const (), 3), // +0x4df770 sub_4df770
    Lifted::new(0x7000bcb7a0, tpl_979::<0x7001805cd8, 0x8, 0x18> as *const (), 3), // +0x4df7a0 sub_4df7a0
    Lifted::new(0x7000bcb7d0, tpl_980::<0x7001805958, 0x8, 0x18> as *const (), 3), // +0x4df7d0 sub_4df7d0
    Lifted::new(0x7000bcb800, tpl_918::<0x7001805b98, 0x8> as *const (), 3), // +0x4df800 sub_4df800
    Lifted::new(0x7000bcb820, tpl_923::<0x7001805c58> as *const (), 1), // +0x4df820 sub_4df820
    Lifted::new(0x7000bcb840, tpl_981::<0x7001805a58, 0x8, 0x18, 0x28> as *const (), 3), // +0x4df840 sub_4df840
    Lifted::new(0x7000bcb870, tpl_919::<0x700177c312> as *const (), 0), // +0x4df870 sub_4df870
    Lifted::new(0x7000bcb890, tpl_919::<0x700177c312> as *const (), 0), // +0x4df890 sub_4df890
    Lifted::new(0x7000bcb8b0, tpl_919::<0x700177c312> as *const (), 0), // +0x4df8b0 sub_4df8b0
    Lifted::new(0x7000bcb8d0, tpl_754::<0x700177c312> as *const (), 0), // +0x4df8d0 sub_4df8d0
    Lifted::new(0x7000bcb8f0, tpl_982::<0x700177c312> as *const (), 0), // +0x4df8f0 sub_4df8f0
    Lifted::new(0x7000bcb910, tpl_919::<0x700177c312> as *const (), 0), // +0x4df910 sub_4df910
    Lifted::new(0x7000bcb930, tpl_983::<0x700177c312> as *const (), 0), // +0x4df930 sub_4df930
    Lifted::new(0x7000bcb950, tpl_919::<0x700177c312> as *const (), 0), // +0x4df950 sub_4df950
    Lifted::new(0x7000bcb970, tpl_919::<0x700177c312> as *const (), 0), // +0x4df970 sub_4df970
    Lifted::new(0x7000bcb990, tpl_984::<0x700177c312> as *const (), 0), // +0x4df990 sub_4df990
    Lifted::new(0x7000bcb9b0, tpl_919::<0x700177c312> as *const (), 0), // +0x4df9b0 sub_4df9b0
    Lifted::new(0x7000bcb9d0, tpl_984::<0x700177c312> as *const (), 0), // +0x4df9d0 sub_4df9d0
    Lifted::new(0x7000bcb9f0, tpl_983::<0x700177c312> as *const (), 0), // +0x4df9f0 sub_4df9f0
    Lifted::new(0x7000bcba10, tpl_984::<0x700177c312> as *const (), 0), // +0x4dfa10 sub_4dfa10
    Lifted::new(0x7000bcba30, tpl_919::<0x700177c312> as *const (), 0), // +0x4dfa30 sub_4dfa30
    Lifted::new(0x7000bcba50, tpl_754::<0x700177c312> as *const (), 0), // +0x4dfa50 sub_4dfa50
    Lifted::new(0x7000bcba70, fn_7000bcba70 as *const (), 0), // +0x4dfa70 sub_4dfa70
    Lifted::new(0x7000bcba90, tpl_530::<0x10> as *const (), 0), // +0x4dfa90 sub_4dfa90
    Lifted::new(0x7000bcbab0, tpl_530::<0x10> as *const (), 0), // +0x4dfab0 sub_4dfab0
    Lifted::new(0x7000bcbad0, tpl_530::<0x10> as *const (), 0), // +0x4dfad0 sub_4dfad0
    Lifted::new(0x7000bcbaf0, fn_7000bcbaf0 as *const (), 0), // +0x4dfaf0 sub_4dfaf0
    Lifted::new(0x7000bcbb70, fn_7000bcbb70 as *const (), 2), // +0x4dfb70 sub_4dfb70
    Lifted::new(0x7000bcbc00, fn_7000bcbc00 as *const (), 0), // +0x4dfc00 sub_4dfc00
    Lifted::new(0x7000bcbcf0, tpl_985::<0x10, 0x8, 0x70> as *const (), 0), // +0x4dfcf0 sub_4dfcf0
    Lifted::new(0x7000bcbd10, tpl_985::<0x10, 0x8, 0x58> as *const (), 0), // +0x4dfd10 sub_4dfd10
    Lifted::new(0x7000bcbd30, tpl_985::<0x10, 0x8, 0x38> as *const (), 0), // +0x4dfd30 sub_4dfd30
    Lifted::new(0x7000bcbd50, tpl_986::<0x88> as *const (), 0), // +0x4dfd50 sub_4dfd50
    Lifted::new(0x7000bcbd60, fn_7000bcbd60 as *const (), 0), // +0x4dfd60 sub_4dfd60
    Lifted::new(0x7000bcbdb0, tpl_985::<0x10, 0x8, 0x48> as *const (), 0), // +0x4dfdb0 sub_4dfdb0
    Lifted::new(0x7000bcbdd0, fn_7000bcbdd0 as *const (), 0), // +0x4dfdd0 sub_4dfdd0
    Lifted::new(0x7000bcbe10, tpl_985::<0x10, 0x8, 0x40> as *const (), 0), // +0x4dfe10 sub_4dfe10
    Lifted::new(0x7000bcbe30, tpl_985::<0x10, 0x8, 0x30> as *const (), 0), // +0x4dfe30 sub_4dfe30
    Lifted::new(0x7000bcbe50, tpl_987::<0x18, 0x10, 0x8, 0x20> as *const (), 0), // +0x4dfe50 sub_4dfe50
    Lifted::new(0x7000bcbe70, tpl_985::<0x10, 0x8, 0x60> as *const (), 0), // +0x4dfe70 sub_4dfe70
    Lifted::new(0x7000bcbe90, fn_7000bcbe90 as *const (), 0), // +0x4dfe90 sub_4dfe90
    Lifted::new(0x7000bcbec0, fn_7000bcbec0 as *const (), 0), // +0x4dfec0 sub_4dfec0
    Lifted::new(0x7000bcbf10, tpl_987::<0x18, 0x10, 0x8, 0x28> as *const (), 0), // +0x4dff10 sub_4dff10
    Lifted::new(0x7000bcbf30, tpl_985::<0x10, 0x8, 0x50> as *const (), 0), // +0x4dff30 sub_4dff30
    Lifted::new(0x7000bcbf50, tpl_986::<0x68> as *const (), 0), // +0x4dff50 sub_4dff50
    Lifted::new(0x7000bcbf60, fn_7000bcbf60 as *const (), 2), // +0x4dff60 sub_4dff60
    Lifted::new(0x7000bcbfc0, fn_7000bcbfc0 as *const (), 0), // +0x4dffc0 sub_4dffc0
    Lifted::new(0x7000bcc0f0, fn_7000bcc0f0 as *const (), 0), // +0x4e00f0 sub_4e00f0
    Lifted::new(0x7000bcc330, tpl_286::<0x7000bcb3a0, 0x70017d37a0, 0x70017790e0, 0x7000bcc51b> as *const (), 0), // +0x4e0330 sub_4e0330
    Lifted::new(0x7000bcc5a0, tpl_286::<0x7000bcb0c0, 0x70017d37a0, 0x70017790e0, 0x7000bcc78b> as *const (), 0), // +0x4e05a0 sub_4e05a0
    Lifted::new(0x7000bcc810, tpl_52::<0x8> as *const (), 1), // +0x4e0810 sub_4e0810
    Lifted::new(0x7000bcc820, tpl_52::<0x8> as *const (), 1), // +0x4e0820 sub_4e0820
    Lifted::new(0x7000bcc830, tpl_52::<0x8> as *const (), 1), // +0x4e0830 sub_4e0830
    Lifted::new(0x7000bcc840, tpl_52::<0x8> as *const (), 1), // +0x4e0840 sub_4e0840
    Lifted::new(0x7000bcc850, tpl_52::<0x8> as *const (), 1), // +0x4e0850 sub_4e0850
    Lifted::new(0x7000bcc860, tpl_52::<0x8> as *const (), 1), // +0x4e0860 sub_4e0860
    Lifted::new(0x7000bcc870, tpl_52::<0x8> as *const (), 1), // +0x4e0870 sub_4e0870
    Lifted::new(0x7000bcc880, tpl_52::<0x8> as *const (), 1), // +0x4e0880 sub_4e0880
    Lifted::new(0x7000bcc890, tpl_52::<0x8> as *const (), 1), // +0x4e0890 sub_4e0890
    Lifted::new(0x7000bcc8a0, tpl_52::<0x8> as *const (), 1), // +0x4e08a0 sub_4e08a0
    Lifted::new(0x7000bcc8b0, tpl_52::<0x8> as *const (), 1), // +0x4e08b0 sub_4e08b0
    Lifted::new(0x7000bcc8c0, tpl_52::<0x8> as *const (), 1), // +0x4e08c0 sub_4e08c0
    Lifted::new(0x7000bcc8d0, tpl_52::<0x8> as *const (), 1), // +0x4e08d0 sub_4e08d0
    Lifted::new(0x7000bcc8e0, tpl_52::<0x8> as *const (), 1), // +0x4e08e0 sub_4e08e0
    Lifted::new(0x7000bcc8f0, tpl_52::<0x8> as *const (), 1), // +0x4e08f0 sub_4e08f0
    Lifted::new(0x7000bcc900, tpl_52::<0x8> as *const (), 1), // +0x4e0900 sub_4e0900
    Lifted::new(0x7000bcc910, tpl_52::<0x8> as *const (), 1), // +0x4e0910 sub_4e0910
    Lifted::new(0x7000bcc920, tpl_196 as *const (), 1), // +0x4e0920 sub_4e0920
    Lifted::new(0x7000bcc930, tpl_988::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000bccaf5, 0x70007f0910, 0x7000bccafb, 0x7000bcd920, 0x7000bccb04> as *const (), 0), // +0x4e0930 sub_4e0930
    Lifted::new(0x7000bccb80, tpl_900::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000bccd48, 0x70007f0910, 0x7000bccd4e, 0x7000bcd940, 0x7000bccd57> as *const (), 0), // +0x4e0b80 sub_4e0b80
    Lifted::new(0x7000bccdd0, tpl_287 as *const (), 1), // +0x4e0dd0 sub_4e0dd0
    Lifted::new(0x7000bcd0f0, tpl_989::<0x80, 0x7001d54740> as *const (), 0), // +0x4e10f0 sub_4e10f0
    Lifted::new(0x7000bcd110, tpl_918::<0x7001805c98, 0x8> as *const (), 3), // +0x4e1110 sub_4e1110
    Lifted::new(0x7000bcd130, tpl_918::<0x7001805bd8, 0x8> as *const (), 3), // +0x4e1130 sub_4e1130
    Lifted::new(0x7000bcd150, tpl_918::<0x7001805ad8, 0x8> as *const (), 3), // +0x4e1150 sub_4e1150
    Lifted::new(0x7000bcd170, tpl_923::<0x7001805d58> as *const (), 1), // +0x4e1170 sub_4e1170
    Lifted::new(0x7000bcd190, tpl_978::<0x7001805a18, 0x8, 0x18, 0x28> as *const (), 3), // +0x4e1190 sub_4e1190
    Lifted::new(0x7000bcd1d0, tpl_918::<0x7001805b58, 0x8> as *const (), 3), // +0x4e11d0 sub_4e11d0
    Lifted::new(0x7000bcd1f0, tpl_979::<0x70018059d8, 0x8, 0x18> as *const (), 3), // +0x4e11f0 sub_4e11f0
    Lifted::new(0x7000bcd220, tpl_918::<0x7001805b18, 0x8> as *const (), 3), // +0x4e1220 sub_4e1220
    Lifted::new(0x7000bcd240, tpl_918::<0x7001805a98, 0x8> as *const (), 3), // +0x4e1240 sub_4e1240
    Lifted::new(0x7000bcd260, tpl_980::<0x7001805998, 0x8, 0x18> as *const (), 3), // +0x4e1260 sub_4e1260
    Lifted::new(0x7000bcd290, tpl_918::<0x7001805c18, 0x8> as *const (), 3), // +0x4e1290 sub_4e1290
    Lifted::new(0x7000bcd2b0, tpl_980::<0x7001805d18, 0x8, 0x18> as *const (), 3), // +0x4e12b0 sub_4e12b0
    Lifted::new(0x7000bcd2e0, tpl_979::<0x7001805cd8, 0x8, 0x18> as *const (), 3), // +0x4e12e0 sub_4e12e0
    Lifted::new(0x7000bcd310, tpl_980::<0x7001805958, 0x8, 0x18> as *const (), 3), // +0x4e1310 sub_4e1310
    Lifted::new(0x7000bcd340, tpl_918::<0x7001805b98, 0x8> as *const (), 3), // +0x4e1340 sub_4e1340
    Lifted::new(0x7000bcd360, tpl_923::<0x7001805c58> as *const (), 1), // +0x4e1360 sub_4e1360
    Lifted::new(0x7000bcd380, tpl_981::<0x7001805a58, 0x8, 0x18, 0x28> as *const (), 3), // +0x4e1380 sub_4e1380
    Lifted::new(0x7000bcd3b0, fn_7000bcd3b0 as *const (), 1), // +0x4e13b0 sub_4e13b0
    Lifted::new(0x7000bcd3f0, tpl_315::<0x7001c6b6a0> as *const (), 1), // +0x4e13f0 sub_4e13f0
    Lifted::new(0x7000bcd400, tpl_315::<0x7001c6b790> as *const (), 1), // +0x4e1400 sub_4e1400
    Lifted::new(0x7000bcd410, tpl_315::<0x7001c6b8d0> as *const (), 1), // +0x4e1410 sub_4e1410
    Lifted::new(0x7000bcd420, tpl_315::<0x7001c6b5b0> as *const (), 1), // +0x4e1420 sub_4e1420
    Lifted::new(0x7000bcd430, tpl_315::<0x7001c6b9c0> as *const (), 1), // +0x4e1430 sub_4e1430
    Lifted::new(0x7000bcd440, tpl_315::<0x7001c6b830> as *const (), 1), // +0x4e1440 sub_4e1440
    Lifted::new(0x7000bcd450, tpl_315::<0x7001c6ba10> as *const (), 1), // +0x4e1450 sub_4e1450
    Lifted::new(0x7000bcd460, tpl_315::<0x7001c6b880> as *const (), 1), // +0x4e1460 sub_4e1460
    Lifted::new(0x7000bcd470, tpl_315::<0x7001c6b920> as *const (), 1), // +0x4e1470 sub_4e1470
    Lifted::new(0x7000bcd480, tpl_315::<0x7001c6ba60> as *const (), 1), // +0x4e1480 sub_4e1480
    Lifted::new(0x7000bcd490, tpl_315::<0x7001c6b740> as *const (), 1), // +0x4e1490 sub_4e1490
    Lifted::new(0x7000bcd4a0, tpl_315::<0x7001c6b600> as *const (), 1), // +0x4e14a0 sub_4e14a0
    Lifted::new(0x7000bcd4b0, tpl_315::<0x7001c6b650> as *const (), 1), // +0x4e14b0 sub_4e14b0
    Lifted::new(0x7000bcd4c0, tpl_315::<0x7001c6bab0> as *const (), 1), // +0x4e14c0 sub_4e14c0
    Lifted::new(0x7000bcd4d0, tpl_315::<0x7001c6b7e0> as *const (), 1), // +0x4e14d0 sub_4e14d0
    Lifted::new(0x7000bcd4e0, tpl_315::<0x7001c6b6f0> as *const (), 1), // +0x4e14e0 sub_4e14e0
    Lifted::new(0x7000bcd4f0, tpl_315::<0x7001c6b970> as *const (), 1), // +0x4e14f0 sub_4e14f0
    Lifted::new(0x7000bcd500, fn_7000bcd500 as *const (), 0), // +0x4e1500 sub_4e1500
    Lifted::new(0x7000bcd5d0, fn_7000bcd5d0 as *const (), 0), // +0x4e15d0 sub_4e15d0
    Lifted::new(0x7000bcd900, tpl_989::<0x80, 0x7001d54738> as *const (), 0), // +0x4e1900 sub_4e1900
    Lifted::new(0x7000bcd920, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000bcd930> as *const (), 0), // +0x4e1920 sub_4e1920
    Lifted::new(0x7000bcd940, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000bcd950> as *const (), 0), // +0x4e1940 sub_4e1940
    Lifted::new(0x7000bcd960, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000bcd970> as *const (), 0), // +0x4e1960 sub_4e1960
    Lifted::new(0x7000bcd980, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000bcd990> as *const (), 0), // +0x4e1980 sub_4e1980
    Lifted::new(0x7000bcd9a0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000bcd9b0> as *const (), 0), // +0x4e19a0 sub_4e19a0
    Lifted::new(0x7000bcd9c0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000bcd9d0> as *const (), 0), // +0x4e19c0 sub_4e19c0
    Lifted::new(0x7000bcd9e0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000bcda34, 0x70007f0910, 0x7000bcda4f> as *const (), 0), // +0x4e19e0 sub_4e19e0
    Lifted::new(0x7000bcda70, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000bcdabd, 0x70007f0910, 0x7000bcdadb> as *const (), 0), // +0x4e1a70 sub_4e1a70
    Lifted::new(0x7000bcdb00, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000bcdb54, 0x70007f0910, 0x7000bcdb6f> as *const (), 0), // +0x4e1b00 sub_4e1b00
    Lifted::new(0x7000bcdb90, tpl_990::<0x10, 0x8, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x4e1b90 sub_4e1b90
    Lifted::new(0x7000bcdc50, fn_7000bcdc50 as *const (), 0), // +0x4e1c50 sub_4e1c50
    Lifted::new(0x7000bcdd60, fn_7000bcdd60 as *const (), 0), // +0x4e1d60 sub_4e1d60
    Lifted::new(0x7000bcdde0, tpl_36::<0xd848> as *const (), 1), // +0x4e1de0 sub_4e1de0
    Lifted::new(0x7000bcddf0, tpl_39::<0xd850> as *const (), 1), // +0x4e1df0 sub_4e1df0
    Lifted::new(0x7000bcde00, tpl_36::<0xd858> as *const (), 1), // +0x4e1e00 sub_4e1e00
    Lifted::new(0x7000bcde10, tpl_39::<0xd860> as *const (), 1), // +0x4e1e10 sub_4e1e10
    Lifted::new(0x7000bcde20, tpl_39::<0x10> as *const (), 1), // +0x4e1e20 sub_4e1e20
    Lifted::new(0x7000bcde30, fn_7000bcde30 as *const (), 0), // +0x4e1e30 sub_4e1e30
    Lifted::new(0x7000bcdec0, tpl_30 as *const (), 1), // +0x4e1ec0 sub_4e1ec0
    Lifted::new(0x7000bcded0, tpl_991::<0xa5a8> as *const (), 1), // +0x4e1ed0 sub_4e1ed0
    Lifted::new(0x7000bcdee0, tpl_47::<0xa6c0> as *const (), 1), // +0x4e1ee0 sub_4e1ee0
    Lifted::new(0x7000bcdef0, fn_7000bcdef0 as *const (), 1), // +0x4e1ef0 sub_4e1ef0
    Lifted::new(0x7000bcdf10, fn_7000bcdf10 as *const (), 0), // +0x4e1f10 sub_4e1f10
    Lifted::new(0x7000bcdf80, tpl_144 as *const (), 1), // +0x4e1f80 sub_4e1f80
    Lifted::new(0x7000bcdf90, tpl_36::<0x18> as *const (), 1), // +0x4e1f90 sub_4e1f90
    Lifted::new(0x7000bcdfa0, tpl_39::<0xc> as *const (), 1), // +0x4e1fa0 sub_4e1fa0
    Lifted::new(0x7000bcdfb0, fn_7000bcdfb0 as *const (), 2), // +0x4e1fb0 sub_4e1fb0
    Lifted::new(0x7000bce2b0, fn_7000bce2b0 as *const (), 1), // +0x4e22b0 sub_4e22b0
    Lifted::new(0x7000bce2c0, fn_7000bce2c0 as *const (), 0), // +0x4e22c0 sub_4e22c0
    Lifted::new(0x7000bce4d0, tpl_275::<0x9760, 0x8> as *const (), 1), // +0x4e24d0 sub_4e24d0
    Lifted::new(0x7000bce4e0, fn_7000bce4e0 as *const (), 0), // +0x4e24e0 sub_4e24e0
    Lifted::new(0x7000bce600, tpl_36::<0x28> as *const (), 1), // +0x4e2600 sub_4e2600
    Lifted::new(0x7000bce610, fn_7000bce610 as *const (), 0), // +0x4e2610 sub_4e2610
    Lifted::new(0x7000bce6b0, fn_7000bce6b0 as *const (), 0), // +0x4e26b0 sub_4e26b0
    Lifted::new(0x7000bce7b0, fn_7000bce7b0 as *const (), 0), // +0x4e27b0 sub_4e27b0
    Lifted::new(0x7000bce8c0, fn_7000bce8c0 as *const (), 0), // +0x4e28c0 sub_4e28c0
    Lifted::new(0x7000bce910, fn_7000bce910 as *const (), 0), // +0x4e2910 sub_4e2910
    Lifted::new(0x7000bcea30, fn_7000bcea30 as *const (), 2), // +0x4e2a30 sub_4e2a30
    Lifted::new(0x7000bced30, fn_7000bced30 as *const (), 2), // +0x4e2d30 sub_4e2d30
    Lifted::new(0x7000bcedf0, fn_7000bcedf0 as *const (), 0), // +0x4e2df0 sub_4e2df0
    Lifted::new(0x7000bcee50, fn_7000bcee50 as *const (), 0), // +0x4e2e50 sub_4e2e50
    Lifted::new(0x7000bcf1d0, fn_7000bcf1d0 as *const (), 0), // +0x4e31d0 sub_4e31d0
    Lifted::new(0x7000bcf250, fn_7000bcf250 as *const (), 1), // +0x4e3250 sub_4e3250
    Lifted::new(0x7000bcf280, fn_7000bcf280 as *const (), 0), // +0x4e3280 sub_4e3280
    Lifted::new(0x7000bcf440, fn_7000bcf440 as *const (), 0), // +0x4e3440 sub_4e3440
    Lifted::new(0x7000bcf550, fn_7000bcf550 as *const (), 0), // +0x4e3550 sub_4e3550
    Lifted::new(0x7000bcf650, fn_7000bcf650 as *const (), 0), // +0x4e3650 sub_4e3650
    Lifted::new(0x7000bcf6a0, tpl_143::<0x90, 0x70007a46e0> as *const (), 0), // +0x4e36a0 sub_4e36a0
    Lifted::new(0x7000bcf6b0, fn_7000bcf6b0 as *const (), 0), // +0x4e36b0 sub_4e36b0
    Lifted::new(0x7000bcf6e0, fn_7000bcf6e0 as *const (), 0), // +0x4e36e0 sub_4e36e0
    Lifted::new(0x7000bcf730, tpl_15 as *const (), 1), // +0x4e3730 sub_4e3730
    Lifted::new(0x7000bcf740, tpl_15 as *const (), 1), // +0x4e3740 sub_4e3740
    Lifted::new(0x7000bcf750, fn_7000bcf750 as *const (), 1), // +0x4e3750 sub_4e3750
    Lifted::new(0x7000bcf780, fn_7000bcf780 as *const (), 1), // +0x4e3780 sub_4e3780
    Lifted::new(0x7000bcf7b0, fn_7000bcf7b0 as *const (), 1), // +0x4e37b0 sub_4e37b0
    Lifted::new(0x7000bcf7e0, fn_7000bcf7e0 as *const (), 1), // +0x4e37e0 sub_4e37e0
    Lifted::new(0x7000bcf800, fn_7000bcf800 as *const (), 1), // +0x4e3800 sub_4e3800
    Lifted::new(0x7000bcf830, fn_7000bcf830 as *const (), 0), // +0x4e3830 sub_4e3830
    Lifted::new(0x7000bcfb70, fn_7000bcfb70 as *const (), 0), // +0x4e3b70 sub_4e3b70
    Lifted::new(0x7000bd02c0, fn_7000bd02c0 as *const (), 2), // +0x4e42c0 sub_4e42c0
    Lifted::new(0x7000bd06b0, fn_7000bd06b0 as *const (), 0), // +0x4e46b0 sub_4e46b0
    Lifted::new(0x7000bd0760, fn_7000bd0760 as *const (), 0), // +0x4e4760 sub_4e4760
    Lifted::new(0x7000bd0c90, fn_7000bd0c90 as *const (), 0), // +0x4e4c90 sub_4e4c90
    Lifted::new(0x7000bd10a0, fn_7000bd10a0 as *const (), 0), // +0x4e50a0 sub_4e50a0
    Lifted::new(0x7000bd11e0, fn_7000bd11e0 as *const (), 0), // +0x4e51e0 sub_4e51e0
    Lifted::new(0x7000bd13c0, fn_7000bd13c0 as *const (), 0), // +0x4e53c0 sub_4e53c0
    Lifted::new(0x7000bd1bd0, fn_7000bd1bd0 as *const (), 0), // +0x4e5bd0 sub_4e5bd0
    Lifted::new(0x7000bd1de0, fn_7000bd1de0 as *const (), 0), // +0x4e5de0 sub_4e5de0
    Lifted::new(0x7000bd1f40, fn_7000bd1f40 as *const (), 2), // +0x4e5f40 sub_4e5f40
    Lifted::new(0x7000bd22d0, fn_7000bd22d0 as *const (), 0), // +0x4e62d0 sub_4e62d0
    Lifted::new(0x7000bd2410, fn_7000bd2410 as *const (), 0), // +0x4e6410 sub_4e6410
    Lifted::new(0x7000bd2690, fn_7000bd2690 as *const (), 0), // +0x4e6690 sub_4e6690
    Lifted::new(0x7000bd2760, fn_7000bd2760 as *const (), 0), // +0x4e6760 sub_4e6760
    Lifted::new(0x7000bd2840, fn_7000bd2840 as *const (), 0), // +0x4e6840 sub_4e6840
    Lifted::new(0x7000bd28e0, fn_7000bd28e0 as *const (), 0), // +0x4e68e0 sub_4e68e0
    Lifted::new(0x7000bd29e0, fn_7000bd29e0 as *const (), 2), // +0x4e69e0 sub_4e69e0
    Lifted::new(0x7000bd2ce0, fn_7000bd2ce0 as *const (), 0), // +0x4e6ce0 sub_4e6ce0
    Lifted::new(0x7000bd31d0, fn_7000bd31d0 as *const (), 2), // +0x4e71d0 sub_4e71d0
    Lifted::new(0x7000bd3570, fn_7000bd3570 as *const (), 0), // +0x4e7570 sub_4e7570
    Lifted::new(0x7000bd4d50, fn_7000bd4d50 as *const (), 2), // +0x4e8d50 sub_4e8d50
    Lifted::new(0x7000bd5360, fn_7000bd5360 as *const (), 2), // +0x4e9360 sub_4e9360
    Lifted::new(0x7000bd5710, fn_7000bd5710 as *const (), 0), // +0x4e9710 sub_4e9710
    Lifted::new(0x7000bd59e0, tpl_388::<0x7000bd6d10, 0x700177c312, 0x7001d54968, 0x7000bd5b6a, 0x7000bd6cf0, 0x7000bd5b70> as *const (), 2), // +0x4e99e0 sub_4e99e0
    Lifted::new(0x7000bd5be0, fn_7000bd5be0 as *const (), 1), // +0x4e9be0 sub_4e9be0
    Lifted::new(0x7000bd5c40, fn_7000bd5c40 as *const (), 0), // +0x4e9c40 sub_4e9c40
    Lifted::new(0x7000bd5d00, fn_7000bd5d00 as *const (), 0), // +0x4e9d00 sub_4e9d00
    Lifted::new(0x7000bd5dc0, fn_7000bd5dc0 as *const (), 2), // +0x4e9dc0 sub_4e9dc0
    Lifted::new(0x7000bd5e40, fn_7000bd5e40 as *const (), 0), // +0x4e9e40 sub_4e9e40
    Lifted::new(0x7000bd5ec0, fn_7000bd5ec0 as *const (), 0), // +0x4e9ec0 sub_4e9ec0
    Lifted::new(0x7000bd5f40, fn_7000bd5f40 as *const (), 0), // +0x4e9f40 sub_4e9f40
    Lifted::new(0x7000bd6070, fn_7000bd6070 as *const (), 0), // +0x4ea070 sub_4ea070
    Lifted::new(0x7000bd6200, fn_7000bd6200 as *const (), 0), // +0x4ea200 sub_4ea200
    Lifted::new(0x7000bd6310, fn_7000bd6310 as *const (), 0), // +0x4ea310 sub_4ea310
    Lifted::new(0x7000bd63f0, fn_7000bd63f0 as *const (), 2), // +0x4ea3f0 sub_4ea3f0
    Lifted::new(0x7000bd6580, fn_7000bd6580 as *const (), 0), // +0x4ea580 sub_4ea580
    Lifted::new(0x7000bd6620, tpl_992::<0x58, 0x50, 0x8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const (), 1), // +0x4ea620 sub_4ea620
    Lifted::new(0x7000bd66a0, fn_7000bd66a0 as *const (), 0), // +0x4ea6a0 sub_4ea6a0
    Lifted::new(0x7000bd6750, fn_7000bd6750 as *const (), 2), // +0x4ea750 sub_4ea750
    Lifted::new(0x7000bd6c60, fn_7000bd6c60 as *const (), 0), // +0x4eac60 sub_4eac60
    Lifted::new(0x7000bd6cf0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000bd6d00> as *const (), 0), // +0x4eacf0 sub_4eacf0
    Lifted::new(0x7000bd6d10, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000bd6d5d, 0x70007f0910, 0x7000bd6d7b> as *const (), 0), // +0x4ead10 sub_4ead10
    Lifted::new(0x7000bd6da0, fn_7000bd6da0 as *const (), 1), // +0x4eada0 sub_4eada0
    Lifted::new(0x7000bd6dd0, fn_7000bd6dd0 as *const (), 0), // +0x4eadd0 sub_4eadd0
    Lifted::new(0x7000bd6e30, fn_7000bd6e30 as *const (), 0), // +0x4eae30 sub_4eae30
    Lifted::new(0x7000bd6e90, fn_7000bd6e90 as *const (), 0), // +0x4eae90 sub_4eae90
    Lifted::new(0x7000bd6ef0, fn_7000bd6ef0 as *const (), 0), // +0x4eaef0 sub_4eaef0
    Lifted::new(0x7000bd6f50, fn_7000bd6f50 as *const (), 0), // +0x4eaf50 sub_4eaf50
    Lifted::new(0x7000bd6fb0, fn_7000bd6fb0 as *const (), 0), // +0x4eafb0 sub_4eafb0
    Lifted::new(0x7000bd7010, fn_7000bd7010 as *const (), 0), // +0x4eb010 sub_4eb010
    Lifted::new(0x7000bd7070, fn_7000bd7070 as *const (), 0), // +0x4eb070 sub_4eb070
    Lifted::new(0x7000bd70d0, fn_7000bd70d0 as *const (), 0), // +0x4eb0d0 sub_4eb0d0
    Lifted::new(0x7000bd7130, fn_7000bd7130 as *const (), 0), // +0x4eb130 sub_4eb130
    Lifted::new(0x7000bd7190, fn_7000bd7190 as *const (), 0), // +0x4eb190 sub_4eb190
    Lifted::new(0x7000bd71f0, fn_7000bd71f0 as *const (), 0), // +0x4eb1f0 sub_4eb1f0
    Lifted::new(0x7000bd7250, fn_7000bd7250 as *const (), 0), // +0x4eb250 sub_4eb250
    Lifted::new(0x7000bd72b0, fn_7000bd72b0 as *const (), 0), // +0x4eb2b0 sub_4eb2b0
    Lifted::new(0x7000bd7310, fn_7000bd7310 as *const (), 0), // +0x4eb310 sub_4eb310
    Lifted::new(0x7000bd7370, fn_7000bd7370 as *const (), 0), // +0x4eb370 sub_4eb370
    Lifted::new(0x7000bd73d0, fn_7000bd73d0 as *const (), 0), // +0x4eb3d0 sub_4eb3d0
    Lifted::new(0x7000bd7430, fn_7000bd7430 as *const (), 2), // +0x4eb430 sub_4eb430
    Lifted::new(0x7000bd75b0, fn_7000bd75b0 as *const (), 2), // +0x4eb5b0 sub_4eb5b0
    Lifted::new(0x7000bd77c0, fn_7000bd77c0 as *const (), 0), // +0x4eb7c0 sub_4eb7c0
    Lifted::new(0x7000bd7840, fn_7000bd7840 as *const (), 0), // +0x4eb840 sub_4eb840
    Lifted::new(0x7000bd7aa0, fn_7000bd7aa0 as *const (), 2), // +0x4ebaa0 sub_4ebaa0
    Lifted::new(0x7000bd7e50, tpl_993::<0x700079a930, 0x7000775960, 0x7000bd7aa0> as *const (), 0), // +0x4ebe50 sub_4ebe50
    Lifted::new(0x7000bd7eb0, tpl_993::<0x700079a930, 0x7000775960, 0x7000bd7aa0> as *const (), 0), // +0x4ebeb0 sub_4ebeb0
    Lifted::new(0x7000bd7f10, fn_7000bd7f10 as *const (), 0), // +0x4ebf10 sub_4ebf10
    Lifted::new(0x7000bd8160, fn_7000bd8160 as *const (), 0), // +0x4ec160 sub_4ec160
    Lifted::new(0x7000bd85d0, fn_7000bd85d0 as *const (), 0), // +0x4ec5d0 sub_4ec5d0
    Lifted::new(0x7000bd8880, fn_7000bd8880 as *const (), 0), // +0x4ec880 sub_4ec880
    Lifted::new(0x7000bd89f0, fn_7000bd89f0 as *const (), 0), // +0x4ec9f0 sub_4ec9f0
    Lifted::new(0x7000bd8a50, fn_7000bd8a50 as *const (), 2), // +0x4eca50 sub_4eca50
    Lifted::new(0x7000bd8b00, fn_7000bd8b00 as *const (), 0), // +0x4ecb00 sub_4ecb00
    Lifted::new(0x7000bd8b60, fn_7000bd8b60 as *const (), 0), // +0x4ecb60 sub_4ecb60
    Lifted::new(0x7000bd8c00, fn_7000bd8c00 as *const (), 0), // +0x4ecc00 sub_4ecc00
    Lifted::new(0x7000bd8c60, fn_7000bd8c60 as *const (), 0), // +0x4ecc60 sub_4ecc60
    Lifted::new(0x7000bd8cc0, fn_7000bd8cc0 as *const (), 0), // +0x4eccc0 sub_4eccc0
    Lifted::new(0x7000bd8d70, fn_7000bd8d70 as *const (), 0), // +0x4ecd70 sub_4ecd70
    Lifted::new(0x7000bd8da0, fn_7000bd8da0 as *const (), 2), // +0x4ecda0 sub_4ecda0
    Lifted::new(0x7000bd9bc0, fn_7000bd9bc0 as *const (), 0), // +0x4edbc0 sub_4edbc0
    Lifted::new(0x7000bd9c40, fn_7000bd9c40 as *const (), 0), // +0x4edc40 sub_4edc40
    Lifted::new(0x7000bd9cc0, fn_7000bd9cc0 as *const (), 0), // +0x4edcc0 sub_4edcc0
    Lifted::new(0x7000bd9d20, fn_7000bd9d20 as *const (), 0), // +0x4edd20 sub_4edd20
    Lifted::new(0x7000bd9e10, fn_7000bd9e10 as *const (), 0), // +0x4ede10 sub_4ede10
    Lifted::new(0x7000bd9f10, fn_7000bd9f10 as *const (), 0), // +0x4edf10 sub_4edf10
    Lifted::new(0x7000bd9fb0, fn_7000bd9fb0 as *const (), 2), // +0x4edfb0 sub_4edfb0
    Lifted::new(0x7000bda030, fn_7000bda030 as *const (), 0), // +0x4ee030 sub_4ee030
    Lifted::new(0x7000bda0b0, fn_7000bda0b0 as *const (), 1), // +0x4ee0b0 sub_4ee0b0
    Lifted::new(0x7000bda0d0, fn_7000bda0d0 as *const (), 1), // +0x4ee0d0 sub_4ee0d0
    Lifted::new(0x7000bda0f0, fn_7000bda0f0 as *const (), 0), // +0x4ee0f0 sub_4ee0f0
    Lifted::new(0x7000bda160, fn_7000bda160 as *const (), 1), // +0x4ee160 sub_4ee160
    Lifted::new(0x7000bda1a0, fn_7000bda1a0 as *const (), 1), // +0x4ee1a0 sub_4ee1a0
    Lifted::new(0x7000bda240, fn_7000bda240 as *const (), 0), // +0x4ee240 sub_4ee240
    Lifted::new(0x7000bda280, fn_7000bda280 as *const (), 0), // +0x4ee280 sub_4ee280
    Lifted::new(0x7000bda350, fn_7000bda350 as *const (), 0), // +0x4ee350 sub_4ee350
    Lifted::new(0x7000bda470, fn_7000bda470 as *const (), 0), // +0x4ee470 sub_4ee470
    Lifted::new(0x7000bda550, fn_7000bda550 as *const (), 0), // +0x4ee550 sub_4ee550
    Lifted::new(0x7000bda650, fn_7000bda650 as *const (), 0), // +0x4ee650 sub_4ee650
    Lifted::new(0x7000bda6b0, fn_7000bda6b0 as *const (), 0), // +0x4ee6b0 sub_4ee6b0
    Lifted::new(0x7000bda760, fn_7000bda760 as *const (), 0), // +0x4ee760 sub_4ee760
    Lifted::new(0x7000bda8b0, fn_7000bda8b0 as *const (), 0), // +0x4ee8b0 sub_4ee8b0
    Lifted::new(0x7000bda8d0, fn_7000bda8d0 as *const (), 0), // +0x4ee8d0 sub_4ee8d0
    Lifted::new(0x7000bda950, fn_7000bda950 as *const (), 0), // +0x4ee950 sub_4ee950
    Lifted::new(0x7000bdaa40, fn_7000bdaa40 as *const (), 1), // +0x4eea40 sub_4eea40
    Lifted::new(0x7000bdaa60, fn_7000bdaa60 as *const (), 0), // +0x4eea60 sub_4eea60
    Lifted::new(0x7000bdaae0, fn_7000bdaae0 as *const (), 0), // +0x4eeae0 sub_4eeae0
    Lifted::new(0x7000bdabf0, fn_7000bdabf0 as *const (), 0), // +0x4eebf0 sub_4eebf0
    Lifted::new(0x7000bdac50, fn_7000bdac50 as *const (), 0), // +0x4eec50 sub_4eec50
    Lifted::new(0x7000bdacf0, fn_7000bdacf0 as *const (), 0), // +0x4eecf0 sub_4eecf0
    Lifted::new(0x7000bdad40, tpl_481::<0x7001c72004> as *const (), 1), // +0x4eed40 sub_4eed40
    Lifted::new(0x7000bdad50, fn_7000bdad50 as *const (), 0), // +0x4eed50 sub_4eed50
    Lifted::new(0x7000bdad80, fn_7000bdad80 as *const (), 0), // +0x4eed80 sub_4eed80
    Lifted::new(0x7000bdadb0, fn_7000bdadb0 as *const (), 0), // +0x4eedb0 sub_4eedb0
    Lifted::new(0x7000bdae60, fn_7000bdae60 as *const (), 0), // +0x4eee60 sub_4eee60
    Lifted::new(0x7000bdaf40, fn_7000bdaf40 as *const (), 0), // +0x4eef40 sub_4eef40
    Lifted::new(0x7000bdaf90, fn_7000bdaf90 as *const (), 0), // +0x4eef90 sub_4eef90
    Lifted::new(0x7000bdb2e0, tpl_994::<0x7000729fb0, 0x70007476f0> as *const (), 0), // +0x4ef2e0 sub_4ef2e0
    Lifted::new(0x7000bdb350, fn_7000bdb350 as *const (), 0), // +0x4ef350 sub_4ef350
    Lifted::new(0x7000bdb400, tpl_969::<0x700180c6c0, 0x10, 0x700177c312> as *const (), 0), // +0x4ef400 sub_4ef400
    Lifted::new(0x7000bdb450, tpl_969::<0x700180c708, 0x10, 0x700177c312> as *const (), 0), // +0x4ef450 sub_4ef450
    Lifted::new(0x7000bdb4a0, fn_7000bdb4a0 as *const (), 0), // +0x4ef4a0 sub_4ef4a0
    Lifted::new(0x7000bdb7f0, fn_7000bdb7f0 as *const (), 0), // +0x4ef7f0 sub_4ef7f0
    Lifted::new(0x7000bdb8c0, fn_7000bdb8c0 as *const (), 0), // +0x4ef8c0 sub_4ef8c0
    Lifted::new(0x7000bdb9a0, tpl_36::<0x10> as *const (), 1), // +0x4ef9a0 sub_4ef9a0
    Lifted::new(0x7000bdb9b0, tpl_36::<0x10> as *const (), 1), // +0x4ef9b0 sub_4ef9b0
    Lifted::new(0x7000bdb9c0, tpl_36::<0x18> as *const (), 1), // +0x4ef9c0 sub_4ef9c0
    Lifted::new(0x7000bdb9d0, tpl_36::<0x18> as *const (), 1), // +0x4ef9d0 sub_4ef9d0
    Lifted::new(0x7000bdb9e0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x4ef9e0 sub_4ef9e0
    Lifted::new(0x7000bdba20, tpl_899::<0x7000bdb9e0> as *const (), 0), // +0x4efa20 sub_4efa20
    Lifted::new(0x7000bdba30, fn_7000bdba30 as *const (), 0), // +0x4efa30 sub_4efa30
    Lifted::new(0x7000bdbd20, tpl_994::<0x7000778ec0, 0x70007476f0> as *const (), 0), // +0x4efd20 sub_4efd20
    Lifted::new(0x7000bdbd90, fn_7000bdbd90 as *const (), 0), // +0x4efd90 sub_4efd90
    Lifted::new(0x7000bdbe00, fn_7000bdbe00 as *const (), 0), // +0x4efe00 sub_4efe00
    Lifted::new(0x7000bdbf20, fn_7000bdbf20 as *const (), 0), // +0x4eff20 sub_4eff20
    Lifted::new(0x7000bdc1d0, fn_7000bdc1d0 as *const (), 0), // +0x4f01d0 sub_4f01d0
    Lifted::new(0x7000bdc300, fn_7000bdc300 as *const (), 0), // +0x4f0300 sub_4f0300
    Lifted::new(0x7000bdc350, tpl_994::<0x7000727550, 0x70007476f0> as *const (), 0), // +0x4f0350 sub_4f0350
    Lifted::new(0x7000bdc3c0, fn_7000bdc3c0 as *const (), 0), // +0x4f03c0 sub_4f03c0
    Lifted::new(0x7000bdc4a0, fn_7000bdc4a0 as *const (), 2), // +0x4f04a0 sub_4f04a0
    Lifted::new(0x7000bdd660, tpl_994::<0x7000756960, 0x70007476f0> as *const (), 0), // +0x4f1660 sub_4f1660
    Lifted::new(0x7000bdd6d0, fn_7000bdd6d0 as *const (), 0), // +0x4f16d0 sub_4f16d0
    Lifted::new(0x7000bdd850, tpl_15 as *const (), 1), // +0x4f1850 sub_4f1850
    Lifted::new(0x7000bdd860, tpl_995::<0x70017d928c, 0x70017a0290, 0x7001d54ef8, 0x7000bdd878> as *const (), 0), // +0x4f1860 sub_4f1860
    Lifted::new(0x7000bdd880, tpl_15 as *const (), 1), // +0x4f1880 sub_4f1880
    Lifted::new(0x7000bdd890, fn_7000bdd890 as *const (), 0), // +0x4f1890 sub_4f1890
    Lifted::new(0x7000bdda70, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4f1a70 sub_4f1a70
    Lifted::new(0x7000bddab0, tpl_392::<0x700177c0e0> as *const (), 0), // +0x4f1ab0 sub_4f1ab0
    Lifted::new(0x7000bddae0, fn_7000bddae0 as *const (), 2), // +0x4f1ae0 sub_4f1ae0
    Lifted::new(0x7000bddc60, fn_7000bddc60 as *const (), 0), // +0x4f1c60 sub_4f1c60
    Lifted::new(0x7000bdded0, fn_7000bdded0 as *const (), 2), // +0x4f1ed0 sub_4f1ed0
    Lifted::new(0x7000bde340, fn_7000bde340 as *const (), 2), // +0x4f2340 sub_4f2340
    Lifted::new(0x7000be2350, tpl_450::<0x7001c7232c> as *const (), 1), // +0x4f6350 sub_4f6350
    Lifted::new(0x7000be2360, fn_7000be2360 as *const (), 0), // +0x4f6360 sub_4f6360
    Lifted::new(0x7000be25d0, fn_7000be25d0 as *const (), 2), // +0x4f65d0 sub_4f65d0
    Lifted::new(0x7000be2810, fn_7000be2810 as *const (), 2), // +0x4f6810 sub_4f6810
    Lifted::new(0x7000be2ac0, fn_7000be2ac0 as *const (), 2), // +0x4f6ac0 sub_4f6ac0
    Lifted::new(0x7000be3540, fn_7000be3540 as *const (), 2), // +0x4f7540 sub_4f7540
    Lifted::new(0x7000be37c0, fn_7000be37c0 as *const (), 0), // +0x4f77c0 sub_4f77c0
    Lifted::new(0x7000be38a0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000be38b0> as *const (), 0), // +0x4f78a0 sub_4f78a0
    Lifted::new(0x7000be38c0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000be3914, 0x70007f0910, 0x7000be392f> as *const (), 0), // +0x4f78c0 sub_4f78c0
    Lifted::new(0x7000be3950, fn_7000be3950 as *const (), 2), // +0x4f7950 sub_4f7950
    Lifted::new(0x7000be4310, tpl_36::<0x8> as *const (), 1), // +0x4f8310 sub_4f8310
    Lifted::new(0x7000be4320, fn_7000be4320 as *const (), 2), // +0x4f8320 sub_4f8320
    Lifted::new(0x7000be4380, fn_7000be4380 as *const (), 0), // +0x4f8380 sub_4f8380
    Lifted::new(0x7000be43f0, fn_7000be43f0 as *const (), 2), // +0x4f83f0 sub_4f83f0
    Lifted::new(0x7000be45d0, fn_7000be45d0 as *const (), 2), // +0x4f85d0 sub_4f85d0
    Lifted::new(0x7000be47b0, fn_7000be47b0 as *const (), 0), // +0x4f87b0 sub_4f87b0
    Lifted::new(0x7000be49c0, fn_7000be49c0 as *const (), 0), // +0x4f89c0 sub_4f89c0
    Lifted::new(0x7000be4bf0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4f8bf0 sub_4f8bf0
    Lifted::new(0x7000be4c30, tpl_55::<0x700177c0d0> as *const (), 0), // +0x4f8c30 sub_4f8c30
    Lifted::new(0x7000be4c70, tpl_876::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000be4e71, 0x7000beaf00, 0x7000be4e77, 0x70007f0910, 0x7000be4e7d> as *const (), 2), // +0x4f8c70 sub_4f8c70
    Lifted::new(0x7000be4f10, tpl_996::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000be51af, 0x7000beaec0, 0x7000be51b5, 0x70007f0910, 0x7000be51bb> as *const (), 2), // +0x4f8f10 sub_4f8f10
    Lifted::new(0x7000be5270, tpl_996::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000be550f, 0x7000beaee0, 0x7000be5515, 0x70007f0910, 0x7000be551b> as *const (), 2), // +0x4f9270 sub_4f9270
    Lifted::new(0x7000be55d0, fn_7000be55d0 as *const (), 0), // +0x4f95d0 sub_4f95d0
    Lifted::new(0x7000be5640, tpl_997::<0x7000b73540, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000be576c, 0x7000b5ae00, 0x7000be5794> as *const (), 0), // +0x4f9640 sub_4f9640
    Lifted::new(0x7000be57f0, tpl_391::<0x7000beaf20, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000be591c, 0x7000beae80, 0x7000be5944> as *const (), 0), // +0x4f97f0 sub_4f97f0
    Lifted::new(0x7000be59a0, tpl_391::<0x7000beafb0, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000be5acc, 0x7000beaea0, 0x7000be5af4> as *const (), 0), // +0x4f99a0 sub_4f99a0
    Lifted::new(0x7000be5b50, tpl_392::<0x700177c0e0> as *const (), 0), // +0x4f9b50 sub_4f9b50
    Lifted::new(0x7000be5b80, tpl_392::<0x700177c0e0> as *const (), 0), // +0x4f9b80 sub_4f9b80
    Lifted::new(0x7000be5bb0, tpl_67::<0x7000b73540, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000be5cf3, 0x7000b5ae00, 0x7000be5cf9> as *const (), 0), // +0x4f9bb0 sub_4f9bb0
    Lifted::new(0x7000be5d50, tpl_395::<0x7000beaf20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000be5e95, 0x7000beae80, 0x7000be5e9b> as *const (), 0), // +0x4f9d50 sub_4f9d50
    Lifted::new(0x7000be5ef0, tpl_395::<0x7000beafb0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000be6035, 0x7000beaea0, 0x7000be603b> as *const (), 0), // +0x4f9ef0 sub_4f9ef0
    Lifted::new(0x7000be6090, fn_7000be6090 as *const (), 0), // +0x4fa090 sub_4fa090
    Lifted::new(0x7000be6230, tpl_998::<0x8, 0x2000, 0x700177c312> as *const (), 0), // +0x4fa230 sub_4fa230
    Lifted::new(0x7000be6270, tpl_998::<0x8, 0x2000, 0x700177c312> as *const (), 0), // +0x4fa270 sub_4fa270
    Lifted::new(0x7000be62b0, fn_7000be62b0 as *const (), 0), // +0x4fa2b0 sub_4fa2b0
    Lifted::new(0x7000be6330, tpl_937::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000be63aa> as *const (), 0), // +0x4fa330 sub_4fa330
    Lifted::new(0x7000be63d0, tpl_937::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000be644a> as *const (), 0), // +0x4fa3d0 sub_4fa3d0
    Lifted::new(0x7000be6470, fn_7000be6470 as *const (), 0), // +0x4fa470 sub_4fa470
    Lifted::new(0x7000be6580, fn_7000be6580 as *const (), 0), // +0x4fa580 sub_4fa580
    Lifted::new(0x7000be67c0, fn_7000be67c0 as *const (), 2), // +0x4fa7c0 sub_4fa7c0
    Lifted::new(0x7000be69b0, tpl_51::<0x10> as *const (), 1), // +0x4fa9b0 sub_4fa9b0
    Lifted::new(0x7000be69c0, fn_7000be69c0 as *const (), 0), // +0x4fa9c0 sub_4fa9c0
    Lifted::new(0x7000be69f0, fn_7000be69f0 as *const (), 0), // +0x4fa9f0 sub_4fa9f0
    Lifted::new(0x7000be6b90, tpl_143::<0x48, 0x7000be69f0> as *const (), 0), // +0x4fab90 sub_4fab90
    Lifted::new(0x7000be6ba0, fn_7000be6ba0 as *const (), 1), // +0x4faba0 sub_4faba0
    Lifted::new(0x7000be6bc0, fn_7000be6bc0 as *const (), 2), // +0x4fabc0 sub_4fabc0
    Lifted::new(0x7000be6d00, fn_7000be6d00 as *const (), 0), // +0x4fad00 sub_4fad00
    Lifted::new(0x7000be6d40, fn_7000be6d40 as *const (), 0), // +0x4fad40 sub_4fad40
    Lifted::new(0x7000be6e30, fn_7000be6e30 as *const (), 0), // +0x4fae30 sub_4fae30
    Lifted::new(0x7000be6f70, fn_7000be6f70 as *const (), 0), // +0x4faf70 sub_4faf70
    Lifted::new(0x7000be71c0, tpl_216::<0x70007a1700> as *const (), 0), // +0x4fb1c0 sub_4fb1c0
    Lifted::new(0x7000be71e0, fn_7000be71e0 as *const (), 2), // +0x4fb1e0 sub_4fb1e0
    Lifted::new(0x7000be75f0, fn_7000be75f0 as *const (), 0), // +0x4fb5f0 sub_4fb5f0
    Lifted::new(0x7000be7690, fn_7000be7690 as *const (), 0), // +0x4fb690 sub_4fb690
    Lifted::new(0x7000be76c0, fn_7000be76c0 as *const (), 0), // +0x4fb6c0 sub_4fb6c0
    Lifted::new(0x7000be76d0, fn_7000be76d0 as *const (), 0), // +0x4fb6d0 sub_4fb6d0
    Lifted::new(0x7000be7720, fn_7000be7720 as *const (), 0), // +0x4fb720 sub_4fb720
    Lifted::new(0x7000be7880, fn_7000be7880 as *const (), 1), // +0x4fb880 sub_4fb880
    Lifted::new(0x7000be78b0, fn_7000be78b0 as *const (), 0), // +0x4fb8b0 sub_4fb8b0
    Lifted::new(0x7000be79a0, tpl_210::<0xa4> as *const (), 1), // +0x4fb9a0 sub_4fb9a0
    Lifted::new(0x7000be79b0, fn_7000be79b0 as *const (), 0), // +0x4fb9b0 sub_4fb9b0
    Lifted::new(0x7000be7a10, fn_7000be7a10 as *const (), 2), // +0x4fba10 sub_4fba10
    Lifted::new(0x7000be7d90, fn_7000be7d90 as *const (), 0), // +0x4fbd90 sub_4fbd90
    Lifted::new(0x7000be7fa0, fn_7000be7fa0 as *const (), 2), // +0x4fbfa0 sub_4fbfa0
    Lifted::new(0x7000be80c0, fn_7000be80c0 as *const (), 2), // +0x4fc0c0 sub_4fc0c0
    Lifted::new(0x7000be8360, fn_7000be8360 as *const (), 1), // +0x4fc360 sub_4fc360
    Lifted::new(0x7000be8380, fn_7000be8380 as *const (), 1), // +0x4fc380 sub_4fc380
    Lifted::new(0x7000be8390, fn_7000be8390 as *const (), 0), // +0x4fc390 sub_4fc390
    Lifted::new(0x7000be8460, fn_7000be8460 as *const (), 0), // +0x4fc460 sub_4fc460
    Lifted::new(0x7000be8500, fn_7000be8500 as *const (), 0), // +0x4fc500 sub_4fc500
    Lifted::new(0x7000be8650, fn_7000be8650 as *const (), 0), // +0x4fc650 sub_4fc650
    Lifted::new(0x7000be8710, fn_7000be8710 as *const (), 0), // +0x4fc710 sub_4fc710
    Lifted::new(0x7000be8810, fn_7000be8810 as *const (), 0), // +0x4fc810 sub_4fc810
    Lifted::new(0x7000be88a0, fn_7000be88a0 as *const (), 0), // +0x4fc8a0 sub_4fc8a0
    Lifted::new(0x7000be8940, tpl_143::<0x48, 0x7000be8390> as *const (), 0), // +0x4fc940 sub_4fc940
    Lifted::new(0x7000be8950, fn_7000be8950 as *const (), 0), // +0x4fc950 sub_4fc950
    Lifted::new(0x7000be8a00, fn_7000be8a00 as *const (), 0), // +0x4fca00 sub_4fca00
    Lifted::new(0x7000be8b30, fn_7000be8b30 as *const (), 0), // +0x4fcb30 sub_4fcb30
    Lifted::new(0x7000be8bd0, fn_7000be8bd0 as *const (), 2), // +0x4fcbd0 sub_4fcbd0
    Lifted::new(0x7000be8ee0, fn_7000be8ee0 as *const (), 0), // +0x4fcee0 sub_4fcee0
    Lifted::new(0x7000be8fb0, fn_7000be8fb0 as *const (), 0), // +0x4fcfb0 sub_4fcfb0
    Lifted::new(0x7000be9050, fn_7000be9050 as *const (), 0), // +0x4fd050 sub_4fd050
    Lifted::new(0x7000be9140, fn_7000be9140 as *const (), 0), // +0x4fd140 sub_4fd140
    Lifted::new(0x7000be91b0, fn_7000be91b0 as *const (), 2), // +0x4fd1b0 sub_4fd1b0
    Lifted::new(0x7000be9510, fn_7000be9510 as *const (), 0), // +0x4fd510 sub_4fd510
    Lifted::new(0x7000be9590, fn_7000be9590 as *const (), 0), // +0x4fd590 sub_4fd590
    Lifted::new(0x7000be95f0, fn_7000be95f0 as *const (), 0), // +0x4fd5f0 sub_4fd5f0
    Lifted::new(0x7000be96a0, tpl_551::<0xb> as *const (), 1), // +0x4fd6a0 sub_4fd6a0
    Lifted::new(0x7000be96b0, fn_7000be96b0 as *const (), 0), // +0x4fd6b0 sub_4fd6b0
    Lifted::new(0x7000be9770, fn_7000be9770 as *const (), 1), // +0x4fd770 sub_4fd770
    Lifted::new(0x7000be97a0, fn_7000be97a0 as *const (), 0), // +0x4fd7a0 sub_4fd7a0
    Lifted::new(0x7000be9a80, fn_7000be9a80 as *const (), 0), // +0x4fda80 sub_4fda80
    Lifted::new(0x7000be9aa0, tpl_275::<0x48, 0x8> as *const (), 1), // +0x4fdaa0 sub_4fdaa0
    Lifted::new(0x7000be9ab0, fn_7000be9ab0 as *const (), 0), // +0x4fdab0 sub_4fdab0
    Lifted::new(0x7000be9b70, fn_7000be9b70 as *const (), 0), // +0x4fdb70 sub_4fdb70
    Lifted::new(0x7000be9c50, fn_7000be9c50 as *const (), 2), // +0x4fdc50 sub_4fdc50
    Lifted::new(0x7000be9e70, fn_7000be9e70 as *const (), 2), // +0x4fde70 sub_4fde70
    Lifted::new(0x7000bea0e0, fn_7000bea0e0 as *const (), 1), // +0x4fe0e0 sub_4fe0e0
    Lifted::new(0x7000bea170, fn_7000bea170 as *const (), 1), // +0x4fe170 sub_4fe170
    Lifted::new(0x7000bea240, fn_7000bea240 as *const (), 0), // +0x4fe240 sub_4fe240
    Lifted::new(0x7000bea370, fn_7000bea370 as *const (), 0), // +0x4fe370 sub_4fe370
    Lifted::new(0x7000bea4a0, tpl_255::<0x10, 0x18> as *const (), 1), // +0x4fe4a0 sub_4fe4a0
    Lifted::new(0x7000bea4b0, fn_7000bea4b0 as *const (), 0), // +0x4fe4b0 sub_4fe4b0
    Lifted::new(0x7000bea4f0, fn_7000bea4f0 as *const (), 1), // +0x4fe4f0 sub_4fe4f0
    Lifted::new(0x7000bea510, fn_7000bea510 as *const (), 0), // +0x4fe510 sub_4fe510
    Lifted::new(0x7000bea540, fn_7000bea540 as *const (), 1), // +0x4fe540 sub_4fe540
    Lifted::new(0x7000bea570, tpl_753::<0x7001819ca0, 0x8> as *const (), 1), // +0x4fe570 sub_4fe570
    Lifted::new(0x7000bea590, tpl_754::<0x700177c312> as *const (), 0), // +0x4fe590 sub_4fe590
    Lifted::new(0x7000bea5b0, tpl_143::<0x8, 0x70006f6ed0> as *const (), 0), // +0x4fe5b0 sub_4fe5b0
    Lifted::new(0x7000bea5c0, fn_7000bea5c0 as *const (), 0), // +0x4fe5c0 sub_4fe5c0
    Lifted::new(0x7000beaa40, tpl_52::<0x8> as *const (), 1), // +0x4fea40 sub_4fea40
    Lifted::new(0x7000beaa50, tpl_287 as *const (), 1), // +0x4fea50 sub_4fea50
    Lifted::new(0x7000bead70, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x4fed70 sub_4fed70
    Lifted::new(0x7000beade0, tpl_753::<0x7001819ca0, 0x8> as *const (), 1), // +0x4fede0 sub_4fede0
    Lifted::new(0x7000beae00, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x4fee00 sub_4fee00
    Lifted::new(0x7000beae70, tpl_315::<0x7001c6bb00> as *const (), 1), // +0x4fee70 sub_4fee70
    Lifted::new(0x7000beae80, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000beae90> as *const (), 0), // +0x4fee80 sub_4fee80
    Lifted::new(0x7000beaea0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000beaeb0> as *const (), 0), // +0x4feea0 sub_4feea0
    Lifted::new(0x7000beaec0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000beaed0> as *const (), 0), // +0x4feec0 sub_4feec0
    Lifted::new(0x7000beaee0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000beaef0> as *const (), 0), // +0x4feee0 sub_4feee0
    Lifted::new(0x7000beaf00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000beaf10> as *const (), 0), // +0x4fef00 sub_4fef00
    Lifted::new(0x7000beaf20, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000beaf74, 0x70007f0910, 0x7000beaf8f> as *const (), 0), // +0x4fef20 sub_4fef20
    Lifted::new(0x7000beafb0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000beb004, 0x70007f0910, 0x7000beb01f> as *const (), 0), // +0x4fefb0 sub_4fefb0
    Lifted::new(0x7000beb040, fn_7000beb040 as *const (), 0), // +0x4ff040 sub_4ff040
    Lifted::new(0x7000beb0d0, fn_7000beb0d0 as *const (), 1), // +0x4ff0d0 sub_4ff0d0
    Lifted::new(0x7000beb0f0, tpl_800::<0x70018da5bc, 0x700177aa00, 0x7000724770, 0x18, 0x70017d8c38, 0x7000723490, 0x7000beb16c> as *const (), 0), // +0x4ff0f0 sub_4ff0f0
    Lifted::new(0x7000beb190, fn_7000beb190 as *const (), 0), // +0x4ff190 sub_4ff190
    Lifted::new(0x7000beb240, fn_7000beb240 as *const (), 0), // +0x4ff240 sub_4ff240
    Lifted::new(0x7000beb3e0, tpl_792::<0x7001819d70, 0x70007844b0, 0x7000770500> as *const (), 0), // +0x4ff3e0 sub_4ff3e0
    Lifted::new(0x7000beb410, fn_7000beb410 as *const (), 0), // +0x4ff410 sub_4ff410
    Lifted::new(0x7000beb510, tpl_793::<0x7001819d70, 0x70007844b0, 0x7000770500, 0x700177c312> as *const (), 0), // +0x4ff510 sub_4ff510
    Lifted::new(0x7000beb570, fn_7000beb570 as *const (), 0), // +0x4ff570 sub_4ff570
    Lifted::new(0x7000beb620, tpl_15 as *const (), 1), // +0x4ff620 sub_4ff620
    Lifted::new(0x7000beb630, tpl_803::<0x20> as *const (), 1), // +0x4ff630 sub_4ff630
    Lifted::new(0x7000beb650, fn_7000beb650 as *const (), 0), // +0x4ff650 sub_4ff650
    Lifted::new(0x7000beb850, fn_7000beb850 as *const (), 0), // +0x4ff850 sub_4ff850
    Lifted::new(0x7000beb980, fn_7000beb980 as *const (), 0), // +0x4ff980 sub_4ff980
    Lifted::new(0x7000beb9c0, fn_7000beb9c0 as *const (), 1), // +0x4ff9c0 sub_4ff9c0
    Lifted::new(0x7000beba10, tpl_52::<0xa6e8> as *const (), 1), // +0x4ffa10 sub_4ffa10
    Lifted::new(0x7000beba20, tpl_52::<0xa6e0> as *const (), 1), // +0x4ffa20 sub_4ffa20
    Lifted::new(0x7000beba30, tpl_52::<0xa6d8> as *const (), 1), // +0x4ffa30 sub_4ffa30
    Lifted::new(0x7000beba40, fn_7000beba40 as *const (), 1), // +0x4ffa40 sub_4ffa40
    Lifted::new(0x7000beba70, fn_7000beba70 as *const (), 0), // +0x4ffa70 sub_4ffa70
    Lifted::new(0x7000bebb40, fn_7000bebb40 as *const (), 0), // +0x4ffb40 sub_4ffb40
    Lifted::new(0x7000bebc10, fn_7000bebc10 as *const (), 0), // +0x4ffc10 sub_4ffc10
    Lifted::new(0x7000bebc50, fn_7000bebc50 as *const (), 0), // +0x4ffc50 sub_4ffc50
    Lifted::new(0x7000bebcd0, fn_7000bebcd0 as *const (), 0), // +0x4ffcd0 sub_4ffcd0
    Lifted::new(0x7000bebdc0, fn_7000bebdc0 as *const (), 1), // +0x4ffdc0 sub_4ffdc0
    Lifted::new(0x7000bebdf0, fn_7000bebdf0 as *const (), 0), // +0x4ffdf0 sub_4ffdf0
    Lifted::new(0x7000bebe50, fn_7000bebe50 as *const (), 0), // +0x4ffe50 sub_4ffe50
    Lifted::new(0x7000bebef0, fn_7000bebef0 as *const (), 0), // +0x4ffef0 sub_4ffef0
    Lifted::new(0x7000bec050, fn_7000bec050 as *const (), 0), // +0x500050 sub_500050
    Lifted::new(0x7000bec160, fn_7000bec160 as *const (), 1), // +0x500160 sub_500160
    Lifted::new(0x7000bec180, fn_7000bec180 as *const (), 0), // +0x500180 sub_500180
    Lifted::new(0x7000bec1e0, fn_7000bec1e0 as *const (), 0), // +0x5001e0 sub_5001e0
    Lifted::new(0x7000bec260, fn_7000bec260 as *const (), 0), // +0x500260 sub_500260
    Lifted::new(0x7000bec300, fn_7000bec300 as *const (), 0), // +0x500300 sub_500300
    Lifted::new(0x7000bec3b0, fn_7000bec3b0 as *const (), 0), // +0x5003b0 sub_5003b0
    Lifted::new(0x7000bec4a0, fn_7000bec4a0 as *const (), 0), // +0x5004a0 sub_5004a0
    Lifted::new(0x7000bec5e0, fn_7000bec5e0 as *const (), 0), // +0x5005e0 sub_5005e0
    Lifted::new(0x7000bec650, tpl_55::<0x700177c0d0> as *const (), 0), // +0x500650 sub_500650
    Lifted::new(0x7000bec7a0, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000bed880, 0x70017d37c0, 0x70017790e0, 0x7000beca81> as *const (), 2), // +0x5007a0 sub_5007a0
    Lifted::new(0x7000becb40, tpl_68 as *const (), 1), // +0x500b40 sub_500b40
    Lifted::new(0x7000becbf0, fn_7000becbf0 as *const (), 3), // +0x500bf0 sub_500bf0
    Lifted::new(0x7000bece60, fn_7000bece60 as *const (), 0), // +0x500e60 sub_500e60
    Lifted::new(0x7000beced0, fn_7000beced0 as *const (), 0), // +0x500ed0 sub_500ed0
    Lifted::new(0x7000becf40, fn_7000becf40 as *const (), 0), // +0x500f40 sub_500f40
    Lifted::new(0x7000bed040, fn_7000bed040 as *const (), 1), // +0x501040 sub_501040
    Lifted::new(0x7000bed080, fn_7000bed080 as *const (), 2), // +0x501080 sub_501080
    Lifted::new(0x7000bed510, fn_7000bed510 as *const (), 0), // +0x501510 sub_501510
    Lifted::new(0x7000bed630, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000bed737, 0x7000becb40, 0x70007f0910, 0x7000bed753> as *const (), 0), // +0x501630 sub_501630
    Lifted::new(0x7000bed7a0, tpl_999::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000bed850> as *const (), 0), // +0x5017a0 sub_5017a0
    Lifted::new(0x7000bed880, tpl_286::<0x7000bed630, 0x70017d37a0, 0x70017790e0, 0x7000beda6b> as *const (), 0), // +0x501880 sub_501880
    Lifted::new(0x7000bedaf0, tpl_1000::<0x700177c312> as *const (), 0), // +0x501af0 sub_501af0
    Lifted::new(0x7000bedda0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000beddb0> as *const (), 0), // +0x501da0 sub_501da0
    Lifted::new(0x7000beddc0, tpl_1001::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000bede18, 0x70007f0910, 0x7000bede33> as *const (), 0), // +0x501dc0 sub_501dc0
    Lifted::new(0x7000bede60, tpl_320::<0x10, 0x38, 0x8, 0x7000bedaf0, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x501e60 sub_501e60
    Lifted::new(0x7000bedf50, tpl_1002::<0x7000beddc0, 0x7000bed7a0, 0x7000bedda0, 0x7000bee0f8> as *const (), 2), // +0x501f50 sub_501f50
    Lifted::new(0x7000bee1d0, fn_7000bee1d0 as *const (), 0), // +0x5021d0 sub_5021d0
    Lifted::new(0x7000bee270, fn_7000bee270 as *const (), 2), // +0x502270 sub_502270
    Lifted::new(0x7000bee520, fn_7000bee520 as *const (), 3), // +0x502520 sub_502520
    Lifted::new(0x7000bee610, fn_7000bee610 as *const (), 0), // +0x502610 sub_502610
    Lifted::new(0x7000bee770, fn_7000bee770 as *const (), 1), // +0x502770 sub_502770
    Lifted::new(0x7000bee800, fn_7000bee800 as *const (), 0), // +0x502800 sub_502800
    Lifted::new(0x7000bee8e0, fn_7000bee8e0 as *const (), 0), // +0x5028e0 sub_5028e0
    Lifted::new(0x7000beea90, fn_7000beea90 as *const (), 0), // +0x502a90 sub_502a90
    Lifted::new(0x7000beebb0, fn_7000beebb0 as *const (), 0), // +0x502bb0 sub_502bb0
    Lifted::new(0x7000beec00, tpl_36::<0xc0> as *const (), 1), // +0x502c00 sub_502c00
    Lifted::new(0x7000beec10, fn_7000beec10 as *const (), 2), // +0x502c10 sub_502c10
    Lifted::new(0x7000beed40, fn_7000beed40 as *const (), 0), // +0x502d40 sub_502d40
    Lifted::new(0x7000beee00, fn_7000beee00 as *const (), 0), // +0x502e00 sub_502e00
    Lifted::new(0x7000beefb0, fn_7000beefb0 as *const (), 2), // +0x502fb0 sub_502fb0
    Lifted::new(0x7000bef190, tpl_157::<0x7000beefb0> as *const (), 0), // +0x503190 sub_503190
    Lifted::new(0x7000bef1a0, tpl_243::<0x7000beefb0> as *const (), 0), // +0x5031a0 sub_5031a0
    Lifted::new(0x7000bef1b0, fn_7000bef1b0 as *const (), 2), // +0x5031b0 sub_5031b0
    Lifted::new(0x7000bef4c0, fn_7000bef4c0 as *const (), 2), // +0x5034c0 sub_5034c0
    Lifted::new(0x7000bef590, fn_7000bef590 as *const (), 0), // +0x503590 sub_503590
    Lifted::new(0x7000bef6d0, fn_7000bef6d0 as *const (), 2), // +0x5036d0 sub_5036d0
    Lifted::new(0x7000bef950, fn_7000bef950 as *const (), 3), // +0x503950 sub_503950
    Lifted::new(0x7000bef990, fn_7000bef990 as *const (), 1), // +0x503990 sub_503990
    Lifted::new(0x7000bef9f0, tpl_36::<0xb8> as *const (), 1), // +0x5039f0 sub_5039f0
    Lifted::new(0x7000befa00, tpl_918::<0x7001819f00, 0x8> as *const (), 3), // +0x503a00 sub_503a00
    Lifted::new(0x7000befa20, tpl_919::<0x700177c312> as *const (), 0), // +0x503a20 sub_503a20
    Lifted::new(0x7000befa40, tpl_110::<0x7000bee270> as *const (), 0), // +0x503a40 sub_503a40
    Lifted::new(0x7000befa50, tpl_52::<0x8> as *const (), 1), // +0x503a50 sub_503a50
    Lifted::new(0x7000befa60, tpl_918::<0x7001819f00, 0x8> as *const (), 3), // +0x503a60 sub_503a60
    Lifted::new(0x7000befa80, tpl_315::<0x7001c6bb50> as *const (), 1), // +0x503a80 sub_503a80
    Lifted::new(0x7000befa90, fn_7000befa90 as *const (), 0), // +0x503a90 sub_503a90
    Lifted::new(0x7000befb90, fn_7000befb90 as *const (), 2), // +0x503b90 sub_503b90
    Lifted::new(0x7000befc10, fn_7000befc10 as *const (), 1), // +0x503c10 sub_503c10
    Lifted::new(0x7000befc30, fn_7000befc30 as *const (), 2), // +0x503c30 sub_503c30
    Lifted::new(0x7000befd60, fn_7000befd60 as *const (), 1), // +0x503d60 sub_503d60
    Lifted::new(0x7000beff10, fn_7000beff10 as *const (), 0), // +0x503f10 sub_503f10
    Lifted::new(0x7000bf0450, fn_7000bf0450 as *const (), 1), // +0x504450 sub_504450
    Lifted::new(0x7000bf0460, fn_7000bf0460 as *const (), 0), // +0x504460 sub_504460
    Lifted::new(0x7000bf0530, fn_7000bf0530 as *const (), 0), // +0x504530 sub_504530
    Lifted::new(0x7000bf05d0, fn_7000bf05d0 as *const (), 0), // +0x5045d0 sub_5045d0
    Lifted::new(0x7000bf0680, fn_7000bf0680 as *const (), 0), // +0x504680 sub_504680
    Lifted::new(0x7000bf06a0, fn_7000bf06a0 as *const (), 0), // +0x5046a0 sub_5046a0
    Lifted::new(0x7000bf0700, tpl_914::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000bf0787> as *const (), 0), // +0x504700 sub_504700
    Lifted::new(0x7000bf07b0, tpl_103::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000bf080b> as *const (), 0), // +0x5047b0 sub_5047b0
    Lifted::new(0x7000bf0850, tpl_1003::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000bf1c30, 0x70017d37c0, 0x70017790e0, 0x7000bf0b42> as *const (), 2), // +0x504850 sub_504850
    Lifted::new(0x7000bf0c00, tpl_68 as *const (), 1), // +0x504c00 sub_504c00
    Lifted::new(0x7000bf0cf0, fn_7000bf0cf0 as *const (), 0), // +0x504cf0 sub_504cf0
    Lifted::new(0x7000bf0db0, tpl_131::<0xfffffffffffffff8, 0x700076f000, 0x700177c312> as *const (), 0), // +0x504db0 sub_504db0
    Lifted::new(0x7000bf0e60, tpl_35::<0x700181a108, 0x700177c312> as *const (), 0), // +0x504e60 sub_504e60
    Lifted::new(0x7000bf0e90, tpl_133::<0x700076f000, 0x700177c312> as *const (), 0), // +0x504e90 sub_504e90
    Lifted::new(0x7000bf0ed0, fn_7000bf0ed0 as *const (), 2), // +0x504ed0 sub_504ed0
    Lifted::new(0x7000bf1310, fn_7000bf1310 as *const (), 0), // +0x505310 sub_505310
    Lifted::new(0x7000bf13b0, tpl_15 as *const (), 1), // +0x5053b0 sub_5053b0
    Lifted::new(0x7000bf13c0, fn_7000bf13c0 as *const (), 0), // +0x5053c0 sub_5053c0
    Lifted::new(0x7000bf1510, fn_7000bf1510 as *const (), 2), // +0x505510 sub_505510
    Lifted::new(0x7000bf1640, fn_7000bf1640 as *const (), 2), // +0x505640 sub_505640
    Lifted::new(0x7000bf1730, tpl_1004::<0x8> as *const (), 1), // +0x505730 sub_505730
    Lifted::new(0x7000bf1740, fn_7000bf1740 as *const (), 0), // +0x505740 sub_505740
    Lifted::new(0x7000bf1850, fn_7000bf1850 as *const (), 0), // +0x505850 sub_505850
    Lifted::new(0x7000bf18b0, fn_7000bf18b0 as *const (), 0), // +0x5058b0 sub_5058b0
    Lifted::new(0x7000bf1a90, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000bf1b97, 0x7000bf0c00, 0x70007f0910, 0x7000bf1bb3> as *const (), 0), // +0x505a90 sub_505a90
    Lifted::new(0x7000bf1c00, tpl_530::<0x10> as *const (), 0), // +0x505c00 sub_505c00
    Lifted::new(0x7000bf1c20, tpl_1005::<0x700076f000> as *const (), 0), // +0x505c20 sub_505c20
    Lifted::new(0x7000bf1c30, tpl_286::<0x7000bf1a90, 0x70017d37a0, 0x70017790e0, 0x7000bf1e1b> as *const (), 0), // +0x505c30 sub_505c30
    Lifted::new(0x7000bf1ea0, fn_7000bf1ea0 as *const (), 0), // +0x505ea0 sub_505ea0
    Lifted::new(0x7000bf2070, fn_7000bf2070 as *const (), 0), // +0x506070 sub_506070
    Lifted::new(0x7000bf2280, fn_7000bf2280 as *const (), 0), // +0x506280 sub_506280
    Lifted::new(0x7000bf23b0, fn_7000bf23b0 as *const (), 1), // +0x5063b0 sub_5063b0
    Lifted::new(0x7000bf23d0, fn_7000bf23d0 as *const (), 0), // +0x5063d0 sub_5063d0
    Lifted::new(0x7000bf2580, tpl_1006::<0x7001779e20, 0xffffffffffff6898, 0x7000763140, 0x700181a180, 0x28, 0x700181a198, 0x20> as *const (), 0), // +0x506580 sub_506580
    Lifted::new(0x7000bf2640, tpl_899::<0x7000bf2650> as *const (), 0), // +0x506640 sub_506640
    Lifted::new(0x7000bf2650, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x506650 sub_506650
    Lifted::new(0x7000bf2690, fn_7000bf2690 as *const (), 0), // +0x506690 sub_506690
    Lifted::new(0x7000bf27f0, fn_7000bf27f0 as *const (), 0), // +0x5067f0 sub_5067f0
    Lifted::new(0x7000bf2a80, fn_7000bf2a80 as *const (), 0), // +0x506a80 sub_506a80
    Lifted::new(0x7000bf2ae0, fn_7000bf2ae0 as *const (), 0), // +0x506ae0 sub_506ae0
    Lifted::new(0x7000bf2b70, fn_7000bf2b70 as *const (), 0), // +0x506b70 sub_506b70
    Lifted::new(0x7000bf2bc0, tpl_143::<0x28, 0x7000c45d00> as *const (), 0), // +0x506bc0 sub_506bc0
    Lifted::new(0x7000bf2bd0, fn_7000bf2bd0 as *const (), 2), // +0x506bd0 sub_506bd0
    Lifted::new(0x7000bf2e50, tpl_47::<0x20> as *const (), 1), // +0x506e50 sub_506e50
    Lifted::new(0x7000bf2e60, fn_7000bf2e60 as *const (), 0), // +0x506e60 sub_506e60
    Lifted::new(0x7000bf2ed0, fn_7000bf2ed0 as *const (), 0), // +0x506ed0 sub_506ed0
    Lifted::new(0x7000bf2f40, fn_7000bf2f40 as *const (), 0), // +0x506f40 sub_506f40
    Lifted::new(0x7000bf2fe0, tpl_899::<0x7000bf2ff0> as *const (), 0), // +0x506fe0 sub_506fe0
    Lifted::new(0x7000bf2ff0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x506ff0 sub_506ff0
    Lifted::new(0x7000bf3030, fn_7000bf3030 as *const (), 2), // +0x507030 sub_507030
    Lifted::new(0x7000bf3260, fn_7000bf3260 as *const (), 0), // +0x507260 sub_507260
    Lifted::new(0x7000bf3350, fn_7000bf3350 as *const (), 0), // +0x507350 sub_507350
    Lifted::new(0x7000bf3440, fn_7000bf3440 as *const (), 2), // +0x507440 sub_507440
    Lifted::new(0x7000bf3630, fn_7000bf3630 as *const (), 0), // +0x507630 sub_507630
    Lifted::new(0x7000bf3650, tpl_1005::<0x7000ce3220> as *const (), 0), // +0x507650 sub_507650
    Lifted::new(0x7000bf3660, tpl_1007::<0x20, 0x28, 0xfffffffffffffffc, 0x7000743b40> as *const (), 0), // +0x507660 sub_507660
    Lifted::new(0x7000bf3690, fn_7000bf3690 as *const (), 2), // +0x507690 sub_507690
    Lifted::new(0x7000bf3c30, fn_7000bf3c30 as *const (), 2), // +0x507c30 sub_507c30
    Lifted::new(0x7000bf3ce0, fn_7000bf3ce0 as *const (), 0), // +0x507ce0 sub_507ce0
    Lifted::new(0x7000bf3d40, fn_7000bf3d40 as *const (), 1), // +0x507d40 sub_507d40
    Lifted::new(0x7000bf3d60, fn_7000bf3d60 as *const (), 1), // +0x507d60 sub_507d60
    Lifted::new(0x7000bf3e50, fn_7000bf3e50 as *const (), 2), // +0x507e50 sub_507e50
    Lifted::new(0x7000bf3f80, tpl_1008::<0xffffffffffffffff, 0x3, 0x7000bf5e10, 0x7000bf5f20> as *const (), 0), // +0x507f80 sub_507f80
    Lifted::new(0x7000bf4060, tpl_1009::<0x23, 0xf, 0x7000bf5e10, 0x4, 0x7000bf5f20> as *const (), 0), // +0x508060 sub_508060
    Lifted::new(0x7000bf4180, tpl_1010::<0x3, 0x38, 0x13, 0x7, 0x7000bf5e10> as *const (), 0), // +0x508180 sub_508180
    Lifted::new(0x7000bf4240, tpl_1011::<0x7000bf5e10> as *const (), 0), // +0x508240 sub_508240
    Lifted::new(0x7000bf4360, tpl_1012::<0x7, 0x3, 0x7000bf5e10> as *const (), 0), // +0x508360 sub_508360
    Lifted::new(0x7000bf4410, tpl_1012::<0x13, 0x3, 0x7000bf5e10> as *const (), 0), // +0x508410 sub_508410
    Lifted::new(0x7000bf44c0, tpl_1012::<0x1b, 0x3, 0x7000bf5e10> as *const (), 0), // +0x5084c0 sub_5084c0
    Lifted::new(0x7000bf4570, tpl_1012::<0xf, 0xb, 0x7000bf5e10> as *const (), 0), // +0x508570 sub_508570
    Lifted::new(0x7000bf4620, tpl_1013::<0x7000bf44c0, 0x27, 0x1b, 0x7000bf5e10> as *const (), 0), // +0x508620 sub_508620
    Lifted::new(0x7000bf46d0, tpl_1013::<0x7000bf44c0, 0x47, 0x1b, 0x7000bf5e10> as *const (), 0), // +0x5086d0 sub_5086d0
    Lifted::new(0x7000bf4780, tpl_1014::<0xffffffffffffffff, 0x3, 0x7000bf5e10> as *const (), 0), // +0x508780 sub_508780
    Lifted::new(0x7000bf4830, tpl_1014::<0xffffffffffffffff, 0x7, 0x7000bf5e10> as *const (), 0), // +0x508830 sub_508830
    Lifted::new(0x7000bf48e0, tpl_1014::<0xffffffffffffffff, 0x17, 0x7000bf5e10> as *const (), 0), // +0x5088e0 sub_5088e0
    Lifted::new(0x7000bf4990, tpl_1014::<0xffffffffffffffff, 0xb, 0x7000bf5e10> as *const (), 0), // +0x508990 sub_508990
    Lifted::new(0x7000bf4a40, tpl_1015::<0x5, 0x7, 0x7000bf5e10> as *const (), 0), // +0x508a40 sub_508a40
    Lifted::new(0x7000bf4af0, tpl_1016::<0x5, 0x7, 0x7000bf5e10> as *const (), 0), // +0x508af0 sub_508af0
    Lifted::new(0x7000bf4ba0, tpl_1017::<0x9, 0xb, 0x7000bf5e10> as *const (), 0), // +0x508ba0 sub_508ba0
    Lifted::new(0x7000bf4c60, tpl_1012::<0xf, 0x3, 0x7000bf5e10> as *const (), 0), // +0x508c60 sub_508c60
    Lifted::new(0x7000bf4d10, tpl_1018::<0x13, 0x3, 0x7000bf5e10, 0x7001c72d48, 0x8, 0x10, 0x700076c500, 0x7000f405a0> as *const (), 0), // +0x508d10 sub_508d10
    Lifted::new(0x7000bf4e30, tpl_1019::<0x3, 0x7000bf5e10> as *const (), 0), // +0x508e30 sub_508e30
    Lifted::new(0x7000bf4ee0, tpl_1020::<0xb, 0x3, 0x7000bf5e10, 0x7000bf5b00> as *const (), 0), // +0x508ee0 sub_508ee0
    Lifted::new(0x7000bf4fa0, tpl_1021::<0x7, 0xfffffffffffffff7, 0x7000bf5e10> as *const (), 0), // +0x508fa0 sub_508fa0
    Lifted::new(0x7000bf5080, tpl_1022::<0xb, 0x3, 0x7000bf5e10, 0x7000bf5c90> as *const (), 0), // +0x509080 sub_509080
    Lifted::new(0x7000bf5140, tpl_1022::<0xb, 0x3, 0x7000bf5e10, 0x7000bf5c90> as *const (), 0), // +0x509140 sub_509140
    Lifted::new(0x7000bf5200, tpl_1012::<0x3fb, 0x3, 0x7000bf5e10> as *const (), 0), // +0x509200 sub_509200
    Lifted::new(0x7000bf52b0, tpl_1023::<0x7000bf5e10, 0x7000bf5f20> as *const (), 0), // +0x5092b0 sub_5092b0
    Lifted::new(0x7000bf5410, tpl_1024::<0x7, 0xffffffffffffffff, 0xb, 0x10, 0x7000bf5e10> as *const (), 0), // +0x509410 sub_509410
    Lifted::new(0x7000bf54d0, tpl_1025::<0x7000bf5c90> as *const (), 0), // +0x5094d0 sub_5094d0
    Lifted::new(0x7000bf5520, tpl_1026::<0x13, 0xffffffffffffffff, 0x1, 0x3, 0xb, 0x7000bf5e10> as *const (), 0), // +0x509520 sub_509520
    Lifted::new(0x7000bf5600, tpl_1012::<0x1b, 0xb, 0x7000bf5e10> as *const (), 0), // +0x509600 sub_509600
    Lifted::new(0x7000bf56b0, tpl_1012::<0x2b, 0xb, 0x7000bf5e10> as *const (), 0), // +0x5096b0 sub_5096b0
    Lifted::new(0x7000bf5760, tpl_1012::<0x23, 0xb, 0x7000bf5e10> as *const (), 0), // +0x509760 sub_509760
    Lifted::new(0x7000bf5810, tpl_1027::<0x7000bf5e10> as *const (), 0), // +0x509810 sub_509810
    Lifted::new(0x7000bf5940, tpl_1028::<0x7000bf5c90> as *const (), 0), // +0x509940 sub_509940
    Lifted::new(0x7000bf59a0, tpl_1029::<0x7000e87780, 0x7000bf5d40, 0x7000bf5e10> as *const (), 0), // +0x5099a0 sub_5099a0
    Lifted::new(0x7000bf5b00, tpl_1030::<0xffffffffffffffff, 0x7000bf5e10, 0x7000bf5f20> as *const (), 0), // +0x509b00 sub_509b00
    Lifted::new(0x7000bf5be0, tpl_1031::<0xffffffffffffffff, 0x4, 0x7000bf5e10> as *const (), 0), // +0x509be0 sub_509be0
    Lifted::new(0x7000bf5c90, tpl_1031::<0xffffffffffffffff, 0x4, 0x7000bf5e10> as *const (), 0), // +0x509c90 sub_509c90
    Lifted::new(0x7000bf5d40, tpl_1032::<0x7000c01050, 0xffffffffffffffff, 0x7000bf5e10> as *const (), 0), // +0x509d40 sub_509d40
    Lifted::new(0x7000bf5e10, fn_7000bf5e10 as *const (), 0), // +0x509e10 sub_509e10
    Lifted::new(0x7000bf5f20, tpl_1033::<0x7000bf3e50, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf7a00> as *const (), 0), // +0x509f20 sub_509f20
    Lifted::new(0x7000bf60d0, tpl_1034::<0x8, 0x7000bf3e50> as *const (), 0), // +0x50a0d0 sub_50a0d0
    Lifted::new(0x7000bf6120, fn_7000bf6120 as *const (), 1), // +0x50a120 sub_50a120
    Lifted::new(0x7000bf6170, fn_7000bf6170 as *const (), 1), // +0x50a170 sub_50a170
    Lifted::new(0x7000bf61c0, fn_7000bf61c0 as *const (), 0), // +0x50a1c0 sub_50a1c0
    Lifted::new(0x7000bf6320, fn_7000bf6320 as *const (), 0), // +0x50a320 sub_50a320
    Lifted::new(0x7000bf6480, fn_7000bf6480 as *const (), 0), // +0x50a480 sub_50a480
    Lifted::new(0x7000bf64e0, fn_7000bf64e0 as *const (), 0), // +0x50a4e0 sub_50a4e0
    Lifted::new(0x7000bf6650, fn_7000bf6650 as *const (), 0), // +0x50a650 sub_50a650
    Lifted::new(0x7000bf6830, tpl_1035::<0x7000bffa90, 0x3, 0x5, 0x7000c01050, 0x7000bf59a0> as *const (), 0), // +0x50a830 sub_50a830
    Lifted::new(0x7000bf6900, fn_7000bf6900 as *const (), 0), // +0x50a900 sub_50a900
    Lifted::new(0x7000bf6a50, fn_7000bf6a50 as *const (), 0), // +0x50aa50 sub_50aa50
    Lifted::new(0x7000bf6ae0, fn_7000bf6ae0 as *const (), 0), // +0x50aae0 sub_50aae0
    Lifted::new(0x7000bf6ba0, fn_7000bf6ba0 as *const (), 2), // +0x50aba0 sub_50aba0
    Lifted::new(0x7000bf6ed0, fn_7000bf6ed0 as *const (), 2), // +0x50aed0 sub_50aed0
    Lifted::new(0x7000bf7210, tpl_68 as *const (), 1), // +0x50b210 sub_50b210
    Lifted::new(0x7000bf72c0, fn_7000bf72c0 as *const (), 1), // +0x50b2c0 sub_50b2c0
    Lifted::new(0x7000bf72e0, fn_7000bf72e0 as *const (), 0), // +0x50b2e0 sub_50b2e0
    Lifted::new(0x7000bf7390, fn_7000bf7390 as *const (), 1), // +0x50b390 sub_50b390
    Lifted::new(0x7000bf73e0, fn_7000bf73e0 as *const (), 0), // +0x50b3e0 sub_50b3e0
    Lifted::new(0x7000bf7540, fn_7000bf7540 as *const (), 0), // +0x50b540 sub_50b540
    Lifted::new(0x7000bf75c0, tpl_1036 as *const (), 1), // +0x50b5c0 sub_50b5c0
    Lifted::new(0x7000bf7a00, tpl_1036 as *const (), 1), // +0x50ba00 sub_50ba00
    Lifted::new(0x7000bf7e40, tpl_1036 as *const (), 1), // +0x50be40 sub_50be40
    Lifted::new(0x7000bf8280, tpl_1037::<0x10, 0x8, 0x40, 0x100, 0xfffffffffffffff8, 0x18, 0x20, 0x28, 0x30, 0x38, 0x48, 0x50> as *const (), 1), // +0x50c280 sub_50c280
    Lifted::new(0x7000bf83a0, tpl_1037::<0x10, 0x8, 0x40, 0x100, 0xfffffffffffffff8, 0x18, 0x20, 0x28, 0x30, 0x38, 0x48, 0x50> as *const (), 1), // +0x50c3a0 sub_50c3a0
    Lifted::new(0x7000bf84c0, tpl_1038::<0x10, 0x8, 0x40, 0x18, 0x20, 0x28, 0x30, 0x38, 0x48, 0x50, 0x58, 0x60> as *const (), 1), // +0x50c4c0 sub_50c4c0
    Lifted::new(0x7000bf85d0, tpl_1039 as *const (), 1), // +0x50c5d0 sub_50c5d0
    Lifted::new(0x7000bf87e0, tpl_1038::<0x10, 0x8, 0x40, 0x18, 0x20, 0x28, 0x30, 0x38, 0x48, 0x50, 0x58, 0x60> as *const (), 1), // +0x50c7e0 sub_50c7e0
    Lifted::new(0x7000bf88f0, fn_7000bf88f0 as *const (), 1), // +0x50c8f0 sub_50c8f0
    Lifted::new(0x7000bf8b10, fn_7000bf8b10 as *const (), 0), // +0x50cb10 sub_50cb10
    Lifted::new(0x7000bf8bd0, tpl_965::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000bf8c50, 0x7001d54968, 0x7000bf8c30> as *const (), 0), // +0x50cbd0 sub_50cbd0
    Lifted::new(0x7000bf8c50, fn_7000bf8c50 as *const (), 0), // +0x50cc50 sub_50cc50
    Lifted::new(0x7000bf8d20, fn_7000bf8d20 as *const (), 0), // +0x50cd20 sub_50cd20
    Lifted::new(0x7000bf8de0, fn_7000bf8de0 as *const (), 0), // +0x50cde0 sub_50cde0
    Lifted::new(0x7000bf8e70, tpl_33::<0x700177c312> as *const (), 0), // +0x50ce70 sub_50ce70
    Lifted::new(0x7000bf8ea0, tpl_33::<0x700177c312> as *const (), 0), // +0x50cea0 sub_50cea0
    Lifted::new(0x7000bf8ed0, tpl_33::<0x700177c312> as *const (), 0), // +0x50ced0 sub_50ced0
    Lifted::new(0x7000bf8f00, tpl_33::<0x700177c312> as *const (), 0), // +0x50cf00 sub_50cf00
    Lifted::new(0x7000bf8f30, tpl_33::<0x700177c312> as *const (), 0), // +0x50cf30 sub_50cf30
    Lifted::new(0x7000bf8f60, tpl_33::<0x700177c312> as *const (), 0), // +0x50cf60 sub_50cf60
    Lifted::new(0x7000bf8f90, tpl_33::<0x700177c312> as *const (), 0), // +0x50cf90 sub_50cf90
    Lifted::new(0x7000bf8fc0, fn_7000bf8fc0 as *const (), 0), // +0x50cfc0 sub_50cfc0
    Lifted::new(0x7000bf90c0, fn_7000bf90c0 as *const (), 0), // +0x50d0c0 sub_50d0c0
    Lifted::new(0x7000bf9270, fn_7000bf9270 as *const (), 0), // +0x50d270 sub_50d270
    Lifted::new(0x7000bf92e0, tpl_1040::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf75c0> as *const (), 0), // +0x50d2e0 sub_50d2e0
    Lifted::new(0x7000bf9410, tpl_1040::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf7a00> as *const (), 0), // +0x50d410 sub_50d410
    Lifted::new(0x7000bf9540, tpl_1040::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf7e40> as *const (), 0), // +0x50d540 sub_50d540
    Lifted::new(0x7000bf9670, tpl_1041::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf8280> as *const (), 0), // +0x50d670 sub_50d670
    Lifted::new(0x7000bf97a0, tpl_1041::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf83a0> as *const (), 0), // +0x50d7a0 sub_50d7a0
    Lifted::new(0x7000bf98d0, tpl_1041::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf84c0> as *const (), 0), // +0x50d8d0 sub_50d8d0
    Lifted::new(0x7000bf9a00, tpl_1041::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf85d0> as *const (), 0), // +0x50da00 sub_50da00
    Lifted::new(0x7000bf9b30, tpl_1041::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf87e0> as *const (), 0), // +0x50db30 sub_50db30
    Lifted::new(0x7000bf9c60, tpl_1041::<0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf88f0> as *const (), 0), // +0x50dc60 sub_50dc60
    Lifted::new(0x7000bf9d90, fn_7000bf9d90 as *const (), 1), // +0x50dd90 sub_50dd90
    Lifted::new(0x7000bf9e70, tpl_143::<0x8, 0x700074ecc0> as *const (), 0), // +0x50de70 sub_50de70
    Lifted::new(0x7000bf9e80, fn_7000bf9e80 as *const (), 1), // +0x50de80 sub_50de80
    Lifted::new(0x7000bf9eb0, fn_7000bf9eb0 as *const (), 0), // +0x50deb0 sub_50deb0
    Lifted::new(0x7000bf9ef0, fn_7000bf9ef0 as *const (), 0), // +0x50def0 sub_50def0
    Lifted::new(0x7000bf9f80, fn_7000bf9f80 as *const (), 0), // +0x50df80 sub_50df80
    Lifted::new(0x7000bfa060, fn_7000bfa060 as *const (), 0), // +0x50e060 sub_50e060
    Lifted::new(0x7000bfa0a0, fn_7000bfa0a0 as *const (), 1), // +0x50e0a0 sub_50e0a0
    Lifted::new(0x7000bfa160, fn_7000bfa160 as *const (), 0), // +0x50e160 sub_50e160
    Lifted::new(0x7000bfa1a0, fn_7000bfa1a0 as *const (), 1), // +0x50e1a0 sub_50e1a0
    Lifted::new(0x7000bfa270, fn_7000bfa270 as *const (), 0), // +0x50e270 sub_50e270
    Lifted::new(0x7000bfa2c0, fn_7000bfa2c0 as *const (), 2), // +0x50e2c0 sub_50e2c0
    Lifted::new(0x7000bfa3f0, tpl_1042::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x50e3f0 sub_50e3f0
    Lifted::new(0x7000bfa470, tpl_1042::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x50e470 sub_50e470
    Lifted::new(0x7000bfa4f0, tpl_1043::<0x10, 0x2, 0x8, 0x7001d54e98, 0x7001d550d8, 0x7000799390, 0x700177c312> as *const (), 0), // +0x50e4f0 sub_50e4f0
    Lifted::new(0x7000bfa600, tpl_1043::<0x10, 0x2, 0x8, 0x7001d54e98, 0x7001d550d8, 0x7000799390, 0x700177c312> as *const (), 0), // +0x50e600 sub_50e600
    Lifted::new(0x7000bfa710, fn_7000bfa710 as *const (), 0), // +0x50e710 sub_50e710
    Lifted::new(0x7000bfa7c0, fn_7000bfa7c0 as *const (), 0), // +0x50e7c0 sub_50e7c0
    Lifted::new(0x7000bfa8e0, fn_7000bfa8e0 as *const (), 2), // +0x50e8e0 sub_50e8e0
    Lifted::new(0x7000bfaa30, fn_7000bfaa30 as *const (), 0), // +0x50ea30 sub_50ea30
    Lifted::new(0x7000bfab30, fn_7000bfab30 as *const (), 0), // +0x50eb30 sub_50eb30
    Lifted::new(0x7000bfade0, fn_7000bfade0 as *const (), 2), // +0x50ede0 sub_50ede0
    Lifted::new(0x7000bfaf30, fn_7000bfaf30 as *const (), 2), // +0x50ef30 sub_50ef30
    Lifted::new(0x7000bfb050, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf8280> as *const (), 0), // +0x50f050 sub_50f050
    Lifted::new(0x7000bfb150, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x50f150 sub_50f150
    Lifted::new(0x7000bfb1b0, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x50f1b0 sub_50f1b0
    Lifted::new(0x7000bfb210, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf83a0> as *const (), 0), // +0x50f210 sub_50f210
    Lifted::new(0x7000bfb310, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf84c0> as *const (), 0), // +0x50f310 sub_50f310
    Lifted::new(0x7000bfb410, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf85d0> as *const (), 0), // +0x50f410 sub_50f410
    Lifted::new(0x7000bfb510, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf87e0> as *const (), 0), // +0x50f510 sub_50f510
    Lifted::new(0x7000bfb610, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf88f0> as *const (), 0), // +0x50f610 sub_50f610
    Lifted::new(0x7000bfb710, fn_7000bfb710 as *const (), 0), // +0x50f710 sub_50f710
    Lifted::new(0x7000bfb840, fn_7000bfb840 as *const (), 0), // +0x50f840 sub_50f840
    Lifted::new(0x7000bfbb20, fn_7000bfbb20 as *const (), 0), // +0x50fb20 sub_50fb20
    Lifted::new(0x7000bfbb70, fn_7000bfbb70 as *const (), 0), // +0x50fb70 sub_50fb70
    Lifted::new(0x7000bfbcc0, fn_7000bfbcc0 as *const (), 2), // +0x50fcc0 sub_50fcc0
    Lifted::new(0x7000bfbda0, fn_7000bfbda0 as *const (), 0), // +0x50fda0 sub_50fda0
    Lifted::new(0x7000bfbe70, fn_7000bfbe70 as *const (), 2), // +0x50fe70 sub_50fe70
    Lifted::new(0x7000bff470, fn_7000bff470 as *const (), 2), // +0x513470 sub_513470
    Lifted::new(0x7000bff780, fn_7000bff780 as *const (), 0), // +0x513780 sub_513780
    Lifted::new(0x7000bff890, fn_7000bff890 as *const (), 0), // +0x513890 sub_513890
    Lifted::new(0x7000bff960, fn_7000bff960 as *const (), 0), // +0x513960 sub_513960
    Lifted::new(0x7000bffa90, fn_7000bffa90 as *const (), 0), // +0x513a90 sub_513a90
    Lifted::new(0x7000bffbd0, fn_7000bffbd0 as *const (), 0), // +0x513bd0 sub_513bd0
    Lifted::new(0x7000bffc00, fn_7000bffc00 as *const (), 1), // +0x513c00 sub_513c00
    Lifted::new(0x7000bffcb0, tpl_1046::<0x13, 0xffffffffffffffff, 0x1, 0x3, 0x8> as *const (), 1), // +0x513cb0 sub_513cb0
    Lifted::new(0x7000bffd20, tpl_1047::<0x8, 0x7001d54e98, 0x7001d550d8, 0x10, 0x7000799390, 0x700177c312> as *const (), 0), // +0x513d20 sub_513d20
    Lifted::new(0x7000bffde0, tpl_1047::<0x8, 0x7001d54e98, 0x7001d550d8, 0x10, 0x7000799390, 0x700177c312> as *const (), 0), // +0x513de0 sub_513de0
    Lifted::new(0x7000bffea0, fn_7000bffea0 as *const (), 1), // +0x513ea0 sub_513ea0
    Lifted::new(0x7000bfff20, fn_7000bfff20 as *const (), 0), // +0x513f20 sub_513f20
    Lifted::new(0x7000bfff40, fn_7000bfff40 as *const (), 0), // +0x513f40 sub_513f40
    Lifted::new(0x7000bfff60, fn_7000bfff60 as *const (), 1), // +0x513f60 sub_513f60
    Lifted::new(0x7000bfffa0, fn_7000bfffa0 as *const (), 0), // +0x513fa0 sub_513fa0
    Lifted::new(0x7000c005b0, fn_7000c005b0 as *const (), 0), // +0x5145b0 sub_5145b0
    Lifted::new(0x7000c008b0, tpl_1048 as *const (), 1), // +0x5148b0 sub_5148b0
    Lifted::new(0x7000c008d0, fn_7000c008d0 as *const (), 1), // +0x5148d0 sub_5148d0
    Lifted::new(0x7000c008f0, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000c00908> as *const (), 0), // +0x5148f0 sub_5148f0
    Lifted::new(0x7000c00910, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000c00928> as *const (), 0), // +0x514910 sub_514910
    Lifted::new(0x7000c00930, fn_7000c00930 as *const (), 0), // +0x514930 sub_514930
    Lifted::new(0x7000c00a70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c00a81> as *const (), 0), // +0x514a70 sub_514a70
    Lifted::new(0x7000c00a90, tpl_1049::<0x700181a4f8, 0x7000723e40, 0x7000c01490, 0x7000bffa90, 0x7000c01500> as *const (), 0), // +0x514a90 sub_514a90
    Lifted::new(0x7000c00be0, tpl_1050::<0x4, 0x20> as *const (), 0), // +0x514be0 sub_514be0
    Lifted::new(0x7000c00bf0, tpl_15 as *const (), 1), // +0x514bf0 sub_514bf0
    Lifted::new(0x7000c00c00, tpl_260::<0x10> as *const (), 0), // +0x514c00 sub_514c00
    Lifted::new(0x7000c00c10, tpl_15 as *const (), 1), // +0x514c10 sub_514c10
    Lifted::new(0x7000c00c20, fn_7000c00c20 as *const (), 2), // +0x514c20 sub_514c20
    Lifted::new(0x7000c00dd0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c00de1> as *const (), 0), // +0x514dd0 sub_514dd0
    Lifted::new(0x7000c00df0, fn_7000c00df0 as *const (), 0), // +0x514df0 sub_514df0
    Lifted::new(0x7000c00e40, tpl_15 as *const (), 1), // +0x514e40 sub_514e40
    Lifted::new(0x7000c00e50, tpl_15 as *const (), 1), // +0x514e50 sub_514e50
    Lifted::new(0x7000c00e60, tpl_15 as *const (), 1), // +0x514e60 sub_514e60
    Lifted::new(0x7000c00e70, fn_7000c00e70 as *const (), 0), // +0x514e70 sub_514e70
    Lifted::new(0x7000c00f70, tpl_1035::<0x7000bffa90, 0x3, 0x5, 0x7000c01050, 0x7000bf59a0> as *const (), 0), // +0x514f70 sub_514f70
    Lifted::new(0x7000c01050, fn_7000c01050 as *const (), 0), // +0x515050 sub_515050
    Lifted::new(0x7000c01100, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c01111> as *const (), 0), // +0x515100 sub_515100
    Lifted::new(0x7000c01120, tpl_15 as *const (), 1), // +0x515120 sub_515120
    Lifted::new(0x7000c01130, tpl_1051::<0x4, 0x7000bf5e10, 0x7000bf5f20> as *const (), 0), // +0x515130 sub_515130
    Lifted::new(0x7000c01210, tpl_1052::<0x4, 0x7000bf5e10> as *const (), 0), // +0x515210 sub_515210
    Lifted::new(0x7000c012c0, tpl_1050::<0x4, 0x8> as *const (), 0), // +0x5152c0 sub_5152c0
    Lifted::new(0x7000c012d0, tpl_1050::<0x4, 0x10> as *const (), 0), // +0x5152d0 sub_5152d0
    Lifted::new(0x7000c012e0, tpl_1053::<0x7000bf5e10, 0x7000bf5f20> as *const (), 0), // +0x5152e0 sub_5152e0
    Lifted::new(0x7000c013c0, tpl_1054::<0x7000bf5e10> as *const (), 0), // +0x5153c0 sub_5153c0
    Lifted::new(0x7000c01470, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c01481> as *const (), 0), // +0x515470 sub_515470
    Lifted::new(0x7000c01490, fn_7000c01490 as *const (), 1), // +0x515490 sub_515490
    Lifted::new(0x7000c01500, fn_7000c01500 as *const (), 0), // +0x515500 sub_515500
    Lifted::new(0x7000c01660, tpl_15 as *const (), 1), // +0x515660 sub_515660
    Lifted::new(0x7000c01670, fn_7000c01670 as *const (), 0), // +0x515670 sub_515670
    Lifted::new(0x7000c016d0, fn_7000c016d0 as *const (), 0), // +0x5156d0 sub_5156d0
    Lifted::new(0x7000c01730, tpl_1049::<0x700181a4f8, 0x7000723e40, 0x7000c01490, 0x7000bffa90, 0x7000c01500> as *const (), 0), // +0x515730 sub_515730
    Lifted::new(0x7000c01880, fn_7000c01880 as *const (), 0), // +0x515880 sub_515880
    Lifted::new(0x7000c019d0, fn_7000c019d0 as *const (), 0), // +0x5159d0 sub_5159d0
    Lifted::new(0x7000c01ba0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c01ca7, 0x7000bf7210, 0x70007f0910, 0x7000c01cc3> as *const (), 0), // +0x515ba0 sub_515ba0
    Lifted::new(0x7000c01d10, tpl_1055::<0x7000c01ba0, 0x70017d37a0, 0x70017790e0, 0x7000c01e86> as *const (), 0), // +0x515d10 sub_515d10
    Lifted::new(0x7000c01ef0, fn_7000c01ef0 as *const (), 1), // +0x515ef0 sub_515ef0
    Lifted::new(0x7000c01f20, fn_7000c01f20 as *const (), 0), // +0x515f20 sub_515f20
    Lifted::new(0x7000c02010, tpl_1056::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c021d6, 0x7000c07d20, 0x7000c021dc, 0x70007f0910, 0x7000c021e2> as *const (), 2), // +0x516010 sub_516010
    Lifted::new(0x7000c02260, tpl_1056::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c02426, 0x7000c07d40, 0x7000c0242c, 0x70007f0910, 0x7000c02432> as *const (), 2), // +0x516260 sub_516260
    Lifted::new(0x7000c025d0, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c07ab0, 0x70017d37c0, 0x70017790e0, 0x7000c028b1> as *const (), 2), // +0x5165d0 sub_5165d0
    Lifted::new(0x7000c02970, tpl_68 as *const (), 1), // +0x516970 sub_516970
    Lifted::new(0x7000c02a20, fn_7000c02a20 as *const (), 0), // +0x516a20 sub_516a20
    Lifted::new(0x7000c02aa0, fn_7000c02aa0 as *const (), 0), // +0x516aa0 sub_516aa0
    Lifted::new(0x7000c02b20, fn_7000c02b20 as *const (), 0), // +0x516b20 sub_516b20
    Lifted::new(0x7000c02bf0, fn_7000c02bf0 as *const (), 0), // +0x516bf0 sub_516bf0
    Lifted::new(0x7000c02c30, tpl_1057::<0x700181b420, 0x8, 0x10, 0x700181b500> as *const (), 1), // +0x516c30 sub_516c30
    Lifted::new(0x7000c02c70, fn_7000c02c70 as *const (), 2), // +0x516c70 sub_516c70
    Lifted::new(0x7000c02e20, tpl_400::<0x8, 0x10, 0x18, 0x20> as *const (), 1), // +0x516e20 sub_516e20
    Lifted::new(0x7000c02e40, fn_7000c02e40 as *const (), 0), // +0x516e40 sub_516e40
    Lifted::new(0x7000c02ed0, fn_7000c02ed0 as *const (), 1), // +0x516ed0 sub_516ed0
    Lifted::new(0x7000c02f20, tpl_1057::<0x700181b420, 0x8, 0x10, 0x700181b490> as *const (), 1), // +0x516f20 sub_516f20
    Lifted::new(0x7000c02f60, tpl_7::<0x700181a8a8> as *const (), 1), // +0x516f60 sub_516f60
    Lifted::new(0x7000c02f80, tpl_7::<0x700181a8a8> as *const (), 1), // +0x516f80 sub_516f80
    Lifted::new(0x7000c02fa0, tpl_1058::<0x700177c0e0> as *const (), 0), // +0x516fa0 sub_516fa0
    Lifted::new(0x7000c02fd0, tpl_400::<0x8, 0x10, 0x18, 0x20> as *const (), 1), // +0x516fd0 sub_516fd0
    Lifted::new(0x7000c02ff0, tpl_1058::<0x700177c0e0> as *const (), 0), // +0x516ff0 sub_516ff0
    Lifted::new(0x7000c03020, tpl_96::<0x700181b420, 0x8, 0x10> as *const (), 1), // +0x517020 sub_517020
    Lifted::new(0x7000c03050, fn_7000c03050 as *const (), 1), // +0x517050 sub_517050
    Lifted::new(0x7000c03080, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c03123> as *const (), 0), // +0x517080 sub_517080
    Lifted::new(0x7000c03150, fn_7000c03150 as *const (), 0), // +0x517150 sub_517150
    Lifted::new(0x7000c031e0, fn_7000c031e0 as *const (), 0), // +0x5171e0 sub_5171e0
    Lifted::new(0x7000c032c0, tpl_15 as *const (), 1), // +0x5172c0 sub_5172c0
    Lifted::new(0x7000c032d0, fn_7000c032d0 as *const (), 1), // +0x5172d0 sub_5172d0
    Lifted::new(0x7000c03300, fn_7000c03300 as *const (), 0), // +0x517300 sub_517300
    Lifted::new(0x7000c033a0, tpl_1059::<0x7000c03080, 0x38, 0x30> as *const (), 0), // +0x5173a0 sub_5173a0
    Lifted::new(0x7000c033f0, tpl_1059::<0x7000c03080, 0x38, 0x30> as *const (), 0), // +0x5173f0 sub_5173f0
    Lifted::new(0x7000c03440, tpl_410::<0x7000c03443, 0x7000c03444, 0x7000c03445, 0x7000c03446, 0x7000c03447, 0x7000c03448, 0x7000c03449, 0x7000c0344a, 0x7000c0344b, 0x7000c0344c, 0x7000c0344d, 0x7000c0344e, 0x7000c0344f> as *const (), 1), // +0x517440 sub_517440
    Lifted::new(0x7000c03460, tpl_15 as *const (), 1), // +0x517460 sub_517460
    Lifted::new(0x7000c03470, tpl_15 as *const (), 1), // +0x517470 sub_517470
    Lifted::new(0x7000c03480, tpl_16::<0x7000c03483, 0x7000c03484, 0x7000c03485, 0x7000c03486, 0x7000c03487, 0x7000c03488, 0x7000c03489, 0x7000c0348a, 0x7000c0348b, 0x7000c0348c, 0x7000c0348d, 0x7000c0348e, 0x7000c0348f, 0x70007a6fc0> as *const (), 0), // +0x517480 sub_517480
    Lifted::new(0x7000c034a0, fn_7000c034a0 as *const (), 0), // +0x5174a0 sub_5174a0
    Lifted::new(0x7000c03540, tpl_1059::<0x7000c03080, 0x38, 0x30> as *const (), 0), // +0x517540 sub_517540
    Lifted::new(0x7000c03590, tpl_410::<0x7000c03593, 0x7000c03594, 0x7000c03595, 0x7000c03596, 0x7000c03597, 0x7000c03598, 0x7000c03599, 0x7000c0359a, 0x7000c0359b, 0x7000c0359c, 0x7000c0359d, 0x7000c0359e, 0x7000c0359f> as *const (), 1), // +0x517590 sub_517590
    Lifted::new(0x7000c035b0, tpl_112::<0x7001d55090, 0x7000c035ba> as *const (), 0), // +0x5175b0 sub_5175b0
    Lifted::new(0x7000c035c0, tpl_112::<0x7001d55090, 0x7000c035ca> as *const (), 0), // +0x5175c0 sub_5175c0
    Lifted::new(0x7000c035d0, tpl_20 as *const (), 1), // +0x5175d0 sub_5175d0
    Lifted::new(0x7000c035e0, tpl_20 as *const (), 1), // +0x5175e0 sub_5175e0
    Lifted::new(0x7000c035f0, tpl_20 as *const (), 1), // +0x5175f0 sub_5175f0
    Lifted::new(0x7000c03600, tpl_20 as *const (), 1), // +0x517600 sub_517600
    Lifted::new(0x7000c03610, tpl_20 as *const (), 1), // +0x517610 sub_517610
    Lifted::new(0x7000c03620, tpl_20 as *const (), 1), // +0x517620 sub_517620
    Lifted::new(0x7000c03630, tpl_20 as *const (), 1), // +0x517630 sub_517630
    Lifted::new(0x7000c03640, tpl_20 as *const (), 1), // +0x517640 sub_517640
    Lifted::new(0x7000c03650, tpl_20 as *const (), 1), // +0x517650 sub_517650
    Lifted::new(0x7000c03660, tpl_20 as *const (), 1), // +0x517660 sub_517660
    Lifted::new(0x7000c03670, tpl_20 as *const (), 1), // +0x517670 sub_517670
    Lifted::new(0x7000c03680, tpl_20 as *const (), 1), // +0x517680 sub_517680
    Lifted::new(0x7000c03690, tpl_20 as *const (), 1), // +0x517690 sub_517690
    Lifted::new(0x7000c036a0, tpl_20 as *const (), 1), // +0x5176a0 sub_5176a0
    Lifted::new(0x7000c036b0, tpl_20 as *const (), 1), // +0x5176b0 sub_5176b0
    Lifted::new(0x7000c036c0, tpl_20 as *const (), 1), // +0x5176c0 sub_5176c0
    Lifted::new(0x7000c036d0, tpl_20 as *const (), 1), // +0x5176d0 sub_5176d0
    Lifted::new(0x7000c036e0, tpl_20 as *const (), 1), // +0x5176e0 sub_5176e0
    Lifted::new(0x7000c036f0, tpl_20 as *const (), 1), // +0x5176f0 sub_5176f0
    Lifted::new(0x7000c03700, tpl_23::<0x8, 0x10> as *const (), 1), // +0x517700 sub_517700
    Lifted::new(0x7000c03720, tpl_21::<0x10> as *const (), 3), // +0x517720 sub_517720
    Lifted::new(0x7000c03740, tpl_20 as *const (), 1), // +0x517740 sub_517740
    Lifted::new(0x7000c03750, tpl_20 as *const (), 1), // +0x517750 sub_517750
    Lifted::new(0x7000c03760, tpl_20 as *const (), 1), // +0x517760 sub_517760
    Lifted::new(0x7000c03770, tpl_20 as *const (), 1), // +0x517770 sub_517770
    Lifted::new(0x7000c03780, tpl_20 as *const (), 1), // +0x517780 sub_517780
    Lifted::new(0x7000c03790, tpl_28 as *const (), 1), // +0x517790 sub_517790
    Lifted::new(0x7000c037a0, tpl_28 as *const (), 1), // +0x5177a0 sub_5177a0
    Lifted::new(0x7000c037b0, tpl_131::<0xfffffffffffffff8, 0x700076acd0, 0x700177c312> as *const (), 0), // +0x5177b0 sub_5177b0
    Lifted::new(0x7000c03860, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x517860 sub_517860
    Lifted::new(0x7000c038b0, fn_7000c038b0 as *const (), 0), // +0x5178b0 sub_5178b0
    Lifted::new(0x7000c03a50, tpl_1060::<0x7000c038b0> as *const (), 0), // +0x517a50 sub_517a50
    Lifted::new(0x7000c03a60, fn_7000c03a60 as *const (), 0), // +0x517a60 sub_517a60
    Lifted::new(0x7000c03a70, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x517a70 sub_517a70
    Lifted::new(0x7000c03ac0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x517ac0 sub_517ac0
    Lifted::new(0x7000c03b10, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x517b10 sub_517b10
    Lifted::new(0x7000c03b60, tpl_35::<0x700181b7c0, 0x700177c312> as *const (), 0), // +0x517b60 sub_517b60
    Lifted::new(0x7000c03b90, fn_7000c03b90 as *const (), 0), // +0x517b90 sub_517b90
    Lifted::new(0x7000c03c40, tpl_133::<0x700076acd0, 0x700177c312> as *const (), 0), // +0x517c40 sub_517c40
    Lifted::new(0x7000c03c80, tpl_33::<0x700177c312> as *const (), 0), // +0x517c80 sub_517c80
    Lifted::new(0x7000c03cb0, tpl_133::<0x700078a390, 0x700177c312> as *const (), 0), // +0x517cb0 sub_517cb0
    Lifted::new(0x7000c03cf0, tpl_33::<0x700177c312> as *const (), 0), // +0x517cf0 sub_517cf0
    Lifted::new(0x7000c03d20, tpl_33::<0x700177c312> as *const (), 0), // +0x517d20 sub_517d20
    Lifted::new(0x7000c03d50, tpl_33::<0x700177c312> as *const (), 0), // +0x517d50 sub_517d50
    Lifted::new(0x7000c03d80, fn_7000c03d80 as *const (), 0), // +0x517d80 sub_517d80
    Lifted::new(0x7000c03e80, tpl_33::<0x700177c312> as *const (), 0), // +0x517e80 sub_517e80
    Lifted::new(0x7000c03eb0, tpl_33::<0x700177c312> as *const (), 0), // +0x517eb0 sub_517eb0
    Lifted::new(0x7000c03ee0, tpl_33::<0x700177c312> as *const (), 0), // +0x517ee0 sub_517ee0
    Lifted::new(0x7000c03f10, tpl_133::<0x700076acd0, 0x700177c312> as *const (), 0), // +0x517f10 sub_517f10
    Lifted::new(0x7000c03f50, tpl_133::<0x700079a720, 0x700177c312> as *const (), 0), // +0x517f50 sub_517f50
    Lifted::new(0x7000c03f90, tpl_33::<0x700177c312> as *const (), 0), // +0x517f90 sub_517f90
    Lifted::new(0x7000c03fc0, tpl_112::<0x7001d55090, 0x7000c03fca> as *const (), 0), // +0x517fc0 sub_517fc0
    Lifted::new(0x7000c03fd0, tpl_112::<0x7001d55090, 0x7000c03fda> as *const (), 0), // +0x517fd0 sub_517fd0
    Lifted::new(0x7000c03fe0, tpl_15 as *const (), 1), // +0x517fe0 sub_517fe0
    Lifted::new(0x7000c03ff0, tpl_15 as *const (), 1), // +0x517ff0 sub_517ff0
    Lifted::new(0x7000c04000, tpl_15 as *const (), 1), // +0x518000 sub_518000
    Lifted::new(0x7000c04010, fn_7000c04010 as *const (), 2), // +0x518010 sub_518010
    Lifted::new(0x7000c040e0, fn_7000c040e0 as *const (), 0), // +0x5180e0 sub_5180e0
    Lifted::new(0x7000c04210, fn_7000c04210 as *const (), 0), // +0x518210 sub_518210
    Lifted::new(0x7000c04300, fn_7000c04300 as *const (), 2), // +0x518300 sub_518300
    Lifted::new(0x7000c04680, fn_7000c04680 as *const (), 0), // +0x518680 sub_518680
    Lifted::new(0x7000c046f0, fn_7000c046f0 as *const (), 0), // +0x5186f0 sub_5186f0
    Lifted::new(0x7000c04750, tpl_1061::<0xf, 0x8, 0x700077b150> as *const (), 0), // +0x518750 sub_518750
    Lifted::new(0x7000c047b0, tpl_1062::<0x8, 0x28, 0x700077b150> as *const (), 0), // +0x5187b0 sub_5187b0
    Lifted::new(0x7000c047e0, fn_7000c047e0 as *const (), 0), // +0x5187e0 sub_5187e0
    Lifted::new(0x7000c04880, tpl_1063::<0x20> as *const (), 1), // +0x518880 sub_518880
    Lifted::new(0x7000c04890, tpl_36::<0x20> as *const (), 1), // +0x518890 sub_518890
    Lifted::new(0x7000c048a0, tpl_15 as *const (), 1), // +0x5188a0 sub_5188a0
    Lifted::new(0x7000c048b0, fn_7000c048b0 as *const (), 0), // +0x5188b0 sub_5188b0
    Lifted::new(0x7000c04920, tpl_15 as *const (), 1), // +0x518920 sub_518920
    Lifted::new(0x7000c04930, fn_7000c04930 as *const (), 0), // +0x518930 sub_518930
    Lifted::new(0x7000c049a0, tpl_15 as *const (), 1), // +0x5189a0 sub_5189a0
    Lifted::new(0x7000c049b0, tpl_15 as *const (), 1), // +0x5189b0 sub_5189b0
    Lifted::new(0x7000c049c0, tpl_20 as *const (), 1), // +0x5189c0 sub_5189c0
    Lifted::new(0x7000c049d0, tpl_20 as *const (), 1), // +0x5189d0 sub_5189d0
    Lifted::new(0x7000c049e0, tpl_20 as *const (), 1), // +0x5189e0 sub_5189e0
    Lifted::new(0x7000c049f0, tpl_20 as *const (), 1), // +0x5189f0 sub_5189f0
    Lifted::new(0x7000c04a00, fn_7000c04a00 as *const (), 2), // +0x518a00 sub_518a00
    Lifted::new(0x7000c04ba0, tpl_44 as *const (), 1), // +0x518ba0 sub_518ba0
    Lifted::new(0x7000c04bb0, fn_7000c04bb0 as *const (), 0), // +0x518bb0 sub_518bb0
    Lifted::new(0x7000c04c60, fn_7000c04c60 as *const (), 0), // +0x518c60 sub_518c60
    Lifted::new(0x7000c04c80, fn_7000c04c80 as *const (), 2), // +0x518c80 sub_518c80
    Lifted::new(0x7000c04fc0, fn_7000c04fc0 as *const (), 0), // +0x518fc0 sub_518fc0
    Lifted::new(0x7000c050d0, fn_7000c050d0 as *const (), 2), // +0x5190d0 sub_5190d0
    Lifted::new(0x7000c051d0, tpl_1064::<0xfffffffffffffe40, 0x7000701d10> as *const (), 0), // +0x5191d0 sub_5191d0
    Lifted::new(0x7000c051f0, fn_7000c051f0 as *const (), 0), // +0x5191f0 sub_5191f0
    Lifted::new(0x7000c05230, fn_7000c05230 as *const (), 0), // +0x519230 sub_519230
    Lifted::new(0x7000c052c0, fn_7000c052c0 as *const (), 0), // +0x5192c0 sub_5192c0
    Lifted::new(0x7000c05340, tpl_1065::<0x8, 0x28> as *const (), 1), // +0x519340 sub_519340
    Lifted::new(0x7000c05350, tpl_1065::<0x8, 0x28> as *const (), 1), // +0x519350 sub_519350
    Lifted::new(0x7000c05360, fn_7000c05360 as *const (), 0), // +0x519360 sub_519360
    Lifted::new(0x7000c05430, fn_7000c05430 as *const (), 0), // +0x519430 sub_519430
    Lifted::new(0x7000c05490, fn_7000c05490 as *const (), 0), // +0x519490 sub_519490
    Lifted::new(0x7000c05510, fn_7000c05510 as *const (), 0), // +0x519510 sub_519510
    Lifted::new(0x7000c055d0, fn_7000c055d0 as *const (), 0), // +0x5195d0 sub_5195d0
    Lifted::new(0x7000c05670, fn_7000c05670 as *const (), 3), // +0x519670 sub_519670
    Lifted::new(0x7000c05730, tpl_1066::<0x108, 0x118, 0x10> as *const (), 3), // +0x519730 sub_519730
    Lifted::new(0x7000c05760, fn_7000c05760 as *const (), 0), // +0x519760 sub_519760
    Lifted::new(0x7000c057a0, tpl_15 as *const (), 1), // +0x5197a0 sub_5197a0
    Lifted::new(0x7000c057b0, fn_7000c057b0 as *const (), 0), // +0x5197b0 sub_5197b0
    Lifted::new(0x7000c057c0, fn_7000c057c0 as *const (), 0), // +0x5197c0 sub_5197c0
    Lifted::new(0x7000c05880, fn_7000c05880 as *const (), 0), // +0x519880 sub_519880
    Lifted::new(0x7000c058d0, fn_7000c058d0 as *const (), 1), // +0x5198d0 sub_5198d0
    Lifted::new(0x7000c058f0, tpl_1067::<0xfffffffffffffe40> as *const (), 1), // +0x5198f0 sub_5198f0
    Lifted::new(0x7000c05910, tpl_1067::<0xfffffffffffffe40> as *const (), 1), // +0x519910 sub_519910
    Lifted::new(0x7000c05930, tpl_1067::<0xfffffffffffffe40> as *const (), 1), // +0x519930 sub_519930
    Lifted::new(0x7000c05950, tpl_1067::<0xfffffffffffffe40> as *const (), 1), // +0x519950 sub_519950
    Lifted::new(0x7000c05970, tpl_1067::<0xfffffffffffffff8> as *const (), 1), // +0x519970 sub_519970
    Lifted::new(0x7000c05990, tpl_1067::<0xfffffffffffffff8> as *const (), 1), // +0x519990 sub_519990
    Lifted::new(0x7000c059b0, tpl_20 as *const (), 1), // +0x5199b0 sub_5199b0
    Lifted::new(0x7000c059c0, tpl_20 as *const (), 1), // +0x5199c0 sub_5199c0
    Lifted::new(0x7000c059d0, tpl_20 as *const (), 1), // +0x5199d0 sub_5199d0
    Lifted::new(0x7000c059e0, tpl_20 as *const (), 1), // +0x5199e0 sub_5199e0
    Lifted::new(0x7000c059f0, tpl_20 as *const (), 1), // +0x5199f0 sub_5199f0
    Lifted::new(0x7000c05a00, tpl_20 as *const (), 1), // +0x519a00 sub_519a00
    Lifted::new(0x7000c05a10, tpl_20 as *const (), 1), // +0x519a10 sub_519a10
    Lifted::new(0x7000c05a20, tpl_20 as *const (), 1), // +0x519a20 sub_519a20
    Lifted::new(0x7000c05a30, tpl_1068::<0x1000> as *const (), 1), // +0x519a30 sub_519a30
    Lifted::new(0x7000c05a50, tpl_1068::<0x1000> as *const (), 1), // +0x519a50 sub_519a50
    Lifted::new(0x7000c05a70, tpl_1069::<0x8> as *const (), 1), // +0x519a70 sub_519a70
    Lifted::new(0x7000c05a80, fn_7000c05a80 as *const (), 1), // +0x519a80 sub_519a80
    Lifted::new(0x7000c05aa0, tpl_455::<0x7001c79e48> as *const (), 1), // +0x519aa0 sub_519aa0
    Lifted::new(0x7000c05ab0, tpl_1070::<0xfffffffffffffef8> as *const (), 1), // +0x519ab0 sub_519ab0
    Lifted::new(0x7000c05ad0, fn_7000c05ad0 as *const (), 0), // +0x519ad0 sub_519ad0
    Lifted::new(0x7000c05c50, fn_7000c05c50 as *const (), 0), // +0x519c50 sub_519c50
    Lifted::new(0x7000c05ca0, tpl_1071::<0xfffffffffffffe48> as *const (), 1), // +0x519ca0 sub_519ca0
    Lifted::new(0x7000c05cc0, fn_7000c05cc0 as *const (), 1), // +0x519cc0 sub_519cc0
    Lifted::new(0x7000c05d00, tpl_275::<0x68, 0x118> as *const (), 1), // +0x519d00 sub_519d00
    Lifted::new(0x7000c05d10, tpl_36::<0x118> as *const (), 1), // +0x519d10 sub_519d10
    Lifted::new(0x7000c05d20, tpl_455::<0x7001c79e48> as *const (), 1), // +0x519d20 sub_519d20
    Lifted::new(0x7000c05d30, tpl_618::<0x8> as *const (), 0), // +0x519d30 sub_519d30
    Lifted::new(0x7000c05d40, tpl_52::<0xb8> as *const (), 1), // +0x519d40 sub_519d40
    Lifted::new(0x7000c05d50, fn_7000c05d50 as *const (), 1), // +0x519d50 sub_519d50
    Lifted::new(0x7000c05dc0, fn_7000c05dc0 as *const (), 0), // +0x519dc0 sub_519dc0
    Lifted::new(0x7000c061d0, tpl_52::<0x130> as *const (), 1), // +0x51a1d0 sub_51a1d0
    Lifted::new(0x7000c061e0, tpl_52::<0x130> as *const (), 1), // +0x51a1e0 sub_51a1e0
    Lifted::new(0x7000c061f0, tpl_52::<0xe0> as *const (), 1), // +0x51a1f0 sub_51a1f0
    Lifted::new(0x7000c06200, tpl_52::<0xe0> as *const (), 1), // +0x51a200 sub_51a200
    Lifted::new(0x7000c06210, tpl_607::<0x8, 0x78> as *const (), 0), // +0x51a210 sub_51a210
    Lifted::new(0x7000c06220, tpl_52::<0x158> as *const (), 1), // +0x51a220 sub_51a220
    Lifted::new(0x7000c06230, tpl_52::<0x158> as *const (), 1), // +0x51a230 sub_51a230
    Lifted::new(0x7000c06240, tpl_52::<0x108> as *const (), 1), // +0x51a240 sub_51a240
    Lifted::new(0x7000c06250, tpl_52::<0x108> as *const (), 1), // +0x51a250 sub_51a250
    Lifted::new(0x7000c06260, tpl_148 as *const (), 1), // +0x51a260 sub_51a260
    Lifted::new(0x7000c06270, tpl_44 as *const (), 1), // +0x51a270 sub_51a270
    Lifted::new(0x7000c06280, fn_7000c06280 as *const (), 1), // +0x51a280 sub_51a280
    Lifted::new(0x7000c062a0, tpl_47::<0x28> as *const (), 1), // +0x51a2a0 sub_51a2a0
    Lifted::new(0x7000c062b0, tpl_172::<0x80> as *const (), 1), // +0x51a2b0 sub_51a2b0
    Lifted::new(0x7000c062c0, tpl_47::<0x2a8> as *const (), 1), // +0x51a2c0 sub_51a2c0
    Lifted::new(0x7000c062d0, tpl_172::<0x8> as *const (), 1), // +0x51a2d0 sub_51a2d0
    Lifted::new(0x7000c062e0, tpl_47::<0x20> as *const (), 1), // +0x51a2e0 sub_51a2e0
    Lifted::new(0x7000c062f0, tpl_259::<0x1a> as *const (), 1), // +0x51a2f0 sub_51a2f0
    Lifted::new(0x7000c06300, fn_7000c06300 as *const (), 1), // +0x51a300 sub_51a300
    Lifted::new(0x7000c06310, tpl_52::<0x48> as *const (), 1), // +0x51a310 sub_51a310
    Lifted::new(0x7000c06320, fn_7000c06320 as *const (), 1), // +0x51a320 sub_51a320
    Lifted::new(0x7000c06330, tpl_47::<0x40> as *const (), 1), // +0x51a330 sub_51a330
    Lifted::new(0x7000c06340, tpl_607::<0x8, 0x60> as *const (), 0), // +0x51a340 sub_51a340
    Lifted::new(0x7000c06350, tpl_52::<0x198> as *const (), 1), // +0x51a350 sub_51a350
    Lifted::new(0x7000c06360, tpl_259::<0x18> as *const (), 1), // +0x51a360 sub_51a360
    Lifted::new(0x7000c06370, tpl_1063::<0x20> as *const (), 1), // +0x51a370 sub_51a370
    Lifted::new(0x7000c06380, fn_7000c06380 as *const (), 1), // +0x51a380 sub_51a380
    Lifted::new(0x7000c063a0, tpl_1072::<0x7000742560, 0x700072e7c0> as *const (), 0), // +0x51a3a0 sub_51a3a0
    Lifted::new(0x7000c06400, tpl_1072::<0x7000736100, 0x700079f6f0> as *const (), 0), // +0x51a400 sub_51a400
    Lifted::new(0x7000c06460, tpl_36::<0x20> as *const (), 1), // +0x51a460 sub_51a460
    Lifted::new(0x7000c06470, fn_7000c06470 as *const (), 0), // +0x51a470 sub_51a470
    Lifted::new(0x7000c06510, fn_7000c06510 as *const (), 0), // +0x51a510 sub_51a510
    Lifted::new(0x7000c065c0, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x51a5c0 sub_51a5c0
    Lifted::new(0x7000c06620, tpl_1073::<0x8, 0x2, 0x7000c06510> as *const (), 2), // +0x51a620 sub_51a620
    Lifted::new(0x7000c066a0, tpl_15 as *const (), 1), // +0x51a6a0 sub_51a6a0
    Lifted::new(0x7000c066b0, fn_7000c066b0 as *const (), 2), // +0x51a6b0 sub_51a6b0
    Lifted::new(0x7000c06810, fn_7000c06810 as *const (), 0), // +0x51a810 sub_51a810
    Lifted::new(0x7000c06890, fn_7000c06890 as *const (), 0), // +0x51a890 sub_51a890
    Lifted::new(0x7000c06a00, tpl_15 as *const (), 1), // +0x51aa00 sub_51aa00
    Lifted::new(0x7000c06a10, tpl_15 as *const (), 1), // +0x51aa10 sub_51aa10
    Lifted::new(0x7000c06a20, tpl_244::<0x18> as *const (), 1), // +0x51aa20 sub_51aa20
    Lifted::new(0x7000c06a30, fn_7000c06a30 as *const (), 2), // +0x51aa30 sub_51aa30
    Lifted::new(0x7000c06c30, tpl_51::<0x20> as *const (), 1), // +0x51ac30 sub_51ac30
    Lifted::new(0x7000c06c40, fn_7000c06c40 as *const (), 1), // +0x51ac40 sub_51ac40
    Lifted::new(0x7000c06c50, tpl_49::<0x1b8> as *const (), 1), // +0x51ac50 sub_51ac50
    Lifted::new(0x7000c06c60, fn_7000c06c60 as *const (), 0), // +0x51ac60 sub_51ac60
    Lifted::new(0x7000c06ce0, fn_7000c06ce0 as *const (), 0), // +0x51ace0 sub_51ace0
    Lifted::new(0x7000c06d50, tpl_51::<0x1b0> as *const (), 1), // +0x51ad50 sub_51ad50
    Lifted::new(0x7000c06d60, tpl_245::<0x1a4> as *const (), 1), // +0x51ad60 sub_51ad60
    Lifted::new(0x7000c06d70, tpl_1074::<0x8> as *const (), 1), // +0x51ad70 sub_51ad70
    Lifted::new(0x7000c06d80, tpl_1074::<0x8> as *const (), 1), // +0x51ad80 sub_51ad80
    Lifted::new(0x7000c06d90, tpl_1075::<0x8> as *const (), 1), // +0x51ad90 sub_51ad90
    Lifted::new(0x7000c06da0, tpl_1075::<0x8> as *const (), 1), // +0x51ada0 sub_51ada0
    Lifted::new(0x7000c06db0, tpl_214::<0x5> as *const (), 1), // +0x51adb0 sub_51adb0
    Lifted::new(0x7000c06dc0, fn_7000c06dc0 as *const (), 0), // +0x51adc0 sub_51adc0
    Lifted::new(0x7000c06e10, tpl_15 as *const (), 1), // +0x51ae10 sub_51ae10
    Lifted::new(0x7000c06e20, tpl_1076 as *const (), 1), // +0x51ae20 sub_51ae20
    Lifted::new(0x7000c06e30, tpl_1069::<0x8> as *const (), 1), // +0x51ae30 sub_51ae30
    Lifted::new(0x7000c06e40, fn_7000c06e40 as *const (), 0), // +0x51ae40 sub_51ae40
    Lifted::new(0x7000c06e80, fn_7000c06e80 as *const (), 0), // +0x51ae80 sub_51ae80
    Lifted::new(0x7000c06ed0, tpl_1067::<0xffffffffffffffd8> as *const (), 1), // +0x51aed0 sub_51aed0
    Lifted::new(0x7000c06ef0, tpl_455::<0x7001c79f00> as *const (), 1), // +0x51aef0 sub_51aef0
    Lifted::new(0x7000c06f00, tpl_455::<0x7001c79ef0> as *const (), 1), // +0x51af00 sub_51af00
    Lifted::new(0x7000c06f10, fn_7000c06f10 as *const (), 2), // +0x51af10 sub_51af10
    Lifted::new(0x7000c07120, fn_7000c07120 as *const (), 2), // +0x51b120 sub_51b120
    Lifted::new(0x7000c073c0, fn_7000c073c0 as *const (), 0), // +0x51b3c0 sub_51b3c0
    Lifted::new(0x7000c074b0, fn_7000c074b0 as *const (), 0), // +0x51b4b0 sub_51b4b0
    Lifted::new(0x7000c075d0, fn_7000c075d0 as *const (), 0), // +0x51b5d0 sub_51b5d0
    Lifted::new(0x7000c07750, tpl_15 as *const (), 1), // +0x51b750 sub_51b750
    Lifted::new(0x7000c07760, tpl_144 as *const (), 1), // +0x51b760 sub_51b760
    Lifted::new(0x7000c07770, tpl_144 as *const (), 1), // +0x51b770 sub_51b770
    Lifted::new(0x7000c07780, tpl_144 as *const (), 1), // +0x51b780 sub_51b780
    Lifted::new(0x7000c07790, tpl_1064::<0xffffffffffffffd8, 0x7000799b00> as *const (), 0), // +0x51b790 sub_51b790
    Lifted::new(0x7000c077b0, tpl_144 as *const (), 1), // +0x51b7b0 sub_51b7b0
    Lifted::new(0x7000c077c0, tpl_144 as *const (), 1), // +0x51b7c0 sub_51b7c0
    Lifted::new(0x7000c077d0, tpl_144 as *const (), 1), // +0x51b7d0 sub_51b7d0
    Lifted::new(0x7000c077e0, tpl_144 as *const (), 1), // +0x51b7e0 sub_51b7e0
    Lifted::new(0x7000c077f0, tpl_144 as *const (), 1), // +0x51b7f0 sub_51b7f0
    Lifted::new(0x7000c07800, tpl_144 as *const (), 1), // +0x51b800 sub_51b800
    Lifted::new(0x7000c07810, tpl_144 as *const (), 1), // +0x51b810 sub_51b810
    Lifted::new(0x7000c07820, tpl_144 as *const (), 1), // +0x51b820 sub_51b820
    Lifted::new(0x7000c07830, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c07841> as *const (), 0), // +0x51b830 sub_51b830
    Lifted::new(0x7000c07850, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c07861> as *const (), 0), // +0x51b850 sub_51b850
    Lifted::new(0x7000c07870, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c07881> as *const (), 0), // +0x51b870 sub_51b870
    Lifted::new(0x7000c07890, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000c078a1> as *const (), 0), // +0x51b890 sub_51b890
    Lifted::new(0x7000c078b0, tpl_260::<0x8> as *const (), 0), // +0x51b8b0 sub_51b8b0
    Lifted::new(0x7000c078c0, fn_7000c078c0 as *const (), 0), // +0x51b8c0 sub_51b8c0
    Lifted::new(0x7000c07910, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c07a17, 0x7000c02970, 0x70007f0910, 0x7000c07a33> as *const (), 0), // +0x51b910 sub_51b910
    Lifted::new(0x7000c07a80, tpl_530::<0x10> as *const (), 0), // +0x51ba80 sub_51ba80
    Lifted::new(0x7000c07aa0, tpl_15 as *const (), 1), // +0x51baa0 sub_51baa0
    Lifted::new(0x7000c07ab0, tpl_286::<0x7000c07910, 0x70017d37a0, 0x70017790e0, 0x7000c07c9b> as *const (), 0), // +0x51bab0 sub_51bab0
    Lifted::new(0x7000c07d20, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c07d30> as *const (), 0), // +0x51bd20 sub_51bd20
    Lifted::new(0x7000c07d40, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c07d50> as *const (), 0), // +0x51bd40 sub_51bd40
    Lifted::new(0x7000c07d60, tpl_36::<0x8> as *const (), 1), // +0x51bd60 sub_51bd60
    Lifted::new(0x7000c07d70, tpl_30 as *const (), 1), // +0x51bd70 sub_51bd70
    Lifted::new(0x7000c07d80, tpl_191::<0x8> as *const (), 1), // +0x51bd80 sub_51bd80
    Lifted::new(0x7000c07d90, tpl_191::<0x8> as *const (), 1), // +0x51bd90 sub_51bd90
    Lifted::new(0x7000c07da0, tpl_191::<0x8> as *const (), 1), // +0x51bda0 sub_51bda0
    Lifted::new(0x7000c07db0, tpl_191::<0x8> as *const (), 1), // +0x51bdb0 sub_51bdb0
    Lifted::new(0x7000c07dc0, tpl_52::<0x38> as *const (), 1), // +0x51bdc0 sub_51bdc0
    Lifted::new(0x7000c07dd0, tpl_52::<0x38> as *const (), 1), // +0x51bdd0 sub_51bdd0
    Lifted::new(0x7000c07de0, tpl_36::<0x20> as *const (), 1), // +0x51bde0 sub_51bde0
    Lifted::new(0x7000c07df0, tpl_52::<0x88> as *const (), 1), // +0x51bdf0 sub_51bdf0
    Lifted::new(0x7000c07e00, tpl_52::<0x300> as *const (), 1), // +0x51be00 sub_51be00
    Lifted::new(0x7000c07e10, tpl_52::<0x310> as *const (), 1), // +0x51be10 sub_51be10
    Lifted::new(0x7000c07e20, fn_7000c07e20 as *const (), 1), // +0x51be20 sub_51be20
    Lifted::new(0x7000c07e40, tpl_191::<0x10> as *const (), 1), // +0x51be40 sub_51be40
    Lifted::new(0x7000c07e50, tpl_36::<0x18> as *const (), 1), // +0x51be50 sub_51be50
    Lifted::new(0x7000c07e60, tpl_191::<0x10> as *const (), 1), // +0x51be60 sub_51be60
    Lifted::new(0x7000c07e70, tpl_191::<0x10> as *const (), 1), // +0x51be70 sub_51be70
    Lifted::new(0x7000c07e80, tpl_1077::<0x700070e6c0, 0x8, 0x10> as *const (), 0), // +0x51be80 sub_51be80
    Lifted::new(0x7000c07ec0, tpl_1077::<0x700079f6f0, 0x8, 0x10> as *const (), 0), // +0x51bec0 sub_51bec0
    Lifted::new(0x7000c07f00, tpl_191::<0x10> as *const (), 1), // +0x51bf00 sub_51bf00
    Lifted::new(0x7000c07f10, tpl_191::<0x10> as *const (), 1), // +0x51bf10 sub_51bf10
    Lifted::new(0x7000c07f20, tpl_36::<0x40> as *const (), 1), // +0x51bf20 sub_51bf20
    Lifted::new(0x7000c07f30, tpl_52::<0x48> as *const (), 1), // +0x51bf30 sub_51bf30
    Lifted::new(0x7000c07f40, tpl_52::<0x48> as *const (), 1), // +0x51bf40 sub_51bf40
    Lifted::new(0x7000c07f50, tpl_30 as *const (), 1), // +0x51bf50 sub_51bf50
    Lifted::new(0x7000c07f60, tpl_36::<0x8> as *const (), 1), // +0x51bf60 sub_51bf60
    Lifted::new(0x7000c07f70, tpl_36::<0x8> as *const (), 1), // +0x51bf70 sub_51bf70
    Lifted::new(0x7000c07f80, tpl_30 as *const (), 1), // +0x51bf80 sub_51bf80
    Lifted::new(0x7000c07f90, tpl_30 as *const (), 1), // +0x51bf90 sub_51bf90
    Lifted::new(0x7000c07fa0, tpl_47::<0x1b8> as *const (), 1), // +0x51bfa0 sub_51bfa0
    Lifted::new(0x7000c07fb0, tpl_1078::<0x198> as *const (), 1), // +0x51bfb0 sub_51bfb0
    Lifted::new(0x7000c07fc0, tpl_1078::<0x190> as *const (), 1), // +0x51bfc0 sub_51bfc0
    Lifted::new(0x7000c07fd0, tpl_36::<0x18> as *const (), 1), // +0x51bfd0 sub_51bfd0
    Lifted::new(0x7000c07fe0, tpl_36::<0x28> as *const (), 1), // +0x51bfe0 sub_51bfe0
    Lifted::new(0x7000c07ff0, tpl_47::<0x34> as *const (), 1), // +0x51bff0 sub_51bff0
    Lifted::new(0x7000c08000, tpl_545::<0x10> as *const (), 1), // +0x51c000 sub_51c000
    Lifted::new(0x7000c08010, tpl_347::<0x30> as *const (), 1), // +0x51c010 sub_51c010
    Lifted::new(0x7000c08020, tpl_52::<0x38> as *const (), 1), // +0x51c020 sub_51c020
    Lifted::new(0x7000c08030, tpl_52::<0x38> as *const (), 1), // +0x51c030 sub_51c030
    Lifted::new(0x7000c08040, tpl_36::<0x80> as *const (), 1), // +0x51c040 sub_51c040
    Lifted::new(0x7000c08050, tpl_36::<0x10> as *const (), 1), // +0x51c050 sub_51c050
    Lifted::new(0x7000c08060, tpl_52::<0xb8> as *const (), 1), // +0x51c060 sub_51c060
    Lifted::new(0x7000c08070, tpl_52::<0xb8> as *const (), 1), // +0x51c070 sub_51c070
    Lifted::new(0x7000c08080, tpl_52::<0x28> as *const (), 1), // +0x51c080 sub_51c080
    Lifted::new(0x7000c08090, tpl_52::<0x28> as *const (), 1), // +0x51c090 sub_51c090
    Lifted::new(0x7000c080a0, tpl_36::<0x1a8> as *const (), 1), // +0x51c0a0 sub_51c0a0
    Lifted::new(0x7000c080b0, tpl_36::<0x58> as *const (), 1), // +0x51c0b0 sub_51c0b0
    Lifted::new(0x7000c080c0, tpl_36::<0x58> as *const (), 1), // +0x51c0c0 sub_51c0c0
    Lifted::new(0x7000c080d0, tpl_36::<0x28> as *const (), 1), // +0x51c0d0 sub_51c0d0
    Lifted::new(0x7000c080e0, tpl_36::<0x28> as *const (), 1), // +0x51c0e0 sub_51c0e0
    Lifted::new(0x7000c080f0, tpl_36::<0x78> as *const (), 1), // +0x51c0f0 sub_51c0f0
    Lifted::new(0x7000c08100, tpl_36::<0x78> as *const (), 1), // +0x51c100 sub_51c100
    Lifted::new(0x7000c08110, tpl_30 as *const (), 1), // +0x51c110 sub_51c110
    Lifted::new(0x7000c08120, tpl_30 as *const (), 1), // +0x51c120 sub_51c120
    Lifted::new(0x7000c08130, tpl_52::<0x8> as *const (), 1), // +0x51c130 sub_51c130
    Lifted::new(0x7000c08140, tpl_52::<0x8> as *const (), 1), // +0x51c140 sub_51c140
    Lifted::new(0x7000c08150, tpl_452::<0x10, 0x8> as *const (), 1), // +0x51c150 sub_51c150
    Lifted::new(0x7000c08160, tpl_452::<0x10, 0x8> as *const (), 1), // +0x51c160 sub_51c160
    Lifted::new(0x7000c08170, tpl_36::<0x10> as *const (), 1), // +0x51c170 sub_51c170
    Lifted::new(0x7000c08180, tpl_36::<0x8> as *const (), 1), // +0x51c180 sub_51c180
    Lifted::new(0x7000c08190, tpl_36::<0x70> as *const (), 1), // +0x51c190 sub_51c190
    Lifted::new(0x7000c081a0, tpl_36::<0x1b0> as *const (), 1), // +0x51c1a0 sub_51c1a0
    Lifted::new(0x7000c081b0, tpl_30 as *const (), 1), // +0x51c1b0 sub_51c1b0
    Lifted::new(0x7000c081c0, tpl_39::<0x1a4> as *const (), 1), // +0x51c1c0 sub_51c1c0
    Lifted::new(0x7000c081d0, tpl_47::<0x1a0> as *const (), 1), // +0x51c1d0 sub_51c1d0
    Lifted::new(0x7000c081e0, tpl_52::<0x320> as *const (), 1), // +0x51c1e0 sub_51c1e0
    Lifted::new(0x7000c081f0, tpl_52::<0x330> as *const (), 1), // +0x51c1f0 sub_51c1f0
    Lifted::new(0x7000c08200, tpl_36::<0x68> as *const (), 1), // +0x51c200 sub_51c200
    Lifted::new(0x7000c08210, tpl_36::<0x68> as *const (), 1), // +0x51c210 sub_51c210
    Lifted::new(0x7000c08220, tpl_52::<0xd0> as *const (), 1), // +0x51c220 sub_51c220
    Lifted::new(0x7000c08230, tpl_52::<0xd0> as *const (), 1), // +0x51c230 sub_51c230
    Lifted::new(0x7000c08240, tpl_52::<0x2f0> as *const (), 1), // +0x51c240 sub_51c240
    Lifted::new(0x7000c08250, fn_7000c08250 as *const (), 0), // +0x51c250 sub_51c250
    Lifted::new(0x7000c082a0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x51c2a0 sub_51c2a0
    Lifted::new(0x7000c082e0, tpl_1079::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c084db, 0x7000c0cbe0, 0x7000c084e1, 0x70007f0910, 0x7000c084e7> as *const (), 0), // +0x51c2e0 sub_51c2e0
    Lifted::new(0x7000c08570, tpl_1080::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c08771, 0x7000c0cc00, 0x7000c08777, 0x70007f0910, 0x7000c0877d> as *const (), 0), // +0x51c570 sub_51c570
    Lifted::new(0x7000c08a30, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c0c870, 0x70017d37c0, 0x70017790e0, 0x7000c08d11> as *const (), 2), // +0x51ca30 sub_51ca30
    Lifted::new(0x7000c08dd0, tpl_1081::<0x7001779e20, 0x700177c312, 0x7000c0cae0, 0x70017d37c0, 0x70017790e0, 0x7000c09073> as *const (), 2), // +0x51cdd0 sub_51cdd0
    Lifted::new(0x7000c09120, tpl_1081::<0x7001779e20, 0x700177c312, 0x7000c0cae0, 0x70017d37c0, 0x70017790e0, 0x7000c093c3> as *const (), 2), // +0x51d120 sub_51d120
    Lifted::new(0x7000c09470, tpl_68 as *const (), 1), // +0x51d470 sub_51d470
    Lifted::new(0x7000c09520, tpl_68 as *const (), 1), // +0x51d520 sub_51d520
    Lifted::new(0x7000c095d0, tpl_1082::<0x7001779e20, 0x700177c0e0> as *const (), 0), // +0x51d5d0 sub_51d5d0
    Lifted::new(0x7000c09650, fn_7000c09650 as *const (), 0), // +0x51d650 sub_51d650
    Lifted::new(0x7000c09700, fn_7000c09700 as *const (), 0), // +0x51d700 sub_51d700
    Lifted::new(0x7000c09860, tpl_965::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000c099b0, 0x7001d54968, 0x7000c098c0> as *const (), 0), // +0x51d860 sub_51d860
    Lifted::new(0x7000c098e0, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c09983> as *const (), 0), // +0x51d8e0 sub_51d8e0
    Lifted::new(0x7000c099b0, tpl_1083::<0x8, 0x18, 0x700177c312> as *const (), 0), // +0x51d9b0 sub_51d9b0
    Lifted::new(0x7000c09a50, tpl_1084::<0x8, 0x700177c312, 0x10, 0xfffffffffffffff8, 0x7001d54968, 0x7000c09ae3> as *const (), 0), // +0x51da50 sub_51da50
    Lifted::new(0x7000c09b50, tpl_1085::<0x7000c09a50, 0x700177c312> as *const (), 0), // +0x51db50 sub_51db50
    Lifted::new(0x7000c09b90, tpl_1085::<0x7000c09a50, 0x700177c312> as *const (), 0), // +0x51db90 sub_51db90
    Lifted::new(0x7000c09bd0, tpl_33::<0x700177c312> as *const (), 0), // +0x51dbd0 sub_51dbd0
    Lifted::new(0x7000c09c00, tpl_33::<0x700177c312> as *const (), 0), // +0x51dc00 sub_51dc00
    Lifted::new(0x7000c09c30, tpl_33::<0x700177c312> as *const (), 0), // +0x51dc30 sub_51dc30
    Lifted::new(0x7000c09c60, fn_7000c09c60 as *const (), 0), // +0x51dc60 sub_51dc60
    Lifted::new(0x7000c09ca0, tpl_33::<0x700177c312> as *const (), 0), // +0x51dca0 sub_51dca0
    Lifted::new(0x7000c09cd0, tpl_33::<0x700177c312> as *const (), 0), // +0x51dcd0 sub_51dcd0
    Lifted::new(0x7000c09d00, tpl_33::<0x700177c312> as *const (), 0), // +0x51dd00 sub_51dd00
    Lifted::new(0x7000c09d30, tpl_33::<0x700177c312> as *const (), 0), // +0x51dd30 sub_51dd30
    Lifted::new(0x7000c09d60, tpl_33::<0x700177c312> as *const (), 0), // +0x51dd60 sub_51dd60
    Lifted::new(0x7000c09d90, tpl_33::<0x700177c312> as *const (), 0), // +0x51dd90 sub_51dd90
    Lifted::new(0x7000c09dc0, fn_7000c09dc0 as *const (), 0), // +0x51ddc0 sub_51ddc0
    Lifted::new(0x7000c0a000, fn_7000c0a000 as *const (), 0), // +0x51e000 sub_51e000
    Lifted::new(0x7000c0a2c0, fn_7000c0a2c0 as *const (), 2), // +0x51e2c0 sub_51e2c0
    Lifted::new(0x7000c0a380, fn_7000c0a380 as *const (), 0), // +0x51e380 sub_51e380
    Lifted::new(0x7000c0a580, fn_7000c0a580 as *const (), 0), // +0x51e580 sub_51e580
    Lifted::new(0x7000c0a660, fn_7000c0a660 as *const (), 0), // +0x51e660 sub_51e660
    Lifted::new(0x7000c0a720, fn_7000c0a720 as *const (), 0), // +0x51e720 sub_51e720
    Lifted::new(0x7000c0aab0, tpl_47::<0x28> as *const (), 1), // +0x51eab0 sub_51eab0
    Lifted::new(0x7000c0aac0, tpl_144 as *const (), 1), // +0x51eac0 sub_51eac0
    Lifted::new(0x7000c0aad0, fn_7000c0aad0 as *const (), 0), // +0x51ead0 sub_51ead0
    Lifted::new(0x7000c0ac20, tpl_196 as *const (), 1), // +0x51ec20 sub_51ec20
    Lifted::new(0x7000c0ac30, fn_7000c0ac30 as *const (), 0), // +0x51ec30 sub_51ec30
    Lifted::new(0x7000c0ad90, fn_7000c0ad90 as *const (), 0), // +0x51ed90 sub_51ed90
    Lifted::new(0x7000c0aed0, tpl_44 as *const (), 1), // +0x51eed0 sub_51eed0
    Lifted::new(0x7000c0aee0, tpl_44 as *const (), 1), // +0x51eee0 sub_51eee0
    Lifted::new(0x7000c0aef0, tpl_144 as *const (), 1), // +0x51eef0 sub_51eef0
    Lifted::new(0x7000c0af00, fn_7000c0af00 as *const (), 0), // +0x51ef00 sub_51ef00
    Lifted::new(0x7000c0afb0, tpl_36::<0x8> as *const (), 1), // +0x51efb0 sub_51efb0
    Lifted::new(0x7000c0afc0, tpl_196 as *const (), 1), // +0x51efc0 sub_51efc0
    Lifted::new(0x7000c0afd0, fn_7000c0afd0 as *const (), 0), // +0x51efd0 sub_51efd0
    Lifted::new(0x7000c0b020, fn_7000c0b020 as *const (), 0), // +0x51f020 sub_51f020
    Lifted::new(0x7000c0b140, fn_7000c0b140 as *const (), 0), // +0x51f140 sub_51f140
    Lifted::new(0x7000c0b1f0, fn_7000c0b1f0 as *const (), 2), // +0x51f1f0 sub_51f1f0
    Lifted::new(0x7000c0b3e0, fn_7000c0b3e0 as *const (), 2), // +0x51f3e0 sub_51f3e0
    Lifted::new(0x7000c0b960, fn_7000c0b960 as *const (), 1), // +0x51f960 sub_51f960
    Lifted::new(0x7000c0b980, fn_7000c0b980 as *const (), 1), // +0x51f980 sub_51f980
    Lifted::new(0x7000c0bac0, fn_7000c0bac0 as *const (), 0), // +0x51fac0 sub_51fac0
    Lifted::new(0x7000c0bae0, tpl_1086::<0x7001c6abd8, 0x10, 0x8, 0x7000c09dc0, 0xfffffffffffffff8, 0x7001d54968, 0x7000c0bb5e, 0x700177c312> as *const (), 0), // +0x51fae0 sub_51fae0
    Lifted::new(0x7000c0bba0, fn_7000c0bba0 as *const (), 0), // +0x51fba0 sub_51fba0
    Lifted::new(0x7000c0bc00, fn_7000c0bc00 as *const (), 0), // +0x51fc00 sub_51fc00
    Lifted::new(0x7000c0bc20, fn_7000c0bc20 as *const (), 0), // +0x51fc20 sub_51fc20
    Lifted::new(0x7000c0bca0, fn_7000c0bca0 as *const (), 0), // +0x51fca0 sub_51fca0
    Lifted::new(0x7000c0bd10, fn_7000c0bd10 as *const (), 0), // +0x51fd10 sub_51fd10
    Lifted::new(0x7000c0bd60, fn_7000c0bd60 as *const (), 0), // +0x51fd60 sub_51fd60
    Lifted::new(0x7000c0bef0, fn_7000c0bef0 as *const (), 0), // +0x51fef0 sub_51fef0
    Lifted::new(0x7000c0bfd0, fn_7000c0bfd0 as *const (), 0), // +0x51ffd0 sub_51ffd0
    Lifted::new(0x7000c0c090, tpl_15 as *const (), 1), // +0x520090 sub_520090
    Lifted::new(0x7000c0c0a0, fn_7000c0c0a0 as *const (), 0), // +0x5200a0 sub_5200a0
    Lifted::new(0x7000c0c150, tpl_1086::<0x7001c6abd8, 0x10, 0x8, 0x7000c09dc0, 0xfffffffffffffff8, 0x7001d54968, 0x7000c0c1ce, 0x700177c312> as *const (), 0), // +0x520150 sub_520150
    Lifted::new(0x7000c0c210, fn_7000c0c210 as *const (), 0), // +0x520210 sub_520210
    Lifted::new(0x7000c0c2c0, fn_7000c0c2c0 as *const (), 0), // +0x5202c0 sub_5202c0
    Lifted::new(0x7000c0c2e0, tpl_15 as *const (), 1), // +0x5202e0 sub_5202e0
    Lifted::new(0x7000c0c2f0, tpl_15 as *const (), 1), // +0x5202f0 sub_5202f0
    Lifted::new(0x7000c0c300, tpl_36::<0x20> as *const (), 1), // +0x520300 sub_520300
    Lifted::new(0x7000c0c310, tpl_196 as *const (), 1), // +0x520310 sub_520310
    Lifted::new(0x7000c0c320, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c0c427, 0x7000c09470, 0x70007f0910, 0x7000c0c443> as *const (), 0), // +0x520320 sub_520320
    Lifted::new(0x7000c0c490, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c0c597, 0x7000c09520, 0x70007f0910, 0x7000c0c5b3> as *const (), 0), // +0x520490 sub_520490
    Lifted::new(0x7000c0c600, tpl_286::<0x7000c0c490, 0x70017d37a0, 0x70017790e0, 0x7000c0c7eb> as *const (), 0), // +0x520600 sub_520600
    Lifted::new(0x7000c0c870, tpl_286::<0x7000c0c320, 0x70017d37a0, 0x70017790e0, 0x7000c0ca5b> as *const (), 0), // +0x520870 sub_520870
    Lifted::new(0x7000c0cae0, tpl_897::<0x10, 0x38, 0x700177c170, 0x70017d37ec, 0x7000c0c600> as *const (), 2), // +0x520ae0 sub_520ae0
    Lifted::new(0x7000c0cbc0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c0cbd0> as *const (), 0), // +0x520bc0 sub_520bc0
    Lifted::new(0x7000c0cbe0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c0cbf0> as *const (), 0), // +0x520be0 sub_520be0
    Lifted::new(0x7000c0cc00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c0cc10> as *const (), 0), // +0x520c00 sub_520c00
    Lifted::new(0x7000c0cc20, fn_7000c0cc20 as *const (), 0), // +0x520c20 sub_520c20
    Lifted::new(0x7000c0cc60, fn_7000c0cc60 as *const (), 0), // +0x520c60 sub_520c60
    Lifted::new(0x7000c0cce0, tpl_1087::<0x7001779e20, 0xc8, 0x700181bd18, 0x8, 0x70007369b0, 0x700181bce8, 0x700181bd08, 0x20> as *const (), 0), // +0x520ce0 sub_520ce0
    Lifted::new(0x7000c0cd80, tpl_1088::<0xfffffffffffffff8, 0x40, 0x7000c03080, 0x700177c312, 0x38, 0x30> as *const (), 0), // +0x520d80 sub_520d80
    Lifted::new(0x7000c0ce90, tpl_1089::<0x7000c03080, 0x38, 0x30, 0x700177c312> as *const (), 0), // +0x520e90 sub_520e90
    Lifted::new(0x7000c0cf00, tpl_33::<0x700177c312> as *const (), 0), // +0x520f00 sub_520f00
    Lifted::new(0x7000c0cf30, tpl_15 as *const (), 1), // +0x520f30 sub_520f30
    Lifted::new(0x7000c0cf40, tpl_1090::<0x8, 0x7001347d10> as *const (), 0), // +0x520f40 sub_520f40
    Lifted::new(0x7000c0cf50, tpl_1090::<0x8, 0x7001347d10> as *const (), 0), // +0x520f50 sub_520f50
    Lifted::new(0x7000c0cf60, fn_7000c0cf60 as *const (), 0), // +0x520f60 sub_520f60
    Lifted::new(0x7000c0d010, tpl_1091::<0x700181b500, 0x8, 0x10> as *const (), 1), // +0x521010 sub_521010
    Lifted::new(0x7000c0d030, tpl_1091::<0x700181b490, 0x8, 0x10> as *const (), 1), // +0x521030 sub_521030
    Lifted::new(0x7000c0d050, tpl_1091::<0x700181b420, 0x8, 0x10> as *const (), 1), // +0x521050 sub_521050
    Lifted::new(0x7000c0d070, tpl_1092::<0x8, 0x78, 0x2, 0x70, 0x7000c0d500, 0x10, 0x20, 0xe0> as *const (), 2), // +0x521070 sub_521070
    Lifted::new(0x7000c0d120, fn_7000c0d120 as *const (), 0), // +0x521120 sub_521120
    Lifted::new(0x7000c0d140, tpl_1093::<0x8, 0xd8, 0x2, 0x7000c0d450, 0x10> as *const (), 0), // +0x521140 sub_521140
    Lifted::new(0x7000c0d1b0, fn_7000c0d1b0 as *const (), 2), // +0x5211b0 sub_5211b0
    Lifted::new(0x7000c0d290, fn_7000c0d290 as *const (), 2), // +0x521290 sub_521290
    Lifted::new(0x7000c0d450, fn_7000c0d450 as *const (), 0), // +0x521450 sub_521450
    Lifted::new(0x7000c0d500, fn_7000c0d500 as *const (), 0), // +0x521500 sub_521500
    Lifted::new(0x7000c0d5b0, fn_7000c0d5b0 as *const (), 0), // +0x5215b0 sub_5215b0
    Lifted::new(0x7000c0d660, fn_7000c0d660 as *const (), 0), // +0x521660 sub_521660
    Lifted::new(0x7000c0d710, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x521710 sub_521710
    Lifted::new(0x7000c0d770, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x521770 sub_521770
    Lifted::new(0x7000c0d7d0, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x5217d0 sub_5217d0
    Lifted::new(0x7000c0d830, tpl_1045::<0x7001d54e98, 0x8, 0x10, 0x7001d550d8> as *const (), 0), // +0x521830 sub_521830
    Lifted::new(0x7000c0d890, tpl_1094::<0x8, 0x2, 0x7000c0d450, 0x10> as *const (), 0), // +0x521890 sub_521890
    Lifted::new(0x7000c0d8f0, fn_7000c0d8f0 as *const (), 2), // +0x5218f0 sub_5218f0
    Lifted::new(0x7000c0d960, fn_7000c0d960 as *const (), 2), // +0x521960 sub_521960
    Lifted::new(0x7000c0d9d0, tpl_1073::<0x8, 0x2, 0x7000c0d660> as *const (), 2), // +0x5219d0 sub_5219d0
    Lifted::new(0x7000c0dad0, tpl_1095::<0x8, 0x48, 0x2, 0x40, 0x7000c0d660> as *const (), 2), // +0x521ad0 sub_521ad0
    Lifted::new(0x7000c0db30, fn_7000c0db30 as *const (), 0), // +0x521b30 sub_521b30
    Lifted::new(0x7000c0db80, tpl_1096::<0x8, 0x7000c0d1b0> as *const (), 2), // +0x521b80 sub_521b80
    Lifted::new(0x7000c0dbb0, tpl_1097::<0x8, 0xe8, 0xfffffffffffffffc, 0xea, 0xfffffffffffffffe, 0x7000c0d1b0, 0x90, 0x2, 0x88, 0x7000c0d5b0, 0x10, 0x20> as *const (), 2), // +0x521bb0 sub_521bb0
    Lifted::new(0x7000c0dce0, tpl_1096::<0x8, 0x7000c0d1b0> as *const (), 2), // +0x521ce0 sub_521ce0
    Lifted::new(0x7000c0dd10, tpl_1098::<0x8, 0xfffffffffffffffc, 0xfffffffffffffffe, 0x48, 0x2, 0x40, 0x7000c0d660> as *const (), 2), // +0x521d10 sub_521d10
    Lifted::new(0x7000c0dd90, tpl_1099::<0x8, 0x7000c0d290> as *const (), 2), // +0x521d90 sub_521d90
    Lifted::new(0x7000c0ddd0, tpl_1100::<0x700070dae0> as *const (), 0), // +0x521dd0 sub_521dd0
    Lifted::new(0x7000c0de10, tpl_388::<0x7000c0ebe0, 0x700177c312, 0x7001d54968, 0x7000c0df9a, 0x7000c0ebc0, 0x7000c0dfa0> as *const (), 2), // +0x521e10 sub_521e10
    Lifted::new(0x7000c0e010, tpl_388::<0x7000c0ebe0, 0x700177c312, 0x7001d54968, 0x7000c0e19a, 0x7000c0ebc0, 0x7000c0e1a0> as *const (), 2), // +0x522010 sub_522010
    Lifted::new(0x7000c0e210, fn_7000c0e210 as *const (), 2), // +0x522210 sub_522210
    Lifted::new(0x7000c0e320, fn_7000c0e320 as *const (), 0), // +0x522320 sub_522320
    Lifted::new(0x7000c0e3c0, fn_7000c0e3c0 as *const (), 2), // +0x5223c0 sub_5223c0
    Lifted::new(0x7000c0e420, fn_7000c0e420 as *const (), 0), // +0x522420 sub_522420
    Lifted::new(0x7000c0e460, fn_7000c0e460 as *const (), 0), // +0x522460 sub_522460
    Lifted::new(0x7000c0e600, fn_7000c0e600 as *const (), 2), // +0x522600 sub_522600
    Lifted::new(0x7000c0e670, fn_7000c0e670 as *const (), 2), // +0x522670 sub_522670
    Lifted::new(0x7000c0e760, tpl_1101::<0x8, 0x20> as *const (), 0), // +0x522760 sub_522760
    Lifted::new(0x7000c0e790, tpl_1101::<0x8, 0x38> as *const (), 0), // +0x522790 sub_522790
    Lifted::new(0x7000c0e7c0, tpl_1101::<0x8, 0x40> as *const (), 0), // +0x5227c0 sub_5227c0
    Lifted::new(0x7000c0e7f0, fn_7000c0e7f0 as *const (), 0), // +0x5227f0 sub_5227f0
    Lifted::new(0x7000c0e840, tpl_607::<0x8, 0x48> as *const (), 0), // +0x522840 sub_522840
    Lifted::new(0x7000c0e850, fn_7000c0e850 as *const (), 0), // +0x522850 sub_522850
    Lifted::new(0x7000c0e8b0, fn_7000c0e8b0 as *const (), 1), // +0x5228b0 sub_5228b0
    Lifted::new(0x7000c0e8e0, fn_7000c0e8e0 as *const (), 0), // +0x5228e0 sub_5228e0
    Lifted::new(0x7000c0e970, fn_7000c0e970 as *const (), 0), // +0x522970 sub_522970
    Lifted::new(0x7000c0e990, fn_7000c0e990 as *const (), 2), // +0x522990 sub_522990
    Lifted::new(0x7000c0ea20, fn_7000c0ea20 as *const (), 2), // +0x522a20 sub_522a20
    Lifted::new(0x7000c0eab0, fn_7000c0eab0 as *const (), 0), // +0x522ab0 sub_522ab0
    Lifted::new(0x7000c0ead0, fn_7000c0ead0 as *const (), 2), // +0x522ad0 sub_522ad0
    Lifted::new(0x7000c0eb00, tpl_278::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000c0eb8d> as *const (), 0), // +0x522b00 sub_522b00
    Lifted::new(0x7000c0ebc0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c0ebd0> as *const (), 0), // +0x522bc0 sub_522bc0
    Lifted::new(0x7000c0ebe0, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000c0ec2d, 0x70007f0910, 0x7000c0ec4b> as *const (), 0), // +0x522be0 sub_522be0
    Lifted::new(0x7000c0ec70, fn_7000c0ec70 as *const (), 0), // +0x522c70 sub_522c70
    Lifted::new(0x7000c0ed30, fn_7000c0ed30 as *const (), 0), // +0x522d30 sub_522d30
    Lifted::new(0x7000c0eee0, fn_7000c0eee0 as *const (), 0), // +0x522ee0 sub_522ee0
    Lifted::new(0x7000c0f090, tpl_1102::<0xa580, 0x7000f20030> as *const (), 0), // +0x523090 sub_523090
    Lifted::new(0x7000c0f0c0, tpl_1102::<0xa580, 0x7000f20330> as *const (), 0), // +0x5230c0 sub_5230c0
    Lifted::new(0x7000c0f0f0, tpl_1102::<0xa580, 0x7000f206c0> as *const (), 0), // +0x5230f0 sub_5230f0
    Lifted::new(0x7000c0f120, tpl_1103::<0x90, 0xa580, 0x7000f20a60> as *const (), 0), // +0x523120 sub_523120
    Lifted::new(0x7000c0f160, tpl_1102::<0xa580, 0x7000f20db0> as *const (), 0), // +0x523160 sub_523160
    Lifted::new(0x7000c0f190, tpl_1103::<0x90, 0xa580, 0x7000f21160> as *const (), 0), // +0x523190 sub_523190
    Lifted::new(0x7000c0f1d0, fn_7000c0f1d0 as *const (), 0), // +0x5231d0 sub_5231d0
    Lifted::new(0x7000c0f290, fn_7000c0f290 as *const (), 0), // +0x523290 sub_523290
    Lifted::new(0x7000c0f360, fn_7000c0f360 as *const (), 0), // +0x523360 sub_523360
    Lifted::new(0x7000c0f420, fn_7000c0f420 as *const (), 0), // +0x523420 sub_523420
    Lifted::new(0x7000c0f4e0, tpl_1104::<0x7000732160> as *const (), 0), // +0x5234e0 sub_5234e0
    Lifted::new(0x7000c0f510, tpl_1104::<0x7000750af0> as *const (), 0), // +0x523510 sub_523510
    Lifted::new(0x7000c0f540, fn_7000c0f540 as *const (), 0), // +0x523540 sub_523540
    Lifted::new(0x7000c0f610, fn_7000c0f610 as *const (), 0), // +0x523610 sub_523610
    Lifted::new(0x7000c0f6e0, tpl_1105::<0x8, 0x700072b4b0, 0x700181bde8, 0x7001d54ef8, 0x7000c0f718> as *const (), 0), // +0x5236e0 sub_5236e0
    Lifted::new(0x7000c0f730, tpl_1105::<0x8, 0x70006f50e0, 0x700181bde8, 0x7001d54ef8, 0x7000c0f768> as *const (), 0), // +0x523730 sub_523730
    Lifted::new(0x7000c0f780, fn_7000c0f780 as *const (), 0), // +0x523780 sub_523780
    Lifted::new(0x7000c0f8b0, fn_7000c0f8b0 as *const (), 0), // +0x5238b0 sub_5238b0
    Lifted::new(0x7000c0f9d0, fn_7000c0f9d0 as *const (), 0), // +0x5239d0 sub_5239d0
    Lifted::new(0x7000c0faf0, fn_7000c0faf0 as *const (), 0), // +0x523af0 sub_523af0
    Lifted::new(0x7000c0fc10, tpl_1106::<0xc, 0x700072b4b0, 0x700181bde8, 0x7001d54ef8, 0x7000c0fc48> as *const (), 0), // +0x523c10 sub_523c10
    Lifted::new(0x7000c0fc60, tpl_1106::<0xc, 0x70006f50e0, 0x700181bde8, 0x7001d54ef8, 0x7000c0fc98> as *const (), 0), // +0x523c60 sub_523c60
    Lifted::new(0x7000c0fcb0, tpl_1107::<0x7000732160, 0xffffffffffffffff> as *const (), 0), // +0x523cb0 sub_523cb0
    Lifted::new(0x7000c0fcf0, tpl_1107::<0x7000750af0, 0xffffffffffffffff> as *const (), 0), // +0x523cf0 sub_523cf0
    Lifted::new(0x7000c0fd30, tpl_1108::<0x4b0, 0x7, 0x8, 0x7000a3be30, 0xa580, 0x70017d91e0, 0x7000f206c0> as *const (), 2), // +0x523d30 sub_523d30
    Lifted::new(0x7000c0fde0, tpl_1108::<0x4b0, 0x7, 0x8, 0x7000a3c050, 0xa580, 0x70017d91f0, 0x7000f20db0> as *const (), 2), // +0x523de0 sub_523de0
    Lifted::new(0x7000c0fe90, tpl_1109::<0x430, 0x8, 0x7, 0x7000a3be30, 0x90, 0x70017d91e0, 0xa580, 0x7000f20a60> as *const (), 2), // +0x523e90 sub_523e90
    Lifted::new(0x7000c0ff50, tpl_1109::<0x430, 0x8, 0x7, 0x7000a3c050, 0x90, 0x70017d91f0, 0xa580, 0x7000f21160> as *const (), 2), // +0x523f50 sub_523f50
    Lifted::new(0x7000c10010, fn_7000c10010 as *const (), 2), // +0x524010 sub_524010
    Lifted::new(0x7000c10170, fn_7000c10170 as *const (), 2), // +0x524170 sub_524170
    Lifted::new(0x7000c102e0, fn_7000c102e0 as *const (), 0), // +0x5242e0 sub_5242e0
    Lifted::new(0x7000c103e0, fn_7000c103e0 as *const (), 0), // +0x5243e0 sub_5243e0
    Lifted::new(0x7000c104e0, fn_7000c104e0 as *const (), 0), // +0x5244e0 sub_5244e0
    Lifted::new(0x7000c10660, fn_7000c10660 as *const (), 0), // +0x524660 sub_524660
    Lifted::new(0x7000c107d0, fn_7000c107d0 as *const (), 0), // +0x5247d0 sub_5247d0
    Lifted::new(0x7000c108b0, fn_7000c108b0 as *const (), 0), // +0x5248b0 sub_5248b0
    Lifted::new(0x7000c109b0, fn_7000c109b0 as *const (), 0), // +0x5249b0 sub_5249b0
    Lifted::new(0x7000c10b20, fn_7000c10b20 as *const (), 0), // +0x524b20 sub_524b20
    Lifted::new(0x7000c10c70, fn_7000c10c70 as *const (), 2), // +0x524c70 sub_524c70
    Lifted::new(0x7000c10f10, fn_7000c10f10 as *const (), 2), // +0x524f10 sub_524f10
    Lifted::new(0x7000c111b0, fn_7000c111b0 as *const (), 0), // +0x5251b0 sub_5251b0
    Lifted::new(0x7000c112c0, fn_7000c112c0 as *const (), 0), // +0x5252c0 sub_5252c0
    Lifted::new(0x7000c113d0, fn_7000c113d0 as *const (), 2), // +0x5253d0 sub_5253d0
    Lifted::new(0x7000c11650, fn_7000c11650 as *const (), 0), // +0x525650 sub_525650
    Lifted::new(0x7000c11b80, fn_7000c11b80 as *const (), 2), // +0x525b80 sub_525b80
    Lifted::new(0x7000c11e00, fn_7000c11e00 as *const (), 0), // +0x525e00 sub_525e00
    Lifted::new(0x7000c123e0, fn_7000c123e0 as *const (), 0), // +0x5263e0 sub_5263e0
    Lifted::new(0x7000c12510, fn_7000c12510 as *const (), 0), // +0x526510 sub_526510
    Lifted::new(0x7000c12640, fn_7000c12640 as *const (), 0), // +0x526640 sub_526640
    Lifted::new(0x7000c12750, fn_7000c12750 as *const (), 0), // +0x526750 sub_526750
    Lifted::new(0x7000c12870, fn_7000c12870 as *const (), 0), // +0x526870 sub_526870
    Lifted::new(0x7000c129a0, fn_7000c129a0 as *const (), 0), // +0x5269a0 sub_5269a0
    Lifted::new(0x7000c12ad0, fn_7000c12ad0 as *const (), 0), // +0x526ad0 sub_526ad0
    Lifted::new(0x7000c12b50, fn_7000c12b50 as *const (), 0), // +0x526b50 sub_526b50
    Lifted::new(0x7000c12bd0, fn_7000c12bd0 as *const (), 0), // +0x526bd0 sub_526bd0
    Lifted::new(0x7000c12cf0, fn_7000c12cf0 as *const (), 0), // +0x526cf0 sub_526cf0
    Lifted::new(0x7000c12e10, fn_7000c12e10 as *const (), 0), // +0x526e10 sub_526e10
    Lifted::new(0x7000c12e80, fn_7000c12e80 as *const (), 0), // +0x526e80 sub_526e80
    Lifted::new(0x7000c12ee0, fn_7000c12ee0 as *const (), 0), // +0x526ee0 sub_526ee0
    Lifted::new(0x7000c12f20, fn_7000c12f20 as *const (), 0), // +0x526f20 sub_526f20
    Lifted::new(0x7000c12f60, fn_7000c12f60 as *const (), 0), // +0x526f60 sub_526f60
    Lifted::new(0x7000c130b0, fn_7000c130b0 as *const (), 0), // +0x5270b0 sub_5270b0
    Lifted::new(0x7000c13210, fn_7000c13210 as *const (), 2), // +0x527210 sub_527210
    Lifted::new(0x7000c13310, fn_7000c13310 as *const (), 2), // +0x527310 sub_527310
    Lifted::new(0x7000c13410, fn_7000c13410 as *const (), 0), // +0x527410 sub_527410
    Lifted::new(0x7000c13510, fn_7000c13510 as *const (), 0), // +0x527510 sub_527510
    Lifted::new(0x7000c13610, tpl_1110::<0x8, 0x70006fd030, 0x70009fc7f0, 0x7001d550a8> as *const (), 2), // +0x527610 sub_527610
    Lifted::new(0x7000c13690, tpl_1110::<0x8, 0x700079b4e0, 0x70009fc7f0, 0x7001d550a8> as *const (), 2), // +0x527690 sub_527690
    Lifted::new(0x7000c13710, fn_7000c13710 as *const (), 0), // +0x527710 sub_527710
    Lifted::new(0x7000c13850, fn_7000c13850 as *const (), 0), // +0x527850 sub_527850
    Lifted::new(0x7000c139a0, fn_7000c139a0 as *const (), 0), // +0x5279a0 sub_5279a0
    Lifted::new(0x7000c13af0, fn_7000c13af0 as *const (), 0), // +0x527af0 sub_527af0
    Lifted::new(0x7000c13c00, fn_7000c13c00 as *const (), 0), // +0x527c00 sub_527c00
    Lifted::new(0x7000c13d50, fn_7000c13d50 as *const (), 0), // +0x527d50 sub_527d50
    Lifted::new(0x7000c13e70, fn_7000c13e70 as *const (), 2), // +0x527e70 sub_527e70
    Lifted::new(0x7000c140e0, fn_7000c140e0 as *const (), 2), // +0x5280e0 sub_5280e0
    Lifted::new(0x7000c14340, fn_7000c14340 as *const (), 0), // +0x528340 sub_528340
    Lifted::new(0x7000c14420, fn_7000c14420 as *const (), 0), // +0x528420 sub_528420
    Lifted::new(0x7000c14510, tpl_1111::<0x70007a46e0, 0x700078f140> as *const (), 0), // +0x528510 sub_528510
    Lifted::new(0x7000c14570, tpl_1111::<0x7000736000, 0x700070cda0> as *const (), 0), // +0x528570 sub_528570
    Lifted::new(0x7000c145d0, fn_7000c145d0 as *const (), 2), // +0x5285d0 sub_5285d0
    Lifted::new(0x7000c14850, fn_7000c14850 as *const (), 2), // +0x528850 sub_528850
    Lifted::new(0x7000c14ac0, tpl_1112::<0x7000788bc0, 0x7000767d50, 0x700074a010, 0x7000a33e20> as *const (), 2), // +0x528ac0 sub_528ac0
    Lifted::new(0x7000c14db0, fn_7000c14db0 as *const (), 0), // +0x528db0 sub_528db0
    Lifted::new(0x7000c14ea0, tpl_1112::<0x7000723d70, 0x7000767d50, 0x700074a010, 0x7000a33e20> as *const (), 2), // +0x528ea0 sub_528ea0
    Lifted::new(0x7000c15190, fn_7000c15190 as *const (), 0), // +0x529190 sub_529190
    Lifted::new(0x7000c15270, fn_7000c15270 as *const (), 0), // +0x529270 sub_529270
    Lifted::new(0x7000c15360, fn_7000c15360 as *const (), 0), // +0x529360 sub_529360
    Lifted::new(0x7000c15440, fn_7000c15440 as *const (), 0), // +0x529440 sub_529440
    Lifted::new(0x7000c15520, fn_7000c15520 as *const (), 0), // +0x529520 sub_529520
    Lifted::new(0x7000c15610, fn_7000c15610 as *const (), 0), // +0x529610 sub_529610
    Lifted::new(0x7000c15720, fn_7000c15720 as *const (), 0), // +0x529720 sub_529720
    Lifted::new(0x7000c15820, tpl_1113::<0x7001d55098, 0x70007345d0> as *const (), 0), // +0x529820 sub_529820
    Lifted::new(0x7000c158b0, tpl_1113::<0x7001d55098, 0x70007522d0> as *const (), 0), // +0x5298b0 sub_5298b0
    Lifted::new(0x7000c15940, fn_7000c15940 as *const (), 0), // +0x529940 sub_529940
    Lifted::new(0x7000c15ad0, fn_7000c15ad0 as *const (), 0), // +0x529ad0 sub_529ad0
    Lifted::new(0x7000c15c60, fn_7000c15c60 as *const (), 2), // +0x529c60 sub_529c60
    Lifted::new(0x7000c15e90, fn_7000c15e90 as *const (), 2), // +0x529e90 sub_529e90
    Lifted::new(0x7000c160c0, tpl_1114::<0x8, 0x7000769ef0, 0x70009fc7f0, 0x7001d550a8> as *const (), 2), // +0x52a0c0 sub_52a0c0
    Lifted::new(0x7000c16140, tpl_1114::<0x8, 0x70006f4580, 0x70009fc7f0, 0x7001d550a8> as *const (), 2), // +0x52a140 sub_52a140
    Lifted::new(0x7000c161c0, tpl_1115::<0x7000732450> as *const (), 0), // +0x52a1c0 sub_52a1c0
    Lifted::new(0x7000c16200, tpl_1115::<0x700074f0f0> as *const (), 0), // +0x52a200 sub_52a200
    Lifted::new(0x7000c16240, tpl_1116::<0x700077d6a0> as *const (), 0), // +0x52a240 sub_52a240
    Lifted::new(0x7000c16270, tpl_1116::<0x70007084d0> as *const (), 0), // +0x52a270 sub_52a270
    Lifted::new(0x7000c162a0, fn_7000c162a0 as *const (), 0), // +0x52a2a0 sub_52a2a0
    Lifted::new(0x7000c162f0, fn_7000c162f0 as *const (), 0), // +0x52a2f0 sub_52a2f0
    Lifted::new(0x7000c16350, fn_7000c16350 as *const (), 0), // +0x52a350 sub_52a350
    Lifted::new(0x7000c164c0, fn_7000c164c0 as *const (), 0), // +0x52a4c0 sub_52a4c0
    Lifted::new(0x7000c16630, tpl_1117::<0x3, 0x7000732160, 0xffffffffffffffff> as *const (), 0), // +0x52a630 sub_52a630
    Lifted::new(0x7000c16680, tpl_1117::<0x3, 0x7000750af0, 0xffffffffffffffff> as *const (), 0), // +0x52a680 sub_52a680
    Lifted::new(0x7000c166d0, fn_7000c166d0 as *const (), 0), // +0x52a6d0 sub_52a6d0
    Lifted::new(0x7000c167e0, fn_7000c167e0 as *const (), 0), // +0x52a7e0 sub_52a7e0
    Lifted::new(0x7000c168f0, fn_7000c168f0 as *const (), 0), // +0x52a8f0 sub_52a8f0
    Lifted::new(0x7000c16a10, fn_7000c16a10 as *const (), 0), // +0x52aa10 sub_52aa10
    Lifted::new(0x7000c16b30, fn_7000c16b30 as *const (), 0), // +0x52ab30 sub_52ab30
    Lifted::new(0x7000c16c50, fn_7000c16c50 as *const (), 0), // +0x52ac50 sub_52ac50
    Lifted::new(0x7000c16d70, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x52ad70 sub_52ad70
    Lifted::new(0x7000c16e80, tpl_1118::<0x7000768d60, 0x7000c399e0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x52ae80 sub_52ae80
    Lifted::new(0x7000c16ff0, fn_7000c16ff0 as *const (), 0), // +0x52aff0 sub_52aff0
    Lifted::new(0x7000c17160, tpl_1118::<0x7000768d60, 0x7000c39910, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x52b160 sub_52b160
    Lifted::new(0x7000c172d0, fn_7000c172d0 as *const (), 0), // +0x52b2d0 sub_52b2d0
    Lifted::new(0x7000c17420, fn_7000c17420 as *const (), 0), // +0x52b420 sub_52b420
    Lifted::new(0x7000c17560, fn_7000c17560 as *const (), 0), // +0x52b560 sub_52b560
    Lifted::new(0x7000c17ab0, tpl_1119::<0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000c17b37, 0x7000c45270, 0x7000767d50, 0x7000c2e500, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x52bab0 sub_52bab0
    Lifted::new(0x7000c17cb0, tpl_1119::<0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000c17d37, 0x7000c45270, 0x7000767d50, 0x7000c2e500, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x52bcb0 sub_52bcb0
    Lifted::new(0x7000c17eb0, fn_7000c17eb0 as *const (), 0), // +0x52beb0 sub_52beb0
    Lifted::new(0x7000c17f50, fn_7000c17f50 as *const (), 0), // +0x52bf50 sub_52bf50
    Lifted::new(0x7000c18010, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c181e4, 0x7000bcd9c0, 0x7000c181ea, 0x70007f0910, 0x7000c181f0> as *const (), 0), // +0x52c010 sub_52c010
    Lifted::new(0x7000c18270, fn_7000c18270 as *const (), 0), // +0x52c270 sub_52c270
    Lifted::new(0x7000c182e0, fn_7000c182e0 as *const (), 0), // +0x52c2e0 sub_52c2e0
    Lifted::new(0x7000c18360, fn_7000c18360 as *const (), 1), // +0x52c360 sub_52c360
    Lifted::new(0x7000c18390, tpl_33::<0x700177c312> as *const (), 0), // +0x52c390 sub_52c390
    Lifted::new(0x7000c183c0, tpl_33::<0x700177c312> as *const (), 0), // +0x52c3c0 sub_52c3c0
    Lifted::new(0x7000c183f0, fn_7000c183f0 as *const (), 0), // +0x52c3f0 sub_52c3f0
    Lifted::new(0x7000c18450, fn_7000c18450 as *const (), 0), // +0x52c450 sub_52c450
    Lifted::new(0x7000c18580, fn_7000c18580 as *const (), 0), // +0x52c580 sub_52c580
    Lifted::new(0x7000c186d0, fn_7000c186d0 as *const (), 0), // +0x52c6d0 sub_52c6d0
    Lifted::new(0x7000c18710, fn_7000c18710 as *const (), 0), // +0x52c710 sub_52c710
    Lifted::new(0x7000c18be0, fn_7000c18be0 as *const (), 2), // +0x52cbe0 sub_52cbe0
    Lifted::new(0x7000c18dd0, fn_7000c18dd0 as *const (), 2), // +0x52cdd0 sub_52cdd0
    Lifted::new(0x7000c18fb0, fn_7000c18fb0 as *const (), 2), // +0x52cfb0 sub_52cfb0
    Lifted::new(0x7000c191d0, fn_7000c191d0 as *const (), 0), // +0x52d1d0 sub_52d1d0
    Lifted::new(0x7000c19220, fn_7000c19220 as *const (), 0), // +0x52d220 sub_52d220
    Lifted::new(0x7000c19340, fn_7000c19340 as *const (), 2), // +0x52d340 sub_52d340
    Lifted::new(0x7000c19c50, fn_7000c19c50 as *const (), 2), // +0x52dc50 sub_52dc50
    Lifted::new(0x7000c1a0d0, fn_7000c1a0d0 as *const (), 1), // +0x52e0d0 sub_52e0d0
    Lifted::new(0x7000c1a0f0, fn_7000c1a0f0 as *const (), 2), // +0x52e0f0 sub_52e0f0
    Lifted::new(0x7000c1a5b0, fn_7000c1a5b0 as *const (), 2), // +0x52e5b0 sub_52e5b0
    Lifted::new(0x7000c1a820, fn_7000c1a820 as *const (), 2), // +0x52e820 sub_52e820
    Lifted::new(0x7000c1abc0, fn_7000c1abc0 as *const (), 2), // +0x52ebc0 sub_52ebc0
    Lifted::new(0x7000c1afb0, fn_7000c1afb0 as *const (), 0), // +0x52efb0 sub_52efb0
    Lifted::new(0x7000c1b0a0, fn_7000c1b0a0 as *const (), 0), // +0x52f0a0 sub_52f0a0
    Lifted::new(0x7000c1b0d0, fn_7000c1b0d0 as *const (), 0), // +0x52f0d0 sub_52f0d0
    Lifted::new(0x7000c1b230, tpl_216::<0x7000c172d0> as *const (), 0), // +0x52f230 sub_52f230
    Lifted::new(0x7000c1b250, fn_7000c1b250 as *const (), 0), // +0x52f250 sub_52f250
    Lifted::new(0x7000c1b340, tpl_264::<0x700074a830> as *const (), 0), // +0x52f340 sub_52f340
    Lifted::new(0x7000c1b360, fn_7000c1b360 as *const (), 2), // +0x52f360 sub_52f360
    Lifted::new(0x7000c1bc40, fn_7000c1bc40 as *const (), 0), // +0x52fc40 sub_52fc40
    Lifted::new(0x7000c1bdb0, fn_7000c1bdb0 as *const (), 0), // +0x52fdb0 sub_52fdb0
    Lifted::new(0x7000c1bed0, fn_7000c1bed0 as *const (), 0), // +0x52fed0 sub_52fed0
    Lifted::new(0x7000c1c040, fn_7000c1c040 as *const (), 0), // +0x530040 sub_530040
    Lifted::new(0x7000c1c140, fn_7000c1c140 as *const (), 0), // +0x530140 sub_530140
    Lifted::new(0x7000c1c380, fn_7000c1c380 as *const (), 0), // +0x530380 sub_530380
    Lifted::new(0x7000c1c680, fn_7000c1c680 as *const (), 0), // +0x530680 sub_530680
    Lifted::new(0x7000c1c920, tpl_172::<0xc8> as *const (), 1), // +0x530920 sub_530920
    Lifted::new(0x7000c1c930, fn_7000c1c930 as *const (), 0), // +0x530930 sub_530930
    Lifted::new(0x7000c1cb80, fn_7000c1cb80 as *const (), 0), // +0x530b80 sub_530b80
    Lifted::new(0x7000c1cca0, fn_7000c1cca0 as *const (), 1), // +0x530ca0 sub_530ca0
    Lifted::new(0x7000c1cd40, fn_7000c1cd40 as *const (), 0), // +0x530d40 sub_530d40
    Lifted::new(0x7000c1d000, fn_7000c1d000 as *const (), 0), // +0x531000 sub_531000
    Lifted::new(0x7000c1d100, fn_7000c1d100 as *const (), 2), // +0x531100 sub_531100
    Lifted::new(0x7000c1d570, fn_7000c1d570 as *const (), 1), // +0x531570 sub_531570
    Lifted::new(0x7000c1d5a0, fn_7000c1d5a0 as *const (), 0), // +0x5315a0 sub_5315a0
    Lifted::new(0x7000c1d7a0, fn_7000c1d7a0 as *const (), 0), // +0x5317a0 sub_5317a0
    Lifted::new(0x7000c1d880, fn_7000c1d880 as *const (), 2), // +0x531880 sub_531880
    Lifted::new(0x7000c1dca0, fn_7000c1dca0 as *const (), 2), // +0x531ca0 sub_531ca0
    Lifted::new(0x7000c1de10, fn_7000c1de10 as *const (), 0), // +0x531e10 sub_531e10
    Lifted::new(0x7000c1de60, fn_7000c1de60 as *const (), 0), // +0x531e60 sub_531e60
    Lifted::new(0x7000c1dff0, fn_7000c1dff0 as *const (), 0), // +0x531ff0 sub_531ff0
    Lifted::new(0x7000c1e140, fn_7000c1e140 as *const (), 0), // +0x532140 sub_532140
    Lifted::new(0x7000c1e270, fn_7000c1e270 as *const (), 0), // +0x532270 sub_532270
    Lifted::new(0x7000c1e370, fn_7000c1e370 as *const (), 2), // +0x532370 sub_532370
    Lifted::new(0x7000c1e5d0, fn_7000c1e5d0 as *const (), 0), // +0x5325d0 sub_5325d0
    Lifted::new(0x7000c1e890, fn_7000c1e890 as *const (), 2), // +0x532890 sub_532890
    Lifted::new(0x7000c1ea70, fn_7000c1ea70 as *const (), 0), // +0x532a70 sub_532a70
    Lifted::new(0x7000c1eb30, fn_7000c1eb30 as *const (), 0), // +0x532b30 sub_532b30
    Lifted::new(0x7000c1ecc0, fn_7000c1ecc0 as *const (), 0), // +0x532cc0 sub_532cc0
    Lifted::new(0x7000c1ee40, fn_7000c1ee40 as *const (), 0), // +0x532e40 sub_532e40
    Lifted::new(0x7000c1ef50, fn_7000c1ef50 as *const (), 0), // +0x532f50 sub_532f50
    Lifted::new(0x7000c1f0c0, fn_7000c1f0c0 as *const (), 2), // +0x5330c0 sub_5330c0
    Lifted::new(0x7000c1f220, fn_7000c1f220 as *const (), 0), // +0x533220 sub_533220
    Lifted::new(0x7000c1f320, fn_7000c1f320 as *const (), 0), // +0x533320 sub_533320
    Lifted::new(0x7000c1f420, fn_7000c1f420 as *const (), 0), // +0x533420 sub_533420
    Lifted::new(0x7000c1f590, fn_7000c1f590 as *const (), 0), // +0x533590 sub_533590
    Lifted::new(0x7000c1f6f0, fn_7000c1f6f0 as *const (), 0), // +0x5336f0 sub_5336f0
    Lifted::new(0x7000c1f750, fn_7000c1f750 as *const (), 0), // +0x533750 sub_533750
    Lifted::new(0x7000c1f810, fn_7000c1f810 as *const (), 2), // +0x533810 sub_533810
    Lifted::new(0x7000c1fa00, fn_7000c1fa00 as *const (), 0), // +0x533a00 sub_533a00
    Lifted::new(0x7000c1fba0, fn_7000c1fba0 as *const (), 2), // +0x533ba0 sub_533ba0
    Lifted::new(0x7000c1fe90, fn_7000c1fe90 as *const (), 0), // +0x533e90 sub_533e90
    Lifted::new(0x7000c1ffa0, fn_7000c1ffa0 as *const (), 2), // +0x533fa0 sub_533fa0
    Lifted::new(0x7000c20350, fn_7000c20350 as *const (), 2), // +0x534350 sub_534350
    Lifted::new(0x7000c20580, fn_7000c20580 as *const (), 0), // +0x534580 sub_534580
    Lifted::new(0x7000c20600, fn_7000c20600 as *const (), 0), // +0x534600 sub_534600
    Lifted::new(0x7000c207a0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x5347a0 sub_5347a0
    Lifted::new(0x7000c208c0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x373, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x5348c0 sub_5348c0
    Lifted::new(0x7000c209e0, fn_7000c209e0 as *const (), 2), // +0x5349e0 sub_5349e0
    Lifted::new(0x7000c20be0, fn_7000c20be0 as *const (), 0), // +0x534be0 sub_534be0
    Lifted::new(0x7000c20ed0, fn_7000c20ed0 as *const (), 0), // +0x534ed0 sub_534ed0
    Lifted::new(0x7000c211c0, fn_7000c211c0 as *const (), 2), // +0x5351c0 sub_5351c0
    Lifted::new(0x7000c21510, fn_7000c21510 as *const (), 0), // +0x535510 sub_535510
    Lifted::new(0x7000c216d0, fn_7000c216d0 as *const (), 0), // +0x5356d0 sub_5356d0
    Lifted::new(0x7000c21730, fn_7000c21730 as *const (), 0), // +0x535730 sub_535730
    Lifted::new(0x7000c21800, fn_7000c21800 as *const (), 2), // +0x535800 sub_535800
    Lifted::new(0x7000c21a80, fn_7000c21a80 as *const (), 2), // +0x535a80 sub_535a80
    Lifted::new(0x7000c21bc0, fn_7000c21bc0 as *const (), 0), // +0x535bc0 sub_535bc0
    Lifted::new(0x7000c21e70, fn_7000c21e70 as *const (), 2), // +0x535e70 sub_535e70
    Lifted::new(0x7000c21f20, fn_7000c21f20 as *const (), 0), // +0x535f20 sub_535f20
    Lifted::new(0x7000c220c0, fn_7000c220c0 as *const (), 0), // +0x5360c0 sub_5360c0
    Lifted::new(0x7000c22160, fn_7000c22160 as *const (), 0), // +0x536160 sub_536160
    Lifted::new(0x7000c22340, fn_7000c22340 as *const (), 0), // +0x536340 sub_536340
    Lifted::new(0x7000c225a0, fn_7000c225a0 as *const (), 2), // +0x5365a0 sub_5365a0
    Lifted::new(0x7000c22890, fn_7000c22890 as *const (), 0), // +0x536890 sub_536890
    Lifted::new(0x7000c22b00, fn_7000c22b00 as *const (), 0), // +0x536b00 sub_536b00
    Lifted::new(0x7000c22be0, fn_7000c22be0 as *const (), 2), // +0x536be0 sub_536be0
    Lifted::new(0x7000c22e20, fn_7000c22e20 as *const (), 0), // +0x536e20 sub_536e20
    Lifted::new(0x7000c22e60, fn_7000c22e60 as *const (), 2), // +0x536e60 sub_536e60
    Lifted::new(0x7000c230c0, fn_7000c230c0 as *const (), 0), // +0x5370c0 sub_5370c0
    Lifted::new(0x7000c23360, fn_7000c23360 as *const (), 2), // +0x537360 sub_537360
    Lifted::new(0x7000c23830, fn_7000c23830 as *const (), 0), // +0x537830 sub_537830
    Lifted::new(0x7000c239d0, fn_7000c239d0 as *const (), 0), // +0x5379d0 sub_5379d0
    Lifted::new(0x7000c23bd0, fn_7000c23bd0 as *const (), 2), // +0x537bd0 sub_537bd0
    Lifted::new(0x7000c24190, fn_7000c24190 as *const (), 0), // +0x538190 sub_538190
    Lifted::new(0x7000c24440, fn_7000c24440 as *const (), 0), // +0x538440 sub_538440
    Lifted::new(0x7000c246b0, fn_7000c246b0 as *const (), 0), // +0x5386b0 sub_5386b0
    Lifted::new(0x7000c24920, fn_7000c24920 as *const (), 0), // +0x538920 sub_538920
    Lifted::new(0x7000c24c80, fn_7000c24c80 as *const (), 0), // +0x538c80 sub_538c80
    Lifted::new(0x7000c24d60, fn_7000c24d60 as *const (), 0), // +0x538d60 sub_538d60
    Lifted::new(0x7000c24e30, fn_7000c24e30 as *const (), 0), // +0x538e30 sub_538e30
    Lifted::new(0x7000c24fd0, fn_7000c24fd0 as *const (), 0), // +0x538fd0 sub_538fd0
    Lifted::new(0x7000c25020, fn_7000c25020 as *const (), 0), // +0x539020 sub_539020
    Lifted::new(0x7000c252d0, fn_7000c252d0 as *const (), 0), // +0x5392d0 sub_5392d0
    Lifted::new(0x7000c255a0, fn_7000c255a0 as *const (), 0), // +0x5395a0 sub_5395a0
    Lifted::new(0x7000c25810, fn_7000c25810 as *const (), 0), // +0x539810 sub_539810
    Lifted::new(0x7000c25b00, fn_7000c25b00 as *const (), 2), // +0x539b00 sub_539b00
    Lifted::new(0x7000c25e40, fn_7000c25e40 as *const (), 0), // +0x539e40 sub_539e40
    Lifted::new(0x7000c26340, fn_7000c26340 as *const (), 0), // +0x53a340 sub_53a340
    Lifted::new(0x7000c26610, fn_7000c26610 as *const (), 0), // +0x53a610 sub_53a610
    Lifted::new(0x7000c266f0, fn_7000c266f0 as *const (), 0), // +0x53a6f0 sub_53a6f0
    Lifted::new(0x7000c26850, fn_7000c26850 as *const (), 0), // +0x53a850 sub_53a850
    Lifted::new(0x7000c26a00, fn_7000c26a00 as *const (), 2), // +0x53aa00 sub_53aa00
    Lifted::new(0x7000c26d40, fn_7000c26d40 as *const (), 2), // +0x53ad40 sub_53ad40
    Lifted::new(0x7000c270b0, fn_7000c270b0 as *const (), 0), // +0x53b0b0 sub_53b0b0
    Lifted::new(0x7000c270f0, fn_7000c270f0 as *const (), 2), // +0x53b0f0 sub_53b0f0
    Lifted::new(0x7000c27780, fn_7000c27780 as *const (), 0), // +0x53b780 sub_53b780
    Lifted::new(0x7000c278e0, fn_7000c278e0 as *const (), 0), // +0x53b8e0 sub_53b8e0
    Lifted::new(0x7000c27a30, fn_7000c27a30 as *const (), 0), // +0x53ba30 sub_53ba30
    Lifted::new(0x7000c27d20, fn_7000c27d20 as *const (), 0), // +0x53bd20 sub_53bd20
    Lifted::new(0x7000c27e80, tpl_1121::<0x4, 0x700075b0a0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000c27eb9> as *const (), 0), // +0x53be80 sub_53be80
    Lifted::new(0x7000c27ee0, tpl_1121::<0x4, 0x70007a26e0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000c27f19> as *const (), 0), // +0x53bee0 sub_53bee0
    Lifted::new(0x7000c27f40, fn_7000c27f40 as *const (), 0), // +0x53bf40 sub_53bf40
    Lifted::new(0x7000c27fa0, fn_7000c27fa0 as *const (), 0), // +0x53bfa0 sub_53bfa0
    Lifted::new(0x7000c28110, fn_7000c28110 as *const (), 0), // +0x53c110 sub_53c110
    Lifted::new(0x7000c28200, fn_7000c28200 as *const (), 0), // +0x53c200 sub_53c200
    Lifted::new(0x7000c28330, fn_7000c28330 as *const (), 2), // +0x53c330 sub_53c330
    Lifted::new(0x7000c28890, fn_7000c28890 as *const (), 0), // +0x53c890 sub_53c890
    Lifted::new(0x7000c289a0, fn_7000c289a0 as *const (), 2), // +0x53c9a0 sub_53c9a0
    Lifted::new(0x7000c28bf0, fn_7000c28bf0 as *const (), 0), // +0x53cbf0 sub_53cbf0
    Lifted::new(0x7000c28cf0, fn_7000c28cf0 as *const (), 0), // +0x53ccf0 sub_53ccf0
    Lifted::new(0x7000c28da0, fn_7000c28da0 as *const (), 0), // +0x53cda0 sub_53cda0
    Lifted::new(0x7000c28ec0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x53cec0 sub_53cec0
    Lifted::new(0x7000c28fe0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x53cfe0 sub_53cfe0
    Lifted::new(0x7000c29100, fn_7000c29100 as *const (), 0), // +0x53d100 sub_53d100
    Lifted::new(0x7000c291b0, fn_7000c291b0 as *const (), 2), // +0x53d1b0 sub_53d1b0
    Lifted::new(0x7000c29440, fn_7000c29440 as *const (), 0), // +0x53d440 sub_53d440
    Lifted::new(0x7000c29490, fn_7000c29490 as *const (), 0), // +0x53d490 sub_53d490
    Lifted::new(0x7000c29550, fn_7000c29550 as *const (), 0), // +0x53d550 sub_53d550
    Lifted::new(0x7000c29590, fn_7000c29590 as *const (), 0), // +0x53d590 sub_53d590
    Lifted::new(0x7000c295d0, fn_7000c295d0 as *const (), 0), // +0x53d5d0 sub_53d5d0
    Lifted::new(0x7000c29610, tpl_1122::<0x7000763c30> as *const (), 0), // +0x53d610 sub_53d610
    Lifted::new(0x7000c29640, tpl_1122::<0x7000763c30> as *const (), 0), // +0x53d640 sub_53d640
    Lifted::new(0x7000c29670, fn_7000c29670 as *const (), 0), // +0x53d670 sub_53d670
    Lifted::new(0x7000c29790, fn_7000c29790 as *const (), 2), // +0x53d790 sub_53d790
    Lifted::new(0x7000c29a30, fn_7000c29a30 as *const (), 0), // +0x53da30 sub_53da30
    Lifted::new(0x7000c29b60, fn_7000c29b60 as *const (), 0), // +0x53db60 sub_53db60
    Lifted::new(0x7000c29c90, fn_7000c29c90 as *const (), 0), // +0x53dc90 sub_53dc90
    Lifted::new(0x7000c29dc0, fn_7000c29dc0 as *const (), 2), // +0x53ddc0 sub_53ddc0
    Lifted::new(0x7000c2a550, fn_7000c2a550 as *const (), 0), // +0x53e550 sub_53e550
    Lifted::new(0x7000c2a6a0, fn_7000c2a6a0 as *const (), 0), // +0x53e6a0 sub_53e6a0
    Lifted::new(0x7000c2a800, fn_7000c2a800 as *const (), 0), // +0x53e800 sub_53e800
    Lifted::new(0x7000c2a8f0, fn_7000c2a8f0 as *const (), 0), // +0x53e8f0 sub_53e8f0
    Lifted::new(0x7000c2aa90, fn_7000c2aa90 as *const (), 0), // +0x53ea90 sub_53ea90
    Lifted::new(0x7000c2aac0, fn_7000c2aac0 as *const (), 0), // +0x53eac0 sub_53eac0
    Lifted::new(0x7000c2ab00, fn_7000c2ab00 as *const (), 2), // +0x53eb00 sub_53eb00
    Lifted::new(0x7000c2ac50, fn_7000c2ac50 as *const (), 0), // +0x53ec50 sub_53ec50
    Lifted::new(0x7000c2ae20, fn_7000c2ae20 as *const (), 0), // +0x53ee20 sub_53ee20
    Lifted::new(0x7000c2aeb0, fn_7000c2aeb0 as *const (), 0), // +0x53eeb0 sub_53eeb0
    Lifted::new(0x7000c2afa0, fn_7000c2afa0 as *const (), 0), // +0x53efa0 sub_53efa0
    Lifted::new(0x7000c2b040, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3c7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x53f040 sub_53f040
    Lifted::new(0x7000c2b160, fn_7000c2b160 as *const (), 2), // +0x53f160 sub_53f160
    Lifted::new(0x7000c2b5f0, fn_7000c2b5f0 as *const (), 0), // +0x53f5f0 sub_53f5f0
    Lifted::new(0x7000c2b750, fn_7000c2b750 as *const (), 2), // +0x53f750 sub_53f750
    Lifted::new(0x7000c2b9a0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3cb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x53f9a0 sub_53f9a0
    Lifted::new(0x7000c2bac0, fn_7000c2bac0 as *const (), 0), // +0x53fac0 sub_53fac0
    Lifted::new(0x7000c2bd40, fn_7000c2bd40 as *const (), 0), // +0x53fd40 sub_53fd40
    Lifted::new(0x7000c2be40, fn_7000c2be40 as *const (), 2), // +0x53fe40 sub_53fe40
    Lifted::new(0x7000c2c2c0, fn_7000c2c2c0 as *const (), 2), // +0x5402c0 sub_5402c0
    Lifted::new(0x7000c2c610, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3d3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x540610 sub_540610
    Lifted::new(0x7000c2c730, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3df, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x540730 sub_540730
    Lifted::new(0x7000c2c850, fn_7000c2c850 as *const (), 2), // +0x540850 sub_540850
    Lifted::new(0x7000c2cc90, fn_7000c2cc90 as *const (), 2), // +0x540c90 sub_540c90
    Lifted::new(0x7000c2cfd0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3d7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x540fd0 sub_540fd0
    Lifted::new(0x7000c2d0f0, tpl_1120::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3db, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30> as *const (), 0), // +0x5410f0 sub_5410f0
    Lifted::new(0x7000c2d210, fn_7000c2d210 as *const (), 2), // +0x541210 sub_541210
    Lifted::new(0x7000c2d630, fn_7000c2d630 as *const (), 2), // +0x541630 sub_541630
    Lifted::new(0x7000c2da30, fn_7000c2da30 as *const (), 0), // +0x541a30 sub_541a30
    Lifted::new(0x7000c2daa0, fn_7000c2daa0 as *const (), 0), // +0x541aa0 sub_541aa0
    Lifted::new(0x7000c2dc10, fn_7000c2dc10 as *const (), 2), // +0x541c10 sub_541c10
    Lifted::new(0x7000c2dcd0, fn_7000c2dcd0 as *const (), 2), // +0x541cd0 sub_541cd0
    Lifted::new(0x7000c2de40, fn_7000c2de40 as *const (), 2), // +0x541e40 sub_541e40
    Lifted::new(0x7000c2e090, fn_7000c2e090 as *const (), 0), // +0x542090 sub_542090
    Lifted::new(0x7000c2e340, fn_7000c2e340 as *const (), 0), // +0x542340 sub_542340
    Lifted::new(0x7000c2e410, fn_7000c2e410 as *const (), 0), // +0x542410 sub_542410
    Lifted::new(0x7000c2e500, fn_7000c2e500 as *const (), 0), // +0x542500 sub_542500
    Lifted::new(0x7000c2e5b0, fn_7000c2e5b0 as *const (), 0), // +0x5425b0 sub_5425b0
    Lifted::new(0x7000c2e830, fn_7000c2e830 as *const (), 2), // +0x542830 sub_542830
    Lifted::new(0x7000c2ea20, fn_7000c2ea20 as *const (), 0), // +0x542a20 sub_542a20
    Lifted::new(0x7000c2eb40, fn_7000c2eb40 as *const (), 0), // +0x542b40 sub_542b40
    Lifted::new(0x7000c2ed50, fn_7000c2ed50 as *const (), 0), // +0x542d50 sub_542d50
    Lifted::new(0x7000c2ef70, fn_7000c2ef70 as *const (), 2), // +0x542f70 sub_542f70
    Lifted::new(0x7000c2f140, fn_7000c2f140 as *const (), 2), // +0x543140 sub_543140
    Lifted::new(0x7000c2f5d0, fn_7000c2f5d0 as *const (), 0), // +0x5435d0 sub_5435d0
    Lifted::new(0x7000c2f640, tpl_1123::<0x7000789230> as *const (), 0), // +0x543640 sub_543640
    Lifted::new(0x7000c2f660, fn_7000c2f660 as *const (), 0), // +0x543660 sub_543660
    Lifted::new(0x7000c2f830, fn_7000c2f830 as *const (), 0), // +0x543830 sub_543830
    Lifted::new(0x7000c2f9e0, fn_7000c2f9e0 as *const (), 0), // +0x5439e0 sub_5439e0
    Lifted::new(0x7000c2fb00, fn_7000c2fb00 as *const (), 0), // +0x543b00 sub_543b00
    Lifted::new(0x7000c2fb60, fn_7000c2fb60 as *const (), 0), // +0x543b60 sub_543b60
    Lifted::new(0x7000c2fcf0, tpl_804::<0x3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x543cf0 sub_543cf0
    Lifted::new(0x7000c2fdc0, fn_7000c2fdc0 as *const (), 0), // +0x543dc0 sub_543dc0
    Lifted::new(0x7000c2fe30, tpl_899::<0x7000c2fe40> as *const (), 0), // +0x543e30 sub_543e30
    Lifted::new(0x7000c2fe40, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x543e40 sub_543e40
    Lifted::new(0x7000c2fe80, fn_7000c2fe80 as *const (), 0), // +0x543e80 sub_543e80
    Lifted::new(0x7000c302b0, tpl_923::<0x700181c5f8> as *const (), 1), // +0x5442b0 sub_5442b0
    Lifted::new(0x7000c302d0, tpl_754::<0x700177c312> as *const (), 0), // +0x5442d0 sub_5442d0
    Lifted::new(0x7000c302f0, tpl_15 as *const (), 1), // +0x5442f0 sub_5442f0
    Lifted::new(0x7000c30300, tpl_52::<0x8> as *const (), 1), // +0x544300 sub_544300
    Lifted::new(0x7000c30310, tpl_923::<0x700181c5f8> as *const (), 1), // +0x544310 sub_544310
    Lifted::new(0x7000c30330, tpl_315::<0x7001c6bba0> as *const (), 1), // +0x544330 sub_544330
    Lifted::new(0x7000c30340, fn_7000c30340 as *const (), 0), // +0x544340 sub_544340
    Lifted::new(0x7000c303f0, fn_7000c303f0 as *const (), 0), // +0x5443f0 sub_5443f0
    Lifted::new(0x7000c30440, fn_7000c30440 as *const (), 0), // +0x544440 sub_544440
    Lifted::new(0x7000c30460, fn_7000c30460 as *const (), 0), // +0x544460 sub_544460
    Lifted::new(0x7000c30540, fn_7000c30540 as *const (), 0), // +0x544540 sub_544540
    Lifted::new(0x7000c30620, tpl_33::<0x700177c312> as *const (), 0), // +0x544620 sub_544620
    Lifted::new(0x7000c30650, tpl_969::<0x700181c6a0, 0x20, 0x700177c312> as *const (), 0), // +0x544650 sub_544650
    Lifted::new(0x7000c306a0, tpl_969::<0x700181c6a0, 0x20, 0x700177c312> as *const (), 0), // +0x5446a0 sub_5446a0
    Lifted::new(0x7000c306f0, fn_7000c306f0 as *const (), 1), // +0x5446f0 sub_5446f0
    Lifted::new(0x7000c30740, fn_7000c30740 as *const (), 1), // +0x544740 sub_544740
    Lifted::new(0x7000c307c0, fn_7000c307c0 as *const (), 0), // +0x5447c0 sub_5447c0
    Lifted::new(0x7000c30970, fn_7000c30970 as *const (), 0), // +0x544970 sub_544970
    Lifted::new(0x7000c30b60, fn_7000c30b60 as *const (), 0), // +0x544b60 sub_544b60
    Lifted::new(0x7000c30f50, fn_7000c30f50 as *const (), 0), // +0x544f50 sub_544f50
    Lifted::new(0x7000c30fa0, fn_7000c30fa0 as *const (), 2), // +0x544fa0 sub_544fa0
    Lifted::new(0x7000c31030, fn_7000c31030 as *const (), 0), // +0x545030 sub_545030
    Lifted::new(0x7000c31130, fn_7000c31130 as *const (), 0), // +0x545130 sub_545130
    Lifted::new(0x7000c31290, fn_7000c31290 as *const (), 0), // +0x545290 sub_545290
    Lifted::new(0x7000c313c0, fn_7000c313c0 as *const (), 1), // +0x5453c0 sub_5453c0
    Lifted::new(0x7000c31400, fn_7000c31400 as *const (), 0), // +0x545400 sub_545400
    Lifted::new(0x7000c31490, fn_7000c31490 as *const (), 1), // +0x545490 sub_545490
    Lifted::new(0x7000c314f0, fn_7000c314f0 as *const (), 1), // +0x5454f0 sub_5454f0
    Lifted::new(0x7000c31570, fn_7000c31570 as *const (), 0), // +0x545570 sub_545570
    Lifted::new(0x7000c315e0, tpl_1124::<0x30> as *const (), 0), // +0x5455e0 sub_5455e0
    Lifted::new(0x7000c31600, fn_7000c31600 as *const (), 1), // +0x545600 sub_545600
    Lifted::new(0x7000c31680, fn_7000c31680 as *const (), 1), // +0x545680 sub_545680
    Lifted::new(0x7000c317a0, fn_7000c317a0 as *const (), 1), // +0x5457a0 sub_5457a0
    Lifted::new(0x7000c31840, fn_7000c31840 as *const (), 1), // +0x545840 sub_545840
    Lifted::new(0x7000c31940, fn_7000c31940 as *const (), 1), // +0x545940 sub_545940
    Lifted::new(0x7000c31990, tpl_1125::<0x70007384e0, 0x7000c31840> as *const (), 0), // +0x545990 sub_545990
    Lifted::new(0x7000c319c0, fn_7000c319c0 as *const (), 0), // +0x5459c0 sub_5459c0
    Lifted::new(0x7000c31b60, fn_7000c31b60 as *const (), 0), // +0x545b60 sub_545b60
    Lifted::new(0x7000c31c80, fn_7000c31c80 as *const (), 1), // +0x545c80 sub_545c80
    Lifted::new(0x7000c31cd0, fn_7000c31cd0 as *const (), 0), // +0x545cd0 sub_545cd0
    Lifted::new(0x7000c31da0, fn_7000c31da0 as *const (), 1), // +0x545da0 sub_545da0
    Lifted::new(0x7000c31dd0, tpl_483::<0x7001c5d4ec> as *const (), 1), // +0x545dd0 sub_545dd0
    Lifted::new(0x7000c31de0, fn_7000c31de0 as *const (), 3), // +0x545de0 sub_545de0
    Lifted::new(0x7000c31e60, fn_7000c31e60 as *const (), 0), // +0x545e60 sub_545e60
    Lifted::new(0x7000c31eb0, fn_7000c31eb0 as *const (), 3), // +0x545eb0 sub_545eb0
    Lifted::new(0x7000c32210, fn_7000c32210 as *const (), 1), // +0x546210 sub_546210
    Lifted::new(0x7000c327a0, fn_7000c327a0 as *const (), 2), // +0x5467a0 sub_5467a0
    Lifted::new(0x7000c33450, fn_7000c33450 as *const (), 2), // +0x547450 sub_547450
    Lifted::new(0x7000c33590, fn_7000c33590 as *const (), 2), // +0x547590 sub_547590
    Lifted::new(0x7000c33730, fn_7000c33730 as *const (), 3), // +0x547730 sub_547730
    Lifted::new(0x7000c33920, fn_7000c33920 as *const (), 1), // +0x547920 sub_547920
    Lifted::new(0x7000c339a0, fn_7000c339a0 as *const (), 2), // +0x5479a0 sub_5479a0
    Lifted::new(0x7000c339d0, fn_7000c339d0 as *const (), 2), // +0x5479d0 sub_5479d0
    Lifted::new(0x7000c33a50, fn_7000c33a50 as *const (), 1), // +0x547a50 sub_547a50
    Lifted::new(0x7000c33ab0, fn_7000c33ab0 as *const (), 2), // +0x547ab0 sub_547ab0
    Lifted::new(0x7000c33b00, fn_7000c33b00 as *const (), 3), // +0x547b00 sub_547b00
    Lifted::new(0x7000c33b40, fn_7000c33b40 as *const (), 2), // +0x547b40 sub_547b40
    Lifted::new(0x7000c33c00, fn_7000c33c00 as *const (), 2), // +0x547c00 sub_547c00
    Lifted::new(0x7000c33ca0, fn_7000c33ca0 as *const (), 3), // +0x547ca0 sub_547ca0
    Lifted::new(0x7000c33e00, tpl_1126::<0xab8> as *const (), 3), // +0x547e00 sub_547e00
    Lifted::new(0x7000c33e10, fn_7000c33e10 as *const (), 3), // +0x547e10 sub_547e10
    Lifted::new(0x7000c33e40, fn_7000c33e40 as *const (), 2), // +0x547e40 sub_547e40
    Lifted::new(0x7000c33f10, fn_7000c33f10 as *const (), 0), // +0x547f10 sub_547f10
    Lifted::new(0x7000c33f20, fn_7000c33f20 as *const (), 0), // +0x547f20 sub_547f20
    Lifted::new(0x7000c33fb0, fn_7000c33fb0 as *const (), 2), // +0x547fb0 sub_547fb0
    Lifted::new(0x7000c34000, tpl_1127::<0x700181f1c0, 0x700075f460> as *const (), 2), // +0x548000 sub_548000
    Lifted::new(0x7000c34010, fn_7000c34010 as *const (), 0), // +0x548010 sub_548010
    Lifted::new(0x7000c34050, tpl_1126::<0xc18> as *const (), 3), // +0x548050 sub_548050
    Lifted::new(0x7000c34060, tpl_1127::<0x700181f1c0, 0x700071a8c0> as *const (), 2), // +0x548060 sub_548060
    Lifted::new(0x7000c34070, fn_7000c34070 as *const (), 2), // +0x548070 sub_548070
    Lifted::new(0x7000c34130, tpl_1126::<0xac0> as *const (), 3), // +0x548130 sub_548130
    Lifted::new(0x7000c34140, fn_7000c34140 as *const (), 2), // +0x548140 sub_548140
    Lifted::new(0x7000c34200, fn_7000c34200 as *const (), 0), // +0x548200 sub_548200
    Lifted::new(0x7000c34220, fn_7000c34220 as *const (), 0), // +0x548220 sub_548220
    Lifted::new(0x7000c34290, fn_7000c34290 as *const (), 0), // +0x548290 sub_548290
    Lifted::new(0x7000c342e0, fn_7000c342e0 as *const (), 0), // +0x5482e0 sub_5482e0
    Lifted::new(0x7000c342f0, fn_7000c342f0 as *const (), 0), // +0x5482f0 sub_5482f0
    Lifted::new(0x7000c34440, fn_7000c34440 as *const (), 3), // +0x548440 sub_548440
    Lifted::new(0x7000c344c0, fn_7000c344c0 as *const (), 0), // +0x5484c0 sub_5484c0
    Lifted::new(0x7000c344d0, fn_7000c344d0 as *const (), 2), // +0x5484d0 sub_5484d0
    Lifted::new(0x7000c34520, fn_7000c34520 as *const (), 0), // +0x548520 sub_548520
    Lifted::new(0x7000c34f20, fn_7000c34f20 as *const (), 0), // +0x548f20 sub_548f20
    Lifted::new(0x7000c34fb0, fn_7000c34fb0 as *const (), 2), // +0x548fb0 sub_548fb0
    Lifted::new(0x7000c35070, fn_7000c35070 as *const (), 0), // +0x549070 sub_549070
    Lifted::new(0x7000c350d0, fn_7000c350d0 as *const (), 2), // +0x5490d0 sub_5490d0
    Lifted::new(0x7000c35140, fn_7000c35140 as *const (), 0), // +0x549140 sub_549140
    Lifted::new(0x7000c35230, fn_7000c35230 as *const (), 0), // +0x549230 sub_549230
    Lifted::new(0x7000c35250, fn_7000c35250 as *const (), 2), // +0x549250 sub_549250
    Lifted::new(0x7000c35310, fn_7000c35310 as *const (), 0), // +0x549310 sub_549310
    Lifted::new(0x7000c353e0, fn_7000c353e0 as *const (), 2), // +0x5493e0 sub_5493e0
    Lifted::new(0x7000c35850, fn_7000c35850 as *const (), 2), // +0x549850 sub_549850
    Lifted::new(0x7000c36490, tpl_1128 as *const (), 1), // +0x54a490 sub_54a490
    Lifted::new(0x7000c364a0, tpl_1129::<0xac0, 0x700181bd98> as *const (), 3), // +0x54a4a0 sub_54a4a0
    Lifted::new(0x7000c36520, fn_7000c36520 as *const (), 2), // +0x54a520 sub_54a520
    Lifted::new(0x7000c367a0, fn_7000c367a0 as *const (), 2), // +0x54a7a0 sub_54a7a0
    Lifted::new(0x7000c36b70, tpl_1129::<0xab0, 0x700181bd98> as *const (), 3), // +0x54ab70 sub_54ab70
    Lifted::new(0x7000c36bf0, fn_7000c36bf0 as *const (), 3), // +0x54abf0 sub_54abf0
    Lifted::new(0x7000c36ce0, fn_7000c36ce0 as *const (), 3), // +0x54ace0 sub_54ace0
    Lifted::new(0x7000c36d20, fn_7000c36d20 as *const (), 2), // +0x54ad20 sub_54ad20
    Lifted::new(0x7000c370a0, fn_7000c370a0 as *const (), 0), // +0x54b0a0 sub_54b0a0
    Lifted::new(0x7000c371c0, fn_7000c371c0 as *const (), 2), // +0x54b1c0 sub_54b1c0
    Lifted::new(0x7000c377e0, fn_7000c377e0 as *const (), 1), // +0x54b7e0 sub_54b7e0
    Lifted::new(0x7000c378f0, fn_7000c378f0 as *const (), 1), // +0x54b8f0 sub_54b8f0
    Lifted::new(0x7000c37900, fn_7000c37900 as *const (), 3), // +0x54b900 sub_54b900
    Lifted::new(0x7000c379d0, fn_7000c379d0 as *const (), 0), // +0x54b9d0 sub_54b9d0
    Lifted::new(0x7000c37a00, fn_7000c37a00 as *const (), 2), // +0x54ba00 sub_54ba00
    Lifted::new(0x7000c38050, fn_7000c38050 as *const (), 2), // +0x54c050 sub_54c050
    Lifted::new(0x7000c381b0, fn_7000c381b0 as *const (), 2), // +0x54c1b0 sub_54c1b0
    Lifted::new(0x7000c38d10, fn_7000c38d10 as *const (), 0), // +0x54cd10 sub_54cd10
    Lifted::new(0x7000c38d80, fn_7000c38d80 as *const (), 1), // +0x54cd80 sub_54cd80
    Lifted::new(0x7000c38e30, fn_7000c38e30 as *const (), 1), // +0x54ce30 sub_54ce30
    Lifted::new(0x7000c38e50, tpl_20 as *const (), 1), // +0x54ce50 sub_54ce50
    Lifted::new(0x7000c38e60, tpl_20 as *const (), 1), // +0x54ce60 sub_54ce60
    Lifted::new(0x7000c38e70, tpl_20 as *const (), 1), // +0x54ce70 sub_54ce70
    Lifted::new(0x7000c38e80, tpl_20 as *const (), 1), // +0x54ce80 sub_54ce80
    Lifted::new(0x7000c38e90, tpl_1130::<0x700181f338, 0x700181f348, 0x7001c721d0, 0x70017d9cb0, 0x70017dbfc8, 0x700181f298, 0x70017a0290, 0x7001d54ef8, 0x7000c38f17, 0x700181f2a8, 0x7000c38f31, 0x70017d8e50, 0x7001c7219d, 0x7001c65c58, 0x700181f2c0, 0x70007941f0> as *const (), 2), // +0x54ce90 sub_54ce90
    Lifted::new(0x7000c39070, tpl_1130::<0x700181f338, 0x700181f348, 0x7001c721d0, 0x70017d9cb0, 0x70017dbfc8, 0x700181f298, 0x70017a0290, 0x7001d54ef8, 0x7000c390f7, 0x700181f2a8, 0x7000c39111, 0x70017d8e50, 0x7001c7219d, 0x7001c65c48, 0x700181f2c0, 0x70007941f0> as *const (), 2), // +0x54d070 sub_54d070
    Lifted::new(0x7000c39250, tpl_1131::<0x700181f308, 0x700181f328, 0x700181f338> as *const (), 3), // +0x54d250 sub_54d250
    Lifted::new(0x7000c392d0, tpl_1131::<0x700181f308, 0x700181f328, 0x700181f338> as *const (), 3), // +0x54d2d0 sub_54d2d0
    Lifted::new(0x7000c39350, tpl_1132::<0x700074d770, 0x700181f328, 0x700181f308, 0x700181f338, 0x7001c7219d, 0x7001c65c58, 0xffffffffffff6898, 0x700181f240, 0x70007941f0> as *const (), 2), // +0x54d350 sub_54d350
    Lifted::new(0x7000c39480, tpl_1132::<0x700070e7a0, 0x700181f328, 0x700181f308, 0x700181f338, 0x7001c7219d, 0x7001c65c48, 0xffffffffffff6898, 0x700181f240, 0x70007941f0> as *const (), 2), // +0x54d480 sub_54d480
    Lifted::new(0x7000c395b0, tpl_1133::<0x70017dc750, 0x700181f318, 0x700181f358, 0x700181f348> as *const (), 3), // +0x54d5b0 sub_54d5b0
    Lifted::new(0x7000c39640, tpl_1133::<0x70017dc750, 0x700181f318, 0x700181f358, 0x700181f348> as *const (), 3), // +0x54d640 sub_54d640
    Lifted::new(0x7000c396d0, tpl_1134 as *const (), 1), // +0x54d6d0 sub_54d6d0
    Lifted::new(0x7000c396f0, tpl_1134 as *const (), 1), // +0x54d6f0 sub_54d6f0
    Lifted::new(0x7000c39710, fn_7000c39710 as *const (), 1), // +0x54d710 sub_54d710
    Lifted::new(0x7000c39740, fn_7000c39740 as *const (), 1), // +0x54d740 sub_54d740
    Lifted::new(0x7000c39780, fn_7000c39780 as *const (), 0), // +0x54d780 sub_54d780
    Lifted::new(0x7000c397b0, tpl_1135::<0x7001c8e33c, 0x7000cb10a0> as *const (), 0), // +0x54d7b0 sub_54d7b0
    Lifted::new(0x7000c397f0, tpl_1136::<0x7001c8e33c, 0x948, 0x7000cb1200> as *const (), 0), // +0x54d7f0 sub_54d7f0
    Lifted::new(0x7000c39830, tpl_1135::<0x7001c8e33c, 0x7000cb1240> as *const (), 0), // +0x54d830 sub_54d830
    Lifted::new(0x7000c39870, tpl_1136::<0x7001c8e33c, 0x948, 0x7000cb12e0> as *const (), 0), // +0x54d870 sub_54d870
    Lifted::new(0x7000c398b0, fn_7000c398b0 as *const (), 1), // +0x54d8b0 sub_54d8b0
    Lifted::new(0x7000c39910, tpl_1137::<0x7001c5d4f9, 0x8f0, 0x58, 0x70009fc7f0, 0x7000c39ff0> as *const (), 0), // +0x54d910 sub_54d910
    Lifted::new(0x7000c399e0, tpl_1137::<0x7001c5d4f9, 0x8f0, 0x58, 0x70009fc7f0, 0x70007309a0> as *const (), 0), // +0x54d9e0 sub_54d9e0
    Lifted::new(0x7000c39ab0, tpl_1138::<0x700070a940, 0x700078a7d0, 0x7000c46890, 0x700177c0e0> as *const (), 0), // +0x54dab0 sub_54dab0
    Lifted::new(0x7000c39ce0, tpl_1138::<0x700070a940, 0x700078a7d0, 0x7000c46890, 0x700177c0e0> as *const (), 0), // +0x54dce0 sub_54dce0
    Lifted::new(0x7000c39f10, fn_7000c39f10 as *const (), 2), // +0x54df10 sub_54df10
    Lifted::new(0x7000c39ff0, fn_7000c39ff0 as *const (), 0), // +0x54dff0 sub_54dff0
    Lifted::new(0x7000c3a5e0, fn_7000c3a5e0 as *const (), 0), // +0x54e5e0 sub_54e5e0
    Lifted::new(0x7000c3acf0, fn_7000c3acf0 as *const (), 0), // +0x54ecf0 sub_54ecf0
    Lifted::new(0x7000c3afa0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x54efa0 sub_54efa0
    Lifted::new(0x7000c3afe0, fn_7000c3afe0 as *const (), 2), // +0x54efe0 sub_54efe0
    Lifted::new(0x7000c3b310, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000c3b49d, 0x7000c657d0, 0x7000c3b4a3, 0x70007f0910, 0x7000c3b4a9> as *const (), 0), // +0x54f310 sub_54f310
    Lifted::new(0x7000c3b510, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c3b6e4, 0x7000c658b0, 0x7000c3b6ea, 0x70007f0910, 0x7000c3b6f0> as *const (), 0), // +0x54f510 sub_54f510
    Lifted::new(0x7000c3b770, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c3b948, 0x7000c657f0, 0x7000c3b94e, 0x70007f0910, 0x7000c3b954> as *const (), 2), // +0x54f770 sub_54f770
    Lifted::new(0x7000c3b9d0, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c3bba8, 0x7000c65810, 0x7000c3bbae, 0x70007f0910, 0x7000c3bbb4> as *const (), 2), // +0x54f9d0 sub_54f9d0
    Lifted::new(0x7000c3bc30, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c3be08, 0x7000c65830, 0x7000c3be0e, 0x70007f0910, 0x7000c3be14> as *const (), 2), // +0x54fc30 sub_54fc30
    Lifted::new(0x7000c3be90, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c3c064, 0x7000c65890, 0x7000c3c06a, 0x70007f0910, 0x7000c3c070> as *const (), 0), // +0x54fe90 sub_54fe90
    Lifted::new(0x7000c3c0f0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c3c2c4, 0x7000c658d0, 0x7000c3c2ca, 0x70007f0910, 0x7000c3c2d0> as *const (), 0), // +0x5500f0 sub_5500f0
    Lifted::new(0x7000c3c350, fn_7000c3c350 as *const (), 0), // +0x550350 sub_550350
    Lifted::new(0x7000c3c580, tpl_1139::<0x19, 0x10, 0x7000c3c580, 0x28, 0x38, 0xfffffffffffffff8, 0x700177c312, 0x30, 0x7001d54968, 0x7000c3c63e> as *const (), 0), // +0x550580 sub_550580
    Lifted::new(0x7000c3c6a0, fn_7000c3c6a0 as *const (), 3), // +0x5506a0 sub_5506a0
    Lifted::new(0x7000c3c860, fn_7000c3c860 as *const (), 1), // +0x550860 sub_550860
    Lifted::new(0x7000c3ca60, fn_7000c3ca60 as *const (), 1), // +0x550a60 sub_550a60
    Lifted::new(0x7000c3cb20, fn_7000c3cb20 as *const (), 1), // +0x550b20 sub_550b20
    Lifted::new(0x7000c3ccb0, fn_7000c3ccb0 as *const (), 0), // +0x550cb0 sub_550cb0
    Lifted::new(0x7000c3d070, fn_7000c3d070 as *const (), 0), // +0x551070 sub_551070
    Lifted::new(0x7000c3d620, fn_7000c3d620 as *const (), 1), // +0x551620 sub_551620
    Lifted::new(0x7000c3d780, fn_7000c3d780 as *const (), 1), // +0x551780 sub_551780
    Lifted::new(0x7000c3d970, tpl_1140::<0x7000bcdb00, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000c3daaf, 0x7000bcd9c0, 0x7000c3dad5> as *const (), 0), // +0x551970 sub_551970
    Lifted::new(0x7000c3db30, fn_7000c3db30 as *const (), 2), // +0x551b30 sub_551b30
    Lifted::new(0x7000c3ddd0, fn_7000c3ddd0 as *const (), 0), // +0x551dd0 sub_551dd0
    Lifted::new(0x7000c3e0b0, tpl_1141::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c63db0, 0x70017d37c0, 0x70017790e0, 0x7000c3e361> as *const (), 2), // +0x5520b0 sub_5520b0
    Lifted::new(0x7000c3e410, tpl_1142::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c64480, 0x70017d37c0, 0x70017790e0, 0x7000c3e6e0> as *const (), 2), // +0x552410 sub_552410
    Lifted::new(0x7000c3e7a0, fn_7000c3e7a0 as *const (), 2), // +0x5527a0 sub_5527a0
    Lifted::new(0x7000c3eb30, tpl_65::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c64920, 0x70017d37c0, 0x70017790e0, 0x7000c3ee1a> as *const (), 2), // +0x552b30 sub_552b30
    Lifted::new(0x7000c3eee0, tpl_1143::<0x30, 0x18, 0x8, 0x10> as *const (), 1), // +0x552ee0 sub_552ee0
    Lifted::new(0x7000c3efc0, tpl_1143::<0x30, 0x18, 0x8, 0x10> as *const (), 1), // +0x552fc0 sub_552fc0
    Lifted::new(0x7000c3f0a0, fn_7000c3f0a0 as *const (), 1), // +0x5530a0 sub_5530a0
    Lifted::new(0x7000c3f1a0, tpl_1143::<0x30, 0x18, 0x8, 0x10> as *const (), 1), // +0x5531a0 sub_5531a0
    Lifted::new(0x7000c3f280, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c64b90, 0x70017d37c0, 0x70017790e0, 0x7000c3f561> as *const (), 2), // +0x553280 sub_553280
    Lifted::new(0x7000c3f620, tpl_1144::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c64e00, 0x70017d37c0, 0x70017790e0, 0x7000c3f8ec> as *const (), 2), // +0x553620 sub_553620
    Lifted::new(0x7000c3f9a0, tpl_590::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000a62120, 0x70017d37c0, 0x70017790e0, 0x7000c3fc4a> as *const (), 2), // +0x5539a0 sub_5539a0
    Lifted::new(0x7000c3fd00, fn_7000c3fd00 as *const (), 2), // +0x553d00 sub_553d00
    Lifted::new(0x7000c400a0, fn_7000c400a0 as *const (), 2), // +0x5540a0 sub_5540a0
    Lifted::new(0x7000c403a0, tpl_68 as *const (), 1), // +0x5543a0 sub_5543a0
    Lifted::new(0x7000c40450, tpl_68 as *const (), 1), // +0x554450 sub_554450
    Lifted::new(0x7000c40500, tpl_68 as *const (), 1), // +0x554500 sub_554500
    Lifted::new(0x7000c405b0, tpl_68 as *const (), 1), // +0x5545b0 sub_5545b0
    Lifted::new(0x7000c40660, tpl_68 as *const (), 1), // +0x554660 sub_554660
    Lifted::new(0x7000c40710, tpl_68 as *const (), 1), // +0x554710 sub_554710
    Lifted::new(0x7000c407c0, tpl_68 as *const (), 1), // +0x5547c0 sub_5547c0
    Lifted::new(0x7000c40870, tpl_68 as *const (), 1), // +0x554870 sub_554870
    Lifted::new(0x7000c40920, fn_7000c40920 as *const (), 0), // +0x554920 sub_554920
    Lifted::new(0x7000c40990, tpl_1006::<0x7001779e20, 0xffffffffffff6898, 0x7000763140, 0x7001820958, 0x28, 0x7001820970, 0x20> as *const (), 0), // +0x554990 sub_554990
    Lifted::new(0x7000c40a40, tpl_1145::<0x7001779e20, 0x7000753e00> as *const (), 0), // +0x554a40 sub_554a40
    Lifted::new(0x7000c40c30, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x7000c635a0> as *const (), 0), // +0x554c30 sub_554c30
    Lifted::new(0x7000c40cc0, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x7000c63710> as *const (), 0), // +0x554cc0 sub_554cc0
    Lifted::new(0x7000c40d50, tpl_78::<0x8, 0x10, 0x7001779e20, 0x18, 0x20, 0x28, 0x7000a61fb0, 0x30, 0x38> as *const (), 0), // +0x554d50 sub_554d50
    Lifted::new(0x7000c40e70, fn_7000c40e70 as *const (), 0), // +0x554e70 sub_554e70
    Lifted::new(0x7000c41060, fn_7000c41060 as *const (), 2), // +0x555060 sub_555060
    Lifted::new(0x7000c41290, fn_7000c41290 as *const (), 0), // +0x555290 sub_555290
    Lifted::new(0x7000c41a80, fn_7000c41a80 as *const (), 2), // +0x555a80 sub_555a80
    Lifted::new(0x7000c41da0, fn_7000c41da0 as *const (), 1), // +0x555da0 sub_555da0
    Lifted::new(0x7000c41dc0, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c41e63> as *const (), 0), // +0x555dc0 sub_555dc0
    Lifted::new(0x7000c41e90, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c41f33> as *const (), 0), // +0x555e90 sub_555e90
    Lifted::new(0x7000c41f60, tpl_965::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000c424c0, 0x7001d54968, 0x7000c41fc0> as *const (), 0), // +0x555f60 sub_555f60
    Lifted::new(0x7000c41fe0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c42083> as *const (), 0), // +0x555fe0 sub_555fe0
    Lifted::new(0x7000c420b0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c42153> as *const (), 0), // +0x5560b0 sub_5560b0
    Lifted::new(0x7000c42180, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c42223> as *const (), 0), // +0x556180 sub_556180
    Lifted::new(0x7000c42250, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c422f3> as *const (), 0), // +0x556250 sub_556250
    Lifted::new(0x7000c42320, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c423c3> as *const (), 0), // +0x556320 sub_556320
    Lifted::new(0x7000c423f0, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c42493> as *const (), 0), // +0x5563f0 sub_5563f0
    Lifted::new(0x7000c424c0, tpl_1146::<0x8, 0x18, 0x7000c42250, 0x700177c312> as *const (), 0), // +0x5564c0 sub_5564c0
    Lifted::new(0x7000c42550, fn_7000c42550 as *const (), 0), // +0x556550 sub_556550
    Lifted::new(0x7000c42650, fn_7000c42650 as *const (), 0), // +0x556650 sub_556650
    Lifted::new(0x7000c426d0, tpl_103::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000c4272b> as *const (), 0), // +0x5566d0 sub_5566d0
    Lifted::new(0x7000c42750, tpl_100::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000c427ca> as *const (), 0), // +0x556750 sub_556750
    Lifted::new(0x7000c427f0, fn_7000c427f0 as *const (), 0), // +0x5567f0 sub_5567f0
    Lifted::new(0x7000c42830, fn_7000c42830 as *const (), 0), // +0x556830 sub_556830
    Lifted::new(0x7000c42a40, fn_7000c42a40 as *const (), 2), // +0x556a40 sub_556a40
    Lifted::new(0x7000c42c40, tpl_409::<0x20, 0x30, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x18, 0x10, 0x7001d54968, 0x7000c42cec> as *const (), 0), // +0x556c40 sub_556c40
    Lifted::new(0x7000c42d20, fn_7000c42d20 as *const (), 0), // +0x556d20 sub_556d20
    Lifted::new(0x7000c43290, fn_7000c43290 as *const (), 0), // +0x557290 sub_557290
    Lifted::new(0x7000c43320, fn_7000c43320 as *const (), 0), // +0x557320 sub_557320
    Lifted::new(0x7000c43500, tpl_102::<0x10, 0x8, 0xc> as *const (), 0), // +0x557500 sub_557500
    Lifted::new(0x7000c43560, tpl_1147::<0x70017d2ca0> as *const (), 1), // +0x557560 sub_557560
    Lifted::new(0x7000c43570, fn_7000c43570 as *const (), 2), // +0x557570 sub_557570
    Lifted::new(0x7000c43610, tpl_113::<0x8, 0xc> as *const (), 0), // +0x557610 sub_557610
    Lifted::new(0x7000c436f0, fn_7000c436f0 as *const (), 1), // +0x5576f0 sub_5576f0
    Lifted::new(0x7000c437a0, fn_7000c437a0 as *const (), 0), // +0x5577a0 sub_5577a0
    Lifted::new(0x7000c43b50, fn_7000c43b50 as *const (), 0), // +0x557b50 sub_557b50
    Lifted::new(0x7000c43c80, fn_7000c43c80 as *const (), 0), // +0x557c80 sub_557c80
    Lifted::new(0x7000c43d60, tpl_1148::<0x7000c43fe0> as *const (), 0), // +0x557d60 sub_557d60
    Lifted::new(0x7000c43d70, tpl_899::<0x7000c44080> as *const (), 0), // +0x557d70 sub_557d70
    Lifted::new(0x7000c43d80, tpl_33::<0x700177c312> as *const (), 0), // +0x557d80 sub_557d80
    Lifted::new(0x7000c43db0, fn_7000c43db0 as *const (), 0), // +0x557db0 sub_557db0
    Lifted::new(0x7000c43e10, tpl_33::<0x700177c312> as *const (), 0), // +0x557e10 sub_557e10
    Lifted::new(0x7000c43e40, tpl_135::<0x70017d3310, 0x70006f0c20, 0x150, 0x7001d550c0, 0x138, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x557e40 sub_557e40
    Lifted::new(0x7000c43ec0, tpl_33::<0x700177c312> as *const (), 0), // +0x557ec0 sub_557ec0
    Lifted::new(0x7000c43ef0, tpl_33::<0x700177c312> as *const (), 0), // +0x557ef0 sub_557ef0
    Lifted::new(0x7000c43f20, tpl_135::<0x70017d3310, 0x70006f0c20, 0x150, 0x7001d550c0, 0x138, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x557f20 sub_557f20
    Lifted::new(0x7000c43fa0, tpl_133::<0x7000c8b680, 0x700177c312> as *const (), 0), // +0x557fa0 sub_557fa0
    Lifted::new(0x7000c43fe0, tpl_1149::<0x18, 0x28, 0xfffffffffffffff8, 0x700177c312, 0x20, 0x7001d54968, 0x7000c44059> as *const (), 0), // +0x557fe0 sub_557fe0
    Lifted::new(0x7000c44080, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x558080 sub_558080
    Lifted::new(0x7000c440c0, tpl_133::<0x7000c8b9b0, 0x700177c312> as *const (), 0), // +0x5580c0 sub_5580c0
    Lifted::new(0x7000c44100, tpl_135::<0x70017d3310, 0x70006f0c20, 0x150, 0x7001d550c0, 0x138, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x558100 sub_558100
    Lifted::new(0x7000c44180, tpl_33::<0x700177c312> as *const (), 0), // +0x558180 sub_558180
    Lifted::new(0x7000c441b0, tpl_33::<0x700177c312> as *const (), 0), // +0x5581b0 sub_5581b0
    Lifted::new(0x7000c441e0, tpl_33::<0x700177c312> as *const (), 0), // +0x5581e0 sub_5581e0
    Lifted::new(0x7000c44210, fn_7000c44210 as *const (), 2), // +0x558210 sub_558210
    Lifted::new(0x7000c442e0, fn_7000c442e0 as *const (), 2), // +0x5582e0 sub_5582e0
    Lifted::new(0x7000c44380, tpl_1150::<0x7000753e00, 0x170, 0x70007a1530, 0x28, 0x110> as *const (), 0), // +0x558380 sub_558380
    Lifted::new(0x7000c44440, fn_7000c44440 as *const (), 0), // +0x558440 sub_558440
    Lifted::new(0x7000c44590, fn_7000c44590 as *const (), 0), // +0x558590 sub_558590
    Lifted::new(0x7000c44600, fn_7000c44600 as *const (), 0), // +0x558600 sub_558600
    Lifted::new(0x7000c44670, fn_7000c44670 as *const (), 0), // +0x558670 sub_558670
    Lifted::new(0x7000c446f0, fn_7000c446f0 as *const (), 0), // +0x5586f0 sub_5586f0
    Lifted::new(0x7000c447b0, fn_7000c447b0 as *const (), 2), // +0x5587b0 sub_5587b0
    Lifted::new(0x7000c44a50, fn_7000c44a50 as *const (), 0), // +0x558a50 sub_558a50
    Lifted::new(0x7000c44ba0, fn_7000c44ba0 as *const (), 0), // +0x558ba0 sub_558ba0
    Lifted::new(0x7000c44cd0, fn_7000c44cd0 as *const (), 0), // +0x558cd0 sub_558cd0
    Lifted::new(0x7000c44df0, fn_7000c44df0 as *const (), 0), // +0x558df0 sub_558df0
    Lifted::new(0x7000c44f70, fn_7000c44f70 as *const (), 0), // +0x558f70 sub_558f70
    Lifted::new(0x7000c45130, fn_7000c45130 as *const (), 0), // +0x559130 sub_559130
    Lifted::new(0x7000c45270, fn_7000c45270 as *const (), 0), // +0x559270 sub_559270
    Lifted::new(0x7000c453e0, fn_7000c453e0 as *const (), 1), // +0x5593e0 sub_5593e0
    Lifted::new(0x7000c45400, fn_7000c45400 as *const (), 0), // +0x559400 sub_559400
    Lifted::new(0x7000c45570, fn_7000c45570 as *const (), 1), // +0x559570 sub_559570
    Lifted::new(0x7000c45580, fn_7000c45580 as *const (), 0), // +0x559580 sub_559580
    Lifted::new(0x7000c45660, tpl_143::<0x8e0, 0x7000bee610> as *const (), 0), // +0x559660 sub_559660
    Lifted::new(0x7000c45670, fn_7000c45670 as *const (), 0), // +0x559670 sub_559670
    Lifted::new(0x7000c456c0, fn_7000c456c0 as *const (), 3), // +0x5596c0 sub_5596c0
    Lifted::new(0x7000c45750, fn_7000c45750 as *const (), 0), // +0x559750 sub_559750
    Lifted::new(0x7000c45810, fn_7000c45810 as *const (), 2), // +0x559810 sub_559810
    Lifted::new(0x7000c45920, fn_7000c45920 as *const (), 2), // +0x559920 sub_559920
    Lifted::new(0x7000c45a30, fn_7000c45a30 as *const (), 0), // +0x559a30 sub_559a30
    Lifted::new(0x7000c45ae0, fn_7000c45ae0 as *const (), 1), // +0x559ae0 sub_559ae0
    Lifted::new(0x7000c45b50, fn_7000c45b50 as *const (), 0), // +0x559b50 sub_559b50
    Lifted::new(0x7000c45bd0, fn_7000c45bd0 as *const (), 0), // +0x559bd0 sub_559bd0
    Lifted::new(0x7000c45c80, fn_7000c45c80 as *const (), 0), // +0x559c80 sub_559c80
    Lifted::new(0x7000c45d00, fn_7000c45d00 as *const (), 0), // +0x559d00 sub_559d00
    Lifted::new(0x7000c45d60, fn_7000c45d60 as *const (), 0), // +0x559d60 sub_559d60
    Lifted::new(0x7000c45de0, fn_7000c45de0 as *const (), 2), // +0x559de0 sub_559de0
    Lifted::new(0x7000c45ed0, fn_7000c45ed0 as *const (), 2), // +0x559ed0 sub_559ed0
    Lifted::new(0x7000c46430, fn_7000c46430 as *const (), 0), // +0x55a430 sub_55a430
    Lifted::new(0x7000c464d0, fn_7000c464d0 as *const (), 0), // +0x55a4d0 sub_55a4d0
    Lifted::new(0x7000c46700, fn_7000c46700 as *const (), 0), // +0x55a700 sub_55a700
    Lifted::new(0x7000c46870, tpl_322::<0xffffffffffff6938, 0xffffffffffff79f0> as *const (), 1), // +0x55a870 sub_55a870
    Lifted::new(0x7000c46890, fn_7000c46890 as *const (), 1), // +0x55a890 sub_55a890
    Lifted::new(0x7000c469f0, fn_7000c469f0 as *const (), 1), // +0x55a9f0 sub_55a9f0
    Lifted::new(0x7000c46aa0, fn_7000c46aa0 as *const (), 0), // +0x55aaa0 sub_55aaa0
    Lifted::new(0x7000c46b10, fn_7000c46b10 as *const (), 2), // +0x55ab10 sub_55ab10
    Lifted::new(0x7000c47290, fn_7000c47290 as *const (), 0), // +0x55b290 sub_55b290
    Lifted::new(0x7000c472d0, fn_7000c472d0 as *const (), 2), // +0x55b2d0 sub_55b2d0
    Lifted::new(0x7000c47530, fn_7000c47530 as *const (), 2), // +0x55b530 sub_55b530
    Lifted::new(0x7000c489f0, fn_7000c489f0 as *const (), 0), // +0x55c9f0 sub_55c9f0
    Lifted::new(0x7000c48a50, fn_7000c48a50 as *const (), 2), // +0x55ca50 sub_55ca50
    Lifted::new(0x7000c48c40, fn_7000c48c40 as *const (), 0), // +0x55cc40 sub_55cc40
    Lifted::new(0x7000c48db0, fn_7000c48db0 as *const (), 0), // +0x55cdb0 sub_55cdb0
    Lifted::new(0x7000c48e70, fn_7000c48e70 as *const (), 0), // +0x55ce70 sub_55ce70
    Lifted::new(0x7000c48ec0, fn_7000c48ec0 as *const (), 0), // +0x55cec0 sub_55cec0
    Lifted::new(0x7000c48fa0, fn_7000c48fa0 as *const (), 0), // +0x55cfa0 sub_55cfa0
    Lifted::new(0x7000c49040, fn_7000c49040 as *const (), 2), // +0x55d040 sub_55d040
    Lifted::new(0x7000c493f0, fn_7000c493f0 as *const (), 2), // +0x55d3f0 sub_55d3f0
    Lifted::new(0x7000c49690, fn_7000c49690 as *const (), 2), // +0x55d690 sub_55d690
    Lifted::new(0x7000c49b50, fn_7000c49b50 as *const (), 2), // +0x55db50 sub_55db50
    Lifted::new(0x7000c49f60, fn_7000c49f60 as *const (), 2), // +0x55df60 sub_55df60
    Lifted::new(0x7000c4a220, fn_7000c4a220 as *const (), 0), // +0x55e220 sub_55e220
    Lifted::new(0x7000c4a320, fn_7000c4a320 as *const (), 0), // +0x55e320 sub_55e320
    Lifted::new(0x7000c4a370, fn_7000c4a370 as *const (), 2), // +0x55e370 sub_55e370
    Lifted::new(0x7000c4a410, fn_7000c4a410 as *const (), 0), // +0x55e410 sub_55e410
    Lifted::new(0x7000c4a9b0, fn_7000c4a9b0 as *const (), 0), // +0x55e9b0 sub_55e9b0
    Lifted::new(0x7000c4a9f0, fn_7000c4a9f0 as *const (), 2), // +0x55e9f0 sub_55e9f0
    Lifted::new(0x7000c4ac90, fn_7000c4ac90 as *const (), 0), // +0x55ec90 sub_55ec90
    Lifted::new(0x7000c4ae00, fn_7000c4ae00 as *const (), 0), // +0x55ee00 sub_55ee00
    Lifted::new(0x7000c4ae90, tpl_1151::<0x7000c4b000> as *const (), 0), // +0x55ee90 sub_55ee90
    Lifted::new(0x7000c4aec0, fn_7000c4aec0 as *const (), 0), // +0x55eec0 sub_55eec0
    Lifted::new(0x7000c4afc0, fn_7000c4afc0 as *const (), 0), // +0x55efc0 sub_55efc0
    Lifted::new(0x7000c4b000, fn_7000c4b000 as *const (), 0), // +0x55f000 sub_55f000
    Lifted::new(0x7000c4b0f0, fn_7000c4b0f0 as *const (), 0), // +0x55f0f0 sub_55f0f0
    Lifted::new(0x7000c4b1d0, fn_7000c4b1d0 as *const (), 0), // +0x55f1d0 sub_55f1d0
    Lifted::new(0x7000c4b250, tpl_196 as *const (), 1), // +0x55f250 sub_55f250
    Lifted::new(0x7000c4b260, fn_7000c4b260 as *const (), 0), // +0x55f260 sub_55f260
    Lifted::new(0x7000c4b310, fn_7000c4b310 as *const (), 0), // +0x55f310 sub_55f310
    Lifted::new(0x7000c4b3f0, fn_7000c4b3f0 as *const (), 0), // +0x55f3f0 sub_55f3f0
    Lifted::new(0x7000c4b5b0, tpl_143::<0x8e0, 0x7000bee800> as *const (), 0), // +0x55f5b0 sub_55f5b0
    Lifted::new(0x7000c4b5c0, fn_7000c4b5c0 as *const (), 0), // +0x55f5c0 sub_55f5c0
    Lifted::new(0x7000c4b610, fn_7000c4b610 as *const (), 0), // +0x55f610 sub_55f610
    Lifted::new(0x7000c4b730, fn_7000c4b730 as *const (), 2), // +0x55f730 sub_55f730
    Lifted::new(0x7000c4c270, fn_7000c4c270 as *const (), 0), // +0x560270 sub_560270
    Lifted::new(0x7000c4c2b0, tpl_355::<0x940, 0x30> as *const (), 1), // +0x5602b0 sub_5602b0
    Lifted::new(0x7000c4c2d0, tpl_355::<0x940, 0x28> as *const (), 1), // +0x5602d0 sub_5602d0
    Lifted::new(0x7000c4c2f0, fn_7000c4c2f0 as *const (), 0), // +0x5602f0 sub_5602f0
    Lifted::new(0x7000c4c350, fn_7000c4c350 as *const (), 0), // +0x560350 sub_560350
    Lifted::new(0x7000c4c4a0, fn_7000c4c4a0 as *const (), 0), // +0x5604a0 sub_5604a0
    Lifted::new(0x7000c4c510, fn_7000c4c510 as *const (), 0), // +0x560510 sub_560510
    Lifted::new(0x7000c4c580, tpl_1152::<0x8c8, 0x2118, 0xb9, 0x8, 0xb0, 0x7000c94850> as *const (), 0), // +0x560580 sub_560580
    Lifted::new(0x7000c4c5e0, fn_7000c4c5e0 as *const (), 0), // +0x5605e0 sub_5605e0
    Lifted::new(0x7000c4c6d0, fn_7000c4c6d0 as *const (), 2), // +0x5606d0 sub_5606d0
    Lifted::new(0x7000c4ccb0, fn_7000c4ccb0 as *const (), 2), // +0x560cb0 sub_560cb0
    Lifted::new(0x7000c4ce80, fn_7000c4ce80 as *const (), 0), // +0x560e80 sub_560e80
    Lifted::new(0x7000c4ceb0, fn_7000c4ceb0 as *const (), 0), // +0x560eb0 sub_560eb0
    Lifted::new(0x7000c4cee0, fn_7000c4cee0 as *const (), 0), // +0x560ee0 sub_560ee0
    Lifted::new(0x7000c4cf80, fn_7000c4cf80 as *const (), 0), // +0x560f80 sub_560f80
    Lifted::new(0x7000c4cfc0, fn_7000c4cfc0 as *const (), 0), // +0x560fc0 sub_560fc0
    Lifted::new(0x7000c4d0f0, fn_7000c4d0f0 as *const (), 2), // +0x5610f0 sub_5610f0
    Lifted::new(0x7000c4d5c0, fn_7000c4d5c0 as *const (), 0), // +0x5615c0 sub_5615c0
    Lifted::new(0x7000c4d880, fn_7000c4d880 as *const (), 0), // +0x561880 sub_561880
    Lifted::new(0x7000c4d9e0, fn_7000c4d9e0 as *const (), 0), // +0x5619e0 sub_5619e0
    Lifted::new(0x7000c4dae0, fn_7000c4dae0 as *const (), 0), // +0x561ae0 sub_561ae0
    Lifted::new(0x7000c4dce0, fn_7000c4dce0 as *const (), 2), // +0x561ce0 sub_561ce0
    Lifted::new(0x7000c4e230, fn_7000c4e230 as *const (), 2), // +0x562230 sub_562230
    Lifted::new(0x7000c4f0e0, fn_7000c4f0e0 as *const (), 2), // +0x5630e0 sub_5630e0
    Lifted::new(0x7000c4f400, fn_7000c4f400 as *const (), 2), // +0x563400 sub_563400
    Lifted::new(0x7000c4f6e0, fn_7000c4f6e0 as *const (), 0), // +0x5636e0 sub_5636e0
    Lifted::new(0x7000c4f940, fn_7000c4f940 as *const (), 0), // +0x563940 sub_563940
    Lifted::new(0x7000c4fa10, fn_7000c4fa10 as *const (), 0), // +0x563a10 sub_563a10
    Lifted::new(0x7000c4fc40, fn_7000c4fc40 as *const (), 1), // +0x563c40 sub_563c40
    Lifted::new(0x7000c4fd20, fn_7000c4fd20 as *const (), 0), // +0x563d20 sub_563d20
    Lifted::new(0x7000c4fde0, fn_7000c4fde0 as *const (), 0), // +0x563de0 sub_563de0
    Lifted::new(0x7000c4fe00, tpl_275::<0x940, 0x8> as *const (), 1), // +0x563e00 sub_563e00
    Lifted::new(0x7000c4fe10, tpl_36::<0x968> as *const (), 1), // +0x563e10 sub_563e10
    Lifted::new(0x7000c4fe20, fn_7000c4fe20 as *const (), 1), // +0x563e20 sub_563e20
    Lifted::new(0x7000c4fe50, fn_7000c4fe50 as *const (), 0), // +0x563e50 sub_563e50
    Lifted::new(0x7000c4fee0, fn_7000c4fee0 as *const (), 0), // +0x563ee0 sub_563ee0
    Lifted::new(0x7000c4ff20, tpl_36::<0x8> as *const (), 1), // +0x563f20 sub_563f20
    Lifted::new(0x7000c4ff30, fn_7000c4ff30 as *const (), 0), // +0x563f30 sub_563f30
    Lifted::new(0x7000c52780, fn_7000c52780 as *const (), 2), // +0x566780 sub_566780
    Lifted::new(0x7000c52840, fn_7000c52840 as *const (), 0), // +0x566840 sub_566840
    Lifted::new(0x7000c52890, fn_7000c52890 as *const (), 0), // +0x566890 sub_566890
    Lifted::new(0x7000c52a50, tpl_172::<0x118> as *const (), 1), // +0x566a50 sub_566a50
    Lifted::new(0x7000c52a60, fn_7000c52a60 as *const (), 0), // +0x566a60 sub_566a60
    Lifted::new(0x7000c52ad0, fn_7000c52ad0 as *const (), 2), // +0x566ad0 sub_566ad0
    Lifted::new(0x7000c52cb0, fn_7000c52cb0 as *const (), 2), // +0x566cb0 sub_566cb0
    Lifted::new(0x7000c52dc0, fn_7000c52dc0 as *const (), 2), // +0x566dc0 sub_566dc0
    Lifted::new(0x7000c52ea0, fn_7000c52ea0 as *const (), 2), // +0x566ea0 sub_566ea0
    Lifted::new(0x7000c52f90, fn_7000c52f90 as *const (), 1), // +0x566f90 sub_566f90
    Lifted::new(0x7000c52fd0, fn_7000c52fd0 as *const (), 0), // +0x566fd0 sub_566fd0
    Lifted::new(0x7000c530e0, fn_7000c530e0 as *const (), 0), // +0x5670e0 sub_5670e0
    Lifted::new(0x7000c53110, fn_7000c53110 as *const (), 0), // +0x567110 sub_567110
    Lifted::new(0x7000c53150, fn_7000c53150 as *const (), 2), // +0x567150 sub_567150
    Lifted::new(0x7000c531b0, fn_7000c531b0 as *const (), 2), // +0x5671b0 sub_5671b0
    Lifted::new(0x7000c537e0, fn_7000c537e0 as *const (), 2), // +0x5677e0 sub_5677e0
    Lifted::new(0x7000c53920, fn_7000c53920 as *const (), 0), // +0x567920 sub_567920
    Lifted::new(0x7000c53aa0, fn_7000c53aa0 as *const (), 0), // +0x567aa0 sub_567aa0
    Lifted::new(0x7000c53c20, fn_7000c53c20 as *const (), 0), // +0x567c20 sub_567c20
    Lifted::new(0x7000c53c60, tpl_440::<0x6a0> as *const (), 1), // +0x567c60 sub_567c60
    Lifted::new(0x7000c53c70, fn_7000c53c70 as *const (), 2), // +0x567c70 sub_567c70
    Lifted::new(0x7000c53f90, fn_7000c53f90 as *const (), 0), // +0x567f90 sub_567f90
    Lifted::new(0x7000c54120, fn_7000c54120 as *const (), 0), // +0x568120 sub_568120
    Lifted::new(0x7000c54180, tpl_51::<0x1b0> as *const (), 1), // +0x568180 sub_568180
    Lifted::new(0x7000c54190, tpl_1153::<0x7000c39f10> as *const (), 0), // +0x568190 sub_568190
    Lifted::new(0x7000c541b0, fn_7000c541b0 as *const (), 0), // +0x5681b0 sub_5681b0
    Lifted::new(0x7000c542c0, fn_7000c542c0 as *const (), 2), // +0x5682c0 sub_5682c0
    Lifted::new(0x7000c545a0, fn_7000c545a0 as *const (), 2), // +0x5685a0 sub_5685a0
    Lifted::new(0x7000c54880, fn_7000c54880 as *const (), 0), // +0x568880 sub_568880
    Lifted::new(0x7000c54a70, fn_7000c54a70 as *const (), 1), // +0x568a70 sub_568a70
    Lifted::new(0x7000c54aa0, fn_7000c54aa0 as *const (), 0), // +0x568aa0 sub_568aa0
    Lifted::new(0x7000c54b60, tpl_47::<0x4258> as *const (), 1), // +0x568b60 sub_568b60
    Lifted::new(0x7000c54b70, fn_7000c54b70 as *const (), 0), // +0x568b70 sub_568b70
    Lifted::new(0x7000c54bd0, fn_7000c54bd0 as *const (), 0), // +0x568bd0 sub_568bd0
    Lifted::new(0x7000c54c70, fn_7000c54c70 as *const (), 0), // +0x568c70 sub_568c70
    Lifted::new(0x7000c54cf0, fn_7000c54cf0 as *const (), 0), // +0x568cf0 sub_568cf0
    Lifted::new(0x7000c55150, fn_7000c55150 as *const (), 0), // +0x569150 sub_569150
    Lifted::new(0x7000c551d0, fn_7000c551d0 as *const (), 0), // +0x5691d0 sub_5691d0
    Lifted::new(0x7000c55260, fn_7000c55260 as *const (), 0), // +0x569260 sub_569260
    Lifted::new(0x7000c552b0, tpl_488::<0x58, 0xa718, 0x700079f490> as *const (), 0), // +0x5692b0 sub_5692b0
    Lifted::new(0x7000c552d0, fn_7000c552d0 as *const (), 0), // +0x5692d0 sub_5692d0
    Lifted::new(0x7000c55370, fn_7000c55370 as *const (), 0), // +0x569370 sub_569370
    Lifted::new(0x7000c553c0, fn_7000c553c0 as *const (), 0), // +0x5693c0 sub_5693c0
    Lifted::new(0x7000c554d0, fn_7000c554d0 as *const (), 0), // +0x5694d0 sub_5694d0
    Lifted::new(0x7000c556a0, fn_7000c556a0 as *const (), 0), // +0x5696a0 sub_5696a0
    Lifted::new(0x7000c556f0, fn_7000c556f0 as *const (), 0), // +0x5696f0 sub_5696f0
    Lifted::new(0x7000c55730, fn_7000c55730 as *const (), 0), // +0x569730 sub_569730
    Lifted::new(0x7000c558a0, fn_7000c558a0 as *const (), 2), // +0x5698a0 sub_5698a0
    Lifted::new(0x7000c55c20, fn_7000c55c20 as *const (), 2), // +0x569c20 sub_569c20
    Lifted::new(0x7000c55ec0, fn_7000c55ec0 as *const (), 2), // +0x569ec0 sub_569ec0
    Lifted::new(0x7000c561c0, fn_7000c561c0 as *const (), 0), // +0x56a1c0 sub_56a1c0
    Lifted::new(0x7000c56290, fn_7000c56290 as *const (), 1), // +0x56a290 sub_56a290
    Lifted::new(0x7000c562f0, fn_7000c562f0 as *const (), 1), // +0x56a2f0 sub_56a2f0
    Lifted::new(0x7000c56310, fn_7000c56310 as *const (), 1), // +0x56a310 sub_56a310
    Lifted::new(0x7000c56370, fn_7000c56370 as *const (), 2), // +0x56a370 sub_56a370
    Lifted::new(0x7000c56760, fn_7000c56760 as *const (), 0), // +0x56a760 sub_56a760
    Lifted::new(0x7000c567c0, fn_7000c567c0 as *const (), 2), // +0x56a7c0 sub_56a7c0
    Lifted::new(0x7000c56b30, fn_7000c56b30 as *const (), 0), // +0x56ab30 sub_56ab30
    Lifted::new(0x7000c56c70, fn_7000c56c70 as *const (), 1), // +0x56ac70 sub_56ac70
    Lifted::new(0x7000c56c80, fn_7000c56c80 as *const (), 1), // +0x56ac80 sub_56ac80
    Lifted::new(0x7000c56c90, fn_7000c56c90 as *const (), 2), // +0x56ac90 sub_56ac90
    Lifted::new(0x7000c56fe0, fn_7000c56fe0 as *const (), 2), // +0x56afe0 sub_56afe0
    Lifted::new(0x7000c57010, fn_7000c57010 as *const (), 0), // +0x56b010 sub_56b010
    Lifted::new(0x7000c57120, fn_7000c57120 as *const (), 0), // +0x56b120 sub_56b120
    Lifted::new(0x7000c57170, fn_7000c57170 as *const (), 0), // +0x56b170 sub_56b170
    Lifted::new(0x7000c571c0, fn_7000c571c0 as *const (), 0), // +0x56b1c0 sub_56b1c0
    Lifted::new(0x7000c57240, fn_7000c57240 as *const (), 0), // +0x56b240 sub_56b240
    Lifted::new(0x7000c57420, fn_7000c57420 as *const (), 0), // +0x56b420 sub_56b420
    Lifted::new(0x7000c574a0, fn_7000c574a0 as *const (), 1), // +0x56b4a0 sub_56b4a0
    Lifted::new(0x7000c574c0, fn_7000c574c0 as *const (), 0), // +0x56b4c0 sub_56b4c0
    Lifted::new(0x7000c57520, fn_7000c57520 as *const (), 0), // +0x56b520 sub_56b520
    Lifted::new(0x7000c576a0, fn_7000c576a0 as *const (), 0), // +0x56b6a0 sub_56b6a0
    Lifted::new(0x7000c576e0, fn_7000c576e0 as *const (), 0), // +0x56b6e0 sub_56b6e0
    Lifted::new(0x7000c57710, fn_7000c57710 as *const (), 0), // +0x56b710 sub_56b710
    Lifted::new(0x7000c57740, tpl_1152::<0x8c8, 0x2118, 0xb9, 0x8, 0xb0, 0x7000c94850> as *const (), 0), // +0x56b740 sub_56b740
    Lifted::new(0x7000c577a0, fn_7000c577a0 as *const (), 2), // +0x56b7a0 sub_56b7a0
    Lifted::new(0x7000c579a0, fn_7000c579a0 as *const (), 0), // +0x56b9a0 sub_56b9a0
    Lifted::new(0x7000c57ae0, fn_7000c57ae0 as *const (), 0), // +0x56bae0 sub_56bae0
    Lifted::new(0x7000c57bf0, fn_7000c57bf0 as *const (), 2), // +0x56bbf0 sub_56bbf0
    Lifted::new(0x7000c57cf0, fn_7000c57cf0 as *const (), 1), // +0x56bcf0 sub_56bcf0
    Lifted::new(0x7000c57d10, fn_7000c57d10 as *const (), 1), // +0x56bd10 sub_56bd10
    Lifted::new(0x7000c57d60, fn_7000c57d60 as *const (), 1), // +0x56bd60 sub_56bd60
    Lifted::new(0x7000c57d70, tpl_1154::<0x918, 0x44d8> as *const (), 1), // +0x56bd70 sub_56bd70
    Lifted::new(0x7000c57da0, tpl_1154::<0x918, 0xce78> as *const (), 1), // +0x56bda0 sub_56bda0
    Lifted::new(0x7000c57dd0, tpl_143::<0x8e0, 0x7000beec00> as *const (), 0), // +0x56bdd0 sub_56bdd0
    Lifted::new(0x7000c57de0, fn_7000c57de0 as *const (), 0), // +0x56bde0 sub_56bde0
    Lifted::new(0x7000c57ed0, fn_7000c57ed0 as *const (), 0), // +0x56bed0 sub_56bed0
    Lifted::new(0x7000c57fa0, fn_7000c57fa0 as *const (), 0), // +0x56bfa0 sub_56bfa0
    Lifted::new(0x7000c58020, fn_7000c58020 as *const (), 0), // +0x56c020 sub_56c020
    Lifted::new(0x7000c58190, fn_7000c58190 as *const (), 0), // +0x56c190 sub_56c190
    Lifted::new(0x7000c58270, tpl_1155::<0x70007542d0, 0x70017dbfc8> as *const (), 2), // +0x56c270 sub_56c270
    Lifted::new(0x7000c583a0, tpl_1155::<0x70007542d0, 0x70017dbfc8> as *const (), 2), // +0x56c3a0 sub_56c3a0
    Lifted::new(0x7000c584d0, fn_7000c584d0 as *const (), 2), // +0x56c4d0 sub_56c4d0
    Lifted::new(0x7000c59190, fn_7000c59190 as *const (), 2), // +0x56d190 sub_56d190
    Lifted::new(0x7000c59310, fn_7000c59310 as *const (), 0), // +0x56d310 sub_56d310
    Lifted::new(0x7000c59460, fn_7000c59460 as *const (), 0), // +0x56d460 sub_56d460
    Lifted::new(0x7000c59570, fn_7000c59570 as *const (), 1), // +0x56d570 sub_56d570
    Lifted::new(0x7000c595e0, fn_7000c595e0 as *const (), 0), // +0x56d5e0 sub_56d5e0
    Lifted::new(0x7000c59680, fn_7000c59680 as *const (), 0), // +0x56d680 sub_56d680
    Lifted::new(0x7000c597e0, fn_7000c597e0 as *const (), 2), // +0x56d7e0 sub_56d7e0
    Lifted::new(0x7000c5a6d0, fn_7000c5a6d0 as *const (), 2), // +0x56e6d0 sub_56e6d0
    Lifted::new(0x7000c5a710, fn_7000c5a710 as *const (), 2), // +0x56e710 sub_56e710
    Lifted::new(0x7000c5aba0, fn_7000c5aba0 as *const (), 2), // +0x56eba0 sub_56eba0
    Lifted::new(0x7000c5b0e0, fn_7000c5b0e0 as *const (), 0), // +0x56f0e0 sub_56f0e0
    Lifted::new(0x7000c5b1a0, tpl_1156::<0x650, 0x7000cc9ad0> as *const (), 0), // +0x56f1a0 sub_56f1a0
    Lifted::new(0x7000c5b1e0, fn_7000c5b1e0 as *const (), 0), // +0x56f1e0 sub_56f1e0
    Lifted::new(0x7000c5b240, tpl_1156::<0x648, 0x7000cc9f40> as *const (), 0), // +0x56f240 sub_56f240
    Lifted::new(0x7000c5b290, fn_7000c5b290 as *const (), 2), // +0x56f290 sub_56f290
    Lifted::new(0x7000c5b920, fn_7000c5b920 as *const (), 0), // +0x56f920 sub_56f920
    Lifted::new(0x7000c5b9e0, tpl_1156::<0x648, 0x7000cc9f40> as *const (), 0), // +0x56f9e0 sub_56f9e0
    Lifted::new(0x7000c5ba30, fn_7000c5ba30 as *const (), 0), // +0x56fa30 sub_56fa30
    Lifted::new(0x7000c5bb80, fn_7000c5bb80 as *const (), 0), // +0x56fb80 sub_56fb80
    Lifted::new(0x7000c5bd60, fn_7000c5bd60 as *const (), 0), // +0x56fd60 sub_56fd60
    Lifted::new(0x7000c5bea0, fn_7000c5bea0 as *const (), 2), // +0x56fea0 sub_56fea0
    Lifted::new(0x7000c5bef0, fn_7000c5bef0 as *const (), 2), // +0x56fef0 sub_56fef0
    Lifted::new(0x7000c5c340, fn_7000c5c340 as *const (), 2), // +0x570340 sub_570340
    Lifted::new(0x7000c5c490, fn_7000c5c490 as *const (), 0), // +0x570490 sub_570490
    Lifted::new(0x7000c5c8a0, fn_7000c5c8a0 as *const (), 2), // +0x5708a0 sub_5708a0
    Lifted::new(0x7000c5c990, fn_7000c5c990 as *const (), 0), // +0x570990 sub_570990
    Lifted::new(0x7000c5cb50, fn_7000c5cb50 as *const (), 0), // +0x570b50 sub_570b50
    Lifted::new(0x7000c5cc20, fn_7000c5cc20 as *const (), 1), // +0x570c20 sub_570c20
    Lifted::new(0x7000c5cc80, tpl_1150::<0x7000753e00, 0x170, 0x70007a1530, 0x30, 0x110> as *const (), 0), // +0x570c80 sub_570c80
    Lifted::new(0x7000c5cd40, fn_7000c5cd40 as *const (), 0), // +0x570d40 sub_570d40
    Lifted::new(0x7000c5ce80, fn_7000c5ce80 as *const (), 2), // +0x570e80 sub_570e80
    Lifted::new(0x7000c5d120, tpl_1157::<0x668, 0x670, 0x10, 0x70017d2c10, 0x7001d54ef8, 0x7000c5d184, 0xffffffffffffffe8, 0xfffffffffffffff0, 0x8, 0xfffffffffffffff8> as *const (), 0), // +0x571120 sub_571120
    Lifted::new(0x7000c5d1e0, tpl_1157::<0x680, 0x688, 0x10, 0x70017d2c10, 0x7001d54ef8, 0x7000c5d244, 0xffffffffffffffe8, 0xfffffffffffffff0, 0x8, 0xfffffffffffffff8> as *const (), 0), // +0x5711e0 sub_5711e0
    Lifted::new(0x7000c5d2a0, fn_7000c5d2a0 as *const (), 0), // +0x5712a0 sub_5712a0
    Lifted::new(0x7000c5d3c0, fn_7000c5d3c0 as *const (), 0), // +0x5713c0 sub_5713c0
    Lifted::new(0x7000c5d4f0, fn_7000c5d4f0 as *const (), 0), // +0x5714f0 sub_5714f0
    Lifted::new(0x7000c5d7c0, fn_7000c5d7c0 as *const (), 0), // +0x5717c0 sub_5717c0
    Lifted::new(0x7000c5d830, fn_7000c5d830 as *const (), 2), // +0x571830 sub_571830
    Lifted::new(0x7000c5db50, fn_7000c5db50 as *const (), 2), // +0x571b50 sub_571b50
    Lifted::new(0x7000c5df60, fn_7000c5df60 as *const (), 0), // +0x571f60 sub_571f60
    Lifted::new(0x7000c5e000, fn_7000c5e000 as *const (), 2), // +0x572000 sub_572000
    Lifted::new(0x7000c5e0f0, fn_7000c5e0f0 as *const (), 0), // +0x5720f0 sub_5720f0
    Lifted::new(0x7000c5e130, fn_7000c5e130 as *const (), 0), // +0x572130 sub_572130
    Lifted::new(0x7000c5e230, fn_7000c5e230 as *const (), 0), // +0x572230 sub_572230
    Lifted::new(0x7000c5e240, fn_7000c5e240 as *const (), 1), // +0x572240 sub_572240
    Lifted::new(0x7000c5e260, tpl_143::<0x28, 0x7000c45ed0> as *const (), 0), // +0x572260 sub_572260
    Lifted::new(0x7000c5e270, fn_7000c5e270 as *const (), 2), // +0x572270 sub_572270
    Lifted::new(0x7000c5e6c0, fn_7000c5e6c0 as *const (), 0), // +0x5726c0 sub_5726c0
    Lifted::new(0x7000c5e880, fn_7000c5e880 as *const (), 1), // +0x572880 sub_572880
    Lifted::new(0x7000c5e8a0, tpl_1158::<0xffffffffffff7a08> as *const (), 1), // +0x5728a0 sub_5728a0
    Lifted::new(0x7000c5e8b0, fn_7000c5e8b0 as *const (), 1), // +0x5728b0 sub_5728b0
    Lifted::new(0x7000c5e900, tpl_51::<0xffffffffffff79b8> as *const (), 1), // +0x572900 sub_572900
    Lifted::new(0x7000c5e910, tpl_143::<0x940, 0x700075b880> as *const (), 0), // +0x572910 sub_572910
    Lifted::new(0x7000c5e920, tpl_51::<0x968> as *const (), 1), // +0x572920 sub_572920
    Lifted::new(0x7000c5e930, fn_7000c5e930 as *const (), 0), // +0x572930 sub_572930
    Lifted::new(0x7000c5e970, tpl_51::<0xffffffffffff79f8> as *const (), 1), // +0x572970 sub_572970
    Lifted::new(0x7000c5e980, tpl_51::<0xffffffffffff79d8> as *const (), 1), // +0x572980 sub_572980
    Lifted::new(0x7000c5e990, tpl_51::<0xffffffffffff79d0> as *const (), 1), // +0x572990 sub_572990
    Lifted::new(0x7000c5e9a0, fn_7000c5e9a0 as *const (), 0), // +0x5729a0 sub_5729a0
    Lifted::new(0x7000c5ee10, fn_7000c5ee10 as *const (), 1), // +0x572e10 sub_572e10
    Lifted::new(0x7000c5ee30, fn_7000c5ee30 as *const (), 0), // +0x572e30 sub_572e30
    Lifted::new(0x7000c5f910, fn_7000c5f910 as *const (), 1), // +0x573910 sub_573910
    Lifted::new(0x7000c5f960, fn_7000c5f960 as *const (), 0), // +0x573960 sub_573960
    Lifted::new(0x7000c5fa90, fn_7000c5fa90 as *const (), 1), // +0x573a90 sub_573a90
    Lifted::new(0x7000c5fab0, fn_7000c5fab0 as *const (), 2), // +0x573ab0 sub_573ab0
    Lifted::new(0x7000c5fb40, fn_7000c5fb40 as *const (), 0), // +0x573b40 sub_573b40
    Lifted::new(0x7000c5fbc0, fn_7000c5fbc0 as *const (), 0), // +0x573bc0 sub_573bc0
    Lifted::new(0x7000c5fc40, fn_7000c5fc40 as *const (), 0), // +0x573c40 sub_573c40
    Lifted::new(0x7000c5fd00, fn_7000c5fd00 as *const (), 0), // +0x573d00 sub_573d00
    Lifted::new(0x7000c5fd20, fn_7000c5fd20 as *const (), 0), // +0x573d20 sub_573d20
    Lifted::new(0x7000c5fde0, fn_7000c5fde0 as *const (), 2), // +0x573de0 sub_573de0
    Lifted::new(0x7000c5ff50, fn_7000c5ff50 as *const (), 0), // +0x573f50 sub_573f50
    Lifted::new(0x7000c60000, fn_7000c60000 as *const (), 0), // +0x574000 sub_574000
    Lifted::new(0x7000c60050, fn_7000c60050 as *const (), 0), // +0x574050 sub_574050
    Lifted::new(0x7000c60070, fn_7000c60070 as *const (), 0), // +0x574070 sub_574070
    Lifted::new(0x7000c600d0, tpl_15 as *const (), 1), // +0x5740d0 sub_5740d0
    Lifted::new(0x7000c600e0, fn_7000c600e0 as *const (), 0), // +0x5740e0 sub_5740e0
    Lifted::new(0x7000c60330, fn_7000c60330 as *const (), 2), // +0x574330 sub_574330
    Lifted::new(0x7000c60960, tpl_488::<0x58, 0xa718, 0x70007207e0> as *const (), 0), // +0x574960 sub_574960
    Lifted::new(0x7000c60980, fn_7000c60980 as *const (), 0), // +0x574980 sub_574980
    Lifted::new(0x7000c60c40, fn_7000c60c40 as *const (), 0), // +0x574c40 sub_574c40
    Lifted::new(0x7000c60c80, fn_7000c60c80 as *const (), 0), // +0x574c80 sub_574c80
    Lifted::new(0x7000c60d10, fn_7000c60d10 as *const (), 0), // +0x574d10 sub_574d10
    Lifted::new(0x7000c60dc0, fn_7000c60dc0 as *const (), 0), // +0x574dc0 sub_574dc0
    Lifted::new(0x7000c60e80, fn_7000c60e80 as *const (), 0), // +0x574e80 sub_574e80
    Lifted::new(0x7000c60f10, fn_7000c60f10 as *const (), 0), // +0x574f10 sub_574f10
    Lifted::new(0x7000c61120, fn_7000c61120 as *const (), 1), // +0x575120 sub_575120
    Lifted::new(0x7000c61170, fn_7000c61170 as *const (), 0), // +0x575170 sub_575170
    Lifted::new(0x7000c611b0, fn_7000c611b0 as *const (), 0), // +0x5751b0 sub_5751b0
    Lifted::new(0x7000c611e0, fn_7000c611e0 as *const (), 0), // +0x5751e0 sub_5751e0
    Lifted::new(0x7000c61280, fn_7000c61280 as *const (), 0), // +0x575280 sub_575280
    Lifted::new(0x7000c61320, fn_7000c61320 as *const (), 2), // +0x575320 sub_575320
    Lifted::new(0x7000c61570, fn_7000c61570 as *const (), 2), // +0x575570 sub_575570
    Lifted::new(0x7000c617e0, fn_7000c617e0 as *const (), 2), // +0x5757e0 sub_5757e0
    Lifted::new(0x7000c61920, fn_7000c61920 as *const (), 1), // +0x575920 sub_575920
    Lifted::new(0x7000c61940, fn_7000c61940 as *const (), 0), // +0x575940 sub_575940
    Lifted::new(0x7000c61b10, fn_7000c61b10 as *const (), 0), // +0x575b10 sub_575b10
    Lifted::new(0x7000c61c30, fn_7000c61c30 as *const (), 0), // +0x575c30 sub_575c30
    Lifted::new(0x7000c61c40, fn_7000c61c40 as *const (), 0), // +0x575c40 sub_575c40
    Lifted::new(0x7000c61c90, fn_7000c61c90 as *const (), 0), // +0x575c90 sub_575c90
    Lifted::new(0x7000c61e30, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000c61e48> as *const (), 0), // +0x575e30 sub_575e30
    Lifted::new(0x7000c61e50, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000c61e68> as *const (), 0), // +0x575e50 sub_575e50
    Lifted::new(0x7000c61e70, fn_7000c61e70 as *const (), 0), // +0x575e70 sub_575e70
    Lifted::new(0x7000c61f50, fn_7000c61f50 as *const (), 0), // +0x575f50 sub_575f50
    Lifted::new(0x7000c620b0, fn_7000c620b0 as *const (), 0), // +0x5760b0 sub_5760b0
    Lifted::new(0x7000c62180, fn_7000c62180 as *const (), 0), // +0x576180 sub_576180
    Lifted::new(0x7000c622d0, fn_7000c622d0 as *const (), 0), // +0x5762d0 sub_5762d0
    Lifted::new(0x7000c623b0, fn_7000c623b0 as *const (), 0), // +0x5763b0 sub_5763b0
    Lifted::new(0x7000c62440, fn_7000c62440 as *const (), 0), // +0x576440 sub_576440
    Lifted::new(0x7000c62450, fn_7000c62450 as *const (), 0), // +0x576450 sub_576450
    Lifted::new(0x7000c62480, fn_7000c62480 as *const (), 1), // +0x576480 sub_576480
    Lifted::new(0x7000c624d0, fn_7000c624d0 as *const (), 1), // +0x5764d0 sub_5764d0
    Lifted::new(0x7000c624f0, fn_7000c624f0 as *const (), 1), // +0x5764f0 sub_5764f0
    Lifted::new(0x7000c62570, fn_7000c62570 as *const (), 0), // +0x576570 sub_576570
    Lifted::new(0x7000c62640, fn_7000c62640 as *const (), 0), // +0x576640 sub_576640
    Lifted::new(0x7000c62700, fn_7000c62700 as *const (), 0), // +0x576700 sub_576700
    Lifted::new(0x7000c627e0, fn_7000c627e0 as *const (), 0), // +0x5767e0 sub_5767e0
    Lifted::new(0x7000c62940, fn_7000c62940 as *const (), 0), // +0x576940 sub_576940
    Lifted::new(0x7000c62ac0, fn_7000c62ac0 as *const (), 0), // +0x576ac0 sub_576ac0
    Lifted::new(0x7000c62b10, fn_7000c62b10 as *const (), 0), // +0x576b10 sub_576b10
    Lifted::new(0x7000c62bd0, fn_7000c62bd0 as *const (), 0), // +0x576bd0 sub_576bd0
    Lifted::new(0x7000c62d30, tpl_143::<0x8e0, 0x7000bef9f0> as *const (), 0), // +0x576d30 sub_576d30
    Lifted::new(0x7000c62d40, fn_7000c62d40 as *const (), 1), // +0x576d40 sub_576d40
    Lifted::new(0x7000c62d90, fn_7000c62d90 as *const (), 1), // +0x576d90 sub_576d90
    Lifted::new(0x7000c62da0, tpl_15 as *const (), 1), // +0x576da0 sub_576da0
    Lifted::new(0x7000c62db0, fn_7000c62db0 as *const (), 0), // +0x576db0 sub_576db0
    Lifted::new(0x7000c62e70, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c62f77, 0x7000c403a0, 0x70007f0910, 0x7000c62f93> as *const (), 0), // +0x576e70 sub_576e70
    Lifted::new(0x7000c62fe0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c630e7, 0x7000c40450, 0x70007f0910, 0x7000c63103> as *const (), 0), // +0x576fe0 sub_576fe0
    Lifted::new(0x7000c63150, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c63257, 0x7000c40500, 0x70007f0910, 0x7000c63273> as *const (), 0), // +0x577150 sub_577150
    Lifted::new(0x7000c632c0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c633c7, 0x7000c405b0, 0x70007f0910, 0x7000c633e3> as *const (), 0), // +0x5772c0 sub_5772c0
    Lifted::new(0x7000c63430, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c63537, 0x7000c40660, 0x70007f0910, 0x7000c63553> as *const (), 0), // +0x577430 sub_577430
    Lifted::new(0x7000c635a0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c636a7, 0x7000c40710, 0x70007f0910, 0x7000c636c3> as *const (), 0), // +0x5775a0 sub_5775a0
    Lifted::new(0x7000c63710, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c63817, 0x7000c407c0, 0x70007f0910, 0x7000c63833> as *const (), 0), // +0x577710 sub_577710
    Lifted::new(0x7000c63880, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c63987, 0x7000c40870, 0x70007f0910, 0x7000c639a3> as *const (), 0), // +0x577880 sub_577880
    Lifted::new(0x7000c639f0, tpl_914::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000c63a77> as *const (), 0), // +0x5779f0 sub_5779f0
    Lifted::new(0x7000c63aa0, tpl_918::<0x70018237e8, 0x8> as *const (), 3), // +0x577aa0 sub_577aa0
    Lifted::new(0x7000c63ac0, tpl_923::<0x7001823828> as *const (), 1), // +0x577ac0 sub_577ac0
    Lifted::new(0x7000c63ae0, tpl_980::<0x7001823868, 0x8, 0x18> as *const (), 3), // +0x577ae0 sub_577ae0
    Lifted::new(0x7000c63b10, tpl_919::<0x700177c312> as *const (), 0), // +0x577b10 sub_577b10
    Lifted::new(0x7000c63b30, tpl_754::<0x700177c312> as *const (), 0), // +0x577b30 sub_577b30
    Lifted::new(0x7000c63b50, tpl_984::<0x700177c312> as *const (), 0), // +0x577b50 sub_577b50
    Lifted::new(0x7000c63b70, fn_7000c63b70 as *const (), 1), // +0x577b70 sub_577b70
    Lifted::new(0x7000c63bb0, fn_7000c63bb0 as *const (), 0), // +0x577bb0 sub_577bb0
    Lifted::new(0x7000c63c10, fn_7000c63c10 as *const (), 0), // +0x577c10 sub_577c10
    Lifted::new(0x7000c63db0, tpl_284::<0x7000c63150, 0x70017d37a0, 0x70017790e0, 0x7000c63f6d> as *const (), 0), // +0x577db0 sub_577db0
    Lifted::new(0x7000c63fe0, tpl_1159::<0x7000c632c0, 0x70017d37a0, 0x70017790e0, 0x7000c641ae> as *const (), 0), // +0x577fe0 sub_577fe0
    Lifted::new(0x7000c64230, tpl_1159::<0x7000c63430, 0x70017d37a0, 0x70017790e0, 0x7000c643fe> as *const (), 0), // +0x578230 sub_578230
    Lifted::new(0x7000c64480, tpl_1159::<0x7000c635a0, 0x70017d37a0, 0x70017790e0, 0x7000c6464e> as *const (), 0), // +0x578480 sub_578480
    Lifted::new(0x7000c646d0, tpl_1159::<0x7000c63710, 0x70017d37a0, 0x70017790e0, 0x7000c6489e> as *const (), 0), // +0x5786d0 sub_5786d0
    Lifted::new(0x7000c64920, tpl_286::<0x7000c63880, 0x70017d37a0, 0x70017790e0, 0x7000c64b0b> as *const (), 0), // +0x578920 sub_578920
    Lifted::new(0x7000c64b90, tpl_286::<0x7000c62e70, 0x70017d37a0, 0x70017790e0, 0x7000c64d7b> as *const (), 0), // +0x578b90 sub_578b90
    Lifted::new(0x7000c64e00, tpl_1159::<0x7000c62fe0, 0x70017d37a0, 0x70017790e0, 0x7000c64fce> as *const (), 0), // +0x578e00 sub_578e00
    Lifted::new(0x7000c65050, tpl_52::<0x8> as *const (), 1), // +0x579050 sub_579050
    Lifted::new(0x7000c65060, tpl_52::<0x8> as *const (), 1), // +0x579060 sub_579060
    Lifted::new(0x7000c65070, tpl_52::<0x8> as *const (), 1), // +0x579070 sub_579070
    Lifted::new(0x7000c65080, tpl_287 as *const (), 1), // +0x579080 sub_579080
    Lifted::new(0x7000c653a0, tpl_918::<0x70018237e8, 0x8> as *const (), 3), // +0x5793a0 sub_5793a0
    Lifted::new(0x7000c653c0, tpl_923::<0x7001823828> as *const (), 1), // +0x5793c0 sub_5793c0
    Lifted::new(0x7000c653e0, tpl_980::<0x7001823868, 0x8, 0x18> as *const (), 3), // +0x5793e0 sub_5793e0
    Lifted::new(0x7000c65410, tpl_897::<0x10, 0x38, 0x700177c170, 0x70017d37ec, 0x7000c63fe0> as *const (), 2), // +0x579410 sub_579410
    Lifted::new(0x7000c654f0, tpl_315::<0x7001c6bc90> as *const (), 1), // +0x5794f0 sub_5794f0
    Lifted::new(0x7000c65500, tpl_315::<0x7001c6bc40> as *const (), 1), // +0x579500 sub_579500
    Lifted::new(0x7000c65510, tpl_315::<0x7001c6bbf0> as *const (), 1), // +0x579510 sub_579510
    Lifted::new(0x7000c65520, tpl_1000::<0x700177c312> as *const (), 0), // +0x579520 sub_579520
    Lifted::new(0x7000c657d0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c657e0> as *const (), 0), // +0x5797d0 sub_5797d0
    Lifted::new(0x7000c657f0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c65800> as *const (), 0), // +0x5797f0 sub_5797f0
    Lifted::new(0x7000c65810, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c65820> as *const (), 0), // +0x579810 sub_579810
    Lifted::new(0x7000c65830, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c65840> as *const (), 0), // +0x579830 sub_579830
    Lifted::new(0x7000c65850, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c65860> as *const (), 0), // +0x579850 sub_579850
    Lifted::new(0x7000c65870, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c65880> as *const (), 0), // +0x579870 sub_579870
    Lifted::new(0x7000c65890, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c658a0> as *const (), 0), // +0x579890 sub_579890
    Lifted::new(0x7000c658b0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c658c0> as *const (), 0), // +0x5798b0 sub_5798b0
    Lifted::new(0x7000c658d0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c658e0> as *const (), 0), // +0x5798d0 sub_5798d0
    Lifted::new(0x7000c658f0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c65900> as *const (), 0), // +0x5798f0 sub_5798f0
    Lifted::new(0x7000c65910, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000c65964, 0x70007f0910, 0x7000c6597f> as *const (), 0), // +0x579910 sub_579910
    Lifted::new(0x7000c659a0, fn_7000c659a0 as *const (), 0), // +0x5799a0 sub_5799a0
    Lifted::new(0x7000c65ab0, fn_7000c65ab0 as *const (), 1), // +0x579ab0 sub_579ab0
    Lifted::new(0x7000c65ad0, fn_7000c65ad0 as *const (), 1), // +0x579ad0 sub_579ad0
    Lifted::new(0x7000c65af0, tpl_990::<0x10, 0x8, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x579af0 sub_579af0
    Lifted::new(0x7000c65bb0, tpl_990::<0x10, 0x8, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x579bb0 sub_579bb0
    Lifted::new(0x7000c65c70, tpl_990::<0x10, 0x8, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x579c70 sub_579c70
    Lifted::new(0x7000c65d30, tpl_320::<0x10, 0x38, 0x8, 0x7000c65520, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x579d30 sub_579d30
    Lifted::new(0x7000c65e20, fn_7000c65e20 as *const (), 0), // +0x579e20 sub_579e20
    Lifted::new(0x7000c65ed0, tpl_36::<0x48> as *const (), 1), // +0x579ed0 sub_579ed0
    Lifted::new(0x7000c65ee0, fn_7000c65ee0 as *const (), 1), // +0x579ee0 sub_579ee0
    Lifted::new(0x7000c65f00, fn_7000c65f00 as *const (), 1), // +0x579f00 sub_579f00
    Lifted::new(0x7000c65f20, tpl_30 as *const (), 1), // +0x579f20 sub_579f20
    Lifted::new(0x7000c65f30, tpl_52::<0xffffffffffff6898> as *const (), 1), // +0x579f30 sub_579f30
    Lifted::new(0x7000c65f40, fn_7000c65f40 as *const (), 1), // +0x579f40 sub_579f40
    Lifted::new(0x7000c65f80, fn_7000c65f80 as *const (), 2), // +0x579f80 sub_579f80
    Lifted::new(0x7000c66040, fn_7000c66040 as *const (), 0), // +0x57a040 sub_57a040
    Lifted::new(0x7000c660d0, fn_7000c660d0 as *const (), 1), // +0x57a0d0 sub_57a0d0
    Lifted::new(0x7000c660e0, fn_7000c660e0 as *const (), 0), // +0x57a0e0 sub_57a0e0
    Lifted::new(0x7000c661c0, tpl_899::<0x7000c661d0> as *const (), 0), // +0x57a1c0 sub_57a1c0
    Lifted::new(0x7000c661d0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x57a1d0 sub_57a1d0
    Lifted::new(0x7000c66210, fn_7000c66210 as *const (), 2), // +0x57a210 sub_57a210
    Lifted::new(0x7000c66250, fn_7000c66250 as *const (), 2), // +0x57a250 sub_57a250
    Lifted::new(0x7000c66500, fn_7000c66500 as *const (), 2), // +0x57a500 sub_57a500
    Lifted::new(0x7000c66780, tpl_281::<0x7000c66500> as *const (), 0), // +0x57a780 sub_57a780
    Lifted::new(0x7000c66790, fn_7000c66790 as *const (), 2), // +0x57a790 sub_57a790
    Lifted::new(0x7000c66800, fn_7000c66800 as *const (), 2), // +0x57a800 sub_57a800
    Lifted::new(0x7000c668f0, tpl_1160 as *const (), 1), // +0x57a8f0 sub_57a8f0
    Lifted::new(0x7000c66a30, tpl_1160 as *const (), 1), // +0x57aa30 sub_57aa30
    Lifted::new(0x7000c66b70, fn_7000c66b70 as *const (), 0), // +0x57ab70 sub_57ab70
    Lifted::new(0x7000c66d00, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c66da3> as *const (), 0), // +0x57ad00 sub_57ad00
    Lifted::new(0x7000c66dd0, tpl_33::<0x700177c312> as *const (), 0), // +0x57add0 sub_57add0
    Lifted::new(0x7000c66e00, tpl_33::<0x700177c312> as *const (), 0), // +0x57ae00 sub_57ae00
    Lifted::new(0x7000c66e30, fn_7000c66e30 as *const (), 1), // +0x57ae30 sub_57ae30
    Lifted::new(0x7000c66e60, fn_7000c66e60 as *const (), 2), // +0x57ae60 sub_57ae60
    Lifted::new(0x7000c672c0, fn_7000c672c0 as *const (), 2), // +0x57b2c0 sub_57b2c0
    Lifted::new(0x7000c67740, fn_7000c67740 as *const (), 0), // +0x57b740 sub_57b740
    Lifted::new(0x7000c677a0, fn_7000c677a0 as *const (), 0), // +0x57b7a0 sub_57b7a0
    Lifted::new(0x7000c67860, fn_7000c67860 as *const (), 2), // +0x57b860 sub_57b860
    Lifted::new(0x7000c678d0, fn_7000c678d0 as *const (), 2), // +0x57b8d0 sub_57b8d0
    Lifted::new(0x7000c67d60, fn_7000c67d60 as *const (), 0), // +0x57bd60 sub_57bd60
    Lifted::new(0x7000c67f70, fn_7000c67f70 as *const (), 1), // +0x57bf70 sub_57bf70
    Lifted::new(0x7000c67f80, fn_7000c67f80 as *const (), 0), // +0x57bf80 sub_57bf80
    Lifted::new(0x7000c67fc0, fn_7000c67fc0 as *const (), 0), // +0x57bfc0 sub_57bfc0
    Lifted::new(0x7000c68010, fn_7000c68010 as *const (), 0), // +0x57c010 sub_57c010
    Lifted::new(0x7000c680b0, tpl_1125::<0x7000737df0, 0x700075dbe0> as *const (), 0), // +0x57c0b0 sub_57c0b0
    Lifted::new(0x7000c680e0, fn_7000c680e0 as *const (), 2), // +0x57c0e0 sub_57c0e0
    Lifted::new(0x7000c68390, fn_7000c68390 as *const (), 0), // +0x57c390 sub_57c390
    Lifted::new(0x7000c68400, fn_7000c68400 as *const (), 0), // +0x57c400 sub_57c400
    Lifted::new(0x7000c68440, fn_7000c68440 as *const (), 2), // +0x57c440 sub_57c440
    Lifted::new(0x7000c68540, fn_7000c68540 as *const (), 0), // +0x57c540 sub_57c540
    Lifted::new(0x7000c68590, fn_7000c68590 as *const (), 2), // +0x57c590 sub_57c590
    Lifted::new(0x7000c68a50, fn_7000c68a50 as *const (), 2), // +0x57ca50 sub_57ca50
    Lifted::new(0x7000c68e20, fn_7000c68e20 as *const (), 0), // +0x57ce20 sub_57ce20
    Lifted::new(0x7000c68f70, fn_7000c68f70 as *const (), 0), // +0x57cf70 sub_57cf70
    Lifted::new(0x7000c69060, fn_7000c69060 as *const (), 0), // +0x57d060 sub_57d060
    Lifted::new(0x7000c69100, fn_7000c69100 as *const (), 2), // +0x57d100 sub_57d100
    Lifted::new(0x7000c69380, fn_7000c69380 as *const (), 0), // +0x57d380 sub_57d380
    Lifted::new(0x7000c69560, fn_7000c69560 as *const (), 0), // +0x57d560 sub_57d560
    Lifted::new(0x7000c69620, fn_7000c69620 as *const (), 0), // +0x57d620 sub_57d620
    Lifted::new(0x7000c696b0, fn_7000c696b0 as *const (), 0), // +0x57d6b0 sub_57d6b0
    Lifted::new(0x7000c69a90, fn_7000c69a90 as *const (), 0), // +0x57da90 sub_57da90
    Lifted::new(0x7000c69b70, fn_7000c69b70 as *const (), 2), // +0x57db70 sub_57db70
    Lifted::new(0x7000c69c90, fn_7000c69c90 as *const (), 1), // +0x57dc90 sub_57dc90
    Lifted::new(0x7000c69cd0, fn_7000c69cd0 as *const (), 1), // +0x57dcd0 sub_57dcd0
    Lifted::new(0x7000c69d90, fn_7000c69d90 as *const (), 2), // +0x57dd90 sub_57dd90
    Lifted::new(0x7000c6a380, fn_7000c6a380 as *const (), 0), // +0x57e380 sub_57e380
    Lifted::new(0x7000c6a430, fn_7000c6a430 as *const (), 2), // +0x57e430 sub_57e430
    Lifted::new(0x7000c6a850, fn_7000c6a850 as *const (), 2), // +0x57e850 sub_57e850
    Lifted::new(0x7000c6ac80, fn_7000c6ac80 as *const (), 2), // +0x57ec80 sub_57ec80
    Lifted::new(0x7000c6ad80, fn_7000c6ad80 as *const (), 0), // +0x57ed80 sub_57ed80
    Lifted::new(0x7000c6adf0, fn_7000c6adf0 as *const (), 0), // +0x57edf0 sub_57edf0
    Lifted::new(0x7000c6aea0, fn_7000c6aea0 as *const (), 2), // +0x57eea0 sub_57eea0
    Lifted::new(0x7000c6b080, fn_7000c6b080 as *const (), 1), // +0x57f080 sub_57f080
    Lifted::new(0x7000c6b0c0, fn_7000c6b0c0 as *const (), 2), // +0x57f0c0 sub_57f0c0
    Lifted::new(0x7000c6b2f0, fn_7000c6b2f0 as *const (), 0), // +0x57f2f0 sub_57f2f0
    Lifted::new(0x7000c6b300, fn_7000c6b300 as *const (), 0), // +0x57f300 sub_57f300
    Lifted::new(0x7000c6b420, tpl_47::<0x5a> as *const (), 1), // +0x57f420 sub_57f420
    Lifted::new(0x7000c6b430, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c6b537, 0x700098cb30, 0x70007f0910, 0x7000c6b553> as *const (), 0), // +0x57f430 sub_57f430
    Lifted::new(0x7000c6b5a0, tpl_296::<0x700177c312> as *const (), 0), // +0x57f5a0 sub_57f5a0
    Lifted::new(0x7000c6b850, tpl_321::<0x10, 0x38, 0x8, 0x7000c6b5a0, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x57f850 sub_57f850
    Lifted::new(0x7000c6b940, tpl_55::<0x700177c0d0> as *const (), 0), // +0x57f940 sub_57f940
    Lifted::new(0x7000c6b980, tpl_898::<0x700177c0e0> as *const (), 0), // +0x57f980 sub_57f980
    Lifted::new(0x7000c6b9c0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x57f9c0 sub_57f9c0
    Lifted::new(0x7000c6ba00, tpl_392::<0x700177c0e0> as *const (), 0), // +0x57fa00 sub_57fa00
    Lifted::new(0x7000c6ba30, tpl_392::<0x700177c0e0> as *const (), 0), // +0x57fa30 sub_57fa30
    Lifted::new(0x7000c6ba60, tpl_55::<0x700177c0d0> as *const (), 0), // +0x57fa60 sub_57fa60
    Lifted::new(0x7000c6baa0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x57faa0 sub_57faa0
    Lifted::new(0x7000c6bae0, fn_7000c6bae0 as *const (), 0), // +0x57fae0 sub_57fae0
    Lifted::new(0x7000c6bd10, tpl_603::<0x20, 0xffffffffffffffff, 0x18, 0x10, 0x8, 0x700177c312, 0xfffffffffffffff8, 0x7001d54968, 0x7000c6bdd3> as *const (), 0), // +0x57fd10 sub_57fd10
    Lifted::new(0x7000c6be10, tpl_603::<0x20, 0xffffffffffffffff, 0x18, 0x10, 0x8, 0x700177c312, 0xfffffffffffffff8, 0x7001d54968, 0x7000c6bed3> as *const (), 0), // +0x57fe10 sub_57fe10
    Lifted::new(0x7000c6bf10, fn_7000c6bf10 as *const (), 2), // +0x57ff10 sub_57ff10
    Lifted::new(0x7000c6c0e0, fn_7000c6c0e0 as *const (), 0), // +0x5800e0 sub_5800e0
    Lifted::new(0x7000c6c200, tpl_988::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000c6c3c5, 0x70007f0910, 0x7000c6c3cb, 0x7000c6c6a0, 0x7000c6c3d4> as *const (), 0), // +0x580200 sub_580200
    Lifted::new(0x7000c6c450, tpl_900::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000c6c618, 0x70007f0910, 0x7000c6c61e, 0x7000c6c6c0, 0x7000c6c627> as *const (), 0), // +0x580450 sub_580450
    Lifted::new(0x7000c6c6a0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000c6c6b0> as *const (), 0), // +0x5806a0 sub_5806a0
    Lifted::new(0x7000c6c6c0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000c6c6d0> as *const (), 0), // +0x5806c0 sub_5806c0
    Lifted::new(0x7000c6c6e0, fn_7000c6c6e0 as *const (), 2), // +0x5806e0 sub_5806e0
    Lifted::new(0x7000c6c7a0, fn_7000c6c7a0 as *const (), 0), // +0x5807a0 sub_5807a0
    Lifted::new(0x7000c6c870, fn_7000c6c870 as *const (), 0), // +0x580870 sub_580870
    Lifted::new(0x7000c6c9b0, tpl_1161::<0x10, 0x19, 0x8> as *const (), 1), // +0x5809b0 sub_5809b0
    Lifted::new(0x7000c6ca30, fn_7000c6ca30 as *const (), 0), // +0x580a30 sub_580a30
    Lifted::new(0x7000c6cb80, fn_7000c6cb80 as *const (), 0), // +0x580b80 sub_580b80
    Lifted::new(0x7000c6ccf0, fn_7000c6ccf0 as *const (), 1), // +0x580cf0 sub_580cf0
    Lifted::new(0x7000c6cd20, fn_7000c6cd20 as *const (), 0), // +0x580d20 sub_580d20
    Lifted::new(0x7000c6cd60, fn_7000c6cd60 as *const (), 0), // +0x580d60 sub_580d60
    Lifted::new(0x7000c6ce20, tpl_1162::<0x98, 0x7000709d30> as *const (), 0), // +0x580e20 sub_580e20
    Lifted::new(0x7000c6ce50, tpl_1162::<0x98, 0x7000773450> as *const (), 0), // +0x580e50 sub_580e50
    Lifted::new(0x7000c6ce80, tpl_1162::<0xa0, 0x7000773450> as *const (), 0), // +0x580e80 sub_580e80
    Lifted::new(0x7000c6ceb0, fn_7000c6ceb0 as *const (), 0), // +0x580eb0 sub_580eb0
    Lifted::new(0x7000c6d330, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x581330 sub_581330
    Lifted::new(0x7000c6d3a0, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x5813a0 sub_5813a0
    Lifted::new(0x7000c6d410, tpl_65::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c6fe60, 0x70017d37c0, 0x70017790e0, 0x7000c6d6fa> as *const (), 2), // +0x581410 sub_581410
    Lifted::new(0x7000c6d7c0, tpl_68 as *const (), 1), // +0x5817c0 sub_5817c0
    Lifted::new(0x7000c6d8a0, fn_7000c6d8a0 as *const (), 2), // +0x5818a0 sub_5818a0
    Lifted::new(0x7000c6d9f0, fn_7000c6d9f0 as *const (), 0), // +0x5819f0 sub_5819f0
    Lifted::new(0x7000c6dae0, fn_7000c6dae0 as *const (), 1), // +0x581ae0 sub_581ae0
    Lifted::new(0x7000c6db00, fn_7000c6db00 as *const (), 0), // +0x581b00 sub_581b00
    Lifted::new(0x7000c6db50, tpl_88::<0x700074e6c0, 0x70018242d8> as *const (), 0), // +0x581b50 sub_581b50
    Lifted::new(0x7000c6db80, fn_7000c6db80 as *const (), 0), // +0x581b80 sub_581b80
    Lifted::new(0x7000c6dbb0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c6dc53> as *const (), 0), // +0x581bb0 sub_581bb0
    Lifted::new(0x7000c6dc80, fn_7000c6dc80 as *const (), 0), // +0x581c80 sub_581c80
    Lifted::new(0x7000c6dd10, tpl_33::<0x7000701460> as *const (), 0), // +0x581d10 sub_581d10
    Lifted::new(0x7000c6dd40, tpl_135::<0x70017d3198, 0x7000780b90, 0x108, 0x7001d550c0, 0xf8, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x581d40 sub_581d40
    Lifted::new(0x7000c6ddc0, tpl_33::<0x700177c312> as *const (), 0), // +0x581dc0 sub_581dc0
    Lifted::new(0x7000c6ddf0, tpl_135::<0x70017d3198, 0x7000780b90, 0x108, 0x7001d550c0, 0xf8, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x581df0 sub_581df0
    Lifted::new(0x7000c6de70, fn_7000c6de70 as *const (), 0), // +0x581e70 sub_581e70
    Lifted::new(0x7000c6df00, fn_7000c6df00 as *const (), 0), // +0x581f00 sub_581f00
    Lifted::new(0x7000c6e030, fn_7000c6e030 as *const (), 0), // +0x582030 sub_582030
    Lifted::new(0x7000c6e0d0, tpl_177::<0x7001824298, 0x7001d54ef8, 0x7000c6e0e1> as *const (), 0), // +0x5820d0 sub_5820d0
    Lifted::new(0x7000c6e0f0, fn_7000c6e0f0 as *const (), 0), // +0x5820f0 sub_5820f0
    Lifted::new(0x7000c6e1c0, tpl_179::<0x7000c6e400> as *const (), 0), // +0x5821c0 sub_5821c0
    Lifted::new(0x7000c6e1f0, fn_7000c6e1f0 as *const (), 0), // +0x5821f0 sub_5821f0
    Lifted::new(0x7000c6e400, fn_7000c6e400 as *const (), 0), // +0x582400 sub_582400
    Lifted::new(0x7000c6e6c0, tpl_264::<0x7000c6e400> as *const (), 0), // +0x5826c0 sub_5826c0
    Lifted::new(0x7000c6e6e0, fn_7000c6e6e0 as *const (), 0), // +0x5826e0 sub_5826e0
    Lifted::new(0x7000c6e950, tpl_196 as *const (), 1), // +0x582950 sub_582950
    Lifted::new(0x7000c6e960, fn_7000c6e960 as *const (), 0), // +0x582960 sub_582960
    Lifted::new(0x7000c6e990, fn_7000c6e990 as *const (), 0), // +0x582990 sub_582990
    Lifted::new(0x7000c6eb80, fn_7000c6eb80 as *const (), 0), // +0x582b80 sub_582b80
    Lifted::new(0x7000c6eec0, tpl_771 as *const (), 0), // +0x582ec0 sub_582ec0
    Lifted::new(0x7000c6eed0, fn_7000c6eed0 as *const (), 1), // +0x582ed0 sub_582ed0
    Lifted::new(0x7000c6eef0, tpl_159::<0x40> as *const (), 1), // +0x582ef0 sub_582ef0
    Lifted::new(0x7000c6ef10, fn_7000c6ef10 as *const (), 1), // +0x582f10 sub_582f10
    Lifted::new(0x7000c6ef40, fn_7000c6ef40 as *const (), 0), // +0x582f40 sub_582f40
    Lifted::new(0x7000c6f0b0, fn_7000c6f0b0 as *const (), 1), // +0x5830b0 sub_5830b0
    Lifted::new(0x7000c6f0f0, tpl_177::<0x7001824298, 0x7001d54ef8, 0x7000c6f101> as *const (), 0), // +0x5830f0 sub_5830f0
    Lifted::new(0x7000c6f110, fn_7000c6f110 as *const (), 0), // +0x583110 sub_583110
    Lifted::new(0x7000c6f2b0, fn_7000c6f2b0 as *const (), 0), // +0x5832b0 sub_5832b0
    Lifted::new(0x7000c6f500, fn_7000c6f500 as *const (), 0), // +0x583500 sub_583500
    Lifted::new(0x7000c6f590, fn_7000c6f590 as *const (), 0), // +0x583590 sub_583590
    Lifted::new(0x7000c6f600, tpl_177::<0x7001824298, 0x7001d54ef8, 0x7000c6f611> as *const (), 0), // +0x583600 sub_583600
    Lifted::new(0x7000c6f620, tpl_177::<0x7001824258, 0x7001d54ef8, 0x7000c6f631> as *const (), 0), // +0x583620 sub_583620
    Lifted::new(0x7000c6f640, fn_7000c6f640 as *const (), 0), // +0x583640 sub_583640
    Lifted::new(0x7000c6f690, fn_7000c6f690 as *const (), 0), // +0x583690 sub_583690
    Lifted::new(0x7000c6f720, fn_7000c6f720 as *const (), 1), // +0x583720 sub_583720
    Lifted::new(0x7000c6f760, fn_7000c6f760 as *const (), 0), // +0x583760 sub_583760
    Lifted::new(0x7000c6f800, fn_7000c6f800 as *const (), 0), // +0x583800 sub_583800
    Lifted::new(0x7000c6fa00, fn_7000c6fa00 as *const (), 0), // +0x583a00 sub_583a00
    Lifted::new(0x7000c6fa60, fn_7000c6fa60 as *const (), 1), // +0x583a60 sub_583a60
    Lifted::new(0x7000c6fb20, tpl_177::<0x7001824298, 0x7001d54ef8, 0x7000c6fb31> as *const (), 0), // +0x583b20 sub_583b20
    Lifted::new(0x7000c6fb40, fn_7000c6fb40 as *const (), 0), // +0x583b40 sub_583b40
    Lifted::new(0x7000c6fb80, fn_7000c6fb80 as *const (), 0), // +0x583b80 sub_583b80
    Lifted::new(0x7000c6fc90, fn_7000c6fc90 as *const (), 0), // +0x583c90 sub_583c90
    Lifted::new(0x7000c6fcf0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c6fdf7, 0x7000c6d7c0, 0x70007f0910, 0x7000c6fe13> as *const (), 0), // +0x583cf0 sub_583cf0
    Lifted::new(0x7000c6fe60, tpl_286::<0x7000c6fcf0, 0x70017d37a0, 0x70017790e0, 0x7000c7004b> as *const (), 0), // +0x583e60 sub_583e60
    Lifted::new(0x7000c700d0, tpl_37::<0x80> as *const (), 1), // +0x5840d0 sub_5840d0
    Lifted::new(0x7000c700f0, tpl_15 as *const (), 1), // +0x5840f0 sub_5840f0
    Lifted::new(0x7000c70100, fn_7000c70100 as *const (), 0), // +0x584100 sub_584100
    Lifted::new(0x7000c70150, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c70328, 0x7000c714a0, 0x7000c7032e, 0x70007f0910, 0x7000c70334> as *const (), 2), // +0x584150 sub_584150
    Lifted::new(0x7000c703b0, fn_7000c703b0 as *const (), 0), // +0x5843b0 sub_5843b0
    Lifted::new(0x7000c705d0, fn_7000c705d0 as *const (), 0), // +0x5845d0 sub_5845d0
    Lifted::new(0x7000c70780, fn_7000c70780 as *const (), 0), // +0x584780 sub_584780
    Lifted::new(0x7000c707d0, fn_7000c707d0 as *const (), 0), // +0x5847d0 sub_5847d0
    Lifted::new(0x7000c70850, fn_7000c70850 as *const (), 1), // +0x584850 sub_584850
    Lifted::new(0x7000c70880, tpl_803::<0x30> as *const (), 1), // +0x584880 sub_584880
    Lifted::new(0x7000c708a0, tpl_15 as *const (), 1), // +0x5848a0 sub_5848a0
    Lifted::new(0x7000c708b0, fn_7000c708b0 as *const (), 0), // +0x5848b0 sub_5848b0
    Lifted::new(0x7000c70950, tpl_1163::<0x7000bf3630> as *const (), 0), // +0x584950 sub_584950
    Lifted::new(0x7000c70960, fn_7000c70960 as *const (), 2), // +0x584960 sub_584960
    Lifted::new(0x7000c709c0, fn_7000c709c0 as *const (), 1), // +0x5849c0 sub_5849c0
    Lifted::new(0x7000c709e0, tpl_1163::<0x7000bf3650> as *const (), 0), // +0x5849e0 sub_5849e0
    Lifted::new(0x7000c709f0, tpl_1163::<0x7000bf3660> as *const (), 0), // +0x5849f0 sub_5849f0
    Lifted::new(0x7000c70a00, fn_7000c70a00 as *const (), 0), // +0x584a00 sub_584a00
    Lifted::new(0x7000c70ac0, fn_7000c70ac0 as *const (), 0), // +0x584ac0 sub_584ac0
    Lifted::new(0x7000c70d80, fn_7000c70d80 as *const (), 1), // +0x584d80 sub_584d80
    Lifted::new(0x7000c70dd0, fn_7000c70dd0 as *const (), 2), // +0x584dd0 sub_584dd0
    Lifted::new(0x7000c710b0, fn_7000c710b0 as *const (), 0), // +0x5850b0 sub_5850b0
    Lifted::new(0x7000c71180, tpl_1163::<0x7000bf3ce0> as *const (), 0), // +0x585180 sub_585180
    Lifted::new(0x7000c71190, fn_7000c71190 as *const (), 2), // +0x585190 sub_585190
    Lifted::new(0x7000c714a0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000c714b0> as *const (), 0), // +0x5854a0 sub_5854a0
    Lifted::new(0x7000c714c0, fn_7000c714c0 as *const (), 0), // +0x5854c0 sub_5854c0
    Lifted::new(0x7000c72300, fn_7000c72300 as *const (), 0), // +0x586300 sub_586300
    Lifted::new(0x7000c73490, tpl_14 as *const (), 1), // +0x587490 sub_587490
    Lifted::new(0x7000c734a0, tpl_14 as *const (), 1), // +0x5874a0 sub_5874a0
    Lifted::new(0x7000c734b0, tpl_14 as *const (), 1), // +0x5874b0 sub_5874b0
    Lifted::new(0x7000c734c0, fn_7000c734c0 as *const (), 1), // +0x5874c0 sub_5874c0
    Lifted::new(0x7000c735c0, fn_7000c735c0 as *const (), 1), // +0x5875c0 sub_5875c0
    Lifted::new(0x7000c736c0, tpl_1164::<0x7000c87110> as *const (), 0), // +0x5876c0 sub_5876c0
    Lifted::new(0x7000c73850, tpl_1164::<0x7000c87110> as *const (), 0), // +0x587850 sub_587850
    Lifted::new(0x7000c739e0, tpl_1164::<0x7000c87110> as *const (), 0), // +0x5879e0 sub_5879e0
    Lifted::new(0x7000c73b70, tpl_1164::<0x7000c87110> as *const (), 0), // +0x587b70 sub_587b70
    Lifted::new(0x7000c73d00, tpl_1164::<0x7000c87110> as *const (), 0), // +0x587d00 sub_587d00
    Lifted::new(0x7000c73e90, fn_7000c73e90 as *const (), 2), // +0x587e90 sub_587e90
    Lifted::new(0x7000c74570, fn_7000c74570 as *const (), 2), // +0x588570 sub_588570
    Lifted::new(0x7000c74bf0, fn_7000c74bf0 as *const (), 0), // +0x588bf0 sub_588bf0
    Lifted::new(0x7000c74c40, fn_7000c74c40 as *const (), 2), // +0x588c40 sub_588c40
    Lifted::new(0x7000c74d10, tpl_1165::<0x7000796870, 0x7000c9b160, 0x7000701460> as *const (), 0), // +0x588d10 sub_588d10
    Lifted::new(0x7000c75040, tpl_1166::<0x7000796870, 0x7000701460> as *const (), 0), // +0x589040 sub_589040
    Lifted::new(0x7000c752f0, tpl_1167::<0x7000796870, 0x7000c734c0, 0x7000701460> as *const (), 0), // +0x5892f0 sub_5892f0
    Lifted::new(0x7000c75600, tpl_1166::<0x7000796870, 0x7000701460> as *const (), 0), // +0x589600 sub_589600
    Lifted::new(0x7000c758b0, tpl_1167::<0x7000796870, 0x7000c734c0, 0x7000701460> as *const (), 0), // +0x5898b0 sub_5898b0
    Lifted::new(0x7000c75bc0, fn_7000c75bc0 as *const (), 2), // +0x589bc0 sub_589bc0
    Lifted::new(0x7000c75f50, tpl_1167::<0x7000796870, 0x7000c735c0, 0x7000701460> as *const (), 0), // +0x589f50 sub_589f50
    Lifted::new(0x7000c76260, tpl_1165::<0x7000796870, 0x7000c9b160, 0x7000701460> as *const (), 0), // +0x58a260 sub_58a260
    Lifted::new(0x7000c76590, tpl_1167::<0x7000796870, 0x7000c735c0, 0x7000701460> as *const (), 0), // +0x58a590 sub_58a590
    Lifted::new(0x7000c768a0, tpl_1012::<0x7, 0x3, 0x7000c80e40> as *const (), 0), // +0x58a8a0 sub_58a8a0
    Lifted::new(0x7000c76950, tpl_1012::<0x13, 0x3, 0x7000c80e40> as *const (), 0), // +0x58a950 sub_58a950
    Lifted::new(0x7000c76a00, tpl_1012::<0x1b, 0x3, 0x7000c80e40> as *const (), 0), // +0x58aa00 sub_58aa00
    Lifted::new(0x7000c76ab0, tpl_1012::<0xf, 0xb, 0x7000c80e40> as *const (), 0), // +0x58aab0 sub_58aab0
    Lifted::new(0x7000c76b60, tpl_1013::<0x7000c76a00, 0x27, 0x1b, 0x7000c80e40> as *const (), 0), // +0x58ab60 sub_58ab60
    Lifted::new(0x7000c76c10, tpl_1013::<0x7000c76a00, 0x47, 0x1b, 0x7000c80e40> as *const (), 0), // +0x58ac10 sub_58ac10
    Lifted::new(0x7000c76cc0, tpl_1014::<0xffffffffffffffff, 0x3, 0x7000c80e40> as *const (), 0), // +0x58acc0 sub_58acc0
    Lifted::new(0x7000c76d70, tpl_1014::<0xffffffffffffffff, 0x7, 0x7000c80e40> as *const (), 0), // +0x58ad70 sub_58ad70
    Lifted::new(0x7000c76e20, tpl_1014::<0xffffffffffffffff, 0x17, 0x7000c80e40> as *const (), 0), // +0x58ae20 sub_58ae20
    Lifted::new(0x7000c76ed0, tpl_1014::<0xffffffffffffffff, 0xb, 0x7000c80e40> as *const (), 0), // +0x58aed0 sub_58aed0
    Lifted::new(0x7000c76f80, tpl_1015::<0x5, 0x7, 0x7000c80e40> as *const (), 0), // +0x58af80 sub_58af80
    Lifted::new(0x7000c77030, tpl_1016::<0x5, 0x7, 0x7000c80e40> as *const (), 0), // +0x58b030 sub_58b030
    Lifted::new(0x7000c770e0, tpl_1017::<0x9, 0xb, 0x7000c80e40> as *const (), 0), // +0x58b0e0 sub_58b0e0
    Lifted::new(0x7000c771a0, tpl_1012::<0xf, 0x3, 0x7000c80e40> as *const (), 0), // +0x58b1a0 sub_58b1a0
    Lifted::new(0x7000c77250, tpl_1018::<0x13, 0x3, 0x7000c80e40, 0x7001c72d48, 0x8, 0x10, 0x700076c500, 0x7000f405a0> as *const (), 0), // +0x58b250 sub_58b250
    Lifted::new(0x7000c77370, tpl_1019::<0x3, 0x7000c80e40> as *const (), 0), // +0x58b370 sub_58b370
    Lifted::new(0x7000c77420, tpl_1020::<0xb, 0x3, 0x7000c80e40, 0x7000c7ac00> as *const (), 0), // +0x58b420 sub_58b420
    Lifted::new(0x7000c774e0, tpl_1021::<0x7, 0xfffffffffffffff7, 0x7000c80e40> as *const (), 0), // +0x58b4e0 sub_58b4e0
    Lifted::new(0x7000c775c0, tpl_1022::<0xb, 0x3, 0x7000c80e40, 0x7000c7aef0> as *const (), 0), // +0x58b5c0 sub_58b5c0
    Lifted::new(0x7000c77680, tpl_1022::<0xb, 0x3, 0x7000c80e40, 0x7000c7aef0> as *const (), 0), // +0x58b680 sub_58b680
    Lifted::new(0x7000c77740, tpl_1012::<0x3fb, 0x3, 0x7000c80e40> as *const (), 0), // +0x58b740 sub_58b740
    Lifted::new(0x7000c777f0, tpl_1023::<0x7000c80e40, 0x7000c80f80> as *const (), 0), // +0x58b7f0 sub_58b7f0
    Lifted::new(0x7000c77950, tpl_1024::<0x7, 0xffffffffffffffff, 0xb, 0x10, 0x7000c80e40> as *const (), 0), // +0x58b950 sub_58b950
    Lifted::new(0x7000c77a10, tpl_1025::<0x7000c7aef0> as *const (), 0), // +0x58ba10 sub_58ba10
    Lifted::new(0x7000c77a60, tpl_1026::<0x13, 0xffffffffffffffff, 0x1, 0x3, 0xb, 0x7000c80e40> as *const (), 0), // +0x58ba60 sub_58ba60
    Lifted::new(0x7000c77b40, tpl_1012::<0x1b, 0xb, 0x7000c80e40> as *const (), 0), // +0x58bb40 sub_58bb40
    Lifted::new(0x7000c77bf0, tpl_1012::<0x2b, 0xb, 0x7000c80e40> as *const (), 0), // +0x58bbf0 sub_58bbf0
    Lifted::new(0x7000c77ca0, fn_7000c77ca0 as *const (), 0), // +0x58bca0 sub_58bca0
    Lifted::new(0x7000c77df0, tpl_1012::<0x23, 0xb, 0x7000c80e40> as *const (), 0), // +0x58bdf0 sub_58bdf0
    Lifted::new(0x7000c77ea0, tpl_1027::<0x7000c80e40> as *const (), 0), // +0x58bea0 sub_58bea0
    Lifted::new(0x7000c77fd0, tpl_1028::<0x7000c7aef0> as *const (), 0), // +0x58bfd0 sub_58bfd0
    Lifted::new(0x7000c78030, tpl_1014::<0xffffffffffffffff, 0x3, 0x7000c80e40> as *const (), 0), // +0x58c030 sub_58c030
    Lifted::new(0x7000c780e0, tpl_1008::<0xffffffffffffffff, 0x3, 0x7000c80e40, 0x7000c80f80> as *const (), 0), // +0x58c0e0 sub_58c0e0
    Lifted::new(0x7000c781c0, tpl_1009::<0x23, 0xf, 0x7000c80e40, 0x4, 0x7000c80f80> as *const (), 0), // +0x58c1c0 sub_58c1c0
    Lifted::new(0x7000c782e0, tpl_1010::<0x3, 0x38, 0x13, 0x7, 0x7000c80e40> as *const (), 0), // +0x58c2e0 sub_58c2e0
    Lifted::new(0x7000c783a0, tpl_1011::<0x7000c80e40> as *const (), 0), // +0x58c3a0 sub_58c3a0
    Lifted::new(0x7000c784c0, tpl_1168::<0x7, 0x3, 0x98> as *const (), 0), // +0x58c4c0 sub_58c4c0
    Lifted::new(0x7000c78570, tpl_1168::<0x1b, 0x3, 0x98> as *const (), 0), // +0x58c570 sub_58c570
    Lifted::new(0x7000c78620, tpl_1169::<0xffffffffffffffff, 0x3, 0x98> as *const (), 0), // +0x58c620 sub_58c620
    Lifted::new(0x7000c786d0, tpl_1169::<0xffffffffffffffff, 0xb, 0x98> as *const (), 0), // +0x58c6d0 sub_58c6d0
    Lifted::new(0x7000c78780, tpl_1170::<0x7, 0x3, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58c780 sub_58c780
    Lifted::new(0x7000c78820, tpl_1170::<0x13, 0x3, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58c820 sub_58c820
    Lifted::new(0x7000c788c0, tpl_1170::<0x1b, 0x3, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58c8c0 sub_58c8c0
    Lifted::new(0x7000c78960, tpl_1170::<0xf, 0xb, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58c960 sub_58c960
    Lifted::new(0x7000c78a00, tpl_1171::<0x7000c788c0, 0x27, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58ca00 sub_58ca00
    Lifted::new(0x7000c78ab0, tpl_1171::<0x7000c788c0, 0x47, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58cab0 sub_58cab0
    Lifted::new(0x7000c78b60, tpl_1172::<0xffffffffffffffff, 0x3, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58cb60 sub_58cb60
    Lifted::new(0x7000c78c10, tpl_1172::<0xffffffffffffffff, 0x7, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58cc10 sub_58cc10
    Lifted::new(0x7000c78cc0, tpl_1172::<0xffffffffffffffff, 0x17, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58ccc0 sub_58ccc0
    Lifted::new(0x7000c78d70, tpl_1172::<0xffffffffffffffff, 0xb, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58cd70 sub_58cd70
    Lifted::new(0x7000c78e20, fn_7000c78e20 as *const (), 0), // +0x58ce20 sub_58ce20
    Lifted::new(0x7000c78ed0, fn_7000c78ed0 as *const (), 0), // +0x58ced0 sub_58ced0
    Lifted::new(0x7000c78f80, fn_7000c78f80 as *const (), 0), // +0x58cf80 sub_58cf80
    Lifted::new(0x7000c79030, fn_7000c79030 as *const (), 0), // +0x58d030 sub_58d030
    Lifted::new(0x7000c79110, fn_7000c79110 as *const (), 0), // +0x58d110 sub_58d110
    Lifted::new(0x7000c79230, fn_7000c79230 as *const (), 0), // +0x58d230 sub_58d230
    Lifted::new(0x7000c792f0, fn_7000c792f0 as *const (), 0), // +0x58d2f0 sub_58d2f0
    Lifted::new(0x7000c793a0, fn_7000c793a0 as *const (), 0), // +0x58d3a0 sub_58d3a0
    Lifted::new(0x7000c794c0, tpl_1173::<0xb, 0x3, 0x8, 0x7000c9bdc0, 0x13> as *const (), 0), // +0x58d4c0 sub_58d4c0
    Lifted::new(0x7000c795b0, tpl_1173::<0xb, 0x3, 0x8, 0x7000c9bdc0, 0x13> as *const (), 0), // +0x58d5b0 sub_58d5b0
    Lifted::new(0x7000c796a0, fn_7000c796a0 as *const (), 0), // +0x58d6a0 sub_58d6a0
    Lifted::new(0x7000c79770, fn_7000c79770 as *const (), 0), // +0x58d770 sub_58d770
    Lifted::new(0x7000c79840, fn_7000c79840 as *const (), 0), // +0x58d840 sub_58d840
    Lifted::new(0x7000c799b0, fn_7000c799b0 as *const (), 0), // +0x58d9b0 sub_58d9b0
    Lifted::new(0x7000c79ac0, fn_7000c79ac0 as *const (), 0), // +0x58dac0 sub_58dac0
    Lifted::new(0x7000c79bb0, fn_7000c79bb0 as *const (), 0), // +0x58dbb0 sub_58dbb0
    Lifted::new(0x7000c79c90, fn_7000c79c90 as *const (), 0), // +0x58dc90 sub_58dc90
    Lifted::new(0x7000c79d60, fn_7000c79d60 as *const (), 0), // +0x58dd60 sub_58dd60
    Lifted::new(0x7000c79eb0, fn_7000c79eb0 as *const (), 0), // +0x58deb0 sub_58deb0
    Lifted::new(0x7000c79f60, fn_7000c79f60 as *const (), 0), // +0x58df60 sub_58df60
    Lifted::new(0x7000c7a010, fn_7000c7a010 as *const (), 0), // +0x58e010 sub_58e010
    Lifted::new(0x7000c7a100, tpl_1170::<0x1b, 0xb, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58e100 sub_58e100
    Lifted::new(0x7000c7a1a0, tpl_1170::<0x2b, 0xb, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58e1a0 sub_58e1a0
    Lifted::new(0x7000c7a240, fn_7000c7a240 as *const (), 0), // +0x58e240 sub_58e240
    Lifted::new(0x7000c7a370, tpl_1170::<0x23, 0xb, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58e370 sub_58e370
    Lifted::new(0x7000c7a410, fn_7000c7a410 as *const (), 0), // +0x58e410 sub_58e410
    Lifted::new(0x7000c7a510, fn_7000c7a510 as *const (), 0), // +0x58e510 sub_58e510
    Lifted::new(0x7000c7a5f0, tpl_1172::<0xffffffffffffffff, 0x3, 0x8, 0x7000c9bdc0> as *const (), 0), // +0x58e5f0 sub_58e5f0
    Lifted::new(0x7000c7a6a0, tpl_1029::<0x7000e87780, 0x7000c7b050, 0x7000c80e40> as *const (), 0), // +0x58e6a0 sub_58e6a0
    Lifted::new(0x7000c7a800, tpl_1174::<0x7000e87780> as *const (), 0), // +0x58e800 sub_58e800
    Lifted::new(0x7000c7a970, fn_7000c7a970 as *const (), 0), // +0x58e970 sub_58e970
    Lifted::new(0x7000c7aab0, fn_7000c7aab0 as *const (), 0), // +0x58eab0 sub_58eab0
    Lifted::new(0x7000c7ac00, tpl_1030::<0xffffffffffffffff, 0x7000c80e40, 0x7000c80f80> as *const (), 0), // +0x58ec00 sub_58ec00
    Lifted::new(0x7000c7ace0, fn_7000c7ace0 as *const (), 1), // +0x58ece0 sub_58ece0
    Lifted::new(0x7000c7ad80, fn_7000c7ad80 as *const (), 0), // +0x58ed80 sub_58ed80
    Lifted::new(0x7000c7ae30, fn_7000c7ae30 as *const (), 0), // +0x58ee30 sub_58ee30
    Lifted::new(0x7000c7aef0, tpl_1031::<0xffffffffffffffff, 0x4, 0x7000c80e40> as *const (), 0), // +0x58eef0 sub_58eef0
    Lifted::new(0x7000c7afa0, tpl_1031::<0xffffffffffffffff, 0x4, 0x7000c80e40> as *const (), 0), // +0x58efa0 sub_58efa0
    Lifted::new(0x7000c7b050, tpl_1032::<0x7000cabac0, 0xffffffffffffffff, 0x7000c80e40> as *const (), 0), // +0x58f050 sub_58f050
    Lifted::new(0x7000c7b120, tpl_1175::<0x88, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x58f120 sub_58f120
    Lifted::new(0x7000c7b1a0, fn_7000c7b1a0 as *const (), 0), // +0x58f1a0 sub_58f1a0
    Lifted::new(0x7000c7b270, fn_7000c7b270 as *const (), 0), // +0x58f270 sub_58f270
    Lifted::new(0x7000c7b330, fn_7000c7b330 as *const (), 2), // +0x58f330 sub_58f330
    Lifted::new(0x7000c7ba60, fn_7000c7ba60 as *const (), 2), // +0x58fa60 sub_58fa60
    Lifted::new(0x7000c7e490, fn_7000c7e490 as *const (), 2), // +0x592490 sub_592490
    Lifted::new(0x7000c80e40, fn_7000c80e40 as *const (), 0), // +0x594e40 sub_594e40
    Lifted::new(0x7000c80f80, tpl_1033::<0x7000bf3e50, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000bf7a00> as *const (), 0), // +0x594f80 sub_594f80
    Lifted::new(0x7000c81130, fn_7000c81130 as *const (), 0), // +0x595130 sub_595130
    Lifted::new(0x7000c813b0, fn_7000c813b0 as *const (), 0), // +0x5953b0 sub_5953b0
    Lifted::new(0x7000c815d0, fn_7000c815d0 as *const (), 0), // +0x5955d0 sub_5955d0
    Lifted::new(0x7000c81810, tpl_1034::<0x8, 0x7000bf3e50> as *const (), 0), // +0x595810 sub_595810
    Lifted::new(0x7000c81860, fn_7000c81860 as *const (), 2), // +0x595860 sub_595860
    Lifted::new(0x7000c81f60, tpl_1176::<0x8, 0xffffffffffff68d0, 0xffffffffffffffff, 0x7000763f70, 0x18, 0x7000caee20> as *const (), 0), // +0x595f60 sub_595f60
    Lifted::new(0x7000c82030, tpl_1176::<0x8, 0xffffffffffff68d0, 0xffffffffffffffff, 0x7000763f70, 0x18, 0x7000caee20> as *const (), 0), // +0x596030 sub_596030
    Lifted::new(0x7000c82100, fn_7000c82100 as *const (), 0), // +0x596100 sub_596100
    Lifted::new(0x7000c823f0, fn_7000c823f0 as *const (), 2), // +0x5963f0 sub_5963f0
    Lifted::new(0x7000c826c0, fn_7000c826c0 as *const (), 0), // +0x5966c0 sub_5966c0
    Lifted::new(0x7000c829b0, fn_7000c829b0 as *const (), 0), // +0x5969b0 sub_5969b0
    Lifted::new(0x7000c82df0, fn_7000c82df0 as *const (), 0), // +0x596df0 sub_596df0
    Lifted::new(0x7000c83230, fn_7000c83230 as *const (), 2), // +0x597230 sub_597230
    Lifted::new(0x7000c836d0, fn_7000c836d0 as *const (), 2), // +0x5976d0 sub_5976d0
    Lifted::new(0x7000c83be0, fn_7000c83be0 as *const (), 2), // +0x597be0 sub_597be0
    Lifted::new(0x7000c840d0, fn_7000c840d0 as *const (), 2), // +0x5980d0 sub_5980d0
    Lifted::new(0x7000c84570, fn_7000c84570 as *const (), 2), // +0x598570 sub_598570
    Lifted::new(0x7000c84a90, fn_7000c84a90 as *const (), 0), // +0x598a90 sub_598a90
    Lifted::new(0x7000c84bc0, fn_7000c84bc0 as *const (), 0), // +0x598bc0 sub_598bc0
    Lifted::new(0x7000c84cf0, fn_7000c84cf0 as *const (), 0), // +0x598cf0 sub_598cf0
    Lifted::new(0x7000c84db0, fn_7000c84db0 as *const (), 0), // +0x598db0 sub_598db0
    Lifted::new(0x7000c84ef0, fn_7000c84ef0 as *const (), 2), // +0x598ef0 sub_598ef0
    Lifted::new(0x7000c85400, fn_7000c85400 as *const (), 2), // +0x599400 sub_599400
    Lifted::new(0x7000c858f0, fn_7000c858f0 as *const (), 2), // +0x5998f0 sub_5998f0
    Lifted::new(0x7000c85d90, fn_7000c85d90 as *const (), 0), // +0x599d90 sub_599d90
    Lifted::new(0x7000c85ea0, fn_7000c85ea0 as *const (), 0), // +0x599ea0 sub_599ea0
    Lifted::new(0x7000c85fb0, fn_7000c85fb0 as *const (), 0), // +0x599fb0 sub_599fb0
    Lifted::new(0x7000c86150, fn_7000c86150 as *const (), 0), // +0x59a150 sub_59a150
    Lifted::new(0x7000c861f0, fn_7000c861f0 as *const (), 0), // +0x59a1f0 sub_59a1f0
    Lifted::new(0x7000c86310, fn_7000c86310 as *const (), 0), // +0x59a310 sub_59a310
    Lifted::new(0x7000c86430, fn_7000c86430 as *const (), 0), // +0x59a430 sub_59a430
    Lifted::new(0x7000c864d0, fn_7000c864d0 as *const (), 0), // +0x59a4d0 sub_59a4d0
    Lifted::new(0x7000c86540, tpl_55::<0x700177c0d0> as *const (), 0), // +0x59a540 sub_59a540
    Lifted::new(0x7000c86580, tpl_55::<0x700177c0d0> as *const (), 0), // +0x59a580 sub_59a580
    Lifted::new(0x7000c865c0, fn_7000c865c0 as *const (), 2), // +0x59a5c0 sub_59a5c0
    Lifted::new(0x7000c86810, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000c8699d, 0x7000caf960, 0x7000c869a3, 0x70007f0910, 0x7000c869a9> as *const (), 0), // +0x59a810 sub_59a810
    Lifted::new(0x7000c86a10, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000c86b9d, 0x7000caf980, 0x7000c86ba3, 0x70007f0910, 0x7000c86ba9> as *const (), 0), // +0x59aa10 sub_59aa10
    Lifted::new(0x7000c86c10, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c86de8, 0x7000caf9a0, 0x7000c86dee, 0x70007f0910, 0x7000c86df4> as *const (), 2), // +0x59ac10 sub_59ac10
    Lifted::new(0x7000c86e70, tpl_1080::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c87071, 0x7000cafa00, 0x7000c87077, 0x70007f0910, 0x7000c8707d> as *const (), 0), // +0x59ae70 sub_59ae70
    Lifted::new(0x7000c87110, tpl_1080::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000c87311, 0x7000cafa20, 0x7000c87317, 0x70007f0910, 0x7000c8731d> as *const (), 0), // +0x59b110 sub_59b110
    Lifted::new(0x7000c873b0, tpl_1177::<0x8> as *const (), 1), // +0x59b3b0 sub_59b3b0
    Lifted::new(0x7000c87450, tpl_1178::<0x7000c873b0> as *const (), 0), // +0x59b450 sub_59b450
    Lifted::new(0x7000c877b0, tpl_1179 as *const (), 1), // +0x59b7b0 sub_59b7b0
    Lifted::new(0x7000c878e0, tpl_1180::<0x7000c87450, 0x7000c878e0, 0x7000c877b0> as *const (), 2), // +0x59b8e0 sub_59b8e0
    Lifted::new(0x7000c87cd0, tpl_1181::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000caf330, 0x70017d37c0, 0x70017790e0, 0x7000c87f4c> as *const (), 2), // +0x59bcd0 sub_59bcd0
    Lifted::new(0x7000c87ff0, tpl_1142::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000c63fe0, 0x70017d37c0, 0x70017790e0, 0x7000c882c0> as *const (), 2), // +0x59bff0 sub_59bff0
    Lifted::new(0x7000c88380, tpl_590::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000a62120, 0x70017d37c0, 0x70017790e0, 0x7000c8862a> as *const (), 2), // +0x59c380 sub_59c380
    Lifted::new(0x7000c886e0, fn_7000c886e0 as *const (), 2), // +0x59c6e0 sub_59c6e0
    Lifted::new(0x7000c889a0, tpl_395::<0x70009a55a0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000c88ae5, 0x70009a4eb0, 0x7000c88aeb> as *const (), 0), // +0x59c9a0 sub_59c9a0
    Lifted::new(0x7000c88b40, tpl_68 as *const (), 1), // +0x59cb40 sub_59cb40
    Lifted::new(0x7000c88bf0, fn_7000c88bf0 as *const (), 2), // +0x59cbf0 sub_59cbf0
    Lifted::new(0x7000c88f70, fn_7000c88f70 as *const (), 0), // +0x59cf70 sub_59cf70
    Lifted::new(0x7000c89090, tpl_1182::<0x7001779e20, 0x7001825680, 0x8, 0x10, 0x18, 0x20, 0x28> as *const (), 0), // +0x59d090 sub_59d090
    Lifted::new(0x7000c89140, tpl_1182::<0x7001779e20, 0x7001825b58, 0x8, 0x10, 0x18, 0x20, 0x28> as *const (), 0), // +0x59d140 sub_59d140
    Lifted::new(0x7000c891f0, tpl_1183::<0x7001779e20, 0x7001825668, 0x30, 0x28, 0x8, 0x10, 0x18, 0x20> as *const (), 0), // +0x59d1f0 sub_59d1f0
    Lifted::new(0x7000c892b0, tpl_1183::<0x7001779e20, 0x7001825b40, 0x30, 0x28, 0x8, 0x10, 0x18, 0x20> as *const (), 0), // +0x59d2b0 sub_59d2b0
    Lifted::new(0x7000c89370, fn_7000c89370 as *const (), 0), // +0x59d370 sub_59d370
    Lifted::new(0x7000c893e0, fn_7000c893e0 as *const (), 0), // +0x59d3e0 sub_59d3e0
    Lifted::new(0x7000c89570, fn_7000c89570 as *const (), 0), // +0x59d570 sub_59d570
    Lifted::new(0x7000c895e0, fn_7000c895e0 as *const (), 0), // +0x59d5e0 sub_59d5e0
    Lifted::new(0x7000c896e0, fn_7000c896e0 as *const (), 0), // +0x59d6e0 sub_59d6e0
    Lifted::new(0x7000c897f0, fn_7000c897f0 as *const (), 0), // +0x59d7f0 sub_59d7f0
    Lifted::new(0x7000c89920, fn_7000c89920 as *const (), 0), // +0x59d920 sub_59d920
    Lifted::new(0x7000c899e0, fn_7000c899e0 as *const (), 0), // +0x59d9e0 sub_59d9e0
    Lifted::new(0x7000c89aa0, tpl_79::<0x8, 0x10, 0x18, 0x7001779e20, 0x30, 0x38, 0x20, 0x7000a61fb0, 0x7000c88bf0> as *const (), 0), // +0x59daa0 sub_59daa0
    Lifted::new(0x7000c89b60, fn_7000c89b60 as *const (), 0), // +0x59db60 sub_59db60
    Lifted::new(0x7000c89c30, fn_7000c89c30 as *const (), 2), // +0x59dc30 sub_59dc30
    Lifted::new(0x7000c89f40, fn_7000c89f40 as *const (), 0), // +0x59df40 sub_59df40
    Lifted::new(0x7000c8a0c0, fn_7000c8a0c0 as *const (), 0), // +0x59e0c0 sub_59e0c0
    Lifted::new(0x7000c8a250, tpl_1039 as *const (), 1), // +0x59e250 sub_59e250
    Lifted::new(0x7000c8a460, fn_7000c8a460 as *const (), 0), // +0x59e460 sub_59e460
    Lifted::new(0x7000c8a990, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8a9f0, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59e990 sub_59e990
    Lifted::new(0x7000c8aa70, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8aad0, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59ea70 sub_59ea70
    Lifted::new(0x7000c8ab50, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8abb0, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59eb50 sub_59eb50
    Lifted::new(0x7000c8ac30, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8ac90, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59ec30 sub_59ec30
    Lifted::new(0x7000c8ad10, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8ad70, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59ed10 sub_59ed10
    Lifted::new(0x7000c8adf0, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8ae50, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59edf0 sub_59edf0
    Lifted::new(0x7000c8aed0, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8af30, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59eed0 sub_59eed0
    Lifted::new(0x7000c8afb0, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8b010, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59efb0 sub_59efb0
    Lifted::new(0x7000c8b090, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8b0f0, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59f090 sub_59f090
    Lifted::new(0x7000c8b170, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000c8b1d0, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x59f170 sub_59f170
    Lifted::new(0x7000c8b250, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c8b2f3> as *const (), 0), // +0x59f250 sub_59f250
    Lifted::new(0x7000c8b320, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000c8b3c3> as *const (), 0), // +0x59f320 sub_59f320
    Lifted::new(0x7000c8b3f0, tpl_103::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000c8b44b> as *const (), 0), // +0x59f3f0 sub_59f3f0
    Lifted::new(0x7000c8b470, tpl_1185::<0x8, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000c8b508> as *const (), 0), // +0x59f470 sub_59f470
    Lifted::new(0x7000c8b540, fn_7000c8b540 as *const (), 0), // +0x59f540 sub_59f540
    Lifted::new(0x7000c8b5d0, fn_7000c8b5d0 as *const (), 0), // +0x59f5d0 sub_59f5d0
    Lifted::new(0x7000c8b680, fn_7000c8b680 as *const (), 0), // +0x59f680 sub_59f680
    Lifted::new(0x7000c8b9b0, fn_7000c8b9b0 as *const (), 0), // +0x59f9b0 sub_59f9b0
    Lifted::new(0x7000c8bae0, fn_7000c8bae0 as *const (), 0), // +0x59fae0 sub_59fae0
    Lifted::new(0x7000c8bd00, fn_7000c8bd00 as *const (), 0), // +0x59fd00 sub_59fd00
    Lifted::new(0x7000c8bdc0, fn_7000c8bdc0 as *const (), 1), // +0x59fdc0 sub_59fdc0
    Lifted::new(0x7000c8bec0, fn_7000c8bec0 as *const (), 0), // +0x59fec0 sub_59fec0
    Lifted::new(0x7000c8bf20, fn_7000c8bf20 as *const (), 1), // +0x59ff20 sub_59ff20
    Lifted::new(0x7000c8c030, tpl_1186::<0xffffffffffffffff> as *const (), 1), // +0x5a0030 sub_5a0030
    Lifted::new(0x7000c8c070, tpl_1186::<0xffffffffffffffff> as *const (), 1), // +0x5a0070 sub_5a0070
    Lifted::new(0x7000c8c0b0, fn_7000c8c0b0 as *const (), 0), // +0x5a00b0 sub_5a00b0
    Lifted::new(0x7000c8c100, fn_7000c8c100 as *const (), 0), // +0x5a0100 sub_5a0100
    Lifted::new(0x7000c8c150, tpl_1148::<0x7000c8c8d0> as *const (), 0), // +0x5a0150 sub_5a0150
    Lifted::new(0x7000c8c160, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0160 sub_5a0160
    Lifted::new(0x7000c8c190, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0190 sub_5a0190
    Lifted::new(0x7000c8c1c0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a01c0 sub_5a01c0
    Lifted::new(0x7000c8c1f0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a01f0 sub_5a01f0
    Lifted::new(0x7000c8c220, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0220 sub_5a0220
    Lifted::new(0x7000c8c250, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0250 sub_5a0250
    Lifted::new(0x7000c8c280, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0280 sub_5a0280
    Lifted::new(0x7000c8c2b0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a02b0 sub_5a02b0
    Lifted::new(0x7000c8c2e0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a02e0 sub_5a02e0
    Lifted::new(0x7000c8c310, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0310 sub_5a0310
    Lifted::new(0x7000c8c340, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0340 sub_5a0340
    Lifted::new(0x7000c8c370, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0370 sub_5a0370
    Lifted::new(0x7000c8c3a0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a03a0 sub_5a03a0
    Lifted::new(0x7000c8c3d0, tpl_133::<0x7000c8b540, 0x700177c312> as *const (), 0), // +0x5a03d0 sub_5a03d0
    Lifted::new(0x7000c8c410, tpl_1149::<0x20, 0x30, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x7001d54968, 0x7000c8c489> as *const (), 0), // +0x5a0410 sub_5a0410
    Lifted::new(0x7000c8c4b0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a04b0 sub_5a04b0
    Lifted::new(0x7000c8c4e0, tpl_1149::<0x20, 0x30, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x7001d54968, 0x7000c8c559> as *const (), 0), // +0x5a04e0 sub_5a04e0
    Lifted::new(0x7000c8c580, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0580 sub_5a0580
    Lifted::new(0x7000c8c5b0, tpl_133::<0x7000c8b5d0, 0x7000701460> as *const (), 0), // +0x5a05b0 sub_5a05b0
    Lifted::new(0x7000c8c5f0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a05f0 sub_5a05f0
    Lifted::new(0x7000c8c620, fn_7000c8c620 as *const (), 0), // +0x5a0620 sub_5a0620
    Lifted::new(0x7000c8c6b0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a06b0 sub_5a06b0
    Lifted::new(0x7000c8c6e0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a06e0 sub_5a06e0
    Lifted::new(0x7000c8c710, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0710 sub_5a0710
    Lifted::new(0x7000c8c740, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0740 sub_5a0740
    Lifted::new(0x7000c8c770, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0770 sub_5a0770
    Lifted::new(0x7000c8c7a0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a07a0 sub_5a07a0
    Lifted::new(0x7000c8c7d0, fn_7000c8c7d0 as *const (), 0), // +0x5a07d0 sub_5a07d0
    Lifted::new(0x7000c8c880, fn_7000c8c880 as *const (), 0), // +0x5a0880 sub_5a0880
    Lifted::new(0x7000c8c8d0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a08d0 sub_5a08d0
    Lifted::new(0x7000c8c900, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0900 sub_5a0900
    Lifted::new(0x7000c8c930, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0930 sub_5a0930
    Lifted::new(0x7000c8c960, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0960 sub_5a0960
    Lifted::new(0x7000c8c990, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0990 sub_5a0990
    Lifted::new(0x7000c8c9c0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a09c0 sub_5a09c0
    Lifted::new(0x7000c8c9f0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a09f0 sub_5a09f0
    Lifted::new(0x7000c8ca20, fn_7000c8ca20 as *const (), 0), // +0x5a0a20 sub_5a0a20
    Lifted::new(0x7000c8caa0, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0aa0 sub_5a0aa0
    Lifted::new(0x7000c8cad0, fn_7000c8cad0 as *const (), 0), // +0x5a0ad0 sub_5a0ad0
    Lifted::new(0x7000c8cb80, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0b80 sub_5a0b80
    Lifted::new(0x7000c8cbb0, fn_7000c8cbb0 as *const (), 0), // +0x5a0bb0 sub_5a0bb0
    Lifted::new(0x7000c8cc00, tpl_33::<0x700177c312> as *const (), 0), // +0x5a0c00 sub_5a0c00
    Lifted::new(0x7000c8cc30, fn_7000c8cc30 as *const (), 0), // +0x5a0c30 sub_5a0c30
    Lifted::new(0x7000c8ccd0, fn_7000c8ccd0 as *const (), 0), // +0x5a0cd0 sub_5a0cd0
    Lifted::new(0x7000c8d1b0, fn_7000c8d1b0 as *const (), 0), // +0x5a11b0 sub_5a11b0
    Lifted::new(0x7000c8d2a0, fn_7000c8d2a0 as *const (), 0), // +0x5a12a0 sub_5a12a0
    Lifted::new(0x7000c8d650, fn_7000c8d650 as *const (), 0), // +0x5a1650 sub_5a1650
    Lifted::new(0x7000c8d9f0, fn_7000c8d9f0 as *const (), 0), // +0x5a19f0 sub_5a19f0
    Lifted::new(0x7000c8dce0, fn_7000c8dce0 as *const (), 0), // +0x5a1ce0 sub_5a1ce0
    Lifted::new(0x7000c8de80, fn_7000c8de80 as *const (), 0), // +0x5a1e80 sub_5a1e80
    Lifted::new(0x7000c8e150, fn_7000c8e150 as *const (), 1), // +0x5a2150 sub_5a2150
    Lifted::new(0x7000c8e1a0, fn_7000c8e1a0 as *const (), 0), // +0x5a21a0 sub_5a21a0
    Lifted::new(0x7000c8e270, tpl_1187::<0x298, 0x8, 0x50, 0xffffffffffffffb0, 0x280, 0x7001d54e98, 0x10, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5a2270 sub_5a2270
    Lifted::new(0x7000c8e320, tpl_1187::<0x298, 0x8, 0x50, 0xffffffffffffffb0, 0x280, 0x7001d54e98, 0x10, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5a2320 sub_5a2320
    Lifted::new(0x7000c8e3d0, fn_7000c8e3d0 as *const (), 0), // +0x5a23d0 sub_5a23d0
    Lifted::new(0x7000c8e580, fn_7000c8e580 as *const (), 2), // +0x5a2580 sub_5a2580
    Lifted::new(0x7000c8e960, fn_7000c8e960 as *const (), 0), // +0x5a2960 sub_5a2960
    Lifted::new(0x7000c8ebb0, fn_7000c8ebb0 as *const (), 2), // +0x5a2bb0 sub_5a2bb0
    Lifted::new(0x7000c8f150, fn_7000c8f150 as *const (), 2), // +0x5a3150 sub_5a3150
    Lifted::new(0x7000c8fe30, fn_7000c8fe30 as *const (), 2), // +0x5a3e30 sub_5a3e30
    Lifted::new(0x7000c90420, fn_7000c90420 as *const (), 0), // +0x5a4420 sub_5a4420
    Lifted::new(0x7000c905f0, fn_7000c905f0 as *const (), 0), // +0x5a45f0 sub_5a45f0
    Lifted::new(0x7000c906d0, fn_7000c906d0 as *const (), 2), // +0x5a46d0 sub_5a46d0
    Lifted::new(0x7000c90bf0, fn_7000c90bf0 as *const (), 2), // +0x5a4bf0 sub_5a4bf0
    Lifted::new(0x7000c91040, fn_7000c91040 as *const (), 2), // +0x5a5040 sub_5a5040
    Lifted::new(0x7000c91c10, fn_7000c91c10 as *const (), 0), // +0x5a5c10 sub_5a5c10
    Lifted::new(0x7000c91cc0, fn_7000c91cc0 as *const (), 2), // +0x5a5cc0 sub_5a5cc0
    Lifted::new(0x7000c92370, fn_7000c92370 as *const (), 0), // +0x5a6370 sub_5a6370
    Lifted::new(0x7000c92500, fn_7000c92500 as *const (), 2), // +0x5a6500 sub_5a6500
    Lifted::new(0x7000c92a00, fn_7000c92a00 as *const (), 2), // +0x5a6a00 sub_5a6a00
    Lifted::new(0x7000c92b40, tpl_1188::<0x8, 0x7001779e20, 0x7001825680, 0x2121, 0x18, 0x10, 0x20, 0x28> as *const (), 0), // +0x5a6b40 sub_5a6b40
    Lifted::new(0x7000c92bf0, tpl_1188::<0x8, 0x7001779e20, 0x7001825b58, 0x59, 0x18, 0x10, 0x20, 0x28> as *const (), 0), // +0x5a6bf0 sub_5a6bf0
    Lifted::new(0x7000c92ca0, tpl_1189::<0x8, 0x7001779e20, 0x7001825668, 0x28, 0x2121, 0x30, 0x10, 0x18, 0x20> as *const (), 0), // +0x5a6ca0 sub_5a6ca0
    Lifted::new(0x7000c92d60, tpl_1189::<0x8, 0x7001779e20, 0x7001825b40, 0x28, 0x59, 0x30, 0x10, 0x18, 0x20> as *const (), 0), // +0x5a6d60 sub_5a6d60
    Lifted::new(0x7000c92e20, fn_7000c92e20 as *const (), 0), // +0x5a6e20 sub_5a6e20
    Lifted::new(0x7000c92e40, fn_7000c92e40 as *const (), 0), // +0x5a6e40 sub_5a6e40
    Lifted::new(0x7000c93ab0, fn_7000c93ab0 as *const (), 0), // +0x5a7ab0 sub_5a7ab0
    Lifted::new(0x7000c93ae0, fn_7000c93ae0 as *const (), 0), // +0x5a7ae0 sub_5a7ae0
    Lifted::new(0x7000c94850, tpl_143::<0x2118, 0x7000ce5910> as *const (), 0), // +0x5a8850 sub_5a8850
    Lifted::new(0x7000c94860, fn_7000c94860 as *const (), 0), // +0x5a8860 sub_5a8860
    Lifted::new(0x7000c94920, fn_7000c94920 as *const (), 2), // +0x5a8920 sub_5a8920
    Lifted::new(0x7000c957b0, fn_7000c957b0 as *const (), 2), // +0x5a97b0 sub_5a97b0
    Lifted::new(0x7000c963a0, fn_7000c963a0 as *const (), 0), // +0x5aa3a0 sub_5aa3a0
    Lifted::new(0x7000c96510, fn_7000c96510 as *const (), 0), // +0x5aa510 sub_5aa510
    Lifted::new(0x7000c96560, fn_7000c96560 as *const (), 2), // +0x5aa560 sub_5aa560
    Lifted::new(0x7000c96920, fn_7000c96920 as *const (), 2), // +0x5aa920 sub_5aa920
    Lifted::new(0x7000c97220, fn_7000c97220 as *const (), 2), // +0x5ab220 sub_5ab220
    Lifted::new(0x7000c97800, fn_7000c97800 as *const (), 0), // +0x5ab800 sub_5ab800
    Lifted::new(0x7000c979d0, fn_7000c979d0 as *const (), 0), // +0x5ab9d0 sub_5ab9d0
    Lifted::new(0x7000c97ad0, fn_7000c97ad0 as *const (), 0), // +0x5abad0 sub_5abad0
    Lifted::new(0x7000c97b70, fn_7000c97b70 as *const (), 2), // +0x5abb70 sub_5abb70
    Lifted::new(0x7000c97cd0, fn_7000c97cd0 as *const (), 2), // +0x5abcd0 sub_5abcd0
    Lifted::new(0x7000c97fb0, fn_7000c97fb0 as *const (), 2), // +0x5abfb0 sub_5abfb0
    Lifted::new(0x7000c98550, fn_7000c98550 as *const (), 0), // +0x5ac550 sub_5ac550
    Lifted::new(0x7000c985d0, fn_7000c985d0 as *const (), 0), // +0x5ac5d0 sub_5ac5d0
    Lifted::new(0x7000c98a50, fn_7000c98a50 as *const (), 1), // +0x5aca50 sub_5aca50
    Lifted::new(0x7000c98a60, fn_7000c98a60 as *const (), 1), // +0x5aca60 sub_5aca60
    Lifted::new(0x7000c98a70, fn_7000c98a70 as *const (), 1), // +0x5aca70 sub_5aca70
    Lifted::new(0x7000c98aa0, fn_7000c98aa0 as *const (), 1), // +0x5acaa0 sub_5acaa0
    Lifted::new(0x7000c98ad0, fn_7000c98ad0 as *const (), 1), // +0x5acad0 sub_5acad0
    Lifted::new(0x7000c98b10, fn_7000c98b10 as *const (), 1), // +0x5acb10 sub_5acb10
    Lifted::new(0x7000c98b20, fn_7000c98b20 as *const (), 1), // +0x5acb20 sub_5acb20
    Lifted::new(0x7000c98b30, fn_7000c98b30 as *const (), 0), // +0x5acb30 sub_5acb30
    Lifted::new(0x7000c98ba0, fn_7000c98ba0 as *const (), 1), // +0x5acba0 sub_5acba0
    Lifted::new(0x7000c98be0, fn_7000c98be0 as *const (), 1), // +0x5acbe0 sub_5acbe0
    Lifted::new(0x7000c98c50, fn_7000c98c50 as *const (), 0), // +0x5acc50 sub_5acc50
    Lifted::new(0x7000c99070, tpl_15 as *const (), 1), // +0x5ad070 sub_5ad070
    Lifted::new(0x7000c99080, fn_7000c99080 as *const (), 0), // +0x5ad080 sub_5ad080
    Lifted::new(0x7000c990d0, fn_7000c990d0 as *const (), 0), // +0x5ad0d0 sub_5ad0d0
    Lifted::new(0x7000c99290, fn_7000c99290 as *const (), 0), // +0x5ad290 sub_5ad290
    Lifted::new(0x7000c99490, fn_7000c99490 as *const (), 0), // +0x5ad490 sub_5ad490
    Lifted::new(0x7000c99760, tpl_143::<0x2060, 0x7000c99490> as *const (), 0), // +0x5ad760 sub_5ad760
    Lifted::new(0x7000c99770, fn_7000c99770 as *const (), 2), // +0x5ad770 sub_5ad770
    Lifted::new(0x7000c9ab30, fn_7000c9ab30 as *const (), 2), // +0x5aeb30 sub_5aeb30
    Lifted::new(0x7000c9b160, fn_7000c9b160 as *const (), 0), // +0x5af160 sub_5af160
    Lifted::new(0x7000c9bdc0, fn_7000c9bdc0 as *const (), 0), // +0x5afdc0 sub_5afdc0
    Lifted::new(0x7000c9be80, fn_7000c9be80 as *const (), 2), // +0x5afe80 sub_5afe80
    Lifted::new(0x7000c9c6b0, fn_7000c9c6b0 as *const (), 0), // +0x5b06b0 sub_5b06b0
    Lifted::new(0x7000c9c750, fn_7000c9c750 as *const (), 0), // +0x5b0750 sub_5b0750
    Lifted::new(0x7000c9c7f0, fn_7000c9c7f0 as *const (), 0), // +0x5b07f0 sub_5b07f0
    Lifted::new(0x7000c9cae0, fn_7000c9cae0 as *const (), 2), // +0x5b0ae0 sub_5b0ae0
    Lifted::new(0x7000c9cd40, fn_7000c9cd40 as *const (), 0), // +0x5b0d40 sub_5b0d40
    Lifted::new(0x7000c9ce20, fn_7000c9ce20 as *const (), 2), // +0x5b0e20 sub_5b0e20
    Lifted::new(0x7000c9d180, fn_7000c9d180 as *const (), 2), // +0x5b1180 sub_5b1180
    Lifted::new(0x7000c9d920, fn_7000c9d920 as *const (), 2), // +0x5b1920 sub_5b1920
    Lifted::new(0x7000c9dd20, fn_7000c9dd20 as *const (), 0), // +0x5b1d20 sub_5b1d20
    Lifted::new(0x7000c9de10, tpl_1190::<0x7001c72c08, 0x700070ecd0, 0x700180c498, 0x7001825d48, 0x7001d54e98, 0x700076e7d0, 0x7000caa4e0, 0x7000caa0a0, 0x7001c72015, 0x700078f610, 0x700076a240, 0x7001d550d8> as *const (), 2), // +0x5b1e10 sub_5b1e10
    Lifted::new(0x7000c9e0b0, tpl_1190::<0x7001c72bf8, 0x700070ecd0, 0x700180c498, 0x7001825d48, 0x7001d54e98, 0x700076e7d0, 0x7000caa7c0, 0x7000caa2c0, 0x7001c72015, 0x700078f610, 0x700076a240, 0x7001d550d8> as *const (), 2), // +0x5b20b0 sub_5b20b0
    Lifted::new(0x7000c9e350, tpl_1191::<0x18, 0x8, 0x7000ca1980, 0x7000ca13c0> as *const (), 0), // +0x5b2350 sub_5b2350
    Lifted::new(0x7000c9e370, tpl_1191::<0x18, 0x8, 0x7000ca1d60, 0x7000ca16a0> as *const (), 0), // +0x5b2370 sub_5b2370
    Lifted::new(0x7000c9e390, fn_7000c9e390 as *const (), 2), // +0x5b2390 sub_5b2390
    Lifted::new(0x7000c9e940, fn_7000c9e940 as *const (), 2), // +0x5b2940 sub_5b2940
    Lifted::new(0x7000c9ee20, fn_7000c9ee20 as *const (), 2), // +0x5b2e20 sub_5b2e20
    Lifted::new(0x7000c9ef70, fn_7000c9ef70 as *const (), 0), // +0x5b2f70 sub_5b2f70
    Lifted::new(0x7000c9f010, fn_7000c9f010 as *const (), 2), // +0x5b3010 sub_5b3010
    Lifted::new(0x7000c9f300, fn_7000c9f300 as *const (), 2), // +0x5b3300 sub_5b3300
    Lifted::new(0x7000c9ff20, fn_7000c9ff20 as *const (), 2), // +0x5b3f20 sub_5b3f20
    Lifted::new(0x7000ca0520, fn_7000ca0520 as *const (), 2), // +0x5b4520 sub_5b4520
    Lifted::new(0x7000ca0960, fn_7000ca0960 as *const (), 0), // +0x5b4960 sub_5b4960
    Lifted::new(0x7000ca0a60, fn_7000ca0a60 as *const (), 2), // +0x5b4a60 sub_5b4a60
    Lifted::new(0x7000ca0e30, fn_7000ca0e30 as *const (), 0), // +0x5b4e30 sub_5b4e30
    Lifted::new(0x7000ca1200, fn_7000ca1200 as *const (), 0), // +0x5b5200 sub_5b5200
    Lifted::new(0x7000ca13c0, tpl_1192::<0x7001c72c48, 0x700070ecd0, 0x700180c498, 0x7001825e10, 0x70018252e0, 0x7001825390, 0x700070a940, 0x7000c714c0> as *const (), 2), // +0x5b53c0 sub_5b53c0
    Lifted::new(0x7000ca16a0, tpl_1192::<0x7001c72c28, 0x700070ecd0, 0x700180c498, 0x7001825e10, 0x70018252e0, 0x7001825390, 0x700070a940, 0x7000c714c0> as *const (), 2), // +0x5b56a0 sub_5b56a0
    Lifted::new(0x7000ca1980, fn_7000ca1980 as *const (), 2), // +0x5b5980 sub_5b5980
    Lifted::new(0x7000ca1d60, fn_7000ca1d60 as *const (), 2), // +0x5b5d60 sub_5b5d60
    Lifted::new(0x7000ca2140, tpl_1044::<0x8, 0x10, 0x280, 0x7001d54e98, 0x7001d550d8, 0x7001779e20, 0x7000c8a250> as *const (), 0), // +0x5b6140 sub_5b6140
    Lifted::new(0x7000ca2240, fn_7000ca2240 as *const (), 2), // +0x5b6240 sub_5b6240
    Lifted::new(0x7000ca2640, fn_7000ca2640 as *const (), 2), // +0x5b6640 sub_5b6640
    Lifted::new(0x7000ca2960, fn_7000ca2960 as *const (), 0), // +0x5b6960 sub_5b6960
    Lifted::new(0x7000ca29b0, fn_7000ca29b0 as *const (), 0), // +0x5b69b0 sub_5b69b0
    Lifted::new(0x7000ca2a60, fn_7000ca2a60 as *const (), 0), // +0x5b6a60 sub_5b6a60
    Lifted::new(0x7000ca2b60, fn_7000ca2b60 as *const (), 2), // +0x5b6b60 sub_5b6b60
    Lifted::new(0x7000ca32e0, fn_7000ca32e0 as *const (), 0), // +0x5b72e0 sub_5b72e0
    Lifted::new(0x7000ca3400, fn_7000ca3400 as *const (), 0), // +0x5b7400 sub_5b7400
    Lifted::new(0x7000ca34d0, fn_7000ca34d0 as *const (), 2), // +0x5b74d0 sub_5b74d0
    Lifted::new(0x7000ca3570, fn_7000ca3570 as *const (), 1), // +0x5b7570 sub_5b7570
    Lifted::new(0x7000ca35b0, fn_7000ca35b0 as *const (), 0), // +0x5b75b0 sub_5b75b0
    Lifted::new(0x7000ca3880, fn_7000ca3880 as *const (), 1), // +0x5b7880 sub_5b7880
    Lifted::new(0x7000ca3900, fn_7000ca3900 as *const (), 2), // +0x5b7900 sub_5b7900
    Lifted::new(0x7000ca58d0, fn_7000ca58d0 as *const (), 0), // +0x5b98d0 sub_5b98d0
    Lifted::new(0x7000ca5a10, fn_7000ca5a10 as *const (), 2), // +0x5b9a10 sub_5b9a10
    Lifted::new(0x7000ca5e60, fn_7000ca5e60 as *const (), 2), // +0x5b9e60 sub_5b9e60
    Lifted::new(0x7000ca6260, fn_7000ca6260 as *const (), 2), // +0x5ba260 sub_5ba260
    Lifted::new(0x7000ca6650, tpl_15 as *const (), 1), // +0x5ba650 sub_5ba650
    Lifted::new(0x7000ca6660, tpl_15 as *const (), 1), // +0x5ba660 sub_5ba660
    Lifted::new(0x7000ca6670, fn_7000ca6670 as *const (), 0), // +0x5ba670 sub_5ba670
    Lifted::new(0x7000ca6780, fn_7000ca6780 as *const (), 0), // +0x5ba780 sub_5ba780
    Lifted::new(0x7000ca6880, fn_7000ca6880 as *const (), 0), // +0x5ba880 sub_5ba880
    Lifted::new(0x7000ca6960, fn_7000ca6960 as *const (), 0), // +0x5ba960 sub_5ba960
    Lifted::new(0x7000ca6d20, fn_7000ca6d20 as *const (), 0), // +0x5bad20 sub_5bad20
    Lifted::new(0x7000ca6e80, fn_7000ca6e80 as *const (), 2), // +0x5bae80 sub_5bae80
    Lifted::new(0x7000ca71b0, fn_7000ca71b0 as *const (), 0), // +0x5bb1b0 sub_5bb1b0
    Lifted::new(0x7000ca7260, fn_7000ca7260 as *const (), 2), // +0x5bb260 sub_5bb260
    Lifted::new(0x7000ca74c0, fn_7000ca74c0 as *const (), 0), // +0x5bb4c0 sub_5bb4c0
    Lifted::new(0x7000ca75c0, tpl_15 as *const (), 1), // +0x5bb5c0 sub_5bb5c0
    Lifted::new(0x7000ca75d0, fn_7000ca75d0 as *const (), 2), // +0x5bb5d0 sub_5bb5d0
    Lifted::new(0x7000ca7a70, fn_7000ca7a70 as *const (), 2), // +0x5bba70 sub_5bba70
    Lifted::new(0x7000ca7bc0, fn_7000ca7bc0 as *const (), 0), // +0x5bbbc0 sub_5bbbc0
    Lifted::new(0x7000ca7e60, fn_7000ca7e60 as *const (), 0), // +0x5bbe60 sub_5bbe60
    Lifted::new(0x7000ca7f50, fn_7000ca7f50 as *const (), 0), // +0x5bbf50 sub_5bbf50
    Lifted::new(0x7000ca80b0, fn_7000ca80b0 as *const (), 0), // +0x5bc0b0 sub_5bc0b0
    Lifted::new(0x7000ca8180, fn_7000ca8180 as *const (), 2), // +0x5bc180 sub_5bc180
    Lifted::new(0x7000ca9070, fn_7000ca9070 as *const (), 2), // +0x5bd070 sub_5bd070
    Lifted::new(0x7000ca9f60, fn_7000ca9f60 as *const (), 0), // +0x5bdf60 sub_5bdf60
    Lifted::new(0x7000caa0a0, tpl_1193::<0x7000c82100, 0x7000cb7d70, 0x7001825db0, 0x70017a0290, 0x7001d54ef8, 0x7000caa1b6, 0x7000c829b0, 0x7000cb7d40> as *const (), 0), // +0x5be0a0 sub_5be0a0
    Lifted::new(0x7000caa2c0, tpl_1193::<0x7000c826c0, 0x7000cb7d70, 0x7001825db0, 0x70017a0290, 0x7001d54ef8, 0x7000caa3d6, 0x7000c82df0, 0x7000cb7d40> as *const (), 0), // +0x5be2c0 sub_5be2c0
    Lifted::new(0x7000caa4e0, tpl_1194::<0x70006fa4a0, 0x7000c752f0, 0x7000cb7d30, 0x700098a360, 0x7000c758b0, 0x7000cb87a0, 0x7000cb7cc0, 0x70007720a0, 0x7000c75600, 0x7000cb7d20, 0x7000cb7c60> as *const (), 2), // +0x5be4e0 sub_5be4e0
    Lifted::new(0x7000caa7c0, tpl_1194::<0x70006fa4a0, 0x7000c75f50, 0x7000cb7d30, 0x700098a360, 0x7000c76590, 0x7000cb87a0, 0x7000cb7cc0, 0x70007720a0, 0x7000c75040, 0x7000cb7d20, 0x7000cb7c60> as *const (), 2), // +0x5be7c0 sub_5be7c0
    Lifted::new(0x7000caaaa0, fn_7000caaaa0 as *const (), 0), // +0x5beaa0 sub_5beaa0
    Lifted::new(0x7000caaae0, fn_7000caaae0 as *const (), 0), // +0x5beae0 sub_5beae0
    Lifted::new(0x7000caab90, fn_7000caab90 as *const (), 0), // +0x5beb90 sub_5beb90
    Lifted::new(0x7000caabd0, fn_7000caabd0 as *const (), 0), // +0x5bebd0 sub_5bebd0
    Lifted::new(0x7000caae90, fn_7000caae90 as *const (), 0), // +0x5bee90 sub_5bee90
    Lifted::new(0x7000caaf80, fn_7000caaf80 as *const (), 0), // +0x5bef80 sub_5bef80
    Lifted::new(0x7000cab020, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab031> as *const (), 0), // +0x5bf020 sub_5bf020
    Lifted::new(0x7000cab040, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000cab058> as *const (), 0), // +0x5bf040 sub_5bf040
    Lifted::new(0x7000cab060, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000cab078> as *const (), 0), // +0x5bf060 sub_5bf060
    Lifted::new(0x7000cab080, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000cab098> as *const (), 0), // +0x5bf080 sub_5bf080
    Lifted::new(0x7000cab0a0, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000cab0b8> as *const (), 0), // +0x5bf0a0 sub_5bf0a0
    Lifted::new(0x7000cab0c0, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000cab0d8> as *const (), 0), // +0x5bf0c0 sub_5bf0c0
    Lifted::new(0x7000cab0e0, fn_7000cab0e0 as *const (), 0), // +0x5bf0e0 sub_5bf0e0
    Lifted::new(0x7000cab230, fn_7000cab230 as *const (), 0), // +0x5bf230 sub_5bf230
    Lifted::new(0x7000cab370, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab381> as *const (), 0), // +0x5bf370 sub_5bf370
    Lifted::new(0x7000cab390, fn_7000cab390 as *const (), 0), // +0x5bf390 sub_5bf390
    Lifted::new(0x7000cab430, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab441> as *const (), 0), // +0x5bf430 sub_5bf430
    Lifted::new(0x7000cab450, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab461> as *const (), 0), // +0x5bf450 sub_5bf450
    Lifted::new(0x7000cab470, tpl_15 as *const (), 1), // +0x5bf470 sub_5bf470
    Lifted::new(0x7000cab480, fn_7000cab480 as *const (), 2), // +0x5bf480 sub_5bf480
    Lifted::new(0x7000cab650, fn_7000cab650 as *const (), 0), // +0x5bf650 sub_5bf650
    Lifted::new(0x7000cab790, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab7a1> as *const (), 0), // +0x5bf790 sub_5bf790
    Lifted::new(0x7000cab7b0, fn_7000cab7b0 as *const (), 0), // +0x5bf7b0 sub_5bf7b0
    Lifted::new(0x7000cab870, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab881> as *const (), 0), // +0x5bf870 sub_5bf870
    Lifted::new(0x7000cab890, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cab8a1> as *const (), 0), // +0x5bf890 sub_5bf890
    Lifted::new(0x7000cab8b0, fn_7000cab8b0 as *const (), 2), // +0x5bf8b0 sub_5bf8b0
    Lifted::new(0x7000caba70, tpl_15 as *const (), 1), // +0x5bfa70 sub_5bfa70
    Lifted::new(0x7000caba80, tpl_15 as *const (), 1), // +0x5bfa80 sub_5bfa80
    Lifted::new(0x7000caba90, tpl_16::<0x7000caba93, 0x7000caba94, 0x7000caba95, 0x7000caba96, 0x7000caba97, 0x7000caba98, 0x7000caba99, 0x7000caba9a, 0x7000caba9b, 0x7000caba9c, 0x7000caba9d, 0x7000caba9e, 0x7000caba9f, 0x7000cabac0> as *const (), 0), // +0x5bfa90 sub_5bfa90
    Lifted::new(0x7000cabab0, tpl_15 as *const (), 1), // +0x5bfab0 sub_5bfab0
    Lifted::new(0x7000cabac0, fn_7000cabac0 as *const (), 0), // +0x5bfac0 sub_5bfac0
    Lifted::new(0x7000cabbb0, fn_7000cabbb0 as *const (), 0), // +0x5bfbb0 sub_5bfbb0
    Lifted::new(0x7000cabcb0, fn_7000cabcb0 as *const (), 2), // +0x5bfcb0 sub_5bfcb0
    Lifted::new(0x7000cadc40, tpl_1051::<0x4, 0x7000c80e40, 0x7000c80f80> as *const (), 0), // +0x5c1c40 sub_5c1c40
    Lifted::new(0x7000cadd20, tpl_1052::<0x4, 0x7000c80e40> as *const (), 0), // +0x5c1d20 sub_5c1d20
    Lifted::new(0x7000caddd0, fn_7000caddd0 as *const (), 0), // +0x5c1dd0 sub_5c1dd0
    Lifted::new(0x7000caded0, fn_7000caded0 as *const (), 1), // +0x5c1ed0 sub_5c1ed0
    Lifted::new(0x7000cadf30, fn_7000cadf30 as *const (), 1), // +0x5c1f30 sub_5c1f30
    Lifted::new(0x7000cadf60, tpl_1195::<0x98> as *const (), 0), // +0x5c1f60 sub_5c1f60
    Lifted::new(0x7000cadf90, tpl_1195::<0x98> as *const (), 0), // +0x5c1f90 sub_5c1f90
    Lifted::new(0x7000cadfc0, fn_7000cadfc0 as *const (), 0), // +0x5c1fc0 sub_5c1fc0
    Lifted::new(0x7000cae010, fn_7000cae010 as *const (), 0), // +0x5c2010 sub_5c2010
    Lifted::new(0x7000cae050, tpl_1053::<0x7000c80e40, 0x7000c80f80> as *const (), 0), // +0x5c2050 sub_5c2050
    Lifted::new(0x7000cae130, tpl_1054::<0x7000c80e40> as *const (), 0), // +0x5c2130 sub_5c2130
    Lifted::new(0x7000cae1e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cae1f1> as *const (), 0), // +0x5c21e0 sub_5c21e0
    Lifted::new(0x7000cae200, fn_7000cae200 as *const (), 0), // +0x5c2200 sub_5c2200
    Lifted::new(0x7000cae360, fn_7000cae360 as *const (), 1), // +0x5c2360 sub_5c2360
    Lifted::new(0x7000cae3e0, fn_7000cae3e0 as *const (), 1), // +0x5c23e0 sub_5c23e0
    Lifted::new(0x7000cae440, tpl_1196::<0x98> as *const (), 0), // +0x5c2440 sub_5c2440
    Lifted::new(0x7000cae4f0, tpl_1196::<0x98> as *const (), 0), // +0x5c24f0 sub_5c24f0
    Lifted::new(0x7000cae5a0, fn_7000cae5a0 as *const (), 0), // +0x5c25a0 sub_5c25a0
    Lifted::new(0x7000cae650, fn_7000cae650 as *const (), 0), // +0x5c2650 sub_5c2650
    Lifted::new(0x7000cae6f0, fn_7000cae6f0 as *const (), 1), // +0x5c26f0 sub_5c26f0
    Lifted::new(0x7000cae720, fn_7000cae720 as *const (), 0), // +0x5c2720 sub_5c2720
    Lifted::new(0x7000cae820, fn_7000cae820 as *const (), 0), // +0x5c2820 sub_5c2820
    Lifted::new(0x7000cae8a0, fn_7000cae8a0 as *const (), 0), // +0x5c28a0 sub_5c28a0
    Lifted::new(0x7000cae990, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cae9a1> as *const (), 0), // +0x5c2990 sub_5c2990
    Lifted::new(0x7000cae9b0, fn_7000cae9b0 as *const (), 1), // +0x5c29b0 sub_5c29b0
    Lifted::new(0x7000caea40, fn_7000caea40 as *const (), 1), // +0x5c2a40 sub_5c2a40
    Lifted::new(0x7000caea90, fn_7000caea90 as *const (), 1), // +0x5c2a90 sub_5c2a90
    Lifted::new(0x7000caeae0, fn_7000caeae0 as *const (), 0), // +0x5c2ae0 sub_5c2ae0
    Lifted::new(0x7000caec20, fn_7000caec20 as *const (), 0), // +0x5c2c20 sub_5c2c20
    Lifted::new(0x7000caece0, fn_7000caece0 as *const (), 0), // +0x5c2ce0 sub_5c2ce0
    Lifted::new(0x7000caedd0, tpl_15 as *const (), 1), // +0x5c2dd0 sub_5c2dd0
    Lifted::new(0x7000caede0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000caedf1> as *const (), 0), // +0x5c2de0 sub_5c2de0
    Lifted::new(0x7000caee00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000caee11> as *const (), 0), // +0x5c2e00 sub_5c2e00
    Lifted::new(0x7000caee20, fn_7000caee20 as *const (), 0), // +0x5c2e20 sub_5c2e20
    Lifted::new(0x7000caeea0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000caefa7, 0x7000c88b40, 0x70007f0910, 0x7000caefc3> as *const (), 0), // +0x5c2ea0 sub_5c2ea0
    Lifted::new(0x7000caf010, tpl_923::<0x7001825fc8> as *const (), 1), // +0x5c3010 sub_5c3010
    Lifted::new(0x7000caf030, tpl_923::<0x7001825f88> as *const (), 1), // +0x5c3030 sub_5c3030
    Lifted::new(0x7000caf050, tpl_923::<0x7001826120> as *const (), 1), // +0x5c3050 sub_5c3050
    Lifted::new(0x7000caf070, tpl_923::<0x7001826048> as *const (), 1), // +0x5c3070 sub_5c3070
    Lifted::new(0x7000caf090, tpl_923::<0x7001826008> as *const (), 1), // +0x5c3090 sub_5c3090
    Lifted::new(0x7000caf0b0, tpl_923::<0x70018260e0> as *const (), 1), // +0x5c30b0 sub_5c30b0
    Lifted::new(0x7000caf0d0, tpl_754::<0x700177c312> as *const (), 0), // +0x5c30d0 sub_5c30d0
    Lifted::new(0x7000caf0f0, tpl_754::<0x700177c312> as *const (), 0), // +0x5c30f0 sub_5c30f0
    Lifted::new(0x7000caf110, tpl_754::<0x700177c312> as *const (), 0), // +0x5c3110 sub_5c3110
    Lifted::new(0x7000caf130, tpl_754::<0x700177c312> as *const (), 0), // +0x5c3130 sub_5c3130
    Lifted::new(0x7000caf150, tpl_754::<0x700177c312> as *const (), 0), // +0x5c3150 sub_5c3150
    Lifted::new(0x7000caf170, tpl_754::<0x700177c312> as *const (), 0), // +0x5c3170 sub_5c3170
    Lifted::new(0x7000caf190, tpl_1197::<0x8, 0x7000bf3e50> as *const (), 0), // +0x5c3190 sub_5c3190
    Lifted::new(0x7000caf1e0, tpl_1197::<0x8, 0x7000bf3e50> as *const (), 0), // +0x5c31e0 sub_5c31e0
    Lifted::new(0x7000caf230, tpl_1197::<0x8, 0x7000bf3e50> as *const (), 0), // +0x5c3230 sub_5c3230
    Lifted::new(0x7000caf280, tpl_44 as *const (), 1), // +0x5c3280 sub_5c3280
    Lifted::new(0x7000caf290, fn_7000caf290 as *const (), 0), // +0x5c3290 sub_5c3290
    Lifted::new(0x7000caf2e0, tpl_1197::<0x8, 0x7000bf3e50> as *const (), 0), // +0x5c32e0 sub_5c32e0
    Lifted::new(0x7000caf330, tpl_1055::<0x7000caeea0, 0x70017d37a0, 0x70017790e0, 0x7000caf4a6> as *const (), 0), // +0x5c3330 sub_5c3330
    Lifted::new(0x7000caf510, tpl_1159::<0x7000c635a0, 0x70017d37a0, 0x70017790e0, 0x7000caf6de> as *const (), 0), // +0x5c3510 sub_5c3510
    Lifted::new(0x7000caf760, tpl_52::<0x8> as *const (), 1), // +0x5c3760 sub_5c3760
    Lifted::new(0x7000caf770, tpl_52::<0x8> as *const (), 1), // +0x5c3770 sub_5c3770
    Lifted::new(0x7000caf780, tpl_52::<0x8> as *const (), 1), // +0x5c3780 sub_5c3780
    Lifted::new(0x7000caf790, tpl_52::<0x8> as *const (), 1), // +0x5c3790 sub_5c3790
    Lifted::new(0x7000caf7a0, tpl_52::<0x8> as *const (), 1), // +0x5c37a0 sub_5c37a0
    Lifted::new(0x7000caf7b0, tpl_52::<0x8> as *const (), 1), // +0x5c37b0 sub_5c37b0
    Lifted::new(0x7000caf7c0, tpl_923::<0x7001825fc8> as *const (), 1), // +0x5c37c0 sub_5c37c0
    Lifted::new(0x7000caf7e0, tpl_923::<0x7001825f88> as *const (), 1), // +0x5c37e0 sub_5c37e0
    Lifted::new(0x7000caf800, tpl_923::<0x7001826120> as *const (), 1), // +0x5c3800 sub_5c3800
    Lifted::new(0x7000caf820, tpl_923::<0x7001826048> as *const (), 1), // +0x5c3820 sub_5c3820
    Lifted::new(0x7000caf840, tpl_923::<0x7001826008> as *const (), 1), // +0x5c3840 sub_5c3840
    Lifted::new(0x7000caf860, tpl_923::<0x70018260e0> as *const (), 1), // +0x5c3860 sub_5c3860
    Lifted::new(0x7000caf880, tpl_315::<0x7001c6bd80> as *const (), 1), // +0x5c3880 sub_5c3880
    Lifted::new(0x7000caf890, tpl_315::<0x7001c6bdd0> as *const (), 1), // +0x5c3890 sub_5c3890
    Lifted::new(0x7000caf8a0, tpl_315::<0x7001c6be20> as *const (), 1), // +0x5c38a0 sub_5c38a0
    Lifted::new(0x7000caf8b0, tpl_315::<0x7001c6bce0> as *const (), 1), // +0x5c38b0 sub_5c38b0
    Lifted::new(0x7000caf8c0, tpl_315::<0x7001c6bd30> as *const (), 1), // +0x5c38c0 sub_5c38c0
    Lifted::new(0x7000caf8d0, tpl_315::<0x7001c6be70> as *const (), 1), // +0x5c38d0 sub_5c38d0
    Lifted::new(0x7000caf8e0, tpl_103::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000caf93b> as *const (), 0), // +0x5c38e0 sub_5c38e0
    Lifted::new(0x7000caf960, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000caf970> as *const (), 0), // +0x5c3960 sub_5c3960
    Lifted::new(0x7000caf980, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000caf990> as *const (), 0), // +0x5c3980 sub_5c3980
    Lifted::new(0x7000caf9a0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000caf9b0> as *const (), 0), // +0x5c39a0 sub_5c39a0
    Lifted::new(0x7000caf9c0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000caf9d0> as *const (), 0), // +0x5c39c0 sub_5c39c0
    Lifted::new(0x7000caf9e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000caf9f0> as *const (), 0), // +0x5c39e0 sub_5c39e0
    Lifted::new(0x7000cafa00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cafa10> as *const (), 0), // +0x5c3a00 sub_5c3a00
    Lifted::new(0x7000cafa20, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cafa30> as *const (), 0), // +0x5c3a20 sub_5c3a20
    Lifted::new(0x7000cafa40, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cafa50> as *const (), 0), // +0x5c3a40 sub_5c3a40
    Lifted::new(0x7000cafa60, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cafaad, 0x70007f0910, 0x7000cafacb> as *const (), 0), // +0x5c3a60 sub_5c3a60
    Lifted::new(0x7000cafaf0, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cafb3d, 0x70007f0910, 0x7000cafb5b> as *const (), 0), // +0x5c3af0 sub_5c3af0
    Lifted::new(0x7000cafb80, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cafbd4, 0x70007f0910, 0x7000cafbef> as *const (), 0), // +0x5c3b80 sub_5c3b80
    Lifted::new(0x7000cafc10, tpl_36::<0x8> as *const (), 1), // +0x5c3c10 sub_5c3c10
    Lifted::new(0x7000cafc20, tpl_1181::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000cb1460, 0x70017d37c0, 0x70017790e0, 0x7000cafe9c> as *const (), 2), // +0x5c3c20 sub_5c3c20
    Lifted::new(0x7000caff40, tpl_68 as *const (), 1), // +0x5c3f40 sub_5c3f40
    Lifted::new(0x7000cb0020, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x7000cb12f0> as *const (), 0), // +0x5c4020 sub_5c4020
    Lifted::new(0x7000cb00b0, tpl_1198::<0x7000799390, 0x8, 0x10> as *const (), 0), // +0x5c40b0 sub_5c40b0
    Lifted::new(0x7000cb00e0, fn_7000cb00e0 as *const (), 0), // +0x5c40e0 sub_5c40e0
    Lifted::new(0x7000cb0150, fn_7000cb0150 as *const (), 0), // +0x5c4150 sub_5c4150
    Lifted::new(0x7000cb01c0, tpl_965::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000cb0240, 0x7001d54968, 0x7000cb0220> as *const (), 0), // +0x5c41c0 sub_5c41c0
    Lifted::new(0x7000cb0240, tpl_1083::<0x8, 0x18, 0x700177c312> as *const (), 0), // +0x5c4240 sub_5c4240
    Lifted::new(0x7000cb02e0, tpl_1199::<0x8, 0x2, 0x7001826180, 0x70017a0290, 0x7001d54ef8, 0x7000cb0311, 0x10, 0x70018261b8, 0x7000cb0336, 0x7000799390, 0x700177c312> as *const (), 0), // +0x5c42e0 sub_5c42e0
    Lifted::new(0x7000cb0390, fn_7000cb0390 as *const (), 0), // +0x5c4390 sub_5c4390
    Lifted::new(0x7000cb03c0, fn_7000cb03c0 as *const (), 0), // +0x5c43c0 sub_5c43c0
    Lifted::new(0x7000cb0590, fn_7000cb0590 as *const (), 0), // +0x5c4590 sub_5c4590
    Lifted::new(0x7000cb0600, tpl_1200::<0x1f0, 0x7000cb8900, 0xd8> as *const (), 0), // +0x5c4600 sub_5c4600
    Lifted::new(0x7000cb0640, tpl_1200::<0xc0, 0x7000cb87e0, 0xd8> as *const (), 0), // +0x5c4640 sub_5c4640
    Lifted::new(0x7000cb0680, fn_7000cb0680 as *const (), 0), // +0x5c4680 sub_5c4680
    Lifted::new(0x7000cb0850, tpl_1201::<0x948, 0x7000cb0680, 0xc10, 0x30, 0x38, 0x20> as *const (), 0), // +0x5c4850 sub_5c4850
    Lifted::new(0x7000cb08a0, tpl_1202::<0x1f0, 0x7000cb8900, 0xd8> as *const (), 0), // +0x5c48a0 sub_5c48a0
    Lifted::new(0x7000cb08e0, tpl_1202::<0xc0, 0x7000cb87e0, 0xd8> as *const (), 0), // +0x5c48e0 sub_5c48e0
    Lifted::new(0x7000cb0920, fn_7000cb0920 as *const (), 0), // +0x5c4920 sub_5c4920
    Lifted::new(0x7000cb0950, fn_7000cb0950 as *const (), 0), // +0x5c4950 sub_5c4950
    Lifted::new(0x7000cb0aa0, fn_7000cb0aa0 as *const (), 0), // +0x5c4aa0 sub_5c4aa0
    Lifted::new(0x7000cb0ce0, tpl_1203::<0x8, 0x2, 0x7000bfab30, 0x10, 0x7000799390, 0x7001d54e98, 0x7001d550d8, 0x7000cb0aa0> as *const (), 0), // +0x5c4ce0 sub_5c4ce0
    Lifted::new(0x7000cb0d90, fn_7000cb0d90 as *const (), 0), // +0x5c4d90 sub_5c4d90
    Lifted::new(0x7000cb0ee0, tpl_1201::<0x948, 0x7000cb0d90, 0xc10, 0x30, 0x38, 0x20> as *const (), 0), // +0x5c4ee0 sub_5c4ee0
    Lifted::new(0x7000cb0f30, tpl_1204::<0x10, 0x7000799390, 0x7001d54e98, 0x8, 0x7001d550d8, 0x7000cb0aa0> as *const (), 0), // +0x5c4f30 sub_5c4f30
    Lifted::new(0x7000cb0fc0, fn_7000cb0fc0 as *const (), 2), // +0x5c4fc0 sub_5c4fc0
    Lifted::new(0x7000cb10a0, fn_7000cb10a0 as *const (), 2), // +0x5c50a0 sub_5c50a0
    Lifted::new(0x7000cb1200, fn_7000cb1200 as *const (), 0), // +0x5c5200 sub_5c5200
    Lifted::new(0x7000cb1240, fn_7000cb1240 as *const (), 0), // +0x5c5240 sub_5c5240
    Lifted::new(0x7000cb12e0, fn_7000cb12e0 as *const (), 1), // +0x5c52e0 sub_5c52e0
    Lifted::new(0x7000cb12f0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cb13f7, 0x7000caff40, 0x70007f0910, 0x7000cb1413> as *const (), 0), // +0x5c52f0 sub_5c52f0
    Lifted::new(0x7000cb1460, tpl_1055::<0x7000cb12f0, 0x70017d37a0, 0x70017790e0, 0x7000cb15d6> as *const (), 0), // +0x5c5460 sub_5c5460
    Lifted::new(0x7000cb1640, fn_7000cb1640 as *const (), 0), // +0x5c5640 sub_5c5640
    Lifted::new(0x7000cb1730, tpl_55::<0x700177c0d0> as *const (), 0), // +0x5c5730 sub_5c5730
    Lifted::new(0x7000cb1770, fn_7000cb1770 as *const (), 0), // +0x5c5770 sub_5c5770
    Lifted::new(0x7000cb1800, tpl_60::<0x7000cb4240, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000cb1942, 0x7000cb4200, 0x7000cb1948> as *const (), 2), // +0x5c5800 sub_5c5800
    Lifted::new(0x7000cb19a0, tpl_1205::<0x7000cb42d0, 0x7000cb1770, 0x700177c312, 0x7001d54968, 0x7000cb1b44, 0x7000cb4220, 0x7000cb1b4a> as *const (), 0), // +0x5c59a0 sub_5c59a0
    Lifted::new(0x7000cb1bc0, tpl_65::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000cb3f90, 0x70017d37c0, 0x70017790e0, 0x7000cb1eaa> as *const (), 2), // +0x5c5bc0 sub_5c5bc0
    Lifted::new(0x7000cb1f70, tpl_68 as *const (), 1), // +0x5c5f70 sub_5c5f70
    Lifted::new(0x7000cb2020, tpl_1206::<0x7001779e20, 0x7000799390, 0x8, 0x10> as *const (), 0), // +0x5c6020 sub_5c6020
    Lifted::new(0x7000cb20e0, tpl_1198::<0x7000799390, 0x8, 0x10> as *const (), 0), // +0x5c60e0 sub_5c60e0
    Lifted::new(0x7000cb2110, fn_7000cb2110 as *const (), 2), // +0x5c6110 sub_5c6110
    Lifted::new(0x7000cb2590, tpl_965::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000cb2610, 0x7001d54968, 0x7000cb25f0> as *const (), 0), // +0x5c6590 sub_5c6590
    Lifted::new(0x7000cb2610, tpl_1207::<0x8, 0x18, 0x7000cb02e0, 0x700177c312> as *const (), 0), // +0x5c6610 sub_5c6610
    Lifted::new(0x7000cb26c0, tpl_1208::<0x8, 0x7000cb1770, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000cb272a> as *const (), 0), // +0x5c66c0 sub_5c66c0
    Lifted::new(0x7000cb2750, tpl_1199::<0x8, 0x2, 0x7001826180, 0x70017a0290, 0x7001d54ef8, 0x7000cb2781, 0x10, 0x70018261b8, 0x7000cb27a6, 0x7000799390, 0x700177c312> as *const (), 0), // +0x5c6750 sub_5c6750
    Lifted::new(0x7000cb2800, fn_7000cb2800 as *const (), 0), // +0x5c6800 sub_5c6800
    Lifted::new(0x7000cb2840, fn_7000cb2840 as *const (), 0), // +0x5c6840 sub_5c6840
    Lifted::new(0x7000cb2940, tpl_1209::<0x7001d54e98, 0x10, 0x8, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5c6940 sub_5c6940
    Lifted::new(0x7000cb29b0, tpl_1209::<0x7001d54e98, 0x10, 0x8, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5c69b0 sub_5c69b0
    Lifted::new(0x7000cb2a20, fn_7000cb2a20 as *const (), 2), // +0x5c6a20 sub_5c6a20
    Lifted::new(0x7000cb2c60, fn_7000cb2c60 as *const (), 2), // +0x5c6c60 sub_5c6c60
    Lifted::new(0x7000cb2ff0, fn_7000cb2ff0 as *const (), 1), // +0x5c6ff0 sub_5c6ff0
    Lifted::new(0x7000cb3030, fn_7000cb3030 as *const (), 0), // +0x5c7030 sub_5c7030
    Lifted::new(0x7000cb31c0, tpl_1210::<0x7001d54e98, 0x8, 0x7001d550d8, 0x10> as *const (), 0), // +0x5c71c0 sub_5c71c0
    Lifted::new(0x7000cb32b0, tpl_1211::<0x7000cb0ce0, 0x7001d54e98, 0x8, 0x7001d550d8, 0x10> as *const (), 0), // +0x5c72b0 sub_5c72b0
    Lifted::new(0x7000cb33a0, fn_7000cb33a0 as *const (), 0), // +0x5c73a0 sub_5c73a0
    Lifted::new(0x7000cb34c0, fn_7000cb34c0 as *const (), 0), // +0x5c74c0 sub_5c74c0
    Lifted::new(0x7000cb3550, fn_7000cb3550 as *const (), 0), // +0x5c7550 sub_5c7550
    Lifted::new(0x7000cb3740, tpl_15 as *const (), 1), // +0x5c7740 sub_5c7740
    Lifted::new(0x7000cb3750, tpl_15 as *const (), 1), // +0x5c7750 sub_5c7750
    Lifted::new(0x7000cb3760, tpl_1203::<0x8, 0x2, 0x7000bfaa30, 0x10, 0x7000799390, 0x7001d54e98, 0x7001d550d8, 0x7000cb34c0> as *const (), 0), // +0x5c7760 sub_5c7760
    Lifted::new(0x7000cb3810, fn_7000cb3810 as *const (), 0), // +0x5c7810 sub_5c7810
    Lifted::new(0x7000cb3920, tpl_1204::<0x10, 0x7000799390, 0x7001d54e98, 0x8, 0x7001d550d8, 0x7000cb34c0> as *const (), 0), // +0x5c7920 sub_5c7920
    Lifted::new(0x7000cb39b0, fn_7000cb39b0 as *const (), 0), // +0x5c79b0 sub_5c79b0
    Lifted::new(0x7000cb39f0, fn_7000cb39f0 as *const (), 0), // +0x5c79f0 sub_5c79f0
    Lifted::new(0x7000cb3aa0, fn_7000cb3aa0 as *const (), 0), // +0x5c7aa0 sub_5c7aa0
    Lifted::new(0x7000cb3ca0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cb3da7, 0x7000cb1f70, 0x70007f0910, 0x7000cb3dc3> as *const (), 0), // +0x5c7ca0 sub_5c7ca0
    Lifted::new(0x7000cb3e10, tpl_278::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000cb3e9d> as *const (), 0), // +0x5c7e10 sub_5c7e10
    Lifted::new(0x7000cb3ed0, fn_7000cb3ed0 as *const (), 0), // +0x5c7ed0 sub_5c7ed0
    Lifted::new(0x7000cb3f90, tpl_286::<0x7000cb3ca0, 0x70017d37a0, 0x70017790e0, 0x7000cb417b> as *const (), 0), // +0x5c7f90 sub_5c7f90
    Lifted::new(0x7000cb4200, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cb4210> as *const (), 0), // +0x5c8200 sub_5c8200
    Lifted::new(0x7000cb4220, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cb4230> as *const (), 0), // +0x5c8220 sub_5c8220
    Lifted::new(0x7000cb4240, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cb428d, 0x70007f0910, 0x7000cb42ab> as *const (), 0), // +0x5c8240 sub_5c8240
    Lifted::new(0x7000cb42d0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cb4324, 0x70007f0910, 0x7000cb433f> as *const (), 0), // +0x5c82d0 sub_5c82d0
    Lifted::new(0x7000cb4360, fn_7000cb4360 as *const (), 1), // +0x5c8360 sub_5c8360
    Lifted::new(0x7000cb4410, fn_7000cb4410 as *const (), 1), // +0x5c8410 sub_5c8410
    Lifted::new(0x7000cb44f0, fn_7000cb44f0 as *const (), 1), // +0x5c84f0 sub_5c84f0
    Lifted::new(0x7000cb4510, fn_7000cb4510 as *const (), 0), // +0x5c8510 sub_5c8510
    Lifted::new(0x7000cb4680, fn_7000cb4680 as *const (), 0), // +0x5c8680 sub_5c8680
    Lifted::new(0x7000cb4710, tpl_1212::<0x7000757a10, 0x7000782350> as *const (), 0), // +0x5c8710 sub_5c8710
    Lifted::new(0x7000cb4750, tpl_1212::<0x7000757a10, 0x7000cbe350> as *const (), 0), // +0x5c8750 sub_5c8750
    Lifted::new(0x7000cb4790, tpl_1213::<0x7000cb6fc0, 0x7000752ef0, 0x70007450b0, 0x7000736190, 0x7000befc30, 0x7000cb8210, 0x700072d8c0> as *const (), 2), // +0x5c8790 sub_5c8790
    Lifted::new(0x7000cb4930, tpl_1213::<0x7000cb6fc0, 0x7000752ef0, 0x70007450b0, 0x7000736190, 0x7000befc30, 0x7000cb8210, 0x700072d8c0> as *const (), 2), // +0x5c8930 sub_5c8930
    Lifted::new(0x7000cb4ad0, fn_7000cb4ad0 as *const (), 0), // +0x5c8ad0 sub_5c8ad0
    Lifted::new(0x7000cb4b10, fn_7000cb4b10 as *const (), 0), // +0x5c8b10 sub_5c8b10
    Lifted::new(0x7000cb4b90, fn_7000cb4b90 as *const (), 0), // +0x5c8b90 sub_5c8b90
    Lifted::new(0x7000cb4c20, fn_7000cb4c20 as *const (), 0), // +0x5c8c20 sub_5c8c20
    Lifted::new(0x7000cb4c80, fn_7000cb4c80 as *const (), 0), // +0x5c8c80 sub_5c8c80
    Lifted::new(0x7000cb4f00, fn_7000cb4f00 as *const (), 0), // +0x5c8f00 sub_5c8f00
    Lifted::new(0x7000cb50e0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x5c90e0 sub_5c90e0
    Lifted::new(0x7000cb5120, tpl_395::<0x7000cb7460, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000cb5265, 0x7000cb7440, 0x7000cb526b> as *const (), 0), // +0x5c9120 sub_5c9120
    Lifted::new(0x7000cb52c0, tpl_1214::<0x7001779e20, 0x7001826248, 0x8, 0xa028, 0x10> as *const (), 0), // +0x5c92c0 sub_5c92c0
    Lifted::new(0x7000cb5380, fn_7000cb5380 as *const (), 0), // +0x5c9380 sub_5c9380
    Lifted::new(0x7000cb54d0, fn_7000cb54d0 as *const (), 0), // +0x5c94d0 sub_5c94d0
    Lifted::new(0x7000cb5690, tpl_33::<0x700177c312> as *const (), 0), // +0x5c9690 sub_5c9690
    Lifted::new(0x7000cb56c0, fn_7000cb56c0 as *const (), 2), // +0x5c96c0 sub_5c96c0
    Lifted::new(0x7000cb5990, fn_7000cb5990 as *const (), 2), // +0x5c9990 sub_5c9990
    Lifted::new(0x7000cb5d60, fn_7000cb5d60 as *const (), 0), // +0x5c9d60 sub_5c9d60
    Lifted::new(0x7000cb5e30, fn_7000cb5e30 as *const (), 0), // +0x5c9e30 sub_5c9e30
    Lifted::new(0x7000cb5e90, fn_7000cb5e90 as *const (), 0), // +0x5c9e90 sub_5c9e90
    Lifted::new(0x7000cb5ee0, fn_7000cb5ee0 as *const (), 0), // +0x5c9ee0 sub_5c9ee0
    Lifted::new(0x7000cb5f50, fn_7000cb5f50 as *const (), 0), // +0x5c9f50 sub_5c9f50
    Lifted::new(0x7000cb60d0, fn_7000cb60d0 as *const (), 0), // +0x5ca0d0 sub_5ca0d0
    Lifted::new(0x7000cb6170, fn_7000cb6170 as *const (), 0), // +0x5ca170 sub_5ca170
    Lifted::new(0x7000cb6210, fn_7000cb6210 as *const (), 0), // +0x5ca210 sub_5ca210
    Lifted::new(0x7000cb62b0, fn_7000cb62b0 as *const (), 0), // +0x5ca2b0 sub_5ca2b0
    Lifted::new(0x7000cb6320, fn_7000cb6320 as *const (), 0), // +0x5ca320 sub_5ca320
    Lifted::new(0x7000cb6360, fn_7000cb6360 as *const (), 0), // +0x5ca360 sub_5ca360
    Lifted::new(0x7000cb64e0, fn_7000cb64e0 as *const (), 0), // +0x5ca4e0 sub_5ca4e0
    Lifted::new(0x7000cb65b0, fn_7000cb65b0 as *const (), 0), // +0x5ca5b0 sub_5ca5b0
    Lifted::new(0x7000cb65d0, fn_7000cb65d0 as *const (), 0), // +0x5ca5d0 sub_5ca5d0
    Lifted::new(0x7000cb6670, fn_7000cb6670 as *const (), 0), // +0x5ca670 sub_5ca670
    Lifted::new(0x7000cb66f0, fn_7000cb66f0 as *const (), 0), // +0x5ca6f0 sub_5ca6f0
    Lifted::new(0x7000cb6760, fn_7000cb6760 as *const (), 0), // +0x5ca760 sub_5ca760
    Lifted::new(0x7000cb6810, fn_7000cb6810 as *const (), 0), // +0x5ca810 sub_5ca810
    Lifted::new(0x7000cb6860, fn_7000cb6860 as *const (), 0), // +0x5ca860 sub_5ca860
    Lifted::new(0x7000cb6940, fn_7000cb6940 as *const (), 0), // +0x5ca940 sub_5ca940
    Lifted::new(0x7000cb69e0, fn_7000cb69e0 as *const (), 0), // +0x5ca9e0 sub_5ca9e0
    Lifted::new(0x7000cb6a90, fn_7000cb6a90 as *const (), 0), // +0x5caa90 sub_5caa90
    Lifted::new(0x7000cb6af0, fn_7000cb6af0 as *const (), 2), // +0x5caaf0 sub_5caaf0
    Lifted::new(0x7000cb6ee0, fn_7000cb6ee0 as *const (), 0), // +0x5caee0 sub_5caee0
    Lifted::new(0x7000cb6f70, fn_7000cb6f70 as *const (), 0), // +0x5caf70 sub_5caf70
    Lifted::new(0x7000cb6fc0, fn_7000cb6fc0 as *const (), 0), // +0x5cafc0 sub_5cafc0
    Lifted::new(0x7000cb70a0, fn_7000cb70a0 as *const (), 0), // +0x5cb0a0 sub_5cb0a0
    Lifted::new(0x7000cb70d0, fn_7000cb70d0 as *const (), 0), // +0x5cb0d0 sub_5cb0d0
    Lifted::new(0x7000cb7360, fn_7000cb7360 as *const (), 1), // +0x5cb360 sub_5cb360
    Lifted::new(0x7000cb7390, tpl_914::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000cb7417> as *const (), 0), // +0x5cb390 sub_5cb390
    Lifted::new(0x7000cb7440, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cb7450> as *const (), 0), // +0x5cb440 sub_5cb440
    Lifted::new(0x7000cb7460, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cb74b4, 0x70007f0910, 0x7000cb74cf> as *const (), 0), // +0x5cb460 sub_5cb460
    Lifted::new(0x7000cb74f0, fn_7000cb74f0 as *const (), 0), // +0x5cb4f0 sub_5cb4f0
    Lifted::new(0x7000cb7560, fn_7000cb7560 as *const (), 1), // +0x5cb560 sub_5cb560
    Lifted::new(0x7000cb7590, fn_7000cb7590 as *const (), 0), // +0x5cb590 sub_5cb590
    Lifted::new(0x7000cb7640, fn_7000cb7640 as *const (), 0), // +0x5cb640 sub_5cb640
    Lifted::new(0x7000cb7670, fn_7000cb7670 as *const (), 0), // +0x5cb670 sub_5cb670
    Lifted::new(0x7000cb76e0, fn_7000cb76e0 as *const (), 0), // +0x5cb6e0 sub_5cb6e0
    Lifted::new(0x7000cb7700, fn_7000cb7700 as *const (), 0), // +0x5cb700 sub_5cb700
    Lifted::new(0x7000cb7740, fn_7000cb7740 as *const (), 1), // +0x5cb740 sub_5cb740
    Lifted::new(0x7000cb7760, fn_7000cb7760 as *const (), 0), // +0x5cb760 sub_5cb760
    Lifted::new(0x7000cb77d0, tpl_1215::<0x7001779e20, 0x8, 0x28, 0x10, 0x18, 0xa0> as *const (), 0), // +0x5cb7d0 sub_5cb7d0
    Lifted::new(0x7000cb7860, tpl_1215::<0x7001779e20, 0x8, 0x28, 0x10, 0x18, 0x98> as *const (), 0), // +0x5cb860 sub_5cb860
    Lifted::new(0x7000cb78f0, tpl_1216::<0x68, 0x7000cb7ef0> as *const (), 0), // +0x5cb8f0 sub_5cb8f0
    Lifted::new(0x7000cb7900, tpl_1216::<0x60, 0x7000cb7ef0> as *const (), 0), // +0x5cb900 sub_5cb900
    Lifted::new(0x7000cb7910, tpl_1217::<0x7001779e20, 0x8, 0x10, 0x70017d3030, 0x18, 0x90> as *const (), 0), // +0x5cb910 sub_5cb910
    Lifted::new(0x7000cb79a0, tpl_1217::<0x7001779e20, 0x8, 0x10, 0x70017d3030, 0x18, 0x88> as *const (), 0), // +0x5cb9a0 sub_5cb9a0
    Lifted::new(0x7000cb7a30, fn_7000cb7a30 as *const (), 0), // +0x5cba30 sub_5cba30
    Lifted::new(0x7000cb7af0, fn_7000cb7af0 as *const (), 0), // +0x5cbaf0 sub_5cbaf0
    Lifted::new(0x7000cb7c60, tpl_1218::<0xa0, 0x700098a360, 0x700177c312> as *const (), 0), // +0x5cbc60 sub_5cbc60
    Lifted::new(0x7000cb7cc0, tpl_1218::<0x98, 0x700098a360, 0x700177c312> as *const (), 0), // +0x5cbcc0 sub_5cbcc0
    Lifted::new(0x7000cb7d20, tpl_1216::<0x68, 0x7000cb8700> as *const (), 0), // +0x5cbd20 sub_5cbd20
    Lifted::new(0x7000cb7d30, tpl_1216::<0x60, 0x7000cb8700> as *const (), 0), // +0x5cbd30 sub_5cbd30
    Lifted::new(0x7000cb7d40, fn_7000cb7d40 as *const (), 0), // +0x5cbd40 sub_5cbd40
    Lifted::new(0x7000cb7d70, fn_7000cb7d70 as *const (), 0), // +0x5cbd70 sub_5cbd70
    Lifted::new(0x7000cb7dc0, fn_7000cb7dc0 as *const (), 2), // +0x5cbdc0 sub_5cbdc0
    Lifted::new(0x7000cb7ef0, fn_7000cb7ef0 as *const (), 0), // +0x5cbef0 sub_5cbef0
    Lifted::new(0x7000cb8000, tpl_1216::<0x80, 0x7000cb7ef0> as *const (), 0), // +0x5cc000 sub_5cc000
    Lifted::new(0x7000cb8010, fn_7000cb8010 as *const (), 0), // +0x5cc010 sub_5cc010
    Lifted::new(0x7000cb8050, fn_7000cb8050 as *const (), 0), // +0x5cc050 sub_5cc050
    Lifted::new(0x7000cb80a0, fn_7000cb80a0 as *const (), 0), // +0x5cc0a0 sub_5cc0a0
    Lifted::new(0x7000cb8180, fn_7000cb8180 as *const (), 0), // +0x5cc180 sub_5cc180
    Lifted::new(0x7000cb8200, tpl_1076 as *const (), 1), // +0x5cc200 sub_5cc200
    Lifted::new(0x7000cb8210, fn_7000cb8210 as *const (), 0), // +0x5cc210 sub_5cc210
    Lifted::new(0x7000cb8430, fn_7000cb8430 as *const (), 0), // +0x5cc430 sub_5cc430
    Lifted::new(0x7000cb84b0, fn_7000cb84b0 as *const (), 0), // +0x5cc4b0 sub_5cc4b0
    Lifted::new(0x7000cb8700, fn_7000cb8700 as *const (), 0), // +0x5cc700 sub_5cc700
    Lifted::new(0x7000cb87a0, tpl_1216::<0x80, 0x7000cb8700> as *const (), 0), // +0x5cc7a0 sub_5cc7a0
    Lifted::new(0x7000cb87b0, fn_7000cb87b0 as *const (), 0), // +0x5cc7b0 sub_5cc7b0
    Lifted::new(0x7000cb87e0, fn_7000cb87e0 as *const (), 1), // +0x5cc7e0 sub_5cc7e0
    Lifted::new(0x7000cb8810, tpl_155::<0x7000cb80a0> as *const (), 0), // +0x5cc810 sub_5cc810
    Lifted::new(0x7000cb8820, fn_7000cb8820 as *const (), 0), // +0x5cc820 sub_5cc820
    Lifted::new(0x7000cb88f0, tpl_157::<0x7000cb80a0> as *const (), 0), // +0x5cc8f0 sub_5cc8f0
    Lifted::new(0x7000cb8900, fn_7000cb8900 as *const (), 1), // +0x5cc900 sub_5cc900
    Lifted::new(0x7000cb8930, tpl_287 as *const (), 1), // +0x5cc930 sub_5cc930
    Lifted::new(0x7000cb8c50, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cb8e28, 0x7000cbc0e0, 0x7000cb8e2e, 0x70007f0910, 0x7000cb8e34> as *const (), 2), // +0x5ccc50 sub_5ccc50
    Lifted::new(0x7000cb8eb0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cb9084, 0x7000cbc100, 0x7000cb908a, 0x70007f0910, 0x7000cb9090> as *const (), 0), // +0x5cceb0 sub_5cceb0
    Lifted::new(0x7000cb9110, tpl_392::<0x700177c0e0> as *const (), 0), // +0x5cd110 sub_5cd110
    Lifted::new(0x7000cb9140, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000b731b0, 0x70017d37c0, 0x70017790e0, 0x7000cb9421> as *const (), 2), // +0x5cd140 sub_5cd140
    Lifted::new(0x7000cb94e0, fn_7000cb94e0 as *const (), 0), // +0x5cd4e0 sub_5cd4e0
    Lifted::new(0x7000cb9570, fn_7000cb9570 as *const (), 1), // +0x5cd570 sub_5cd570
    Lifted::new(0x7000cb95b0, fn_7000cb95b0 as *const (), 0), // +0x5cd5b0 sub_5cd5b0
    Lifted::new(0x7000cb9650, fn_7000cb9650 as *const (), 0), // +0x5cd650 sub_5cd650
    Lifted::new(0x7000cb9720, tpl_1219::<0x7001826660, 0x10, 0x700072ea80, 0x18> as *const (), 0), // +0x5cd720 sub_5cd720
    Lifted::new(0x7000cb9760, fn_7000cb9760 as *const (), 0), // +0x5cd760 sub_5cd760
    Lifted::new(0x7000cb9800, fn_7000cb9800 as *const (), 1), // +0x5cd800 sub_5cd800
    Lifted::new(0x7000cb9830, fn_7000cb9830 as *const (), 0), // +0x5cd830 sub_5cd830
    Lifted::new(0x7000cb9930, tpl_1220::<0x7001826660, 0x10, 0x700072ea80, 0x18, 0x700177c312> as *const (), 0), // +0x5cd930 sub_5cd930
    Lifted::new(0x7000cb9990, fn_7000cb9990 as *const (), 2), // +0x5cd990 sub_5cd990
    Lifted::new(0x7000cb9bc0, fn_7000cb9bc0 as *const (), 0), // +0x5cdbc0 sub_5cdbc0
    Lifted::new(0x7000cb9c50, fn_7000cb9c50 as *const (), 2), // +0x5cdc50 sub_5cdc50
    Lifted::new(0x7000cba010, fn_7000cba010 as *const (), 0), // +0x5ce010 sub_5ce010
    Lifted::new(0x7000cba500, fn_7000cba500 as *const (), 0), // +0x5ce500 sub_5ce500
    Lifted::new(0x7000cba590, fn_7000cba590 as *const (), 1), // +0x5ce590 sub_5ce590
    Lifted::new(0x7000cba5f0, fn_7000cba5f0 as *const (), 1), // +0x5ce5f0 sub_5ce5f0
    Lifted::new(0x7000cba690, fn_7000cba690 as *const (), 0), // +0x5ce690 sub_5ce690
    Lifted::new(0x7000cba7c0, fn_7000cba7c0 as *const (), 2), // +0x5ce7c0 sub_5ce7c0
    Lifted::new(0x7000cbadd0, fn_7000cbadd0 as *const (), 0), // +0x5cedd0 sub_5cedd0
    Lifted::new(0x7000cbae50, fn_7000cbae50 as *const (), 0), // +0x5cee50 sub_5cee50
    Lifted::new(0x7000cbaf40, fn_7000cbaf40 as *const (), 2), // +0x5cef40 sub_5cef40
    Lifted::new(0x7000cbb040, fn_7000cbb040 as *const (), 2), // +0x5cf040 sub_5cf040
    Lifted::new(0x7000cbb250, fn_7000cbb250 as *const (), 0), // +0x5cf250 sub_5cf250
    Lifted::new(0x7000cbb330, fn_7000cbb330 as *const (), 0), // +0x5cf330 sub_5cf330
    Lifted::new(0x7000cbb360, fn_7000cbb360 as *const (), 2), // +0x5cf360 sub_5cf360
    Lifted::new(0x7000cbb840, fn_7000cbb840 as *const (), 2), // +0x5cf840 sub_5cf840
    Lifted::new(0x7000cbba30, fn_7000cbba30 as *const (), 0), // +0x5cfa30 sub_5cfa30
    Lifted::new(0x7000cbbba0, fn_7000cbbba0 as *const (), 1), // +0x5cfba0 sub_5cfba0
    Lifted::new(0x7000cbbbd0, fn_7000cbbbd0 as *const (), 2), // +0x5cfbd0 sub_5cfbd0
    Lifted::new(0x7000cbbef0, tpl_753::<0x7001826680, 0x8> as *const (), 1), // +0x5cfef0 sub_5cfef0
    Lifted::new(0x7000cbbf10, tpl_918::<0x70018266c0, 0x8> as *const (), 3), // +0x5cff10 sub_5cff10
    Lifted::new(0x7000cbbf30, tpl_754::<0x700177c312> as *const (), 0), // +0x5cff30 sub_5cff30
    Lifted::new(0x7000cbbf50, tpl_919::<0x700177c312> as *const (), 0), // +0x5cff50 sub_5cff50
    Lifted::new(0x7000cbbf70, fn_7000cbbf70 as *const (), 0), // +0x5cff70 sub_5cff70
    Lifted::new(0x7000cbbf90, fn_7000cbbf90 as *const (), 0), // +0x5cff90 sub_5cff90
    Lifted::new(0x7000cbc060, tpl_52::<0x8> as *const (), 1), // +0x5d0060 sub_5d0060
    Lifted::new(0x7000cbc070, tpl_52::<0x8> as *const (), 1), // +0x5d0070 sub_5d0070
    Lifted::new(0x7000cbc080, tpl_753::<0x7001826680, 0x8> as *const (), 1), // +0x5d0080 sub_5d0080
    Lifted::new(0x7000cbc0a0, tpl_918::<0x70018266c0, 0x8> as *const (), 3), // +0x5d00a0 sub_5d00a0
    Lifted::new(0x7000cbc0c0, tpl_315::<0x7001c6bf10> as *const (), 1), // +0x5d00c0 sub_5d00c0
    Lifted::new(0x7000cbc0d0, tpl_315::<0x7001c6bec0> as *const (), 1), // +0x5d00d0 sub_5d00d0
    Lifted::new(0x7000cbc0e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cbc0f0> as *const (), 0), // +0x5d00e0 sub_5d00e0
    Lifted::new(0x7000cbc100, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cbc110> as *const (), 0), // +0x5d0100 sub_5d0100
    Lifted::new(0x7000cbc120, fn_7000cbc120 as *const (), 0), // +0x5d0120 sub_5d0120
    Lifted::new(0x7000cbc1d0, fn_7000cbc1d0 as *const (), 0), // +0x5d01d0 sub_5d01d0
    Lifted::new(0x7000cbc240, fn_7000cbc240 as *const (), 0), // +0x5d0240 sub_5d0240
    Lifted::new(0x7000cbc2a0, tpl_899::<0x7000cbc2b0> as *const (), 0), // +0x5d02a0 sub_5d02a0
    Lifted::new(0x7000cbc2b0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x5d02b0 sub_5d02b0
    Lifted::new(0x7000cbc2f0, fn_7000cbc2f0 as *const (), 2), // +0x5d02f0 sub_5d02f0
    Lifted::new(0x7000cbc3d0, fn_7000cbc3d0 as *const (), 2), // +0x5d03d0 sub_5d03d0
    Lifted::new(0x7000cbc460, fn_7000cbc460 as *const (), 2), // +0x5d0460 sub_5d0460
    Lifted::new(0x7000cbc5e0, fn_7000cbc5e0 as *const (), 2), // +0x5d05e0 sub_5d05e0
    Lifted::new(0x7000cbc7a0, fn_7000cbc7a0 as *const (), 2), // +0x5d07a0 sub_5d07a0
    Lifted::new(0x7000cbc8a0, fn_7000cbc8a0 as *const (), 2), // +0x5d08a0 sub_5d08a0
    Lifted::new(0x7000cbcc20, fn_7000cbcc20 as *const (), 3), // +0x5d0c20 sub_5d0c20
    Lifted::new(0x7000cbcc60, fn_7000cbcc60 as *const (), 3), // +0x5d0c60 sub_5d0c60
    Lifted::new(0x7000cbcca0, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cbce78, 0x7000cbf220, 0x7000cbce7e, 0x70007f0910, 0x7000cbce84> as *const (), 2), // +0x5d0ca0 sub_5d0ca0
    Lifted::new(0x7000cbcf00, fn_7000cbcf00 as *const (), 2), // +0x5d0f00 sub_5d0f00
    Lifted::new(0x7000cbd100, fn_7000cbd100 as *const (), 0), // +0x5d1100 sub_5d1100
    Lifted::new(0x7000cbd230, tpl_133::<0x700070d940, 0x7000701460> as *const (), 0), // +0x5d1230 sub_5d1230
    Lifted::new(0x7000cbd270, tpl_33::<0x7000701460> as *const (), 0), // +0x5d1270 sub_5d1270
    Lifted::new(0x7000cbd2a0, tpl_133::<0x700070d940, 0x7000701460> as *const (), 0), // +0x5d12a0 sub_5d12a0
    Lifted::new(0x7000cbd2e0, fn_7000cbd2e0 as *const (), 2), // +0x5d12e0 sub_5d12e0
    Lifted::new(0x7000cbd450, fn_7000cbd450 as *const (), 0), // +0x5d1450 sub_5d1450
    Lifted::new(0x7000cbd4a0, fn_7000cbd4a0 as *const (), 0), // +0x5d14a0 sub_5d14a0
    Lifted::new(0x7000cbd690, fn_7000cbd690 as *const (), 0), // +0x5d1690 sub_5d1690
    Lifted::new(0x7000cbd7e0, fn_7000cbd7e0 as *const (), 0), // +0x5d17e0 sub_5d17e0
    Lifted::new(0x7000cbd810, fn_7000cbd810 as *const (), 0), // +0x5d1810 sub_5d1810
    Lifted::new(0x7000cbd910, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cbd921> as *const (), 0), // +0x5d1910 sub_5d1910
    Lifted::new(0x7000cbd930, fn_7000cbd930 as *const (), 0), // +0x5d1930 sub_5d1930
    Lifted::new(0x7000cbdab0, fn_7000cbdab0 as *const (), 0), // +0x5d1ab0 sub_5d1ab0
    Lifted::new(0x7000cbdb90, fn_7000cbdb90 as *const (), 0), // +0x5d1b90 sub_5d1b90
    Lifted::new(0x7000cbdc00, fn_7000cbdc00 as *const (), 0), // +0x5d1c00 sub_5d1c00
    Lifted::new(0x7000cbdd00, fn_7000cbdd00 as *const (), 0), // +0x5d1d00 sub_5d1d00
    Lifted::new(0x7000cbe000, fn_7000cbe000 as *const (), 0), // +0x5d2000 sub_5d2000
    Lifted::new(0x7000cbe020, fn_7000cbe020 as *const (), 0), // +0x5d2020 sub_5d2020
    Lifted::new(0x7000cbe0b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cbe0c1> as *const (), 0), // +0x5d20b0 sub_5d20b0
    Lifted::new(0x7000cbe0d0, fn_7000cbe0d0 as *const (), 0), // +0x5d20d0 sub_5d20d0
    Lifted::new(0x7000cbe170, fn_7000cbe170 as *const (), 0), // +0x5d2170 sub_5d2170
    Lifted::new(0x7000cbe350, fn_7000cbe350 as *const (), 0), // +0x5d2350 sub_5d2350
    Lifted::new(0x7000cbe410, fn_7000cbe410 as *const (), 1), // +0x5d2410 sub_5d2410
    Lifted::new(0x7000cbe540, fn_7000cbe540 as *const (), 0), // +0x5d2540 sub_5d2540
    Lifted::new(0x7000cbe660, fn_7000cbe660 as *const (), 1), // +0x5d2660 sub_5d2660
    Lifted::new(0x7000cbe750, fn_7000cbe750 as *const (), 0), // +0x5d2750 sub_5d2750
    Lifted::new(0x7000cbe7a0, fn_7000cbe7a0 as *const (), 1), // +0x5d27a0 sub_5d27a0
    Lifted::new(0x7000cbe8c0, fn_7000cbe8c0 as *const (), 0), // +0x5d28c0 sub_5d28c0
    Lifted::new(0x7000cbe970, tpl_322::<0x378, 0x380> as *const (), 1), // +0x5d2970 sub_5d2970
    Lifted::new(0x7000cbe990, fn_7000cbe990 as *const (), 0), // +0x5d2990 sub_5d2990
    Lifted::new(0x7000cbea90, fn_7000cbea90 as *const (), 0), // +0x5d2a90 sub_5d2a90
    Lifted::new(0x7000cbeb80, fn_7000cbeb80 as *const (), 0), // +0x5d2b80 sub_5d2b80
    Lifted::new(0x7000cbec30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cbec41> as *const (), 0), // +0x5d2c30 sub_5d2c30
    Lifted::new(0x7000cbec50, tpl_260::<0x18> as *const (), 0), // +0x5d2c50 sub_5d2c50
    Lifted::new(0x7000cbec60, fn_7000cbec60 as *const (), 1), // +0x5d2c60 sub_5d2c60
    Lifted::new(0x7000cbee90, fn_7000cbee90 as *const (), 0), // +0x5d2e90 sub_5d2e90
    Lifted::new(0x7000cbef50, fn_7000cbef50 as *const (), 0), // +0x5d2f50 sub_5d2f50
    Lifted::new(0x7000cbefc0, fn_7000cbefc0 as *const (), 0), // +0x5d2fc0 sub_5d2fc0
    Lifted::new(0x7000cbf0e0, fn_7000cbf0e0 as *const (), 0), // +0x5d30e0 sub_5d30e0
    Lifted::new(0x7000cbf120, fn_7000cbf120 as *const (), 0), // +0x5d3120 sub_5d3120
    Lifted::new(0x7000cbf210, tpl_15 as *const (), 1), // +0x5d3210 sub_5d3210
    Lifted::new(0x7000cbf220, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cbf230> as *const (), 0), // +0x5d3220 sub_5d3220
    Lifted::new(0x7000cbf240, fn_7000cbf240 as *const (), 1), // +0x5d3240 sub_5d3240
    Lifted::new(0x7000cbf2a0, fn_7000cbf2a0 as *const (), 0), // +0x5d32a0 sub_5d32a0
    Lifted::new(0x7000cbf560, fn_7000cbf560 as *const (), 0), // +0x5d3560 sub_5d3560
    Lifted::new(0x7000cbf640, tpl_1221::<0xf, 0x7, 0xb, 0xffffffffffffffff, 0x700070a940, 0x7000cc8d70> as *const (), 0), // +0x5d3640 sub_5d3640
    Lifted::new(0x7000cbf720, tpl_1221::<0xf, 0x7, 0xb, 0xffffffffffffffff, 0x700070a940, 0x7000cc8d70> as *const (), 0), // +0x5d3720 sub_5d3720
    Lifted::new(0x7000cbf800, fn_7000cbf800 as *const (), 2), // +0x5d3800 sub_5d3800
    Lifted::new(0x7000cbfb90, tpl_68 as *const (), 1), // +0x5d3b90 sub_5d3b90
    Lifted::new(0x7000cbfc70, fn_7000cbfc70 as *const (), 0), // +0x5d3c70 sub_5d3c70
    Lifted::new(0x7000cbfea0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000cbff43> as *const (), 0), // +0x5d3ea0 sub_5d3ea0
    Lifted::new(0x7000cbff70, tpl_916::<0x7000cbfea0, 0x700177c312> as *const (), 0), // +0x5d3f70 sub_5d3f70
    Lifted::new(0x7000cbffe0, fn_7000cbffe0 as *const (), 2), // +0x5d3fe0 sub_5d3fe0
    Lifted::new(0x7000cc0150, fn_7000cc0150 as *const (), 0), // +0x5d4150 sub_5d4150
    Lifted::new(0x7000cc0230, fn_7000cc0230 as *const (), 0), // +0x5d4230 sub_5d4230
    Lifted::new(0x7000cc0440, fn_7000cc0440 as *const (), 0), // +0x5d4440 sub_5d4440
    Lifted::new(0x7000cc0700, fn_7000cc0700 as *const (), 0), // +0x5d4700 sub_5d4700
    Lifted::new(0x7000cc0de0, fn_7000cc0de0 as *const (), 0), // +0x5d4de0 sub_5d4de0
    Lifted::new(0x7000cc0ea0, fn_7000cc0ea0 as *const (), 2), // +0x5d4ea0 sub_5d4ea0
    Lifted::new(0x7000cc3430, fn_7000cc3430 as *const (), 0), // +0x5d7430 sub_5d7430
    Lifted::new(0x7000cc3620, fn_7000cc3620 as *const (), 0), // +0x5d7620 sub_5d7620
    Lifted::new(0x7000cc3660, fn_7000cc3660 as *const (), 0), // +0x5d7660 sub_5d7660
    Lifted::new(0x7000cc39b0, fn_7000cc39b0 as *const (), 1), // +0x5d79b0 sub_5d79b0
    Lifted::new(0x7000cc3af0, fn_7000cc3af0 as *const (), 0), // +0x5d7af0 sub_5d7af0
    Lifted::new(0x7000cc3c60, fn_7000cc3c60 as *const (), 2), // +0x5d7c60 sub_5d7c60
    Lifted::new(0x7000cc6870, fn_7000cc6870 as *const (), 0), // +0x5da870 sub_5da870
    Lifted::new(0x7000cc6950, fn_7000cc6950 as *const (), 0), // +0x5da950 sub_5da950
    Lifted::new(0x7000cc6b20, fn_7000cc6b20 as *const (), 0), // +0x5dab20 sub_5dab20
    Lifted::new(0x7000cc6bd0, fn_7000cc6bd0 as *const (), 0), // +0x5dabd0 sub_5dabd0
    Lifted::new(0x7000cc6d50, fn_7000cc6d50 as *const (), 0), // +0x5dad50 sub_5dad50
    Lifted::new(0x7000cc6df0, fn_7000cc6df0 as *const (), 0), // +0x5dadf0 sub_5dadf0
    Lifted::new(0x7000cc6f90, fn_7000cc6f90 as *const (), 0), // +0x5daf90 sub_5daf90
    Lifted::new(0x7000cc7200, fn_7000cc7200 as *const (), 0), // +0x5db200 sub_5db200
    Lifted::new(0x7000cc7500, fn_7000cc7500 as *const (), 0), // +0x5db500 sub_5db500
    Lifted::new(0x7000cc7880, fn_7000cc7880 as *const (), 0), // +0x5db880 sub_5db880
    Lifted::new(0x7000cc79c0, fn_7000cc79c0 as *const (), 0), // +0x5db9c0 sub_5db9c0
    Lifted::new(0x7000cc7a40, fn_7000cc7a40 as *const (), 0), // +0x5dba40 sub_5dba40
    Lifted::new(0x7000cc8140, fn_7000cc8140 as *const (), 0), // +0x5dc140 sub_5dc140
    Lifted::new(0x7000cc81f0, fn_7000cc81f0 as *const (), 0), // +0x5dc1f0 sub_5dc1f0
    Lifted::new(0x7000cc8330, fn_7000cc8330 as *const (), 0), // +0x5dc330 sub_5dc330
    Lifted::new(0x7000cc8490, fn_7000cc8490 as *const (), 2), // +0x5dc490 sub_5dc490
    Lifted::new(0x7000cc8a50, fn_7000cc8a50 as *const (), 0), // +0x5dca50 sub_5dca50
    Lifted::new(0x7000cc8cb0, fn_7000cc8cb0 as *const (), 0), // +0x5dccb0 sub_5dccb0
    Lifted::new(0x7000cc8d70, fn_7000cc8d70 as *const (), 0), // +0x5dcd70 sub_5dcd70
    Lifted::new(0x7000cc8fc0, fn_7000cc8fc0 as *const (), 0), // +0x5dcfc0 sub_5dcfc0
    Lifted::new(0x7000cc9090, fn_7000cc9090 as *const (), 0), // +0x5dd090 sub_5dd090
    Lifted::new(0x7000cc92c0, fn_7000cc92c0 as *const (), 0), // +0x5dd2c0 sub_5dd2c0
    Lifted::new(0x7000cc93d0, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000cc93e8> as *const (), 0), // +0x5dd3d0 sub_5dd3d0
    Lifted::new(0x7000cc93f0, tpl_15 as *const (), 1), // +0x5dd3f0 sub_5dd3f0
    Lifted::new(0x7000cc9400, fn_7000cc9400 as *const (), 1), // +0x5dd400 sub_5dd400
    Lifted::new(0x7000cc9410, tpl_15 as *const (), 1), // +0x5dd410 sub_5dd410
    Lifted::new(0x7000cc9420, tpl_1222::<0x48> as *const (), 1), // +0x5dd420 sub_5dd420
    Lifted::new(0x7000cc9440, tpl_1222::<0x48> as *const (), 1), // +0x5dd440 sub_5dd440
    Lifted::new(0x7000cc9460, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cc9567, 0x7000cbfb90, 0x70007f0910, 0x7000cc9583> as *const (), 0), // +0x5dd460 sub_5dd460
    Lifted::new(0x7000cc95d0, tpl_1159::<0x7000cc9460, 0x70017d37a0, 0x70017790e0, 0x7000cc979e> as *const (), 0), // +0x5dd5d0 sub_5dd5d0
    Lifted::new(0x7000cc9820, fn_7000cc9820 as *const (), 1), // +0x5dd820 sub_5dd820
    Lifted::new(0x7000cc9890, fn_7000cc9890 as *const (), 0), // +0x5dd890 sub_5dd890
    Lifted::new(0x7000cc9970, fn_7000cc9970 as *const (), 0), // +0x5dd970 sub_5dd970
    Lifted::new(0x7000cc9ad0, fn_7000cc9ad0 as *const (), 0), // +0x5ddad0 sub_5ddad0
    Lifted::new(0x7000cc9cb0, fn_7000cc9cb0 as *const (), 0), // +0x5ddcb0 sub_5ddcb0
    Lifted::new(0x7000cc9f40, fn_7000cc9f40 as *const (), 0), // +0x5ddf40 sub_5ddf40
    Lifted::new(0x7000cca290, fn_7000cca290 as *const (), 0), // +0x5de290 sub_5de290
    Lifted::new(0x7000cca490, tpl_1177::<0x8> as *const (), 1), // +0x5de490 sub_5de490
    Lifted::new(0x7000cca530, tpl_1178::<0x7000cca490> as *const (), 0), // +0x5de530 sub_5de530
    Lifted::new(0x7000cca890, tpl_1179 as *const (), 1), // +0x5de890 sub_5de890
    Lifted::new(0x7000cca9c0, tpl_1180::<0x7000cca530, 0x7000cca9c0, 0x7000cca890> as *const (), 2), // +0x5de9c0 sub_5de9c0
    Lifted::new(0x7000ccadb0, fn_7000ccadb0 as *const (), 2), // +0x5dedb0 sub_5dedb0
    Lifted::new(0x7000ccaee0, fn_7000ccaee0 as *const (), 0), // +0x5deee0 sub_5deee0
    Lifted::new(0x7000ccaf60, fn_7000ccaf60 as *const (), 0), // +0x5def60 sub_5def60
    Lifted::new(0x7000ccafe0, fn_7000ccafe0 as *const (), 0), // +0x5defe0 sub_5defe0
    Lifted::new(0x7000ccb170, fn_7000ccb170 as *const (), 1), // +0x5df170 sub_5df170
    Lifted::new(0x7000ccb1b0, fn_7000ccb1b0 as *const (), 0), // +0x5df1b0 sub_5df1b0
    Lifted::new(0x7000ccb1e0, fn_7000ccb1e0 as *const (), 0), // +0x5df1e0 sub_5df1e0
    Lifted::new(0x7000ccb260, fn_7000ccb260 as *const (), 0), // +0x5df260 sub_5df260
    Lifted::new(0x7000ccb2f0, fn_7000ccb2f0 as *const (), 0), // +0x5df2f0 sub_5df2f0
    Lifted::new(0x7000ccb3d0, fn_7000ccb3d0 as *const (), 1), // +0x5df3d0 sub_5df3d0
    Lifted::new(0x7000ccb410, fn_7000ccb410 as *const (), 0), // +0x5df410 sub_5df410
    Lifted::new(0x7000ccb4c0, fn_7000ccb4c0 as *const (), 1), // +0x5df4c0 sub_5df4c0
    Lifted::new(0x7000ccb4f0, fn_7000ccb4f0 as *const (), 0), // +0x5df4f0 sub_5df4f0
    Lifted::new(0x7000ccb680, fn_7000ccb680 as *const (), 0), // +0x5df680 sub_5df680
    Lifted::new(0x7000ccb6f0, fn_7000ccb6f0 as *const (), 0), // +0x5df6f0 sub_5df6f0
    Lifted::new(0x7000ccb830, fn_7000ccb830 as *const (), 2), // +0x5df830 sub_5df830
    Lifted::new(0x7000ccb9b0, fn_7000ccb9b0 as *const (), 0), // +0x5df9b0 sub_5df9b0
    Lifted::new(0x7000ccbac0, fn_7000ccbac0 as *const (), 0), // +0x5dfac0 sub_5dfac0
    Lifted::new(0x7000ccbb80, fn_7000ccbb80 as *const (), 0), // +0x5dfb80 sub_5dfb80
    Lifted::new(0x7000ccbc10, fn_7000ccbc10 as *const (), 0), // +0x5dfc10 sub_5dfc10
    Lifted::new(0x7000ccbd40, tpl_1007::<0xe8, 0xf0, 0xfffffffffffffffc, 0x7000772040> as *const (), 0), // +0x5dfd40 sub_5dfd40
    Lifted::new(0x7000ccbd80, fn_7000ccbd80 as *const (), 0), // +0x5dfd80 sub_5dfd80
    Lifted::new(0x7000ccbec0, fn_7000ccbec0 as *const (), 0), // +0x5dfec0 sub_5dfec0
    Lifted::new(0x7000ccbef0, fn_7000ccbef0 as *const (), 2), // +0x5dfef0 sub_5dfef0
    Lifted::new(0x7000ccc1c0, fn_7000ccc1c0 as *const (), 0), // +0x5e01c0 sub_5e01c0
    Lifted::new(0x7000ccc410, fn_7000ccc410 as *const (), 0), // +0x5e0410 sub_5e0410
    Lifted::new(0x7000ccc730, fn_7000ccc730 as *const (), 2), // +0x5e0730 sub_5e0730
    Lifted::new(0x7000ccc830, fn_7000ccc830 as *const (), 0), // +0x5e0830 sub_5e0830
    Lifted::new(0x7000ccc8a0, fn_7000ccc8a0 as *const (), 0), // +0x5e08a0 sub_5e08a0
    Lifted::new(0x7000ccc960, fn_7000ccc960 as *const (), 0), // +0x5e0960 sub_5e0960
    Lifted::new(0x7000cccac0, fn_7000cccac0 as *const (), 0), // +0x5e0ac0 sub_5e0ac0
    Lifted::new(0x7000cccb40, fn_7000cccb40 as *const (), 0), // +0x5e0b40 sub_5e0b40
    Lifted::new(0x7000cccba0, fn_7000cccba0 as *const (), 0), // +0x5e0ba0 sub_5e0ba0
    Lifted::new(0x7000cccc20, tpl_1223::<0x700076a240, 0x7001826fe0, 0x70017a0290, 0x7001d54ef8, 0x7000cccd74> as *const (), 0), // +0x5e0c20 sub_5e0c20
    Lifted::new(0x7000cccdd0, tpl_1223::<0x700076e7d0, 0x7001826fe0, 0x70017a0290, 0x7001d54ef8, 0x7000cccf24> as *const (), 0), // +0x5e0dd0 sub_5e0dd0
    Lifted::new(0x7000cccf80, tpl_1223::<0x700078f610, 0x7001826fe0, 0x70017a0290, 0x7001d54ef8, 0x7000ccd0d4> as *const (), 0), // +0x5e0f80 sub_5e0f80
    Lifted::new(0x7000ccd130, fn_7000ccd130 as *const (), 0), // +0x5e1130 sub_5e1130
    Lifted::new(0x7000ccd240, fn_7000ccd240 as *const (), 0), // +0x5e1240 sub_5e1240
    Lifted::new(0x7000ccd320, fn_7000ccd320 as *const (), 0), // +0x5e1320 sub_5e1320
    Lifted::new(0x7000ccd360, fn_7000ccd360 as *const (), 2), // +0x5e1360 sub_5e1360
    Lifted::new(0x7000ccd700, fn_7000ccd700 as *const (), 0), // +0x5e1700 sub_5e1700
    Lifted::new(0x7000ccd800, fn_7000ccd800 as *const (), 2), // +0x5e1800 sub_5e1800
    Lifted::new(0x7000ccd9c0, fn_7000ccd9c0 as *const (), 0), // +0x5e19c0 sub_5e19c0
    Lifted::new(0x7000ccdc00, fn_7000ccdc00 as *const (), 0), // +0x5e1c00 sub_5e1c00
    Lifted::new(0x7000ccdc90, tpl_1007::<0xe8, 0xf0, 0xfffffffffffffffc, 0x7000ce2ee0> as *const (), 0), // +0x5e1c90 sub_5e1c90
    Lifted::new(0x7000ccdcd0, fn_7000ccdcd0 as *const (), 0), // +0x5e1cd0 sub_5e1cd0
    Lifted::new(0x7000ccdd10, fn_7000ccdd10 as *const (), 0), // +0x5e1d10 sub_5e1d10
    Lifted::new(0x7000ccddf0, fn_7000ccddf0 as *const (), 0), // +0x5e1df0 sub_5e1df0
    Lifted::new(0x7000ccde40, fn_7000ccde40 as *const (), 0), // +0x5e1e40 sub_5e1e40
    Lifted::new(0x7000ccde90, fn_7000ccde90 as *const (), 1), // +0x5e1e90 sub_5e1e90
    Lifted::new(0x7000ccded0, tpl_1185::<0x8, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000ccdf68> as *const (), 0), // +0x5e1ed0 sub_5e1ed0
    Lifted::new(0x7000ccdf90, fn_7000ccdf90 as *const (), 0), // +0x5e1f90 sub_5e1f90
    Lifted::new(0x7000cce040, tpl_1224::<0x18, 0x28, 0xfffffffffffffff8, 0x700177c312, 0x20, 0x7001d54968, 0x7000cce09d> as *const (), 0), // +0x5e2040 sub_5e2040
    Lifted::new(0x7000cce0c0, tpl_35::<0x70018271a0, 0x700177c312> as *const (), 0), // +0x5e20c0 sub_5e20c0
    Lifted::new(0x7000cce0f0, tpl_35::<0x70018271c8, 0x700177c312> as *const (), 0), // +0x5e20f0 sub_5e20f0
    Lifted::new(0x7000cce120, fn_7000cce120 as *const (), 0), // +0x5e2120 sub_5e2120
    Lifted::new(0x7000cce1f0, tpl_133::<0x7000ccdf90, 0x700177c312> as *const (), 0), // +0x5e21f0 sub_5e21f0
    Lifted::new(0x7000cce230, tpl_133::<0x7000758f80, 0x700177c312> as *const (), 0), // +0x5e2230 sub_5e2230
    Lifted::new(0x7000cce270, tpl_1149::<0x18, 0x28, 0xfffffffffffffff8, 0x700177c312, 0x20, 0x7001d54968, 0x7000cce2e9> as *const (), 0), // +0x5e2270 sub_5e2270
    Lifted::new(0x7000cce310, tpl_133::<0x7000cd0770, 0x700177c312> as *const (), 0), // +0x5e2310 sub_5e2310
    Lifted::new(0x7000cce350, tpl_1149::<0x18, 0x28, 0xfffffffffffffff8, 0x700177c312, 0x20, 0x7001d54968, 0x7000cce3c9> as *const (), 0), // +0x5e2350 sub_5e2350
    Lifted::new(0x7000cce3f0, tpl_35::<0x700179fe60, 0x700177c312> as *const (), 0), // +0x5e23f0 sub_5e23f0
    Lifted::new(0x7000cce420, tpl_1225::<0x8> as *const (), 1), // +0x5e2420 sub_5e2420
    Lifted::new(0x7000cce440, tpl_1225::<0x8> as *const (), 1), // +0x5e2440 sub_5e2440
    Lifted::new(0x7000cce460, fn_7000cce460 as *const (), 0), // +0x5e2460 sub_5e2460
    Lifted::new(0x7000cce5b0, fn_7000cce5b0 as *const (), 2), // +0x5e25b0 sub_5e25b0
    Lifted::new(0x7000cce690, fn_7000cce690 as *const (), 0), // +0x5e2690 sub_5e2690
    Lifted::new(0x7000cce6f0, fn_7000cce6f0 as *const (), 2), // +0x5e26f0 sub_5e26f0
    Lifted::new(0x7000cce8b0, tpl_1004::<0x8> as *const (), 1), // +0x5e28b0 sub_5e28b0
    Lifted::new(0x7000cce8c0, tpl_15 as *const (), 1), // +0x5e28c0 sub_5e28c0
    Lifted::new(0x7000cce8d0, fn_7000cce8d0 as *const (), 0), // +0x5e28d0 sub_5e28d0
    Lifted::new(0x7000cce8f0, tpl_15 as *const (), 1), // +0x5e28f0 sub_5e28f0
    Lifted::new(0x7000cce900, fn_7000cce900 as *const (), 0), // +0x5e2900 sub_5e2900
    Lifted::new(0x7000cce920, fn_7000cce920 as *const (), 2), // +0x5e2920 sub_5e2920
    Lifted::new(0x7000cceb20, tpl_44 as *const (), 1), // +0x5e2b20 sub_5e2b20
    Lifted::new(0x7000cceb30, tpl_144 as *const (), 1), // +0x5e2b30 sub_5e2b30
    Lifted::new(0x7000cceb40, fn_7000cceb40 as *const (), 0), // +0x5e2b40 sub_5e2b40
    Lifted::new(0x7000ccebd0, fn_7000ccebd0 as *const (), 0), // +0x5e2bd0 sub_5e2bd0
    Lifted::new(0x7000cced50, fn_7000cced50 as *const (), 0), // +0x5e2d50 sub_5e2d50
    Lifted::new(0x7000cceda0, tpl_15 as *const (), 1), // +0x5e2da0 sub_5e2da0
    Lifted::new(0x7000ccedb0, fn_7000ccedb0 as *const (), 2), // +0x5e2db0 sub_5e2db0
    Lifted::new(0x7000ccef40, fn_7000ccef40 as *const (), 2), // +0x5e2f40 sub_5e2f40
    Lifted::new(0x7000ccf4c0, tpl_530::<0x10> as *const (), 0), // +0x5e34c0 sub_5e34c0
    Lifted::new(0x7000ccf4e0, tpl_530::<0x10> as *const (), 0), // +0x5e34e0 sub_5e34e0
    Lifted::new(0x7000ccf500, tpl_1226::<0x10> as *const (), 0), // +0x5e3500 sub_5e3500
    Lifted::new(0x7000ccf510, tpl_1226::<0x10> as *const (), 0), // +0x5e3510 sub_5e3510
    Lifted::new(0x7000ccf520, tpl_46::<0x7000ccde90, 0x70007f08c0, 0x7000ccf536> as *const (), 0), // +0x5e3520 sub_5e3520
    Lifted::new(0x7000ccf540, fn_7000ccf540 as *const (), 1), // +0x5e3540 sub_5e3540
    Lifted::new(0x7000ccf580, tpl_452::<0x20, 0x18> as *const (), 1), // +0x5e3580 sub_5e3580
    Lifted::new(0x7000ccf590, tpl_315::<0x7001827060> as *const (), 1), // +0x5e3590 sub_5e3590
    Lifted::new(0x7000ccf5a0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x5e35a0 sub_5e35a0
    Lifted::new(0x7000ccf5e0, tpl_1080::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000ccf7e1, 0x7000cd2ae0, 0x7000ccf7e7, 0x70007f0910, 0x7000ccf7ed> as *const (), 0), // +0x5e35e0 sub_5e35e0
    Lifted::new(0x7000ccf880, tpl_1080::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000ccfa81, 0x7000cd2b00, 0x7000ccfa87, 0x70007f0910, 0x7000ccfa8d> as *const (), 0), // +0x5e3880 sub_5e3880
    Lifted::new(0x7000ccfb20, tpl_391::<0x7000cd2b20, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000ccfc4c, 0x7000cd2ac0, 0x7000ccfc74> as *const (), 0), // +0x5e3b20 sub_5e3b20
    Lifted::new(0x7000ccfcd0, tpl_392::<0x700177c0e0> as *const (), 0), // +0x5e3cd0 sub_5e3cd0
    Lifted::new(0x7000ccfd00, tpl_395::<0x7000cd2b20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000ccfe45, 0x7000cd2ac0, 0x7000ccfe4b> as *const (), 0), // +0x5e3d00 sub_5e3d00
    Lifted::new(0x7000ccfea0, tpl_395::<0x7000cd2b20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000ccffe5, 0x7000cd2ac0, 0x7000ccffeb> as *const (), 0), // +0x5e3ea0 sub_5e3ea0
    Lifted::new(0x7000cd0040, fn_7000cd0040 as *const (), 0), // +0x5e4040 sub_5e4040
    Lifted::new(0x7000cd0180, fn_7000cd0180 as *const (), 0), // +0x5e4180 sub_5e4180
    Lifted::new(0x7000cd02d0, fn_7000cd02d0 as *const (), 0), // +0x5e42d0 sub_5e42d0
    Lifted::new(0x7000cd0370, fn_7000cd0370 as *const (), 0), // +0x5e4370 sub_5e4370
    Lifted::new(0x7000cd04f0, fn_7000cd04f0 as *const (), 0), // +0x5e44f0 sub_5e44f0
    Lifted::new(0x7000cd05d0, fn_7000cd05d0 as *const (), 0), // +0x5e45d0 sub_5e45d0
    Lifted::new(0x7000cd06e0, fn_7000cd06e0 as *const (), 0), // +0x5e46e0 sub_5e46e0
    Lifted::new(0x7000cd0770, fn_7000cd0770 as *const (), 0), // +0x5e4770 sub_5e4770
    Lifted::new(0x7000cd08d0, tpl_116::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000cd096a> as *const (), 0), // +0x5e48d0 sub_5e48d0
    Lifted::new(0x7000cd09a0, fn_7000cd09a0 as *const (), 0), // +0x5e49a0 sub_5e49a0
    Lifted::new(0x7000cd0b50, fn_7000cd0b50 as *const (), 0), // +0x5e4b50 sub_5e4b50
    Lifted::new(0x7000cd0ba0, tpl_1227::<0x7000cd1950, 0x60, 0x50, 0xfffffffffffffff8, 0x40, 0x28, 0x1, 0x70017d2c88, 0x70017a0290, 0x7001d54ef8, 0x7000cd0c01> as *const (), 0), // +0x5e4ba0 sub_5e4ba0
    Lifted::new(0x7000cd0c40, fn_7000cd0c40 as *const (), 0), // +0x5e4c40 sub_5e4c40
    Lifted::new(0x7000cd0e20, tpl_1227::<0x7000cd1950, 0x60, 0x50, 0xfffffffffffffff8, 0x40, 0x28, 0x1, 0x70017d2c88, 0x70017a0290, 0x7001d54ef8, 0x7000cd0e81> as *const (), 0), // +0x5e4e20 sub_5e4e20
    Lifted::new(0x7000cd0ec0, fn_7000cd0ec0 as *const (), 0), // +0x5e4ec0 sub_5e4ec0
    Lifted::new(0x7000cd1010, fn_7000cd1010 as *const (), 0), // +0x5e5010 sub_5e5010
    Lifted::new(0x7000cd10b0, fn_7000cd10b0 as *const (), 1), // +0x5e50b0 sub_5e50b0
    Lifted::new(0x7000cd10f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000cd1101> as *const (), 0), // +0x5e50f0 sub_5e50f0
    Lifted::new(0x7000cd1110, fn_7000cd1110 as *const (), 2), // +0x5e5110 sub_5e5110
    Lifted::new(0x7000cd1810, fn_7000cd1810 as *const (), 0), // +0x5e5810 sub_5e5810
    Lifted::new(0x7000cd1950, fn_7000cd1950 as *const (), 0), // +0x5e5950 sub_5e5950
    Lifted::new(0x7000cd1a70, fn_7000cd1a70 as *const (), 0), // +0x5e5a70 sub_5e5a70
    Lifted::new(0x7000cd1b30, tpl_1228::<0x7000cd1fa0, 0x7000cd1b39> as *const (), 0), // +0x5e5b30 sub_5e5b30
    Lifted::new(0x7000cd1b40, tpl_36::<0x40> as *const (), 1), // +0x5e5b40 sub_5e5b40
    Lifted::new(0x7000cd1b50, fn_7000cd1b50 as *const (), 0), // +0x5e5b50 sub_5e5b50
    Lifted::new(0x7000cd1d00, fn_7000cd1d00 as *const (), 0), // +0x5e5d00 sub_5e5d00
    Lifted::new(0x7000cd1ee0, tpl_15 as *const (), 1), // +0x5e5ee0 sub_5e5ee0
    Lifted::new(0x7000cd1ef0, fn_7000cd1ef0 as *const (), 0), // +0x5e5ef0 sub_5e5ef0
    Lifted::new(0x7000cd1f50, fn_7000cd1f50 as *const (), 0), // +0x5e5f50 sub_5e5f50
    Lifted::new(0x7000cd1fa0, fn_7000cd1fa0 as *const (), 0), // +0x5e5fa0 sub_5e5fa0
    Lifted::new(0x7000cd2020, fn_7000cd2020 as *const (), 0), // +0x5e6020 sub_5e6020
    Lifted::new(0x7000cd20e0, fn_7000cd20e0 as *const (), 0), // +0x5e60e0 sub_5e60e0
    Lifted::new(0x7000cd2100, fn_7000cd2100 as *const (), 0), // +0x5e6100 sub_5e6100
    Lifted::new(0x7000cd21b0, fn_7000cd21b0 as *const (), 0), // +0x5e61b0 sub_5e61b0
    Lifted::new(0x7000cd2270, fn_7000cd2270 as *const (), 0), // +0x5e6270 sub_5e6270
    Lifted::new(0x7000cd2350, fn_7000cd2350 as *const (), 0), // +0x5e6350 sub_5e6350
    Lifted::new(0x7000cd2400, fn_7000cd2400 as *const (), 0), // +0x5e6400 sub_5e6400
    Lifted::new(0x7000cd24f0, fn_7000cd24f0 as *const (), 0), // +0x5e64f0 sub_5e64f0
    Lifted::new(0x7000cd26e0, tpl_36::<0x40> as *const (), 1), // +0x5e66e0 sub_5e66e0
    Lifted::new(0x7000cd26f0, fn_7000cd26f0 as *const (), 0), // +0x5e66f0 sub_5e66f0
    Lifted::new(0x7000cd2780, fn_7000cd2780 as *const (), 0), // +0x5e6780 sub_5e6780
    Lifted::new(0x7000cd27c0, fn_7000cd27c0 as *const (), 0), // +0x5e67c0 sub_5e67c0
    Lifted::new(0x7000cd2890, fn_7000cd2890 as *const (), 0), // +0x5e6890 sub_5e6890
    Lifted::new(0x7000cd28e0, tpl_15 as *const (), 1), // +0x5e68e0 sub_5e68e0
    Lifted::new(0x7000cd28f0, tpl_15 as *const (), 1), // +0x5e68f0 sub_5e68f0
    Lifted::new(0x7000cd2900, tpl_15 as *const (), 1), // +0x5e6900 sub_5e6900
    Lifted::new(0x7000cd2910, tpl_280::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7000cd2b20, 0x7001d54968, 0x7000cd29c8, 0x7000cd2ac0, 0x7000cd29ce> as *const (), 0), // +0x5e6910 sub_5e6910
    Lifted::new(0x7000cd2a00, tpl_1185::<0x8, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000cd2a98> as *const (), 0), // +0x5e6a00 sub_5e6a00
    Lifted::new(0x7000cd2ac0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cd2ad0> as *const (), 0), // +0x5e6ac0 sub_5e6ac0
    Lifted::new(0x7000cd2ae0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cd2af0> as *const (), 0), // +0x5e6ae0 sub_5e6ae0
    Lifted::new(0x7000cd2b00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000cd2b10> as *const (), 0), // +0x5e6b00 sub_5e6b00
    Lifted::new(0x7000cd2b20, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000cd2b74, 0x70007f0910, 0x7000cd2b8f> as *const (), 0), // +0x5e6b20 sub_5e6b20
    Lifted::new(0x7000cd2bb0, tpl_30 as *const (), 1), // +0x5e6bb0 sub_5e6bb0
    Lifted::new(0x7000cd2bc0, fn_7000cd2bc0 as *const (), 1), // +0x5e6bc0 sub_5e6bc0
    Lifted::new(0x7000cd2be0, tpl_465 as *const (), 1), // +0x5e6be0 sub_5e6be0
    Lifted::new(0x7000cd2bf0, fn_7000cd2bf0 as *const (), 0), // +0x5e6bf0 sub_5e6bf0
    Lifted::new(0x7000cd2c60, fn_7000cd2c60 as *const (), 0), // +0x5e6c60 sub_5e6c60
    Lifted::new(0x7000cd2ce0, fn_7000cd2ce0 as *const (), 0), // +0x5e6ce0 sub_5e6ce0
    Lifted::new(0x7000cd2d10, fn_7000cd2d10 as *const (), 0), // +0x5e6d10 sub_5e6d10
    Lifted::new(0x7000cd2dd0, fn_7000cd2dd0 as *const (), 0), // +0x5e6dd0 sub_5e6dd0
    Lifted::new(0x7000cd2e10, tpl_1229::<0x28, 0x7001d54e98, 0x30, 0x7001d550d8> as *const (), 0), // +0x5e6e10 sub_5e6e10
    Lifted::new(0x7000cd2e60, fn_7000cd2e60 as *const (), 0), // +0x5e6e60 sub_5e6e60
    Lifted::new(0x7000cd2ee0, fn_7000cd2ee0 as *const (), 0), // +0x5e6ee0 sub_5e6ee0
    Lifted::new(0x7000cd2f20, fn_7000cd2f20 as *const (), 2), // +0x5e6f20 sub_5e6f20
    Lifted::new(0x7000cd3310, fn_7000cd3310 as *const (), 0), // +0x5e7310 sub_5e7310
    Lifted::new(0x7000cd3360, fn_7000cd3360 as *const (), 0), // +0x5e7360 sub_5e7360
    Lifted::new(0x7000cd3420, tpl_1230::<0x7001d54e98, 0x18, 0x7001827500, 0x70017a0290, 0x7001d54ef8, 0x7000cd3449, 0x1c, 0x10, 0x7001d54bf0, 0x7001d550d8> as *const (), 0), // +0x5e7420 sub_5e7420
    Lifted::new(0x7000cd3480, tpl_1230::<0x7001d54e98, 0x18, 0x7001827500, 0x70017a0290, 0x7001d54ef8, 0x7000cd34a9, 0x1c, 0x10, 0x7001d54bf0, 0x7001d550d8> as *const (), 0), // +0x5e7480 sub_5e7480
    Lifted::new(0x7000cd34e0, tpl_1231::<0x7001d54e98, 0x18, 0x7001827500, 0x70017a0290, 0x7001d54ef8, 0x7000cd3509, 0x1c, 0x10, 0x7001d54bf0, 0x8, 0x7001d54be8, 0x7001d550d8> as *const (), 0), // +0x5e74e0 sub_5e74e0
    Lifted::new(0x7000cd3570, tpl_1231::<0x7001d54e98, 0x18, 0x7001827500, 0x70017a0290, 0x7001d54ef8, 0x7000cd3599, 0x1c, 0x10, 0x7001d54bf0, 0x8, 0x7001d54be8, 0x7001d550d8> as *const (), 0), // +0x5e7570 sub_5e7570
    Lifted::new(0x7000cd3600, tpl_1232::<0x7001d54e98, 0x18, 0x8, 0x7001d54be8, 0x7001d550d8> as *const (), 0), // +0x5e7600 sub_5e7600
    Lifted::new(0x7000cd3660, tpl_1232::<0x7001d54e98, 0x18, 0x8, 0x7001d54be8, 0x7001d550d8> as *const (), 0), // +0x5e7660 sub_5e7660
    Lifted::new(0x7000cd36c0, fn_7000cd36c0 as *const (), 0), // +0x5e76c0 sub_5e76c0
    Lifted::new(0x7000cd3720, tpl_1233::<0x7001779e20, 0x7000763140, 0x7001827520, 0x28, 0x7001827538, 0x20, 0x30> as *const (), 0), // +0x5e7720 sub_5e7720
    Lifted::new(0x7000cd37d0, tpl_899::<0x7000cd37e0> as *const (), 0), // +0x5e77d0 sub_5e77d0
    Lifted::new(0x7000cd37e0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x5e77e0 sub_5e77e0
    Lifted::new(0x7000cd3820, fn_7000cd3820 as *const (), 0), // +0x5e7820 sub_5e7820
    Lifted::new(0x7000cd3940, fn_7000cd3940 as *const (), 0), // +0x5e7940 sub_5e7940
    Lifted::new(0x7000cd3ab0, fn_7000cd3ab0 as *const (), 0), // +0x5e7ab0 sub_5e7ab0
    Lifted::new(0x7000cd3b80, fn_7000cd3b80 as *const (), 0), // +0x5e7b80 sub_5e7b80
    Lifted::new(0x7000cd3bf0, fn_7000cd3bf0 as *const (), 2), // +0x5e7bf0 sub_5e7bf0
    Lifted::new(0x7000cd4ad0, tpl_14 as *const (), 1), // +0x5e8ad0 sub_5e8ad0
    Lifted::new(0x7000cd4ae0, tpl_14 as *const (), 1), // +0x5e8ae0 sub_5e8ae0
    Lifted::new(0x7000cd4af0, tpl_14 as *const (), 1), // +0x5e8af0 sub_5e8af0
    Lifted::new(0x7000cd4b00, fn_7000cd4b00 as *const (), 0), // +0x5e8b00 sub_5e8b00
    Lifted::new(0x7000cd4c10, fn_7000cd4c10 as *const (), 0), // +0x5e8c10 sub_5e8c10
    Lifted::new(0x7000cd4e80, fn_7000cd4e80 as *const (), 0), // +0x5e8e80 sub_5e8e80
    Lifted::new(0x7000cd5110, tpl_1234::<0x7000cd95c0, 0x7000cdfd70, 0x7000cd7d00, 0x7000cd7270, 0x7000ce1d90, 0x70018279e8, 0x7000c4ceb0, 0x7000cd52a8> as *const (), 0), // +0x5e9110 sub_5e9110
    Lifted::new(0x7000cd5310, tpl_1234::<0x7000cd95c0, 0x7000cdfd70, 0x7000cd8380, 0x7000cd77a0, 0x7000ce1d90, 0x70018279e8, 0x7000c4ceb0, 0x7000cd54a8> as *const (), 0), // +0x5e9310 sub_5e9310
    Lifted::new(0x7000cd5510, fn_7000cd5510 as *const (), 2), // +0x5e9510 sub_5e9510
    Lifted::new(0x7000cd5620, fn_7000cd5620 as *const (), 0), // +0x5e9620 sub_5e9620
    Lifted::new(0x7000cd59b0, fn_7000cd59b0 as *const (), 0), // +0x5e99b0 sub_5e99b0
    Lifted::new(0x7000cd5cc0, fn_7000cd5cc0 as *const (), 0), // +0x5e9cc0 sub_5e9cc0
    Lifted::new(0x7000cd60c0, tpl_1235::<0x18, 0x7000cd7180, 0xc> as *const (), 0), // +0x5ea0c0 sub_5ea0c0
    Lifted::new(0x7000cd6110, tpl_1235::<0x18, 0x7000cd7180, 0x2c> as *const (), 0), // +0x5ea110 sub_5ea110
    Lifted::new(0x7000cd6160, fn_7000cd6160 as *const (), 0), // +0x5ea160 sub_5ea160
    Lifted::new(0x7000cd6220, fn_7000cd6220 as *const (), 0), // +0x5ea220 sub_5ea220
    Lifted::new(0x7000cd6280, fn_7000cd6280 as *const (), 0), // +0x5ea280 sub_5ea280
    Lifted::new(0x7000cd6300, tpl_1236::<0x8, 0x7000cd7180, 0x13, 0x4, 0x10> as *const (), 0), // +0x5ea300 sub_5ea300
    Lifted::new(0x7000cd6350, tpl_1236::<0x8, 0x7000cd7180, 0x13, 0x4, 0x10> as *const (), 0), // +0x5ea350 sub_5ea350
    Lifted::new(0x7000cd63a0, fn_7000cd63a0 as *const (), 0), // +0x5ea3a0 sub_5ea3a0
    Lifted::new(0x7000cd6420, fn_7000cd6420 as *const (), 0), // +0x5ea420 sub_5ea420
    Lifted::new(0x7000cd64a0, fn_7000cd64a0 as *const (), 0), // +0x5ea4a0 sub_5ea4a0
    Lifted::new(0x7000cd6570, fn_7000cd6570 as *const (), 0), // +0x5ea570 sub_5ea570
    Lifted::new(0x7000cd6600, fn_7000cd6600 as *const (), 0), // +0x5ea600 sub_5ea600
    Lifted::new(0x7000cd6690, fn_7000cd6690 as *const (), 0), // +0x5ea690 sub_5ea690
    Lifted::new(0x7000cd66e0, fn_7000cd66e0 as *const (), 0), // +0x5ea6e0 sub_5ea6e0
    Lifted::new(0x7000cd6740, fn_7000cd6740 as *const (), 0), // +0x5ea740 sub_5ea740
    Lifted::new(0x7000cd67c0, fn_7000cd67c0 as *const (), 0), // +0x5ea7c0 sub_5ea7c0
    Lifted::new(0x7000cd6810, fn_7000cd6810 as *const (), 0), // +0x5ea810 sub_5ea810
    Lifted::new(0x7000cd6870, fn_7000cd6870 as *const (), 0), // +0x5ea870 sub_5ea870
    Lifted::new(0x7000cd6940, fn_7000cd6940 as *const (), 0), // +0x5ea940 sub_5ea940
    Lifted::new(0x7000cd6a20, fn_7000cd6a20 as *const (), 0), // +0x5eaa20 sub_5eaa20
    Lifted::new(0x7000cd6a90, tpl_1237::<0x7, 0x9, 0x7000e87780, 0x4, 0x7000cd6fa0, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x5eaa90 sub_5eaa90
    Lifted::new(0x7000cd6bb0, tpl_1237::<0x7, 0x9, 0x7000e87780, 0x4, 0x7000cd7180, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x5eabb0 sub_5eabb0
    Lifted::new(0x7000cd6cd0, fn_7000cd6cd0 as *const (), 0), // +0x5eacd0 sub_5eacd0
    Lifted::new(0x7000cd6eb0, fn_7000cd6eb0 as *const (), 0), // +0x5eaeb0 sub_5eaeb0
    Lifted::new(0x7000cd6fa0, fn_7000cd6fa0 as *const (), 0), // +0x5eafa0 sub_5eafa0
    Lifted::new(0x7000cd7180, fn_7000cd7180 as *const (), 0), // +0x5eb180 sub_5eb180
    Lifted::new(0x7000cd7270, fn_7000cd7270 as *const (), 0), // +0x5eb270 sub_5eb270
    Lifted::new(0x7000cd77a0, fn_7000cd77a0 as *const (), 0), // +0x5eb7a0 sub_5eb7a0
    Lifted::new(0x7000cd7d00, fn_7000cd7d00 as *const (), 2), // +0x5ebd00 sub_5ebd00
    Lifted::new(0x7000cd8380, fn_7000cd8380 as *const (), 2), // +0x5ec380 sub_5ec380
    Lifted::new(0x7000cd8a20, fn_7000cd8a20 as *const (), 0), // +0x5eca20 sub_5eca20
    Lifted::new(0x7000cd8d10, fn_7000cd8d10 as *const (), 0), // +0x5ecd10 sub_5ecd10
    Lifted::new(0x7000cd8de0, fn_7000cd8de0 as *const (), 0), // +0x5ecde0 sub_5ecde0
    Lifted::new(0x7000cd9070, fn_7000cd9070 as *const (), 2), // +0x5ed070 sub_5ed070
    Lifted::new(0x7000cd92c0, fn_7000cd92c0 as *const (), 2), // +0x5ed2c0 sub_5ed2c0
    Lifted::new(0x7000cd95c0, fn_7000cd95c0 as *const (), 2), // +0x5ed5c0 sub_5ed5c0
    Lifted::new(0x7000cd98c0, tpl_68 as *const (), 1), // +0x5ed8c0 sub_5ed8c0
    Lifted::new(0x7000cd9970, fn_7000cd9970 as *const (), 0), // +0x5ed970 sub_5ed970
    Lifted::new(0x7000cd9ab0, fn_7000cd9ab0 as *const (), 0), // +0x5edab0 sub_5edab0
    Lifted::new(0x7000cd9bf0, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x7000ce1780> as *const (), 0), // +0x5edbf0 sub_5edbf0
    Lifted::new(0x7000cd9c80, fn_7000cd9c80 as *const (), 0), // +0x5edc80 sub_5edc80
    Lifted::new(0x7000cd9df0, fn_7000cd9df0 as *const (), 2), // +0x5eddf0 sub_5eddf0
    Lifted::new(0x7000cda1c0, fn_7000cda1c0 as *const (), 0), // +0x5ee1c0 sub_5ee1c0
    Lifted::new(0x7000cda200, fn_7000cda200 as *const (), 1), // +0x5ee200 sub_5ee200
    Lifted::new(0x7000cda320, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000cda380, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x5ee320 sub_5ee320
    Lifted::new(0x7000cda400, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000cda460, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x5ee400 sub_5ee400
    Lifted::new(0x7000cda4e0, tpl_1184::<0x298, 0x8, 0x288, 0x70017d3738, 0x70017a0290, 0x7001d54ef8, 0x7000cda540, 0x700177c312, 0x50, 0x280, 0x7001d54ea0> as *const (), 0), // +0x5ee4e0 sub_5ee4e0
    Lifted::new(0x7000cda5c0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000cda663> as *const (), 0), // +0x5ee5c0 sub_5ee5c0
    Lifted::new(0x7000cda690, fn_7000cda690 as *const (), 0), // +0x5ee690 sub_5ee690
    Lifted::new(0x7000cda6e0, fn_7000cda6e0 as *const (), 0), // +0x5ee6e0 sub_5ee6e0
    Lifted::new(0x7000cda7d0, tpl_33::<0x700177c312> as *const (), 0), // +0x5ee7d0 sub_5ee7d0
    Lifted::new(0x7000cda800, tpl_33::<0x700177c312> as *const (), 0), // +0x5ee800 sub_5ee800
    Lifted::new(0x7000cda830, tpl_33::<0x700177c312> as *const (), 0), // +0x5ee830 sub_5ee830
    Lifted::new(0x7000cda860, fn_7000cda860 as *const (), 0), // +0x5ee860 sub_5ee860
    Lifted::new(0x7000cda910, tpl_33::<0x700177c312> as *const (), 0), // +0x5ee910 sub_5ee910
    Lifted::new(0x7000cda940, tpl_33::<0x700177c312> as *const (), 0), // +0x5ee940 sub_5ee940
    Lifted::new(0x7000cda970, tpl_33::<0x700177c312> as *const (), 0), // +0x5ee970 sub_5ee970
    Lifted::new(0x7000cda9a0, fn_7000cda9a0 as *const (), 0), // +0x5ee9a0 sub_5ee9a0
    Lifted::new(0x7000cdab00, fn_7000cdab00 as *const (), 0), // +0x5eeb00 sub_5eeb00
    Lifted::new(0x7000cdaf20, fn_7000cdaf20 as *const (), 0), // +0x5eef20 sub_5eef20
    Lifted::new(0x7000cdaff0, fn_7000cdaff0 as *const (), 2), // +0x5eeff0 sub_5eeff0
    Lifted::new(0x7000cdd280, fn_7000cdd280 as *const (), 0), // +0x5f1280 sub_5f1280
    Lifted::new(0x7000cdd380, fn_7000cdd380 as *const (), 2), // +0x5f1380 sub_5f1380
    Lifted::new(0x7000cdd800, fn_7000cdd800 as *const (), 0), // +0x5f1800 sub_5f1800
    Lifted::new(0x7000cdd9a0, fn_7000cdd9a0 as *const (), 0), // +0x5f19a0 sub_5f19a0
    Lifted::new(0x7000cddb30, fn_7000cddb30 as *const (), 0), // +0x5f1b30 sub_5f1b30
    Lifted::new(0x7000cddc10, fn_7000cddc10 as *const (), 1), // +0x5f1c10 sub_5f1c10
    Lifted::new(0x7000cddc80, fn_7000cddc80 as *const (), 0), // +0x5f1c80 sub_5f1c80
    Lifted::new(0x7000cddd10, fn_7000cddd10 as *const (), 1), // +0x5f1d10 sub_5f1d10
    Lifted::new(0x7000cddd50, fn_7000cddd50 as *const (), 0), // +0x5f1d50 sub_5f1d50
    Lifted::new(0x7000cdde50, fn_7000cdde50 as *const (), 2), // +0x5f1e50 sub_5f1e50
    Lifted::new(0x7000cde340, fn_7000cde340 as *const (), 0), // +0x5f2340 sub_5f2340
    Lifted::new(0x7000cde3c0, fn_7000cde3c0 as *const (), 0), // +0x5f23c0 sub_5f23c0
    Lifted::new(0x7000cde550, fn_7000cde550 as *const (), 0), // +0x5f2550 sub_5f2550
    Lifted::new(0x7000cde690, fn_7000cde690 as *const (), 2), // +0x5f2690 sub_5f2690
    Lifted::new(0x7000cde870, fn_7000cde870 as *const (), 0), // +0x5f2870 sub_5f2870
    Lifted::new(0x7000cdf930, fn_7000cdf930 as *const (), 2), // +0x5f3930 sub_5f3930
    Lifted::new(0x7000cdfaf0, fn_7000cdfaf0 as *const (), 0), // +0x5f3af0 sub_5f3af0
    Lifted::new(0x7000cdfbf0, fn_7000cdfbf0 as *const (), 2), // +0x5f3bf0 sub_5f3bf0
    Lifted::new(0x7000cdfd70, fn_7000cdfd70 as *const (), 2), // +0x5f3d70 sub_5f3d70
    Lifted::new(0x7000cdfee0, fn_7000cdfee0 as *const (), 2), // +0x5f3ee0 sub_5f3ee0
    Lifted::new(0x7000cdff20, fn_7000cdff20 as *const (), 0), // +0x5f3f20 sub_5f3f20
    Lifted::new(0x7000cdff70, fn_7000cdff70 as *const (), 2), // +0x5f3f70 sub_5f3f70
    Lifted::new(0x7000ce0090, fn_7000ce0090 as *const (), 1), // +0x5f4090 sub_5f4090
    Lifted::new(0x7000ce0100, fn_7000ce0100 as *const (), 2), // +0x5f4100 sub_5f4100
    Lifted::new(0x7000ce0510, fn_7000ce0510 as *const (), 2), // +0x5f4510 sub_5f4510
    Lifted::new(0x7000ce09f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ce0a01> as *const (), 0), // +0x5f49f0 sub_5f49f0
    Lifted::new(0x7000ce0a10, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000ce0a28> as *const (), 0), // +0x5f4a10 sub_5f4a10
    Lifted::new(0x7000ce0a30, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000ce0a48> as *const (), 0), // +0x5f4a30 sub_5f4a30
    Lifted::new(0x7000ce0a50, fn_7000ce0a50 as *const (), 0), // +0x5f4a50 sub_5f4a50
    Lifted::new(0x7000ce0c10, fn_7000ce0c10 as *const (), 0), // +0x5f4c10 sub_5f4c10
    Lifted::new(0x7000ce0ce0, fn_7000ce0ce0 as *const (), 0), // +0x5f4ce0 sub_5f4ce0
    Lifted::new(0x7000ce0e70, fn_7000ce0e70 as *const (), 0), // +0x5f4e70 sub_5f4e70
    Lifted::new(0x7000ce0f20, fn_7000ce0f20 as *const (), 2), // +0x5f4f20 sub_5f4f20
    Lifted::new(0x7000ce10d0, tpl_15 as *const (), 1), // +0x5f50d0 sub_5f50d0
    Lifted::new(0x7000ce10e0, fn_7000ce10e0 as *const (), 0), // +0x5f50e0 sub_5f50e0
    Lifted::new(0x7000ce12b0, fn_7000ce12b0 as *const (), 0), // +0x5f52b0 sub_5f52b0
    Lifted::new(0x7000ce1460, fn_7000ce1460 as *const (), 0), // +0x5f5460 sub_5f5460
    Lifted::new(0x7000ce1540, fn_7000ce1540 as *const (), 0), // +0x5f5540 sub_5f5540
    Lifted::new(0x7000ce1610, fn_7000ce1610 as *const (), 0), // +0x5f5610 sub_5f5610
    Lifted::new(0x7000ce1680, fn_7000ce1680 as *const (), 0), // +0x5f5680 sub_5f5680
    Lifted::new(0x7000ce1740, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ce1751> as *const (), 0), // +0x5f5740 sub_5f5740
    Lifted::new(0x7000ce1760, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ce1771> as *const (), 0), // +0x5f5760 sub_5f5760
    Lifted::new(0x7000ce1780, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000ce1887, 0x7000cd98c0, 0x70007f0910, 0x7000ce18a3> as *const (), 0), // +0x5f5780 sub_5f5780
    Lifted::new(0x7000ce18f0, tpl_923::<0x70018279a8> as *const (), 1), // +0x5f58f0 sub_5f58f0
    Lifted::new(0x7000ce1910, tpl_754::<0x700177c312> as *const (), 0), // +0x5f5910 sub_5f5910
    Lifted::new(0x7000ce1930, tpl_44 as *const (), 1), // +0x5f5930 sub_5f5930
    Lifted::new(0x7000ce1940, tpl_1159::<0x7000ce1780, 0x70017d37a0, 0x70017790e0, 0x7000ce1b0e> as *const (), 0), // +0x5f5940 sub_5f5940
    Lifted::new(0x7000ce1b90, tpl_52::<0x8> as *const (), 1), // +0x5f5b90 sub_5f5b90
    Lifted::new(0x7000ce1ba0, tpl_923::<0x70018279a8> as *const (), 1), // +0x5f5ba0 sub_5f5ba0
    Lifted::new(0x7000ce1bc0, tpl_897::<0x10, 0x38, 0x700177c170, 0x70017d37ec, 0x7000ce1940> as *const (), 2), // +0x5f5bc0 sub_5f5bc0
    Lifted::new(0x7000ce1ca0, tpl_315::<0x7001c6bf60> as *const (), 1), // +0x5f5ca0 sub_5f5ca0
    Lifted::new(0x7000ce1cb0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000ce1cc0> as *const (), 0), // +0x5f5cb0 sub_5f5cb0
    Lifted::new(0x7000ce1cd0, tpl_990::<0x10, 0x8, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x5f5cd0 sub_5f5cd0
    Lifted::new(0x7000ce1d90, tpl_36::<0x8> as *const (), 1), // +0x5f5d90 sub_5f5d90
    Lifted::new(0x7000ce1da0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x5f5da0 sub_5f5da0
    Lifted::new(0x7000ce1de0, tpl_67::<0x7000ce2460, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000ce1f23, 0x7000ce2440, 0x7000ce1f29> as *const (), 0), // +0x5f5de0 sub_5f5de0
    Lifted::new(0x7000ce1f80, fn_7000ce1f80 as *const (), 0), // +0x5f5f80 sub_5f5f80
    Lifted::new(0x7000ce2060, fn_7000ce2060 as *const (), 1), // +0x5f6060 sub_5f6060
    Lifted::new(0x7000ce21b0, fn_7000ce21b0 as *const (), 0), // +0x5f61b0 sub_5f61b0
    Lifted::new(0x7000ce2250, fn_7000ce2250 as *const (), 0), // +0x5f6250 sub_5f6250
    Lifted::new(0x7000ce2280, fn_7000ce2280 as *const (), 1), // +0x5f6280 sub_5f6280
    Lifted::new(0x7000ce22e0, fn_7000ce22e0 as *const (), 0), // +0x5f62e0 sub_5f62e0
    Lifted::new(0x7000ce2440, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000ce2450> as *const (), 0), // +0x5f6440 sub_5f6440
    Lifted::new(0x7000ce2460, tpl_298::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000ce24b4, 0x70007f0910, 0x7000ce24cf> as *const (), 0), // +0x5f6460 sub_5f6460
    Lifted::new(0x7000ce24f0, fn_7000ce24f0 as *const (), 2), // +0x5f64f0 sub_5f64f0
    Lifted::new(0x7000ce2840, fn_7000ce2840 as *const (), 2), // +0x5f6840 sub_5f6840
    Lifted::new(0x7000ce28a0, fn_7000ce28a0 as *const (), 3), // +0x5f68a0 sub_5f68a0
    Lifted::new(0x7000ce28f0, fn_7000ce28f0 as *const (), 3), // +0x5f68f0 sub_5f68f0
    Lifted::new(0x7000ce2930, tpl_1238::<0x700078d390> as *const (), 0), // +0x5f6930 sub_5f6930
    Lifted::new(0x7000ce2940, tpl_1239::<0xb8, 0xe8, 0xe0, 0x70007717d0, 0x7000ce3250, 0x30, 0x700078d390, 0x80> as *const (), 0), // +0x5f6940 sub_5f6940
    Lifted::new(0x7000ce29e0, fn_7000ce29e0 as *const (), 0), // +0x5f69e0 sub_5f69e0
    Lifted::new(0x7000ce2a30, fn_7000ce2a30 as *const (), 0), // +0x5f6a30 sub_5f6a30
    Lifted::new(0x7000ce2bc0, fn_7000ce2bc0 as *const (), 2), // +0x5f6bc0 sub_5f6bc0
    Lifted::new(0x7000ce2c70, fn_7000ce2c70 as *const (), 0), // +0x5f6c70 sub_5f6c70
    Lifted::new(0x7000ce2d60, fn_7000ce2d60 as *const (), 0), // +0x5f6d60 sub_5f6d60
    Lifted::new(0x7000ce2dc0, fn_7000ce2dc0 as *const (), 0), // +0x5f6dc0 sub_5f6dc0
    Lifted::new(0x7000ce2e20, fn_7000ce2e20 as *const (), 0), // +0x5f6e20 sub_5f6e20
    Lifted::new(0x7000ce2ee0, fn_7000ce2ee0 as *const (), 0), // +0x5f6ee0 sub_5f6ee0
    Lifted::new(0x7000ce2f40, fn_7000ce2f40 as *const (), 0), // +0x5f6f40 sub_5f6f40
    Lifted::new(0x7000ce3000, fn_7000ce3000 as *const (), 0), // +0x5f7000 sub_5f7000
    Lifted::new(0x7000ce30a0, fn_7000ce30a0 as *const (), 1), // +0x5f70a0 sub_5f70a0
    Lifted::new(0x7000ce3110, fn_7000ce3110 as *const (), 0), // +0x5f7110 sub_5f7110
    Lifted::new(0x7000ce3220, fn_7000ce3220 as *const (), 0), // +0x5f7220 sub_5f7220
    Lifted::new(0x7000ce3250, fn_7000ce3250 as *const (), 0), // +0x5f7250 sub_5f7250
    Lifted::new(0x7000ce32d0, fn_7000ce32d0 as *const (), 2), // +0x5f72d0 sub_5f72d0
    Lifted::new(0x7000ce33b0, fn_7000ce33b0 as *const (), 0), // +0x5f73b0 sub_5f73b0
    Lifted::new(0x7000ce3400, fn_7000ce3400 as *const (), 1), // +0x5f7400 sub_5f7400
    Lifted::new(0x7000ce3420, fn_7000ce3420 as *const (), 0), // +0x5f7420 sub_5f7420
    Lifted::new(0x7000ce3480, fn_7000ce3480 as *const (), 0), // +0x5f7480 sub_5f7480
    Lifted::new(0x7000ce34f0, fn_7000ce34f0 as *const (), 0), // +0x5f74f0 sub_5f74f0
    Lifted::new(0x7000ce35c0, tpl_1238::<0x7000752a30> as *const (), 0), // +0x5f75c0 sub_5f75c0
    Lifted::new(0x7000ce35d0, tpl_1239::<0xb8, 0xe8, 0xe0, 0x70007717d0, 0x7000ce3250, 0x30, 0x7000752a30, 0x80> as *const (), 0), // +0x5f75d0 sub_5f75d0
    Lifted::new(0x7000ce3670, fn_7000ce3670 as *const (), 1), // +0x5f7670 sub_5f7670
    Lifted::new(0x7000ce3690, fn_7000ce3690 as *const (), 0), // +0x5f7690 sub_5f7690
    Lifted::new(0x7000ce36d0, fn_7000ce36d0 as *const (), 1), // +0x5f76d0 sub_5f76d0
    Lifted::new(0x7000ce3860, fn_7000ce3860 as *const (), 0), // +0x5f7860 sub_5f7860
    Lifted::new(0x7000ce3a60, fn_7000ce3a60 as *const (), 1), // +0x5f7a60 sub_5f7a60
    Lifted::new(0x7000ce3a70, fn_7000ce3a70 as *const (), 1), // +0x5f7a70 sub_5f7a70
    Lifted::new(0x7000ce3a90, tpl_33::<0x700177c312> as *const (), 0), // +0x5f7a90 sub_5f7a90
    Lifted::new(0x7000ce3ac0, fn_7000ce3ac0 as *const (), 0), // +0x5f7ac0 sub_5f7ac0
    Lifted::new(0x7000ce3b20, fn_7000ce3b20 as *const (), 0), // +0x5f7b20 sub_5f7b20
    Lifted::new(0x7000ce3be0, tpl_33::<0x700177c312> as *const (), 0), // +0x5f7be0 sub_5f7be0
    Lifted::new(0x7000ce3c10, tpl_47::<0x1c> as *const (), 1), // +0x5f7c10 sub_5f7c10
    Lifted::new(0x7000ce3c20, tpl_1126::<0x20> as *const (), 3), // +0x5f7c20 sub_5f7c20
    Lifted::new(0x7000ce3c30, fn_7000ce3c30 as *const (), 0), // +0x5f7c30 sub_5f7c30
    Lifted::new(0x7000ce3ca0, fn_7000ce3ca0 as *const (), 2), // +0x5f7ca0 sub_5f7ca0
    Lifted::new(0x7000ce3e50, fn_7000ce3e50 as *const (), 2), // +0x5f7e50 sub_5f7e50
    Lifted::new(0x7000ce4060, tpl_64::<0x19, 0x10, 0x7000ce4060, 0x700177c312> as *const (), 0), // +0x5f8060 sub_5f8060
    Lifted::new(0x7000ce40d0, fn_7000ce40d0 as *const (), 1), // +0x5f80d0 sub_5f80d0
    Lifted::new(0x7000ce4160, fn_7000ce4160 as *const (), 0), // +0x5f8160 sub_5f8160
    Lifted::new(0x7000ce44d0, fn_7000ce44d0 as *const (), 0), // +0x5f84d0 sub_5f84d0
    Lifted::new(0x7000ce48a0, tpl_1233::<0x7001779e20, 0x7000763140, 0x7001827c40, 0x28, 0x7001827c58, 0x20, 0x30> as *const (), 0), // +0x5f88a0 sub_5f88a0
    Lifted::new(0x7000ce4950, fn_7000ce4950 as *const (), 0), // +0x5f8950 sub_5f8950
    Lifted::new(0x7000ce4a20, tpl_1214::<0x7001779e20, 0x7001827c20, 0x8, 0xa028, 0x10> as *const (), 0), // +0x5f8a20 sub_5f8a20
    Lifted::new(0x7000ce4ae0, fn_7000ce4ae0 as *const (), 0), // +0x5f8ae0 sub_5f8ae0
    Lifted::new(0x7000ce4c00, fn_7000ce4c00 as *const (), 0), // +0x5f8c00 sub_5f8c00
    Lifted::new(0x7000ce4cc0, fn_7000ce4cc0 as *const (), 0), // +0x5f8cc0 sub_5f8cc0
    Lifted::new(0x7000ce4dd0, fn_7000ce4dd0 as *const (), 0), // +0x5f8dd0 sub_5f8dd0
    Lifted::new(0x7000ce4e90, fn_7000ce4e90 as *const (), 0), // +0x5f8e90 sub_5f8e90
    Lifted::new(0x7000ce4ec0, tpl_116::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000ce4f5a> as *const (), 0), // +0x5f8ec0 sub_5f8ec0
    Lifted::new(0x7000ce4f90, fn_7000ce4f90 as *const (), 0), // +0x5f8f90 sub_5f8f90
    Lifted::new(0x7000ce5080, fn_7000ce5080 as *const (), 0), // +0x5f9080 sub_5f9080
    Lifted::new(0x7000ce5150, tpl_899::<0x7000ce5170> as *const (), 0), // +0x5f9150 sub_5f9150
    Lifted::new(0x7000ce5160, tpl_899::<0x7000ce51b0> as *const (), 0), // +0x5f9160 sub_5f9160
    Lifted::new(0x7000ce5170, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x5f9170 sub_5f9170
    Lifted::new(0x7000ce51b0, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x5f91b0 sub_5f91b0
    Lifted::new(0x7000ce51f0, tpl_33::<0x700177c312> as *const (), 0), // +0x5f91f0 sub_5f91f0
    Lifted::new(0x7000ce5220, fn_7000ce5220 as *const (), 0), // +0x5f9220 sub_5f9220
    Lifted::new(0x7000ce5310, fn_7000ce5310 as *const (), 0), // +0x5f9310 sub_5f9310
    Lifted::new(0x7000ce5370, fn_7000ce5370 as *const (), 0), // +0x5f9370 sub_5f9370
    Lifted::new(0x7000ce53c0, fn_7000ce53c0 as *const (), 0), // +0x5f93c0 sub_5f93c0
    Lifted::new(0x7000ce5420, fn_7000ce5420 as *const (), 0), // +0x5f9420 sub_5f9420
    Lifted::new(0x7000ce5490, fn_7000ce5490 as *const (), 0), // +0x5f9490 sub_5f9490
    Lifted::new(0x7000ce5590, fn_7000ce5590 as *const (), 0), // +0x5f9590 sub_5f9590
    Lifted::new(0x7000ce5600, fn_7000ce5600 as *const (), 0), // +0x5f9600 sub_5f9600
    Lifted::new(0x7000ce56e0, fn_7000ce56e0 as *const (), 0), // +0x5f96e0 sub_5f96e0
    Lifted::new(0x7000ce57d0, fn_7000ce57d0 as *const (), 0), // +0x5f97d0 sub_5f97d0
    Lifted::new(0x7000ce5910, fn_7000ce5910 as *const (), 0), // +0x5f9910 sub_5f9910
    Lifted::new(0x7000ce5ac0, fn_7000ce5ac0 as *const (), 0), // +0x5f9ac0 sub_5f9ac0
    Lifted::new(0x7000ce5b60, fn_7000ce5b60 as *const (), 0), // +0x5f9b60 sub_5f9b60
    Lifted::new(0x7000ce5c00, fn_7000ce5c00 as *const (), 0), // +0x5f9c00 sub_5f9c00
    Lifted::new(0x7000ce5c80, fn_7000ce5c80 as *const (), 0), // +0x5f9c80 sub_5f9c80
    Lifted::new(0x7000ce5ce0, fn_7000ce5ce0 as *const (), 0), // +0x5f9ce0 sub_5f9ce0
    Lifted::new(0x7000ce5d60, fn_7000ce5d60 as *const (), 0), // +0x5f9d60 sub_5f9d60
    Lifted::new(0x7000ce5e30, fn_7000ce5e30 as *const (), 0), // +0x5f9e30 sub_5f9e30
    Lifted::new(0x7000ce5fc0, fn_7000ce5fc0 as *const (), 0), // +0x5f9fc0 sub_5f9fc0
    Lifted::new(0x7000ce60e0, fn_7000ce60e0 as *const (), 0), // +0x5fa0e0 sub_5fa0e0
    Lifted::new(0x7000ce6150, fn_7000ce6150 as *const (), 2), // +0x5fa150 sub_5fa150
    Lifted::new(0x7000ce6b60, fn_7000ce6b60 as *const (), 2), // +0x5fab60 sub_5fab60
    Lifted::new(0x7000ce6f20, fn_7000ce6f20 as *const (), 0), // +0x5faf20 sub_5faf20
    Lifted::new(0x7000ce7060, fn_7000ce7060 as *const (), 0), // +0x5fb060 sub_5fb060
    Lifted::new(0x7000ce71c0, fn_7000ce71c0 as *const (), 2), // +0x5fb1c0 sub_5fb1c0
    Lifted::new(0x7000ce7500, fn_7000ce7500 as *const (), 0), // +0x5fb500 sub_5fb500
    Lifted::new(0x7000ce7650, fn_7000ce7650 as *const (), 0), // +0x5fb650 sub_5fb650
    Lifted::new(0x7000ce7760, fn_7000ce7760 as *const (), 0), // +0x5fb760 sub_5fb760
    Lifted::new(0x7000ce7880, fn_7000ce7880 as *const (), 0), // +0x5fb880 sub_5fb880
    Lifted::new(0x7000ce7930, fn_7000ce7930 as *const (), 0), // +0x5fb930 sub_5fb930
    Lifted::new(0x7000ce7980, tpl_1240::<0x10, 0x30, 0x18> as *const (), 0), // +0x5fb980 sub_5fb980
    Lifted::new(0x7000ce79c0, fn_7000ce79c0 as *const (), 0), // +0x5fb9c0 sub_5fb9c0
    Lifted::new(0x7000ce7a70, tpl_287 as *const (), 1), // +0x5fba70 sub_5fba70
    Lifted::new(0x7000ce7d90, fn_7000ce7d90 as *const (), 2), // +0x5fbd90 sub_5fbd90
    Lifted::new(0x7000ce8020, fn_7000ce8020 as *const (), 2), // +0x5fc020 sub_5fc020
    Lifted::new(0x7000ce83a0, tpl_1241::<0x7001d54e98, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5fc3a0 sub_5fc3a0
    Lifted::new(0x7000ce8670, tpl_1241::<0x7001d54e98, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5fc670 sub_5fc670
    Lifted::new(0x7000ce8940, fn_7000ce8940 as *const (), 2), // +0x5fc940 sub_5fc940
    Lifted::new(0x7000ce8c80, tpl_1241::<0x7001d54e98, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5fcc80 sub_5fcc80
    Lifted::new(0x7000ce8f50, tpl_1241::<0x7001d54e98, 0x700177c312, 0x7001d550d8> as *const (), 0), // +0x5fcf50 sub_5fcf50
    Lifted::new(0x7000ce9220, fn_7000ce9220 as *const (), 3), // +0x5fd220 sub_5fd220
    Lifted::new(0x7000ce92f0, fn_7000ce92f0 as *const (), 0), // +0x5fd2f0 sub_5fd2f0
    Lifted::new(0x7000ce94b0, fn_7000ce94b0 as *const (), 0), // +0x5fd4b0 sub_5fd4b0
    Lifted::new(0x7000ce9710, fn_7000ce9710 as *const (), 0), // +0x5fd710 sub_5fd710
    Lifted::new(0x7000ce99a0, fn_7000ce99a0 as *const (), 0), // +0x5fd9a0 sub_5fd9a0
    Lifted::new(0x7000ce9bc0, fn_7000ce9bc0 as *const (), 0), // +0x5fdbc0 sub_5fdbc0
    Lifted::new(0x7000ce9df0, fn_7000ce9df0 as *const (), 0), // +0x5fddf0 sub_5fddf0
    Lifted::new(0x7000ce9f40, tpl_1242::<0x17, 0x1f, 0xb, 0x3, 0xffffffffffffffff, 0x7, 0x7000ce99a0> as *const (), 0), // +0x5fdf40 sub_5fdf40
    Lifted::new(0x7000cea000, fn_7000cea000 as *const (), 0), // +0x5fe000 sub_5fe000
    Lifted::new(0x7000cea170, tpl_1242::<0x17, 0x1f, 0xb, 0x3, 0xffffffffffffffff, 0x7, 0x7000ce9bc0> as *const (), 0), // +0x5fe170 sub_5fe170
    Lifted::new(0x7000cea230, fn_7000cea230 as *const (), 0), // +0x5fe230 sub_5fe230
    Lifted::new(0x7000cea490, fn_7000cea490 as *const (), 0), // +0x5fe490 sub_5fe490
    Lifted::new(0x7000cea6f0, fn_7000cea6f0 as *const (), 1), // +0x5fe6f0 sub_5fe6f0
    Lifted::new(0x7000cea7f0, fn_7000cea7f0 as *const (), 0), // +0x5fe7f0 sub_5fe7f0
    Lifted::new(0x7000cea8b0, fn_7000cea8b0 as *const (), 1), // +0x5fe8b0 sub_5fe8b0
    Lifted::new(0x7000cea970, fn_7000cea970 as *const (), 2), // +0x5fe970 sub_5fe970
    Lifted::new(0x7000ceab00, tpl_1243::<0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000ceac94, 0x7000cec2e0> as *const (), 2), // +0x5feb00 sub_5feb00
    Lifted::new(0x7000ceaef0, tpl_1243::<0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000ceb084, 0x7000cec2e0> as *const (), 2), // +0x5feef0 sub_5feef0
    Lifted::new(0x7000ceb2e0, fn_7000ceb2e0 as *const (), 1), // +0x5ff2e0 sub_5ff2e0
];
