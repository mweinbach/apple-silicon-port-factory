#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::alias(0x7001ed1020, fn_7001ed9120 as *const (), 1, 0x9120), // +0x1020 ?IsArgumentFilterEnabled@TraceConfig@tracing@platform@v8@@QEBA_NXZ (thunk -> +0x9120)
    Lifted::alias(0x7001ed1030, fn_7001ee62e0 as *const (), 1, 0x162e0), // +0x1030 sub_1030 (thunk -> +0x162e0)
    Lifted::alias(0x7001ed1040, fn_7001edf1e0 as *const (), 0, 0xf1e0), // +0x1040 sub_1040 (thunk -> +0xf1e0)
    Lifted::alias(0x7001ed1050, fn_7001edcc60 as *const (), 2, 0xcc60), // +0x1050 sub_1050 (thunk -> +0xcc60)
    Lifted::alias(0x7001ed1060, fn_7001edeb10 as *const (), 0, 0xeb10), // +0x1060 sub_1060 (thunk -> +0xeb10)
    Lifted::alias(0x7001ed1070, fn_7001ee8150 as *const (), 0, 0x18150), // +0x1070 sub_1070 (thunk -> +0x18150)
    Lifted::alias(0x7001ed1080, fn_7001ee4f40 as *const (), 0, 0x14f40), // +0x1080 sub_1080 (thunk -> +0x14f40)
    Lifted::alias(0x7001ed1090, fn_7001ee8fb0 as *const (), 0, 0x18fb0), // +0x1090 sub_1090 (thunk -> +0x18fb0)
    Lifted::alias(0x7001ed10a0, tpl_20 as *const (), 1, 0xdb20), // +0x10a0 sub_10a0 (thunk -> +0xdb20)
    Lifted::alias(0x7001ed10d0, fn_7001ee7a30 as *const (), 0, 0x17a30), // +0x10d0 ?UpdateCategoryGroupEnabledFlag@TracingController@tracing@platform@v8@@AEAAX_K@Z (thunk -> +0x17a30)
    Lifted::alias(0x7001ed10e0, tpl_14::<0x7001eea7e2> as *const (), 0, 0xb570), // +0x10e0 sub_10e0 (thunk -> +0xb570)
    Lifted::alias(0x7001ed1100, fn_7001edf590 as *const (), 0, 0xf590), // +0x1100 sub_1100 (thunk -> +0xf590)
    Lifted::alias(0x7001ed1110, fn_7001edc0c0 as *const (), 0, 0xc0c0), // +0x1110 sub_1110 (thunk -> +0xc0c0)
    Lifted::alias(0x7001ed1130, fn_7001eea2a0 as *const (), 1, 0x1a2a0), // +0x1130 sub_1130 (thunk -> +0x1a2a0)
    Lifted::alias(0x7001ed1140, fn_7001ee8a40 as *const (), 0, 0x18a40), // +0x1140 sub_1140 (thunk -> +0x18a40)
    Lifted::alias(0x7001ed1150, tpl_37 as *const (), 1, 0xd530), // +0x1150 sub_1150 (thunk -> +0xd530)
    Lifted::alias(0x7001ed1160, tpl_27::<0x80> as *const (), 1, 0xaa70), // +0x1160 ?flags@TraceObject@tracing@platform@v8@@QEBAIXZ (thunk -> +0xaa70)
    Lifted::alias(0x7001ed1170, fn_7001edc270 as *const (), 0, 0xc270), // +0x1170 ?UpdatePriority@DefaultJobState@platform@v8@@QEAAXW4TaskPriority@3@@Z (thunk -> +0xc270)
    Lifted::alias(0x7001ed1180, tpl_42::<0x10, 0x19, 0x8> as *const (), 1, 0x10c70), // +0x1180 sub_1180 (thunk -> +0x10c70)
    Lifted::alias(0x7001ed11b0, fn_7001edafc0 as *const (), 0, 0xafc0), // +0x11b0 ??1?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@IEAA@XZ (thunk -> +0xafc0)
    Lifted::alias(0x7001ed11c0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0x8ed0), // +0x11c0 sub_11c0 (thunk -> +0x8ed0)
    Lifted::alias(0x7001ed11d0, fn_7001ede160 as *const (), 0, 0xe160), // +0x11d0 ?SetTracingController@platform@v8@@YAXPEAVPlatform@2@PEAVTracingController@tracing@12@@Z (thunk -> +0xe160)
    Lifted::alias(0x7001ed11e0, fn_7001ee82a0 as *const (), 0, 0x182a0), // +0x11e0 ?Run@WorkerThread@platform@v8@@UEAAXXZ (thunk -> +0x182a0)
    Lifted::alias(0x7001ed11f0, fn_7001ed9cb0 as *const (), 0, 0x9cb0), // +0x11f0 ?PostTaskLocked@DefaultForegroundTaskRunner@platform@v8@@AEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@ (thunk -> +0x9cb0)
    Lifted::alias(0x7001ed1200, tpl_14::<0x7001eea7e2> as *const (), 0, 0x16fb0), // +0x1200 sub_1200 (thunk -> +0x16fb0)
    Lifted::alias(0x7001ed1220, tpl_37 as *const (), 1, 0xd510), // +0x1220 sub_1220 (thunk -> +0xd510)
    Lifted::alias(0x7001ed1230, fn_7001edc2f0 as *const (), 0, 0xc2f0), // +0x1230 ?WaitForParticipationOpportunityLockRequired@DefaultJobState@platform@v8@@AEAA_NXZ (thunk -> +0xc2f0)
    Lifted::alias(0x7001ed1240, fn_7001ee9380 as *const (), 0, 0x19380), // +0x1240 sub_1240 (thunk -> +0x19380)
    Lifted::alias(0x7001ed1250, tpl_12 as *const (), 1, 0x17bd0), // +0x1250 sub_1250 (thunk -> +0x17bd0)
    Lifted::alias(0x7001ed1260, tpl_44::<0x7001eef448, 0x7001efd678, 0x7001efd6f0> as *const (), 0, 0x18110), // +0x1260 ??1WorkerThread@platform@v8@@UEAA@XZ (thunk -> +0x18110)
    Lifted::alias(0x7001ed1270, tpl_14::<0x7001eea7e2> as *const (), 0, 0xb540), // +0x1270 sub_1270 (thunk -> +0xb540)
    Lifted::alias(0x7001ed1290, fn_7001ee5e20 as *const (), 0, 0x15e20), // +0x1290 ?CreateJSONTraceWriter@TraceWriter@tracing@platform@v8@@SAPEAV1234@AEAV?$basic_ostream@DU?$char_traits@D@std@@@std@@AEBV (thunk -> +0x15e20)
    Lifted::alias(0x7001ed12a0, fn_7001ee7550 as *const (), 0, 0x17550), // +0x12a0 ?Initialize@TracingController@tracing@platform@v8@@QEAAXPEAVTraceBuffer@234@@Z (thunk -> +0x17550)
    Lifted::alias(0x7001ed12e0, tpl_25::<0x7001eeec70, 0x7001ee88f0, 0x7001edeb00> as *const (), 0, 0xeaf0), // +0x12e0 sub_12e0 (thunk -> +0xeaf0)
    Lifted::alias(0x7001ed12f0, fn_7001ee0d00 as *const (), 0, 0x10d00), // +0x12f0 sub_12f0 (thunk -> +0x10d00)
    Lifted::alias(0x7001ed1300, tpl_26::<0x18> as *const (), 1, 0xac40), // +0x1300 ?scope@TraceObject@tracing@platform@v8@@QEBAPEBDXZ (thunk -> +0xac40)
    Lifted::alias(0x7001ed1310, fn_7001ee9320 as *const (), 1, 0x19320), // +0x1310 sub_1310 (thunk -> +0x19320)
    Lifted::alias(0x7001ed1320, fn_7001ee40c0 as *const (), 0, 0x140c0), // +0x1320 sub_1320 (thunk -> +0x140c0)
    Lifted::alias(0x7001ed1330, fn_7001ee58e0 as *const (), 0, 0x158e0), // +0x1330 sub_1330 (thunk -> +0x158e0)
    Lifted::alias(0x7001ed1340, tpl_45::<0x7001eeeed0, 0x7001efd678, 0x7001efd6f0, 0x7001eea7e2> as *const (), 0, 0xf360), // +0x1340 sub_1340 (thunk -> +0xf360)
    Lifted::alias(0x7001ed1360, tpl_12 as *const (), 1, 0xa0e0), // +0x1360 sub_1360 (thunk -> +0xa0e0)
    Lifted::alias(0x7001ed1390, tpl_27::<0x10> as *const (), 1, 0xdb00), // +0x1390 ?NumberOfWorkerThreads@DefaultPlatform@platform@v8@@UEAAHXZ (thunk -> +0xdb00)
    Lifted::alias(0x7001ed13a0, fn_7001ed8420 as *const (), 0, 0x8420), // +0x13a0 sub_13a0 (thunk -> +0x8420)
    Lifted::alias(0x7001ed13b0, fn_7001ed8b40 as *const (), 0, 0x8b40), // +0x13b0 ??1TraceBufferChunk@tracing@platform@v8@@QEAA@XZ (thunk -> +0x8b40)
    Lifted::alias(0x7001ed13e0, fn_7001edbce0 as *const (), 1, 0xbce0), // +0x13e0 ?IsValid@DefaultJobHandle@platform@v8@@UEAA_NXZ (thunk -> +0xbce0)
    Lifted::alias(0x7001ed13f0, fn_7001edbbf0 as *const (), 0, 0xbbf0), // +0x13f0 ?IsActive@DefaultJobHandle@platform@v8@@UEAA_NXZ (thunk -> +0xbbf0)
    Lifted::alias(0x7001ed1410, tpl_31::<0x8, 0x7001ed2c10, 0x10, 0xc> as *const (), 0, 0xb7d0), // +0x1410 ?Cancel@DefaultJobHandle@platform@v8@@UEAAXXZ (thunk -> +0xb7d0)
    Lifted::alias(0x7001ed1420, fn_7001edca80 as *const (), 0, 0xca80), // +0x1420 ??1DefaultPlatform@platform@v8@@UEAA@XZ (thunk -> +0xca80)
    Lifted::alias(0x7001ed1430, tpl_14::<0x7001eea7e2> as *const (), 0, 0x16fb0), // +0x1430 sub_1430 (thunk -> +0x16fb0)
    Lifted::alias(0x7001ed1440, fn_7001edb2e0 as *const (), 0, 0xb2e0), // +0x1440 sub_1440 (thunk -> +0xb2e0)
    Lifted::alias(0x7001ed1450, tpl_19::<0xa0> as *const (), 2, 0x9150), // +0x1450 ?MonotonicallyIncreasingTime@DefaultForegroundTaskRunner@platform@v8@@QEAANXZ (thunk -> +0x9150)
    Lifted::alias(0x7001ed1460, fn_7001ed8500 as *const (), 1, 0x8500), // +0x1460 ??0TraceConfig@tracing@platform@v8@@QEAA@XZ (thunk -> +0x8500)
    Lifted::alias(0x7001ed1470, tpl_32::<0x8, 0x7001ed2f20> as *const (), 0, 0xc090), // +0x1470 sub_1470 (thunk -> +0xc090)
    Lifted::alias(0x7001ed1480, tpl_25::<0x7001eeeab0, 0x7001ee88f0, 0x7001ee0cf0> as *const (), 0, 0x10ce0), // +0x1480 sub_1480 (thunk -> +0x10ce0)
    Lifted::alias(0x7001ed14b0, fn_7001edc0a0 as *const (), 1, 0xc0a0), // +0x14b0 ?ReleaseTaskId@DefaultJobState@platform@v8@@QEAAXE@Z (thunk -> +0xc0a0)
    Lifted::alias(0x7001ed14c0, fn_7001ee3710 as *const (), 1, 0x13710), // +0x14c0 sub_14c0 (thunk -> +0x13710)
    Lifted::alias(0x7001ed14d0, fn_7001edee70 as *const (), 0, 0xee70), // +0x14d0 ??0DefaultWorkerThreadsTaskRunner@platform@v8@@QEAA@IP6ANXZ@Z (thunk -> +0xee70)
    Lifted::alias(0x7001ed14f0, tpl_37 as *const (), 1, 0xe6e0), // +0x14f0 sub_14f0 (thunk -> +0xe6e0)
    Lifted::alias(0x7001ed1500, fn_7001eebec0 as *const (), 2, 0x1bec0), // +0x1500 sub_1500 (thunk -> +0x1bec0)
    Lifted::alias(0x7001ed1520, fn_7001eea4c0 as *const (), 0, 0x1a4c0), // +0x1520 sub_1520 (thunk -> +0x1a4c0)
    Lifted::alias(0x7001ed1530, fn_7001ee72a0 as *const (), 0, 0x172a0), // +0x1530 ?AddTraceStateObserver@TracingController@tracing@platform@v8@@UEAAXPEAVTraceStateObserver@14@@Z (thunk -> +0x172a0)
    Lifted::alias(0x7001ed1540, tpl_41::<0x10, 0x19, 0x8> as *const (), 1, 0xea10), // +0x1540 sub_1540 (thunk -> +0xea10)
    Lifted::alias(0x7001ed1550, fn_7001ee8740 as *const (), 0, 0x18740), // +0x1550 ?IsEnabled@Recorder@tracing@platform@v8@@QEAA_NXZ (thunk -> +0x18740)
    Lifted::alias(0x7001ed1570, fn_7001ee8420 as *const (), 0, 0x18420), // +0x1570 ?AddEvent@Recorder@tracing@platform@v8@@QEAAXPEAVTraceObject@234@@Z (thunk -> +0x18420)
    Lifted::alias(0x7001ed1580, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0xb5a0), // +0x1580 sub_1580 (thunk -> +0xb5a0)
    Lifted::alias(0x7001ed1590, tpl_8::<0x7001eee998> as *const (), 1, 0x8550), // +0x1590 ??0TraceWriter@tracing@platform@v8@@QEAA@XZ (thunk -> +0x8550)
    Lifted::alias(0x7001ed15a0, fn_7001ed9c20 as *const (), 0, 0x9c20), // +0x15a0 ?PostTask@DefaultForegroundTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@ (thunk -> +0x9c20)
    Lifted::alias(0x7001ed15b0, tpl_13::<0xfffffffffffffff8, 0x7001eea7e2> as *const (), 0, 0x8d00), // +0x15b0 sub_15b0 (thunk -> +0x8d00)
    Lifted::alias(0x7001ed15e0, fn_7001edaad0 as *const (), 2, 0xaad0), // +0x15e0 sub_15e0 (thunk -> +0xaad0)
    Lifted::alias(0x7001ed15f0, fn_7001ee1700 as *const (), 0, 0x11700), // +0x15f0 sub_15f0 (thunk -> +0x11700)
    Lifted::alias(0x7001ed1600, fn_7001ee8730 as *const (), 1, 0x18730), // +0x1600 ?IsEnabled@Recorder@tracing@platform@v8@@QEAA_NE@Z (thunk -> +0x18730)
    Lifted::alias(0x7001ed1640, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0x8f00), // +0x1640 sub_1640 (thunk -> +0x8f00)
    Lifted::alias(0x7001ed1660, fn_7001eebea0 as *const (), 2, 0x1bea0), // +0x1660 sub_1660 (thunk -> +0x1bea0)
    Lifted::alias(0x7001ed1670, fn_7001edd570 as *const (), 2, 0xd570), // +0x1670 ?MonotonicallyIncreasingTime@DefaultPlatform@platform@v8@@UEAANXZ (thunk -> +0xd570)
    Lifted::alias(0x7001ed1690, tpl_43::<0x7001eef460, 0x7001efd6e8, 0x7001eef448, 0x30, 0x7001efd700, 0x7001eeef90, 0x7001eeea80, 0x7001efd778, 0x7001ee80de> as *const (), 0, 0x18080), // +0x1690 ??0WorkerThread@platform@v8@@QEAA@PEAVTaskQueue@12@@Z (thunk -> +0x18080)
    Lifted::alias(0x7001ed16c0, fn_7001edfaa0 as *const (), 0, 0xfaa0), // +0x16c0 ??1DelayedTaskQueue@platform@v8@@QEAA@XZ (thunk -> +0xfaa0)
    Lifted::alias(0x7001ed16e0, tpl_25::<0x7001eeeab0, 0x7001ee88f0, 0x7001eda680> as *const (), 0, 0xa670), // +0x16e0 sub_16e0 (thunk -> +0xa670)
    Lifted::alias(0x7001ed1710, fn_7001ee0e10 as *const (), 0, 0x10e10), // +0x1710 ??0TaskQueue@platform@v8@@QEAA@XZ (thunk -> +0x10e10)
    Lifted::alias(0x7001ed1720, fn_7001edde30 as *const (), 2, 0xde30), // +0x1720 ?RunIdleTasks@DefaultPlatform@platform@v8@@QEAAXPEAVIsolate@3@N@Z (thunk -> +0xde30)
    Lifted::alias(0x7001ed1730, fn_7001ee6a40 as *const (), 0, 0x16a40), // +0x1730 ??0TracingController@tracing@platform@v8@@QEAA@XZ (thunk -> +0x16a40)
    Lifted::alias(0x7001ed1740, fn_7001edf440 as *const (), 2, 0xf440), // +0x1740 ?PostDelayedTask@DefaultWorkerThreadsTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@s (thunk -> +0xf440)
    Lifted::alias(0x7001ed1750, tpl_12 as *const (), 1, 0x8b30), // +0x1750 ??1TraceBuffer@tracing@platform@v8@@UEAA@XZ (thunk -> +0x8b30)
    Lifted::alias(0x7001ed1760, fn_7001ee4f40 as *const (), 0, 0x14f40), // +0x1760 sub_1760 (thunk -> +0x14f40)
    Lifted::alias(0x7001ed1790, tpl_46::<0x7001efd760, 0x48, 0x7001efd668, 0x8, 0x7001efd758> as *const (), 0, 0x101c0), // +0x1790 ?Terminate@DelayedTaskQueue@platform@v8@@QEAAXXZ (thunk -> +0x101c0)
    Lifted::alias(0x7001ed17b0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0x8ed0), // +0x17b0 sub_17b0 (thunk -> +0x8ed0)
    Lifted::alias(0x7001ed17d0, tpl_26::<0x40> as *const (), 1, 0xd500), // +0x17d0 ?GetPageAllocator@DefaultPlatform@platform@v8@@UEAAPEAVPageAllocator@3@XZ (thunk -> +0xd500)
    Lifted::alias(0x7001ed17e0, fn_7001eea190 as *const (), 0, 0x1a190), // +0x17e0 sub_17e0 (thunk -> +0x1a190)
    Lifted::alias(0x7001ed17f0, fn_7001edd960 as *const (), 0, 0xd960), // +0x17f0 ?NotifyIsolateShutdown@platform@v8@@YAXPEAVPlatform@2@PEAVIsolate@2@@Z (thunk -> +0xd960)
    Lifted::alias(0x7001ed1810, tpl_21 as *const (), 1, 0x9280), // +0x1810 ?NonNestableTasksEnabled@DefaultForegroundTaskRunner@platform@v8@@UEBA_NXZ (thunk -> +0x9280)
    Lifted::alias(0x7001ed1820, tpl_14::<0x7001eea7e2> as *const (), 0, 0x8e40), // +0x1820 sub_1820 (thunk -> +0x8e40)
    Lifted::alias(0x7001ed1830, fn_7001ee4d20 as *const (), 0, 0x14d20), // +0x1830 ??1TraceData@tracing@platform@v8@@QEAA@XZ (thunk -> +0x14d20)
    Lifted::alias(0x7001ed1840, tpl_27::<0x2a08> as *const (), 1, 0xac50), // +0x1840 ?seq@TraceBufferChunk@tracing@platform@v8@@QEBAIXZ (thunk -> +0xac50)
    Lifted::alias(0x7001ed1850, fn_7001ee3560 as *const (), 1, 0x13560), // +0x1850 sub_1850 (thunk -> +0x13560)
    Lifted::alias(0x7001ed1870, tpl_26::<0x20> as *const (), 1, 0xa720), // +0x1870 ?category_enabled_flag@TraceObject@tracing@platform@v8@@QEBAPEBEXZ (thunk -> +0xa720)
    Lifted::alias(0x7001ed1890, tpl_50::<0xffffffffffffff68, 0x4, 0x7001eef070, 0xffffffffffffff64, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd228, 0x7001efd250, 0x7001eea7e2> as *const (), 0, 0x127a0), // +0x1890 sub_1890 (thunk -> +0x127a0)
    Lifted::alias(0x7001ed18a0, tpl_40 as *const (), 1, 0xe6f0), // +0x18a0 sub_18a0 (thunk -> +0xe6f0)
    Lifted::alias(0x7001ed18c0, fn_7001edbd70 as *const (), 0, 0xbd70), // +0x18c0 ?Join@DefaultJobState@platform@v8@@QEAAXXZ (thunk -> +0xbd70)
    Lifted::alias(0x7001ed18e0, fn_7001ed96b0 as *const (), 2, 0x96b0), // +0x18e0 ?PostDelayedTask@DefaultForegroundTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@ (thunk -> +0x96b0)
    Lifted::alias(0x7001ed18f0, tpl_40 as *const (), 1, 0x108e0), // +0x18f0 sub_18f0 (thunk -> +0x108e0)
    Lifted::alias(0x7001ed1900, tpl_26::<0x28> as *const (), 1, 0xaa80), // +0x1900 ?id@TraceObject@tracing@platform@v8@@QEBA_KXZ (thunk -> +0xaa80)
    Lifted::alias(0x7001ed1910, fn_7001ee32e0 as *const (), 0, 0x132e0), // +0x1910 sub_1910 (thunk -> +0x132e0)
    Lifted::alias(0x7001ed1920, fn_7001ee1ca0 as *const (), 1, 0x11ca0), // +0x1920 ?Reset@TraceBufferChunk@tracing@platform@v8@@QEAAXI@Z (thunk -> +0x11ca0)
    Lifted::alias(0x7001ed1930, fn_7001ee6090 as *const (), 2, 0x16090), // +0x1930 sub_1930 (thunk -> +0x16090)
    Lifted::alias(0x7001ed1940, fn_7001edc4e0 as *const (), 0, 0xc4e0), // +0x1940 ?weak_from_this@?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@QEBA?AV?$weak_ptr@$$CBVDefaultJobState@pla (thunk -> +0xc4e0)
    Lifted::alias(0x7001ed1960, tpl_33::<0x8, 0x1, 0x7001edc3b0, 0x7001edc48d> as *const (), 0, 0xc440), // +0x1960 ?shared_from_this@?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@QEBA?AV?$shared_ptr@$$CBVDefaultJobState (thunk -> +0xc440)
    Lifted::alias(0x7001ed1970, tpl_34::<0x7001eeeb18> as *const (), 1, 0xc5b0), // +0x1970 sub_1970 (thunk -> +0xc5b0)
    Lifted::alias(0x7001ed1980, fn_7001edbbc0 as *const (), 0, 0xbbc0), // +0x1980 sub_1980 (thunk -> +0xbbc0)
    Lifted::alias(0x7001ed1990, tpl_12 as *const (), 1, 0x17330), // +0x1990 sub_1990 (thunk -> +0x17330)
    Lifted::alias(0x7001ed19a0, tpl_16::<0x40> as *const (), 1, 0xa6e0), // +0x19a0 ?arg_names@TraceObject@tracing@platform@v8@@QEAAPEAPEBDXZ (thunk -> +0xa6e0)
    Lifted::alias(0x7001ed19b0, tpl_12 as *const (), 1, 0xdb10), // +0x19b0 sub_19b0 (thunk -> +0xdb10)
    Lifted::alias(0x7001ed19d0, fn_7001edf890 as *const (), 0, 0xf890), // +0x19d0 ??0DelayedTaskQueue@platform@v8@@QEAA@P6ANXZ@Z (thunk -> +0xf890)
    Lifted::alias(0x7001ed19f0, fn_7001ee1040 as *const (), 0, 0x11040), // +0x19f0 ?BlockUntilQueueEmptyForTesting@TaskQueue@platform@v8@@AEAAXXZ (thunk -> +0x11040)
    Lifted::alias(0x7001ed1a00, fn_7001edb5d0 as *const (), 1, 0xb5d0), // +0x1a00 ?AcquireTaskId@DefaultJobState@platform@v8@@QEAAEXZ (thunk -> +0xb5d0)
    Lifted::alias(0x7001ed1a10, fn_7001edf4e0 as *const (), 0, 0xf4e0), // +0x1a10 ?PostIdleTask@DefaultWorkerThreadsTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VIdleTask@v8@@U?$default_delete@VIdleTask@v (thunk -> +0xf4e0)
    Lifted::alias(0x7001ed1a20, fn_7001edb670 as *const (), 0, 0xb670), // +0x1a20 ?CallOnWorkerThread@DefaultJobState@platform@v8@@AEAAXW4TaskPriority@3@V?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v (thunk -> +0xb670)
    Lifted::alias(0x7001ed1a30, fn_7001ee10a0 as *const (), 0, 0x110a0), // +0x1a30 ?GetNext@TaskQueue@platform@v8@@QEAA?AV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@XZ (thunk -> +0x110a0)
    Lifted::alias(0x7001ed1a40, fn_7001ed92a0 as *const (), 2, 0x92a0), // +0x1a40 ?PopTaskFromDelayedQueueLocked@DefaultForegroundTaskRunner@platform@v8@@AEAA?AV?$unique_ptr@VTask@v8@@U?$default_delete@ (thunk -> +0x92a0)
    Lifted::alias(0x7001ed1a60, tpl_29 as *const (), 1, 0xb170), // +0x1a60 ??4?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@IEAAAEAV01@AEBV01@@Z (thunk -> +0xb170)
    Lifted::alias(0x7001ed1a80, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0x8f30), // +0x1a80 sub_1a80 (thunk -> +0x8f30)
    Lifted::alias(0x7001ed1a90, fn_7001ed8590 as *const (), 0, 0x8590), // +0x1a90 sub_1a90 (thunk -> +0x8590)
    Lifted::alias(0x7001ed1aa0, tpl_9::<0x8> as *const (), 1, 0x8520), // +0x1aa0 ??0TraceData@tracing@platform@v8@@QEAA@XZ (thunk -> +0x8520)
    Lifted::alias(0x7001ed1ad0, tpl_51::<0x100, 0x8> as *const (), 0, 0x158c0), // +0x1ad0 sub_1ad0 (thunk -> +0x158c0)
    Lifted::alias(0x7001ed1ae0, tpl_17 as *const (), 1, 0x9090), // +0x1ae0 ?GetTraceRecordMode@TraceConfig@tracing@platform@v8@@QEBA?AW4TraceRecordMode@234@XZ (thunk -> +0x9090)
    Lifted::alias(0x7001ed1af0, tpl_15::<0x7001eeee80, 0x7001eea7e2> as *const (), 0, 0xcec0), // +0x1af0 sub_1af0 (thunk -> +0xcec0)
    Lifted::alias(0x7001ed1b00, tpl_26::<0x90> as *const (), 1, 0xac90), // +0x1b00 ?tts@TraceObject@tracing@platform@v8@@QEAA_JXZ (thunk -> +0xac90)
    Lifted::alias(0x7001ed1b10, fn_7001ed7dc0 as *const (), 2, 0x7dc0), // +0x1b10 sub_1b10 (thunk -> +0x7dc0)
    Lifted::alias(0x7001ed1b20, fn_7001edb060 as *const (), 0, 0xb060), // +0x1b20 ??1DefaultJobState@platform@v8@@UEAA@XZ (thunk -> +0xb060)
    Lifted::alias(0x7001ed1b30, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0, 0x19210), // +0x1b30 sub_1b30 (thunk -> +0x19210)
    Lifted::alias(0x7001ed1b50, fn_7001ee4ee0 as *const (), 0, 0x14ee0), // +0x1b50 sub_1b50 (thunk -> +0x14ee0)
    Lifted::alias(0x7001ed1b70, tpl_15::<0x7001eeeea8, 0x7001eea7e2> as *const (), 0, 0xce60), // +0x1b70 sub_1b70 (thunk -> +0xce60)
    Lifted::alias(0x7001ed1b80, fn_7001edf3c0 as *const (), 0, 0xf3c0), // +0x1b80 ?GetNext@DefaultWorkerThreadsTaskRunner@platform@v8@@AEAA?AV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@st (thunk -> +0xf3c0)
    Lifted::alias(0x7001ed1b90, fn_7001ee7380 as *const (), 0, 0x17380), // +0x1b90 ?GetCategoryGroupEnabled@TracingController@tracing@platform@v8@@UEAAPEBEPEBD@Z (thunk -> +0x17380)
    Lifted::alias(0x7001ed1ba0, fn_7001ee45c0 as *const (), 0, 0x145c0), // +0x1ba0 sub_1ba0 (thunk -> +0x145c0)
    Lifted::alias(0x7001ed1bb0, fn_7001ee1d90 as *const (), 2, 0x11d90), // +0x1bb0 sub_1bb0 (thunk -> +0x11d90)
    Lifted::alias(0x7001ed1bc0, fn_7001edd250 as *const (), 2, 0xd250), // +0x1bc0 ?GetForegroundTaskRunner@DefaultPlatform@platform@v8@@UEAA?AV?$shared_ptr@VTaskRunner@v8@@@std@@PEAVIsolate@3@@Z (thunk -> +0xd250)
    Lifted::alias(0x7001ed1bd0, fn_7001eea4c0 as *const (), 0, 0x1a4c0), // +0x1bd0 sub_1bd0 (thunk -> +0x1a4c0)
    Lifted::alias(0x7001ed1bf0, fn_7001ee7af0 as *const (), 0, 0x17af0), // +0x1bf0 ?UpdateCategoryGroupEnabledFlags@TracingController@tracing@platform@v8@@AEAAXXZ (thunk -> +0x17af0)
    Lifted::alias(0x7001ed1c20, fn_7001ed9030 as *const (), 1, 0x9030), // +0x1c20 ?EnableArgumentFilter@TraceConfig@tracing@platform@v8@@QEAAXXZ (thunk -> +0x9030)
    Lifted::alias(0x7001ed1c30, fn_7001ee7530 as *const (), 1, 0x17530), // +0x1c30 ?GetCategoryGroupName@TracingController@tracing@platform@v8@@SAPEBDPEBE@Z (thunk -> +0x17530)
    Lifted::alias(0x7001ed1c40, tpl_20 as *const (), 1, 0xc2e0), // +0x1c40 sub_1c40 (thunk -> +0xc2e0)
    Lifted::alias(0x7001ed1c50, fn_7001edb180 as *const (), 0, 0xb180), // +0x1c50 sub_1c50 (thunk -> +0xb180)
    Lifted::alias(0x7001ed1c70, fn_7001edc210 as *const (), 1, 0xc210), // +0x1c70 sub_1c70 (thunk -> +0xc210)
    Lifted::alias(0x7001ed1ca0, tpl_52::<0x7001efd630> as *const (), 0, 0x17340), // +0x1ca0 ?CurrentCpuTimestampMicroseconds@TracingController@tracing@platform@v8@@MEAA_JXZ (thunk -> +0x17340)
    Lifted::alias(0x7001ed1cb0, fn_7001ee1530 as *const (), 1, 0x11530), // +0x1cb0 ??0TraceBufferChunk@tracing@platform@v8@@QEAA@I@Z (thunk -> +0x11530)
    Lifted::alias(0x7001ed1ce0, fn_7001ed9aa0 as *const (), 2, 0x9aa0), // +0x1ce0 ?PostNonNestableDelayedTask@DefaultForegroundTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTas (thunk -> +0x9aa0)
    Lifted::alias(0x7001ed1cf0, fn_7001eda0b0 as *const (), 0, 0xa0b0), // +0x1cf0 sub_1cf0 (thunk -> +0xa0b0)
    Lifted::alias(0x7001ed1d10, fn_7001ede230 as *const (), 0, 0xe230), // +0x1d10 sub_1d10 (thunk -> +0xe230)
    Lifted::alias(0x7001ed1d20, fn_7001ee8a40 as *const (), 0, 0x18a40), // +0x1d20 sub_1d20 (thunk -> +0x18a40)
    Lifted::alias(0x7001ed1d30, tpl_26::<0x98> as *const (), 1, 0xa740), // +0x1d30 ?duration@TraceObject@tracing@platform@v8@@QEAA_KXZ (thunk -> +0xa740)
    Lifted::alias(0x7001ed1d40, tpl_19::<0x88> as *const (), 2, 0xf430), // +0x1d40 ?MonotonicallyIncreasingTime@DefaultWorkerThreadsTaskRunner@platform@v8@@QEAANXZ (thunk -> +0xf430)
    Lifted::alias(0x7001ed1d50, tpl_15::<0x7001eeeea8, 0x7001eea7e2> as *const (), 0, 0xce60), // +0x1d50 sub_1d50 (thunk -> +0xce60)
    Lifted::alias(0x7001ed1d60, fn_7001ee7e10 as *const (), 0, 0x17e10), // +0x1d60 sub_1d60 (thunk -> +0x17e10)
    Lifted::alias(0x7001ed1d70, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001eda6c0> as *const (), 0, 0xa6b0), // +0x1d70 sub_1d70 (thunk -> +0xa6b0)
    Lifted::alias(0x7001ed1d80, fn_7001ee30d0 as *const (), 0, 0x130d0), // +0x1d80 sub_1d80 (thunk -> +0x130d0)
    Lifted::alias(0x7001ed1d90, fn_7001ee1700 as *const (), 0, 0x11700), // +0x1d90 sub_1d90 (thunk -> +0x11700)
    Lifted::alias(0x7001ed1dc0, tpl_32::<0x8, 0x7001ed2f20> as *const (), 0, 0xbe80), // +0x1dc0 ?NotifyConcurrencyIncrease@DefaultJobHandle@platform@v8@@UEAAXXZ (thunk -> +0xbe80)
    Lifted::alias(0x7001ed1dd0, tpl_27::<0x4> as *const (), 1, 0xac70), // +0x1dd0 ?tid@TraceObject@tracing@platform@v8@@QEBAHXZ (thunk -> +0xac70)
    Lifted::alias(0x7001ed1de0, tpl_37 as *const (), 1, 0x170e0), // +0x1de0 sub_1de0 (thunk -> +0x170e0)
    Lifted::alias(0x7001ed1df0, fn_7001eda0f0 as *const (), 0, 0xa0f0), // +0x1df0 sub_1df0 (thunk -> +0xa0f0)
    Lifted::alias(0x7001ed1e00, fn_7001eddb30 as *const (), 0, 0xdb30), // +0x1e00 ?PostJob@DefaultPlatform@platform@v8@@UEAA?AV?$unique_ptr@VJobHandle@v8@@U?$default_delete@VJobHandle@v8@@@std@@@std@@W4 (thunk -> +0xdb30)
    Lifted::alias(0x7001ed1e40, tpl_13::<0xfffffffffffffff8, 0x7001eea7e2> as *const (), 0, 0x8d50), // +0x1e40 sub_1e40 (thunk -> +0x8d50)
    Lifted::alias(0x7001ed1e50, fn_7001ee68a0 as *const (), 1, 0x168a0), // +0x1e50 sub_1e50 (thunk -> +0x168a0)
    Lifted::alias(0x7001ed1e60, tpl_41::<0x10, 0x19, 0x8> as *const (), 1, 0x10c00), // +0x1e60 sub_1e60 (thunk -> +0x10c00)
    Lifted::alias(0x7001ed1e70, tpl_12 as *const (), 1, 0xd160), // +0x1e70 sub_1e70 (thunk -> +0xd160)
    Lifted::alias(0x7001ed1e80, tpl_19::<0x50> as *const (), 2, 0x100e0), // +0x1e80 ?MonotonicallyIncreasingTime@DelayedTaskQueue@platform@v8@@QEAANXZ (thunk -> +0x100e0)
    Lifted::alias(0x7001ed1ea0, fn_7001ee1240 as *const (), 0, 0x11240), // +0x1ea0 sub_1ea0 (thunk -> +0x11240)
    Lifted::alias(0x7001ed1eb0, fn_7001ed9050 as *const (), 1, 0x9050), // +0x1eb0 ?Finalize@TraceWriter@tracing@platform@v8@@UEAA?AVTraceData@234@XZ (thunk -> +0x9050)
    Lifted::alias(0x7001ed1ef0, fn_7001ee1790 as *const (), 0, 0x11790), // +0x1ef0 sub_1ef0 (thunk -> +0x11790)
    Lifted::alias(0x7001ed1f00, fn_7001ed8b90 as *const (), 0, 0x8b90), // +0x1f00 ??1TraceConfig@tracing@platform@v8@@QEAA@XZ (thunk -> +0x8b90)
    Lifted::alias(0x7001ed1f10, fn_7001ee8b60 as *const (), 0, 0x18b60), // +0x1f10 sub_1f10 (thunk -> +0x18b60)
    Lifted::alias(0x7001ed1f20, tpl_14::<0x7001eea7e2> as *const (), 0, 0x8e40), // +0x1f20 sub_1f20 (thunk -> +0x8e40)
    Lifted::alias(0x7001ed1f30, fn_7001ed8300 as *const (), 0, 0x8300), // +0x1f30 ??0DefaultForegroundTaskRunner@platform@v8@@QEAA@W4IdleTaskSupport@12@P6ANXZ@Z (thunk -> +0x8300)
    Lifted::alias(0x7001ed1f40, fn_7001ee70f0 as *const (), 0, 0x170f0), // +0x1f40 ?AddTraceEventWithTimestamp@TracingController@tracing@platform@v8@@UEAA_KDPEBEPEBD1_K2HPEAPEBD0PEB_KPEAV?$unique_ptr@VCo (thunk -> +0x170f0)
    Lifted::alias(0x7001ed1f50, tpl_50::<0xffffffffffffff78, 0x4, 0x7001eef148, 0xffffffffffffff74, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd1c8, 0x7001efd250, 0x7001eea7e2> as *const (), 0, 0x14d80), // +0x1f50 sub_1f50 (thunk -> +0x14d80)
    Lifted::alias(0x7001ed1f60, fn_7001ee2190 as *const (), 2, 0x12190), // +0x1f60 sub_1f60 (thunk -> +0x12190)
    Lifted::alias(0x7001ed1f90, tpl_25::<0x7001eeefc8, 0x7001ee88f0, 0x7001ee3030> as *const (), 0, 0x13020), // +0x1f90 sub_1f90 (thunk -> +0x13020)
    Lifted::alias(0x7001ed1fa0, fn_7001ee8340 as *const (), 2, 0x18340), // +0x1fa0 ??0Recorder@tracing@platform@v8@@QEAA@XZ (thunk -> +0x18340)
    Lifted::alias(0x7001ed1fe0, fn_7001ee6eb0 as *const (), 0, 0x16eb0), // +0x1fe0 sub_1fe0 (thunk -> +0x16eb0)
    Lifted::alias(0x7001ed1ff0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0xb5a0), // +0x1ff0 sub_1ff0 (thunk -> +0xb5a0)
    Lifted::alias(0x7001ed2010, fn_7001ed8ab0 as *const (), 0, 0x8ab0), // +0x2010 sub_2010 (thunk -> +0x8ab0)
    Lifted::alias(0x7001ed2020, tpl_14::<0x7001eea7e2> as *const (), 0, 0xb570), // +0x2020 sub_2020 (thunk -> +0xb570)
    Lifted::alias(0x7001ed2030, fn_7001ed9de0 as *const (), 2, 0x9de0), // +0x2030 ?Terminate@DefaultForegroundTaskRunner@platform@v8@@QEAAXXZ (thunk -> +0x9de0)
    Lifted::alias(0x7001ed2040, fn_7001ee3df0 as *const (), 1, 0x13df0), // +0x2040 ?InitializeForTesting@TraceObject@tracing@platform@v8@@QEAAXDPEBEPEBD1_K2HPEAPEBD0PEB_KPEAV?$unique_ptr@VConvertableToTr (thunk -> +0x13df0)
    Lifted::alias(0x7001ed2050, tpl_49::<0xfffffffffffffffc, 0x7001ee27a0> as *const (), 0, 0x12750), // +0x2050 sub_2050 (thunk -> +0x12750)
    Lifted::alias(0x7001ed2070, tpl_16::<0x58> as *const (), 1, 0xa700), // +0x2070 ?arg_values@TraceObject@tracing@platform@v8@@QEAAPEATArgValue@1234@XZ (thunk -> +0xa700)
    Lifted::alias(0x7001ed2080, tpl_20 as *const (), 1, 0x9290), // +0x2080 sub_2080 (thunk -> +0x9290)
    Lifted::alias(0x7001ed2090, fn_7001ed8070 as *const (), 2, 0x8070), // +0x2090 sub_2090 (thunk -> +0x8070)
    Lifted::alias(0x7001ed20a0, fn_7001ed9040 as *const (), 1, 0x9040), // +0x20a0 ?EnableSystrace@TraceConfig@tracing@platform@v8@@QEAAXXZ (thunk -> +0x9040)
    Lifted::alias(0x7001ed20d0, tpl_33::<0x8, 0x1, 0x7001edc3b0, 0x7001edc41d> as *const (), 0, 0xc3d0), // +0x20d0 ?shared_from_this@?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@QEAA?AV?$shared_ptr@VDefaultJobState@pla (thunk -> +0xc3d0)
    Lifted::alias(0x7001ed20e0, fn_7001ee38c0 as *const (), 0, 0x138c0), // +0x20e0 ??1TraceObject@tracing@platform@v8@@QEAA@XZ (thunk -> +0x138c0)
    Lifted::alias(0x7001ed2130, fn_7001ee7be0 as *const (), 0, 0x17be0), // +0x2130 sub_2130 (thunk -> +0x17be0)
    Lifted::alias(0x7001ed2140, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001edf6f0> as *const (), 0, 0xf6e0), // +0x2140 sub_2140 (thunk -> +0xf6e0)
    Lifted::alias(0x7001ed21a0, fn_7001edd000 as *const (), 0, 0xd000), // +0x21a0 ?CallDelayedOnWorkerThread@DefaultPlatform@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@ (thunk -> +0xd000)
    Lifted::alias(0x7001ed21b0, tpl_10::<0x7001eeeb08, 0x8, 0x7001eeeae8> as *const (), 1, 0xafa0), // +0x21b0 sub_21b0 (thunk -> +0xafa0)
    Lifted::alias(0x7001ed21c0, fn_7001edb4e0 as *const (), 0, 0xb4e0), // +0x21c0 sub_21c0 (thunk -> +0xb4e0)
    Lifted::alias(0x7001ed21d0, fn_7001ee00f0 as *const (), 2, 0x100f0), // +0x21d0 ?PopTaskFromDelayedQueue@DelayedTaskQueue@platform@v8@@AEAA?AV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@ (thunk -> +0x100f0)
    Lifted::alias(0x7001ed21f0, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0, 0x191b0), // +0x21f0 sub_21f0 (thunk -> +0x191b0)
    Lifted::alias(0x7001ed2200, tpl_30::<0x7001ee25f0, 0x7001eea7e2> as *const (), 0, 0x12760), // +0x2200 sub_2200 (thunk -> +0x12760)
    Lifted::alias(0x7001ed2210, fn_7001edbc60 as *const (), 0, 0xbc60), // +0x2210 ?IsActive@DefaultJobState@platform@v8@@QEAA_NXZ (thunk -> +0xbc60)
    Lifted::alias(0x7001ed2220, tpl_51::<0x100, 0x10> as *const (), 0, 0x162a0), // +0x2220 sub_2220 (thunk -> +0x162a0)
    Lifted::alias(0x7001ed2230, fn_7001edaff0 as *const (), 0, 0xaff0), // +0x2230 ??1DefaultJobHandle@platform@v8@@UEAA@XZ (thunk -> +0xaff0)
    Lifted::alias(0x7001ed2240, tpl_22 as *const (), 0, 0x9c00), // +0x2240 sub_2240 (thunk -> +0x9c00)
    Lifted::alias(0x7001ed2250, tpl_17 as *const (), 1, 0xaac0), // +0x2250 ?pid@TraceObject@tracing@platform@v8@@QEBAHXZ (thunk -> +0xaac0)
    Lifted::alias(0x7001ed2260, fn_7001ee2970 as *const (), 2, 0x12970), // +0x2260 ?CreateDefaultTraceConfig@TraceConfig@tracing@platform@v8@@SAPEAV1234@XZ (thunk -> +0x12970)
    Lifted::alias(0x7001ed2280, fn_7001edf740 as *const (), 0, 0xf740), // +0x2280 sub_2280 (thunk -> +0xf740)
    Lifted::alias(0x7001ed2290, fn_7001edac60 as *const (), 1, 0xac60), // +0x2290 ?size@TraceBufferChunk@tracing@platform@v8@@QEBA_KXZ (thunk -> +0xac60)
    Lifted::alias(0x7001ed22a0, fn_7001ee4ee0 as *const (), 0, 0x14ee0), // +0x22a0 sub_22a0 (thunk -> +0x14ee0)
    Lifted::alias(0x7001ed22b0, tpl_16::<0x8> as *const (), 1, 0x9060), // +0x22b0 ?GetEnabledCategories@TraceConfig@tracing@platform@v8@@QEBAAEBV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$alloca (thunk -> +0x9060)
    Lifted::alias(0x7001ed22c0, tpl_26::<0x88> as *const (), 1, 0xac80), // +0x22c0 ?ts@TraceObject@tracing@platform@v8@@QEAA_JXZ (thunk -> +0xac80)
    Lifted::alias(0x7001ed22d0, tpl_9::<0x8> as *const (), 1, 0xade0), // +0x22d0 ??0?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@IEAA@AEBV01@@Z (thunk -> +0xade0)
    Lifted::alias(0x7001ed22f0, fn_7001ee69b0 as *const (), 0, 0x169b0), // +0x22f0 sub_22f0 (thunk -> +0x169b0)
    Lifted::alias(0x7001ed2310, fn_7001ee43c0 as *const (), 0, 0x143c0), // +0x2310 sub_2310 (thunk -> +0x143c0)
    Lifted::alias(0x7001ed2330, fn_7001ee5de0 as *const (), 0, 0x15de0), // +0x2330 ?CreateJSONTraceWriter@TraceWriter@tracing@platform@v8@@SAPEAV1234@AEAV?$basic_ostream@DU?$char_traits@D@std@@@std@@@Z (thunk -> +0x15de0)
    Lifted::alias(0x7001ed2340, tpl_39::<0x10, 0x40> as *const (), 0, 0xe240), // +0x2340 sub_2340 (thunk -> +0xe240)
    Lifted::alias(0x7001ed2360, tpl_12 as *const (), 1, 0xc2c0), // +0x2360 sub_2360 (thunk -> +0xc2c0)
    Lifted::alias(0x7001ed2370, fn_7001ed9760 as *const (), 2, 0x9760), // +0x2370 ?PostDelayedTaskLocked@DefaultForegroundTaskRunner@platform@v8@@AEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@ (thunk -> +0x9760)
    Lifted::alias(0x7001ed23a0, fn_7001ee4a70 as *const (), 0, 0x14a70), // +0x23a0 sub_23a0 (thunk -> +0x14a70)
    Lifted::alias(0x7001ed23b0, tpl_27::<0x38> as *const (), 1, 0xaaa0), // +0x23b0 ?num_args@TraceObject@tracing@platform@v8@@QEBAHXZ (thunk -> +0xaaa0)
    Lifted::new(0x7001ed23c0, fn_7001ed23c0 as *const (), 0), // +0x23c0 ?CurrentClockTimeMillis@DefaultPlatform@platform@v8@@UEAANXZ
    Lifted::alias(0x7001ed23f0, tpl_12 as *const (), 1, 0x162c0), // +0x23f0 sub_23f0 (thunk -> +0x162c0)
    Lifted::alias(0x7001ed2410, fn_7001ede260 as *const (), 0, 0xe260), // +0x2410 sub_2410 (thunk -> +0xe260)
    Lifted::alias(0x7001ed2460, fn_7001ee4be0 as *const (), 1, 0x14be0), // +0x2460 ??0TraceData@tracing@platform@v8@@QEAA@$$QEAV0123@@Z (thunk -> +0x14be0)
    Lifted::alias(0x7001ed2470, tpl_11::<0x7001eea7e2, 0x7001efd520, 0x7001ed87e3> as *const (), 0, 0x86e0), // +0x2470 sub_2470 (thunk -> +0x86e0)
    Lifted::alias(0x7001ed2480, fn_7001ee4d40 as *const (), 1, 0x14d40), // +0x2480 ??4TraceData@tracing@platform@v8@@QEAAAEAV0123@$$QEAV0123@@Z (thunk -> +0x14d40)
    Lifted::alias(0x7001ed2490, fn_7001eddc50 as *const (), 0, 0xdc50), // +0x2490 ?PumpMessageLoop@DefaultPlatform@platform@v8@@QEAA_NPEAVIsolate@3@W4MessageLoopBehavior@23@@Z (thunk -> +0xdc50)
    Lifted::alias(0x7001ed24a0, fn_7001edaca0 as *const (), 1, 0xaca0), // +0x24a0 sub_24a0 (thunk -> +0xaca0)
    Lifted::alias(0x7001ed24b0, tpl_34::<0x7001eddc00> as *const (), 1, 0xd520), // +0x24b0 ?GetStackTracePrinter@DefaultPlatform@platform@v8@@UEAAP6AXXZXZ (thunk -> +0xd520)
    Lifted::alias(0x7001ed24d0, fn_7001ed8850 as *const (), 0, 0x8850), // +0x24d0 sub_24d0 (thunk -> +0x8850)
    Lifted::alias(0x7001ed24e0, tpl_15::<0x7001eeee80, 0x7001eea7e2> as *const (), 0, 0xcec0), // +0x24e0 sub_24e0 (thunk -> +0xcec0)
    Lifted::alias(0x7001ed24f0, fn_7001ee1fd0 as *const (), 0, 0x11fd0), // +0x24f0 sub_24f0 (thunk -> +0x11fd0)
    Lifted::alias(0x7001ed2500, fn_7001ee3840 as *const (), 1, 0x13840), // +0x2500 sub_2500 (thunk -> +0x13840)
    Lifted::alias(0x7001ed2510, fn_7001eddc50 as *const (), 0, 0xdc50), // +0x2510 ?PumpMessageLoop@platform@v8@@YA_NPEAVPlatform@2@PEAVIsolate@2@W4MessageLoopBehavior@12@@Z (thunk -> +0xdc50)
    Lifted::alias(0x7001ed2530, fn_7001ed8da0 as *const (), 0, 0x8da0), // +0x2530 ??_FDefaultPlatform@platform@v8@@QEAAXXZ (thunk -> +0x8da0)
    Lifted::alias(0x7001ed2540, fn_7001ed9920 as *const (), 0, 0x9920), // +0x2540 ?PostIdleTask@DefaultForegroundTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VIdleTask@v8@@U?$default_delete@VIdleTask@v8@@ (thunk -> +0x9920)
    Lifted::alias(0x7001ed2550, fn_7001edfbc0 as *const (), 2, 0xfbc0), // +0x2550 ?AppendDelayed@DelayedTaskQueue@platform@v8@@QEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@N@Z (thunk -> +0xfbc0)
    Lifted::alias(0x7001ed25a0, fn_7001edc4b0 as *const (), 1, 0xc4b0), // +0x25a0 ?weak_from_this@?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@QEAA?AV?$weak_ptr@VDefaultJobState@platfor (thunk -> +0xc4b0)
    Lifted::alias(0x7001ed25e0, fn_7001edcd90 as *const (), 0, 0xcd90), // +0x25e0 sub_25e0 (thunk -> +0xcd90)
    Lifted::alias(0x7001ed25f0, tpl_9::<0x8> as *const (), 1, 0xadf0), // +0x25f0 ??0?$enable_shared_from_this@VDefaultJobState@platform@v8@@@std@@IEAA@XZ (thunk -> +0xadf0)
    Lifted::alias(0x7001ed2610, fn_7001ee7d50 as *const (), 2, 0x17d50), // +0x2610 sub_2610 (thunk -> +0x17d50)
    Lifted::alias(0x7001ed2620, fn_7001edf090 as *const (), 0, 0xf090), // +0x2620 sub_2620 (thunk -> +0xf090)
    Lifted::alias(0x7001ed2630, tpl_50::<0xffffffffffffff68, 0x4, 0x7001eef070, 0xffffffffffffff64, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd228, 0x7001efd250, 0x7001eea7e2> as *const (), 0, 0x127a0), // +0x2630 sub_2630 (thunk -> +0x127a0)
    Lifted::alias(0x7001ed2680, fn_7001eda750 as *const (), 0, 0xa750), // +0x2680 sub_2680 (thunk -> +0xa750)
    Lifted::alias(0x7001ed2690, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0, 0x191b0), // +0x2690 sub_2690 (thunk -> +0x191b0)
    Lifted::alias(0x7001ed26a0, fn_7001ee5fb0 as *const (), 0, 0x15fb0), // +0x26a0 ?CreateSystemInstrumentationTraceWriter@TraceWriter@tracing@platform@v8@@SAPEAV1234@XZ (thunk -> +0x15fb0)
    Lifted::alias(0x7001ed26b0, fn_7001edd170 as *const (), 0, 0xd170), // +0x26b0 ?EnsureBackgroundTaskRunnerInitialized@DefaultPlatform@platform@v8@@QEAAXXZ (thunk -> +0xd170)
    Lifted::alias(0x7001ed26c0, fn_7001edb960 as *const (), 0, 0xb960), // +0x26c0 ?CappedMaxConcurrency@DefaultJobState@platform@v8@@AEBA_K_K@Z (thunk -> +0xb960)
    Lifted::alias(0x7001ed26d0, fn_7001ee4920 as *const (), 2, 0x14920), // +0x26d0 sub_26d0 (thunk -> +0x14920)
    Lifted::alias(0x7001ed26e0, fn_7001ee7b50 as *const (), 0, 0x17b50), // +0x26e0 ?UpdateTraceEventDuration@TracingController@tracing@platform@v8@@UEAAXPEBEPEBD_K@Z (thunk -> +0x17b50)
    Lifted::alias(0x7001ed2710, fn_7001ee6580 as *const (), 2, 0x16580), // +0x2710 sub_2710 (thunk -> +0x16580)
    Lifted::alias(0x7001ed2720, fn_7001ee25f0 as *const (), 0, 0x125f0), // +0x2720 sub_2720 (thunk -> +0x125f0)
    Lifted::alias(0x7001ed2730, fn_7001ee3290 as *const (), 0, 0x13290), // +0x2730 sub_2730 (thunk -> +0x13290)
    Lifted::alias(0x7001ed2740, tpl_46::<0x7001efd760, 0x38, 0x7001efd650, 0x8, 0x7001efd758> as *const (), 0, 0x111f0), // +0x2740 ?Terminate@TaskQueue@platform@v8@@QEAAXXZ (thunk -> +0x111f0)
    Lifted::alias(0x7001ed2760, fn_7001ee3ef0 as *const (), 0, 0x13ef0), // +0x2760 sub_2760 (thunk -> +0x13ef0)
    Lifted::alias(0x7001ed2770, fn_7001edd8c0 as *const (), 0, 0xd8c0), // +0x2770 ?NewSingleThreadedDefaultPlatform@platform@v8@@YA?AV?$unique_ptr@VPlatform@v8@@U?$default_delete@VPlatform@v8@@@std@@@st (thunk -> +0xd8c0)
    Lifted::alias(0x7001ed27a0, fn_7001eda010 as *const (), 2, 0xa010), // +0x27a0 ?WaitForTaskLocked@DefaultForegroundTaskRunner@platform@v8@@AEAAXAEBV?$LockGuard@VMutex@base@v8@@$0A@@base@3@@Z (thunk -> +0xa010)
    Lifted::alias(0x7001ed27b0, fn_7001ed56b0 as *const (), 0, 0x56b0), // +0x27b0 sub_27b0 (thunk -> +0x56b0)
    Lifted::alias(0x7001ed27c0, tpl_26::<0x30> as *const (), 1, 0xa710), // +0x27c0 ?bind_id@TraceObject@tracing@platform@v8@@QEBA_KXZ (thunk -> +0xa710)
    Lifted::alias(0x7001ed27d0, fn_7001eda590 as *const (), 0, 0xa590), // +0x27d0 sub_27d0 (thunk -> +0xa590)
    Lifted::alias(0x7001ed27e0, fn_7001edc8e0 as *const (), 0, 0xc8e0), // +0x27e0 ??0DefaultPlatform@platform@v8@@QEAA@HW4IdleTaskSupport@12@V?$unique_ptr@VTracingController@v8@@U?$default_delete@VTraci (thunk -> +0xc8e0)
    Lifted::alias(0x7001ed27f0, fn_7001ee7580 as *const (), 0, 0x17580), // +0x27f0 ?RemoveTraceStateObserver@TracingController@tracing@platform@v8@@UEAAXPEAVTraceStateObserver@14@@Z (thunk -> +0x17580)
    Lifted::alias(0x7001ed2810, fn_7001ee6bc0 as *const (), 0, 0x16bc0), // +0x2810 ??1TracingController@tracing@platform@v8@@UEAA@XZ (thunk -> +0x16bc0)
    Lifted::alias(0x7001ed2820, tpl_20 as *const (), 1, 0x9270), // +0x2820 sub_2820 (thunk -> +0x9270)
    Lifted::alias(0x7001ed2830, tpl_28::<0x11> as *const (), 1, 0xbcd0), // +0x2830 sub_2830 (thunk -> +0xbcd0)
    Lifted::alias(0x7001ed2850, tpl_26::<0x10> as *const (), 1, 0xaa90), // +0x2850 ?name@TraceObject@tracing@platform@v8@@QEBAPEBDXZ (thunk -> +0xaa90)
    Lifted::alias(0x7001ed2870, fn_7001ee2850 as *const (), 2, 0x12850), // +0x2870 ?AddIncludedCategory@TraceConfig@tracing@platform@v8@@QEAAXPEBD@Z (thunk -> +0x12850)
    Lifted::alias(0x7001ed2880, fn_7001ee7730 as *const (), 2, 0x17730), // +0x2880 ?StartTracing@TracingController@tracing@platform@v8@@QEAAXPEAVTraceConfig@234@@Z (thunk -> +0x17730)
    Lifted::alias(0x7001ed2890, fn_7001ed9130 as *const (), 1, 0x9130), // +0x2890 ?IsFull@TraceBufferChunk@tracing@platform@v8@@QEBA_NXZ (thunk -> +0x9130)
    Lifted::alias(0x7001ed28a0, tpl_29 as *const (), 1, 0x14d30), // +0x28a0 ??4Recorder@tracing@platform@v8@@QEAAAEAV0123@AEBV0123@@Z (thunk -> +0x14d30)
    Lifted::alias(0x7001ed28b0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0x8f00), // +0x28b0 sub_28b0 (thunk -> +0x8f00)
    Lifted::alias(0x7001ed28f0, fn_7001ed9460 as *const (), 2, 0x9460), // +0x28f0 ?PopTaskFromQueue@DefaultForegroundTaskRunner@platform@v8@@QEAA?AV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@st (thunk -> +0x9460)
    Lifted::alias(0x7001ed2900, tpl_14::<0x7001eea7e2> as *const (), 0, 0xb540), // +0x2900 sub_2900 (thunk -> +0xb540)
    Lifted::alias(0x7001ed2910, fn_7001edde30 as *const (), 2, 0xde30), // +0x2910 ?RunIdleTasks@platform@v8@@YAXPEAVPlatform@2@PEAVIsolate@2@N@Z (thunk -> +0xde30)
    Lifted::alias(0x7001ed2920, fn_7001ed9160 as *const (), 2, 0x9160), // +0x2920 ?MoveExpiredDelayedTasks@DefaultForegroundTaskRunner@platform@v8@@AEAAXAEBV?$LockGuard@VMutex@base@v8@@$0A@@base@3@@Z (thunk -> +0x9160)
    Lifted::alias(0x7001ed2960, fn_7001ee12f0 as *const (), 0, 0x112f0), // +0x2960 sub_2960 (thunk -> +0x112f0)
    Lifted::alias(0x7001ed2970, tpl_24::<0x7001ed8570, 0x7001eda590, 0x7001eda5f6> as *const (), 0, 0xa5e0), // +0x2970 sub_2970 (thunk -> +0xa5e0)
    Lifted::alias(0x7001ed2980, tpl_50::<0xffffffffffffff78, 0x4, 0x7001eef148, 0xffffffffffffff74, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd1c8, 0x7001efd250, 0x7001eea7e2> as *const (), 0, 0x14d80), // +0x2980 sub_2980 (thunk -> +0x14d80)
    Lifted::alias(0x7001ed29a0, fn_7001edc220 as *const (), 0, 0xc220), // +0x29a0 ?UpdatePriority@DefaultJobHandle@platform@v8@@UEAAXW4TaskPriority@3@@Z (thunk -> +0xc220)
    Lifted::alias(0x7001ed29b0, tpl_38::<0x10> as *const (), 0, 0xe1d0), // +0x29b0 sub_29b0 (thunk -> +0xe1d0)
    Lifted::alias(0x7001ed29e0, fn_7001edb4e0 as *const (), 0, 0xb4e0), // +0x29e0 sub_29e0 (thunk -> +0xb4e0)
    Lifted::alias(0x7001ed29f0, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001ee1cd0> as *const (), 0, 0x11cc0), // +0x29f0 sub_29f0 (thunk -> +0x11cc0)
    Lifted::alias(0x7001ed2a00, fn_7001ee1770 as *const (), 1, 0x11770), // +0x2a00 ?AddTraceEvent@TraceBufferChunk@tracing@platform@v8@@QEAAPEAVTraceObject@234@PEA_K@Z (thunk -> +0x11770)
    Lifted::alias(0x7001ed2a10, fn_7001eda600 as *const (), 1, 0xa600), // +0x2a10 sub_2a10 (thunk -> +0xa600)
    Lifted::alias(0x7001ed2a30, fn_7001ee5d90 as *const (), 0, 0x15d90), // +0x2a30 sub_2a30 (thunk -> +0x15d90)
    Lifted::alias(0x7001ed2a50, tpl_38::<0x10> as *const (), 0, 0xe1b0), // +0x2a50 sub_2a50 (thunk -> +0xe1b0)
    Lifted::alias(0x7001ed2a60, tpl_31::<0x8, 0x7001ed18c0, 0x10, 0xc> as *const (), 0, 0xbcf0), // +0x2a60 ?Join@DefaultJobHandle@platform@v8@@UEAAXXZ (thunk -> +0xbcf0)
    Lifted::alias(0x7001ed2a70, fn_7001ee3920 as *const (), 0, 0x13920), // +0x2a70 ?Initialize@TraceObject@tracing@platform@v8@@QEAAXDPEBEPEBD1_K2HPEAPEBD0PEB_KPEAV?$unique_ptr@VConvertableToTraceFormat@ (thunk -> +0x13920)
    Lifted::alias(0x7001ed2a80, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0, 0x19270), // +0x2a80 sub_2a80 (thunk -> +0x19270)
    Lifted::alias(0x7001ed2a90, tpl_16::<0x68> as *const (), 1, 0xa6d0), // +0x2a90 ?arg_convertables@TraceObject@tracing@platform@v8@@QEAAPEAV?$unique_ptr@VConvertableToTraceFormat@v8@@U?$default_delete@ (thunk -> +0xa6d0)
    Lifted::alias(0x7001ed2aa0, fn_7001edb8d0 as *const (), 1, 0xb8d0), // +0x2aa0 ?CancelAndDetach@DefaultJobState@platform@v8@@QEAAXXZ (thunk -> +0xb8d0)
    Lifted::alias(0x7001ed2ab0, fn_7001edd960 as *const (), 0, 0xd960), // +0x2ab0 ?NotifyIsolateShutdown@DefaultPlatform@platform@v8@@QEAAXPEAVIsolate@3@@Z (thunk -> +0xd960)
    Lifted::alias(0x7001ed2ac0, tpl_36::<0x30> as *const (), 0, 0xcfa0), // +0x2ac0 sub_2ac0 (thunk -> +0xcfa0)
    Lifted::alias(0x7001ed2af0, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0, 0x19210), // +0x2af0 sub_2af0 (thunk -> +0x19210)
    Lifted::alias(0x7001ed2b10, fn_7001ee4e30 as *const (), 0, 0x14e30), // +0x2b10 sub_2b10 (thunk -> +0x14e30)
    Lifted::alias(0x7001ed2b20, fn_7001ede0f0 as *const (), 0, 0xe0f0), // +0x2b20 ?SetTracingController@DefaultPlatform@platform@v8@@QEAAXV?$unique_ptr@VTracingController@v8@@U?$default_delete@VTracingC (thunk -> +0xe0f0)
    Lifted::alias(0x7001ed2b30, fn_7001ee1a90 as *const (), 0, 0x11a90), // +0x2b30 sub_2b30 (thunk -> +0x11a90)
    Lifted::alias(0x7001ed2b40, fn_7001edcf30 as *const (), 0, 0xcf30), // +0x2b40 sub_2b40 (thunk -> +0xcf30)
    Lifted::alias(0x7001ed2b60, tpl_24::<0x7001edafa0, 0x7001eda590, 0x7001edc3c6> as *const (), 0, 0xc3b0), // +0x2b60 sub_2b60 (thunk -> +0xc3b0)
    Lifted::alias(0x7001ed2b80, tpl_49::<0xfffffffffffffffc, 0x7001ee4d80> as *const (), 0, 0x14d70), // +0x2b80 sub_2b80 (thunk -> +0x14d70)
    Lifted::alias(0x7001ed2b90, fn_7001ede0a0 as *const (), 0, 0xe0a0), // +0x2b90 ?SetTimeFunctionForTesting@DefaultPlatform@platform@v8@@QEAAXP6ANXZ@Z (thunk -> +0xe0a0)
    Lifted::alias(0x7001ed2bb0, tpl_30::<0x7001ee25f0, 0x7001eea7e2> as *const (), 0, 0x12760), // +0x2bb0 sub_2bb0 (thunk -> +0x12760)
    Lifted::alias(0x7001ed2bc0, fn_7001ed9dd0 as *const (), 1, 0x9dd0), // +0x2bc0 ?SetTraceRecordMode@TraceConfig@tracing@platform@v8@@QEAAXW4TraceRecordMode@234@@Z (thunk -> +0x9dd0)
    Lifted::alias(0x7001ed2be0, fn_7001edabc0 as *const (), 0, 0xabc0), // +0x2be0 sub_2be0 (thunk -> +0xabc0)
    Lifted::alias(0x7001ed2c00, tpl_18::<0x50> as *const (), 1, 0x9110), // +0x2c00 ?IdleTasksEnabled@DefaultForegroundTaskRunner@platform@v8@@UEAA_NXZ (thunk -> +0x9110)
    Lifted::alias(0x7001ed2c10, fn_7001edb8e0 as *const (), 0, 0xb8e0), // +0x2c10 ?CancelAndWait@DefaultJobState@platform@v8@@QEAAXXZ (thunk -> +0xb8e0)
    Lifted::alias(0x7001ed2c20, tpl_15::<0x7001eeee58, 0x7001eea7e2> as *const (), 0, 0xce90), // +0x2c20 sub_2c20 (thunk -> +0xce90)
    Lifted::alias(0x7001ed2c30, fn_7001ee3060 as *const (), 0, 0x13060), // +0x2c30 sub_2c30 (thunk -> +0x13060)
    Lifted::alias(0x7001ed2c40, tpl_15::<0x7001eeee58, 0x7001eea7e2> as *const (), 0, 0xce90), // +0x2c40 sub_2c40 (thunk -> +0xce90)
    Lifted::alias(0x7001ed2c70, tpl_11::<0x7001eea7e2, 0x7001efd520, 0x7001edfa53> as *const (), 0, 0xf950), // +0x2c70 sub_2c70 (thunk -> +0xf950)
    Lifted::alias(0x7001ed2c80, fn_7001edb990 as *const (), 0, 0xb990), // +0x2c80 ?DidRunTask@DefaultJobState@platform@v8@@QEAA_NXZ (thunk -> +0xb990)
    Lifted::alias(0x7001ed2c90, fn_7001ee3ea0 as *const (), 0, 0x13ea0), // +0x2c90 ?UpdateDuration@TraceObject@tracing@platform@v8@@QEAAX_J0@Z (thunk -> +0x13ea0)
    Lifted::alias(0x7001ed2ca0, fn_7001edcf30 as *const (), 0, 0xcf30), // +0x2ca0 sub_2ca0 (thunk -> +0xcf30)
    Lifted::alias(0x7001ed2cb0, fn_7001ee6fe0 as *const (), 0, 0x16fe0), // +0x2cb0 ?AddTraceEvent@TracingController@tracing@platform@v8@@UEAA_KDPEBEPEBD1_K2HPEAPEBD0PEB_KPEAV?$unique_ptr@VConvertableToTr (thunk -> +0x16fe0)
    Lifted::alias(0x7001ed2cd0, fn_7001edc5c0 as *const (), 0, 0xc5c0), // +0x2cd0 sub_2cd0 (thunk -> +0xc5c0)
    Lifted::alias(0x7001ed2cf0, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001ee3050> as *const (), 0, 0x13040), // +0x2cf0 sub_2cf0 (thunk -> +0x13040)
    Lifted::alias(0x7001ed2d10, tpl_38::<0x10> as *const (), 0, 0xe1f0), // +0x2d10 sub_2d10 (thunk -> +0xe1f0)
    Lifted::alias(0x7001ed2d20, fn_7001ee6300 as *const (), 0, 0x16300), // +0x2d20 sub_2d20 (thunk -> +0x16300)
    Lifted::alias(0x7001ed2d50, fn_7001ee0210 as *const (), 0, 0x10210), // +0x2d50 sub_2d50 (thunk -> +0x10210)
    Lifted::alias(0x7001ed2d70, tpl_20 as *const (), 1, 0xf420), // +0x2d70 ?IdleTasksEnabled@DefaultWorkerThreadsTaskRunner@platform@v8@@UEAA_NXZ (thunk -> +0xf420)
    Lifted::alias(0x7001ed2d80, fn_7001ed8a60 as *const (), 0, 0x8a60), // +0x2d80 ??1DefaultForegroundTaskRunner@platform@v8@@UEAA@XZ (thunk -> +0x8a60)
    Lifted::alias(0x7001ed2d90, tpl_8::<0x7001eee9c0> as *const (), 1, 0x84e0), // +0x2d90 ??0TraceBuffer@tracing@platform@v8@@QEAA@XZ (thunk -> +0x84e0)
    Lifted::alias(0x7001ed2da0, fn_7001edd5c0 as *const (), 0, 0xd5c0), // +0x2da0 ?NewDefaultJobHandle@platform@v8@@YA?AV?$unique_ptr@VJobHandle@v8@@U?$default_delete@VJobHandle@v8@@@std@@@std@@PEAVPlat (thunk -> +0xd5c0)
    Lifted::alias(0x7001ed2db0, fn_7001ee42e0 as *const (), 0, 0x142e0), // +0x2db0 sub_2db0 (thunk -> +0x142e0)
    Lifted::alias(0x7001ed2dc0, tpl_14::<0x7001eea7e2> as *const (), 0, 0xcf70), // +0x2dc0 sub_2dc0 (thunk -> +0xcf70)
    Lifted::alias(0x7001ed2dd0, fn_7001edf500 as *const (), 0, 0xf500), // +0x2dd0 ?PostTask@DefaultWorkerThreadsTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@st (thunk -> +0xf500)
    Lifted::alias(0x7001ed2de0, fn_7001ee34f0 as *const (), 1, 0x134f0), // +0x2de0 sub_2de0 (thunk -> +0x134f0)
    Lifted::alias(0x7001ed2e20, fn_7001ee1980 as *const (), 0, 0x11980), // +0x2e20 ?CreateTraceBufferRingBuffer@TraceBuffer@tracing@platform@v8@@SAPEAV1234@_KPEAVTraceWriter@234@@Z (thunk -> +0x11980)
    Lifted::alias(0x7001ed2e40, fn_7001ee83f0 as *const (), 0, 0x183f0), // +0x2e40 ??1Recorder@tracing@platform@v8@@QEAA@XZ (thunk -> +0x183f0)
    Lifted::alias(0x7001ed2e70, tpl_22 as *const (), 0, 0x9b50), // +0x2e70 sub_2e70 (thunk -> +0x9b50)
    Lifted::alias(0x7001ed2e80, fn_7001ee0e90 as *const (), 0, 0x10e90), // +0x2e80 ??1TaskQueue@platform@v8@@QEAA@XZ (thunk -> +0x10e90)
    Lifted::alias(0x7001ed2e90, fn_7001ed8940 as *const (), 0, 0x8940), // +0x2e90 sub_2e90 (thunk -> +0x8940)
    Lifted::alias(0x7001ed2ea0, fn_7001ee5e80 as *const (), 0, 0x15e80), // +0x2ea0 ?CreateJSONTraceWriterCohtml@TraceWriter@tracing@platform@v8@@SAPEAV1234@XZ (thunk -> +0x15e80)
    Lifted::alias(0x7001ed2ec0, tpl_25::<0x7001eeeab0, 0x7001ee88f0, 0x7001eda6a0> as *const (), 0, 0xa690), // +0x2ec0 sub_2ec0 (thunk -> +0xa690)
    Lifted::alias(0x7001ed2ed0, tpl_18::<0x14> as *const (), 1, 0xd550), // +0x2ed0 ?IdleTasksEnabled@DefaultPlatform@platform@v8@@UEAA_NPEAVIsolate@3@@Z (thunk -> +0xd550)
    Lifted::alias(0x7001ed2ee0, tpl_26::<0x38> as *const (), 1, 0xd540), // +0x2ee0 ?GetTracingController@DefaultPlatform@platform@v8@@UEAAPEAVTracingController@3@XZ (thunk -> +0xd540)
    Lifted::alias(0x7001ed2f00, tpl_23::<0x7001ee8a40, 0x7001eea730, 0x7001eea740, 0x7001eea7e2, 0x7001efd520, 0x7001eda508, 0x7001eda5e0, 0x7001eda50e, 0x7001eda690, 0x7001eda517> as *const (), 0, 0xa340), // +0x2f00 sub_2f00 (thunk -> +0xa340)
    Lifted::alias(0x7001ed2f10, fn_7001ee0ef0 as *const (), 0, 0x10ef0), // +0x2f10 ?Append@TaskQueue@platform@v8@@QEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@@Z (thunk -> +0x10ef0)
    Lifted::alias(0x7001ed2f20, fn_7001edbe90 as *const (), 0, 0xbe90), // +0x2f20 ?NotifyConcurrencyIncrease@DefaultJobState@platform@v8@@QEAAXXZ (thunk -> +0xbe90)
    Lifted::alias(0x7001ed2f30, tpl_42::<0x10, 0x19, 0x8> as *const (), 1, 0xea80), // +0x2f30 sub_2f30 (thunk -> +0xea80)
    Lifted::alias(0x7001ed2f40, fn_7001ed9b70 as *const (), 0, 0x9b70), // +0x2f40 ?PostNonNestableTask@DefaultForegroundTaskRunner@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@ (thunk -> +0x9b70)
    Lifted::alias(0x7001ed2f60, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0, 0x19270), // +0x2f60 sub_2f60 (thunk -> +0x19270)
    Lifted::alias(0x7001ed2f70, fn_7001ee1ce0 as *const (), 0, 0x11ce0), // +0x2f70 sub_2f70 (thunk -> +0x11ce0)
    Lifted::alias(0x7001ed2f80, fn_7001edb850 as *const (), 0, 0xb850), // +0x2f80 ?CancelAndDetach@DefaultJobHandle@platform@v8@@UEAAXXZ (thunk -> +0xb850)
    Lifted::alias(0x7001ed2f90, fn_7001ed8530 as *const (), 1, 0x8530), // +0x2f90 ??0TraceObject@tracing@platform@v8@@QEAA@XZ (thunk -> +0x8530)
    Lifted::alias(0x7001ed2fb0, tpl_20 as *const (), 1, 0xd560), // +0x2fb0 sub_2fb0 (thunk -> +0xd560)
    Lifted::alias(0x7001ed2fc0, fn_7001ed9070 as *const (), 1, 0x9070), // +0x2fc0 ?GetEventAt@TraceBufferChunk@tracing@platform@v8@@QEAAPEAVTraceObject@234@_K@Z (thunk -> +0x9070)
    Lifted::alias(0x7001ed2fe0, fn_7001ed9140 as *const (), 1, 0x9140), // +0x2fe0 ?IsSystraceEnabled@TraceConfig@tracing@platform@v8@@QEBA_NXZ (thunk -> +0x9140)
    Lifted::alias(0x7001ed2ff0, tpl_14::<0x7001eea7e2> as *const (), 0, 0xcf70), // +0x2ff0 sub_2ff0 (thunk -> +0xcf70)
    Lifted::alias(0x7001ed3000, fn_7001ed8bd0 as *const (), 0, 0x8bd0), // +0x3000 sub_3000 (thunk -> +0x8bd0)
    Lifted::alias(0x7001ed3010, fn_7001edfb40 as *const (), 0, 0xfb40), // +0x3010 ?Append@DelayedTaskQueue@platform@v8@@QEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@@Z (thunk -> +0xfb40)
    Lifted::alias(0x7001ed3030, fn_7001edd7d0 as *const (), 0, 0xd7d0), // +0x3030 ?NewDefaultPlatform@platform@v8@@YA?AV?$unique_ptr@VPlatform@v8@@U?$default_delete@VPlatform@v8@@@std@@@std@@HW4IdleTask (thunk -> +0xd7d0)
    Lifted::alias(0x7001ed3040, tpl_45::<0x7001eeeed0, 0x7001efd678, 0x7001efd6f0, 0x7001eea7e2> as *const (), 0, 0xf360), // +0x3040 sub_3040 (thunk -> +0xf360)
    Lifted::alias(0x7001ed3050, tpl_52::<0x7001efd728> as *const (), 0, 0x17360), // +0x3050 ?CurrentTimestampMicroseconds@TracingController@tracing@platform@v8@@MEAA_JXZ (thunk -> +0x17360)
    Lifted::alias(0x7001ed3070, fn_7001ee4ff0 as *const (), 2, 0x14ff0), // +0x3070 sub_3070 (thunk -> +0x14ff0)
    Lifted::alias(0x7001ed3080, tpl_36::<0x30> as *const (), 0, 0xd060), // +0x3080 sub_3080 (thunk -> +0xd060)
    Lifted::alias(0x7001ed3090, tpl_12 as *const (), 1, 0x17720), // +0x3090 sub_3090 (thunk -> +0x17720)
    Lifted::alias(0x7001ed30a0, tpl_34::<0x7001ef9cf0> as *const (), 1, 0x17520), // +0x30a0 sub_30a0 (thunk -> +0x17520)
    Lifted::alias(0x7001ed30b0, tpl_39::<0x10, 0x40> as *const (), 0, 0xe210), // +0x30b0 sub_30b0 (thunk -> +0xe210)
    Lifted::alias(0x7001ed30c0, fn_7001edfd50 as *const (), 2, 0xfd50), // +0x30c0 ?GetNext@DelayedTaskQueue@platform@v8@@QEAA?AV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@XZ (thunk -> +0xfd50)
    Lifted::alias(0x7001ed30d0, tpl_20 as *const (), 1, 0xde20), // +0x30d0 sub_30d0 (thunk -> +0xde20)
    Lifted::alias(0x7001ed30e0, tpl_28::<0x8> as *const (), 1, 0xaab0), // +0x30e0 ?phase@TraceObject@tracing@platform@v8@@QEBADXZ (thunk -> +0xaab0)
    Lifted::alias(0x7001ed30f0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0, 0x8f30), // +0x30f0 sub_30f0 (thunk -> +0x8f30)
    Lifted::alias(0x7001ed3100, fn_7001ee6d00 as *const (), 0, 0x16d00), // +0x3100 sub_3100 (thunk -> +0x16d00)
    Lifted::alias(0x7001ed3120, fn_7001ee1b90 as *const (), 0, 0x11b90), // +0x3120 sub_3120 (thunk -> +0x11b90)
    Lifted::alias(0x7001ed3150, tpl_21 as *const (), 1, 0xc2d0), // +0x3150 ?UpdatePriorityEnabled@DefaultJobHandle@platform@v8@@UEBA_NXZ (thunk -> +0xc2d0)
    Lifted::alias(0x7001ed3160, tpl_23::<0x7001ee8a40, 0x7001eea730, 0x7001eea740, 0x7001eea7e2, 0x7001efd520, 0x7001ee0858, 0x7001eda5e0, 0x7001ee085e, 0x7001ee0ce0, 0x7001ee0867> as *const (), 0, 0x10690), // +0x3160 sub_3160 (thunk -> +0x10690)
    Lifted::alias(0x7001ed3170, fn_7001ed8ba0 as *const (), 1, 0x8ba0), // +0x3170 ??1TraceWriter@tracing@platform@v8@@UEAA@XZ (thunk -> +0x8ba0)
    Lifted::alias(0x7001ed31c0, fn_7001edf150 as *const (), 0, 0xf150), // +0x31c0 ??1DefaultWorkerThreadsTaskRunner@platform@v8@@UEAA@XZ (thunk -> +0xf150)
    Lifted::alias(0x7001ed31d0, fn_7001edd0c0 as *const (), 0, 0xd0c0), // +0x31d0 ?CallOnWorkerThread@DefaultPlatform@platform@v8@@UEAAXV?$unique_ptr@VTask@v8@@U?$default_delete@VTask@v8@@@std@@@std@@@Z (thunk -> +0xd0c0)
    Lifted::alias(0x7001ed31e0, fn_7001ee4e30 as *const (), 0, 0x14e30), // +0x31e0 sub_31e0 (thunk -> +0x14e30)
    Lifted::alias(0x7001ed31f0, tpl_26::<0xa0> as *const (), 1, 0xa730), // +0x31f0 ?cpu_duration@TraceObject@tracing@platform@v8@@QEAA_KXZ (thunk -> +0xa730)
    Lifted::alias(0x7001ed3210, fn_7001edb730 as *const (), 0, 0xb730), // +0x3210 ?CanRunFirstTask@DefaultJobState@platform@v8@@QEAA_NXZ (thunk -> +0xb730)
    Lifted::alias(0x7001ed3220, fn_7001ed9380 as *const (), 0, 0x9380), // +0x3220 ?PopTaskFromIdleQueue@DefaultForegroundTaskRunner@platform@v8@@QEAA?AV?$unique_ptr@VIdleTask@v8@@U?$default_delete@VIdle (thunk -> +0x9380)
    Lifted::alias(0x7001ed3240, fn_7001ee2aa0 as *const (), 0, 0x12aa0), // +0x3240 ?IsCategoryGroupEnabled@TraceConfig@tracing@platform@v8@@QEBA_NPEBD@Z (thunk -> +0x12aa0)
    Lifted::alias(0x7001ed3250, fn_7001edf630 as *const (), 0, 0xf630), // +0x3250 ?Terminate@DefaultWorkerThreadsTaskRunner@platform@v8@@QEAAXXZ (thunk -> +0xf630)
    Lifted::alias(0x7001ed3260, tpl_10::<0x7001eee920, 0x8, 0x7001eee900> as *const (), 1, 0x8570), // +0x3260 sub_3260 (thunk -> +0x8570)
    Lifted::alias(0x7001ed3290, tpl_12 as *const (), 1, 0x162d0), // +0x3290 sub_3290 (thunk -> +0x162d0)
    Lifted::alias(0x7001ed32a0, fn_7001ee78a0 as *const (), 2, 0x178a0), // +0x32a0 ?StopTracing@TracingController@tracing@platform@v8@@QEAAXXZ (thunk -> +0x178a0)
    Lifted::alias(0x7001ed32b0, fn_7001ee1620 as *const (), 0, 0x11620), // +0x32b0 sub_32b0 (thunk -> +0x11620)
    Lifted::alias(0x7001ed32c0, fn_7001edaed0 as *const (), 0, 0xaed0), // +0x32c0 ??0DefaultJobState@platform@v8@@QEAA@PEAVPlatform@2@V?$unique_ptr@VJobTask@v8@@U?$default_delete@VJobTask@v8@@@std@@@std (thunk -> +0xaed0)
    Lifted::alias(0x7001ed32e0, fn_7001edae00 as *const (), 0, 0xae00), // +0x32e0 ??0DefaultJobHandle@platform@v8@@QEAA@V?$shared_ptr@VDefaultJobState@platform@v8@@@std@@@Z (thunk -> +0xae00)
    Lifted::alias(0x7001ed3300, fn_7001ed7a90 as *const (), 0, 0x7a90), // +0x3300 sub_3300 (thunk -> +0x7a90)
    Lifted::alias(0x7001ed3320, fn_7001ee6af0 as *const (), 0, 0x16af0), // +0x3320 sub_3320 (thunk -> +0x16af0)
    Lifted::alias(0x7001ed3330, tpl_37 as *const (), 1, 0x17290), // +0x3330 sub_3330 (thunk -> +0x17290)
    Lifted::alias(0x7001ed3340, tpl_16::<0x50> as *const (), 1, 0xa6f0), // +0x3340 ?arg_types@TraceObject@tracing@platform@v8@@QEAAPEAEXZ (thunk -> +0xa6f0)
    Lifted::alias(0x7001ed3350, fn_7001ed90a0 as *const (), 1, 0x90a0), // +0x3350 ?HasPoppableTaskInQueue@DefaultForegroundTaskRunner@platform@v8@@AEBA_NXZ (thunk -> +0x90a0)
    Lifted::new(0x7001ed56b0, fn_7001ed56b0 as *const (), 0), // +0x56b0 sub_56b0
    Lifted::new(0x7001ed5740, fn_7001ed5740 as *const (), 0), // +0x5740 sub_5740
    Lifted::new(0x7001ed79c0, fn_7001ed79c0 as *const (), 0), // +0x79c0 sub_79c0
    Lifted::new(0x7001ed7a10, tpl_5::<0x7001eea730> as *const (), 0), // +0x7a10 sub_7a10
    Lifted::new(0x7001ed7a50, tpl_5::<0x7001eea730> as *const (), 0), // +0x7a50 sub_7a50
    Lifted::new(0x7001ed7a90, fn_7001ed7a90 as *const (), 0), // +0x7a90 sub_7a90
    Lifted::new(0x7001ed7dc0, fn_7001ed7dc0 as *const (), 2), // +0x7dc0 sub_7dc0
    Lifted::new(0x7001ed8070, fn_7001ed8070 as *const (), 2), // +0x8070 sub_8070
    Lifted::new(0x7001ed81a0, tpl_6::<0x7001eea740> as *const (), 0), // +0x81a0 sub_81a0
    Lifted::new(0x7001ed81e0, tpl_6::<0x7001eea740> as *const (), 0), // +0x81e0 sub_81e0
    Lifted::new(0x7001ed8220, tpl_7::<0x7001eea740> as *const (), 0), // +0x8220 sub_8220
    Lifted::new(0x7001ed8250, tpl_7::<0x7001eea740> as *const (), 0), // +0x8250 sub_8250
    Lifted::new(0x7001ed8280, tpl_5::<0x7001eea730> as *const (), 0), // +0x8280 sub_8280
    Lifted::new(0x7001ed82c0, tpl_5::<0x7001eea730> as *const (), 0), // +0x82c0 sub_82c0
    Lifted::new(0x7001ed8300, fn_7001ed8300 as *const (), 0), // +0x8300 sub_8300
    Lifted::new(0x7001ed8420, fn_7001ed8420 as *const (), 0), // +0x8420 sub_8420
    Lifted::new(0x7001ed84e0, tpl_8::<0x7001eee9c0> as *const (), 1), // +0x84e0 sub_84e0
    Lifted::new(0x7001ed8500, fn_7001ed8500 as *const (), 1), // +0x8500 sub_8500
    Lifted::new(0x7001ed8520, tpl_9::<0x8> as *const (), 1), // +0x8520 sub_8520
    Lifted::new(0x7001ed8530, fn_7001ed8530 as *const (), 1), // +0x8530 sub_8530
    Lifted::new(0x7001ed8550, tpl_8::<0x7001eee998> as *const (), 1), // +0x8550 sub_8550
    Lifted::new(0x7001ed8570, tpl_10::<0x7001eee920, 0x8, 0x7001eee900> as *const (), 1), // +0x8570 sub_8570
    Lifted::new(0x7001ed8590, fn_7001ed8590 as *const (), 0), // +0x8590 sub_8590
    Lifted::new(0x7001ed86e0, tpl_11::<0x7001eea7e2, 0x7001efd520, 0x7001ed87e3> as *const (), 0), // +0x86e0 sub_86e0
    Lifted::new(0x7001ed8850, fn_7001ed8850 as *const (), 0), // +0x8850 sub_8850
    Lifted::new(0x7001ed8940, fn_7001ed8940 as *const (), 0), // +0x8940 sub_8940
    Lifted::new(0x7001ed8a60, fn_7001ed8a60 as *const (), 0), // +0x8a60 sub_8a60
    Lifted::new(0x7001ed8ab0, fn_7001ed8ab0 as *const (), 0), // +0x8ab0 sub_8ab0
    Lifted::new(0x7001ed8b30, tpl_12 as *const (), 1), // +0x8b30 sub_8b30
    Lifted::new(0x7001ed8b40, fn_7001ed8b40 as *const (), 0), // +0x8b40 sub_8b40
    Lifted::new(0x7001ed8b90, fn_7001ed8b90 as *const (), 0), // +0x8b90 sub_8b90
    Lifted::new(0x7001ed8ba0, fn_7001ed8ba0 as *const (), 1), // +0x8ba0 sub_8ba0
    Lifted::new(0x7001ed8bd0, fn_7001ed8bd0 as *const (), 0), // +0x8bd0 sub_8bd0
    Lifted::new(0x7001ed8d00, tpl_13::<0xfffffffffffffff8, 0x7001eea7e2> as *const (), 0), // +0x8d00 sub_8d00
    Lifted::new(0x7001ed8d50, tpl_13::<0xfffffffffffffff8, 0x7001eea7e2> as *const (), 0), // +0x8d50 sub_8d50
    Lifted::new(0x7001ed8da0, fn_7001ed8da0 as *const (), 0), // +0x8da0 sub_8da0
    Lifted::new(0x7001ed8dd0, fn_7001ed8dd0 as *const (), 0), // +0x8dd0 sub_8dd0
    Lifted::new(0x7001ed8e40, tpl_14::<0x7001eea7e2> as *const (), 0), // +0x8e40 sub_8e40
    Lifted::new(0x7001ed8e70, tpl_14::<0x7001eea7e2> as *const (), 0), // +0x8e70 sub_8e70
    Lifted::new(0x7001ed8ea0, tpl_14::<0x7001eea7e2> as *const (), 0), // +0x8ea0 sub_8ea0
    Lifted::new(0x7001ed8ed0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0), // +0x8ed0 sub_8ed0
    Lifted::new(0x7001ed8f00, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0), // +0x8f00 sub_8f00
    Lifted::new(0x7001ed8f30, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0), // +0x8f30 sub_8f30
    Lifted::new(0x7001ed8f60, fn_7001ed8f60 as *const (), 0), // +0x8f60 sub_8f60
    Lifted::new(0x7001ed8fc0, fn_7001ed8fc0 as *const (), 0), // +0x8fc0 sub_8fc0
    Lifted::new(0x7001ed9030, fn_7001ed9030 as *const (), 1), // +0x9030 sub_9030
    Lifted::new(0x7001ed9040, fn_7001ed9040 as *const (), 1), // +0x9040 sub_9040
    Lifted::new(0x7001ed9050, fn_7001ed9050 as *const (), 1), // +0x9050 sub_9050
    Lifted::new(0x7001ed9060, tpl_16::<0x8> as *const (), 1), // +0x9060 sub_9060
    Lifted::new(0x7001ed9070, fn_7001ed9070 as *const (), 1), // +0x9070 sub_9070
    Lifted::new(0x7001ed9090, tpl_17 as *const (), 1), // +0x9090 sub_9090
    Lifted::new(0x7001ed90a0, fn_7001ed90a0 as *const (), 1), // +0x90a0 sub_90a0
    Lifted::new(0x7001ed9110, tpl_18::<0x50> as *const (), 1), // +0x9110 sub_9110
    Lifted::new(0x7001ed9120, fn_7001ed9120 as *const (), 1), // +0x9120 sub_9120
    Lifted::new(0x7001ed9130, fn_7001ed9130 as *const (), 1), // +0x9130 sub_9130
    Lifted::new(0x7001ed9140, fn_7001ed9140 as *const (), 1), // +0x9140 sub_9140
    Lifted::new(0x7001ed9150, tpl_19::<0xa0> as *const (), 2), // +0x9150 sub_9150
    Lifted::new(0x7001ed9160, fn_7001ed9160 as *const (), 2), // +0x9160 sub_9160
    Lifted::new(0x7001ed9270, tpl_20 as *const (), 1), // +0x9270 sub_9270
    Lifted::new(0x7001ed9280, tpl_21 as *const (), 1), // +0x9280 sub_9280
    Lifted::new(0x7001ed9290, tpl_20 as *const (), 1), // +0x9290 sub_9290
    Lifted::new(0x7001ed92a0, fn_7001ed92a0 as *const (), 2), // +0x92a0 sub_92a0
    Lifted::new(0x7001ed9380, fn_7001ed9380 as *const (), 0), // +0x9380 sub_9380
    Lifted::new(0x7001ed9460, fn_7001ed9460 as *const (), 2), // +0x9460 sub_9460
    Lifted::new(0x7001ed96b0, fn_7001ed96b0 as *const (), 2), // +0x96b0 sub_96b0
    Lifted::new(0x7001ed9760, fn_7001ed9760 as *const (), 2), // +0x9760 sub_9760
    Lifted::new(0x7001ed9920, fn_7001ed9920 as *const (), 0), // +0x9920 sub_9920
    Lifted::new(0x7001ed9aa0, fn_7001ed9aa0 as *const (), 2), // +0x9aa0 sub_9aa0
    Lifted::new(0x7001ed9b50, tpl_22 as *const (), 0), // +0x9b50 sub_9b50
    Lifted::new(0x7001ed9b70, fn_7001ed9b70 as *const (), 0), // +0x9b70 sub_9b70
    Lifted::new(0x7001ed9c00, tpl_22 as *const (), 0), // +0x9c00 sub_9c00
    Lifted::new(0x7001ed9c20, fn_7001ed9c20 as *const (), 0), // +0x9c20 sub_9c20
    Lifted::new(0x7001ed9cb0, fn_7001ed9cb0 as *const (), 0), // +0x9cb0 sub_9cb0
    Lifted::new(0x7001ed9dd0, fn_7001ed9dd0 as *const (), 1), // +0x9dd0 sub_9dd0
    Lifted::new(0x7001ed9de0, fn_7001ed9de0 as *const (), 2), // +0x9de0 sub_9de0
    Lifted::new(0x7001eda010, fn_7001eda010 as *const (), 2), // +0xa010 sub_a010
    Lifted::new(0x7001eda0b0, fn_7001eda0b0 as *const (), 0), // +0xa0b0 sub_a0b0
    Lifted::new(0x7001eda0e0, tpl_12 as *const (), 1), // +0xa0e0 sub_a0e0
    Lifted::new(0x7001eda0f0, fn_7001eda0f0 as *const (), 0), // +0xa0f0 sub_a0f0
    Lifted::new(0x7001eda340, tpl_23::<0x7001ee8a40, 0x7001eea730, 0x7001eea740, 0x7001eea7e2, 0x7001efd520, 0x7001eda508, 0x7001eda5e0, 0x7001eda50e, 0x7001eda690, 0x7001eda517> as *const (), 0), // +0xa340 sub_a340
    Lifted::new(0x7001eda590, fn_7001eda590 as *const (), 0), // +0xa590 sub_a590
    Lifted::new(0x7001eda5e0, tpl_24::<0x7001ed8570, 0x7001eda590, 0x7001eda5f6> as *const (), 0), // +0xa5e0 sub_a5e0
    Lifted::new(0x7001eda600, fn_7001eda600 as *const (), 1), // +0xa600 sub_a600
    Lifted::new(0x7001eda670, tpl_25::<0x7001eeeab0, 0x7001ee88f0, 0x7001eda680> as *const (), 0), // +0xa670 sub_a670
    Lifted::new(0x7001eda690, tpl_25::<0x7001eeeab0, 0x7001ee88f0, 0x7001eda6a0> as *const (), 0), // +0xa690 sub_a690
    Lifted::new(0x7001eda6b0, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001eda6c0> as *const (), 0), // +0xa6b0 sub_a6b0
    Lifted::new(0x7001eda6d0, tpl_16::<0x68> as *const (), 1), // +0xa6d0 sub_a6d0
    Lifted::new(0x7001eda6e0, tpl_16::<0x40> as *const (), 1), // +0xa6e0 sub_a6e0
    Lifted::new(0x7001eda6f0, tpl_16::<0x50> as *const (), 1), // +0xa6f0 sub_a6f0
    Lifted::new(0x7001eda700, tpl_16::<0x58> as *const (), 1), // +0xa700 sub_a700
    Lifted::new(0x7001eda710, tpl_26::<0x30> as *const (), 1), // +0xa710 sub_a710
    Lifted::new(0x7001eda720, tpl_26::<0x20> as *const (), 1), // +0xa720 sub_a720
    Lifted::new(0x7001eda730, tpl_26::<0xa0> as *const (), 1), // +0xa730 sub_a730
    Lifted::new(0x7001eda740, tpl_26::<0x98> as *const (), 1), // +0xa740 sub_a740
    Lifted::new(0x7001eda750, fn_7001eda750 as *const (), 0), // +0xa750 sub_a750
    Lifted::new(0x7001edaa70, tpl_27::<0x80> as *const (), 1), // +0xaa70 sub_aa70
    Lifted::new(0x7001edaa80, tpl_26::<0x28> as *const (), 1), // +0xaa80 sub_aa80
    Lifted::new(0x7001edaa90, tpl_26::<0x10> as *const (), 1), // +0xaa90 sub_aa90
    Lifted::new(0x7001edaaa0, tpl_27::<0x38> as *const (), 1), // +0xaaa0 sub_aaa0
    Lifted::new(0x7001edaab0, tpl_28::<0x8> as *const (), 1), // +0xaab0 sub_aab0
    Lifted::new(0x7001edaac0, tpl_17 as *const (), 1), // +0xaac0 sub_aac0
    Lifted::new(0x7001edaad0, fn_7001edaad0 as *const (), 2), // +0xaad0 sub_aad0
    Lifted::new(0x7001edabc0, fn_7001edabc0 as *const (), 0), // +0xabc0 sub_abc0
    Lifted::new(0x7001edac40, tpl_26::<0x18> as *const (), 1), // +0xac40 sub_ac40
    Lifted::new(0x7001edac50, tpl_27::<0x2a08> as *const (), 1), // +0xac50 sub_ac50
    Lifted::new(0x7001edac60, fn_7001edac60 as *const (), 1), // +0xac60 sub_ac60
    Lifted::new(0x7001edac70, tpl_27::<0x4> as *const (), 1), // +0xac70 sub_ac70
    Lifted::new(0x7001edac80, tpl_26::<0x88> as *const (), 1), // +0xac80 sub_ac80
    Lifted::new(0x7001edac90, tpl_26::<0x90> as *const (), 1), // +0xac90 sub_ac90
    Lifted::new(0x7001edaca0, fn_7001edaca0 as *const (), 1), // +0xaca0 sub_aca0
    Lifted::new(0x7001edad30, fn_7001edad30 as *const (), 0), // +0xad30 sub_ad30
    Lifted::new(0x7001edade0, tpl_9::<0x8> as *const (), 1), // +0xade0 sub_ade0
    Lifted::new(0x7001edadf0, tpl_9::<0x8> as *const (), 1), // +0xadf0 sub_adf0
    Lifted::new(0x7001edae00, fn_7001edae00 as *const (), 0), // +0xae00 sub_ae00
    Lifted::new(0x7001edaed0, fn_7001edaed0 as *const (), 0), // +0xaed0 sub_aed0
    Lifted::new(0x7001edafa0, tpl_10::<0x7001eeeb08, 0x8, 0x7001eeeae8> as *const (), 1), // +0xafa0 sub_afa0
    Lifted::new(0x7001edafc0, fn_7001edafc0 as *const (), 0), // +0xafc0 sub_afc0
    Lifted::new(0x7001edaff0, fn_7001edaff0 as *const (), 0), // +0xaff0 sub_aff0
    Lifted::new(0x7001edb060, fn_7001edb060 as *const (), 0), // +0xb060 sub_b060
    Lifted::new(0x7001edb170, tpl_29 as *const (), 1), // +0xb170 sub_b170
    Lifted::new(0x7001edb180, fn_7001edb180 as *const (), 0), // +0xb180 sub_b180
    Lifted::new(0x7001edb2e0, fn_7001edb2e0 as *const (), 0), // +0xb2e0 sub_b2e0
    Lifted::new(0x7001edb400, fn_7001edb400 as *const (), 0), // +0xb400 sub_b400
    Lifted::new(0x7001edb4a0, tpl_30::<0x7001ed1b20, 0x7001eea7e2> as *const (), 0), // +0xb4a0 sub_b4a0
    Lifted::new(0x7001edb4e0, fn_7001edb4e0 as *const (), 0), // +0xb4e0 sub_b4e0
    Lifted::new(0x7001edb540, tpl_14::<0x7001eea7e2> as *const (), 0), // +0xb540 sub_b540
    Lifted::new(0x7001edb570, tpl_14::<0x7001eea7e2> as *const (), 0), // +0xb570 sub_b570
    Lifted::new(0x7001edb5a0, tpl_15::<0x7001eee8a8, 0x7001eea7e2> as *const (), 0), // +0xb5a0 sub_b5a0
    Lifted::new(0x7001edb5d0, fn_7001edb5d0 as *const (), 1), // +0xb5d0 sub_b5d0
    Lifted::new(0x7001edb670, fn_7001edb670 as *const (), 0), // +0xb670 sub_b670
    Lifted::new(0x7001edb730, fn_7001edb730 as *const (), 0), // +0xb730 sub_b730
    Lifted::new(0x7001edb7d0, tpl_31::<0x8, 0x7001ed2c10, 0x10, 0xc> as *const (), 0), // +0xb7d0 sub_b7d0
    Lifted::new(0x7001edb850, fn_7001edb850 as *const (), 0), // +0xb850 sub_b850
    Lifted::new(0x7001edb8d0, fn_7001edb8d0 as *const (), 1), // +0xb8d0 sub_b8d0
    Lifted::new(0x7001edb8e0, fn_7001edb8e0 as *const (), 0), // +0xb8e0 sub_b8e0
    Lifted::new(0x7001edb960, fn_7001edb960 as *const (), 0), // +0xb960 sub_b960
    Lifted::new(0x7001edb990, fn_7001edb990 as *const (), 0), // +0xb990 sub_b990
    Lifted::new(0x7001edbbc0, fn_7001edbbc0 as *const (), 0), // +0xbbc0 sub_bbc0
    Lifted::new(0x7001edbbf0, fn_7001edbbf0 as *const (), 0), // +0xbbf0 sub_bbf0
    Lifted::new(0x7001edbc60, fn_7001edbc60 as *const (), 0), // +0xbc60 sub_bc60
    Lifted::new(0x7001edbcd0, tpl_28::<0x11> as *const (), 1), // +0xbcd0 sub_bcd0
    Lifted::new(0x7001edbce0, fn_7001edbce0 as *const (), 1), // +0xbce0 sub_bce0
    Lifted::new(0x7001edbcf0, tpl_31::<0x8, 0x7001ed18c0, 0x10, 0xc> as *const (), 0), // +0xbcf0 sub_bcf0
    Lifted::new(0x7001edbd70, fn_7001edbd70 as *const (), 0), // +0xbd70 sub_bd70
    Lifted::new(0x7001edbe80, tpl_32::<0x8, 0x7001ed2f20> as *const (), 0), // +0xbe80 sub_be80
    Lifted::new(0x7001edbe90, fn_7001edbe90 as *const (), 0), // +0xbe90 sub_be90
    Lifted::new(0x7001edc090, tpl_32::<0x8, 0x7001ed2f20> as *const (), 0), // +0xc090 sub_c090
    Lifted::new(0x7001edc0a0, fn_7001edc0a0 as *const (), 1), // +0xc0a0 sub_c0a0
    Lifted::new(0x7001edc0c0, fn_7001edc0c0 as *const (), 0), // +0xc0c0 sub_c0c0
    Lifted::new(0x7001edc210, fn_7001edc210 as *const (), 1), // +0xc210 sub_c210
    Lifted::new(0x7001edc220, fn_7001edc220 as *const (), 0), // +0xc220 sub_c220
    Lifted::new(0x7001edc270, fn_7001edc270 as *const (), 0), // +0xc270 sub_c270
    Lifted::new(0x7001edc2c0, tpl_12 as *const (), 1), // +0xc2c0 sub_c2c0
    Lifted::new(0x7001edc2d0, tpl_21 as *const (), 1), // +0xc2d0 sub_c2d0
    Lifted::new(0x7001edc2e0, tpl_20 as *const (), 1), // +0xc2e0 sub_c2e0
    Lifted::new(0x7001edc2f0, fn_7001edc2f0 as *const (), 0), // +0xc2f0 sub_c2f0
    Lifted::new(0x7001edc3b0, tpl_24::<0x7001edafa0, 0x7001eda590, 0x7001edc3c6> as *const (), 0), // +0xc3b0 sub_c3b0
    Lifted::new(0x7001edc3d0, tpl_33::<0x8, 0x1, 0x7001edc3b0, 0x7001edc41d> as *const (), 0), // +0xc3d0 sub_c3d0
    Lifted::new(0x7001edc440, tpl_33::<0x8, 0x1, 0x7001edc3b0, 0x7001edc48d> as *const (), 0), // +0xc440 sub_c440
    Lifted::new(0x7001edc4b0, fn_7001edc4b0 as *const (), 1), // +0xc4b0 sub_c4b0
    Lifted::new(0x7001edc4e0, fn_7001edc4e0 as *const (), 0), // +0xc4e0 sub_c4e0
    Lifted::new(0x7001edc5b0, tpl_34::<0x7001eeeb18> as *const (), 1), // +0xc5b0 sub_c5b0
    Lifted::new(0x7001edc5c0, fn_7001edc5c0 as *const (), 0), // +0xc5c0 sub_c5c0
    Lifted::new(0x7001edc6a0, fn_7001edc6a0 as *const (), 0), // +0xc6a0 sub_c6a0
    Lifted::new(0x7001edc730, tpl_35::<0x7001ee8a40, 0x7001ed27e0> as *const (), 0), // +0xc730 sub_c730
    Lifted::new(0x7001edc7d0, tpl_35::<0x7001ee8a40, 0x7001ed27e0> as *const (), 0), // +0xc7d0 sub_c7d0
    Lifted::new(0x7001edc870, fn_7001edc870 as *const (), 0), // +0xc870 sub_c870
    Lifted::new(0x7001edc8e0, fn_7001edc8e0 as *const (), 0), // +0xc8e0 sub_c8e0
    Lifted::new(0x7001edca80, fn_7001edca80 as *const (), 0), // +0xca80 sub_ca80
    Lifted::new(0x7001edcc60, fn_7001edcc60 as *const (), 2), // +0xcc60 sub_cc60
    Lifted::new(0x7001edcd90, fn_7001edcd90 as *const (), 0), // +0xcd90 sub_cd90
    Lifted::new(0x7001edce60, tpl_15::<0x7001eeeea8, 0x7001eea7e2> as *const (), 0), // +0xce60 sub_ce60
    Lifted::new(0x7001edce90, tpl_15::<0x7001eeee58, 0x7001eea7e2> as *const (), 0), // +0xce90 sub_ce90
    Lifted::new(0x7001edcec0, tpl_15::<0x7001eeee80, 0x7001eea7e2> as *const (), 0), // +0xcec0 sub_cec0
    Lifted::new(0x7001edcef0, tpl_30::<0x7001ed1420, 0x7001eea7e2> as *const (), 0), // +0xcef0 sub_cef0
    Lifted::new(0x7001edcf30, fn_7001edcf30 as *const (), 0), // +0xcf30 sub_cf30
    Lifted::new(0x7001edcf70, tpl_14::<0x7001eea7e2> as *const (), 0), // +0xcf70 sub_cf70
    Lifted::new(0x7001edcfa0, tpl_36::<0x30> as *const (), 0), // +0xcfa0 sub_cfa0
    Lifted::new(0x7001edd000, fn_7001edd000 as *const (), 0), // +0xd000 sub_d000
    Lifted::new(0x7001edd060, tpl_36::<0x30> as *const (), 0), // +0xd060 sub_d060
    Lifted::new(0x7001edd0c0, fn_7001edd0c0 as *const (), 0), // +0xd0c0 sub_d0c0
    Lifted::new(0x7001edd130, fn_7001edd130 as *const (), 2), // +0xd130 sub_d130
    Lifted::new(0x7001edd160, tpl_12 as *const (), 1), // +0xd160 sub_d160
    Lifted::new(0x7001edd170, fn_7001edd170 as *const (), 0), // +0xd170 sub_d170
    Lifted::new(0x7001edd250, fn_7001edd250 as *const (), 2), // +0xd250 sub_d250
    Lifted::new(0x7001edd500, tpl_26::<0x40> as *const (), 1), // +0xd500 sub_d500
    Lifted::new(0x7001edd510, tpl_37 as *const (), 1), // +0xd510 sub_d510
    Lifted::new(0x7001edd520, tpl_34::<0x7001eddc00> as *const (), 1), // +0xd520 sub_d520
    Lifted::new(0x7001edd530, tpl_37 as *const (), 1), // +0xd530 sub_d530
    Lifted::new(0x7001edd540, tpl_26::<0x38> as *const (), 1), // +0xd540 sub_d540
    Lifted::new(0x7001edd550, tpl_18::<0x14> as *const (), 1), // +0xd550 sub_d550
    Lifted::new(0x7001edd560, tpl_20 as *const (), 1), // +0xd560 sub_d560
    Lifted::new(0x7001edd570, fn_7001edd570 as *const (), 2), // +0xd570 sub_d570
    Lifted::new(0x7001edd5c0, fn_7001edd5c0 as *const (), 0), // +0xd5c0 sub_d5c0
    Lifted::new(0x7001edd7d0, fn_7001edd7d0 as *const (), 0), // +0xd7d0 sub_d7d0
    Lifted::new(0x7001edd8c0, fn_7001edd8c0 as *const (), 0), // +0xd8c0 sub_d8c0
    Lifted::new(0x7001edd960, fn_7001edd960 as *const (), 0), // +0xd960 sub_d960
    Lifted::new(0x7001eddb00, tpl_27::<0x10> as *const (), 1), // +0xdb00 sub_db00
    Lifted::new(0x7001eddb10, tpl_12 as *const (), 1), // +0xdb10 sub_db10
    Lifted::new(0x7001eddb20, tpl_20 as *const (), 1), // +0xdb20 sub_db20
    Lifted::new(0x7001eddb30, fn_7001eddb30 as *const (), 0), // +0xdb30 sub_db30
    Lifted::new(0x7001eddc00, fn_7001eddc00 as *const (), 0), // +0xdc00 sub_dc00
    Lifted::new(0x7001eddc50, fn_7001eddc50 as *const (), 0), // +0xdc50 sub_dc50
    Lifted::new(0x7001edde20, tpl_20 as *const (), 1), // +0xde20 sub_de20
    Lifted::new(0x7001edde30, fn_7001edde30 as *const (), 2), // +0xde30 sub_de30
    Lifted::new(0x7001ede0a0, fn_7001ede0a0 as *const (), 0), // +0xe0a0 sub_e0a0
    Lifted::new(0x7001ede0f0, fn_7001ede0f0 as *const (), 0), // +0xe0f0 sub_e0f0
    Lifted::new(0x7001ede160, fn_7001ede160 as *const (), 0), // +0xe160 sub_e160
    Lifted::new(0x7001ede1b0, tpl_38::<0x10> as *const (), 0), // +0xe1b0 sub_e1b0
    Lifted::new(0x7001ede1d0, tpl_38::<0x10> as *const (), 0), // +0xe1d0 sub_e1d0
    Lifted::new(0x7001ede1f0, tpl_38::<0x10> as *const (), 0), // +0xe1f0 sub_e1f0
    Lifted::new(0x7001ede210, tpl_39::<0x10, 0x40> as *const (), 0), // +0xe210 sub_e210
    Lifted::new(0x7001ede230, fn_7001ede230 as *const (), 0), // +0xe230 sub_e230
    Lifted::new(0x7001ede240, tpl_39::<0x10, 0x40> as *const (), 0), // +0xe240 sub_e240
    Lifted::new(0x7001ede260, fn_7001ede260 as *const (), 0), // +0xe260 sub_e260
    Lifted::new(0x7001ede6e0, tpl_37 as *const (), 1), // +0xe6e0 sub_e6e0
    Lifted::new(0x7001ede6f0, tpl_40 as *const (), 1), // +0xe6f0 sub_e6f0
    Lifted::new(0x7001edea10, tpl_41::<0x10, 0x19, 0x8> as *const (), 1), // +0xea10 sub_ea10
    Lifted::new(0x7001edea80, tpl_42::<0x10, 0x19, 0x8> as *const (), 1), // +0xea80 sub_ea80
    Lifted::new(0x7001edeaf0, tpl_25::<0x7001eeec70, 0x7001ee88f0, 0x7001edeb00> as *const (), 0), // +0xeaf0 sub_eaf0
    Lifted::new(0x7001edeb10, fn_7001edeb10 as *const (), 0), // +0xeb10 sub_eb10
    Lifted::new(0x7001ededb0, fn_7001ededb0 as *const (), 0), // +0xedb0 sub_edb0
    Lifted::new(0x7001edee70, fn_7001edee70 as *const (), 0), // +0xee70 sub_ee70
    Lifted::new(0x7001edf000, tpl_43::<0x7001eeef58, 0x7001efd6e8, 0x7001eeeed0, 0x30, 0x7001efd700, 0x7001eeef90, 0x7001eeea80, 0x7001efd778, 0x7001edf05e> as *const (), 0), // +0xf000 sub_f000
    Lifted::new(0x7001edf090, fn_7001edf090 as *const (), 0), // +0xf090 sub_f090
    Lifted::new(0x7001edf150, fn_7001edf150 as *const (), 0), // +0xf150 sub_f150
    Lifted::new(0x7001edf1a0, tpl_44::<0x7001eeeed0, 0x7001efd678, 0x7001efd6f0> as *const (), 0), // +0xf1a0 sub_f1a0
    Lifted::new(0x7001edf1e0, fn_7001edf1e0 as *const (), 0), // +0xf1e0 sub_f1e0
    Lifted::new(0x7001edf2f0, fn_7001edf2f0 as *const (), 0), // +0xf2f0 sub_f2f0
    Lifted::new(0x7001edf360, tpl_45::<0x7001eeeed0, 0x7001efd678, 0x7001efd6f0, 0x7001eea7e2> as *const (), 0), // +0xf360 sub_f360
    Lifted::new(0x7001edf3c0, fn_7001edf3c0 as *const (), 0), // +0xf3c0 sub_f3c0
    Lifted::new(0x7001edf420, tpl_20 as *const (), 1), // +0xf420 sub_f420
    Lifted::new(0x7001edf430, tpl_19::<0x88> as *const (), 2), // +0xf430 sub_f430
    Lifted::new(0x7001edf440, fn_7001edf440 as *const (), 2), // +0xf440 sub_f440
    Lifted::new(0x7001edf4e0, fn_7001edf4e0 as *const (), 0), // +0xf4e0 sub_f4e0
    Lifted::new(0x7001edf500, fn_7001edf500 as *const (), 0), // +0xf500 sub_f500
    Lifted::new(0x7001edf590, fn_7001edf590 as *const (), 0), // +0xf590 sub_f590
    Lifted::new(0x7001edf630, fn_7001edf630 as *const (), 0), // +0xf630 sub_f630
    Lifted::new(0x7001edf6e0, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001edf6f0> as *const (), 0), // +0xf6e0 sub_f6e0
    Lifted::new(0x7001edf700, tpl_5::<0x7001eea730> as *const (), 0), // +0xf700 sub_f700
    Lifted::new(0x7001edf740, fn_7001edf740 as *const (), 0), // +0xf740 sub_f740
    Lifted::new(0x7001edf7e0, tpl_6::<0x7001eea740> as *const (), 0), // +0xf7e0 sub_f7e0
    Lifted::new(0x7001edf820, tpl_7::<0x7001eea740> as *const (), 0), // +0xf820 sub_f820
    Lifted::new(0x7001edf850, tpl_5::<0x7001eea730> as *const (), 0), // +0xf850 sub_f850
    Lifted::new(0x7001edf890, fn_7001edf890 as *const (), 0), // +0xf890 sub_f890
    Lifted::new(0x7001edf950, tpl_11::<0x7001eea7e2, 0x7001efd520, 0x7001edfa53> as *const (), 0), // +0xf950 sub_f950
    Lifted::new(0x7001edfaa0, fn_7001edfaa0 as *const (), 0), // +0xfaa0 sub_faa0
    Lifted::new(0x7001edfb40, fn_7001edfb40 as *const (), 0), // +0xfb40 sub_fb40
    Lifted::new(0x7001edfbc0, fn_7001edfbc0 as *const (), 2), // +0xfbc0 sub_fbc0
    Lifted::new(0x7001edfd50, fn_7001edfd50 as *const (), 2), // +0xfd50 sub_fd50
    Lifted::new(0x7001ee00e0, tpl_19::<0x50> as *const (), 2), // +0x100e0 sub_100e0
    Lifted::new(0x7001ee00f0, fn_7001ee00f0 as *const (), 2), // +0x100f0 sub_100f0
    Lifted::new(0x7001ee01c0, tpl_46::<0x7001efd760, 0x48, 0x7001efd668, 0x8, 0x7001efd758> as *const (), 0), // +0x101c0 sub_101c0
    Lifted::new(0x7001ee0210, fn_7001ee0210 as *const (), 0), // +0x10210 sub_10210
    Lifted::new(0x7001ee0690, tpl_23::<0x7001ee8a40, 0x7001eea730, 0x7001eea740, 0x7001eea7e2, 0x7001efd520, 0x7001ee0858, 0x7001eda5e0, 0x7001ee085e, 0x7001ee0ce0, 0x7001ee0867> as *const (), 0), // +0x10690 sub_10690
    Lifted::new(0x7001ee08e0, tpl_40 as *const (), 1), // +0x108e0 sub_108e0
    Lifted::new(0x7001ee0c00, tpl_41::<0x10, 0x19, 0x8> as *const (), 1), // +0x10c00 sub_10c00
    Lifted::new(0x7001ee0c70, tpl_42::<0x10, 0x19, 0x8> as *const (), 1), // +0x10c70 sub_10c70
    Lifted::new(0x7001ee0ce0, tpl_25::<0x7001eeeab0, 0x7001ee88f0, 0x7001ee0cf0> as *const (), 0), // +0x10ce0 sub_10ce0
    Lifted::new(0x7001ee0d00, fn_7001ee0d00 as *const (), 0), // +0x10d00 sub_10d00
    Lifted::new(0x7001ee0e10, fn_7001ee0e10 as *const (), 0), // +0x10e10 sub_10e10
    Lifted::new(0x7001ee0e90, fn_7001ee0e90 as *const (), 0), // +0x10e90 sub_10e90
    Lifted::new(0x7001ee0ef0, fn_7001ee0ef0 as *const (), 0), // +0x10ef0 sub_10ef0
    Lifted::new(0x7001ee1040, fn_7001ee1040 as *const (), 0), // +0x11040 sub_11040
    Lifted::new(0x7001ee10a0, fn_7001ee10a0 as *const (), 0), // +0x110a0 sub_110a0
    Lifted::new(0x7001ee11f0, tpl_46::<0x7001efd760, 0x38, 0x7001efd650, 0x8, 0x7001efd758> as *const (), 0), // +0x111f0 sub_111f0
    Lifted::new(0x7001ee1240, fn_7001ee1240 as *const (), 0), // +0x11240 sub_11240
    Lifted::new(0x7001ee12f0, fn_7001ee12f0 as *const (), 0), // +0x112f0 sub_112f0
    Lifted::new(0x7001ee1530, fn_7001ee1530 as *const (), 1), // +0x11530 sub_11530
    Lifted::new(0x7001ee1580, fn_7001ee1580 as *const (), 0), // +0x11580 sub_11580
    Lifted::new(0x7001ee1620, fn_7001ee1620 as *const (), 0), // +0x11620 sub_11620
    Lifted::new(0x7001ee16b0, fn_7001ee16b0 as *const (), 0), // +0x116b0 sub_116b0
    Lifted::new(0x7001ee1700, fn_7001ee1700 as *const (), 0), // +0x11700 sub_11700
    Lifted::new(0x7001ee1770, fn_7001ee1770 as *const (), 1), // +0x11770 sub_11770
    Lifted::new(0x7001ee1790, fn_7001ee1790 as *const (), 0), // +0x11790 sub_11790
    Lifted::new(0x7001ee1980, fn_7001ee1980 as *const (), 0), // +0x11980 sub_11980
    Lifted::new(0x7001ee1a90, fn_7001ee1a90 as *const (), 0), // +0x11a90 sub_11a90
    Lifted::new(0x7001ee1b90, fn_7001ee1b90 as *const (), 0), // +0x11b90 sub_11b90
    Lifted::new(0x7001ee1ca0, fn_7001ee1ca0 as *const (), 1), // +0x11ca0 sub_11ca0
    Lifted::new(0x7001ee1cc0, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001ee1cd0> as *const (), 0), // +0x11cc0 sub_11cc0
    Lifted::new(0x7001ee1ce0, fn_7001ee1ce0 as *const (), 0), // +0x11ce0 sub_11ce0
    Lifted::new(0x7001ee1d90, fn_7001ee1d90 as *const (), 2), // +0x11d90 sub_11d90
    Lifted::new(0x7001ee1fd0, fn_7001ee1fd0 as *const (), 0), // +0x11fd0 sub_11fd0
    Lifted::new(0x7001ee2190, fn_7001ee2190 as *const (), 2), // +0x12190 sub_12190
    Lifted::new(0x7001ee2230, tpl_47::<0x7001efd238, 0x7001efd2a0, 0x7001ee1fd0, 0x7001efd260, 0x7001efd248, 0x7001efd298> as *const (), 0), // +0x12230 sub_12230
    Lifted::new(0x7001ee2410, tpl_47::<0x7001efd238, 0x7001efd2a0, 0x7001ee1fd0, 0x7001efd260, 0x7001efd248, 0x7001efd298> as *const (), 0), // +0x12410 sub_12410
    Lifted::new(0x7001ee25f0, fn_7001ee25f0 as *const (), 0), // +0x125f0 sub_125f0
    Lifted::new(0x7001ee26e0, tpl_48::<0xffffffffffffff68, 0x4, 0x7001eef070, 0xffffffffffffff64, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd228> as *const (), 0), // +0x126e0 sub_126e0
    Lifted::new(0x7001ee2750, tpl_49::<0xfffffffffffffffc, 0x7001ee27a0> as *const (), 0), // +0x12750 sub_12750
    Lifted::new(0x7001ee2760, tpl_30::<0x7001ee25f0, 0x7001eea7e2> as *const (), 0), // +0x12760 sub_12760
    Lifted::new(0x7001ee27a0, tpl_50::<0xffffffffffffff68, 0x4, 0x7001eef070, 0xffffffffffffff64, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd228, 0x7001efd250, 0x7001eea7e2> as *const (), 0), // +0x127a0 sub_127a0
    Lifted::new(0x7001ee2850, fn_7001ee2850 as *const (), 2), // +0x12850 sub_12850
    Lifted::new(0x7001ee2970, fn_7001ee2970 as *const (), 2), // +0x12970 sub_12970
    Lifted::new(0x7001ee2aa0, fn_7001ee2aa0 as *const (), 0), // +0x12aa0 sub_12aa0
    Lifted::new(0x7001ee3020, tpl_25::<0x7001eeefc8, 0x7001ee88f0, 0x7001ee3030> as *const (), 0), // +0x13020 sub_13020
    Lifted::new(0x7001ee3040, tpl_25::<0x7001eeea98, 0x7001ee88f0, 0x7001ee3050> as *const (), 0), // +0x13040 sub_13040
    Lifted::new(0x7001ee3060, fn_7001ee3060 as *const (), 0), // +0x13060 sub_13060
    Lifted::new(0x7001ee30d0, fn_7001ee30d0 as *const (), 0), // +0x130d0 sub_130d0
    Lifted::new(0x7001ee3290, fn_7001ee3290 as *const (), 0), // +0x13290 sub_13290
    Lifted::new(0x7001ee32e0, fn_7001ee32e0 as *const (), 0), // +0x132e0 sub_132e0
    Lifted::new(0x7001ee34f0, fn_7001ee34f0 as *const (), 1), // +0x134f0 sub_134f0
    Lifted::new(0x7001ee3560, fn_7001ee3560 as *const (), 1), // +0x13560 sub_13560
    Lifted::new(0x7001ee3710, fn_7001ee3710 as *const (), 1), // +0x13710 sub_13710
    Lifted::new(0x7001ee3840, fn_7001ee3840 as *const (), 1), // +0x13840 sub_13840
    Lifted::new(0x7001ee38c0, fn_7001ee38c0 as *const (), 0), // +0x138c0 sub_138c0
    Lifted::new(0x7001ee3920, fn_7001ee3920 as *const (), 0), // +0x13920 sub_13920
    Lifted::new(0x7001ee3df0, fn_7001ee3df0 as *const (), 1), // +0x13df0 sub_13df0
    Lifted::new(0x7001ee3ea0, fn_7001ee3ea0 as *const (), 0), // +0x13ea0 sub_13ea0
    Lifted::new(0x7001ee3ef0, fn_7001ee3ef0 as *const (), 0), // +0x13ef0 sub_13ef0
    Lifted::new(0x7001ee40c0, fn_7001ee40c0 as *const (), 0), // +0x140c0 sub_140c0
    Lifted::new(0x7001ee42e0, fn_7001ee42e0 as *const (), 0), // +0x142e0 sub_142e0
    Lifted::new(0x7001ee43a0, fn_7001ee43a0 as *const (), 0), // +0x143a0 sub_143a0
    Lifted::new(0x7001ee43c0, fn_7001ee43c0 as *const (), 0), // +0x143c0 sub_143c0
    Lifted::new(0x7001ee45c0, fn_7001ee45c0 as *const (), 0), // +0x145c0 sub_145c0
    Lifted::new(0x7001ee47b0, fn_7001ee47b0 as *const (), 0), // +0x147b0 sub_147b0
    Lifted::new(0x7001ee4800, fn_7001ee4800 as *const (), 0), // +0x14800 sub_14800
    Lifted::new(0x7001ee4920, fn_7001ee4920 as *const (), 2), // +0x14920 sub_14920
    Lifted::new(0x7001ee4a70, fn_7001ee4a70 as *const (), 0), // +0x14a70 sub_14a70
    Lifted::new(0x7001ee4b00, fn_7001ee4b00 as *const (), 0), // +0x14b00 sub_14b00
    Lifted::new(0x7001ee4be0, fn_7001ee4be0 as *const (), 1), // +0x14be0 sub_14be0
    Lifted::new(0x7001ee4c10, tpl_48::<0xffffffffffffff78, 0x4, 0x7001eef148, 0xffffffffffffff74, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd1c8> as *const (), 0), // +0x14c10 sub_14c10
    Lifted::new(0x7001ee4ca0, fn_7001ee4ca0 as *const (), 0), // +0x14ca0 sub_14ca0
    Lifted::new(0x7001ee4d20, fn_7001ee4d20 as *const (), 0), // +0x14d20 sub_14d20
    Lifted::new(0x7001ee4d30, tpl_29 as *const (), 1), // +0x14d30 sub_14d30
    Lifted::new(0x7001ee4d40, fn_7001ee4d40 as *const (), 1), // +0x14d40 sub_14d40
    Lifted::new(0x7001ee4d70, tpl_49::<0xfffffffffffffffc, 0x7001ee4d80> as *const (), 0), // +0x14d70 sub_14d70
    Lifted::new(0x7001ee4d80, tpl_50::<0xffffffffffffff78, 0x4, 0x7001eef148, 0xffffffffffffff74, 0x7001ee25f0, 0xffffffffffffff88, 0x7001efd1c8, 0x7001efd250, 0x7001eea7e2> as *const (), 0), // +0x14d80 sub_14d80
    Lifted::new(0x7001ee4e30, fn_7001ee4e30 as *const (), 0), // +0x14e30 sub_14e30
    Lifted::new(0x7001ee4ee0, fn_7001ee4ee0 as *const (), 0), // +0x14ee0 sub_14ee0
    Lifted::new(0x7001ee4f40, fn_7001ee4f40 as *const (), 0), // +0x14f40 sub_14f40
    Lifted::new(0x7001ee4ff0, fn_7001ee4ff0 as *const (), 2), // +0x14ff0 sub_14ff0
    Lifted::new(0x7001ee57e0, fn_7001ee57e0 as *const (), 0), // +0x157e0 sub_157e0
    Lifted::new(0x7001ee58c0, tpl_51::<0x100, 0x8> as *const (), 0), // +0x158c0 sub_158c0
    Lifted::new(0x7001ee58e0, fn_7001ee58e0 as *const (), 0), // +0x158e0 sub_158e0
    Lifted::new(0x7001ee5d90, fn_7001ee5d90 as *const (), 0), // +0x15d90 sub_15d90
    Lifted::new(0x7001ee5de0, fn_7001ee5de0 as *const (), 0), // +0x15de0 sub_15de0
    Lifted::new(0x7001ee5e20, fn_7001ee5e20 as *const (), 0), // +0x15e20 sub_15e20
    Lifted::new(0x7001ee5e80, fn_7001ee5e80 as *const (), 0), // +0x15e80 sub_15e80
    Lifted::new(0x7001ee5fb0, fn_7001ee5fb0 as *const (), 0), // +0x15fb0 sub_15fb0
    Lifted::new(0x7001ee6090, fn_7001ee6090 as *const (), 2), // +0x16090 sub_16090
    Lifted::new(0x7001ee62a0, tpl_51::<0x100, 0x10> as *const (), 0), // +0x162a0 sub_162a0
    Lifted::new(0x7001ee62c0, tpl_12 as *const (), 1), // +0x162c0 sub_162c0
    Lifted::new(0x7001ee62d0, tpl_12 as *const (), 1), // +0x162d0 sub_162d0
    Lifted::new(0x7001ee62e0, fn_7001ee62e0 as *const (), 1), // +0x162e0 sub_162e0
    Lifted::new(0x7001ee6300, fn_7001ee6300 as *const (), 0), // +0x16300 sub_16300
    Lifted::new(0x7001ee6580, fn_7001ee6580 as *const (), 2), // +0x16580 sub_16580
    Lifted::new(0x7001ee68a0, fn_7001ee68a0 as *const (), 1), // +0x168a0 sub_168a0
    Lifted::new(0x7001ee69b0, fn_7001ee69b0 as *const (), 0), // +0x169b0 sub_169b0
    Lifted::new(0x7001ee6a40, fn_7001ee6a40 as *const (), 0), // +0x16a40 sub_16a40
    Lifted::new(0x7001ee6af0, fn_7001ee6af0 as *const (), 0), // +0x16af0 sub_16af0
    Lifted::new(0x7001ee6bc0, fn_7001ee6bc0 as *const (), 0), // +0x16bc0 sub_16bc0
    Lifted::new(0x7001ee6d00, fn_7001ee6d00 as *const (), 0), // +0x16d00 sub_16d00
    Lifted::new(0x7001ee6eb0, fn_7001ee6eb0 as *const (), 0), // +0x16eb0 sub_16eb0
    Lifted::new(0x7001ee6f70, tpl_30::<0x7001ed2810, 0x7001eea7e2> as *const (), 0), // +0x16f70 sub_16f70
    Lifted::new(0x7001ee6fb0, tpl_14::<0x7001eea7e2> as *const (), 0), // +0x16fb0 sub_16fb0
    Lifted::new(0x7001ee6fe0, fn_7001ee6fe0 as *const (), 0), // +0x16fe0 sub_16fe0
    Lifted::new(0x7001ee70e0, tpl_37 as *const (), 1), // +0x170e0 sub_170e0
    Lifted::new(0x7001ee70f0, fn_7001ee70f0 as *const (), 0), // +0x170f0 sub_170f0
    Lifted::new(0x7001ee7290, tpl_37 as *const (), 1), // +0x17290 sub_17290
    Lifted::new(0x7001ee72a0, fn_7001ee72a0 as *const (), 0), // +0x172a0 sub_172a0
    Lifted::new(0x7001ee7330, tpl_12 as *const (), 1), // +0x17330 sub_17330
    Lifted::new(0x7001ee7340, tpl_52::<0x7001efd630> as *const (), 0), // +0x17340 sub_17340
    Lifted::new(0x7001ee7360, tpl_52::<0x7001efd728> as *const (), 0), // +0x17360 sub_17360
    Lifted::new(0x7001ee7380, fn_7001ee7380 as *const (), 0), // +0x17380 sub_17380
    Lifted::new(0x7001ee7520, tpl_34::<0x7001ef9cf0> as *const (), 1), // +0x17520 sub_17520
    Lifted::new(0x7001ee7530, fn_7001ee7530 as *const (), 1), // +0x17530 sub_17530
    Lifted::new(0x7001ee7550, fn_7001ee7550 as *const (), 0), // +0x17550 sub_17550
    Lifted::new(0x7001ee7580, fn_7001ee7580 as *const (), 0), // +0x17580 sub_17580
    Lifted::new(0x7001ee7720, tpl_12 as *const (), 1), // +0x17720 sub_17720
    Lifted::new(0x7001ee7730, fn_7001ee7730 as *const (), 2), // +0x17730 sub_17730
    Lifted::new(0x7001ee78a0, fn_7001ee78a0 as *const (), 2), // +0x178a0 sub_178a0
    Lifted::new(0x7001ee7a30, fn_7001ee7a30 as *const (), 0), // +0x17a30 sub_17a30
    Lifted::new(0x7001ee7af0, fn_7001ee7af0 as *const (), 0), // +0x17af0 sub_17af0
    Lifted::new(0x7001ee7b50, fn_7001ee7b50 as *const (), 0), // +0x17b50 sub_17b50
    Lifted::new(0x7001ee7bd0, tpl_12 as *const (), 1), // +0x17bd0 sub_17bd0
    Lifted::new(0x7001ee7be0, fn_7001ee7be0 as *const (), 0), // +0x17be0 sub_17be0
    Lifted::new(0x7001ee7d50, fn_7001ee7d50 as *const (), 2), // +0x17d50 sub_17d50
    Lifted::new(0x7001ee7e10, fn_7001ee7e10 as *const (), 0), // +0x17e10 sub_17e10
    Lifted::new(0x7001ee8080, tpl_43::<0x7001eef460, 0x7001efd6e8, 0x7001eef448, 0x30, 0x7001efd700, 0x7001eeef90, 0x7001eeea80, 0x7001efd778, 0x7001ee80de> as *const (), 0), // +0x18080 sub_18080
    Lifted::new(0x7001ee8110, tpl_44::<0x7001eef448, 0x7001efd678, 0x7001efd6f0> as *const (), 0), // +0x18110 sub_18110
    Lifted::new(0x7001ee8150, fn_7001ee8150 as *const (), 0), // +0x18150 sub_18150
    Lifted::new(0x7001ee8240, tpl_45::<0x7001eef448, 0x7001efd678, 0x7001efd6f0, 0x7001eea7e2> as *const (), 0), // +0x18240 sub_18240
    Lifted::new(0x7001ee82a0, fn_7001ee82a0 as *const (), 0), // +0x182a0 sub_182a0
    Lifted::new(0x7001ee8340, fn_7001ee8340 as *const (), 2), // +0x18340 sub_18340
    Lifted::new(0x7001ee83f0, fn_7001ee83f0 as *const (), 0), // +0x183f0 sub_183f0
    Lifted::new(0x7001ee8420, fn_7001ee8420 as *const (), 0), // +0x18420 sub_18420
    Lifted::new(0x7001ee8730, fn_7001ee8730 as *const (), 1), // +0x18730 sub_18730
    Lifted::new(0x7001ee8740, fn_7001ee8740 as *const (), 0), // +0x18740 sub_18740
    Lifted::new(0x7001ee8a40, fn_7001ee8a40 as *const (), 0), // +0x18a40 sub_18a40
    Lifted::new(0x7001ee8ab8, fn_7001ee8ab8 as *const (), 0), // +0x18ab8 sub_18ab8
    Lifted::new(0x7001ee8adc, fn_7001ee8adc as *const (), 0), // +0x18adc sub_18adc
    Lifted::new(0x7001ee8b60, fn_7001ee8b60 as *const (), 0), // +0x18b60 sub_18b60
    Lifted::new(0x7001ee8ba0, fn_7001ee8ba0 as *const (), 0), // +0x18ba0 sub_18ba0
    Lifted::new(0x7001ee8c04, fn_7001ee8c04 as *const (), 0), // +0x18c04 sub_18c04
    Lifted::new(0x7001ee8d60, fn_7001ee8d60 as *const (), 0), // +0x18d60 sub_18d60
    Lifted::new(0x7001ee8e08, fn_7001ee8e08 as *const (), 0), // +0x18e08 sub_18e08
    Lifted::new(0x7001ee8fb0, fn_7001ee8fb0 as *const (), 0), // +0x18fb0 sub_18fb0
    Lifted::new(0x7001ee9018, tpl_53::<0x7001eef4a0, 0x8, 0x7001eea77c, 0x7001eef4d8> as *const (), 2), // +0x19018 sub_19018
    Lifted::new(0x7001ee9064, tpl_54::<0x10, 0x7001eef4f0, 0x8, 0x7001eef4d8> as *const (), 1), // +0x19064 sub_19064
    Lifted::new(0x7001ee908c, tpl_53::<0x7001eef4a0, 0x8, 0x7001eea77c, 0x7001eef510> as *const (), 2), // +0x1908c sub_1908c
    Lifted::new(0x7001ee90d8, tpl_54::<0x10, 0x7001eee920, 0x8, 0x7001eef510> as *const (), 1), // +0x190d8 sub_190d8
    Lifted::new(0x7001ee9100, fn_7001ee9100 as *const (), 2), // +0x19100 sub_19100
    Lifted::new(0x7001ee91b0, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0), // +0x191b0 sub_191b0
    Lifted::new(0x7001ee9210, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0), // +0x19210 sub_19210
    Lifted::new(0x7001ee9270, tpl_55::<0x7001eef4a0, 0x7001eea782, 0x7001eea7e2> as *const (), 0), // +0x19270 sub_19270
    Lifted::new(0x7001ee92c4, tpl_56::<0x7001ee9064, 0x7001ef34b8, 0x7001eea788, 0x7001ee92e3> as *const (), 0), // +0x192c4 sub_192c4
    Lifted::new(0x7001ee92ec, tpl_56::<0x7001ee90d8, 0x7001ef3540, 0x7001eea788, 0x7001ee930b> as *const (), 0), // +0x192ec sub_192ec
    Lifted::new(0x7001ee9320, fn_7001ee9320 as *const (), 1), // +0x19320 sub_19320
    Lifted::new(0x7001ee9338, fn_7001ee9338 as *const (), 0), // +0x19338 sub_19338
    Lifted::new(0x7001ee9380, fn_7001ee9380 as *const (), 0), // +0x19380 sub_19380
    Lifted::new(0x7001ee9488, fn_7001ee9488 as *const (), 0), // +0x19488 sub_19488
    Lifted::new(0x7001ee94a0, fn_7001ee94a0 as *const (), 0), // +0x194a0 sub_194a0
    Lifted::new(0x7001ee9564, fn_7001ee9564 as *const (), 0), // +0x19564 sub_19564
    Lifted::new(0x7001ee97d0, fn_7001ee97d0 as *const (), 0), // +0x197d0 sub_197d0
    Lifted::new(0x7001ee9858, fn_7001ee9858 as *const (), 0), // +0x19858 sub_19858
    Lifted::new(0x7001ee9930, fn_7001ee9930 as *const (), 0), // +0x19930 sub_19930
    Lifted::new(0x7001ee995c, fn_7001ee995c as *const (), 0), // +0x1995c sub_1995c
    Lifted::new(0x7001ee9970, fn_7001ee9970 as *const (), 0), // +0x19970 sub_19970
    Lifted::new(0x7001ee9980, tpl_34::<0x7001efa3a8> as *const (), 1), // +0x19980 sub_19980
    Lifted::new(0x7001ee998c, tpl_34::<0x7001efa3b8> as *const (), 1), // +0x1998c sub_1998c
    Lifted::new(0x7001ee9998, fn_7001ee9998 as *const (), 0), // +0x19998 sub_19998
    Lifted::new(0x7001ee99bc, fn_7001ee99bc as *const (), 0), // +0x199bc sub_199bc
    Lifted::new(0x7001ee9a84, fn_7001ee9a84 as *const (), 0), // +0x19a84 sub_19a84
    Lifted::new(0x7001ee9acc, fn_7001ee9acc as *const (), 0), // +0x19acc sub_19acc
    Lifted::new(0x7001ee9b10, fn_7001ee9b10 as *const (), 0), // +0x19b10 sub_19b10
    Lifted::new(0x7001ee9b2c, fn_7001ee9b2c as *const (), 0), // +0x19b2c sub_19b2c
    Lifted::new(0x7001ee9b60, fn_7001ee9b60 as *const (), 0), // +0x19b60 sub_19b60
    Lifted::new(0x7001ee9b7c, fn_7001ee9b7c as *const (), 0), // +0x19b7c sub_19b7c
    Lifted::new(0x7001ee9bf4, fn_7001ee9bf4 as *const (), 0), // +0x19bf4 sub_19bf4
    Lifted::new(0x7001ee9c30, fn_7001ee9c30 as *const (), 0), // +0x19c30 sub_19c30
    Lifted::new(0x7001ee9c4c, fn_7001ee9c4c as *const (), 0), // +0x19c4c sub_19c4c
    Lifted::new(0x7001ee9ca8, fn_7001ee9ca8 as *const (), 2), // +0x19ca8 sub_19ca8
    Lifted::new(0x7001ee9d58, fn_7001ee9d58 as *const (), 1), // +0x19d58 sub_19d58
    Lifted::new(0x7001ee9e18, fn_7001ee9e18 as *const (), 0), // +0x19e18 sub_19e18
    Lifted::new(0x7001ee9e48, fn_7001ee9e48 as *const (), 0), // +0x19e48 sub_19e48
    Lifted::new(0x7001ee9e7c, fn_7001ee9e7c as *const (), 0), // +0x19e7c sub_19e7c
    Lifted::new(0x7001ee9ee8, fn_7001ee9ee8 as *const (), 0), // +0x19ee8 sub_19ee8
    Lifted::new(0x7001ee9f04, fn_7001ee9f04 as *const (), 1), // +0x19f04 sub_19f04
    Lifted::new(0x7001eea0bc, fn_7001eea0bc as *const (), 0), // +0x1a0bc sub_1a0bc
    Lifted::new(0x7001eea110, fn_7001eea110 as *const (), 0), // +0x1a110 sub_1a110
    Lifted::new(0x7001eea190, fn_7001eea190 as *const (), 0), // +0x1a190 sub_1a190
    Lifted::new(0x7001eea204, tpl_57::<0x7001ef0458, 0x7001ef0560, 0x7001f01020> as *const (), 0), // +0x1a204 sub_1a204
    Lifted::new(0x7001eea250, tpl_57::<0x7001ef0678, 0x7001ef0780, 0x7001f01020> as *const (), 0), // +0x1a250 sub_1a250
    Lifted::new(0x7001eea2a0, fn_7001eea2a0 as *const (), 1), // +0x1a2a0 sub_1a2a0
    Lifted::new(0x7001eea300, fn_7001eea300 as *const (), 0), // +0x1a300 sub_1a300
    Lifted::new(0x7001eea404, fn_7001eea404 as *const (), 0), // +0x1a404 sub_1a404
    Lifted::new(0x7001eea4c0, fn_7001eea4c0 as *const (), 0), // +0x1a4c0 sub_1a4c0
    Lifted::new(0x7001eea4f8, fn_7001eea4f8 as *const (), 1), // +0x1a4f8 sub_1a4f8
    Lifted::new(0x7001eea704, fn_7001eea704 as *const (), 1), // +0x1a704 sub_1a704
    Lifted::new(0x7001eea70c, fn_7001eea70c as *const (), 1), // +0x1a70c sub_1a70c
    Lifted::new(0x7001eea898, tpl_21 as *const (), 1), // +0x1a898 sub_1a898
    Lifted::new(0x7001eea89c, tpl_21 as *const (), 1), // +0x1a89c sub_1a89c
    Lifted::new(0x7001eea8a0, tpl_21 as *const (), 1), // +0x1a8a0 sub_1a8a0
    Lifted::new(0x7001eea8a4, tpl_21 as *const (), 1), // +0x1a8a4 sub_1a8a4
    Lifted::new(0x7001eea8a8, tpl_21 as *const (), 1), // +0x1a8a8 sub_1a8a8
    Lifted::new(0x7001eea8ac, tpl_37 as *const (), 1), // +0x1a8ac sub_1a8ac
    Lifted::new(0x7001eebea0, fn_7001eebea0 as *const (), 2), // +0x1bea0 sub_1bea0
    Lifted::new(0x7001eebec0, fn_7001eebec0 as *const (), 2), // +0x1bec0 sub_1bec0
    Lifted::new(0x7001eecece, fn_7001eecece as *const (), 0), // +0x1cece sub_1cece
    Lifted::new(0x7001eeceea, fn_7001eeceea as *const (), 0), // +0x1ceea sub_1ceea
    Lifted::new(0x7001eecf03, fn_7001eecf03 as *const (), 0), // +0x1cf03 sub_1cf03
    Lifted::new(0x7001eecf22, fn_7001eecf22 as *const (), 0), // +0x1cf22 sub_1cf22
    Lifted::new(0x7001eecf65, fn_7001eecf65 as *const (), 1), // +0x1cf65 sub_1cf65
];
