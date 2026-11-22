#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x7000513090, fn_7000513090 as *const (), 0), // +0x513090 sub_513090
    Lifted::new(0x7000513170, fn_7000513170 as *const (), 0), // +0x513170 sub_513170
    Lifted::new(0x7000513320, fn_7000513320 as *const (), 0), // +0x513320 sub_513320
    Lifted::new(0x7000513630, fn_7000513630 as *const (), 0), // +0x513630 sub_513630
    Lifted::new(0x7000513980, fn_7000513980 as *const (), 0), // +0x513980 sub_513980
    Lifted::new(0x7000513a10, fn_7000513a10 as *const (), 0), // +0x513a10 sub_513a10
    Lifted::new(0x7000513d80, fn_7000513d80 as *const (), 0), // +0x513d80 sub_513d80
    Lifted::new(0x7000513fc0, fn_7000513fc0 as *const (), 0), // +0x513fc0 sub_513fc0
    Lifted::new(0x7000514050, tpl_445::<0x70004b2bd0, 0x58, 0x8, 0xb0, 0x70005a8e60, 0x10, 0x18> as *const (), 0), // +0x514050 sub_514050
    Lifted::new(0x70005140b0, fn_70005140b0 as *const (), 0), // +0x5140b0 sub_5140b0
    Lifted::new(0x70005143a0, fn_70005143a0 as *const (), 2), // +0x5143a0 sub_5143a0
    Lifted::new(0x70005148a0, fn_70005148a0 as *const (), 0), // +0x5148a0 sub_5148a0
    Lifted::new(0x7000514bb0, fn_7000514bb0 as *const (), 0), // +0x514bb0 sub_514bb0
    Lifted::new(0x7000514c40, fn_7000514c40 as *const (), 0), // +0x514c40 sub_514c40
    Lifted::new(0x7000514d50, fn_7000514d50 as *const (), 0), // +0x514d50 sub_514d50
    Lifted::new(0x7000514e80, fn_7000514e80 as *const (), 2), // +0x514e80 sub_514e80
    Lifted::new(0x7000514fb0, fn_7000514fb0 as *const (), 0), // +0x514fb0 sub_514fb0
    Lifted::new(0x70005150e0, fn_70005150e0 as *const (), 2), // +0x5150e0 sub_5150e0
    Lifted::new(0x7000515210, tpl_446::<0x70004b2bd0, 0x8, 0x3220, 0x18, 0x28, 0x700010b4f0, 0x10> as *const (), 0), // +0x515210 sub_515210
    Lifted::new(0x70005152a0, tpl_447::<0x70004b2bd0, 0x2bd0, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x5152a0 sub_5152a0
    Lifted::new(0x7000515310, tpl_447::<0x70004b2bd0, 0x2bc8, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x515310 sub_515310
    Lifted::new(0x7000515380, tpl_447::<0x70004b2bd0, 0x3218, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x515380 sub_515380
    Lifted::new(0x70005153f0, tpl_447::<0x70004b2bd0, 0x2bd8, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x5153f0 sub_5153f0
    Lifted::new(0x7000515460, fn_7000515460 as *const (), 2), // +0x515460 sub_515460
    Lifted::new(0x7000515640, tpl_448::<0x70004b2bd0, 0x8, 0x70003a16f0, 0x28, 0x700010b4f0, 0x10, 0x18> as *const (), 0), // +0x515640 sub_515640
    Lifted::new(0x70005156d0, tpl_446::<0x70004b2bd0, 0x8, 0x2ff8, 0x18, 0x28, 0x700010b4f0, 0x10> as *const (), 0), // +0x5156d0 sub_5156d0
    Lifted::new(0x7000515760, fn_7000515760 as *const (), 0), // +0x515760 sub_515760
    Lifted::new(0x7000515840, tpl_417::<0x7000623fd0, 0x700057eb50> as *const (), 0), // +0x515840 sub_515840
    Lifted::new(0x7000515850, tpl_418::<0x70004b2bd0, 0x70005fb574, 0x7000623fd0, 0x700057ec20> as *const (), 0), // +0x515850 sub_515850
    Lifted::new(0x7000515880, tpl_417::<0x7000624078, 0x700057eb50> as *const (), 0), // +0x515880 sub_515880
    Lifted::new(0x7000515890, tpl_418::<0x70004b2bd0, 0x70005fb58c, 0x7000624078, 0x700057ec20> as *const (), 0), // +0x515890 sub_515890
    Lifted::new(0x70005158c0, tpl_417::<0x7000624080, 0x700057eb50> as *const (), 0), // +0x5158c0 sub_5158c0
    Lifted::new(0x70005158d0, tpl_418::<0x70004b2bd0, 0x70005fb584, 0x7000624080, 0x700057ec20> as *const (), 0), // +0x5158d0 sub_5158d0
    Lifted::new(0x7000515900, tpl_417::<0x7000624088, 0x700057eb50> as *const (), 0), // +0x515900 sub_515900
    Lifted::new(0x7000515910, tpl_418::<0x70004b2bd0, 0x70005fb5a0, 0x7000624088, 0x700057ec20> as *const (), 0), // +0x515910 sub_515910
    Lifted::new(0x7000515940, tpl_417::<0x7000624098, 0x700057eb50> as *const (), 0), // +0x515940 sub_515940
    Lifted::new(0x7000515950, tpl_418::<0x70004b2bd0, 0x70005f8384, 0x7000624098, 0x700057ec20> as *const (), 0), // +0x515950 sub_515950
    Lifted::new(0x7000515980, tpl_417::<0x7000624048, 0x700057eb50> as *const (), 0), // +0x515980 sub_515980
    Lifted::new(0x7000515990, tpl_418::<0x70004b2bd0, 0x70005fb594, 0x7000624048, 0x700057ec20> as *const (), 0), // +0x515990 sub_515990
    Lifted::new(0x70005159c0, tpl_417::<0x7000624050, 0x700057eb50> as *const (), 0), // +0x5159c0 sub_5159c0
    Lifted::new(0x70005159d0, tpl_418::<0x70004b2bd0, 0x70005fb5d8, 0x7000624050, 0x700057ec20> as *const (), 0), // +0x5159d0 sub_5159d0
    Lifted::new(0x7000515a00, tpl_417::<0x7000624060, 0x700057eb50> as *const (), 0), // +0x515a00 sub_515a00
    Lifted::new(0x7000515a10, tpl_418::<0x70004b2bd0, 0x70005fb5c8, 0x7000624060, 0x700057ec20> as *const (), 0), // +0x515a10 sub_515a10
    Lifted::new(0x7000515a40, tpl_417::<0x7000624070, 0x700057eb50> as *const (), 0), // +0x515a40 sub_515a40
    Lifted::new(0x7000515a50, tpl_418::<0x70004b2bd0, 0x70005fb5e8, 0x7000624070, 0x700057ec20> as *const (), 0), // +0x515a50 sub_515a50
    Lifted::new(0x7000515a80, tpl_417::<0x70006240dc, 0x700057eb50> as *const (), 0), // +0x515a80 sub_515a80
    Lifted::new(0x7000515a90, tpl_418::<0x70004b2bd0, 0x70005fb5e0, 0x70006240dc, 0x700057ec20> as *const (), 0), // +0x515a90 sub_515a90
    Lifted::new(0x7000515ac0, tpl_417::<0x70006240e8, 0x700057eb50> as *const (), 0), // +0x515ac0 sub_515ac0
    Lifted::new(0x7000515ad0, tpl_418::<0x70004b2bd0, 0x70005fb600, 0x70006240e8, 0x700057ec20> as *const (), 0), // +0x515ad0 sub_515ad0
    Lifted::new(0x7000515b00, tpl_417::<0x70006240f8, 0x700057eb50> as *const (), 0), // +0x515b00 sub_515b00
    Lifted::new(0x7000515b10, tpl_418::<0x70004b2bd0, 0x70005fb5f0, 0x70006240f8, 0x700057ec20> as *const (), 0), // +0x515b10 sub_515b10
    Lifted::new(0x7000515b40, tpl_417::<0x7000624108, 0x700057eb50> as *const (), 0), // +0x515b40 sub_515b40
    Lifted::new(0x7000515b50, tpl_418::<0x70004b2bd0, 0x70005fb620, 0x7000624108, 0x700057ec20> as *const (), 0), // +0x515b50 sub_515b50
    Lifted::new(0x7000515b80, tpl_417::<0x70006240a0, 0x700057eb50> as *const (), 0), // +0x515b80 sub_515b80
    Lifted::new(0x7000515b90, tpl_418::<0x70004b2bd0, 0x70005fb610, 0x70006240a0, 0x700057ec20> as *const (), 0), // +0x515b90 sub_515b90
    Lifted::new(0x7000515bc0, tpl_417::<0x70006240b0, 0x700057eb50> as *const (), 0), // +0x515bc0 sub_515bc0
    Lifted::new(0x7000515bd0, tpl_418::<0x70004b2bd0, 0x70005fb640, 0x70006240b0, 0x700057ec20> as *const (), 0), // +0x515bd0 sub_515bd0
    Lifted::new(0x7000515c00, tpl_417::<0x70006240c0, 0x700057eb50> as *const (), 0), // +0x515c00 sub_515c00
    Lifted::new(0x7000515c10, tpl_418::<0x70004b2bd0, 0x70005fb630, 0x70006240c0, 0x700057ec20> as *const (), 0), // +0x515c10 sub_515c10
    Lifted::new(0x7000515c40, tpl_417::<0x70006240d0, 0x700057eb50> as *const (), 0), // +0x515c40 sub_515c40
    Lifted::new(0x7000515c50, tpl_418::<0x70004b2bd0, 0x70005fb658, 0x70006240d0, 0x700057ec20> as *const (), 0), // +0x515c50 sub_515c50
    Lifted::new(0x7000515c80, tpl_417::<0x7000624210, 0x700057eb50> as *const (), 0), // +0x515c80 sub_515c80
    Lifted::new(0x7000515c90, tpl_418::<0x70004b2bd0, 0x70005f8064, 0x7000624210, 0x700057ec20> as *const (), 0), // +0x515c90 sub_515c90
    Lifted::new(0x7000515cc0, tpl_417::<0x7000624218, 0x700057eb50> as *const (), 0), // +0x515cc0 sub_515cc0
    Lifted::new(0x7000515cd0, tpl_418::<0x70004b2bd0, 0x70005f83b0, 0x7000624218, 0x700057ec20> as *const (), 0), // +0x515cd0 sub_515cd0
    Lifted::new(0x7000515d00, tpl_417::<0x7000624228, 0x700057eb50> as *const (), 0), // +0x515d00 sub_515d00
    Lifted::new(0x7000515d10, tpl_418::<0x70004b2bd0, 0x70005fb674, 0x7000624228, 0x700057ec20> as *const (), 0), // +0x515d10 sub_515d10
    Lifted::new(0x7000515d40, tpl_417::<0x7000624230, 0x700057eb50> as *const (), 0), // +0x515d40 sub_515d40
    Lifted::new(0x7000515d50, tpl_418::<0x70004b2bd0, 0x70005fb690, 0x7000624230, 0x700057ec20> as *const (), 0), // +0x515d50 sub_515d50
    Lifted::new(0x7000515d80, tpl_417::<0x7000624118, 0x700057eb50> as *const (), 0), // +0x515d80 sub_515d80
    Lifted::new(0x7000515d90, tpl_418::<0x70004b2bd0, 0x70005fb680, 0x7000624118, 0x700057ec20> as *const (), 0), // +0x515d90 sub_515d90
    Lifted::new(0x7000515dc0, tpl_417::<0x7000624128, 0x700057eb50> as *const (), 0), // +0x515dc0 sub_515dc0
    Lifted::new(0x7000515dd0, tpl_418::<0x70004b2bd0, 0x70005fb6a8, 0x7000624128, 0x700057ec20> as *const (), 0), // +0x515dd0 sub_515dd0
    Lifted::new(0x7000515e00, fn_7000515e00 as *const (), 2), // +0x515e00 sub_515e00
    Lifted::new(0x7000516060, fn_7000516060 as *const (), 0), // +0x516060 sub_516060
    Lifted::new(0x7000516200, fn_7000516200 as *const (), 0), // +0x516200 sub_516200
    Lifted::new(0x70005164f0, fn_70005164f0 as *const (), 0), // +0x5164f0 sub_5164f0
    Lifted::new(0x70005166a0, fn_70005166a0 as *const (), 0), // +0x5166a0 sub_5166a0
    Lifted::new(0x7000516800, fn_7000516800 as *const (), 0), // +0x516800 sub_516800
    Lifted::new(0x70005169a0, fn_70005169a0 as *const (), 0), // +0x5169a0 sub_5169a0
    Lifted::new(0x7000516b90, fn_7000516b90 as *const (), 0), // +0x516b90 sub_516b90
    Lifted::new(0x7000516d70, fn_7000516d70 as *const (), 0), // +0x516d70 sub_516d70
    Lifted::new(0x7000517310, fn_7000517310 as *const (), 0), // +0x517310 sub_517310
    Lifted::new(0x7000517640, fn_7000517640 as *const (), 2), // +0x517640 sub_517640
    Lifted::new(0x7000517850, fn_7000517850 as *const (), 2), // +0x517850 sub_517850
    Lifted::new(0x7000517bf0, fn_7000517bf0 as *const (), 0), // +0x517bf0 sub_517bf0
    Lifted::new(0x7000517ed0, tpl_432::<0x70006245c0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70003c3c90, 0x7000697150> as *const (), 0), // +0x517ed0 sub_517ed0
    Lifted::new(0x70005180c0, fn_70005180c0 as *const (), 0), // +0x5180c0 sub_5180c0
    Lifted::new(0x70005183a0, tpl_432::<0x70006248c0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70003c3660, 0x7000697150> as *const (), 0), // +0x5183a0 sub_5183a0
    Lifted::new(0x7000518590, fn_7000518590 as *const (), 0), // +0x518590 sub_518590
    Lifted::new(0x7000518700, tpl_449::<0x10, 0x8, 0x7000624990, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0xb8, 0x18> as *const (), 0), // +0x518700 sub_518700
    Lifted::new(0x70005187f0, fn_70005187f0 as *const (), 0), // +0x5187f0 sub_5187f0
    Lifted::new(0x7000518b70, tpl_450::<0x70006247a0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70002a4670, 0x700004da10, 0x7000697150> as *const (), 0), // +0x518b70 sub_518b70
    Lifted::new(0x7000518e00, tpl_451::<0x7000624800, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70002a4670, 0x7000697150, 0x7000600c38, 0x700004da10> as *const (), 0), // +0x518e00 sub_518e00
    Lifted::new(0x7000519070, fn_7000519070 as *const (), 0), // +0x519070 sub_519070
    Lifted::new(0x7000519440, fn_7000519440 as *const (), 0), // +0x519440 sub_519440
    Lifted::new(0x7000519570, fn_7000519570 as *const (), 0), // +0x519570 sub_519570
    Lifted::new(0x700051c4b0, fn_700051c4b0 as *const (), 0), // +0x51c4b0 sub_51c4b0
    Lifted::new(0x700051c540, fn_700051c540 as *const (), 0), // +0x51c540 sub_51c540
    Lifted::new(0x700051c740, fn_700051c740 as *const (), 0), // +0x51c740 sub_51c740
    Lifted::new(0x700051c7d0, fn_700051c7d0 as *const (), 0), // +0x51c7d0 sub_51c7d0
    Lifted::new(0x700051c8f0, fn_700051c8f0 as *const (), 0), // +0x51c8f0 sub_51c8f0
    Lifted::new(0x700051c9f0, fn_700051c9f0 as *const (), 0), // +0x51c9f0 sub_51c9f0
    Lifted::new(0x700051cd70, fn_700051cd70 as *const (), 0), // +0x51cd70 sub_51cd70
    Lifted::new(0x700051ce00, fn_700051ce00 as *const (), 2), // +0x51ce00 sub_51ce00
    Lifted::new(0x700051ce50, tpl_452::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x18> as *const (), 2), // +0x51ce50 sub_51ce50
    Lifted::new(0x700051ced0, tpl_421::<0x70004b2bd0, 0x20, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x51ced0 sub_51ced0
    Lifted::new(0x700051cf20, tpl_452::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x20> as *const (), 2), // +0x51cf20 sub_51cf20
    Lifted::new(0x700051cfa0, tpl_421::<0x70004b2bd0, 0x28, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x51cfa0 sub_51cfa0
    Lifted::new(0x700051cff0, tpl_452::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x28> as *const (), 2), // +0x51cff0 sub_51cff0
    Lifted::new(0x700051d070, tpl_421::<0x70004b2bd0, 0x30, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x51d070 sub_51d070
    Lifted::new(0x700051d0c0, tpl_452::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x30> as *const (), 2), // +0x51d0c0 sub_51d0c0
    Lifted::new(0x700051d140, fn_700051d140 as *const (), 0), // +0x51d140 sub_51d140
    Lifted::new(0x700051d1e0, fn_700051d1e0 as *const (), 0), // +0x51d1e0 sub_51d1e0
    Lifted::new(0x700051d750, fn_700051d750 as *const (), 0), // +0x51d750 sub_51d750
    Lifted::new(0x700051d7e0, fn_700051d7e0 as *const (), 2), // +0x51d7e0 sub_51d7e0
    Lifted::new(0x700051d830, fn_700051d830 as *const (), 2), // +0x51d830 sub_51d830
    Lifted::new(0x700051d880, fn_700051d880 as *const (), 0), // +0x51d880 sub_51d880
    Lifted::new(0x700051df00, fn_700051df00 as *const (), 0), // +0x51df00 sub_51df00
    Lifted::new(0x700051e1b0, fn_700051e1b0 as *const (), 0), // +0x51e1b0 sub_51e1b0
    Lifted::new(0x700051e360, fn_700051e360 as *const (), 0), // +0x51e360 sub_51e360
    Lifted::new(0x700051e4f0, fn_700051e4f0 as *const (), 0), // +0x51e4f0 sub_51e4f0
    Lifted::new(0x700051e5c0, fn_700051e5c0 as *const (), 0), // +0x51e5c0 sub_51e5c0
    Lifted::new(0x700051e760, fn_700051e760 as *const (), 0), // +0x51e760 sub_51e760
    Lifted::new(0x700051e7f0, fn_700051e7f0 as *const (), 0), // +0x51e7f0 sub_51e7f0
    Lifted::new(0x700051e890, fn_700051e890 as *const (), 0), // +0x51e890 sub_51e890
    Lifted::new(0x700051e990, fn_700051e990 as *const (), 0), // +0x51e990 sub_51e990
    Lifted::new(0x700051ea50, tpl_453::<0x7000624e40, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x700024b3a0, 0x70003c0c80, 0x700010b4f0, 0x7000697150> as *const (), 0), // +0x51ea50 sub_51ea50
    Lifted::new(0x700051ec00, fn_700051ec00 as *const (), 0), // +0x51ec00 sub_51ec00
    Lifted::new(0x700051ed50, fn_700051ed50 as *const (), 0), // +0x51ed50 sub_51ed50
    Lifted::new(0x700051ef30, tpl_453::<0x70006250c0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x700024b3a0, 0x70003c0ef0, 0x700010b4f0, 0x7000697150> as *const (), 0), // +0x51ef30 sub_51ef30
    Lifted::new(0x700051f0e0, fn_700051f0e0 as *const (), 0), // +0x51f0e0 sub_51f0e0
    Lifted::new(0x700051f2d0, fn_700051f2d0 as *const (), 0), // +0x51f2d0 sub_51f2d0
    Lifted::new(0x700051f410, fn_700051f410 as *const (), 0), // +0x51f410 sub_51f410
    Lifted::new(0x700051f6c0, tpl_425::<0x18, 0x8, 0x70005a8840, 0x10, 0x28, 0x70005a88b0, 0x70003c9ab0> as *const (), 0), // +0x51f6c0 sub_51f6c0
    Lifted::new(0x700051f720, fn_700051f720 as *const (), 0), // +0x51f720 sub_51f720
    Lifted::new(0x700051f820, fn_700051f820 as *const (), 0), // +0x51f820 sub_51f820
    Lifted::new(0x70005200c0, fn_70005200c0 as *const (), 0), // +0x5200c0 sub_5200c0
    Lifted::new(0x7000520150, tpl_454::<0x70004b2bd0, 0x8, 0x128, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x520150 sub_520150
    Lifted::new(0x7000520220, fn_7000520220 as *const (), 0), // +0x520220 sub_520220
    Lifted::new(0x7000520260, fn_7000520260 as *const (), 0), // +0x520260 sub_520260
    Lifted::new(0x7000520330, fn_7000520330 as *const (), 0), // +0x520330 sub_520330
    Lifted::new(0x7000520450, fn_7000520450 as *const (), 0), // +0x520450 sub_520450
    Lifted::new(0x70005204d0, tpl_455::<0x70004b2bd0, 0x198, 0x8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x5204d0 sub_5204d0
    Lifted::new(0x7000520550, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f228, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x520550 sub_520550
    Lifted::new(0x70005205e0, tpl_448::<0x70004b2bd0, 0x8, 0x70003c1650, 0x28, 0x700010b4f0, 0x10, 0x18> as *const (), 0), // +0x5205e0 sub_5205e0
    Lifted::new(0x7000520670, fn_7000520670 as *const (), 0), // +0x520670 sub_520670
    Lifted::new(0x7000520830, fn_7000520830 as *const (), 0), // +0x520830 sub_520830
    Lifted::new(0x70005209c0, fn_70005209c0 as *const (), 0), // +0x5209c0 sub_5209c0
    Lifted::new(0x7000520b60, fn_7000520b60 as *const (), 0), // +0x520b60 sub_520b60
    Lifted::new(0x7000520be0, tpl_457::<0x70004b2bd0, 0x8, 0x88, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x520be0 sub_520be0
    Lifted::new(0x7000520c50, fn_7000520c50 as *const (), 2), // +0x520c50 sub_520c50
    Lifted::new(0x7000520cf0, tpl_458::<0x70004b2bd0, 0x8, 0x30, 0x88, 0x700062e470, 0x2da0, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x520cf0 sub_520cf0
    Lifted::new(0x7000520d70, tpl_459::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x30, 0x2da0, 0x700062e470, 0x98> as *const (), 2), // +0x520d70 sub_520d70
    Lifted::new(0x7000520e10, tpl_458::<0x70004b2bd0, 0x8, 0x30, 0x88, 0x700062e470, 0x2da4, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x520e10 sub_520e10
    Lifted::new(0x7000520e90, tpl_459::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x30, 0x2da4, 0x700062e470, 0x98> as *const (), 2), // +0x520e90 sub_520e90
    Lifted::new(0x7000520f30, tpl_457::<0x70004b2bd0, 0x8, 0x80, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x520f30 sub_520f30
    Lifted::new(0x7000520fa0, fn_7000520fa0 as *const (), 2), // +0x520fa0 sub_520fa0
    Lifted::new(0x7000521040, tpl_458::<0x70004b2bd0, 0x8, 0x30, 0x80, 0x700062e470, 0x2da0, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x521040 sub_521040
    Lifted::new(0x70005210c0, tpl_459::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x30, 0x2da0, 0x700062e470, 0x90> as *const (), 2), // +0x5210c0 sub_5210c0
    Lifted::new(0x7000521160, tpl_458::<0x70004b2bd0, 0x8, 0x30, 0x80, 0x700062e470, 0x2da4, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x521160 sub_521160
    Lifted::new(0x70005211e0, tpl_459::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x30, 0x2da4, 0x700062e470, 0x90> as *const (), 2), // +0x5211e0 sub_5211e0
    Lifted::new(0x7000521280, tpl_460::<0x70004b2bd0, 0x8, 0xa0, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x521280 sub_521280
    Lifted::new(0x70005212f0, tpl_461::<0x70004b2bd0, 0x8, 0x30, 0xa0, 0x700062e470, 0x2da0, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x5212f0 sub_5212f0
    Lifted::new(0x7000521370, tpl_461::<0x70004b2bd0, 0x8, 0x30, 0xa0, 0x700062e470, 0x2da4, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x521370 sub_521370
    Lifted::new(0x70005213f0, tpl_460::<0x70004b2bd0, 0x8, 0xa8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x5213f0 sub_5213f0
    Lifted::new(0x7000521460, tpl_461::<0x70004b2bd0, 0x8, 0x30, 0xa8, 0x700062e470, 0x2da0, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x521460 sub_521460
    Lifted::new(0x70005214e0, tpl_461::<0x70004b2bd0, 0x8, 0x30, 0xa8, 0x700062e470, 0x2da4, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x5214e0 sub_5214e0
    Lifted::new(0x7000521560, tpl_423::<0x70004b2bd0, 0x8, 0x70003c04d0, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x521560 sub_521560
    Lifted::new(0x7000521630, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f220, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x521630 sub_521630
    Lifted::new(0x70005216c0, fn_70005216c0 as *const (), 0), // +0x5216c0 sub_5216c0
    Lifted::new(0x7000521830, tpl_423::<0x70004b2bd0, 0x8, 0x70003c6220, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x521830 sub_521830
    Lifted::new(0x7000521900, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x70003c6440, 0x7000697150> as *const (), 0), // +0x521900 sub_521900
    Lifted::new(0x7000521990, fn_7000521990 as *const (), 0), // +0x521990 sub_521990
    Lifted::new(0x7000521a80, fn_7000521a80 as *const (), 0), // +0x521a80 sub_521a80
    Lifted::new(0x7000521b10, tpl_417::<0x70006251d0, 0x700057eb50> as *const (), 0), // +0x521b10 sub_521b10
    Lifted::new(0x7000521b20, tpl_418::<0x70004b2bd0, 0x70005fb5c0, 0x70006251d0, 0x700057ec20> as *const (), 0), // +0x521b20 sub_521b20
    Lifted::new(0x7000521b50, tpl_417::<0x70006251e0, 0x700057eb50> as *const (), 0), // +0x521b50 sub_521b50
    Lifted::new(0x7000521b60, tpl_418::<0x70004b2bd0, 0x70005fb5b0, 0x70006251e0, 0x700057ec20> as *const (), 0), // +0x521b60 sub_521b60
    Lifted::new(0x7000521b90, tpl_417::<0x70006251f0, 0x700057eb50> as *const (), 0), // +0x521b90 sub_521b90
    Lifted::new(0x7000521ba0, tpl_418::<0x70004b2bd0, 0x70005fb64c, 0x70006251f0, 0x700057ec20> as *const (), 0), // +0x521ba0 sub_521ba0
    Lifted::new(0x7000521bd0, tpl_417::<0x7000625200, 0x700057eb50> as *const (), 0), // +0x521bd0 sub_521bd0
    Lifted::new(0x7000521be0, tpl_418::<0x70004b2bd0, 0x700060de90, 0x7000625200, 0x700057ec20> as *const (), 0), // +0x521be0 sub_521be0
    Lifted::new(0x7000521c10, tpl_417::<0x7000625178, 0x700057eb50> as *const (), 0), // +0x521c10 sub_521c10
    Lifted::new(0x7000521c20, tpl_418::<0x70004b2bd0, 0x700060df00, 0x7000625178, 0x700057ec20> as *const (), 0), // +0x521c20 sub_521c20
    Lifted::new(0x7000521c50, tpl_417::<0x7000625188, 0x700057eb50> as *const (), 0), // +0x521c50 sub_521c50
    Lifted::new(0x7000521c60, tpl_418::<0x70004b2bd0, 0x700060def0, 0x7000625188, 0x700057ec20> as *const (), 0), // +0x521c60 sub_521c60
    Lifted::new(0x7000521c90, tpl_417::<0x70006251a0, 0x700057eb50> as *const (), 0), // +0x521c90 sub_521c90
    Lifted::new(0x7000521ca0, tpl_418::<0x70004b2bd0, 0x700060ded8, 0x70006251a0, 0x700057ec20> as *const (), 0), // +0x521ca0 sub_521ca0
    Lifted::new(0x7000521cd0, tpl_417::<0x70006251b8, 0x700057eb50> as *const (), 0), // +0x521cd0 sub_521cd0
    Lifted::new(0x7000521ce0, tpl_418::<0x70004b2bd0, 0x700060dec8, 0x70006251b8, 0x700057ec20> as *const (), 0), // +0x521ce0 sub_521ce0
    Lifted::new(0x7000521d10, tpl_417::<0x7000625370, 0x700057eb50> as *const (), 0), // +0x521d10 sub_521d10
    Lifted::new(0x7000521d20, tpl_418::<0x70004b2bd0, 0x700060df50, 0x7000625370, 0x700057ec20> as *const (), 0), // +0x521d20 sub_521d20
    Lifted::new(0x7000521d50, tpl_417::<0x7000625380, 0x700057eb50> as *const (), 0), // +0x521d50 sub_521d50
    Lifted::new(0x7000521d60, tpl_418::<0x70004b2bd0, 0x700060df38, 0x7000625380, 0x700057ec20> as *const (), 0), // +0x521d60 sub_521d60
    Lifted::new(0x7000521d90, tpl_417::<0x7000625398, 0x700057eb50> as *const (), 0), // +0x521d90 sub_521d90
    Lifted::new(0x7000521da0, tpl_418::<0x70004b2bd0, 0x700060df28, 0x7000625398, 0x700057ec20> as *const (), 0), // +0x521da0 sub_521da0
    Lifted::new(0x7000521dd0, tpl_462::<0x10, 0x8, 0x70006253b0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x1b0> as *const (), 0), // +0x521dd0 sub_521dd0
    Lifted::new(0x7000521ea0, fn_7000521ea0 as *const (), 0), // +0x521ea0 sub_521ea0
    Lifted::new(0x7000522050, tpl_462::<0x10, 0x8, 0x7000625280, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x1a8> as *const (), 0), // +0x522050 sub_522050
    Lifted::new(0x7000522120, fn_7000522120 as *const (), 0), // +0x522120 sub_522120
    Lifted::new(0x70005225c0, fn_70005225c0 as *const (), 0), // +0x5225c0 sub_5225c0
    Lifted::new(0x7000522860, fn_7000522860 as *const (), 0), // +0x522860 sub_522860
    Lifted::new(0x7000522b40, fn_7000522b40 as *const (), 0), // +0x522b40 sub_522b40
    Lifted::new(0x7000522c80, fn_7000522c80 as *const (), 0), // +0x522c80 sub_522c80
    Lifted::new(0x7000522e20, fn_7000522e20 as *const (), 0), // +0x522e20 sub_522e20
    Lifted::new(0x7000523140, fn_7000523140 as *const (), 0), // +0x523140 sub_523140
    Lifted::new(0x7000523360, fn_7000523360 as *const (), 0), // +0x523360 sub_523360
    Lifted::new(0x7000523580, fn_7000523580 as *const (), 0), // +0x523580 sub_523580
    Lifted::new(0x7000523750, tpl_432::<0x70006258d0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70003c3c90, 0x7000697150> as *const (), 0), // +0x523750 sub_523750
    Lifted::new(0x7000523940, tpl_432::<0x7000625940, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70003c3660, 0x7000697150> as *const (), 0), // +0x523940 sub_523940
    Lifted::new(0x7000523b30, fn_7000523b30 as *const (), 0), // +0x523b30 sub_523b30
    Lifted::new(0x7000523e70, fn_7000523e70 as *const (), 0), // +0x523e70 sub_523e70
    Lifted::new(0x7000524090, fn_7000524090 as *const (), 0), // +0x524090 sub_524090
    Lifted::new(0x7000524200, fn_7000524200 as *const (), 0), // +0x524200 sub_524200
    Lifted::new(0x70005243c0, fn_70005243c0 as *const (), 0), // +0x5243c0 sub_5243c0
    Lifted::new(0x7000524500, fn_7000524500 as *const (), 0), // +0x524500 sub_524500
    Lifted::new(0x70005247c0, tpl_463::<0x7000625c40, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70003c4710, 0x7000697150> as *const (), 0), // +0x5247c0 sub_5247c0
    Lifted::new(0x7000524940, fn_7000524940 as *const (), 0), // +0x524940 sub_524940
    Lifted::new(0x7000524b90, fn_7000524b90 as *const (), 0), // +0x524b90 sub_524b90
    Lifted::new(0x7000524d00, tpl_450::<0x7000625a90, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70002a4670, 0x700004da10, 0x7000697150> as *const (), 0), // +0x524d00 sub_524d00
    Lifted::new(0x7000524f90, tpl_451::<0x7000625af0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70002a4670, 0x7000697150, 0x7000600c38, 0x700004da10> as *const (), 0), // +0x524f90 sub_524f90
    Lifted::new(0x7000525200, fn_7000525200 as *const (), 0), // +0x525200 sub_525200
    Lifted::new(0x7000525330, fn_7000525330 as *const (), 0), // +0x525330 sub_525330
    Lifted::new(0x70005254b0, fn_70005254b0 as *const (), 0), // +0x5254b0 sub_5254b0
    Lifted::new(0x7000525860, tpl_463::<0x7000625ec0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70003c1960, 0x7000697150> as *const (), 0), // +0x525860 sub_525860
    Lifted::new(0x70005259e0, fn_70005259e0 as *const (), 0), // +0x5259e0 sub_5259e0
    Lifted::new(0x7000525bc0, fn_7000525bc0 as *const (), 0), // +0x525bc0 sub_525bc0
    Lifted::new(0x7000525dc0, fn_7000525dc0 as *const (), 0), // +0x525dc0 sub_525dc0
    Lifted::new(0x7000525fd0, fn_7000525fd0 as *const (), 0), // +0x525fd0 sub_525fd0
    Lifted::new(0x700052a980, fn_700052a980 as *const (), 0), // +0x52a980 sub_52a980
    Lifted::new(0x700052aa10, fn_700052aa10 as *const (), 0), // +0x52aa10 sub_52aa10
    Lifted::new(0x700052ac50, fn_700052ac50 as *const (), 0), // +0x52ac50 sub_52ac50
    Lifted::new(0x700052ace0, fn_700052ace0 as *const (), 0), // +0x52ace0 sub_52ace0
    Lifted::new(0x700052ad40, tpl_420::<0x70004b2bd0, 0x20, 0x8, 0x21, 0x30, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x52ad40 sub_52ad40
    Lifted::new(0x700052adc0, fn_700052adc0 as *const (), 0), // +0x52adc0 sub_52adc0
    Lifted::new(0x700052b0c0, fn_700052b0c0 as *const (), 0), // +0x52b0c0 sub_52b0c0
    Lifted::new(0x700052b150, tpl_424::<0x70004b2bd0, 0x88, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x52b150 sub_52b150
    Lifted::new(0x700052b1a0, tpl_424::<0x70004b2bd0, 0x8c, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x52b1a0 sub_52b1a0
    Lifted::new(0x700052b1f0, tpl_445::<0x70004b2bd0, 0x90, 0x8, 0xb0, 0x70005a8e60, 0x10, 0x18> as *const (), 0), // +0x52b1f0 sub_52b1f0
    Lifted::new(0x700052b250, fn_700052b250 as *const (), 2), // +0x52b250 sub_52b250
    Lifted::new(0x700052b630, fn_700052b630 as *const (), 0), // +0x52b630 sub_52b630
    Lifted::new(0x700052baf0, fn_700052baf0 as *const (), 0), // +0x52baf0 sub_52baf0
    Lifted::new(0x700052bb80, tpl_455::<0x70004b2bd0, 0x20, 0x8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x52bb80 sub_52bb80
    Lifted::new(0x700052bc00, tpl_447::<0x70004b2bd0, 0x38, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x52bc00 sub_52bc00
    Lifted::new(0x700052bc70, tpl_447::<0x70004b2bd0, 0x40, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x52bc70 sub_52bc70
    Lifted::new(0x700052bce0, tpl_464::<0x70004b2bd0, 0x30, 0x18> as *const (), 0), // +0x52bce0 sub_52bce0
    Lifted::new(0x700052bd10, tpl_465::<0x70004b2bd0, 0x28, 0x8, 0x10, 0x18> as *const (), 0), // +0x52bd10 sub_52bd10
    Lifted::new(0x700052bd60, tpl_465::<0x70004b2bd0, 0x29, 0x8, 0x10, 0x18> as *const (), 0), // +0x52bd60 sub_52bd60
    Lifted::new(0x700052bdb0, tpl_465::<0x70004b2bd0, 0x2c, 0x8, 0x10, 0x18> as *const (), 0), // +0x52bdb0 sub_52bdb0
    Lifted::new(0x700052be00, fn_700052be00 as *const (), 0), // +0x52be00 sub_52be00
    Lifted::new(0x700052c0d0, fn_700052c0d0 as *const (), 0), // +0x52c0d0 sub_52c0d0
    Lifted::new(0x700052c1a0, tpl_466::<0x10, 0x8, 0x70006263c0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x2b> as *const (), 0), // +0x52c1a0 sub_52c1a0
    Lifted::new(0x700052c270, tpl_466::<0x10, 0x8, 0x7000626430, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x2a> as *const (), 0), // +0x52c270 sub_52c270
    Lifted::new(0x700052c340, fn_700052c340 as *const (), 0), // +0x52c340 sub_52c340
    Lifted::new(0x700052c720, fn_700052c720 as *const (), 2), // +0x52c720 sub_52c720
    Lifted::new(0x700052d220, fn_700052d220 as *const (), 0), // +0x52d220 sub_52d220
    Lifted::new(0x700052d2b0, fn_700052d2b0 as *const (), 0), // +0x52d2b0 sub_52d2b0
    Lifted::new(0x700052d430, fn_700052d430 as *const (), 0), // +0x52d430 sub_52d430
    Lifted::new(0x700052d670, fn_700052d670 as *const (), 0), // +0x52d670 sub_52d670
    Lifted::new(0x700052d700, tpl_467::<0x7000626640, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70004b9560, 0x70006266a8, 0x70005a8890, 0x70004b5280, 0x70004b5010, 0x70003ce3f0> as *const (), 0), // +0x52d700 sub_52d700
    Lifted::new(0x700052d970, tpl_468::<0x70006267e0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8870, 0x7000626770, 0x70004b5120, 0x70003cea90> as *const (), 0), // +0x52d970 sub_52d970
    Lifted::new(0x700052db20, tpl_467::<0x7000626840, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70004b9560, 0x70006266a8, 0x70005a8890, 0x70004b5280, 0x70004b5010, 0x70003ce7d0> as *const (), 0), // +0x52db20 sub_52db20
    Lifted::new(0x700052dd90, fn_700052dd90 as *const (), 0), // +0x52dd90 sub_52dd90
    Lifted::new(0x700052e0c0, tpl_446::<0x70004b2bd0, 0x8, 0x58, 0x18, 0x28, 0x700010b4f0, 0x10> as *const (), 0), // +0x52e0c0 sub_52e0c0
    Lifted::new(0x700052e140, fn_700052e140 as *const (), 0), // +0x52e140 sub_52e140
    Lifted::new(0x700052e5b0, fn_700052e5b0 as *const (), 0), // +0x52e5b0 sub_52e5b0
    Lifted::new(0x700052e810, fn_700052e810 as *const (), 0), // +0x52e810 sub_52e810
    Lifted::new(0x700052e8a0, fn_700052e8a0 as *const (), 0), // +0x52e8a0 sub_52e8a0
    Lifted::new(0x700052e970, fn_700052e970 as *const (), 0), // +0x52e970 sub_52e970
    Lifted::new(0x700052e9c0, tpl_469::<0x70004b2bd0, 0x18, 0x31, 0x8, 0x10> as *const (), 0), // +0x52e9c0 sub_52e9c0
    Lifted::new(0x700052ea10, tpl_470::<0x70004b2bd0, 0x18, 0x8, 0x34, 0x70005a8b10, 0x10> as *const (), 2), // +0x52ea10 sub_52ea10
    Lifted::new(0x700052ea70, fn_700052ea70 as *const (), 0), // +0x52ea70 sub_52ea70
    Lifted::new(0x700052eb00, fn_700052eb00 as *const (), 2), // +0x52eb00 sub_52eb00
    Lifted::new(0x700052ebf0, fn_700052ebf0 as *const (), 0), // +0x52ebf0 sub_52ebf0
    Lifted::new(0x700052ed10, fn_700052ed10 as *const (), 0), // +0x52ed10 sub_52ed10
    Lifted::new(0x700052ed60, tpl_423::<0x70004b2bd0, 0x8, 0x7000574d10, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x52ed60 sub_52ed60
    Lifted::new(0x700052ee30, fn_700052ee30 as *const (), 0), // +0x52ee30 sub_52ee30
    Lifted::new(0x700052eff0, fn_700052eff0 as *const (), 0), // +0x52eff0 sub_52eff0
    Lifted::new(0x700052f7c0, fn_700052f7c0 as *const (), 0), // +0x52f7c0 sub_52f7c0
    Lifted::new(0x700052f850, fn_700052f850 as *const (), 2), // +0x52f850 sub_52f850
    Lifted::new(0x700052f8b0, tpl_470::<0x70004b2bd0, 0x18, 0x8, 0x4, 0x70005a8b10, 0x10> as *const (), 2), // +0x52f8b0 sub_52f8b0
    Lifted::new(0x700052f910, fn_700052f910 as *const (), 0), // +0x52f910 sub_52f910
    Lifted::new(0x700052fc70, fn_700052fc70 as *const (), 0), // +0x52fc70 sub_52fc70
    Lifted::new(0x700052fd00, tpl_471::<0x70004b2bd0, 0x58, 0x8, 0x10, 0x28, 0x20, 0x7000697150, 0x18> as *const (), 0), // +0x52fd00 sub_52fd00
    Lifted::new(0x700052fde0, fn_700052fde0 as *const (), 0), // +0x52fde0 sub_52fde0
    Lifted::new(0x7000530210, fn_7000530210 as *const (), 0), // +0x530210 sub_530210
    Lifted::new(0x7000530470, fn_7000530470 as *const (), 0), // +0x530470 sub_530470
    Lifted::new(0x7000530500, tpl_469::<0x70004b2bd0, 0x18, 0x53, 0x8, 0x10> as *const (), 0), // +0x530500 sub_530500
    Lifted::new(0x7000530550, tpl_469::<0x70004b2bd0, 0x18, 0x52, 0x8, 0x10> as *const (), 0), // +0x530550 sub_530550
    Lifted::new(0x70005305a0, tpl_472::<0x70004b2bd0, 0x18, 0x52, 0x50, 0x4, 0xc, 0x8, 0x7000573490, 0x10> as *const (), 2), // +0x5305a0 sub_5305a0
    Lifted::new(0x7000530660, fn_7000530660 as *const (), 2), // +0x530660 sub_530660
    Lifted::new(0x7000530720, tpl_472::<0x70004b2bd0, 0x18, 0x52, 0x50, 0x1c, 0x24, 0x8, 0x7000573490, 0x10> as *const (), 2), // +0x530720 sub_530720
    Lifted::new(0x70005307e0, fn_70005307e0 as *const (), 2), // +0x5307e0 sub_5307e0
    Lifted::new(0x7000530a00, tpl_472::<0x70004b2bd0, 0x18, 0x53, 0x51, 0x38, 0x40, 0x8, 0x7000573490, 0x10> as *const (), 2), // +0x530a00 sub_530a00
    Lifted::new(0x7000530ac0, tpl_472::<0x70004b2bd0, 0x18, 0x53, 0x51, 0x44, 0x4c, 0x8, 0x7000573490, 0x10> as *const (), 2), // +0x530ac0 sub_530ac0
    Lifted::new(0x7000530b80, fn_7000530b80 as *const (), 0), // +0x530b80 sub_530b80
    Lifted::new(0x7000531200, fn_7000531200 as *const (), 0), // +0x531200 sub_531200
    Lifted::new(0x7000531290, fn_7000531290 as *const (), 0), // +0x531290 sub_531290
    Lifted::new(0x7000531320, tpl_424::<0x70004b2bd0, 0x30, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x531320 sub_531320
    Lifted::new(0x7000531370, tpl_424::<0x70004b2bd0, 0x20, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x531370 sub_531370
    Lifted::new(0x70005313c0, fn_70005313c0 as *const (), 0), // +0x5313c0 sub_5313c0
    Lifted::new(0x70005314e0, fn_70005314e0 as *const (), 0), // +0x5314e0 sub_5314e0
    Lifted::new(0x70005315b0, fn_70005315b0 as *const (), 0), // +0x5315b0 sub_5315b0
    Lifted::new(0x7000531680, fn_7000531680 as *const (), 0), // +0x531680 sub_531680
    Lifted::new(0x7000531800, tpl_473::<0x70006269f0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x700004de60, 0x70005a8ca0, 0x700023bd50, 0x7000697150> as *const (), 0), // +0x531800 sub_531800
    Lifted::new(0x7000531b50, tpl_473::<0x7000626b80, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x700004de60, 0x70005a8ca0, 0x700023c6d0, 0x7000697150> as *const (), 0), // +0x531b50 sub_531b50
    Lifted::new(0x7000531ea0, fn_7000531ea0 as *const (), 0), // +0x531ea0 sub_531ea0
    Lifted::new(0x70005324f0, fn_70005324f0 as *const (), 0), // +0x5324f0 sub_5324f0
    Lifted::new(0x7000532580, fn_7000532580 as *const (), 0), // +0x532580 sub_532580
    Lifted::new(0x7000532610, fn_7000532610 as *const (), 0), // +0x532610 sub_532610
    Lifted::new(0x70005326a0, tpl_424::<0x70004b2bd0, 0x1d8, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x5326a0 sub_5326a0
    Lifted::new(0x70005326f0, tpl_474::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f148, 0x70003c21a0, 0x1e8, 0x700023ddb0, 0x70002424d0> as *const (), 0), // +0x5326f0 sub_5326f0
    Lifted::new(0x70005327a0, tpl_424::<0x70004b2bd0, 0x1dc, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x5327a0 sub_5327a0
    Lifted::new(0x70005327f0, tpl_474::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f150, 0x70003c21a0, 0x1e8, 0x700023ddb0, 0x70002424d0> as *const (), 0), // +0x5327f0 sub_5327f0
    Lifted::new(0x70005328a0, fn_70005328a0 as *const (), 0), // +0x5328a0 sub_5328a0
    Lifted::new(0x7000532ab0, fn_7000532ab0 as *const (), 0), // +0x532ab0 sub_532ab0
    Lifted::new(0x7000532c80, fn_7000532c80 as *const (), 0), // +0x532c80 sub_532c80
    Lifted::new(0x7000533130, fn_7000533130 as *const (), 0), // +0x533130 sub_533130
    Lifted::new(0x70005331c0, fn_70005331c0 as *const (), 0), // +0x5331c0 sub_5331c0
    Lifted::new(0x7000533270, fn_7000533270 as *const (), 0), // +0x533270 sub_533270
    Lifted::new(0x7000533570, fn_7000533570 as *const (), 0), // +0x533570 sub_533570
    Lifted::new(0x7000533600, fn_7000533600 as *const (), 0), // +0x533600 sub_533600
    Lifted::new(0x7000533690, fn_7000533690 as *const (), 0), // +0x533690 sub_533690
    Lifted::new(0x7000533720, tpl_475::<0x70004b2bd0, 0x700024b920, 0x7000697150> as *const (), 0), // +0x533720 sub_533720
    Lifted::new(0x7000533800, tpl_448::<0x70004b2bd0, 0x8, 0x7000574c30, 0x28, 0x700010b4f0, 0x10, 0x18> as *const (), 0), // +0x533800 sub_533800
    Lifted::new(0x7000533890, fn_7000533890 as *const (), 0), // +0x533890 sub_533890
    Lifted::new(0x7000533930, fn_7000533930 as *const (), 0), // +0x533930 sub_533930
    Lifted::new(0x70005339f0, fn_70005339f0 as *const (), 2), // +0x5339f0 sub_5339f0
    Lifted::new(0x7000533b20, fn_7000533b20 as *const (), 2), // +0x533b20 sub_533b20
    Lifted::new(0x7000533c50, fn_7000533c50 as *const (), 2), // +0x533c50 sub_533c50
    Lifted::new(0x7000533d10, fn_7000533d10 as *const (), 2), // +0x533d10 sub_533d10
    Lifted::new(0x7000533dd0, fn_7000533dd0 as *const (), 0), // +0x533dd0 sub_533dd0
    Lifted::new(0x70005340f0, fn_70005340f0 as *const (), 0), // +0x5340f0 sub_5340f0
    Lifted::new(0x7000534750, fn_7000534750 as *const (), 0), // +0x534750 sub_534750
    Lifted::new(0x70005347e0, fn_70005347e0 as *const (), 0), // +0x5347e0 sub_5347e0
    Lifted::new(0x7000534870, fn_7000534870 as *const (), 0), // +0x534870 sub_534870
    Lifted::new(0x7000534900, fn_7000534900 as *const (), 0), // +0x534900 sub_534900
    Lifted::new(0x7000534a00, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f3b0, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x534a00 sub_534a00
    Lifted::new(0x7000534a90, tpl_424::<0x70004b2bd0, 0x210, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x534a90 sub_534a90
    Lifted::new(0x7000534ae0, tpl_476::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f148, 0x70003c21a0> as *const (), 0), // +0x534ae0 sub_534ae0
    Lifted::new(0x7000534b70, tpl_424::<0x70004b2bd0, 0x214, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x534b70 sub_534b70
    Lifted::new(0x7000534bc0, tpl_476::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f150, 0x70003c21a0> as *const (), 0), // +0x534bc0 sub_534bc0
    Lifted::new(0x7000534c50, fn_7000534c50 as *const (), 0), // +0x534c50 sub_534c50
    Lifted::new(0x7000534e50, fn_7000534e50 as *const (), 0), // +0x534e50 sub_534e50
    Lifted::new(0x7000535250, fn_7000535250 as *const (), 0), // +0x535250 sub_535250
    Lifted::new(0x7000535760, fn_7000535760 as *const (), 0), // +0x535760 sub_535760
    Lifted::new(0x70005357f0, tpl_477::<0x70004b2bd0, 0x8, 0x1d8, 0x70001a4530, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x5357f0 sub_5357f0
    Lifted::new(0x70005358c0, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x70002526b0, 0x7000697150> as *const (), 0), // +0x5358c0 sub_5358c0
    Lifted::new(0x7000535950, fn_7000535950 as *const (), 0), // +0x535950 sub_535950
    Lifted::new(0x7000535a00, fn_7000535a00 as *const (), 0), // +0x535a00 sub_535a00
    Lifted::new(0x7000535a60, tpl_424::<0x70004b2bd0, 0x1f8, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x535a60 sub_535a60
    Lifted::new(0x7000535ab0, tpl_478::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f428, 0x70003c21a0> as *const (), 0), // +0x535ab0 sub_535ab0
    Lifted::new(0x7000535b40, tpl_424::<0x70004b2bd0, 0x1fc, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x535b40 sub_535b40
    Lifted::new(0x7000535b90, tpl_478::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f420, 0x70003c21a0> as *const (), 0), // +0x535b90 sub_535b90
    Lifted::new(0x7000535c20, tpl_424::<0x70004b2bd0, 0x200, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x535c20 sub_535c20
    Lifted::new(0x7000535c70, fn_7000535c70 as *const (), 0), // +0x535c70 sub_535c70
    Lifted::new(0x7000535d00, tpl_424::<0x70004b2bd0, 0x208, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x535d00 sub_535d00
    Lifted::new(0x7000535d50, fn_7000535d50 as *const (), 0), // +0x535d50 sub_535d50
    Lifted::new(0x7000535e20, tpl_424::<0x70004b2bd0, 0x20c, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x535e20 sub_535e20
    Lifted::new(0x7000535e70, fn_7000535e70 as *const (), 0), // +0x535e70 sub_535e70
    Lifted::new(0x7000535f40, tpl_479::<0x70004b2bd0, 0x210, 0x700068f3f8, 0x700068f3e8, 0x700068f3f0, 0x8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x535f40 sub_535f40
    Lifted::new(0x7000535fe0, fn_7000535fe0 as *const (), 0), // +0x535fe0 sub_535fe0
    Lifted::new(0x70005360c0, fn_70005360c0 as *const (), 0), // +0x5360c0 sub_5360c0
    Lifted::new(0x70005361d0, fn_70005361d0 as *const (), 0), // +0x5361d0 sub_5361d0
    Lifted::new(0x7000536490, fn_7000536490 as *const (), 0), // +0x536490 sub_536490
    Lifted::new(0x7000536870, fn_7000536870 as *const (), 0), // +0x536870 sub_536870
    Lifted::new(0x7000537360, fn_7000537360 as *const (), 0), // +0x537360 sub_537360
    Lifted::new(0x70005373f0, fn_70005373f0 as *const (), 0), // +0x5373f0 sub_5373f0
    Lifted::new(0x70005374f0, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f2f0, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x5374f0 sub_5374f0
    Lifted::new(0x7000537580, tpl_420::<0x70004b2bd0, 0x1d0, 0x8, 0x1d1, 0x1e0, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x537580 sub_537580
    Lifted::new(0x7000537610, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f1e0, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x537610 sub_537610
    Lifted::new(0x70005376a0, tpl_420::<0x70004b2bd0, 0x1e8, 0x8, 0x1e9, 0x1f8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x5376a0 sub_5376a0
    Lifted::new(0x7000537730, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f440, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x537730 sub_537730
    Lifted::new(0x70005377c0, fn_70005377c0 as *const (), 0), // +0x5377c0 sub_5377c0
    Lifted::new(0x7000537c40, fn_7000537c40 as *const (), 0), // +0x537c40 sub_537c40
    Lifted::new(0x7000537cd0, tpl_465::<0x70004b2bd0, 0x290, 0x8, 0x10, 0x18> as *const (), 0), // +0x537cd0 sub_537cd0
    Lifted::new(0x7000537d20, tpl_480::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x700068f3a0, 0x70003c2100, 0x7000697150, 0x70003c85e0, 0x1b8> as *const (), 0), // +0x537d20 sub_537d20
    Lifted::new(0x7000537e00, fn_7000537e00 as *const (), 0), // +0x537e00 sub_537e00
    Lifted::new(0x7000537e90, tpl_420::<0x70004b2bd0, 0x200, 0x8, 0x201, 0x210, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x537e90 sub_537e90
    Lifted::new(0x7000537f20, tpl_420::<0x70004b2bd0, 0x218, 0x8, 0x219, 0x228, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x537f20 sub_537f20
    Lifted::new(0x7000537fb0, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f380, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x537fb0 sub_537fb0
    Lifted::new(0x7000538040, tpl_481::<0x70004b2bd0, 0x230, 0x40000000, 0x18, 0x8, 0x70005a8aa8, 0x10> as *const (), 0), // +0x538040 sub_538040
    Lifted::new(0x70005380b0, tpl_420::<0x70004b2bd0, 0x238, 0x8, 0x239, 0x248, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x5380b0 sub_5380b0
    Lifted::new(0x7000538140, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f388, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x538140 sub_538140
    Lifted::new(0x70005381d0, tpl_475::<0x70004b2bd0, 0x700024e670, 0x7000697150> as *const (), 0), // +0x5381d0 sub_5381d0
    Lifted::new(0x70005382b0, tpl_481::<0x70004b2bd0, 0x250, 0x40000000, 0x18, 0x8, 0x70005a8aa8, 0x10> as *const (), 0), // +0x5382b0 sub_5382b0
    Lifted::new(0x7000538320, tpl_465::<0x70004b2bd0, 0x289, 0x8, 0x10, 0x18> as *const (), 0), // +0x538320 sub_538320
    Lifted::new(0x7000538370, fn_7000538370 as *const (), 2), // +0x538370 sub_538370
    Lifted::new(0x70005383d0, fn_70005383d0 as *const (), 2), // +0x5383d0 sub_5383d0
    Lifted::new(0x7000538460, tpl_421::<0x70004b2bd0, 0x268, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x538460 sub_538460
    Lifted::new(0x70005384b0, tpl_465::<0x70004b2bd0, 0x288, 0x8, 0x10, 0x18> as *const (), 0), // +0x5384b0 sub_5384b0
    Lifted::new(0x7000538500, tpl_421::<0x70004b2bd0, 0x270, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x538500 sub_538500
    Lifted::new(0x7000538550, tpl_452::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x270> as *const (), 2), // +0x538550 sub_538550
    Lifted::new(0x70005385d0, tpl_421::<0x70004b2bd0, 0x278, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x5385d0 sub_5385d0
    Lifted::new(0x7000538620, tpl_452::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x278> as *const (), 2), // +0x538620 sub_538620
    Lifted::new(0x70005386a0, fn_70005386a0 as *const (), 0), // +0x5386a0 sub_5386a0
    Lifted::new(0x7000538780, tpl_465::<0x70004b2bd0, 0x28a, 0x8, 0x10, 0x18> as *const (), 0), // +0x538780 sub_538780
    Lifted::new(0x70005387d0, tpl_465::<0x70004b2bd0, 0x28b, 0x8, 0x10, 0x18> as *const (), 0), // +0x5387d0 sub_5387d0
    Lifted::new(0x7000538820, tpl_480::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x700068f3a8, 0x70003c2100, 0x7000697150, 0x70003c85e0, 0x1b8> as *const (), 0), // +0x538820 sub_538820
    Lifted::new(0x7000538900, tpl_465::<0x70004b2bd0, 0x28c, 0x8, 0x10, 0x18> as *const (), 0), // +0x538900 sub_538900
    Lifted::new(0x7000538950, tpl_480::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x700068f390, 0x70003c2100, 0x7000697150, 0x70003c85e0, 0x1b8> as *const (), 0), // +0x538950 sub_538950
    Lifted::new(0x7000538a30, tpl_421::<0x70004b2bd0, 0x280, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x538a30 sub_538a30
    Lifted::new(0x7000538a80, fn_7000538a80 as *const (), 2), // +0x538a80 sub_538a80
    Lifted::new(0x7000538bd0, tpl_465::<0x70004b2bd0, 0x28d, 0x8, 0x10, 0x18> as *const (), 0), // +0x538bd0 sub_538bd0
    Lifted::new(0x7000538c20, fn_7000538c20 as *const (), 2), // +0x538c20 sub_538c20
    Lifted::new(0x7000538cf0, tpl_417::<0x7000626f30, 0x700057eb50> as *const (), 0), // +0x538cf0 sub_538cf0
    Lifted::new(0x7000538d00, tpl_418::<0x70004b2bd0, 0x70005fb6f0, 0x7000626f30, 0x700057ec20> as *const (), 0), // +0x538d00 sub_538d00
    Lifted::new(0x7000538d30, tpl_417::<0x7000626fd0, 0x700057eb50> as *const (), 0), // +0x538d30 sub_538d30
    Lifted::new(0x7000538d40, tpl_418::<0x70004b2bd0, 0x70005fb6e8, 0x7000626fd0, 0x700057ec20> as *const (), 0), // +0x538d40 sub_538d40
    Lifted::new(0x7000538d70, tpl_417::<0x7000626fe0, 0x700057eb50> as *const (), 0), // +0x538d70 sub_538d70
    Lifted::new(0x7000538d80, tpl_418::<0x70004b2bd0, 0x70005fb708, 0x7000626fe0, 0x700057ec20> as *const (), 0), // +0x538d80 sub_538d80
    Lifted::new(0x7000538db0, tpl_417::<0x7000626fe8, 0x700057eb50> as *const (), 0), // +0x538db0 sub_538db0
    Lifted::new(0x7000538dc0, tpl_418::<0x70004b2bd0, 0x700060df60, 0x7000626fe8, 0x700057ec20> as *const (), 0), // +0x538dc0 sub_538dc0
    Lifted::new(0x7000538df0, tpl_417::<0x7000626ff8, 0x700057eb50> as *const (), 0), // +0x538df0 sub_538df0
    Lifted::new(0x7000538e00, tpl_418::<0x70004b2bd0, 0x70005fb728, 0x7000626ff8, 0x700057ec20> as *const (), 0), // +0x538e00 sub_538e00
    Lifted::new(0x7000538e30, tpl_417::<0x7000626f90, 0x700057eb50> as *const (), 0), // +0x538e30 sub_538e30
    Lifted::new(0x7000538e40, tpl_418::<0x70004b2bd0, 0x70005fb720, 0x7000626f90, 0x700057ec20> as *const (), 0), // +0x538e40 sub_538e40
    Lifted::new(0x7000538e70, tpl_417::<0x7000626fa0, 0x700057eb50> as *const (), 0), // +0x538e70 sub_538e70
    Lifted::new(0x7000538e80, tpl_418::<0x70004b2bd0, 0x70005fb740, 0x7000626fa0, 0x700057ec20> as *const (), 0), // +0x538e80 sub_538e80
    Lifted::new(0x7000538eb0, tpl_417::<0x7000626fb0, 0x700057eb50> as *const (), 0), // +0x538eb0 sub_538eb0
    Lifted::new(0x7000538ec0, tpl_418::<0x70004b2bd0, 0x70005fb730, 0x7000626fb0, 0x700057ec20> as *const (), 0), // +0x538ec0 sub_538ec0
    Lifted::new(0x7000538ef0, tpl_417::<0x7000626fc0, 0x700057eb50> as *const (), 0), // +0x538ef0 sub_538ef0
    Lifted::new(0x7000538f00, tpl_418::<0x70004b2bd0, 0x70005fb69c, 0x7000626fc0, 0x700057ec20> as *const (), 0), // +0x538f00 sub_538f00
    Lifted::new(0x7000538f30, fn_7000538f30 as *const (), 0), // +0x538f30 sub_538f30
    Lifted::new(0x70005390f0, fn_70005390f0 as *const (), 2), // +0x5390f0 sub_5390f0
    Lifted::new(0x70005392f0, fn_70005392f0 as *const (), 2), // +0x5392f0 sub_5392f0
    Lifted::new(0x70005394a0, tpl_430::<0x10, 0x8, 0x7000627250, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x700024e760> as *const (), 0), // +0x5394a0 sub_5394a0
    Lifted::new(0x7000539570, tpl_430::<0x10, 0x8, 0x7000627010, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x700024ea40> as *const (), 0), // +0x539570 sub_539570
    Lifted::new(0x7000539640, tpl_430::<0x10, 0x8, 0x7000627070, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x700024ead0> as *const (), 0), // +0x539640 sub_539640
    Lifted::new(0x7000539710, fn_7000539710 as *const (), 2), // +0x539710 sub_539710
    Lifted::new(0x700053bd20, fn_700053bd20 as *const (), 0), // +0x53bd20 sub_53bd20
    Lifted::new(0x700053bdb0, fn_700053bdb0 as *const (), 0), // +0x53bdb0 sub_53bdb0
    Lifted::new(0x700053be40, fn_700053be40 as *const (), 0), // +0x53be40 sub_53be40
    Lifted::new(0x700053bed0, tpl_482::<0x70004b2bd0, 0x8, 0x700068f1f8, 0x700068f228, 0x20, 0x198, 0x700068f220, 0x70003c0720, 0x188, 0x190, 0x700068f258, 0xb8> as *const (), 0), // +0x53bed0 sub_53bed0
    Lifted::new(0x700053bfb0, tpl_483::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x700068f1f8, 0x70003c21a0> as *const (), 0), // +0x53bfb0 sub_53bfb0
    Lifted::new(0x700053c010, tpl_482::<0x70004b2bd0, 0x8, 0x700068f1f0, 0x700068f228, 0x20, 0x198, 0x700068f220, 0x70003c0720, 0x188, 0x190, 0x700068f258, 0xb8> as *const (), 0), // +0x53c010 sub_53c010
    Lifted::new(0x700053c0f0, tpl_483::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x700068f1f0, 0x70003c21a0> as *const (), 0), // +0x53c0f0 sub_53c0f0
    Lifted::new(0x700053c150, fn_700053c150 as *const (), 0), // +0x53c150 sub_53c150
    Lifted::new(0x700053c2b0, fn_700053c2b0 as *const (), 0), // +0x53c2b0 sub_53c2b0
    Lifted::new(0x700053c360, fn_700053c360 as *const (), 0), // +0x53c360 sub_53c360
    Lifted::new(0x700053c3f0, fn_700053c3f0 as *const (), 0), // +0x53c3f0 sub_53c3f0
    Lifted::new(0x700053c4f0, fn_700053c4f0 as *const (), 0), // +0x53c4f0 sub_53c4f0
    Lifted::new(0x700053cb50, fn_700053cb50 as *const (), 0), // +0x53cb50 sub_53cb50
    Lifted::new(0x700053cbe0, tpl_484::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x1d0, 0x1, 0x700004de60, 0x7000697150> as *const (), 0), // +0x53cbe0 sub_53cbe0
    Lifted::new(0x700053cc90, tpl_484::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x1e8, 0x1, 0x700004de60, 0x7000697150> as *const (), 0), // +0x53cc90 sub_53cc90
    Lifted::new(0x700053cd40, tpl_484::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x200, 0x1, 0x700004de60, 0x7000697150> as *const (), 0), // +0x53cd40 sub_53cd40
    Lifted::new(0x700053cdf0, fn_700053cdf0 as *const (), 0), // +0x53cdf0 sub_53cdf0
    Lifted::new(0x700053d270, fn_700053d270 as *const (), 0), // +0x53d270 sub_53d270
    Lifted::new(0x700053d300, fn_700053d300 as *const (), 0), // +0x53d300 sub_53d300
    Lifted::new(0x700053d390, fn_700053d390 as *const (), 0), // +0x53d390 sub_53d390
    Lifted::new(0x700053d420, tpl_477::<0x70004b2bd0, 0x8, 0x1d0, 0x70001a4530, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x53d420 sub_53d420
    Lifted::new(0x700053d4f0, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x70002a77a0, 0x7000697150> as *const (), 0), // +0x53d4f0 sub_53d4f0
    Lifted::new(0x700053d580, tpl_424::<0x70004b2bd0, 0x204, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53d580 sub_53d580
    Lifted::new(0x700053d5d0, tpl_478::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f288, 0x70003c21a0> as *const (), 0), // +0x53d5d0 sub_53d5d0
    Lifted::new(0x700053d660, tpl_478::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f280, 0x70003c21a0> as *const (), 0), // +0x53d660 sub_53d660
    Lifted::new(0x700053d6f0, tpl_477::<0x70004b2bd0, 0x8, 0x218, 0x70001a4530, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x53d6f0 sub_53d6f0
    Lifted::new(0x700053d7c0, tpl_456::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700068f278, 0x70003c2100, 0x7000697150> as *const (), 0), // +0x53d7c0 sub_53d7c0
    Lifted::new(0x700053d850, tpl_424::<0x70004b2bd0, 0x270, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53d850 sub_53d850
    Lifted::new(0x700053d8a0, fn_700053d8a0 as *const (), 0), // +0x53d8a0 sub_53d8a0
    Lifted::new(0x700053d940, tpl_424::<0x70004b2bd0, 0x274, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53d940 sub_53d940
    Lifted::new(0x700053d990, fn_700053d990 as *const (), 0), // +0x53d990 sub_53d990
    Lifted::new(0x700053da20, tpl_479::<0x70004b2bd0, 0x278, 0x700068f3f8, 0x700068f3e8, 0x700068f3f0, 0x8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x53da20 sub_53da20
    Lifted::new(0x700053dac0, fn_700053dac0 as *const (), 0), // +0x53dac0 sub_53dac0
    Lifted::new(0x700053db50, fn_700053db50 as *const (), 0), // +0x53db50 sub_53db50
    Lifted::new(0x700053dc30, fn_700053dc30 as *const (), 0), // +0x53dc30 sub_53dc30
    Lifted::new(0x700053de90, fn_700053de90 as *const (), 0), // +0x53de90 sub_53de90
    Lifted::new(0x700053e1f0, fn_700053e1f0 as *const (), 0), // +0x53e1f0 sub_53e1f0
    Lifted::new(0x700053ee70, fn_700053ee70 as *const (), 0), // +0x53ee70 sub_53ee70
    Lifted::new(0x700053ef00, fn_700053ef00 as *const (), 0), // +0x53ef00 sub_53ef00
    Lifted::new(0x700053ef90, fn_700053ef90 as *const (), 0), // +0x53ef90 sub_53ef90
    Lifted::new(0x700053f020, tpl_424::<0x70004b2bd0, 0x2b8, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53f020 sub_53f020
    Lifted::new(0x700053f070, tpl_478::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f148, 0x70003c21a0> as *const (), 0), // +0x53f070 sub_53f070
    Lifted::new(0x700053f100, tpl_424::<0x70004b2bd0, 0x2bc, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53f100 sub_53f100
    Lifted::new(0x700053f150, tpl_478::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8b18, 0x4, 0x700068f150, 0x70003c21a0> as *const (), 0), // +0x53f150 sub_53f150
    Lifted::new(0x700053f1e0, tpl_424::<0x70004b2bd0, 0x2c0, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53f1e0 sub_53f1e0
    Lifted::new(0x700053f230, tpl_424::<0x70004b2bd0, 0x2c4, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x53f230 sub_53f230
    Lifted::new(0x700053f280, tpl_420::<0x70004b2bd0, 0x2c8, 0x8, 0x2c9, 0x2d8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x53f280 sub_53f280
    Lifted::new(0x700053f310, tpl_484::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x2c8, 0x1, 0x700004de60, 0x7000697150> as *const (), 0), // +0x53f310 sub_53f310
    Lifted::new(0x700053f3c0, fn_700053f3c0 as *const (), 0), // +0x53f3c0 sub_53f3c0
    Lifted::new(0x700053f980, fn_700053f980 as *const (), 0), // +0x53f980 sub_53f980
    Lifted::new(0x700053fa10, fn_700053fa10 as *const (), 0), // +0x53fa10 sub_53fa10
    Lifted::new(0x7000541540, fn_7000541540 as *const (), 0), // +0x541540 sub_541540
    Lifted::new(0x70005473b0, fn_70005473b0 as *const (), 0), // +0x5473b0 sub_5473b0
    Lifted::new(0x7000547540, fn_7000547540 as *const (), 0), // +0x547540 sub_547540
    Lifted::new(0x7000547940, fn_7000547940 as *const (), 0), // +0x547940 sub_547940
    Lifted::new(0x70005479a0, tpl_424::<0x70004b2bd0, 0x58, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x5479a0 sub_5479a0
    Lifted::new(0x70005479f0, tpl_423::<0x70004b2bd0, 0x8, 0x70005473b0, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x5479f0 sub_5479f0
    Lifted::new(0x7000547ac0, tpl_423::<0x70004b2bd0, 0x8, 0x7000547540, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x547ac0 sub_547ac0
    Lifted::new(0x7000547b90, tpl_424::<0x70004b2bd0, 0x68, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x547b90 sub_547b90
    Lifted::new(0x7000547be0, tpl_465::<0x70004b2bd0, 0x6c, 0x8, 0x10, 0x18> as *const (), 0), // +0x547be0 sub_547be0
    Lifted::new(0x7000547c30, tpl_465::<0x70004b2bd0, 0x61, 0x8, 0x10, 0x18> as *const (), 0), // +0x547c30 sub_547c30
    Lifted::new(0x7000547c80, tpl_465::<0x70004b2bd0, 0x60, 0x8, 0x10, 0x18> as *const (), 0), // +0x547c80 sub_547c80
    Lifted::new(0x7000547cd0, tpl_465::<0x70004b2bd0, 0x62, 0x8, 0x10, 0x18> as *const (), 0), // +0x547cd0 sub_547cd0
    Lifted::new(0x7000547d20, tpl_465::<0x70004b2bd0, 0x64, 0x8, 0x10, 0x18> as *const (), 0), // +0x547d20 sub_547d20
    Lifted::new(0x7000547d70, fn_7000547d70 as *const (), 0), // +0x547d70 sub_547d70
    Lifted::new(0x7000547ec0, fn_7000547ec0 as *const (), 0), // +0x547ec0 sub_547ec0
    Lifted::new(0x7000548a60, fn_7000548a60 as *const (), 0), // +0x548a60 sub_548a60
    Lifted::new(0x7000548f60, fn_7000548f60 as *const (), 2), // +0x548f60 sub_548f60
    Lifted::new(0x7000549bd0, fn_7000549bd0 as *const (), 0), // +0x549bd0 sub_549bd0
    Lifted::new(0x7000549c60, fn_7000549c60 as *const (), 0), // +0x549c60 sub_549c60
    Lifted::new(0x7000549e00, fn_7000549e00 as *const (), 0), // +0x549e00 sub_549e00
    Lifted::new(0x7000549ef0, fn_7000549ef0 as *const (), 0), // +0x549ef0 sub_549ef0
    Lifted::new(0x700054a080, tpl_423::<0x70004b2bd0, 0x8, 0x7000256fb0, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x54a080 sub_54a080
    Lifted::new(0x700054a150, fn_700054a150 as *const (), 0), // +0x54a150 sub_54a150
    Lifted::new(0x700054a240, fn_700054a240 as *const (), 0), // +0x54a240 sub_54a240
    Lifted::new(0x700054a3d0, fn_700054a3d0 as *const (), 0), // +0x54a3d0 sub_54a3d0
    Lifted::new(0x700054a4c0, tpl_423::<0x70004b2bd0, 0x8, 0x7000257170, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x54a4c0 sub_54a4c0
    Lifted::new(0x700054a590, fn_700054a590 as *const (), 0), // +0x54a590 sub_54a590
    Lifted::new(0x700054a680, tpl_423::<0x70004b2bd0, 0x8, 0x7000257320, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x54a680 sub_54a680
    Lifted::new(0x700054a750, fn_700054a750 as *const (), 0), // +0x54a750 sub_54a750
    Lifted::new(0x700054a840, fn_700054a840 as *const (), 0), // +0x54a840 sub_54a840
    Lifted::new(0x700054a9c0, fn_700054a9c0 as *const (), 0), // +0x54a9c0 sub_54a9c0
    Lifted::new(0x700054aab0, tpl_423::<0x70004b2bd0, 0x8, 0x70002574d0, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x54aab0 sub_54aab0
    Lifted::new(0x700054ab80, fn_700054ab80 as *const (), 0), // +0x54ab80 sub_54ab80
    Lifted::new(0x700054ac70, tpl_423::<0x70004b2bd0, 0x8, 0x7000257810, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x54ac70 sub_54ac70
    Lifted::new(0x700054ad40, fn_700054ad40 as *const (), 0), // +0x54ad40 sub_54ad40
    Lifted::new(0x700054ae30, fn_700054ae30 as *const (), 0), // +0x54ae30 sub_54ae30
    Lifted::new(0x700054aea0, tpl_434::<0x70006278c0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70002579d0, 0x7000697150> as *const (), 0), // +0x54aea0 sub_54aea0
    Lifted::new(0x700054afc0, fn_700054afc0 as *const (), 0), // +0x54afc0 sub_54afc0
    Lifted::new(0x700054b110, tpl_434::<0x7000627980, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x7000257e70, 0x7000697150> as *const (), 0), // +0x54b110 sub_54b110
    Lifted::new(0x700054b230, fn_700054b230 as *const (), 0), // +0x54b230 sub_54b230
    Lifted::new(0x700054be40, fn_700054be40 as *const (), 0), // +0x54be40 sub_54be40
    Lifted::new(0x700054bed0, fn_700054bed0 as *const (), 0), // +0x54bed0 sub_54bed0
    Lifted::new(0x700054bf40, tpl_477::<0x70004b2bd0, 0x8, 0x20, 0x70001a4530, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x54bf40 sub_54bf40
    Lifted::new(0x700054c010, fn_700054c010 as *const (), 2), // +0x54c010 sub_54c010
    Lifted::new(0x700054c5a0, fn_700054c5a0 as *const (), 0), // +0x54c5a0 sub_54c5a0
    Lifted::new(0x700054c630, fn_700054c630 as *const (), 0), // +0x54c630 sub_54c630
    Lifted::new(0x700054c6d0, fn_700054c6d0 as *const (), 0), // +0x54c6d0 sub_54c6d0
    Lifted::new(0x700054cbc0, fn_700054cbc0 as *const (), 0), // +0x54cbc0 sub_54cbc0
    Lifted::new(0x700054cf40, fn_700054cf40 as *const (), 0), // +0x54cf40 sub_54cf40
    Lifted::new(0x700054cfd0, fn_700054cfd0 as *const (), 2), // +0x54cfd0 sub_54cfd0
    Lifted::new(0x700054d0a0, fn_700054d0a0 as *const (), 2), // +0x54d0a0 sub_54d0a0
    Lifted::new(0x700054d190, fn_700054d190 as *const (), 2), // +0x54d190 sub_54d190
    Lifted::new(0x700054d240, fn_700054d240 as *const (), 2), // +0x54d240 sub_54d240
    Lifted::new(0x700054d320, fn_700054d320 as *const (), 2), // +0x54d320 sub_54d320
    Lifted::new(0x700054d4a0, fn_700054d4a0 as *const (), 0), // +0x54d4a0 sub_54d4a0
    Lifted::new(0x700054d5b0, fn_700054d5b0 as *const (), 0), // +0x54d5b0 sub_54d5b0
    Lifted::new(0x700054d900, fn_700054d900 as *const (), 2), // +0x54d900 sub_54d900
    Lifted::new(0x700054da50, fn_700054da50 as *const (), 0), // +0x54da50 sub_54da50
    Lifted::new(0x700054e100, fn_700054e100 as *const (), 0), // +0x54e100 sub_54e100
    Lifted::new(0x700054e190, fn_700054e190 as *const (), 0), // +0x54e190 sub_54e190
    Lifted::new(0x700054e320, tpl_424::<0x70004b2bd0, 0x60, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x54e320 sub_54e320
    Lifted::new(0x700054e370, tpl_424::<0x70004b2bd0, 0x64, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x54e370 sub_54e370
    Lifted::new(0x700054e3c0, tpl_485::<0x70004b2bd0, 0x58, 0x70, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x54e3c0 sub_54e3c0
    Lifted::new(0x700054e420, tpl_485::<0x70004b2bd0, 0x58, 0x74, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x54e420 sub_54e420
    Lifted::new(0x700054e480, tpl_465::<0x70004b2bd0, 0x79, 0x8, 0x10, 0x18> as *const (), 0), // +0x54e480 sub_54e480
    Lifted::new(0x700054e4d0, tpl_465::<0x70004b2bd0, 0x78, 0x8, 0x10, 0x18> as *const (), 0), // +0x54e4d0 sub_54e4d0
    Lifted::new(0x700054e520, tpl_465::<0x70004b2bd0, 0x7a, 0x8, 0x10, 0x18> as *const (), 0), // +0x54e520 sub_54e520
    Lifted::new(0x700054e570, tpl_465::<0x70004b2bd0, 0x7c, 0x8, 0x10, 0x18> as *const (), 0), // +0x54e570 sub_54e570
    Lifted::new(0x700054e5c0, fn_700054e5c0 as *const (), 0), // +0x54e5c0 sub_54e5c0
    Lifted::new(0x700054e610, fn_700054e610 as *const (), 0), // +0x54e610 sub_54e610
    Lifted::new(0x700054e670, tpl_486::<0x70004b2bd0, 0x68, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x54e670 sub_54e670
    Lifted::new(0x700054e6c0, tpl_486::<0x70004b2bd0, 0x6c, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x54e6c0 sub_54e6c0
    Lifted::new(0x700054e710, fn_700054e710 as *const (), 0), // +0x54e710 sub_54e710
    Lifted::new(0x700054e730, tpl_446::<0x70004b2bd0, 0x8, 0x88, 0x18, 0x28, 0x700010b4f0, 0x10> as *const (), 0), // +0x54e730 sub_54e730
    Lifted::new(0x700054e7c0, fn_700054e7c0 as *const (), 0), // +0x54e7c0 sub_54e7c0
    Lifted::new(0x700054e910, fn_700054e910 as *const (), 0), // +0x54e910 sub_54e910
    Lifted::new(0x700054f890, fn_700054f890 as *const (), 0), // +0x54f890 sub_54f890
    Lifted::new(0x700054fc50, fn_700054fc50 as *const (), 0), // +0x54fc50 sub_54fc50
    Lifted::new(0x7000550a60, fn_7000550a60 as *const (), 0), // +0x550a60 sub_550a60
    Lifted::new(0x7000550af0, tpl_430::<0x10, 0x8, 0x7000627d90, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002a06a0> as *const (), 0), // +0x550af0 sub_550af0
    Lifted::new(0x7000550bc0, fn_7000550bc0 as *const (), 0), // +0x550bc0 sub_550bc0
    Lifted::new(0x7000551030, fn_7000551030 as *const (), 0), // +0x551030 sub_551030
    Lifted::new(0x7000551270, fn_7000551270 as *const (), 0), // +0x551270 sub_551270
    Lifted::new(0x7000551530, fn_7000551530 as *const (), 0), // +0x551530 sub_551530
    Lifted::new(0x7000551870, fn_7000551870 as *const (), 0), // +0x551870 sub_551870
    Lifted::new(0x7000551900, fn_7000551900 as *const (), 0), // +0x551900 sub_551900
    Lifted::new(0x7000551950, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x18> as *const (), 0), // +0x551950 sub_551950
    Lifted::new(0x70005519a0, tpl_465::<0x70004b2bd0, 0x19, 0x8, 0x10, 0x18> as *const (), 0), // +0x5519a0 sub_5519a0
    Lifted::new(0x70005519f0, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x19> as *const (), 0), // +0x5519f0 sub_5519f0
    Lifted::new(0x7000551a40, tpl_465::<0x70004b2bd0, 0x1a, 0x8, 0x10, 0x18> as *const (), 0), // +0x551a40 sub_551a40
    Lifted::new(0x7000551a90, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x1a> as *const (), 0), // +0x551a90 sub_551a90
    Lifted::new(0x7000551ae0, tpl_465::<0x70004b2bd0, 0x1b, 0x8, 0x10, 0x18> as *const (), 0), // +0x551ae0 sub_551ae0
    Lifted::new(0x7000551b30, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x1b> as *const (), 0), // +0x551b30 sub_551b30
    Lifted::new(0x7000551b80, tpl_465::<0x70004b2bd0, 0x1c, 0x8, 0x10, 0x18> as *const (), 0), // +0x551b80 sub_551b80
    Lifted::new(0x7000551bd0, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x1c> as *const (), 0), // +0x551bd0 sub_551bd0
    Lifted::new(0x7000551c20, tpl_465::<0x70004b2bd0, 0x1d, 0x8, 0x10, 0x18> as *const (), 0), // +0x551c20 sub_551c20
    Lifted::new(0x7000551c70, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x1d> as *const (), 0), // +0x551c70 sub_551c70
    Lifted::new(0x7000551cc0, fn_7000551cc0 as *const (), 0), // +0x551cc0 sub_551cc0
    Lifted::new(0x7000551d10, fn_7000551d10 as *const (), 0), // +0x551d10 sub_551d10
    Lifted::new(0x7000551d90, fn_7000551d90 as *const (), 0), // +0x551d90 sub_551d90
    Lifted::new(0x70005525b0, fn_70005525b0 as *const (), 0), // +0x5525b0 sub_5525b0
    Lifted::new(0x7000552640, tpl_488::<0x70004b2bd0, 0x30, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x552640 sub_552640
    Lifted::new(0x70005526b0, tpl_488::<0x70004b2bd0, 0x38, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x5526b0 sub_5526b0
    Lifted::new(0x7000552720, tpl_488::<0x70004b2bd0, 0x40, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x552720 sub_552720
    Lifted::new(0x7000552790, tpl_488::<0x70004b2bd0, 0x48, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x552790 sub_552790
    Lifted::new(0x7000552800, tpl_488::<0x70004b2bd0, 0x50, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x552800 sub_552800
    Lifted::new(0x7000552870, fn_7000552870 as *const (), 0), // +0x552870 sub_552870
    Lifted::new(0x70005529c0, fn_70005529c0 as *const (), 0), // +0x5529c0 sub_5529c0
    Lifted::new(0x7000552b10, fn_7000552b10 as *const (), 0), // +0x552b10 sub_552b10
    Lifted::new(0x7000552c60, fn_7000552c60 as *const (), 0), // +0x552c60 sub_552c60
    Lifted::new(0x7000553390, fn_7000553390 as *const (), 0), // +0x553390 sub_553390
    Lifted::new(0x7000553420, tpl_424::<0x70004b2bd0, 0x28, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x553420 sub_553420
    Lifted::new(0x7000553470, fn_7000553470 as *const (), 0), // +0x553470 sub_553470
    Lifted::new(0x7000553600, fn_7000553600 as *const (), 0), // +0x553600 sub_553600
    Lifted::new(0x7000553790, fn_7000553790 as *const (), 0), // +0x553790 sub_553790
    Lifted::new(0x7000553840, fn_7000553840 as *const (), 0), // +0x553840 sub_553840
    Lifted::new(0x7000553d00, fn_7000553d00 as *const (), 0), // +0x553d00 sub_553d00
    Lifted::new(0x7000553d90, fn_7000553d90 as *const (), 0), // +0x553d90 sub_553d90
    Lifted::new(0x7000553e30, fn_7000553e30 as *const (), 0), // +0x553e30 sub_553e30
    Lifted::new(0x7000554000, fn_7000554000 as *const (), 0), // +0x554000 sub_554000
    Lifted::new(0x70005542f0, fn_70005542f0 as *const (), 0), // +0x5542f0 sub_5542f0
    Lifted::new(0x7000554380, fn_7000554380 as *const (), 0), // +0x554380 sub_554380
    Lifted::new(0x7000554460, tpl_489::<0x70004b2bd0, 0x10, 0x78, 0x8, 0x70001b9110, 0x7000697150> as *const (), 0), // +0x554460 sub_554460
    Lifted::new(0x7000554500, tpl_454::<0x70004b2bd0, 0x8, 0xe0, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x554500 sub_554500
    Lifted::new(0x70005545d0, tpl_489::<0x70004b2bd0, 0x10, 0xf0, 0x8, 0x70001b9110, 0x7000697150> as *const (), 0), // +0x5545d0 sub_5545d0
    Lifted::new(0x7000554670, fn_7000554670 as *const (), 0), // +0x554670 sub_554670
    Lifted::new(0x7000554700, fn_7000554700 as *const (), 0), // +0x554700 sub_554700
    Lifted::new(0x7000554770, fn_7000554770 as *const (), 0), // +0x554770 sub_554770
    Lifted::new(0x70005547f0, fn_70005547f0 as *const (), 0), // +0x5547f0 sub_5547f0
    Lifted::new(0x7000554980, fn_7000554980 as *const (), 0), // +0x554980 sub_554980
    Lifted::new(0x7000554a00, fn_7000554a00 as *const (), 0), // +0x554a00 sub_554a00
    Lifted::new(0x7000554a80, tpl_490::<0x70004b2bd0, 0x8, 0x70003d7a00, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x554a80 sub_554a80
    Lifted::new(0x7000554b00, tpl_490::<0x70004b2bd0, 0x8, 0x70003d7af0, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x554b00 sub_554b00
    Lifted::new(0x7000554b80, tpl_447::<0x70004b2bd0, 0x30, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x554b80 sub_554b80
    Lifted::new(0x7000554bf0, fn_7000554bf0 as *const (), 0), // +0x554bf0 sub_554bf0
    Lifted::new(0x7000554d00, fn_7000554d00 as *const (), 0), // +0x554d00 sub_554d00
    Lifted::new(0x7000554d60, fn_7000554d60 as *const (), 0), // +0x554d60 sub_554d60
    Lifted::new(0x7000554f80, fn_7000554f80 as *const (), 0), // +0x554f80 sub_554f80
    Lifted::new(0x7000555160, fn_7000555160 as *const (), 0), // +0x555160 sub_555160
    Lifted::new(0x7000555300, fn_7000555300 as *const (), 0), // +0x555300 sub_555300
    Lifted::new(0x7000555660, fn_7000555660 as *const (), 0), // +0x555660 sub_555660
    Lifted::new(0x7000555950, fn_7000555950 as *const (), 0), // +0x555950 sub_555950
    Lifted::new(0x7000555a50, fn_7000555a50 as *const (), 0), // +0x555a50 sub_555a50
    Lifted::new(0x7000555e80, fn_7000555e80 as *const (), 0), // +0x555e80 sub_555e80
    Lifted::new(0x70005560a0, fn_70005560a0 as *const (), 0), // +0x5560a0 sub_5560a0
    Lifted::new(0x70005563f0, fn_70005563f0 as *const (), 0), // +0x5563f0 sub_5563f0
    Lifted::new(0x7000556710, fn_7000556710 as *const (), 0), // +0x556710 sub_556710
    Lifted::new(0x70005569a0, fn_70005569a0 as *const (), 0), // +0x5569a0 sub_5569a0
    Lifted::new(0x7000556d00, tpl_430::<0x10, 0x8, 0x7000628720, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70003dc920> as *const (), 0), // +0x556d00 sub_556d00
    Lifted::new(0x7000556dd0, fn_7000556dd0 as *const (), 0), // +0x556dd0 sub_556dd0
    Lifted::new(0x7000557000, fn_7000557000 as *const (), 0), // +0x557000 sub_557000
    Lifted::new(0x70005572b0, fn_70005572b0 as *const (), 2), // +0x5572b0 sub_5572b0
    Lifted::new(0x7000559350, fn_7000559350 as *const (), 0), // +0x559350 sub_559350
    Lifted::new(0x70005593e0, fn_70005593e0 as *const (), 0), // +0x5593e0 sub_5593e0
    Lifted::new(0x7000559580, fn_7000559580 as *const (), 2), // +0x559580 sub_559580
    Lifted::new(0x700055a2f0, fn_700055a2f0 as *const (), 0), // +0x55a2f0 sub_55a2f0
    Lifted::new(0x700055a380, tpl_446::<0x70004b2bd0, 0x8, 0x20, 0x18, 0x28, 0x700010b4f0, 0x10> as *const (), 0), // +0x55a380 sub_55a380
    Lifted::new(0x700055a400, tpl_424::<0x70004b2bd0, 0x30, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x55a400 sub_55a400
    Lifted::new(0x700055a450, tpl_471::<0x70004b2bd0, 0x38, 0x8, 0x10, 0x28, 0x20, 0x7000697150, 0x18> as *const (), 0), // +0x55a450 sub_55a450
    Lifted::new(0x700055a530, tpl_491::<0x7000628ca0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002a3490, 0x700010b4f0> as *const (), 0), // +0x55a530 sub_55a530
    Lifted::new(0x700055a670, tpl_491::<0x7000628d00, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002a35f0, 0x700010b4f0> as *const (), 0), // +0x55a670 sub_55a670
    Lifted::new(0x700055a7b0, fn_700055a7b0 as *const (), 0), // +0x55a7b0 sub_55a7b0
    Lifted::new(0x700055ad80, fn_700055ad80 as *const (), 0), // +0x55ad80 sub_55ad80
    Lifted::new(0x700055ae10, tpl_460::<0x70004b2bd0, 0x8, 0x30, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x55ae10 sub_55ae10
    Lifted::new(0x700055ae80, fn_700055ae80 as *const (), 0), // +0x55ae80 sub_55ae80
    Lifted::new(0x700055b030, fn_700055b030 as *const (), 0), // +0x55b030 sub_55b030
    Lifted::new(0x700055b460, fn_700055b460 as *const (), 0), // +0x55b460 sub_55b460
    Lifted::new(0x700055b4f0, fn_700055b4f0 as *const (), 2), // +0x55b4f0 sub_55b4f0
    Lifted::new(0x700055b650, fn_700055b650 as *const (), 0), // +0x55b650 sub_55b650
    Lifted::new(0x700055b8b0, fn_700055b8b0 as *const (), 0), // +0x55b8b0 sub_55b8b0
    Lifted::new(0x700055b940, tpl_445::<0x70004b2bd0, 0x60, 0x8, 0xb0, 0x70005a8e60, 0x10, 0x18> as *const (), 0), // +0x55b940 sub_55b940
    Lifted::new(0x700055b9a0, fn_700055b9a0 as *const (), 0), // +0x55b9a0 sub_55b9a0
    Lifted::new(0x700055bd60, fn_700055bd60 as *const (), 0), // +0x55bd60 sub_55bd60
    Lifted::new(0x700055bfc0, fn_700055bfc0 as *const (), 0), // +0x55bfc0 sub_55bfc0
    Lifted::new(0x700055c050, tpl_465::<0x70004b2bd0, 0x58, 0x8, 0x10, 0x18> as *const (), 0), // +0x55c050 sub_55c050
    Lifted::new(0x700055c0a0, tpl_424::<0x70004b2bd0, 0x60, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x55c0a0 sub_55c0a0
    Lifted::new(0x700055c0f0, tpl_424::<0x70004b2bd0, 0x68, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x55c0f0 sub_55c0f0
    Lifted::new(0x700055c140, fn_700055c140 as *const (), 0), // +0x55c140 sub_55c140
    Lifted::new(0x700055c4c0, fn_700055c4c0 as *const (), 0), // +0x55c4c0 sub_55c4c0
    Lifted::new(0x700055c550, tpl_420::<0x70004b2bd0, 0x60, 0x8, 0x61, 0x70, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x55c550 sub_55c550
    Lifted::new(0x700055c5d0, fn_700055c5d0 as *const (), 0), // +0x55c5d0 sub_55c5d0
    Lifted::new(0x700055c8a0, fn_700055c8a0 as *const (), 0), // +0x55c8a0 sub_55c8a0
    Lifted::new(0x700055cbb0, fn_700055cbb0 as *const (), 0), // +0x55cbb0 sub_55cbb0
    Lifted::new(0x700055cc40, fn_700055cc40 as *const (), 0), // +0x55cc40 sub_55cc40
    Lifted::new(0x700055ccd0, tpl_492::<0x70004b2bd0, 0x18, 0x8, 0x160, 0x48, 0xc, 0x70, 0x70005a8aa8, 0x10> as *const (), 0), // +0x55ccd0 sub_55ccd0
    Lifted::new(0x700055cd60, fn_700055cd60 as *const (), 0), // +0x55cd60 sub_55cd60
    Lifted::new(0x700055cdf0, tpl_492::<0x70004b2bd0, 0x18, 0x8, 0x160, 0x48, 0x4, 0x70, 0x70005a8aa8, 0x10> as *const (), 0), // +0x55cdf0 sub_55cdf0
    Lifted::new(0x700055ce80, fn_700055ce80 as *const (), 0), // +0x55ce80 sub_55ce80
    Lifted::new(0x700055cf00, fn_700055cf00 as *const (), 0), // +0x55cf00 sub_55cf00
    Lifted::new(0x700055cf80, fn_700055cf80 as *const (), 0), // +0x55cf80 sub_55cf80
    Lifted::new(0x700055d4c0, fn_700055d4c0 as *const (), 0), // +0x55d4c0 sub_55d4c0
    Lifted::new(0x700055d550, fn_700055d550 as *const (), 0), // +0x55d550 sub_55d550
    Lifted::new(0x700055d5f0, fn_700055d5f0 as *const (), 0), // +0x55d5f0 sub_55d5f0
    Lifted::new(0x700055d690, tpl_430::<0x10, 0x8, 0x7000628f80, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002a6290> as *const (), 0), // +0x55d690 sub_55d690
    Lifted::new(0x700055d760, tpl_430::<0x10, 0x8, 0x7000628fe0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002a6290> as *const (), 0), // +0x55d760 sub_55d760
    Lifted::new(0x700055d830, fn_700055d830 as *const (), 0), // +0x55d830 sub_55d830
    Lifted::new(0x700055db00, tpl_493::<0x7000628f00, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002a66d0, 0x7000697150> as *const (), 0), // +0x55db00 sub_55db00
    Lifted::new(0x700055dc60, fn_700055dc60 as *const (), 0), // +0x55dc60 sub_55dc60
    Lifted::new(0x700055e2b0, fn_700055e2b0 as *const (), 0), // +0x55e2b0 sub_55e2b0
    Lifted::new(0x700055e340, fn_700055e340 as *const (), 0), // +0x55e340 sub_55e340
    Lifted::new(0x700055e430, fn_700055e430 as *const (), 0), // +0x55e430 sub_55e430
    Lifted::new(0x700055e590, fn_700055e590 as *const (), 0), // +0x55e590 sub_55e590
    Lifted::new(0x700055e7d0, fn_700055e7d0 as *const (), 0), // +0x55e7d0 sub_55e7d0
    Lifted::new(0x700055ea10, fn_700055ea10 as *const (), 0), // +0x55ea10 sub_55ea10
    Lifted::new(0x700055eb30, fn_700055eb30 as *const (), 0), // +0x55eb30 sub_55eb30
    Lifted::new(0x700055ed70, fn_700055ed70 as *const (), 0), // +0x55ed70 sub_55ed70
    Lifted::new(0x700055f280, fn_700055f280 as *const (), 0), // +0x55f280 sub_55f280
    Lifted::new(0x700055f310, fn_700055f310 as *const (), 0), // +0x55f310 sub_55f310
    Lifted::new(0x700055f3a0, fn_700055f3a0 as *const (), 0), // +0x55f3a0 sub_55f3a0
    Lifted::new(0x700055f5d0, fn_700055f5d0 as *const (), 0), // +0x55f5d0 sub_55f5d0
    Lifted::new(0x700055f620, fn_700055f620 as *const (), 0), // +0x55f620 sub_55f620
    Lifted::new(0x700055f800, fn_700055f800 as *const (), 0), // +0x55f800 sub_55f800
    Lifted::new(0x700055f920, fn_700055f920 as *const (), 0), // +0x55f920 sub_55f920
    Lifted::new(0x700055fd40, fn_700055fd40 as *const (), 0), // +0x55fd40 sub_55fd40
    Lifted::new(0x700055fdd0, fn_700055fdd0 as *const (), 0), // +0x55fdd0 sub_55fdd0
    Lifted::new(0x70005600a0, fn_70005600a0 as *const (), 0), // +0x5600a0 sub_5600a0
    Lifted::new(0x7000560130, fn_7000560130 as *const (), 0), // +0x560130 sub_560130
    Lifted::new(0x70005602c0, fn_70005602c0 as *const (), 0), // +0x5602c0 sub_5602c0
    Lifted::new(0x7000560450, fn_7000560450 as *const (), 0), // +0x560450 sub_560450
    Lifted::new(0x7000560800, fn_7000560800 as *const (), 0), // +0x560800 sub_560800
    Lifted::new(0x7000560b10, fn_7000560b10 as *const (), 0), // +0x560b10 sub_560b10
    Lifted::new(0x7000560ba0, fn_7000560ba0 as *const (), 2), // +0x560ba0 sub_560ba0
    Lifted::new(0x7000560bf0, tpl_486::<0x70004b2bd0, 0x1c, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x560bf0 sub_560bf0
    Lifted::new(0x7000560c40, tpl_486::<0x70004b2bd0, 0x20, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x560c40 sub_560c40
    Lifted::new(0x7000560c90, tpl_486::<0x70004b2bd0, 0x24, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x560c90 sub_560c90
    Lifted::new(0x7000560ce0, tpl_486::<0x70004b2bd0, 0x28, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x560ce0 sub_560ce0
    Lifted::new(0x7000560d30, fn_7000560d30 as *const (), 0), // +0x560d30 sub_560d30
    Lifted::new(0x70005611d0, fn_70005611d0 as *const (), 0), // +0x5611d0 sub_5611d0
    Lifted::new(0x7000561260, fn_7000561260 as *const (), 2), // +0x561260 sub_561260
    Lifted::new(0x70005613d0, fn_70005613d0 as *const (), 2), // +0x5613d0 sub_5613d0
    Lifted::new(0x7000561540, fn_7000561540 as *const (), 0), // +0x561540 sub_561540
    Lifted::new(0x7000561890, fn_7000561890 as *const (), 0), // +0x561890 sub_561890
    Lifted::new(0x7000561920, tpl_446::<0x70004b2bd0, 0x8, 0x30, 0x18, 0x28, 0x700010b4f0, 0x10> as *const (), 0), // +0x561920 sub_561920
    Lifted::new(0x70005619a0, tpl_494::<0x70004b2bd0, 0x18, 0x8, 0x70005a8aa8, 0x10> as *const (), 0), // +0x5619a0 sub_5619a0
    Lifted::new(0x70005619f0, tpl_424::<0x70004b2bd0, 0x1c, 0x8, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x5619f0 sub_5619f0
    Lifted::new(0x7000561a40, fn_7000561a40 as *const (), 0), // +0x561a40 sub_561a40
    Lifted::new(0x70005620c0, fn_70005620c0 as *const (), 0), // +0x5620c0 sub_5620c0
    Lifted::new(0x7000562150, tpl_495::<0x70004b2bd0, 0x58, 0x8, 0x20, 0x18, 0x29, 0x10> as *const (), 0), // +0x562150 sub_562150
    Lifted::new(0x70005621b0, tpl_495::<0x70004b2bd0, 0x58, 0x8, 0x20, 0x18, 0x28, 0x10> as *const (), 0), // +0x5621b0 sub_5621b0
    Lifted::new(0x7000562210, tpl_495::<0x70004b2bd0, 0x58, 0x8, 0x20, 0x18, 0x2a, 0x10> as *const (), 0), // +0x562210 sub_562210
    Lifted::new(0x7000562270, tpl_495::<0x70004b2bd0, 0x58, 0x8, 0x20, 0x18, 0x2c, 0x10> as *const (), 0), // +0x562270 sub_562270
    Lifted::new(0x70005622d0, fn_70005622d0 as *const (), 0), // +0x5622d0 sub_5622d0
    Lifted::new(0x7000562670, fn_7000562670 as *const (), 0), // +0x562670 sub_562670
    Lifted::new(0x7000562b30, fn_7000562b30 as *const (), 0), // +0x562b30 sub_562b30
    Lifted::new(0x7000562bc0, fn_7000562bc0 as *const (), 0), // +0x562bc0 sub_562bc0
    Lifted::new(0x7000562d50, fn_7000562d50 as *const (), 0), // +0x562d50 sub_562d50
    Lifted::new(0x7000562e00, fn_7000562e00 as *const (), 0), // +0x562e00 sub_562e00
    Lifted::new(0x7000563230, fn_7000563230 as *const (), 0), // +0x563230 sub_563230
    Lifted::new(0x70005632c0, tpl_422::<0x70005a8848, 0x70005a8e58, 0x70005a8e28, 0x70005a8910, 0x700061a940, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70005a8c68, 0x70005a8c60, 0x70004b7b60, 0x70004b5010, 0x7000697150, 0x7000613c50, 0x70006130c0, 0x70001a4530> as *const (), 2), // +0x5632c0 sub_5632c0
    Lifted::new(0x7000563720, fn_7000563720 as *const (), 0), // +0x563720 sub_563720
    Lifted::new(0x7000563aa0, fn_7000563aa0 as *const (), 0), // +0x563aa0 sub_563aa0
    Lifted::new(0x7000563b30, fn_7000563b30 as *const (), 0), // +0x563b30 sub_563b30
    Lifted::new(0x70005641d0, fn_70005641d0 as *const (), 0), // +0x5641d0 sub_5641d0
    Lifted::new(0x70005645b0, fn_70005645b0 as *const (), 0), // +0x5645b0 sub_5645b0
    Lifted::new(0x7000564800, fn_7000564800 as *const (), 0), // +0x564800 sub_564800
    Lifted::new(0x7000564890, fn_7000564890 as *const (), 0), // +0x564890 sub_564890
    Lifted::new(0x7000564b80, fn_7000564b80 as *const (), 0), // +0x564b80 sub_564b80
    Lifted::new(0x7000564d40, fn_7000564d40 as *const (), 0), // +0x564d40 sub_564d40
    Lifted::new(0x7000565010, tpl_494::<0x70004b2bd0, 0x18, 0x8, 0x70005a8958, 0x10> as *const (), 0), // +0x565010 sub_565010
    Lifted::new(0x7000565060, fn_7000565060 as *const (), 0), // +0x565060 sub_565060
    Lifted::new(0x7000565120, tpl_424::<0x70004b2bd0, 0x1c, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x565120 sub_565120
    Lifted::new(0x7000565170, fn_7000565170 as *const (), 0), // +0x565170 sub_565170
    Lifted::new(0x70005651f0, fn_70005651f0 as *const (), 0), // +0x5651f0 sub_5651f0
    Lifted::new(0x7000565550, fn_7000565550 as *const (), 0), // +0x565550 sub_565550
    Lifted::new(0x70005655e0, tpl_464::<0x70004b2bd0, 0x58, 0x18> as *const (), 0), // +0x5655e0 sub_5655e0
    Lifted::new(0x7000565610, fn_7000565610 as *const (), 0), // +0x565610 sub_565610
    Lifted::new(0x7000565680, fn_7000565680 as *const (), 0), // +0x565680 sub_565680
    Lifted::new(0x70005657a0, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x7000258e00, 0x7000697150> as *const (), 0), // +0x5657a0 sub_5657a0
    Lifted::new(0x7000565830, fn_7000565830 as *const (), 0), // +0x565830 sub_565830
    Lifted::new(0x7000565930, tpl_477::<0x70004b2bd0, 0x8, 0x28, 0x70001a4530, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x565930 sub_565930
    Lifted::new(0x7000565a00, fn_7000565a00 as *const (), 0), // +0x565a00 sub_565a00
    Lifted::new(0x7000565ad0, tpl_417::<0x70006297f4, 0x700057eb50> as *const (), 0), // +0x565ad0 sub_565ad0
    Lifted::new(0x7000565ae0, tpl_418::<0x70004b2bd0, 0x700060ddc4, 0x70006297f4, 0x700057ec20> as *const (), 0), // +0x565ae0 sub_565ae0
    Lifted::new(0x7000565b10, tpl_417::<0x7000629750, 0x700057eb50> as *const (), 0), // +0x565b10 sub_565b10
    Lifted::new(0x7000565b20, tpl_418::<0x70004b2bd0, 0x700060e0e4, 0x7000629750, 0x700057ec20> as *const (), 0), // +0x565b20 sub_565b20
    Lifted::new(0x7000565b50, tpl_417::<0x7000629758, 0x700057eb50> as *const (), 0), // +0x565b50 sub_565b50
    Lifted::new(0x7000565b60, tpl_418::<0x70004b2bd0, 0x700060a3a0, 0x7000629758, 0x700057ec20> as *const (), 0), // +0x565b60 sub_565b60
    Lifted::new(0x7000565b90, fn_7000565b90 as *const (), 0), // +0x565b90 sub_565b90
    Lifted::new(0x7000565ee0, fn_7000565ee0 as *const (), 2), // +0x565ee0 sub_565ee0
    Lifted::new(0x7000566440, fn_7000566440 as *const (), 0), // +0x566440 sub_566440
    Lifted::new(0x7000566690, fn_7000566690 as *const (), 2), // +0x566690 sub_566690
    Lifted::new(0x7000567400, fn_7000567400 as *const (), 0), // +0x567400 sub_567400
    Lifted::new(0x7000567490, tpl_447::<0x70004b4d00, 0x58, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x567490 sub_567490
    Lifted::new(0x7000567500, tpl_447::<0x70004b4d00, 0x60, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x567500 sub_567500
    Lifted::new(0x7000567570, tpl_447::<0x70004b4d00, 0x90, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x567570 sub_567570
    Lifted::new(0x70005675e0, tpl_447::<0x70004b4d00, 0x78, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x5675e0 sub_5675e0
    Lifted::new(0x7000567650, tpl_447::<0x70004b4d00, 0x70, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x567650 sub_567650
    Lifted::new(0x70005676c0, tpl_447::<0x70004b4d00, 0x68, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x5676c0 sub_5676c0
    Lifted::new(0x7000567730, fn_7000567730 as *const (), 0), // +0x567730 sub_567730
    Lifted::new(0x70005677a0, tpl_447::<0x70004b4d00, 0x80, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x5677a0 sub_5677a0
    Lifted::new(0x7000567810, fn_7000567810 as *const (), 0), // +0x567810 sub_567810
    Lifted::new(0x7000567890, tpl_447::<0x70004b4d00, 0x88, 0x8, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x567890 sub_567890
    Lifted::new(0x7000567900, tpl_490::<0x70004b4d00, 0x8, 0x70002aba10, 0xb0, 0x28, 0x10, 0x18> as *const (), 0), // +0x567900 sub_567900
    Lifted::new(0x7000567980, tpl_496::<0x70004b4d00, 0x8, 0x58, 0x1c8, 0x90, 0x7000496870, 0x2da4, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x567980 sub_567980
    Lifted::new(0x7000567a20, tpl_496::<0x70004b4d00, 0x8, 0x58, 0x1c8, 0x90, 0x7000496870, 0x2da0, 0x70005a8aa8, 0x10, 0x18> as *const (), 0), // +0x567a20 sub_567a20
    Lifted::new(0x7000567ac0, fn_7000567ac0 as *const (), 0), // +0x567ac0 sub_567ac0
    Lifted::new(0x7000567b10, tpl_497::<0x70004b4d00, 0x58, 0x8, 0x2bc8, 0x88, 0x70005a8aa8, 0x10, 0x18> as *const (), 2), // +0x567b10 sub_567b10
    Lifted::new(0x7000567b90, tpl_497::<0x70004b4d00, 0x58, 0x8, 0x2bc8, 0x80, 0x70005a8aa8, 0x10, 0x18> as *const (), 2), // +0x567b90 sub_567b90
    Lifted::new(0x7000567c10, fn_7000567c10 as *const (), 2), // +0x567c10 sub_567c10
    Lifted::new(0x7000567c60, tpl_417::<0x7000629898, 0x700057eb50> as *const (), 0), // +0x567c60 sub_567c60
    Lifted::new(0x7000567c70, tpl_418::<0x70004b4d00, 0x700060df10, 0x7000629898, 0x700057ec20> as *const (), 0), // +0x567c70 sub_567c70
    Lifted::new(0x7000567ca0, tpl_417::<0x70006298b0, 0x700057eb50> as *const (), 0), // +0x567ca0 sub_567ca0
    Lifted::new(0x7000567cb0, tpl_418::<0x70004b4d00, 0x700060df80, 0x70006298b0, 0x700057ec20> as *const (), 0), // +0x567cb0 sub_567cb0
    Lifted::new(0x7000567ce0, tpl_417::<0x70006298c8, 0x700057eb50> as *const (), 0), // +0x567ce0 sub_567ce0
    Lifted::new(0x7000567cf0, tpl_418::<0x70004b4d00, 0x700060e090, 0x70006298c8, 0x700057ec20> as *const (), 0), // +0x567cf0 sub_567cf0
    Lifted::new(0x7000567d20, tpl_418::<0x70004b4d00, 0x70005fb69c, 0x7000626fc0, 0x700057ec20> as *const (), 0), // +0x567d20 sub_567d20
    Lifted::new(0x7000567d50, tpl_417::<0x7000629a90, 0x700057eb50> as *const (), 0), // +0x567d50 sub_567d50
    Lifted::new(0x7000567d60, tpl_418::<0x70004b4d00, 0x700060e0d0, 0x7000629a90, 0x700057ec20> as *const (), 0), // +0x567d60 sub_567d60
    Lifted::new(0x7000567d90, tpl_418::<0x70004b4d00, 0x70005fb574, 0x7000623fd0, 0x700057ec20> as *const (), 0), // +0x567d90 sub_567d90
    Lifted::new(0x7000567dc0, tpl_418::<0x70004b4d00, 0x70005fb58c, 0x7000624078, 0x700057ec20> as *const (), 0), // +0x567dc0 sub_567dc0
    Lifted::new(0x7000567df0, tpl_418::<0x70004b4d00, 0x70005fb584, 0x7000624080, 0x700057ec20> as *const (), 0), // +0x567df0 sub_567df0
    Lifted::new(0x7000567e20, tpl_418::<0x70004b4d00, 0x70005fb5a0, 0x7000624088, 0x700057ec20> as *const (), 0), // +0x567e20 sub_567e20
    Lifted::new(0x7000567e50, tpl_418::<0x70004b4d00, 0x70005f8384, 0x7000624098, 0x700057ec20> as *const (), 0), // +0x567e50 sub_567e50
    Lifted::new(0x7000567e80, tpl_418::<0x70004b4d00, 0x70005fb594, 0x7000624048, 0x700057ec20> as *const (), 0), // +0x567e80 sub_567e80
    Lifted::new(0x7000567eb0, tpl_418::<0x70004b4d00, 0x70005fb5d8, 0x7000624050, 0x700057ec20> as *const (), 0), // +0x567eb0 sub_567eb0
    Lifted::new(0x7000567ee0, tpl_418::<0x70004b4d00, 0x70005fb5c8, 0x7000624060, 0x700057ec20> as *const (), 0), // +0x567ee0 sub_567ee0
    Lifted::new(0x7000567f10, tpl_418::<0x70004b4d00, 0x70005fb5e8, 0x7000624070, 0x700057ec20> as *const (), 0), // +0x567f10 sub_567f10
    Lifted::new(0x7000567f40, tpl_418::<0x70004b4d00, 0x70005fb5e0, 0x70006240dc, 0x700057ec20> as *const (), 0), // +0x567f40 sub_567f40
    Lifted::new(0x7000567f70, tpl_418::<0x70004b4d00, 0x70005fb600, 0x70006240e8, 0x700057ec20> as *const (), 0), // +0x567f70 sub_567f70
    Lifted::new(0x7000567fa0, tpl_418::<0x70004b4d00, 0x70005fb5f0, 0x70006240f8, 0x700057ec20> as *const (), 0), // +0x567fa0 sub_567fa0
    Lifted::new(0x7000567fd0, tpl_418::<0x70004b4d00, 0x70005fb620, 0x7000624108, 0x700057ec20> as *const (), 0), // +0x567fd0 sub_567fd0
    Lifted::new(0x7000568000, tpl_418::<0x70004b4d00, 0x70005fb610, 0x70006240a0, 0x700057ec20> as *const (), 0), // +0x568000 sub_568000
    Lifted::new(0x7000568030, tpl_418::<0x70004b4d00, 0x70005fb640, 0x70006240b0, 0x700057ec20> as *const (), 0), // +0x568030 sub_568030
    Lifted::new(0x7000568060, tpl_418::<0x70004b4d00, 0x70005fb630, 0x70006240c0, 0x700057ec20> as *const (), 0), // +0x568060 sub_568060
    Lifted::new(0x7000568090, tpl_418::<0x70004b4d00, 0x70005fb658, 0x70006240d0, 0x700057ec20> as *const (), 0), // +0x568090 sub_568090
    Lifted::new(0x70005680c0, tpl_418::<0x70004b4d00, 0x70005f8064, 0x7000624210, 0x700057ec20> as *const (), 0), // +0x5680c0 sub_5680c0
    Lifted::new(0x70005680f0, tpl_418::<0x70004b4d00, 0x70005f83b0, 0x7000624218, 0x700057ec20> as *const (), 0), // +0x5680f0 sub_5680f0
    Lifted::new(0x7000568120, tpl_418::<0x70004b4d00, 0x70005fb674, 0x7000624228, 0x700057ec20> as *const (), 0), // +0x568120 sub_568120
    Lifted::new(0x7000568150, tpl_418::<0x70004b4d00, 0x70005fb690, 0x7000624230, 0x700057ec20> as *const (), 0), // +0x568150 sub_568150
    Lifted::new(0x7000568180, tpl_418::<0x70004b4d00, 0x70005fb680, 0x7000624118, 0x700057ec20> as *const (), 0), // +0x568180 sub_568180
    Lifted::new(0x70005681b0, tpl_418::<0x70004b4d00, 0x70005fb6a8, 0x7000624128, 0x700057ec20> as *const (), 0), // +0x5681b0 sub_5681b0
    Lifted::new(0x70005681e0, tpl_467::<0x7000629ab0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b4d00, 0x70004b9560, 0x70006266a8, 0x70005a8890, 0x70004b5280, 0x70004b5010, 0x70003ce3f0> as *const (), 0), // +0x5681e0 sub_5681e0
    Lifted::new(0x7000568450, fn_7000568450 as *const (), 0), // +0x568450 sub_568450
    Lifted::new(0x70005685e0, tpl_498::<0x7000629b80, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b4d00, 0x70005a8e58, 0x70005a8b18, 0x70002ab3d0> as *const (), 0), // +0x5685e0 sub_5685e0
    Lifted::new(0x70005686f0, tpl_498::<0x7000629910, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b4d00, 0x70005a8e58, 0x70005a8b18, 0x70002ab3d0> as *const (), 0), // +0x5686f0 sub_5686f0
    Lifted::new(0x7000568800, tpl_468::<0x7000629970, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b4d00, 0x70005a8870, 0x7000626770, 0x70004b5120, 0x70003cea90> as *const (), 0), // +0x568800 sub_568800
    Lifted::new(0x70005689b0, fn_70005689b0 as *const (), 0), // +0x5689b0 sub_5689b0
    Lifted::new(0x7000568bc0, tpl_449::<0x10, 0x8, 0x7000629a30, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b4d00, 0xb0, 0x18> as *const (), 0), // +0x568bc0 sub_568bc0
    Lifted::new(0x7000568cb0, tpl_467::<0x7000629d00, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b4d00, 0x70004b9560, 0x70006266a8, 0x70005a8890, 0x70004b5280, 0x70004b5010, 0x70003ce7d0> as *const (), 0), // +0x568cb0 sub_568cb0
    Lifted::new(0x7000568f20, fn_7000568f20 as *const (), 2), // +0x568f20 sub_568f20
    Lifted::new(0x70005691f0, fn_70005691f0 as *const (), 2), // +0x5691f0 sub_5691f0
    Lifted::new(0x70005694d0, fn_70005694d0 as *const (), 2), // +0x5694d0 sub_5694d0
    Lifted::new(0x7000569750, fn_7000569750 as *const (), 2), // +0x569750 sub_569750
    Lifted::new(0x7000569910, fn_7000569910 as *const (), 2), // +0x569910 sub_569910
    Lifted::new(0x7000569b20, fn_7000569b20 as *const (), 0), // +0x569b20 sub_569b20
    Lifted::new(0x700056ccb0, fn_700056ccb0 as *const (), 0), // +0x56ccb0 sub_56ccb0
    Lifted::new(0x700056cd40, tpl_464::<0x70004b2bd0, 0x3c, 0x18> as *const (), 0), // +0x56cd40 sub_56cd40
    Lifted::new(0x700056cd70, tpl_424::<0x70004b2bd0, 0x38, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x56cd70 sub_56cd70
    Lifted::new(0x700056cdc0, fn_700056cdc0 as *const (), 0), // +0x56cdc0 sub_56cdc0
    Lifted::new(0x700056ce90, tpl_465::<0x70004b2bd0, 0x41, 0x8, 0x10, 0x18> as *const (), 0), // +0x56ce90 sub_56ce90
    Lifted::new(0x700056cee0, tpl_487::<0x70004b2bd0, 0x10, 0x8, 0x70005a8890, 0x41> as *const (), 0), // +0x56cee0 sub_56cee0
    Lifted::new(0x700056cf30, fn_700056cf30 as *const (), 0), // +0x56cf30 sub_56cf30
    Lifted::new(0x700056d080, fn_700056d080 as *const (), 0), // +0x56d080 sub_56d080
    Lifted::new(0x700056d0c0, fn_700056d0c0 as *const (), 0), // +0x56d0c0 sub_56d0c0
    Lifted::new(0x700056d220, fn_700056d220 as *const (), 0), // +0x56d220 sub_56d220
    Lifted::new(0x700056d2e0, fn_700056d2e0 as *const (), 0), // +0x56d2e0 sub_56d2e0
    Lifted::new(0x700056d410, fn_700056d410 as *const (), 0), // +0x56d410 sub_56d410
    Lifted::new(0x700056d510, tpl_417::<0x700062a140, 0x700057eb50> as *const (), 0), // +0x56d510 sub_56d510
    Lifted::new(0x700056d520, tpl_418::<0x70004b2bd0, 0x700060dfe0, 0x700062a140, 0x700057ec20> as *const (), 0), // +0x56d520 sub_56d520
    Lifted::new(0x700056d550, tpl_417::<0x700062a158, 0x700057eb50> as *const (), 0), // +0x56d550 sub_56d550
    Lifted::new(0x700056d560, tpl_418::<0x70004b2bd0, 0x70005f80c8, 0x700062a158, 0x700057ec20> as *const (), 0), // +0x56d560 sub_56d560
    Lifted::new(0x700056d590, tpl_417::<0x700062a030, 0x700057eb50> as *const (), 0), // +0x56d590 sub_56d590
    Lifted::new(0x700056d5a0, tpl_418::<0x70004b2bd0, 0x700060df78, 0x700062a030, 0x700057ec20> as *const (), 0), // +0x56d5a0 sub_56d5a0
    Lifted::new(0x700056d5d0, tpl_417::<0x700062a040, 0x700057eb50> as *const (), 0), // +0x56d5d0 sub_56d5d0
    Lifted::new(0x700056d5e0, tpl_418::<0x70004b2bd0, 0x700060df70, 0x700062a040, 0x700057ec20> as *const (), 0), // +0x56d5e0 sub_56d5e0
    Lifted::new(0x700056d610, tpl_430::<0x10, 0x8, 0x700062a050, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002ae3d0> as *const (), 0), // +0x56d610 sub_56d610
    Lifted::new(0x700056d6e0, tpl_493::<0x700062a0b0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002ae4d0, 0x7000697150> as *const (), 0), // +0x56d6e0 sub_56d6e0
    Lifted::new(0x700056d840, fn_700056d840 as *const (), 0), // +0x56d840 sub_56d840
    Lifted::new(0x700056da60, fn_700056da60 as *const (), 0), // +0x56da60 sub_56da60
    Lifted::new(0x700056df60, fn_700056df60 as *const (), 0), // +0x56df60 sub_56df60
    Lifted::new(0x700056e080, fn_700056e080 as *const (), 0), // +0x56e080 sub_56e080
    Lifted::new(0x700056e1d0, fn_700056e1d0 as *const (), 0), // +0x56e1d0 sub_56e1d0
    Lifted::new(0x700056e380, fn_700056e380 as *const (), 0), // +0x56e380 sub_56e380
    Lifted::new(0x700056e5e0, fn_700056e5e0 as *const (), 0), // +0x56e5e0 sub_56e5e0
    Lifted::new(0x700056e860, fn_700056e860 as *const (), 0), // +0x56e860 sub_56e860
    Lifted::new(0x700056ea40, fn_700056ea40 as *const (), 2), // +0x56ea40 sub_56ea40
    Lifted::new(0x70005702c0, fn_70005702c0 as *const (), 0), // +0x5702c0 sub_5702c0
    Lifted::new(0x7000570350, fn_7000570350 as *const (), 0), // +0x570350 sub_570350
    Lifted::new(0x70005704e0, fn_70005704e0 as *const (), 2), // +0x5704e0 sub_5704e0
    Lifted::new(0x70005705b0, fn_70005705b0 as *const (), 2), // +0x5705b0 sub_5705b0
    Lifted::new(0x70005706b0, fn_70005706b0 as *const (), 2), // +0x5706b0 sub_5706b0
    Lifted::new(0x7000570b30, fn_7000570b30 as *const (), 0), // +0x570b30 sub_570b30
    Lifted::new(0x7000570f20, fn_7000570f20 as *const (), 0), // +0x570f20 sub_570f20
    Lifted::new(0x7000571310, fn_7000571310 as *const (), 2), // +0x571310 sub_571310
    Lifted::new(0x70005714d0, fn_70005714d0 as *const (), 0), // +0x5714d0 sub_5714d0
    Lifted::new(0x7000571660, tpl_499::<0x70006974b8, 0x700062a578, 0x70005f84a0, 0x7000130980, 0x70005a8858, 0x70005a8988, 0x70005a89a0, 0x7000461040, 0x7000570b30, 0x7000461680, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60, 0x7000697150> as *const (), 0), // +0x571660 sub_571660
    Lifted::new(0x7000571840, fn_7000571840 as *const (), 0), // +0x571840 sub_571840
    Lifted::new(0x70005719f0, tpl_499::<0x70006974b8, 0x700062a578, 0x70005f84a0, 0x7000130980, 0x70005a8858, 0x70005a8988, 0x70005a89a0, 0x7000458e20, 0x7000570b30, 0x7000459240, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60, 0x7000697150> as *const (), 0), // +0x5719f0 sub_5719f0
    Lifted::new(0x7000571bd0, fn_7000571bd0 as *const (), 0), // +0x571bd0 sub_571bd0
    Lifted::new(0x7000571d80, fn_7000571d80 as *const (), 0), // +0x571d80 sub_571d80
    Lifted::new(0x7000571f30, fn_7000571f30 as *const (), 0), // +0x571f30 sub_571f30
    Lifted::new(0x70005720e0, fn_70005720e0 as *const (), 2), // +0x5720e0 sub_5720e0
    Lifted::new(0x70005722f0, fn_70005722f0 as *const (), 2), // +0x5722f0 sub_5722f0
    Lifted::new(0x70005724d0, tpl_499::<0x70006974b8, 0x700062a578, 0x70005f84a0, 0x7000130980, 0x70005a8858, 0x70005a8988, 0x70005a89a0, 0x70004622d0, 0x7000570b30, 0x7000462990, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60, 0x7000697150> as *const (), 0), // +0x5724d0 sub_5724d0
    Lifted::new(0x70005726b0, fn_70005726b0 as *const (), 0), // +0x5726b0 sub_5726b0
    Lifted::new(0x7000572880, tpl_81::<0x700068f138, 0x70003cdfe0, 0x700010b4f0, 0x7000697150, 0x7000421650, 0x70003c6cc0, 0x7000573f90, 0x70003c7f40, 0x7000429920, 0x700039fa20, 0x70003c8240, 0x7000428b80, 0x700029ce60, 0x700048beb0, 0x7000132560, 0x7000572a23> as *const (), 0), // +0x572880 sub_572880
    Lifted::new(0x7000572a30, fn_7000572a30 as *const (), 0), // +0x572a30 sub_572a30
    Lifted::new(0x7000572bb0, fn_7000572bb0 as *const (), 0), // +0x572bb0 sub_572bb0
    Lifted::new(0x7000572d50, tpl_500::<0x70005a8240, 0x700061e190, 0x70002445b0, 0x70005a81b0, 0x7000607cc8, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x572d50 sub_572d50
    Lifted::new(0x7000572ee0, tpl_500::<0x70005a8240, 0x700061eaa0, 0x70002445b0, 0x70005a81b0, 0x7000607cc8, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x572ee0 sub_572ee0
    Lifted::new(0x7000573070, tpl_500::<0x70005a8240, 0x7000620960, 0x70002445b0, 0x70005a81b0, 0x7000607cc8, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x573070 sub_573070
    Lifted::new(0x7000573200, tpl_500::<0x70005a8240, 0x70006216b0, 0x70002445b0, 0x70005a81b0, 0x7000607cc8, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x573200 sub_573200
    Lifted::new(0x7000573390, fn_7000573390 as *const (), 0), // +0x573390 sub_573390
    Lifted::new(0x7000573490, fn_7000573490 as *const (), 2), // +0x573490 sub_573490
    Lifted::new(0x7000573610, fn_7000573610 as *const (), 1), // +0x573610 sub_573610
    Lifted::new(0x7000573620, fn_7000573620 as *const (), 2), // +0x573620 sub_573620
    Lifted::new(0x7000573980, fn_7000573980 as *const (), 2), // +0x573980 sub_573980
    Lifted::new(0x7000573cf0, fn_7000573cf0 as *const (), 2), // +0x573cf0 sub_573cf0
    Lifted::new(0x7000573f90, fn_7000573f90 as *const (), 0), // +0x573f90 sub_573f90
    Lifted::new(0x70005741a0, fn_70005741a0 as *const (), 0), // +0x5741a0 sub_5741a0
    Lifted::new(0x70005742d0, fn_70005742d0 as *const (), 0), // +0x5742d0 sub_5742d0
    Lifted::new(0x7000574580, fn_7000574580 as *const (), 2), // +0x574580 sub_574580
    Lifted::new(0x7000574940, fn_7000574940 as *const (), 0), // +0x574940 sub_574940
    Lifted::new(0x7000574c00, fn_7000574c00 as *const (), 0), // +0x574c00 sub_574c00
    Lifted::new(0x7000574c30, fn_7000574c30 as *const (), 0), // +0x574c30 sub_574c30
    Lifted::new(0x7000574d10, fn_7000574d10 as *const (), 0), // +0x574d10 sub_574d10
    Lifted::new(0x7000574e40, fn_7000574e40 as *const (), 0), // +0x574e40 sub_574e40
    Lifted::new(0x7000574ea0, tpl_501::<0x30> as *const (), 0), // +0x574ea0 sub_574ea0
    Lifted::new(0x7000574ef0, fn_7000574ef0 as *const (), 2), // +0x574ef0 sub_574ef0
    Lifted::new(0x7000575110, fn_7000575110 as *const (), 0), // +0x575110 sub_575110
    Lifted::new(0x70005751e0, fn_70005751e0 as *const (), 0), // +0x5751e0 sub_5751e0
    Lifted::new(0x7000575250, tpl_25::<0x7000694974, 0x7000699830, 0x70005a4174, 0x70005a7af0, 0x70006997d0, 0x70006997e8, 0x7000699800, 0x7000699818, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x575250 sub_575250
    Lifted::new(0x70005752d0, fn_70005752d0 as *const (), 0), // +0x5752d0 sub_5752d0
    Lifted::new(0x7000575360, fn_7000575360 as *const (), 0), // +0x575360 sub_575360
    Lifted::new(0x70005753e0, tpl_25::<0x7000694974, 0x70006997c0, 0x70005a4174, 0x70005a7b00, 0x7000699740, 0x7000699760, 0x7000699780, 0x70006997a0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x5753e0 sub_5753e0
    Lifted::new(0x7000575460, fn_7000575460 as *const (), 0), // +0x575460 sub_575460
    Lifted::new(0x70005754a0, fn_70005754a0 as *const (), 2), // +0x5754a0 sub_5754a0
    Lifted::new(0x7000575810, fn_7000575810 as *const (), 0), // +0x575810 sub_575810
    Lifted::new(0x7000575d80, fn_7000575d80 as *const (), 2), // +0x575d80 sub_575d80
    Lifted::new(0x7000576390, fn_7000576390 as *const (), 0), // +0x576390 sub_576390
    Lifted::new(0x7000576690, fn_7000576690 as *const (), 0), // +0x576690 sub_576690
    Lifted::new(0x70005768a0, fn_70005768a0 as *const (), 0), // +0x5768a0 sub_5768a0
    Lifted::new(0x70005768b0, tpl_9::<0x70006940d0> as *const (), 1), // +0x5768b0 sub_5768b0
    Lifted::new(0x70005768c0, fn_70005768c0 as *const (), 0), // +0x5768c0 sub_5768c0
    Lifted::new(0x7000576a70, fn_7000576a70 as *const (), 3), // +0x576a70 sub_576a70
    Lifted::new(0x7000576a90, fn_7000576a90 as *const (), 0), // +0x576a90 sub_576a90
    Lifted::new(0x7000576ad0, fn_7000576ad0 as *const (), 0), // +0x576ad0 sub_576ad0
    Lifted::new(0x7000576b90, fn_7000576b90 as *const (), 0), // +0x576b90 sub_576b90
    Lifted::new(0x7000576c30, fn_7000576c30 as *const (), 0), // +0x576c30 sub_576c30
    Lifted::new(0x7000576d10, fn_7000576d10 as *const (), 2), // +0x576d10 sub_576d10
    Lifted::new(0x7000576fa0, fn_7000576fa0 as *const (), 0), // +0x576fa0 sub_576fa0
    Lifted::new(0x7000577050, fn_7000577050 as *const (), 2), // +0x577050 sub_577050
    Lifted::new(0x70005771c0, fn_70005771c0 as *const (), 0), // +0x5771c0 sub_5771c0
    Lifted::new(0x70005773a0, fn_70005773a0 as *const (), 0), // +0x5773a0 sub_5773a0
    Lifted::new(0x7000577540, fn_7000577540 as *const (), 0), // +0x577540 sub_577540
    Lifted::new(0x70005775e0, fn_70005775e0 as *const (), 2), // +0x5775e0 sub_5775e0
    Lifted::new(0x7000577730, fn_7000577730 as *const (), 0), // +0x577730 sub_577730
    Lifted::new(0x7000577770, fn_7000577770 as *const (), 0), // +0x577770 sub_577770
    Lifted::new(0x7000577810, fn_7000577810 as *const (), 0), // +0x577810 sub_577810
    Lifted::new(0x7000577880, tpl_160::<0x700062e090> as *const (), 1), // +0x577880 ??0ITimeZoneProvider@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x7000577890, tpl_161::<0xfffffffffffffff8, 0x700062e090, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x577890 sub_577890
    Lifted::new(0x7000577920, tpl_207::<0x8, 0x18> as *const (), 0), // +0x577920 sub_577920
    Lifted::new(0x7000577930, tpl_304::<0x700048c2c0> as *const (), 0), // +0x577930 sub_577930
    Lifted::new(0x7000577960, fn_7000577960 as *const (), 0), // +0x577960 sub_577960
    Lifted::new(0x70005779a0, fn_70005779a0 as *const (), 0), // +0x5779a0 sub_5779a0
    Lifted::new(0x70005779c0, fn_70005779c0 as *const (), 0), // +0x5779c0 sub_5779c0
    Lifted::new(0x70005779e0, fn_70005779e0 as *const (), 0), // +0x5779e0 sub_5779e0
    Lifted::new(0x7000577a10, fn_7000577a10 as *const (), 0), // +0x577a10 sub_577a10
    Lifted::new(0x7000577b90, fn_7000577b90 as *const (), 0), // +0x577b90 sub_577b90
    Lifted::new(0x7000577d30, fn_7000577d30 as *const (), 0), // +0x577d30 sub_577d30
    Lifted::new(0x7000578110, fn_7000578110 as *const (), 0), // +0x578110 sub_578110
    Lifted::new(0x7000578330, fn_7000578330 as *const (), 0), // +0x578330 sub_578330
    Lifted::new(0x7000578620, fn_7000578620 as *const (), 0), // +0x578620 sub_578620
    Lifted::new(0x7000578c90, fn_7000578c90 as *const (), 0), // +0x578c90 sub_578c90
    Lifted::new(0x7000578fb0, fn_7000578fb0 as *const (), 0), // +0x578fb0 sub_578fb0
    Lifted::new(0x70005792d0, fn_70005792d0 as *const (), 0), // +0x5792d0 sub_5792d0
    Lifted::new(0x7000579430, fn_7000579430 as *const (), 2), // +0x579430 sub_579430
    Lifted::new(0x70005795d0, fn_70005795d0 as *const (), 0), // +0x5795d0 sub_5795d0
    Lifted::new(0x7000579860, fn_7000579860 as *const (), 2), // +0x579860 sub_579860
    Lifted::new(0x7000579ae0, fn_7000579ae0 as *const (), 0), // +0x579ae0 sub_579ae0
    Lifted::new(0x7000579be0, fn_7000579be0 as *const (), 2), // +0x579be0 sub_579be0
    Lifted::new(0x7000579ce0, fn_7000579ce0 as *const (), 0), // +0x579ce0 sub_579ce0
    Lifted::new(0x7000579f50, fn_7000579f50 as *const (), 0), // +0x579f50 sub_579f50
    Lifted::new(0x7000579fb0, fn_7000579fb0 as *const (), 0), // +0x579fb0 sub_579fb0
    Lifted::new(0x700057a010, fn_700057a010 as *const (), 2), // +0x57a010 sub_57a010
    Lifted::new(0x700057aa40, fn_700057aa40 as *const (), 0), // +0x57aa40 sub_57aa40
    Lifted::new(0x700057aa80, fn_700057aa80 as *const (), 2), // +0x57aa80 sub_57aa80
    Lifted::new(0x700057ae00, fn_700057ae00 as *const (), 0), // +0x57ae00 sub_57ae00
    Lifted::new(0x700057aeb0, fn_700057aeb0 as *const (), 0), // +0x57aeb0 sub_57aeb0
    Lifted::new(0x700057af30, fn_700057af30 as *const (), 0), // +0x57af30 sub_57af30
    Lifted::new(0x700057b230, tpl_135::<0x700057aeb0, 0x10, 0x8, 0x18> as *const (), 0), // +0x57b230 sub_57b230
    Lifted::new(0x700057b2a0, fn_700057b2a0 as *const (), 0), // +0x57b2a0 sub_57b2a0
    Lifted::new(0x700057b320, fn_700057b320 as *const (), 0), // +0x57b320 sub_57b320
    Lifted::new(0x700057b430, fn_700057b430 as *const (), 0), // +0x57b430 sub_57b430
    Lifted::new(0x700057b5a0, fn_700057b5a0 as *const (), 0), // +0x57b5a0 sub_57b5a0
    Lifted::new(0x700057b720, fn_700057b720 as *const (), 0), // +0x57b720 sub_57b720
    Lifted::new(0x700057b8f0, tpl_244::<0x18, 0x8, 0x700057bd80> as *const (), 0), // +0x57b8f0 sub_57b8f0
    Lifted::new(0x700057b990, fn_700057b990 as *const (), 0), // +0x57b990 sub_57b990
    Lifted::new(0x700057ba80, fn_700057ba80 as *const (), 0), // +0x57ba80 sub_57ba80
    Lifted::new(0x700057bb60, fn_700057bb60 as *const (), 0), // +0x57bb60 sub_57bb60
    Lifted::new(0x700057bbf0, fn_700057bbf0 as *const (), 0), // +0x57bbf0 sub_57bbf0
    Lifted::new(0x700057bd80, tpl_166::<0x700057cc40, 0x700057b8f0> as *const (), 2), // +0x57bd80 sub_57bd80
    Lifted::new(0x700057bfb0, fn_700057bfb0 as *const (), 0), // +0x57bfb0 sub_57bfb0
    Lifted::new(0x700057cb40, tpl_9::<0x7000694210> as *const (), 1), // +0x57cb40 sub_57cb40
    Lifted::new(0x700057cb50, fn_700057cb50 as *const (), 2), // +0x57cb50 sub_57cb50
    Lifted::new(0x700057cb70, tpl_178::<0x700062b120, 0x8> as *const (), 3), // +0x57cb70 sub_57cb70
    Lifted::new(0x700057cb90, tpl_9::<0x70006941d0> as *const (), 1), // +0x57cb90 sub_57cb90
    Lifted::new(0x700057cba0, fn_700057cba0 as *const (), 0), // +0x57cba0 sub_57cba0
    Lifted::new(0x700057cc20, tpl_119::<0x700062b150, 0x8> as *const (), 1), // +0x57cc20 sub_57cc20
    Lifted::new(0x700057cc40, fn_700057cc40 as *const (), 2), // +0x57cc40 sub_57cc40
    Lifted::new(0x700057cef0, tpl_9::<0x7000694190> as *const (), 1), // +0x57cef0 sub_57cef0
    Lifted::new(0x700057cf00, fn_700057cf00 as *const (), 0), // +0x57cf00 sub_57cf00
    Lifted::new(0x700057cf20, tpl_178::<0x700062b090, 0x8> as *const (), 3), // +0x57cf20 sub_57cf20
    Lifted::new(0x700057cf40, tpl_9::<0x7000694150> as *const (), 1), // +0x57cf40 sub_57cf40
    Lifted::new(0x700057cf50, fn_700057cf50 as *const (), 0), // +0x57cf50 sub_57cf50
    Lifted::new(0x700057cf70, tpl_178::<0x700062b0c0, 0x8> as *const (), 3), // +0x57cf70 sub_57cf70
    Lifted::new(0x700057cf90, tpl_9::<0x7000694110> as *const (), 1), // +0x57cf90 sub_57cf90
    Lifted::new(0x700057cfa0, tpl_178::<0x700062b0f0, 0x8> as *const (), 3), // +0x57cfa0 sub_57cfa0
    Lifted::new(0x700057cfc0, fn_700057cfc0 as *const (), 0), // +0x57cfc0 sub_57cfc0
    Lifted::new(0x700057cfe0, fn_700057cfe0 as *const (), 0), // +0x57cfe0 sub_57cfe0
    Lifted::new(0x700057d000, fn_700057d000 as *const (), 0), // +0x57d000 sub_57d000
    Lifted::new(0x700057d050, tpl_502::<0x7000697140, 0x10> as *const (), 0), // +0x57d050 sub_57d050
    Lifted::new(0x700057d070, fn_700057d070 as *const (), 0), // +0x57d070 sub_57d070
    Lifted::new(0x700057d0e0, fn_700057d0e0 as *const (), 0), // +0x57d0e0 sub_57d0e0
    Lifted::new(0x700057d100, tpl_502::<0x7000697140, 0x20> as *const (), 0), // +0x57d100 sub_57d100
    Lifted::new(0x700057d120, fn_700057d120 as *const (), 2), // +0x57d120 sub_57d120
    Lifted::new(0x700057d2d0, fn_700057d2d0 as *const (), 2), // +0x57d2d0 sub_57d2d0
    Lifted::new(0x700057d4d0, fn_700057d4d0 as *const (), 0), // +0x57d4d0 sub_57d4d0
    Lifted::new(0x700057d4f0, fn_700057d4f0 as *const (), 0), // +0x57d4f0 sub_57d4f0
    Lifted::new(0x700057d540, tpl_501::<0x10> as *const (), 0), // +0x57d540 sub_57d540
    Lifted::new(0x700057d590, fn_700057d590 as *const (), 1), // +0x57d590 sub_57d590
    Lifted::new(0x700057d610, fn_700057d610 as *const (), 0), // +0x57d610 sub_57d610
    Lifted::new(0x700057d700, fn_700057d700 as *const (), 2), // +0x57d700 sub_57d700
    Lifted::new(0x700057d800, fn_700057d800 as *const (), 1), // +0x57d800 sub_57d800
    Lifted::new(0x700057d820, fn_700057d820 as *const (), 0), // +0x57d820 sub_57d820
    Lifted::new(0x700057da90, fn_700057da90 as *const (), 0), // +0x57da90 sub_57da90
    Lifted::new(0x700057db10, fn_700057db10 as *const (), 2), // +0x57db10 sub_57db10
    Lifted::new(0x700057db40, fn_700057db40 as *const (), 2), // +0x57db40 sub_57db40
    Lifted::new(0x700057db60, fn_700057db60 as *const (), 0), // +0x57db60 sub_57db60
    Lifted::new(0x700057dc50, tpl_67::<0x18, 0x8, 0x20, 0x700057e090> as *const (), 0), // +0x57dc50 sub_57dc50
    Lifted::new(0x700057dce0, fn_700057dce0 as *const (), 0), // +0x57dce0 sub_57dce0
    Lifted::new(0x700057dd10, fn_700057dd10 as *const (), 0), // +0x57dd10 sub_57dd10
    Lifted::new(0x700057dd20, fn_700057dd20 as *const (), 0), // +0x57dd20 sub_57dd20
    Lifted::new(0x700057dd60, fn_700057dd60 as *const (), 0), // +0x57dd60 sub_57dd60
    Lifted::new(0x700057de30, tpl_86::<0x700062b518, 0x700048c2c0> as *const (), 0), // +0x57de30 sub_57de30
    Lifted::new(0x700057de60, fn_700057de60 as *const (), 2), // +0x57de60 sub_57de60
    Lifted::new(0x700057df60, fn_700057df60 as *const (), 0), // +0x57df60 sub_57df60
    Lifted::new(0x700057e030, tpl_342::<0x70006041e0, 0x700048c2c0> as *const (), 0), // +0x57e030 sub_57e030
    Lifted::new(0x700057e060, tpl_70::<0x70006041e0, 0x700048c2c0> as *const (), 0), // +0x57e060 sub_57e060
    Lifted::new(0x700057e090, fn_700057e090 as *const (), 2), // +0x57e090 sub_57e090
    Lifted::new(0x700057e380, fn_700057e380 as *const (), 2), // +0x57e380 sub_57e380
    Lifted::new(0x700057e630, fn_700057e630 as *const (), 0), // +0x57e630 sub_57e630
    Lifted::new(0x700057e6d0, fn_700057e6d0 as *const (), 0), // +0x57e6d0 sub_57e6d0
    Lifted::new(0x700057e7f0, fn_700057e7f0 as *const (), 0), // +0x57e7f0 sub_57e7f0
    Lifted::new(0x700057e8d0, fn_700057e8d0 as *const (), 0), // +0x57e8d0 sub_57e8d0
    Lifted::new(0x700057e900, fn_700057e900 as *const (), 0), // +0x57e900 sub_57e900
    Lifted::new(0x700057e950, fn_700057e950 as *const (), 0), // +0x57e950 sub_57e950
    Lifted::new(0x700057ea80, fn_700057ea80 as *const (), 0), // +0x57ea80 sub_57ea80
    Lifted::new(0x700057eb50, fn_700057eb50 as *const (), 0), // +0x57eb50 sub_57eb50
    Lifted::new(0x700057ec20, fn_700057ec20 as *const (), 0), // +0x57ec20 sub_57ec20
    Lifted::new(0x700057eea0, fn_700057eea0 as *const (), 0), // +0x57eea0 sub_57eea0
    Lifted::new(0x700057ef10, tpl_187::<0x18, 0x19, 0x28> as *const (), 1), // +0x57ef10 sub_57ef10
    Lifted::new(0x700057ef20, tpl_234::<0x18> as *const (), 1), // +0x57ef20 sub_57ef20
    Lifted::new(0x700057ef30, fn_700057ef30 as *const (), 0), // +0x57ef30 sub_57ef30
    Lifted::new(0x700057ef50, fn_700057ef50 as *const (), 0), // +0x57ef50 sub_57ef50
    Lifted::new(0x700057f160, fn_700057f160 as *const (), 0), // +0x57f160 sub_57f160
    Lifted::new(0x700057f1f0, fn_700057f1f0 as *const (), 0), // +0x57f1f0 sub_57f1f0
    Lifted::new(0x700057f360, fn_700057f360 as *const (), 2), // +0x57f360 sub_57f360
    Lifted::new(0x700057f450, fn_700057f450 as *const (), 0), // +0x57f450 sub_57f450
    Lifted::new(0x700057f500, fn_700057f500 as *const (), 0), // +0x57f500 sub_57f500
    Lifted::new(0x700057f580, fn_700057f580 as *const (), 0), // +0x57f580 sub_57f580
    Lifted::new(0x700057f5c0, fn_700057f5c0 as *const (), 0), // +0x57f5c0 sub_57f5c0
    Lifted::new(0x700057f5f0, fn_700057f5f0 as *const (), 0), // +0x57f5f0 sub_57f5f0
    Lifted::new(0x700057f640, fn_700057f640 as *const (), 1), // +0x57f640 sub_57f640
    Lifted::new(0x700057f650, fn_700057f650 as *const (), 0), // +0x57f650 sub_57f650
    Lifted::new(0x700057f7b0, fn_700057f7b0 as *const (), 0), // +0x57f7b0 sub_57f7b0
    Lifted::new(0x700057f7d0, fn_700057f7d0 as *const (), 0), // +0x57f7d0 sub_57f7d0
    Lifted::new(0x700057f8b0, fn_700057f8b0 as *const (), 1), // +0x57f8b0 sub_57f8b0
    Lifted::new(0x700057f8c0, fn_700057f8c0 as *const (), 0), // +0x57f8c0 sub_57f8c0
    Lifted::new(0x700057f980, fn_700057f980 as *const (), 0), // +0x57f980 sub_57f980
    Lifted::new(0x700057fb20, fn_700057fb20 as *const (), 2), // +0x57fb20 sub_57fb20
    Lifted::new(0x700057fc40, fn_700057fc40 as *const (), 2), // +0x57fc40 sub_57fc40
    Lifted::new(0x700057fd30, fn_700057fd30 as *const (), 0), // +0x57fd30 sub_57fd30
    Lifted::new(0x700057fe30, fn_700057fe30 as *const (), 2), // +0x57fe30 sub_57fe30
    Lifted::new(0x70005800c0, fn_70005800c0 as *const (), 2), // +0x5800c0 sub_5800c0
    Lifted::new(0x7000580200, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000580290> as *const (), 2), // +0x580200 sub_580200
    Lifted::new(0x7000580290, fn_7000580290 as *const (), 0), // +0x580290 sub_580290
    Lifted::new(0x7000580310, fn_7000580310 as *const (), 2), // +0x580310 sub_580310
    Lifted::new(0x7000580440, fn_7000580440 as *const (), 2), // +0x580440 sub_580440
    Lifted::new(0x7000580530, fn_7000580530 as *const (), 0), // +0x580530 sub_580530
    Lifted::new(0x70005806b0, fn_70005806b0 as *const (), 1), // +0x5806b0 sub_5806b0
    Lifted::new(0x70005806c0, tpl_372::<0x18> as *const (), 1), // +0x5806c0 sub_5806c0
    Lifted::new(0x70005806d0, tpl_372::<0x20> as *const (), 1), // +0x5806d0 sub_5806d0
    Lifted::new(0x70005806e0, fn_70005806e0 as *const (), 0), // +0x5806e0 sub_5806e0
    Lifted::new(0x7000580830, fn_7000580830 as *const (), 0), // +0x580830 sub_580830
    Lifted::new(0x70005808b0, fn_70005808b0 as *const (), 0), // +0x5808b0 sub_5808b0
    Lifted::new(0x7000580950, fn_7000580950 as *const (), 3), // +0x580950 sub_580950
    Lifted::new(0x7000580a30, fn_7000580a30 as *const (), 0), // +0x580a30 sub_580a30
    Lifted::new(0x7000580ad0, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x7000588dc0> as *const (), 0), // +0x580ad0 sub_580ad0
    Lifted::new(0x7000580b40, fn_7000580b40 as *const (), 0), // +0x580b40 sub_580b40
    Lifted::new(0x7000580cc0, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x7000588ff0> as *const (), 0), // +0x580cc0 sub_580cc0
    Lifted::new(0x7000580d30, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x70005895d0> as *const (), 0), // +0x580d30 sub_580d30
    Lifted::new(0x7000580da0, fn_7000580da0 as *const (), 0), // +0x580da0 sub_580da0
    Lifted::new(0x7000580ed0, fn_7000580ed0 as *const (), 0), // +0x580ed0 sub_580ed0
    Lifted::new(0x7000580f90, fn_7000580f90 as *const (), 0), // +0x580f90 sub_580f90
    Lifted::new(0x7000581020, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x7000589b40> as *const (), 0), // +0x581020 sub_581020
    Lifted::new(0x7000581090, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058a030> as *const (), 0), // +0x581090 sub_581090
    Lifted::new(0x7000581100, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058a1e0> as *const (), 0), // +0x581100 sub_581100
    Lifted::new(0x7000581170, fn_7000581170 as *const (), 0), // +0x581170 sub_581170
    Lifted::new(0x7000581220, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058a440> as *const (), 0), // +0x581220 sub_581220
    Lifted::new(0x7000581290, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058a600> as *const (), 0), // +0x581290 sub_581290
    Lifted::new(0x7000581300, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058aa80> as *const (), 0), // +0x581300 sub_581300
    Lifted::new(0x7000581370, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058ae10> as *const (), 0), // +0x581370 sub_581370
    Lifted::new(0x70005813e0, tpl_503::<0xffffffffffffffff, 0x7, 0x70005a8c68, 0xb, 0x70005a8c60, 0x700058bc00> as *const (), 0), // +0x5813e0 sub_5813e0
    Lifted::new(0x7000581450, fn_7000581450 as *const (), 2), // +0x581450 sub_581450
    Lifted::new(0x7000581570, fn_7000581570 as *const (), 0), // +0x581570 sub_581570
    Lifted::new(0x70005816d0, fn_70005816d0 as *const (), 0), // +0x5816d0 sub_5816d0
    Lifted::new(0x7000581a50, fn_7000581a50 as *const (), 0), // +0x581a50 sub_581a50
    Lifted::new(0x7000581a90, fn_7000581a90 as *const (), 0), // +0x581a90 sub_581a90
    Lifted::new(0x7000581f30, fn_7000581f30 as *const (), 0), // +0x581f30 sub_581f30
    Lifted::new(0x7000582c70, fn_7000582c70 as *const (), 0), // +0x582c70 sub_582c70
    Lifted::new(0x7000582d70, fn_7000582d70 as *const (), 0), // +0x582d70 sub_582d70
    Lifted::new(0x7000582da0, fn_7000582da0 as *const (), 0), // +0x582da0 sub_582da0
    Lifted::new(0x7000582dd0, fn_7000582dd0 as *const (), 0), // +0x582dd0 sub_582dd0
    Lifted::new(0x7000582e10, fn_7000582e10 as *const (), 0), // +0x582e10 sub_582e10
    Lifted::new(0x7000582e50, fn_7000582e50 as *const (), 0), // +0x582e50 sub_582e50
    Lifted::new(0x7000582e90, tpl_504::<0x48, 0x70005a8aa8, 0x7000582c70> as *const (), 0), // +0x582e90 sub_582e90
    Lifted::new(0x7000582ed0, tpl_504::<0x48, 0x70005a8958, 0x7000582c70> as *const (), 0), // +0x582ed0 sub_582ed0
    Lifted::new(0x7000582f10, fn_7000582f10 as *const (), 2), // +0x582f10 sub_582f10
    Lifted::new(0x7000582f90, fn_7000582f90 as *const (), 2), // +0x582f90 sub_582f90
    Lifted::new(0x7000583030, fn_7000583030 as *const (), 2), // +0x583030 sub_583030
    Lifted::new(0x7000583070, fn_7000583070 as *const (), 2), // +0x583070 sub_583070
    Lifted::new(0x70005830b0, fn_70005830b0 as *const (), 0), // +0x5830b0 sub_5830b0
    Lifted::new(0x70005830f0, fn_70005830f0 as *const (), 0), // +0x5830f0 sub_5830f0
    Lifted::new(0x7000583130, fn_7000583130 as *const (), 2), // +0x583130 sub_583130
    Lifted::new(0x7000583170, fn_7000583170 as *const (), 2), // +0x583170 sub_583170
    Lifted::new(0x70005831b0, fn_70005831b0 as *const (), 0), // +0x5831b0 sub_5831b0
    Lifted::new(0x7000583230, fn_7000583230 as *const (), 0), // +0x583230 sub_583230
    Lifted::new(0x70005832c0, fn_70005832c0 as *const (), 0), // +0x5832c0 sub_5832c0
    Lifted::new(0x7000583500, fn_7000583500 as *const (), 0), // +0x583500 sub_583500
    Lifted::new(0x70005835a0, tpl_505::<0x48, 0x70005a88a0, 0x70005a89a0, 0x70005a4d14, 0x70005a8768, 0x7000582c70> as *const (), 0), // +0x5835a0 sub_5835a0
    Lifted::new(0x7000583630, tpl_505::<0x48, 0x70005a88a0, 0x70005a89a0, 0x70005a4d14, 0x70005a88a8, 0x7000582c70> as *const (), 0), // +0x583630 sub_583630
    Lifted::new(0x70005836c0, fn_70005836c0 as *const (), 2), // +0x5836c0 sub_5836c0
    Lifted::new(0x70005837d0, fn_70005837d0 as *const (), 2), // +0x5837d0 sub_5837d0
    Lifted::new(0x7000583960, tpl_220::<0x150> as *const (), 0), // +0x583960 sub_583960
    Lifted::new(0x7000583970, fn_7000583970 as *const (), 2), // +0x583970 sub_583970
    Lifted::new(0x7000583af0, fn_7000583af0 as *const (), 0), // +0x583af0 sub_583af0
    Lifted::new(0x7000583b00, fn_7000583b00 as *const (), 0), // +0x583b00 sub_583b00
    Lifted::new(0x7000584000, fn_7000584000 as *const (), 0), // +0x584000 sub_584000
    Lifted::new(0x7000584080, fn_7000584080 as *const (), 0), // +0x584080 sub_584080
    Lifted::new(0x70005841f0, fn_70005841f0 as *const (), 2), // +0x5841f0 sub_5841f0
    Lifted::new(0x70005843d0, fn_70005843d0 as *const (), 0), // +0x5843d0 sub_5843d0
    Lifted::new(0x7000584450, fn_7000584450 as *const (), 0), // +0x584450 sub_584450
    Lifted::new(0x7000584530, fn_7000584530 as *const (), 0), // +0x584530 sub_584530
    Lifted::new(0x7000584610, tpl_506::<0x7000584670> as *const (), 0), // +0x584610 sub_584610
    Lifted::new(0x7000584640, tpl_506::<0x7000584820> as *const (), 0), // +0x584640 sub_584640
    Lifted::new(0x7000584670, tpl_507::<0x700058c200, 0x7000697150, 0x70005a8850, 0x70005a8db8, 0x700058b6c0, 0x700062c370, 0x700058b830, 0x70005a8e60, 0x70005a8db0, 0x70005a8b30, 0x70005a8da8, 0x700062c388, 0x70005a8d90, 0x70005a8998> as *const (), 0), // +0x584670 sub_584670
    Lifted::new(0x7000584820, tpl_507::<0x700058c200, 0x7000697150, 0x70005a8850, 0x70005a8db8, 0x700058b6c0, 0x700062c370, 0x700058b830, 0x70005a8e60, 0x70005a8db0, 0x70005a8b30, 0x70005a8da8, 0x700062c388, 0x70005a8da0, 0x70005a8b18> as *const (), 0), // +0x584820 sub_584820
    Lifted::new(0x70005849d0, tpl_508::<0x700058c200, 0x7000697150, 0x70005a8850, 0x70005a8db8, 0x700058b6c0, 0x700062c370, 0x700058b830, 0x70005a8e60, 0x70005a8db0, 0x70005a8b30, 0x70005a8da8, 0x700062c388, 0x70005a8d90, 0x70005a8990, 0x7000580950> as *const (), 2), // +0x5849d0 sub_5849d0
    Lifted::new(0x7000584b90, tpl_508::<0x700058c200, 0x7000697150, 0x70005a8850, 0x70005a8db8, 0x700058b6c0, 0x700062c370, 0x700058b830, 0x70005a8e60, 0x70005a8db0, 0x70005a8b30, 0x70005a8da8, 0x700062c388, 0x70005a8da0, 0x70005a8990, 0x7000580950> as *const (), 2), // +0x584b90 sub_584b90
    Lifted::new(0x7000584d50, tpl_509::<0x7000584670> as *const (), 0), // +0x584d50 sub_584d50
    Lifted::new(0x7000584d80, tpl_509::<0x7000584820> as *const (), 0), // +0x584d80 sub_584d80
    Lifted::new(0x7000584db0, tpl_510::<0x7000584670> as *const (), 0), // +0x584db0 sub_584db0
    Lifted::new(0x7000584de0, tpl_510::<0x7000584820> as *const (), 0), // +0x584de0 sub_584de0
    Lifted::new(0x7000584e10, fn_7000584e10 as *const (), 2), // +0x584e10 sub_584e10
    Lifted::new(0x7000584e40, fn_7000584e40 as *const (), 2), // +0x584e40 sub_584e40
    Lifted::new(0x7000585040, fn_7000585040 as *const (), 0), // +0x585040 sub_585040
    Lifted::new(0x7000585200, fn_7000585200 as *const (), 0), // +0x585200 sub_585200
    Lifted::new(0x7000585490, fn_7000585490 as *const (), 0), // +0x585490 sub_585490
    Lifted::new(0x7000585720, fn_7000585720 as *const (), 0), // +0x585720 sub_585720
    Lifted::new(0x70005859b0, fn_70005859b0 as *const (), 0), // +0x5859b0 sub_5859b0
    Lifted::new(0x7000585b40, fn_7000585b40 as *const (), 0), // +0x585b40 sub_585b40
    Lifted::new(0x7000585e30, fn_7000585e30 as *const (), 0), // +0x585e30 sub_585e30
    Lifted::new(0x7000586010, fn_7000586010 as *const (), 0), // +0x586010 sub_586010
    Lifted::new(0x7000586080, fn_7000586080 as *const (), 0), // +0x586080 sub_586080
    Lifted::new(0x7000586190, fn_7000586190 as *const (), 2), // +0x586190 sub_586190
    Lifted::new(0x70005862d0, fn_70005862d0 as *const (), 0), // +0x5862d0 sub_5862d0
    Lifted::new(0x70005864b0, fn_70005864b0 as *const (), 0), // +0x5864b0 sub_5864b0
    Lifted::new(0x7000586580, fn_7000586580 as *const (), 0), // +0x586580 sub_586580
    Lifted::new(0x70005866b0, fn_70005866b0 as *const (), 0), // +0x5866b0 sub_5866b0
    Lifted::new(0x70005868e0, fn_70005868e0 as *const (), 0), // +0x5868e0 sub_5868e0
    Lifted::new(0x70005869e0, fn_70005869e0 as *const (), 0), // +0x5869e0 sub_5869e0
    Lifted::new(0x7000586aa0, fn_7000586aa0 as *const (), 0), // +0x586aa0 sub_586aa0
    Lifted::new(0x7000586c80, fn_7000586c80 as *const (), 0), // +0x586c80 sub_586c80
    Lifted::new(0x7000586dc0, fn_7000586dc0 as *const (), 0), // +0x586dc0 sub_586dc0
    Lifted::new(0x7000586f30, tpl_144::<0x200> as *const (), 1), // +0x586f30 sub_586f30
    Lifted::new(0x7000586f40, tpl_143::<0x200> as *const (), 1), // +0x586f40 sub_586f40
    Lifted::new(0x7000586f50, fn_7000586f50 as *const (), 2), // +0x586f50 sub_586f50
    Lifted::new(0x7000587090, fn_7000587090 as *const (), 0), // +0x587090 sub_587090
    Lifted::new(0x7000587230, fn_7000587230 as *const (), 0), // +0x587230 sub_587230
    Lifted::new(0x7000587290, fn_7000587290 as *const (), 0), // +0x587290 sub_587290
    Lifted::new(0x70005875d0, fn_70005875d0 as *const (), 0), // +0x5875d0 sub_5875d0
    Lifted::new(0x7000587970, fn_7000587970 as *const (), 3), // +0x587970 sub_587970
    Lifted::new(0x7000587980, fn_7000587980 as *const (), 2), // +0x587980 sub_587980
    Lifted::new(0x7000587cf0, fn_7000587cf0 as *const (), 0), // +0x587cf0 sub_587cf0
    Lifted::new(0x7000587fe0, fn_7000587fe0 as *const (), 0), // +0x587fe0 sub_587fe0
    Lifted::new(0x70005883d0, fn_70005883d0 as *const (), 0), // +0x5883d0 sub_5883d0
    Lifted::new(0x70005885f0, fn_70005885f0 as *const (), 0), // +0x5885f0 sub_5885f0
    Lifted::new(0x7000588a00, fn_7000588a00 as *const (), 0), // +0x588a00 sub_588a00
    Lifted::new(0x7000588dc0, fn_7000588dc0 as *const (), 0), // +0x588dc0 sub_588dc0
    Lifted::new(0x7000588ff0, fn_7000588ff0 as *const (), 2), // +0x588ff0 sub_588ff0
    Lifted::new(0x70005895d0, fn_70005895d0 as *const (), 0), // +0x5895d0 sub_5895d0
    Lifted::new(0x7000589b40, fn_7000589b40 as *const (), 0), // +0x589b40 sub_589b40
    Lifted::new(0x700058a030, fn_700058a030 as *const (), 0), // +0x58a030 sub_58a030
    Lifted::new(0x700058a1e0, fn_700058a1e0 as *const (), 0), // +0x58a1e0 sub_58a1e0
    Lifted::new(0x700058a440, fn_700058a440 as *const (), 0), // +0x58a440 sub_58a440
    Lifted::new(0x700058a600, fn_700058a600 as *const (), 0), // +0x58a600 sub_58a600
    Lifted::new(0x700058aa80, fn_700058aa80 as *const (), 0), // +0x58aa80 sub_58aa80
    Lifted::new(0x700058ae10, fn_700058ae10 as *const (), 0), // +0x58ae10 sub_58ae10
    Lifted::new(0x700058b0f0, fn_700058b0f0 as *const (), 0), // +0x58b0f0 sub_58b0f0
    Lifted::new(0x700058b1a0, fn_700058b1a0 as *const (), 0), // +0x58b1a0 sub_58b1a0
    Lifted::new(0x700058b1c0, fn_700058b1c0 as *const (), 0), // +0x58b1c0 sub_58b1c0
    Lifted::new(0x700058b280, fn_700058b280 as *const (), 0), // +0x58b280 sub_58b280
    Lifted::new(0x700058b440, fn_700058b440 as *const (), 0), // +0x58b440 sub_58b440
    Lifted::new(0x700058b540, fn_700058b540 as *const (), 0), // +0x58b540 sub_58b540
    Lifted::new(0x700058b6c0, fn_700058b6c0 as *const (), 0), // +0x58b6c0 sub_58b6c0
    Lifted::new(0x700058b830, fn_700058b830 as *const (), 2), // +0x58b830 sub_58b830
    Lifted::new(0x700058bb80, fn_700058bb80 as *const (), 1), // +0x58bb80 sub_58bb80
    Lifted::new(0x700058bc00, fn_700058bc00 as *const (), 0), // +0x58bc00 sub_58bc00
    Lifted::new(0x700058be80, fn_700058be80 as *const (), 0), // +0x58be80 sub_58be80
    Lifted::new(0x700058bf70, fn_700058bf70 as *const (), 0), // +0x58bf70 sub_58bf70
    Lifted::new(0x700058c030, fn_700058c030 as *const (), 0), // +0x58c030 sub_58c030
    Lifted::new(0x700058c0a0, tpl_90::<0x8, 0x19, 0x10, 0x700058cf20, 0x20> as *const (), 0), // +0x58c0a0 sub_58c0a0
    Lifted::new(0x700058c120, tpl_17::<0x8, 0x700058c950> as *const (), 0), // +0x58c120 sub_58c120
    Lifted::new(0x700058c190, fn_700058c190 as *const (), 0), // +0x58c190 sub_58c190
    Lifted::new(0x700058c200, fn_700058c200 as *const (), 1), // +0x58c200 sub_58c200
    Lifted::new(0x700058c270, fn_700058c270 as *const (), 0), // +0x58c270 sub_58c270
    Lifted::new(0x700058c2d0, fn_700058c2d0 as *const (), 0), // +0x58c2d0 sub_58c2d0
    Lifted::new(0x700058c380, fn_700058c380 as *const (), 0), // +0x58c380 sub_58c380
    Lifted::new(0x700058c480, fn_700058c480 as *const (), 0), // +0x58c480 sub_58c480
    Lifted::new(0x700058c500, fn_700058c500 as *const (), 2), // +0x58c500 sub_58c500
    Lifted::new(0x700058c750, fn_700058c750 as *const (), 2), // +0x58c750 sub_58c750
    Lifted::new(0x700058c870, fn_700058c870 as *const (), 2), // +0x58c870 sub_58c870
    Lifted::new(0x700058c950, fn_700058c950 as *const (), 2), // +0x58c950 sub_58c950
    Lifted::new(0x700058ca40, fn_700058ca40 as *const (), 2), // +0x58ca40 sub_58ca40
    Lifted::new(0x700058cb30, fn_700058cb30 as *const (), 0), // +0x58cb30 sub_58cb30
    Lifted::new(0x700058cbc0, fn_700058cbc0 as *const (), 2), // +0x58cbc0 sub_58cbc0
    Lifted::new(0x700058ccb0, fn_700058ccb0 as *const (), 2), // +0x58ccb0 sub_58ccb0
    Lifted::new(0x700058cd70, fn_700058cd70 as *const (), 0), // +0x58cd70 sub_58cd70
    Lifted::new(0x700058ce10, fn_700058ce10 as *const (), 2), // +0x58ce10 sub_58ce10
    Lifted::new(0x700058cf20, fn_700058cf20 as *const (), 0), // +0x58cf20 sub_58cf20
    Lifted::new(0x700058cf90, tpl_25::<0x7000694974, 0x700069b9e0, 0x70005a4174, 0x70005a7cb0, 0x700069b9e8, 0x700069ba10, 0x700069ba38, 0x700069ba60, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x58cf90 sub_58cf90
    Lifted::new(0x700058d010, tpl_25::<0x7000694974, 0x700069ba88, 0x70005a4174, 0x70005a7ca0, 0x700069ba90, 0x700069bab8, 0x700069bae0, 0x700069bb08, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x58d010 sub_58d010
    Lifted::new(0x700058d090, fn_700058d090 as *const (), 0), // +0x58d090 sub_58d090
    Lifted::new(0x700058d180, fn_700058d180 as *const (), 0), // +0x58d180 sub_58d180
    Lifted::new(0x700058d210, fn_700058d210 as *const (), 1), // +0x58d210 sub_58d210
    Lifted::new(0x700058d2e0, fn_700058d2e0 as *const (), 1), // +0x58d2e0 sub_58d2e0
    Lifted::new(0x700058d3b0, fn_700058d3b0 as *const (), 1), // +0x58d3b0 sub_58d3b0
    Lifted::new(0x700058d4a0, tpl_226::<0x70005a8240, 0x700062b888, 0x70002445b0, 0x700062b878, 0x70005a8230> as *const (), 0), // +0x58d4a0 sub_58d4a0
    Lifted::new(0x700058d5f0, fn_700058d5f0 as *const (), 2), // +0x58d5f0 sub_58d5f0
    Lifted::new(0x700058d6f0, fn_700058d6f0 as *const (), 0), // +0x58d6f0 sub_58d6f0
    Lifted::new(0x700058d870, tpl_73::<0x7000007030, 0x700012fa40, 0x700058e1a0> as *const (), 0), // +0x58d870 sub_58d870
    Lifted::new(0x700058d960, fn_700058d960 as *const (), 0), // +0x58d960 sub_58d960
    Lifted::new(0x700058dac0, tpl_229::<0x70005a8240, 0x700062c440, 0x70002445b0, 0x700062c3d0, 0x70005a8230> as *const (), 0), // +0x58dac0 sub_58dac0
    Lifted::new(0x700058dc10, tpl_73::<0x7000007030, 0x700012fa40, 0x700058e490> as *const (), 0), // +0x58dc10 sub_58dc10
    Lifted::new(0x700058dd00, fn_700058dd00 as *const (), 2), // +0x58dd00 sub_58dd00
    Lifted::new(0x700058dd80, fn_700058dd80 as *const (), 0), // +0x58dd80 sub_58dd80
    Lifted::new(0x700058dea0, fn_700058dea0 as *const (), 2), // +0x58dea0 sub_58dea0
    Lifted::new(0x700058e1a0, fn_700058e1a0 as *const (), 2), // +0x58e1a0 sub_58e1a0
    Lifted::new(0x700058e490, fn_700058e490 as *const (), 2), // +0x58e490 sub_58e490
    Lifted::new(0x700058e790, fn_700058e790 as *const (), 2), // +0x58e790 sub_58e790
    Lifted::new(0x700058f1a0, fn_700058f1a0 as *const (), 2), // +0x58f1a0 sub_58f1a0
    Lifted::new(0x700058fd90, fn_700058fd90 as *const (), 0), // +0x58fd90 sub_58fd90
    Lifted::new(0x700058fe10, fn_700058fe10 as *const (), 0), // +0x58fe10 sub_58fe10
    Lifted::new(0x700058fe90, tpl_74::<0x10, 0x2, 0x700058ff90> as *const (), 0), // +0x58fe90 sub_58fe90
    Lifted::new(0x700058fec0, fn_700058fec0 as *const (), 1), // +0x58fec0 sub_58fec0
    Lifted::new(0x700058ff60, tpl_74::<0x10, 0x2, 0x70005905e0> as *const (), 0), // +0x58ff60 sub_58ff60
    Lifted::new(0x700058ff90, fn_700058ff90 as *const (), 2), // +0x58ff90 sub_58ff90
    Lifted::new(0x7000590250, fn_7000590250 as *const (), 0), // +0x590250 sub_590250
    Lifted::new(0x7000590430, fn_7000590430 as *const (), 0), // +0x590430 sub_590430
    Lifted::new(0x70005905e0, fn_70005905e0 as *const (), 2), // +0x5905e0 sub_5905e0
    Lifted::new(0x70005908d0, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x7000590c40> as *const (), 0), // +0x5908d0 sub_5908d0
    Lifted::new(0x70005909a0, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x7000590ef0> as *const (), 0), // +0x5909a0 sub_5909a0
    Lifted::new(0x7000590a70, tpl_333::<0x10, 0x8, 0x18> as *const (), 3), // +0x590a70 sub_590a70
    Lifted::new(0x7000590b30, fn_7000590b30 as *const (), 3), // +0x590b30 sub_590b30
    Lifted::new(0x7000590c40, fn_7000590c40 as *const (), 2), // +0x590c40 sub_590c40
    Lifted::new(0x7000590ef0, fn_7000590ef0 as *const (), 2), // +0x590ef0 sub_590ef0
    Lifted::new(0x70005911a4, tpl_511::<0x7000580830> as *const (), 0), // +0x5911a4 sub_5911a4
    Lifted::new(0x70005911b0, tpl_512::<0x7000580830> as *const (), 0), // +0x5911b0 sub_5911b0
    Lifted::new(0x70005911bc, tpl_511::<0x7000581a50> as *const (), 0), // +0x5911bc sub_5911bc
    Lifted::new(0x70005911c8, tpl_512::<0x7000581a50> as *const (), 0), // +0x5911c8 sub_5911c8
    Lifted::new(0x70005911e0, tpl_78::<0x700062ca58, 0x700048c2c0> as *const (), 0), // +0x5911e0 sub_5911e0
    Lifted::new(0x7000591210, fn_7000591210 as *const (), 0), // +0x591210 sub_591210
    Lifted::new(0x7000591280, fn_7000591280 as *const (), 2), // +0x591280 sub_591280
    Lifted::new(0x70005913f0, fn_70005913f0 as *const (), 2), // +0x5913f0 sub_5913f0
    Lifted::new(0x7000591560, fn_7000591560 as *const (), 0), // +0x591560 sub_591560
    Lifted::new(0x70005915e0, fn_70005915e0 as *const (), 2), // +0x5915e0 sub_5915e0
    Lifted::new(0x70005916e0, tpl_513::<0x6e8, 0x7000591280> as *const (), 0), // +0x5916e0 sub_5916e0
    Lifted::new(0x7000591700, fn_7000591700 as *const (), 2), // +0x591700 sub_591700
    Lifted::new(0x70005917d0, tpl_513::<0x6e8, 0x70005913f0> as *const (), 0), // +0x5917d0 sub_5917d0
    Lifted::new(0x70005917f0, fn_70005917f0 as *const (), 2), // +0x5917f0 sub_5917f0
    Lifted::new(0x70005918c0, fn_70005918c0 as *const (), 0), // +0x5918c0 sub_5918c0
    Lifted::new(0x70005918e0, fn_70005918e0 as *const (), 0), // +0x5918e0 sub_5918e0
    Lifted::new(0x7000591900, fn_7000591900 as *const (), 0), // +0x591900 sub_591900
    Lifted::new(0x7000591920, fn_7000591920 as *const (), 0), // +0x591920 sub_591920
    Lifted::new(0x7000591950, fn_7000591950 as *const (), 2), // +0x591950 sub_591950
    Lifted::new(0x7000591b90, fn_7000591b90 as *const (), 2), // +0x591b90 sub_591b90
    Lifted::new(0x70005920b0, fn_70005920b0 as *const (), 2), // +0x5920b0 sub_5920b0
    Lifted::new(0x7000592270, fn_7000592270 as *const (), 2), // +0x592270 sub_592270
    Lifted::new(0x7000592360, fn_7000592360 as *const (), 0), // +0x592360 sub_592360
    Lifted::new(0x7000592490, fn_7000592490 as *const (), 0), // +0x592490 sub_592490
    Lifted::new(0x7000592680, fn_7000592680 as *const (), 0), // +0x592680 sub_592680
    Lifted::new(0x7000592710, fn_7000592710 as *const (), 0), // +0x592710 sub_592710
    Lifted::new(0x70005927b0, fn_70005927b0 as *const (), 0), // +0x5927b0 sub_5927b0
    Lifted::new(0x7000592830, fn_7000592830 as *const (), 0), // +0x592830 sub_592830
    Lifted::new(0x70005928d0, fn_70005928d0 as *const (), 2), // +0x5928d0 sub_5928d0
    Lifted::new(0x70005929c0, fn_70005929c0 as *const (), 2), // +0x5929c0 sub_5929c0
    Lifted::new(0x7000592ab0, tpl_25::<0x7000694974, 0x700069bb30, 0x70005a4174, 0x70005a7d50, 0x700069bb40, 0x700069bb58, 0x700069bb70, 0x700069bb88, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x592ab0 sub_592ab0
    Lifted::new(0x7000592b30, fn_7000592b30 as *const (), 0), // +0x592b30 sub_592b30
    Lifted::new(0x7000592c90, fn_7000592c90 as *const (), 0), // +0x592c90 sub_592c90
    Lifted::new(0x7000592df0, fn_7000592df0 as *const (), 0), // +0x592df0 sub_592df0
    Lifted::new(0x7000592f40, fn_7000592f40 as *const (), 0), // +0x592f40 sub_592f40
    Lifted::new(0x7000593090, tpl_244::<0x18, 0x8, 0x7000593140> as *const (), 0), // +0x593090 sub_593090
    Lifted::new(0x7000593130, fn_7000593130 as *const (), 0), // +0x593130 sub_593130
    Lifted::new(0x7000593140, fn_7000593140 as *const (), 2), // +0x593140 sub_593140
    Lifted::new(0x70005933c0, fn_70005933c0 as *const (), 2), // +0x5933c0 sub_5933c0
    Lifted::new(0x7000593690, tpl_220::<0x8> as *const (), 0), // +0x593690 sub_593690
    Lifted::new(0x70005936a0, fn_70005936a0 as *const (), 2), // +0x5936a0 sub_5936a0
    Lifted::new(0x70005939a0, fn_70005939a0 as *const (), 0), // +0x5939a0 sub_5939a0
    Lifted::new(0x7000593b00, tpl_139::<0x38, 0x40, 0x48, 0x50, 0x18, 0x20, 0x28, 0x30, 0x7000597d50, 0x7000597c00> as *const (), 0), // +0x593b00 sub_593b00
    Lifted::new(0x7000593b70, tpl_139::<0x38, 0x40, 0x48, 0x50, 0x18, 0x20, 0x28, 0x30, 0x7000597c80, 0x7000597c00> as *const (), 0), // +0x593b70 sub_593b70
    Lifted::new(0x7000593be0, fn_7000593be0 as *const (), 2), // +0x593be0 sub_593be0
    Lifted::new(0x7000593ee0, fn_7000593ee0 as *const (), 2), // +0x593ee0 sub_593ee0
    Lifted::new(0x7000594200, fn_7000594200 as *const (), 2), // +0x594200 sub_594200
    Lifted::new(0x70005945e0, fn_70005945e0 as *const (), 2), // +0x5945e0 sub_5945e0
    Lifted::new(0x7000594950, fn_7000594950 as *const (), 2), // +0x594950 sub_594950
    Lifted::new(0x7000594f90, fn_7000594f90 as *const (), 2), // +0x594f90 sub_594f90
    Lifted::new(0x7000595480, fn_7000595480 as *const (), 2), // +0x595480 sub_595480
    Lifted::new(0x70005956d0, fn_70005956d0 as *const (), 2), // +0x5956d0 sub_5956d0
    Lifted::new(0x7000595810, fn_7000595810 as *const (), 0), // +0x595810 sub_595810
    Lifted::new(0x70005959c0, fn_70005959c0 as *const (), 0), // +0x5959c0 sub_5959c0
    Lifted::new(0x7000595b80, fn_7000595b80 as *const (), 2), // +0x595b80 sub_595b80
    Lifted::new(0x7000595f70, fn_7000595f70 as *const (), 0), // +0x595f70 sub_595f70
    Lifted::new(0x7000596130, fn_7000596130 as *const (), 2), // +0x596130 sub_596130
    Lifted::new(0x7000596330, fn_7000596330 as *const (), 2), // +0x596330 sub_596330
    Lifted::new(0x7000596620, fn_7000596620 as *const (), 2), // +0x596620 sub_596620
    Lifted::new(0x70005969b0, fn_70005969b0 as *const (), 2), // +0x5969b0 sub_5969b0
    Lifted::new(0x7000596ad0, fn_7000596ad0 as *const (), 2), // +0x596ad0 sub_596ad0
    Lifted::new(0x7000596af0, fn_7000596af0 as *const (), 2), // +0x596af0 sub_596af0
    Lifted::new(0x7000596e50, fn_7000596e50 as *const (), 2), // +0x596e50 sub_596e50
    Lifted::new(0x7000597170, fn_7000597170 as *const (), 2), // +0x597170 sub_597170
    Lifted::new(0x7000597440, fn_7000597440 as *const (), 2), // +0x597440 sub_597440
    Lifted::new(0x7000597580, fn_7000597580 as *const (), 0), // +0x597580 sub_597580
    Lifted::new(0x7000597640, fn_7000597640 as *const (), 0), // +0x597640 sub_597640
    Lifted::new(0x7000597700, fn_7000597700 as *const (), 0), // +0x597700 sub_597700
    Lifted::new(0x7000597840, fn_7000597840 as *const (), 2), // +0x597840 sub_597840
    Lifted::new(0x7000597a10, fn_7000597a10 as *const (), 0), // +0x597a10 sub_597a10
    Lifted::new(0x7000597ab0, fn_7000597ab0 as *const (), 2), // +0x597ab0 sub_597ab0
    Lifted::new(0x7000597c00, fn_7000597c00 as *const (), 0), // +0x597c00 sub_597c00
    Lifted::new(0x7000597c80, fn_7000597c80 as *const (), 0), // +0x597c80 sub_597c80
    Lifted::new(0x7000597d50, fn_7000597d50 as *const (), 0), // +0x597d50 sub_597d50
    Lifted::new(0x7000597e20, fn_7000597e20 as *const (), 0), // +0x597e20 sub_597e20
    Lifted::new(0x7000597f10, fn_7000597f10 as *const (), 0), // +0x597f10 sub_597f10
    Lifted::new(0x7000597fe0, fn_7000597fe0 as *const (), 2), // +0x597fe0 sub_597fe0
    Lifted::new(0x7000598130, fn_7000598130 as *const (), 0), // +0x598130 sub_598130
    Lifted::new(0x70005981b0, fn_70005981b0 as *const (), 2), // +0x5981b0 sub_5981b0
    Lifted::new(0x70005982c0, fn_70005982c0 as *const (), 2), // +0x5982c0 sub_5982c0
    Lifted::new(0x70005984b0, fn_70005984b0 as *const (), 0), // +0x5984b0 sub_5984b0
    Lifted::new(0x7000598550, fn_7000598550 as *const (), 0), // +0x598550 sub_598550
    Lifted::new(0x7000598610, fn_7000598610 as *const (), 1), // +0x598610 sub_598610
    Lifted::new(0x70005986b0, fn_70005986b0 as *const (), 1), // +0x5986b0 sub_5986b0
    Lifted::new(0x7000598750, fn_7000598750 as *const (), 0), // +0x598750 sub_598750
    Lifted::new(0x70005987c0, fn_70005987c0 as *const (), 1), // +0x5987c0 sub_5987c0
    Lifted::new(0x70005988b0, fn_70005988b0 as *const (), 1), // +0x5988b0 sub_5988b0
    Lifted::new(0x70005989a0, tpl_514::<0x70005a8240, 0x700062d148, 0x70002445b0, 0x70006015b0, 0x70005f6d29, 0x70005a8230> as *const (), 0), // +0x5989a0 sub_5989a0
    Lifted::new(0x7000598ae0, tpl_514::<0x70005a8240, 0x700062d0a8, 0x70002445b0, 0x70006015b0, 0x70005f6d29, 0x70005a8230> as *const (), 0), // +0x598ae0 sub_598ae0
    Lifted::new(0x7000598c20, fn_7000598c20 as *const (), 2), // +0x598c20 sub_598c20
    Lifted::new(0x7000598d90, fn_7000598d90 as *const (), 0), // +0x598d90 sub_598d90
    Lifted::new(0x7000598ee0, tpl_515::<0x70005a8240, 0x700062d260, 0x70002445b0, 0x70006015b0, 0x70005a81d0, 0x70005a8230> as *const (), 2), // +0x598ee0 sub_598ee0
    Lifted::new(0x7000599030, fn_7000599030 as *const (), 0), // +0x599030 sub_599030
    Lifted::new(0x7000599170, fn_7000599170 as *const (), 0), // +0x599170 sub_599170
    Lifted::new(0x70005992f0, fn_70005992f0 as *const (), 2), // +0x5992f0 sub_5992f0
    Lifted::new(0x7000599480, tpl_515::<0x70005a8240, 0x700062d408, 0x70002445b0, 0x70006015b0, 0x70005a81d0, 0x70005a8230> as *const (), 2), // +0x599480 sub_599480
    Lifted::new(0x70005995d0, fn_70005995d0 as *const (), 2), // +0x5995d0 sub_5995d0
    Lifted::new(0x7000599740, fn_7000599740 as *const (), 2), // +0x599740 sub_599740
    Lifted::new(0x7000599890, tpl_516::<0x70005a8240, 0x700062d5c8, 0x70002445b0, 0x70006015b0, 0x70005a81d0, 0x70005a8230> as *const (), 2), // +0x599890 sub_599890
    Lifted::new(0x70005999d0, tpl_514::<0x70005a8240, 0x700062d648, 0x70002445b0, 0x70006015b0, 0x70005f6d29, 0x70005a8230> as *const (), 0), // +0x5999d0 sub_5999d0
    Lifted::new(0x7000599b10, fn_7000599b10 as *const (), 2), // +0x599b10 sub_599b10
    Lifted::new(0x7000599c90, fn_7000599c90 as *const (), 2), // +0x599c90 sub_599c90
    Lifted::new(0x7000599e20, fn_7000599e20 as *const (), 2), // +0x599e20 sub_599e20
    Lifted::new(0x7000599fa0, tpl_229::<0x70005a8240, 0x700062d988, 0x70002445b0, 0x700062d970, 0x70005a8230> as *const (), 0), // +0x599fa0 sub_599fa0
    Lifted::new(0x700059a0f0, tpl_516::<0x70005a8240, 0x700062d890, 0x70002445b0, 0x70006015b0, 0x70005a81d0, 0x70005a8230> as *const (), 2), // +0x59a0f0 sub_59a0f0
    Lifted::new(0x700059a230, fn_700059a230 as *const (), 2), // +0x59a230 sub_59a230
    Lifted::new(0x700059a2f0, fn_700059a2f0 as *const (), 2), // +0x59a2f0 sub_59a2f0
    Lifted::new(0x700059a3d0, tpl_348::<0x8, 0x70005984b0> as *const (), 0), // +0x59a3d0 sub_59a3d0
    Lifted::new(0x700059a430, fn_700059a430 as *const (), 0), // +0x59a430 sub_59a430
    Lifted::new(0x700059a4c0, fn_700059a4c0 as *const (), 2), // +0x59a4c0 sub_59a4c0
    Lifted::new(0x700059a560, fn_700059a560 as *const (), 2), // +0x59a560 sub_59a560
    Lifted::new(0x700059aca0, fn_700059aca0 as *const (), 2), // +0x59aca0 sub_59aca0
    Lifted::new(0x700059b910, fn_700059b910 as *const (), 2), // +0x59b910 sub_59b910
    Lifted::new(0x700059c650, fn_700059c650 as *const (), 0), // +0x59c650 sub_59c650
    Lifted::new(0x700059c760, fn_700059c760 as *const (), 0), // +0x59c760 sub_59c760
    Lifted::new(0x700059c860, fn_700059c860 as *const (), 0), // +0x59c860 sub_59c860
    Lifted::new(0x700059c930, fn_700059c930 as *const (), 0), // +0x59c930 sub_59c930
    Lifted::new(0x700059cb10, fn_700059cb10 as *const (), 0), // +0x59cb10 sub_59cb10
    Lifted::new(0x700059ccf0, fn_700059ccf0 as *const (), 3), // +0x59ccf0 sub_59ccf0
    Lifted::new(0x700059cda0, fn_700059cda0 as *const (), 3), // +0x59cda0 sub_59cda0
    Lifted::new(0x700059ce60, fn_700059ce60 as *const (), 3), // +0x59ce60 sub_59ce60
    Lifted::new(0x700059cfb0, fn_700059cfb0 as *const (), 3), // +0x59cfb0 sub_59cfb0
    Lifted::new(0x700059d100, fn_700059d100 as *const (), 3), // +0x59d100 sub_59d100
    Lifted::new(0x700059d260, fn_700059d260 as *const (), 3), // +0x59d260 sub_59d260
    Lifted::new(0x700059d3d0, fn_700059d3d0 as *const (), 3), // +0x59d3d0 sub_59d3d0
    Lifted::new(0x700059d530, fn_700059d530 as *const (), 3), // +0x59d530 sub_59d530
    Lifted::new(0x700059d698, fn_700059d698 as *const (), 0), // +0x59d698 sub_59d698
    Lifted::new(0x700059d6b0, fn_700059d6b0 as *const (), 2), // +0x59d6b0 sub_59d6b0
    Lifted::new(0x700059d750, fn_700059d750 as *const (), 2), // +0x59d750 sub_59d750
    Lifted::new(0x700059d970, fn_700059d970 as *const (), 3), // +0x59d970 sub_59d970
    Lifted::new(0x700059da00, fn_700059da00 as *const (), 2), // +0x59da00 sub_59da00
    Lifted::new(0x700059def0, fn_700059def0 as *const (), 2), // +0x59def0 sub_59def0
    Lifted::new(0x700059e750, fn_700059e750 as *const (), 2), // +0x59e750 sub_59e750
    Lifted::new(0x700059e7d0, fn_700059e7d0 as *const (), 2), // +0x59e7d0 sub_59e7d0
    Lifted::new(0x700059e8d0, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x700059e960> as *const (), 2), // +0x59e8d0 sub_59e8d0
    Lifted::new(0x700059e960, fn_700059e960 as *const (), 0), // +0x59e960 sub_59e960
    Lifted::new(0x700059e9e0, fn_700059e9e0 as *const (), 0), // +0x59e9e0 sub_59e9e0
    Lifted::new(0x700059eae0, fn_700059eae0 as *const (), 2), // +0x59eae0 sub_59eae0
    Lifted::new(0x700059edb0, fn_700059edb0 as *const (), 2), // +0x59edb0 sub_59edb0
    Lifted::new(0x700059f020, fn_700059f020 as *const (), 2), // +0x59f020 sub_59f020
    Lifted::new(0x700059f2f0, fn_700059f2f0 as *const (), 0), // +0x59f2f0 sub_59f2f0
    Lifted::new(0x700059f370, fn_700059f370 as *const (), 2), // +0x59f370 sub_59f370
    Lifted::new(0x700059f600, fn_700059f600 as *const (), 0), // +0x59f600 sub_59f600
    Lifted::new(0x700059f700, fn_700059f700 as *const (), 2), // +0x59f700 sub_59f700
    Lifted::new(0x700059f7a0, fn_700059f7a0 as *const (), 0), // +0x59f7a0 sub_59f7a0
    Lifted::new(0x700059f830, fn_700059f830 as *const (), 2), // +0x59f830 sub_59f830
    Lifted::new(0x700059f9c0, fn_700059f9c0 as *const (), 0), // +0x59f9c0 sub_59f9c0
    Lifted::new(0x700059fb00, fn_700059fb00 as *const (), 0), // +0x59fb00 sub_59fb00
    Lifted::new(0x700059fdd0, fn_700059fdd0 as *const (), 0), // +0x59fdd0 sub_59fdd0
    Lifted::new(0x70005a0020, fn_70005a0020 as *const (), 2), // +0x5a0020 sub_5a0020
    Lifted::new(0x70005a02f0, fn_70005a02f0 as *const (), 2), // +0x5a02f0 sub_5a02f0
    Lifted::new(0x70005a0620, fn_70005a0620 as *const (), 3), // +0x5a0620 sub_5a0620
    Lifted::new(0x70005a06d0, fn_70005a06d0 as *const (), 2), // +0x5a06d0 sub_5a06d0
    Lifted::new(0x70005a0b10, fn_70005a0b10 as *const (), 2), // +0x5a0b10 sub_5a0b10
    Lifted::new(0x70005a0dd0, fn_70005a0dd0 as *const (), 2), // +0x5a0dd0 sub_5a0dd0
    Lifted::new(0x70005a0ed0, fn_70005a0ed0 as *const (), 2), // +0x5a0ed0 sub_5a0ed0
    Lifted::new(0x70005a1020, fn_70005a1020 as *const (), 1), // +0x5a1020 sub_5a1020
    Lifted::new(0x70005a1040, fn_70005a1040 as *const (), 2), // +0x5a1040 sub_5a1040
    Lifted::new(0x70005a1280, fn_70005a1280 as *const (), 0), // +0x5a1280 sub_5a1280
    Lifted::new(0x70005a1290, fn_70005a1290 as *const (), 0), // +0x5a1290 sub_5a1290
    Lifted::new(0x70005a1330, fn_70005a1330 as *const (), 2), // +0x5a1330 sub_5a1330
    Lifted::new(0x70005a1420, fn_70005a1420 as *const (), 0), // +0x5a1420 sub_5a1420
    Lifted::new(0x70005a14a0, fn_70005a14a0 as *const (), 0), // +0x5a14a0 sub_5a14a0
    Lifted::new(0x70005a1530, fn_70005a1530 as *const (), 2), // +0x5a1530 sub_5a1530
    Lifted::new(0x70005a17a0, fn_70005a17a0 as *const (), 2), // +0x5a17a0 sub_5a17a0
    Lifted::new(0x70005a1830, fn_70005a1830 as *const (), 2), // +0x5a1830 sub_5a1830
    Lifted::new(0x70005a1a10, fn_70005a1a10 as *const (), 2), // +0x5a1a10 sub_5a1a10
    Lifted::new(0x70005a1be0, fn_70005a1be0 as *const (), 2), // +0x5a1be0 sub_5a1be0
    Lifted::new(0x70005a1de0, fn_70005a1de0 as *const (), 2), // +0x5a1de0 sub_5a1de0
    Lifted::new(0x70005a1f60, fn_70005a1f60 as *const (), 0), // +0x5a1f60 sub_5a1f60
    Lifted::new(0x70005a2020, tpl_249::<0x7000694258, 0x70005a83d0, 0x7000694250, 0x8> as *const (), 2), // +0x5a2020 sub_5a2020
    Lifted::new(0x70005a20a0, fn_70005a20a0 as *const (), 0), // +0x5a20a0 sub_5a20a0
    Lifted::new(0x70005a20c0, tpl_249::<0x7000694298, 0x70005a83d0, 0x7000694290, 0x8> as *const (), 2), // +0x5a20c0 sub_5a20c0
    Lifted::new(0x70005a2140, fn_70005a2140 as *const (), 2), // +0x5a2140 sub_5a2140
    Lifted::new(0x70005a2250, fn_70005a2250 as *const (), 2), // +0x5a2250 sub_5a2250
    Lifted::new(0x70005a2440, fn_70005a2440 as *const (), 2), // +0x5a2440 sub_5a2440
    Lifted::new(0x70005a2550, fn_70005a2550 as *const (), 0), // +0x5a2550 sub_5a2550
    Lifted::new(0x70005a26b0, fn_70005a26b0 as *const (), 2), // +0x5a26b0 sub_5a26b0
    Lifted::new(0x70005a2a60, tpl_399::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000598750> as *const (), 2), // +0x5a2a60 sub_5a2a60
    Lifted::new(0x70005a2b00, fn_70005a2b00 as *const (), 0), // +0x5a2b00 ?Send@IClientSideSocket@cohtml@@UEAAXPEBDIW4OpCode@websocket@2@@Z
    Lifted::new(0x70005a2b10, tpl_160::<0x700062e0b8> as *const (), 1), // +0x5a2b10 ??0IClientSideSocket@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70005a2b20, tpl_161::<0xfffffffffffffff8, 0x700062e0b8, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x5a2b20 sub_5a2b20
    Lifted::new(0x70005a2bb0, tpl_160::<0x700062e0f8> as *const (), 1), // +0x5a2bb0 ??0ISocketListener@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70005a2bc0, tpl_161::<0xfffffffffffffff8, 0x700062e0f8, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x5a2bc0 sub_5a2bc0
    Lifted::new(0x70005a2c50, fn_70005a2c50 as *const (), 1), // +0x5a2c50 ?OnScreenInfoRequested@IViewListener@cohtml@@UEAA?AUScreenInfo@2@XZ
    Lifted::new(0x70005a2c80, tpl_160::<0x700062e130> as *const (), 1), // +0x5a2c80 ??0IViewListener@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70005a2c90, tpl_161::<0xfffffffffffffff8, 0x700062e130, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x5a2c90 sub_5a2c90
    Lifted::new(0x70005a2d20, fn_70005a2d20 as *const (), 0), // +0x5a2d20 sub_5a2d20
    Lifted::new(0x70005a31c0, fn_70005a31c0 as *const (), 0), // +0x5a31c0 sub_5a31c0
    Lifted::new(0x70005a31f0, fn_70005a31f0 as *const (), 0), // +0x5a31f0 sub_5a31f0
    Lifted::new(0x70005a33c0, tpl_164::<0x700062dfe8> as *const (), 1), // +0x5a33c0 ??1IDataStorageRequest@cohtml@@UEAA@XZ
    Lifted::new(0x70005a33d0, tpl_164::<0x700062dfb0> as *const (), 1), // +0x5a33d0 ??1IDataStorageResponse@cohtml@@UEAA@XZ
    Lifted::new(0x70005a33e0, tpl_164::<0x700062df00> as *const (), 1), // +0x5a33e0 ??1IDataStorageHandler@cohtml@@UEAA@XZ
    Lifted::new(0x70005a33f0, tpl_164::<0x700062dd28> as *const (), 1), // +0x5a33f0 ??1IInternationalizationManager@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3400, tpl_164::<0x700062dcc0> as *const (), 1), // +0x5a3400 ??1IBreakIterator@i18n@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3410, tpl_164::<0x700062dda8> as *const (), 1), // +0x5a3410 ??1IAllocator@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3420, tpl_164::<0x700062e018> as *const (), 1), // +0x5a3420 ??1IVirtualAllocator@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3430, tpl_164::<0x700062e070> as *const (), 1), // +0x5a3430 ??1ILocalizationManager@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3440, tpl_164::<0x700062dfd0> as *const (), 1), // +0x5a3440 ??1ILogHandler@Logging@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3450, tpl_164::<0x700062df60> as *const (), 1), // +0x5a3450 ??1IPerformanceHandler@Profile@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3460, tpl_164::<0x700062ddf0> as *const (), 1), // +0x5a3460 ??1IAsyncResourceRequest@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3470, tpl_164::<0x700062de68> as *const (), 1), // +0x5a3470 ??1IAsyncResourceResponse@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3480, tpl_164::<0x700062df90> as *const (), 1), // +0x5a3480 ??1IAsyncResourceStreamResponse@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3490, tpl_164::<0x700062df38> as *const (), 1), // +0x5a3490 ??1IAsyncResourceHandler@cohtml@@UEAA@XZ
    Lifted::new(0x70005a34a0, tpl_517::<0x70005a82f8, 0x700062deb8> as *const (), 0), // +0x5a34a0 ??0ISyncStreamReader@cohtml@@IEAA@XZ
    Lifted::new(0x70005a34d0, tpl_518::<0x700062deb8, 0x70005a8300> as *const (), 0), // +0x5a34d0 ??1ISyncStreamReader@cohtml@@UEAA@XZ
    Lifted::new(0x70005a34f0, tpl_517::<0x70005a8370, 0x700062e048> as *const (), 0), // +0x5a34f0 ??0IFileSystemReader@cohtml@@IEAA@XZ
    Lifted::new(0x70005a3520, tpl_518::<0x700062e048, 0x70005a8390> as *const (), 0), // +0x5a3520 ??1IFileSystemReader@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3540, tpl_164::<0x700062e0b8> as *const (), 1), // +0x5a3540 ??1IClientSideSocket@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3550, tpl_164::<0x700062e0f8> as *const (), 1), // +0x5a3550 ??1ISocketListener@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3560, tpl_164::<0x700062dc90> as *const (), 1), // +0x5a3560 ??1ITextInputHandler@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3570, tpl_164::<0x700062dc70> as *const (), 1), // +0x5a3570 ??1ITextTransformationManager@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3580, tpl_164::<0x700062e130> as *const (), 1), // +0x5a3580 ??1IViewListener@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3590, tpl_164::<0x700062e090> as *const (), 1), // +0x5a3590 ??1ITimeZoneProvider@cohtml@@UEAA@XZ
    Lifted::new(0x70005a35a0, tpl_164::<0x700062df78> as *const (), 1), // +0x5a35a0 ??1IFileSystemWriter@cohtml@@UEAA@XZ
    Lifted::new(0x70005a35b0, fn_70005a35b0 as *const (), 0), // +0x5a35b0 ??1ISyncStreamWriter@cohtml@@UEAA@XZ
    Lifted::new(0x70005a3670, fn_70005a3670 as *const (), 3), // +0x5a3670 sub_5a3670
    Lifted::new(0x70005a3840, fn_70005a3840 as *const (), 3), // +0x5a3840 sub_5a3840
    Lifted::new(0x70005a3940, fn_70005a3940 as *const (), 3), // +0x5a3940 sub_5a3940
    Lifted::new(0x70005a3a10, fn_70005a3a10 as *const (), 3), // +0x5a3a10 sub_5a3a10
    Lifted::new(0x70005a3ac0, fn_70005a3ac0 as *const (), 2), // +0x5a3ac0 sub_5a3ac0
    Lifted::new(0x70005a3ba0, fn_70005a3ba0 as *const (), 3), // +0x5a3ba0 sub_5a3ba0
    Lifted::new(0x70005a3c70, fn_70005a3c70 as *const (), 3), // +0x5a3c70 sub_5a3c70
    Lifted::new(0x70005a3d90, fn_70005a3d90 as *const (), 0), // +0x5a3d90 sub_5a3d90
    Lifted::new(0x70005a3dcc, fn_70005a3dcc as *const (), 0), // +0x5a3dcc sub_5a3dcc
    Lifted::new(0x70005a3e00, fn_70005a3e00 as *const (), 0), // +0x5a3e00 sub_5a3e00
    Lifted::new(0x70005a3e18, fn_70005a3e18 as *const (), 0), // +0x5a3e18 sub_5a3e18
    Lifted::new(0x70005a3e40, fn_70005a3e40 as *const (), 0), // +0x5a3e40 sub_5a3e40
    Lifted::new(0x70005a3e58, fn_70005a3e58 as *const (), 0), // +0x5a3e58 sub_5a3e58
    Lifted::new(0x70005a3eb8, fn_70005a3eb8 as *const (), 0), // +0x5a3eb8 sub_5a3eb8
    Lifted::new(0x70005a3ee8, fn_70005a3ee8 as *const (), 0), // +0x5a3ee8 sub_5a3ee8
    Lifted::new(0x70005a3efc, fn_70005a3efc as *const (), 0), // +0x5a3efc sub_5a3efc
    Lifted::new(0x70005a3f38, fn_70005a3f38 as *const (), 2), // +0x5a3f38 sub_5a3f38
    Lifted::new(0x70005a3fc4, fn_70005a3fc4 as *const (), 1), // +0x5a3fc4 sub_5a3fc4
    Lifted::new(0x70005a405c, fn_70005a405c as *const (), 0), // +0x5a405c sub_5a405c
    Lifted::new(0x70005a4080, fn_70005a4080 as *const (), 0), // +0x5a4080 sub_5a4080
    Lifted::new(0x70005a40ac, fn_70005a40ac as *const (), 0), // +0x5a40ac sub_5a40ac
    Lifted::new(0x70005a40e8, fn_70005a40e8 as *const (), 0), // +0x5a40e8 sub_5a40e8
    Lifted::new(0x70005a4108, fn_70005a4108 as *const (), 0), // +0x5a4108 sub_5a4108
    Lifted::new(0x70005a4174, fn_70005a4174 as *const (), 0), // +0x5a4174 sub_5a4174
    Lifted::new(0x70005a41ec, fn_70005a41ec as *const (), 0), // +0x5a41ec sub_5a41ec
    Lifted::new(0x70005a4254, fn_70005a4254 as *const (), 0), // +0x5a4254 sub_5a4254
    Lifted::new(0x70005a4264, tpl_94::<0x70005a9758, 0x700048c2c0> as *const (), 0), // +0x5a4264 sub_5a4264
    Lifted::new(0x70005a4298, fn_70005a4298 as *const (), 0), // +0x5a4298 sub_5a4298
    Lifted::new(0x70005a42e8, fn_70005a42e8 as *const (), 0), // +0x5a42e8 sub_5a42e8
    Lifted::new(0x70005a4400, fn_70005a4400 as *const (), 0), // +0x5a4400 sub_5a4400
    Lifted::new(0x70005a4480, fn_70005a4480 as *const (), 0), // +0x5a4480 sub_5a4480
    Lifted::new(0x70005a45b4, fn_70005a45b4 as *const (), 0), // +0x5a45b4 sub_5a45b4
    Lifted::new(0x70005a4610, fn_70005a4610 as *const (), 0), // +0x5a4610 sub_5a4610
    Lifted::new(0x70005a4630, fn_70005a4630 as *const (), 1), // +0x5a4630 sub_5a4630
    Lifted::new(0x70005a47dc, fn_70005a47dc as *const (), 1), // +0x5a47dc sub_5a47dc
    Lifted::new(0x70005a47e4, fn_70005a47e4 as *const (), 1), // +0x5a47e4 sub_5a47e4
    Lifted::new(0x70005a4940, fn_70005a4940 as *const (), 0), // +0x5a4940 sub_5a4940
    Lifted::new(0x70005a49ec, fn_70005a49ec as *const (), 0), // +0x5a49ec sub_5a49ec
    Lifted::new(0x70005a4a10, tpl_519::<0x7000694980, 0x70005a8020> as *const (), 0), // +0x5a4a10 sub_5a4a10
    Lifted::new(0x70005a4a20, tpl_3::<0x7000694980, 0x70005a4c68> as *const (), 0), // +0x5a4a20 sub_5a4a20
    Lifted::new(0x70005a4a2c, tpl_9::<0x7000694990> as *const (), 1), // +0x5a4a2c sub_5a4a2c
    Lifted::new(0x70005a4a34, fn_70005a4a34 as *const (), 0), // +0x5a4a34 sub_5a4a34
    Lifted::new(0x70005a4a50, tpl_9::<0x70005a9748> as *const (), 1), // +0x5a4a50 sub_5a4a50
    Lifted::new(0x70005a4a58, tpl_520::<0x700062fd60, 0x70005a8f10> as *const (), 0), // +0x5a4a58 sub_5a4a58
    Lifted::new(0x70005a4a94, tpl_520::<0x700062fd70, 0x70005a8f10> as *const (), 0), // +0x5a4a94 sub_5a4a94
    Lifted::new(0x70005a4ad0, fn_70005a4ad0 as *const (), 0), // +0x5a4ad0 sub_5a4ad0
    Lifted::new(0x70005a4b04, fn_70005a4b04 as *const (), 0), // +0x5a4b04 sub_5a4b04
    Lifted::new(0x70005a4cc0, fn_70005a4cc0 as *const (), 1), // +0x5a4cc0 sub_5a4cc0
    Lifted::new(0x70005a4d90, fn_70005a4d90 as *const (), 2), // +0x5a4d90 sub_5a4d90
    Lifted::new(0x70005a4db0, fn_70005a4db0 as *const (), 2), // +0x5a4db0 sub_5a4db0
    Lifted::new(0x70005a4db6, fn_70005a4db6 as *const (), 1), // +0x5a4db6 sub_5a4db6
    Lifted::new(0x70005a4dce, fn_70005a4dce as *const (), 0), // +0x5a4dce sub_5a4dce
    Lifted::new(0x70005a4de5, fn_70005a4de5 as *const (), 0), // +0x5a4de5 sub_5a4de5
    Lifted::new(0x70005a4dfe, fn_70005a4dfe as *const (), 0), // +0x5a4dfe sub_5a4dfe
    Lifted::new(0x70005a4e12, fn_70005a4e12 as *const (), 0), // +0x5a4e12 sub_5a4e12
    Lifted::new(0x70005a4e50, tpl_519::<0x7000699840, 0x70005a8108> as *const (), 0), // +0x5a4e50 sub_5a4e50
    Lifted::new(0x70005a4e60, tpl_519::<0x7000699ac0, 0x70005a8108> as *const (), 0), // +0x5a4e60 sub_5a4e60
    Lifted::new(0x70005a4e70, tpl_519::<0x7000699a70, 0x70005a8108> as *const (), 0), // +0x5a4e70 sub_5a4e70
    Lifted::new(0x70005a4e80, tpl_519::<0x7000699a20, 0x70005a8108> as *const (), 0), // +0x5a4e80 sub_5a4e80
    Lifted::new(0x70005a4e90, tpl_519::<0x70006999d0, 0x70005a8108> as *const (), 0), // +0x5a4e90 sub_5a4e90
    Lifted::new(0x70005a4ea0, tpl_519::<0x7000699980, 0x70005a8108> as *const (), 0), // +0x5a4ea0 sub_5a4ea0
    Lifted::new(0x70005a4eb0, tpl_519::<0x7000699930, 0x70005a8108> as *const (), 0), // +0x5a4eb0 sub_5a4eb0
    Lifted::new(0x70005a4ec0, tpl_519::<0x70006998e0, 0x70005a8108> as *const (), 0), // +0x5a4ec0 sub_5a4ec0
    Lifted::new(0x70005a4ed0, tpl_519::<0x7000699890, 0x70005a8108> as *const (), 0), // +0x5a4ed0 sub_5a4ed0
    Lifted::new(0x70005a4ee0, tpl_521 as *const (), 1), // +0x5a4ee0 sub_5a4ee0
    Lifted::new(0x70005a4ef0, tpl_521 as *const (), 1), // +0x5a4ef0 sub_5a4ef0
    Lifted::new(0x70005a4f00, tpl_521 as *const (), 1), // +0x5a4f00 sub_5a4f00
    Lifted::new(0x70005a4f10, tpl_521 as *const (), 1), // +0x5a4f10 sub_5a4f10
    Lifted::new(0x70005a4f20, tpl_521 as *const (), 1), // +0x5a4f20 sub_5a4f20
    Lifted::new(0x70005a4f30, tpl_521 as *const (), 1), // +0x5a4f30 sub_5a4f30
    Lifted::new(0x70005a4f40, tpl_521 as *const (), 1), // +0x5a4f40 sub_5a4f40
    Lifted::new(0x70005a4f50, tpl_521 as *const (), 1), // +0x5a4f50 sub_5a4f50
    Lifted::new(0x70005a4f60, tpl_521 as *const (), 1), // +0x5a4f60 sub_5a4f60
    Lifted::new(0x70005a4f70, tpl_521 as *const (), 1), // +0x5a4f70 sub_5a4f70
    Lifted::new(0x70005a4f80, tpl_521 as *const (), 1), // +0x5a4f80 sub_5a4f80
    Lifted::new(0x70005a4f90, tpl_521 as *const (), 1), // +0x5a4f90 sub_5a4f90
    Lifted::new(0x70005a4fa0, tpl_521 as *const (), 1), // +0x5a4fa0 sub_5a4fa0
    Lifted::new(0x70005a4fb0, tpl_521 as *const (), 1), // +0x5a4fb0 sub_5a4fb0
    Lifted::new(0x70005a4fc0, tpl_521 as *const (), 1), // +0x5a4fc0 sub_5a4fc0
    Lifted::new(0x70005a4fd0, tpl_521 as *const (), 1), // +0x5a4fd0 sub_5a4fd0
    Lifted::new(0x70005a4fe0, tpl_521 as *const (), 1), // +0x5a4fe0 sub_5a4fe0
    Lifted::new(0x70005a4ff0, tpl_521 as *const (), 1), // +0x5a4ff0 sub_5a4ff0
    Lifted::new(0x70005a5000, tpl_521 as *const (), 1), // +0x5a5000 sub_5a5000
    Lifted::new(0x70005a5010, tpl_521 as *const (), 1), // +0x5a5010 sub_5a5010
    Lifted::new(0x70005a5020, tpl_521 as *const (), 1), // +0x5a5020 sub_5a5020
    Lifted::new(0x70005a5030, tpl_521 as *const (), 1), // +0x5a5030 sub_5a5030
    Lifted::new(0x70005a5040, tpl_521 as *const (), 1), // +0x5a5040 sub_5a5040
    Lifted::new(0x70005a5050, tpl_521 as *const (), 1), // +0x5a5050 sub_5a5050
    Lifted::new(0x70005a5060, tpl_521 as *const (), 1), // +0x5a5060 sub_5a5060
    Lifted::new(0x70005a5070, tpl_521 as *const (), 1), // +0x5a5070 sub_5a5070
    Lifted::new(0x70005a5080, tpl_521 as *const (), 1), // +0x5a5080 sub_5a5080
    Lifted::new(0x70005a5090, tpl_521 as *const (), 1), // +0x5a5090 sub_5a5090
    Lifted::new(0x70005a50a0, tpl_521 as *const (), 1), // +0x5a50a0 sub_5a50a0
    Lifted::new(0x70005a50b0, tpl_521 as *const (), 1), // +0x5a50b0 sub_5a50b0
    Lifted::new(0x70005a50c0, tpl_521 as *const (), 1), // +0x5a50c0 sub_5a50c0
    Lifted::new(0x70005a50d0, tpl_521 as *const (), 1), // +0x5a50d0 sub_5a50d0
    Lifted::new(0x70005a50e0, tpl_521 as *const (), 1), // +0x5a50e0 sub_5a50e0
    Lifted::new(0x70005a50f0, tpl_521 as *const (), 1), // +0x5a50f0 sub_5a50f0
    Lifted::new(0x70005a5100, tpl_521 as *const (), 1), // +0x5a5100 sub_5a5100
    Lifted::new(0x70005a5110, tpl_521 as *const (), 1), // +0x5a5110 sub_5a5110
    Lifted::new(0x70005a5120, tpl_521 as *const (), 1), // +0x5a5120 sub_5a5120
    Lifted::new(0x70005a5130, tpl_521 as *const (), 1), // +0x5a5130 sub_5a5130
    Lifted::new(0x70005a5140, tpl_522::<0x70006904e8, 0x70006904f8, 0x70006904f0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5140 sub_5a5140
    Lifted::new(0x70005a5180, tpl_522::<0x7000690500, 0x7000690510, 0x7000690508, 0x7000697150, 0x10> as *const (), 0), // +0x5a5180 sub_5a5180
    Lifted::new(0x70005a51c0, tpl_521 as *const (), 1), // +0x5a51c0 sub_5a51c0
    Lifted::new(0x70005a51d0, tpl_522::<0x7000690518, 0x7000690528, 0x7000690520, 0x7000697150, 0x10> as *const (), 0), // +0x5a51d0 sub_5a51d0
    Lifted::new(0x70005a5210, tpl_522::<0x7000690530, 0x7000690540, 0x7000690538, 0x7000697150, 0x10> as *const (), 0), // +0x5a5210 sub_5a5210
    Lifted::new(0x70005a5250, tpl_521 as *const (), 1), // +0x5a5250 sub_5a5250
    Lifted::new(0x70005a5260, tpl_521 as *const (), 1), // +0x5a5260 sub_5a5260
    Lifted::new(0x70005a5270, tpl_521 as *const (), 1), // +0x5a5270 sub_5a5270
    Lifted::new(0x70005a5280, tpl_521 as *const (), 1), // +0x5a5280 sub_5a5280
    Lifted::new(0x70005a5290, tpl_521 as *const (), 1), // +0x5a5290 sub_5a5290
    Lifted::new(0x70005a52a0, tpl_521 as *const (), 1), // +0x5a52a0 sub_5a52a0
    Lifted::new(0x70005a52b0, tpl_521 as *const (), 1), // +0x5a52b0 sub_5a52b0
    Lifted::new(0x70005a52c0, tpl_521 as *const (), 1), // +0x5a52c0 sub_5a52c0
    Lifted::new(0x70005a52d0, tpl_521 as *const (), 1), // +0x5a52d0 sub_5a52d0
    Lifted::new(0x70005a52e0, tpl_522::<0x7000690548, 0x7000690558, 0x7000690550, 0x7000697150, 0x10> as *const (), 0), // +0x5a52e0 sub_5a52e0
    Lifted::new(0x70005a5320, tpl_522::<0x7000690560, 0x7000690570, 0x7000690568, 0x7000697150, 0x10> as *const (), 0), // +0x5a5320 sub_5a5320
    Lifted::new(0x70005a5360, tpl_521 as *const (), 1), // +0x5a5360 sub_5a5360
    Lifted::new(0x70005a5370, tpl_521 as *const (), 1), // +0x5a5370 sub_5a5370
    Lifted::new(0x70005a5380, tpl_521 as *const (), 1), // +0x5a5380 sub_5a5380
    Lifted::new(0x70005a5390, tpl_521 as *const (), 1), // +0x5a5390 sub_5a5390
    Lifted::new(0x70005a53a0, tpl_521 as *const (), 1), // +0x5a53a0 sub_5a53a0
    Lifted::new(0x70005a53b0, tpl_521 as *const (), 1), // +0x5a53b0 sub_5a53b0
    Lifted::new(0x70005a53c0, tpl_521 as *const (), 1), // +0x5a53c0 sub_5a53c0
    Lifted::new(0x70005a53d0, tpl_521 as *const (), 1), // +0x5a53d0 sub_5a53d0
    Lifted::new(0x70005a53e0, tpl_522::<0x7000690600, 0x7000690610, 0x7000690608, 0x7000697150, 0x10> as *const (), 0), // +0x5a53e0 sub_5a53e0
    Lifted::new(0x70005a5420, tpl_522::<0x7000690618, 0x7000690628, 0x7000690620, 0x7000697150, 0x10> as *const (), 0), // +0x5a5420 sub_5a5420
    Lifted::new(0x70005a5460, tpl_522::<0x7000690630, 0x7000690640, 0x7000690638, 0x7000697150, 0x10> as *const (), 0), // +0x5a5460 sub_5a5460
    Lifted::new(0x70005a54a0, tpl_522::<0x7000690648, 0x7000690658, 0x7000690650, 0x7000697150, 0x10> as *const (), 0), // +0x5a54a0 sub_5a54a0
    Lifted::new(0x70005a54e0, tpl_521 as *const (), 1), // +0x5a54e0 sub_5a54e0
    Lifted::new(0x70005a54f0, tpl_521 as *const (), 1), // +0x5a54f0 sub_5a54f0
    Lifted::new(0x70005a5500, tpl_522::<0x7000690660, 0x7000690670, 0x7000690668, 0x7000697150, 0x10> as *const (), 0), // +0x5a5500 sub_5a5500
    Lifted::new(0x70005a5540, tpl_523::<0x7000690678, 0x7000690688, 0x7000690680, 0x7000697150, 0x10, 0x70005a5574, 0x70005a5575, 0x70005a5576, 0x70005a5577, 0x70005a5578, 0x70005a5579, 0x70005a557a, 0x70005a557b, 0x70005a557c, 0x70005a557d, 0x70005a557e, 0x70005a557f, 0x700062e0b8, 0x7000690690> as *const (), 0), // +0x5a5540 sub_5a5540
    Lifted::new(0x70005a5590, tpl_522::<0x7000690698, 0x70006906a8, 0x70006906a0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5590 sub_5a5590
    Lifted::new(0x70005a55d0, tpl_522::<0x70006906b0, 0x70006906c0, 0x70006906b8, 0x7000697150, 0x10> as *const (), 0), // +0x5a55d0 sub_5a55d0
    Lifted::new(0x70005a5610, tpl_521 as *const (), 1), // +0x5a5610 sub_5a5610
    Lifted::new(0x70005a5620, tpl_522::<0x70006906c8, 0x70006906d8, 0x70006906d0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5620 sub_5a5620
    Lifted::new(0x70005a5660, tpl_522::<0x70006906e0, 0x70006906f0, 0x70006906e8, 0x7000697150, 0x10> as *const (), 0), // +0x5a5660 sub_5a5660
    Lifted::new(0x70005a56a0, tpl_521 as *const (), 1), // +0x5a56a0 sub_5a56a0
    Lifted::new(0x70005a56b0, tpl_522::<0x70006906f8, 0x7000690708, 0x7000690700, 0x7000697150, 0x10> as *const (), 0), // +0x5a56b0 sub_5a56b0
    Lifted::new(0x70005a56f0, tpl_522::<0x7000690710, 0x7000690720, 0x7000690718, 0x7000697150, 0x10> as *const (), 0), // +0x5a56f0 sub_5a56f0
    Lifted::new(0x70005a5730, tpl_522::<0x7000690728, 0x7000690738, 0x7000690730, 0x7000697150, 0x10> as *const (), 0), // +0x5a5730 sub_5a5730
    Lifted::new(0x70005a5770, tpl_522::<0x7000690740, 0x7000690750, 0x7000690748, 0x7000697150, 0x10> as *const (), 0), // +0x5a5770 sub_5a5770
    Lifted::new(0x70005a57b0, tpl_522::<0x7000690758, 0x7000690768, 0x7000690760, 0x7000697150, 0x10> as *const (), 0), // +0x5a57b0 sub_5a57b0
    Lifted::new(0x70005a57f0, tpl_522::<0x7000690770, 0x7000690780, 0x7000690778, 0x7000697150, 0x10> as *const (), 0), // +0x5a57f0 sub_5a57f0
    Lifted::new(0x70005a5830, tpl_522::<0x7000690788, 0x7000690798, 0x7000690790, 0x7000697150, 0x10> as *const (), 0), // +0x5a5830 sub_5a5830
    Lifted::new(0x70005a5870, tpl_522::<0x70006907a0, 0x70006907b0, 0x70006907a8, 0x7000697150, 0x10> as *const (), 0), // +0x5a5870 sub_5a5870
    Lifted::new(0x70005a58b0, tpl_522::<0x70006907b8, 0x70006907c8, 0x70006907c0, 0x7000697150, 0x10> as *const (), 0), // +0x5a58b0 sub_5a58b0
    Lifted::new(0x70005a58f0, tpl_522::<0x70006907d0, 0x70006907e0, 0x70006907d8, 0x7000697150, 0x10> as *const (), 0), // +0x5a58f0 sub_5a58f0
    Lifted::new(0x70005a5930, tpl_522::<0x70006907e8, 0x70006907f8, 0x70006907f0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5930 sub_5a5930
    Lifted::new(0x70005a5970, tpl_522::<0x7000690800, 0x7000690810, 0x7000690808, 0x7000697150, 0x10> as *const (), 0), // +0x5a5970 sub_5a5970
    Lifted::new(0x70005a59b0, tpl_521 as *const (), 1), // +0x5a59b0 sub_5a59b0
    Lifted::new(0x70005a59c0, tpl_522::<0x7000690818, 0x7000690828, 0x7000690820, 0x7000697150, 0x10> as *const (), 0), // +0x5a59c0 sub_5a59c0
    Lifted::new(0x70005a5a00, tpl_522::<0x7000690830, 0x7000690840, 0x7000690838, 0x7000697150, 0x10> as *const (), 0), // +0x5a5a00 sub_5a5a00
    Lifted::new(0x70005a5a40, tpl_522::<0x7000690848, 0x7000690858, 0x7000690850, 0x7000697150, 0x10> as *const (), 0), // +0x5a5a40 sub_5a5a40
    Lifted::new(0x70005a5a80, tpl_522::<0x7000690860, 0x7000690870, 0x7000690868, 0x7000697150, 0x10> as *const (), 0), // +0x5a5a80 sub_5a5a80
    Lifted::new(0x70005a5ac0, tpl_522::<0x7000690878, 0x7000690888, 0x7000690880, 0x7000697150, 0x10> as *const (), 0), // +0x5a5ac0 sub_5a5ac0
    Lifted::new(0x70005a5b00, tpl_522::<0x7000690890, 0x70006908a0, 0x7000690898, 0x7000697150, 0x10> as *const (), 0), // +0x5a5b00 sub_5a5b00
    Lifted::new(0x70005a5b40, tpl_521 as *const (), 1), // +0x5a5b40 sub_5a5b40
    Lifted::new(0x70005a5b50, tpl_522::<0x70006908a8, 0x70006908b8, 0x70006908b0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5b50 sub_5a5b50
    Lifted::new(0x70005a5b90, tpl_522::<0x70006908c0, 0x70006908d0, 0x70006908c8, 0x7000697150, 0x10> as *const (), 0), // +0x5a5b90 sub_5a5b90
    Lifted::new(0x70005a5bd0, tpl_521 as *const (), 1), // +0x5a5bd0 sub_5a5bd0
    Lifted::new(0x70005a5be0, tpl_521 as *const (), 1), // +0x5a5be0 sub_5a5be0
    Lifted::new(0x70005a5bf0, tpl_522::<0x70006908d8, 0x70006908e8, 0x70006908e0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5bf0 sub_5a5bf0
    Lifted::new(0x70005a5c30, tpl_522::<0x70006908f0, 0x7000690900, 0x70006908f8, 0x7000697150, 0x10> as *const (), 0), // +0x5a5c30 sub_5a5c30
    Lifted::new(0x70005a5c70, tpl_521 as *const (), 1), // +0x5a5c70 sub_5a5c70
    Lifted::new(0x70005a5c80, tpl_521 as *const (), 1), // +0x5a5c80 sub_5a5c80
    Lifted::new(0x70005a5c90, tpl_522::<0x7000690908, 0x7000690918, 0x7000690910, 0x7000697150, 0x10> as *const (), 0), // +0x5a5c90 sub_5a5c90
    Lifted::new(0x70005a5cd0, tpl_522::<0x7000690920, 0x7000690930, 0x7000690928, 0x7000697150, 0x10> as *const (), 0), // +0x5a5cd0 sub_5a5cd0
    Lifted::new(0x70005a5d10, tpl_522::<0x7000690938, 0x7000690948, 0x7000690940, 0x7000697150, 0x10> as *const (), 0), // +0x5a5d10 sub_5a5d10
    Lifted::new(0x70005a5d50, tpl_522::<0x7000690950, 0x7000690960, 0x7000690958, 0x7000697150, 0x10> as *const (), 0), // +0x5a5d50 sub_5a5d50
    Lifted::new(0x70005a5d90, tpl_521 as *const (), 1), // +0x5a5d90 sub_5a5d90
    Lifted::new(0x70005a5da0, tpl_522::<0x7000690968, 0x7000690978, 0x7000690970, 0x7000697150, 0x10> as *const (), 0), // +0x5a5da0 sub_5a5da0
    Lifted::new(0x70005a5de0, tpl_522::<0x7000690980, 0x7000690990, 0x7000690988, 0x7000697150, 0x10> as *const (), 0), // +0x5a5de0 sub_5a5de0
    Lifted::new(0x70005a5e20, tpl_522::<0x7000690998, 0x70006909a8, 0x70006909a0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5e20 sub_5a5e20
    Lifted::new(0x70005a5e60, tpl_522::<0x70006909b0, 0x70006909c0, 0x70006909b8, 0x7000697150, 0x10> as *const (), 0), // +0x5a5e60 sub_5a5e60
    Lifted::new(0x70005a5ea0, tpl_522::<0x70006909c8, 0x70006909d8, 0x70006909d0, 0x7000697150, 0x10> as *const (), 0), // +0x5a5ea0 sub_5a5ea0
    Lifted::new(0x70005a5ee0, tpl_522::<0x70006909e0, 0x70006909f0, 0x70006909e8, 0x7000697150, 0x10> as *const (), 0), // +0x5a5ee0 sub_5a5ee0
    Lifted::new(0x70005a5f20, tpl_522::<0x70006909f8, 0x7000690a08, 0x7000690a00, 0x7000697150, 0x10> as *const (), 0), // +0x5a5f20 sub_5a5f20
    Lifted::new(0x70005a5f60, tpl_522::<0x7000690a10, 0x7000690a20, 0x7000690a18, 0x7000697150, 0x10> as *const (), 0), // +0x5a5f60 sub_5a5f60
    Lifted::new(0x70005a5fa0, tpl_521 as *const (), 1), // +0x5a5fa0 sub_5a5fa0
    Lifted::new(0x70005a5fb0, tpl_521 as *const (), 1), // +0x5a5fb0 sub_5a5fb0
    Lifted::new(0x70005a5fc0, tpl_521 as *const (), 1), // +0x5a5fc0 sub_5a5fc0
    Lifted::new(0x70005a5fd0, tpl_521 as *const (), 1), // +0x5a5fd0 sub_5a5fd0
    Lifted::new(0x70005a5fe0, tpl_522::<0x7000690a28, 0x7000690a38, 0x7000690a30, 0x7000697150, 0x10> as *const (), 0), // +0x5a5fe0 sub_5a5fe0
    Lifted::new(0x70005a6020, tpl_522::<0x7000690a40, 0x7000690a50, 0x7000690a48, 0x7000697150, 0x10> as *const (), 0), // +0x5a6020 sub_5a6020
    Lifted::new(0x70005a6060, tpl_521 as *const (), 1), // +0x5a6060 sub_5a6060
    Lifted::new(0x70005a6070, tpl_521 as *const (), 1), // +0x5a6070 sub_5a6070
    Lifted::new(0x70005a6080, tpl_521 as *const (), 1), // +0x5a6080 sub_5a6080
    Lifted::new(0x70005a6090, tpl_521 as *const (), 1), // +0x5a6090 sub_5a6090
    Lifted::new(0x70005a60a0, tpl_521 as *const (), 1), // +0x5a60a0 sub_5a60a0
    Lifted::new(0x70005a60b0, tpl_521 as *const (), 1), // +0x5a60b0 sub_5a60b0
    Lifted::new(0x70005a60c0, tpl_521 as *const (), 1), // +0x5a60c0 sub_5a60c0
    Lifted::new(0x70005a60d0, tpl_521 as *const (), 1), // +0x5a60d0 sub_5a60d0
    Lifted::new(0x70005a60e0, tpl_521 as *const (), 1), // +0x5a60e0 sub_5a60e0
    Lifted::new(0x70005a60f0, tpl_521 as *const (), 1), // +0x5a60f0 sub_5a60f0
    Lifted::new(0x70005a6100, tpl_522::<0x7000690a58, 0x7000690a68, 0x7000690a60, 0x7000697150, 0x10> as *const (), 0), // +0x5a6100 sub_5a6100
    Lifted::new(0x70005a6140, tpl_522::<0x7000690a70, 0x7000690a80, 0x7000690a78, 0x7000697150, 0x10> as *const (), 0), // +0x5a6140 sub_5a6140
    Lifted::new(0x70005a6180, tpl_521 as *const (), 1), // +0x5a6180 sub_5a6180
    Lifted::new(0x70005a6190, tpl_522::<0x7000690a88, 0x7000690a98, 0x7000690a90, 0x7000697150, 0x10> as *const (), 0), // +0x5a6190 sub_5a6190
    Lifted::new(0x70005a61d0, tpl_522::<0x7000690aa0, 0x7000690ab0, 0x7000690aa8, 0x7000697150, 0x10> as *const (), 0), // +0x5a61d0 sub_5a61d0
    Lifted::new(0x70005a6210, tpl_522::<0x7000690ab8, 0x7000690ac8, 0x7000690ac0, 0x7000697150, 0x10> as *const (), 0), // +0x5a6210 sub_5a6210
    Lifted::new(0x70005a6250, tpl_522::<0x7000690ad0, 0x7000690ae0, 0x7000690ad8, 0x7000697150, 0x10> as *const (), 0), // +0x5a6250 sub_5a6250
    Lifted::new(0x70005a6290, tpl_522::<0x7000690ae8, 0x7000690af8, 0x7000690af0, 0x7000697150, 0x10> as *const (), 0), // +0x5a6290 sub_5a6290
    Lifted::new(0x70005a62d0, tpl_522::<0x7000690b00, 0x7000690b10, 0x7000690b08, 0x7000697150, 0x10> as *const (), 0), // +0x5a62d0 sub_5a62d0
    Lifted::new(0x70005a6310, tpl_521 as *const (), 1), // +0x5a6310 sub_5a6310
    Lifted::new(0x70005a6320, tpl_522::<0x7000690b18, 0x7000690b28, 0x7000690b20, 0x7000697150, 0x10> as *const (), 0), // +0x5a6320 sub_5a6320
    Lifted::new(0x70005a6360, tpl_522::<0x7000690b30, 0x7000690b40, 0x7000690b38, 0x7000697150, 0x10> as *const (), 0), // +0x5a6360 sub_5a6360
    Lifted::new(0x70005a63a0, tpl_521 as *const (), 1), // +0x5a63a0 sub_5a63a0
    Lifted::new(0x70005a63b0, tpl_521 as *const (), 1), // +0x5a63b0 sub_5a63b0
    Lifted::new(0x70005a63c0, tpl_521 as *const (), 1), // +0x5a63c0 sub_5a63c0
    Lifted::new(0x70005a63d0, tpl_521 as *const (), 1), // +0x5a63d0 sub_5a63d0
    Lifted::new(0x70005a63e0, tpl_521 as *const (), 1), // +0x5a63e0 sub_5a63e0
    Lifted::new(0x70005a63f0, tpl_521 as *const (), 1), // +0x5a63f0 sub_5a63f0
    Lifted::new(0x70005a6400, tpl_521 as *const (), 1), // +0x5a6400 sub_5a6400
    Lifted::new(0x70005a6410, tpl_521 as *const (), 1), // +0x5a6410 sub_5a6410
    Lifted::new(0x70005a6420, tpl_522::<0x7000690b48, 0x7000690b58, 0x7000690b50, 0x7000697150, 0x10> as *const (), 0), // +0x5a6420 sub_5a6420
    Lifted::new(0x70005a6460, tpl_522::<0x7000690b60, 0x7000690b70, 0x7000690b68, 0x7000697150, 0x10> as *const (), 0), // +0x5a6460 sub_5a6460
    Lifted::new(0x70005a64a0, tpl_522::<0x7000690b78, 0x7000690b88, 0x7000690b80, 0x7000697150, 0x10> as *const (), 0), // +0x5a64a0 sub_5a64a0
    Lifted::new(0x70005a64e0, tpl_522::<0x7000690b90, 0x7000690ba0, 0x7000690b98, 0x7000697150, 0x10> as *const (), 0), // +0x5a64e0 sub_5a64e0
    Lifted::new(0x70005a6520, tpl_522::<0x7000690ba8, 0x7000690bb8, 0x7000690bb0, 0x7000697150, 0x10> as *const (), 0), // +0x5a6520 sub_5a6520
    Lifted::new(0x70005a6560, tpl_522::<0x7000690bc0, 0x7000690bd0, 0x7000690bc8, 0x7000697150, 0x10> as *const (), 0), // +0x5a6560 sub_5a6560
    Lifted::new(0x70005a65a0, tpl_522::<0x7000690bd8, 0x7000690be8, 0x7000690be0, 0x7000697150, 0x10> as *const (), 0), // +0x5a65a0 sub_5a65a0
    Lifted::new(0x70005a65e0, tpl_522::<0x7000690bf0, 0x7000690c00, 0x7000690bf8, 0x7000697150, 0x10> as *const (), 0), // +0x5a65e0 sub_5a65e0
    Lifted::new(0x70005a6620, tpl_522::<0x7000690c08, 0x7000690c18, 0x7000690c10, 0x7000697150, 0x10> as *const (), 0), // +0x5a6620 sub_5a6620
    Lifted::new(0x70005a6660, tpl_522::<0x7000690c20, 0x7000690c30, 0x7000690c28, 0x7000697150, 0x10> as *const (), 0), // +0x5a6660 sub_5a6660
    Lifted::new(0x70005a66a0, tpl_522::<0x7000690c38, 0x7000690c48, 0x7000690c40, 0x7000697150, 0x10> as *const (), 0), // +0x5a66a0 sub_5a66a0
    Lifted::new(0x70005a66e0, tpl_522::<0x7000690c50, 0x7000690c60, 0x7000690c58, 0x7000697150, 0x10> as *const (), 0), // +0x5a66e0 sub_5a66e0
    Lifted::new(0x70005a6720, tpl_521 as *const (), 1), // +0x5a6720 sub_5a6720
    Lifted::new(0x70005a6730, tpl_522::<0x7000690c68, 0x7000690c78, 0x7000690c70, 0x7000697150, 0x10> as *const (), 0), // +0x5a6730 sub_5a6730
    Lifted::new(0x70005a6770, tpl_522::<0x7000690c80, 0x7000690c90, 0x7000690c88, 0x7000697150, 0x10> as *const (), 0), // +0x5a6770 sub_5a6770
    Lifted::new(0x70005a67b0, tpl_521 as *const (), 1), // +0x5a67b0 sub_5a67b0
    Lifted::new(0x70005a67c0, tpl_521 as *const (), 1), // +0x5a67c0 sub_5a67c0
    Lifted::new(0x70005a67d0, tpl_522::<0x7000690c98, 0x7000690ca8, 0x7000690ca0, 0x7000697150, 0x10> as *const (), 0), // +0x5a67d0 sub_5a67d0
    Lifted::new(0x70005a6810, tpl_522::<0x7000690cb0, 0x7000690cc0, 0x7000690cb8, 0x7000697150, 0x10> as *const (), 0), // +0x5a6810 sub_5a6810
    Lifted::new(0x70005a6850, tpl_522::<0x7000690cc8, 0x7000690cd8, 0x7000690cd0, 0x7000697150, 0x10> as *const (), 0), // +0x5a6850 sub_5a6850
    Lifted::new(0x70005a6890, tpl_522::<0x7000690ce0, 0x7000690cf0, 0x7000690ce8, 0x7000697150, 0x10> as *const (), 0), // +0x5a6890 sub_5a6890
    Lifted::new(0x70005a68d0, tpl_522::<0x7000690cf8, 0x7000690d08, 0x7000690d00, 0x7000697150, 0x10> as *const (), 0), // +0x5a68d0 sub_5a68d0
    Lifted::new(0x70005a6910, tpl_522::<0x7000690d10, 0x7000690d20, 0x7000690d18, 0x7000697150, 0x10> as *const (), 0), // +0x5a6910 sub_5a6910
    Lifted::new(0x70005a6950, tpl_521 as *const (), 1), // +0x5a6950 sub_5a6950
    Lifted::new(0x70005a6960, tpl_521 as *const (), 1), // +0x5a6960 sub_5a6960
    Lifted::new(0x70005a6970, tpl_521 as *const (), 1), // +0x5a6970 sub_5a6970
    Lifted::new(0x70005a6980, tpl_521 as *const (), 1), // +0x5a6980 sub_5a6980
    Lifted::new(0x70005a6990, tpl_521 as *const (), 1), // +0x5a6990 sub_5a6990
    Lifted::new(0x70005a69a0, tpl_521 as *const (), 1), // +0x5a69a0 sub_5a69a0
    Lifted::new(0x70005a69b0, tpl_521 as *const (), 1), // +0x5a69b0 sub_5a69b0
    Lifted::new(0x70005a69c0, tpl_521 as *const (), 1), // +0x5a69c0 sub_5a69c0
    Lifted::new(0x70005a69d0, tpl_521 as *const (), 1), // +0x5a69d0 sub_5a69d0
    Lifted::new(0x70005a69e0, tpl_522::<0x7000691070, 0x7000691080, 0x7000691078, 0x7000697150, 0x10> as *const (), 0), // +0x5a69e0 sub_5a69e0
    Lifted::new(0x70005a6a20, tpl_522::<0x7000691088, 0x7000691098, 0x7000691090, 0x7000697150, 0x10> as *const (), 0), // +0x5a6a20 sub_5a6a20
    Lifted::new(0x70005a6a60, tpl_521 as *const (), 1), // +0x5a6a60 sub_5a6a60
    Lifted::new(0x70005a6a70, tpl_521 as *const (), 1), // +0x5a6a70 sub_5a6a70
    Lifted::new(0x70005a6a80, tpl_521 as *const (), 1), // +0x5a6a80 sub_5a6a80
    Lifted::new(0x70005a6a90, tpl_521 as *const (), 1), // +0x5a6a90 sub_5a6a90
    Lifted::new(0x70005a6aa0, tpl_521 as *const (), 1), // +0x5a6aa0 sub_5a6aa0
    Lifted::new(0x70005a6ab0, tpl_522::<0x7000691418, 0x7000691428, 0x7000691420, 0x7000697150, 0x10> as *const (), 0), // +0x5a6ab0 sub_5a6ab0
    Lifted::new(0x70005a6af0, tpl_522::<0x7000691430, 0x7000691440, 0x7000691438, 0x7000697150, 0x10> as *const (), 0), // +0x5a6af0 sub_5a6af0
    Lifted::new(0x70005a6b30, tpl_522::<0x7000691448, 0x7000691458, 0x7000691450, 0x7000697150, 0x10> as *const (), 0), // +0x5a6b30 sub_5a6b30
    Lifted::new(0x70005a6b70, tpl_522::<0x7000691460, 0x7000691470, 0x7000691468, 0x7000697150, 0x10> as *const (), 0), // +0x5a6b70 sub_5a6b70
    Lifted::new(0x70005a6bb0, tpl_521 as *const (), 1), // +0x5a6bb0 sub_5a6bb0
    Lifted::new(0x70005a6bc0, tpl_521 as *const (), 1), // +0x5a6bc0 sub_5a6bc0
    Lifted::new(0x70005a6bd0, tpl_521 as *const (), 1), // +0x5a6bd0 sub_5a6bd0
    Lifted::new(0x70005a6be0, tpl_521 as *const (), 1), // +0x5a6be0 sub_5a6be0
    Lifted::new(0x70005a6bf0, tpl_521 as *const (), 1), // +0x5a6bf0 sub_5a6bf0
    Lifted::new(0x70005a6c00, tpl_521 as *const (), 1), // +0x5a6c00 sub_5a6c00
    Lifted::new(0x70005a6c10, tpl_521 as *const (), 1), // +0x5a6c10 sub_5a6c10
    Lifted::new(0x70005a6c20, tpl_521 as *const (), 1), // +0x5a6c20 sub_5a6c20
    Lifted::new(0x70005a6c30, tpl_522::<0x7000691478, 0x7000691488, 0x7000691480, 0x7000697150, 0x10> as *const (), 0), // +0x5a6c30 sub_5a6c30
    Lifted::new(0x70005a6c70, tpl_522::<0x7000691490, 0x70006914a0, 0x7000691498, 0x7000697150, 0x10> as *const (), 0), // +0x5a6c70 sub_5a6c70
    Lifted::new(0x70005a6cb0, tpl_522::<0x70006914a8, 0x70006914b8, 0x70006914b0, 0x7000697150, 0x10> as *const (), 0), // +0x5a6cb0 sub_5a6cb0
    Lifted::new(0x70005a6cf0, tpl_522::<0x70006914c0, 0x70006914d0, 0x70006914c8, 0x7000697150, 0x10> as *const (), 0), // +0x5a6cf0 sub_5a6cf0
    Lifted::new(0x70005a6d30, tpl_521 as *const (), 1), // +0x5a6d30 sub_5a6d30
    Lifted::new(0x70005a6d40, tpl_521 as *const (), 1), // +0x5a6d40 sub_5a6d40
    Lifted::new(0x70005a6d50, tpl_521 as *const (), 1), // +0x5a6d50 sub_5a6d50
    Lifted::new(0x70005a6d60, tpl_521 as *const (), 1), // +0x5a6d60 sub_5a6d60
    Lifted::new(0x70005a6d70, tpl_522::<0x70006914f0, 0x7000691500, 0x70006914f8, 0x7000697150, 0x10> as *const (), 0), // +0x5a6d70 sub_5a6d70
    Lifted::new(0x70005a6db0, tpl_522::<0x7000691508, 0x7000691518, 0x7000691510, 0x7000697150, 0x10> as *const (), 0), // +0x5a6db0 sub_5a6db0
    Lifted::new(0x70005a6df0, fn_70005a6df0 as *const (), 1), // +0x5a6df0 sub_5a6df0
    Lifted::new(0x70005a6e00, fn_70005a6e00 as *const (), 0), // +0x5a6e00 sub_5a6e00
    Lifted::new(0x70005a6e50, fn_70005a6e50 as *const (), 0), // +0x5a6e50 sub_5a6e50
    Lifted::new(0x70005a6eb0, tpl_524::<0x70006915f8, 0x7000697150, 0x70006915e8, 0x10> as *const (), 0), // +0x5a6eb0 sub_5a6eb0
    Lifted::new(0x70005a6f00, tpl_524::<0x7000691610, 0x7000697150, 0x7000691600, 0x10> as *const (), 0), // +0x5a6f00 sub_5a6f00
    Lifted::new(0x70005a6f50, tpl_524::<0x7000691638, 0x7000697150, 0x7000691618, 0x10> as *const (), 0), // +0x5a6f50 sub_5a6f50
    Lifted::new(0x70005a6fa0, tpl_524::<0x700069164c, 0x7000697150, 0x7000691640, 0x10> as *const (), 0), // +0x5a6fa0 sub_5a6fa0
    Lifted::new(0x70005a6ff0, tpl_525::<0x70006971f0, 0x3c, 0x7000697150, 0x30, 0x10, 0x70006971e8, 0x70001a3c50> as *const (), 0), // +0x5a6ff0 sub_5a6ff0
    Lifted::new(0x70005a7050, tpl_525::<0x7000697208, 0x3c, 0x7000697150, 0x30, 0x10, 0x7000697200, 0x70001a3c50> as *const (), 0), // +0x5a7050 sub_5a7050
    Lifted::new(0x70005a70b0, tpl_526::<0x700069721c, 0x7000697150, 0x7000697210, 0x10> as *const (), 0), // +0x5a70b0 sub_5a70b0
    Lifted::new(0x70005a70f0, tpl_526::<0x700069722c, 0x7000697150, 0x7000697220, 0x10> as *const (), 0), // +0x5a70f0 sub_5a70f0
    Lifted::new(0x70005a7130, tpl_526::<0x700069723c, 0x7000697150, 0x7000697230, 0x10> as *const (), 0), // +0x5a7130 sub_5a7130
    Lifted::new(0x70005a7170, tpl_526::<0x700069724c, 0x7000697150, 0x7000697240, 0x10> as *const (), 0), // +0x5a7170 sub_5a7170
    Lifted::new(0x70005a72c0, tpl_524::<0x7000691734, 0x7000697150, 0x7000691728, 0x10> as *const (), 0), // +0x5a72c0 sub_5a72c0
    Lifted::new(0x70005a7310, fn_70005a7310 as *const (), 0), // +0x5a7310 sub_5a7310
    Lifted::new(0x70005a7370, fn_70005a7370 as *const (), 0), // +0x5a7370 sub_5a7370
    Lifted::new(0x70005a73d0, tpl_524::<0x70006917b4, 0x7000697150, 0x70006917a0, 0x10> as *const (), 0), // +0x5a73d0 sub_5a73d0
    Lifted::new(0x70005a7420, tpl_524::<0x70006917c4, 0x7000697150, 0x70006917b8, 0x10> as *const (), 0), // +0x5a7420 sub_5a7420
    Lifted::new(0x70005a7470, tpl_524::<0x70006917d4, 0x7000697150, 0x70006917c8, 0x10> as *const (), 0), // +0x5a7470 sub_5a7470
    Lifted::new(0x70005a74c0, tpl_527::<0x70006917e4, 0x7000697150, 0x70006917d8, 0x10> as *const (), 0), // +0x5a74c0 sub_5a74c0
    Lifted::new(0x70005a7510, tpl_527::<0x70006917f4, 0x7000697150, 0x70006917e8, 0x10> as *const (), 0), // +0x5a7510 sub_5a7510
    Lifted::new(0x70005a7560, tpl_528::<0x7000697454, 0x7000697150, 0x7000697448, 0x10> as *const (), 0), // +0x5a7560 sub_5a7560
    Lifted::new(0x70005a75a0, tpl_527::<0x7000691804, 0x7000697150, 0x70006917f8, 0x10> as *const (), 0), // +0x5a75a0 sub_5a75a0
    Lifted::new(0x70005a75f0, fn_70005a75f0 as *const (), 0), // +0x5a75f0 sub_5a75f0
    Lifted::new(0x70005a7640, tpl_524::<0x7000691834, 0x7000697150, 0x7000691820, 0x10> as *const (), 0), // +0x5a7640 sub_5a7640
    Lifted::new(0x70005a7690, tpl_524::<0x7000691844, 0x7000697150, 0x7000691838, 0x10> as *const (), 0), // +0x5a7690 sub_5a7690
    Lifted::new(0x70005a76e0, tpl_528::<0x7000697464, 0x7000697150, 0x7000697458, 0x10> as *const (), 0), // +0x5a76e0 sub_5a76e0
    Lifted::new(0x70005a77e0, tpl_529::<0x7000697474, 0x7000697150, 0x7000697468, 0x10> as *const (), 0), // +0x5a77e0 sub_5a77e0
    Lifted::new(0x70005a7860, tpl_529::<0x7000697484, 0x7000697150, 0x7000697478, 0x10> as *const (), 0), // +0x5a7860 sub_5a7860
    Lifted::new(0x70005a78c0, tpl_521 as *const (), 1), // +0x5a78c0 sub_5a78c0
    Lifted::new(0x70005a78d0, tpl_521 as *const (), 1), // +0x5a78d0 sub_5a78d0
    Lifted::new(0x70005a78e0, tpl_521 as *const (), 1), // +0x5a78e0 sub_5a78e0
    Lifted::new(0x70005a78f0, tpl_521 as *const (), 1), // +0x5a78f0 sub_5a78f0
    Lifted::new(0x70005a7900, tpl_522::<0x7000692730, 0x7000692740, 0x7000692738, 0x7000697150, 0x10> as *const (), 0), // +0x5a7900 sub_5a7900
    Lifted::new(0x70005a7940, fn_70005a7940 as *const (), 0), // +0x5a7940 sub_5a7940
    Lifted::new(0x70005a79e0, tpl_522::<0x7000692888, 0x7000692898, 0x7000692890, 0x7000697150, 0x10> as *const (), 0), // +0x5a79e0 sub_5a79e0
    Lifted::new(0x70005a7a20, tpl_522::<0x70006928a0, 0x70006928b0, 0x70006928a8, 0x7000697150, 0x10> as *const (), 0), // +0x5a7a20 sub_5a7a20
    Lifted::new(0x70005a7a60, tpl_522::<0x70006928b8, 0x70006928c8, 0x70006928c0, 0x7000697150, 0x10> as *const (), 0), // +0x5a7a60 sub_5a7a60
    Lifted::new(0x70005a7aa0, tpl_522::<0x70006928d0, 0x70006928e0, 0x70006928d8, 0x7000697150, 0x10> as *const (), 0), // +0x5a7aa0 sub_5a7aa0
    Lifted::new(0x70005a7ae0, tpl_521 as *const (), 1), // +0x5a7ae0 sub_5a7ae0
    Lifted::new(0x70005a7af0, tpl_521 as *const (), 1), // +0x5a7af0 sub_5a7af0
    Lifted::new(0x70005a7b00, tpl_521 as *const (), 1), // +0x5a7b00 sub_5a7b00
    Lifted::new(0x70005a7b10, tpl_522::<0x70006928e8, 0x70006928f8, 0x70006928f0, 0x7000697150, 0x10> as *const (), 0), // +0x5a7b10 sub_5a7b10
    Lifted::new(0x70005a7b50, tpl_522::<0x7000692900, 0x7000692910, 0x7000692908, 0x7000697150, 0x10> as *const (), 0), // +0x5a7b50 sub_5a7b50
    Lifted::new(0x70005a7b90, tpl_522::<0x7000692918, 0x7000692928, 0x7000692920, 0x7000697150, 0x10> as *const (), 0), // +0x5a7b90 sub_5a7b90
    Lifted::new(0x70005a7bd0, fn_70005a7bd0 as *const (), 0), // +0x5a7bd0 sub_5a7bd0
    Lifted::new(0x70005a7c20, tpl_522::<0x7000692970, 0x7000692980, 0x7000692978, 0x7000697150, 0x10> as *const (), 0), // +0x5a7c20 sub_5a7c20
    Lifted::new(0x70005a7c60, tpl_522::<0x7000692988, 0x7000692998, 0x7000692990, 0x7000697150, 0x10> as *const (), 0), // +0x5a7c60 sub_5a7c60
    Lifted::new(0x70005a7ca0, tpl_521 as *const (), 1), // +0x5a7ca0 sub_5a7ca0
    Lifted::new(0x70005a7cb0, tpl_521 as *const (), 1), // +0x5a7cb0 sub_5a7cb0
    Lifted::new(0x70005a7cc0, tpl_522::<0x70006929a0, 0x70006929b0, 0x70006929a8, 0x7000697150, 0x10> as *const (), 0), // +0x5a7cc0 sub_5a7cc0
    Lifted::new(0x70005a7d00, tpl_523::<0x70006929b8, 0x70006929c8, 0x70006929c0, 0x7000697150, 0x10, 0x70005a7d34, 0x70005a7d35, 0x70005a7d36, 0x70005a7d37, 0x70005a7d38, 0x70005a7d39, 0x70005a7d3a, 0x70005a7d3b, 0x70005a7d3c, 0x70005a7d3d, 0x70005a7d3e, 0x70005a7d3f, 0x70006188d8, 0x70006929d0> as *const (), 0), // +0x5a7d00 sub_5a7d00
    Lifted::new(0x70005a7d50, tpl_521 as *const (), 1), // +0x5a7d50 sub_5a7d50
];
