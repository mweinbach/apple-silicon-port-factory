#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::alias(0x7001dc5010, fn_7001dd4770 as *const (), 3, 0x10770), // +0x1010 ?expm1@ieee754@base@v8@@YANN@Z (thunk -> +0x10770)
    Lifted::alias(0x7001dc5020, tpl_10 as *const (), 1, 0x25ad0), // +0x1020 sub_1020 (thunk -> +0x25ad0)
    Lifted::alias(0x7001dc5030, tpl_29::<0x7001e070b8, 0x7001dc5b50, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd8b51> as *const (), 2, 0x14840), // +0x1030 ??$MakeCheckOpString@_K_K@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@_K0PEBD@Z (thunk -> +0x14840)
    Lifted::alias(0x7001dc5040, tpl_35::<0x7001e074d8> as *const (), 2, 0x1e330), // +0x1040 ?Strdup@base@v8@@YAPEADPEBD@Z (thunk -> +0x1e330)
    Lifted::new(0x7001dc5050, tpl_1::<0x7001de9b30> as *const (), 2), // +0x1050 ?CreateThreadLocalKey@Thread@base@v8@@SAHXZ
    Lifted::new(0x7001dc5060, tpl_1::<0x7001de3090> as *const (), 2), // +0x1060 ?Lock@Mutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc5070, tpl_51 as *const (), 1, 0x1e7f0), // +0x1070 ??O?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NVThreadTicks@23@@Z (thunk -> +0x1e7f0)
    Lifted::alias(0x7001dc5080, tpl_57 as *const (), 1, 0x1ece0), // +0x1080 ?IsMin@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ece0)
    Lifted::alias(0x7001dc5090, tpl_17::<0x54> as *const (), 1, 0xb0c0), // +0x1090 ?has_neon@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb0c0)
    Lifted::alias(0x7001dc50b0, fn_7001de2220 as *const (), 0, 0x1e220), // +0x10b0 ?FreePages@PageAllocator@base@v8@@UEAA_NPEAX_K@Z (thunk -> +0x1e220)
    Lifted::alias(0x7001dc50c0, fn_7001deee50 as *const (), 0, 0x2ae50), // +0x10c0 sub_10c0 (thunk -> +0x2ae50)
    Lifted::alias(0x7001dc50d0, fn_7001de8930 as *const (), 2, 0x24930), // +0x10d0 sub_10d0 (thunk -> +0x24930)
    Lifted::alias(0x7001dc50e0, tpl_15 as *const (), 1, 0xcd20), // +0x10e0 ?native_handle@Semaphore@base@v8@@QEBAAEBQEAXXZ (thunk -> +0xcd20)
    Lifted::alias(0x7001dc5100, fn_7001dd4420 as *const (), 3, 0x10420), // +0x1100 ?exp@ieee754@base@v8@@YANN@Z (thunk -> +0x10420)
    Lifted::alias(0x7001dc5110, tpl_45 as *const (), 1, 0x1e570), // +0x1110 ??9?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NVThreadTicks@23@@Z (thunk -> +0x1e570)
    Lifted::alias(0x7001dc5120, tpl_6::<0x7001dd9550> as *const (), 0, 0x154f0), // +0x1120 ??$PrintCheckOperand@PEAC@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAC@Z (thunk -> +0x154f0)
    Lifted::alias(0x7001dc5130, fn_7001de9e70 as *const (), 0, 0x25e70), // +0x1130 ?FPrint@OS@base@v8@@SAXPEAU_iobuf@@PEBDZZ (thunk -> +0x25e70)
    Lifted::alias(0x7001dc5140, tpl_49 as *const (), 1, 0x1e790), // +0x1140 ??M?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NVTimeTicks@23@@Z (thunk -> +0x1e790)
    Lifted::alias(0x7001dc5150, tpl_25 as *const (), 1, 0x1eac0), // +0x1150 ?FromMicroseconds@TimeDelta@base@v8@@SA?AV123@_J@Z (thunk -> +0x1eac0)
    Lifted::alias(0x7001dc5160, tpl_16::<0x38> as *const (), 1, 0xb1b0), // +0x1160 ?icache_line_size@CPU@base@v8@@QEBAHXZ (thunk -> +0xb1b0)
    Lifted::alias(0x7001dc5170, tpl_48::<0x7001dc6500> as *const (), 0, 0x1e6d0), // +0x1170 ??H?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA?AVTime@23@VTimeDelta@23@@Z (thunk -> +0x1e6d0)
    Lifted::alias(0x7001dc5180, fn_7001de2200 as *const (), 0, 0x1e200), // +0x1180 ?DiscardSystemPages@PageAllocator@base@v8@@UEAA_NPEAX_K@Z (thunk -> +0x1e200)
    Lifted::alias(0x7001dc5190, tpl_15 as *const (), 1, 0xa9c0), // +0x1190 ?native_handle@Mutex@base@v8@@QEBAAEBUV8_SRWLOCK@@XZ (thunk -> +0xa9c0)
    Lifted::alias(0x7001dc51a0, tpl_68::<0x7001e07678> as *const (), 1, 0x287c0), // +0x11a0 sub_11a0 (thunk -> +0x287c0)
    Lifted::alias(0x7001dc51b0, tpl_17::<0x48> as *const (), 1, 0xb130), // +0x11b0 ?has_sse41@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb130)
    Lifted::alias(0x7001dc51c0, fn_7001ddd180 as *const (), 0, 0x19180), // +0x11c0 ?AssignHexString@Bignum@base@v8@@QEAAXV?$Vector@$$CBD@23@@Z (thunk -> +0x19180)
    Lifted::alias(0x7001dc51d0, tpl_22::<0x7001df78b8, 0x7001deddc0, 0x7001de8a30> as *const (), 0, 0x24a20), // +0x11d0 sub_11d0 (thunk -> +0x24a20)
    Lifted::alias(0x7001dc51e0, tpl_58 as *const (), 1, 0x1ed70), // +0x11e0 ?IsNull@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ed70)
    Lifted::alias(0x7001dc51f0, fn_7001dce980 as *const (), 1, 0xa980), // +0x11f0 ?end@RegionAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa980)
    Lifted::alias(0x7001dc5200, tpl_6::<0x7001dc7b40> as *const (), 0, 0x15770), // +0x1200 ??$PrintCheckOperand@PEBC@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBC@Z (thunk -> +0x15770)
    Lifted::alias(0x7001dc5210, fn_7001ddf7d0 as *const (), 2, 0x1b7d0), // +0x1210 ?FastDtoa@base@v8@@YA_NNW4FastDtoaMode@12@HV?$Vector@D@12@PEAH2@Z (thunk -> +0x1b7d0)
    Lifted::alias(0x7001dc5220, fn_7001def630 as *const (), 1, 0x2b630), // +0x1220 sub_1220 (thunk -> +0x2b630)
    Lifted::alias(0x7001dc5230, tpl_39::<0x7001e074f0, 0x7001de2017, 0x7001de2018, 0x7001de2019, 0x7001de201a, 0x7001de201b, 0x7001de201c, 0x7001de201d, 0x7001de201e, 0x7001de201f> as *const (), 0, 0x1e010), // +0x1230 sub_1230 (thunk -> +0x1e010)
    Lifted::alias(0x7001dc5240, tpl_6::<0x7001dc6eb0> as *const (), 0, 0xa120), // +0x1240 ??0RandomNumberGenerator@base@v8@@QEAA@_J@Z (thunk -> +0xa120)
    Lifted::alias(0x7001dc5250, fn_7001dce8a0 as *const (), 3, 0xa8a0), // +0x1250 ?ToDouble@RandomNumberGenerator@base@v8@@SAN_K@Z (thunk -> +0xa8a0)
    Lifted::alias(0x7001dc5260, fn_7001deba50 as *const (), 0, 0x27a50), // +0x1260 ?PrintError@OS@base@v8@@SAXPEBDZZ (thunk -> +0x27a50)
    Lifted::alias(0x7001dc5270, fn_7001de66e0 as *const (), 2, 0x226e0), // +0x1270 ?SNPrintF@base@v8@@YAHV?$Vector@D@12@PEBDZZ (thunk -> +0x226e0)
    Lifted::alias(0x7001dc5280, tpl_46::<0x7001dc5fa0> as *const (), 0, 0x1e600), // +0x1280 ??G?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA?AVTime@23@VTimeDelta@23@@Z (thunk -> +0x1e600)
    Lifted::alias(0x7001dc5290, fn_7001de8b90 as *const (), 1, 0x24b90), // +0x1290 ?VLQBase64Decode@base@v8@@YAHPEBD_KPEA_K@Z (thunk -> +0x24b90)
    Lifted::alias(0x7001dc52a0, fn_7001de9720 as *const (), 0, 0x25720), // +0x12a0 sub_12a0 (thunk -> +0x25720)
    Lifted::alias(0x7001dc52b0, fn_7001dd0c50 as *const (), 0, 0xcc50), // +0x12b0 ?StartSynchronously@Thread@base@v8@@QEAA_NXZ (thunk -> +0xcc50)
    Lifted::alias(0x7001dc52c0, fn_7001de2820 as *const (), 1, 0x1e820), // +0x12c0 ??OTimeDelta@base@v8@@QEBA_NAEBV012@@Z (thunk -> +0x1e820)
    Lifted::alias(0x7001dc52d0, tpl_17::<0x42> as *const (), 1, 0xb100), // +0x12d0 ?has_sahf@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb100)
    Lifted::alias(0x7001dc52e0, fn_7001de6510 as *const (), 0, 0x22510), // +0x12e0 ??0LsanPageAllocator@base@v8@@QEAA@PEAVPageAllocator@2@@Z (thunk -> +0x22510)
    Lifted::alias(0x7001dc5320, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0, 0xb6a0), // +0x1320 sub_1320 (thunk -> +0xb6a0)
    Lifted::alias(0x7001dc5330, fn_7001de8c70 as *const (), 1, 0x24c70), // +0x1330 ?charToDigitDecodeForTesting@base@v8@@YACE@Z (thunk -> +0x24c70)
    Lifted::alias(0x7001dc5340, fn_7001dd0290 as *const (), 1, 0xc290), // +0x1340 ??$SignedDivisionByConstant@I@base@v8@@YA?AU?$MagicNumbersForDivision@I@01@I@Z (thunk -> +0xc290)
    Lifted::new(0x7001dc5350, tpl_1::<0x7001de3150> as *const (), 2), // +0x1350 ?Unlock@Mutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc5360, tpl_38::<0x7001dc5540> as *const (), 0, 0x18560), // +0x1360 ?LessEqual@Bignum@base@v8@@SA_NAEBV123@0@Z (thunk -> +0x18560)
    Lifted::alias(0x7001dc5370, fn_7001dce6d0 as *const (), 0, 0xa6d0), // +0x1370 ?NextInt@RandomNumberGenerator@base@v8@@QEAAHXZ (thunk -> +0xa6d0)
    Lifted::alias(0x7001dc5390, tpl_62::<0x7001e0a0b0> as *const (), 0, 0x1f130), // +0x1390 ?TryLockShared@SharedMutex@base@v8@@QEAA_NXZ (thunk -> +0x1f130)
    Lifted::alias(0x7001dc53a0, fn_7001deeee0 as *const (), 0, 0x2aee0), // +0x13a0 sub_13a0 (thunk -> +0x2aee0)
    Lifted::alias(0x7001dc53b0, tpl_17::<0x45> as *const (), 1, 0xb110), // +0x13b0 ?has_sse2@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb110)
    Lifted::alias(0x7001dc53c0, fn_7001dea8c0 as *const (), 0, 0x268c0), // +0x13c0 sub_13c0 (thunk -> +0x268c0)
    Lifted::alias(0x7001dc53d0, tpl_47 as *const (), 1, 0x1e630), // +0x13d0 ??G?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA?AVTimeDelta@23@VTime@23@@Z (thunk -> +0x1e630)
    Lifted::alias(0x7001dc53e0, fn_7001de9cd0 as *const (), 0, 0x25cd0), // +0x13e0 ?DiscardSystemPages@OS@base@v8@@CA_NPEAX_K@Z (thunk -> +0x25cd0)
    Lifted::alias(0x7001dc53f0, tpl_16::<0x20> as *const (), 1, 0xafd0), // +0x13f0 ?ext_family@CPU@base@v8@@QEBAHXZ (thunk -> +0xafd0)
    Lifted::alias(0x7001dc5400, tpl_35::<0x7001e074e0> as *const (), 2, 0x1e1d0), // +0x1400 ?Calloc@base@v8@@YAPEAX_K0@Z (thunk -> +0x1e1d0)
    Lifted::alias(0x7001dc5410, fn_7001dce140 as *const (), 0, 0xa140), // +0x1410 ??1BoundedPageAllocator@base@v8@@UEAA@XZ (thunk -> +0xa140)
    Lifted::alias(0x7001dc5430, fn_7001de9ea0 as *const (), 0, 0x25ea0), // +0x1430 ?Free@OS@base@v8@@CA_NPEAX_K@Z (thunk -> +0x25ea0)
    Lifted::alias(0x7001dc5440, tpl_28::<0x7001e070b8, 0x7001dc78a0, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd7b2e> as *const (), 2, 0x13820), // +0x1440 ??$MakeCheckOpString@JJ@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@JJPEBD@Z (thunk -> +0x13820)
    Lifted::alias(0x7001dc5450, tpl_22::<0x7001df7620, 0x7001deddc0, 0x7001de64d0> as *const (), 0, 0x224c0), // +0x1450 sub_1450 (thunk -> +0x224c0)
    Lifted::alias(0x7001dc5460, fn_7001dce6e0 as *const (), 0, 0xa6e0), // +0x1460 ?ReleasePages@BoundedPageAllocator@base@v8@@UEAA_NPEAX_K1@Z (thunk -> +0xa6e0)
    Lifted::alias(0x7001dc5470, fn_7001de8420 as *const (), 0, 0x24420), // +0x1470 sub_1470 (thunk -> +0x24420)
    Lifted::alias(0x7001dc5480, tpl_61 as *const (), 1, 0x1eff0), // +0x1480 ?since_origin@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA?AVTimeDelta@34@XZ (thunk -> +0x1eff0)
    Lifted::alias(0x7001dc5490, fn_7001de2f50 as *const (), 0, 0x1ef50), // +0x1490 ?Wait@ConditionVariable@base@v8@@QEAAXPEAVMutex@23@@Z (thunk -> +0x1ef50)
    Lifted::alias(0x7001dc54a0, fn_7001dcdfb0 as *const (), 1, 0x9fb0), // +0x14a0 ?SignedMulOverflow32@bits@base@v8@@YA_NHHPEAH@Z (thunk -> +0x9fb0)
    Lifted::alias(0x7001dc54b0, tpl_42 as *const (), 1, 0x1f020), // +0x14b0 sub_14b0 (thunk -> +0x1f020)
    Lifted::alias(0x7001dc54c0, fn_7001dd6e00 as *const (), 0, 0x12e00), // +0x14c0 sub_14c0 (thunk -> +0x12e00)
    Lifted::alias(0x7001dc54d0, fn_7001de6db0 as *const (), 2, 0x22db0), // +0x14d0 ??0RandomNumberGenerator@base@v8@@QEAA@XZ (thunk -> +0x22db0)
    Lifted::alias(0x7001dc54e0, tpl_62::<0x7001e0a0a8> as *const (), 0, 0x1f110), // +0x14e0 ?TryLockExclusive@SharedMutex@base@v8@@QEAA_NXZ (thunk -> +0x1f110)
    Lifted::alias(0x7001dc54f0, fn_7001dd6300 as *const (), 2, 0x12300), // +0x14f0 ?sin@ieee754@base@v8@@YANN@Z (thunk -> +0x12300)
    Lifted::alias(0x7001dc5500, fn_7001ddb7c0 as *const (), 1, 0x177c0), // +0x1500 ?SetDcheckFunction@base@v8@@YAXP6AXPEBDH0@Z@Z (thunk -> +0x177c0)
    Lifted::alias(0x7001dc5510, fn_7001dd4bf0 as *const (), 2, 0x10bf0), // +0x1510 ?log10@ieee754@base@v8@@YANN@Z (thunk -> +0x10bf0)
    Lifted::alias(0x7001dc5520, tpl_24::<0xfffffffffffffff8, 0x7001de2110, 0x7001de2030> as *const (), 0, 0x22570), // +0x1520 sub_1520 (thunk -> +0x22570)
    Lifted::alias(0x7001dc5530, tpl_12::<0x8, 0x18> as *const (), 0, 0x226d0), // +0x1530 ?SetRandomMmapSeed@LsanPageAllocator@base@v8@@UEAAX_J@Z (thunk -> +0x226d0)
    Lifted::alias(0x7001dc5540, fn_7001ddd920 as *const (), 1, 0x19920), // +0x1540 ?Compare@Bignum@base@v8@@SAHAEBV123@0@Z (thunk -> +0x19920)
    Lifted::alias(0x7001dc5550, tpl_42 as *const (), 1, 0x1ef40), // +0x1550 ?UnixEpoch@Time@base@v8@@SA?AV123@XZ (thunk -> +0x1ef40)
    Lifted::alias(0x7001dc5560, tpl_10 as *const (), 1, 0xb970), // +0x1560 sub_1560 (thunk -> +0xb970)
    Lifted::alias(0x7001dc5580, fn_7001de2f70 as *const (), 0, 0x1ef70), // +0x1580 ?WaitFor@ConditionVariable@base@v8@@QEAA_NPEAVMutex@23@AEBVTimeDelta@23@@Z (thunk -> +0x1ef70)
    Lifted::alias(0x7001dc5590, fn_7001dde230 as *const (), 1, 0x1a230), // +0x1590 ?PlusCompare@Bignum@base@v8@@SAHAEBV123@00@Z (thunk -> +0x1a230)
    Lifted::alias(0x7001dc55a0, fn_7001de6cd0 as *const (), 1, 0x22cd0), // +0x15a0 sub_15a0 (thunk -> +0x22cd0)
    Lifted::alias(0x7001dc55b0, fn_7001de1950 as *const (), 2, 0x1d950), // +0x15b0 ?Strtod@base@v8@@YANV?$Vector@$$CBD@12@H@Z (thunk -> +0x1d950)
    Lifted::alias(0x7001dc55c0, tpl_17::<0x43> as *const (), 1, 0xb0a0), // +0x15c0 ?has_mmx@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb0a0)
    Lifted::alias(0x7001dc55f0, fn_7001de90a0 as *const (), 2, 0x250a0), // +0x15f0 sub_15f0 (thunk -> +0x250a0)
    Lifted::alias(0x7001dc5600, fn_7001deb8b0 as *const (), 0, 0x278b0), // +0x1600 ?OpenTemporaryFile@OS@base@v8@@SAPEAU_iobuf@@XZ (thunk -> +0x278b0)
    Lifted::alias(0x7001dc5610, tpl_52 as *const (), 1, 0x1e850), // +0x1610 ??P?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NVTimeTicks@23@@Z (thunk -> +0x1e850)
    Lifted::alias(0x7001dc5620, tpl_59 as *const (), 1, 0x1ee00), // +0x1620 ?Max@TimeDelta@base@v8@@SA?AV123@XZ (thunk -> +0x1ee00)
    Lifted::alias(0x7001dc5630, fn_7001dde500 as *const (), 0, 0x1a500), // +0x1630 ?ShiftLeft@Bignum@base@v8@@QEAAXH@Z (thunk -> +0x1a500)
    Lifted::alias(0x7001dc5640, tpl_26::<0x8> as *const (), 1, 0x24900), // +0x1640 sub_1640 (thunk -> +0x24900)
    Lifted::alias(0x7001dc5650, fn_7001de8660 as *const (), 0, 0x24660), // +0x1650 sub_1650 (thunk -> +0x24660)
    Lifted::alias(0x7001dc5660, tpl_17::<0x52> as *const (), 1, 0xb0f0), // +0x1660 ?has_popcnt@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb0f0)
    Lifted::alias(0x7001dc5670, tpl_43 as *const (), 1, 0x1e520), // +0x1670 ??4TimeTicks@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x1e520)
    Lifted::alias(0x7001dc5680, fn_7001debd90 as *const (), 0, 0x27d90), // +0x1680 ?Start@Thread@base@v8@@QEAA_NXZ (thunk -> +0x27d90)
    Lifted::alias(0x7001dc5690, fn_7001dcdf90 as *const (), 1, 0x9f90), // +0x1690 ?SignedMulHighAndAdd32@bits@base@v8@@YAHHHH@Z (thunk -> +0x9f90)
    Lifted::alias(0x7001dc56a0, fn_7001dcf2b0 as *const (), 0, 0xb2b0), // +0x16a0 ??0StackTrace@debug@base@v8@@QEAA@PEBQEBX_K@Z (thunk -> +0xb2b0)
    Lifted::alias(0x7001dc56b0, fn_7001de9530 as *const (), 0, 0x25530), // +0x16b0 sub_16b0 (thunk -> +0x25530)
    Lifted::new(0x7001dc56c0, tpl_0::<0x7001de30b0, 0x7001dc56c5, 0x7001dc56c6, 0x7001dc56c7, 0x7001dc56c8, 0x7001dc56c9, 0x7001dc56ca, 0x7001dc56cb, 0x7001dc56cc, 0x7001dc56cd, 0x7001dc56ce, 0x7001dc56cf, 0x7001dd00c0> as *const (), 0), // +0x16c0 ?LockExclusive@SharedMutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc56d0, fn_7001dd00c0 as *const (), 1, 0xc0c0), // +0x16d0 sub_16d0 (thunk -> +0xc0c0)
    Lifted::alias(0x7001dc56e0, tpl_29::<0x7001e070b8, 0x7001dc64b0, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd8741> as *const (), 2, 0x14430), // +0x16e0 ??$MakeCheckOpString@_J_J@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@_J0PEBD@Z (thunk -> +0x14430)
    Lifted::alias(0x7001dc56f0, tpl_8::<0x18> as *const (), 1, 0xa5a0), // +0x16f0 ?CommitPageSize@BoundedPageAllocator@base@v8@@UEAA_KXZ (thunk -> +0xa5a0)
    Lifted::alias(0x7001dc5710, fn_7001ddd810 as *const (), 1, 0x19810), // +0x1710 ?BigitsShiftLeft@Bignum@base@v8@@AEAAXH@Z (thunk -> +0x19810)
    Lifted::alias(0x7001dc5720, tpl_35::<0x7001e10020> as *const (), 2, 0x2d7c0), // +0x1720 sub_1720 (thunk -> +0x2d7c0)
    Lifted::alias(0x7001dc5730, fn_7001de3800 as *const (), 1, 0x1f800), // +0x1730 ?InNanoseconds@TimeDelta@base@v8@@QEBA_JXZ (thunk -> +0x1f800)
    Lifted::alias(0x7001dc5770, fn_7001dee430 as *const (), 0, 0x2a430), // +0x1770 sub_1770 (thunk -> +0x2a430)
    Lifted::new(0x7001dc5780, tpl_1::<0x7001de3180> as *const (), 2), // +0x1780 ?UnlockShared@SharedMutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc5790, tpl_60 as *const (), 1, 0x1ee20), // +0x1790 ?Min@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@SA?AVThreadTicks@34@XZ (thunk -> +0x1ee20)
    Lifted::alias(0x7001dc57a0, fn_7001de3ac0 as *const (), 2, 0x1fac0), // +0x17a0 ?Now@ThreadTicks@base@v8@@SA?AV123@XZ (thunk -> +0x1fac0)
    Lifted::alias(0x7001dc57b0, tpl_58 as *const (), 1, 0x1ed60), // +0x17b0 ?IsNull@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ed60)
    Lifted::alias(0x7001dc57c0, fn_7001ddee80 as *const (), 2, 0x1ae80), // +0x17c0 sub_17c0 (thunk -> +0x1ae80)
    Lifted::alias(0x7001dc57d0, fn_7001debb20 as *const (), 0, 0x27b20), // +0x17d0 ?SetPermissions@OS@base@v8@@CA_NPEAX_KW4MemoryPermission@123@@Z (thunk -> +0x27b20)
    Lifted::alias(0x7001dc57e0, fn_7001de52f0 as *const (), 0, 0x212f0), // +0x17e0 ?IsFree@RegionAllocator@base@v8@@QEAA_N_K0@Z (thunk -> +0x212f0)
    Lifted::alias(0x7001dc57f0, tpl_40 as *const (), 1, 0x1e2f0), // +0x17f0 ?RemapShared@PageAllocator@base@v8@@AEAAPEAXPEAX0_K@Z (thunk -> +0x1e2f0)
    Lifted::alias(0x7001dc5820, fn_7001de55a0 as *const (), 0, 0x215a0), // +0x1820 ?Print@RegionAllocator@base@v8@@QEBAXAEAV?$basic_ostream@DU?$char_traits@D@std@@@std@@@Z (thunk -> +0x215a0)
    Lifted::alias(0x7001dc5830, tpl_30::<0x7001ddb390> as *const (), 0, 0x14c70), // +0x1830 ??$PrintCheckOperand@D@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@D@Z (thunk -> +0x14c70)
    Lifted::alias(0x7001dc5840, tpl_25 as *const (), 1, 0x1eab0), // +0x1840 ?FromInternalValue@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@SA?AVTimeTicks@34@_J@Z (thunk -> +0x1eab0)
    Lifted::alias(0x7001dc5860, tpl_20::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2030> as *const (), 0, 0x172c0), // +0x1860 sub_1860 (thunk -> +0x172c0)
    Lifted::alias(0x7001dc5870, fn_7001de2a70 as *const (), 1, 0x1ea70), // +0x1870 ?FromHours@TimeDelta@base@v8@@SA?AV123@H@Z (thunk -> +0x1ea70)
    Lifted::alias(0x7001dc5880, tpl_59 as *const (), 1, 0x1edc0), // +0x1880 ?Max@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@SA?AVTime@34@XZ (thunk -> +0x1edc0)
    Lifted::alias(0x7001dc5890, fn_7001dce3a0 as *const (), 0, 0xa3a0), // +0x1890 ?AllocatePages@BoundedPageAllocator@base@v8@@UEAAPEAXPEAX_K1W4Permission@PageAllocator@3@@Z (thunk -> +0xa3a0)
    Lifted::new(0x7001dc58a0, tpl_1::<0x7001dea440> as *const (), 2), // +0x18a0 ?GetThreadLocal@Thread@base@v8@@SAPEAXH@Z
    Lifted::alias(0x7001dc58b0, tpl_43 as *const (), 1, 0x1e4b0), // +0x18b0 ??4ThreadTicks@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0x1e4b0)
    Lifted::alias(0x7001dc58c0, tpl_50 as *const (), 1, 0x1e7d0), // +0x18c0 ??N?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NVTimeTicks@23@@Z (thunk -> +0x1e7d0)
    Lifted::alias(0x7001dc58e0, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0, 0xb6d0), // +0x18e0 sub_18e0 (thunk -> +0xb6d0)
    Lifted::alias(0x7001dc5910, fn_7001dd0920 as *const (), 1, 0xc920), // +0x1910 ??8?$MagicNumbersForDivision@_K@base@v8@@QEBA_NAEBU012@@Z (thunk -> +0xc920)
    Lifted::alias(0x7001dc5920, fn_7001dd3160 as *const (), 2, 0xf160), // +0x1920 ?asinh@ieee754@base@v8@@YANN@Z (thunk -> +0xf160)
    Lifted::alias(0x7001dc5930, fn_7001df17a0 as *const (), 2, 0x2d7a0), // +0x1930 sub_1930 (thunk -> +0x2d7a0)
    Lifted::alias(0x7001dc5940, fn_7001dd08e0 as *const (), 3, 0xc8e0), // +0x1940 ??4?$MagicNumbersForDivision@_K@base@v8@@QEAAAEAU012@AEBU012@@Z (thunk -> +0xc8e0)
    Lifted::alias(0x7001dc5950, fn_7001dd0b10 as *const (), 0, 0xcb10), // +0x1950 ?NotifyStartedAndRun@Thread@base@v8@@QEAAXXZ (thunk -> +0xcb10)
    Lifted::alias(0x7001dc5960, fn_7001de45d0 as *const (), 0, 0x205d0), // +0x1960 sub_1960 (thunk -> +0x205d0)
    Lifted::alias(0x7001dc5980, tpl_44 as *const (), 1, 0x1e550), // +0x1980 ??8?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NVTimeTicks@23@@Z (thunk -> +0x1e550)
    Lifted::alias(0x7001dc5990, fn_7001de4e70 as *const (), 0, 0x20e70), // +0x1990 ?AllocateRegionAt@RegionAllocator@base@v8@@QEAA_N_K0W4RegionState@123@@Z (thunk -> +0x20e70)
    Lifted::alias(0x7001dc59b0, tpl_28::<0x7001e070b8, 0x7001dc6f20, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd732e> as *const (), 2, 0x13020), // +0x19b0 ??$MakeCheckOpString@HH@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@HHPEBD@Z (thunk -> +0x13020)
    Lifted::new(0x7001dc59c0, tpl_1::<0x7001de2ea0> as *const (), 2), // +0x19c0 ?NotifyAll@ConditionVariable@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc59d0, fn_7001de2760 as *const (), 1, 0x1e760), // +0x19d0 ??KTimeDelta@base@v8@@QEBA_JAEBV012@@Z (thunk -> +0x1e760)
    Lifted::alias(0x7001dc59e0, tpl_17::<0x57> as *const (), 1, 0xb180), // +0x19e0 ?has_vfp3@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb180)
    Lifted::alias(0x7001dc59f0, fn_7001de39a0 as *const (), 0, 0x1f9a0), // +0x19f0 ?IsHighResolution@TimeTicks@base@v8@@SA_NXZ (thunk -> +0x1f9a0)
    Lifted::alias(0x7001dc5a00, fn_7001de9f10 as *const (), 0, 0x25f10), // +0x1a00 sub_1a00 (thunk -> +0x25f10)
    Lifted::alias(0x7001dc5a10, tpl_10 as *const (), 1, 0x27d50), // +0x1a10 ?SignalCodeMovingGC@OS@base@v8@@SAXXZ (thunk -> +0x27d50)
    Lifted::alias(0x7001dc5a20, fn_7001de1f90 as *const (), 1, 0x1df90), // +0x1a20 ??0PageAllocator@base@v8@@QEAA@AEBV012@@Z (thunk -> +0x1df90)
    Lifted::alias(0x7001dc5a30, fn_7001de9ae0 as *const (), 1, 0x25ae0), // +0x1a30 sub_1a30 (thunk -> +0x25ae0)
    Lifted::alias(0x7001dc5a40, tpl_7::<0x7001de2030> as *const (), 0, 0x256f0), // +0x1a40 sub_1a40 (thunk -> +0x256f0)
    Lifted::alias(0x7001dc5a50, tpl_17::<0x49> as *const (), 1, 0xb140), // +0x1a50 ?has_sse42@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb140)
    Lifted::alias(0x7001dc5a60, fn_7001deb9f0 as *const (), 0, 0x279f0), // +0x1a60 ?Print@OS@base@v8@@SAXPEBDZZ (thunk -> +0x279f0)
    Lifted::alias(0x7001dc5a70, tpl_12::<0x8, 0x20> as *const (), 0, 0x226a0), // +0x1a70 ?GetRandomMmapAddr@LsanPageAllocator@base@v8@@UEAAPEAXXZ (thunk -> +0x226a0)
    Lifted::alias(0x7001dc5a80, tpl_56 as *const (), 1, 0x1ec80), // +0x1a80 ?IsMax@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ec80)
    Lifted::alias(0x7001dc5a90, fn_7001dd08a0 as *const (), 3, 0xc8a0), // +0x1a90 ??4?$MagicNumbersForDivision@I@base@v8@@QEAAAEAU012@AEBU012@@Z (thunk -> +0xc8a0)
    Lifted::alias(0x7001dc5aa0, tpl_16::<0x30> as *const (), 1, 0xb240), // +0x1aa0 ?variant@CPU@base@v8@@QEBAHXZ (thunk -> +0xb240)
    Lifted::alias(0x7001dc5ab0, fn_7001dd0af0 as *const (), 0, 0xcaf0), // +0x1ab0 ?HasThreadLocal@Thread@base@v8@@SA_NH@Z (thunk -> +0xcaf0)
    Lifted::alias(0x7001dc5ac0, tpl_25 as *const (), 1, 0x1e340), // +0x1ac0 ??0?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@IEAA@_J@Z (thunk -> +0x1e340)
    Lifted::alias(0x7001dc5ad0, tpl_63::<0x7001e0a0d0> as *const (), 0, 0x1f0f0), // +0x1ad0 ?TryLock@RecursiveMutex@base@v8@@QEAA_NXZ (thunk -> +0x1f0f0)
    Lifted::alias(0x7001dc5b00, tpl_45 as *const (), 1, 0x1e580), // +0x1b00 ??9?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NVTime@23@@Z (thunk -> +0x1e580)
    Lifted::alias(0x7001dc5b10, fn_7001de9a90 as *const (), 0, 0x25a90), // +0x1b10 ?AllocatePageSize@OS@base@v8@@CA_KXZ (thunk -> +0x25a90)
    Lifted::alias(0x7001dc5b20, tpl_21::<0x7001df7a88, 0x7001de2030> as *const (), 0, 0x257a0), // +0x1b20 sub_1b20 (thunk -> +0x257a0)
    Lifted::alias(0x7001dc5b30, fn_7001dd03c0 as *const (), 1, 0xc3c0), // +0x1b30 ??$SignedDivisionByConstant@_K@base@v8@@YA?AU?$MagicNumbersForDivision@_K@01@_K@Z (thunk -> +0xc3c0)
    Lifted::alias(0x7001dc5b40, fn_7001de7330 as *const (), 1, 0x23330), // +0x1b40 ?Next@RandomNumberGenerator@base@v8@@AEAAHH@Z (thunk -> +0x23330)
    Lifted::alias(0x7001dc5b50, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a430, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x15c10), // +0x1b50 ??$PrintCheckOperand@_K@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@_K@Z (thunk -> +0x15c10)
    Lifted::alias(0x7001dc5b60, fn_7001dd0860 as *const (), 1, 0xc860), // +0x1b60 ??0?$MagicNumbersForDivision@_K@base@v8@@QEAA@_KI_N@Z (thunk -> +0xc860)
    Lifted::alias(0x7001dc5b80, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0, 0xb6a0), // +0x1b80 sub_1b80 (thunk -> +0xb6a0)
    Lifted::alias(0x7001dc5bb0, tpl_43 as *const (), 1, 0x1e480), // +0x1bb0 ??4?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEAAAEAV0123@$$QEAV0123@@Z (thunk -> +0x1e480)
    Lifted::alias(0x7001dc5bc0, fn_7001dce1c0 as *const (), 0, 0xa1c0), // +0x1bc0 sub_1bc0 (thunk -> +0xa1c0)
    Lifted::alias(0x7001dc5bd0, tpl_12::<0x8, 0x30> as *const (), 0, 0x22690), // +0x1bd0 ?FreePages@LsanPageAllocator@base@v8@@UEAA_NPEAX_K@Z (thunk -> +0x22690)
    Lifted::alias(0x7001dc5be0, fn_7001ddcde0 as *const (), 2, 0x18de0), // +0x1be0 ?AssignDecimalString@Bignum@base@v8@@QEAAXV?$Vector@$$CBD@23@@Z (thunk -> +0x18de0)
    Lifted::alias(0x7001dc5bf0, tpl_54::<0x7001dc5fa0> as *const (), 0, 0x1e950), // +0x1bf0 ??Z?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEAAAEAVTime@23@VTimeDelta@23@@Z (thunk -> +0x1e950)
    Lifted::alias(0x7001dc5c00, fn_7001def5b0 as *const (), 0, 0x2b5b0), // +0x1c00 sub_1c00 (thunk -> +0x2b5b0)
    Lifted::alias(0x7001dc5c10, tpl_44 as *const (), 1, 0x1e530), // +0x1c10 ??8?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NVThreadTicks@23@@Z (thunk -> +0x1e530)
    Lifted::alias(0x7001dc5c20, fn_7001de29b0 as *const (), 1, 0x1e9b0), // +0x1c20 ??ZTimeDelta@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x1e9b0)
    Lifted::alias(0x7001dc5c30, fn_7001de4210 as *const (), 3, 0x20210), // +0x1c30 ?ToJsTime@Time@base@v8@@QEBANXZ (thunk -> +0x20210)
    Lifted::alias(0x7001dc5c50, tpl_8::<0x8> as *const (), 1, 0xccf0), // +0x1c50 ?data@Thread@base@v8@@QEAAPEAVPlatformData@123@XZ (thunk -> +0xccf0)
    Lifted::alias(0x7001dc5c60, tpl_25 as *const (), 1, 0x1e4a0), // +0x1c60 ??4?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEAAAEAVTimeTicks@23@V423@@Z (thunk -> +0x1e4a0)
    Lifted::alias(0x7001dc5c70, fn_7001de4640 as *const (), 1, 0x20640), // +0x1c70 sub_1c70 (thunk -> +0x20640)
    Lifted::alias(0x7001dc5c80, tpl_17::<0x4d> as *const (), 1, 0xb000), // +0x1c80 ?has_avx2@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb000)
    Lifted::alias(0x7001dc5ca0, fn_7001de2160 as *const (), 0, 0x1e160), // +0x1ca0 ?AlignedMalloc@base@v8@@YAPEAX_K0@Z (thunk -> +0x1e160)
    Lifted::alias(0x7001dc5cb0, fn_7001dcff10 as *const (), 1, 0xbf10), // +0x1cb0 sub_1cb0 (thunk -> +0xbf10)
    Lifted::alias(0x7001dc5cc0, tpl_35::<0x7001e07070> as *const (), 2, 0x177f0), // +0x1cc0 ?V8_Dcheck@@YAXPEBDH0@Z (thunk -> +0x177f0)
    Lifted::alias(0x7001dc5cd0, fn_7001de22d0 as *const (), 0, 0x1e2d0), // +0x1cd0 ?ReleasePages@PageAllocator@base@v8@@UEAA_NPEAX_K1@Z (thunk -> +0x1e2d0)
    Lifted::alias(0x7001dc5ce0, fn_7001debe60 as *const (), 0, 0x27e60), // +0x1ce0 ?TimeCurrentMillis@OS@base@v8@@SANXZ (thunk -> +0x27e60)
    Lifted::alias(0x7001dc5cf0, fn_7001de72e0 as *const (), 1, 0x232e0), // +0x1cf0 ?MurmurHash3@RandomNumberGenerator@base@v8@@SA_K_K@Z (thunk -> +0x232e0)
    Lifted::alias(0x7001dc5d00, fn_7001de7960 as *const (), 2, 0x23960), // +0x1d00 ?NextSampleSlow@RandomNumberGenerator@base@v8@@QEAA?AV?$vector@_KV?$allocator@_K@std@@@std@@_K0AEBV?$unordered_set@_KU?$ (thunk -> +0x23960)
    Lifted::alias(0x7001dc5d10, fn_7001de92d0 as *const (), 0, 0x252d0), // +0x1d10 ??0Thread@base@v8@@QEAA@AEBVOptions@012@@Z (thunk -> +0x252d0)
    Lifted::alias(0x7001dc5d20, fn_7001de27a0 as *const (), 1, 0x1e7a0), // +0x1d20 ??MTimeDelta@base@v8@@QEBA_NAEBV012@@Z (thunk -> +0x1e7a0)
    Lifted::alias(0x7001dc5d30, fn_7001dd5160 as *const (), 3, 0x11160), // +0x1d30 ?log2@ieee754@base@v8@@YANN@Z (thunk -> +0x11160)
    Lifted::alias(0x7001dc5d40, tpl_17::<0x50> as *const (), 1, 0xb030), // +0x1d40 ?has_bmi2@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb030)
    Lifted::alias(0x7001dc5d50, tpl_25 as *const (), 1, 0x1e390), // +0x1d50 ??0ThreadTicks@base@v8@@AEAA@_J@Z (thunk -> +0x1e390)
    Lifted::alias(0x7001dc5d70, tpl_16::<0x24> as *const (), 1, 0xb230), // +0x1d70 ?type@CPU@base@v8@@QEBAHXZ (thunk -> +0xb230)
    Lifted::alias(0x7001dc5d80, tpl_66::<0x8, 0x7001dc5fb0> as *const (), 0, 0x22730), // +0x1d80 ?StrNCpy@base@v8@@YAXV?$Vector@D@12@PEBD_K@Z (thunk -> +0x22730)
    Lifted::alias(0x7001dc5da0, fn_7001de2910 as *const (), 1, 0x1e910), // +0x1da0 ??YTimeDelta@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x1e910)
    Lifted::alias(0x7001dc5db0, tpl_20::<0xffffffffffffff68, 0x4, 0x7001df5f18, 0xffffffffffffff64, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a378, 0x7001e0a360, 0x7001de2030> as *const (), 0, 0xb5f0), // +0x1db0 sub_1db0 (thunk -> +0xb5f0)
    Lifted::alias(0x7001dc5dd0, fn_7001de3cf0 as *const (), 0, 0x1fcf0), // +0x1dd0 ?NowFromSystemTime@Time@base@v8@@SA?AV123@XZ (thunk -> +0x1fcf0)
    Lifted::alias(0x7001dc5de0, fn_7001de2140 as *const (), 0, 0x1e140), // +0x1de0 ?AlignedFree@base@v8@@YAXPEAX@Z (thunk -> +0x1e140)
    Lifted::alias(0x7001dc5df0, tpl_16::<0x14> as *const (), 1, 0xb200), // +0x1df0 ?model@CPU@base@v8@@QEBAHXZ (thunk -> +0xb200)
    Lifted::alias(0x7001dc5e00, fn_7001deb540 as *const (), 2, 0x27540), // +0x1e00 sub_1e00 (thunk -> +0x27540)
    Lifted::alias(0x7001dc5e10, fn_7001debef0 as *const (), 0, 0x27ef0), // +0x1e10 ?VPrint@OS@base@v8@@SAXPEBDPEAD@Z (thunk -> +0x27ef0)
    Lifted::alias(0x7001dc5e20, fn_7001de51d0 as *const (), 1, 0x211d0), // +0x1e20 ?FreeListFindRegion@RegionAllocator@base@v8@@AEAAPEAVRegion@123@_K@Z (thunk -> +0x211d0)
    Lifted::alias(0x7001dc5e30, tpl_52 as *const (), 1, 0x1e840), // +0x1e30 ??P?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NVTime@23@@Z (thunk -> +0x1e840)
    Lifted::alias(0x7001dc5e40, fn_7001dcfc90 as *const (), 0, 0xbc90), // +0x1e40 sub_1e40 (thunk -> +0xbc90)
    Lifted::alias(0x7001dc5e50, fn_7001de9e40 as *const (), 0, 0x25e40), // +0x1e50 ?FOpen@OS@base@v8@@SAPEAU_iobuf@@PEBD0@Z (thunk -> +0x25e40)
    Lifted::alias(0x7001dc5e60, fn_7001ddca70 as *const (), 0, 0x18a70), // +0x1e60 ?AddBignum@Bignum@base@v8@@QEAAXAEBV123@@Z (thunk -> +0x18a70)
    Lifted::alias(0x7001dc5e70, fn_7001dea140 as *const (), 0, 0x26140), // +0x1e70 ?GetSharedLibraryAddresses@OS@base@v8@@SA?AV?$vector@USharedLibraryAddress@OS@base@v8@@V?$allocator@USharedLibraryAddres (thunk -> +0x26140)
    Lifted::alias(0x7001dc5e80, fn_7001de33a0 as *const (), 1, 0x1f3a0), // +0x1e80 ?FromFiletime@Time@base@v8@@SA?AV123@U_FILETIME@@@Z (thunk -> +0x1f3a0)
    Lifted::alias(0x7001dc5e90, fn_7001ddcd30 as *const (), 1, 0x18d30), // +0x1e90 ?AssignBignum@Bignum@base@v8@@QEAAXAEBV123@@Z (thunk -> +0x18d30)
    Lifted::alias(0x7001dc5ea0, fn_7001dec590 as *const (), 1, 0x28590), // +0x1ea0 ?isDirectorySeparator@OS@base@v8@@SA_ND@Z (thunk -> +0x28590)
    Lifted::alias(0x7001dc5eb0, tpl_61 as *const (), 1, 0x1f000), // +0x1eb0 ?since_origin@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA?AVTimeDelta@34@XZ (thunk -> +0x1f000)
    Lifted::alias(0x7001dc5ec0, fn_7001de2e80 as *const (), 0, 0x1ee80), // +0x1ec0 ?Min@TimeDelta@base@v8@@SA?AV123@XZ (thunk -> +0x1ee80)
    Lifted::alias(0x7001dc5ed0, fn_7001de8680 as *const (), 0, 0x24680), // +0x1ed0 sub_1ed0 (thunk -> +0x24680)
    Lifted::alias(0x7001dc5ee0, fn_7001dd2870 as *const (), 2, 0xe870), // +0x1ee0 ?acos@ieee754@base@v8@@YANN@Z (thunk -> +0xe870)
    Lifted::alias(0x7001dc5ef0, fn_7001de64e0 as *const (), 1, 0x224e0), // +0x1ef0 ??0LsanPageAllocator@base@v8@@QEAA@AEBV012@@Z (thunk -> +0x224e0)
    Lifted::alias(0x7001dc5f00, fn_7001de7570 as *const (), 2, 0x23570), // +0x1f00 ?NextSample@RandomNumberGenerator@base@v8@@QEAA?AV?$vector@_KV?$allocator@_K@std@@@std@@_K0@Z (thunk -> +0x23570)
    Lifted::alias(0x7001dc5f10, tpl_16::<0x2c> as *const (), 1, 0xafb0), // +0x1f10 ?architecture@CPU@base@v8@@QEBAHXZ (thunk -> +0xafb0)
    Lifted::alias(0x7001dc5f20, tpl_57 as *const (), 1, 0x1ed20), // +0x1f20 ?IsMin@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ed20)
    Lifted::alias(0x7001dc5f30, fn_7001de5400 as *const (), 0, 0x21400), // +0x1f30 ?Merge@RegionAllocator@base@v8@@AEAAXV?$_Tree_const_iterator@V?$_Tree_val@U?$_Tree_simple_types@PEAVRegion@RegionAllocat (thunk -> +0x21400)
    Lifted::alias(0x7001dc5f40, fn_7001de4cf0 as *const (), 0, 0x20cf0), // +0x1f40 ?AllocateRegion@RegionAllocator@base@v8@@QEAA_KPEAVRandomNumberGenerator@23@_K@Z (thunk -> +0x20cf0)
    Lifted::alias(0x7001dc5f50, tpl_41::<0x7001e0a068> as *const (), 0, 0x1e370), // +0x1f50 ??0ConditionVariable@base@v8@@QEAA@XZ (thunk -> +0x1e370)
    Lifted::alias(0x7001dc5f60, fn_7001dea630 as *const (), 0, 0x26630), // +0x1f60 sub_1f60 (thunk -> +0x26630)
    Lifted::alias(0x7001dc5f70, fn_7001dddf80 as *const (), 0, 0x19f80), // +0x1f70 ?MultiplyByUInt32@Bignum@base@v8@@QEAAXI@Z (thunk -> +0x19f80)
    Lifted::alias(0x7001dc5f80, fn_7001decaa0 as *const (), 0, 0x28aa0), // +0x1f80 ?EnableInProcessStackDumping@debug@base@v8@@YA_NXZ (thunk -> +0x28aa0)
    Lifted::alias(0x7001dc5f90, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0, 0xb700), // +0x1f90 sub_1f90 (thunk -> +0xb700)
    Lifted::alias(0x7001dc5fa0, fn_7001dce030 as *const (), 1, 0xa030), // +0x1fa0 ?SignedSaturatedSub64@bits@base@v8@@YA_J_J0@Z (thunk -> +0xa030)
    Lifted::alias(0x7001dc5fb0, fn_7001debdf0 as *const (), 0, 0x27df0), // +0x1fb0 ?StrNCpy@OS@base@v8@@SAXPEADHPEBD_K@Z (thunk -> +0x27df0)
    Lifted::alias(0x7001dc5fc0, tpl_12::<0x8, 0x10> as *const (), 0, 0x25bc0), // +0x1fc0 sub_1fc0 (thunk -> +0x25bc0)
    Lifted::alias(0x7001dc5fe0, tpl_17::<0x47> as *const (), 1, 0xb160), // +0x1fe0 ?has_ssse3@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb160)
    Lifted::alias(0x7001dc6000, fn_7001ded780 as *const (), 0, 0x29780), // +0x2000 ?Print@StackTrace@debug@base@v8@@QEBAXXZ (thunk -> +0x29780)
    Lifted::alias(0x7001dc6010, fn_7001ded8a0 as *const (), 0, 0x298a0), // +0x2010 sub_2010 (thunk -> +0x298a0)
    Lifted::alias(0x7001dc6020, tpl_53::<0x7001dc6500> as *const (), 0, 0x1e8b0), // +0x2020 ??Y?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEAAAEAVTime@23@VTimeDelta@23@@Z (thunk -> +0x1e8b0)
    Lifted::alias(0x7001dc6030, fn_7001de6610 as *const (), 0, 0x22610), // +0x2030 ?AllocateSharedPages@LsanPageAllocator@base@v8@@UEAA?AV?$unique_ptr@VSharedMemory@PageAllocator@v8@@U?$default_delete@VS (thunk -> +0x22610)
    Lifted::alias(0x7001dc6040, fn_7001dec410 as *const (), 0, 0x28410), // +0x2040 ?create@MemoryMappedFile@OS@base@v8@@SAPEAV1234@PEBD_KPEAX@Z (thunk -> +0x28410)
    Lifted::alias(0x7001dc6050, fn_7001de4c10 as *const (), 0, 0x20c10), // +0x2050 ??1RegionAllocator@base@v8@@QEAA@XZ (thunk -> +0x20c10)
    Lifted::alias(0x7001dc6060, fn_7001dcdf20 as *const (), 1, 0x9f20), // +0x2060 ?SignedDiv32@bits@base@v8@@YAHHH@Z (thunk -> +0x9f20)
    Lifted::alias(0x7001dc6070, fn_7001de8ad0 as *const (), 0, 0x24ad0), // +0x2070 sub_2070 (thunk -> +0x24ad0)
    Lifted::alias(0x7001dc6080, tpl_44 as *const (), 1, 0x1e540), // +0x2080 ??8?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NVTime@23@@Z (thunk -> +0x1e540)
    Lifted::alias(0x7001dc6090, fn_7001dddcd0 as *const (), 0, 0x19cd0), // +0x2090 ?MultiplyByPowerOfTen@Bignum@base@v8@@QEAAXH@Z (thunk -> +0x19cd0)
    Lifted::new(0x7001dc60a0, tpl_0::<0x7001de30c0, 0x7001dc60a5, 0x7001dc60a6, 0x7001dc60a7, 0x7001dc60a8, 0x7001dc60a9, 0x7001dc60aa, 0x7001dc60ab, 0x7001dc60ac, 0x7001dc60ad, 0x7001dc60ae, 0x7001dc60af, 0x7001dee8f4> as *const (), 0), // +0x20a0 ?LockShared@SharedMutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc60c0, tpl_15 as *const (), 1, 0xca30), // +0x20c0 ??4Stack@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0xca30)
    Lifted::alias(0x7001dc60d0, fn_7001dd2e10 as *const (), 2, 0xee10), // +0x20d0 ?asin@ieee754@base@v8@@YANN@Z (thunk -> +0xee10)
    Lifted::alias(0x7001dc60f0, fn_7001dcdec0 as *const (), 1, 0x9ec0), // +0x20f0 ?RoundUpToPowerOfTwo64@bits@base@v8@@YA_K_K@Z (thunk -> +0x9ec0)
    Lifted::alias(0x7001dc6100, tpl_22::<0x7001df78b8, 0x7001deddc0, 0x7001dec1b0> as *const (), 0, 0x281a0), // +0x2100 sub_2100 (thunk -> +0x281a0)
    Lifted::alias(0x7001dc6110, tpl_8::<0x18> as *const (), 1, 0x22680), // +0x2110 ?CommitPageSize@LsanPageAllocator@base@v8@@UEAA_KXZ (thunk -> +0x22680)
    Lifted::alias(0x7001dc6120, fn_7001de8fe0 as *const (), 2, 0x24fe0), // +0x2120 sub_2120 (thunk -> +0x24fe0)
    Lifted::alias(0x7001dc6130, tpl_11 as *const (), 1, 0x1e1e0), // +0x2130 ?CanAllocateSharedPages@PageAllocator@base@v8@@UEAA_NXZ (thunk -> +0x1e1e0)
    Lifted::alias(0x7001dc6140, fn_7001ddb910 as *const (), 1, 0x17910), // +0x2140 ?BigitLength@Bignum@base@v8@@AEBAHXZ (thunk -> +0x17910)
    Lifted::alias(0x7001dc6150, tpl_23::<0x7001df5f58> as *const (), 1, 0xc9d0), // +0x2150 ??0MemoryMappedFile@OS@base@v8@@QEAA@AEBV0123@@Z (thunk -> +0xc9d0)
    Lifted::alias(0x7001dc6160, fn_7001ded810 as *const (), 0, 0x29810), // +0x2160 sub_2160 (thunk -> +0x29810)
    Lifted::alias(0x7001dc6170, tpl_46::<0x7001dc5fa0> as *const (), 0, 0x1e650), // +0x2170 ??G?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA?AVTimeTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e650)
    Lifted::alias(0x7001dc6180, tpl_10 as *const (), 1, 0xa550), // +0x2180 ?AssertHeld@RecursiveMutex@base@v8@@QEBAXXZ (thunk -> +0xa550)
    Lifted::alias(0x7001dc6190, fn_7001de25a0 as *const (), 1, 0x1e5a0), // +0x2190 ??9TimeDelta@base@v8@@QEBA_NAEBV012@@Z (thunk -> +0x1e5a0)
    Lifted::alias(0x7001dc61a0, tpl_49 as *const (), 1, 0x1e770), // +0x21a0 ??M?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NVThreadTicks@23@@Z (thunk -> +0x1e770)
    Lifted::alias(0x7001dc61b0, fn_7001dee430 as *const (), 0, 0x2a430), // +0x21b0 sub_21b0 (thunk -> +0x2a430)
    Lifted::alias(0x7001dc61d0, fn_7001dea6d0 as *const (), 0, 0x266d0), // +0x21d0 sub_21d0 (thunk -> +0x266d0)
    Lifted::alias(0x7001dc61e0, tpl_6::<0x7001dc7b40> as *const (), 0, 0x157b0), // +0x21e0 ??$PrintCheckOperand@PEBE@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBE@Z (thunk -> +0x157b0)
    Lifted::alias(0x7001dc61f0, fn_7001dee9a0 as *const (), 0, 0x2a9a0), // +0x21f0 sub_21f0 (thunk -> +0x2a9a0)
    Lifted::alias(0x7001dc6200, fn_7001dce6a0 as *const (), 0, 0xa6a0), // +0x2200 ?GetRandomMmapAddr@BoundedPageAllocator@base@v8@@UEAAPEAXXZ (thunk -> +0xa6a0)
    Lifted::alias(0x7001dc6210, tpl_25 as *const (), 1, 0x1e440), // +0x2210 ??4?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEAAAEAVThreadTicks@23@V423@@Z (thunk -> +0x1e440)
    Lifted::alias(0x7001dc6220, tpl_36::<0x7001dc5590> as *const (), 0, 0x18580), // +0x2220 ?PlusEqual@Bignum@base@v8@@SA_NAEBV123@00@Z (thunk -> +0x18580)
    Lifted::new(0x7001dc6250, tpl_1::<0x7001debc60> as *const (), 2), // +0x2250 ?SetThreadLocal@Thread@base@v8@@SAXHPEAX@Z
    Lifted::alias(0x7001dc6260, tpl_55::<0x7001df75d0, 0x7001df75e0, 0x7001df75f0> as *const (), 3, 0x1ebe0), // +0x2260 ?FromSecondsD@TimeDelta@base@v8@@SA?AV123@N@Z (thunk -> +0x1ebe0)
    Lifted::alias(0x7001dc6270, fn_7001ddeb90 as *const (), 1, 0x1ab90), // +0x2270 ?ToHexString@Bignum@base@v8@@QEBA_NPEADH@Z (thunk -> +0x1ab90)
    Lifted::alias(0x7001dc6280, tpl_17::<0x51> as *const (), 1, 0xb090), // +0x2280 ?has_lzcnt@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb090)
    Lifted::alias(0x7001dc62a0, fn_7001deca00 as *const (), 0, 0x28a00), // +0x22a0 ??0StackTrace@debug@base@v8@@QEAA@PEAU_EXCEPTION_POINTERS@@@Z (thunk -> +0x28a00)
    Lifted::alias(0x7001dc62b0, fn_7001de8590 as *const (), 0, 0x24590), // +0x22b0 sub_22b0 (thunk -> +0x24590)
    Lifted::alias(0x7001dc62c0, fn_7001debf70 as *const (), 0, 0x27f70), // +0x22c0 ?VFPrint@OS@base@v8@@SAXPEAU_iobuf@@PEBDPEAD@Z (thunk -> +0x27f70)
    Lifted::alias(0x7001dc62d0, fn_7001dd3aa0 as *const (), 3, 0xfaa0), // +0x22d0 ?cbrt@ieee754@base@v8@@YANN@Z (thunk -> +0xfaa0)
    Lifted::alias(0x7001dc62f0, fn_7001de2320 as *const (), 0, 0x1e320), // +0x22f0 ?SetRandomMmapSeed@PageAllocator@base@v8@@UEAAX_J@Z (thunk -> +0x1e320)
    Lifted::alias(0x7001dc6300, fn_7001de97d0 as *const (), 0, 0x257d0), // +0x2300 ?Abort@OS@base@v8@@SAXXZ (thunk -> +0x257d0)
    Lifted::alias(0x7001dc6310, fn_7001de9860 as *const (), 0, 0x25860), // +0x2310 ?Allocate@OS@base@v8@@CAPEAXPEAX_K1W4MemoryPermission@123@@Z (thunk -> +0x25860)
    Lifted::alias(0x7001dc6320, tpl_17::<0x5c> as *const (), 1, 0xb1f0), // +0x2320 ?is_running_in_vm@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb1f0)
    Lifted::alias(0x7001dc6330, tpl_43 as *const (), 1, 0x1e4f0), // +0x2330 ??4TimeDelta@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0x1e4f0)
    Lifted::alias(0x7001dc6340, tpl_17::<0x58> as *const (), 1, 0xb190), // +0x2340 ?has_vfp3_d32@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb190)
    Lifted::alias(0x7001dc6360, fn_7001dcf4a0 as *const (), 1, 0xb4a0), // +0x2360 ??1StackTrace@debug@base@v8@@QEAA@XZ (thunk -> +0xb4a0)
    Lifted::alias(0x7001dc6370, tpl_17::<0x5d> as *const (), 1, 0xb0b0), // +0x2370 ?has_msa@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb0b0)
    Lifted::alias(0x7001dc6380, tpl_51 as *const (), 1, 0x1e810), // +0x2380 ??O?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NVTimeTicks@23@@Z (thunk -> +0x1e810)
    Lifted::alias(0x7001dc6390, fn_7001de8690 as *const (), 0, 0x24690), // +0x2390 sub_2390 (thunk -> +0x24690)
    Lifted::alias(0x7001dc63a0, fn_7001dea6c0 as *const (), 1, 0x266c0), // +0x23a0 ?Initialize@OS@base@v8@@SAX_NQEBD@Z (thunk -> +0x266c0)
    Lifted::alias(0x7001dc63b0, tpl_59 as *const (), 1, 0x1ede0), // +0x23b0 ?Max@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@SA?AVTimeTicks@34@XZ (thunk -> +0x1ede0)
    Lifted::alias(0x7001dc63c0, fn_7001de2560 as *const (), 1, 0x1e560), // +0x23c0 ??8TimeDelta@base@v8@@QEBA_NAEBV012@@Z (thunk -> +0x1e560)
    Lifted::alias(0x7001dc63d0, fn_7001dcf940 as *const (), 0, 0xb940), // +0x23d0 sub_23d0 (thunk -> +0xb940)
    Lifted::alias(0x7001dc63e0, tpl_39::<0x7001e074f0, 0x7001de20f7, 0x7001de20f8, 0x7001de20f9, 0x7001de20fa, 0x7001de20fb, 0x7001de20fc, 0x7001de20fd, 0x7001de20fe, 0x7001de20ff> as *const (), 0, 0x1e0f0), // +0x23e0 sub_23e0 (thunk -> +0x1e0f0)
    Lifted::alias(0x7001dc63f0, fn_7001de2300 as *const (), 0, 0x1e300), // +0x23f0 ?SetPermissions@PageAllocator@base@v8@@UEAA_NPEAX_KW4Permission@13@@Z (thunk -> +0x1e300)
    Lifted::alias(0x7001dc6400, fn_7001de3a00 as *const (), 0, 0x1fa00), // +0x2400 ?IsSupportedWin@ThreadTicks@base@v8@@CA_NXZ (thunk -> +0x1fa00)
    Lifted::alias(0x7001dc6410, fn_7001dcfa80 as *const (), 0, 0xba80), // +0x2410 sub_2410 (thunk -> +0xba80)
    Lifted::alias(0x7001dc6420, tpl_7::<0x7001de2030> as *const (), 0, 0x256f0), // +0x2420 sub_2420 (thunk -> +0x256f0)
    Lifted::alias(0x7001dc6430, tpl_43 as *const (), 1, 0x1e430), // +0x2430 ??4?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEAAAEAV0123@AEBV0123@@Z (thunk -> +0x1e430)
    Lifted::alias(0x7001dc6440, tpl_43 as *const (), 1, 0x1e500), // +0x2440 ??4TimeDelta@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x1e500)
    Lifted::alias(0x7001dc6450, fn_7001dddca0 as *const (), 1, 0x19ca0), // +0x2450 ?IsClamped@Bignum@base@v8@@AEBA_NXZ (thunk -> +0x19ca0)
    Lifted::alias(0x7001dc6460, tpl_16::<0x3c> as *const (), 1, 0xafc0), // +0x2460 ?dcache_line_size@CPU@base@v8@@QEBAHXZ (thunk -> +0xafc0)
    Lifted::alias(0x7001dc6470, tpl_42 as *const (), 1, 0x1e3e0), // +0x2470 ??0TimeDelta@base@v8@@QEAA@XZ (thunk -> +0x1e3e0)
    Lifted::alias(0x7001dc64a0, fn_7001de2bc0 as *const (), 1, 0x1ebc0), // +0x24a0 ?FromSeconds@TimeDelta@base@v8@@SA?AV123@_J@Z (thunk -> +0x1ebc0)
    Lifted::alias(0x7001dc64b0, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a428, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x159f0), // +0x24b0 ??$PrintCheckOperand@_J@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@_J@Z (thunk -> +0x159f0)
    Lifted::alias(0x7001dc64c0, tpl_35::<0x7001e074d0> as *const (), 2, 0x1e2c0), // +0x24c0 ?Realloc@base@v8@@YAPEAXPEAX_K@Z (thunk -> +0x1e2c0)
    Lifted::alias(0x7001dc64d0, fn_7001dce460 as *const (), 0, 0xa460), // +0x24d0 ?AllocatePagesAt@BoundedPageAllocator@base@v8@@QEAA_N_K0W4Permission@PageAllocator@3@@Z (thunk -> +0xa460)
    Lifted::alias(0x7001dc64e0, tpl_21::<0x7001df7a88, 0x7001de2030> as *const (), 0, 0x257a0), // +0x24e0 sub_24e0 (thunk -> +0x257a0)
    Lifted::alias(0x7001dc64f0, tpl_25 as *const (), 1, 0x1e3b0), // +0x24f0 ??0Time@base@v8@@AEAA@_J@Z (thunk -> +0x1e3b0)
    Lifted::alias(0x7001dc6500, fn_7001dcdfe0 as *const (), 1, 0x9fe0), // +0x2500 ?SignedSaturatedAdd64@bits@base@v8@@YA_J_J0@Z (thunk -> +0x9fe0)
    Lifted::alias(0x7001dc6520, tpl_56 as *const (), 1, 0x1ecc0), // +0x2520 ?IsMax@TimeDelta@base@v8@@QEBA_NXZ (thunk -> +0x1ecc0)
    Lifted::alias(0x7001dc6530, fn_7001de2ec0 as *const (), 3, 0x1eec0), // +0x2530 ?PercentOf@TimeDelta@base@v8@@QEBANAEBV123@@Z (thunk -> +0x1eec0)
    Lifted::alias(0x7001dc6550, fn_7001de2110 as *const (), 0, 0x1e110), // +0x2550 sub_2550 (thunk -> +0x1e110)
    Lifted::new(0x7001dc6560, tpl_1::<0x7001dc58a0> as *const (), 2), // +0x2560 ?GetExistingThreadLocal@Thread@base@v8@@SAPEAXH@Z
    Lifted::alias(0x7001dc6570, tpl_11 as *const (), 1, 0x26620), // +0x2570 ?HasLazyCommits@OS@base@v8@@SA_NXZ (thunk -> +0x26620)
    Lifted::alias(0x7001dc6580, fn_7001dedf90 as *const (), 0, 0x29f90), // +0x2580 sub_2580 (thunk -> +0x29f90)
    Lifted::alias(0x7001dc6590, tpl_15 as *const (), 1, 0xb250), // +0x2590 ?vendor@CPU@base@v8@@QEBAPEBDXZ (thunk -> +0xb250)
    Lifted::alias(0x7001dc65a0, tpl_10 as *const (), 1, 0x1e410), // +0x25a0 ??1ConditionVariable@base@v8@@QEAA@XZ (thunk -> +0x1e410)
    Lifted::alias(0x7001dc65b0, fn_7001ddff30 as *const (), 2, 0x1bf30), // +0x25b0 ?FastFixedDtoa@base@v8@@YA_NNHV?$Vector@D@12@PEAH1@Z (thunk -> +0x1bf30)
    Lifted::alias(0x7001dc65c0, fn_7001dcef70 as *const (), 3, 0xaf70), // +0x25c0 ??4CPU@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0xaf70)
    Lifted::alias(0x7001dc65d0, tpl_20::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2030> as *const (), 0, 0x17210), // +0x25d0 sub_25d0 (thunk -> +0x17210)
    Lifted::alias(0x7001dc6610, tpl_8::<0x10> as *const (), 1, 0x225f0), // +0x2610 ?AllocatePageSize@LsanPageAllocator@base@v8@@UEAA_KXZ (thunk -> +0x225f0)
    Lifted::alias(0x7001dc6620, fn_7001de9bf0 as *const (), 2, 0x25bf0), // +0x2620 sub_2620 (thunk -> +0x25bf0)
    Lifted::alias(0x7001dc6630, fn_7001debab0 as *const (), 0, 0x27ab0), // +0x2630 ?Release@OS@base@v8@@CA_NPEAX_K@Z (thunk -> +0x27ab0)
    Lifted::alias(0x7001dc6640, tpl_7::<0x7001de2030> as *const (), 0, 0xa2f0), // +0x2640 sub_2640 (thunk -> +0xa2f0)
    Lifted::alias(0x7001dc6650, fn_7001de3820 as *const (), 1, 0x1f820), // +0x2650 ?InSeconds@TimeDelta@base@v8@@QEBA_JXZ (thunk -> +0x1f820)
    Lifted::alias(0x7001dc6660, fn_7001de4fa0 as *const (), 1, 0x20fa0), // +0x2660 ?CheckRegion@RegionAllocator@base@v8@@QEAA_K_K@Z (thunk -> +0x20fa0)
    Lifted::alias(0x7001dc6670, tpl_22::<0x7001df5e70, 0x7001deddc0, 0x7001dcfa00> as *const (), 0, 0xb9f0), // +0x2670 sub_2670 (thunk -> +0xb9f0)
    Lifted::alias(0x7001dc6680, tpl_30::<0x7001ddb390> as *const (), 0, 0x14c50), // +0x2680 ??$PrintCheckOperand@C@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@C@Z (thunk -> +0x14c50)
    Lifted::alias(0x7001dc6690, tpl_7::<0x7001de2030> as *const (), 0, 0x25650), // +0x2690 sub_2690 (thunk -> +0x25650)
    Lifted::alias(0x7001dc66a0, tpl_27 as *const (), 1, 0xcda0), // +0x26a0 ?hash_value@base@v8@@YA_KK@Z (thunk -> +0xcda0)
    Lifted::alias(0x7001dc66b0, fn_7001deca50 as *const (), 0, 0x28a50), // +0x26b0 ??0StackTrace@debug@base@v8@@QEAA@XZ (thunk -> +0x28a50)
    Lifted::alias(0x7001dc66c0, fn_7001de2740 as *const (), 1, 0x1e740), // +0x26c0 ??KTimeDelta@base@v8@@QEBA?AV012@_J@Z (thunk -> +0x1e740)
    Lifted::alias(0x7001dc66d0, tpl_6::<0x7001dd9550> as *const (), 0, 0x15530), // +0x26d0 ??$PrintCheckOperand@PEAE@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z (thunk -> +0x15530)
    Lifted::alias(0x7001dc66e0, tpl_60 as *const (), 1, 0x1ee60), // +0x26e0 ?Min@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@SA?AVTimeTicks@34@XZ (thunk -> +0x1ee60)
    Lifted::alias(0x7001dc6700, tpl_19::<0xfffffffffffffffc, 0x7001ddb2c0> as *const (), 0, 0x17200), // +0x2700 sub_2700 (thunk -> +0x17200)
    Lifted::alias(0x7001dc6710, tpl_54::<0x7001dc5fa0> as *const (), 0, 0x1e980), // +0x2710 ??Z?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEAAAEAVTimeTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e980)
    Lifted::alias(0x7001dc6720, tpl_43 as *const (), 1, 0x1e4e0), // +0x2720 ??4Time@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x1e4e0)
    Lifted::alias(0x7001dc6730, tpl_19::<0xfffffffffffffffc, 0x7001dcf5f0> as *const (), 0, 0xb5a0), // +0x2730 sub_2730 (thunk -> +0xb5a0)
    Lifted::alias(0x7001dc6740, tpl_43 as *const (), 1, 0x1e490), // +0x2740 ??4?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEAAAEAV0123@AEBV0123@@Z (thunk -> +0x1e490)
    Lifted::alias(0x7001dc6770, tpl_17::<0x59> as *const (), 1, 0xb080), // +0x2770 ?has_jscvt@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb080)
    Lifted::alias(0x7001dc6780, fn_7001deb6a0 as *const (), 2, 0x276a0), // +0x2780 sub_2780 (thunk -> +0x276a0)
    Lifted::alias(0x7001dc67b0, tpl_64::<0x7001e07090> as *const (), 0, 0x1fcc0), // +0x27b0 ?Now@TimeTicks@base@v8@@SA?AV123@XZ (thunk -> +0x1fcc0)
    Lifted::alias(0x7001dc67c0, tpl_42 as *const (), 1, 0x1e3a0), // +0x27c0 ??0ThreadTicks@base@v8@@QEAA@XZ (thunk -> +0x1e3a0)
    Lifted::alias(0x7001dc67d0, fn_7001de6820 as *const (), 2, 0x22820), // +0x27d0 ?NumberOfProcessors@SysInfo@base@v8@@SAHXZ (thunk -> +0x22820)
    Lifted::alias(0x7001dc67f0, fn_7001dd36d0 as *const (), 3, 0xf6d0), // +0x27f0 ?atan@ieee754@base@v8@@YANN@Z (thunk -> +0xf6d0)
    Lifted::alias(0x7001dc6820, fn_7001de2a00 as *const (), 3, 0x1ea00), // +0x2820 ?FromDouble@TimeDelta@base@v8@@CA?AV123@N@Z (thunk -> +0x1ea00)
    Lifted::alias(0x7001dc6840, tpl_10 as *const (), 1, 0x25850), // +0x2840 ?AdjustSchedulingParams@OS@base@v8@@SAXXZ (thunk -> +0x25850)
    Lifted::alias(0x7001dc6850, tpl_12::<0x8, 0x18> as *const (), 0, 0x27690), // +0x2850 sub_2850 (thunk -> +0x27690)
    Lifted::alias(0x7001dc6860, tpl_57 as *const (), 1, 0x1ed00), // +0x2860 ?IsMin@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ed00)
    Lifted::alias(0x7001dc6870, tpl_35::<0x7001e074c0> as *const (), 2, 0x1e2b0), // +0x2870 ?Malloc@base@v8@@YAPEAX_K@Z (thunk -> +0x1e2b0)
    Lifted::alias(0x7001dc6880, fn_7001dce5d0 as *const (), 0, 0xa5d0), // +0x2880 ?FreePages@BoundedPageAllocator@base@v8@@UEAA_NPEAX_K@Z (thunk -> +0xa5d0)
    Lifted::alias(0x7001dc68a0, tpl_47 as *const (), 1, 0x1e5f0), // +0x28a0 ??G?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA?AVTimeDelta@23@VThreadTicks@23@@Z (thunk -> +0x1e5f0)
    Lifted::alias(0x7001dc68b0, tpl_42 as *const (), 1, 0x1e3c0), // +0x28b0 ??0Time@base@v8@@QEAA@XZ (thunk -> +0x1e3c0)
    Lifted::alias(0x7001dc68d0, tpl_14 as *const (), 1, 0x1ef30), // +0x28d0 ?ToInternalValue@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_JXZ (thunk -> +0x1ef30)
    Lifted::alias(0x7001dc68e0, fn_7001dcf5b0 as *const (), 0, 0xb5b0), // +0x28e0 sub_28e0 (thunk -> +0xb5b0)
    Lifted::alias(0x7001dc68f0, fn_7001de9460 as *const (), 0, 0x25460), // +0x28f0 ??1Thread@base@v8@@UEAA@XZ (thunk -> +0x25460)
    Lifted::alias(0x7001dc6900, fn_7001ddb920 as *const (), 2, 0x17920), // +0x2900 ?BignumDtoa@base@v8@@YAXNW4BignumDtoaMode@12@HV?$Vector@D@12@PEAH2@Z (thunk -> +0x17920)
    Lifted::alias(0x7001dc6920, fn_7001de9840 as *const (), 1, 0x25840), // +0x2920 ?ActivationFrameAlignment@OS@base@v8@@SAHXZ (thunk -> +0x25840)
    Lifted::alias(0x7001dc6930, tpl_17::<0x56> as *const (), 1, 0xb1a0), // +0x2930 ?has_vfp@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb1a0)
    Lifted::alias(0x7001dc6940, fn_7001dcf4d0 as *const (), 3, 0xb4d0), // +0x2940 ??4StackTrace@debug@base@v8@@QEAAAEAV0123@AEBV0123@@Z (thunk -> +0xb4d0)
    Lifted::alias(0x7001dc6950, fn_7001de9d90 as *const (), 0, 0x25d90), // +0x2950 ?EnsureConsoleOutputWin32@base@v8@@YAXXZ (thunk -> +0x25d90)
    Lifted::alias(0x7001dc6960, tpl_17::<0x41> as *const (), 1, 0xb040), // +0x2960 ?has_cmov@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb040)
    Lifted::alias(0x7001dc6970, fn_7001de60c0 as *const (), 1, 0x220c0), // +0x2970 sub_2970 (thunk -> +0x220c0)
    Lifted::alias(0x7001dc6990, fn_7001de3080 as *const (), 0, 0x1f080), // +0x2990 ??1SharedMutex@base@v8@@QEAA@XZ (thunk -> +0x1f080)
    Lifted::alias(0x7001dc69a0, tpl_48::<0x7001dc6500> as *const (), 0, 0x1e6a0), // +0x29a0 ??H?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA?AVThreadTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e6a0)
    Lifted::alias(0x7001dc69b0, tpl_51 as *const (), 1, 0x1e800), // +0x29b0 ??O?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NVTime@23@@Z (thunk -> +0x1e800)
    Lifted::alias(0x7001dc69c0, fn_7001de3200 as *const (), 0, 0x1f200), // +0x29c0 ?Wait@Semaphore@base@v8@@QEAAXXZ (thunk -> +0x1f200)
    Lifted::alias(0x7001dc69d0, tpl_17::<0x5a> as *const (), 1, 0xb1e0), // +0x29d0 ?is_fp64_mode@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb1e0)
    Lifted::alias(0x7001dc69e0, tpl_14 as *const (), 1, 0x1ef10), // +0x29e0 ?ToInternalValue@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_JXZ (thunk -> +0x1ef10)
    Lifted::alias(0x7001dc6a10, fn_7001ddaef0 as *const (), 0, 0x16ef0), // +0x2a10 sub_2a10 (thunk -> +0x16ef0)
    Lifted::alias(0x7001dc6a20, fn_7001de5960 as *const (), 0, 0x21960), // +0x2a20 ?TrimRegion@RegionAllocator@base@v8@@QEAA_K_K0@Z (thunk -> +0x21960)
    Lifted::alias(0x7001dc6a40, fn_7001dd3930 as *const (), 2, 0xf930), // +0x2a40 ?atanh@ieee754@base@v8@@YANN@Z (thunk -> +0xf930)
    Lifted::alias(0x7001dc6a50, tpl_17::<0x40> as *const (), 1, 0xb060), // +0x2a50 ?has_fpu@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb060)
    Lifted::alias(0x7001dc6a60, tpl_57 as *const (), 1, 0x1ed40), // +0x2a60 ?IsMin@TimeDelta@base@v8@@QEBA_NXZ (thunk -> +0x1ed40)
    Lifted::alias(0x7001dc6a70, fn_7001dce080 as *const (), 0, 0xa080), // +0x2a70 ??0BoundedPageAllocator@base@v8@@QEAA@PEAVPageAllocator@2@_K11@Z (thunk -> +0xa080)
    Lifted::alias(0x7001dc6a80, fn_7001dd08f0 as *const (), 1, 0xc8f0), // +0x2a80 ??8?$MagicNumbersForDivision@I@base@v8@@QEBA_NAEBU012@@Z (thunk -> +0xc8f0)
    Lifted::alias(0x7001dc6aa0, fn_7001dec7d0 as *const (), 2, 0x287d0), // +0x2aa0 sub_2aa0 (thunk -> +0x287d0)
    Lifted::alias(0x7001dc6ac0, tpl_25 as *const (), 1, 0xcae0), // +0x2ac0 ?GetRealStackAddressForSlot@Stack@base@v8@@SA?AUStackSlot@123@U4123@@Z (thunk -> +0xcae0)
    Lifted::alias(0x7001dc6ae0, fn_7001de36e0 as *const (), 1, 0x1f6e0), // +0x2ae0 ?InMilliseconds@TimeDelta@base@v8@@QEBA_JXZ (thunk -> +0x1f6e0)
    Lifted::alias(0x7001dc6b00, tpl_24::<0xfffffffffffffff8, 0x7001de2110, 0x7001de2030> as *const (), 0, 0xca50), // +0x2b00 sub_2b00 (thunk -> +0xca50)
    Lifted::alias(0x7001dc6b20, tpl_65::<0x7001df6ac0, 0x7001df75d0> as *const (), 3, 0x1f860), // +0x2b20 ?InSecondsF@TimeDelta@base@v8@@QEBANXZ (thunk -> +0x1f860)
    Lifted::alias(0x7001dc6b30, fn_7001de6ef0 as *const (), 0, 0x22ef0), // +0x2b30 sub_2b30 (thunk -> +0x22ef0)
    Lifted::alias(0x7001dc6b50, tpl_15 as *const (), 1, 0x22780), // +0x2b50 ??4SysInfo@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x22780)
    Lifted::alias(0x7001dc6b60, fn_7001ddc5f0 as *const (), 1, 0x185f0), // +0x2b60 ??0Bignum@base@v8@@QEAA@XZ (thunk -> +0x185f0)
    Lifted::alias(0x7001dc6b70, fn_7001de2ad0 as *const (), 1, 0x1ead0), // +0x2b70 ?FromMilliseconds@TimeDelta@base@v8@@SA?AV123@_J@Z (thunk -> +0x1ead0)
    Lifted::alias(0x7001dc6b80, fn_7001dce6b0 as *const (), 0, 0xa6b0), // +0x2b80 ?NextBool@RandomNumberGenerator@base@v8@@QEAA_NXZ (thunk -> +0xa6b0)
    Lifted::alias(0x7001dc6b90, fn_7001dea060 as *const (), 2, 0x26060), // +0x2b90 ?GetRandomMmapAddr@PageAllocator@base@v8@@UEAAPEAXXZ (thunk -> +0x26060)
    Lifted::alias(0x7001dc6bb0, tpl_25 as *const (), 1, 0x1e470), // +0x2bb0 ??4?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEAAAEAVTime@23@V423@@Z (thunk -> +0x1e470)
    Lifted::alias(0x7001dc6bc0, fn_7001dd3c30 as *const (), 2, 0xfc30), // +0x2bc0 ?cos@ieee754@base@v8@@YANN@Z (thunk -> +0xfc30)
    Lifted::alias(0x7001dc6bd0, fn_7001dde580 as *const (), 0, 0x1a580), // +0x2bd0 ?Square@Bignum@base@v8@@QEAAXXZ (thunk -> +0x1a580)
    Lifted::alias(0x7001dc6be0, fn_7001dd0690 as *const (), 1, 0xc690), // +0x2be0 ??$UnsignedDivisionByConstant@_K@base@v8@@YA?AU?$MagicNumbersForDivision@_K@01@_KI@Z (thunk -> +0xc690)
    Lifted::alias(0x7001dc6bf0, fn_7001dd42b0 as *const (), 2, 0x102b0), // +0x2bf0 ?cosh@ieee754@base@v8@@YANN@Z (thunk -> +0x102b0)
    Lifted::alias(0x7001dc6c00, fn_7001dd0270 as *const (), 1, 0xc270), // +0x2c00 sub_2c00 (thunk -> +0xc270)
    Lifted::alias(0x7001dc6c10, tpl_6::<0x7001dd9550> as *const (), 0, 0x15510), // +0x2c10 ??$PrintCheckOperand@PEAD@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAD@Z (thunk -> +0x15510)
    Lifted::alias(0x7001dc6c20, fn_7001de5870 as *const (), 0, 0x21870), // +0x2c20 ?Split@RegionAllocator@base@v8@@AEAAPEAVRegion@123@PEAV4123@_K@Z (thunk -> +0x21870)
    Lifted::alias(0x7001dc6c30, fn_7001de2730 as *const (), 1, 0x1e730), // +0x2c30 ??HTimeDelta@base@v8@@QEBA?AV012@AEBV012@@Z (thunk -> +0x1e730)
    Lifted::alias(0x7001dc6c40, fn_7001ddc5e0 as *const (), 0, 0x185e0), // +0x2c40 ?Times10@Bignum@base@v8@@QEAAXXZ (thunk -> +0x185e0)
    Lifted::alias(0x7001dc6c50, fn_7001dd01f0 as *const (), 1, 0xc1f0), // +0x2c50 sub_2c50 (thunk -> +0xc1f0)
    Lifted::alias(0x7001dc6c60, fn_7001ddda70 as *const (), 0, 0x19a70), // +0x2c60 ?DivideModuloIntBignum@Bignum@base@v8@@QEAAGAEBV123@@Z (thunk -> +0x19a70)
    Lifted::alias(0x7001dc6c70, tpl_50 as *const (), 1, 0x1e7b0), // +0x2c70 ??N?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NVThreadTicks@23@@Z (thunk -> +0x1e7b0)
    Lifted::alias(0x7001dc6c80, fn_7001de9b40 as *const (), 0, 0x25b40), // +0x2c80 ?CreateTimezoneCache@OS@base@v8@@SAPEAVTimezoneCache@23@PEAVTimeZoneProvider@3@@Z (thunk -> +0x25b40)
    Lifted::alias(0x7001dc6c90, fn_7001dcdf50 as *const (), 1, 0x9f50), // +0x2c90 ?SignedMod32@bits@base@v8@@YAHHH@Z (thunk -> +0x9f50)
    Lifted::alias(0x7001dc6ca0, tpl_17::<0x5b> as *const (), 1, 0xb0d0), // +0x2ca0 ?has_non_stop_time_stamp_counter@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb0d0)
    Lifted::alias(0x7001dc6cb0, fn_7001dec1c0 as *const (), 0, 0x281c0), // +0x2cb0 sub_2cb0 (thunk -> +0x281c0)
    Lifted::alias(0x7001dc6cc0, tpl_15 as *const (), 1, 0x1efe0), // +0x2cc0 ?native_handle@ConditionVariable@base@v8@@QEBAAEBUV8_CONDITION_VARIABLE@@XZ (thunk -> +0x1efe0)
    Lifted::alias(0x7001dc6cd0, fn_7001debaf0 as *const (), 0, 0x27af0), // +0x2cd0 ?SNPrintF@OS@base@v8@@SAHPEADHPEBDZZ (thunk -> +0x27af0)
    Lifted::alias(0x7001dc6ce0, fn_7001debd60 as *const (), 0, 0x27d60), // +0x2ce0 ?Sleep@OS@base@v8@@SAXVTimeDelta@23@@Z (thunk -> +0x27d60)
    Lifted::alias(0x7001dc6cf0, tpl_49 as *const (), 1, 0x1e780), // +0x2cf0 ??M?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NVTime@23@@Z (thunk -> +0x1e780)
    Lifted::alias(0x7001dc6d00, fn_7001dce1a0 as *const (), 3, 0xa1a0), // +0x2d00 ??4RandomNumberGenerator@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0xa1a0)
    Lifted::alias(0x7001dc6d10, fn_7001de4260 as *const (), 2, 0x20260), // +0x2d10 ?WaitUntilInitializedWin@ThreadTicks@base@v8@@CAXXZ (thunk -> +0x20260)
    Lifted::alias(0x7001dc6d30, tpl_9 as *const (), 1, 0xa530), // +0x2d30 sub_2d30 (thunk -> +0xa530)
    Lifted::alias(0x7001dc6d40, fn_7001de4760 as *const (), 2, 0x20760), // +0x2d40 sub_2d40 (thunk -> +0x20760)
    Lifted::alias(0x7001dc6d60, tpl_60 as *const (), 1, 0x1ee40), // +0x2d60 ?Min@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@SA?AVTime@34@XZ (thunk -> +0x1ee40)
    Lifted::alias(0x7001dc6d70, fn_7001dd0b40 as *const (), 0, 0xcb40), // +0x2d70 ?RelativePath@base@v8@@YA?AV?$unique_ptr@$$BY0A@DU?$default_delete@$$BY0A@D@std@@@std@@PEBD0@Z (thunk -> +0xcb40)
    Lifted::alias(0x7001dc6d80, fn_7001de73f0 as *const (), 3, 0x233f0), // +0x2d80 ?NextDouble@RandomNumberGenerator@base@v8@@QEAANXZ (thunk -> +0x233f0)
    Lifted::alias(0x7001dc6da0, tpl_17::<0x46> as *const (), 1, 0xb120), // +0x2da0 ?has_sse3@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb120)
    Lifted::alias(0x7001dc6dc0, fn_7001dd0880 as *const (), 1, 0xc880), // +0x2dc0 ??4?$MagicNumbersForDivision@I@base@v8@@QEAAAEAU012@$$QEAU012@@Z (thunk -> +0xc880)
    Lifted::alias(0x7001dc6dd0, tpl_10 as *const (), 1, 0xa560), // +0x2dd0 ?AssertHeldAndUnmark@Mutex@base@v8@@AEAAXXZ (thunk -> +0xa560)
    Lifted::alias(0x7001dc6de0, fn_7001de8d70 as *const (), 2, 0x24d70), // +0x2de0 sub_2de0 (thunk -> +0x24d70)
    Lifted::alias(0x7001dc6df0, fn_7001dd8c90 as *const (), 0, 0x14c90), // +0x2df0 ??$PrintCheckOperand@E@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@E@Z (thunk -> +0x14c90)
    Lifted::alias(0x7001dc6e00, tpl_9 as *const (), 1, 0x1e1c0), // +0x2e00 ?AllocateSharedPages@PageAllocator@base@v8@@UEAA?AV?$unique_ptr@VSharedMemory@PageAllocator@v8@@U?$default_delete@VShare (thunk -> +0x1e1c0)
    Lifted::alias(0x7001dc6e20, tpl_12::<0x20, 0x40> as *const (), 0, 0xa880), // +0x2e20 ?SetPermissions@BoundedPageAllocator@base@v8@@UEAA_NPEAX_KW4Permission@PageAllocator@3@@Z (thunk -> +0xa880)
    Lifted::alias(0x7001dc6e30, fn_7001de3440 as *const (), 1, 0x1f440), // +0x2e30 ?FromJsTime@Time@base@v8@@SA?AV123@N@Z (thunk -> +0x1f440)
    Lifted::alias(0x7001dc6e40, tpl_20::<0xffffffffffffff68, 0x4, 0x7001df5f18, 0xffffffffffffff64, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a378, 0x7001e0a360, 0x7001de2030> as *const (), 0, 0xb5f0), // +0x2e40 sub_2e40 (thunk -> +0xb5f0)
    Lifted::alias(0x7001dc6e80, tpl_14 as *const (), 1, 0x1ef20), // +0x2e80 ?ToInternalValue@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_JXZ (thunk -> +0x1ef20)
    Lifted::alias(0x7001dc6e90, tpl_47 as *const (), 1, 0x1e640), // +0x2e90 ??G?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA?AVTimeDelta@23@VTimeTicks@23@@Z (thunk -> +0x1e640)
    Lifted::alias(0x7001dc6ea0, fn_7001ddd7d0 as *const (), 1, 0x197d0), // +0x2ea0 ?BigitAt@Bignum@base@v8@@AEBAIH@Z (thunk -> +0x197d0)
    Lifted::alias(0x7001dc6eb0, fn_7001de8350 as *const (), 0, 0x24350), // +0x2eb0 ?SetSeed@RandomNumberGenerator@base@v8@@QEAAX_J@Z (thunk -> +0x24350)
    Lifted::alias(0x7001dc6ed0, fn_7001de2680 as *const (), 1, 0x1e680), // +0x2ed0 ??GTimeDelta@base@v8@@QEBA?AV012@AEBV012@@Z (thunk -> +0x1e680)
    Lifted::alias(0x7001dc6ee0, fn_7001dce690 as *const (), 0, 0xa690), // +0x2ee0 ?FreeRegion@RegionAllocator@base@v8@@QEAA_K_K@Z (thunk -> +0xa690)
    Lifted::alias(0x7001dc6ef0, tpl_6::<0x7001dc7b40> as *const (), 0, 0x15790), // +0x2ef0 ??$PrintCheckOperand@PEBD@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBD@Z (thunk -> +0x15790)
    Lifted::alias(0x7001dc6f00, tpl_35::<0x7001e074e8> as *const (), 2, 0x1e210), // +0x2f00 ?Free@base@v8@@YAXPEAX@Z (thunk -> +0x1e210)
    Lifted::alias(0x7001dc6f10, fn_7001dd0d30 as *const (), 1, 0xcd30), // +0x2f10 ?hash_combine@base@v8@@YA_K_K0@Z (thunk -> +0xcd30)
    Lifted::alias(0x7001dc6f20, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a408, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x14cb0), // +0x2f20 ??$PrintCheckOperand@H@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@H@Z (thunk -> +0x14cb0)
    Lifted::alias(0x7001dc6f30, tpl_12::<0x8, 0x38> as *const (), 0, 0x226b0), // +0x2f30 ?ReleasePages@LsanPageAllocator@base@v8@@UEAA_NPEAX_K1@Z (thunk -> +0x226b0)
    Lifted::new(0x7001dc6f50, tpl_1::<0x7001de9ef0> as *const (), 2), // +0x2f50 ?GetCurrentThreadId@OS@base@v8@@SAHXZ
    Lifted::alias(0x7001dc6f60, fn_7001dde990 as *const (), 0, 0x1a990), // +0x2f60 ?SubtractTimes@Bignum@base@v8@@AEAAXAEBV123@H@Z (thunk -> +0x1a990)
    Lifted::alias(0x7001dc6f70, fn_7001de3060 as *const (), 0, 0x1f060), // +0x2f70 ??1Mutex@base@v8@@QEAA@XZ (thunk -> +0x1f060)
    Lifted::alias(0x7001dc6f80, fn_7001ddd660 as *const (), 1, 0x19660), // +0x2f80 ?AssignUInt16@Bignum@base@v8@@QEAAXG@Z (thunk -> +0x19660)
    Lifted::alias(0x7001dc6f90, fn_7001dcf340 as *const (), 0, 0xb340), // +0x2f90 sub_2f90 (thunk -> +0xb340)
    Lifted::alias(0x7001dc6fa0, tpl_41::<0x7001e0a0b8> as *const (), 0, 0x1f030), // +0x2fa0 ??0RecursiveMutex@base@v8@@QEAA@XZ (thunk -> +0x1f030)
    Lifted::alias(0x7001dc6fb0, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a410, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x14ec0), // +0x2fb0 ??$PrintCheckOperand@I@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@I@Z (thunk -> +0x14ec0)
    Lifted::alias(0x7001dc6fc0, fn_7001dcfc40 as *const (), 0, 0xbc40), // +0x2fc0 sub_2fc0 (thunk -> +0xbc40)
    Lifted::alias(0x7001dc6fe0, fn_7001dd6c30 as *const (), 0, 0x12c30), // +0x2fe0 sub_2fe0 (thunk -> +0x12c30)
    Lifted::alias(0x7001dc6ff0, fn_7001de37b0 as *const (), 1, 0x1f7b0), // +0x2ff0 ?InMinutes@TimeDelta@base@v8@@QEBAHXZ (thunk -> +0x1f7b0)
    Lifted::new(0x7001dc7000, tpl_1::<0x7001de3070> as *const (), 2), // +0x3000 ??1RecursiveMutex@base@v8@@QEAA@XZ
    Lifted::alias(0x7001dc7010, fn_7001dcf730 as *const (), 1, 0xb730), // +0x3010 ?Addresses@StackTrace@debug@base@v8@@QEBAPEBQEBXPEA_K@Z (thunk -> +0xb730)
    Lifted::alias(0x7001dc7030, fn_7001de2870 as *const (), 1, 0x1e870), // +0x3030 ??XTimeDelta@base@v8@@QEAAAEAV012@_J@Z (thunk -> +0x1e870)
    Lifted::new(0x7001dc7050, tpl_1::<0x7001de9cb0> as *const (), 2), // +0x3050 ?DeleteThreadLocalKey@Thread@base@v8@@SAXH@Z
    Lifted::alias(0x7001dc7060, tpl_43 as *const (), 1, 0x1e460), // +0x3060 ??4?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEAAAEAV0123@AEBV0123@@Z (thunk -> +0x1e460)
    Lifted::alias(0x7001dc7080, tpl_15 as *const (), 1, 0x1efd0), // +0x3080 ?native_handle@ConditionVariable@base@v8@@QEAAAEAUV8_CONDITION_VARIABLE@@XZ (thunk -> +0x1efd0)
    Lifted::alias(0x7001dc7090, fn_7001dd5400 as *const (), 3, 0x11400), // +0x3090 ?log@ieee754@base@v8@@YANN@Z (thunk -> +0x11400)
    Lifted::alias(0x7001dc70a0, fn_7001de2b90 as *const (), 1, 0x1eb90), // +0x30a0 ?FromNanoseconds@TimeDelta@base@v8@@SA?AV123@_J@Z (thunk -> +0x1eb90)
    Lifted::alias(0x7001dc70b0, fn_7001dcf980 as *const (), 0, 0xb980), // +0x30b0 sub_30b0 (thunk -> +0xb980)
    Lifted::alias(0x7001dc70c0, fn_7001ddef30 as *const (), 3, 0x1af30), // +0x30c0 sub_30c0 (thunk -> +0x1af30)
    Lifted::alias(0x7001dc70d0, fn_7001dea9b0 as *const (), 0, 0x269b0), // +0x30d0 ?Join@Thread@base@v8@@QEAAXXZ (thunk -> +0x269b0)
    Lifted::alias(0x7001dc7100, fn_7001de3220 as *const (), 0, 0x1f220), // +0x3100 ?WaitFor@Semaphore@base@v8@@QEAA_NAEBVTimeDelta@23@@Z (thunk -> +0x1f220)
    Lifted::alias(0x7001dc7110, tpl_25 as *const (), 1, 0x1e350), // +0x3110 ??0?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@IEAA@_J@Z (thunk -> +0x1e350)
    Lifted::alias(0x7001dc7120, fn_7001de31d0 as *const (), 0, 0x1f1d0), // +0x3120 ?Signal@Semaphore@base@v8@@QEAAXXZ (thunk -> +0x1f1d0)
    Lifted::alias(0x7001dc7140, fn_7001de9ca0 as *const (), 0, 0x25ca0), // +0x3140 ?DebugBreak@OS@base@v8@@SAXXZ (thunk -> +0x25ca0)
    Lifted::alias(0x7001dc7150, fn_7001dd4d50 as *const (), 3, 0x10d50), // +0x3150 ?log1p@ieee754@base@v8@@YANN@Z (thunk -> +0x10d50)
    Lifted::alias(0x7001dc7160, fn_7001de2190 as *const (), 0, 0x1e190), // +0x3160 ?AllocatePages@PageAllocator@base@v8@@UEAAPEAXPEAX_K1W4Permission@13@@Z (thunk -> +0x1e190)
    Lifted::alias(0x7001dc7170, fn_7001dce970 as *const (), 1, 0xa970), // +0x3170 ?contains@RegionAllocator@base@v8@@QEBA_N_K@Z (thunk -> +0xa970)
    Lifted::alias(0x7001dc7180, fn_7001dd68b0 as *const (), 2, 0x128b0), // +0x3180 ?sinh@ieee754@base@v8@@YANN@Z (thunk -> +0x128b0)
    Lifted::alias(0x7001dc7190, tpl_15 as *const (), 1, 0xca20), // +0x3190 ??4MemoryMappedFile@OS@base@v8@@QEAAAEAV0123@AEBV0123@@Z (thunk -> +0xca20)
    Lifted::alias(0x7001dc71a0, tpl_29::<0x7001e070b8, 0x7001dc7b40, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd8331> as *const (), 2, 0x14020), // +0x31a0 ??$MakeCheckOpString@PEBXPEBX@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBX0PEBD@Z (thunk -> +0x14020)
    Lifted::alias(0x7001dc71c0, fn_7001dd5730 as *const (), 2, 0x11730), // +0x31c0 ?pow@ieee754@base@v8@@YANNN@Z (thunk -> +0x11730)
    Lifted::alias(0x7001dc71d0, tpl_17::<0x53> as *const (), 1, 0xb070), // +0x31d0 ?has_idiva@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb070)
    Lifted::alias(0x7001dc71e0, tpl_62::<0x7001e0a0a8> as *const (), 0, 0x1f0d0), // +0x31e0 ?TryLock@Mutex@base@v8@@QEAA_NXZ (thunk -> +0x1f0d0)
    Lifted::alias(0x7001dc71f0, fn_7001dd08c0 as *const (), 1, 0xc8c0), // +0x31f0 ??4?$MagicNumbersForDivision@_K@base@v8@@QEAAAEAU012@$$QEAU012@@Z (thunk -> +0xc8c0)
    Lifted::alias(0x7001dc7200, tpl_16::<0x1c> as *const (), 1, 0xaff0), // +0x3200 ?family@CPU@base@v8@@QEBAHXZ (thunk -> +0xaff0)
    Lifted::alias(0x7001dc7210, fn_7001dd2c60 as *const (), 2, 0xec60), // +0x3210 ?acosh@ieee754@base@v8@@YANN@Z (thunk -> +0xec60)
    Lifted::new(0x7001dc7220, tpl_1::<0x7001de30a0> as *const (), 2), // +0x3220 ?Lock@RecursiveMutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc7230, fn_7001de48d0 as *const (), 2, 0x208d0), // +0x3230 ??0RegionAllocator@base@v8@@QEAA@_K00@Z (thunk -> +0x208d0)
    Lifted::alias(0x7001dc7240, fn_7001debf30 as *const (), 0, 0x27f30), // +0x3240 ?VPrintError@OS@base@v8@@SAXPEBDPEAD@Z (thunk -> +0x27f30)
    Lifted::alias(0x7001dc7280, tpl_43 as *const (), 1, 0x1e4c0), // +0x3280 ??4ThreadTicks@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0x1e4c0)
    Lifted::alias(0x7001dc72a0, tpl_7::<0x7001de2030> as *const (), 0, 0x25650), // +0x32a0 sub_32a0 (thunk -> +0x25650)
    Lifted::alias(0x7001dc72b0, tpl_48::<0x7001dc6500> as *const (), 0, 0x1e700), // +0x32b0 ??H?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA?AVTimeTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e700)
    Lifted::alias(0x7001dc72c0, tpl_43 as *const (), 1, 0x1e450), // +0x32c0 ??4?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEAAAEAV0123@$$QEAV0123@@Z (thunk -> +0x1e450)
    Lifted::alias(0x7001dc72d0, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0, 0xb6d0), // +0x32d0 sub_32d0 (thunk -> +0xb6d0)
    Lifted::alias(0x7001dc72e0, tpl_52 as *const (), 1, 0x1e830), // +0x32e0 ??P?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NVThreadTicks@23@@Z (thunk -> +0x1e830)
    Lifted::alias(0x7001dc7310, fn_7001ddcbc0 as *const (), 0, 0x18bc0), // +0x3310 ?AddUInt64@Bignum@base@v8@@QEAAX_K@Z (thunk -> +0x18bc0)
    Lifted::alias(0x7001dc7320, fn_7001de29e0 as *const (), 1, 0x1e9e0), // +0x3320 ?FromDays@TimeDelta@base@v8@@SA?AV123@H@Z (thunk -> +0x1e9e0)
    Lifted::alias(0x7001dc7330, fn_7001de74a0 as *const (), 1, 0x234a0), // +0x3330 ?NextInt@RandomNumberGenerator@base@v8@@QEAAHH@Z (thunk -> +0x234a0)
    Lifted::alias(0x7001dc7350, fn_7001dcf9d0 as *const (), 0, 0xb9d0), // +0x3350 sub_3350 (thunk -> +0xb9d0)
    Lifted::alias(0x7001dc7360, fn_7001de9af0 as *const (), 0, 0x25af0), // +0x3360 ?CommitPageSize@OS@base@v8@@CA_KXZ (thunk -> +0x25af0)
    Lifted::alias(0x7001dc7370, fn_7001de2240 as *const (), 1, 0x1e240), // +0x3370 ?InitializeV8AllocationFunctions@script@cohtml@@YAXP6APEAX_K@ZP6APEAXPEAX0@ZP6APEAX00@ZP6AX2@ZP6APEADPEBD@Z4515@Z (thunk -> +0x1e240)
    Lifted::alias(0x7001dc7380, tpl_20::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2030> as *const (), 0, 0x17210), // +0x3380 sub_3380 (thunk -> +0x17210)
    Lifted::alias(0x7001dc7390, fn_7001de5450 as *const (), 0, 0x21450), // +0x3390 sub_3390 (thunk -> +0x21450)
    Lifted::alias(0x7001dc73a0, tpl_38::<0x7001dc5590> as *const (), 0, 0x185c0), // +0x33a0 ?PlusLessEqual@Bignum@base@v8@@SA_NAEBV123@00@Z (thunk -> +0x185c0)
    Lifted::alias(0x7001dc73c0, tpl_12::<0x8, 0x60> as *const (), 0, 0x22670), // +0x33c0 ?CanAllocateSharedPages@LsanPageAllocator@base@v8@@UEAA_NXZ (thunk -> +0x22670)
    Lifted::alias(0x7001dc73d0, fn_7001dd3310 as *const (), 2, 0xf310), // +0x33d0 ?atan2@ieee754@base@v8@@YANNN@Z (thunk -> +0xf310)
    Lifted::alias(0x7001dc73e0, fn_7001dce8d0 as *const (), 1, 0xa8d0), // +0x33e0 ?XorShift128@RandomNumberGenerator@base@v8@@SAXPEA_K0@Z (thunk -> +0xa8d0)
    Lifted::alias(0x7001dc73f0, tpl_35::<0x7001e074c8> as *const (), 2, 0x1e050), // +0x33f0 sub_33f0 (thunk -> +0x1e050)
    Lifted::alias(0x7001dc7400, fn_7001de3190 as *const (), 0, 0x1f190), // +0x3400 ??0Semaphore@base@v8@@QEAA@H@Z (thunk -> +0x1f190)
    Lifted::alias(0x7001dc7420, tpl_36::<0x7001dc5540> as *const (), 0, 0x18050), // +0x3420 ?Equal@Bignum@base@v8@@SA_NAEBV123@0@Z (thunk -> +0x18050)
    Lifted::alias(0x7001dc7430, fn_7001dec0f0 as *const (), 0, 0x280f0), // +0x3430 ?VSNPrintF@OS@base@v8@@SAHPEADHPEBD0@Z (thunk -> +0x280f0)
    Lifted::alias(0x7001dc7440, tpl_12::<0x8, 0x40> as *const (), 0, 0x226c0), // +0x3440 ?SetPermissions@LsanPageAllocator@base@v8@@UEAA_NPEAX_KW4Permission@PageAllocator@3@@Z (thunk -> +0x226c0)
    Lifted::alias(0x7001dc7450, fn_7001de9cc0 as *const (), 1, 0x25cc0), // +0x3450 ?DirectorySeparator@OS@base@v8@@SADXZ (thunk -> +0x25cc0)
    Lifted::alias(0x7001dc7460, fn_7001de2690 as *const (), 1, 0x1e690), // +0x3460 ??GTimeDelta@base@v8@@QEBA?AV012@XZ (thunk -> +0x1e690)
    Lifted::alias(0x7001dc7470, fn_7001de2860 as *const (), 1, 0x1e860), // +0x3470 ??PTimeDelta@base@v8@@QEBA_NAEBV012@@Z (thunk -> +0x1e860)
    Lifted::alias(0x7001dc7480, fn_7001de5c40 as *const (), 0, 0x21c40), // +0x3480 sub_3480 (thunk -> +0x21c40)
    Lifted::alias(0x7001dc74a0, tpl_59 as *const (), 1, 0x1eda0), // +0x34a0 ?Max@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@SA?AVThreadTicks@34@XZ (thunk -> +0x1eda0)
    Lifted::alias(0x7001dc74c0, fn_7001de5220 as *const (), 0, 0x21220), // +0x34c0 ?FreeListRemoveRegion@RegionAllocator@base@v8@@AEAAXPEAVRegion@123@@Z (thunk -> +0x21220)
    Lifted::alias(0x7001dc74d0, tpl_7::<0x7001de2030> as *const (), 0, 0xa2f0), // +0x34d0 sub_34d0 (thunk -> +0xa2f0)
    Lifted::alias(0x7001dc74f0, fn_7001ddee30 as *const (), 1, 0x1ae30), // +0x34f0 ?Zero@Bignum@base@v8@@AEAAXXZ (thunk -> +0x1ae30)
    Lifted::alias(0x7001dc7500, fn_7001de3a00 as *const (), 0, 0x1fa00), // +0x3500 ?IsSupported@ThreadTicks@base@v8@@SA_NXZ (thunk -> +0x1fa00)
    Lifted::alias(0x7001dc7510, tpl_25 as *const (), 1, 0x1ea90), // +0x3510 ?FromInternalValue@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@SA?AVThreadTicks@34@_J@Z (thunk -> +0x1ea90)
    Lifted::alias(0x7001dc7520, tpl_10 as *const (), 1, 0x22560), // +0x3520 ??1LsanPageAllocator@base@v8@@UEAA@XZ (thunk -> +0x22560)
    Lifted::alias(0x7001dc7540, fn_7001dce7a0 as *const (), 0, 0xa7a0), // +0x3540 ?ReserveForSharedMemoryMapping@BoundedPageAllocator@base@v8@@UEAA_NPEAX_K@Z (thunk -> +0xa7a0)
    Lifted::alias(0x7001dc7550, tpl_25 as *const (), 1, 0x1e3f0), // +0x3550 ??0TimeTicks@base@v8@@AEAA@_J@Z (thunk -> +0x1e3f0)
    Lifted::alias(0x7001dc7560, fn_7001dcf750 as *const (), 2, 0xb750), // +0x3560 ?ToString@StackTrace@debug@base@v8@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ (thunk -> +0xb750)
    Lifted::alias(0x7001dc7570, fn_7001de91c0 as *const (), 0, 0x251c0), // +0x3570 sub_3570 (thunk -> +0x251c0)
    Lifted::alias(0x7001dc7590, fn_7001de8a40 as *const (), 0, 0x24a40), // +0x3590 sub_3590 (thunk -> +0x24a40)
    Lifted::alias(0x7001dc75a0, fn_7001de4260 as *const (), 2, 0x20260), // +0x35a0 ?WaitUntilInitialized@ThreadTicks@base@v8@@SAXXZ (thunk -> +0x20260)
    Lifted::alias(0x7001dc75b0, fn_7001dec5c0 as *const (), 0, 0x285c0), // +0x35b0 ?open@MemoryMappedFile@OS@base@v8@@SAPEAV1234@PEBDW4FileMode@1234@@Z (thunk -> +0x285c0)
    Lifted::alias(0x7001dc75d0, tpl_40 as *const (), 1, 0x22810), // +0x35d0 ?AmountOfVirtualMemory@SysInfo@base@v8@@SA_JXZ (thunk -> +0x22810)
    Lifted::alias(0x7001dc75e0, tpl_37::<0x7001dc5590> as *const (), 0, 0x185a0), // +0x35e0 ?PlusLess@Bignum@base@v8@@SA_NAEBV123@00@Z (thunk -> +0x185a0)
    Lifted::alias(0x7001dc75f0, tpl_8::<0x10> as *const (), 1, 0xa390), // +0x35f0 ?AllocatePageSize@BoundedPageAllocator@base@v8@@UEAA_KXZ (thunk -> +0xa390)
    Lifted::alias(0x7001dc7610, tpl_39::<0x7001e074c8, 0x7001de2037, 0x7001de2038, 0x7001de2039, 0x7001de203a, 0x7001de203b, 0x7001de203c, 0x7001de203d, 0x7001de203e, 0x7001de203f> as *const (), 0, 0x1e030), // +0x3610 sub_3610 (thunk -> +0x1e030)
    Lifted::alias(0x7001dc7620, tpl_16::<0x34> as *const (), 1, 0xb210), // +0x3620 ?part@CPU@base@v8@@QEBAHXZ (thunk -> +0xb210)
    Lifted::alias(0x7001dc7630, tpl_54::<0x7001dc5fa0> as *const (), 0, 0x1e920), // +0x3630 ??Z?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEAAAEAVThreadTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e920)
    Lifted::alias(0x7001dc7640, fn_7001de69c0 as *const (), 2, 0x229c0), // +0x3640 sub_3640 (thunk -> +0x229c0)
    Lifted::alias(0x7001dc7650, tpl_19::<0xfffffffffffffffc, 0x7001ddb210> as *const (), 0, 0x171f0), // +0x3650 sub_3650 (thunk -> +0x171f0)
    Lifted::alias(0x7001dc7660, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a420, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x152e0), // +0x3660 ??$PrintCheckOperand@K@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@K@Z (thunk -> +0x152e0)
    Lifted::alias(0x7001dc7670, tpl_25 as *const (), 1, 0x1e360), // +0x3670 ??0?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@IEAA@_J@Z (thunk -> +0x1e360)
    Lifted::alias(0x7001dc7680, fn_7001deb790 as *const (), 0, 0x27790), // +0x3680 sub_3680 (thunk -> +0x27790)
    Lifted::alias(0x7001dc7690, tpl_63::<0x7001e0a160> as *const (), 0, 0x27ad0), // +0x3690 ?Remove@OS@base@v8@@SA_NPEBD@Z (thunk -> +0x27ad0)
    Lifted::alias(0x7001dc76b0, fn_7001dcf5b0 as *const (), 0, 0xb5b0), // +0x36b0 sub_36b0 (thunk -> +0xb5b0)
    Lifted::alias(0x7001dc76c0, fn_7001de3630 as *const (), 1, 0x1f630), // +0x36c0 ?InDays@TimeDelta@base@v8@@QEBAHXZ (thunk -> +0x1f630)
    Lifted::alias(0x7001dc76d0, tpl_8::<0x30> as *const (), 1, 0xa9d0), // +0x36d0 ?page_size@RegionAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa9d0)
    Lifted::alias(0x7001dc76e0, fn_7001de3b80 as *const (), 0, 0x1fb80), // +0x36e0 ?Now@Time@base@v8@@SA?AV123@XZ (thunk -> +0x1fb80)
    Lifted::alias(0x7001dc7700, fn_7001ddd3c0 as *const (), 0, 0x193c0), // +0x3700 ?AssignPowerUInt16@Bignum@base@v8@@QEAAXGH@Z (thunk -> +0x193c0)
    Lifted::alias(0x7001dc7710, fn_7001de3670 as *const (), 1, 0x1f670), // +0x3710 ?InHours@TimeDelta@base@v8@@QEBAHXZ (thunk -> +0x1f670)
    Lifted::alias(0x7001dc7720, tpl_14 as *const (), 1, 0xa9a0), // +0x3720 ?initial_seed@RandomNumberGenerator@base@v8@@QEBA_JXZ (thunk -> +0xa9a0)
    Lifted::alias(0x7001dc7730, fn_7001dcede0 as *const (), 1, 0xade0), // +0x3730 ??4CPU@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0xade0)
    Lifted::alias(0x7001dc7740, tpl_8::<0x30> as *const (), 1, 0xa9e0), // +0x3740 ?size@BoundedPageAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa9e0)
    Lifted::alias(0x7001dc7750, fn_7001dee2a0 as *const (), 0, 0x2a2a0), // +0x3750 sub_3750 (thunk -> +0x2a2a0)
    Lifted::alias(0x7001dc7760, fn_7001ddefa0 as *const (), 1, 0x1afa0), // +0x3760 ?Multiply@DiyFp@base@v8@@QEAAXAEBV123@@Z (thunk -> +0x1afa0)
    Lifted::alias(0x7001dc7770, fn_7001dcfa10 as *const (), 0, 0xba10), // +0x3770 sub_3770 (thunk -> +0xba10)
    Lifted::alias(0x7001dc7780, tpl_10 as *const (), 1, 0xa540), // +0x3780 ?AssertHeld@Mutex@base@v8@@QEBAXXZ (thunk -> +0xa540)
    Lifted::alias(0x7001dc7790, tpl_8::<0x8> as *const (), 1, 0x1e180), // +0x3790 ?AllocatePageSize@PageAllocator@base@v8@@UEAA_KXZ (thunk -> +0x1e180)
    Lifted::alias(0x7001dc77a0, fn_7001de18c0 as *const (), 3, 0x1d8c0), // +0x37a0 sub_37a0 (thunk -> +0x1d8c0)
    Lifted::alias(0x7001dc77b0, tpl_20::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2030> as *const (), 0, 0x172c0), // +0x37b0 sub_37b0 (thunk -> +0x172c0)
    Lifted::alias(0x7001dc77d0, fn_7001de2060 as *const (), 0, 0x1e060), // +0x37d0 sub_37d0 (thunk -> +0x1e060)
    Lifted::alias(0x7001dc77e0, fn_7001dcdf70 as *const (), 1, 0x9f70), // +0x37e0 ?SignedMulHigh32@bits@base@v8@@YAHHH@Z (thunk -> +0x9f70)
    Lifted::alias(0x7001dc77f0, fn_7001de7460 as *const (), 1, 0x23460), // +0x37f0 ?NextInt64@RandomNumberGenerator@base@v8@@QEAA_JXZ (thunk -> +0x23460)
    Lifted::alias(0x7001dc7800, tpl_42 as *const (), 1, 0x1f050), // +0x3800 ??0SharedMutex@base@v8@@QEAA@XZ (thunk -> +0x1f050)
    Lifted::alias(0x7001dc7810, fn_7001de3760 as *const (), 1, 0x1f760), // +0x3810 ?InMillisecondsRoundedUp@TimeDelta@base@v8@@QEBA_JXZ (thunk -> +0x1f760)
    Lifted::alias(0x7001dc7820, tpl_65::<0x7001df6ac0, 0x7001df75c0> as *const (), 3, 0x1f720), // +0x3820 ?InMillisecondsF@TimeDelta@base@v8@@QEBANXZ (thunk -> +0x1f720)
    Lifted::alias(0x7001dc7840, tpl_15 as *const (), 1, 0x22770), // +0x3840 ??4SysInfo@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0x22770)
    Lifted::alias(0x7001dc7850, fn_7001de5090 as *const (), 2, 0x21090), // +0x3850 ?FreeListAddRegion@RegionAllocator@base@v8@@AEAAXPEAVRegion@123@@Z (thunk -> +0x21090)
    Lifted::alias(0x7001dc7860, fn_7001de8cc0 as *const (), 0, 0x24cc0), // +0x3860 sub_3860 (thunk -> +0x24cc0)
    Lifted::alias(0x7001dc7870, tpl_8::<0x28> as *const (), 1, 0xa910), // +0x3870 ?begin@BoundedPageAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa910)
    Lifted::alias(0x7001dc7880, fn_7001ddb8f0 as *const (), 1, 0x178f0), // +0x3880 sub_3880 (thunk -> +0x178f0)
    Lifted::alias(0x7001dc7890, tpl_66::<0x8, 0x7001dc7430> as *const (), 0, 0x22750), // +0x3890 ?VSNPrintF@base@v8@@YAHV?$Vector@D@12@PEBDPEAD@Z (thunk -> +0x22750)
    Lifted::alias(0x7001dc78a0, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a418, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x150d0), // +0x38a0 ??$PrintCheckOperand@J@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@J@Z (thunk -> +0x150d0)
    Lifted::alias(0x7001dc78b0, tpl_15 as *const (), 1, 0xca40), // +0x38b0 ??4Stack@base@v8@@QEAAAEAV012@AEBV012@@Z (thunk -> +0xca40)
    Lifted::alias(0x7001dc78c0, tpl_67::<0x7001df7910, 0x8> as *const (), 3, 0x24910), // +0x38c0 sub_38c0 (thunk -> +0x24910)
    Lifted::alias(0x7001dc78d0, fn_7001deca90 as *const (), 1, 0x28a90), // +0x38d0 ?DisableSignalStackDump@debug@base@v8@@YAXXZ (thunk -> +0x28a90)
    Lifted::alias(0x7001dc78e0, fn_7001dee220 as *const (), 0, 0x2a220), // +0x38e0 sub_38e0 (thunk -> +0x2a220)
    Lifted::alias(0x7001dc78f0, fn_7001ddd6e0 as *const (), 1, 0x196e0), // +0x38f0 ?AssignUInt64@Bignum@base@v8@@QEAAX_K@Z (thunk -> +0x196e0)
    Lifted::alias(0x7001dc7900, tpl_43 as *const (), 1, 0x1e420), // +0x3900 ??4?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEAAAEAV0123@$$QEAV0123@@Z (thunk -> +0x1e420)
    Lifted::alias(0x7001dc7910, fn_7001dd6ac0 as *const (), 2, 0x12ac0), // +0x3910 ?tanh@ieee754@base@v8@@YANN@Z (thunk -> +0x12ac0)
    Lifted::alias(0x7001dc7920, tpl_42 as *const (), 1, 0x1e400), // +0x3920 ??0TimeTicks@base@v8@@QEAA@XZ (thunk -> +0x1e400)
    Lifted::alias(0x7001dc7930, fn_7001de3550 as *const (), 2, 0x1f550), // +0x3930 ?GetForThread@ThreadTicks@base@v8@@SA?AV123@AEBQEAX@Z (thunk -> +0x1f550)
    Lifted::alias(0x7001dc7940, tpl_67::<0x7001df7910, 0x8> as *const (), 3, 0x24640), // +0x3940 sub_3940 (thunk -> +0x24640)
    Lifted::alias(0x7001dc7950, fn_7001de31c0 as *const (), 0, 0x1f1c0), // +0x3950 ??1Semaphore@base@v8@@QEAA@XZ (thunk -> +0x1f1c0)
    Lifted::alias(0x7001dc7960, fn_7001dee9d0 as *const (), 1, 0x2a9d0), // +0x3960 sub_3960 (thunk -> +0x2a9d0)
    Lifted::alias(0x7001dc7970, fn_7001ddc020 as *const (), 0, 0x18020), // +0x3970 ?EnsureCapacity@Bignum@base@v8@@AEAAXH@Z (thunk -> +0x18020)
    Lifted::alias(0x7001dc7980, fn_7001de1fc0 as *const (), 0, 0x1dfc0), // +0x3980 ??0PageAllocator@base@v8@@QEAA@XZ (thunk -> +0x1dfc0)
    Lifted::alias(0x7001dc7990, tpl_8::<0x20> as *const (), 1, 0x287b0), // +0x3990 sub_3990 (thunk -> +0x287b0)
    Lifted::alias(0x7001dc79a0, tpl_58 as *const (), 1, 0x1ed80), // +0x39a0 ?IsNull@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ed80)
    Lifted::alias(0x7001dc79b0, tpl_14 as *const (), 1, 0xa920), // +0x39b0 ?begin@RegionAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa920)
    Lifted::alias(0x7001dc79c0, fn_7001de4dc0 as *const (), 0, 0x20dc0), // +0x39c0 ?AllocateRegion@RegionAllocator@base@v8@@QEAA_K_K@Z (thunk -> +0x20dc0)
    Lifted::alias(0x7001dc79d0, fn_7001dcfea0 as *const (), 1, 0xbea0), // +0x39d0 sub_39d0 (thunk -> +0xbea0)
    Lifted::new(0x7001dc79e0, tpl_1::<0x7001de3170> as *const (), 2), // +0x39e0 ?UnlockExclusive@SharedMutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc79f0, tpl_17::<0x55> as *const (), 1, 0xb170), // +0x39f0 ?has_thumb2@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb170)
    Lifted::alias(0x7001dc7a00, fn_7001deb800 as *const (), 2, 0x27800), // +0x3a00 sub_3a00 (thunk -> +0x27800)
    Lifted::alias(0x7001dc7a10, tpl_43 as *const (), 1, 0x1e4d0), // +0x3a10 ??4Time@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0x1e4d0)
    Lifted::alias(0x7001dc7a20, fn_7001de6450 as *const (), 1, 0x22450), // +0x3a20 sub_3a20 (thunk -> +0x22450)
    Lifted::alias(0x7001dc7a40, tpl_27 as *const (), 1, 0xcd60), // +0x3a40 ?hash_value@base@v8@@YA_KI@Z (thunk -> +0xcd60)
    Lifted::alias(0x7001dc7a50, fn_7001de3f40 as *const (), 2, 0x1ff40), // +0x3a50 ?TSCTicksPerSecond@ThreadTicks@base@v8@@CANXZ (thunk -> +0x1ff40)
    Lifted::alias(0x7001dc7a60, tpl_6::<0x7001dc7bc0> as *const (), 0, 0x28a30), // +0x3a60 ??0StackTrace@debug@base@v8@@QEAA@PEBU_CONTEXT@@@Z (thunk -> +0x28a30)
    Lifted::alias(0x7001dc7a70, tpl_46::<0x7001dc5fa0> as *const (), 0, 0x1e5c0), // +0x3a70 ??G?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA?AVThreadTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e5c0)
    Lifted::alias(0x7001dc7a90, tpl_10 as *const (), 1, 0xa580), // +0x3a90 ?AssertUnheldAndMark@Mutex@base@v8@@AEAAXXZ (thunk -> +0xa580)
    Lifted::alias(0x7001dc7ab0, fn_7001dea420 as *const (), 0, 0x26420), // +0x3ab0 ?GetStackStart@Stack@base@v8@@SA?AUStackSlot@123@XZ (thunk -> +0x26420)
    Lifted::alias(0x7001dc7ac0, tpl_16::<0x18> as *const (), 1, 0xafe0), // +0x3ac0 ?ext_model@CPU@base@v8@@QEBAHXZ (thunk -> +0xafe0)
    Lifted::alias(0x7001dc7ae0, fn_7001ddf050 as *const (), 2, 0x1b050), // +0x3ae0 ?DoubleToAscii@base@v8@@YAXNW4DtoaMode@12@HV?$Vector@D@12@PEAH22@Z (thunk -> +0x1b050)
    Lifted::alias(0x7001dc7af0, fn_7001de6790 as *const (), 0, 0x22790), // +0x3af0 ?AmountOfPhysicalMemory@SysInfo@base@v8@@SA_JXZ (thunk -> +0x22790)
    Lifted::alias(0x7001dc7b00, tpl_16::<0x10> as *const (), 1, 0xb220), // +0x3b00 ?stepping@CPU@base@v8@@QEBAHXZ (thunk -> +0xb220)
    Lifted::alias(0x7001dc7b10, tpl_8::<0x28> as *const (), 1, 0xa990), // +0x3b10 ?free_size@RegionAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa990)
    Lifted::alias(0x7001dc7b20, fn_7001dd0840 as *const (), 1, 0xc840), // +0x3b20 ??0?$MagicNumbersForDivision@I@base@v8@@QEAA@II_N@Z (thunk -> +0xc840)
    Lifted::alias(0x7001dc7b30, fn_7001dd0e30 as *const (), 2, 0xce30), // +0x3b30 sub_3b30 (thunk -> +0xce30)
    Lifted::alias(0x7001dc7b40, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a438, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x157d0), // +0x3b40 ??$PrintCheckOperand@PEBX@base@v8@@YA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBX@Z (thunk -> +0x157d0)
    Lifted::alias(0x7001dc7b50, fn_7001de5020 as *const (), 1, 0x21020), // +0x3b50 ?FindRegion@RegionAllocator@base@v8@@AEAA?AV?$_Tree_const_iterator@V?$_Tree_val@U?$_Tree_simple_types@PEAVRegion@RegionA (thunk -> +0x21020)
    Lifted::alias(0x7001dc7b60, tpl_23::<0x7001df5f58> as *const (), 1, 0xc9f0), // +0x3b60 ??0MemoryMappedFile@OS@base@v8@@QEAA@XZ (thunk -> +0xc9f0)
    Lifted::alias(0x7001dc7b70, tpl_45 as *const (), 1, 0x1e590), // +0x3b70 ??9?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NVTimeTicks@23@@Z (thunk -> +0x1e590)
    Lifted::alias(0x7001dc7b80, fn_7001dcdde0 as *const (), 1, 0x9de0), // +0x3b80 ?RoundUpToPowerOfTwo32@bits@base@v8@@YAII@Z (thunk -> +0x9de0)
    Lifted::alias(0x7001dc7b90, fn_7001de6600 as *const (), 0, 0x22600), // +0x3b90 ?AllocatePages@LsanPageAllocator@base@v8@@UEAAPEAXPEAX_K1W4Permission@PageAllocator@3@@Z (thunk -> +0x22600)
    Lifted::alias(0x7001dc7ba0, fn_7001de27e0 as *const (), 1, 0x1e7e0), // +0x3ba0 ??NTimeDelta@base@v8@@QEBA_NAEBV012@@Z (thunk -> +0x1e7e0)
    Lifted::alias(0x7001dc7bb0, tpl_37::<0x7001dc5540> as *const (), 0, 0x18540), // +0x3bb0 ?Less@Bignum@base@v8@@SA_NAEBV123@0@Z (thunk -> +0x18540)
    Lifted::alias(0x7001dc7bc0, fn_7001decae0 as *const (), 2, 0x28ae0), // +0x3bc0 ?InitTrace@StackTrace@debug@base@v8@@AEAAXPEBU_CONTEXT@@@Z (thunk -> +0x28ae0)
    Lifted::alias(0x7001dc7be0, fn_7001dce180 as *const (), 1, 0xa180), // +0x3be0 ??4RandomNumberGenerator@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0xa180)
    Lifted::alias(0x7001dc7bf0, fn_7001dcea00 as *const (), 2, 0xaa00), // +0x3bf0 ??0CPU@base@v8@@QEAA@XZ (thunk -> +0xaa00)
    Lifted::alias(0x7001dc7c00, tpl_53::<0x7001dc6500> as *const (), 0, 0x1e880), // +0x3c00 ??Y?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEAAAEAVThreadTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e880)
    Lifted::alias(0x7001dc7c10, tpl_43 as *const (), 1, 0x1e510), // +0x3c10 ??4TimeTicks@base@v8@@QEAAAEAV012@$$QEAV012@@Z (thunk -> +0x1e510)
    Lifted::alias(0x7001dc7c20, tpl_56 as *const (), 1, 0x1eca0), // +0x3c20 ?IsMax@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1eca0)
    Lifted::alias(0x7001dc7c30, fn_7001de1ef0 as *const (), 0, 0x1def0), // +0x3c30 ?CallOnceImpl@base@v8@@YAXPEAU?$atomic@E@std@@V?$function@$$A6AXXZ@4@@Z (thunk -> +0x1def0)
    Lifted::alias(0x7001dc7c40, fn_7001dec250 as *const (), 0, 0x28250), // +0x3c40 sub_3c40 (thunk -> +0x28250)
    Lifted::alias(0x7001dc7c50, tpl_15 as *const (), 1, 0xcd10), // +0x3c50 ?native_handle@Semaphore@base@v8@@QEAAAEAPEAXXZ (thunk -> +0xcd10)
    Lifted::alias(0x7001dc7c60, tpl_10 as *const (), 1, 0x1e000), // +0x3c60 ??1PageAllocator@base@v8@@UEAA@XZ (thunk -> +0x1e000)
    Lifted::alias(0x7001dc7c70, fn_7001deda80 as *const (), 0, 0x29a80), // +0x3c70 sub_3c70 (thunk -> +0x29a80)
    Lifted::alias(0x7001dc7c80, tpl_15 as *const (), 1, 0xa9b0), // +0x3c80 ?native_handle@Mutex@base@v8@@QEAAAEAUV8_SRWLOCK@@XZ (thunk -> +0xa9b0)
    Lifted::alias(0x7001dc7c90, fn_7001de3340 as *const (), 2, 0x1f340), // +0x3c90 ??6base@v8@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@std@@AEAV23@AEBVTime@01@@Z (thunk -> +0x1f340)
    Lifted::alias(0x7001dc7ca0, tpl_53::<0x7001dc6500> as *const (), 0, 0x1e8e0), // +0x3ca0 ??Y?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEAAAEAVTimeTicks@23@VTimeDelta@23@@Z (thunk -> +0x1e8e0)
    Lifted::alias(0x7001dc7cc0, tpl_17::<0x4f> as *const (), 1, 0xb020), // +0x3cc0 ?has_bmi1@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb020)
    Lifted::alias(0x7001dc7cd0, fn_7001ddb800 as *const (), 0, 0x17800), // +0x3cd0 ?V8_Fatal@@YAXPEBDZZ (thunk -> +0x17800)
    Lifted::alias(0x7001dc7ce0, tpl_17::<0x4a> as *const (), 1, 0xb1d0), // +0x3ce0 ?is_atom@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb1d0)
    Lifted::alias(0x7001dc7d10, tpl_17::<0x4c> as *const (), 1, 0xb010), // +0x3d10 ?has_avx@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb010)
    Lifted::new(0x7001dc7d20, tpl_1::<0x7001de3160> as *const (), 2), // +0x3d20 ?Unlock@RecursiveMutex@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc7d30, fn_7001dd6a30 as *const (), 2, 0x12a30), // +0x3d30 ?tan@ieee754@base@v8@@YANN@Z (thunk -> +0x12a30)
    Lifted::alias(0x7001dc7d40, tpl_68::<0x7001e07210> as *const (), 1, 0x24a10), // +0x3d40 sub_3d40 (thunk -> +0x24a10)
    Lifted::alias(0x7001dc7d50, tpl_56 as *const (), 1, 0x1ec60), // +0x3d50 ?IsMax@?$TimeBase@VThreadTicks@base@v8@@@time_internal@base@v8@@QEBA_NXZ (thunk -> +0x1ec60)
    Lifted::alias(0x7001dc7d60, fn_7001ded3c0 as *const (), 2, 0x293c0), // +0x3d60 ?OutputToStream@StackTrace@debug@base@v8@@QEBAXPEAV?$basic_ostream@DU?$char_traits@D@std@@@std@@@Z (thunk -> +0x293c0)
    Lifted::alias(0x7001dc7d80, fn_7001de82a0 as *const (), 2, 0x242a0), // +0x3d80 ?SetEntropySource@RandomNumberGenerator@base@v8@@SAXP6A_NPEAE_K@Z@Z (thunk -> +0x242a0)
    Lifted::alias(0x7001dc7da0, fn_7001ddb7e0 as *const (), 1, 0x177e0), // +0x3da0 ?SetPrintStackTrace@base@v8@@YAXP6AXXZ@Z (thunk -> +0x177e0)
    Lifted::alias(0x7001dc7db0, tpl_17::<0x4b> as *const (), 1, 0xb0e0), // +0x3db0 ?has_osxsave@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb0e0)
    Lifted::alias(0x7001dc7dc0, fn_7001dea060 as *const (), 2, 0x26060), // +0x3dc0 ?GetRandomMmapAddr@OS@base@v8@@CAPEAXXZ (thunk -> +0x26060)
    Lifted::alias(0x7001dc7de0, tpl_11 as *const (), 1, 0xa870), // +0x3de0 sub_3de0 (thunk -> +0xa870)
    Lifted::alias(0x7001dc7df0, tpl_12::<0x20, 0x48> as *const (), 0, 0xa5b0), // +0x3df0 ?DiscardSystemPages@BoundedPageAllocator@base@v8@@UEAA_NPEAX_K@Z (thunk -> +0xa5b0)
    Lifted::alias(0x7001dc7e00, fn_7001de9720 as *const (), 0, 0x25720), // +0x3e00 sub_3e00 (thunk -> +0x25720)
    Lifted::alias(0x7001dc7e10, fn_7001debba0 as *const (), 2, 0x27ba0), // +0x3e10 ?SetRandomMmapSeed@OS@base@v8@@CAX_J@Z (thunk -> +0x27ba0)
    Lifted::alias(0x7001dc7e20, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0, 0xb700), // +0x3e20 sub_3e20 (thunk -> +0xb700)
    Lifted::alias(0x7001dc7e30, fn_7001dde0b0 as *const (), 0, 0x1a0b0), // +0x3e30 ?MultiplyByUInt64@Bignum@base@v8@@QEAAX_K@Z (thunk -> +0x1a0b0)
    Lifted::alias(0x7001dc7e40, tpl_10 as *const (), 1, 0xa570), // +0x3e40 ?AssertUnheld@Mutex@base@v8@@QEBAXXZ (thunk -> +0xa570)
    Lifted::alias(0x7001dc7e50, fn_7001de36c0 as *const (), 1, 0x1f6c0), // +0x3e50 ?InMicroseconds@TimeDelta@base@v8@@QEBA_JXZ (thunk -> +0x1f6c0)
    Lifted::alias(0x7001dc7e70, tpl_10 as *const (), 1, 0xca10), // +0x3e70 ??1MemoryMappedFile@OS@base@v8@@UEAA@XZ (thunk -> +0xca10)
    Lifted::alias(0x7001dc7e80, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a438, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2, 0x15550), // +0x3e80 sub_3e80 (thunk -> +0x15550)
    Lifted::alias(0x7001dc7e90, fn_7001de63e0 as *const (), 1, 0x223e0), // +0x3e90 sub_3e90 (thunk -> +0x223e0)
    Lifted::alias(0x7001dc7ea0, tpl_50 as *const (), 1, 0x1e7c0), // +0x3ea0 ??N?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@QEBA_NVTime@23@@Z (thunk -> +0x1e7c0)
    Lifted::alias(0x7001dc7eb0, tpl_8::<0x8> as *const (), 1, 0xa9f0), // +0x3eb0 ?size@RegionAllocator@base@v8@@QEBA_KXZ (thunk -> +0xa9f0)
    Lifted::alias(0x7001dc7ed0, fn_7001de9dd0 as *const (), 0, 0x25dd0), // +0x3ed0 ?ExitProcess@OS@base@v8@@SAXH@Z (thunk -> +0x25dd0)
    Lifted::alias(0x7001dc7f00, tpl_8::<0x18> as *const (), 1, 0x285b0), // +0x3f00 sub_3f00 (thunk -> +0x285b0)
    Lifted::alias(0x7001dc7f10, tpl_13 as *const (), 1, 0xa5c0), // +0x3f10 sub_3f10 (thunk -> +0xa5c0)
    Lifted::alias(0x7001dc7f20, fn_7001de2b70 as *const (), 1, 0x1eb70), // +0x3f20 ?FromMinutes@TimeDelta@base@v8@@SA?AV123@H@Z (thunk -> +0x1eb70)
    Lifted::alias(0x7001dc7f30, fn_7001dd0de0 as *const (), 1, 0xcde0), // +0x3f30 ?hash_value@base@v8@@YA_K_K@Z (thunk -> +0xcde0)
    Lifted::alias(0x7001dc7f40, fn_7001de2ef0 as *const (), 3, 0x1eef0), // +0x3f40 ?TimesOf@TimeDelta@base@v8@@QEBANAEBV123@@Z (thunk -> +0x1eef0)
    Lifted::alias(0x7001dc7f50, fn_7001dec750 as *const (), 0, 0x28750), // +0x3f50 ?set_name@Thread@base@v8@@AEAAXPEBD@Z (thunk -> +0x28750)
    Lifted::alias(0x7001dc7f60, tpl_17::<0x44> as *const (), 1, 0xb150), // +0x3f60 ?has_sse@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb150)
    Lifted::alias(0x7001dc7f70, tpl_25 as *const (), 1, 0x1e3d0), // +0x3f70 ??0TimeDelta@base@v8@@AEAA@_J@Z (thunk -> +0x1e3d0)
    Lifted::alias(0x7001dc7f80, fn_7001ddcc50 as *const (), 0, 0x18c50), // +0x3f80 ?Align@Bignum@base@v8@@AEAAXAEBV123@@Z (thunk -> +0x18c50)
    Lifted::alias(0x7001dc7f90, tpl_12::<0x20, 0x18> as *const (), 0, 0xa890), // +0x3f90 ?SetRandomMmapSeed@BoundedPageAllocator@base@v8@@UEAAX_J@Z (thunk -> +0xa890)
    Lifted::alias(0x7001dc7fa0, tpl_25 as *const (), 1, 0x1eaa0), // +0x3fa0 ?FromInternalValue@?$TimeBase@VTime@base@v8@@@time_internal@base@v8@@SA?AVTime@34@_J@Z (thunk -> +0x1eaa0)
    Lifted::alias(0x7001dc7fb0, fn_7001de29c0 as *const (), 1, 0x1e9c0), // +0x3fb0 ??_0TimeDelta@base@v8@@QEAAAEAV012@_J@Z (thunk -> +0x1e9c0)
    Lifted::alias(0x7001dc7fc0, tpl_26::<0x10> as *const (), 1, 0xcd00), // +0x3fc0 ?name@Thread@base@v8@@QEBAPEBDXZ (thunk -> +0xcd00)
    Lifted::alias(0x7001dc7fd0, fn_7001dea500 as *const (), 1, 0x26500), // +0x3fd0 sub_3fd0 (thunk -> +0x26500)
    Lifted::alias(0x7001dc7fe0, tpl_11 as *const (), 1, 0xa590), // +0x3fe0 sub_3fe0 (thunk -> +0xa590)
    Lifted::alias(0x7001dc7ff0, tpl_58 as *const (), 1, 0x1ed90), // +0x3ff0 ?IsZero@TimeDelta@base@v8@@QEBA_NXZ (thunk -> +0x1ed90)
    Lifted::alias(0x7001dc8020, fn_7001ddd8d0 as *const (), 1, 0x198d0), // +0x4020 ?Clamp@Bignum@base@v8@@AEAAXXZ (thunk -> +0x198d0)
    Lifted::alias(0x7001dc8030, tpl_28::<0x7001e070b8, 0x7001dc6fb0, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd772e> as *const (), 2, 0x13420), // +0x4030 ??$MakeCheckOpString@II@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@IIPEBD@Z (thunk -> +0x13420)
    Lifted::alias(0x7001dc8040, tpl_28::<0x7001e070b8, 0x7001dc7660, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd7f2e> as *const (), 2, 0x13c20), // +0x4040 ??$MakeCheckOpString@KK@base@v8@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@KKPEBD@Z (thunk -> +0x13c20)
    Lifted::alias(0x7001dc8060, tpl_61 as *const (), 1, 0x1f010), // +0x4060 ?since_origin@?$TimeBase@VTimeTicks@base@v8@@@time_internal@base@v8@@QEBA?AVTimeDelta@34@XZ (thunk -> +0x1f010)
    Lifted::alias(0x7001dc8070, tpl_42 as *const (), 1, 0x1f020), // +0x4070 ??0Mutex@base@v8@@QEAA@XZ (thunk -> +0x1f020)
    Lifted::alias(0x7001dc8080, tpl_8::<0x10> as *const (), 1, 0x1e1f0), // +0x4080 ?CommitPageSize@PageAllocator@base@v8@@UEAA_KXZ (thunk -> +0x1e1f0)
    Lifted::alias(0x7001dc8090, fn_7001dde810 as *const (), 0, 0x1a810), // +0x4090 ?SubtractBignum@Bignum@base@v8@@QEAAXAEBV123@@Z (thunk -> +0x1a810)
    Lifted::alias(0x7001dc80b0, fn_7001dce940 as *const (), 1, 0xa940), // +0x40b0 ?contains@RegionAllocator@base@v8@@QEBA_N_K0@Z (thunk -> +0xa940)
    Lifted::alias(0x7001dc80c0, fn_7001de4540 as *const (), 0, 0x20540), // +0x40c0 sub_40c0 (thunk -> +0x20540)
    Lifted::alias(0x7001dc80d0, tpl_17::<0x4e> as *const (), 1, 0xb050), // +0x40d0 ?has_fma3@CPU@base@v8@@QEBA_NXZ (thunk -> +0xb050)
    Lifted::alias(0x7001dc80e0, fn_7001dcf320 as *const (), 1, 0xb320), // +0x40e0 sub_40e0 (thunk -> +0xb320)
    Lifted::alias(0x7001dc8110, fn_7001dd0510 as *const (), 1, 0xc510), // +0x4110 ??$UnsignedDivisionByConstant@I@base@v8@@YA?AU?$MagicNumbersForDivision@I@01@II@Z (thunk -> +0xc510)
    Lifted::alias(0x7001dc8120, fn_7001dce930 as *const (), 1, 0xa930), // +0x4120 ?contains@BoundedPageAllocator@base@v8@@QEBA_N_K@Z (thunk -> +0xa930)
    Lifted::alias(0x7001dc8130, tpl_16::<0x28> as *const (), 1, 0xb1c0), // +0x4130 ?implementer@CPU@base@v8@@QEBAHXZ (thunk -> +0xb1c0)
    Lifted::alias(0x7001dc8160, tpl_64::<0x7001e07090> as *const (), 0, 0x1f600), // +0x4160 ?HighResolutionNow@TimeTicks@base@v8@@SA?AV123@XZ (thunk -> +0x1f600)
    Lifted::alias(0x7001dc8180, fn_7001de7380 as *const (), 1, 0x23380), // +0x4180 ?NextBytes@RandomNumberGenerator@base@v8@@QEAAXPEAX_K@Z (thunk -> +0x23380)
    Lifted::alias(0x7001dc8190, fn_7001de25b0 as *const (), 1, 0x1e5b0), // +0x4190 ??DTimeDelta@base@v8@@QEBA?AV012@_J@Z (thunk -> +0x1e5b0)
    Lifted::new(0x7001dc81a0, tpl_1::<0x7001de9f00> as *const (), 2), // +0x41a0 ?GetLastError@OS@base@v8@@SAHXZ
    Lifted::new(0x7001dc81b0, tpl_1::<0x7001de9ed0> as *const (), 2), // +0x41b0 ?GetCurrentProcessId@OS@base@v8@@SAHXZ
    Lifted::alias(0x7001dc81c0, fn_7001dea450 as *const (), 0, 0x26450), // +0x41c0 ?GetUserTime@OS@base@v8@@SAHPEAI0@Z (thunk -> +0x26450)
    Lifted::new(0x7001dc81d0, tpl_1::<0x7001de2eb0> as *const (), 2), // +0x41d0 ?NotifyOne@ConditionVariable@base@v8@@QEAAXXZ
    Lifted::alias(0x7001dc81e0, tpl_55::<0x7001df75c0, 0x7001df75e0, 0x7001df75f0> as *const (), 3, 0x1eaf0), // +0x41e0 ?FromMillisecondsD@TimeDelta@base@v8@@SA?AV123@N@Z (thunk -> +0x1eaf0)
    Lifted::alias(0x7001dc81f0, fn_7001de41b0 as *const (), 1, 0x201b0), // +0x41f0 ?ToFiletime@Time@base@v8@@QEBA?AU_FILETIME@@XZ (thunk -> +0x201b0)
    Lifted::new(0x7001dcb3f0, tpl_5::<0x7001e0a608, 0x7001e074e0> as *const (), 1), // +0x73f0 sub_73f0
    Lifted::new(0x7001dcb410, tpl_5::<0x7001e0a600, 0x7001e074c8> as *const (), 1), // +0x7410 sub_7410
    Lifted::new(0x7001dcb430, tpl_5::<0x7001e0a600, 0x7001e074e8> as *const (), 1), // +0x7430 sub_7430
    Lifted::new(0x7001dcb450, tpl_5::<0x7001e0a618, 0x7001e074c0> as *const (), 1), // +0x7450 sub_7450
    Lifted::new(0x7001dcb470, tpl_5::<0x7001e0a618, 0x7001e074f0> as *const (), 1), // +0x7470 sub_7470
    Lifted::new(0x7001dcb490, tpl_5::<0x7001e0a620, 0x7001e074d0> as *const (), 1), // +0x7490 sub_7490
    Lifted::new(0x7001dcb4b0, tpl_5::<0x7001e0a8b8, 0x7001e074d8> as *const (), 1), // +0x74b0 sub_74b0
    Lifted::new(0x7001dcdde0, fn_7001dcdde0 as *const (), 1), // +0x9de0 sub_9de0
    Lifted::new(0x7001dcdec0, fn_7001dcdec0 as *const (), 1), // +0x9ec0 sub_9ec0
    Lifted::new(0x7001dcdf20, fn_7001dcdf20 as *const (), 1), // +0x9f20 sub_9f20
    Lifted::new(0x7001dcdf50, fn_7001dcdf50 as *const (), 1), // +0x9f50 sub_9f50
    Lifted::new(0x7001dcdf70, fn_7001dcdf70 as *const (), 1), // +0x9f70 sub_9f70
    Lifted::new(0x7001dcdf90, fn_7001dcdf90 as *const (), 1), // +0x9f90 sub_9f90
    Lifted::new(0x7001dcdfb0, fn_7001dcdfb0 as *const (), 1), // +0x9fb0 sub_9fb0
    Lifted::new(0x7001dcdfe0, fn_7001dcdfe0 as *const (), 1), // +0x9fe0 sub_9fe0
    Lifted::new(0x7001dce030, fn_7001dce030 as *const (), 1), // +0xa030 sub_a030
    Lifted::new(0x7001dce080, fn_7001dce080 as *const (), 0), // +0xa080 sub_a080
    Lifted::new(0x7001dce120, tpl_6::<0x7001dc6eb0> as *const (), 0), // +0xa120 sub_a120
    Lifted::new(0x7001dce140, fn_7001dce140 as *const (), 0), // +0xa140 sub_a140
    Lifted::new(0x7001dce180, fn_7001dce180 as *const (), 1), // +0xa180 sub_a180
    Lifted::new(0x7001dce1a0, fn_7001dce1a0 as *const (), 3), // +0xa1a0 sub_a1a0
    Lifted::new(0x7001dce1c0, fn_7001dce1c0 as *const (), 0), // +0xa1c0 sub_a1c0
    Lifted::new(0x7001dce2a0, fn_7001dce2a0 as *const (), 0), // +0xa2a0 sub_a2a0
    Lifted::new(0x7001dce2f0, tpl_7::<0x7001de2030> as *const (), 0), // +0xa2f0 sub_a2f0
    Lifted::new(0x7001dce320, fn_7001dce320 as *const (), 0), // +0xa320 sub_a320
    Lifted::new(0x7001dce390, tpl_8::<0x10> as *const (), 1), // +0xa390 sub_a390
    Lifted::new(0x7001dce3a0, fn_7001dce3a0 as *const (), 0), // +0xa3a0 sub_a3a0
    Lifted::new(0x7001dce460, fn_7001dce460 as *const (), 0), // +0xa460 sub_a460
    Lifted::new(0x7001dce530, tpl_9 as *const (), 1), // +0xa530 sub_a530
    Lifted::new(0x7001dce540, tpl_10 as *const (), 1), // +0xa540 sub_a540
    Lifted::new(0x7001dce550, tpl_10 as *const (), 1), // +0xa550 sub_a550
    Lifted::new(0x7001dce560, tpl_10 as *const (), 1), // +0xa560 sub_a560
    Lifted::new(0x7001dce570, tpl_10 as *const (), 1), // +0xa570 sub_a570
    Lifted::new(0x7001dce580, tpl_10 as *const (), 1), // +0xa580 sub_a580
    Lifted::new(0x7001dce590, tpl_11 as *const (), 1), // +0xa590 sub_a590
    Lifted::new(0x7001dce5a0, tpl_8::<0x18> as *const (), 1), // +0xa5a0 sub_a5a0
    Lifted::new(0x7001dce5b0, tpl_12::<0x20, 0x48> as *const (), 0), // +0xa5b0 sub_a5b0
    Lifted::new(0x7001dce5c0, tpl_13 as *const (), 1), // +0xa5c0 sub_a5c0
    Lifted::new(0x7001dce5d0, fn_7001dce5d0 as *const (), 0), // +0xa5d0 sub_a5d0
    Lifted::new(0x7001dce690, fn_7001dce690 as *const (), 0), // +0xa690 sub_a690
    Lifted::new(0x7001dce6a0, fn_7001dce6a0 as *const (), 0), // +0xa6a0 sub_a6a0
    Lifted::new(0x7001dce6b0, fn_7001dce6b0 as *const (), 0), // +0xa6b0 sub_a6b0
    Lifted::new(0x7001dce6d0, fn_7001dce6d0 as *const (), 0), // +0xa6d0 sub_a6d0
    Lifted::new(0x7001dce6e0, fn_7001dce6e0 as *const (), 0), // +0xa6e0 sub_a6e0
    Lifted::new(0x7001dce7a0, fn_7001dce7a0 as *const (), 0), // +0xa7a0 sub_a7a0
    Lifted::new(0x7001dce870, tpl_11 as *const (), 1), // +0xa870 sub_a870
    Lifted::new(0x7001dce880, tpl_12::<0x20, 0x40> as *const (), 0), // +0xa880 sub_a880
    Lifted::new(0x7001dce890, tpl_12::<0x20, 0x18> as *const (), 0), // +0xa890 sub_a890
    Lifted::new(0x7001dce8a0, fn_7001dce8a0 as *const (), 3), // +0xa8a0 sub_a8a0
    Lifted::new(0x7001dce8d0, fn_7001dce8d0 as *const (), 1), // +0xa8d0 sub_a8d0
    Lifted::new(0x7001dce910, tpl_8::<0x28> as *const (), 1), // +0xa910 sub_a910
    Lifted::new(0x7001dce920, tpl_14 as *const (), 1), // +0xa920 sub_a920
    Lifted::new(0x7001dce930, fn_7001dce930 as *const (), 1), // +0xa930 sub_a930
    Lifted::new(0x7001dce940, fn_7001dce940 as *const (), 1), // +0xa940 sub_a940
    Lifted::new(0x7001dce970, fn_7001dce970 as *const (), 1), // +0xa970 sub_a970
    Lifted::new(0x7001dce980, fn_7001dce980 as *const (), 1), // +0xa980 sub_a980
    Lifted::new(0x7001dce990, tpl_8::<0x28> as *const (), 1), // +0xa990 sub_a990
    Lifted::new(0x7001dce9a0, tpl_14 as *const (), 1), // +0xa9a0 sub_a9a0
    Lifted::new(0x7001dce9b0, tpl_15 as *const (), 1), // +0xa9b0 sub_a9b0
    Lifted::new(0x7001dce9c0, tpl_15 as *const (), 1), // +0xa9c0 sub_a9c0
    Lifted::new(0x7001dce9d0, tpl_8::<0x30> as *const (), 1), // +0xa9d0 sub_a9d0
    Lifted::new(0x7001dce9e0, tpl_8::<0x30> as *const (), 1), // +0xa9e0 sub_a9e0
    Lifted::new(0x7001dce9f0, tpl_8::<0x8> as *const (), 1), // +0xa9f0 sub_a9f0
    Lifted::new(0x7001dcea00, fn_7001dcea00 as *const (), 2), // +0xaa00 sub_aa00
    Lifted::new(0x7001dcede0, fn_7001dcede0 as *const (), 1), // +0xade0 sub_ade0
    Lifted::new(0x7001dcef70, fn_7001dcef70 as *const (), 3), // +0xaf70 sub_af70
    Lifted::new(0x7001dcefb0, tpl_16::<0x2c> as *const (), 1), // +0xafb0 sub_afb0
    Lifted::new(0x7001dcefc0, tpl_16::<0x3c> as *const (), 1), // +0xafc0 sub_afc0
    Lifted::new(0x7001dcefd0, tpl_16::<0x20> as *const (), 1), // +0xafd0 sub_afd0
    Lifted::new(0x7001dcefe0, tpl_16::<0x18> as *const (), 1), // +0xafe0 sub_afe0
    Lifted::new(0x7001dceff0, tpl_16::<0x1c> as *const (), 1), // +0xaff0 sub_aff0
    Lifted::new(0x7001dcf000, tpl_17::<0x4d> as *const (), 1), // +0xb000 sub_b000
    Lifted::new(0x7001dcf010, tpl_17::<0x4c> as *const (), 1), // +0xb010 sub_b010
    Lifted::new(0x7001dcf020, tpl_17::<0x4f> as *const (), 1), // +0xb020 sub_b020
    Lifted::new(0x7001dcf030, tpl_17::<0x50> as *const (), 1), // +0xb030 sub_b030
    Lifted::new(0x7001dcf040, tpl_17::<0x41> as *const (), 1), // +0xb040 sub_b040
    Lifted::new(0x7001dcf050, tpl_17::<0x4e> as *const (), 1), // +0xb050 sub_b050
    Lifted::new(0x7001dcf060, tpl_17::<0x40> as *const (), 1), // +0xb060 sub_b060
    Lifted::new(0x7001dcf070, tpl_17::<0x53> as *const (), 1), // +0xb070 sub_b070
    Lifted::new(0x7001dcf080, tpl_17::<0x59> as *const (), 1), // +0xb080 sub_b080
    Lifted::new(0x7001dcf090, tpl_17::<0x51> as *const (), 1), // +0xb090 sub_b090
    Lifted::new(0x7001dcf0a0, tpl_17::<0x43> as *const (), 1), // +0xb0a0 sub_b0a0
    Lifted::new(0x7001dcf0b0, tpl_17::<0x5d> as *const (), 1), // +0xb0b0 sub_b0b0
    Lifted::new(0x7001dcf0c0, tpl_17::<0x54> as *const (), 1), // +0xb0c0 sub_b0c0
    Lifted::new(0x7001dcf0d0, tpl_17::<0x5b> as *const (), 1), // +0xb0d0 sub_b0d0
    Lifted::new(0x7001dcf0e0, tpl_17::<0x4b> as *const (), 1), // +0xb0e0 sub_b0e0
    Lifted::new(0x7001dcf0f0, tpl_17::<0x52> as *const (), 1), // +0xb0f0 sub_b0f0
    Lifted::new(0x7001dcf100, tpl_17::<0x42> as *const (), 1), // +0xb100 sub_b100
    Lifted::new(0x7001dcf110, tpl_17::<0x45> as *const (), 1), // +0xb110 sub_b110
    Lifted::new(0x7001dcf120, tpl_17::<0x46> as *const (), 1), // +0xb120 sub_b120
    Lifted::new(0x7001dcf130, tpl_17::<0x48> as *const (), 1), // +0xb130 sub_b130
    Lifted::new(0x7001dcf140, tpl_17::<0x49> as *const (), 1), // +0xb140 sub_b140
    Lifted::new(0x7001dcf150, tpl_17::<0x44> as *const (), 1), // +0xb150 sub_b150
    Lifted::new(0x7001dcf160, tpl_17::<0x47> as *const (), 1), // +0xb160 sub_b160
    Lifted::new(0x7001dcf170, tpl_17::<0x55> as *const (), 1), // +0xb170 sub_b170
    Lifted::new(0x7001dcf180, tpl_17::<0x57> as *const (), 1), // +0xb180 sub_b180
    Lifted::new(0x7001dcf190, tpl_17::<0x58> as *const (), 1), // +0xb190 sub_b190
    Lifted::new(0x7001dcf1a0, tpl_17::<0x56> as *const (), 1), // +0xb1a0 sub_b1a0
    Lifted::new(0x7001dcf1b0, tpl_16::<0x38> as *const (), 1), // +0xb1b0 sub_b1b0
    Lifted::new(0x7001dcf1c0, tpl_16::<0x28> as *const (), 1), // +0xb1c0 sub_b1c0
    Lifted::new(0x7001dcf1d0, tpl_17::<0x4a> as *const (), 1), // +0xb1d0 sub_b1d0
    Lifted::new(0x7001dcf1e0, tpl_17::<0x5a> as *const (), 1), // +0xb1e0 sub_b1e0
    Lifted::new(0x7001dcf1f0, tpl_17::<0x5c> as *const (), 1), // +0xb1f0 sub_b1f0
    Lifted::new(0x7001dcf200, tpl_16::<0x14> as *const (), 1), // +0xb200 sub_b200
    Lifted::new(0x7001dcf210, tpl_16::<0x34> as *const (), 1), // +0xb210 sub_b210
    Lifted::new(0x7001dcf220, tpl_16::<0x10> as *const (), 1), // +0xb220 sub_b220
    Lifted::new(0x7001dcf230, tpl_16::<0x24> as *const (), 1), // +0xb230 sub_b230
    Lifted::new(0x7001dcf240, tpl_16::<0x30> as *const (), 1), // +0xb240 sub_b240
    Lifted::new(0x7001dcf250, tpl_15 as *const (), 1), // +0xb250 sub_b250
    Lifted::new(0x7001dcf260, fn_7001dcf260 as *const (), 0), // +0xb260 sub_b260
    Lifted::new(0x7001dcf2b0, fn_7001dcf2b0 as *const (), 0), // +0xb2b0 sub_b2b0
    Lifted::new(0x7001dcf320, fn_7001dcf320 as *const (), 1), // +0xb320 sub_b320
    Lifted::new(0x7001dcf340, fn_7001dcf340 as *const (), 0), // +0xb340 sub_b340
    Lifted::new(0x7001dcf430, tpl_18::<0xffffffffffffff68, 0x4, 0x7001df5f18, 0xffffffffffffff64, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a378> as *const (), 0), // +0xb430 sub_b430
    Lifted::new(0x7001dcf4a0, fn_7001dcf4a0 as *const (), 1), // +0xb4a0 sub_b4a0
    Lifted::new(0x7001dcf4d0, fn_7001dcf4d0 as *const (), 3), // +0xb4d0 sub_b4d0
    Lifted::new(0x7001dcf5a0, tpl_19::<0xfffffffffffffffc, 0x7001dcf5f0> as *const (), 0), // +0xb5a0 sub_b5a0
    Lifted::new(0x7001dcf5b0, fn_7001dcf5b0 as *const (), 0), // +0xb5b0 sub_b5b0
    Lifted::new(0x7001dcf5f0, tpl_20::<0xffffffffffffff68, 0x4, 0x7001df5f18, 0xffffffffffffff64, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a378, 0x7001e0a360, 0x7001de2030> as *const (), 0), // +0xb5f0 sub_b5f0
    Lifted::new(0x7001dcf6a0, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0), // +0xb6a0 sub_b6a0
    Lifted::new(0x7001dcf6d0, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0), // +0xb6d0 sub_b6d0
    Lifted::new(0x7001dcf700, tpl_21::<0x7001df5dd8, 0x7001de2030> as *const (), 0), // +0xb700 sub_b700
    Lifted::new(0x7001dcf730, fn_7001dcf730 as *const (), 1), // +0xb730 sub_b730
    Lifted::new(0x7001dcf750, fn_7001dcf750 as *const (), 2), // +0xb750 sub_b750
    Lifted::new(0x7001dcf940, fn_7001dcf940 as *const (), 0), // +0xb940 sub_b940
    Lifted::new(0x7001dcf970, tpl_10 as *const (), 1), // +0xb970 sub_b970
    Lifted::new(0x7001dcf980, fn_7001dcf980 as *const (), 0), // +0xb980 sub_b980
    Lifted::new(0x7001dcf9d0, fn_7001dcf9d0 as *const (), 0), // +0xb9d0 sub_b9d0
    Lifted::new(0x7001dcf9f0, tpl_22::<0x7001df5e70, 0x7001deddc0, 0x7001dcfa00> as *const (), 0), // +0xb9f0 sub_b9f0
    Lifted::new(0x7001dcfa10, fn_7001dcfa10 as *const (), 0), // +0xba10 sub_ba10
    Lifted::new(0x7001dcfa80, fn_7001dcfa80 as *const (), 0), // +0xba80 sub_ba80
    Lifted::new(0x7001dcfc40, fn_7001dcfc40 as *const (), 0), // +0xbc40 sub_bc40
    Lifted::new(0x7001dcfc90, fn_7001dcfc90 as *const (), 0), // +0xbc90 sub_bc90
    Lifted::new(0x7001dcfea0, fn_7001dcfea0 as *const (), 1), // +0xbea0 sub_bea0
    Lifted::new(0x7001dcff10, fn_7001dcff10 as *const (), 1), // +0xbf10 sub_bf10
    Lifted::new(0x7001dd00c0, fn_7001dd00c0 as *const (), 1), // +0xc0c0 sub_c0c0
    Lifted::new(0x7001dd01f0, fn_7001dd01f0 as *const (), 1), // +0xc1f0 sub_c1f0
    Lifted::new(0x7001dd0270, fn_7001dd0270 as *const (), 1), // +0xc270 sub_c270
    Lifted::new(0x7001dd0290, fn_7001dd0290 as *const (), 1), // +0xc290 sub_c290
    Lifted::new(0x7001dd03c0, fn_7001dd03c0 as *const (), 1), // +0xc3c0 sub_c3c0
    Lifted::new(0x7001dd0510, fn_7001dd0510 as *const (), 1), // +0xc510 sub_c510
    Lifted::new(0x7001dd0690, fn_7001dd0690 as *const (), 1), // +0xc690 sub_c690
    Lifted::new(0x7001dd0840, fn_7001dd0840 as *const (), 1), // +0xc840 sub_c840
    Lifted::new(0x7001dd0860, fn_7001dd0860 as *const (), 1), // +0xc860 sub_c860
    Lifted::new(0x7001dd0880, fn_7001dd0880 as *const (), 1), // +0xc880 sub_c880
    Lifted::new(0x7001dd08a0, fn_7001dd08a0 as *const (), 3), // +0xc8a0 sub_c8a0
    Lifted::new(0x7001dd08c0, fn_7001dd08c0 as *const (), 1), // +0xc8c0 sub_c8c0
    Lifted::new(0x7001dd08e0, fn_7001dd08e0 as *const (), 3), // +0xc8e0 sub_c8e0
    Lifted::new(0x7001dd08f0, fn_7001dd08f0 as *const (), 1), // +0xc8f0 sub_c8f0
    Lifted::new(0x7001dd0920, fn_7001dd0920 as *const (), 1), // +0xc920 sub_c920
    Lifted::new(0x7001dd0950, fn_7001dd0950 as *const (), 0), // +0xc950 sub_c950
    Lifted::new(0x7001dd09d0, tpl_23::<0x7001df5f58> as *const (), 1), // +0xc9d0 sub_c9d0
    Lifted::new(0x7001dd09f0, tpl_23::<0x7001df5f58> as *const (), 1), // +0xc9f0 sub_c9f0
    Lifted::new(0x7001dd0a10, tpl_10 as *const (), 1), // +0xca10 sub_ca10
    Lifted::new(0x7001dd0a20, tpl_15 as *const (), 1), // +0xca20 sub_ca20
    Lifted::new(0x7001dd0a30, tpl_15 as *const (), 1), // +0xca30 sub_ca30
    Lifted::new(0x7001dd0a40, tpl_15 as *const (), 1), // +0xca40 sub_ca40
    Lifted::new(0x7001dd0a50, tpl_24::<0xfffffffffffffff8, 0x7001de2110, 0x7001de2030> as *const (), 0), // +0xca50 sub_ca50
    Lifted::new(0x7001dd0aa0, tpl_7::<0x7001de2030> as *const (), 0), // +0xcaa0 sub_caa0
    Lifted::new(0x7001dd0ae0, tpl_25 as *const (), 1), // +0xcae0 sub_cae0
    Lifted::new(0x7001dd0af0, fn_7001dd0af0 as *const (), 0), // +0xcaf0 sub_caf0
    Lifted::new(0x7001dd0b10, fn_7001dd0b10 as *const (), 0), // +0xcb10 sub_cb10
    Lifted::new(0x7001dd0b40, fn_7001dd0b40 as *const (), 0), // +0xcb40 sub_cb40
    Lifted::new(0x7001dd0c50, fn_7001dd0c50 as *const (), 0), // +0xcc50 sub_cc50
    Lifted::new(0x7001dd0cf0, tpl_8::<0x8> as *const (), 1), // +0xccf0 sub_ccf0
    Lifted::new(0x7001dd0d00, tpl_26::<0x10> as *const (), 1), // +0xcd00 sub_cd00
    Lifted::new(0x7001dd0d10, tpl_15 as *const (), 1), // +0xcd10 sub_cd10
    Lifted::new(0x7001dd0d20, tpl_15 as *const (), 1), // +0xcd20 sub_cd20
    Lifted::new(0x7001dd0d30, fn_7001dd0d30 as *const (), 1), // +0xcd30 sub_cd30
    Lifted::new(0x7001dd0d60, tpl_27 as *const (), 1), // +0xcd60 sub_cd60
    Lifted::new(0x7001dd0da0, tpl_27 as *const (), 1), // +0xcda0 sub_cda0
    Lifted::new(0x7001dd0de0, fn_7001dd0de0 as *const (), 1), // +0xcde0 sub_cde0
    Lifted::new(0x7001dd0e30, fn_7001dd0e30 as *const (), 2), // +0xce30 sub_ce30
    Lifted::new(0x7001dd0f00, fn_7001dd0f00 as *const (), 2), // +0xcf00 sub_cf00
    Lifted::new(0x7001dd1350, fn_7001dd1350 as *const (), 2), // +0xd350 sub_d350
    Lifted::new(0x7001dd2510, fn_7001dd2510 as *const (), 3), // +0xe510 sub_e510
    Lifted::new(0x7001dd2870, fn_7001dd2870 as *const (), 2), // +0xe870 sub_e870
    Lifted::new(0x7001dd2c60, fn_7001dd2c60 as *const (), 2), // +0xec60 sub_ec60
    Lifted::new(0x7001dd2e10, fn_7001dd2e10 as *const (), 2), // +0xee10 sub_ee10
    Lifted::new(0x7001dd3160, fn_7001dd3160 as *const (), 2), // +0xf160 sub_f160
    Lifted::new(0x7001dd3310, fn_7001dd3310 as *const (), 2), // +0xf310 sub_f310
    Lifted::new(0x7001dd36d0, fn_7001dd36d0 as *const (), 3), // +0xf6d0 sub_f6d0
    Lifted::new(0x7001dd3930, fn_7001dd3930 as *const (), 2), // +0xf930 sub_f930
    Lifted::new(0x7001dd3aa0, fn_7001dd3aa0 as *const (), 3), // +0xfaa0 sub_faa0
    Lifted::new(0x7001dd3c30, fn_7001dd3c30 as *const (), 2), // +0xfc30 sub_fc30
    Lifted::new(0x7001dd42b0, fn_7001dd42b0 as *const (), 2), // +0x102b0 sub_102b0
    Lifted::new(0x7001dd4420, fn_7001dd4420 as *const (), 3), // +0x10420 sub_10420
    Lifted::new(0x7001dd4770, fn_7001dd4770 as *const (), 3), // +0x10770 sub_10770
    Lifted::new(0x7001dd4bf0, fn_7001dd4bf0 as *const (), 2), // +0x10bf0 sub_10bf0
    Lifted::new(0x7001dd4d50, fn_7001dd4d50 as *const (), 3), // +0x10d50 sub_10d50
    Lifted::new(0x7001dd5160, fn_7001dd5160 as *const (), 3), // +0x11160 sub_11160
    Lifted::new(0x7001dd5400, fn_7001dd5400 as *const (), 3), // +0x11400 sub_11400
    Lifted::new(0x7001dd5730, fn_7001dd5730 as *const (), 2), // +0x11730 sub_11730
    Lifted::new(0x7001dd6300, fn_7001dd6300 as *const (), 2), // +0x12300 sub_12300
    Lifted::new(0x7001dd68b0, fn_7001dd68b0 as *const (), 2), // +0x128b0 sub_128b0
    Lifted::new(0x7001dd6a30, fn_7001dd6a30 as *const (), 2), // +0x12a30 sub_12a30
    Lifted::new(0x7001dd6ac0, fn_7001dd6ac0 as *const (), 2), // +0x12ac0 sub_12ac0
    Lifted::new(0x7001dd6c30, fn_7001dd6c30 as *const (), 0), // +0x12c30 sub_12c30
    Lifted::new(0x7001dd6e00, fn_7001dd6e00 as *const (), 0), // +0x12e00 sub_12e00
    Lifted::new(0x7001dd7020, tpl_28::<0x7001e070b8, 0x7001dc6f20, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd732e> as *const (), 2), // +0x13020 sub_13020
    Lifted::new(0x7001dd7420, tpl_28::<0x7001e070b8, 0x7001dc6fb0, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd772e> as *const (), 2), // +0x13420 sub_13420
    Lifted::new(0x7001dd7820, tpl_28::<0x7001e070b8, 0x7001dc78a0, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd7b2e> as *const (), 2), // +0x13820 sub_13820
    Lifted::new(0x7001dd7c20, tpl_28::<0x7001e070b8, 0x7001dc7660, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd7f2e> as *const (), 2), // +0x13c20 sub_13c20
    Lifted::new(0x7001dd8020, tpl_29::<0x7001e070b8, 0x7001dc7b40, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd8331> as *const (), 2), // +0x14020 sub_14020
    Lifted::new(0x7001dd8430, tpl_29::<0x7001e070b8, 0x7001dc64b0, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd8741> as *const (), 2), // +0x14430 sub_14430
    Lifted::new(0x7001dd8840, tpl_29::<0x7001e070b8, 0x7001dc5b50, 0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001dd6e00, 0x7001df6d1c, 0x7001ddaef0, 0x7001df6d14, 0x7001df6d10, 0x7001df6d34, 0x7001df6d28, 0x7001df6d20, 0x7001de2010, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2050, 0x7001e0a720, 0x7001dd8b51> as *const (), 2), // +0x14840 sub_14840
    Lifted::new(0x7001dd8c50, tpl_30::<0x7001ddb390> as *const (), 0), // +0x14c50 sub_14c50
    Lifted::new(0x7001dd8c70, tpl_30::<0x7001ddb390> as *const (), 0), // +0x14c70 sub_14c70
    Lifted::new(0x7001dd8c90, fn_7001dd8c90 as *const (), 0), // +0x14c90 sub_14c90
    Lifted::new(0x7001dd8cb0, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a408, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x14cb0 sub_14cb0
    Lifted::new(0x7001dd8ec0, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a410, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x14ec0 sub_14ec0
    Lifted::new(0x7001dd90d0, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a418, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x150d0 sub_150d0
    Lifted::new(0x7001dd92e0, tpl_31::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a420, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x152e0 sub_152e0
    Lifted::new(0x7001dd94f0, tpl_6::<0x7001dd9550> as *const (), 0), // +0x154f0 sub_154f0
    Lifted::new(0x7001dd9510, tpl_6::<0x7001dd9550> as *const (), 0), // +0x15510 sub_15510
    Lifted::new(0x7001dd9530, tpl_6::<0x7001dd9550> as *const (), 0), // +0x15530 sub_15530
    Lifted::new(0x7001dd9550, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a438, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x15550 sub_15550
    Lifted::new(0x7001dd9770, tpl_6::<0x7001dc7b40> as *const (), 0), // +0x15770 sub_15770
    Lifted::new(0x7001dd9790, tpl_6::<0x7001dc7b40> as *const (), 0), // +0x15790 sub_15790
    Lifted::new(0x7001dd97b0, tpl_6::<0x7001dc7b40> as *const (), 0), // +0x157b0 sub_157b0
    Lifted::new(0x7001dd97d0, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a438, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x157d0 sub_157d0
    Lifted::new(0x7001dd99f0, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a428, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x159f0 sub_159f0
    Lifted::new(0x7001dd9c10, tpl_32::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a430, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x15c10 sub_15c10
    Lifted::new(0x7001dd9e30, tpl_33::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a408, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x15e30 sub_15e30
    Lifted::new(0x7001dda040, tpl_33::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a410, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16040 sub_16040
    Lifted::new(0x7001dda250, tpl_33::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a418, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16250 sub_16250
    Lifted::new(0x7001dda460, tpl_33::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a420, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16460 sub_16460
    Lifted::new(0x7001dda670, tpl_34::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a438, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16670 sub_16670
    Lifted::new(0x7001dda890, tpl_34::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a438, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16890 sub_16890
    Lifted::new(0x7001ddaab0, tpl_34::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a428, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16ab0 sub_16ab0
    Lifted::new(0x7001ddacd0, tpl_34::<0x7001df6c38, 0x7001e0a368, 0x7001e0a3e8, 0x7001df6c08, 0x7001e0a348, 0x7001df5e88, 0x7001df6c28, 0x7001e0a430, 0x7001dcfa80, 0x7001dcf340, 0x7001e0a3f0, 0x7001e0a360> as *const (), 2), // +0x16cd0 sub_16cd0
    Lifted::new(0x7001ddaef0, fn_7001ddaef0 as *const (), 0), // +0x16ef0 sub_16ef0
    Lifted::new(0x7001ddb0f0, fn_7001ddb0f0 as *const (), 0), // +0x170f0 sub_170f0
    Lifted::new(0x7001ddb180, tpl_18::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0> as *const (), 0), // +0x17180 sub_17180
    Lifted::new(0x7001ddb1f0, tpl_19::<0xfffffffffffffffc, 0x7001ddb210> as *const (), 0), // +0x171f0 sub_171f0
    Lifted::new(0x7001ddb200, tpl_19::<0xfffffffffffffffc, 0x7001ddb2c0> as *const (), 0), // +0x17200 sub_17200
    Lifted::new(0x7001ddb210, tpl_20::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2030> as *const (), 0), // +0x17210 sub_17210
    Lifted::new(0x7001ddb2c0, tpl_20::<0xffffffffffffff78, 0x4, 0x7001df6c08, 0xffffffffffffff74, 0x7001dcf340, 0xffffffffffffff88, 0x7001e0a3f0, 0x7001e0a360, 0x7001de2030> as *const (), 0), // +0x172c0 sub_172c0
    Lifted::new(0x7001ddb370, fn_7001ddb370 as *const (), 0), // +0x17370 sub_17370
    Lifted::new(0x7001ddb390, fn_7001ddb390 as *const (), 2), // +0x17390 sub_17390
    Lifted::new(0x7001ddb7c0, fn_7001ddb7c0 as *const (), 1), // +0x177c0 sub_177c0
    Lifted::new(0x7001ddb7e0, fn_7001ddb7e0 as *const (), 1), // +0x177e0 sub_177e0
    Lifted::new(0x7001ddb7f0, tpl_35::<0x7001e07070> as *const (), 2), // +0x177f0 sub_177f0
    Lifted::new(0x7001ddb800, fn_7001ddb800 as *const (), 0), // +0x17800 sub_17800
    Lifted::new(0x7001ddb8f0, fn_7001ddb8f0 as *const (), 1), // +0x178f0 sub_178f0
    Lifted::new(0x7001ddb910, fn_7001ddb910 as *const (), 1), // +0x17910 sub_17910
    Lifted::new(0x7001ddb920, fn_7001ddb920 as *const (), 2), // +0x17920 sub_17920
    Lifted::new(0x7001ddc020, fn_7001ddc020 as *const (), 0), // +0x18020 sub_18020
    Lifted::new(0x7001ddc050, tpl_36::<0x7001dc5540> as *const (), 0), // +0x18050 sub_18050
    Lifted::new(0x7001ddc070, fn_7001ddc070 as *const (), 2), // +0x18070 sub_18070
    Lifted::new(0x7001ddc540, tpl_37::<0x7001dc5540> as *const (), 0), // +0x18540 sub_18540
    Lifted::new(0x7001ddc560, tpl_38::<0x7001dc5540> as *const (), 0), // +0x18560 sub_18560
    Lifted::new(0x7001ddc580, tpl_36::<0x7001dc5590> as *const (), 0), // +0x18580 sub_18580
    Lifted::new(0x7001ddc5a0, tpl_37::<0x7001dc5590> as *const (), 0), // +0x185a0 sub_185a0
    Lifted::new(0x7001ddc5c0, tpl_38::<0x7001dc5590> as *const (), 0), // +0x185c0 sub_185c0
    Lifted::new(0x7001ddc5e0, fn_7001ddc5e0 as *const (), 0), // +0x185e0 sub_185e0
    Lifted::new(0x7001ddc5f0, fn_7001ddc5f0 as *const (), 1), // +0x185f0 sub_185f0
    Lifted::new(0x7001ddca70, fn_7001ddca70 as *const (), 0), // +0x18a70 sub_18a70
    Lifted::new(0x7001ddcbc0, fn_7001ddcbc0 as *const (), 0), // +0x18bc0 sub_18bc0
    Lifted::new(0x7001ddcc50, fn_7001ddcc50 as *const (), 0), // +0x18c50 sub_18c50
    Lifted::new(0x7001ddcd30, fn_7001ddcd30 as *const (), 1), // +0x18d30 sub_18d30
    Lifted::new(0x7001ddcde0, fn_7001ddcde0 as *const (), 2), // +0x18de0 sub_18de0
    Lifted::new(0x7001ddd180, fn_7001ddd180 as *const (), 0), // +0x19180 sub_19180
    Lifted::new(0x7001ddd3c0, fn_7001ddd3c0 as *const (), 0), // +0x193c0 sub_193c0
    Lifted::new(0x7001ddd660, fn_7001ddd660 as *const (), 1), // +0x19660 sub_19660
    Lifted::new(0x7001ddd6e0, fn_7001ddd6e0 as *const (), 1), // +0x196e0 sub_196e0
    Lifted::new(0x7001ddd7d0, fn_7001ddd7d0 as *const (), 1), // +0x197d0 sub_197d0
    Lifted::new(0x7001ddd810, fn_7001ddd810 as *const (), 1), // +0x19810 sub_19810
    Lifted::new(0x7001ddd8d0, fn_7001ddd8d0 as *const (), 1), // +0x198d0 sub_198d0
    Lifted::new(0x7001ddd920, fn_7001ddd920 as *const (), 1), // +0x19920 sub_19920
    Lifted::new(0x7001ddda70, fn_7001ddda70 as *const (), 0), // +0x19a70 sub_19a70
    Lifted::new(0x7001dddca0, fn_7001dddca0 as *const (), 1), // +0x19ca0 sub_19ca0
    Lifted::new(0x7001dddcd0, fn_7001dddcd0 as *const (), 0), // +0x19cd0 sub_19cd0
    Lifted::new(0x7001dddf80, fn_7001dddf80 as *const (), 0), // +0x19f80 sub_19f80
    Lifted::new(0x7001dde0b0, fn_7001dde0b0 as *const (), 0), // +0x1a0b0 sub_1a0b0
    Lifted::new(0x7001dde230, fn_7001dde230 as *const (), 1), // +0x1a230 sub_1a230
    Lifted::new(0x7001dde500, fn_7001dde500 as *const (), 0), // +0x1a500 sub_1a500
    Lifted::new(0x7001dde580, fn_7001dde580 as *const (), 0), // +0x1a580 sub_1a580
    Lifted::new(0x7001dde810, fn_7001dde810 as *const (), 0), // +0x1a810 sub_1a810
    Lifted::new(0x7001dde990, fn_7001dde990 as *const (), 0), // +0x1a990 sub_1a990
    Lifted::new(0x7001ddeb90, fn_7001ddeb90 as *const (), 1), // +0x1ab90 sub_1ab90
    Lifted::new(0x7001ddee30, fn_7001ddee30 as *const (), 1), // +0x1ae30 sub_1ae30
    Lifted::new(0x7001ddee80, fn_7001ddee80 as *const (), 2), // +0x1ae80 sub_1ae80
    Lifted::new(0x7001ddef30, fn_7001ddef30 as *const (), 3), // +0x1af30 sub_1af30
    Lifted::new(0x7001ddefa0, fn_7001ddefa0 as *const (), 1), // +0x1afa0 sub_1afa0
    Lifted::new(0x7001ddf050, fn_7001ddf050 as *const (), 2), // +0x1b050 sub_1b050
    Lifted::new(0x7001ddf270, fn_7001ddf270 as *const (), 0), // +0x1b270 sub_1b270
    Lifted::new(0x7001ddf480, fn_7001ddf480 as *const (), 2), // +0x1b480 sub_1b480
    Lifted::new(0x7001ddf7d0, fn_7001ddf7d0 as *const (), 2), // +0x1b7d0 sub_1b7d0
    Lifted::new(0x7001ddf9e0, fn_7001ddf9e0 as *const (), 2), // +0x1b9e0 sub_1b9e0
    Lifted::new(0x7001ddfe20, fn_7001ddfe20 as *const (), 1), // +0x1be20 sub_1be20
    Lifted::new(0x7001ddff30, fn_7001ddff30 as *const (), 2), // +0x1bf30 sub_1bf30
    Lifted::new(0x7001de02c0, fn_7001de02c0 as *const (), 1), // +0x1c2c0 sub_1c2c0
    Lifted::new(0x7001de0390, fn_7001de0390 as *const (), 2), // +0x1c390 sub_1c390
    Lifted::new(0x7001de09d0, fn_7001de09d0 as *const (), 1), // +0x1c9d0 sub_1c9d0
    Lifted::new(0x7001de0de0, fn_7001de0de0 as *const (), 3), // +0x1cde0 sub_1cde0
    Lifted::new(0x7001de1150, fn_7001de1150 as *const (), 2), // +0x1d150 sub_1d150
    Lifted::new(0x7001de15d0, fn_7001de15d0 as *const (), 3), // +0x1d5d0 sub_1d5d0
    Lifted::new(0x7001de18c0, fn_7001de18c0 as *const (), 3), // +0x1d8c0 sub_1d8c0
    Lifted::new(0x7001de1950, fn_7001de1950 as *const (), 2), // +0x1d950 sub_1d950
    Lifted::new(0x7001de1ef0, fn_7001de1ef0 as *const (), 0), // +0x1def0 sub_1def0
    Lifted::new(0x7001de1f90, fn_7001de1f90 as *const (), 1), // +0x1df90 sub_1df90
    Lifted::new(0x7001de1fc0, fn_7001de1fc0 as *const (), 0), // +0x1dfc0 sub_1dfc0
    Lifted::new(0x7001de2000, tpl_10 as *const (), 1), // +0x1e000 sub_1e000
    Lifted::new(0x7001de2010, tpl_39::<0x7001e074f0, 0x7001de2017, 0x7001de2018, 0x7001de2019, 0x7001de201a, 0x7001de201b, 0x7001de201c, 0x7001de201d, 0x7001de201e, 0x7001de201f> as *const (), 0), // +0x1e010 sub_1e010
    Lifted::new(0x7001de2030, tpl_39::<0x7001e074c8, 0x7001de2037, 0x7001de2038, 0x7001de2039, 0x7001de203a, 0x7001de203b, 0x7001de203c, 0x7001de203d, 0x7001de203e, 0x7001de203f> as *const (), 0), // +0x1e030 sub_1e030
    Lifted::new(0x7001de2050, tpl_35::<0x7001e074c8> as *const (), 2), // +0x1e050 sub_1e050
    Lifted::new(0x7001de2060, fn_7001de2060 as *const (), 0), // +0x1e060 sub_1e060
    Lifted::new(0x7001de20c0, fn_7001de20c0 as *const (), 0), // +0x1e0c0 sub_1e0c0
    Lifted::new(0x7001de20f0, tpl_39::<0x7001e074f0, 0x7001de20f7, 0x7001de20f8, 0x7001de20f9, 0x7001de20fa, 0x7001de20fb, 0x7001de20fc, 0x7001de20fd, 0x7001de20fe, 0x7001de20ff> as *const (), 0), // +0x1e0f0 sub_1e0f0
    Lifted::new(0x7001de2110, fn_7001de2110 as *const (), 0), // +0x1e110 sub_1e110
    Lifted::new(0x7001de2140, fn_7001de2140 as *const (), 0), // +0x1e140 sub_1e140
    Lifted::new(0x7001de2160, fn_7001de2160 as *const (), 0), // +0x1e160 sub_1e160
    Lifted::new(0x7001de2180, tpl_8::<0x8> as *const (), 1), // +0x1e180 sub_1e180
    Lifted::new(0x7001de2190, fn_7001de2190 as *const (), 0), // +0x1e190 sub_1e190
    Lifted::new(0x7001de21c0, tpl_9 as *const (), 1), // +0x1e1c0 sub_1e1c0
    Lifted::new(0x7001de21d0, tpl_35::<0x7001e074e0> as *const (), 2), // +0x1e1d0 sub_1e1d0
    Lifted::new(0x7001de21e0, tpl_11 as *const (), 1), // +0x1e1e0 sub_1e1e0
    Lifted::new(0x7001de21f0, tpl_8::<0x10> as *const (), 1), // +0x1e1f0 sub_1e1f0
    Lifted::new(0x7001de2200, fn_7001de2200 as *const (), 0), // +0x1e200 sub_1e200
    Lifted::new(0x7001de2210, tpl_35::<0x7001e074e8> as *const (), 2), // +0x1e210 sub_1e210
    Lifted::new(0x7001de2220, fn_7001de2220 as *const (), 0), // +0x1e220 sub_1e220
    Lifted::new(0x7001de2240, fn_7001de2240 as *const (), 1), // +0x1e240 sub_1e240
    Lifted::new(0x7001de22b0, tpl_35::<0x7001e074c0> as *const (), 2), // +0x1e2b0 sub_1e2b0
    Lifted::new(0x7001de22c0, tpl_35::<0x7001e074d0> as *const (), 2), // +0x1e2c0 sub_1e2c0
    Lifted::new(0x7001de22d0, fn_7001de22d0 as *const (), 0), // +0x1e2d0 sub_1e2d0
    Lifted::new(0x7001de22f0, tpl_40 as *const (), 1), // +0x1e2f0 sub_1e2f0
    Lifted::new(0x7001de2300, fn_7001de2300 as *const (), 0), // +0x1e300 sub_1e300
    Lifted::new(0x7001de2320, fn_7001de2320 as *const (), 0), // +0x1e320 sub_1e320
    Lifted::new(0x7001de2330, tpl_35::<0x7001e074d8> as *const (), 2), // +0x1e330 sub_1e330
    Lifted::new(0x7001de2340, tpl_25 as *const (), 1), // +0x1e340 sub_1e340
    Lifted::new(0x7001de2350, tpl_25 as *const (), 1), // +0x1e350 sub_1e350
    Lifted::new(0x7001de2360, tpl_25 as *const (), 1), // +0x1e360 sub_1e360
    Lifted::new(0x7001de2370, tpl_41::<0x7001e0a068> as *const (), 0), // +0x1e370 sub_1e370
    Lifted::new(0x7001de2390, tpl_25 as *const (), 1), // +0x1e390 sub_1e390
    Lifted::new(0x7001de23a0, tpl_42 as *const (), 1), // +0x1e3a0 sub_1e3a0
    Lifted::new(0x7001de23b0, tpl_25 as *const (), 1), // +0x1e3b0 sub_1e3b0
    Lifted::new(0x7001de23c0, tpl_42 as *const (), 1), // +0x1e3c0 sub_1e3c0
    Lifted::new(0x7001de23d0, tpl_25 as *const (), 1), // +0x1e3d0 sub_1e3d0
    Lifted::new(0x7001de23e0, tpl_42 as *const (), 1), // +0x1e3e0 sub_1e3e0
    Lifted::new(0x7001de23f0, tpl_25 as *const (), 1), // +0x1e3f0 sub_1e3f0
    Lifted::new(0x7001de2400, tpl_42 as *const (), 1), // +0x1e400 sub_1e400
    Lifted::new(0x7001de2410, tpl_10 as *const (), 1), // +0x1e410 sub_1e410
    Lifted::new(0x7001de2420, tpl_43 as *const (), 1), // +0x1e420 sub_1e420
    Lifted::new(0x7001de2430, tpl_43 as *const (), 1), // +0x1e430 sub_1e430
    Lifted::new(0x7001de2440, tpl_25 as *const (), 1), // +0x1e440 sub_1e440
    Lifted::new(0x7001de2450, tpl_43 as *const (), 1), // +0x1e450 sub_1e450
    Lifted::new(0x7001de2460, tpl_43 as *const (), 1), // +0x1e460 sub_1e460
    Lifted::new(0x7001de2470, tpl_25 as *const (), 1), // +0x1e470 sub_1e470
    Lifted::new(0x7001de2480, tpl_43 as *const (), 1), // +0x1e480 sub_1e480
    Lifted::new(0x7001de2490, tpl_43 as *const (), 1), // +0x1e490 sub_1e490
    Lifted::new(0x7001de24a0, tpl_25 as *const (), 1), // +0x1e4a0 sub_1e4a0
    Lifted::new(0x7001de24b0, tpl_43 as *const (), 1), // +0x1e4b0 sub_1e4b0
    Lifted::new(0x7001de24c0, tpl_43 as *const (), 1), // +0x1e4c0 sub_1e4c0
    Lifted::new(0x7001de24d0, tpl_43 as *const (), 1), // +0x1e4d0 sub_1e4d0
    Lifted::new(0x7001de24e0, tpl_43 as *const (), 1), // +0x1e4e0 sub_1e4e0
    Lifted::new(0x7001de24f0, tpl_43 as *const (), 1), // +0x1e4f0 sub_1e4f0
    Lifted::new(0x7001de2500, tpl_43 as *const (), 1), // +0x1e500 sub_1e500
    Lifted::new(0x7001de2510, tpl_43 as *const (), 1), // +0x1e510 sub_1e510
    Lifted::new(0x7001de2520, tpl_43 as *const (), 1), // +0x1e520 sub_1e520
    Lifted::new(0x7001de2530, tpl_44 as *const (), 1), // +0x1e530 sub_1e530
    Lifted::new(0x7001de2540, tpl_44 as *const (), 1), // +0x1e540 sub_1e540
    Lifted::new(0x7001de2550, tpl_44 as *const (), 1), // +0x1e550 sub_1e550
    Lifted::new(0x7001de2560, fn_7001de2560 as *const (), 1), // +0x1e560 sub_1e560
    Lifted::new(0x7001de2570, tpl_45 as *const (), 1), // +0x1e570 sub_1e570
    Lifted::new(0x7001de2580, tpl_45 as *const (), 1), // +0x1e580 sub_1e580
    Lifted::new(0x7001de2590, tpl_45 as *const (), 1), // +0x1e590 sub_1e590
    Lifted::new(0x7001de25a0, fn_7001de25a0 as *const (), 1), // +0x1e5a0 sub_1e5a0
    Lifted::new(0x7001de25b0, fn_7001de25b0 as *const (), 1), // +0x1e5b0 sub_1e5b0
    Lifted::new(0x7001de25c0, tpl_46::<0x7001dc5fa0> as *const (), 0), // +0x1e5c0 sub_1e5c0
    Lifted::new(0x7001de25f0, tpl_47 as *const (), 1), // +0x1e5f0 sub_1e5f0
    Lifted::new(0x7001de2600, tpl_46::<0x7001dc5fa0> as *const (), 0), // +0x1e600 sub_1e600
    Lifted::new(0x7001de2630, tpl_47 as *const (), 1), // +0x1e630 sub_1e630
    Lifted::new(0x7001de2640, tpl_47 as *const (), 1), // +0x1e640 sub_1e640
    Lifted::new(0x7001de2650, tpl_46::<0x7001dc5fa0> as *const (), 0), // +0x1e650 sub_1e650
    Lifted::new(0x7001de2680, fn_7001de2680 as *const (), 1), // +0x1e680 sub_1e680
    Lifted::new(0x7001de2690, fn_7001de2690 as *const (), 1), // +0x1e690 sub_1e690
    Lifted::new(0x7001de26a0, tpl_48::<0x7001dc6500> as *const (), 0), // +0x1e6a0 sub_1e6a0
    Lifted::new(0x7001de26d0, tpl_48::<0x7001dc6500> as *const (), 0), // +0x1e6d0 sub_1e6d0
    Lifted::new(0x7001de2700, tpl_48::<0x7001dc6500> as *const (), 0), // +0x1e700 sub_1e700
    Lifted::new(0x7001de2730, fn_7001de2730 as *const (), 1), // +0x1e730 sub_1e730
    Lifted::new(0x7001de2740, fn_7001de2740 as *const (), 1), // +0x1e740 sub_1e740
    Lifted::new(0x7001de2760, fn_7001de2760 as *const (), 1), // +0x1e760 sub_1e760
    Lifted::new(0x7001de2770, tpl_49 as *const (), 1), // +0x1e770 sub_1e770
    Lifted::new(0x7001de2780, tpl_49 as *const (), 1), // +0x1e780 sub_1e780
    Lifted::new(0x7001de2790, tpl_49 as *const (), 1), // +0x1e790 sub_1e790
    Lifted::new(0x7001de27a0, fn_7001de27a0 as *const (), 1), // +0x1e7a0 sub_1e7a0
    Lifted::new(0x7001de27b0, tpl_50 as *const (), 1), // +0x1e7b0 sub_1e7b0
    Lifted::new(0x7001de27c0, tpl_50 as *const (), 1), // +0x1e7c0 sub_1e7c0
    Lifted::new(0x7001de27d0, tpl_50 as *const (), 1), // +0x1e7d0 sub_1e7d0
    Lifted::new(0x7001de27e0, fn_7001de27e0 as *const (), 1), // +0x1e7e0 sub_1e7e0
    Lifted::new(0x7001de27f0, tpl_51 as *const (), 1), // +0x1e7f0 sub_1e7f0
    Lifted::new(0x7001de2800, tpl_51 as *const (), 1), // +0x1e800 sub_1e800
    Lifted::new(0x7001de2810, tpl_51 as *const (), 1), // +0x1e810 sub_1e810
    Lifted::new(0x7001de2820, fn_7001de2820 as *const (), 1), // +0x1e820 sub_1e820
    Lifted::new(0x7001de2830, tpl_52 as *const (), 1), // +0x1e830 sub_1e830
    Lifted::new(0x7001de2840, tpl_52 as *const (), 1), // +0x1e840 sub_1e840
    Lifted::new(0x7001de2850, tpl_52 as *const (), 1), // +0x1e850 sub_1e850
    Lifted::new(0x7001de2860, fn_7001de2860 as *const (), 1), // +0x1e860 sub_1e860
    Lifted::new(0x7001de2870, fn_7001de2870 as *const (), 1), // +0x1e870 sub_1e870
    Lifted::new(0x7001de2880, tpl_53::<0x7001dc6500> as *const (), 0), // +0x1e880 sub_1e880
    Lifted::new(0x7001de28b0, tpl_53::<0x7001dc6500> as *const (), 0), // +0x1e8b0 sub_1e8b0
    Lifted::new(0x7001de28e0, tpl_53::<0x7001dc6500> as *const (), 0), // +0x1e8e0 sub_1e8e0
    Lifted::new(0x7001de2910, fn_7001de2910 as *const (), 1), // +0x1e910 sub_1e910
    Lifted::new(0x7001de2920, tpl_54::<0x7001dc5fa0> as *const (), 0), // +0x1e920 sub_1e920
    Lifted::new(0x7001de2950, tpl_54::<0x7001dc5fa0> as *const (), 0), // +0x1e950 sub_1e950
    Lifted::new(0x7001de2980, tpl_54::<0x7001dc5fa0> as *const (), 0), // +0x1e980 sub_1e980
    Lifted::new(0x7001de29b0, fn_7001de29b0 as *const (), 1), // +0x1e9b0 sub_1e9b0
    Lifted::new(0x7001de29c0, fn_7001de29c0 as *const (), 1), // +0x1e9c0 sub_1e9c0
    Lifted::new(0x7001de29e0, fn_7001de29e0 as *const (), 1), // +0x1e9e0 sub_1e9e0
    Lifted::new(0x7001de2a00, fn_7001de2a00 as *const (), 3), // +0x1ea00 sub_1ea00
    Lifted::new(0x7001de2a70, fn_7001de2a70 as *const (), 1), // +0x1ea70 sub_1ea70
    Lifted::new(0x7001de2a90, tpl_25 as *const (), 1), // +0x1ea90 sub_1ea90
    Lifted::new(0x7001de2aa0, tpl_25 as *const (), 1), // +0x1eaa0 sub_1eaa0
    Lifted::new(0x7001de2ab0, tpl_25 as *const (), 1), // +0x1eab0 sub_1eab0
    Lifted::new(0x7001de2ac0, tpl_25 as *const (), 1), // +0x1eac0 sub_1eac0
    Lifted::new(0x7001de2ad0, fn_7001de2ad0 as *const (), 1), // +0x1ead0 sub_1ead0
    Lifted::new(0x7001de2af0, tpl_55::<0x7001df75c0, 0x7001df75e0, 0x7001df75f0> as *const (), 3), // +0x1eaf0 sub_1eaf0
    Lifted::new(0x7001de2b70, fn_7001de2b70 as *const (), 1), // +0x1eb70 sub_1eb70
    Lifted::new(0x7001de2b90, fn_7001de2b90 as *const (), 1), // +0x1eb90 sub_1eb90
    Lifted::new(0x7001de2bc0, fn_7001de2bc0 as *const (), 1), // +0x1ebc0 sub_1ebc0
    Lifted::new(0x7001de2be0, tpl_55::<0x7001df75d0, 0x7001df75e0, 0x7001df75f0> as *const (), 3), // +0x1ebe0 sub_1ebe0
    Lifted::new(0x7001de2c60, tpl_56 as *const (), 1), // +0x1ec60 sub_1ec60
    Lifted::new(0x7001de2c80, tpl_56 as *const (), 1), // +0x1ec80 sub_1ec80
    Lifted::new(0x7001de2ca0, tpl_56 as *const (), 1), // +0x1eca0 sub_1eca0
    Lifted::new(0x7001de2cc0, tpl_56 as *const (), 1), // +0x1ecc0 sub_1ecc0
    Lifted::new(0x7001de2ce0, tpl_57 as *const (), 1), // +0x1ece0 sub_1ece0
    Lifted::new(0x7001de2d00, tpl_57 as *const (), 1), // +0x1ed00 sub_1ed00
    Lifted::new(0x7001de2d20, tpl_57 as *const (), 1), // +0x1ed20 sub_1ed20
    Lifted::new(0x7001de2d40, tpl_57 as *const (), 1), // +0x1ed40 sub_1ed40
    Lifted::new(0x7001de2d60, tpl_58 as *const (), 1), // +0x1ed60 sub_1ed60
    Lifted::new(0x7001de2d70, tpl_58 as *const (), 1), // +0x1ed70 sub_1ed70
    Lifted::new(0x7001de2d80, tpl_58 as *const (), 1), // +0x1ed80 sub_1ed80
    Lifted::new(0x7001de2d90, tpl_58 as *const (), 1), // +0x1ed90 sub_1ed90
    Lifted::new(0x7001de2da0, tpl_59 as *const (), 1), // +0x1eda0 sub_1eda0
    Lifted::new(0x7001de2dc0, tpl_59 as *const (), 1), // +0x1edc0 sub_1edc0
    Lifted::new(0x7001de2de0, tpl_59 as *const (), 1), // +0x1ede0 sub_1ede0
    Lifted::new(0x7001de2e00, tpl_59 as *const (), 1), // +0x1ee00 sub_1ee00
    Lifted::new(0x7001de2e20, tpl_60 as *const (), 1), // +0x1ee20 sub_1ee20
    Lifted::new(0x7001de2e40, tpl_60 as *const (), 1), // +0x1ee40 sub_1ee40
    Lifted::new(0x7001de2e60, tpl_60 as *const (), 1), // +0x1ee60 sub_1ee60
    Lifted::new(0x7001de2e80, fn_7001de2e80 as *const (), 0), // +0x1ee80 sub_1ee80
    Lifted::new(0x7001de2ec0, fn_7001de2ec0 as *const (), 3), // +0x1eec0 sub_1eec0
    Lifted::new(0x7001de2ef0, fn_7001de2ef0 as *const (), 3), // +0x1eef0 sub_1eef0
    Lifted::new(0x7001de2f10, tpl_14 as *const (), 1), // +0x1ef10 sub_1ef10
    Lifted::new(0x7001de2f20, tpl_14 as *const (), 1), // +0x1ef20 sub_1ef20
    Lifted::new(0x7001de2f30, tpl_14 as *const (), 1), // +0x1ef30 sub_1ef30
    Lifted::new(0x7001de2f40, tpl_42 as *const (), 1), // +0x1ef40 sub_1ef40
    Lifted::new(0x7001de2f50, fn_7001de2f50 as *const (), 0), // +0x1ef50 sub_1ef50
    Lifted::new(0x7001de2f70, fn_7001de2f70 as *const (), 0), // +0x1ef70 sub_1ef70
    Lifted::new(0x7001de2fd0, tpl_15 as *const (), 1), // +0x1efd0 sub_1efd0
    Lifted::new(0x7001de2fe0, tpl_15 as *const (), 1), // +0x1efe0 sub_1efe0
    Lifted::new(0x7001de2ff0, tpl_61 as *const (), 1), // +0x1eff0 sub_1eff0
    Lifted::new(0x7001de3000, tpl_61 as *const (), 1), // +0x1f000 sub_1f000
    Lifted::new(0x7001de3010, tpl_61 as *const (), 1), // +0x1f010 sub_1f010
    Lifted::new(0x7001de3020, tpl_42 as *const (), 1), // +0x1f020 sub_1f020
    Lifted::new(0x7001de3030, tpl_41::<0x7001e0a0b8> as *const (), 0), // +0x1f030 sub_1f030
    Lifted::new(0x7001de3050, tpl_42 as *const (), 1), // +0x1f050 sub_1f050
    Lifted::new(0x7001de3060, fn_7001de3060 as *const (), 0), // +0x1f060 sub_1f060
    Lifted::new(0x7001de3080, fn_7001de3080 as *const (), 0), // +0x1f080 sub_1f080
    Lifted::new(0x7001de30d0, tpl_62::<0x7001e0a0a8> as *const (), 0), // +0x1f0d0 sub_1f0d0
    Lifted::new(0x7001de30f0, tpl_63::<0x7001e0a0d0> as *const (), 0), // +0x1f0f0 sub_1f0f0
    Lifted::new(0x7001de3110, tpl_62::<0x7001e0a0a8> as *const (), 0), // +0x1f110 sub_1f110
    Lifted::new(0x7001de3130, tpl_62::<0x7001e0a0b0> as *const (), 0), // +0x1f130 sub_1f130
    Lifted::new(0x7001de3190, fn_7001de3190 as *const (), 0), // +0x1f190 sub_1f190
    Lifted::new(0x7001de31c0, fn_7001de31c0 as *const (), 0), // +0x1f1c0 sub_1f1c0
    Lifted::new(0x7001de31d0, fn_7001de31d0 as *const (), 0), // +0x1f1d0 sub_1f1d0
    Lifted::new(0x7001de3200, fn_7001de3200 as *const (), 0), // +0x1f200 sub_1f200
    Lifted::new(0x7001de3220, fn_7001de3220 as *const (), 0), // +0x1f220 sub_1f220
    Lifted::new(0x7001de3340, fn_7001de3340 as *const (), 2), // +0x1f340 sub_1f340
    Lifted::new(0x7001de33a0, fn_7001de33a0 as *const (), 1), // +0x1f3a0 sub_1f3a0
    Lifted::new(0x7001de3440, fn_7001de3440 as *const (), 1), // +0x1f440 sub_1f440
    Lifted::new(0x7001de3480, fn_7001de3480 as *const (), 0), // +0x1f480 sub_1f480
    Lifted::new(0x7001de3550, fn_7001de3550 as *const (), 2), // +0x1f550 sub_1f550
    Lifted::new(0x7001de3600, tpl_64::<0x7001e07090> as *const (), 0), // +0x1f600 sub_1f600
    Lifted::new(0x7001de3630, fn_7001de3630 as *const (), 1), // +0x1f630 sub_1f630
    Lifted::new(0x7001de3670, fn_7001de3670 as *const (), 1), // +0x1f670 sub_1f670
    Lifted::new(0x7001de36c0, fn_7001de36c0 as *const (), 1), // +0x1f6c0 sub_1f6c0
    Lifted::new(0x7001de36e0, fn_7001de36e0 as *const (), 1), // +0x1f6e0 sub_1f6e0
    Lifted::new(0x7001de3720, tpl_65::<0x7001df6ac0, 0x7001df75c0> as *const (), 3), // +0x1f720 sub_1f720
    Lifted::new(0x7001de3760, fn_7001de3760 as *const (), 1), // +0x1f760 sub_1f760
    Lifted::new(0x7001de37b0, fn_7001de37b0 as *const (), 1), // +0x1f7b0 sub_1f7b0
    Lifted::new(0x7001de3800, fn_7001de3800 as *const (), 1), // +0x1f800 sub_1f800
    Lifted::new(0x7001de3820, fn_7001de3820 as *const (), 1), // +0x1f820 sub_1f820
    Lifted::new(0x7001de3860, tpl_65::<0x7001df6ac0, 0x7001df75d0> as *const (), 3), // +0x1f860 sub_1f860
    Lifted::new(0x7001de38a0, fn_7001de38a0 as *const (), 0), // +0x1f8a0 sub_1f8a0
    Lifted::new(0x7001de38e0, fn_7001de38e0 as *const (), 0), // +0x1f8e0 sub_1f8e0
    Lifted::new(0x7001de39a0, fn_7001de39a0 as *const (), 0), // +0x1f9a0 sub_1f9a0
    Lifted::new(0x7001de3a00, fn_7001de3a00 as *const (), 0), // +0x1fa00 sub_1fa00
    Lifted::new(0x7001de3ac0, fn_7001de3ac0 as *const (), 2), // +0x1fac0 sub_1fac0
    Lifted::new(0x7001de3b80, fn_7001de3b80 as *const (), 0), // +0x1fb80 sub_1fb80
    Lifted::new(0x7001de3cc0, tpl_64::<0x7001e07090> as *const (), 0), // +0x1fcc0 sub_1fcc0
    Lifted::new(0x7001de3cf0, fn_7001de3cf0 as *const (), 0), // +0x1fcf0 sub_1fcf0
    Lifted::new(0x7001de3df0, fn_7001de3df0 as *const (), 0), // +0x1fdf0 sub_1fdf0
    Lifted::new(0x7001de3ea0, fn_7001de3ea0 as *const (), 0), // +0x1fea0 sub_1fea0
    Lifted::new(0x7001de3f40, fn_7001de3f40 as *const (), 2), // +0x1ff40 sub_1ff40
    Lifted::new(0x7001de41b0, fn_7001de41b0 as *const (), 1), // +0x201b0 sub_201b0
    Lifted::new(0x7001de4210, fn_7001de4210 as *const (), 3), // +0x20210 sub_20210
    Lifted::new(0x7001de4260, fn_7001de4260 as *const (), 2), // +0x20260 sub_20260
    Lifted::new(0x7001de4540, fn_7001de4540 as *const (), 0), // +0x20540 sub_20540
    Lifted::new(0x7001de45d0, fn_7001de45d0 as *const (), 0), // +0x205d0 sub_205d0
    Lifted::new(0x7001de4640, fn_7001de4640 as *const (), 1), // +0x20640 sub_20640
    Lifted::new(0x7001de4760, fn_7001de4760 as *const (), 2), // +0x20760 sub_20760
    Lifted::new(0x7001de48d0, fn_7001de48d0 as *const (), 2), // +0x208d0 sub_208d0
    Lifted::new(0x7001de4c10, fn_7001de4c10 as *const (), 0), // +0x20c10 sub_20c10
    Lifted::new(0x7001de4cf0, fn_7001de4cf0 as *const (), 0), // +0x20cf0 sub_20cf0
    Lifted::new(0x7001de4dc0, fn_7001de4dc0 as *const (), 0), // +0x20dc0 sub_20dc0
    Lifted::new(0x7001de4e70, fn_7001de4e70 as *const (), 0), // +0x20e70 sub_20e70
    Lifted::new(0x7001de4fa0, fn_7001de4fa0 as *const (), 1), // +0x20fa0 sub_20fa0
    Lifted::new(0x7001de5020, fn_7001de5020 as *const (), 1), // +0x21020 sub_21020
    Lifted::new(0x7001de5090, fn_7001de5090 as *const (), 2), // +0x21090 sub_21090
    Lifted::new(0x7001de51d0, fn_7001de51d0 as *const (), 1), // +0x211d0 sub_211d0
    Lifted::new(0x7001de5220, fn_7001de5220 as *const (), 0), // +0x21220 sub_21220
    Lifted::new(0x7001de52f0, fn_7001de52f0 as *const (), 0), // +0x212f0 sub_212f0
    Lifted::new(0x7001de5400, fn_7001de5400 as *const (), 0), // +0x21400 sub_21400
    Lifted::new(0x7001de5450, fn_7001de5450 as *const (), 0), // +0x21450 sub_21450
    Lifted::new(0x7001de55a0, fn_7001de55a0 as *const (), 0), // +0x215a0 sub_215a0
    Lifted::new(0x7001de5870, fn_7001de5870 as *const (), 0), // +0x21870 sub_21870
    Lifted::new(0x7001de5960, fn_7001de5960 as *const (), 0), // +0x21960 sub_21960
    Lifted::new(0x7001de5c40, fn_7001de5c40 as *const (), 0), // +0x21c40 sub_21c40
    Lifted::new(0x7001de60c0, fn_7001de60c0 as *const (), 1), // +0x220c0 sub_220c0
    Lifted::new(0x7001de63e0, fn_7001de63e0 as *const (), 1), // +0x223e0 sub_223e0
    Lifted::new(0x7001de6450, fn_7001de6450 as *const (), 1), // +0x22450 sub_22450
    Lifted::new(0x7001de64c0, tpl_22::<0x7001df7620, 0x7001deddc0, 0x7001de64d0> as *const (), 0), // +0x224c0 sub_224c0
    Lifted::new(0x7001de64e0, fn_7001de64e0 as *const (), 1), // +0x224e0 sub_224e0
    Lifted::new(0x7001de6510, fn_7001de6510 as *const (), 0), // +0x22510 sub_22510
    Lifted::new(0x7001de6560, tpl_10 as *const (), 1), // +0x22560 sub_22560
    Lifted::new(0x7001de6570, tpl_24::<0xfffffffffffffff8, 0x7001de2110, 0x7001de2030> as *const (), 0), // +0x22570 sub_22570
    Lifted::new(0x7001de65c0, tpl_7::<0x7001de2030> as *const (), 0), // +0x225c0 sub_225c0
    Lifted::new(0x7001de65f0, tpl_8::<0x10> as *const (), 1), // +0x225f0 sub_225f0
    Lifted::new(0x7001de6600, fn_7001de6600 as *const (), 0), // +0x22600 sub_22600
    Lifted::new(0x7001de6610, fn_7001de6610 as *const (), 0), // +0x22610 sub_22610
    Lifted::new(0x7001de6670, tpl_12::<0x8, 0x60> as *const (), 0), // +0x22670 sub_22670
    Lifted::new(0x7001de6680, tpl_8::<0x18> as *const (), 1), // +0x22680 sub_22680
    Lifted::new(0x7001de6690, tpl_12::<0x8, 0x30> as *const (), 0), // +0x22690 sub_22690
    Lifted::new(0x7001de66a0, tpl_12::<0x8, 0x20> as *const (), 0), // +0x226a0 sub_226a0
    Lifted::new(0x7001de66b0, tpl_12::<0x8, 0x38> as *const (), 0), // +0x226b0 sub_226b0
    Lifted::new(0x7001de66c0, tpl_12::<0x8, 0x40> as *const (), 0), // +0x226c0 sub_226c0
    Lifted::new(0x7001de66d0, tpl_12::<0x8, 0x18> as *const (), 0), // +0x226d0 sub_226d0
    Lifted::new(0x7001de66e0, fn_7001de66e0 as *const (), 2), // +0x226e0 sub_226e0
    Lifted::new(0x7001de6730, tpl_66::<0x8, 0x7001dc5fb0> as *const (), 0), // +0x22730 sub_22730
    Lifted::new(0x7001de6750, tpl_66::<0x8, 0x7001dc7430> as *const (), 0), // +0x22750 sub_22750
    Lifted::new(0x7001de6770, tpl_15 as *const (), 1), // +0x22770 sub_22770
    Lifted::new(0x7001de6780, tpl_15 as *const (), 1), // +0x22780 sub_22780
    Lifted::new(0x7001de6790, fn_7001de6790 as *const (), 0), // +0x22790 sub_22790
    Lifted::new(0x7001de6810, tpl_40 as *const (), 1), // +0x22810 sub_22810
    Lifted::new(0x7001de6820, fn_7001de6820 as *const (), 2), // +0x22820 sub_22820
    Lifted::new(0x7001de6860, fn_7001de6860 as *const (), 0), // +0x22860 sub_22860
    Lifted::new(0x7001de69c0, fn_7001de69c0 as *const (), 2), // +0x229c0 sub_229c0
    Lifted::new(0x7001de6cd0, fn_7001de6cd0 as *const (), 1), // +0x22cd0 sub_22cd0
    Lifted::new(0x7001de6db0, fn_7001de6db0 as *const (), 2), // +0x22db0 sub_22db0
    Lifted::new(0x7001de6ef0, fn_7001de6ef0 as *const (), 0), // +0x22ef0 sub_22ef0
    Lifted::new(0x7001de6fc0, fn_7001de6fc0 as *const (), 0), // +0x22fc0 sub_22fc0
    Lifted::new(0x7001de72e0, fn_7001de72e0 as *const (), 1), // +0x232e0 sub_232e0
    Lifted::new(0x7001de7330, fn_7001de7330 as *const (), 1), // +0x23330 sub_23330
    Lifted::new(0x7001de7380, fn_7001de7380 as *const (), 1), // +0x23380 sub_23380
    Lifted::new(0x7001de73f0, fn_7001de73f0 as *const (), 3), // +0x233f0 sub_233f0
    Lifted::new(0x7001de7460, fn_7001de7460 as *const (), 1), // +0x23460 sub_23460
    Lifted::new(0x7001de74a0, fn_7001de74a0 as *const (), 1), // +0x234a0 sub_234a0
    Lifted::new(0x7001de7570, fn_7001de7570 as *const (), 2), // +0x23570 sub_23570
    Lifted::new(0x7001de7960, fn_7001de7960 as *const (), 2), // +0x23960 sub_23960
    Lifted::new(0x7001de82a0, fn_7001de82a0 as *const (), 2), // +0x242a0 sub_242a0
    Lifted::new(0x7001de8350, fn_7001de8350 as *const (), 0), // +0x24350 sub_24350
    Lifted::new(0x7001de8420, fn_7001de8420 as *const (), 0), // +0x24420 sub_24420
    Lifted::new(0x7001de8590, fn_7001de8590 as *const (), 0), // +0x24590 sub_24590
    Lifted::new(0x7001de8640, tpl_67::<0x7001df7910, 0x8> as *const (), 3), // +0x24640 sub_24640
    Lifted::new(0x7001de8660, fn_7001de8660 as *const (), 0), // +0x24660 sub_24660
    Lifted::new(0x7001de8680, fn_7001de8680 as *const (), 0), // +0x24680 sub_24680
    Lifted::new(0x7001de8690, fn_7001de8690 as *const (), 0), // +0x24690 sub_24690
    Lifted::new(0x7001de8900, tpl_26::<0x8> as *const (), 1), // +0x24900 sub_24900
    Lifted::new(0x7001de8910, tpl_67::<0x7001df7910, 0x8> as *const (), 3), // +0x24910 sub_24910
    Lifted::new(0x7001de8930, fn_7001de8930 as *const (), 2), // +0x24930 sub_24930
    Lifted::new(0x7001de8a10, tpl_68::<0x7001e07210> as *const (), 1), // +0x24a10 sub_24a10
    Lifted::new(0x7001de8a20, tpl_22::<0x7001df78b8, 0x7001deddc0, 0x7001de8a30> as *const (), 0), // +0x24a20 sub_24a20
    Lifted::new(0x7001de8a40, fn_7001de8a40 as *const (), 0), // +0x24a40 sub_24a40
    Lifted::new(0x7001de8ad0, fn_7001de8ad0 as *const (), 0), // +0x24ad0 sub_24ad0
    Lifted::new(0x7001de8b90, fn_7001de8b90 as *const (), 1), // +0x24b90 sub_24b90
    Lifted::new(0x7001de8c70, fn_7001de8c70 as *const (), 1), // +0x24c70 sub_24c70
    Lifted::new(0x7001de8cc0, fn_7001de8cc0 as *const (), 0), // +0x24cc0 sub_24cc0
    Lifted::new(0x7001de8d70, fn_7001de8d70 as *const (), 2), // +0x24d70 sub_24d70
    Lifted::new(0x7001de8fe0, fn_7001de8fe0 as *const (), 2), // +0x24fe0 sub_24fe0
    Lifted::new(0x7001de90a0, fn_7001de90a0 as *const (), 2), // +0x250a0 sub_250a0
    Lifted::new(0x7001de91c0, fn_7001de91c0 as *const (), 0), // +0x251c0 sub_251c0
    Lifted::new(0x7001de92d0, fn_7001de92d0 as *const (), 0), // +0x252d0 sub_252d0
    Lifted::new(0x7001de9380, fn_7001de9380 as *const (), 0), // +0x25380 sub_25380
    Lifted::new(0x7001de9460, fn_7001de9460 as *const (), 0), // +0x25460 sub_25460
    Lifted::new(0x7001de94c0, fn_7001de94c0 as *const (), 0), // +0x254c0 sub_254c0
    Lifted::new(0x7001de9530, fn_7001de9530 as *const (), 0), // +0x25530 sub_25530
    Lifted::new(0x7001de9650, tpl_7::<0x7001de2030> as *const (), 0), // +0x25650 sub_25650
    Lifted::new(0x7001de9680, fn_7001de9680 as *const (), 0), // +0x25680 sub_25680
    Lifted::new(0x7001de96f0, tpl_7::<0x7001de2030> as *const (), 0), // +0x256f0 sub_256f0
    Lifted::new(0x7001de9720, fn_7001de9720 as *const (), 0), // +0x25720 sub_25720
    Lifted::new(0x7001de97a0, tpl_21::<0x7001df7a88, 0x7001de2030> as *const (), 0), // +0x257a0 sub_257a0
    Lifted::new(0x7001de97d0, fn_7001de97d0 as *const (), 0), // +0x257d0 sub_257d0
    Lifted::new(0x7001de9840, fn_7001de9840 as *const (), 1), // +0x25840 sub_25840
    Lifted::new(0x7001de9850, tpl_10 as *const (), 1), // +0x25850 sub_25850
    Lifted::new(0x7001de9860, fn_7001de9860 as *const (), 0), // +0x25860 sub_25860
    Lifted::new(0x7001de9a90, fn_7001de9a90 as *const (), 0), // +0x25a90 sub_25a90
    Lifted::new(0x7001de9ad0, tpl_10 as *const (), 1), // +0x25ad0 sub_25ad0
    Lifted::new(0x7001de9ae0, fn_7001de9ae0 as *const (), 1), // +0x25ae0 sub_25ae0
    Lifted::new(0x7001de9af0, fn_7001de9af0 as *const (), 0), // +0x25af0 sub_25af0
    Lifted::new(0x7001de9b40, fn_7001de9b40 as *const (), 0), // +0x25b40 sub_25b40
    Lifted::new(0x7001de9bc0, tpl_12::<0x8, 0x10> as *const (), 0), // +0x25bc0 sub_25bc0
    Lifted::new(0x7001de9bd0, fn_7001de9bd0 as *const (), 0), // +0x25bd0 sub_25bd0
    Lifted::new(0x7001de9bf0, fn_7001de9bf0 as *const (), 2), // +0x25bf0 sub_25bf0
    Lifted::new(0x7001de9ca0, fn_7001de9ca0 as *const (), 0), // +0x25ca0 sub_25ca0
    Lifted::new(0x7001de9cc0, fn_7001de9cc0 as *const (), 1), // +0x25cc0 sub_25cc0
    Lifted::new(0x7001de9cd0, fn_7001de9cd0 as *const (), 0), // +0x25cd0 sub_25cd0
    Lifted::new(0x7001de9d90, fn_7001de9d90 as *const (), 0), // +0x25d90 sub_25d90
    Lifted::new(0x7001de9dd0, fn_7001de9dd0 as *const (), 0), // +0x25dd0 sub_25dd0
    Lifted::new(0x7001de9e40, fn_7001de9e40 as *const (), 0), // +0x25e40 sub_25e40
    Lifted::new(0x7001de9e70, fn_7001de9e70 as *const (), 0), // +0x25e70 sub_25e70
    Lifted::new(0x7001de9ea0, fn_7001de9ea0 as *const (), 0), // +0x25ea0 sub_25ea0
    Lifted::new(0x7001de9f10, fn_7001de9f10 as *const (), 0), // +0x25f10 sub_25f10
    Lifted::new(0x7001de9fa0, fn_7001de9fa0 as *const (), 0), // +0x25fa0 sub_25fa0
    Lifted::new(0x7001dea060, fn_7001dea060 as *const (), 2), // +0x26060 sub_26060
    Lifted::new(0x7001dea140, fn_7001dea140 as *const (), 0), // +0x26140 sub_26140
    Lifted::new(0x7001dea420, fn_7001dea420 as *const (), 0), // +0x26420 sub_26420
    Lifted::new(0x7001dea450, fn_7001dea450 as *const (), 0), // +0x26450 sub_26450
    Lifted::new(0x7001dea500, fn_7001dea500 as *const (), 1), // +0x26500 sub_26500
    Lifted::new(0x7001dea620, tpl_11 as *const (), 1), // +0x26620 sub_26620
    Lifted::new(0x7001dea630, fn_7001dea630 as *const (), 0), // +0x26630 sub_26630
    Lifted::new(0x7001dea6c0, fn_7001dea6c0 as *const (), 1), // +0x266c0 sub_266c0
    Lifted::new(0x7001dea6d0, fn_7001dea6d0 as *const (), 0), // +0x266d0 sub_266d0
    Lifted::new(0x7001dea8c0, fn_7001dea8c0 as *const (), 0), // +0x268c0 sub_268c0
    Lifted::new(0x7001dea9b0, fn_7001dea9b0 as *const (), 0), // +0x269b0 sub_269b0
    Lifted::new(0x7001dea9f0, fn_7001dea9f0 as *const (), 2), // +0x269f0 sub_269f0
    Lifted::new(0x7001deb540, fn_7001deb540 as *const (), 2), // +0x27540 sub_27540
    Lifted::new(0x7001deb690, tpl_12::<0x8, 0x18> as *const (), 0), // +0x27690 sub_27690
    Lifted::new(0x7001deb6a0, fn_7001deb6a0 as *const (), 2), // +0x276a0 sub_276a0
    Lifted::new(0x7001deb790, fn_7001deb790 as *const (), 0), // +0x27790 sub_27790
    Lifted::new(0x7001deb7d0, fn_7001deb7d0 as *const (), 0), // +0x277d0 sub_277d0
    Lifted::new(0x7001deb800, fn_7001deb800 as *const (), 2), // +0x27800 sub_27800
    Lifted::new(0x7001deb8b0, fn_7001deb8b0 as *const (), 0), // +0x278b0 sub_278b0
    Lifted::new(0x7001deb9f0, fn_7001deb9f0 as *const (), 0), // +0x279f0 sub_279f0
    Lifted::new(0x7001deba50, fn_7001deba50 as *const (), 0), // +0x27a50 sub_27a50
    Lifted::new(0x7001debab0, fn_7001debab0 as *const (), 0), // +0x27ab0 sub_27ab0
    Lifted::new(0x7001debad0, tpl_63::<0x7001e0a160> as *const (), 0), // +0x27ad0 sub_27ad0
    Lifted::new(0x7001debaf0, fn_7001debaf0 as *const (), 0), // +0x27af0 sub_27af0
    Lifted::new(0x7001debb20, fn_7001debb20 as *const (), 0), // +0x27b20 sub_27b20
    Lifted::new(0x7001debba0, fn_7001debba0 as *const (), 2), // +0x27ba0 sub_27ba0
    Lifted::new(0x7001debc70, fn_7001debc70 as *const (), 0), // +0x27c70 sub_27c70
    Lifted::new(0x7001debd50, tpl_10 as *const (), 1), // +0x27d50 sub_27d50
    Lifted::new(0x7001debd60, fn_7001debd60 as *const (), 0), // +0x27d60 sub_27d60
    Lifted::new(0x7001debd90, fn_7001debd90 as *const (), 0), // +0x27d90 sub_27d90
    Lifted::new(0x7001debdf0, fn_7001debdf0 as *const (), 0), // +0x27df0 sub_27df0
    Lifted::new(0x7001debe20, fn_7001debe20 as *const (), 0), // +0x27e20 sub_27e20
    Lifted::new(0x7001debe60, fn_7001debe60 as *const (), 0), // +0x27e60 sub_27e60
    Lifted::new(0x7001debef0, fn_7001debef0 as *const (), 0), // +0x27ef0 sub_27ef0
    Lifted::new(0x7001debf30, fn_7001debf30 as *const (), 0), // +0x27f30 sub_27f30
    Lifted::new(0x7001debf70, fn_7001debf70 as *const (), 0), // +0x27f70 sub_27f70
    Lifted::new(0x7001dec0f0, fn_7001dec0f0 as *const (), 0), // +0x280f0 sub_280f0
    Lifted::new(0x7001dec1a0, tpl_22::<0x7001df78b8, 0x7001deddc0, 0x7001dec1b0> as *const (), 0), // +0x281a0 sub_281a0
    Lifted::new(0x7001dec1c0, fn_7001dec1c0 as *const (), 0), // +0x281c0 sub_281c0
    Lifted::new(0x7001dec250, fn_7001dec250 as *const (), 0), // +0x28250 sub_28250
    Lifted::new(0x7001dec410, fn_7001dec410 as *const (), 0), // +0x28410 sub_28410
    Lifted::new(0x7001dec590, fn_7001dec590 as *const (), 1), // +0x28590 sub_28590
    Lifted::new(0x7001dec5b0, tpl_8::<0x18> as *const (), 1), // +0x285b0 sub_285b0
    Lifted::new(0x7001dec5c0, fn_7001dec5c0 as *const (), 0), // +0x285c0 sub_285c0
    Lifted::new(0x7001dec750, fn_7001dec750 as *const (), 0), // +0x28750 sub_28750
    Lifted::new(0x7001dec7b0, tpl_8::<0x20> as *const (), 1), // +0x287b0 sub_287b0
    Lifted::new(0x7001dec7c0, tpl_68::<0x7001e07678> as *const (), 1), // +0x287c0 sub_287c0
    Lifted::new(0x7001dec7d0, fn_7001dec7d0 as *const (), 2), // +0x287d0 sub_287d0
    Lifted::new(0x7001deca00, fn_7001deca00 as *const (), 0), // +0x28a00 sub_28a00
    Lifted::new(0x7001deca30, tpl_6::<0x7001dc7bc0> as *const (), 0), // +0x28a30 sub_28a30
    Lifted::new(0x7001deca50, fn_7001deca50 as *const (), 0), // +0x28a50 sub_28a50
    Lifted::new(0x7001deca90, fn_7001deca90 as *const (), 1), // +0x28a90 sub_28a90
    Lifted::new(0x7001decaa0, fn_7001decaa0 as *const (), 0), // +0x28aa0 sub_28aa0
    Lifted::new(0x7001decae0, fn_7001decae0 as *const (), 2), // +0x28ae0 sub_28ae0
    Lifted::new(0x7001decdd0, fn_7001decdd0 as *const (), 2), // +0x28dd0 sub_28dd0
    Lifted::new(0x7001ded3c0, fn_7001ded3c0 as *const (), 2), // +0x293c0 sub_293c0
    Lifted::new(0x7001ded780, fn_7001ded780 as *const (), 0), // +0x29780 sub_29780
    Lifted::new(0x7001ded790, fn_7001ded790 as *const (), 0), // +0x29790 sub_29790
    Lifted::new(0x7001ded810, fn_7001ded810 as *const (), 0), // +0x29810 sub_29810
    Lifted::new(0x7001ded8a0, fn_7001ded8a0 as *const (), 0), // +0x298a0 sub_298a0
    Lifted::new(0x7001deda80, fn_7001deda80 as *const (), 0), // +0x29a80 sub_29a80
    Lifted::new(0x7001dedee4, fn_7001dedee4 as *const (), 0), // +0x29ee4 sub_29ee4
    Lifted::new(0x7001dedf08, fn_7001dedf08 as *const (), 0), // +0x29f08 sub_29f08
    Lifted::new(0x7001dedf90, fn_7001dedf90 as *const (), 0), // +0x29f90 sub_29f90
    Lifted::new(0x7001dedfc0, fn_7001dedfc0 as *const (), 0), // +0x29fc0 sub_29fc0
    Lifted::new(0x7001dee0c4, fn_7001dee0c4 as *const (), 0), // +0x2a0c4 sub_2a0c4
    Lifted::new(0x7001dee1b0, fn_7001dee1b0 as *const (), 0), // +0x2a1b0 sub_2a1b0
    Lifted::new(0x7001dee1e4, fn_7001dee1e4 as *const (), 0), // +0x2a1e4 sub_2a1e4
    Lifted::new(0x7001dee220, fn_7001dee220 as *const (), 0), // +0x2a220 sub_2a220
    Lifted::new(0x7001dee2a0, fn_7001dee2a0 as *const (), 0), // +0x2a2a0 sub_2a2a0
    Lifted::new(0x7001dee334, fn_7001dee334 as *const (), 0), // +0x2a334 sub_2a334
    Lifted::new(0x7001dee39c, fn_7001dee39c as *const (), 0), // +0x2a39c sub_2a39c
    Lifted::new(0x7001dee430, fn_7001dee430 as *const (), 0), // +0x2a430 sub_2a430
    Lifted::new(0x7001dee468, fn_7001dee468 as *const (), 0), // +0x2a468 sub_2a468
    Lifted::new(0x7001dee530, fn_7001dee530 as *const (), 0), // +0x2a530 sub_2a530
    Lifted::new(0x7001dee578, fn_7001dee578 as *const (), 0), // +0x2a578 sub_2a578
    Lifted::new(0x7001dee5bc, fn_7001dee5bc as *const (), 0), // +0x2a5bc sub_2a5bc
    Lifted::new(0x7001dee5d8, fn_7001dee5d8 as *const (), 0), // +0x2a5d8 sub_2a5d8
    Lifted::new(0x7001dee60c, fn_7001dee60c as *const (), 0), // +0x2a60c sub_2a60c
    Lifted::new(0x7001dee628, fn_7001dee628 as *const (), 0), // +0x2a628 sub_2a628
    Lifted::new(0x7001dee6a0, fn_7001dee6a0 as *const (), 0), // +0x2a6a0 sub_2a6a0
    Lifted::new(0x7001dee6dc, fn_7001dee6dc as *const (), 0), // +0x2a6dc sub_2a6dc
    Lifted::new(0x7001dee6f8, fn_7001dee6f8 as *const (), 0), // +0x2a6f8 sub_2a6f8
    Lifted::new(0x7001dee754, fn_7001dee754 as *const (), 2), // +0x2a754 sub_2a754
    Lifted::new(0x7001dee804, fn_7001dee804 as *const (), 1), // +0x2a804 sub_2a804
    Lifted::new(0x7001dee8c4, fn_7001dee8c4 as *const (), 0), // +0x2a8c4 sub_2a8c4
    Lifted::new(0x7001dee8f4, fn_7001dee8f4 as *const (), 0), // +0x2a8f4 sub_2a8f4
    Lifted::new(0x7001dee928, fn_7001dee928 as *const (), 0), // +0x2a928 sub_2a928
    Lifted::new(0x7001dee9a0, fn_7001dee9a0 as *const (), 0), // +0x2a9a0 sub_2a9a0
    Lifted::new(0x7001dee9d0, fn_7001dee9d0 as *const (), 1), // +0x2a9d0 sub_2a9d0
    Lifted::new(0x7001deea40, fn_7001deea40 as *const (), 0), // +0x2aa40 sub_2aa40
    Lifted::new(0x7001deeaa4, fn_7001deeaa4 as *const (), 0), // +0x2aaa4 sub_2aaa4
    Lifted::new(0x7001deec00, fn_7001deec00 as *const (), 0), // +0x2ac00 sub_2ac00
    Lifted::new(0x7001deeca8, fn_7001deeca8 as *const (), 0), // +0x2aca8 sub_2aca8
    Lifted::new(0x7001deee50, fn_7001deee50 as *const (), 0), // +0x2ae50 sub_2ae50
    Lifted::new(0x7001deee9c, fn_7001deee9c as *const (), 0), // +0x2ae9c sub_2ae9c
    Lifted::new(0x7001deeee0, fn_7001deeee0 as *const (), 0), // +0x2aee0 sub_2aee0
    Lifted::new(0x7001deefe8, fn_7001deefe8 as *const (), 0), // +0x2afe8 sub_2afe8
    Lifted::new(0x7001def000, fn_7001def000 as *const (), 0), // +0x2b000 sub_2b000
    Lifted::new(0x7001def0c4, fn_7001def0c4 as *const (), 0), // +0x2b0c4 sub_2b0c4
    Lifted::new(0x7001def4dc, fn_7001def4dc as *const (), 0), // +0x2b4dc sub_2b4dc
    Lifted::new(0x7001def530, fn_7001def530 as *const (), 0), // +0x2b530 sub_2b530
    Lifted::new(0x7001def5b0, fn_7001def5b0 as *const (), 0), // +0x2b5b0 sub_2b5b0
    Lifted::new(0x7001def630, fn_7001def630 as *const (), 1), // +0x2b630 sub_2b630
    Lifted::new(0x7001def690, fn_7001def690 as *const (), 0), // +0x2b690 sub_2b690
    Lifted::new(0x7001def794, fn_7001def794 as *const (), 0), // +0x2b794 sub_2b794
    Lifted::new(0x7001def840, fn_7001def840 as *const (), 1), // +0x2b840 sub_2b840
    Lifted::new(0x7001defa4c, fn_7001defa4c as *const (), 1), // +0x2ba4c sub_2ba4c
    Lifted::new(0x7001defa54, fn_7001defa54 as *const (), 1), // +0x2ba54 sub_2ba54
    Lifted::new(0x7001defa64, fn_7001defa64 as *const (), 0), // +0x2ba64 sub_2ba64
    Lifted::new(0x7001defaec, fn_7001defaec as *const (), 0), // +0x2baec sub_2baec
    Lifted::new(0x7001defbc4, fn_7001defbc4 as *const (), 0), // +0x2bbc4 sub_2bbc4
    Lifted::new(0x7001defbf0, fn_7001defbf0 as *const (), 0), // +0x2bbf0 sub_2bbf0
    Lifted::new(0x7001defc04, fn_7001defc04 as *const (), 0), // +0x2bc04 sub_2bc04
    Lifted::new(0x7001defc14, tpl_68::<0x7001e07e98> as *const (), 1), // +0x2bc14 sub_2bc14
    Lifted::new(0x7001defc20, fn_7001defc20 as *const (), 0), // +0x2bc20 sub_2bc20
    Lifted::new(0x7001defc44, tpl_68::<0x7001e07ea8> as *const (), 1), // +0x2bc44 sub_2bc44
    Lifted::new(0x7001defc50, tpl_69::<0x7001df8b38, 0x7001df8c40, 0x7001e10020> as *const (), 0), // +0x2bc50 sub_2bc50
    Lifted::new(0x7001defc9c, tpl_69::<0x7001df8d58, 0x7001df8e60, 0x7001e10020> as *const (), 0), // +0x2bc9c sub_2bc9c
    Lifted::new(0x7001defea0, tpl_13 as *const (), 1), // +0x2bea0 sub_2bea0
    Lifted::new(0x7001defea4, tpl_13 as *const (), 1), // +0x2bea4 sub_2bea4
    Lifted::new(0x7001defea8, tpl_13 as *const (), 1), // +0x2bea8 sub_2bea8
    Lifted::new(0x7001defeac, tpl_13 as *const (), 1), // +0x2beac sub_2beac
    Lifted::new(0x7001defeb0, tpl_13 as *const (), 1), // +0x2beb0 sub_2beb0
    Lifted::new(0x7001defeb4, tpl_40 as *const (), 1), // +0x2beb4 sub_2beb4
    Lifted::new(0x7001df17a0, fn_7001df17a0 as *const (), 2), // +0x2d7a0 sub_2d7a0
    Lifted::new(0x7001df17c0, tpl_35::<0x7001e10020> as *const (), 2), // +0x2d7c0 sub_2d7c0
    Lifted::new(0x7001df27ce, fn_7001df27ce as *const (), 1), // +0x2e7ce sub_2e7ce
    Lifted::new(0x7001df27ec, fn_7001df27ec as *const (), 0), // +0x2e7ec sub_2e7ec
    Lifted::new(0x7001df2808, fn_7001df2808 as *const (), 0), // +0x2e808 sub_2e808
    Lifted::new(0x7001df2821, fn_7001df2821 as *const (), 0), // +0x2e821 sub_2e821
    Lifted::new(0x7001df2840, fn_7001df2840 as *const (), 0), // +0x2e840 sub_2e840
    Lifted::new(0x7001df3890, fn_7001df3890 as *const (), 2), // +0x2f890 sub_2f890
];
