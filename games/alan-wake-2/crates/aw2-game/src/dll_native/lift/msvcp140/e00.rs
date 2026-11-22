#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x7001e15000, fn_7001e15000 as *const (), 0), // +0x1000 sub_1000
    Lifted::new(0x7001e15050, fn_7001e15050 as *const (), 0), // +0x1050 sub_1050
    Lifted::new(0x7001e150a0, tpl_0::<0x7001e98c20, 0x7001e98bb0, 0x7001e17bf0, 0x7001e67ad0, 0x7001e657dc> as *const (), 0), // +0x10a0 sub_10a0
    Lifted::new(0x7001e150d0, tpl_1::<0x7001e69588, 0x7001e98c20, 0x7001e17c90, 0x7001e6bd28, 0x7001e1a548, 0x7001e67b30, 0x7001e657dc> as *const (), 0), // +0x10d0 sub_10d0
    Lifted::new(0x7001e15130, tpl_2::<0x7001e98bb0, 0x7001e990d0, 0x4, 0x7001e990c8, 0x50, 0x18> as *const (), 1), // +0x1130 sub_1130
    Lifted::new(0x7001e15170, tpl_3::<0x7001e98ba0, 0x7001e49dd0, 0x7001e67b40, 0x7001e657dc> as *const (), 0), // +0x1170 sub_1170
    Lifted::new(0x7001e15190, tpl_0::<0x7001e98cf0, 0x7001e98d90, 0x7001e1d8e0, 0x7001e67b50, 0x7001e657dc> as *const (), 0), // +0x1190 sub_1190
    Lifted::new(0x7001e151c0, fn_7001e151c0 as *const (), 0), // +0x11c0 sub_11c0
    Lifted::new(0x7001e15210, tpl_4::<0x7001e98d90, 0x7001e990c0, 0x4, 0x7001e990c8, 0x50> as *const (), 1), // +0x1210 sub_1210
    Lifted::new(0x7001e15240, tpl_3::<0x7001e98ce0, 0x7001e49dd0, 0x7001e67bc0, 0x7001e657dc> as *const (), 0), // +0x1240 sub_1240
    Lifted::new(0x7001e15260, tpl_0::<0x7001e98e20, 0x7001e98ec0, 0x7001e17bf0, 0x7001e67bd0, 0x7001e657dc> as *const (), 0), // +0x1260 sub_1260
    Lifted::new(0x7001e15290, tpl_1::<0x7001e69588, 0x7001e98e20, 0x7001e17c90, 0x7001e6bd28, 0x7001e1a548, 0x7001e67c30, 0x7001e657dc> as *const (), 0), // +0x1290 sub_1290
    Lifted::new(0x7001e152f0, tpl_5::<0x7001e98ec0, 0x7001e990d8> as *const (), 1), // +0x12f0 sub_12f0
    Lifted::new(0x7001e15300, tpl_3::<0x7001e98e10, 0x7001e49dd0, 0x7001e67c40, 0x7001e657dc> as *const (), 0), // +0x1300 sub_1300
    Lifted::new(0x7001e15320, tpl_0::<0x7001e98fb0, 0x7001e98f40, 0x7001e17bf0, 0x7001e67c50, 0x7001e657dc> as *const (), 0), // +0x1320 sub_1320
    Lifted::new(0x7001e15350, fn_7001e15350 as *const (), 0), // +0x1350 sub_1350
    Lifted::new(0x7001e153b0, tpl_6::<0x7001e990c0, 0x7001e98f40, 0x7001e990c8, 0x4, 0x50, 0x7001e990d0> as *const (), 1), // +0x13b0 sub_13b0
    Lifted::new(0x7001e15400, tpl_3::<0x7001e98f30, 0x7001e49dd0, 0x7001e67cc0, 0x7001e657dc> as *const (), 0), // +0x1400 sub_1400
    Lifted::new(0x7001e15420, tpl_3::<0x7001e99100, 0x7001e49dd0, 0x7001e67d10, 0x7001e657dc> as *const (), 0), // +0x1420 sub_1420
    Lifted::new(0x7001e15440, tpl_7::<0x7001e67cd0, 0x7001e657dc> as *const (), 0), // +0x1440 sub_1440
    Lifted::new(0x7001e15450, tpl_7::<0x7001e67d80, 0x7001e657dc> as *const (), 0), // +0x1450 sub_1450
    Lifted::new(0x7001e15460, tpl_7::<0x7001e67d20, 0x7001e657dc> as *const (), 0), // +0x1460 sub_1460
    Lifted::new(0x7001e15470, tpl_3::<0x7001e99214, 0x7001e49dd0, 0x7001e67dc0, 0x7001e657dc> as *const (), 0), // +0x1470 sub_1470
    Lifted::new(0x7001e15490, tpl_2::<0x7001e99570, 0x7001e99a08, 0x4, 0x7001e99a00, 0x50, 0x18> as *const (), 1), // +0x1490 sub_1490
    Lifted::new(0x7001e154d0, tpl_3::<0x7001e99560, 0x7001e49dd0, 0x7001e67dd0, 0x7001e657dc> as *const (), 0), // +0x14d0 sub_14d0
    Lifted::new(0x7001e154f0, tpl_0::<0x7001e995e0, 0x7001e99570, 0x7001e272a0, 0x7001e67de0, 0x7001e657dc> as *const (), 0), // +0x14f0 sub_14f0
    Lifted::new(0x7001e15520, tpl_8::<0x7001e69588, 0x7001e995e0, 0x7001e271c8, 0x7001e67e40, 0x7001e657dc> as *const (), 0), // +0x1520 sub_1520
    Lifted::new(0x7001e15550, tpl_4::<0x7001e99740, 0x7001e999f8, 0x4, 0x7001e99a00, 0x50> as *const (), 1), // +0x1550 sub_1550
    Lifted::new(0x7001e15580, tpl_3::<0x7001e99690, 0x7001e49dd0, 0x7001e67e50, 0x7001e657dc> as *const (), 0), // +0x1580 sub_1580
    Lifted::new(0x7001e155a0, tpl_0::<0x7001e996a0, 0x7001e99740, 0x7001e28510, 0x7001e67e60, 0x7001e657dc> as *const (), 0), // +0x15a0 sub_15a0
    Lifted::new(0x7001e155d0, tpl_9::<0x7001e69588, 0x7001e996a0, 0x7001e271c8, 0x7001e67ec0, 0x7001e657dc> as *const (), 0), // +0x15d0 sub_15d0
    Lifted::new(0x7001e15600, tpl_5::<0x7001e997d0, 0x7001e99a10> as *const (), 1), // +0x1600 sub_1600
    Lifted::new(0x7001e15610, tpl_3::<0x7001e997c0, 0x7001e49dd0, 0x7001e67ed0, 0x7001e657dc> as *const (), 0), // +0x1610 sub_1610
    Lifted::new(0x7001e15630, tpl_0::<0x7001e99840, 0x7001e997d0, 0x7001e272a0, 0x7001e67ee0, 0x7001e657dc> as *const (), 0), // +0x1630 sub_1630
    Lifted::new(0x7001e15660, tpl_8::<0x7001e69588, 0x7001e99840, 0x7001e271c8, 0x7001e67f40, 0x7001e657dc> as *const (), 0), // +0x1660 sub_1660
    Lifted::new(0x7001e15690, tpl_6::<0x7001e999f8, 0x7001e998f0, 0x7001e99a00, 0x4, 0x50, 0x7001e99a08> as *const (), 1), // +0x1690 sub_1690
    Lifted::new(0x7001e156e0, tpl_3::<0x7001e998e0, 0x7001e49dd0, 0x7001e67f50, 0x7001e657dc> as *const (), 0), // +0x16e0 sub_16e0
    Lifted::new(0x7001e15700, tpl_0::<0x7001e99960, 0x7001e998f0, 0x7001e272a0, 0x7001e67f60, 0x7001e657dc> as *const (), 0), // +0x1700 sub_1700
    Lifted::new(0x7001e15730, tpl_10::<0x7001e69588, 0x7001e99960, 0x7001e271c8, 0x7001e67fc0, 0x7001e657dc> as *const (), 0), // +0x1730 sub_1730
    Lifted::new(0x7001e15760, tpl_2::<0x7001e99a30, 0x7001e990f0, 0x4, 0x7001e990e8, 0x50, 0x18> as *const (), 1), // +0x1760 sub_1760
    Lifted::new(0x7001e157a0, tpl_3::<0x7001e99a20, 0x7001e49dd0, 0x7001e67fd0, 0x7001e657dc> as *const (), 0), // +0x17a0 sub_17a0
    Lifted::new(0x7001e157c0, tpl_0::<0x7001e99aa0, 0x7001e99a30, 0x7001e289b0, 0x7001e67fe0, 0x7001e657dc> as *const (), 0), // +0x17c0 sub_17c0
    Lifted::new(0x7001e157f0, tpl_8::<0x7001e69588, 0x7001e99aa0, 0x7001e288d8, 0x7001e68040, 0x7001e657dc> as *const (), 0), // +0x17f0 sub_17f0
    Lifted::new(0x7001e15820, tpl_4::<0x7001e99c00, 0x7001e990e0, 0x4, 0x7001e990e8, 0x50> as *const (), 1), // +0x1820 sub_1820
    Lifted::new(0x7001e15850, tpl_3::<0x7001e99b50, 0x7001e49dd0, 0x7001e68050, 0x7001e657dc> as *const (), 0), // +0x1850 sub_1850
    Lifted::new(0x7001e15870, tpl_0::<0x7001e99b60, 0x7001e99c00, 0x7001e29040, 0x7001e68060, 0x7001e657dc> as *const (), 0), // +0x1870 sub_1870
    Lifted::new(0x7001e158a0, tpl_9::<0x7001e69588, 0x7001e99b60, 0x7001e288d8, 0x7001e680c0, 0x7001e657dc> as *const (), 0), // +0x18a0 sub_18a0
    Lifted::new(0x7001e158d0, tpl_5::<0x7001e99c90, 0x7001e990f8> as *const (), 1), // +0x18d0 sub_18d0
    Lifted::new(0x7001e158e0, tpl_3::<0x7001e99c80, 0x7001e49dd0, 0x7001e680d0, 0x7001e657dc> as *const (), 0), // +0x18e0 sub_18e0
    Lifted::new(0x7001e15900, tpl_0::<0x7001e99d00, 0x7001e99c90, 0x7001e289b0, 0x7001e680e0, 0x7001e657dc> as *const (), 0), // +0x1900 sub_1900
    Lifted::new(0x7001e15930, tpl_8::<0x7001e69588, 0x7001e99d00, 0x7001e288d8, 0x7001e68140, 0x7001e657dc> as *const (), 0), // +0x1930 sub_1930
    Lifted::new(0x7001e15960, tpl_6::<0x7001e990e0, 0x7001e99db0, 0x7001e990e8, 0x4, 0x50, 0x7001e990f0> as *const (), 1), // +0x1960 sub_1960
    Lifted::new(0x7001e159b0, tpl_3::<0x7001e99da0, 0x7001e49dd0, 0x7001e68150, 0x7001e657dc> as *const (), 0), // +0x19b0 sub_19b0
    Lifted::new(0x7001e159d0, tpl_0::<0x7001e99e20, 0x7001e99db0, 0x7001e289b0, 0x7001e68160, 0x7001e657dc> as *const (), 0), // +0x19d0 sub_19d0
    Lifted::new(0x7001e15a00, tpl_10::<0x7001e69588, 0x7001e99e20, 0x7001e288d8, 0x7001e681c0, 0x7001e657dc> as *const (), 0), // +0x1a00 sub_1a00
    Lifted::new(0x7001e15a30, tpl_3::<0x7001e9a0a0, 0x7001e49dd0, 0x7001e681d0, 0x7001e657dc> as *const (), 0), // +0x1a30 sub_1a30
    Lifted::new(0x7001e15a50, fn_7001e15a50 as *const (), 2), // +0x1a50 sub_1a50
    Lifted::new(0x7001e15a94, fn_7001e15a94 as *const (), 3), // +0x1a94 sub_1a94
    Lifted::new(0x7001e15ac4, fn_7001e15ac4 as *const (), 2), // +0x1ac4 sub_1ac4
    Lifted::new(0x7001e15b08, fn_7001e15b08 as *const (), 3), // +0x1b08 sub_1b08
    Lifted::new(0x7001e15b38, fn_7001e15b38 as *const (), 2), // +0x1b38 sub_1b38
    Lifted::new(0x7001e15b6c, fn_7001e15b6c as *const (), 0), // +0x1b6c sub_1b6c
    Lifted::new(0x7001e15b9c, fn_7001e15b9c as *const (), 0), // +0x1b9c sub_1b9c
    Lifted::new(0x7001e15bec, fn_7001e15bec as *const (), 0), // +0x1bec sub_1bec
    Lifted::new(0x7001e15c10, fn_7001e15c10 as *const (), 0), // +0x1c10 sub_1c10
    Lifted::new(0x7001e15c30, fn_7001e15c30 as *const (), 2), // +0x1c30 sub_1c30
    Lifted::new(0x7001e15cc4, fn_7001e15cc4 as *const (), 2), // +0x1cc4 sub_1cc4
    Lifted::new(0x7001e15d60, fn_7001e15d60 as *const (), 0), // +0x1d60 sub_1d60
    Lifted::new(0x7001e15db4, fn_7001e15db4 as *const (), 0), // +0x1db4 sub_1db4
    Lifted::new(0x7001e15e30, fn_7001e15e30 as *const (), 0), // +0x1e30 sub_1e30
    Lifted::new(0x7001e15e60, fn_7001e15e60 as *const (), 0), // +0x1e60 sub_1e60
    Lifted::new(0x7001e15eb0, fn_7001e15eb0 as *const (), 0), // +0x1eb0 sub_1eb0
    Lifted::new(0x7001e15ef4, fn_7001e15ef4 as *const (), 2), // +0x1ef4 sub_1ef4
    Lifted::new(0x7001e160dc, fn_7001e160dc as *const (), 2), // +0x20dc sub_20dc
    Lifted::new(0x7001e16234, fn_7001e16234 as *const (), 2), // +0x2234 sub_2234
    Lifted::new(0x7001e16238, fn_7001e16238 as *const (), 0), // +0x2238 sub_2238
    Lifted::new(0x7001e16244, fn_7001e16244 as *const (), 0), // +0x2244 sub_2244
    Lifted::new(0x7001e16254, fn_7001e16254 as *const (), 0), // +0x2254 sub_2254
    Lifted::new(0x7001e162f0, tpl_11 as *const (), 1), // +0x22f0 ?ReportUnhandledError@_ExceptionHolder@details@Concurrency@@AEAAXXZ
    Lifted::new(0x7001e16300, fn_7001e16300 as *const (), 0), // +0x2300 sub_2300
    Lifted::new(0x7001e16310, fn_7001e16310 as *const (), 0), // +0x2310 sub_2310
    Lifted::new(0x7001e163a0, fn_7001e163a0 as *const (), 1), // +0x23a0 ?do_encoding@?$codecvt@_SDU_Mbstatet@@@std@@MEBAHXZ
    Lifted::new(0x7001e163a4, fn_7001e163a4 as *const (), 0), // +0x23a4 sub_23a4
    Lifted::alias(0x7001e16440, fn_7001e15cc4 as *const (), 2, 0x1cc4), // +0x2440 ?__ExceptionPtrAssign@@YAXPEAXPEBX@Z (thunk -> +0x1cc4)
    Lifted::new(0x7001e16450, fn_7001e16450 as *const (), 1), // +0x2450 ?__ExceptionPtrCompare@@YA_NPEBX0@Z
    Lifted::new(0x7001e16460, fn_7001e16460 as *const (), 1), // +0x2460 ?__ExceptionPtrCopy@@YAXPEAXPEBX@Z
    Lifted::new(0x7001e16490, fn_7001e16490 as *const (), 0), // +0x2490 ?__ExceptionPtrCopyException@@YAXPEAXPEBX1@Z
    Lifted::new(0x7001e164c0, fn_7001e164c0 as *const (), 1), // +0x24c0 ?__ExceptionPtrCreate@@YAXPEAX@Z
    Lifted::new(0x7001e164d0, fn_7001e164d0 as *const (), 0), // +0x24d0 ?__ExceptionPtrCurrentException@@YAXPEAX@Z
    Lifted::new(0x7001e16550, tpl_13::<0x7001e15db4> as *const (), 0), // +0x2550 ?__ExceptionPtrDestroy@@YAXPEAX@Z
    Lifted::new(0x7001e16560, fn_7001e16560 as *const (), 2), // +0x2560 ?__ExceptionPtrRethrow@@YAXPEBX@Z
    Lifted::new(0x7001e166f0, fn_7001e166f0 as *const (), 1), // +0x26f0 ?__ExceptionPtrSwap@@YAXPEAX0@Z
    Lifted::new(0x7001e16710, fn_7001e16710 as *const (), 1), // +0x2710 ?__ExceptionPtrToBool@@YA_NPEBX@Z
    Lifted::new(0x7001e16720, fn_7001e16720 as *const (), 1), // +0x2720 sub_2720
    Lifted::new(0x7001e16740, fn_7001e16740 as *const (), 0), // +0x2740 __crtCompareStringA
    Lifted::new(0x7001e16ab0, fn_7001e16ab0 as *const (), 0), // +0x2ab0 __crtCompareStringW
    Lifted::new(0x7001e16b70, fn_7001e16b70 as *const (), 0), // +0x2b70 __crtLCMapStringA
    Lifted::new(0x7001e16e70, fn_7001e16e70 as *const (), 0), // +0x2e70 __crtLCMapStringW
    Lifted::new(0x7001e16f00, tpl_14::<0x7001e69010> as *const (), 0), // +0x2f00 __crtCompareStringEx
    Lifted::new(0x7001e16f40, tpl_15::<0x7001e69040> as *const (), 2), // +0x2f40 __crtGetLocaleInfoEx
    Lifted::new(0x7001e16f50, tpl_14::<0x7001e69038> as *const (), 0), // +0x2f50 __crtLCMapStringEx
    Lifted::new(0x7001e16f90, fn_7001e16f90 as *const (), 2), // +0x2f90 _Getctype
    Lifted::new(0x7001e17060, fn_7001e17060 as *const (), 0), // +0x3060 _Tolower
    Lifted::new(0x7001e171a0, fn_7001e171a0 as *const (), 0), // +0x31a0 _Toupper
    Lifted::new(0x7001e172f0, tpl_16::<0x7001e98b90, 0x7001e69050> as *const (), 0), // +0x32f0 _Lock_shared_ptr_spin_lock
    Lifted::new(0x7001e17300, tpl_16::<0x7001e98b90, 0x7001e69048> as *const (), 0), // +0x3300 _Unlock_shared_ptr_spin_lock
    Lifted::new(0x7001e17310, fn_7001e17310 as *const (), 0), // +0x3310 sub_3310
    Lifted::new(0x7001e1734c, fn_7001e1734c as *const (), 0), // +0x334c sub_334c
    Lifted::new(0x7001e17424, fn_7001e17424 as *const (), 0), // +0x3424 sub_3424
    Lifted::new(0x7001e174fc, fn_7001e174fc as *const (), 0), // +0x34fc sub_34fc
    Lifted::new(0x7001e175dc, fn_7001e175dc as *const (), 2), // +0x35dc sub_35dc
    Lifted::new(0x7001e176a4, fn_7001e176a4 as *const (), 0), // +0x36a4 sub_36a4
    Lifted::new(0x7001e177fc, fn_7001e177fc as *const (), 0), // +0x37fc sub_37fc
    Lifted::new(0x7001e1793c, fn_7001e1793c as *const (), 0), // +0x393c sub_393c
    Lifted::new(0x7001e17a54, fn_7001e17a54 as *const (), 0), // +0x3a54 sub_3a54
    Lifted::new(0x7001e17b70, fn_7001e17b70 as *const (), 0), // +0x3b70 ??0?$_Yarn@D@std@@QEAA@PEBD@Z
    Lifted::new(0x7001e17b90, fn_7001e17b90 as *const (), 1), // +0x3b90 ??0?$_Yarn@D@std@@QEAA@XZ
    Lifted::new(0x7001e17ba0, fn_7001e17ba0 as *const (), 1), // +0x3ba0 ??0?$_Yarn@G@std@@QEAA@XZ
    Lifted::new(0x7001e17bb0, fn_7001e17bb0 as *const (), 1), // +0x3bb0 ??0?$basic_ios@DU?$char_traits@D@std@@@std@@IEAA@XZ
    Lifted::new(0x7001e17bf0, fn_7001e17bf0 as *const (), 0), // +0x3bf0 ??0?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N@Z
    Lifted::new(0x7001e17c90, fn_7001e17c90 as *const (), 0), // +0x3c90 ??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@XZ
    Lifted::new(0x7001e17d80, tpl_17::<0x7001e6bda8, 0x8> as *const (), 1), // +0x3d80 ??0?$codecvt@DDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e17da0, tpl_18::<0x7001e6ba18, 0x8, 0x7001e4d900, 0x10, 0x20, 0x30, 0x28, 0x38> as *const (), 2), // +0x3da0 ??0?$codecvt@GDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e17df0, fn_7001e17df0 as *const (), 2), // +0x3df0 ??0?$codecvt@GDU_Mbstatet@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e17e60, tpl_19::<0x8, 0x7001e6b910, 0x10, 0x14> as *const (), 1), // +0x3e60 ??0?$codecvt@_SDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@KW4_Codecvt_mode@1@_K@Z
    Lifted::new(0x7001e17e80, fn_7001e17e80 as *const (), 1), // +0x3e80 ??0?$codecvt@_SDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e17ea0, fn_7001e17ea0 as *const (), 0), // +0x3ea0 ??0?$codecvt@_SDU_Mbstatet@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e17f00, tpl_19::<0x8, 0x7001e6b968, 0x10, 0x14> as *const (), 1), // +0x3f00 ??0?$codecvt@_UDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@KW4_Codecvt_mode@1@_K@Z
    Lifted::new(0x7001e17f20, fn_7001e17f20 as *const (), 1), // +0x3f20 ??0?$codecvt@_UDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e17f40, fn_7001e17f40 as *const (), 0), // +0x3f40 ??0?$codecvt@_UDU_Mbstatet@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e17f90, tpl_18::<0x7001e6b9c0, 0x8, 0x7001e4d900, 0x10, 0x20, 0x30, 0x28, 0x38> as *const (), 2), // +0x3f90 ??0?$codecvt@_WDU_Mbstatet@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e17fe0, fn_7001e17fe0 as *const (), 2), // +0x3fe0 ??0?$codecvt@_WDU_Mbstatet@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e18050, fn_7001e18050 as *const (), 2), // +0x4050 ??0?$ctype@D@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e18090, fn_7001e18090 as *const (), 2), // +0x4090 ??0?$ctype@D@std@@QEAA@PEBF_N_K@Z
    Lifted::new(0x7001e18160, tpl_20::<0x7001e6bb70, 0x8, 0x7001e16f90, 0x10, 0x20, 0x7001e4d900, 0x30, 0x40, 0x50, 0x28, 0x58> as *const (), 2), // +0x4160 ??0?$ctype@G@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e181d0, fn_7001e181d0 as *const (), 2), // +0x41d0 ??0?$ctype@G@std@@QEAA@_K@Z
    Lifted::new(0x7001e18260, tpl_20::<0x7001e6baf0, 0x8, 0x7001e16f90, 0x10, 0x20, 0x7001e4d900, 0x30, 0x40, 0x50, 0x28, 0x58> as *const (), 2), // +0x4260 ??0?$ctype@_W@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e182d0, fn_7001e182d0 as *const (), 2), // +0x42d0 ??0?$ctype@_W@std@@QEAA@_K@Z
    Lifted::new(0x7001e18360, tpl_21::<0x7001e1f1a0> as *const (), 0), // +0x4360 ??0Init@ios_base@std@@QEAA@XZ
    Lifted::new(0x7001e18380, fn_7001e18380 as *const (), 1), // +0x4380 ??0_Facet_base@std@@QEAA@AEBV01@@Z
    Lifted::new(0x7001e18390, fn_7001e18390 as *const (), 0), // +0x4390 ??0_Locimp@locale@std@@AEAA@AEBV012@@Z
    Lifted::new(0x7001e18410, fn_7001e18410 as *const (), 0), // +0x4410 ??0_Locimp@locale@std@@AEAA@_N@Z
    Lifted::new(0x7001e18460, fn_7001e18460 as *const (), 0), // +0x4460 ??0_Locinfo@std@@QEAA@HPEBD@Z
    Lifted::new(0x7001e184f0, fn_7001e184f0 as *const (), 0), // +0x44f0 ??0_Locinfo@std@@QEAA@PEBD@Z
    Lifted::new(0x7001e18568, fn_7001e18568 as *const (), 2), // +0x4568 sub_4568
    Lifted::new(0x7001e18678, tpl_22::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b4d0, 0x18> as *const (), 2), // +0x4678 sub_4678
    Lifted::new(0x7001e186d0, fn_7001e186d0 as *const (), 0), // +0x46d0 ??0_Timevec@std@@QEAA@AEBV01@@Z
    Lifted::new(0x7001e18710, fn_7001e18710 as *const (), 1), // +0x4710 ??0_Timevec@std@@QEAA@PEAX@Z
    Lifted::new(0x7001e18718, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b478> as *const (), 2), // +0x4718 sub_4718
    Lifted::new(0x7001e18754, tpl_24::<0x10, 0x7001e6b488, 0x8, 0x7001e6b478> as *const (), 1), // +0x4754 sub_4754
    Lifted::new(0x7001e18774, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b568> as *const (), 2), // +0x4774 sub_4774
    Lifted::new(0x7001e187b0, tpl_24::<0x10, 0x7001e6b578, 0x8, 0x7001e6b568> as *const (), 1), // +0x47b0 sub_47b0
    Lifted::new(0x7001e187d0, tpl_25::<0x7001e6b8d8, 0x8> as *const (), 1), // +0x47d0 ??0codecvt_base@std@@QEAA@_K@Z
    Lifted::new(0x7001e187f0, fn_7001e187f0 as *const (), 1), // +0x47f0 ??0ctype_base@std@@QEAA@_K@Z
    Lifted::new(0x7001e18804, fn_7001e18804 as *const (), 1), // +0x4804 sub_4804
    Lifted::new(0x7001e18810, tpl_25::<0x7001e6b890, 0x8> as *const (), 1), // +0x4810 ??0facet@locale@std@@IEAA@_K@Z
    Lifted::new(0x7001e18824, tpl_26::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b4d0, 0x7001e6bbf0, 0x18> as *const (), 2), // +0x4824 sub_4824
    Lifted::new(0x7001e1887c, fn_7001e1887c as *const (), 2), // +0x487c sub_487c
    Lifted::new(0x7001e188b0, fn_7001e188b0 as *const (), 1), // +0x48b0 ??0ios_base@std@@IEAA@XZ
    Lifted::new(0x7001e188e0, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b4b8> as *const (), 2), // +0x48e0 sub_48e0
    Lifted::new(0x7001e1891c, tpl_26::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b4d0, 0x7001e6b4f0, 0x18> as *const (), 2), // +0x491c sub_491c
    Lifted::new(0x7001e18974, fn_7001e18974 as *const (), 2), // +0x4974 sub_4974
    Lifted::new(0x7001e18a50, tpl_25::<0x7001e6bc60, 0x8> as *const (), 1), // +0x4a50 ??0time_base@std@@QEAA@_K@Z
    Lifted::new(0x7001e18a70, fn_7001e18a70 as *const (), 0), // +0x4a70 ??1?$_Yarn@D@std@@QEAA@XZ
    Lifted::new(0x7001e18a94, fn_7001e18a94 as *const (), 0), // +0x4a94 sub_4a94
    Lifted::new(0x7001e18b50, fn_7001e18b50 as *const (), 0), // +0x4b50 ??1?$basic_ios@DU?$char_traits@D@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e18b70, tpl_27::<0xfffffffffffffff0, 0x4, 0x7001e6bc90, 0xffffffffffffffec> as *const (), 1), // +0x4b70 ??1?$basic_ostream@DU?$char_traits@D@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e18ba0, tpl_28::<0x60, 0x7001e6bca8, 0x8, 0x10, 0x7001e696a0, 0x7001e6649c> as *const (), 0), // +0x4ba0 ??1?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e18bfc, fn_7001e18bfc as *const (), 0), // +0x4bfc sub_4bfc
    Lifted::new(0x7001e18c60, fn_7001e18c60 as *const (), 1), // +0x4c60 ??1?$codecvt@DDU_Mbstatet@@@std@@MEAA@XZ
    Lifted::new(0x7001e18c70, fn_7001e18c70 as *const (), 0), // +0x4c70 ??1?$ctype@D@std@@MEAA@XZ
    Lifted::new(0x7001e18cc0, tpl_29::<0x20, 0x7001e6bb70, 0x18, 0x7001e693a8, 0x28, 0x7001e6b590> as *const (), 0), // +0x4cc0 ??1?$ctype@G@std@@MEAA@XZ
    Lifted::new(0x7001e18d00, tpl_29::<0x20, 0x7001e6baf0, 0x18, 0x7001e693a8, 0x28, 0x7001e6b590> as *const (), 0), // +0x4d00 ??1?$ctype@_W@std@@MEAA@XZ
    Lifted::new(0x7001e18d40, fn_7001e18d40 as *const (), 0), // +0x4d40 sub_4d40
    Lifted::new(0x7001e18d70, fn_7001e18d70 as *const (), 0), // +0x4d70 ??1Init@ios_base@std@@QEAA@XZ
    Lifted::new(0x7001e18d80, fn_7001e18d80 as *const (), 0), // +0x4d80 ??1_Locimp@locale@std@@MEAA@XZ
    Lifted::new(0x7001e18dc0, fn_7001e18dc0 as *const (), 0), // +0x4dc0 ??1_Locinfo@std@@QEAA@XZ
    Lifted::new(0x7001e18e60, fn_7001e18e60 as *const (), 0), // +0x4e60 ??1_Timevec@std@@QEAA@XZ
    Lifted::new(0x7001e18e80, fn_7001e18e80 as *const (), 0), // +0x4e80 ??1ios_base@std@@UEAA@XZ
    Lifted::new(0x7001e18e9c, fn_7001e18e9c as *const (), 0), // +0x4e9c sub_4e9c
    Lifted::new(0x7001e18ee0, fn_7001e18ee0 as *const (), 1), // +0x4ee0 ??4?$_Iosb@H@std@@QEAAAEAV01@$$QEAV01@@Z
    Lifted::new(0x7001e18ef0, fn_7001e18ef0 as *const (), 0), // +0x4ef0 ??4?$_Yarn@D@std@@QEAAAEAV01@PEBD@Z
    Lifted::new(0x7001e18f70, fn_7001e18f70 as *const (), 0), // +0x4f70 ??4?$_Yarn@G@std@@QEAAAEAV01@PEBG@Z
    Lifted::new(0x7001e19000, fn_7001e19000 as *const (), 0), // +0x5000 ??4_Timevec@std@@QEAAAEAV01@AEBV01@@Z
    Lifted::new(0x7001e19040, fn_7001e19040 as *const (), 1), // +0x5040 ??7ios_base@std@@QEBA_NXZ
    Lifted::new(0x7001e19050, fn_7001e19050 as *const (), 0), // +0x5050 ??Bid@locale@std@@QEAA_KXZ
    Lifted::new(0x7001e190a0, fn_7001e190a0 as *const (), 1), // +0x50a0 ??Bios_base@std@@QEBA_NXZ
    Lifted::new(0x7001e190b0, fn_7001e190b0 as *const (), 0), // +0x50b0 ??_D?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e190f0, tpl_30::<0xfffffffffffffffc, 0x7001e5b5dc> as *const (), 0), // +0x50f0 sub_50f0
    Lifted::new(0x7001e19100, fn_7001e19100 as *const (), 0), // +0x5100 sub_5100
    Lifted::new(0x7001e19180, fn_7001e19180 as *const (), 0), // +0x5180 sub_5180
    Lifted::new(0x7001e19200, fn_7001e19200 as *const (), 0), // +0x5200 sub_5200
    Lifted::new(0x7001e192c0, fn_7001e192c0 as *const (), 0), // +0x52c0 sub_52c0
    Lifted::new(0x7001e19370, fn_7001e19370 as *const (), 0), // +0x5370 sub_5370
    Lifted::new(0x7001e19420, fn_7001e19420 as *const (), 0), // +0x5420 sub_5420
    Lifted::new(0x7001e194a0, fn_7001e194a0 as *const (), 0), // +0x54a0 sub_54a0
    Lifted::new(0x7001e19540, fn_7001e19540 as *const (), 0), // +0x5540 sub_5540
    Lifted::new(0x7001e195c0, fn_7001e195c0 as *const (), 0), // +0x55c0 sub_55c0
    Lifted::new(0x7001e19650, tpl_13::<0x7001e17df0> as *const (), 0), // +0x5650 ??_F?$codecvt@GDU_Mbstatet@@@std@@QEAAXXZ
    Lifted::new(0x7001e19660, fn_7001e19660 as *const (), 0), // +0x5660 ??_F?$codecvt@_SDU_Mbstatet@@@std@@QEAAXXZ
    Lifted::new(0x7001e196b0, fn_7001e196b0 as *const (), 0), // +0x56b0 ??_F?$codecvt@_UDU_Mbstatet@@@std@@QEAAXXZ
    Lifted::new(0x7001e19700, tpl_13::<0x7001e17fe0> as *const (), 0), // +0x5700 ??_F?$codecvt@_WDU_Mbstatet@@@std@@QEAAXXZ
    Lifted::new(0x7001e19710, fn_7001e19710 as *const (), 0), // +0x5710 ??_F?$ctype@D@std@@QEAAXXZ
    Lifted::new(0x7001e19720, tpl_13::<0x7001e181d0> as *const (), 0), // +0x5720 ??_F?$ctype@G@std@@QEAAXXZ
    Lifted::new(0x7001e19730, tpl_13::<0x7001e182d0> as *const (), 0), // +0x5730 ??_F?$ctype@_W@std@@QEAAXXZ
    Lifted::new(0x7001e19740, fn_7001e19740 as *const (), 0), // +0x5740 ??_F_Locinfo@std@@QEAAXXZ
    Lifted::new(0x7001e19750, fn_7001e19750 as *const (), 1), // +0x5750 ??_F_Timevec@std@@QEAAXXZ
    Lifted::new(0x7001e19760, tpl_31::<0x8, 0x7001e6b8d8> as *const (), 1), // +0x5760 ??_Fcodecvt_base@std@@QEAAXXZ
    Lifted::new(0x7001e19770, tpl_31::<0x8, 0x7001e6ba70> as *const (), 1), // +0x5770 ??_Fctype_base@std@@QEAAXXZ
    Lifted::new(0x7001e19780, tpl_31::<0x8, 0x7001e6b890> as *const (), 1), // +0x5780 ??_Ffacet@locale@std@@QEAAXXZ
    Lifted::new(0x7001e19790, tpl_31::<0x8, 0x7001e6bc60> as *const (), 1), // +0x5790 ??_Ftime_base@std@@QEAAXXZ
    Lifted::new(0x7001e197a0, tpl_32::<0x7001e18a94, 0x7001e6649c> as *const (), 0), // +0x57a0 sub_57a0
    Lifted::new(0x7001e197e0, fn_7001e197e0 as *const (), 0), // +0x57e0 sub_57e0
    Lifted::new(0x7001e19810, fn_7001e19810 as *const (), 0), // +0x5810 sub_5810
    Lifted::new(0x7001e19860, fn_7001e19860 as *const (), 0), // +0x5860 sub_5860
    Lifted::new(0x7001e198b0, fn_7001e198b0 as *const (), 0), // +0x58b0 ?_Addcats@_Locinfo@std@@QEAAAEAV12@HPEBD@Z
    Lifted::alias(0x7001e198d0, fn_7001e22e90 as *const (), 0, 0xee90), // +0x58d0 ?_Addfac@_Locimp@locale@std@@AEAAXPEAVfacet@23@_K@Z (thunk -> +0xee90)
    Lifted::new(0x7001e198e0, fn_7001e198e0 as *const (), 1), // +0x58e0 ?_C_str@?$_Yarn@D@std@@QEBAPEBDXZ
    Lifted::new(0x7001e198f0, fn_7001e198f0 as *const (), 0), // +0x58f0 ?_Callfns@ios_base@std@@AEAAXW4event@12@@Z
    Lifted::new(0x7001e19940, fn_7001e19940 as *const (), 1), // +0x5940 ?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ
    Lifted::new(0x7001e19950, fn_7001e19950 as *const (), 0), // +0x5950 ?_Donarrow@?$ctype@G@std@@IEBADGD@Z
    Lifted::new(0x7001e199b0, fn_7001e199b0 as *const (), 0), // +0x59b0 ?_Dowiden@?$ctype@G@std@@IEBAGD@Z
    Lifted::new(0x7001e19a00, fn_7001e19a00 as *const (), 0), // +0x5a00 ?_Dowiden@?$ctype@_W@std@@IEBA_WD@Z
    Lifted::new(0x7001e19a50, fn_7001e19a50 as *const (), 1), // +0x5a50 ?_Empty@?$_Yarn@D@std@@QEBA_NXZ
    Lifted::new(0x7001e19a58, fn_7001e19a58 as *const (), 0), // +0x5a58 sub_5a58
    Lifted::new(0x7001e19b30, fn_7001e19b30 as *const (), 0), // +0x5b30 ?_Findarr@ios_base@std@@AEAAAEAU_Iosarray@12@H@Z
    Lifted::new(0x7001e19bd0, fn_7001e19bd0 as *const (), 0), // +0x5bd0 ?_Getcat@?$codecvt@DDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e19c90, fn_7001e19c90 as *const (), 2), // +0x5c90 ?_Getcat@?$codecvt@GDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e19d80, fn_7001e19d80 as *const (), 0), // +0x5d80 ?_Getcat@?$codecvt@_SDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e19e50, fn_7001e19e50 as *const (), 0), // +0x5e50 ?_Getcat@?$codecvt@_UDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e19f20, fn_7001e19f20 as *const (), 2), // +0x5f20 ?_Getcat@?$codecvt@_WDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e1a010, fn_7001e1a010 as *const (), 2), // +0x6010 ?_Getcat@?$ctype@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e1a0f0, fn_7001e1a0f0 as *const (), 2), // +0x60f0 ?_Getcat@?$ctype@G@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e1a200, fn_7001e1a200 as *const (), 2), // +0x6200 ?_Getcat@?$ctype@_W@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e1a310, fn_7001e1a310 as *const (), 1), // +0x6310 ?_Getcat@facet@locale@std@@SA_KPEAPEBV123@PEBV23@@Z
    Lifted::new(0x7001e1a320, fn_7001e1a320 as *const (), 2), // +0x6320 ?_Getcoll@_Locinfo@std@@QEBA?AU_Collvec@@XZ
    Lifted::new(0x7001e1a350, fn_7001e1a350 as *const (), 2), // +0x6350 ?_Getctype@_Locinfo@std@@QEBA?AU_Ctypevec@@XZ
    Lifted::new(0x7001e1a380, fn_7001e1a380 as *const (), 2), // +0x6380 ?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ
    Lifted::alias(0x7001e1a3c0, fn_7001e416a0 as *const (), 0, 0x2d6a0), // +0x63c0 ?_Getdateorder@_Locinfo@std@@QEBAHXZ (thunk -> +0x2d6a0)
    Lifted::new(0x7001e1a3d0, tpl_33::<0x7001e69660, 0x8, 0x7001e18ef0, 0x7001e693a8, 0x7001e6b5c0> as *const (), 0), // +0x63d0 ?_Getdays@_Locinfo@std@@QEBAPEBDXZ
    Lifted::new(0x7001e1a420, tpl_34::<0x7001e6b6a8> as *const (), 1), // +0x6420 ?_Getfalse@_Locinfo@std@@QEBAPEBDXZ
    Lifted::new(0x7001e1a430, tpl_15::<0x7001e693f8> as *const (), 2), // +0x6430 ?_Getlconv@_Locinfo@std@@QEBAPEBUlconv@@XZ
    Lifted::new(0x7001e1a440, tpl_33::<0x7001e69670, 0x18, 0x7001e18ef0, 0x7001e693a8, 0x7001e6b620> as *const (), 0), // +0x6440 ?_Getmonths@_Locinfo@std@@QEBAPEBDXZ
    Lifted::new(0x7001e1a490, fn_7001e1a490 as *const (), 1), // +0x6490 ?_Getname@_Locinfo@std@@QEBAPEBDXZ
    Lifted::new(0x7001e1a4a0, fn_7001e1a4a0 as *const (), 1), // +0x64a0 ?_Getptr@_Timevec@std@@QEBAPEAXXZ
    Lifted::new(0x7001e1a4b0, tpl_35::<0x7001e69668> as *const (), 0), // +0x64b0 ?_Gettnames@_Locinfo@std@@QEBA?AV_Timevec@2@XZ
    Lifted::new(0x7001e1a4d0, tpl_34::<0x7001e6b6b0> as *const (), 1), // +0x64d0 ?_Gettrue@_Locinfo@std@@QEBAPEBDXZ
    Lifted::new(0x7001e1a4e0, tpl_36::<0x38, 0x50> as *const (), 1), // +0x64e0 ?_Gnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ
    Lifted::new(0x7001e1a500, fn_7001e1a500 as *const (), 1), // +0x6500 ?_Gndec@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ
    Lifted::new(0x7001e1a520, tpl_37::<0x50, 0x38, 0x1> as *const (), 1), // +0x6520 ?_Gninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ
    Lifted::new(0x7001e1a540, fn_7001e1a540 as *const (), 1), // +0x6540 ?_Incref@facet@locale@std@@UEAAXXZ
    Lifted::new(0x7001e1a548, fn_7001e1a548 as *const (), 0), // +0x6548 sub_6548
    Lifted::new(0x7001e1a620, fn_7001e1a620 as *const (), 1), // +0x6620 ?_Init@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAPEAD0PEAH001@Z
    Lifted::new(0x7001e1a650, fn_7001e1a650 as *const (), 1), // +0x6650 ?_Init@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXXZ
    Lifted::new(0x7001e1a6a0, fn_7001e1a6a0 as *const (), 2), // +0x66a0 ?_Init@?$codecvt@GDU_Mbstatet@@@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e1a6e0, fn_7001e1a6e0 as *const (), 2), // +0x66e0 ?_Init@?$ctype@D@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e1a710, fn_7001e1a710 as *const (), 2), // +0x6710 ?_Init@?$ctype@G@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e1a770, fn_7001e1a770 as *const (), 0), // +0x6770 ?_Init@ios_base@std@@IEAAXXZ
    Lifted::new(0x7001e1a7e0, fn_7001e1a7e0 as *const (), 0), // +0x67e0 sub_67e0
    Lifted::new(0x7001e1a860, tpl_38::<0x80, 0x7001e69398> as *const (), 0), // +0x6860 sub_6860
    Lifted::new(0x7001e1a87c, fn_7001e1a87c as *const (), 2), // +0x687c sub_687c
    Lifted::new(0x7001e1a9a0, tpl_36::<0x40, 0x58> as *const (), 1), // +0x69a0 ?_Pnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ
    Lifted::new(0x7001e1a9c0, tpl_37::<0x58, 0x40, 0x1> as *const (), 1), // +0x69c0 ?_Pninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ
    Lifted::new(0x7001e1a9d8, tpl_39::<0x7001e18754, 0x7001e76c28, 0x7001e66444, 0x7001e1a9f7> as *const (), 0), // +0x69d8 sub_69d8
    Lifted::new(0x7001e1a9f8, tpl_39::<0x7001e187b0, 0x7001e76d08, 0x7001e66444, 0x7001e1aa17> as *const (), 0), // +0x69f8 sub_69f8
    Lifted::new(0x7001e1aa20, fn_7001e1aa20 as *const (), 0), // +0x6a20 ?_Tidy@?$ctype@D@std@@IEAAXXZ
    Lifted::new(0x7001e1aa60, fn_7001e1aa60 as *const (), 0), // +0x6a60 ?_Tidy@ios_base@std@@AEAAXXZ
    Lifted::new(0x7001e1aae0, tpl_38::<0x80, 0x7001e69380> as *const (), 0), // +0x6ae0 sub_6ae0
    Lifted::new(0x7001e1ab00, tpl_33::<0x7001e69650, 0x28, 0x7001e18f70, 0x7001e693a8, 0x7001e6b6c0> as *const (), 0), // +0x6b00 ?_W_Getdays@_Locinfo@std@@QEBAPEBGXZ
    Lifted::new(0x7001e1ab50, tpl_33::<0x7001e69648, 0x38, 0x7001e18f70, 0x7001e693a8, 0x7001e6b770> as *const (), 0), // +0x6b50 ?_W_Getmonths@_Locinfo@std@@QEBAPEBGXZ
    Lifted::new(0x7001e1aba0, tpl_35::<0x7001e69640> as *const (), 0), // +0x6ba0 ?_W_Gettnames@_Locinfo@std@@QEBA?AV_Timevec@2@XZ
    Lifted::new(0x7001e1abbc, tpl_40::<0x7001e6b4a0, 0x7001e4d4f0, 0x7001e1abcc> as *const (), 0), // +0x6bbc sub_6bbc
    Lifted::new(0x7001e1abd0, tpl_40::<0x7001e6bdf8, 0x7001e4d520, 0x7001e1abe0> as *const (), 0), // +0x6bd0 sub_6bd0
    Lifted::new(0x7001e1abf0, tpl_41::<0x18, 0x7001e696a0> as *const (), 0), // +0x6bf0 ?always_noconv@codecvt_base@std@@QEBA_NXZ
    Lifted::new(0x7001e1ac00, fn_7001e1ac00 as *const (), 0), // +0x6c00 sub_6c00
    Lifted::new(0x7001e1ac80, fn_7001e1ac80 as *const (), 1), // +0x6c80 ?bad@ios_base@std@@QEBA_NXZ
    Lifted::new(0x7001e1ac90, fn_7001e1ac90 as *const (), 0), // +0x6c90 ?classic_table@?$ctype@D@std@@SAPEBFXZ
    Lifted::new(0x7001e1acb0, fn_7001e1acb0 as *const (), 0), // +0x6cb0 ?clear@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z
    Lifted::new(0x7001e1acd0, fn_7001e1acd0 as *const (), 0), // +0x6cd0 ?clear@ios_base@std@@QEAAXH@Z
    Lifted::new(0x7001e1ad40, fn_7001e1ad40 as *const (), 0), // +0x6d40 ?clear@ios_base@std@@QEAAXH_N@Z
    Lifted::alias(0x7001e1adc0, fn_7001e1acd0 as *const (), 0, 0x6cd0), // +0x6dc0 ?clear@ios_base@std@@QEAAXI@Z (thunk -> +0x6cd0)
    Lifted::new(0x7001e1adc8, fn_7001e1adc8 as *const (), 0), // +0x6dc8 sub_6dc8
    Lifted::new(0x7001e1ae60, fn_7001e1ae60 as *const (), 0), // +0x6e60 ?copyfmt@ios_base@std@@QEAAAEAV12@AEBV12@@Z
    Lifted::new(0x7001e1b090, fn_7001e1b090 as *const (), 1), // +0x7090 sub_7090
    Lifted::new(0x7001e1b0a0, fn_7001e1b0a0 as *const (), 1), // +0x70a0 ?do_always_noconv@?$codecvt@DDU_Mbstatet@@@std@@MEBA_NXZ
    Lifted::new(0x7001e1b0b0, fn_7001e1b0b0 as *const (), 1), // +0x70b0 ?_IsCurrentOriginSTA@_ContextCallback@details@Concurrency@@CA_NXZ
    Lifted::new(0x7001e1b0c0, fn_7001e1b0c0 as *const (), 1), // +0x70c0 ?do_encoding@?$codecvt@GDU_Mbstatet@@@std@@MEBAHXZ
    Lifted::new(0x7001e1b0d0, fn_7001e1b0d0 as *const (), 1), // +0x70d0 ?do_encoding@?$codecvt@_UDU_Mbstatet@@@std@@MEBAHXZ
    Lifted::new(0x7001e1b0e0, fn_7001e1b0e0 as *const (), 1), // +0x70e0 ?do_encoding@codecvt_base@std@@MEBAHXZ
    Lifted::new(0x7001e1b0f0, fn_7001e1b0f0 as *const (), 1), // +0x70f0 ?do_in@?$codecvt@DDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z
    Lifted::new(0x7001e1b110, fn_7001e1b110 as *const (), 0), // +0x7110 ?do_in@?$codecvt@GDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAG3AEAPEAG@Z
    Lifted::new(0x7001e1b1c0, fn_7001e1b1c0 as *const (), 0), // +0x71c0 ?do_in@?$codecvt@_SDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEA_S3AEAPEA_S@Z
    Lifted::new(0x7001e1b480, fn_7001e1b480 as *const (), 0), // +0x7480 ?do_in@?$codecvt@_UDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEA_U3AEAPEA_U@Z
    Lifted::new(0x7001e1b640, fn_7001e1b640 as *const (), 0), // +0x7640 ?do_is@?$ctype@G@std@@MEBAPEBGPEBG0PEAF@Z
    Lifted::new(0x7001e1b660, fn_7001e1b660 as *const (), 0), // +0x7660 ?do_is@?$ctype@G@std@@MEBA_NFG@Z
    Lifted::new(0x7001e1b690, fn_7001e1b690 as *const (), 1), // +0x7690 ?do_length@?$codecvt@DDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1_K@Z
    Lifted::new(0x7001e1b6b0, fn_7001e1b6b0 as *const (), 0), // +0x76b0 ?do_length@?$codecvt@GDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1_K@Z
    Lifted::alias(0x7001e1b750, fn_7001e1734c as *const (), 0, 0x334c), // +0x7750 ?do_length@?$codecvt@_SDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1_K@Z (thunk -> +0x334c)
    Lifted::alias(0x7001e1b760, fn_7001e17424 as *const (), 0, 0x3424), // +0x7760 ?do_length@?$codecvt@_UDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBD1_K@Z (thunk -> +0x3424)
    Lifted::new(0x7001e1b770, tpl_42::<0x14> as *const (), 1), // +0x7770 ?do_max_length@?$codecvt@GDU_Mbstatet@@@std@@MEBAHXZ
    Lifted::new(0x7001e1b780, fn_7001e1b780 as *const (), 1), // +0x7780 ?do_max_length@?$codecvt@_SDU_Mbstatet@@@std@@MEBAHXZ
    Lifted::new(0x7001e1b7a0, fn_7001e1b7a0 as *const (), 1), // +0x77a0 ?do_max_length@?$codecvt@_UDU_Mbstatet@@@std@@MEBAHXZ
    Lifted::new(0x7001e1b7b0, fn_7001e1b7b0 as *const (), 1), // +0x77b0 ?do_narrow@?$ctype@D@std@@MEBADDD@Z
    Lifted::new(0x7001e1b7c0, fn_7001e1b7c0 as *const (), 0), // +0x77c0 ?do_narrow@?$ctype@D@std@@MEBAPEBDPEBD0DPEAD@Z
    Lifted::new(0x7001e1b7e0, fn_7001e1b7e0 as *const (), 0), // +0x77e0 ?do_narrow@?$ctype@G@std@@MEBAPEBGPEBG0DPEAD@Z
    Lifted::new(0x7001e1b880, fn_7001e1b880 as *const (), 0), // +0x7880 ?do_out@?$codecvt@GDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEBG1AEAPEBGPEAD3AEAPEAD@Z
    Lifted::new(0x7001e1b9b0, fn_7001e1b9b0 as *const (), 1), // +0x79b0 ?do_out@?$codecvt@_SDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEB_S1AEAPEB_SPEAD3AEAPEAD@Z
    Lifted::new(0x7001e1bba0, fn_7001e1bba0 as *const (), 1), // +0x7ba0 ?do_out@?$codecvt@_UDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEB_U1AEAPEB_UPEAD3AEAPEAD@Z
    Lifted::new(0x7001e1bd60, fn_7001e1bd60 as *const (), 0), // +0x7d60 ?do_scan_is@?$ctype@G@std@@MEBAPEBGFPEBG0@Z
    Lifted::new(0x7001e1bdd0, fn_7001e1bdd0 as *const (), 0), // +0x7dd0 ?do_scan_not@?$ctype@G@std@@MEBAPEBGFPEBG0@Z
    Lifted::new(0x7001e1be40, tpl_43::<0x10, 0x7001e17060> as *const (), 0), // +0x7e40 ?do_tolower@?$ctype@D@std@@MEBADD@Z
    Lifted::new(0x7001e1be50, tpl_44::<0x10, 0x7001e17060> as *const (), 0), // +0x7e50 ?do_tolower@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z
    Lifted::new(0x7001e1bea0, tpl_45::<0x10, 0x7001e4d5f0> as *const (), 0), // +0x7ea0 ?do_tolower@?$ctype@G@std@@MEBAGG@Z
    Lifted::new(0x7001e1beb0, tpl_46::<0x10, 0x7001e4d5f0> as *const (), 0), // +0x7eb0 ?do_tolower@?$ctype@G@std@@MEBAPEBGPEAGPEBG@Z
    Lifted::new(0x7001e1bf00, tpl_43::<0x10, 0x7001e171a0> as *const (), 0), // +0x7f00 ?do_toupper@?$ctype@D@std@@MEBADD@Z
    Lifted::new(0x7001e1bf10, tpl_44::<0x10, 0x7001e171a0> as *const (), 0), // +0x7f10 ?do_toupper@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z
    Lifted::new(0x7001e1bf60, tpl_45::<0x10, 0x7001e4d670> as *const (), 0), // +0x7f60 ?do_toupper@?$ctype@G@std@@MEBAGG@Z
    Lifted::new(0x7001e1bf70, tpl_46::<0x10, 0x7001e4d670> as *const (), 0), // +0x7f70 ?do_toupper@?$ctype@G@std@@MEBAPEBGPEAGPEBG@Z
    Lifted::new(0x7001e1bfc0, fn_7001e1bfc0 as *const (), 1), // +0x7fc0 ?do_unshift@?$codecvt@DDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEAD1AEAPEAD@Z
    Lifted::new(0x7001e1bfd0, fn_7001e1bfd0 as *const (), 0), // +0x7fd0 ?do_unshift@?$codecvt@GDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEAD1AEAPEAD@Z
    Lifted::new(0x7001e1c080, fn_7001e1c080 as *const (), 1), // +0x8080 ?do_unshift@?$codecvt@_SDU_Mbstatet@@@std@@MEBAHAEAU_Mbstatet@@PEAD1AEAPEAD@Z
    Lifted::new(0x7001e1c0a0, fn_7001e1c0a0 as *const (), 0), // +0x80a0 ?do_widen@?$ctype@D@std@@MEBAPEBDPEBD0PEAD@Z
    Lifted::new(0x7001e1c0c0, fn_7001e1c0c0 as *const (), 0), // +0x80c0 ?do_widen@?$ctype@G@std@@MEBAGD@Z
    Lifted::new(0x7001e1c110, fn_7001e1c110 as *const (), 0), // +0x8110 ?do_widen@?$ctype@G@std@@MEBAPEBDPEBD0PEAG@Z
    Lifted::new(0x7001e1c190, fn_7001e1c190 as *const (), 0), // +0x8190 ?do_widen@?$ctype@_W@std@@MEBAPEBDPEBD0PEA_W@Z
    Lifted::new(0x7001e1c210, fn_7001e1c210 as *const (), 0), // +0x8210 ?do_widen@?$ctype@_W@std@@MEBA_WD@Z
    Lifted::new(0x7001e1c260, tpl_47::<0x18> as *const (), 1), // +0x8260 ?eback@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ
    Lifted::new(0x7001e1c270, fn_7001e1c270 as *const (), 1), // +0x8270 ?egptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ
    Lifted::new(0x7001e1c280, tpl_41::<0x28, 0x7001e696a0> as *const (), 0), // +0x8280 ?encoding@codecvt_base@std@@QEBAHXZ
    Lifted::new(0x7001e1c290, fn_7001e1c290 as *const (), 1), // +0x8290 ?eof@ios_base@std@@QEBA_NXZ
    Lifted::new(0x7001e1c2a0, fn_7001e1c2a0 as *const (), 1), // +0x82a0 ?epptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ
    Lifted::new(0x7001e1c2b0, fn_7001e1c2b0 as *const (), 1), // +0x82b0 sub_82b0
    Lifted::new(0x7001e1c2d0, fn_7001e1c2d0 as *const (), 0), // +0x82d0 sub_82d0
    Lifted::new(0x7001e1c320, fn_7001e1c320 as *const (), 0), // +0x8320 ?exceptions@ios_base@std@@QEAAXH@Z
    Lifted::new(0x7001e1c330, fn_7001e1c330 as *const (), 1), // +0x8330 ?flags@ios_base@std@@QEAAHH@Z
    Lifted::new(0x7001e1c340, tpl_42::<0x18> as *const (), 1), // +0x8340 ?flags@ios_base@std@@QEBAHXZ
    Lifted::new(0x7001e1c350, tpl_48::<0x50, 0x38> as *const (), 1), // +0x8350 ?gbump@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXH@Z
    Lifted::new(0x7001e1c370, tpl_49::<0x40, 0x8, 0x7001e696a0> as *const (), 0), // +0x8370 ?getloc@ios_base@std@@QEBA?AVlocale@2@XZ
    Lifted::new(0x7001e1c3a0, fn_7001e1c3a0 as *const (), 1), // +0x83a0 ?good@ios_base@std@@QEBA_NXZ
    Lifted::new(0x7001e1c3b0, tpl_47::<0x38> as *const (), 1), // +0x83b0 ?gptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ
    Lifted::new(0x7001e1c3c0, tpl_50::<0x7001e1793c, 0x7001e1a7e0> as *const (), 0), // +0x83c0 sub_83c0
    Lifted::new(0x7001e1c3f0, fn_7001e1c3f0 as *const (), 0), // +0x83f0 ?imbue@ios_base@std@@QEAA?AVlocale@2@AEBV32@@Z
    Lifted::new(0x7001e1c4c0, tpl_51::<0x30, 0x7001e696a0> as *const (), 0), // +0x84c0 ?in@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z
    Lifted::new(0x7001e1c510, fn_7001e1c510 as *const (), 0), // +0x8510 ?init@?$basic_ios@DU?$char_traits@D@std@@@std@@IEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@_N@Z
    Lifted::new(0x7001e1c5d0, fn_7001e1c5d0 as *const (), 1), // +0x85d0 ?is@?$ctype@D@std@@QEBAPEBDPEBD0PEAF@Z
    Lifted::new(0x7001e1c600, fn_7001e1c600 as *const (), 1), // +0x8600 ?is@?$ctype@D@std@@QEBA_NFD@Z
    Lifted::new(0x7001e1c620, tpl_52::<0x18, 0x7001e696a0> as *const (), 0), // +0x8620 ?is@?$ctype@G@std@@QEBAPEBGPEBG0PEAF@Z
    Lifted::new(0x7001e1c640, tpl_41::<0x20, 0x7001e696a0> as *const (), 0), // +0x8640 ?is@?$ctype@G@std@@QEBA_NFG@Z
    Lifted::new(0x7001e1c650, fn_7001e1c650 as *const (), 0), // +0x8650 ?iword@ios_base@std@@QEAAAEAJH@Z
    Lifted::new(0x7001e1c670, tpl_53::<0x48, 0x7001e696a0> as *const (), 0), // +0x8670 ?length@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1_K@Z
    Lifted::new(0x7001e1c690, fn_7001e1c690 as *const (), 1), // +0x8690 sub_8690
    Lifted::new(0x7001e1c6b0, fn_7001e1c6b0 as *const (), 2), // +0x86b0 sub_86b0
    Lifted::new(0x7001e1c720, tpl_34::<0x7001e6b538> as *const (), 1), // +0x8720 sub_8720
    Lifted::new(0x7001e1c730, tpl_41::<0x50, 0x7001e696a0> as *const (), 0), // +0x8730 ?narrow@?$ctype@D@std@@QEBADDD@Z
    Lifted::new(0x7001e1c740, tpl_41::<0x70, 0x7001e696a0> as *const (), 0), // +0x8740 ?narrow@?$ctype@G@std@@QEBADGD@Z
    Lifted::new(0x7001e1c750, tpl_53::<0x68, 0x7001e696a0> as *const (), 0), // +0x8750 ?narrow@?$ctype@G@std@@QEBAPEBGPEBG0DPEAD@Z
    Lifted::new(0x7001e1c770, tpl_51::<0x38, 0x7001e696a0> as *const (), 0), // +0x8770 ?out@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z
    Lifted::new(0x7001e1c7c0, fn_7001e1c7c0 as *const (), 0), // +0x87c0 sub_87c0
    Lifted::new(0x7001e1c950, fn_7001e1c950 as *const (), 1), // +0x8950 ?overflow@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHH@Z
    Lifted::new(0x7001e1c960, fn_7001e1c960 as *const (), 0), // +0x8960 sub_8960
    Lifted::new(0x7001e1ca30, tpl_48::<0x58, 0x40> as *const (), 1), // +0x8a30 ?pbump@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXH@Z
    Lifted::new(0x7001e1ca50, tpl_47::<0x40> as *const (), 1), // +0x8a50 ?pptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ
    Lifted::new(0x7001e1ca60, tpl_54::<0x20> as *const (), 1), // +0x8a60 ?precision@ios_base@std@@QEAA_J_J@Z
    Lifted::new(0x7001e1ca70, tpl_55::<0x20> as *const (), 1), // +0x8a70 ?precision@ios_base@std@@QEBA_JXZ
    Lifted::new(0x7001e1ca80, fn_7001e1ca80 as *const (), 0), // +0x8a80 ?pword@ios_base@std@@QEAAAEAPEAXH@Z
    Lifted::new(0x7001e1caa0, tpl_42::<0x10> as *const (), 1), // +0x8aa0 ?rdstate@ios_base@std@@QEBAHXZ
    Lifted::new(0x7001e1cab0, fn_7001e1cab0 as *const (), 0), // +0x8ab0 ?register_callback@ios_base@std@@QEAAXP6AXW4event@12@AEAV12@H@ZH@Z
    Lifted::new(0x7001e1cb00, fn_7001e1cb00 as *const (), 1), // +0x8b00 ?scan_is@?$ctype@D@std@@QEBAPEBDFPEBD0@Z
    Lifted::new(0x7001e1cb20, tpl_52::<0x28, 0x7001e696a0> as *const (), 0), // +0x8b20 ?scan_is@?$ctype@G@std@@QEBAPEBGFPEBG0@Z
    Lifted::new(0x7001e1cb40, fn_7001e1cb40 as *const (), 1), // +0x8b40 ?scan_not@?$ctype@D@std@@QEBAPEBDFPEBD0@Z
    Lifted::new(0x7001e1cb60, tpl_52::<0x30, 0x7001e696a0> as *const (), 0), // +0x8b60 ?scan_not@?$ctype@G@std@@QEBAPEBGFPEBG0@Z
    Lifted::new(0x7001e1cb80, fn_7001e1cb80 as *const (), 0), // +0x8b80 sub_8b80
    Lifted::new(0x7001e1cc70, fn_7001e1cc70 as *const (), 1), // +0x8c70 ?seekoff@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA?AV?$fpos@U_Mbstatet@@@2@_JHH@Z
    Lifted::new(0x7001e1cc90, fn_7001e1cc90 as *const (), 0), // +0x8c90 sub_8c90
    Lifted::new(0x7001e1cd50, fn_7001e1cd50 as *const (), 0), // +0x8d50 sub_8d50
    Lifted::new(0x7001e1cdb0, fn_7001e1cdb0 as *const (), 1), // +0x8db0 ?setf@ios_base@std@@QEAAHH@Z
    Lifted::new(0x7001e1cdc0, fn_7001e1cdc0 as *const (), 1), // +0x8dc0 ?setf@ios_base@std@@QEAAHHH@Z
    Lifted::new(0x7001e1cde0, tpl_56::<0x18, 0x38, 0x50> as *const (), 1), // +0x8de0 ?setg@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAD00@Z
    Lifted::new(0x7001e1ce00, fn_7001e1ce00 as *const (), 1), // +0x8e00 ?setp@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAD0@Z
    Lifted::new(0x7001e1ce20, fn_7001e1ce20 as *const (), 0), // +0x8e20 ?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z
    Lifted::new(0x7001e1ce40, fn_7001e1ce40 as *const (), 0), // +0x8e40 ?setstate@ios_base@std@@QEAAXH@Z
    Lifted::new(0x7001e1ceb0, fn_7001e1ceb0 as *const (), 0), // +0x8eb0 ?setstate@ios_base@std@@QEAAXH_N@Z
    Lifted::alias(0x7001e1cec0, fn_7001e1ce40 as *const (), 0, 0x8e40), // +0x8ec0 ?setstate@ios_base@std@@QEAAXI@Z (thunk -> +0x8e40)
    Lifted::new(0x7001e1ced0, fn_7001e1ced0 as *const (), 1), // +0x8ed0 ?swap@ios_base@std@@QEAAXAEAV12@@Z
    Lifted::new(0x7001e1cf60, fn_7001e1cf60 as *const (), 0), // +0x8f60 sub_8f60
    Lifted::new(0x7001e1cfc0, fn_7001e1cfc0 as *const (), 0), // +0x8fc0 ?sync_with_stdio@ios_base@std@@SA_N_N@Z
    Lifted::new(0x7001e1d000, tpl_55::<0x18> as *const (), 1), // +0x9000 ?table@?$ctype@D@std@@QEBAPEBFXZ
    Lifted::new(0x7001e1d010, tpl_54::<0x50> as *const (), 1), // +0x9010 ?tie@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAPEAV?$basic_ostream@DU?$char_traits@D@std@@@2@PEAV32@@Z
    Lifted::new(0x7001e1d020, tpl_41::<0x40, 0x7001e696a0> as *const (), 0), // +0x9020 ?sgetn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEAD_J@Z
    Lifted::new(0x7001e1d030, tpl_41::<0x38, 0x7001e696a0> as *const (), 0), // +0x9030 ?tolower@?$ctype@G@std@@QEBAPEBGPEAGPEBG@Z
    Lifted::new(0x7001e1d040, tpl_41::<0x30, 0x7001e696a0> as *const (), 0), // +0x9040 ?toupper@?$ctype@D@std@@QEBADD@Z
    Lifted::new(0x7001e1d050, tpl_41::<0x48, 0x7001e696a0> as *const (), 0), // +0x9050 ?sputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEBD_J@Z
    Lifted::new(0x7001e1d060, fn_7001e1d060 as *const (), 2), // +0x9060 sub_9060
    Lifted::new(0x7001e1d320, fn_7001e1d320 as *const (), 0), // +0x9320 ?uflow@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHXZ
    Lifted::new(0x7001e1d360, fn_7001e1d360 as *const (), 0), // +0x9360 sub_9360
    Lifted::new(0x7001e1d3d0, fn_7001e1d3d0 as *const (), 1), // +0x93d0 ?unsetf@ios_base@std@@QEAAXH@Z
    Lifted::new(0x7001e1d3e0, tpl_53::<0x40, 0x7001e696a0> as *const (), 0), // +0x93e0 ?unshift@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEAD1AEAPEAD@Z
    Lifted::new(0x7001e1d400, fn_7001e1d400 as *const (), 0), // +0x9400 ?widen@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADD@Z
    Lifted::new(0x7001e1d490, tpl_52::<0x38, 0x7001e696a0> as *const (), 0), // +0x9490 ?widen@?$ctype@D@std@@QEBAPEBDPEBD0PEAD@Z
    Lifted::new(0x7001e1d4b0, tpl_41::<0x60, 0x7001e696a0> as *const (), 0), // +0x94b0 ?pubsetbuf@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAPEAV12@PEAD_J@Z
    Lifted::new(0x7001e1d4c0, tpl_52::<0x58, 0x7001e696a0> as *const (), 0), // +0x94c0 ?widen@?$ctype@G@std@@QEBAPEBDPEBD0PEAG@Z
    Lifted::new(0x7001e1d4e0, tpl_54::<0x28> as *const (), 1), // +0x94e0 ?width@ios_base@std@@QEAA_J_J@Z
    Lifted::new(0x7001e1d4f0, tpl_55::<0x28> as *const (), 1), // +0x94f0 ?width@ios_base@std@@QEBA_JXZ
    Lifted::new(0x7001e1d500, fn_7001e1d500 as *const (), 0), // +0x9500 ?xalloc@ios_base@std@@SAHXZ
    Lifted::new(0x7001e1d540, fn_7001e1d540 as *const (), 0), // +0x9540 sub_9540
    Lifted::new(0x7001e1d680, fn_7001e1d680 as *const (), 0), // +0x9680 ?xsgetn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JPEAD_J@Z
    Lifted::new(0x7001e1d750, fn_7001e1d750 as *const (), 0), // +0x9750 sub_9750
    Lifted::new(0x7001e1d810, fn_7001e1d810 as *const (), 0), // +0x9810 ?xsputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JPEBD_J@Z
    Lifted::new(0x7001e1d8e0, fn_7001e1d8e0 as *const (), 0), // +0x98e0 ??0?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N@Z
    Lifted::new(0x7001e1d990, tpl_27::<0xffffffffffffffe8, 0x4, 0x7001e6be18, 0xffffffffffffffe4> as *const (), 1), // +0x9990 ??1?$basic_istream@DU?$char_traits@D@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e1d9c0, fn_7001e1d9c0 as *const (), 0), // +0x99c0 ??_D?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e1da00, tpl_30::<0xfffffffffffffffc, 0x7001e5b3cc> as *const (), 0), // +0x9a00 sub_9a00
    Lifted::new(0x7001e1da0c, fn_7001e1da0c as *const (), 0), // +0x9a0c sub_9a0c
    Lifted::new(0x7001e1db14, fn_7001e1db14 as *const (), 0), // +0x9b14 sub_9b14
    Lifted::new(0x7001e1dca0, fn_7001e1dca0 as *const (), 0), // +0x9ca0 sub_9ca0
    Lifted::new(0x7001e1de14, fn_7001e1de14 as *const (), 0), // +0x9e14 sub_9e14
    Lifted::new(0x7001e1de78, fn_7001e1de78 as *const (), 0), // +0x9e78 sub_9e78
    Lifted::new(0x7001e1df00, fn_7001e1df00 as *const (), 0), // +0x9f00 _Chmod
    Lifted::new(0x7001e1df90, tpl_15::<0x7001e69068> as *const (), 2), // +0x9f90 _Close_dir
    Lifted::new(0x7001e1dfa0, fn_7001e1dfa0 as *const (), 0), // +0x9fa0 _Copy_file
    Lifted::new(0x7001e1dfd0, fn_7001e1dfd0 as *const (), 0), // +0x9fd0 _Current_get
    Lifted::new(0x7001e1e010, fn_7001e1e010 as *const (), 0), // +0xa010 _Current_set
    Lifted::new(0x7001e1e030, fn_7001e1e030 as *const (), 2), // +0xa030 _Equivalent
    Lifted::new(0x7001e1e180, fn_7001e1e180 as *const (), 0), // +0xa180 _File_size
    Lifted::new(0x7001e1e1d0, fn_7001e1e1d0 as *const (), 2), // +0xa1d0 _Hard_links
    Lifted::new(0x7001e1e290, fn_7001e1e290 as *const (), 0), // +0xa290 _Last_write_time
    Lifted::new(0x7001e1e2f0, fn_7001e1e2f0 as *const (), 0), // +0xa2f0 _Link
    Lifted::alias(0x7001e1e320, fn_7001e1e740 as *const (), 0, 0xa740), // +0xa320 _Lstat (thunk -> +0xa740)
    Lifted::new(0x7001e1e330, fn_7001e1e330 as *const (), 0), // +0xa330 _Make_dir
    Lifted::new(0x7001e1e360, fn_7001e1e360 as *const (), 2), // +0xa360 _Open_dir
    Lifted::new(0x7001e1e500, fn_7001e1e500 as *const (), 0), // +0xa500 _Read_dir
    Lifted::new(0x7001e1e5e0, fn_7001e1e5e0 as *const (), 0), // +0xa5e0 _Remove_dir
    Lifted::new(0x7001e1e600, tpl_57::<0x7001e69370, 0x7001e69260> as *const (), 0), // +0xa600 _Rename
    Lifted::new(0x7001e1e620, fn_7001e1e620 as *const (), 0), // +0xa620 _Resize
    Lifted::new(0x7001e1e6b0, fn_7001e1e6b0 as *const (), 0), // +0xa6b0 _Set_last_write_time
    Lifted::new(0x7001e1e740, fn_7001e1e740 as *const (), 0), // +0xa740 _Stat
    Lifted::new(0x7001e1e7f0, fn_7001e1e7f0 as *const (), 2), // +0xa7f0 _Statvfs
    Lifted::new(0x7001e1e980, fn_7001e1e980 as *const (), 0), // +0xa980 _Symlink
    Lifted::new(0x7001e1e9b0, fn_7001e1e9b0 as *const (), 1), // +0xa9b0 _Symlink_get
    Lifted::new(0x7001e1e9c0, fn_7001e1e9c0 as *const (), 0), // +0xa9c0 _Temp_get
    Lifted::new(0x7001e1ea30, fn_7001e1ea30 as *const (), 0), // +0xaa30 _To_byte
    Lifted::new(0x7001e1ea80, fn_7001e1ea80 as *const (), 0), // +0xaa80 _To_wide
    Lifted::new(0x7001e1ead0, tpl_57::<0x7001e69388, 0x7001e69260> as *const (), 0), // +0xaad0 _Unlink
    Lifted::new(0x7001e1eaf0, tpl_58::<0x7001e97190, 0x7001e971e0, 0x7001e6be40, 0x7001e69518, 0x2, 0x7001e69510, 0x7001e69598> as *const (), 0), // +0xaaf0 ?_Fiopen@std@@YAPEAU_iobuf@@PEBDHH@Z
    Lifted::alias(0x7001e1ebb0, tpl_58::<0x7001e971e0, 0x7001e97230, 0x7001e6bf30, 0x7001e69520, 0x2, 0x7001e69510, 0x7001e69598> as *const (), 0, 0xabc0), // +0xabb0 ?_Fiopen@std@@YAPEAU_iobuf@@PEBGHH@Z (thunk -> +0xabc0)
    Lifted::new(0x7001e1ebc0, tpl_58::<0x7001e971e0, 0x7001e97230, 0x7001e6bf30, 0x7001e69520, 0x2, 0x7001e69510, 0x7001e69598> as *const (), 0), // +0xabc0 ?_Fiopen@std@@YAPEAU_iobuf@@PEB_WHH@Z
    Lifted::new(0x7001e1ec7c, fn_7001e1ec7c as *const (), 0), // +0xac7c sub_ac7c
    Lifted::new(0x7001e1ec9c, tpl_22::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6c0d8, 0x18> as *const (), 2), // +0xac9c sub_ac9c
    Lifted::new(0x7001e1ecec, fn_7001e1ecec as *const (), 2), // +0xacec sub_acec
    Lifted::new(0x7001e1ed4c, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6c060> as *const (), 2), // +0xad4c sub_ad4c
    Lifted::new(0x7001e1ed90, fn_7001e1ed90 as *const (), 0), // +0xad90 ?_Rethrow_future_exception@std@@YAXVexception_ptr@1@@Z
    Lifted::new(0x7001e1edb0, fn_7001e1edb0 as *const (), 2), // +0xadb0 ?_Throw_future_error@std@@YAXAEBVerror_code@1@@Z
    Lifted::new(0x7001e1ede0, fn_7001e1ede0 as *const (), 0), // +0xade0 sub_ade0
    Lifted::new(0x7001e1edf0, fn_7001e1edf0 as *const (), 1), // +0xadf0 sub_adf0
    Lifted::new(0x7001e1ee30, fn_7001e1ee30 as *const (), 1), // +0xae30 ?resetiosflags@std@@YA?AU?$_Smanip@H@1@H@Z
    Lifted::new(0x7001e1ee50, fn_7001e1ee50 as *const (), 1), // +0xae50 sub_ae50
    Lifted::new(0x7001e1ee90, tpl_25::<0x7001e1ee50, 0x8> as *const (), 1), // +0xae90 ?setbase@std@@YA?AU?$_Smanip@H@1@H@Z
    Lifted::new(0x7001e1eeb0, tpl_25::<0x7001e1ef10, 0x8> as *const (), 1), // +0xaeb0 ?setiosflags@std@@YA?AU?$_Smanip@H@1@H@Z
    Lifted::new(0x7001e1eed0, fn_7001e1eed0 as *const (), 1), // +0xaed0 ?setprecision@std@@YA?AU?$_Smanip@_J@1@_J@Z
    Lifted::new(0x7001e1eef0, fn_7001e1eef0 as *const (), 1), // +0xaef0 ?setw@std@@YA?AU?$_Smanip@_J@1@_J@Z
    Lifted::new(0x7001e1ef10, fn_7001e1ef10 as *const (), 1), // +0xaf10 sub_af10
    Lifted::new(0x7001e1ef30, tpl_59::<0x20> as *const (), 1), // +0xaf30 sub_af30
    Lifted::new(0x7001e1ef40, tpl_59::<0x28> as *const (), 1), // +0xaf40 sub_af40
    Lifted::new(0x7001e1ef50, fn_7001e1ef50 as *const (), 0), // +0xaf50 ?_Addstd@ios_base@std@@SAXPEAV12@@Z
    Lifted::new(0x7001e1efd0, fn_7001e1efd0 as *const (), 0), // +0xafd0 ?_Ios_base_dtor@ios_base@std@@CAXPEAV12@@Z
    Lifted::new(0x7001e1f050, fn_7001e1f050 as *const (), 0), // +0xb050 ?_Atexit@@YAXP6AXXZ@Z
    Lifted::new(0x7001e1f08c, fn_7001e1f08c as *const (), 0), // +0xb08c sub_b08c
    Lifted::new(0x7001e1f118, fn_7001e1f118 as *const (), 0), // +0xb118 sub_b118
    Lifted::new(0x7001e1f144, fn_7001e1f144 as *const (), 0), // +0xb144 sub_b144
    Lifted::new(0x7001e1f190, tpl_34::<0x7001e97240> as *const (), 1), // +0xb190 ?_Init_cnt_func@Init@ios_base@std@@CAAEAHXZ
    Lifted::new(0x7001e1f1a0, fn_7001e1f1a0 as *const (), 1), // +0xb1a0 ?_Init_ctor@Init@ios_base@std@@CAXPEAV123@@Z
    Lifted::new(0x7001e1f1c0, fn_7001e1f1c0 as *const (), 0), // +0xb1c0 ?_Init_dtor@Init@ios_base@std@@CAXPEAV123@@Z
    Lifted::new(0x7001e1f210, fn_7001e1f210 as *const (), 0), // +0xb210 ?_Osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e1f2c0, fn_7001e1f2c0 as *const (), 0), // +0xb2c0 ?flush@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ
    Lifted::new(0x7001e1f410, tpl_41::<0x68, 0x7001e696a0> as *const (), 0), // +0xb410 ?pubsync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e1f420, tpl_55::<0x48> as *const (), 1), // +0xb420 ?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ
    Lifted::new(0x7001e1f430, tpl_55::<0x50> as *const (), 1), // +0xb430 ?tie@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_ostream@DU?$char_traits@D@std@@@2@XZ
    Lifted::new(0x7001e1f438, fn_7001e1f438 as *const (), 0), // +0xb438 sub_b438
    Lifted::new(0x7001e1f518, fn_7001e1f518 as *const (), 2), // +0xb518 sub_b518
    Lifted::new(0x7001e1f9b8, fn_7001e1f9b8 as *const (), 2), // +0xb9b8 sub_b9b8
    Lifted::new(0x7001e1fcdc, fn_7001e1fcdc as *const (), 0), // +0xbcdc sub_bcdc
    Lifted::new(0x7001e1fd44, fn_7001e1fd44 as *const (), 2), // +0xbd44 sub_bd44
    Lifted::new(0x7001e2132c, fn_7001e2132c as *const (), 2), // +0xd32c sub_d32c
    Lifted::new(0x7001e21da4, fn_7001e21da4 as *const (), 0), // +0xdda4 sub_dda4
    Lifted::new(0x7001e21f00, fn_7001e21f00 as *const (), 0), // +0xdf00 sub_df00
    Lifted::new(0x7001e22084, fn_7001e22084 as *const (), 0), // +0xe084 sub_e084
    Lifted::new(0x7001e2219c, fn_7001e2219c as *const (), 0), // +0xe19c sub_e19c
    Lifted::new(0x7001e222b4, fn_7001e222b4 as *const (), 0), // +0xe2b4 sub_e2b4
    Lifted::new(0x7001e223d0, tpl_17::<0x7001e6c0f8, 0x8> as *const (), 1), // +0xe3d0 ??0?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e223f0, tpl_17::<0x7001e6c170, 0x8> as *const (), 1), // +0xe3f0 ??0?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e22404, tpl_60::<0x10, 0x7001e693a8, 0x20, 0x28> as *const (), 0), // +0xe404 sub_e404
    Lifted::new(0x7001e22438, fn_7001e22438 as *const (), 2), // +0xe438 sub_e438
    Lifted::new(0x7001e224c4, fn_7001e224c4 as *const (), 0), // +0xe4c4 sub_e4c4
    Lifted::new(0x7001e22540, tpl_61::<0x7001e6c1d0, 0x10, 0x7001e693a8, 0x20, 0x28, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0xe540 sub_e540
    Lifted::new(0x7001e225b0, fn_7001e225b0 as *const (), 1), // +0xe5b0 ?_Ffmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADDH@Z
    Lifted::new(0x7001e22650, fn_7001e22650 as *const (), 0), // +0xe650 ?_Getcat@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e22710, fn_7001e22710 as *const (), 0), // +0xe710 ?_Getcat@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e227cc, fn_7001e227cc as *const (), 0), // +0xe7cc sub_e7cc
    Lifted::new(0x7001e228a0, fn_7001e228a0 as *const (), 1), // +0xe8a0 ?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z
    Lifted::new(0x7001e2291c, fn_7001e2291c as *const (), 0), // +0xe91c sub_e91c
    Lifted::new(0x7001e22980, fn_7001e22980 as *const (), 0), // +0xe980 sub_e980
    Lifted::new(0x7001e22a60, fn_7001e22a60 as *const (), 2), // +0xea60 ?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits
    Lifted::new(0x7001e22e90, fn_7001e22e90 as *const (), 0), // +0xee90 ?_Locimp_Addfac@_Locimp@locale@std@@CAXPEAV123@PEAVfacet@23@_K@Z
    Lifted::new(0x7001e22f90, fn_7001e22f90 as *const (), 0), // +0xef90 ?_Locimp_ctor@_Locimp@locale@std@@CAXPEAV123@AEBV123@@Z
    Lifted::new(0x7001e23070, fn_7001e23070 as *const (), 0), // +0xf070 ?_Locinfo_Addcats@_Locinfo@std@@SAAEAV12@PEAV12@HPEBD@Z
    Lifted::new(0x7001e23140, fn_7001e23140 as *const (), 0), // +0xf140 ?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@HPEBD@Z
    Lifted::new(0x7001e231a0, fn_7001e231a0 as *const (), 2), // +0xf1a0 ?_Makeloc@_Locimp@locale@std@@CAPEAV123@AEBV_Locinfo@3@HPEAV123@PEBV23@@Z
    Lifted::new(0x7001e23690, fn_7001e23690 as *const (), 2), // +0xf690 ?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@
    Lifted::new(0x7001e236f0, fn_7001e236f0 as *const (), 2), // +0xf6f0 ?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@
    Lifted::new(0x7001e23748, fn_7001e23748 as *const (), 0), // +0xf748 sub_f748
    Lifted::new(0x7001e237d0, tpl_62::<0x18> as *const (), 1), // +0xf7d0 sub_f7d0
    Lifted::new(0x7001e237e0, tpl_63::<0x20, 0x10, 0x18, 0x7001e174fc> as *const (), 2), // +0xf7e0 sub_f7e0
    Lifted::new(0x7001e23820, fn_7001e23820 as *const (), 2), // +0xf820 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23990, tpl_64::<0x7001e23b30> as *const (), 2), // +0xf990 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23a10, fn_7001e23a10 as *const (), 2), // +0xfa10 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23b30, fn_7001e23b30 as *const (), 2), // +0xfb30 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23c50, fn_7001e23c50 as *const (), 2), // +0xfc50 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23da0, fn_7001e23da0 as *const (), 2), // +0xfda0 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23ef0, tpl_65::<0x7001e23da0> as *const (), 2), // +0xfef0 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e23f80, fn_7001e23f80 as *const (), 2), // +0xff80 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e240d0, fn_7001e240d0 as *const (), 2), // +0x100d0 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e241f0, fn_7001e241f0 as *const (), 2), // +0x101f0 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24310, fn_7001e24310 as *const (), 2), // +0x10310 ?do_get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24680, fn_7001e24680 as *const (), 2), // +0x10680 sub_10680
    Lifted::new(0x7001e246c0, fn_7001e246c0 as *const (), 2), // +0x106c0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e247d0, fn_7001e247d0 as *const (), 2), // +0x107d0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e248e0, fn_7001e248e0 as *const (), 2), // +0x108e0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24af0, fn_7001e24af0 as *const (), 2), // +0x10af0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24d00, fn_7001e24d00 as *const (), 2), // +0x10d00 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24db0, fn_7001e24db0 as *const (), 2), // +0x10db0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24ed0, fn_7001e24ed0 as *const (), 2), // +0x10ed0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e24ff0, fn_7001e24ff0 as *const (), 2), // +0x10ff0 ?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trait
    Lifted::new(0x7001e25250, tpl_62::<0x19> as *const (), 1), // +0x11250 sub_11250
    Lifted::new(0x7001e25260, tpl_63::<0x28, 0x10, 0x18, 0x7001e174fc> as *const (), 2), // +0x11260 sub_11260
    Lifted::new(0x7001e2529c, fn_7001e2529c as *const (), 0), // +0x1129c sub_1129c
    Lifted::new(0x7001e25370, fn_7001e25370 as *const (), 0), // +0x11370 ?global@locale@std@@SA?AV12@AEBV12@@Z
    Lifted::new(0x7001e25470, fn_7001e25470 as *const (), 0), // +0x11470 sub_11470
    Lifted::new(0x7001e25520, fn_7001e25520 as *const (), 0), // +0x11520 ?sbumpc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e25560, fn_7001e25560 as *const (), 0), // +0x11560 ?sgetc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e25590, fn_7001e25590 as *const (), 0), // +0x11590 ?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z
    Lifted::new(0x7001e255ec, tpl_34::<0x7001e99160> as *const (), 1), // +0x115ec sub_115ec
    Lifted::new(0x7001e255f4, fn_7001e255f4 as *const (), 0), // +0x115f4 sub_115f4
    Lifted::new(0x7001e25648, fn_7001e25648 as *const (), 0), // +0x11648 sub_11648
    Lifted::new(0x7001e25680, fn_7001e25680 as *const (), 1), // +0x11680 ?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ
    Lifted::new(0x7001e25690, fn_7001e25690 as *const (), 0), // +0x11690 ?_Init@locale@std@@CAPEAV_Locimp@12@_N@Z
    Lifted::new(0x7001e25730, fn_7001e25730 as *const (), 0), // +0x11730 ?_Locimp_dtor@_Locimp@locale@std@@CAXPEAV123@@Z
    Lifted::new(0x7001e257b0, fn_7001e257b0 as *const (), 0), // +0x117b0 ?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@PEBD@Z
    Lifted::new(0x7001e25820, fn_7001e25820 as *const (), 0), // +0x11820 ?_Locinfo_dtor@_Locinfo@std@@SAXPEAV12@@Z
    Lifted::new(0x7001e25840, fn_7001e25840 as *const (), 0), // +0x11840 ?_New_Locimp@_Locimp@locale@std@@CAPEAV123@AEBV123@@Z
    Lifted::new(0x7001e25880, fn_7001e25880 as *const (), 0), // +0x11880 ?_New_Locimp@_Locimp@locale@std@@CAPEAV123@_N@Z
    Lifted::new(0x7001e258f0, fn_7001e258f0 as *const (), 0), // +0x118f0 ?_Setgloballocale@locale@std@@CAXPEAX@Z
    Lifted::new(0x7001e25930, fn_7001e25930 as *const (), 0), // +0x11930 ?classic@locale@std@@SAAEBV12@XZ
    Lifted::new(0x7001e25960, fn_7001e25960 as *const (), 0), // +0x11960 ?empty@locale@std@@SA?AV12@XZ
    Lifted::new(0x7001e25984, fn_7001e25984 as *const (), 0), // +0x11984 sub_11984
    Lifted::new(0x7001e259c0, fn_7001e259c0 as *const (), 0), // +0x119c0 sub_119c0
    Lifted::new(0x7001e25a00, fn_7001e25a00 as *const (), 0), // +0x11a00 ?_MP_Add@std@@YAXQEA_K_K@Z
    Lifted::new(0x7001e25a30, fn_7001e25a30 as *const (), 1), // +0x11a30 ?_MP_Get@std@@YA_KQEA_K@Z
    Lifted::new(0x7001e25a40, fn_7001e25a40 as *const (), 3), // +0x11a40 ?_MP_Mul@std@@YAXQEA_K_K1@Z
    Lifted::new(0x7001e25af0, fn_7001e25af0 as *const (), 0), // +0x11af0 ?_MP_Rem@std@@YAXQEA_K_K@Z
    Lifted::new(0x7001e25d74, fn_7001e25d74 as *const (), 1), // +0x11d74 sub_11d74
    Lifted::new(0x7001e25e00, fn_7001e25e00 as *const (), 0), // +0x11e00 sub_11e00
    Lifted::new(0x7001e25e20, fn_7001e25e20 as *const (), 0), // +0x11e20 ?get_new_handler@std@@YAP6AXXZXZ
    Lifted::new(0x7001e25e50, fn_7001e25e50 as *const (), 0), // +0x11e50 ?set_new_handler@std@@YAP6AXXZP6AXXZ@Z
    Lifted::new(0x7001e25eb0, fn_7001e25eb0 as *const (), 1), // +0x11eb0 ?_Syserror_map@std@@YAPEBDH@Z
    Lifted::new(0x7001e25ee0, fn_7001e25ee0 as *const (), 1), // +0x11ee0 ?_Winerror_map@std@@YAHH@Z
    Lifted::new(0x7001e25f10, fn_7001e25f10 as *const (), 0), // +0x11f10 ?_Winerror_message@std@@YAKKPEADK@Z
    Lifted::new(0x7001e25f6c, fn_7001e25f6c as *const (), 0), // +0x11f6c sub_11f6c
    Lifted::new(0x7001e26010, tpl_66::<0x7001e69218> as *const (), 0), // +0x12010 _Cnd_broadcast
    Lifted::new(0x7001e26030, fn_7001e26030 as *const (), 0), // +0x12030 _Cnd_destroy
    Lifted::new(0x7001e26050, fn_7001e26050 as *const (), 0), // +0x12050 _Cnd_init
    Lifted::new(0x7001e26090, fn_7001e26090 as *const (), 1), // +0x12090 _Cnd_init_in_situ
    Lifted::new(0x7001e260a0, tpl_66::<0x7001e69220> as *const (), 0), // +0x120a0 _Cnd_signal
    Lifted::new(0x7001e260c0, fn_7001e260c0 as *const (), 0), // +0x120c0 _Cnd_timedwait
    Lifted::new(0x7001e26190, fn_7001e26190 as *const (), 0), // +0x12190 _Cnd_wait
    Lifted::new(0x7001e261e0, fn_7001e261e0 as *const (), 2), // +0x121e0 sub_121e0
    Lifted::new(0x7001e26260, fn_7001e26260 as *const (), 0), // +0x12260 _Thrd_create
    Lifted::new(0x7001e26320, fn_7001e26320 as *const (), 0), // +0x12320 _Thrd_current
    Lifted::new(0x7001e26340, fn_7001e26340 as *const (), 0), // +0x12340 _Thrd_detach
    Lifted::new(0x7001e26360, fn_7001e26360 as *const (), 1), // +0x12360 _Thrd_equal
    Lifted::new(0x7001e26370, tpl_15::<0x7001e694a8> as *const (), 2), // +0x12370 _Thrd_exit
    Lifted::new(0x7001e26380, fn_7001e26380 as *const (), 0), // +0x12380 _Thrd_hardware_concurrency
    Lifted::new(0x7001e263a0, tpl_15::<0x7001e691f0> as *const (), 2), // +0x123a0 ?GetCurrentThreadId@platform@details@Concurrency@@YAJXZ
    Lifted::new(0x7001e263b0, fn_7001e263b0 as *const (), 0), // +0x123b0 _Thrd_join
    Lifted::new(0x7001e26420, fn_7001e26420 as *const (), 0), // +0x12420 _Thrd_sleep
    Lifted::new(0x7001e26490, fn_7001e26490 as *const (), 0), // +0x12490 _Thrd_start
    Lifted::new(0x7001e264d0, tpl_15::<0x7001e691f8> as *const (), 2), // +0x124d0 _Thrd_yield
    Lifted::new(0x7001e264e0, fn_7001e264e0 as *const (), 1), // +0x124e0 _Mtx_clear_owner
    Lifted::new(0x7001e264f0, fn_7001e264f0 as *const (), 0), // +0x124f0 _Mtx_current_owns
    Lifted::new(0x7001e26530, tpl_67::<0x8> as *const (), 1), // +0x12530 _Mtx_getconcrtcs
    Lifted::new(0x7001e26540, fn_7001e26540 as *const (), 0), // +0x12540 _Mtx_init
    Lifted::new(0x7001e265a0, fn_7001e265a0 as *const (), 1), // +0x125a0 _Mtx_init_in_situ
    Lifted::new(0x7001e265c0, tpl_13::<0x7001e26690> as *const (), 0), // +0x125c0 _Mtx_lock
    Lifted::new(0x7001e265d0, fn_7001e265d0 as *const (), 0), // +0x125d0 _Mtx_reset_owner
    Lifted::new(0x7001e265f0, fn_7001e265f0 as *const (), 0), // +0x125f0 _Mtx_timedlock
    Lifted::new(0x7001e26610, fn_7001e26610 as *const (), 0), // +0x12610 _Mtx_trylock
    Lifted::new(0x7001e26630, fn_7001e26630 as *const (), 0), // +0x12630 _Mtx_unlock
    Lifted::new(0x7001e26650, fn_7001e26650 as *const (), 0), // +0x12650 ?_Rng_abort@std@@YAXPEBD@Z
    Lifted::new(0x7001e26690, fn_7001e26690 as *const (), 0), // +0x12690 sub_12690
    Lifted::new(0x7001e267c4, fn_7001e267c4 as *const (), 0), // +0x127c4 sub_127c4
    Lifted::new(0x7001e26800, fn_7001e26800 as *const (), 0), // +0x12800 ?CaptureCallstack@platform@details@Concurrency@@YA_KPEAPEAX_K1@Z
    Lifted::new(0x7001e26820, fn_7001e26820 as *const (), 1), // +0x12820 ?GetNextAsyncId@platform@details@Concurrency@@YAIXZ
    Lifted::new(0x7001e26830, fn_7001e26830 as *const (), 0), // +0x12830 ?_CallInContext@_ContextCallback@details@Concurrency@@QEBAXV?$function@$$A6AXXZ@std@@_N@Z
    Lifted::new(0x7001e26890, fn_7001e26890 as *const (), 0), // +0x12890 ?_ReportUnobservedException@details@Concurrency@@YAXXZ
    Lifted::new(0x7001e268c0, fn_7001e268c0 as *const (), 0), // +0x128c0 ?_Release_chore@details@Concurrency@@YAXPEAU_Threadpool_chore@12@@Z
    Lifted::new(0x7001e268f0, fn_7001e268f0 as *const (), 0), // +0x128f0 ?_Reschedule_chore@details@Concurrency@@YAHPEBU_Threadpool_chore@12@@Z
    Lifted::new(0x7001e26920, fn_7001e26920 as *const (), 0), // +0x12920 ?_Schedule_chore@details@Concurrency@@YAHPEAU_Threadpool_chore@12@@Z
    Lifted::new(0x7001e26960, fn_7001e26960 as *const (), 0), // +0x12960 sub_12960
    Lifted::new(0x7001e269c0, fn_7001e269c0 as *const (), 0), // +0x129c0 _Cnd_do_broadcast_at_thread_exit
    Lifted::new(0x7001e26a70, fn_7001e26a70 as *const (), 0), // +0x12a70 _Cnd_register_at_thread_exit
    Lifted::new(0x7001e26b30, fn_7001e26b30 as *const (), 0), // +0x12b30 _Cnd_unregister_at_thread_exit
    Lifted::new(0x7001e26ba0, fn_7001e26ba0 as *const (), 0), // +0x12ba0 _Query_perf_counter
    Lifted::new(0x7001e26bc0, fn_7001e26bc0 as *const (), 0), // +0x12bc0 _Query_perf_frequency
    Lifted::new(0x7001e26bec, fn_7001e26bec as *const (), 0), // +0x12bec sub_12bec
    Lifted::new(0x7001e26c30, fn_7001e26c30 as *const (), 0), // +0x12c30 _Xtime_diff_to_millis
    Lifted::new(0x7001e26c80, fn_7001e26c80 as *const (), 3), // +0x12c80 _Xtime_diff_to_millis2
    Lifted::new(0x7001e26d50, fn_7001e26d50 as *const (), 0), // +0x12d50 _Xtime_get_ticks
    Lifted::new(0x7001e26d80, fn_7001e26d80 as *const (), 0), // +0x12d80 xtime_get
    Lifted::new(0x7001e26da4, fn_7001e26da4 as *const (), 2), // +0x12da4 sub_12da4
    Lifted::new(0x7001e26e90, fn_7001e26e90 as *const (), 0), // +0x12e90 ?_Throw_C_error@std@@YAXH@Z
    Lifted::new(0x7001e26ed0, fn_7001e26ed0 as *const (), 0), // +0x12ed0 ?_Throw_Cpp_error@std@@YAXH@Z
    Lifted::new(0x7001e26f18, tpl_34::<0x7001e97248> as *const (), 1), // +0x12f18 sub_12f18
    Lifted::new(0x7001e26f20, fn_7001e26f20 as *const (), 2), // +0x12f20 sub_12f20
    Lifted::new(0x7001e26f70, tpl_34::<0x7001e6d2a8> as *const (), 1), // +0x12f70 sub_12f70
    Lifted::new(0x7001e26f80, tpl_12::<0x7001e66478> as *const (), 2), // +0x12f80 ?uncaught_exception@std@@YA_NXZ
    Lifted::new(0x7001e26f90, tpl_12::<0x7001e6647e> as *const (), 2), // +0x12f90 ?uncaught_exceptions@std@@YAHXZ
    Lifted::new(0x7001e26f98, fn_7001e26f98 as *const (), 0), // +0x12f98 sub_12f98
    Lifted::new(0x7001e270b0, fn_7001e270b0 as *const (), 0), // +0x130b0 sub_130b0
    Lifted::new(0x7001e271c8, tpl_68::<0x7001e27340, 0x7001e6d338, 0x8, 0x7c, 0x4c, 0x58, 0x10, 0x72, 0x28, 0x18, 0x30, 0x20, 0x7001e99678> as *const (), 0), // +0x131c8 sub_131c8
    Lifted::new(0x7001e27260, tpl_69::<0x7001e6d3b8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const (), 1), // +0x13260 ??0?$basic_ios@GU?$char_traits@G@std@@@std@@IEAA@XZ
    Lifted::new(0x7001e272a0, fn_7001e272a0 as *const (), 0), // +0x132a0 ??0?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAA@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@_N@Z
    Lifted::new(0x7001e27340, fn_7001e27340 as *const (), 0), // +0x13340 ??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@XZ
    Lifted::new(0x7001e27428, fn_7001e27428 as *const (), 0), // +0x13428 sub_13428
    Lifted::new(0x7001e274e0, tpl_27::<0xfffffffffffffff0, 0x4, 0x7001e6d3c8, 0xffffffffffffffec> as *const (), 1), // +0x134e0 ??1?$basic_ostream@GU?$char_traits@G@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e27510, tpl_28::<0x60, 0x7001e6d2b8, 0x8, 0x10, 0x7001e696a0, 0x7001e6649c> as *const (), 0), // +0x13510 ??1?$basic_streambuf@GU?$char_traits@G@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e27570, fn_7001e27570 as *const (), 0), // +0x13570 ??_D?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e275b0, tpl_30::<0xfffffffffffffffc, 0x7001e5b68c> as *const (), 0), // +0x135b0 sub_135b0
    Lifted::new(0x7001e275c0, tpl_32::<0x7001e27428, 0x7001e6649c> as *const (), 0), // +0x135c0 sub_135c0
    Lifted::new(0x7001e275f4, fn_7001e275f4 as *const (), 0), // +0x135f4 sub_135f4
    Lifted::new(0x7001e276d0, fn_7001e276d0 as *const (), 1), // +0x136d0 ?_Gndec@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAPEAGXZ
    Lifted::new(0x7001e276f0, tpl_37::<0x50, 0x38, 0x2> as *const (), 1), // +0x136f0 ?_Gninc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAPEAGXZ
    Lifted::new(0x7001e27710, tpl_37::<0x58, 0x40, 0x2> as *const (), 1), // +0x13710 ?_Pninc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAPEAGXZ
    Lifted::new(0x7001e27728, tpl_70::<0x7001e275f4, 0x7001e69598, 0x7001e99678> as *const (), 0), // +0x13728 sub_13728
    Lifted::new(0x7001e27830, tpl_71::<0x50, 0x38> as *const (), 1), // +0x13830 ?egptr@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEBAPEAGXZ
    Lifted::new(0x7001e27850, tpl_71::<0x58, 0x40> as *const (), 1), // +0x13850 ?epptr@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEBAPEAGXZ
    Lifted::new(0x7001e27870, tpl_72::<0x50, 0x38> as *const (), 1), // +0x13870 ?gbump@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAXH@Z
    Lifted::new(0x7001e27890, tpl_50::<0x7001e26f98, 0x7001e1a7e0> as *const (), 0), // +0x13890 sub_13890
    Lifted::new(0x7001e278c0, tpl_73::<0x7001e1a770, 0x50, 0x48, 0x7001e282b0, 0x58, 0x10, 0x14, 0x7001e1ef50, 0x7001e6bc10, 0x7001e6bc28, 0x7001e6bc40, 0x7001e1c690, 0x7001e1887c, 0x7001e76d68, 0x7001e66444, 0x7001e2797a> as *const (), 0), // +0x138c0 ?init@?$basic_ios@GU?$char_traits@G@std@@@std@@IEAAXPEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@_N@Z
    Lifted::new(0x7001e27980, fn_7001e27980 as *const (), 0), // +0x13980 sub_13980
    Lifted::new(0x7001e27b20, fn_7001e27b20 as *const (), 1), // +0x13b20 ?overflow@?$basic_streambuf@GU?$char_traits@G@std@@@std@@MEAAGG@Z
    Lifted::new(0x7001e27b30, fn_7001e27b30 as *const (), 0), // +0x13b30 sub_13b30
    Lifted::new(0x7001e27c10, tpl_72::<0x58, 0x40> as *const (), 1), // +0x13c10 ?pbump@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAXH@Z
    Lifted::new(0x7001e27c30, fn_7001e27c30 as *const (), 0), // +0x13c30 sub_13c30
    Lifted::new(0x7001e27d20, fn_7001e27d20 as *const (), 0), // +0x13d20 sub_13d20
    Lifted::new(0x7001e27de0, tpl_74::<0x4, 0x80, 0x7001e69580, 0x8, 0x72, 0x10, 0x7c, 0x4c, 0x58, 0x28, 0x18, 0x30, 0x7001e99678> as *const (), 0), // +0x13de0 sub_13de0
    Lifted::new(0x7001e27e90, tpl_75::<0x18, 0x38, 0x50> as *const (), 1), // +0x13e90 ?setg@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAXPEAG00@Z
    Lifted::new(0x7001e27eb0, fn_7001e27eb0 as *const (), 1), // +0x13eb0 ?setp@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAXPEAG0@Z
    Lifted::new(0x7001e27ed0, fn_7001e27ed0 as *const (), 0), // +0x13ed0 sub_13ed0
    Lifted::new(0x7001e27f30, fn_7001e27f30 as *const (), 2), // +0x13f30 sub_13f30
    Lifted::new(0x7001e28200, fn_7001e28200 as *const (), 0), // +0x14200 ?uflow@?$basic_streambuf@GU?$char_traits@G@std@@@std@@MEAAGXZ
    Lifted::new(0x7001e28240, fn_7001e28240 as *const (), 0), // +0x14240 sub_14240
    Lifted::new(0x7001e282b0, fn_7001e282b0 as *const (), 0), // +0x142b0 ?widen@?$basic_ios@GU?$char_traits@G@std@@@std@@QEBAGD@Z
    Lifted::alias(0x7001e28340, fn_7001e28350 as *const (), 0, 0x14350), // +0x14340 sub_14340 (thunk -> +0x14350)
    Lifted::new(0x7001e28350, fn_7001e28350 as *const (), 0), // +0x14350 ?xsgetn@?$basic_streambuf@GU?$char_traits@G@std@@@std@@MEAA_JPEAG_J@Z
    Lifted::alias(0x7001e28420, fn_7001e28430 as *const (), 0, 0x14430), // +0x14420 sub_14420 (thunk -> +0x14430)
    Lifted::new(0x7001e28430, fn_7001e28430 as *const (), 0), // +0x14430 ?xsputn@?$basic_streambuf@GU?$char_traits@G@std@@@std@@MEAA_JPEBG_J@Z
    Lifted::new(0x7001e28510, fn_7001e28510 as *const (), 0), // +0x14510 ??0?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@_N@Z
    Lifted::new(0x7001e285c0, tpl_27::<0xffffffffffffffe8, 0x4, 0x7001e6d3e0, 0xffffffffffffffe4> as *const (), 1), // +0x145c0 ??1?$basic_istream@GU?$char_traits@G@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e285f0, fn_7001e285f0 as *const (), 0), // +0x145f0 ??_D?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e28630, tpl_30::<0xfffffffffffffffc, 0x7001e5b47c> as *const (), 0), // +0x14630 sub_14630
    Lifted::new(0x7001e28640, tpl_76::<0x7001e97258> as *const (), 1), // +0x14640 ??0_UShinit@std@@QEAA@XZ
    Lifted::new(0x7001e28660, fn_7001e28660 as *const (), 0), // +0x14660 ??1_UShinit@std@@QEAA@XZ
    Lifted::new(0x7001e286a8, fn_7001e286a8 as *const (), 0), // +0x146a8 sub_146a8
    Lifted::new(0x7001e287c0, fn_7001e287c0 as *const (), 0), // +0x147c0 sub_147c0
    Lifted::new(0x7001e288d8, tpl_68::<0x7001e28a50, 0x7001e6d478, 0x8, 0x7c, 0x4c, 0x58, 0x10, 0x72, 0x28, 0x18, 0x30, 0x20, 0x7001e99b38> as *const (), 0), // +0x148d8 sub_148d8
    Lifted::new(0x7001e28970, tpl_69::<0x7001e6d4f8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const (), 1), // +0x14970 ??0?$basic_ios@_WU?$char_traits@_W@std@@@std@@IEAA@XZ
    Lifted::new(0x7001e289b0, fn_7001e289b0 as *const (), 0), // +0x149b0 ??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@_N@Z
    Lifted::new(0x7001e28a50, fn_7001e28a50 as *const (), 0), // +0x14a50 ??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@XZ
    Lifted::new(0x7001e28b38, fn_7001e28b38 as *const (), 0), // +0x14b38 sub_14b38
    Lifted::new(0x7001e28bf0, tpl_27::<0xfffffffffffffff0, 0x4, 0x7001e6d508, 0xffffffffffffffec> as *const (), 1), // +0x14bf0 ??1?$basic_ostream@_WU?$char_traits@_W@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e28c20, tpl_28::<0x60, 0x7001e6d3f8, 0x8, 0x10, 0x7001e696a0, 0x7001e6649c> as *const (), 0), // +0x14c20 ??1?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e28c80, fn_7001e28c80 as *const (), 0), // +0x14c80 ??_D?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e28cc0, tpl_30::<0xfffffffffffffffc, 0x7001e5b73c> as *const (), 0), // +0x14cc0 sub_14cc0
    Lifted::new(0x7001e28cd0, tpl_32::<0x7001e28b38, 0x7001e6649c> as *const (), 0), // +0x14cd0 sub_14cd0
    Lifted::new(0x7001e28d04, tpl_70::<0x7001e275f4, 0x7001e69598, 0x7001e99b38> as *const (), 0), // +0x14d04 sub_14d04
    Lifted::new(0x7001e28e10, tpl_50::<0x7001e286a8, 0x7001e1a7e0> as *const (), 0), // +0x14e10 sub_14e10
    Lifted::new(0x7001e28e40, tpl_73::<0x7001e1a770, 0x50, 0x48, 0x7001e28fb0, 0x58, 0x10, 0x14, 0x7001e1ef50, 0x7001e6bc10, 0x7001e6bc28, 0x7001e6bc40, 0x7001e1c690, 0x7001e1887c, 0x7001e76d68, 0x7001e66444, 0x7001e28efa> as *const (), 0), // +0x14e40 ?init@?$basic_ios@_WU?$char_traits@_W@std@@@std@@IEAAXPEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@_N@Z
    Lifted::new(0x7001e28f00, tpl_74::<0x4, 0x80, 0x7001e69580, 0x8, 0x72, 0x10, 0x7c, 0x4c, 0x58, 0x28, 0x18, 0x30, 0x7001e99b38> as *const (), 0), // +0x14f00 sub_14f00
    Lifted::new(0x7001e28fb0, fn_7001e28fb0 as *const (), 0), // +0x14fb0 ?widen@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEBA_WD@Z
    Lifted::new(0x7001e29040, fn_7001e29040 as *const (), 0), // +0x15040 ??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@_N@Z
    Lifted::new(0x7001e290f0, tpl_27::<0xffffffffffffffe8, 0x4, 0x7001e6d520, 0xffffffffffffffe4> as *const (), 1), // +0x150f0 ??1?$basic_istream@_WU?$char_traits@_W@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e29120, fn_7001e29120 as *const (), 0), // +0x15120 ??_D?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e29160, tpl_30::<0xfffffffffffffffc, 0x7001e5b52c> as *const (), 0), // +0x15160 sub_15160
    Lifted::new(0x7001e29170, tpl_76::<0x7001e9725c> as *const (), 1), // +0x15170 ??0_Winit@std@@QEAA@XZ
    Lifted::new(0x7001e29190, fn_7001e29190 as *const (), 0), // +0x15190 ??1_Winit@std@@QEAA@XZ
    Lifted::new(0x7001e291d8, fn_7001e291d8 as *const (), 2), // +0x151d8 sub_151d8
    Lifted::new(0x7001e292c0, fn_7001e292c0 as *const (), 0), // +0x152c0 sub_152c0
    Lifted::new(0x7001e293c8, fn_7001e293c8 as *const (), 2), // +0x153c8 sub_153c8
    Lifted::new(0x7001e29480, fn_7001e29480 as *const (), 2), // +0x15480 sub_15480
    Lifted::new(0x7001e29900, fn_7001e29900 as *const (), 2), // +0x15900 sub_15900
    Lifted::new(0x7001e29d80, fn_7001e29d80 as *const (), 0), // +0x15d80 sub_15d80
    Lifted::new(0x7001e2a2a8, fn_7001e2a2a8 as *const (), 2), // +0x162a8 sub_162a8
    Lifted::new(0x7001e2a5dc, fn_7001e2a5dc as *const (), 2), // +0x165dc sub_165dc
    Lifted::new(0x7001e2a934, fn_7001e2a934 as *const (), 2), // +0x16934 sub_16934
    Lifted::new(0x7001e2ac8c, fn_7001e2ac8c as *const (), 0), // +0x16c8c sub_16c8c
    Lifted::new(0x7001e2ad7c, fn_7001e2ad7c as *const (), 0), // +0x16d7c sub_16d7c
    Lifted::new(0x7001e2ae08, tpl_77::<0x7001e4d900, 0x7001e69660, 0x7001e18ef0, 0x7001e693a8, 0x7001e6b5c0, 0x7001e2b09c, 0x7001e69670, 0x7001e6b620, 0x7001e6dcc0> as *const (), 2), // +0x16e08 sub_16e08
    Lifted::new(0x7001e2af10, fn_7001e2af10 as *const (), 2), // +0x16f10 ??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z
    Lifted::new(0x7001e2b02c, fn_7001e2b02c as *const (), 1), // +0x1702c sub_1702c
    Lifted::new(0x7001e2b068, fn_7001e2b068 as *const (), 0), // +0x17068 sub_17068
    Lifted::new(0x7001e2b09c, fn_7001e2b09c as *const (), 0), // +0x1709c sub_1709c
    Lifted::new(0x7001e2b184, fn_7001e2b184 as *const (), 2), // +0x17184 sub_17184
    Lifted::new(0x7001e2c9a0, fn_7001e2c9a0 as *const (), 2), // +0x189a0 sub_189a0
    Lifted::new(0x7001e2e1bc, fn_7001e2e1bc as *const (), 2), // +0x1a1bc sub_1a1bc
    Lifted::new(0x7001e2ecdc, fn_7001e2ecdc as *const (), 2), // +0x1acdc sub_1acdc
    Lifted::new(0x7001e2f7fc, fn_7001e2f7fc as *const (), 0), // +0x1b7fc sub_1b7fc
    Lifted::new(0x7001e2f98c, fn_7001e2f98c as *const (), 0), // +0x1b98c sub_1b98c
    Lifted::new(0x7001e2faf4, fn_7001e2faf4 as *const (), 0), // +0x1baf4 sub_1baf4
    Lifted::new(0x7001e2fc0c, fn_7001e2fc0c as *const (), 0), // +0x1bc0c sub_1bc0c
    Lifted::new(0x7001e2fd24, fn_7001e2fd24 as *const (), 0), // +0x1bd24 sub_1bd24
    Lifted::new(0x7001e2fe3c, fn_7001e2fe3c as *const (), 0), // +0x1be3c sub_1be3c
    Lifted::new(0x7001e2ff54, fn_7001e2ff54 as *const (), 0), // +0x1bf54 sub_1bf54
    Lifted::new(0x7001e3006c, fn_7001e3006c as *const (), 0), // +0x1c06c sub_1c06c
    Lifted::new(0x7001e30184, fn_7001e30184 as *const (), 0), // +0x1c184 sub_1c184
    Lifted::new(0x7001e3029c, fn_7001e3029c as *const (), 0), // +0x1c29c sub_1c29c
    Lifted::new(0x7001e303b4, fn_7001e303b4 as *const (), 0), // +0x1c3b4 sub_1c3b4
    Lifted::new(0x7001e304cc, fn_7001e304cc as *const (), 0), // +0x1c4cc sub_1c4cc
    Lifted::new(0x7001e305e4, fn_7001e305e4 as *const (), 0), // +0x1c5e4 sub_1c5e4
    Lifted::new(0x7001e306fc, fn_7001e306fc as *const (), 0), // +0x1c6fc sub_1c6fc
    Lifted::new(0x7001e30814, fn_7001e30814 as *const (), 0), // +0x1c814 sub_1c814
    Lifted::new(0x7001e3092c, fn_7001e3092c as *const (), 0), // +0x1c92c sub_1c92c
    Lifted::new(0x7001e30a44, fn_7001e30a44 as *const (), 0), // +0x1ca44 sub_1ca44
    Lifted::new(0x7001e30b5c, fn_7001e30b5c as *const (), 0), // +0x1cb5c sub_1cb5c
    Lifted::new(0x7001e30c74, fn_7001e30c74 as *const (), 0), // +0x1cc74 sub_1cc74
    Lifted::new(0x7001e30d8c, fn_7001e30d8c as *const (), 0), // +0x1cd8c sub_1cd8c
    Lifted::new(0x7001e30ea4, fn_7001e30ea4 as *const (), 0), // +0x1cea4 sub_1cea4
    Lifted::new(0x7001e30fbc, fn_7001e30fbc as *const (), 0), // +0x1cfbc sub_1cfbc
    Lifted::new(0x7001e310d4, fn_7001e310d4 as *const (), 0), // +0x1d0d4 sub_1d0d4
    Lifted::new(0x7001e311ec, fn_7001e311ec as *const (), 0), // +0x1d1ec sub_1d1ec
    Lifted::new(0x7001e31310, tpl_17::<0x7001e6d8e0, 0x8> as *const (), 1), // +0x1d310 ??0?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e31330, tpl_17::<0x7001e6d560, 0x8> as *const (), 1), // +0x1d330 ??0?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e31350, tpl_17::<0x7001e6d958, 0x8> as *const (), 1), // +0x1d350 ??0?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e31370, tpl_17::<0x7001e6d5d8, 0x8> as *const (), 1), // +0x1d370 ??0?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e31390, fn_7001e31390 as *const (), 0), // +0x1d390 ??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e313e0, fn_7001e313e0 as *const (), 0), // +0x1d3e0 ??0?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e31430, tpl_78::<0x7001e6dc60, 0x8, 0x10, 0x7001e69640, 0x7001e693a8> as *const (), 0), // +0x1d430 ??0?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e314a0, tpl_78::<0x7001e6d538, 0x8, 0x10, 0x7001e69640, 0x7001e693a8> as *const (), 0), // +0x1d4a0 ??0?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e31508, fn_7001e31508 as *const (), 0), // +0x1d508 sub_1d508
    Lifted::new(0x7001e31544, tpl_60::<0x10, 0x7001e693a8, 0x18, 0x20> as *const (), 0), // +0x1d544 sub_1d544
    Lifted::new(0x7001e31580, tpl_79::<0x7001e6dc08, 0x10, 0x7001e693a8, 0x18, 0x20, 0x7001e6b590> as *const (), 0), // +0x1d580 ??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEAA@XZ
    Lifted::new(0x7001e315d0, tpl_79::<0x7001e6d888, 0x10, 0x7001e693a8, 0x18, 0x20, 0x7001e6b590> as *const (), 0), // +0x1d5d0 ??1?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEAA@XZ
    Lifted::new(0x7001e31620, tpl_80::<0x7001e6dc60, 0x10, 0x7001e693a8, 0x7001e6b590> as *const (), 0), // +0x1d620 ??1?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEAA@XZ
    Lifted::new(0x7001e31650, tpl_80::<0x7001e6d538, 0x10, 0x7001e693a8, 0x7001e6b590> as *const (), 0), // +0x1d650 ??1?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEAA@XZ
    Lifted::new(0x7001e31680, fn_7001e31680 as *const (), 0), // +0x1d680 sub_1d680
    Lifted::new(0x7001e31710, tpl_81::<0x7001e6dad0, 0x10, 0x7001e693a8, 0x20, 0x28, 0x30, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x1d710 sub_1d710
    Lifted::new(0x7001e31780, tpl_81::<0x7001e6d750, 0x10, 0x7001e693a8, 0x20, 0x28, 0x30, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x1d780 sub_1d780
    Lifted::new(0x7001e317f0, tpl_82::<0x7001e6da00, 0x18, 0x7001e693a8, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x1d7f0 sub_1d7f0
    Lifted::new(0x7001e31840, tpl_82::<0x7001e6d680, 0x18, 0x7001e693a8, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x1d840 sub_1d840
    Lifted::new(0x7001e31890, fn_7001e31890 as *const (), 0), // +0x1d890 sub_1d890
    Lifted::new(0x7001e318c0, tpl_61::<0x7001e6d9b8, 0x10, 0x7001e693a8, 0x20, 0x28, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x1d8c0 sub_1d8c0
    Lifted::new(0x7001e31930, tpl_61::<0x7001e6d638, 0x10, 0x7001e693a8, 0x20, 0x28, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x1d930 sub_1d930
    Lifted::new(0x7001e31994, fn_7001e31994 as *const (), 2), // +0x1d994 sub_1d994
    Lifted::new(0x7001e31acc, fn_7001e31acc as *const (), 2), // +0x1dacc sub_1dacc
    Lifted::new(0x7001e31c04, fn_7001e31c04 as *const (), 0), // +0x1dc04 sub_1dc04
    Lifted::new(0x7001e31cc0, fn_7001e31cc0 as *const (), 0), // +0x1dcc0 sub_1dcc0
    Lifted::new(0x7001e31d7c, fn_7001e31d7c as *const (), 0), // +0x1dd7c sub_1dd7c
    Lifted::new(0x7001e31e38, fn_7001e31e38 as *const (), 0), // +0x1de38 sub_1de38
    Lifted::new(0x7001e31ef4, fn_7001e31ef4 as *const (), 0), // +0x1def4 sub_1def4
    Lifted::new(0x7001e31fb0, fn_7001e31fb0 as *const (), 0), // +0x1dfb0 sub_1dfb0
    Lifted::new(0x7001e3206c, fn_7001e3206c as *const (), 0), // +0x1e06c sub_1e06c
    Lifted::new(0x7001e32144, fn_7001e32144 as *const (), 0), // +0x1e144 sub_1e144
    Lifted::new(0x7001e3221c, fn_7001e3221c as *const (), 0), // +0x1e21c sub_1e21c
    Lifted::new(0x7001e322f4, fn_7001e322f4 as *const (), 0), // +0x1e2f4 sub_1e2f4
    Lifted::new(0x7001e323d0, fn_7001e323d0 as *const (), 0), // +0x1e3d0 ?_Getcat@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32490, fn_7001e32490 as *const (), 0), // +0x1e490 ?_Getcat@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32550, fn_7001e32550 as *const (), 0), // +0x1e550 ?_Getcat@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32610, fn_7001e32610 as *const (), 0), // +0x1e610 ?_Getcat@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e326cc, fn_7001e326cc as *const (), 0), // +0x1e6cc sub_1e6cc
    Lifted::new(0x7001e32794, fn_7001e32794 as *const (), 0), // +0x1e794 sub_1e794
    Lifted::new(0x7001e32860, fn_7001e32860 as *const (), 0), // +0x1e860 ?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32950, fn_7001e32950 as *const (), 0), // +0x1e950 ?_Getcat@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32a40, fn_7001e32a40 as *const (), 0), // +0x1ea40 ?_Getcat@?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32b30, fn_7001e32b30 as *const (), 0), // +0x1eb30 ?_Getcat@?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e32c20, fn_7001e32c20 as *const (), 2), // +0x1ec20 ?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@GU?$char_tra
    Lifted::new(0x7001e32f70, fn_7001e32f70 as *const (), 2), // +0x1ef70 ?_Getfmt@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@_WU?$char
    Lifted::new(0x7001e332c0, tpl_83::<0x7001e29d80> as *const (), 0), // +0x1f2c0 ?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@GU?$char_t
    Lifted::new(0x7001e3330c, fn_7001e3330c as *const (), 2), // +0x1f30c sub_1f30c
    Lifted::new(0x7001e347c8, fn_7001e347c8 as *const (), 2), // +0x207c8 sub_207c8
    Lifted::new(0x7001e35c84, fn_7001e35c84 as *const (), 0), // +0x21c84 sub_21c84
    Lifted::new(0x7001e35d0c, fn_7001e35d0c as *const (), 0), // +0x21d0c sub_21d0c
    Lifted::new(0x7001e35d74, fn_7001e35d74 as *const (), 2), // +0x21d74 sub_21d74
    Lifted::new(0x7001e35ebc, fn_7001e35ebc as *const (), 2), // +0x21ebc sub_21ebc
    Lifted::new(0x7001e36004, fn_7001e36004 as *const (), 2), // +0x22004 sub_22004
    Lifted::new(0x7001e36190, fn_7001e36190 as *const (), 2), // +0x22190 sub_22190
    Lifted::new(0x7001e36300, fn_7001e36300 as *const (), 0), // +0x22300 ?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e36340, fn_7001e36340 as *const (), 0), // +0x22340 ?_Init@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e36380, tpl_84::<0x7001e69640, 0x7001e693a8> as *const (), 0), // +0x22380 ?_Init@?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e363d0, fn_7001e363d0 as *const (), 2), // +0x223d0 ?_Iput@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@GU?$char_traits
    Lifted::new(0x7001e367d0, fn_7001e367d0 as *const (), 2), // +0x227d0 ?_Iput@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@_WU?$char_tr
    Lifted::new(0x7001e36bd0, fn_7001e36bd0 as *const (), 2), // +0x22bd0 ?_Makeushloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z
    Lifted::new(0x7001e37880, fn_7001e37880 as *const (), 2), // +0x23880 ?_Makewloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z
    Lifted::new(0x7001e3852c, fn_7001e3852c as *const (), 0), // +0x2452c sub_2452c
    Lifted::new(0x7001e38598, fn_7001e38598 as *const (), 2), // +0x24598 sub_24598
    Lifted::new(0x7001e38650, fn_7001e38650 as *const (), 2), // +0x24650 ?_Put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@GU?$char_traits@
    Lifted::new(0x7001e38708, fn_7001e38708 as *const (), 2), // +0x24708 sub_24708
    Lifted::new(0x7001e38fa8, fn_7001e38fa8 as *const (), 2), // +0x24fa8 sub_24fa8
    Lifted::new(0x7001e39848, fn_7001e39848 as *const (), 2), // +0x25848 sub_25848
    Lifted::new(0x7001e39900, fn_7001e39900 as *const (), 2), // +0x25900 ?_Rep@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@GU?$char_traits@
    Lifted::new(0x7001e399b0, fn_7001e399b0 as *const (), 0), // +0x259b0 ?_Tidy@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEAAXXZ
    Lifted::new(0x7001e399e0, fn_7001e399e0 as *const (), 0), // +0x259e0 sub_259e0
    Lifted::new(0x7001e39a60, tpl_85::<0x10, 0x7001e2b02c> as *const (), 0), // +0x25a60 sub_25a60
    Lifted::new(0x7001e39ab0, tpl_85::<0x10, 0x7001e4d6f0> as *const (), 0), // +0x25ab0 sub_25ab0
    Lifted::new(0x7001e39b00, tpl_86::<0x20, 0x10, 0x18, 0x2, 0x7001e1da0c> as *const (), 2), // +0x25b00 sub_25b00
    Lifted::new(0x7001e39b50, tpl_87::<0x20, 0x10, 0x18, 0x7001e1da0c> as *const (), 2), // +0x25b50 sub_25b50
    Lifted::new(0x7001e39b90, fn_7001e39b90 as *const (), 1), // +0x25b90 ?do_date_order@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AW4dateorder@time_base@2@XZ
    Lifted::new(0x7001e39ba0, tpl_88::<0x18> as *const (), 1), // +0x25ba0 sub_25ba0
    Lifted::new(0x7001e39bb0, tpl_42::<0x38> as *const (), 1), // +0x25bb0 sub_25bb0
    Lifted::new(0x7001e39bc0, fn_7001e39bc0 as *const (), 2), // +0x25bc0 sub_25bc0
    Lifted::new(0x7001e39c00, fn_7001e39c00 as *const (), 2), // +0x25c00 sub_25c00
    Lifted::new(0x7001e39d40, fn_7001e39d40 as *const (), 2), // +0x25d40 sub_25d40
    Lifted::new(0x7001e39ec0, fn_7001e39ec0 as *const (), 2), // +0x25ec0 sub_25ec0
    Lifted::new(0x7001e3a000, fn_7001e3a000 as *const (), 2), // +0x26000 sub_26000
    Lifted::new(0x7001e3a180, fn_7001e3a180 as *const (), 2), // +0x26180 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a2f0, tpl_64::<0x7001e3a490> as *const (), 2), // +0x262f0 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a370, fn_7001e3a370 as *const (), 2), // +0x26370 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a490, fn_7001e3a490 as *const (), 2), // +0x26490 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a5b0, fn_7001e3a5b0 as *const (), 2), // +0x265b0 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a700, fn_7001e3a700 as *const (), 2), // +0x26700 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a850, tpl_65::<0x7001e3a700> as *const (), 2), // +0x26850 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3a8e0, fn_7001e3a8e0 as *const (), 2), // +0x268e0 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3aa30, fn_7001e3aa30 as *const (), 2), // +0x26a30 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3ab50, fn_7001e3ab50 as *const (), 2), // +0x26b50 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3ac70, fn_7001e3ac70 as *const (), 2), // +0x26c70 ?do_get@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3af80, fn_7001e3af80 as *const (), 2), // +0x26f80 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b0f0, tpl_64::<0x7001e3b290> as *const (), 2), // +0x270f0 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b170, fn_7001e3b170 as *const (), 2), // +0x27170 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b290, fn_7001e3b290 as *const (), 2), // +0x27290 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b3b0, fn_7001e3b3b0 as *const (), 2), // +0x273b0 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b500, fn_7001e3b500 as *const (), 2), // +0x27500 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b650, tpl_65::<0x7001e3b500> as *const (), 2), // +0x27650 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b6e0, fn_7001e3b6e0 as *const (), 2), // +0x276e0 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b830, fn_7001e3b830 as *const (), 2), // +0x27830 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3b950, fn_7001e3b950 as *const (), 2), // +0x27950 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3ba70, fn_7001e3ba70 as *const (), 2), // +0x27a70 ?do_get@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e3bd80, fn_7001e3bd80 as *const (), 2), // +0x27d80 ?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_trai
    Lifted::new(0x7001e3c300, fn_7001e3c300 as *const (), 2), // +0x28300 ?do_get@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$char_
    Lifted::new(0x7001e3c880, fn_7001e3c880 as *const (), 2), // +0x28880 ?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char
    Lifted::new(0x7001e3d300, fn_7001e3d300 as *const (), 2), // +0x29300 ?do_get_date@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$
    Lifted::new(0x7001e3dd80, tpl_89::<0x18, 0x7001e2a5dc, 0x10> as *const (), 2), // +0x29d80 ?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?
    Lifted::new(0x7001e3dde0, tpl_89::<0x18, 0x7001e2a934, 0x10> as *const (), 2), // +0x29de0 ?do_get_monthname@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@
    Lifted::new(0x7001e3de40, fn_7001e3de40 as *const (), 2), // +0x29e40 ?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char
    Lifted::new(0x7001e3e110, fn_7001e3e110 as *const (), 2), // +0x2a110 ?do_get_time@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$
    Lifted::new(0x7001e3e3e0, tpl_89::<0x10, 0x7001e2a5dc, 0x18> as *const (), 2), // +0x2a3e0 ?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$c
    Lifted::new(0x7001e3e440, tpl_89::<0x10, 0x7001e2a934, 0x18> as *const (), 2), // +0x2a440 ?do_get_weekday@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_W
    Lifted::new(0x7001e3e4a0, fn_7001e3e4a0 as *const (), 2), // +0x2a4a0 ?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char
    Lifted::new(0x7001e3e5c0, fn_7001e3e5c0 as *const (), 2), // +0x2a5c0 ?do_get_year@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@_WU?$
    Lifted::new(0x7001e3e6e0, fn_7001e3e6e0 as *const (), 0), // +0x2a6e0 sub_2a6e0
    Lifted::new(0x7001e3e700, tpl_90::<0x40> as *const (), 1), // +0x2a700 sub_2a700
    Lifted::new(0x7001e3e710, tpl_86::<0x30, 0x10, 0x18, 0x2, 0x7001e1da0c> as *const (), 2), // +0x2a710 sub_2a710
    Lifted::new(0x7001e3e760, tpl_87::<0x30, 0x10, 0x18, 0x7001e1da0c> as *const (), 2), // +0x2a760 sub_2a760
    Lifted::new(0x7001e3e7a0, tpl_90::<0x3c> as *const (), 1), // +0x2a7a0 sub_2a7a0
    Lifted::new(0x7001e3e7b0, tpl_86::<0x28, 0x10, 0x18, 0x2, 0x7001e1da0c> as *const (), 2), // +0x2a7b0 sub_2a7b0
    Lifted::new(0x7001e3e800, tpl_87::<0x28, 0x10, 0x18, 0x7001e1da0c> as *const (), 2), // +0x2a800 sub_2a800
    Lifted::new(0x7001e3e840, fn_7001e3e840 as *const (), 2), // +0x2a840 sub_2a840
    Lifted::new(0x7001e3eb90, fn_7001e3eb90 as *const (), 2), // +0x2ab90 sub_2ab90
    Lifted::new(0x7001e3eec0, fn_7001e3eec0 as *const (), 2), // +0x2aec0 sub_2aec0
    Lifted::new(0x7001e3f210, fn_7001e3f210 as *const (), 2), // +0x2b210 sub_2b210
    Lifted::new(0x7001e3f540, tpl_91::<0x7001e97380, 0x7001e255f4, 0x7001e363d0> as *const (), 2), // +0x2b540 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3f650, tpl_92::<0x7001e97380, 0x7001e255f4, 0x7001e363d0> as *const (), 2), // +0x2b650 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3f760, fn_7001e3f760 as *const (), 2), // +0x2b760 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3f970, fn_7001e3f970 as *const (), 2), // +0x2b970 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3fb80, tpl_93::<0x7001e97380, 0x7001e6c210, 0x7001e255f4, 0x7001e363d0> as *const (), 2), // +0x2bb80 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3fc30, tpl_94::<0x7001e97380, 0x7001e255f4, 0x7001e363d0> as *const (), 2), // +0x2bc30 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3fd40, tpl_95::<0x7001e97380, 0x7001e255f4, 0x7001e363d0> as *const (), 2), // +0x2bd40 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e3fe50, fn_7001e3fe50 as *const (), 2), // +0x2be50 ?do_put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trait
    Lifted::new(0x7001e400d0, tpl_91::<0x7001e97380, 0x7001e255f4, 0x7001e367d0> as *const (), 2), // +0x2c0d0 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e401e0, tpl_92::<0x7001e97380, 0x7001e255f4, 0x7001e367d0> as *const (), 2), // +0x2c1e0 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e402f0, fn_7001e402f0 as *const (), 2), // +0x2c2f0 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e40500, fn_7001e40500 as *const (), 2), // +0x2c500 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e40710, tpl_93::<0x7001e97380, 0x7001e6c210, 0x7001e255f4, 0x7001e367d0> as *const (), 2), // +0x2c710 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e407c0, tpl_94::<0x7001e97380, 0x7001e255f4, 0x7001e367d0> as *const (), 2), // +0x2c7c0 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e408d0, tpl_95::<0x7001e97380, 0x7001e255f4, 0x7001e367d0> as *const (), 2), // +0x2c8d0 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e409e0, fn_7001e409e0 as *const (), 2), // +0x2c9e0 ?do_put@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@_WU?$char_t
    Lifted::new(0x7001e40c60, fn_7001e40c60 as *const (), 2), // +0x2cc60 ?do_put@?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@GU?$char_trai
    Lifted::new(0x7001e40e00, tpl_88::<0x1a> as *const (), 1), // +0x2ce00 sub_2ce00
    Lifted::new(0x7001e40e10, fn_7001e40e10 as *const (), 2), // +0x2ce10 sub_2ce10
    Lifted::new(0x7001e40f20, fn_7001e40f20 as *const (), 2), // +0x2cf20 sub_2cf20
    Lifted::new(0x7001e4102c, fn_7001e4102c as *const (), 0), // +0x2d02c sub_2d02c
    Lifted::new(0x7001e41120, tpl_96::<0x38, 0x7001e696a0> as *const (), 2), // +0x2d120 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e41190, tpl_96::<0x30, 0x7001e696a0> as *const (), 2), // +0x2d190 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e41200, tpl_96::<0x40, 0x7001e696a0> as *const (), 2), // +0x2d200 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e41264, fn_7001e41264 as *const (), 0), // +0x2d264 sub_2d264
    Lifted::new(0x7001e41318, fn_7001e41318 as *const (), 2), // +0x2d318 sub_2d318
    Lifted::new(0x7001e41480, fn_7001e41480 as *const (), 0), // +0x2d480 ?sbumpc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e414c0, fn_7001e414c0 as *const (), 0), // +0x2d4c0 ?sgetc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e414f0, fn_7001e414f0 as *const (), 0), // +0x2d4f0 ?sputc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAGG@Z
    Lifted::new(0x7001e41540, fn_7001e41540 as *const (), 0), // +0x2d540 ?_Mtx_delete@threads@stdext@@YAXPEAX@Z
    Lifted::new(0x7001e41570, tpl_15::<0x7001e69000> as *const (), 2), // +0x2d570 ?_Mtx_lock@threads@stdext@@YAXPEAX@Z
    Lifted::new(0x7001e41580, fn_7001e41580 as *const (), 0), // +0x2d580 ?_Mtx_new@threads@stdext@@YAXAEAPEAX@Z
    Lifted::new(0x7001e415b0, tpl_15::<0x7001e69180> as *const (), 2), // +0x2d5b0 ?_Mtx_unlock@threads@stdext@@YAXPEAX@Z
    Lifted::new(0x7001e415c0, tpl_97::<0x7001e41dd0, 0x7001e6dea0, 0x7001e6ecf8, 0x7001e6deb8, 0x7001e41f20, 0x7001e6deb0, 0x7001e42770> as *const (), 2), // +0x2d5c0 _Cosh
    Lifted::new(0x7001e416a0, fn_7001e416a0 as *const (), 0), // +0x2d6a0 _Getdateorder
    Lifted::new(0x7001e416f8, fn_7001e416f8 as *const (), 1), // +0x2d6f8 sub_2d6f8
    Lifted::new(0x7001e41810, fn_7001e41810 as *const (), 1), // +0x2d810 sub_2d810
    Lifted::new(0x7001e41994, fn_7001e41994 as *const (), 2), // +0x2d994 sub_2d994
    Lifted::new(0x7001e41c4c, tpl_98::<0x7001e97380, 0x7001e6df20, 0x7001e6df28, 0x7001e4b658, 0x7001e6df00, 0x7001e4a4d8, 0x7001e4a468, 0x7001e4b52c, 0x7001e6deb8, 0x7001e4a3f4, 0x7001e97318, 0x7001e6dea0, 0x7001e694e0> as *const (), 2), // +0x2dc4c sub_2dc4c
    Lifted::new(0x7001e41dd0, fn_7001e41dd0 as *const (), 1), // +0x2ddd0 _Dtest
    Lifted::new(0x7001e41e34, tpl_67::<0x6> as *const (), 1), // +0x2de34 sub_2de34
    Lifted::new(0x7001e41e3c, fn_7001e41e3c as *const (), 0), // +0x2de3c sub_2de3c
    Lifted::new(0x7001e41ed8, tpl_99::<0x7001e6dfb8, 0x7001e69428> as *const (), 2), // +0x2ded8 sub_2ded8
    Lifted::new(0x7001e41efc, tpl_99::<0x7001e6df60, 0x7001e69428> as *const (), 2), // +0x2defc sub_2defc
    Lifted::new(0x7001e41f20, fn_7001e41f20 as *const (), 2), // +0x2df20 _Exp
    Lifted::new(0x7001e420b0, fn_7001e420b0 as *const (), 2), // +0x2e0b0 _FCosh
    Lifted::new(0x7001e42188, fn_7001e42188 as *const (), 1), // +0x2e188 sub_2e188
    Lifted::new(0x7001e42260, fn_7001e42260 as *const (), 1), // +0x2e260 sub_2e260
    Lifted::new(0x7001e4233c, fn_7001e4233c as *const (), 2), // +0x2e33c sub_2e33c
    Lifted::new(0x7001e42518, fn_7001e42518 as *const (), 2), // +0x2e518 sub_2e518
    Lifted::new(0x7001e42690, fn_7001e42690 as *const (), 1), // +0x2e690 _FDtest
    Lifted::new(0x7001e426dc, tpl_67::<0x2> as *const (), 1), // +0x2e6dc sub_2e6dc
    Lifted::new(0x7001e426e4, fn_7001e426e4 as *const (), 0), // +0x2e6e4 sub_2e6e4
    Lifted::new(0x7001e42770, fn_7001e42770 as *const (), 0), // +0x2e770 sub_2e770
    Lifted::new(0x7001e427a0, fn_7001e427a0 as *const (), 2), // +0x2e7a0 _FExp
    Lifted::new(0x7001e42938, fn_7001e42938 as *const (), 2), // +0x2e938 sub_2e938
    Lifted::new(0x7001e42c88, fn_7001e42c88 as *const (), 2), // +0x2ec88 sub_2ec88
    Lifted::new(0x7001e42cf8, fn_7001e42cf8 as *const (), 3), // +0x2ecf8 sub_2ecf8
    Lifted::new(0x7001e42d68, fn_7001e42d68 as *const (), 2), // +0x2ed68 sub_2ed68
    Lifted::new(0x7001e42de0, fn_7001e42de0 as *const (), 0), // +0x2ede0 sub_2ede0
    Lifted::new(0x7001e42e04, fn_7001e42e04 as *const (), 2), // +0x2ee04 sub_2ee04
    Lifted::new(0x7001e42ff8, fn_7001e42ff8 as *const (), 2), // +0x2eff8 sub_2eff8
    Lifted::new(0x7001e43100, fn_7001e43100 as *const (), 2), // +0x2f100 sub_2f100
    Lifted::new(0x7001e43184, fn_7001e43184 as *const (), 2), // +0x2f184 sub_2f184
    Lifted::new(0x7001e43230, fn_7001e43230 as *const (), 2), // +0x2f230 _FSinh
    Lifted::new(0x7001e433c0, fn_7001e433c0 as *const (), 0), // +0x2f3c0 _Getwctype
    Lifted::new(0x7001e433f0, fn_7001e433f0 as *const (), 0), // +0x2f3f0 _Getwctypes
    Lifted::new(0x7001e43420, tpl_97::<0x7001e43690, 0x7001e6dea0, 0x7001e6e518, 0x7001e6deb8, 0x7001e436a0, 0x7001e6deb0, 0x7001e42770> as *const (), 2), // +0x2f420 _LCosh
    Lifted::new(0x7001e4350c, tpl_98::<0x7001e97380, 0x7001e6df20, 0x7001e6df28, 0x7001e4a828, 0x7001e6e0c8, 0x7001e4a4d8, 0x7001e4a468, 0x7001e4a6fc, 0x7001e6deb8, 0x7001e4a3f4, 0x7001e972b8, 0x7001e6dea0, 0x7001e694e0> as *const (), 2), // +0x2f50c sub_2f50c
    Lifted::alias(0x7001e43690, fn_7001e41dd0 as *const (), 1, 0x2ddd0), // +0x2f690 _LDtest (thunk -> +0x2ddd0)
    Lifted::new(0x7001e436a0, fn_7001e436a0 as *const (), 2), // +0x2f6a0 _LExp
    Lifted::new(0x7001e43880, fn_7001e43880 as *const (), 2), // +0x2f880 ?_XLgamma@std@@YAMM@Z
    Lifted::new(0x7001e43920, fn_7001e43920 as *const (), 2), // +0x2f920 ?_XLgamma@std@@YANN@Z
    Lifted::new(0x7001e439c0, fn_7001e439c0 as *const (), 2), // +0x2f9c0 ?_XLgamma@std@@YAOO@Z
    Lifted::new(0x7001e43a5c, fn_7001e43a5c as *const (), 0), // +0x2fa5c sub_2fa5c
    Lifted::new(0x7001e43ed4, fn_7001e43ed4 as *const (), 0), // +0x2fed4 sub_2fed4
    Lifted::new(0x7001e43f6c, tpl_77::<0x7001e4d900, 0x7001e69660, 0x7001e18ef0, 0x7001e693a8, 0x7001e6b5c0, 0x7001e1fcdc, 0x7001e69670, 0x7001e6b620, 0x7001e6dcc0> as *const (), 2), // +0x2ff6c sub_2ff6c
    Lifted::new(0x7001e44070, fn_7001e44070 as *const (), 1), // +0x30070 sub_30070
    Lifted::new(0x7001e440a4, fn_7001e440a4 as *const (), 0), // +0x300a4 sub_300a4
    Lifted::new(0x7001e441bc, fn_7001e441bc as *const (), 0), // +0x301bc sub_301bc
    Lifted::new(0x7001e442d4, fn_7001e442d4 as *const (), 0), // +0x302d4 sub_302d4
    Lifted::new(0x7001e443ec, fn_7001e443ec as *const (), 0), // +0x303ec sub_303ec
    Lifted::new(0x7001e44504, fn_7001e44504 as *const (), 0), // +0x30504 sub_30504
    Lifted::new(0x7001e4461c, fn_7001e4461c as *const (), 0), // +0x3061c sub_3061c
    Lifted::new(0x7001e44734, fn_7001e44734 as *const (), 0), // +0x30734 sub_30734
    Lifted::new(0x7001e4484c, fn_7001e4484c as *const (), 0), // +0x3084c sub_3084c
    Lifted::new(0x7001e44970, fn_7001e44970 as *const (), 0), // +0x30970 ??0?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e449c0, tpl_78::<0x7001e6e468, 0x8, 0x10, 0x7001e69668, 0x7001e693a8> as *const (), 0), // +0x309c0 ??0?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z
    Lifted::new(0x7001e44a30, tpl_79::<0x7001e6e410, 0x10, 0x7001e693a8, 0x18, 0x20, 0x7001e6b590> as *const (), 0), // +0x30a30 ??1?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAA@XZ
    Lifted::new(0x7001e44a80, tpl_80::<0x7001e6e468, 0x10, 0x7001e693a8, 0x7001e6b590> as *const (), 0), // +0x30a80 ??1?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAA@XZ
    Lifted::new(0x7001e44ab0, tpl_81::<0x7001e6e2d8, 0x10, 0x7001e693a8, 0x20, 0x28, 0x30, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x30ab0 sub_30ab0
    Lifted::new(0x7001e44b20, tpl_82::<0x7001e6e208, 0x18, 0x7001e693a8, 0x7001e6b590, 0x7001e6649c> as *const (), 0), // +0x30b20 sub_30b20
    Lifted::new(0x7001e44b70, fn_7001e44b70 as *const (), 2), // +0x30b70 sub_30b70
    Lifted::new(0x7001e44ca8, fn_7001e44ca8 as *const (), 0), // +0x30ca8 sub_30ca8
    Lifted::new(0x7001e44d64, fn_7001e44d64 as *const (), 0), // +0x30d64 sub_30d64
    Lifted::new(0x7001e44e20, fn_7001e44e20 as *const (), 0), // +0x30e20 sub_30e20
    Lifted::new(0x7001e44edc, fn_7001e44edc as *const (), 0), // +0x30edc sub_30edc
    Lifted::new(0x7001e44fb4, fn_7001e44fb4 as *const (), 0), // +0x30fb4 sub_30fb4
    Lifted::new(0x7001e45090, fn_7001e45090 as *const (), 0), // +0x31090 ?_Getcat@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e45180, fn_7001e45180 as *const (), 0), // +0x31180 ?_Getcat@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z
    Lifted::new(0x7001e45270, fn_7001e45270 as *const (), 2), // +0x31270 ?_Getfmt@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@DU?$char_tra
    Lifted::new(0x7001e45590, tpl_83::<0x7001e43a5c> as *const (), 0), // +0x31590 ?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@DU?$char_t
    Lifted::new(0x7001e455dc, fn_7001e455dc as *const (), 2), // +0x315dc sub_315dc
    Lifted::new(0x7001e46ab4, fn_7001e46ab4 as *const (), 2), // +0x32ab4 sub_32ab4
    Lifted::new(0x7001e46c00, fn_7001e46c00 as *const (), 0), // +0x32c00 ?_Init@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e46c40, tpl_84::<0x7001e69668, 0x7001e693a8> as *const (), 0), // +0x32c40 ?_Init@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z
    Lifted::new(0x7001e46c90, fn_7001e46c90 as *const (), 2), // +0x32c90 ?_Makexloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z
    Lifted::new(0x7001e47460, fn_7001e47460 as *const (), 2), // +0x33460 sub_33460
    Lifted::new(0x7001e47d90, tpl_85::<0x10, 0x7001e4ce40> as *const (), 0), // +0x33d90 sub_33d90
    Lifted::new(0x7001e47de0, fn_7001e47de0 as *const (), 2), // +0x33de0 sub_33de0
    Lifted::new(0x7001e47e20, fn_7001e47e20 as *const (), 2), // +0x33e20 sub_33e20
    Lifted::new(0x7001e47f60, fn_7001e47f60 as *const (), 2), // +0x33f60 sub_33f60
    Lifted::new(0x7001e480d0, fn_7001e480d0 as *const (), 2), // +0x340d0 ?do_get@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char_trai
    Lifted::new(0x7001e48650, fn_7001e48650 as *const (), 2), // +0x34650 ?do_get_date@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char
    Lifted::new(0x7001e49010, tpl_89::<0x18, 0x7001e1f9b8, 0x10> as *const (), 2), // +0x35010 ?do_get_monthname@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?
    Lifted::new(0x7001e49070, fn_7001e49070 as *const (), 2), // +0x35070 ?do_get_time@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char
    Lifted::new(0x7001e49330, tpl_89::<0x10, 0x7001e1f9b8, 0x18> as *const (), 2), // +0x35330 ?do_get_weekday@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$c
    Lifted::new(0x7001e49390, fn_7001e49390 as *const (), 2), // +0x35390 ?do_get_year@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@DU?$char
    Lifted::new(0x7001e494b0, fn_7001e494b0 as *const (), 0), // +0x354b0 sub_354b0
    Lifted::new(0x7001e494c0, tpl_63::<0x30, 0x10, 0x18, 0x7001e174fc> as *const (), 2), // +0x354c0 sub_354c0
    Lifted::new(0x7001e49500, fn_7001e49500 as *const (), 2), // +0x35500 sub_35500
    Lifted::new(0x7001e49820, fn_7001e49820 as *const (), 2), // +0x35820 sub_35820
    Lifted::new(0x7001e49b10, fn_7001e49b10 as *const (), 2), // +0x35b10 ?do_put@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_trai
    Lifted::new(0x7001e49cc0, fn_7001e49cc0 as *const (), 2), // +0x35cc0 sub_35cc0
    Lifted::new(0x7001e49dd0, fn_7001e49dd0 as *const (), 0), // +0x35dd0 ??0_Init_locks@std@@QEAA@XZ
    Lifted::new(0x7001e49e20, fn_7001e49e20 as *const (), 0), // +0x35e20 ??0_Lockit@std@@QEAA@H@Z
    Lifted::new(0x7001e49e60, fn_7001e49e60 as *const (), 0), // +0x35e60 ??0_Lockit@std@@QEAA@XZ
    Lifted::new(0x7001e49e80, fn_7001e49e80 as *const (), 0), // +0x35e80 ??1_Init_locks@std@@QEAA@XZ
    Lifted::new(0x7001e49ec0, fn_7001e49ec0 as *const (), 0), // +0x35ec0 ??1_Lockit@std@@QEAA@XZ
    Lifted::new(0x7001e49f00, fn_7001e49f00 as *const (), 0), // +0x35f00 ?_Init_locks_ctor@_Init_locks@std@@CAXPEAV12@@Z
    Lifted::new(0x7001e49f40, tpl_7::<0x7001e9a0b0, 0x7001e41570> as *const (), 0), // +0x35f40 ?_Lockit_ctor@_Lockit@std@@CAXPEAV12@@Z
    Lifted::new(0x7001e49f50, fn_7001e49f50 as *const (), 0), // +0x35f50 ?_Lockit_ctor@_Lockit@std@@CAXPEAV12@H@Z
    Lifted::new(0x7001e49f80, fn_7001e49f80 as *const (), 0), // +0x35f80 ?_Lockit_ctor@_Lockit@std@@SAXH@Z
    Lifted::new(0x7001e49fb0, fn_7001e49fb0 as *const (), 0), // +0x35fb0 ?_Lockit_dtor@_Lockit@std@@CAXPEAV12@@Z
    Lifted::new(0x7001e49fd0, fn_7001e49fd0 as *const (), 2), // +0x35fd0 ?_Lockit_dtor@_Lockit@std@@SAXH@Z
    Lifted::new(0x7001e49ff4, fn_7001e49ff4 as *const (), 0), // +0x35ff4 sub_35ff4
    Lifted::new(0x7001e4a00c, fn_7001e4a00c as *const (), 3), // +0x3600c sub_3600c
    Lifted::new(0x7001e4a028, fn_7001e4a028 as *const (), 2), // +0x36028 sub_36028
    Lifted::new(0x7001e4a384, tpl_100::<0x8, 0x10, 0x18, 0x20, 0x7001e6df20, 0x7001e4a028> as *const (), 2), // +0x36384 sub_36384
    Lifted::new(0x7001e4a3f4, fn_7001e4a3f4 as *const (), 3), // +0x363f4 sub_363f4
    Lifted::new(0x7001e4a468, fn_7001e4a468 as *const (), 2), // +0x36468 sub_36468
    Lifted::new(0x7001e4a4d8, fn_7001e4a4d8 as *const (), 0), // +0x364d8 sub_364d8
    Lifted::new(0x7001e4a4fc, tpl_101::<0x7001e97380, 0x7001e43690, 0x7001e42770, 0x7001e41e3c, 0x7001e416f8, 0x7001e41994, 0x7001e4a028> as *const (), 2), // +0x364fc sub_364fc
    Lifted::new(0x7001e4a6fc, tpl_102::<0x7001e6df20, 0x7001e4a4fc, 0x7001e66450, 0x7001e4a384> as *const (), 2), // +0x366fc sub_366fc
    Lifted::new(0x7001e4a808, tpl_103::<0x7001e4a828> as *const (), 2), // +0x36808 sub_36808
    Lifted::new(0x7001e4a828, tpl_104::<0x7001e41e3c, 0x7001e416f8, 0x7001e41994, 0x7001e6df20> as *const (), 2), // +0x36828 sub_36828
    Lifted::new(0x7001e4a940, fn_7001e4a940 as *const (), 2), // +0x36940 _LSinh
    Lifted::new(0x7001e4aae0, fn_7001e4aae0 as *const (), 0), // +0x36ae0 _Mbrtowc
    Lifted::new(0x7001e4ac80, tpl_15::<0x7001e69170> as *const (), 2), // +0x36c80 _Mtxdst
    Lifted::new(0x7001e4ac90, fn_7001e4ac90 as *const (), 0), // +0x36c90 _Mtxinit
    Lifted::new(0x7001e4aca0, fn_7001e4aca0 as *const (), 0), // +0x36ca0 sub_36ca0
    Lifted::new(0x7001e4acb0, fn_7001e4acb0 as *const (), 2), // +0x36cb0 sub_36cb0
    Lifted::new(0x7001e4ada0, fn_7001e4ada0 as *const (), 2), // +0x36da0 sub_36da0
    Lifted::new(0x7001e4ade0, fn_7001e4ade0 as *const (), 0), // +0x36de0 ?_Execute_once@std@@YAHAEAUonce_flag@1@P6AHPEAX1PEAPEAX@Z1@Z
    Lifted::new(0x7001e4ae10, fn_7001e4ae10 as *const (), 2), // +0x36e10 ?_XGetLastError@std@@YAXXZ
    Lifted::new(0x7001e4ae60, fn_7001e4ae60 as *const (), 0), // +0x36e60 sub_36e60
    Lifted::new(0x7001e4aec0, fn_7001e4aec0 as *const (), 2), // +0x36ec0 sub_36ec0
    Lifted::new(0x7001e4af50, tpl_34::<0x7001e6e558> as *const (), 1), // +0x36f50 sub_36f50
    Lifted::new(0x7001e4af58, tpl_34::<0x7001e972e8> as *const (), 1), // +0x36f58 sub_36f58
    Lifted::new(0x7001e4af60, fn_7001e4af60 as *const (), 2), // +0x36f60 sub_36f60
    Lifted::new(0x7001e4b2bc, tpl_100::<0x8, 0x10, 0x18, 0x20, 0x7001e6df20, 0x7001e4af60> as *const (), 2), // +0x372bc sub_372bc
    Lifted::new(0x7001e4b32c, tpl_101::<0x7001e97380, 0x7001e41dd0, 0x7001e42770, 0x7001e41e3c, 0x7001e416f8, 0x7001e41994, 0x7001e4af60> as *const (), 2), // +0x3732c sub_3732c
    Lifted::new(0x7001e4b52c, tpl_102::<0x7001e6df20, 0x7001e4b32c, 0x7001e66450, 0x7001e4b2bc> as *const (), 2), // +0x3752c sub_3752c
    Lifted::new(0x7001e4b638, tpl_103::<0x7001e4b658> as *const (), 2), // +0x37638 sub_37638
    Lifted::new(0x7001e4b658, tpl_104::<0x7001e41e3c, 0x7001e416f8, 0x7001e41994, 0x7001e6df20> as *const (), 2), // +0x37658 sub_37658
    Lifted::new(0x7001e4b770, fn_7001e4b770 as *const (), 0), // +0x37770 ?_Random_device@std@@YAIXZ
    Lifted::new(0x7001e4b7a0, fn_7001e4b7a0 as *const (), 2), // +0x377a0 _Sinh
    Lifted::new(0x7001e4b940, tpl_105::<0x7001e4b950> as *const (), 0), // +0x37940 _Stod
    Lifted::new(0x7001e4b950, tpl_106::<0x7001e97380, 0x7001e4c550, 0x7001e4be18, 0x7001e6e5d0, 0x7001e4b658, 0x7001e4b638, 0x7001e4b52c, 0x7001e4b2bc, 0x7001e41c4c, 0x7001e4caec, 0x7001e6e5c8, 0x7001e41994, 0x7001e97318, 0x7001e97328, 0x7001e41e34> as *const (), 2), // +0x37950 _Stodx
    Lifted::new(0x7001e4bbc0, tpl_105::<0x7001e4bbd0> as *const (), 0), // +0x37bc0 _Stof
    Lifted::new(0x7001e4bbd0, tpl_107::<0x7001e97380, 0x7001e4c550, 0x7001e4be18, 0x7001e6e5dc, 0x7001e43184, 0x7001e43100, 0x7001e42ff8, 0x7001e42c88, 0x7001e42518, 0x7001e4caec, 0x7001e6e5d8, 0x7001e4233c, 0x7001e97270, 0x7001e97280, 0x7001e426dc> as *const (), 2), // +0x37bd0 _Stofx
    Lifted::new(0x7001e4be18, fn_7001e4be18 as *const (), 0), // +0x37e18 sub_37e18
    Lifted::new(0x7001e4c0e0, fn_7001e4c0e0 as *const (), 0), // +0x380e0 _Stolx
    Lifted::new(0x7001e4c1c0, tpl_105::<0x7001e4c1d0> as *const (), 0), // +0x381c0 _Stold
    Lifted::new(0x7001e4c1d0, tpl_106::<0x7001e97380, 0x7001e4c550, 0x7001e4be18, 0x7001e6e5d0, 0x7001e4a828, 0x7001e4a808, 0x7001e4a6fc, 0x7001e4a384, 0x7001e4350c, 0x7001e4caec, 0x7001e6e5c8, 0x7001e41994, 0x7001e972b8, 0x7001e972c8, 0x7001e41e34> as *const (), 2), // +0x381d0 _Stoldx
    Lifted::new(0x7001e4c440, tpl_105::<0x7001e4c450> as *const (), 0), // +0x38440 _Stoll
    Lifted::new(0x7001e4c450, fn_7001e4c450 as *const (), 0), // +0x38450 _Stollx
    Lifted::new(0x7001e4c550, fn_7001e4c550 as *const (), 0), // +0x38550 sub_38550
    Lifted::new(0x7001e4c6d0, tpl_105::<0x7001e4c6e0> as *const (), 0), // +0x386d0 _Stoul
    Lifted::new(0x7001e4c6e0, fn_7001e4c6e0 as *const (), 0), // +0x386e0 _Stoulx
    Lifted::new(0x7001e4c8e0, tpl_105::<0x7001e4c8f0> as *const (), 0), // +0x388e0 _Stoull
    Lifted::new(0x7001e4c8f0, fn_7001e4c8f0 as *const (), 0), // +0x388f0 _Stoullx
    Lifted::new(0x7001e4caec, fn_7001e4caec as *const (), 0), // +0x38aec sub_38aec
    Lifted::new(0x7001e4ce00, fn_7001e4ce00 as *const (), 0), // +0x38e00 _Getcoll
    Lifted::new(0x7001e4ce40, fn_7001e4ce40 as *const (), 0), // +0x38e40 _Strcoll
    Lifted::new(0x7001e4cf10, fn_7001e4cf10 as *const (), 0), // +0x38f10 _Strxfrm
    Lifted::new(0x7001e4d010, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e6b8> as *const (), 2), // +0x39010 sub_39010
    Lifted::new(0x7001e4d04c, fn_7001e4d04c as *const (), 3), // +0x3904c sub_3904c
    Lifted::new(0x7001e4d064, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e6e8> as *const (), 2), // +0x39064 sub_39064
    Lifted::new(0x7001e4d0a0, tpl_108::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e6e8> as *const (), 2), // +0x390a0 sub_390a0
    Lifted::new(0x7001e4d0e8, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e700> as *const (), 2), // +0x390e8 sub_390e8
    Lifted::new(0x7001e4d124, tpl_108::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e700> as *const (), 2), // +0x39124 sub_39124
    Lifted::new(0x7001e4d16c, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e718> as *const (), 2), // +0x3916c sub_3916c
    Lifted::new(0x7001e4d1a8, tpl_108::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e718> as *const (), 2), // +0x391a8 sub_391a8
    Lifted::new(0x7001e4d1f0, tpl_23::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e730> as *const (), 2), // +0x391f0 sub_391f0
    Lifted::new(0x7001e4d22c, tpl_108::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6e730> as *const (), 2), // +0x3922c sub_3922c
    Lifted::new(0x7001e4d274, fn_7001e4d274 as *const (), 2), // +0x39274 sub_39274
    Lifted::new(0x7001e4d2c0, fn_7001e4d2c0 as *const (), 2), // +0x392c0 sub_392c0
    Lifted::new(0x7001e4d324, tpl_108::<0x7001e6b378, 0x8, 0x7001e6642c, 0x7001e6b4b8> as *const (), 2), // +0x39324 sub_39324
    Lifted::new(0x7001e4d370, fn_7001e4d370 as *const (), 0), // +0x39370 sub_39370
    Lifted::new(0x7001e4d3b4, fn_7001e4d3b4 as *const (), 1), // +0x393b4 sub_393b4
    Lifted::new(0x7001e4d480, tpl_39::<0x7001e15a94, 0x7001e76bf0, 0x7001e66444, 0x7001e4d49f> as *const (), 0), // +0x39480 ?_Xbad_alloc@std@@YAXXZ
    Lifted::new(0x7001e4d4a0, tpl_39::<0x7001e4d04c, 0x7001e77090, 0x7001e66444, 0x7001e4d4bf> as *const (), 0), // +0x394a0 ?_Xbad_function_call@std@@YAXXZ
    Lifted::new(0x7001e4d4c0, tpl_109::<0x7001e4d0a0, 0x7001e76eb8, 0x7001e66444, 0x7001e4d4e2> as *const (), 0), // +0x394c0 ?_Xinvalid_argument@std@@YAXPEBD@Z
    Lifted::new(0x7001e4d4f0, tpl_109::<0x7001e4d124, 0x7001e76f20, 0x7001e66444, 0x7001e4d512> as *const (), 0), // +0x394f0 ?_Xlength_error@std@@YAXPEBD@Z
    Lifted::new(0x7001e4d520, tpl_109::<0x7001e4d1a8, 0x7001e76f88, 0x7001e66444, 0x7001e4d542> as *const (), 0), // +0x39520 ?_Xout_of_range@std@@YAXPEBD@Z
    Lifted::new(0x7001e4d550, tpl_109::<0x7001e4d22c, 0x7001e76ff0, 0x7001e66444, 0x7001e4d572> as *const (), 0), // +0x39550 ?_Xoverflow_error@std@@YAXPEBD@Z
    Lifted::new(0x7001e4d580, fn_7001e4d580 as *const (), 0), // +0x39580 ?_Xregex_error@std@@YAXW4error_type@regex_constants@1@@Z
    Lifted::new(0x7001e4d5b0, tpl_109::<0x7001e4d324, 0x7001e77058, 0x7001e66444, 0x7001e4d5d2> as *const (), 0), // +0x395b0 ?_Xruntime_error@std@@YAXPEBD@Z
    Lifted::new(0x7001e4d5e0, tpl_34::<0x7001e6e6c8> as *const (), 1), // +0x395e0 sub_395e0
    Lifted::new(0x7001e4d5f0, fn_7001e4d5f0 as *const (), 0), // +0x395f0 _Towlower
    Lifted::new(0x7001e4d670, fn_7001e4d670 as *const (), 0), // +0x39670 _Towupper
    Lifted::new(0x7001e4d6f0, fn_7001e4d6f0 as *const (), 0), // +0x396f0 _Wcscoll
    Lifted::new(0x7001e4d7e0, fn_7001e4d7e0 as *const (), 0), // +0x397e0 _Wcsxfrm
    Lifted::new(0x7001e4d900, fn_7001e4d900 as *const (), 2), // +0x39900 _Getcvt
    Lifted::new(0x7001e4d9a0, fn_7001e4d9a0 as *const (), 0), // +0x399a0 _Wcrtomb
    Lifted::alias(0x7001e4da20, fn_7001e4d9a0 as *const (), 0, 0x399a0), // +0x39a20 __Wcrtomb_lk (thunk -> +0x399a0)
    Lifted::new(0x7001e4da30, tpl_105::<0x7001e4da40> as *const (), 0), // +0x39a30 _WStod
    Lifted::new(0x7001e4da40, tpl_106::<0x7001e97380, 0x7001e4e448, 0x7001e4df08, 0x7001e6e5d0, 0x7001e4b658, 0x7001e4b638, 0x7001e4b52c, 0x7001e4b2bc, 0x7001e41c4c, 0x7001e4e5fc, 0x7001e6e5c8, 0x7001e41994, 0x7001e97318, 0x7001e97328, 0x7001e41e34> as *const (), 2), // +0x39a40 _WStodx
    Lifted::new(0x7001e4dcb0, tpl_105::<0x7001e4dcc0> as *const (), 0), // +0x39cb0 _WStof
    Lifted::new(0x7001e4dcc0, tpl_107::<0x7001e97380, 0x7001e4e448, 0x7001e4df08, 0x7001e6e5dc, 0x7001e43184, 0x7001e43100, 0x7001e42ff8, 0x7001e42c88, 0x7001e42518, 0x7001e4e5fc, 0x7001e6e5d8, 0x7001e4233c, 0x7001e97270, 0x7001e97280, 0x7001e426dc> as *const (), 2), // +0x39cc0 _WStofx
    Lifted::new(0x7001e4df08, fn_7001e4df08 as *const (), 0), // +0x39f08 sub_39f08
    Lifted::new(0x7001e4e1d0, tpl_105::<0x7001e4e1e0> as *const (), 0), // +0x3a1d0 _WStold
    Lifted::new(0x7001e4e1e0, tpl_106::<0x7001e97380, 0x7001e4e448, 0x7001e4df08, 0x7001e6e5d0, 0x7001e4a828, 0x7001e4a808, 0x7001e4a6fc, 0x7001e4a384, 0x7001e4350c, 0x7001e4e5fc, 0x7001e6e5c8, 0x7001e41994, 0x7001e972b8, 0x7001e972c8, 0x7001e41e34> as *const (), 2), // +0x3a1e0 _WStoldx
    Lifted::new(0x7001e4e448, fn_7001e4e448 as *const (), 0), // +0x3a448 sub_3a448
    Lifted::new(0x7001e4e5fc, fn_7001e4e5fc as *const (), 0), // +0x3a5fc sub_3a5fc
    Lifted::new(0x7001e4e910, fn_7001e4e910 as *const (), 0), // +0x3a910 sub_3a910
    Lifted::new(0x7001e4e980, tpl_15::<0x7001e69108> as *const (), 2), // +0x3a980 __crtCloseThreadpoolTimer
    Lifted::new(0x7001e4e990, tpl_15::<0x7001e690f0> as *const (), 2), // +0x3a990 __crtCloseThreadpoolWait
    Lifted::new(0x7001e4e9a0, tpl_15::<0x7001e69150> as *const (), 2), // +0x3a9a0 __crtCreateEventExW
    Lifted::new(0x7001e4e9b0, tpl_15::<0x7001e69148> as *const (), 2), // +0x3a9b0 __crtCreateSemaphoreExW
    Lifted::new(0x7001e4e9c0, tpl_15::<0x7001e69248> as *const (), 2), // +0x3a9c0 __crtCreateSymbolicLinkW
    Lifted::new(0x7001e4e9d0, tpl_15::<0x7001e69120> as *const (), 2), // +0x3a9d0 __crtCreateThreadpoolTimer
    Lifted::new(0x7001e4e9e0, tpl_15::<0x7001e69100> as *const (), 2), // +0x3a9e0 __crtCreateThreadpoolWait
    Lifted::new(0x7001e4e9f0, tpl_15::<0x7001e69140> as *const (), 2), // +0x3a9f0 __crtFlushProcessWriteBuffers
    Lifted::new(0x7001e4ea00, tpl_15::<0x7001e691c0> as *const (), 2), // +0x3aa00 __crtFreeLibraryWhenCallbackReturns
    Lifted::new(0x7001e4ea10, tpl_15::<0x7001e69138> as *const (), 2), // +0x3aa10 __crtGetCurrentProcessorNumber
    Lifted::new(0x7001e4ea20, tpl_15::<0x7001e690d8> as *const (), 2), // +0x3aa20 __crtGetFileInformationByHandleEx
    Lifted::new(0x7001e4ea30, fn_7001e4ea30 as *const (), 0), // +0x3aa30 __crtGetSystemTimePreciseAsFileTime
    Lifted::new(0x7001e4ea4c, fn_7001e4ea4c as *const (), 0), // +0x3aa4c sub_3aa4c
    Lifted::new(0x7001e4ea70, tpl_15::<0x7001e69128> as *const (), 2), // +0x3aa70 __crtGetTickCount64
    Lifted::new(0x7001e4ea80, tpl_15::<0x7001e69160> as *const (), 2), // +0x3aa80 __crtInitOnceExecuteOnce
    Lifted::new(0x7001e4ea90, tpl_15::<0x7001e69178> as *const (), 2), // +0x3aa90 __crtInitializeCriticalSectionEx
    Lifted::new(0x7001e4eaa0, fn_7001e4eaa0 as *const (), 0), // +0x3aaa0 __crtIsPackagedApp
    Lifted::new(0x7001e4ead0, tpl_15::<0x7001e69280> as *const (), 2), // +0x3aad0 __crtSetFileInformationByHandle
    Lifted::new(0x7001e4eae0, tpl_15::<0x7001e69118> as *const (), 2), // +0x3aae0 __crtSetThreadpoolTimer
    Lifted::new(0x7001e4eaf0, tpl_15::<0x7001e690f8> as *const (), 2), // +0x3aaf0 __crtSetThreadpoolWait
    Lifted::new(0x7001e4eb00, tpl_15::<0x7001e69110> as *const (), 2), // +0x3ab00 __crtWaitForThreadpoolTimerCallbacks
    Lifted::new(0x7001e4eb08, fn_7001e4eb08 as *const (), 0), // +0x3ab08 sub_3ab08
    Lifted::new(0x7001e4eb3c, fn_7001e4eb3c as *const (), 2), // +0x3ab3c sub_3ab3c
    Lifted::new(0x7001e4ed40, fn_7001e4ed40 as *const (), 2), // +0x3ad40 sub_3ad40
    Lifted::new(0x7001e4ef54, fn_7001e4ef54 as *const (), 2), // +0x3af54 sub_3af54
    Lifted::new(0x7001e4f168, fn_7001e4f168 as *const (), 2), // +0x3b168 sub_3b168
    Lifted::new(0x7001e4f36c, fn_7001e4f36c as *const (), 2), // +0x3b36c sub_3b36c
    Lifted::new(0x7001e4f580, fn_7001e4f580 as *const (), 2), // +0x3b580 sub_3b580
    Lifted::new(0x7001e4f794, fn_7001e4f794 as *const (), 2), // +0x3b794 sub_3b794
    Lifted::new(0x7001e4f998, fn_7001e4f998 as *const (), 2), // +0x3b998 sub_3b998
    Lifted::new(0x7001e4fbac, fn_7001e4fbac as *const (), 2), // +0x3bbac sub_3bbac
    Lifted::new(0x7001e4fdc0, fn_7001e4fdc0 as *const (), 2), // +0x3bdc0 sub_3bdc0
    Lifted::new(0x7001e4ffc4, fn_7001e4ffc4 as *const (), 2), // +0x3bfc4 sub_3bfc4
    Lifted::new(0x7001e501d8, fn_7001e501d8 as *const (), 2), // +0x3c1d8 sub_3c1d8
    Lifted::new(0x7001e503ec, fn_7001e503ec as *const (), 2), // +0x3c3ec sub_3c3ec
    Lifted::new(0x7001e505f0, fn_7001e505f0 as *const (), 2), // +0x3c5f0 sub_3c5f0
    Lifted::new(0x7001e50804, fn_7001e50804 as *const (), 2), // +0x3c804 sub_3c804
    Lifted::new(0x7001e50a18, fn_7001e50a18 as *const (), 2), // +0x3ca18 sub_3ca18
    Lifted::new(0x7001e50c1c, fn_7001e50c1c as *const (), 2), // +0x3cc1c sub_3cc1c
    Lifted::new(0x7001e50e30, fn_7001e50e30 as *const (), 2), // +0x3ce30 sub_3ce30
    Lifted::new(0x7001e51044, fn_7001e51044 as *const (), 2), // +0x3d044 sub_3d044
    Lifted::new(0x7001e51248, fn_7001e51248 as *const (), 2), // +0x3d248 sub_3d248
    Lifted::new(0x7001e5145c, fn_7001e5145c as *const (), 2), // +0x3d45c sub_3d45c
    Lifted::new(0x7001e51670, fn_7001e51670 as *const (), 2), // +0x3d670 sub_3d670
    Lifted::new(0x7001e51874, fn_7001e51874 as *const (), 2), // +0x3d874 sub_3d874
    Lifted::new(0x7001e51a88, fn_7001e51a88 as *const (), 2), // +0x3da88 sub_3da88
    Lifted::new(0x7001e51c9c, fn_7001e51c9c as *const (), 2), // +0x3dc9c sub_3dc9c
    Lifted::new(0x7001e51ea0, fn_7001e51ea0 as *const (), 2), // +0x3dea0 sub_3dea0
    Lifted::new(0x7001e520b4, fn_7001e520b4 as *const (), 2), // +0x3e0b4 sub_3e0b4
    Lifted::new(0x7001e522c8, fn_7001e522c8 as *const (), 2), // +0x3e2c8 sub_3e2c8
    Lifted::new(0x7001e524cc, fn_7001e524cc as *const (), 2), // +0x3e4cc sub_3e4cc
    Lifted::new(0x7001e526e0, fn_7001e526e0 as *const (), 2), // +0x3e6e0 sub_3e6e0
    Lifted::new(0x7001e528f4, fn_7001e528f4 as *const (), 2), // +0x3e8f4 sub_3e8f4
    Lifted::new(0x7001e52af8, fn_7001e52af8 as *const (), 2), // +0x3eaf8 sub_3eaf8
    Lifted::new(0x7001e52d0c, fn_7001e52d0c as *const (), 2), // +0x3ed0c sub_3ed0c
    Lifted::new(0x7001e52f20, fn_7001e52f20 as *const (), 0), // +0x3ef20 ??0?$_Yarn@D@std@@QEAA@AEBV01@@Z
    Lifted::new(0x7001e52f50, fn_7001e52f50 as *const (), 0), // +0x3ef50 ??0?$_Yarn@G@std@@QEAA@AEBV01@@Z
    Lifted::new(0x7001e52f80, fn_7001e52f80 as *const (), 0), // +0x3ef80 ??0?$_Yarn@G@std@@QEAA@PEBG@Z
    Lifted::new(0x7001e52fa0, fn_7001e52fa0 as *const (), 0), // +0x3efa0 ??0?$basic_ios@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z
    Lifted::new(0x7001e53000, fn_7001e53000 as *const (), 0), // +0x3f000 ??0?$basic_ios@GU?$char_traits@G@std@@@std@@QEAA@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@@Z
    Lifted::new(0x7001e53060, fn_7001e53060 as *const (), 0), // +0x3f060 ??0?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@@Z
    Lifted::new(0x7001e530c0, fn_7001e530c0 as *const (), 0), // +0x3f0c0 ??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e53240, fn_7001e53240 as *const (), 0), // +0x3f240 ??0?$basic_iostream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z
    Lifted::new(0x7001e53340, fn_7001e53340 as *const (), 0), // +0x3f340 ??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e534c0, fn_7001e534c0 as *const (), 0), // +0x3f4c0 ??0?$basic_iostream@GU?$char_traits@G@std@@@std@@QEAA@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@@Z
    Lifted::new(0x7001e535c0, fn_7001e535c0 as *const (), 0), // +0x3f5c0 ??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e53740, fn_7001e53740 as *const (), 0), // +0x3f740 ??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@@Z
    Lifted::new(0x7001e53840, fn_7001e53840 as *const (), 0), // +0x3f840 ??0?$basic_istream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e53950, fn_7001e53950 as *const (), 0), // +0x3f950 ??0?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N1@Z
    Lifted::new(0x7001e53a00, fn_7001e53a00 as *const (), 0), // +0x3fa00 ??0?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA@W4_Uninitialized@1@@Z
    Lifted::new(0x7001e53aa0, fn_7001e53aa0 as *const (), 0), // +0x3faa0 ??0?$basic_istream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e53bc0, fn_7001e53bc0 as *const (), 0), // +0x3fbc0 ??0?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@_N1@Z
    Lifted::new(0x7001e53c70, fn_7001e53c70 as *const (), 0), // +0x3fc70 ??0?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA@W4_Uninitialized@1@@Z
    Lifted::new(0x7001e53d20, fn_7001e53d20 as *const (), 0), // +0x3fd20 ??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e53e40, fn_7001e53e40 as *const (), 0), // +0x3fe40 ??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@_N1@Z
    Lifted::new(0x7001e53ef0, fn_7001e53ef0 as *const (), 0), // +0x3fef0 ??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA@W4_Uninitialized@1@@Z
    Lifted::new(0x7001e53fa0, fn_7001e53fa0 as *const (), 0), // +0x3ffa0 ??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e540b0, fn_7001e540b0 as *const (), 0), // +0x400b0 ??0?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@W4_Uninitialized@1@_N@Z
    Lifted::new(0x7001e54160, fn_7001e54160 as *const (), 0), // +0x40160 ??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e54270, fn_7001e54270 as *const (), 0), // +0x40270 ??0?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAA@W4_Uninitialized@1@_N@Z
    Lifted::new(0x7001e54320, fn_7001e54320 as *const (), 0), // +0x40320 ??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z
    Lifted::new(0x7001e54430, fn_7001e54430 as *const (), 0), // +0x40430 ??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAA@W4_Uninitialized@1@_N@Z
    Lifted::new(0x7001e544e0, fn_7001e544e0 as *const (), 0), // +0x404e0 ??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@AEBV01@@Z
    Lifted::new(0x7001e54640, tpl_110::<0x7001e6bca8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const (), 1), // +0x40640 ??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@W4_Uninitialized@1@@Z
    Lifted::new(0x7001e54680, tpl_111::<0x7001e6d2b8, 0x7001e659fc, 0x7001e696a0> as *const (), 0), // +0x40680 ??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@AEBV01@@Z
    Lifted::new(0x7001e547d0, tpl_110::<0x7001e6d2b8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const (), 1), // +0x407d0 ??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@W4_Uninitialized@1@@Z
    Lifted::new(0x7001e54810, tpl_111::<0x7001e6d3f8, 0x7001e659fc, 0x7001e696a0> as *const (), 0), // +0x40810 ??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@AEBV01@@Z
    Lifted::new(0x7001e54960, tpl_110::<0x7001e6d3f8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const (), 1), // +0x40960 ??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@W4_Uninitialized@1@@Z
    Lifted::new(0x7001e549a0, fn_7001e549a0 as *const (), 0), // +0x409a0 ??0?$codecvt@DDU_Mbstatet@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e549f0, fn_7001e549f0 as *const (), 0), // +0x409f0 ??0?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54a40, fn_7001e54a40 as *const (), 0), // +0x40a40 ??0?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54a90, fn_7001e54a90 as *const (), 0), // +0x40a90 ??0?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54ae0, fn_7001e54ae0 as *const (), 0), // +0x40ae0 ??0?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54b30, fn_7001e54b30 as *const (), 0), // +0x40b30 ??0?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54b80, fn_7001e54b80 as *const (), 0), // +0x40b80 ??0?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54bd0, fn_7001e54bd0 as *const (), 0), // +0x40bd0 ??0?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAA@PEBD_K@Z
    Lifted::new(0x7001e54c40, fn_7001e54c40 as *const (), 0), // +0x40c40 ??0?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54cc0, fn_7001e54cc0 as *const (), 0), // +0x40cc0 ??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAA@PEBD_K@Z
    Lifted::new(0x7001e54d30, fn_7001e54d30 as *const (), 0), // +0x40d30 ??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54db0, fn_7001e54db0 as *const (), 0), // +0x40db0 ??0?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAA@PEBD_K@Z
    Lifted::new(0x7001e54e20, fn_7001e54e20 as *const (), 0), // +0x40e20 ??0?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54ea0, fn_7001e54ea0 as *const (), 0), // +0x40ea0 ??0?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e54f30, fn_7001e54f30 as *const (), 0), // +0x40f30 ??0?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAA@PEBD_K@Z
    Lifted::new(0x7001e54fc0, fn_7001e54fc0 as *const (), 0), // +0x40fc0 ??0?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e55050, fn_7001e55050 as *const (), 0), // +0x41050 ??0?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAA@PEBD_K@Z
    Lifted::new(0x7001e550e0, fn_7001e550e0 as *const (), 0), // +0x410e0 ??0?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAA@_K@Z
    Lifted::new(0x7001e55170, tpl_112::<0xffffffffffffffe0, 0x4, 0x7001e6edc0, 0xffffffffffffffdc, 0xfffffffffffffff0, 0x7001e6bc90, 0xffffffffffffffec, 0x7001e6be18, 0xffffffffffffffe8> as *const (), 1), // +0x41170 ??1?$basic_iostream@DU?$char_traits@D@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e551f0, tpl_112::<0xffffffffffffffe0, 0x4, 0x7001e6ee00, 0xffffffffffffffdc, 0xfffffffffffffff0, 0x7001e6d3c8, 0xffffffffffffffec, 0x7001e6d3e0, 0xffffffffffffffe8> as *const (), 1), // +0x411f0 ??1?$basic_iostream@GU?$char_traits@G@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e55270, tpl_112::<0xffffffffffffffe0, 0x4, 0x7001e6ede0, 0xffffffffffffffdc, 0xfffffffffffffff0, 0x7001e6d508, 0xffffffffffffffec, 0x7001e6d520, 0xffffffffffffffe8> as *const (), 1), // +0x41270 ??1?$basic_iostream@_WU?$char_traits@_W@std@@@std@@UEAA@XZ
    Lifted::new(0x7001e552e4, fn_7001e552e4 as *const (), 0), // +0x412e4 sub_412e4
    Lifted::new(0x7001e55310, tpl_113::<0x7001e18ef0> as *const (), 0), // +0x41310 ??4?$_Yarn@D@std@@QEAAAEAV01@AEBV01@@Z
    Lifted::new(0x7001e55320, tpl_113::<0x7001e18f70> as *const (), 0), // +0x41320 ??4?$_Yarn@G@std@@QEAAAEAV01@AEBV01@@Z
    Lifted::new(0x7001e55330, fn_7001e55330 as *const (), 0), // +0x41330 ??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z
    Lifted::new(0x7001e553a0, fn_7001e553a0 as *const (), 0), // +0x413a0 ??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z
    Lifted::new(0x7001e55410, tpl_21::<0x7001e64260> as *const (), 0), // +0x41410 ??4?$basic_istream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z
    Lifted::new(0x7001e55430, tpl_21::<0x7001e642e0> as *const (), 0), // +0x41430 ??4?$basic_istream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z
    Lifted::new(0x7001e55450, fn_7001e55450 as *const (), 0), // +0x41450 ??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAAEAV01@AEBV01@@Z
    Lifted::new(0x7001e55570, fn_7001e55570 as *const (), 0), // +0x41570 ??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAAEAV01@AEBV01@@Z
    Lifted::new(0x7001e55680, fn_7001e55680 as *const (), 2), // +0x41680 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAF@Z
    Lifted::alias(0x7001e558c0, fn_7001e4eb3c as *const (), 2, 0x3ab3c), // +0x418c0 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAG@Z (thunk -> +0x3ab3c)
    Lifted::new(0x7001e558d0, tpl_114::<0x7001e4f794> as *const (), 0), // +0x418d0 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAH@Z
    Lifted::alias(0x7001e55910, fn_7001e4f168 as *const (), 2, 0x3b168), // +0x41910 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAI@Z (thunk -> +0x3b168)
    Lifted::alias(0x7001e55920, fn_7001e4f794 as *const (), 2, 0x3b794), // +0x41920 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAJ@Z (thunk -> +0x3b794)
    Lifted::alias(0x7001e55930, fn_7001e4fdc0 as *const (), 2, 0x3bdc0), // +0x41930 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAK@Z (thunk -> +0x3bdc0)
    Lifted::alias(0x7001e55940, fn_7001e503ec as *const (), 2, 0x3c3ec), // +0x41940 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAM@Z (thunk -> +0x3c3ec)
    Lifted::alias(0x7001e55950, fn_7001e50a18 as *const (), 2, 0x3ca18), // +0x41950 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAN@Z (thunk -> +0x3ca18)
    Lifted::alias(0x7001e55960, fn_7001e51044 as *const (), 2, 0x3d044), // +0x41960 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAO@Z (thunk -> +0x3d044)
    Lifted::alias(0x7001e55970, fn_7001e51670 as *const (), 2, 0x3d670), // +0x41970 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEAPEAX@Z (thunk -> +0x3d670)
    Lifted::alias(0x7001e55980, fn_7001e51c9c as *const (), 2, 0x3dc9c), // +0x41980 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEA_J@Z (thunk -> +0x3dc9c)
    Lifted::alias(0x7001e55990, fn_7001e522c8 as *const (), 2, 0x3e2c8), // +0x41990 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEA_K@Z (thunk -> +0x3e2c8)
    Lifted::alias(0x7001e559a0, fn_7001e528f4 as *const (), 2, 0x3e8f4), // +0x419a0 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@AEA_N@Z (thunk -> +0x3e8f4)
    Lifted::new(0x7001e559b0, fn_7001e559b0 as *const (), 0), // +0x419b0 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@P6AAEAV01@AEAV01@@Z@Z
    Lifted::new(0x7001e559c0, fn_7001e559c0 as *const (), 0), // +0x419c0 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@P6AAEAV?$basic_ios@DU?$char_traits@D@std@@@1@AEAV21@@Z@Z
    Lifted::new(0x7001e559f0, fn_7001e559f0 as *const (), 0), // +0x419f0 ??5?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z
    Lifted::new(0x7001e55c20, fn_7001e55c20 as *const (), 2), // +0x41c20 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAF@Z
    Lifted::alias(0x7001e55e70, fn_7001e4ed40 as *const (), 2, 0x3ad40), // +0x41e70 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAG@Z (thunk -> +0x3ad40)
    Lifted::new(0x7001e55e80, tpl_114::<0x7001e4f998> as *const (), 0), // +0x41e80 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAH@Z
    Lifted::alias(0x7001e55ec0, fn_7001e4f36c as *const (), 2, 0x3b36c), // +0x41ec0 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAI@Z (thunk -> +0x3b36c)
    Lifted::alias(0x7001e55ed0, fn_7001e4f998 as *const (), 2, 0x3b998), // +0x41ed0 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAJ@Z (thunk -> +0x3b998)
    Lifted::alias(0x7001e55ee0, fn_7001e4ffc4 as *const (), 2, 0x3bfc4), // +0x41ee0 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAK@Z (thunk -> +0x3bfc4)
    Lifted::alias(0x7001e55ef0, fn_7001e505f0 as *const (), 2, 0x3c5f0), // +0x41ef0 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAM@Z (thunk -> +0x3c5f0)
    Lifted::alias(0x7001e55f00, fn_7001e50c1c as *const (), 2, 0x3cc1c), // +0x41f00 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAN@Z (thunk -> +0x3cc1c)
    Lifted::alias(0x7001e55f10, fn_7001e51248 as *const (), 2, 0x3d248), // +0x41f10 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAO@Z (thunk -> +0x3d248)
    Lifted::alias(0x7001e55f20, fn_7001e51874 as *const (), 2, 0x3d874), // +0x41f20 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAPEAX@Z (thunk -> +0x3d874)
    Lifted::alias(0x7001e55f30, fn_7001e51ea0 as *const (), 2, 0x3dea0), // +0x41f30 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEA_J@Z (thunk -> +0x3dea0)
    Lifted::alias(0x7001e55f40, fn_7001e524cc as *const (), 2, 0x3e4cc), // +0x41f40 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEA_K@Z (thunk -> +0x3e4cc)
    Lifted::alias(0x7001e55f50, fn_7001e52af8 as *const (), 2, 0x3eaf8), // +0x41f50 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEA_N@Z (thunk -> +0x3eaf8)
    Lifted::new(0x7001e55f60, fn_7001e55f60 as *const (), 0), // +0x41f60 ??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@@Z
    Lifted::new(0x7001e56190, fn_7001e56190 as *const (), 2), // +0x42190 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAF@Z
    Lifted::alias(0x7001e563e0, fn_7001e4ef54 as *const (), 2, 0x3af54), // +0x423e0 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAG@Z (thunk -> +0x3af54)
    Lifted::new(0x7001e563f0, tpl_114::<0x7001e4fbac> as *const (), 0), // +0x423f0 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAH@Z
    Lifted::alias(0x7001e56430, fn_7001e4f580 as *const (), 2, 0x3b580), // +0x42430 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAI@Z (thunk -> +0x3b580)
    Lifted::alias(0x7001e56440, fn_7001e4fbac as *const (), 2, 0x3bbac), // +0x42440 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAJ@Z (thunk -> +0x3bbac)
    Lifted::alias(0x7001e56450, fn_7001e501d8 as *const (), 2, 0x3c1d8), // +0x42450 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAK@Z (thunk -> +0x3c1d8)
    Lifted::alias(0x7001e56460, fn_7001e50804 as *const (), 2, 0x3c804), // +0x42460 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAM@Z (thunk -> +0x3c804)
    Lifted::alias(0x7001e56470, fn_7001e50e30 as *const (), 2, 0x3ce30), // +0x42470 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAN@Z (thunk -> +0x3ce30)
    Lifted::alias(0x7001e56480, fn_7001e5145c as *const (), 2, 0x3d45c), // +0x42480 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAO@Z (thunk -> +0x3d45c)
    Lifted::alias(0x7001e56490, fn_7001e51a88 as *const (), 2, 0x3da88), // +0x42490 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEAPEAX@Z (thunk -> +0x3da88)
    Lifted::alias(0x7001e564a0, fn_7001e520b4 as *const (), 2, 0x3e0b4), // +0x424a0 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEA_J@Z (thunk -> +0x3e0b4)
    Lifted::alias(0x7001e564b0, fn_7001e526e0 as *const (), 2, 0x3e6e0), // +0x424b0 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEA_K@Z (thunk -> +0x3e6e0)
    Lifted::alias(0x7001e564c0, fn_7001e52d0c as *const (), 2, 0x3ed0c), // +0x424c0 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@AEA_N@Z (thunk -> +0x3ed0c)
    Lifted::new(0x7001e564d0, fn_7001e564d0 as *const (), 0), // +0x424d0 ??5?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@@Z
    Lifted::new(0x7001e56700, fn_7001e56700 as *const (), 2), // +0x42700 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@F@Z
    Lifted::new(0x7001e56900, fn_7001e56900 as *const (), 2), // +0x42900 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@G@Z
    Lifted::new(0x7001e56ae0, fn_7001e56ae0 as *const (), 2), // +0x42ae0 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@H@Z
    Lifted::new(0x7001e56cc0, fn_7001e56cc0 as *const (), 2), // +0x42cc0 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@I@Z
    Lifted::new(0x7001e56ea0, fn_7001e56ea0 as *const (), 2), // +0x42ea0 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@J@Z
    Lifted::new(0x7001e57080, fn_7001e57080 as *const (), 2), // +0x43080 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@M@Z
    Lifted::new(0x7001e57280, fn_7001e57280 as *const (), 2), // +0x43280 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@N@Z
    Lifted::new(0x7001e57470, fn_7001e57470 as *const (), 2), // +0x43470 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@O@Z
    Lifted::new(0x7001e57660, fn_7001e57660 as *const (), 0), // +0x43660 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z
    Lifted::new(0x7001e57880, fn_7001e57880 as *const (), 2), // +0x43880 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@PEBX@Z
    Lifted::new(0x7001e57a60, fn_7001e57a60 as *const (), 2), // +0x43a60 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_J@Z
    Lifted::new(0x7001e57c40, fn_7001e57c40 as *const (), 2), // +0x43c40 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_K@Z
    Lifted::new(0x7001e57e20, fn_7001e57e20 as *const (), 2), // +0x43e20 ??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_N@Z
    Lifted::new(0x7001e58000, fn_7001e58000 as *const (), 2), // +0x44000 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@F@Z
    Lifted::new(0x7001e58200, fn_7001e58200 as *const (), 2), // +0x44200 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@G@Z
    Lifted::new(0x7001e583e0, fn_7001e583e0 as *const (), 2), // +0x443e0 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@H@Z
    Lifted::new(0x7001e585c0, fn_7001e585c0 as *const (), 2), // +0x445c0 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@I@Z
    Lifted::new(0x7001e587a0, fn_7001e587a0 as *const (), 2), // +0x447a0 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@J@Z
    Lifted::new(0x7001e58980, fn_7001e58980 as *const (), 2), // +0x44980 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@M@Z
    Lifted::new(0x7001e58b80, fn_7001e58b80 as *const (), 2), // +0x44b80 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@N@Z
    Lifted::new(0x7001e58d70, fn_7001e58d70 as *const (), 2), // +0x44d70 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@O@Z
    Lifted::new(0x7001e58f60, fn_7001e58f60 as *const (), 0), // +0x44f60 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@PEAV?$basic_streambuf@GU?$char_traits@G@std@@@1@@Z
    Lifted::new(0x7001e59180, fn_7001e59180 as *const (), 2), // +0x45180 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@PEBX@Z
    Lifted::new(0x7001e59360, fn_7001e59360 as *const (), 2), // +0x45360 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@_J@Z
    Lifted::new(0x7001e59540, fn_7001e59540 as *const (), 2), // +0x45540 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@_K@Z
    Lifted::new(0x7001e59720, fn_7001e59720 as *const (), 2), // +0x45720 ??6?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@_N@Z
    Lifted::new(0x7001e59900, fn_7001e59900 as *const (), 2), // +0x45900 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@F@Z
    Lifted::new(0x7001e59b00, fn_7001e59b00 as *const (), 2), // +0x45b00 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@G@Z
    Lifted::new(0x7001e59ce0, fn_7001e59ce0 as *const (), 2), // +0x45ce0 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@H@Z
    Lifted::new(0x7001e59ec0, fn_7001e59ec0 as *const (), 2), // +0x45ec0 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@I@Z
    Lifted::new(0x7001e5a0a0, fn_7001e5a0a0 as *const (), 2), // +0x460a0 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@J@Z
    Lifted::new(0x7001e5a280, fn_7001e5a280 as *const (), 2), // +0x46280 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@M@Z
    Lifted::new(0x7001e5a480, fn_7001e5a480 as *const (), 2), // +0x46480 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@N@Z
    Lifted::new(0x7001e5a670, fn_7001e5a670 as *const (), 2), // +0x46670 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@O@Z
    Lifted::new(0x7001e5a860, fn_7001e5a860 as *const (), 2), // +0x46860 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@PEBX@Z
    Lifted::new(0x7001e5aa40, fn_7001e5aa40 as *const (), 2), // +0x46a40 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@_J@Z
    Lifted::new(0x7001e5ac20, fn_7001e5ac20 as *const (), 2), // +0x46c20 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@_K@Z
    Lifted::new(0x7001e5ae00, fn_7001e5ae00 as *const (), 2), // +0x46e00 ??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@_N@Z
    Lifted::new(0x7001e5afe0, fn_7001e5afe0 as *const (), 0), // +0x46fe0 ??_D?$basic_iostream@DU?$char_traits@D@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5b070, fn_7001e5b070 as *const (), 0), // +0x47070 ??_D?$basic_iostream@GU?$char_traits@G@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5b100, fn_7001e5b100 as *const (), 0), // +0x47100 ??_D?$basic_iostream@_WU?$char_traits@_W@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5b190, fn_7001e5b190 as *const (), 0), // +0x47190 sub_47190
    Lifted::new(0x7001e5b220, tpl_30::<0xfffffffffffffffc, 0x7001e5b22c> as *const (), 0), // +0x47220 sub_47220
    Lifted::new(0x7001e5b22c, fn_7001e5b22c as *const (), 0), // +0x4722c sub_4722c
    Lifted::new(0x7001e5b2b0, tpl_30::<0xfffffffffffffffc, 0x7001e5b2bc> as *const (), 0), // +0x472b0 sub_472b0
    Lifted::new(0x7001e5b2bc, fn_7001e5b2bc as *const (), 0), // +0x472bc sub_472bc
    Lifted::new(0x7001e5b340, tpl_30::<0xfffffffffffffffc, 0x7001e5b34c> as *const (), 0), // +0x47340 sub_47340
    Lifted::new(0x7001e5b34c, fn_7001e5b34c as *const (), 0), // +0x4734c sub_4734c
    Lifted::new(0x7001e5b3cc, fn_7001e5b3cc as *const (), 0), // +0x473cc sub_473cc
    Lifted::new(0x7001e5b47c, fn_7001e5b47c as *const (), 0), // +0x4747c sub_4747c
    Lifted::new(0x7001e5b52c, fn_7001e5b52c as *const (), 0), // +0x4752c sub_4752c
    Lifted::new(0x7001e5b5dc, fn_7001e5b5dc as *const (), 0), // +0x475dc sub_475dc
    Lifted::new(0x7001e5b68c, fn_7001e5b68c as *const (), 0), // +0x4768c sub_4768c
    Lifted::new(0x7001e5b73c, fn_7001e5b73c as *const (), 0), // +0x4773c sub_4773c
    Lifted::new(0x7001e5b7f0, fn_7001e5b7f0 as *const (), 0), // +0x477f0 sub_477f0
    Lifted::new(0x7001e5b8c0, fn_7001e5b8c0 as *const (), 0), // +0x478c0 sub_478c0
    Lifted::new(0x7001e5b990, fn_7001e5b990 as *const (), 0), // +0x47990 sub_47990
    Lifted::new(0x7001e5ba60, fn_7001e5ba60 as *const (), 0), // +0x47a60 sub_47a60
    Lifted::new(0x7001e5bb10, fn_7001e5bb10 as *const (), 0), // +0x47b10 sub_47b10
    Lifted::new(0x7001e5bbc0, fn_7001e5bbc0 as *const (), 0), // +0x47bc0 sub_47bc0
    Lifted::new(0x7001e5bc70, fn_7001e5bc70 as *const (), 0), // +0x47c70 sub_47c70
    Lifted::new(0x7001e5bd10, fn_7001e5bd10 as *const (), 0), // +0x47d10 sub_47d10
    Lifted::new(0x7001e5bdb0, fn_7001e5bdb0 as *const (), 0), // +0x47db0 sub_47db0
    Lifted::new(0x7001e5be50, fn_7001e5be50 as *const (), 0), // +0x47e50 ??_F?$codecvt@DDU_Mbstatet@@@std@@QEAAXXZ
    Lifted::new(0x7001e5be90, fn_7001e5be90 as *const (), 0), // +0x47e90 ??_F?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5bed0, fn_7001e5bed0 as *const (), 0), // +0x47ed0 ??_F?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5bf10, fn_7001e5bf10 as *const (), 0), // +0x47f10 ??_F?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5bf50, fn_7001e5bf50 as *const (), 0), // +0x47f50 ??_F?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5bf90, fn_7001e5bf90 as *const (), 0), // +0x47f90 ??_F?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5bfd0, fn_7001e5bfd0 as *const (), 0), // +0x47fd0 ??_F?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c010, tpl_13::<0x7001e54c40> as *const (), 0), // +0x48010 ??_F?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c020, tpl_13::<0x7001e54d30> as *const (), 0), // +0x48020 ??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c030, tpl_13::<0x7001e54e20> as *const (), 0), // +0x48030 ??_F?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c040, tpl_13::<0x7001e54ea0> as *const (), 0), // +0x48040 ??_F?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c050, tpl_13::<0x7001e54fc0> as *const (), 0), // +0x48050 ??_F?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c060, tpl_13::<0x7001e550e0> as *const (), 0), // +0x48060 ??_F?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e5c070, fn_7001e5c070 as *const (), 2), // +0x48070 ?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits
    Lifted::new(0x7001e5c0c0, tpl_115::<0x7001e29480> as *const (), 2), // +0x480c0 ?_Fput@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@GU?$char_traits
    Lifted::new(0x7001e5c120, tpl_115::<0x7001e29900> as *const (), 2), // +0x48120 ?_Fput@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@_WU?$char_tr
    Lifted::new(0x7001e5c180, fn_7001e5c180 as *const (), 0), // +0x48180 ?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$ch
    Lifted::new(0x7001e5c280, fn_7001e5c280 as *const (), 0), // +0x48280 ?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$ch
    Lifted::new(0x7001e5c380, fn_7001e5c380 as *const (), 0), // +0x48380 ?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU
    Lifted::new(0x7001e5c480, fn_7001e5c480 as *const (), 0), // +0x48480 ?_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$c
    Lifted::new(0x7001e5c580, fn_7001e5c580 as *const (), 0), // +0x48580 ?_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$c
    Lifted::new(0x7001e5c680, fn_7001e5c680 as *const (), 0), // +0x48680 ?_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_W
    Lifted::new(0x7001e5c780, tpl_116::<0x7001e6eee8, 0x7001e2132c> as *const (), 0), // +0x48780 ?_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$ch
    Lifted::new(0x7001e5c7f0, tpl_116::<0x7001e6ee48, 0x7001e2e1bc> as *const (), 0), // +0x487f0 ?_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$ch
    Lifted::new(0x7001e5c860, tpl_116::<0x7001e6ee98, 0x7001e2ecdc> as *const (), 0), // +0x48860 ?_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU
    Lifted::new(0x7001e5c8d0, fn_7001e5c8d0 as *const (), 1), // +0x488d0 ?_Gnpreinc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ
    Lifted::new(0x7001e5c8f0, fn_7001e5c8f0 as *const (), 1), // +0x488f0 ?_Gnpreinc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAPEAGXZ
    Lifted::new(0x7001e5c910, fn_7001e5c910 as *const (), 0), // +0x48910 ?_Ipfx@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA_N_N@Z
    Lifted::new(0x7001e5cb70, fn_7001e5cb70 as *const (), 0), // +0x48b70 ?_Ipfx@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA_N_N@Z
    Lifted::new(0x7001e5cdf0, fn_7001e5cdf0 as *const (), 0), // +0x48df0 ?_Ipfx@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA_N_N@Z
    Lifted::new(0x7001e5d070, fn_7001e5d070 as *const (), 0), // +0x49070 ?clear@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXI@Z
    Lifted::new(0x7001e5d100, fn_7001e5d100 as *const (), 0), // +0x49100 ?copyfmt@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAAEAV12@AEBV12@@Z
    Lifted::new(0x7001e5d130, fn_7001e5d130 as *const (), 0), // +0x49130 ?copyfmt@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAAEAV12@AEBV12@@Z
    Lifted::new(0x7001e5d160, fn_7001e5d160 as *const (), 1), // +0x49160 ?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAADD@Z
    Lifted::new(0x7001e5d170, tpl_62::<0x58> as *const (), 1), // +0x49170 ?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ
    Lifted::new(0x7001e5d180, fn_7001e5d180 as *const (), 1), // +0x49180 ?fill@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAGG@Z
    Lifted::new(0x7001e5d190, tpl_88::<0x58> as *const (), 1), // +0x49190 ?fill@?$basic_ios@GU?$char_traits@G@std@@@std@@QEBAGXZ
    Lifted::new(0x7001e5d1a0, tpl_55::<0x8> as *const (), 1), // +0x491a0 ?gcount@?$basic_istream@DU?$char_traits@D@std@@@std@@QEBA_JXZ
    Lifted::new(0x7001e5d1b0, fn_7001e5d1b0 as *const (), 0), // +0x491b0 ?get@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@AEAD@Z
    Lifted::new(0x7001e5d1e0, fn_7001e5d1e0 as *const (), 0), // +0x491e0 ?get@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@AEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z
    Lifted::new(0x7001e5d220, fn_7001e5d220 as *const (), 0), // +0x49220 ?get@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@AEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@D@Z
    Lifted::new(0x7001e5d460, tpl_117::<0x4, 0x7001e1d400, 0x7001e5d4b0> as *const (), 0), // +0x49460 ?get@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_J@Z
    Lifted::new(0x7001e5d4b0, fn_7001e5d4b0 as *const (), 0), // +0x494b0 ?get@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_JD@Z
    Lifted::new(0x7001e5d6a0, fn_7001e5d6a0 as *const (), 0), // +0x496a0 ?get@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e5d860, tpl_118::<0x7001e5dd70> as *const (), 0), // +0x49860 ?get@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@AEAG@Z
    Lifted::new(0x7001e5d890, tpl_119::<0x4, 0x7001e282b0, 0x7001e5d8d0> as *const (), 0), // +0x49890 ?get@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@AEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z
    Lifted::new(0x7001e5d8d0, fn_7001e5d8d0 as *const (), 0), // +0x498d0 ?get@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@AEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@G@Z
    Lifted::new(0x7001e5db20, tpl_120::<0x4, 0x7001e282b0, 0x7001e5db70> as *const (), 0), // +0x49b20 ?get@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@PEAG_J@Z
    Lifted::new(0x7001e5db70, fn_7001e5db70 as *const (), 0), // +0x49b70 ?get@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@PEAG_JG@Z
    Lifted::new(0x7001e5dd70, fn_7001e5dd70 as *const (), 0), // +0x49d70 ?get@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e5df40, tpl_119::<0x4, 0x7001e28fb0, 0x7001e5df80> as *const (), 0), // +0x49f40 ?get@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@AEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z
    Lifted::new(0x7001e5df80, fn_7001e5df80 as *const (), 0), // +0x49f80 ?get@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@AEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@_W@Z
    Lifted::new(0x7001e5e1d0, tpl_118::<0x7001e5e450> as *const (), 0), // +0x4a1d0 ?get@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@AEA_W@Z
    Lifted::new(0x7001e5e200, tpl_120::<0x4, 0x7001e28fb0, 0x7001e5e250> as *const (), 0), // +0x4a200 ?get@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@PEA_W_J@Z
    Lifted::new(0x7001e5e250, fn_7001e5e250 as *const (), 0), // +0x4a250 ?get@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@PEA_W_J_W@Z
    Lifted::new(0x7001e5e450, fn_7001e5e450 as *const (), 0), // +0x4a450 ?get@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e5e620, tpl_96::<0x60, 0x7001e696a0> as *const (), 2), // +0x4a620 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e690, tpl_96::<0x58, 0x7001e696a0> as *const (), 2), // +0x4a690 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e700, tpl_96::<0x50, 0x7001e696a0> as *const (), 2), // +0x4a700 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e770, tpl_96::<0x48, 0x7001e696a0> as *const (), 2), // +0x4a770 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e7e0, tpl_96::<0x28, 0x7001e696a0> as *const (), 2), // +0x4a7e0 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e850, tpl_96::<0x20, 0x7001e696a0> as *const (), 2), // +0x4a850 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e8c0, tpl_96::<0x18, 0x7001e696a0> as *const (), 2), // +0x4a8c0 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e930, tpl_96::<0x68, 0x7001e696a0> as *const (), 2), // +0x4a930 ?get@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e5e9a0, fn_7001e5e9a0 as *const (), 2), // +0x4a9a0 ?get@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@
    Lifted::new(0x7001e5ea20, fn_7001e5ea20 as *const (), 2), // +0x4aa20 ?get@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@DU?$char_traits@
    Lifted::new(0x7001e5ee00, fn_7001e5ee00 as *const (), 2), // +0x4ae00 ?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@
    Lifted::new(0x7001e5f220, fn_7001e5f220 as *const (), 2), // +0x4b220 ?get@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@_WU?$char_tra
    Lifted::new(0x7001e5f640, tpl_117::<0x4, 0x7001e1d400, 0x7001e5f690> as *const (), 0), // +0x4b640 ?getline@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_J@Z
    Lifted::new(0x7001e5f690, fn_7001e5f690 as *const (), 0), // +0x4b690 ?getline@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_JD@Z
    Lifted::new(0x7001e5f8f0, tpl_120::<0x4, 0x7001e282b0, 0x7001e5f940> as *const (), 0), // +0x4b8f0 ?getline@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@PEAG_J@Z
    Lifted::new(0x7001e5f940, fn_7001e5f940 as *const (), 0), // +0x4b940 ?getline@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@PEAG_JG@Z
    Lifted::new(0x7001e5fb90, tpl_120::<0x4, 0x7001e28fb0, 0x7001e5fbe0> as *const (), 0), // +0x4bb90 ?getline@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@PEA_W_J@Z
    Lifted::new(0x7001e5fbe0, fn_7001e5fbe0 as *const (), 0), // +0x4bbe0 ?getline@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@PEA_W_J_W@Z
    Lifted::new(0x7001e5fe30, tpl_49::<0x60, 0x8, 0x7001e696a0> as *const (), 0), // +0x4be30 ?getloc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEBA?AVlocale@2@XZ
    Lifted::new(0x7001e5fe60, fn_7001e5fe60 as *const (), 0), // +0x4be60 ?ignore@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@_JH@Z
    Lifted::new(0x7001e60020, fn_7001e60020 as *const (), 0), // +0x4c020 ?ignore@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@_JG@Z
    Lifted::new(0x7001e601f0, fn_7001e601f0 as *const (), 0), // +0x4c1f0 ?ignore@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@_JG@Z
    Lifted::new(0x7001e603c0, fn_7001e603c0 as *const (), 0), // +0x4c3c0 ?imbue@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAA?AVlocale@2@AEBV32@@Z
    Lifted::new(0x7001e60450, fn_7001e60450 as *const (), 0), // +0x4c450 ?in_avail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JXZ
    Lifted::alias(0x7001e60490, fn_7001e5c910 as *const (), 0, 0x48910), // +0x4c490 ?ipfx@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA_N_N@Z (thunk -> +0x48910)
    Lifted::alias(0x7001e604a0, fn_7001e5cb70 as *const (), 0, 0x48b70), // +0x4c4a0 ?ipfx@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA_N_N@Z (thunk -> +0x48b70)
    Lifted::alias(0x7001e604b0, fn_7001e5cdf0 as *const (), 0, 0x48df0), // +0x4c4b0 ?ipfx@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA_N_N@Z (thunk -> +0x48df0)
    Lifted::new(0x7001e604c0, fn_7001e604c0 as *const (), 0), // +0x4c4c0 ?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAX$$QEAV12@@Z
    Lifted::new(0x7001e60510, fn_7001e60510 as *const (), 0), // +0x4c510 ?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAX$$QEAV12@@Z
    Lifted::new(0x7001e60560, fn_7001e60560 as *const (), 0), // +0x4c560 ?narrow@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADDD@Z
    Lifted::new(0x7001e60600, fn_7001e60600 as *const (), 0), // +0x4c600 ?narrow@?$basic_ios@GU?$char_traits@G@std@@@std@@QEBADGD@Z
    Lifted::new(0x7001e606a0, fn_7001e606a0 as *const (), 0), // +0x4c6a0 ?narrow@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEBAD_WD@Z
    Lifted::new(0x7001e60740, fn_7001e60740 as *const (), 0), // +0x4c740 ?opfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA_NXZ
    Lifted::alias(0x7001e60790, fn_7001e1f210 as *const (), 0, 0xb210), // +0x4c790 ?osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ (thunk -> +0xb210)
    Lifted::new(0x7001e607a0, tpl_47::<0x20> as *const (), 1), // +0x4c7a0 ?pbase@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ
    Lifted::new(0x7001e607b0, fn_7001e607b0 as *const (), 0), // +0x4c7b0 ?peek@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e60930, fn_7001e60930 as *const (), 0), // +0x4c930 ?peek@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e60ac0, fn_7001e60ac0 as *const (), 0), // +0x4cac0 ?peek@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e60c50, fn_7001e60c50 as *const (), 0), // +0x4cc50 ?pubimbue@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA?AVlocale@2@AEBV32@@Z
    Lifted::new(0x7001e60d10, fn_7001e60d10 as *const (), 0), // +0x4cd10 ?pubseekoff@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@_JHH@Z
    Lifted::new(0x7001e60d40, fn_7001e60d40 as *const (), 2), // +0x4cd40 ?pubseekpos@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@V32@H@Z
    Lifted::new(0x7001e60d80, fn_7001e60d80 as *const (), 0), // +0x4cd80 ?put@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@D@Z
    Lifted::new(0x7001e60f10, fn_7001e60f10 as *const (), 0), // +0x4cf10 ?put@?$basic_ostream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@G@Z
    Lifted::new(0x7001e610a0, tpl_121::<0x48, 0x7001e696a0> as *const (), 2), // +0x4d0a0 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e610f0, tpl_121::<0x40, 0x7001e696a0> as *const (), 2), // +0x4d0f0 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e61140, tpl_122::<0x28, 0x7001e696a0> as *const (), 2), // +0x4d140 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e61190, tpl_122::<0x20, 0x7001e696a0> as *const (), 2), // +0x4d190 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e611e0, tpl_123::<0x18, 0x7001e696a0> as *const (), 2), // +0x4d1e0 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e61230, tpl_123::<0x38, 0x7001e696a0> as *const (), 2), // +0x4d230 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e61280, tpl_123::<0x30, 0x7001e696a0> as *const (), 2), // +0x4d280 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e612d0, fn_7001e612d0 as *const (), 2), // +0x4d2d0 ?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D
    Lifted::new(0x7001e61320, tpl_124::<0x48, 0x7001e696a0> as *const (), 2), // +0x4d320 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e61370, tpl_124::<0x40, 0x7001e696a0> as *const (), 2), // +0x4d370 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e613c0, tpl_125::<0x28, 0x7001e696a0> as *const (), 2), // +0x4d3c0 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e61410, tpl_125::<0x20, 0x7001e696a0> as *const (), 2), // +0x4d410 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e61460, tpl_126::<0x18, 0x7001e696a0> as *const (), 2), // +0x4d460 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e614b0, tpl_126::<0x38, 0x7001e696a0> as *const (), 2), // +0x4d4b0 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e61500, tpl_126::<0x30, 0x7001e696a0> as *const (), 2), // +0x4d500 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e61550, fn_7001e61550 as *const (), 2), // +0x4d550 ?put@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@G
    Lifted::new(0x7001e615a0, fn_7001e615a0 as *const (), 2), // +0x4d5a0 ?put@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@
    Lifted::new(0x7001e61600, fn_7001e61600 as *const (), 2), // +0x4d600 ?put@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@
    Lifted::new(0x7001e617e0, fn_7001e617e0 as *const (), 2), // +0x4d7e0 ?put@?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@
    Lifted::new(0x7001e61850, fn_7001e61850 as *const (), 2), // +0x4d850 ?put@?$time_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@GU?$char_traits@
    Lifted::new(0x7001e61b90, fn_7001e61b90 as *const (), 2), // +0x4db90 ?put@?$time_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@_WU?$char_tra
    Lifted::new(0x7001e61ed0, fn_7001e61ed0 as *const (), 0), // +0x4ded0 ?putback@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@D@Z
    Lifted::new(0x7001e620e0, fn_7001e620e0 as *const (), 0), // +0x4e0e0 ?putback@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@G@Z
    Lifted::new(0x7001e62300, fn_7001e62300 as *const (), 0), // +0x4e300 ?putback@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@_W@Z
    Lifted::new(0x7001e62520, fn_7001e62520 as *const (), 0), // +0x4e520 ?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@PEAV32@@Z
    Lifted::new(0x7001e62580, fn_7001e62580 as *const (), 0), // +0x4e580 ?read@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_J@Z
    Lifted::new(0x7001e626e0, fn_7001e626e0 as *const (), 0), // +0x4e6e0 ?read@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@PEAG_J@Z
    Lifted::new(0x7001e62840, fn_7001e62840 as *const (), 0), // +0x4e840 ?read@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@PEA_W_J@Z
    Lifted::new(0x7001e629a0, fn_7001e629a0 as *const (), 0), // +0x4e9a0 ?readsome@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA_JPEAD_J@Z
    Lifted::new(0x7001e62b20, fn_7001e62b20 as *const (), 0), // +0x4eb20 ?readsome@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA_JPEAG_J@Z
    Lifted::new(0x7001e62ca0, fn_7001e62ca0 as *const (), 0), // +0x4eca0 ?readsome@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA_JPEA_W_J@Z
    Lifted::new(0x7001e62e20, fn_7001e62e20 as *const (), 2), // +0x4ee20 ?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@V?$fpos@U_Mbstatet@@@2@@Z
    Lifted::new(0x7001e63030, fn_7001e63030 as *const (), 0), // +0x4f030 ?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@_JH@Z
    Lifted::new(0x7001e63240, fn_7001e63240 as *const (), 2), // +0x4f240 ?seekg@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@V?$fpos@U_Mbstatet@@@2@@Z
    Lifted::new(0x7001e63450, fn_7001e63450 as *const (), 0), // +0x4f450 ?seekg@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@_JH@Z
    Lifted::new(0x7001e63660, fn_7001e63660 as *const (), 2), // +0x4f660 ?seekg@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@V?$fpos@U_Mbstatet@@@2@@Z
    Lifted::new(0x7001e63870, fn_7001e63870 as *const (), 0), // +0x4f870 ?seekg@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@_JH@Z
    Lifted::new(0x7001e63a80, fn_7001e63a80 as *const (), 2), // +0x4fa80 ?seekp@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@V?$fpos@U_Mbstatet@@@2@@Z
    Lifted::new(0x7001e63c00, fn_7001e63c00 as *const (), 0), // +0x4fc00 ?seekp@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@_JH@Z
    Lifted::new(0x7001e63d70, tpl_59::<0x48> as *const (), 1), // +0x4fd70 ?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z
    Lifted::new(0x7001e63d80, tpl_56::<0x20, 0x40, 0x58> as *const (), 1), // +0x4fd80 ?setp@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAD00@Z
    Lifted::new(0x7001e63da0, tpl_75::<0x20, 0x40, 0x58> as *const (), 1), // +0x4fda0 ?setp@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAXPEAG00@Z
    Lifted::new(0x7001e63dc0, fn_7001e63dc0 as *const (), 0), // +0x4fdc0 ?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXI@Z
    Lifted::new(0x7001e63e50, fn_7001e63e50 as *const (), 0), // +0x4fe50 ?snextc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e63f00, fn_7001e63f00 as *const (), 0), // +0x4ff00 ?snextc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e63fa0, fn_7001e63fa0 as *const (), 0), // +0x4ffa0 ?sputbackc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z
    Lifted::new(0x7001e63fe0, fn_7001e63fe0 as *const (), 0), // +0x4ffe0 ?sputbackc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAGG@Z
    Lifted::new(0x7001e64030, fn_7001e64030 as *const (), 0), // +0x50030 ?stossc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e64060, fn_7001e64060 as *const (), 0), // +0x50060 ?stossc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAXXZ
    Lifted::new(0x7001e64090, fn_7001e64090 as *const (), 0), // +0x50090 ?sungetc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e640d0, fn_7001e640d0 as *const (), 0), // +0x500d0 ?sungetc@?$basic_streambuf@GU?$char_traits@G@std@@@std@@QEAAGXZ
    Lifted::new(0x7001e64110, fn_7001e64110 as *const (), 0), // +0x50110 ?swap@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXAEAV12@@Z
    Lifted::new(0x7001e64150, fn_7001e64150 as *const (), 0), // +0x50150 ?swap@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXAEAV12@@Z
    Lifted::new(0x7001e641a0, fn_7001e641a0 as *const (), 0), // +0x501a0 ?swap@?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z
    Lifted::new(0x7001e64200, fn_7001e64200 as *const (), 0), // +0x50200 ?swap@?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAAXAEAV12@@Z
    Lifted::new(0x7001e64260, fn_7001e64260 as *const (), 0), // +0x50260 ?swap@?$basic_istream@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z
    Lifted::new(0x7001e642e0, fn_7001e642e0 as *const (), 0), // +0x502e0 ?swap@?$basic_istream@GU?$char_traits@G@std@@@std@@IEAAXAEAV12@@Z
    Lifted::new(0x7001e64360, fn_7001e64360 as *const (), 1), // +0x50360 ?swap@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z
    Lifted::new(0x7001e64470, fn_7001e64470 as *const (), 1), // +0x50470 ?swap@?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAXAEAV12@@Z
    Lifted::new(0x7001e64570, fn_7001e64570 as *const (), 0), // +0x50570 ?sync@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e646b0, fn_7001e646b0 as *const (), 0), // +0x506b0 ?sync@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e647f0, fn_7001e647f0 as *const (), 0), // +0x507f0 ?sync@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAHXZ
    Lifted::new(0x7001e64930, fn_7001e64930 as *const (), 0), // +0x50930 ?tellg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ
    Lifted::new(0x7001e64a20, fn_7001e64a20 as *const (), 0), // +0x50a20 ?tellg@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ
    Lifted::new(0x7001e64b10, fn_7001e64b10 as *const (), 0), // +0x50b10 ?tellg@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ
    Lifted::new(0x7001e64c00, fn_7001e64c00 as *const (), 0), // +0x50c00 ?tellp@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ
    Lifted::new(0x7001e64d00, fn_7001e64d00 as *const (), 0), // +0x50d00 ?unget@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ
    Lifted::new(0x7001e64f00, fn_7001e64f00 as *const (), 0), // +0x50f00 ?unget@?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV12@XZ
    Lifted::new(0x7001e65110, fn_7001e65110 as *const (), 0), // +0x51110 ?unget@?$basic_istream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@XZ
    Lifted::new(0x7001e65320, fn_7001e65320 as *const (), 0), // +0x51320 ?write@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEBD_J@Z
    Lifted::new(0x7001e65484, fn_7001e65484 as *const (), 0), // +0x51484 sub_51484
    Lifted::new(0x7001e654c0, fn_7001e654c0 as *const (), 0), // +0x514c0 sub_514c0
    Lifted::new(0x7001e654f4, fn_7001e654f4 as *const (), 0), // +0x514f4 sub_514f4
    Lifted::new(0x7001e6550c, fn_7001e6550c as *const (), 0), // +0x5150c sub_5150c
    Lifted::new(0x7001e65534, fn_7001e65534 as *const (), 0), // +0x51534 sub_51534
    Lifted::new(0x7001e6554c, fn_7001e6554c as *const (), 0), // +0x5154c sub_5154c
    Lifted::new(0x7001e655ac, fn_7001e655ac as *const (), 0), // +0x515ac sub_515ac
    Lifted::new(0x7001e655dc, fn_7001e655dc as *const (), 0), // +0x515dc sub_515dc
    Lifted::new(0x7001e655f0, fn_7001e655f0 as *const (), 0), // +0x515f0 sub_515f0
    Lifted::new(0x7001e6562c, fn_7001e6562c as *const (), 2), // +0x5162c sub_5162c
    Lifted::new(0x7001e656b8, fn_7001e656b8 as *const (), 1), // +0x516b8 sub_516b8
    Lifted::new(0x7001e65750, fn_7001e65750 as *const (), 0), // +0x51750 sub_51750
    Lifted::new(0x7001e65774, fn_7001e65774 as *const (), 0), // +0x51774 sub_51774
    Lifted::new(0x7001e657a0, fn_7001e657a0 as *const (), 0), // +0x517a0 sub_517a0
    Lifted::new(0x7001e657dc, fn_7001e657dc as *const (), 0), // +0x517dc sub_517dc
    Lifted::new(0x7001e657f4, fn_7001e657f4 as *const (), 0), // +0x517f4 sub_517f4
    Lifted::new(0x7001e65814, fn_7001e65814 as *const (), 0), // +0x51814 sub_51814
    Lifted::new(0x7001e65880, fn_7001e65880 as *const (), 1), // +0x51880 sub_51880
    Lifted::new(0x7001e658e0, fn_7001e658e0 as *const (), 0), // +0x518e0 sub_518e0
    Lifted::new(0x7001e65900, fn_7001e65900 as *const (), 0), // +0x51900 sub_51900
    Lifted::new(0x7001e6592c, fn_7001e6592c as *const (), 0), // +0x5192c sub_5192c
    Lifted::new(0x7001e65998, fn_7001e65998 as *const (), 0), // +0x51998 sub_51998
    Lifted::new(0x7001e659fc, fn_7001e659fc as *const (), 0), // +0x519fc sub_519fc
    Lifted::new(0x7001e65a40, fn_7001e65a40 as *const (), 0), // +0x51a40 sub_51a40
    Lifted::new(0x7001e65ac0, fn_7001e65ac0 as *const (), 0), // +0x51ac0 sub_51ac0
    Lifted::new(0x7001e65b10, fn_7001e65b10 as *const (), 0), // +0x51b10 sub_51b10
    Lifted::new(0x7001e65c28, fn_7001e65c28 as *const (), 0), // +0x51c28 sub_51c28
    Lifted::new(0x7001e65ca8, fn_7001e65ca8 as *const (), 0), // +0x51ca8 sub_51ca8
    Lifted::new(0x7001e65de0, fn_7001e65de0 as *const (), 0), // +0x51de0 sub_51de0
    Lifted::new(0x7001e65e28, fn_7001e65e28 as *const (), 1), // +0x51e28 sub_51e28
    Lifted::new(0x7001e65fd4, fn_7001e65fd4 as *const (), 1), // +0x51fd4 sub_51fd4
    Lifted::new(0x7001e66130, fn_7001e66130 as *const (), 0), // +0x52130 sub_52130
    Lifted::new(0x7001e66170, fn_7001e66170 as *const (), 0), // +0x52170 sub_52170
    Lifted::new(0x7001e662c0, fn_7001e662c0 as *const (), 0), // +0x522c0 sub_522c0
    Lifted::new(0x7001e6636c, tpl_16::<0x7001e9a7d0, 0x7001e69028> as *const (), 0), // +0x5236c sub_5236c
    Lifted::new(0x7001e6637c, tpl_7::<0x7001e9a7d0, 0x7001e66496> as *const (), 0), // +0x5237c sub_5237c
    Lifted::new(0x7001e66388, tpl_34::<0x7001e9a7e0> as *const (), 1), // +0x52388 sub_52388
    Lifted::new(0x7001e66390, fn_7001e66390 as *const (), 0), // +0x52390 sub_52390
    Lifted::new(0x7001e663ac, tpl_34::<0x7001e9a7e8> as *const (), 1), // +0x523ac sub_523ac
    Lifted::new(0x7001e663b4, tpl_127::<0x7001e72dc0, 0x7001e696a0> as *const (), 0), // +0x523b4 sub_523b4
    Lifted::new(0x7001e663f0, tpl_127::<0x7001e72dd0, 0x7001e696a0> as *const (), 0), // +0x523f0 sub_523f0
    Lifted::new(0x7001e66530, fn_7001e66530 as *const (), 2), // +0x52530 sub_52530
    Lifted::new(0x7001e66550, tpl_15::<0x7001e696a0> as *const (), 2), // +0x52550 sub_52550
    Lifted::new(0x7001e665bd, fn_7001e665bd as *const (), 0), // +0x525bd sub_525bd
    Lifted::new(0x7001e665e3, fn_7001e665e3 as *const (), 0), // +0x525e3 sub_525e3
    Lifted::new(0x7001e66609, fn_7001e66609 as *const (), 0), // +0x52609 sub_52609
    Lifted::new(0x7001e6662f, fn_7001e6662f as *const (), 0), // +0x5262f sub_5262f
    Lifted::new(0x7001e66655, fn_7001e66655 as *const (), 2), // +0x52655 sub_52655
    Lifted::new(0x7001e6670f, fn_7001e6670f as *const (), 2), // +0x5270f sub_5270f
    Lifted::new(0x7001e66a25, fn_7001e66a25 as *const (), 0), // +0x52a25 sub_52a25
    Lifted::new(0x7001e66a63, fn_7001e66a63 as *const (), 0), // +0x52a63 sub_52a63
    Lifted::new(0x7001e66b35, fn_7001e66b35 as *const (), 0), // +0x52b35 sub_52b35
    Lifted::new(0x7001e66b55, fn_7001e66b55 as *const (), 0), // +0x52b55 sub_52b55
    Lifted::new(0x7001e66b75, fn_7001e66b75 as *const (), 0), // +0x52b75 sub_52b75
    Lifted::new(0x7001e66b95, fn_7001e66b95 as *const (), 0), // +0x52b95 sub_52b95
    Lifted::new(0x7001e66bb5, fn_7001e66bb5 as *const (), 0), // +0x52bb5 sub_52bb5
    Lifted::new(0x7001e66be1, fn_7001e66be1 as *const (), 0), // +0x52be1 sub_52be1
    Lifted::new(0x7001e66c1f, fn_7001e66c1f as *const (), 0), // +0x52c1f sub_52c1f
    Lifted::new(0x7001e66c6d, fn_7001e66c6d as *const (), 1), // +0x52c6d sub_52c6d
    Lifted::new(0x7001e66c97, fn_7001e66c97 as *const (), 0), // +0x52c97 sub_52c97
    Lifted::new(0x7001e66d3c, fn_7001e66d3c as *const (), 0), // +0x52d3c sub_52d3c
    Lifted::new(0x7001e66dbc, fn_7001e66dbc as *const (), 0), // +0x52dbc sub_52dbc
    Lifted::new(0x7001e66e0c, fn_7001e66e0c as *const (), 0), // +0x52e0c sub_52e0c
    Lifted::new(0x7001e66e32, fn_7001e66e32 as *const (), 0), // +0x52e32 sub_52e32
    Lifted::new(0x7001e66e8b, fn_7001e66e8b as *const (), 0), // +0x52e8b sub_52e8b
    Lifted::new(0x7001e66ea8, fn_7001e66ea8 as *const (), 0), // +0x52ea8 sub_52ea8
    Lifted::new(0x7001e66ed1, fn_7001e66ed1 as *const (), 0), // +0x52ed1 sub_52ed1
    Lifted::new(0x7001e66ef1, fn_7001e66ef1 as *const (), 0), // +0x52ef1 sub_52ef1
    Lifted::new(0x7001e66f1d, fn_7001e66f1d as *const (), 0), // +0x52f1d sub_52f1d
    Lifted::new(0x7001e66f55, fn_7001e66f55 as *const (), 0), // +0x52f55 sub_52f55
    Lifted::new(0x7001e66f8a, fn_7001e66f8a as *const (), 0), // +0x52f8a sub_52f8a
    Lifted::new(0x7001e66fb3, tpl_128::<0x7001e1de14> as *const (), 0), // +0x52fb3 sub_52fb3
    Lifted::new(0x7001e66fdc, fn_7001e66fdc as *const (), 0), // +0x52fdc sub_52fdc
    Lifted::new(0x7001e67005, tpl_129::<0x7001e1de14> as *const (), 0), // +0x53005 sub_53005
    Lifted::new(0x7001e6702e, tpl_130::<0x7001e1de14> as *const (), 0), // +0x5302e sub_5302e
    Lifted::new(0x7001e6709f, fn_7001e6709f as *const (), 0), // +0x5309f sub_5309f
    Lifted::new(0x7001e670bc, fn_7001e670bc as *const (), 0), // +0x530bc sub_530bc
    Lifted::new(0x7001e670d9, fn_7001e670d9 as *const (), 0), // +0x530d9 sub_530d9
    Lifted::new(0x7001e6717a, fn_7001e6717a as *const (), 0), // +0x5317a sub_5317a
    Lifted::new(0x7001e671c4, fn_7001e671c4 as *const (), 0), // +0x531c4 sub_531c4
    Lifted::new(0x7001e671ed, tpl_128::<0x7001e18bfc> as *const (), 0), // +0x531ed sub_531ed
    Lifted::new(0x7001e67216, fn_7001e67216 as *const (), 0), // +0x53216 sub_53216
    Lifted::new(0x7001e6723f, tpl_129::<0x7001e18bfc> as *const (), 0), // +0x5323f sub_5323f
    Lifted::new(0x7001e67268, tpl_130::<0x7001e18bfc> as *const (), 0), // +0x53268 sub_53268
    Lifted::new(0x7001e672e5, fn_7001e672e5 as *const (), 0), // +0x532e5 sub_532e5
    Lifted::new(0x7001e6733b, fn_7001e6733b as *const (), 0), // +0x5333b sub_5333b
    Lifted::new(0x7001e673a4, fn_7001e673a4 as *const (), 0), // +0x533a4 sub_533a4
    Lifted::new(0x7001e673ee, fn_7001e673ee as *const (), 0), // +0x533ee sub_533ee
    Lifted::new(0x7001e67488, fn_7001e67488 as *const (), 0), // +0x53488 sub_53488
    Lifted::new(0x7001e674b2, fn_7001e674b2 as *const (), 0), // +0x534b2 sub_534b2
    Lifted::new(0x7001e674dc, fn_7001e674dc as *const (), 0), // +0x534dc sub_534dc
    Lifted::new(0x7001e67506, fn_7001e67506 as *const (), 0), // +0x53506 sub_53506
    Lifted::new(0x7001e6757c, fn_7001e6757c as *const (), 1), // +0x5357c sub_5357c
    Lifted::new(0x7001e6759a, fn_7001e6759a as *const (), 0), // +0x5359a sub_5359a
    Lifted::new(0x7001e67603, fn_7001e67603 as *const (), 0), // +0x53603 sub_53603
    Lifted::new(0x7001e6766c, fn_7001e6766c as *const (), 0), // +0x5366c sub_5366c
    Lifted::new(0x7001e676a0, fn_7001e676a0 as *const (), 0), // +0x536a0 sub_536a0
    Lifted::new(0x7001e676fd, fn_7001e676fd as *const (), 0), // +0x536fd sub_536fd
    Lifted::new(0x7001e67761, fn_7001e67761 as *const (), 0), // +0x53761 sub_53761
    Lifted::new(0x7001e677d6, fn_7001e677d6 as *const (), 0), // +0x537d6 sub_537d6
    Lifted::new(0x7001e677fc, fn_7001e677fc as *const (), 0), // +0x537fc sub_537fc
    Lifted::new(0x7001e67859, fn_7001e67859 as *const (), 0), // +0x53859 sub_53859
    Lifted::new(0x7001e678c2, fn_7001e678c2 as *const (), 0), // +0x538c2 sub_538c2
    Lifted::new(0x7001e6792b, fn_7001e6792b as *const (), 0), // +0x5392b sub_5392b
    Lifted::new(0x7001e67985, fn_7001e67985 as *const (), 1), // +0x53985 sub_53985
    Lifted::new(0x7001e6799d, fn_7001e6799d as *const (), 0), // +0x5399d sub_5399d
    Lifted::new(0x7001e679c9, fn_7001e679c9 as *const (), 0), // +0x539c9 sub_539c9
    Lifted::new(0x7001e67a28, fn_7001e67a28 as *const (), 0), // +0x53a28 sub_53a28
    Lifted::new(0x7001e67a3f, fn_7001e67a3f as *const (), 0), // +0x53a3f sub_53a3f
    Lifted::new(0x7001e67a58, fn_7001e67a58 as *const (), 0), // +0x53a58 sub_53a58
    Lifted::new(0x7001e67a6c, fn_7001e67a6c as *const (), 0), // +0x53a6c sub_53a6c
    Lifted::new(0x7001e67ab0, tpl_11 as *const (), 1), // +0x53ab0 sub_53ab0
    Lifted::new(0x7001e67ac0, tpl_11 as *const (), 1), // +0x53ac0 sub_53ac0
    Lifted::new(0x7001e67ad0, fn_7001e67ad0 as *const (), 0), // +0x53ad0 sub_53ad0
    Lifted::new(0x7001e67b30, tpl_7::<0x7001e98c20, 0x7001e18a94> as *const (), 0), // +0x53b30 sub_53b30
    Lifted::new(0x7001e67b40, tpl_7::<0x7001e98ba0, 0x7001e49e80> as *const (), 0), // +0x53b40 sub_53b40
    Lifted::new(0x7001e67b50, fn_7001e67b50 as *const (), 0), // +0x53b50 sub_53b50
    Lifted::new(0x7001e67bb0, tpl_7::<0x7001e98cf0, 0x7001e18a94> as *const (), 0), // +0x53bb0 sub_53bb0
    Lifted::new(0x7001e67bc0, tpl_7::<0x7001e98ce0, 0x7001e49e80> as *const (), 0), // +0x53bc0 sub_53bc0
    Lifted::new(0x7001e67bd0, fn_7001e67bd0 as *const (), 0), // +0x53bd0 sub_53bd0
    Lifted::new(0x7001e67c30, tpl_7::<0x7001e98e20, 0x7001e18a94> as *const (), 0), // +0x53c30 sub_53c30
    Lifted::new(0x7001e67c40, tpl_7::<0x7001e98e10, 0x7001e49e80> as *const (), 0), // +0x53c40 sub_53c40
    Lifted::new(0x7001e67c50, fn_7001e67c50 as *const (), 0), // +0x53c50 sub_53c50
    Lifted::new(0x7001e67cb0, tpl_7::<0x7001e98fb0, 0x7001e18a94> as *const (), 0), // +0x53cb0 sub_53cb0
    Lifted::new(0x7001e67cc0, tpl_7::<0x7001e98f30, 0x7001e49e80> as *const (), 0), // +0x53cc0 sub_53cc0
    Lifted::new(0x7001e67cd0, fn_7001e67cd0 as *const (), 0), // +0x53cd0 sub_53cd0
    Lifted::new(0x7001e67d10, tpl_7::<0x7001e99100, 0x7001e49e80> as *const (), 0), // +0x53d10 sub_53d10
    Lifted::new(0x7001e67d20, fn_7001e67d20 as *const (), 0), // +0x53d20 sub_53d20
    Lifted::new(0x7001e67d80, fn_7001e67d80 as *const (), 0), // +0x53d80 sub_53d80
    Lifted::new(0x7001e67dc0, tpl_7::<0x7001e99214, 0x7001e49e80> as *const (), 0), // +0x53dc0 sub_53dc0
    Lifted::new(0x7001e67dd0, tpl_7::<0x7001e99560, 0x7001e49e80> as *const (), 0), // +0x53dd0 sub_53dd0
    Lifted::new(0x7001e67de0, fn_7001e67de0 as *const (), 0), // +0x53de0 sub_53de0
    Lifted::new(0x7001e67e40, tpl_7::<0x7001e995e0, 0x7001e27428> as *const (), 0), // +0x53e40 sub_53e40
    Lifted::new(0x7001e67e50, tpl_7::<0x7001e99690, 0x7001e49e80> as *const (), 0), // +0x53e50 sub_53e50
    Lifted::new(0x7001e67e60, fn_7001e67e60 as *const (), 0), // +0x53e60 sub_53e60
    Lifted::new(0x7001e67ec0, tpl_7::<0x7001e996a0, 0x7001e27428> as *const (), 0), // +0x53ec0 sub_53ec0
    Lifted::new(0x7001e67ed0, tpl_7::<0x7001e997c0, 0x7001e49e80> as *const (), 0), // +0x53ed0 sub_53ed0
    Lifted::new(0x7001e67ee0, fn_7001e67ee0 as *const (), 0), // +0x53ee0 sub_53ee0
    Lifted::new(0x7001e67f40, tpl_7::<0x7001e99840, 0x7001e27428> as *const (), 0), // +0x53f40 sub_53f40
    Lifted::new(0x7001e67f50, tpl_7::<0x7001e998e0, 0x7001e49e80> as *const (), 0), // +0x53f50 sub_53f50
    Lifted::new(0x7001e67f60, fn_7001e67f60 as *const (), 0), // +0x53f60 sub_53f60
    Lifted::new(0x7001e67fc0, tpl_7::<0x7001e99960, 0x7001e27428> as *const (), 0), // +0x53fc0 sub_53fc0
    Lifted::new(0x7001e67fd0, tpl_7::<0x7001e99a20, 0x7001e49e80> as *const (), 0), // +0x53fd0 sub_53fd0
    Lifted::new(0x7001e67fe0, fn_7001e67fe0 as *const (), 0), // +0x53fe0 sub_53fe0
    Lifted::new(0x7001e68040, tpl_7::<0x7001e99aa0, 0x7001e28b38> as *const (), 0), // +0x54040 sub_54040
    Lifted::new(0x7001e68050, tpl_7::<0x7001e99b50, 0x7001e49e80> as *const (), 0), // +0x54050 sub_54050
    Lifted::new(0x7001e68060, fn_7001e68060 as *const (), 0), // +0x54060 sub_54060
    Lifted::new(0x7001e680c0, tpl_7::<0x7001e99b60, 0x7001e28b38> as *const (), 0), // +0x540c0 sub_540c0
    Lifted::new(0x7001e680d0, tpl_7::<0x7001e99c80, 0x7001e49e80> as *const (), 0), // +0x540d0 sub_540d0
    Lifted::new(0x7001e680e0, fn_7001e680e0 as *const (), 0), // +0x540e0 sub_540e0
    Lifted::new(0x7001e68140, tpl_7::<0x7001e99d00, 0x7001e28b38> as *const (), 0), // +0x54140 sub_54140
    Lifted::new(0x7001e68150, tpl_7::<0x7001e99da0, 0x7001e49e80> as *const (), 0), // +0x54150 sub_54150
    Lifted::new(0x7001e68160, fn_7001e68160 as *const (), 0), // +0x54160 sub_54160
    Lifted::new(0x7001e681c0, tpl_7::<0x7001e99e20, 0x7001e28b38> as *const (), 0), // +0x541c0 sub_541c0
    Lifted::new(0x7001e681d0, tpl_7::<0x7001e9a0a0, 0x7001e49e80> as *const (), 0), // +0x541d0 sub_541d0
];
